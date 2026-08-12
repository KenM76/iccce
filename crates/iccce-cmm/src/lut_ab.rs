//! # lutAToBType / lutBToAType evaluation — Pass 4 assembly, stage 4
//!
//! Evaluates the v4 element pipelines (`mAB ` clause 10.12, `mBA `
//! 10.13 via `ICC_Spec/icc/icc__type__lutAtoB_lutBtoA.md`; clause
//! numbers + CLUT rules primary_spec, byte tables code-derived — the
//! corpus says so and the profile layer's module doc repeats it):
//!
//! ```text
//!   mAB (device→PCS):  A curves → CLUT → M curves → Matrix(3×4) → B curves
//!   mBA (PCS→device):  B curves → Matrix(3×4) → M curves → CLUT → A curves
//! ```
//!
//! Absent elements (offset 0) are simply not in the pipeline; the
//! present ones evaluate in the type's order. Which subsets are LEGAL
//! is corpus A23 (partially sourced: the CLUT is mandatory when
//! channel counts differ); this evaluator runs whatever the file
//! carries — legality judgements belong to a validator, not here.
//!
//! ## PCS encodings — NOT the legacy ones
//!
//! `mAB `/`mBA ` are not in 6.3.4.2 NOTE 3's legacy set ("lut16Type
//! and namedColor2Type, and only those tag types"), so their PCS side
//! uses the v4 16-bit encodings of Tables 12/13. In normalised (0..1)
//! form these are EXACT small numbers: `L* = n × 100` (65535/655.35 =
//! 100 exactly) and `a*/b* = n × 255 − 128` (65535/257 = 255 exactly);
//! XYZ is u1Fixed15 (`n × 65535/32768`). The exactness is why the
//! decode below multiplies by 100/255 directly rather than
//! round-tripping through code space.
//!
//! ## The 3×4 matrix
//!
//! Nine coefficients row-major, then the three OFFSET terms e03/e13/
//! e23 added per row — the "reading 36 bytes and stopping" trap the
//! profile layer already made unrepresentable; here the offsets are
//! applied, and a test asserts their effect on measured output.

use crate::clut::Clut;
use crate::curve::{CurveError, Trc};
use crate::lut_transform::{PcsKind, PcsValue};

/// Which way this model evaluates — the tag type's property, fixed at
/// build, making the wrong-direction call structurally inert.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    AToB,
    BToA,
}
use iccce_color::{Lab, Xyz};
use iccce_profile::lut::{ClutSamples, CurveElement, LutAB};

/// Errors building the evaluator.
#[derive(Debug, Clone, PartialEq)]
pub enum LutAbError {
    /// A curve element failed conversion (unknown funcType etc.).
    Curve(CurveError),
    /// CLUT shape unusable (degenerate grid, wrong sample count).
    BadClutShape,
    /// The PCS-side arity is not 3 (B curves / matrix side).
    BadChannelCounts { input: u8, output: u8 },
}

impl From<CurveError> for LutAbError {
    fn from(e: CurveError) -> Self {
        Self::Curve(e)
    }
}

impl std::fmt::Display for LutAbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Curve(e) => write!(f, "mAB/mBA curve: {e}"),
            Self::BadClutShape => write!(f, "mAB/mBA clut shape unusable"),
            Self::BadChannelCounts { input, output } => {
                write!(f, "mAB/mBA channel counts unusable in={input} out={output}")
            }
        }
    }
}

/// An `mAB ` or `mBA ` compiled to an evaluable pipeline.
///
/// HISTORY NOTE: the first version was mAB-only, refusing mBA on a
/// curve-count contradiction found during design. The refusal was
/// vindicated within the hour — GP-001: the guessed counts WOULD have
/// been wrong. The counts are now settled from the PDF (mAB
/// 10.12.2/4/6: B/M = output, A = input; mBA 10.13.2/4/6: B/M =
/// input, A = output — conformance's direct reads, fixed in the
/// profile layer), and both directions evaluate. The private `Direction`
/// field makes calling the wrong method a `None`, not a wrong number.
#[derive(Debug, Clone)]
pub struct LutAbModel {
    direction: Direction,
    /// Device-side channel count (input for mAB, output for mBA).
    device_chan: usize,
    /// PCS-side channel count — always 3 here (checked at build).
    a_curves: Option<Vec<Trc>>,
    clut: Option<Clut>,
    m_curves: Option<Vec<Trc>>,
    /// 3×4: [e00..e22 row-major | e03 e13 e23].
    matrix: Option<[f64; 12]>,
    b_curves: Option<Vec<Trc>>,
    pcs: PcsKind,
}

fn curves_to_trc(elements: &[CurveElement]) -> Result<Vec<Trc>, CurveError> {
    elements
        .iter()
        .map(|e| match e {
            CurveElement::Curve(c) => Trc::from_curve(c),
            CurveElement::Parametric(p) => Trc::from_parametric(p),
        })
        .collect()
}

impl LutAbModel {
    /// Build from a decoded `mBA ` tag: input = PCS (3), output =
    /// device. Same stored elements, evaluated the other way.
    pub fn from_mba(lut: &LutAB, pcs: PcsKind) -> Result<LutAbModel, LutAbError> {
        let mut m = Self::build(lut, pcs)?;
        m.direction = Direction::BToA;
        m.device_chan = usize::from(lut.output_chan);
        Ok(m)
    }

    /// Build from a decoded `mAB ` tag (`pcs` from `header.pcs`).
    /// The tag's input side is the device side.
    pub fn from_lut_ab(lut: &LutAB, pcs: PcsKind) -> Result<LutAbModel, LutAbError> {
        Self::build(lut, pcs)
    }

    fn build(lut: &LutAB, pcs: PcsKind) -> Result<LutAbModel, LutAbError> {
        let (input, output) = (usize::from(lut.input_chan), usize::from(lut.output_chan));
        if input == 0 || output == 0 || input > 15 {
            return Err(LutAbError::BadChannelCounts {
                input: lut.input_chan,
                output: lut.output_chan,
            });
        }

        let a_curves = lut.a_curves.as_deref().map(curves_to_trc).transpose()?;
        let m_curves = lut.m_curves.as_deref().map(curves_to_trc).transpose()?;
        let b_curves = lut.b_curves.as_deref().map(curves_to_trc).transpose()?;

        let clut = match &lut.clut {
            None => None,
            Some(c) => {
                let grid: Vec<usize> = c.grid_points[..input.min(16)]
                    .iter()
                    .map(|&g| usize::from(g))
                    .collect();
                let samples: Vec<f64> = match &c.samples {
                    ClutSamples::U8(v) => v.iter().map(|&s| f64::from(s) / 255.0).collect(),
                    ClutSamples::U16(v) => v.iter().map(|&s| f64::from(s) / 65535.0).collect(),
                };
                Some(Clut::new(grid, output, samples).map_err(|_| LutAbError::BadClutShape)?)
            }
        };

        let matrix = lut.matrix.map(|m| {
            let mut out = [0.0f64; 12];
            for (i, v) in m.iter().enumerate() {
                out[i] = v.to_f64();
            }
            out
        });

        // mAB reading by default; from_mba overrides.
        Ok(LutAbModel {
            direction: Direction::AToB,
            device_chan: input,
            a_curves,
            clut,
            m_curves,
            matrix,
            b_curves,
            pcs,
        })
    }

    pub fn device_channels(&self) -> usize {
        self.device_chan
    }

    pub fn pcs_kind(&self) -> PcsKind {
        self.pcs
    }

    /// `mAB `: device → PCS. A → CLUT → M → Matrix → B → decode.
    #[must_use]
    pub fn device_to_pcs(&self, device: &[f64]) -> Option<PcsValue> {
        if self.direction != Direction::AToB || device.len() != self.device_chan {
            return None;
        }
        let mut v: Vec<f64> = device.to_vec();
        if let Some(a) = &self.a_curves {
            v = apply_curves(a, &v)?;
        }
        if let Some(clut) = &self.clut {
            let mut out = vec![0.0f64; clut.outputs()];
            if !clut.eval(&v, &mut out) {
                return None;
            }
            v = out;
        }
        if let Some(m) = &self.m_curves {
            v = apply_curves(m, &v)?;
        }
        if v.len() != 3 {
            return None; // PCS side must be 3 by here
        }
        if let Some(mx) = &self.matrix {
            v = apply_matrix_3x4(mx, &v);
        }
        if let Some(b) = &self.b_curves {
            v = apply_curves(b, &v)?;
        }
        Some(decode_v4_pcs(self.pcs, [v[0], v[1], v[2]]))
    }

    /// `mBA `: PCS → device. encode → B → Matrix → M → CLUT → A
    /// (clause 10.13's order; counts per 10.13.2/4/6 fixed in the
    /// profile layer — GP-001).
    #[must_use]
    pub fn pcs_to_device(&self, pcs: PcsValue) -> Option<Vec<f64>> {
        if self.direction != Direction::BToA {
            return None;
        }
        let mut v: Vec<f64> = encode_v4_pcs(self.pcs, pcs)?.to_vec();
        if let Some(b) = &self.b_curves {
            v = apply_curves(b, &v)?;
        }
        if let Some(mx) = &self.matrix {
            if v.len() != 3 {
                return None;
            }
            v = apply_matrix_3x4(mx, &v);
        }
        if let Some(m) = &self.m_curves {
            v = apply_curves(m, &v)?;
        }
        if let Some(clut) = &self.clut {
            let mut out = vec![0.0f64; clut.outputs()];
            if !clut.eval(&v, &mut out) {
                return None;
            }
            v = out;
        }
        if let Some(a) = &self.a_curves {
            v = apply_curves(a, &v)?;
        }
        Some(v)
    }
}

fn apply_curves(curves: &[Trc], v: &[f64]) -> Option<Vec<f64>> {
    if curves.len() != v.len() {
        return None;
    }
    Some(curves.iter().zip(v).map(|(c, &x)| c.eval(x)).collect())
}

/// The 3×4: nine coefficients row-major, then the three offsets added
/// per row (corpus: dropping them is "a uniform colour cast that looks
/// like a white-point problem"). Output is clamped to [0,1] — the
/// NORMATIVE matrix-output clamp captured in the corpus's per-type
/// re-transcription of 10.12/10.13 (2026-08-11, seventh pass); it was
/// absent from the first implementation of this function because the
/// clause text had not been transcribed yet.
fn apply_matrix_3x4(m: &[f64; 12], v: &[f64]) -> Vec<f64> {
    vec![
        (m[0] * v[0] + m[1] * v[1] + m[2] * v[2] + m[9]).clamp(0.0, 1.0),
        (m[3] * v[0] + m[4] * v[1] + m[5] * v[2] + m[10]).clamp(0.0, 1.0),
        (m[6] * v[0] + m[7] * v[1] + m[8] * v[2] + m[11]).clamp(0.0, 1.0),
    ]
}

/// v4 16-bit PCS decode, normalised form (module doc: L = n×100 and
/// ab = n×255 − 128 are EXACT restatements of Tables 12/13's 16-bit
/// column; XYZ = n × 65535/32768 is u1Fixed15).
fn decode_v4_pcs(kind: PcsKind, n: [f64; 3]) -> PcsValue {
    match kind {
        PcsKind::Lab => PcsValue::Lab(Lab {
            l: n[0] * 100.0,
            a: n[1] * 255.0 - 128.0,
            b: n[2] * 255.0 - 128.0,
        }),
        PcsKind::Xyz => PcsValue::Xyz(Xyz {
            x: n[0] * 65535.0 / 32768.0,
            y: n[1] * 65535.0 / 32768.0,
            z: n[2] * 65535.0 / 32768.0,
        }),
    }
}

/// v4 16-bit PCS encode (inverse of the decode), clamped to [0,1].
fn encode_v4_pcs(kind: PcsKind, pcs: PcsValue) -> Option<[f64; 3]> {
    Some(match (kind, pcs) {
        (PcsKind::Lab, PcsValue::Lab(lab)) => [
            (lab.l / 100.0).clamp(0.0, 1.0),
            ((lab.a + 128.0) / 255.0).clamp(0.0, 1.0),
            ((lab.b + 128.0) / 255.0).clamp(0.0, 1.0),
        ],
        (PcsKind::Xyz, PcsValue::Xyz(xyz)) => [
            (xyz.x * 32768.0 / 65535.0).clamp(0.0, 1.0),
            (xyz.y * 32768.0 / 65535.0).clamp(0.0, 1.0),
            (xyz.z * 32768.0 / 65535.0).clamp(0.0, 1.0),
        ],
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use iccce_profile::lut::CurveElement;
    use iccce_profile::tag_types::Curve;

    fn identity_curves(n: usize) -> Option<Vec<CurveElement>> {
        Some(vec![CurveElement::Curve(Curve::Identity); n])
    }

    /// B-only mAB with identity curves and Lab PCS: the v4 decode's
    /// exact endpoints — n=1 → L*=100 exactly, n=0.50196… (0x8080/
    /// 65535) → a*=0 exactly. Exact-value class (Tables 12/13).
    #[test]
    fn b_only_v4_lab_decode_exact() {
        let lut = LutAB {
            input_chan: 3,
            output_chan: 3,
            b_curves: identity_curves(3),
            matrix: None,
            m_curves: None,
            clut: None,
            a_curves: None,
        };
        let m = LutAbModel::from_lut_ab(&lut, PcsKind::Lab).unwrap();
        match m
            .device_to_pcs(&[1.0, 128.0 / 255.0, 128.0 / 255.0])
            .unwrap()
        {
            PcsValue::Lab(lab) => {
                assert_eq!(lab.l, 100.0);
                assert_eq!(lab.a, 0.0);
                assert_eq!(lab.b, 0.0);
            }
            other => panic!("{other:?}"),
        }
    }

    /// The 3×4 offsets ARRIVE in the output — measured effect, the
    /// trap's regression: same matrix with and without offsets differs
    /// by exactly the offset (through identity B curves and XYZ PCS).
    #[test]
    fn matrix_offsets_applied() {
        let mut mx = [0.0f64; 12];
        mx[0] = 1.0;
        mx[4] = 1.0;
        mx[8] = 1.0; // identity 3×3 part
        mx[9] = 0.25; // e03: X offset
        let make = |matrix| LutAB {
            input_chan: 3,
            output_chan: 3,
            b_curves: identity_curves(3),
            matrix,
            m_curves: identity_curves(3),
            clut: None,
            a_curves: None,
        };
        let with = LutAbModel::from_lut_ab(&make(Some(map_to_s15(mx))), PcsKind::Xyz).unwrap();
        let without = {
            let mut m2 = mx;
            m2[9] = 0.0;
            LutAbModel::from_lut_ab(&make(Some(map_to_s15(m2))), PcsKind::Xyz).unwrap()
        };
        let input = [0.25, 0.5, 0.5];
        let (PcsValue::Xyz(a), PcsValue::Xyz(b)) = (
            with.device_to_pcs(&input).unwrap(),
            without.device_to_pcs(&input).unwrap(),
        ) else {
            panic!("xyz expected");
        };
        let expected_delta = 0.25 * 65535.0 / 32768.0;
        assert!(((a.x - b.x) - expected_delta).abs() < 1e-9, "{}", a.x - b.x);
        assert_eq!(a.y, b.y);
    }

    fn map_to_s15(m: [f64; 12]) -> [iccce_profile::num::S15Fixed16; 12] {
        let mut out = [iccce_profile::num::S15Fixed16(0); 12];
        for (i, v) in m.iter().enumerate() {
            #[allow(clippy::cast_possible_truncation)]
            let raw = (v * 65536.0).round() as i32;
            out[i] = iccce_profile::num::S15Fixed16(raw);
        }
        out
    }

    /// Full mAB pipeline with an identity CLUT: device values pass
    /// through A → CLUT → M → B unchanged and decode as v4 Lab.
    /// Arithmetic identity on the synthetic.
    #[test]
    fn mab_full_pipeline_identity_clut() {
        let mut clut_samples = Vec::new();
        for i0 in 0..2u16 {
            for i1 in 0..2u16 {
                for i2 in 0..2u16 {
                    for v in [i0, i1, i2] {
                        clut_samples.extend_from_slice(&(v * 65535).to_be_bytes());
                    }
                }
            }
        }
        let clut = iccce_profile::lut::ClutElement {
            grid_points: {
                let mut g = [0u8; 16];
                g[0] = 2;
                g[1] = 2;
                g[2] = 2;
                g
            },
            precision: 2,
            samples: iccce_profile::lut::ClutSamples::U16(
                clut_samples
                    .chunks_exact(2)
                    .map(|c| u16::from_be_bytes([c[0], c[1]]))
                    .collect(),
            ),
        };
        let lut = LutAB {
            input_chan: 3,
            output_chan: 3,
            b_curves: identity_curves(3),
            matrix: None,
            m_curves: identity_curves(3),
            clut: Some(clut),
            a_curves: identity_curves(3),
        };
        let m = LutAbModel::from_lut_ab(&lut, PcsKind::Lab).unwrap();
        let probe = [0.3, 0.6, 0.9];
        match m.device_to_pcs(&probe).unwrap() {
            PcsValue::Lab(lab) => {
                // Identity pipeline: the normalised values decode
                // directly per v4 Tables 12/13.
                assert!((lab.l - 30.0).abs() < 1e-9, "{}", lab.l);
                assert!((lab.a - (0.6 * 255.0 - 128.0)).abs() < 1e-9);
                assert!((lab.b - (0.9 * 255.0 - 128.0)).abs() < 1e-9);
            }
            other => panic!("{other:?}"),
        }
    }

    /// mBA on the committed synthetic fixture (category (a)),
    /// cross-checked against transicc's recorded output for the SAME
    /// tag: Lab(50, 0, 0) → CMYK with K = 49.6117%
    /// (tools/gen-profiles/README.md §5, conformance's run at the
    /// pin). implementation-cross-check class: both read identical
    /// synthetic bytes, so agreement bounds the evaluators, not the
    /// spec. Tolerance 1e-3: transicc prints 4 decimals of percent
    /// (~1e-6 in 0..1) but its pipeline quantises to u16 (~1.5e-5)
    /// and the ragged-grid interpolation differs (n-linear vs
    /// hybrid) away from nodes; 1e-3 admits those and still refuses
    /// a wrong curve count (GP-001's symptom was a REFUSAL, and a
    /// swapped count shifts K by whole percent).
    #[test]
    fn mba_fixture_matches_transicc_recorded_value() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/synthetic/v4-cmyk-mab-lab.icc"
        );
        let Ok(bytes) = std::fs::read(path) else {
            panic!("committed fixture missing: {path}");
        };
        let profile = iccce_profile::Profile::parse(&bytes).unwrap();
        let entry = profile
            .tags
            .iter()
            .find(|t| t.sig.to_string() == "'B2A0'")
            .unwrap();
        let decoded = profile.decode_tag(entry).unwrap().unwrap();
        let iccce_profile::tag_types::TagData::LutBToA(l) = decoded.data else {
            panic!("B2A0 not mBA");
        };
        let m = LutAbModel::from_mba(&l, PcsKind::Lab).unwrap();
        let cmyk = m
            .pcs_to_device(PcsValue::Lab(iccce_color::Lab {
                l: 50.0,
                a: 0.0,
                b: 0.0,
            }))
            .unwrap();
        assert_eq!(cmyk.len(), 4);
        assert!(
            (cmyk[3] - 0.496117).abs() < 1e-3,
            "K = {} vs transicc 0.496117",
            cmyk[3]
        );
    }
}
