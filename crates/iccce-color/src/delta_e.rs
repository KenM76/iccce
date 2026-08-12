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
//! ## ΔE94 and ΔE CMC — present, and WEAKER CLAIMS than ΔE2000
//!
//! Added 2026-08-12, transcribed from lcms2 `cmsCIE94DeltaE` and
//! `cmsCMCdeltaE` at the pinned commit. **Read the difference in
//! standing before using either:**
//!
//! | metric | expectation source | strength |
//! |---|---|---|
//! | ΔE2000 | Sharma et al. (2005), 34 published pairs | **ground truth** |
//! | ΔE76 | closed form, arithmetic identity | exact |
//! | **ΔE94** | lcms2 transcription | **impl_crosscheck** |
//! | **ΔE CMC** | lcms2 transcription | **impl_crosscheck** |
//!
//! CIE 116-1995 (ΔE94) and BS 6923 (CMC) are paywalled and NOT
//! sourced, and **no published worked example was obtained for
//! either** — so these two are validated by *agreement with one
//! implementation*, which project rule 3 says is a weaker claim than
//! ground truth and must never be written as though it were the same.
//! Their tests below assert structural properties (reduction to ΔE76
//! at unit weights on neutrals, symmetry where the metric is
//! symmetric) rather than published numbers, because there are no
//! published numbers to assert.
//!
//! **Grade suites in ΔE2000.** These exist because some published
//! tolerances are stated in them, not because they are as trustworthy.
//!
//! ★ CMC is deliberately NOT symmetric — it weights by the FIRST
//! colour (the reference), which is a property of the metric, not a
//! bug. `cmc(a,b) != cmc(b,a)` and the test asserts that it differs,
//! so nobody later "fixes" it into symmetry.

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

/// ΔE94 (CIE 1994), **graphic-arts** parametric factors
/// `kL = kC = kH = 1`, `K1 = 0.045`, `K2 = 0.015`.
///
/// Transcribed from lcms2 `cmsCIE94DeltaE` (`impl_crosscheck` — see
/// the module doc; CIE 116-1995 is paywalled and unsourced). Note
/// lcms2 hard-codes the graphic-arts weights as `sc = 1 + 0.048·C̄`
/// and `sh = 1 + 0.014·C̄`; the *textiles* variant (`2:1:1`, different
/// K) is a different metric and is not offered rather than guessed.
///
/// `dh` is recovered as `√(ΔE76² − ΔL² − ΔC²)` and floored at zero —
/// the standard trick to avoid a hue-angle branch, and lcms2's; the
/// floor matters because floating-point can make that difference
/// slightly negative for near-identical colours.
#[must_use]
pub fn delta_e_94(s: Lab, t: Lab) -> f64 {
    let dl = (s.l - t.l).abs();
    let (lch1, lch2) = (s.to_lch(), t.to_lch());
    let dc = (lch1.c - lch2.c).abs();
    let de = delta_e_76(s, t);
    let dhsq = de * de - dl * dl - dc * dc;
    let dh = if dhsq < 0.0 { 0.0 } else { dhsq.sqrt() };
    let c12 = (lch1.c * lch2.c).sqrt();
    let sc = 1.0 + 0.048 * c12;
    let sh = 1.0 + 0.014 * c12;
    (dl * dl + (dc * dc) / (sc * sc) + (dh * dh) / (sh * sh)).sqrt()
}

/// ΔE CMC(l:c) — the two common parameterisations are `2:1`
/// (acceptability) and `1:1` (perceptibility).
///
/// Transcribed from lcms2 `cmsCMCdeltaE` (`impl_crosscheck`; BS 6923
/// paywalled and unsourced).
///
/// ★ **Asymmetric by design**: every weighting term is computed from
/// `s` — the metric's *reference* colour — so `cmc(a,b)` and
/// `cmc(b,a)` legitimately differ. That is the definition, not a
/// defect, and a test below pins it so it cannot be "corrected".
///
/// Two guards carried verbatim from lcms2, both load-bearing: two
/// black colours return exactly 0 (the `L = 0` case would otherwise
/// divide by zero through `sl`), and `L < 16` pins `sl = 0.511`
/// rather than continuing the ratio downward.
#[must_use]
pub fn delta_e_cmc(s: Lab, t: Lab, l_weight: f64, c_weight: f64) -> f64 {
    if s.l == 0.0 && t.l == 0.0 {
        return 0.0;
    }
    let (lch1, lch2) = (s.to_lch(), t.to_lch());
    let dl = t.l - s.l;
    let dc = lch2.c - lch1.c;
    let de = delta_e_76(s, t);
    let dhsq = de * de - dl * dl - dc * dc;
    let dh = if dhsq > 0.0 { dhsq.sqrt() } else { 0.0 };

    // The hue-dependent term switches near the blue region.
    let t_term = if lch1.h > 164.0 && lch1.h < 345.0 {
        0.56 + (0.2 * (lch1.h + 168.0).to_radians().cos()).abs()
    } else {
        0.36 + (0.4 * (lch1.h + 35.0).to_radians().cos()).abs()
    };
    let sc = 0.0638 * lch1.c / (1.0 + 0.0131 * lch1.c) + 0.638;
    let sl = if s.l < 16.0 {
        0.511
    } else {
        0.040975 * s.l / (1.0 + 0.01765 * s.l)
    };
    let c4 = lch1.c.powi(4);
    let f = (c4 / (c4 + 1900.0)).sqrt();
    let sh = sc * (t_term * f + 1.0 - f);

    let a = dl / (l_weight * sl);
    let b = dc / (c_weight * sc);
    let c = dh / sh;
    (a * a + b * b + c * c).sqrt()
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

    /// ΔE94 and CMC reduce to ΔE76 on a pure LIGHTNESS difference
    /// between neutrals: C = 0 on both sides makes every weighting
    /// term 1 (ΔE94) or the documented constants (CMC), leaving the
    /// ΔL term alone. Structural property of the transcribed
    /// formulas — asserted because no published worked example was
    /// obtainable for either metric (module doc).
    #[test]
    fn de94_and_cmc_reduce_on_neutral_lightness_difference() {
        let a = Lab {
            l: 50.0,
            a: 0.0,
            b: 0.0,
        };
        let b = Lab {
            l: 60.0,
            a: 0.0,
            b: 0.0,
        };
        // ΔE94: sc = sh = 1 at C = 0, so it IS ΔL = ΔE76.
        assert!((delta_e_94(a, b) - 10.0).abs() < 1e-12);
        assert!((delta_e_94(a, b) - delta_e_76(a, b)).abs() < 1e-12);
        // CMC at 1:1 divides ΔL by sl, which at L=50 is
        // 0.040975·50/(1+0.01765·50) = 1.0872...; assert against the
        // formula's own arithmetic, not a recalled number.
        let sl = 0.040975 * 50.0 / (1.0 + 0.01765 * 50.0);
        assert!((delta_e_cmc(a, b, 1.0, 1.0) - 10.0 / sl).abs() < 1e-12);
    }

    /// ΔE94 and CMC against lcms2's OWN OUTPUT — the only external
    /// check available for these two, and labelled for exactly what
    /// it is: **implementation-cross-check, NOT ground truth**
    /// (project rule 3). CIE 116-1995 and BS 6923 are paywalled and
    /// no published worked example was obtainable, so agreement here
    /// says the transcription is faithful, and says nothing about
    /// whether lcms2 reads those standards correctly.
    ///
    /// Expected values produced 2026-08-12 by compiling a C probe
    /// against the pinned lcms2 (`cmsCIE94DeltaE`, `cmsCMCdeltaE`)
    /// and printing 10 decimals. Recorded here because the oracle is
    /// a subprocess the unit tests cannot reach.
    ///
    /// Agreement was exact to all ten printed digits on first run —
    /// which is expected for a transcription, and is why this is a
    /// weak test: it would also pass if both were wrong the same way.
    #[test]
    fn de94_and_cmc_match_lcms2_transcription() {
        let cases = [
            (
                Lab {
                    l: 50.0,
                    a: 2.6772,
                    b: -79.7751,
                },
                Lab {
                    l: 50.0,
                    a: 0.0,
                    b: -82.7485,
                },
                1.408_310_081_4,
                1.738_736_105_7,
                1.738_736_105_7,
            ),
            (
                Lab {
                    l: 20.0,
                    a: 40.0,
                    b: -30.0,
                },
                Lab {
                    l: 70.0,
                    a: -10.0,
                    b: 25.0,
                },
                68.911_643_645_3,
                58.055_319_818_0,
                92.094_183_238_0,
            ),
            (
                Lab {
                    l: 35.0,
                    a: -44.1164,
                    b: 3.7933,
                },
                Lab {
                    l: 35.0232,
                    a: -40.0716,
                    b: 1.5901,
                },
                1.844_619_451_0,
                2.024_752_084_5,
                2.024_878_928_7,
            ),
        ];
        for (a, b, e94, cmc21, cmc11) in cases {
            assert!(
                (delta_e_94(a, b) - e94).abs() < 1e-9,
                "dE94 {}",
                delta_e_94(a, b)
            );
            assert!(
                (delta_e_cmc(a, b, 2.0, 1.0) - cmc21).abs() < 1e-9,
                "CMC 2:1 {}",
                delta_e_cmc(a, b, 2.0, 1.0)
            );
            assert!(
                (delta_e_cmc(a, b, 1.0, 1.0) - cmc11).abs() < 1e-9,
                "CMC 1:1 {}",
                delta_e_cmc(a, b, 1.0, 1.0)
            );
        }
    }

    /// ★ CMC is ASYMMETRIC BY DEFINITION — it weights by the first
    /// (reference) colour. This test exists so nobody later "fixes"
    /// it into symmetry; ΔE94 and ΔE2000, by contrast, are symmetric
    /// and are asserted so elsewhere.
    #[test]
    fn cmc_is_asymmetric_on_purpose() {
        let a = Lab {
            l: 20.0,
            a: 40.0,
            b: -30.0,
        };
        let b = Lab {
            l: 70.0,
            a: -10.0,
            b: 25.0,
        };
        let fwd = delta_e_cmc(a, b, 2.0, 1.0);
        let rev = delta_e_cmc(b, a, 2.0, 1.0);
        assert!(
            (fwd - rev).abs() > 1e-6,
            "CMC weights by the reference colour: {fwd} vs {rev}"
        );
        // ΔE94 IS symmetric (its c12 is the geometric mean).
        assert!((delta_e_94(a, b) - delta_e_94(b, a)).abs() < 1e-12);
    }

    /// The two lcms2 guards carried verbatim, both load-bearing:
    /// two blacks return exactly 0 (else `sl` divides by zero), and
    /// `L < 16` pins `sl = 0.511`.
    #[test]
    fn cmc_guards_hold() {
        let black = Lab {
            l: 0.0,
            a: 0.0,
            b: 0.0,
        };
        assert_eq!(delta_e_cmc(black, black, 1.0, 1.0), 0.0);
        // Below L = 16 the lightness weight is the pinned constant:
        // a 1-unit ΔL at 1:1 is exactly 1/0.511.
        let dark = Lab {
            l: 10.0,
            a: 0.0,
            b: 0.0,
        };
        let dark2 = Lab {
            l: 11.0,
            a: 0.0,
            b: 0.0,
        };
        assert!((delta_e_cmc(dark, dark2, 1.0, 1.0) - 1.0 / 0.511).abs() < 1e-12);
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
