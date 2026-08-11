//! # 16-bit PCS encodings — PCSXYZ and both PCSLAB variants
//!
//! Decodes/encodes the 16-bit integer PCS values LUT and named-colour
//! tags carry, per `ICC_Spec/icc/icc__s__pcs_encoding.md`
//! (cross-verified; the selector rule primary_spec).
//!
//! ## The rule that makes this module exist (D1/D2, settled twice)
//!
//! The **legacy** 16-bit PCSLAB encoding belongs to `lut16Type` and
//! `namedColor2Type` — "**and only those tag types**" — **in a profile
//! of ANY version** (ICC.1:2022 6.3.4.2 NOTE 3, primary_spec via the
//! corpus; and MEASURED as lcms2's behaviour at the pin, 2026-08-11 —
//! `icc__ref__lcms2_measured_behaviour.md` M1). The selector is the
//! **tag type**, never `header.version`. Everything else 16-bit uses
//! the v4 general encoding.
//!
//! ## Why the tests here are exact-value, not ΔE
//!
//! Confusing the two Lab encodings costs **ΔE ≈ 0.3–0.5 — below the
//! 1.0 perceptibility threshold** (corpus D1; ledger DL-005): a suite
//! graded in ΔE at the anchor *cannot detect* the confusion, and
//! eyeball QA cannot either. The exact-value invariants below
//! (`0xFF00 → 100.0` legacy, `0x8080 → 0.0` v4, and the
//! discriminating cross-decodes) are the only tests with the power to
//! catch D1, which is why they are stated as `assert_eq!` on exact
//! `f64` values, not tolerances.

/// The two 16-bit PCSLAB encodings. Which one applies is the TAG
/// TYPE's property (see module doc) — this enum exists so the caller
/// must say which it means; there is no default.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabEncoding {
    /// `lut16Type` / `namedColor2Type`, any profile version.
    /// L*: `v / 652.8` (max code 0xFF00 = 100.0);
    /// a*/b*: `v / 256 − 128` (zero at 0x8000).
    Legacy,
    /// Everything else 16-bit (ICC.1:2022 6.3.4.2 Tables 12/13).
    /// L*: `v / 655.35` (max code 0xFFFF = 100.0);
    /// a*/b*: `v / 257 − 128` (zero at 0x8080).
    V4,
}

impl LabEncoding {
    /// Decode an L* code.
    #[must_use]
    pub fn decode_l(self, v: u16) -> f64 {
        match self {
            Self::Legacy => f64::from(v) / 652.8,
            Self::V4 => f64::from(v) / 655.35,
        }
    }

    /// Decode an a*/b* code.
    #[must_use]
    pub fn decode_ab(self, v: u16) -> f64 {
        match self {
            Self::Legacy => f64::from(v) / 256.0 - 128.0,
            Self::V4 => f64::from(v) / 257.0 - 128.0,
        }
    }

    /// Encode an L* value (clamped to the encoding's representable
    /// range; rounding is round-half-away-from-zero via `f64::round`,
    /// the corpus A14 note: a rounding MODE is not spec-mandated, and
    /// this is the recorded choice matching ICC's `icRoundOffset`).
    #[must_use]
    pub fn encode_l(self, l: f64) -> u16 {
        let scaled = match self {
            Self::Legacy => l * 652.8,
            Self::V4 => l * 655.35,
        };
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let v = scaled.round().clamp(0.0, 65535.0) as u16;
        v
    }

    /// Encode an a*/b* value (same clamping/rounding posture).
    #[must_use]
    pub fn encode_ab(self, ab: f64) -> u16 {
        let scaled = match self {
            Self::Legacy => (ab + 128.0) * 256.0,
            Self::V4 => (ab + 128.0) * 257.0,
        };
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        let v = scaled.round().clamp(0.0, 65535.0) as u16;
        v
    }
}

/// PCSXYZ 16-bit (`u1Fixed15`-style): `X = code / 32768`, so
/// `0x8000 = 1.0` and `0xFFFF ≈ 1.99997`. Version-independent
/// (`icc__s__pcs_encoding.md` §1, cross-verified — note the corpus's
/// trap warning: ICC's own `icXyzFromPcs` operates on an already-
/// normalised value; from a RAW u16 the decode is exactly this).
#[must_use]
pub fn decode_pcs_xyz(v: u16) -> f64 {
    f64::from(v) / 32768.0
}

/// Encode a PCSXYZ component. Values ≥ 2.0 are unrepresentable and
/// clamp to 0xFFFF — who clips where is corpus ambiguity A9; clamping
/// at the encoding boundary is one of the normatively specified clips.
#[must_use]
pub fn encode_pcs_xyz(x: f64) -> u16 {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let v = (x * 32768.0).round().clamp(0.0, 65535.0) as u16;
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    /// ★ THE D1 invariants — exact by construction, the only test
    /// class able to catch a legacy/v4 confusion (module doc).
    /// Expectations from `icc__s__pcs_encoding.md` §2's table
    /// (65280/652.8 = 100.0 exactly; 32896/257 = 128.0 exactly) —
    /// published corpus values, cross-verified in two codebases.
    #[test]
    fn d1_exact_value_invariants() {
        // Legacy: 0xFF00 is full-scale L* = 100.
        assert_eq!(LabEncoding::Legacy.decode_l(0xFF00), 100.0);
        // Legacy: a/b zero at 0x8000.
        assert_eq!(LabEncoding::Legacy.decode_ab(0x8000), 0.0);
        // V4: 0xFFFF is full-scale L* = 100.
        assert_eq!(LabEncoding::V4.decode_l(0xFFFF), 100.0);
        // V4: a/b zero at 0x8080.
        assert_eq!(LabEncoding::V4.decode_ab(0x8080), 0.0);
    }

    /// The discriminator: decoding legacy full-scale WITH the v4 rule
    /// gives 99.610894… — the exact wrong value the legacy-Lab probe
    /// measured lcms2 NOT producing (P1 "general" column, 99.6109).
    /// If someone swaps the encodings, this fails loudly while a ΔE
    /// gate would stay green.
    #[test]
    fn d1_discriminator_wrong_decode_is_the_probe_value() {
        let wrong = LabEncoding::V4.decode_l(0xFF00);
        assert!((wrong - 99.6109).abs() < 5e-5, "got {wrong}");
        assert_ne!(wrong, 100.0);
    }

    /// Round trips: decode∘encode is identity on every code (both
    /// encodings, both channels) — arithmetic identity over the full
    /// u16 range, cheap enough to run exhaustively.
    #[test]
    fn encode_decode_round_trips_exhaustively() {
        for v in 0..=u16::MAX {
            for enc in [LabEncoding::Legacy, LabEncoding::V4] {
                assert_eq!(enc.encode_l(enc.decode_l(v)), v, "{enc:?} L {v}");
                assert_eq!(enc.encode_ab(enc.decode_ab(v)), v, "{enc:?} ab {v}");
            }
            assert_eq!(encode_pcs_xyz(decode_pcs_xyz(v)), v, "XYZ {v}");
        }
    }

    /// PCSXYZ anchors: 0x8000 = 1.0 exactly; 0xFFFF = 65535/32768
    /// (the corpus's 1.99997 print).
    #[test]
    fn pcs_xyz_anchors() {
        assert_eq!(decode_pcs_xyz(0x8000), 1.0);
        assert_eq!(decode_pcs_xyz(0xFFFF), 65535.0 / 32768.0);
        assert_eq!(decode_pcs_xyz(0), 0.0);
        // ≥ 2.0 clamps at the encoding boundary (A9's specified clip).
        assert_eq!(encode_pcs_xyz(2.5), 0xFFFF);
    }
}
