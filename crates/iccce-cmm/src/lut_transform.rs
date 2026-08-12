//! # lut8/lut16 evaluation pipeline — Pass 4 assembly, stages 1+3
//!
//! Evaluates `mft2` (lut16Type) and `mft1` (lut8Type) tags in BOTH
//! directions — A2B (device→PCS, decode at the end) and B2A
//! (PCS→device, encode at the start; the stored pipeline IS that
//! direction, evaluated forward — no inversion exists in the LUT
//! path):
//!
//! ```text
//!   in → input tables → [3×3 matrix] → CLUT → output tables → out
//! ```
//!
//! Pipeline order per ICC.1:2022 clauses 10.10/10.11 via
//! `ICC_Spec/icc/icc__type__lut8_lut16.md` (primary_spec, Tables
//! 40/44). (The type name `Lut16Model` predates lut8 support and is
//! kept for API continuity; the `PcsCodec` carries the per-depth
//! difference.)
//!
//! ## The rules this module enacts, with sources
//!
//! - **Tables interpolate linearly** (10.6's normative rule applies to
//!   the 1-D input/output tables; entries ÷ 65535 — 10.10: entries
//!   "should be divided by 65 535,0").
//! - **The 3×3 matrix applies ONLY when the input space is PCSXYZ**
//!   (10.10, A21 resolved: "shall be an identity matrix unless the
//!   input is in the PCSXYZ colour space"). This evaluator takes the
//!   input space as a parameter and applies the matrix only then; a
//!   non-identity matrix on non-XYZ input is the caller's malformation
//!   to report (the data is still evaluated with the matrix skipped —
//!   evaluating what the spec defines, reporting what the file gets
//!   wrong).
//! - **CLUT interpolation is n-linear** — the A16 named choice
//!   (`clut.rs` module doc; tetrahedral deferred until sourced).
//! - **Lab PCS encodings are per TAG TYPE** (the D1/D2 rule):
//!   `lut16` uses the LEGACY 16-bit encoding (6.3.4.2 NOTE 3 primary
//!   spec + measured lcms2 behaviour M1); `lut8` uses the 8-bit
//!   Tables 12/13 encoding (A10 resolved) and is NOT in the legacy
//!   set. XYZ PCS uses u1Fixed15 (16-bit); the 8-bit XYZ form is
//!   unsourced and refused by name. See the private `PcsCodec`'s variants.
//!
//! ## Scope
//!
//! Media-relative (the values as stored); intent selection and the
//! source→destination chain live in [`crate::transform`]. Still
//! absent: `mAB `/`mBA ` evaluation (the v4 element pipelines — note
//! they are NOT in the legacy set either and use the v4 16-bit
//!   encodings when they arrive).

use crate::clut::Clut;
use iccce_color::{Lab, Xyz};
use iccce_profile::lut::Lut16;

/// What the PCS side of the tag encodes — from `header.pcs`, supplied
/// by the caller (the tag itself does not know).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PcsKind {
    Xyz,
    Lab,
}

/// The PCS codec a pipeline's end uses — the (tag type × PCS kind)
/// product, all sourced:
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PcsCodec {
    /// lut16 + Lab: the LEGACY 16-bit encoding (6.3.4.2 NOTE 3).
    Lab16Legacy,
    /// lut8 + Lab: the 8-bit encoding of Tables 12/13 — corpus A10
    /// RESOLVED against the PDF: L* 0→00h, 100,0→FFh (v×100/255);
    /// a*/b* −128,0→00h, 0→80h, 127,0→FFh (v−128). lut8 is NOT in
    /// the legacy set ("and only those tag types").
    Lab8,
    /// 16-bit XYZ: u1Fixed15 (code/32768).
    Xyz16,
}

/// The evaluated PCS result, decoded to colorimetric values.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PcsValue {
    Xyz(Xyz),
    /// Lab decoded from the LEGACY encoding (this tag type's rule).
    Lab(Lab),
}

/// Errors constructing the evaluator from a decoded tag.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LutModelError {
    /// Input/output channel counts unusable (input ≤ 0, > 15, or
    /// output not 3 for a PCS-side tag).
    BadChannelCounts { input: u8, output: u8 },
    /// A table dimension violates 10.10's own bounds (entries 2–4096
    /// — A22 resolved) or the CLUT is degenerate.
    BadTableShape,
    /// A lut8 tag with XYZ PCS: the 8-bit XYZ encoding is NOT sourced
    /// in the corpus (Tables 12/13 cover Lab; the XYZ 8-bit form has
    /// no verified row). Refused by name until sourced, never guessed.
    Lut8XyzPcsUnsourced,
}

impl std::fmt::Display for LutModelError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadChannelCounts { input, output } => {
                write!(f, "lut16: unusable channel counts in={input} out={output}")
            }
            Self::BadTableShape => write!(f, "lut16: table shape outside 10.10's bounds"),
            Self::Lut8XyzPcsUnsourced => write!(
                f,
                "lut8 with XYZ PCS: 8-bit XYZ encoding unsourced, refused by name"
            ),
        }
    }
}

/// An `mft2` tag compiled to an evaluable device→PCS pipeline.
#[derive(Debug, Clone, PartialEq)]
pub struct Lut16Model {
    input_chan: usize,
    output_chan: usize,
    /// Per-channel input tables, normalised to f64 (÷65535).
    input_tables: Vec<Vec<f64>>,
    /// The 3×3, applied only for XYZ input (A21) — `None` here means
    /// "not applicable" (non-XYZ input or identity).
    matrix: Option<[[f64; 3]; 3]>,
    clut: Clut,
    output_tables: Vec<Vec<f64>>,
    codec: PcsCodec,
}

impl Lut16Model {
    /// Build from a decoded tag. `input_is_pcs_xyz` — whether the
    /// tag's INPUT side is PCSXYZ (true only for B2A-direction tags in
    /// XYZ-PCS profiles); governs matrix applicability per A21.
    /// `pcs` — what the tag's PCS side encodes, from `header.pcs`.
    pub fn from_lut16(
        lut: &Lut16,
        input_is_pcs_xyz: bool,
        pcs: PcsKind,
    ) -> Result<Lut16Model, LutModelError> {
        let input_chan = usize::from(lut.input_chan);
        let output_chan = usize::from(lut.output_chan);
        if input_chan == 0 || input_chan > 15 || output_chan == 0 {
            return Err(LutModelError::BadChannelCounts {
                input: lut.input_chan,
                output: lut.output_chan,
            });
        }
        let in_ent = usize::from(lut.input_ent);
        let out_ent = usize::from(lut.output_ent);
        // 10.10 (A22 resolved): 2..=4096 entries.
        if !(2..=4096).contains(&in_ent) || !(2..=4096).contains(&out_ent) {
            return Err(LutModelError::BadTableShape);
        }

        let norm = |v: u16| f64::from(v) / 65535.0;
        let input_tables: Vec<Vec<f64>> = (0..input_chan)
            .map(|c| {
                lut.input_tables[c * in_ent..(c + 1) * in_ent]
                    .iter()
                    .copied()
                    .map(norm)
                    .collect()
            })
            .collect();
        let output_tables: Vec<Vec<f64>> = (0..output_chan)
            .map(|c| {
                lut.output_tables[c * out_ent..(c + 1) * out_ent]
                    .iter()
                    .copied()
                    .map(norm)
                    .collect()
            })
            .collect();

        let clut = Clut::new(
            vec![usize::from(lut.clut_points); input_chan],
            output_chan,
            lut.clut.iter().copied().map(norm).collect(),
        )
        .map_err(|_| LutModelError::BadTableShape)?;

        // A21: matrix meaningful only for PCSXYZ input; skip (and let
        // the caller report) otherwise. Identity is also skipped —
        // multiplying by identity buys nothing but rounding.
        let matrix = if input_is_pcs_xyz && !lut.matrix_is_identity() {
            let m = &lut.matrix;
            let f = |i: usize| m[i].to_f64();
            Some([[f(0), f(1), f(2)], [f(3), f(4), f(5)], [f(6), f(7), f(8)]])
        } else {
            None
        };

        Ok(Lut16Model {
            input_chan,
            output_chan,
            input_tables,
            matrix,
            clut,
            output_tables,
            codec: match pcs {
                PcsKind::Lab => PcsCodec::Lab16Legacy,
                PcsKind::Xyz => PcsCodec::Xyz16,
            },
        })
    }

    /// Build from a decoded `lut8Type`. Tables are always exactly 256
    /// entries and all samples `uInt8` (clause 10.11; normalised
    /// ÷255). Lab PCS uses the 8-bit Tables 12/13 encoding (A10
    /// resolved); XYZ PCS is refused by name — its 8-bit encoding is
    /// unsourced (see [`LutModelError::Lut8XyzPcsUnsourced`]).
    pub fn from_lut8(
        lut: &iccce_profile::lut::Lut8,
        input_is_pcs_xyz: bool,
        pcs: PcsKind,
    ) -> Result<Lut16Model, LutModelError> {
        if pcs == PcsKind::Xyz {
            return Err(LutModelError::Lut8XyzPcsUnsourced);
        }
        let input_chan = usize::from(lut.input_chan);
        let output_chan = usize::from(lut.output_chan);
        if input_chan == 0 || input_chan > 15 || output_chan == 0 {
            return Err(LutModelError::BadChannelCounts {
                input: lut.input_chan,
                output: lut.output_chan,
            });
        }
        let norm8 = |v: u8| f64::from(v) / 255.0;
        let input_tables: Vec<Vec<f64>> = (0..input_chan)
            .map(|c| {
                lut.input_tables[c * 256..(c + 1) * 256]
                    .iter()
                    .copied()
                    .map(norm8)
                    .collect()
            })
            .collect();
        let output_tables: Vec<Vec<f64>> = (0..output_chan)
            .map(|c| {
                lut.output_tables[c * 256..(c + 1) * 256]
                    .iter()
                    .copied()
                    .map(norm8)
                    .collect()
            })
            .collect();
        let clut = Clut::new(
            vec![usize::from(lut.clut_points); input_chan],
            output_chan,
            lut.clut.iter().copied().map(norm8).collect(),
        )
        .map_err(|_| LutModelError::BadTableShape)?;
        let matrix = if input_is_pcs_xyz && !lut.matrix_is_identity() {
            let m = &lut.matrix;
            let f = |i: usize| m[i].to_f64();
            Some([[f(0), f(1), f(2)], [f(3), f(4), f(5)], [f(6), f(7), f(8)]])
        } else {
            None
        };
        Ok(Lut16Model {
            input_chan,
            output_chan,
            input_tables,
            matrix,
            clut,
            output_tables,
            codec: PcsCodec::Lab8,
        })
    }

    pub fn input_channels(&self) -> usize {
        self.input_chan
    }

    pub fn output_channels(&self) -> usize {
        self.output_chan
    }

    /// What the tag's PCS side encodes (as built).
    pub fn pcs_kind(&self) -> PcsKind {
        match self.codec {
            PcsCodec::Lab16Legacy | PcsCodec::Lab8 => PcsKind::Lab,
            PcsCodec::Xyz16 => PcsKind::Xyz,
        }
    }

    /// Evaluate a B2A-direction tag: PCS value in, device values out.
    ///
    /// The PCS value is ENCODED to normalised 16-bit code space first
    /// (the tables consume codes, not colorimetric values): Lab via
    /// the LEGACY encoding — the same tag-type rule as the A2B
    /// direction (6.3.4.2 NOTE 3) — and XYZ via u1Fixed15
    /// (`pcs_encoding` formulas, continuous form). Requires
    /// `input_chan == 3` (the PCS side is three components).
    ///
    /// The 3×3 matrix: for a B2A tag in an XYZ-PCS profile the input
    /// side genuinely IS PCSXYZ, so A21 makes the matrix applicable —
    /// the caller signals that via `from_lut16`'s `input_is_pcs_xyz`.
    #[must_use]
    pub fn pcs_to_device(&self, pcs: PcsValue) -> Option<Vec<f64>> {
        if self.input_chan != 3 {
            return None;
        }
        // Colorimetric → normalised code space, the inverse of the
        // decode direction, per codec.
        let norm16 = |code: f64| (code / 65535.0).clamp(0.0, 1.0);
        let input: [f64; 3] = match (self.codec, pcs) {
            (PcsCodec::Lab16Legacy, PcsValue::Lab(lab)) => [
                norm16(lab.l * 652.8), // LEGACY encode, this tag type's rule
                norm16((lab.a + 128.0) * 256.0),
                norm16((lab.b + 128.0) * 256.0),
            ],
            // 8-bit Tables 12/13: L 0..100 → 0..1; ab −128..127 → 0..1.
            (PcsCodec::Lab8, PcsValue::Lab(lab)) => [
                (lab.l / 100.0).clamp(0.0, 1.0),
                ((lab.a + 128.0) / 255.0).clamp(0.0, 1.0),
                ((lab.b + 128.0) / 255.0).clamp(0.0, 1.0),
            ],
            (PcsCodec::Xyz16, PcsValue::Xyz(xyz)) => [
                norm16(xyz.x * 32768.0), // u1Fixed15 encode
                norm16(xyz.y * 32768.0),
                norm16(xyz.z * 32768.0),
            ],
            _ => return None, // PCS kind mismatch: caller error
        };

        // Same pipeline as the A2B direction; output stays device 0..1.
        self.eval_pipeline(&input)
    }

    /// Evaluate device values (each 0..1) to the decoded PCS value.
    /// Returns `None` on arity mismatch or when `output_chan != 3`
    /// (a PCS-side result needs exactly three components; other
    /// output arities belong to devicelink evaluation, a later stage).
    #[must_use]
    pub fn device_to_pcs(&self, device: &[f64]) -> Option<PcsValue> {
        if self.output_chan != 3 {
            return None;
        }
        let o = self.eval_pipeline(device)?;

        // PCS decode per codec. Normalised table outputs map back to
        // code space continuously (rounding mid-pipeline would
        // quantise the transform — module doc).
        let code16 = |x: f64| x * 65535.0;
        Some(match self.codec {
            PcsCodec::Xyz16 => PcsValue::Xyz(Xyz {
                // u1Fixed15: value = code/32768 = normalised × 65535/32768.
                x: decode_pcs_xyz_f(code16(o[0])),
                y: decode_pcs_xyz_f(code16(o[1])),
                z: decode_pcs_xyz_f(code16(o[2])),
            }),
            PcsCodec::Lab16Legacy => PcsValue::Lab(Lab {
                l: decode_lab_l_f(code16(o[0])),
                a: decode_lab_ab_f(code16(o[1])),
                b: decode_lab_ab_f(code16(o[2])),
            }),
            // 8-bit Tables 12/13, continuous: normalised 0..1 → L
            // 0..100, ab −128..127 (A10 resolved).
            PcsCodec::Lab8 => PcsValue::Lab(Lab {
                l: o[0] * 100.0,
                a: o[1] * 255.0 - 128.0,
                b: o[2] * 255.0 - 128.0,
            }),
        })
    }

    /// Stages 1–4 (tables → matrix → CLUT → tables) on normalised
    /// code-space values — shared by both directions; only the PCS
    /// encode/decode at the ends differs.
    fn eval_pipeline(&self, input: &[f64]) -> Option<Vec<f64>> {
        if input.len() != self.input_chan {
            return None;
        }
        // Stage 1: per-channel input tables (linear interp, 10.6).
        let mut v: Vec<f64> = input
            .iter()
            .zip(&self.input_tables)
            .map(|(&x, t)| interp_table(t, x))
            .collect();

        // Stage 2: the 3×3, XYZ-input only (A21).
        if let Some(m) = &self.matrix {
            if v.len() == 3 {
                let out = [
                    m[0][0] * v[0] + m[0][1] * v[1] + m[0][2] * v[2],
                    m[1][0] * v[0] + m[1][1] * v[1] + m[1][2] * v[2],
                    m[2][0] * v[0] + m[2][1] * v[1] + m[2][2] * v[2],
                ];
                v = out.to_vec();
            }
        }

        // Stage 3: CLUT (n-linear, the A16 choice).
        let mut clut_out = vec![0.0f64; self.output_chan];
        if !self.clut.eval(&v, &mut clut_out) {
            return None;
        }

        // Stage 4: output tables.
        Some(
            clut_out
                .iter()
                .zip(&self.output_tables)
                .map(|(&x, t)| interp_table(t, x))
                .collect(),
        )
    }
}

/// Continuous (f64-code) versions of the integer decoders in
/// `pcs_encoding` — interpolated table outputs land BETWEEN integer
/// codes, and rounding them to u16 before decoding would quantise the
/// pipeline to 16 bits mid-stream. Same formulas, same sources.
fn decode_pcs_xyz_f(code: f64) -> f64 {
    code / 32768.0
}
fn decode_lab_l_f(code: f64) -> f64 {
    code / 652.8 // LEGACY, always, for this tag type (6.3.4.2 NOTE 3)
}
fn decode_lab_ab_f(code: f64) -> f64 {
    code / 256.0 - 128.0
}

/// 1-D table linear interpolation over [0,1] — the same
/// clamped-index-then-fraction shape as `curve::eval_table` (the
/// DL-016 bug class stays fixed by construction).
fn interp_table(t: &[f64], x: f64) -> f64 {
    let n = t.len();
    debug_assert!(n >= 2);
    let x = x.clamp(0.0, 1.0);
    #[allow(clippy::cast_precision_loss)]
    let pos = x * (n - 1) as f64;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let idx = (pos.floor() as usize).min(n - 2);
    #[allow(clippy::cast_precision_loss)]
    let frac = pos - idx as f64;
    t[idx] + (t[idx + 1] - t[idx]) * frac
}

#[cfg(test)]
mod tests {
    use super::*;
    use iccce_profile::num::S15Fixed16;

    /// A synthetic identity-ish mft2: 3-in/3-out, identity tables
    /// (0, 65535), 2-point CLUT storing the identity mapping, XYZ PCS.
    fn identity_lut(pcs_codes: [[u16; 3]; 8]) -> Lut16 {
        let identity_matrix = {
            let mut m = [S15Fixed16(0); 9];
            for (i, s) in m.iter_mut().enumerate() {
                if i % 4 == 0 {
                    *s = S15Fixed16(0x0001_0000);
                }
            }
            m
        };
        let mut clut = Vec::new();
        for corner in pcs_codes {
            clut.extend_from_slice(&corner);
        }
        Lut16 {
            input_chan: 3,
            output_chan: 3,
            clut_points: 2,
            matrix: identity_matrix,
            input_ent: 2,
            output_ent: 2,
            input_tables: vec![0, 65535, 0, 65535, 0, 65535],
            clut,
            output_tables: vec![0, 65535, 0, 65535, 0, 65535],
        }
    }

    /// White corner carrying the LEGACY Lab white (0xFF00, 0x8000,
    /// 0x8000) must decode to Lab(100, 0, 0) EXACTLY — the D1
    /// invariant carried through the whole pipeline, the same probe
    /// shape the difftest ran against lcms2 (P1). Exact-value class.
    #[test]
    fn lab_pcs_decodes_legacy_through_full_pipeline() {
        let mut corners = [[0u16; 3]; 8];
        corners[7] = [0xFF00, 0x8000, 0x8000]; // node (1,1,1)
        for c in &mut corners[..7] {
            *c = [0, 0x8000, 0x8000];
        }
        let lut = identity_lut(corners);
        let model = Lut16Model::from_lut16(&lut, false, PcsKind::Lab).unwrap();
        match model.device_to_pcs(&[1.0, 1.0, 1.0]).unwrap() {
            PcsValue::Lab(lab) => {
                assert_eq!(lab.l, 100.0);
                assert_eq!(lab.a, 0.0);
                assert_eq!(lab.b, 0.0);
            }
            other => panic!("{other:?}"),
        }
    }

    /// XYZ PCS: 0x8000 codes decode to exactly 1.0 (u1Fixed15).
    #[test]
    fn xyz_pcs_decodes_u1fixed15() {
        let mut corners = [[0u16; 3]; 8];
        corners[7] = [0x8000, 0x8000, 0x8000];
        let lut = identity_lut(corners);
        let model = Lut16Model::from_lut16(&lut, false, PcsKind::Xyz).unwrap();
        match model.device_to_pcs(&[1.0, 1.0, 1.0]).unwrap() {
            PcsValue::Xyz(xyz) => {
                assert_eq!(xyz.x, 1.0);
                assert_eq!(xyz.y, 1.0);
                assert_eq!(xyz.z, 1.0);
            }
            other => panic!("{other:?}"),
        }
    }

    /// The matrix is SKIPPED for non-XYZ input even when non-identity
    /// (A21: only meaningful for PCSXYZ input) — measured output, not
    /// structure: a scaling matrix present but input_is_pcs_xyz=false
    /// must not change the result.
    #[test]
    fn matrix_skipped_for_non_xyz_input() {
        let mut corners = [[0u16; 3]; 8];
        corners[7] = [0x8000, 0x8000, 0x8000];
        let mut lut = identity_lut(corners);
        lut.matrix[0] = S15Fixed16(0x0002_0000); // 2.0: non-identity
        let model = Lut16Model::from_lut16(&lut, false, PcsKind::Xyz).unwrap();
        match model.device_to_pcs(&[1.0, 1.0, 1.0]).unwrap() {
            PcsValue::Xyz(xyz) => assert_eq!(xyz.x, 1.0), // unscaled
            other => panic!("{other:?}"),
        }
    }

    /// Table bounds per 10.10 (A22): entries outside 2..=4096 refuse.
    #[test]
    fn table_bounds_enforced() {
        let mut corners = [[0u16; 3]; 8];
        corners[7] = [0x8000, 0x8000, 0x8000];
        let mut lut = identity_lut(corners);
        lut.input_ent = 1;
        lut.input_tables = vec![0, 0, 0];
        assert_eq!(
            Lut16Model::from_lut16(&lut, false, PcsKind::Xyz),
            Err(LutModelError::BadTableShape)
        );
    }
}
