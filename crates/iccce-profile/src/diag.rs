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

/// Which edition's rule a [`Malformation::UnknownRenderingIntent`]
/// report is made under.
///
/// This exists because **the two editions license different claims about
/// the same bytes**, and a report that does not say which one it is
/// speaking under is making the stronger claim by default. iccce reports
/// and does not repair (project rule 6) — but a report is itself an
/// assertion, and an over-strong one impugns a conforming file with no
/// layer above the parser to catch it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntentRule {
    /// ICC.1:2022 (v4.x). High 16 bits **shall** be zero (quoted);
    /// low half outside 0..=3 is prohibited **by inference** (`A56`).
    V4Prohibited,
    /// ICC.1:2001-04 (v2.x). Table 18 defines four values and the clause
    /// forbids nothing; the high 16 bits are vendor-available by the
    /// same parallel construction 6.1.8 uses for the profile flags.
    /// **A high-half value is not reported at all under this rule.**
    V2Undefined,
}

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
    /// The `renderingIntent` field (header offset 64..67) carries a
    /// value the applicable edition does not define.
    ///
    /// ## Why this variant carries an edition, and why the two readings
    /// are not the same claim
    ///
    /// **The two editions differ, and reporting them in identical words
    /// was a defect** (fixed 2026-08-18; the v2 half of the report was
    /// false). What each edition actually says:
    ///
    /// * **ICC.1:2022 (v4) 7.2.15** — *"The field is a uInt32Number in
    ///   which the least-significant 16 bits shall be used to encode the
    ///   rendering intent"*, *"the most significant 16 bits shall be set
    ///   to zero"*, and *"These shall be identified using the values
    ///   shown in Table 23"*. The high half is **prohibited** from being
    ///   non-zero — that is quoted, not inferred.
    ///
    /// * **ICC.1:2001-04 (v2) 6.1.11** — the clause body in full is
    ///   *"Perceptual, media-relative colorimetric, saturation and
    ///   ICC-absolute colorimetric are the four intents required to be
    ///   supported. The least-significant 16 bits are reserved for the
    ///   ICC."* plus Table 18's four rows. It contains **no `shall`, no
    ///   "must", no "only"**, and no "other values are reserved"
    ///   sentence — which the same document *does* use elsewhere when it
    ///   means to close a set (6.5.4 / Table 38, `dataType`: *"other
    ///   values are reserved for future use"*). So in v2 a value outside
    ///   0–3 is **undefined, not forbidden**, and v2's *"least-significant
    ///   16 bits are reserved for the ICC"* is the identical boilerplate
    ///   6.1.8 uses for the profile flags, where the high half is
    ///   demonstrably vendor space. **A v2 profile with high bits set is
    ///   using the field as its own edition invites.**
    ///
    /// ★ Consequence for the emitted string, which is the part a
    /// consumer reads: *"outside the defined 0..=3"* is a true statement
    /// about v4 and a **false statement about v2**. The wording is
    /// therefore selected by [`IntentRule`], not shared.
    ///
    /// ## The v4 claim is inferred, and says so
    ///
    /// No sentence in ICC.1:2022 forbids a *low-half* value outside 0–3
    /// in as many words. The prohibition is reached by chaining
    /// *"shall specify the rendering intent"* to *"These shall be
    /// identified using the values shown in Table 23"* — a value naming
    /// none of the four specifies no intent. That is a **two-step
    /// inference and the citation is the chain, not a quotation**
    /// (register entry `A56`).
    ///
    /// ## Unheld edition
    ///
    /// **ICC.1:2010-12 (v4.3) is not held** (same blocker as `A31`,
    /// `A47`, `A51`, `A55`). v4.x is exactly where the high-half `shall`
    /// appeared, so it is the plausible place for a low-half wording
    /// change too, and this corpus cannot exclude one.
    UnknownRenderingIntent { value: u32, rule: IntentRule },
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
            Self::UnknownRenderingIntent { value, rule } => match rule {
                // v4 states the prohibition (high half) and supports the
                // low-half one by inference; "outside the defined range"
                // is defensible. The doc comment carries the chain.
                IntentRule::V4Prohibited => write!(
                    f,
                    "rendering intent 0x{value:08X} is outside the defined 0..=3 \
                     (ICC.1:2022 7.2.15 + Table 23)"
                ),
                // ★ v2 defines four values and forbids nothing. Saying
                // "outside the defined range" here would assert a rule
                // ICC.1:2001-04 does not contain — the report would be
                // as wrong as the file it accuses.
                IntentRule::V2Undefined => write!(
                    f,
                    "unrecognised rendering intent value 0x{value:08X} \
                     (ICC.1:2001-04 6.1.11 / Table 18 define only 0..=3 \
                     and do not forbid others)"
                ),
            },
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
