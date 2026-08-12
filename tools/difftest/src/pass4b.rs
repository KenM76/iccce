//! # Pass 4b — the three directions Pass 4 left unmeasured
//!
//! Read `tools/difftest/README.md` **§15** for the narrative and the findings;
//! this file is the apparatus. It closes the three holes `README.md` §14.9 and
//! `docs/TOLERANCES.md` §3.4.3 record as owed after Pass 4's CMYK→RGB
//! differential:
//!
//! | § | direction | tag type exercised | what it is the first of |
//! |---|---|---|---|
//! | A | sRGB → SWOP, **RGB→CMYK** | `mft1` (`lut8Type`), 3→4, 33³ | the **B2A** direction, and the first `lut8` evaluation compared to anything |
//! | B | the synthetic fixture, **both** directions | `mAB `/`mBA `, ragged 5×4×3×2 and 3³ | the first **v4** LUT of any kind, and the first **derived** (non-oracle) expectation for a LUT transform |
//! | C | `ewgray22.icm` → sRGB, **GRAY→RGB** | none — Annex **F.2** grayTRC | the first monochrome transform |
//!
//! ## The one thing to read before believing any number below
//!
//! **The deviation sources were derived from lcms2's source at pin `21c582a`
//! BEFORE any comparison was run**, because Pass 4 was taught this the hard
//! way: its expected divergence was stated as *"lcms2 interpolates
//! tetrahedrally"*, and reading `cmsintrp.c` showed the four-input scheme is a
//! **hybrid** that is not tetrahedral at all. A tolerance derived from the
//! wrong algorithm is a number with a story attached, not a bound.
//!
//! So, for each of the three sections, what was read first and what it
//! predicted:
//!
//! ### A — the B2A direction: **lcms2 does not use tetrahedral here either**
//!
//! ```c
//! // cmsio1.c, _cmsReadOutputLUT — verbatim, including the comment:
//! // Now it is time for a controversial stuff. I found that for 3D LUTS using
//! // Lab used as indexer space,  trilinear interpolation should be used
//! if (cmsGetPCS(hProfile) == cmsSigLabData)
//!     ChangeInterpolationToTrilinear(Lut);
//! ```
//!
//! `ChangeInterpolationToTrilinear` sets `CMS_LERP_FLAGS_TRILINEAR` on every
//! CLUT stage of the pipeline, which sends `DefaultInterpolatorsFactory`'s
//! `case 3` down `TrilinearInterpFloat`/`TrilinearInterp16` instead of the
//! tetrahedral routines. **Trilinear over three inputs *is* n-linear**, which
//! is exactly what `iccce-cmm`'s `Clut::eval` computes (NA-006).
//!
//! **Predicted in advance, therefore: the interpolation-method envelope that
//! dominated Pass 4 — 1.57 ΔE2000 — is IDENTICALLY ZERO in the B2A direction**,
//! for every Lab-PCS profile, which is every CMYK output profile on this
//! machine. What is left is quantisation, and quantisation is bounded.
//!
//! That prediction is not asserted here: [`B2aAnalysis::counterfactual`] runs
//! the tetrahedral geometry over the same table and reports what the
//! disagreement *would* have been, which is both the sensitivity control (the
//! comparison can see a geometry difference) and the measurement of what
//! lcms2's override is worth.
//!
//! ### A — the other two sources, and one non-source
//!
//! - **8-bit table data is NOT a divergence.** `Type_LUT8_Read` widens every
//!   8-bit sample with `FROM_8_TO_16(v) = (v<<8)|v = v·257`, and `257·255 =
//!   65535`, so lcms2's normalised sample is `v/255` — bit-identical to
//!   `iccce-cmm`'s `f64::from(v)/255.0`. The 1/255 granularity of the *table*
//!   is shared by both implementations and cancels in the difference. What it
//!   does do is make the pipeline *sensitive*: one lsb of table data is
//!   3.9×10⁻³ of the device range, so a small input difference can be
//!   amplified by the local slope of a coarse table.
//! - **lcms2 quantises three times inside this pipeline**, and all three are
//!   modelled in [`B2aPipeline::eval`]: the 256-entry input curves are
//!   `cmsBuildTabulatedToneCurve16` (`nSegments == 0`), so
//!   `cmsEvalToneCurveFloat` rounds their input *and* output to 1/65535 (the
//!   Pass 3 finding, §13.6.1); `EvaluateCLUTfloatIn16` rounds the CLUT stage
//!   input to `u16` and returns `u16/65535`; and the output curves are
//!   tabulated too, so they round twice more.
//! - **The Lab encoding is NOT a divergence, and it is worth saying why**,
//!   because it is the one place a plausible bug would be invisible.
//!   `_cmsReadOutputLUT` inserts `_cmsStageAllocLabV4ToV2` **only when
//!   `OriginalType == cmsSigLut16Type`** — for a `lut8Type` tag it does not,
//!   so the pipeline keeps lcms2's internal v4-normalised Lab (`L*/100`,
//!   `(ab+128)/255`, from `cmspack.c`'s `UnrollLabDoubleToFloat`). iccce's
//!   `PcsCodec::Lab8` encodes `L/100`, `(ab+128)/255` (Tables 12/13's 8-bit
//!   column, corpus A10). **The two agree exactly** — the legacy 652,8 scale
//!   belongs to `lut16Type` and neither implementation applies it here. Had
//!   iccce applied it, `L*` would be 0,39 % low and `a*`/`b*` 0,39 % off, which
//!   is ≈0,2 ΔE2000 — *below* the perceptibility anchor and invisible to a
//!   suite graded at it.
//!
//! ### B — the fixture: an **affine** CLUT, which is what makes an expectation
//!
//! `fixtures/synthetic/v4-cmyk-mab-lab.icc` was authored by
//! `tools/gen-profiles` to test the `mAB `/`mBA ` *byte layout*. Reading its
//! recipe shows both CLUTs store functions that are **affine in one input and
//! constant in the others**:
//!
//! - `A2B0` (`mAB `, 4→3, grid 5×4×3×2): `L* = 100·(1 − K)`, `a* = b* = 0`,
//!   independent of C, M and Y.
//! - `B2A0` (`mBA `, 3→4, grid 3×3×3): `K = 1 − L*/100`, `C = M = Y = 0`,
//!   independent of `a*` and `b*`.
//!
//! **Every interpolation scheme reproduces an affine function exactly** —
//! n-linear, trilinear, Sakamoto tetrahedral, lcms2's 4-D hybrid — so the
//! method difference that dominates Pass 4 is *provably* zero here, and the
//! output is a closed form in the input. [`expected_mab_lab`] and
//! [`expected_mba_cmyk`] state it, derived from ICC.1:2022 10.12/10.13's
//! element order and Tables 45/47's encoding plus the fixture's own stored
//! bytes — **no implementation's output enters either**. That is
//! [`Kind::DerivedExpectation`], the first non-cross-check expectation any LUT
//! row in this repository has had.
//!
//! Two details of the derivation that a reader should check rather than trust:
//!
//! 1. **The 3×4 matrix's offsets are `1/256`, `2/256`, `3/256`** and they are
//!    applied in the *normalised* domain, so they shift `L*` by `+0,390625`,
//!    `a*` by `+1,9921875` and `b*` by `+2,98828125`. They are in the fixture
//!    precisely because dropping them is a classic misread; here they also do
//!    something the generator did not intend, which is §B's finding.
//! 2. **The stored `K` nodes are rounded to `u16`**, so the mBA function is
//!    piecewise-linear through `(0, 1)`, `(½, 32768/65535)`, `(1, 0)` rather
//!    than exactly `1 − L`. The expectation uses the stored nodes, not the
//!    idealised line; an expectation that ignored the rounding would be wrong
//!    by 7,6×10⁻⁶ and would look like an implementation defect.
//!
//! ### C — gray: lcms2 and iccce build the **same** model, from the same D50
//!
//! ```c
//! // cmsio1.c: the gray input pipeline is TRC then a 1x3 matrix of D50.
//! static const cmsFloat64Number GrayInputMatrix[] =
//!     { (InpAdj*cmsD50X), (InpAdj*cmsD50Y), (InpAdj*cmsD50Z) };
//! ```
//!
//! `cmsD50X/Y/Z` are `0.9642 / 1.0 / 0.8249` (`lcms2.h`) and
//! `iccce_color::D50` is `0.9642 / 1.0000 / 0.8249` — **the same three
//! literals**, so the F.2 white multiplication cannot diverge. (`InpAdj` is
//! lcms2's internal 1/1,99997 XYZ encoding scale; it is undone by the matching
//! `OutpAdj` on the destination side and does not survive into the answer.)
//! Two consequences worth stating in advance:
//!
//! - **`wtpt` is not read by either side at the media-relative intent**, so
//!   Pass 4's §14.6 divergence — lcms2 substituting D50 for a v2 display
//!   profile's `wtpt` — **cannot fire here even though `ewgray22.icm` is
//!   exactly such a profile** (v2.2, `mntr`, `wtpt` = D65). It would fire at
//!   ICC-absolute; that intent is deliberately out of scope for §C and stays
//!   attributed to §14.6.
//! - **The residual is therefore almost entirely the destination**, and that
//!   makes §C the cleanest measurement available of lcms2's sRGB *output*
//!   model: `BuildRGBOutputMatrixShaper` inverts each 1024-entry `curv` with
//!   `cmsReverseToneCurve`, i.e. `cmsReverseToneCurveEx(4096, ...)` — a
//!   **4096-entry `u16` resampling** of the inverse, itself then evaluated
//!   through the 16-bit float path. iccce inverts the stored table directly.
//!   [`ReverseCurve`] reimplements lcms2's, which turns §C from an observation
//!   into an attribution and answers README §14.9 item 4.
//!
//! ## What this module refuses to do
//!
//! - **It does not put a tetrahedral evaluator in `crates/`.** Both geometries
//!   live here, in the harness, as models of the two implementations.
//! - **It does not grade a mean.** Means are recorded next to their maxima and
//!   marked REPORTED, NOT GRADED, for the reason `pass3`/`pass4` state.
//! - **It does not decide the §B clamp question.** iccce and lcms2 disagree
//!   about what happens when the `mAB ` matrix pushes an encoded PCS value
//!   above 1,0; that is a specification question, it is recorded as a finding
//!   with the exact text to put to `icc-spec-librarian`, and the affected rows
//!   are REPORTED, NOT GRADED until it is settled — the same posture §14.6
//!   takes with A4b.

use std::path::{Path, PathBuf};

use iccce_cmm::gray_trc::GrayTrc;
use iccce_cmm::lut_ab::LutAbModel;
use iccce_cmm::lut_transform::{Lut16Model, PcsKind, PcsValue};
use iccce_cmm::matrix_trc::MatrixTrc;
use iccce_color::{D50, Lab, Xyz, delta_e_2000};
use iccce_profile::Profile;
use iccce_profile::lut::{ClutSamples, Lut8, LutAB};
use iccce_profile::num::Signature;
use iccce_profile::tag_types::TagData;

use crate::pass4::{cell, cell_lcms2, fclamp, interp_table};
use crate::{
    Bpc, DiffError, Iccce, Intent, Kind, Metric, Oracle, Precalc, Record, Request, Space, Tolerance,
};

// ===========================================================================
// The corpus
// ===========================================================================

/// The Windows system sRGB profile — **category (c)** (`LEGAL.md` §3: read
/// locally, never committed, never a required input). The source in §A, the
/// destination in §C, and the destination of the fixture's `mAB ` leg in §B.
pub const SRGB: &str = r"C:\Windows\System32\spool\drivers\color\sRGB Color Space Profile.icm";

/// `U.S. Web Coated (SWOP) v2` — category (c). Pass 4 used its `A2B*`; §A uses
/// its **`B2A0`/`B2A1`**, which are `mft1` (`lut8Type`), 3 in / 4 out, 33
/// points per axis, 256-entry 8-bit input and output tables.
pub const SWOP: &str = r"C:\Windows\System32\spool\drivers\color\USWebCoatedSWOP.icc";

/// `EPSON  Gray - Gamma 2.2` — category (c). v2.2.0, `mntr`, `GRAY` → `XYZ `,
/// one `kTRC` (`curv` with a single u8Fixed8 gamma = 2,19921875), `wtpt` = D65.
///
/// **A single-value `curv` is analytic on both sides** — lcms2 turns it into a
/// type-1 parametric curve (`Type_Curve_Read`, `Count == 1`), so `nSegments !=
/// 0` and the 16-bit tone-curve quantisation of §13.6.1 does **not** apply to
/// the source. That is why §C isolates the destination.
pub const GRAY: &str = r"C:\Windows\System32\spool\drivers\color\ewgray22.icm";

/// The synthetic v4 fixture, **category (a)** — committed, reproducible byte
/// for byte by `tools/gen-profiles`, and therefore the only part of Pass 4b
/// that does not skip on a machine without the Windows colour directory.
///
/// Path is resolved against `CARGO_MANIFEST_DIR` so it does not depend on the
/// working directory a `cargo run` was launched from.
#[must_use]
pub fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/synthetic/v4-cmyk-mab-lab.icc")
}

mod tag {
    use iccce_profile::num::Signature;
    pub const A2B0: Signature = Signature(0x4132_4230);
    pub const B2A0: Signature = Signature(0x4232_4130);
    pub const B2A1: Signature = Signature(0x4232_4131);
    /// Added 2026-08-12 for the saturation table. `'B2A2'` = `0x42324132`.
    pub const B2A2: Signature = Signature(0x4232_4132);
    pub const A2B1: Signature = Signature(0x4132_4231);
}

// ===========================================================================
// Tolerances — each is the envelope its `why` describes, plus headroom
// ===========================================================================

/// **§A, device space.** iccce's CMYK against lcms2's, worst component over the
/// grid, normalised 0..1.
///
/// The envelope is **computed, not bounded in closed form**, for the reason
/// Pass 4 records: a union bound over this pipeline is useless (the XYZ→Lab
/// sensitivity `da*/dX ≤ 4038` multiplied by a coarse table's local slope
/// exceeds the device range). Instead [`B2aPipeline`] models **every one of
/// lcms2's arithmetic departures** — tabulated input curves rounded to 1/65535
/// in and out, the CLUT stage input rounded to `u16`, the CLUT output returned
/// as `u16/65535`, the output curves rounded twice more, and the source's
/// 1024-entry `curv` TRCs rounded the same way — and the envelope is the
/// largest difference that model makes over the actual grid. **No lcms2 output
/// enters it.**
///
/// Computed envelope 2026-08-11: **1,330×10⁻⁴** device units at media-relative
/// (`B2A1`) and **9,602×10⁻⁵** at perceptual (`B2A0`). `5×10⁻⁴` is the larger
/// with ~276 % headroom, covering the two arithmetic departures deliberately
/// *not* modelled — lcms2 interpolates its 256-entry curves and its trilinear
/// CLUT in **16-bit fixed point** where the model uses `f64`, each worth up to
/// one further lsb — and the fact that the envelope, being a property of *where
/// the table is steep*, moves with the grid.
///
/// The observed residual lands **within 0,02 % of the envelope** (1,330×10⁻⁴
/// against 1,330 241×10⁻⁴): the disagreement is not merely small, it is
/// *accounted for*.
///
/// ## ★ Re-derived 2026-08-12 when the saturation table was added — the
/// NUMBER did not move, the STATED ENVELOPE did
///
/// `B2A2` is a steeper table than either of the other two, so its computed
/// envelope is **1,552 5×10⁻⁴** — larger than both figures the `why` string
/// originally named (1,330×10⁻⁴ media-relative, 9,602×10⁻⁵ perceptual). The
/// constant stays at `5×10⁻⁴`, now ~3,2× the worst of the three rather than
/// ~3,8× the worst of two, and **the `why` string was corrected to name the new
/// maximum**. This is the case §4's log calls a corrected justification rather
/// than a widened tolerance, and the direction of travel is the diagnostic one:
/// the *justification* moved toward the observation while the *number* stayed
/// put. Had it been the other way round it would have been tuning.
///
/// The observed saturation residual is **1,550 0×10⁻⁴** against that
/// 1,552 5×10⁻⁴ envelope — **99,8 % accounted for**, the same signature as the
/// other two intents.
///
/// **GRID-DEPENDENT BY CONSTRUCTION** (the envelope is largest where the B2A
/// table is steepest, which is the near-neutral shadow) and
/// **arithmetic-agreement, not perceptual**: §2's 1,0 ΔE2000 anchor is
/// irrelevant to it.
pub const DEVICE_B2A: Tolerance = Tolerance::new(
    5e-4,
    "the quantisation envelope computed from lcms2's OWN arithmetic over this pipeline \
     (tabulated 256-entry curves rounded to 1/65535 in and out, CLUT input rounded to u16, \
     CLUT output returned as u16/65535, source 1024-entry curv TRCs likewise) propagated \
     through the actual B2A table: 1.5525e-4 device units at SATURATION (B2A2, the steepest of \
     the three tables, measured 2026-08-12), 1.330e-4 at media-relative and 9.602e-5 at \
     perceptual, plus ~222% headroom over the worst of the three for lcms2's \
     16-bit FIXED-POINT curve and CLUT interpolation, which the f64 model does not reproduce. \
     No lcms2 output enters the envelope. GRID-DEPENDENT: a grid reaching further into the \
     shadow re-derives it. Arithmetic agreement, NOT perceptual",
);

/// **§A, perceptual weight of the same disagreement.** Both sides' CMYK carried
/// back through SWOP's own `A2B1` into D50 CIELAB, then ΔE2000.
///
/// This exists because a CMYK device-space number is not a colour statement: 4
/// ink components have no perceptual metric, and 1,3×10⁻⁴ of ink could be
/// anything until it is carried into a space where a ΔE means something. The
/// route back is `A2B1` — **the same file's own colorimetric table**, which is
/// what "what would this ink difference look like" means for this profile, and
/// which is why the record says *round trip* rather than pretending to a second
/// opinion.
///
/// The bound follows from the device envelope and the table's slope: `A2B1`'s
/// steepest node-to-node step is ≈0,1 in normalised `L*` per 1/8 of device, so
/// `dL*/d(device) ≲ 80`; 1,330×10⁻⁴ × 80 = 1,06×10⁻² `L*` — but the four inks
/// move together and their `L*` contributions add rather than cancel, so take
/// ~2× that and ÷ `S_L ≈ 1,2` in the midtones ≈ **1,8×10⁻² ΔE00**. `5×10⁻²` is
/// ~2,8× that. **20× below §2's ⚠ provisional 1,0 anchor**, whose ⚠ it inherits.
/// Observed 7,1×10⁻³ (perceptual) and 5,7×10⁻³ (media-relative).
pub const DE_B2A_ROUNDTRIP: Tolerance = Tolerance::new(
    5e-2,
    "the device envelope (1.330e-4) carried back through SWOP's own A2B1: its steepest \
     node-to-node step is about 0.1 normalised L* per 1/8 device, so dL*/d(device) is about 80, \
     giving 1.06e-2 L* per ink; the four inks move together and add rather than cancel, so about \
     2x that, and about 1.8e-2 dE00 after S_L. 5e-2 is ~2.8x that and 20x below the provisional \
     1.0 perceptibility anchor, whose provisional mark it inherits",
);

/// **§A, the attribution.** iccce's *modelled* prediction of lcms2 — the same
/// pipeline with every one of lcms2's roundings switched on — against lcms2's
/// actual printed CMYK.
///
/// If the model is right this collapses to the oracle's print floor. `transicc`
/// prints CMYK to four decimals in 0..100, so one printed lsb is **10⁻⁶** in
/// normalised device units; the two unmodelled fixed-point interpolations are
/// worth up to ~1,5×10⁻⁵ each. `5×10⁻⁵` is that sum with headroom, and it is
/// **10× tighter than [`DEVICE_B2A`]** — which is what makes the attribution a
/// claim rather than a restatement.
///
/// ★ **Measured 3,100×10⁻⁵, which is 2,03 lsb of 1/65535.** The residual is not
/// merely under the bound: it is *exactly the two roundings the model leaves
/// out*, to three significant figures, at both intents and on the PCS-side
/// comparison as well. An attribution rarely gets to be that specific, and it
/// is worth stating what it says — what remains after modelling lcms2's
/// arithmetic is lcms2's **fixed-point** arithmetic, and nothing else.
pub const DEVICE_B2A_MODELLED: Tolerance = Tolerance::new(
    5e-5,
    "what must remain once EVERY lcms2 rounding in this pipeline is modelled: transicc's \
     4-decimal CMYK print floor (1e-6 normalised) plus the two roundings the f64 model does \
     NOT reproduce (lcms2 interpolates its 256-entry curves and its trilinear CLUT in 16-bit \
     fixed point, up to ~1.5e-5 each). 20x tighter than DEVICE_B2A, so it is a claim about \
     the mechanism and not a restatement of the observation",
);

/// **§A, apparatus.** The harness's own `lut8` reimplementation against
/// `iccce-cmm`'s `Lut16Model::pcs_to_device`, exact arithmetic, every point.
///
/// The precondition for believing §A's experiments: they need one pipeline
/// evaluated several ways, and that substitution cannot be made inside
/// `crates/`. 10⁻⁹ is ~7 orders above `f64` noise on this arithmetic and ~5
/// below anything colorimetric — it can neither pass a real divergence nor fail
/// on rounding.
/// **§A, the precondition for the saturation records (added 2026-08-12).**
/// How many of the three `B2A*` tag-data blocks are **byte-identical**.
///
/// `0,0 — exact`, and this is one of the few places 0,0 is honestly available:
/// the quantity is a **count of integer comparisons on file bytes**, not a
/// floating-point residual, so there is no rounding for a tolerance to absorb.
/// §3.4.4 row B0's rule ("0,0 only when the two sides are the same operations
/// in the same order") is about arithmetic; this is not arithmetic.
///
/// **What it catches, and it is not hypothetical.** In the *A2B* direction of
/// this very file `A2B0` and `A2B2` are one block at one offset, and Pass 4
/// graded their equality at exactly zero for that reason. Had `B2A0`/`B2A2`
/// been laid out the same way, §A's saturation run would have reproduced the
/// perceptual run bit for bit and the suite would have gained three green lines
/// that measured nothing — the precise failure mode `CLAUDE.md` rule 5 and
/// `TOLERANCES.md` §6 exist to prevent. **A null that is null by construction
/// must be identified before it is collected, not explained afterwards.**
pub const TAGS_ARE_DISTINCT: Tolerance = Tolerance::new(
    0.0,
    "a COUNT of byte-identical tag-data blocks among B2A0/B2A1/B2A2, read from the file with \
     no parser in the way; not a floating-point residual, so 0.0 needs no rounding allowance. \
     Any non-zero value means one of section A's three intent runs is a restatement of another \
     rather than a third measurement - which is exactly how A2B0/A2B2 behave in this same file",
);

pub const APPARATUS_B2A: Tolerance = Tolerance::new(
    1e-9,
    "apparatus self-check: the harness's lut8 pipeline must be the crate's, to f64 noise. \
     1e-9 is ~7 orders above the rounding of this arithmetic and ~5 below anything \
     colorimetric, so it cannot pass a real divergence nor fail on noise",
);

/// **§B, the derived expectation, iccce side.** iccce's `mAB `/`mBA `
/// evaluation against the closed form of [`expected_mab_lab`] /
/// [`expected_mba_cmyk`].
///
/// iccce evaluates the whole chain in `f64` with no intermediate quantisation,
/// and the expectation is the same arithmetic written independently from the
/// clause text, so the residual is **`f64` rounding on a handful of
/// operations** — a few ulp. In `L*` units an ulp near 100 is 1,4×10⁻¹⁴, so
/// `1×10⁻¹²` is ~70 of them; in device units it is ~4500. Either way it is
/// seven orders below one `u16` lsb (1,5×10⁻⁵), so a single-lsb error in a
/// stored node still fails the row — which is what a derived-expectation row is
/// for. Observed 2,8×10⁻¹⁴ (`mAB `, `L*` units) and 2,2×10⁻¹⁶ (`mBA `, device).
pub const DERIVED_EXACT: Tolerance = Tolerance::new(
    1e-12,
    "the expectation is the same f64 arithmetic derived independently from ICC.1:2022 \
     10.12/10.13 and Tables 45/47, and iccce evaluates the chain in f64 with no intermediate \
     quantisation, so the residual is a few ulp. 1e-12 is far above that and 7 orders below \
     one u16 lsb (1.5e-5), so a single-lsb error in a stored node still fails",
);

/// **§B, the derived expectation, lcms2 side.** The same closed form against
/// `transicc`'s output — the *third* reading, which is what stops the fixture
/// and the derivation being wrong together.
///
/// lcms2's residual against an exact closed form is its own quantisation, and
/// it is one-sided in a way the iccce row is not: `EvaluateCLUTfloatIn16`
/// rounds the CLUT input to `u16` (½ lsb = 7,6×10⁻⁶ of the axis) and returns
/// the interpolated value as `u16/65535` (1 lsb = 1,5×10⁻⁵), and the fixed-point
/// interpolation adds up to another. In `L*` units (`L* = 100·n`) that is
/// **≈3×10⁻³**; measured 2,3×10⁻³ at `K = ½`. `1×10⁻²` `L*` is that with ~3×
/// headroom — **still 40× below the 0,39 `L*` matrix-offset the row exists to
/// confirm is applied**, and 100× below the legacy/general Lab confusion
/// (0,39 %) it would also catch.
pub const DERIVED_LCMS2: Tolerance = Tolerance::new(
    1e-2,
    "lcms2's own quantisation against an exact closed form: CLUT input rounded to u16 \
     (0.5 lsb = 7.6e-6 of the axis), CLUT output returned as u16/65535 (1 lsb = 1.5e-5), plus \
     fixed-point interpolation, which in L* units (L*=100n) is about 3e-3; 1e-2 is ~3x that. \
     Still 40x below the 0.390625 L* matrix offset this row confirms is applied, and 100x \
     below the 0.39% legacy-vs-general PCSLAB confusion it would also catch",
);

/// **§B, the cross-check.** iccce's shipped binary against `transicc`, through
/// the fixture in both directions, in device units.
///
/// The interpolation-method envelope is **zero by construction** (both CLUTs
/// are affine — see the module header, and it is *measured* as 0,0 by
/// [`MabAnalysis::scheme_envelope`]), so what is left is lcms2's `u16`
/// quantisation of the CLUT boundary carried into device units: 1 lsb =
/// 1,5×10⁻⁵, and the mBA's `K` axis has slope 1 in device per unit of
/// normalised `L*`, so the term does not amplify. `1×10⁻⁴` is ~6× that,
/// covering the fixed-point interpolation and `transicc`'s print floor.
///
/// **This row does not cover the K = 0 / `L*` > 100 points** — see
/// [`MabAnalysis::clamp_divergence`] and §15.3's finding; those are excluded
/// from it by construction and reported separately, because grading them would
/// mean grading an unsettled specification question.
/// **§B, the derived expectation, lcms2 side, in DEVICE units.**
///
/// [`DERIVED_LCMS2`] is stated in `L*` units and cannot be applied to a CMYK row
/// without a conversion nobody performed — the two would look commensurable and
/// would not be. Same mechanism, same lsb, expressed where it is measured: 1 lsb
/// of CLUT output is 1,5×10⁻⁵ of the device range, the mBA table's `K` axis has
/// unit slope so nothing amplifies it, and the input rounding adds another half.
/// `1×10⁻⁴` is ~4× that.
pub const DERIVED_LCMS2_DEVICE: Tolerance = Tolerance::new(
    1e-4,
    "the same lcms2 quantisation as DERIVED_LCMS2 but expressed in the units this row is \
     measured in: 1 lsb of CLUT output is 1.5e-5 of the device range and the mBA table's K axis \
     has unit slope, so nothing amplifies it; 1e-4 is ~4x that. Stated separately rather than \
     converting an L* bound into device units, which nobody would be able to check",
);

/// **§B, the mAB direction end to end, into the sRGB destination.**
///
/// Distinct from [`DEVICE_MAB_CROSSCHECK`] because **the destination is not the
/// same kind of thing**: converting *into* the fixture ends at a CLUT, while
/// converting *out of* it ends at sRGB's inverse tone curves — and lcms2's
/// inverse tone curves are a 4096-entry `u16` resampling whose envelope §C
/// measures independently at **9,68×10⁻⁵ device**. That term dominates and has
/// nothing whatever to do with `mAB `.
///
/// Envelope: §C's 9,68×10⁻⁵ (same destination, and this fixture's colours are
/// near-neutral, so it is the same part of it) plus the fixture CLUT's own
/// 1,5×10⁻⁵ carried through ≈ 1,15×10⁻⁴. `2,5×10⁻⁴` is ~2,2× that.
///
/// **The first draft of this row shared [`DEVICE_MAB_CROSSCHECK`]'s 1×10⁻⁴ and
/// failed at 1,012×10⁻⁴.** What was wrong was a *missing term in a derivation*
/// — the destination — not a number that needed room, and `TOLERANCES.md` §4
/// logs it as such.
pub const DEVICE_MAB_TO_SRGB: Tolerance = Tolerance::new(
    2.5e-4,
    "converting OUT of the fixture ends at sRGB's inverse tone curves, which lcms2 builds as a \
     4096-entry u16 resampling (cmsReverseToneCurveEx) - the term section C measures \
     independently at 9.68e-5 device on the same destination - plus the fixture CLUT's own \
     1.5e-5 carried through: about 1.15e-4, and 2.5e-4 is ~2.2x that. NOT the same tolerance as \
     the mBA direction, which ends at a CLUT and has no such term",
);

pub const DEVICE_MAB_CROSSCHECK: Tolerance = Tolerance::new(
    1e-4,
    "both fixture CLUTs are affine, so the interpolation-method envelope is identically zero \
     (measured 0.0), leaving lcms2's u16 quantisation of the CLUT boundary: 1 lsb = 1.5e-5, \
     unamplified because these tables have unit slope in device per normalised PCS unit. \
     1e-4 is ~6x that, covering fixed-point interpolation and transicc's print floor. \
     EXCLUDES the encoded-PCS-overflow points, which are an unsettled spec question and are \
     reported separately rather than absorbed",
);

/// **§C, device space.** iccce's sRGB against lcms2's over the gray axis.
///
/// Predicted in advance from the *destination*, because the source cannot
/// contribute: both implementations multiply the same analytic
/// `g^2,19921875` by the same D50 literals. lcms2's destination inverse TRC is
/// `cmsReverseToneCurveEx(4096, ·)` — a 4096-entry `u16` resampling of the
/// inverse of a 1024-entry `u16` table — evaluated through the float path,
/// which rounds input and output to 1/65535 (§13.6.1). Three terms:
/// resampling (the reverse table's knots do not coincide with the forward
/// table's, so a knot falling inside a reverse cell is chorded), `u16` storage
/// of the reverse table (½ lsb = 7,6×10⁻⁶ device), and the float evaluator's
/// two roundings.
///
/// Computed envelope from [`ReverseCurve`] 2026-08-11: **9,680×10⁻⁵** device.
/// `2,5×10⁻⁴` is that with ~158 % headroom. **Arithmetic-agreement, not
/// perceptual.**
///
/// The observed residual is **9,686×10⁻⁵ — 0,06 % ABOVE the envelope**, and that
/// is expected rather than alarming: the envelope is computed between two `f64`
/// pipelines, while the observation additionally carries `transicc`'s 4-decimal
/// print of 0..255 (3,9×10⁻⁷) and `iccce transform`'s 6-decimal print
/// (5×10⁻⁷). A residual far *below* the envelope would be the surprising
/// outcome, because it would mean the model of lcms2 was pessimistic about
/// lcms2.
pub const DEVICE_GRAY: Tolerance = Tolerance::new(
    2.5e-4,
    "the source cannot contribute (both sides evaluate the same analytic gamma and multiply \
     by the same D50 literals, 0.9642/1.0/0.8249), so this is the destination alone: lcms2 \
     inverts each 1024-entry curv with cmsReverseToneCurveEx(4096), a u16 resampling whose \
     knots do not coincide with the forward table's, then evaluates it through the float path \
     that rounds input and output to 1/65535. Envelope computed from that model over this \
     axis: 9.680e-5 device; 2.5e-4 is ~2.6x it. Arithmetic agreement, NOT perceptual",
);

/// **§C, the same disagreement in ΔE2000**, through the destination's own
/// model.
///
/// ★ **The maximum is near BLACK, and the reason inverts a note from Pass 3.**
/// §13.6's observation was *"near black the device metric explodes while ΔE
/// stays small"* — that is about the *inverse* TRC's unbounded slope acting on a
/// device comparison. Here the comparison is already in device units and the
/// amplification runs the other way: below sRGB's linear breakpoint a device
/// difference `δ` becomes `δ/12,92` of linear light, and CIELAB's **chromatic**
/// sensitivity on *its* linear segment is `da*/dX = 500·7,787/X_n = 4038`. The
/// three channels carry slightly different `δ` (they have independent reverse
/// tables), so the worst point is a very dark one and its ΔE is dominated by
/// `a*`/`b*`, not `L*`:
///
/// - `ΔL* ≈ 903,3 × δ/12,92 = 69,9 δ` → 6,8×10⁻³ at `δ = 9,68×10⁻⁵`
/// - `Δa* ≈ 4038 × (δ/12,92) × X_R = 136 δ` → 1,3×10⁻² (`X_R = 0,4361`)
///
/// and ΔE00 near neutral has `S_C ≈ 1` while `S_L ≈ 1,75`, so the chromatic term
/// is the larger by ~3×. Union ≈ **2×10⁻² ΔE00**. `5×10⁻²` is ~2,4× that and
/// **20× below §2's ⚠ provisional 1,0 anchor**, whose ⚠ it inherits.
///
/// **The first draft said 1×10⁻² from a derivation taken at white, and failed at
/// 2,17×10⁻².** It was looking at the wrong end of the axis; `TOLERANCES.md` §4
/// logs the correction, and the mechanism is independently confirmed by
/// [`DEVICE_GRAY_MODELLED`] collapsing the same disagreement 457×.
pub const DE_GRAY: Tolerance = Tolerance::new(
    5e-2,
    "the 9.68e-5 device envelope carried into CIELAB at the DARK end, where it is largest: below \
     sRGB's linear breakpoint a device difference d becomes d/12.92 of linear light, and CIELAB's \
     chromatic sensitivity on its own linear segment is da*/dX = 500*7.787/Xn = 4038, so \
     da* = 136 d = 1.3e-2 against dL* = 69.9 d = 6.8e-3; with S_C about 1 and S_L about 1.75 the \
     chromatic term dominates and the union is about 2e-2 dE00. 5e-2 is ~2.4x that and 20x below \
     the provisional 1.0 perceptibility anchor, whose provisional mark it inherits",
);

/// **§C, the attribution.** iccce's model of lcms2's destination — the 4096-entry
/// reverse curve with both roundings — against lcms2's actual output.
///
/// Collapses to `transicc`'s print floor if the mechanism is right: RGB is
/// printed to four decimals in 0..255, so one printed lsb is 3,9×10⁻⁷
/// normalised. `5×10⁻⁶` allows ~13 of those for the parts not modelled (lcms2
/// evaluates the reverse table's interpolation in 16-bit fixed point, and its
/// matrix in `float32`). **20× tighter than [`DEVICE_GRAY`]**, which is what
/// makes it an attribution.
pub const DEVICE_GRAY_MODELLED: Tolerance = Tolerance::new(
    5e-6,
    "what must remain once lcms2's 4096-entry reverse tone curve and its two 1/65535 \
     roundings are modelled: transicc's 4-decimal RGB print floor is 3.9e-7 normalised, and \
     the unmodelled parts (16-bit fixed-point table interpolation, float32 matrix) are worth a \
     few more. 5e-6 is ~13 print lsb, and 20x tighter than DEVICE_GRAY",
);

/// **§C, structural.** Perceptual and media-relative must be the *same
/// transform*, on both sides.
///
/// A monochrome profile has no `A2B*`/`B2A*` tags at all, so clause 8.10.2's
/// intent-indexed tag selection has nothing to select: every intent falls
/// through to step 4's grayTRC model (F.2). The destination is matrix/TRC, with
/// the same property. **There is no arithmetic anywhere in either chain that
/// could produce a small difference between the two intents** — a difference is
/// an intent-dispatch defect, and `0,0` with `<=` is the only honest bound.
/// (ICC-absolute is excluded and *does* differ: it reads `wtpt`, which is where
/// §14.6's finding lives.)
pub const GRAY_INTENT_IDENTITY: Tolerance = Tolerance::new(
    0.0,
    "a monochrome profile carries no A2Bx/B2Ax, so 8.10.2's intent-indexed selection has \
     nothing to select and perceptual and media-relative both fall through to step 4's F.2 \
     grayTRC model; the destination is matrix/TRC with the same property. No arithmetic in \
     either chain could make the difference small, so any difference is an intent-dispatch \
     defect and exact equality is the only honest bound. ICC-absolute is excluded: it reads \
     wtpt, which is Pass 4 section 14.6's finding",
);

/// The tolerance used for rows that are **reported and not graded**.
pub const REPORTED: Tolerance = Tolerance::new(
    f64::INFINITY,
    "REPORTED, NOT GRADED - recorded so the number is on file next to the ones that are graded",
);

// ===========================================================================
// The grids
// ===========================================================================

/// §A's source grid: **deterministic RGB triples in `[0,1]`**.
///
/// | block | count | why |
/// |---|---|---|
/// | cube corners | 8 | black, white, the three primaries and the three secondaries — the extremes of the source gamut, every one far outside SWOP's, so the B2A table's own gamut mapping is exercised |
/// | neutral axis | 17 | `r = g = b = i/16`. The axis a CMYK separation treats specially (GCR/UCR); a transposed ink or a wrong `K` shows here first |
/// | primary ramps | 27 | `(t,0,0)`, `(0,t,0)`, `(0,0,t)` at 9 steps — one channel at a time |
/// | 3-D lattice | 125 | `{0, ¼, ½, ¾, 1}³`, systematic interior coverage |
/// | pseudo-random | 64 | fixed-seed LCG (MMIX constants) into `[0.02, 0.98]³` — a systematic grid can sit on structure; these deliberately do not |
///
/// De-duplicated by exact bit pattern. **Deterministic by construction** — no
/// `rand`, no clock, no hash seed.
///
/// **What it does not cover:** nothing between 0 and 1/16 except through the
/// random block, which matters because that is where the source EOTF's inverse
/// slope and the XYZ→Lab sensitivity are both largest; and no out-of-`[0,1]`
/// device value, which the shipped CLI does not accept.
#[must_use]
pub fn rgb_grid() -> Vec<[f64; 3]> {
    let mut out: Vec<[f64; 3]> = Vec::new();
    let push = |t: [f64; 3], out: &mut Vec<[f64; 3]>| {
        let key = |v: f64| v.to_bits();
        if !out.iter().any(|e| (0..3).all(|i| key(e[i]) == key(t[i]))) {
            out.push(t);
        }
    };
    for r in [0.0, 1.0] {
        for g in [0.0, 1.0] {
            for b in [0.0, 1.0] {
                push([r, g, b], &mut out);
            }
        }
    }
    for i in 0..=16 {
        let v = f64::from(i) / 16.0;
        push([v, v, v], &mut out);
    }
    for i in 0..=8 {
        let v = f64::from(i) / 8.0;
        push([v, 0.0, 0.0], &mut out);
        push([0.0, v, 0.0], &mut out);
        push([0.0, 0.0, v], &mut out);
    }
    let axis = [0.0, 0.25, 0.5, 0.75, 1.0];
    for &r in &axis {
        for &g in &axis {
            for &b in &axis {
                push([r, g, b], &mut out);
            }
        }
    }
    let mut x: u64 = 0x1CCC_E000_0004_00B2;
    let mut next = || -> f64 {
        x = x
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        #[allow(clippy::cast_precision_loss)] // exactly 53 bits: lossless
        let u = (x >> 11) as f64 / ((1u64 << 53) as f64);
        0.02 + u * 0.96
    };
    for _ in 0..64 {
        let t = [next(), next(), next()];
        push(t, &mut out);
    }
    out
}

/// §A's and §B's PCS-side grid: **deterministic D50 CIELAB values**.
///
/// Built in the *encoded* domain rather than by eye, so the blocks mean
/// something to the tables being indexed:
///
/// | block | count | why |
/// |---|---|---|
/// | node-aligned | 125 | `L*`, `a*`, `b*` each on `{0, 8, 16, 24, 32}/32` of the encoded axis — **exact CLUT nodes of a 33-point table**, the nearest thing §A has to an interpolation-free control |
/// | neutral axis | 21 | `a* = b* = 0`, `L*` from 0 to 100 in 5s: where a printer profile's black generation lives |
/// | saturated hues | 48 | `L*` ∈ {30, 50, 70, 90} × 12 hue angles at `C* = 60` — well outside any CMYK gamut, so the table's own clipping is exercised |
/// | pseudo-random | 64 | fixed-seed LCG over `L* ∈ [0,100]`, `a*,b* ∈ [−100,100]` |
///
/// **`a*`/`b*` stay inside ±128** so nothing is lost to the encoding clamp
/// before the comparison starts; a value outside it would test the clamp, not
/// the table, and both implementations clamp there anyway.
#[must_use]
pub fn lab_grid() -> Vec<Lab> {
    let mut out: Vec<Lab> = Vec::new();
    let push = |l: f64, a: f64, b: f64, out: &mut Vec<Lab>| {
        let k = |v: f64| v.to_bits();
        if !out
            .iter()
            .any(|e| k(e.l) == k(l) && k(e.a) == k(a) && k(e.b) == k(b))
        {
            out.push(Lab { l, a, b });
        }
    };
    // Node-aligned: normalised n = j/32 for j in {0, 8, 16, 24, 32}. The
    // decodes are exact in binary f64 (100/32 = 3.125, 255/32 = 7.96875).
    for &jl in &[0usize, 8, 16, 24, 32] {
        for &ja in &[0usize, 8, 16, 24, 32] {
            for &jb in &[0usize, 8, 16, 24, 32] {
                #[allow(clippy::cast_precision_loss)]
                let n = |j: usize| j as f64 / 32.0;
                push(
                    100.0 * n(jl),
                    255.0 * n(ja) - 128.0,
                    255.0 * n(jb) - 128.0,
                    &mut out,
                );
            }
        }
    }
    for i in 0..=20 {
        push(f64::from(i) * 5.0, 0.0, 0.0, &mut out);
    }
    for &l in &[30.0, 50.0, 70.0, 90.0] {
        for h in 0..12 {
            let th = f64::from(h) * std::f64::consts::TAU / 12.0;
            push(l, 60.0 * th.cos(), 60.0 * th.sin(), &mut out);
        }
    }
    let mut x: u64 = 0x1CCC_E000_0004_0A2B;
    let mut next = || -> f64 {
        x = x
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        #[allow(clippy::cast_precision_loss)]
        let u = (x >> 11) as f64 / ((1u64 << 53) as f64);
        u
    };
    for _ in 0..64 {
        let l = next() * 100.0;
        let a = next() * 200.0 - 100.0;
        let b = next() * 200.0 - 100.0;
        push(l, a, b, &mut out);
    }
    out
}

/// §B's device grid: **deterministic CMYK quadruples in `[0,1]`**.
///
/// The fixture's `A2B0` depends only on `K` by construction, so the grid's job
/// is different from Pass 4's: it must prove that *nothing else* moves the
/// answer. Hence a `K` ramp at every one of the two `K` nodes and between them,
/// crossed with C/M/Y values chosen to sit on, between and outside the 5/4/3
/// nodes of the axes that must not matter.
///
/// | block | count | why |
/// |---|---|---|
/// | K ramp at C=M=Y=0 | 17 | the axis that does matter, at 1/16 steps |
/// | K ramp × 4 CMY settings | 68 | the same ramp with C/M/Y at node, off-node and extreme values: any dependence on them is a defect |
/// | corners | 16 | the hypercube |
/// | pseudo-random | 48 | fixed-seed LCG into `[0,1]⁴` |
#[must_use]
pub fn cmyk_grid() -> Vec<[f64; 4]> {
    let mut out: Vec<[f64; 4]> = Vec::new();
    let push = |t: [f64; 4], out: &mut Vec<[f64; 4]>| {
        let key = |v: f64| v.to_bits();
        if !out.iter().any(|e| (0..4).all(|i| key(e[i]) == key(t[i]))) {
            out.push(t);
        }
    };
    for c in [0.0, 1.0] {
        for m in [0.0, 1.0] {
            for y in [0.0, 1.0] {
                for k in [0.0, 1.0] {
                    push([c, m, y, k], &mut out);
                }
            }
        }
    }
    // C/M/Y settings: all-zero, exact nodes (1/4 of a 5-node axis, 1/3 of a
    // 4-node axis, 1/2 of a 3-node axis), deliberately off-node, and full.
    let cmy: [[f64; 3]; 4] = [
        [0.0, 0.0, 0.0],
        [0.25, 1.0 / 3.0, 0.5],
        [0.37, 0.61, 0.83],
        [1.0, 1.0, 1.0],
    ];
    for s in cmy {
        for i in 0..=16 {
            let k = f64::from(i) / 16.0;
            push([s[0], s[1], s[2], k], &mut out);
        }
    }
    let mut x: u64 = 0x1CCC_E000_0004_0C4B;
    let mut next = || -> f64 {
        x = x
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        #[allow(clippy::cast_precision_loss)]
        let u = (x >> 11) as f64 / ((1u64 << 53) as f64);
        u
    };
    for _ in 0..48 {
        let t = [next(), next(), next(), next()];
        push(t, &mut out);
    }
    out
}

/// §C's axis: `0 … 1` in 1/64 steps, plus the four darkest 8-bit codes.
///
/// 65 uniform steps because the question is the *shape* of the disagreement
/// along the axis, not a sample of it; the 8-bit codes 1/255 … 4/255 are added
/// because the destination inverse TRC's slope is steepest there and every
/// term in [`DEVICE_GRAY`]'s derivation is largest there.
#[must_use]
pub fn gray_axis() -> Vec<f64> {
    let mut v: Vec<f64> = (0..=64).map(|i| f64::from(i) / 64.0).collect();
    for i in 1..=4 {
        v.push(f64::from(i) / 255.0);
    }
    v.sort_by(f64::total_cmp);
    v.dedup();
    v
}

// ===========================================================================
// A CLUT, evaluated two ways — the ragged generalisation of pass4's
// ===========================================================================

/// Which CLUT geometry to evaluate with. Same meaning as
/// [`crate::pass4::Scheme`], restated here because §A's question is the
/// *opposite* of §14.2's: there, lcms2's geometry differed from iccce's and the
/// difference had to be modelled; here lcms2 forces the geometry to match, and
/// the tetrahedral arm exists only to show what that override is worth.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scheme {
    /// n-linear over every input (trilinear when there are three). iccce's
    /// choice (NA-006), **and** lcms2's for any Lab-PCS output LUT.
    NLinear,
    /// lcms2's default for 3 inputs — Sakamoto tetrahedral — recursing on the
    /// leading channels when there are more, exactly as `Eval4Inputs` does.
    /// **Not reached for a Lab-PCS B2A**; the counterfactual arm.
    Lcms2Default,
}

/// A CLUT with **per-dimension** grid sizes, evaluated by either geometry.
///
/// `lut16Type`'s `clutPoints` is one byte for every axis; `lutAToBType` has a
/// 16-byte array. This type carries the array so the same code serves both, and
/// so §B's ragged 5×4×3×2 fixture is evaluated by the same lines as §A's
/// hypercubic 33³ — a second implementation for the ragged case would be a
/// second place for an index-order bug to hide.
///
/// Samples are normalised `f64`, first input channel **slowest** (clause 10.10
/// / 10.12, corpus A20), a node's outputs contiguous.
#[derive(Debug, Clone)]
pub struct HarnessClut {
    dims: Vec<usize>,
    outputs: usize,
    data: Vec<f64>,
}

impl HarnessClut {
    /// Build from raw normalised samples. Panics if the sample count does not
    /// match the declared shape — a mis-shaped CLUT is a programming error in
    /// the harness, not a runtime condition, and a plausible answer computed
    /// from the wrong shape is exactly what this crate exists to prevent.
    #[must_use]
    pub fn new(dims: Vec<usize>, outputs: usize, data: Vec<f64>) -> HarnessClut {
        let want: usize = dims.iter().product::<usize>() * outputs;
        assert_eq!(want, data.len(), "CLUT sample count does not match its shape");
        HarnessClut {
            dims,
            outputs,
            data,
        }
    }

    fn node(&self, idx: &[usize]) -> usize {
        let mut flat = 0usize;
        for (d, &i) in idx.iter().enumerate() {
            flat = flat * self.dims[d] + i;
        }
        flat * self.outputs
    }

    /// Evaluate at `input` (each component 0..1) with the given geometry.
    pub fn eval(&self, input: &[f64], scheme: Scheme, out: &mut [f64]) {
        match scheme {
            Scheme::NLinear => self.eval_nlinear(input, out),
            Scheme::Lcms2Default => self.eval_lcms2(input, 0, 0, out),
        }
    }

    /// The 2ⁿ-corner convex combination — `iccce_cmm::clut::Clut::eval`'s
    /// algorithm, including its clamped-index-then-fraction pairing.
    fn eval_nlinear(&self, input: &[f64], out: &mut [f64]) {
        let d = self.dims.len();
        let mut base = vec![0usize; d];
        let mut frac = vec![0.0f64; d];
        for dim in 0..d {
            let (i, f) = cell(input[dim], self.dims[dim]);
            base[dim] = i;
            frac[dim] = f;
        }
        out.fill(0.0);
        let mut idx = vec![0usize; d];
        for corner in 0..(1usize << d) {
            let mut w = 1.0f64;
            for dim in 0..d {
                let hi = (corner >> (d - 1 - dim)) & 1 == 1;
                idx[dim] = base[dim] + usize::from(hi);
                w *= if hi { frac[dim] } else { 1.0 - frac[dim] };
            }
            if w == 0.0 {
                continue;
            }
            let b = self.node(&idx);
            for (o, slot) in out.iter_mut().enumerate() {
                *slot += w * self.data[b + o];
            }
        }
    }

    /// lcms2's default geometry: tetrahedral over the last three channels,
    /// linear along each leading one (`Eval4Inputs`' structure).
    fn eval_lcms2(&self, input: &[f64], dim: usize, base: usize, out: &mut [f64]) {
        let remaining = self.dims.len() - dim;
        if remaining == 3 {
            self.tetrahedral(input, dim, base, out);
            return;
        }
        if remaining == 1 {
            // Not a shape lcms2 reaches through this factory, but the harness
            // must not silently produce a wrong answer for it.
            let (k0, rest) = cell_lcms2(input[dim], self.dims[dim]);
            let stride = self.outputs;
            let lo = base + stride * k0;
            let hi = if fclamp(input[dim]) >= 1.0 { lo } else { lo + stride };
            for (o, slot) in out.iter_mut().enumerate() {
                *slot = self.data[lo + o] + (self.data[hi + o] - self.data[lo + o]) * rest;
            }
            return;
        }
        if remaining == 2 {
            let (k0, rest) = cell_lcms2(input[dim], self.dims[dim]);
            let stride = self.outputs * self.dims[dim + 1];
            let lo = base + stride * k0;
            let hi = if fclamp(input[dim]) >= 1.0 { lo } else { lo + stride };
            let mut a = vec![0.0f64; self.outputs];
            let mut b = vec![0.0f64; self.outputs];
            self.eval_lcms2(input, dim + 1, lo, &mut a);
            self.eval_lcms2(input, dim + 1, hi, &mut b);
            for o in 0..self.outputs {
                out[o] = a[o] + (b[o] - a[o]) * rest;
            }
            return;
        }
        let (k0, rest) = cell_lcms2(input[dim], self.dims[dim]);
        let mut stride = self.outputs;
        for d in (dim + 1)..self.dims.len() {
            stride *= self.dims[d];
        }
        let lo = base + stride * k0;
        let hi = if fclamp(input[dim]) >= 1.0 { lo } else { lo + stride };
        let mut a = vec![0.0f64; self.outputs];
        let mut b = vec![0.0f64; self.outputs];
        self.eval_lcms2(input, dim + 1, lo, &mut a);
        self.eval_lcms2(input, dim + 1, hi, &mut b);
        for o in 0..self.outputs {
            out[o] = a[o] + (b[o] - a[o]) * rest;
        }
    }

    /// Sakamoto tetrahedral over three channels of a sub-cube rooted at flat
    /// offset `base`, transcribed from `TetrahedralInterpFloat` at pin
    /// `21c582a`. `dim` is the first of the three; `dim` has the largest
    /// stride, `dim+2` the smallest.
    fn tetrahedral(&self, input: &[f64], dim: usize, base: usize, out: &mut [f64]) {
        let n = self.outputs;
        let sz = n;
        let sy = n * self.dims[dim + 2];
        let sx = n * self.dims[dim + 2] * self.dims[dim + 1];
        let (x0, rx) = cell_lcms2(input[dim], self.dims[dim]);
        let (y0, ry) = cell_lcms2(input[dim + 1], self.dims[dim + 1]);
        let (z0, rz) = cell_lcms2(input[dim + 2], self.dims[dim + 2]);
        let big_x0 = sx * x0;
        let big_x1 = big_x0 + if fclamp(input[dim]) >= 1.0 { 0 } else { sx };
        let big_y0 = sy * y0;
        let big_y1 = big_y0 + if fclamp(input[dim + 1]) >= 1.0 { 0 } else { sy };
        let big_z0 = sz * z0;
        let big_z1 = big_z0 + if fclamp(input[dim + 2]) >= 1.0 { 0 } else { sz };

        for (ch, slot) in out.iter_mut().enumerate().take(n) {
            let dens = |i: usize, j: usize, k: usize| self.data[base + i + j + k + ch];
            let c0 = dens(big_x0, big_y0, big_z0);
            let (c1, c2, c3) = if rx >= ry && ry >= rz {
                (
                    dens(big_x1, big_y0, big_z0) - c0,
                    dens(big_x1, big_y1, big_z0) - dens(big_x1, big_y0, big_z0),
                    dens(big_x1, big_y1, big_z1) - dens(big_x1, big_y1, big_z0),
                )
            } else if rx >= rz && rz >= ry {
                (
                    dens(big_x1, big_y0, big_z0) - c0,
                    dens(big_x1, big_y1, big_z1) - dens(big_x1, big_y0, big_z1),
                    dens(big_x1, big_y0, big_z1) - dens(big_x1, big_y0, big_z0),
                )
            } else if rz >= rx && rx >= ry {
                (
                    dens(big_x1, big_y0, big_z1) - dens(big_x0, big_y0, big_z1),
                    dens(big_x1, big_y1, big_z1) - dens(big_x1, big_y0, big_z1),
                    dens(big_x0, big_y0, big_z1) - c0,
                )
            } else if ry >= rx && rx >= rz {
                (
                    dens(big_x1, big_y1, big_z0) - dens(big_x0, big_y1, big_z0),
                    dens(big_x0, big_y1, big_z0) - c0,
                    dens(big_x1, big_y1, big_z1) - dens(big_x1, big_y1, big_z0),
                )
            } else if ry >= rz && rz >= rx {
                (
                    dens(big_x1, big_y1, big_z1) - dens(big_x0, big_y1, big_z1),
                    dens(big_x0, big_y1, big_z0) - c0,
                    dens(big_x0, big_y1, big_z1) - dens(big_x0, big_y1, big_z0),
                )
            } else if rz >= ry && ry >= rx {
                (
                    dens(big_x1, big_y1, big_z1) - dens(big_x0, big_y1, big_z1),
                    dens(big_x0, big_y1, big_z1) - dens(big_x0, big_y0, big_z1),
                    dens(big_x0, big_y0, big_z1) - c0,
                )
            } else {
                (0.0, 0.0, 0.0)
            };
            *slot = c0 + c1 * rx + c2 * ry + c3 * rz;
        }
    }

    /// The largest absolute step between adjacent nodes along any axis, per
    /// output channel — the quantity every "one lsb of input costs this much
    /// output" estimate in this file's tolerances is built from. Reported so
    /// the estimates can be checked rather than believed.
    #[must_use]
    pub fn max_adjacent_step(&self) -> f64 {
        let d = self.dims.len();
        let mut idx = vec![0usize; d];
        let total: usize = self.dims.iter().product();
        let mut worst = 0.0f64;
        for flat in 0..total {
            let mut rest = flat;
            for dim in (0..d).rev() {
                idx[dim] = rest % self.dims[dim];
                rest /= self.dims[dim];
            }
            for dim in 0..d {
                if idx[dim] + 1 >= self.dims[dim] {
                    continue;
                }
                let a = self.node(&idx);
                let mut j = idx.clone();
                j[dim] += 1;
                let b = self.node(&j);
                for o in 0..self.outputs {
                    worst = worst.max((self.data[b + o] - self.data[a + o]).abs());
                }
            }
        }
        worst
    }
}

/// Round to the nearest 1/65535 — lcms2's `_cmsQuickSaturateWord(v*65535)`
/// followed by `/65535`, which is what its float path does at every tabulated
/// curve and at every CLUT stage boundary.
fn q16(v: f64) -> f64 {
    (v.clamp(0.0, 1.0) * 65535.0).round() / 65535.0
}

// ===========================================================================
// §A — the B2A pipeline, reimplemented so lcms2's arithmetic can be switched on
// ===========================================================================

/// The `lut8Type` **PCS→device** pipeline, rebuilt in the harness.
///
/// Exists for the same reason [`crate::pass4::SourcePipeline`] does: the
/// experiments need one pipeline evaluated several ways, differing in exactly
/// one component, and that substitution cannot be made inside `crates/`. The
/// two switches are [`Scheme`] (geometry) and `quantise` (whether lcms2's
/// roundings are applied).
///
/// `iccce-cmm`'s own evaluator is held against the `NLinear`, unquantised arm
/// on every grid point before any conclusion is drawn from this type
/// ([`APPARATUS_B2A`]).
#[derive(Debug, Clone)]
pub struct B2aPipeline {
    /// 3 tables of 256 entries, normalised ÷255.
    input_tables: Vec<Vec<f64>>,
    clut: HarnessClut,
    /// 4 tables of 256 entries, normalised ÷255.
    output_tables: Vec<Vec<f64>>,
}

impl B2aPipeline {
    /// Build from a decoded `mft1` tag. The 3×3 matrix is **not** applied: it
    /// is only applicable when the tag's input is PCSXYZ (clause 10.10, corpus
    /// A21), and this profile's PCS is `Lab `.
    #[must_use]
    pub fn from_lut8(lut: &Lut8) -> B2aPipeline {
        let inputs = usize::from(lut.input_chan);
        let outputs = usize::from(lut.output_chan);
        let points = usize::from(lut.clut_points);
        // FROM_8_TO_16(v)/65535 == v/255 exactly (257*255 = 65535), so this is
        // lcms2's normalisation as well as iccce's.
        let norm = |v: u8| f64::from(v) / 255.0;
        B2aPipeline {
            input_tables: (0..inputs)
                .map(|c| lut.input_tables[c * 256..(c + 1) * 256].iter().copied().map(norm).collect())
                .collect(),
            clut: HarnessClut::new(
                vec![points; inputs],
                outputs,
                lut.clut.iter().copied().map(norm).collect(),
            ),
            output_tables: (0..outputs)
                .map(|c| {
                    lut.output_tables[c * 256..(c + 1) * 256]
                        .iter()
                        .copied()
                        .map(norm)
                        .collect()
                })
                .collect(),
        }
    }

    /// The largest adjacent-node step in the CLUT, per output channel.
    #[must_use]
    pub fn max_adjacent_step(&self) -> f64 {
        self.clut.max_adjacent_step()
    }

    /// Encode a D50 CIELAB value the way **both** implementations do for a
    /// `lut8Type` tag: `L*/100`, `(ab + 128)/255`, clamped.
    ///
    /// See the module header for why this is not the legacy 652,8 encoding and
    /// why the fact is worth an assertion rather than a comment: the two forms
    /// differ by 0,39 %, which is ≈0,2 ΔE2000 — below the perceptibility anchor
    /// and therefore invisible to any suite graded at it.
    #[must_use]
    pub fn encode_lab(lab: Lab) -> [f64; 3] {
        [
            (lab.l / 100.0).clamp(0.0, 1.0),
            ((lab.a + 128.0) / 255.0).clamp(0.0, 1.0),
            ((lab.b + 128.0) / 255.0).clamp(0.0, 1.0),
        ]
    }

    /// Evaluate PCS → device.
    ///
    /// With `quantise = false` this is iccce's arithmetic: `f64` throughout.
    /// With `quantise = true` it is lcms2's, modelled stage by stage:
    ///
    /// | lcms2 | what it does | modelled as |
    /// |---|---|---|
    /// | `cmsEvalToneCurveFloat` on a 256-entry `Table16` (`nSegments == 0`) | rounds input **and** output to 1/65535 | `q16(interp(t, q16(x)))` |
    /// | `EvaluateCLUTfloatIn16` | rounds the stage input to `u16`, returns `u16/65535` | `q16` either side of the CLUT |
    /// | `TrilinearInterp16` | interpolates in 16-bit fixed point | **not modelled** — `f64`; budgeted in [`DEVICE_B2A_MODELLED`] |
    #[must_use]
    pub fn eval(&self, lab: Lab, scheme: Scheme, quantise: bool) -> Vec<f64> {
        let enc = Self::encode_lab(lab);
        let stage = |t: &[f64], x: f64| -> f64 {
            if quantise {
                q16(interp_table(t, q16(x)))
            } else {
                interp_table(t, x)
            }
        };
        let v: Vec<f64> = (0..3).map(|i| stage(&self.input_tables[i], enc[i])).collect();
        let v: Vec<f64> = if quantise { v.iter().map(|&x| q16(x)).collect() } else { v };
        let mut clut_out = vec![0.0f64; self.output_tables.len()];
        self.clut.eval(&v, scheme, &mut clut_out);
        if quantise {
            for c in &mut clut_out {
                *c = q16(*c);
            }
        }
        (0..self.output_tables.len())
            .map(|i| stage(&self.output_tables[i], clut_out[i]))
            .collect()
    }
}

/// lcms2's source-side matrix/TRC evaluation, with its tabulated-curve
/// quantisation switched on or off.
///
/// `cmsEvalToneCurveFloat` rounds a sampled curve's input *and* output to
/// 1/65535 (§13.6.1, measured at the pin); the sRGB profile's TRCs are
/// 1024-entry `curv` tables, so this fires on every conversion out of it. The
/// colorant matrix is `f64` on both sides.
fn srgb_to_xyz(model: &MatrixTrc, rgb: [f64; 3], quantise: bool) -> Xyz {
    let linear = if quantise {
        [
            q16(model.trc[0].eval(q16(rgb[0]))),
            q16(model.trc[1].eval(q16(rgb[1]))),
            q16(model.trc[2].eval(q16(rgb[2]))),
        ]
    } else {
        [
            model.trc[0].eval(rgb[0]),
            model.trc[1].eval(rgb[1]),
            model.trc[2].eval(rgb[2]),
        ]
    };
    let v = model.matrix().apply(linear);
    Xyz {
        x: v[0],
        y: v[1],
        z: v[2],
    }
}

// ===========================================================================
// §C — lcms2's reverse tone curve, reimplemented
// ===========================================================================

/// lcms2's `cmsReverseToneCurveEx(4096, curve)` for a **tabulated** curve, plus
/// the float evaluation of the result.
///
/// Transcribed from `cmsgamma.c` at pin `21c582a`. The algorithm, and the two
/// details that matter:
///
/// ```c
/// for (i = 0; i < nResultSamples; i++) {
///     y = i * 65535.0 / (nResultSamples - 1);
///     j = GetInterval(y, InCurve->Table16, InCurve->InterpParams);
///     x1 = Table16[j];  x2 = Table16[j+1];
///     y1 = j*65535.0/(nEntries-1);  y2 = (j+1)*65535.0/(nEntries-1);
///     a = (y2 - y1) / (x2 - x1);  b = y2 - a*x2;
///     out->Table16[i] = _cmsQuickSaturateWord(a*y + b);
/// }
/// ```
///
/// 1. **The result is a `u16` table**, so every sample is rounded to 1/65535 —
///    and being tabulated, `cmsEvalToneCurveFloat` then rounds its input and
///    output to 1/65535 again on every evaluation.
/// 2. **Its 4096 knots do not coincide with the forward table's 1024 knots.**
///    Between two reverse knots the true inverse is only piecewise linear if no
///    forward knot falls inside; where one does, the reverse table chords
///    across it. That is the resampling term in [`DEVICE_GRAY`], and it is why
///    the disagreement is *not* uniform along the axis.
#[derive(Debug, Clone)]
pub struct ReverseCurve {
    table: Vec<f64>,
}

impl ReverseCurve {
    /// Build the reverse of a normalised forward table, lcms2's way.
    #[must_use]
    pub fn build(forward: &[u16], samples: usize) -> ReverseCurve {
        let n_entries = forward.len();
        // lcms2 works in u16 codes throughout, and so does the stored `curv`
        // tag, so the codes are taken as stored rather than round-tripped
        // through a normalised f64 — a round trip would be exact here but
        // would put a rounding step in a model of an algorithm that has none.
        let fwd16: Vec<f64> = forward.iter().map(|&v| f64::from(v)).collect();
        let ascending = fwd16[0] < fwd16[n_entries - 1];
        let mut out = Vec::with_capacity(samples);
        #[allow(clippy::cast_precision_loss)]
        let last = (samples - 1) as f64;
        for i in 0..samples {
            #[allow(clippy::cast_precision_loss)]
            let y = i as f64 * 65535.0 / last;
            // GetInterval, ascending branch: scan from the top down.
            let mut j: i64 = -1;
            if ascending {
                for k in (0..n_entries - 1).rev() {
                    let (y0, y1) = (fwd16[k], fwd16[k + 1]);
                    let hit = if y0 <= y1 {
                        y >= y0 && y <= y1
                    } else {
                        y >= y1 && y <= y0
                    };
                    if hit {
                        #[allow(clippy::cast_possible_wrap)]
                        {
                            j = k as i64;
                        }
                        break;
                    }
                }
            } else {
                for k in 0..n_entries - 1 {
                    let (y0, y1) = (fwd16[k], fwd16[k + 1]);
                    let hit = if y0 <= y1 {
                        y >= y0 && y <= y1
                    } else {
                        y >= y1 && y <= y0
                    };
                    if hit {
                        #[allow(clippy::cast_possible_wrap)]
                        {
                            j = k as i64;
                        }
                        break;
                    }
                }
            }
            let v = if j >= 0 {
                #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
                let ju = j as usize;
                let x1 = fwd16[ju];
                let x2 = fwd16[ju + 1];
                #[allow(clippy::cast_precision_loss)]
                let denom = (n_entries - 1) as f64;
                #[allow(clippy::cast_precision_loss)]
                let y1 = ju as f64 * 65535.0 / denom;
                #[allow(clippy::cast_precision_loss)]
                let y2 = (ju + 1) as f64 * 65535.0 / denom;
                if (x1 - x2).abs() < f64::EPSILON {
                    if ascending { y2 } else { y1 }
                } else {
                    let a = (y2 - y1) / (x2 - x1);
                    let b = y2 - a * x2;
                    a * y + b
                }
            } else {
                // lcms2 falls through with whatever a and b were last set to.
                // On a monotonic curve every y finds an interval, so this arm
                // is unreachable here; producing 0 rather than stale state
                // makes any future non-monotonic case fail loudly.
                0.0
            };
            out.push(v.clamp(0.0, 65535.0).round() / 65535.0);
        }
        ReverseCurve { table: out }
    }

    /// Evaluate as `cmsEvalToneCurveFloat` does on a tabulated curve: round the
    /// input to 1/65535, interpolate, round the output to 1/65535.
    #[must_use]
    pub fn eval(&self, x: f64) -> f64 {
        q16(interp_table(&self.table, q16(x)))
    }
}

// ===========================================================================
// §B — the derived expectations
// ===========================================================================

/// The `mAB ` A2B0 of `v4-cmyk-mab-lab.icc`, in closed form.
///
/// Derivation, every step citing what it comes from:
///
/// 1. **Element order** — ICC.1:2022 10.12.1: `A` curves, CLUT, `M` curves,
///    matrix, `B` curves. All the curves in this fixture are `curv` with
///    `count = 0`, i.e. the identity (10.5).
/// 2. **The CLUT** stores `L*` as `general_lab_l(100·(1 − k))` at the two `K`
///    nodes and `general_lab_ab(0)` for `a*`/`b*`, independent of C, M, Y
///    (`tools/gen-profiles/src/recipes.rs`, `v4_cmyk_mab_lab`). In normalised
///    terms the nodes are `1` and `0` for `L*` and `32896/65535` for `a*`/`b*`
///    (`0x8080`, Table 13's zero code). With two nodes on the `K` axis the
///    interpolant over the whole axis is the exact line `1 − K`, and **every
///    interpolation geometry reproduces it exactly**.
/// 3. **The matrix** (10.12.5) is the identity with offsets `1/256`, `2/256`,
///    `3/256`, applied in the normalised domain.
/// 4. **The decode** is the *general* 16-bit PCSLAB encoding of 6.3.4.2 Tables
///    12/13 — `L* = 100n`, `ab = 255n − 128` — because `mAB ` is **not** in
///    NOTE 3's legacy set (`lut16Type` and `namedColor2Type`, "and only those
///    tag types").
///
/// giving `L* = 100(1 − K) + 0,390625`, `a* = 1,9921875`, `b* = 2,98828125`.
///
/// ## ★ The clamp question this function does NOT answer
///
/// At `K = 0` the encoded `L*` is `1 + 1/256 = 1,00390625` — **outside the
/// representable range of the PCS encoding it is about to be read as**. iccce
/// clamps (its `Trc::eval` enforces clause 10.18's `[0,1]` domain at the `B`
/// curve, so `L* = 100`); lcms2 does not (its identity curve is an analytic
/// gamma-1 segment evaluated unbounded, so `L* = 100,390625`). The function
/// takes a parameter rather than a side: `clamped = true` is iccce's reading,
/// `false` is lcms2's, and §15.3 records the question and the fact that it is
/// unsettled.
#[must_use]
pub fn expected_mab_lab(cmyk: [f64; 4], clamped: bool) -> Lab {
    let k = cmyk[3].clamp(0.0, 1.0);
    // The stored nodes, exactly as the generator wrote them: 0xFFFF and
    // 0x0000 for L*, 0x8080 for a*/b* (Table 13's zero code). Normalised by
    // 65535, so the L* nodes are 1 and 0 and the interpolant over the two-node
    // K axis is the exact line 1 - K.
    // 65535/65535; written as the literal because clippy rejects the division
    // of equal operands, and the point of the fraction was only to show which
    // stored code it is.
    const L_NODE_HI: f64 = 1.0;
    const L_NODE_LO: f64 = 0.0;
    const AB_ZERO: f64 = 32896.0 / 65535.0;
    let l_n = L_NODE_LO + (L_NODE_HI - L_NODE_LO) * (1.0 - k);
    let ab_n = AB_ZERO;
    let after = |n: f64, off: f64| {
        let v = n + off;
        if clamped { v.clamp(0.0, 1.0) } else { v }
    };
    Lab {
        l: after(l_n, 1.0 / 256.0) * 100.0,
        a: after(ab_n, 2.0 / 256.0) * 255.0 - 128.0,
        b: after(ab_n, 3.0 / 256.0) * 255.0 - 128.0,
    }
}

/// The `mBA ` B2A0 of `v4-cmyk-mab-lab.icc`, in closed form.
///
/// Same derivation, mirrored (10.13.1's order is `B`, matrix, `M`, CLUT, `A`):
///
/// 1. Encode the PCS with the **general** encoding: `n_L = L*/100`,
///    `n_ab = (ab + 128)/255`, clamped to `[0,1]` — this direction's clamp is
///    not in dispute, because an encoded value outside `[0,1]` has no `u16` to
///    be.
/// 2. `B` curves identity; matrix adds `1/256` to `n_L`.
/// 3. CLUT: `C = M = Y = 0` and `K` interpolated along the **`L*` axis only**,
///    from the stored nodes `65535`, `32768`, `0` — note the middle node is
///    `round(0,5 · 65535) = 32768`, i.e. `0,500 007 63`, **not** `0,5`. The
///    expectation uses the stored value; an idealised `1 − L` would be wrong by
///    7,6×10⁻⁶ and would look like an implementation defect.
/// 4. `A` curves identity; the result is device CMYK in `[0,1]`.
///
/// The `a*` and `b*` axes are constant in the stored table, so their offsets
/// (`2/256`, `3/256`) are applied and provably cannot change the answer — which
/// is a property worth having a grid wide in `a*`/`b*` to confirm.
#[must_use]
pub fn expected_mba_cmyk(lab: Lab) -> [f64; 4] {
    let n_l = (lab.l / 100.0).clamp(0.0, 1.0) + 1.0 / 256.0;
    let n_l = n_l.clamp(0.0, 1.0);
    // Three nodes at 0, 1/2, 1 of the axis, values 65535, 32768, 0 (÷65535).
    let nodes = [1.0, 32768.0 / 65535.0, 0.0];
    let pos = n_l * 2.0;
    let i = if pos >= 1.0 { 1usize } else { 0usize };
    #[allow(clippy::cast_precision_loss)]
    let f = pos - i as f64;
    let k = nodes[i] + (nodes[i + 1] - nodes[i]) * f;
    [0.0, 0.0, 0.0, k]
}

// ===========================================================================
// Reductions and small helpers
// ===========================================================================

fn max_mean(v: &[f64]) -> (f64, f64) {
    if v.is_empty() {
        return (f64::NAN, f64::NAN);
    }
    let finite: Vec<f64> = v.iter().copied().filter(|x| x.is_finite()).collect();
    if finite.is_empty() {
        return (f64::NAN, f64::NAN);
    }
    #[allow(clippy::cast_precision_loss)]
    let n = finite.len() as f64;
    (
        finite.iter().copied().fold(0.0f64, f64::max),
        finite.iter().sum::<f64>() / n,
    )
}

fn max_at(v: &[f64], keep: &[bool]) -> f64 {
    v.iter()
        .zip(keep)
        .filter(|(x, k)| **k && x.is_finite())
        .map(|(x, _)| *x)
        .fold(0.0f64, f64::max)
}

fn lab_dist(a: Lab, b: Lab) -> f64 {
    (a.l - b.l).abs().max((a.a - b.a).abs()).max((a.b - b.b).abs())
}

fn to_lab(model: &MatrixTrc, rgb: [f64; 3]) -> Lab {
    Lab::from_xyz(model.device_to_pcs(rgb), D50)
}

fn read_lut8(profile: &Profile, sig: Signature) -> Option<Lut8> {
    let entry = profile.tags.iter().find(|t| t.sig == sig)?;
    match profile.decode_tag(entry) {
        Some(Ok(d)) => match d.data {
            TagData::Lut8(l) => Some(l),
            _ => None,
        },
        _ => None,
    }
}

fn read_lut_ab(profile: &Profile, sig: Signature) -> Option<LutAB> {
    let entry = profile.tags.iter().find(|t| t.sig == sig)?;
    match profile.decode_tag(entry) {
        Some(Ok(d)) => match d.data {
            TagData::LutAToB(l) | TagData::LutBToA(l) => Some(l),
            _ => None,
        },
        _ => None,
    }
}

fn read_lut16(profile: &Profile, sig: Signature) -> Option<iccce_profile::lut::Lut16> {
    let entry = profile.tags.iter().find(|t| t.sig == sig)?;
    match profile.decode_tag(entry) {
        Some(Ok(d)) => match d.data {
            TagData::Lut16(l) => Some(l),
            _ => None,
        },
        _ => None,
    }
}

/// Why a section could not run. Same contract as [`crate::pass4::Unavailable`]:
/// a missing category (c) profile is a **skip**, a broken oracle is an
/// **error**, and neither is ever a pass.
#[derive(Debug)]
pub enum Unavailable {
    Skip(String),
    Error(String),
}

impl std::fmt::Display for Unavailable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unavailable::Skip(s) | Unavailable::Error(s) => write!(f, "{s}"),
        }
    }
}

impl From<DiffError> for Unavailable {
    fn from(e: DiffError) -> Self {
        Unavailable::Error(e.to_string())
    }
}

// ===========================================================================
// §A — the analysis
// ===========================================================================

/// One intent's worth of §A.
#[derive(Debug)]
pub struct B2aIntentRun {
    pub intent: Intent,
    /// iccce's CMYK from the **shipped binary**, 0..1.
    pub iccce_cmyk: Vec<Vec<f64>>,
    /// lcms2's CMYK as printed, 0..100.
    pub lcms2_cmyk_100: Vec<Vec<f64>>,
    /// The harness's model of lcms2: quantised source TRCs, quantised lut8
    /// pipeline, trilinear.
    pub modelled_cmyk: Vec<Vec<f64>>,
    /// The same pipeline with **no** quantisation — iccce's arithmetic.
    pub exact_cmyk: Vec<Vec<f64>>,
    /// Device-space deviations, normalised 0..1.
    pub device_dev: Vec<f64>,
    pub device_dev_modelled: Vec<f64>,
    /// The quantisation envelope: exact vs fully-modelled, **no lcms2 output**.
    pub envelope: Vec<f64>,
    /// Both sides' CMYK carried back through `A2B1` into Lab, then ΔE2000.
    pub de_roundtrip: Vec<f64>,
    /// The counterfactual: trilinear vs tetrahedral on this same table.
    pub counterfactual: Vec<f64>,
    /// The apparatus check: harness pipeline vs `iccce-cmm`'s evaluator.
    pub apparatus: f64,
}

/// Everything §A measured.
#[derive(Debug)]
pub struct B2aAnalysis {
    pub lab_grid: Vec<Lab>,
    pub rgb_grid: Vec<[f64; 3]>,
    pub runs: Vec<B2aIntentRun>,
    /// PCS-side: Lab → CMYK, iccce's `Lut16Model` (in-process) vs `transicc`.
    pub pcs_device_dev: Vec<f64>,
    pub pcs_device_dev_modelled: Vec<f64>,
    pub pcs_counterfactual: Vec<f64>,
    pub structure: String,
    pub max_step: f64,
    /// ★ Added 2026-08-12. Byte-level distinctness of the three `B2A*` tags,
    /// read straight from the file with no parser in the way. See
    /// [`TagDistinctness`].
    pub tag_distinctness: TagDistinctness,
}

/// ★ Are the three `B2A*` tags actually three tables?
///
/// **This exists because the opposite is true one direction away.** Pass 4
/// found `A2B0` and `A2B2` in this same file sharing **one** block of tag data
/// at **one** offset, and graded `pass4/swop/perceptual-equals-saturation` at
/// exactly zero on that basis. If `B2A0` and `B2A2` were laid out the same way,
/// every saturation record in §A would be a byte-identical restatement of the
/// perceptual one, and the coverage statement "saturation was run in the B2A
/// direction" would be true and worthless.
///
/// So it is **measured, from the raw file bytes**, before anything is
/// converted: for each of the three pairs, how many of the 145 588 bytes
/// differ. `identical_pairs` is the graded quantity and must be **0**.
///
/// This is a *structural* fact about one file, not a claim about ICC profiles
/// in general — plenty of real profiles do alias their intent tables, which is
/// exactly why 8.10.2 has a fallback and why this has to be checked per file
/// rather than assumed either way.
#[derive(Debug, Clone)]
pub struct TagDistinctness {
    /// `(label, differing bytes, total bytes)` for `0-vs-1`, `0-vs-2`, `1-vs-2`.
    pub pairs: Vec<(&'static str, usize, usize)>,
    /// Offsets, so a reader can see whether the tags even alias.
    pub offsets: Vec<(&'static str, u32, u32)>,
}

impl TagDistinctness {
    /// The graded quantity: how many of the three pairs are byte-identical.
    /// Must be zero for §A's three intents to be three measurements.
    #[must_use]
    pub fn identical_pairs(&self) -> f64 {
        #[allow(clippy::cast_precision_loss)]
        {
            self.pairs.iter().filter(|(_, d, _)| *d == 0).count() as f64
        }
    }

    /// The least-distinct pair, as a fraction of bytes differing — reported so
    /// "distinct" cannot mean "one byte in 145 588".
    #[must_use]
    pub fn least_distinct_fraction(&self) -> f64 {
        #[allow(clippy::cast_precision_loss)]
        self.pairs
            .iter()
            .map(|(_, d, t)| *d as f64 / *t as f64)
            .fold(f64::INFINITY, f64::min)
    }

    fn describe(&self) -> String {
        let o = self
            .offsets
            .iter()
            .map(|(n, off, sz)| format!("{n}@{off}({sz}B)"))
            .collect::<Vec<_>>()
            .join(" ");
        let p = self
            .pairs
            .iter()
            .map(|(n, d, t)| {
                #[allow(clippy::cast_precision_loss)]
                let f = *d as f64 / *t as f64;
                format!("{n}: {d}/{t} bytes differ ({:.1}%)", f * 100.0)
            })
            .collect::<Vec<_>>()
            .join(" | ");
        format!("{o} | {p}")
    }
}

/// Read the three `B2A*` tags' raw bytes and compare them pairwise.
///
/// Deliberately byte-level and parser-free: the question is whether the file
/// stores three tables, and routing it through `iccce-profile` would make the
/// answer depend on the code under test.
fn measure_tag_distinctness(profile: &Profile, bytes: &[u8]) -> TagDistinctness {
    let sigs: [(&'static str, Signature); 3] =
        [("B2A0", tag::B2A0), ("B2A1", tag::B2A1), ("B2A2", tag::B2A2)];
    let mut offsets = Vec::new();
    let mut blocks: Vec<(&'static str, &[u8])> = Vec::new();
    for (name, sig) in sigs {
        if let Some(t) = profile.tags.iter().find(|t| t.sig == sig) {
            offsets.push((name, t.offset, t.size));
            let start = t.offset as usize;
            let end = start.saturating_add(t.size as usize).min(bytes.len());
            if start < end {
                blocks.push((name, &bytes[start..end]));
            }
        }
    }
    let mut pairs = Vec::new();
    for i in 0..blocks.len() {
        for j in (i + 1)..blocks.len() {
            let (na, a) = blocks[i];
            let (nb, b) = blocks[j];
            let label: &'static str = match (na, nb) {
                ("B2A0", "B2A1") => "B2A0-vs-B2A1",
                ("B2A0", "B2A2") => "B2A0-vs-B2A2",
                ("B2A1", "B2A2") => "B2A1-vs-B2A2",
                _ => "pair",
            };
            let n = a.len().min(b.len());
            let differing = (0..n).filter(|&k| a[k] != b[k]).count() + a.len().abs_diff(b.len());
            pairs.push((label, differing, a.len().max(b.len())));
        }
    }
    TagDistinctness { pairs, offsets }
}

/// Run §A: `sRGB → USWebCoatedSWOP` at perceptual and media-relative, plus the
/// PCS-side `*Lab4 → USWebCoatedSWOP` comparison.
///
/// **Intents.** Media-relative uses `B2A1`, perceptual `B2A0`, saturation
/// `B2A2`; unlike the `A2B0`/`A2B2` pair Pass 4 found sharing one block of tag
/// data, this file's three `B2A*` tags are at three different offsets with
/// three different contents, so the three intents are genuinely different
/// tables and each is worth its own record.
///
/// ## ★ Saturation added 2026-08-12 (`icc-conformance`), and why it is not a
/// third copy of the same shape
///
/// The original text of this comment said *"saturation adds a third copy of the
/// same shape"* and put it out of scope. **That sentence was an assumption, and
/// it was wrong.** SWOP's `B2A2` sits at offset 374 568, 145 588 bytes long —
/// the same *shape* as `B2A0`@83 392 and `B2A1`@228 980, and **not the same
/// bytes**. Pass 4's mirror-image row (`pass4/swop/perceptual-equals-saturation`,
/// graded at *exactly zero*) is what made the assumption plausible: in the
/// **A2B** direction `A2B0` and `A2B2` are one block at one offset, so
/// perceptual and saturation there are the same numbers by construction. **The
/// B2A direction is not built that way in this file**, and
/// [`B2aAnalysis::tag_distinctness`] measures it rather than asserting it, so
/// the saturation records below are a third independent table and not a
/// restatement.
///
/// **ICC-absolute remains out of scope** — it would re-measure §14.6's
/// white-point divergence rather than the B2A path.
///
/// **No forced-BPC confound**: both profiles are v2.1
/// (`cmsGetEncodedICCversion < 0x4000000`), and the versions are printed on
/// every record so a future substitution cannot reintroduce it silently.
pub fn analyse_b2a(oracle: &Oracle) -> Result<B2aAnalysis, Unavailable> {
    let src_path = Path::new(SRGB);
    let dst_path = Path::new(SWOP);
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

    let src_bytes = std::fs::read(src_path).map_err(|e| Unavailable::Error(e.to_string()))?;
    let dst_bytes = std::fs::read(dst_path).map_err(|e| Unavailable::Error(e.to_string()))?;
    let src = Profile::parse(&src_bytes).map_err(|e| Unavailable::Error(e.to_string()))?;
    let dst = Profile::parse(&dst_bytes).map_err(|e| Unavailable::Error(e.to_string()))?;
    let src_model = MatrixTrc::from_profile(&src)
        .map_err(|e| Unavailable::Error(format!("source has no matrix/TRC model: {e}")))?;

    // The route back into a colorimetric space for the ΔE row: the same
    // file's own A2B1. Not a second opinion — the same table's inverse — and
    // the record says so.
    let a2b1 = read_lut16(&dst, tag::A2B1)
        .ok_or_else(|| Unavailable::Error("SWOP has no decodable A2B1".into()))?;
    let a2b1_model = Lut16Model::from_lut16(&a2b1, false, PcsKind::Lab)
        .map_err(|e| Unavailable::Error(e.to_string()))?;
    let back_to_lab = |cmyk: &[f64]| -> Option<Lab> {
        match a2b1_model.device_to_pcs(cmyk) {
            Some(PcsValue::Lab(l)) => Some(l),
            _ => None,
        }
    };

    // ★ Read the three B2A tags' bytes BEFORE anything is converted. If two of
    // them were the same block, one of the three intent runs below would be a
    // restatement rather than a measurement, and §A's coverage sentence would
    // be false in the most flattering possible way.
    let tag_distinctness = measure_tag_distinctness(&dst, &dst_bytes);

    let structure = format!(
        "src v{:08X} {} {}->{} TRC=tabulated | dst v{:08X} {} {}->{} B2A0@{} B2A1@{} B2A2@{} (mft1, 3->4, 33 pts, 8-bit)",
        src.header.version.raw,
        src.header.device_class,
        src.header.color_space,
        src.header.pcs,
        dst.header.version.raw,
        dst.header.device_class,
        dst.header.color_space,
        dst.header.pcs,
        dst.tags
            .iter()
            .find(|t| t.sig == tag::B2A0)
            .map_or("absent".to_string(), |t| t.offset.to_string()),
        dst.tags
            .iter()
            .find(|t| t.sig == tag::B2A1)
            .map_or("absent".to_string(), |t| t.offset.to_string()),
        dst.tags
            .iter()
            .find(|t| t.sig == tag::B2A2)
            .map_or("absent".to_string(), |t| t.offset.to_string()),
    );

    let rgb = rgb_grid();
    let labs = lab_grid();
    let mut runs = Vec::new();
    let mut max_step = 0.0f64;

    for (intent, sig) in [
        (Intent::Perceptual, tag::B2A0),
        (Intent::RelativeColorimetric, tag::B2A1),
        (Intent::Saturation, tag::B2A2),
    ] {
        let lut = read_lut8(&dst, sig)
            .ok_or_else(|| Unavailable::Error(format!("no decodable mft1 for {}", intent.name())))?;
        let pipe = B2aPipeline::from_lut8(&lut);
        let model = Lut16Model::from_lut8(&lut, false, PcsKind::Lab)
            .map_err(|e| Unavailable::Error(e.to_string()))?;
        max_step = max_step.max(pipe.max_adjacent_step());

        // --- iccce, the SHIPPED binary --------------------------------------
        let rows: Vec<Vec<f64>> = rgb.iter().map(|t| t.to_vec()).collect();
        let iccce_cmyk = iccce.transform_rows_shaped(src_path, dst_path, intent, &rows, 4)?;

        // --- lcms2, subprocess. RGB in 0..255, CMYK out 0..100 --------------
        let req = Request {
            input: Space::profile(src_path),
            output: Space::profile(dst_path),
            intent,
            precalc: Precalc::Exact,
            bpc: Bpc::Off,
            values: rgb.iter().flat_map(|t| t.iter().map(|v| v * 255.0)).collect(),
        };
        let lcms2_cmyk_100 = oracle.convert_batch_shaped(&req, 3, 4)?;

        // --- the harness's two models, and the apparatus check ---------------
        let mut modelled_cmyk = Vec::with_capacity(rgb.len());
        let mut exact_cmyk = Vec::with_capacity(rgb.len());
        let mut envelope = Vec::with_capacity(rgb.len());
        let mut counterfactual = Vec::with_capacity(rgb.len());
        let mut apparatus = 0.0f64;
        for t in &rgb {
            let lab_exact = Lab::from_xyz(srgb_to_xyz(&src_model, *t, false), D50);
            let lab_quant = Lab::from_xyz(srgb_to_xyz(&src_model, *t, true), D50);
            let exact = pipe.eval(lab_exact, Scheme::NLinear, false);
            let modelled = pipe.eval(lab_quant, Scheme::NLinear, true);
            let tetra = pipe.eval(lab_exact, Scheme::Lcms2Default, false);
            // The apparatus: the crate's own evaluator on the same Lab.
            match model.pcs_to_device(PcsValue::Lab(lab_exact)) {
                Some(theirs) => {
                    for (a, b) in exact.iter().zip(&theirs) {
                        apparatus = apparatus.max((a - b).abs());
                    }
                }
                None => {
                    return Err(Unavailable::Error(
                        "Lut16Model::pcs_to_device refused a grid point".into(),
                    ));
                }
            }
            envelope.push(
                exact
                    .iter()
                    .zip(&modelled)
                    .map(|(a, b)| (a - b).abs())
                    .fold(0.0f64, f64::max),
            );
            counterfactual.push(
                exact
                    .iter()
                    .zip(&tetra)
                    .map(|(a, b)| (a - b).abs())
                    .fold(0.0f64, f64::max),
            );
            modelled_cmyk.push(modelled);
            exact_cmyk.push(exact);
        }

        // --- reductions -----------------------------------------------------
        let mut device_dev = Vec::with_capacity(rgb.len());
        let mut device_dev_modelled = Vec::with_capacity(rgb.len());
        let mut de_roundtrip = Vec::with_capacity(rgb.len());
        for i in 0..rgb.len() {
            let mine = &iccce_cmyk[i];
            let theirs: Vec<f64> = lcms2_cmyk_100[i].iter().map(|v| v / 100.0).collect();
            device_dev.push(
                mine.iter()
                    .zip(&theirs)
                    .map(|(a, b)| (a - b).abs())
                    .fold(0.0f64, f64::max),
            );
            device_dev_modelled.push(
                modelled_cmyk[i]
                    .iter()
                    .zip(&theirs)
                    .map(|(a, b)| (a - b).abs())
                    .fold(0.0f64, f64::max),
            );
            de_roundtrip.push(match (back_to_lab(mine), back_to_lab(&theirs)) {
                (Some(a), Some(b)) => delta_e_2000(a, b),
                _ => f64::NAN,
            });
        }

        runs.push(B2aIntentRun {
            intent,
            iccce_cmyk,
            lcms2_cmyk_100,
            modelled_cmyk,
            exact_cmyk,
            device_dev,
            device_dev_modelled,
            envelope,
            de_roundtrip,
            counterfactual,
            apparatus,
        });
    }

    // --- the PCS side: Lab -> CMYK, media-relative -------------------------
    // iccce is in-process here and the record says so: the shipped CLI has no
    // Lab entry point, so this grades the MODEL, not the binary. Its value is
    // that the source model is out of the picture entirely, which is what
    // isolates the B2A table.
    let lut = read_lut8(&dst, tag::B2A1).expect("read above");
    let pipe = B2aPipeline::from_lut8(&lut);
    let model = Lut16Model::from_lut8(&lut, false, PcsKind::Lab)
        .map_err(|e| Unavailable::Error(e.to_string()))?;
    let req = Request {
        input: Space::lab_v4(),
        output: Space::profile(dst_path),
        intent: Intent::RelativeColorimetric,
        precalc: Precalc::Exact,
        bpc: Bpc::Off,
        values: labs.iter().flat_map(|l| [l.l, l.a, l.b]).collect(),
    };
    let lcms2 = oracle.convert_batch_shaped(&req, 3, 4)?;
    let mut pcs_device_dev = Vec::with_capacity(labs.len());
    let mut pcs_device_dev_modelled = Vec::with_capacity(labs.len());
    let mut pcs_counterfactual = Vec::with_capacity(labs.len());
    for (i, lab) in labs.iter().enumerate() {
        let mine = model
            .pcs_to_device(PcsValue::Lab(*lab))
            .ok_or_else(|| Unavailable::Error("Lut16Model refused a Lab grid point".into()))?;
        let modelled = pipe.eval(*lab, Scheme::NLinear, true);
        let tetra = pipe.eval(*lab, Scheme::Lcms2Default, false);
        let theirs: Vec<f64> = lcms2[i].iter().map(|v| v / 100.0).collect();
        pcs_device_dev.push(
            mine.iter()
                .zip(&theirs)
                .map(|(a, b)| (a - b).abs())
                .fold(0.0f64, f64::max),
        );
        pcs_device_dev_modelled.push(
            modelled
                .iter()
                .zip(&theirs)
                .map(|(a, b)| (a - b).abs())
                .fold(0.0f64, f64::max),
        );
        pcs_counterfactual.push(
            mine.iter()
                .zip(&tetra)
                .map(|(a, b)| (a - b).abs())
                .fold(0.0f64, f64::max),
        );
    }

    Ok(B2aAnalysis {
        lab_grid: labs,
        rgb_grid: rgb,
        runs,
        pcs_device_dev,
        pcs_device_dev_modelled,
        pcs_counterfactual,
        structure,
        max_step,
        tag_distinctness,
    })
}

// ===========================================================================
// §B — the analysis
// ===========================================================================

/// Everything §B measured.
#[derive(Debug)]
pub struct MabAnalysis {
    pub cmyk_grid: Vec<[f64; 4]>,
    pub lab_grid: Vec<Lab>,
    /// mAB: iccce's Lab (in-process `LutAbModel`) vs the closed form, clamped
    /// reading, in `L*`/`a*`/`b*` units.
    pub mab_iccce_vs_derived: Vec<f64>,
    /// mAB: `transicc -o*Lab4` vs the closed form, **unclamped** reading.
    pub mab_lcms2_vs_derived: Vec<f64>,
    /// mAB: the two implementations against each other, in `L*` units.
    pub mab_cross: Vec<f64>,
    /// Which grid points overflow the encoded PCS (the clamp question).
    pub mab_overflows: Vec<bool>,
    /// mBA: iccce's CMYK (in-process) vs the closed form.
    pub mba_iccce_vs_derived: Vec<f64>,
    /// mBA: `transicc` vs the closed form.
    pub mba_lcms2_vs_derived: Vec<f64>,
    /// End-to-end through the shipped binary: sRGB → fixture (`mBA `), device.
    pub e2e_mba_device: Vec<f64>,
    /// End-to-end: fixture → sRGB (`mAB `), device.
    pub e2e_mab_device: Vec<f64>,
    pub e2e_mab_de: Vec<f64>,
    /// The interpolation-scheme envelope on the fixture's own CLUTs — must be
    /// identically zero, because both are affine.
    pub scheme_envelope: f64,
    /// The measured cost of the clamp divergence, in ΔE2000 through sRGB.
    pub clamp_divergence: f64,
    /// ★ What lcms2's forced BPC does to this v4 fixture at perceptual —
    /// measured **in both directions**, because they are not the same.
    /// `.0` is the fixture as **source** into a v2 destination; `.1` is a v2
    /// source into the fixture as **destination**. See [`analyse_mab`].
    pub forced_bpc_cost: (f64, f64),
    pub structure: String,
}

/// Run §B: the synthetic v4 fixture, both directions.
///
/// **Media-relative only, and that is a measurement rather than a preference.**
/// The fixture is v4, so at perceptual and saturation lcms2 sets `BPC = TRUE`
/// on its own authority (`cmscnvrt.c`, DL-013/M2) *and* `cmsDetectBlackPoint`
/// returns the fixed v4 perceptual black — a real transform, not a no-op. At
/// media-relative neither fires, and clause 8.10.2's fallback sends both
/// implementations to the `A2B0`/`B2A0` tag data anyway (lcms2:
/// `if (!cmsIsTag(hProfile, tag16)) tag16 = Device2PCS16[0]`). The perceptual
/// case is measured and reported as [`MabAnalysis::forced_bpc_cost`] so the
/// confound has a number instead of a paragraph.
///
/// ## ★ And the measurement refines the recorded finding
///
/// DL-013 / corpus **M2** is recorded as *"lcms2 forces BPC on v4 profiles at
/// perceptual and saturation"*. Measured here **in both directions**, that is
/// half the rule. `_cmsLinkProfiles` sets `BPC[i]` per profile, but
/// `DefaultICCintents` consumes it as `ComputeConversion(i, …, BPC[i], …)`,
/// which is the conversion **from `hProfiles[i-1]` into `hProfiles[i]`** — so
/// the flag that decides is the **destination** profile's version, and a v4
/// *source* into a v2 destination never reads the flag that was set for it.
///
/// Measured, and it is not subtle: fixture → sRGB is **bit-identical** between
/// the two intents, while sRGB → fixture moves `K` at black from 99,6094 % to
/// 96,4721 %. Anyone applying M2 to decide whether a comparison is confounded
/// needs the direction, not just the version.
pub fn analyse_mab(oracle: &Oracle) -> Result<MabAnalysis, Unavailable> {
    let fx = fixture_path();
    if !fx.is_file() {
        return Err(Unavailable::Error(format!(
            "synthetic fixture missing: {} — this is category (a), it is committed, \
             so its absence is a repository error and not a skip",
            fx.display()
        )));
    }
    let srgb = Path::new(SRGB);
    let iccce = match Iccce::locate() {
        Err(e) => return Err(Unavailable::Error(e.to_string())),
        Ok(None) => {
            return Err(Unavailable::Skip(
                "iccce binary not found: run `cargo build --release -p iccce-cli`".to_string(),
            ));
        }
        Ok(Some(i)) => i,
    };
    let bytes = std::fs::read(&fx).map_err(|e| Unavailable::Error(e.to_string()))?;
    let profile = Profile::parse(&bytes).map_err(|e| Unavailable::Error(e.to_string()))?;
    let a2b = read_lut_ab(&profile, tag::A2B0)
        .ok_or_else(|| Unavailable::Error("fixture has no decodable mAB A2B0".into()))?;
    let b2a = read_lut_ab(&profile, tag::B2A0)
        .ok_or_else(|| Unavailable::Error("fixture has no decodable mBA B2A0".into()))?;
    let mab = LutAbModel::from_lut_ab(&a2b, PcsKind::Lab)
        .map_err(|e| Unavailable::Error(format!("mAB model: {e}")))?;
    let mba = LutAbModel::from_mba(&b2a, PcsKind::Lab)
        .map_err(|e| Unavailable::Error(format!("mBA model: {e}")))?;

    let structure = format!(
        "fixture v{:08X} {} {}->{} | A2B0 mAB {}->{} grid {:?} | B2A0 mBA {}->{} grid {:?}",
        profile.header.version.raw,
        profile.header.device_class,
        profile.header.color_space,
        profile.header.pcs,
        a2b.input_chan,
        a2b.output_chan,
        a2b.clut.as_ref().map(|c| c.grid_points[..4].to_vec()),
        b2a.input_chan,
        b2a.output_chan,
        b2a.clut.as_ref().map(|c| c.grid_points[..3].to_vec()),
    );

    // --- the affine claim, tested rather than asserted ---------------------
    // Both CLUTs are affine in one input and constant in the others, so both
    // geometries must reproduce them identically. If this is ever non-zero,
    // every "the method envelope is zero here" sentence in §15 is void.
    let mut scheme_envelope = 0.0f64;
    if let Some(c) = &a2b.clut {
        let dims: Vec<usize> = (0..usize::from(a2b.input_chan))
            .map(|i| usize::from(c.grid_points[i]))
            .collect();
        let data: Vec<f64> = match &c.samples {
            ClutSamples::U16(v) => v.iter().map(|&s| f64::from(s) / 65535.0).collect(),
            ClutSamples::U8(v) => v.iter().map(|&s| f64::from(s) / 255.0).collect(),
        };
        let clut = HarnessClut::new(dims, usize::from(a2b.output_chan), data);
        let mut a = vec![0.0; usize::from(a2b.output_chan)];
        let mut b = vec![0.0; usize::from(a2b.output_chan)];
        for q in cmyk_grid() {
            clut.eval(&q, Scheme::NLinear, &mut a);
            clut.eval(&q, Scheme::Lcms2Default, &mut b);
            for (x, y) in a.iter().zip(&b) {
                scheme_envelope = scheme_envelope.max((x - y).abs());
            }
        }
    }
    if let Some(c) = &b2a.clut {
        let dims: Vec<usize> = (0..usize::from(b2a.input_chan))
            .map(|i| usize::from(c.grid_points[i]))
            .collect();
        let data: Vec<f64> = match &c.samples {
            ClutSamples::U16(v) => v.iter().map(|&s| f64::from(s) / 65535.0).collect(),
            ClutSamples::U8(v) => v.iter().map(|&s| f64::from(s) / 255.0).collect(),
        };
        let clut = HarnessClut::new(dims, usize::from(b2a.output_chan), data);
        let mut a = vec![0.0; usize::from(b2a.output_chan)];
        let mut b = vec![0.0; usize::from(b2a.output_chan)];
        for l in lab_grid() {
            let enc = [
                (l.l / 100.0).clamp(0.0, 1.0),
                ((l.a + 128.0) / 255.0).clamp(0.0, 1.0),
                ((l.b + 128.0) / 255.0).clamp(0.0, 1.0),
            ];
            clut.eval(&enc, Scheme::NLinear, &mut a);
            clut.eval(&enc, Scheme::Lcms2Default, &mut b);
            for (x, y) in a.iter().zip(&b) {
                scheme_envelope = scheme_envelope.max((x - y).abs());
            }
        }
    }

    // --- mAB: device -> PCS ------------------------------------------------
    let cmyk = cmyk_grid();
    let req = Request {
        input: Space::profile(&fx),
        output: Space::lab_v4(),
        intent: Intent::RelativeColorimetric,
        precalc: Precalc::Exact,
        bpc: Bpc::Off,
        values: cmyk.iter().flat_map(|q| q.iter().map(|v| v * 100.0)).collect(),
    };
    let lcms2_lab = oracle.convert_batch_shaped(&req, 4, 3)?;

    let mut mab_iccce_vs_derived = Vec::new();
    let mut mab_lcms2_vs_derived = Vec::new();
    let mut mab_cross = Vec::new();
    let mut mab_overflows = Vec::new();
    for (i, q) in cmyk.iter().enumerate() {
        let mine = match mab.device_to_pcs(q) {
            Some(PcsValue::Lab(l)) => l,
            _ => return Err(Unavailable::Error("LutAbModel refused a CMYK grid point".into())),
        };
        let theirs = Lab {
            l: lcms2_lab[i][0],
            a: lcms2_lab[i][1],
            b: lcms2_lab[i][2],
        };
        mab_iccce_vs_derived.push(lab_dist(mine, expected_mab_lab(*q, true)));
        mab_lcms2_vs_derived.push(lab_dist(theirs, expected_mab_lab(*q, false)));
        mab_cross.push(lab_dist(mine, theirs));
        // The overflow set: where the encoded L* leaves [0,1] after the matrix.
        mab_overflows.push((1.0 - q[3]) + 1.0 / 256.0 > 1.0);
    }

    // --- mBA: PCS -> device ------------------------------------------------
    let labs = lab_grid();
    let req = Request {
        input: Space::lab_v4(),
        output: Space::profile(&fx),
        intent: Intent::RelativeColorimetric,
        precalc: Precalc::Exact,
        bpc: Bpc::Off,
        values: labs.iter().flat_map(|l| [l.l, l.a, l.b]).collect(),
    };
    let lcms2_cmyk = oracle.convert_batch_shaped(&req, 3, 4)?;
    let mut mba_iccce_vs_derived = Vec::new();
    let mut mba_lcms2_vs_derived = Vec::new();
    for (i, l) in labs.iter().enumerate() {
        let want = expected_mba_cmyk(*l);
        let mine = mba
            .pcs_to_device(PcsValue::Lab(*l))
            .ok_or_else(|| Unavailable::Error("LutAbModel refused a Lab grid point".into()))?;
        mba_iccce_vs_derived.push(
            mine.iter()
                .zip(&want)
                .map(|(a, b)| (a - b).abs())
                .fold(0.0f64, f64::max),
        );
        mba_lcms2_vs_derived.push(
            lcms2_cmyk[i]
                .iter()
                .map(|v| v / 100.0)
                .zip(&want)
                .map(|(a, b)| (a - b).abs())
                .fold(0.0f64, f64::max),
        );
    }

    // --- end to end, through the shipped binary ----------------------------
    let srgb_present = srgb.is_file();
    let mut e2e_mba_device = Vec::new();
    let mut e2e_mab_device = Vec::new();
    let mut e2e_mab_de = Vec::new();
    let mut clamp_divergence = 0.0f64;
    let mut forced_bpc_cost = (0.0f64, 0.0f64);
    if srgb_present {
        let srgb_bytes = std::fs::read(srgb).map_err(|e| Unavailable::Error(e.to_string()))?;
        let srgb_profile =
            Profile::parse(&srgb_bytes).map_err(|e| Unavailable::Error(e.to_string()))?;
        let srgb_model = MatrixTrc::from_profile(&srgb_profile)
            .map_err(|e| Unavailable::Error(e.to_string()))?;

        // sRGB -> fixture: the mBA direction, shipped binary both sides.
        let rgb = rgb_grid();
        let rows: Vec<Vec<f64>> = rgb.iter().map(|t| t.to_vec()).collect();
        let mine =
            iccce.transform_rows_shaped(srgb, &fx, Intent::RelativeColorimetric, &rows, 4)?;
        let req = Request {
            input: Space::profile(srgb),
            output: Space::profile(&fx),
            intent: Intent::RelativeColorimetric,
            precalc: Precalc::Exact,
            bpc: Bpc::Off,
            values: rgb.iter().flat_map(|t| t.iter().map(|v| v * 255.0)).collect(),
        };
        let theirs = oracle.convert_batch_shaped(&req, 3, 4)?;
        for i in 0..rgb.len() {
            e2e_mba_device.push(
                mine[i]
                    .iter()
                    .zip(theirs[i].iter().map(|v| v / 100.0))
                    .map(|(a, b)| (a - b).abs())
                    .fold(0.0f64, f64::max),
            );
        }

        // fixture -> sRGB: the mAB direction. This is where the clamp
        // divergence shows up in a colour, so the ΔE is recorded here.
        let rows: Vec<Vec<f64>> = cmyk.iter().map(|q| q.to_vec()).collect();
        let mine = iccce.transform_rows_shaped(&fx, srgb, Intent::RelativeColorimetric, &rows, 3)?;
        let req = Request {
            input: Space::profile(&fx),
            output: Space::profile(srgb),
            intent: Intent::RelativeColorimetric,
            precalc: Precalc::Exact,
            bpc: Bpc::Off,
            values: cmyk.iter().flat_map(|q| q.iter().map(|v| v * 100.0)).collect(),
        };
        let theirs = oracle.convert_batch_shaped(&req, 4, 3)?;
        for i in 0..cmyk.len() {
            let a = [mine[i][0], mine[i][1], mine[i][2]];
            let b = [
                theirs[i][0] / 255.0,
                theirs[i][1] / 255.0,
                theirs[i][2] / 255.0,
            ];
            let dev = (0..3).map(|c| (a[c] - b[c]).abs()).fold(0.0f64, f64::max);
            let de = delta_e_2000(to_lab(&srgb_model, a), to_lab(&srgb_model, b));
            e2e_mab_device.push(dev);
            e2e_mab_de.push(de);
            if mab_overflows[i] {
                clamp_divergence = clamp_divergence.max(de);
            }
        }

        // ★ The forced-BPC measurement, BOTH directions. Both sides of each
        // comparison are lcms2: its own media-relative output against its own
        // perceptual output. The pair of numbers is the finding — see the
        // function doc.
        //
        // (a) the v4 profile as SOURCE, v2 destination.
        let req = Request {
            input: Space::profile(&fx),
            output: Space::profile(srgb),
            intent: Intent::Perceptual,
            precalc: Precalc::Exact,
            bpc: Bpc::Off,
            values: cmyk.iter().flat_map(|q| q.iter().map(|v| v * 100.0)).collect(),
        };
        let perceptual = oracle.convert_batch_shaped(&req, 4, 3)?;
        for i in 0..cmyk.len() {
            forced_bpc_cost.0 = forced_bpc_cost.0.max(
                (0..3)
                    .map(|c| (theirs[i][c] - perceptual[i][c]).abs() / 255.0)
                    .fold(0.0f64, f64::max),
            );
        }

        // (b) the v4 profile as DESTINATION, v2 source — the direction the
        // flag is actually read in.
        let base = Request {
            input: Space::profile(srgb),
            output: Space::profile(&fx),
            intent: Intent::RelativeColorimetric,
            precalc: Precalc::Exact,
            bpc: Bpc::Off,
            values: rgb.iter().flat_map(|t| t.iter().map(|v| v * 255.0)).collect(),
        };
        let rel_into = oracle.convert_batch_shaped(&base, 3, 4)?;
        let per_req = Request {
            intent: Intent::Perceptual,
            ..base
        };
        let per_into = oracle.convert_batch_shaped(&per_req, 3, 4)?;
        for i in 0..rgb.len() {
            forced_bpc_cost.1 = forced_bpc_cost.1.max(
                (0..4)
                    .map(|c| (rel_into[i][c] - per_into[i][c]).abs() / 100.0)
                    .fold(0.0f64, f64::max),
            );
        }
    }

    Ok(MabAnalysis {
        cmyk_grid: cmyk,
        lab_grid: labs,
        mab_iccce_vs_derived,
        mab_lcms2_vs_derived,
        mab_cross,
        mab_overflows,
        mba_iccce_vs_derived,
        mba_lcms2_vs_derived,
        e2e_mba_device,
        e2e_mab_device,
        e2e_mab_de,
        scheme_envelope,
        clamp_divergence,
        forced_bpc_cost,
        structure,
    })
}

// ===========================================================================
// §C — the analysis
// ===========================================================================

/// Everything §C measured.
#[derive(Debug)]
pub struct GrayAnalysis {
    pub axis: Vec<f64>,
    pub iccce_rgb: Vec<[f64; 3]>,
    pub lcms2_rgb_255: Vec<[f64; 3]>,
    /// The harness's model of lcms2: F.2 forward, then the destination with
    /// lcms2's 4096-entry reverse curve and its two roundings.
    pub modelled_rgb: Vec<[f64; 3]>,
    pub device_dev: Vec<f64>,
    pub device_dev_modelled: Vec<f64>,
    pub de: Vec<f64>,
    /// The envelope: iccce's exact destination vs the modelled one. No lcms2
    /// output in it.
    pub envelope: Vec<f64>,
    /// Perceptual against media-relative, both sides.
    pub intent_identity_iccce: f64,
    pub intent_identity_lcms2: f64,
    pub structure: String,
}

/// Run §C: `ewgray22.icm → sRGB` over the gray axis, media-relative and
/// perceptual.
pub fn analyse_gray(oracle: &Oracle) -> Result<GrayAnalysis, Unavailable> {
    let src_path = Path::new(GRAY);
    let dst_path = Path::new(SRGB);
    for p in [src_path, dst_path] {
        if !p.is_file() {
            return Err(Unavailable::Skip(format!(
                "profile not present on this machine: {} (LEGAL.md §3 category (c))",
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
    let src_bytes = std::fs::read(src_path).map_err(|e| Unavailable::Error(e.to_string()))?;
    let dst_bytes = std::fs::read(dst_path).map_err(|e| Unavailable::Error(e.to_string()))?;
    let src = Profile::parse(&src_bytes).map_err(|e| Unavailable::Error(e.to_string()))?;
    let dst = Profile::parse(&dst_bytes).map_err(|e| Unavailable::Error(e.to_string()))?;
    let gray = GrayTrc::from_profile(&src)
        .map_err(|e| Unavailable::Error(format!("no grayTRC model: {e}")))?;
    let dst_model = MatrixTrc::from_profile(&dst)
        .map_err(|e| Unavailable::Error(format!("destination has no matrix/TRC model: {e}")))?;

    // lcms2's destination: three 4096-entry reverse curves built from the
    // stored 1024-entry tables.
    let dst_inverse = dst_model
        .matrix()
        .inverse()
        .ok_or_else(|| Unavailable::Error("destination colorant matrix is singular".into()))?;
    let reverses: Vec<Option<ReverseCurve>> = (0..3)
        .map(|i| match &dst_model.trc[i] {
            iccce_cmm::curve::Trc::Table(t) => Some(ReverseCurve::build(t, 4096)),
            _ => None,
        })
        .collect();
    let tabulated = reverses.iter().all(Option::is_some);

    let structure = format!(
        "src v{:08X} {} {}->{} kTRC={} wtpt={:?} | dst v{:08X} TRC tabulated={} (lcms2 reverses \
         each with cmsReverseToneCurveEx(4096))",
        src.header.version.raw,
        src.header.device_class,
        src.header.color_space,
        src.header.pcs,
        match &dst_model.trc[0] {
            iccce_cmm::curve::Trc::Table(t) => format!("dst-table-{}", t.len()),
            _ => "dst-analytic".into(),
        },
        {
            const WTPT: Signature = Signature(0x7774_7074);
            src.tags
                .iter()
                .find(|t| t.sig == WTPT)
                .and_then(|e| match src.decode_tag(e) {
                    Some(Ok(d)) => match d.data {
                        TagData::Xyz(v) if v.len() == 1 => {
                            Some((v[0].x.to_f64(), v[0].y.to_f64(), v[0].z.to_f64()))
                        }
                        _ => None,
                    },
                    _ => None,
                })
        },
        dst.header.version.raw,
        tabulated,
    );

    let axis = gray_axis();
    let rows: Vec<Vec<f64>> = axis.iter().map(|&g| vec![g]).collect();
    let iccce_rel =
        iccce.transform_rows(src_path, dst_path, Intent::RelativeColorimetric, &rows)?;
    let iccce_per = iccce.transform_rows(src_path, dst_path, Intent::Perceptual, &rows)?;

    let req = |intent: Intent| Request {
        input: Space::profile(src_path),
        output: Space::profile(dst_path),
        intent,
        precalc: Precalc::Exact,
        bpc: Bpc::Off,
        values: axis.iter().map(|g| g * 255.0).collect(),
    };
    let lcms2_rel = oracle.convert_batch_shaped(&req(Intent::RelativeColorimetric), 1, 3)?;
    let lcms2_per = oracle.convert_batch_shaped(&req(Intent::Perceptual), 1, 3)?;

    let intent_identity_iccce = iccce_rel
        .iter()
        .zip(&iccce_per)
        .flat_map(|(a, b)| (0..3).map(move |c| (a[c] - b[c]).abs()))
        .fold(0.0f64, f64::max);
    let intent_identity_lcms2 = lcms2_rel
        .iter()
        .zip(&lcms2_per)
        .flat_map(|(a, b)| (0..3).map(move |c| (a[c] - b[c]).abs() / 255.0))
        .fold(0.0f64, f64::max);

    let mut modelled_rgb = Vec::with_capacity(axis.len());
    let mut device_dev = Vec::with_capacity(axis.len());
    let mut device_dev_modelled = Vec::with_capacity(axis.len());
    let mut de = Vec::with_capacity(axis.len());
    let mut envelope = Vec::with_capacity(axis.len());
    let lcms2_rgb_255: Vec<[f64; 3]> = lcms2_rel.iter().map(|r| [r[0], r[1], r[2]]).collect();

    for (i, &g) in axis.iter().enumerate() {
        // F.2 forward — identical on both sides (same D50 literals, same
        // analytic gamma), so this is not a modelled quantity but a shared one.
        let xyz = gray.device_to_pcs(g);
        // iccce's destination, exactly.
        let exact = dst_model
            .pcs_to_device(xyz)
            .map_err(|e| Unavailable::Error(format!("destination refused a PCS value: {e}")))?;
        // lcms2's destination, modelled.
        let linear = dst_inverse.apply([xyz.x, xyz.y, xyz.z]);
        let modelled = if tabulated {
            let mut out = [0.0f64; 3];
            for c in 0..3 {
                out[c] = reverses[c]
                    .as_ref()
                    .expect("checked")
                    .eval(linear[c].clamp(0.0, 1.0));
            }
            out
        } else {
            exact
        };
        modelled_rgb.push(modelled);
        envelope.push((0..3).map(|c| (exact[c] - modelled[c]).abs()).fold(0.0f64, f64::max));
        let theirs = [
            lcms2_rgb_255[i][0] / 255.0,
            lcms2_rgb_255[i][1] / 255.0,
            lcms2_rgb_255[i][2] / 255.0,
        ];
        device_dev.push(
            (0..3)
                .map(|c| (iccce_rel[i][c] - theirs[c]).abs())
                .fold(0.0f64, f64::max),
        );
        device_dev_modelled.push(
            (0..3)
                .map(|c| (modelled[c] - theirs[c]).abs())
                .fold(0.0f64, f64::max),
        );
        de.push(delta_e_2000(
            to_lab(&dst_model, iccce_rel[i]),
            to_lab(&dst_model, theirs),
        ));
    }

    Ok(GrayAnalysis {
        axis,
        iccce_rgb: iccce_rel,
        lcms2_rgb_255,
        modelled_rgb,
        device_dev,
        device_dev_modelled,
        de,
        envelope,
        intent_identity_iccce,
        intent_identity_lcms2,
        structure,
    })
}

// ===========================================================================
// Records
// ===========================================================================

fn slug(i: Intent) -> &'static str {
    match i {
        Intent::Perceptual => "perceptual",
        Intent::RelativeColorimetric => "media-relative",
        Intent::Saturation => "saturation",
        Intent::AbsoluteColorimetric => "icc-absolute",
    }
}

/// §A's records.
#[must_use]
pub fn b2a_records(a: &B2aAnalysis) -> Vec<Record> {
    let mut out = Vec::new();
    let both = format!(
        "sRGB -> USWebCoatedSWOP B2A (mft1/lut8, 3->4, 33 pts), {} RGB points; {}",
        a.rgb_grid.len(),
        a.structure
    );
    out.push(Record::graded(
        "pass4b/srgb-to-swop/b2a-tags-are-three-distinct-tables",
        Kind::SelfConsistency,
        Metric::AbsMaxComponent,
        TAGS_ARE_DISTINCT,
        a.tag_distinctness.identical_pairs(),
        "raw file bytes, no parser: the count of byte-identical pairs among B2A0/B2A1/B2A2. \
         The PRECONDITION for reading the three intent runs below as three measurements",
        format!(
            "{} | least-distinct pair differs in {:.1}% of its bytes | \
             CONTRAST: in the A2B direction of this same file A2B0 and A2B2 are ONE block at \
             ONE offset, which is why pass4/swop/perceptual-equals-saturation is graded at \
             exactly zero",
            a.tag_distinctness.describe(),
            a.tag_distinctness.least_distinct_fraction() * 100.0
        ),
    ));
    for r in &a.runs {
        let s = slug(r.intent);
        let (dev_max, dev_mean) = max_mean(&r.device_dev);
        let (mod_max, _) = max_mean(&r.device_dev_modelled);
        let (env_max, env_mean) = max_mean(&r.envelope);
        let (de_max, de_mean) = max_mean(&r.de_roundtrip);
        let (cf_max, _) = max_mean(&r.counterfactual);

        out.push(Record::graded(
            format!("pass4b/srgb-to-swop/{s}/apparatus-lut8-matches-iccce-cmm"),
            Kind::SelfConsistency,
            Metric::AbsMaxComponent,
            APPARATUS_B2A,
            r.apparatus,
            "both sides computed in this run: the harness's lut8 pipeline and \
             iccce-cmm's Lut16Model::pcs_to_device, same Lab inputs",
            format!("{both} | intent={}", r.intent.name()),
        ));
        out.push(Record::graded(
            format!("pass4b/srgb-to-swop/{s}/device-vs-lcms2"),
            Kind::CrossCheck,
            Metric::DeviceAbsMaxNormalised,
            DEVICE_B2A,
            dev_max,
            "both sides computed in this run: shipped `iccce transform` vs transicc -c0",
            format!(
                "{both} | intent={} | envelope(max/mean)={env_max:.4e}/{env_mean:.4e} | \
                 counterfactual if lcms2 had NOT forced trilinear={cf_max:.4e} | \
                 CLUT max adjacent step={:.4}",
                r.intent.name(),
                a.max_step
            ),
        ));
        out.push(Record::graded(
            format!("pass4b/srgb-to-swop/{s}/device-mean"),
            Kind::CrossCheck,
            Metric::DeviceAbsMeanNormalised,
            REPORTED,
            dev_mean,
            "both sides computed in this run",
            format!("{both} | intent={}", r.intent.name()),
        ));
        out.push(Record::graded(
            format!("pass4b/srgb-to-swop/{s}/device-lcms2-arithmetic-modelled"),
            Kind::CrossCheck,
            Metric::DeviceAbsMaxNormalised,
            DEVICE_B2A_MODELLED,
            mod_max,
            "the harness's model of lcms2's own arithmetic (its roundings, read at pin \
             21c582a) against transicc's actual output",
            format!(
                "{both} | intent={} | unmodelled residual was {dev_max:.4e}",
                r.intent.name()
            ),
        ));
        out.push(Record::graded(
            format!("pass4b/srgb-to-swop/{s}/roundtrip-lab-de2000"),
            Kind::CrossCheck,
            Metric::DeltaE2000Max,
            DE_B2A_ROUNDTRIP,
            de_max,
            "both sides' CMYK carried back through the SAME file's A2B1 (not a second \
             opinion - the same table's forward direction) and compared in D50 CIELAB",
            format!("{both} | intent={} | mean={de_mean:.4e}", r.intent.name()),
        ));
        out.push(Record::graded(
            format!("pass4b/srgb-to-swop/{s}/counterfactual-tetrahedral"),
            Kind::CrossCheck,
            Metric::DeviceAbsMaxNormalised,
            REPORTED,
            cf_max,
            "computed from the B2A table and the two geometries alone - no lcms2 output. \
             What the disagreement WOULD have been had _cmsReadOutputLUT not forced \
             trilinear for a Lab-PCS LUT",
            format!(
                "{both} | intent={} | ratio to the observed residual: {:.0}x",
                r.intent.name(),
                if dev_max > 0.0 { cf_max / dev_max } else { f64::NAN }
            ),
        ));
    }
    let (pcs_max, pcs_mean) = max_mean(&a.pcs_device_dev);
    let (pcs_mod, _) = max_mean(&a.pcs_device_dev_modelled);
    let (pcs_cf, _) = max_mean(&a.pcs_counterfactual);
    out.push(Record::graded(
        "pass4b/lab-to-swop/media-relative/pcs-device-vs-lcms2",
        Kind::CrossCheck,
        Metric::DeviceAbsMaxNormalised,
        DEVICE_B2A,
        pcs_max,
        "iccce IN-PROCESS (Lut16Model::pcs_to_device - the shipped CLI has no Lab entry \
         point, so this grades the MODEL, not the binary) against transicc -i*Lab4",
        format!(
            "Lab -> USWebCoatedSWOP B2A1, {} Lab points, source model entirely out of the \
             picture | mean={pcs_mean:.4e} | modelled residual={pcs_mod:.4e} | \
             counterfactual tetrahedral={pcs_cf:.4e}",
            a.lab_grid.len()
        ),
    ));
    out.push(Record::graded(
        "pass4b/lab-to-swop/media-relative/pcs-device-lcms2-arithmetic-modelled",
        Kind::CrossCheck,
        Metric::DeviceAbsMaxNormalised,
        DEVICE_B2A_MODELLED,
        pcs_mod,
        "the harness's model of lcms2's arithmetic against transicc's actual output, with \
         no source profile in the chain at all",
        format!("Lab -> USWebCoatedSWOP B2A1, {} Lab points", a.lab_grid.len()),
    ));
    out
}

/// §B's records.
#[must_use]
pub fn mab_records(m: &MabAnalysis) -> Vec<Record> {
    let mut out = Vec::new();
    let ctx = format!(
        "{} | {} CMYK points, {} Lab points",
        m.structure,
        m.cmyk_grid.len(),
        m.lab_grid.len()
    );

    // The affine claim, which everything else in §B rests on.
    out.push(Record::graded(
        "pass4b/fixture/clut-is-affine-both-geometries-agree",
        Kind::SelfConsistency,
        Metric::AbsMaxComponent,
        Tolerance::new(
            1e-14,
            "both fixture CLUTs store a function affine in one input and constant in the \
             others, and every interpolation geometry reproduces an affine function exactly IN \
             EXACT ARITHMETIC - but the two algorithms reach that value by different sequences \
             of f64 operations, so they agree to rounding, not bit-identically. The n-linear arm \
             sums 2^4 = 16 products of values in [0,1], so ~16 ulp = 3.6e-15; 1e-14 is ~3x that \
             and 11 orders below one u16 lsb, so it remains the precondition for the \
             derived-expectation rows. The first draft said 0.0 and failed at 1.1e-16: a wrong \
             justification (real arithmetic mistaken for floating point), not a number that \
             needed room",
        ),
        m.scheme_envelope,
        "computed from the fixture's own CLUT samples and the two geometries alone",
        ctx.clone(),
    ));

    // Derived expectations, both implementations.
    let keep_no_overflow: Vec<bool> = m.mab_overflows.iter().map(|o| !o).collect();
    out.push(Record::graded(
        "pass4b/fixture/mab/iccce-vs-derived-expectation",
        Kind::DerivedExpectation,
        Metric::AbsMaxComponent,
        DERIVED_EXACT,
        max_at(&m.mab_iccce_vs_derived, &keep_no_overflow),
        "closed form derived from ICC.1:2022 10.12.1 (element order), 10.12.5 (the 3x4 \
         matrix and its offsets) and 6.3.4.2 Tables 12/13 (the GENERAL 16-bit PCSLAB \
         encoding - mAB is not in NOTE 3's legacy set), plus the fixture's own stored nodes. \
         No implementation's output enters it",
        format!(
            "{ctx} | iccce IN-PROCESS (LutAbModel) | L*/a*/b* units | EXCLUDES the {} \
             encoded-PCS-overflow points (see the clamp record)",
            m.mab_overflows.iter().filter(|o| **o).count()
        ),
    ));
    out.push(Record::graded(
        "pass4b/fixture/mab/lcms2-vs-derived-expectation",
        Kind::DerivedExpectation,
        Metric::AbsMaxComponent,
        DERIVED_LCMS2,
        max_mean(&m.mab_lcms2_vs_derived).0,
        "the same closed form against transicc - the THIRD reading, which is what stops the \
         fixture and the derivation being wrong together",
        format!(
            "{ctx} | unclamped reading of the overflow points, which is what lcms2 computes \
             | L*/a*/b* units"
        ),
    ));
    out.push(Record::graded(
        "pass4b/fixture/mba/iccce-vs-derived-expectation",
        Kind::DerivedExpectation,
        Metric::AbsMaxComponent,
        DERIVED_EXACT,
        max_mean(&m.mba_iccce_vs_derived).0,
        "closed form derived from ICC.1:2022 10.13.1 (element order, the mirror), 10.13.4 \
         (matrix) and Tables 12/13, plus the fixture's own stored nodes INCLUDING the u16 \
         rounding of the middle node (32768, not 32767.5) - an idealised 1-L would be wrong \
         by 7.6e-6 and would look like an implementation defect",
        format!("{ctx} | iccce IN-PROCESS (LutAbModel) | device units 0..1"),
    ));
    out.push(Record::graded(
        "pass4b/fixture/mba/lcms2-vs-derived-expectation",
        Kind::DerivedExpectation,
        Metric::AbsMaxComponent,
        DERIVED_LCMS2_DEVICE,
        max_mean(&m.mba_lcms2_vs_derived).0,
        "the same closed form against transicc - the third reading, in the device units it is \
         measured in",
        format!("{ctx} | device units 0..1"),
    ));

    // The cross-checks, end to end through the shipped binary.
    if !m.e2e_mba_device.is_empty() {
        let (mx, mean) = max_mean(&m.e2e_mba_device);
        out.push(Record::graded(
            "pass4b/srgb-to-fixture/media-relative/device-vs-lcms2",
            Kind::CrossCheck,
            Metric::DeviceAbsMaxNormalised,
            DEVICE_MAB_CROSSCHECK,
            mx,
            "both sides computed in this run: shipped `iccce transform` vs transicc -c0",
            format!("{ctx} | the mBA direction, end to end | mean={mean:.4e}"),
        ));
    }
    if !m.e2e_mab_device.is_empty() {
        let keep = keep_no_overflow.clone();
        let mx = max_at(&m.e2e_mab_device, &keep);
        out.push(Record::graded(
            "pass4b/fixture-to-srgb/media-relative/device-vs-lcms2",
            Kind::CrossCheck,
            Metric::DeviceAbsMaxNormalised,
            DEVICE_MAB_TO_SRGB,
            mx,
            "both sides computed in this run: shipped `iccce transform` vs transicc -c0",
            format!(
                "{ctx} | the mAB direction, end to end | EXCLUDES the {} \
                 encoded-PCS-overflow points",
                m.mab_overflows.iter().filter(|o| **o).count()
            ),
        ));
        out.push(Record::graded(
            "pass4b/fixture/mab/encoded-pcs-overflow-divergence",
            Kind::CrossCheck,
            Metric::DeltaE2000Max,
            REPORTED,
            m.clamp_divergence,
            "both sides computed in this run; NOT GRADED because which behaviour the \
             specification requires is UNSETTLED - see README section 15.3, the question is \
             owed to icc-spec-librarian",
            format!(
                "{ctx} | iccce clamps the encoded PCS to [0,1] at the B curve (clause 10.18's \
                 domain, via Trc::eval), lcms2 does not (its identity curve is an analytic \
                 gamma-1 segment, evaluated unbounded), so L* = 100 vs 100.390625 wherever \
                 the 3x4 matrix's +1/256 offset pushes past full scale"
            ),
        ));
    }
    if !m.e2e_mab_device.is_empty() {
        out.push(Record::graded(
            "pass4b/fixture/forced-bpc-is-decided-by-the-DESTINATION-version",
            Kind::OracleReproducibility,
            Metric::DeviceAbsMaxNormalised,
            REPORTED,
            m.forced_bpc_cost.0.max(m.forced_bpc_cost.1),
            "BOTH SIDES ARE lcms2: its own media-relative output against its own perceptual \
             output, on the same pair of profiles, in each direction. Says nothing whatever \
             about iccce - it is the size of the DL-013 / corpus M2 confound, measured rather \
             than assumed, and it refines M2",
            format!(
                "{ctx} | v4 fixture as SOURCE into a v2 destination: {:.4e} (bit-identical - \
                 the forced flag is never read) | v2 source into the v4 fixture as \
                 DESTINATION: {:.4e} | mechanism: _cmsLinkProfiles sets BPC[i] per profile but \
                 DefaultICCintents consumes it as ComputeConversion(i, .., BPC[i], ..), the \
                 conversion INTO hProfiles[i], so the DESTINATION profile's version decides",
                m.forced_bpc_cost.0, m.forced_bpc_cost.1
            ),
        ));
    }
    out
}

/// §C's records.
#[must_use]
pub fn gray_records(g: &GrayAnalysis) -> Vec<Record> {
    let (dev_max, dev_mean) = max_mean(&g.device_dev);
    let (mod_max, _) = max_mean(&g.device_dev_modelled);
    let (de_max, de_mean) = max_mean(&g.de);
    let (env_max, env_mean) = max_mean(&g.envelope);
    let ctx = format!(
        "ewgray22.icm -> sRGB, {} points on the gray axis | {}",
        g.axis.len(),
        g.structure
    );
    vec![
        Record::graded(
            "pass4b/gray-to-srgb/media-relative/device-vs-lcms2",
            Kind::CrossCheck,
            Metric::DeviceAbsMaxNormalised,
            DEVICE_GRAY,
            dev_max,
            "both sides computed in this run: shipped `iccce transform` vs transicc -c0",
            format!("{ctx} | envelope(max/mean)={env_max:.4e}/{env_mean:.4e}"),
        ),
        Record::graded(
            "pass4b/gray-to-srgb/media-relative/device-mean",
            Kind::CrossCheck,
            Metric::DeviceAbsMeanNormalised,
            REPORTED,
            dev_mean,
            "both sides computed in this run",
            ctx.clone(),
        ),
        Record::graded(
            "pass4b/gray-to-srgb/media-relative/de2000-vs-lcms2",
            Kind::CrossCheck,
            Metric::DeltaE2000Max,
            DE_GRAY,
            de_max,
            "both sides carried into D50 CIELAB through the destination's own model",
            format!("{ctx} | mean={de_mean:.4e}"),
        ),
        Record::graded(
            "pass4b/gray-to-srgb/media-relative/device-lcms2-arithmetic-modelled",
            Kind::CrossCheck,
            Metric::DeviceAbsMaxNormalised,
            DEVICE_GRAY_MODELLED,
            mod_max,
            "the harness's reimplementation of cmsReverseToneCurveEx(4096) plus the float \
             path's two 1/65535 roundings, against transicc's actual output",
            format!("{ctx} | unmodelled residual was {dev_max:.4e}"),
        ),
        Record::graded(
            "pass4b/gray-to-srgb/perceptual-equals-media-relative",
            Kind::CrossCheck,
            Metric::DeviceAbsMaxNormalised,
            GRAY_INTENT_IDENTITY,
            g.intent_identity_iccce.max(g.intent_identity_lcms2),
            "both sides computed in this run; graded on the larger of the two",
            format!(
                "{ctx} | iccce={:.4e} lcms2={:.4e}",
                g.intent_identity_iccce, g.intent_identity_lcms2
            ),
        ),
    ]
}

/// The ids, kinds, metrics and tolerances of every Pass 4b record, for the
/// skip/error path.
///
/// A suite that emits nothing when it cannot run is indistinguishable, in a
/// log, from one that was never wired up — so the shape of the report is the
/// same on a machine with no colour directory, with every line marked SKIP and
/// carrying its reason.
#[must_use]
pub fn unavailable_records(u: &Unavailable, section: &str) -> Vec<Record> {
    let reason = u.to_string();
    let mut specs: Vec<(String, Kind, Metric, Tolerance)> = Vec::new();
    if section == "b2a" {
        specs.push((
            "pass4b/srgb-to-swop/b2a-tags-are-three-distinct-tables".to_string(),
            Kind::SelfConsistency,
            Metric::AbsMaxComponent,
            TAGS_ARE_DISTINCT,
        ));
        for s in ["perceptual", "media-relative", "saturation"] {
            specs.push((
                format!("pass4b/srgb-to-swop/{s}/apparatus-lut8-matches-iccce-cmm"),
                Kind::SelfConsistency,
                Metric::AbsMaxComponent,
                APPARATUS_B2A,
            ));
            specs.push((
                format!("pass4b/srgb-to-swop/{s}/device-vs-lcms2"),
                Kind::CrossCheck,
                Metric::DeviceAbsMaxNormalised,
                DEVICE_B2A,
            ));
            specs.push((
                format!("pass4b/srgb-to-swop/{s}/device-mean"),
                Kind::CrossCheck,
                Metric::DeviceAbsMeanNormalised,
                REPORTED,
            ));
            specs.push((
                format!("pass4b/srgb-to-swop/{s}/device-lcms2-arithmetic-modelled"),
                Kind::CrossCheck,
                Metric::DeviceAbsMaxNormalised,
                DEVICE_B2A_MODELLED,
            ));
            specs.push((
                format!("pass4b/srgb-to-swop/{s}/roundtrip-lab-de2000"),
                Kind::CrossCheck,
                Metric::DeltaE2000Max,
                DE_B2A_ROUNDTRIP,
            ));
            specs.push((
                format!("pass4b/srgb-to-swop/{s}/counterfactual-tetrahedral"),
                Kind::CrossCheck,
                Metric::DeviceAbsMaxNormalised,
                REPORTED,
            ));
        }
        specs.push((
            "pass4b/lab-to-swop/media-relative/pcs-device-vs-lcms2".into(),
            Kind::CrossCheck,
            Metric::DeviceAbsMaxNormalised,
            DEVICE_B2A,
        ));
        specs.push((
            "pass4b/lab-to-swop/media-relative/pcs-device-lcms2-arithmetic-modelled".into(),
            Kind::CrossCheck,
            Metric::DeviceAbsMaxNormalised,
            DEVICE_B2A_MODELLED,
        ));
    } else if section == "mab" {
        specs.push((
            "pass4b/fixture/clut-is-affine-both-geometries-agree".into(),
            Kind::SelfConsistency,
            Metric::AbsMaxComponent,
            Tolerance::new(1e-14, "affine CLUT: every geometry agrees to f64 rounding"),
        ));
        specs.push((
            "pass4b/fixture/mab/iccce-vs-derived-expectation".into(),
            Kind::DerivedExpectation,
            Metric::AbsMaxComponent,
            DERIVED_EXACT,
        ));
        specs.push((
            "pass4b/fixture/mab/lcms2-vs-derived-expectation".into(),
            Kind::DerivedExpectation,
            Metric::AbsMaxComponent,
            DERIVED_LCMS2,
        ));
        specs.push((
            "pass4b/fixture/mba/iccce-vs-derived-expectation".into(),
            Kind::DerivedExpectation,
            Metric::AbsMaxComponent,
            DERIVED_EXACT,
        ));
        specs.push((
            "pass4b/fixture/mba/lcms2-vs-derived-expectation".into(),
            Kind::DerivedExpectation,
            Metric::AbsMaxComponent,
            DERIVED_LCMS2_DEVICE,
        ));
        specs.push((
            "pass4b/srgb-to-fixture/media-relative/device-vs-lcms2".into(),
            Kind::CrossCheck,
            Metric::DeviceAbsMaxNormalised,
            DEVICE_MAB_CROSSCHECK,
        ));
        specs.push((
            "pass4b/fixture-to-srgb/media-relative/device-vs-lcms2".into(),
            Kind::CrossCheck,
            Metric::DeviceAbsMaxNormalised,
            DEVICE_MAB_TO_SRGB,
        ));
        specs.push((
            "pass4b/fixture/mab/encoded-pcs-overflow-divergence".into(),
            Kind::CrossCheck,
            Metric::DeltaE2000Max,
            REPORTED,
        ));
    } else {
        specs.push((
            "pass4b/gray-to-srgb/media-relative/device-vs-lcms2".into(),
            Kind::CrossCheck,
            Metric::DeviceAbsMaxNormalised,
            DEVICE_GRAY,
        ));
        specs.push((
            "pass4b/gray-to-srgb/media-relative/device-mean".into(),
            Kind::CrossCheck,
            Metric::DeviceAbsMeanNormalised,
            REPORTED,
        ));
        specs.push((
            "pass4b/gray-to-srgb/media-relative/de2000-vs-lcms2".into(),
            Kind::CrossCheck,
            Metric::DeltaE2000Max,
            DE_GRAY,
        ));
        specs.push((
            "pass4b/gray-to-srgb/media-relative/device-lcms2-arithmetic-modelled".into(),
            Kind::CrossCheck,
            Metric::DeviceAbsMaxNormalised,
            DEVICE_GRAY_MODELLED,
        ));
        specs.push((
            "pass4b/gray-to-srgb/perceptual-equals-media-relative".into(),
            Kind::CrossCheck,
            Metric::DeviceAbsMaxNormalised,
            GRAY_INTENT_IDENTITY,
        ));
    }
    specs
        .into_iter()
        .map(|(id, kind, metric, tol)| match u {
            Unavailable::Skip(_) => {
                Record::skipped(id, kind, metric, tol, "not run on this machine", reason.clone())
            }
            Unavailable::Error(_) => {
                Record::errored(id, kind, metric, tol, "not run on this machine", reason.clone())
            }
        })
        .collect()
}

/// Everything Pass 4b produced, for the report binary.
pub struct Bundle {
    pub b2a: Option<B2aAnalysis>,
    pub mab: Option<MabAnalysis>,
    pub gray: Option<GrayAnalysis>,
}

/// Run all three sections. Each is independent: a missing SWOP does not stop
/// the synthetic fixture from being measured, which is the whole point of
/// having a category (a) fixture.
#[must_use]
pub fn run(oracle: &Oracle) -> (Bundle, Vec<Record>) {
    let mut records = Vec::new();
    let b2a = match analyse_b2a(oracle) {
        Ok(a) => {
            records.extend(b2a_records(&a));
            Some(a)
        }
        Err(u) => {
            records.extend(unavailable_records(&u, "b2a"));
            None
        }
    };
    let mab = match analyse_mab(oracle) {
        Ok(a) => {
            records.extend(mab_records(&a));
            Some(a)
        }
        Err(u) => {
            records.extend(unavailable_records(&u, "mab"));
            None
        }
    };
    let gray = match analyse_gray(oracle) {
        Ok(a) => {
            records.extend(gray_records(&a));
            Some(a)
        }
        Err(u) => {
            records.extend(unavailable_records(&u, "gray"));
            None
        }
    };
    (Bundle { b2a, mab, gray }, records)
}

// ===========================================================================
// Tests — of the apparatus, not of any colour
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grids_are_deterministic() {
        assert_eq!(rgb_grid(), rgb_grid());
        assert_eq!(cmyk_grid(), cmyk_grid());
        assert_eq!(gray_axis(), gray_axis());
        let a = lab_grid();
        let b = lab_grid();
        assert_eq!(a.len(), b.len());
        for (x, y) in a.iter().zip(&b) {
            assert!((x.l - y.l).abs() < f64::EPSILON && (x.a - y.a).abs() < f64::EPSILON);
        }
    }

    #[test]
    fn grids_are_in_range() {
        for t in rgb_grid() {
            for c in t {
                assert!((0.0..=1.0).contains(&c), "rgb out of range: {t:?}");
            }
        }
        for q in cmyk_grid() {
            for c in q {
                assert!((0.0..=1.0).contains(&c), "cmyk out of range: {q:?}");
            }
        }
        for l in lab_grid() {
            assert!((0.0..=100.0).contains(&l.l), "L* out of range: {l:?}");
            assert!(l.a.abs() <= 128.0 && l.b.abs() <= 128.0, "ab out of range: {l:?}");
        }
    }

    /// The two geometries must agree exactly on a **separable affine** table,
    /// and disagree on a curved one. The first half is what §B's derived
    /// expectations rest on; the second is what makes the counterfactual arm
    /// of §A meaningful rather than a duplicate of the n-linear one.
    ///
    /// This is the test that caught the index-convention mixing in
    /// `pass4`'s first draft, transplanted to the ragged case.
    #[test]
    fn schemes_agree_on_affine_and_differ_on_curved() {
        let dims = vec![5usize, 4, 3];
        let mut affine = Vec::new();
        let mut curved = Vec::new();
        for i in 0..dims[0] {
            for j in 0..dims[1] {
                for k in 0..dims[2] {
                    #[allow(clippy::cast_precision_loss)]
                    let (x, y, z) = (
                        i as f64 / (dims[0] - 1) as f64,
                        j as f64 / (dims[1] - 1) as f64,
                        k as f64 / (dims[2] - 1) as f64,
                    );
                    affine.push(0.1 + 0.2 * x + 0.3 * y + 0.4 * z);
                    curved.push(x * y * z);
                }
            }
        }
        let a = HarnessClut::new(dims.clone(), 1, affine);
        let c = HarnessClut::new(dims, 1, curved);
        let mut worst_affine = 0.0f64;
        let mut worst_curved = 0.0f64;
        let mut o1 = [0.0];
        let mut o2 = [0.0];
        for i in 0..=10 {
            for j in 0..=10 {
                for k in 0..=10 {
                    let p = [
                        f64::from(i) / 10.0,
                        f64::from(j) / 10.0,
                        f64::from(k) / 10.0,
                    ];
                    a.eval(&p, Scheme::NLinear, &mut o1);
                    a.eval(&p, Scheme::Lcms2Default, &mut o2);
                    worst_affine = worst_affine.max((o1[0] - o2[0]).abs());
                    c.eval(&p, Scheme::NLinear, &mut o1);
                    c.eval(&p, Scheme::Lcms2Default, &mut o2);
                    worst_curved = worst_curved.max((o1[0] - o2[0]).abs());
                }
            }
        }
        assert!(
            worst_affine < 1e-12,
            "the two geometries must be identical on an affine table, got {worst_affine}"
        );
        assert!(
            worst_curved > 1e-3,
            "the two geometries must differ on a curved table, or the counterfactual arm \
             is measuring nothing: got {worst_curved}"
        );
    }

    /// The closed forms must not depend on the channels the fixture's CLUTs
    /// are constant in — the property the grids are wide in those channels to
    /// confirm, asserted here so a future edit to the fixture breaks a test
    /// rather than a claim.
    #[test]
    fn derived_expectations_ignore_the_constant_channels() {
        let a = expected_mab_lab([0.0, 0.0, 0.0, 0.4], true);
        let b = expected_mab_lab([1.0, 0.7, 0.2, 0.4], true);
        assert!((a.l - b.l).abs() < 1e-15 && (a.a - b.a).abs() < 1e-15);
        let c = expected_mba_cmyk(Lab { l: 40.0, a: 0.0, b: 0.0 });
        let d = expected_mba_cmyk(Lab {
            l: 40.0,
            a: 90.0,
            b: -70.0,
        });
        assert!((c[3] - d[3]).abs() < 1e-15);
    }

    /// The `mAB ` closed form at the two `K` nodes, written out longhand so a
    /// reader can check the arithmetic against the clause text without running
    /// anything.
    #[test]
    fn derived_mab_matches_the_hand_derivation() {
        // K = 1: L* node is 0, plus the matrix offset 1/256 -> 0.390625.
        let k1 = expected_mab_lab([0.0, 0.0, 0.0, 1.0], true);
        assert!((k1.l - 0.390625).abs() < 1e-12, "L* {}", k1.l);
        assert!((k1.a - 1.9921875).abs() < 1e-12, "a* {}", k1.a);
        assert!((k1.b - 2.98828125).abs() < 1e-12, "b* {}", k1.b);
        // K = 0 unclamped: 100 + 0.390625. Clamped: exactly 100.
        let k0u = expected_mab_lab([0.0, 0.0, 0.0, 0.0], false);
        assert!((k0u.l - 100.390625).abs() < 1e-12, "L* {}", k0u.l);
        let k0c = expected_mab_lab([0.0, 0.0, 0.0, 0.0], true);
        assert!((k0c.l - 100.0).abs() < 1e-12, "L* {}", k0c.l);
    }

    /// `q16` must be lcms2's rounding, including at the boundaries.
    #[test]
    fn q16_rounds_to_16_bit_codes() {
        assert!((q16(0.0) - 0.0).abs() < f64::EPSILON);
        assert!((q16(1.0) - 1.0).abs() < f64::EPSILON);
        // 0.5 * 65535 = 32767.5 -> 32768 (round half away from zero).
        assert!((q16(0.5) - 32768.0 / 65535.0).abs() < 1e-15);
    }

    /// The reverse-curve reimplementation must invert an exactly-representable
    /// curve exactly. A gamma table would fold its own resampling error in and
    /// would not distinguish "wrong algorithm" from "expected error".
    #[test]
    fn reverse_curve_inverts_an_identity_table() {
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let fwd: Vec<u16> = (0..1024)
            .map(|i| (f64::from(i) / 1023.0 * 65535.0).round() as u16)
            .collect();
        let rev = ReverseCurve::build(&fwd, 4096);
        for i in 0..=20 {
            let x = f64::from(i) / 20.0;
            assert!(
                (rev.eval(x) - x).abs() < 1e-4,
                "identity table must reverse to itself: {x} -> {}",
                rev.eval(x)
            );
        }
    }
}
