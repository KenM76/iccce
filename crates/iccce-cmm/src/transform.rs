//! # The source→destination transform chain — Pass 4 assembly, stage 2
//!
//! Chains a source profile's device→PCS model into a destination
//! profile's PCS→device model, unifying the PCS in between. This is
//! what makes CMYK→RGB through a real press profile evaluable.
//!
//! ## Tag selection — the SOURCED fallback order
//!
//! ICC.1:2022 clause 8.10.2 a)–d), `shall`-level, via
//! `ICC_Spec/icc/icc__s__rendering_intents.md` §4:
//!
//! 1. `D2Bx`/`B2Dx` — multiProcessingElements (NOT implemented; when a
//!    profile carries one, this stage proceeds to step 2 — which is a
//!    DEVIATION from the shall-order, recorded here and in the model's
//!    `notes`, until `mpet` support exists. Skipping silently would be
//!    the sin; skipping loudly is the recorded state.)
//! 2. `A2Bx`/`B2Ax` for the requested intent (x = 0 perceptual,
//!    1 colorimetric, 2 saturation).
//! 3. `A2B0`/`B2A0`.
//! 4. The TRC + colorant matrix model.
//!
//! ## PCS unification
//!
//! A Lab-PCS source result converts to XYZ via the D50-relative
//! `Lab::to_xyz` (`iccce-color`, cross-verified formulas) because the
//! destination matrix/TRC model consumes PCSXYZ. The PCS white is the
//! ICC 4-figure D50 everywhere (the mixing-precision rule).
//!
//! ## Scope
//!
//! Source side: matrix/TRC, or `lut16`/`lut8` A2B. Destination side:
//! matrix/TRC inverse, or `lut16`/`lut8` B2A evaluated forward
//! (`mAB `/`mBA ` are the remaining absentees). Intents:
//! media-relative fully; perceptual/saturation select their A2Bx per
//! the fallback and are otherwise colorimetric (for LUT profiles the
//! A2B0/A2B2 tables THEMSELVES carry the vendor's perceptual
//! rendering — corpus A27: vendor-defined by design); absolute uses
//! the D.6/D.7 scale on the unified XYZ, exactly as in `matrix_trc`.

use crate::lut_transform::{Lut16Model, PcsKind, PcsValue};
use crate::matrix_trc::{Intent, MatrixTrc, ModelError};
use iccce_color::{D50, Xyz};
use iccce_profile::Profile;
use iccce_profile::num::Signature;
use iccce_profile::tag_types::TagData;

/// A2B tag signatures, intent-indexed per 8.10.2.
mod tag {
    use iccce_profile::num::Signature;
    pub const A2B0: Signature = Signature(0x4132_4230);
    pub const A2B1: Signature = Signature(0x4132_4231);
    pub const A2B2: Signature = Signature(0x4132_4232);
    pub const B2A0: Signature = Signature(0x4232_4130);
    pub const B2A1: Signature = Signature(0x4232_4131);
    pub const B2A2: Signature = Signature(0x4232_4132);
    pub const PCS_LAB: Signature = Signature(0x4C61_6220); // 'Lab '
}

/// A source profile's device→PCS model, chosen per the fallback.
#[derive(Debug, Clone)]
pub enum SourceModel {
    MatrixTrc(Box<MatrixTrc>),
    Lut16(Box<Lut16Model>),
}

/// A destination profile's PCS→device model, chosen per the same
/// 8.10.2 fallback (B2Ax → B2A0 → TRC/matrix inverse).
#[derive(Debug, Clone)]
pub enum DestModel {
    MatrixTrc(Box<MatrixTrc>),
    /// A lut16 B2A tag: evaluated FORWARD (its stored pipeline IS the
    /// PCS→device direction — no inversion happens anywhere).
    Lut16B2a(Box<Lut16Model>),
}

/// Chain build/run errors.
#[derive(Debug, Clone, PartialEq)]
pub enum ChainError {
    Model(ModelError),
    /// No usable source path: no A2B tag decodes and the matrix/TRC
    /// fallback also failed (its error carried).
    NoSourcePath {
        matrix_trc_said: String,
    },
    /// The source model's channel count doesn't match the input.
    ChannelMismatch {
        expected: usize,
        actual: usize,
    },
    /// The tag selected by the fallback exists but is a type this
    /// stage does not evaluate (mft1/mAB — stage 3), named so the
    /// caller can distinguish "unsupported yet" from "broken".
    SourceTagUnsupported {
        sig: Signature,
        type_name: String,
    },
}

impl std::fmt::Display for ChainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Model(e) => write!(f, "{e}"),
            Self::NoSourcePath { matrix_trc_said } => write!(
                f,
                "no usable source path: no A2B tag evaluable, and matrix/TRC said: {matrix_trc_said}"
            ),
            Self::ChannelMismatch { expected, actual } => {
                write!(f, "source expects {expected} channels, got {actual}")
            }
            Self::SourceTagUnsupported { sig, type_name } => write!(
                f,
                "source tag {sig} is {type_name}, not yet evaluable (assembly stage 3)"
            ),
        }
    }
}

impl From<ModelError> for ChainError {
    fn from(e: ModelError) -> Self {
        Self::Model(e)
    }
}

/// A built source→destination conversion.
#[derive(Debug, Clone)]
pub struct Chain {
    pub source: SourceModel,
    pub dst: DestModel,
    intent: Intent,
    /// Media whites for the absolute scale, captured at build.
    src_white: Option<Xyz>,
    dst_white: Option<Xyz>,
}

impl Chain {
    /// Build for the given intent. Source side follows the 8.10.2
    /// fallback (module doc); destination must currently be
    /// matrix/TRC.
    pub fn new(src: &Profile, dst: &Profile, intent: Intent) -> Result<Chain, ChainError> {
        // Destination: B2Ax → B2A0 → matrix/TRC (the same 8.10.2
        // fallback, B-side). A lut16 B2A's input side is the PCS, so
        // in an XYZ-PCS profile the 3×3 matrix APPLIES (A21) —
        // input_is_pcs_xyz is keyed on the destination's PCS.
        let dst_intent_tag = match intent {
            Intent::Perceptual => tag::B2A0,
            Intent::MediaRelative | Intent::Absolute => tag::B2A1,
            Intent::Saturation => tag::B2A2,
        };
        let mut dst_model = None;
        for sig in [dst_intent_tag, tag::B2A0] {
            if dst_model.is_some() {
                break;
            }
            let Some(entry) = dst.tags.iter().find(|t| t.sig == sig) else {
                continue;
            };
            if let Some(Ok(decoded)) = dst.decode_tag(entry) {
                let (pcs, input_is_xyz) = if dst.header.pcs == tag::PCS_LAB {
                    (PcsKind::Lab, false)
                } else {
                    (PcsKind::Xyz, true)
                };
                let built = match decoded.data {
                    TagData::Lut16(l) => Lut16Model::from_lut16(&l, input_is_xyz, pcs).ok(),
                    // Real press profiles ship mft1 B2A tables (SWOP
                    // does); Lab-8-bit per Tables 12/13 (A10 resolved).
                    TagData::Lut8(l) => Lut16Model::from_lut8(&l, input_is_xyz, pcs).ok(),
                    _ => None,
                };
                if let Some(m) = built {
                    if m.input_channels() == 3 {
                        dst_model = Some(DestModel::Lut16B2a(Box::new(m)));
                    }
                }
            }
        }
        let dst_model = match dst_model {
            Some(m) => m,
            None => DestModel::MatrixTrc(Box::new(MatrixTrc::from_profile(dst)?)),
        };

        // 8.10.2 step 2: the intent's own A2Bx; step 3: A2B0.
        let intent_tag = match intent {
            Intent::Perceptual => tag::A2B0,
            Intent::MediaRelative | Intent::Absolute => tag::A2B1,
            Intent::Saturation => tag::A2B2,
        };
        let mut source = None;
        let mut unsupported: Option<ChainError> = None;
        for sig in [intent_tag, tag::A2B0] {
            if source.is_some() {
                break;
            }
            let Some(entry) = src.tags.iter().find(|t| t.sig == sig) else {
                continue;
            };
            if let Some(Ok(decoded)) = src.decode_tag(entry) {
                match decoded.data {
                    TagData::Lut16(l) => {
                        let pcs = if src.header.pcs == tag::PCS_LAB {
                            PcsKind::Lab
                        } else {
                            PcsKind::Xyz
                        };
                        // A2B input side is DEVICE, never PCSXYZ →
                        // matrix not applicable (A21).
                        if let Ok(m) = Lut16Model::from_lut16(&l, false, pcs) {
                            source = Some(SourceModel::Lut16(Box::new(m)));
                        }
                    }
                    TagData::Lut8(l) => {
                        let pcs = if src.header.pcs == tag::PCS_LAB {
                            PcsKind::Lab
                        } else {
                            PcsKind::Xyz
                        };
                        // A2B input is device — matrix not applicable.
                        match Lut16Model::from_lut8(&l, false, pcs) {
                            Ok(m) => source = Some(SourceModel::Lut16(Box::new(m))),
                            Err(e) => {
                                unsupported = Some(ChainError::SourceTagUnsupported {
                                    sig,
                                    type_name: format!("lut8Type ({e})"),
                                });
                            }
                        }
                    }
                    TagData::LutAToB(_) => {
                        unsupported = Some(ChainError::SourceTagUnsupported {
                            sig,
                            type_name: "lutAToBType".into(),
                        });
                    }
                    _ => {}
                }
            }
        }

        // 8.10.2 step 4: TRC + colorant matrix.
        let source = match source {
            Some(s) => s,
            None => match MatrixTrc::from_profile(src) {
                Ok(m) => SourceModel::MatrixTrc(Box::new(m)),
                Err(e) => {
                    // Prefer the more specific "tag exists but is
                    // stage-3 material" report when we saw one.
                    return Err(unsupported.unwrap_or(ChainError::NoSourcePath {
                        matrix_trc_said: e.to_string(),
                    }));
                }
            },
        };

        let src_white = read_wtpt(src);
        let dst_white = read_wtpt(dst);
        Ok(Chain {
            source,
            dst: dst_model,
            intent,
            src_white,
            dst_white,
        })
    }

    /// Destination device channel count (3 for matrix/TRC, the B2A
    /// tag's output count otherwise — e.g. 4 for CMYK).
    pub fn output_channels(&self) -> usize {
        match &self.dst {
            DestModel::MatrixTrc(_) => 3,
            DestModel::Lut16B2a(l) => l.output_channels(),
        }
    }

    /// Source device channel count.
    pub fn input_channels(&self) -> usize {
        match &self.source {
            SourceModel::MatrixTrc(_) => 3,
            SourceModel::Lut16(l) => l.input_channels(),
        }
    }

    /// Convert one set of source device values to destination device
    /// values (`output_channels()` of them — 3 for RGB, 4 for CMYK…).
    pub fn convert(&self, device: &[f64]) -> Result<Vec<f64>, ChainError> {
        let expected = self.input_channels();
        if device.len() != expected {
            return Err(ChainError::ChannelMismatch {
                expected,
                actual: device.len(),
            });
        }

        // Source → unified XYZ PCS (media-relative).
        let xyz = match &self.source {
            SourceModel::MatrixTrc(m) => m.device_to_pcs([device[0], device[1], device[2]]),
            SourceModel::Lut16(l) => match l.device_to_pcs(device) {
                Some(PcsValue::Xyz(x)) => x,
                // Lab PCS → XYZ relative to the PCS white (D50) —
                // `iccce-color`'s cross-verified formulas.
                Some(PcsValue::Lab(lab)) => lab.to_xyz(D50),
                None => {
                    return Err(ChainError::ChannelMismatch {
                        expected,
                        actual: device.len(),
                    });
                }
            },
        };

        // Absolute: the D.6/D.7 composite on unified XYZ (same rule
        // and same corrected direction as matrix_trc's path).
        let xyz = if self.intent == Intent::Absolute {
            let mw_src = self
                .src_white
                .ok_or(ChainError::Model(ModelError::AbsoluteNeedsWtpt))?;
            let mw_dst = self
                .dst_white
                .ok_or(ChainError::Model(ModelError::AbsoluteNeedsWtpt))?;
            if mw_dst.x <= 0.0 || mw_dst.y <= 0.0 || mw_dst.z <= 0.0 {
                return Err(ChainError::Model(ModelError::AbsoluteNeedsWtpt));
            }
            Xyz {
                x: xyz.x * (mw_src.x / mw_dst.x),
                y: xyz.y * (mw_src.y / mw_dst.y),
                z: xyz.z * (mw_src.z / mw_dst.z),
            }
        } else {
            xyz
        };

        // Destination: matrix/TRC inverse, or the B2A pipeline
        // evaluated forward. A Lab-PCS B2A consumes Lab: the unified
        // XYZ converts back via the D50-relative formulas — the exact
        // inverse of the source side's unification, so an XYZ↔Lab pair
        // in the middle costs only f64 noise (the round trip is an
        // arithmetic identity, tested in iccce-color).
        match &self.dst {
            DestModel::MatrixTrc(m) => Ok(m.pcs_to_device(xyz)?.to_vec()),
            DestModel::Lut16B2a(l) => {
                let pcs_value = match l.pcs_kind() {
                    PcsKind::Lab => PcsValue::Lab(iccce_color::Lab::from_xyz(xyz, D50)),
                    PcsKind::Xyz => PcsValue::Xyz(xyz),
                };
                l.pcs_to_device(pcs_value)
                    .ok_or(ChainError::ChannelMismatch {
                        expected: 3,
                        actual: 3,
                    })
            }
        }
    }
}

fn read_wtpt(profile: &Profile) -> Option<Xyz> {
    const WTPT: Signature = Signature(0x7774_7074);
    let entry = profile.tags.iter().find(|t| t.sig == WTPT)?;
    match profile.decode_tag(entry) {
        Some(Ok(d)) => match d.data {
            TagData::Xyz(v) if v.len() == 1 => Some(Xyz {
                x: v[0].x.to_f64(),
                y: v[0].y.to_f64(),
                z: v[0].z.to_f64(),
            }),
            _ => None,
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// End-to-end on REAL profiles: USWebCoatedSWOP (CMYK, v2, Lab
    /// PCS, mft2) → system sRGB. Category (c): local read, skip when
    /// absent. Sanity assertions on measured output — the full
    /// differential against lcms2 is icc-conformance's Pass 4 run.
    #[test]
    fn swop_to_srgb_end_to_end_sanity() {
        let swop = r"C:\Windows\System32\spool\drivers\color\USWebCoatedSWOP.icc";
        let srgb = r"C:\Windows\System32\spool\drivers\color\sRGB Color Space Profile.icm";
        let (Ok(s), Ok(d)) = (std::fs::read(swop), std::fs::read(srgb)) else {
            eprintln!("skipped: system profiles absent");
            return;
        };
        let src = Profile::parse(&s).unwrap();
        let dst = Profile::parse(&d).unwrap();
        let chain = Chain::new(&src, &dst, Intent::MediaRelative).unwrap();
        assert_eq!(chain.input_channels(), 4);

        // Paper (no ink): near-white RGB, all channels high.
        let paper = chain.convert(&[0.0, 0.0, 0.0, 0.0]).unwrap();
        for (i, c) in paper.iter().enumerate() {
            assert!(*c > 0.85, "paper channel {i} = {c}");
        }
        // 100% K: dark, all channels low.
        let black = chain.convert(&[0.0, 0.0, 0.0, 1.0]).unwrap();
        for (i, c) in black.iter().enumerate() {
            assert!(*c < 0.35, "black channel {i} = {c}");
        }
        // 100% C: blue-cyan — red channel clearly below green/blue.
        let cyan = chain.convert(&[1.0, 0.0, 0.0, 0.0]).unwrap();
        assert!(cyan[0] < cyan[1] && cyan[0] < cyan[2], "cyan {cyan:?}");
        // M+Y = red-ish: red channel dominant.
        let red = chain.convert(&[0.0, 1.0, 1.0, 0.0]).unwrap();
        assert!(red[0] > red[1] && red[0] > red[2], "red {red:?}");
    }

    /// The perceptual chain on this pair selects A2B0 and the
    /// saturation chain A2B2 — which in SWOP share one offset (the
    /// Pass 0 finding), so their outputs must be IDENTICAL. A real-
    /// file regression of both the fallback selection and the shared-
    /// tag-data handling.
    #[test]
    fn swop_perceptual_equals_saturation_shared_tag() {
        let swop = r"C:\Windows\System32\spool\drivers\color\USWebCoatedSWOP.icc";
        let srgb = r"C:\Windows\System32\spool\drivers\color\sRGB Color Space Profile.icm";
        let (Ok(s), Ok(d)) = (std::fs::read(swop), std::fs::read(srgb)) else {
            eprintln!("skipped: system profiles absent");
            return;
        };
        let src = Profile::parse(&s).unwrap();
        let dst = Profile::parse(&d).unwrap();
        let per = Chain::new(&src, &dst, Intent::Perceptual).unwrap();
        let sat = Chain::new(&src, &dst, Intent::Saturation).unwrap();
        let probe = [0.3, 0.5, 0.2, 0.1];
        assert_eq!(per.convert(&probe).unwrap(), sat.convert(&probe).unwrap());
    }
}
