//! # Multi-dimensional CLUT evaluation — Pass 4 groundwork
//!
//! Evaluates the colour lookup tables the profile layer represents
//! (`lut8`/`lut16`'s hypercubic grids, `mAB `/`mBA `'s per-dimension
//! grids) at arbitrary input points.
//!
//! ## The A16 silence, and iccce's named choice
//!
//! **ICC.1 does not specify n-D interpolation between CLUT grid
//! points** — corpus ambiguity **A16**, confirmed SILENT against the
//! primary spec (an exhaustive search: the only normative sentence is
//! a constraint on the profile *author* for the two-grid-point case).
//! Two conformant CMMs can produce visibly different colour from the
//! same profile; the trilinear-vs-tetrahedral choice is worth up to
//! **~1 ΔE** in regions of high CLUT curvature
//! (`icc__type__lut8_lut16.md`) — at the perceptibility threshold, so
//! the choice is measurable, not academic.
//!
//! **iccce's choice, per rule 4 (named and measured): n-linear
//! interpolation**, citing A16 as the licence to choose. Why n-linear
//! first: it is fully specified by its own definition (no scheme
//! variants), works for every input dimensionality, and is exact on
//! the class of functions the spec's own two-grid-point sentence
//! contemplates. **Tetrahedral is deliberately absent**: lcms2 uses it
//! for 3-input CLUTs and the difference is the biggest expected
//! iccce-vs-lcms2 deviation in Pass 4 — but the cube decomposition
//! has several published variants and the corpus does not yet carry
//! lcms2's; it will be sourced (as measured/impl-reference material)
//! before it is written, not recalled. Until then, Pass 4 differential
//! tolerances must budget for the interpolation-method difference and
//! say so.
//!
//! ## Index ordering
//!
//! First input channel varies SLOWEST (clause 10.10, corpus A20
//! resolved): flat node index of `(i₀, …, i_{d−1})` is
//! `((i₀·P₁ + i₁)·P₂ + …)·P_{d−1} + i_{d−1}`, times `outputs`, with a
//! node's output values contiguous. Getting this backwards
//! channel-swaps the image — loud, but cheap to prevent with the
//! asymmetric-grid test below.

/// A CLUT in evaluable form: normalised `f64` samples. Conversion
/// from stored `u8`/`u16` (÷255, ÷65535) — and any legacy-Lab
/// decoding — is the transform assembler's cited act, not this
/// module's; by the time samples are here they are plain numbers.
#[derive(Debug, Clone, PartialEq)]
pub struct Clut {
    /// Grid points per input dimension, in channel order. Length =
    /// input dimensionality. INVARIANT: every entry ≥ 2 and the
    /// products bounded (enforced by [`Clut::new`]).
    pub grid: Vec<usize>,
    /// Output channels per node.
    pub outputs: usize,
    /// `Π grid[i] × outputs` values, first channel slowest.
    pub samples: Vec<f64>,
}

/// Construction errors — all invariant violations, reported.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClutError {
    /// A dimension with fewer than 2 grid points cannot interpolate.
    /// (A gridPoints of 0/1 in a real tag is a malformation the
    /// profile layer reports; here it is a hard construction error.)
    DimensionTooSmall { dim: usize, points: usize },
    /// No input dimensions at all.
    NoDimensions,
    /// `samples.len()` disagrees with `Π grid × outputs`.
    SampleCountMismatch { expected: usize, actual: usize },
    /// More than 16 dimensions (the ICC format's own `gridPoints[16]`
    /// bound) — also caps the 2^d corner walk.
    TooManyDimensions { dims: usize },
}

impl std::fmt::Display for ClutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DimensionTooSmall { dim, points } => {
                write!(
                    f,
                    "clut dimension {dim} has {points} grid point(s), minimum 2"
                )
            }
            Self::NoDimensions => write!(f, "clut has no input dimensions"),
            Self::SampleCountMismatch { expected, actual } => {
                write!(f, "clut sample count {actual}, expected {expected}")
            }
            Self::TooManyDimensions { dims } => {
                write!(f, "clut has {dims} dimensions, format maximum is 16")
            }
        }
    }
}

impl Clut {
    pub fn new(grid: Vec<usize>, outputs: usize, samples: Vec<f64>) -> Result<Clut, ClutError> {
        if grid.is_empty() {
            return Err(ClutError::NoDimensions);
        }
        if grid.len() > 16 {
            return Err(ClutError::TooManyDimensions { dims: grid.len() });
        }
        for (dim, &points) in grid.iter().enumerate() {
            if points < 2 {
                return Err(ClutError::DimensionTooSmall { dim, points });
            }
        }
        let expected: usize = grid.iter().product::<usize>() * outputs;
        if samples.len() != expected {
            return Err(ClutError::SampleCountMismatch {
                expected,
                actual: samples.len(),
            });
        }
        Ok(Clut {
            grid,
            outputs,
            samples,
        })
    }

    /// Flat sample index of a grid node (first channel slowest — A20).
    fn node_base(&self, idx: &[usize]) -> usize {
        let mut flat = 0usize;
        for (dim, &i) in idx.iter().enumerate() {
            flat = flat * self.grid[dim] + i;
        }
        flat * self.outputs
    }

    /// n-linear interpolation at `input` (each component clamped to
    /// [0,1] — the CLUT's domain; out-of-domain handling beyond that
    /// is gamut policy, which lives with the transform, not here).
    ///
    /// Walks the 2^d corners of the containing cell; weight of a
    /// corner is the product over dimensions of `frac` or `1−frac`.
    /// d ≤ 16 by construction, so the walk is bounded at 65536
    /// corners; real profiles are ≤ 4-D (16 corners).
    #[must_use]
    pub fn eval(&self, input: &[f64], out: &mut [f64]) -> bool {
        let d = self.grid.len();
        if input.len() != d || out.len() != self.outputs {
            return false;
        }
        // Cell origin and fraction per dimension.
        let mut base_idx = vec![0usize; d];
        let mut frac = vec![0.0f64; d];
        for dim in 0..d {
            let p = self.grid[dim];
            let x = input[dim].clamp(0.0, 1.0);
            #[allow(clippy::cast_precision_loss)] // p ≤ 65536ish, exact
            let pos = x * (p - 1) as f64;
            // Same clamped-index-then-fraction pairing as the 1-D
            // curve evaluator — the bug class caught there on
            // 2026-08-11 (TRC(1.0) landing on the previous sample) is
            // exactly reproducible here, so the fix is inherited, not
            // re-derived.
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let idx = (pos.floor() as usize).min(p - 2);
            #[allow(clippy::cast_precision_loss)]
            let fr = pos - idx as f64;
            base_idx[dim] = idx;
            frac[dim] = fr;
        }

        out.fill(0.0);
        let corners = 1usize << d;
        let mut idx = vec![0usize; d];
        for corner in 0..corners {
            let mut weight = 1.0f64;
            for dim in 0..d {
                let hi = (corner >> (d - 1 - dim)) & 1 == 1;
                idx[dim] = base_idx[dim] + usize::from(hi);
                weight *= if hi { frac[dim] } else { 1.0 - frac[dim] };
            }
            if weight == 0.0 {
                continue;
            }
            let base = self.node_base(&idx);
            for (o, slot) in out.iter_mut().enumerate() {
                *slot += weight * self.samples[base + o];
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Interpolation at grid nodes returns the stored samples exactly
    /// — arithmetic identity (every weight is 0 or 1 at a node).
    #[test]
    fn exact_at_grid_nodes() {
        // 3×2×2 grid, 2 outputs, distinct values per node.
        let grid = vec![3usize, 2, 2];
        let n: usize = grid.iter().product();
        // Tiny test values: the casts are exact (all < 2^52).
        #[allow(clippy::cast_precision_loss)]
        let samples: Vec<f64> = (0..n * 2).map(|i| i as f64).collect();
        let clut = Clut::new(grid.clone(), 2, samples.clone()).unwrap();
        let mut out = [0.0f64; 2];
        for i0 in 0..3usize {
            for i1 in 0..2usize {
                for i2 in 0..2usize {
                    #[allow(clippy::cast_precision_loss)]
                    let input = [i0 as f64 / 2.0, i1 as f64, i2 as f64];
                    assert!(clut.eval(&input, &mut out));
                    let base = ((i0 * 2 + i1) * 2 + i2) * 2;
                    assert_eq!(
                        out,
                        [samples[base], samples[base + 1]],
                        "node {i0},{i1},{i2}"
                    );
                }
            }
        }
    }

    /// n-linear reproduces multilinear functions exactly — the
    /// function class the spec's own two-grid-point sentence
    /// contemplates (10.10: data "set so that the correct results are
    /// obtained when linear interpolation is used"). Arithmetic
    /// identity: f(x,y,z) = 2x + 3y + 5z + xyz sampled on a 2³ grid.
    #[test]
    fn reproduces_multilinear_exactly() {
        let f = |x: f64, y: f64, z: f64| 2.0 * x + 3.0 * y + 5.0 * z + x * y * z;
        let mut samples = Vec::new();
        for &x in &[0.0, 1.0] {
            for &y in &[0.0, 1.0] {
                for &z in &[0.0, 1.0] {
                    samples.push(f(x, y, z));
                }
            }
        }
        let clut = Clut::new(vec![2, 2, 2], 1, samples).unwrap();
        let mut out = [0.0f64];
        for &(x, y, z) in &[(0.3, 0.7, 0.1), (0.5, 0.5, 0.5), (0.99, 0.01, 0.6)] {
            assert!(clut.eval(&[x, y, z], &mut out));
            assert!(
                (out[0] - f(x, y, z)).abs() < 1e-12,
                "f({x},{y},{z}) = {}, got {}",
                f(x, y, z),
                out[0]
            );
        }
    }

    /// Index ordering: FIRST channel slowest (A20, primary spec). An
    /// asymmetric 2×3 grid where the value encodes the node identity
    /// distinguishes the two orderings — the transposed reading would
    /// return a different node's value.
    #[test]
    fn first_channel_varies_slowest() {
        // grid = [2, 3]: node (i0, i1) stored at flat i0*3 + i1.
        // Value = 10*i0 + i1 encodes identity.
        let samples = vec![0.0, 1.0, 2.0, 10.0, 11.0, 12.0];
        let clut = Clut::new(vec![2, 3], 1, samples).unwrap();
        let mut out = [0.0f64];
        // Node (1, 2): input (1.0, 1.0) → must be 12, not 21-style mixup.
        assert!(clut.eval(&[1.0, 1.0], &mut out));
        assert_eq!(out[0], 12.0);
        // Node (0, 1): input (0.0, 0.5) → 1.
        assert!(clut.eval(&[0.0, 0.5], &mut out));
        assert_eq!(out[0], 1.0);
    }

    /// The clamped-index/unclamped-fraction bug class (caught in the
    /// 1-D evaluator) stays fixed here: input exactly 1.0 returns the
    /// LAST node, not the second-to-last.
    #[test]
    fn input_one_hits_last_node() {
        let samples = vec![0.0, 0.25, 0.5, 1.0]; // 4-point 1-D grid
        let clut = Clut::new(vec![4], 1, samples).unwrap();
        let mut out = [0.0f64];
        assert!(clut.eval(&[1.0], &mut out));
        assert_eq!(out[0], 1.0);
    }

    /// Construction invariants refuse bad shapes by name.
    #[test]
    fn construction_invariants() {
        assert_eq!(Clut::new(vec![], 1, vec![]), Err(ClutError::NoDimensions));
        assert_eq!(
            Clut::new(vec![2, 1], 1, vec![0.0, 0.0]),
            Err(ClutError::DimensionTooSmall { dim: 1, points: 1 })
        );
        assert_eq!(
            Clut::new(vec![2, 2], 1, vec![0.0; 3]),
            Err(ClutError::SampleCountMismatch {
                expected: 4,
                actual: 3
            })
        );
    }
}
