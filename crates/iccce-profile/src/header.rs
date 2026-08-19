//! # The 128-byte profile header
//!
//! Layout per `ICC_Spec/icc/icc__s__header.md` (evidence tier
//! `cross_verified_2src`: ICC's `icProfileHeader.h` and `lcms2.h` agree
//! field-for-field, and the field sizes sum to exactly 128).
//!
//! ## ★ CORRECTED 2026-08-18 — clause numbers are no longer absent
//!
//! This header previously read *"Clause numbers are deliberately absent:
//! the corpus does not yet have the ICC.1 PDF"*. **That ceased to be
//! true and the note outlived it.** `ICC.1-2022-05.pdf` (v4.4.0.0) and
//! `ICC.1-2001-04.pdf` (v2.4) are both held in `ICC_Spec/_sources/`, and
//! the `renderingIntent` validation below now cites 7.2.15, 6.1.11 and
//! Table 18 from text read directly and cross-verified in three
//! independent extraction channels.
//!
//! **The retracted principle was right and is retained:** a clause
//! number nobody read must never appear here. What changed is only that
//! some clauses have now been read. Fields whose notes still say
//! `NOT SOURCED` mean exactly that — the gap is per-field, and the
//! absence of a citation on a field is information, not an oversight.
//!
//! Fields are read individually, big-endian — never by overlaying a
//! packed struct (`icc__s__header.md` traps: `attributes` is an 8-byte
//! value at the non-8-aligned offset 56).

use crate::diag::{IntentRule, Malformation};
use crate::num::{DateTimeNumber, Signature, XyzNumber, u32_be, u64_be};

/// The profile version, kept raw alongside its BCD decoding.
///
/// Encoding per `icc__s__header.md` offset 8: `04 30 00 00` = v4.3 —
/// byte 0 is the major version, byte 1 packs minor (high nibble) and
/// bug-fix (low nibble) as BCD.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProfileVersion {
    /// The four bytes as read; retained because the raw value is what
    /// diagnostics and the v2/v4 selector (divergence D7) key on.
    pub raw: u32,
}

impl ProfileVersion {
    pub fn major(self) -> u8 {
        #[allow(clippy::cast_possible_truncation)] // top byte extraction
        let b = (self.raw >> 24) as u8;
        b
    }
    pub fn minor(self) -> u8 {
        #[allow(clippy::cast_possible_truncation)]
        let b = ((self.raw >> 16) as u8) >> 4;
        b
    }
    pub fn bugfix(self) -> u8 {
        #[allow(clippy::cast_possible_truncation)]
        let b = ((self.raw >> 16) as u8) & 0x0F;
        b
    }
}

impl std::fmt::Display for ProfileVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major(), self.minor(), self.bugfix())
    }
}

/// The header, represented as the file states it — including fields
/// whose values are wrong. Judgement lives in diagnostics, not here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    /// Offset 0: declared total profile size, including this header.
    pub size: u32,
    /// Offset 4: preferred CMM. Informational — per `icc__s__header.md`
    /// **never dispatch on it**; `0` is legal and common.
    pub cmm_id: Signature,
    /// Offset 8: version, BCD.
    pub version: ProfileVersion,
    /// Offset 12: device class (`'mntr'`, `'prtr'`, `'link'`, …).
    pub device_class: Signature,
    /// Offset 16: data (device-side) colour space.
    pub color_space: Signature,
    /// Offset 20: PCS — `'XYZ '` or `'Lab '` (for `'link'` class it
    /// carries the output space instead; NOT SOURCED, ambiguity A6).
    pub pcs: Signature,
    /// Offset 24: creation date/time, UTC. All-zero = unspecified.
    pub date: DateTimeNumber,
    /// Offset 36: `'acsp'`. Parse fails earlier if it is not; stored
    /// anyway so the representation is complete.
    pub magic: Signature,
    /// Offset 40: primary platform; `0` = unspecified.
    pub platform: Signature,
    /// Offset 44: profile flags. Bits 0–15 ICC-defined, 16–31 vendor.
    pub flags: u32,
    /// Offset 48: device manufacturer; `0` legal.
    pub manufacturer: Signature,
    /// Offset 52: device model; `0` legal.
    pub model: u32,
    /// Offset 56: media attributes. Held opaque: exact bit assignments
    /// are NOT SOURCED (ambiguity A8) — parsing named booleans out of
    /// unverified bit positions would be guessing with confidence.
    pub attributes: u64,
    /// Offset 64: rendering intent, **all 32 bits exactly as read** —
    /// never masked, in either edition.
    ///
    /// ★ The note here formerly read *"(A7)"*, deferring to an ambiguity
    /// **resolved from the primary text on 2026-08-11**. Corrected
    /// 2026-08-18. What is now sourced: ICC.1:2022 7.2.15 puts the
    /// intent in the least-significant 16 bits and requires the most
    /// significant 16 to be zero; ICC.1:2001-04 6.1.11 states the field
    /// type nowhere at all (Table 9's cell reads *"see below"*), so in
    /// v2 the low-half reading is an inference resting on *"the
    /// least-significant 16 bits are reserved for the ICC"*.
    ///
    /// Storing the raw 32 bits is what lets the two editions be reported
    /// differently without the parser having to choose one to believe.
    pub rendering_intent: u32,
    /// Offset 68: the PCS illuminant — architecturally always D50; not
    /// the measurement illuminant (`icc__s__header.md` field notes).
    pub illuminant: XyzNumber,
    /// Offset 80: profile creator; `0` legal.
    pub creator: Signature,
    /// Offset 84: MD5 of the profile (v4). All-zero = not computed,
    /// which is NOT an error (divergence D4).
    pub profile_id: [u8; 16],
    /// Offsets 100–127: reserved, shall be zero in ICC.1. Retained
    /// verbatim so a non-zero region is reportable and inspectable.
    pub reserved: [u8; 28],
}

impl Header {
    /// Parse the 128 header bytes. The caller (`Profile::parse`) has
    /// already guaranteed ≥ 132 bytes and checked magic and version,
    /// so every `read` here is in range; the `expect`s document that
    /// contract rather than handle a reachable case.
    pub(crate) fn parse(bytes: &[u8], malformations: &mut Vec<Malformation>) -> Header {
        let g = "caller guarantees >= 132 bytes";
        let header = Header {
            size: u32_be(bytes, 0).expect(g),
            cmm_id: Signature::read(bytes, 4).expect(g),
            version: ProfileVersion {
                raw: u32_be(bytes, 8).expect(g),
            },
            device_class: Signature::read(bytes, 12).expect(g),
            color_space: Signature::read(bytes, 16).expect(g),
            pcs: Signature::read(bytes, 20).expect(g),
            date: DateTimeNumber::read(bytes, 24).expect(g),
            magic: Signature::read(bytes, 36).expect(g),
            platform: Signature::read(bytes, 40).expect(g),
            flags: u32_be(bytes, 44).expect(g),
            manufacturer: Signature::read(bytes, 48).expect(g),
            model: u32_be(bytes, 52).expect(g),
            attributes: u64_be(bytes, 56).expect(g),
            rendering_intent: u32_be(bytes, 64).expect(g),
            illuminant: XyzNumber::read(bytes, 68).expect(g),
            creator: Signature::read(bytes, 80).expect(g),
            profile_id: bytes[84..100].try_into().expect(g),
            reserved: bytes[100..128].try_into().expect(g),
        };

        // Report — never zero out — what the header carries.
        //
        // ★★ EDITION-GATED RANGE, and this is the THIRD instance of the
        // same mechanism (after the rendering-intent report fixed
        // 2026-08-18 and the `Malformation` doc comment fixed
        // 2026-08-19): a v4-only concept applied to a v2 profile.
        //
        // The reserved block is 44 bytes in BOTH editions, but it does
        // not START in the same place, because `profileID` is a v4
        // addition:
        //
        //   v4 (ICC.1:2022 7.2.18/7.2.19) — 84..99 profileID,
        //                                   100..127 reserved
        //   v2 (ICC.1:2001-04 Table 9)    — 84..127 ALL reserved,
        //                                   "44 bytes reserved for
        //                                   future expansion"
        //
        // ★ Checking only 100..128 on a v2 profile therefore MISSES 16
        // bytes of that edition's reserved block entirely — and iccce
        // was simultaneously presenting those same 16 bytes as a
        // `profileID`, a field v2 does not have. Measured before the
        // fix: a v2 profile with 0xDEADBEEF... at 84..100 printed
        // `header.id: deadbeef...` and `malformations: 0`. That is
        // worse than a false accusation; it is a FABRICATED VALUE, and
        // a consumer would reasonably believe the profile carried an
        // MD5 profile ID. It carries no such field.
        //
        // ★ Modality differs too, and is NOT symmetric — see
        // `Malformation::HeaderReservedNonZero` and
        // `ViolationStatus`: v4 says these bytes "shall be set to
        // zero" (a requirement a file can breach); v2's Table 9 cell is
        // the unmodalised "44 bytes reserved for future expansion" and
        // is the only mention in that document. So on v2 this is a
        // DISCLOSURE and not a violation. It is still reported, because
        // the parser reports (rule 6) — what changes is the claim, not
        // the visibility.
        let (first_byte, reserved_dirty) = if header.version.major() >= 4 {
            (100, header.reserved.iter().any(|&b| b != 0))
        } else {
            (
                84,
                header
                    .profile_id
                    .iter()
                    .chain(header.reserved.iter())
                    .any(|&b| b != 0),
            )
        };
        if reserved_dirty {
            malformations.push(Malformation::HeaderReservedNonZero { first_byte });
        }
        // ★ VERSION-GATED, and the gate is two conditions rather than
        // one. Before 2026-08-18 this read `if rendering_intent > 3`
        // unconditionally, which reported a v2 profile in the same words
        // as a v4 one — for a requirement v2 does not impose on either
        // half of the field. See `Malformation::UnknownRenderingIntent`
        // for the clause-by-clause sourcing; the short form is:
        //
        //   v4 (ICC.1:2022 7.2.15) — high 16 bits SHALL be zero
        //     (quoted); the whole field must therefore be 0..=3.
        //   v2 (ICC.1:2001-04 6.1.11 / Table 18) — the high half is
        //     vendor-available by the same construction 6.1.8 uses for
        //     the flags field, and the low half's four defined values
        //     are not a closed set. Only an unrecognised LOW half is
        //     reportable, and only as "unrecognised".
        //
        // ★ Note what this deliberately does NOT do: it does not mask,
        // normalise, or otherwise repair the stored value. `Header`
        // keeps all 32 bits exactly as read in every case (rule 6). The
        // edition changes what iccce is entitled to SAY, never what the
        // file is recorded as containing.
        //
        // v2 does not state the field's type at all — Table 9's cell
        // reads "see below" and 6.1.11 names none — so reading the low
        // half as the intent is itself an inference in v2, resting on
        // "the least-significant 16 bits are reserved for the ICC".
        // Recorded here because it is the assumption a future reader is
        // most likely to mistake for quoted text.
        if header.version.major() >= 4 {
            if header.rendering_intent > 3 {
                malformations.push(Malformation::UnknownRenderingIntent {
                    value: header.rendering_intent,
                    rule: IntentRule::V4Prohibited,
                });
            }
        } else if header.rendering_intent & 0xFFFF > 3 {
            malformations.push(Malformation::UnknownRenderingIntent {
                value: header.rendering_intent,
                rule: IntentRule::V2Undefined,
            });
        }
        header
    }
}
