//! # The grayTRC computational model — ICC.1:2022 Annex F.2 (normative)
//!
//! Monochrome profiles: one tone curve, `'kTRC'` (grayTRCTag). Per
//! `ICC_Spec/icc/icc__s__computational_models.md` §2 (primary_spec,
//! verbatim): `connection = grayTRC[device]`, and the connection value
//! is a 0..1 scalar that must be **"multiplied by the PCSXYZ or PCSLAB
//! values of the PCS white point"** to become the PCS value. Clause
//! 8.3.4/8.4.4/8.5.3 bind this model normatively (`shall`).
//!
//! ★ The corpus's named trap, honoured here: using the scalar directly
//! as `Y` is right only because `Y_white = 1.0`; using it directly as
//! `X` or `Z` is wrong by the D50 chromaticity — "a monochrome profile
//! renders with a green cast" — so the multiplication is by the FULL
//! white triple. For PCSLAB the white is (100, 0, 0): `L* = t × 100`,
//! `a* = b* = 0`.
//!
//! The inverse is `device = grayTRC⁻¹[connection]` (Eq F.2), with
//! F.1's inversion rules via [`crate::curve::Trc::eval_inverse`]. The
//! connection scalar is recovered from the achromatic channel (`Y/Yn`
//! for PCSXYZ, `L*/100` for PCSLAB) — the same channel NOTE 1 says the
//! tag is usually derived from.

use crate::curve::{CurveError, Trc};
use iccce_color::{D50, Xyz};
use iccce_profile::Profile;
use iccce_profile::num::Signature;
use iccce_profile::tag_types::TagData;

const K_TRC: Signature = Signature(0x6B54_5243); // 'kTRC'
const PCS_LAB: Signature = Signature(0x4C61_6220); // 'Lab '

/// Errors building or running the model.
#[derive(Debug, Clone, PartialEq)]
pub enum GrayError {
    /// No `kTRC` tag, or it decodes to something that isn't a curve.
    NoUsableKtrc,
    Curve(CurveError),
}

impl From<CurveError> for GrayError {
    fn from(e: CurveError) -> Self {
        Self::Curve(e)
    }
}

impl std::fmt::Display for GrayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoUsableKtrc => write!(f, "no usable grayTRC (kTRC) tag"),
            Self::Curve(e) => write!(f, "grayTRC curve: {e}"),
        }
    }
}

/// A monochrome profile's F.2 model. The PCS kind is captured so the
/// white multiplication uses the right triple.
#[derive(Debug, Clone)]
pub struct GrayTrc {
    trc: Trc,
    lab_pcs: bool,
}

impl GrayTrc {
    /// Build from a parsed profile (first `kTRC` per A13).
    pub fn from_profile(profile: &Profile) -> Result<GrayTrc, GrayError> {
        let entry = profile
            .tags
            .iter()
            .find(|t| t.sig == K_TRC)
            .ok_or(GrayError::NoUsableKtrc)?;
        let trc = match profile.decode_tag(entry) {
            Some(Ok(d)) => match d.data {
                TagData::Curve(c) => Trc::from_curve(&c)?,
                TagData::ParametricCurve(p) => Trc::from_parametric(&p)?,
                _ => return Err(GrayError::NoUsableKtrc),
            },
            _ => return Err(GrayError::NoUsableKtrc),
        };
        Ok(GrayTrc {
            trc,
            lab_pcs: profile.header.pcs == PCS_LAB,
        })
    }

    /// Device gray (0..1) → media-relative PCSXYZ. F.1 forward, then
    /// the full-white multiplication (module doc). A Lab-PCS gray
    /// profile's (t×100, 0, 0) converts to XYZ via the D50-relative
    /// formulas so the Chain's unified PCS stays XYZ.
    #[must_use]
    pub fn device_to_pcs(&self, gray: f64) -> Xyz {
        let t = self.trc.eval(gray);
        if self.lab_pcs {
            iccce_color::Lab {
                l: t * 100.0,
                a: 0.0,
                b: 0.0,
            }
            .to_xyz(D50)
        } else {
            Xyz {
                x: t * D50.x,
                y: t * D50.y,
                z: t * D50.z,
            }
        }
    }

    /// Media-relative PCSXYZ → device gray. The connection scalar is
    /// recovered from the achromatic channel (Y/Yn; for a Lab-PCS
    /// profile via L*/100 of the D50-relative Lab), clamped to [0,1],
    /// then inverted per F.1 — chromatic content of the input is
    /// DISCARDED, which is what "the achromatic channel" means for a
    /// monochrome device; stated rather than hidden.
    pub fn pcs_to_device(&self, xyz: Xyz) -> Result<f64, GrayError> {
        let t = if self.lab_pcs {
            iccce_color::Lab::from_xyz(xyz, D50).l / 100.0
        } else {
            xyz.y / D50.y
        };
        Ok(self.trc.eval_inverse(t.clamp(0.0, 1.0))?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Real EIZO gray profile (category (c): local read, skip when
    /// absent): device white → the full D50 triple, NOT (1, 1, 1) —
    /// the green-cast trap's regression, on measured output.
    #[test]
    fn real_gray_profile_white_maps_to_d50() {
        let path = r"C:\Windows\System32\spool\drivers\color\ewgray22.icm";
        let Ok(bytes) = std::fs::read(path) else {
            eprintln!("skipped: EIZO gray profile absent");
            return;
        };
        let profile = Profile::parse(&bytes).unwrap();
        let g = GrayTrc::from_profile(&profile).unwrap();
        let w = g.device_to_pcs(1.0);
        // TRC(1.0) = 1.0 for any sane curve → exactly the white triple
        // (whichever PCS): X/Z must carry the D50 chromaticity.
        assert!((w.x - D50.x).abs() < 1e-3, "X {}", w.x);
        assert!((w.y - D50.y).abs() < 1e-3, "Y {}", w.y);
        assert!((w.z - D50.z).abs() < 1e-3, "Z {}", w.z);

        // Round trip through the real curve (self-consistency).
        for &d in &[0.0, 0.25, 0.5, 0.75, 1.0] {
            let back = g.pcs_to_device(g.device_to_pcs(d)).unwrap();
            assert!((back - d).abs() < 2e-3, "d={d} back={back}");
        }
    }

    /// Synthetic: gamma-2.2 gray, XYZ PCS. Forward at 0.5 is
    /// 0.5^2.2 × D50 — arithmetic on the sourced formula.
    #[test]
    fn synthetic_gray_forward_multiplies_full_white() {
        let g = GrayTrc {
            trc: Trc::Gamma(2.2),
            lab_pcs: false,
        };
        let t = 0.5f64.powf(2.2);
        let p = g.device_to_pcs(0.5);
        assert!((p.x - t * D50.x).abs() < 1e-12);
        assert!((p.z - t * D50.z).abs() < 1e-12);
        let back = g.pcs_to_device(p).unwrap();
        assert!((back - 0.5).abs() < 1e-12);
    }
}
