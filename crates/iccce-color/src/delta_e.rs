//! # Colour-difference metrics — ΔE76 and CIEDE2000
//!
//! `ICC_Spec/cie/cie__ref__delta_e.md`: "the metric is the instrument;
//! an uncalibrated instrument makes every measurement it takes
//! worthless — and, worse, *confidently* worthless." This module is
//! therefore the most heavily tested code in the crate: the ΔE2000
//! implementation is validated against **all 34 Sharma et al. (2005)
//! pairs** — published, peer-reviewed ground truth constructed
//! specifically to break implementations — at the data's own precision
//! of 1×10⁻⁴.
//!
//! The formula transcription is from lcms2 `cmsCIE2000DeltaE`
//! (`impl_crosscheck` tier — CIE 142:2001 / ISO 11664-6 are paywalled
//! and not sourced). **Its correctness is established by the 34 pairs,
//! not by its provenance** — which is the point of ground-truth data.
//!
//! ## Deliberately absent (recorded gaps, not oversights)
//!
//! ΔE94 and ΔE CMC(l:c): the corpus has not yet transcribed their
//! formulas from a citable source, and no published worked examples are
//! in hand — an implementation now could only be cross-checked against
//! lcms2, a weaker claim that project rule 3 requires labelling. They
//! land when sourced (`cie__ref__delta_e.md`, GAP).

use crate::lab::Lab;

/// ΔE\*ab (CIE 1976): `√(ΔL² + Δa² + Δb²)`
/// (`cie__ref__delta_e.md`, cross-checked lcms2 `cmsDeltaE`).
///
/// Badly perceptually non-uniform — a ΔE76 of 2 is very visible in some
/// hues, invisible in others (corpus). Kept because it is cheap and
/// some published tolerances are stated in it; grade suites in ΔE2000.
#[must_use]
pub fn delta_e_76(s: Lab, t: Lab) -> f64 {
    let dl = s.l - t.l;
    let da = s.a - t.a;
    let db = s.b - t.b;
    (dl * dl + da * da + db * db).sqrt()
}

/// CIEDE2000 with `kL = kC = kH = 1` (the parametric factors the Sharma
/// data is stated for, and the reference viewing-condition default).
#[must_use]
pub fn delta_e_2000(s: Lab, t: Lab) -> f64 {
    delta_e_2000_k(s, t, 1.0, 1.0, 1.0)
}

/// `atan2` in degrees, wrapped to `[0, 360)`, with `atan2(0,0)`
/// defined as 0 — lcms2 special-cases this because the C result is
/// platform-dependent (`cie__ref__delta_e.md` trap 2). Rust's
/// `f64::atan2(0.0, 0.0)` is 0 by IEEE 754, but the guard is kept
/// explicit so the contract is visible and signed-zero inputs
/// (`atan2(-0.0, x)`) cannot produce −0.0-adjacent surprises.
fn atan2_deg(b: f64, a: f64) -> f64 {
    if a == 0.0 && b == 0.0 {
        return 0.0;
    }
    let mut h = b.atan2(a).to_degrees();
    if h < 0.0 {
        h += 360.0;
    }
    h
}

/// CIEDE2000 with explicit parametric factors.
///
/// Transcribed step-for-step from `cie__ref__delta_e.md` (lcms2
/// `cmsCIE2000DeltaE` verbatim), traps preserved deliberately:
///
/// - the `180.000001` epsilons are **verbatim from lcms2** — a
///   deliberate guard against floating-point equality at the branch,
///   and exactly what Sharma pairs 9–16 (the hue discontinuity) test;
/// - `G` uses the ORIGINAL chromas, everything after uses primed;
/// - `R_T` is a cross term inside the sqrt (pairs 1–6 catch omission);
/// - trig arguments are converted degree→radian explicitly at each
///   site — mixing them "produces a near-correct answer, the worst
///   kind" (trap 4);
/// - all `f64`: `C̄'⁷` reaches 10¹⁴ and overflows `f32` (trap 5).
#[must_use]
pub fn delta_e_2000_k(s: Lab, t: Lab, kl: f64, kc: f64, kh: f64) -> f64 {
    const POW25_7: f64 = 6_103_515_625.0; // 25⁷, exact in f64

    // -- Chroma correction: G from ORIGINAL chromas (trap 6).
    let c1 = s.a.hypot(s.b);
    let c2 = t.a.hypot(t.b);
    let c_mean = (c1 + c2) / 2.0;
    let c_mean7 = c_mean.powi(7);
    let g = 0.5 * (1.0 - (c_mean7 / (c_mean7 + POW25_7)).sqrt());

    let a1p = (1.0 + g) * s.a;
    let a2p = (1.0 + g) * t.a;
    let c1p = a1p.hypot(s.b);
    let c2p = a2p.hypot(t.b);
    let h1p = atan2_deg(s.b, a1p);
    let h2p = atan2_deg(t.b, a2p);

    // -- Differences.
    let dl = t.l - s.l; // destination minus source (trap 7)
    let dc = c2p - c1p;
    let dh_raw = h2p - h1p;
    let dh = if dh_raw <= -180.000001 {
        dh_raw + 360.0
    } else if dh_raw > 180.0 {
        dh_raw - 360.0
    } else {
        dh_raw
    };
    let dhh = 2.0 * (c1p * c2p).sqrt() * (dh / 2.0).to_radians().sin();

    // -- Means.
    let l_mean = (s.l + t.l) / 2.0;
    let cp_mean = (c1p + c2p) / 2.0;
    let h_sum = h1p + h2p;
    let hp_mean = if (h2p - h1p).abs() <= 180.000001 {
        h_sum / 2.0
    } else if h_sum < 360.0 {
        (h_sum + 360.0) / 2.0
    } else {
        (h_sum - 360.0) / 2.0
    };

    // -- Weighting functions. Trig in degrees, converted per-site.
    let tt = 1.0 - 0.17 * (hp_mean - 30.0).to_radians().cos()
        + 0.24 * (2.0 * hp_mean).to_radians().cos()
        + 0.32 * (3.0 * hp_mean + 6.0).to_radians().cos()
        - 0.20 * (4.0 * hp_mean - 63.0).to_radians().cos();
    let l50 = (l_mean - 50.0) * (l_mean - 50.0);
    let sl = 1.0 + 0.015 * l50 / (20.0 + l50).sqrt();
    let sc = 1.0 + 0.045 * cp_mean;
    let sh = 1.0 + 0.015 * cp_mean * tt;

    // -- Hue rotation (the blue-region term pairs 1–6 exist for).
    let d_theta = 30.0 * (-((hp_mean - 275.0) / 25.0).powi(2)).exp();
    let cp_mean7 = cp_mean.powi(7);
    let rc = 2.0 * (cp_mean7 / (cp_mean7 + POW25_7)).sqrt();
    let rt = -(2.0 * d_theta).to_radians().sin() * rc;

    let dl_t = dl / (kl * sl);
    let dc_t = dc / (kc * sc);
    let dh_t = dhh / (kh * sh);
    (dl_t * dl_t + dc_t * dc_t + dh_t * dh_t + rt * dc_t * dh_t).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// ΔE76 on a 3-4-5-… quadruple: √(3²+4²+12²) = 13 exactly.
    /// Arithmetic identity, not a measurement.
    #[test]
    fn de76_pythagorean_identity() {
        let s = Lab {
            l: 50.0,
            a: 10.0,
            b: 20.0,
        };
        let t = Lab {
            l: 53.0,
            a: 14.0,
            b: 32.0,
        };
        assert_eq!(delta_e_76(s, t), 13.0);
    }

    /// ★ GROUND TRUTH — Sharma, Wu & Dalal (2005), Color Research &
    /// Application 30(1), 21–30, DOI 10.1002/col.20070. All 34 pairs,
    /// transcribed from `cie__ref__delta_e.md` (retrieved verbatim from
    /// the first author's dataset 2026-08-11). kL = kC = kH = 1.
    ///
    /// Tolerance 1×10⁻⁴ — the published data's own precision, per the
    /// corpus's tolerance table. This is a real published expectation
    /// (project rule 3, strongest class), and the whole set runs:
    /// cherry-picking defeats the dataset's design (pairs 1–6 catch an
    /// omitted R_T; 7–8 asymmetry; 9–16 the hue discontinuity — where
    /// b differing in the 4th decimal legitimately changes the answer;
    /// 21–24 calibrate ΔE = 1.0 in four directions; 33–34 very dark).
    #[rustfmt::skip]
    const SHARMA_34: [(f64, f64, f64, f64, f64, f64, f64); 34] = [
        (50.0000,   2.6772, -79.7751, 50.0000,   0.0000, -82.7485,  2.0425),
        (50.0000,   3.1571, -77.2803, 50.0000,   0.0000, -82.7485,  2.8615),
        (50.0000,   2.8361, -74.0200, 50.0000,   0.0000, -82.7485,  3.4412),
        (50.0000,  -1.3802, -84.2814, 50.0000,   0.0000, -82.7485,  1.0000),
        (50.0000,  -1.1848, -84.8006, 50.0000,   0.0000, -82.7485,  1.0000),
        (50.0000,  -0.9009, -85.5211, 50.0000,   0.0000, -82.7485,  1.0000),
        (50.0000,   0.0000,   0.0000, 50.0000,  -1.0000,   2.0000,  2.3669),
        (50.0000,  -1.0000,   2.0000, 50.0000,   0.0000,   0.0000,  2.3669),
        (50.0000,   2.4900,  -0.0010, 50.0000,  -2.4900,   0.0009,  7.1792),
        (50.0000,   2.4900,  -0.0010, 50.0000,  -2.4900,   0.0010,  7.1792),
        (50.0000,   2.4900,  -0.0010, 50.0000,  -2.4900,   0.0011,  7.2195),
        (50.0000,   2.4900,  -0.0010, 50.0000,  -2.4900,   0.0012,  7.2195),
        (50.0000,  -0.0010,   2.4900, 50.0000,   0.0009,  -2.4900,  4.8045),
        (50.0000,  -0.0010,   2.4900, 50.0000,   0.0010,  -2.4900,  4.8045),
        (50.0000,  -0.0010,   2.4900, 50.0000,   0.0011,  -2.4900,  4.7461),
        (50.0000,   2.5000,   0.0000, 50.0000,   0.0000,  -2.5000,  4.3065),
        (50.0000,   2.5000,   0.0000, 73.0000,  25.0000, -18.0000, 27.1492),
        (50.0000,   2.5000,   0.0000, 61.0000,  -5.0000,  29.0000, 22.8977),
        (50.0000,   2.5000,   0.0000, 56.0000, -27.0000,  -3.0000, 31.9030),
        (50.0000,   2.5000,   0.0000, 58.0000,  24.0000,  15.0000, 19.4535),
        (50.0000,   2.5000,   0.0000, 50.0000,   3.1736,   0.5854,  1.0000),
        (50.0000,   2.5000,   0.0000, 50.0000,   3.2972,   0.0000,  1.0000),
        (50.0000,   2.5000,   0.0000, 50.0000,   1.8634,   0.5757,  1.0000),
        (50.0000,   2.5000,   0.0000, 50.0000,   3.2592,   0.3350,  1.0000),
        (60.2574, -34.0099,  36.2677, 60.4626, -34.1751,  39.4387,  1.2644),
        (63.0109, -31.0961,  -5.8663, 62.8187, -29.7946,  -4.0864,  1.2630),
        (61.2901,   3.7196,  -5.3901, 61.4292,   2.2480,  -4.9620,  1.8731),
        (35.0831, -44.1164,   3.7933, 35.0232, -40.0716,   1.5901,  1.8645),
        (22.7233,  20.0904, -46.6940, 23.0331,  14.9730, -42.5619,  2.0373),
        (36.4612,  47.8580,  18.3852, 36.2715,  50.5065,  21.2231,  1.4146),
        (90.8027,  -2.0831,   1.4410, 91.1528,  -1.6435,   0.0447,  1.4441),
        (90.9257,  -0.5406,  -0.9208, 88.6381,  -0.8985,  -0.7239,  1.5381),
        ( 6.7747,  -0.2908,  -2.4247,  5.8714,  -0.0985,  -2.2286,  0.6377),
        ( 2.0776,   0.0795,  -1.1350,  0.9033,  -0.0636,  -0.5514,  0.9082),
    ];

    #[test]
    fn de2000_matches_all_34_sharma_pairs() {
        for (i, &(l1, a1, b1, l2, a2, b2, expected)) in SHARMA_34.iter().enumerate() {
            let s = Lab {
                l: l1,
                a: a1,
                b: b1,
            };
            let t = Lab {
                l: l2,
                a: a2,
                b: b2,
            };
            let got = delta_e_2000(s, t);
            assert!(
                (got - expected).abs() < 1e-4,
                "Sharma pair {}: got {got:.6}, published {expected}",
                i + 1
            );
        }
    }

    /// Symmetry over the full dataset: ΔE(A,B) == ΔE(B,A). Pairs 7–8
    /// publish this property for one pair; asserting it across all 34
    /// costs nothing and catches asymmetric mean-hue handling anywhere
    /// in the gamut.
    #[test]
    fn de2000_is_symmetric() {
        for &(l1, a1, b1, l2, a2, b2, _) in &SHARMA_34 {
            let s = Lab {
                l: l1,
                a: a1,
                b: b1,
            };
            let t = Lab {
                l: l2,
                a: a2,
                b: b2,
            };
            let fwd = delta_e_2000(s, t);
            let rev = delta_e_2000(t, s);
            assert!(
                (fwd - rev).abs() < 1e-12,
                "asymmetry: {fwd} vs {rev} for ({l1},{a1},{b1})↔({l2},{a2},{b2})"
            );
        }
    }

    /// Identical colours: exactly zero. Arithmetic identity.
    #[test]
    fn de2000_of_identical_is_zero() {
        let s = Lab {
            l: 50.0,
            a: 2.5,
            b: 0.0,
        };
        assert_eq!(delta_e_2000(s, s), 0.0);
    }
}
