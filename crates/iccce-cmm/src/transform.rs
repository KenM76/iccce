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
//! Source side: matrix/TRC, grayTRC (F.2), or `lut16`/`lut8`/`mAB `
//! A2B. Destination side: matrix/TRC inverse, grayTRC inverse, or
//! `lut16`/`lut8`/`mBA ` B2A evaluated forward. Intents:
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
    /// A v4 `mAB ` A2B pipeline.
    LutAb(Box<crate::lut_ab::LutAbModel>),
    /// A monochrome profile's F.2 grayTRC model (1 channel).
    Gray(Box<crate::gray_trc::GrayTrc>),
}

/// A destination profile's PCS→device model, chosen per the same
/// 8.10.2 fallback (B2Ax → B2A0 → TRC/matrix inverse).
#[derive(Debug, Clone)]
pub enum DestModel {
    MatrixTrc(Box<MatrixTrc>),
    /// A lut16 B2A tag: evaluated FORWARD (its stored pipeline IS the
    /// PCS→device direction — no inversion happens anywhere).
    Lut16B2a(Box<Lut16Model>),
    /// A v4 `mBA ` pipeline (counts per 10.13.2/4/6 — GP-001).
    LutAb(Box<crate::lut_ab::LutAbModel>),
    /// A monochrome destination (F.2 inverse; chromatic content of
    /// the PCS is discarded by the achromatic-channel rule — stated
    /// in `gray_trc`, not hidden).
    Gray(Box<crate::gray_trc::GrayTrc>),
}

/// ★★ What the caller is handing over as the destination — and, when it
/// is handing over nothing, **why** it has nothing.
///
/// # Why this is not an `Option<&Profile>`
///
/// This is the single most safety-relevant type in the crate, and the
/// reason is worth the words. `docs/DEFAULT_DESTINATION.md` §2 states
/// the contract the operator's decision carries:
///
/// > **"Doesn't exist" must mean *absent*, never *unresolved*.**
///
/// A colour-managed document very often **declares** its destination. A
/// PDF/X file's `/OutputIntents` names a print condition and embeds the
/// profile. If a caller hands iccce a document whose declared
/// destination *failed to parse*, or *was not looked for*, or *was found
/// and then dropped*, and iccce quietly substitutes sRGB, the result is:
///
/// - a plausible-looking image,
/// - rendered to the wrong destination,
/// - with the document's own declared print condition silently
///   discarded,
/// - **and no error anywhere.**
///
/// That is this project's founding hazard — *a wrong colour looks
/// exactly like a right one* — and it would be **caused** by the
/// fallback rather than caught by it.
///
/// **An `Option` being `None` cannot distinguish "there was none" from
/// "I failed to get one."** Only the caller knows which, because only
/// the caller knows whether it looked. So the API makes the caller
/// *say*. That is the whole purpose of this enum, and it is why
/// [`Self::None`] is spelled out rather than inferred.
///
/// A destination that was declared and could not be parsed remains a
/// **named refusal** — the caller propagates its parse error and does
/// not reach this type at all.
#[derive(Debug, Clone, Copy)]
pub enum Destination<'a> {
    /// The caller has a destination profile. **It is used. Always.**
    /// No exception, no override, no "improvement".
    Profile(&'a Profile),
    /// The caller looked, and there is genuinely no destination to use.
    /// iccce constructs sRGB from published constants
    /// ([`crate::builtin::srgb`]) and the chain records that it did, so
    /// the substitution is disclosed rather than silent.
    ///
    /// ★ **Do not reach for this as recovery from a failure to obtain a
    /// destination.** Doing so converts a loud, correct refusal into a
    /// quiet, wrong picture. If the document declared a destination and
    /// you could not use it, that is a refusal to report, not a reason
    /// to substitute a different destination.
    None,
}

/// Where a built [`Chain`]'s destination came from.
///
/// # Why this exists: rule 6 has a CMM analogue
///
/// *The parser reports; it does not repair.* Silently substituting a
/// destination is the transform-layer version of a silently corrected
/// tag — it hides a material fact from the only layer that could
/// disclose it.
///
/// The fallback is **not an error**; the operator decided it should
/// happen. But it is a *fact about the conversion*, and a consumer must
/// be able to log it, surface it in a preflight report, or gate on it. A
/// renderer that cannot tell whether a page went to a document-declared
/// destination or to iccce's default cannot honestly report what it did.
///
/// **This is the difference between a default and a cover-up, and it
/// costs one accessor to stay on the right side of it.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DestinationProvenance {
    /// A caller-supplied profile was used.
    CallerSupplied,
    /// No destination was supplied and iccce constructed sRGB.
    BuiltInSrgb,
}

impl DestinationProvenance {
    /// A short human-readable disclosure, suitable for a log line or a
    /// preflight report. `None` when nothing needs disclosing.
    #[must_use]
    pub fn note(self) -> Option<&'static str> {
        match self {
            Self::CallerSupplied => None,
            Self::BuiltInSrgb => Some(
                "no destination profile was supplied: converted to iccce's built-in sRGB, \
                 constructed from ITU-R BT.709-6 primaries and W3C-published transfer-function \
                 constants and Bradford-adapted to the D50 PCS (ICC.1:2022 Annex E.3). This is \
                 NOT the document's declared output intent — if one was declared, it was not \
                 passed to iccce",
            ),
        }
    }
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
    /// BPC requested at the absolute intent — excluded because BPC
    /// presupposes both whites already at D50 (sourced, Maria 2013).
    BpcNotApplicable,
    /// BPC requested but a side's black point is outside the named
    /// estimation subset (A42) — refused, not guessed.
    BpcEstimationUnsupported,
    /// A stage that should have evaluated returned nothing — an
    /// internal shape inconsistency, not a caller error.
    ///
    /// Added by the pre-publication audit (2026-08-12), which found
    /// `ChannelMismatch { expected: 3, actual: 3 }` being returned as
    /// a stand-in for unrelated failures: a public error saying "3
    /// channels expected, 3 given" is **misinformation on the API
    /// surface**, and this project does not get to report an
    /// implausible-but-tidy answer anywhere.
    EvaluationFailed {
        stage: &'static str,
    },
    /// A compiled grid's node count overflows this machine's address
    /// space. Refused rather than wrapped into a small allocation
    /// that would silently produce a wrong transform.
    GridTooLarge {
        grid_points: usize,
        dimensions: usize,
    },
    /// ★★ A compiled grid whose node count is a perfectly ordinary
    /// `usize` and whose ALLOCATION is not survivable.
    ///
    /// Distinct from [`Self::GridTooLarge`], which is the arithmetic
    /// overflow case. This one is the case that actually happens: on a
    /// 64-bit machine `33^7` nodes is a fine number and 0.93 TiB is not a
    /// fine allocation.
    ///
    /// **Why it is an error at all rather than "let the allocator
    /// decide":** the allocator's decision is `abort()`. A `Vec`
    /// allocation failure kills the process — not a catchable panic, a
    /// hard exit (`0xC0000409` on Windows). For a library that is
    /// unacceptable: it takes the consumer's process down and the
    /// consumer had no way to see it coming. Turning it into a named
    /// refusal is rule 6 applied at the allocation layer.
    ///
    /// Found by Pass H on ICC's own published seven-channel
    /// `APTEC_CMYKOGV_Coated_LinearCTV_2025.icc`, 2026-08-17.
    GridExceedsBudget {
        nodes: usize,
        bytes: usize,
        budget_bytes: usize,
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
            Self::BpcNotApplicable => write!(
                f,
                "BPC is excluded at the absolute intent (both whites are already D50-anchored)"
            ),
            Self::BpcEstimationUnsupported => write!(
                f,
                "black point not estimable within iccce's named subset (A42); refused, not guessed"
            ),
            Self::EvaluationFailed { stage } => {
                write!(f, "internal: the {stage} stage produced no value")
            }
            Self::GridTooLarge {
                grid_points,
                dimensions,
            } => write!(
                f,
                "compiled grid {grid_points}^{dimensions} exceeds addressable memory; refused"
            ),
            Self::GridExceedsBudget {
                nodes,
                bytes,
                budget_bytes,
            } => write!(
                f,
                "compiled grid would need {nodes} nodes = {bytes} bytes, over iccce's \
                 {budget_bytes}-byte budget; refused rather than allowed to abort the process. \
                 Pass a smaller --grid, or use the reference path (`transform`), which has no \
                 grid at all"
            ),
        }
    }
}

impl From<ModelError> for ChainError {
    fn from(e: ModelError) -> Self {
        Self::Model(e)
    }
}

/// ## Why this impl exists, and why its absence was a real defect
///
/// `iccce_profile::ParseError` has implemented [`std::error::Error`]
/// since the parser was written; `ChainError` did not. The asymmetry was
/// **noticed but unscoped** for several sessions and looked cosmetic.
///
/// It is not cosmetic. Without this impl, a consumer cannot write
///
/// ```ignore
/// fn convert(..) -> Result<(), Box<dyn std::error::Error>> {
///     let chain = Chain::with_destination(src, dst, intent)?;   // E0277
/// }
/// ```
///
/// which is the single most common error-handling shape in Rust, and the
/// one a library consumer reaches for first. The gap therefore forced
/// every caller to hand-wrap, or — far worse — to `.ok()` the result and
/// lose the named refusal entirely. **A refusal that is inconvenient to
/// propagate is a refusal that gets discarded**, and every variant of
/// this enum exists precisely so that a failure is reported rather than
/// guessed at.
///
/// ★ It was caught by a **doc example** written for
/// [`Chain::with_destination`]: the example did the ordinary thing, and
/// the doctest refused to compile. That is the value of a doc example
/// that is compiled — it exercises the API as a stranger would, which no
/// amount of internal review does.
///
/// `source` is implemented rather than defaulted so a caller
/// walking the chain reaches the underlying [`ModelError`] and, through
/// it, any [`CurveError`](crate::curve::CurveError) beneath — the
/// specific clause-level refusal is usually the part worth logging, and
/// a flat error would strand it behind a `Display` string.
impl std::error::Error for ChainError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Model(e) => Some(e),
            _ => None,
        }
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
    /// Black point compensation, when the caller opted in via
    /// [`Chain::with_bpc`]. NEVER forced: lcms2 forces BPC for v4
    /// perceptual/saturation on the authority of an unpublished
    /// reading (M2/DL-013, and its "always" has no published
    /// corroboration — corpus `icc__ref__bpc.md`); iccce makes it an
    /// explicit caller act, which is itself a recorded policy
    /// difference from the oracle.
    bpc: Option<crate::bpc::BpcScale>,
    /// Major versions captured at build — the fixed-perceptual-black
    /// estimation rule keys on them.
    src_major: u8,
    dst_major: u8,
    /// The DESTINATION profile's device→PCS direction, captured at
    /// build solely so BPC's ISO/CD 18619 4.2.5.2.3 round trip has
    /// its outward leg. Unused by any conversion.
    dst_a2b: Option<SourceModel>,
    /// Whether the destination was caller-supplied or constructed.
    /// Read via [`Chain::destination_provenance`]; see
    /// [`DestinationProvenance`] for why disclosing this is mandatory
    /// rather than a nicety.
    dst_provenance: DestinationProvenance,
}

/// Device channel count of a captured A2B model.
fn a2b_channels(m: &SourceModel) -> usize {
    match m {
        SourceModel::MatrixTrc(_) => 3,
        SourceModel::Lut16(l) => l.input_channels(),
        SourceModel::LutAb(l) => l.device_channels(),
        SourceModel::Gray(_) => 1,
    }
}

/// Evaluate a captured A2B model to Lab (the BT round trip's return
/// leg, and the darkest-vertex search's probe).
fn a2b_to_lab(m: &SourceModel, device: &[f64]) -> Option<iccce_color::Lab> {
    let xyz = match m {
        SourceModel::MatrixTrc(mt) => {
            if device.len() < 3 {
                return None;
            }
            mt.device_to_pcs([device[0], device[1], device[2]])
        }
        SourceModel::Lut16(l) => match l.device_to_pcs(device)? {
            PcsValue::Xyz(x) => x,
            PcsValue::Lab(lab) => return Some(lab),
        },
        SourceModel::LutAb(l) => match l.device_to_pcs(device)? {
            PcsValue::Xyz(x) => x,
            PcsValue::Lab(lab) => return Some(lab),
        },
        SourceModel::Gray(g) => {
            if device.is_empty() {
                return None;
            }
            g.device_to_pcs(device[0])
        }
    };
    Some(iccce_color::Lab::from_xyz(xyz, D50))
}

impl Chain {
    /// Build for the given intent. Source side follows the 8.10.2
    /// fallback (module doc); destination must currently be
    /// matrix/TRC.
    pub fn new(src: &Profile, dst: &Profile, intent: Intent) -> Result<Chain, ChainError> {
        Chain::new_inner(src, dst, intent, true)
    }

    /// Build with an explicit statement of what the destination is —
    /// including the case where the caller has none.
    ///
    /// This is [`Chain::new`]'s superset and the entry point a consumer
    /// that may lack a destination should use. See [`Destination`] for
    /// why the "no destination" case is a named variant rather than an
    /// `Option`, and [`DestinationProvenance`] for why the result
    /// discloses which case occurred.
    ///
    /// ```no_run
    /// # use iccce_cmm::transform::{Chain, Destination};
    /// # use iccce_cmm::matrix_trc::Intent;
    /// # fn f(src: &iccce_profile::Profile) -> Result<(), Box<dyn std::error::Error>> {
    /// let chain = Chain::with_destination(src, Destination::None, Intent::MediaRelative)?;
    /// if let Some(note) = chain.destination_provenance().note() {
    ///     eprintln!("iccce: {note}");   // disclosed, not silent
    /// }
    /// # Ok(()) }
    /// ```
    pub fn with_destination(
        src: &Profile,
        dst: Destination<'_>,
        intent: Intent,
    ) -> Result<Chain, ChainError> {
        match dst {
            Destination::Profile(p) => Chain::new_inner(src, p, intent, true),
            Destination::None => Chain::new_builtin_srgb_dest(src, intent),
        }
    }
    /// Derive the SOURCE half's device→PCS model, per the 8.10.2
    /// fallback: the intent's own `A2Bx`, then `A2B0`, then the
    /// matrix/TRC model (F.3), then `grayTRC` (F.2).
    ///
    /// ## Why this is a separate function
    ///
    /// ★ **Extracted 2026-08-17 to fix a real defect, and the defect is
    /// worth recording because the symptom was a plausible lie.**
    ///
    /// `Chain::with_destination(src, Destination::None, ..)` originally
    /// obtained the source model by building a scaffold chain
    /// `src → src` and discarding its destination half. That works
    /// whenever the source can also serve as a destination — which every
    /// profile tested at the time could.
    ///
    /// It fails for a profile carrying an `A2B` tag but **no `B2A`, no
    /// colorant matrix and no `grayTRC`** — a perfectly conformant shape
    /// (clause 8 requires a `B2A` only for classes that need the reverse
    /// direction). Four such profiles are in ICC's own published set: the
    /// colour-vision-deficiency simulation profiles, which are `scnr`
    /// class, Lab PCS, and one-directional by design.
    ///
    /// **The symptom was the dangerous part.** The scaffold's destination
    /// derivation failed and surfaced its own error:
    ///
    /// ```text
    /// matrix/TRC model requires PCSXYZ (Annex F.3, normative); profile PCS is 'Lab '
    /// ```
    ///
    /// — a correct, clause-cited message **about a model iccce was about
    /// to throw away.** A caller reads that as *"my source profile is
    /// unusable"*, which is false: the source was fine, and the
    /// destination it was complaining about was never going to be used.
    /// A refusal that names the wrong clause is worse than a vague one,
    /// because the citation makes it persuasive.
    ///
    /// Deriving the source half directly removes the phantom
    /// destination, so the built-in path now succeeds on exactly the
    /// profiles it should. Sharing this function with `new_inner` is the
    /// point: the 8.10.2 dispatch is the most intricate logic in the
    /// crate and a second copy would be free to drift.
    fn derive_source_model(src: &Profile, intent: Intent) -> Result<SourceModel, ChainError> {
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
                    TagData::LutAToB(l) => {
                        let pcs = if src.header.pcs == tag::PCS_LAB {
                            PcsKind::Lab
                        } else {
                            PcsKind::Xyz
                        };
                        match crate::lut_ab::LutAbModel::from_lut_ab(&l, pcs) {
                            Ok(m) => source = Some(SourceModel::LutAb(Box::new(m))),
                            Err(e) => {
                                unsupported = Some(ChainError::SourceTagUnsupported {
                                    sig,
                                    type_name: format!("lutAToBType ({e})"),
                                });
                            }
                        }
                    }
                    _ => {}
                }
            }
        }

        // 8.10.2 step 4: TRC + colorant matrix (F.3), or grayTRC (F.2)
        // for monochrome profiles.
        let source = match source {
            Some(s) => s,
            None => match MatrixTrc::from_profile(src) {
                Ok(m) => SourceModel::MatrixTrc(Box::new(m)),
                Err(matrix_err) => match crate::gray_trc::GrayTrc::from_profile(src) {
                    Ok(g) => SourceModel::Gray(Box::new(g)),
                    Err(_) => {
                        // Prefer the more specific "tag exists but is
                        // unsupported" report when we saw one.
                        return Err(unsupported.unwrap_or(ChainError::NoSourcePath {
                            matrix_trc_said: matrix_err.to_string(),
                        }));
                    }
                },
            },
        };
        Ok(source)
    }

    /// Build against the constructed sRGB destination.
    ///
    /// ## Why this does not go through `new_inner`
    ///
    /// `new_inner` derives the destination model by walking a parsed
    /// profile's `B2Ax` tags through the 8.10.2 fallback. There is no
    /// profile here and no tags to walk — the model is *computed*. So
    /// the destination half is supplied directly and the source half
    /// comes from [`Chain::derive_source_model`], which `new_inner` also
    /// uses.
    ///
    /// ★ Sharing that function rather than duplicating it is deliberate:
    /// the 8.10.2 dispatch is the most intricate logic in the crate and a
    /// second copy would be free to drift. **An earlier version obtained
    /// the source model by building a scaffold `src → src` chain and
    /// discarding its destination — see `derive_source_model`'s doc for
    /// why that was wrong and what it reported instead.**
    fn new_builtin_srgb_dest(src: &Profile, intent: Intent) -> Result<Chain, ChainError> {
        // The source half only. No phantom destination is derived, so a
        // profile that cannot serve as a destination — an A2B-only
        // profile with no B2A, no colorant matrix and no grayTRC — is
        // still a perfectly good SOURCE here.
        let source = Chain::derive_source_model(src, intent)?;

        let srgb = crate::builtin::srgb();
        // The destination's own device→PCS direction, needed only by
        // BPC's round trip. For a matrix/TRC destination this IS the
        // forward model, so it is exact rather than approximated.
        let dst_a2b = Some(SourceModel::MatrixTrc(Box::new(srgb.clone())));

        Ok(Chain {
            source,
            dst: DestModel::MatrixTrc(Box::new(srgb)),
            intent,
            src_white: read_wtpt(src),
            // The constructed sRGB is D50-adapted and self-consistent:
            // its media white IS the PCS white. Unlike the shipped HP
            // profile, this needs no caveat.
            dst_white: Some(D50),
            bpc: None,
            src_major: src.header.version.major(),
            // The constructed model is v4-shaped. ★ This field is
            // consulted ONLY by the perceptual black-point estimation
            // rule, and only for LUT-backed destinations; a matrix/TRC
            // destination takes its black from `device_to_pcs([0,0,0])`
            // directly, so this value is not reachable on this path. It
            // is stated rather than left to a default so that a future
            // change to the estimation rule cannot silently pick up an
            // unconsidered 0.
            dst_major: 4,
            dst_a2b,
            dst_provenance: DestinationProvenance::BuiltInSrgb,
        })
    }

    /// Where this chain's destination came from — **caller-supplied or
    /// constructed by iccce**.
    ///
    /// Always check this when the chain may have been built with
    /// [`Destination::None`]. `DestinationProvenance::note()` gives
    /// ready-to-log wording.
    #[must_use]
    pub fn destination_provenance(&self) -> DestinationProvenance {
        self.dst_provenance
    }

    /// `capture_dst_a2b` is false only for the internal build of the
    /// destination's own A2B side, which must not recurse.
    fn new_inner(
        src: &Profile,
        dst: &Profile,
        intent: Intent,
        capture_dst_a2b: bool,
    ) -> Result<Chain, ChainError> {
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
                match decoded.data {
                    TagData::Lut16(l) => {
                        if let Ok(m) = Lut16Model::from_lut16(&l, input_is_xyz, pcs) {
                            if m.input_channels() == 3 {
                                dst_model = Some(DestModel::Lut16B2a(Box::new(m)));
                            }
                        }
                    }
                    // Real press profiles ship mft1 B2A tables (SWOP
                    // does); Lab-8-bit per Tables 12/13 (A10 resolved).
                    TagData::Lut8(l) => {
                        if let Ok(m) = Lut16Model::from_lut8(&l, input_is_xyz, pcs) {
                            if m.input_channels() == 3 {
                                dst_model = Some(DestModel::Lut16B2a(Box::new(m)));
                            }
                        }
                    }
                    TagData::LutBToA(l) => {
                        if let Ok(m) = crate::lut_ab::LutAbModel::from_mba(&l, pcs) {
                            dst_model = Some(DestModel::LutAb(Box::new(m)));
                        }
                    }
                    _ => {}
                }
            }
        }
        let dst_model = match dst_model {
            Some(m) => m,
            // 8.10.2 step 4 has two shapes: three-component
            // matrix/TRC (F.3) or grayTRC (F.2) — clause 8's
            // per-class requirements decide which tags exist.
            None => match MatrixTrc::from_profile(dst) {
                Ok(m) => DestModel::MatrixTrc(Box::new(m)),
                Err(matrix_err) => match crate::gray_trc::GrayTrc::from_profile(dst) {
                    Ok(g) => DestModel::Gray(Box::new(g)),
                    Err(_) => return Err(ChainError::Model(matrix_err)),
                },
            },
        };

        let source = Self::derive_source_model(src, intent)?;

        let src_white = read_wtpt(src);
        let dst_white = read_wtpt(dst);
        // The destination's OWN A2B side, for BPC's round trip only
        // (ISO/CD 18619 4.2.5.2.3). Built by the same fallback used
        // for a real source, so it cannot diverge from how the
        // destination would behave as a source. `None` is fine — it
        // only narrows which black points are estimable.
        //
        // NOTE the recursion guard: this must NOT call `Chain::new`,
        // which would build its own `dst_a2b` and never terminate.
        // (Written that way first; caught at compile-and-think time,
        // before it could run.)
        let dst_a2b = if capture_dst_a2b {
            Chain::new_inner(dst, dst, Intent::MediaRelative, false)
                .ok()
                .map(|c| c.source)
        } else {
            None
        };
        Ok(Chain {
            source,
            dst: dst_model,
            intent,
            src_white,
            dst_white,
            bpc: None,
            src_major: src.header.version.major(),
            dst_major: dst.header.version.major(),
            dst_a2b,
            dst_provenance: DestinationProvenance::CallerSupplied,
        })
    }

    /// Opt in to black point compensation (Pass 5). Errors are
    /// refusals with names, never guesses:
    ///
    /// - **Absolute intent excluded** — BPC presupposes both whites
    ///   already at D50 (Maria 2013's sourced exclusion, corpus
    ///   `icc__ref__bpc.md`).
    /// - **Estimation subset** (a labelled re-implementation subset of
    ///   lcms2's, A42): a v4 side at perceptual uses the fixed
    ///   perceptual black (A41 triple); a matrix/TRC or gray side uses
    ///   its media-relative device-black; anything else —
    ///   notably v2 LUT sources, where lcms2 runs an unattributed Lab
    ///   ridge search — is [`ChainError::BpcEstimationUnsupported`].
    pub fn with_bpc(mut self) -> Result<Chain, ChainError> {
        if self.intent == Intent::Absolute {
            return Err(ChainError::BpcNotApplicable);
        }
        let src_black = self.estimate_src_black()?;
        let dst_black = self.estimate_dst_black()?;
        self.bpc = Some(
            crate::bpc::BpcScale::new(src_black, dst_black)
                .ok_or(ChainError::BpcEstimationUnsupported)?,
        );
        Ok(self)
    }

    fn estimate_src_black(&self) -> Result<Xyz, ChainError> {
        match &self.source {
            SourceModel::MatrixTrc(m) => Ok(m.device_to_pcs([0.0, 0.0, 0.0])),
            SourceModel::Gray(g) => Ok(g.device_to_pcs(0.0)),
            SourceModel::Lut16(_) | SourceModel::LutAb(_) => {
                if self.src_major >= 4 && self.intent == Intent::Perceptual {
                    Ok(crate::bpc::PERCEPTUAL_BLACK)
                } else {
                    Err(ChainError::BpcEstimationUnsupported)
                }
            }
        }
    }

    fn estimate_dst_black(&self) -> Result<Xyz, ChainError> {
        match &self.dst {
            DestModel::MatrixTrc(m) => Ok(m.device_to_pcs([0.0, 0.0, 0.0])),
            DestModel::Gray(g) => Ok(g.device_to_pcs(0.0)),
            DestModel::Lut16B2a(_) | DestModel::LutAb(_) => {
                // v4 perceptual: the fixed black, which both
                // implementations return without searching (A41).
                if self.dst_major >= 4 && self.intent == Intent::Perceptual {
                    return Ok(crate::bpc::PERCEPTUAL_BLACK);
                }
                // ★ Otherwise: the ISO/CD 18619 4.2.5 procedure —
                // WIRED 2026-08-12. It was implemented in `bpc` and
                // reachable from nothing; icc-conformance's Pass 5b
                // sweep found it had no caller, so the shipped
                // binary still refused exactly the case ISO 4.2.5
                // exists for (a v2 CMYK LUT destination at media
                // relative). "An unused capability is not a feature"
                // — the lesson this project carried in from its
                // sibling, demonstrated on its own code.
                //
                // BT (4.2.5.2.3) needs BOTH directions of the
                // destination: Lab→device with the user's intent,
                // then device→Lab with RELATIVE always. The A2B side
                // is captured at build for exactly this.
                let a2b = self
                    .dst_a2b
                    .as_ref()
                    .ok_or(ChainError::BpcEstimationUnsupported)?;
                let intent = match self.intent {
                    Intent::MediaRelative | Intent::Absolute => {
                        crate::bpc::EstimationIntent::RelativeColorimetric
                    }
                    Intent::Perceptual | Intent::Saturation => {
                        crate::bpc::EstimationIntent::PerceptualOrSaturation
                    }
                };
                // 4.2.5.2.1 InitialLab: perceptual/saturation start
                // at (0,0,0); relative starts from the destination's
                // own darkest colour, neutralised and clipped (4.2.3).
                let initial = if intent == crate::bpc::EstimationIntent::PerceptualOrSaturation {
                    iccce_color::Lab {
                        l: 0.0,
                        a: 0.0,
                        b: 0.0,
                    }
                } else {
                    let channels = a2b_channels(a2b);
                    let darkest = crate::bpc::darkest_vertex(channels, |v| {
                        a2b_to_lab(a2b, v).unwrap_or(iccce_color::Lab {
                            l: 100.0,
                            a: 0.0,
                            b: 0.0,
                        })
                    });
                    let lab =
                        a2b_to_lab(a2b, &darkest).ok_or(ChainError::BpcEstimationUnsupported)?;
                    crate::bpc::neutralise_and_clip(lab.l)
                };

                let black = crate::bpc::estimate_lut_destination_black(initial, intent, |lab| {
                    // BT: out with the user's intent, back with
                    // relative — the one place a "relative" appears
                    // that is not the caller's choice.
                    self.pcs_to_destination(lab.to_xyz(D50))
                        .ok()
                        .and_then(|device| a2b_to_lab(a2b, &device))
                        .unwrap_or(iccce_color::Lab {
                            l: 0.0,
                            a: 0.0,
                            b: 0.0,
                        })
                });
                // The map consumes XYZ. NOTE the black may legitimately
                // be CHROMATIC: 4.2.5.4's short-circuit returns
                // InitialLab unchanged, and 4.2.5.2.1 zeroes chroma
                // only for CMYK — so a Gray/RGB LUT destination yields
                // a chromatic DestinationBlackPoint by ISO's own
                // design. Not neutralised here; 4.2.6 ignores a/b
                // downstream anyway, so passing it through costs
                // nothing and asserting neutrality would be a second,
                // quieter departure from the clause.
                Ok(black.to_xyz(D50))
            }
        }
    }

    /// Convert a PCS value straight into a destination profile's
    /// device space — the entry point a named colour needs (Pass 7),
    /// where there is no source profile at all because the spot's
    /// colorimetry IS the source.
    ///
    /// Builds the destination side through the ordinary [`Chain`]
    /// machinery (same 8.10.2 fallback, same B2A/matrix/gray
    /// selection) so a spot colour cannot silently take a different
    /// path from every other conversion. Media-relative by
    /// construction: Table 66 requires `ncl2` PCS values to be
    /// relative colorimetric, so no intent choice arises here.
    pub fn convert_pcs_to_device(dst: &Profile, pcs: PcsValue) -> Result<Vec<f64>, ChainError> {
        // The destination-only chain: source is irrelevant, so build
        // against `dst` on both sides and use the destination half.
        let chain = Chain::new(dst, dst, Intent::MediaRelative)?;
        let xyz = match pcs {
            PcsValue::Xyz(x) => x,
            PcsValue::Lab(lab) => lab.to_xyz(D50),
        };
        chain.pcs_to_destination(xyz)
    }

    /// The destination half of `convert`, exposed so a PCS-side
    /// caller (named colour) reaches exactly the same code as a
    /// device-side one.
    pub fn pcs_to_destination(&self, xyz: Xyz) -> Result<Vec<f64>, ChainError> {
        match &self.dst {
            DestModel::MatrixTrc(m) => Ok(m.pcs_to_device(xyz)?.to_vec()),
            DestModel::Lut16B2a(l) => {
                let pcs_value = match l.pcs_kind() {
                    PcsKind::Lab => PcsValue::Lab(iccce_color::Lab::from_xyz(xyz, D50)),
                    PcsKind::Xyz => PcsValue::Xyz(xyz),
                };
                l.pcs_to_device(pcs_value)
                    .ok_or(ChainError::EvaluationFailed {
                        stage: "destination B2A",
                    })
            }
            DestModel::LutAb(l) => {
                let pcs_value = match l.pcs_kind() {
                    PcsKind::Lab => PcsValue::Lab(iccce_color::Lab::from_xyz(xyz, D50)),
                    PcsKind::Xyz => PcsValue::Xyz(xyz),
                };
                l.pcs_to_device(pcs_value)
                    .ok_or(ChainError::EvaluationFailed {
                        stage: "destination B2A",
                    })
            }
            DestModel::Gray(g) => Ok(vec![g.pcs_to_device(xyz).map_err(|e| {
                ChainError::NoSourcePath {
                    matrix_trc_said: e.to_string(),
                }
            })?]),
        }
    }

    /// Destination device channel count (3 for matrix/TRC, the B2A
    /// tag's output count otherwise — e.g. 4 for CMYK).
    pub fn output_channels(&self) -> usize {
        match &self.dst {
            DestModel::MatrixTrc(_) => 3,
            DestModel::Lut16B2a(l) => l.output_channels(),
            DestModel::LutAb(l) => l.device_channels(),
            DestModel::Gray(_) => 1,
        }
    }

    /// Source device channel count.
    pub fn input_channels(&self) -> usize {
        match &self.source {
            SourceModel::MatrixTrc(_) => 3,
            SourceModel::Lut16(l) => l.input_channels(),
            SourceModel::LutAb(l) => l.device_channels(),
            SourceModel::Gray(_) => 1,
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
            SourceModel::LutAb(l) => match l.device_to_pcs(device) {
                Some(PcsValue::Xyz(x)) => x,
                Some(PcsValue::Lab(lab)) => lab.to_xyz(D50),
                None => {
                    return Err(ChainError::ChannelMismatch {
                        expected,
                        actual: device.len(),
                    });
                }
            },
            SourceModel::Gray(g) => g.device_to_pcs(device[0]),
        };

        // BPC, when opted in: applied to the unified media-relative
        // XYZ, before the destination (and never combined with
        // absolute — with_bpc refuses that at build).
        let xyz = match &self.bpc {
            Some(scale) => scale.apply(xyz),
            None => xyz,
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
        //
        // ONE destination implementation, shared with the PCS-side
        // entry point a named colour uses: a spot colour that took a
        // different path from every other conversion would be exactly
        // the kind of quiet divergence this project exists to avoid.
        self.pcs_to_destination(xyz)
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

    /// BPC on/off differ in the DOCUMENTED DIRECTION (Pass 5's
    /// done-when clause 1, at the chain level). The FIRST version of
    /// this test used sRGB→AdobeRGB and asserted a nonzero shift —
    /// wrong premise: matrix/TRC device blacks are exactly zero on
    /// both sides, so BPC between them is the IDENTITY (a documented
    /// consequence of the two-constraint map, asserted below as its
    /// own fact). The direction test needs DISTINCT blacks: the
    /// committed v4 fixture at perceptual (fixed A41 black) into
    /// sRGB (zero black) — the −3.148 L* anchor direction.
    #[test]
    fn bpc_on_off_differ_in_documented_direction() {
        let fixture = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/synthetic/v4-cmyk-mab-lab.icc"
        );
        let srgb = r"C:\Windows\System32\spool\drivers\color\sRGB Color Space Profile.icm";
        let (Ok(s), Ok(d)) = (std::fs::read(fixture), std::fs::read(srgb)) else {
            eprintln!("skipped: profile absent");
            return;
        };
        let src = Profile::parse(&s).unwrap();
        let dst = Profile::parse(&d).unwrap();

        // Equal-blacks identity: matrix/TRC → matrix/TRC-shaped BPC
        // between exactly-zero blacks changes nothing. (Both source
        // and destination here would need matrix/TRC; use sRGB→sRGB.)
        let plain = Chain::new(&dst, &dst, Intent::MediaRelative).unwrap();
        let bpc_id = Chain::new(&dst, &dst, Intent::MediaRelative)
            .unwrap()
            .with_bpc()
            .unwrap();
        let probe = [0.05, 0.05, 0.05];
        assert_eq!(
            plain.convert(&probe).unwrap(),
            bpc_id.convert(&probe).unwrap(),
            "equal blacks: BPC is the identity"
        );

        // Distinct blacks: v4 LUT source at perceptual (fixed A41
        // black) → sRGB (zero black). Dark values must move, and
        // move MORE than light values (the map converges to identity
        // at white).
        let plain = Chain::new(&src, &dst, Intent::Perceptual).unwrap();
        let bpc = Chain::new(&src, &dst, Intent::Perceptual)
            .unwrap()
            .with_bpc()
            .unwrap();
        let delta = |a: &[f64], b: &[f64]| {
            a.iter()
                .zip(b)
                .map(|(x, y)| (x - y).abs())
                .fold(0.0f64, f64::max)
        };
        let dark_in = [0.1, 0.1, 0.1, 0.9]; // heavy K: dark
        let light_in = [0.05, 0.05, 0.05, 0.0]; // near-paper: light
        let dark_shift = delta(
            &plain.convert(&dark_in).unwrap(),
            &bpc.convert(&dark_in).unwrap(),
        );
        let light_shift = delta(
            &plain.convert(&light_in).unwrap(),
            &bpc.convert(&light_in).unwrap(),
        );
        assert!(dark_shift > 0.0, "BPC must change dark values");
        assert!(
            dark_shift > light_shift,
            "dark {dark_shift} vs light {light_shift}"
        );

        // Absolute + BPC: refused by name.
        assert_eq!(
            Chain::new(&src, &dst, Intent::Absolute)
                .unwrap()
                .with_bpc()
                .err(),
            Some(ChainError::BpcNotApplicable)
        );
    }

    /// ★ THE ISO ESTIMATOR IS REACHED (regression for the defect
    /// icc-conformance's Pass 5b sweep found): a v2 CMYK LUT
    /// destination at media-relative is exactly the case ISO/CD 18619
    /// 4.2.5 exists for, and the shipped binary REFUSED it because
    /// `estimate_lut_destination_black` had no caller. It converts
    /// now, and BPC must move the answer.
    #[test]
    fn iso_estimator_is_reached_for_a_lut_destination() {
        let srgb = r"C:\Windows\System32\spool\drivers\color\sRGB Color Space Profile.icm";
        let swop = r"C:\Windows\System32\spool\drivers\color\USWebCoatedSWOP.icc";
        let (Ok(s), Ok(d)) = (std::fs::read(srgb), std::fs::read(swop)) else {
            eprintln!("skipped: system profiles absent");
            return;
        };
        let src = Profile::parse(&s).unwrap();
        let dst = Profile::parse(&d).unwrap();
        let plain = Chain::new(&src, &dst, Intent::MediaRelative).unwrap();
        let bpc = Chain::new(&src, &dst, Intent::MediaRelative)
            .unwrap()
            .with_bpc()
            .expect("ISO 4.2.5 estimates this; it used to be BpcEstimationUnsupported");
        let probe = [0.05, 0.05, 0.05];
        let a = plain.convert(&probe).unwrap();
        let b = bpc.convert(&probe).unwrap();
        assert_eq!(a.len(), 4, "CMYK destination");
        let moved = a
            .iter()
            .zip(&b)
            .map(|(x, y)| (x - y).abs())
            .fold(0.0f64, f64::max);
        assert!(moved > 0.0, "BPC must change a dark value; moved {moved}");
    }

    /// Gray THROUGH THE CHAIN (the librarian's audit found both gray
    /// tests stopped at the model — no gray value had ever traversed
    /// Chain). Real EIZO gray → system sRGB: neutrality of the full
    /// path on measured output, and channel counts 1 → 3.
    #[test]
    fn gray_through_chain_stays_neutral() {
        let gray = r"C:\Windows\System32\spool\drivers\color\ewgray22.icm";
        let srgb = r"C:\Windows\System32\spool\drivers\color\sRGB Color Space Profile.icm";
        let (Ok(g), Ok(d)) = (std::fs::read(gray), std::fs::read(srgb)) else {
            eprintln!("skipped: system profiles absent");
            return;
        };
        let src = Profile::parse(&g).unwrap();
        let dst = Profile::parse(&d).unwrap();
        let chain = Chain::new(&src, &dst, Intent::MediaRelative).unwrap();
        assert_eq!(chain.input_channels(), 1);
        assert_eq!(chain.output_channels(), 3);
        for &v in &[0.0, 0.25, 0.5, 0.75, 1.0] {
            let rgb = chain.convert(&[v]).unwrap();
            // Neutral in, neutral out: max channel spread bounded at
            // 2e-3 — the gray TRC and sRGB TRC quantisation floors
            // (1024-entry tables ≈ 1e-3 each), not a perceptual claim.
            let spread = rgb.iter().fold(0.0f64, |m, &c| m.max(c))
                - rgb.iter().fold(1.0f64, |m, &c| m.min(c));
            assert!(spread < 2e-3, "v={v} rgb={rgb:?} spread={spread}");
        }
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
