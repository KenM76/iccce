//! # Pass L — **which reading of sRGB does lcms2 implement?**, measured
//!
//! ## The question, and why no document can answer it
//!
//! On 2026-08-19 `icc-spec-librarian` filed register row **`A57`**: two
//! currently-in-force standards define "sRGB" with **different
//! transfer-function constants**, and neither is a typo.
//!
//! | reading | offset `α` | breakpoint (encoded) | printed by |
//! |---|---|---|---|
//! | **C⁰** — value continuity, offset pinned at `0.055` | `1.055` exactly | `0.04045` | ICC's own sRGB document, W3C CSS Color 4, Khronos KDF |
//! | **C¹** — value *and slope* continuity | `1.055 010 718 947 586 4` | `0.039 293 370 676 847 5` | **required** by `Rec. ITU-T H.273 (V4) (07/2024) / ISO/IEC 23091-2` clause 8.2 for `TransferCharacteristics = 13`, which AVIF, HEIF, AV1 and ISOBMFF `nclx` all point at |
//!
//! `iccce` ships both — [`iccce_cmm::builtin::SrgbTrc::ValueContinuous`] (the
//! default, unchanged) and [`iccce_cmm::builtin::SrgbTrc::SlopeContinuous`],
//! whose constants are **derived** from H.273's clause at run time by
//! bisection and never transcribed.
//!
//! **`A57` is a documentation disagreement and it stays OPEN.** Nothing in
//! this module bears on what sRGB *is*. What this module establishes is a
//! different and separately useful thing:
//!
//! > **Which of the two readings the world's most widely deployed CMM actually
//! > implements** — an empirical fact about lcms2, obtained by measurement,
//! > which no amount of reading could have produced.
//!
//! ★ **Do not let a Pass L result be quoted as settling `A57`.** IEC's
//! normative text would say what sRGB is; this says what one implementation
//! does. Those are different claims, and the `source` string on every §A
//! record says so in the record itself, where a quoter will trip over it.
//!
//! ## ★★ The difficulty, which is the whole design problem
//!
//! The two curves are **`4.777 283×10⁻⁶` apart at most in linear light** and
//! **exactly zero apart over all 256 8-bit codes**. An ordinary difftest probe
//! — 8-bit in, 8-bit out, at a round number — has *no* discriminating power
//! whatever. Three things had to be got right, and each is a trap.
//!
//! ### Trap 1 — float I/O, not 8-bit
//!
//! `transicc`'s default is already floating point in and out, and
//! [`crate::Precalc::Exact`] (`-c0`, `cmsFLAGS_NOOPTIMIZE`) keeps the pipeline
//! unflattened. Had the harness passed `-w`/`-x` the 16-bit quantum
//! (`1.526×10⁻⁵`) alone would have erased the signal.
//!
//! ### Trap 2 — the maximum separation is INTERIOR, and it is in a DIFFERENT
//! ### PLACE for each output space
//!
//! `iccce_cmm::builtin`'s
//! `tests::breakpoint_is_the_c0_solution_not_the_1996_value` already records
//! that for the *other* breakpoint question the obvious "probe at the
//! boundary" choice has **exactly zero** discriminating power. The same trap
//! applies here, twice over:
//!
//! | probe | separation |
//! |---|---|
//! | at the **C¹ breakpoint** (code `10.019 81`) | ★ **exactly `0`** — C¹'s `β` is *defined* by value continuity with the linear segment both readings share, so the two curves **meet there by construction** |
//! | at the **C⁰ breakpoint** (code `10.314 75`) | `6.93×10⁻⁴` `L*` — 6.9 printed quanta; usable, but 42 % below the best available |
//! | at the **`L*` interior maximum** (code `23.513 60`) | **`1.202 916×10⁻³`** `L*` — 12.0 printed quanta |
//! | at the **linear-light interior maximum** (code `142.905 68`) | `4.777 283×10⁻⁶` in `Y`, but only `4.38×10⁻⁴` `L*` — **64 % of the `L*` signal thrown away** |
//!
//! **The two instruments' maxima are 119 codes apart.** Choosing the
//! linear-light maximum — the number the feature's own doc comment prints, and
//! therefore the obvious probe — costs nearly two thirds of the signal in the
//! instrument that actually resolves it. Row
//! `passl/A/design/l-signal-at-the-linear-light-max` exists to keep that
//! statement true rather than merely written down.
//!
//! ### Trap 3 — the model must be lcms2's, not the standard's
//!
//! The residual `|lcms2 − prediction|` is evidence about the *curve* only if
//! every other stage of the model is exact. Two choices make it so:
//!
//! 1. **Neutral probes only in §A.** For `R = G = B`, `XYZ = TRC(v) · W`, and
//!    lcms2's built-in sRGB stores colorants already Bradford-adapted, so `W`
//!    is D50 with `Y = 1` exactly. Therefore `Y_PCS = TRC(v)` *identically* —
//!    no matrix, no adaptation, nothing left to model wrong.
//! 2. **lcms2's own `f()`.** `src/cmspcs.c` `f()` uses `Limit = (24/116)³`,
//!    which *is* `216/24389` exactly (`24/116 = 6/29`), so lcms2 and ICC.1
//!    agree here and the choice costs nothing today. It is written in lcms2's
//!    form anyway, because what is being modelled is lcms2's output, and a
//!    model of an implementation that quietly substitutes the standard's
//!    constant would measure the wrong thing on the one day they differ.
//!
//! The apparatus row `passl/A/apparatus/model-exact-in-shared-region` grades
//! that model **where the two readings are identical**, so it can validate the
//! model without being able to answer the question. Its separation is
//! **exactly zero and the report says so** — the correct disclosure for a row
//! that is deliberately unable to discriminate.
//!
//! ## §A — what was measured, and the verdict
//!
//! Instrument: `transicc -i*sRGB -o*Lab4 -t1 -c0 -n`, 275 neutral probes (all
//! 256 8-bit codes plus 19 designed points), float in, float out. **Measured
//! 2026-08-19**, lcms2 pin `21c582a`, MSVC build, Windows 11. ★ Every numeral
//! in the tables below is also emitted by the records themselves, computed;
//! if a table and a record ever disagree, the record is right and the table is
//! stale.
//!
//! | | `L*` |
//! |---|---|
//! | max `abs(lcms2 − C⁰)` over the sweep | **`5.301×10⁻⁵`** — below one printed quantum |
//! | max `abs(lcms2 − C¹)` over the sweep | **`1.230 354×10⁻³`** — 12.3 printed quanta |
//! | probes resolvable at ≥ 2 quanta that sit closer to **C¹** | **`0`** of 204 |
//!
//! **lcms2 implements the C⁰ reading**, and the second instrument (`-o*XYZ`,
//! probed at the linear-light maximum where *its* signal is best) agrees
//! independently. The **whitebox corroboration** is
//! `vendor/lcms2/src/cmsvirt.c` `Build_sRGBGamma()` L640–647, which writes
//! `Parameters[1] = 1./1.055`, `[2] = 0.055/1.055`, `[3] = 1./12.92`,
//! `[4] = 0.04045` into a type-4 parametric curve — graded as a
//! [`crate::Kind::OracleReproducibility`] row, so a pin move that changed it
//! would be caught rather than silently invalidating every §A number.
//!
//! ★★ **This is agreement, not disagreement, so `CLAUDE.md` rule 7 is not
//! engaged** — but the pre-registered posture was that a C¹ result would have
//! been *a finding, not a failure*: it would have meant the ICC-ecosystem
//! consensus and the dominant ICC implementation disagree. Nothing was
//! adjusted to make it come out this way; the whitebox read was done first and
//! the blackbox measurement was designed to be able to contradict it.
//!
//! ## §B — the honest precision statement
//!
//! `transicc` prints every float with **`%.4f`**
//! (`utils/transicc/transicc.c`, `PrintFloatResults`, L694–698). That is the
//! instrument's quantum and no flag widens it. What it can and cannot see:
//!
//! | output space | printed unit | quantum | max separation available | verdict |
//! |---|---|---|---|---|
//! | `*Lab` | `L*`, `0..100` | `1×10⁻⁴` | `1.202 916×10⁻³` | **resolvable, 12.0 quanta** |
//! | `*XYZ` | `Y × 100` | `1×10⁻⁴` (= `1×10⁻⁶` in `Y`) | `4.777×10⁻⁴` | **resolvable, 4.8 quanta** |
//! | `*XYZ`, probed at the *`L*`* maximum instead | — | — | `1.33×10⁻⁴` | 1.3 quanta — **not** resolvable |
//! | a **destination profile's `A2B`** used as a ruler | `L*` | see below | — | ★ **NOT resolvable** |
//!
//! ★★★ **The oracle cannot be the ruler for the destination half**, and that
//! was measured rather than assumed. lcms2 evaluates a `lut16Type` (`mft2`)
//! CLUT stage inside a *float* pipeline through `EvaluateCLUTfloatIn16`
//! (`src/cmslut.c` L445–456), whose first act is
//! `FromFloatTo16(In, In16, …)` — **it quantises its float input to 16 bits
//! before the lookup**. A control sweep of one ink across `2.95×10⁻²` % in
//! `5×10⁻⁴` % steps returned **7 distinct `L*` values out of 60 samples**: a
//! staircase whose tread is `≈4.9×10⁻³` % ink. The largest device separation
//! the reading choice produces at a real destination is `6.36×10⁻³` % —
//! **≈1.3 treads at its maximum, and a small fraction of one typically**. §C
//! therefore measures the destination half **entirely in process, in `f64`**,
//! and every §C record says so.
//!
//! ## §C — what the choice COSTS, which is the number a caller needs
//!
//! *Self-comparison: both sides are iccce.* The operator's standing rule is
//! that **a variant option is noise unless the ΔE between variants is
//! measured**. This is that number.
//!
//! ★ **§C is in process, and it has to be.** The shipped `iccce` binary has no
//! flag that selects [`iccce_cmm::builtin::SrgbTrc`] — the variant is
//! reachable **only through the library API**. Pass 5b set the precedent (it
//! drives `iccce_cmm::bpc` in process because the shipped chain cannot reach
//! the ISO estimator). That is a real scope statement about the *feature*, not
//! merely about the harness: **no CLI user can select the C¹ reading today.**
//!
//! | | ΔE2000 |
//! |---|---|
//! | **PCS, max** (`sRGB(variant) → PCS Lab D50`) | **`1.857 907×10⁻³`** at `rgb(0.039 299, 0.093 208, 0.039 299)` = codes `(10.021, 23.768, 10.021)` |
//! | PCS, mean over 55 938 probes | `3.630×10⁻⁴` |
//! | PCS, max **on the neutral ramp alone** | `7.396×10⁻⁴` at code `23.5135` |
//!
//! ★★ **The gray ramp — the natural probe for a transfer-function question —
//! understates the cost by 2.5×.** The true maximum is *off* the neutral axis,
//! at a point where `R` and `B` sit essentially **exactly on the C¹
//! breakpoint** (where their own contribution is zero) while `G` sits at the
//! `L*` junction. ΔE2000's chroma and hue terms put the worst case exactly
//! where a one-dimensional probe cannot reach it.
//!
//! ★★★ **And the qualifier that matters most.** `SrgbTrc`'s doc comment
//! records "**`0` of 256 8-bit codes change**". That is true, and it is about
//! **sRGB's own encoding**. It is **not** true end to end: pushed through a
//! real destination, `14` of `5 169` grid points move an 8-bit ink code in
//! `USWebCoatedSWOP.icc` and `11` of `5 169` in `AdobeRGB1998.icc` (`17` and
//! `6` respectively on a half-step-offset grid, so the effect is real and not
//! a grid artefact). The mechanism is *not* amplification — the device
//! separation is at most `6.36×10⁻⁵` of full scale, **one 62nd of an 8-bit
//! code**. It is that a difference that small still flips a code whenever the
//! value happens to straddle a rounding boundary, which ≈0.3 % of points do.
//!
//! ★★ **The destination does not wash the difference out; it slightly
//! amplifies it.** Round-tripping `sRGB(variant) → USWebCoatedSWOP → sRGB →
//! Lab` entirely in `f64` gives a maximum of `2.207 972×10⁻³` ΔE2000 against
//! `1.452 896×10⁻³` for the PCS alone over the same probe set — a gain of
//! **`1.52×`**.
//!
//! ## Coverage — stated so "verified" cannot be read without its scope
//!
//! - **§A** is `lcms2 2.19.1` at pin `21c582a`, **MSVC build, Windows 11**,
//!   **one profile** — lcms2's own *built-in* sRGB
//!   (`cmsCreate_sRGBProfileTHR`), **not** a system sRGB file, whose curve is
//!   a 1 024-entry table and would answer a different question — **one
//!   intent** (media-relative), **one direction**, **neutral probes only**,
//!   `-c0`.
//! - **§B**'s destination-ruler finding is `mft2`-specific: it is a property
//!   of lcms2's 16-bit CLUT stage and says nothing about `mAB `.
//! - **§C** is iccce against iccce. It is **not** evidence that either variant
//!   is *right*; it is the price of the choice.
//! - Nothing here is ground truth. §A is a fact about an implementation; §C is
//!   a self-comparison. The only ground-truth-class statement in the
//!   neighbourhood is H.273 clause 8.2 itself, and that is `ICC_Spec`'s to
//!   hold.

use std::path::{Path, PathBuf};

use iccce_cmm::MatrixTrc;
use iccce_cmm::builtin::{SrgbTrc, srgb_trc_params, srgb_with};
use iccce_cmm::lut_transform::PcsValue;
use iccce_cmm::matrix_trc::Intent as CmmIntent;
use iccce_cmm::transform::Chain;
use iccce_color::{D50, Lab, delta_e_2000};
use iccce_profile::Profile;

use crate::{
    Bpc, Intent, Kind, Metric, Oracle, Precalc, Record, Request, SepUnits, Separation, Space,
    Tolerance,
};

// ===========================================================================
// The instrument's quantum, and the tolerances derived from it
// ===========================================================================

/// `transicc` prints every floating-point result with **`%.4f`**
/// (`utils/transicc/transicc.c`, `PrintFloatResults`, L694–698). There is no
/// flag that widens it. Every §A tolerance is derived from this one number, so
/// that no tolerance in this pass is a number somebody chose.
const PRINT_QUANTUM: f64 = 1.0e-4;

/// Agreement to the precision the oracle prints, and no tighter.
///
/// Round-to-nearest bounds the printing error at **half** a quantum
/// (`5×10⁻⁵`); the other half is head-room for lcms2's own pipeline, which
/// evaluates tone curves through `cmsEvalToneCurveFloat` in
/// `cmsFloat32Number`. The measured excess over the pure rounding bound is
/// `3.0×10⁻⁶`, so the second half of the quantum is ~16× more head-room than
/// the observed need — deliberately, because a tolerance that has to be
/// widened later is a tolerance nobody could defend.
const TOL_PRINTED: Tolerance = Tolerance::new(
    PRINT_QUANTUM,
    "one printed quantum of transicc's %.4f (utils/transicc/transicc.c L694-698): \
     round-to-nearest bounds the print error at half a quantum, and the remaining half covers \
     lcms2's float32 curve evaluation, whose measured excess over the rounding bound is 3.0e-6",
);

/// **A rival hypothesis must be rejected by at least two printed quanta.**
///
/// One quantum can be produced by rounding alone, so a one-quantum rejection
/// is not a rejection. Two is the smallest gap no rounding can manufacture.
/// The row that uses this expresses it as
/// `2 · quantum / residual_against_the_rival ≤ 1`, which makes it an
/// *instrument-power* gate: it goes red not when lcms2 changes its answer but
/// when this measurement stops being able to tell the two readings apart.
const TOL_REJECTION: Tolerance = Tolerance::new(
    1.0,
    "the rival reading must be rejected by at least TWO printed quanta; one quantum can be \
     produced by round-to-nearest alone, so a one-quantum rejection is indistinguishable from a \
     rounding artefact. Stated as (2*quantum)/residual so it can be graded as <=",
);

/// An instrument resolves a separation only when its quantum is at most half
/// of it — the same two-quanta argument as [`TOL_REJECTION`], applied to the
/// fixture rather than to the answer.
const TOL_RESOLVING: Tolerance = Tolerance::new(
    0.5,
    "an instrument can resolve a separation only if its printed quantum is at most half of it \
     (two quanta is the smallest gap round-to-nearest cannot manufacture). Stated as \
     quantum/separation so it can be graded as <=",
);

/// The ΔE2000 line every §C row is graded against.
const TOL_PERCEPTIBLE: Tolerance = Tolerance::new(
    1.0,
    "1 dE2000 is the accepted threshold of perceptible difference for adjacent patches. The rows \
     that carry it assert the CLAIM the feature ships under - that choosing between the two \
     readings is a curiosity and not a fork in colour. If one ever failed, the option would need \
     a migration story rather than a doc comment",
);

// ===========================================================================
// The two readings, evaluated exactly as `iccce_cmm::builtin` builds them
// ===========================================================================

/// ICC `parametricCurveType` **function type 3**, which is what
/// [`srgb_trc_params`] returns and what lcms2's type-4 parametric curve
/// computes: `Y = (a·X + b)^g` for `X ≥ d`, and `Y = c·X` below it.
fn eval_type3(p: &[f64; 5], x: f64) -> f64 {
    let [g, a, b, c, d] = *p;
    if x >= d { (a * x + b).powf(g) } else { c * x }
}

/// **lcms2's** `L*` from `Y`, transcribed from `src/cmspcs.c` `f()`.
///
/// `Yn = 1` because lcms2's built-in sRGB stores D50-adapted colorants, so a
/// neutral input lands on the PCS white with `Y = 1` identically. See the
/// module header, Trap 3, for why this is lcms2's form of the constant rather
/// than ICC.1's (they are equal; the point is which one is being modelled).
fn lcms2_lstar(y: f64) -> f64 {
    const LIMIT: f64 = (24.0 / 116.0) * (24.0 / 116.0) * (24.0 / 116.0);
    let f = if y <= LIMIT {
        (841.0 / 108.0) * y + 16.0 / 116.0
    } else {
        y.cbrt()
    };
    116.0 * f - 16.0
}

/// The probe set: every 8-bit code, plus the designed interior points.
///
/// Values are **`transicc`'s own 0..255 device units**; the oracle divides by
/// 255 after `atof`, so the harness must predict from exactly `code / 255.0`
/// in `f64` or prediction and measurement would be evaluating different
/// arguments. `format!("{v}")` round-trips an `f64` exactly and `atof` reads
/// the same double back, so the trip through the pipe is lossless.
fn probe_codes(d_c0: f64, d_c1: f64, l_max_code: f64, y_max_code: f64) -> Vec<f64> {
    let mut codes: Vec<f64> = (0..=255).map(f64::from).collect();
    codes.extend([
        d_c1 * 255.0, // the C1 breakpoint: separation EXACTLY zero
        d_c0 * 255.0, // the C0 breakpoint: the other obvious wrong probe
        l_max_code,   // the L* interior maximum
        y_max_code,   // the linear-light interior maximum
        5.1,
        7.65,
        10.0725,
        10.2,
        12.75,
        17.85,
        22.95,
        23.5875,
        25.5,
        38.25,
        51.0,
        76.5,
        102.0,
        178.5,
        216.75,
    ]);
    codes
}

/// Locate the interior maximum of a separation function over the **encoded**
/// domain `0..1`, by a scan followed by a golden refinement.
///
/// Bisection on the derivative would be faster and is deliberately not used:
/// the two curves are piecewise, their derivatives are discontinuous at two
/// *different* breakpoints, and a scan cannot land on the wrong branch.
/// Returns `(separation, x)`.
fn interior_max(f: &dyn Fn(f64) -> f64) -> (f64, f64) {
    const N: usize = 400_000;
    #[allow(clippy::cast_precision_loss)]
    let n = N as f64;
    let mut best = (0.0_f64, 0.0_f64);
    for i in 0..=N {
        #[allow(clippy::cast_precision_loss)]
        let x = i as f64 / n;
        let v = f(x);
        if v > best.0 {
            best = (v, x);
        }
    }
    let (mut lo, mut hi) = ((best.1 - 2.0 / n).max(0.0), (best.1 + 2.0 / n).min(1.0));
    for _ in 0..200 {
        let m1 = lo + (hi - lo) / 3.0;
        let m2 = hi - (hi - lo) / 3.0;
        if f(m1) < f(m2) {
            lo = m1;
        } else {
            hi = m2;
        }
        if hi - lo < 1.0e-15 {
            break;
        }
    }
    let x = 0.5 * (lo + hi);
    let v = f(x);
    if v > best.0 { (v, x) } else { best }
}

// ===========================================================================
// The analysis
// ===========================================================================

/// Everything Pass L measured, so `passl_probe` can print the full table
/// without driving the oracle a second time.
#[derive(Debug, Clone)]
pub struct Analysis {
    pub p_c0: [f64; 5],
    pub p_c1: [f64; 5],
    /// Max `abs(L*_C0 − L*_C1)` over the encoded domain, and where.
    pub l_sep_max: f64,
    pub l_sep_code: f64,
    /// Max `abs(Y_C0 − Y_C1)` (linear light) over the encoded domain, and
    /// where.
    pub y_sep_max: f64,
    pub y_sep_code: f64,
    /// §A, the `*Lab` instrument.
    pub lab: Option<Instrument>,
    /// §A, the `*XYZ` instrument at the linear-light maximum.
    pub xyz_resid_c0: Option<f64>,
    pub xyz_resid_c1: Option<f64>,
    /// §A whitebox: how many of the four C⁰ constants were **not** found in
    /// the pinned source, or `None` when `vendor/lcms2` is not on disk.
    pub source_constants_missing: Option<usize>,
    /// §B control: `(distinct L* runs, samples, span in % ink)`.
    pub staircase: Option<(usize, usize, f64)>,
    pub pcs_de_max: f64,
    pub pcs_de_arg: [f64; 3],
    pub pcs_de_mean: f64,
    pub pcs_de_n: usize,
    pub pcs_de_neutral_max: f64,
    pub pcs_de_neutral_code: f64,
    /// ΔE2000 of one 16-bit PCS `L*` quantum **at the argmax point** — the
    /// yardstick the feature's own doc comment reaches for, evaluated where it
    /// matters rather than at an assumed lightness.
    pub pcs_quantum_de: f64,
    pub dests: Vec<DestCost>,
}

/// §A's per-instrument reduction.
#[derive(Debug, Clone)]
pub struct Instrument {
    pub n: usize,
    pub resid_c0_max: f64,
    pub resid_c0_code: f64,
    pub resid_c1_max: f64,
    pub resid_c1_code: f64,
    /// Of the probes whose two candidate predictions are ≥ 2 printed quanta
    /// apart, how many sit closer to each candidate.
    pub votes_c0: usize,
    pub votes_c1: usize,
    pub votes_n: usize,
    /// `(residual vs C⁰, residual vs C¹)` at the C¹ breakpoint, where the two
    /// candidates are the same number.
    pub at_c1_break: (f64, f64),
    /// `(residual vs C⁰, residual vs C¹)` at the `L*` interior maximum.
    pub at_l_max: (f64, f64),
    /// The `L*` separation available at the *linear-light* maximum — the
    /// signal a probe placed there would have had.
    pub l_sep_at_y_max: f64,
}

/// §C's per-destination reduction.
#[derive(Debug, Clone)]
pub struct DestCost {
    pub name: String,
    pub committed: bool,
    pub n: usize,
    /// Max separation of the destination's device values, normalised `0..1`.
    pub device_sep_max: f64,
    pub codes_changed_8: usize,
    pub codes_changed_16: usize,
    /// Round-trip ΔE2000 through this destination and back, when a return
    /// path is available.
    pub e2e_de_max: Option<f64>,
    pub e2e_de_mean: Option<f64>,
    pub e2e_pcs_de_max: Option<f64>,
    pub e2e_arg: [f64; 3],
}

fn lab_of(v: SrgbTrc, rgb: [f64; 3]) -> Lab {
    Lab::from_xyz(srgb_with(v).device_to_pcs(rgb), D50)
}

fn de_between(rgb: [f64; 3]) -> f64 {
    delta_e_2000(
        lab_of(SrgbTrc::ValueContinuous, rgb),
        lab_of(SrgbTrc::SlopeContinuous, rgb),
    )
}

/// The deterministic §C probe set: a 33³ encoded grid plus a 20 001-point
/// neutral ramp. The ramp is there because §A's whole subject lives on it; the
/// grid is there because §C's maximum turns out **not** to.
fn cost_probe_set() -> Vec<[f64; 3]> {
    const N: usize = 33;
    let mut pts = Vec::with_capacity(N * N * N + 20_001);
    #[allow(clippy::cast_precision_loss)]
    let g = |i: usize| i as f64 / (N - 1) as f64;
    for i in 0..N {
        for j in 0..N {
            for k in 0..N {
                pts.push([g(i), g(j), g(k)]);
            }
        }
    }
    for i in 0..=20_000 {
        #[allow(clippy::cast_precision_loss)]
        let x = i as f64 / 20_000.0;
        pts.push([x, x, x]);
    }
    pts
}

/// A coarser grid for the destination arms, which pay for a CLUT evaluation
/// per point per variant.
fn dest_probe_set() -> Vec<[f64; 3]> {
    const N: usize = 17;
    let mut pts = Vec::with_capacity(N * N * N + 256);
    #[allow(clippy::cast_precision_loss)]
    let g = |i: usize| i as f64 / (N - 1) as f64;
    for i in 0..N {
        for j in 0..N {
            for k in 0..N {
                pts.push([g(i), g(j), g(k)]);
            }
        }
    }
    for i in 0..=255 {
        let x = f64::from(i) / 255.0;
        pts.push([x, x, x]);
    }
    pts
}

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/synthetic")
}

const SYSTEM_COLOR_DIR: &str = r"C:\Windows\System32\spool\drivers\color";

/// Measure one destination arm.
///
/// `back_to` is an optional **return path** — a matrix/TRC profile the
/// destination's device values are converted into, so the result can be
/// expressed as a colour again. Both variants travel the identical return
/// path, so the return path's own error cancels and what survives is a
/// property of the two readings and of the destination. Without one, only the
/// device statistics are produced.
fn measure_dest(
    name: &str,
    committed: bool,
    dst: &Path,
    back_to: Option<&Path>,
    probes: &[[f64; 3]],
) -> Option<DestCost> {
    let prof = Profile::parse(&std::fs::read(dst).ok()?).ok()?;
    let ret = back_to.and_then(|p| {
        let bp = Profile::parse(&std::fs::read(p).ok()?).ok()?;
        let model = MatrixTrc::from_profile(&bp).ok()?;
        Some((bp, model))
    });
    let chain = ret
        .as_ref()
        .and_then(|(bp, _)| Chain::new(&prof, bp, CmmIntent::MediaRelative).ok());

    let mut device_sep_max = 0.0_f64;
    let (mut c8, mut c16) = (0_usize, 0_usize);
    let (mut e2e_max, mut e2e_sum, mut e2e_n) = (0.0_f64, 0.0_f64, 0_usize);
    let mut e2e_pcs_max = 0.0_f64;
    let mut e2e_arg = [0.0_f64; 3];

    for p in probes {
        let x0 = srgb_with(SrgbTrc::ValueContinuous).device_to_pcs(*p);
        let x1 = srgb_with(SrgbTrc::SlopeContinuous).device_to_pcs(*p);
        let d0 = Chain::convert_pcs_to_device(&prof, PcsValue::Xyz(x0)).ok()?;
        let d1 = Chain::convert_pcs_to_device(&prof, PcsValue::Xyz(x1)).ok()?;
        device_sep_max = device_sep_max.max(
            d0.iter()
                .zip(&d1)
                .map(|(a, b)| (a - b).abs())
                .fold(0.0_f64, f64::max),
        );
        let quantised_differs = |scale: f64| {
            d0.iter()
                .zip(&d1)
                .any(|(a, b)| (a * scale).round() != (b * scale).round())
        };
        if quantised_differs(255.0) {
            c8 += 1;
        }
        if quantised_differs(65_535.0) {
            c16 += 1;
        }
        if let (Some(chain), Some((_, model))) = (chain.as_ref(), ret.as_ref())
            && let (Ok(r0), Ok(r1)) = (chain.convert(&d0), chain.convert(&d1))
            && r0.len() >= 3
            && r1.len() >= 3
        {
            let l0 = Lab::from_xyz(model.device_to_pcs([r0[0], r0[1], r0[2]]), D50);
            let l1 = Lab::from_xyz(model.device_to_pcs([r1[0], r1[1], r1[2]]), D50);
            let d = delta_e_2000(l0, l1);
            e2e_sum += d;
            e2e_n += 1;
            if d > e2e_max {
                e2e_max = d;
                e2e_arg = *p;
            }
            e2e_pcs_max = e2e_pcs_max.max(de_between(*p));
        }
    }

    #[allow(clippy::cast_precision_loss)]
    let mean = (e2e_n > 0).then(|| e2e_sum / e2e_n as f64);
    Some(DestCost {
        name: name.to_string(),
        committed,
        n: probes.len(),
        device_sep_max,
        codes_changed_8: c8,
        codes_changed_16: c16,
        e2e_de_max: (e2e_n > 0).then_some(e2e_max),
        e2e_de_mean: mean,
        e2e_pcs_de_max: (e2e_n > 0).then_some(e2e_pcs_max),
        e2e_arg,
    })
}

/// Percentage helper, kept in one place so the `cast_precision_loss` allow
/// does not have to be sprinkled through the record builders.
#[allow(clippy::cast_precision_loss)]
fn pct(num: usize, den: usize) -> f64 {
    if den == 0 {
        0.0
    } else {
        100.0 * num as f64 / den as f64
    }
}

/// The ids §A emits, in order — used to emit the same rows as skips when there
/// is no oracle, so that a run on a machine without lcms2 has the same shape.
const A_LAB_IDS: [&str; 7] = [
    "passl/A/apparatus/model-exact-in-shared-region",
    "passl/A/trap/breakpoint-probe-has-zero-discriminating-power",
    "passl/A/lab/residual-against-c0-at-the-interior-maximum",
    "passl/A/lab/residual-against-c0-over-the-sweep",
    "passl/A/lab/rival-reading-is-rejected-by-two-printed-quanta",
    "passl/A/lab/probes-closer-to-the-c1-reading",
    "passl/A/design/l-signal-at-the-linear-light-max",
];

/// Run Pass L.
///
/// `oracle` is an `Option` so that §C — which needs no oracle at all, both
/// sides being iccce — still produces its numbers on a machine with no lcms2
/// build. §A and §B skip **with a reason** in that case rather than silently
/// vanishing, because a suite that emits nothing when it cannot run is
/// indistinguishable, in a log, from one that was never wired up.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn run(oracle: Option<&Oracle>) -> (Analysis, Vec<Record>) {
    let p_c0 = srgb_trc_params(SrgbTrc::ValueContinuous);
    let p_c1 = srgb_trc_params(SrgbTrc::SlopeContinuous);

    // --- where the curves actually separate, in each instrument's units ---
    let (l_sep_max, l_sep_x) = interior_max(&|x| {
        (lcms2_lstar(eval_type3(&p_c0, x)) - lcms2_lstar(eval_type3(&p_c1, x))).abs()
    });
    let (y_sep_max, y_sep_x) =
        interior_max(&|x| (eval_type3(&p_c0, x) - eval_type3(&p_c1, x)).abs());
    let l_sep_code = l_sep_x * 255.0;
    let y_sep_code = y_sep_x * 255.0;

    let mut records: Vec<Record> = Vec::new();

    // =====================================================================
    // §A — which reading does lcms2 implement?
    // =====================================================================
    let codes = probe_codes(p_c0[4], p_c1[4], l_sep_code, y_sep_code);
    let src = format!(
        "measured this run: transicc -i*sRGB -o*Lab4 -t1 -c0 -n, {} neutral probes, float I/O. \
         A FACT ABOUT lcms2, NOT a claim about what sRGB is: ICC_Spec A57 stays OPEN",
        codes.len()
    );

    let lab = oracle.and_then(|o| {
        let req = Request {
            input: Space::srgb_builtin(),
            output: Space::lab_v4(),
            intent: Intent::RelativeColorimetric,
            precalc: Precalc::Exact,
            bpc: Bpc::Off,
            values: codes.iter().flat_map(|c| [*c, *c, *c]).collect(),
        };
        let rows = o.convert_batch_shaped(&req, 3, 3).ok()?;
        let mut inst = Instrument {
            n: codes.len(),
            resid_c0_max: 0.0,
            resid_c0_code: 0.0,
            resid_c1_max: 0.0,
            resid_c1_code: 0.0,
            votes_c0: 0,
            votes_c1: 0,
            votes_n: 0,
            at_c1_break: (0.0, 0.0),
            at_l_max: (0.0, 0.0),
            l_sep_at_y_max: (lcms2_lstar(eval_type3(&p_c0, y_sep_x))
                - lcms2_lstar(eval_type3(&p_c1, y_sep_x)))
            .abs(),
        };
        let mut shared_max = 0.0_f64;
        for (code, row) in codes.iter().zip(&rows) {
            let x = code / 255.0;
            let l0 = lcms2_lstar(eval_type3(&p_c0, x));
            let l1 = lcms2_lstar(eval_type3(&p_c1, x));
            let (d0, d1) = ((row[0] - l0).abs(), (row[0] - l1).abs());
            if d0 > inst.resid_c0_max {
                inst.resid_c0_max = d0;
                inst.resid_c0_code = *code;
            }
            if d1 > inst.resid_c1_max {
                inst.resid_c1_max = d1;
                inst.resid_c1_code = *code;
            }
            if x <= p_c1[4] {
                shared_max = shared_max.max(d0);
            }
            if (l0 - l1).abs() >= 2.0 * PRINT_QUANTUM {
                inst.votes_n += 1;
                if d0 < d1 {
                    inst.votes_c0 += 1;
                } else if d1 < d0 {
                    inst.votes_c1 += 1;
                }
            }
            if (code - p_c1[4] * 255.0).abs() < 1.0e-9 {
                inst.at_c1_break = (d0, d1);
            }
            if (code - l_sep_code).abs() < 1.0e-9 {
                inst.at_l_max = (d0, d1);
            }
        }
        Some((inst, shared_max))
    });

    if let Some((inst, shared_max)) = &lab {
        // A1 — the apparatus, graded exactly where it CANNOT answer the
        // question. Its separation is zero and the report prints that.
        records.push(
            Record::graded(
                A_LAB_IDS[0],
                Kind::CrossCheck,
                Metric::AbsMaxComponent,
                TOL_PRINTED,
                *shared_max,
                src.clone(),
                format!(
                    "for encoded X <= the C1 breakpoint {:.12} the two readings ARE the same \
                     function - they share the linear segment c = 1/12.92 - so this row grades \
                     the MODEL (neutral input => Y_PCS = TRC(X) with Yn = 1, then lcms2's own \
                     f() from src/cmspcs.c) and is structurally incapable of grading the answer. \
                     max |lcms2 - model| = {shared_max:.6e} L*",
                    p_c1[4]
                ),
            )
            .with_separation(Separation::against_distance(
                "the C1 (H.273 clause 8.2) reading, which over this region is the SAME function",
                *shared_max,
                0.0,
                SepUnits::SameAsMetric,
            )),
        );

        // A2 — the trap: the obvious probe has exactly zero power.
        let sep_at_break = (lcms2_lstar(eval_type3(&p_c0, p_c1[4]))
            - lcms2_lstar(eval_type3(&p_c1, p_c1[4])))
        .abs();
        records.push(
            Record::graded(
                A_LAB_IDS[1],
                Kind::DerivedExpectation,
                Metric::AbsMaxComponent,
                Tolerance::new(
                    1.0e-12,
                    "H.273 clause 8.2 DEFINES beta by value continuity with the same linear \
                     segment the C0 reading uses, so the two curves meet at beta EXACTLY. Only \
                     f64 round-off in evaluating the two branches can appear; 1e-12 is about 1e4 \
                     ulps at L* ~ 2.7 and refuses anything structural",
                ),
                sep_at_break,
                "derived from Rec. ITU-T H.273 (V4) clause 8.2 via ICC_Spec A57; no \
                 implementation's output appears in this row",
                format!(
                    "probing at the breakpoint is the obvious choice and it is worthless: the \
                     separation there is {sep_at_break:.3e} L*. The usable maximum is INTERIOR, \
                     at code {l_sep_code:.5}, where it is {l_sep_max:.6e} L* ({:.1} printed \
                     quanta). Same trap as \
                     iccce_cmm::builtin::tests::breakpoint_is_the_c0_solution_not_the_1996_value",
                    l_sep_max / PRINT_QUANTUM
                ),
            )
            .with_separation(Separation::against_distance(
                "the interior maximum at code 23.5136, which is the signal this probe throws away",
                l_sep_max,
                0.0,
                SepUnits::SameAsMetric,
            )),
        );

        // A3 — the discriminating probe.
        records.push(
            Record::graded(
                A_LAB_IDS[2],
                Kind::CrossCheck,
                Metric::AbsMaxComponent,
                TOL_PRINTED,
                inst.at_l_max.0,
                src.clone(),
                format!(
                    "at code {l_sep_code:.5} (encoded X = {l_sep_x:.9}), the single probe with \
                     the most signal: |lcms2 - C0| = {:.6e} L*, |lcms2 - C1| = {:.6e} L*. lcms2 \
                     sits with the C0 (ICC / W3C / Khronos) reading",
                    inst.at_l_max.0, inst.at_l_max.1
                ),
            )
            .with_separation(Separation::against_distance(
                "lcms2 implements the C1 (H.273 clause 8.2) reading",
                inst.at_l_max.1,
                l_sep_max,
                SepUnits::SameAsMetric,
            )),
        );

        // A4 — the whole sweep.
        records.push(
            Record::graded(
                A_LAB_IDS[3],
                Kind::CrossCheck,
                Metric::AbsMaxComponent,
                TOL_PRINTED,
                inst.resid_c0_max,
                src.clone(),
                format!(
                    "max over {} probes = {:.6e} L* at code {:.5}; the same sweep against the C1 \
                     reading maxes at {:.6e} L* at code {:.5}, i.e. {:.1}x further away",
                    inst.n,
                    inst.resid_c0_max,
                    inst.resid_c0_code,
                    inst.resid_c1_max,
                    inst.resid_c1_code,
                    inst.resid_c1_max / inst.resid_c0_max
                ),
            )
            .with_separation(Separation::against_distance(
                "lcms2 implements the C1 (H.273 clause 8.2) reading",
                inst.resid_c1_max,
                l_sep_max,
                SepUnits::SameAsMetric,
            )),
        );

        // A5 — instrument POWER, not lcms2's answer.
        records.push(Record::graded(
            A_LAB_IDS[4],
            Kind::CrossCheck,
            Metric::AbsMaxComponent,
            TOL_REJECTION,
            2.0 * PRINT_QUANTUM / inst.resid_c1_max,
            src.clone(),
            format!(
                "(2 x {PRINT_QUANTUM:.1e}) / {:.6e} = {:.4}. This row grades the INSTRUMENT, not \
                 lcms2: it goes red when the measurement loses the power to tell the two readings \
                 apart, which is a different and earlier failure than lcms2 changing its answer",
                inst.resid_c1_max,
                2.0 * PRINT_QUANTUM / inst.resid_c1_max
            ),
        ));

        // A6 — the vote.
        #[allow(clippy::cast_precision_loss)]
        records.push(
            Record::graded(
                A_LAB_IDS[5],
                Kind::CrossCheck,
                Metric::IndicatorCount,
                Tolerance::new(
                    0.0,
                    "a count of probes whose measurement favours the rival reading. There is no \
                     instrument error in a count, and no reason any probe should split: one \
                     dissenting probe would mean the two readings are not cleanly separable and \
                     the verdict would have to be withdrawn rather than averaged",
                ),
                inst.votes_c1 as f64,
                src.clone(),
                format!(
                    "of {} probes whose two candidate predictions are >= 2 printed quanta apart, \
                     {} sit closer to C0 and {} closer to C1",
                    inst.votes_n, inst.votes_c0, inst.votes_c1
                ),
            )
            .with_separation(Separation::against_distance(
                "lcms2 implements the C1 reading, under which this count would be every probe",
                inst.votes_n as f64,
                inst.votes_n as f64,
                SepUnits::SameAsMetric,
            )),
        );

        // A7 — the design guard on the probe set's own justification.
        records.push(Record::graded(
            A_LAB_IDS[6],
            Kind::DerivedExpectation,
            Metric::AbsMaxComponent,
            Tolerance::new(
                0.5,
                "the fraction of the L* signal still available at the LINEAR-LIGHT maximum. The \
                 row keeps the probe set's justification true: if this ever rose towards 1 the \
                 two instruments would share a probe, the 'the maximum is interior AND \
                 instrument-specific' argument would be false, and the probe set would need \
                 rewriting rather than re-blessing",
            ),
            inst.l_sep_at_y_max / l_sep_max,
            "derived from the two curves; no implementation's output appears in this row",
            format!(
                "L* separation is {:.6e} at the linear-light max (code {y_sep_code:.5}) against \
                 {l_sep_max:.6e} at the L* max (code {l_sep_code:.5}): the obvious probe, taken \
                 from the feature's own doc comment, throws away {:.0}% of the signal",
                inst.l_sep_at_y_max,
                100.0 * (1.0 - inst.l_sep_at_y_max / l_sep_max)
            ),
        ));
    } else {
        for id in A_LAB_IDS {
            records.push(Record::skipped(
                id,
                Kind::CrossCheck,
                Metric::AbsMaxComponent,
                TOL_PRINTED,
                src.clone(),
                "no oracle on this machine",
            ));
        }
    }

    // --- the second instrument, probed at its own maximum ---
    let xyz = oracle.and_then(|o| {
        let req = Request {
            input: Space::srgb_builtin(),
            output: Space::Builtin("XYZ".into()),
            intent: Intent::RelativeColorimetric,
            precalc: Precalc::Exact,
            bpc: Bpc::Off,
            values: vec![y_sep_code, y_sep_code, y_sep_code],
        };
        let rows = o.convert_batch_shaped(&req, 3, 3).ok()?;
        // transicc prints XYZ scaled by 100; RGB and gray by 255; inks by 1
        // with the formatter dividing by 100. A number quoted without its
        // scale is wrong by a factor nobody notices.
        let got = *rows.first()?.get(1)?;
        Some((
            (got - eval_type3(&p_c0, y_sep_x) * 100.0).abs(),
            (got - eval_type3(&p_c1, y_sep_x) * 100.0).abs(),
        ))
    });

    if let Some((a, b)) = xyz {
        records.push(
            Record::graded(
                "passl/A/xyz/residual-against-c0-at-the-linear-light-maximum",
                Kind::CrossCheck,
                Metric::AbsMaxComponent,
                TOL_PRINTED,
                a,
                format!(
                    "measured this run: transicc -i*sRGB -o*XYZ -t1 -c0 -n at code \
                     {y_sep_code:.5}. An INDEPENDENT instrument - different output space, \
                     different probe, different unit. A FACT ABOUT lcms2; A57 stays OPEN"
                ),
                format!(
                    "|lcms2 - C0| = {a:.6e} and |lcms2 - C1| = {b:.6e}, in transicc's Y x 100 \
                     units. The linear-light separation is {:.6e} there, i.e. {:.1} printed \
                     quanta - enough, but 2.5x weaker than the Lab instrument at its own maximum",
                    y_sep_max * 100.0,
                    y_sep_max * 100.0 / PRINT_QUANTUM
                ),
            )
            .with_separation(Separation::against_distance(
                "lcms2 implements the C1 (H.273 clause 8.2) reading",
                b,
                y_sep_max * 100.0,
                SepUnits::SameAsMetric,
            )),
        );
    } else {
        records.push(Record::skipped(
            "passl/A/xyz/residual-against-c0-at-the-linear-light-maximum",
            Kind::CrossCheck,
            Metric::AbsMaxComponent,
            TOL_PRINTED,
            "measured this run",
            "no oracle on this machine",
        ));
    }

    // --- whitebox corroboration from the pinned source ---
    let vendor = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("vendor/lcms2/src/cmsvirt.c");
    let source_constants_missing = std::fs::read_to_string(&vendor).ok().map(|s| {
        ["1. / 1.055", "0.055 / 1.055", "1. / 12.92", "0.04045"]
            .iter()
            .filter(|needle| !s.contains(**needle))
            .count()
    });
    if let Some(missing) = source_constants_missing {
        #[allow(clippy::cast_precision_loss)]
        records.push(Record::graded(
            "passl/A/source/pinned-lcms2-builds-srgb-with-the-c0-constants",
            Kind::OracleReproducibility,
            Metric::IndicatorCount,
            Tolerance::new(
                0.0,
                "a count of expected constants absent from the pinned source. Zero, because the \
                 four are literal text in Build_sRGBGamma(); the row's job is to notice a pin \
                 move that changed them, which would invalidate every A-section number without \
                 changing any of them",
            ),
            missing as f64,
            format!(
                "vendor/lcms2/src/cmsvirt.c at pin 21c582a, Build_sRGBGamma() L640-647: \
                 Parameters[1] = 1./1.055, [2] = 0.055/1.055, [3] = 1./12.92, [4] = 0.04045, fed \
                 to cmsBuildParametricToneCurve(type 4). Path {}",
                vendor.display()
            ),
            "WHITEBOX. The blackbox measurement above was designed to be able to contradict this, \
             and does not. Note that cmsBuildSegmentedToneCurve leaves nSegments = 1, so \
             cmsEvalToneCurveFloat takes the ANALYTIC branch and the 4096-entry Table16 is used \
             only by 8/16-bit transforms - which is precisely why float I/O is what makes this \
             measurable at all",
        ));
    } else {
        records.push(Record::skipped(
            "passl/A/source/pinned-lcms2-builds-srgb-with-the-c0-constants",
            Kind::OracleReproducibility,
            Metric::IndicatorCount,
            Tolerance::new(0.0, "see the graded form of this row"),
            "vendor/lcms2 is git-ignored and fetched on demand",
            "vendor/lcms2/src/cmsvirt.c is not on disk: run fetch-lcms2.sh",
        ));
    }

    // =====================================================================
    // §B — the honest precision statement
    // =====================================================================
    records.push(Record::graded(
        "passl/B/lab/printed-quantum-over-available-separation",
        Kind::DerivedExpectation,
        Metric::AbsMaxComponent,
        TOL_RESOLVING,
        PRINT_QUANTUM / l_sep_max,
        "derived from transicc's %.4f and from the two curves; no measurement in this row",
        format!(
            "{PRINT_QUANTUM:.1e} / {l_sep_max:.6e} = {:.4}: the Lab instrument resolves the \
             question with {:.1} printed quanta of signal",
            PRINT_QUANTUM / l_sep_max,
            l_sep_max / PRINT_QUANTUM
        ),
    ));
    records.push(Record::graded(
        "passl/B/xyz/printed-quantum-over-available-separation",
        Kind::DerivedExpectation,
        Metric::AbsMaxComponent,
        TOL_RESOLVING,
        PRINT_QUANTUM / (y_sep_max * 100.0),
        "derived from transicc's %.4f (XYZ is printed x100) and from the two curves",
        format!(
            "{PRINT_QUANTUM:.1e} / {:.6e} = {:.4}, i.e. {:.1} printed quanta. Had this instrument \
             been probed at the L* maximum instead, the XYZ separation there is only {:.2} quanta \
             and it would NOT have resolved the question - the two instruments do not share a \
             probe",
            y_sep_max * 100.0,
            PRINT_QUANTUM / (y_sep_max * 100.0),
            y_sep_max * 100.0 / PRINT_QUANTUM,
            (eval_type3(&p_c0, l_sep_x) - eval_type3(&p_c1, l_sep_x)).abs() * 100.0 / PRINT_QUANTUM
        ),
    ));

    // --- the control that rules the oracle OUT as a destination ruler ---
    let swop = PathBuf::from(SYSTEM_COLOR_DIR).join("USWebCoatedSWOP.icc");
    let staircase = oracle.filter(|_| swop.is_file()).and_then(|o| {
        const N: usize = 60;
        const STEP: f64 = 5.0e-4;
        let mut values = Vec::with_capacity(N * 4);
        for i in 0..N {
            #[allow(clippy::cast_precision_loss)]
            let c = 30.0 + i as f64 * STEP;
            values.extend([c, 20.0, 10.0, 5.0]);
        }
        let req = Request {
            input: Space::profile(swop.clone()),
            output: Space::lab_v4(),
            intent: Intent::RelativeColorimetric,
            precalc: Precalc::Exact,
            bpc: Bpc::Off,
            values,
        };
        let rows = o.convert_batch_shaped(&req, 4, 3).ok()?;
        let mut runs = 0_usize;
        let mut last = f64::NAN;
        for r in &rows {
            if r[0] != last {
                runs += 1;
                last = r[0];
            }
        }
        #[allow(clippy::cast_precision_loss)]
        Some((runs, rows.len(), STEP * (N - 1) as f64))
    });
    if let Some((runs, n, span)) = staircase {
        #[allow(clippy::cast_precision_loss)]
        let frac = runs as f64 / n as f64;
        #[allow(clippy::cast_precision_loss)]
        let tread = span / (runs as f64 - 1.0).max(1.0);
        records.push(Record::graded(
            "passl/B/dest/oracle-a2b-input-is-quantised-to-16-bits",
            Kind::CrossCheck,
            Metric::AbsMaxComponent,
            Tolerance::new(
                0.5,
                "distinct-output runs divided by samples. A CONTINUOUS evaluator returns 1.0 - \
                 every distinct input gives a distinct output. 0.5 is the loosest bound that \
                 still refuses a continuous evaluator, and it is loose on purpose: the row's job \
                 is to establish that a staircase EXISTS, not to pin its tread, which depends on \
                 the profile's input curves as well as on the CLUT",
            ),
            frac,
            "measured this run: transicc on USWebCoatedSWOP.icc, one ink swept in 5e-4 % steps. \
             WHITEBOX explanation: lcms2 src/cmslut.c EvaluateCLUTfloatIn16 L445-456 calls \
             FromFloatTo16 before Lerp16, so a lut16Type CLUT quantises its FLOAT input",
            format!(
                "{runs} distinct L* values over {n} samples spanning {span:.4e} % ink (= {:.2} \
                 sixteen-bit quanta), so the tread is about {tread:.2e} % ink. THIS IS WHY \
                 SECTION C DOES NOT USE THE ORACLE: the largest device separation the reading \
                 choice produces at a destination is about 6.4e-3 % ink, roughly one tread, and \
                 typically far less",
                span * 65_535.0 / 100.0
            ),
        ));
    } else {
        records.push(Record::skipped(
            "passl/B/dest/oracle-a2b-input-is-quantised-to-16-bits",
            Kind::CrossCheck,
            Metric::AbsMaxComponent,
            Tolerance::new(0.5, "see the graded form of this row"),
            "measured this run",
            "no oracle, or USWebCoatedSWOP.icc absent (LEGAL.md §3 category (c))",
        ));
    }

    // =====================================================================
    // §C — what the choice costs. Self-comparison; no oracle anywhere.
    // =====================================================================
    let probes = cost_probe_set();
    let (mut pcs_de_max, mut pcs_de_sum) = (0.0_f64, 0.0_f64);
    let mut pcs_de_arg = [0.0_f64; 3];
    for p in &probes {
        let d = de_between(*p);
        pcs_de_sum += d;
        if d > pcs_de_max {
            pcs_de_max = d;
            pcs_de_arg = *p;
        }
    }
    // Coordinate descent off the grid: the maximum is NOT at a grid node, and
    // a grid-only number would understate it by ~28 %.
    {
        let mut step = 1.0 / 32.0;
        while step > 1.0e-10 {
            let mut moved = false;
            for axis in 0..3 {
                for s in [-step, step] {
                    let mut c = pcs_de_arg;
                    c[axis] = (c[axis] + s).clamp(0.0, 1.0);
                    let v = de_between(c);
                    if v > pcs_de_max {
                        pcs_de_max = v;
                        pcs_de_arg = c;
                        moved = true;
                    }
                }
            }
            if !moved {
                step *= 0.5;
            }
        }
    }
    let (mut pcs_de_neutral_max, mut pcs_de_neutral_code) = (0.0_f64, 0.0_f64);
    for i in 0..=200_000 {
        #[allow(clippy::cast_precision_loss)]
        let x = i as f64 / 200_000.0;
        let d = de_between([x, x, x]);
        if d > pcs_de_neutral_max {
            pcs_de_neutral_max = d;
            pcs_de_neutral_code = x * 255.0;
        }
    }
    // One 16-bit PCS L* quantum, expressed as a ΔE2000 AT the argmax point.
    // Evaluating it anywhere else would be dishonest: ΔE2000's SL term varies
    // by a factor of 1.6 between L* 6 and L* 50.
    let base = lab_of(SrgbTrc::ValueContinuous, pcs_de_arg);
    let bumped = Lab {
        l: base.l + 100.0 / 65_535.0,
        a: base.a,
        b: base.b,
    };
    let pcs_quantum_de = delta_e_2000(base, bumped);
    #[allow(clippy::cast_precision_loss)]
    let pcs_de_mean = pcs_de_sum / probes.len() as f64;

    let c_src = format!(
        "self-comparison: iccce vs iccce, {} probes (33^3 encoded grid + a 20 001-point neutral \
         ramp) plus coordinate descent off the grid. Computed in f64 IN PROCESS, because the \
         shipped iccce binary has NO FLAG that selects SrgbTrc - the variant is reachable only \
         through the library API (same posture as Pass 5b)",
        probes.len()
    );

    records.push(Record::graded(
        "passl/C/pcs/de2000-max-between-the-two-readings",
        Kind::SelfConsistency,
        Metric::DeltaE2000Max,
        TOL_PERCEPTIBLE,
        pcs_de_max,
        c_src.clone(),
        format!(
            "max {pcs_de_max:.6e} dE2000 at rgb ({:.9}, {:.9}, {:.9}) = codes ({:.4}, {:.4}, \
             {:.4}); mean {pcs_de_mean:.6e}. For scale, ONE 16-bit PCS L* quantum AT THAT SAME \
             POINT is {pcs_quantum_de:.6e} dE2000, so the maximum is {:.2}x a PCS quantum: the \
             'below one 16-bit quantum' statement in SrgbTrc's doc comment is about the ENCODED \
             domain and does NOT carry over to dE2000 in the PCS",
            pcs_de_arg[0],
            pcs_de_arg[1],
            pcs_de_arg[2],
            pcs_de_arg[0] * 255.0,
            pcs_de_arg[1] * 255.0,
            pcs_de_arg[2] * 255.0,
            pcs_de_max / pcs_quantum_de
        ),
    ));
    records.push(Record::graded(
        "passl/C/pcs/de2000-mean-between-the-two-readings",
        Kind::SelfConsistency,
        Metric::DeltaE2000Mean,
        TOL_PERCEPTIBLE,
        pcs_de_mean,
        c_src.clone(),
        format!(
            "mean {pcs_de_mean:.6e} dE2000 over {} probes. Reported ALONGSIDE the max and never \
             instead of it: a mean over a grid hides exactly the outlier a colour engine gets \
             wrong",
            probes.len()
        ),
    ));
    records.push(Record::graded(
        "passl/C/pcs/the-neutral-ramp-understates-the-cost",
        Kind::SelfConsistency,
        Metric::DeltaE2000Max,
        Tolerance::new(
            0.75,
            "the neutral-axis maximum as a fraction of the true maximum. A gray ramp is the \
             natural probe for a transfer-function question and it is the WRONG one here; the \
             bound refuses the reading that a 1-D probe would have been sufficient. It goes red \
             if the two ever converge, which would mean the 3-D grid has stopped buying anything \
             and the probe set should be simplified rather than kept out of habit",
        ),
        pcs_de_neutral_max / pcs_de_max,
        c_src,
        format!(
            "neutral-ramp max {pcs_de_neutral_max:.6e} at code {pcs_de_neutral_code:.4} against \
             the true max {pcs_de_max:.6e} off-axis: the ramp sees {:.0}% of the cost. The \
             maximum sits where R and B are essentially ON the C1 breakpoint (their own \
             contribution therefore zero) and G is at the L* junction - dE2000's chroma and hue \
             terms put the worst case where a 1-D probe cannot reach it",
            100.0 * pcs_de_neutral_max / pcs_de_max
        ),
    ));

    // --- destinations ---
    let dprobes = dest_probe_set();
    let mut dests: Vec<DestCost> = Vec::new();
    let fx = fixtures_dir();
    let sysdir = PathBuf::from(SYSTEM_COLOR_DIR);
    let arms: [(&str, bool, PathBuf, Option<PathBuf>); 3] = [
        (
            "v2-cmyk-mft2-lab.icc",
            true,
            fx.join("v2-cmyk-mft2-lab.icc"),
            Some(fx.join("v4-rgb-matrix-trc.icc")),
        ),
        (
            "USWebCoatedSWOP.icc",
            false,
            sysdir.join("USWebCoatedSWOP.icc"),
            Some(sysdir.join("sRGB Color Space Profile.icm")),
        ),
        (
            "AdobeRGB1998.icc",
            false,
            sysdir.join("AdobeRGB1998.icc"),
            None,
        ),
    ];
    let dev_tol = Tolerance::new(
        1.0 / 255.0,
        "one 8-bit device code. The claim being defended is that the reading choice does not MOVE \
         a destination colorant by a whole code. It says nothing about whether a value already \
         sitting on a rounding boundary can be flipped across it - that is a separate \
         measurement, and it is reported in this record's own detail rather than hidden by the \
         bound",
    );
    for (name, committed, dst, back) in arms {
        if !dst.is_file() {
            records.push(Record::skipped(
                format!("passl/C/dest/{name}/device-separation"),
                Kind::SelfConsistency,
                Metric::DeviceAbsMaxNormalised,
                dev_tol,
                "self-comparison: iccce vs iccce",
                format!("{} absent (LEGAL.md §3 category (c))", dst.display()),
            ));
            continue;
        }
        let back = back.filter(|p| p.is_file());
        let Some(cost) = measure_dest(name, committed, &dst, back.as_deref(), &dprobes) else {
            records.push(Record::errored(
                format!("passl/C/dest/{name}/device-separation"),
                Kind::SelfConsistency,
                Metric::DeviceAbsMaxNormalised,
                dev_tol,
                "self-comparison: iccce vs iccce",
                format!("could not evaluate {}", dst.display()),
            ));
            continue;
        };
        records.push(Record::graded(
            format!("passl/C/dest/{name}/device-separation"),
            Kind::SelfConsistency,
            Metric::DeviceAbsMaxNormalised,
            dev_tol,
            cost.device_sep_max,
            format!(
                "self-comparison: iccce vs iccce, {} probes (17^3 encoded grid + a 256-step \
                 neutral ramp), PCS -> device through the profile's own B2A/matrix, entirely in \
                 f64. Fixture is {}",
                cost.n,
                if cost.committed {
                    "COMMITTED (fixtures/synthetic)"
                } else {
                    "a category (c) system profile, never committed"
                }
            ),
            format!(
                "max device separation {:.6e} of full scale = 1/{:.0} of an 8-bit code. 8-BIT \
                 OUTPUT CODES THAT CHANGE: {} of {} probes ({:.2}%); at 16 bits, {} of {} \
                 ({:.1}%). The 'zero of 256 8-bit codes' figure in SrgbTrc's doc comment is about \
                 sRGB's OWN encoding and does not survive an end-to-end conversion",
                cost.device_sep_max,
                1.0 / (cost.device_sep_max * 255.0),
                cost.codes_changed_8,
                cost.n,
                pct(cost.codes_changed_8, cost.n),
                cost.codes_changed_16,
                cost.n,
                pct(cost.codes_changed_16, cost.n),
            ),
        ));
        if let (Some(e2e), Some(mean), Some(pcs)) =
            (cost.e2e_de_max, cost.e2e_de_mean, cost.e2e_pcs_de_max)
        {
            records.push(Record::graded(
                format!("passl/C/dest/{name}/end-to-end-de2000-max"),
                Kind::SelfConsistency,
                Metric::DeltaE2000Max,
                TOL_PERCEPTIBLE,
                e2e,
                format!(
                    "self-comparison: iccce vs iccce, {} probes. sRGB(variant) -> {name} device \
                     -> back through a matrix/TRC profile -> Lab(D50), all in f64. The ORACLE \
                     CANNOT MEASURE THIS - see passl/B/dest/oracle-a2b-input-is-quantised-to-16-bits",
                    cost.n
                ),
                format!(
                    "max {e2e:.6e} dE2000 at rgb ({:.6}, {:.6}, {:.6}), mean {mean:.6e}. The PCS \
                     difference alone over the same probe set is {pcs:.6e}, so this destination \
                     {} the reading choice, by {:.2}x",
                    cost.e2e_arg[0],
                    cost.e2e_arg[1],
                    cost.e2e_arg[2],
                    if e2e >= pcs { "AMPLIFIES" } else { "ATTENUATES" },
                    e2e / pcs
                ),
            ));
        }
        dests.push(cost);
    }

    let analysis = Analysis {
        p_c0,
        p_c1,
        l_sep_max,
        l_sep_code,
        y_sep_max,
        y_sep_code,
        lab: lab.map(|(i, _)| i),
        xyz_resid_c0: xyz.map(|(a, _)| a),
        xyz_resid_c1: xyz.map(|(_, b)| b),
        source_constants_missing,
        staircase,
        pcs_de_max,
        pcs_de_arg,
        pcs_de_mean,
        pcs_de_n: probes.len(),
        pcs_de_neutral_max,
        pcs_de_neutral_code,
        pcs_quantum_de,
        dests,
    };
    (analysis, records)
}

/// One line for the suite's `note` stream.
///
/// The verdict is **computed from the records**, never typed: three separate
/// incidents in this suite's history involved a hand-typed numeral in an
/// emitted string going false within a day of being written.
#[must_use]
pub fn note(a: &Analysis) -> String {
    let verdict = match &a.lab {
        Some(i) if i.votes_n > 0 && i.votes_c1 == 0 => format!(
            "lcms2 implements the C0 (value-continuous, 0.055 / 0.04045) reading: {} of {} \
             resolvable probes, max |lcms2-C0| {:.3e} L* against |lcms2-C1| {:.3e} L*",
            i.votes_c0, i.votes_n, i.resid_c0_max, i.resid_c1_max
        ),
        Some(i) if i.votes_n > 0 && i.votes_c0 == 0 => format!(
            "FINDING: lcms2 implements the C1 (H.273 clause 8.2) reading, against the \
             ICC/W3C/Khronos consensus - {} of {} resolvable probes",
            i.votes_c1, i.votes_n
        ),
        Some(i) => format!(
            "SPLIT VERDICT - {} probes favour C0 and {} favour C1; the measurement does not \
             support a conclusion",
            i.votes_c0, i.votes_c1
        ),
        None => "no oracle: the lcms2 question was not measured".to_string(),
    };
    format!(
        "{verdict}. Cost of the choice (self-comparison): PCS max {:.4e} dE2000, mean {:.4e}. \
         ICC_Spec A57 REMAINS OPEN - this measures an implementation, not the standard",
        a.pcs_de_max, a.pcs_de_mean
    )
}
