//! # Number encodings — every primitive an ICC profile is built from
//!
//! ## Purpose
//!
//! This module is the bottom of the generator: it turns Rust numbers into the
//! exact big-endian byte patterns ICC.1 defines, and nothing else. Everything
//! above it (`tags`, `profile`, `recipes`) composes these and never emits a
//! raw byte of its own for a numeric field.
//!
//! ## Sourcing
//!
//! Layouts and scale factors are from
//! `D:\Dev\Rag-Specialized\ICC_Spec\icc\icc__s__number_encodings.md`
//! (`evidence: primary_spec`, ICC.1:2022 clause 4, Tables 4–7, verified
//! 2026-08-11 against the PDF). The one derived constant — the encoded PCS
//! illuminant — is additionally checked against a real shipped profile; see
//! [`D50_ENCODED`].
//!
//! ## The rule that shapes every function here
//!
//! **Byte order is big-endian throughout the entire profile, without
//! exception, and there are no per-tag byte-order flags** (clause 4). A
//! generator that gets one field's endianness wrong produces a file that
//! usually still parses — which is the failure mode this whole project exists
//! to make impossible — so every conversion goes through one of these five
//! functions and none is written inline anywhere else.
//!
//! ## Rounding, and why it is stated rather than assumed
//!
//! `icc__s__number_encodings.md` records ambiguity **A14**: ICC.1:2022 clause
//! 4.6 gives only the encoding table and **no rounding mode**; the single
//! "shall" on rounding anywhere is clause 10.10's "real values shall be
//! rounded to the nearest 16-bit integer", which states **no tie-break rule**.
//! This generator therefore *states* its choice rather than inheriting one:
//! **round-half-away-from-zero** (Rust's `f64::round`), matching ICC's own
//! `icRoundOffset` helper. Where a fixture needs an exact bit pattern the
//! recipe uses [`s15_fixed16_raw`] and supplies the integer directly, so no
//! rounding happens at all — see the colorant columns in `recipes`.

/// `uInt16Number` — clause 4.10. Big-endian.
#[must_use]
pub fn be16(v: u16) -> [u8; 2] {
    v.to_be_bytes()
}

/// `uInt32Number` — clause 4.11. Big-endian.
#[must_use]
pub fn be32(v: u32) -> [u8; 4] {
    v.to_be_bytes()
}

/// `s15Fixed16Number` — clause 4.6 / Table 4: a **signed** 32-bit fixed-point
/// value with 16 fractional bits, i.e. `round(x · 65536)`.
///
/// Table 4's endpoints: `80000000h` = −32 768,0, `00010000h` = 1,0,
/// `7FFFFFFFh` = 32 767 + 65 535/65 536.
///
/// The debug assertion is not decoration. `s15Fixed16` is signed and a silent
/// wrap would produce a *plausible-looking wrong profile* — a fixture that
/// parses, carries a number nobody expected, and grades some later transform
/// against it. A fixture generator failing loudly is the whole point.
#[must_use]
pub fn s15_fixed16(x: f64) -> [u8; 4] {
    let v = (x * 65536.0).round();
    debug_assert!(
        (-2_147_483_648.0..2_147_483_648.0).contains(&v),
        "s15Fixed16 out of range: {x}"
    );
    #[expect(
        clippy::cast_possible_truncation,
        reason = "range checked immediately above"
    )]
    let bits = v as i32;
    bits.to_be_bytes()
}

/// `s15Fixed16Number` from the raw encoded integer, with no arithmetic.
///
/// Used where a recipe needs an **exact** bit pattern and must not depend on
/// float rounding to get it — e.g. colorant columns chosen so their encoded
/// integers sum to the encoded D50 white point exactly. Stating the integer is
/// more honest there than stating a decimal that happens to round to it.
#[must_use]
pub fn s15_fixed16_raw(bits: i32) -> [u8; 4] {
    bits.to_be_bytes()
}

/// `u8Fixed8Number` — clause 4.9 / Table 7: 8 integer bits, 8 fractional,
/// scale 256. `0100h` = 1,0; `FFFFh` = 255 + 255/256.
///
/// This is the encoding of the **`curveType` `count == 1` gamma shorthand**
/// (clause 10.6, Table 35 footnote), which
/// `icc__type__curve_parametric.md` calls "the highest-value trap in this
/// file": gamma 2,2 stores as `0233h` = 563, and a reader that treats the
/// entry as a table sample computes 563/65535 ≈ 0,0086 and crushes everything
/// to black. The `v2-gray-curv-gamma` recipe exists to give that trap a
/// fixture, and it uses gamma **2,0** (`0200h`) precisely because 2,0 *is*
/// exactly representable here and 2,2 is not.
#[must_use]
pub fn u8_fixed8(x: f64) -> [u8; 2] {
    let v = (x * 256.0).round();
    debug_assert!((0.0..65536.0).contains(&v), "u8Fixed8 out of range: {x}");
    #[expect(
        clippy::cast_possible_truncation,
        reason = "range checked immediately above"
    )]
    let bits = v as u16;
    bits.to_be_bytes()
}

/// The PCS illuminant, D50, as its three encoded `s15Fixed16` words.
///
/// **Provenance, and why this one constant is safe to hard-code.**
/// `icc__s__header.md` records that these values have two independent routes:
/// the arithmetic (`round(0,9642 × 65536) = 63190 = F6D6h`,
/// `round(0,8249 × 65536) = 54061 = D32Dh`, with the float triple itself
/// cross-verified between ICC's `IccUtil.cpp` and lcms2's `cmsD50X/Y/Z`), and
/// a byte-level read of Windows' shipped *sRGB Color Space Profile.icm* at
/// offset 68, which matches all three components. That file explicitly
/// discharges the "verify against a real profile before relying on the hex"
/// caveat and says these two constants are safe as parser expectations.
///
/// Clause 7.2.16 requires the header's illuminant to be D50 in every profile,
/// so every recipe writes this; the colorant recipes additionally *sum to it*
/// exactly, which is asserted in the tests.
pub const D50_ENCODED: [i32; 3] = [0x0000_F6D6, 0x0001_0000, 0x0000_D32D];

/// Encode `L*` into the **legacy** 16-bit PCSLAB encoding.
///
/// ICC.1:2022 Tables 42/43, the encoding clause 10.10 assigns to `lut16Type`
/// and clause 10.17 to `namedColor2Type` — and, per clause 6.3.4.2 NOTE 3,
/// **to those tag types and only those**, in a profile of any version.
/// `L* = 100,0` encodes as `FF00h` (65 280 = 100 × 652,8), not `FFFFh`.
///
/// **This is not the same as the general encoding**, and the difference is the
/// ≈0,39 % darkening of neutrals that `docs/ARCHITECTURE.md` DL-005 says hides
/// below the perceptibility anchor. The two fixtures `v4-rgb-mft2-lab`
/// (`mft2`, legacy) and `v4-cmyk-mab-lab` (`mAB `, general) encode the *same*
/// `L*` differently on purpose, so the pair is a discriminator for whether a
/// consumer keys the choice off the tag type.
///
/// # Panics
/// Debug-asserts the input is within `0,0..=100,0`; clause 10.10 says values
/// above 100,0 "shall not be used".
#[must_use]
pub fn legacy_lab_l(l: f64) -> u16 {
    debug_assert!((0.0..=100.0).contains(&l), "legacy L* out of range: {l}");
    let v = (l * 652.80).round();
    #[expect(
        clippy::cast_possible_truncation,
        reason = "0..=100 * 652.8 <= 65280, checked above"
    )]
    let bits = v as u16;
    bits
}

/// Encode `a*` or `b*` into the **legacy** 16-bit PCSLAB encoding:
/// `v = (ab + 128,0) × 256`, so 0 encodes as `8000h` (Tables 42/43).
///
/// The legacy range is −128,0 … +127,996; the general (6.3.4.2) range is
/// −128,0 … +127,0 with zero at `8080h`. Different scale, different zero.
///
/// # Panics
/// Debug-asserts the input is within the legacy representable range.
#[must_use]
pub fn legacy_lab_ab(ab: f64) -> u16 {
    debug_assert!(
        (-128.0..=127.99609375).contains(&ab),
        "legacy a*/b* out of range: {ab}"
    );
    let v = ((ab + 128.0) * 256.0).round();
    #[expect(
        clippy::cast_possible_truncation,
        reason = "range checked immediately above yields 0..=65535"
    )]
    let bits = v as u16;
    bits
}

/// Encode `L*` into the **general** 16-bit PCSLAB encoding of clause 6.3.4.2
/// (Tables 12/13): `L* = 100,0` is `FFFFh`, i.e. `v = L × 655,35`.
///
/// Used by every PCSLAB-carrying tag type that is **not** `lut16Type` or
/// `namedColor2Type` — in this crate, the `mAB `/`mBA ` fixtures.
#[must_use]
pub fn general_lab_l(l: f64) -> u16 {
    debug_assert!((0.0..=100.0).contains(&l), "general L* out of range: {l}");
    let v = (l * 655.35).round();
    #[expect(
        clippy::cast_possible_truncation,
        reason = "0..=100 * 655.35 <= 65535, checked above"
    )]
    let bits = v as u16;
    bits
}

/// Encode `a*`/`b*` into the **general** encoding: `v = (ab + 128,0) × 257`,
/// so 0 encodes as `8080h` and +127,0 is full scale.
#[must_use]
pub fn general_lab_ab(ab: f64) -> u16 {
    debug_assert!(
        (-128.0..=127.0).contains(&ab),
        "general a*/b* out of range: {ab}"
    );
    let v = ((ab + 128.0) * 257.0).round();
    #[expect(
        clippy::cast_possible_truncation,
        reason = "range checked immediately above yields 0..=65535"
    )]
    let bits = v as u16;
    bits
}

/// A growable big-endian byte sink.
///
/// Exists so a tag constructor reads as a transcription of the specification's
/// encoding table — one `push_*` per table row, in table order — rather than
/// as a sequence of `extend_from_slice` calls whose argument types are the
/// only clue to the field width.
#[derive(Debug, Default, Clone)]
pub struct Buf(pub Vec<u8>);

impl Buf {
    #[must_use]
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// A 4-byte signature (clause 4.15 / 7.2): four ASCII characters, MSB
    /// first. Trailing spaces are significant (`'XYZ '`, `'Lab '`, `'mAB '`),
    /// which is why signatures are always written as 4-byte literals here and
    /// never as trimmed strings.
    pub fn sig(&mut self, s: &[u8; 4]) -> &mut Self {
        self.0.extend_from_slice(s);
        self
    }

    pub fn u8(&mut self, v: u8) -> &mut Self {
        self.0.push(v);
        self
    }

    pub fn u16(&mut self, v: u16) -> &mut Self {
        self.0.extend_from_slice(&be16(v));
        self
    }

    pub fn u32(&mut self, v: u32) -> &mut Self {
        self.0.extend_from_slice(&be32(v));
        self
    }

    pub fn s15(&mut self, v: f64) -> &mut Self {
        self.0.extend_from_slice(&s15_fixed16(v));
        self
    }

    pub fn s15_raw(&mut self, bits: i32) -> &mut Self {
        self.0.extend_from_slice(&s15_fixed16_raw(bits));
        self
    }

    /// `n` zero bytes. Used for reserved fields and for pad bytes, which
    /// clause 7.1.2 d) requires to be NULL.
    pub fn zeros(&mut self, n: usize) -> &mut Self {
        self.0.extend(std::iter::repeat_n(0u8, n));
        self
    }

    pub fn bytes(&mut self, b: &[u8]) -> &mut Self {
        self.0.extend_from_slice(b);
        self
    }

    /// A fixed-width, NUL-padded ASCII field — `ncl2`'s 32-byte `prefix`,
    /// `suffix` and `rootName` (clause 10.17, Table 66).
    ///
    /// # Panics
    /// If the string does not leave room for at least one terminating NUL.
    /// Silently truncating a name would produce a fixture whose content
    /// differs from the recipe that claims to describe it.
    pub fn ascii_fixed(&mut self, s: &str, width: usize) -> &mut Self {
        assert!(
            s.len() < width,
            "`{s}` needs {} bytes, field is {width} (must leave a NUL)",
            s.len() + 1
        );
        assert!(s.is_ascii(), "`{s}` is not 7-bit ASCII");
        self.0.extend_from_slice(s.as_bytes());
        self.zeros(width - s.len());
        self
    }

    /// Pad to the next 4-byte boundary with NULs.
    ///
    /// Clause 7.1.2 c): "all tagged element data, **including the last**,
    /// shall be padded by no more than three following pad bytes to reach a
    /// 4-byte boundary"; d) "all pad bytes shall be NULL". Used *inside*
    /// composite tags too — `mAB `/`mBA ` curve elements are individually
    /// 4-aligned (clauses 10.12.2, 10.13.2).
    pub fn align4(&mut self) -> &mut Self {
        let pad = (4 - (self.0.len() % 4)) % 4;
        self.zeros(pad)
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[must_use]
    pub fn done(self) -> Vec<u8> {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The canonical byte pattern for the PCS illuminant. If `s15_fixed16` is
    /// wrong, every number every recipe writes is wrong in a way that would be
    /// very hard to see — so it is checked against a value whose encoding is
    /// independently attested by a real shipped profile (`icc__s__header.md`).
    #[test]
    fn s15_fixed16_encodes_the_pcs_illuminant_canonically() {
        assert_eq!(s15_fixed16(0.9642), [0x00, 0x00, 0xF6, 0xD6]);
        assert_eq!(s15_fixed16(1.0000), [0x00, 0x01, 0x00, 0x00]);
        assert_eq!(s15_fixed16(0.8249), [0x00, 0x00, 0xD3, 0x2D]);
        // …and that the raw form agrees with the arithmetic form.
        for (i, x) in [0.9642, 1.0, 0.8249].into_iter().enumerate() {
            assert_eq!(s15_fixed16(x), s15_fixed16_raw(D50_ENCODED[i]));
        }
    }

    /// Table 4's three endpoints, verbatim.
    #[test]
    fn s15_fixed16_endpoints_match_table_4() {
        assert_eq!(s15_fixed16(-32768.0), [0x80, 0x00, 0x00, 0x00]);
        assert_eq!(s15_fixed16(1.0), [0x00, 0x01, 0x00, 0x00]);
        assert_eq!(
            s15_fixed16(32767.0 + 65535.0 / 65536.0),
            [0x7F, 0xFF, 0xFF, 0xFF]
        );
    }

    /// Table 7: `0100h` is 1,0 and `FFFFh` is 255 + 255/256. Gamma 2,0 is
    /// exactly `0200h`; gamma 2,2 is `0233h` and is NOT exactly 2,2 — the
    /// trap `icc__type__curve_parametric.md` names, asserted so the recipe
    /// that avoids it has something to point at.
    #[test]
    fn u8_fixed8_endpoints_and_the_gamma_trap() {
        assert_eq!(u8_fixed8(1.0), [0x01, 0x00]);
        assert_eq!(u8_fixed8(255.0 + 255.0 / 256.0), [0xFF, 0xFF]);
        assert_eq!(u8_fixed8(2.0), [0x02, 0x00]);
        assert_eq!(u8_fixed8(2.2), [0x02, 0x33]);
        assert!((f64::from(0x0233_u16) / 256.0 - 2.2).abs() > 1e-9);
    }

    /// The legacy/general split, at the three points where a mix-up shows.
    /// Legacy full scale is `FF00h` and legacy zero-chroma is `8000h`;
    /// general full scale is `FFFFh` and general zero-chroma is `8080h`.
    #[test]
    fn legacy_and_general_lab_differ_exactly_where_the_tables_say() {
        assert_eq!(legacy_lab_l(100.0), 0xFF00);
        assert_eq!(legacy_lab_l(50.0), 0x7F80);
        assert_eq!(legacy_lab_l(0.0), 0x0000);
        assert_eq!(legacy_lab_ab(0.0), 0x8000);
        assert_eq!(legacy_lab_ab(64.0), 0xC000);
        assert_eq!(legacy_lab_ab(-64.0), 0x4000);

        assert_eq!(general_lab_l(100.0), 0xFFFF);
        assert_eq!(general_lab_l(0.0), 0x0000);
        assert_eq!(general_lab_ab(0.0), 0x8080);

        // The two encodings of L*=100 differ by 255 codes — the ≈0,39 %
        // that DL-005 says sits below the perceptibility anchor.
        assert_eq!(
            u32::from(general_lab_l(100.0)) - u32::from(legacy_lab_l(100.0)),
            255
        );
    }

    #[test]
    fn align4_pads_with_nuls_to_a_four_byte_boundary() {
        for n in 0..8_usize {
            let mut b = Buf::new();
            b.zeros(n).align4();
            assert_eq!(b.len() % 4, 0);
            assert!(b.len() >= n && b.len() < n + 4);
            assert!(b.0.iter().all(|&x| x == 0));
        }
    }
}
