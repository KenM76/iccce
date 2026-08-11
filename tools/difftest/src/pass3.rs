//! # Pass 3 — the matrix/TRC differential, iccce against lcms2
//!
//! This module answers ROADMAP Pass 3's done-when:
//!
//! > *sRGB → AdobeRGB round-trips within a stated ΔE, and matches lcms2
//! > within a stated tolerance, with both numbers written down.*
//!
//! Read `tools/difftest/README.md` §13 for the prose record of what was run
//! and what it found. This file is the apparatus; the README is the finding.
//!
//! ## The shape of the experiment
//!
//! ```text
//!   grid (133 RGB triples in 0..1, deterministic — see `grid()`)
//!     │
//!     ├──▶ iccce transform --src sRGB --dst AdobeRGB      (subprocess)
//!     │        → A: destination device RGB, 0..1, 6 decimals
//!     │
//!     └──▶ transicc -isRGB -oAdobeRGB -t1 -c0 -n          (subprocess)
//!              → B: destination device RGB, 0..255, 4 decimals
//!
//!   compare  A  vs  B/255   in DEVICE space          → check 1
//!   carry both into D50 CIELAB, compare in ΔE2000    → check 2
//!   iccce sRGB→AdobeRGB→sRGB, compare to the grid    → check 3
//!   instrument check: the Lab ruler, iccce vs lcms2  → check 4
//! ```
//!
//! **Both sides cross a process boundary.** iccce's answer comes from the
//! shipped `iccce transform` binary, not from the `iccce-cmm` crate this
//! module links — see the doc comment on [`crate::Iccce`] for why that
//! asymmetry would matter.
//!
//! ## Why the intent is media-relative colorimetric and nothing else
//!
//! Two independent reasons, and the second is the one that took measuring:
//!
//! 1. **iccce implements only that intent in Pass 3.** `iccce transform`
//!    refuses any other by name rather than substituting.
//! 2. **Perceptual and saturation would be comparing a transform with black
//!    point compensation silently switched on.** `ARCHITECTURE.md` DL-013 /
//!    `tools/difftest/README.md` §12.4: lcms2's `_cmsLinkProfiles` sets
//!    `BPC = TRUE` on its own authority ("following Adobe's document") when
//!    the intent is perceptual or saturation **and** the profile is v4.
//!    Measured effect ≈3.15 `L*` at black — nothing like sub-perceptual. A
//!    tolerance set at those intents without knowing that is a tolerance set
//!    on the wrong quantity.
//!
//!    Note carefully that **this pair would have escaped that trap anyway**:
//!    both profiles are v2.1 (`0x02100000`), and the forced-BPC branch is
//!    gated on `cmsGetEncodedICCversion >= 0x4000000`. Escaping a trap by
//!    accident is not the same as avoiding it, so the intent is pinned at
//!    media-relative *by construction* and the v2-ness is recorded as a
//!    second, independent reason the comparison is clean rather than as the
//!    reason.
//!
//! ## Why `-c0`
//!
//! `-c0` is `cmsFLAGS_NOOPTIMIZE`: lcms2 evaluates the pipeline as read
//! instead of flattening it into a sampled grid. An oracle must be the
//! reference implementation's **most accurate** path, for the same reason
//! `fast_float` is never built (README §3). Any other `-c` would make a
//! disagreement ambiguous between "iccce is wrong" and "lcms2 approximated".
//!
//! ## The units trap, stated once so it is never re-derived
//!
//! | | input | output |
//! |---|---|---|
//! | `iccce transform` | one triple per line, floats **0..1** | 6 decimals, **0..1** |
//! | `transicc` (8-bit RGB) | one component per line, **0..255** | 4 decimals, **0..255** |
//!
//! Everything below works in normalised 0..1 and converts at the `transicc`
//! boundary only. A number quoted without its scale is wrong by a factor of
//! 255, which looks like catastrophic colour error rather than a units bug.

use std::path::{Path, PathBuf};

use iccce_cmm::MatrixTrc;
use iccce_color::{D50, Lab, delta_e_2000};
use iccce_profile::Profile;

use crate::{
    Bpc, DiffError, Iccce, Intent, Kind, Metric, Oracle, Precalc, Record, Request, Space, Tolerance,
};

// ===========================================================================
// The corpus — category (c), read locally, never committed
// ===========================================================================

/// The source profile: the Windows system sRGB profile.
///
/// **Category (c) per `LEGAL.md` §3** — read from the local system, never
/// committed, never a required input. Every check here **skips** when it is
/// absent, so the Linux runner reports 3 ("nothing ran"), not 0.
///
/// As read by `iccce inspect` on 2026-08-11: v2.1, `mntr`, `RGB `/`XYZ `,
/// `desc` = "sRGB IEC61966-2.1", colorants
/// r(0.4361, 0.2225, 0.0139) g(0.3851, 0.7169, 0.0971) b(0.1431, 0.0606,
/// 0.7141), and — this is the part that matters below — **`rTRC`/`gTRC`/
/// `bTRC` are 1024-entry sampled `curv` tables sharing one block of tag data
/// at offset 1084.**
pub const SRGB: &str = r"C:\Windows\System32\spool\drivers\color\sRGB Color Space Profile.icm";

/// The destination profile: Adobe RGB (1998), as shipped in the Windows
/// colour directory.
///
/// **Category (c) per `LEGAL.md` §3.** This is the *named* profile of the
/// ROADMAP's done-when, so no substitution-with-reason is needed; the search
/// for an alternative second RGB profile was made moot by finding it present.
///
/// As read by `iccce inspect` on 2026-08-11: v2.1, `mntr`, `RGB `/`XYZ `,
/// `cprt` = "Copyright 2000 Adobe Systems Incorporated", `desc` = "Adobe RGB
/// (1998)", colorants r(0.6097, 0.3111, 0.0195) g(0.2053, 0.6257, 0.0609)
/// b(0.1492, 0.0632, 0.7446), and **`rTRC`/`gTRC`/`bTRC` are single-value
/// `curv` gamma tags, γ = 2.19921875** (`u8Fixed8` `0x0233`, exact in binary).
///
/// The two profiles are therefore *deliberately unalike in the way that
/// matters*: the source's tone curve is a sampled table and the
/// destination's is an analytic gamma, so the comparison exercises both of
/// `iccce-cmm::curve`'s evaluation paths and both of its inversion paths in
/// one run. Had both been gammas, the whole table-interpolation half of the
/// crate would have gone untested while the report said "sRGB → Adobe RGB
/// verified".
pub const ADOBE_RGB: &str = r"C:\Windows\System32\spool\drivers\color\AdobeRGB1998.icc";

// ===========================================================================
// The tolerances — every one of them, with the reason it is that number
// ===========================================================================

/// **Check 1 — device-space agreement, iccce vs lcms2, sRGB → Adobe RGB.**
///
/// Metric: max |Δ| over every component of every grid point, **normalised
/// device units (0..1)**, after clamping lcms2's output into `[0,1]` (the
/// unclamped comparison is reported separately as a *finding*, see
/// [`Analysis::lcms2_out_of_range`]).
///
/// ## Where 5×10⁻⁴ comes from — three terms, none of them "it passed"
///
/// The dominant disagreement is **not** iccce's; it is a documented
/// approximation inside lcms2, and the bound is derived from lcms2's own
/// arithmetic:
///
/// **(a) lcms2 quantises tabulated tone curves to 16 bits, even in float.**
/// `cmsgamma.c`, `cmsEvalToneCurveFloat`:
///
/// ```c
/// // Check for 16 bits table. If so, this is a limited-precision tone curve
/// if (Curve->nSegments == 0) {
///     In  = (cmsUInt16Number) _cmsQuickSaturateWord(v * 65535.0);
///     Out = cmsEvalToneCurve16(Curve, In);
///     return (cmsFloat32Number) (Out / 65535.0);
/// }
/// ```
///
/// The source profile's TRCs are exactly that case — 1024-entry sampled
/// `curv` tables with no analytic segments. So lcms2 rounds the *input* to
/// 1/65535 and rounds the curve's *output* to 1/65535, twice per channel,
/// where iccce interpolates in `f64` throughout. Each rounding is ≤ ½ lsb =
/// **7.63×10⁻⁶**.
///
/// **(b) The input rounding is amplified by the source TRC's slope.** The
/// sRGB EOTF's derivative peaks at white: `d/dx[((x+0.055)/1.055)^2.4]` =
/// `2.4/1.055 · 1.0^1.4` ≈ **2.275**. So (a)'s input term reaches ≈1.74×10⁻⁵
/// in *linear* units and the output term adds ≈7.63×10⁻⁶ — call it
/// **2.5×10⁻⁵ in the source's linear space**, per channel.
///
/// **(c) The inverse destination TRC amplifies without bound near black.**
/// `d device/d linear` for γ = 2.19921875 is `(1/γ)·L^(1/γ − 1)`, which → ∞
/// as `L` → 0. **There is therefore no finite uniform device-space tolerance
/// that is valid over the whole cube**, and pretending otherwise would be the
/// dishonest part. 5×10⁻⁴ is the envelope evaluated over *this grid*: its
/// smallest non-zero source value is 1/16 (0.0625 device → ≈4.03×10⁻³
/// linear), where the amplification factor is
/// `(1/2.199)·(4.03e-3)^(-0.5453)` ≈ **11.6**, giving 2.5×10⁻⁵ × 11.6 ≈
/// 2.9×10⁻⁴. Rounded up to 5×10⁻⁴ (≈0.13 in 0..255 units) to leave room for
/// the matrix term and for `transicc`'s own 4-decimal print floor of
/// 1×10⁻⁴/255 ≈ 3.9×10⁻⁷.
///
/// **What this tolerance is NOT.** It is not perceptual. `TOLERANCES.md` §2's
/// 1.0 ΔE2000 anchor is irrelevant to it and must not be cited in its
/// support — the perceptual statement is [`DE_CROSSCHECK`]'s job. It is also
/// **grid-dependent by construction**: extend the grid nearer to black and
/// this number must be re-derived, not re-tuned. That is written into the
/// `why` string so it travels with the number.
pub const DEVICE_CROSSCHECK: Tolerance = Tolerance::new(
    5e-4,
    "envelope of lcms2's OWN 16-bit quantisation of tabulated tone curves \
     (cmsEvalToneCurveFloat: input and output each rounded to 1/65535) = 2.5e-5 in source-linear, \
     amplified by the destination inverse-gamma slope (1/g)L^(1/g-1) evaluated at THIS grid's \
     darkest non-zero step L=4.03e-3 -> x11.6 = 2.9e-4, rounded up to 5e-4; \
     GRID-DEPENDENT BY CONSTRUCTION (the amplification is unbounded as L->0, so a grid \
     extended nearer black must RE-DERIVE this, never re-tune it); \
     arithmetic-agreement, NOT perceptual",
);

/// **Check 2 — perceptual agreement, iccce vs lcms2, in ΔE2000.**
///
/// Metric: max CIEDE2000 (`kL=kC=kH=1`, D50 CIELAB) between the two
/// implementations' destination device outputs, both carried into Lab through
/// the same destination matrix/TRC model.
///
/// ## Where 0.02 ΔE2000 comes from
///
/// Unlike [`DEVICE_CROSSCHECK`], this one **does** have a finite bound over
/// the whole cube, because carrying the device value back through the
/// destination model undoes the inverse-gamma amplification that made the
/// device metric grid-dependent. What is left is the error in PCS XYZ,
/// propagated into Lab:
///
/// - Error in source-linear ≤ **2.5×10⁻⁵** per channel (see
///   [`DEVICE_CROSSCHECK`] (a)–(b); the source of the disagreement is
///   unchanged, only the space it is measured in).
/// - `XYZ = M_src · linear`, and `‖M_src‖∞` = max row sum = 1.0 (the Y row,
///   by construction for a D50-referenced media-relative profile). So the
///   error in PCS XYZ is ≤ **2.5×10⁻⁵** per channel.
/// - Lab's sensitivity to XYZ is greatest on `f`'s **linear** segment, where
///   `f'(t) = 7.787`: `dL*/dY ≤ 116·7.787 = 903.3`, so `ΔL* ≤ 0.023`; and
///   `da*/dX ≤ 500·7.787/0.9642 = 4038`, so `Δa* ≤ 0.097` from the X term
///   plus 0.097 from the Y term.
/// - Worst case, all terms aligned and all at their maxima:
///   `ΔE00 ≲ √(0.023² + 0.195² + 0.195²) ≈ 0.28`.
///
/// That 0.28 is a genuine analytic ceiling but a very pessimistic one: it
/// assumes ½-lsb rounding at maximum in every channel simultaneously, at the
/// most Lab-sensitive point in the space, with the errors adding rather than
/// partially cancelling. **0.02 is set instead, and it is a deliberately
/// tighter gate than the analysis alone requires**, on the reasoning that a
/// tolerance should be the tightest bound the mechanism can be expected to
/// meet on the corpus actually run — a residual that has quietly grown from
/// 10⁻³ to 0.27 would still pass a 0.28 gate and nothing would show it
/// (`TOLERANCES.md` §3.1's boxed warning). If a future grid legitimately
/// exceeds 0.02, the fix is a **new row in §4 with a new justification**,
/// never an edit in place — and the first question is still whether the code
/// is wrong.
///
/// **Relation to the perceptual anchor.** 0.02 ΔE2000 is **50× below**
/// `TOLERANCES.md` §2's 1.0 ΔE2000 perceptibility anchor. That anchor is
/// itself ⚠ PROVISIONAL (its citation is not yet verified from primary text),
/// so this comparison inherits the ⚠ — which costs nothing here, because
/// 50× of headroom survives any plausible correction to a threshold whose
/// measured range is 0.8–2.3.
pub const DE_CROSSCHECK: Tolerance = Tolerance::new(
    2e-2,
    "analytic ceiling from lcms2's 16-bit tabulated-curve quantisation (2.5e-5 in PCS XYZ) \
     through Lab's steepest sensitivity (dL*/dY<=903.3, da*/dX<=4038) is 0.28 dE00; \
     0.02 is set deliberately TIGHTER than that ceiling so a residual that grew by two \
     orders of magnitude could not still pass; 50x below the (PROVISIONAL) 1.0 dE00 \
     perceptibility anchor, which is inherited with its warning",
);

/// **Check 3 — the round trip, iccce alone: sRGB → Adobe RGB → sRGB.**
///
/// Metric: max CIEDE2000 (`kL=kC=kH=1`, D50 CIELAB) between the original grid
/// point and the round-tripped one, both carried into Lab through the
/// **source** model (both are sRGB device values).
///
/// ## Why this is the weakest kind of evidence, said before the number
///
/// `Kind::SelfConsistency`. A round trip through a *wrong* matrix round-trips
/// perfectly; a mis-transcribed colorant that survives inversion is invisible
/// here. What it does price is the one thing it can: **the cost of the
/// approximations in the loop**, which for this pair are (i) 1024-entry
/// sampled-table interpolation in the sRGB TRC and its Annex-F.1 inversion,
/// and (ii) the `[0,1]` clamp F.8–F.16 requires before the inverse TRC.
///
/// ## ★ Where 2.5×10⁻² ΔE2000 comes from — and the wrong number it replaced
///
/// **This tolerance was 1×10⁻² for the length of one run, and that run FAILED
/// at 1.8788×10⁻².** The record of why is kept here rather than tidied away,
/// because `TOLERANCES.md` §0 makes the *order* of the diagnosis the point.
///
/// The original justification read: *"sRGB and Adobe RGB (1998) share their
/// red (0.64, 0.33) and blue (0.15, 0.06) primaries and Adobe's green is more
/// saturated, so the sRGB triangle is strictly contained, no grid point is
/// clipped, and the only losses are interpolation ones."* Every clause of
/// that is true **of the two colour spaces** and the conclusion is false **of
/// the two files**:
///
/// > A matrix/TRC profile's media white is its colorant sum `M·(1,1,1)`, and
/// > the two files' colorants were authored and rounded to `s15Fixed16`
/// > independently — HP in 1998, Adobe in 2000. Measured from the tags:
/// > sRGB's encoded white is (0.964 279 17, 0.999 969 48, 0.825 088 50) and
/// > Adobe RGB's is (0.964 202 88, 1.0, 0.824 905 40), differing by
/// > (+7.629×10⁻⁵, −3.052×10⁻⁵, +1.831×10⁻⁴) — respectively 5, 2 and 12 units
/// > of `s15Fixed16`'s 1/65536 lsb, accumulated over three colorant tags.
/// >
/// > So the source's device white lands at (1.000 106, 0.999 873, 1.000 254)
/// > in the destination's **linear** space, two channels of it outside
/// > `[0,1]`, and the **normative** F.8–F.16 clamp discards the excess. The
/// > return trip cannot recover it. **25 of the 133 grid points are clipped
/// > somewhere**, all of them on the high-value faces of the cube.
///
/// The mechanism was not assumed. `pass3_report`'s §5 predicts the round-trip
/// ΔE at white **from the two matrices and the clamp alone** — no tone curve,
/// no lcms2, no measurement — and gets **1.878 244×10⁻²** against an observed
/// **1.878 818×10⁻²**: agreement to **0.03%**. That is a mechanism
/// established, not a coincidence absorbed.
///
/// ### The number
///
/// | term | value | where from |
/// |---|---|---|
/// | F.8–F.16 clamp of the encoded white-point mismatch | 1.8782×10⁻² | closed form from the two files' colorant tags |
/// | 1024-entry table interpolation, forward + inverse | ≈1×10⁻³ | `h²·max|f''|/8`, `h = 1/1023`, `max|f''| ≈ 3.0` for the sRGB EOTF, ×903.3 `dL*/dY`, two non-cancelling evaluations |
/// | **sum** | **≈1.98×10⁻²** | |
/// | **tolerance** | **2.5×10⁻²** | the sum with ~25% headroom, because the closed form is evaluated at the white corner only and the other 24 clipped points were not separately predicted |
///
/// **This is a corpus-specific number and it says so.** Point it at two
/// profiles whose encoded whites agree exactly and the dominant term vanishes;
/// point it at a pair further apart and it is too tight. A different profile
/// pair **re-derives** this from its own colorant tags. It does not inherit it.
///
/// **40× below the (⚠ provisional) 1.0 ΔE2000 perceptibility anchor.**
///
/// ### ★ What this tolerance cannot catch, stated because it is uncomfortable
///
/// It is an **upper** bound, and the dominant term is a *cost* rather than an
/// *error*. If iccce stopped clamping altogether, the round trip would get
/// **better**, the observed value would fall toward zero, and this check would
/// go green. **A gate that rewards removing a normative requirement is not a
/// gate.** That is why [`WHITE_CLAMP_PREDICTION`] exists: it pins the observed
/// cost *to* a closed-form prediction from both sides, so a collapse toward
/// zero is a failure rather than an improvement. Neither check is sufficient
/// alone and they are deliberately not merged.
pub const DE_ROUNDTRIP: Tolerance = Tolerance::new(
    2.5e-2,
    "dominated by the NORMATIVE F.8-F.16 clamp discarding the two files' encoded \
     white-point mismatch: sRGB's colorant sum (0.96427917,0.99996948,0.82508850) vs \
     AdobeRGB's (0.96420288,1.0,0.82490540) = 5/2/12 lsb of s15Fixed16, putting source \
     white at (1.000106,0.999873,1.000254) in destination linear space; closed-form \
     prediction from the matrices + clamp alone is 1.8782e-2 (observed 1.8788e-2, 0.03% \
     agreement), plus ~1e-3 for 1024-entry table interpolation; 2.5e-2 is that sum with \
     ~25% headroom. CORPUS-SPECIFIC: another profile pair RE-DERIVES this from its own \
     colorant tags, never inherits it. SUPERSEDES a 1e-2 whose justification wrongly \
     assumed nothing was clipped (see TOLERANCES.md §4). Upper bound only - removing the \
     clamp would IMPROVE this number, which is what WHITE_CLAMP_PREDICTION guards",
);

/// **Check 5 — the clamp is real, and it costs exactly what the files say it
/// should.**
///
/// Metric: |predicted − observed| round-trip ΔE2000 **at device white**,
/// where the prediction comes from [`predicted_white_clamp_de`] — the two
/// profiles' colorant matrices and the F.8–F.16 clamp, and nothing else.
///
/// ## Why this exists
///
/// [`DE_ROUNDTRIP`] is an upper bound on a quantity that is mostly a
/// *deliberate cost*. Remove iccce's range clamping and the round trip
/// becomes **more** accurate: the upper bound would pass while a normative
/// requirement had been deleted. This check refuses that trade. The observed
/// cost would collapse from 1.88×10⁻² toward zero while the prediction stayed
/// at 1.88×10⁻², and the difference would exceed this tolerance by **19×**
/// (`pass3_report` §5 prints that control explicitly).
///
/// ## ★ Exactly what it does and does not pin down — a correction on record
///
/// The first draft of this doc claimed it "makes the normative F.8–F.16
/// *ordering* falsifiable — delete the clamp from `pcs_to_device` and this
/// check fails". **That claim was wrong and is kept here as a correction
/// rather than deleted.** Reading `iccce-cmm::curve`, range clamping happens
/// at *three* independent sites, each with its own normative citation:
///
/// | site | clause | what it clamps |
/// |---|---|---|
/// | `MatrixTrc::pcs_to_device` | **F.8–F.16** | linear → `[0,1]` before TRC⁻¹ |
/// | `Trc::eval` | **10.18** (domain) | curve input → `[0,1]` |
/// | `Trc::eval_inverse` / `invert_table` | **F.1(b)** | `y` → the attainable range |
///
/// So deleting the F.8–F.16 clamp alone changes **nothing** for this pair:
/// `eval_inverse` re-clamps immediately, and on the return leg `eval` clamps
/// again. What this check therefore pins is the **net range policy plus the
/// matrices**, not the ordering:
///
/// - a colorant matrix that is not the one in the file (transposed columns, a
///   wrong `s15Fixed16` scale, a spurious adaptation) moves the prediction and
///   the observation *differently* → fails;
/// - range clamping removed from **all three** sites → observation collapses,
///   prediction does not → fails;
/// - the F.8–F.16 clamp removed **on its own** → **not detected**, because
///   iccce's other two clamps make it redundant.
///
/// **That last line is the honest scope statement.** For this profile pair the
/// F.8–F.16 *ordering* is unobservable at the shipped surface — it is defence
/// in depth, not a load-bearing distinction — and no test in this repository
/// currently distinguishes clamp-before from clamp-after. `matrix_trc.rs`'s
/// module doc is right that the order is normative and right about the symptom
/// if a CMM got it wrong; it is `iccce-cmm`'s own belt-and-braces clamping
/// that makes it undetectable *here*. Distinguishing the two orders needs a
/// TRC whose inverse is defined outside `[0,1]`, which iccce never permits.
/// **Recorded as owed, not as covered.**
///
/// ## Where 1×10⁻³ ΔE2000 comes from
///
/// The prediction is exact `f64` arithmetic on the two matrices; the
/// observation crosses two subprocess boundaries, each printing device values
/// at **6 decimals**. So each round-tripped component is known only to
/// ±5×10⁻⁷, twice. At white, `dY/d device = γ·1^(γ−1) = 2.199` and
/// `dL*/dY = 116/3 = 38.7`, so `dL*/d device ≈ 85` and the print quantisation
/// contributes ≲8.5×10⁻⁵ in `L*`, i.e. ≈1×10⁻⁴ in ΔE00 after `S_L ≈ 1.75` at
/// `L* ≈ 100`. **1×10⁻³ is ten times that floor** — room for the difference
/// between the prediction's exact `f64` matrix inverse and the shipped
/// binary's, without room for a missing clamp.
pub const WHITE_CLAMP_PREDICTION: Tolerance = Tolerance::new(
    1e-3,
    "iccce prints device values at 6 decimals on both legs => +-5e-7 per component, \
     x(dL*/d device ~= 85 at white) => ~1e-4 dE00 print floor; 1e-3 is 10x that. \
     Sized to admit f64 inverse-matrix noise and to REFUSE both a wrong colorant matrix \
     and range clamping removed from all three of its sites (F.8-F.16, 10.18, F.1(b)), \
     which would collapse the observation to ~0 while the prediction stayed at 1.88e-2 \
     -- 19x this bound. SCOPE: it does NOT detect the F.8-F.16 clamp being removed on its \
     own, because iccce's other two clamps make it redundant; the clamp-before vs \
     clamp-after ORDERING is unobservable at the shipped surface for this pair and is \
     recorded as OWED, not covered",
);

/// **Check 4 — the instrument check.** Does the Lab ruler used by check 2
/// agree with lcms2's Lab rendering of the same profile?
///
/// Metric: max CIEDE2000 between (a) Adobe RGB device → Lab through
/// `iccce-cmm`'s destination model, called **in-process** — this one is the
/// instrument, not the shipped surface, and is labelled as such — and (b) the
/// same device values through `transicc -iAdobeRGB -o*Lab4 -t1 -c0`.
///
/// ## Why this check exists at all
///
/// Check 2 measures a disagreement between two implementations *with a ruler
/// built partly out of one of them*. If iccce's destination forward model
/// were wrong, check 2's ΔE would be systematically mis-scaled and the error
/// would hide inside the metric instead of appearing as a number. This check
/// drags it out: it is the ruler, held against a second ruler.
///
/// It is **not** redundant with check 2. Check 2 compares two *answers* in one
/// space; this compares two *mappings* of the same answer into that space.
///
/// ## Where 0.05 ΔE2000 comes from
///
/// Dominated by `transicc`'s print precision, not by either implementation:
/// Lab is printed to 4 decimals, so `L*`, `a*` and `b*` are each known only
/// to ±5×10⁻⁵, giving a ΔE00 floor of ≈1×10⁻⁴ before any arithmetic. The rest
/// of the budget is lcms2's `cmsPipelineEvalFloat` through its own
/// matrix-shaper stage, where the destination TRC is an analytic γ (no
/// tabulated quantisation applies in this direction) but the D50 Lab
/// conversion goes through lcms2's `cmsXYZ2Lab` with its own white-point
/// constants — `cmsD50X/Y/Z`, which agree with iccce's `D50` to 4 decimals by
/// construction (`illuminant.rs` cites both) but not beyond. A 1×10⁻⁴
/// difference in the white point moves `L*` by ~0.01. **0.05 is ~500× the
/// print floor and ~5× the white-point term** — loose enough not to fail on
/// known, understood differences, tight enough that a genuinely wrong ruler
/// (a swapped colorant, a missing D50 adaptation, a v2/v4 Lab encoding
/// confusion at ≈0.39 `L*`) could not pass.
pub const DE_INSTRUMENT: Tolerance = Tolerance::new(
    5e-2,
    "transicc prints Lab to 4 decimals => dE00 floor ~1e-4; lcms2's and iccce's D50 agree \
     to 4 decimals by construction but not beyond, worth ~0.01 in L*; 0.05 is ~5x that \
     and would still catch a swapped colorant, a missing D50 adaptation, or the v2/v4 Lab \
     encoding error (~0.39 L*). This grades the MEASURING INSTRUMENT of the dE cross-check, \
     not the shipped iccce binary",
);

// ===========================================================================
// The grid
// ===========================================================================

/// The input grid: **133 deterministic RGB triples in `[0,1]`**, assembled so
/// that a reader can tell what is covered without running it.
///
/// | block | count | why it is there |
/// |---|---|---|
/// | cube corners | 8 | black, white, the three primaries, the three secondaries — where clamping, gamut edges and TRC endpoints all live |
/// | neutral axis | 17 | `k/16`, `k = 0..16`. Neutrals are where a wrong white point or a channel-asymmetric TRC shows up as a visible cast, and where nothing else in the cube can hide it |
/// | 4×4×4 lattice | 64 | `{0, ⅓, ⅔, 1}³` — systematic interior coverage that cannot accidentally miss a face or an edge |
/// | primaries / secondaries at half | 6 | mid-tone saturated colour: the case a lattice at ⅓/⅔ approximates but does not hit |
/// | pseudo-random interior | 48 | an LCG with a fixed seed. Systematic grids can sit exactly on table entries and never interpolate; these deliberately do not |
///
/// Duplicates across blocks are removed by exact bit pattern, which is why
/// the total is 133 rather than 143 — the corners recur in the lattice.
///
/// **Determinism is the point.** The LCG is
/// `x ← x·6364136223846793005 + 1442695040888963407` (Knuth's MMIX
/// constants), seeded `0x0000_0003_ICCCE_...` — see the code. No `rand`
/// dependency, no clock, no thread-id: two runs on two machines compare the
/// same 133 colours, or the comparison between their reports means nothing.
///
/// ## What this grid does NOT cover, stated because "verified" without scope
/// is the claim this whole role exists to prevent
///
/// - **Nothing below 1/16 except exact zero.** The destination inverse gamma
///   amplifies without bound as linear → 0 (see [`DEVICE_CROSSCHECK`]), so
///   the deep-shadow region is where the device-space tolerance is least
///   transferable. It is not covered, and the tolerance says so.
/// - **No out-of-gamut inputs**, because sRGB ⊂ Adobe RGB makes them
///   impossible in this direction. The clamp path of F.8–F.16 is therefore
///   exercised only by the tiny over-range excursions at white
///   ([`Analysis::lcms2_out_of_range`]) and not by a genuine gamut clip. The
///   reverse direction would exercise it and is **not run here**.
/// - **One profile pair, one intent, one direction, one platform.**
#[must_use]
pub fn grid() -> Vec<[f64; 3]> {
    let mut out: Vec<[f64; 3]> = Vec::new();
    let push = |t: [f64; 3], out: &mut Vec<[f64; 3]>| {
        let key = |v: f64| v.to_bits();
        if !out
            .iter()
            .any(|e| key(e[0]) == key(t[0]) && key(e[1]) == key(t[1]) && key(e[2]) == key(t[2]))
        {
            out.push(t);
        }
    };

    // 1. The eight cube corners, first and explicitly. If anything in this
    //    grid is going to be read by a human it is these.
    for r in [0.0, 1.0] {
        for g in [0.0, 1.0] {
            for b in [0.0, 1.0] {
                push([r, g, b], &mut out);
            }
        }
    }

    // 2. The neutral axis in 16 steps.
    for k in 0..=16 {
        let v = f64::from(k) / 16.0;
        push([v, v, v], &mut out);
    }

    // 3. A 4x4x4 lattice on {0, 1/3, 2/3, 1}.
    let axis = [0.0, 1.0 / 3.0, 2.0 / 3.0, 1.0];
    for &r in &axis {
        for &g in &axis {
            for &b in &axis {
                push([r, g, b], &mut out);
            }
        }
    }

    // 4. Primaries and secondaries at half intensity.
    for t in [
        [0.5, 0.0, 0.0],
        [0.0, 0.5, 0.0],
        [0.0, 0.0, 0.5],
        [0.0, 0.5, 0.5],
        [0.5, 0.0, 0.5],
        [0.5, 0.5, 0.0],
    ] {
        push(t, &mut out);
    }

    // 5. Pseudo-random interior points. Deterministic LCG (MMIX constants),
    //    fixed seed. Values are mapped into [0.02, 0.98] rather than [0,1] so
    //    that this block cannot accidentally re-cover the corners it exists
    //    to complement.
    let mut x: u64 = 0x1CCC_E000_0003_0001;
    let mut next = || -> f64 {
        x = x
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        // Top 53 bits -> [0,1). f64 has 53 bits of mantissa, so this is the
        // widest lossless mapping available.
        #[allow(clippy::cast_precision_loss)] // exactly 53 bits: lossless
        let u = (x >> 11) as f64 / ((1u64 << 53) as f64);
        0.02 + u * 0.96
    };
    for _ in 0..48 {
        let t = [next(), next(), next()];
        push(t, &mut out);
    }

    out
}

// ===========================================================================
// The measuring instrument
// ===========================================================================

/// Device RGB → D50 CIELAB, through a profile's matrix/TRC model.
///
/// **This is the instrument, and it is made of the code under test.** Check 4
/// ([`DE_INSTRUMENT`]) exists to bound its error against lcms2's rendering of
/// the same profile. Read that before quoting any ΔE from here.
///
/// One consequence to keep in view: `MatrixTrc::device_to_pcs` evaluates the
/// TRC, and `Trc::eval` clamps its input to `[0,1]` (clause 10.18's normative
/// domain). So a device value of 1.000048 — which lcms2 does produce here,
/// see [`Analysis::lcms2_out_of_range`] — maps to exactly the same colour as
/// 1.0, and the ΔE metric reads that pair as **zero difference**.
///
/// That is **correct, not a blind spot**: a device code outside `[0,1]` does
/// not denote a colour in that device space, so there is no colour difference
/// to measure. But it does mean the ΔE metric is *structurally unable* to see
/// the range-policy disagreement, which is why the device-space metric
/// reports it separately and why this paragraph exists.
fn to_lab(model: &MatrixTrc, rgb: [f64; 3]) -> Lab {
    Lab::from_xyz(model.device_to_pcs(rgb), D50)
}

/// ΔE2000 between two device triples in the same device space.
fn de(model: &MatrixTrc, a: [f64; 3], b: [f64; 3]) -> f64 {
    delta_e_2000(to_lab(model, a), to_lab(model, b))
}

/// ★ **Closed-form prediction of the round-trip ΔE2000 at device white**,
/// from the two profiles' colorant matrices and the F.8–F.16 clamp alone.
///
/// ```text
///   W_src   = M_src · (1,1,1)              the source's ENCODED media white
///   linear  = M_dst⁻¹ · W_src              where that white sits in the
///                                          destination's linear space
///   clamped = clamp(linear, 0, 1)          Annex F.8–F.16, NORMATIVE
///   W_back  = M_dst · clamped              what survives the return trip
///   ΔE      = ΔE2000( Lab(W_src), Lab(W_back) )   D50, kL=kC=kH=1
/// ```
///
/// **No tone curve appears**, and that is not an omission: every TRC in this
/// pair evaluates to exactly 1 at 1 (a `curv` table's last entry is `0xFFFF`;
/// any gamma has `1^γ = 1`), and every inverse to exactly 1 at 1. So at the
/// white corner the tone curves are the identity and the whole round-trip
/// cost is matrix-and-clamp. **No lcms2 appears either** — this is a
/// prediction about iccce, made from the files.
///
/// If this quantity is ≈0, the two files' encoded whites agree and the
/// round-trip's dominant term is absent — in which case [`DE_ROUNDTRIP`]'s
/// derivation does not apply to that pair and must be redone.
///
/// # Panics
/// If the destination colorant matrix is singular, which
/// `MatrixTrc::from_profile` already refuses to build a model from — so
/// reaching the panic means the invariant broke upstream, which is worth a
/// loud stop rather than a plausible number.
#[must_use]
pub fn predicted_white_clamp_de(src: &MatrixTrc, dst: &MatrixTrc) -> f64 {
    let w_src = src.matrix.apply([1.0, 1.0, 1.0]);
    let inv = dst
        .matrix
        .inverse()
        .expect("MatrixTrc::from_profile refuses a singular colorant matrix");
    let linear = inv.apply(w_src);
    let clamped = [
        linear[0].clamp(0.0, 1.0),
        linear[1].clamp(0.0, 1.0),
        linear[2].clamp(0.0, 1.0),
    ];
    let w_back = dst.matrix.apply(clamped);
    let lab = |v: [f64; 3]| {
        Lab::from_xyz(
            iccce_color::Xyz {
                x: v[0],
                y: v[1],
                z: v[2],
            },
            D50,
        )
    };
    delta_e_2000(lab(w_src), lab(w_back))
}

/// Max and mean of a slice, in one pass. Returned together because a max
/// without a mean hides how typical the worst case is, and a mean without a
/// max hides the worst case entirely.
fn max_mean(v: &[f64]) -> (f64, f64) {
    if v.is_empty() {
        return (f64::NAN, f64::NAN);
    }
    let max = v.iter().copied().fold(0.0_f64, f64::max);
    #[allow(clippy::cast_precision_loss)] // grid sizes are ~10^2
    let mean = v.iter().sum::<f64>() / v.len() as f64;
    (max, mean)
}

// ===========================================================================
// The analysis
// ===========================================================================

/// Everything one Pass 3 run measured. Kept as raw per-point data alongside
/// the reductions, so `pass3_report` can print the worst offenders and a
/// future question can be asked without re-running.
#[derive(Debug)]
pub struct Analysis {
    pub src_path: PathBuf,
    pub dst_path: PathBuf,
    /// The input grid, source device RGB in 0..1.
    pub grid: Vec<[f64; 3]>,
    /// iccce's destination device output, 0..1, from `iccce transform`.
    pub iccce_out: Vec<[f64; 3]>,
    /// lcms2's destination device output **as printed**, 0..255.
    pub lcms2_out_255: Vec<[f64; 3]>,
    /// lcms2's output normalised to 0..1, *not* clamped.
    pub lcms2_out: Vec<[f64; 3]>,
    /// iccce's round trip back into source device space, 0..1.
    pub roundtrip: Vec<[f64; 3]>,
    /// lcms2's Lab rendering of iccce's destination outputs (instrument check).
    pub lcms2_lab_of_iccce_out: Vec<Lab>,

    // --- reductions -------------------------------------------------------
    /// Per-point max |Δ| in device units 0..1, lcms2 clamped into [0,1].
    pub device_dev_clamped: Vec<f64>,
    /// Per-point max |Δ| in device units 0..1, lcms2 **as printed**.
    pub device_dev_raw: Vec<f64>,
    /// Per-point ΔE2000, iccce vs lcms2, through iccce's destination model.
    pub de_crosscheck: Vec<f64>,
    /// Per-point ΔE2000 of the round trip, through the source model.
    pub de_roundtrip: Vec<f64>,
    /// Per-point max |Δ| of the round trip in source device units 0..1.
    pub device_roundtrip: Vec<f64>,
    /// Per-point ΔE2000 between the two Lab rulers (instrument check).
    pub de_instrument: Vec<f64>,

    /// Every lcms2 output component that fell outside `[0,1]`, as
    /// `(grid index, channel, value)`. **A finding, not a failure** — see
    /// README §13.4.
    pub lcms2_out_of_range: Vec<(usize, usize, f64)>,

    /// Closed-form prediction of the round-trip ΔE2000 at device white, from
    /// the two colorant matrices and the F.8–F.16 clamp
    /// ([`predicted_white_clamp_de`]).
    pub white_clamp_predicted: f64,
    /// The same quantity as measured through two invocations of the shipped
    /// binary.
    pub white_clamp_observed: f64,
    /// How many grid points are clipped somewhere in the destination's linear
    /// space. **Reported because the original `DE_ROUNDTRIP` justification
    /// asserted this was zero**, and a claim that has been wrong once should
    /// be a number from then on.
    pub clipped_points: usize,

    /// Provenance, printed on every record.
    pub oracle_banner: String,
    pub iccce_exe: PathBuf,
    pub iccce_is_debug: bool,
}

impl Analysis {
    /// Load a profile and build its matrix/TRC model — the instrument's
    /// destination and source models.
    fn model(path: &Path) -> Result<MatrixTrc, String> {
        let bytes = std::fs::read(path).map_err(|e| format!("cannot read {}: {e}", path.display()))?;
        let profile =
            Profile::parse(&bytes).map_err(|e| format!("{} refused: {e}", path.display()))?;
        MatrixTrc::from_profile(&profile)
            .map_err(|e| format!("{}: no matrix/TRC model: {e}", path.display()))
    }
}

/// Why a Pass 3 run could not happen. Distinguishes **skip** (a category (c)
/// profile or an unbuilt binary is absent, which is not this suite's fault)
/// from **error** (something that was supposed to work did not).
#[derive(Debug)]
pub enum Unavailable {
    /// Nothing to report against: a profile or a binary is missing.
    Skip(String),
    /// Something ran and misbehaved.
    Error(String),
}

impl std::fmt::Display for Unavailable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unavailable::Skip(s) | Unavailable::Error(s) => f.write_str(s),
        }
    }
}

impl From<DiffError> for Unavailable {
    fn from(e: DiffError) -> Self {
        Unavailable::Error(e.to_string())
    }
}

/// Run the whole Pass 3 experiment and return everything it measured.
///
/// Six subprocess invocations, in this order, and the order is deliberate —
/// the cheapest failure comes first so a missing binary is reported in
/// milliseconds rather than after a grid has been pushed through lcms2:
///
/// 1. `iccce transform` src → dst (the answer under test)
/// 2. `transicc` src → dst (the oracle's answer)
/// 3. `iccce transform` dst → src on (1)'s output (the round trip)
/// 4. `transicc` dst → `*Lab4` on (1)'s output (the second ruler)
///
/// # Errors
/// [`Unavailable::Skip`] when a category (c) profile or the `iccce` binary is
/// absent; [`Unavailable::Error`] when a subprocess fails or its output does
/// not parse.
pub fn analyse(oracle: &Oracle, src_path: &Path, dst_path: &Path) -> Result<Analysis, Unavailable> {
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
                "iccce binary not found: run `cargo build --release -p iccce-cli`, \
                 or set $ICCCE_BIN"
                    .to_string(),
            ));
        }
        Ok(Some(i)) => i,
    };

    let src_model = Analysis::model(src_path).map_err(Unavailable::Error)?;
    let dst_model = Analysis::model(dst_path).map_err(Unavailable::Error)?;

    let grid = grid();

    // (1) iccce, source -> destination. The answer under test.
    let iccce_out = iccce.transform_grid(src_path, dst_path, &grid)?;

    // (2) lcms2, source -> destination. transicc wants 0..255, one component
    //     per line; the multiply happens HERE and nowhere else.
    let lcms2_req = Request {
        input: Space::profile(src_path),
        output: Space::profile(dst_path),
        intent: Intent::RelativeColorimetric,
        precalc: Precalc::Exact,
        // Bpc::Off means "we did not ask for it", not "it did not happen" —
        // but at media-relative colorimetric lcms2's forced-BPC branch is
        // unreachable at any profile version, and both profiles are v2.1
        // anyway. See the module doc.
        bpc: Bpc::Off,
        values: grid.iter().flat_map(|t| t.iter().map(|v| v * 255.0)).collect(),
    };
    let lcms2_rows = oracle.convert_batch(&lcms2_req, 3)?;
    let lcms2_out_255: Vec<[f64; 3]> = lcms2_rows.iter().map(|r| [r[0], r[1], r[2]]).collect();
    let lcms2_out: Vec<[f64; 3]> = lcms2_out_255
        .iter()
        .map(|r| [r[0] / 255.0, r[1] / 255.0, r[2] / 255.0])
        .collect();

    // (3) iccce, destination -> source, on iccce's own output. The round trip
    //     is iccce ALONE — lcms2 is not in this loop, and the Kind says so.
    let roundtrip = iccce.transform_grid(dst_path, src_path, &iccce_out)?;

    // (4) The second ruler: lcms2's Lab rendering of iccce's dst output.
    let lab_req = Request {
        input: Space::profile(dst_path),
        output: Space::lab_v4(),
        intent: Intent::RelativeColorimetric,
        precalc: Precalc::Exact,
        bpc: Bpc::Off,
        values: iccce_out
            .iter()
            .flat_map(|t| t.iter().map(|v| v * 255.0))
            .collect(),
    };
    let lab_rows = oracle.convert_batch(&lab_req, 3)?;
    let lcms2_lab_of_iccce_out: Vec<Lab> = lab_rows
        .iter()
        .map(|r| Lab {
            l: r[0],
            a: r[1],
            b: r[2],
        })
        .collect();

    // --- reductions -------------------------------------------------------
    let n = grid.len();
    let mut device_dev_clamped = Vec::with_capacity(n);
    let mut device_dev_raw = Vec::with_capacity(n);
    let mut de_crosscheck = Vec::with_capacity(n);
    let mut de_roundtrip = Vec::with_capacity(n);
    let mut device_roundtrip = Vec::with_capacity(n);
    let mut de_instrument = Vec::with_capacity(n);
    let mut lcms2_out_of_range = Vec::new();

    for i in 0..n {
        let a = iccce_out[i];
        let b = lcms2_out[i];
        let mut d_raw = 0.0_f64;
        let mut d_clamped = 0.0_f64;
        for c in 0..3 {
            d_raw = d_raw.max((a[c] - b[c]).abs());
            d_clamped = d_clamped.max((a[c] - b[c].clamp(0.0, 1.0)).abs());
            if !(0.0..=1.0).contains(&b[c]) {
                lcms2_out_of_range.push((i, c, b[c]));
            }
        }
        device_dev_raw.push(d_raw);
        device_dev_clamped.push(d_clamped);

        de_crosscheck.push(de(&dst_model, a, b));

        de_roundtrip.push(de(&src_model, grid[i], roundtrip[i]));
        device_roundtrip.push(
            (0..3)
                .map(|c| (grid[i][c] - roundtrip[i][c]).abs())
                .fold(0.0_f64, f64::max),
        );

        de_instrument.push(delta_e_2000(
            to_lab(&dst_model, a),
            lcms2_lab_of_iccce_out[i],
        ));
    }

    // The white-corner prediction, and the count of clipped points that the
    // superseded DE_ROUNDTRIP justification claimed was zero.
    let white_clamp_predicted = predicted_white_clamp_de(&src_model, &dst_model);
    let white_idx = grid
        .iter()
        .position(|t| *t == [1.0, 1.0, 1.0])
        .expect("grid() always contains device white — see its unit tests");
    let white_clamp_observed = de_roundtrip[white_idx];
    let dst_inv = dst_model
        .matrix
        .inverse()
        .expect("MatrixTrc::from_profile refuses a singular colorant matrix");
    let clipped_points = grid
        .iter()
        .filter(|x| {
            let lin_src = [
                src_model.trc[0].eval(x[0]),
                src_model.trc[1].eval(x[1]),
                src_model.trc[2].eval(x[2]),
            ];
            let l = dst_inv.apply(src_model.matrix.apply(lin_src));
            l.iter().any(|&v| !(0.0..=1.0).contains(&v))
        })
        .count();

    Ok(Analysis {
        white_clamp_predicted,
        white_clamp_observed,
        clipped_points,
        src_path: src_path.to_path_buf(),
        dst_path: dst_path.to_path_buf(),
        grid,
        iccce_out,
        lcms2_out_255,
        lcms2_out,
        roundtrip,
        lcms2_lab_of_iccce_out,
        device_dev_clamped,
        device_dev_raw,
        de_crosscheck,
        de_roundtrip,
        device_roundtrip,
        de_instrument,
        lcms2_out_of_range,
        oracle_banner: oracle.banner().unwrap_or_default(),
        iccce_exe: iccce.path().to_path_buf(),
        iccce_is_debug: iccce.is_debug_build(),
    })
}

// ===========================================================================
// Turning the analysis into graded records
// ===========================================================================

/// The seven records a Pass 3 run produces: five graded comparisons and two
/// reported-only reductions (the means), which carry a tolerance of
/// **infinity** and exist so the number is on the record rather than only in
/// a paragraph.
///
/// A mean with an infinite tolerance is not a green light. It is `PASS`
/// because there is nothing for it to fail — and the `why` string says so, so
/// nobody can quote it as evidence of anything.
#[must_use]
pub fn records(a: &Analysis) -> Vec<Record> {
    let pair = format!(
        "src={} dst={} intent=media-relative-colorimetric precalc=exact(-c0,NOOPTIMIZE) \
         bpc=not-requested grid={} points",
        a.src_path.display(),
        a.dst_path.display(),
        a.grid.len()
    );
    let provenance = format!(
        "iccce={} ({}) | oracle={}",
        a.iccce_exe.display(),
        if a.iccce_is_debug {
            "DEBUG BUILD - not the shipped artefact"
        } else {
            "release"
        },
        a.oracle_banner
    );

    let (dev_max, dev_mean) = max_mean(&a.device_dev_clamped);
    let (dev_raw_max, _) = max_mean(&a.device_dev_raw);
    let (de_max, de_mean) = max_mean(&a.de_crosscheck);
    let (rt_max, rt_mean) = max_mean(&a.de_roundtrip);
    let (rt_dev_max, _) = max_mean(&a.device_roundtrip);
    let (inst_max, _) = max_mean(&a.de_instrument);

    let both_ran = "BOTH SIDES COMPUTED IN THIS RUN — no recorded expectation is being \
                    reproduced. iccce's numbers from the shipped `iccce transform` binary, \
                    lcms2's from the pinned transicc. Cross-check, NOT ground truth.";

    vec![
        Record::graded(
            "pass3/srgb-to-adobergb/device-vs-lcms2",
            Kind::CrossCheck,
            Metric::DeviceAbsMaxNormalised,
            DEVICE_CROSSCHECK,
            dev_max,
            both_ran,
            format!(
                "{pair} | {provenance} | lcms2 output clamped into [0,1] before comparison; \
                 UNCLAMPED max would be {dev_raw_max:.6e} and {} components fell outside \
                 [0,1] (see README §13.4 — a FINDING about range policy, graded separately \
                 from arithmetic)",
                a.lcms2_out_of_range.len()
            ),
        ),
        Record::graded(
            "pass3/srgb-to-adobergb/device-mean",
            Kind::CrossCheck,
            Metric::DeviceAbsMeanNormalised,
            Tolerance::new(
                f64::INFINITY,
                "REPORTED, NOT GRADED. A mean over a grid hides the outlier a colour engine \
                 gets wrong; it is recorded so the distribution is on file next to the max, \
                 and it must never be quoted as if it were the max",
            ),
            dev_mean,
            both_ran,
            format!("{pair} | {provenance}"),
        ),
        Record::graded(
            "pass3/srgb-to-adobergb/de2000-vs-lcms2",
            Kind::CrossCheck,
            Metric::DeltaE2000Max,
            DE_CROSSCHECK,
            de_max,
            both_ran,
            format!(
                "{pair} | {provenance} | both device outputs carried into D50 CIELAB through \
                 iccce's destination matrix/TRC model; the metric is iccce_color::delta_e_2000, \
                 validated against all 34 Sharma/Wu/Dalal (2005) pairs at 1e-4 (NC-001). \
                 A VALIDATED RULER DOES NOT MAKE THIS GROUND TRUTH. \
                 Note the ruler clamps device values into [0,1] (clause 10.18's normative \
                 domain), so lcms2's over-range outputs at white read as zero difference here \
                 BY CONSTRUCTION — that disagreement lives in the device-space record"
            ),
        ),
        Record::graded(
            "pass3/srgb-to-adobergb/de2000-mean",
            Kind::CrossCheck,
            Metric::DeltaE2000Mean,
            Tolerance::new(
                f64::INFINITY,
                "REPORTED, NOT GRADED — see the device-mean record. Recorded so the \
                 distribution is on file next to the max",
            ),
            de_mean,
            both_ran,
            format!("{pair} | {provenance}"),
        ),
        Record::graded(
            "pass3/srgb-to-adobergb-to-srgb/roundtrip-de2000",
            Kind::SelfConsistency,
            Metric::DeltaE2000Max,
            DE_ROUNDTRIP,
            rt_max,
            "BOTH SIDES ARE ICCCE. This prices the approximations in the loop (1024-entry \
             sampled-table interpolation and its Annex F.1 inversion, plus the F.8-F.16 \
             clamp). It is WORTHLESS as correctness evidence: a round trip through a wrong \
             matrix round-trips perfectly.",
            format!(
                "{pair} | {provenance} | sRGB -> AdobeRGB -> sRGB, two invocations of the \
                 shipped binary; compared in D50 CIELAB through iccce's SOURCE model. \
                 mean={rt_mean:.6e} dE00; max device deviation {rt_dev_max:.6e} (0..1). \
                 {} of {} grid points are CLIPPED in the destination's linear space \
                 (the superseded 1e-2 justification asserted this was zero); the maximum \
                 is at device white and is the F.8-F.16 clamp discarding the two files' \
                 encoded white-point mismatch",
                a.clipped_points,
                a.grid.len()
            ),
        ),
        Record::graded(
            "pass3/roundtrip/white-clamp-cost-matches-prediction",
            Kind::SelfConsistency,
            Metric::DeltaE2000Max,
            WHITE_CLAMP_PREDICTION,
            (a.white_clamp_predicted - a.white_clamp_observed).abs(),
            "BOTH SIDES ARE ICCCE, but they are independent of each other: the PREDICTION \
             is closed-form f64 arithmetic on the two profiles' colorant matrices plus the \
             clamp, computed in-process; the OBSERVATION crosses two subprocess boundaries \
             through the shipped binary's full TRC-and-matrix pipeline. Neither is a \
             published value, so this is self-consistency — but it is the kind that can \
             FAIL, which the round-trip upper bound alone cannot.",
            format!(
                "{pair} | {provenance} | at device white (1,1,1): predicted \
                 {:.6e} dE00 from M_src, M_dst and the clamp alone (no tone curve \
                 enters: every TRC here is exactly 1 at 1); observed {:.6e} through two \
                 invocations of `iccce transform`. Pins the COLORANT MATRICES and the NET \
                 RANGE POLICY; does NOT pin the F.8-F.16 clamp ORDERING, which is redundant \
                 with 10.18 and F.1(b) in iccce and is recorded as owed",
                a.white_clamp_predicted, a.white_clamp_observed
            ),
        ),
        Record::graded(
            "pass3/instrument/adobergb-device-to-lab-ruler",
            Kind::CrossCheck,
            Metric::DeltaE2000Max,
            DE_INSTRUMENT,
            inst_max,
            "BOTH RULERS COMPUTED IN THIS RUN. iccce's side is an IN-PROCESS library call \
             (iccce-cmm), deliberately — this record grades the MEASURING INSTRUMENT of the \
             dE cross-check, not the shipped binary. It is the one place in this module where \
             iccce is not invoked as a subprocess, and that is the reason.",
            format!(
                "{pair} | {provenance} | Adobe RGB device -> D50 CIELAB, iccce-cmm's model \
                 vs transicc -i<dst> -o*Lab4 -t1 -c0, over iccce's {} destination outputs. \
                 Bounds the systematic mis-scaling that a wrong destination model would \
                 introduce into the dE2000 cross-check above",
                a.iccce_out.len()
            ),
        ),
    ]
}

/// Records for a run that could not happen, so the report has the same seven
/// lines whether or not the machine could run them.
///
/// **This is not cosmetic.** A suite that emits nothing when it cannot run
/// looks identical, in a log, to a suite that was never wired up. Seven `SKIP`
/// lines with reasons look like neither.
#[must_use]
pub fn unavailable_records(u: &Unavailable) -> Vec<Record> {
    let reason = u.to_string();
    let specs: [(&str, Kind, Metric, Tolerance); 7] = [
        (
            "pass3/srgb-to-adobergb/device-vs-lcms2",
            Kind::CrossCheck,
            Metric::DeviceAbsMaxNormalised,
            DEVICE_CROSSCHECK,
        ),
        (
            "pass3/srgb-to-adobergb/device-mean",
            Kind::CrossCheck,
            Metric::DeviceAbsMeanNormalised,
            Tolerance::new(f64::INFINITY, "REPORTED, NOT GRADED"),
        ),
        (
            "pass3/srgb-to-adobergb/de2000-vs-lcms2",
            Kind::CrossCheck,
            Metric::DeltaE2000Max,
            DE_CROSSCHECK,
        ),
        (
            "pass3/srgb-to-adobergb/de2000-mean",
            Kind::CrossCheck,
            Metric::DeltaE2000Mean,
            Tolerance::new(f64::INFINITY, "REPORTED, NOT GRADED"),
        ),
        (
            "pass3/srgb-to-adobergb-to-srgb/roundtrip-de2000",
            Kind::SelfConsistency,
            Metric::DeltaE2000Max,
            DE_ROUNDTRIP,
        ),
        (
            "pass3/roundtrip/white-clamp-cost-matches-prediction",
            Kind::SelfConsistency,
            Metric::DeltaE2000Max,
            WHITE_CLAMP_PREDICTION,
        ),
        (
            "pass3/instrument/adobergb-device-to-lab-ruler",
            Kind::CrossCheck,
            Metric::DeltaE2000Max,
            DE_INSTRUMENT,
        ),
    ];
    specs
        .into_iter()
        .map(|(id, kind, metric, tol)| match u {
            Unavailable::Skip(_) => Record::skipped(
                id,
                kind,
                metric,
                tol,
                "not run on this machine",
                reason.clone(),
            ),
            Unavailable::Error(_) => Record::errored(
                id,
                kind,
                metric,
                tol,
                "not run on this machine",
                reason.clone(),
            ),
        })
        .collect()
}

/// Convenience: run the standard sRGB → Adobe RGB (1998) experiment and
/// return its six records, whatever happened.
#[must_use]
pub fn run(oracle: &Oracle) -> (Option<Analysis>, Vec<Record>) {
    match analyse(oracle, Path::new(SRGB), Path::new(ADOBE_RGB)) {
        Ok(a) => {
            let r = records(&a);
            (Some(a), r)
        }
        Err(u) => (None, unavailable_records(&u)),
    }
}

// ===========================================================================
// Tests — of the apparatus, not of any colour
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// The grid is deterministic and the documented size. If this fails, two
    /// reports from two machines are no longer comparable and every recorded
    /// number silently changes scope.
    #[test]
    fn grid_is_deterministic_and_documented_size() {
        let a = grid();
        let b = grid();
        assert_eq!(a.len(), 133, "grid size is quoted in the docs and in README §13");
        assert_eq!(a, b, "grid must not depend on clock, hash seed or thread");
    }

    /// Every grid point is inside the unit cube. An input outside it would be
    /// clamped by both implementations and would silently test the clamp
    /// instead of the transform.
    #[test]
    fn grid_is_inside_the_unit_cube() {
        for t in grid() {
            for c in t {
                assert!((0.0..=1.0).contains(&c), "grid point out of range: {t:?}");
            }
        }
    }

    /// The eight corners are present, and they are the first eight — a human
    /// reading the head of a per-point dump should see them.
    #[test]
    fn grid_starts_with_the_eight_corners() {
        let g = grid();
        for (i, t) in g.iter().take(8).enumerate() {
            assert!(
                t.iter().all(|&c| c == 0.0 || c == 1.0),
                "position {i} is not a cube corner: {t:?}"
            );
        }
        assert_eq!(g[0], [0.0, 0.0, 0.0]);
        assert_eq!(g[7], [1.0, 1.0, 1.0]);
    }

    /// The neutral axis is present in full. Neutrals are where a wrong white
    /// point becomes a visible cast, so losing them would be a real loss of
    /// coverage that no other block replaces.
    #[test]
    fn grid_contains_the_whole_neutral_axis() {
        let g = grid();
        for k in 0..=16 {
            let v = f64::from(k) / 16.0;
            assert!(
                g.iter().any(|t| t[0] == v && t[1] == v && t[2] == v),
                "neutral step {k}/16 missing"
            );
        }
    }

    /// No duplicates: a repeated point would weight the mean without
    /// improving coverage.
    #[test]
    fn grid_has_no_duplicates() {
        let g = grid();
        for i in 0..g.len() {
            for j in (i + 1)..g.len() {
                assert!(g[i] != g[j], "duplicate at {i} and {j}: {:?}", g[i]);
            }
        }
    }

    /// max_mean reports the max, not the mean, and vice versa — the one
    /// transposition that would make every record in this module a lie.
    #[test]
    fn max_mean_does_not_transpose() {
        let (max, mean) = max_mean(&[1.0, 2.0, 3.0]);
        assert!((max - 3.0).abs() < 1e-15);
        assert!((mean - 2.0).abs() < 1e-15);
    }
}
