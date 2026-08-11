//! # Chromatic adaptation — the von Kries method, with Bradford cones
//!
//! `ICC_Spec/cie/cie__ref__chromatic_adaptation.md` calls this module's
//! subject "the canonical example of the whole problem": an adaptation
//! matrix written from memory is *nearly* Bradford, looks fine, and is
//! wrong. **Every digit here is transcribed from the corpus, which
//! extracted it from two independent sources** (lcms2 `cmswtpnt.c`'s
//! `LamRigg`, and the CRAN `spacesXYZ` vignette — agreeing row for
//! row).
//!
//! ## The method (corpus, cross-verified structurally against lcms2)
//!
//! ```text
//! cone_src = M_A · white_src          cone response of source white
//! cone_dst = M_A · white_dst          cone response of dest white
//! D        = diag(cone_dst / cone_src)
//! M        = M_A⁻¹ · D · M_A          applied to column vectors
//! ```
//!
//! **Order matters and is easy to invert** — `M_A · D · M_A⁻¹` produces
//! a nearly-right matrix with small off-diagonal sign differences. The
//! `identity` test below is the corpus's recommended single best check:
//! src == dst must give exactly the identity transform.
//!
//! ## What is deliberately absent
//!
//! - **von Kries (HPE) matrix**: the corpus's digits are a placeholder
//!   marked DO NOT USE (unsourced this session, and "von Kries" is
//!   ambiguous between the general method — implemented here — and the
//!   specific HPE cone matrix). It lands when sourced.
//! - **CAT02**: CIE 159 is paywalled; not sourced, not needed for ICC.1.
//! - **ICC mandates no CAT at all** (corpus ambiguity A29): a profile's
//!   `chad` stores the *resulting matrix*, so when a profile lacks
//!   `chad` the CMM's choice of Bradford here is a *policy*, citable as
//!   A29, not a spec requirement.

use crate::mat3::Mat3;
use crate::xyz::Xyz;

/// The Bradford cone-response matrix (Lam & Rigg).
///
/// Source: `cie__ref__chromatic_adaptation.md` — **primary-spec tier as
/// of 2026-08-11**: ICC.1:2022 Annex E.3 states these nine values and
/// agrees exactly with both prior independent extractions (lcms2
/// `LamRigg`; CRAN `spacesXYZ`). Scope of that claim, precisely: Annex
/// E is **informative** — "primary-spec" means the digits are printed
/// in the specification, NOT that Bradford is mandated. ICC.1 mandates
/// no CAT at all (corpus A29); choosing Bradford remains iccce policy,
/// per the module doc. Note the corpus's extraction-hazard
/// warning: the PDF sets minus signs in Symbol font and text extractors
/// drop them silently (the matrix extracts all-positive) — the digits
/// below carry their signs from the cross-verified code sources, which
/// the Annex confirms. Rows are cone responses (ρ, γ, β); columns are
/// (X, Y, Z); row-major, applied to a column vector — so
/// ρ = 0.8951·X + 0.2664·Y − 0.1614·Z.
///
/// The corpus's transcription checks: row sums 1.0001 / 1.0000 / 1.0000
/// — the first row's 1.0001 is real, not a typo (asserted in tests, so
/// a mis-transcribed digit here fails loudly).
pub const BRADFORD: Mat3 = Mat3 {
    rows: [
        [0.8951, 0.2664, -0.1614],
        [-0.7502, 1.7135, 0.0367],
        [0.0389, -0.0685, 1.0296],
    ],
};

/// Build the adaptation matrix `M = M_A⁻¹ · D · M_A` taking colours
/// viewed under `white_src` to their appearance under `white_dst`,
/// using cone matrix `cone` (use [`BRADFORD`] unless there is a stated
/// reason not to — it is the de facto ICC default, per the corpus).
///
/// Returns `None` if `cone` is singular or any source cone response is
/// zero (a degenerate white point; dividing by it would manufacture
/// infinities that surface as plausible-looking garbage downstream).
///
/// The inverse is computed at runtime from the sourced forward matrix —
/// the corpus marks published inverse digits NOT SOURCED and directs
/// exactly this (`cie__ref__chromatic_adaptation.md`, GAP note).
#[must_use]
pub fn adaptation_matrix(cone: &Mat3, white_src: Xyz, white_dst: Xyz) -> Option<Mat3> {
    let cone_inv = cone.inverse()?;
    let s = cone.apply([white_src.x, white_src.y, white_src.z]);
    let d = cone.apply([white_dst.x, white_dst.y, white_dst.z]);
    if s.contains(&0.0) {
        return None;
    }
    let diag = Mat3 {
        rows: [
            [d[0] / s[0], 0.0, 0.0],
            [0.0, d[1] / s[1], 0.0],
            [0.0, 0.0, d[2] / s[2]],
        ],
    };
    // M_A⁻¹ · (D · M_A) — the order lcms2 computes, cross-verified.
    Some(cone_inv.mul(&diag.mul(cone)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::illuminant::{D50, D65_XY};
    use crate::xyz::XyY;

    /// Transcription guard: the corpus's row-sum checks (1.0001,
    /// 1.0000, 1.0000 — "the first row's 1.0001 is real"). Catches a
    /// typo'd digit or a transposition between corpus and code.
    /// Expectation source: `cie__ref__chromatic_adaptation.md` sanity
    /// checks — published derivation of the sourced digits, not this
    /// crate's output.
    #[test]
    fn bradford_row_sums_match_corpus() {
        let sums: Vec<f64> = BRADFORD.rows.iter().map(|r| r.iter().sum()).collect();
        assert!((sums[0] - 1.0001).abs() < 1e-12, "row 0 sum {}", sums[0]);
        assert!((sums[1] - 1.0000).abs() < 1e-12, "row 1 sum {}", sums[1]);
        assert!((sums[2] - 1.0000).abs() < 1e-12, "row 2 sum {}", sums[2]);
    }

    /// The corpus's "best single test": src == dst ⇒ exactly the
    /// identity. Catches order, transposition and inversion bugs at
    /// once. Arithmetic-identity expectation class.
    #[test]
    fn same_white_gives_identity() {
        let m = adaptation_matrix(&BRADFORD, D50, D50).unwrap();
        for r in 0..3 {
            for c in 0..3 {
                let expected = if r == c { 1.0 } else { 0.0 };
                assert!(
                    (m.rows[r][c] - expected).abs() < 1e-14,
                    "m[{r}][{c}] = {}",
                    m.rows[r][c]
                );
            }
        }
    }

    /// By construction M maps the source white exactly to the dest
    /// white: M·W_s = M_A⁻¹·D·(M_A·W_s) = M_A⁻¹·cone_dst = W_d.
    /// Arithmetic identity; verifies the pipeline end to end.
    #[test]
    fn adaptation_maps_src_white_to_dst_white() {
        let d65 = XyY {
            x: D65_XY.0,
            y: D65_XY.1,
            luma_y: 1.0,
        }
        .to_xyz()
        .unwrap();
        let m = adaptation_matrix(&BRADFORD, d65, D50).unwrap();
        let out = m.apply([d65.x, d65.y, d65.z]);
        assert!((out[0] - D50.x).abs() < 1e-12);
        assert!((out[1] - D50.y).abs() < 1e-12);
        assert!((out[2] - D50.z).abs() < 1e-12);
    }

    /// Round trip D65→D50→D65 recovers the input (M_b·M_f = I because
    /// the diagonal ratios invert exactly). Arithmetic identity.
    #[test]
    fn adaptation_round_trip() {
        let d65 = XyY {
            x: D65_XY.0,
            y: D65_XY.1,
            luma_y: 1.0,
        }
        .to_xyz()
        .unwrap();
        let fwd = adaptation_matrix(&BRADFORD, d65, D50).unwrap();
        let back = adaptation_matrix(&BRADFORD, D50, d65).unwrap();
        let sample = [0.4, 0.2, 0.7];
        let out = back.apply(fwd.apply(sample));
        for i in 0..3 {
            assert!(
                (out[i] - sample[i]).abs() < 1e-12,
                "channel {i}: {}",
                out[i]
            );
        }
    }

    /// Degenerate white: zero cone response must be refused, not
    /// propagated as infinities.
    #[test]
    fn zero_white_is_refused() {
        let zero = Xyz {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        assert!(adaptation_matrix(&BRADFORD, zero, D50).is_none());
    }
}
