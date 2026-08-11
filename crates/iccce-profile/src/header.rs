//! # The 128-byte profile header
//!
//! Layout per `ICC_Spec/icc/icc__s__header.md` (evidence tier
//! `cross_verified_2src`: ICC's `icProfileHeader.h` and `lcms2.h` agree
//! field-for-field, and the field sizes sum to exactly 128). Clause
//! numbers are deliberately absent: the corpus does not yet have the
//! ICC.1 PDF (`docs/LEGAL.md` §2), and citing a clause number nobody
//! read would be the paraphrase-as-spec-text failure the project rules
//! prohibit.
//!
//! Fields are read individually, big-endian — never by overlaying a
//! packed struct (`icc__s__header.md` traps: `attributes` is an 8-byte
//! value at the non-8-aligned offset 56).

use crate::diag::Malformation;
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
    /// Offset 64: rendering intent, all 32 bits as read (A7).
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

        // Report — never zero out — violations the header carries.
        if header.reserved.iter().any(|&b| b != 0) {
            malformations.push(Malformation::HeaderReservedNonZero);
        }
        if header.rendering_intent > 3 {
            malformations.push(Malformation::UnknownRenderingIntent {
                value: header.rendering_intent,
            });
        }
        header
    }
}
