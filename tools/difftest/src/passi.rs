//! # Pass I — ICC's **published** chromatic-adaptation matrix, graded cell by cell
//!
//! **Subject:** `iccce_color::adaptation_matrix(&BRADFORD, D65, D50)` — the
//! matrix every D65-referenced conversion in this library rests on — against
//! the nine cells ICC prints at fifteen decimal places in
//!
//! > ICC, *How to interpret the sRGB color space (specified in
//! > IEC 61966-2-1) for ICC profiles*, Jack Holm, **2015-04-27**, **§B.2**.
//! > Corpus file `ICC_Spec/icc/icc__s__srgb_for_icc_profiles.md`; source PDF
//! > `ICC_Spec/_sources/srgb_bt709/srgb_icc_specification_of_srgb_2015.pdf`.
//!
//! This is the document `ICC.1:2022` Annex E.4.2 points at when it says *"ICC
//! has published recommended values for this instance of the
//! chromaticAdaptationTag"*, and which this project recorded as **not
//! obtained** until the operator supplied it on 2026-08-17. It is the third
//! `published-ground-truth` subject in the repository (after NC-001/Sharma and
//! the sRGB colorants) and the **first for chromatic adaptation** — the error
//! class `RAG_PLAN.md` names as the canonical from-memory mistake.
//!
//! ## ★★★ What this section does NOT claim, stated before what it does
//!
//! **`ICC.1` mandates no chromatic-adaptation transform at all** (corpus
//! ambiguity **A29**; `iccce-color/src/adapt.rs` module header). A profile's
//! `chad` tag stores a *resulting matrix*, not a method, so when a profile has
//! no `chad` the CMM's choice of Bradford is **iccce policy, not a
//! requirement**. Every row below therefore grades exactly one sentence:
//!
//! > *iccce's Bradford-derived D65→D50 matrix agrees with the matrix ICC
//! > recommends, to the extent that the two constructions' published inputs
//! > entail.*
//!
//! It does **not** say "iccce's chromatic adaptation is correct". There is no
//! specification text against which that sentence could be graded, and any
//! reader who quotes these rows for it has been given the wrong number.
//!
//! ## ★★★ The residual is NOT zero, and the reason is two documented input
//! ## differences — one of which the commissioning brief did not contain
//!
//! The brief for this pass proposed deriving the bound from the
//! cone-matrix difference alone:
//!
//! - **ICC.1:2022 Annex E.3 Eq. (E.1) prints Bradford `M_A[0][0] = 0.8951`**,
//!   and that is what `iccce_color::BRADFORD` carries, because it is what the
//!   specification prints.
//! - **ICC's published `chad` was built with `0.8950`** — recovered by
//!   `icc-spec-librarian` by eigendecomposition (a von-Kries matrix
//!   `M_A⁻¹ · D · M_A` has the rows of `M_A` as its left eigenvectors) and
//!   confirmed by exact reconstruction: `0.8951` leaves `5.661e-6`, `0.8950`
//!   leaves `5.7e-16`. The tell was in the corpus for six days as the row-sum
//!   check — E.1's first row sums to `1.0001`, the recovered one to `1.0000`.
//!
//! That term is real and it is **`5.661342e-6`**. It is also **not the
//! dominant one**, and a bound derived from it would have failed this pass at
//! **7.4× its value**:
//!
//! - **ICC's `chad` adapts a *rounded* white.** `chad⁻¹ · D50` returns
//!   `(0.9505, 1.0000, 1.0890)` exactly — the white the same document states in
//!   §A.4 as `Xabs/Yabs/Zabs = 76.04/80/87.12`. **iccce's D65 is derived from
//!   BT.709-6 item 1.4's chromaticities** `(0.3127, 0.3290)`, which give
//!   `(0.950455927…, 1, 1.089057751…)`. The two differ by `−4.407e-5` in X and
//!   `+5.775e-5` in Z, and that difference propagates to **`4.453188e-5`** in
//!   the matrix — **7.9× the cone term.**
//!
//! The two terms **partially cancel**, and the exact-arithmetic prediction for
//! iccce as shipped is `4.164937e-5` at cell `(0,0)`. Every bound in §B is that
//! prediction, per cell, plus one numerical allowance.
//!
//! > **The generalisation, and it is the third instance of one failure shape in
//! > this crate** (Pass 4b `B6`, Pass G `SWEEP_DEVICE`, Pass H
//! > `SEVEN_CORNER`): *when a tolerance's derivation names only the components
//! > the row owns, the missing term is in a component it does not own.* Here
//! > the derivation named the cone matrix — which the row is *about* — and
//! > omitted the white point, which the row merely *uses*.
//!
//! ## The four matrices this module computes, and why all four exist
//!
//! | | cone `M_A[0][0]` | source white | what it is |
//! |---|---|---|---|
//! | **A** | `0.8951` | chromaticity-derived | **iccce as shipped** — the subject |
//! | **B** | `0.8951` | `0.9505/1/1.0890` | isolates the **cone** term |
//! | **C** | `0.8950` | `0.9505/1/1.0890` | **ICC's own construction** — must reproduce the published matrix |
//! | **D** | `0.8950` | chromaticity-derived | the **named rival**: iccce with ICC's cone cell |
//!
//! **C is the instrument check** (§A). If the harness's own CAT — its own
//! typed digits, its own adjugate inverse, no iccce code in the loop —
//! reproduces ICC's fifteen published decimals from ICC's own inputs, then the
//! harness's predictions for A, B and D are trustworthy, and the librarian's
//! `0.8950` finding has been reproduced by a second route in a second language.
//! If C ever fails, **nothing else in this module means anything**, which is
//! why it is graded first and at the tightest bound.
//!
//! ## Why there is no oracle in this file
//!
//! `lcms2` is not invoked, no profile is read, no fixture is resolved and no
//! environment variable is consulted. Pass I runs identically on a bare CI
//! machine. That is deliberate: **a ground-truth-shaped row must not be hostage
//! to an oracle** (`README.md` §21; the Pass 5c fixture rows record the same
//! rule). It is also why this is the only pass whose `run()` takes no
//! [`crate::Oracle`].
//!
//! ## What is in the loop, and what is therefore invisible here
//!
//! §A–§D call `iccce_color::adaptation_matrix` **in process**. §E calls
//! `iccce_cmm::builtin::srgb()`, which is the construction the shipped binary
//! actually uses. Neither runs the `iccce` executable, so **a wiring defect
//! between the CLI and the library is invisible to every row in this file** —
//! the Pass H lesson, applied in advance: *ask of every row not "what does it
//! measure" but "which layer is in the loop".* §E is the layer closest to the
//! product that this subject can reach without an oracle.

use crate::{Kind, Metric, Record, SepUnits, Separation, Tolerance};

// ===========================================================================
// Published constants — transcribed here, independently of `crates/`
// ===========================================================================
//
// ★ These digits are typed in THIS file on purpose. The row's power comes
// entirely from the prediction side being independent of the code under test:
// if the harness imported `iccce_color::BRADFORD` or `iccce_color::D50`, a
// corrupted constant in the library would move both sides together and every
// row here would stay green while the product went wrong. The cost of the
// independence is a second transcription that can itself be mistyped, which is
// what §A's structural checks are for.

/// ICC's recommended D65→D50 chromatic-adaptation matrix, §B.2, fifteen
/// decimals, row-major, applied to a column vector.
///
/// ★★★ **THREE MINUS SIGNS.** They are `U+F02D` (Symbol-font hyphen) in the
/// PDF; `pdftotext -layout` **drops all three and additionally scrambles the
/// rows**, returning an all-positive matrix that looks entirely plausible. The
/// corpus verified the signs at raw-codepoint level with `pypdf` and
/// independently by `pdfminer.six` glyph coordinates. If this matrix is ever
/// re-extracted, re-verify the signs before trusting the extraction.
pub const PUBLISHED_CHAD: [[f64; 3]; 3] = [
    [1.047_844_353_856_414, 0.022_898_981_050_086, -0.050_206_647_741_605],
    [0.029_549_007_606_644, 0.990_508_028_941_971, -0.017_074_711_360_960],
    [-0.009_250_984_365_223, 0.015_072_338_237_051, 0.751_717_835_079_977],
];

/// ICC's D50-adapted sRGB colorants, §B.2, fifteen decimals. Column 1 is
/// `rXYZ`, column 2 `gXYZ`, column 3 `bXYZ`.
///
/// Structural check ICC's own prose implies and the corpus verified in exact
/// rational arithmetic: the row sums are `0.964200009…/0.999999989…/
/// 0.824900079…`, i.e. D50 to `9.3e-9`. §E asserts that here too, because it
/// is the cheapest available guard against a mistyped or transposed cell.
pub const PUBLISHED_COLORANTS: [[f64; 3]; 3] = [
    [0.436_030_342_570_117, 0.385_101_860_087_134, 0.143_067_806_654_203],
    [0.222_438_466_210_245, 0.716_942_745_571_917, 0.060_618_777_416_563],
    [0.013_897_440_074_263, 0.097_076_381_494_207, 0.713_926_257_896_652],
];

/// Bradford as **ICC.1:2022 Annex E.3 Equation (E.1) prints it** — the variant
/// `iccce_color::BRADFORD` carries. First row sums to `1.0001`, which the
/// corpus records as real and not a transcription error.
const BRADFORD_E3: [[f64; 3]; 3] = [
    [0.8951, 0.2664, -0.1614],
    [-0.7502, 1.7135, 0.0367],
    [0.0389, -0.0685, 1.0296],
];

/// Bradford as **recovered from ICC's own published `chad`** — identical to
/// [`BRADFORD_E3`] except `M_A[0][0] = 0.8950`. First row sums to `1.0000`.
const BRADFORD_RECOVERED: [[f64; 3]; 3] = [
    [0.8950, 0.2664, -0.1614],
    [-0.7502, 1.7135, 0.0367],
    [0.0389, -0.0685, 1.0296],
];

/// The ICC PCS white, four figures, as both documents state it.
const WHITE_D50: [f64; 3] = [0.9642, 1.0, 0.8249];

/// The white ICC's `chad` actually adapts **from**: §A.4's `76.04/80` and
/// `87.12/80`, exact. Not the same number as the chromaticity-derived D65
/// below — §A.3 and §A.4 of the one document disagree by `4.6e-5` in X and
/// `5.8e-5` in Z, filed as `icc__ref__spec_defects.md` §24.3.
const WHITE_D65_ROUNDED: [f64; 3] = [0.9505, 1.0, 1.0890];

/// D65 chromaticity, **Rec. ITU-R BT.709-6 (06/2015) item 1.4** — the harness's
/// own transcription of the constant `iccce_color::D65_XY` also carries.
const D65_XY_PUBLISHED: (f64, f64) = (0.3127, 0.3290);

// ===========================================================================
// Predictions — exact rational arithmetic, computed OUTSIDE any implementation
// ===========================================================================
//
// ★ Every number in this block is the output of exact `fractions.Fraction`
// arithmetic over the published constants above, computed 2026-08-17 BEFORE
// this module was run for the first time, and recorded in `docs/TOLERANCES.md`
// §3.9 with its derivation. They are *authored derivation constants*, not
// measurements: the distinction matters because a measured number typed into a
// harness rots silently, while a derived one fails loudly the moment the
// derivation stops holding — which is exactly what §A.3 makes it do, by
// recomputing all of them in f64 and grading the agreement.

/// Signed per-cell prediction of `iccce − ICC published chad`, exact rational
/// arithmetic. Max `|cell|` is `4.164937e-5` at `(0,0)`.
const PREDICTED_CHAD_RESIDUAL: [[f64; 3]; 3] = [
    [4.164_936_613_631_601e-5, 1.978_412_469_348_972e-5, -9.447_570_128_005_676e-6],
    [3.277_489_135_939_422e-5, -2.451_045_142_242_036e-5, -3.996_343_522_679_527e-6],
    [-8.964_739_858_434_573e-7, 2.692_499_803_166_244e-7, -3.970_146_237_358_895e-5],
];

/// Signed per-cell prediction of
/// `iccce_cmm::builtin::srgb().matrix() − ICC published colorants`, exact
/// rational arithmetic. Max `|cell|` is `4.607402e-5 = 3.020 ULP` at `(1,0)` —
/// the figure `builtin.rs` declares as the model's named approximation.
const PREDICTED_COLORANT_RESIDUAL: [[f64; 3]; 3] = [
    [1.090_904_593_402_795e-5, 1.105_071_102_140_814e-5, -2.196_906_840_943_608e-5],
    [4.607_401_923_245_187e-5, -3.766_696_345_971_274e-5, -8.396_254_497_739_137e-6],
    [2.274_739_711_239_209e-5, -9.142_797_082_991_546e-6, -1.368_406_515_140_055e-5],
];

/// **The cone term, isolated:** max `|cell|` of (Bradford `0.8951` at ICC's own
/// rounded white) − (published `chad`). Exact: `5.661341564633735e-6`.
const CONE_TERM_EXACT: f64 = 5.661_341_564_633_735e-6;

/// **The white term, isolated:** max `|cell|` of (chromaticity D65) −
/// (rounded D65), same cone matrix. Exact: `4.453187573657197e-5`.
const WHITE_TERM_EXACT: f64 = 4.453_187_573_657_197e-5;

/// Distance between iccce's matrix and the **named rival** (variant D: iccce's
/// white with ICC's `0.8950` cone cell). Exact: `5.662962099557275e-6`. A
/// property of the published constants, not of any run — which is why the
/// separations below use [`Separation::against_distance`] and not
/// [`Separation::against`].
const RIVAL_CONE_DISTANCE: f64 = 5.662_962_099_557_275e-6;

/// **The row sums of ICC's published colorants miss D50 by `7.946512e-8`**, and
/// that is ICC's arithmetic, not a transcription error: their colorants are
/// `chad × inv(§A.7)`, §A.7 is printed to seven decimals, and the implied white
/// of `inv(§A.7)` therefore sits `1.060763e-7` above `1.0890` in Z. Applying the
/// published `chad` to that difference reproduces `7.946512e-8` to every digit.
/// The bound below is the next power of ten above it — stated in the unit of
/// §A.7's own print precision rather than fitted to the observation.
const COLORANT_ROW_SUM_BOUND: f64 = 1e-7;

/// One ULP of `s15Fixed16`, the encoding both the `chad` tag and the colorant
/// tags are written in. Used only to express results in the unit a profile
/// author cares about; nothing is graded in ULP.
const S15F16_ULP: f64 = 1.0 / 65536.0;

// ===========================================================================
// The one numerical allowance in this module
// ===========================================================================

/// **The f64 round-off allowance, `1e-12`, and it is the only slack in Pass I.**
///
/// Every bound below is `an exactly-derived prediction + F64_NOISE`. This
/// number therefore has to be defensible on its own, because it is the only
/// place a failing row could be made to pass.
///
/// **Derivation.** The computation being compared is: one 3×3 adjugate inverse
/// (nine 2×2 minors, one determinant, nine divisions), two 3×3 matrix products
/// (three-term dot products), and three divisions for the cone ratios — all at
/// magnitudes of order 1, with no cancellation of leading digits (the largest
/// intermediate is `1.7135`). A conservative worst case is `≈50` ulp of that
/// magnitude, `50 × 2.220e-16 × 1.72 ≈ 1.9e-14`. `1e-12` is **50× that**.
///
/// **Why it cannot hide what the section exists to detect.** The smallest
/// defect any row here is designed to see is the cone-cell substitution, worth
/// `5.663e-6`. The allowance is **`5.7e6×` smaller than that**. There is no
/// setting of this constant between `1e-14` and `1e-8` that changes any
/// verdict in this file, which is the property a numerical allowance should
/// have and a tuned tolerance never does.
///
/// **What would justify moving it:** a measured f64 residual above `1e-13` on
/// any platform, which would be a finding about floating-point accumulation and
/// would be recorded as one — *not* a licence to widen. The observed values are
/// emitted in every record's detail so that this can be checked, not assumed.
const F64_NOISE: f64 = 1e-12;

// ===========================================================================
// The harness's own arithmetic — deliberately not iccce's
// ===========================================================================

type Mat = [[f64; 3]; 3];

/// 3×3 inverse by adjugate over determinant. Written here rather than reused
/// from `iccce_color::Mat3` so that the prediction side of every comparison
/// contains none of the code under test.
///
/// Returns `None` on a singular matrix rather than producing infinities; every
/// call site in this module treats that as a harness bug and says so, because
/// none of the published matrices here is anywhere near singular.
fn inv3(a: &Mat) -> Option<Mat> {
    let det = a[0][0] * (a[1][1] * a[2][2] - a[1][2] * a[2][1])
        - a[0][1] * (a[1][0] * a[2][2] - a[1][2] * a[2][0])
        + a[0][2] * (a[1][0] * a[2][1] - a[1][1] * a[2][0]);
    if det == 0.0 || !det.is_finite() {
        return None;
    }
    let mut out = [[0.0f64; 3]; 3];
    for (i, row) in out.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            // Cofactor of (j,i) — transposed, which is what makes it the adjugate.
            let (r1, r2) = ((j + 1) % 3, (j + 2) % 3);
            let (c1, c2) = ((i + 1) % 3, (i + 2) % 3);
            *cell = (a[r1][c1] * a[r2][c2] - a[r1][c2] * a[r2][c1]) / det;
        }
    }
    Some(out)
}

fn mul3(a: &Mat, b: &Mat) -> Mat {
    let mut out = [[0.0f64; 3]; 3];
    for (i, row) in out.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            *cell = a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j];
        }
    }
    out
}

fn apply3(a: &Mat, v: [f64; 3]) -> [f64; 3] {
    let mut out = [0.0f64; 3];
    for (i, o) in out.iter_mut().enumerate() {
        *o = a[i][0] * v[0] + a[i][1] * v[1] + a[i][2] * v[2];
    }
    out
}

/// The von Kries construction, `M = M_A⁻¹ · D · M_A`, in the harness's own
/// code. The operand order is the trap this whole subject is famous for:
/// `M_A · D · M_A⁻¹` yields a nearly-right matrix with small off-diagonal sign
/// differences and no error anywhere.
fn cat(cone: &Mat, white_src: [f64; 3], white_dst: [f64; 3]) -> Option<Mat> {
    let cone_inv = inv3(cone)?;
    let s = apply3(cone, white_src);
    let d = apply3(cone, white_dst);
    if s.contains(&0.0) {
        return None;
    }
    let diag = [
        [d[0] / s[0], 0.0, 0.0],
        [0.0, d[1] / s[1], 0.0],
        [0.0, 0.0, d[2] / s[2]],
    ];
    Some(mul3(&cone_inv, &mul3(&diag, cone)))
}

/// xyY(Y = 1) → XYZ, the same three divisions `iccce_color::XyY::to_xyz`
/// performs. Present so that the harness can build the chromaticity-derived
/// D65 without calling the crate under test.
fn xy_to_xyz(xy: (f64, f64)) -> [f64; 3] {
    [xy.0 / xy.1, 1.0, (1.0 - xy.0 - xy.1) / xy.1]
}

fn sub3(a: &Mat, b: &Mat) -> Mat {
    let mut out = [[0.0f64; 3]; 3];
    for (i, row) in out.iter_mut().enumerate() {
        for (j, cell) in row.iter_mut().enumerate() {
            *cell = a[i][j] - b[i][j];
        }
    }
    out
}

/// Max `|cell|` and its `(row, col)`.
fn max_cell(a: &Mat) -> (f64, usize, usize) {
    let mut best = (0.0f64, 0usize, 0usize);
    for (i, row) in a.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if cell.abs() > best.0 {
                best = (cell.abs(), i, j);
            }
        }
    }
    best
}

/// `s15Fixed16` encoding, round-half-away-from-zero, as `iccce-profile` writes
/// it. Used only to count how many cells would land on different **bytes**.
#[expect(
    clippy::cast_possible_truncation,
    reason = "the clamp immediately before the cast bounds the value to i32's range, so the \
              cast cannot truncate; the lint cannot see through the clamp. Stated with expect \
              rather than allow so that it becomes a warning again if the clamp is ever removed."
)]
fn enc_s15f16(v: f64) -> i32 {
    // The values here are all well inside ±32767, so the cast cannot truncate
    // meaningfully; it is still written with an explicit clamp so that a future
    // caller with a wild value gets a saturated integer rather than a surprise.
    (v * 65536.0).round().clamp(-2_147_483_648.0, 2_147_483_647.0) as i32
}

// ===========================================================================
// Analysis
// ===========================================================================

/// Everything Pass I measures, computed once.
pub struct Analysis {
    /// **A** — iccce as shipped: `adaptation_matrix(&BRADFORD, D65, D50)`.
    pub iccce_chad: Mat,
    /// **B** — harness CAT, E.3 Bradford (`0.8951`) at ICC's rounded white.
    pub variant_cone: Mat,
    /// **C** — harness CAT, recovered Bradford (`0.8950`) at ICC's rounded
    /// white: ICC's own construction.
    pub variant_icc: Mat,
    /// **D** — harness CAT, recovered Bradford at iccce's chromaticity white:
    /// the named rival.
    pub variant_rival: Mat,
    /// The harness's f64 re-computation of variant A, used only to check the
    /// typed exact predictions have not gone stale.
    pub predicted_chad: Mat,
    /// `iccce_cmm::builtin::srgb().matrix()` — the shipped construction.
    pub shipped_colorants: Mat,
    /// How many of the nine `chad` cells encode to different `s15Fixed16`
    /// words, and the largest difference in LSBs.
    pub enc_cells_differing: usize,
    pub enc_max_lsb: i32,
    /// The same count for the E.3-recompute at ICC's *own* rounded white — the
    /// case the corpus describes as `0.371 ULP`.
    pub enc_cells_differing_cone_only: usize,
}

/// Compute every matrix this pass grades. Infallible in practice: all four
/// cone matrices are far from singular and no white is degenerate, so a `None`
/// here would be a mistyped constant in this file, and the expectation is
/// documented rather than silently defaulted.
///
/// # Panics
///
/// If any published constant in this file has been edited into a singular
/// matrix or a degenerate white. That is a corruption of the reference data,
/// not a runtime condition, and a panic at the point of corruption is far
/// better than a plausible-looking wrong matrix reaching a `Record`.
#[must_use]
pub fn analyse() -> Analysis {
    let d65_chroma = xy_to_xyz(D65_XY_PUBLISHED);

    // --- the subject: iccce's own code, its own constants -------------------
    let iccce_d65 = iccce_color::XyY {
        x: iccce_color::D65_XY.0,
        y: iccce_color::D65_XY.1,
        luma_y: 1.0,
    }
    .to_xyz()
    .expect("iccce_color::D65_XY is a valid chromaticity");
    let iccce_chad =
        iccce_color::adaptation_matrix(&iccce_color::BRADFORD, iccce_d65, iccce_color::D50)
            .expect("Bradford is invertible and neither white is degenerate")
            .rows;

    // --- the harness's four variants ---------------------------------------
    let variant_cone = cat(&BRADFORD_E3, WHITE_D65_ROUNDED, WHITE_D50)
        .expect("BRADFORD_E3 in this file is invertible");
    let variant_icc = cat(&BRADFORD_RECOVERED, WHITE_D65_ROUNDED, WHITE_D50)
        .expect("BRADFORD_RECOVERED in this file is invertible");
    let variant_rival = cat(&BRADFORD_RECOVERED, d65_chroma, WHITE_D50)
        .expect("BRADFORD_RECOVERED in this file is invertible");
    let predicted_chad =
        cat(&BRADFORD_E3, d65_chroma, WHITE_D50).expect("BRADFORD_E3 in this file is invertible");

    let shipped_colorants = iccce_cmm::builtin::srgb().matrix().rows;

    // --- encoding census ----------------------------------------------------
    let mut enc_cells_differing = 0usize;
    let mut enc_max_lsb = 0i32;
    let mut enc_cells_differing_cone_only = 0usize;
    for i in 0..3 {
        for j in 0..3 {
            let d = enc_s15f16(iccce_chad[i][j]) - enc_s15f16(PUBLISHED_CHAD[i][j]);
            if d != 0 {
                enc_cells_differing += 1;
                if d.abs() > enc_max_lsb.abs() {
                    enc_max_lsb = d;
                }
            }
            if enc_s15f16(variant_cone[i][j]) != enc_s15f16(PUBLISHED_CHAD[i][j]) {
                enc_cells_differing_cone_only += 1;
            }
        }
    }

    Analysis {
        iccce_chad,
        variant_cone,
        variant_icc,
        variant_rival,
        predicted_chad,
        shipped_colorants,
        enc_cells_differing,
        enc_max_lsb,
        enc_cells_differing_cone_only,
    }
}

// ===========================================================================
// Records
// ===========================================================================

const CITE: &str = "ICC, How to interpret the sRGB color space (specified in IEC 61966-2-1) \
                    for ICC profiles, Jack Holm, 2015-04-27, section B.2 — the document \
                    ICC.1:2022 Annex E.4.2 refers to. Corpus: \
                    ICC_Spec/icc/icc__s__srgb_for_icc_profiles.md";

const CELL_NAMES: [[&str; 3]; 3] = [
    ["r0c0", "r0c1", "r0c2"],
    ["r1c0", "r1c1", "r1c2"],
    ["r2c0", "r2c1", "r2c2"],
];

/// Build every Pass I record.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn records(a: &Analysis) -> Vec<Record> {
    let mut out = Vec::new();

    // -----------------------------------------------------------------------
    // §A — the instrument. Nothing below §A means anything if §A is red.
    // -----------------------------------------------------------------------

    let (a1, a1i, a1j) = max_cell(&sub3(&a.variant_icc, &PUBLISHED_CHAD));
    out.push(
        Record::graded(
            "passi/A/harness-CAT-reproduces-ICC-published-chad-from-Bradford-0.8950",
            Kind::GroundTruth,
            Metric::AbsMaxComponent,
            Tolerance::new(
                F64_NOISE,
                "The published matrix is printed to fifteen decimals, so the print itself \
                 carries up to 5e-16 of rounding, and exact rational reconstruction from \
                 Bradford(0.8950) leaves 5.668e-16 — the print floor. Everything above that is \
                 f64 accumulation over one adjugate inverse and two 3x3 products at magnitudes \
                 of order 1, conservatively 50 ulp = 1.9e-14. The bound is 50x that headroom \
                 and 5.7e6x below the 5.663e-6 cone-cell difference it must not mask.",
            ),
            a1,
            CITE,
            format!(
                "INSTRUMENT CHECK, and it is a ground-truth row in its own right. \
                 The harness's own CAT — its own typed digits, its own adjugate inverse, no \
                 iccce code in the loop — is given ICC's own construction inputs \
                 (Bradford with M_A[0][0]=0.8950 as recovered by eigendecomposition, source \
                 white 0.9505/1/1.0890 from section A.4, destination 0.9642/1/0.8249) and must \
                 return the nine cells ICC printed. Max |cell| difference {a1:.6e} at \
                 ({a1i},{a1j}). This reproduces icc-spec-librarian's 0.8950 finding by a second \
                 route, in a second language, from an independent transcription — and it is \
                 what licenses every prediction in sections B, C and E.",
            ),
        )
        .with_separation(Separation::against_distance(
            "the same construction with ICC.1:2022 Annex E.3's printed Bradford (0.8951) — \
             i.e. the reading that ICC's chad was computed the way ICC.1 prints it",
            CONE_TERM_EXACT,
            CONE_TERM_EXACT,
            SepUnits::SameAsMetric,
        )),
    );

    let (a2, _, _) = max_cell(&sub3(&a.variant_cone, &PUBLISHED_CHAD));
    out.push(Record::graded(
        "passi/A/E.3-Bradford-does-NOT-reproduce-the-published-chad",
        Kind::DerivedExpectation,
        Metric::AbsMaxComponent,
        Tolerance::new(
            F64_NOISE,
            "Graded quantity is |measured - exact prediction|, not a colour difference: the \
             prediction 5.661341564633735e-6 is exact rational arithmetic over the published \
             constants, so the only admissible discrepancy is f64 round-off. Same derivation as \
             the row above.",
        ),
        (a2 - CONE_TERM_EXACT).abs(),
        CITE,
        format!(
            "THE CONE TERM, ISOLATED — and the row that stops the section above from being a \
             tautology. Substituting ICC.1:2022 Annex E.3 Eq. (E.1)'s printed 0.8951 for the \
             recovered 0.8950, with every other input identical, moves the matrix by \
             {a2:.6e} (predicted exactly: {CONE_TERM_EXACT:.6e}). Two ICC publications, two \
             Bradford matrices. iccce uses 0.8951 because that is what the specification prints; \
             this row records the divergence and does not adjudicate it — see section D.",
        ),
    )
    .with_separation(Separation::none(
        "the graded quantity is |measured - exact prediction|, and an exact rational value has \
         no rival candidate: the only alternative reading is that the arithmetic is wrong, which \
         is what the row measures",
    )));

    let (a3, a3i, a3j) = max_cell(&sub3(
        &sub3(&a.predicted_chad, &PUBLISHED_CHAD),
        &PREDICTED_CHAD_RESIDUAL,
    ));
    out.push(Record::graded(
        "passi/A/typed-exact-predictions-still-hold",
        Kind::DerivedExpectation,
        Metric::AbsMaxComponent,
        Tolerance::new(
            F64_NOISE,
            "Same f64-round-off derivation. This row exists because the per-cell bounds in \
             section B are typed constants: if a published digit in this file is ever edited, or \
             the exact derivation in TOLERANCES.md section 3.9 is superseded, the bounds must \
             fail loudly here rather than quietly grade against a stale prediction.",
        ),
        a3,
        "Exact rational arithmetic (Python Fraction) over the published constants in this file, \
         2026-08-17; derivation recorded in docs/TOLERANCES.md section 3.9",
        format!(
            "STALE-CONSTANT GUARD. The harness recomputes all nine predicted residuals in f64 \
             from the published inputs and compares them to the exact values typed above. \
             Worst disagreement {a3:.6e} at ({a3i},{a3j}).",
        ),
    )
    .with_separation(Separation::none(
        "no rival candidate: the row compares one derivation against the same derivation in \
         another arithmetic",
    )));

    // -----------------------------------------------------------------------
    // §B — the graded claim: nine cells against ICC's published matrix.
    // -----------------------------------------------------------------------

    let diff = sub3(&a.iccce_chad, &PUBLISHED_CHAD);
    let rival_diff = sub3(&a.variant_rival, &PUBLISHED_CHAD);
    let rival_dist = sub3(&a.iccce_chad, &a.variant_rival);
    // First pass: how much of section B would the named rival actually break?
    // Computed before any row is built, because the answer appears in the rows'
    // own text and a number quoted in a detail string must be one the harness
    // holds, never one somebody typed. (`README.md` §20; three stale literals
    // were found in one sweep on 2026-08-12 and two more on 2026-08-17.)
    let mut rival_breaches = 0usize;
    let mut rival_worst_ratio = 0.0f64;
    for i in 0..3 {
        for j in 0..3 {
            let bound = PREDICTED_CHAD_RESIDUAL[i][j].abs() + F64_NOISE;
            let alt = rival_diff[i][j].abs();
            if alt > bound {
                rival_breaches += 1;
                rival_worst_ratio = rival_worst_ratio.max(alt / bound);
            }
        }
    }

    for i in 0..3 {
        for j in 0..3 {
            let predicted = PREDICTED_CHAD_RESIDUAL[i][j].abs();
            let bound = predicted + F64_NOISE;
            let observed = diff[i][j].abs();
            let alt = rival_diff[i][j].abs();
            out.push(
                Record::graded(
                    format!("passi/B/chad-cell-{}", CELL_NAMES[i][j]),
                    Kind::GroundTruth,
                    Metric::AbsMaxComponent,
                    Tolerance::new(
                        bound,
                        "The exactly-derived residual for this cell plus one f64 allowance. It \
                         is a PREDICTION, not an observation: the residual is the sum of two \
                         documented input differences — ICC built its chad with Bradford \
                         M_A[0][0]=0.8950 while iccce uses ICC.1:2022 E.3's printed 0.8951 \
                         (5.661e-6), and ICC's chad adapts the 4-dp-rounded white 0.9505/1/1.0890 \
                         while iccce derives D65 from BT.709-6's chromaticities (4.453e-5). Both \
                         terms are computable in exact rational arithmetic from published \
                         constants alone, which is why this bound could be written down before \
                         the row was ever run. ONE-SIDED BY CONSTRUCTION: a change that moved \
                         iccce TOWARD ICC's own construction would pass silently here — section \
                         C is the two-sided gate.",
                    ),
                    observed,
                    CITE,
                    format!(
                        "iccce {:.15} vs ICC published {:.15}; signed residual {:+.6e} \
                         ({:+.3} ULP of s15Fixed16). Predicted exactly {:+.6e}. \
                         GROUND TRUTH ABOUT AGREEMENT WITH A RECOMMENDATION, not about \
                         correctness: ICC.1 mandates no CAT at all (corpus A29), so what is \
                         graded is that iccce's Bradford-derived matrix agrees with the one ICC \
                         recommends to the extent the two constructions' inputs entail. \
                         ★ READ THE BLIND FLAG CORRECTLY: the candidate distance ({:.6e}) is \
                         smaller than this bound ({:.6e}), so the mechanism reports BLIND — \
                         which is the conservative and correct verdict for a DISTANCE test. It \
                         understates this row, because the observation sits exactly AT its bound \
                         by construction, so the rival breaches it on {rival_breaches} of the 9 \
                         cells anyway (worst exceedance {rival_worst_ratio:.3}x). The row that \
                         carries this power without needing an argument is passi/C, where the \
                         same separation is DISCRIMINATING by six orders.",
                        a.iccce_chad[i][j],
                        PUBLISHED_CHAD[i][j],
                        diff[i][j],
                        diff[i][j] / S15F16_ULP,
                        PREDICTED_CHAD_RESIDUAL[i][j],
                        rival_dist[i][j].abs(),
                        bound,
                    ),
                )
                .with_separation(Separation::against_distance(
                    "iccce built with ICC's own cone matrix (M_A[0][0] = 0.8950, recovered from \
                     the published chad) instead of ICC.1:2022 E.3's printed 0.8951",
                    alt,
                    rival_dist[i][j].abs(),
                    SepUnits::SameAsMetric,
                )),
            );
        }
    }

    let (bmax, bmaxi, bmaxj) = max_cell(&diff);
    let (pmax, _, _) = max_cell(&PREDICTED_CHAD_RESIDUAL);
    out.push(
        Record::graded(
            "passi/B/chad-max-over-nine-cells",
            Kind::GroundTruth,
            Metric::AbsMaxComponent,
            Tolerance::new(
                pmax + F64_NOISE,
                "The maximum of the nine per-cell predictions plus one f64 allowance. This is \
                 the headline number for the pass and the one to quote; the per-cell rows are \
                 what make it a test rather than a summary.",
            ),
            bmax,
            CITE,
            format!(
                "THE HEADLINE. iccce's D65->D50 Bradford matrix differs from ICC's published \
                 recommended chad by at most {bmax:.6e} ({:.3} ULP of s15Fixed16), at cell \
                 ({bmaxi},{bmaxj}); predicted exactly {pmax:.6e}. Decomposition, exact: the \
                 white-point term contributes {WHITE_TERM_EXACT:.6e} and the cone-matrix term \
                 {CONE_TERM_EXACT:.6e}, and they PARTIALLY CANCEL. ★ A bound derived from the \
                 cone term alone — which is what this pass was commissioned with — would have \
                 been {CONE_TERM_EXACT:.6e} and this row would have failed at {:.1}x it.",
                bmax / S15F16_ULP,
                bmax / CONE_TERM_EXACT,
            ),
        )
        .with_separation(Separation::against_distance(
            format!(
                "iccce built with ICC's own cone matrix (0.8950): the rival lands at {:.6e}, \
                 which is {:.3}x this row's bound",
                max_cell(&rival_diff).0,
                max_cell(&rival_diff).0 / (pmax + F64_NOISE),
            ),
            max_cell(&rival_diff).0,
            RIVAL_CONE_DISTANCE,
            SepUnits::SameAsMetric,
        )),
    );

    // -----------------------------------------------------------------------
    // §C — the two-sided regression gate, and the DL-018 sensitivity control.
    // -----------------------------------------------------------------------

    let (cmax, cmaxi, cmaxj) = max_cell(&sub3(&a.iccce_chad, &a.predicted_chad));
    out.push(
        Record::graded(
            "passi/C/iccce-matches-the-independent-prediction-two-sided",
            Kind::DerivedExpectation,
            Metric::AbsMaxComponent,
            Tolerance::new(
                F64_NOISE,
                "Two f64 implementations of one construction over identical published inputs \
                 may differ only by round-off; the 1e-12 derivation is on F64_NOISE. This is the \
                 row with power against a change in EITHER direction, which section B's \
                 one-sided form does not have: it detects a corrupted BRADFORD digit, a \
                 corrupted D65_XY or D50, an inverted operand order (M_A.D.M_A^-1), or a \
                 transposition — all of which produce a matrix that still looks like an \
                 adaptation matrix.",
            ),
            cmax,
            "Independent f64 re-derivation in this file from the published constants; the \
             prediction side contains no iccce code",
            format!(
                "REGRESSION GATE. iccce's matrix vs the harness's own construction from the same \
                 published inputs: worst cell {cmax:.6e} at ({cmaxi},{cmaxj}). \
                 SENSITIVITY (DL-018): substituting ICC's own 0.8950 cone cell moves iccce's \
                 matrix by {RIVAL_CONE_DISTANCE:.6e}, which is {:.2e}x this bound — the \
                 instrument can see the thing the pass is about, by six orders of magnitude. \
                 In section B the same substitution breaches {rival_breaches} of the 9 per-cell \
                 bounds, worst exceedance {rival_worst_ratio:.3}x, so section B is load-bearing \
                 on the cone cell too, if less emphatically. \
                 ★ WHAT THIS ROW CANNOT DO, stated because the observation is {cmax:.6e}: \
                 inv3() here is an adjugate-over-determinant inverse and so is \
                 iccce_color::Mat3::inverse, and both evaluate M_A^-1 . (D . M_A) in that \
                 association, so bit-for-bit agreement is EXPECTED and is not evidence that \
                 either implementation's arithmetic is well conditioned. Its power is against \
                 CHANGE — a constant, an operand order, a transposition — and the check that the \
                 arithmetic is right at all is section A, which grades against published digits.",
                RIVAL_CONE_DISTANCE / F64_NOISE,
            ),
        )
        .with_separation(Separation::against_distance(
            "iccce built with ICC's own cone matrix (M_A[0][0] = 0.8950)",
            RIVAL_CONE_DISTANCE,
            RIVAL_CONE_DISTANCE,
            SepUnits::SameAsMetric,
        )),
    );

    // -----------------------------------------------------------------------
    // §D — REPORTED. Two ICC publications disagree; iccce records, not adjudicates.
    // -----------------------------------------------------------------------

    out.push(Record::graded(
        "passi/D/two-ICC-publications-print-different-Bradford-matrices",
        Kind::GroundTruth,
        Metric::AbsMaxComponent,
        Tolerance::new(
            f64::INFINITY,
            "REPORTED, NOT GRADED, and deliberately: there is no clause under which one of two \
             ICC publications is the wrong one. ICC.1:2022 Annex E.3 Eq. (E.1) prints 0.8951; \
             ICC's own sRGB guidance computed its recommended chad with 0.8950. Annex E is \
             informative and ICC.1 mandates no CAT at all (A29), so neither value is required of \
             anybody. iccce follows the printed specification. Grading this would mean this \
             project deciding which ICC document is authoritative, which is not a decision a \
             conformance suite is entitled to make.",
        ),
        CONE_TERM_EXACT,
        CITE,
        format!(
            "FINDING, recorded: the two published Bradford matrices differ only in M_A[0][0] \
             (0.8951 vs 0.8950) and that difference is worth {CONE_TERM_EXACT:.6e} in the \
             resulting adaptation matrix, {:.3} ULP of s15Fixed16. The distinguishing digit was \
             in this project's corpus for six days as a sanity-check footnote: E.1's first row \
             sums to 1.0001, the recovered matrix's to exactly 1.0000. Filed \
             icc__ref__spec_defects.md 24.4.",
            CONE_TERM_EXACT / S15F16_ULP,
        ),
    )
    .with_separation(Separation::none(
        "REPORTED: the row's subject IS the divergence between two candidate readings, so the \
         candidates are not rivals for its observation — they are its observation",
    )));

    out.push(
        Record::graded(
            "passi/D/encoded-chad-cells-differing-from-ICC-published",
            Kind::GroundTruth,
            Metric::IndicatorCount,
            Tolerance::new(
                f64::INFINITY,
                "REPORTED. A count of encoding differences is not a conformance requirement: \
                 iccce writes no chad tag for its built-in sRGB today, and no clause requires a \
                 profile's chad to equal ICC's recommended one. It is emitted because it is the \
                 unit a profile author cares about and because it CORRECTS AN INFERENCE this \
                 project made in writing.",
            ),
            {
                let n = a.enc_cells_differing;
                // Counts are small; the cast is exact for any n <= 2^53.
                f64::from(u32::try_from(n).unwrap_or(u32::MAX))
            },
            CITE,
            format!(
                "★ CORRECTION TO A COROLLARY, measured. {} of 9 cells of iccce's chad encode to \
                 a different s15Fixed16 word than ICC's published chad, largest difference {} \
                 LSB. And the corpus's own consequence note — that a chad recomputed from E.3 \
                 differs by 0.371 ULP and therefore 'the written tag bytes are identical' — does \
                 not follow even in its own case: at ICC's rounded white, where the difference \
                 IS 0.371 ULP, {} of 9 cells still encode differently, because a sub-ULP \
                 difference near a half-ULP rounding boundary still flips the LSB. SUB-ULP DOES \
                 NOT MEAN IDENTICAL BYTES; it means at most 1 LSB.",
                a.enc_cells_differing, a.enc_max_lsb, a.enc_cells_differing_cone_only,
            ),
        )
        .with_separation(Separation::none(
            "a count has no rival candidate: it is the same nine cells encoded by one rule",
        )),
    );

    // -----------------------------------------------------------------------
    // §E — the SHIPPED construction, against ICC's published colorants.
    // -----------------------------------------------------------------------

    let col_diff = sub3(&a.shipped_colorants, &PUBLISHED_COLORANTS);
    let (emax, emaxi, emaxj) = max_cell(&col_diff);
    let (epred, _, _) = max_cell(&PREDICTED_COLORANT_RESIDUAL);

    // iccce's D65 RGB→XYZ matrix, recovered as `chad⁻¹ · colorants`. Used for
    // the rival construction and for the attribution row below; legitimate
    // because the `chad` is far from singular and both uses are informational.
    let m_d65 = {
        let inv_chad = inv3(&a.iccce_chad).expect("iccce's chad is invertible");
        mul3(&inv_chad, &a.shipped_colorants)
    };
    let col_rival = mul3(&a.variant_rival, &m_d65);
    let (col_rival_resid, _, _) = max_cell(&sub3(&col_rival, &PUBLISHED_COLORANTS));
    let (col_rival_dist, _, _) = max_cell(&sub3(&a.shipped_colorants, &col_rival));

    out.push(Record::graded(
        "passi/E/shipped-srgb-colorants-vs-ICC-published",
        Kind::GroundTruth,
        Metric::AbsMaxComponent,
        Tolerance::new(
            epred + F64_NOISE,
            "The exactly-derived worst-cell residual of the shipped construction against ICC's \
             published colorants (4.607402e-5 = 3.020 ULP of s15Fixed16), plus one f64 \
             allowance. Derived, not observed: it is chad(0.8951, chromaticity D65) x \
             rgb_to_xyz(BT.709-6 primaries) minus ICC's printed matrix, in exact rational \
             arithmetic. It matches the figure builtin.rs declares as the model's one named \
             approximation, which is the point — rule 4 requires the approximation to be \
             measured, and until this row nothing measured it.",
        ),
        emax,
        CITE,
        format!(
            "★ This row checks iccce's CONSTRUCTION against ICC's published colorants from the \
             harness side. ★★ CORRECTED 2026-08-17, same day: this text claimed to be 'the only \
             place in the repository that checks ICC's published colorants', on a grep for \
             '0.436030...' that returned nothing under crates/. THAT GREP WAS DEFEATED BY RUST \
             NUMERIC SEPARATORS — the value is written 0.436_030_342_570_117 and is asserted at \
             crates/iccce-cmm/src/builtin.rs by \
             matches_icc_published_colorants_within_stated_ulps, which bounds the same worst cell \
             at 4 ULP. The claim that builtin.rs's 'asserted in the tests' was untrue is \
             WITHDRAWN; it was true. ★ Worth keeping rather than deleting: a NEGATIVE finding \
             from a grep is only as good as the grep, and a language feature that exists purely \
             to aid human reading silently broke this one (DL-042 — a negative finding removes \
             its own auditor). Measured worst cell {emax:.6e} = {:.3} ULP at ({emaxi},{emaxj}); \
             predicted {epred:.6e}. This row remains valuable as an INDEPENDENT second check, in \
             a different crate and a different language of expression from builtin.rs's, and it \
             puts iccce_cmm::builtin::srgb() — the construction the shipped binary uses — in the \
             loop, which sections A to D do not.",
            emax / S15F16_ULP,
        ),
    )
    .with_separation(Separation::against_distance(
        format!(
            "the same construction with ICC's own cone matrix (M_A[0][0] = 0.8950): the rival's \
             colorants land {col_rival_resid:.6e} from the published ones, i.e. FURTHER away \
             than iccce's {emax:.6e} — adopting ICC's cone cell would make this row WORSE",
        ),
        col_rival_resid,
        col_rival_dist,
        SepUnits::SameAsMetric,
    )));

    // The decomposition of that residual, which is where the interesting
    // finding is: builtin.rs attributes it to one cause and there are two.
    let chad_term = mul3(&sub3(&a.iccce_chad, &PUBLISHED_CHAD), &m_d65);
    let (ct, cti, ctj) = max_cell(&chad_term);
    out.push(Record::graded(
        "passi/E/colorant-residual-attribution",
        Kind::GroundTruth,
        Metric::AbsMaxComponent,
        Tolerance::new(
            f64::INFINITY,
            "REPORTED. The subject is an ATTRIBUTION — which of two inputs a known residual came \
             from — and there is no clause and no published value against which an attribution \
             is graded. The number it carries is graded in the row above.",
        ),
        ct,
        "Exact rational decomposition, 2026-08-17, recorded in docs/TOLERANCES.md section 3.9",
        format!(
            "★★ FINDING — builtin.rs says the colorant residual is 'entirely accounted for by \
             which D65 matrix each side starts from'. It is not. Exact decomposition of \
             (iccce - ICC) into (iccce_chad - published_chad).M_d65 + published_chad.(M_d65 - \
             inv(A.7)): the CHAD term reaches 2.482 ULP and the PRIMARIES term 2.480 ULP — the \
             same size. Worst chad-term cell measured here {ct:.6e} at ({cti},{ctj}). On \
             bXYZ.Z the two terms are -2.482 and +1.586 ULP and CANCEL to the -0.897 ULP the doc \
             comment presents as a small error; it is a cancellation between two errors five \
             times its size. There are TWO named approximations in the built-in sRGB \
             construction, not one, and the second is the Bradford variant.",
        ),
    ));

    let white_sum = apply3(&PUBLISHED_COLORANTS, [1.0, 1.0, 1.0]);
    let white_err = white_sum
        .iter()
        .zip(WHITE_D50)
        .map(|(g, e)| (g - e).abs())
        .fold(0.0f64, f64::max);
    out.push(Record::graded(
        "passi/E/published-colorant-rows-sum-to-D50",
        Kind::GroundTruth,
        Metric::AbsMaxComponent,
        Tolerance::new(
            COLORANT_ROW_SUM_BOUND,
            "A transcription guard on the REFERENCE DATA, not on iccce. ICC's own prose requires \
             these colorants to be D50-adapted, so their row sums must be D50 — but not exactly, \
             and the size of the miss is derivable rather than observable. ICC's colorants are \
             chad x inv(A.7), and A.7 is printed to SEVEN decimals, so each of its cells is known \
             only to +/-5e-8; the implied white of inv(A.7) departs from the exact 0.9505/1/1.0890 \
             by 1.0608e-7 in Z, and the published chad carries that difference to 7.946512e-8 in \
             the row sums (exact rational arithmetic, and it closes to every printed digit). The \
             bound is the next power of ten above that, stated in the unit of A.7's own print \
             precision. Discrimination: one mistyped digit in the third decimal of any published \
             cell moves this quantity by ~1e-3, four orders above the bound.",
        ),
        white_err,
        CITE,
        format!(
            "REFERENCE-DATA GUARD, and a CORRECTION to the corpus's own summary of it. \
             Row sums of ICC's published colorant matrix: ({:.15}, {:.15}, {:.15}) against D50 \
             0.9642/1/0.8249; residuals {:+.6e} / {:+.6e} / {:+.6e}, worst {white_err:.6e}. \
             ★ icc__s__srgb_for_icc_profiles.md prints exactly these three row sums and then \
             summarises them as reproducing D50 'to 9,3x10^-9' — that is the FIRST row's \
             residual quoted as though it were the maximum; the Z row is 8.5x larger. This bound \
             was first derived from the 9.3e-9 figure and FAILED at {white_err:.6e}, which is how \
             the mis-summary was found. Neither iccce nor the published data is wrong; the \
             summary sentence was.",
            white_sum[0],
            white_sum[1],
            white_sum[2],
            white_sum[0] - WHITE_D50[0],
            white_sum[1] - WHITE_D50[1],
            white_sum[2] - WHITE_D50[2],
        ),
    ));

    out
}

/// Run the pass. Takes no [`crate::Oracle`] — see the module header.
#[must_use]
pub fn run() -> (Analysis, Vec<Record>) {
    let a = analyse();
    let r = records(&a);
    (a, r)
}

/// One line for the run's note block.
#[must_use]
pub fn note(a: &Analysis) -> String {
    let (bmax, _, _) = max_cell(&sub3(&a.iccce_chad, &PUBLISHED_CHAD));
    let (emax, _, _) = max_cell(&sub3(&a.shipped_colorants, &PUBLISHED_COLORANTS));
    format!(
        "chad vs ICC published (srgb.pdf B.2): max cell {bmax:.6e} = {:.3} ULP \
         [white term {WHITE_TERM_EXACT:.3e} + cone term {CONE_TERM_EXACT:.3e}, partially \
         cancelling] | shipped sRGB colorants vs published: {emax:.6e} = {:.3} ULP | \
         {} of 9 chad cells encode differently | no oracle, no fixture, runs everywhere",
        bmax / S15F16_ULP,
        emax / S15F16_ULP,
        a.enc_cells_differing,
    )
}
