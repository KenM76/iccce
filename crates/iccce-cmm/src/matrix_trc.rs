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
    pub const WTPT: Signature = Signature(0x7774_7074); // 'wtpt'
    pub const PCS_XYZ: Signature = Signature(0x5859_5A20); // 'XYZ '
}

/// The four ICC rendering intents, as this transform serves them.
///
/// On a matrix/TRC profile the policy is SOURCED, not invented:
/// ICC.1:2022 Table 25 (via `icc__s__rendering_intents.md` §4) marks
/// the TRC/matrix model column "Colorimetric" — **perceptual and
/// saturation are served by the colorimetric model, and ICC.1
/// specifies no perceptual adjustment for this profile shape.** The
/// fallback order behind that is 8.10.2 a)–d), `shall`-level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Intent {
    /// Served by the media-relative colorimetric model (Table 25).
    Perceptual,
    MediaRelative,
    /// Served by the media-relative colorimetric model (Table 25).
    Saturation,
    /// Media-relative plus the white-point scaling below.
    ///
    /// Formula, VERBATIM ICC.1:2022 Annex D.6.1 (informative; the
    /// normative twin is 6.3.2.2 Eq (1)–(6), reconstructed in the
    /// corpus): `Xa = (Xmw / Xi) Xr` and `Xr = (Xi / Xmw) Xa` —
    /// a per-component diagonal scale, NOT a matrix. `Xmw` =
    /// mediaWhitePointTag as stored; `Xi` = the PCS white
    /// (0.9642/1.0000/0.8249, Table 14). `chad` is NOT un-applied
    /// (6.2.1 NOTE 1 / E.4: it is a provenance record).
    ///
    /// ★ Direction, per corpus spec-defect §12: clause 6.2.3's prose
    /// states the composite ratio BACKWARDS; the equations govern.
    /// Composite src→dst scale is `mw_src / mw_dst` (tested below
    /// against the corpus's own printed intermediates).
    ///
    /// Known consequences, sourced: for a CONFORMING v4 display
    /// profile `wtpt` shall equal the PCS illuminant (9.2.36), so
    /// absolute ≡ media-relative — not a bug. For v2 profiles the
    /// meaning of a non-D50 `wtpt` is corpus A4b (UNVERIFIED —
    /// implementation consensus says use it as stored, which is what
    /// this code does with the fact recorded here).
    Absolute,
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
    /// Absolute intent requested but the profile carries no usable
    /// `wtpt` (mediaWhitePointTag) — the D.6/D.7 scaling has no input.
    /// Refused, not defaulted: substituting Xi would silently make
    /// absolute ≡ relative for a profile where that may be false.
    AbsoluteNeedsWtpt,
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
            Self::AbsoluteNeedsWtpt => write!(
                f,
                "absolute intent requires a mediaWhitePointTag; refused rather than defaulted"
            ),
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
    /// `wtpt` (mediaWhitePointTag) as stored, when present and
    /// well-shaped. Used ONLY by the absolute intent (D.6/D.7);
    /// `None` is fine for every other intent, hence not a build error.
    pub media_white: Option<Xyz>,
    /// A4c: this profile's `wtpt` disagrees with its own colorant sum
    /// while carrying no `chad`. See [`MatrixTrc::white_point_note`].
    pub white_point_inconsistent: bool,
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

        // wtpt is optional at build time (only absolute needs it) and
        // tolerant of absence/misshape — but a present, well-shaped
        // one is captured as stored (A4b: no adaptation second-guessed).
        let media_white = find_first(profile, tag::WTPT).ok().and_then(|entry| {
            match decoded(profile, tag::WTPT, entry) {
                Ok(TagData::Xyz(v)) if v.len() == 1 => Some(Xyz {
                    x: v[0].x.to_f64(),
                    y: v[0].y.to_f64(),
                    z: v[0].z.to_f64(),
                }),
                _ => None,
            }
        });

        // A4c detection (see `white_point_note`). The colorant sum IS
        // the profile's adapted media white in a matrix/TRC model
        // (F.3: device white → the sum of the columns), so comparing
        // it against `wtpt` costs nothing and is decidable from the
        // file's own bytes. Threshold 1e-3 per component: far above
        // s15Fixed16 quantisation (1.5e-5) and authoring rounding
        // (~2e-4, measured on the HP sRGB profile), far below the
        // D65-vs-D50 separation this exists to catch (0.26 in Z).
        let colorant_sum = Xyz {
            x: r[0] + g[0] + b[0],
            y: r[1] + g[1] + b[1],
            z: r[2] + g[2] + b[2],
        };
        let has_chad = profile.tags.iter().any(|t| t.sig == Signature(0x6368_6164)); // 'chad'
        let white_point_inconsistent = media_white.is_some_and(|w| {
            !has_chad
                && ((w.x - colorant_sum.x).abs() > 1e-3
                    || (w.y - colorant_sum.y).abs() > 1e-3
                    || (w.z - colorant_sum.z).abs() > 1e-3)
        });

        Ok(MatrixTrc {
            matrix,
            matrix_inv,
            trc,
            media_white,
            white_point_inconsistent,
        })
    }

    /// A4c disclosure — the residue ICC.1:2001-04 leaves open.
    ///
    /// Annex A.3.1.1 (v2) recommends the profile's AUTHOR set `wtpt`
    /// to the PCS white when the viewer fully adapts, and says
    /// nothing about what a READER should do with a file whose author
    /// did not comply. lcms2 substitutes D50 for any v2 display
    /// profile's `wtpt` (M5) — applying a builder-directed
    /// recommendation at read time, worth **11.2 ΔE2000** on the stock
    /// Windows sRGB profile, which stores D65 while its colorants sum
    /// to D50 with no `chad`.
    ///
    /// **iccce uses `wtpt` as stored (NA-007) and DISCLOSES the
    /// inconsistency instead of choosing silently.** Neither policy is
    /// authorised by a clause; what the standard leaves undecided,
    /// this engine surfaces — the report-don't-repair rule applied one
    /// layer above the parser. `None` when the profile is coherent (or
    /// carries a `chad`, which explains the difference legitimately).
    #[must_use]
    pub fn white_point_note(&self) -> Option<&'static str> {
        self.white_point_inconsistent.then_some(
            "mediaWhitePointTag disagrees with the colorant sum and no chromaticAdaptationTag \
             explains it (A4c): iccce uses wtpt as stored; lcms2 would substitute D50 for a v2 \
             display profile — a difference of up to ~11 ΔE2000 at the ICC-absolute intent",
        )
    }

    /// Device RGB (each component in \[0,1\]) → media-relative PCSXYZ.
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
    /// component is clamped to \[0,1\] BEFORE its inverse TRC (see
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
        self.convert_with_intent(rgb, Intent::MediaRelative)
    }

    /// Convert at a named intent. Perceptual and saturation are served
    /// by the colorimetric model — the SOURCED Table 25 policy for
    /// matrix/TRC profiles, not a shortcut (see [`Intent`]).
    ///
    /// Absolute (D.6/D.7): source PCS is scaled up by
    /// `mw_src / Xi`, destination inverse scales down by
    /// `Xi / mw_dst`; the composite is `mw_src / mw_dst` per
    /// component — the corrected direction (corpus spec-defect §12:
    /// clause 6.2.3's prose has it backwards; the equations govern).
    pub fn convert_with_intent(
        &self,
        rgb: [f64; 3],
        intent: Intent,
    ) -> Result<[f64; 3], ModelError> {
        let pcs = self.src.device_to_pcs(rgb);
        let pcs = match intent {
            Intent::Perceptual | Intent::MediaRelative | Intent::Saturation => pcs,
            Intent::Absolute => {
                // Xi: the PCS white, ICC's 4-figure triple (Table 14;
                // same constant as iccce_color::D50, used everywhere
                // per the mixing-precision rule).
                let mw_src = self.src.media_white.ok_or(ModelError::AbsoluteNeedsWtpt)?;
                let mw_dst = self.dst.media_white.ok_or(ModelError::AbsoluteNeedsWtpt)?;
                // A zero/negative media-white component makes the
                // scale meaningless — same refusal as absence (a
                // degenerate wtpt IS an unusable wtpt).
                if mw_dst.x <= 0.0 || mw_dst.y <= 0.0 || mw_dst.z <= 0.0 {
                    return Err(ModelError::AbsoluteNeedsWtpt);
                }
                // Xa = (mw_src / Xi) · Xr   (D.7, source side)
                // Xr' = (Xi / mw_dst) · Xa  (D.6, destination side)
                // Xi cancels; composite per component: mw_src / mw_dst.
                // Ratio computed FIRST: equal whites give exactly 1.0
                // (IEEE x/x), making the sourced 9.2.36 consequence
                // (absolute ≡ relative for conforming v4 displays)
                // bit-exact rather than within-rounding.
                Xyz {
                    x: pcs.x * (mw_src.x / mw_dst.x),
                    y: pcs.y * (mw_src.y / mw_dst.y),
                    z: pcs.z * (mw_src.z / mw_dst.z),
                }
            }
        };
        self.dst.pcs_to_device(pcs)
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
            media_white: None,
            white_point_inconsistent: false,
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
        // approximation. Bound 1e-3 device units ≈ 1.02× the table's
        // input spacing (1/1023 = 9.775e-4) — i.e. roughly ONE
        // spacing, which per DL-016 means this bound CANNOT
        // discriminate an off-by-one-sample bug; the exact-value
        // sample-point tests carry that duty. (This comment originally
        // claimed "~2× the spacing"; icc-librarian's audit corrected
        // the arithmetic, 2026-08-11.) The difftest prices the
        // approximation properly against lcms2.
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

    /// Build a transform from two synthetic models with given media
    /// whites, identity TRCs, well-conditioned identical matrices.
    fn transform_with_whites(mw_src: Option<Xyz>, mw_dst: Option<Xyz>) -> MatrixTrcTransform {
        let mut src = model(1.0);
        let mut dst = model(1.0);
        src.trc = [Trc::Identity, Trc::Identity, Trc::Identity];
        dst.trc = [Trc::Identity, Trc::Identity, Trc::Identity];
        src.media_white = mw_src;
        dst.media_white = mw_dst;
        MatrixTrcTransform { src, dst }
    }

    /// Absolute composite direction: the scale is mw_src / mw_dst —
    /// verified against the CORPUS'S OWN printed intermediates for
    /// spec-defect §12 (0.7067/0.85 = 0.831412; the backwards reading
    /// 0.85/0.7067 = 1.202773 is asserted absent). Expectation source:
    /// `icc__s__rendering_intents.md` §3 / spec_defects §12 arithmetic
    /// — a cross-check against the corpus derivation, not this code.
    #[test]
    fn absolute_composite_direction_matches_corpus_derivation() {
        let mw = |v: f64| Xyz { x: v, y: v, z: v };
        let t = transform_with_whites(Some(mw(0.7067)), Some(mw(0.85)));
        // Same matrices and identity TRCs: relative conversion is the
        // identity, so the absolute output/input ratio IS the
        // composite scale. Probe mid-range linear value.
        let rel = t
            .convert_with_intent([0.4, 0.4, 0.4], Intent::MediaRelative)
            .unwrap();
        let abs = t
            .convert_with_intent([0.4, 0.4, 0.4], Intent::Absolute)
            .unwrap();
        let ratio = abs[1] / rel[1];
        assert!((ratio - 0.831412).abs() < 5e-6, "ratio {ratio}");
        assert!((ratio - 1.202773).abs() > 0.3, "backwards direction!");
    }

    /// Sourced consequence (9.2.36): a conforming v4 display profile
    /// has wtpt == the PCS illuminant, making absolute ≡ relative
    /// EXACTLY — not a bug. Arithmetic identity given equal whites.
    #[test]
    fn absolute_equals_relative_when_wtpt_is_pcs_white() {
        let t = transform_with_whites(Some(iccce_color::D50), Some(iccce_color::D50));
        for &rgb in &[[0.1, 0.5, 0.9], [1.0, 1.0, 1.0]] {
            let rel = t.convert_with_intent(rgb, Intent::MediaRelative).unwrap();
            let abs = t.convert_with_intent(rgb, Intent::Absolute).unwrap();
            assert_eq!(rel, abs);
        }
    }

    /// Absolute without wtpt refuses by name; a degenerate wtpt is
    /// equally unusable.
    #[test]
    fn absolute_without_wtpt_refused() {
        let t = transform_with_whites(None, Some(iccce_color::D50));
        assert_eq!(
            t.convert_with_intent([0.5, 0.5, 0.5], Intent::Absolute),
            Err(ModelError::AbsoluteNeedsWtpt)
        );
        let degenerate = transform_with_whites(
            Some(iccce_color::D50),
            Some(Xyz {
                x: 0.9,
                y: 0.0,
                z: 0.8,
            }),
        );
        assert_eq!(
            degenerate.convert_with_intent([0.5, 0.5, 0.5], Intent::Absolute),
            Err(ModelError::AbsoluteNeedsWtpt)
        );
    }

    /// Table 25 policy: perceptual and saturation on matrix/TRC are
    /// served by the colorimetric model — identical outputs, sourced
    /// (`icc__s__rendering_intents.md` §4), not a shortcut.
    #[test]
    fn perceptual_and_saturation_serve_colorimetric_on_matrix_trc() {
        let m = model(2.2);
        let t = MatrixTrcTransform {
            src: m.clone(),
            dst: m,
        };
        let rgb = [0.2, 0.6, 0.8];
        let rel = t.convert_with_intent(rgb, Intent::MediaRelative).unwrap();
        assert_eq!(t.convert_with_intent(rgb, Intent::Perceptual).unwrap(), rel);
        assert_eq!(t.convert_with_intent(rgb, Intent::Saturation).unwrap(), rel);
    }

    /// A4c on a REAL file: the stock Windows sRGB v2 display profile
    /// stores wtpt = D65 while its colorants sum to D50 and it carries
    /// no chad — the exact configuration ICC.1:2001-04 A.3.1.1 leaves
    /// undecided for readers, and the cause of lcms2's 11.2 ΔE2000
    /// absolute-intent divergence (M5). iccce discloses it.
    /// Expectation source: the profile's own bytes plus the corpus's
    /// M5 analysis — not this code.
    #[test]
    fn a4c_disclosed_on_the_real_srgb_profile() {
        let path = r"C:\Windows\System32\spool\drivers\color\sRGB Color Space Profile.icm";
        let Ok(bytes) = std::fs::read(path) else {
            eprintln!("skipped: system sRGB profile absent");
            return;
        };
        let profile = Profile::parse(&bytes).unwrap();
        let m = MatrixTrc::from_profile(&profile).unwrap();
        // wtpt is D65-ish; colorants sum to D50-ish; no chad.
        let w = m.media_white.expect("sRGB carries wtpt");
        assert!(w.z > 1.0, "wtpt Z {} should be D65-like (~1.089)", w.z);
        let sum = m.matrix.apply([1.0, 1.0, 1.0]);
        assert!(sum[2] < 0.9, "colorant sum Z {} should be D50-like", sum[2]);
        assert!(m.white_point_inconsistent);
        assert!(m.white_point_note().unwrap().contains("A4c"));
    }

    /// The disclosure must STAY SILENT on a coherent profile — a note
    /// that fires on everything discloses nothing. The committed
    /// synthetic fixture is coherent by construction (wtpt = D50 =
    /// its colorant sum, plus an identity chad).
    ///
    /// This test originally used AdobeRGB1998.icc, assuming it
    /// carried a chad. It does not — and the failure produced a
    /// finding bigger than the test: a sweep of this machine's
    /// profiles shows **AdobeRGB1998, AppleRGB, PAL_SECAM, SMPTE-C,
    /// ewrgb18, ewsrgb and the stock sRGB all store wtpt = D65 with
    /// colorants summing to D50 and no chad.** The A4c configuration
    /// is the v2 authoring NORM, not an outlier — which is exactly
    /// why lcms2 substitutes D50, and why iccce's disclosure will
    /// fire often and must therefore be worth reading.
    #[test]
    fn a4c_silent_on_a_coherent_profile() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/synthetic/v2-rgb-matrix-trc-curv.icc"
        );
        let bytes = std::fs::read(path).expect("committed fixture");
        let profile = Profile::parse(&bytes).unwrap();
        let m = MatrixTrc::from_profile(&profile).unwrap();
        assert!(!m.white_point_inconsistent);
        assert!(m.white_point_note().is_none());
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
