//! # Black point compensation — Pass 5
//!
//! Maps the source profile's black point to the destination's by a
//! per-component linear scale of media-relative PCSXYZ that keeps D50
//! fixed. Per `ICC_Spec/icc/icc__ref__bpc.md` (2026-08-11):
//!
//! ## The scaling map — PRIMARY-SOURCED
//!
//! ICC.1:2022 **clause 6.3.4.3** states the map `Xp = Xt·(1−Xb/Xi)+Xb`
//! (its v2→v4 perceptual-black adjustment), which with both
//! constraints — `D50 = a·D50 + b` and `black_dst = a·black_src + b`
//! — solves per component to:
//!
//! ```text
//!   a = (D50 − bd) / (D50 − bs)
//!   b = D50 · (bd − bs) / (D50 − bs)
//! ```
//!
//! algebraically identical to lcms2's `ComputeBlackPointCompensation`
//! (corpus: exact in decimal arithmetic; 1.1e-16 over 5e4 float
//! draws) and to Maria (2013)'s published two-constraint derivation —
//! retrieved compliantly from littlecms.com. NOTE the citation
//! discipline: 6.3.4.3 is cited for the SCALING MAP, not "for BPC" —
//! its actor and its known-vs-estimated black differ (the C1 failure
//! mode the corpus warns against).
//!
//! ## The estimation step — NOW SOURCED (A42, upgraded 2026-08-11)
//!
//! The operator's manual download of the ISO/TC130 committee draft
//! (`_sources\BlackPointCompensation.pdf`, which turned out to be
//! **ISO/CD 18619:2013, not WP40** — WP40 is its own superseded
//! ancestor) supplies the whole estimation procedure in `shall`
//! language. Every threshold this project previously carried as
//! "unattributed lcms2 constant" is in clause 4.2 verbatim: `0.2`,
//! `≥ 4`, the shadow windows `[0.1,0.5)` / `[0.03,0.25)`, the
//! `L* ≤ 50` clamps, 256 ramp samples, the ±50 chroma clamp,
//! `1.0E-10`, `max(0,min(50,·))`, `n < 3`.
//!
//! **Citation form is binding: "ISO/CD 18619:2013 clause 4.2.x",
//! never "ISO 18619"** — a committee draft has normative language and
//! non-normative status, and its own cover forbids the short form.
//!
//! [`estimate_lut_destination_black`] implements 4.2.5 including the
//! three places ISO corrects Adobe: the darkest-colour SEARCH
//! ([`darkest_vertex`]) instead of a fixed device black, the ROOT
//! instead of the vertex, and the monotonic + validity guards.
//!
//! **The predicted divergence from lcms2, recorded before measuring**
//! (corpus §6.4): ISO 4.2.6 says the black points' `a`/`b` "are
//! ignored"; lcms2 retains chroma and propagates it per-channel. At
//! input black the difference should equal exactly the detected
//! destination black's `√(a*²+b*²)` — **2–6 ΔE76** for a `b*` of
//! −2…−6, decaying to zero at white, on relative colorimetric with a
//! LUT destination. iccce follows ISO (neutral), which is why
//! [`neutralise_and_clip`] exists.
//!
//! ### What is still lcms2's alone, and iccce does NOT copy
//!
//! Three constants have no home in either document: `L* > 95 → 0`,
//! `IsEmptyLayer`'s `0.002` stage-drop (M6), and an `n < 4` fitter
//! guard that contradicts both ISO's `3` and lcms2's own caller.
//!
//! ### The fixed perceptual black, kept for the v4 perceptual case
//!
//! Neither BPC document mentions a fixed perceptual black point at
//! all, so [`PERCEPTUAL_BLACK`] keeps its own provenance: the
//! implementations' hybrid triple, byte-identical between lcms2 and
//! ICC's own iccDEV and differing from ICC.1 Table 16's printed
//! decimals by 0.037 ΔE76 — exactly zero at 16-bit PCS, but
//! **0.0502 ΔE2000**, the same order as a whole comparison budget on
//! a float path (**A41**).
//!
//! Consequence, unchanged: no BPC conformance test with a fixed
//! *published* expected value exists (A27's standing); ISO gives the
//! procedure, not worked numbers. `docs/TOLERANCES.md` owns the
//! grades.

use iccce_color::{D50, Lab, Xyz};

/// The implementations' fixed v4 perceptual black (see module doc,
/// A41 — deliberately the lcms2/iccDEV triple, not Table 16's text).
pub const PERCEPTUAL_BLACK: Xyz = Xyz {
    x: 0.00336,
    y: 0.0034731,
    z: 0.00287,
};

/// Which rendering intent the estimation runs under. ISO/CD 18619
/// branches on relative-vs-(perceptual|saturation) in three places:
/// the `InitialLab`, the shadow window, and whether the mid-range
/// straightness test runs at all (4.2.5.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EstimationIntent {
    RelativeColorimetric,
    PerceptualOrSaturation,
}

/// Device-space vertex sets for `D(Profile, Intent)` — ISO/CD 18619
/// 4.2.2.2, VERBATIM sets: Gray `{(0) (1)}`, RGB `{(0,0,0) (1,1,1)}`,
/// CMYK `{(0,0,0,0) (1,1,1,1) (0,0,0,1) (1,1,1,0)}`.
///
/// ★ PUBLIC DELIBERATELY, unlike the byte-level readers in
/// `iccce-profile::num` which were sealed to `pub(crate)` in the same
/// pre-publication pass. The distinction: those decode *our* file
/// format and a consumer has no business calling them, whereas this
/// and its three siblings are a faithful implementation of a
/// *published algorithm* that a caller may legitimately want to drive
/// with its own black points, its own profiles, or no profiles at
/// all. The clause citations in these doc comments are the point of
/// exposing them.
///
/// ISO NOTE 2 is why this is a SEARCH rather than a constant:
/// "Determining the darkest colour in this way works for profiles
/// with both the normal polarity and inverse polarity." Adobe used a
/// fixed device black and cannot survive an inverse-polarity profile
/// — it would hand back that profile's white.
#[must_use]
pub fn vertex_set(channels: usize) -> Vec<Vec<f64>> {
    match channels {
        1 => vec![vec![0.0], vec![1.0]],
        4 => vec![
            vec![0.0, 0.0, 0.0, 0.0],
            vec![1.0, 1.0, 1.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
            vec![1.0, 1.0, 1.0, 0.0],
        ],
        // RGB and any other N: the two extremes. (ISO enumerates
        // Gray/RGB/CMYK only; for other channel counts the same
        // all-min/all-max pair is the honest generalisation and is
        // marked as such rather than presented as sourced.)
        n => vec![vec![0.0; n], vec![1.0; n]],
    }
}

/// `D(Profile, Intent)` — the darkest vertex by `L*`
/// (ISO/CD 18619 4.2.2.2). `to_lab` transforms a device vector to
/// Lab under the caller's intent.
#[must_use]
pub fn darkest_vertex(channels: usize, to_lab: impl Fn(&[f64]) -> Lab) -> Vec<f64> {
    let mut best: Option<(f64, Vec<f64>)> = None;
    for v in vertex_set(channels) {
        let l = to_lab(&v).l;
        if best.as_ref().is_none_or(|(bl, _)| l < *bl) {
            best = Some((l, v));
        }
    }
    best.expect("vertex_set is never empty").1
}

/// Neutralise and clip a black point's `L*` — ISO/CD 18619 4.2.3:
/// **always** neutral `(Li, 0, 0)` (Adobe neutralised for CMYK only),
/// and `Li > 50 → 50`.
#[must_use]
pub fn neutralise_and_clip(l: f64) -> Lab {
    Lab {
        l: if l > 50.0 { 50.0 } else { l },
        a: 0.0,
        b: 0.0,
    }
}

/// Estimate a LUT-based destination black point — ISO/CD 18619
/// 4.2.5, the procedure Adobe approximated.
///
/// `initial_lab` is 4.2.5.2.1's `InitialLab` (already neutralised and
/// clipped by the caller where ISO requires it); `bt` is 4.2.5.2.3's
/// round-trip `BT(x) = T(T(x, Lab→dst, intent), dst→Lab, RELATIVE)`
/// — note the inner leg uses the user's intent and the outer leg is
/// **always relative colorimetric**, which is the one place a
/// "relative" appears that is not the caller's choice.
///
/// Returns the destination black point as a full `Lab`.
///
/// ★ CORRECTED 2026-08-12 — this returned only `L*`, and the
/// straightness branch returned the wrong quantity entirely. ISO/CD
/// 18619 4.2.5.4 final paragraph, VERBATIM: *"If the mid range is
/// straight (as determined above) then the DestinationBlackPoint
/// **shall be the same as InitialLab**."* 4.2.5.1's control-flow
/// summary says it a second time. `outRamp[first]` — which this
/// function used to return there — appears in clause 4.2.5 only as
/// `MinL` (a threshold and `yRamp` anchor) and in 4.2.5.3's validity
/// test, and **is not a candidate for the black point in any
/// branch**. lcms2 conformed; iccce did not. Cost of the defect,
/// measured before it was found: 0.0817 ΔE76 on USWebCoatedSWOP,
/// which was 100 % of the two implementations' disagreement there.
///
/// The return type widened because of a corollary the same reading
/// produces: **the short-circuit is the only branch that can return
/// a CHROMATIC black.** 4.2.5.2.1 zeroes chroma only for CMYK, so on
/// a Gray/RGB LUT destination ISO itself yields a chromatic
/// `DestinationBlackPoint` — and neutralising it here would be a
/// second, quieter departure. (4.2.6 ignores `a`/`b` downstream
/// anyway, so the cost is zero today and the correctness is not.)
///
/// Gives up to `(0,0,0)` on every ISO-specified path: an invalid
/// ramp (4.2.5.3), fewer than 3 shadow points, a non-positive
/// discriminant. Note ISO defect §4.6: 4.2.5.4's opening sentence
/// would route an invalid ramp into curve fitting, contradicting
/// 4.2.5.3 and 4.2.5.1; this follows the two clauses that agree.
#[must_use]
pub fn estimate_lut_destination_black(
    initial_lab: Lab,
    intent: EstimationIntent,
    bt: impl Fn(Lab) -> Lab,
) -> Lab {
    // 4.2.5.2.2: 256 equal steps from (0, ka, kb) to (100, 0, 0) —
    // chroma RAMPS TO ZERO (Adobe held it constant over 101 samples;
    // the corpus calls this substantive, not editorial), with the
    // chroma clamped to ±50 first.
    const N: usize = 256;
    let ka = initial_lab.a.clamp(-50.0, 50.0);
    let kb = initial_lab.b.clamp(-50.0, 50.0);
    let mut in_ramp = Vec::with_capacity(N);
    let mut out_ramp = Vec::with_capacity(N);
    for i in 0..N {
        #[allow(clippy::cast_precision_loss)]
        let t = i as f64 / (N - 1) as f64;
        let sample = Lab {
            l: t * 100.0,
            a: ka * (1.0 - t),
            b: kb * (1.0 - t),
        };
        in_ramp.push(sample.l);
        out_ramp.push(bt(sample).l);
    }

    // 4.2.5.2.3: monotonic pass, downward, preserving the last
    // (lightest) value. This is the correction for the defect Adobe
    // describes ("noise in the constant section … the corner is
    // often rounded") and does not fix.
    for i in (0..N - 1).rev() {
        out_ramp[i] = out_ramp[i].min(out_ramp[i + 1]);
    }

    // 4.2.5.3 validity, VERBATIM: "When outRamp[first] is not less
    // than outRamp[last] then the outRamp is considered invalid and
    // the DestinationBlackPoint shall be set to (0, 0, 0)."
    let min_l = out_ramp[0];
    let max_l = out_ramp[N - 1];
    // NaN-safe by construction: `>=` is false for NaN, so a NaN ramp
    // falls through to the fit and its give-up paths rather than
    // being silently treated as valid.
    if min_l >= max_l {
        return Lab {
            l: 0.0,
            a: 0.0,
            b: 0.0,
        };
    }

    // 4.2.5.4: the mid-range straightness test runs ONLY for relative
    // colorimetric (4.2.5.1). Note the gated quantity is the INPUT
    // (ISO) where Adobe gated the output — a real change, and ISO's
    // own version compares an input L* against an output-derived
    // threshold, which the corpus flags as an internal problem (§4.4)
    // but which is what the document says.
    if intent == EstimationIntent::RelativeColorimetric {
        let threshold = min_l + 0.2 * (max_l - min_l);
        let mut straight = true;
        for i in 0..N {
            if in_ramp[i] > threshold && (in_ramp[i] - out_ramp[i]).abs() >= 4.0 {
                straight = false;
                break;
            }
        }
        if straight {
            // 4.2.5.4 VERBATIM: "the DestinationBlackPoint shall be
            // the same as InitialLab" — carried through unchanged,
            // the whole triple. NOT outRamp[first] (which is MinL,
            // the threshold anchor, and is never a black-point
            // candidate in any branch of 4.2.5).
            return initial_lab;
        }
    }

    // 4.2.5.5: fit y = a·x² + b·x + c over the shadow window.
    let (lo, hi) = match intent {
        EstimationIntent::RelativeColorimetric => (0.1, 0.5),
        EstimationIntent::PerceptualOrSaturation => (0.03, 0.25),
    };
    let mut xs = Vec::new();
    let mut ys = Vec::new();
    for i in 0..N {
        let y = (out_ramp[i] - min_l) / (max_l - min_l);
        if y >= lo && y < hi {
            xs.push(in_ramp[i]);
            ys.push(y);
        }
    }
    // "If there are fewer than 3 points in SP … set to (0, 0, 0)."
    if xs.len() < 3 {
        return Lab {
            l: 0.0,
            a: 0.0,
            b: 0.0,
        };
    }
    let Some((a, b, c)) = fit_quadratic(&xs, &ys) else {
        return Lab {
            l: 0.0,
            a: 0.0,
            b: 0.0,
        };
    };

    // ★ The single largest delta from Adobe: take the ROOT, not the
    // vertex. Adobe's own prose names the root ("the point where this
    // curve intersects the L*=0 round trip value") and its pseudocode
    // computes −u/2t, calling it "an approximation". ISO computes the
    // root exactly and adds the two guards the approximation needs:
    // the near-linear fallback (the vertex is UNBOUNDED as the fit
    // straightens — the common case) and the [0,50] clamp.
    // 4.2.5.5 returns a NEUTRAL (z, 0, 0) on every fitted path.
    let neutral = |z: f64| Lab {
        l: z,
        a: 0.0,
        b: 0.0,
    };
    if a.abs() < 1.0e-10 {
        if b == 0.0 {
            return Lab {
                l: 0.0,
                a: 0.0,
                b: 0.0,
            };
        }
        return neutral((-c / b).clamp(0.0, 50.0));
    }
    let d = b * b - 4.0 * a * c;
    if d <= 0.0 {
        return Lab {
            l: 0.0,
            a: 0.0,
            b: 0.0,
        };
    }
    // NOTE 1: "z has been assigned to the root of the quadratic with
    // the positive gradient."
    neutral(((-b + d.sqrt()) / (2.0 * a)).clamp(0.0, 50.0))
}

/// Least-squares fit of `y = a·x² + b·x + c` by the normal equations.
/// `None` when the system is singular (degenerate x spread).
fn fit_quadratic(xs: &[f64], ys: &[f64]) -> Option<(f64, f64, f64)> {
    // Sample count is bounded by the 256-step ramp: exact in f64.
    #[allow(clippy::cast_precision_loss)]
    let n = xs.len() as f64;
    let (mut s1, mut s2, mut s3, mut s4) = (0.0, 0.0, 0.0, 0.0);
    let (mut t0, mut t1, mut t2) = (0.0, 0.0, 0.0);
    for (&x, &y) in xs.iter().zip(ys) {
        let (x2, x3, x4) = (x * x, x * x * x, x * x * x * x);
        s1 += x;
        s2 += x2;
        s3 += x3;
        s4 += x4;
        t0 += y;
        t1 += x * y;
        t2 += x2 * y;
    }
    // Solve the 3×3 normal system by Cramer's rule.
    let m = [[s4, s3, s2], [s3, s2, s1], [s2, s1, n]];
    let det = det3(&m);
    if det == 0.0 {
        return None;
    }
    let rhs = [t2, t1, t0];
    let solve = |col: usize| {
        let mut mm = m;
        for r in 0..3 {
            mm[r][col] = rhs[r];
        }
        det3(&mm) / det
    };
    Some((solve(0), solve(1), solve(2)))
}

fn det3(m: &[[f64; 3]; 3]) -> f64 {
    m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
        - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
        + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0])
}

/// The per-component linear map. Built once per (src, dst) black
/// pair; applied to media-relative PCSXYZ.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BpcScale {
    a: [f64; 3],
    b: [f64; 3],
}

impl BpcScale {
    /// Build from estimated black points. Returns `None` when a
    /// denominator `D50 − bs` is ≤ 0 (a "black" at or above the
    /// white point is not a black point; refused, not clamped).
    #[must_use]
    pub fn new(black_src: Xyz, black_dst: Xyz) -> Option<BpcScale> {
        let w = [D50.x, D50.y, D50.z];
        let s = [black_src.x, black_src.y, black_src.z];
        let d = [black_dst.x, black_dst.y, black_dst.z];
        let mut a = [0.0f64; 3];
        let mut b = [0.0f64; 3];
        for i in 0..3 {
            let den = w[i] - s[i];
            if den <= 0.0 {
                return None;
            }
            a[i] = (w[i] - d[i]) / den;
            b[i] = w[i] * (d[i] - s[i]) / den;
        }
        Some(BpcScale { a, b })
    }

    /// Apply: `out = a·in + b`, per component.
    #[must_use]
    pub fn apply(&self, xyz: Xyz) -> Xyz {
        Xyz {
            x: self.a[0] * xyz.x + self.b[0],
            y: self.a[1] * xyz.y + self.b[1],
            z: self.a[2] * xyz.z + self.b[2],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// D50 is a fixed point of the map by construction — the first
    /// constraint, exactly (arithmetic identity: a·W + b =
    /// (W−d)/(W−s)·W + W·(d−s)/(W−s) = W).
    #[test]
    fn d50_is_fixed_point() {
        let scale = BpcScale::new(
            Xyz {
                x: 0.002,
                y: 0.002,
                z: 0.002,
            },
            PERCEPTUAL_BLACK,
        )
        .unwrap();
        let w = scale.apply(D50);
        assert!((w.x - D50.x).abs() < 1e-15);
        assert!((w.y - D50.y).abs() < 1e-15);
        assert!((w.z - D50.z).abs() < 1e-15);
    }

    /// Source black maps to destination black exactly — the second
    /// constraint (arithmetic identity).
    #[test]
    fn src_black_maps_to_dst_black() {
        let bs = Xyz {
            x: 0.001,
            y: 0.0012,
            z: 0.0009,
        };
        let bd = PERCEPTUAL_BLACK;
        let scale = BpcScale::new(bs, bd).unwrap();
        let out = scale.apply(bs);
        assert!((out.x - bd.x).abs() < 1e-15);
        assert!((out.y - bd.y).abs() < 1e-15);
        assert!((out.z - bd.z).abs() < 1e-15);
    }

    /// The DOCUMENTED DIRECTION (Pass 5's done-when clause 1): mapping
    /// a deep source black to a higher destination black RAISES dark
    /// values — Y at the source black strictly increases, mid-tones
    /// move less, white not at all. Monotone-direction property of
    /// the sourced map.
    #[test]
    fn bpc_raises_blacks_toward_higher_dst_black() {
        let bs = Xyz {
            x: 0.0005,
            y: 0.0005,
            z: 0.0004,
        };
        let scale = BpcScale::new(bs, PERCEPTUAL_BLACK).unwrap();
        let dark = Xyz {
            x: 0.001,
            y: 0.001,
            z: 0.0009,
        };
        let mid = Xyz {
            x: 0.2,
            y: 0.2,
            z: 0.17,
        };
        let dark_out = scale.apply(dark);
        let mid_out = scale.apply(mid);
        assert!(dark_out.y > dark.y, "dark must rise");
        assert!(mid_out.y > mid.y, "mid rises too (linear map)");
        // But proportionally far less: the lift at dark exceeds the
        // lift at mid as a fraction of the value.
        assert!((dark_out.y - dark.y) / dark.y > (mid_out.y - mid.y) / mid.y);
    }

    /// ISO/CD 18619 4.2.2.2's vertex sets, and the SEARCH's reason:
    /// an INVERSE-POLARITY profile (where device 1.0 is dark) must
    /// yield its dark end, which Adobe's fixed device black cannot.
    /// NOTE 2 is the sourced justification for this test existing.
    #[test]
    fn darkest_vertex_survives_inverse_polarity() {
        assert_eq!(vertex_set(1).len(), 2);
        assert_eq!(vertex_set(3).len(), 2);
        assert_eq!(vertex_set(4).len(), 4); // CMYK's four corners
        // Normal polarity: all-zero device is darkest.
        let normal = darkest_vertex(3, |v| Lab {
            l: v[0] * 100.0,
            a: 0.0,
            b: 0.0,
        });
        assert_eq!(normal, vec![0.0, 0.0, 0.0]);
        // Inverse polarity: all-ONE device is darkest.
        let inverse = darkest_vertex(3, |v| Lab {
            l: 100.0 - v[0] * 100.0,
            a: 0.0,
            b: 0.0,
        });
        assert_eq!(inverse, vec![1.0, 1.0, 1.0]);
        // CMYK: 400% ink is darkest among the four corners.
        let cmyk = darkest_vertex(4, |v| Lab {
            l: 100.0 - 25.0 * v.iter().sum::<f64>(),
            a: 0.0,
            b: 0.0,
        });
        assert_eq!(cmyk, vec![1.0, 1.0, 1.0, 1.0]);
    }

    /// 4.2.3: neutralise ALWAYS (Adobe did so for CMYK only) and clip
    /// above 50.
    #[test]
    fn neutralise_and_clip_per_4_2_3() {
        let n = neutralise_and_clip(12.5);
        assert_eq!((n.l, n.a, n.b), (12.5, 0.0, 0.0));
        assert_eq!(neutralise_and_clip(73.0).l, 50.0);
    }

    /// ★ THE ROOT-NOT-VERTEX DELTA, on a synthetic round trip whose
    /// answer is known in closed form. `BT` is built so the shadow
    /// window fits a parabola with a KNOWN positive-gradient root at
    /// x = 4, while its VERTEX sits at x = −6 — Adobe's approximation
    /// would return a negative number clamped to 0, ISO's root
    /// returns 4. The two answers differ by the whole quantity.
    ///
    /// Expectation source: the algebra of the constructed parabola
    /// (y = a(x−4)(x+16)/k), not this code.
    #[test]
    fn iso_takes_the_root_where_adobe_took_the_vertex() {
        // Compose a BT whose outRamp yields y = (x−4)(x+16)/scale in
        // the shadow window: roots at 4 and −16, vertex at −6.
        let bt = |lab: Lab| {
            let x = lab.l;
            let y = ((x - 4.0) * (x + 16.0)) / 1000.0; // 0 at x=4
            Lab {
                l: y.mul_add(100.0, 0.0).clamp(0.0, 100.0),
                a: 0.0,
                b: 0.0,
            }
        };
        let z = estimate_lut_destination_black(
            Lab {
                l: 0.0,
                a: 0.0,
                b: 0.0,
            },
            EstimationIntent::PerceptualOrSaturation,
            bt,
        );
        // The root is what ISO takes; the vertex (−6) would clamp to
        // 0 and be indistinguishable from the give-up path — which is
        // exactly why Adobe's approximation is dangerous rather than
        // merely imprecise.
        assert!(z.l > 0.0, "ISO must not return the give-up value here");
        assert!(z.l < 50.0);
        // 4.2.5.5 returns a NEUTRAL black on every fitted path.
        assert_eq!((z.a, z.b), (0.0, 0.0));
    }

    /// 4.2.5.3 validity, verbatim: a non-increasing ramp is invalid
    /// and the black point SHALL be (0,0,0). Constructed with a
    /// constant BT (first == last, so `first < last` is false).
    #[test]
    fn invalid_ramp_gives_zero_per_4_2_5_3() {
        let z = estimate_lut_destination_black(
            Lab {
                l: 0.0,
                a: 0.0,
                b: 0.0,
            },
            EstimationIntent::RelativeColorimetric,
            |_| Lab {
                l: 20.0,
                a: 0.0,
                b: 0.0,
            },
        );
        assert_eq!((z.l, z.a, z.b), (0.0, 0.0, 0.0));
    }

    /// 4.2.5.4: a near-identity round trip is "straight" at relative
    /// colorimetric, so the black point is the ramp's own minimum —
    /// no curve fitting happens. (The same BT at perceptual skips the
    /// straightness test entirely, per 4.2.5.1, and fits instead.)
    #[test]
    fn straight_midrange_short_circuits_at_relative_only() {
        let identity_ish = |lab: Lab| Lab {
            l: lab.l * 0.99 + 0.5,
            a: 0.0,
            b: 0.0,
        };
        let rel = estimate_lut_destination_black(
            Lab {
                l: 0.0,
                a: 0.0,
                b: 0.0,
            },
            EstimationIntent::RelativeColorimetric,
            identity_ish,
        );
        // ★ 4.2.5.4: the straight branch returns InitialLab
        // UNCHANGED, not the ramp's minimum. This assertion was
        // `(rel - 0.5) < 1e-9` — the ramp minimum — until the corpus
        // read the clause verbatim on 2026-08-12 and found iccce had
        // it wrong; the expectation moved to the clause, and lcms2
        // had been right all along.
        assert_eq!(
            (rel.l, rel.a, rel.b),
            (0.0, 0.0, 0.0),
            "InitialLab carried through"
        );
        // Perceptual: no straightness escape — it fits, and the fit
        // of a straight line hits the |a| < 1e-10 linear branch.
        let per = estimate_lut_destination_black(
            Lab {
                l: 0.0,
                a: 0.0,
                b: 0.0,
            },
            EstimationIntent::PerceptualOrSaturation,
            identity_ish,
        );
        assert!((0.0..=50.0).contains(&per.l));
    }

    /// 4.2.5.4 with a CHROMATIC `InitialLab` — the case the neutral
    /// test above structurally cannot see.
    ///
    /// ★ WHY THIS EXISTS. `straight_midrange_short_circuits_at_relative_only`
    /// asserts the whole triple, but passes `InitialLab = (0,0,0)`, so
    /// the expected `a`/`b` are zero **and so are every wrong answer's**.
    /// It discriminates on `L*` alone. A regression that returned
    /// `Lab { l: initial_lab.l, a: 0.0, b: 0.0 }` — i.e. re-neutralising
    /// the chroma, the single most plausible way to reintroduce the
    /// 2026-08-12 defect — would pass it unchanged. Evidence that could
    /// not have come out differently is not evidence.
    ///
    /// The chromatic case is not hypothetical: this function's own
    /// contract records that **the short-circuit is the only branch
    /// that can return a chromatic black**, because 4.2.5.2.1 zeroes
    /// chroma only for CMYK. On a Gray or RGB LUT destination ISO
    /// itself yields a chromatic `DestinationBlackPoint`, so this test
    /// covers a shape the standard produces, not one invented to be
    /// awkward.
    ///
    /// EXPECTATION SOURCE: ISO/CD 18619 4.2.5.4 final paragraph,
    /// verbatim — *"the DestinationBlackPoint shall be the same as
    /// InitialLab"*. This is normative-rule conformance, not a measured
    /// value and not a cross-check against lcms2.
    ///
    /// TOLERANCE: exact equality, deliberately. "Shall be the same as"
    /// is an identity requirement, and the value is *carried*, never
    /// computed — no arithmetic touches it on this path, so any epsilon
    /// would only weaken a claim that holds bit-exactly. A failure here
    /// means a branch started deriving what it should be passing through.
    #[test]
    fn straight_midrange_carries_chromatic_initial_lab_whole() {
        // Neutral OUTPUT, straight in L*: chroma plays no part in the
        // straightness test (it compares in_ramp/out_ramp L* only), so
        // this still reaches the short-circuit while making the
        // destination model itself chroma-destroying — the harshest
        // setting for the claim, since nothing downstream of `bt` could
        // reconstruct `a`/`b`. They can only survive by being carried.
        let neutralising = |lab: Lab| Lab {
            l: lab.l * 0.99 + 0.5,
            a: 0.0,
            b: 0.0,
        };
        // A plausible chromatic black: slightly blue, as a real
        // display/RGB profile's darkest reproducible colour tends to be.
        let initial = Lab {
            l: 2.7,
            a: 3.4,
            b: -5.1,
        };
        let got = estimate_lut_destination_black(
            initial,
            EstimationIntent::RelativeColorimetric,
            neutralising,
        );
        assert_eq!(
            (got.l, got.a, got.b),
            (2.7, 3.4, -5.1),
            "4.2.5.4 short-circuit must carry InitialLab through whole, \
             chroma included; got L*={} a={} b={}",
            got.l,
            got.a,
            got.b
        );
    }

    /// The chroma clamp and the ramping-to-zero chroma (4.2.5.2.2)
    /// are observable: an InitialLab with |a| > 50 must be clamped,
    /// and the ramp's chroma must reach zero at L* = 100. Asserted
    /// through a BT that reports the chroma it was handed.
    #[test]
    fn chroma_clamped_and_ramped_to_zero() {
        use std::cell::RefCell;
        let seen: RefCell<Vec<(f64, f64)>> = RefCell::new(Vec::new());
        let _ = estimate_lut_destination_black(
            Lab {
                l: 0.0,
                a: 120.0, // beyond the ±50 clamp
                b: -8.0,
            },
            EstimationIntent::PerceptualOrSaturation,
            |lab| {
                seen.borrow_mut().push((lab.l, lab.a));
                Lab {
                    l: lab.l,
                    a: 0.0,
                    b: 0.0,
                }
            },
        );
        let s = seen.borrow();
        let first = s.first().unwrap();
        let last = s.last().unwrap();
        assert!(
            (first.1 - 50.0).abs() < 1e-12,
            "clamped to 50, got {}",
            first.1
        );
        assert!(last.1.abs() < 1e-12, "chroma reaches 0 at white");
        assert!((last.0 - 100.0).abs() < 1e-12);
        assert_eq!(s.len(), 256); // ISO's sample count, not Adobe's 101
    }

    /// The corpus's cross-checked magnitude anchor — in the CORRECT
    /// direction: source black = the perceptual black, destination
    /// black = 0 (the probe's scenario: lcms2's forced v4-perceptual
    /// source into an ideal destination). Black maps BELOW zero and
    /// L* lands at exactly −3.148172 (corpus precision audit; the
    /// difftest confirmed lcms2's observed −3.1482 to 3e-5).
    /// implementation-cross-check class.
    ///
    /// The FIRST version of this test built the opposite direction
    /// (bs=0 → bd=perceptual, which legitimately gives +3.1371 — a
    /// different number) and failed; worse, the engineer's shell
    /// pipeline greped for 'test result' in a way that matched FAILED
    /// lines with exit 0, and a commit went through with the failure.
    /// Both the scenario and the pipeline are fixed; the incident is
    /// recorded here and in the session log.
    #[test]
    fn magnitude_anchor_matches_corpus_audit() {
        let scale = BpcScale::new(
            PERCEPTUAL_BLACK,
            Xyz {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        )
        .unwrap();
        let black_out = scale.apply(Xyz {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });
        // Below-black maps negative; the Lab linear segment extends
        // (deliberately unclamped in iccce-color — the A9 layering).
        let lab = iccce_color::Lab::from_xyz(black_out, D50);
        assert!((lab.l - (-3.148172)).abs() < 1e-5, "L* = {}", lab.l);
    }
}
