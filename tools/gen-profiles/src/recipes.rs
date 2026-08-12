//! # The recipe set — what the corpus contains, and why each fixture exists
//!
//! ## Two categories, and the difference is the whole design
//!
//! **Well-formed** fixtures assert that the parser *reads* correctly: each one
//! must produce zero malformations and a stated decoded summary. **Malformed**
//! fixtures assert that the parser *reports* correctly: each carries exactly
//! one named defect and must produce exactly the corresponding refusal or
//! report — no more (an over-eager parser inventing a second finding is a bug)
//! and no fewer.
//!
//! The second category is the one that is usually missing from a fixture
//! corpus, and it is the more valuable of the two here, because
//! `docs/ARCHITECTURE.md` §3.2 makes *reporting* — not repairing — the
//! parser's contract. A contract with no failing input has no test.
//!
//! ## How a malformed fixture is built
//!
//! **One well-formed base, plus one named mutation.** Never two. A fixture
//! that is broken in two ways cannot tell you which one the consumer reported,
//! and a consumer that reports the wrong one still looks correct. Where the
//! defect is inside a tag rather than in the header or table, the base's tag
//! list is edited instead of its bytes, which keeps the mutation expressed in
//! the vocabulary of the rule it breaks.
//!
//! ## What these fixtures are NOT
//!
//! ★ **Nothing here is a colorimetric reference.** The colorant columns are an
//! arbitrary split of the encoded D50 white point, chosen so their integers sum
//! to it exactly (see [`COLORANT_R`]); the tone curves are exact powers of two
//! or linear ramps; the CLUTs are simple documented functions of their grid
//! indices. Every one of those numbers was chosen to be *checkable by hand*,
//! not to describe any real device. A fixture from this corpus is evidence
//! about **structure** — that bytes in a stated layout decode to stated values
//! — and never about colour. `docs/NUMERIC_CLAIMS.md` classes exist for the
//! other thing; none of them is earned here.
//!
//! The single exception is flagged at its site: `v4-rgb-para-type3` carries the
//! sRGB-shaped ICC type-3 parameters, whose provenance is **one source**
//! (lcms2), tier `impl_crosscheck`, because IEC 61966-2-1 is paywalled and was
//! not obtained. That fixture is still not an sRGB profile — its primaries are
//! the arbitrary split — and must not be described as one.

use crate::bytes::{Buf, D50_ENCODED, general_lab_ab, general_lab_l, legacy_lab_ab, legacy_lab_l};
use crate::profile::{ProfileSpec, Tag, TagBody};
use crate::tags::{self, AbClut, LutAb, LutAbKind, Mft1, Mft2, Ncl2Entry};

/// What a fixture is for: reading, or reporting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    WellFormed,
    Malformed,
}

impl Category {
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::WellFormed => "well-formed",
            Self::Malformed => "malformed",
        }
    }
}

/// One fixture: its name, what it contains, and what a conformant consumer
/// must do with it.
///
/// `expect` is written **before** the fixture is run against anything. That
/// ordering is the point: an expectation recorded after the fact is a
/// description of the implementation, not a test of it
/// (`CLAUDE.md` rule 3, and `docs/TOLERANCES.md` §0 on tuned numbers).
pub struct Recipe {
    pub name: &'static str,
    pub category: Category,
    /// The tag types and structures the fixture covers.
    pub covers: &'static str,
    /// What the bytes are, in one sentence.
    pub what: &'static str,
    /// The stated, pre-run expectation.
    pub expect: &'static str,
    pub build: fn() -> Vec<u8>,
}

// ===========================================================================
// Shared material
// ===========================================================================

/// The copyright string every fixture carries.
///
/// `LEGAL.md` §3 category (a): a profile this project authored byte by byte
/// carries no third-party content and no restriction. Saying so *inside* the
/// file means a fixture that escapes the repository still states its own
/// provenance.
const CPRT: &str = "Synthetic fixture authored by the iccce project. MIT. No third-party content.";

/// ★ The three colorant columns, as **encoded `s15Fixed16` integers**, chosen
/// so that
///
/// ```text
/// rXYZ + gXYZ + bXYZ  ==  the encoded PCS illuminant, component by component
/// ```
///
/// exactly, with no rounding anywhere:
///
/// | | X | Y | Z |
/// |---|---:|---:|---:|
/// | red   | 31 595 | 16 384 | 6 757 |
/// | green | 15 797 | 32 768 | 6 758 |
/// | blue  | 15 798 | 16 384 | 40 546 |
/// | **sum** | **63 190** = `F6D6h` | **65 536** = `00010000h` | **54 061** = `D32Dh` |
///
/// **What this is.** A *structural* invariant — device (1, 1, 1) reaches the
/// PCS white point exactly, which is the property a matrix/TRC profile is
/// supposed to have and the one a fixture can meaningfully assert. It is
/// asserted in [`tests::colorants_sum_to_the_encoded_white_point`].
///
/// **What this is not.** These are not anybody's primaries. They are an
/// arbitrary split (X: ½/¼/¼, Y: ¼/½/¼, Z: ⅛/⅛/¾, with the odd unit given to
/// whichever channel keeps the sum exact) and they describe no device. The
/// alternative — using real sRGB primaries adapted to D50 — was rejected
/// deliberately: `CLAUDE.md` rule 2 forbids writing colour maths from memory,
/// the corpus holds sRGB's chromaticities at `impl_crosscheck` tier only, and
/// a Bradford adaptation computed here would be a numeric claim this role has
/// no business minting inside a fixture generator. A number nobody can mistake
/// for colorimetry is safer than a plausible one.
pub const COLORANT_R: [i32; 3] = [31_595, 16_384, 6_757];
/// See [`COLORANT_R`].
pub const COLORANT_G: [i32; 3] = [15_797, 32_768, 6_758];
/// See [`COLORANT_R`].
pub const COLORANT_B: [i32; 3] = [15_798, 16_384, 40_546];

/// `i / (n − 1)` as a fraction of the grid, for CLUT fills.
#[expect(
    clippy::cast_precision_loss,
    reason = "grid indices and sizes are small integers (< 256); no precision is at risk"
)]
fn frac(i: usize, n: usize) -> f64 {
    i as f64 / (n - 1) as f64
}

/// `desc` + `cprt` in their **v4** forms (`multiLocalizedUnicodeType`, D3).
fn v4_meta(desc: &str) -> Vec<Tag> {
    vec![
        Tag::own(b"desc", tags::mluc_en_us(desc)),
        Tag::own(b"cprt", tags::mluc_en_us(CPRT)),
    ]
}

/// `desc` + `cprt` in their **v2** forms (`textDescriptionType` + `textType`).
fn v2_meta(desc: &str) -> Vec<Tag> {
    vec![
        Tag::own(b"desc", tags::text_description(desc, true)),
        Tag::own(b"cprt", tags::text(CPRT)),
    ]
}

/// `wtpt` = the PCS illuminant.
///
/// Clause 8.2 requires `mediaWhitePointTag` in every class except DeviceLink.
/// Setting it to D50 means **no chromatic adaptation happens anywhere** in any
/// fixture, which keeps every measurable quantity a property of the encoding
/// and of nothing else — the same reason the difftest probe did it.
fn wtpt() -> Tag {
    Tag::own(b"wtpt", tags::xyz_raw(&[D50_ENCODED]))
}

// ===========================================================================
// (a) Well-formed: v4 matrix/TRC RGB, XYZ PCS, para type-0 TRCs
// ===========================================================================

/// Required tags for a three-component matrix-based Display profile: clause
/// 8.4.3 (`rXYZ` `gXYZ` `bXYZ` `rTRC` `gTRC` `bTRC`) plus clause 8.2's
/// `desc` `cprt` `wtpt`. `chad` is conditionally required and correctly absent
/// — the fixture's white point *is* the PCS adopted white.
///
/// Clause 8.3.3/8.4.3, verbatim: "Only the PCSXYZ encoding can be used with
/// matrix/TRC models." Hence `pcs = 'XYZ '`, and the `v4-rgb-matrix-trc-labpcs`
/// counter-fixture is *not* in this corpus — it would be a conformance
/// violation with no parser-level report attached, which is a validator
/// question and not Pass 2's.
pub fn v4_rgb_matrix_trc_spec() -> ProfileSpec {
    let mut tags_ = v4_meta("iccce synthetic v4 matrix/TRC RGB");
    tags_.push(wtpt());
    tags_.push(Tag::own(b"rXYZ", tags::xyz_raw(&[COLORANT_R])));
    tags_.push(Tag::own(b"gXYZ", tags::xyz_raw(&[COLORANT_G])));
    tags_.push(Tag::own(b"bXYZ", tags::xyz_raw(&[COLORANT_B])));
    // parametricCurveType funcType 0: Y = X^g, one parameter. g = 2,0 is
    // chosen because it is EXACTLY representable in s15Fixed16 (00020000h) —
    // a fixture whose curve is exact leaves nothing for a later disagreement
    // to hide behind. (Gamma 2,2 would encode as 00023333h ≈ 2,19999.)
    for sig in [b"rTRC", b"gTRC", b"bTRC"] {
        tags_.push(Tag::own(sig, tags::para(0, &[2.0])));
    }
    ProfileSpec {
        version: 0x0440_0000, // 4.4.0.0 — the edition ICC.1:2022 defines
        class: *b"mntr",
        color_space: *b"RGB ",
        pcs: *b"XYZ ",
        rendering_intent: 1, // media-relative colorimetric
        tags: tags_,
    }
}

fn v4_rgb_matrix_trc() -> Vec<u8> {
    v4_rgb_matrix_trc_spec().assemble()
}

/// The same shape with `parametricCurveType` **funcType 3** TRCs — the
/// five-parameter piecewise form `Y = (aX + b)^g` for `X ≥ d`, `Y = cX`
/// otherwise.
///
/// ★ **Sourcing, stated because it is weaker than everything else here.** The
/// parameters are the sRGB transfer function's, and the corpus file
/// `iec/iec__s__srgb.md` carries a sourcing warning in its first section: IEC
/// 61966-2-1 is **paywalled and was NOT obtained**, so the values come from
/// **one** source, lcms2's `Build_sRGBGamma` (MIT), at tier `impl_crosscheck`
/// — *not* cross-verified. They are written here as computed expressions
/// (`1.0 / 1.055`, not `0.947867…`) exactly as that file instructs, so the only
/// error introduced is the unavoidable `s15Fixed16` quantisation.
///
/// ★ **This is not an sRGB profile.** Its primaries are [`COLORANT_R`]'s
/// arbitrary split. It is a fixture that exercises the five-parameter `para`
/// layout and the linear-segment shape; describing it as sRGB would be exactly
/// the "plausible-looking" claim this project exists to prevent.
///
/// ★ **The lcms2 off-by-one applies when this fixture meets the oracle.** ICC
/// `funcType` 3 is lcms2 curve type **4**; passing 5 parameters to lcms2 type 3
/// reads only 4 of them.
fn v4_rgb_para_type3() -> Vec<u8> {
    let mut spec = v4_rgb_matrix_trc_spec();
    let srgb_shaped = [
        2.4,           // g
        1.0 / 1.055,   // a
        0.055 / 1.055, // b
        1.0 / 12.92,   // c
        0.04045,       // d
    ];
    for t in &mut spec.tags {
        if matches!(&t.sig, b"rTRC" | b"gTRC" | b"bTRC") {
            t.body = TagBody::Own(tags::para(3, &srgb_shaped));
        }
    }
    for t in &mut spec.tags {
        if &t.sig == b"desc" {
            t.body = TagBody::Own(tags::mluc_en_us(
                "iccce synthetic v4 RGB, para funcType 3 TRCs",
            ));
        }
    }
    spec.assemble()
}

// ===========================================================================
// (b) Well-formed: v2 matrix/TRC RGB with curv-table TRCs
// ===========================================================================

/// v2.4 three-component matrix-based Display profile with the **v2** metadata
/// types and `curveType` tables.
///
/// The TRCs are 9-entry linear ramps: per clause 10.6 the first entry is input
/// 0,0 and the last is 1,0 with uniform spacing and normative linear
/// interpolation between them, so a linear ramp **is** the identity response
/// expressed as a table. That makes the corpus carry the same transform in two
/// encodings — `count == 0` in `v2-gray-curv-identity`, a table here — which is
/// what lets a test distinguish "decoded the type" from "decoded the function".
///
/// `chad` is present as the identity; see [`tags::chad_identity`] for why that
/// is a stated judgement call rather than an oversight.
pub fn v2_rgb_matrix_trc_curv_spec() -> ProfileSpec {
    let mut tags_ = v2_meta("iccce synthetic v2 matrix/TRC RGB, curv tables");
    tags_.push(wtpt());
    tags_.push(Tag::own(b"chad", tags::chad_identity()));
    tags_.push(Tag::own(b"rXYZ", tags::xyz_raw(&[COLORANT_R])));
    tags_.push(Tag::own(b"gXYZ", tags::xyz_raw(&[COLORANT_G])));
    tags_.push(Tag::own(b"bXYZ", tags::xyz_raw(&[COLORANT_B])));
    for sig in [b"rTRC", b"gTRC", b"bTRC"] {
        tags_.push(Tag::own(sig, tags::curv_table(&tags::linear_ramp(9))));
    }
    ProfileSpec {
        version: 0x0240_0000, // 2.4.0.0
        class: *b"mntr",
        color_space: *b"RGB ",
        pcs: *b"XYZ ",
        rendering_intent: 1,
        tags: tags_,
    }
}

fn v2_rgb_matrix_trc_curv() -> Vec<u8> {
    v2_rgb_matrix_trc_curv_spec().assemble()
}

/// The same profile with `gTRC` and `bTRC` **aliased** onto `rTRC`'s data.
///
/// ★ Clause 7.3.1, verbatim: the tag table "may contain multiple tags
/// signatures that all reference the same tag data element offset … In such
/// cases, both the offset and size … shall be the same." Full aliasing is
/// **explicitly legal**; only *partial* overlap is illegal. `rTRC`/`gTRC`/
/// `bTRC` sharing one curve is the commonest real instance.
///
/// This is a **well-formed** fixture and must report zero malformations. It is
/// here because the corpus states the failure mode plainly — "a parser that
/// treats any offset collision as an error rejects conformant profiles" — and
/// because a *writer* that cannot share produces profiles 2–3× larger than the
/// tools everyone compares against.
fn v2_rgb_shared_trc() -> Vec<u8> {
    let mut spec = v2_rgb_matrix_trc_curv_spec();
    for t in &mut spec.tags {
        match &t.sig {
            b"gTRC" | b"bTRC" => t.body = TagBody::Alias(*b"rTRC"),
            b"desc" => {
                t.body = TagBody::Own(tags::text_description(
                    "iccce synthetic v2 RGB, gTRC/bTRC aliased onto rTRC",
                    true,
                ));
            }
            _ => {}
        }
    }
    spec.assemble()
}

// ===========================================================================
// Well-formed: the two remaining curveType cases, as monochrome profiles
// ===========================================================================

fn v2_gray(desc: &str, ktrc: Vec<u8>) -> Vec<u8> {
    // Clause 8.4.4: a monochrome Display profile requires `kTRC` beyond 8.2's
    // common set. Nothing else.
    let mut tags_ = v2_meta(desc);
    tags_.push(wtpt());
    tags_.push(Tag::own(b"kTRC", ktrc));
    ProfileSpec {
        version: 0x0240_0000,
        class: *b"mntr",
        color_space: *b"GRAY",
        pcs: *b"XYZ ",
        rendering_intent: 1,
        tags: tags_,
    }
    .assemble()
}

/// `curveType` `count == 1`: the `u8Fixed8` gamma shorthand — "the
/// highest-value trap in this file" per the corpus, because a consumer that
/// reads the entry as a table sample computes `value/65535` and crushes
/// everything to black.
///
/// Gamma is **2,0** (`0200h`), which is exactly representable; 2,2 is not
/// (`0233h` = 2,19921875) and would put an unnecessary approximation into a
/// fixture whose job is to be exact.
fn v2_gray_curv_gamma() -> Vec<u8> {
    v2_gray(
        "iccce synthetic v2 GRAY, kTRC curv count=1 (gamma 2,0)",
        tags::curv_gamma(2.0),
    )
}

/// `curveType` `count == 0`: the identity response, no data following.
///
/// The corpus names this the *dangerous* half of the pair — not because it is
/// hard, but because treating an empty curve as invalid rejects a perfectly
/// valid identity TRC, and that failure is quiet.
fn v2_gray_curv_identity() -> Vec<u8> {
    v2_gray(
        "iccce synthetic v2 GRAY, kTRC curv count=0 (identity)",
        tags::curv_identity(),
    )
}

// ===========================================================================
// (c) Well-formed: v2 CMYK, mft2 A2B0 + B2A0, Lab PCS
// ===========================================================================

/// The A2B0 CLUT: a CMYK → Lab table in which **only K matters**.
///
/// `L* = 100 · (1 − k/(P−1))`, `a* = b* = 0`, for every (C, M, Y). It is a
/// legal, dull, entirely hand-checkable transform, and choosing "dull" is
/// deliberate: a fixture whose expected values can be computed in one's head
/// is a fixture whose failures are diagnosable.
///
/// ★ Index order is the specification's: "the dimension corresponding to the
/// **first input channel varies least rapidly** and the dimension
/// corresponding to the **last input channel varies most rapidly**" (clause
/// 10.10). So C is the outer loop and K the inner one, and the `outputChan`
/// values for a node are contiguous. Getting this backwards produces a
/// channel-swapped image — loud, but only if something actually renders it.
///
/// ★ Values use the **legacy** PCSLAB encoding, unconditionally: clause 10.10
/// verbatim, "this tag uses the legacy 16-bit PCSLAB encoding … not the 16-bit
/// PCSLAB encoding defined in 6.3.4.2", with **no version test**. `L* = 100`
/// is therefore `FF00h` here and `FFFFh` in the `mAB ` fixture.
fn cmyk_to_lab_clut_legacy(points: u8) -> Vec<u16> {
    let p = usize::from(points);
    let mut v = Vec::with_capacity(p * p * p * p * 3);
    for _c in 0..p {
        for _m in 0..p {
            for _y in 0..p {
                for k in 0..p {
                    v.push(legacy_lab_l(100.0 * (1.0 - frac(k, p))));
                    v.push(legacy_lab_ab(0.0));
                    v.push(legacy_lab_ab(0.0));
                }
            }
        }
    }
    v
}

/// The B2A0 CLUT: the approximate inverse — `K = 1 − L*/100`, `C = M = Y = 0`.
/// L is the first (slowest) input dimension.
fn lab_to_cmyk_clut(points: u8) -> Vec<u16> {
    let p = usize::from(points);
    let mut v = Vec::with_capacity(p * p * p * 4);
    for l in 0..p {
        for _a in 0..p {
            for _b in 0..p {
                #[expect(
                    clippy::cast_possible_truncation,
                    clippy::cast_sign_loss,
                    reason = "the expression is bounded by 65535 and non-negative by construction"
                )]
                let k = ((1.0 - frac(l, p)) * 65535.0).round() as u16;
                v.extend_from_slice(&[0, 0, 0, k]);
            }
        }
    }
    v
}

/// v2.4 CMYK Output profile with `mft2` A2B0 and B2A0, small grids.
///
/// ★ **A conformance note that is a scope statement, not a defect.** Clause
/// 8.5.2 requires an N-component LUT-based **Output** profile to carry all six
/// `A2Bx`/`B2Ax` plus `gamt`; this fixture carries two tags. That is
/// deliberate — the fixture's job is the `lut16Type` byte layout, not clause 8
/// — and it is *not* a v2 conformance violation that can be asserted anyway:
/// `icc__s__required_tags.md` §7 records **A34**, that ICC.1:2022 states
/// requirements for 4.4.0.0 profiles and does not restate v2's, so "parse v2
/// profiles; do not declare them non-conformant against clause 8". A future
/// validator must not flag this file, and if it does, the validator is
/// applying v4 requirements to a v2 file.
fn v2_cmyk_mft2_lab() -> Vec<u8> {
    let a2b0 = Mft2 {
        input_chan: 4,
        output_chan: 3,
        clut_points: 3,
        pad: 0,
        // Clause 10.10: "The matrix shall be an identity matrix unless the
        // input is in the PCSXYZ colour space." The input here is CMYK.
        matrix: Mft2::IDENTITY,
        // Clause 10.10 / A22: a minimum of two and a maximum of 4 096 entries.
        // Two is the minimum, and with two there is nothing to interpolate, so
        // the table is exactly the identity and contributes no approximation.
        input_ent: 2,
        output_ent: 2,
        input_tables: [0x0000u16, 0xFFFF].repeat(4),
        clut: cmyk_to_lab_clut_legacy(3),
        output_tables: [0x0000u16, 0xFFFF].repeat(3),
    };
    let b2a0 = Mft2 {
        input_chan: 3,
        output_chan: 4,
        clut_points: 3,
        pad: 0,
        matrix: Mft2::IDENTITY,
        input_ent: 2,
        output_ent: 2,
        input_tables: [0x0000u16, 0xFFFF].repeat(3),
        clut: lab_to_cmyk_clut(3),
        output_tables: [0x0000u16, 0xFFFF].repeat(4),
    };
    let mut tags_ = v2_meta("iccce synthetic v2 CMYK, mft2 A2B0 + B2A0, Lab PCS");
    tags_.push(wtpt());
    tags_.push(Tag::own(b"A2B0", a2b0.encode()));
    tags_.push(Tag::own(b"B2A0", b2a0.encode()));
    ProfileSpec {
        version: 0x0240_0000,
        class: *b"prtr",
        color_space: *b"CMYK",
        pcs: *b"Lab ",
        rendering_intent: 1,
        tags: tags_,
    }
    .assemble()
}

/// v2.4 CMYK Output profile with an **`mft1`** A2B0.
///
/// ★ `lut8Type` is **not** in the legacy PCSLAB set — clause 6.3.4.2 NOTE 3
/// names `lut16Type` and `namedColor2Type` "and only those tag types". So this
/// CLUT uses the general 8-bit encoding, `L* = v × 100/255` and
/// `a*/b* = v − 128` with zero at `80h`.
///
/// ★ **Stated uncertainty.** `icc__s__pcs_encoding.md` flags that 8-bit
/// encoding as **A10, NOT SOURCED** — inferred from the structure and ICC's
/// `icLabFromPcs`, with the explicit warning "do not ship an 8-bit Lab path on
/// this alone". This fixture therefore encodes what A10 *implies* and says so:
/// a consumer that disagrees with these byte values is a **finding to settle
/// from the specification text**, not a fixture to patch. `L* = 100` is `FFh`
/// and `L* = 50` is `80h` = 50,196 — the half-integer code 127,5 that would
/// give exactly 50 does not exist, which is itself worth having in a fixture.
fn v2_cmyk_mft1_lab() -> Vec<u8> {
    let p = 3usize;
    let mut clut = Vec::with_capacity(p * p * p * p * 3);
    for _c in 0..p {
        for _m in 0..p {
            for _y in 0..p {
                for k in 0..p {
                    #[expect(
                        clippy::cast_possible_truncation,
                        clippy::cast_sign_loss,
                        reason = "bounded by 255 and non-negative by construction"
                    )]
                    let l = ((1.0 - frac(k, p)) * 255.0).round() as u8;
                    clut.extend_from_slice(&[l, 128, 128]);
                }
            }
        }
    }
    let a2b0 = Mft1 {
        input_chan: 4,
        output_chan: 3,
        clut_points: 3,
        pad: 0,
        matrix: Mft2::IDENTITY,
        input_tables: tags::ramp256().repeat(4),
        clut,
        output_tables: tags::ramp256().repeat(3),
    };
    let mut tags_ = v2_meta("iccce synthetic v2 CMYK, mft1 A2B0, Lab PCS");
    tags_.push(wtpt());
    tags_.push(Tag::own(b"A2B0", a2b0.encode()));
    ProfileSpec {
        version: 0x0240_0000,
        class: *b"prtr",
        color_space: *b"CMYK",
        pcs: *b"Lab ",
        rendering_intent: 1,
        tags: tags_,
    }
    .assemble()
}

// ===========================================================================
// (d) Well-formed: the D2 discriminator pair — mft2 Lab in v4 and in v2
// ===========================================================================

/// The `A2B0` of the discriminator pair: an `mft2` with a 2×2×2 CLUT whose
/// corners are chosen so the legacy and general PCSLAB decodings give visibly
/// different answers.
///
/// **These are the exact corner values `tools/difftest`'s `legacy_lab_probe`
/// used**, reproduced here because `tools/difftest/README.md` §10 asked for the
/// probe's profiles to be ported onto the generator once it existed, and
/// because keeping the numbers identical means the measurement recorded as
/// **DL-012** stays reproducible from a committed fixture rather than from a
/// binary in a git-ignored directory.
///
/// | corner (R,G,B) | value | legacy | general |
/// |---|---|---|---|
/// | 1,1,1 | `FF00 8000 8000` | 100,0000 0,0 0,0 | 99,6109 −0,4980 −0,4980 |
/// | 0,0,0 | `0000 8000 8000` | 0,0 0,0 0,0 | 0,0 −0,4980 −0,4980 |
/// | 1,0,0 | `8000 8000 8000` | 50,1961 0,0 0,0 | 50,0008 −0,4980 −0,4980 |
/// | 0,1,0 | `FF00 FF00 0000` | 100,0 127,0 −128,0 | 99,6109 126,0078 −128,0 |
///
/// `FF00h` is the legacy full-scale point — the value at which the two rules
/// are furthest apart, and the one whose misdecoding produces the ≈0,39 %
/// darkening of neutrals that DL-005 says hides below the perceptibility
/// anchor. Every probe input lands **exactly on a corner**, so no interpolation
/// happens and the number read back is a decoded table entry and nothing else.
fn probe_mft2() -> Vec<u8> {
    const MID: [u16; 3] = [0x8000, 0x8000, 0x8000];
    let mut clut = [MID; 8];
    clut[0b111] = [0xFF00, 0x8000, 0x8000];
    clut[0b000] = [0x0000, 0x8000, 0x8000];
    clut[0b100] = [0x8000, 0x8000, 0x8000];
    clut[0b010] = [0xFF00, 0xFF00, 0x0000];
    Mft2 {
        input_chan: 3,
        output_chan: 3,
        clut_points: 2,
        pad: 0,
        matrix: Mft2::IDENTITY,
        input_ent: 2,
        output_ent: 2,
        input_tables: [0x0000u16, 0xFFFF].repeat(3),
        clut: clut.into_iter().flatten().collect(),
        output_tables: [0x0000u16, 0xFFFF].repeat(3),
    }
    .encode()
}

/// The discriminator profile, parameterised only by the version word.
///
/// ★ **The v4 and v2 members of this pair are byte-identical except for the
/// four version bytes at header offset 8**, which is why both use the v2-era
/// `desc`/`cprt` types even in the v4 file. That is a deliberate,
/// *acknowledged* non-conformity in the v4 member: `textDescriptionType` is not
/// a v4 type. It is accepted because the fixture's entire purpose is to isolate
/// one variable, and a metadata type cannot plausibly reach the LUT decode
/// path. "Cannot plausibly" is not a measurement, so the probe closed that
/// objection with a fourth profile carrying proper `mluc` metadata; here the
/// same job is done by `v4-rgb-matrix-trc` and `v4-cmyk-mab-lab`, which are
/// `mluc`-carrying v4 files.
///
/// The byte-identity is asserted in
/// [`tests::the_discriminator_pair_differs_only_in_the_version_word`] — an
/// experiment whose apparatus is not shown to isolate its variable is not an
/// experiment.
fn probe_profile(version: u32) -> Vec<u8> {
    let mut tags_ = vec![Tag::own(b"A2B0", probe_mft2())];
    tags_.extend(v2_meta("iccce legacy-Lab discriminator"));
    tags_.push(wtpt());
    ProfileSpec {
        version,
        class: *b"scnr",
        color_space: *b"RGB ",
        pcs: *b"Lab ",
        rendering_intent: 1,
        tags: tags_,
    }
    .assemble()
}

fn v4_rgb_mft2_lab() -> Vec<u8> {
    probe_profile(0x0430_0000)
}

fn v2_rgb_mft2_lab() -> Vec<u8> {
    probe_profile(0x0210_0000)
}

// ===========================================================================
// Well-formed: the v4 LUT family — mAB and mBA
// ===========================================================================

/// The 3×4 matrix used by both `mAB `/`mBA ` fixtures: identity coefficients
/// with **non-zero offset terms** `e03 = 1/256`, `e13 = 2/256`, `e23 = 3/256`.
///
/// ★ The offsets are non-zero **on purpose**. The classic misread of this
/// element is to take 36 bytes (a 3×3) and stop, leaving the three offsets
/// unapplied — "a uniform colour cast that looks like a white-point problem",
/// plausible and diagnosed in the wrong place. A fixture whose offsets are zero
/// **cannot detect that misread**, because both readings then agree. These
/// three values are exactly representable (`00000100h`, `00000200h`,
/// `00000300h`), so a consumer's output either carries them or does not.
const MATRIX_3X4: [f64; 12] = [
    1.0,
    0.0,
    0.0, //
    0.0,
    1.0,
    0.0, //
    0.0,
    0.0,
    1.0, //
    1.0 / 256.0,
    2.0 / 256.0,
    3.0 / 256.0,
];

/// `gridPoints[16]` with the first `dims.len()` entries set and the rest zero
/// ("unused entries shall be set to 00h").
fn grid(dims: &[u8]) -> [u8; 16] {
    let mut g = [0u8; 16];
    g[..dims.len()].copy_from_slice(dims);
    g
}

/// v4.4 CMYK Output profile carrying `A2B0` as `lutAToBType` and `B2A0` as
/// `lutBToAType`.
///
/// ★ **The A2B0 CLUT grid is deliberately ragged: 5 × 4 × 3 × 2.** Per-dimension
/// grid sizes are the substantive advance of `mAB ` over `lut16Type`, whose
/// `clutPoints` is one byte applied to every dimension. A hypercubic fixture
/// would let a consumer that ignores the per-dimension array produce the right
/// answer by accident; a ragged one cannot.
///
/// ★ **Curve counts follow clause 10.12.2/4/6 and 10.13.2/4/6**, read directly
/// from the PDF — see [`tags::spec_curve_counts`] for the verbatim sentences.
/// For this profile that is:
///
/// | tag | in | out | B | M | A |
/// |---|---:|---:|---:|---:|---:|
/// | `A2B0` `mAB ` | 4 | 3 | 3 | 3 | 4 |
/// | `B2A0` `mBA ` | 3 | 4 | **3** | **3** | **4** |
///
/// The `mBA ` row is the one that matters. A reading of "A curves = inputChan,
/// B and M = outputChan" — the single blanket sentence the corpus carries —
/// gives (4, 4, 3) there and mis-parses the tag. The two readings agree
/// whenever `inputChan == outputChan`, so **this fixture is the smallest thing
/// that can tell them apart**, which is why it exists in this shape rather than
/// as a 3-in-3-out toy.
///
/// ★ PCSLAB values here use the **general** encoding (`L* = 100` → `FFFFh`),
/// because `mAB `/`mBA ` are not in clause 6.3.4.2 NOTE 3's legacy set. The
/// contrast with `v2-cmyk-mft2-lab`'s `FF00h` is the point of having both.
fn v4_cmyk_mab_lab() -> Vec<u8> {
    // --- A2B0: CMYK -> Lab, ragged grid, general PCSLAB encoding -----------
    let dims = [5u8, 4, 3, 2];
    let mut a2b_clut = Vec::new();
    for _c in 0..usize::from(dims[0]) {
        for _m in 0..usize::from(dims[1]) {
            for _y in 0..usize::from(dims[2]) {
                for k in 0..usize::from(dims[3]) {
                    a2b_clut.push(general_lab_l(100.0 * (1.0 - frac(k, usize::from(dims[3])))));
                    a2b_clut.push(general_lab_ab(0.0));
                    a2b_clut.push(general_lab_ab(0.0));
                }
            }
        }
    }
    let (b_n, m_n, a_n) = tags::spec_curve_counts(LutAbKind::AToB, 4, 3);
    let a2b0 = LutAb {
        kind: LutAbKind::AToB,
        input_chan: 4,
        output_chan: 3,
        b_curves: vec![tags::curv_identity(); usize::from(b_n)],
        matrix: Some(MATRIX_3X4),
        m_curves: vec![tags::curv_identity(); usize::from(m_n)],
        clut: Some(AbClut {
            grid_points: grid(&dims),
            precision: 2,
            data: a2b_clut,
        }),
        a_curves: vec![tags::curv_identity(); usize::from(a_n)],
    };

    // --- B2A0: Lab -> CMYK, 3x3x3 grid ------------------------------------
    let (b_n, m_n, a_n) = tags::spec_curve_counts(LutAbKind::BToA, 3, 4);
    let b2a0 = LutAb {
        kind: LutAbKind::BToA,
        input_chan: 3,
        output_chan: 4,
        b_curves: vec![tags::curv_identity(); usize::from(b_n)],
        matrix: Some(MATRIX_3X4),
        m_curves: vec![tags::curv_identity(); usize::from(m_n)],
        clut: Some(AbClut {
            grid_points: grid(&[3, 3, 3]),
            precision: 2,
            data: lab_to_cmyk_clut(3),
        }),
        a_curves: vec![tags::curv_identity(); usize::from(a_n)],
    };

    let mut tags_ = v4_meta("iccce synthetic v4 CMYK, mAB A2B0 + mBA B2A0, Lab PCS");
    tags_.push(wtpt());
    tags_.push(Tag::own(b"A2B0", a2b0.encode()));
    tags_.push(Tag::own(b"B2A0", b2a0.encode()));
    ProfileSpec {
        version: 0x0440_0000,
        class: *b"prtr",
        color_space: *b"CMYK",
        pcs: *b"Lab ",
        rendering_intent: 1,
        tags: tags_,
    }
    .assemble()
}

// ===========================================================================
// (g) Well-formed: v4 RGB mAB/mBA with a NON-ZERO, SLIGHTLY CHROMATIC BLACK
// ===========================================================================

/// The synthetic device's black, in `L*a*b*`. **This is the whole reason the
/// fixture exists**, so it is a named constant rather than three numbers
/// buried in a loop.
///
/// `L* = 20` — a non-zero black, which no profile in this corpus and no
/// profile on the authoring machine's system had. `a* = +4, b* = −3` — chroma
/// **5,0**, "slightly chromatic": the same order as a real ink black's
/// departure from neutral (`USWebCoatedSWOP`'s darkest colorant is 0,834 off
/// neutral) but several times larger, so that an estimator which **drops** the
/// chroma and one which **keeps** it differ by far more than the round-trip
/// noise that defeated the first attempt at this measurement.
///
/// **It was not chosen to make a prediction come true.** The corpus's
/// pre-registered magnitude band for the estimator divergence was 2–6 ΔE76;
/// chroma 5,0 sits inside it, which is a coincidence of the same reasoning
/// (both are "what a chromatic printer black looks like") and not a fit. What
/// the fixture is for is the *mechanism*, which is a yes/no question about
/// whether one implementation zeroes `a*` and `b*` and the other does not.
const SYNTH_BLACK_L: f64 = 20.0;
const SYNTH_BLACK_A: f64 = 4.0;
const SYNTH_BLACK_B: f64 = -3.0;

/// The forward colour model — `A2B`, device RGB in `0..1` to `L*a*b*`.
///
/// ```text
/// L* = 20 + 80·G
/// a* =  4·(1 − G) + 60·(R − G)
/// b* = −3·(1 − G) + 60·(B − G)
/// ```
///
/// ## Why this shape, and what it is not
///
/// It is **not colorimetry.** No instrument produced it and no device behaves
/// like it; like every colorant in this corpus it is an arbitrary but *exactly
/// stated* structural relation (see `fixtures/synthetic/README.md`). What it
/// has to be is four things, and it is each of them on purpose:
///
/// 1. **Non-zero, chromatic at device black.** `f(0,0,0) = (20, 4, −3)`. A
///    black-point estimator has something to find, and a chroma-dropping
///    estimator and a chroma-keeping one must disagree.
/// 2. **Neutral and full-scale at device white.** `f(1,1,1) = (100, 0, 0)`,
///    so the profile's white *is* the PCS adopted white and the media-relative
///    white scaling is the identity. A fixture whose white was not the adopted
///    white would put a second, unrelated difference into every comparison.
/// 3. **Multi-affine, therefore reproduced EXACTLY by multilinear CLUT
///    interpolation at any grid size.** There are no cross terms — each of
///    `L*`, `a*`, `b*` is a sum of terms each linear in one channel — so a
///    consumer that interpolates the CLUT correctly gets the closed form back
///    to encoding precision, at 9 nodes or at 65. **That makes the fixture an
///    instrument rather than a sample**: any disagreement between two
///    consumers is a disagreement about the ICC pipeline, not about
///    interpolation error, which is the confound `NA-006` names.
/// 4. **Invertible in closed form**, so the `B2A` side is the *exact* inverse
///    rather than a numerical approximation of one. See [`lab_to_rgb_chromatic_black`].
///
/// Out-of-range results are clamped to the encodable PCSLAB range, and the
/// clamp is the only non-affine thing in the file. It bites only for `a*`/`b*`
/// beyond ±128 at the far corners.
fn rgb_to_lab_chromatic_black(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    let l = SYNTH_BLACK_L + (100.0 - SYNTH_BLACK_L) * g;
    let a_star = SYNTH_BLACK_A * (1.0 - g) + 60.0 * (r - g);
    let b_star = SYNTH_BLACK_B * (1.0 - g) + 60.0 * (b - g);
    (
        l.clamp(0.0, 100.0),
        a_star.clamp(-128.0, 127.0),
        b_star.clamp(-128.0, 127.0),
    )
}

/// The exact inverse of [`rgb_to_lab_chromatic_black`] — `B2A`, `L*a*b*` to
/// device RGB in `0..1`, clamped to the unit cube.
///
/// ```text
/// G = (L* − 20) / 80
/// R = G + (a* −  4·(1 − G)) / 60
/// B = G + (b* +  3·(1 − G)) / 60
/// ```
///
/// **Why an exact inverse matters here.** Both black-point estimators under
/// test evaluate the round trip `BT(x) = A2B(B2A(x))`. If `B2A` were a
/// numerical inverse, `BT` would carry that inversion's error and the
/// estimators would be comparing their own noise. With the closed form,
/// `BT` is the identity everywhere the clamps do not bite — which makes the
/// *in-gamut* part of the round trip exactly straight, and that is
/// deliberate: it is the configuration in which the two implementations'
/// mid-range straightness tests both fire, so the fixture isolates **what the
/// short-circuit returns** rather than mixing it with a quadratic fit.
fn lab_to_rgb_chromatic_black(l: f64, a_star: f64, b_star: f64) -> (f64, f64, f64) {
    let g = (l - SYNTH_BLACK_L) / (100.0 - SYNTH_BLACK_L);
    let r = g + (a_star - SYNTH_BLACK_A * (1.0 - g)) / 60.0;
    let b = g + (b_star - SYNTH_BLACK_B * (1.0 - g)) / 60.0;
    (r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0))
}

/// v4.4 RGB Output profile carrying `A2B0`/`A2B1` as `lutAToBType` and
/// `B2A0`/`B2A1` as `lutBToAType`, whose **device black is non-zero and
/// slightly chromatic**.
///
/// ## ★★ Why this fixture exists, and why it is RGB rather than CMYK
///
/// `TOLERANCES.md` §3.5.7 and `tools/difftest/README.md` §17.6 item 5 have
/// asked since Pass 5 for "a synthetic v4 LUT fixture with a non-zero device
/// black", to discriminate the two black-point estimators where no real
/// profile could. **Reading lcms2's source at the pin changed what that
/// fixture has to be, in two ways that are worth stating in the file that
/// implements them.**
///
/// **First, it cannot be a perceptual-intent fixture.** Pass 5 asked for the
/// *v4 perceptual* arm. `cmssamp.c` L432–446 short-circuits: for a v4 profile
/// at perceptual or saturation lcms2 returns the fixed `cmsPERCEPTUAL_BLACK`
/// triple **without looking at the profile at all**, and
/// `Chain::estimate_dst_black` does the same with `bpc::PERCEPTUAL_BLACK`.
/// Two implementations that both return a constant cannot be discriminated by
/// any profile, however black its black. What such a fixture *can* do — and
/// what this one does, by carrying `A2B0`/`B2A0` as well — is **measure how
/// wrong that constant is**: the A41 triple is `L* ≈ 3,1` and this device's
/// black is `L* 20`, so the perceptual arm can be made to state its own error
/// instead of hiding it.
///
/// **Second, the intent that CAN discriminate is media-relative, and the
/// colour space must not be an ink space.** At relative colorimetric lcms2
/// picks its `InitialLab` through `cmsDetectBlackPoint`, which branches
/// (`cmssamp.c` L370–374):
///
/// ```text
/// if (Intent == INTENT_RELATIVE_COLORIMETRIC &&
///     cmsGetDeviceClass(hProfile) == cmsSigOutputClass &&
///     isInkColorspace(cmsGetColorSpace(hProfile)))
///     return BlackPointUsingPerceptualBlack(BlackPoint, hProfile);
/// ```
///
/// `BlackPointUsingPerceptualBlack` **forces `a* = b* = 0`** (L174). So on a
/// CMYK output profile — `USWebCoatedSWOP`, the only real LUT profile in
/// reach — lcms2's detected black is *neutral*, ISO/CD 18619 4.2.3's is
/// neutral, and the prediction's mechanism ("ISO drops the chroma, lcms2
/// retains it") **cannot be exercised at all**. Pass 5c measured exactly that
/// and recorded it.
///
/// An **RGB** output profile is not an ink colorspace, so the branch is not
/// taken and lcms2 falls through to `BlackPointAsDarkerColorant`, which clips
/// `L*` to 50 and **keeps `a*` and `b*`**. That is the one configuration in
/// which the two estimators genuinely differ in chroma — and no profile on the
/// authoring machine had it. **This fixture is the instrument for the claim,
/// and the claim is why the fixture is RGB.**
///
/// ## What a conformant consumer must produce
///
/// * `A2B1` evaluated at device `(0,0,0)` is `Lab(20, 4, −3)` to encoding
///   precision (the general 16-bit PCSLAB encoding of 6.3.4.2, so ±0,002 in
///   `L*` and ±0,002 in `a*`/`b*`).
/// * `B2A1(A2B1(x)) = x` for every `x` whose image is inside the encodable
///   PCSLAB range — the model is affine and its inverse is exact, so the only
///   residue is the two encodings.
/// * `mAB `: `A = inputChan = 3`, `M = B = outputChan = 3` (10.12.2/4/6);
///   `mBA `: `B = M = inputChan = 3`, `A = outputChan = 3` (10.13.2/4/6).
///   Both are square here, so — unlike `v4-cmyk-mab-lab` — this fixture
///   **cannot** catch GP-001, and that is stated rather than hoped.
/// * Both `mAB ` and `mBA ` use the `A, CLUT, B` element combination
///   (10.12.1 / 10.13.1): **no matrix and no M curves**, `offsetMat` and
///   `offsetM` are `0`. The sibling CMYK fixture exercises the matrix path
///   with non-zero offsets; this one deliberately does not, so that a
///   black-point measurement made with it has no 3×4 offset in the chain to
///   attribute anything to.
///
/// ## ★★ What this fixture CANNOT see — `InitialLab` and `outRamp[first]` are
/// the same number here (FINDING GP-002, README §4.1)
///
/// **Read this before pruning, regenerating or "simplifying" this fixture, and
/// before quoting any black-point number measured with it.**
///
/// ISO/CD 18619 **4.2.5.4** distinguishes two quantities: `InitialLab` (4.2.2.2's
/// darkest device **vertex**, neutralised) and `outRamp[first]` (the floor of
/// the monotonised round-trip ramp). On 2026-08-12 `iccce-cmm` was found to be
/// returning the second where the clause says the first, and the correction
/// (`fd34a44`) moved the `USWebCoatedSWOP` arm by **4,717 441 `L*`**.
///
/// **On this fixture it moved nothing at all, because here both quantities are
/// `L* 20`.** That is not an authoring mistake — it follows from three
/// properties chosen above for good reasons:
///
/// 1. the model is **affine**, so the ramp is straight;
/// 2. [`lab_to_rgb_chromatic_black`] is its **exact** inverse, so the in-gamut
///    round trip is the identity;
/// 3. `SYNTH_BLACK_L` is the image of the **darkest vertex**, and dark
///    out-of-gamut inputs clamp onto that vertex.
///
/// So the round trip's floor *is* the neutralised darkest vertex. On a real ink
/// set none of the three holds exactly, which is why the vendor profile
/// separates the two candidates and this one does not.
///
/// **Consequence, stated plainly:** `pass5c/synthetic/*` has **zero power** on
/// the 4.2.5.4 question. It would stay green through a full reversion of that
/// correction. The only differential evidence is `pass5c/swop/*`, whose profile
/// is **category (c)** — never committed, absent on any machine without the
/// Windows colour directory, where those rows **skip**. The harness now emits
/// this as a machine-readable `ZERO-SEPARATION` verdict on the row itself
/// (`tools/difftest/src/lib.rs`, `Separation`), so the hole is countable rather
/// than merely documented.
///
/// **What this fixture is still the only instrument for** — and why it must not
/// be deleted as "the arm that proves nothing" — is the branch question above:
/// it is the only profile in reach that reaches `BlackPointAsDarkerColorant`,
/// and its `5,000 000 ΔE76` is *authored chroma*, evidence for the **mechanism**
/// and for nothing else.
///
/// **If a fixture with distinct values is wanted**, the cheapest construction
/// that separates them without touching anything measured here is a `B2A` whose
/// dark clamp cannot reach the darkest vertex — a floor on `G` for **every**
/// input, not only out-of-gamut ones, which lifts the round-trip floor above
/// `SYNTH_BLACK_L` while leaving `A2B(0,0,0)` alone. It belongs in a **new**
/// recipe, not in this one: changing these bytes would move
/// `NUMERIC_CLAIMS.md` NC-166's companion device figure (`5,725×10⁻²`) and
/// several filed statements that are true of *this* fixture.
///
/// ## What it deliberately does not do
///
/// The grid is **hypercubic** (9 per axis) rather than ragged: raggedness is
/// `v4-cmyk-mab-lab`'s job, and mixing the two would mean a black-point
/// disagreement and a grid-parsing disagreement could not be told apart.
/// Clause 8.4's full required-tag set for an Output profile is **not**
/// complete — no `gamt`, no `A2B2`/`B2A2` — exactly as in the sibling recipe;
/// the fixture is well-formed for the path under test and its manifest says
/// so.
fn v4_rgb_mab_chromatic_black() -> Vec<u8> {
    /// Nodes per axis, both directions. 9 is the smallest odd grid that puts
    /// nodes on eighths (so the CLUT's own lattice is legible in a hex dump)
    /// and it is more than enough: the model is multi-affine and multilinear
    /// interpolation reproduces it exactly at any size.
    const N: usize = 9;

    // --- A2B: RGB -> Lab ----------------------------------------------------
    let mut a2b_clut = Vec::with_capacity(N * N * N * 3);
    for ri in 0..N {
        for gi in 0..N {
            for bi in 0..N {
                let (l, a_star, b_star) =
                    rgb_to_lab_chromatic_black(frac(ri, N), frac(gi, N), frac(bi, N));
                a2b_clut.push(general_lab_l(l));
                a2b_clut.push(general_lab_ab(a_star));
                a2b_clut.push(general_lab_ab(b_star));
            }
        }
    }
    let (b_n, _m_n, a_n) = tags::spec_curve_counts(LutAbKind::AToB, 3, 3);
    let a2b = LutAb {
        kind: LutAbKind::AToB,
        input_chan: 3,
        output_chan: 3,
        b_curves: vec![tags::curv_identity(); usize::from(b_n)],
        // A, CLUT, B — 10.12.1's third permitted combination.
        matrix: None,
        m_curves: Vec::new(),
        clut: Some(AbClut {
            grid_points: grid(&[N as u8, N as u8, N as u8]),
            precision: 2,
            data: a2b_clut,
        }),
        a_curves: vec![tags::curv_identity(); usize::from(a_n)],
    };

    // --- B2A: Lab -> RGB ----------------------------------------------------
    // The CLUT's input axes are the ENCODED PCSLAB axes, so node (li, ai, bi)
    // stands for L* = 100·li/(N−1), a* = b* = −128 + 255·ai/(N−1). That is the
    // general encoding of 6.3.4.2 read backwards, and getting it wrong is the
    // classic B2A authoring error: a fixture whose B2A axes assumed [-128,127]
    // mapped linearly onto [0,1] with a different span would still parse and
    // would be silently wrong everywhere except the neutral axis.
    let mut b2a_clut = Vec::with_capacity(N * N * N * 3);
    for li in 0..N {
        for ai in 0..N {
            for bi in 0..N {
                let l = 100.0 * frac(li, N);
                let a_star = -128.0 + 255.0 * frac(ai, N);
                let b_star = -128.0 + 255.0 * frac(bi, N);
                let (r, g, b) = lab_to_rgb_chromatic_black(l, a_star, b_star);
                #[expect(
                    clippy::cast_possible_truncation,
                    clippy::cast_sign_loss,
                    reason = "each value is clamped to 0..1 above, so ×65535 is 0..=65535"
                )]
                {
                    b2a_clut.push((r * 65535.0).round() as u16);
                    b2a_clut.push((g * 65535.0).round() as u16);
                    b2a_clut.push((b * 65535.0).round() as u16);
                }
            }
        }
    }
    let (b_n, _m_n, a_n) = tags::spec_curve_counts(LutAbKind::BToA, 3, 3);
    let b2a = LutAb {
        kind: LutAbKind::BToA,
        input_chan: 3,
        output_chan: 3,
        b_curves: vec![tags::curv_identity(); usize::from(b_n)],
        matrix: None,
        m_curves: Vec::new(),
        clut: Some(AbClut {
            grid_points: grid(&[N as u8, N as u8, N as u8]),
            precision: 2,
            data: b2a_clut,
        }),
        a_curves: vec![tags::curv_identity(); usize::from(a_n)],
    };

    let mut tags_ = v4_meta(
        "iccce synthetic v4 RGB, mAB/mBA, non-zero slightly chromatic black Lab(20 4 -3)",
    );
    tags_.push(wtpt());
    // Perceptual and media-relative carry the SAME tables. That is not
    // laziness: it makes the perceptual arm's use of the fixed A41 black
    // measurable against this profile's real black without a second colour
    // model to attribute anything to.
    let a2b_bytes = a2b.encode();
    let b2a_bytes = b2a.encode();
    tags_.push(Tag::own(b"A2B0", a2b_bytes.clone()));
    tags_.push(Tag::own(b"A2B1", a2b_bytes));
    tags_.push(Tag::own(b"B2A0", b2a_bytes.clone()));
    tags_.push(Tag::own(b"B2A1", b2a_bytes));
    ProfileSpec {
        version: 0x0440_0000,
        class: *b"prtr",
        color_space: *b"RGB ",
        pcs: *b"Lab ",
        rendering_intent: 1,
        tags: tags_,
    }
    .assemble()
}

// ===========================================================================
// (f) Well-formed: ncl2 named-colour profile
// ===========================================================================

/// v2.4 NamedColor profile: clause 8.9 requires `ncl2` beyond 8.2's common
/// set.
///
/// ★ **`pcsCoords` use the legacy 16-bit PCSLAB encoding in a profile of ANY
/// version** — clause 10.17 verbatim: "For colour values that are in PCSLAB,
/// this tag uses the legacy 16-bit PCSLAB encoding defined in 10.8 [*sic* —
/// 10.10] (Tables 42 and 43), **not** the 16-bit PCSLAB encoding that is
/// defined in 6.3.4.2." Table 66 adds: "Only PCSXYZ and legacy 16-bit PCSLAB
/// encodings are permitted. **PCS values shall be relative colorimetric.**"
/// That closes the corpus's **A26**.
///
/// ★ Why this fixture matters more than its size suggests: `ncl2` is the one
/// place the ≈0,4 % legacy/general error is least acceptable, because spot
/// colours *are* brand matching — quiet symptom, expensive consequence — and
/// because `docs/ROADMAP.md` records that iccce's `ncl2` legacy-Lab handling
/// "was **not** tested behaviourally against lcms2; that case rests on a source
/// reading". This is the fixture that lets that stop being true.
///
/// Every `L*` chosen is an exact multiple of 0,625, so `L × 652,8` is an exact
/// integer and no rounding enters the fixture; every `a*`/`b*` is an integer,
/// which `(ab + 128) × 256` always encodes exactly.
///
/// Names are prefixed and suffixed per Table 66 — the full name is
/// `prefix + rootName + suffix` concatenated, and **is not stored whole
/// anywhere**, which is a decoding trap worth having a fixture for.
fn v2_ncl2_named() -> Vec<u8> {
    let entries = vec![
        Ncl2Entry {
            root: "Neutral 100",
            pcs: [legacy_lab_l(100.0), legacy_lab_ab(0.0), legacy_lab_ab(0.0)],
            device: vec![0, 0, 0, 0],
        },
        Ncl2Entry {
            root: "Neutral 50",
            pcs: [legacy_lab_l(50.0), legacy_lab_ab(0.0), legacy_lab_ab(0.0)],
            device: vec![0, 0, 0, 0x8000],
        },
        Ncl2Entry {
            root: "Neutral 0",
            pcs: [legacy_lab_l(0.0), legacy_lab_ab(0.0), legacy_lab_ab(0.0)],
            device: vec![0, 0, 0, 0xFFFF],
        },
        Ncl2Entry {
            root: "Chroma a+64 b-64",
            pcs: [
                legacy_lab_l(50.0),
                legacy_lab_ab(64.0),
                legacy_lab_ab(-64.0),
            ],
            device: vec![0xFFFF, 0, 0, 0],
        },
    ];
    let mut tags_ = v2_meta("iccce synthetic v2 NamedColor, ncl2 with legacy Lab pcsCoords");
    tags_.push(wtpt());
    tags_.push(Tag::own(
        b"ncl2",
        // nDeviceCoords must equal the header's device channel count — 4 for
        // CMYK. A mismatch is a malformation the consumer can only detect with
        // the header in hand, which is why `iccce-profile` exposes it as a
        // caller-invoked check rather than a decode-time one.
        tags::ncl2(0, 4, "ICCCE ", " (synthetic)", &entries),
    ));
    ProfileSpec {
        version: 0x0240_0000,
        class: *b"nmcl",
        color_space: *b"CMYK",
        pcs: *b"Lab ",
        rendering_intent: 1,
        tags: tags_,
    }
    .assemble()
}

// ===========================================================================
// (e) Malformed — one named defect each
// ===========================================================================

/// Replace one tag's data in the v4 base, keeping everything else identical.
fn v4_base_with(sig: &[u8; 4], data: Vec<u8>) -> Vec<u8> {
    let mut spec = v4_rgb_matrix_trc_spec();
    let mut replaced = false;
    for t in &mut spec.tags {
        if &t.sig == sig {
            t.body = TagBody::Own(data.clone());
            replaced = true;
        }
    }
    assert!(replaced, "tag not present in the base profile");
    spec.assemble()
}

/// Add a tag to the v4 base.
fn v4_base_plus(sig: &[u8; 4], data: Vec<u8>) -> Vec<u8> {
    let mut spec = v4_rgb_matrix_trc_spec();
    spec.tags.push(Tag::own(sig, data));
    spec.assemble()
}

// --- header identity ------------------------------------------------------

/// Offset 36 is not `'acsp'`. Per `icc__s__header.md` the magic is **the only
/// reliable format check** — `size`, `cmmId` and `creator` are all forgeable or
/// zero in the wild — so without it these bytes are not claimed to be an ICC
/// profile at all.
fn bad_magic() -> Vec<u8> {
    let mut p = v4_rgb_matrix_trc();
    p[36..40].copy_from_slice(b"nope");
    p
}

/// `header.size` declares 64 bytes more than the file has: truncated.
///
/// The opposite case is a *report*, not a refusal — see `trailing-bytes` — and
/// the asymmetry is deliberate: a container that pads an embedded profile is
/// normal, whereas bytes the header promises and cannot deliver mean the tail
/// of the file is unknowable.
fn truncated_declared_size() -> Vec<u8> {
    let mut p = v4_rgb_matrix_trc();
    let declared = crate::profile::read_u32(&p, 0) + 64;
    crate::profile::set_u32(&mut p, 0, declared);
    p
}

/// 100 bytes: shorter than the 128-byte header plus the 4-byte tag count.
/// "There is no short header."
fn too_short() -> Vec<u8> {
    let mut p = v4_rgb_matrix_trc();
    p.truncate(100);
    p
}

/// Major version 5 — iccMAX (ICC.2).
///
/// It must be **identified and refused by name**, not mistaken for corruption:
/// iccMAX reclaims the 28 bytes at 100–127 that ICC.1 reserves, so parsing it
/// with v4 semantics silently misreads them as `spectralPCS` / `mcs` /
/// `deviceSubClass`. `README.md` puts iccMAX out of scope; the refusal is how a
/// user learns that rather than getting plausible nonsense.
fn iccmax_version() -> Vec<u8> {
    let mut p = v4_rgb_matrix_trc();
    crate::profile::set_u32(&mut p, 8, 0x0500_0000);
    p
}

/// `tagCount = 0xFFFFFFFF`, which demands `132 + 12 × 4 294 967 295` bytes of
/// directory.
///
/// ★ **The allocation must be refused before it is attempted.** `tagCount` is
/// attacker-controlled and multiplies by 12 — the same class of bug as a PDF
/// xref-count overflow. A fixture that merely *reports* this is not enough; the
/// property under test is that nothing tries to reserve 51 GB first.
fn hostile_tag_count() -> Vec<u8> {
    let mut p = v4_rgb_matrix_trc();
    crate::profile::set_u32(&mut p, 128, 0xFFFF_FFFF);
    p
}

// --- header content -------------------------------------------------------

/// Header byte 100 is `01h`. Clause 7.2.19: bytes 100–127 are "reserved for
/// future ICC definition and **shall be set to zero**".
///
/// ★ This is where ICC's *own* published `icProfileHeader.h` disagrees with
/// ICC.1 — that header is the ICC.2/iccMAX superset and subdivides these 28
/// bytes into v5 fields. An implementer laying the most authoritative-looking
/// available source over a v4 profile reads reserved zeros as `spectralPCS` and
/// friends. For ICC.1, lcms2 is right and the bytes are reserved.
fn header_reserved_nonzero() -> Vec<u8> {
    let mut p = v4_rgb_matrix_trc();
    p[100] = 0x01;
    p
}

/// Eight bytes appended without updating `header.size`.
///
/// ★ **Report, never refuse.** An embedded profile in a PDF or TIFF is often
/// padded to a 4-byte boundary by the container, so `size` < stream length is
/// *normal* there. A parser that treats it as an error rejects a large class of
/// perfectly good embedded profiles — and `docs/ARCHITECTURE.md` cross-refers
/// the PDF corpus at §8.6 precisely because that is where iccce will meet them.
fn trailing_bytes() -> Vec<u8> {
    let mut p = v4_rgb_matrix_trc();
    p.extend_from_slice(&[0u8; 8]);
    p
}

/// Rendering intent `0x00010001`.
///
/// The low 16 bits say 1 (media-relative colorimetric); the high 16 are
/// non-zero, and clause 7.2.15 says the high half "shall be set to zero".
/// The value is chosen to probe ambiguity **A7** specifically: iccce reads all
/// 32 bits and reports rather than masking, so a masking implementation would
/// silently accept this and a reporting one must not.
fn rendering_intent_high_bits() -> Vec<u8> {
    let mut p = v4_rgb_matrix_trc();
    crate::profile::set_u32(&mut p, 64, 0x0001_0001);
    p
}

// --- tag table ------------------------------------------------------------

/// `wtpt`'s declared size is enlarged so its data extends past `header.size`.
fn tag_overrun() -> Vec<u8> {
    let mut p = v4_rgb_matrix_trc();
    let i = crate::profile::tag_index(&p, b"wtpt");
    let declared = crate::profile::read_u32(&p, 0);
    crate::profile::set_u32(&mut p, crate::profile::tag_size_field(i), declared);
    p
}

/// `wtpt`'s offset is 132 — inside the tag table itself. Clause 7.3 requires
/// `offset >= 132 + 12 × tagCount`.
fn tag_overlaps_table() -> Vec<u8> {
    let mut p = v4_rgb_matrix_trc();
    let i = crate::profile::tag_index(&p, b"wtpt");
    crate::profile::set_u32(&mut p, crate::profile::tag_offset_field(i), 132);
    p
}

/// `wtpt`'s offset is decremented by one.
///
/// Clause 7.3.4: "All tag data elements shall start on a 4-byte boundary …
/// the two least-significant bits of each tag data offset shall be zero."
/// Decrementing rather than incrementing keeps `offset + size` inside the
/// file, so **misalignment is the only defect** and no overrun is reported
/// alongside it.
fn tag_misaligned() -> Vec<u8> {
    let mut p = v4_rgb_matrix_trc();
    let i = crate::profile::tag_index(&p, b"wtpt");
    let off = crate::profile::read_u32(&p, crate::profile::tag_offset_field(i));
    crate::profile::set_u32(&mut p, crate::profile::tag_offset_field(i), off - 1);
    p
}

/// `wtpt`'s declared size is 4 — too small to hold even the 8-byte `icTagBase`
/// that every tag data element must begin with.
fn tag_too_small() -> Vec<u8> {
    let mut p = v4_rgb_matrix_trc();
    let i = crate::profile::tag_index(&p, b"wtpt");
    crate::profile::set_u32(&mut p, crate::profile::tag_size_field(i), 4);
    p
}

/// The four reserved bytes of `wtpt`'s `icTagBase` are `01010101h`; they
/// "shall be 0" (clause 7.3). The tag remains fully decodable, so this is a
/// *report alongside a decoded value* — the exact shape
/// `docs/ARCHITECTURE.md` §3.2 requires for a violation that leaves the layout
/// knowable.
fn tagbase_reserved_nonzero() -> Vec<u8> {
    let mut p = v4_rgb_matrix_trc();
    let (off, _) = crate::profile::tag_location(&p, b"wtpt");
    crate::profile::set_u32(&mut p, off as usize + 4, 0x0101_0101);
    p
}

/// Two `wtpt` entries in the directory, fully aliased onto one data element.
///
/// ★ The **aliasing** is legal (clause 7.3.1) and `v2-rgb-shared-trc` proves
/// iccce accepts it. What is malformed here is the **duplicate signature**:
/// "Duplicate tag signatures shall not be included." The pair of fixtures
/// therefore separates two things that look identical in a naive
/// implementation — same offset, different signatures (legal) versus same
/// signature twice (not) — and an implementation that conflates them fails
/// exactly one of the two.
///
/// Whether a duplicate is legal in **v2** is not sourced; the contiguity and
/// uniqueness rules are listed in ICC.1:2022's Foreword as changes introduced
/// in that edition, so v2 and v4.3 files in the wild may legitimately violate
/// them. iccce reports and names the edition; it does not reject (**A13**).
/// This fixture is a **v4.4** profile so the rule unambiguously applies.
fn duplicate_tag_signature() -> Vec<u8> {
    let mut spec = v4_rgb_matrix_trc_spec();
    spec.tags.push(Tag::alias(b"wtpt", b"wtpt"));
    spec.assemble()
}

// --- tag content ----------------------------------------------------------

/// `mluc` with `recordSize = 16`.
///
/// Clause 10.15 makes the field readable-and-honoured on purpose — "this minor
/// extra effort allows for future expansion of the record encoding … without
/// having to define a new tag type" — so a consumer that assumes 12 misparses
/// the day that expansion happens. lcms2 refuses outright
/// (`"multiLocalizedUnicodeType of len != 12 is not supported."`); iccce
/// report-and-refuses. **The refusal is the correct behaviour**, and this
/// fixture is what stops it from silently becoming an assumption later.
fn mluc_record_size_16() -> Vec<u8> {
    v4_base_with(b"desc", tags::mluc_en_us_with("record size 16", 16, 0))
}

/// `mluc` whose string offset is odd (29).
///
/// The offset indexes UTF-16 code units, so an odd value cannot be a valid
/// string start; lcms2 rejects with `if (Offset & 1) goto Error;`. One trailing
/// byte is appended to the tag so the record still fits inside it — otherwise
/// the fixture would carry **two** defects and could not tell you which one was
/// reported.
fn mluc_odd_offset() -> Vec<u8> {
    let mut t = tags::mluc_en_us_with("odd offset", 12, 1);
    t.push(0);
    v4_base_with(b"desc", t)
}

/// `curveType` declaring `count = 1000` with two entries present.
///
/// The count is attacker-controlled and multiplies by 2; the check must happen
/// **before** allocation, and the refusal must name the count.
fn curv_count_overflows_tag() -> Vec<u8> {
    let mut b = Buf::new();
    b.sig(b"curv").u32(0).u32(1000).u16(0).u16(0xFFFF);
    v4_base_with(b"rTRC", b.done())
}

/// `parametricCurveType` with `funcType = 9`.
///
/// Table 68 defines exactly five (0–4). Clause 10.18 also declares complex or
/// undefined parameter combinations **explicitly undefined**, which is a
/// stronger position than silence — so an unknown function type is a thing to
/// report and keep raw, not to guess at. The single parameter also makes the
/// parameter count unverifiable, which is itself part of the report.
fn para_unknown_functype() -> Vec<u8> {
    v4_base_with(b"rTRC", tags::para(9, &[2.0]))
}

/// Replace one tag's data in the **v2** base, keeping everything else
/// identical. Used where the defect belongs to a v2-era tag type, so that the
/// fixture does not have to introduce a second, unrelated irregularity (a
/// `textType` `cprt` in a v4 profile) just to carry it.
fn v2_base_with(sig: &[u8; 4], data: Vec<u8>) -> Vec<u8> {
    let mut spec = v2_rgb_matrix_trc_curv_spec();
    let mut replaced = false;
    for t in &mut spec.tags {
        if &t.sig == sig {
            t.body = TagBody::Own(data.clone());
            replaced = true;
        }
    }
    assert!(replaced, "tag not present in the v2 base profile");
    spec.assemble()
}

/// `textType` containing `E9h` (Latin-1 'é').
///
/// The type is **7-bit ASCII**; bytes ≥ 0x80 are a malformation to report, not
/// to decode. "Do not assume UTF-8" — guessing an encoding is a repair, and the
/// parser does not repair.
///
/// Built on the **v2** base, whose `cprt` is already a `textType`, so the
/// non-ASCII byte is the only irregularity. (An earlier draft appended a
/// second `cprt` to the v4 base and thereby produced a duplicate-signature
/// malformation as well — two defects in one fixture, which is precisely what
/// this corpus forbids.)
fn text_not_ascii() -> Vec<u8> {
    v2_base_with(b"cprt", tags::text_raw(b"caf\xE9\x00"))
}

/// `textType` with no terminating NUL. Occurs in the wild; the corpus directs
/// trusting `size`, not the NUL, and reporting the absence.
fn text_unterminated() -> Vec<u8> {
    v2_base_with(b"cprt", tags::text_raw(b"no terminator here"))
}

/// A v2 profile whose `desc` stops before the 67-byte Macintosh ScriptCode
/// block.
///
/// ★ **This is the one malformation in the corpus that was found in the wild
/// first.** `docs/ROADMAP.md`'s machine-wide sweep of 40 profiles reported
/// exactly this, four times — `ewgray18.icm`, `ewgray22.icm`, `ewrgb18.icm`,
/// `ewsrgb.icm`, the EIZO v2 profiles this machine ships — and the corpus had
/// predicted that the Mac block is "the most frequently malformed structure in
/// real v2 profiles". A fixture makes that finding a **regression test** that
/// survives the machine being reimaged, which a sweep result in a document does
/// not.
fn desc_short_mac_block() -> Vec<u8> {
    let mut spec = v2_rgb_matrix_trc_curv_spec();
    for t in &mut spec.tags {
        if &t.sig == b"desc" {
            t.body = TagBody::Own(tags::text_description(
                "iccce synthetic v2, desc Mac ScriptCode block absent",
                false,
            ));
        }
    }
    spec.assemble()
}

/// An `mft2` header claiming `inputChan = 8`, `clutPoints = 255` — a CLUT of
/// `255^8 × 3` samples — with a few bytes of data behind it.
///
/// ★ `255^8` is about 1,7 × 10^19: it **overflows a `u64`** when multiplied out
/// with the output channels, which is why the size must be computed in a
/// widened type and compared against the tag's actual length **before**
/// allocating. Refusing this is not politeness, it is the difference between a
/// parse error and an out-of-memory abort on attacker-controlled input.
fn mft2_clut_size_exceeds_tag() -> Vec<u8> {
    let mut b = Buf::new();
    b.sig(b"mft2").u32(0).u8(8).u8(3).u8(255).u8(0);
    for m in Mft2::IDENTITY {
        b.s15(m);
    }
    b.u16(2).u16(2).zeros(16);
    v4_base_plus(b"A2B0", b.done())
}

/// An `mft2` whose pad byte at offset 11 is `01h`; it "shall be zero".
/// The tag is otherwise valid and fully decodable, so this must be an issue
/// reported *alongside* the decoded LUT.
fn mft2_pad_nonzero() -> Vec<u8> {
    let mut lut = Mft2 {
        input_chan: 3,
        output_chan: 3,
        clut_points: 2,
        pad: 1, // <- the defect
        matrix: Mft2::IDENTITY,
        input_ent: 2,
        output_ent: 2,
        input_tables: [0x0000u16, 0xFFFF].repeat(3),
        clut: vec![0x8000; 8 * 3],
        output_tables: [0x0000u16, 0xFFFF].repeat(3),
    };
    lut.pad = 1;
    v4_base_plus(b"A2B0", lut.encode())
}

/// An `mAB ` CLUT with `precision = 3`.
///
/// Clause 10.12.3, Table 46: "Shall be either 01h or 02h." With any other value
/// the **sample width is unknowable**, so there is no partial result to be
/// tempted by — the tag must be refused rather than decoded on a guess.
fn mab_clut_precision_3() -> Vec<u8> {
    let t = LutAb {
        kind: LutAbKind::AToB,
        input_chan: 3,
        output_chan: 3,
        b_curves: vec![tags::curv_identity(); 3],
        matrix: None,
        m_curves: vec![],
        clut: Some(AbClut {
            grid_points: grid(&[2, 2, 2]),
            precision: 3, // <- the defect
            data: vec![0x8000; 8 * 3],
        }),
        a_curves: vec![tags::curv_identity(); 3],
    };
    v4_base_plus(b"A2B0", t.encode())
}

/// An `mAB ` whose second B curve is four bytes of `'curv'` and nothing else.
///
/// ★ **Curve chains fail positionally.** The elements have no count field and
/// no per-curve offsets, so curve *n* must be parsed to find curve *n+1*, and
/// one malformed element makes everything after it **unreachable rather than
/// merely wrong**. The useful report is therefore *which element and at what
/// byte* — a generic "short data" would leave the reader to find it.
///
/// ★ **The break does not surface at the element that is broken, and the
/// reason is worth having a fixture for.** Walking the bytes of this tag by
/// hand:
///
/// | byte | content | read as |
/// |---:|---|---|
/// | 32–43 | `'curv'` `00000000` `00000000` | element 0, count 0, identity |
/// | 44–47 | `'curv'` | element 1's signature |
/// | 48–51 | *element 2's* `'curv'` signature | element 1's **reserved** — non-zero |
/// | 52–55 | *element 2's* reserved, `00000000` | element 1's **count = 0** |
/// | 56–59 | *element 2's* count, `00000000` | element 2's **signature** — not a curve type |
///
/// The truncated element **swallows the following element's header**, comes
/// out looking like a valid identity curve, and the chain breaks one element
/// later: **element 2 at byte 56**. That cascade is exactly why the report has
/// to name a position rather than an element alone, and it is a property of
/// the byte layout, derivable from the table above without running anything.
/// (This crate's first draft predicted "element 1, byte 44" and was wrong; the
/// prediction was corrected against the *bytes*, not against the parser's
/// output.)
fn mab_curve_chain_broken() -> Vec<u8> {
    let t = LutAb {
        kind: LutAbKind::AToB,
        input_chan: 3,
        output_chan: 3,
        b_curves: vec![
            tags::curv_identity(),
            b"curv".to_vec(),
            tags::curv_identity(),
        ],
        matrix: None,
        m_curves: vec![],
        clut: None,
        a_curves: vec![],
    };
    v4_base_plus(b"A2B0", t.encode())
}

/// An `XYZType` with 16 bytes of payload: one whole `XYZNumber` and four bytes
/// left over.
///
/// Whole entries decode; the remainder is reported. Truncating silently would
/// hide a writer bug from the only layer that can disclose it.
fn xyz_trailing_bytes() -> Vec<u8> {
    let mut t = tags::xyz_raw(&[D50_ENCODED]);
    t.extend_from_slice(&[0, 0, 0, 1]);
    v4_base_with(b"wtpt", t)
}

// ===========================================================================
// The table
// ===========================================================================

/// Every recipe, in the order they are written to disk and listed in the
/// manifest.
#[must_use]
pub fn all() -> Vec<Recipe> {
    use Category::{Malformed, WellFormed};
    vec![
        // ---------------- well-formed ----------------
        Recipe {
            name: "v4-rgb-matrix-trc",
            category: WellFormed,
            covers: "mluc, XYZ, para(0)",
            what: "v4.4.0.0 mntr RGB, XYZ PCS, three-component matrix/TRC, para funcType 0 (g=2,0) TRCs",
            expect: "parses; 0 malformations; 9 tags; para funcType=0 params=2.000000; xyz colorants sum to D50",
            build: v4_rgb_matrix_trc,
        },
        Recipe {
            name: "v4-rgb-para-type3",
            category: WellFormed,
            covers: "para(3) five-parameter piecewise",
            what: "as v4-rgb-matrix-trc but TRCs are para funcType 3 with sRGB-shaped parameters (impl_crosscheck provenance; NOT an sRGB profile)",
            expect: "parses; 0 malformations; para funcType=3 with 5 parameters",
            build: v4_rgb_para_type3,
        },
        Recipe {
            name: "v2-rgb-matrix-trc-curv",
            category: WellFormed,
            covers: "desc, text, sf32(chad), curv table",
            what: "v2.4.0.0 mntr RGB, XYZ PCS, v2 metadata types, 9-entry curv table TRCs, identity chad",
            expect: "parses; 0 malformations; 10 tags; curve table n=9; sf32 n=9",
            build: v2_rgb_matrix_trc_curv,
        },
        Recipe {
            name: "v2-rgb-shared-trc",
            category: WellFormed,
            covers: "legal full tag aliasing (clause 7.3.1)",
            what: "as v2-rgb-matrix-trc-curv but gTRC and bTRC share rTRC's data element (same offset AND size)",
            expect: "parses; 0 malformations; the three TRC entries report identical offset and size",
            build: v2_rgb_shared_trc,
        },
        Recipe {
            name: "v2-gray-curv-gamma",
            category: WellFormed,
            covers: "curv count==1 (u8Fixed8 gamma)",
            what: "v2.4.0.0 mntr GRAY monochrome, kTRC as the gamma shorthand, gamma 2,0",
            expect: "parses; 0 malformations; curve gamma=2",
            build: v2_gray_curv_gamma,
        },
        Recipe {
            name: "v2-gray-curv-identity",
            category: WellFormed,
            covers: "curv count==0 (identity)",
            what: "v2.4.0.0 mntr GRAY monochrome, kTRC as the 12-byte identity curve",
            expect: "parses; 0 malformations; curve identity",
            build: v2_gray_curv_identity,
        },
        Recipe {
            name: "v2-cmyk-mft2-lab",
            category: WellFormed,
            covers: "mft2 (lut16Type), both directions, legacy PCSLAB",
            what: "v2.4.0.0 prtr CMYK, Lab PCS, mft2 A2B0 (4->3, 3^4 grid) and B2A0 (3->4, 3^3 grid)",
            expect: "parses; 0 malformations; lut16 in=4 out=3 clutPoints=3 and lut16 in=3 out=4 clutPoints=3; matrixIdentity=true",
            build: v2_cmyk_mft2_lab,
        },
        Recipe {
            name: "v2-cmyk-mft1-lab",
            category: WellFormed,
            covers: "mft1 (lut8Type), 256-entry tables",
            what: "v2.4.0.0 prtr CMYK, Lab PCS, mft1 A2B0 (4->3, 3^4 grid), general 8-bit Lab per A10",
            expect: "parses; 0 malformations; lut8 in=4 out=3 clutPoints=3; matrixIdentity=true",
            build: v2_cmyk_mft1_lab,
        },
        Recipe {
            name: "v4-rgb-mft2-lab",
            category: WellFormed,
            covers: "mft2 in a v4 profile — the D2 discriminator",
            what: "v4.3.0.0 scnr RGB, Lab PCS, mft2 A2B0 with the legacy_lab_probe's 2x2x2 corner values",
            expect: "parses; 0 malformations; lut16 in=3 out=3 clutPoints=2; byte-identical to v2-rgb-mft2-lab except header bytes 8..12",
            build: v4_rgb_mft2_lab,
        },
        Recipe {
            name: "v2-rgb-mft2-lab",
            category: WellFormed,
            covers: "the control arm of the discriminator pair",
            what: "v2.1.0.0 twin of v4-rgb-mft2-lab, differing ONLY in the version word",
            expect: "parses; 0 malformations; identical decoded summaries to v4-rgb-mft2-lab",
            build: v2_rgb_mft2_lab,
        },
        Recipe {
            name: "v4-cmyk-mab-lab",
            category: WellFormed,
            covers: "mAB and mBA, ragged CLUT grid, 3x4 matrix with non-zero offsets",
            what: "v4.4.0.0 prtr CMYK, Lab PCS, A2B0 as mAB (4->3, 5x4x3x2 grid) and B2A0 as mBA (3->4, 3x3x3 grid), general PCSLAB encoding",
            expect: "parses; 0 malformations; lutAToB in=4 out=3 B=3 M=3 A=4 grid=5x4x3x2 matrix=3x4; \
                     lutBToA in=3 out=4 B=3 M=3 A=4 per clauses 10.13.2/10.13.4/10.13.6. \
                     ★ AS OF 2026-08-11 ICCCE DOES NOT MEET THIS: it refuses the mBA tag with \
                     \"curve chain broken at element 3 (byte 68)\" because it counts B and M curves by \
                     outputChan for both tag types. The fixture is correct per the primary spec (and per \
                     lcms2's Type_LUTB2A_Read at the pin); see tools/gen-profiles/README.md \u{a7}5 \
                     \u{2014} FINDING GP-001. Do not change the fixture to match the parser.",
            build: v4_cmyk_mab_lab,
        },
        Recipe {
            name: "v4-rgb-mab-chromatic-black",
            category: WellFormed,
            covers: "mAB and mBA with the A,CLUT,B combination (no matrix, no M curves); a NON-ZERO, slightly chromatic device black",
            what: "v4.4.0.0 prtr RGB, Lab PCS, A2B0/A2B1 as mAB and B2A0/B2A1 as mBA (3->3, 9x9x9 grids, general PCSLAB encoding). Device black maps to Lab(20 4 -3), chroma 5.0; device white to Lab(100 0 0). The colour model is affine and the B2A CLUT is its EXACT closed-form inverse",
            expect: "parses; 0 malformations; lutAToB in=3 out=3 B=3 M=0 A=3 grid=9x9x9 matrix=absent; \
                     lutBToA in=3 out=3 B=3 M=0 A=3. A2B1 at device (0,0,0) is Lab(20 4 -3) to encoding \
                     precision; B2A1(A2B1(x)) = x wherever the PCS image is encodable. \
                     \u{2605} THE POINT OF THE FIXTURE: at MEDIA-RELATIVE this is an OUTPUT-class, \
                     NON-INK profile, so lcms2 reaches BlackPointAsDarkerColorant (cmssamp.c L370-374 \
                     does NOT fire) and RETAINS the black's chroma, while ISO/CD 18619 4.2.3 neutralises \
                     it - the one configuration in which the two estimators differ in chroma, and one \
                     that no profile on the authoring machine had. At PERCEPTUAL both implementations \
                     return the fixed A41 triple (L* ~ 3.1) WITHOUT reading the profile, so the fixture \
                     cannot discriminate them there; what it can do is measure how far that constant is \
                     from this device's real black of L* 20. \
                     \u{2605}\u{2605} WHAT THIS FIXTURE CANNOT SEE, READ BEFORE PRUNING OR QUOTING IT \
                     (FINDING GP-002, tools/gen-profiles/README.md \u{a7}4.1): ISO/CD 18619 4.2.5.4 \
                     distinguishes InitialLab from outRamp[first], and ON THIS FIXTURE THEY ARE BOTH \
                     L* 20 - the model is affine, the B2A is its exact inverse, and the black IS the \
                     darkest vertex, so the round trip's floor equals the neutralised vertex. A defect \
                     that returned the wrong one of the two moved USWebCoatedSWOP by 4.717441 L* on \
                     2026-08-12 and moved this fixture by EXACTLY ZERO. pass5c/synthetic/* would stay \
                     green through a full reversion of that correction; the only differential evidence \
                     is pass5c/swop/*, and that profile is category (c) - never committed, and those \
                     rows SKIP on a machine without the Windows colour directory. Do NOT delete this \
                     fixture on that account: it is still the only profile in reach that reaches \
                     lcms2's BlackPointAsDarkerColorant branch at all. A fixture with the two values \
                     DISTINCT would be a NEW recipe, not an edit to this one.",
            build: v4_rgb_mab_chromatic_black,
        },
        Recipe {
            name: "v2-ncl2-named",
            category: WellFormed,
            covers: "ncl2 with legacy Lab pcsCoords",
            what: "v2.4.0.0 nmcl CMYK/Lab NamedColor profile, 4 entries, prefix/suffix, 4 device coords each",
            expect: "parses; 0 malformations; ncl2 colors=4 deviceCoords=4",
            build: v2_ncl2_named,
        },
        // ---------------- malformed ----------------
        Recipe {
            name: "bad-magic",
            category: Malformed,
            covers: "ParseError::BadMagic",
            what: "v4 base with offset 36 set to 'nope'",
            expect: "REFUSED: not an ICC profile: magic at offset 36 is nope, expected 'acsp'; exit 1",
            build: bad_magic,
        },
        Recipe {
            name: "truncated-declared-size",
            category: Malformed,
            covers: "ParseError::Truncated",
            what: "v4 base whose header.size is 64 bytes larger than the file",
            expect: "REFUSED: truncated: header declares N bytes, only N-64 present; exit 1",
            build: truncated_declared_size,
        },
        Recipe {
            name: "too-short",
            category: Malformed,
            covers: "ParseError::TooShort",
            what: "the first 100 bytes of the v4 base",
            expect: "REFUSED: not an ICC profile: 100 bytes, minimum is 132; exit 1",
            build: too_short,
        },
        Recipe {
            name: "iccmax-version",
            category: Malformed,
            covers: "ParseError::IccMaxRefused",
            what: "v4 base with the version word set to 0x05000000",
            expect: "REFUSED by name: the message contains 'iccMAX'; exit 1",
            build: iccmax_version,
        },
        Recipe {
            name: "hostile-tag-count",
            category: Malformed,
            covers: "ParseError::TagCountOverflowsFile",
            what: "v4 base with tagCount = 0xFFFFFFFF",
            expect: "REFUSED before allocating: tag count 4294967295 requires 51539607672 bytes of directory; exit 1; no OOM, no hang",
            build: hostile_tag_count,
        },
        Recipe {
            name: "header-reserved-nonzero",
            category: Malformed,
            covers: "Malformation::HeaderReservedNonZero",
            what: "v4 base with header byte 100 set to 0x01",
            expect: "parses; exactly 1 malformation: header reserved bytes 100..128 are not all zero",
            build: header_reserved_nonzero,
        },
        Recipe {
            name: "trailing-bytes",
            category: Malformed,
            covers: "Malformation::TrailingBytes",
            what: "v4 base with 8 bytes appended and header.size left alone",
            expect: "parses; exactly 1 malformation: 8 trailing byte(s) (normal for container-embedded profiles)",
            build: trailing_bytes,
        },
        Recipe {
            name: "rendering-intent-high-bits",
            category: Malformed,
            covers: "Malformation::UnknownRenderingIntent (ambiguity A7)",
            what: "v4 base with rendering intent 0x00010001 — low half legal, high half non-zero",
            expect: "parses; exactly 1 malformation: rendering intent 0x00010001 is outside the defined 0..=3",
            build: rendering_intent_high_bits,
        },
        Recipe {
            name: "tag-overrun",
            category: Malformed,
            covers: "Malformation::TagOverrun",
            what: "v4 base with wtpt's declared size enlarged past the end of the profile",
            expect: "parses; exactly 1 malformation: tag[i] wtpt: data extends past declared profile size; the tag decodes to nothing",
            build: tag_overrun,
        },
        Recipe {
            name: "tag-overlaps-table",
            category: Malformed,
            covers: "Malformation::TagOverlapsTable",
            what: "v4 base with wtpt's offset set to 132, inside the tag table",
            expect: "parses; primary malformation: tag[2] wtpt: data offset lies inside the tag table. \
                     ENTAILED (one mutation, several reports): the bytes at 132 are the first directory \
                     entry, so the tag's 'type signature' reads as 'desc' and its icTagBase reserved word \
                     reads as that entry's offset field, hence also 'reserved bytes non-zero' and a decode \
                     refusal. Derivable from the fixture's own bytes.",
            build: tag_overlaps_table,
        },
        Recipe {
            name: "tag-misaligned",
            category: Malformed,
            covers: "Malformation::TagMisaligned",
            what: "v4 base with wtpt's offset decremented by one",
            expect: "parses; primary malformation: tag[2] wtpt: offset not 4-byte aligned. \
                     ENTAILED: reading 8 bytes from offset-1 puts the trailing space of 'XYZ ' into the \
                     icTagBase reserved word, so 'reserved bytes non-zero' is reported too. One mutation; \
                     the second report is a consequence of it, derivable from the bytes.",
            build: tag_misaligned,
        },
        Recipe {
            name: "tag-too-small",
            category: Malformed,
            covers: "Malformation::TagTooSmall",
            what: "v4 base with wtpt's declared size set to 4",
            expect: "parses; exactly 1 malformation: tag[i] wtpt: size < 8, too small for a type signature",
            build: tag_too_small,
        },
        Recipe {
            name: "tagbase-reserved-nonzero",
            category: Malformed,
            covers: "Malformation::TagBaseReservedNonZero + TagIssue::BaseReservedNonZero",
            what: "v4 base with the four reserved bytes after wtpt's type signature set to 0x01010101",
            expect: "parses; 1 malformation (reserved bytes after type signature non-zero) AND the tag still decodes",
            build: tagbase_reserved_nonzero,
        },
        Recipe {
            name: "duplicate-tag-signature",
            category: Malformed,
            covers: "Malformation::DuplicateTagSignature",
            what: "v4 base with a second wtpt entry aliased onto the first",
            expect: "parses; exactly 1 malformation: duplicate of tag[i] (consumers take the first; A13)",
            build: duplicate_tag_signature,
        },
        Recipe {
            name: "mluc-record-size-16",
            category: Malformed,
            covers: "TagDecodeError::MlucUnsupportedRecordSize",
            what: "v4 base whose desc is an mluc declaring recordSize = 16",
            expect: "parses; 0 table-level malformations; desc decode REFUSED: mluc recordSize 16 (shall be 12)",
            build: mluc_record_size_16,
        },
        Recipe {
            name: "mluc-odd-offset",
            category: Malformed,
            covers: "TagIssue::MlucOddOffset",
            what: "v4 base whose desc is an mluc with an odd string offset (29)",
            expect: "parses; 0 table-level malformations; desc issue: record 0 string offset is odd",
            build: mluc_odd_offset,
        },
        Recipe {
            name: "curv-count-overflows-tag",
            category: Malformed,
            covers: "TagDecodeError::CountOverflowsData",
            what: "v4 base whose rTRC is a curv declaring count = 1000 with two entries present",
            expect: "parses; rTRC decode REFUSED: curv: count 1000 exceeds tag data; refused before allocation",
            build: curv_count_overflows_tag,
        },
        Recipe {
            name: "para-unknown-functype",
            category: Malformed,
            covers: "TagIssue::UnknownParametricFunction",
            what: "v4 base whose rTRC is a para with funcType = 9",
            expect: "parses; rTRC decodes with issue: parametric funcType 9 not in 0..=4; parameters kept raw",
            build: para_unknown_functype,
        },
        Recipe {
            name: "text-not-ascii",
            category: Malformed,
            covers: "TagIssue::TextNotAscii",
            what: "v2 base whose textType cprt contains byte 0xE9",
            expect: "parses; 0 malformations; cprt decodes with issue: textType contains non-ASCII bytes (kept verbatim, not transcoded)",
            build: text_not_ascii,
        },
        Recipe {
            name: "text-unterminated",
            category: Malformed,
            covers: "TagIssue::TextUnterminated",
            what: "v2 base whose textType cprt has no terminating NUL",
            expect: "parses; 0 malformations; cprt decodes with issue: textType lacks a terminating NUL; the declared size is used",
            build: text_unterminated,
        },
        Recipe {
            name: "desc-short-mac-block",
            category: Malformed,
            covers: "TagIssue::DescMacBlockShortOrMissing — the shape found in the wild",
            what: "v2 base whose desc ends before the fixed 67-byte Macintosh ScriptCode block",
            expect: "parses; desc decodes with issue: Macintosh ScriptCode block short or missing — the same report the 40-profile sweep produced on this machine's EIZO profiles",
            build: desc_short_mac_block,
        },
        Recipe {
            name: "mft2-clut-size-exceeds-tag",
            category: Malformed,
            covers: "TagDecodeError::LutSizeExceedsTag / LutSizeOverflow",
            what: "v4 base plus an A2B0 mft2 header claiming inputChan=8, clutPoints=255 with 16 bytes of data",
            expect: "parses; A2B0 decode REFUSED with the needed size named; no allocation attempted; no OOM, no hang",
            build: mft2_clut_size_exceeds_tag,
        },
        Recipe {
            name: "mft2-pad-nonzero",
            category: Malformed,
            covers: "TagIssue::LutPadNonZero",
            what: "v4 base plus an otherwise valid A2B0 mft2 whose pad byte at offset 11 is 0x01",
            expect: "parses; A2B0 decodes fully AND reports the non-zero pad",
            build: mft2_pad_nonzero,
        },
        Recipe {
            name: "mab-clut-precision-3",
            category: Malformed,
            covers: "TagDecodeError::ClutBadPrecision",
            what: "v4 base plus an A2B0 mAB whose CLUT precision byte is 3",
            expect: "parses; A2B0 decode REFUSED: clut precision 3 (shall be 1 or 2) — the sample width is unknowable",
            build: mab_clut_precision_3,
        },
        Recipe {
            name: "mab-curve-chain-broken",
            category: Malformed,
            covers: "TagDecodeError::CurveChainBroken",
            what: "v4 base plus an A2B0 mAB whose second B curve is a bare 'curv' signature",
            expect: "parses; A2B0 decode REFUSED naming element 2 at byte 56 — the truncated element swallows \
                     the next element's header and the chain breaks one element later (byte table in the \
                     recipe's doc comment); the report is a position, not a generic short-data error",
            build: mab_curve_chain_broken,
        },
        Recipe {
            name: "xyz-trailing-bytes",
            category: Malformed,
            covers: "TagIssue::XyzTrailingBytes",
            what: "v4 base whose wtpt carries one XYZNumber plus four spare bytes",
            expect: "parses; wtpt decodes n=1 AND reports remainder 4",
            build: xyz_trailing_bytes,
        },
    ]
}

/// Look a recipe up by name.
#[must_use]
pub fn find(name: &str) -> Option<Recipe> {
    all().into_iter().find(|r| r.name == name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::profile::read_u32;

    /// ★ The structural invariant the colorant split exists for: device
    /// (1, 1, 1) reaches the PCS white point **exactly**, integer for integer,
    /// with no rounding anywhere.
    #[test]
    fn colorants_sum_to_the_encoded_white_point() {
        for c in 0..3 {
            assert_eq!(
                COLORANT_R[c] + COLORANT_G[c] + COLORANT_B[c],
                D50_ENCODED[c],
                "component {c} does not sum to the encoded D50 white point"
            );
        }
    }

    /// Every recipe must produce a profile whose declared size matches its
    /// byte count and whose magic is `'acsp'` — except the ones whose entire
    /// purpose is to break exactly that.
    #[test]
    fn every_recipe_produces_a_self_consistent_profile_unless_it_is_the_defect() {
        for r in all() {
            let p = (r.build)();
            assert!(!p.is_empty(), "{} produced nothing", r.name);
            if !matches!(
                r.name,
                "bad-magic" | "too-short" | "truncated-declared-size" | "trailing-bytes"
            ) {
                assert_eq!(&p[36..40], b"acsp", "{} lost its magic", r.name);
                assert_eq!(
                    read_u32(&p, 0) as usize,
                    p.len(),
                    "{} has a size field that disagrees with its length",
                    r.name
                );
            }
        }
    }

    /// Names are unique and file-system safe — they become file names.
    #[test]
    fn recipe_names_are_unique_and_safe() {
        let mut seen = std::collections::BTreeSet::new();
        for r in all() {
            assert!(seen.insert(r.name), "duplicate recipe name {}", r.name);
            assert!(
                r.name
                    .bytes()
                    .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-'),
                "{} is not a safe file name",
                r.name
            );
        }
    }

    /// Generation is deterministic — the property `gen-profiles verify`
    /// depends on, and the reason nothing here reads a clock.
    #[test]
    fn every_recipe_is_deterministic() {
        for r in all() {
            assert_eq!((r.build)(), (r.build)(), "{} is not deterministic", r.name);
        }
    }

    /// ★ The discriminator pair's control: the two profiles differ **only** in
    /// the four version bytes at header offset 8. If this fails, any difference
    /// a consumer shows between them could have another cause and the pair
    /// measures nothing.
    #[test]
    fn the_discriminator_pair_differs_only_in_the_version_word() {
        let a = v4_rgb_mft2_lab();
        let b = v2_rgb_mft2_lab();
        assert_eq!(a.len(), b.len());
        let diffs: Vec<usize> = (0..a.len()).filter(|&i| a[i] != b[i]).collect();
        assert!(!diffs.is_empty(), "the pair is identical — no variable");
        assert!(
            diffs.iter().all(|&i| (8..12).contains(&i)),
            "differences outside the version word: {diffs:?}"
        );
    }

    /// The `mBA ` fixture carries the curve counts clause 10.13 states, not
    /// the mirror of the `mAB ` rule. Asserted on the produced bytes rather
    /// than on the constant, so a builder change cannot quietly undo it.
    #[test]
    fn the_mba_fixture_has_three_b_curves_and_four_a_curves() {
        let p = v4_cmyk_mab_lab();
        let (off, _) = crate::profile::tag_location(&p, b"B2A0");
        let off = off as usize;
        assert_eq!(&p[off..off + 4], b"mBA ");
        assert_eq!(p[off + 8], 3, "inputChan (Lab)");
        assert_eq!(p[off + 9], 4, "outputChan (CMYK)");
        let offset_b = read_u32(&p, off + 12) as usize;
        let offset_mat = read_u32(&p, off + 16) as usize;
        // Three 12-byte identity curves sit between offsetB and offsetMat.
        assert_eq!(offset_mat - offset_b, 3 * 12, "three B curves, per 10.13.2");
        let offset_a = read_u32(&p, off + 28) as usize;
        let tag_end = crate::profile::tag_location(&p, b"B2A0").1 as usize;
        assert_eq!(tag_end - offset_a, 4 * 12, "four A curves, per 10.13.6");
    }

    /// Malformed fixtures must be malformed in exactly one way, which at this
    /// level means: they must still be a profile (unless that is the defect).
    #[test]
    fn malformed_fixtures_are_still_recognisable_files() {
        for r in all()
            .into_iter()
            .filter(|r| r.category == Category::Malformed)
        {
            let p = (r.build)();
            assert!(p.len() >= 100, "{} is implausibly small", r.name);
        }
    }
}
