//! # Tag type decoding — Pass 2, batch 1 (the non-LUT types)
//!
//! Decodes tag *data* (the bytes a `TagEntry` points at, starting with
//! the 8-byte `icTagBase`) into typed representations. Covered here:
//! `curv`, `para`, `text`, `mluc`, `desc`, `ncl2`, `XYZ `, `sf32`.
//! The LUT family (`mft1`/`mft2`/`mAB `/`mBA `) is batch 2.
//!
//! ## Contracts (unchanged from the crate)
//!
//! - **Report, don't repair.** Rule violations that leave the layout
//!   decodable become [`TagIssue`]s alongside the decoded value; ones
//!   that make the layout unknowable (short data, an unhonourable
//!   `recordSize`) are `Err` — there is no partial result to be
//!   tempted by.
//! - **No colour maths.** `curveType` here is a `count` and raw
//!   entries; *evaluating* the curve is `iccce-cmm`'s job.
//! - **Attacker-controlled counts are bounded against the actual byte
//!   length before allocation** — same rule as the tag table.
//!
//! ## Sourcing
//!
//! Layouts cite `ICC_Spec/icc/icc__type__curve_parametric.md`
//! (primary_spec: ICC.1:2022 10.6/10.18) and
//! `icc__type__text_mluc_namedcolor2.md` (primary_spec for `ncl2`
//! clause 10.17; the others clause-number-verified, layouts
//! cross_verified_2src — `desc` is v2-only, has NO clause in
//! ICC.1:2022, and its layout is code-derived until ICC.1:2001-04 is
//! obtained, which the doc comments below repeat where it matters).

use crate::num::{S15Fixed16, Signature, U8Fixed8, XyzNumber, u16_be, u32_be};

/// Tag type signatures decoded by this module
/// (`icc__ref__signatures.md`).
pub mod sig {
    use crate::num::Signature;
    pub const CURV: Signature = Signature(0x6375_7276); // 'curv'
    pub const PARA: Signature = Signature(0x7061_7261); // 'para'
    pub const TEXT: Signature = Signature(0x7465_7874); // 'text'
    pub const MLUC: Signature = Signature(0x6D6C_7563); // 'mluc'
    pub const DESC: Signature = Signature(0x6465_7363); // 'desc'
    pub const NCL2: Signature = Signature(0x6E63_6C32); // 'ncl2'
    pub const XYZ: Signature = Signature(0x5859_5A20); // 'XYZ '
    pub const SF32: Signature = Signature(0x7366_3332); // 'sf32'
    pub const MFT1: Signature = Signature(0x6D66_7431); // 'mft1'
    pub const MFT2: Signature = Signature(0x6D66_7432); // 'mft2'
    pub const MAB: Signature = Signature(0x6D41_4220); // 'mAB '
    pub const MBA: Signature = Signature(0x6D42_4120); // 'mBA '
}

/// A reported (never repaired) issue inside a tag's data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TagIssue {
    /// `parametricCurveType` `funcType` outside 0–4 (ICC.1:2022
    /// Table 68 defines exactly five). Parameters are kept raw.
    UnknownParametricFunction { func_type: u16 },
    /// `parametricCurveType` carries a different parameter count than
    /// Table 67 states for its `funcType` (0→1, 1→3, 2→4, 3→5, 4→7).
    ParametricParamCountMismatch {
        func_type: u16,
        expected: usize,
        actual: usize,
    },
    /// `textType` contains bytes ≥ 0x80 — the type is 7-bit ASCII;
    /// reported, and the bytes kept verbatim, per rule 6 ("do not
    /// assume UTF-8").
    TextNotAscii,
    /// `textType` is not NUL-terminated. Occurs in the wild; the
    /// corpus directs trusting `size`, not the NUL.
    TextUnterminated,
    /// An `mluc` record's string `offset` is odd — it indexes UTF-16
    /// code units; lcms2 rejects outright (`Type_MLU_Read`).
    MlucOddOffset { record: usize },
    /// An `mluc` record's `offset + length` leaves the tag (including
    /// the u32-overflow case lcms2 guards: `(Offset+Len) < Len`).
    MlucRecordOutOfBounds { record: usize },
    /// An `mluc` record's offset points into the record table itself.
    MlucOffsetInRecordTable { record: usize },
    /// `ncl2`'s `nDeviceCoords` disagrees with the header's device
    /// channel count (checkable only by the caller who has the
    /// header; emitted by [`NamedColor2::check_device_channels`]).
    Ncl2DeviceCoordCountMismatch {
        n_device_coords: u32,
        header_channels: u32,
    },
    /// `XYZType` data length is not a multiple of 12: trailing bytes
    /// after the last whole `XYZNumber`. Whole entries are decoded;
    /// the remainder is reported.
    XyzTrailingBytes { remainder: usize },
    /// `s15Fixed16ArrayType` length not a multiple of 4.
    Sf32TrailingBytes { remainder: usize },
    /// The 4 reserved bytes of the `icTagBase` are non-zero (also
    /// checked table-side; repeated here because this module can be
    /// used on bytes from any source, e.g. a PDF stream).
    BaseReservedNonZero,
    /// `desc`'s Macintosh ScriptCode block (fixed 67 bytes) is absent
    /// or short — "the most frequently malformed structure in real v2
    /// profiles" (`icc__type__text_mluc_namedcolor2.md` §3).
    DescMacBlockShortOrMissing,
    /// `desc`'s Unicode block is absent or short.
    DescUnicodeShortOrMissing,
    /// A LUT-family pad byte that shall be zero is not.
    LutPadNonZero,
    /// `mAB `/`mBA ` CLUT: `gridPoints` entries beyond `inputChan`
    /// shall be zero (`icc__type__lutAtoB_lutBtoA.md`, code-derived).
    ClutGridPointsBeyondInputChan,
    /// `mAB `/`mBA ` with `offsetB == 0`: B curves appear in ALL four
    /// permitted element combinations (10.12.1/10.13.1, A23 CLOSED —
    /// the corpus's seventh pass), so an absent B chain is a
    /// malformation. Reported; the tag still decodes.
    LutAbMissingBCurves,
}

impl std::fmt::Display for TagIssue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownParametricFunction { func_type } => {
                write!(f, "parametric funcType {func_type} not in 0..=4")
            }
            Self::ParametricParamCountMismatch {
                func_type,
                expected,
                actual,
            } => write!(
                f,
                "parametric funcType {func_type}: {actual} params, Table 67 states {expected}"
            ),
            Self::TextNotAscii => write!(f, "textType contains non-ASCII bytes (kept verbatim)"),
            Self::TextUnterminated => write!(f, "textType lacks a terminating NUL"),
            Self::MlucOddOffset { record } => write!(f, "mluc record {record}: odd string offset"),
            Self::MlucRecordOutOfBounds { record } => {
                write!(f, "mluc record {record}: string exceeds tag bounds")
            }
            Self::MlucOffsetInRecordTable { record } => {
                write!(
                    f,
                    "mluc record {record}: offset points into the record table"
                )
            }
            Self::Ncl2DeviceCoordCountMismatch {
                n_device_coords,
                header_channels,
            } => write!(
                f,
                "ncl2 nDeviceCoords {n_device_coords} != header device channels {header_channels}"
            ),
            Self::XyzTrailingBytes { remainder } => {
                write!(
                    f,
                    "XYZType: {remainder} trailing byte(s) after last XYZNumber"
                )
            }
            Self::Sf32TrailingBytes { remainder } => {
                write!(
                    f,
                    "sf32: {remainder} trailing byte(s) after last s15Fixed16"
                )
            }
            Self::BaseReservedNonZero => write!(f, "tag base reserved bytes non-zero"),
            Self::DescMacBlockShortOrMissing => {
                write!(f, "desc: Macintosh ScriptCode block short or missing")
            }
            Self::DescUnicodeShortOrMissing => write!(f, "desc: Unicode block short or missing"),
            Self::LutPadNonZero => write!(f, "lut: pad byte(s) non-zero"),
            Self::ClutGridPointsBeyondInputChan => {
                write!(f, "clut: gridPoints beyond inputChan not zero")
            }
            Self::LutAbMissingBCurves => write!(
                f,
                "mAB/mBA: offsetB == 0, but B curves are in every permitted \
                 element combination (10.12.1/10.13.1)"
            ),
        }
    }
}

/// The layout could not be decoded at all. Terminal for this tag; the
/// tag-table entry itself remains represented.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TagDecodeError {
    /// Shorter than the 8-byte `icTagBase`.
    TooSmall { actual: usize },
    /// Data too short for the type's fixed fields.
    ShortForType {
        type_sig: Signature,
        needed: usize,
        actual: usize,
    },
    /// A count field requires more bytes than the tag has. Refused
    /// before allocation (attacker-controlled).
    CountOverflowsData { type_sig: Signature, count: u32 },
    /// `mluc` `recordSize != 12`: the record layout is then unknown;
    /// assuming 12 would misparse. Report-and-refuse (the corpus's
    /// stated direction, matching lcms2's refusal).
    MlucUnsupportedRecordSize { record_size: u32 },
    /// A LUT's computed size overflows even `u128` — dimensions are
    /// attacker-controlled bytes (`255^255` must refuse, not wrap).
    LutSizeOverflow { type_sig: Signature },
    /// A LUT's computed size exceeds the tag's actual bytes. Refused
    /// BEFORE allocation; `needed` is `u128` because the honest
    /// number may not fit anything smaller.
    LutSizeExceedsTag {
        type_sig: Signature,
        needed: u128,
        actual: usize,
    },
    /// `mAB `/`mBA ` CLUT `precision` not 1 or 2 — the sample width
    /// is unknowable ("No other value is legal",
    /// `icc__type__lutAtoB_lutBtoA.md`).
    ClutBadPrecision { precision: u8 },
    /// An `mAB `/`mBA ` curve chain broke at element `element`
    /// (byte `position` from tag start). Elements have no count field
    /// and no per-curve offsets — curve n must parse to find n+1, so
    /// everything after the break is unreachable and the tag is
    /// undecodable as a transform; the position is the report.
    CurveChainBroken {
        type_sig: Signature,
        element: u8,
        position: usize,
    },
}

impl std::fmt::Display for TagDecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooSmall { actual } => {
                write!(f, "tag data {actual} bytes, minimum 8 (icTagBase)")
            }
            Self::ShortForType {
                type_sig,
                needed,
                actual,
            } => {
                write!(f, "{type_sig}: needs {needed} bytes, has {actual}")
            }
            Self::CountOverflowsData { type_sig, count } => {
                write!(f, "{type_sig}: count {count} exceeds tag data")
            }
            Self::MlucUnsupportedRecordSize { record_size } => write!(
                f,
                "mluc recordSize {record_size} unsupported (10.15 says SHOULD contain 12; \
                 Table 54 prints the constant — corpus defect §17): record layout unknown, refused"
            ),
            Self::LutSizeOverflow { type_sig } => {
                write!(f, "{type_sig}: computed LUT size overflows, refused")
            }
            Self::LutSizeExceedsTag {
                type_sig,
                needed,
                actual,
            } => write!(f, "{type_sig}: LUT needs {needed} bytes, tag has {actual}"),
            Self::ClutBadPrecision { precision } => {
                write!(f, "clut precision {precision} (shall be 1 or 2): refused")
            }
            Self::CurveChainBroken {
                type_sig,
                element,
                position,
            } => write!(
                f,
                "{type_sig}: curve chain broken at element {element} (byte {position}); \
                 later elements unreachable"
            ),
        }
    }
}

/// `curveType` (`'curv'`, ICC.1:2022 clause 10.6, primary_spec).
///
/// The three cases of `count` — all three normative
/// (`icc__type__curve_parametric.md`):
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Curve {
    /// `count == 0`: "an identity response is assumed" (10.6,
    /// verbatim). A valid TRC, not an empty/invalid one — the corpus
    /// names erroring here as "the dangerous variant" of the count
    /// trap.
    Identity,
    /// `count == 1`: the single entry is a `u8Fixed8` GAMMA, not a
    /// table sample. "Gamma shall be interpreted as the exponent …
    /// and not as an inverse" (10.6, verbatim). Gamma 2.2 is stored
    /// as 0x0233 = 2.19921875 — not exactly 2.2, and cannot be.
    Gamma(U8Fixed8),
    /// `count ≥ 2`: uniformly spaced samples over \[0,1\], 0x0000→0.0,
    /// 0xFFFF→1.0, increment 1/(n−1). Interpolation between entries
    /// is normatively LINEAR (10.6 verbatim; ambiguity A15 resolved —
    /// the first corpus pass wrongly called this silent).
    Table(Vec<u16>),
}

/// `parametricCurveType` (`'para'`, ICC.1:2022 clause 10.18,
/// primary_spec; **v4-only** — divergence D6).
///
/// Parameters in the fixed order `g, a, b, c, d, e, f` (Table 67).
/// Held raw — evaluation, its `a == 0` and `pow(neg, frac)` guards,
/// and the lcms2 type-numbering off-by-one (lcms2 type = funcType + 1)
/// are `iccce-cmm`'s concern and documented there when built.
///
/// Note held for the evaluator, recorded now: ICC.1:2010's Table 68
/// DIFFERED (10.18 NOTE 2; what changed is unsourced until that PDF is
/// obtained — divergence D10/A31), and types 3–4 are not required to
/// be continuous at X = d (A18: do not "fix" a discontinuous curve).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParametricCurve {
    pub func_type: u16,
    pub params: Vec<S15Fixed16>,
}

/// `textType` (`'text'`, clause 10.24; layout cross_verified_2src).
/// 7-bit ASCII, NUL included in the tag size. Bytes kept verbatim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextAscii {
    /// Raw bytes after the base, INCLUDING any terminating NUL —
    /// the representation is the file's, not a cleaned-up string.
    pub bytes: Vec<u8>,
}

impl TextAscii {
    /// The text up to the first NUL (or all of it if unterminated),
    /// lossily rendered. A convenience view; `bytes` is the truth.
    pub fn to_string_lossy(&self) -> String {
        let end = self
            .bytes
            .iter()
            .position(|&b| b == 0)
            .unwrap_or(self.bytes.len());
        String::from_utf8_lossy(&self.bytes[..end]).into_owned()
    }
}

/// One `mluc` record: language/country as packed ASCII pairs
/// (`'en'` = 0x656E — compared as `u16`, never decoded to str), and
/// the UTF-16BE string bytes (NOT NUL-terminated, length in BYTES).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MlucRecord {
    pub language: u16,
    pub country: u16,
    /// Raw UTF-16BE bytes as stored. `None` when the record's
    /// offset/length failed bounds checks (the record row itself is
    /// still represented — report, don't drop).
    pub utf16be: Option<Vec<u8>>,
}

impl MlucRecord {
    /// Decode the UTF-16BE string, lossily. Convenience; raw bytes
    /// are the truth.
    pub fn to_string_lossy(&self) -> Option<String> {
        let raw = self.utf16be.as_ref()?;
        let units: Vec<u16> = raw
            .chunks_exact(2)
            .map(|c| u16::from_be_bytes([c[0], c[1]]))
            .collect();
        Some(String::from_utf16_lossy(&units))
    }
}

/// `multiLocalizedUnicodeType` (`'mluc'`, clause 10.15; layout
/// cross_verified_2src against lcms2 `Type_MLU_Read`; **v4** — D3).
///
/// `count == 0` is legal (an empty mluc). Record selection when no
/// exact language match exists is unspecified (ambiguity A25, still
/// open) — this module only represents; selection policy is a
/// consumer decision that must cite A25.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mluc {
    pub records: Vec<MlucRecord>,
}

/// `textDescriptionType` (`'desc'`, **v2-only** — D3).
///
/// ★ Sourcing caveat, the strongest in this module: `desc` has NO
/// clause in ICC.1:2022 (removed in v4); its normative home is
/// ICC.1:2001-04, NOT OBTAINED. This layout is code-derived
/// (`icc__type__text_mluc_namedcolor2.md` §3) — "parse it, but you
/// cannot cite it". The Mac ScriptCode block (fixed 67 bytes
/// regardless of its length byte) is the most frequently malformed
/// structure in real v2 profiles; every read is bounded and shortfalls
/// are reported, not repaired.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextDescription {
    /// ASCII block (count includes its NUL); bytes kept verbatim.
    pub ascii: Vec<u8>,
    /// Unicode block: (language code, raw UCS-2 bytes), if present
    /// and in bounds.
    pub unicode: Option<(u32, Vec<u8>)>,
    /// Mac block: (script code, length byte, raw 67-byte buffer), if
    /// present in full.
    pub mac: Option<(u16, u8, Vec<u8>)>,
}

/// One named colour (`ncl2` entry).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedColorEntry {
    /// 32-byte root name field, verbatim (NUL-terminated within).
    /// Full name = prefix + root + suffix, concatenated — "the name
    /// is not stored whole anywhere" (clause 10.17 discussion).
    pub root_name: [u8; 32],
    /// PCS coordinates, always 3 × uInt16 — and in PCSLAB they use
    /// the LEGACY 16-bit encoding in a profile of ANY version
    /// (ICC.1:2022 10.17 verbatim, A26 resolved 2026-08-11). Held
    /// raw here; decoding them is the CMM's job and MUST use the
    /// legacy tables — a 0.4% L* error on a brand colour is the
    /// least acceptable defect in the system.
    pub pcs_coords: [u16; 3],
    /// Device coordinates, 0x0000→min, 0xFFFF→max (full-range 0..1 —
    /// NOT the u1Fixed15 PCS encoding; different scale, same struct).
    pub device_coords: Vec<u16>,
}

/// `namedColor2Type` (`'ncl2'`, ICC.1:2022 clause 10.17,
/// primary_spec).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedColor2 {
    pub vendor_flag: u32,
    pub n_device_coords: u32,
    pub prefix: [u8; 32],
    pub suffix: [u8; 32],
    pub entries: Vec<NamedColorEntry>,
}

impl NamedColor2 {
    /// Cross-check `nDeviceCoords` against the header's device
    /// channel count (clause 10.17: they must agree). Separated out
    /// because only the caller holds the header; returns the issue to
    /// report rather than judging silently.
    pub fn check_device_channels(&self, header_channels: u32) -> Option<TagIssue> {
        (self.n_device_coords != header_channels).then_some(
            TagIssue::Ncl2DeviceCoordCountMismatch {
                n_device_coords: self.n_device_coords,
                header_channels,
            },
        )
    }
}

/// A decoded tag: the typed data plus every issue found on the way.
#[derive(Debug, Clone, PartialEq)]
pub struct DecodedTag {
    pub type_sig: Signature,
    pub data: TagData,
    pub issues: Vec<TagIssue>,
}

/// The typed representations this batch covers. `Unknown` carries the
/// signature so callers can distinguish "iccce does not decode this
/// yet" (e.g. the LUT family, batch 2) from corruption — the same
/// refuse-by-name posture as iccMAX.
#[derive(Debug, Clone, PartialEq)]
pub enum TagData {
    Curve(Curve),
    ParametricCurve(ParametricCurve),
    Text(TextAscii),
    Mluc(Mluc),
    TextDescription(TextDescription),
    NamedColor2(NamedColor2),
    /// `XYZType` — one or more XYZNumbers (`icSigXYZType` and
    /// `icSigXYZArrayType` are THE SAME VALUE; the array is the
    /// singular type with n ≥ 1).
    Xyz(Vec<XyzNumber>),
    /// `s15Fixed16ArrayType` — a bare array with no self-describing
    /// shape. The principal use is `chad` (exactly 9, row-major 3×3);
    /// that arity check belongs to the consumer that knows the tag
    /// signature, not to this type decoder.
    S15Fixed16Array(Vec<S15Fixed16>),
    /// `lut8Type` (`'mft1'`) — general 8-bit encodings; NOT in the
    /// legacy-Lab set (6.3.4.2 NOTE 3: "and only those tag types").
    Lut8(crate::lut::Lut8),
    /// `lut16Type` (`'mft2'`) — Lab PCS data uses the LEGACY 16-bit
    /// encoding in a profile of any version (see `lut` module doc).
    Lut16(crate::lut::Lut16),
    /// `lutAToBType` (`'mAB '`): A → CLUT → M → Matrix → B.
    LutAToB(crate::lut::LutAB),
    /// `lutBToAType` (`'mBA '`): same storage, reverse traversal.
    LutBToA(crate::lut::LutAB),
    Unknown,
}

/// Decode one tag's data (starting at its `icTagBase`).
pub fn decode(data: &[u8]) -> Result<DecodedTag, TagDecodeError> {
    if data.len() < 8 {
        return Err(TagDecodeError::TooSmall { actual: data.len() });
    }
    let type_sig = Signature::read(data, 0).expect("length checked");
    let mut issues = Vec::new();
    if data[4..8].iter().any(|&b| b != 0) {
        issues.push(TagIssue::BaseReservedNonZero);
    }
    let body = &data[8..];

    let tag_data = match type_sig {
        sig::CURV => TagData::Curve(decode_curve(type_sig, body)?),
        sig::PARA => TagData::ParametricCurve(decode_parametric(type_sig, body, &mut issues)?),
        sig::TEXT => TagData::Text(decode_text(body, &mut issues)),
        sig::MLUC => TagData::Mluc(decode_mluc(type_sig, data, &mut issues)?),
        sig::DESC => TagData::TextDescription(decode_desc(type_sig, body, &mut issues)?),
        sig::NCL2 => TagData::NamedColor2(decode_ncl2(type_sig, body)?),
        sig::XYZ => TagData::Xyz(decode_xyz(body, &mut issues)),
        sig::SF32 => TagData::S15Fixed16Array(decode_sf32(body, &mut issues)),
        // LUT family: offsets inside are tag-start-relative, so these
        // take `data`, not `body`.
        sig::MFT1 => TagData::Lut8(crate::lut::decode_lut8(type_sig, data, &mut issues)?),
        sig::MFT2 => TagData::Lut16(crate::lut::decode_lut16(type_sig, data, &mut issues)?),
        sig::MAB => TagData::LutAToB(crate::lut::decode_lut_ab(type_sig, data, &mut issues)?),
        sig::MBA => TagData::LutBToA(crate::lut::decode_lut_ab(type_sig, data, &mut issues)?),
        _ => TagData::Unknown,
    };
    Ok(DecodedTag {
        type_sig,
        data: tag_data,
        issues,
    })
}

fn decode_curve(type_sig: Signature, body: &[u8]) -> Result<Curve, TagDecodeError> {
    let count = u32_be(body, 0).ok_or(TagDecodeError::ShortForType {
        type_sig,
        needed: 4,
        actual: body.len(),
    })?;
    match count {
        0 => Ok(Curve::Identity),
        1 => U8Fixed8::read(body, 4)
            .map(Curve::Gamma)
            .ok_or(TagDecodeError::ShortForType {
                type_sig,
                needed: 6,
                actual: body.len(),
            }),
        n => {
            // Bound before allocating: n × 2 bytes must exist.
            // Saturating: an overflowing product can only exceed the
            // real byte length, so saturation preserves the refusal.
            let needed = 4usize.saturating_add((n as usize).saturating_mul(2));
            if needed > body.len() {
                return Err(TagDecodeError::CountOverflowsData { type_sig, count: n });
            }
            let table = (0..n as usize)
                .map(|i| u16_be(body, 4 + 2 * i).expect("bounded above"))
                .collect();
            Ok(Curve::Table(table))
        }
    }
}

fn decode_parametric(
    type_sig: Signature,
    body: &[u8],
    issues: &mut Vec<TagIssue>,
) -> Result<ParametricCurve, TagDecodeError> {
    let func_type = u16_be(body, 0).ok_or(TagDecodeError::ShortForType {
        type_sig,
        needed: 4,
        actual: body.len(),
    })?;
    // Table 67 parameter counts per funcType: 0→1, 1→3, 2→4, 3→5, 4→7.
    let expected = match func_type {
        0 => Some(1),
        1 => Some(3),
        2 => Some(4),
        3 => Some(5),
        4 => Some(7),
        _ => {
            issues.push(TagIssue::UnknownParametricFunction { func_type });
            None
        }
    };
    // Read every whole s15Fixed16 present — the file's content, not
    // the expected shape, is what gets represented.
    let avail = (body.len().saturating_sub(4)) / 4;
    let params: Vec<S15Fixed16> = (0..avail)
        .map(|i| S15Fixed16::read(body, 4 + 4 * i).expect("bounded"))
        .collect();
    if let Some(exp) = expected {
        if params.len() != exp {
            issues.push(TagIssue::ParametricParamCountMismatch {
                func_type,
                expected: exp,
                actual: params.len(),
            });
        }
    }
    Ok(ParametricCurve { func_type, params })
}

fn decode_text(body: &[u8], issues: &mut Vec<TagIssue>) -> TextAscii {
    if body.iter().any(|&b| b >= 0x80) {
        issues.push(TagIssue::TextNotAscii);
    }
    if !body.contains(&0) {
        issues.push(TagIssue::TextUnterminated);
    }
    TextAscii {
        bytes: body.to_vec(),
    }
}

/// `mluc` needs the WHOLE tag (offsets are from the tag start), hence
/// `data` not `body`.
fn decode_mluc(
    type_sig: Signature,
    data: &[u8],
    issues: &mut Vec<TagIssue>,
) -> Result<Mluc, TagDecodeError> {
    let count = u32_be(data, 8).ok_or(TagDecodeError::ShortForType {
        type_sig,
        needed: 16,
        actual: data.len(),
    })?;
    let record_size = u32_be(data, 12).ok_or(TagDecodeError::ShortForType {
        type_sig,
        needed: 16,
        actual: data.len(),
    })?;
    // recordSize exists for forward extension; anything but 12 makes
    // the record layout unknown. Report-and-refuse, matching lcms2.
    if record_size != 12 {
        return Err(TagDecodeError::MlucUnsupportedRecordSize { record_size });
    }
    let table_end = 16usize.saturating_add((count as usize).saturating_mul(12));
    if table_end > data.len() {
        return Err(TagDecodeError::CountOverflowsData { type_sig, count });
    }

    let mut records = Vec::with_capacity(count as usize);
    for i in 0..count as usize {
        let base = 16 + 12 * i;
        let language = u16_be(data, base).expect("bounded");
        let country = u16_be(data, base + 2).expect("bounded");
        let length = u32_be(data, base + 4).expect("bounded") as usize;
        let offset = u32_be(data, base + 8).expect("bounded") as usize;

        // The lcms2 hardening set (`Type_MLU_Read`, verbatim in the
        // corpus): odd offset; offset into the record table; and the
        // overflow-aware bounds check.
        let mut ok = true;
        if offset % 2 != 0 {
            issues.push(TagIssue::MlucOddOffset { record: i });
            ok = false;
        }
        if offset < table_end {
            issues.push(TagIssue::MlucOffsetInRecordTable { record: i });
            ok = false;
        }
        let end = offset.checked_add(length);
        if end.is_none_or(|e| e > data.len()) {
            issues.push(TagIssue::MlucRecordOutOfBounds { record: i });
            ok = false;
        }
        let utf16be = if ok {
            Some(data[offset..offset + length].to_vec())
        } else {
            None
        };
        records.push(MlucRecord {
            language,
            country,
            utf16be,
        });
    }
    Ok(Mluc { records })
}

fn decode_desc(
    type_sig: Signature,
    body: &[u8],
    issues: &mut Vec<TagIssue>,
) -> Result<TextDescription, TagDecodeError> {
    let ascii_count = u32_be(body, 0).ok_or(TagDecodeError::ShortForType {
        type_sig,
        needed: 4,
        actual: body.len(),
    })? as usize;
    let ascii_end = 4usize.saturating_add(ascii_count);
    if ascii_end > body.len() {
        // `ascii_count` came from a u32 read, so the cast is lossless.
        #[allow(clippy::cast_possible_truncation)]
        let count = ascii_count as u32;
        return Err(TagDecodeError::CountOverflowsData { type_sig, count });
    }
    let ascii = body[4..ascii_end].to_vec();

    // Unicode block: u32 language code + u32 count + UCS-2 data.
    let mut cursor = ascii_end;
    let unicode = match (u32_be(body, cursor), u32_be(body, cursor + 4)) {
        (Some(lang), Some(ucount)) => {
            let bytes = (ucount as usize).checked_mul(2);
            let start = cursor + 8;
            match bytes.and_then(|b| start.checked_add(b)) {
                Some(end) if end <= body.len() => {
                    cursor = end;
                    Some((lang, body[start..end].to_vec()))
                }
                _ => {
                    issues.push(TagIssue::DescUnicodeShortOrMissing);
                    None
                }
            }
        }
        _ => {
            issues.push(TagIssue::DescUnicodeShortOrMissing);
            None
        }
    };

    // Mac block: u16 script code + u8 length + FIXED 67-byte buffer,
    // regardless of the length byte.
    let mac = match (u16_be(body, cursor), body.get(cursor + 2).copied()) {
        (Some(code), Some(len)) if cursor + 3 + 67 <= body.len() => {
            Some((code, len, body[cursor + 3..cursor + 3 + 67].to_vec()))
        }
        _ => {
            issues.push(TagIssue::DescMacBlockShortOrMissing);
            None
        }
    };

    Ok(TextDescription {
        ascii,
        unicode,
        mac,
    })
}

fn decode_ncl2(type_sig: Signature, body: &[u8]) -> Result<NamedColor2, TagDecodeError> {
    // Fixed fields: vendorFlag(4) count(4) nDeviceCoords(4)
    // prefix(32) suffix(32) = 76 bytes (offsets 8..84 from tag start).
    if body.len() < 76 {
        return Err(TagDecodeError::ShortForType {
            type_sig,
            needed: 76,
            actual: body.len(),
        });
    }
    let vendor_flag = u32_be(body, 0).expect("checked");
    let count = u32_be(body, 4).expect("checked");
    let n_device_coords = u32_be(body, 8).expect("checked");
    let prefix: [u8; 32] = body[12..44].try_into().expect("checked");
    let suffix: [u8; 32] = body[44..76].try_into().expect("checked");

    // Entry stride: 32 (root) + 6 (pcs) + 2·nDeviceCoords. With
    // nDeviceCoords == 0 the device field is simply absent (clause
    // 10.17 / ICC comment verbatim: "this field is not given") —
    // stride 38, a normal spot-library shape, not an error.
    let stride = 38usize.saturating_add((n_device_coords as usize).saturating_mul(2));
    let needed = 76usize.saturating_add((count as usize).saturating_mul(stride));
    if needed > body.len() {
        return Err(TagDecodeError::CountOverflowsData { type_sig, count });
    }

    let mut entries = Vec::with_capacity(count as usize);
    for i in 0..count as usize {
        let base = 76 + i * stride;
        let root_name: [u8; 32] = body[base..base + 32].try_into().expect("bounded");
        let pcs_coords = [
            u16_be(body, base + 32).expect("bounded"),
            u16_be(body, base + 34).expect("bounded"),
            u16_be(body, base + 36).expect("bounded"),
        ];
        let device_coords = (0..n_device_coords as usize)
            .map(|c| u16_be(body, base + 38 + 2 * c).expect("bounded"))
            .collect();
        entries.push(NamedColorEntry {
            root_name,
            pcs_coords,
            device_coords,
        });
    }
    Ok(NamedColor2 {
        vendor_flag,
        n_device_coords,
        prefix,
        suffix,
        entries,
    })
}

fn decode_xyz(body: &[u8], issues: &mut Vec<TagIssue>) -> Vec<XyzNumber> {
    let n = body.len() / 12;
    let remainder = body.len() % 12;
    if remainder != 0 {
        issues.push(TagIssue::XyzTrailingBytes { remainder });
    }
    (0..n)
        .map(|i| XyzNumber::read(body, 12 * i).expect("bounded"))
        .collect()
}

fn decode_sf32(body: &[u8], issues: &mut Vec<TagIssue>) -> Vec<S15Fixed16> {
    let n = body.len() / 4;
    let remainder = body.len() % 4;
    if remainder != 0 {
        issues.push(TagIssue::Sf32TrailingBytes { remainder });
    }
    (0..n)
        .map(|i| S15Fixed16::read(body, 4 * i).expect("bounded"))
        .collect()
}

/// Parse ONE curve element (`curv` or `para`, with its own 8-byte
/// base) at absolute position `pos` inside `mAB `/`mBA ` tag data.
/// Returns the element and its UNPADDED byte length (the caller
/// applies the 4-byte padding rule between elements), or `None` when
/// the element is unparseable — which the caller reports positionally
/// as [`TagDecodeError::CurveChainBroken`], because with no count
/// field and no per-curve offsets, everything after it is
/// unreachable (`icc__type__lutAtoB_lutBtoA.md`).
///
/// A `para` with an unknown `funcType` is unparseable BY CONSTRUCTION
/// here even though the standalone decoder tolerates it: without
/// Table 67's parameter count the element's length is unknowable, and
/// guessing a length would silently corrupt every element after it.
pub(crate) fn decode_curve_element(
    data: &[u8],
    pos: usize,
    issues: &mut Vec<TagIssue>,
) -> Option<(crate::lut::CurveElement, usize)> {
    use crate::lut::CurveElement;
    let ts = Signature::read(data, pos)?;
    let reserved = data.get(pos + 4..pos + 8)?;
    if reserved.iter().any(|&b| b != 0) {
        issues.push(TagIssue::BaseReservedNonZero);
    }
    match ts {
        sig::CURV => {
            let count = u32_be(data, pos + 8)? as usize;
            match count {
                0 => Some((CurveElement::Curve(Curve::Identity), 12)),
                1 => {
                    let g = U8Fixed8::read(data, pos + 12)?;
                    Some((CurveElement::Curve(Curve::Gamma(g)), 14))
                }
                n => {
                    let end = pos.checked_add(12)?.checked_add(n.checked_mul(2)?)?;
                    if end > data.len() {
                        return None;
                    }
                    let table = (0..n)
                        .map(|i| u16_be(data, pos + 12 + 2 * i).expect("bounded"))
                        .collect();
                    Some((CurveElement::Curve(Curve::Table(table)), 12 + 2 * n))
                }
            }
        }
        sig::PARA => {
            let func_type = u16_be(data, pos + 8)?;
            let n = match func_type {
                0 => 1,
                1 => 3,
                2 => 4,
                3 => 5,
                4 => 7,
                _ => return None, // length unknowable, see doc comment
            };
            let end = pos + 12 + 4 * n;
            if end > data.len() {
                return None;
            }
            let params = (0..n)
                .map(|i| S15Fixed16::read(data, pos + 12 + 4 * i).expect("bounded"))
                .collect();
            Some((
                CurveElement::Parametric(ParametricCurve { func_type, params }),
                12 + 4 * n,
            ))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: an 8-byte icTagBase with the given type signature.
    fn base(type_sig: &[u8; 4]) -> Vec<u8> {
        let mut v = type_sig.to_vec();
        v.extend_from_slice(&[0u8; 4]);
        v
    }

    /// count == 0 is a valid IDENTITY curve, not an error — the
    /// "dangerous variant" of the count trap
    /// (`icc__type__curve_parametric.md`).
    #[test]
    fn curve_count_zero_is_identity() {
        let mut t = base(b"curv");
        t.extend_from_slice(&0u32.to_be_bytes());
        let d = decode(&t).unwrap();
        assert_eq!(d.data, TagData::Curve(Curve::Identity));
        assert!(d.issues.is_empty());
    }

    /// count == 1: 0x0233 is gamma 2.19921875 exactly — the corpus's
    /// own worked value ("gamma 2.2 is stored as 0x0233 = 563;
    /// 563/256 = 2.19921875 — not exactly 2.2, and cannot be").
    /// Expectation from the corpus's arithmetic, not this code.
    #[test]
    fn curve_count_one_is_u8fixed8_gamma() {
        let mut t = base(b"curv");
        t.extend_from_slice(&1u32.to_be_bytes());
        t.extend_from_slice(&0x0233u16.to_be_bytes());
        let d = decode(&t).unwrap();
        match d.data {
            TagData::Curve(Curve::Gamma(g)) => assert_eq!(g.to_f64(), 2.19921875),
            other => panic!("expected gamma, got {other:?}"),
        }
    }

    #[test]
    fn curve_table_decodes_and_hostile_count_refused() {
        let mut t = base(b"curv");
        t.extend_from_slice(&3u32.to_be_bytes());
        for v in [0u16, 0x8000, 0xFFFF] {
            t.extend_from_slice(&v.to_be_bytes());
        }
        let d = decode(&t).unwrap();
        assert_eq!(
            d.data,
            TagData::Curve(Curve::Table(vec![0, 0x8000, 0xFFFF]))
        );

        let mut hostile = base(b"curv");
        hostile.extend_from_slice(&u32::MAX.to_be_bytes());
        assert!(matches!(
            decode(&hostile),
            Err(TagDecodeError::CountOverflowsData { .. })
        ));
    }

    /// funcType 3 (the sRGB-shaped curve) carries exactly 5 params
    /// per Table 67; a wrong count is reported, and the params the
    /// file actually has are kept.
    #[test]
    fn parametric_param_count_checked_against_table_67() {
        let mut t = base(b"para");
        t.extend_from_slice(&3u16.to_be_bytes());
        t.extend_from_slice(&[0u8; 2]); // pad
        for _ in 0..5 {
            t.extend_from_slice(&0x0001_0000u32.to_be_bytes()); // 1.0
        }
        let d = decode(&t).unwrap();
        assert!(d.issues.is_empty());
        match &d.data {
            TagData::ParametricCurve(p) => {
                assert_eq!(p.func_type, 3);
                assert_eq!(p.params.len(), 5);
                assert_eq!(p.params[0].to_f64(), 1.0);
            }
            other => panic!("expected para, got {other:?}"),
        }

        // Same tag with one param missing: reported, not repaired.
        let mut short = base(b"para");
        short.extend_from_slice(&3u16.to_be_bytes());
        short.extend_from_slice(&[0u8; 2]);
        for _ in 0..4 {
            short.extend_from_slice(&0x0001_0000u32.to_be_bytes());
        }
        let d = decode(&short).unwrap();
        assert!(d.issues.iter().any(|i| matches!(
            i,
            TagIssue::ParametricParamCountMismatch {
                func_type: 3,
                expected: 5,
                actual: 4
            }
        )));
    }

    #[test]
    fn text_reports_unterminated_and_non_ascii_verbatim() {
        let mut t = base(b"text");
        t.extend_from_slice(b"hi\xC3\xA9"); // no NUL, non-ASCII
        let d = decode(&t).unwrap();
        assert!(d.issues.contains(&TagIssue::TextNotAscii));
        assert!(d.issues.contains(&TagIssue::TextUnterminated));
        match d.data {
            TagData::Text(txt) => assert_eq!(txt.bytes, b"hi\xC3\xA9"), // kept verbatim
            other => panic!("{other:?}"),
        }
    }

    /// One-record mluc with 'enUS' and "Hi" in UTF-16BE.
    #[test]
    fn mluc_round_trip() {
        let mut t = base(b"mluc");
        t.extend_from_slice(&1u32.to_be_bytes()); // count
        t.extend_from_slice(&12u32.to_be_bytes()); // recordSize
        t.extend_from_slice(&0x656Eu16.to_be_bytes()); // 'en'
        t.extend_from_slice(&0x5553u16.to_be_bytes()); // 'US'
        t.extend_from_slice(&4u32.to_be_bytes()); // length BYTES
        t.extend_from_slice(&28u32.to_be_bytes()); // offset from tag start
        t.extend_from_slice(&[0x00, b'H', 0x00, b'i']); // UTF-16BE
        let d = decode(&t).unwrap();
        assert!(d.issues.is_empty());
        match &d.data {
            TagData::Mluc(m) => {
                assert_eq!(m.records[0].language, 0x656E);
                assert_eq!(m.records[0].to_string_lossy().unwrap(), "Hi");
            }
            other => panic!("{other:?}"),
        }
    }

    /// recordSize != 12 makes the layout unknown: report-and-refuse
    /// (the corpus's direction), never assume-12-and-misparse.
    #[test]
    fn mluc_bad_record_size_refused_by_name() {
        let mut t = base(b"mluc");
        t.extend_from_slice(&0u32.to_be_bytes());
        t.extend_from_slice(&16u32.to_be_bytes());
        assert_eq!(
            decode(&t),
            Err(TagDecodeError::MlucUnsupportedRecordSize { record_size: 16 })
        );
    }

    /// Odd offsets / out-of-bounds records are reported and the
    /// record ROW is kept (utf16be = None) — represent the broken
    /// directory, don't drop it.
    #[test]
    fn mluc_bad_offset_reported_row_kept() {
        let mut t = base(b"mluc");
        t.extend_from_slice(&1u32.to_be_bytes());
        t.extend_from_slice(&12u32.to_be_bytes());
        t.extend_from_slice(&0x656Eu16.to_be_bytes());
        t.extend_from_slice(&0x5553u16.to_be_bytes());
        t.extend_from_slice(&4u32.to_be_bytes());
        t.extend_from_slice(&29u32.to_be_bytes()); // ODD offset
        t.extend_from_slice(&[0u8; 6]);
        let d = decode(&t).unwrap();
        assert!(
            d.issues
                .iter()
                .any(|i| matches!(i, TagIssue::MlucOddOffset { record: 0 }))
        );
        match &d.data {
            TagData::Mluc(m) => {
                assert_eq!(m.records.len(), 1);
                assert_eq!(m.records[0].utf16be, None);
            }
            other => panic!("{other:?}"),
        }
    }

    /// ncl2 with 2 colours and nDeviceCoords == 0 — the "field is not
    /// given" case (clause 10.17): stride 38, legal, a normal
    /// spot-library shape.
    #[test]
    fn ncl2_zero_device_coords_is_legal() {
        let mut t = base(b"ncl2");
        t.extend_from_slice(&0u32.to_be_bytes()); // vendorFlag
        t.extend_from_slice(&2u32.to_be_bytes()); // count
        t.extend_from_slice(&0u32.to_be_bytes()); // nDeviceCoords
        t.extend_from_slice(&[0u8; 64]); // prefix + suffix
        for name in [b"PANTONE Red\0", b"PANTONE Blu\0"] {
            let mut root = [0u8; 32];
            root[..name.len()].copy_from_slice(&name[..]);
            t.extend_from_slice(&root);
            for c in [0xFF00u16, 0x8080, 0x8080] {
                t.extend_from_slice(&c.to_be_bytes());
            }
        }
        let d = decode(&t).unwrap();
        match &d.data {
            TagData::NamedColor2(n) => {
                assert_eq!(n.entries.len(), 2);
                assert_eq!(n.entries[0].pcs_coords, [0xFF00, 0x8080, 0x8080]);
                assert!(n.entries[0].device_coords.is_empty());
                // Device-channel cross-check emits an issue on mismatch.
                assert!(n.check_device_channels(0).is_none());
                assert!(n.check_device_channels(4).is_some());
            }
            other => panic!("{other:?}"),
        }
    }

    /// XYZType with a negative component — sign extension through the
    /// tag path (bkpt and wide-gamut primaries carry negatives).
    #[test]
    fn xyz_type_preserves_negative_components() {
        let mut t = base(b"XYZ ");
        t.extend_from_slice(&0xFFFF_0000u32.to_be_bytes()); // -1.0
        t.extend_from_slice(&0x0001_0000u32.to_be_bytes()); // 1.0
        t.extend_from_slice(&0x0000_0000u32.to_be_bytes()); // 0.0
        let d = decode(&t).unwrap();
        match &d.data {
            TagData::Xyz(v) => {
                assert_eq!(v.len(), 1);
                assert_eq!(v[0].x.to_f64(), -1.0);
                assert_eq!(v[0].y.to_f64(), 1.0);
            }
            other => panic!("{other:?}"),
        }
    }

    /// sf32 (chad-shaped): 9 values decode; a 10th trailing partial
    /// byte pair is reported. (The 9-element arity rule for 'chad'
    /// specifically belongs to the consumer, per the type doc.)
    #[test]
    fn sf32_decodes_and_reports_trailing() {
        let mut t = base(b"sf32");
        for _ in 0..9 {
            t.extend_from_slice(&0x0001_0000u32.to_be_bytes());
        }
        t.extend_from_slice(&[0u8; 2]); // trailing fragment
        let d = decode(&t).unwrap();
        match &d.data {
            TagData::S15Fixed16Array(v) => assert_eq!(v.len(), 9),
            other => panic!("{other:?}"),
        }
        assert!(
            d.issues
                .iter()
                .any(|i| matches!(i, TagIssue::Sf32TrailingBytes { remainder: 2 }))
        );
    }

    /// A type iccce does not decode comes back Unknown-with-sig,
    /// distinguishable from corruption. ('view' is a real ICC type —
    /// viewingConditionsType — that iccce has not implemented; it was
    /// 'mAB ' until batch 2 made that decodable, which this test
    /// caught, as designed.)
    #[test]
    fn unknown_type_is_named_not_corrupt() {
        let mut t = base(b"view");
        t.extend_from_slice(&[0u8; 24]);
        let d = decode(&t).unwrap();
        assert_eq!(d.data, TagData::Unknown);
        assert_eq!(d.type_sig.to_string(), "'view'");
    }

    /// mft2: 3-in/1-out, clutPoints=2, tiny but complete. Layout per
    /// `icc__type__lut8_lut16.md` Table 40 (primary_spec): head 52
    /// bytes, then input tables (3×2 entries), CLUT (2³×1 = 8), output
    /// table (1×2).
    #[test]
    fn lut16_decodes_and_sections_partition() {
        let mut t = base(b"mft2");
        t.extend_from_slice(&[3, 1, 2, 0]); // in, out, clutPoints, pad
        // Identity 3×3.
        for i in 0..9u32 {
            let v = if i % 4 == 0 { 0x0001_0000u32 } else { 0 };
            t.extend_from_slice(&v.to_be_bytes());
        }
        t.extend_from_slice(&2u16.to_be_bytes()); // inputEnt
        t.extend_from_slice(&2u16.to_be_bytes()); // outputEnt
        for v in 0..6u16 {
            t.extend_from_slice(&v.to_be_bytes()); // input tables
        }
        for v in 100..108u16 {
            t.extend_from_slice(&v.to_be_bytes()); // CLUT
        }
        for v in [0u16, 0xFFFF] {
            t.extend_from_slice(&v.to_be_bytes()); // output table
        }
        let d = decode(&t).unwrap();
        match &d.data {
            TagData::Lut16(l) => {
                assert_eq!((l.input_chan, l.output_chan, l.clut_points), (3, 1, 2));
                assert!(l.matrix_is_identity());
                assert_eq!(l.input_tables, vec![0, 1, 2, 3, 4, 5]);
                assert_eq!(l.clut, (100..108).collect::<Vec<u16>>());
                assert_eq!(l.output_tables, vec![0, 0xFFFF]);
            }
            other => panic!("{other:?}"),
        }
        assert!(d.issues.is_empty());
    }

    /// Hostile mft2 dimensions refuse BEFORE allocation:
    /// clutPoints=255, inputChan=255 → 255^255 overflows u128.
    #[test]
    fn lut16_hostile_dimensions_refused() {
        let mut t = base(b"mft2");
        t.extend_from_slice(&[255, 255, 255, 0]);
        t.extend_from_slice(&[0u8; 40]); // matrix + ents
        assert!(matches!(
            decode(&t),
            Err(TagDecodeError::LutSizeOverflow { .. })
        ));
    }

    /// mft1 has NO ent fields — tables are exactly 256 entries
    /// (clause 10.11). 1-in/1-out, clutPoints=2.
    #[test]
    fn lut8_fixed_256_tables() {
        let mut t = base(b"mft1");
        t.extend_from_slice(&[1, 1, 2, 0]);
        for i in 0..9u32 {
            let v = if i % 4 == 0 { 0x0001_0000u32 } else { 0 };
            t.extend_from_slice(&v.to_be_bytes());
        }
        t.extend_from_slice(&[7u8; 256]); // input table
        t.extend_from_slice(&[9u8; 2]); // CLUT: 2^1 × 1
        t.extend_from_slice(&[8u8; 256]); // output table
        let d = decode(&t).unwrap();
        match &d.data {
            TagData::Lut8(l) => {
                assert_eq!(l.input_tables.len(), 256);
                assert_eq!(l.clut, vec![9, 9]);
                assert_eq!(l.output_tables.len(), 256);
            }
            other => panic!("{other:?}"),
        }
    }

    /// mAB with B curves only (identity curv × 3), everything else at
    /// offset 0 = ABSENT (the sentinel is unambiguous: offset 0 would
    /// be the type signature).
    #[test]
    fn lut_ab_b_only_pipeline() {
        let mut t = base(b"mAB ");
        t.extend_from_slice(&[3, 3, 0, 0]); // in, out, pad
        t.extend_from_slice(&32u32.to_be_bytes()); // offsetB
        t.extend_from_slice(&[0u8; 16]); // mat, M, CLUT, A all absent
        // Three identity curv elements, each 12 bytes (already 4-aligned).
        for _ in 0..3 {
            t.extend_from_slice(b"curv");
            t.extend_from_slice(&[0u8; 4]); // reserved
            t.extend_from_slice(&0u32.to_be_bytes()); // count = 0
        }
        let d = decode(&t).unwrap();
        match &d.data {
            TagData::LutAToB(l) => {
                let b = l.b_curves.as_ref().unwrap();
                assert_eq!(b.len(), 3);
                assert!(
                    b.iter()
                        .all(|c| matches!(c, crate::lut::CurveElement::Curve(Curve::Identity)))
                );
                assert!(l.matrix.is_none());
                assert!(l.clut.is_none());
                assert!(l.m_curves.is_none());
                assert!(l.a_curves.is_none());
            }
            other => panic!("{other:?}"),
        }
    }

    /// Full mAB : A(1) + CLUT(2×… per-dim) + M(1) + Matrix(3×4) + B(1),
    /// 1-in/1-out… no — matrix requires 3-channel M-side; keep the
    /// structural decode test at 1-in/1-out WITH a matrix present
    /// anyway: the DECODER represents what the file says (whether a
    /// matrix is *meaningful* for non-3-channel data is A24, a
    /// consumer question, still unsourced).
    #[test]
    fn lut_ab_full_pipeline_with_3x4_matrix_and_per_dim_clut() {
        let mut t = base(b"mAB ");
        t.extend_from_slice(&[1, 1, 0, 0]);
        // Element order in the file is our choice; offsets say where.
        // Layout: B@32(curv,12) Matrix@44(48) M@92(curv,12)
        // CLUT@104(20+2·2=24) A@128(para type0, 16)
        t.extend_from_slice(&32u32.to_be_bytes()); // B
        t.extend_from_slice(&44u32.to_be_bytes()); // Matrix
        t.extend_from_slice(&92u32.to_be_bytes()); // M
        t.extend_from_slice(&104u32.to_be_bytes()); // CLUT
        t.extend_from_slice(&128u32.to_be_bytes()); // A
        // B: identity curv.
        t.extend_from_slice(b"curv");
        t.extend_from_slice(&[0u8; 4]);
        t.extend_from_slice(&0u32.to_be_bytes());
        // Matrix: 12 × s15Fixed16 — e03/e13/e23 are the trap; give
        // them distinct values and assert they arrive.
        for i in 0..12u32 {
            t.extend_from_slice(&(i * 0x0001_0000).to_be_bytes());
        }
        // M: identity curv.
        t.extend_from_slice(b"curv");
        t.extend_from_slice(&[0u8; 4]);
        t.extend_from_slice(&0u32.to_be_bytes());
        // CLUT: gridPoints[0]=2 (rest zero), prec=2, pad, 2 samples.
        let mut grid = [0u8; 16];
        grid[0] = 2;
        t.extend_from_slice(&grid);
        t.extend_from_slice(&[2, 0, 0, 0]); // prec=2 + pad
        t.extend_from_slice(&0x1111u16.to_be_bytes());
        t.extend_from_slice(&0x2222u16.to_be_bytes());
        // A: para funcType 0, one param (gamma 1.0).
        t.extend_from_slice(b"para");
        t.extend_from_slice(&[0u8; 4]);
        t.extend_from_slice(&0u16.to_be_bytes());
        t.extend_from_slice(&[0u8; 2]);
        t.extend_from_slice(&0x0001_0000u32.to_be_bytes());

        let d = decode(&t).unwrap();
        match &d.data {
            TagData::LutAToB(l) => {
                let m = l.matrix.unwrap();
                // The three offset terms (indices 9..12) survived —
                // the 36-byte misread would have lost them.
                assert_eq!(m[9].to_f64(), 9.0);
                assert_eq!(m[11].to_f64(), 11.0);
                let clut = l.clut.as_ref().unwrap();
                assert_eq!(clut.grid_points[0], 2);
                assert_eq!(clut.precision, 2);
                assert_eq!(
                    clut.samples,
                    crate::lut::ClutSamples::U16(vec![0x1111, 0x2222])
                );
                assert!(matches!(
                    l.a_curves.as_ref().unwrap()[0],
                    crate::lut::CurveElement::Parametric(_)
                ));
            }
            other => panic!("{other:?}"),
        }
        assert!(d.issues.is_empty());
    }

    /// A broken element in a curve chain fails positionally — with no
    /// count field, everything after it is unreachable.
    #[test]
    fn lut_ab_broken_curve_chain_reported_by_position() {
        let mut t = base(b"mAB ");
        t.extend_from_slice(&[3, 3, 0, 0]);
        t.extend_from_slice(&32u32.to_be_bytes()); // offsetB
        t.extend_from_slice(&[0u8; 16]);
        // First element valid, second is garbage.
        t.extend_from_slice(b"curv");
        t.extend_from_slice(&[0u8; 4]);
        t.extend_from_slice(&0u32.to_be_bytes());
        t.extend_from_slice(b"XXXX");
        t.extend_from_slice(&[0u8; 8]);
        assert!(matches!(
            decode(&t),
            Err(TagDecodeError::CurveChainBroken {
                element: 1,
                position: 44,
                ..
            })
        ));
    }

    /// CLUT precision outside {1,2}: sample width unknowable, refused.
    #[test]
    fn clut_bad_precision_refused() {
        let mut t = base(b"mBA ");
        t.extend_from_slice(&[1, 1, 0, 0]);
        t.extend_from_slice(&[0u8; 8]); // B, Matrix absent
        t.extend_from_slice(&0u32.to_be_bytes()); // M absent
        t.extend_from_slice(&32u32.to_be_bytes()); // CLUT
        t.extend_from_slice(&0u32.to_be_bytes()); // A absent
        let mut grid = [0u8; 16];
        grid[0] = 2;
        t.extend_from_slice(&grid);
        t.extend_from_slice(&[3, 0, 0, 0]); // prec=3: illegal
        t.extend_from_slice(&[0u8; 4]);
        assert_eq!(
            decode(&t),
            Err(TagDecodeError::ClutBadPrecision { precision: 3 })
        );
    }
}
