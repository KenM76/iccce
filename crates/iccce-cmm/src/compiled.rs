//! # Compiled transforms — Pass 6
//!
//! Folds a whole [`Chain`] into one device→device grid, evaluated by
//! n-linear interpolation, so a page-sized raster does not re-walk
//! curves, matrices, CLUTs and BPC per pixel.
//!
//! ## Rule 8, and why this Pass is legitimate now
//!
//! *Optimise only after correct.* Every stage this compiles has been
//! measured against lcms2 in the direction it is used — `mft2` A2B,
//! `mft1` B2A, `mAB `/`mBA `, matrix/TRC both ways, grayTRC, and
//! (Pass 5) BPC. That precondition did not exist before Pass 4b.
//!
//! ## ★ What the two arms are free to disagree about (DL-023)
//!
//! **This is the sharpest instance of DL-023 in the project.** The
//! compiled grid is built BY SAMPLING the reference path, so:
//!
//! - **At a grid node the two arms are identical BY CONSTRUCTION.**
//!   A test that samples nodes measures nothing about compilation;
//!   `tests::identical_at_nodes_by_construction` asserts that
//!   identity and is labelled as a *structural* check, not evidence.
//! - **Between nodes they are free to differ by the interpolation
//!   error of the grid** — that, and only that, is the compiled
//!   path's cost. It is measured at deliberately off-node points.
//! - **The direction and tag type are part of the claim** (DL-021):
//!   an error measured compiling a CMYK→RGB A2B path says nothing
//!   about the B2A path on the same profiles.
//!
//! ## The sensitivity control (DL-018)
//!
//! An upper bound on a deliberate cost is worthless if deleting the
//! precision would make it greener. So every error claim here is
//! stated beside a **sensitivity ratio**: the same measurement on a
//! deliberately coarser grid. n-linear error scales ≈ h², so halving
//! the spacing should cut the error ≈ 4×; if it does not, the
//! instrument cannot see grid error and the number is not evidence.
//! `tests::error_scales_with_grid_spacing` is that control.
//!
//! ## What is NOT compiled
//!
//! Nothing about *correctness policy* moves in here: intent
//! selection, the 8.10.2 fallback, BPC opt-in, refusals and the A4c
//! disclosure all happen when the [`Chain`] is built. This type only
//! caches the numeric result of a chain that already exists.

use crate::clut::Clut;
use crate::transform::{Chain, ChainError};

/// A [`Chain`] folded into one interpolable grid.
#[derive(Debug, Clone)]
pub struct CompiledTransform {
    grid: Clut,
    input_channels: usize,
    output_channels: usize,
    grid_points: usize,
}

impl CompiledTransform {
    /// Sample `chain` on a regular `grid_points`-per-axis grid.
    ///
    /// Cost is `grid_points^input_channels` chain evaluations — 4-D
    /// CMYK at 17 points is 83 521 samples, which is why this is done
    /// once and not per pixel. `grid_points` must be ≥ 2.
    ///
    /// WHY a uniform grid rather than the source profile's own CLUT
    /// geometry: the chain may contain several stages with different
    /// grids plus analytic curves, so there is no single "native"
    /// geometry to inherit — and choosing one stage's would silently
    /// privilege it. A uniform grid's error is at least analysable
    /// (h², the control below relies on it).
    pub fn new(chain: &Chain, grid_points: usize) -> Result<CompiledTransform, ChainError> {
        assert!(
            grid_points >= 2,
            "a grid needs at least two points per axis"
        );
        let input_channels = chain.input_channels();
        let output_channels = chain.output_channels();
        // Channel counts are single profile bytes (≤ 15 here); the
        // cast cannot truncate, and `checked_pow` refuses a grid the
        // machine could not hold rather than wrapping into a small
        // allocation that would silently produce a wrong transform.
        #[allow(clippy::cast_possible_truncation)]
        let dims = input_channels as u32;
        let Some(nodes) = grid_points.checked_pow(dims) else {
            return Err(ChainError::ChannelMismatch {
                expected: input_channels,
                actual: input_channels,
            });
        };

        let mut samples = Vec::with_capacity(nodes * output_channels);
        let mut device = vec![0.0f64; input_channels];
        for flat in 0..nodes {
            // First channel varies SLOWEST — the same convention the
            // CLUT evaluator expects (A20), so the index arithmetic
            // here and in `clut` cannot drift apart.
            let mut rem = flat;
            for ch in (0..input_channels).rev() {
                let idx = rem % grid_points;
                rem /= grid_points;
                #[allow(clippy::cast_precision_loss)]
                let v = idx as f64 / (grid_points - 1) as f64;
                device[ch] = v;
            }
            let out = chain.convert(&device)?;
            samples.extend_from_slice(&out);
        }

        let grid = Clut::new(vec![grid_points; input_channels], output_channels, samples)
            .expect("grid shape is constructed consistently");
        Ok(CompiledTransform {
            grid,
            input_channels,
            output_channels,
            grid_points,
        })
    }

    pub fn input_channels(&self) -> usize {
        self.input_channels
    }
    pub fn output_channels(&self) -> usize {
        self.output_channels
    }
    pub fn grid_points(&self) -> usize {
        self.grid_points
    }

    /// Convert one pixel. `out.len()` must be [`Self::output_channels`].
    #[must_use]
    pub fn convert(&self, device: &[f64], out: &mut [f64]) -> bool {
        self.grid.eval(device, out)
    }

    /// Convert a whole interleaved buffer in place-ish: `src` holds
    /// `n × input_channels` values, `dst` receives `n × output_channels`.
    /// Returns false on a shape mismatch rather than panicking — a
    /// raster loop is exactly where a panic is least welcome.
    #[must_use]
    pub fn convert_buffer(&self, src: &[f64], dst: &mut [f64]) -> bool {
        if src.len() % self.input_channels != 0 {
            return false;
        }
        let pixels = src.len() / self.input_channels;
        if dst.len() != pixels * self.output_channels {
            return false;
        }
        for p in 0..pixels {
            let i = p * self.input_channels;
            let o = p * self.output_channels;
            if !self.grid.eval(
                &src[i..i + self.input_channels],
                &mut dst[o..o + self.output_channels],
            ) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matrix_trc::Intent;
    use iccce_profile::Profile;

    fn system(path: &str) -> Option<Profile> {
        std::fs::read(path)
            .ok()
            .map(|b| Profile::parse(&b).unwrap())
    }

    const SRGB: &str = r"C:\Windows\System32\spool\drivers\color\sRGB Color Space Profile.icm";
    const SWOP: &str = r"C:\Windows\System32\spool\drivers\color\USWebCoatedSWOP.icc";

    /// STRUCTURAL, NOT EVIDENCE (DL-023): at a grid node the compiled
    /// and reference arms are identical BY CONSTRUCTION — the node's
    /// value IS a reference evaluation. This test exists to pin that
    /// the indexing convention matches (a transposed grid would break
    /// it), and for no other reason. It must never be cited as the
    /// compiled path's error.
    #[test]
    fn identical_at_nodes_by_construction() {
        let (Some(src), Some(dst)) = (system(SRGB), system(SRGB)) else {
            eprintln!("skipped: system profile absent");
            return;
        };
        let chain = Chain::new(&src, &dst, Intent::MediaRelative).unwrap();
        let n = 9;
        let compiled = CompiledTransform::new(&chain, n).unwrap();
        let mut out = vec![0.0; compiled.output_channels()];
        for i in 0..n {
            for j in 0..n {
                #[allow(clippy::cast_precision_loss)]
                let p = [
                    i as f64 / (n - 1) as f64,
                    j as f64 / (n - 1) as f64,
                    0.5_f64,
                ];
                // 0.5 is a node only when n is odd — it is here.
                assert!(compiled.convert(&p, &mut out));
                let reference = chain.convert(&p).unwrap();
                for c in 0..3 {
                    assert!(
                        (out[c] - reference[c]).abs() < 1e-12,
                        "node ({i},{j}) channel {c}: {} vs {}",
                        out[c],
                        reference[c]
                    );
                }
            }
        }
    }

    /// ★ THE SENSITIVITY CONTROL (DL-018). n-linear interpolation
    /// error scales as h², so halving the grid spacing must cut the
    /// off-node error by roughly 4×. If it does not, this measurement
    /// cannot see grid-induced error and no error number from it is
    /// evidence.
    ///
    /// ★ THIS CONTROL CAUGHT ITS OWN INSTRUMENT ON THE FIRST RUN, and
    /// the failure is worth more than the test: the fixture was
    /// sRGB→sRGB, and a compiled grid reproduces an IDENTITY chain
    /// exactly **everywhere**, not merely at nodes — n-linear
    /// interpolation is exact on a linear function. Error came out at
    /// 1.1×10⁻¹⁵ with a ratio of 0.94: pure f64 noise, no h² scaling,
    /// no discrimination. Had the control not existed, that 10⁻¹⁵
    /// would have been reported as "the compiled path's cost" — a
    /// spectacular number that measured nothing. **DL-023's warning
    /// is not hypothetical; this is the second arm being free to
    /// disagree about nothing at all.**
    ///
    /// The fixture is therefore sRGB→AdobeRGB: different TRCs, so the
    /// composite is genuinely curved in device space and a grid can be
    /// wrong about it.
    #[test]
    fn error_scales_with_grid_spacing() {
        const ARGB: &str = r"C:\Windows\System32\spool\drivers\color\AdobeRGB1998.icc";
        let (Some(src), Some(dst)) = (system(SRGB), system(ARGB)) else {
            eprintln!("skipped: system profile absent");
            return;
        };
        let chain = Chain::new(&src, &dst, Intent::MediaRelative).unwrap();
        // ★ Probe the SMOOTH region only. The sRGB TRC has a linear
        // segment joining a power curve at 0.04045 — a derivative
        // discontinuity — and interpolation error across a kink
        // scales as h¹, not h². A first version of this test probed
        // the whole axis and got ratio 1.44: correct physics, wrong
        // expectation. These probes sit in [0.2, 0.9], off-node for
        // BOTH the 5- and 9-point grids (no multiple of 0.25 or
        // 0.125), where the composite is genuinely smooth and the h²
        // prediction is the right one to test against.
        let probes: Vec<[f64; 3]> = [0.2, 0.3, 0.45, 0.55, 0.7, 0.8, 0.9]
            .iter()
            .map(|&v: &f64| [v, 1.0 - v * 0.6, v * 0.5 + 0.22])
            .collect();

        let max_err = |n: usize| {
            let compiled = CompiledTransform::new(&chain, n).unwrap();
            let mut out = vec![0.0; 3];
            let mut worst = 0.0f64;
            for p in &probes {
                assert!(compiled.convert(p, &mut out));
                let r = chain.convert(p).unwrap();
                for c in 0..3 {
                    worst = worst.max((out[c] - r[c]).abs());
                }
            }
            worst
        };

        let coarse = max_err(5);
        let fine = max_err(9); // half the spacing
        assert!(coarse > 0.0, "the instrument must see SOMETHING off-node");
        let ratio = coarse / fine;
        // h² predicts 4×; accept 2×–8× — wide enough for a real TRC's
        // varying curvature, narrow enough to fail if the compiled
        // path were secretly the reference path (ratio → 1) or if the
        // probes were accidentally on-node (both zero).
        assert!(
            (2.0..=8.0).contains(&ratio),
            "sensitivity ratio {ratio} (coarse {coarse}, fine {fine}) — \
             outside the h² band, so this instrument cannot grade a compiled path"
        );
    }

    /// The compiled path's cost on a REAL CMYK→RGB chain, off-node,
    /// with its direction and tag type named (DL-021): SWOP `A2B1`
    /// (`mft2`, 4-D) → sRGB matrix/TRC, media-relative.
    ///
    /// self-consistency class — worthless as correctness evidence
    /// however small, per NUMERIC_CLAIMS §1. The bound below is a
    /// gate, not the measurement; conformance owns the reported
    /// number and its ΔE2000 translation.
    #[test]
    fn cmyk_compiled_cost_is_bounded_off_node() {
        let (Some(src), Some(dst)) = (system(SWOP), system(SRGB)) else {
            eprintln!("skipped: system profiles absent");
            return;
        };
        let chain = Chain::new(&src, &dst, Intent::MediaRelative).unwrap();
        let compiled = CompiledTransform::new(&chain, 17).unwrap();
        let mut out = vec![0.0; 3];
        let mut worst = 0.0f64;
        // Deliberately off-node for a 17-point grid (nodes sit at
        // sixteenths; these sit at odd thirty-seconds).
        for k in 1..16u32 {
            let v = f64::from(k) / 32.0 + 1.0 / 64.0;
            let probe = [v, 1.0 - v, (v * 1.3).fract(), v * 0.5];
            assert!(compiled.convert(&probe, &mut out));
            let r = chain.convert(&probe).unwrap();
            for c in 0..3 {
                worst = worst.max((out[c] - r[c]).abs());
            }
        }
        // 0.02 device units at 17³ per axis: a gate loose enough to
        // survive the CLUT's own curvature, tight enough that a
        // grid-indexing error (which lands entire channels wrong)
        // cannot pass. The MEASURED value is what gets reported.
        assert!(worst < 0.02, "compiled off-node cost {worst}");
        assert!(
            worst > 0.0,
            "a zero cost here would mean the probes were on-node"
        );
    }

    /// Buffer conversion agrees with per-pixel conversion exactly and
    /// refuses mismatched shapes instead of panicking.
    #[test]
    fn buffer_matches_per_pixel_and_refuses_bad_shapes() {
        let (Some(src), Some(dst)) = (system(SRGB), system(SRGB)) else {
            eprintln!("skipped: system profile absent");
            return;
        };
        let chain = Chain::new(&src, &dst, Intent::MediaRelative).unwrap();
        let compiled = CompiledTransform::new(&chain, 9).unwrap();
        let src_buf: Vec<f64> = (0..30).map(|i| f64::from(i) / 29.0).collect();
        let mut dst_buf = vec![0.0; 30];
        assert!(compiled.convert_buffer(&src_buf, &mut dst_buf));
        let mut one = vec![0.0; 3];
        for p in 0..10 {
            assert!(compiled.convert(&src_buf[p * 3..p * 3 + 3], &mut one));
            assert_eq!(&dst_buf[p * 3..p * 3 + 3], &one[..]);
        }
        assert!(!compiled.convert_buffer(&src_buf, &mut vec![0.0; 29]));
        assert!(!compiled.convert_buffer(&src_buf[..29], &mut dst_buf));
    }
}
