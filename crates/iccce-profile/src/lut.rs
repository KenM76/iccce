//! # LUT tag types — Pass 2, batch 2
//!
//! `lut8Type` (`'mft1'`, ICC.1:2022 clause 10.11) and `lut16Type`
//! (`'mft2'`, clause 10.10) — the v2-era fixed pipeline
//! (input curves → 3×3 matrix → CLUT → output curves) — and
//! `lutAToBType` (`'mAB '`, 10.12) / `lutBToAType` (`'mBA '`, 10.13),
//! the v4 element pipelines with per-dimension grids.
//!
//! ## Contracts
//!
//! - **Represent, don't evaluate.** Curves, matrices and CLUTs are
//!   held as raw stored samples; interpolation (the A16 silence, worth
//!   up to ~1 ΔE between trilinear and tetrahedral) and Lab decoding
//!   are `iccce-cmm`'s named-and-measured choices, not this crate's.
//! - **The legacy-Lab rule, stated where it will be read**
//!   (ICC.1:2022 6.3.4.2 NOTE 3, primary_spec; and MEASURED in lcms2
//!   at the pin, 2026-08-11 — tools/difftest): a `lut16Type` with Lab
//!   PCS data uses the **legacy 16-bit PCSLAB encoding in a profile
//!   of ANY version** — the selector is the TAG TYPE. `lut8Type` is
//!   NOT in the legacy set ("and only those tag types"): it uses the
//!   general 8-bit encoding. The consumer decodes; this module only
//!   repeats the rule so the consumer cannot miss it.
//! - **Sizes are computed in widened integers and checked against the
//!   actual byte length BEFORE allocation.** `clutPoints` is a single
//!   attacker-controlled byte and `255^15` overflows everything;
//!   `u128::checked_pow` + refusal is the guard.
//!
//! ## Sourcing
//!
//! `icc__type__lut8_lut16.md` (primary_spec, Tables 40/44 confirmed);
//! `icc__type__lutAtoB_lutBtoA.md` (clause numbers + CLUT/mandatory
//! rules primary_spec; **the mAB/mBA byte tables were NOT
//! re-transcribed from the PDF and remain code-derived** — the corpus
//! says so, and so does this line; A23/A24 remain open there).

use crate::num::{S15Fixed16, Signature, u16_be, u32_be};
use crate::tag_types::{Curve, ParametricCurve, TagDecodeError, TagIssue, decode_curve_element};

/// `lut16Type` (`'mft2'`), clause 10.10, Table 40. Fixed pipeline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lut16 {
    pub input_chan: u8,
    pub output_chan: u8,
    /// One byte, applying to EVERY input dimension — the CLUT is
    /// strictly hypercubic (`clutPoints^inputChan` nodes). The
    /// defining limitation versus `lutAToBType`.
    pub clut_points: u8,
    /// 3×3, row-major, 9 × s15Fixed16. Clause 10.10 (A21 resolved):
    /// "shall be an identity matrix unless the input is in the PCSXYZ
    /// colour space" — checkable only by a caller that knows the
    /// input space; see [`Lut16::matrix_is_identity`].
    pub matrix: [S15Fixed16; 9],
    pub input_ent: u16,
    pub output_ent: u16,
    /// `input_chan` tables of `input_ent` entries, back to back.
    pub input_tables: Vec<u16>,
    /// `clut_points^input_chan × output_chan` samples; FIRST input
    /// channel varies SLOWEST (clause 10.10, A20 resolved), a node's
    /// output values contiguous.
    pub clut: Vec<u16>,
    pub output_tables: Vec<u16>,
}

impl Lut16 {
    /// The A21 identity check (`0x00010000` diagonal, zero
    /// elsewhere), for the caller that knows the input space.
    pub fn matrix_is_identity(&self) -> bool {
        matrix9_is_identity(&self.matrix)
    }
}

/// `lut8Type` (`'mft1'`), clause 10.11, Table 44. As `lut16Type`
/// EXCEPT: no `inputEnt`/`outputEnt` fields (tables are always
/// exactly 256 entries), and all samples are `uInt8`. Reading the
/// `mft2` layout onto an `mft1` shifts everything by 4 bytes — the
/// distinct struct makes that impossible here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lut8 {
    pub input_chan: u8,
    pub output_chan: u8,
    pub clut_points: u8,
    pub matrix: [S15Fixed16; 9],
    /// 256 × `input_chan`, back to back.
    pub input_tables: Vec<u8>,
    pub clut: Vec<u8>,
    pub output_tables: Vec<u8>,
}

impl Lut8 {
    pub fn matrix_is_identity(&self) -> bool {
        matrix9_is_identity(&self.matrix)
    }
}

fn matrix9_is_identity(m: &[S15Fixed16; 9]) -> bool {
    const ONE: i32 = 0x0001_0000;
    m.iter().enumerate().all(|(i, v)| {
        let expected = if i % 4 == 0 { ONE } else { 0 };
        v.0 == expected
    })
}

/// One curve element inside an `mAB `/`mBA ` chain — a full
/// `curveType` or `parametricCurveType` element (own type signature +
/// reserved bytes), padded to a 4-byte boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CurveElement {
    Curve(Curve),
    Parametric(ParametricCurve),
}

/// The CLUT element of an `mAB `/`mBA ` (`icCLutStruct`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClutElement {
    /// Per-dimension grid sizes — the substantive advance over
    /// `lut16Type`. Only the first `input_chan` entries are
    /// meaningful; the rest shall be zero (reported when not).
    pub grid_points: [u8; 16],
    /// 1 = uInt8, 2 = uInt16. No other value is legal (refused at
    /// decode — the sample width is otherwise unknowable).
    pub precision: u8,
    pub samples: ClutSamples,
}

/// CLUT samples at their stored width. Held raw: normalisation (÷255
/// or ÷65535) and any Lab decode are the consumer's cited acts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClutSamples {
    U8(Vec<u8>),
    U16(Vec<u16>),
}

/// `lutAToBType` / `lutBToAType` — one storage layout, two traversal
/// orders (`'mAB '`: A → CLUT → M → Matrix → B, device→PCS;
/// `'mBA '`: B → Matrix → M → CLUT → A, PCS→device). The letters name
/// the same storage in both; `B` is always the PCS-side end. Which
/// traversal applies is carried by the tag's type signature, kept by
/// the caller — the layout itself is direction-blind, which is why
/// one struct serves both.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LutAB {
    pub input_chan: u8,
    pub output_chan: u8,
    /// B curves (`output_chan` of them for mAB). `None` = offset 0 =
    /// element absent.
    pub b_curves: Option<Vec<CurveElement>>,
    /// The 3×4 matrix: e00..e22 row-major THEN the three offset terms
    /// e03/e13/e23 — 12 values, 48 bytes. Reading 36 and stopping
    /// leaves the offsets unapplied: "a uniform colour cast that
    /// looks like a white-point problem" (corpus). The fixed-size
    /// array makes the 36-byte mistake unrepresentable.
    pub matrix: Option<[S15Fixed16; 12]>,
    pub m_curves: Option<Vec<CurveElement>>,
    pub clut: Option<ClutElement>,
    pub a_curves: Option<Vec<CurveElement>>,
}

/// Decode `'mft2'`. `data` starts at the icTagBase.
pub(crate) fn decode_lut16(
    type_sig: Signature,
    data: &[u8],
    issues: &mut Vec<TagIssue>,
) -> Result<Lut16, TagDecodeError> {
    // Fixed head: chans(3) + pad(1) + matrix(36) + ent(4) = 44 bytes
    // after the base (offsets 8..52 from tag start).
    if data.len() < 52 {
        return Err(TagDecodeError::ShortForType {
            type_sig,
            needed: 52,
            actual: data.len(),
        });
    }
    let input_chan = data[8];
    let output_chan = data[9];
    let clut_points = data[10];
    if data[11] != 0 {
        issues.push(TagIssue::LutPadNonZero);
    }
    let matrix = read_matrix9(data, 12);
    let input_ent = u16_be(data, 48).expect("length checked");
    let output_ent = u16_be(data, 50).expect("length checked");

    // Sizes in u128 BEFORE allocation (clutPoints^inputChan explodes).
    let in_n = u128::from(input_chan) * u128::from(input_ent);
    let clut_n = clut_nodes_hypercube(clut_points, input_chan)
        .and_then(|nodes| nodes.checked_mul(u128::from(output_chan)));
    let out_n = u128::from(output_chan) * u128::from(output_ent);
    let total_bytes = clut_n
        .and_then(|c| c.checked_add(in_n)?.checked_add(out_n)?.checked_mul(2))
        .and_then(|b| b.checked_add(52));
    let Some(total) = total_bytes else {
        return Err(TagDecodeError::LutSizeOverflow { type_sig });
    };
    if total > data.len() as u128 {
        return Err(TagDecodeError::LutSizeExceedsTag {
            type_sig,
            needed: total,
            actual: data.len(),
        });
    }

    // All bounded above; conversion to usize is now safe.
    #[allow(clippy::cast_possible_truncation)]
    let (in_n, clut_n, out_n) = (in_n as usize, clut_n.unwrap() as usize, out_n as usize);
    let mut off = 52;
    let read_u16s = |off: usize, n: usize| -> Vec<u16> {
        (0..n)
            .map(|i| u16_be(data, off + 2 * i).expect("bounded"))
            .collect()
    };
    let input_tables = read_u16s(off, in_n);
    off += 2 * in_n;
    let clut = read_u16s(off, clut_n);
    off += 2 * clut_n;
    let output_tables = read_u16s(off, out_n);

    Ok(Lut16 {
        input_chan,
        output_chan,
        clut_points,
        matrix,
        input_ent,
        output_ent,
        input_tables,
        clut,
        output_tables,
    })
}

/// Decode `'mft1'`.
pub(crate) fn decode_lut8(
    type_sig: Signature,
    data: &[u8],
    issues: &mut Vec<TagIssue>,
) -> Result<Lut8, TagDecodeError> {
    // Head: chans(3) + pad(1) + matrix(36) = 40 after base (8..48).
    // NO inputEnt/outputEnt — tables are always exactly 256 entries
    // (clause 10.11; ICC comment verbatim: "always 256 bytes").
    if data.len() < 48 {
        return Err(TagDecodeError::ShortForType {
            type_sig,
            needed: 48,
            actual: data.len(),
        });
    }
    let input_chan = data[8];
    let output_chan = data[9];
    let clut_points = data[10];
    if data[11] != 0 {
        issues.push(TagIssue::LutPadNonZero);
    }
    let matrix = read_matrix9(data, 12);

    let in_n = 256u128 * u128::from(input_chan);
    let clut_n = clut_nodes_hypercube(clut_points, input_chan)
        .and_then(|nodes| nodes.checked_mul(u128::from(output_chan)));
    let out_n = 256u128 * u128::from(output_chan);
    let total = clut_n.and_then(|c| c.checked_add(in_n)?.checked_add(out_n)?.checked_add(48));
    let Some(total) = total else {
        return Err(TagDecodeError::LutSizeOverflow { type_sig });
    };
    if total > data.len() as u128 {
        return Err(TagDecodeError::LutSizeExceedsTag {
            type_sig,
            needed: total,
            actual: data.len(),
        });
    }

    #[allow(clippy::cast_possible_truncation)]
    let (in_n, clut_n, out_n) = (in_n as usize, clut_n.unwrap() as usize, out_n as usize);
    let mut off = 48;
    let input_tables = data[off..off + in_n].to_vec();
    off += in_n;
    let clut = data[off..off + clut_n].to_vec();
    off += clut_n;
    let output_tables = data[off..off + out_n].to_vec();

    Ok(Lut8 {
        input_chan,
        output_chan,
        clut_points,
        matrix,
        input_tables,
        clut,
        output_tables,
    })
}

/// Decode `'mAB '` / `'mBA '` (one layout; see [`LutAB`]).
pub(crate) fn decode_lut_ab(
    type_sig: Signature,
    data: &[u8],
    issues: &mut Vec<TagIssue>,
) -> Result<LutAB, TagDecodeError> {
    // Header: chans(2) + pad(2) + five u32 offsets = 24 after base
    // (offsets 8..32 from tag start). Byte table code-derived — see
    // module doc.
    if data.len() < 32 {
        return Err(TagDecodeError::ShortForType {
            type_sig,
            needed: 32,
            actual: data.len(),
        });
    }
    let input_chan = data[8];
    let output_chan = data[9];
    if data[10] != 0 || data[11] != 0 {
        issues.push(TagIssue::LutPadNonZero);
    }
    // All five offsets are from the START OF THE TAG (not the base,
    // not the profile); 0 = element ABSENT (offset 0 would be the
    // type signature, so the sentinel is unambiguous). "The
    // highest-value fact in the file" — getting the base wrong by 8
    // lands mid-curve and produces a curve that still parses.
    let offset_b = u32_be(data, 12).expect("length checked");
    let offset_mat = u32_be(data, 16).expect("length checked");
    let offset_m = u32_be(data, 20).expect("length checked");
    let offset_c = u32_be(data, 24).expect("length checked");
    let offset_a = u32_be(data, 28).expect("length checked");

    // Curve counts are PER TAG TYPE — GP-001 (2026-08-11): the first
    // version used the mAB convention for both types, breaking every
    // real CMYK B2A0 while square LUTs hid it; caught by the synthetic
    // fixture corpus, confirmed against the PDF by icc-conformance:
    //   mAB (10.12.2/4/6): B = output, M = output, A = input
    //   mBA (10.13.2/4/6): B = input,  M = input,  A = output
    // Consistent with "B is always the PCS-side end": for mAB the PCS
    // is the output; for mBA it is the input. lcms2's Type_LUTB2A_Read
    // agrees. (Corpus per-type transcription owed to
    // icc-spec-librarian; the clause readings above are conformance's
    // direct PDF reads, recorded in tools/gen-profiles/README.md §5.)
    let is_mba = type_sig == crate::tag_types::sig::MBA;
    let (b_count, m_count, a_count) = if is_mba {
        (input_chan, input_chan, output_chan)
    } else {
        (output_chan, output_chan, input_chan)
    };
    // Elements are stored back to back, each padded to 4 bytes, with
    // no count field — curve n must be parsed to find curve n+1, so
    // one malformed curve makes the rest unreachable (reported by
    // position via CurveChainBroken).
    let b_curves = decode_curve_chain(type_sig, data, offset_b, b_count, issues)?;
    let m_curves = decode_curve_chain(type_sig, data, offset_m, m_count, issues)?;
    let a_curves = decode_curve_chain(type_sig, data, offset_a, a_count, issues)?;

    let matrix = if offset_mat == 0 {
        None
    } else {
        let off = offset_mat as usize;
        if off + 48 > data.len() {
            return Err(TagDecodeError::ShortForType {
                type_sig,
                needed: off + 48,
                actual: data.len(),
            });
        }
        let mut m = [S15Fixed16(0); 12];
        for (i, slot) in m.iter_mut().enumerate() {
            *slot = S15Fixed16::read(data, off + 4 * i).expect("bounded");
        }
        Some(m)
    };

    let clut = if offset_c == 0 {
        None
    } else {
        Some(decode_clut_element(
            type_sig,
            data,
            offset_c as usize,
            input_chan,
            output_chan,
            issues,
        )?)
    };

    Ok(LutAB {
        input_chan,
        output_chan,
        b_curves,
        matrix,
        m_curves,
        clut,
        a_curves,
    })
}

/// The hypercubic node count `points^chan`, in u128, `None` on
/// overflow. `points` and `chan` are attacker-controlled single
/// bytes; `255^255` must refuse, not wrap.
fn clut_nodes_hypercube(points: u8, chan: u8) -> Option<u128> {
    u128::from(points).checked_pow(u32::from(chan))
}

fn read_matrix9(data: &[u8], off: usize) -> [S15Fixed16; 9] {
    let mut m = [S15Fixed16(0); 9];
    for (i, slot) in m.iter_mut().enumerate() {
        *slot = S15Fixed16::read(data, off + 4 * i).expect("caller bounded");
    }
    m
}

/// Parse `count` back-to-back curve elements starting at `offset`
/// (tag-start-relative; 0 = absent → `Ok(None)`).
fn decode_curve_chain(
    type_sig: Signature,
    data: &[u8],
    offset: u32,
    count: u8,
    issues: &mut Vec<TagIssue>,
) -> Result<Option<Vec<CurveElement>>, TagDecodeError> {
    if offset == 0 {
        return Ok(None);
    }
    let mut pos = offset as usize;
    let mut curves = Vec::with_capacity(usize::from(count));
    for i in 0..count {
        match decode_curve_element(data, pos, issues) {
            Some((element, consumed)) => {
                curves.push(element);
                // Each element padded to a 4-byte boundary.
                pos += consumed.next_multiple_of(4);
            }
            None => {
                return Err(TagDecodeError::CurveChainBroken {
                    type_sig,
                    element: i,
                    position: pos,
                });
            }
        }
    }
    Ok(Some(curves))
}

fn decode_clut_element(
    type_sig: Signature,
    data: &[u8],
    off: usize,
    input_chan: u8,
    output_chan: u8,
    issues: &mut Vec<TagIssue>,
) -> Result<ClutElement, TagDecodeError> {
    // icCLutStruct: gridPoints[16] + prec + 3 pad = 20 bytes.
    if off + 20 > data.len() {
        return Err(TagDecodeError::ShortForType {
            type_sig,
            needed: off + 20,
            actual: data.len(),
        });
    }
    let grid_points: [u8; 16] = data[off..off + 16].try_into().expect("bounded");
    let precision = data[off + 16];
    if data[off + 17..off + 20].iter().any(|&b| b != 0) {
        issues.push(TagIssue::LutPadNonZero);
    }
    // Entries beyond inputChan shall be zero (corpus, code-derived).
    if grid_points[usize::from(input_chan).min(16)..]
        .iter()
        .any(|&g| g != 0)
    {
        issues.push(TagIssue::ClutGridPointsBeyondInputChan);
    }
    // prec ∈ {1,2} or the sample width is unknowable: refuse.
    if precision != 1 && precision != 2 {
        return Err(TagDecodeError::ClutBadPrecision { precision });
    }

    // Node count = Π gridPoints[i] for i < inputChan, in u128.
    let nodes = grid_points[..usize::from(input_chan).min(16)]
        .iter()
        .try_fold(1u128, |acc, &g| acc.checked_mul(u128::from(g)));
    let total_bytes = nodes
        .and_then(|n| n.checked_mul(u128::from(output_chan)))
        .and_then(|s| s.checked_mul(u128::from(precision)))
        .and_then(|b| b.checked_add(off as u128 + 20));
    let Some(total) = total_bytes else {
        return Err(TagDecodeError::LutSizeOverflow { type_sig });
    };
    if total > data.len() as u128 {
        return Err(TagDecodeError::LutSizeExceedsTag {
            type_sig,
            needed: total,
            actual: data.len(),
        });
    }
    #[allow(clippy::cast_possible_truncation)]
    let sample_count = (nodes.unwrap() * u128::from(output_chan)) as usize;

    let start = off + 20;
    let samples = if precision == 1 {
        ClutSamples::U8(data[start..start + sample_count].to_vec())
    } else {
        ClutSamples::U16(
            (0..sample_count)
                .map(|i| u16_be(data, start + 2 * i).expect("bounded"))
                .collect(),
        )
    };
    Ok(ClutElement {
        grid_points,
        precision,
        samples,
    })
}
