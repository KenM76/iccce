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
//! Pass 3 DONE; Pass 4's evaluation surface COMPLETE as of 2026-08-11
//! (`docs/ROADMAP.md` has the records; this block has been stale twice
//! before — if a module below contradicts it, trust the module):
//! [`curve`] (F.1), [`matrix_trc`] (F.3, four intents),
//! [`gray_trc`] (F.2), [`clut`] (n-linear, A16), [`pcs_encoding`]
//! (both 16-bit Lab encodings + XYZ), [`lut_transform`] (mft1/mft2,
//! both directions, per-type Lab codecs), [`lut_ab`] (mAB/mBA, GP-001
//! counts), [`transform::Chain`] (the 8.10.2 fallback, N↔M channel
//! chains, opt-in [`bpc`] via `with_bpc`), [`bpc`] (Pass 5: the
//! 6.3.4.3-sourced scaling, A41 black, A42 estimation subset), and
//! [`named_color`] (Pass 7's core: ncl2 with the always-legacy Lab
//! decode, A26). Still to come: compiled transforms (Pass 6).

pub mod bpc;
pub mod clut;
pub mod compiled;
pub mod curve;
pub mod gray_trc;
pub mod lut_ab;
pub mod lut_transform;
pub mod matrix_trc;
pub mod named_color;
pub mod pcs_encoding;
pub mod transform;

pub use curve::{CurveError, Trc};
pub use matrix_trc::{MatrixTrc, MatrixTrcTransform, ModelError};
