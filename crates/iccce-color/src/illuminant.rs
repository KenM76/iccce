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
/// **Source: Rec. ITU-R BT.709-6 (06/2015), item 1.4**, corroborated by
/// W3C's 1996 sRGB proposal Table 0.2, by W3C CSS Color 4, and by lcms2
/// `cmsvirt.c` (`{ 0.3127, 0.3290, 1.0 }`). **Four publications, zero
/// disagreement** — `cross_verified_2src`, and the primary source is a
/// standards body that is neither IEC nor an implementation.
///
/// ★★ **This comment said "SINGLE SOURCE — not cross-verified" until
/// 2026-08-17, and that had become false.** It mattered more than a
/// stale note usually does: **lcms2 is this project's differential
/// oracle**, so while D65 rested on lcms2 alone, anything built on it —
/// notably `iccce_cmm::builtin::srgb` — would have put the oracle's own
/// white point underneath every conversion iccce then checks *against
/// that oracle*, and the resulting agreement would have been evidence of
/// nothing. **BT.709-6 is what broke that circularity**, and it was one
/// `curl` away for five days while the corpus recorded `itu.int` as
/// blocking agents — a finding that turned out to be an artifact of the
/// User-Agent string.
///
/// ★ **Do not "improve" this with CIE's own 5-figure `0.312 72 /
/// 0.329 03`.** That is a *different number*; the sRGB matrix is defined
/// by the 4-figure value and substituting CIE's changes every cell while
/// looking like a precision upgrade.
///
/// Exposed as
/// chromaticity, which is what the source states; the XYZ form is
/// derived by the caller via [`crate::xyz::XyY`], keeping the
/// derivation visible instead of baking an unsourced XYZ triple in as
/// though it were published.
///
/// D65 is NOT the ICC PCS white point — a D65-referenced profile
/// carries `chad` to bring it to D50 (corpus divergence D5).
pub const D65_XY: (f64, f64) = (0.3127, 0.3290);
