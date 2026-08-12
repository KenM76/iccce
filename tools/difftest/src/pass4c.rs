//! # Pass 4c — ICC-absolute through a LUT destination, with the `wtpt`
//! substitution held at zero
//!
//! Read `tools/difftest/README.md` **§19** for the narrative; this file is the
//! apparatus. It closes the last measurement item of Pass 4 and it exists to
//! answer one question that had been deferred through eight filings:
//!
//! > **Does iccce's ICC-absolute arithmetic agree with lcms2's, when the one
//! > known policy difference between the two is not in the way?**
//!
//! ## Why the question could not previously be asked
//!
//! Pass 4 measured `USWebCoatedSWOP.icc → sRGB` at ICC-absolute and found
//! **11.217 ΔE2000** (**NC-053**). The mechanism was established by reading
//! lcms2's source at pin `21c582a`: `cmsio1.c`'s `_cmsReadMediaWhitePoint`
//! substitutes D50 for a profile's stored `wtpt` when the profile is **both**
//!
//! ```text
//! cmsGetEncodedICCversion(hProfile) < 0x4000000   /* version 2.x */
//! && cmsGetDeviceClass(hProfile) == cmsSigDisplayClass  /* 'mntr' */
//! ```
//!
//! The system sRGB profile is v2.1.0 `'mntr'` and stores `wtpt` = D65
//! (0.9505, 1.0000, 1.0891), so at ICC-absolute the two implementations were
//! building the same scaling diagonal out of **different destination media
//! whites** — a 32 % error in `Z`, applied to every colour.
//!
//! That made NC-053 unusable as evidence about the *arithmetic*: the residual
//! it reports is a **policy** difference, and it swamps anything else by two
//! orders of magnitude. The only gate the ICC-absolute path had was
//! **NC-054**, which grades a *model* — lcms2 re-predicted with the
//! substitution and the CLUT geometry both emulated. A model can absorb a
//! genuine arithmetic error in iccce's absolute path along with the policy
//! difference it was built to isolate, and nothing in the suite could tell.
//!
//! ## The clause, cited in the edition-stable form
//!
//! The normative statement is **`ICC.1:2022` 6.3.2.2, Equations (4)–(6)**,
//! restated verbatim at **D.6.1 Equation (D.7)**:
//!
//! ```text
//! Xa = (Xmw / Xi) Xr        Xr, Yr, Zr  media-relative (PCSXYZ)
//! Ya = (Ymw / Yi) Yr        Xa, Ya, Za  ICC-absolute (nCIEXYZ)
//! Za = (Zmw / Zi) Zr        Xmw..Zmw    mediaWhitePointTag, ADAPTED
//!                           Xi..Zi      PCS white, FIXED at
//!                                       (0,964 2 · 1,0 · 0,824 9)
//! ```
//!
//! D.6.1 calls it *"a simple scaling operation"*: three independent scalars, a
//! diagonal, **no cross-channel term** — it is not a chromatic adaptation and
//! must not be replaced by one. `Xi` is a constant from Table D.1 / Table 14,
//! **not** the header's PCS-illuminant field; reading it from the header at
//! runtime turns a header defect into a colour defect.
//!
//! **★ Citation hazard, and the reason this module does not say "D.6/D.7"
//! bare.** The label is **not edition-stable**. In `ICC.1:2001-04` Annex D the
//! equations are (D.1)–(D.6) — there is **no (D.7)**, and that edition's (D.6)
//! is the single `Z` component of the *inverse*. A v2-context "(D.6)" cites a
//! different equation from a v4-context one. Since this module is entirely
//! about a v2 file's `wtpt`, the ambiguity was live here.
//!
//! Sourced by `icc-spec-librarian`, 2026-08-12, from
//! `icc__s__rendering_intents.md` §3.1–§3.4 (`evidence: primary_spec`).
//!
//! ## ★ What the standard does and does NOT say about lcms2's substitution
//!
//! Sourced 2026-08-12, and it is sharper than "the standard is silent":
//!
//! - **`ICC.1:2022` 9.2.36** gates the `wtpt` rule on **device class**, with
//!   **no version gate**: *"For displays, the values specified shall be those
//!   of the PCS illuminant."*
//! - **`ICC.1:2001-04` A.3.1.1** gates it on the **adaptation condition**, not
//!   on class at all: *"If the viewer completely adapts to the white point of
//!   the medium (as is often the case with monitors) this tag **should** be set
//!   to Xi, Yi, Zi."* Monitors are the typical case, not the condition.
//! - So **lcms2's `version < 4 && class == 'mntr'` predicate reproduces no
//!   clause in either edition** — it applies v4's class gate, plus a version
//!   gate v4 does not have, to v2, where the rule is not class-gated.
//! - Both clauses address the profile's **author**. The conformance clause
//!   (`ICC.1:2022` clause 5, `ICC.1:2001-04` clause 3) binds the ability to
//!   **read** profiles. **A CMM's computed output is not constrained by
//!   either.** Corollary, and it binds every document in this project: say
//!   lcms2 **diverges**, never that it is *non-conforming*.
//! - A.3.1.1 is a `should`, and `ICC.1:2001-04` has no defined verbal-form
//!   hierarchy — deontic weight **qualified**, not `shall`.
//!
//! The corpus's own action line for this project, verbatim: *"iccce action:
//! keep `NA-007` (use `wtpt` as stored) and REPORT the mismatch; do not adopt
//! lcms2's substitution."*
//!
//! ## What this section does instead: choose a pair where the gate cannot fire
//!
//! The substitution is conditioned on a **conjunction**. Break either half on
//! **both** profiles and the policy difference is not modelled, not
//! subtracted, not tolerated — it is **structurally absent**, because lcms2's
//! own branch is not taken.
//!
//! | role | profile | version | class | gate | why it fails |
//! |---|---|---|---|---|---|
//! | source | `fixtures/synthetic/v4-rgb-matrix-trc.icc` | **4.4.0** | `'mntr'` | **not taken** | fails the *version* half |
//! | destination | `USWebCoatedSWOP.icc` | 2.1.0 | **`'prtr'`** | **not taken** | fails the *class* half |
//!
//! Each profile fails a *different* half of the conjunction, which is worth
//! more than two profiles failing the same half: it means the pair is not
//! quietly relying on one property.
//!
//! §A grades that conjunction as a **precondition**, at exactly zero, read
//! from the parsed headers of the two files actually opened. It is graded
//! rather than asserted in a comment because the entire section's claim rests
//! on it, and a profile can be replaced on a machine.
//!
//! ## The second confound, and why it is also zero here
//!
//! Pass 4b established (`_cmsReadOutputLUT`) that lcms2 forces **trilinear**
//! interpolation for any Lab-PCS output LUT, and trilinear over three inputs
//! **is** `iccce-cmm`'s n-linear. So NA-006's interpolation-geometry cost is
//! identically zero on the destination side of this pair. The source is
//! matrix/TRC and has **no CLUT at all**. Neither of Pass 4's two large known
//! divergences is present.
//!
//! ## The counterfactual is EXACT, not modelled — and it is the NC-053
//! mechanism itself
//!
//! This is the part that makes the section evidence rather than a reassuring
//! number. Ask DL-025's question: *what would this comparison return if the
//! effect were identically absent?*
//!
//! The source's stored `wtpt` **is** D50 exactly (it is a synthetic fixture;
//! `tools/gen-profiles` writes the PCS illuminant). So in the composite chain
//! — 6.3.2.2 Eq (4)–(6) forward on the source, its inverse Eq (1)–(3) on the
//! destination, giving the `mw_src / mw_dst` composite —
//!
//! ```text
//! XYZ_abs      = XYZ_rel_src × (WP_src / D50)      ← identity here
//! XYZ_rel_dst  = XYZ_abs     × (D50 / WP_dst)      ← the live term
//! ```
//!
//! For this destination the live term is `Xi/Xmw` =
//! **(1,361 095 · 1,358 880 · 1,444 658)** — derived twice, `f64` and 30-digit
//! decimal, by `icc-spec-librarian` from the file's own stored `wtpt`. A 36 %
//! brightening, which is why the effect is large enough to measure rather than
//! a rounding perturbation.
//!
//! The source factor is already identity, and **if lcms2 had substituted D50
//! for `WP_dst` the destination factor would become identity too** — and the
//! ICC-absolute output would equal the media-relative output *exactly*.
//!
//! Therefore `iccce absolute vs iccce media-relative`, on this pair and this
//! grid, is **not an approximation of the counterfactual — it is the
//! counterfactual**, computed rather than assumed, and it is precisely the
//! size the NC-053 substitution would have had here. Measured: **2.0558×10⁻¹
//! device units**, against an observed iccce-vs-lcms2 residual of
//! **8.90×10⁻⁵**. The comparison can see the effect it is claiming to have
//! ruled out, by a factor of ~2300.
//!
//! The degeneracy guard sits next to it: a point where the absolute scaling
//! moved nothing carries no evidence either way, and §A counts them. **One of
//! 729** — device black, which is the fixed point of any diagonal scaling and
//! is arithmetic rather than a defect.
//!
//! ## §B — the same policy, measured in the OTHER direction
//!
//! DL-021: *a behaviour is a fact about one direction and one path.* NC-053
//! measured the substitution with the v2 `'mntr'` profile as **destination**.
//! §B measures it with the same profile as **source** — `sRGB → SWOP` at
//! ICC-absolute — where the gate fires on `WP_src` instead.
//!
//! **The prediction was written before the run** (DL-023): the divergence
//! should move to the source side and stay **large**, order 10⁻¹ device units,
//! because iccce scales by `D65/D50 = (0.98579, 1.0, 1.32027)` where lcms2
//! scales by identity. Had it come out small, the mechanism as recorded in
//! NC-053 would have been wrong about its own generality, and that would have
//! been the more interesting result — this project has already once predicted
//! an lcms2 divergence and measured it **absent** (DL-011 → DL-012).
//!
//! It did not come out small: **2.134×10⁻¹ device units**, against a
//! media-relative floor of **1.29×10⁻⁴** on the same pair and grid — a factor
//! of **1650**. The policy is direction-symmetric.
//!
//! §B's absolute row is **REPORTED, NOT GRADED**. Its media-relative twin is
//! graded, and it is what makes §B's magnitude meaningful: without a floor
//! measured on the *same two files*, "0.21 device units" is a number with
//! nothing to be large compared to.
//!
//! ## What this section does NOT claim
//!
//! - **Not that iccce's absolute arithmetic is correct.** Both rows are
//!   cross-checks. Two implementations can read 6.3.2.2 the same way and both
//!   be wrong. There is no published value for this transform, and Pass 4c
//!   does not create one. (`ICC.1:2022` Table D.2 *does* print an nCIEXYZ
//!   media white for SWOP — 0,706 7 · 0,734 6 · 0,570 3, within ~2×10⁻³ of
//!   this file's — but that is a published value for a **white point**, not
//!   for a transform, and a different characterization revision besides. It is
//!   not promoted to ground truth here.)
//! - **Not that lcms2 is non-conforming.** That verdict is not available:
//!   the conformance clause binds **reading**, not computed output. What the
//!   sourcing above supports is narrower and is all this section claims —
//!   lcms2's predicate **reproduces no clause in either edition**, and the
//!   clauses that do exist address the profile's author. This section
//!   *avoids* the disagreement so the arithmetic can be seen; it does not
//!   adjudicate it, and no measurement in it could.
//! - **Nothing about A4c.** Whether a profile's `wtpt` must agree with its own
//!   colorants is a separate ambiguity, still **SILENT**, and it did **not**
//!   clear when A4b cleared. The system sRGB profile is exactly such a
//!   self-inconsistent file and this section neither adjudicates nor repairs
//!   it.
//! - **Not that the source side of the absolute scaling is exercised.** It is
//!   identity by construction in §A — that is what buys the exact
//!   counterfactual, and it is a cost as well as a benefit. §A measures the
//!   **destination-side** term only, which is the term NC-053 got wrong.
//!   §B exercises the source-side term, but with the policy live, so it
//!   cannot grade it.
//! - **One machine, one pin, one grid, one pair per section, Windows/MSVC.**

use std::path::{Path, PathBuf};

use iccce_profile::Profile;

use crate::{
    Bpc, DiffError, Iccce, Intent, Kind, Metric, Oracle, Precalc, Record, Request, Space, Tolerance,
};

// ===========================================================================
// The profiles
// ===========================================================================

/// The confound-free source: v4.4.0, `'mntr'`, RGB → XYZ PCS, matrix plus
/// `para` funcType 0 (γ = 2.0) TRCs, `wtpt` = D50 exactly.
///
/// It is `'mntr'` — the *class* half of lcms2's gate is satisfied. What
/// defeats the gate is the **version**. That is deliberate: a pair that
/// defeated the gate twice over on the same half would be a weaker
/// demonstration, because a single change to lcms2's version predicate would
/// invalidate both halves at once.
pub fn v4_matrix_src() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/synthetic/v4-rgb-matrix-trc.icc")
}

/// The LUT destination for both sections: v2.1.0, `'prtr'`, CMYK, `Lab ` PCS,
/// `B2A1` = `mft1` (`lut8Type`), 3 → 4, 33³ grid, `wtpt` =
/// (0.7084, 0.7359, 0.5710).
///
/// Note `Y = 0.7359`, not 1.0 — it is a paper white, not an illuminant, which
/// is what makes the ICC-absolute diagonal on this destination large enough to
/// measure rather than a rounding perturbation.
pub const SWOP: &str = r"C:\Windows\System32\spool\drivers\color\USWebCoatedSWOP.icc";

/// The policy-exposed source for §B: v2.1.0, `'mntr'`, `wtpt` = D65 as stored.
/// **Both** halves of lcms2's gate are satisfied, so the substitution fires.
/// This is the same file that produced NC-053.
pub const SRGB: &str = r"C:\Windows\System32\spool\drivers\color\sRGB Color Space Profile.icm";

// ===========================================================================
// Tolerances
// ===========================================================================

/// **Reused from `pass4b::DEVICE_B2A`, unchanged, deliberately.**
///
/// Pass 4c's graded device rows end in the *same destination table* as Pass
/// 4b's §A — SWOP's `B2A1`, an `mft1`/`lut8`, 3 → 4, 33³ — reached by the same
/// evaluator through the same 8-bit CLUT. The quantisation envelope that
/// justifies `5×10⁻⁴` is a property of that pipeline, not of the source that
/// feeds it, so the constant transfers with its justification intact.
///
/// **Minting a new number here would have been the wrong move**, and worse,
/// the tempting one: a fresh constant fitted to a fresh observation is a
/// tolerance chosen because it passed. `TOLERANCES.md` §3.4.4.6 set the
/// precedent when the saturation table was added — five rows reused this
/// constant and only the `why` string moved, *toward* the observation, while
/// the value stayed put.
const DEVICE_B2A: Tolerance = Tolerance::new(
    5e-4,
    "REUSED UNCHANGED from pass4b::DEVICE_B2A. Same destination table (SWOP B2A1, mft1/lut8, \
     3->4, 33 points, 8-bit), same evaluator, same direction, so the same quantisation envelope \
     applies: tabulated curves rounded to 1/65535 in and out, CLUT input rounded to u16, CLUT \
     output returned as u16/65535, measured at 1.5525e-4 (saturation, the steepest table), \
     1.330e-4 (media-relative) and 9.602e-5 (perceptual) in Pass 4b, plus headroom for lcms2's \
     16-bit FIXED-POINT curve and CLUT interpolation which the f64 model does not reproduce. \
     A NEW constant fitted to Pass 4c's own observation would have been a number chosen because \
     it passed",
);

/// The precondition: **how many of the pair's two profiles satisfy lcms2's
/// substitution predicate.** Graded at exactly zero.
///
/// Zero is not a rounding allowance here and needs none — it is a count of
/// profiles, read from two parsed headers. One is the whole claim of §A
/// collapsing, and there is no arithmetic that could make it *nearly* zero.
const GATE_NOT_TRIPPED: Tolerance = Tolerance::new(
    0.0,
    "a COUNT, over the two profiles of the pair, of those satisfying BOTH halves of lcms2's \
     _cmsReadMediaWhitePoint predicate (encoded version < 0x4000000 AND device class == 'mntr'), \
     read from the parsed headers of the files actually opened. Section A's entire claim is that \
     the substitution is STRUCTURALLY absent rather than modelled or tolerated; if either \
     profile trips the gate that claim is false and every number below it is measuring the \
     policy again. Exactly 0 because it is a count of files, not a float",
);

/// The sensitivity floor, expressed as a **violation** quantity so it grades
/// against zero like every other row (the idiom `pass6` uses for its
/// convergence band).
///
/// `max(0, 100 − ratio)`, where `ratio` is the exact counterfactual divided by
/// the observed residual.
///
/// **Why 100.** Pass 4b's B2A counterfactual ratios — the same direction, the
/// same destination table, a control of the same shape — landed at **99×, 139×
/// and 191×**, and were accepted as demonstrating that the comparison can see
/// a geometry difference. A comparison that cannot separate the substitution
/// from the arithmetic by at least the margin an already-accepted control
/// achieved is not entitled to conclude that the arithmetic agrees. The number
/// is transcribed from a measured band, not chosen to clear the observation:
/// the observation is **2310×**, twenty-three times the floor, and the floor
/// would be the same had the observation been 105×.
const SENSITIVITY_FLOOR: Tolerance = Tolerance::new(
    0.0,
    "violation max(0, 100 - r) on r = counterfactual / observed, where the counterfactual is \
     EXACT rather than modelled: because the source's stored wtpt IS D50, substituting D50 for \
     the destination's wtpt would collapse the whole 6.3.2.2 scaling diagonal to identity, so \
     'absolute vs media-relative on this pair' IS what lcms2's substitution would have cost \
     here. The floor of 100 is transcribed from Pass 4b's accepted counterfactual band of \
     99x-191x on this same table and direction. DL-025: an instrument is only as good as its \
     fixture, and this row is what stops 8.9e-5 from being a measurement of nothing",
);

/// A point the absolute scaling did not move carries no evidence about the
/// absolute scaling. Graded as a **fraction** of the grid.
///
/// **Why 0.05.** A diagonal scaling fixes the origin, so device black is
/// expected to be unmoved and is not a defect — one point in 729 is
/// 1.4×10⁻³. The budget is set an order of magnitude above that single
/// expected fixed point and far below any level at which gamut clipping would
/// be pinning the grid. This is the guard against the *other* way §A could be
/// a measurement of nothing: if the ICC-absolute scaling pushed most of the
/// grid outside the destination gamut, both implementations would clamp to the
/// same boundary and agree perfectly while computing nothing.
const DEGENERACY_GUARD: Tolerance = Tolerance::new(
    0.05,
    "the FRACTION of grid points on which iccce's absolute output equals its media-relative \
     output to within 1e-9, i.e. points the absolute scaling did not move and which therefore \
     carry no evidence either way. A diagonal scaling fixes the origin, so device black is \
     expected here and is arithmetic, not a defect: 1/729 = 1.4e-3. 0.05 sits an order of \
     magnitude above that one expected fixed point and far below any level at which gamut \
     clipping would be pinning the grid and manufacturing agreement",
);

// ===========================================================================
// The grid
// ===========================================================================

/// A deterministic 9×9×9 grid on the **8-bit RGB lattice**, 729 points.
///
/// Integer codes, so both implementations receive a value each can represent
/// exactly; the driver hands `transicc` the integer in its own 0–255
/// convention and hands `iccce transform` the f64 quotient, so no rounding
/// asymmetry is introduced by the harness itself. A grid of fractional codes
/// would put a parsing difference into the residual and call it colour.
///
/// 9 levels rather than Pass 4b's 213-point structured grid because §A's
/// subject is a *global multiplicative* term — a diagonal scaling touches
/// every colour equally, so uniform coverage of the cube is the right sampling
/// and there is no boundary feature to concentrate on. The count is stated in
/// every record's `detail`.
pub fn rgb_grid() -> Vec<[u32; 3]> {
    const LEVELS: [u32; 9] = [0, 32, 64, 96, 128, 160, 192, 224, 255];
    let mut g = Vec::with_capacity(729);
    for r in LEVELS {
        for gg in LEVELS {
            for b in LEVELS {
                g.push([r, gg, b]);
            }
        }
    }
    g
}

// ===========================================================================
// Analysis
// ===========================================================================

/// One direction's worth of numbers. Every field is a reduction whose metric
/// is named where it is graded; nothing here is a bare "error".
#[derive(Debug, Clone)]
pub struct AbsAnalysis {
    /// One line naming both files' version, class, colour space, PCS and
    /// stored `wtpt`, built from the parsed headers rather than from these
    /// doc comments.
    pub structure: String,
    /// How many of the two profiles satisfy lcms2's substitution predicate,
    /// read as the **conjunction** the source actually contains.
    pub gate_count: f64,
    /// ★ The same count under the reading of ICC.1:2022 **9.2.36** — class
    /// only, **no version gate** — which is what the *standard* says and what a
    /// careful reader transcribing the specification rather than the code would
    /// have implemented. Carried so the precondition row can state what it
    /// would have observed under that reading instead of asserting that it
    /// does not matter. See [`section_a_records`]'s separation.
    pub gate_count_class_only: f64,
    /// The same count with lcms2's `&&` read as `||`. The third live reading,
    /// and the one §B is sensitive to.
    pub gate_count_disjunction: f64,
    /// iccce vs lcms2 at ICC-absolute, max over components and rows,
    /// normalised device units.
    pub abs_max: f64,
    pub abs_mean: f64,
    /// The same comparison at media-relative — the floor for this pair, this
    /// grid, this destination table.
    pub rel_max: f64,
    pub rel_mean: f64,
    /// iccce absolute vs iccce media-relative. In §A this is the **exact**
    /// counterfactual; in §B it is only the size of the absolute effect on
    /// one side, and §B's records say so.
    pub counterfactual: f64,
    /// The same reduction as a **mean**, so the mean row can state its own
    /// separation in its own metric instead of borrowing the max's. Quoting a
    /// max's counterfactual on a mean row would be the population error Pass 6
    /// row R4 records.
    pub counterfactual_mean: f64,
    /// Fraction of grid points the absolute scaling did not move.
    pub unmoved_fraction: f64,
    /// Index into [`rgb_grid`] of the worst absolute-intent point, with both
    /// implementations' output there.
    pub worst_index: usize,
    pub worst_iccce: Vec<f64>,
    pub worst_lcms2: Vec<f64>,
}

impl AbsAnalysis {
    /// The sensitivity ratio, guarded against a zero denominator. A residual
    /// of exactly zero would be a harness failure (two subprocesses cannot
    /// agree to the last bit through a 4-decimal print floor), so it is
    /// reported as infinity rather than silently dividing.
    pub fn ratio(&self) -> f64 {
        if self.abs_max == 0.0 {
            f64::INFINITY
        } else {
            self.counterfactual / self.abs_max
        }
    }
}

/// Why a section could not run. Mirrors `pass4b::Unavailable`: a missing
/// system profile is a **skip** (LEGAL.md §3 category (c) — read locally,
/// never committed, never a required input), anything else is an error.
#[derive(Debug, Clone)]
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
    fn from(e: DiffError) -> Unavailable {
        Unavailable::Error(e.to_string())
    }
}

/// Does this profile satisfy **both** halves of lcms2's `wtpt` substitution
/// predicate?
///
/// Transcribed from `cmsio1.c`'s `_cmsReadMediaWhitePoint` at pin `21c582a`:
///
/// ```c
/// if (cmsGetEncodedICCversion(hProfile) < 0x4000000 &&
///     cmsGetDeviceClass(hProfile) == cmsSigDisplayClass) {
///     ...  /* returns D50 whatever the tag says */
/// }
/// ```
///
/// **This is a transcription of an implementation, not of a standard**, and it
/// is a source reading until something measures it — which §B does, from the
/// other direction, at 1650× the floor. It is also the first thing the pin
/// moving invalidates.
fn trips_lcms2_wtpt_gate(p: &Profile) -> bool {
    const DISPLAY_CLASS: u32 = 0x6D6E_7472; // 'mntr'
    p.header.version.raw < 0x0400_0000 && p.header.device_class.0 == DISPLAY_CLASS
}

/// The **class** half of the predicate on its own — ICC.1:2022 **9.2.36**'s
/// reading, which is class-gated with no version gate at all.
///
/// Split out rather than inlined because it is the difference between the two
/// readings the precondition rows price against each other, and a rival that
/// exists only inside an expression is a rival nobody can find.
fn is_display_class(p: &Profile) -> bool {
    const DISPLAY_CLASS: u32 = 0x6D6E_7472; // 'mntr'
    p.header.device_class.0 == DISPLAY_CLASS
}

/// Format a profile's `wtpt` for the structure line, or say it is absent.
/// Read from the file, never from this module's prose.
fn wtpt_of(p: &Profile) -> String {
    const WTPT: u32 = 0x7774_7074; // 'wtpt'
    use iccce_profile::tag_types::TagData;
    match p.tags.iter().find(|t| t.sig.0 == WTPT) {
        None => "absent".to_string(),
        Some(t) => match p.decode_tag(t) {
            Some(Ok(d)) => match &d.data {
                TagData::Xyz(v) if !v.is_empty() => {
                    format!(
                        "({:.4}, {:.4}, {:.4})",
                        v[0].x.to_f64(),
                        v[0].y.to_f64(),
                        v[0].z.to_f64()
                    )
                }
                _ => "not-an-XYZ-tag".to_string(),
            },
            _ => "undecodable".to_string(),
        },
    }
}

/// Run one source → SWOP pair at ICC-absolute and at media-relative, on the
/// same grid, through both implementations.
///
/// Both sides cross a process boundary. That is not incidental: an in-process
/// call on iccce's side would make the two arms asymmetric, and this section's
/// whole content is a difference between two arms.
fn analyse(oracle: &Oracle, src_path: &Path, dst_path: &Path) -> Result<AbsAnalysis, Unavailable> {
    for p in [src_path, dst_path] {
        if !p.exists() {
            return Err(Unavailable::Skip(format!(
                "profile not present on this machine: {} (LEGAL.md §3 category (c): read \
                 locally, never committed, never a required input)",
                p.display()
            )));
        }
    }

    let src_bytes = std::fs::read(src_path).map_err(|e| Unavailable::Error(e.to_string()))?;
    let dst_bytes = std::fs::read(dst_path).map_err(|e| Unavailable::Error(e.to_string()))?;
    let src = Profile::parse(&src_bytes).map_err(|e| Unavailable::Error(e.to_string()))?;
    let dst = Profile::parse(&dst_bytes).map_err(|e| Unavailable::Error(e.to_string()))?;

    // ★ The precondition, computed BEFORE anything is converted. If it is not
    // zero, the numbers below are measuring the policy again and the section
    // says so on its own face rather than in a footnote.
    let gate_count = f64::from(u8::from(trips_lcms2_wtpt_gate(&src)))
        + f64::from(u8::from(trips_lcms2_wtpt_gate(&dst)));
    // ★ The same count under the two OTHER live readings of the same
    // predicate, computed here so the precondition rows can PRICE them rather
    // than assert that the conjunction reading is obviously the right one.
    // Neither is hypothetical: `class only` is ICC.1:2022 9.2.36 as the
    // spec-librarian returned it (v4's clause is class-gated with NO version
    // gate), and the disjunction is the single-character misreading of
    // `cmsio1.c` that this whole section's fixture pair is chosen to defeat.
    let gate_count_class_only = f64::from(u8::from(is_display_class(&src)))
        + f64::from(u8::from(is_display_class(&dst)));
    let gate_count_disjunction = f64::from(u8::from(
        is_display_class(&src) || src.header.version.raw < 0x0400_0000,
    )) + f64::from(u8::from(
        is_display_class(&dst) || dst.header.version.raw < 0x0400_0000,
    ));

    let structure = format!(
        "src v{:08X} {} {}->{} wtpt={} gate={} | dst v{:08X} {} {}->{} wtpt={} gate={}",
        src.header.version.raw,
        src.header.device_class,
        src.header.color_space,
        src.header.pcs,
        wtpt_of(&src),
        trips_lcms2_wtpt_gate(&src),
        dst.header.version.raw,
        dst.header.device_class,
        dst.header.color_space,
        dst.header.pcs,
        wtpt_of(&dst),
        trips_lcms2_wtpt_gate(&dst),
    );

    let grid = rgb_grid();
    let iccce = Iccce::locate().map_err(|e| Unavailable::Error(e.to_string()))?;
    let iccce = match iccce {
        None => {
            return Err(Unavailable::Skip(
                "no iccce release binary — build it with `cargo build --release`".into(),
            ));
        }
        Some(i) => i,
    };

    // iccce takes 0..1; transicc takes RGB in 0..255. Same integer lattice
    // both sides, expressed in each side's own convention, so the only
    // difference that can reach the residual is colour arithmetic.
    let rows: Vec<Vec<f64>> = grid
        .iter()
        .map(|t| t.iter().map(|c| f64::from(*c) / 255.0).collect())
        .collect();

    let mut out = Vec::new();
    for intent in [Intent::AbsoluteColorimetric, Intent::RelativeColorimetric] {
        let mine = iccce.transform_rows_shaped(src_path, dst_path, intent, &rows, 4)?;
        let req = Request {
            input: Space::profile(src_path),
            output: Space::profile(dst_path),
            intent,
            precalc: Precalc::Exact,
            bpc: Bpc::Off,
            values: grid
                .iter()
                .flat_map(|t| t.iter().map(|c| f64::from(*c)))
                .collect(),
        };
        let theirs_100 = oracle.convert_batch_shaped(&req, 3, 4)?;
        let theirs: Vec<Vec<f64>> = theirs_100
            .into_iter()
            .map(|r| r.into_iter().map(|v| v / 100.0).collect())
            .collect();
        if mine.len() != grid.len() || theirs.len() != grid.len() {
            return Err(Unavailable::Error(format!(
                "row-count disagreement at {}: iccce {}, lcms2 {}, grid {}",
                intent.name(),
                mine.len(),
                theirs.len(),
                grid.len()
            )));
        }
        out.push((mine, theirs));
    }
    let (abs_mine, abs_theirs) = out[0].clone();
    let (rel_mine, rel_theirs) = out[1].clone();

    let reduce = |a: &[Vec<f64>], b: &[Vec<f64>]| -> (f64, f64, usize) {
        let mut max = 0.0f64;
        let mut sum = 0.0f64;
        let mut at = 0usize;
        for (i, (u, v)) in a.iter().zip(b).enumerate() {
            let d = u
                .iter()
                .zip(v)
                .map(|(x, y)| (x - y).abs())
                .fold(0.0f64, f64::max);
            sum += d;
            if d > max {
                max = d;
                at = i;
            }
        }
        (max, sum / a.len() as f64, at)
    };

    let (abs_max, abs_mean, worst_index) = reduce(&abs_mine, &abs_theirs);
    let (rel_max, rel_mean, _) = reduce(&rel_mine, &rel_theirs);
    let (counterfactual, counterfactual_mean, _) = reduce(&abs_mine, &rel_mine);

    let unmoved = abs_mine
        .iter()
        .zip(&rel_mine)
        .filter(|(u, v)| {
            u.iter()
                .zip(v.iter())
                .map(|(x, y)| (x - y).abs())
                .fold(0.0f64, f64::max)
                < 1e-9
        })
        .count();

    Ok(AbsAnalysis {
        structure,
        gate_count,
        gate_count_class_only,
        gate_count_disjunction,
        abs_max,
        abs_mean,
        rel_max,
        rel_mean,
        counterfactual,
        counterfactual_mean,
        unmoved_fraction: unmoved as f64 / grid.len() as f64,
        worst_index,
        worst_iccce: abs_mine[worst_index].clone(),
        worst_lcms2: abs_theirs[worst_index].clone(),
    })
}

// ===========================================================================
// Records
// ===========================================================================

fn pass_or_fail(
    id: &str,
    kind: Kind,
    metric: Metric,
    tol: Tolerance,
    source: &str,
    detail: String,
    observed: f64,
) -> Record {
    let outcome = if observed <= tol.value {
        crate::Outcome::Pass {
            observed,
            got: vec![observed],
        }
    } else {
        crate::Outcome::Fail {
            observed,
            got: vec![observed],
        }
    };
    Record {
        id: id.to_string(),
        kind,
        metric,
        tolerance: tol,
        source: source.to_string(),
        detail,
        // The default is `Unstated` and every caller in this module now
        // overrides it with `.with_separation(...)` — four `Measured` and three
        // `NoNamedAlternative`, each with its reason. The default stays
        // `Unstated` rather than becoming a required argument for the reason
        // `Record::with_separation` documents: a required argument would have
        // produced a corpus of invented rivals, which is worse than none.
        separation: crate::Separation::Unstated,
        outcome,
    }
}

/// **REPORTED, NOT GRADED** — tolerance `∞`. Used where the number is a
/// finding rather than a gate. It is a distinct object from a passing row and
/// `TOLERANCES.md` §1 requires it to be readable as such on its own line.
const REPORTED: Tolerance = Tolerance::new(
    f64::INFINITY,
    "REPORTED, NOT GRADED. The quantity is a finding about a policy difference the \
     specification does not adjudicate (A4b resolved SILENT on readers), not a claim that \
     either implementation is within a budget. A tolerance here would be a number chosen to \
     accommodate a disagreement whose size nobody controls",
);

/// §A's records — the confound-free pair.
///
/// ## ★ Candidate separations, added 2026-08-12
///
/// Four of these seven rows carry one and three do not, and the three are
/// [`crate::Separation::NoNamedAlternative`] with their reason rather than
/// `UNSTATED`: somebody looked. The rule applied throughout is the engineer's —
/// *a named alternative a reader cannot go and check is worse than none* — so
/// the alternatives here are all readings of a specific line of source or a
/// specific clause, and every value is computed in the run.
///
/// **Three live readings of one predicate** are priced by the two precondition
/// rows, because there genuinely are three and this project has met all of them:
///
/// | reading | source | count on §A's pair | count on §B's |
/// |---|---|---|---|
/// | `version < 0x4000000 AND class == 'mntr'` | `cmsio1.c` at the pin — what the code does | **0** | **1** |
/// | `class == 'mntr'` | **ICC.1:2022 9.2.36** — what the *standard* says (no version gate) | 1 | 1 |
/// | `version < 0x4000000 OR class == 'mntr'` | the single-character misreading | 2 | 2 |
///
/// Each row names the reading that is the **strongest threat to its own claim**
/// and enumerates the others in the alternative's text, because `Separation`
/// holds one alternative and picking the flattering one would be the tuning this
/// whole mechanism exists to prevent. On §A the threat is the class-only
/// reading: under it the count is 1, the precondition fails, and every number in
/// the section is measuring the policy again. On §B the class-only reading gives
/// the *same* observation, so the threat there is the disjunction.
fn section_a_records(a: &AbsAnalysis) -> Vec<Record> {
    let n = rgb_grid().len();
    let src = "both sides computed in this run; no expectation is transcribed from anywhere. \
               Precondition read from the two parsed headers";
    vec![
        pass_or_fail(
            "pass4c/v4matrix-to-swop/precondition-neither-profile-trips-lcms2-wtpt-gate",
            Kind::SelfConsistency,
            Metric::AbsMaxComponent,
            GATE_NOT_TRIPPED,
            "cmsio1.c _cmsReadMediaWhitePoint at pin 21c582a, transcribed; evaluated against the \
             parsed headers of the two files opened in this run",
            format!(
                "count over 2 profiles of (version < 0x4000000 AND class == 'mntr'): {} | the \
                 same count under the other two live readings: class-only (ICC.1:2022 9.2.36) = \
                 {:.0}, disjunction = {:.0}",
                a.structure, a.gate_count_class_only, a.gate_count_disjunction
            ),
            a.gate_count,
        )
        .with_separation(crate::Separation::against(
            "the predicate read as ICC.1:2022 9.2.36 says it - CLASS ONLY, no version gate. That \
             is not a hypothetical misreading: it is what the STANDARD says, and the \
             spec-librarian's dispatch found that lcms2's predicate reproduces no clause in \
             either edition (v4's 9.2.36 is class-gated with no version gate; v2's A.3.1.1 is \
             gated on the adaptation condition and not on class at all). A harness that had \
             transcribed the specification instead of the code would count the v4 'mntr' SOURCE \
             as tripping, this precondition would fail, and section A's whole claim of a \
             structurally absent confound would be false. The third reading, the '&&' misread as \
             '||', would count 2 - further still, and it is what the fixture pair is chosen to \
             defeat, each profile failing a DIFFERENT half",
            a.gate_count_class_only,
            a.gate_count,
            crate::SepUnits::SameAsMetric,
        )),
        pass_or_fail(
            "pass4c/v4matrix-to-swop/absolute/device-vs-lcms2",
            Kind::CrossCheck,
            Metric::DeviceAbsMaxNormalised,
            DEVICE_B2A,
            src,
            format!(
                "★ THE MEASUREMENT. v4 matrix/TRC -> SWOP B2A1 at ICC-absolute, {n} RGB points \
                 (9x9x9 on the 8-bit lattice), -c0, no BPC either side. lcms2's wtpt \
                 substitution is STRUCTURALLY ABSENT (see the precondition row), and its forced \
                 trilinear for a Lab-PCS output LUT IS iccce's n-linear, so NA-006 is zero here \
                 too. The source has no CLUT"
            ),
            a.abs_max,
        )
        // ★ THE ROW THE ENGINEER NAMED. Its whole claim is that a known
        // divergence CANNOT fire on this pair, and the value it would have
        // observed if it had is not a guess: it is the counterfactual row's
        // number, which is EXACT here rather than modelled, because the
        // source's stored wtpt IS D50 and substituting D50 for the
        // destination's collapses the 6.3.2.2 diagonal to identity.
        .with_separation(crate::Separation::against(
            "lcms2's wtpt substitution HAVING FIRED on the destination - i.e. the NC-053 policy \
             difference reaching this pair. The value is the counterfactual row's, which on this \
             pair is EXACT and not modelled: the source's stored wtpt IS D50, so substituting D50 \
             for the DESTINATION's wtpt collapses the whole 6.3.2.2 scaling diagonal to identity, \
             and lcms2's absolute output would then BE its media-relative output. A reader can \
             check it against row absolute/counterfactual-wtpt-substituted in the same run",
            a.counterfactual,
            a.abs_max,
            crate::SepUnits::SameAsMetric,
        )),
        pass_or_fail(
            "pass4c/v4matrix-to-swop/absolute/device-mean",
            Kind::CrossCheck,
            Metric::DeviceAbsMeanNormalised,
            REPORTED,
            src,
            format!("mean over {n} points; reported beside the max, never instead of it"),
            a.abs_mean,
        )
        // The same alternative as the max row, but priced as a MEAN over the
        // same grid. Borrowing the max's counterfactual here would be Pass 6
        // row R4's population error - two reductions of different populations
        // quoted for one another.
        .with_separation(crate::Separation::against(
            "the same alternative as the max row - lcms2's wtpt substitution having fired - \
             reduced as a MEAN over the same 729 points rather than a max, because a max's \
             counterfactual quoted on a mean row is two different reductions read for one another",
            a.counterfactual_mean,
            a.abs_mean,
            crate::SepUnits::SameAsMetric,
        )),
        pass_or_fail(
            "pass4c/v4matrix-to-swop/media-relative/device-vs-lcms2",
            Kind::CrossCheck,
            Metric::DeviceAbsMaxNormalised,
            DEVICE_B2A,
            src,
            format!(
                "THE FLOOR. The same pair, the same grid, the same destination table, at an \
                 intent with no absolute scaling in it at all. This is what the absolute row \
                 has to be compared against: the 8-bit lut8 quantisation cost of this \
                 direction. Observed absolute {:.6e} vs media-relative {:.6e} — the ICC-absolute \
                 arithmetic adds nothing detectable above the floor already established for \
                 this direction in Pass 4b",
                a.abs_max, a.rel_max
            ),
            a.rel_max,
        )
        .with_separation(crate::Separation::none(
            "considered, and there is none. The rival readings this module holds are all about \
             the MEDIA WHITE POINT, and lcms2 consults it only for the ICC-absolute adjustment - \
             at media-relative both implementations normalise by white on both sides and the \
             predicate is never evaluated. The other candidate divergence for this direction, \
             NA-006's interpolation scheme, is structurally zero too: Pass 4b measured that lcms2 \
             FORCES trilinear for a Lab-PCS output LUT, which is iccce's n-linear on a 3-input \
             table. What is left is 8-bit quantisation, and quantisation has one value, not two",
        )),
        pass_or_fail(
            "pass4c/v4matrix-to-swop/absolute/counterfactual-wtpt-substituted",
            Kind::SelfConsistency,
            Metric::DeviceAbsMaxNormalised,
            REPORTED,
            "computed from iccce alone, both arms; no oracle output in it",
            format!(
                "★ EXACT, not modelled. Because the source's stored wtpt IS D50, substituting \
                 D50 for the DESTINATION's wtpt collapses the whole 6.3.2.2 scaling diagonal to \
                 identity, so absolute-vs-media-relative on this pair IS what lcms2's \
                 substitution would have cost here. It is the NC-053 mechanism, priced on this \
                 pair: {:.6e} device units against an observed residual of {:.6e}",
                a.counterfactual, a.abs_max
            ),
            a.counterfactual,
        )
        // A real alternative with a computable value, and it is EXACTLY ZERO —
        // which is the point. Substituting on the source instead of the
        // destination costs nothing here because the source's wtpt already IS
        // D50, and that asymmetry is the reason the counterfactual is exact
        // rather than approximate.
        .with_separation(crate::Separation::against(
            "the substitution firing on the SOURCE instead of the destination. lcms2's predicate \
             is applied per profile, so which of the pair would have tripped is a real \
             alternative and not a hypothetical - and here it costs EXACTLY ZERO, because the \
             source's stored wtpt already IS D50 and substituting D50 for D50 is the identity. \
             That asymmetry is precisely why this counterfactual is exact rather than modelled, \
             and stating it as a separation puts the reason on the row instead of in a paragraph",
            0.0,
            a.counterfactual,
            crate::SepUnits::SameAsMetric,
        )),
        pass_or_fail(
            "pass4c/v4matrix-to-swop/absolute/sensitivity-floor",
            Kind::SelfConsistency,
            Metric::AbsMaxComponent,
            SENSITIVITY_FLOOR,
            "Pass 4b's accepted counterfactual band (99x, 139x, 191x) on this same table and \
             direction, transcribed as a floor",
            format!(
                "violation max(0, 100 - r); observed r = {:.0}x. DL-025: ask what the control \
                 would return if the effect were identically absent, and check its fixture is \
                 not that case",
                a.ratio()
            ),
            (100.0 - a.ratio()).max(0.0),
        )
        .with_separation(crate::Separation::none(
            "considered, and there is none. Both terms of the ratio are measured in this run from \
             iccce alone, and neither has a rival READING: the counterfactual is exact for the \
             reason the row above states, and the observed residual is a subtraction. The only \
             alternative one could name is a different FLOOR than 100x - and that is a tolerance \
             question, answered in this row's `why` from Pass 4b's accepted 99x/139x/191x band, \
             not an alternative value this row could have observed. Separations and tolerances are \
             different objects and conflating them is how a separation becomes a second, \
             undocumented gate",
        )),
        pass_or_fail(
            "pass4c/v4matrix-to-swop/absolute/degeneracy-guard-unmoved-fraction",
            Kind::SelfConsistency,
            Metric::AbsMaxComponent,
            DEGENERACY_GUARD,
            "computed from iccce alone, both arms",
            format!(
                "fraction of the {n} points where |absolute - media-relative| < 1e-9, i.e. \
                 points carrying no evidence. Observed {:.6e} = {} point(s): a diagonal scaling \
                 fixes the origin, so device black is expected and is arithmetic, not a defect. \
                 This is the guard against the OTHER null: if the absolute scaling had pushed \
                 the grid out of gamut, both implementations would clamp to the same boundary \
                 and agree perfectly while computing nothing",
                a.unmoved_fraction,
                (a.unmoved_fraction * n as f64).round() as u32
            ),
            a.unmoved_fraction,
        )
        .with_separation(crate::Separation::none(
            "considered, and there is none. This row counts points, from iccce's own two arms, \
             against a threshold of 1e-9. A rival would have to be a rival READING of 'the \
             absolute scaling did not move this point', and there is not one - the 1e-9 is a \
             numerical-zero threshold, not an interpretation. The null the row guards against \
             (gamut clipping pinning the grid and manufacturing agreement) is the HYPOTHESIS the \
             row tests, not an alternative value it could have observed, which is the same \
             distinction pass5c's ATTRIBUTION row makes",
        )),
    ]
}

/// §B's records — the policy-exposed pair, the direction-symmetry measurement.
fn section_b_records(a: &AbsAnalysis) -> Vec<Record> {
    let n = rgb_grid().len();
    let src = "both sides computed in this run";
    vec![
        pass_or_fail(
            "pass4c/srgb-to-swop/precondition-source-DOES-trip-lcms2-wtpt-gate",
            Kind::SelfConsistency,
            Metric::AbsMaxComponent,
            Tolerance::new(
                0.0,
                "violation |count - 1|: section B's subject is the substitution FIRING, so \
                 exactly one of the two profiles must satisfy the predicate — the v2 'mntr' \
                 source. Zero would mean the section is measuring nothing; two would mean the \
                 destination is also substituting and the attribution below is wrong. Graded \
                 at 0 exact because it is a count of files",
            ),
            "cmsio1.c _cmsReadMediaWhitePoint at pin 21c582a; evaluated against parsed headers",
            format!(
                "|gate count - 1| over the pair: {} | the same count under the other two live \
                 readings: class-only (ICC.1:2022 9.2.36) = {:.0}, disjunction = {:.0}",
                a.structure, a.gate_count_class_only, a.gate_count_disjunction
            ),
            (a.gate_count - 1.0).abs(),
        )
        // ★ A DIFFERENT rival from §A's, and the difference is the honest part.
        // The class-only reading gives the SAME count here (1), so it is not a
        // threat to this row and naming it would have manufactured a
        // ZERO-SEPARATION. The reading this row is sensitive to is the
        // disjunction, which would count 2 and would mean the destination is
        // substituting too — under which §B's attribution of the whole effect
        // to the SOURCE's white point is wrong.
        .with_separation(crate::Separation::against(
            "the '&&' in cmsio1.c read as '||'. Under that reading BOTH profiles trip (the v2 \
             'mntr' source on both halves, the v2.1 'prtr' destination on the version half), the \
             count is 2, and section B's attribution of the whole effect to the SOURCE's white \
             point is wrong because the destination would be substituting as well. NOTE THAT THIS \
             IS A DIFFERENT RIVAL FROM SECTION A's, deliberately: the class-only reading of \
             ICC.1:2022 9.2.36 gives the SAME count as the conjunction on this pair (1), so it is \
             not a threat to this row and naming it would have manufactured a ZERO-SEPARATION out \
             of a rival that is real but harmless here",
            (a.gate_count_disjunction - 1.0).abs(),
            (a.gate_count - 1.0).abs(),
            crate::SepUnits::SameAsMetric,
        )),
        pass_or_fail(
            "pass4c/srgb-to-swop/absolute/device-vs-lcms2",
            Kind::CrossCheck,
            Metric::DeviceAbsMaxNormalised,
            REPORTED,
            src,
            format!(
                "★ NC-053's MECHANISM, MEASURED IN THE OTHER DIRECTION (DL-021). NC-053 had the \
                 v2 'mntr' profile as DESTINATION and measured 11.217 dE2000; here the same \
                 file is the SOURCE and the gate fires on WP_src instead. Predicted before the \
                 run to stay LARGE — iccce scales by D65/D50 = (0.98579, 1.0, 1.32027) where \
                 lcms2 scales by identity. Observed {:.6e} device units over {n} points against \
                 a media-relative floor of {:.6e} on the same pair and grid: a factor of {:.0}. \
                 The policy is direction-symmetric",
                a.abs_max,
                a.rel_max,
                a.abs_max / a.rel_max
            ),
            a.abs_max,
        )
        // §A's separation mirrored: there the alternative was "it fired", here
        // it is "it did NOT". If lcms2 did not substitute, both sides would
        // apply the same 6.3.2.2 scaling and this row would fall to the
        // media-relative floor measured on the same pair and grid in this run.
        .with_separation(crate::Separation::against(
            "lcms2 NOT substituting - i.e. the gate not firing on the v2 'mntr' source. Both \
             implementations would then apply the same 6.3.2.2 scaling and this row would fall to \
             the media-relative residual measured on the SAME pair and grid in this run, which is \
             the floor row below. It is section A's separation mirrored: there the named \
             alternative is 'the substitution fired' and here it is 'it did not', and the two \
             rows together are what make the policy's direction-symmetry a measurement rather \
             than a pair of anecdotes",
            a.rel_max,
            a.abs_max,
            crate::SepUnits::SameAsMetric,
        )),
        pass_or_fail(
            "pass4c/srgb-to-swop/media-relative/device-vs-lcms2",
            Kind::CrossCheck,
            Metric::DeviceAbsMaxNormalised,
            DEVICE_B2A,
            src,
            format!(
                "the floor that makes the row above mean something. Without a residual measured \
                 on the SAME two files at an intent with no absolute scaling in it, '{:.3e} \
                 device units' is a number with nothing to be large compared to",
                a.abs_max
            ),
            a.rel_max,
        )
        .with_separation(crate::Separation::none(
            "considered, and there is none, for the same reason as section A's floor row: lcms2 \
             consults the media white point only for the ICC-absolute adjustment, so at \
             media-relative the predicate whose three readings this module prices is never \
             evaluated at all. A floor row measures quantisation, and quantisation has one value",
        )),
    ]
}

/// Rows emitted when a section could not run, so a skip is never mistaken for
/// a pass in a summary line.
fn unavailable_records(u: &Unavailable, ids: &[&str]) -> Vec<Record> {
    ids.iter()
        .map(|id| {
            Record::skipped(
                (*id).to_string(),
                Kind::CrossCheck,
                Metric::DeviceAbsMaxNormalised,
                DEVICE_B2A,
                "not run".to_string(),
                u.to_string(),
            )
        })
        .collect()
}

/// Both sections' analyses, each independent of the other.
#[derive(Debug, Clone, Default)]
pub struct Bundle {
    /// §A — the confound-free pair. The measurement.
    pub clean: Option<AbsAnalysis>,
    /// §B — the policy-exposed pair. The direction-symmetry finding.
    pub exposed: Option<AbsAnalysis>,
}

/// Run both sections.
///
/// §A needs the committed synthetic fixture plus SWOP; §B needs two system
/// profiles. §A is the one that carries the graded claim, so it is run first
/// and is not made to depend on §B in any way.
#[must_use]
pub fn run(oracle: &Oracle) -> (Bundle, Vec<Record>) {
    let mut records = Vec::new();

    let clean = match analyse(oracle, &v4_matrix_src(), Path::new(SWOP)) {
        Ok(a) => {
            records.extend(section_a_records(&a));
            Some(a)
        }
        Err(u) => {
            records.extend(unavailable_records(
                &u,
                &[
                    "pass4c/v4matrix-to-swop/precondition-neither-profile-trips-lcms2-wtpt-gate",
                    "pass4c/v4matrix-to-swop/absolute/device-vs-lcms2",
                    "pass4c/v4matrix-to-swop/absolute/device-mean",
                    "pass4c/v4matrix-to-swop/media-relative/device-vs-lcms2",
                    "pass4c/v4matrix-to-swop/absolute/counterfactual-wtpt-substituted",
                    "pass4c/v4matrix-to-swop/absolute/sensitivity-floor",
                    "pass4c/v4matrix-to-swop/absolute/degeneracy-guard-unmoved-fraction",
                ],
            ));
            None
        }
    };

    let exposed = match analyse(oracle, Path::new(SRGB), Path::new(SWOP)) {
        Ok(a) => {
            records.extend(section_b_records(&a));
            Some(a)
        }
        Err(u) => {
            records.extend(unavailable_records(
                &u,
                &[
                    "pass4c/srgb-to-swop/precondition-source-DOES-trip-lcms2-wtpt-gate",
                    "pass4c/srgb-to-swop/absolute/device-vs-lcms2",
                    "pass4c/srgb-to-swop/media-relative/device-vs-lcms2",
                ],
            ));
            None
        }
    };

    (Bundle { clean, exposed }, records)
}

// ===========================================================================
// Tests — of the apparatus, not of any colour
// ===========================================================================
//
// These assert on OUTCOMES computed by the code that ships, not on the shape
// of that code. A test that asserts a grid "looks right" by re-deriving it the
// same way certifies the bug it was written to catch.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_is_deterministic_and_the_stated_size() {
        assert_eq!(rgb_grid(), rgb_grid());
        assert_eq!(rgb_grid().len(), 729, "9^3; every record's detail says 729");
    }

    #[test]
    fn grid_is_on_the_8bit_lattice_and_covers_both_ends() {
        let g = rgb_grid();
        for p in &g {
            for c in p {
                assert!(*c <= 255, "outside the 8-bit lattice: {p:?}");
            }
        }
        assert!(
            g.contains(&[0, 0, 0]),
            "black must be present: it is the fixed point the degeneracy guard budgets for"
        );
        assert!(
            g.contains(&[255, 255, 255]),
            "white must be present: it is where the absolute scaling clips"
        );
    }

    /// The gate predicate is the whole precondition, so it is tested against
    /// the **committed fixture's actual bytes** rather than against a
    /// hand-built header. If the fixture is ever regenerated as v2, §A stops
    /// being confound-free and this fails at test time rather than silently
    /// producing a number that measures the policy.
    #[test]
    fn the_committed_v4_source_does_not_trip_the_gate() {
        let bytes = std::fs::read(v4_matrix_src()).expect("committed fixture must be readable");
        let p = Profile::parse(&bytes).expect("committed fixture must parse");
        assert!(
            p.header.version.raw >= 0x0400_0000,
            "fixture is no longer v4; section A's precondition depends on it"
        );
        assert!(
            !trips_lcms2_wtpt_gate(&p),
            "the confound-free source now trips lcms2's wtpt gate — section A is invalid"
        );
    }

    /// The predicate must be able to return **true**, or the precondition row
    /// is graded against a function that cannot fail. Verified on the real
    /// v2 `'mntr'` profile when present; skipped, loudly, when it is not.
    #[test]
    fn the_gate_predicate_can_return_true() {
        let path = Path::new(SRGB);
        if !path.exists() {
            eprintln!(
                "SKIP: system sRGB absent; the gate predicate's true-branch is unverified on this machine"
            );
            return;
        }
        let bytes = std::fs::read(path).expect("readable");
        let p = Profile::parse(&bytes).expect("parses");
        assert!(
            trips_lcms2_wtpt_gate(&p),
            "system sRGB is v2 'mntr' and MUST trip the gate; if it does not, the predicate is \
             wrong and section A's precondition proves nothing"
        );
    }

    #[test]
    fn ratio_does_not_divide_by_zero() {
        let a = AbsAnalysis {
            structure: String::new(),
            gate_count: 0.0,
            gate_count_class_only: 0.0,
            gate_count_disjunction: 0.0,
            abs_max: 0.0,
            abs_mean: 0.0,
            rel_max: 0.0,
            rel_mean: 0.0,
            counterfactual: 1.0,
            counterfactual_mean: 0.0,
            unmoved_fraction: 0.0,
            worst_index: 0,
            worst_iccce: vec![],
            worst_lcms2: vec![],
        };
        assert!(a.ratio().is_infinite());
    }

    /// ★ The three readings of lcms2's `wtpt` predicate must give **different**
    /// counts on §A's pair, or the separations §3.4.5.1 states are theatre.
    ///
    /// This is the same discipline as `the_gate_predicate_can_return_true`
    /// applied to a candidate separation rather than to a precondition: a rival
    /// reading that yields the same observation as the implemented one prices
    /// nothing, and the only way to know is to compute both. Verified on the
    /// real fixture pair when the vendor profile is present; skipped, loudly,
    /// when it is not.
    #[test]
    fn the_three_predicate_readings_are_not_the_same_reading() {
        let src = v4_matrix_src();
        let dst = Path::new(SWOP);
        if !src.exists() || !dst.exists() {
            eprintln!(
                "SKIP: section A's profile pair is not both present; the separations on the \
                 precondition rows are unverified on this machine"
            );
            return;
        }
        let sb = std::fs::read(&src).expect("readable");
        let db = std::fs::read(dst).expect("readable");
        let s = Profile::parse(&sb).expect("parses");
        let d = Profile::parse(&db).expect("parses");

        let conjunction = u8::from(trips_lcms2_wtpt_gate(&s)) + u8::from(trips_lcms2_wtpt_gate(&d));
        let class_only = u8::from(is_display_class(&s)) + u8::from(is_display_class(&d));
        let disjunction = u8::from(is_display_class(&s) || s.header.version.raw < 0x0400_0000)
            + u8::from(is_display_class(&d) || d.header.version.raw < 0x0400_0000);

        assert_eq!(
            (conjunction, class_only, disjunction),
            (0, 1, 2),
            "section A's pair must separate all three readings of lcms2's predicate: the \
             conjunction it implements (0 — the confound is structurally absent), ICC.1:2022 \
             9.2.36's class-only reading (1 — the v4 'mntr' SOURCE trips, and section A would be \
             measuring the policy again), and the '&&' misread as '||' (2). If any two coincide, \
             the candidate separation on \
             pass4c/v4matrix-to-swop/precondition-neither-profile-trips-lcms2-wtpt-gate is \
             pricing nothing"
        );
    }
}
