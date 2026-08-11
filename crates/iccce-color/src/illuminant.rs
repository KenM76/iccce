//! # Standard illuminant white points
//!
//! Sourced from `ICC_Spec/cie/cie__ref__colorimetry_core.md`. The
//! evidence tier of each constant is stated on the constant, because
//! they are NOT equal: D50 is the corpus's most solidly sourced number;
//! D65 is single-source.
//!
//! ## Why the 4-figure D50, and not a longer one
//!
//! Higher-precision D50 values circulate (0.96422…/0.82521…). The
//! corpus's instruction: **use ICC's 4-figure triple everywhere.**
//! Mixing precisions between the white point that built a matrix and
//! the one that adapts it produces a small, uniform, untraceable cast —
//! the archetypal wrong-but-plausible defect
//! (`cie__ref__colorimetry_core.md` §2).

use crate::xyz::Xyz;

/// D50 — the ICC PCS white point, normalised to Y = 1.
///
/// Source: cross-verified from two independent codebases (ICC
/// `IccUtil.cpp` `icD50XYZ` and `lcms2.h` `cmsD50X/Y/Z`), identical to
/// 4 decimal places (`cie__ref__colorimetry_core.md` §2 — "the most
/// solidly sourced number in the corpus"). CIE 15 itself is paywalled
/// and NOT sourced; this constant cites code, not CIE.
///
/// The encoded header form (0xF6D6/0x10000/0xD32D) was additionally
/// verified byte-for-byte against a real profile on 2026-08-11
/// (`icc__s__header.md`).
pub const D50: Xyz = Xyz {
    x: 0.9642,
    y: 1.0000,
    z: 0.8249,
};

/// D65 chromaticity — the sRGB / BT.709 white point.
///
/// Source: lcms2 `cmsvirt.c` verbatim (`{ 0.3127, 0.3290, 1.0 }`).
/// **SINGLE SOURCE — not cross-verified** (`cie__ref__colorimetry_core.md`
/// §2, recorded gap: an independent D65 source). Exposed as
/// chromaticity, which is what the source states; the XYZ form is
/// derived by the caller via [`crate::xyz::XyY`], keeping the
/// derivation visible instead of baking an unsourced XYZ triple in as
/// though it were published.
///
/// D65 is NOT the ICC PCS white point — a D65-referenced profile
/// carries `chad` to bring it to D50 (corpus divergence D5).
pub const D65_XY: (f64, f64) = (0.3127, 0.3290);
