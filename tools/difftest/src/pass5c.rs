//! # Pass 5c — lcms2's black-point estimator REIMPLEMENTED, and Pass 5b's
//! claim 3 settled
//!
//! Pass 5b (`src/pass5b.rs`, README §17, `TOLERANCES.md` §3.5.7) compared
//! ISO/CD 18619 4.2.5's destination-black estimate against lcms2's. It could
//! not *read* lcms2's black point — `transicc` has no flag that prints one — so
//! it **recovered** it from lcms2's own BPC output through
//! `A2B1(B2A1(·))`, and that recovery has an error. The error bar came out at
//! **95 % of the effect it was bounding** (row Q1, ratio 0,948). Pass 5b
//! therefore recorded three of its four pre-registered claims as verdicts and
//! the fourth — *claim 3, "the divergence IS the chroma"* — as **NOT
//! ESTABLISHED**, because the observed `L*` term (0,725) sat inside the error
//! bar (0,814).
//!
//! §17.6 item 1 named the remedy and called it "the single highest-value item
//! left in Pass 5's family":
//!
//! > A harness reimplementation of `cmsDetectDestinationBlackPoint` —
//! > constant-chroma ramp, its own `BlackPointAsDarkerColorant` — modelled from
//! > `cmssamp.c` at the pin, exactly as §15 modelled `cmsReverseToneCurveEx`.
//! > It would remove §17.3.1's error bar entirely and turn claim 3 from
//! > *unattributed* into a finding either way.
//!
//! **This module is that reimplementation.** Its kind is `impl_crosscheck` and
//! its provenance is **source-read at the pin**, exactly as Pass 4b §C's
//! `cmsReverseToneCurveEx` model was: no lcms2 binary is executed to produce
//! the black point, so the two arms can be compared without a recovery step in
//! between.
//!
//! ## ★★ What the source read found, and it contradicts Pass 5b's own table
//!
//! Pass 5b's §17.2 table said lcms2 "holds the ramp's chroma constant" at
//! `clamp(±50, InitialLab.a/.b)` and "returns `Lab.a = InitialLab.a`" — both
//! true as written. What that table did **not** trace is where `InitialLab`
//! comes from for the fixture actually under test, and that turns out to
//! decide the whole question.
//!
//! `cmsDetectDestinationBlackPoint` (`src/cmssamp.c` L397+) computes, for
//! `INTENT_RELATIVE_COLORIMETRIC`:
//!
//! ```text
//! cmsDetectBlackPoint(&IniXYZ, hProfile, Intent, dwFlags);   // L466
//! cmsXYZ2Lab(NULL, &InitialLab, &IniXYZ);                    // L471
//! ```
//!
//! and `cmsDetectBlackPoint` (L217+) branches **before** it ever reaches
//! `BlackPointAsDarkerColorant`:
//!
//! ```text
//! // If output profile, discount ink-limiting and that's all
//! if (Intent == INTENT_RELATIVE_COLORIMETRIC &&
//!     (cmsGetDeviceClass(hProfile) == cmsSigOutputClass) &&
//!     (isInkColorspace(cmsGetColorSpace(hProfile))))
//!     return BlackPointUsingPerceptualBlack(BlackPoint, hProfile);   // L370-374
//! ```
//!
//! `USWebCoatedSWOP.icc` is `prtr` (output class) and `CMYK` (an ink
//! colorspace) and Pass 5b drove it at media-relative, so **that branch is the
//! one that fires** — `BlackPointAsDarkerColorant` is never called. And
//! `BlackPointUsingPerceptualBlack` (L146+) ends:
//!
//! ```text
//! // Clip Lab to reasonable limits
//! if (LabOut.L > 50) LabOut.L = 50;
//! LabOut.a = LabOut.b = 0;                                   // L174
//! ```
//!
//! **`InitialLab` is forced neutral.** Therefore lcms2's ramp chroma is `0`,
//! identical to ISO 4.2.3's neutralised ramp, and lcms2's *returned* chroma —
//! `Lab.a = InitialLab.a` at L590–591 — is **`0` as well**.
//!
//! **lcms2's detected destination black for this fixture is NEUTRAL.** The
//! `a* 0,3472 / b* 0,3001` Pass 5b recorded is not lcms2's black point's
//! chroma; it is chroma the `A2B1 ∘ B2A1` recovery *introduced*. That is
//! exactly the failure mode the error bar existed to warn about, and it is why
//! a marginal apparatus was reported as marginal rather than quoted as green.
//!
//! ## The remaining algorithmic divergences, for this fixture and intent
//!
//! With both `InitialLab`s neutral the two ramps are pointwise identical, so
//! what is left is small, enumerable, and **testable**:
//!
//! | | ISO/CD 18619 4.2.5 (`iccce_cmm::bpc`) | lcms2 2.19.1 `cmssamp.c` |
//! |---|---|---|
//! | `InitialLab` (rel-col) | darkest **vertex** (4.2.2.2) → neutralise+clip (4.2.3) | `BlackPointUsingPerceptualBlack` = `A2B1(B2A0(Lab 0,0,0))`, `L*` clipped to 50, chroma **forced to 0** |
//! | monotonic pass | `for i in (0..255).rev()` — **includes index 0** | `for (l = 254; l > 0; --l)` — **index 0 is never touched** (L505) |
//! | straight-midrange return | **`InitialLab`** — 4.2.5.4 final paragraph, *"shall be the same as InitialLab"* (**corrected `fd34a44`**; iccce returned `outRamp[first]` here until 2026-08-12 and that had no textual support in any branch of 4.2.5) | **`InitialLab`** (L536), i.e. **its own**, from a *different* round trip |
//! | minimum shadow points | `xs.len() < 3` → give up | caller checks `n < 3`, but `RootOfLeastSquaresFitQuadraticCurve` **returns 0 for `n < 4`** (L317) |
//! | near-linear fallback | `b == 0.0` → 0 | `fabs(b) < 1.0E-10` → 0 |
//! | singular normal equations | rejected on the determinant of the Gaussian solve | `_cmsMAT3inverse` rejects on `fabs(det) < 1.0E-4` (`lcms2_internal.h` L142) |
//!
//! ★★ **Row 3 is now the whole of the divergence on `swop`, and it is not the
//! divergence it was.** Since `fd34a44` **both sides return a quantity each
//! document calls `InitialLab`** — so the entire disagreement is that the two
//! documents *mean different things by that name*: ISO's is 4.2.2.2's darkest
//! **device vertex** neutralised (`L* 11,772 365` here), lcms2's is the
//! **perceptual black round trip** (`L* 16,571 474`). Row 1 of this table was
//! always the real difference; row 3 used to hide it, because the
//! non-conformant `outRamp[first]` happened to land 0,082 `L*` from lcms2's
//! answer. **Conformance moved iccce 59× further from the oracle.**
//!
//! The first three are the substantive ones. The monotonic-pass difference is
//! a genuine off-by-one in lcms2 with a real consequence: `outRamp[0]` is
//! `MinL`, and `MinL` normalises `yRamp`, which selects the shadow window that
//! the quadratic is fitted over. If `outRamp[0] > outRamp[1]` the two
//! implementations fit **different point sets**. Whether it bites on this
//! fixture is measured, not assumed.
//!
//! ## What this module deliberately does NOT do
//!
//! It does not reproduce lcms2's *pipeline*. lcms2 builds its round trip with
//! `CreateRoundtripXForm` (L40) — `cmsCreateExtendedTransform` over
//! `[Lab4, profile, profile, Lab4]` with `NOOPTIMIZE|NOCACHE` — and evaluates
//! it through lcms2's 16-bit machinery, whereas the `bt` handed to both arms
//! here is iccce's `Lut16Model` in `f64`. That is **deliberate and is the
//! point**: feeding both estimators the *same* round trip isolates the
//! algorithm difference, which is the only thing under test. The residual
//! pipeline difference is separately bounded by Pass 4b §A (iccce against
//! lcms2 on this exact `B2A1` table, 1,330×10⁻⁴ device) and is checked here
//! end to end against the real `transicc` in §B.
//!
//! ## Sections
//!
//! - **§A — the reimplementation against ISO**, on the same `bt`. No oracle,
//!   no recovery, no error bar. This is what settles Pass 5b's claim 3.
//! - **§B — the reimplementation validated against the real lcms2.** The
//!   reimplemented black is pushed through `B2A1` and compared, in **device**
//!   units, against what `transicc` actually emits at input black with BPC on.
//!   A sensitivity `d(device)/d(L*)` measured on the same table converts that
//!   residual into a bound on the black point in `L*` — which is the error bar
//!   Pass 5b had, now measured on the *right* quantity and expected to be
//!   an order of magnitude smaller.

use std::path::{Path, PathBuf};

use iccce_cmm::bpc::{
    EstimationIntent, darkest_vertex, estimate_lut_destination_black, neutralise_and_clip,
};
use iccce_cmm::lut_ab::LutAbModel;
use iccce_cmm::lut_transform::{Lut16Model, PcsKind, PcsValue};
use iccce_color::Lab;
use iccce_profile::Profile;
use iccce_profile::num::Signature;
use iccce_profile::tag_types::TagData;

use crate::{Kind, Metric, Record, SepUnits, Separation, Tolerance};

pub mod tag {
    use iccce_profile::num::Signature;
    pub const A2B0: Signature = Signature(0x4132_4230);
    pub const A2B1: Signature = Signature(0x4132_4231);
    pub const B2A0: Signature = Signature(0x4232_4130);
    pub const B2A1: Signature = Signature(0x4232_4131);
}

// ===========================================================================
// The reimplementation
// ===========================================================================

/// Every intermediate lcms2 computes on its way to a destination black point,
/// kept so that a divergence can be attributed to a step rather than guessed
/// at from the endpoint. Pass 5b's claim 3 was left unattributed precisely
/// because only the endpoint was visible.
#[derive(Debug, Clone)]
pub struct Lcms2Detect {
    /// Which branch of `cmsDetectBlackPoint` supplied `InitialLab`.
    pub initial_branch: &'static str,
    /// `InitialLab` — for this fixture, neutral by L174.
    pub initial_lab: Lab,
    /// `outRamp[0]` **before** the monotonic pass, i.e. `BT(Lab(0, ka, kb)).L`.
    pub out_ramp_0_raw: f64,
    /// `outRamp[0]` as lcms2 leaves it (untouched by the monotonic pass) and
    /// as ISO leaves it (index 0 included). Equal unless the raw ramp is
    /// non-monotonic at the very bottom.
    pub min_l_lcms2: f64,
    pub min_l_iso: f64,
    pub max_l: f64,
    /// Did the mid-range straightness test fire? If it did, **both sides
    /// return their own `InitialLab`** (lcms2 `cmssamp.c` L536; ISO/CD 18619
    /// 4.2.5.4 final paragraph, since `fd34a44` — before that correction
    /// iccce returned `outRamp[first]` here) — a divergence with a completely
    /// different mechanism from the fit, and on `swop` the *only* one.
    pub nearly_straight: bool,
    /// How many shadow points landed in `[lo, hi)`.
    pub n_shadow: usize,
    /// The quadratic's coefficients, `y = a·x² + b·x + c`.
    pub fit: Option<(f64, f64, f64)>,
    /// The root, before the `[0, 50]` clamp.
    pub root_raw: f64,
    /// The black point lcms2 would return.
    pub black: Lab,
    /// Set when lcms2 would have given up and returned `XYZ(0,0,0)`.
    pub gave_up: Option<&'static str>,
}

/// Which branch of `cmsDetectBlackPoint` (`cmssamp.c` L217-386) supplies
/// `InitialLab`, at `INTENT_RELATIVE_COLORIMETRIC`.
///
/// ##  This enum IS the finding
///
/// lcms2 does not have one way of guessing a black point; at relative
/// colorimetric it has two, and **which one runs is decided by the
/// destination profile's device class and colour space, not by anything
/// about black**:
///
/// ```text
/// if (Intent == INTENT_RELATIVE_COLORIMETRIC &&
///     cmsGetDeviceClass(hProfile) == cmsSigOutputClass &&
///     isInkColorspace(cmsGetColorSpace(hProfile)))
///     return BlackPointUsingPerceptualBlack(BlackPoint, hProfile);   // L370-374
/// ...
/// return BlackPointAsDarkerColorant(hProfile, Intent, BlackPoint, dwFlags);  // L385
/// ```
///
/// The two disagree about the one thing Pass 5b's pre-registered prediction
/// was about: `BlackPointUsingPerceptualBlack` **forces the chroma to zero**
/// (L174) and `BlackPointAsDarkerColorant` **keeps it**. So "does lcms2
/// retain the black's chroma?" has no single answer - it has one answer for
/// a CMYK press profile and the opposite answer for an RGB printer profile,
/// and the only real LUT profile within reach was the first kind.
///
/// `TOLERANCES.md` §0 asks that a tolerance name its scope in the same
/// breath as its number; this is the same discipline applied to a mechanism.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InitialLabBranch {
    /// Output class **and** an ink colour space: `BlackPointUsingPerceptualBlack`,
    /// chroma forced to zero. `USWebCoatedSWOP.icc` takes this one.
    PerceptualBlackDiscountingInk,
    /// Anything else: `BlackPointAsDarkerColorant`, chroma retained.
    /// The synthetic RGB `prtr` fixture takes this one.
    DarkerColorant,
}

/// Choose the branch the way lcms2 chooses it, from the two header fields
/// that decide it. `is_ink` follows `isInkColorspace` (`cmssamp.c` L188-215):
/// CMY, CMYK and every `nCLR`/`MCH` space; **not** RGB, gray, Lab or XYZ.
#[must_use]
pub fn branch_for(is_output_class: bool, is_ink_space: bool) -> InitialLabBranch {
    if is_output_class && is_ink_space {
        InitialLabBranch::PerceptualBlackDiscountingInk
    } else {
        InitialLabBranch::DarkerColorant
    }
}

/// lcms2 2.19.1 `cmsDetectDestinationBlackPoint`, reimplemented from
/// `src/cmssamp.c` at pin `21c582a`, for the case under test: a **v2, output
/// class, ink colorspace, CLUT-based** profile at
/// **`INTENT_RELATIVE_COLORIMETRIC`**.
///
/// # Arguments
///
/// * `bt_rel` — lcms2's `CreateRoundtripXForm(hProfile, INTENT_RELATIVE_COLORIMETRIC)`:
///   `Lab → B2A1 → device → A2B1 → Lab`. The outer leg is always relative
///   (the function's own comment at L38 says so), so at this intent both legs
///   are the media-relative tables.
/// * `bt_perc` — `CreateRoundtripXForm(hProfile, INTENT_PERCEPTUAL)`:
///   `Lab → B2A0 → device → A2B1 → Lab`. Needed only for `InitialLab`, via
///   `BlackPointUsingPerceptualBlack`.
///
/// # Why the branches above this one are asserted by the caller rather than
/// tested here
///
/// The three gates lcms2 applies before reaching this code — device class not
/// link/abstract/named, intent one of perceptual/relative/saturation, and
/// `cmsIsCLUT(profile, intent, USED_AS_OUTPUT)` with a gray/RGB/ink
/// colorspace — are all properties of the *fixture*, checked once in
/// [`analyse`] against the parsed header rather than re-derived per call. A
/// reimplementation that silently took a different branch from the original
/// would be worse than no reimplementation at all, so the branch taken is
/// recorded on the result as [`Lcms2Detect::initial_branch`] and printed on
/// the record.
#[must_use]
pub fn detect_destination_black_point(
    branch: InitialLabBranch,
    bt_rel: &impl Fn(Lab) -> Lab,
    bt_perc: &impl Fn(Lab) -> Lab,
    darkest_colorant_lab: Lab,
) -> Lcms2Detect {
    // --- InitialLab: cmsDetectBlackPoint(REL) ------------------------------
    // L370-374: output class + ink colorspace + relative -> discount the ink
    // limiting via BlackPointUsingPerceptualBlack (L146+), which round trips
    // Lab(0,0,0) through the PERCEPTUAL B2A and the RELATIVE A2B, clips L* to
    // 50, and FORCES the chroma to zero (L174).
    let (initial_lab, branch_note) = match branch {
        InitialLabBranch::PerceptualBlackDiscountingInk => {
            let perc_out = bt_perc(Lab {
                l: 0.0,
                a: 0.0,
                b: 0.0,
            });
            (
                Lab {
                    l: if perc_out.l > 50.0 { 50.0 } else { perc_out.l },
                    a: 0.0,
                    b: 0.0,
                },
                "cmsDetectBlackPoint -> BlackPointUsingPerceptualBlack (L370-374): output class \
                 + INK colorspace + relative. Round trips Lab(0,0,0) through the PERCEPTUAL B2A \
                 and the RELATIVE A2B, clips L* to 50, and FORCES the chroma to 0 at L174",
            )
        }
        InitialLabBranch::DarkerColorant => {
            // BlackPointAsDarkerColorant (L62-145): transform the space's
            // darkest colorant (_cmsEndPointsBySpace: RGB -> (0,0,0),
            // CMYK -> 400 % ink) through the profile's A2B at the given
            // intent, then clip L* ONLY:
            //   if (Lab.L > 95) Lab.L = 0;   // "synthetical negative profiles"
            //   else if (Lab.L < 0) Lab.L = 0;
            //   else if (Lab.L > 50) Lab.L = 50;
            // ★ a* and b* are NOT touched. This is the branch in which the
            // pre-registered mechanism claim can be exercised at all.
            let d = darkest_colorant_lab;
            let l = if d.l > 95.0 {
                0.0
            } else if d.l < 0.0 {
                0.0
            } else if d.l > 50.0 {
                50.0
            } else {
                d.l
            };
            (
                Lab { l, a: d.a, b: d.b },
                "cmsDetectBlackPoint -> BlackPointAsDarkerColorant (L385, the fall-through): the \
                 space's darkest colorant through A2B at the caller's intent, L* clipped to \
                 [0,50] with the >95 rule, CHROMA RETAINED",
            )
        }
    };
    // NOTE: lcms2 converts to XYZ here and back to Lab in the caller
    // (cmsLab2XYZ at L177, cmsXYZ2Lab at L471). That round trip is exact to
    // f64 rounding for a neutral Lab and is not modelled.

    // --- Step 2: the ramps (L491-503) --------------------------------------
    const N: usize = 256;
    let ka = initial_lab.a.clamp(-50.0, 50.0);
    let kb = initial_lab.b.clamp(-50.0, 50.0);
    let mut in_ramp = [0.0f64; N];
    let mut out_ramp = [0.0f64; N];
    for l in 0..N {
        #[allow(clippy::cast_precision_loss)]
        let sample = Lab {
            l: (l as f64) * 100.0 / 255.0,
            // ★ HELD CONSTANT — this is the divergence Pass 5b named, and on
            // this fixture it is worth nothing because ka = kb = 0.
            a: ka,
            b: kb,
        };
        in_ramp[l] = sample.l;
        out_ramp[l] = bt_rel(sample).l;
    }
    let out_ramp_0_raw = out_ramp[0];

    // --- Make monotonic (L506-508) -----------------------------------------
    // ★ `for (l = 254; l > 0; --l)` — index 0 is NEVER assigned. ISO's loop
    // runs to 0. Recorded both ways so the difference is measured.
    let mut iso_ramp = out_ramp;
    for l in (0..N - 1).rev() {
        iso_ramp[l] = iso_ramp[l].min(iso_ramp[l + 1]);
    }
    for l in (1..N - 1).rev() {
        out_ramp[l] = out_ramp[l].min(out_ramp[l + 1]);
    }

    let min_l = out_ramp[0];
    let max_l = out_ramp[N - 1];

    let mut det = Lcms2Detect {
        initial_branch: branch_note,
        initial_lab,
        out_ramp_0_raw,
        min_l_lcms2: min_l,
        min_l_iso: iso_ramp[0],
        max_l,
        nearly_straight: false,
        n_shadow: 0,
        fit: None,
        root_raw: f64::NAN,
        black: Lab {
            l: 0.0,
            a: 0.0,
            b: 0.0,
        },
        gave_up: None,
    };

    // --- Validity (L511-517) ------------------------------------------------
    if !(min_l < max_l) {
        det.gave_up = Some("outRamp[0] >= outRamp[255] (L511)");
        return det;
    }

    // --- Mid-range straightness, relative only (L521-545) -------------------
    let threshold = min_l + 0.2 * (max_l - min_l);
    let mut straight = true;
    for l in 0..N {
        if !(in_ramp[l] <= threshold || (in_ramp[l] - out_ramp[l]).abs() < 4.0) {
            straight = false;
        }
    }
    det.nearly_straight = straight;
    if straight {
        // ★ lcms2 returns InitialLab ITSELF (L536-539) — a value from the
        // PERCEPTUAL round trip, not from the ramp it just computed. ISO
        // 4.2.5.4 also returns *its own* InitialLab (since `fd34a44`), and
        // the two InitialLabs are built from different round trips, which is
        // the whole of the divergence on the `swop` arm.
        det.black = det.initial_lab;
        return det;
    }

    // --- The fit (L549-596) -------------------------------------------------
    let (lo, hi) = (0.1, 0.5); // relative colorimetric (L560-561)
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for l in 0..N {
        let y = (out_ramp[l] - min_l) / (max_l - min_l);
        if y >= lo && y < hi {
            xs.push(in_ramp[l]);
            ys.push(y);
        }
    }
    det.n_shadow = xs.len();
    if xs.len() < 3 {
        det.gave_up = Some("fewer than 3 shadow points (L580)");
        return det;
    }
    // ★ The caller's guard is `n < 3`; the fitter's own is `n < 4` (L317).
    if xs.len() < 4 {
        det.root_raw = 0.0;
        det.black = Lab {
            l: 0.0,
            a: det.initial_lab.a,
            b: det.initial_lab.b,
        };
        det.gave_up = Some("RootOfLeastSquaresFitQuadraticCurve returns 0 for n < 4 (L317)");
        return det;
    }

    match root_of_least_squares_fit_quadratic_curve(&xs, &ys) {
        FitOutcome::Singular => {
            det.root_raw = 0.0;
            det.gave_up = Some("_cmsMAT3inverse: |det| < 1e-4 (lcms2_internal.h L142)");
            det.black = Lab {
                l: 0.0,
                a: det.initial_lab.a,
                b: det.initial_lab.b,
            };
        }
        FitOutcome::Root {
            a,
            b,
            c,
            raw,
            clamped,
        } => {
            det.fit = Some((a, b, c));
            det.root_raw = raw;
            // L588-591: clip a negative vertex to zero, keep InitialLab's
            // chroma. On this fixture that chroma is zero.
            let l = if clamped < 0.0 { 0.0 } else { clamped };
            det.black = Lab {
                l,
                a: det.initial_lab.a,
                b: det.initial_lab.b,
            };
        }
    }
    det
}

enum FitOutcome {
    Singular,
    Root {
        a: f64,
        b: f64,
        c: f64,
        raw: f64,
        clamped: f64,
    },
}

/// `RootOfLeastSquaresFitQuadraticCurve` (`cmssamp.c` L308-392), including
/// lcms2's own matrix inverse and its two clamps.
///
/// lcms2 forms the normal equations and solves them by **inverting** the 3×3
/// (`_cmsMAT3solve` → `_cmsMAT3inverse`, `cmsmtrx.c` L129/L156), rejecting the
/// system when `|det| < 1.0E-4`. That is reproduced rather than replaced with
/// a Gaussian solve: on badly conditioned shadow windows the two do not agree
/// on whether a system is singular, and the whole point of a reimplementation
/// is that its give-up paths are the original's.
fn root_of_least_squares_fit_quadratic_curve(xs: &[f64], ys: &[f64]) -> FitOutcome {
    let n = xs.len();
    #[allow(clippy::cast_precision_loss)]
    let nf = n as f64;
    let (mut sx, mut sx2, mut sx3, mut sx4) = (0.0, 0.0, 0.0, 0.0);
    let (mut sy, mut syx, mut syx2) = (0.0, 0.0, 0.0);
    for i in 0..n {
        let (x, y) = (xs[i], ys[i]);
        sx += x;
        sx2 += x * x;
        sx3 += x * x * x;
        sx4 += x * x * x * x;
        sy += y;
        syx += y * x;
        syx2 += y * x * x;
    }
    let m = [[nf, sx, sx2], [sx, sx2, sx3], [sx2, sx3, sx4]];
    let v = [sy, syx, syx2];
    let Some(res) = mat3_solve(&m, &v) else {
        return FitOutcome::Singular;
    };
    let (a, b, c) = (res[2], res[1], res[0]);

    if a.abs() < 1.0e-10 {
        if b.abs() < 1.0e-10 {
            return FitOutcome::Root {
                a,
                b,
                c,
                raw: 0.0,
                clamped: 0.0,
            };
        }
        let raw = -c / b;
        return FitOutcome::Root {
            a,
            b,
            c,
            raw,
            clamped: raw.clamp(0.0, 50.0),
        };
    }
    let d = b * b - 4.0 * a * c;
    if d <= 0.0 {
        return FitOutcome::Root {
            a,
            b,
            c,
            raw: 0.0,
            clamped: 0.0,
        };
    }
    let raw = (-b + d.sqrt()) / (2.0 * a);
    FitOutcome::Root {
        a,
        b,
        c,
        raw,
        clamped: raw.clamp(0.0, 50.0),
    }
}

/// `_cmsMAT3solve` (`cmsmtrx.c` L156) — invert and multiply, with lcms2's
/// `MATRIX_DET_TOLERANCE` of `1.0E-4`.
fn mat3_solve(m: &[[f64; 3]; 3], v: &[f64; 3]) -> Option<[f64; 3]> {
    let c0 = m[1][1] * m[2][2] - m[1][2] * m[2][1];
    let c1 = -m[1][0] * m[2][2] + m[1][2] * m[2][0];
    let c2 = m[1][0] * m[2][1] - m[1][1] * m[2][0];
    let det = m[0][0] * c0 + m[0][1] * c1 + m[0][2] * c2;
    if det.abs() < 1.0e-4 {
        return None;
    }
    let inv = [
        [
            c0 / det,
            (m[0][2] * m[2][1] - m[0][1] * m[2][2]) / det,
            (m[0][1] * m[1][2] - m[0][2] * m[1][1]) / det,
        ],
        [
            c1 / det,
            (m[0][0] * m[2][2] - m[0][2] * m[2][0]) / det,
            (m[0][2] * m[1][0] - m[0][0] * m[1][2]) / det,
        ],
        [
            c2 / det,
            (m[0][1] * m[2][0] - m[0][0] * m[2][1]) / det,
            (m[0][0] * m[1][1] - m[0][1] * m[1][0]) / det,
        ],
    ];
    Some([
        inv[0][0] * v[0] + inv[0][1] * v[1] + inv[0][2] * v[2],
        inv[1][0] * v[0] + inv[1][1] * v[1] + inv[1][2] * v[2],
        inv[2][0] * v[0] + inv[2][1] * v[1] + inv[2][2] * v[2],
    ])
}

// ===========================================================================
// Fixture plumbing
// ===========================================================================

fn read_lut16(p: &Profile, sig: Signature) -> Option<iccce_profile::lut::Lut16> {
    let e = p.tags.iter().find(|t| t.sig == sig)?;
    match p.decode_tag(e) {
        Some(Ok(d)) => match d.data {
            TagData::Lut16(l) => Some(l),
            _ => None,
        },
        _ => None,
    }
}

fn read_lut8(p: &Profile, sig: Signature) -> Option<iccce_profile::lut::Lut8> {
    let e = p.tags.iter().find(|t| t.sig == sig)?;
    match p.decode_tag(e) {
        Some(Ok(d)) => match d.data {
            TagData::Lut8(l) => Some(l),
            _ => None,
        },
        _ => None,
    }
}

/// A decoded LUT tag of **any** of the three flavours a Lab-PCS profile may
/// carry, behind one evaluator.
///
/// ## Why this abstraction is here rather than in the CMM
///
/// Pass 5c has to drive two fixtures whose tag types have nothing in common —
/// `USWebCoatedSWOP.icc` carries `mft2`/`mft1` and the synthetic v4 fixture
/// carries `mAB `/`mBA ` — through **the same** black-point estimators, and
/// the estimators must not be able to tell which. `Chain` already unifies them
/// for a whole conversion; what is needed here is a bare tag evaluator,
/// because both estimators need `A2B1` and `B2A1` *separately* (the round trip
/// `BT` composes them in a way no `Chain` exposes) and one of them needs
/// `B2A0` as well.
///
/// If a later session finds itself adding a fourth arm here, that is the
/// signal to ask the CMM for the abstraction rather than widening this enum
/// again.
pub enum TagModel {
    /// `lut16Type` or `lut8Type` (`mft2` / `mft1`).
    Lut16(Lut16Model),
    /// `lutAToBType` (`mAB `), evaluated device to PCS.
    Mab(LutAbModel),
    /// `lutBToAType` (`mBA `), evaluated PCS to device.
    Mba(LutAbModel),
}

impl TagModel {
    #[must_use]
    pub fn device_to_pcs(&self, device: &[f64]) -> Option<PcsValue> {
        match self {
            TagModel::Lut16(m) => m.device_to_pcs(device),
            TagModel::Mab(m) | TagModel::Mba(m) => m.device_to_pcs(device),
        }
    }
    #[must_use]
    pub fn pcs_to_device(&self, pcs: PcsValue) -> Option<Vec<f64>> {
        match self {
            TagModel::Lut16(m) => m.pcs_to_device(pcs),
            TagModel::Mab(m) | TagModel::Mba(m) => m.pcs_to_device(pcs),
        }
    }
    /// The DEVICE-side channel count, whichever direction the tag runs in.
    #[must_use]
    pub fn device_channels(&self) -> usize {
        match self {
            TagModel::Lut16(m) => m.input_channels(),
            TagModel::Mab(m) | TagModel::Mba(m) => m.device_channels(),
        }
    }
}

/// Decode one tag into a [`TagModel`], trying every flavour a Lab-PCS profile
/// may legally use for it. `a_to_b` selects how an `mAB `/`mBA ` tag is
/// evaluated, because the same stored elements run in opposite directions.
fn model(p: &Profile, sig: Signature, a_to_b: bool) -> Option<TagModel> {
    if let Some(l) = read_lut16(p, sig) {
        return Lut16Model::from_lut16(&l, false, PcsKind::Lab)
            .ok()
            .map(TagModel::Lut16);
    }
    if let Some(l) = read_lut8(p, sig) {
        return Lut16Model::from_lut8(&l, false, PcsKind::Lab)
            .ok()
            .map(TagModel::Lut16);
    }
    let l = read_lut_ab(p, sig)?;
    if a_to_b {
        LutAbModel::from_lut_ab(&l, PcsKind::Lab).ok().map(TagModel::Mab)
    } else {
        LutAbModel::from_mba(&l, PcsKind::Lab).ok().map(TagModel::Mba)
    }
}

fn read_lut_ab(p: &Profile, sig: Signature) -> Option<iccce_profile::lut::LutAB> {
    let e = p.tags.iter().find(|t| t.sig == sig)?;
    match p.decode_tag(e) {
        Some(Ok(d)) => match d.data {
            TagData::LutAToB(l) | TagData::LutBToA(l) => Some(l),
            _ => None,
        },
        _ => None,
    }
}

/// The three models a Pass 5c run needs from one destination profile.
pub struct Fixture {
    pub a2b1: TagModel,
    pub b2a1: TagModel,
    pub b2a0: TagModel,
    pub channels: usize,
    /// `cmsGetDeviceClass(hProfile) == cmsSigOutputClass`.
    pub is_output_class: bool,
    /// `isInkColorspace(cmsGetColorSpace(hProfile))` - `cmssamp.c` L188-215.
    pub is_ink_space: bool,
    /// ★ The full-scale device value `transicc` PRINTS for this colour space.
    ///
    /// **This is not cosmetic and it silently destroyed a measurement.**
    /// `transicc` prints ink spaces as percentages (`0..100`) and RGB/gray as
    /// `0..255`. Pass 5b and this module's first draft divided every oracle
    /// output by 100 because the only destination in reach was CMYK; the first
    /// run of the synthetic RGB arm therefore read lcms2's device values 2,55x
    /// too large, produced a 0,0998 device residual where the truth is
    /// 4,7e-7, and would have been reported as "the reimplementation does not
    /// reproduce lcms2 on this fixture".
    ///
    /// It was caught because §B carries a SECOND, independent prediction — the
    /// ISO candidate — and BOTH candidates missed by roughly the same amount.
    /// A residual that is large for every hypothesis is an apparatus fault, not
    /// a finding, and that is what the discrimination row is for.
    pub dev_scale: f64,
    pub describe: String,
}

impl Fixture {
    /// Parse a destination profile and build the three LUT models.
    ///
    /// # Errors
    /// When the file cannot be read or parsed, or when any of `A2B1`, `B2A1`
    /// or `B2A0` is missing or not decodable to a LUT model.
    pub fn open(path: &Path) -> Result<Fixture, String> {
        let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
        let p = Profile::parse(&bytes).map_err(|e| e.to_string())?;
        let a2b1 = model(&p, tag::A2B1, true).ok_or("no decodable A2B1")?;
        let b2a1 = model(&p, tag::B2A1, false).ok_or("no decodable B2A1")?;
        // lcms2 falls back through cmsReadOutputLUT's 8.10.2 chain when B2A0
        // is absent; every fixture here has one, and a missing B2A0 is an
        // error rather than a silent substitution.
        let b2a0 = model(&p, tag::B2A0, false).ok_or("no decodable B2A0")?;
        // The device channel count, taken from the A2B tag's own input side
        // rather than from a header signature table — the tag is what the
        // vertex search will actually be evaluated against.
        let channels = a2b1.device_channels();
        let describe = format!(
            "v{:08X} {} {}->{} ({} ch)",
            p.header.version.raw,
            p.header.device_class,
            p.header.color_space,
            p.header.pcs,
            channels
        );
        // The two header fields lcms2's branch turns on, read once here rather
        // than re-derived per call so a reimplementation cannot drift onto a
        // branch the original would not have taken.
        let class = format!("{}", p.header.device_class);
        let space = format!("{}", p.header.color_space);
        let is_output_class = class.contains("prtr");
        let is_ink_space = space.contains("CMYK")
            || space.contains("CMY ")
            || space.contains("CLR")
            || space.contains("MCH");
        // transicc prints ink spaces 0..100 and RGB/gray 0..255.
        let dev_scale = if is_ink_space { 100.0 } else { 255.0 };
        Ok(Fixture {
            a2b1,
            b2a1,
            b2a0,
            channels,
            is_output_class,
            is_ink_space,
            dev_scale,
            describe,
        })
    }

    /// `A2B1` — device to Lab, media-relative.
    #[must_use]
    pub fn a2b1_lab(&self, device: &[f64]) -> Lab {
        match self.a2b1.device_to_pcs(device) {
            Some(PcsValue::Lab(l)) => l,
            _ => Lab {
                l: f64::NAN,
                a: f64::NAN,
                b: f64::NAN,
            },
        }
    }

    /// `BT` at media-relative: `Lab → B2A1 → device → A2B1 → Lab`.
    #[must_use]
    pub fn bt_rel(&self, lab: Lab) -> Lab {
        let d = self.b2a1.pcs_to_device(PcsValue::Lab(lab)).unwrap_or_default();
        self.a2b1_lab(&d)
    }

    /// `BT` at perceptual: `Lab → B2A0 → device → A2B1 → Lab`. The outer leg
    /// is relative in both cases — `CreateRoundtripXForm`'s `Intents[2]` is
    /// hard-coded to `INTENT_RELATIVE_COLORIMETRIC` (L51).
    #[must_use]
    pub fn bt_perc(&self, lab: Lab) -> Lab {
        let d = self.b2a0.pcs_to_device(PcsValue::Lab(lab)).unwrap_or_default();
        self.a2b1_lab(&d)
    }

    /// ISO/CD 18619 4.2.2.2 → 4.2.3 → 4.2.5 on the same round trip.
    #[must_use]
    pub fn iso_black(&self) -> (Vec<f64>, Lab, Lab, f64) {
        let darkest = darkest_vertex(self.channels, |d| self.a2b1_lab(d));
        let darkest_lab = self.a2b1_lab(&darkest);
        let initial = neutralise_and_clip(darkest_lab.l);
        // Full Lab since 2026-08-12 (4.2.5.4 carries InitialLab
        // through, chroma included on non-CMYK destinations).
        let black = estimate_lut_destination_black(
            initial,
            EstimationIntent::RelativeColorimetric,
            |lab| self.bt_rel(lab),
        );
        (darkest, darkest_lab, initial, black.l)
    }
}

// ===========================================================================
// §A + §B — the analysis
// ===========================================================================

/// Everything one Pass 5c run produces. Kept as data rather than printed in
/// place so the same numbers feed the records, the report binary and the
/// coverage statement without being recomputed (and possibly recomputed
/// differently).
#[derive(Debug)]
pub struct Analysis {
    /// Which arm produced this: `swop` or `synthetic`. It prefixes every
    /// record id, because the two arms reach OPPOSITE conclusions about the
    /// same prediction and a reader must never be able to quote one as the
    /// other.
    pub arm: &'static str,
    pub structure: String,
    /// ISO 4.2.2.2's darkest vertex and its Lab.
    pub darkest_device: Vec<f64>,
    pub darkest_lab: Lab,
    pub iso_initial: Lab,
    /// ISO 4.2.5's estimate — neutral by 4.2.3.
    pub iso_black: Lab,
    /// lcms2's, from the reimplementation. No oracle in it.
    pub lcms2: Lcms2Detect,
    /// ★ Which branch of `cmsDetectBlackPoint` this destination's header
    /// selected. Kept as the enum, not only as the prose on
    /// [`Lcms2Detect::initial_branch`], because the **candidate separation** on
    /// the chroma row is "what the OTHER branch would have predicted", and
    /// deriving that from a sentence by substring match is how an apparatus
    /// starts lying.
    pub branch: InitialLabBranch,
    /// §B: what `transicc` actually emits at input black with BPC on.
    pub observed_device: Vec<f64>,
    /// §B: `B2A1(black)` for each candidate black point.
    pub predicted_from_lcms2: Vec<f64>,
    pub predicted_from_iso: Vec<f64>,
    /// §B: what the SHIPPED `iccce transform --bpc` emits, now that the ISO
    /// estimator has a caller.
    pub shipped_device: Option<Vec<f64>>,
    pub shipped_error: Option<String>,
    /// `d(device)/d(L*)` on `B2A1` at the black, max over channels, measured
    /// by a central difference of ±0,5 `L*`.
    pub sensitivity: f64,
    /// `transicc`'s full-scale device value for this colour space: 100 for an
    /// ink space, 255 for RGB or gray. Printed on every record because a
    /// misread of it looks exactly like a disagreement (see [`Fixture::dev_scale`]).
    pub dev_scale: f64,
    /// Pass 5b's recovered black, recomputed here from the same `transicc`
    /// output, and the round trip of the reimplemented black that explains it.
    pub pass5b_recovered: Lab,
    pub roundtrip_of_reimpl: Lab,
}

impl Analysis {
    /// §A's headline: how far apart the two estimators really are.
    #[must_use]
    pub fn estimator_divergence_de76(&self) -> f64 {
        de76(self.iso_black, self.lcms2.black)
    }
    /// What the chroma of the divergence MUST be if lcms2 took the branch this
    /// destination's header selects.
    ///
    /// ISO/CD 18619 4.2.3 always returns a neutral black, so the whole chroma
    /// of the divergence is lcms2's. lcms2's returned chroma is
    /// `InitialLab.a/.b` verbatim (`cmssamp.c` L590-591), and `InitialLab`
    /// comes from whichever branch of `cmsDetectBlackPoint` the **device class
    /// and colour space** select:
    ///
    /// * output class + ink space -> `BlackPointUsingPerceptualBlack`, chroma
    ///   forced to `0` (L174);
    /// * anything else -> `BlackPointAsDarkerColorant`, chroma of the darkest
    ///   colorant, untouched.
    ///
    /// So this is `0` on `USWebCoatedSWOP` and `5,0` on the synthetic RGB
    /// fixture, from the same code, and that difference is the finding.
    #[must_use]
    pub fn expected_divergence_chroma(&self) -> f64 {
        (self.lcms2.initial_lab.a.powi(2) + self.lcms2.initial_lab.b.powi(2)).sqrt()
    }
    /// The chroma component of the divergence, as measured.
    #[must_use]
    pub fn divergence_chroma(&self) -> f64 {
        ((self.iso_black.a - self.lcms2.black.a).powi(2)
            + (self.iso_black.b - self.lcms2.black.b).powi(2))
        .sqrt()
    }
    #[must_use]
    pub fn divergence_lightness(&self) -> f64 {
        (self.iso_black.l - self.lcms2.black.l).abs()
    }
    /// §B: max device residual, reimplemented black against the real lcms2.
    #[must_use]
    pub fn device_residual_lcms2(&self) -> f64 {
        max_abs(&self.predicted_from_lcms2, &self.observed_device)
    }
    /// §B: the same with ISO's black — the discriminating control. If this is
    /// not materially larger, §B has no resolving power and says so.
    #[must_use]
    pub fn device_residual_iso(&self) -> f64 {
        max_abs(&self.predicted_from_iso, &self.observed_device)
    }
    /// §B's error bar, in the unit the claim is made in: the device residual
    /// converted back to `L*` through the measured sensitivity.
    #[must_use]
    pub fn l_star_bound(&self) -> f64 {
        if self.sensitivity > 0.0 {
            self.device_residual_lcms2() / self.sensitivity
        } else {
            f64::INFINITY
        }
    }
    /// How much of Pass 5b's 0,858 ΔE76 the round trip explains.
    #[must_use]
    pub fn recovery_explained(&self) -> f64 {
        de76(self.roundtrip_of_reimpl, self.pass5b_recovered)
    }

    // -----------------------------------------------------------------------
    // ★★★ The named alternative candidates — added 2026-08-12 (DL-033)
    //
    // Each of these is a value one of this section's rows WOULD have observed
    // under a rival reading that was live at some point, and each is computed
    // rather than typed. Together they are what `crate::Separation` needs in
    // order to state, on the record itself, how much power the row has.
    // -----------------------------------------------------------------------

    /// **The alternative black point: `outRamp[first]`** — the quantity
    /// `bpc.rs`'s 4.2.5.4 short-circuit returned until commit `fd34a44`.
    ///
    /// It is not a black-point candidate in any branch of ISO/CD 18619 4.2.5;
    /// it is the floor of the monotonised round-trip ramp, and it is here
    /// because **it is what this project's own code returned for two sessions**
    /// — the definition of a plausible-at-the-time rival. Neutral, because
    /// 4.2.3 neutralises and the ramp carries no chroma of its own.
    #[must_use]
    pub fn alt_black_outramp_first(&self) -> Lab {
        Lab {
            l: self.lcms2.min_l_iso,
            a: 0.0,
            b: 0.0,
        }
    }

    /// What `estimators/black-points-in-lab` would have observed under that
    /// alternative: `8,1668×10⁻²` on the `swop` arm, which is the number the
    /// suite reported for two sessions.
    #[must_use]
    pub fn alt_estimator_divergence_de76(&self) -> f64 {
        de76(self.alt_black_outramp_first(), self.lcms2.black)
    }

    /// ★★★ **The separation itself**: how far apart the two candidate black
    /// points are, in ΔE76. `4,717 441` on `swop` — 57,8× the divergence the
    /// defect was blamed for — and **exactly `0` on the synthetic arm**, whose
    /// `InitialLab` and `outRamp[first]` are both `L* 20`.
    #[must_use]
    pub fn black_candidate_separation_de76(&self) -> f64 {
        de76(self.iso_black, self.alt_black_outramp_first())
    }

    /// The chroma the **other** branch of `cmsDetectBlackPoint` would have
    /// predicted for the divergence.
    ///
    /// `BlackPointUsingPerceptualBlack` forces the chroma to `0` (L174);
    /// `BlackPointAsDarkerColorant` keeps the darkest colorant's. So whichever
    /// branch fired, the rival's prediction differs from the actual one by
    /// **the darkest colorant's chroma** — `0,834` on `USWebCoatedSWOP`,
    /// `5,0` on the synthetic RGB fixture. Computed from the fixture's own
    /// darkest colorant rather than from those two literals, which is the
    /// §3.5.8.6 rule.
    #[must_use]
    pub fn other_branch_predicted_chroma(&self) -> f64 {
        let colorant_chroma =
            (self.darkest_lab.a.powi(2) + self.darkest_lab.b.powi(2)).sqrt();
        match self.branch {
            InitialLabBranch::PerceptualBlackDiscountingInk => colorant_chroma,
            InitialLabBranch::DarkerColorant => 0.0,
        }
    }

    /// The device separation between the two candidate blacks, at the point
    /// where a black point is actually observable: `|B2A1(ISO) − B2A1(lcms2)|`.
    /// `2,46×10⁻³` on `swop` — three orders above the shipped row's `10⁻⁶`
    /// bound, which is the claim `SHIPPED_MATCHES_LIBRARY` used to make with a
    /// typed literal and now makes with this.
    #[must_use]
    pub fn device_candidate_separation(&self) -> f64 {
        max_abs(&self.predicted_from_iso, &self.predicted_from_lcms2)
    }
}

fn de76(a: Lab, b: Lab) -> f64 {
    ((a.l - b.l).powi(2) + (a.a - b.a).powi(2) + (a.b - b.b).powi(2)).sqrt()
}

fn max_abs(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b)
        .map(|(x, y)| (x - y).abs())
        .fold(0.0f64, f64::max)
}

/// Human-readable dump for the scratch bin.
///
/// # Errors
/// Propagates [`Fixture::open`]'s message.
pub fn probe(path: &Path) -> Result<String, String> {
    let f = Fixture::open(path)?;
    let (darkest, darkest_lab, initial, iso_l) = f.iso_black();
    let det = detect_destination_black_point(
        branch_for(f.is_output_class, f.is_ink_space),
        &|l| f.bt_rel(l),
        &|l| f.bt_perc(l),
        darkest_lab,
    );
    let mut s = String::new();
    s.push_str(&format!("fixture: {}\n", f.describe));
    s.push_str(&format!(
        "ISO   : darkest vertex {darkest:?} -> Lab({:.6} {:.6} {:.6}); InitialLab({:.6} {:.6} {:.6}); black L* = {iso_l:.6}\n",
        darkest_lab.l, darkest_lab.a, darkest_lab.b, initial.l, initial.a, initial.b
    ));
    s.push_str(&format!(
        "lcms2 : branch = {}\n        InitialLab({:.6} {:.6} {:.6})\n        outRamp[0] raw {:.6}; MinL lcms2 {:.6} / ISO {:.6}; MaxL {:.6}\n        nearlyStraight = {}; n shadow = {}; fit = {:?}; root raw {:.6}\n        black = Lab({:.6} {:.6} {:.6}); gave up: {:?}\n",
        det.initial_branch,
        det.initial_lab.l, det.initial_lab.a, det.initial_lab.b,
        det.out_ramp_0_raw, det.min_l_lcms2, det.min_l_iso, det.max_l,
        det.nearly_straight, det.n_shadow, det.fit, det.root_raw,
        det.black.l, det.black.a, det.black.b, det.gave_up
    ));
    s.push_str(&format!(
        "DIFF  : dL* = {:.6}; da* = {:.6}; db* = {:.6}\n",
        iso_l - det.black.l,
        0.0 - det.black.a,
        0.0 - det.black.b
    ));
    Ok(s)
}

// ===========================================================================
// The run
// ===========================================================================

const SWOP: &str = r"C:\Windows\System32\spool\drivers\color\USWebCoatedSWOP.icc";
const SRGB: &str = r"C:\Windows\System32\spool\drivers\color\sRGB Color Space Profile.icm";

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

impl From<crate::DiffError> for Unavailable {
    fn from(e: crate::DiffError) -> Self {
        Unavailable::Error(e.to_string())
    }
}

/// Run Pass 5c.
///
/// §A needs no oracle at all — both black points are computed here, one by
/// `iccce_cmm::bpc` and one by this file's reimplementation, on the *same*
/// round trip. §B needs `transicc` and the shipped `iccce` binary.
///
/// # Errors
/// [`Unavailable::Skip`] when a profile or a binary is absent (a machine
/// without the colour directory is not a failing machine); [`Unavailable::Error`]
/// when something that should have worked did not.
pub fn analyse(
    oracle: &crate::Oracle,
    arm: &'static str,
    dst: &Path,
) -> Result<Analysis, Unavailable> {
    let src_path = Path::new(SRGB);
    let dst_path = dst;
    for p in [src_path, dst_path] {
        if !p.is_file() {
            return Err(Unavailable::Skip(format!(
                "profile not present on this machine: {} (LEGAL.md §3 category (c))",
                p.display()
            )));
        }
    }
    let iccce = match crate::Iccce::locate() {
        Err(e) => return Err(Unavailable::Error(e.to_string())),
        Ok(None) => {
            return Err(Unavailable::Skip(
                "iccce binary not found: run `cargo build --release -p iccce-cli`".into(),
            ));
        }
        Ok(Some(i)) => i,
    };

    let f = Fixture::open(dst_path).map_err(Unavailable::Error)?;

    // --- §A: the two estimators, on one round trip, with no oracle ----------
    let (darkest_device, darkest_lab, iso_initial, iso_l) = f.iso_black();
    let lcms2 = detect_destination_black_point(
        branch_for(f.is_output_class, f.is_ink_space),
        &|l| f.bt_rel(l),
        &|l| f.bt_perc(l),
        darkest_lab,
    );
    let iso_black = Lab { l: iso_l, a: 0.0, b: 0.0 };

    // --- §B: the real lcms2, at input black, BPC on -------------------------
    let req = crate::Request {
        input: crate::Space::profile(src_path),
        output: crate::Space::profile(dst_path),
        intent: crate::Intent::RelativeColorimetric,
        precalc: crate::Precalc::Exact,
        bpc: crate::Bpc::On,
        values: vec![0.0, 0.0, 0.0],
    };
    let observed = oracle.convert_batch_shaped(&req, 3, f.channels)?;
    let observed_device: Vec<f64> = observed[0].iter().map(|v| v / f.dev_scale).collect();

    // BPC's second constraint sends the source black EXACTLY to the
    // destination black (Pass 5 row P3, 3,33e-16), and this source's black is
    // XYZ(0,0,0), so the CMYK an implementation emits at input black is
    // `B2A1(its own detected destination black)` and nothing else. That is why
    // a black point can be validated in DEVICE units with no round trip.
    let predicted_from_lcms2 = f
        .b2a1
        .pcs_to_device(PcsValue::Lab(lcms2.black))
        .unwrap_or_default();
    let predicted_from_iso = f
        .b2a1
        .pcs_to_device(PcsValue::Lab(iso_black))
        .unwrap_or_default();

    // The shipped binary's ISO arm, reachable since the estimator was wired.
    let (shipped_device, shipped_error) = match iccce.transform_rows_shaped_bpc(
        src_path,
        dst_path,
        crate::Intent::RelativeColorimetric,
        &[vec![0.0, 0.0, 0.0]],
        f.channels,
        true,
    ) {
        Ok(rows) => (Some(rows[0].clone()), None),
        Err(e) => (None, Some(e.to_string())),
    };

    // d(device)/d(L*) at the black — a central difference of ±0,5 L*, which is
    // the unit conversion §B's residual has to pass through to become a claim
    // about a black point. Measured, never assumed.
    let up = f
        .b2a1
        .pcs_to_device(PcsValue::Lab(Lab { l: lcms2.black.l + 0.5, a: 0.0, b: 0.0 }))
        .unwrap_or_default();
    let down = f
        .b2a1
        .pcs_to_device(PcsValue::Lab(Lab { l: lcms2.black.l - 0.5, a: 0.0, b: 0.0 }))
        .unwrap_or_default();
    let sensitivity = max_abs(&up, &down); // ÷ 1,0 L*

    // Pass 5b's recovery, recomputed: A2B1 of what lcms2 emitted, against the
    // round trip of the reimplemented black. If the two agree, Pass 5b's
    // 0,858 dE76 is fully accounted for as apparatus.
    let pass5b_recovered = f.a2b1_lab(&observed_device);
    let roundtrip_of_reimpl = f.bt_rel(lcms2.black);

    let structure = format!(
        "arm={arm} | dst {} (output_class={}, ink_space={} -> lcms2 InitialLab branch {:?}) | src \
         sRGB v2.1 matrix/TRC | media-relative | lcms2's estimator REIMPLEMENTED from cmssamp.c \
         at pin 21c582a (source-read, no lcms2 binary in section A); ISO/CD 18619 4.2.5 from \
         iccce_cmm::bpc. Both driven on the SAME round trip, in f64",
        f.describe,
        f.is_output_class,
        f.is_ink_space,
        branch_for(f.is_output_class, f.is_ink_space),
    );

    Ok(Analysis {
        arm,
        structure,
        darkest_device,
        darkest_lab,
        iso_initial,
        iso_black,
        lcms2,
        branch: branch_for(f.is_output_class, f.is_ink_space),
        observed_device,
        predicted_from_lcms2,
        predicted_from_iso,
        shipped_device,
        shipped_error,
        sensitivity,
        dev_scale: f.dev_scale,
        pass5b_recovered,
        roundtrip_of_reimpl,
    })
}

// ===========================================================================
// Tolerances
// ===========================================================================

/// **§B, the apparatus — and it is deliberately Pass 5b row Q1's constant.**
///
/// Q1's derivation is carried verbatim because it is the right one and it has
/// no free parameter: *an error bar is readable exactly when it is smaller
/// than the effect it bounds.* What changes is the error bar, not the rule.
///
/// - Pass 5b bounded its recovered black by `A2B1 ∘ B2A1`'s round-trip
///   residual: **0,813 7 ΔE76 against an effect of 0,858 2 — ratio 0,948**,
///   green by 5 %, and three of its four claims had to be qualified by it.
/// - Pass 5c does not recover anything. It **computes** lcms2's black from
///   lcms2's own algorithm, pushes it through `B2A1`, and compares in
///   **device** units against what `transicc` actually emitted. The residual
///   is then converted back to `L*` through a sensitivity measured on the same
///   table, so the bound is in the unit the claim is made in.
///
/// Graded quantity: `L* bound ÷ the estimators' divergence`. Below 1 the
/// experiment can discriminate; at or above 1 every §A row sits inside its own
/// uncertainty and this section is void rather than merely worse — exactly
/// Q1's wording, exactly Q1's number.
/// ★★ **Is the destination black point observable in device space on this
/// fixture at all?** Declared per arm, in this table, rather than inferred at
/// run time.
///
/// ## The measurement that made this necessary
///
/// §B's whole method is: push a candidate black through `B2A1`, compare the
/// device values against what `transicc` emits, and convert the residual back to
/// `L*` through `d(device)/d(L*)` measured on the same table. **That conversion
/// presupposes the derivative is non-zero**, and on the `floored` arm it is
/// **structurally zero by construction**: that fixture's `B2A` floors `G` for
/// every input, so every `Lab` with `L* ≤ 37,5` maps to the *same* device value
/// and one `L*` of black-point error moves the device output by nothing. The
/// first run measured `d(device)/d(L*) = 1,11×10⁻¹⁶` and the row reported an
/// `L*` bound of `3,8×10¹⁰` — correctly. §B is **void** on that arm and the
/// apparatus row is what said so.
///
/// ## Why a declared table and not a run-time test
///
/// Switching a row from graded to reported because a measured quantity came out
/// small is the auto-widening this whole document family exists to prevent: a
/// genuine sensitivity collapse on `swop` would silently disable the row that
/// would have caught it. So the *decision* is authored here, reviewable in a
/// diff, and the *measurement* is graded against it by
/// `apparatus/black-is-device-observable-as-declared`. Neither can drift without
/// a row moving.
///
/// The cutoff is [`OBSERVABLE_FLOOR`], and the margins are not close: `swop`
/// measures `1,7×10⁻²`, `synthetic` `8,1×10⁻³`, `floored` `1,1×10⁻¹⁶`.
pub const DEVICE_OBSERVABLE: &[(&str, bool)] = &[
    // A real ink set: B2A1 is a genuine gamut mapping and moving the black's
    // L* moves the CMYK it separates to.
    ("swop", true),
    // The affine RGB model with an EXACT inverse: dL*/dG is 80 and dG/dL* is
    // 1/80 everywhere, so the black is as observable as anything in the file.
    ("synthetic", true),
    // ★ FALSE BY DESIGN. The floor is the fixture's whole purpose and it makes
    // the black unobservable in device space below L* 37.5. That is not a
    // defect of the fixture: the fixture is for §C, which measures the returned
    // Lab directly and needs no device observability at all.
    ("floored", false),
];

/// The cutoff for "observable", in normalised device units per `L*`.
///
/// **Derived from the shipped surface, not chosen.** `iccce transform` prints
/// device values to six decimals, so one printed lsb is `10⁻⁶` — the same
/// constant [`SHIPPED_MATCHES_LIBRARY`] rests on. A sensitivity below it means
/// a whole `L*` of black-point error moves the printed output by less than one
/// printed digit, which is the operational definition of unobservable at the
/// only surface a user sees.
pub const OBSERVABLE_FLOOR: f64 = 1e-6;

/// Look up [`DEVICE_OBSERVABLE`]. An arm absent from the table is treated as
/// observable, because that is the assumption §B was written under and a new arm
/// should have to *declare* the exception rather than acquire it by omission.
#[must_use]
pub fn declared_observable(arm: &str) -> bool {
    DEVICE_OBSERVABLE
        .iter()
        .find(|(a, _)| *a == arm)
        .is_none_or(|(_, o)| *o)
}

/// **§B, the precondition.** Does the measured `d(device)/d(L*)` agree with
/// [`DEVICE_OBSERVABLE`]'s declaration for this arm?
///
/// `0/1`, graded at exactly zero. This is the row that keeps the declaration
/// honest in **both** directions: an arm declared observable whose sensitivity
/// collapsed would fail here rather than quietly producing an enormous `L*`
/// bound, and an arm declared unobservable that became observable would fail
/// here rather than quietly keeping a reported row that could now be graded.
pub const OBSERVABILITY_AS_DECLARED: Tolerance = Tolerance::new(
    0.0,
    "0 if the measured d(device)/d(L*) on this arm's B2A1 agrees with DEVICE_OBSERVABLE's declared \
     value for the arm, 1 otherwise; the cutoff is 1e-6 normalised device per L*, which is the \
     CLI's own printed lsb - one whole L* of black-point error moving the printed output by less \
     than one printed digit. GRADED AT EXACTLY ZERO because the declaration is authored in a table \
     and the measurement either agrees with it or does not; there is no intermediate state to \
     tolerate. The margins are ten orders wide in one direction and four in the other (swop \
     1.7e-2, synthetic 8.1e-3, floored 1.1e-16), so this is not a marginal test - it exists so \
     that a fixture cannot acquire OR lose device observability without a row moving",
);

/// **§B on an arm where the black is not observable in device space.** Reported,
/// with the reason, rather than graded.
///
/// This is **not** a widened [`APPARATUS_RATIO`]. The constant `1.0` is
/// unchanged and still applies wherever the conversion it depends on exists; on
/// the `floored` arm the conversion does not exist, because `d(device)/d(L*)` is
/// zero by the fixture's construction. Grading a ratio whose denominator is a
/// division by a structural zero would be arithmetic, not evidence — the same
/// judgement the `ATTRIBUTION` row makes about incommensurable units and Pass 6
/// row R5 made about populations.
pub const APPARATUS_VOID_UNOBSERVABLE: Tolerance = Tolerance::new(
    f64::INFINITY,
    "REPORTED, NOT GRADED, and the reason is structural rather than convenient: this arm's fixture \
     floors its B2A, so d(device)/d(L*) is ZERO BY CONSTRUCTION and section B's device residual \
     carries no information about a black point's L* at all. The APPARATUS_RATIO constant 1.0 is \
     NOT widened - it is unchanged and still applies on every arm where the conversion it needs \
     exists. Which arms those are is declared in DEVICE_OBSERVABLE and graded by \
     apparatus/black-is-device-observable-as-declared, so this row cannot become reported by \
     accident. What section B is void FOR here is the L* bound; the device rows below still \
     compare real device values and still discriminate, on this fixture through the CHROMA path",
);

pub const APPARATUS_RATIO: Tolerance = Tolerance::new(
    1.0,
    "DELIBERATELY THE SAME CONSTANT AS PASS 5b ROW Q1, and the same derivation: an error bar is \
     readable exactly when it is smaller than the effect it bounds, so the constant is 1.0 and \
     there is no free parameter in it. What changed is the error bar. Q1 bounded a RECOVERED \
     black by the A2B1(B2A1(.)) round trip and scored 0.948 - green by 5 percent. This row \
     bounds a REIMPLEMENTED black by its own device residual against transicc, converted to L* \
     through a sensitivity measured on the same B2A1 table",
);

/// **§A, ★★ the row that settles Pass 5b's claim 3.**
///
/// The chroma component of the two estimators' divergence, graded at
/// **exactly zero**.
///
/// Both implementations return a **neutral** destination black for this
/// fixture and intent, by two entirely different routes:
///
/// - ISO/CD 18619 **4.2.3** neutralises the initial black and 4.2.5 returns
///   `(z, 0, 0)`.
/// - lcms2 reaches `BlackPointUsingPerceptualBlack` (`cmssamp.c` L370-374,
///   because SWOP is output-class and CMYK at relative), which **forces**
///   `LabOut.a = LabOut.b = 0` at L174, and then returns
///   `Lab.a = InitialLab.a` at L590.
///
/// So the pre-registered prediction's *shape* claim — "at input black the
/// divergence should equal the detected destination black's `sqrt(a*^2+b*^2)`"
/// — is **FALSIFIED**: that quantity is **0**, and 100 % of the divergence is
/// in `L*`.
///
/// `0,0 — exact`, not an epsilon: both sides assign a literal `0.0`, so any
/// non-zero value means a reimplementation took a branch the original does
/// not. The obvious wrong branch, `BlackPointAsDarkerColorant`, would return
/// this profile's darkest colorant with its chroma intact, which is orders
/// above anything a rounding argument could excuse.
///
/// ★ **That distance is no longer asserted here — it is the row's emitted
/// candidate separation** ([`Analysis::other_branch_predicted_chroma`], the
/// `separation` column). This justification used to spell it out as *"0,834 on
/// USWebCoatedSWOP, 5,0 on the synthetic RGB fixture"*; both were true, and
/// both are properties of *which fixture is loaded*, so a third arm would have
/// made the sentence wrong without touching a line of it.
pub const NEUTRAL_EXACT: Tolerance = Tolerance::new(
    0.0,
    "the chroma of the divergence MINUS what lcms2's selected branch says it must be, graded at \
     EXACTLY zero because both quantities are assigned literally rather than computed: ISO/CD \
     18619 4.2.3 returns a neutral black, and lcms2 returns InitialLab.a/.b verbatim (L590-591) \
     from a branch chosen by the destination's DEVICE CLASS and COLOUR SPACE. Not an epsilon: \
     taking the OTHER branch changes this by the darkest colorant's WHOLE chroma, which no \
     rounding argument reaches - and that distance is PRINTED as this row's candidate separation \
     rather than asserted here, because it is a property of whichever fixture is loaded. \
     STRUCTURAL on the reimplementation's side; section B is what makes it evidence",
);

/// **§A, the branch.** `0/1`: did BOTH estimators take the mid-range
/// straightness short-circuit?
///
/// This is graded rather than reported because **Pass 5b asserted the
/// opposite** — §17.3 called this configuration "precisely lcms2's method-4
/// (quadratic-fit) territory". It is not. On this fixture the round trip is
/// straight enough above the shadow that `cmssamp.c` L521-545 returns before
/// any fitting happens, and `bpc.rs`'s own 4.2.5.4 test does the same. **No
/// quadratic is fitted by either implementation**, so every statement in
/// Pass 5b about the fit, the shadow window or the root describes code that
/// did not run.
///
/// A build in which either side stopped short-circuiting would change what
/// the whole section is about, which is why the branch is a graded
/// precondition and not a footnote.
pub const BRANCH_EXACT: Tolerance = Tolerance::new(
    0.0,
    "0 if BOTH estimators took the mid-range straightness short-circuit, 1 otherwise. GRADED, not \
     reported, because Pass 5b section 17.3 asserted the opposite - it called this configuration \
     lcms2's method-4 (quadratic fit) territory, and no quadratic is fitted by either side. Every \
     Pass 5b statement about the shadow window or the root describes code that did not run",
);

/// **§B, the discrimination.** `residual under the lcms2 hypothesis ÷ residual
/// under the ISO hypothesis`.
///
/// A reimplementation that predicted lcms2's output no better than the rival
/// candidate did would be evidence of nothing, **however small its absolute
/// residual**. This row is what establishes that §B can tell the two
/// candidates apart at all.
///
/// ★ **The separation between the candidates is now measured and printed
/// rather than quoted.** This justification used to assert *"the two
/// candidates are only 0,082 `L*` apart"* — a literal typed in on
/// **2026-08-12** when it was true, and false by the afternoon of the same
/// day. Commit `fd34a44` corrected `bpc.rs`'s 4.2.5.4 short-circuit to return
/// `InitialLab`, and the separation on the `swop` arm went to **4,799 `L*`**,
/// 59× larger. **The argument survives the change and the number did not**,
/// which is exactly why the number does not belong in the prose: a
/// claim-bearing figure the apparatus can compute must be interpolated at run
/// time, never spelled out beside the code that computes it.
///
/// Tolerance **1,0**, no free parameter: below 1 the lcms2 model is the better
/// explanation of lcms2's own output; at or above 1 §B has no resolving power
/// and must be read as such. **Note the direction of the evidence.** The
/// smaller this ratio, the further apart the candidates are — so a run in
/// which it *improves* is not necessarily a run in which anything got better.
/// On 2026-08-12 it improved from 1,715×10⁻¹ to 4,258×10⁻² **because iccce's
/// own estimate moved away from lcms2's**, which is a finding and not a
/// success. Read it with row `estimators/black-points-in-lab` beside it.
pub const DISCRIMINATES: Tolerance = Tolerance::new(
    1.0,
    "the device residual against transicc under the REIMPLEMENTED lcms2 black, divided by the \
     residual under the ISO black. A reimplementation that predicted lcms2's own output no better \
     than the rival candidate did would be evidence of nothing however small its absolute \
     residual. Below 1 the lcms2 model is the better explanation; at or above 1 this section has \
     no resolving power. The separation between the two candidates is PRINTED in the context \
     field rather than quoted here: it was 0.082 L* until commit fd34a44 corrected 4.2.5.4, and \
     4.799 L* after, so a literal in this string would have been false within a day. A SMALLER \
     ratio does not mean a better reimplementation - it can also mean the candidates moved \
     apart, which is what happened",
);

/// **§B, the attribution.** `what Pass 5b's recovery leaves unexplained ÷
/// this section's own error bar`.
///
/// Pass 5b recovered a black point of `L* 17,215 · a* 0,347 · b* 0,300` and
/// reported it 0,858 ΔE76 from ISO's. If that was the round trip rather than
/// lcms2, then `BT(black_reimplemented)` should land on it. This row grades
/// how much is left over when it does.
///
/// The denominator is §B's own `L*` bound rather than a chosen constant:
/// **an explanation accounts for an effect when what is left over is inside
/// the uncertainty of the explanation.** It is a slightly *strict* denominator
/// — the true uncertainty on `BT(black)` also contains iccce's and lcms2's
/// disagreement about the two tables — and being strict in the direction that
/// could only fail the row is the right way round.
pub const RECOVERY_EXPLAINED: Tolerance = Tolerance::new(
    1.0,
    "what Pass 5b's recovered black leaves unexplained once BT(reimplemented black) is subtracted \
     from it, divided by this section's own L* bound. An explanation accounts for an effect when \
     what is left over is inside the uncertainty of the explanation. The denominator is \
     deliberately STRICT - the true uncertainty on BT(black) also contains the two \
     implementations' disagreement about the A2B1 and B2A1 tables - because being strict in the \
     direction that can only fail the row is the right way round",
);

/// **§B, the shipped surface.** `iccce transform --bpc` against the same ISO
/// black driven in process, in device units.
///
/// Pass 5b's row Q8 graded this case as a **refusal**: the ISO estimator had
/// no caller, so the shipped binary exited 1. It has since been wired
/// (`Chain::estimate_dst_black`), and the row is superseded — Q8's premise is
/// gone and grading a refusal that no longer happens would be grading history.
///
/// The replacement grades the thing that matters now: **the shipped binary
/// reaches the same black point the library function does.** A wiring that
/// passed a differently-derived `InitialLab`, or the perceptual `BT` instead
/// of the relative one, would still convert and still look plausible.
///
/// `1×10⁻⁶`: the CLI prints device values to **six** decimals, so one printed
/// lsb is `10⁻⁶` and the bound is that and nothing else. It cannot absorb a
/// different black point, and **how far it is from being able to is this row's
/// emitted candidate separation** rather than a number in this sentence.
///
/// ## ★ A fourth stale literal, caught by the separation mechanism itself
///
/// Until 2026-08-12 (later) this justification asserted *"the two candidate
/// blacks in play are `2,46×10⁻³` apart in this same quantity, **three orders**
/// above the bound"*. When the separation was computed instead of typed, it
/// came out at **`9,574×10⁻³`** — because `2,46×10⁻³` was the pre-`fd34a44`
/// figure (README §19.10's table records `2,4639×10⁻³ → 9,9211×10⁻³` for the
/// ISO-hypothesis residual on the same commit). The argument was unharmed and
/// got *stronger*: the separation is **~9 600×** the bound, four orders, not
/// three. It joins the three literals §3.5.8.6 already records, and it is the
/// first one found **by an apparatus rather than by a person**, on the first
/// run of the field that computes it.
pub const SHIPPED_MATCHES_LIBRARY: Tolerance = Tolerance::new(
    1e-6,
    "the shipped `iccce transform --bpc` against B2A1(ISO black) computed in process. The CLI \
     prints device values to six decimals, so one printed lsb is 1e-6 and the bound is that and \
     nothing else. It CANNOT absorb a different black point, and the distance to the rival \
     candidate - the shipped binary reaching lcms2's black instead - is PRINTED as this row's \
     candidate separation rather than asserted here. This string used to assert '2.46e-3, three \
     orders above the bound'; computing it gave 9.574e-3 (the 2.46e-3 was the PRE-fd34a44 \
     figure), so the claim was understated by a factor of 4 and was going to go on being wrong. \
     SUPERSEDES Pass 5b row Q8, whose premise (the estimator has no caller) no longer holds",
);

pub const REPORTED: Tolerance = Tolerance::new(
    f64::INFINITY,
    "REPORTED, NOT GRADED - recorded so the number is on file next to the ones that are graded",
);

/// ★★ **`estimators/black-points-in-lab` stays `REPORTED`, and the question is
/// worth answering in full because the answer generalises.**
///
/// The engineer asked, on 2026-08-12: the row named for the whole Pass 5c
/// finding is `UNGRADED` with tolerance `∞`, so the apparatus now says it has no
/// power. Does the `4,717 441` separation supply the derivation basis for a real
/// tolerance?
///
/// **No — and the reason is that a separation is a derivation basis for a
/// FIXTURE, not for a tolerance.**
///
/// 1. **There is nothing for a tolerance on this row to mean.** The row observes
///    the distance between two implementations' *destination black point*
///    estimates. Since `fd34a44` both sides return a quantity their own document
///    calls `InitialLab`, and the two documents mean different things by the
///    name: ISO/CD 18619 4.2.2.2's darkest device vertex, and lcms2's perceptual
///    round trip. **No clause requires them to agree**, and grading their
///    difference would be grading iccce against lcms2's reading of a document
///    iccce does not implement — exactly what `CLAUDE.md` rule 7 forbids and
///    what `TOLERANCES.md` §1 means by a weak claim quoted as a strong one.
/// 2. **A bound derived from the separation would be a number tuned to one
///    known defect.** Any value below `4,717 441` would have failed the
///    pre-`fd34a44` build and any value above it would not; nothing else
///    constrains it. And it could not be one number: the three arms observe
///    `4,799`, `5,000` and `10,000`, so it would have to be three constants,
///    each fitted to its own fixture. That is the shape of a tolerance somebody
///    moved until the suite went green, arrived at from the other end.
/// 3. **The defect it would have caught is now caught by a row with a real
///    derivation.** [`CLAUSE_4254`] grades the same regression at half a PCSLAB
///    quantum against a constant in `recipes.rs`, with no free parameter, and
///    the proof-of-power run showed it failing at `2,500 019×10¹` against
///    `7,629×10⁻⁴` while everything else stayed green.
///
/// So the shape the engineer questioned — *the row named for the finding has no
/// power; the power lives elsewhere* — **is right, and is now demonstrably
/// right rather than merely tolerated.** What the separation did was tell us the
/// power was in the wrong place. The correct response to that was to build an
/// instrument with a derivable bound, not to attach a tuned bound to a row whose
/// quantity no standard constrains.
///
/// **The generalisation, for the next time this question comes up:** a large
/// separation on an `UNGRADED` row is a request for a *fixture and a graded row
/// elsewhere*, not a licence to grade that row. Ask what clause the number would
/// be graded against; if the answer is "none, but it would have caught the bug",
/// the bound is fitted to the bug.
const _WHY_THE_ESTIMATOR_ROW_IS_NOT_GRADED: () = ();

// ===========================================================================
// Records
// ===========================================================================

#[must_use]
pub fn records(a: &Analysis) -> Vec<Record> {
    // Every record id is prefixed with the arm; the two arms reach opposite
    // verdicts and must never be quotable as one another.
    let arm = a.arm;
    let ctx = format!(
        "{} | ISO: darkest vertex {:?} -> Lab({:.4} {:.4} {:.4}), InitialLab({:.4} 0 0), black \
         L*={:.6} | lcms2 REIMPLEMENTED: branch = {}, InitialLab L*={:.6} (chroma FORCED to 0 at \
         cmssamp.c L174), nearlyStraight={}, shadow points={}, black Lab({:.6} {:.6} {:.6}) | \
         MinL(lcms2, index 0 untouched)={:.6} vs MinL(ISO, index 0 included)={:.6} | \
         transicc CMYK at input black {:?} | B2A1(lcms2 black) {:?} | B2A1(ISO black) {:?} | \
         d(device)/d(L*) = {:.6} | transicc device scale for this space = {:.0}",
        a.structure,
        a.darkest_device,
        a.darkest_lab.l,
        a.darkest_lab.a,
        a.darkest_lab.b,
        a.iso_initial.l,
        a.iso_black.l,
        a.lcms2.initial_branch,
        a.lcms2.initial_lab.l,
        a.lcms2.nearly_straight,
        a.lcms2.n_shadow,
        a.lcms2.black.l,
        a.lcms2.black.a,
        a.lcms2.black.b,
        a.lcms2.min_l_lcms2,
        a.lcms2.min_l_iso,
        a.observed_device,
        a.predicted_from_lcms2,
        a.predicted_from_iso,
        a.sensitivity,
        a.dev_scale,
    );
    let observable_declared = declared_observable(arm);
    let observable_measured = a.sensitivity > OBSERVABLE_FLOOR;
    let mut out = vec![
        Record::graded(
            format!("pass5c/{arm}/apparatus/black-is-device-observable-as-declared"),
            Kind::SelfConsistency,
            Metric::AbsMaxComponent,
            OBSERVABILITY_AS_DECLARED,
            if observable_measured == observable_declared {
                0.0
            } else {
                1.0
            },
            "the measured d(device)/d(L*) on this arm's B2A1 against DEVICE_OBSERVABLE's declared \
             value. Section B converts a device residual into an L* bound by dividing by this \
             derivative, which presupposes it is non-zero; the `floored` fixture makes it ZERO BY \
             CONSTRUCTION and is declared so. The declaration is authored in a table rather than \
             inferred at run time, because a row that switched itself from graded to reported \
             whenever a measured quantity came out small would disable exactly the check that \
             would have caught a real collapse",
            format!(
                "{ctx} | measured d(device)/d(L*) = {:.6e}, cutoff {:.0e} (the CLI's own printed \
                 lsb) -> observable={observable_measured}; DEVICE_OBSERVABLE declares \
                 {observable_declared}",
                a.sensitivity, OBSERVABLE_FLOOR,
            ),
        )
        // A 0/1 indicator: the two candidate observations are 0 and 1, one
        // apart, whichever this build produces. See the sibling comment on the
        // quadratic row and `Separation::against`'s doc comment.
        .with_separation(Separation::against_distance(
            "the opposite declaration for this arm in DEVICE_OBSERVABLE - i.e. an arm whose \
             fixture makes the black unobservable being graded as though section B's L* bound \
             meant something, or an observable arm's bound being silently reported",
            1.0,
            1.0,
            SepUnits::SameAsMetric,
        )),
        Record::graded(
            format!("pass5c/{arm}/apparatus/error-bar-is-smaller-than-the-effect"),
            Kind::SelfConsistency,
            Metric::AbsMaxComponent,
            if observable_declared {
                APPARATUS_RATIO
            } else {
                APPARATUS_VOID_UNOBSERVABLE
            },
            if a.estimator_divergence_de76() > 0.0 {
                a.l_star_bound() / a.estimator_divergence_de76()
            } else {
                f64::INFINITY
            },
            "the device residual of the reimplemented black against transicc, converted to L* \
             through a sensitivity measured on the same B2A1 table, divided by the estimators' \
             divergence. Pass 5b's equivalent row scored 0.948 on a RECOVERED black; this one \
             replaces the recovery with a reimplementation",
            format!(
                "{ctx} | device residual {:.6e} / sensitivity {:.6} = L* bound {:.6}; effect \
                 {:.6} dE76; ratio {:.4} against Pass 5b row Q1's 0.9482 on the same fixture",
                a.device_residual_lcms2(),
                a.sensitivity,
                a.l_star_bound(),
                a.estimator_divergence_de76(),
                a.l_star_bound() / a.estimator_divergence_de76(),
            ),
        )
        .with_separation(Separation::none(
            "both terms of this ratio are measured in THIS run from the same tables - the \
             numerator is a device residual against transicc converted through a measured \
             sensitivity, the denominator is the estimators' divergence. There is no rival \
             READING of either; a rival would be a different apparatus, which is what section B \
             already is. Note that the numerator and denominator can move independently and did \
             (2026-08-12: the denominator grew 59x with the numerator unmoved), which is a \
             different hazard from a small separation and is called out on the row itself",
        )),
        Record::graded(
            format!("pass5c/{arm}/FINDING/divergence-chroma-follows-lcms2-BRANCH"),
            Kind::CrossCheck,
            Metric::AbsMaxComponent,
            NEUTRAL_EXACT,
            (a.divergence_chroma() - a.expected_divergence_chroma()).abs(),
            "THE FINDING, and it has two opposite answers from ONE piece of lcms2 code. \
             cmsDetectDestinationBlackPoint returns Lab.a = InitialLab.a (L590-591), and \
             InitialLab comes from whichever branch of cmsDetectBlackPoint the destination's \
             DEVICE CLASS and COLOUR SPACE select: output class + ink space -> \
             BlackPointUsingPerceptualBlack, chroma FORCED to 0 at L174; anything else -> \
             BlackPointAsDarkerColorant, chroma RETAINED. So lcms2 neutralises its black on a \
             CMYK press profile and keeps it on an RGB printer profile. STRUCTURAL on the \
             reimplementation's side and labelled so - what makes it evidence is that section B \
             predicts the REAL transicc's device output from this black and nothing else",
            format!(
                "{ctx} | chroma of the divergence = {:.6}, branch predicts {:.6}, residual \
                 {:.3e}. THE PRE-REGISTERED PREDICTION, PER ARM: on the swop arm claim 1 (the \
                 mechanism, 'ISO drops the chroma and lcms2 retains it') is FALSIFIED - lcms2 \
                 drops it too, by L174 - and claim 3 (the shape, 'the divergence IS the chroma') \
                 is FALSIFIED with the chroma term at exactly 0 and 100% of the divergence in L*. \
                 On the synthetic arm BOTH are CONFIRMED, with the divergence exactly 100% \
                 chroma and dL* exactly 0. Pass 5b reported 0.458924 for this quantity on the \
                 swop arm and that number was the RECOVERY, not lcms2. \
                 NOTE ON THE MAGNITUDE: the synthetic arm's 5.0 dE76 is the chroma this project \
                 AUTHORED into the fixture, so it is evidence for the MECHANISM and for nothing \
                 else - the prediction's 2-6 dE76 band is not tested by a fixture whose black \
                 this suite chose",
                a.divergence_chroma(),
                a.expected_divergence_chroma(),
                (a.divergence_chroma() - a.expected_divergence_chroma()).abs()
            ),
        )
        .with_separation(Separation::against(
            "the OTHER branch of cmsDetectBlackPoint (cmssamp.c L370-374): if the reimplementation \
             had dispatched on the wrong one of BlackPointUsingPerceptualBlack (chroma FORCED to 0 \
             at L174) and BlackPointAsDarkerColorant (chroma RETAINED), the predicted chroma would \
             be the darkest colorant's instead of 0, or 0 instead of it",
            (a.divergence_chroma() - a.other_branch_predicted_chroma()).abs(),
            (a.divergence_chroma() - a.expected_divergence_chroma()).abs(),
            SepUnits::SameAsMetric,
        )),
        Record::graded(
            format!("pass5c/{arm}/FINDING/neither-implementation-fits-a-quadratic-here"),
            Kind::CrossCheck,
            Metric::AbsMaxComponent,
            BRANCH_EXACT,
            if a.lcms2.nearly_straight && a.lcms2.n_shadow == 0 {
                0.0
            } else {
                1.0
            },
            "both estimators take the mid-range straightness short-circuit (cmssamp.c L521-545; \
             bpc.rs's 4.2.5.4 gate), so NEITHER fits a quadratic on this fixture. Pass 5b 17.3 \
             called this configuration 'precisely lcms2's method-4 (quadratic-fit) territory' - \
             it is not, and every Pass 5b statement about the shadow window or the root describes \
             code that did not run",
            format!(
                "{ctx} | nearlyStraight = {} on lcms2's ramp. THE DIVERGENCE IS THEREFORE \
                 ENTIRELY IN WHAT THE SHORT-CIRCUIT RETURNS - and since commit fd34a44 BOTH \
                 SIDES RETURN A QUANTITY THEY EACH CALL InitialLab, so the whole of it is that \
                 the two documents mean different things by that name. lcms2 (cmssamp.c L536) \
                 returns its InitialLab = BlackPointUsingPerceptualBlack, the PERCEPTUAL round \
                 trip A2B1(B2A0(Lab 0,0,0)) with chroma forced to 0 at L174, L*={:.6}. ISO/CD \
                 18619 4.2.5.4 returns its InitialLab = 4.2.2.2's darkest DEVICE VERTEX carried \
                 through A2B1 and neutralised by 4.2.3, L*={:.6}. NOT outRamp[first]/MinL \
                 ({:.6} on lcms2's ramp, {:.6} on ISO's) - that is what iccce returned here \
                 until fd34a44 and it has no textual support in 4.2.5 in any branch. \
                 SEPARATION OF THE TWO CANDIDATES: {:.6} L*",
                a.lcms2.nearly_straight,
                a.lcms2.initial_lab.l,
                a.iso_black.l,
                a.lcms2.min_l_lcms2,
                a.lcms2.min_l_iso,
                a.divergence_lightness(),
            ),
        )
        // `against_distance` with a stated 1.0, not `against`: this is a 0/1
        // INDICATOR, so the two candidate observations are 0 and 1 and their
        // distance is 1 whichever one this build produces. Deriving it as
        // |observed − alt| would print ZERO-SEPARATION on exactly the run where
        // the indicator fired — see `Separation::against`'s doc comment for the
        // measured instance of that.
        .with_separation(Separation::against_distance(
            "the quadratic-fit path: Pass 5b section 17.3 asserted this configuration was \
             'precisely lcms2's method-4 (quadratic-fit) territory', which is indicator 1 - a \
             build in which either side stopped short-circuiting would land there",
            1.0,
            1.0,
            SepUnits::SameAsMetric,
        )),
        Record::graded(
            format!("pass5c/{arm}/estimators/black-points-in-lab"),
            Kind::CrossCheck,
            Metric::AbsMaxComponent,
            REPORTED,
            a.estimator_divergence_de76(),
            "REPORTED: the two estimators' black points, with lcms2's REIMPLEMENTED from \
             cmssamp.c at pin 21c582a rather than recovered from its output. This supersedes \
             Pass 5b row Q2's 0.8582 dE76, which was 95% apparatus. \
             ** RE-MEASURED 2026-08-12 ON THE CORRECTED 4.2.5.4 CODE (commit fd34a44, harness \
             at cc03f3d) AND THE swop FIGURE DID NOT COLLAPSE - IT GREW 58.8x, from 8.1668e-2 \
             to 4.799109 dE76, still 100% L*. ** The collapse was predicted because the \
             pre-correction gap had been attributed in full to the 4.2.5.4 defect. That \
             attribution was right about the CAUSE and wrong about the CONSEQUENCE: iccce's \
             non-conformant return value (outRamp[first] = MinL = 16.489806) sat 0.0817 L* from \
             lcms2's answer, and the CONFORMANT return value (InitialLab = 11.772365) sits \
             4.7991 L* from it. The defect's own magnitude - how far the old code was from the \
             new - is 4.717441 L*, i.e. 57.8x the divergence it was blamed for; it was very \
             nearly INVISIBLE in the cross-check. (THAT DISTANCE IS NOW THIS ROW'S EMITTED \
             CANDIDATE SEPARATION - the figures in this sentence are the dated 2026-08-12 \
             measurement, the field is the live one, and if they ever disagree the field is \
             right.) AGREEMENT WITH THE ORACLE WAS THE SYMPTOM OF \
             OUR DEFECT, AND CONFORMING TO THE CLAUSE MADE THE CROSS-CHECK WORSE. Both sides \
             now return a quantity their own document calls InitialLab; the entire remaining \
             divergence is that ISO/CD 18619 4.2.2.2 and lcms2's cmsDetectBlackPoint mean \
             different things by that name. This is a cross-check, NOT ground truth: no \
             published value exists for either black point. \
             WHERE THIS QUESTION'S POWER ACTUALLY LIVES, since this row is REPORTED and its \
             separation therefore reads UNGRADED: row \
             CLAUSE/4.2.5.4-returns-InitialLab-not-outRamp-first on the `floored` arm, which \
             grades the same regression against an AUTHORED constant at half a PCSLAB quantum and \
             was shown to fail under an injected reversion while everything else stayed green. \
             This row stays REPORTED deliberately - no clause requires two implementations of two \
             different documents to agree, so any bound here would be one fitted to the single \
             defect it was meant to catch, and would have to be three different constants for the \
             three arms. See _WHY_THE_ESTIMATOR_ROW_IS_NOT_GRADED in this file for the argument in \
             full",
            format!(
                "{ctx} | ISO L*={:.6} vs lcms2 L*={:.6}: dL*={:.6}, chroma term {:.3e}, total \
                 {:.6} dE76. Pass 5b row Q2 reported 0.858170 for this quantity - 10.5x larger - \
                 because it measured BT(black) rather than black",
                a.iso_black.l,
                a.lcms2.black.l,
                a.divergence_lightness(),
                a.divergence_chroma(),
                a.estimator_divergence_de76(),
            ),
        )
        // ★★★ THE ROW DL-033 IS ABOUT. The alternative is not hypothetical: it
        // is what `bpc.rs` returned at 4.2.5.4 until commit fd34a44, and the
        // suite reported its consequence (8.1668e-2) as this row's observed
        // value for two sessions. `outRamp[first]` is not a black-point
        // candidate in any branch of 4.2.5 — that is exactly why the distance
        // to it is the measure of how nearly this cross-check missed.
        //
        // The distance is the ΔE76 between the two candidate BLACK POINTS
        // rather than the difference of the two observed ΔE76s. They coincide
        // here because both candidates are neutral and lcms2's is too, so the
        // three points are collinear in Lab; the geometric form is used because
        // it stays right when they are not.
        .with_separation(Separation::against_distance(
            "outRamp[first] (= MinL on ISO's index-0-included ramp), the quantity bpc.rs's 4.2.5.4 \
             short-circuit returned until commit fd34a44 and which has no textual support in any \
             branch of ISO/CD 18619 4.2.5",
            a.alt_estimator_divergence_de76(),
            a.black_candidate_separation_de76(),
            SepUnits::SameAsMetric,
        )),
        Record::graded(
            format!("pass5c/{arm}/validation/reimplementation-beats-the-rival-candidate"),
            Kind::CrossCheck,
            Metric::DeviceAbsMaxNormalised,
            DISCRIMINATES,
            if a.device_residual_iso() > 0.0 {
                a.device_residual_lcms2() / a.device_residual_iso()
            } else {
                f64::INFINITY
            },
            "the reimplemented black predicts the REAL transicc's CMYK at input black better than \
             the ISO black does. Without this row section B's absolute residual would be evidence \
             of nothing. The separation between the two candidates is printed in the context \
             field, because it MOVED (0.082 L* before commit fd34a44, 4.799 after) and a literal \
             here would now be false",
            format!(
                "{ctx} | residual under the lcms2 model {:.6e} vs under the ISO model {:.6e} = \
                 {:.4}, with the candidates {:.6} L* apart. BPC's second constraint sends the \
                 source black EXACTLY to the destination black (Pass 5 row P3, 3.33e-16) and \
                 this source's black is XYZ(0,0,0), so the CMYK an implementation emits at input \
                 black IS B2A1(its own detected black) - which is why a black point can be \
                 validated in device units with no round trip anywhere in the comparison",
                a.device_residual_lcms2(),
                a.device_residual_iso(),
                a.device_residual_lcms2() / a.device_residual_iso(),
                a.divergence_lightness(),
            ),
        )
        // The candidates ARE this row's subject, so the separation is real and
        // large — but it is in normalised device units while the row's metric
        // is a dimensionless ratio, so the blindness test must NOT be applied
        // to it. Stating that is the point of SepUnits::Other: the number is
        // information, the verdict would be arithmetic across incommensurable
        // units (Pass 6 row R5, and this module's own ATTRIBUTION row).
        .with_separation(Separation::against_distance(
            "the ISO black as the explanation of transicc's output - i.e. B2A1(ISO black) instead \
             of B2A1(reimplemented lcms2 black) at the input black",
            a.device_residual_iso(),
            a.device_candidate_separation(),
            SepUnits::Other("normalised device (0..1); this row's metric is a dimensionless ratio"),
        )),
        Record::graded(
            format!("pass5c/{arm}/validation/device-residual-against-transicc"),
            Kind::CrossCheck,
            Metric::DeviceAbsMaxNormalised,
            REPORTED,
            a.device_residual_lcms2(),
            "REPORTED, NOT GRADED: the absolute device residual between B2A1(reimplemented black) \
             and what transicc emits. It is NOT graded against Pass 4b A's 1.330e-4 envelope for \
             the same table, because that number is a maximum over Pass 4b's own point set and \
             this is one deep neutral shadow point outside it - Pass 6 row R4's lesson about \
             maxima over different populations",
            format!(
                "{ctx} | {:.6e} device, against Pass 4b A's 1.330e-4 maximum over ITS points. \
                 The residue is the pipeline difference, not the black point: lcms2 evaluates its \
                 round trip through the 16-bit machinery (cmsCreateExtendedTransform with \
                 NOOPTIMIZE) and this harness evaluates iccce's Lut16Model in f64",
                a.device_residual_lcms2()
            ),
        )
        .with_separation(Separation::against(
            "the residual under the ISO black - what this same quantity would be if the ISO \
             candidate were the explanation of transicc's output",
            a.device_residual_iso(),
            a.device_residual_lcms2(),
            SepUnits::SameAsMetric,
        )),
        Record::graded(
            format!("pass5c/{arm}/ATTRIBUTION/pass5b-recovery-was-the-round-trip"),
            Kind::SelfConsistency,
            Metric::AbsMaxComponent,
            // ★ GRADED ON THE SWOP ARM ONLY, and the reason is a units
            // mismatch rather than a convenience.
            //
            // The numerator is a full dE76 in Lab; the denominator is an
            // L*-only bound (a device residual divided by d(device)/d(L*)).
            // On the swop arm the estimators' divergence is 100 % L*, so the
            // two are commensurate and the ratio means what it says. On the
            // synthetic arm the divergence is 100 % CHROMA, and an L*-derived
            // ruler is the wrong instrument for it: both quantities land at
            // ~1.1e-3, the ratio comes out at 1.06, and that 6 % is
            // arithmetic about incommensurable units, not evidence about a
            // black point. Pass 6 row R5 recorded the same lesson - the unit
            // the requirement is stated in is the one that may carry the
            // tolerance.
            //
            // There is also nothing on the synthetic arm for the row to
            // attribute: Pass 5b only ever ran on USWebCoatedSWOP.
            if a.arm == "swop" {
                RECOVERY_EXPLAINED
            } else {
                REPORTED
            },
            if a.l_star_bound() > 0.0 {
                a.recovery_explained() / a.l_star_bound()
            } else {
                f64::INFINITY
            },
            "Pass 5b's recovered black is BT(the reimplemented black) to within this section's \
             own error bar. That is the attribution row: 0.858 dE76 of 'estimator divergence' was \
             0.082 of estimator and the rest of round trip, and it is now shown rather than \
             suspected. BOTH SIDES OF THIS ROW ARE lcms2's, so commit fd34a44's correction to \
             iccce's 4.2.5.4 does not touch it and the decomposition stands as measured. The \
             0.082 in it is the estimator divergence AS ICCCE THEN COMPUTED IT, before that \
             correction; the conformant divergence is on row estimators/black-points-in-lab and \
             is 59x larger, and it is NOT a term in Pass 5b's 0.858",
            format!(
                "{ctx} | Pass 5b recovered Lab({:.6} {:.6} {:.6}); BT(reimplemented black) = \
                 Lab({:.6} {:.6} {:.6}); unexplained {:.6} dE76 against an L* bound of {:.6} = \
                 {:.4}. Pass 5b row Q1 measured this same round-trip residual at 0.8137 dE76 and \
                 called it an error bar; it was not an error bar, it was the measurement",
                a.pass5b_recovered.l,
                a.pass5b_recovered.a,
                a.pass5b_recovered.b,
                a.roundtrip_of_reimpl.l,
                a.roundtrip_of_reimpl.a,
                a.roundtrip_of_reimpl.b,
                a.recovery_explained(),
                a.l_star_bound(),
                a.recovery_explained() / a.l_star_bound(),
            ),
        )
        .with_separation(Separation::none(
            "BOTH SIDES OF THIS ROW ARE lcms2's - Pass 5b's recovered black and BT(the \
             reimplemented lcms2 black) - so there is no rival READING for either term. The one \
             thing that could have been named, 'Pass 5b's recovery WAS lcms2's black rather than \
             its round trip', is not an alternative value this row could have observed: it is the \
             hypothesis the row exists to test, and it is what the ratio itself reports",
        )),
    ];

    // The shipped surface. Pass 5b row Q8 graded a refusal; the estimator has
    // since been wired, so the refusal is gone and the row is superseded.
    match (&a.shipped_device, &a.shipped_error) {
        (Some(dev), _) => out.push(Record::graded(
            format!("pass5c/{arm}/shipped/binary-reaches-the-iso-estimator"),
            Kind::SelfConsistency,
            Metric::DeviceAbsMaxNormalised,
            SHIPPED_MATCHES_LIBRARY,
            max_abs(dev, &a.predicted_from_iso),
            "SUPERSEDES Pass 5b row Q8. That row graded `iccce transform --bpc` REFUSING this \
             exact case, because bpc::estimate_lut_destination_black had no caller. It has since \
             been wired into Chain::estimate_dst_black, the binary converts, and what is worth \
             grading now is that it reaches the SAME black point the library function does - a \
             wiring that passed a different InitialLab, or the perceptual BT instead of the \
             relative one, would still convert and still look plausible",
            format!(
                "{ctx} | shipped {:?} vs B2A1(ISO black in process) {:?} -> {:.3e}",
                dev,
                a.predicted_from_iso,
                max_abs(dev, &a.predicted_from_iso)
            ),
        )
        // The wiring hazard this row exists to catch, priced: a Chain that
        // reached lcms2's black instead of ISO's would still convert and still
        // look plausible, and this is how far its output would be. It replaces
        // SHIPPED_MATCHES_LIBRARY's typed "2.46e-3" with the computed value.
        .with_separation(Separation::against(
            "the shipped binary reaching lcms2's black instead of the ISO estimator's - a wiring \
             that passed a different InitialLab, or the perceptual BT instead of the relative one, \
             would still convert and still look plausible",
            max_abs(dev, &a.predicted_from_lcms2),
            max_abs(dev, &a.predicted_from_iso),
            SepUnits::SameAsMetric,
        ))),
        (None, Some(err)) => out.push(Record::graded(
            format!("pass5c/{arm}/shipped/binary-reaches-the-iso-estimator"),
            Kind::SelfConsistency,
            Metric::DeviceAbsMaxNormalised,
            SHIPPED_MATCHES_LIBRARY,
            f64::INFINITY,
            "the shipped binary was expected to CONVERT this case since the ISO estimator was \
             wired into Chain::estimate_dst_black. It refused",
            format!("{ctx} | binary said: {}", crate::sanitise(err)),
        )
        .with_separation(Separation::none(
            "the binary produced no value, so there is nothing for an alternative candidate to be \
             an alternative TO. The row is graded on the refusal itself",
        ))),
        (None, None) => {}
    }
    out
}

fn specs(arm: &str) -> Vec<(String, Kind, Metric, Tolerance)> {
    vec![
        (
            format!("pass5c/{arm}/apparatus/black-is-device-observable-as-declared"),
            Kind::SelfConsistency,
            Metric::AbsMaxComponent,
            OBSERVABILITY_AS_DECLARED,
        ),
        (
            format!("pass5c/{arm}/apparatus/error-bar-is-smaller-than-the-effect"),
            Kind::SelfConsistency,
            Metric::AbsMaxComponent,
            if declared_observable(arm) {
                APPARATUS_RATIO
            } else {
                APPARATUS_VOID_UNOBSERVABLE
            },
        ),
        (
            format!("pass5c/{arm}/FINDING/divergence-chroma-follows-lcms2-BRANCH"),
            Kind::CrossCheck,
            Metric::AbsMaxComponent,
            NEUTRAL_EXACT,
        ),
        (
            format!("pass5c/{arm}/FINDING/neither-implementation-fits-a-quadratic-here"),
            Kind::CrossCheck,
            Metric::AbsMaxComponent,
            BRANCH_EXACT,
        ),
        (
            format!("pass5c/{arm}/estimators/black-points-in-lab"),
            Kind::CrossCheck,
            Metric::AbsMaxComponent,
            REPORTED,
        ),
        (
            format!("pass5c/{arm}/validation/reimplementation-beats-the-rival-candidate"),
            Kind::CrossCheck,
            Metric::DeviceAbsMaxNormalised,
            DISCRIMINATES,
        ),
        (
            format!("pass5c/{arm}/validation/device-residual-against-transicc"),
            Kind::CrossCheck,
            Metric::DeviceAbsMaxNormalised,
            REPORTED,
        ),
        (
            format!("pass5c/{arm}/ATTRIBUTION/pass5b-recovery-was-the-round-trip"),
            Kind::SelfConsistency,
            Metric::AbsMaxComponent,
            RECOVERY_EXPLAINED,
        ),
        (
            format!("pass5c/{arm}/shipped/binary-reaches-the-iso-estimator"),
            Kind::SelfConsistency,
            Metric::DeviceAbsMaxNormalised,
            SHIPPED_MATCHES_LIBRARY,
        ),
    ]
}

#[must_use]
pub fn unavailable_records(arm: &str, u: &Unavailable) -> Vec<Record> {
    let reason = u.to_string();
    specs(arm)
        .into_iter()
        .map(|(id, kind, metric, tol)| {
            let id: &str = &id;
            let source = "lcms2 2.19.1 cmsDetectDestinationBlackPoint REIMPLEMENTED from source \
                          at pin 21c582a, against ISO/CD 18619 4.2.5";
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

/// Run Pass 5c on **both** arms.
///
/// ## Why two arms and not one
///
/// The two fixtures reach **opposite** verdicts on the same pre-registered
/// claim, and neither verdict is the answer on its own:
///
/// | arm | destination | lcms2's `InitialLab` branch | divergence |
/// |---|---|---|---|
/// | `swop` | `USWebCoatedSWOP.icc`, v2 CMYK `prtr` | `BlackPointUsingPerceptualBlack` - chroma FORCED to 0 | **all `L*`** |
/// | `synthetic` | `v4-rgb-mab-chromatic-black.icc`, v4 RGB `prtr` | `BlackPointAsDarkerColorant` - chroma RETAINED | **all chroma** |
///
/// A session that ran only the first would report "lcms2 neutralises its black
/// too, the prediction's mechanism is falsified". A session that ran only the
/// second would report "confirmed". **Both are true, of different profiles**,
/// and the discriminating variable is the destination's device class and
/// colour space — which is a fact about lcms2's dispatch and not about black
/// points at all.
///
/// ## The third arm, added 2026-08-12, and why it is a third fixture rather
/// than an edit to the second
///
/// | arm | destination | what only it can see |
/// |---|---|---|
/// | `swop` | `USWebCoatedSWOP.icc`, v2 CMYK `prtr` — **category (c)** | the `BlackPointUsingPerceptualBlack` branch on a real ink set |
/// | `synthetic` | `v4-rgb-mab-chromatic-black.icc` | the `BlackPointAsDarkerColorant` branch, i.e. lcms2 **retaining** a black's chroma |
/// | `floored` | `v4-rgb-mab-floored-b2a.icc` | **4.2.5.4's two candidate return values, as two different numbers** |
///
/// The second and third differ in exactly one structural property — the third's
/// `B2A` floors `G` for every input, so the round trip cannot reach the darkest
/// vertex — plus a deliberate change of every shared constant so that a figure
/// quoted without its arm is obviously wrong rather than plausibly right.
/// Regenerating the second instead would have moved `NUMERIC_CLAIMS.md` NC-166's
/// companion figure and several statements that are true of *those* bytes.
///
/// The synthetic arms **skip** rather than error when a fixture is absent, like
/// every other row in this suite that needs a file; they are committed, so
/// absence means a checkout without `fixtures/`.
///
/// ★ §C's rows are emitted **outside** `analyse` — see [`clause_records`] —
/// because they need no oracle, no system profile and no shipped binary, and a
/// derived expectation must not be hostage to any of the three.
#[must_use]
pub fn run(oracle: &crate::Oracle) -> (Option<Analysis>, Vec<Record>) {
    let mut records_out = Vec::new();
    let mut first = None;
    for (arm, path) in [
        ("swop", PathBuf::from(SWOP)),
        ("synthetic", synthetic_fixture()),
        ("floored", floored_fixture()),
    ] {
        match analyse(oracle, arm, &path) {
            Ok(a) => {
                records_out.extend(records(&a));
                if first.is_none() {
                    first = Some(a);
                }
            }
            Err(u) => records_out.extend(unavailable_records(arm, &u)),
        }
        // §C, unconditionally, for every arm whose fixture this project
        // authored. `swop` has no authored constant and therefore no derived
        // expectation available; that is stated by its absence from `AUTHORED`
        // rather than by a weaker row wearing the same name.
        if let Some((_, authored)) = AUTHORED.iter().find(|(a, _)| *a == arm) {
            let fixture_path = match arm {
                "synthetic" => synthetic_fixture(),
                _ => floored_fixture(),
            };
            records_out.extend(clause_records(arm, &fixture_path, *authored));
        }
    }
    (first, records_out)
}

/// `fixtures/synthetic/v4-rgb-mab-chromatic-black.icc`, resolved from the
/// crate root so the suite does not depend on the working directory.
///
/// `LEGAL.md` §3 category **(a)**: authored by this project byte by byte,
/// regenerable by `gen-profiles`, committed, and carrying its own provenance
/// in its `cprt` tag.
#[must_use]
pub fn synthetic_fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/synthetic/v4-rgb-mab-chromatic-black.icc")
}

/// `fixtures/synthetic/v4-rgb-mab-floored-b2a.icc` — the third arm's fixture,
/// same category (a), added 2026-08-12. See [`AUTHORED`] for what it is for.
#[must_use]
pub fn floored_fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/synthetic/v4-rgb-mab-floored-b2a.icc")
}

// ===========================================================================
// §C — ISO/CD 18619 4.2.5.4 graded against an AUTHORED constant, with no
//      oracle, no system profile and no network in the loop
// ===========================================================================

/// What a **synthetic** fixture's generator authored into its bytes, transcribed
/// here so that an expectation exists which did not come from any
/// implementation.
///
/// ## Why this struct is literals and why that is safe
///
/// `TOLERANCES.md` §3.5.8.6 records four claim-bearing literals in this crate
/// that went false inside a day, and the rule that came out of it — *interpolate,
/// never type*. These numbers are the exception, and the exception has a
/// mechanism behind it rather than a preference:
///
/// * they are not **measurements**, they are the fixture's *design*, fixed in
///   `tools/gen-profiles/src/recipes.rs` as named constants;
/// * `gen-profiles verify` proves the committed bytes are the ones that
///   generator produces, byte for byte, so the only way for these to drift is
///   for somebody to change the recipe **and** regenerate — at which point this
///   row fails loudly, which is the behaviour wanted;
/// * a value computed from the fixture would be an expectation derived from the
///   thing under test, which is precisely what `CLAUDE.md` rule 3 forbids.
///
/// So the direction of the coupling matters: a *measured* number typed into
/// prose rots silently; a *design* number typed into an assertion fails loudly.
#[derive(Debug, Clone, Copy)]
pub struct Authored {
    /// The recipe that produced the bytes, for the failure message.
    pub recipe: &'static str,
    /// `A2B1` at the darkest device vertex, as the generator wrote it —
    /// **before** ISO/CD 18619 4.2.3 neutralises it.
    pub black: Lab,
    /// The `L*` floor the round trip `A2B1(B2A1(·))` cannot go below, as
    /// authored: `outRamp[first]`, the rival return value of 4.2.5.4.
    pub roundtrip_floor_l: f64,
}

impl Authored {
    /// ISO/CD 18619 **4.2.3**: the initial black point is *always* neutral,
    /// `L*` clipped at 50. Applied here to the authored device black, so the
    /// expectation is `clause(authored bytes)` with no implementation in it.
    #[must_use]
    pub fn initial_lab(&self) -> Lab {
        Lab {
            l: if self.black.l > 50.0 {
                50.0
            } else {
                self.black.l
            },
            a: 0.0,
            b: 0.0,
        }
    }
    /// The separation the fixture was **designed** to have between 4.2.5.4's two
    /// candidate return values, in `L*`.
    #[must_use]
    pub fn designed_separation_l(&self) -> f64 {
        (self.roundtrip_floor_l - self.initial_lab().l).abs()
    }
}

/// The two synthetic arms' authored constants, transcribed from
/// `tools/gen-profiles/src/recipes.rs`.
///
/// ★ **The pair is the point.** The same two rows run on both fixtures and
/// reach opposite verdicts about their own power: the sibling's candidates are
/// the same number (`ZERO-SEPARATION`, and it would stay green through a full
/// reversion of `fd34a44`), the floored one's are `25 L*` apart. A reader who
/// sees only the second learns nothing about what a fixture has to do to be an
/// instrument; a reader who sees both learns it from one screen.
pub const AUTHORED: &[(&str, Authored)] = &[
    (
        "synthetic",
        Authored {
            recipe: "v4-rgb-mab-chromatic-black",
            // SYNTH_BLACK_L / _A / _B in recipes.rs.
            black: Lab {
                l: 20.0,
                a: 4.0,
                b: -3.0,
            },
            // ★ EQUAL to the neutralised black's L*, and that is the finding
            // GP-002 records: the model is affine, the B2A is its exact
            // inverse, and the black IS the darkest vertex, so the round trip's
            // floor cannot be anywhere else.
            roundtrip_floor_l: 20.0,
        },
    ),
    (
        "floored",
        Authored {
            recipe: "v4-rgb-mab-floored-b2a",
            // FLOORED_BLACK_L / _A / _B in recipes.rs.
            black: Lab {
                l: 12.5,
                a: 6.0,
                b: -8.0,
            },
            // FLOORED_ROUNDTRIP_L — 25.0 above the black, by construction.
            roundtrip_floor_l: 37.5,
        },
    ),
];

/// The general 16-bit PCSLAB `L*` quantum of ICC.1:2022 **6.3.4.2**:
/// `L* = 100 × value / 65 535`, so one code is `1,525 9×10⁻³` `L*`.
const PCSLAB_L_QUANTUM: f64 = 100.0 / 65535.0;

/// **§C, the clause.** `estimate_lut_destination_black` on a fixture whose
/// darkest vertex the generator authored, against that authored constant
/// neutralised by ISO/CD 18619 4.2.3.
///
/// ## What makes this a [`Kind::DerivedExpectation`] and not a cross-check
///
/// The expected value is `4.2.3(the number in recipes.rs)`. No implementation
/// produced it, lcms2 is not consulted, and neither is any system profile — the
/// row runs on a bare checkout with a fixture and nothing else. That is the
/// property the row exists for; see [`clause_records`].
///
/// ## The tolerance, derived
///
/// `A2B1` at device `(0,0,0)` is a **CLUT corner**: the `A` curves are the
/// identity, the lookup lands on node `(0,0,0)`, and no interpolation happens.
/// So the only departure from the authored constant is the generator's own
/// round-to-nearest when it encoded that constant into 6.3.4.2's general 16-bit
/// PCSLAB form — **half a quantum, `0,5 × 100/65 535 = 7,629×10⁻⁴`, and nothing
/// else.** There is no free parameter: no interpolation term because there is no
/// interpolation, no oracle term because there is no oracle, and the chroma
/// terms are exactly zero because 4.2.3 assigns `0` literally on one side and
/// the generator wrote `0` on the other.
///
/// It cannot absorb the rival return value; **how far it is from being able to
/// is this row's emitted candidate separation**, not a number in this sentence
/// (§3.5.8.6).
pub const CLAUSE_4254: Tolerance = Tolerance::new(
    0.5 * PCSLAB_L_QUANTUM,
    "HALF ONE PCSLAB L* QUANTUM, and nothing else. The expectation is the fixture's AUTHORED device \
     black (a named constant in tools/gen-profiles/src/recipes.rs) put through ISO/CD 18619 4.2.3, \
     so no implementation's output is in it. A2B1 at device (0,0,0) is a CLUT CORNER read through \
     identity curves - no interpolation happens - so the only departure available is the \
     generator's round-to-nearest into the general 16-bit PCSLAB encoding of ICC.1:2022 6.3.4.2, \
     whose quantum is 100/65535 = 1.5259e-3 L*. Half of that is 7.6294e-4. The chroma terms are \
     EXACTLY zero on both sides: 4.2.3 assigns neutral literally and the estimator carries \
     InitialLab through without arithmetic. There is no free parameter in this number and no room \
     to widen it without saying which of the three sentences above is false",
);

/// **§C, the fixture's own power.** The separation this fixture actually has
/// between 4.2.5.4's two candidate return values, against the separation its
/// recipe was **designed** to give it.
///
/// ## Why a fixture needs a graded row of its own
///
/// GP-002 is the whole reason: `v4-rgb-mab-chromatic-black`'s two candidates
/// collapsed to one number, not by an authoring mistake but as a *consequence*
/// of three properties each chosen for a good reason. A future edit to
/// `v4-rgb-mab-floored-b2a` — removing the floor as "an odd special case",
/// making the `B2A` a clean inverse again, moving a constant to match the
/// sibling — would re-collapse it in exactly the same way, and every row above
/// would stay green while the suite silently stopped being able to see the
/// defect it was built for. **The separation mechanism can report that a row is
/// blind; only a graded row can stop it becoming blind.**
///
/// ## The tolerance, derived
///
/// Three named half-quanta, one for each encoding the number passes through:
///
/// 1. `InitialLab`'s own encode into general PCSLAB `L*` — `0,5 × 100/65 535`;
/// 2. the round-trip floor's, which is read out of the same encoding (from two
///    interpolated `A2B` nodes, each within half a quantum, so their convex
///    combination is too) — `0,5 × 100/65 535`;
/// 3. the `B2A`'s stored `G` floor, a `u16` device code whose half-quantum is
///    `0,5/65 535` in `G`; converting to `L*` needs the model's `dL*/dG`, and
///    the bound uses **100** — the whole lightness range — rather than the
///    fixture's own `87,5`, because a fixture-specific slope in a tolerance is
///    a thing that goes stale when a constant moves.
///
/// Sum: `1,5 × 100/65 535 = 2,289×10⁻³`. Conservative in the only direction
/// that matters — the direction that can only make the row harder to pass.
pub const FIXTURE_SEPARATION_AS_DESIGNED: Tolerance = Tolerance::new(
    1.5 * PCSLAB_L_QUANTUM,
    "the separation the fixture HAS between 4.2.5.4's two candidate return values, against the \
     separation its recipe was DESIGNED to give it. THREE named half-quanta, one per encoding the \
     number passes through: InitialLab's own encode into general PCSLAB L* (0.5 x 100/65535), the \
     round-trip floor's read back out of two interpolated A2B nodes each within half a quantum \
     (0.5 x 100/65535), and the B2A's stored u16 G floor converted to L* through the model's \
     dL*/dG - bounded by 100, the WHOLE lightness range, rather than this fixture's own 87.5, \
     because a fixture-specific slope inside a tolerance goes stale when a constant moves. Sum \
     1.5 x 100/65535 = 2.2888e-3. THIS ROW EXISTS BECAUSE THE SEPARATION MECHANISM CAN REPORT THAT \
     A ROW IS BLIND BUT CANNOT STOP IT BECOMING BLIND: GP-002 collapsed the sibling fixture's two \
     candidates as a CONSEQUENCE of three separately reasonable properties, and the same edit here \
     - dropping the floor, restoring a clean inverse, matching the sibling's constants - would \
     silently remove this arm's whole power while every other row stayed green",
);

/// ISO/CD 18619 4.2.5.2.2 + 4.2.5.2.3, reimplemented here for one purpose:
/// to state `outRamp[first]` — **the rival return value** — independently of
/// the code being graded.
///
/// It is a deliberate duplication of twelve lines of `iccce_cmm::bpc`. Calling
/// that crate for the rival would mean a build in which the estimator is broken
/// reports its own broken rival, and the separation would move in lockstep with
/// the observation it is supposed to bound. A harness must be able to say how
/// far away the wrong answer is **while the library is giving the wrong
/// answer**; that is the entire use of the number.
#[must_use]
pub fn iso_out_ramp_first(initial_lab: Lab, bt: impl Fn(Lab) -> Lab) -> f64 {
    const N: usize = 256;
    let ka = initial_lab.a.clamp(-50.0, 50.0);
    let kb = initial_lab.b.clamp(-50.0, 50.0);
    let mut out = [0.0f64; N];
    for (i, slot) in out.iter_mut().enumerate() {
        #[allow(clippy::cast_precision_loss)]
        let t = i as f64 / (N - 1) as f64;
        *slot = bt(Lab {
            l: t * 100.0,
            a: ka * (1.0 - t),
            b: kb * (1.0 - t),
        })
        .l;
    }
    // 4.2.5.2.3's downward monotonic pass, index 0 INCLUDED (lcms2's own loop
    // stops at 1; that difference is measured by §A's `min_l_lcms2`).
    for i in (0..N - 1).rev() {
        out[i] = out[i].min(out[i + 1]);
    }
    out[0]
}

/// §C's two records, for one synthetic arm.
///
/// ## ★★ Why this does not go through [`analyse`]
///
/// `analyse` needs three things this does not: the **system sRGB profile**
/// (`LEGAL.md` §3 category (c) — absent on any machine without the Windows
/// colour directory), the **`transicc` oracle** (a vendored build, not
/// committed), and the **shipped `iccce` binary** (a release build). Any one of
/// them missing skips the whole arm, which is correct for a cross-check and
/// wrong for this.
///
/// These two rows are `DerivedExpectation`: their expectation is a constant in
/// `recipes.rs` and a clause in ISO/CD 18619, and **a ground-truth row must not
/// be hostage to an oracle**. So they take a separate path that needs the
/// committed fixture and nothing else, and they are the rows that make a
/// 4.2.5.4 regression visible on a bare checkout.
///
/// The measured position that made this necessary, recorded here because it is
/// the reason the function exists: as of `e26d9ba`, **a full reversion of
/// `fd34a44` turned no row of this suite red on any machine.** `pass5c/swop/*`
/// moved its reported numbers and stayed green; `pass5c/synthetic/*` did not
/// move at all. (The clause was not undefended — `iccce_cmm::bpc`'s own unit
/// tests fail on that reversion — but nothing exercised it through a parsed
/// profile, which is where a wiring defect between `Chain` and the estimator
/// would live.)
#[must_use]
pub fn clause_records(arm: &'static str, path: &Path, authored: Authored) -> Vec<Record> {
    let source = format!(
        "ISO/CD 18619 4.2.3 + 4.2.5.4 applied to the AUTHORED constants of \
         tools/gen-profiles/src/recipes.rs recipe `{}`. No lcms2, no system profile, no oracle",
        authored.recipe
    );
    let f = match Fixture::open(path) {
        Ok(f) => f,
        Err(e) => {
            let reason = format!("{}: {e}", path.display());
            return clause_specs(arm)
                .into_iter()
                .map(|(id, kind, metric, tol)| {
                    Record::skipped(id, kind, metric, tol, source.clone(), reason.clone())
                })
                .collect();
        }
    };

    // The full ISO chain, as the library implements it: 4.2.2.2's vertex
    // search, 4.2.3's neutralise-and-clip, 4.2.5's estimate.
    let darkest = darkest_vertex(f.channels, |d| f.a2b1_lab(d));
    let darkest_lab = f.a2b1_lab(&darkest);
    let initial = neutralise_and_clip(darkest_lab.l);
    let returned = estimate_lut_destination_black(
        initial,
        EstimationIntent::RelativeColorimetric,
        |lab| f.bt_rel(lab),
    );
    // The rival, computed by this file rather than by the crate under test.
    let ramp_first = iso_out_ramp_first(initial, |lab| f.bt_rel(lab));

    let expected = authored.initial_lab();
    let observed = (returned.l - expected.l)
        .abs()
        .max((returned.a - expected.a).abs())
        .max((returned.b - expected.b).abs());
    // What this row would have observed had 4.2.5.4 returned outRamp[first] —
    // the value `bpc.rs` returned until commit fd34a44. Neutral, because 4.2.3
    // neutralises and the ramp carries no chroma of its own on either fixture.
    let alt_observed = (ramp_first - expected.l)
        .abs()
        .max(expected.a.abs())
        .max(expected.b.abs());

    let ctx = format!(
        "arm={arm} | fixture {} ({}) | AUTHORED device black Lab({:.4} {:.4} {:.4}) -> 4.2.3 \
         InitialLab Lab({:.6} {:.6} {:.6}) | MEASURED darkest vertex {darkest:?} -> Lab({:.6} \
         {:.6} {:.6}) -> InitialLab Lab({:.6} {:.6} {:.6}) | 4.2.5 RETURNED Lab({:.6} {:.6} \
         {:.6}) | outRamp[first] (the rival, computed IN THIS FILE) L*={:.6} | authored floor \
         L*={:.4}",
        f.describe,
        authored.recipe,
        authored.black.l,
        authored.black.a,
        authored.black.b,
        expected.l,
        expected.a,
        expected.b,
        darkest_lab.l,
        darkest_lab.a,
        darkest_lab.b,
        initial.l,
        initial.a,
        initial.b,
        returned.l,
        returned.a,
        returned.b,
        ramp_first,
        authored.roundtrip_floor_l,
    );

    vec![
        Record::graded(
            format!("pass5c/{arm}/CLAUSE/4.2.5.4-returns-InitialLab-not-outRamp-first"),
            Kind::DerivedExpectation,
            Metric::AbsMaxComponent,
            CLAUSE_4254,
            observed,
            "ISO/CD 18619 4.2.5.4 final paragraph, VERBATIM: 'If the mid range is nearly straight \
             then the DestinationBlackPoint shall be the same as InitialLab.' Graded against the \
             fixture's AUTHORED device black neutralised by 4.2.3 - a constant in recipes.rs and a \
             clause, with no implementation's output in the expectation and no oracle, no system \
             profile and no network in the loop. It runs on a bare checkout. iccce returned \
             outRamp[first] here until commit fd34a44 and NO ROW OF THIS SUITE WENT RED; this is \
             the row that would have",
            format!(
                "{ctx} | expected Lab({:.6} {:.6} {:.6}), returned Lab({:.6} {:.6} {:.6}), max \
                 component {:.6e} against a bound of {:.6e}",
                expected.l,
                expected.a,
                expected.b,
                returned.l,
                returned.a,
                returned.b,
                observed,
                CLAUSE_4254.value,
            ),
        )
        // ★★ `against_distance`, NOT `against`, and the reason was measured
        // rather than foreseen.
        //
        // `Separation::against` derives the distance as |observed −
        // alt_observed|. On this row the alternative is *the code returning the
        // other candidate*, so when the defect is actually present `observed`
        // BECOMES `alt_observed` and that derived distance is exactly zero. The
        // proof-of-power run on 2026-08-12 showed it: with the pre-fd34a44
        // behaviour injected, this row failed at 2.500019e1 — correctly — and
        // reported `ZERO-SEPARATION`, i.e. the mechanism disclaimed its own
        // power on the one run where it had just demonstrated it.
        //
        // The distance that means something here is a property of the FIXTURE,
        // not of the run: how far apart the two candidate black points are.
        // That is `|InitialLab − outRamp[first]|`, and it is 25 L* whichever one
        // the library returns.
        .with_separation(Separation::against_distance(
            "outRamp[first] - the floor of the monotonised round-trip ramp, which is what \
             bpc.rs's 4.2.5.4 short-circuit returned until commit fd34a44 and which has no \
             textual support in any branch of ISO/CD 18619 4.2.5. Computed by \
             pass5c::iso_out_ramp_first, NOT by the crate under test, so this number stays \
             right while the library is wrong",
            alt_observed,
            (ramp_first - expected.l).abs(),
            SepUnits::SameAsMetric,
        )),
        Record::graded(
            format!("pass5c/{arm}/FIXTURE/candidates-are-separated-as-designed"),
            Kind::DerivedExpectation,
            Metric::AbsMaxComponent,
            FIXTURE_SEPARATION_AS_DESIGNED,
            ((ramp_first - initial.l).abs() - authored.designed_separation_l()).abs(),
            "the fixture's OWN power, graded. How far apart 4.2.5.4's two candidate return values \
             actually are in these bytes, against how far apart the recipe was written to put \
             them. On `synthetic` the designed separation is ZERO and this row records that the \
             collapse is by construction (GP-002) rather than by accident; on `floored` it is \
             25.0 L* and this row is what stops a later 'simplification' of the recipe from \
             silently returning the arm to zero power while every other row stays green",
            format!(
                "{ctx} | measured separation {:.6} L*, designed {:.4} L*, difference {:.6e} \
                 against a bound of {:.6e}",
                (ramp_first - initial.l).abs(),
                authored.designed_separation_l(),
                ((ramp_first - initial.l).abs() - authored.designed_separation_l()).abs(),
                FIXTURE_SEPARATION_AS_DESIGNED.value,
            ),
        )
        .with_separation(Separation::none(
            "both sides are the fixture's own bytes measured against the recipe that wrote them. \
             There is no rival READING of a separation - the alternative to 'the fixture has the \
             power it was designed to have' is not another value this row could have observed, it \
             is the row FAILING, which is what it is for. The rival candidate that matters is \
             named on the CLAUSE row above, where it belongs",
        )),
    ]
}

fn clause_specs(arm: &str) -> Vec<(String, Kind, Metric, Tolerance)> {
    vec![
        (
            format!("pass5c/{arm}/CLAUSE/4.2.5.4-returns-InitialLab-not-outRamp-first"),
            Kind::DerivedExpectation,
            Metric::AbsMaxComponent,
            CLAUSE_4254,
        ),
        (
            format!("pass5c/{arm}/FIXTURE/candidates-are-separated-as-designed"),
            Kind::DerivedExpectation,
            Metric::AbsMaxComponent,
            FIXTURE_SEPARATION_AS_DESIGNED,
        ),
    ]
}

// ===========================================================================
// Tests
// ===========================================================================
//
// ★ These exist because of what commit `2835d23` established — the tests in
// `tools/` now gate CI — and because of what §C is FOR. The `CLAUSE` row runs
// in the difftest runner; these run in `cargo test`, on the same committed
// fixture, with no oracle and no system profile. Between them a 4.2.5.4
// regression has to get past two independent surfaces.
//
// They assert on OUTCOMES computed from the committed bytes, never on the shape
// of the code that computes them.

#[cfg(test)]
mod tests {
    use super::*;

    /// ★★ The fixture's whole purpose, asserted directly: on
    /// `v4-rgb-mab-floored-b2a`, ISO/CD 18619 4.2.5.4's two candidate return
    /// values must be **far apart**.
    ///
    /// If this ever fails, the fixture has been "simplified" and every row
    /// measured with it has silently lost its power — which is exactly how
    /// FINDING GP-002 happened to the sibling fixture, as a consequence of
    /// three separately reasonable properties rather than as a mistake.
    ///
    /// The margin asserted is `20 L*` against an authored `25 L*`: loose enough
    /// that encoding and interpolation cannot reach it, tight enough that no
    /// plausible collapse survives it. It is deliberately **not** the graded
    /// row's `2,289×10⁻³` — that row grades the separation against its design
    /// and belongs in the runner; this one asserts the property a maintainer
    /// could destroy, and a test that duplicated the tolerance would just be
    /// the same row twice.
    #[test]
    fn the_floored_fixture_separates_4254s_two_candidates() {
        let path = floored_fixture();
        assert!(
            path.is_file(),
            "the committed fixture is missing: {}",
            path.display()
        );
        let f = Fixture::open(&path).expect("the committed fixture parses");
        let darkest = darkest_vertex(f.channels, |d| f.a2b1_lab(d));
        let initial = neutralise_and_clip(f.a2b1_lab(&darkest).l);
        let ramp_first = iso_out_ramp_first(initial, |lab| f.bt_rel(lab));
        assert!(
            (ramp_first - initial.l).abs() > 20.0,
            "v4-rgb-mab-floored-b2a exists SO THAT InitialLab and outRamp[first] are different \
             numbers; they are now L*={} and L*={}, {} apart. If the B2A's G floor has been \
             removed or the constants moved, pass5c's CLAUSE row has lost its power and no other \
             row will say so",
            initial.l,
            ramp_first,
            (ramp_first - initial.l).abs()
        );
    }

    /// ISO/CD 18619 **4.2.5.4** through a parsed profile: *"the
    /// DestinationBlackPoint shall be the same as InitialLab"*.
    ///
    /// `iccce_cmm::bpc`'s own unit tests already assert this on a synthetic
    /// closure and they are what caught the injected reversion on 2026-08-12.
    /// What they cannot reach is the clause exercised through the **parse → LUT
    /// model → estimator** path, which is where a wiring defect lives — and on
    /// a fixture where returning the wrong candidate is a **25 `L*`** error
    /// rather than a no-op.
    ///
    /// The expectation is the AUTHORED constant from `recipes.rs`, not a value
    /// read back from the fixture, so nothing in the chain under test supplies
    /// its own expectation. Bound: half one general-PCSLAB `L*` quantum, the
    /// same derivation as [`CLAUSE_4254`], because `A2B1(0,0,0)` is a CLUT
    /// corner and no interpolation happens.
    #[test]
    fn clause_4254_returns_initial_lab_through_a_parsed_profile() {
        for (arm, authored) in AUTHORED {
            let path = match *arm {
                "synthetic" => synthetic_fixture(),
                _ => floored_fixture(),
            };
            if !path.is_file() {
                eprintln!("SKIP {arm}: {} absent", path.display());
                continue;
            }
            let f = Fixture::open(&path).expect("committed fixture parses");
            let darkest = darkest_vertex(f.channels, |d| f.a2b1_lab(d));
            let initial = neutralise_and_clip(f.a2b1_lab(&darkest).l);
            let got = estimate_lut_destination_black(
                initial,
                EstimationIntent::RelativeColorimetric,
                |lab| f.bt_rel(lab),
            );
            let want = authored.initial_lab();
            let err = (got.l - want.l)
                .abs()
                .max((got.a - want.a).abs())
                .max((got.b - want.b).abs());
            assert!(
                err <= CLAUSE_4254.value,
                "arm {arm} ({}): 4.2.5.4 must return InitialLab. Expected the AUTHORED black \
                 neutralised by 4.2.3, Lab({} {} {}); got Lab({} {} {}); max component {err:e} \
                 against a bound of {:e}. The rival return value outRamp[first] would land at \
                 L*={}",
                authored.recipe,
                want.l,
                want.a,
                want.b,
                got.l,
                got.a,
                got.b,
                CLAUSE_4254.value,
                iso_out_ramp_first(initial, |lab| f.bt_rel(lab)),
            );
        }
    }

    /// Every arm named in [`DEVICE_OBSERVABLE`] must correspond to an arm the
    /// runner actually drives, and vice versa.
    ///
    /// A declaration table whose keys have drifted from the arm names is worse
    /// than no table: [`declared_observable`] treats an unknown arm as
    /// observable — the safe default for a *new* arm — so a renamed arm would
    /// silently reacquire a graded row whose conversion does not exist, and
    /// nothing downstream would say so.
    #[test]
    fn the_observability_table_covers_exactly_the_arms_that_exist() {
        let arms = ["swop", "synthetic", "floored"];
        for (a, _) in DEVICE_OBSERVABLE {
            assert!(
                arms.contains(a),
                "DEVICE_OBSERVABLE names an arm `{a}` that run() does not drive"
            );
        }
        for a in arms {
            assert!(
                DEVICE_OBSERVABLE.iter().any(|(k, _)| *k == a),
                "run() drives arm `{a}` with no line in DEVICE_OBSERVABLE; it would default to \
                 observable, which is right for a new arm and wrong for a renamed one"
            );
        }
    }
}
