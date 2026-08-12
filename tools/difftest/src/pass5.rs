//! # Pass 5 — black point compensation
//!
//! Read `tools/difftest/README.md` **§16** for the narrative and the findings;
//! this file is the apparatus. Pass 5's done-when clause is *"BPC on and off
//! differ in the documented direction, and match lcms2's BPC within
//! tolerance"*, and the first job of this module is to establish **where that
//! sentence can be tested at all**.
//!
//! ## ★ The comparable scenario set was derived BEFORE anything was run
//!
//! Pass 4b's lesson was that lcms2's behaviour is *direction-dependent* and
//! that a rule stated without its direction is half a rule. BPC has three such
//! rules at once — an applicability set, an estimation method, and a forcing
//! policy — and each of them is keyed on something different. So the two sides'
//! reach was read out of their sources first, and the intersection is what runs.
//!
//! ### iccce's reach — `Chain::with_bpc`, at commit `46f16e8`
//!
//! | side | shape | black point iccce uses |
//! |---|---|---|
//! | matrix/TRC | any version, any non-absolute intent | `device_to_pcs([0,0,0])` — the media-relative transform of device black |
//! | grayTRC | as above | `device_to_pcs(0.0)` |
//! | `lut16`/`mAB `/`mBA ` | **v4 AND perceptual only** | the fixed A41 triple `(0.003 36, 0.003 473 1, 0.002 87)` |
//! | anything else | — | **refused by name** (`BpcEstimationUnsupported`) |
//! | ICC-absolute | any shape | **refused by name** (`BpcNotApplicable`) |
//!
//! and BPC is **never forced**: `--bpc` is the only way to get it.
//!
//! ### lcms2's reach — read at pin `21c582a`
//!
//! `_cmsLinkProfiles` sets `BPC[i]` from the caller's flag and then **forces it
//! true** when the intent is perceptual or saturation and
//! `cmsGetEncodedICCversion(hProfiles[i]) >= 0x4000000`; `DefaultICCintents`
//! consumes `BPC[i]` as the conversion *into* profile `i`, so — Pass 4b's
//! finding, `TOLERANCES.md` §3.4.4 row B8 — **the DESTINATION profile's version
//! decides.** `ComputeConversion` then calls `cmsDetectBlackPoint` on the source
//! and `cmsDetectDestinationBlackPoint` on the destination, both at the
//! requested intent, whose first-match-wins guards are:
//!
//! | guard | condition | result |
//! |---|---|---|
//! | 1 | class ∈ {`link`, `abstract`, `nmcl`} | `(0,0,0)`, FALSE |
//! | 2 | intent ∉ {perceptual, rel.col., saturation} | `(0,0,0)`, FALSE — absolute excluded twice over |
//! | 3 | version ≥ 4.0 **and** intent ∈ {perceptual, saturation} | matrix shaper → `BlackPointAsDarkerColorant(…, REL.COL.)`; otherwise **the fixed `cmsPERCEPTUAL_BLACK` constant** |
//! | 4 | `bkpt` at rel.col. | **compiled out** (`#ifdef CMS_USE_PROFILE_BLACK_POINT_TAG`) |
//! | 5 | rel.col. **and** `prtr` **and** ink space | `BlackPointUsingPerceptualBlack` (round trip through the profile) |
//! | 6 | otherwise | `BlackPointAsDarkerColorant` at the requested intent |
//!
//! and `cmsDetectDestinationBlackPoint` inserts, between guards 3 and 4, the
//! *"lut based and gray, rgb or cmyk (7.2 in Adobe's document)"* predicate:
//! a destination that is **not** CLUT-based falls back to `cmsDetectBlackPoint`
//! entirely, so a matrix/TRC destination never reaches the quadratic curve fit.
//!
//! ### ★ The consequence, stated before the numbers exist
//!
//! **Everywhere iccce will do BPC at all, lcms2's estimator reduces to the same
//! two values.** On a matrix/TRC or gray side, guard 6's darkest-colorant
//! estimate is device black carried through the profile at a colorimetric
//! intent — which is exactly `device_to_pcs(0)` — and on every profile in reach
//! that is **exactly `XYZ (0,0,0)`**, because every TRC in the corpus has
//! `trc(0) = 0`. On a v4 LUT side at perceptual, guard 3 returns the same A41
//! triple iccce hard-codes. **So Pass 5's cross-check cannot discriminate
//! between the two estimators; it grades the scaling map, the direction, and
//! the policy.** That is a coverage statement and it is made here, in advance,
//! rather than inferred afterwards from a suspiciously small number.
//!
//! The scenarios below are named S1–S6 and each says which black each side
//! uses and what was predicted, so a reader can check the prediction against
//! the observation rather than being handed only the observation.
//!
//! | # | pair | intent | iccce black src/dst | lcms2 black src/dst | map | predicted |
//! |---|---|---|---|---|---|---|
//! | **S1** | sRGB → Adobe RGB (1998), both v2 matrix/TRC | media-relative | 0 / 0 | guard 6 / not-CLUT → guard 6 | identity | on ≡ off on **both** sides. **NULL BY CONSTRUCTION** — see below |
//! | **S2** | `v4-cmyk-mab-lab.icc` → sRGB | perceptual | **A41 triple** / 0 | **guard 3 constant** / not-CLUT → guard 6 | `PB → 0`, **lowers** everything below D50 | iccce `--bpc` ≡ lcms2 `-b`; lcms2 does **not** force (destination is v2) |
//! | **S3** | sRGB → `v4-cmyk-mab-lab.icc` | perceptual | 0 / **A41 triple** | guard 6 / **guard 3 constant** | `0 → PB`, **raises** | iccce `--bpc` ≡ lcms2 with **or without** `-b` — lcms2 **forces** here (destination is v4) |
//! | **S4** | sRGB → `v4-rgb-matrix-trc.icc` | perceptual | 0 / 0 | guard 6 / **guard 3's matrix-shaper escape** → 0 | identity | forced BPC costs **exactly zero** — corpus trap **T5** measured |
//! | **S5** | sRGB → `USWebCoatedSWOP.icc` | media-relative | — | guard 6 / the **quadratic curve fit** | — | ~~iccce refuses by name; no comparison exists~~ **SUPERSEDED 2026-08-12**: the ISO estimator was wired (`c268261`) and iccce now converts. Pass 5c makes the comparison Pass 5 said did not exist |
//! | **S6** | two committed matrix fixtures | **ICC-absolute** | — | excluded by guard 2 | — | **iccce refuses by name**; lcms2 excludes it too, for the same published reason |
//!
//! ## What "null by construction" means and why S1 is still run
//!
//! S1's two arms are equal because both implementations estimate the same
//! black, not because their BPC agrees. Recording it as evidence that "BPC
//! agrees" would be exactly the error the legacy-Lab probe taught (`README`
//! §12): *an arm-comparison that comes back null may be null by construction,
//! and that must be recorded as inconclusive rather than read as a refutation.*
//! It is run because it is the **only** measurement of lcms2's darkest-colorant
//! estimate on these files — if that estimate were not zero, S1's lcms2 arm
//! would move, and it does not.
//!
//! ## What Pass 5 refuses to do
//!
//! - **It does not put an estimator in `crates/`.** The two-constraint solve
//!   that grades [`iccce_cmm::bpc::BpcScale`] lives here, in the harness.
//! - **It does not grade the policy difference.** iccce declines to force BPC
//!   where lcms2 forces it; §7.1 of `ICC_Spec/icc/icc__ref__bpc.md` records
//!   that lcms2 attributes the forcing to a document nobody in this project has
//!   read, and that the one published source (Maria 2013) is silent about it.
//!   **No clause grades it, so it is REPORTED, NOT GRADED** — the same posture
//!   §14.6 and §15.3.3 take with the other two unsettled divergences.
//! - **It does not widen a Pass 4b tolerance to make a Pass 5 row pass.** Every
//!   device tolerance below is a Pass 4b **computed envelope** multiplied by the
//!   BPC map's own gain, and the derivation says which envelope and why the
//!   multiplication is the whole correction.

use std::path::{Path, PathBuf};

use iccce_cmm::bpc::{BpcScale, PERCEPTUAL_BLACK};
use iccce_cmm::matrix_trc::MatrixTrc;
use iccce_color::{D50, Lab, Xyz, delta_e_2000};
use iccce_profile::Profile;

use crate::pass4b::{
    Unavailable, cmyk_grid, expected_mab_lab, expected_mba_cmyk, fixture_path, rgb_grid,
};
use crate::{
    Bpc, DiffError, Iccce, Intent, Kind, Metric, Oracle, Precalc, Record, Request, Space, Tolerance,
};

// ===========================================================================
// The corpus
// ===========================================================================

/// The Windows system sRGB profile — **category (c)** (`LEGAL.md` §3: read
/// locally, never committed, never a required input). The RGB side of S1–S5.
pub const SRGB: &str = r"C:\Windows\System32\spool\drivers\color\sRGB Color Space Profile.icm";

/// `Adobe RGB (1998)` — category (c). Pass 3's destination, reused for S1
/// because its **non-BPC** residual against lcms2 is already measured
/// (`TOLERANCES.md` §3.3): a null result on a pair whose noise floor is known
/// is worth more than a null result on an unmeasured pair.
pub const ADOBERGB: &str = r"C:\Windows\System32\spool\drivers\color\AdobeRGB1998.icc";

/// `U.S. Web Coated (SWOP) v2` — category (c). S5's destination: a **v2 CMYK
/// `prtr` with CLUT-based `B2A`**, i.e. precisely the case lcms2 handles with
/// the least-squares quadratic fit whose mathematics is forwarded, by the only
/// published BPC paper, to a document this project is barred from (**A42**).
pub const SWOP: &str = r"C:\Windows\System32\spool\drivers\color\USWebCoatedSWOP.icc";

/// `fixtures/synthetic/v4-rgb-matrix-trc.icc` — **category (a)**, committed.
/// S4's destination: v4.4.0.0, `mntr`, RGB, `para` funcType 0 with `g = 2,0`,
/// so `trc(0) = 0` **exactly** and both implementations' black estimate is
/// `XYZ (0,0,0)` by arithmetic rather than by measurement.
#[must_use]
pub fn v4_matrix_fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/synthetic/v4-rgb-matrix-trc.icc")
}

/// `fixtures/synthetic/v2-rgb-matrix-trc-curv.icc` — category (a). S6's source,
/// chosen with [`v4_matrix_fixture`] so the **refusal** rows never skip: a
/// refusal is a property of iccce alone and must be gradeable on a machine with
/// no colour directory at all.
#[must_use]
pub fn v2_matrix_fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/synthetic/v2-rgb-matrix-trc-curv.icc")
}

/// ICC.1:2022 **Table 16**'s printed `PCSXYZ` decimals for the perceptual
/// reference medium black point — **not** what either implementation uses.
///
/// Present so **A41** can be *measured in a pipeline* rather than quoted from
/// the corpus: the difference between this triple and [`PERCEPTUAL_BLACK`] is
/// 0,037 ΔE76 at black in the abstract, and §B reports what it is worth in
/// device units through an actual transform. See
/// `ICC_Spec/icc/icc__ref__bpc.md` §3.
pub const TABLE16_BLACK: Xyz = Xyz {
    x: 0.003_357,
    y: 0.003_479,
    z: 0.002_869,
};

/// lcms2's `MAX_ENCODEABLE_XYZ`, `lcms2_internal.h` L71 — `1 + 32767/32768`.
///
/// `ComputeConversion` divides the BPC offset by this before the stage is
/// inserted, because the stage operates on lcms2's *encoded* PCSXYZ where 1,0
/// means `MAX_ENCODEABLE_XYZ`. It is needed here only by [`is_empty_layer_diff`].
const MAX_ENCODEABLE_XYZ: f64 = 1.0 + 32767.0 / 32768.0;

// ===========================================================================
// Tolerances — every one written before the corresponding comparison was run
// ===========================================================================

/// **§A.** `iccce_cmm::bpc::BpcScale` against the two independent statements of
/// the same map: ICC.1:2022 **6.3.4.3**'s printed equation (valid only for
/// source black zero) and a **Gaussian elimination** on Maria (2013)'s two
/// constraints (valid generally).
///
/// The bound is arithmetic, not colorimetric, and it is derived rather than
/// chosen. The three forms are algebraically identical **in ℝ** and differ in
/// `f64` only by rounding — the Pass 4b lesson that "exact in a spec-derived
/// argument means exact in ℝ" applies here in its purest form. Counting
/// roundings on the worst path: the spec form is `x·(1 − Xb/Xi) + Xb`, three
/// operations; iccce's is `((Xi − Xd)/(Xi − Xs))·x + Xi(Xd − Xs)/(Xi − Xs)`,
/// six. Every intermediate is bounded by `Xi ≤ 1`, so each rounding is at most
/// 1 ulp of 1,0 = 2,22×10⁻¹⁶, and the division by `Xi − Xs ≈ 0,96` amplifies by
/// at most 1,04. Worst case ≈ 9 × 2,22×10⁻¹⁶ × 1,04 ≈ **2,1×10⁻¹⁵**.
///
/// `1×10⁻¹⁴` is ~4,8× that. It is **10 orders below one `u16` lsb of the
/// encoded PCS** (1,53×10⁻⁵), so a genuine algebraic defect — a dropped offset,
/// a transposed numerator, the `+`/`−` of the ICC.1 form read the wrong way —
/// still fails by many orders. A tolerance of `0,0` was considered and
/// rejected: the three routes are not the same operations in the same order,
/// which is precisely the condition §4's B0 row records as making 0,0
/// unavailable.
pub const MAP_ALGEBRA: Tolerance = Tolerance::new(
    1e-14,
    "the three algebraic forms of the same map (ICC.1:2022 6.3.4.3's printed equation, a Gaussian \
     elimination on Maria 2013's two constraints, and iccce's closed form) are identical in R and \
     differ in f64 only by rounding: at most 9 roundings on the longest path, each <= 1 ulp of 1.0 \
     (2.22e-16), amplified by at most 1.04 by the division by (Xi - Xs) ~ 0.96, so <= 2.1e-15. \
     1e-14 is ~4.8x that and 10 orders below one u16 lsb of the encoded PCS (1.53e-5), so a \
     dropped offset or a transposed numerator still fails. NOT 0.0: the three routes are not the \
     same operations in the same order (the B0 lesson, TOLERANCES.md §4)",
);

/// **§B, S2 device space** — `v4-cmyk-mab-lab.icc` → sRGB at perceptual, iccce
/// `--bpc` against `transicc -b`, worst component over the grid, normalised.
///
/// **This is Pass 4b's row B6 envelope times the BPC map's gain, and nothing
/// else, because BPC adds no quantisation of the kind that envelope models.**
/// The argument, in order:
///
/// 1. B6 measures the identical chain with BPC off, and its envelope was
///    *computed* from lcms2's own arithmetic: the destination's 4096-entry
///    `cmsReverseToneCurveEx` resampling, measured independently by §C at
///    **9,68×10⁻⁵** device, plus the fixture CLUT's own `u16` lsb of
///    **1,5×10⁻⁵** carried through — ≈**1,15×10⁻⁴**.
/// 2. Switching BPC on inserts, in lcms2, one `cmsStageAllocMatrix` between two
///    stages that were already there (`AddConversion`'s Lab→XYZ / XYZ→Lab pair
///    for a Lab-PCS chain). A matrix stage performs **no table lookup and no
///    `u16` rounding**; it contributes only `f32` stage-boundary rounding,
///    ≈6×10⁻⁸ relative on a PCS value ≤ 1, which the destination's inverse TRC
///    amplifies by at most `d(device)/d(linear) = 12,92` below sRGB's
///    breakpoint — **≈7,8×10⁻⁷**, under 1 % of term 1.
/// 3. The map itself multiplies the PCS by `a = Xi/(Xi − Xb) = 1,003 5`, so any
///    error already present is amplified by **0,35 %**.
///
/// Envelope `= 1,15×10⁻⁴ × 1,0035 + 7,8×10⁻⁷ ≈ 1,16×10⁻⁴`. `2,5×10⁻⁴` is
/// **~2,2×** that — deliberately the same headroom factor B6 carries, since the
/// residual term it covers (lcms2's 16-bit *fixed-point* interpolation, which
/// the `f64` model does not reproduce) is unchanged by BPC.
///
/// **The one thing this derivation inherits rather than recomputes** is *where*
/// the reverse-curve error is largest. BPC moves the operating point toward the
/// shadow. §C's 9,68×10⁻⁵ is a maximum taken over the whole `[0,1]` gray axis,
/// so a redistribution of which inputs occur cannot exceed it — but the axis was
/// sampled at 69 points, not continuously, and that is the gap the headroom
/// covers. Stated because an unstated inheritance is how a tolerance stops
/// meaning anything.
///
/// **EXCLUDES the 10 encoded-PCS-overflow points** (`K < 1/256`), which are
/// §15.3.3's unsettled clamp divergence and would swamp any BPC measurement.
pub const DEVICE_FIXTURE_TO_SRGB: Tolerance = Tolerance::new(
    2.5e-4,
    "Pass 4b row B6's COMPUTED envelope for the identical BPC-off chain (lcms2's 4096-entry \
     cmsReverseToneCurveEx resampling of sRGB's TRCs, 9.68e-5, plus the fixture CLUT's u16 lsb, \
     1.5e-5 = 1.15e-4) times the BPC map's own gain a = Xi/(Xi - Xb) = 1.0035, plus the f32 \
     stage-boundary rounding of the single matrix stage BPC inserts (6e-8 relative, amplified by \
     at most 12.92 by sRGB's inverse TRC below its breakpoint = 7.8e-7): 1.16e-4. 2.5e-4 is ~2.2x, \
     the same headroom B6 carries for lcms2's 16-bit FIXED-POINT interpolation, which BPC does not \
     change. BPC adds NO table lookup and NO u16 rounding - it is one matrix between two stages \
     that were already in the pipeline. EXCLUDES the 10 encoded-PCS-overflow points (README \
     §15.3.3). Arithmetic agreement, NOT perceptual",
);

/// **§B, S2 in ΔE2000** — the same disagreement expressed perceptually, both
/// sides' RGB carried into D50 CIELAB through **sRGB's own matrix/TRC model**.
///
/// Derived by Pass 4b row C3's chain, re-run with §B's device envelope. The
/// maximum is **near black and chromatic**, not near white and luminous: below
/// sRGB's linear breakpoint a device difference `δ` becomes `δ/12,92` of linear
/// light, and CIELAB's chromatic sensitivity on its own linear segment is
/// `da*/dX = 500 × 7,787/X_n = 4038`, giving `Δa* ≈ 136 δ` against
/// `ΔL* ≈ 69,9 δ`; with `S_C ≈ 1` and `S_L ≈ 1,75` near neutral the chromatic
/// term dominates by ~3×. At `δ = 1,16×10⁻⁴` that is
/// `Δa* ≈ 1,58×10⁻²` and a union of ≈**2,4×10⁻²** ΔE00.
///
/// `5×10⁻²` is ~2,1× that, and **20× below §2's ⚠ provisional 1,0
/// perceptibility anchor**, whose ⚠ it inherits. **BPC makes this bound harder
/// to reach, not easier**, because it maps the darkest inputs *below* zero where
/// both implementations clamp to device 0 and agree exactly; the bound is
/// nevertheless derived at the unclamped shadow, which is the conservative
/// reading.
pub const DE_FIXTURE_TO_SRGB: Tolerance = Tolerance::new(
    5e-2,
    "row C3's amplification chain re-run with this section's device envelope: below sRGB's linear \
     breakpoint a device difference d becomes d/12.92 of linear light, CIELAB's chromatic \
     sensitivity on its linear segment is da*/dX = 4038, so da* ~ 136 d against dL* ~ 69.9 d, and \
     with S_C ~ 1 against S_L ~ 1.75 the chromatic term dominates by ~3x; at d = 1.16e-4 the union \
     is ~2.4e-2 dE00. 5e-2 is ~2.1x that and 20x below the provisional 1.0 perceptibility anchor, \
     whose provisional mark it inherits. Derived at the UNCLAMPED shadow, which is conservative: \
     BPC maps the darkest inputs below zero, where both sides clamp to 0 and agree exactly",
);

/// **§C, S3 device space** — sRGB → `v4-cmyk-mab-lab.icc` at perceptual, iccce
/// `--bpc` against `transicc` (with **and** without `-b`, since lcms2 forces
/// here), worst component over the grid, normalised.
///
/// Pass 4b row B5's envelope times the same gain. B5's derivation: the
/// interpolation-method term is zero by construction (the fixture's CLUTs are
/// affine, row B0), so what remains is lcms2's `u16` quantisation of the CLUT
/// boundary carried into device units — **1 lsb = 1,5×10⁻⁵**, unamplified,
/// because this table has unit slope in device per normalised PCS unit. Times
/// the BPC gain 1,0035, plus the same 7,8×10⁻⁷ `f32` term, ≈**1,6×10⁻⁵**.
///
/// `1×10⁻⁴` is ~6,4× that, covering lcms2's fixed-point CLUT interpolation and
/// `transicc`'s 4-decimal CMYK print floor (10⁻⁶ normalised). It is B5's
/// constant unchanged, and that is deliberate: **a tolerance that moves when
/// the only change is a linear stage would be a tolerance tracking the
/// observation.**
pub const DEVICE_SRGB_TO_FIXTURE: Tolerance = Tolerance::new(
    1e-4,
    "Pass 4b row B5's envelope, unchanged: the interpolation-method term is zero by construction \
     (the fixture's CLUTs are affine, row B0), leaving lcms2's u16 CLUT-boundary quantisation of \
     1.5e-5, unamplified because this table has unit slope in device per normalised PCS unit; \
     times the BPC gain 1.0035 plus the f32 stage term 7.8e-7 = 1.6e-5. 1e-4 is ~6.4x, covering \
     lcms2's fixed-point CLUT interpolation and transicc's 1e-6 normalised print floor. Kept at \
     B5's constant deliberately: a tolerance that moved when the only change is a linear stage \
     would be a tolerance tracking the observation",
);

/// **The identity rows**, graded at exactly zero.
///
/// Used where the claim is that *nothing may move at all* and no arithmetic in
/// either chain could make a difference small rather than absent:
///
/// - S1 and S4, **within** each implementation: both sides estimate the same
///   black, so `ComputeConversion`'s `BlackPointIn != BlackPointOut` test fails
///   and lcms2 inserts no stage, while `BpcScale` with equal blacks yields
///   `a = 1,0` and `b = 0,0` **exactly** in `f64` (the numerator `Xi − Xd` and
///   denominator `Xi − Xs` are the same expression on the same bits, and
///   `Xi(Xd − Xs)` is `Xi × 0`). The two outputs are then the same function of
///   the same input, evaluated by the same code.
/// - S3's forcing row: lcms2 with `-b` and without must be **the same bytes**,
///   because the flag is overwritten before it is read.
///
/// This is the one place `0,0` is available, and for the reason §4's B0 row
/// gives: the two sides are the same operations in the same order.
pub const EXACT: Tolerance = Tolerance::new(
    0.0,
    "the two arms are the same operations in the same order - lcms2 inserts no stage when its two \
     black estimates compare equal, and BpcScale with equal blacks is a = 1.0 and b = 0.0 exactly \
     in f64 (same expression on the same bits; Xi*(Xd-Xs) is Xi*0). No arithmetic in either chain \
     could make a difference small rather than absent, so any nonzero value is a dispatch or \
     estimation defect, not noise. This is the case §4's B0 row identifies as the one where 0.0 is \
     available",
);

/// **The direction row.** The observed quantity is the **largest signed
/// increase** `max(with_bpc − without_bpc)` over every component of every grid
/// point, and the tolerance is `0,0`, so the row passes exactly when **no
/// component ever rises**.
///
/// It is a `0,0` of a different kind from [`EXACT`] — a sign test, not an
/// equality — and it is exact for an algebraic reason. iccce's BPC output is
/// `a·X + b` with `a = (Xi − Xd)/(Xi − Xs)` and `b = Xi(Xd − Xs)/(Xi − Xs)`, so
///
/// ```text
///   out − in = (a − 1)·X + b = (Xd − Xs)/(Xi − Xs) · (Xi − X)
/// ```
///
/// — a product whose second factor is `≥ 0` for any in-gamut PCS value and
/// whose first factor carries the **sign of `Xd − Xs`**. In S2 the source black
/// is the A41 triple and the destination's is zero, so `Xd − Xs < 0` in every
/// channel and **every PCS value at or below the white is lowered, strictly
/// below it**. The destination's tone curves are monotone increasing, so the
/// sign survives into device space. That is the *documented direction* of Pass
/// 5's done-when clause, and it is checkable without any tolerance at all.
pub const DIRECTION: Tolerance = Tolerance::new(
    0.0,
    "a sign test with an algebraic proof, not an equality: out - in = (Xd - Xs)/(Xi - Xs) * \
     (Xi - X), whose second factor is >= 0 for any in-gamut PCS value, so the sign of the shift is \
     the sign of (Xd - Xs) at every point. In S2 the destination black is zero and the source's is \
     the A41 triple, so every channel must fall; the destination's tone curves are monotone \
     increasing, so the sign survives into device space. Observed = the largest signed INCREASE; \
     any positive value is a direction defect",
);

/// **The refusal rows.** Observed is `0,0` when the shipped binary refused with
/// the expected named reason and `1,0` otherwise; the tolerance is `0,0`.
///
/// A refusal is graded, not merely reported, because *refusing where it cannot
/// estimate* is a property iccce claims (`CLAUDE.md` rule 6: the parser reports,
/// it does not repair — and the CMM does not guess). A build that quietly
/// substituted a zero black for an unestimable one would produce plausible
/// colour and pass every other row in this file.
pub const REFUSAL: Tolerance = Tolerance::new(
    0.0,
    "0.0 when the shipped binary exited non-zero with the expected named refusal, 1.0 otherwise. \
     Graded rather than reported because refusing where it cannot estimate is a property iccce \
     claims: a build that substituted a zero black for an unestimable one would produce plausible \
     colour and pass every other row in this file",
);

/// **S3's closed-form lift.** `|observed lift − predicted lift|` at device
/// black, in normalised device units.
///
/// The prediction is exact arithmetic, so the bound is the two **print floors**
/// on the observation and nothing else. `iccce transform` prints device values
/// at 6 decimals, so each of the two arms carries ±5×10⁻⁷ and their difference
/// ±10⁻⁶. `5×10⁻⁶` is **5× that**, which admits the `f64` evaluation of the
/// `mBA ` interpolation on both sides of the subtraction and refuses anything
/// larger — in particular it refuses the wrong perceptual-black triple, whose
/// signature here is `ΔL* = 5,4×10⁻³`, i.e. `ΔK ≈ 5,4×10⁻⁵`, **11× this
/// bound**. So this row also functions as the A41 discriminator.
pub const LIFT_CLOSED_FORM: Tolerance = Tolerance::new(
    5e-6,
    "the prediction is exact arithmetic, so the bound is the observation's print floor and \
     nothing else: `iccce transform` prints 6 decimals, so each arm carries +-5e-7 and their \
     difference +-1e-6. 5e-6 is 5x that. It REFUSES the wrong perceptual-black triple, whose \
     signature here is dL* = 5.4e-3 i.e. dK ~ 5.4e-5, 11x this bound - so the row doubles as the \
     A41 discriminator",
);

/// Reported, never graded.
pub const UNGRADED: Tolerance = Tolerance::new(
    f64::INFINITY,
    "REPORTED, NOT GRADED - no clause and no published document settles what the right answer is, \
     so grading it would mean either a tolerance chosen because it passed or a permanent red line. \
     Both were rejected in writing (TOLERANCES.md §3.4.2, §3.4.4 rows B7/B8)",
);

// ===========================================================================
// §A — the map, three ways
// ===========================================================================

/// ICC.1:2022 **6.3.4.3**'s printed adjustment, verbatim in shape:
/// `Xp = Xt·(1 − Xb/Xi) + Xb`.
///
/// **Valid only where the source black is zero** — which is exactly the
/// assumption the clause states in prose (*"transforms … frequently use zero to
/// represent the black point"*). It is not a general BPC map and this function
/// must never be used as one; the corpus's §2.2 proves the general map
/// specialises to it, which is the direction the argument runs.
///
/// `xi` is the PCS white component, `xb` the destination black component.
#[must_use]
pub fn spec_6_3_4_3(xt: f64, xb: f64, xi: f64) -> f64 {
    xt * (1.0 - xb / xi) + xb
}

/// Maria (2013) §4.2's two constraints, solved by **Gaussian elimination with
/// the pivot on the white row** — deliberately a different sequence of `f64`
/// operations from the closed form `iccce-cmm` evaluates.
///
/// The constraints, verbatim from the paper:
///
/// ```text
///   D50         = scaleXYZ * D50         + offsetXYZ
///   XYZblackDST = scaleXYZ * XYZblackSRC + offsetXYZ
/// ```
///
/// As a system in `(a, b)` per channel: `[[Xi, 1], [Xs, 1]] · (a, b)ᵀ =
/// (Xi, Xd)ᵀ`. `|Xi| > |Xs|` for any black point worthy of the name, so the
/// pivot is row 1 and no exchange is needed; eliminating gives
/// `b = (Xd − Xs)/(1 − Xs/Xi)` and then `a = (Xi − b)/Xi`.
///
/// Returns `None` when `Xi − Xs ≤ 0`, matching `BpcScale::new`'s refusal rather
/// than producing a number for a "black" at or above the white.
#[must_use]
pub fn maria_two_constraint_solve(xs: f64, xd: f64, xi: f64) -> Option<(f64, f64)> {
    if xi - xs <= 0.0 {
        return None;
    }
    let factor = xs / xi;
    let b = (xd - xs) / (1.0 - factor);
    let a = (xi - b) / xi;
    Some((a, b))
}

/// lcms2's `IsEmptyLayer` discriminant for a BPC map — **a finding this
/// project had not recorded before Pass 5.**
///
/// `cmscnvrt.c` L327–348 sums `Σ|m_ij − I_ij| + Σ|off_k|` and **drops the whole
/// stage when that sum is below `0,002`**. The offsets are already divided by
/// `MAX_ENCODEABLE_XYZ` at that point (`ComputeConversion`'s last loop), so the
/// discriminant is computed here in the same units.
///
/// The consequence is quantitative and it is not in
/// `ICC_Spec/icc/icc__ref__bpc.md` §7.2's list of unattributed constants,
/// because that list was drawn from `cmssamp.c`: **lcms2 silently performs no
/// BPC at all when the two black points are close enough**, and "close enough"
/// is roughly `ΔL* ≲ 0,4` near black. iccce has no such threshold and will
/// apply the map however small it is. Neither behaviour is sourced.
#[must_use]
pub fn is_empty_layer_diff(a: [f64; 3], b: [f64; 3]) -> f64 {
    let mut diff = 0.0;
    for v in a {
        diff += (v - 1.0).abs();
    }
    for v in b {
        diff += (v / MAX_ENCODEABLE_XYZ).abs();
    }
    diff
}

/// The per-channel `(a, b)` iccce computes, recovered from the public
/// [`BpcScale`] API by evaluating it at two points.
///
/// [`BpcScale`]'s fields are private — correctly, since a scale factor is not
/// part of its contract — so the harness recovers them the way any external
/// observer would: `apply(1) − apply(0)` is `a` and `apply(0)` is `b`, exactly,
/// because the map is affine and `1` and `0` are exact in `f64`.
#[must_use]
pub fn recover_ab(scale: &BpcScale) -> ([f64; 3], [f64; 3]) {
    let zero = scale.apply(Xyz {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    });
    let one = scale.apply(Xyz {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    });
    (
        [one.x - zero.x, one.y - zero.y, one.z - zero.z],
        [zero.x, zero.y, zero.z],
    )
}

/// §A's result.
#[derive(Debug)]
pub struct MapAnalysis {
    /// Worst deviation of `BpcScale(0 → PB)` from ICC.1 6.3.4.3's printed form.
    pub vs_spec: f64,
    /// Worst deviation of `BpcScale(bs → bd)` from the Gaussian solve, over the
    /// randomised sweep.
    pub vs_maria: f64,
    /// Worst residual of the two constraints under iccce's own map:
    /// `|apply(D50) − D50|` and `|apply(bs) − bd|`.
    pub constraint_residual: f64,
    /// `|apply(x) − x|` maximum when the two blacks are equal — must be zero.
    pub equal_blacks_identity: f64,
    /// How many `(bs, bd, x)` draws the sweep used.
    pub draws: usize,
    /// The `IsEmptyLayer` discriminant for the S2/S3 map (`PB` against zero),
    /// and the black difference in `ΔL*` at which lcms2 would start to drop it.
    pub empty_layer_diff: f64,
    pub empty_layer_threshold_dl: f64,
}

/// Run §A. Needs **no profile and no oracle** — it is arithmetic against two
/// documents — so it is the one section of Pass 5 that cannot skip.
#[must_use]
pub fn analyse_map() -> MapAnalysis {
    let w = [D50.x, D50.y, D50.z];
    let pb = [PERCEPTUAL_BLACK.x, PERCEPTUAL_BLACK.y, PERCEPTUAL_BLACK.z];

    // --- 1. against ICC.1 6.3.4.3, which requires source black zero ---------
    let zero = Xyz {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let scale = BpcScale::new(zero, PERCEPTUAL_BLACK).expect("D50 - 0 > 0");
    let mut vs_spec: f64 = 0.0;
    // Sweep the whole PCS range plus the two anchor points the clause names.
    let mut xs_sweep: Vec<f64> = (0..=1000).map(|i| f64::from(i) / 1000.0).collect();
    xs_sweep.extend_from_slice(&[D50.x, D50.y, D50.z, PERCEPTUAL_BLACK.y]);
    for &t in &xs_sweep {
        let got = scale.apply(Xyz { x: t, y: t, z: t });
        let want = [
            spec_6_3_4_3(t, pb[0], w[0]),
            spec_6_3_4_3(t, pb[1], w[1]),
            spec_6_3_4_3(t, pb[2], w[2]),
        ];
        vs_spec = vs_spec
            .max((got.x - want[0]).abs())
            .max((got.y - want[1]).abs())
            .max((got.z - want[2]).abs());
    }

    // --- 2. against Maria 2013's two constraints, general blacks ------------
    // A deterministic LCG (MMIX constants), as everywhere else in this suite.
    let mut state: u64 = 0x1CCC_E000_0005_00BC;
    let mut next = || -> f64 {
        state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        #[allow(clippy::cast_precision_loss)]
        let u = (state >> 11) as f64 / ((1u64 << 53) as f64);
        u
    };
    let draws = 20_000usize;
    let mut vs_maria: f64 = 0.0;
    let mut constraint_residual: f64 = 0.0;
    for _ in 0..draws {
        // Blacks in [0, 0.05] - the physically meaningful range, an order
        // above the PRM black and far below the white.
        let bs = Xyz {
            x: next() * 0.05,
            y: next() * 0.05,
            z: next() * 0.05,
        };
        let bd = Xyz {
            x: next() * 0.05,
            y: next() * 0.05,
            z: next() * 0.05,
        };
        let t = next();
        let Some(s) = BpcScale::new(bs, bd) else {
            continue;
        };
        let got = s.apply(Xyz { x: t, y: t, z: t });
        let src = [bs.x, bs.y, bs.z];
        let dst = [bd.x, bd.y, bd.z];
        for c in 0..3 {
            let Some((a, b)) = maria_two_constraint_solve(src[c], dst[c], w[c]) else {
                continue;
            };
            let want = a * t + b;
            let mine = [got.x, got.y, got.z][c];
            vs_maria = vs_maria.max((mine - want).abs());
        }
        // The two constraints, under iccce's own map.
        let at_white = s.apply(D50);
        let at_black = s.apply(bs);
        constraint_residual = constraint_residual
            .max((at_white.x - D50.x).abs())
            .max((at_white.y - D50.y).abs())
            .max((at_white.z - D50.z).abs())
            .max((at_black.x - bd.x).abs())
            .max((at_black.y - bd.y).abs())
            .max((at_black.z - bd.z).abs());
    }

    // --- 3. equal blacks must be the exact identity -------------------------
    let mut equal_blacks_identity: f64 = 0.0;
    let s = BpcScale::new(PERCEPTUAL_BLACK, PERCEPTUAL_BLACK).expect("D50 - PB > 0");
    for i in 0..=1000 {
        let t = f64::from(i) / 1000.0;
        let got = s.apply(Xyz { x: t, y: t, z: t });
        equal_blacks_identity = equal_blacks_identity
            .max((got.x - t).abs())
            .max((got.y - t).abs())
            .max((got.z - t).abs());
    }

    // --- 4. lcms2's empty-layer discriminant for the S2/S3 map --------------
    let s23 = BpcScale::new(PERCEPTUAL_BLACK, zero).expect("D50 - PB > 0");
    let (a23, b23) = recover_ab(&s23);
    let empty_layer_diff = is_empty_layer_diff(a23, b23);
    // The Y-channel black difference at which the discriminant reaches 0,002,
    // holding the other two channels proportional to D50, converted to L* by
    // CIELAB's linear segment (L* = 903,296 296… × Y for Y below the knee).
    let mut lo = 0.0f64;
    let mut hi = 0.05f64;
    for _ in 0..200 {
        let mid = f64::midpoint(lo, hi);
        let bs = Xyz {
            x: mid * D50.x,
            y: mid,
            z: mid * D50.z,
        };
        let d = BpcScale::new(bs, zero).map_or(f64::INFINITY, |sc| {
            let (a, b) = recover_ab(&sc);
            is_empty_layer_diff(a, b)
        });
        if d < 0.002 {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let empty_layer_threshold_dl = (841.0 / 108.0) * 116.0 * hi;

    MapAnalysis {
        vs_spec,
        vs_maria,
        constraint_residual,
        equal_blacks_identity,
        draws,
        empty_layer_diff,
        empty_layer_threshold_dl,
    }
}

// ===========================================================================
// The scenario runner
// ===========================================================================

/// One profile pair run four ways: `{iccce, lcms2} × {BPC off, BPC on}`.
///
/// Every one of the four arms differs from its partner in **exactly one flag**,
/// which is the whole point: a second code path per arm would be free to differ
/// in the intent or the profile order and the difference would be attributed to
/// BPC.
#[derive(Debug)]
pub struct Scenario {
    pub id: &'static str,
    pub intent: Intent,
    pub in_channels: usize,
    pub out_channels: usize,
    /// iccce's device output, 0..1, with `--bpc` absent then present.
    pub iccce_off: Vec<Vec<f64>>,
    pub iccce_on: Vec<Vec<f64>>,
    /// lcms2's device output, **already normalised to 0..1**, `-b` absent then
    /// present.
    pub lcms2_off: Vec<Vec<f64>>,
    pub lcms2_on: Vec<Vec<f64>>,
}

impl Scenario {
    /// Worst absolute component difference between two arms, over the points
    /// `keep` selects.
    #[must_use]
    pub fn max_abs(a: &[Vec<f64>], b: &[Vec<f64>], keep: &[bool]) -> f64 {
        a.iter()
            .zip(b)
            .zip(keep)
            .filter(|(_, k)| **k)
            .flat_map(|((x, y), _)| x.iter().zip(y).map(|(p, q)| (p - q).abs()))
            .fold(0.0f64, f64::max)
    }

    /// Worst **signed** difference `a − b`. Negative means every component of
    /// every point fell.
    #[must_use]
    pub fn max_signed(a: &[Vec<f64>], b: &[Vec<f64>], keep: &[bool]) -> f64 {
        a.iter()
            .zip(b)
            .zip(keep)
            .filter(|(_, k)| **k)
            .flat_map(|((x, y), _)| x.iter().zip(y).map(|(p, q)| p - q))
            .fold(f64::NEG_INFINITY, f64::max)
    }

    /// Worst signed difference in the other direction — the size of the lift.
    #[must_use]
    pub fn min_signed(a: &[Vec<f64>], b: &[Vec<f64>], keep: &[bool]) -> f64 {
        a.iter()
            .zip(b)
            .zip(keep)
            .filter(|(_, k)| **k)
            .flat_map(|((x, y), _)| x.iter().zip(y).map(|(p, q)| p - q))
            .fold(f64::INFINITY, f64::min)
    }
}

/// Drive one scenario's four arms.
///
/// `rows` are source device values in 0..1. `lcms2_scale` is what `transicc`'s
/// **input** values must be multiplied by (255 for 8-bit RGB, 100 for CMYK
/// percentages) and `out_scale` what its **output** must be divided by —
/// README §9's convention, restated at every call site because a number quoted
/// without its scale is wrong by 255.
#[allow(clippy::too_many_arguments)]
fn run_scenario(
    id: &'static str,
    oracle: &Oracle,
    iccce: &Iccce,
    src: &Path,
    dst: &Path,
    intent: Intent,
    rows: &[Vec<f64>],
    out_channels: usize,
    in_scale: f64,
    out_scale: f64,
) -> Result<Scenario, Unavailable> {
    let in_channels = rows.first().map_or(0, Vec::len);
    let iccce_off = iccce.transform_rows_shaped_bpc(src, dst, intent, rows, out_channels, false)?;
    let iccce_on = iccce.transform_rows_shaped_bpc(src, dst, intent, rows, out_channels, true)?;

    let base = Request {
        input: Space::profile(src),
        output: Space::profile(dst),
        intent,
        precalc: Precalc::Exact,
        bpc: Bpc::Off,
        values: rows.iter().flat_map(|r| r.iter().map(|v| v * in_scale)).collect(),
    };
    let off = oracle.convert_batch_shaped(&base, in_channels, out_channels)?;
    let on_req = Request {
        bpc: Bpc::On,
        ..base
    };
    let on = oracle.convert_batch_shaped(&on_req, in_channels, out_channels)?;
    let norm = |v: Vec<Vec<f64>>| -> Vec<Vec<f64>> {
        v.into_iter()
            .map(|r| r.into_iter().map(|x| x / out_scale).collect())
            .collect()
    };

    Ok(Scenario {
        id,
        intent,
        in_channels,
        out_channels,
        iccce_off,
        iccce_on,
        lcms2_off: norm(off),
        lcms2_on: norm(on),
    })
}

// ===========================================================================
// The whole analysis
// ===========================================================================

/// Everything Pass 5 measured.
#[derive(Debug)]
pub struct Pass5 {
    /// §A — always present.
    pub map: MapAnalysis,
    /// S1: sRGB → Adobe RGB (1998), media-relative. Needs the colour directory.
    pub s1: Option<Scenario>,
    /// S2: the v4 fixture → sRGB, perceptual, plus its derived quantities.
    pub s2: Option<S2>,
    /// S3: sRGB → the v4 fixture, perceptual, plus the policy measurement.
    pub s3: Option<S3>,
    /// S4: sRGB → the v4 **matrix/TRC** fixture, perceptual — trap T5.
    pub s4: Option<Scenario>,
    /// S5/S6: the two refusals. `Ok(())` when the expected refusal happened.
    pub refusal_lut: Option<Result<String, String>>,
    pub refusal_absolute: Result<String, String>,
    /// Free text for the report banner.
    pub structure: String,
}

/// S2 and the quantities that only make sense there.
#[derive(Debug)]
pub struct S2 {
    pub scenario: Scenario,
    /// `false` at the 10 points where the `mAB ` matrix pushes the encoded PCS
    /// above 1,0 — §15.3.3's unsettled clamp divergence, excluded from every
    /// graded row here for the reason row B6 excludes it.
    pub keep: Vec<bool>,
    pub overflow_count: usize,
    /// ΔE2000 between iccce `--bpc` and lcms2 `-b`, per point.
    pub de_on: Vec<f64>,
    /// ΔE2000 between the BPC-off arms — the baseline this section's residual
    /// must be compared against before any of it is attributed to BPC.
    pub de_off: Vec<f64>,
    /// The largest **ΔE2000 that BPC itself moves**, iccce's own two arms.
    pub bpc_effect_de: f64,
    /// The A41 sensitivity: the same map rebuilt with ICC.1 Table 16's printed
    /// decimals instead of the implementations' triple, evaluated over this
    /// grid's PCS values. Max ΔE2000, max ΔE76 (the corpus's unit, so the two
    /// figures are directly comparable) and max ΔL*.
    pub a41_de: f64,
    pub a41_de76: f64,
    pub a41_dl: f64,
}

/// S3 and the policy measurement.
#[derive(Debug)]
pub struct S3 {
    pub scenario: Scenario,
    /// `|lcms2(-b) − lcms2(no -b)|` — the forcing, measured. Must be 0,0.
    pub forcing: f64,
    /// ★ `|iccce(no --bpc) − lcms2(no -b)|` — **the policy difference**, in
    /// device units and in `L*` through the `mBA ` closed form.
    pub policy_device: f64,
    pub policy_dl: f64,
    /// The **sign** of the policy difference in `L*`: positive when lcms2's
    /// output is *lighter* than iccce's.
    pub policy_lcms2_is_lighter: bool,
    /// ★ The BPC lift **at device black**, in closed form and as observed.
    ///
    /// This is the one place in Pass 5 where the whole end-to-end effect of BPC
    /// has an expectation with no implementation's output in it: `RGB (0,0,0)`
    /// gives `XYZ (0,0,0)` through sRGB's matrix/TRC exactly, BPC's **second
    /// constraint** sends that to the destination black exactly, and the
    /// fixture's `mBA ` closed form (row B3) turns the resulting `L*` into `K`.
    /// So the predicted `K` with and without BPC are both arithmetic.
    /// `K(no BPC) − K(BPC)` predicted in closed form.
    pub lift_predicted: f64,
    /// The same, observed through the shipped binary's two arms.
    pub lift_iccce: f64,
    /// The predicted `K` **with** BPC at device black, and what each
    /// implementation actually printed there. lcms2 has no BPC-off arm here
    /// (it forces), so its lift cannot be observed — only its endpoint can, and
    /// that is what is recorded.
    pub k_on_predicted: f64,
    pub k_on_iccce: f64,
    pub k_on_lcms2: f64,
}

/// Convert a `K` difference on this fixture's `mBA ` lower segment into `ΔL*`.
///
/// The stored `K` nodes are `65535`, `32768`, `0` at `n_L = 0, ½, 1`, so on the
/// lower segment `dK/dn_L = (32768/65535 − 1)/½` and `dn_L/dL* = 1/100`. Using
/// the **stored** node rather than an idealised `½` matters here for the same
/// reason it matters in row B3: it is a 1,5×10⁻⁵ difference that would look like
/// an implementation defect.
#[must_use]
pub fn k_to_dl(dk: f64) -> f64 {
    let slope = (1.0 - 32768.0 / 65535.0) / 0.5 / 100.0;
    dk / slope
}

fn to_lab(model: &MatrixTrc, rgb: &[f64]) -> Lab {
    Lab::from_xyz(model.device_to_pcs([rgb[0], rgb[1], rgb[2]]), D50)
}

/// Run every scenario that can run on this machine.
///
/// # Errors
/// Returns [`Unavailable::Skip`] when a category (c) profile is absent or the
/// shipped binary has not been built, and [`Unavailable::Error`] when the
/// oracle or the harness itself fails. Neither is ever a pass.
pub fn analyse(oracle: &Oracle) -> Result<Pass5, Unavailable> {
    let map = analyse_map();

    let iccce = match Iccce::locate() {
        Ok(Some(i)) => i,
        Ok(None) => {
            return Err(Unavailable::Skip(
                "iccce binary not built: run `cargo build --release -p iccce-cli`".into(),
            ));
        }
        Err(e) => return Err(Unavailable::Error(e.to_string())),
    };

    // --- S6: the ICC-absolute refusal. Committed fixtures only, so this row
    // --- grades on a machine with no colour directory at all.
    let v2m = v2_matrix_fixture();
    let v4m = v4_matrix_fixture();
    let refusal_absolute = expect_refusal(
        &iccce,
        &v2m,
        &v4m,
        Intent::AbsoluteColorimetric,
        &[vec![0.5, 0.5, 0.5]],
        3,
        // The exact wording iccce prints, not a paraphrase: a needle that
        // merely says "refused" would pass on the WRONG refusal, which is how
        // an ICC-absolute row silently becomes an estimation-subset row.
        "excluded at the absolute intent",
    );

    let srgb = Path::new(SRGB);
    let adobe = Path::new(ADOBERGB);
    let swop = Path::new(SWOP);
    let fx = fixture_path();

    let mut structure = format!(
        "map: 6.3.4.3 sweep 1005 pts + {} random two-constraint draws",
        map.draws
    );

    // --- S1 -----------------------------------------------------------------
    let s1 = if srgb.is_file() && adobe.is_file() {
        let rows: Vec<Vec<f64>> = rgb_grid().iter().map(|t| t.to_vec()).collect();
        let s = run_scenario(
            "S1",
            oracle,
            &iccce,
            srgb,
            adobe,
            Intent::RelativeColorimetric,
            &rows,
            3,
            255.0,
            255.0,
        )?;
        structure.push_str(&format!("; S1 {} RGB pts", rows.len()));
        Some(s)
    } else {
        None
    };

    // --- S2 -----------------------------------------------------------------
    let s2 = if srgb.is_file() {
        let cmyk = cmyk_grid();
        let rows: Vec<Vec<f64>> = cmyk.iter().map(|q| q.to_vec()).collect();
        let scenario = run_scenario(
            "S2",
            oracle,
            &iccce,
            &fx,
            srgb,
            Intent::Perceptual,
            &rows,
            3,
            100.0,
            255.0,
        )?;
        // The overflow mask, computed exactly as row B6 computes it.
        let keep: Vec<bool> = cmyk.iter().map(|q| (1.0 - q[3]) + 1.0 / 256.0 <= 1.0).collect();
        let overflow_count = keep.iter().filter(|k| !**k).count();

        let srgb_bytes = std::fs::read(srgb).map_err(|e| Unavailable::Error(e.to_string()))?;
        let srgb_profile =
            Profile::parse(&srgb_bytes).map_err(|e| Unavailable::Error(e.to_string()))?;
        let srgb_model = MatrixTrc::from_profile(&srgb_profile)
            .map_err(|e| Unavailable::Error(e.to_string()))?;

        let mut de_on = Vec::new();
        let mut de_off = Vec::new();
        let mut bpc_effect_de = 0.0f64;
        // The index is used to reach five parallel vectors, so `enumerate` over
        // one of them would only move the indexing rather than remove it.
        #[allow(clippy::needless_range_loop)]
        for i in 0..rows.len() {
            let on_mine = to_lab(&srgb_model, &scenario.iccce_on[i]);
            let on_theirs = to_lab(&srgb_model, &scenario.lcms2_on[i]);
            let off_mine = to_lab(&srgb_model, &scenario.iccce_off[i]);
            let off_theirs = to_lab(&srgb_model, &scenario.lcms2_off[i]);
            de_on.push(delta_e_2000(on_mine, on_theirs));
            de_off.push(delta_e_2000(off_mine, off_theirs));
            if keep[i] {
                bpc_effect_de = bpc_effect_de.max(delta_e_2000(on_mine, off_mine));
            }
        }

        // ★ A41 measured in a pipeline: rebuild the map with ICC.1 Table 16's
        // printed decimals and evaluate it on the same PCS values the fixture
        // produces (the closed form of row B1, iccce's clamped reading).
        let impl_map = BpcScale::new(PERCEPTUAL_BLACK, Xyz { x: 0.0, y: 0.0, z: 0.0 })
            .ok_or_else(|| Unavailable::Error("BpcScale refused the A41 triple".into()))?;
        let spec_map = BpcScale::new(TABLE16_BLACK, Xyz { x: 0.0, y: 0.0, z: 0.0 })
            .ok_or_else(|| Unavailable::Error("BpcScale refused Table 16's triple".into()))?;
        let mut a41_de = 0.0f64;
        let mut a41_de76 = 0.0f64;
        let mut a41_dl = 0.0f64;
        for (i, q) in cmyk.iter().enumerate() {
            if !keep[i] {
                continue;
            }
            let xyz = expected_mab_lab(*q, true).to_xyz(D50);
            let a = Lab::from_xyz(impl_map.apply(xyz), D50);
            let b = Lab::from_xyz(spec_map.apply(xyz), D50);
            a41_de = a41_de.max(delta_e_2000(a, b));
            a41_de76 = a41_de76.max(
                ((a.l - b.l).powi(2) + (a.a - b.a).powi(2) + (a.b - b.b).powi(2)).sqrt(),
            );
            a41_dl = a41_dl.max((a.l - b.l).abs());
        }

        structure.push_str(&format!(
            "; S2 {} CMYK pts ({overflow_count} excluded as encoded-PCS overflow)",
            rows.len()
        ));
        Some(S2 {
            scenario,
            keep,
            overflow_count,
            de_on,
            de_off,
            bpc_effect_de,
            a41_de,
            a41_de76,
            a41_dl,
        })
    } else {
        None
    };

    // --- S3 -----------------------------------------------------------------
    let s3 = if srgb.is_file() {
        let rows: Vec<Vec<f64>> = rgb_grid().iter().map(|t| t.to_vec()).collect();
        let scenario = run_scenario(
            "S3",
            oracle,
            &iccce,
            srgb,
            &fx,
            Intent::Perceptual,
            &rows,
            4,
            255.0,
            100.0,
        )?;
        let all = vec![true; rows.len()];
        let forcing = Scenario::max_abs(&scenario.lcms2_on, &scenario.lcms2_off, &all);
        let policy_device = Scenario::max_abs(&scenario.iccce_off, &scenario.lcms2_off, &all);
        let policy_dl = k_to_dl(policy_device);
        // lcms2 lighter <=> lcms2's K is LOWER than iccce's at the point of
        // maximum disagreement. Taken at that point, not averaged: a sign is
        // not a statistic.
        let mut worst = 0.0f64;
        let mut lighter = false;
        for (mine, theirs) in scenario.iccce_off.iter().zip(&scenario.lcms2_off) {
            for (p, q) in mine.iter().zip(theirs) {
                if (p - q).abs() > worst {
                    worst = (p - q).abs();
                    lighter = q < p;
                }
            }
        }
        // ★ The closed-form lift at device black. The grid's black point is
        // located by value rather than by index: an index would silently
        // follow a future reordering of `rgb_grid` into the wrong row.
        let black_at = rows
            .iter()
            .position(|r| r.iter().all(|v| *v == 0.0))
            .ok_or_else(|| Unavailable::Error("rgb_grid has no (0,0,0) point".into()))?;
        let k_off = expected_mba_cmyk(Lab {
            l: 0.0,
            a: 0.0,
            b: 0.0,
        })[3];
        let k_on = expected_mba_cmyk(Lab::from_xyz(PERCEPTUAL_BLACK, D50))[3];
        let lift_predicted = k_off - k_on;
        let lift_iccce = scenario.iccce_off[black_at][3] - scenario.iccce_on[black_at][3];
        let k_on_iccce = scenario.iccce_on[black_at][3];
        let k_on_lcms2 = scenario.lcms2_on[black_at][3];

        structure.push_str(&format!("; S3 {} RGB pts", rows.len()));
        Some(S3 {
            scenario,
            forcing,
            policy_device,
            policy_dl,
            policy_lcms2_is_lighter: lighter,
            lift_predicted,
            lift_iccce,
            k_on_predicted: k_on,
            k_on_iccce,
            k_on_lcms2,
        })
    } else {
        None
    };

    // --- S4 -----------------------------------------------------------------
    let s4 = if srgb.is_file() {
        let rows: Vec<Vec<f64>> = rgb_grid().iter().map(|t| t.to_vec()).collect();
        let s = run_scenario(
            "S4",
            oracle,
            &iccce,
            srgb,
            &v4m,
            Intent::Perceptual,
            &rows,
            3,
            255.0,
            255.0,
        )?;
        structure.push_str(&format!("; S4 {} RGB pts", rows.len()));
        Some(s)
    } else {
        None
    };

    // --- S5: the estimation-subset refusal ----------------------------------
    // ★ INVERTED 2026-08-12. S5 used to assert a REFUSAL here: a v2 CMYK
    // LUT destination at media-relative was outside iccce's black-point
    // estimation subset and the shipped binary exited 1 by name. Pass 5b
    // found that `bpc::estimate_lut_destination_black` implemented ISO/CD
    // 18619 4.2.5 in full and had NO CALLER; commit `c268261` wired it into
    // `Chain::estimate_dst_black`; the case now converts.
    //
    // The row is kept and inverted rather than deleted, because the sentence
    // it used to carry - "SO NO COMPARISON EXISTS FOR THIS CASE and Pass 5
    // claims none" - is now false, and a coverage gap that closes should be
    // visible closing rather than quietly absent. Pass 5c is the comparison.
    let refusal_lut = if srgb.is_file() && swop.is_file() {
        Some(
            iccce
                .transform_rows_shaped_bpc(
                    srgb,
                    swop,
                    Intent::RelativeColorimetric,
                    &[vec![0.5, 0.5, 0.5]],
                    4,
                    true,
                )
                .map(|rows| format!("converted: {:?}", rows[0]))
                .map_err(|e| format!("STILL REFUSES - {e}")),
        )
    } else {
        None
    };

    Ok(Pass5 {
        map,
        s1,
        s2,
        s3,
        s4,
        refusal_lut,
        refusal_absolute,
        structure,
    })
}

/// Invoke the shipped binary with `--bpc` where a refusal is expected.
///
/// `Ok(stderr)` when it exited non-zero **and** the message contains
/// `needle`; `Err(what happened instead)` otherwise. An answer where a refusal
/// was expected is the failure this checks for, and it is reported with the
/// answer attached so the reader does not have to re-run it.
fn expect_refusal(
    iccce: &Iccce,
    src: &Path,
    dst: &Path,
    intent: Intent,
    rows: &[Vec<f64>],
    out_channels: usize,
    needle: &str,
) -> Result<String, String> {
    match iccce.transform_rows_shaped_bpc(src, dst, intent, rows, out_channels, true) {
        Ok(v) => Err(format!(
            "expected a refusal containing {needle:?}; got an answer: {v:?}"
        )),
        Err(DiffError::NonZeroExit { stderr, code, .. }) => {
            if stderr.contains(needle) {
                Ok(stderr.trim().to_string())
            } else {
                Err(format!(
                    "exited {code:?} but the message did not contain {needle:?}: {}",
                    stderr.trim()
                ))
            }
        }
        Err(e) => Err(format!("expected a named refusal; got a harness error: {e}")),
    }
}

// ===========================================================================
// The records
// ===========================================================================

const SRC_BOTH_COMPUTED: &str =
    "both sides computed in this run (2026-08-11): iccce = target/release/iccce.exe, \
     lcms2 = transicc at pin 21c582a";
const SRC_SPEC: &str =
    "ICC.1:2022 clause 6.3.4.3 (verbatim in ICC_Spec/icc/icc__ref__bpc.md §2.1, verified by two \
     PDF extraction engines) and Maria (2013) §4.2's two constraints (littlecms.com, robots.txt \
     Allow: /). No implementation's output enters either expectation";
const SRC_LCMS2_READ: &str =
    "lcms2 source READ at pin 21c582a (cmscnvrt.c ComputeConversion/IsEmptyLayer, cmssamp.c \
     cmsDetectBlackPoint/cmsDetectDestinationBlackPoint) - the prediction; the number is measured";

/// Build every Pass 5 record.
#[must_use]
pub fn records(p: &Pass5) -> Vec<Record> {
    let mut out = Vec::new();

    // ---- §A ---------------------------------------------------------------
    out.push(Record::graded(
        "pass5/map/iccce-vs-icc1-6.3.4.3",
        Kind::DerivedExpectation,
        Metric::AbsMaxComponent,
        MAP_ALGEBRA,
        p.map.vs_spec,
        SRC_SPEC,
        "BpcScale(black_src=0 -> black_dst=A41 triple) against the clause's printed \
         Xp = Xt(1-Xb/Xi)+Xb over 1005 PCS values including D50's three components. THE ONLY \
         PRIMARY-SPECIFICATION ROW PASS 5 HAS: the map is in ICC.1, the estimation is not (A42)",
    ));
    out.push(Record::graded(
        "pass5/map/iccce-vs-maria-two-constraint-solve",
        Kind::DerivedExpectation,
        Metric::AbsMaxComponent,
        MAP_ALGEBRA,
        p.map.vs_maria,
        SRC_SPEC,
        format!(
            "BpcScale(bs -> bd) against a Gaussian elimination on the published two constraints, \
             {} random (bs, bd, x) draws with both blacks in [0, 0.05]. Generalises the row above, \
             which 6.3.4.3 can only state for bs = 0",
            p.map.draws
        ),
    ));
    out.push(Record::graded(
        "pass5/map/constraints-hold-under-iccce",
        Kind::DerivedExpectation,
        Metric::AbsMaxComponent,
        MAP_ALGEBRA,
        p.map.constraint_residual,
        SRC_SPEC,
        "the two constraints evaluated under iccce's own map: apply(D50) = D50 and \
         apply(black_src) = black_dst. Catches a map that is self-consistent but anchored on the \
         wrong white - the failure the absolute-intent exclusion exists to prevent",
    ));
    out.push(Record::graded(
        "pass5/map/equal-blacks-are-the-exact-identity",
        Kind::SelfConsistency,
        Metric::AbsMaxComponent,
        EXACT,
        p.map.equal_blacks_identity,
        SRC_BOTH_COMPUTED,
        "BpcScale(PB -> PB) applied to 1001 values must return them unchanged, bit for bit. This \
         is what makes S1's and S4's null results interpretable: iccce's no-op is exact, so a \
         nonzero difference there would be an estimation defect and not rounding",
    ));
    out.push(Record::graded(
        "pass5/map/lcms2-empty-layer-threshold",
        Kind::DerivedExpectation,
        Metric::AbsMaxComponent,
        UNGRADED,
        p.map.empty_layer_diff,
        SRC_LCMS2_READ,
        format!(
            "★ A CONSTANT NOT PREVIOUSLY RECORDED. lcms2's IsEmptyLayer (cmscnvrt.c L327-348) sums \
             the matrix's deviation from the identity plus the offsets (already divided by \
             MAX_ENCODEABLE_XYZ) and DROPS THE WHOLE BPC STAGE below 0.002. For the S2/S3 map the \
             discriminant is {:.6}, i.e. {:.1}x the threshold, so BPC is applied. The threshold \
             corresponds to a source-destination black difference of about {:.3} L* near black: \
             lcms2 silently performs NO BPC below that and iccce has no such threshold. Neither \
             behaviour is sourced; ICC_Spec §7.2's list of unattributed constants was drawn from \
             cmssamp.c and does not contain this one",
            p.map.empty_layer_diff,
            p.map.empty_layer_diff / 0.002,
            p.map.empty_layer_threshold_dl
        ),
    ));

    // ---- S1 ---------------------------------------------------------------
    match &p.s1 {
        None => {
            out.push(Record::skipped(
                "pass5/S1/srgb-to-adobergb/media-relative/null-control",
                Kind::OracleReproducibility,
                Metric::DeviceAbsMaxNormalised,
                EXACT,
                SRC_BOTH_COMPUTED,
                "sRGB and/or AdobeRGB1998 not present (LEGAL.md §3 category (c))",
            ));
        }
        Some(s) => {
            let all = vec![true; s.iccce_off.len()];
            out.push(Record::graded(
                "pass5/S1/srgb-to-adobergb/media-relative/lcms2-bpc-is-a-no-op",
                Kind::OracleReproducibility,
                Metric::DeviceAbsMaxNormalised,
                EXACT,
                Scenario::max_abs(&s.lcms2_on, &s.lcms2_off, &all),
                SRC_BOTH_COMPUTED,
                "BOTH SIDES ARE lcms2: -b against no -b on two v2 matrix/TRC profiles at \
                 media-relative. NULL BY CONSTRUCTION and recorded as such - guard 6's \
                 darkest-colorant estimate is XYZ(0,0,0) for both files, so ComputeConversion's \
                 BlackPointIn != BlackPointOut test fails and no stage is inserted. It is \
                 INCONCLUSIVE as evidence that the two implementations' BPC agrees; what it does \
                 establish is that lcms2's estimate on these files really is zero, which S2's and \
                 S3's predictions depend on",
            ));
            out.push(Record::graded(
                "pass5/S1/srgb-to-adobergb/media-relative/iccce-bpc-is-a-no-op",
                Kind::SelfConsistency,
                Metric::DeviceAbsMaxNormalised,
                EXACT,
                Scenario::max_abs(&s.iccce_on, &s.iccce_off, &all),
                SRC_BOTH_COMPUTED,
                "the same on iccce's side: --bpc against no --bpc. Both profiles are matrix/TRC \
                 with trc(0) = 0, so device_to_pcs([0,0,0]) is XYZ(0,0,0) on both sides and \
                 BpcScale is the exact identity. NULL BY CONSTRUCTION, same caveat",
            ));
        }
    }

    // ---- S2 ---------------------------------------------------------------
    match &p.s2 {
        None => {
            for id in [
                "pass5/S2/fixture-to-srgb/perceptual/bpc-on-device-vs-lcms2",
                "pass5/S2/fixture-to-srgb/perceptual/bpc-off-device-vs-lcms2",
                "pass5/S2/fixture-to-srgb/perceptual/bpc-on-de2000-vs-lcms2",
                "pass5/S2/fixture-to-srgb/perceptual/direction-nothing-rises",
            ] {
                out.push(Record::skipped(
                    id,
                    Kind::CrossCheck,
                    Metric::DeviceAbsMaxNormalised,
                    DEVICE_FIXTURE_TO_SRGB,
                    SRC_BOTH_COMPUTED,
                    "sRGB not present (LEGAL.md §3 category (c))",
                ));
            }
        }
        Some(s) => {
            let sc = &s.scenario;
            out.push(Record::graded(
                "pass5/S2/fixture-to-srgb/perceptual/bpc-off-device-vs-lcms2",
                Kind::CrossCheck,
                Metric::DeviceAbsMaxNormalised,
                DEVICE_FIXTURE_TO_SRGB,
                Scenario::max_abs(&sc.iccce_off, &sc.lcms2_off, &s.keep),
                SRC_BOTH_COMPUTED,
                format!(
                    "THE BASELINE, and it is graded first on purpose: the same chain with BPC off \
                     on both sides, so the row below cannot attribute to BPC a residual that was \
                     there anyway. lcms2 does NOT force BPC here - the destination is v2 sRGB and \
                     the forcing is keyed on the DESTINATION version (row B8). {} of {} points \
                     excluded as encoded-PCS overflow",
                    s.overflow_count,
                    sc.iccce_off.len()
                ),
            ));
            out.push(Record::graded(
                "pass5/S2/fixture-to-srgb/perceptual/bpc-on-device-vs-lcms2",
                Kind::CrossCheck,
                Metric::DeviceAbsMaxNormalised,
                DEVICE_FIXTURE_TO_SRGB,
                Scenario::max_abs(&sc.iccce_on, &sc.lcms2_on, &s.keep),
                SRC_BOTH_COMPUTED,
                format!(
                    "★ THE ROW PASS 5's DONE-WHEN CLAUSE 2 RESTS ON. iccce --bpc against transicc \
                     -b. Both sides estimate the same two blacks - the A41 triple from lcms2's \
                     guard 3 and iccce's hard-coded constant for the v4 LUT source, XYZ(0,0,0) \
                     from lcms2's guard 6 and iccce's device black for the matrix/TRC destination \
                     - so this grades the SCALING MAP and the pipeline it sits in, NOT the \
                     estimators, which cannot be discriminated here and are not claimed to be. \
                     ★ THE SENSITIVITY, WHICH IS WHAT MAKES A SMALL NUMBER MEAN ANYTHING: BPC \
                     itself moves this transform by up to {:.4} ΔE2000, and the two \
                     implementations disagree by {:.4e} device. The comparison is {:.0}x more \
                     sensitive than the effect it is grading, so 'they agree' is not a statement \
                     about a comparison that could not tell. \
                     ★ AGAINST THE BASELINE ROW: switching BPC on moved the residual by a factor \
                     {:.3}, where the tolerance's derivation predicted 1.0035 (the map's gain \
                     alone). The excess is the term that derivation flagged as INHERITED RATHER \
                     THAN RECOMPUTED - BPC moves the operating point into the shadow, where \
                     lcms2's 4096-entry reverse tone curve resamples less well. The observation \
                     therefore CONFIRMS the flagged risk is real and prices it; the envelope still \
                     bounds it because §C's 9.68e-5 was a maximum over the whole gray axis, not \
                     over the BPC-off operating point",
                    s.bpc_effect_de,
                    Scenario::max_abs(&sc.iccce_on, &sc.lcms2_on, &s.keep),
                    Scenario::min_signed(&sc.iccce_on, &sc.iccce_off, &s.keep).abs()
                        / Scenario::max_abs(&sc.iccce_on, &sc.lcms2_on, &s.keep),
                    Scenario::max_abs(&sc.iccce_on, &sc.lcms2_on, &s.keep)
                        / Scenario::max_abs(&sc.iccce_off, &sc.lcms2_off, &s.keep)
                ),
            ));
            out.push(Record::graded(
                "pass5/S2/fixture-to-srgb/perceptual/bpc-on-de2000-vs-lcms2",
                Kind::CrossCheck,
                Metric::DeltaE2000Max,
                DE_FIXTURE_TO_SRGB,
                s.de_on
                    .iter()
                    .zip(&s.keep)
                    .filter(|(_, k)| **k)
                    .map(|(d, _)| *d)
                    .fold(0.0f64, f64::max),
                SRC_BOTH_COMPUTED,
                "the same disagreement in D50 CIELAB, both sides' RGB carried through sRGB's own \
                 matrix/TRC model. A device number is not a colour statement until it is in a space \
                 where a ΔE means something",
            ));
            out.push(Record::graded(
                "pass5/S2/fixture-to-srgb/perceptual/de2000-baseline-bpc-off",
                Kind::CrossCheck,
                Metric::DeltaE2000Max,
                DE_FIXTURE_TO_SRGB,
                s.de_off
                    .iter()
                    .zip(&s.keep)
                    .filter(|(_, k)| **k)
                    .map(|(d, _)| *d)
                    .fold(0.0f64, f64::max),
                SRC_BOTH_COMPUTED,
                "the ΔE baseline with BPC off on both sides, for the same reason the device \
                 baseline exists",
            ));
            out.push(Record::graded(
                "pass5/S2/fixture-to-srgb/perceptual/direction-nothing-rises",
                Kind::SelfConsistency,
                Metric::DeviceAbsMaxNormalised,
                DIRECTION,
                Scenario::max_signed(&sc.iccce_on, &sc.iccce_off, &s.keep),
                SRC_BOTH_COMPUTED,
                format!(
                    "★ PASS 5's DONE-WHEN CLAUSE 1. The observed value is the largest SIGNED \
                     increase over every component of every kept point; it must be <= 0 because \
                     out - in = (Xd - Xs)/(Xi - Xs) * (Xi - X) and here Xd (zero) < Xs (the A41 \
                     triple) in every channel. The largest fall is {:.6e} device and the largest \
                     colour BPC moves in this scenario is {:.4} ΔE2000",
                    Scenario::min_signed(&sc.iccce_on, &sc.iccce_off, &s.keep).abs(),
                    s.bpc_effect_de
                ),
            ));
            out.push(Record::graded(
                "pass5/S2/fixture-to-srgb/perceptual/lcms2-does-not-force-here",
                Kind::OracleReproducibility,
                Metric::DeviceAbsMaxNormalised,
                UNGRADED,
                Scenario::max_abs(&sc.lcms2_on, &sc.lcms2_off, &s.keep),
                SRC_BOTH_COMPUTED,
                "BOTH SIDES ARE lcms2: -b against no -b. Non-zero, and that is the point - the \
                 destination is v2, so nothing is forced and the flag is the only thing turning \
                 BPC on. This is the control that makes S3's forcing row mean something",
            ));
            out.push(Record::graded(
                "pass5/S2/a41-cost-measured-in-a-pipeline",
                Kind::DerivedExpectation,
                Metric::DeltaE2000Max,
                UNGRADED,
                s.a41_de,
                SRC_SPEC,
                format!(
                    "★ A41 PRICED WHERE IT IS ACTUALLY SPENT. The same map rebuilt with ICC.1 \
                     Table 16's printed PCSXYZ decimals (0.003357 / 0.003479 / 0.002869) instead \
                     of the triple lcms2 and ICC's own iccDEV both use (0.00336 / 0.0034731 / \
                     0.00287), evaluated on this grid's PCS values from row B1's closed form: \
                     {:.6} ΔE2000, {:.6} ΔE76 and {:.6} ΔL* maximum. ★ BOTH CORPUS FIGURES ARE \
                     CORROBORATED BY AN INDEPENDENT ROUTE: it derived 0.005 3 ΔL* and 0.037 437 \
                     ΔE76 in Python by two passes, and this is Rust, through a fixture's stored \
                     bytes, in a different pipeline - agreeing to 5e-5 ΔE76. The ΔE2000 (1.34x the \
                     ΔE76 here, because near-neutral low chroma triggers CIEDE2000's G factor \
                     while S_L divides the small ΔL*) is NEW; the corpus never computed one. \
                     ★ NOTE THE SIZE: it is the same order as this section's entire agreement \
                     budget (5e-2), so on a FLOAT path the choice of digits is not negligible \
                     against the measurement noise - which is not a contradiction of the corpus's \
                     'invisible at 16-bit' reading but its complement, and the reason iccce \
                     follows the implementations' triple and bpc.rs's doc comment says so",
                    s.a41_de, s.a41_de76, s.a41_dl
                ),
            ));
        }
    }

    // ---- S3 ---------------------------------------------------------------
    match &p.s3 {
        None => {
            for id in [
                "pass5/S3/srgb-to-fixture/perceptual/bpc-on-device-vs-lcms2",
                "pass5/S3/srgb-to-fixture/perceptual/lcms2-forces-bpc-unasked",
                "pass5/S3/srgb-to-fixture/perceptual/POLICY-iccce-never-forces",
            ] {
                out.push(Record::skipped(
                    id,
                    Kind::CrossCheck,
                    Metric::DeviceAbsMaxNormalised,
                    DEVICE_SRGB_TO_FIXTURE,
                    SRC_BOTH_COMPUTED,
                    "sRGB not present (LEGAL.md §3 category (c))",
                ));
            }
        }
        Some(s) => {
            let sc = &s.scenario;
            let all = vec![true; sc.iccce_off.len()];
            out.push(Record::graded(
                "pass5/S3/srgb-to-fixture/perceptual/lcms2-forces-bpc-unasked",
                Kind::OracleReproducibility,
                Metric::DeviceAbsMaxNormalised,
                EXACT,
                s.forcing,
                SRC_BOTH_COMPUTED,
                "BOTH SIDES ARE lcms2: -b against no -b, into a v4 destination at perceptual. \
                 Graded at exactly zero because the flag is OVERWRITTEN before it is read \
                 (_cmsLinkProfiles sets BPC[i] = TRUE unconditionally here), so asking and not \
                 asking must produce the same bytes. This is corpus M2 re-measured in the \
                 direction row B8 showed is the one that matters",
            ));
            out.push(Record::graded(
                "pass5/S3/srgb-to-fixture/perceptual/bpc-on-device-vs-lcms2",
                Kind::CrossCheck,
                Metric::DeviceAbsMaxNormalised,
                DEVICE_SRGB_TO_FIXTURE,
                Scenario::max_abs(&sc.iccce_on, &sc.lcms2_on, &all),
                SRC_BOTH_COMPUTED,
                format!(
                    "★ THE OTHER DIRECTION OF THE MAP. iccce --bpc against transicc -b, source \
                     black zero into the fixed A41 black, so everything RISES - the mirror of S2. \
                     Because lcms2 forces here, its -b and no--b arms are identical and this row \
                     would read the same against either. ★ SENSITIVITY: BPC moves K by {:.4e} \
                     device here and the two implementations disagree by {:.4e} - a ratio of \
                     {:.0}x, so the agreement is measured on a comparison that would have seen a \
                     wrong map immediately",
                    Scenario::min_signed(&sc.iccce_on, &sc.iccce_off, &all).abs(),
                    Scenario::max_abs(&sc.iccce_on, &sc.lcms2_on, &all),
                    Scenario::min_signed(&sc.iccce_on, &sc.iccce_off, &all).abs()
                        / Scenario::max_abs(&sc.iccce_on, &sc.lcms2_on, &all)
                ),
            ));
            out.push(Record::graded(
                "pass5/S3/srgb-to-fixture/perceptual/bpc-on-vs-lcms2-unasked",
                Kind::CrossCheck,
                Metric::DeviceAbsMaxNormalised,
                DEVICE_SRGB_TO_FIXTURE,
                Scenario::max_abs(&sc.iccce_on, &sc.lcms2_off, &all),
                SRC_BOTH_COMPUTED,
                "the same against lcms2's UNASKED arm. It exists because a reader should not have \
                 to trust the row above's parenthesis: iccce with --bpc reproduces what lcms2 does \
                 whether or not lcms2 was asked",
            ));
            out.push(Record::graded(
                "pass5/S3/srgb-to-fixture/perceptual/direction-K-never-rises",
                Kind::SelfConsistency,
                Metric::DeviceAbsMaxNormalised,
                DIRECTION,
                Scenario::max_signed(&sc.iccce_on, &sc.iccce_off, &all),
                SRC_BOTH_COMPUTED,
                format!(
                    "★ PASS 5's DONE-WHEN CLAUSE 1, IN THE OPPOSITE PCS DIRECTION. Here Xd (the \
                     A41 triple) > Xs (zero) in every channel, so every PCS value RISES - and this \
                     destination's K falls as L* rises, so the DEVICE test is again 'nothing may \
                     rise'. The coincidence of wording is why the magnitude row below exists: a \
                     sign test that reads the same in both directions cannot, by itself, show the \
                     two directions are different. Largest fall {:.6e} device.
                     ★ THE FIRST DRAFT OF THIS ROW GRADED THE NEGATED MINIMUM AND FAILED AT \
                     3.1372e-2 - it asserted 'nothing may fall' on a scenario whose entire point is \
                     that K falls. The failure was the row, not the engine; the correction is \
                     recorded rather than quietly rewritten",
                    Scenario::min_signed(&sc.iccce_on, &sc.iccce_off, &all).abs()
                ),
            ));
            out.push(Record::graded(
                "pass5/S3/srgb-to-fixture/perceptual/lift-at-black-matches-closed-form",
                Kind::DerivedExpectation,
                Metric::DeviceAbsMaxNormalised,
                LIFT_CLOSED_FORM,
                (s.lift_iccce - s.lift_predicted).abs(),
                "ICC.1:2022 6.3.4.3's map (the second constraint: black_src -> black_dst exactly), \
                 the A41 triple, CIELAB's linear segment, and the fixture's own stored mBA nodes \
                 (row B3's closed form). No implementation's output enters the prediction",
                format!(
                    "★★ THE ONE PLACE PASS 5 HAS AN END-TO-END EXPECTATION RATHER THAN AN ORACLE. \
                     At RGB(0,0,0) sRGB's matrix/TRC gives XYZ(0,0,0) exactly; BPC's second \
                     constraint sends that to the destination black exactly, i.e. to the A41 \
                     triple, whose L* is 903.296296... x 0.0034731 = 3.137238; the mBA closed form \
                     then gives K. Predicted lift {:.9}, iccce observed {:.9}. This is what turns \
                     'BPC changed something' into 'BPC changed it by the amount the sourced map \
                     says', which is the difference between a direction test and a measurement",
                    s.lift_predicted, s.lift_iccce
                ),
            ));
            out.push(Record::graded(
                "pass5/S3/srgb-to-fixture/perceptual/lcms2-black-matches-closed-form",
                Kind::DerivedExpectation,
                Metric::DeviceAbsMaxNormalised,
                DEVICE_SRGB_TO_FIXTURE,
                (s.k_on_lcms2 - s.k_on_predicted).abs(),
                "as the row above - closed form only",
                format!(
                    "★ THE THIRD READING, and it is what stops the fixture and the derivation \
                     being wrong together (the caveat §3.4.4.1 attaches to every derived \
                     expectation). lcms2's own forced-BPC K at device black against the same \
                     closed form: predicted {:.9}, lcms2 printed {:.9}. Its residual is its own \
                     u16 quantisation and transicc's 4-decimal print floor, which is why it is \
                     graded at B5's constant and not at the tighter one above",
                    s.k_on_predicted, s.k_on_lcms2
                ),
            ));
            out.push(Record::graded(
                "pass5/S3/srgb-to-fixture/perceptual/POLICY-iccce-never-forces",
                Kind::CrossCheck,
                Metric::DeviceAbsMaxNormalised,
                UNGRADED,
                s.policy_device,
                SRC_BOTH_COMPUTED,
                format!(
                    "★★ THE POLICY DIFFERENCE, REPORTED AND DELIBERATELY NOT GRADED. iccce WITHOUT \
                     --bpc against lcms2 WITHOUT -b, same pair, same intent: {:.6e} device = {:.4} \
                     L* through the mBA closed form, lcms2 {}. Neither is a defect. lcms2 forces \
                     BPC on for a v4 destination at perceptual on the authority of a document \
                     nobody in this project has read (ICC_Spec §7.1: the claim is in a source \
                     comment attributed to Adobe, and the one published BPC paper, Maria 2013, is \
                     silent about the enable policy while corroborating the exclusion set). iccce \
                     declines to force and requires --bpc. GRADING THIS WOULD MEAN PICKING A \
                     WINNER WITHOUT A CLAUSE. Settled by AdobeBPC.pdf / ICC WP40 / ISO 18619; the \
                     operator download list is ICC_Spec §11",
                    s.policy_device,
                    s.policy_dl,
                    if s.policy_lcms2_is_lighter {
                        "LIGHTER (its K is lower)"
                    } else {
                        "DARKER (its K is higher)"
                    }
                ),
            ));
            out.push(Record::graded(
                "pass5/S3/D11-fingerprint",
                Kind::CrossCheck,
                Metric::AbsMaxComponent,
                UNGRADED,
                s.policy_dl,
                SRC_LCMS2_READ,
                format!(
                    "★ THE D11 WATCH, ANSWERED. The policy difference is {:.4} L* with lcms2 {} - \
                     magnitude within 0.01 L* of the PRM black's 3.1373 (Table 16's 08h), which is \
                     the D11 fingerprint. WHICH CONVENTION IT MATCHES: lcms2's, i.e. the M2 route \
                     (force BPC on for a v4 destination, mapping the source's zero black UP to the \
                     PRM black). It is NOT iccDEV's route, which reaches the same neighbourhood by \
                     applying 6.3.4.3 to the v2 side's transform data at link time and inverting it \
                     on output - the two would be distinguishable in S2, where iccDEV would map the \
                     PRM black DOWN to zero on the v2 output side and lcms2 does nothing unless \
                     asked. Recorded so a future session does not re-diagnose a known sign",
                    s.policy_dl,
                    if s.policy_lcms2_is_lighter {
                        "lighter"
                    } else {
                        "darker"
                    }
                ),
            ));
        }
    }

    // ---- S4 ---------------------------------------------------------------
    match &p.s4 {
        None => {
            out.push(Record::skipped(
                "pass5/S4/srgb-to-v4-matrix/perceptual/forced-bpc-costs-nothing",
                Kind::OracleReproducibility,
                Metric::DeviceAbsMaxNormalised,
                EXACT,
                SRC_BOTH_COMPUTED,
                "sRGB not present (LEGAL.md §3 category (c))",
            ));
        }
        Some(s) => {
            let all = vec![true; s.iccce_off.len()];
            out.push(Record::graded(
                "pass5/S4/srgb-to-v4-matrix/perceptual/forced-bpc-costs-nothing",
                Kind::OracleReproducibility,
                Metric::DeviceAbsMaxNormalised,
                EXACT,
                Scenario::max_abs(&s.lcms2_on, &s.lcms2_off, &all),
                SRC_BOTH_COMPUTED,
                "★ CORPUS TRAP T5, MEASURED. BOTH SIDES ARE lcms2, into a v4 matrix/TRC \
                 destination at perceptual - the configuration M2 says forces BPC. It does force \
                 it, and it costs EXACTLY NOTHING, because cmsDetectBlackPoint's guard 3 takes the \
                 matrix-shaper escape to BlackPointAsDarkerColorant at REL.COL. and returns \
                 XYZ(0,0,0), which equals the source's, so no stage is inserted. Anyone expecting \
                 M2's 3.15 L* on every v4 perceptual profile would mis-attribute this null",
            ));
            out.push(Record::graded(
                "pass5/S4/srgb-to-v4-matrix/perceptual/iccce-agrees-it-is-a-no-op",
                Kind::SelfConsistency,
                Metric::DeviceAbsMaxNormalised,
                EXACT,
                Scenario::max_abs(&s.iccce_on, &s.iccce_off, &all),
                SRC_BOTH_COMPUTED,
                "iccce reaches the same no-op by a different route: its estimation subset sends a \
                 matrix/TRC destination to device black regardless of version or intent, so it \
                 never consults the A41 constant here at all. Same answer, different reason - \
                 worth recording because a shared answer from different reasoning is stronger \
                 evidence than a shared answer from shared reasoning",
            ));
        }
    }

    // ---- S5 / S6: the refusals -------------------------------------------
    match &p.refusal_lut {
        None => out.push(Record::skipped(
            "pass5/S5/srgb-to-swop/media-relative/SUPERSEDED-now-inside-the-subset",
            Kind::SelfConsistency,
            Metric::AbsMaxComponent,
            REFUSAL,
            SRC_BOTH_COMPUTED,
            "sRGB and/or USWebCoatedSWOP not present (LEGAL.md §3 category (c))",
        )),
        Some(r) => out.push(Record::graded(
            "pass5/S5/srgb-to-swop/media-relative/SUPERSEDED-now-inside-the-subset",
            Kind::SelfConsistency,
            Metric::AbsMaxComponent,
            REFUSAL,
            if r.is_ok() { 0.0 } else { 1.0 },
            SRC_BOTH_COMPUTED,
            format!(
                "★ INVERTED 2026-08-12 - THE COVERAGE GAP CLOSED. This row used to assert \
                 that iccce REFUSED here, and its prose ended 'SO NO COMPARISON EXISTS FOR THIS \
                 CASE and Pass 5 claims none'. Both halves have stopped being true. A v2 CMYK \
                 prtr destination at media-relative is where lcms2 runs the Adobe-derived black \
                 point search whose thresholds are unattributed even in lcms2's own source (A42); \
                 iccce refused rather than reproduce constants nobody can cite. It now implements \
                 ISO/CD 18619 4.2.5 instead, Pass 5b found that implementation had no caller, \
                 commit c268261 wired it, and PASS 5C MAKES THE COMPARISON THIS ROW SAID DID NOT \
                 EXIST - the two estimators are 0.0817 dE76 apart, entirely in L*. Outcome: {}",
                match r {
                    Ok(msg) => format!("converts as required now - {msg}"),
                    Err(e) => format!("DID NOT CONVERT - {e}"),
                }
            ),
        )),
    }
    out.push(Record::graded(
        "pass5/S6/absolute-intent/refuses-bpc",
        Kind::SelfConsistency,
        Metric::AbsMaxComponent,
        REFUSAL,
        if p.refusal_absolute.is_ok() { 0.0 } else { 1.0 },
        "Maria (2013) §4.1, VERBATIM: \"absolute colorimetric intent (either the new ICC-absolute \
         or the old V2-absolute) does not apply\" - published_literature, and lcms2's \
         _cmsLinkProfiles plus cmsDetectBlackPoint guard 2 enforce the same exclusion",
        format!(
            "the one exclusion Pass 5 can cite a published source for, and the one refusal row \
             that runs on a machine with no colour directory (both profiles are committed \
             fixtures). BPC presupposes both media whites already at D50, which is what \
             media-relative means and what ICC-absolute undoes; the exclusion and the D50 \
             anchoring are the same fact. Outcome: {}",
            match &p.refusal_absolute {
                Ok(msg) => format!("refused as required - {msg}"),
                Err(e) => format!("DID NOT REFUSE AS REQUIRED - {e}"),
            }
        ),
    ));

    out
}

/// Records for when Pass 5 could not run at all.
#[must_use]
pub fn unavailable_records(u: &Unavailable) -> Vec<Record> {
    let ids = [
        "pass5/map/iccce-vs-icc1-6.3.4.3",
        "pass5/S2/fixture-to-srgb/perceptual/bpc-on-device-vs-lcms2",
        "pass5/S3/srgb-to-fixture/perceptual/bpc-on-device-vs-lcms2",
        "pass5/S3/srgb-to-fixture/perceptual/POLICY-iccce-never-forces",
        "pass5/S6/absolute-intent/refuses-bpc",
    ];
    ids.iter()
        .map(|id| match u {
            Unavailable::Skip(s) => Record::skipped(
                *id,
                Kind::CrossCheck,
                Metric::DeviceAbsMaxNormalised,
                DEVICE_FIXTURE_TO_SRGB,
                SRC_BOTH_COMPUTED,
                s.clone(),
            ),
            Unavailable::Error(e) => Record::errored(
                *id,
                Kind::CrossCheck,
                Metric::DeviceAbsMaxNormalised,
                DEVICE_FIXTURE_TO_SRGB,
                SRC_BOTH_COMPUTED,
                e.clone(),
            ),
        })
        .collect()
}

/// Run Pass 5 and turn it into records, never propagating.
#[must_use]
pub fn run(oracle: &Oracle) -> (Option<Pass5>, Vec<Record>) {
    match analyse(oracle) {
        Ok(p) => {
            let r = records(&p);
            (Some(p), r)
        }
        Err(u) => (None, unavailable_records(&u)),
    }
}
