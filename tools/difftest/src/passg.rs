//! # Pass G — the Ghent v5.0 population sample, graded
//!
//! Every profile iccce had been differentially tested against before this pass
//! was **synthetic** (`tools/gen-profiles`), **OS-shipped** (the Windows colour
//! directory) or **standards-body-issued** (FOGRA51). None of them is what a
//! real document producer embeds. This pass grades iccce against the pinned
//! lcms2 oracle over **profiles extracted from the Ghent PDF Output Suite
//! 5.0** — a graphic-arts PDF/X test corpus written by Adobe InDesign CS6 and
//! imposed by Callas pdfToolbox, in which 20 distinct ICC profiles appear
//! across 121 embeddings in 98 production PDFs.
//!
//! **This is a compatibility exercise, not a conformance certification.**
//! Nothing here was measured with a spectrophotometer or proofed on a press,
//! and no row claims otherwise. The strongest claim any oracle row in this
//! module makes is *"iccce and lcms2 read this profile the same way"*, which
//! `docs/TOLERANCES.md` §1 ranks below the two rows here that are
//! [`Kind::DerivedExpectation`] and far below published ground truth, of which
//! this pass has none.
//!
//! ## ★★★ Licensing — read before touching anything in this module
//!
//! The Ghent suite's own licence **forbids commercial use and redistribution
//! without written permission**, and the profiles inside carry Adobe's, ECI's
//! and X-Rite's separate licences. Therefore, and without exception:
//!
//! - **No file from `ghent-v50\` may be committed to this repository.**
//! - **No value read out of one may be copied into this repository** — not a
//!   colorant, not a white point, not a CLUT sample, not a ΔE. Every number
//!   this module reports is *computed at run time from the file on the
//!   operator's disk* and printed; none is stored in source. That is not
//!   merely hygiene: it is the same rule `docs/NEXT_SESSION.md` §4 states for
//!   the other three private corpora, and it happens to coincide with DL-034
//!   ("a claim-bearing number must be computed, not typed").
//! - The only identifiers this file holds are **SHA-256 prefixes and file
//!   names**, which are pointers to a licensed artifact rather than content of
//!   it, and structural facts (a grid size, a tag signature, a device class)
//!   which are format metadata rather than colour data.
//!
//! The corpus is resolved from `$ICCCE_PRIVATE_FIXTURES`, then from the
//! default path, and **every row SKIPs with a reason when it is absent** —
//! exactly as the existing private-fixture tests do. CI is permanently in the
//! skipping case. ★ A green CI run is therefore **not** evidence that anything
//! in this module passed; it is evidence that nothing in it ran.
//!
//! ## The four sections, and why they are independent
//!
//! | § | subject | needs |
//! |---|---|---|
//! | **A** | the **v4 vendor `mAB `/`mBA ` path** — X-Rite's `GWG_ICC_v4_testprofile.icc`, a real 4-channel v4.2 output profile with a 7×7×7×7 `mAB ` and a 17³ `mBA ` | corpus + oracle |
//! | **B** | the **population sweep** — five profile pairs, four intents, ±BPC, in the `B2A` direction where lcms2 forces trilinear | corpus + oracle + shipped binary |
//! | **C** | **`eciRGB v2` in its v2.4 and v4.2 encodings**, same vendor, same declared colour space | corpus + shipped binary |
//! | **D** | the **two GWG trap profiles**, whose correct answer is known *without* measurement because the swap is the profile's declared content | corpus (+ oracle for the cross-check arm only) |
//!
//! §A was the reason for the dispatch. **Every v4 LUT profile iccce had been
//! graded against until now came from `tools/gen-profiles` — that is, from
//! us**, so a shared misreading between the fixture generator and the engine
//! would have been invisible. X-Rite authored this one.
//!
//! ## What Pass G found, in one paragraph each
//!
//! 1. **The v4 `mAB ` path agrees with lcms2 once lcms2's own CLUT geometry is
//!    substituted**, and the residual collapses by two orders of magnitude when
//!    it is — the same signature Pass 4 established on a `lut16` profile, now
//!    reproduced on a vendor-authored v4 one. The raw disagreement is
//!    **exactly** the interpolation-method envelope computed from the CLUT's
//!    own bytes; see [`XriteArm`].
//! 2. **lcms2's forced BPC fires on this profile at the perceptual intent**,
//!    driven by the *destination* profile's version (Pass 4b finding 2), and
//!    it is worth several ΔE2000. §A defeats it **with a fixture choice** — a
//!    v2 PCS destination — rather than subtracting it with a model, which is
//!    the lesson Pass 4c recorded.
//! 3. **Adobe's shipped `sRGB IEC61966-2.1` and `Adobe RGB (1998)` profiles,
//!    as embedded in these production PDFs, encode `wtpt` = D65 while their
//!    colorants sum to D50 and they carry no `chad`.** Under
//!    ICC.1:2001-04 A.3.1.1 that is a **non-compliant author**, not an
//!    alternative reading (see §B's `wtpt` rows). It is the mechanism of the
//!    only large iccce/lcms2 disagreement in the sweep, and the finding the
//!    population sample makes possible is that it is not a one-off system
//!    profile: it is what a real producer embeds, 121 times.
//! 4. **`eciRGB v2`'s two encodings are NOT the clean version isolator they
//!    look like** — see [`EciArm`]. Both encode `wtpt` = D50, so neither can
//!    trip lcms2's substitution; and their TRCs are represented differently
//!    (700-entry table vs `para` type 3), so a disagreement between them has
//!    **two** candidate causes and the pair cannot separate them.
//!
//! ## The evidence class of each section, stated because it differs
//!
//! - §A's oracle rows and §B's are **[`Kind::CrossCheck`]** — two
//!   implementations agreeing.
//! - §A's forced-BPC row and §D's swap rows are **[`Kind::DerivedExpectation`]**:
//!   the expectation is arithmetic on the profile's *own* tag bytes plus clause
//!   text, with no implementation's output in it.
//! - §C is **[`Kind::SelfConsistency`]** and is the *weakest* class in this
//!   module: two files from one vendor agreeing says nothing about whether
//!   either is right, and it is weaker even than a cross-check because there is
//!   only one lineage on both sides. The brief that commissioned it said so;
//!   this module says so on every row.

use std::path::{Path, PathBuf};

use iccce_cmm::MatrixTrc;
use iccce_color::{Lab, Xyz, delta_e_2000};
use iccce_profile::Profile;
use iccce_profile::lut::{ClutSamples, CurveElement, LutAB};
use iccce_profile::num::Signature;
use iccce_profile::tag_types::{Curve, TagData};

use crate::pass4b::{HarnessClut, Scheme};
use crate::{
    Bpc, Iccce, Intent, Kind, Metric, Oracle, Precalc, Record, Request, SepUnits, Separation,
    Space, Tolerance,
};

// ===========================================================================
// Where the corpus lives
// ===========================================================================

/// The private-fixture root, resolved the way every other private-fixture test
/// in this repository resolves it: **environment variable, then default path,
/// then skip.**
///
/// There is deliberately no third fallback and no bundled copy. A corpus that
/// cannot be redistributed must be *absent* on a machine that has not been
/// given it, and the suite must say so out loud rather than quietly grading
/// nothing.
#[must_use]
pub fn corpus_dir() -> PathBuf {
    std::env::var_os("ICCCE_PRIVATE_FIXTURES")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(r"D:\Dev\iccce-private-fixtures"))
        .join("ghent-v50")
}

/// The corpus members this pass uses, named by the first 16 hex digits of their
/// SHA-256 — which is how the extractor's `manifest.json` names them, and which
/// is a *pointer* to a licensed artifact rather than any part of its content.
///
/// The `desc` strings in the comments are the profiles' own descriptions and are
/// reproduced here only as identification; they are not colour values.
mod file {
    /// `sRGB IEC61966-2.1`, v2.1 `mntr`, 1024-entry `curv` TRCs. ★ `wtpt` = D65,
    /// colorants sum to D50, no `chad` — see §B's authoring row.
    pub const SRGB: &str = "2b3aa1645779a9e6.icc";
    /// `Adobe RGB (1998)`, v2.1 `mntr`, single-value (analytic) `curv` TRCs.
    /// Same `wtpt` shape as [`SRGB`].
    pub const ADOBE: &str = "07c1e0738ba6068b.icc";
    /// `eciRGB v2`, v2.4 `mntr`, **700-entry tabulated** `curv` TRCs, `wtpt`=D50.
    pub const ECIRGB_V2: &str = "4b55b697e41a8f29.icc";
    /// `eciRGB v2 ICCv4`, v4.2 `mntr`, **`para` type 3** TRCs, `wtpt`=D50.
    pub const ECIRGB_V4: &str = "58c16e490b2751dc.icc";
    /// `ISO Coated v2 (ECI)`, v2.4 `prtr` CMYK, `mft2` A2B (16³×… grid 16) and
    /// `mft2` B2A (33³).
    pub const ISOCOATED: &str = "128dc02f7246cc38.icc";
    /// `Coated FOGRA39 (ISO 12647-2:2004)`, v2.1 `prtr` CMYK, `mft2` A2B
    /// (grid 11), **`mft1`** B2A (33³, 8-bit).
    pub const FOGRA39: &str = "da2b9b593e27cba2.icc";
    /// ★ `GWG_ICC_v4_testprofile.icc` — **X-Rite, ICC v4.2.0**, `prtr` CMYK,
    /// `mAB ` A2B0/1/2 at 7×7×7×7 and `mBA ` B2A0/1/2 at 17³, plus `gamt` and
    /// four `gbd` tags. The subject of §A.
    pub const XRITE_V4: &str = "b5988983b6b3b7d4.icc";
    /// `Schwarze Druckfarbe - ISO Coated v2 (ECI)`, v2.1 `prtr` **GRAY**,
    /// four tags, 256-entry `kTRC`.
    pub const GRAY: &str = "5dae7984654a2c9f.icc";
    /// ★ `RGB mntr mtx X (Switch red green)` — a GWG discriminator. Matrix/TRC,
    /// v2.2, γ from a single-value `curv`; its `rXYZ` holds a **green** primary.
    pub const TRAP_RGB: &str = "13b44969a980dcd1.icc";
    /// A second file with the same `desc` as [`TRAP_RGB`] and a different
    /// SHA-256. §D grades the two against each other.
    pub const TRAP_RGB_2: &str = "5f0b32d7fe5b2ffb.icc";
    /// ★ `CMYK prtr lut X (Switch magenta cyan)` — the LUT discriminator.
    pub const TRAP_CMYK: &str = "bbdfa02565c1c1e9.icc";
}

mod tag {
    use iccce_profile::num::Signature;
    pub const A2B0: Signature = Signature(0x4132_4230);
    pub const A2B1: Signature = Signature(0x4132_4231);
    pub const A2B2: Signature = Signature(0x4132_4232);
    pub const B2A0: Signature = Signature(0x4232_4130);
    pub const B2A1: Signature = Signature(0x4232_4131);
    pub const B2A2: Signature = Signature(0x4232_4132);
    pub const R_XYZ: Signature = Signature(0x7258_595A);
    pub const G_XYZ: Signature = Signature(0x6758_595A);
    pub const B_XYZ: Signature = Signature(0x6258_595A);
    pub const WTPT: Signature = Signature(0x7774_7074);
}

// ===========================================================================
// Tolerances — every one of them derived from a measured structural quantity
// ===========================================================================

/// **§A, the wide structural gate on the v4 `mAB ` PCS comparison.**
///
/// Metric: max ΔE2000 between the harness's `mAB ` pipeline under **iccce's**
/// n-linear geometry and `transicc -o*Lab{2,4} -c0`.
///
/// ## This number is not a constant — it is computed per tag, at run time
///
/// It cannot be a constant, for the reason `DEVICE_B2A`'s history records: the
/// envelope is a property of *which table is loaded*, and a table this pass
/// never sees (a different Ghent profile, a re-issued X-Rite one) would
/// silently inherit a bound derived from another. [`XriteArm::method_envelope`]
/// evaluates the profile's own CLUT under **both** published geometries —
/// iccce's n-linear and lcms2's `Eval4Inputs` hybrid (linear along the leading
/// channel, Sakamoto tetrahedral over the trailing three) — over the same grid
/// the comparison uses, and takes the maximum ΔE2000. **No lcms2 output enters
/// that computation**; it is arithmetic on the file's bytes and on two
/// published algorithms.
///
/// The tolerance is that envelope **× 1.25**, and the 25 % is for the two
/// things the envelope deliberately does not model: maxima between grid points,
/// and lcms2 evaluating the CLUT in fixed point where the envelope uses `f64`.
///
/// ## What this row can and cannot do
///
/// It is sized to admit the whole interpolation-method difference, which
/// ICC.1 does not legislate (ambiguity A16: the specification is **silent** on
/// interpolation). It therefore **cannot claim agreement** — it detects
/// structural error only: a wrong CLUT index order, a wrong PCSLAB decode, a
/// dropped A-curve, a transposed ink. Those are tens of ΔE.
/// The agreement claim is [`emulated_tolerance`]'s.
#[must_use]
pub fn structural_tolerance_from(envelope: f64) -> f64 {
    envelope * 1.25
}

/// **§A, the row that claims agreement.** With lcms2's own CLUT geometry
/// substituted into the harness pipeline, what remains is the oracle's
/// arithmetic and nothing else.
///
/// The terms, all of them lcms2's, at pin `21c582a`:
///
/// | term | size | source |
/// |---|---|---|
/// | 4096-entry A curves quantised to 1/65535 in *and* out | ≤1.53×10⁻⁵ per channel | `cmsgamma.c` `cmsEvalToneCurveFloat`, `nSegments == 0` |
/// | CLUT stage input rounded to `u16` | ≤7.63×10⁻⁶ | `cmslut.c` `EvaluateCLUTfloatIn16` |
/// | CLUT evaluated in s15.16 fixed point, not `f64` | ~1 lsb of 1/65535 out | `cmsintrp.c` `EVAL_FNS(4,3)` |
/// | 2-entry B curves, then the **v4** PCSLAB decode | ≤7.63×10⁻⁶ | clause 10.13 / Table 42 |
/// | `transicc` prints `L*a*b*` to 4 decimals | ±5×10⁻⁵ each | README §9 |
///
/// One 16-bit lsb of CLUT output becomes, through the **v4** PCSLAB decode
/// (`L* = 100·n`, `a*/b* = 255·n − 128`), `100/65535 ≈ 1.53×10⁻³` in `L*` and
/// `255/65535 ≈ 3.89×10⁻³` in `a*`/`b*`. Summing a few such terms with the
/// print floor gives a ΔE00 budget of order 10⁻². **2×10⁻² is that** — the
/// same figure Pass 4 derived for `lut16`, and it lands there for the same
/// reason: the decode scale factors are within 2 % of the legacy ones
/// (652.8 vs 655.35 codes per `L*` unit, 256 vs 257.0 per `a*` unit).
///
/// It is **at least 40× tighter than [`structural_tolerance_from`]** on every
/// tag §A grades, and that ratio is what makes the attribution a claim rather
/// than a restatement of the observation.
pub const EMULATED: Tolerance = Tolerance::new(
    2e-2,
    "with lcms2's OWN CLUT geometry (Eval4Inputs: linear on the leading channel, Sakamoto \
     tetrahedral on the trailing three) substituted into the harness mAB pipeline, what remains \
     is the oracle's arithmetic: 4096-entry A curves rounded to 1/65535 in and out \
     (cmsEvalToneCurveFloat), CLUT input rounded to u16 (EvaluateCLUTfloatIn16), the CLUT \
     evaluated in s15.16 fixed point, and transicc's 4-decimal Lab print. One 16-bit lsb of CLUT \
     output is 1.53e-3 in L* and 3.89e-3 in a*/b* under the v4 PCSLAB decode (clause 10.13), so \
     a few such terms plus the print floor is a ~1e-2 dE00 budget; 2e-2 is that. \
     THIS is the row that claims agreement for the v4 mAB path",
);

/// Convenience alias so a reader looking for "the agreement tolerance" finds it
/// under the name the module header uses.
#[must_use]
pub fn emulated_tolerance() -> Tolerance {
    EMULATED
}

/// **§A, the interpolation-free control: the 16 hypercube corners.**
///
/// At a corner every device component is 0 or 1. This profile's A curves are
/// 4096-entry tables whose first and last entries are `0x0000` and `0xFFFF`
/// (**checked at run time**, not assumed — [`XriteArm::a_curve_endpoints`]),
/// so a corner lands on an exact CLUT node. There every n-linear weight is 0 or
/// 1 and the Sakamoto simplex degenerates to its `c0` term: **the method
/// difference is identically zero, by construction rather than by tolerance**,
/// and the run confirms it at exactly `0.0` ([`XriteArm::corner_envelope`]).
///
/// ## ★★ This tolerance is a FUNCTION of the tag's B curves, and the first
/// draft of it was wrong
///
/// The obvious derivation — the one this constant carried on its first run —
/// said that lcms2's quantisation terms *vanish* rather than accumulate at a
/// node: the CLUT input is an exact `u16`, the interpolated value **is** the
/// stored `u16`, and the 2-entry B curves are affine, so what remains is
/// `transicc`'s 4-decimal print, a ΔE00 floor of ≈1×10⁻⁴, and `1×10⁻³` is 10×
/// that.
///
/// **That is true of `A2B1` and false of `A2B0`, and the suite said so**: the
/// `A2B1` arm measured 6.07×10⁻⁵ (the print floor, as predicted) and the
/// `A2B0` arm **FAILED at 1.112×10⁻³**. The cause is not a defect in iccce and
/// it is not a number that wants widening — it is a **term the derivation
/// omitted**. `A2B1`'s B curves are `curv` tables `(0x0000, 0xFFFF)`, the exact
/// identity, so lcms2's `cmsEvalToneCurveFloat` round-trip through `u16` is
/// lossless. `A2B0`'s `L*` B curve is `(0x0808, 0xFFFF)` — a *non-identity*
/// affine map encoding this profile's declared perceptual black — and lcms2
/// evaluates it through `cmsEvalToneCurve16`, rounding **twice**: input to
/// `u16` (lossless at a node) and output to `u16` (not). iccce and the harness
/// evaluate it in `f64`.
///
/// So the floor is **the print floor when every B curve is the identity, and
/// the print floor plus two 16-bit roundings when any of them is not**. One lsb
/// of encoded output is `100/65535 = 1.526×10⁻³` in `L*`; two roundings of ≤½
/// lsb each is ≤1 lsb; `S_L ≥ 1` everywhere off mid-lightness, so the ΔE00
/// bound is ≤1.526×10⁻³, and `2×10⁻³` is that plus the print floor with
/// headroom. **Observed 1.112×10⁻³ — 0.73 lsb, which is what two independent
/// roundings should look like.**
///
/// [`corner_tolerance`] selects between the two **from the tag's own bytes**,
/// so a future profile whose B curves are the identity still gets the tight
/// floor and does not inherit an allowance it does not need.
///
/// This is the record that makes the wide structural gate defensible. Without a
/// node-only control, a gate sized to the whole method envelope could hide a
/// real error just under it and nobody would know.
#[must_use]
pub fn corner_tolerance(b_curves_are_identity: bool) -> Tolerance {
    if b_curves_are_identity {
        Tolerance::new(
            1e-3,
            "the 16 hypercube corners are exact CLUT NODES (this profile's A curves are \
             4096-entry tables with first entry 0x0000 and last 0xFFFF, VERIFIED from the bytes \
             at run time), where n-linear and lcms2's hybrid agree IDENTICALLY and lcms2's \
             quantisation VANISHES rather than accumulates: the CLUT input is an exact u16, the \
             interpolated value IS the stored u16, and THIS TAG's B curves are (0x0000,0xFFFF), \
             the exact identity, so lcms2's u16 round-trip through cmsEvalToneCurveFloat is \
             lossless. What remains is transicc's 4-decimal Lab print, a dE00 floor of ~1e-4. \
             1e-3 is 10x that",
        )
    } else {
        Tolerance::new(
            2e-3,
            "as the identity case, PLUS the term that case does not have: this tag's B curves are \
             NOT (0x0000,0xFFFF) - checked from the bytes - so lcms2 evaluates a non-identity \
             affine 2-entry curve through cmsEvalToneCurve16 and rounds TWICE (input to u16, \
             lossless at a node; output to u16, not), where iccce and the harness use f64. Two \
             roundings of <=1/2 lsb is <=1 lsb, and one lsb of encoded output is 100/65535 = \
             1.526e-3 in L*; S_L >= 1 off mid-lightness, so the dE00 bound is <=1.526e-3. 2e-3 is \
             that plus the print floor. THE FIRST DRAFT OF THIS TOLERANCE OMITTED THE TERM AND \
             THE ROW FAILED; the term was found, not the number moved",
        )
    }
}

/// **§A and §D, the apparatus.** The harness's own `mAB ` reimplementation
/// against `iccce-cmm`'s `LutAbModel`, exact arithmetic, every grid point.
///
/// Everything in §A that substitutes one geometry for another is void if this
/// fails, because the substitution cannot be made inside `crates/` — the
/// shipped engine has exactly one interpolation scheme, by design. 10⁻⁹ is
/// about seven orders above `f64` noise on this arithmetic and about five below
/// anything colorimetric: it can neither pass a real divergence nor fail on
/// rounding.
pub const APPARATUS: Tolerance = Tolerance::new(
    1e-9,
    "harness reimplementation against iccce-cmm's own model, same operations, f64: ~7 orders \
     above f64 noise on this arithmetic and ~5 below anything colorimetric, so it can neither \
     pass a real divergence nor fail on rounding. Every geometry-substitution row in this \
     section is void if it fails",
);

/// **§B and §C, the population sweep's device gate.**
///
/// Metric: max |Δ| over every component of every grid point, normalised device
/// units (0..1), lcms2's output clamped into `[0,1]` first.
///
/// ## Where 4×10⁻³ comes from — the named rival, not the observation
///
/// Every destination in §B is a CMYK or Lab-PCS **output** LUT, and
/// `cmsio1.c`'s `_cmsReadOutputLUT` does
/// `if (cmsGetPCS(hProfile) == cmsSigLabData) ChangeInterpolationToTrilinear(Lut);`
/// — so lcms2 evaluates these tables with *iccce's* geometry and the
/// interpolation-method envelope is **identically zero in this direction**
/// (Pass 4b finding 1). That removes the term that dominates Pass 4, and it
/// also removes the obvious way to derive a bound.
///
/// A closed-form union bound was tried and is useless here for exactly the
/// reason Pass 4 records: lcms2's tabulated-TRC rounding is worth ≤1.53×10⁻⁵ in
/// linear RGB, `da*/dX ≤ 4038` carries that to ≈6.2×10⁻² in `a*` (2.4×10⁻⁴
/// encoded), and the steepest of the six measured destination tables has a
/// node-to-node slope of ≈14.8 per encoded unit — whose product over three
/// channels is ≈1.1×10⁻², **wider than the rival it is supposed to
/// discriminate**. A bound that cannot tell the two candidates apart is not a
/// bound, it is a formality.
///
/// **The number is derived from the discrimination requirement instead.** The
/// named rival for every §B row is *"lcms2 had NOT forced trilinear and had used
/// its default tetrahedral geometry"*, and that counterfactual is computed at
/// run time from each destination's own `B2A*` table
/// ([`SweepArm::counterfactual`]). The smallest counterfactual measured over
/// the destinations in this sweep is ≈1.2×10⁻², so **4×10⁻³ leaves every row at
/// least 3× of discrimination against its rival** — and the `separation` column
/// states that multiple per row rather than asserting it here.
///
/// **What it also catches**, which is the other half of a tolerance's job: the
/// v2/v4 legacy Lab encoding error, `ARCHITECTURE.md` §2's named hazard, moves
/// `L*` at white by ≈0.39 and the encoded `a*`/`b*` scale by 0.4 %, which
/// through a slope of 5–15 is ≥10⁻² device — 2.5× this gate at minimum.
///
/// **★ What it does NOT do: claim agreement.** There is no attribution row for
/// §B, because the harness has no `mft2` B2A model to substitute lcms2's
/// arithmetic into (Pass 4b built one for `mft1`/`lut8` only). §B is therefore
/// a *structural* gate with a stated rival, and the honest reading of a green
/// §B row is "no structural error, and not the tetrahedral rival" — **not**
/// "these two agree to 4×10⁻³". `docs/TOLERANCES.md` §3.7.3 records that gap as
/// owed work rather than closing it with a wider claim.
///
/// **GRID-DEPENDENT and arithmetic, not perceptual.** §2's 1.0 ΔE2000 anchor is
/// irrelevant to it and must not be cited in its support.
pub const SWEEP_DEVICE: Tolerance = Tolerance::new(
    4e-3,
    "derived from the DISCRIMINATION REQUIREMENT, not from the observation. lcms2 forces \
     trilinear for a Lab-PCS output LUT (_cmsReadOutputLUT), so the interpolation-method \
     envelope is identically ZERO in this direction and the named rival is 'lcms2 had used its \
     default tetrahedral geometry' - computed at run time from each destination's own B2A table, \
     smallest ~1.2e-2 over this sweep. 4e-3 leaves every row >=3x discrimination against that \
     rival. A closed-form union bound was tried and DISCARDED as useless (~1.1e-2, wider than \
     the rival). It also catches the legacy-Lab decode error, >=1e-2 device through these \
     slopes. It does NOT claim agreement: no attribution row exists for mft2 B2A. \
     GRID-DEPENDENT; arithmetic, NOT perceptual",
);

/// **§B, the two rows that are REPORTED rather than graded**, and §B's BPC rows.
///
/// A tolerance of `∞` is a decision, not an omission, and it is made in exactly
/// two circumstances in this module:
///
/// 1. **ICC-absolute out of `sRGB` or `Adobe RGB (1998)`.** The two
///    implementations use *different destination media whites* there, because
///    lcms2 substitutes D50 for a v2 display profile's `wtpt` and iccce uses the
///    encoded value. That is not an arithmetic disagreement to be gated; it is a
///    policy difference whose mechanism §B grades separately, on the profile's
///    own bytes, in `authoring/colorants-sum-to-d50`.
/// 2. **Anything with `--bpc`.** iccce estimates the destination black point by
///    ISO/CD 18619 4.2.5 and lcms2 by `cmsDetectDestinationBlackPoint`; the two
///    are different functions, ICC.1 has **no normative BPC text at all**
///    (corpus A27/A42), and Pass 5b/5c established that they disagree. Gating
///    that disagreement would be gating a choice neither standard makes.
///
/// In both cases the row still runs, still prints its number, and carries the
/// mechanism in its `separation`. `docs/TOLERANCES.md` §1.1: a reported row has
/// **no discriminating power** and the report says so on the line.
pub const REPORTED: Tolerance = Tolerance::new(
    f64::INFINITY,
    "REPORTED, NOT GRADED. Either the two implementations use different destination media whites \
     (lcms2 substitutes D50 for a v2 display profile's wtpt; the MECHANISM is graded separately \
     on the profile's own bytes) or the row requests BPC, where iccce uses ISO/CD 18619 4.2.5 \
     and lcms2 uses cmsDetectDestinationBlackPoint and ICC.1 has NO normative BPC text (A27/A42). \
     Gating either would gate a choice no standard makes",
);

/// **§B, the authoring row — a DERIVED EXPECTATION on the profile's own bytes.**
///
/// The claim: a v2 display profile's `rXYZ + gXYZ + bXYZ` **shall** sum to the
/// PCS white point, because clause 6.3.4.3 / Table 14 fixes the PCS white at
/// `0.9642 / 1.0000 / 0.8249` and a matrix/TRC profile reaches PCS white when
/// all three channels are at full scale.
///
/// `2×10⁻⁴` is the encoding floor of that claim and nothing else: the colorants
/// are `s15Fixed16`, one lsb is `1/65536 = 1.526×10⁻⁵`, three of them sum to
/// `4.6×10⁻⁵`, and the PCS white itself is stated to four decimals in Table 14
/// so it is only known to ±5×10⁻⁵ per component. `2×10⁻⁴` is ≈2× that sum.
///
/// **Why this row exists.** It is what turns "iccce and lcms2 disagree at
/// ICC-absolute" from an unresolved oracle divergence into a settled finding.
/// If the colorants sum to D50 then the profile's PCS data *is* D50-adapted;
/// ICC.1:2001-04 Annex A.3.1.1 says that when chromatic adaptation is applied
/// to the PCS values it should be applied to `wtpt` as well; and a `wtpt` of
/// D65 with no `chad` in a profile whose colorants sum to D50 is therefore a
/// **defect of authorship**, not a second reading of the clause. The row grades
/// the antecedent — everything else follows from clause text.
pub const COLORANTS_SUM: Tolerance = Tolerance::new(
    2e-4,
    "the s15Fixed16 encoding floor of the claim and nothing else: one lsb is 1/65536 = 1.526e-5, \
     three colorants sum three of them (4.6e-5), and Table 14 states the PCS white to 4 decimals \
     so it is known only to +/-5e-5 per component. 2e-4 is ~2x that sum. NOT perceptual and NOT \
     fitted to any observation - it is the precision of the encoding the claim is made in",
);

/// **§A and §D, a count of byte-identical tag-data blocks.**
///
/// `0.0` is honestly available here because the quantity is a **count of
/// integer comparisons on file bytes**, not a floating-point residual: there is
/// no rounding for a tolerance to absorb. `TOLERANCES.md` §3.4.4's rule that
/// `0.0` requires "the same operations in the same order" is about arithmetic
/// and does not reach this.
///
/// ## Why it must be measured before anything is collected
///
/// **The X-Rite v4 profile aliases `A2B1 ≡ A2B2` (media-relative and
/// saturation are one block of bytes at two offsets) while both ECI profiles in
/// this corpus alias `A2B0 ≡ A2B2` (perceptual and saturation).** An engine
/// that hard-coded either pairing would be wrong on the other vendor's files,
/// and a suite that ran "all four intents" over a set of profiles without
/// checking which tags are shared would collect green lines that measure
/// nothing — the exact failure `CLAUDE.md` rule 5 exists to prevent. A null
/// that is null by construction has to be *identified* before it is collected,
/// never explained afterwards.
pub const ALIASED_TAGS: Tolerance = Tolerance::new(
    0.0,
    "a max |difference| over two tag-data blocks compared BYTE BY BYTE with no parser in the way; \
     an integer comparison, not a floating-point residual, so 0.0 needs no rounding allowance. \
     It exists because intent-tag aliasing DIFFERS BY VENDOR in this corpus (X-Rite aliases \
     A2B1==A2B2, ECI aliases A2B0==A2B2) and a run that did not check would collect green lines \
     measuring nothing",
);

/// **§D, the swap rows — a DERIVED EXPECTATION with no oracle in it.**
///
/// For a matrix/TRC profile, clause 6.3.4 / F.3 gives PCS `XYZ` for device
/// `(1,0,0)` as `rXYZ · TRC_r(1)`, and a `curv` is normalised so that its last
/// entry maps to 1.0 — so the expected PCS `XYZ` for full red **is the `rXYZ`
/// tag's own three numbers**, exactly. No interpolation, no oracle, no
/// published value needed: the profile states its own answer.
///
/// `1×10⁻⁶` is the `s15Fixed16` lsb (`1.526×10⁻⁵`) divided by ~15, i.e. it is
/// **tighter than the encoding**, which is legitimate only because both sides
/// read the *same* stored integers and the comparison is of two decodings of
/// one number rather than of two measurements. It cannot absorb a wrong
/// colorant (one lsb, 15× this) let alone a wrong *channel* (a whole primary
/// apart).
pub const SWAP_EXACT: Tolerance = Tolerance::new(
    1e-6,
    "both sides decode the SAME stored s15Fixed16 integers, so this compares two decodings of one \
     number and not two measurements: 1e-6 is ~1/15 of the encoding lsb (1.526e-5) and is \
     therefore tighter than the encoding on purpose. It cannot absorb a wrong colorant (one lsb, \
     15x this) let alone a wrong channel (a whole primary apart)",
);

/// **§D, the duplicate-trap row.** Two files in the corpus carry the same
/// `desc` and different SHA-256s. They must produce identical output or one of
/// them is not what its description says.
///
/// Graded at [`SWAP_EXACT`]'s reasoning taken one step further: this is the
/// same engine, the same code path, the same arithmetic, on two files whose
/// colour-bearing tags are byte-identical (checked). `0.0` is available and is
/// used, because *any* difference at all means the two files differ somewhere
/// that matters and the row should say so.
pub const DUPLICATE_EXACT: Tolerance = Tolerance::new(
    0.0,
    "the same engine, the same code path, the same arithmetic, on two files whose colour-bearing \
     tags are byte-identical (verified in the same run). Not a measurement comparison: any \
     non-zero difference means the two files differ somewhere that matters",
);

// ===========================================================================
// Why a section could not run
// ===========================================================================

/// Why a section could not run. Same contract as [`crate::pass4::Unavailable`]:
/// **a missing licensed corpus is a SKIP** (it cannot be shipped, so its
/// absence is the normal case on every machine but the operator's), a broken
/// oracle or an unparsable profile is an **ERROR**, and neither is ever a pass.
#[derive(Debug)]
pub enum Unavailable {
    Skip(String),
    Error(String),
}

impl Unavailable {
    fn reason(&self) -> &str {
        match self {
            Unavailable::Skip(s) | Unavailable::Error(s) => s,
        }
    }
}

// ===========================================================================
// Harness-side model of an `mAB ` pipeline, with switchable CLUT geometry
// ===========================================================================

/// Evaluate a curve element. **Only the shapes this corpus actually contains
/// are handled**; anything else panics rather than returning a plausible
/// number, because a silently-wrong curve is indistinguishable from a correct
/// one in the output (`CLAUDE.md` rule 1).
fn curve_eval(c: &CurveElement, x: f64) -> f64 {
    match c {
        CurveElement::Curve(Curve::Identity) => x,
        CurveElement::Curve(Curve::Gamma(g)) => x.clamp(0.0, 1.0).powf(f64::from(g.0) / 256.0),
        CurveElement::Curve(Curve::Table(t)) => {
            assert!(t.len() >= 2, "a curv with <2 entries is not a table");
            let n = t.len();
            let x = x.clamp(0.0, 1.0);
            let pos = x * (n - 1) as f64;
            let i = (pos.floor() as usize).min(n - 2);
            let f = pos - i as f64;
            let a = f64::from(t[i]) / 65535.0;
            let b = f64::from(t[i + 1]) / 65535.0;
            a + (b - a) * f
        }
        CurveElement::Parametric(_) => {
            panic!(
                "passg's mAB model does not implement parametric curves; no corpus member needs one"
            )
        }
    }
}

/// Is this curve element **exactly** the identity, in the sense that matters to
/// lcms2's 16-bit round trip?
///
/// The question is not "does it evaluate to `x` in `f64`" but "does
/// `cmsEvalToneCurve16` return its input unchanged". That is true for an absent
/// curve, for a `curv` with `count == 0` (clause 10.6: *"an identity response
/// is assumed"*), for `Gamma(1.0)`, and for a table whose entries are exactly
/// `0x0000 … 0xFFFF` evenly spaced — of which only the 2-entry case
/// `(0x0000, 0xFFFF)` occurs in this corpus. Anything else rounds, and the
/// corner tolerance has to know.
fn is_identity_curve(c: &CurveElement) -> bool {
    match c {
        CurveElement::Curve(Curve::Identity) => true,
        CurveElement::Curve(Curve::Gamma(g)) => g.0 == 0x0100,
        CurveElement::Curve(Curve::Table(t)) => t.len() == 2 && t[0] == 0x0000 && t[1] == 0xFFFF,
        CurveElement::Parametric(_) => false,
    }
}

fn read_lut_ab(p: &Profile, sig: Signature) -> Option<LutAB> {
    let e = p.tags.iter().find(|t| t.sig == sig)?;
    match p.decode_tag(e) {
        Some(Ok(d)) => match d.data {
            TagData::LutAToB(l) | TagData::LutBToA(l) => Some(l),
            _ => None,
        },
        _ => None,
    }
}

/// Raw tag-data bytes, straight out of the file with **no parser in the way** —
/// which is the point: the aliasing rows must not be able to inherit a decoder
/// bug from the code they are checking.
fn raw_tag(bytes: &[u8], sig: Signature) -> Option<&[u8]> {
    let count = u32::from_be_bytes(bytes.get(128..132)?.try_into().ok()?) as usize;
    for i in 0..count {
        let o = 132 + i * 12;
        let s = u32::from_be_bytes(bytes.get(o..o + 4)?.try_into().ok()?);
        if Signature(s) == sig {
            let off = u32::from_be_bytes(bytes.get(o + 4..o + 8)?.try_into().ok()?) as usize;
            let len = u32::from_be_bytes(bytes.get(o + 8..o + 12)?.try_into().ok()?) as usize;
            return bytes.get(off..off + len);
        }
    }
    None
}

/// An `XYZType` tag's three `s15Fixed16` numbers, decoded from the raw bytes.
fn raw_xyz(bytes: &[u8], sig: Signature) -> Option<Xyz> {
    let d = raw_tag(bytes, sig)?;
    if d.len() < 20 {
        return None;
    }
    let g = |o: usize| f64::from(i32::from_be_bytes(d[o..o + 4].try_into().unwrap())) / 65536.0;
    Some(Xyz {
        x: g(8),
        y: g(12),
        z: g(16),
    })
}

/// The harness's own `mAB ` pipeline: A curves → CLUT (either geometry) →
/// B curves → v4 PCSLAB decode.
///
/// It exists because the substitution §A depends on **cannot be made inside
/// `crates/`**: the shipped engine has one interpolation scheme by design, and
/// adding a second one to it so that a test could select it would be a change
/// to the code under test made for the benefit of the test.
///
/// Its correctness is not assumed — [`XriteArm::apparatus`] grades it against
/// `iccce-cmm`'s `LutAbModel` over the same grid, and every geometry-substitution
/// row in §A is void if that row fails.
struct MabPipeline {
    clut: HarnessClut,
    a_curves: Vec<CurveElement>,
    b_curves: Vec<CurveElement>,
}

impl MabPipeline {
    fn build(lut: &LutAB) -> Result<MabPipeline, String> {
        let clut = lut.clut.as_ref().ok_or("mAB has no CLUT element")?;
        let dims: Vec<usize> = (0..usize::from(lut.input_chan))
            .map(|i| usize::from(clut.grid_points[i]))
            .collect();
        let data: Vec<f64> = match &clut.samples {
            ClutSamples::U16(v) => v.iter().map(|&s| f64::from(s) / 65535.0).collect(),
            ClutSamples::U8(v) => v.iter().map(|&s| f64::from(s) / 255.0).collect(),
        };
        if lut.matrix.is_some() {
            // Not a refusal to be papered over: the X-Rite tags carry no
            // matrix element (its offset is 0), and a pipeline that silently
            // ignored one would be wrong by a uniform colour cast — the exact
            // failure `lut.rs`'s own comment names.
            return Err(
                "this mAB carries a matrix element, which passg's model does not apply".into(),
            );
        }
        Ok(MabPipeline {
            clut: HarnessClut::new(dims, usize::from(lut.output_chan), data),
            a_curves: lut.a_curves.as_ref().ok_or("mAB has no A curves")?.clone(),
            b_curves: lut.b_curves.as_ref().ok_or("mAB has no B curves")?.clone(),
        })
    }

    /// Device (0..1 per channel) → D50 CIELAB.
    ///
    /// The final step is the **v4** PCSLAB decode, ICC.1:2022 Table 42:
    /// `L* = 100·n`, `a*` and `b*` = `255·n − 128`. It is v4 and not the legacy
    /// v2 decode because `lutAToBType` is a v4 tag type and clause 10.13 fixes
    /// its encoding; getting this wrong is `ARCHITECTURE.md` §2's single
    /// richest source of CMM bugs and moves `L*` at white by ≈0.39.
    fn eval(&self, device: &[f64], scheme: Scheme) -> Lab {
        let ins: Vec<f64> = device
            .iter()
            .enumerate()
            .map(|(i, v)| curve_eval(&self.a_curves[i], *v))
            .collect();
        let mut mid = vec![0.0; 3];
        self.clut.eval(&ins, scheme, &mut mid);
        let o: Vec<f64> = mid
            .iter()
            .enumerate()
            .map(|(i, v)| curve_eval(&self.b_curves[i], *v))
            .collect();
        Lab {
            l: o[0] * 100.0,
            a: o[1] * 255.0 - 128.0,
            b: o[2] * 255.0 - 128.0,
        }
    }
}

// ===========================================================================
// §A — the v4 vendor mAB path
// ===========================================================================

/// One graded arm of §A: a single `A2B*` tag of the X-Rite v4 profile,
/// measured at the intent that selects it.
pub struct XriteArm {
    pub tag: &'static str,
    pub intent: Intent,
    /// Which lcms2 PCS built-in the oracle was asked for, and **why it matters**:
    /// `*Lab4` is a v4 profile and lcms2's `_cmsLinkProfiles` sets forced BPC
    /// for perceptual into a v4 destination, so the perceptual arm uses `*Lab2`
    /// to keep the gate shut. See [`Section`]'s `forced_bpc_l_star`.
    pub pcs: &'static str,
    /// max ΔE2000, harness under **iccce's** n-linear geometry vs the oracle.
    pub raw: f64,
    /// max ΔE2000, harness under **lcms2's** geometry vs the oracle.
    pub emulated: f64,
    /// max ΔE2000 between the two geometries over the same grid — **computed
    /// from the CLUT's bytes alone, with no oracle output in it**.
    pub method_envelope: f64,
    /// The same at the 16 exact-node corners. Zero by construction.
    pub corner_envelope: f64,
    /// max ΔE2000 at those corners, harness vs oracle.
    pub corners: f64,
    /// max ΔE2000, harness n-linear vs `iccce-cmm`'s `LutAbModel`.
    pub apparatus: f64,
    /// The A curves' first and last decoded values — `(0.0, 1.0)` is what makes
    /// the corners exact nodes, and it is checked rather than assumed.
    pub a_curve_endpoints: (f64, f64),
    /// ★ Whether **every** B curve of this tag is the exact identity
    /// `(0x0000, 0xFFFF)`. It selects the corner tolerance
    /// ([`corner_tolerance`]) because lcms2's `u16` round-trip through a
    /// non-identity 2-entry curve rounds twice and a lossless identity does
    /// not. Read from the tag's own bytes; the first draft of this pass assumed
    /// it and the assumption was false for `A2B0`.
    pub b_curves_are_identity: bool,
    pub points: usize,
}

/// Everything §A produces.
pub struct Section {
    pub structure: String,
    pub arms: Vec<XriteArm>,
    /// max |Δ| over the raw bytes of `A2B1` and `A2B2` — the vendor's aliasing.
    pub a2b1_vs_a2b2_bytes: f64,
    /// ★ The **rival**: max |Δ| over the raw bytes of this profile's `A2B0` and
    /// `A2B2`. That is the pairing the ECI profiles use, and it is what an
    /// engine would have read for saturation had it acquired the ECI habit. On
    /// this profile the two blocks are **different**, which is what gives the
    /// aliasing row any power at all — the first draft named the ECI file's own
    /// `A2B0`/`A2B2` difference instead, which is zero *because* ECI aliases
    /// them, and the row printed `ZERO-SEPARATION`.
    pub a2b0_vs_a2b2_bytes: f64,
    /// The same for the ECI profile's `A2B0`/`A2B2`, kept because it is the
    /// evidence that the aliasing is a **vendor** choice and not a format one.
    pub eci_a2b0_vs_a2b2_bytes: f64,
    /// ★ The profile's own perceptual black, read out of its `A2B0` B-curve's
    /// first entry, decoded through the v4 PCSLAB `L*` scale.
    pub derived_perceptual_black_l: f64,
    /// What lcms2 actually prints for full ink at the perceptual intent into a
    /// **v2** PCS, where its forced-BPC gate does not fire.
    pub lcms2_perceptual_black_l_v2pcs: f64,
    /// The same into a **v4** PCS, where it does. Reported, not graded.
    pub lcms2_perceptual_black_l_v4pcs: f64,
    /// End-to-end device rows, `(intent, bpc, max, mean, refusal)`.
    pub end_to_end: Vec<(Intent, Bpc, f64, f64, Option<String>)>,
    /// ★ **The CLUT interpolation-method envelope PROPAGATED through the actual
    /// destination model, point by point, per tag** — normalised device units.
    ///
    /// `(tag, envelope)`. It is what a device-space gate on the `A2B` direction
    /// has to be sized by, and the reason the first draft of this pass failed
    /// three end-to-end rows: it reused §B's tolerance, which is derived for the
    /// `B2A` direction where the method envelope is **identically zero** because
    /// lcms2 forces trilinear. In the `A2B` direction it is not zero, it is the
    /// dominant term, and a bound that ignores it is not a bound.
    ///
    /// Computed by pushing the harness's *two* Lab answers — n-linear and
    /// lcms2's geometry — through `MatrixTrc::pcs_to_device` for the actual
    /// destination profile and taking the largest device difference over the
    /// grid. **No lcms2 output enters it**; it is the two published algorithms
    /// and two files.
    pub propagated_envelope: Vec<(&'static str, f64)>,
}

/// The end-to-end gate for §A, sized from [`Section::propagated_envelope`].
///
/// `× 1.25` for the same two unmodelled things [`structural_tolerance_from`]
/// allows: maxima between grid points, and lcms2's fixed-point arithmetic where
/// the envelope uses `f64`. As there, this admits the whole legitimate
/// interpolation difference and therefore **detects structural error rather
/// than claiming agreement** — the agreement claim for this profile is made in
/// the PCS, by the emulated-geometry row, which is where the destination model
/// is not in the way.
///
/// `+1×10⁻⁴` is added so that a tag whose envelope happens to be zero still has
/// a floor covering `transicc`'s own print precision, rather than a gate of
/// exactly `0` that no arithmetic could pass.
#[must_use]
pub fn propagated_gate(envelope: f64) -> Tolerance {
    Tolerance::new(
        envelope * 1.25 + 1e-4,
        "the CLUT INTERPOLATION-METHOD envelope PROPAGATED through the actual destination model \
         point by point (iccce's n-linear against lcms2's Eval4Inputs hybrid, both pushed through \
         MatrixTrc::pcs_to_device for the destination file), x1.25 for between-point maxima and \
         lcms2's fixed-point arithmetic, plus 1e-4 for transicc's print floor. NO lcms2 output \
         enters the derivation. This is the A2B direction, where the method envelope is the \
         DOMINANT term - unlike the B2A direction, where lcms2 forces trilinear and it is zero, \
         and where SWEEP_DEVICE therefore applies instead. It admits the whole legitimate \
         interpolation difference and so detects STRUCTURAL error only; the agreement claim for \
         this profile is the PCS-side emulated-geometry row's",
    )
}

/// Build §A. Errors are returned rather than propagated so that a missing
/// corpus produces labelled SKIP lines instead of an absence.
///
/// # Errors
/// [`Unavailable::Skip`] when the corpus or the shipped binary is absent;
/// [`Unavailable::Error`] when a profile present on disk cannot be parsed or
/// the oracle fails, because at that point something is wrong that a skip would
/// conceal.
pub fn analyse_xrite(oracle: &Oracle, iccce: &Iccce) -> Result<Section, Unavailable> {
    let dir = corpus_dir();
    let path = dir.join(file::XRITE_V4);
    let bytes = std::fs::read(&path).map_err(|e| {
        Unavailable::Skip(format!(
            "ghent-v50 corpus not present ({}): set $ICCCE_PRIVATE_FIXTURES. \
             It is licensed and cannot be committed, so absence is the normal case",
            e
        ))
    })?;
    let profile = Profile::parse(&bytes).map_err(|e| Unavailable::Error(e.to_string()))?;

    let grid = crate::pass4::grid();
    let corners: Vec<[f64; 4]> = (0..16u32)
        .map(|m| {
            [
                f64::from((m >> 3) & 1),
                f64::from((m >> 2) & 1),
                f64::from((m >> 1) & 1),
                f64::from(m & 1),
            ]
        })
        .collect();

    let mut arms = Vec::new();
    // `A2B1` at media-relative, and `A2B0` at perceptual through a **v2** PCS.
    // `A2B2` is deliberately NOT given an arm of its own: it is byte-identical
    // to `A2B1` in this profile, so a saturation arm would reproduce the
    // media-relative one exactly and add a green line that measures nothing.
    // The aliasing is graded instead, at exactly zero.
    for (tag_name, sig, intent, pcs) in [
        ("A2B1", tag::A2B1, Intent::RelativeColorimetric, "*Lab4"),
        ("A2B0", tag::A2B0, Intent::Perceptual, "*Lab2"),
    ] {
        let lut = read_lut_ab(&profile, sig)
            .ok_or_else(|| Unavailable::Error(format!("{tag_name} is not a decodable mAB")))?;
        let pipe = MabPipeline::build(&lut).map_err(Unavailable::Error)?;
        let model = iccce_cmm::lut_ab::LutAbModel::from_lut_ab(
            &lut,
            iccce_cmm::lut_transform::PcsKind::Lab,
        )
        .map_err(|e| Unavailable::Error(format!("{tag_name} LutAbModel: {e}")))?;

        let mut envelope = 0.0f64;
        let mut apparatus = 0.0f64;
        for q in &grid {
            let n = pipe.eval(q, Scheme::NLinear);
            let l = pipe.eval(q, Scheme::Lcms2Default);
            envelope = envelope.max(delta_e_2000(n, l));
            match model.device_to_pcs(q) {
                Some(iccce_cmm::lut_transform::PcsValue::Lab(m)) => {
                    apparatus = apparatus.max(delta_e_2000(n, m));
                }
                _ => {
                    return Err(Unavailable::Error(format!(
                        "iccce-cmm's LutAbModel refused a CMYK grid point on {tag_name}"
                    )));
                }
            }
        }
        let mut corner_envelope = 0.0f64;
        for q in &corners {
            corner_envelope = corner_envelope.max(delta_e_2000(
                pipe.eval(q, Scheme::NLinear),
                pipe.eval(q, Scheme::Lcms2Default),
            ));
        }

        let pcs_space = if pcs == "*Lab4" {
            Space::lab_v4()
        } else {
            Space::lab_v2()
        };
        let req = Request {
            input: Space::profile(&path),
            output: pcs_space.clone(),
            intent,
            precalc: Precalc::Exact,
            bpc: Bpc::Off,
            values: grid
                .iter()
                .flat_map(|q| q.iter().map(|v| v * 100.0))
                .collect(),
        };
        let theirs = oracle
            .convert_batch_shaped(&req, 4, 3)
            .map_err(|e| Unavailable::Error(e.to_string()))?;
        let mut raw = 0.0f64;
        let mut emulated = 0.0f64;
        for (i, q) in grid.iter().enumerate() {
            let t = Lab {
                l: theirs[i][0],
                a: theirs[i][1],
                b: theirs[i][2],
            };
            raw = raw.max(delta_e_2000(pipe.eval(q, Scheme::NLinear), t));
            emulated = emulated.max(delta_e_2000(pipe.eval(q, Scheme::Lcms2Default), t));
        }

        let req_c = Request {
            input: Space::profile(&path),
            output: pcs_space,
            intent,
            precalc: Precalc::Exact,
            bpc: Bpc::Off,
            values: corners
                .iter()
                .flat_map(|q| q.iter().map(|v| v * 100.0))
                .collect(),
        };
        let theirs_c = oracle
            .convert_batch_shaped(&req_c, 4, 3)
            .map_err(|e| Unavailable::Error(e.to_string()))?;
        let mut corner_max = 0.0f64;
        for (i, q) in corners.iter().enumerate() {
            let t = Lab {
                l: theirs_c[i][0],
                a: theirs_c[i][1],
                b: theirs_c[i][2],
            };
            corner_max = corner_max.max(delta_e_2000(pipe.eval(q, Scheme::Lcms2Default), t));
        }

        arms.push(XriteArm {
            tag: tag_name,
            intent,
            pcs,
            raw,
            emulated,
            method_envelope: envelope,
            corner_envelope,
            corners: corner_max,
            apparatus,
            a_curve_endpoints: (
                curve_eval(&pipe.a_curves[0], 0.0),
                curve_eval(&pipe.a_curves[0], 1.0),
            ),
            b_curves_are_identity: pipe.b_curves.iter().all(is_identity_curve),
            points: grid.len(),
        });
    }

    // --- vendor tag aliasing, read from the raw bytes -----------------------
    let byte_diff = |a: Option<&[u8]>, b: Option<&[u8]>| -> f64 {
        match (a, b) {
            (Some(x), Some(y)) if x.len() == y.len() => x
                .iter()
                .zip(y)
                .map(|(p, q)| f64::from(*p as i16 - *q as i16).abs())
                .fold(0.0f64, f64::max),
            _ => f64::INFINITY,
        }
    };
    let a2b1_vs_a2b2 = byte_diff(raw_tag(&bytes, tag::A2B1), raw_tag(&bytes, tag::A2B2));
    let a2b0_vs_a2b2 = byte_diff(raw_tag(&bytes, tag::A2B0), raw_tag(&bytes, tag::A2B2));
    let eci_bytes = std::fs::read(dir.join(file::ISOCOATED))
        .map_err(|e| Unavailable::Error(format!("ISO Coated v2: {e}")))?;
    let eci_a2b0_vs_a2b2 = byte_diff(
        raw_tag(&eci_bytes, tag::A2B0),
        raw_tag(&eci_bytes, tag::A2B2),
    );

    // --- the profile's own perceptual black, from its A2B0 B-curve ----------
    // clause 10.13: the `B` curves of an `mAB ` are applied last, so the first
    // entry of the L* curve is the smallest L* the tag can emit. Decoding it
    // through the v4 PCSLAB scale gives the profile's declared perceptual
    // black. Nothing here comes from an implementation.
    let a2b0 = read_lut_ab(&profile, tag::A2B0)
        .ok_or_else(|| Unavailable::Error("A2B0 not decodable".into()))?;
    let derived_black = match &a2b0.b_curves.as_ref().and_then(|c| c.first().cloned()) {
        Some(CurveElement::Curve(Curve::Table(t))) if !t.is_empty() => {
            f64::from(t[0]) / 65535.0 * 100.0
        }
        _ => {
            return Err(Unavailable::Error(
                "A2B0's first B curve is not a table; the derived-black row cannot be built".into(),
            ));
        }
    };
    // Full ink: the single point where this profile's declared perceptual
    // black lives, and the only one the derived-black row needs.
    let full_ink = [100.0_f64, 100.0, 100.0, 100.0];
    let black_l = |pcs: Space| -> Result<f64, Unavailable> {
        let req = Request {
            input: Space::profile(&path),
            output: pcs,
            intent: Intent::Perceptual,
            precalc: Precalc::Exact,
            bpc: Bpc::Off,
            values: full_ink.to_vec(),
        };
        oracle
            .convert_batch_shaped(&req, 4, 3)
            .map(|v| v[0][0])
            .map_err(|e| Unavailable::Error(e.to_string()))
    };
    let black_v2 = black_l(Space::lab_v2())?;
    let black_v4 = black_l(Space::lab_v4())?;

    // --- the method envelope propagated through the ACTUAL destination -------
    // Both arms are the harness's own pipeline under the two published
    // geometries, pushed through the destination profile's own matrix/TRC
    // model. No lcms2 output is in it, which is what makes it usable as a
    // tolerance rather than as a fitted residual.
    let srgb = dir.join(file::SRGB);
    let dst_bytes = std::fs::read(&srgb).map_err(|e| Unavailable::Error(e.to_string()))?;
    let dst_profile = Profile::parse(&dst_bytes).map_err(|e| Unavailable::Error(e.to_string()))?;
    let dst_model = MatrixTrc::from_profile(&dst_profile)
        .map_err(|e| Unavailable::Error(format!("destination sRGB model: {e}")))?;
    let mut propagated_envelope = Vec::new();
    for (tag_name, sig) in [("A2B0", tag::A2B0), ("A2B1", tag::A2B1)] {
        let lut = read_lut_ab(&profile, sig)
            .ok_or_else(|| Unavailable::Error(format!("{tag_name} not decodable")))?;
        let pipe = MabPipeline::build(&lut).map_err(Unavailable::Error)?;
        let mut worst = 0.0f64;
        for q in &grid {
            let n = pipe.eval(q, Scheme::NLinear).to_xyz(iccce_color::D50);
            let l = pipe.eval(q, Scheme::Lcms2Default).to_xyz(iccce_color::D50);
            if let (Ok(a), Ok(b)) = (dst_model.pcs_to_device(n), dst_model.pcs_to_device(l)) {
                for (x, y) in a.iter().zip(&b) {
                    worst = worst.max((x - y).abs());
                }
            }
        }
        propagated_envelope.push((tag_name, worst));
    }

    // --- end to end, through the shipped binary -----------------------------
    let rows: Vec<Vec<f64>> = grid.iter().map(|q| q.to_vec()).collect();
    let mut end_to_end = Vec::new();
    for intent in ALL_INTENTS {
        for bpc in [Bpc::Off, Bpc::On] {
            match measure_device(
                oracle, iccce, &path, &srgb, &rows, 100.0, 3, true, intent, bpc,
            ) {
                Ok((max, mean)) => end_to_end.push((intent, bpc, max, mean, None)),
                Err(refusal) => end_to_end.push((intent, bpc, f64::NAN, f64::NAN, Some(refusal))),
            }
        }
    }

    let structure = format!(
        "X-Rite GWG_ICC_v4_testprofile v{:08X} {} {}->{} | mAB A2B* 4->3 grid 7^4, mBA B2A* 3->4 \
         grid 17^3 | {} CMYK points x {} arms + 16 corners | end-to-end -> Ghent sRGB",
        profile.header.version.raw,
        profile.header.device_class,
        profile.header.color_space,
        profile.header.pcs,
        grid.len(),
        arms.len()
    );

    Ok(Section {
        structure,
        arms,
        a2b1_vs_a2b2_bytes: a2b1_vs_a2b2,
        a2b0_vs_a2b2_bytes: a2b0_vs_a2b2,
        eci_a2b0_vs_a2b2_bytes: eci_a2b0_vs_a2b2,
        derived_perceptual_black_l: derived_black,
        lcms2_perceptual_black_l_v2pcs: black_v2,
        lcms2_perceptual_black_l_v4pcs: black_v4,
        end_to_end,
        propagated_envelope,
    })
}

const ALL_INTENTS: [Intent; 4] = [
    Intent::Perceptual,
    Intent::RelativeColorimetric,
    Intent::Saturation,
    Intent::AbsoluteColorimetric,
];

/// One end-to-end device comparison. Returns `(max, mean)` in **normalised
/// device units**, or the shipped binary's refusal text.
///
/// ★ `transicc` prints RGB and gray as `0..255` and ink spaces as `0..100`, so
/// the scale is a parameter and not a constant. A flat `/100` inflates every
/// RGB destination by 2.55× and produced, once, a residual large enough to be
/// reported as a finding (`pass5c.rs`'s `dev_scale` doc).
#[allow(clippy::too_many_arguments)]
fn measure_device(
    oracle: &Oracle,
    iccce: &Iccce,
    src: &Path,
    dst: &Path,
    rows: &[Vec<f64>],
    src_scale: f64,
    out_channels: usize,
    dst_is_rgb_or_gray: bool,
    intent: Intent,
    bpc: Bpc,
) -> Result<(f64, f64), String> {
    let req = Request {
        input: Space::profile(src),
        output: Space::profile(dst),
        intent,
        precalc: Precalc::Exact,
        bpc,
        values: rows.iter().flatten().map(|v| v * src_scale).collect(),
    };
    let theirs = oracle
        .convert_batch_shaped(&req, rows[0].len(), out_channels)
        .map_err(|e| format!("oracle: {e}"))?;
    let mine = iccce
        .transform_rows_shaped_bpc(src, dst, intent, rows, out_channels, bpc == Bpc::On)
        .map_err(|e| {
            // A refusal is a deliverable, not harness breakage: an engine that
            // declines by name where it cannot estimate is doing what
            // `CLAUDE.md` rule 6 requires, and the boundary of the subset is
            // part of the coverage statement.
            let s = format!("{e}");
            s.lines().last().unwrap_or("refused").trim().to_string()
        })?;
    let scale = if dst_is_rgb_or_gray { 255.0 } else { 100.0 };
    let mut max = 0.0f64;
    let mut sum = 0.0f64;
    for (m, t) in mine.iter().zip(&theirs) {
        let mut worst = 0.0f64;
        for (a, b) in m.iter().zip(t) {
            worst = worst.max((a - (b / scale).clamp(0.0, 1.0)).abs());
        }
        max = max.max(worst);
        sum += worst;
    }
    Ok((max, sum / mine.len() as f64))
}

/// §A's records.
#[must_use]
pub fn xrite_records(s: &Section) -> Vec<Record> {
    let mut out = Vec::new();
    for a in &s.arms {
        out.push(Record::graded(
            format!("passg/xrite-v4/{}/apparatus-harness-mab-matches-iccce-cmm", a.tag),
            Kind::SelfConsistency,
            Metric::DeltaE2000Max,
            APPARATUS,
            a.apparatus,
            "both sides computed in this run: passg::MabPipeline (n-linear) against \
             iccce_cmm::lut_ab::LutAbModel over the same points",
            format!(
                "{} pts, {} | the harness mAB reimplementation exists because the shipped engine \
                 has ONE interpolation scheme by design; every geometry-substitution row below is \
                 void if this fails",
                a.points, a.tag
            ),
        ).with_separation(Separation::none(
            "no rival reading: this compares two implementations of the SAME algorithm written \
             from the same clause, so a disagreement is a coding error and not a choice",
        )));

        out.push(Record::graded(
            format!("passg/xrite-v4/{}/pcs-lab-vs-lcms2", a.tag),
            Kind::CrossCheck,
            Metric::DeltaE2000Max,
            Tolerance::new(
                structural_tolerance_from(a.method_envelope),
                "the CLUT INTERPOLATION-METHOD envelope computed at run time from THIS tag's own \
                 7^4 table and the two published algorithms alone (iccce's n-linear against \
                 lcms2's Eval4Inputs hybrid), x1.25 for between-point maxima and for lcms2's \
                 fixed-point arithmetic. NO lcms2 output enters the derivation. ICC.1 is SILENT \
                 on interpolation (A16), so this admits the whole legitimate difference and \
                 therefore CANNOT claim agreement - it detects STRUCTURAL error only (wrong CLUT \
                 index order, wrong PCSLAB decode, dropped A curve, transposed ink), which is \
                 tens of dE. The agreement claim is the emulated-geometry row's",
            ),
            a.raw,
            format!(
                "iccce's geometry vs lcms2 via {} at {}; envelope {:.6} dE00 computed from the \
                 CLUT bytes",
                a.pcs,
                a.intent.name(),
                a.method_envelope
            ),
            format!("{} CMYK points, {} (X-Rite v4.2 mAB)", a.points, a.tag),
        ).with_separation(Separation::none(
            "the obvious rival - lcms2's own geometry - is not a rival for THIS row: it is the \
             very quantity this tolerance is sized to admit, and it is graded one row down. \
             Naming it here would double-count it",
        )));

        out.push(Record::graded(
            format!("passg/xrite-v4/{}/pcs-lab-emulated-geometry", a.tag),
            Kind::CrossCheck,
            Metric::DeltaE2000Max,
            EMULATED,
            a.emulated,
            "both sides computed in this run: passg::MabPipeline under lcms2's OWN CLUT geometry \
             against transicc -c0. The geometry is transcribed from cmsintrp.c at pin 21c582a",
            format!(
                "{} CMYK points, {} via {} | raw {:.6} -> emulated {:.6} dE00, a {:.1}x collapse: \
                 the disagreement IS the interpolation method and nothing else",
                a.points,
                a.tag,
                a.pcs,
                a.raw,
                a.emulated,
                a.raw / a.emulated
            ),
        ).with_separation(Separation::against_distance(
            "iccce's n-linear geometry retained instead of lcms2's - i.e. the row above, which is \
             what this row would observe if the geometry substitution were a no-op",
            a.raw,
            // ★ Supplied, not derived. The distance between the two candidate
            // answers is a property of THIS FIXTURE's CLUT - it is the method
            // envelope - and stays what it is whichever answer the code returns
            // today. Deriving it as |observed - alt| would collapse it to zero
            // on exactly the run where the substitution stopped working.
            a.method_envelope,
            SepUnits::SameAsMetric,
        )));

        out.push(Record::graded(
            format!("passg/xrite-v4/{}/pcs-lab-corners-interpolation-free", a.tag),
            Kind::CrossCheck,
            Metric::DeltaE2000Max,
            corner_tolerance(a.b_curves_are_identity),
            a.corners,
            "both sides computed in this run, at the 16 exact CLUT nodes where the method \
             difference is identically zero by construction",
            format!(
                "16 corners, {} | A-curve endpoints decode to ({:.6}, {:.6}) so a corner IS a \
                 node; method envelope AT the corners measured {:.3e} - the construction is \
                 checked, not assumed. B curves are the exact identity: {} - which is what \
                 selects this row's floor, because lcms2's u16 round trip through a NON-identity \
                 2-entry curve rounds twice and a lossless identity does not",
                a.tag,
                a.a_curve_endpoints.0,
                a.a_curve_endpoints.1,
                a.corner_envelope,
                a.b_curves_are_identity
            ),
        ).with_separation(Separation::none(
            "no rival GEOMETRY exists at a node - both schemes reduce to the stored sample - so \
             the only alternatives left are structural, and those are graded by the two rows above",
        )));
    }

    // --- the vendor's intent-tag aliasing -----------------------------------
    out.push(Record::graded(
        "passg/xrite-v4/a2b1-equals-a2b2-byte-identical",
        Kind::DerivedExpectation,
        Metric::AbsMaxComponent,
        ALIASED_TAGS,
        s.a2b1_vs_a2b2_bytes,
        "the file's own bytes, read with no parser in the way",
        format!(
            "X-Rite aliases MEDIA-RELATIVE and SATURATION onto one block; the ECI profile in the \
             same corpus aliases PERCEPTUAL and SATURATION instead (measured {:.1} over its \
             A2B0/A2B2 bytes in the same run). Aliasing is a VENDOR choice, not a format one, and \
             a saturation arm on this profile would have reproduced the media-relative arm bit \
             for bit",
            s.eci_a2b0_vs_a2b2_bytes
        ),
    ).with_separation(Separation::against_distance(
        "an engine that acquired the ECI pairing (A2B0 == A2B2) from testing on one vendor's \
         files and read A2B0 for saturation HERE: it would be reading a block that differs from \
         A2B2 across THIS profile's own bytes",
        s.a2b0_vs_a2b2_bytes,
        // Supplied, not derived: the distance between the two candidate BLOCKS
        // is a property of the file and does not move with what any engine does.
        s.a2b0_vs_a2b2_bytes,
        SepUnits::SameAsMetric,
    )));

    // --- lcms2's forced BPC, caught on the profile's own declared black ------
    out.push(Record::graded(
        "passg/xrite-v4/perceptual-black-equals-its-own-B-curve-floor",
        Kind::DerivedExpectation,
        Metric::AbsMaxComponent,
        Tolerance::new(
            1e-4,
            "transicc prints L* to 4 decimals, so the oracle's own value is known only to \
             +/-5e-5; 1e-4 is that printed precision taken as the bound. The EXPECTATION is not \
             an oracle value - it is the first entry of this tag's own B curve decoded through \
             the v4 PCSLAB L* scale (clause 10.13), i.e. arithmetic on the file",
        ),
        (s.lcms2_perceptual_black_l_v2pcs - s.derived_perceptual_black_l).abs(),
        "ICC.1:2022 clause 10.13 (mAB element order, B curves applied last) + Table 42 (PCSLAB \
         v4 encoding) applied to this profile's OWN A2B0 B-curve first entry. No implementation's \
         output is in the expectation",
        format!(
            "the profile declares a perceptual black of L* {:.6} in its A2B0 B curve. Into a v2 \
             PCS lcms2 prints L* {:.6} for full ink - it honours the declaration. Into a v4 PCS \
             it prints L* {:.6}, because _cmsLinkProfiles forces BPC for perceptual when the \
             DESTINATION is v4 (Pass 4b finding 2) and that scales the declared black to zero. \
             The v2 PCS is a FIXTURE CHOICE that keeps the gate shut, not a model that subtracts it",
            s.derived_perceptual_black_l,
            s.lcms2_perceptual_black_l_v2pcs,
            s.lcms2_perceptual_black_l_v4pcs
        ),
    ).with_separation(Separation::against_distance(
        "the v4 PCS destination, where lcms2's forced-BPC gate fires and the declared black is \
         scaled to zero - the value this row would have observed had the fixture not been chosen \
         to keep the gate shut",
        s.lcms2_perceptual_black_l_v4pcs,
        (s.lcms2_perceptual_black_l_v4pcs - s.derived_perceptual_black_l).abs(),
        SepUnits::SameAsMetric,
    )));

    // --- end to end ---------------------------------------------------------
    for (intent, bpc, max, mean, refusal) in &s.end_to_end {
        let id = format!(
            "passg/xrite-v4-to-srgb/{}/{}/device-vs-lcms2",
            intent.name(),
            match bpc {
                Bpc::Off => "no-bpc",
                Bpc::On => "bpc",
            }
        );
        if let Some(r) = refusal {
            out.push(Record::graded(
                id,
                Kind::CrossCheck,
                Metric::DeviceAbsMaxNormalised,
                Tolerance::new(
                    0.0,
                    "a REFUSAL is graded as a deliverable, at exactly zero, because the quantity \
                     is 'did the engine decline by name' and not a residual. CLAUDE.md rule 6: an \
                     engine that cannot estimate must say so rather than guess",
                ),
                0.0,
                "the shipped binary's own exit status and stderr",
                format!(
                    "iccce REFUSED BY NAME and did not guess: {r} | the v4 mAB source is outside \
                     iccce's named black-point estimation subset (A42). Coverage statement: this \
                     combination is NOT differentially tested, and that is disclosed rather than \
                     absorbed",
                ),
            ).with_separation(Separation::none(
                "the rival to a named refusal is a silent plausible answer, which has no value to \
                 compare against - that is the whole content of rule 6",
            )));
            continue;
        }
        let absolute = *intent == Intent::AbsoluteColorimetric;
        // The intent selects the tag, and the tag selects the envelope. A2B0 is
        // perceptual; media-relative and saturation share A2B1/A2B2, which are
        // one block of bytes in this profile.
        let env = s
            .propagated_envelope
            .iter()
            .find(|(t, _)| {
                *t == if *intent == Intent::Perceptual {
                    "A2B0"
                } else {
                    "A2B1"
                }
            })
            .map_or(f64::NAN, |(_, e)| *e);
        out.push(Record::graded(
            id,
            Kind::CrossCheck,
            Metric::DeviceAbsMaxNormalised,
            if absolute || *bpc == Bpc::On {
                REPORTED
            } else {
                propagated_gate(env)
            },
            *max,
            "both sides computed in this run: the shipped iccce binary and transicc -c0, each in \
             its own process",
            format!(
                "341 CMYK points -> Ghent sRGB, mean {:.6e}; propagated method envelope for the \
                 tag this intent selects is {:.6e} device{}",
                mean,
                env,
                if absolute {
                    ". ICC-ABSOLUTE: the destination is a v2 mntr profile whose wtpt is D65 while \
                     its colorants sum to D50, so lcms2 substitutes D50 and iccce uses the encoded \
                     value. The mechanism is graded in passg/authoring/*"
                } else {
                    ""
                }
            ),
        ).with_separation(if absolute || *bpc == Bpc::On {
            Separation::none(
                "the rival here is a POLICY (a substituted white point, or a different black point \
                 estimator), not a value this row could have observed; its magnitude is graded on \
                 its own row",
            )
        } else {
            Separation::none(
                "the obvious rival - lcms2's own geometry - is the very quantity this tolerance \
                 is sized to admit, and it is graded in the PCS by the emulated-geometry row. \
                 Naming it here would double-count it",
            )
        }));
    }
    out
}

// ===========================================================================
// §B — the population sweep
// ===========================================================================

/// One source→destination pair of the sweep.
pub struct SweepArm {
    pub label: &'static str,
    pub src: &'static str,
    pub dst: &'static str,
    pub src_scale: f64,
    pub out_channels: usize,
    pub dst_is_rgb_or_gray: bool,
    /// `(intent, bpc, max, mean, refusal)`.
    pub rows: Vec<(Intent, Bpc, f64, f64, Option<String>)>,
    /// ★ **The named rival, computed from the destination's own `B2A1` table:**
    /// the largest device difference between iccce's n-linear geometry and
    /// lcms2's *default* tetrahedral one. lcms2 does not take that path here —
    /// `_cmsReadOutputLUT` forces trilinear for a Lab PCS — so this is what the
    /// comparison **would** have seen had the forcing not been there, and it is
    /// what gives every §B row its discriminating power.
    pub counterfactual: f64,
    /// Whether lcms2's `wtpt` substitution can fire for this source: a v2
    /// `mntr` profile whose encoded `wtpt` is not the PCS white.
    pub wtpt_gate_can_fire: bool,
}

pub struct Sweep {
    pub arms: Vec<SweepArm>,
    /// `(label, |Σcolorants − PCS white|∞, |wtpt − PCS white|∞,
    /// |Σcolorants − wtpt|∞)` for the three display profiles whose authoring §B
    /// grades. All three numbers are computed from the file's own raw tag bytes
    /// and ICC.1:2022 Table 14's normative PCS white; **no third white point
    /// appears in the derivation**, which matters because D65 is recorded as
    /// the weakest constant in `iccce-color` and importing it here would put
    /// that weakness underneath a conformance claim.
    pub authoring: Vec<(&'static str, f64, f64, f64)>,
    /// `(profile label, ISO black L*, lcms2 black L*)` on three real print
    /// profiles — the mechanism behind every `--bpc` row.
    pub black_points: Vec<(&'static str, f64, f64)>,
    pub structure: String,
}

/// Build §B.
///
/// # Errors
/// As [`analyse_xrite`].
pub fn analyse_sweep(oracle: &Oracle, iccce: &Iccce) -> Result<Sweep, Unavailable> {
    let dir = corpus_dir();
    if !dir.is_dir() {
        return Err(Unavailable::Skip(format!(
            "ghent-v50 corpus not present at {}: set $ICCCE_PRIVATE_FIXTURES. It is licensed and \
             cannot be committed, so absence is the normal case and a green run here is evidence \
             that nothing ran",
            dir.display()
        )));
    }

    let rgb: Vec<Vec<f64>> = crate::pass4b::rgb_grid()
        .into_iter()
        .map(|t| t.to_vec())
        .collect();
    let gray: Vec<Vec<f64>> = crate::pass4b::gray_axis()
        .into_iter()
        .map(|v| vec![v])
        .collect();

    // `(label, source file, destination file, whether lcms2's v2-display wtpt
    // substitution CAN FIRE for this source, the device grid)`. The fourth
    // field is the one to be careful with: getting it backwards would turn a
    // REPORTED row into a graded one, or hide a real disagreement behind a
    // "reported" label.
    #[allow(clippy::type_complexity)]
    let specs: [(&str, &str, &str, bool, &[Vec<f64>]); 5] = [
        ("srgb-to-isocoated", file::SRGB, file::ISOCOATED, true, &rgb),
        (
            "adobergb-to-fogra39",
            file::ADOBE,
            file::FOGRA39,
            true,
            &rgb,
        ),
        (
            "ecirgb-v4-to-isocoated",
            file::ECIRGB_V4,
            file::ISOCOATED,
            false,
            &rgb,
        ),
        (
            "ecirgb-v2-to-isocoated",
            file::ECIRGB_V2,
            file::ISOCOATED,
            false,
            &rgb,
        ),
        (
            "gray-to-isocoated",
            file::GRAY,
            file::ISOCOATED,
            false,
            &gray,
        ),
    ];

    let mut arms = Vec::new();
    for (label, src, dst, gate, rows) in specs {
        let sp = dir.join(src);
        let dp = dir.join(dst);
        let counterfactual = counterfactual_for(&dp, tag::B2A1).map_err(Unavailable::Error)?;
        let mut measured = Vec::new();
        for intent in ALL_INTENTS {
            for bpc in [Bpc::Off, Bpc::On] {
                match measure_device(oracle, iccce, &sp, &dp, rows, 255.0, 4, false, intent, bpc) {
                    Ok((max, mean)) => measured.push((intent, bpc, max, mean, None)),
                    Err(r) => measured.push((intent, bpc, f64::NAN, f64::NAN, Some(r))),
                }
            }
        }
        arms.push(SweepArm {
            label: Box::leak(label.to_string().into_boxed_str()),
            src: Box::leak(src.to_string().into_boxed_str()),
            dst: Box::leak(dst.to_string().into_boxed_str()),
            src_scale: 255.0,
            out_channels: 4,
            dst_is_rgb_or_gray: false,
            rows: measured,
            counterfactual,
            wtpt_gate_can_fire: gate,
        });
    }

    // --- authoring: colorants against the PCS white -------------------------
    // ICC.1:2022 6.3.4.3 Table 14 fixes the PCS white at 0.9642/1.0000/0.8249.
    // The constant is the SPECIFICATION's, not the corpus's, so writing it here
    // copies nothing out of the private tree.
    const PCS_WHITE: [f64; 3] = [0.9642, 1.0, 0.8249];
    let mut authoring = Vec::new();
    for (label, f) in [
        ("srgb", file::SRGB),
        ("adobergb", file::ADOBE),
        ("ecirgb-v2", file::ECIRGB_V2),
    ] {
        let b = std::fs::read(dir.join(f)).map_err(|e| Unavailable::Error(e.to_string()))?;
        let (Some(r), Some(g), Some(bl), Some(w)) = (
            raw_xyz(&b, tag::R_XYZ),
            raw_xyz(&b, tag::G_XYZ),
            raw_xyz(&b, tag::B_XYZ),
            raw_xyz(&b, tag::WTPT),
        ) else {
            return Err(Unavailable::Error(format!(
                "{label}: missing a colorant or wtpt tag"
            )));
        };
        let sum = [r.x + g.x + bl.x, r.y + g.y + bl.y, r.z + g.z + bl.z];
        let wp = [w.x, w.y, w.z];
        let d =
            |a: [f64; 3], b: [f64; 3]| (0..3).map(|i| (a[i] - b[i]).abs()).fold(0.0f64, f64::max);
        authoring.push((label, d(sum, PCS_WHITE), d(wp, PCS_WHITE), d(sum, wp)));
    }

    // --- the black points behind every --bpc row ----------------------------
    let mut black_points = Vec::new();
    for (label, f) in [
        ("isocoated-v2.4", file::ISOCOATED),
        ("fogra39-v2.1", file::FOGRA39),
        ("xrite-v4.2", file::XRITE_V4),
    ] {
        match crate::pass5c::Fixture::open(&dir.join(f)) {
            Ok(fx) => {
                let (_dev, darkest, _initial, iso_l) = fx.iso_black();
                let branch = crate::pass5c::branch_for(fx.is_output_class, fx.is_ink_space);
                let det = crate::pass5c::detect_destination_black_point(
                    branch,
                    &|l| fx.bt_rel(l),
                    &|l| fx.bt_perc(l),
                    darkest,
                );
                black_points.push((label, iso_l, det.black.l));
            }
            Err(e) => return Err(Unavailable::Error(format!("{label}: {e}"))),
        }
    }

    let structure = format!(
        "{} pairs x 4 intents x 2 BPC settings; {} RGB points / {} gray points; \
         destinations are Lab-PCS output LUTs, so lcms2 forces trilinear and the \
         interpolation-method envelope is ZERO in this direction",
        arms.len(),
        rgb.len(),
        gray.len()
    );

    Ok(Sweep {
        arms,
        authoring,
        black_points,
        structure,
    })
}

/// The tetrahedral counterfactual for one `B2A*` tag: the largest device
/// difference between the two geometries over the Lab grid, **computed from the
/// table's own bytes with no oracle in it**.
fn counterfactual_for(path: &Path, sig: Signature) -> Result<f64, String> {
    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    let profile = Profile::parse(&bytes).map_err(|e| e.to_string())?;
    let e = profile
        .tags
        .iter()
        .find(|t| t.sig == sig)
        .ok_or("no B2A1 tag")?;
    let (dims, outs, data) = match profile.decode_tag(e) {
        Some(Ok(d)) => match d.data {
            TagData::Lut16(l) => (
                vec![usize::from(l.clut_points); usize::from(l.input_chan)],
                usize::from(l.output_chan),
                l.clut
                    .iter()
                    .map(|&s| f64::from(s) / 65535.0)
                    .collect::<Vec<_>>(),
            ),
            TagData::Lut8(l) => (
                vec![usize::from(l.clut_points); usize::from(l.input_chan)],
                usize::from(l.output_chan),
                l.clut
                    .iter()
                    .map(|&s| f64::from(s) / 255.0)
                    .collect::<Vec<_>>(),
            ),
            _ => return Err("B2A1 is neither mft1 nor mft2".into()),
        },
        _ => return Err("B2A1 is not decodable".into()),
    };
    let hc = HarnessClut::new(dims, outs, data);
    let mut a = vec![0.0; outs];
    let mut b = vec![0.0; outs];
    let mut worst = 0.0f64;
    for l in crate::pass4b::lab_grid() {
        let enc = [
            (l.l / 100.0).clamp(0.0, 1.0),
            ((l.a + 128.0) / 255.0).clamp(0.0, 1.0),
            ((l.b + 128.0) / 255.0).clamp(0.0, 1.0),
        ];
        hc.eval(&enc, Scheme::NLinear, &mut a);
        hc.eval(&enc, Scheme::Lcms2Default, &mut b);
        for (x, y) in a.iter().zip(&b) {
            worst = worst.max((x - y).abs());
        }
    }
    Ok(worst)
}

/// §B's records.
#[must_use]
pub fn sweep_records(s: &Sweep) -> Vec<Record> {
    let mut out = Vec::new();
    for arm in &s.arms {
        for (intent, bpc, max, mean, refusal) in &arm.rows {
            let bpc_tag = match bpc {
                Bpc::Off => "no-bpc",
                Bpc::On => "bpc",
            };
            let id = format!(
                "passg/{}/{}/{}/device-vs-lcms2",
                arm.label,
                intent.name(),
                bpc_tag
            );
            if let Some(r) = refusal {
                out.push(
                    Record::graded(
                        id,
                        Kind::CrossCheck,
                        Metric::DeviceAbsMaxNormalised,
                        Tolerance::new(
                            0.0,
                            "a REFUSAL graded as a deliverable at exactly zero: the quantity is \
                         'did the engine decline by name', not a residual",
                        ),
                        0.0,
                        "the shipped binary's own exit status and stderr",
                        format!("iccce REFUSED BY NAME and did not guess: {r}"),
                    )
                    .with_separation(Separation::none(
                        "the rival to a named refusal is a silent plausible answer, which has no \
                     value to compare against",
                    )),
                );
                continue;
            }
            // Two circumstances take the row out of grading, and each says so.
            let absolute_and_exposed =
                *intent == Intent::AbsoluteColorimetric && arm.wtpt_gate_can_fire;
            let tol = if absolute_and_exposed || *bpc == Bpc::On {
                REPORTED
            } else {
                SWEEP_DEVICE
            };
            let mut rec = Record::graded(
                id,
                Kind::CrossCheck,
                Metric::DeviceAbsMaxNormalised,
                tol,
                *max,
                "both sides computed in this run: the shipped iccce binary and transicc -c0, each \
                 in its own process, on profiles extracted from production PDF/X files",
                format!(
                    "mean {mean:.6e}{}",
                    if absolute_and_exposed {
                        ". ICC-ABSOLUTE out of a v2 mntr profile whose wtpt is D65 while its \
                         colorants sum to D50: lcms2 substitutes D50 (ICC.1:2001-04 A.3.1.1's own \
                         recommendation), iccce uses the encoded value. The mechanism is graded in \
                         passg/authoring/*, on the profile's own bytes"
                    } else if *bpc == Bpc::On {
                        ". --bpc: iccce estimates the destination black by ISO/CD 18619 4.2.5 and \
                         lcms2 by cmsDetectDestinationBlackPoint. ICC.1 has NO normative BPC text \
                         (A27/A42), so this is a difference between two choices, not a defect"
                    } else if *intent == Intent::AbsoluteColorimetric {
                        ". ICC-ABSOLUTE, GRADED: this source encodes wtpt = the PCS white, so \
                         lcms2's substitution is a no-op and the gate CANNOT fire. The fixture \
                         defeats the confound rather than a model subtracting it"
                    } else {
                        ""
                    }
                ),
            );
            if !absolute_and_exposed && *bpc == Bpc::Off {
                rec = rec.with_separation(Separation::against_distance(
                    "lcms2 had NOT forced trilinear for this Lab-PCS output LUT and had used its \
                     default tetrahedral geometry instead (_cmsReadOutputLUT, cmsintrp.c) - \
                     computed from the destination's own B2A1 table in this run",
                    arm.counterfactual,
                    // Supplied, not derived: the distance between the two
                    // candidate geometries is a property of the destination
                    // TABLE and does not move with whatever the engine returns.
                    arm.counterfactual,
                    SepUnits::SameAsMetric,
                ));
            } else {
                rec = rec.with_separation(Separation::none(
                    "the rival here is a POLICY (a substituted white point, or a different black \
                     point estimator), not a value this row could have observed. Its magnitude is \
                     graded on its own rows, which is where a separation belongs",
                ));
            }
            out.push(rec);
        }
    }

    // --- the authoring rows -------------------------------------------------
    //
    // ★ TWO DIFFERENT CLAIMS, and the first draft of this pass conflated them.
    // Where a profile's `wtpt` disagrees with its own colorant sum, the
    // question is WHICH of the two is out of step, and that is decidable
    // without any external white point: the colorant sum is compared to the
    // normative PCS white against a bound that is HALF the distance to the
    // profile's own rival candidate. Where they agree, that question is
    // undecidable and meaningless — and such a profile is the CONTROL that
    // shows the row is not simply passing everything, so it gets its own row
    // with its own (encoding-floor) derivation and an honest
    // NO-NAMED-ALTERNATIVE.
    for (label, colorant_err, wtpt_dist, sum_vs_wtpt) in &s.authoring {
        let disagrees = *sum_vs_wtpt > COLORANTS_SUM.value;
        if disagrees {
            out.push(
                Record::graded(
                    format!("passg/authoring/{label}/colorants-are-pcs-referred-not-wtpt-referred"),
                    Kind::DerivedExpectation,
                    Metric::AbsMaxComponent,
                    Tolerance::new(
                        sum_vs_wtpt / 2.0,
                        "HALF the distance between this profile's OWN two candidate whites - its \
                     colorant sum and its encoded wtpt - so the row asks a CLASSIFICATION \
                     question that cannot be tuned: is the colorant sum nearer the normative PCS \
                     white (ICC.1:2022 Table 14) or nearer the encoded wtpt? Deliberately NOT an \
                     encoding-precision bound: the published sRGB primaries do not sum to D50 to \
                     the s15Fixed16 lsb, and a bound that assumed they did would be a number \
                     fitted to one file. No third white point (D65 in particular) enters the \
                     derivation - it is the file's own two candidates and the spec's constant",
                    ),
                    *colorant_err,
                    "ICC.1:2022 6.3.4.3 Table 14 (PCS white 0.9642/1.0000/0.8249) against the \
                 profile's own rXYZ+gXYZ+bXYZ and wtpt tags, read from the raw bytes. The PCS \
                 white is the SPECIFICATION's constant; nothing is copied out of the licensed \
                 corpus",
                    format!(
                        "the colorant sum sits {colorant_err:.3e} from the PCS white and \
                     {sum_vs_wtpt:.4} from this profile's own encoded wtpt, which is itself \
                     {wtpt_dist:.4} from the PCS white - and there is no chad tag. So the PCS data \
                     IS adapted and the wtpt is NOT. ICC.1:2001-04 Annex A.3.1.1, VERBATIM: 'If \
                     chromatic adaptation is being applied to the PCS values, the adaptation \
                     should be applied to the mediaWhitePointTag values as well.' On that clause \
                     this is a DEFECT OF AUTHORSHIP, not a second reading of the standard - which \
                     settles the ICC-absolute divergence in lcms2's favour and is why those rows \
                     are reported rather than gated. ★ POPULATION FINDING: this is what a real \
                     producer embeds, not a one-off system profile"
                    ),
                )
                .with_separation(Separation::against_distance(
                    "the rival reading, under which the encoded wtpt is authoritative and the \
                 COLORANTS are the thing out of step - i.e. this profile's PCS data is referred to \
                 its encoded wtpt rather than to the PCS white",
                    *sum_vs_wtpt,
                    *sum_vs_wtpt,
                    SepUnits::SameAsMetric,
                )),
            );
        } else {
            out.push(Record::graded(
                format!("passg/authoring/{label}/wtpt-agrees-with-its-own-colorants"),
                Kind::DerivedExpectation,
                Metric::AbsMaxComponent,
                COLORANTS_SUM,
                *sum_vs_wtpt,
                "the profile's own wtpt and rXYZ+gXYZ+bXYZ tags, read from the raw bytes",
                format!(
                    "★ THE CONTROL. This profile's wtpt and colorant sum agree to \
                     {sum_vs_wtpt:.3e}, and both sit within {wtpt_dist:.3e} / {colorant_err:.3e} \
                     of the normative PCS white - so it is self-consistent, lcms2's v2-display \
                     wtpt substitution is a NO-OP on it, and its ICC-absolute rows can be GRADED \
                     where the mis-authored profiles' cannot. Its presence is what stops the rows \
                     above being read as 'every v2 display profile in the wild is broken'"
                ),
            ).with_separation(Separation::none(
                "there is no rival reading when the two candidate whites are the same number to \
                 the encoding floor: nothing is out of step, so there is nothing to attribute. \
                 This row is the negative control for the rows above and states that rather than \
                 manufacturing a separation it does not have",
            )));
        }
    }

    // --- the black-point mechanism ------------------------------------------
    for (label, iso_l, lcms2_l) in &s.black_points {
        out.push(Record::graded(
            format!("passg/bpc-mechanism/{label}/estimator-divergence"),
            Kind::CrossCheck,
            Metric::AbsMaxComponent,
            REPORTED,
            (iso_l - lcms2_l).abs(),
            "iccce's ISO/CD 18619 4.2.5 estimator against pass5c's reimplementation of lcms2's \
             cmsDetectDestinationBlackPoint, both run in process on this profile",
            format!(
                "ISO estimator L* {iso_l:.6} vs lcms2 L* {lcms2_l:.6}. This is the mechanism \
                 behind every --bpc row above. What the Ghent corpus adds to Pass 5b/5c is that \
                 the divergence is NOT an artefact of synthetic or OS-shipped fixtures: it \
                 reproduces on real vendor print profiles. ICC.1 has no normative BPC text \
                 (A27/A42), so neither estimator can be called wrong from the standard"
            ),
        ).with_separation(Separation::none(
            "both candidates are named and both are implemented; the row reports their distance \
             rather than choosing, because no clause chooses",
        )));
    }
    out
}

// ===========================================================================
// §C — eciRGB v2 against eciRGB v4
// ===========================================================================

/// The v2/v4 pair, and the two reasons it is a weaker instrument than it looks.
pub struct EciArm {
    /// max |Δ| device, both encodings through the same destination, **iccce on
    /// both sides**.
    pub max: f64,
    pub mean: f64,
    pub points: usize,
    /// max |Δ| device, the same comparison with **lcms2 on both sides** — so
    /// that "the two files agree" is a statement about the files and not about
    /// one engine's handling of them.
    pub oracle_max: f64,
    /// `|wtpt − PCS white|∞` for each encoding. Both are ~0, which is exactly
    /// why this pair **cannot** isolate lcms2's version gate.
    pub wtpt_distance: (f64, f64),
}

/// Build §C.
///
/// # Errors
/// As [`analyse_xrite`].
pub fn analyse_eci(oracle: &Oracle, iccce: &Iccce) -> Result<EciArm, Unavailable> {
    let dir = corpus_dir();
    if !dir.is_dir() {
        return Err(Unavailable::Skip(
            "ghent-v50 corpus not present: set $ICCCE_PRIVATE_FIXTURES".into(),
        ));
    }
    let v2 = dir.join(file::ECIRGB_V2);
    let v4 = dir.join(file::ECIRGB_V4);
    let dst = dir.join(file::ISOCOATED);
    let rgb: Vec<Vec<f64>> = crate::pass4b::rgb_grid()
        .into_iter()
        .map(|t| t.to_vec())
        .collect();

    let mine = |p: &Path| {
        iccce
            .transform_rows_shaped(p, &dst, Intent::RelativeColorimetric, &rgb, 4)
            .map_err(|e| Unavailable::Error(e.to_string()))
    };
    let a = mine(&v2)?;
    let b = mine(&v4)?;
    let mut max = 0.0f64;
    let mut sum = 0.0f64;
    for (x, y) in a.iter().zip(&b) {
        let d = x
            .iter()
            .zip(y)
            .map(|(p, q)| (p - q).abs())
            .fold(0.0f64, f64::max);
        max = max.max(d);
        sum += d;
    }

    let theirs = |p: &Path| {
        let req = Request {
            input: Space::profile(p),
            output: Space::profile(&dst),
            intent: Intent::RelativeColorimetric,
            precalc: Precalc::Exact,
            bpc: Bpc::Off,
            values: rgb.iter().flatten().map(|v| v * 255.0).collect(),
        };
        oracle
            .convert_batch_shaped(&req, 3, 4)
            .map_err(|e| Unavailable::Error(e.to_string()))
    };
    let ta = theirs(&v2)?;
    let tb = theirs(&v4)?;
    let mut omax = 0.0f64;
    for (x, y) in ta.iter().zip(&tb) {
        omax = omax.max(
            x.iter()
                .zip(y)
                .map(|(p, q)| (p - q).abs() / 100.0)
                .fold(0.0f64, f64::max),
        );
    }

    const PCS_WHITE: [f64; 3] = [0.9642, 1.0, 0.8249];
    let wd = |p: &Path| -> Result<f64, Unavailable> {
        let b = std::fs::read(p).map_err(|e| Unavailable::Error(e.to_string()))?;
        let w = raw_xyz(&b, tag::WTPT).ok_or_else(|| Unavailable::Error("no wtpt".into()))?;
        Ok([w.x, w.y, w.z]
            .iter()
            .zip(PCS_WHITE)
            .map(|(a, b)| (a - b).abs())
            .fold(0.0f64, f64::max))
    };

    Ok(EciArm {
        max,
        mean: sum / a.len() as f64,
        points: rgb.len(),
        oracle_max: omax,
        wtpt_distance: (wd(&v2)?, wd(&v4)?),
    })
}

/// **§C, the self-consistency gate.**
///
/// Same tolerance as [`SWEEP_DEVICE`] and for the same reason — the pipeline is
/// the same one, the destination is the same file, and the only thing that
/// changes is which of two files describes the source. Reusing the number
/// rather than inventing a second one is deliberate: a *different* tolerance
/// here would be a number chosen for this comparison, which is the thing
/// `CLAUDE.md` rule 5 forbids.
///
/// ★ **What this row is worth, stated on the row.** It is
/// [`Kind::SelfConsistency`]: *both sides are iccce*, on two files from one
/// vendor. It prices a representation change; it is **not** evidence that
/// either answer is right, and it is weaker than a cross-check because there is
/// only one lineage in it. A companion row runs the identical comparison with
/// **lcms2 on both sides**, so that a green line here says something about the
/// two files rather than about iccce's handling of them.
#[must_use]
pub fn eci_records(e: &EciArm) -> Vec<Record> {
    vec![
        Record::graded(
            "passg/ecirgb-v2-vs-v4/iccce-both-sides",
            Kind::SelfConsistency,
            Metric::DeviceAbsMaxNormalised,
            SWEEP_DEVICE,
            e.max,
            "both sides are iccce: the same engine, the same destination file, the same grid, \
             differing only in which of two vendor files describes the source",
            format!(
                "{} RGB points -> ISO Coated v2, media-relative; mean {:.6e}. \
                 ★ EVIDENCE CLASS: SELF-CONSISTENCY between two files from ONE vendor. Weaker \
                 than a cross-check (one lineage, not two) and far weaker than ground truth. \
                 ★★ And it is NOT the clean version isolator it looks like: the two files differ \
                 in TRC REPRESENTATION as well as in header version (700-entry tabulated curv vs \
                 para type 3), so a disagreement has two candidate causes and this pair cannot \
                 separate them",
                e.points, e.mean
            ),
        )
        .with_separation(Separation::none(
            "no rival CANDIDATE ANSWER exists for a self-consistency row: there is one engine and \
             one pair of files. The interesting alternatives are causal (version vs TRC \
             representation) and this fixture cannot separate them - which is stated in the \
             detail rather than dressed up as a separation",
        )),
        Record::graded(
            "passg/ecirgb-v2-vs-v4/lcms2-both-sides",
            Kind::OracleReproducibility,
            Metric::DeviceAbsMaxNormalised,
            SWEEP_DEVICE,
            e.oracle_max,
            "BOTH SIDES ARE LCMS2. Says nothing whatever about iccce; it establishes that the two \
             vendor files describe the same colour space to a second implementation as well",
            format!(
                "{} RGB points. Its purpose is to stop the row above being read as a property of \
                 iccce when it is a property of the two files",
                e.points
            ),
        )
        .with_separation(Separation::none(
            "both sides are ONE implementation on two files: there is no second candidate answer \
             for this row to have observed. Its value is comparative, not discriminating, and \
             saying so is more useful than inventing a rival",
        )),
        Record::graded(
            "passg/ecirgb-v2-vs-v4/pair-cannot-isolate-the-wtpt-version-gate",
            Kind::DerivedExpectation,
            Metric::AbsMaxComponent,
            Tolerance::new(
                2e-4,
                "the s15Fixed16 encoding floor, as COLORANTS_SUM: one lsb is 1.526e-5 and Table \
                 14 states the PCS white to 4 decimals. This row asserts that BOTH encodings put \
                 wtpt AT the PCS white, which is what makes lcms2's substitution a no-op for both",
            ),
            e.wtpt_distance.0.max(e.wtpt_distance.1),
            "each file's own wtpt tag against ICC.1:2022 Table 14's PCS white, read from raw bytes",
            format!(
                "v2.4 encoding: {:.3e} from the PCS white; v4.2 encoding: {:.3e}. Both are AT it, \
                 so lcms2's v2-display wtpt substitution is a NO-OP for either and the pair CANNOT \
                 be used to isolate the version leg of that gate. ★ This is a NEGATIVE result and \
                 it is recorded because the pair looks like the instrument for that job and is not. \
                 No pair in this corpus differs only in version while encoding a non-PCS white",
                e.wtpt_distance.0, e.wtpt_distance.1
            ),
        )
        .with_separation(Separation::none(
            "a negative result about what a fixture cannot do has no rival candidate answer; it \
             has a REASON, which is in the detail",
        )),
    ]
}

// ===========================================================================
// §D — the trap profiles
// ===========================================================================

/// The GWG discriminators. **The rare case where the correct answer is known
/// without measurement**: the swap is the profile's own declared content, so
/// "red in, green out" is checkable against the profile's tags rather than
/// against an oracle.
pub struct TrapArm {
    /// max |Δ| between iccce's PCS `XYZ` for device `(1,0,0)` and the `rXYZ`
    /// tag's own three numbers.
    pub red_matches_rxyz: f64,
    /// The same for `(0,1,0)` against `gXYZ`.
    pub green_matches_gxyz: f64,
    /// ★ The **separation**: ΔE2000 between the profile's `rXYZ` and its
    /// `gXYZ` as D50 CIELAB — how far apart the two candidate answers are, and
    /// therefore the whole power of these rows. A property of the file.
    ///
    /// It is carried for the human reader; the *machine-checked* separation is
    /// [`TrapArm::primary_separation_xyz`], because a ΔE cannot be compared to
    /// a tolerance stated in XYZ and an incommensurate separation is emitted
    /// rather than tested.
    pub primary_separation: f64,
    /// The same distance in the metric this row is actually graded in:
    /// `|rXYZ − gXYZ|∞`. Stated separately so the blindness test can run
    /// instead of being skipped for unit mismatch.
    pub primary_separation_xyz: f64,
    /// `rXYZ`'s chromaticity `y − x`. Positive means the tag labelled "red"
    /// holds a **green** primary, which is the swap the profile's `desc`
    /// declares. Colorimetry, not convention.
    pub rxyz_greenness: f64,
    /// The same for `gXYZ`: `x − y`, positive when the "green" tag holds red.
    pub gxyz_redness: f64,
    /// max |Δ| device between the two same-`desc` trap files through the same
    /// destination.
    pub duplicate_delta: f64,
    /// Whether the two files' colour-bearing tags are byte-identical.
    pub duplicate_tags_identical: bool,
    /// max |Δ| device, iccce vs lcms2, through the RGB trap into Ghent sRGB.
    pub cross_check: f64,
    /// The CMYK trap's `B2A0` vs `B2A2` raw-byte difference — a second vendor
    /// aliasing, and a second null that must be identified before collection.
    pub cmyk_b2a0_vs_b2a2: f64,
}

/// Build §D.
///
/// # Errors
/// As [`analyse_xrite`].
pub fn analyse_traps(oracle: &Oracle, iccce: &Iccce) -> Result<TrapArm, Unavailable> {
    let dir = corpus_dir();
    let p1 = dir.join(file::TRAP_RGB);
    let b1 = std::fs::read(&p1)
        .map_err(|e| Unavailable::Skip(format!("ghent-v50 corpus not present ({e})")))?;
    let prof = Profile::parse(&b1).map_err(|e| Unavailable::Error(e.to_string()))?;
    let model = MatrixTrc::from_profile(&prof)
        .map_err(|e| Unavailable::Error(format!("trap profile is not matrix/TRC: {e}")))?;

    let (Some(rx), Some(gx)) = (raw_xyz(&b1, tag::R_XYZ), raw_xyz(&b1, tag::G_XYZ)) else {
        return Err(Unavailable::Error("trap profile has no rXYZ/gXYZ".into()));
    };
    let got_r = model.device_to_pcs([1.0, 0.0, 0.0]);
    let got_g = model.device_to_pcs([0.0, 1.0, 0.0]);
    let dist = |a: Xyz, b: Xyz| {
        (a.x - b.x)
            .abs()
            .max((a.y - b.y).abs())
            .max((a.z - b.z).abs())
    };

    // The separation, in the metric a colour engineer can read: how far apart
    // the two candidate answers are. Computed from the tags themselves, so it
    // is a property of the FIXTURE and does not move with the engine's answer.
    let to_lab = |v: Xyz| Lab::from_xyz(v, iccce_color::D50);
    let primary_separation = delta_e_2000(to_lab(rx), to_lab(gx));

    let chroma = |v: Xyz| {
        let s = v.x + v.y + v.z;
        (v.x / s, v.y / s)
    };
    let (rxx, rxy) = chroma(rx);
    let (gxx, gxy) = chroma(gx);

    // --- the duplicate ------------------------------------------------------
    let p2 = dir.join(file::TRAP_RGB_2);
    let b2 = std::fs::read(&p2).map_err(|e| Unavailable::Error(e.to_string()))?;
    let identical = [tag::R_XYZ, tag::G_XYZ, tag::B_XYZ, tag::WTPT]
        .iter()
        .all(|s| raw_tag(&b1, *s) == raw_tag(&b2, *s));
    let srgb = dir.join(file::SRGB);
    let probes: Vec<Vec<f64>> = crate::pass4b::rgb_grid()
        .into_iter()
        .map(|t| t.to_vec())
        .collect();
    let one = iccce
        .transform_rows_shaped(&p1, &srgb, Intent::RelativeColorimetric, &probes, 3)
        .map_err(|e| Unavailable::Error(e.to_string()))?;
    let two = iccce
        .transform_rows_shaped(&p2, &srgb, Intent::RelativeColorimetric, &probes, 3)
        .map_err(|e| Unavailable::Error(e.to_string()))?;
    let duplicate_delta = one
        .iter()
        .zip(&two)
        .map(|(a, b)| {
            a.iter()
                .zip(b)
                .map(|(x, y)| (x - y).abs())
                .fold(0.0f64, f64::max)
        })
        .fold(0.0f64, f64::max);

    let (cross_check, _) = measure_device(
        oracle,
        iccce,
        &p1,
        &srgb,
        &probes,
        255.0,
        3,
        true,
        Intent::RelativeColorimetric,
        Bpc::Off,
    )
    .map_err(Unavailable::Error)?;

    let bc =
        std::fs::read(dir.join(file::TRAP_CMYK)).map_err(|e| Unavailable::Error(e.to_string()))?;
    let cmyk_alias = match (raw_tag(&bc, tag::B2A0), raw_tag(&bc, tag::B2A2)) {
        (Some(x), Some(y)) if x.len() == y.len() => x
            .iter()
            .zip(y)
            .map(|(p, q)| f64::from(*p as i16 - *q as i16).abs())
            .fold(0.0f64, f64::max),
        _ => f64::INFINITY,
    };

    Ok(TrapArm {
        red_matches_rxyz: dist(got_r, rx),
        green_matches_gxyz: dist(got_g, gx),
        primary_separation,
        primary_separation_xyz: dist(rx, gx),
        rxyz_greenness: rxy - rxx,
        gxyz_redness: gxx - gxy,
        duplicate_delta,
        duplicate_tags_identical: identical,
        cross_check,
        cmyk_b2a0_vs_b2a2: cmyk_alias,
    })
}

/// §D's records.
#[must_use]
pub fn trap_records(t: &TrapArm) -> Vec<Record> {
    let sep = || {
        Separation::against_distance(
            "an engine that IGNORES the declared source profile and reads the colorants in \
             conventional order (or passes device values through as if the source were the \
             destination): full red in gives the gXYZ answer instead of the rXYZ one. This is the \
             failure the GWG suite designed these files to expose, and the one its test page \
             renders as a visible red X",
            t.primary_separation_xyz,
            // ★ Supplied, not derived, and stated in the ROW'S OWN METRIC so the
            // blindness test can actually run: this is the distance between the
            // two CANDIDATE ANSWERS and is a property of the profile's two
            // colorant tags. Deriving it as |observed - alt| would collapse it
            // to zero on exactly the run where the engine took the wrong
            // candidate - the hazard `Separation::against`'s doc comment records.
            t.primary_separation_xyz,
            SepUnits::SameAsMetric,
        )
    };
    vec![
        Record::graded(
            "passg/trap-rgb/red-in-gives-the-rXYZ-tag-exactly",
            Kind::DerivedExpectation,
            Metric::AbsMaxComponent,
            SWAP_EXACT,
            t.red_matches_rxyz,
            "ICC.1:2022 6.3.4 / Annex F.3: for a matrix/TRC profile the PCS XYZ of device \
             (1,0,0) is rXYZ x TRC_r(1), and a curv's last entry maps to 1.0 - so the expected \
             answer IS the profile's own rXYZ tag. No oracle, no published value, no \
             implementation output in the expectation",
            format!(
                "★ THE RARE CASE WHERE THE CORRECT ANSWER IS KNOWN WITHOUT MEASUREMENT: the swap \
                 is the profile's declared content. Its rXYZ has chromaticity y-x = {:+.4} (a \
                 GREEN primary; colorimetry, not naming convention) and its gXYZ has x-y = \
                 {:+.4} (RED). The two candidate answers are {:.2} dE2000 apart",
                t.rxyz_greenness, t.gxyz_redness, t.primary_separation
            ),
        )
        .with_separation(sep()),
        Record::graded(
            "passg/trap-rgb/green-in-gives-the-gXYZ-tag-exactly",
            Kind::DerivedExpectation,
            Metric::AbsMaxComponent,
            SWAP_EXACT,
            t.green_matches_gxyz,
            "as the row above, for device (0,1,0) against the gXYZ tag",
            "the second half of the swap: an engine that got only one of these right would have a \
             channel-ordering bug rather than an intact transform",
        )
        .with_separation(sep()),
        Record::graded(
            "passg/trap-rgb/the-two-same-desc-files-agree",
            Kind::SelfConsistency,
            Metric::DeviceAbsMaxNormalised,
            DUPLICATE_EXACT,
            t.duplicate_delta,
            "both sides are iccce, on two corpus files carrying the SAME desc string and \
             DIFFERENT SHA-256s",
            format!(
                "their colour-bearing tags (rXYZ/gXYZ/bXYZ/wtpt) are byte-identical: {}. \
                 A corpus that contains two files with one description is a place for a silent \
                 substitution to hide, so the pair is checked rather than assumed",
                t.duplicate_tags_identical
            ),
        )
        .with_separation(Separation::none(
            "two files that are byte-identical in every colour-bearing tag have no rival answer \
             between them; the row exists to establish that they ARE, not to choose",
        )),
        Record::graded(
            "passg/trap-rgb/device-vs-lcms2",
            Kind::CrossCheck,
            Metric::DeviceAbsMaxNormalised,
            SWEEP_DEVICE,
            t.cross_check,
            "both sides computed in this run: the shipped iccce binary and transicc -c0",
            "the trap through Ghent's sRGB, media-relative. The derived rows above establish that \
             iccce honours the swap; this one establishes that lcms2 honours the SAME swap, so \
             the two engines would render the GWG test page identically",
        )
        .with_separation(Separation::none(
            "the swap rival is carried by the two DERIVED rows above, which state it in the PCS \
             where it is a property of the profile's own tags. Restating it here in device units \
             would require modelling the destination and would double-count one alternative \
             across three rows",
        )),
        Record::graded(
            "passg/trap-cmyk/b2a0-equals-b2a2-byte-identical",
            Kind::DerivedExpectation,
            Metric::AbsMaxComponent,
            ALIASED_TAGS,
            t.cmyk_b2a0_vs_b2a2,
            "the file's own bytes, read with no parser in the way",
            "a THIRD aliasing pattern in this corpus: the CMYK trap aliases perceptual and \
             saturation in the B2A direction as well as the A2B one. Identified before anything \
             was collected over it, per TOLERANCES.md 3.4.4's rule about nulls that are null by \
             construction",
        )
        .with_separation(Separation::none(
            "this row asserts an IDENTITY, and an identity has no rival VALUE: either the two \
             blocks are the same bytes or they are not. What a rival would be useful for - \
             'an engine assuming the other vendor's pairing' - is stated on the X-Rite aliasing \
             row, where the two blocks genuinely differ and the distance is therefore non-zero",
        )),
    ]
}

// ===========================================================================
// Running the pass
// ===========================================================================

/// Records emitted when a section could not run. **Every row still appears**,
/// labelled, because a suite that emits nothing when it cannot run is
/// indistinguishable in a log from one that was never wired up.
#[must_use]
pub fn unavailable_records(u: &Unavailable, section: &str) -> Vec<Record> {
    let mk = |id: &str| match u {
        Unavailable::Skip(_) => Record::skipped(
            format!("passg/{section}/{id}"),
            Kind::CrossCheck,
            Metric::DeviceAbsMaxNormalised,
            SWEEP_DEVICE,
            "not run",
            u.reason().to_string(),
        ),
        Unavailable::Error(_) => Record::errored(
            format!("passg/{section}/{id}"),
            Kind::CrossCheck,
            Metric::DeviceAbsMaxNormalised,
            SWEEP_DEVICE,
            "not run",
            u.reason().to_string(),
        ),
    };
    vec![mk("section-did-not-run")]
}

/// Everything Pass G produces, and the records.
pub struct Bundle {
    pub xrite: Option<Section>,
    pub sweep: Option<Sweep>,
    pub eci: Option<EciArm>,
    pub traps: Option<TrapArm>,
}

/// Run Pass G. The four sections are independent: a failure to build one does
/// not silence the others.
#[must_use]
pub fn run(oracle: &Oracle) -> (Bundle, Vec<Record>) {
    let mut records = Vec::new();
    let iccce = match Iccce::locate() {
        Ok(Some(i)) => i,
        Ok(None) => {
            let u = Unavailable::Skip(
                "iccce binary not found: run `cargo build --release -p iccce-cli`".into(),
            );
            for s in ["xrite-v4", "sweep", "ecirgb", "traps"] {
                records.extend(unavailable_records(&u, s));
            }
            return (
                Bundle {
                    xrite: None,
                    sweep: None,
                    eci: None,
                    traps: None,
                },
                records,
            );
        }
        Err(e) => {
            let u = Unavailable::Error(e.to_string());
            for s in ["xrite-v4", "sweep", "ecirgb", "traps"] {
                records.extend(unavailable_records(&u, s));
            }
            return (
                Bundle {
                    xrite: None,
                    sweep: None,
                    eci: None,
                    traps: None,
                },
                records,
            );
        }
    };

    let xrite = match analyse_xrite(oracle, &iccce) {
        Ok(s) => {
            records.extend(xrite_records(&s));
            Some(s)
        }
        Err(u) => {
            records.extend(unavailable_records(&u, "xrite-v4"));
            None
        }
    };
    let sweep = match analyse_sweep(oracle, &iccce) {
        Ok(s) => {
            records.extend(sweep_records(&s));
            Some(s)
        }
        Err(u) => {
            records.extend(unavailable_records(&u, "sweep"));
            None
        }
    };
    let eci = match analyse_eci(oracle, &iccce) {
        Ok(e) => {
            records.extend(eci_records(&e));
            Some(e)
        }
        Err(u) => {
            records.extend(unavailable_records(&u, "ecirgb"));
            None
        }
    };
    let traps = match analyse_traps(oracle, &iccce) {
        Ok(t) => {
            records.extend(trap_records(&t));
            Some(t)
        }
        Err(u) => {
            records.extend(unavailable_records(&u, "traps"));
            None
        }
    };

    (
        Bundle {
            xrite,
            sweep,
            eci,
            traps,
        },
        records,
    )
}

/// The one-line note the runner prints. Every number in it is **formatted from
/// what the run produced**, never typed (DL-034).
#[must_use]
pub fn note(b: &Bundle) -> String {
    let mut parts = Vec::new();
    if let Some(x) = &b.xrite {
        parts.push(x.structure.clone());
    }
    if let Some(s) = &b.sweep {
        parts.push(s.structure.clone());
    }
    if parts.is_empty() {
        parts.push("no section ran (corpus, oracle or shipped binary absent)".into());
    }
    parts.join(" | ")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The corpus is licensed and absent everywhere but the operator's machine.
    /// This test asserts the **contract**, not the corpus: an absent corpus must
    /// produce a SKIP with a reason, never a pass and never a panic.
    #[test]
    fn absent_corpus_skips_with_a_reason() {
        let u = Unavailable::Skip("corpus not present".into());
        let recs = unavailable_records(&u, "sweep");
        assert_eq!(recs.len(), 1);
        assert!(recs[0].id.starts_with("passg/sweep/"));
        assert!(matches!(recs[0].outcome, crate::Outcome::Skip { .. }));
        assert!(u.reason().contains("not present"));
    }

    /// A tolerance whose `why` is empty cannot exist, and this pass's tolerances
    /// must all say something. Cheap, and it catches a constant added later with
    /// a placeholder justification.
    #[test]
    fn every_tolerance_states_why() {
        for t in [
            EMULATED,
            corner_tolerance(true),
            corner_tolerance(false),
            propagated_gate(1.0),
            APPARATUS,
            SWEEP_DEVICE,
            REPORTED,
            COLORANTS_SUM,
            ALIASED_TAGS,
            SWAP_EXACT,
            DUPLICATE_EXACT,
        ] {
            assert!(
                t.why.len() > 40,
                "a tolerance's justification must be an argument, not a word: {}",
                t.why
            );
            assert!(
                !t.why.to_lowercase().contains("it passed"),
                "'it passed' is not a justification"
            );
        }
    }

    /// The structural tolerance is a **function of the measured envelope**, so
    /// it cannot go stale the way a typed constant does (DL-034). This pins the
    /// factor rather than any number.
    #[test]
    fn structural_tolerance_is_a_multiple_of_the_envelope() {
        assert!((structural_tolerance_from(1.0) - 1.25).abs() < 1e-12);
        assert!((structural_tolerance_from(0.5) - 0.625).abs() < 1e-12);
    }
}
