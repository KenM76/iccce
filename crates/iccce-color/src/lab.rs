//! # CIE Lab and LCh
//!
//! XYZ ↔ Lab ↔ LCh per `ICC_Spec/icc/icc__s__pcs_encoding.md` §3–4
//! (cross-verified: ICC `icXYZtoLab`/`icLabtoXYZ` against lcms2
//! `cmsXYZ2Lab`/`cmsLab2XYZ`). L* is 0–100, a*/b* unbounded around 0 —
//! the *colorimetric* values; the 16-bit PCS *encodings* of Lab (where
//! the v2/v4 divergence lives) belong to the profile layer, not here.
//!
//! ## Named DEVIATION — the f(t) breakpoint (ambiguity A11, resolved)
//!
//! With ICC.1:2022 ingested (2026-08-11), A11 is resolved and the
//! status of this choice sharpened: **ICC.1:2022's own normative text
//! writes the breakpoint as the decimal `0,008 856`** (delegating the
//! underlying colorimetry to ISO 13655), while lcms2 uses the exact
//! rationals `(24/116)³` / `24/116`, and ICC's reference code uses the
//! decimal with an inverse threshold inconsistent with its own forward
//! one.
//!
//! **iccce adopts the EXACT RATIONAL form, and this is now a stated
//! deviation from ICC.1:2022's printed constant** — not merely a pick
//! between disagreeing implementations. Why deviate: the rational form
//! makes `f` and `f⁻¹` exact mutual inverses at the breakpoint (the
//! decimal form provably cannot, and ICC's reference code demonstrates
//! the resulting inconsistency), and the rational is the modern
//! CIE 15 / ISO 11664-4 statement per the corpus (that clause itself
//! remains paywalled/unsourced).
//!
//! **Cost, stated per project rule 4:** versus the decimal-threshold
//! form, the difference is ~10⁻⁷ in `f`, i.e. **~10⁻⁵ in L\* — far
//! below any measurable ΔE** (the perceptibility anchor is 1.0 ΔE2000;
//! see `docs/TOLERANCES.md`). It matters only to bit-exact round-trip
//! comparisons against implementations using the other form.
//! Recorded in `docs/NUMERIC_CLAIMS.md` as a named approximation.

use crate::xyz::Xyz;

/// CIE L\*a\*b\*.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lab {
    pub l: f64,
    pub a: f64,
    pub b: f64,
}

/// CIE L\*C\*h — cylindrical Lab. `h` in degrees, `[0, 360)`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lch {
    pub l: f64,
    pub c: f64,
    pub h: f64,
}

/// The Lab transfer function `f(t)`: cube root above the breakpoint,
/// linear below. Exact rational constants — see module doc (A11).
///
/// Sourced form: lcms2 `cmspcs.c` `f()` verbatim —
/// `Limit = (24/116)³`, linear branch `(841/108)·t + 16/116`.
fn f(t: f64) -> f64 {
    const LIMIT: f64 = (24.0 / 116.0) * (24.0 / 116.0) * (24.0 / 116.0);
    if t <= LIMIT {
        (841.0 / 108.0) * t + 16.0 / 116.0
    } else {
        t.cbrt()
    }
}

/// Inverse of [`f`]. Sourced form: lcms2 `cmspcs.c` `f_1()` verbatim —
/// `Limit = 24/116`, linear branch `(108/841)·(t − 16/116)`.
///
/// No clamp to zero below the linear segment: ICC's own reference code
/// makes negative-XYZ clamping a compile-time *option* (corpus A9/A11 —
/// the reference implementation declines to decide), so iccce's colour
/// layer computes the unclamped value and leaves gamut policy to the
/// CMM layer, where it can be a named, per-transform decision.
fn f_inv(t: f64) -> f64 {
    const LIMIT: f64 = 24.0 / 116.0;
    if t <= LIMIT {
        (108.0 / 841.0) * (t - 16.0 / 116.0)
    } else {
        t * t * t
    }
}

impl Lab {
    /// XYZ → Lab relative to white point `white` (`Xn, Yn, Zn`):
    ///
    /// ```text
    /// L* = 116·f(Y/Yn) − 16
    /// a* = 500·(f(X/Xn) − f(Y/Yn))
    /// b* = 200·(f(Y/Yn) − f(Z/Zn))
    /// ```
    ///
    /// (`icc__s__pcs_encoding.md` §3, cross-verified 2 sources.) The
    /// white point is an explicit parameter — both reference codebases
    /// *default* it to D50, and a hidden default is how a D65-relative
    /// Lab silently gets computed against the wrong white.
    #[must_use]
    pub fn from_xyz(xyz: Xyz, white: Xyz) -> Lab {
        let fx = f(xyz.x / white.x);
        let fy = f(xyz.y / white.y);
        let fz = f(xyz.z / white.z);
        Lab {
            l: 116.0 * fy - 16.0,
            a: 500.0 * (fx - fy),
            b: 200.0 * (fy - fz),
        }
    }

    /// Lab → XYZ: `fy = (L*+16)/116`, `fx = fy + a*/500`,
    /// `fz = fy − b*/200`, then `f⁻¹` times the white point
    /// (`icc__s__pcs_encoding.md` §3, cross-verified 2 sources).
    #[must_use]
    pub fn to_xyz(&self, white: Xyz) -> Xyz {
        let fy = (self.l + 16.0) / 116.0;
        let fx = fy + self.a / 500.0;
        let fz = fy - self.b / 200.0;
        Xyz {
            x: f_inv(fx) * white.x,
            y: f_inv(fy) * white.y,
            z: f_inv(fz) * white.z,
        }
    }

    /// Lab → LCh: `C* = √(a*²+b*²)`, `h = atan2(b*,a*)` in degrees
    /// wrapped to `[0, 360)` (`icc__s__pcs_encoding.md` §4,
    /// cross-verified against ICC `icLab2Lch`).
    #[must_use]
    pub fn to_lch(&self) -> Lch {
        let c = self.a.hypot(self.b);
        let mut h = self.b.atan2(self.a).to_degrees();
        // ICC normalises only negative h; atan2 returns (−180, 180] so
        // one wrap suffices and h = 360.0 is unrepresentable — the
        // corpus notes an add-then-wrap variant can emit exactly 360.0.
        if h < 0.0 {
            h += 360.0;
        }
        Lch { l: self.l, c, h }
    }
}

impl Lch {
    /// LCh → Lab: `a* = C*·cos h`, `b* = C*·sin h`
    /// (`icc__s__pcs_encoding.md` §4).
    #[must_use]
    pub fn to_lab(&self) -> Lab {
        let hr = self.h.to_radians();
        Lab {
            l: self.l,
            a: self.c * hr.cos(),
            b: self.c * hr.sin(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::illuminant::D50;

    /// The white point itself maps to L* = 100, a* = b* = 0 exactly:
    /// X/Xn = Y/Yn = Z/Zn = 1, f(1) = 1, 116−16 = 100. An arithmetic
    /// identity (corpus §5 expectation class), not a measurement.
    #[test]
    fn white_maps_to_l100_exactly() {
        let lab = Lab::from_xyz(D50, D50);
        assert_eq!(lab.l, 100.0);
        assert_eq!(lab.a, 0.0);
        assert_eq!(lab.b, 0.0);
    }

    /// Y = 0 maps to L* = 0 exactly under the rational form:
    /// f(0) = 16/116, L* = 116·(16/116) − 16 = 0. Arithmetic identity —
    /// and one that only holds exactly BECAUSE the linear segment is
    /// present; a cube-root-only f gives f(0) = 0, L* = −16.
    #[test]
    fn black_maps_to_l0_exactly() {
        let lab = Lab::from_xyz(
            Xyz {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            D50,
        );
        assert_eq!(lab.l, 0.0);
    }

    /// f and f⁻¹ are exact mutual inverses at and around the
    /// breakpoint — the property the rational form was chosen FOR
    /// (module doc, A11). Probes both branches and the joint.
    #[test]
    fn f_round_trips_across_breakpoint() {
        const LIMIT_T: f64 = (24.0 / 116.0) * (24.0 / 116.0) * (24.0 / 116.0);
        for &t in &[
            0.0,
            LIMIT_T / 2.0,
            LIMIT_T,          // exactly the breakpoint
            LIMIT_T * 1.0001, // just above
            0.18,
            0.5,
            1.0,
        ] {
            let back = f_inv(f(t));
            assert!(
                (back - t).abs() < 1e-15,
                "f_inv(f({t})) = {back}, drift {}",
                back - t
            );
        }
    }

    /// Full XYZ → Lab → XYZ round trip, both branches of f.
    /// Arithmetic-identity class; tolerance is f64 arithmetic noise,
    /// not a perceptual claim.
    #[test]
    fn xyz_lab_round_trip() {
        for &xyz in &[
            Xyz {
                x: 0.20,
                y: 0.30,
                z: 0.40,
            },
            // Below the breakpoint in all channels (linear branch).
            Xyz {
                x: 0.004,
                y: 0.005,
                z: 0.003,
            },
        ] {
            let back = Lab::from_xyz(xyz, D50).to_xyz(D50);
            assert!((back.x - xyz.x).abs() < 1e-12);
            assert!((back.y - xyz.y).abs() < 1e-12);
            assert!((back.z - xyz.z).abs() < 1e-12);
        }
    }

    /// Lab ↔ LCh round trip; h wraps into [0, 360).
    #[test]
    fn lab_lch_round_trip_and_hue_range() {
        let lab = Lab {
            l: 50.0,
            a: -20.0,
            b: -30.0, // third quadrant → atan2 negative → wrapped
        };
        let lch = lab.to_lch();
        assert!((0.0..360.0).contains(&lch.h));
        let back = lch.to_lab();
        assert!((back.a - lab.a).abs() < 1e-12);
        assert!((back.b - lab.b).abs() < 1e-12);
    }
}
