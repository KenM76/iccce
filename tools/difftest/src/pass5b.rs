//! # Pass 5b — the black-point ESTIMATORS, and a pre-registered prediction
//!
//! Pass 5 graded the BPC **scaling map**, the **direction** and the **policy**,
//! and said so in its own coverage statement: *"Both sides estimate the same
//! two black points in every scenario in reach, so this is agreement about the
//! MAP and the pipeline, not about the ESTIMATORS."* Closing that was left as
//! §16.8 item 4.
//!
//! Two things have since changed:
//!
//! 1. The operator obtained **ISO/CD 18619:2013**, and `crates/iccce-cmm`'s
//!    `bpc.rs` now implements clause 4.2.5's estimation procedure in full —
//!    ramp, monotonic pass, validity test, root-not-vertex, guards.
//! 2. The corpus recorded a **prediction, before this ran**:
//!
//!    > ISO ignores the black points' chroma and lcms2 retains it, so at input
//!    > black the divergence should equal the detected destination black's
//!    > `√(a*² + b*²)` — 2–6 ΔE76 — decaying to zero at white, on relative
//!    > colorimetric with a LUT destination.
//!
//! **A pre-registered prediction is the strongest thing this suite can carry
//! that is not a published value**, because it cannot have been fitted to the
//! observation: the mechanism, the sign, the magnitude band and the shape of
//! the decay were all written down first. This module measures it.
//!
//! ## ★★ FINDING, before any number — the estimator has NO CALLER
//!
//! `bpc::estimate_lut_destination_black` is implemented, documented and unit
//! tested, and **nothing outside its own test module calls it**.
//! `Chain::estimate_dst_black` (`transform.rs`) still reads:
//!
//! ```text
//! DestModel::Lut16B2a(_) | DestModel::LutAb(_) => {
//!     if self.dst_major >= 4 && self.intent == Intent::Perceptual {
//!         Ok(crate::bpc::PERCEPTUAL_BLACK)
//!     } else {
//!         Err(ChainError::BpcEstimationUnsupported)
//!     }
//! }
//! ```
//!
//! — the pre-ISO subset. So a **v2 CMYK LUT destination at media-relative**,
//! which is exactly the case ISO/CD 18619 4.2.5 exists for and exactly the case
//! Pass 5's row P19 recorded as a coverage gap, is still refused by the shipped
//! binary. §C grades that refusal, because it is the honest statement of where
//! the ISO work has and has not landed.
//!
//! **Consequence for what this module is allowed to claim.** §A and §B grade
//! `iccce_cmm::bpc`'s **library function**, in process, against lcms2's shipped
//! behaviour. They do **not** grade the shipped `iccce transform --bpc`, which
//! cannot reach this path. That is the same distinction Pass 4b §A draws
//! between its end-to-end rows and its PCS-side row, and it is stated on every
//! record rather than left to a reader.
//!
//! ## The two estimators, read at the pin before anything was run
//!
//! | | ISO/CD 18619 4.2.5 (iccce) | lcms2 2.19.1 `cmssamp.c` |
//! |---|---|---|
//! | ramp chroma | **ramps to zero**: `(t·100, ka(1−t), kb(1−t))` | **held constant**: `Lab.a = clamp(±50, InitialLab.a)` for every sample (L455–500) |
//! | samples | 256 | 256 |
//! | monotonic pass | downward, preserving the lightest | `outRamp[l] = min(outRamp[l], outRamp[l+1])` — identical |
//! | fit | least squares quadratic, **root** | `RootOfLeastSquaresFitQuadraticCurve` — **also the root**, so iccce's "root not vertex" correction of *Adobe* is not a divergence from *lcms2* |
//! | returned chroma | **`(L, 0, 0)` — neutral** (4.2.3) | **`Lab.a = InitialLab.a; Lab.b = InitialLab.b`** (L592) — the initial black's chroma, retained |
//!
//! **The prediction's mechanism is therefore confirmed by reading, not
//! inferred from the residual** — which is the discipline `TOLERANCES.md` §6.5
//! records as Pass 5's first carried-forward lesson.
//!
//! ★ A third difference falls out of the same two pages and was **not**
//! predicted: lcms2 clamps the chroma to ±50 *for the ramp* and then returns
//! the **unclamped** `InitialLab.a`/`.b`. Where a profile's darkest colorant
//! has |a*| or |b*| above 50 those are two different numbers inside one
//! function. It is reported, not graded — no profile in reach triggers it.
//!
//! ## The fixture, and why no new one was needed
//!
//! `USWebCoatedSWOP.icc` — v2.1, `prtr`, CMYK, `Lab ` PCS, `B2A1`/`A2B1` — **at
//! media-relative**. Pass 5's §16.8 item 4 asked for a synthetic v4 LUT fixture
//! with a non-zero device black to discriminate the estimators. That is the
//! right instrument for the *v4 perceptual* arm, where both implementations
//! currently return a constant. It is **not** needed here: SWOP's darkest
//! colorant is emphatically not `XYZ(0,0,0)`, and media-relative with a
//! v2 CMYK LUT destination is precisely lcms2's method-4 territory — the one
//! Pass 5 could not reach because iccce refused. The ISO function can now be
//! driven directly, so the discriminating measurement is available on a real
//! profile today.

use std::path::Path;

use iccce_cmm::bpc::{
    BpcScale, EstimationIntent, darkest_vertex, estimate_lut_destination_black, neutralise_and_clip,
};
use iccce_cmm::lut_transform::{Lut16Model, PcsKind, PcsValue};
use iccce_cmm::matrix_trc::MatrixTrc;
use iccce_color::{D50, Lab, Xyz, delta_e_2000};
use iccce_profile::Profile;
use iccce_profile::num::Signature;
use iccce_profile::tag_types::TagData;

use crate::{
    Bpc, Iccce, Intent, Kind, Metric, Oracle, Precalc, Record, Request, Space, Tolerance,
};

const SWOP: &str = r"C:\Windows\System32\spool\drivers\color\USWebCoatedSWOP.icc";
const SRGB: &str = r"C:\Windows\System32\spool\drivers\color\sRGB Color Space Profile.icm";

mod tag {
    use iccce_profile::num::Signature;
    pub const A2B1: Signature = Signature(0x4132_4231);
    pub const B2A1: Signature = Signature(0x4232_4131);
}

// ===========================================================================
// Tolerances
// ===========================================================================

/// **§A, the apparatus, re-derived after its first run failed.**
///
/// lcms2's detected destination black is not printable — `transicc` has no flag
/// for it — so it is **recovered** from its own output: with BPC on and a
/// source whose black is `XYZ(0,0,0)`, the second BPC constraint sends PCS zero
/// to the destination black *exactly* (Pass 5 row P3 graded that at
/// 3,33×10⁻¹⁶), so lcms2's CMYK at source black is `B2A1(black_lcms2)` and
/// `A2B1` carries it back. `A2B1 ∘ B2A1` is not the identity, so the recovery
/// has an error and it needs a bar.
///
/// ## ★ The first version of this row measured the wrong thing, and failed
///
/// It took the round-trip residual over `L* ∈ [0, 20]` and graded it at
/// `2,0 ΔE76`. **It failed at 16,49** — and the failure was neither the code
/// nor the tolerance. `USWebCoatedSWOP`'s darkest colorant is
/// `Lab(11,77 · 0,766 · 0,328)` and its estimated black sits at `L* ≈ 16,5`, so
/// **most of `[0, 20]` is outside the destination gamut**: `B2A1` clips, `A2B1`
/// returns the gamut floor, and the "round-trip error" being measured was the
/// gamut boundary rather than any inaccuracy. §0's procedure in order: the code
/// is not wrong, there is no recorded expectation, and **the fixture — the
/// probe range — was wrong.** The range is now the in-gamut shadow *above* the
/// estimated black, which is the only region the recovery reads.
///
/// ## The bound is a RATIO, and it has no free parameter
///
/// An error bar is readable exactly when it is **smaller than the effect it
/// bounds**. So the graded quantity is
/// `roundtrip error ÷ the black-point divergence it is the error bar for`, and
/// the tolerance is **1,0**. Below 1 the recovery can discriminate; at or above
/// 1 every §B number is inside its own uncertainty and the section is void
/// rather than merely worse. Nothing was chosen: the constant is the definition
/// of "readable".
pub const APPARATUS_RATIO: Tolerance = Tolerance::new(
    1.0,
    "the apparatus error divided by the effect it is the error bar for. An error bar is readable      exactly when it is smaller than what it bounds, so the constant is 1.0 and there is no free      parameter in it; at or above 1 every section B number sits inside its own uncertainty and      the section is void, not merely worse. The FIRST version of this row measured the      round-trip over L* in [0,20], which is mostly OUTSIDE this destination's gamut, and failed      at 16.49 dE76 on the gamut boundary rather than on any inaccuracy - the fixture was wrong,      not the number",
);

/// **§B, ★★ THE PRE-REGISTERED PREDICTION'S MECHANISM.**
///
/// The corpus predicted, before this ran:
///
/// > ISO ignores the black points' chroma, lcms2 retains it, so at input black
/// > the divergence should equal the detected destination black's
/// > `√(a*² + b*²)` — 2–6 ΔE76 — decaying to zero at white.
///
/// The prediction has **four separable claims** and they did not all survive.
/// This constant grades the one that did: the **chroma component** of the two
/// estimators' black-point divergence is the detected black's chroma.
///
/// `1×10⁻¹²`, and the row is labelled **structural on iccce's side**: ISO 4.2.3
/// returns `(L, 0, 0)`, so `Δa* = −a*_lcms2` and `Δb* = −b*_lcms2`
/// *identically*, and the residual is `f64` noise. **What it therefore grades
/// is that `neutralise_and_clip` and `estimate_lut_destination_black` really do
/// return a neutral black** — a genuine check of clause 4.2.3 against a build
/// that had quietly kept the chroma — and **not** that the prediction's
/// interesting half was right. Saying which is which is the whole point of the
/// row.
pub const MECHANISM_EXACT: Tolerance = Tolerance::new(
    1e-12,
    "STRUCTURAL on iccce's side: ISO/CD 18619 4.2.3 returns a NEUTRAL black, so the chroma      component of the divergence is the detected black's chroma identically and the residual is      f64 noise. It grades that clause 4.2.3 is implemented - a build that kept the chroma fails      it - NOT that the prediction's magnitude or shape were right, both of which were falsified.      1e-12 is ~4 orders above f64 noise on this arithmetic and ~10 below the effect",
);

/// **§B, the decay.** The prediction's second half: the divergence decays to
/// zero at white.
///
/// This is not decoration. BPC is a linear map anchored on `D50` at the white
/// end (Pass 5 row P3 graded `apply(D50) = D50` at 3,33×10⁻¹⁶), so *whatever*
/// the two estimators disagree about at the black end must vanish at the white
/// end. A divergence that did **not** decay would mean the disagreement is not
/// in the black point at all — it would be somewhere in the pipeline, and every
/// other row here would be attributing it to the wrong thing.
///
/// `5×10⁻² ΔE76` at device white. The residual there is not zero because the
/// two arms still differ in the B2A evaluation itself: Pass 4b §A measured
/// iccce against lcms2 on this exact table at **1,330×10⁻⁴ device**, and
/// row `DE_B2A_ROUNDTRIP`'s chain carries that to ≈1,8×10⁻² ΔE00 through
/// `A2B1`. `5×10⁻²` is ~2,8× that — **the same constant Pass 4b's row A4 uses,
/// deliberately unchanged**, because the quantity at white *is* Pass 4b's row
/// A4 and a different number here would be a tolerance tracking an observation.
pub const DECAYS_TO_ZERO: Tolerance = Tolerance::new(
    5e-2,
    "at device white BPC is anchored on D50 exactly (Pass 5 row P3, 3.33e-16), so a black-point \
     disagreement MUST vanish there; anything left is the B2A evaluation itself. Deliberately \
     the SAME constant as Pass 4b row A4 (DE_B2A_ROUNDTRIP), because the quantity at white IS \
     that row - a different number here would be a tolerance tracking an observation",
);

/// **§C.** The shipped binary must still refuse, and it must refuse **by
/// name**.
///
/// `iccce transform --bpc` into a v2 CMYK LUT destination at media-relative is
/// the case ISO/CD 18619 4.2.5 exists for, and `Chain::estimate_dst_black` does
/// not call the ISO estimator, so it refuses. **That refusal is graded rather
/// than reported**, for the reason Pass 5's row P19 gives: a build that quietly
/// substituted a zero black for an unestimable one would produce plausible
/// colour and pass every other row in this suite.
///
/// The needle is the **exact wording** iccce prints, not a paraphrase — Pass 5
/// row P20 failed on that distinction and it is not repeated here.
///
/// `0,0 — exact`: the quantity is `0` if the refusal matched and `1` if it did
/// not.
pub const REFUSES_BY_NAME: Tolerance = Tolerance::new(
    0.0,
    "0 if the shipped binary refused with the exact wording, 1 otherwise. Graded, not reported: a \
     build that silently substituted a zero black for an unestimable one would produce plausible \
     colour and pass every other row here. The needle is the exact Display text, not the error \
     variant's name - Pass 5 row P20 failed on precisely that",
);

pub const REPORTED: Tolerance = Tolerance::new(
    f64::INFINITY,
    "REPORTED, NOT GRADED - recorded so the number is on file next to the ones that are graded",
);

// ===========================================================================
// Analysis
// ===========================================================================

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

#[derive(Debug)]
pub struct Analysis {
    /// ISO 4.2.2.2's darkest vertex for this CMYK profile, and its Lab.
    pub darkest_device: Vec<f64>,
    pub darkest_lab: Lab,
    /// ISO's `InitialLab` after 4.2.3 — neutral by construction.
    pub initial_lab: Lab,
    /// ISO 4.2.5's estimate: `L*`, neutral.
    pub iso_black: Lab,
    /// lcms2's, recovered from its own BPC output.
    pub lcms2_black: Lab,
    /// `√(a*²+b*²)` of lcms2's recovered black — the prediction's magnitude.
    pub lcms2_chroma: f64,
    /// The apparatus: `A2B1(B2A1(x))` residual over the **in-gamut** neutral
    /// shadow above the estimated black, ΔE76.
    pub roundtrip_error: f64,
    /// ★ What `A2B1(B2A1(Lab(0,0,0)))` returns — the destination's **gamut
    /// floor**. The first version of the apparatus row measured the distance to
    /// this and called it a round-trip error. It is also the mechanism that
    /// absorbs most of the estimators' disagreement end to end.
    pub gamut_floor_l: f64,
    /// The gray ramp: `k` from 0 to 1, and the divergence at each.
    pub ramp: Vec<f64>,
    pub divergence_de76: Vec<f64>,
    pub divergence_de2000: Vec<f64>,
    pub divergence_device: Vec<f64>,
    /// Did the shipped binary refuse, with the exact wording?
    pub refusal_matched: bool,
    pub refusal_text: String,
    pub structure: String,
}

impl Analysis {
    /// The two estimators' black points, ΔE76 apart in the PCS. This is the
    /// effect §A's error bar has to be smaller than.
    #[must_use]
    pub fn black_point_divergence(&self) -> f64 {
        de76(self.iso_black, self.lcms2_black)
    }
    /// §A's graded quantity: the apparatus error as a fraction of the effect.
    #[must_use]
    pub fn apparatus_ratio(&self) -> f64 {
        let d = self.black_point_divergence();
        if d > 0.0 { self.roundtrip_error / d } else { f64::INFINITY }
    }
    /// The chroma component of the black-point divergence.
    #[must_use]
    pub fn divergence_chroma(&self) -> f64 {
        ((self.iso_black.a - self.lcms2_black.a).powi(2)
            + (self.iso_black.b - self.lcms2_black.b).powi(2))
        .sqrt()
    }
    /// §B's graded residual: does the chroma component equal the detected
    /// black's chroma? Zero by construction on iccce's side.
    #[must_use]
    pub fn mechanism_residual(&self) -> f64 {
        (self.divergence_chroma() - self.lcms2_chroma).abs()
    }
    /// The `L*` component the prediction dismissed.
    #[must_use]
    pub fn divergence_lightness(&self) -> f64 {
        (self.iso_black.l - self.lcms2_black.l).abs()
    }
    /// `L*` term ÷ chroma term. The prediction implies this is « 1; it is not.
    #[must_use]
    pub fn lightness_over_chroma(&self) -> f64 {
        self.divergence_lightness() / self.divergence_chroma()
    }
    /// `| observed at input black − predicted |`, kept for the report.
    #[must_use]
    pub fn prediction_residual(&self) -> f64 {
        (self.divergence_de76[0] - self.lcms2_chroma).abs()
    }
    #[must_use]
    pub fn divergence_at_white(&self) -> f64 {
        *self
            .divergence_de76
            .last()
            .expect("the ramp always has a white end")
    }
}

fn de76(a: Lab, b: Lab) -> f64 {
    ((a.l - b.l).powi(2) + (a.a - b.a).powi(2) + (a.b - b.b).powi(2)).sqrt()
}

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

/// The gray ramp the divergence is measured along: 21 steps of sRGB neutral.
/// Deliberately the **neutral axis**, because that is where a black-point
/// disagreement is largest and where its decay to white is legible; a
/// chromatic grid would mix the effect with the table's own curvature.
#[must_use]
pub fn gray_ramp() -> Vec<f64> {
    (0..21)
        .map(|i| f64::from(i) / 20.0)
        .collect()
}

/// Run Pass 5b.
pub fn analyse(oracle: &Oracle) -> Result<Analysis, Unavailable> {
    let src_path = Path::new(SRGB);
    let dst_path = Path::new(SWOP);
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
                "iccce binary not found: run `cargo build --release -p iccce-cli`".into(),
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

    let a2b1 = read_lut16(&dst, tag::A2B1)
        .ok_or_else(|| Unavailable::Error("SWOP has no decodable A2B1".into()))?;
    let b2a1 = read_lut8(&dst, tag::B2A1)
        .ok_or_else(|| Unavailable::Error("SWOP has no decodable B2A1".into()))?;
    let fwd = Lut16Model::from_lut16(&a2b1, false, PcsKind::Lab)
        .map_err(|e| Unavailable::Error(e.to_string()))?;
    let rev = Lut16Model::from_lut8(&b2a1, false, PcsKind::Lab)
        .map_err(|e| Unavailable::Error(e.to_string()))?;

    let dev_to_lab = |d: &[f64]| -> Lab {
        match fwd.device_to_pcs(d) {
            Some(PcsValue::Lab(l)) => l,
            _ => Lab {
                l: f64::NAN,
                a: f64::NAN,
                b: f64::NAN,
            },
        }
    };
    let lab_to_dev = |l: Lab| -> Vec<f64> { rev.pcs_to_device(PcsValue::Lab(l)).unwrap_or_default() };

    // --- ISO/CD 18619 4.2.2.2 → 4.2.3 → 4.2.5 -------------------------------
    let darkest_device = darkest_vertex(4, dev_to_lab);
    let darkest_lab = dev_to_lab(&darkest_device);
    let initial_lab = neutralise_and_clip(darkest_lab.l);
    // 4.2.5.2.3's round trip: Lab -> device (user's intent) -> Lab (ALWAYS
    // relative). Media-relative here, so both legs use the same pair of tables.
    let bt = |l: Lab| -> Lab { dev_to_lab(&lab_to_dev(l)) };
    let iso_l = estimate_lut_destination_black(initial_lab, EstimationIntent::RelativeColorimetric, bt);
    let iso_black = Lab {
        l: iso_l,
        a: 0.0,
        b: 0.0,
    };

    // --- the apparatus: how well does A2B1(B2A1(.)) round trip? -------------
    // ★ Measured over the IN-GAMUT neutral shadow ABOVE the estimated black,
    // which is the only region the recovery below reads. The first version of
    // this took L* in [0,20] and failed at 16.49 dE76 on the destination's
    // GAMUT BOUNDARY - most of that range is unreproducible by this profile,
    // and the number it produced was the clip, not an inaccuracy.
    let gamut_floor_l = bt(Lab {
        l: 0.0,
        a: 0.0,
        b: 0.0,
    })
    .l;
    let mut roundtrip_error = 0.0f64;
    for i in 0..=30 {
        let l = Lab {
            l: iso_l + f64::from(i) * 0.5,
            a: 0.0,
            b: 0.0,
        };
        roundtrip_error = roundtrip_error.max(de76(bt(l), l));
    }

    // --- lcms2, BPC on, over the gray ramp ----------------------------------
    let ramp = gray_ramp();
    let rows: Vec<[f64; 3]> = ramp.iter().map(|&k| [k, k, k]).collect();
    let req = |bpc: Bpc| Request {
        input: Space::profile(src_path),
        output: Space::profile(dst_path),
        intent: Intent::RelativeColorimetric,
        precalc: Precalc::Exact,
        bpc,
        values: rows.iter().flat_map(|t| t.iter().map(|v| v * 255.0)).collect(),
    };
    let lcms2_on = oracle.convert_batch_shaped(&req(Bpc::On), 3, 4)?;

    // Recover lcms2's destination black: its CMYK at source black is
    // B2A1(black_lcms2) because the second BPC constraint is exact, and A2B1
    // carries it back. The error in that recovery is `roundtrip_error`.
    let lcms2_black = dev_to_lab(
        &lcms2_on[0]
            .iter()
            .map(|v| v / 100.0)
            .collect::<Vec<f64>>(),
    );
    let lcms2_chroma = (lcms2_black.a.powi(2) + lcms2_black.b.powi(2)).sqrt();

    // --- iccce's ISO arm, built in the harness -------------------------------
    // The map is `BpcScale`, graded by Pass 5 §A against ICC.1:2022 6.3.4.3 at
    // 1,11e-16; the B2A evaluation is `Lut16Model`, graded by Pass 4b §A
    // against lcms2 at 1,33e-4 device. Neither is new here — what is new is
    // the black point the map is built from.
    let iso_scale = BpcScale::new(
        Xyz {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        iso_black.to_xyz(D50),
    )
    .ok_or_else(|| Unavailable::Error("BpcScale refused the ISO black".into()))?;

    let mut divergence_de76 = Vec::with_capacity(ramp.len());
    let mut divergence_de2000 = Vec::with_capacity(ramp.len());
    let mut divergence_device = Vec::with_capacity(ramp.len());
    for (i, &k) in ramp.iter().enumerate() {
        let pcs = src_model.device_to_pcs([k, k, k]);
        let mine = lab_to_dev(Lab::from_xyz(iso_scale.apply(pcs), D50));
        let theirs: Vec<f64> = lcms2_on[i].iter().map(|v| v / 100.0).collect();
        divergence_device.push(
            mine.iter()
                .zip(&theirs)
                .map(|(a, b)| (a - b).abs())
                .fold(0.0f64, f64::max),
        );
        let la = dev_to_lab(&mine);
        let lb = dev_to_lab(&theirs);
        divergence_de76.push(de76(la, lb));
        divergence_de2000.push(delta_e_2000(la, lb));
    }

    // --- §C: does the shipped binary still refuse? ---------------------------
    let refusal = iccce
        .transform_rows_shaped_bpc(
            src_path,
            dst_path,
            Intent::RelativeColorimetric,
            &[vec![0.0, 0.0, 0.0]],
            4,
            true,
        )
        .err()
        .map(|e| e.to_string())
        .unwrap_or_else(|| "NO REFUSAL: the transform succeeded".to_string());
    let refusal_matched =
        refusal.contains("black point not estimable within iccce's named subset (A42)");

    let structure = format!(
        "dst v{:08X} {} {}->{} B2A1 mft1 3->4 33 pts, A2B1 mft2 4->3 9 pts | src v{:08X} {} \
         matrix/TRC | media-relative | ISO/CD 18619 4.2.5 driven in process; the SHIPPED chain \
         cannot reach this path",
        dst.header.version.raw,
        dst.header.device_class,
        dst.header.color_space,
        dst.header.pcs,
        src.header.version.raw,
        src.header.device_class,
    );

    Ok(Analysis {
        darkest_device,
        darkest_lab,
        initial_lab,
        iso_black,
        lcms2_black,
        lcms2_chroma,
        roundtrip_error,
        gamut_floor_l,
        ramp,
        divergence_de76,
        divergence_de2000,
        divergence_device,
        refusal_matched,
        refusal_text: refusal,
        structure,
    })
}

// ===========================================================================
// Records
// ===========================================================================

#[must_use]
pub fn records(a: &Analysis) -> Vec<Record> {
    let ctx = format!(
        "{} | ISO darkest vertex {:?} -> Lab({:.4} {:.4} {:.4}), InitialLab({:.4} 0 0) | \
         ISO black L*={:.4} (neutral by 4.2.3) | lcms2 black recovered = \
         L*={:.4} a*={:.4} b*={:.4}, chroma={:.4} | roundtrip error bar={:.4} dE76",
        a.structure,
        a.darkest_device,
        a.darkest_lab.l,
        a.darkest_lab.a,
        a.darkest_lab.b,
        a.initial_lab.l,
        a.iso_black.l,
        a.lcms2_black.l,
        a.lcms2_black.a,
        a.lcms2_black.b,
        a.lcms2_chroma,
        a.roundtrip_error,
    );
    vec![
        Record::graded(
            "pass5b/apparatus/a2b1-b2a1-roundtrip-error-bar",
            Kind::SelfConsistency,
            Metric::AbsMaxComponent,
            ROUNDTRIP_ERROR_BAR,
            a.roundtrip_error,
            "iccce's own A2B1(B2A1(Lab)) over the neutral shadow L* in [0,20], the region the \
             black-point recovery reads. Both sides computed in this run; no lcms2 output in it",
            format!("{ctx} | 21 neutral Lab points"),
        ),
        Record::graded(
            "pass5b/estimators/black-points-in-lab",
            Kind::CrossCheck,
            Metric::AbsMaxComponent,
            REPORTED,
            de76(a.iso_black, a.lcms2_black),
            "REPORTED: ISO/CD 18619 4.2.5's estimate (iccce_cmm::bpc, in process) against lcms2 \
             2.19.1's, recovered from its own BPC output because transicc cannot print one",
            format!(
                "{ctx} | dL*={:.4} da*={:.4} db*={:.4} - the L* term is what the prediction \
                 omits and the chroma term is what it is about",
                a.iso_black.l - a.lcms2_black.l,
                a.iso_black.a - a.lcms2_black.a,
                a.iso_black.b - a.lcms2_black.b,
            ),
        ),
        Record::graded(
            "pass5b/PREDICTION/divergence-at-black-equals-lcms2-black-chroma",
            Kind::CrossCheck,
            Metric::AbsMaxComponent,
            PREDICTION_RESIDUAL,
            a.prediction_residual(),
            "★★ A PRE-REGISTERED PREDICTION, written into the corpus BEFORE this ran: ISO \
             ignores the black points' chroma and lcms2 retains it (cmssamp.c L592 `Lab.a = \
             InitialLab.a`), so at input black the divergence should EQUAL the detected \
             destination black's sqrt(a*^2+b*^2), 2-6 dE76. Graded on the RESIDUAL of that \
             prediction, not on the observation",
            format!(
                "{ctx} | observed at input black {:.4} dE76 ({:.4} dE2000, {:.6} device) vs \
                 predicted {:.4} -> residual {:.4}; the prediction's band was 2-6 dE76",
                a.divergence_de76[0],
                a.divergence_de2000[0],
                a.divergence_device[0],
                a.lcms2_chroma,
                a.prediction_residual(),
            ),
        ),
        Record::graded(
            "pass5b/PREDICTION/divergence-decays-to-zero-at-white",
            Kind::CrossCheck,
            Metric::AbsMaxComponent,
            DECAYS_TO_ZERO,
            a.divergence_at_white(),
            "the prediction's second half. BPC is anchored on D50 exactly at the white end \
             (Pass 5 row P3, 3.33e-16), so a black-point disagreement MUST vanish there; if it \
             did not, the divergence would not be in the black point and every other row here \
             would attribute it to the wrong thing",
            format!(
                "{ctx} | ramp dE76 at k=0,0.25,0.5,0.75,1: {:.4} {:.4} {:.4} {:.4} {:.4}",
                a.divergence_de76[0],
                a.divergence_de76[5],
                a.divergence_de76[10],
                a.divergence_de76[15],
                a.divergence_de76[20],
            ),
        ),
        Record::graded(
            "pass5b/coverage/shipped-chain-cannot-reach-the-iso-estimator",
            Kind::SelfConsistency,
            Metric::AbsMaxComponent,
            REFUSES_BY_NAME,
            if a.refusal_matched { 0.0 } else { 1.0 },
            "★ the honest statement of where the ISO work has landed: bpc::\
             estimate_lut_destination_black is implemented and unit tested but has NO CALLER - \
             Chain::estimate_dst_black still carries the pre-ISO subset - so the shipped \
             `iccce transform --bpc` refuses this exact case. Graded on the EXACT wording",
            format!("{ctx} | binary said: {}", crate::sanitise(&a.refusal_text)),
        ),
    ]
}

#[must_use]
pub fn unavailable_records(u: &Unavailable) -> Vec<Record> {
    let reason = u.to_string();
    let specs: Vec<(&str, Kind, Metric, Tolerance)> = vec![
        (
            "pass5b/apparatus/a2b1-b2a1-roundtrip-error-bar",
            Kind::SelfConsistency,
            Metric::AbsMaxComponent,
            ROUNDTRIP_ERROR_BAR,
        ),
        (
            "pass5b/estimators/black-points-in-lab",
            Kind::CrossCheck,
            Metric::AbsMaxComponent,
            REPORTED,
        ),
        (
            "pass5b/PREDICTION/divergence-at-black-equals-lcms2-black-chroma",
            Kind::CrossCheck,
            Metric::AbsMaxComponent,
            PREDICTION_RESIDUAL,
        ),
        (
            "pass5b/PREDICTION/divergence-decays-to-zero-at-white",
            Kind::CrossCheck,
            Metric::AbsMaxComponent,
            DECAYS_TO_ZERO,
        ),
        (
            "pass5b/coverage/shipped-chain-cannot-reach-the-iso-estimator",
            Kind::SelfConsistency,
            Metric::AbsMaxComponent,
            REFUSES_BY_NAME,
        ),
    ];
    specs
        .into_iter()
        .map(|(id, kind, metric, tol)| {
            let source = "ISO/CD 18619 4.2.5 against lcms2 2.19.1's cmsDetectDestinationBlackPoint";
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

#[must_use]
pub fn run(oracle: &Oracle) -> (Option<Analysis>, Vec<Record>) {
    match analyse(oracle) {
        Ok(a) => {
            let r = records(&a);
            (Some(a), r)
        }
        Err(u) => (None, unavailable_records(&u)),
    }
}
