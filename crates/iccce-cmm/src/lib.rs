//! # iccce-cmm — the colour management module proper
//!
//! Builds, evaluates and caches colour transforms. The pipeline
//! (`docs/ARCHITECTURE.md` §2):
//!
//! ```text
//!   Profile ──▶ TransformPlan ──▶ CompiledTransform ──▶ pixels
//!              (choose the path)  (flatten to a         (evaluate)
//!                                  fast form)
//! ```
//!
//! ## Contracts
//!
//! - **This is where every approximation lives** — and every one is
//!   named and measured. A CMM is a stack of interpolations; the
//!   difference between an engineering choice and a bug is whether the
//!   error is stated. Any departure from exact colorimetry carries a doc
//!   comment saying what it is and what it costs in ΔE, and an entry in
//!   `docs/NUMERIC_CLAIMS.md`. (`docs/ARCHITECTURE.md` §3.3.)
//! - **Path selection follows the specified fallback order**, not an
//!   invented reasonable one, when a profile lacks the tag for the
//!   requested intent. Absolute colorimetric is media-relative plus a
//!   white-point adjustment — not a fourth table.
//! - **lcms2 is the oracle, never a dependency.** Differential testing
//!   lives in `tools/difftest`, outside the workspace, and disagreement
//!   with lcms2 is a finding settled from the specification text — not
//!   automatically a failure. (`CLAUDE.md` rule 7.)
//! - **Optimisation waits for correctness.** The compiled path is
//!   Pass 6; its error against the uncompiled path is measured and
//!   stated, not assumed negligible.
//!
//! ## Status
//!
//! Pass 3 core: the Annex F.3 matrix/TRC model ([`matrix_trc`]) with
//! curve evaluation/inversion per Annex F.1 ([`curve`]) —
//! media-relative colorimetric only; the absolute intent awaits its
//! sourced formula. Pass 4 groundwork: the n-linear CLUT evaluator
//! ([`clut`], the A16 named choice). Still to come: LUT transform
//! assembly and intents (Pass 4), BPC (Pass 5), compilation (Pass 6)
//! (`docs/ROADMAP.md`).

pub mod clut;
pub mod curve;
pub mod matrix_trc;

pub use curve::{CurveError, Trc};
pub use matrix_trc::{MatrixTrc, MatrixTrcTransform, ModelError};

// Passes 4–7 will populate: lut, intent, bpc, compiled, named_color.
