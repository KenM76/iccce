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
//! ## The estimation step — A LABELLED RE-IMPLEMENTATION (A42)
//!
//! No published document defines black-point ESTIMATION; `bkpt` is
//! untrustworthy (cross-verified). **What this module implements is a
//! subset re-implementation of lcms2's estimation**, and says so:
//!
//! - v4 profile at perceptual intent: the FIXED perceptual black —
//!   the implementations' hybrid triple (0.00336, 0.0034731, 0.00287),
//!   which is byte-identical between lcms2 and ICC's own iccDEV and
//!   differs from ICC.1 Table 16's printed decimals by 0.037 ΔE76 /
//!   exactly zero at 16-bit PCS (**A41** — the first place both
//!   ICC-adjacent implementations agree against the spec's text;
//!   iccce matches the implementations, citing A41).
//! - otherwise: the media-relative transform of device black — the
//!   escape route lcms2 itself takes for v4 matrix/TRC profiles.
//!   lcms2's fuller estimation (thresholded Lab ridge search) is NOT
//!   reproduced; its thresholds are unattributed even in its own
//!   source (**A42**).
//!
//! Consequence, stated: no BPC conformance test with a fixed expected
//! value exists (same standing as perceptual, A27); the grade is
//! agreement with lcms2, and `docs/TOLERANCES.md` owns the number.

use iccce_color::{D50, Xyz};

/// The implementations' fixed v4 perceptual black (see module doc,
/// A41 — deliberately the lcms2/iccDEV triple, not Table 16's text).
pub const PERCEPTUAL_BLACK: Xyz = Xyz {
    x: 0.00336,
    y: 0.0034731,
    z: 0.00287,
};

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

    /// The corpus's cross-checked magnitude anchor: with source black
    /// 0 and the perceptual black as destination, L* of black lands
    /// at ≈ 3.148 (the difftest's predict_bpc_lstar confirmed lcms2's
    /// observed −3.1482 to 3e-5; corpus precision audit gives exact
    /// 3.148172). implementation-cross-check class: the expectation
    /// is lcms2's arithmetic, published via Maria 2013's constraints.
    #[test]
    fn magnitude_anchor_matches_corpus_audit() {
        let scale = BpcScale::new(
            Xyz {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            PERCEPTUAL_BLACK,
        )
        .unwrap();
        let black_out = scale.apply(Xyz {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });
        let lab = iccce_color::Lab::from_xyz(black_out, D50);
        assert!((lab.l - 3.148172).abs() < 1e-5, "L* = {}", lab.l);
    }
}
