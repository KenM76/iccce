//! # Built-in colour spaces — constructed, never loaded
//!
//! ## Purpose
//!
//! This module supplies colour-space models that iccce **computes from
//! published constants** rather than parsing out of a file. Today there
//! is exactly one: **sRGB**, used as the default destination when a
//! caller has no destination profile.
//!
//! ## Why "constructed, never loaded" is the whole design
//!
//! The alternative — shipping somebody's `.icc` and embedding its bytes
//! — fails three constraints that the first consumer (`pdfce`, a PDF
//! engine) enforces in CI, and that this project has independently:
//!
//! | constraint | a shipped profile blob | this module |
//! |---|---|---|
//! | builds for `wasm32-unknown-unknown` | needs the bytes reachable at run time (`include_bytes!` or a file read) | **no I/O of any kind** |
//! | no copyleft anywhere in the tree | depends whose profile you ship, and profile `cprt` tags are a licensing minefield (23 distinct grants across 46 files in ICC's own published set) | **no dependency, no third-party bytes** |
//! | MIT, redistributable | redistributing an ICC profile is redistributing someone's copyrighted file | **numbers with citations; a chromaticity is not copyrightable expression** |
//!
//! See `docs/DEFAULT_DESTINATION.md` for the operator's decision and its
//! conditions.
//!
//! ## Sourcing — every constant, and its evidence tier
//!
//! Project rule 2 forbids writing colour maths from memory. Nothing here
//! was recalled. Each constant names its document and clause:
//!
//! | constant | source | tier |
//! |---|---|---|
//! | primaries `0.640/0.330`, `0.300/0.600`, `0.150/0.060` | **Rec. ITU-R BT.709-6 (06/2015), item 1.3** | `cross_verified_2src` — BT.709-6 primary, plus W3C 1996 Table 0.2, W3C CSS Color 4, lcms2. **Four sources, zero disagreement.** |
//! | white point `D65 = 0.3127/0.3290` | **Rec. ITU-R BT.709-6, item 1.4** | `cross_verified_2src` — same four |
//! | transfer function `12.92 / 1.055 / 0.055 / 2.4` | W3C CSS Color 4 §"sRGB transfer"; W3C 1996 eq. 1.2/1.7 | `cross_verified_2src` |
//! | breakpoint `0.04045` | W3C CSS Color 4 | `cross_verified_2src` (CSS + lcms2) — ⚠ **neither is IEC**; see below |
//! | Bradford cone matrix | **ICC.1:2022 Annex E.3, Eq. (E.1)** | `primary_spec` (Annex E is *informative*) |
//! | adaptation form `M_BFD⁻¹ · D · M_BFD` | **ICC.1:2022 Annex E.2–E.4** | `primary_spec` |
//! | PCS white `D50 = 0.9642/1/0.8249` | ICC.1 header illuminant | see [`iccce_color::illuminant::D50`] |
//!
//! **★ `IEC 61966-2-1` — sRGB's actual defining standard — is paywalled
//! and has never been obtained by this project.** Every number above is
//! corroborated from documents that are *not* IEC and, importantly, *not
//! lcms2 alone*: **primaries and white point come from ITU, a different
//! standards body entirely.** That matters more than it looks. lcms2 is
//! this project's differential oracle, and a destination built on the
//! oracle's own constants would make subsequent agreement with that
//! oracle evidence of nothing (`docs/NEXT_SESSION.md` §5.2 — a
//! non-conformant estimator once landed 0.082 ΔE76 from lcms2 while
//! carrying a 4.717 L\* defect, 57.8× the signal it produced).
//! **BT.709-6 is what breaks that circularity.**
//!
//! `0.04045` is the one constant still not settled from a standards
//! *text*: CSS Color 4 prints it, but CSS's own normative reference for
//! sRGB *is* the IEC paywall, so CSS restates rather than corroborates.
//! Recorded as an open sourcing item, not a blocker.
//!
//! ## ★ The transfer function's linear segment, and why it is here
//!
//! sRGB is **not** gamma 2.2. It is a piecewise curve with a linear
//! segment near black, and omitting that segment is the classic
//! wrong-but-plausible sRGB error: at encoded `V = 0.01` the correct
//! linear value is `0.000774` and pure `V^2.2` gives `0.0000631` — a
//! factor of **12**. Midtones and highlights are indistinguishable, so a
//! side-by-side of a normal image shows nothing, and it surfaces only on
//! dark imagery or after a round-trip through a linear working space.
//! The curve below is stored as an ICC `parametricCurveType` **funcType
//! 3**, which is exactly this shape, so the segment cannot be lost.
//!
//! ## ★★ Rule 4 — the named approximation, measured against PUBLISHED
//! ## values
//!
//! See [`srgb`]'s own doc comment for the full statement. In one line:
//! **this construction agrees with ICC's own published D50-adapted sRGB
//! colorants to 3.02 ULP of `s15Fixed16`, and the difference is fully
//! explained** — ICC starts from their §A.7 matrix as printed to 7
//! decimals, iccce builds it exactly from BT.709-6's chromaticities.
//!
//! ★ It is **not** byte-identical to the shipped HP/`sRGB2014` profile
//! either, and that is now known to be the file's problem: the file
//! misses ICC's published values by **11.13 ULP** in `bXYZ.Z` while this
//! construction misses them by **0.90 ULP**. Do not restore a test
//! against the file — see [`srgb`] for why that framing was wrong for
//! most of a day.

use crate::curve::Trc;
use crate::matrix_trc::MatrixTrc;
use iccce_color::{BRADFORD, D50, D65_XY, Mat3, XyY, Xyz, adaptation_matrix};

/// sRGB / BT.709 primary chromaticities, `(x, y)` per primary, in R, G,
/// B order.
///
/// **Source: Rec. ITU-R BT.709-6 (06/2015), item 1.3** ("Chromaticity
/// coordinates (CIE, 1931)"), corroborated by W3C's 1996 sRGB proposal
/// Table 0.2, by W3C CSS Color 4, and by lcms2 `cmsvirt.c`. Four
/// publications, identical to the four significant figures all of them
/// print, zero disagreement.
///
/// ★ **None of the four prints more than four significant figures.**
/// Do not "improve" these with a longer value from elsewhere — the sRGB
/// matrix is *defined* by the 4-figure numbers, and substituting CIE's
/// own 5-figure D65 (`0.312 72 / 0.329 03`) changes every cell of the
/// resulting matrix while looking like a precision upgrade.
pub const SRGB_PRIMARIES_XY: [(f64, f64); 3] = [(0.640, 0.330), (0.300, 0.600), (0.150, 0.060)];

/// sRGB's transfer function as ICC `parametricCurveType` **funcType 3**
/// parameters, in Table 67 order `(g, a, b, c, d)`.
///
/// ICC.1:2022 clause 10.18, funcType 3 defines
/// `Y = (a·X + b)^g` for `X ≥ d`, and `Y = c·X` otherwise.
/// Substituting the values below gives
/// `Y = ((X + 0.055)/1.055)^2.4` and `Y = X/12.92` — sRGB exactly.
///
/// **Source of the four shape constants** (`12.92`, `1.055`, `0.055`,
/// `2.4`): W3C CSS Color 4 and W3C's 1996 sRGB proposal eq. 1.2/1.7,
/// corroborated by lcms2. **Source of the breakpoint `0.04045`:** W3C
/// CSS Color 4.
///
/// ## ★ Why `0.04045` and not `0.03928`
///
/// Both are *published*, by the specification's own authors, and
/// `0.03928` is the single most common wrong-but-plausible sRGB constant
/// in circulation. The two documents solve **different equations**:
///
/// - `0.04045` is the **C⁰** (value-continuity) solution with `a` pinned
///   at exactly `0.055` — the exact root is `0.040 448 236 277…`, and
///   `0.04045` is it, correctly rounded to five decimals.
/// - `0.03928` is the **C¹** (value *and slope*) solution, which
///   requires `a = 0.055 010 718 9…` — but W3C 1996 printed that
///   breakpoint *alongside* `a` rounded to `0.055`, which are mutually
///   inconsistent choices.
///
/// W3C's own live erratum banner on that page says so: *"During
/// standardization, a small numerical error caused by rounding error was
/// corrected."* IEC's correction was to keep `a = 0.055` and re-solve
/// for value continuity.
///
/// ★ **Adopting `a = 0.0550107` would be a DIFFERENT CURVE, not a more
/// precise sRGB.** Anyone who "fixes" sRGB that way has silently left
/// the standard.
///
/// ## ★★ What using `0.03928` actually costs — measured here, and it is
/// ## WORSE than "small"
///
/// The two constants disagree only for encoded values strictly inside
/// `(0.03928, 0.04045)`, and the **maximum** separation anywhere in that
/// window is `7.5548×10⁻⁷` in linear light — about 20× below the 16-bit
/// PCS quantum (`1/65535 = 1.5×10⁻⁵`).
///
/// ★★ **The maximum is INTERIOR, at `V ≈ 0.039 302 447`, and both
/// endpoints of the window have essentially no power at all.** This is
/// the counter-intuitive part and it decides where a test must go:
///
/// | `V` | separation |
/// |---|---|
/// | `0.03928` — the window's lower edge | **exactly `0`** — both constants take the *linear* branch there |
/// | `0.039 302 447` — interior maximum | **`7.5548×10⁻⁷`** |
/// | `0.040 449` — approaching the upper edge | `1.0×10⁻⁹` |
///
/// So a test placed at `0.03928` — the obvious "test at the boundary"
/// choice — has **exactly zero** discriminating power, the same as an
/// 8-bit vector. *(Located twice: derivative root-find and ternary
/// search, agreeing to 12 figures; independently reproduced at 60-digit
/// precision by `icc-spec-librarian` via two further routes.)*
///
/// ★ **And no 8-bit code lands in the window at all.** `10/255 =
/// 0.039216` sits *below* `0.03928`, and `11/255 = 0.043137` sits
/// *above* `0.04045`, so both constants take the same branch for every
/// one of the 256 codes and the separation at 8-bit input precision is
/// **exactly zero** *(measured 2026-08-17)*. This **corrects** the
/// widely repeated claim — carried in this project's own standards
/// corpus until today — that the error "affects 8-bit codes 10 and 11":
/// it affects **neither**.
///
/// The practical consequence is the uncomfortable one. A wrong
/// breakpoint here is:
///
/// - invisible in every image,
/// - invisible to any 8-bit test vector,
/// - invisible to a round-trip, because the same wrong constant inverts
///   itself, and
/// - **invisible to a differential test against lcms2 or any other
///   implementation that made the same choice.**
///
/// It surfaces *only* against a correctly built reference evaluated at
/// non-8-bit precision inside the window. That is exactly the defect
/// class this project exists to catch — a wrong answer that looks
/// identical to a right one — which is why the constant is cited rather
/// than typed, and why
/// `tests::breakpoint_is_the_c0_solution_not_the_1996_value` exists
/// and evaluates where it does.
///
/// ★ `a`, `b` and `c` are written as **computed expressions**, never as
/// truncated decimals: they are irrational-looking reciprocals
/// (`1/1.055`, `0.055/1.055`, `1/12.92`) and typing rounded decimals
/// would compound an avoidable error onto the unavoidable one.
pub const SRGB_TRC_PARAMS: [f64; 5] = [
    2.4,           // g
    1.0 / 1.055,   // a
    0.055 / 1.055, // b
    1.0 / 12.92,   // c
    0.04045,       // d — the C⁰ breakpoint; see above
];

/// Build the sRGB→XYZ matrix for a set of primary chromaticities and a
/// white point, by Grassmann's laws.
///
/// ## The construction, and why it is computed rather than transcribed
///
/// Given primaries as chromaticities and an adopted white, the RGB→XYZ
/// matrix is determined: form the matrix `P` whose columns are each
/// primary's `(x/y, 1, (1−x−y)/y)`, solve `P · s = W` for the per-primary
/// luminance scalars `s`, and scale `P`'s columns by `s`. The result maps
/// device `(1,1,1)` to exactly `W`.
///
/// **This is linear algebra, not a recalled constant** — which is why it
/// is safe to compute here while the chromaticities themselves are
/// sourced. Transcribing the resulting nine numbers instead would add a
/// transcription risk to constants that are already four sources deep.
///
/// ## The self-check that makes this trustworthy
///
/// The unit tests assert that this function, fed the sourced BT.709
/// primaries and D65, reproduces the matrix W3C's 1996 document *prints*
/// (eq. 1.8) to within that document's own 4-decimal rounding. That is
/// an **external** check on this code's arithmetic — the expectation
/// comes from a publication, not from this function.
///
/// Returns `None` if the primaries are degenerate (collinear in the
/// chromaticity plane, or a zero `y`), which would make `P` singular.
/// Refused rather than defaulted: a silently substituted matrix here
/// would be invisible.
#[must_use]
pub fn rgb_to_xyz(primaries_xy: &[(f64, f64); 3], white: Xyz) -> Option<Mat3> {
    let col = |(x, y): (f64, f64)| -> Option<[f64; 3]> {
        if y == 0.0 {
            return None;
        }
        Some([x / y, 1.0, (1.0 - x - y) / y])
    };
    let c = [
        col(primaries_xy[0])?,
        col(primaries_xy[1])?,
        col(primaries_xy[2])?,
    ];
    // P's COLUMNS are the primaries, so row i is (c0[i], c1[i], c2[i]).
    let p = Mat3 {
        rows: [
            [c[0][0], c[1][0], c[2][0]],
            [c[0][1], c[1][1], c[2][1]],
            [c[0][2], c[1][2], c[2][2]],
        ],
    };
    let s = p.inverse()?.apply([white.x, white.y, white.z]);
    Some(Mat3 {
        rows: [
            [c[0][0] * s[0], c[1][0] * s[1], c[2][0] * s[2]],
            [c[0][1] * s[0], c[1][1] * s[1], c[2][1] * s[2]],
            [c[0][2] * s[0], c[1][2] * s[1], c[2][2] * s[2]],
        ],
    })
}

/// The D65 white point of sRGB, as XYZ normalised to `Y = 1`.
///
/// Derived from [`iccce_color::illuminant::D65_XY`] — the chromaticity
/// is what BT.709-6 item 1.4 states, and the XYZ form is computed here
/// rather than baked in as a constant, so the derivation stays visible
/// instead of appearing to be published.
///
/// Returns `None` only if the chromaticity is degenerate, which the
/// sourced value is not; the `Option` exists so this shares one code
/// path with caller-supplied whites.
#[must_use]
pub fn srgb_white_d65() -> Option<Xyz> {
    XyY {
        x: D65_XY.0,
        y: D65_XY.1,
        luma_y: 1.0,
    }
    .to_xyz()
}

/// **The built-in sRGB destination model**, D50-adapted for the ICC PCS.
///
/// This is the model iccce uses when a caller states it has no
/// destination profile. It is computed at call time from the constants
/// above; there is no file, no embedded blob, and no I/O.
///
/// ## What it is, step by step
///
/// 1. BT.709-6 primaries + D65 → the D65-referred RGB→XYZ matrix, by
///    [`rgb_to_xyz`].
/// 2. That matrix is chromatically adapted **D65 → D50** with linear
///    Bradford (ICC.1:2022 Annex E.3), because ICC's PCS is D50 and a
///    profile's colorants are stored D50-adapted.
/// 3. The TRC is the sRGB parametric curve, ICC funcType 3.
/// 4. `media_white` is set to **D50**, which is the PCS white.
///
/// ★ Step 4 is a deliberate divergence from the most widely deployed
/// sRGB profile in existence, and it is the *correct* direction. The
/// 1998 HP `sRGB IEC61966-2.1` file — the one Windows ships — stores
/// `wtpt` = **D65** while its colorants sum to **D50**, and carries **no
/// `chad`** to explain the difference. ICC.1:2001-04 Annex A.3.1.1 makes
/// that a defect. iccce's constructed profile is self-consistent:
/// `wtpt` is the PCS white, and the colorant sum agrees with it.
/// (iccce *parses* the HP file happily and discloses the inconsistency
/// via [`MatrixTrc::white_point_note`] — reading what real files contain
/// and choosing what to construct are different jobs.)
///
/// ## ★★★ Rule 4 — the named approximation. REWRITTEN 2026-08-17, and
/// ## the correction inverts who owns the error
///
/// **Until the operator obtained ICC's own "Specification of sRGB"
/// (Jack Holm, ICC, 2015-04-27) on 2026-08-17, this section said the
/// wrong thing**, and it is worth stating what it said and why it was
/// wrong, because the mistake was reasonable and would recur.
///
/// It said: reconstructing the shipped profile's colorants from the
/// sourced chromaticities lands 8 of 9 cells within 2 ULP but misses
/// `bXYZ.Z` by ~12 ULP, that no D50 tier closes it, and that **this was
/// iccce's approximation to declare**. The corpus agreed, recording the
/// D50-adapted colorants as `measured_file_behaviour` from one file and
/// noting that *"NO document publishes them."*
///
/// **That last claim was false, and everything downstream of it was
/// mis-attributed.** ICC's document publishes the D50-adapted colorants
/// outright, at 15 decimal places (§B.2). Measured against them:
///
/// | | worst cell | `bXYZ.Z` |
/// |---|---|---|
/// | **this construction** | **3.02 ULP** | **0.90 ULP** |
/// | the shipped HP 1998 / `sRGB2014.icc` file | **11.13 ULP** | **11.13 ULP** |
///
/// ★★ **The ~12 ULP blue-Z residual is the FILE's error, not ours.** The
/// most widely deployed sRGB profile in the world disagrees with ICC's
/// own published specification by 11 ULP in blue-Z, and iccce's
/// from-constants construction is **four times closer to the published
/// values than the file is**. Every earlier route "failed to close" the
/// residual because it was measuring against an artifact that does not
/// match the specification either.
///
/// ★ **The general lesson, and it is the expensive one:** the residual
/// was measured against *the only reference available at the time*, and
/// the absence of a published value was silently treated as evidence
/// that the file **was** the reference. A gap in the literature is not a
/// licence to promote an implementation to ground truth — and the
/// mis-attribution survived because the number itself was correct. **We
/// had the right residual and the wrong owner for it.**
///
/// ## What the remaining 3.02 ULP actually is — fully explained
///
/// ICC's construction is exactly recoverable from ICC's own two printed
/// matrices: **their published `chad` × the inverse of their §A.7
/// XYZ(D65)→RGB matrix reproduces their published colorants to
/// `0.00 ULP`** *(verified 2026-08-17, exact rational arithmetic)*.
///
/// So the difference is entirely accounted for by **which D65 matrix
/// each side starts from**: ICC inverts their own matrix as *printed to
/// 7 decimals*; iccce builds it *exactly* from BT.709-6's chromaticities
/// by Grassmann's laws. iccce's route carries no transcription of a
/// rounded intermediate, which is why it is kept.
///
/// ★ A related finding worth knowing before trusting the published
/// `chad` blindly: **it does not map D65 exactly onto ICC's own stated
/// D50.** `chad × D65 = (0.964150918938, 0.999997711611,
/// 0.824943819994)` against a stated `0.9642 / 1 / 0.8249` — off by
/// `≈4.9×10⁻⁵`. ICC's recommended matrix is itself slightly inconsistent
/// with the 4-figure D50 it is meant to reach.
///
/// **Consequences, and they are binding:**
///
/// - **Never write a byte-equality test against a shipped sRGB profile.**
///   Unchanged, and now better justified: the file does not match ICC's
///   published values either, so equality with it would be a *worse*
///   claim, not a stricter one.
/// - **The published colorants are the reference.** They are
///   `published-ground-truth` — a specification body's own stated values,
///   not an implementation's output — which is a strictly stronger
///   evidence class than anything this model could be checked against
///   before today.
/// - This model's one named approximation is now the **3.02 ULP
///   (`4.6×10⁻⁵` XYZ) worst-cell difference from ICC's published
///   colorants**, whose cause is stated above and asserted in the tests.
///
/// ## ★ A trap worth naming: `sRGB2014.icc` is not a second source
///
/// ICC published `sRGB2014.icc` in 2015 and it *looks* like an
/// independent, better-authored reference — it has the compliant `wtpt`
/// and the `chad` the HP file lacks. **Measured 2026-08-17: its
/// `rXYZ`/`gXYZ`/`bXYZ` and all three TRC tables are BYTE-IDENTICAL to
/// the 1998 HP profile's.** Only the header, `wtpt`, `bkpt` and `chad`
/// differ. It is the same nine numbers in a corrected wrapper, so
/// checking against it is not a second opinion — **there is exactly one
/// FILE lineage for the D50-adapted sRGB colorants.**
///
/// ★★ **That sentence used to end "…and no document publishes them at
/// all", and it survived the rewrite seventy lines above that names the
/// document which does.** Caught by `icc-librarian` reading the source,
/// not by me editing it.
///
/// The mechanism is worth more than the correction: **the clause was
/// still true right up to its last six words.** One lineage among
/// *files* remains a fact; "no document publishes them" became false the
/// moment `srgb.pdf` arrived. A reader scanning for staleness checks
/// whether a sentence is wrong, and this one reads correct until the
/// end — which is precisely why a retraction has to hunt the *claim*
/// through every clause that carries part of it, not just the paragraph
/// that states it.
#[must_use]
pub fn srgb() -> MatrixTrc {
    // These unwraps are on sourced, non-degenerate constants. If either
    // ever fails, a constant above was edited into nonsense, and a panic
    // at the point of corruption is far better than a plausible-looking
    // wrong matrix travelling downstream.
    let white_d65 = srgb_white_d65().expect("D65_XY is a valid chromaticity (BT.709-6 item 1.4)");
    let m_d65 = rgb_to_xyz(&SRGB_PRIMARIES_XY, white_d65)
        .expect("BT.709-6 primaries are non-degenerate (item 1.3)");
    let adapt = adaptation_matrix(&BRADFORD, white_d65, D50)
        .expect("Bradford is invertible and neither white is degenerate");
    let matrix = adapt.mul(&m_d65);

    let trc = Trc::Parametric {
        func_type: 3,
        params: SRGB_TRC_PARAMS.to_vec(),
    };

    MatrixTrc::from_constructed(
        matrix,
        [trc.clone(), trc.clone(), trc],
        // The constructed profile IS D50-adapted, so its media white is
        // the PCS white. Self-consistent by construction — see above.
        Some(D50),
    )
    .expect("the sRGB colorant matrix is non-singular")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The construction's arithmetic, checked against a PUBLISHED
    /// matrix rather than against itself.
    ///
    /// Expectation source: **W3C's 1996 sRGB proposal, equation (1.8)**,
    /// which prints the sRGB→XYZ (D65) matrix at 4 decimal places. This
    /// is published literature, not this crate's output — the distinction
    /// project rule 3 turns on.
    ///
    /// Tolerance: `5e-5`, which is exactly the half-ULP of a 4-decimal
    /// printed value. It is not a tuned number: a 4-dp figure `0.4124`
    /// denotes the interval `[0.41235, 0.41245]`, so any correct
    /// computation must land inside `±5e-5` and a wrong one has no reason
    /// to. Asserting tighter would be asserting against W3C's rounding;
    /// asserting looser would stop discriminating.
    #[test]
    fn d65_matrix_matches_w3c_1996_equation_1_8() {
        let w = srgb_white_d65().unwrap();
        let m = rgb_to_xyz(&SRGB_PRIMARIES_XY, w).unwrap();
        let published = [
            [0.4124, 0.3576, 0.1805],
            [0.2126, 0.7152, 0.0722],
            [0.0193, 0.1192, 0.9505],
        ];
        for (i, (got_row, want_row)) in m.rows.iter().zip(published.iter()).enumerate() {
            for (j, (got, want)) in got_row.iter().zip(want_row.iter()).enumerate() {
                let d = (got - want).abs();
                assert!(
                    d < 5e-5,
                    "cell [{i}][{j}]: computed {got} vs W3C 1996 eq.(1.8) {want} (Δ {d:.3e})"
                );
            }
        }
    }

    /// Structural guard: the matrix must map device white `(1,1,1)` to
    /// the adopted white exactly. This is what the Grassmann solve is
    /// *for*, so it catches a transposition, a wrong column order, or a
    /// mis-scaled primary — all of which produce a matrix that still
    /// looks like a plausible RGB→XYZ matrix.
    #[test]
    fn device_white_maps_to_d65() {
        let w = srgb_white_d65().unwrap();
        let m = rgb_to_xyz(&SRGB_PRIMARIES_XY, w).unwrap();
        let got = m.apply([1.0, 1.0, 1.0]);
        for (g, e) in got.iter().zip([w.x, w.y, w.z]) {
            assert!(
                (g - e).abs() < 1e-12,
                "device white → {got:?}, expected {w:?}"
            );
        }
    }

    /// The built model's colorant sum must be D50 — i.e. the adaptation
    /// actually happened and landed where it was aimed.
    ///
    /// ★ This is the test that catches "forgot to adapt" and "adapted
    /// twice", which are the two failure modes of step 2 and which both
    /// produce a whole-image cast rather than an error.
    ///
    /// ## ★★★ DO NOT DELETE THIS AS REDUNDANT
    ///
    /// It looks redundant next to
    /// `tests/builtin_srgb_destination.rs`'s ΔE comparison, which
    /// measures the same construction against a real sRGB profile to a
    /// far more meaningful-sounding tolerance. **It is not redundant, and
    /// it is the only test in the crate with power against one whole
    /// direction of white-point error.**
    ///
    /// Measured 2026-08-17 by injecting drift into the D50 target this
    /// adaptation aims at:
    ///
    /// | drift in `Z` | the ΔE test vs the profile | **this test** |
    /// |---|---|---|
    /// | `−3.0×10⁻⁴` | FAILS | FAILS |
    /// | **`+3.0×10⁻⁴`** | **PASSES — reports `0.029` vs the correct build's `0.033`, i.e. it looks BETTER** | **FAILS** |
    ///
    /// The reason is structural. The reference profile's own white sits
    /// `+1.885×10⁻⁴` above D50 in `Z`, so drifting *upward* moves our
    /// construction **toward the reference** and shrinks the measured
    /// difference. A differential test has no power against an error that
    /// moves your answer toward the thing you are comparing to
    /// (`docs/NEXT_SESSION.md` §5.2 — the project has already paid for
    /// this once, with a black-point estimator whose buggy build agreed
    /// with lcms2 *better* than the correct one).
    ///
    /// **This test compares against `D50` itself — an absolute, sourced
    /// constant — with no file anywhere in it.** That is precisely why it
    /// still has power where the differential one does not. Removing it
    /// would open the blind spot **and every remaining test would stay
    /// green while it happened.**
    #[test]
    fn constructed_colorant_sum_is_d50() {
        let m = srgb().matrix();
        let sum = m.apply([1.0, 1.0, 1.0]);
        for (g, e) in sum.iter().zip([D50.x, D50.y, D50.z]) {
            assert!(
                (g - e).abs() < 1e-9,
                "colorant sum {sum:?} is not D50 {:?}",
                (D50.x, D50.y, D50.z)
            );
        }
    }

    /// The constructed model is self-consistent where the shipped HP
    /// profile is not: `wtpt` equals the colorant sum, so
    /// `white_point_note` stays silent.
    #[test]
    fn constructed_srgb_is_white_point_consistent() {
        assert!(
            srgb().white_point_note().is_none(),
            "the constructed sRGB should not trip the A4c disclosure"
        );
    }

    /// The TRC's linear segment is present and is the sRGB one.
    ///
    /// Expectations are the published piecewise definition evaluated at
    /// three points, computed from the *definition* rather than from this
    /// crate: below the knee `L = V/12.92`, above it
    /// `L = ((V+0.055)/1.055)^2.4`.
    ///
    /// ★ The third point is what gives this test power. Asserting only
    /// near white would pass against a pure-gamma-2.2 curve, which is the
    /// defect this test exists to catch; at `V = 0.01` the two differ by
    /// a factor of 12.
    #[test]
    fn trc_has_the_linear_segment_and_is_not_gamma_2_2() {
        let trc = &srgb().trc[0];
        // Below the breakpoint: strictly linear.
        let v = 0.01_f64;
        let expect_linear = v / 12.92;
        let got = trc.eval(v);
        assert!(
            (got - expect_linear).abs() < 1e-12,
            "at V=0.01 expected the linear segment {expect_linear:.9}, got {got:.9}"
        );
        // And it must NOT be gamma 2.2 there — the classic error.
        let gamma22 = v.powf(2.2);
        assert!(
            (got - gamma22).abs() > 5e-4,
            "at V=0.01 the curve is indistinguishable from gamma 2.2 ({gamma22:.9}) — \
             the linear segment has been lost"
        );
        // Above the breakpoint: the power segment.
        for v in [0.5_f64, 1.0] {
            let expect = ((v + 0.055) / 1.055).powf(2.4);
            let got = trc.eval(v);
            assert!(
                (got - expect).abs() < 1e-12,
                "at V={v} expected {expect:.12}, got {got:.12}"
            );
        }
    }

    /// ★★ The breakpoint is the C⁰ solution `0.04045`, **not** W3C
    /// 1996's `0.03928` — and this is the only test in the module with
    /// any power against that substitution.
    ///
    /// ## Why this test had to be written specially
    ///
    /// It was added after an **injection audit** (2026-08-17) that
    /// swapped `0.04045` for `0.03928` and ran the rest of this module's
    /// suite: **all of it passed.** Five tests, zero power. That is not
    /// a surprise in hindsight — the matrix tests do not touch the
    /// curve, and the curve test evaluated at `0.01`, `0.5` and `1.0`,
    /// none of which is inside the window where the two constants
    /// disagree. A test that cannot fail is not evidence (project rule
    /// §5.3), and a suite that documents a constant at length while
    /// being unable to detect its corruption is the worst version of
    /// that: it reads as protection.
    ///
    /// ## ★★ Where it evaluates, and why moving it would break it
    ///
    /// `V = 0.0393`, which sits **at 99.9995 % of the interior maximum**
    /// of the separation between the two candidate curves.
    ///
    /// **This is not "just inside the window", and the distinction is
    /// the whole reason this paragraph exists.** The separation is
    /// **exactly zero at `V = 0.03928`**, the window's own lower edge,
    /// because *both* constants take the linear branch there. It rises
    /// to `7.5548×10⁻⁷` at an interior peak near `V ≈ 0.039 302 447` and
    /// decays to `1.0×10⁻⁹` approaching `0.04045`.
    ///
    /// | `V` | separation | test power |
    /// |---|---|---|
    /// | `0.03928` (lower edge) | **`0`** | **none** |
    /// | `0.0393` — **used here** | `7.5548×10⁻⁷` | maximal |
    /// | `0.040449` (near upper edge) | `1.0×10⁻⁹` | near `f64` noise |
    ///
    /// ★ **So the two obvious "tidier" choices are both wrong.**
    /// Snapping this to the boundary `0.03928` gives a test with
    /// **exactly zero** power — as blind as an 8-bit vector — and moving
    /// it toward `0.04045` gives one whose margin is near `f64`
    /// comparison noise. Neither failure would be visible: the test would
    /// still pass, still read as protection, and still catch nothing.
    /// **Do not move this number.** The `separation > 5e-7` guard below
    /// is what enforces that, and it is why the guard asserts on the
    /// *separation* rather than only on the output.
    ///
    /// ## Candidate separation
    ///
    /// - **observed** (breakpoint `0.04045`, linear branch):
    ///   `0.0393 / 12.92`
    /// - **named alternative** (breakpoint `0.03928`, power branch):
    ///   `((0.0393 + 0.055) / 1.055)^2.4`
    /// - **separation:** `7.55×10⁻⁷`, versus a tolerance of `1×10⁻¹²`.
    ///   The test therefore has ~`7.6×10⁵`× margin over its own
    ///   threshold, so a pass is a real discrimination and not an
    ///   accident of rounding.
    ///
    /// Expectation source: the **published piecewise definition** (W3C
    /// CSS Color 4; W3C 1996 eq. 1.2), evaluated independently — not
    /// produced by the function under test.
    ///
    /// Tolerance `1e-12`: this is exact `f64` arithmetic on the same two
    /// operations the implementation performs, so the only admissible
    /// difference is last-bit rounding. It is not tuned — it was chosen
    /// as "tight enough that only bit-level noise fits", and the
    /// separation above shows it is nowhere near the discrimination
    /// boundary.
    #[test]
    fn breakpoint_is_the_c0_solution_not_the_1996_value() {
        let trc = &srgb().trc[0];
        // Inside (0.03928, 0.04045), at the maximum-separation end.
        let v = 0.0393_f64;

        // What the correct breakpoint requires: V <= 0.04045, so the
        // LINEAR segment applies.
        let observed_expected = v / 12.92;
        // What the 1996 breakpoint would give: V > 0.03928, so the
        // POWER segment applies instead.
        let alternative = ((v + 0.055) / 1.055).powf(2.4);

        let separation = (observed_expected - alternative).abs();
        assert!(
            separation > 5e-7,
            "the two candidate breakpoints are separated by only {separation:.3e} at V={v}; \
             this test has lost its discrimination and must be moved, not relaxed"
        );

        let got = trc.eval(v);
        assert!(
            (got - observed_expected).abs() < 1e-12,
            "at V={v} (inside the 0.03928/0.04045 disagreement window) expected the linear \
             segment {observed_expected:.15}, got {got:.15}. The 1996 value 0.03928 would give \
             {alternative:.15} — if that is what came back, the breakpoint has been replaced by \
             the C¹ solution, which is a DIFFERENT CURVE, not a more precise sRGB."
        );
    }

    /// ★ The measured claim that no 8-bit code can detect a wrong
    /// breakpoint — asserted, so the module doc cannot go stale.
    ///
    /// This corrects a claim carried in the project's standards corpus
    /// (that the error "affects 8-bit codes 10 and 11"). Both candidate
    /// breakpoints take the **same branch** for all 256 codes, so the
    /// separation over the whole 8-bit domain is exactly zero.
    ///
    /// The test is deliberately written as a statement about the
    /// *constants*, not about our output: it is checkable by anyone with
    /// a calculator and does not depend on this crate being right.
    #[test]
    fn no_eight_bit_code_lies_between_the_two_candidate_breakpoints() {
        let (lo, hi) = (0.03928_f64, 0.04045_f64);
        let offenders: Vec<u8> = (0u8..=255)
            .filter(|&c| {
                let v = f64::from(c) / 255.0;
                v > lo && v < hi
            })
            .collect();
        assert!(
            offenders.is_empty(),
            "8-bit codes {offenders:?} fall between the candidate breakpoints; the module doc's \
             claim that 8-bit input cannot distinguish them is no longer true"
        );
        // And the neighbours that bracket the window, named so the
        // reason is legible rather than merely asserted.
        assert!(
            10.0 / 255.0 < lo,
            "code 10 should sit below both breakpoints"
        );
        assert!(
            11.0 / 255.0 > hi,
            "code 11 should sit above both breakpoints"
        );
    }

    /// ★★★ **Rule 4's named approximation, against ICC's own PUBLISHED
    /// colorants** — `published-ground-truth`, the strongest evidence
    /// class available to this model.
    ///
    /// ## Expectation source
    ///
    /// **"How to interpret the sRGB color space (specified in IEC
    /// 61966-2-1) for ICC profiles"**, Jack Holm, International Color
    /// Consortium, 2015-04-27, §B.2. It prints the D50-adapted
    /// matrix — the one "which can be used for ICC Matrix/TRC sRGB
    /// profiles" — at **15 decimal places**. Its columns are `rXYZ`,
    /// `gXYZ`, `bXYZ`.
    ///
    /// This is a **standards body publishing values for its own format**,
    /// not an implementation's output and not a file's tag. It is
    /// therefore *ground truth* in the sense project rule 3 means, and
    /// strictly stronger than either of the two classes this model could
    /// previously be checked against.
    ///
    /// ## ★★★ Ground truth for the CONSTRUCTION, not for the colorimetry
    ///
    /// **This rider is mandatory and must travel with any quotation of
    /// this row.** ICC is the definer of *the artifact* — sRGB itself has
    /// no D50 colorants; only an ICC profile's encoding of sRGB does — so
    /// ICC's values are authoritative for **what an sRGB ICC profile
    /// should contain**. They are not thereby the more accurate
    /// colorimetry, and on the evidence they are the less accurate:
    ///
    /// - ICC's §A.7 matrix **descends from W3C 1996's 4-decimal print**
    ///   (`inv(§A.7)` reproduces that 4-dp matrix to `4.7×10⁻⁸`, and
    ///   re-inverting and rounding to 7 dp reproduces all nine cells of
    ///   §A.7). iccce builds its D65 matrix **exactly** from BT.709-6's
    ///   chromaticities.
    /// - ICC's published `chad` **does not map D65 onto D50** — it maps
    ///   the *rounded* white `(0.9505, 1.0000, 1.0890)` that §A.4 states,
    ///   which is why it misses ICC's own stated D50 by `≈4.9×10⁻⁵`.
    ///
    /// **So this test asserts INTEROPERABILITY with ICC's published
    /// encoding, not accuracy against it.** Do not let it be written as
    /// the latter — that would be the exact claim-inflation this project
    /// exists to prevent, and it would be inflating in the wrong
    /// direction.
    ///
    /// ## ★★ And a related trap: ICC's `chad` is NOT ICC.1's Bradford
    ///
    /// Eigendecomposing the published `chad` recovers a cone matrix whose
    /// `M_A[0][0]` is **`0.8950`**. **ICC.1:2022 Annex E.3 Eq. (E.1)
    /// prints `0.8951`.** Exact reconstruction confirms it: `0.8951`
    /// leaves `5.66×10⁻⁶` (0.371 ULP), `0.8950` leaves `5.7×10⁻¹⁶` —
    /// exact. The discriminating digit was in the corpus all along, as
    /// the row-sum check: E.1's first row sums to `1.0001`, the recovered
    /// one to `1.0000`.
    ///
    /// It is below one `s15Fixed16` step so no colour changes, **but the
    /// statement "recompute Annex E.3's Bradford and you get ICC's
    /// recommended `chad`" is false.** iccce uses E.3's `0.8951`, which
    /// is why the two differ at all.
    /// *(Established by `icc-spec-librarian`, 2026-08-17.)*
    ///
    /// ★ The document was obtained by the operator on 2026-08-17 from
    /// `color.org`, whose terms bar automated retrieval. Held at
    /// `ICC_Spec/_sources/srgb_bt709/`.
    ///
    /// ## ★★ What this test REPLACED, and why that matters
    ///
    /// It replaces one that asserted a ~12 ULP `bXYZ.Z` residual against
    /// the *shipped HP profile* and called that residual **iccce's
    /// approximation**. Measured against ICC's published values, the
    /// ownership inverts:
    ///
    /// | | worst cell | `bXYZ.Z` |
    /// |---|---|---|
    /// | this construction | **3.02 ULP** | **0.90 ULP** |
    /// | the shipped file | **11.13 ULP** | **11.13 ULP** |
    ///
    /// **The residual was the file's, not ours.** The old test was not
    /// wrong about the number — it was wrong about whose it was, and it
    /// would have gone on certifying a defect in somebody else's artifact
    /// as a cost of ours indefinitely.
    ///
    /// ## Tolerance, and why 4 ULP
    ///
    /// `4.0` ULP of `s15Fixed16` (`6.1×10⁻⁵` XYZ). **Derived, not
    /// fitted:** the entire difference is accounted for by ICC starting
    /// from their §A.7 XYZ→RGB matrix *as printed to 7 decimals* while
    /// iccce builds the D65 matrix exactly from BT.709-6 chromaticities.
    /// A 7-decimal print of a matrix whose cells are of order 1–3 admits
    /// a half-ULP of `5×10⁻⁸` per cell, which propagates through the
    /// inversion and the adaptation to the few-ULP level observed. The
    /// bound sits just above the **measured `3.02`** so a real change in
    /// the construction moves it, and it is far below the `11.13` the
    /// shipped file exhibits — **so this test also distinguishes our construction
    /// from the file's, which is the discrimination that matters.**
    ///
    /// This needs no corpus and cannot skip: the expectation is
    /// transcribed from a document, not read from a file.
    #[test]
    fn matches_icc_published_colorants_within_stated_ulps() {
        // ICC 2015, §B.2 — columns are rXYZ, gXYZ, bXYZ.
        const PUBLISHED: [[f64; 3]; 3] = [
            [
                0.436_030_342_570_117,
                0.385_101_860_087_134,
                0.143_067_806_654_203,
            ],
            [
                0.222_438_466_210_245,
                0.716_942_745_571_917,
                0.060_618_777_416_563,
            ],
            [
                0.013_897_440_074_263,
                0.097_076_381_494_207,
                0.713_926_257_896_652,
            ],
        ];
        const ULP: f64 = 1.0 / 65536.0;
        const TOLERANCE_ULPS: f64 = 4.0;

        // ★ Quote the PAIR, never the bound alone: "within 4 ULP" reads
        // as a number someone chose; "3.02 observed against a 4 ULP
        // bound" is a tolerance with its margin visible. Printed every
        // run so the margin cannot become folklore.
        let ours = srgb().matrix();
        let mut worst = 0.0_f64;
        let mut worst_cell = (0, 0);
        for (i, (ours_row, pub_row)) in ours.rows.iter().zip(PUBLISHED.iter()).enumerate() {
            for (j, (o, p)) in ours_row.iter().zip(pub_row.iter()).enumerate() {
                let d = (o - p).abs() / ULP;
                if d > worst {
                    worst = d;
                    worst_cell = (i, j);
                }
            }
        }
        println!(
            "constructed sRGB vs ICC's published colorants: {worst:.4} ULP worst              (cell {worst_cell:?}), bound {TOLERANCE_ULPS} ULP"
        );
        assert!(
            worst <= TOLERANCE_ULPS,
            "constructed sRGB colorants differ from ICC's PUBLISHED values by {worst:.2} ULP at              cell {worst_cell:?}, over the {TOLERANCE_ULPS} ULP bound. The expectation is ICC's              own document, not a file — if this fails, the CONSTRUCTION is what to check."
        );

        // ★ The discrimination check: the bound must also be tight
        // enough to tell our construction apart from the shipped
        // profile's colorants, which miss the published values by
        // ~11.13 ULP in bXYZ.Z. Without this, a loose bound would pass
        // for both and the test would not distinguish the two lineages.
        const FILE_BLUE_Z: f64 = 0.714_096_069_335_937_5;
        let file_residual = (FILE_BLUE_Z - PUBLISHED[2][2]).abs() / ULP;
        assert!(
            file_residual > TOLERANCE_ULPS,
            "the shipped profile's bXYZ.Z is {file_residual:.2} ULP from ICC's published value,              which no longer exceeds this test's {TOLERANCE_ULPS} ULP bound — the test has              stopped discriminating between our construction and the file's"
        );
    }
}
