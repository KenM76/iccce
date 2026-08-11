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
//! ## Status
//!
//! Pass 0 scaffold. The colorimetry itself is Pass 1, which begins only
//! once the `ICC_Spec` corpus provides sourced reference values
//! (`docs/ROADMAP.md` Pass 1).

// Pass 1 will populate: xyz, lab, lch, illuminant, adapt, delta_e.
