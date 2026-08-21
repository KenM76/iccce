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

/// Grid points per axis that keep the compiled path's cost inside
/// what the two implementations already differ by on the same
/// transform — measured, not chosen.
///
/// ★ MEASURED (icc-conformance, 2026-08-12, SWOP `A2B1` → sRGB,
/// media-relative, ΔE2000 against the reference path):
///
/// | grid | ΔE2000 | build |
/// |---|---|---|
/// | 5 | 0.728 | — |
/// | 9 | 0.405 | — |
/// | **17** | **0.297** | 1.0 s |
/// | **33** | **0.168** | 13.8 s |
///
/// The gate is Pass 4's own measured iccce-vs-lcms2 agreement on this
/// pair, **0.25294 ΔE2000**: compiling must not move the answer
/// further than the two implementations already disagree. **17 FAILS
/// that gate by 17 %** — which is why this constant is 33 for 4-D and
/// not the 17 the first implementation defaulted to.
///
/// The convergence order is **1.32, not the h²=2 a smooth function
/// would give**: SWOP's `mft2` carries 256-entry input tables read by
/// linear interpolation, i.e. 255 derivative kinks per axis, and
/// `gcd(255, N) = 1` for every N tried, so no grid ever lands on
/// them. **Doubling the grid costs ~15× the build and buys ~2.5×.**
/// A caller who needs better than 0.168 should be told the honest
/// answer — that this is the wrong lever — rather than handed 65.
pub const fn recommended_grid_points(input_channels: usize) -> usize {
    match input_channels {
        // 1-D and 2-D are cheap; spend the nodes.
        0..=2 => 129,
        // 3-D at 33 is 35 937 nodes — the industry-standard size.
        3 => 33,
        // 4-D at 33 is 1 185 921 nodes, and is what the measurement
        // above says is needed.
        //
        // ★★ COST, RE-MEASURED 2026-08-21 AND NOW LABELLED, because the
        // previous version of this comment said "~27 MB, ~14 s to build"
        // and BOTH halves were misleading — each in a different way, and
        // each in the direction that makes a grid look more expensive
        // than it is.
        //
        //   build, RELEASE, four committed synthetic CMYK sources
        //       1.419442 / 1.439321 / 1.585943 / 1.783205 seconds
        //   build, DEBUG, the same profile pair as the 1.419442 above
        //       14.323247 seconds
        //
        // ★★★ 10.1x, one profile pair, one machine, one afternoon. The
        // old "~14 s" was a DEBUG measurement with no build profile
        // named. An unlabelled timing is not merely stale — it is wrong
        // by an order of magnitude for every reader who assumes the
        // other profile, and this one sat inside the function that
        // decides how many nodes to spend. A performance number without
        // its build profile is not a number.
        //
        // And the memory figure depended on a variable this function
        // cannot see. Grid storage is nodes x OUTPUT channels x 8 bytes,
        // and `recommended_grid_points` takes INPUT channels only:
        //
        //   CMYK -> RGB   1 185 921 x 3 x 8  =  27.2 MiB   <- the old "~27 MB"
        //   CMYK -> CMYK  1 185 921 x 4 x 8  =  36.2 MiB
        //
        // So the old figure was one destination shape stated as if it
        // were the grid's cost. Both rows are DERIVED from the node
        // count, not measured by instrumenting the allocator.
        4 => 33,
        // ★★ FIVE CHANNELS AND UP — COMPUTED, not tabulated, and a
        // MEMORY bound rather than an accuracy result. See the note on
        // [`MAX_COMPILED_GRID_BYTES`] for why these are a different kind
        // of number from the 33 above.
        //
        // Computed rather than hand-written because a table of fourteen
        // hand-evaluated powers is fourteen chances to typo a number
        // that no test could distinguish from a correct one — and the
        // first version of this function shipped exactly that class of
        // error as a `_ => 33` catch-all.
        channels => {
            let mut n = 33;
            while n > 2 && !grid_fits_budget(n, channels) {
                n -= 1;
            }
            n
        }
    }
}

/// Would an `n`-per-axis grid over `channels` inputs fit the budget, at
/// the worst output width ICC.1 permits?
///
/// ★ Uses **15 output channels** — `FCLR`, Table 19's ceiling — rather
/// than the 3 of an RGB destination. The recommendation is consumed
/// before the destination is known in some call paths, so sizing it for
/// the actual output width would make the default's safety depend on
/// something the caller has not told us yet. Sizing for the worst case
/// costs a slightly smaller grid and removes the dependency.
const fn grid_fits_budget(n: usize, channels: usize) -> bool {
    // A channel count above u32::MAX is impossible (ICC.1 Table 19 tops
    // out at 15), but this is a `const fn` with no place to assert, so
    // the truncating case is handled as "does not fit" rather than
    // silently wrapping to a small exponent and reporting a huge grid as
    // affordable.
    if channels > 64 {
        return false;
    }
    #[allow(clippy::cast_possible_truncation)]
    let dims = channels as u32;
    let Some(nodes) = n.checked_pow(dims) else {
        return false;
    };
    let Some(samples) = nodes.checked_mul(15) else {
        return false;
    };
    let Some(bytes) = samples.checked_mul(8) else {
        return false;
    };
    bytes <= MAX_COMPILED_GRID_BYTES
}

/// ★★ Why the ≥5-channel recommendations above are a different KIND of
/// number from the ones below them, and must never be quoted as though
/// they were the same.
///
/// **The 33 for 3-D and 4-D is a measured result.** It is gated on Pass
/// 4's iccce-vs-lcms2 agreement on a real profile pair, and 17 was
/// rejected because it failed that gate by 17 %. It carries a ΔE number.
///
/// **The values for 5 channels and up are not.** No measurement exists
/// for them, because until 2026-08-17 this project had never seen a
/// profile with more than four channels, and the one it now has — ICC's
/// `APTEC_CMYKOGV_Coated_LinearCTV_2025.icc`, `7CLR` — has no second
/// implementation to be gated against on the compiled path. They are
/// **computed** as the largest grid that fits this budget at the worst
/// output width ICC.1 permits, and nothing more:
///
/// | channels | grid | nodes |
/// |---|---|---|
/// | 5 | 14 | 537 824 |
/// | 6 | 9 | 531 441 |
/// | 7 | 6 | 279 936 |
/// | 8 | 5 | 390 625 |
/// | 9 | 4 | 262 144 |
/// | 10–12 | 3 | 59 049 … 531 441 |
/// | 13–15 | 2 | 8 192 … 32 768 |
///
/// *(The table is illustrative; the function computes these — see its
/// own note on why a hand-written table was the wrong shape here.)*
///
/// ## ★★ The one tension, stated rather than hidden
///
/// **The measured 33 for 4 channels does NOT fit this budget at 15
/// output channels** — `33^4 × 15 × 8` is ~136 MiB. That is deliberate
/// and is the right way round:
///
/// - **A measured value is not weakened to satisfy a memory bound.**
///   The 33 is gated on a ΔE result; shrinking it to fit an arbitrary
///   byte budget would discard evidence in favour of convenience.
/// - **The budget still protects the process**, because the guard in
///   [`CompiledTransform::new`] uses the *actual* output width, not the
///   worst case. CMYK → RGB at grid 33 is ~27 MiB and builds fine. Only
///   a CMYK → 15-channel destination would exceed, and that is then a
///   **named refusal**, which is all this budget was ever for.
///
/// So the worst-case sizing applies where there is no measurement to
/// protect, and the measurement wins where there is one.
///
/// ★ **This was a catch-all `_ => 33` until 2026-08-17, and the catch-all
/// is the whole story.** Its doc comment reasoned carefully about 3-D and
/// 4-D and then silently applied its conclusion to every higher
/// dimension. At 7 channels that is `33^7` nodes ≈ **0.93 TiB**, and the
/// observed result was not a slow build or a bad number — the process
/// **aborted**. A constant justified for one regime, extended by a
/// wildcard to regimes nobody measured, is the same defect class as an
/// unstated approximation (rule 4): it looks like a decision and is
/// actually an absence of one.
///
/// **What a caller who cares should do:** pass an explicit grid. These
/// values will not produce a wrong answer — the compiled path's error
/// against the reference path is measurable at any grid, and
/// `iccce bench` prints it — but they carry **no ΔE claim**, and
/// anything above four channels should be treated as unmeasured until
/// someone measures it.
pub const MAX_COMPILED_GRID_BYTES: usize = 64 * 1024 * 1024;

/// A [`Chain`] folded into one interpolable grid.
#[derive(Debug, Clone)]
pub struct CompiledTransform {
    grid: Clut,
    input_channels: usize,
    output_channels: usize,
    grid_points: usize,
    /// ★ The black-preservation policy, carried OUTSIDE the grid.
    ///
    /// The grid holds the smooth colorimetric answer only. This branch
    /// is applied per-pixel in [`CompiledTransform::convert`], exactly
    /// as `Chain::convert` applies it — see
    /// `Chain::convert_colorimetric` for the measurement that forced
    /// this shape (0.617 of wrong ink within one cell of the neutral
    /// axis, and refining the grid did not move it).
    k_preserve: Option<crate::black_preserve::KPreserve>,
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
            return Err(ChainError::GridTooLarge {
                grid_points,
                dimensions: input_channels,
            });
        };

        // ★★ The SIZE guard, distinct from the overflow guard above.
        //
        // `checked_pow` catches a node count that will not fit in a
        // `usize`. It does NOT catch one that fits perfectly well as a
        // number and is far too large to allocate — and on a 64-bit
        // machine that is nearly every interesting case.
        //
        // Found 2026-08-17 by Pass H, on a real file: ICC's published
        // `APTEC_CMYKOGV_Coated_LinearCTV_2025.icc` is a SEVEN-channel
        // (`7CLR`) press profile. At the recommended 33 points per axis
        // that is 33^7 = 42_618_442_977 nodes, which is a perfectly
        // ordinary `usize`, times 3 output channels times 8 bytes =
        // **1_022_842_631_448 bytes, about 0.93 TiB**.
        //
        // ★ The observed behaviour was the worst possible one: `Vec`'s
        // allocation failure **aborts the process** — bare exit
        // `0xC0000409`, stderr "memory allocation of 1022842631448 bytes
        // failed", stdout empty. Not an `Err`, not a panic a caller can
        // catch: process death. For a *library* that is unacceptable
        // regardless of the number involved, because it takes the
        // consumer's process down with it and the consumer had no way to
        // see it coming.
        //
        // So the budget below is not primarily about memory management;
        // it is about **converting an abort into a named refusal**,
        // which is rule 6 at the allocation layer. A caller who genuinely
        // wants a bigger grid is not blocked by an accident of arithmetic
        // — they are told the number and can decide.
        let Some(sample_count) = nodes.checked_mul(output_channels) else {
            return Err(ChainError::GridTooLarge {
                grid_points,
                dimensions: input_channels,
            });
        };
        let Some(bytes) = sample_count.checked_mul(std::mem::size_of::<f64>()) else {
            return Err(ChainError::GridTooLarge {
                grid_points,
                dimensions: input_channels,
            });
        };
        if bytes > MAX_COMPILED_GRID_BYTES {
            return Err(ChainError::GridExceedsBudget {
                nodes,
                bytes,
                budget_bytes: MAX_COMPILED_GRID_BYTES,
            });
        }

        let mut samples = Vec::with_capacity(sample_count);
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
            // ★ `convert_colorimetric`, NOT `convert`: the grid must
            // hold the smooth answer. Sampling the preserving
            // conversion bakes a step discontinuity into data that is
            // about to be linearly interpolated, and no grid density
            // fixes it — measured at O(1) in h. See
            // `Chain::convert_colorimetric`'s doc comment.
            let out = chain.convert_colorimetric(&device)?;
            samples.extend_from_slice(&out);
        }

        let grid = Clut::new(vec![grid_points; input_channels], output_channels, samples)
            .expect("grid shape is constructed consistently");
        Ok(CompiledTransform {
            grid,
            input_channels,
            output_channels,
            grid_points,
            k_preserve: chain.k_preserve_built().cloned(),
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
        // ★ The policy runs BEFORE the grid and never through it. The
        // exact-zero test is meaningful only on the caller's actual
        // input; once a value has been interpolated between nodes it is
        // no longer exactly zero and the question cannot be asked.
        if let Some(kp) = &self.k_preserve {
            if let Some(preserved) = kp.apply(device) {
                if out.len() != preserved.len() {
                    return false;
                }
                out.copy_from_slice(&preserved);
                return true;
            }
        }
        self.grid.eval(device, out)
    }

    /// Convert a whole interleaved buffer in place-ish: `src` holds
    /// `n × input_channels` values, `dst` receives `n × output_channels`.
    /// Returns false on a shape mismatch rather than panicking — a
    /// raster loop is exactly where a panic is least welcome.
    ///
    /// # This is [`Self::convert`] in a loop, and that is a contract
    ///
    /// ★★★ **It delegates to [`Self::convert`] rather than reproducing
    /// its body, and that is deliberate rather than stylistic.** From
    /// its introduction until 2026-08-21 this function called
    /// `self.grid.eval` directly and therefore **never consulted the
    /// K-preservation policy at all**. The consequence was two answers
    /// from one `CompiledTransform`: a caller who asked for black
    /// preservation and evaluated one pixel at a time got it, and the
    /// same caller evaluating the identical values as a buffer did not.
    ///
    /// Measured at the fix, on `v2-cmyk-chromatic-neutral`, grid 17:
    ///
    /// ```text
    ///   worst |convert - convert_buffer|   7.195269e-1 of ink
    ///   at input                           [0.0, 0.0, 0.0, 1.0]
    ///   the same on non-qualifying probes  0.0   (exact)
    /// ```
    ///
    /// **Seventy-two percent of a colorant channel**, on solid black, on
    /// the entry point a renderer would actually use — and the control
    /// was exactly zero, so it was the policy and nothing else. For the
    /// perceptual size of what was being discarded see `NC-269`
    /// (`3.681203` ΔE2000 max, `1.580674` mean, for a real press pair).
    ///
    /// ★★ **Why nothing caught it, and this is the transferable half.**
    /// The only caller of this function in the whole repository is
    /// `iccce bench` (`crates/iccce-cli/src/main.rs:614`), and `bench`
    /// builds its chain with a bare [`crate::transform::Chain::new`] —
    /// **it cannot request black preservation at all.** So the defect
    /// was unreachable from the CLI, and `difftest` drives the CLI.
    /// **A defect that only a LIBRARY consumer can reach is invisible
    /// to a CLI-driven suite**, however green that suite is. The unit
    /// tests that did exist all evaluated through
    /// [`Self::convert`], so the two entry points were never once
    /// compared to each other.
    ///
    /// The lesson is in the shape, not the bug: **one struct offering
    /// two evaluation surfaces owes callers a guarantee that they are
    /// the same transform.** The only way to keep that guarantee under
    /// future edits is for one to be defined in terms of the other, so
    /// there is no second place for a policy to be forgotten. The
    /// pattern was already in this crate —
    /// `MatrixTrcTransform::convert` is a one-line delegation to
    /// `convert_with_intent` — so this was an inconsistently applied
    /// technique rather than an unknown one.
    /// `crates/iccce-cmm/tests/compiled_buffer_agrees_with_single_pixel.rs`
    /// asserts it by bit-equality, which is the right strength of claim
    /// here because both paths run the same arithmetic on the same
    /// inputs — any difference is structural, so a tolerance would only
    /// be somewhere for the next instance to hide.
    ///
    /// # Cost, stated because rule 8 says optimise only after correct
    ///
    /// [`crate::black_preserve::KPreserve::apply`] returns an owned
    /// `Vec<f64>`, so a **qualifying** pixel now allocates. Only pixels
    /// with `C = M = Y = 0` exactly take that branch, and only on a
    /// transform built with [`crate::transform::Chain::with_black_preservation`]
    /// — every other pixel, and every transform without the policy, runs
    /// exactly the arithmetic it ran before. That allocation is a known
    /// and named cost of correctness here, **not** a measured throughput
    /// figure: no benchmark has been run against it, and none is claimed.
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
            if !self.convert(
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
        // PRINTED ALWAYS, not only on failure: icc-librarian's audit
        // noted that both FAILING ratios (0.94, 1.44) were on record
        // while the passing one was not — so nobody could tell whether
        // the control sits comfortably inside its band or at the edge.
        // `cargo test -- --nocapture` now records it.
        eprintln!(
            "sensitivity control: coarse(5)={coarse:.9} fine(9)={fine:.9} ratio={ratio:.4} (band 2.0-8.0)"
        );
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

    /// ★★ Regression: a grid that is arithmetically fine and far too
    /// large to allocate must be a NAMED REFUSAL, never a process abort.
    ///
    /// ## The defect this pins
    ///
    /// Found by Pass H on 2026-08-17, on ICC's own published
    /// seven-channel `APTEC_CMYKOGV_Coated_LinearCTV_2025.icc`. At the
    /// then-recommended 33 points per axis, `33^7 = 42_618_442_977`
    /// nodes — a perfectly ordinary `usize` — times 3 outputs times 8
    /// bytes is **1_022_842_631_448 bytes ≈ 0.93 TiB**.
    ///
    /// `checked_pow` did not catch it, because nothing overflowed. `Vec`
    /// then tried the allocation and **the process aborted**: bare exit
    /// `0xC0000409`, stderr "memory allocation of 1022842631448 bytes
    /// failed", stdout empty.
    ///
    /// ★ **An abort is the worst available failure for a library.** It
    /// is not an `Err` and not a catchable panic — it takes the
    /// consumer's process down, and the consumer had no way to see it
    /// coming. There is no tolerance to tune here and no number that
    /// could be moved: the graded property is that the failure is
    /// *reportable*.
    ///
    /// ## Why the assertion is on arithmetic rather than on a real build
    ///
    /// This test must not itself attempt the allocation it is checking
    /// against — a test that aborts the test process proves nothing and
    /// takes its siblings with it. So it asserts the guard's own
    /// arithmetic on the exact numbers from the real profile, plus the
    /// two properties that make the guard meaningful.
    #[test]
    fn oversized_grid_arithmetic_is_refused_not_aborted() {
        // The real case, from APTEC_CMYKOGV (7CLR) at grid 33.
        let nodes = 33usize
            .checked_pow(7)
            .expect("33^7 fits a usize — that is the whole point");
        assert_eq!(nodes, 42_618_442_977);
        let bytes = nodes * 3 * std::mem::size_of::<f64>();
        assert_eq!(bytes, 1_022_842_631_448);
        assert!(
            bytes > MAX_COMPILED_GRID_BYTES,
            "the budget must reject the case that aborted the process"
        );

        // ★ Every UNMEASURED recommendation (>=5 channels) must fit the
        // budget at the worst output width, so the default path can
        // never be the thing that refuses — let alone aborts.
        for channels in 5..=15usize {
            let g = recommended_grid_points(channels);
            let n = g
                .checked_pow(u32::try_from(channels).unwrap())
                .unwrap_or(usize::MAX);
            // 15 = 'FCLR', Table 19's ceiling — the width the
            // recommendation itself sizes for.
            let b = n.saturating_mul(15 * std::mem::size_of::<f64>());
            assert!(
                b <= MAX_COMPILED_GRID_BYTES,
                "recommended_grid_points({channels}) = {g} gives {n} nodes = {b} bytes, over                  the {MAX_COMPILED_GRID_BYTES}-byte budget — the DEFAULT path would refuse (or,                  before the budget existed, abort)"
            );
            assert!(g >= 2, "a grid needs at least two points per axis");
        }

        // ★★ And the deliberate exception, asserted so it stays
        // deliberate: the MEASURED 4-channel 33 does NOT fit the
        // worst-case budget, and must not be shrunk to make it. The
        // real guard uses the ACTUAL output width, where CMYK -> RGB is
        // ~27 MiB and builds fine. A measured value is not weakened to
        // satisfy a memory bound; see the note on MAX_COMPILED_GRID_BYTES.
        let four_d_worst = 33usize.pow(4) * 15 * std::mem::size_of::<f64>();
        assert!(
            four_d_worst > MAX_COMPILED_GRID_BYTES,
            "if 4-D at 33 now fits the worst case, this exception is stale and the note on              MAX_COMPILED_GRID_BYTES should be simplified rather than left claiming a tension              that no longer exists"
        );
        let four_d_rgb = 33usize.pow(4) * 3 * std::mem::size_of::<f64>();
        assert!(
            four_d_rgb <= MAX_COMPILED_GRID_BYTES,
            "CMYK -> RGB at the measured grid 33 must still build: {four_d_rgb} bytes"
        );

        // ★ The recommendation must still be the MEASURED 33 where a
        // measurement exists. A fix that made everything small would
        // have silently discarded Pass 4's result.
        assert_eq!(recommended_grid_points(3), 33, "3-D is a measured value");
        assert_eq!(recommended_grid_points(4), 33, "4-D is a measured value");
        assert_eq!(recommended_grid_points(1), 129);
    }
}
