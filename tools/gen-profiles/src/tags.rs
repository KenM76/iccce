//! # Tag type constructors — one function per ICC tag type, transcribed
//!
//! ## Purpose
//!
//! Each function here emits the **complete tag data element**: the 4-byte type
//! signature, the 4 reserved bytes that clause 7.3 (`icTagBase`) requires to be
//! zero, and the type's own content. Nothing here knows about the tag table,
//! about offsets, or about which tag signature the data will be filed under —
//! that is `profile.rs`'s job, and keeping the split sharp is what lets the
//! same `curveType` bytes be filed as `rTRC`, as `kTRC`, or (see
//! `v2-rgb-shared-trc`) as all three at once from one copy.
//!
//! ## How to read a constructor
//!
//! Every one is written as a transcription of the specification's encoding
//! table, one `push` per table row, in table order, with the byte offsets in
//! the doc comment. If a constructor and its doc comment disagree, the doc
//! comment is what a reviewer should trust and the code is the bug.
//!
//! ## Sourcing, and where it is weaker than it looks
//!
//! | Type | Clause | Corpus file | Evidence tier |
//! |---|---|---|---|
//! | `curv` | 10.6, Table 35 | `icc__type__curve_parametric.md` | `primary_spec` |
//! | `para` | 10.18, Tables 67/68 | `icc__type__curve_parametric.md` | `primary_spec` |
//! | `text` | 10.24 | `icc__type__text_mluc_namedcolor2.md` | clause number verified; **layout code-derived** |
//! | `mluc` | 10.15, Table 54 | `icc__type__text_mluc_namedcolor2.md` | corpus code-derived; **layout re-read from the PDF for this crate** |
//! | `desc` | — | `icc__type__text_mluc_namedcolor2.md` §3 | ★ **no clause exists**: removed in v4, defined in ICC.1:2001-04, NOT OBTAINED |
//! | `ncl2` | 10.17, Table 66 | `icc__type__text_mluc_namedcolor2.md` | `primary_spec` |
//! | `XYZ ` | 10.31 | `icc__type__text_mluc_namedcolor2.md` §5 | clause number verified; layout code-derived |
//! | `sf32` | 10.22 | `icc__type__text_mluc_namedcolor2.md` §6 | clause number verified; layout code-derived |
//! | `mft2` | 10.10, Table 40 | `icc__type__lut8_lut16.md` | `primary_spec` |
//! | `mft1` | 10.11, Table 44 | `icc__type__lut8_lut16.md` | `primary_spec` |
//! | `mAB `/`mBA ` | 10.12/10.13, Tables 45/47 | `icc__type__lutAtoB_lutBtoA.md` | corpus byte tables **code-derived (A23/A24 open)**; **curve counts re-read from the PDF for this crate** — see [`spec_curve_counts`] |
//!
//! ★ **`textDescriptionType` is the weakest thing in this file and that is
//! recorded rather than smoothed over.** It has no clause in ICC.1:2022; its
//! normative definition is in ICC.1:2001-04, which this project does not hold.
//! The layout below is code-derived and *unverifiable against any
//! specification the corpus contains*. It is generated anyway because it is
//! what every v2 profile on the machine actually carries, and because the
//! `v2-desc-short-mac-block` recipe turns the most common real-world
//! malformation into a regression fixture — but a fixture built on an
//! unverifiable layout must not be cited as evidence about the standard.

use crate::bytes::Buf;

/// The 8-byte `icTagBase` every tag data element starts with: the type
/// signature, then 4 reserved bytes that "shall be 0" (clause 7.3).
fn base(sig: &[u8; 4]) -> Buf {
    let mut b = Buf::new();
    b.sig(sig).u32(0);
    b
}

// ===========================================================================
// curveType — 'curv', clause 10.6, Table 35 (v2 + v4)
// ===========================================================================
//
//   0..4   'curv'
//   4..8   reserved, 0
//   8..12  count (uInt32)
//   12..   count × uInt16
//
// The three cases of `count` are the whole point of the type and each gets a
// constructor, because `icc__type__curve_parametric.md` records that two of
// the three are traps:
//
//   count == 0 : identity. No data follows; the tag is exactly 12 bytes.
//                Treating this as "empty/invalid" rejects a perfectly valid
//                identity TRC — the *quiet* failure of the three.
//   count == 1 : the single uInt16 is a u8Fixed8 GAMMA, not a table sample.
//                Verbatim 10.6: "Gamma shall be interpreted as the exponent
//                in the equation y = x^γ and not as an inverse."
//   count >= 2 : a sampled table, first entry at input 0,0, last at 1,0,
//                increment 1,0/(n−1), with linear interpolation between
//                entries (normative — A15 is RESOLVED, it is not silent).

/// `curveType` with `count == 0` — the identity response.
#[must_use]
pub fn curv_identity() -> Vec<u8> {
    base(b"curv").u32(0).clone().done()
}

/// `curveType` with `count == 1` — the `u8Fixed8` gamma shorthand.
#[must_use]
pub fn curv_gamma(gamma: f64) -> Vec<u8> {
    let mut b = base(b"curv");
    b.u32(1)
        .bytes(&crate::bytes::u8_fixed8(gamma))
        .clone()
        .done()
}

/// `curveType` with `count >= 2` — a sampled table.
///
/// # Panics
/// If fewer than two entries are given: clause 10.6's `count >= 2` case is a
/// *table*, and `icc__type__lut8_lut16.md` records (A22) that the analogous
/// LUT tables have a normative minimum of two. A one-entry table would in fact
/// be re-read as the gamma shorthand by any conformant consumer, which is
/// exactly the confusion this crate must never author by accident.
#[must_use]
pub fn curv_table(entries: &[u16]) -> Vec<u8> {
    assert!(
        entries.len() >= 2,
        "curv table needs >= 2 entries (1 would be re-read as the gamma shorthand)"
    );
    let mut b = base(b"curv");
    b.u32(u32::try_from(entries.len()).expect("entry count fits u32"));
    for &e in entries {
        b.u16(e);
    }
    b.done()
}

/// A linear ramp of `n` entries from `0x0000` to `0xFFFF`, for use with
/// [`curv_table`].
///
/// Clause 10.6 puts the first entry at input 0,0 and the last at 1,0, so a
/// linear ramp is the identity response *expressed as a table* — the same
/// transform as `count == 0`, in the other representation. That equality is
/// useful in a fixture: the parse must produce two different `TagData` shapes
/// from two encodings of one function.
///
/// # Panics
/// If `n < 2`.
#[must_use]
pub fn linear_ramp(n: usize) -> Vec<u16> {
    assert!(n >= 2, "a ramp needs >= 2 entries");
    (0..n)
        .map(|i| {
            #[expect(
                clippy::cast_possible_truncation,
                clippy::cast_precision_loss,
                clippy::cast_sign_loss,
                reason = "i < n <= 65536 and the product is bounded by 65535 by construction"
            )]
            let v = ((i as f64) * 65535.0 / ((n - 1) as f64)).round() as u16;
            v
        })
        .collect()
}

// ===========================================================================
// parametricCurveType — 'para', clause 10.18, Tables 67/68 (v4 only, D6)
// ===========================================================================
//
//   0..4    'para'
//   4..8    reserved, 0
//   8..10   funcType (uInt16)
//   10..12  reserved, 0        <- alignment pad, easy to forget
//   12..    N × s15Fixed16, in the FIXED order g, a, b, c, d, e, f

/// Parameter counts per `funcType`, ICC.1:2022 Table 68: 0→1, 1→3, 2→4, 3→5,
/// 4→7.
///
/// ★ **The off-by-one trap, recorded because this crate is what a differential
/// test will feed to lcms2.** lcms2's curve type = ICC `funcType` **+ 1**
/// (`icc__type__curve_parametric.md`). `cmsBuildParametricToneCurve(ctx, 4,
/// …)` builds ICC type **3**. A fixture written here as ICC type 4 and read
/// there as lcms2 type 4 silently drops `e` and `f` and produces a
/// plausible-looking curve that is wrong only in the toe. When one of these
/// fixtures is used against lcms2, translate the number explicitly.
#[must_use]
pub fn para_param_count(func_type: u16) -> Option<usize> {
    match func_type {
        0 => Some(1),
        1 => Some(3),
        2 => Some(4),
        3 => Some(5),
        4 => Some(7),
        _ => None,
    }
}

/// `parametricCurveType`.
///
/// # Panics
/// If the parameter count disagrees with Table 68 for a *known* `funcType`.
/// Unknown function types are allowed through deliberately — the
/// `v4-para-unknown-functype` recipe needs to author one, and the assertion
/// would otherwise make the malformed fixture unwritable by its own generator.
#[must_use]
pub fn para(func_type: u16, params: &[f64]) -> Vec<u8> {
    if let Some(n) = para_param_count(func_type) {
        assert_eq!(
            params.len(),
            n,
            "ICC funcType {func_type} takes {n} parameters per Table 68"
        );
    }
    let mut b = base(b"para");
    b.u16(func_type).u16(0);
    for &p in params {
        b.s15(p);
    }
    b.done()
}

// ===========================================================================
// textType — 'text', clause 10.24 (v2 + v4)
// ===========================================================================

/// `textType`: 7-bit ASCII, NUL-terminated, filling the remainder of the tag.
///
/// The terminating NUL **is** included in the tag size. `icc-profile` reports
/// a missing NUL (`TextUnterminated`) and any byte ≥ 0x80 (`TextNotAscii`)
/// rather than repairing either; both have malformed recipes.
///
/// # Panics
/// If the string is not 7-bit ASCII — a well-formed fixture must not carry the
/// malformation by accident.
#[must_use]
pub fn text(s: &str) -> Vec<u8> {
    assert!(s.is_ascii(), "textType is 7-bit ASCII: `{s}`");
    let mut b = base(b"text");
    b.bytes(s.as_bytes()).u8(0).clone().done()
}

/// `textType` with the caller's exact bytes and no NUL appended — for the
/// malformed recipes only.
#[must_use]
pub fn text_raw(bytes: &[u8]) -> Vec<u8> {
    let mut b = base(b"text");
    b.bytes(bytes).clone().done()
}

// ===========================================================================
// multiLocalizedUnicodeType — 'mluc', clause 10.15, Table 54 (v4, D3)
// ===========================================================================
//
//   0..4    'mluc'
//   4..8    reserved, 0
//   8..12   number of records n (uInt32)
//   12..16  record size — "The value is 12" (Table 54 gives the literal
//           encoding 0000000Ch)
//   16..    n × 12-byte record: language (uInt16, ISO 639-1),
//           country (uInt16, ISO 3166-1), length in BYTES (uInt32),
//           offset FROM THE START OF THE TAG (uInt32)
//   ..      UTF-16BE string storage, NOT NUL-terminated
//
// Clause 10.15 verbatim on the record size: "Any code that needs to access the
// nth record should determine the record's offset by multiplying n by the
// contents of this size field and adding 16. This minor extra effort allows
// for future expansion." That is why `iccce-profile` refuses a recordSize != 12
// instead of assuming 12 — and why `v4-mluc-record-size-16` exists.

/// One `en-US` record. `length` is in **bytes**, not characters — halve it to
/// get the character count, or read twice the string.
#[must_use]
pub fn mluc_en_us(s: &str) -> Vec<u8> {
    mluc_en_us_with(s, 12, 0)
}

/// `mluc` with two deliberately corruptible knobs, for the malformed recipes.
///
/// * `record_size` — written into the record-size field. The *layout* is always
///   the 12-byte one; a value other than 12 therefore makes the file
///   self-inconsistent in exactly the way clause 10.15's forward-extension
///   provision would if it were ever used.
/// * `offset_delta` — added to the string offset. `+1` makes it odd, which is
///   invalid because the offset indexes UTF-16 code units; lcms2 rejects
///   outright (`Type_MLU_Read`: `if (Offset & 1) goto Error;`).
///
/// # Panics
/// Never for `offset_delta == 0`; a non-zero delta may make the record point
/// outside the tag, which is the caller's intent in a malformed recipe.
#[must_use]
pub fn mluc_en_us_with(s: &str, record_size: u32, offset_delta: i64) -> Vec<u8> {
    let utf16: Vec<u8> = s.encode_utf16().flat_map(u16::to_be_bytes).collect();
    // 8 (icTagBase) + 4 (count) + 4 (recordSize) + 12 (one record) = 28.
    let offset = i64::from(28u32) + offset_delta;
    let mut b = base(b"mluc");
    b.u32(1) // number of records
        .u32(record_size)
        .sig(b"enUS") // language 'en' + country 'US', packed as two uInt16
        .u32(u32::try_from(utf16.len()).expect("string length fits u32"))
        .u32(u32::try_from(offset).expect("string offset fits u32"))
        .bytes(&utf16);
    b.done()
}

// ===========================================================================
// textDescriptionType — 'desc' (v2 only; NO ICC.1:2022 clause — see header)
// ===========================================================================

/// `textDescriptionType`: three concatenated representations —
/// ASCII (`uInt32` count including the NUL, then the string), Unicode
/// (`uInt32` language code, `uInt32` count, then UCS-2), and Macintosh
/// ScriptCode (`uInt16` code, `uInt8` length, then a **fixed 67-byte** buffer
/// regardless of that length).
///
/// `include_mac` exists for one recipe. The 67-byte Mac block is "the most
/// frequently malformed structure in real v2 profiles" — and the machine-wide
/// sweep recorded in `docs/ROADMAP.md` found exactly that, four times, on the
/// EIZO profiles this machine ships. `v2-desc-short-mac-block` turns that
/// real-world finding into a fixture, so the *report* has regression coverage
/// on a shape that is known to occur rather than only on one invented here.
#[must_use]
pub fn text_description(s: &str, include_mac: bool) -> Vec<u8> {
    assert!(s.is_ascii(), "desc ASCII block is 7-bit ASCII: `{s}`");
    let mut b = base(b"desc");
    b.u32(u32::try_from(s.len()).expect("length fits u32") + 1)
        .bytes(s.as_bytes())
        .u8(0)
        .u32(0) // Unicode language code
        .u32(0); // Unicode count — no Unicode block
    if include_mac {
        b.u16(0) // ScriptCode code
            .u8(0) // ScriptCode length
            .zeros(67); // the fixed-width buffer
    }
    b.done()
}

// ===========================================================================
// XYZType — 'XYZ ', clause 10.31 (v2 + v4)
// ===========================================================================

/// `XYZType`: one or more `XYZNumber`, each 3 × `s15Fixed16` = 12 bytes.
///
/// ★ `icSigXYZType` and `icSigXYZArrayType` are **the same value**; there is
/// no separate array type, only the singular type with n ≥ 1.
#[must_use]
pub fn xyz_raw(values: &[[i32; 3]]) -> Vec<u8> {
    let mut b = base(b"XYZ ");
    for v in values {
        b.s15_raw(v[0]).s15_raw(v[1]).s15_raw(v[2]);
    }
    b.done()
}

// ===========================================================================
// s15Fixed16ArrayType — 'sf32', clause 10.22 (v2 + v4)
// ===========================================================================

/// `s15Fixed16ArrayType`: `(size − 8) / 4` values, no self-describing shape.
///
/// Principal use is `chromaticAdaptationTag` (`chad`), which is **exactly 9
/// values, row-major 3×3**.
#[must_use]
pub fn sf32(values: &[f64]) -> Vec<u8> {
    let mut b = base(b"sf32");
    for &v in values {
        b.s15(v);
    }
    b.done()
}

/// The 3×3 identity as a `chad` payload.
///
/// **A judgement call, stated rather than buried.** Clause 8.2 makes `chad`
/// *conditionally* required — "when the measurement data used to calculate the
/// profile was specified for an adopted white with a chromaticity different
/// from that of the PCS adopted white". A synthetic fixture has no measurement,
/// and every recipe here declares its white point to *be* the PCS illuminant,
/// so `chad` is **not required** in any of them. It is included in one recipe
/// anyway, as the identity, for two reasons: `sf32` is a tag type the corpus
/// must cover, and the identity is the correct encoding of "no adaptation was
/// applied". A validator that treats a present-but-identity `chad` as a
/// contradiction would be wrong, and this fixture says so.
#[must_use]
pub fn chad_identity() -> Vec<u8> {
    sf32(&[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0])
}

// ===========================================================================
// namedColor2Type — 'ncl2', clause 10.17, Table 66 (v2 + v4)
// ===========================================================================

/// One named-colour entry: a 32-byte NUL-padded root name, 3 × `uInt16` PCS
/// coordinates, then `nDeviceCoords` × `uInt16` device coordinates.
#[derive(Debug, Clone)]
pub struct Ncl2Entry {
    pub root: &'static str,
    /// **Already encoded.** Clause 10.17 verbatim: "For colour values that are
    /// in PCSLAB, this tag uses the legacy 16-bit PCSLAB encoding defined in
    /// 10.8 [*sic* — 10.10] (Tables 42 and 43), **not** the 16-bit PCSLAB
    /// encoding that is defined in 6.3.4.2" — **in a profile of any version**
    /// (A26, RESOLVED). Recipes therefore build these with
    /// [`crate::bytes::legacy_lab_l`] / [`crate::bytes::legacy_lab_ab`], never
    /// the general functions.
    pub pcs: [u16; 3],
    /// Full-range `uInt16`: `0000h` is the minimum and `FFFFh` the maximum —
    /// **not** the `u1Fixed15` PCS encoding. Two different scales in one
    /// struct, which is why they are separate fields here.
    pub device: Vec<u16>,
}

/// `namedColor2Type`.
///
/// Layout (offsets from tag start): 8 `vendorFlag`, 12 `count`,
/// 16 `nDeviceCoords`, 20 `prefix[32]`, 52 `suffix[32]`, 84 entries of stride
/// `32 + 6 + 2·nDeviceCoords`.
///
/// The full colour name is `prefix + rootName + suffix` concatenated; it is
/// **not stored whole anywhere**, which is why all three are separate
/// fixed-width fields here.
///
/// # Panics
/// If any entry's device-coordinate count disagrees with `n_device_coords`.
/// `nDeviceCoords == 0` is legal and means the device field is absent (stride
/// 38) — a normal spot-colour library shape, not an error.
#[must_use]
pub fn ncl2(
    vendor_flag: u32,
    n_device_coords: u32,
    prefix: &str,
    suffix: &str,
    entries: &[Ncl2Entry],
) -> Vec<u8> {
    let mut b = base(b"ncl2");
    b.u32(vendor_flag)
        .u32(u32::try_from(entries.len()).expect("count fits u32"))
        .u32(n_device_coords)
        .ascii_fixed(prefix, 32)
        .ascii_fixed(suffix, 32);
    for e in entries {
        assert_eq!(
            u32::try_from(e.device.len()).expect("device coord count fits u32"),
            n_device_coords,
            "entry `{}` has {} device coords, header says {n_device_coords}",
            e.root,
            e.device.len()
        );
        b.ascii_fixed(e.root, 32);
        for c in e.pcs {
            b.u16(c);
        }
        for &c in &e.device {
            b.u16(c);
        }
    }
    b.done()
}

// ===========================================================================
// lut16Type — 'mft2', clause 10.10, Table 40 (v2 + v4)
// ===========================================================================

/// `lut16Type`. Fixed pipeline: input curves → 3×3 matrix → CLUT → output
/// curves.
///
/// Layout (offsets from tag start): 8 `inputChan`, 9 `outputChan`,
/// 10 `clutPoints`, 11 pad (shall be 0), 12 the 3×3 matrix as 9 ×
/// `s15Fixed16` row-major, 48 `inputEnt`, 50 `outputEnt`, 52 input tables,
/// then CLUT, then output tables.
///
/// Three facts this struct's shape is chosen to keep visible:
///
/// 1. **`clutPoints` is a single byte and applies to EVERY dimension** — the
///    CLUT is strictly hypercubic, `clutPoints^inputChan` nodes. That is the
///    defining limitation versus `lutAToBType`, and it is why `clut` is a flat
///    `Vec` here with a length assertion rather than a shaped type.
/// 2. **The matrix shall be the identity unless the input is PCSXYZ**
///    (clause 10.10, stated twice; same rule in 10.11). Every recipe in this
///    crate has device or PCSLAB input, so every one passes the identity.
/// 3. **PCSLAB content in this tag type uses the LEGACY encoding** — clause
///    10.10 verbatim, "this tag uses the legacy 16-bit PCSLAB encoding …, not
///    the 16-bit PCSLAB encoding defined in 6.3.4.2", **unconditionally, with
///    no version test**. Recipes encode CLUT Lab values with
///    [`crate::bytes::legacy_lab_l`].
///
/// `pad` is a parameter solely so `v2-mft2-pad-nonzero` can set it; it is 0 in
/// every well-formed recipe.
#[derive(Debug, Clone)]
pub struct Mft2 {
    pub input_chan: u8,
    pub output_chan: u8,
    pub clut_points: u8,
    pub pad: u8,
    pub matrix: [f64; 9],
    pub input_ent: u16,
    pub output_ent: u16,
    pub input_tables: Vec<u16>,
    pub clut: Vec<u16>,
    pub output_tables: Vec<u16>,
}

impl Mft2 {
    /// The 3×3 identity — `e00 = e11 = e22 = 00010000h`, the rest 0.
    pub const IDENTITY: [f64; 9] = [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0];

    /// Expected sample counts, computed the way a *consumer* must compute them
    /// before allocating: `inputChan · inputEnt`, `clutPoints^inputChan ·
    /// outputChan`, `outputChan · outputEnt`.
    #[must_use]
    pub fn expected_lengths(&self) -> (usize, usize, usize) {
        let nodes = usize::from(self.clut_points).pow(u32::from(self.input_chan));
        (
            usize::from(self.input_chan) * usize::from(self.input_ent),
            nodes * usize::from(self.output_chan),
            usize::from(self.output_chan) * usize::from(self.output_ent),
        )
    }

    /// Encode. Asserts the three table lengths, because a table of the wrong
    /// length produces a tag that *still parses* with everything after it
    /// shifted — the silent-corruption failure mode this crate must never
    /// author by accident.
    ///
    /// # Panics
    /// If any table length disagrees with [`Self::expected_lengths`].
    #[must_use]
    pub fn encode(&self) -> Vec<u8> {
        let (i, c, o) = self.expected_lengths();
        assert_eq!(self.input_tables.len(), i, "mft2 input table length");
        assert_eq!(self.clut.len(), c, "mft2 CLUT length");
        assert_eq!(self.output_tables.len(), o, "mft2 output table length");

        let mut b = base(b"mft2");
        b.u8(self.input_chan)
            .u8(self.output_chan)
            .u8(self.clut_points)
            .u8(self.pad);
        for m in self.matrix {
            b.s15(m);
        }
        b.u16(self.input_ent).u16(self.output_ent);
        for v in self
            .input_tables
            .iter()
            .chain(&self.clut)
            .chain(&self.output_tables)
        {
            b.u16(*v);
        }
        b.done()
    }
}

// ===========================================================================
// lut8Type — 'mft1', clause 10.11, Table 44 (v2 + v4)
// ===========================================================================

/// `lut8Type`. Identical to `lut16Type` **except**: there are **no
/// `inputEnt`/`outputEnt` fields**, all samples are `uInt8`, and the input and
/// output tables are **always exactly 256 entries**.
///
/// ★ Reading the `mft2` layout onto an `mft1` shifts everything by 4 bytes and
/// yields garbage that still parses. `iccce-profile` makes that
/// unrepresentable by giving the two types separate structs; this crate makes
/// it unrepresentable by giving them separate constructors with different
/// field sets. Neither relies on discipline.
///
/// ★ **`lut8Type` is NOT in the legacy PCSLAB set.** Clause 6.3.4.2 NOTE 3
/// names `lut16Type` and `namedColor2Type` "and only those tag types", so an
/// `mft1` with Lab PCS uses the **general 8-bit** encoding
/// (`L* = v × 100/255`, `a*/b* = v − 128`, zero at `80h`) — which
/// `icc__s__pcs_encoding.md` flags as **A10, NOT SOURCED**, inferred from the
/// structure and ICC's `icLabFromPcs`. The `v2-cmyk-mft1-lab` recipe therefore
/// carries a **stated uncertainty**: its CLUT byte values are what A10 implies,
/// and a consumer disagreeing with them is a finding to settle from the text,
/// not a fixture bug to patch.
#[derive(Debug, Clone)]
pub struct Mft1 {
    pub input_chan: u8,
    pub output_chan: u8,
    pub clut_points: u8,
    pub pad: u8,
    pub matrix: [f64; 9],
    /// 256 × `input_chan`.
    pub input_tables: Vec<u8>,
    /// `clut_points^input_chan` × `output_chan`.
    pub clut: Vec<u8>,
    /// 256 × `output_chan`.
    pub output_tables: Vec<u8>,
}

impl Mft1 {
    #[must_use]
    pub fn expected_lengths(&self) -> (usize, usize, usize) {
        let nodes = usize::from(self.clut_points).pow(u32::from(self.input_chan));
        (
            256 * usize::from(self.input_chan),
            nodes * usize::from(self.output_chan),
            256 * usize::from(self.output_chan),
        )
    }

    /// # Panics
    /// If any table length disagrees with [`Self::expected_lengths`].
    #[must_use]
    pub fn encode(&self) -> Vec<u8> {
        let (i, c, o) = self.expected_lengths();
        assert_eq!(self.input_tables.len(), i, "mft1 input table length");
        assert_eq!(self.clut.len(), c, "mft1 CLUT length");
        assert_eq!(self.output_tables.len(), o, "mft1 output table length");

        let mut b = base(b"mft1");
        b.u8(self.input_chan)
            .u8(self.output_chan)
            .u8(self.clut_points)
            .u8(self.pad);
        for m in self.matrix {
            b.s15(m);
        }
        b.bytes(&self.input_tables)
            .bytes(&self.clut)
            .bytes(&self.output_tables);
        b.done()
    }
}

/// A 256-entry 8-bit identity ramp (`0..=255`), the `mft1` table that does
/// nothing.
#[must_use]
pub fn ramp256() -> Vec<u8> {
    (0..=255u8).collect()
}

// ===========================================================================
// lutAToBType / lutBToAType — 'mAB ' / 'mBA ', clauses 10.12 / 10.13 (v4)
// ===========================================================================

/// Which of the two direction-blind twins is being written.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LutAbKind {
    /// `'mAB '` (6D414220h) — device → PCS. Processing order
    /// A → CLUT → M → Matrix → B (clause 10.12.1).
    AToB,
    /// `'mBA '` (6D424120h) — PCS → device. Processing order
    /// B → Matrix → M → CLUT → A (clause 10.13.1).
    BToA,
}

impl LutAbKind {
    #[must_use]
    pub fn sig(self) -> &'static [u8; 4] {
        match self {
            Self::AToB => b"mAB ",
            Self::BToA => b"mBA ",
        }
    }
}

/// ★★ **How many A, M and B curves each type carries — settled from the
/// primary specification, because the corpus was not specific enough to author
/// bytes from.**
///
/// `icc__type__lutAtoB_lutBtoA.md` says only *"A curves = `inputChan`; B and M
/// curves = `outputChan`"*, one blanket sentence for both types, and its
/// frontmatter marks the byte layouts `icc_secondary_code` with **A23 open**.
/// That sentence is correct for `mAB ` and **wrong for `mBA `**, and the
/// difference is invisible whenever `inputChan == outputChan` — i.e. it hides
/// on every fixture except a real CMYK B2A0.
///
/// Read directly from `ICC.1-2022-05.pdf` on 2026-08-11 (`pdftotext -layout`),
/// VERBATIM:
///
/// * 10.12.2 — "There are the same number of "A" curves as there are **input**
///   channels."
/// * 10.12.4 — "There are the same number of "M" curves as there are
///   **output** channels."
/// * 10.12.6 — "There are the same number of "B" curves as there are
///   **output** channels."
/// * 10.13.2 — "There are the same number of "B" curves as there are **input**
///   channels."
/// * 10.13.4 — "There are the same number of "M" curves as there are **input**
///   channels."
/// * 10.13.6 — "There are the same number of "A" curves as there are
///   **output** channels."
///
/// So the rule is not "A goes with input" — it is **the curve set on the
/// data's entry side is counted by `inputChan`, the set on its exit side by
/// `outputChan`**, and which letter that is depends on the direction. lcms2
/// implements exactly this (`cmstypes.c`, `Type_LUTA2B_Read` /
/// `Type_LUTB2A_Read`, read at the pin in `tools/difftest/vendor/`).
///
/// Returns `(b_count, m_count, a_count)`.
#[must_use]
pub fn spec_curve_counts(kind: LutAbKind, input_chan: u8, output_chan: u8) -> (u8, u8, u8) {
    match kind {
        // A = input, M = output, B = output.
        LutAbKind::AToB => (output_chan, output_chan, input_chan),
        // B = input, M = input, A = output.
        LutAbKind::BToA => (input_chan, input_chan, output_chan),
    }
}

/// The CLUT element of an `mAB `/`mBA ` tag.
///
/// Layout from `offsetC`: 16 bytes of `gridPoints[16]` (only the first
/// `inputChan` are used; "unused entries shall be set to 00h"), 1 byte
/// `precision` ("Shall be either 01h or 02h"), 3 pad bytes, then the data.
///
/// **This is the substantive advance over `lut16Type`: per-dimension grid
/// sizes.** `v4-cmyk-mab-lab` uses a deliberately ragged `5×4×3×2` grid for
/// exactly that reason — a hypercubic grid would let a consumer that ignores
/// the per-dimension array produce the right answer by accident.
#[derive(Debug, Clone)]
pub struct AbClut {
    pub grid_points: [u8; 16],
    /// 1 = `uInt8`, 2 = `uInt16`. No other value is legal; `precision = 3` is
    /// the `v4-mab-clut-precision-3` recipe.
    pub precision: u8,
    /// Samples in `uInt16` form; written as bytes when `precision == 1`.
    pub data: Vec<u16>,
}

impl AbClut {
    /// Node count = Π `grid_points[i]` for `i < input_chan`; sample count =
    /// that × `output_chan`.
    #[must_use]
    pub fn expected_samples(&self, input_chan: u8, output_chan: u8) -> usize {
        self.grid_points[..usize::from(input_chan)]
            .iter()
            .map(|&g| usize::from(g))
            .product::<usize>()
            * usize::from(output_chan)
    }

    fn encode(&self, input_chan: u8, output_chan: u8) -> Vec<u8> {
        // Only asserted for the legal precisions: a malformed recipe sets 3
        // precisely so the consumer's refusal can be exercised, and it must be
        // able to carry an arbitrary payload.
        if self.precision == 1 || self.precision == 2 {
            assert_eq!(
                self.data.len(),
                self.expected_samples(input_chan, output_chan),
                "mAB/mBA CLUT sample count"
            );
        }
        let mut b = Buf::new();
        b.bytes(&self.grid_points).u8(self.precision).zeros(3);
        for &v in &self.data {
            if self.precision == 1 {
                #[expect(
                    clippy::cast_possible_truncation,
                    reason = "an 8-bit CLUT's samples are authored in 0..=255 by the recipe"
                )]
                let byte = v as u8;
                b.u8(byte);
            } else {
                b.u16(v);
            }
        }
        b.done()
    }
}

/// An `mAB `/`mBA ` tag.
///
/// Header layout (offsets from tag start): 8 `inputChan`, 9 `outputChan`,
/// 10–11 pad (zero), 12 `offsetB`, 16 `offsetMat`, 20 `offsetM`,
/// 24 `offsetC`, 28 `offsetA`.
///
/// ★ **All five offsets are relative to the START OF THE TAG**, not to the end
/// of the 8-byte `icTagBase` and not to the start of the profile, and **`0`
/// means the element is ABSENT** rather than "at the beginning" (offset 0
/// would be the type signature, so 0 is unambiguous as a sentinel). Getting the
/// base wrong by 8 lands mid-curve and produces a curve that still parses —
/// the corpus calls this the highest-value fact in its file, and this encoder
/// is where a fixture could get it wrong, so it is stated at the site.
///
/// ★ Clause 10.12.1 / 10.13.1 also settle the corpus's open **A23**: only four
/// element combinations are permitted. For `mAB `: `B`; `M, Matrix, B`;
/// `A, CLUT, B`; `A, CLUT, M, Matrix, B`. For `mBA `: `B`; `B, Matrix, M`;
/// `B, CLUT, A`; `B, Matrix, M, CLUT, A`. Verbatim in both: "At least one
/// processing element shall be included."
///
/// Storage order here is B, matrix, M, CLUT, A, each 4-aligned, which is the
/// order the encoding tables list the offsets in; the *processing* order is
/// different and is a property of the type signature, not of the layout
/// (clause 10.12.1 NOTE: "The processing elements are not in this order in the
/// tag to allow for simplified reading and writing of profiles.").
#[derive(Debug, Clone)]
pub struct LutAb {
    pub kind: LutAbKind,
    pub input_chan: u8,
    pub output_chan: u8,
    /// Each element is a complete embedded `curveType`/`parametricCurveType`
    /// tag — signature and reserved bytes included (clauses 10.12.2/10.13.2:
    /// "the entire tag type, including the tag type signature and reserved
    /// bytes, is included for each curve"). Empty = element absent.
    pub b_curves: Vec<Vec<u8>>,
    pub matrix: Option<[f64; 12]>,
    pub m_curves: Vec<Vec<u8>>,
    pub clut: Option<AbClut>,
    pub a_curves: Vec<Vec<u8>>,
}

impl LutAb {
    /// Encode, laying elements out in the order B, matrix, M, CLUT, A and
    /// filling in each offset as the element is placed.
    #[must_use]
    pub fn encode(&self) -> Vec<u8> {
        // Elements are assembled first so their offsets are known; the 32-byte
        // header is then written with real numbers rather than patched later.
        let mut body = Buf::new();
        // The header occupies bytes 0..32 of the tag.
        body.zeros(32);

        let place_curves = |body: &mut Buf, curves: &[Vec<u8>]| -> u32 {
            if curves.is_empty() {
                return 0;
            }
            let at = u32::try_from(body.len()).expect("tag offset fits u32");
            for c in curves {
                body.bytes(c).align4();
            }
            at
        };

        let offset_b = place_curves(&mut body, &self.b_curves);
        let offset_mat = match &self.matrix {
            None => 0,
            Some(m) => {
                let at = u32::try_from(body.len()).expect("tag offset fits u32");
                for v in *m {
                    body.s15(v);
                }
                body.align4();
                at
            }
        };
        let offset_m = place_curves(&mut body, &self.m_curves);
        let offset_c = match &self.clut {
            None => 0,
            Some(c) => {
                let at = u32::try_from(body.len()).expect("tag offset fits u32");
                body.bytes(&c.encode(self.input_chan, self.output_chan))
                    .align4();
                at
            }
        };
        let offset_a = place_curves(&mut body, &self.a_curves);

        let mut head = Buf::new();
        head.sig(self.kind.sig())
            .u32(0)
            .u8(self.input_chan)
            .u8(self.output_chan)
            .u16(0) // pad, shall be 0
            .u32(offset_b)
            .u32(offset_mat)
            .u32(offset_m)
            .u32(offset_c)
            .u32(offset_a);
        debug_assert_eq!(head.len(), 32);

        let mut out = body.done();
        out[..32].copy_from_slice(&head.done());
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn curv_identity_is_exactly_twelve_bytes() {
        // Clause 10.6: "When n is equal to 0, an identity response is
        // assumed" — and no data follows, so the tag is 8 + 4 bytes.
        assert_eq!(curv_identity().len(), 12);
        assert_eq!(&curv_identity()[..4], b"curv");
    }

    #[test]
    fn curv_gamma_stores_a_u8fixed8_not_a_table_sample() {
        let t = curv_gamma(2.0);
        assert_eq!(t.len(), 14); // 8 base + 4 count + 2 value
        assert_eq!(&t[8..12], &[0, 0, 0, 1]); // count == 1
        assert_eq!(&t[12..14], &[0x02, 0x00]); // 2,0 as u8Fixed8
    }

    #[test]
    fn linear_ramp_hits_both_endpoints_exactly() {
        for n in [2usize, 3, 9, 33, 256] {
            let r = linear_ramp(n);
            assert_eq!(r.len(), n);
            assert_eq!(r[0], 0x0000, "first entry is input 0,0");
            assert_eq!(r[n - 1], 0xFFFF, "last entry is input 1,0");
            assert!(r.windows(2).all(|w| w[0] <= w[1]), "monotonic");
        }
    }

    #[test]
    fn para_layout_has_the_alignment_pad_at_offset_ten() {
        let t = para(0, &[2.0]);
        assert_eq!(&t[..4], b"para");
        assert_eq!(&t[8..10], &[0x00, 0x00]); // funcType 0
        assert_eq!(&t[10..12], &[0x00, 0x00]); // the pad
        assert_eq!(&t[12..16], &[0x00, 0x02, 0x00, 0x00]); // g = 2,0
        assert_eq!(t.len(), 16);
    }

    /// Table 68's parameter counts, which are also the lcms2 off-by-one's
    /// corroborating evidence (lcms2 types 1–5 take 1, 3, 4, 5, 7).
    #[test]
    fn para_param_counts_match_table_68() {
        assert_eq!(
            (0..5)
                .map(|t| para_param_count(t).unwrap())
                .collect::<Vec<_>>(),
            vec![1, 3, 4, 5, 7]
        );
        assert_eq!(para_param_count(5), None);
    }

    #[test]
    fn mluc_offset_is_measured_from_the_start_of_the_tag() {
        let t = mluc_en_us("hi");
        assert_eq!(&t[8..12], &[0, 0, 0, 1]); // one record
        assert_eq!(&t[12..16], &[0, 0, 0, 12]); // recordSize 12
        assert_eq!(&t[16..20], b"enUS");
        assert_eq!(&t[20..24], &[0, 0, 0, 4]); // length in BYTES, not chars
        assert_eq!(&t[24..28], &[0, 0, 0, 28]); // offset from tag start
        assert_eq!(&t[28..], &[0x00, b'h', 0x00, b'i']); // UTF-16BE, no NUL
    }

    /// The `desc` Mac block is fixed-width regardless of its length byte, and
    /// omitting it is the malformation the machine sweep actually found.
    #[test]
    fn desc_mac_block_is_seventy_bytes_and_optional_only_for_the_bad_fixture() {
        let with = text_description("x", true);
        let without = text_description("x", false);
        assert_eq!(with.len() - without.len(), 2 + 1 + 67);
    }

    /// ★ The curve-count rule, asserted against the clause text quoted in
    /// [`spec_curve_counts`]. This is the assertion that would have caught the
    /// blanket-sentence reading: for a CMYK B2A0 the two readings differ.
    #[test]
    fn mba_curve_counts_are_not_the_mirror_of_mab() {
        // CMYK A2B0: 4 device in, 3 PCS out.
        assert_eq!(spec_curve_counts(LutAbKind::AToB, 4, 3), (3, 3, 4));
        // CMYK B2A0: 3 PCS in, 4 device out. B and M are counted by the INPUT
        // channel count here, and A by the output — the opposite of what a
        // single "A=input, B/M=output" rule would give (which would be
        // (4, 4, 3) and would mis-parse every real CMYK B2A0).
        assert_eq!(spec_curve_counts(LutAbKind::BToA, 3, 4), (3, 3, 4));
    }

    /// Offsets are from the start of the tag, absent elements are 0, and every
    /// element starts on a 4-byte boundary.
    #[test]
    fn lut_ab_offsets_are_tag_relative_and_four_aligned() {
        let t = LutAb {
            kind: LutAbKind::AToB,
            input_chan: 2,
            output_chan: 2,
            b_curves: vec![curv_identity(), curv_identity()],
            matrix: None,
            m_curves: vec![],
            clut: Some(AbClut {
                grid_points: {
                    let mut g = [0u8; 16];
                    g[0] = 2;
                    g[1] = 2;
                    g
                },
                precision: 2,
                data: vec![0; 2 * 2 * 2],
            }),
            a_curves: vec![curv_identity(), curv_identity()],
        }
        .encode();

        let at = |o: usize| u32::from_be_bytes([t[o], t[o + 1], t[o + 2], t[o + 3]]);
        assert_eq!(&t[..4], b"mAB ");
        assert_eq!(at(12), 32, "B curves follow the 32-byte header");
        assert_eq!(at(16), 0, "matrix absent");
        assert_eq!(at(20), 0, "M curves absent");
        assert_ne!(at(24), 0, "CLUT present");
        assert_ne!(at(28), 0, "A curves present");
        for o in [12, 24, 28] {
            assert_eq!(at(o) % 4, 0, "element at byte {o} is not 4-aligned");
        }
        // The B curves are two 12-byte identity curves, so the CLUT starts at
        // 32 + 24 = 56 — a number a reader can check by hand against the
        // layout in the doc comment.
        assert_eq!(at(24), 56);
    }

    #[test]
    fn mft2_asserts_its_table_lengths() {
        let l = Mft2 {
            input_chan: 4,
            output_chan: 3,
            clut_points: 3,
            pad: 0,
            matrix: Mft2::IDENTITY,
            input_ent: 2,
            output_ent: 2,
            input_tables: vec![0; 8],
            clut: vec![0; 81 * 3],
            output_tables: vec![0; 6],
        };
        assert_eq!(l.expected_lengths(), (8, 243, 6));
        let bytes = l.encode();
        // 8 base + 4 + 36 matrix + 4 counts + 2·(8 + 243 + 6)
        assert_eq!(bytes.len(), 52 + 2 * (8 + 243 + 6));
    }

    #[test]
    fn mft1_has_no_entry_count_fields_and_256_entry_tables() {
        let l = Mft1 {
            input_chan: 4,
            output_chan: 3,
            clut_points: 3,
            pad: 0,
            matrix: Mft2::IDENTITY,
            input_tables: vec![0; 1024],
            clut: vec![0; 243],
            output_tables: vec![0; 768],
        };
        let bytes = l.encode();
        // 8 base + 4 + 36 matrix + tables — and NOTHING between the matrix and
        // the input tables, which is the 4-byte shift that makes an mft2 read
        // of an mft1 produce plausible garbage.
        assert_eq!(bytes.len(), 48 + 1024 + 243 + 768);
        assert_eq!(&bytes[..4], b"mft1");
    }
}
