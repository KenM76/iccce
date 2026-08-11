//! # CIE XYZ and xyY
//!
//! Conversions per `ICC_Spec/cie/cie__ref__colorimetry_core.md` §1
//! (cross-verified against lcms2 `cmsXYZ2xyY` and the standard
//! relations). CIE 15 itself is paywalled and not sourced; formulas
//! cite the corpus's cross-verified extraction.

/// CIE tristimulus values. Y = 1.0 is the nominal white luminance in
/// this crate (the ICC convention); no scaling to 100 anywhere
/// internally — one convention, everywhere, so a factor-of-100 slip is
/// impossible to introduce silently.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Xyz {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Chromaticity + luminance. `x`,`y` are the chromaticity coordinates;
/// `luma_y` is the tristimulus Y (capital Y in the literature — renamed
/// here because Rust cannot case-distinguish `y` and `Y` in one struct,
/// and a silent `y`-for-`Y` mixup is exactly the bug the naming must
/// prevent).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XyY {
    pub x: f64,
    pub y: f64,
    pub luma_y: f64,
}

impl Xyz {
    /// XYZ → xyY: `x = X/(X+Y+Z)`, `y = Y/(X+Y+Z)`, Y carried
    /// (`cie__ref__colorimetry_core.md` §1, cross-verified).
    ///
    /// Returns `None` when `X+Y+Z == 0` — black has no defined
    /// chromaticity, and the corpus notes both reference codebases can
    /// divide by zero here; iccce must not.
    #[must_use]
    pub fn to_xyy(&self) -> Option<XyY> {
        let sum = self.x + self.y + self.z;
        if sum == 0.0 {
            return None;
        }
        Some(XyY {
            x: self.x / sum,
            y: self.y / sum,
            luma_y: self.y,
        })
    }
}

impl XyY {
    /// xyY → XYZ: `X = (x/y)·Y`, `Z = ((1−x−y)/y)·Y`
    /// (`cie__ref__colorimetry_core.md` §1).
    ///
    /// Returns `None` when `y == 0` (chromaticity undefined ↔
    /// luminance-free point; the guard mirrors [`Xyz::to_xyy`]).
    #[must_use]
    pub fn to_xyz(&self) -> Option<Xyz> {
        if self.y == 0.0 {
            return None;
        }
        Some(Xyz {
            x: (self.x / self.y) * self.luma_y,
            y: self.luma_y,
            z: ((1.0 - self.x - self.y) / self.y) * self.luma_y,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::illuminant::{D50, D65_XY};

    /// Round trip XYZ → xyY → XYZ is an arithmetic identity (corpus §5:
    /// self-checking invariant, legitimate expectation class).
    #[test]
    fn xyy_round_trip_is_identity() {
        let back = D50.to_xyy().unwrap().to_xyz().unwrap();
        assert!((back.x - D50.x).abs() < 1e-14);
        assert!((back.y - D50.y).abs() < 1e-14);
        assert!((back.z - D50.z).abs() < 1e-14);
    }

    /// FINDING (2026-08-11, reported to icc-spec-librarian): the
    /// corpus's derived D50 chromaticity (0.34567, 0.35850) is NOT the
    /// chromaticity of its own sourced 4-figure triple — it matches the
    /// higher-precision D50 (0.96422/1/0.82521) instead, i.e. the
    /// corpus's own derivation committed the mixing-precision trap its
    /// §2 warns about. From the sourced (0.9642, 1.0000, 0.8249):
    /// x = 0.9642/2.7891 = 0.345703, y = 1/2.7891 = 0.358539.
    /// This test asserts the CORRECT derivation from the sourced
    /// triple — a consistency check on arithmetic, not a published
    /// expectation (the corpus marks all such values DERIVED).
    #[test]
    fn d50_chromaticity_derives_from_sourced_triple() {
        let c = D50.to_xyy().unwrap();
        assert!((c.x - 0.345703).abs() < 5e-7, "x = {}", c.x);
        assert!((c.y - 0.358539).abs() < 5e-7, "y = {}", c.y);
    }

    /// Same status: the corpus derives D65 XYZ ≈ (0.95046, 1, 1.08906)
    /// from the single-source (x,y) and marks it DERIVED. Consistency
    /// check on shared arithmetic, not ground truth.
    #[test]
    fn d65_xyz_matches_corpus_derivation() {
        let w = XyY {
            x: D65_XY.0,
            y: D65_XY.1,
            luma_y: 1.0,
        }
        .to_xyz()
        .unwrap();
        assert!((w.x - 0.95046).abs() < 5e-6);
        assert!((w.z - 1.08906).abs() < 5e-6);
    }

    /// Black: no defined chromaticity, and no panic.
    #[test]
    fn black_has_no_chromaticity() {
        let black = Xyz {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        assert!(black.to_xyy().is_none());
    }
}
