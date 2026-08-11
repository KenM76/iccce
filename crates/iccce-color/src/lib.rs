//! # iccce-color — CIE colorimetry
//!
//! The mathematical foundation of the engine: CIE XYZ, xyY, Lab and LCh
//! representations; standard illuminants and observers; chromatic
//! adaptation (Bradford, von Kries); and the colour-difference metrics
//! ΔE76, ΔE94, ΔE-CMC and ΔE2000.
//!
//! ## Contracts
//!
//! - **INVARIANT: this crate depends on nothing.** No ICC concepts, no
//!   I/O, no other crates. Its correctness is checkable against published
//!   CIE reference values, and that check must not require constructing
//!   an ICC file. (`docs/ARCHITECTURE.md` §1.)
//! - **Every numeric constant is cited.** Adaptation matrices, illuminant
//!   white points, transfer-function breakpoints — each carries a doc
//!   comment naming the standard and clause it came from, sourced through
//!   the `ICC_Spec` corpus, never from memory. A wrong-by-a-little
//!   constant produces a picture that looks fine and is wrong.
//! - **Test expectations come from the literature**, never from this
//!   crate's own output. A test whose expected value was produced by the
//!   function under test detects change, not error.
//!
//! ## Status — Pass 1
//!
//! Implemented: XYZ/xyY ([`xyz`]), Lab/LCh ([`lab`]), standard
//! illuminants ([`illuminant`]), von Kries-method chromatic adaptation
//! with the Bradford cone matrix ([`adapt`]), ΔE76 and CIEDE2000
//! ([`delta_e`]) — the latter validated against all 34 Sharma et al.
//! (2005) ground-truth pairs.
//!
//! Deliberately absent, as recorded gaps (not oversights):
//!
//! - **ΔE94 / ΔE CMC** — formulas not yet transcribed from a citable
//!   source; an implementation now could only be lcms2-cross-checked,
//!   a weaker claim rule 3 requires labelling
//!   (`cie__ref__delta_e.md`).
//! - **von Kries (HPE) cone matrix** — corpus digits are a placeholder
//!   marked DO NOT USE. The general *method* is implemented; the
//!   specific matrix lands when sourced.
//! - **CAT02** — CIE 159 paywalled; not needed for ICC.1.
//! - **Observer CMF tables** — not needed until spectral input exists.

pub mod adapt;
pub mod delta_e;
pub mod illuminant;
pub mod lab;
pub mod mat3;
pub mod xyz;

pub use adapt::{BRADFORD, adaptation_matrix};
pub use delta_e::{delta_e_76, delta_e_2000, delta_e_2000_k};
pub use illuminant::{D50, D65_XY};
pub use lab::{Lab, Lch};
pub use mat3::Mat3;
pub use xyz::{XyY, Xyz};
