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
//! - [`Malformation`] — something about the file worth telling the caller,
//!   where a faithful representation is still constructible. These
//!   accumulate on the parsed [`Profile`](crate::Profile); nothing is
//!   corrected, and the caller (e.g. `iccce-cli inspect`) is the
//!   disclosure surface.
//!
//!   ★ **Not every one of these is a rule violation**, and this line said
//!   otherwise until 2026-08-19. Two variants report on files that breach
//!   nothing — `TrailingBytes` (normal for a container-embedded profile)
//!   and `UnknownRenderingIntent` under [`IntentRule::V2Undefined`]
//!   (ICC.1:2001-04 defines four values and forbids no others). The
//!   consequence is that **`malformations: N` is not a conformance
//!   verdict**; see [`Malformation`]'s own doc comment for what a caller
//!   may and may not conclude from the count, and `docs/ARCHITECTURE.md`
//!   DL-063 for why the mixed channel is kept rather than split.
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

/// Whether a reported [`Malformation`] is a breach of ICC.1, is not, or
/// has never been established.
///
/// ## ★★ Why this is THREE states and not a `bool`
///
/// `pdfce` asked for a `Malformation::is_violation()` predicate
/// (2026-08-19) so that a report can say *"3 observations, 1 of which
/// breaches ICC.1"* while still showing all three — their rule forbids
/// hiding diagnostics, so a filtered accessor was explicitly declined.
///
/// **A `bool` turned out to be unimplementable honestly.** Two of the
/// nine variants have **no requirement behind them in either edition**
/// ([`Malformation::TagTooSmall`], `ICC_Spec` **`A61`**), and one has
/// none in v2 ([`Malformation::TagOverrun`], **`A62`**). A boolean would
/// have forced an invented answer for those, and — this is the part that
/// matters — **the invention would have looked exactly as authoritative
/// as the seven that are sourced.**
///
/// ★ [`ViolationStatus::Unsourced`] means **iccce has not established
/// the modality**. It does **not** mean the file is fine, and it must
/// never be rendered as though it did. It is a statement about this
/// project's knowledge, not about the profile.
///
/// ## ★ Why the edition must be supplied
///
/// [`Malformation::violation_status`] takes the profile's version
/// because **the same condition is a breach in one edition and not in
/// the other**, for three of the nine variants. That is not a quirk of
/// two badly-drafted clauses; it is what happens when a field is *added*
/// between editions. Asking "is this a violation?" of a malformation
/// alone is not a well-formed question.
///
/// ★★ **A `shall`-grep gets v2 backwards.** ICC.1:2001-04 requires with
/// **"must"** (76 occurrences) rather than `shall` (27, three of them in
/// the copyright notice and none on a header, tag-table or tag-type
/// rule); its own change list concedes it *"does not meet all of the
/// ISO/IEC drafting rules"*. Symmetrically, v2's **unmodalised**
/// sentences really are silent, because the drafters used "must" in the
/// adjacent sentence. Anyone re-deriving this table must know that
/// first.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViolationStatus {
    /// The file breaches a stated requirement of the applicable edition.
    Violation,
    /// The condition is reported, but the file breaches nothing. A
    /// conforming profile can carry this.
    NotAViolation,
    /// ★ **iccce has not established the modality.** Not a verdict of
    /// innocence — an admission of ignorance, and it must be rendered as
    /// one. Carries the `ICC_Spec` ambiguity-register id so a caller can
    /// find out what is actually unknown.
    Unsourced {
        /// The `ICC_Spec` ambiguity-register row, e.g. `"A61"`.
        register_id: &'static str,
    },
}

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

/// A **disclosure** about the file that the parser makes and never acts
/// on: something worth telling a caller, where the representation
/// nonetheless survives and parsing continues. Reported verbatim;
/// **never corrected** (project rule 6).
///
/// ## ★ This is a channel for disclosures, NOT only for violations
///
/// The obvious reading of the name — and this type's own doc comment
/// until 2026-08-19, which said *"a rule violation the file carries"* —
/// is that every variant reports a **breach of ICC.1**. **That reading is
/// false, and it is falsified by two of the variants below.** It is
/// corrected here rather than left implicit because the mis-reading is
/// not harmless: `iccce inspect` prints `malformations: N`, and a
/// consumer that treats a non-zero `N` as *"this file violates the
/// specification"* — a **conformance verdict** — will condemn conforming
/// files.
///
/// The two variants that carry no violation at all:
///
/// | variant | what it actually reports | why it is not a violation |
/// |---|---|---|
/// | [`Malformation::TrailingBytes`] | bytes past the header's declared size | **Normal** for a profile embedded in a container (a PDF stream, an ICC-tagged image). The emitted text says so. Nothing in ICC.1 is breached by a caller handing us a longer buffer than the profile claims. |
/// | [`Malformation::UnknownRenderingIntent`] with [`IntentRule::V2Undefined`] | a `renderingIntent` value ICC.1:2001-04 does not define | 2001-04 Table 18 defines four values and **forbids no others**. The report says *"unrecognised"*, deliberately, and *"do not forbid others"* in the same sentence. The file may be entirely conforming. |
///
/// Both were already careful in their **wording** — the sentences a user
/// reads do not accuse anyone. What was not careful is the **channel**:
/// they arrive through a type named `Malformation` and are added into a
/// machine-readable count. Words are read by humans; the count is read by
/// code.
///
/// ## What a caller may and may not conclude from the count
///
/// * **`N == 0`** — iccce found nothing to say. This is **not** a
///   certificate of conformance: iccce checks the constraints it has
///   implemented, not every clause of ICC.1.
/// * **`N > 0`** — there is at least one thing worth reading. **It does
///   not follow that the file is non-conforming.** To decide that, a
///   caller must look at *which* variants, not how many.
///
/// A caller wanting a conformance verdict must therefore match on the
/// variants; the count is a prompt to look, not an answer. This is a
/// deliberate design choice and not an accident of naming — see
/// `docs/ARCHITECTURE.md`'s decision log. The alternative, splitting the
/// type into `Violation` and `Observation`, was considered and is
/// recorded there with the reason it was not taken.
///
/// ## Why the name is kept anyway
///
/// Renaming the type is a public API break with no numeric benefit, and
/// it would move the ambiguity rather than remove it: an
/// `Observation::TrailingBytes` next to an `Observation::ReservedNonZero`
/// under-states the second exactly as much as the present name
/// over-states the first. The mixed channel is real, so it is
/// **documented** here and at the print site rather than papered over
/// with a rename that would still need this table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Malformation {
    /// Header bytes 100–127 shall be zero in ICC.1 (`icc__s__header.md`
    /// — for v2/v4 these are reserved; only iccMAX reclaims them).
    HeaderReservedNonZero { first_byte: usize },
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

impl Malformation {
    /// Whether this report is a breach of ICC.1 **under the given
    /// edition**, is not, or has never been established.
    ///
    /// ## The table, and every verdict's source
    ///
    /// Sourced 2026-08-19 from both editions' primary text, quotations in
    /// `ICC_Spec/icc/icc__s__tag_table.md` §7. ★ The verdicts are **not**
    /// derived from `icc__s__tag_table.md`'s validation table — that was
    /// written as *"checks iccce should perform and REPORT"* and **no row
    /// of it was ever a quotation of a requirement**. It is a derived
    /// convenience and is now bannered as one.
    ///
    /// | variant | v4 (ICC.1:2022) | v2 (ICC.1:2001-04) |
    /// |---|---|---|
    /// | `HeaderReservedNonZero` | **Violation** (7.2.19 *"shall be set to zero"*) | **NotAViolation** — Table 9's cell is the unmodalised *"44 bytes reserved for future expansion"*, and it is the **only** mention in the document |
    /// | `TrailingBytes` | NotAViolation | NotAViolation — 7.2.2's `shall` sizes the **profile**, not the file, and both editions contemplate embedding in the same sentence |
    /// | `UnknownRenderingIntent` | **Violation** via `IntentRule` | NotAViolation via `IntentRule` |
    /// | `TagOverrun` | **Violation** (7.2.2 *"shall be the exact size"*) | **Unsourced `A62`** — v2 6.1.1 is one unmodalised sentence and v2 has no contiguity or padding clause |
    /// | `TagOverlapsTable` | **Violation** (7.1.2 b) *"shall immediately follow"*) | **Violation** — clause 6's ordering `shall` |
    /// | `TagMisaligned` | **Violation** (7.3.4) | **Violation** (6.2.2 *"is required to"* / *"must be zero"*) |
    /// | `TagTooSmall` | **Unsourced `A61`** | **Unsourced `A61`** |
    /// | `TagBaseReservedNonZero` | **Violation** (10.1 *"shall be set to 0"*) | **Violation** (6.5, *"must"*) |
    /// | `DuplicateTagSignature` | **Violation** (7.3.1 *"Duplicate tag signatures shall not be included"*) | **Violation** (6.2 *"must be unique"*) |
    ///
    /// ## ★★ Two entries that were WRONG in this project until today
    ///
    /// 1. **`DuplicateTagSignature` was labelled *"Legality NOT
    ///    SOURCED"*** and graded SILENT in the ambiguity register.
    ///    **Both editions prohibit duplicates outright**, v2 twice — the
    ///    rule plus a change-list item, *"A tag can now only appear once
    ///    in a profile. Per: Resolution voted 1998-03-15"*. The stale
    ///    label survived because a batch resolution leaves a batch of
    ///    stale prose. ★ The *decision* built on it (keep both, first
    ///    wins) is unaffected — **which** duplicate wins is still
    ///    genuinely unsourced — but its stated rationale was false.
    /// 2. **`TagTooSmall` is not sourced at all.** Nothing states a
    ///    minimum tag size, and each edition has a sentence pointing the
    ///    other way — v4 7.4 *"shall only be restricted by the limits
    ///    imposed by the 32-bit … values"*, v2 6.2.3 ***"An element may
    ///    have any size"***. *"A byte that does not exist has not been
    ///    set to 0"* is an **inference, not a quotation**.
    ///
    /// ## ★ Where iccce is deliberately WEAKER than the specification
    ///
    /// `TagOverlapsTable` fires only when tag data begins *inside* the
    /// tag table. v4 7.1.2 b) also forbids a **gap** between the table
    /// and the first tag. iccce does not detect the gap, so a `Violation`
    /// here is sound but the **absence** of one is not a clean bill.
    #[must_use]
    pub fn violation_status(&self, version: crate::header::ProfileVersion) -> ViolationStatus {
        let v4 = version.major() >= 4;
        match self {
            // v4 7.2.19 states a `shall`; v2 Table 9 states nothing.
            Self::HeaderReservedNonZero { .. } => {
                if v4 {
                    ViolationStatus::Violation
                } else {
                    ViolationStatus::NotAViolation
                }
            }
            // Both editions: the size field sizes the PROFILE, and both
            // contemplate embedded profiles explicitly.
            Self::TrailingBytes { .. } => ViolationStatus::NotAViolation,
            // The edition is already carried, so it is used rather than
            // re-derived from `version` — if the two ever disagreed, the
            // rule the REPORT was made under is the honest one.
            Self::UnknownRenderingIntent { rule, .. } => match rule {
                IntentRule::V4Prohibited => ViolationStatus::Violation,
                IntentRule::V2Undefined => ViolationStatus::NotAViolation,
            },
            Self::TagOverrun { .. } => {
                if v4 {
                    ViolationStatus::Violation
                } else {
                    ViolationStatus::Unsourced { register_id: "A62" }
                }
            }
            Self::TagOverlapsTable { .. } | Self::TagMisaligned { .. } => {
                ViolationStatus::Violation
            }
            Self::TagTooSmall { .. } => ViolationStatus::Unsourced { register_id: "A61" },
            Self::TagBaseReservedNonZero { .. } | Self::DuplicateTagSignature { .. } => {
                ViolationStatus::Violation
            }
        }
    }
}

impl std::fmt::Display for Malformation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HeaderReservedNonZero { first_byte } => {
                // The range is carried rather than fixed because it
                // DIFFERS BY EDITION: v4 reserves 100..128 (84..100 is
                // `profileID`), v2 reserves 84..128 and has no
                // `profileID` at all. Printing a fixed "100..128" on a
                // v2 profile understated the block by 16 bytes.
                write!(
                    f,
                    "header reserved bytes {first_byte}..128 are not all zero"
                )
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
