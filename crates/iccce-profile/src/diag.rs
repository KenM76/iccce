//! # Diagnostics — how the parser reports without repairing
//!
//! The crate's core invariant (`docs/ARCHITECTURE.md` §3.2): **the
//! parser reports; it does not repair.** This module is the reporting
//! half of that sentence.
//!
//! Two severities, and the distinction is load-bearing:
//!
//! - [`ParseError`] — the parse cannot proceed *at all*: the bytes are
//!   not an ICC profile, are truncated below the declared size, or are
//!   an iccMAX (ICC.2) profile, which this engine identifies and
//!   refuses **by name** rather than executing or mistaking for
//!   corruption (`README.md` scope).
//! - [`Malformation`] — the file violates a rule but a faithful
//!   representation is still constructible. These accumulate on the
//!   parsed [`Profile`](crate::Profile); nothing is corrected, and the
//!   caller (e.g. `iccce-cli inspect`) is the disclosure surface.
//!
//! WHY not one merged error list: a caller that gets a `Profile` back
//! must be able to trust that every field means what the *file* said,
//! and a caller that gets a `ParseError` must know there is no partial
//! result to be tempted by.

use crate::num::Signature;

/// The parse could not produce a faithful representation. Terminal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// Fewer than 132 bytes: cannot even hold the 128-byte header plus
    /// the tag count (`icc__s__header.md`: "there is no short header").
    TooShort { actual: usize },
    /// Offset 36 is not `'acsp'` — per `icc__s__header.md` the only
    /// reliable format-identity check; without it, these bytes are not
    /// claimed to be an ICC profile at all.
    BadMagic { found: Signature },
    /// `header.size` exceeds the bytes provided: the profile is
    /// truncated. Refused per `icc__s__header.md` field notes (the
    /// opposite case, trailing bytes, is a reported malformation —
    /// containers legally pad embedded profiles).
    Truncated { declared: u32, actual: usize },
    /// Major version ≥ 5: iccMAX (ICC.2). Identified and refused by
    /// name — parsing it with v4 semantics would misread the header's
    /// last 28 bytes, which iccMAX reclaims from the v4 reserved region
    /// (`icc__s__header.md`, bytes 100–127 finding).
    IccMaxRefused { version_raw: u32 },
    /// `132 + 12 × tagCount` exceeds the file length. Checked BEFORE
    /// allocating: `tagCount` is attacker-controlled and multiplies by
    /// 12 (`icc__s__tag_table.md` — same class as a PDF xref-count
    /// overflow).
    TagCountOverflowsFile { tag_count: u32, actual: usize },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TooShort { actual } => write!(
                f,
                "not an ICC profile: {actual} bytes, minimum is 132 (128-byte header + tag count)"
            ),
            Self::BadMagic { found } => write!(
                f,
                "not an ICC profile: magic at offset 36 is {found}, expected 'acsp'"
            ),
            Self::Truncated { declared, actual } => write!(
                f,
                "truncated: header declares {declared} bytes, only {actual} present"
            ),
            Self::IccMaxRefused { version_raw } => write!(
                f,
                "iccMAX (ICC.2) profile refused: version 0x{version_raw:08X} is major version {} \
                 — iccce parses ICC v2/v4 only and does not execute iccMAX by design",
                version_raw >> 24
            ),
            Self::TagCountOverflowsFile { tag_count, actual } => write!(
                f,
                "tag count {tag_count} requires {} bytes of directory, file has {actual}",
                132u64 + 12 * u64::from(*tag_count)
            ),
        }
    }
}

impl std::error::Error for ParseError {}

/// A rule violation the file carries but the representation survives.
/// Reported verbatim; **never corrected**.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Malformation {
    /// Header bytes 100–127 shall be zero in ICC.1 (`icc__s__header.md`
    /// — for v2/v4 these are reserved; only iccMAX reclaims them).
    HeaderReservedNonZero,
    /// `header.size` < actual byte count. Normal for profiles embedded
    /// in PDF/TIFF (containers pad to 4-byte boundaries), so this is a
    /// report, not an error (`icc__s__header.md` field notes).
    TrailingBytes { declared: u32, actual: usize },
    /// Rendering intent (offset 64) outside 0–3
    /// (`icc__s__header.md`: 0=perceptual, 1=media-relative,
    /// 2=saturation, 3=ICC-absolute). Read as all 32 bits and reported
    /// rather than masked, per ambiguity A7 (whether only the low 16
    /// bits carry the intent is NOT SOURCED).
    UnknownRenderingIntent { value: u32 },
    /// Tag data extends past `header.size` (`icc__s__tag_table.md`
    /// validation table: overrun).
    TagOverrun { index: usize, sig: Signature },
    /// Tag data begins inside the tag table
    /// (`icc__s__tag_table.md`: `offset >= 132 + 12·tagCount`).
    TagOverlapsTable { index: usize, sig: Signature },
    /// Tag offset not 4-byte aligned (`icc__s__tag_table.md`).
    TagMisaligned { index: usize, sig: Signature },
    /// Tag smaller than the 8-byte `icTagBase` it must begin with
    /// (`icc__s__tag_table.md`).
    TagTooSmall { index: usize, sig: Signature },
    /// The 4 reserved bytes after a tag's type signature are non-zero
    /// (`icc__s__tag_table.md`, `icTagBase.reserved` shall be 0).
    TagBaseReservedNonZero { index: usize, sig: Signature },
    /// Same tag signature appears more than once in the directory.
    /// Legality NOT SOURCED; iccce's recorded choice (ambiguity A13) is
    /// that *consumers* take the first — the table itself keeps both.
    DuplicateTagSignature {
        first_index: usize,
        dup_index: usize,
        sig: Signature,
    },
}

impl std::fmt::Display for Malformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HeaderReservedNonZero => {
                write!(f, "header reserved bytes 100..128 are not all zero")
            }
            Self::TrailingBytes { declared, actual } => write!(
                f,
                "{} trailing byte(s): header declares {declared}, data has {actual} \
                 (normal for container-embedded profiles)",
                actual - *declared as usize
            ),
            Self::UnknownRenderingIntent { value } => {
                write!(
                    f,
                    "rendering intent 0x{value:08X} is outside the defined 0..=3"
                )
            }
            Self::TagOverrun { index, sig } => {
                write!(
                    f,
                    "tag[{index}] {sig}: data extends past declared profile size"
                )
            }
            Self::TagOverlapsTable { index, sig } => {
                write!(
                    f,
                    "tag[{index}] {sig}: data offset lies inside the tag table"
                )
            }
            Self::TagMisaligned { index, sig } => {
                write!(f, "tag[{index}] {sig}: offset not 4-byte aligned")
            }
            Self::TagTooSmall { index, sig } => {
                write!(
                    f,
                    "tag[{index}] {sig}: size < 8, too small for a type signature"
                )
            }
            Self::TagBaseReservedNonZero { index, sig } => {
                write!(
                    f,
                    "tag[{index}] {sig}: reserved bytes after type signature non-zero"
                )
            }
            Self::DuplicateTagSignature {
                first_index,
                dup_index,
                sig,
            } => write!(
                f,
                "tag[{dup_index}] {sig}: duplicate of tag[{first_index}] \
                 (consumers take the first; recorded choice A13)"
            ),
        }
    }
}
