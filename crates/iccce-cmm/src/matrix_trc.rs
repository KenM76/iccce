//! # The matrix/TRC computational model — ICC.1:2022 Annex F.3 (normative)
//!
//! The analytic path for three-component matrix-based profiles — most
//! display profiles (sRGB, Adobe RGB, Display P3). Per
//! `ICC_Spec/icc/icc__s__computational_models.md` (primary_spec):
//!
//! ```text
//!   forward (device → PCS):  linear_i = TRC_i(device_i)
//!                            XYZ = M · linear        (F.3–F.6)
//!   inverse (PCS → device):  linear = M⁻¹ · XYZ
//!                            clamp each linear_i to [0,1]   ★ BEFORE
//!                            device_i = TRC_i⁻¹(linear_i)   (F.8–F.16)
//! ```
//!
//! ★ **The clamp comes BEFORE the inverse TRC, normatively** — clamping
//! the device value afterwards differs whenever TRC⁻¹ is non-identity
//! near the endpoints, which is always for a gamma curve. Symptom of
//! the wrong order: "the gamut boundary is subtly the wrong shape",
//! and it is quiet (corpus F.3 notes).
//!
//! **PCSXYZ only, normatively**: "Only the PCSXYZ encoding can be used
//! with matrix/TRC models" (F.3 verbatim). A Lab-PCS profile without
//! LUTs is refused by name, not approximated.
//!
//! ## What the matrix is
//!
//! Columns are the `rXYZ`/`gXYZ`/`bXYZ` colorant values as stored —
//! which in a well-formed profile are already media-relative and
//! D50-referenced (their sum is the PCS white). Consequence: the
//! forward model lands directly in media-relative PCSXYZ, so chaining
//! source-forward → destination-inverse IS the media-relative
//! colorimetric conversion, with no further adaptation step. The
//! `wtpt`/`chad` machinery matters for the ICC-absolute intent, which
//! is NOT implemented in this Pass (recorded remainder — its formula
//! has not been sourced from the corpus yet, and it will not be
//! written from memory).

use crate::curve::{CurveError, Trc};
use iccce_color::{Mat3, Xyz};
use iccce_profile::num::Signature;
use iccce_profile::tag_types::TagData;
use iccce_profile::{Profile, TagEntry};

/// Tag (not type) signatures the model consumes.
mod tag {
    use iccce_profile::num::Signature;
    pub const R_XYZ: Signature = Signature(0x7258_595A); // 'rXYZ'
    pub const G_XYZ: Signature = Signature(0x6758_595A); // 'gXYZ'
    pub const B_XYZ: Signature = Signature(0x6258_595A); // 'bXYZ'
    pub const R_TRC: Signature = Signature(0x7254_5243); // 'rTRC'
    pub const G_TRC: Signature = Signature(0x6754_5243); // 'gTRC'
    pub const B_TRC: Signature = Signature(0x6254_5243); // 'bTRC'
    pub const PCS_XYZ: Signature = Signature(0x5859_5A20); // 'XYZ '
}

/// Why a profile could not become a matrix/TRC model, or a conversion
/// could not run.
#[derive(Debug, Clone, PartialEq)]
pub enum ModelError {
    /// The PCS is not `'XYZ '` — F.3 verbatim permits only PCSXYZ for
    /// matrix/TRC. Refused by name.
    PcsNotXyzRefused { pcs: Signature },
    /// A required tag is absent (`rXYZ`/`gXYZ`/`bXYZ`/`rTRC`/`gTRC`/
    /// `bTRC` — clause 8.4.3's required set for three-component
    /// matrix-based display profiles, `icc__s__required_tags.md`).
    MissingTag { sig: Signature },
    /// The tag exists but decodes to the wrong shape (e.g. a colorant
    /// tag without exactly one XYZNumber, or a TRC that is neither
    /// curveType nor parametricCurveType).
    WrongTagShape { sig: Signature },
    /// The tag's bytes are undecodable (wraps the profile layer's
    /// refusal, stringified to keep the layers decoupled).
    TagUndecodable { sig: Signature, reason: String },
    /// The colorant matrix is singular — no PCS→device direction
    /// exists.
    SingularMatrix,
    /// A curve failed (constant, non-monotonic, unsupported inverse…).
    Curve(CurveError),
}

impl std::fmt::Display for ModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PcsNotXyzRefused { pcs } => write!(
                f,
                "matrix/TRC model requires PCSXYZ (Annex F.3, normative); profile PCS is {pcs}"
            ),
            Self::MissingTag { sig } => write!(f, "required tag {sig} absent"),
            Self::WrongTagShape { sig } => {
                write!(f, "tag {sig} has the wrong shape for this model")
            }
            Self::TagUndecodable { sig, reason } => write!(f, "tag {sig} undecodable: {reason}"),
            Self::SingularMatrix => write!(f, "colorant matrix is singular"),
            Self::Curve(e) => write!(f, "curve: {e}"),
        }
    }
}

impl From<CurveError> for ModelError {
    fn from(e: CurveError) -> Self {
        Self::Curve(e)
    }
}

/// A three-component matrix/TRC model extracted from a profile.
#[derive(Debug, Clone)]
pub struct MatrixTrc {
    /// Columns are rXYZ, gXYZ, bXYZ as stored (media-relative, D50).
    pub matrix: Mat3,
    /// Cached inverse (computed once at build; `f64`, from the stored
    /// forward values — same posture as the Bradford inverse).
    matrix_inv: Mat3,
    pub trc: [Trc; 3],
}

impl MatrixTrc {
    /// Build from a parsed profile. Consumes the FIRST entry per tag
    /// signature when duplicates exist — the recorded A13 choice,
    /// which the profile layer has already reported as a malformation.
    pub fn from_profile(profile: &Profile) -> Result<MatrixTrc, ModelError> {
        if profile.header.pcs != tag::PCS_XYZ {
            return Err(ModelError::PcsNotXyzRefused {
                pcs: profile.header.pcs,
            });
        }

        let col = |sig: Signature| -> Result<[f64; 3], ModelError> {
            let entry = find_first(profile, sig)?;
            match decoded(profile, sig, entry)? {
                TagData::Xyz(v) if v.len() == 1 => {
                    Ok([v[0].x.to_f64(), v[0].y.to_f64(), v[0].z.to_f64()])
                }
                _ => Err(ModelError::WrongTagShape { sig }),
            }
        };
        let r = col(tag::R_XYZ)?;
        let g = col(tag::G_XYZ)?;
        let b = col(tag::B_XYZ)?;
        // Columns are the colorants: row i of the matrix takes
        // (X_i of r, X_i of g, X_i of b).
        let matrix = Mat3 {
            rows: [[r[0], g[0], b[0]], [r[1], g[1], b[1]], [r[2], g[2], b[2]]],
        };
        let matrix_inv = matrix.inverse().ok_or(ModelError::SingularMatrix)?;

        let curve = |sig: Signature| -> Result<Trc, ModelError> {
            let entry = find_first(profile, sig)?;
            match decoded(profile, sig, entry)? {
                TagData::Curve(c) => Ok(Trc::from_curve(&c)?),
                TagData::ParametricCurve(p) => Ok(Trc::from_parametric(&p)?),
                _ => Err(ModelError::WrongTagShape { sig }),
            }
        };
        let trc = [curve(tag::R_TRC)?, curve(tag::G_TRC)?, curve(tag::B_TRC)?];

        Ok(MatrixTrc {
            matrix,
            matrix_inv,
            trc,
        })
    }

    /// Device RGB (each component in [0,1]) → media-relative PCSXYZ.
    /// F.3–F.6.
    #[must_use]
    pub fn device_to_pcs(&self, rgb: [f64; 3]) -> Xyz {
        let linear = [
            self.trc[0].eval(rgb[0]),
            self.trc[1].eval(rgb[1]),
            self.trc[2].eval(rgb[2]),
        ];
        let v = self.matrix.apply(linear);
        Xyz {
            x: v[0],
            y: v[1],
            z: v[2],
        }
    }

    /// Media-relative PCSXYZ → device RGB. F.8–F.16: each linear
    /// component is clamped to [0,1] BEFORE its inverse TRC (see
    /// module doc for why the order is load-bearing).
    pub fn pcs_to_device(&self, xyz: Xyz) -> Result<[f64; 3], ModelError> {
        let linear = self.matrix_inv.apply([xyz.x, xyz.y, xyz.z]);
        let mut rgb = [0.0f64; 3];
        for i in 0..3 {
            rgb[i] = self.trc[i].eval_inverse(linear[i].clamp(0.0, 1.0))?;
        }
        Ok(rgb)
    }
}

/// A source→destination conversion between two matrix/TRC profiles at
/// the media-relative colorimetric intent — the only intent this Pass
/// implements (see module doc; absolute is a recorded remainder
/// pending a sourced formula; perceptual/saturation have no meaning on
/// matrix/TRC profiles beyond media-relative, which is what lcms2 does
/// with them too — but THAT equivalence is an unverified expectation
/// here, not a claim; the differential test owns it).
#[derive(Debug, Clone)]
pub struct MatrixTrcTransform {
    pub src: MatrixTrc,
    pub dst: MatrixTrc,
}

impl MatrixTrcTransform {
    pub fn new(src: &Profile, dst: &Profile) -> Result<Self, ModelError> {
        Ok(Self {
            src: MatrixTrc::from_profile(src)?,
            dst: MatrixTrc::from_profile(dst)?,
        })
    }

    /// Convert one RGB triple, source device space → destination
    /// device space, media-relative.
    pub fn convert(&self, rgb: [f64; 3]) -> Result<[f64; 3], ModelError> {
        self.dst.pcs_to_device(self.src.device_to_pcs(rgb))
    }
}

fn find_first(profile: &Profile, sig: Signature) -> Result<&TagEntry, ModelError> {
    profile
        .tags
        .iter()
        .find(|t| t.sig == sig)
        .ok_or(ModelError::MissingTag { sig })
}

fn decoded(profile: &Profile, sig: Signature, entry: &TagEntry) -> Result<TagData, ModelError> {
    match profile.decode_tag(entry) {
        None => Err(ModelError::TagUndecodable {
            sig,
            reason: "data out of bounds".to_string(),
        }),
        Some(Err(e)) => Err(ModelError::TagUndecodable {
            sig,
            reason: e.to_string(),
        }),
        Some(Ok(d)) => Ok(d.data),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a synthetic matrix/TRC model directly (no profile bytes):
    /// gamma-2.2-ish TRCs and a well-conditioned matrix.
    fn model(gamma: f64) -> MatrixTrc {
        let matrix = Mat3 {
            rows: [
                [0.4361, 0.3851, 0.1431],
                [0.2225, 0.7169, 0.0606],
                [0.0139, 0.0971, 0.7141],
            ],
        };
        MatrixTrc {
            matrix,
            matrix_inv: matrix.inverse().unwrap(),
            trc: [Trc::Gamma(gamma), Trc::Gamma(gamma), Trc::Gamma(gamma)],
        }
    }

    /// Round trip device → PCS → device is an arithmetic identity for
    /// invertible curves and matrix; tolerance is f64 noise.
    #[test]
    fn round_trip_is_identity() {
        let m = model(2.19921875);
        for &rgb in &[[0.0, 0.0, 0.0], [1.0, 1.0, 1.0], [0.2, 0.5, 0.8]] {
            let back = m.pcs_to_device(m.device_to_pcs(rgb)).unwrap();
            for i in 0..3 {
                assert!((back[i] - rgb[i]).abs() < 1e-12, "{rgb:?} -> {back:?}");
            }
        }
    }

    /// White (1,1,1) maps to the colorant sum — for D50-referenced
    /// colorants that is the PCS white. Arithmetic property of the
    /// model (TRC(1) = 1, then row sums).
    #[test]
    fn white_maps_to_colorant_sum() {
        let m = model(2.2);
        let w = m.device_to_pcs([1.0, 1.0, 1.0]);
        assert!((w.x - (0.4361 + 0.3851 + 0.1431)).abs() < 1e-12);
        assert!((w.y - 1.0).abs() < 1e-12);
    }

    /// F.8–F.16 order: out-of-gamut PCS produces a NEGATIVE linear
    /// component; the clamp-to-0 happens BEFORE TRC⁻¹, so the device
    /// value is exactly TRC⁻¹(0) = 0 — clamping after would first take
    /// 0.powf(1/g) of a negative (NaN) or clamp a NaN. This asserts
    /// the measured output, not the code path.
    #[test]
    fn out_of_gamut_clamps_before_inverse_trc() {
        let m = model(2.2);
        // A saturated green far outside these primaries.
        let xyz = Xyz {
            x: 0.05,
            y: 0.9,
            z: 0.02,
        };
        let rgb = m.pcs_to_device(xyz).unwrap();
        for (i, c) in rgb.iter().enumerate() {
            assert!(c.is_finite(), "channel {i} not finite: {c}");
            assert!((0.0..=1.0).contains(c), "channel {i} out of range: {c}");
        }
        // Red and blue are driven negative by this XYZ → exactly 0.
        assert_eq!(rgb[0], 0.0);
        assert_eq!(rgb[2], 0.0);
    }

    /// End-to-end on the REAL system sRGB profile — category (c) per
    /// LEGAL.md §3: read locally, skipped when absent, never
    /// committed.
    #[test]
    fn system_srgb_profile_end_to_end() {
        let path = r"C:\Windows\System32\spool\drivers\color\sRGB Color Space Profile.icm";
        let Ok(bytes) = std::fs::read(path) else {
            eprintln!("skipped: system sRGB profile absent");
            return;
        };
        let profile = Profile::parse(&bytes).unwrap();
        let m = MatrixTrc::from_profile(&profile).unwrap();

        // Device white → PCS: colorants sum to (approximately) the
        // PCS white. FINDING (2026-08-11, first run): this profile's
        // colorant Z sums to 0.825089, i.e. 1.9e-4 from ICC's
        // 4-figure D50 — the 1998 author's own white rounding, a fact
        // about the FILE, not about iccce. A quantization-based 1e-4
        // tolerance was therefore a claim the file never made. The
        // tolerance below is 1e-2, justified by what the check exists
        // to catch: D65-referenced colorants would put Z at ~1.089
        // (0.26 away, 26x the bound), while authoring spread is
        // ~2e-4 (50x inside it). Discriminates the failure mode;
        // tolerates the file.
        let w = m.device_to_pcs([1.0, 1.0, 1.0]);
        assert!((w.x - 0.9642).abs() < 1e-2, "X {}", w.x);
        assert!((w.y - 1.0000).abs() < 1e-3, "Y {}", w.y);
        assert!((w.z - 0.8249).abs() < 1e-2, "Z {}", w.z);

        // Round trip through the 1024-entry TRC table. Self-consistency
        // class: prices the table-quantisation + F.1-inversion
        // approximation. Bound 1e-3 device units — justified as ~2×
        // the table's input spacing (1/1023), the worst linear-interp
        // inversion mismatch scale for a smooth curve; measured
        // residuals are far below it, and the difftest will price this
        // properly against lcms2 (recorded in the ROADMAP).
        for &rgb in &[[0.0, 0.0, 0.0], [1.0, 1.0, 1.0], [0.25, 0.5, 0.75]] {
            let back = m.pcs_to_device(m.device_to_pcs(rgb)).unwrap();
            for i in 0..3 {
                assert!(
                    (back[i] - rgb[i]).abs() < 1e-3,
                    "{rgb:?} -> {back:?} (channel {i})"
                );
            }
        }
    }

    /// A Lab-PCS profile is refused BY NAME for this model (F.3:
    /// PCSXYZ only) — the CMYK press profile is exactly that shape.
    #[test]
    fn lab_pcs_profile_refused_by_name() {
        let path = r"C:\Windows\System32\spool\drivers\color\USWebCoatedSWOP.icc";
        let Ok(bytes) = std::fs::read(path) else {
            eprintln!("skipped: SWOP profile absent");
            return;
        };
        let profile = Profile::parse(&bytes).unwrap();
        let err = MatrixTrc::from_profile(&profile).unwrap_err();
        assert!(matches!(err, ModelError::PcsNotXyzRefused { .. }));
        assert!(err.to_string().contains("Annex F.3"));
    }
}
