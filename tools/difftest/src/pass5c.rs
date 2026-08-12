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
//! | straight-midrange return | `outRamp[first]`, clamped `[0,50]` | **`InitialLab`** (L536), i.e. a value from a *different* round trip |
//! | minimum shadow points | `xs.len() < 3` → give up | caller checks `n < 3`, but `RootOfLeastSquaresFitQuadraticCurve` **returns 0 for `n < 4`** (L317) |
//! | near-linear fallback | `b == 0.0` → 0 | `fabs(b) < 1.0E-10` → 0 |
//! | singular normal equations | rejected on the determinant of the Gaussian solve | `_cmsMAT3inverse` rejects on `fabs(det) < 1.0E-4` (`lcms2_internal.h` L142) |
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

use crate::{Kind, Metric, Record, Tolerance};

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
    /// Did the mid-range straightness test fire? If it did, lcms2 returns
    /// `InitialLab` and ISO returns `outRamp[first]` — a divergence with a
    /// completely different mechanism from the fit.
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
        // returns outRamp[first].
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
/// this profile's darkest colorant with its chroma intact — **0,834** — which
/// is four orders above anything a rounding argument could excuse.
pub const NEUTRAL_EXACT: Tolerance = Tolerance::new(
    0.0,
    "the chroma of the divergence MINUS what lcms2's selected branch says it must be, graded at \
     EXACTLY zero because both quantities are assigned literally rather than computed: ISO/CD \
     18619 4.2.3 returns a neutral black, and lcms2 returns InitialLab.a/.b verbatim (L590-591) \
     from a branch chosen by the destination's DEVICE CLASS and COLOUR SPACE. Not an epsilon: \
     taking the OTHER branch changes this by the darkest colorant's whole chroma - 0.834 on \
     USWebCoatedSWOP, 5.0 on the synthetic RGB fixture - which no rounding argument reaches. \
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
/// residual**. The two candidate black points here are only 0,082 `L*` apart,
/// so this row is what establishes that §B can tell them apart at all.
///
/// Tolerance **1,0**, no free parameter: below 1 the lcms2 model is the better
/// explanation of lcms2's own output; at or above 1 §B has no resolving power
/// and must be read as such.
pub const DISCRIMINATES: Tolerance = Tolerance::new(
    1.0,
    "the device residual against transicc under the REIMPLEMENTED lcms2 black, divided by the \
     residual under the ISO black. A reimplementation that predicted lcms2's own output no better \
     than the rival candidate did would be evidence of nothing however small its absolute \
     residual, and the two candidates are only 0.082 L* apart. Below 1 the lcms2 model is the \
     better explanation; at or above 1 this section has no resolving power",
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
/// different black point — the two candidates in play here are `2,46×10⁻³`
/// apart in this same quantity, three orders above the bound.
pub const SHIPPED_MATCHES_LIBRARY: Tolerance = Tolerance::new(
    1e-6,
    "the shipped `iccce transform --bpc` against B2A1(ISO black) computed in process. The CLI \
     prints device values to six decimals, so one printed lsb is 1e-6 and the bound is that and \
     nothing else. It CANNOT absorb a different black point: the two candidate blacks in play are \
     2.46e-3 apart in this same quantity, three orders above the bound. SUPERSEDES Pass 5b row \
     Q8, whose premise (the estimator has no caller) no longer holds",
);

pub const REPORTED: Tolerance = Tolerance::new(
    f64::INFINITY,
    "REPORTED, NOT GRADED - recorded so the number is on file next to the ones that are graded",
);

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
    let mut out = vec![
        Record::graded(
            format!("pass5c/{arm}/apparatus/error-bar-is-smaller-than-the-effect"),
            Kind::SelfConsistency,
            Metric::AbsMaxComponent,
            APPARATUS_RATIO,
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
        ),
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
        ),
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
                "{ctx} | nearlyStraight = {} on lcms2's ramp; ISO returned exactly outRamp[first] \
                 ({:.6} against MinL {:.6}). THE DIVERGENCE IS THEREFORE ENTIRELY IN WHAT THE \
                 SHORT-CIRCUIT RETURNS: lcms2 returns InitialLab (L536), a value from the \
                 PERCEPTUAL round trip A2B1(B2A0(Lab 0,0,0)) = {:.6}; ISO returns outRamp[first], \
                 a value from the RELATIVE round trip A2B1(B2A1(Lab 0,0,0)) = {:.6}. Two \
                 different tables, one short-circuit",
                a.lcms2.nearly_straight,
                a.iso_black.l,
                a.lcms2.min_l_iso,
                a.lcms2.initial_lab.l,
                a.lcms2.min_l_lcms2,
            ),
        ),
        Record::graded(
            format!("pass5c/{arm}/estimators/black-points-in-lab"),
            Kind::CrossCheck,
            Metric::AbsMaxComponent,
            REPORTED,
            a.estimator_divergence_de76(),
            "REPORTED: the two estimators' black points, with lcms2's REIMPLEMENTED from \
             cmssamp.c at pin 21c582a rather than recovered from its output. This supersedes \
             Pass 5b row Q2's 0.8582 dE76, which was 95% apparatus",
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
        ),
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
             of nothing: the two candidates are only 0.082 L* apart",
            format!(
                "{ctx} | residual under the lcms2 model {:.6e} vs under the ISO model {:.6e} = \
                 {:.4}. BPC's second constraint sends the source black EXACTLY to the destination \
                 black (Pass 5 row P3, 3.33e-16) and this source's black is XYZ(0,0,0), so the \
                 CMYK an implementation emits at input black IS B2A1(its own detected black) - \
                 which is why a black point can be validated in device units with no round trip \
                 anywhere in the comparison",
                a.device_residual_lcms2(),
                a.device_residual_iso(),
                a.device_residual_lcms2() / a.device_residual_iso(),
            ),
        ),
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
        ),
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
             suspected",
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
        ),
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
        )),
        (None, Some(err)) => out.push(Record::graded(
            format!("pass5c/{arm}/shipped/binary-reaches-the-iso-estimator"),
            Kind::SelfConsistency,
            Metric::DeviceAbsMaxNormalised,
            SHIPPED_MATCHES_LIBRARY,
            f64::INFINITY,
            "the shipped binary was expected to CONVERT this case since the ISO estimator was \
             wired into Chain::estimate_dst_black. It refused",
            format!("{ctx} | binary said: {}", crate::sanitise(err)),
        )),
        (None, None) => {}
    }
    out
}

fn specs(arm: &str) -> Vec<(String, Kind, Metric, Tolerance)> {
    vec![
        (
            format!("pass5c/{arm}/apparatus/error-bar-is-smaller-than-the-effect"),
            Kind::SelfConsistency,
            Metric::AbsMaxComponent,
            APPARATUS_RATIO,
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
/// The synthetic arm **skips** rather than errors when the fixture is absent,
/// like every other row in this suite that needs a file; it is committed, so
/// absence means a checkout without `fixtures/`.
#[must_use]
pub fn run(oracle: &crate::Oracle) -> (Option<Analysis>, Vec<Record>) {
    let mut records_out = Vec::new();
    let mut first = None;
    for (arm, path) in [
        ("swop", PathBuf::from(SWOP)),
        ("synthetic", synthetic_fixture()),
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
