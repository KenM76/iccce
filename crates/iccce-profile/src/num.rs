//! # Numeric primitives of the ICC byte format
//!
//! Every multi-byte quantity in an ICC profile is **big-endian, without
//! exception and without per-tag byte-order flags**
//! (`ICC_Spec/icc/icc__s__number_encodings.md`, evidence tier
//! `cross_verified_2src`). All readers here take `(bytes, offset)` and
//! return `None` on out-of-range rather than panicking, because the
//! input is untrusted (an embedded stream from a hostile PDF is a
//! first-class caller).
//!
//! ## Citation discipline
//!
//! Encodings in this module are sourced from the `ICC_Spec` corpus, which
//! is currently built by cross-verifying ICC's own `icProfileHeader.h`
//! (BSD-3 DemoIccMAX) against `lcms2.h` (MIT) — **not** from the ICC.1
//! PDF, which is not yet legally retrieved (see `docs/LEGAL.md` §2).
//! Where the corpus marks a detail NOT SOURCED, that flag is repeated
//! here on the item.

/// Read a big-endian `u16` at `off`.
pub fn u16_be(bytes: &[u8], off: usize) -> Option<u16> {
    let b = bytes.get(off..off + 2)?;
    Some(u16::from_be_bytes([b[0], b[1]]))
}

/// Read a big-endian `u32` at `off`.
pub fn u32_be(bytes: &[u8], off: usize) -> Option<u32> {
    let b = bytes.get(off..off + 4)?;
    Some(u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
}

/// Read a big-endian `u64` at `off`.
///
/// ICC.1 defines `uInt64Number` as *two* big-endian `uInt32`s with
/// element `[0]` the high word (`icc__s__number_encodings.md`), which is
/// byte-identical to one big-endian `u64` — so a single 8-byte read is
/// faithful. Only `header.attributes` uses this type in ICC.1.
pub fn u64_be(bytes: &[u8], off: usize) -> Option<u64> {
    let b = bytes.get(off..off + 8)?;
    Some(u64::from_be_bytes([
        b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7],
    ]))
}

/// `s15Fixed16Number`: signed 32-bit, 16 fractional bits, scale 65536,
/// two's complement (`icc__s__number_encodings.md`, cross-verified).
///
/// Stored raw. Conversion to `f64` is `raw as f64 / 65536.0` — exact,
/// since every `i32` is representable in an `f64`.
///
/// WHY raw is kept: the corpus flags sign-extension failure as "the
/// single most common ICC primitive bug" (reading as `u32` then
/// dividing turns legitimately negative matrix entries into huge
/// positive ones). Holding the value as `i32` from the moment of the
/// read makes the mistake unrepresentable in this codebase.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S15Fixed16(pub i32);

impl S15Fixed16 {
    /// Read at `off`. The `as i32` cast is the sign-extension step.
    pub fn read(bytes: &[u8], off: usize) -> Option<Self> {
        #[allow(clippy::cast_possible_wrap)] // the wrap IS the decode: two's complement
        u32_be(bytes, off).map(|raw| Self(raw as i32))
    }

    /// Exact conversion to `f64`.
    pub fn to_f64(self) -> f64 {
        f64::from(self.0) / 65536.0
    }
}

/// `u8Fixed8Number`: unsigned 16-bit, 8 integer + 8 fraction bits,
/// scale 256, range 0.0…255.996 (`icc__s__number_encodings.md`,
/// cross-verified).
///
/// Sole ICC.1 use: `curveType`'s `count == 1` gamma shorthand — where
/// misreading it as a table sample "builds a curve that crushes
/// everything to black" (`icc__type__curve_parametric.md`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct U8Fixed8(pub u16);

impl U8Fixed8 {
    pub fn read(bytes: &[u8], off: usize) -> Option<Self> {
        u16_be(bytes, off).map(Self)
    }

    /// Exact conversion: `raw / 256.0`.
    pub fn to_f64(self) -> f64 {
        f64::from(self.0) / 256.0
    }
}

/// A 4-byte signature: `uInt32`, conventionally four printable ASCII
/// characters, MSB first (`icc__s__number_encodings.md`).
///
/// **Compared as `u32`, never as a string** — trailing-space signatures
/// (`'XYZ '`, `'Lab '`, `'mAB '`…) are common and whitespace-trimming
/// string comparison silently conflates them; some signatures are not
/// valid UTF-8 at all. `Display` is for humans; `==` is for logic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Signature(pub u32);

impl Signature {
    pub fn read(bytes: &[u8], off: usize) -> Option<Self> {
        u32_be(bytes, off).map(Self)
    }

    /// The profile-format magic `'acsp'` = `0x61637370`
    /// (`icc__s__header.md`, offset 36) — per the corpus, the ONLY
    /// reliable format-identity check in the header.
    pub const ACSP: Signature = Signature(0x6163_7370);
}

impl std::fmt::Display for Signature {
    /// Render as `'abcd'` when all four bytes are printable ASCII,
    /// otherwise as `0x????????` — never a lossy mix, so the output is
    /// unambiguous to a diffing script.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let b = self.0.to_be_bytes();
        if b.iter().all(|&c| (0x20..=0x7e).contains(&c)) {
            write!(
                f,
                "'{}{}{}{}'",
                b[0] as char, b[1] as char, b[2] as char, b[3] as char
            )
        } else {
            write!(f, "0x{:08X}", self.0)
        }
    }
}

/// `dateTimeNumber`: six big-endian `uInt16`s — year, month, day,
/// hours, minutes, seconds — in **UTC**; `year` is the full four-digit
/// year (`icc__s__number_encodings.md`, verbatim struct).
///
/// All-zero is common and is **not an error** (means "unspecified").
/// Out-of-range values (month 13) are NOT SOURCED as must-reject; per
/// the report-don't-repair rule they are represented as read.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateTimeNumber {
    pub year: u16,
    pub month: u16,
    pub day: u16,
    pub hours: u16,
    pub minutes: u16,
    pub seconds: u16,
}

impl DateTimeNumber {
    pub fn read(bytes: &[u8], off: usize) -> Option<Self> {
        Some(Self {
            year: u16_be(bytes, off)?,
            month: u16_be(bytes, off + 2)?,
            day: u16_be(bytes, off + 4)?,
            hours: u16_be(bytes, off + 6)?,
            minutes: u16_be(bytes, off + 8)?,
            seconds: u16_be(bytes, off + 10)?,
        })
    }

    pub fn is_unspecified(&self) -> bool {
        *self
            == Self {
                year: 0,
                month: 0,
                day: 0,
                hours: 0,
                minutes: 0,
                seconds: 0,
            }
    }
}

/// `XYZNumber`: three consecutive `s15Fixed16Number`s, 12 bytes
/// (`icc__s__number_encodings.md`, verbatim struct, cross-verified
/// against lcms2's `cmsEncodedXYZNumber`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XyzNumber {
    pub x: S15Fixed16,
    pub y: S15Fixed16,
    pub z: S15Fixed16,
}

impl XyzNumber {
    pub fn read(bytes: &[u8], off: usize) -> Option<Self> {
        Some(Self {
            x: S15Fixed16::read(bytes, off)?,
            y: S15Fixed16::read(bytes, off + 4)?,
            z: S15Fixed16::read(bytes, off + 8)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Expectation source: `icc__s__number_encodings.md` — "The identity
    /// value is `0x00010000` = 1.0" (cross-verified constant, not this
    /// crate's output).
    #[test]
    fn s15fixed16_identity_is_one() {
        let bytes = [0x00, 0x01, 0x00, 0x00];
        let v = S15Fixed16::read(&bytes, 0).unwrap();
        assert_eq!(v.to_f64(), 1.0);
    }

    /// Sign extension: `0xFFFF0000` is −1.0, not 65535.0. Guards the
    /// corpus's "single most common ICC primitive bug".
    /// Expectation derived from the two's-complement definition in
    /// `icc__s__number_encodings.md` (range −32768.0…+32767.99998474).
    #[test]
    fn s15fixed16_is_sign_extended() {
        let bytes = [0xFF, 0xFF, 0x00, 0x00];
        let v = S15Fixed16::read(&bytes, 0).unwrap();
        assert_eq!(v.to_f64(), -1.0);
    }

    /// Expectation source: `icc__s__header.md` — `'acsp'` = `0x61637370`.
    #[test]
    fn acsp_magic_value() {
        assert_eq!(Signature::ACSP.0, 0x6163_7370);
        assert_eq!(Signature::ACSP.to_string(), "'acsp'");
    }

    /// Trailing-space signatures render faithfully, not trimmed.
    #[test]
    fn signature_display_keeps_trailing_space() {
        let sig = Signature::read(b"XYZ ", 0).unwrap();
        assert_eq!(sig.to_string(), "'XYZ '");
    }

    /// Non-printable signatures render as hex, unambiguously.
    #[test]
    fn signature_display_hex_fallback() {
        assert_eq!(Signature(0x0000_0001).to_string(), "0x00000001");
    }

    /// Out-of-range reads return None instead of panicking — untrusted
    /// input must not be able to abort the process.
    #[test]
    fn short_reads_are_none() {
        assert_eq!(u32_be(&[0x00, 0x01], 0), None);
        assert!(S15Fixed16::read(&[], 0).is_none());
        assert!(DateTimeNumber::read(&[0u8; 11], 0).is_none());
    }
}
