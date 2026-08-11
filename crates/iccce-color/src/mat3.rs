//! # 3×3 matrix arithmetic for colorimetry
//!
//! The smallest matrix type that chromatic adaptation and (later)
//! matrix/TRC transforms need: multiply, apply-to-vector, invert. All
//! `f64` — the corpus warns that ΔE2000's `C̄'⁷` alone overflows `f32`
//! (`cie__ref__delta_e.md` trap 5), and matrix chains accumulate error,
//! so the whole crate computes in `f64` and lets callers narrow at the
//! edges.
//!
//! ## Convention — row-major, applied to column vectors
//!
//! `m.rows[r][c]`, and `apply` computes `y_r = Σ_c m[r][c] · x_c`.
//! This matches the corpus's statement of the Bradford convention
//! (`cie__ref__chromatic_adaptation.md`: "row-major, applied to a
//! column vector — storing it transposed gives a matrix that adapts
//! *something*, just not what you meant").

/// Row-major 3×3 matrix over `f64`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat3 {
    pub rows: [[f64; 3]; 3],
}

impl Mat3 {
    pub const IDENTITY: Mat3 = Mat3 {
        rows: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
    };

    /// `self · other` (standard matrix product; `self` applied second).
    #[must_use]
    pub fn mul(&self, other: &Mat3) -> Mat3 {
        let mut out = [[0.0f64; 3]; 3];
        for (r, out_row) in out.iter_mut().enumerate() {
            for (c, out_cell) in out_row.iter_mut().enumerate() {
                *out_cell = (0..3).map(|k| self.rows[r][k] * other.rows[k][c]).sum();
            }
        }
        Mat3 { rows: out }
    }

    /// `self · v` with `v` a column vector.
    #[must_use]
    pub fn apply(&self, v: [f64; 3]) -> [f64; 3] {
        let r = &self.rows;
        [
            r[0][0] * v[0] + r[0][1] * v[1] + r[0][2] * v[2],
            r[1][0] * v[0] + r[1][1] * v[1] + r[1][2] * v[2],
            r[2][0] * v[0] + r[2][1] * v[1] + r[2][2] * v[2],
        ]
    }

    /// Inverse by adjugate over determinant, or `None` when singular.
    ///
    /// WHY runtime inversion exists at all: the corpus sources the
    /// Bradford *forward* matrix from two independent sources but marks
    /// the published inverse digits as NOT SOURCED — "invert the
    /// sourced forward matrix at runtime, in f64"
    /// (`cie__ref__chromatic_adaptation.md`). Inverting a sourced
    /// matrix is arithmetic; transcribing an unsourced inverse would be
    /// a nine-digit act of faith.
    ///
    /// Singularity test is `det == 0.0` exactly, not an epsilon: the
    /// matrices inverted here (Bradford, primary matrices) are far from
    /// singular, and an epsilon threshold would be a tuned number with
    /// no citation. A caller that hits `None` has a broken input, not a
    /// borderline one.
    #[must_use]
    pub fn inverse(&self) -> Option<Mat3> {
        let m = &self.rows;
        let cof = |r1: usize, r2: usize, c1: usize, c2: usize| {
            m[r1][c1] * m[r2][c2] - m[r1][c2] * m[r2][c1]
        };
        // Determinant by expansion along the first row.
        let det = m[0][0] * cof(1, 2, 1, 2) - m[0][1] * cof(1, 2, 0, 2) + m[0][2] * cof(1, 2, 0, 1);
        if det == 0.0 {
            return None;
        }
        // Adjugate (transposed cofactor matrix) / det.
        Some(Mat3 {
            rows: [
                [
                    cof(1, 2, 1, 2) / det,
                    -cof(0, 2, 1, 2) / det,
                    cof(0, 1, 1, 2) / det,
                ],
                [
                    -cof(1, 2, 0, 2) / det,
                    cof(0, 2, 0, 2) / det,
                    -cof(0, 1, 0, 2) / det,
                ],
                [
                    cof(1, 2, 0, 1) / det,
                    -cof(0, 2, 0, 1) / det,
                    cof(0, 1, 0, 1) / det,
                ],
            ],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// M · M⁻¹ = I is an arithmetic identity, not a measurement — a
    /// legitimate self-checking expectation (corpus
    /// `cie__ref__colorimetry_core.md` §5 classification).
    #[test]
    fn inverse_times_forward_is_identity() {
        let m = Mat3 {
            rows: [[2.0, 1.0, 0.5], [0.0, 3.0, 1.0], [1.0, 0.0, 2.0]],
        };
        let inv = m.inverse().unwrap();
        let prod = m.mul(&inv);
        for r in 0..3 {
            for c in 0..3 {
                let expected = if r == c { 1.0 } else { 0.0 };
                assert!(
                    (prod.rows[r][c] - expected).abs() < 1e-14,
                    "prod[{r}][{c}] = {}",
                    prod.rows[r][c]
                );
            }
        }
    }

    #[test]
    fn singular_matrix_returns_none() {
        let m = Mat3 {
            rows: [[1.0, 2.0, 3.0], [2.0, 4.0, 6.0], [0.0, 1.0, 0.0]],
        };
        assert_eq!(m.inverse(), None);
    }

    #[test]
    fn identity_applies_as_noop() {
        let v = [0.9642, 1.0, 0.8249];
        assert_eq!(Mat3::IDENTITY.apply(v), v);
    }
}
