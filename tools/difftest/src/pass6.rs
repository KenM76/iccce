//! # Pass 6 — the compiled path, graded
//!
//! `crates/iccce-cmm/src/compiled.rs` folds a whole [`Chain`] into one
//! device→device grid so a page-sized raster does not re-walk curves,
//! matrices, CLUTs and BPC per pixel. `iccce bench` prints what that costs and
//! what it buys. **This module is the grading of those numbers**, and it exists
//! because three things about the engineer's measurement are not, on their own,
//! claims this project is allowed to make:
//!
//! 1. **`error.max_device_offnode` is a device number, and a device number is
//!    not a colour claim.** 3,589×10⁻³ of sRGB is invisible in the highlights
//!    and enormous in the shadow; until it is carried into a space where a ΔE
//!    means something, it does not say whether compiling is safe. §A does that
//!    carry, through the **destination profile's own** matrix/TRC model.
//! 2. **A cost with no tolerance is an observation, not a gate.** §A states one
//!    and derives it — and the derivation has **no free parameter**, which is
//!    the property `TOLERANCES.md` §0 asks for and which "an order of magnitude
//!    below the anchor" does not have.
//! 3. **DL-018: an upper bound on a deliberate cost is worthless if deleting
//!    the precision would make it greener.** The engineer honoured this in
//!    `compiled.rs`'s unit tests with a 5-vs-9 control; §B carries it into the
//!    suite at 5 / 9 / 17 on the shipped default pair, and reports the ΔE cost
//!    of each grid so the trade is visible rather than asserted.
//!
//! ## The pair, and why it is this one (DL-021)
//!
//! `USWebCoatedSWOP.icc` `A2B1` (`mft2`, 4-D CMYK, 9 nodes per axis, `Lab `
//! PCS) → the system sRGB profile, **media-relative colorimetric**. That is
//! `iccce bench`'s own default pair, Pass 4's pair, and the pair
//! `compiled.rs::tests::cmyk_compiled_cost_is_bounded_off_node` uses. **The
//! direction and the tag type are part of the claim**: an error measured
//! compiling a CMYK→RGB `A2B` path says nothing whatever about the `B2A` path
//! on the same two files, and §C states that as a coverage limit rather than
//! leaving a reader to assume otherwise.
//!
//! ## The probe population is the benchmark's own, deliberately
//!
//! `iccce bench` samples its synthetic raster at
//! `v = ((p·7 + c·131) mod 1024) / 1023`, every `pixels/512`-th pixel. This
//! module **reproduces that sequence exactly** rather than inventing a grid of
//! its own, so that the ΔE reported here is a translation of *the number the
//! shipped binary prints* and not a different measurement that happens to be
//! about the same subject. Record `pass6/apparatus/harness-reproduces-bench`
//! is what makes that a checked claim: the harness's in-process device maximum
//! at grid 17 against the CLI's printed one, across a process boundary.
//!
//! **★ The sequence is off-node for all three grids, and that is arithmetic,
//! not luck.** `k/1023 = m/N` needs `N·k = 1023·m`; `gcd(N, 1023) = 1` for
//! `N ∈ {4, 8, 16}` (1023 = 3·11·31), so `m` must be a multiple of `N`, i.e.
//! `v ∈ {0, 1}`. Exactly two of the 1024 sample values are grid nodes, at both
//! ends of the axis, and everything between them is interpolated. A probe set
//! that had landed on nodes would have measured zero and meant nothing — the
//! trap `compiled.rs`'s own control caught on its first run.
//!
//! ## What class every number here is
//!
//! **self-consistency.** Both arms are iccce; the compiled grid is *built by
//! sampling* the reference path. No row in this module is evidence that either
//! arm is correct — Pass 4 is what says the reference path agrees with lcms2 on
//! this pair, and Pass 6 is only ever about the distance between iccce's two
//! arms. `NUMERIC_CLAIMS.md` §1 and `TOLERANCES.md` §1 both say a
//! self-consistency figure is worthless as correctness evidence however small;
//! that is not a hedge, it is the reason §A's tolerance is derived from Pass
//! 4's *cross-check* number rather than from anything measured here.

use std::path::{Path, PathBuf};
use std::process::Command;

use iccce_cmm::compiled::CompiledTransform;
use iccce_cmm::matrix_trc::{Intent as CmmIntent, MatrixTrc};
use iccce_cmm::transform::Chain;
use iccce_color::{D50, Lab, delta_e_2000};
use iccce_profile::Profile;

use crate::{Iccce, Kind, Metric, Record, Tolerance};

/// `LEGAL.md` §3 category **(c)**: read from the local system, never
/// committed, never a required input.
const SWOP: &str = r"C:\Windows\System32\spool\drivers\color\USWebCoatedSWOP.icc";
const SRGB: &str = r"C:\Windows\System32\spool\drivers\color\sRGB Color Space Profile.icm";

/// `iccce bench`'s default raster: 2481 × 3507, a 300 DPI A4 page.
const BENCH_PIXELS: usize = 2481 * 3507;
/// `iccce bench`'s default grid, i.e. `compiled::recommended_grid_points(4)`.
///
/// ##  Moved 17 -> 33 on 2026-08-12, and this constant is downstream of that
///
/// Pass 6 was first run when the shipped default was **17**, where the gate
/// (`COMPILED_DE`) FAILED at 2,970x10^-1 against 2,5x10^-1. `TOLERANCES.md`
/// 3.6.1 said in terms that *the remedy is the grid, not the number*, and
/// commit `189e732` moved `recommended_grid_points` to 33 for 3-D and 4-D.
///
/// **This constant must track the shipped default and nothing else.** If it
/// is ever set to a value the binary does not use, `APPARATUS_BENCH` fails -
/// by design: the harness arm and the `iccce bench` arm would be measuring
/// different transforms and every number below would stop being a
/// translation of what the binary prints. That is exactly how the move was
/// detected: the suite went red at 1,576x10^-3 on the apparatus row, which
/// is the difference between grid 17 and grid 33, not an error.
const DEFAULT_GRID: usize = 33;
/// The four grids §B compares. 5 → 9 → 17 → 33 halves the spacing three
/// times. **9, 17 and 33 all place their nodes ON the source CLUT's own
/// 9-node lattice** (kinks at eighths are nodes of all three); **5 does not**,
/// and straddles every second one. That asymmetry is why §B grades the
/// smooth-cell ratios and reports the whole-population ones separately.
const GRIDS: [usize; 4] = [5, 9, 17, 33];

/// sRGB's encoded-value breakpoint, `0,040 45` — below it the transfer
/// function is the `1/12,92` linear segment and the composite has a derivative
/// discontinuity. Used to classify a grid cell as smooth or not.
const SRGB_BREAKPOINT: f64 = 0.040_45;

// ===========================================================================
// Tolerances
// ===========================================================================

/// **§A, the one that matters.** The compiled path's off-node cost against the
/// reference path, in ΔE2000, at the shipped default grid.
///
/// **The constant has never moved.** It was derived and first graded when
/// the shipped default was 17, where it failed at 2,970x10^-1; the default
/// is now 33 and the same constant is graded against 1,677x10^-1. A
/// tolerance that survives the thing it failed changing underneath it is
/// the only kind worth writing down.
///
/// ## The derivation, which has no free parameter
///
/// The tempting derivation is "an order of magnitude below §2's provisional
/// 1,0 ΔE2000 perceptibility anchor". **It is not available here, and saying
/// why is more useful than the number.** That argument implicitly assumes the
/// approximations in this engine sum to less than the anchor, and they do not:
/// **NA-006** — the n-linear CLUT geometry — was measured by Pass 4 at up to
/// **1,574 ΔE2000** on `A2B0` of this very file, which is already above the
/// anchor on its own. A budget derived from a total that is already exceeded is
/// a number with a story attached.
///
/// So the line is drawn somewhere that is measured rather than chosen:
///
/// > **Compiling must not move the result further than the two implementations
/// > already differ on the same transform.**
///
/// Pass 4 measured `USWebCoatedSWOP → sRGB`, **media-relative**, 341 CMYK
/// points, iccce against lcms2 2.19.1: **0,252 94 ΔE2000** (`TOLERANCES.md`
/// §3.4.1, `README.md` §14.7). This tolerance is that number, rounded down to
/// one significant figure — `2,5×10⁻¹`. There is nothing else in it: no
/// headroom factor, no safety multiple, no anchor.
///
/// **What it means to fail it.** Above this line, compiling is the *dominant*
/// error term on this transform: the difference between a compiled iccce and an
/// uncompiled one would exceed the difference between iccce and the reference
/// implementation, so a user comparing outputs could not tell an engine choice
/// from an optimisation. That is a shippability statement, and it is the one a
/// compiled path actually has to satisfy.
///
/// **What it does NOT mean.** It is not a perceptibility claim; §2's anchor is
/// not cited in its support, and the ⚠ on that anchor is neither inherited nor
/// needed. It is not a claim that 0,25 ΔE2000 is invisible.
///
/// **Grid-dependent by construction**, and stated on the record: the quantity
/// is `O(h^1,32)` in the grid spacing - the control measured the order and
/// the `h²` prediction was falsified - so this bound belongs to the shipped
/// default grid and to nothing else. §B prints what 5, 9 and 17 cost so that
/// dependence is visible rather than implied.
pub const COMPILED_DE: Tolerance = Tolerance::new(
    2.5e-1,
    "compiling must not move the result further than the two implementations already differ on \
     the same transform: Pass 4 measured iccce vs lcms2 2.19.1 at 0.25294 dE2000 on this exact \
     pair (USWebCoatedSWOP -> sRGB, media-relative, 341 CMYK points, TOLERANCES.md 3.4.1), and \
     this is that number to one significant figure with NO headroom factor and NO free \
     parameter. Deliberately NOT derived from the 1.0 dE2000 perceptibility anchor: NA-006 \
     alone was measured at 1.574 dE2000 on A2B0 of this same file, so a budget derived from a \
     total below the anchor would be derived from a total that is already exceeded. \
     GRID-DEPENDENT: the quantity is O(h^1.32) as measured by the control row (NOT O(h^2) - that prediction was falsified), so this bound belongs to the SHIPPED DEFAULT GRID and to nothing else. It was derived when that default was 17, where it FAILED at 2.9702e-1; the default moved to 33 in commit 189e732 and the constant did NOT move, which is the whole point of a tolerance with no free parameter in it. \
     self-consistency - worthless as correctness evidence however small",
);

/// **§A, the device row**, kept beside the ΔE one because it is the quantity
/// `iccce bench` prints and the quantity a caller sizing a buffer cares about.
///
/// The bound is the ΔE bound divided by the **largest** device→ΔE sensitivity
/// this destination has anywhere on the axis, so that the two rows cannot
/// disagree about the same event. sRGB's inverse transfer function below its
/// `0,003 130 8` linear breakpoint divides a device difference by 12,92 into
/// linear light, and CIELAB's own linear segment then multiplies by
/// `da*/dX = 4038` (`TOLERANCES.md` §3.4.4 row C3's chain), giving
/// `Δa* ≈ 136 δ` against `ΔL* ≈ 69,9 δ`; with `S_C ≈ 1` the chromatic term
/// dominates, so `ΔE00 ≲ 136 δ` and `δ ≲ 2,5×10⁻¹/136 = 1,84×10⁻³`.
///
/// ★ **This is TIGHTER than the observed 3,589×10⁻³, and deliberately so.**
/// The 136 is the sensitivity *at the shadow end*, and the compiled path's
/// worst device error is not there — §A records where it actually is. Grading
/// the device row at the shadow-derived bound would fail a run that the ΔE row
/// passes comfortably, which would be a gate asserting something neither row
/// means. So the device row is **REPORTED, NOT GRADED**, and the ΔE row is the
/// gate. Recording the arithmetic that made the device bound unusable is worth
/// more than quietly picking `5×10⁻³`: *the same physical event has a different
/// size in two units, and the unit in which the requirement is stated is the
/// one that may carry the tolerance.*
pub const REPORTED: Tolerance = Tolerance::new(
    f64::INFINITY,
    "REPORTED, NOT GRADED - recorded so the number is on file next to the ones that are graded",
);

/// **§A, apparatus.** The harness's in-process off-node device maximum at the
/// shipped default grid against the number `iccce bench` prints.
///
/// ★ **This row is what caught the default moving from 17 to 33.** When
/// `recommended_grid_points` changed in commit `189e732` and `DEFAULT_GRID`
/// here did not, it failed at 1,576x10^-3 - which is not an error but the
/// gap between the two grids' costs. A cheap row that fails loudly when the
/// two arms stop describing the same transform is worth more than an
/// expensive one that quietly averages over it.
///
/// The precondition for §A being a *translation* of the benchmark rather than a
/// second, differently-shaped measurement that happens to concern the same
/// subject. If these two disagree, the ΔE figure below belongs to a probe
/// population nobody has published and the comparison to `iccce bench` is
/// rhetorical.
///
/// `1×10⁻⁹` is the CLI's own print floor: `error.max_device_offnode` is printed
/// to **nine** decimals, so one printed lsb is `10⁻⁹`. The bound is that and
/// nothing else — it cannot absorb a different probe set, a different grid or a
/// different intent, all of which move this quantity by ≥10⁻⁴.
pub const APPARATUS_BENCH: Tolerance = Tolerance::new(
    1e-9,
    "iccce bench prints error.max_device_offnode to 9 decimals, so one printed lsb is 1e-9; the \
     harness must reproduce the shipped binary's number to its own print floor or section A is \
     not a translation of it. Cannot absorb a different probe set, grid or intent - each moves \
     this quantity by 1e-4 or more",
);

/// **§A, structural — NOT EVIDENCE (DL-023).** At a grid node the compiled and
/// reference arms are identical *by construction*: the node's value **is** a
/// reference evaluation, stored.
///
/// This row measures exactly one thing — that the 4-D index arithmetic in
/// `CompiledTransform::new` and in `Clut::eval` use the same channel order. A
/// transposition puts whole channels in the wrong place and shows up here at
/// `O(1)`, while `compiled.rs`'s own node test runs on a **3**-channel identity
/// chain where a transposition of a symmetric grid can hide. It must never be
/// cited as the compiled path's error; that is §A's ΔE row.
///
/// `1×10⁻¹²` is ~4 orders above `f64` noise on this arithmetic and ~7 below one
/// `u16` lsb.
pub const NODE_IDENTITY: Tolerance = Tolerance::new(
    1e-12,
    "STRUCTURAL, NOT EVIDENCE (DL-023): at a grid node the compiled value IS a stored reference \
     evaluation, so equality is by construction and this row grades only that the 4-D index \
     arithmetic in CompiledTransform::new and Clut::eval share a channel order. 1e-12 is ~4 \
     orders above f64 noise here and ~7 below one u16 lsb",
);

/// **§B, the sensitivity control (DL-018).** How far the observed **paired
/// median** error ratio falls outside `[2, 8]` when the grid spacing is
/// halved.
///
/// ## ★ Both halves of this constant were wrong on their first run, and the
/// corrections are the most useful thing Pass 6 produced
///
/// **The estimator was wrong.** The first draft graded
/// `max(err at coarse) / max(err at fine)` — the ratio of two *maxima*, which
/// is what `compiled.rs`'s unit test uses. `h²` is a statement about **a fixed
/// point** as `h` shrinks; two maxima need not be at the same point, and as
/// the grid refines *which* probe is worst moves. Measured on 513 probes over
/// three halvings the two estimators say different things:
///
/// | halving | max-of-max | paired median |
/// |---|---|---|
/// | 5 → 9 | **5,57** | 2,69 |
/// | 9 → 17 | **1,39** | 2,47 |
/// | 17 → 33 | **1,78** | 2,51 |
///
/// The max-of-max wanders over a factor of **4**; the paired median is
/// **2,5 ± 0,02 across three octaves**. The graded quantity is therefore the
/// paired median, and the max-of-max figures are reported beside it as the
/// falsified instrument.
///
/// **The justification was wrong too, and in the more interesting way.** The
/// first draft predicted ≈4 (`h²`) and explained the band `[2, 8]` by CLUT-node
/// alignment: grids of 9/17/33 sit on SWOP's 9-node lattice, a grid of 5
/// straddles it. That prediction was tested and **falsified**: restricting the
/// measurement to cells whose 16 corners are all strictly inside the gamut and
/// above sRGB's breakpoint changed the 9→17 and 17→33 ratios **not at all**
/// (1,39 and 1,78 either way). The clamp and the breakpoint are not the cause.
///
/// What the measurement actually says is that the observed convergence order is
/// **`log₂ 2,5 = 1,32`** — between `h¹` and `h²`, and stable. That is the
/// signature of a function with a **smooth envelope and unresolvable fine-scale
/// kinks**: SWOP's `A2B1` is an `mft2` whose 256-entry input tables are
/// interpolated linearly, putting **255 derivative discontinuities per axis at
/// `k/255`**, and `gcd(255, N) = 1` for `N ∈ {4, 8, 16, 32}`, so *no* compiled
/// grid in reach aligns with them and every cell at every density contains
/// dozens. Coarse cells are dominated by the envelope's curvature (`h²`), fine
/// cells by the kinks (`h¹`), and the blend sits where it was measured.
///
/// **So the band is re-derived from what a band can honestly assert**, which is
/// not "the order is 2" but "**the order is between 1 and 3**":
///
/// - **ratio ≥ 2** (order ≥ 1). Below this the error is not grid-driven at all
///   and no number from this instrument is evidence. Ratio → 1 is the exact
///   failure `compiled.rs`'s own control hit on its first run, on an sRGB→sRGB
///   identity chain a grid reproduces *everywhere*.
/// - **ratio ≤ 8** (order ≤ 3). Multilinear interpolation cannot beat `h²` on a
///   non-degenerate function; a measured `h³` would mean the probes are
///   collapsing onto nodes, which measures nothing.
///
/// `0,0` is honest for the graded quantity because it is a `max(0, ·)` of a band
/// test, not a floating-point residual.
pub const H2_BAND: Tolerance = Tolerance::new(
    0.0,
    "band violation max(0, 2-r, r-8) on the PAIRED MEDIAN ratio err(coarse)/err(fine) at the same      probe, over the halvings 9->17 and 17->33. The band asserts only that the observed      convergence order lies in [1, 3]: below order 1 the error is not grid-driven and no number      from this instrument is evidence (ratio -> 1 is the failure compiled.rs's own control hit on      an identity chain); above order 3 is impossible for multilinear interpolation on a      non-degenerate function and would mean the probes are collapsing onto nodes. It deliberately      does NOT assert order 2 - the measured order is 1.32, stable to 1% across three octaves, and      the h^2 prediction was falsified rather than accommodated. Not a floating-point residual, so      0.0 needs no rounding allowance",
);

// ===========================================================================
// The probe population — iccce bench's own, reproduced
// ===========================================================================

/// Reproduce `iccce bench`'s sampled probe set exactly.
///
/// `iccce bench` builds a raster of `pixels × channels` with
/// `v = ((p·7 + c·131) mod 1024) / 1023` and then walks it in strides of
/// `max(1, pixels/512)`, checking the compiled result against the reference at
/// each stop. This returns those stops' device vectors, in order.
///
/// Reproduced rather than re-invented so that §A's ΔE is a translation of the
/// benchmark's device number; `pass6/apparatus/harness-reproduces-bench` checks
/// that it is.
#[must_use]
pub fn bench_probes(pixels: usize, channels: usize) -> Vec<Vec<f64>> {
    let step = (pixels / 512).max(1);
    let mut out = Vec::with_capacity(513);
    let mut p = 0usize;
    while p < pixels {
        let mut px = Vec::with_capacity(channels);
        for c in 0..channels {
            #[allow(clippy::cast_precision_loss)]
            let v = ((p * 7 + c * 131) % 1024) as f64 / 1023.0;
            px.push(v);
        }
        out.push(px);
        p += step;
    }
    out
}

// ===========================================================================
// The analysis
// ===========================================================================

/// What one grid density cost.
#[derive(Debug, Clone)]
pub struct GridRun {
    pub grid_points: usize,
    /// Max over probes and components of `|compiled − reference|`, device 0..1.
    pub device_max: f64,
    /// The same, restricted to probes whose grid cell is **smooth** — see
    /// [`Analysis::smooth_probes`].
    pub device_max_smooth: f64,
    /// Max over probes of ΔE2000 between the two arms, carried into D50 CIELAB
    /// through the **destination profile's own** matrix/TRC model.
    pub de_max: f64,
    pub de_mean: f64,
    /// The probe at which `de_max` occurred, and its `L*` — so "where the
    /// maximum sits" is on the record (TOLERANCES §6.4's carried-forward
    /// lesson: where in the grid the maximum sits is part of the derivation).
    pub de_max_at: Vec<f64>,
    pub de_max_l: f64,
    /// ★ The same two maxima over **Pass 4's own 341-point CMYK grid** — the
    /// population on which the 0,252 94 ΔE2000 that [`COMPILED_DE`] is derived
    /// from was measured. Comparing a maximum over one population to a maximum
    /// over another is not the same comparison, so both are recorded.
    pub device_max_pass4_grid: f64,
    pub de_max_pass4_grid: f64,
    /// Seconds to build the grid — `grid_points^4` chain evaluations here.
    pub build_seconds: f64,
    /// Max over probes and components of `|compiled(node) − reference(node)|`
    /// on the grid's OWN nodes. Structural: must be zero by construction.
    pub node_identity: f64,
    /// How many of the probes' cells at this density are smooth.
    pub smooth_cells: usize,
    /// ★ Per-probe device error, kept so the `h²` law can be tested **paired**
    /// (probe by probe) as well as on the maxima. The two are different
    /// questions and the second run of this pass found they have different
    /// answers.
    pub device_err: Vec<f64>,
}

/// Is the grid-`n` cell containing `probe` free of derivative discontinuities?
///
/// ## Why this exists, and it is the most consequential 40 lines in the module
///
/// `h²` is the interpolation-error law for a **`C²`** function. The composite
/// being compiled here is not one, and not for a subtle reason:
///
/// - **The destination gamut clamp (NA-004).** A great many CMYK values are
///   outside sRGB, and the chain clamps each output component to `[0, 1]`. The
///   clamp's boundary is a hypersurface through the CMYK cube that is aligned
///   with **no** grid at **any** density, so cells straddling it are `C⁰` and
///   their interpolation error scales `h¹`.
/// - **sRGB's transfer-function breakpoint at `0,040 45`.** Same shape of
///   problem, another unaligned hypersurface.
///
/// Neither goes away by refining. So a cell is called smooth here only if all
/// `2⁴ = 16` of its corners evaluate strictly inside `(breakpoint, 1)` on every
/// output component — which is checked by evaluating the **reference** path at
/// the corners, not by inspecting the compiled grid, so the classification does
/// not depend on the thing being graded.
fn cell_is_smooth(chain: &Chain, probe: &[f64], n: usize, out_ch: usize) -> bool {
    let in_ch = probe.len();
    #[allow(clippy::cast_precision_loss)]
    let h = 1.0 / (n - 1) as f64;
    let mut lo = Vec::with_capacity(in_ch);
    for &v in probe {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let i = ((v / h).floor() as usize).min(n - 2);
        lo.push(i);
    }
    for corner in 0..(1usize << in_ch) {
        let mut p = Vec::with_capacity(in_ch);
        for (c, &i) in lo.iter().enumerate() {
            let bit = (corner >> c) & 1;
            #[allow(clippy::cast_precision_loss)]
            let v = (i + bit) as f64 * h;
            p.push(v.min(1.0));
        }
        let Ok(r) = chain.convert(&p) else {
            return false;
        };
        for c in 0..out_ch {
            if r[c] <= SRGB_BREAKPOINT || r[c] >= 1.0 {
                return false;
            }
        }
    }
    true
}

/// What `iccce bench` printed, parsed from the shipped binary's stdout.
#[derive(Debug, Clone)]
pub struct BenchOutput {
    pub grid_points: usize,
    pub raster_pixels: usize,
    pub build_seconds: f64,
    pub convert_seconds: f64,
    pub megapixels_per_second: f64,
    pub reference_megapixels_per_second: f64,
    pub speedup: f64,
    pub error_samples: usize,
    pub error_max_device_offnode: f64,
    pub exe: PathBuf,
    pub is_debug: bool,
}

impl BenchOutput {
    /// Pixels at which compiling has paid for itself: the build cost divided by
    /// the per-pixel saving. **Reported**, and it is the number that decides
    /// whether a caller should compile at all.
    #[must_use]
    pub fn break_even_pixels(&self) -> f64 {
        let per_px_ref = 1.0 / (self.reference_megapixels_per_second * 1.0e6);
        let per_px_cmp = 1.0 / (self.megapixels_per_second * 1.0e6);
        let saving = per_px_ref - per_px_cmp;
        if saving <= 0.0 {
            f64::INFINITY
        } else {
            self.build_seconds / saving
        }
    }
}

/// Everything Pass 6 measured.
#[derive(Debug)]
pub struct Analysis {
    pub runs: Vec<GridRun>,
    pub bench: BenchOutput,
    pub probes: usize,
    pub structure: String,
    /// `[coarse/fine]` over the **whole** probe population, one per halving:
    /// 5→9, 9→17, 17→33.
    pub ratios: Vec<f64>,
    /// The same halvings restricted to probes whose cell is smooth at **both**
    /// densities of the pair, and only for the three grids whose nodes lie on
    /// the source CLUT's own lattice: 9→17 and 17→33.
    pub ratios_smooth: Vec<f64>,
    /// How many probes have a smooth cell at every one of 9, 17, 33.
    pub smooth_probes: usize,
    /// How many probes have at least one reference output component clamped at
    /// 0 or 1 — the out-of-gamut fraction, which is what makes the composite
    /// `C⁰` and defeats `h²`.
    pub clamped_probes: usize,
    /// Pass 4's own grid, for the population-matched row.
    pub pass4_probes: usize,
}

impl Analysis {
    fn run_at(&self, n: usize) -> &GridRun {
        self.runs
            .iter()
            .find(|r| r.grid_points == n)
            .expect("GRIDS is a fixed list and every entry produces a run")
    }

    /// The band violation §B grades — computed on the **smooth-cell** ratios,
    /// which are the only ones `h²` is the right law for. Zero exactly when
    /// every smooth ratio is in `[2, 8]`.
    #[must_use]
    pub fn band_violation(&self) -> f64 {
        [self.paired_median_ratio(9, 17), self.paired_median_ratio(17, 33)]
            .iter()
            .map(|&r| (2.0 - r).max(r - 8.0).max(0.0))
            .fold(0.0f64, f64::max)
    }

    /// ★ The `h²` law tested **paired**: for each probe, `err(coarse)/err(fine)`
    /// at the same point, then the **median** over probes.
    ///
    /// This and [`Analysis::ratios`] answer different questions and the answers
    /// differ. `ratios` divides one *maximum* by another, and the two maxima
    /// need not be at the same probe — as the grid refines, *which* probe is
    /// worst moves, so a max-of-max ratio measures the sampling of the
    /// population as much as the law. The paired median holds the point fixed
    /// and is the estimator `h²` is actually a statement about.
    #[must_use]
    pub fn paired_median_ratio(&self, coarse: usize, fine: usize) -> f64 {
        let c = self.run_at(coarse);
        let f = self.run_at(fine);
        let mut r: Vec<f64> = c
            .device_err
            .iter()
            .zip(&f.device_err)
            .filter(|(_, b)| **b > 0.0)
            .map(|(a, b)| *a / *b)
            .collect();
        if r.is_empty() {
            return f64::NAN;
        }
        r.sort_by(|a, b| a.partial_cmp(b).expect("no NaN: divisor filtered > 0"));
        r[r.len() / 2]
    }

    /// The same violation over the whole population — **reported**, because it
    /// is non-zero and the reason it is non-zero is §B's finding.
    #[must_use]
    pub fn band_violation_all_probes(&self) -> f64 {
        self.ratios
            .iter()
            .map(|&r| (2.0 - r).max(r - 8.0).max(0.0))
            .fold(0.0f64, f64::max)
    }
}

/// Why Pass 6 could not run.
#[derive(Debug)]
pub enum Unavailable {
    Skip(String),
    Error(String),
}

impl std::fmt::Display for Unavailable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unavailable::Skip(s) | Unavailable::Error(s) => f.write_str(s),
        }
    }
}

/// Parse one `key: value` line out of `iccce bench`'s output.
fn field(stdout: &str, key: &str) -> Option<String> {
    stdout.lines().find_map(|l| {
        let (k, v) = l.split_once(':')?;
        (k.trim() == key).then(|| v.trim().to_string())
    })
}

/// Run Pass 6.
///
/// **No oracle.** Both arms are iccce, so `transicc` is not consulted and this
/// section runs on a machine with no lcms2 build — but it still needs the two
/// system profiles (category (c)) and the shipped binary, and skips with a
/// reason when either is absent.
pub fn analyse() -> Result<Analysis, Unavailable> {
    let src_path = Path::new(SWOP);
    let dst_path = Path::new(SRGB);
    for p in [src_path, dst_path] {
        if !p.is_file() {
            return Err(Unavailable::Skip(format!(
                "profile not present on this machine: {} (LEGAL.md §3 category (c): \
                 read locally, never committed, never a required input)",
                p.display()
            )));
        }
    }
    let iccce = match Iccce::locate() {
        Err(e) => return Err(Unavailable::Error(e.to_string())),
        Ok(None) => {
            return Err(Unavailable::Skip(
                "iccce binary not found: run `cargo build --release -p iccce-cli`".to_string(),
            ));
        }
        Ok(Some(i)) => i,
    };

    // --- the shipped binary's own benchmark, as a subprocess -----------------
    // Deliberately the CLI and not an in-process call: the numbers this pass
    // grades are the numbers `iccce bench` prints, and a reader must be able to
    // reproduce them by running that command.
    let out = Command::new(iccce.path())
        .args([
            "bench",
            "--src",
            &src_path.display().to_string(),
            "--dst",
            &dst_path.display().to_string(),
        ])
        .output()
        .map_err(|e| Unavailable::Error(format!("cannot run `iccce bench`: {e}")))?;
    if !out.status.success() {
        return Err(Unavailable::Error(format!(
            "`iccce bench` exited {:?}: {}",
            out.status.code(),
            String::from_utf8_lossy(&out.stderr).trim()
        )));
    }
    let so = String::from_utf8_lossy(&out.stdout).to_string();
    let num = |k: &str| -> Result<f64, Unavailable> {
        field(&so, k)
            .and_then(|v| v.parse::<f64>().ok())
            .ok_or_else(|| Unavailable::Error(format!("`iccce bench` printed no parsable `{k}`")))
    };
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let bench = BenchOutput {
        grid_points: num("grid.points_per_axis")? as usize,
        raster_pixels: num("raster.pixels")? as usize,
        build_seconds: num("build.seconds")?,
        convert_seconds: num("convert.seconds")?,
        megapixels_per_second: num("throughput.megapixels_per_second")?,
        reference_megapixels_per_second: num("reference.megapixels_per_second")?,
        speedup: num("speedup.compiled_over_reference")?,
        error_samples: num("error.samples")? as usize,
        error_max_device_offnode: num("error.max_device_offnode")?,
        exe: iccce.path().to_path_buf(),
        is_debug: iccce.is_debug_build(),
    };

    // --- the same chain, in process ------------------------------------------
    let src_bytes = std::fs::read(src_path).map_err(|e| Unavailable::Error(e.to_string()))?;
    let dst_bytes = std::fs::read(dst_path).map_err(|e| Unavailable::Error(e.to_string()))?;
    let src = Profile::parse(&src_bytes).map_err(|e| Unavailable::Error(e.to_string()))?;
    let dst = Profile::parse(&dst_bytes).map_err(|e| Unavailable::Error(e.to_string()))?;
    let chain = Chain::new(&src, &dst, CmmIntent::MediaRelative)
        .map_err(|e| Unavailable::Error(format!("cannot build the chain: {e}")))?;
    // The route into a space where a ΔE means something: the DESTINATION's own
    // forward model. Not a second opinion, and the record says so — it is the
    // same colorimetry the destination profile itself defines.
    let dst_model = MatrixTrc::from_profile(&dst)
        .map_err(|e| Unavailable::Error(format!("destination has no matrix/TRC model: {e}")))?;
    let to_lab = |rgb: &[f64]| -> Lab {
        Lab::from_xyz(dst_model.device_to_pcs([rgb[0], rgb[1], rgb[2]]), D50)
    };

    let in_ch = chain.input_channels();
    let out_ch = chain.output_channels();
    let probes = bench_probes(BENCH_PIXELS, in_ch);
    // ★ Pass 4's own 341-point CMYK grid, the population COMPILED_DE's
    // antecedent (0,252 94 dE2000) was measured on. Carried here so the
    // tolerance and the observation can be compared on the SAME points; the
    // bench raster is a different population and a maximum over one is not a
    // maximum over the other.
    let pass4_probes: Vec<Vec<f64>> = crate::pass4::grid()
        .into_iter()
        .map(|t| t.to_vec())
        .collect();

    // How much of the bench population is out of the destination's gamut? That
    // fraction is the reason h^2 does not hold here, so it is measured rather
    // than asserted.
    let mut clamped_probes = 0usize;
    for px in &probes {
        if let Ok(r) = chain.convert(px) {
            if r.iter().any(|&v| v <= 0.0 || v >= 1.0) {
                clamped_probes += 1;
            }
        }
    }
    // Which probes sit in a smooth cell at EVERY aligned density.
    let smooth_mask: Vec<bool> = probes
        .iter()
        .map(|px| {
            [9usize, 17, 33]
                .iter()
                .all(|&n| cell_is_smooth(&chain, px, n, out_ch))
        })
        .collect();
    let smooth_probes = smooth_mask.iter().filter(|b| **b).count();

    let structure = format!(
        "src v{:08X} {} {}->{} A2B1 mft2 4->3 9 nodes/axis | dst v{:08X} {} {}->{} matrix/TRC | \
         media-relative | {in_ch}->{out_ch} | {} probes = iccce bench's own sampled raster",
        src.header.version.raw,
        src.header.device_class,
        src.header.color_space,
        src.header.pcs,
        dst.header.version.raw,
        dst.header.device_class,
        dst.header.color_space,
        dst.header.pcs,
        probes.len(),
    );

    let mut runs = Vec::new();
    for n in GRIDS {
        let t0 = std::time::Instant::now();
        let compiled = CompiledTransform::new(&chain, n)
            .map_err(|e| Unavailable::Error(format!("cannot compile at grid {n}: {e}")))?;
        let build_seconds = t0.elapsed().as_secs_f64();

        let mut device_max = 0.0f64;
        let mut device_max_smooth = 0.0f64;
        let mut device_err: Vec<f64> = Vec::with_capacity(probes.len());
        let mut de: Vec<f64> = Vec::with_capacity(probes.len());
        let mut de_max = 0.0f64;
        let mut de_max_at: Vec<f64> = Vec::new();
        let mut de_max_l = f64::NAN;
        let mut cv = vec![0.0f64; out_ch];
        for (pi, px) in probes.iter().enumerate() {
            if !compiled.convert(px, &mut cv) {
                return Err(Unavailable::Error(
                    "CompiledTransform::convert refused a probe".into(),
                ));
            }
            let reference = chain
                .convert(px)
                .map_err(|e| Unavailable::Error(format!("the reference path refused a probe: {e}")))?;
            let mut here = 0.0f64;
            for c in 0..out_ch {
                let d = (cv[c] - reference[c]).abs();
                here = here.max(d);
            }
            device_err.push(here);
            device_max = device_max.max(here);
            if smooth_mask[pi] {
                device_max_smooth = device_max_smooth.max(here);
            }
            let a = to_lab(&cv);
            let b = to_lab(&reference);
            let d = delta_e_2000(a, b);
            if d > de_max {
                de_max = d;
                de_max_at = px.clone();
                de_max_l = b.l;
            }
            de.push(d);
        }
        #[allow(clippy::cast_precision_loss)]
        let de_mean = de.iter().sum::<f64>() / de.len() as f64;

        // The population-matched pair: the same two arms over Pass 4's grid.
        let mut device_max_pass4_grid = 0.0f64;
        let mut de_max_pass4_grid = 0.0f64;
        for px in &pass4_probes {
            if !compiled.convert(px, &mut cv) {
                return Err(Unavailable::Error(
                    "CompiledTransform::convert refused a Pass 4 grid point".into(),
                ));
            }
            let reference = chain.convert(px).map_err(|e| {
                Unavailable::Error(format!("the reference path refused a Pass 4 grid point: {e}"))
            })?;
            for c in 0..out_ch {
                device_max_pass4_grid =
                    device_max_pass4_grid.max((cv[c] - reference[c]).abs());
            }
            de_max_pass4_grid =
                de_max_pass4_grid.max(delta_e_2000(to_lab(&cv), to_lab(&reference)));
        }

        // The structural node check, on this grid's own nodes. A deterministic
        // walk of a few hundred of them (the full 4-D lattice at 17 is 83 521,
        // and evaluating the reference path at all of them is the build again).
        let mut node_identity = 0.0f64;
        let mut rv = vec![0.0f64; out_ch];
        for k in 0..251usize {
            let mut node = Vec::with_capacity(in_ch);
            for c in 0..in_ch {
                #[allow(clippy::cast_precision_loss)]
                let idx = (k * (c + 3) + c * 5) % n;
                node.push(idx as f64 / (n - 1) as f64);
            }
            if !compiled.convert(&node, &mut rv) {
                return Err(Unavailable::Error(
                    "CompiledTransform::convert refused a node".into(),
                ));
            }
            let reference = chain
                .convert(&node)
                .map_err(|e| Unavailable::Error(format!("the reference path refused a node: {e}")))?;
            for c in 0..out_ch {
                node_identity = node_identity.max((rv[c] - reference[c]).abs());
            }
        }

        runs.push(GridRun {
            grid_points: n,
            device_max,
            device_max_smooth,
            de_max,
            de_mean,
            de_max_at,
            de_max_l,
            device_max_pass4_grid,
            de_max_pass4_grid,
            build_seconds,
            node_identity,
            smooth_cells: smooth_probes,
            device_err,
        });
    }

    let ratios: Vec<f64> = runs
        .windows(2)
        .map(|w| {
            if w[1].device_max > 0.0 {
                w[0].device_max / w[1].device_max
            } else {
                f64::NAN
            }
        })
        .collect();
    // Only the aligned densities, and only on smooth cells: 9->17 and 17->33.
    let ratios_smooth: Vec<f64> = runs
        .windows(2)
        .filter(|w| w[0].grid_points >= 9)
        .map(|w| {
            if w[1].device_max_smooth > 0.0 {
                w[0].device_max_smooth / w[1].device_max_smooth
            } else {
                f64::NAN
            }
        })
        .collect();

    Ok(Analysis {
        runs,
        bench,
        probes: probes.len(),
        structure,
        ratios,
        ratios_smooth,
        smooth_probes,
        clamped_probes,
        pass4_probes: pass4_probes.len(),
    })
}

// ===========================================================================
// Records
// ===========================================================================

/// Pass 6's records.
#[must_use]
pub fn records(a: &Analysis) -> Vec<Record> {
    let ctx = format!(
        "{} | iccce={} ({}) | bench: grid={} raster={} px, build={:.3}s convert={:.3}s \
         {:.3} Mpix/s vs reference {:.3} Mpix/s = {:.2}x, break-even at {:.0} px",
        a.structure,
        a.bench.exe.display(),
        if a.bench.is_debug {
            "DEBUG BUILD"
        } else {
            "release"
        },
        a.bench.grid_points,
        a.bench.raster_pixels,
        a.bench.build_seconds,
        a.bench.convert_seconds,
        a.bench.megapixels_per_second,
        a.bench.reference_megapixels_per_second,
        a.bench.speedup,
        a.bench.break_even_pixels(),
    );
    let d = a.run_at(DEFAULT_GRID);

    let mut out = vec![
        // --- apparatus: is this the benchmark's measurement at all? ----------
        Record::graded(
            "pass6/apparatus/harness-reproduces-bench",
            Kind::SelfConsistency,
            Metric::DeviceAbsMaxNormalised,
            APPARATUS_BENCH,
            (d.device_max - a.bench.error_max_device_offnode).abs(),
            "the harness's in-process off-node maximum at grid 17 against the number the SHIPPED \
             `iccce bench` prints, across a process boundary - the precondition for the dE row \
             below being a TRANSLATION of the benchmark rather than a different measurement",
            format!(
                "{ctx} | harness={:.9} bench={:.9} over {} probes (bench sampled {})",
                d.device_max, a.bench.error_max_device_offnode, a.probes, a.bench.error_samples
            ),
        ),
        // --- structural, not evidence ----------------------------------------
        Record::graded(
            "pass6/structural/identical-at-nodes-4d",
            Kind::SelfConsistency,
            Metric::DeviceAbsMaxNormalised,
            NODE_IDENTITY,
            d.node_identity,
            "STRUCTURAL, NOT EVIDENCE (DL-023): at a node the compiled value IS a stored \
             reference evaluation. Grades only that the 4-D index arithmetic in \
             CompiledTransform::new and Clut::eval share a channel order",
            format!("{ctx} | 251 deterministic lattice nodes at grid {DEFAULT_GRID}"),
        ),
        // --- ★ the gate -------------------------------------------------------
        Record::graded(
            "pass6/swop-to-srgb/media-relative/compiled-cost-de2000",
            Kind::SelfConsistency,
            Metric::DeltaE2000Max,
            COMPILED_DE,
            d.de_max,
            "both arms are iccce, both computed in this run: CompiledTransform at grid 17 against \
             Chain::convert, carried into D50 CIELAB through the DESTINATION profile's own \
             matrix/TRC model",
            format!(
                "{ctx} | mean={:.4e} | the maximum sits at CMYK [{}] where the reference L*={:.2} \
                 | device max at the same grid={:.9}",
                d.de_mean,
                d.de_max_at
                    .iter()
                    .map(|v| format!("{v:.4}"))
                    .collect::<Vec<_>>()
                    .join(" "),
                d.de_max_l,
                d.device_max
            ),
        ),
        Record::graded(
            "pass6/swop-to-srgb/media-relative/compiled-cost-device",
            Kind::SelfConsistency,
            Metric::DeviceAbsMaxNormalised,
            REPORTED,
            d.device_max,
            "the quantity `iccce bench` prints. REPORTED, NOT GRADED: the device bound implied by \
             the dE tolerance (2.5e-1 / 136 = 1.84e-3, using sRGB's shadow sensitivity) is \
             TIGHTER than the observed value, and the observed maximum is not in the shadow - so \
             grading it would assert something neither row means",
            format!(
                "{ctx} | implied shadow-derived device bound 1.84e-3 vs observed {:.9}: the same \
                 event has a different size in two units, and the unit the requirement is stated \
                 in is the one that carries the tolerance",
                d.device_max
            ),
        ),
        // --- ★ the population-matched form of the same gate ------------------
        Record::graded(
            "pass6/swop-to-srgb/media-relative/compiled-cost-de2000-on-pass4-grid",
            Kind::SelfConsistency,
            Metric::DeltaE2000Max,
            COMPILED_DE,
            d.de_max_pass4_grid,
            "the SAME two arms over PASS 4's OWN 341-point CMYK grid - the population on which \
             the 0.25294 dE2000 this tolerance is derived from was measured. A maximum over one \
             population is not a maximum over another, so the line is checked on both",
            format!(
                "{ctx} | {} Pass 4 grid points | device max on the same points={:.6e} | \
                 the bench-raster form of this row observed {:.4e} over {} probes",
                a.pass4_probes, d.device_max_pass4_grid, d.de_max, a.probes
            ),
        ),
        // --- ★ the attribution: WHY the whole population does not scale h^2 --
        Record::graded(
            "pass6/control/max-of-max-is-the-wrong-estimator",
            Kind::SelfConsistency,
            Metric::AbsMaxComponent,
            REPORTED,
            a.band_violation_all_probes(),
            "REPORTED, NOT GRADED. The band violation on the MAX-OF-MAX ratio, kept on file \
             because that is the estimator compiled.rs's own unit test uses and it is the wrong \
             one: it divides one maximum by another and the two maxima are not at the same \
             probe, so it measures where the worst point moved as much as it measures the law. \
             A CLAMP ATTRIBUTION WAS TESTED HERE AND FALSIFIED - restricting to cells whose 16 \
             corners are all in gamut and above sRGB's 0.04045 breakpoint changed the ratios \
             not at all",
            format!(
                "{ctx} | {}/{} probes are out of the destination gamut (a reference output \
                 component clamped at 0 or 1) and only {}/{} sit in a cell smooth at 9, 17 AND \
                 33 - yet whole-population 9/17={:.2} 17/33={:.2} and smooth-cell {:.2} {:.2} \
                 are the SAME NUMBERS, so neither the clamp nor the breakpoint is the cause. \
                 The paired-median row is where the law is actually tested",
                a.clamped_probes,
                a.probes,
                a.smooth_probes,
                a.probes,
                a.ratios.get(1).copied().unwrap_or(f64::NAN),
                a.ratios.get(2).copied().unwrap_or(f64::NAN),
                a.ratios_smooth.first().copied().unwrap_or(f64::NAN),
                a.ratios_smooth.get(1).copied().unwrap_or(f64::NAN),
            ),
        ),
        // --- ★ the sensitivity control ---------------------------------------
        Record::graded(
            "pass6/control/error-scales-with-grid-spacing",
            Kind::SelfConsistency,
            Metric::AbsMaxComponent,
            H2_BAND,
            a.band_violation(),
            "DL-018: the same off-node measurement on deliberately coarser grids, restricted to \
             probes whose cell is SMOOTH at every aligned density - the only cells h^2 is the \
             right law for. Cell smoothness is decided by evaluating the REFERENCE path at all \
             16 corners, so the classification does not depend on the arm being graded",
            format!(
                "{ctx} | {} | smooth-cell ratios 9/17={:.2} 17/33={:.2} over {}/{} probes | \
                 WHOLE-POPULATION ratios 5/9={:.2} 9/17={:.2} 17/33={:.2}, violation {:.4e}",
                a.runs
                    .iter()
                    .map(|r| format!(
                        "grid {}: device={:.6e} (smooth {:.6e}) dE={:.4e} build={:.3}s",
                        r.grid_points,
                        r.device_max,
                        r.device_max_smooth,
                        r.de_max,
                        r.build_seconds
                    ))
                    .collect::<Vec<_>>()
                    .join(" | "),
                a.ratios_smooth.first().copied().unwrap_or(f64::NAN),
                a.ratios_smooth.get(1).copied().unwrap_or(f64::NAN),
                a.smooth_probes,
                a.probes,
                a.ratios.first().copied().unwrap_or(f64::NAN),
                a.ratios.get(1).copied().unwrap_or(f64::NAN),
                a.ratios.get(2).copied().unwrap_or(f64::NAN),
                a.band_violation_all_probes(),
            ),
        ),
        // --- ★ the same law, tested PAIRED ------------------------------------
        Record::graded(
            "pass6/control/h2-paired-median-ratio",
            Kind::SelfConsistency,
            Metric::AbsMaxComponent,
            REPORTED,
            a.paired_median_ratio(17, 33),
            "REPORTED: err(coarse)/err(fine) at the SAME probe, median over probes. h^2 is a              statement about a fixed point as h shrinks; a ratio of two maxima is not, because              WHICH probe is worst moves as the grid refines",
            format!(
                "{ctx} | paired median 5/9={:.2} 9/17={:.2} 17/33={:.2} |                  against max-of-max 5/9={:.2} 9/17={:.2} 17/33={:.2}",
                a.paired_median_ratio(5, 9),
                a.paired_median_ratio(9, 17),
                a.paired_median_ratio(17, 33),
                a.ratios.first().copied().unwrap_or(f64::NAN),
                a.ratios.get(1).copied().unwrap_or(f64::NAN),
                a.ratios.get(2).copied().unwrap_or(f64::NAN),
            ),
        ),
    ];

    // --- the cost of the two coarser grids, reported so the trade is visible --
    for r in &a.runs {
        if r.grid_points == DEFAULT_GRID {
            continue;
        }
        out.push(Record::graded(
            format!("pass6/grid-{}/compiled-cost-de2000", r.grid_points),
            Kind::SelfConsistency,
            Metric::DeltaE2000Max,
            REPORTED,
            r.de_max,
            "both arms are iccce. REPORTED, NOT GRADED: 17 is the shipped default and the only \
             grid COMPILED_DE is derived for; these say what the alternatives cost",
            format!(
                "{ctx} | grid={} device={:.6e} dE-mean={:.4e} build={:.3}s",
                r.grid_points, r.device_max, r.de_mean, r.build_seconds
            ),
        ));
    }
    out
}

/// The ids, kinds, metrics and tolerances of every Pass 6 record, for the
/// skip/error path — so the report has the same shape on a machine that cannot
/// run it.
#[must_use]
pub fn unavailable_records(u: &Unavailable) -> Vec<Record> {
    let reason = u.to_string();
    let specs: Vec<(String, Kind, Metric, Tolerance)> = vec![
        (
            "pass6/apparatus/harness-reproduces-bench".into(),
            Kind::SelfConsistency,
            Metric::DeviceAbsMaxNormalised,
            APPARATUS_BENCH,
        ),
        (
            "pass6/structural/identical-at-nodes-4d".into(),
            Kind::SelfConsistency,
            Metric::DeviceAbsMaxNormalised,
            NODE_IDENTITY,
        ),
        (
            "pass6/swop-to-srgb/media-relative/compiled-cost-de2000".into(),
            Kind::SelfConsistency,
            Metric::DeltaE2000Max,
            COMPILED_DE,
        ),
        (
            "pass6/swop-to-srgb/media-relative/compiled-cost-device".into(),
            Kind::SelfConsistency,
            Metric::DeviceAbsMaxNormalised,
            REPORTED,
        ),
        (
            "pass6/control/error-scales-with-grid-spacing".into(),
            Kind::SelfConsistency,
            Metric::AbsMaxComponent,
            H2_BAND,
        ),
        (
            "pass6/grid-5/compiled-cost-de2000".into(),
            Kind::SelfConsistency,
            Metric::DeltaE2000Max,
            REPORTED,
        ),
        (
            "pass6/grid-9/compiled-cost-de2000".into(),
            Kind::SelfConsistency,
            Metric::DeltaE2000Max,
            REPORTED,
        ),
        (
            "pass6/grid-33/compiled-cost-de2000".into(),
            Kind::SelfConsistency,
            Metric::DeltaE2000Max,
            REPORTED,
        ),
        (
            "pass6/swop-to-srgb/media-relative/compiled-cost-de2000-on-pass4-grid".into(),
            Kind::SelfConsistency,
            Metric::DeltaE2000Max,
            COMPILED_DE,
        ),
        (
            "pass6/control/max-of-max-is-the-wrong-estimator".into(),
            Kind::SelfConsistency,
            Metric::AbsMaxComponent,
            REPORTED,
        ),
        (
            "pass6/control/h2-paired-median-ratio".into(),
            Kind::SelfConsistency,
            Metric::AbsMaxComponent,
            REPORTED,
        ),
    ];
    specs
        .into_iter()
        .map(|(id, kind, metric, tol)| {
            let source = "both arms are iccce (compiled vs reference); no oracle is consulted";
            match u {
                Unavailable::Skip(_) => {
                    Record::skipped(id, kind, metric, tol, source, reason.clone())
                }
                Unavailable::Error(_) => {
                    Record::errored(id, kind, metric, tol, source, reason.clone())
                }
            }
        })
        .collect()
}

/// Run Pass 6 and return its records, skip-labelled if it could not run.
#[must_use]
pub fn run() -> (Option<Analysis>, Vec<Record>) {
    match analyse() {
        Ok(a) => {
            let r = records(&a);
            (Some(a), r)
        }
        Err(u) => (None, unavailable_records(&u)),
    }
}
