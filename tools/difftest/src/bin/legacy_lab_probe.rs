//! # `legacy_lab_probe` — what does lcms2 *actually do* with an `mft2` Lab
//! LUT in a **v4** profile?
//!
//! This binary exists to settle one question, owed by `ARCHITECTURE.md`
//! **DL-011** and gating Pass 4's PCSLAB decoder:
//!
//! > When the legacy 16-bit PCSLAB encoding applies is, per ICC.1:2022
//! > **6.3.4.2 NOTE 3** and **10.10**, a property of the **tag type**
//! > (`lut16Type`, `namedColor2Type`) and of nothing else. The corpus's first
//! > pass claimed it keyed off the **profile version**, and recorded that
//! > lcms2 does the same. DL-011 retracted the first claim and left the
//! > second **unverified**: *"Whether lcms2 has another code path that makes
//! > those cases come out right anyway was not verified; no lcms2 tree was
//! > read in that corpus pass."*
//! >
//! > DL-011: *"What would settle it — and it is owed to `icc-conformance`:
//! > a behavioural difftest. Build a synthetic v4 profile containing an
//! > `mft2` Lab `A2B0`, push a known `L*` through `transicc`, and see which
//! > of `652.8` / `655.35` lcms2 used."*
//!
//! That is exactly what this does.
//!
//! ## The design, and why each part of it is there
//!
//! **The instrument.** Four synthetic profiles, authored byte by byte here
//! (category (a) per `LEGAL.md` §3 — unrestricted, and the only kind that
//! cannot inherit a bug from the code under test). Each is an input (`scnr`)
//! profile, RGB device space, **Lab PCS**, whose only transform tag is an
//! `A2B0` of type **`mft2` (`lut16Type`)** carrying a 2×2×2 CLUT whose corner
//! values are chosen so that the two candidate decodings give visibly
//! different answers.
//!
//! **The variable.** Three of the four profiles are **byte-identical except
//! for the four version bytes at header offset 8**:
//!
//! | file | header version | |
//! |---|---|---|
//! | `probe_v2_1.icc` | `0x02100000` | v2.1 — the control |
//! | `probe_v4_3.icc` | `0x04300000` | v4.3 |
//! | `probe_v4_4.icc` | `0x04400000` | v4.4 — the edition ICC.1:2022 defines |
//!
//! If the outputs differ, the difference is caused by the version field and
//! by nothing else, because nothing else differs. That is the whole point of
//! making them byte-identical, and it is why the profiles are generated here
//! rather than collected.
//!
//! **The control matters as much as the test.** The v2.1 profile is the case
//! where *both* candidate rules agree (legacy). If it does not read as
//! legacy, the instrument is broken and the v4 result means nothing. An
//! experiment whose apparatus is not shown to be able to detect the effect it
//! is looking for is not an experiment.
//!
//! **The fourth profile** (`probe_v4_3_mluc.icc`) closes the one objection
//! the first three invite. To keep the three byte-identical, all of them use
//! v2-era `textDescriptionType`/`textType` for `desc`/`cprt`, which is not
//! the correct type for a v4 profile. That cannot plausibly reach the LUT
//! decode path — but "cannot plausibly" is not a measurement, so the fourth
//! profile is a v4.3 with proper `multiLocalizedUnicodeType` metadata, and
//! must give the same answer as `probe_v4_3.icc`.
//!
//! ## The probes, and their two predictions
//!
//! Legacy (ICC.1:2022 Tables 42/43): `L* = v / 652.80`, `a*,b* = v/256 − 128`.
//! General (6.3.4.2 Tables 12/13):   `L* = v·100/65535`, `a*,b* = v·255/65535 − 128`.
//!
//! | probe | RGB in | CLUT value (L, a, b) | legacy | general |
//! |---|---|---|---|---|
//! | P1 | 255,255,255 | `FF00 8000 8000` | 100.0000, 0.0, 0.0 | 99.6109, −0.4980, −0.4980 |
//! | P2 | 0,0,0 | `0000 8000 8000` | 0.0, 0.0, 0.0 | 0.0, −0.4980, −0.4980 |
//! | P3 | 255,0,0 | `8000 8000 8000` | 50.1961, 0.0, 0.0 | 50.0008, −0.4980, −0.4980 |
//! | P4 | 0,255,0 | `FF00 FF00 0000` | 100.0000, 127.0, −128.0 | 99.6109, 126.0078, −128.0 |
//!
//! (P3/P4 "general" cells corrected 2026-08-11 by `icc-engineer` after
//! `icc-librarian` recomputed them from this file's own `decode_general`:
//! `32768·100/65535 = 50.000763 → 50.0008`; `65280·255/65535 − 128 =
//! 126.007782 → 126.0078`. README §12.1 was already right; the wrong
//! cells sat on the REJECTED hypothesis and run-time predictions are
//! computed, not read from this table, so no verdict was affected.)
//!
//! `0xFF00` is chosen because it is the legacy full-scale point: it is the
//! value at which the two rules are furthest apart and the one whose
//! mis-decoding produces the ≈0.39 % darkening of neutrals that DL-005 says
//! hides below the perceptibility anchor. P2's `L*` is deliberately a case
//! where the two rules **agree** (zero maps to zero either way) while their
//! `a*`/`b*` still differ — a probe that separates the channels.
//!
//! All corners are exact CLUT grid points, so no interpolation happens: the
//! numbers read back are the decoded table entries and not an artefact of the
//! grid. `-c0` (`cmsFLAGS_NOOPTIMIZE`) is used so lcms2 evaluates the
//! pipeline as read rather than a flattened resampling of it.
//!
//! ## ★ The confound this experiment ran into, and why the verdict is taken
//! at media-relative colorimetric only
//!
//! The first run of this probe used **both** intent 0 (perceptual) and intent
//! 1 (media-relative colorimetric). At intent 1 every profile gave a clean
//! answer. At intent 0 the **v4** profiles returned values that matched
//! *neither* hypothesis — black came back at `L* = −3.1482` instead of 0 —
//! while the v2 profile was unaffected.
//!
//! That is not the Lab encoding. It is a **second, entirely separate,
//! version-keyed behaviour in lcms2**, found by reading the source at the pin
//! (`src/cmscnvrt.c`, `_cmsLinkProfiles`):
//!
//! ```text
//! // Check if black point is really needed or allowed. Note that
//! // following Adobe's document:
//! // BPC does not apply to devicelink profiles, nor to abs colorimetric,
//! // and applies always on V4 perceptual and saturation.
//! if (TheIntents[i] == INTENT_PERCEPTUAL || TheIntents[i] == INTENT_SATURATION) {
//!     // Force BPC for V4 profiles in perceptual and saturation
//!     if (cmsGetEncodedICCversion(hProfiles[i]) >= 0x4000000)
//!         BPC[i] = TRUE;
//! }
//! ```
//!
//! — with the black point itself taken from a fixed constant for that case
//! (`src/cmssamp.c`: *"v4 + perceptual & saturation intents does have its own
//! black point… Black point tag is deprecated in V4"*, using
//! `cmsPERCEPTUAL_BLACK_X/Y/Z` = 0.003 36 / 0.003 473 1 / 0.002 87, the
//! perceptual reference medium black).
//!
//! So **lcms2 silently enables black point compensation for v4 profiles at
//! perceptual and saturation, on the authority of an Adobe document rather
//! than ICC.1**, and that transformation sits between the LUT and the number
//! this probe reads. Two consequences:
//!
//! 1. **The encoding verdict is taken at intent 1 only**, where no such stage
//!    exists. Intent 0 is still run and printed, labelled as confounded, and
//!    excluded from the verdict — deleting it would hide a real finding.
//! 2. The confound is **confirmed rather than assumed**, by two tests whose
//!    outcomes were different and both of which are reported:
//!
//!    - *Test 1, which does not decide.* Re-run the byte-identical **v2**
//!      profile at intent 0 with `-b`, expecting it to reproduce the v4
//!      numbers. It does not — `-b` changes nothing on the v2 fixture, because
//!      `cmsDetectBlackPoint` never finds a black point for it (the fixed
//!      perceptual constant sits behind the same `>= 0x4000000` guard, and the
//!      fallback darker-colorant search has nothing to chew on in a 2×2×2
//!      CLUT). Equal source and destination black points make lcms2 skip the
//!      stage. So the two arms differ in more than the flag and the comparison
//!      is **inconclusive by construction** — recorded, not deleted, because a
//!      reader who repeats it will otherwise think it refutes the hypothesis.
//!    - *Test 2, which decides.* Predict the intent-0 values **quantitatively**
//!      from lcms2's own BPC arithmetic (`ComputeBlackPointCompensation`,
//!      transcribed in [`predict_bpc_lstar`]) with the perceptual reference
//!      black, and compare. Measured 2026-08-11: predicted `L*` matches
//!      observed to **3×10⁻⁵** across all four probes, including the
//!      `0 → −3.1482` shift. The mechanism is identified.
//!
//! This is worth more than the answer it was blocking. It is a version-keyed
//! divergence that lands in Pass 4 (intent handling) and Pass 5 (BPC), it is
//! not required by ICC.1, and it is a plausible origin for the corpus's
//! belief that *"lcms2 keys this decision on the profile version"* — lcms2
//! **does** key a decision on the profile version at perceptual intent. Just
//! not this one.
//!
//! ## What this can and cannot establish
//!
//! It establishes **what lcms2 2.19.1 does**, on this machine, for this shape
//! of profile. It is an `implementation-cross-check`-class observation
//! (`NUMERIC_CLAIMS.md` §1), *not* ground truth: the specification text is the
//! authority for what iccce must do, and this measurement cannot overrule it.
//! Its value is that it converts DL-011's *"a reading of two texts"* into a
//! measured fact about the field's dominant CMM — which is what decides
//! whether iccce's spec-following behaviour will visibly disagree with it.
//!
//! ## Running it
//!
//! ```text
//! cd tools/difftest && cargo run --bin legacy_lab_probe
//! ```
//!
//! Profiles are written to `tools/difftest/out/` (git-ignored: `*.icc`).
//! Exit codes: `0` a verdict was reached, `2` the oracle is missing or an
//! invocation failed, `3` the control failed so no verdict is possible.

use std::io::Write;
use std::path::{Path, PathBuf};

use iccce_difftest::{Bpc, Intent, Oracle, Precalc, Request, Space};

// ===========================================================================
// Byte-level profile authoring
// ===========================================================================
//
// Everything below writes ICC big-endian binary by hand. It is deliberately
// not a general profile writer — `tools/gen-profiles` (Pass 2) is that, and
// when it exists this probe should be ported onto it. What is here is the
// minimum that makes the question answerable today, with every field stated
// so a reader can check the bytes against the specification without running
// anything.

fn be32(v: u32) -> [u8; 4] {
    v.to_be_bytes()
}
fn be16(v: u16) -> [u8; 2] {
    v.to_be_bytes()
}

/// `s15Fixed16Number`: a signed 32-bit fixed-point value with 16 fractional
/// bits, i.e. `round(x · 65536)`. ICC.1:2022 6.1.3 / 4.6.
///
/// Round-half-away-from-zero via `round()`. The PCS illuminant encodes as
/// `0x0000F6D6 / 0x00010000 / 0x0000D32D`, which is the canonical byte
/// pattern seen in every real profile — a useful check that this function is
/// right, and it is asserted as such in `tests`.
fn s15fixed16(x: f64) -> [u8; 4] {
    let v = (x * 65536.0).round();
    // Explicit, checked narrowing. ICC s15Fixed16 spans [-32768, 32768);
    // anything outside it is a caller error in a fixture, and a silent wrap
    // would produce a plausible-looking wrong profile — the exact failure
    // mode this project exists to make impossible.
    debug_assert!(
        (-2147483648.0..2147483648.0).contains(&v),
        "s15Fixed16 out of range: {x}"
    );
    #[expect(clippy::cast_possible_truncation, reason = "range checked immediately above")]
    let bits = v as i32;
    bits.to_be_bytes()
}

/// Which metadata tag types to use for `desc` and `cprt`.
#[derive(Clone, Copy, PartialEq, Eq)]
enum MetaStyle {
    /// `textDescriptionType` + `textType` — the v2 forms. Used for all three
    /// version-varying profiles so they can be byte-identical apart from the
    /// version word.
    V2Text,
    /// `multiLocalizedUnicodeType` — the v4 form.
    V4Mluc,
}

struct Tag {
    sig: [u8; 4],
    data: Vec<u8>,
}

/// Assemble a complete ICC profile: 128-byte header, tag table, tag data.
///
/// Layout, per ICC.1:2022 clause 7:
/// - header is exactly 128 bytes;
/// - tag table is a `uInt32` count followed by 12 bytes per tag
///   (signature, offset from profile start, size **excluding** padding);
/// - each tag's data begins on a 4-byte boundary and is zero-padded to one.
///
/// The profile size at offset 0 is patched in at the end, because it is not
/// known until the data is laid out — a small thing that is nonetheless the
/// most common way a hand-written profile comes out unreadable.
fn build_profile(version: u32, meta: MetaStyle) -> Vec<u8> {
    let tags = vec![
        Tag {
            sig: *b"A2B0",
            data: mft2_probe_tag(),
        },
        Tag {
            sig: *b"desc",
            data: match meta {
                MetaStyle::V2Text => text_description_type("iccce legacy-Lab probe"),
                MetaStyle::V4Mluc => mluc_type("iccce legacy-Lab probe"),
            },
        },
        Tag {
            sig: *b"cprt",
            data: match meta {
                MetaStyle::V2Text => text_type(
                    "Synthetic fixture authored by the iccce project. MIT. No third-party content.",
                ),
                MetaStyle::V4Mluc => mluc_type(
                    "Synthetic fixture authored by the iccce project. MIT. No third-party content.",
                ),
            },
        },
        Tag {
            sig: *b"wtpt",
            // Media white point = the PCS illuminant D50 (0,9642 / 1,0000 /
            // 0,8249). Sourced: ICC.1:2022's PCS illuminant, the same triple
            // `iccce-color` uses (NUMERIC_CLAIMS NC-017). Setting it to D50
            // means no chromatic adaptation happens anywhere in this probe,
            // which keeps the measured quantity the encoding and only the
            // encoding.
            data: xyz_type(0.9642, 1.0000, 0.8249),
        },
    ];

    let n = tags.len();
    let table_bytes = 4 + 12 * n;
    let mut data_offset = 128 + table_bytes;
    // 128 + 4 + 12n is always a multiple of 4, so no initial alignment gap.
    debug_assert_eq!(data_offset % 4, 0);

    let mut table = Vec::with_capacity(table_bytes);
    table.extend_from_slice(&be32(u32::try_from(n).expect("tag count fits u32")));
    let mut blob = Vec::new();
    for t in &tags {
        table.extend_from_slice(&t.sig);
        table.extend_from_slice(&be32(
            u32::try_from(data_offset).expect("tag offset fits u32"),
        ));
        table.extend_from_slice(&be32(
            u32::try_from(t.data.len()).expect("tag size fits u32"),
        ));
        blob.extend_from_slice(&t.data);
        let pad = (4 - (t.data.len() % 4)) % 4;
        blob.extend(std::iter::repeat_n(0u8, pad));
        data_offset += t.data.len() + pad;
    }

    let mut p = Vec::with_capacity(data_offset);
    p.extend_from_slice(&be32(0)); //   0 profile size — patched below
    p.extend_from_slice(&be32(0)); //   4 preferred CMM: none
    p.extend_from_slice(&be32(version)); // 8 profile version — THE VARIABLE
    p.extend_from_slice(b"scnr"); //  12 device class: input
    p.extend_from_slice(b"RGB "); //  16 data colour space
    p.extend_from_slice(b"Lab "); //  20 PCS — Lab, which is what makes the
    //                                     encoding question exist at all
    // 24 date/time: 2026-08-11 00:00:00 UTC, as six uInt16
    for v in [2026u16, 8, 11, 0, 0, 0] {
        p.extend_from_slice(&be16(v));
    }
    p.extend_from_slice(b"acsp"); //  36 profile file signature
    p.extend_from_slice(&be32(0)); //  40 primary platform: none
    p.extend_from_slice(&be32(0)); //  44 profile flags: not embedded, independent
    p.extend_from_slice(&be32(0)); //  48 device manufacturer
    p.extend_from_slice(&be32(0)); //  52 device model
    p.extend_from_slice(&be32(0)); //  56 device attributes (hi)
    p.extend_from_slice(&be32(0)); //  60 device attributes (lo)
    p.extend_from_slice(&be32(0)); //  64 rendering intent: perceptual
    p.extend_from_slice(&s15fixed16(0.9642)); // 68 PCS illuminant X
    p.extend_from_slice(&s15fixed16(1.0000)); // 72 PCS illuminant Y
    p.extend_from_slice(&s15fixed16(0.8249)); // 76 PCS illuminant Z
    p.extend_from_slice(&be32(0)); //  80 profile creator
    p.extend(std::iter::repeat_n(0u8, 16)); // 84 profile ID (unset)
    p.extend(std::iter::repeat_n(0u8, 28)); // 100 reserved
    debug_assert_eq!(p.len(), 128);

    p.extend_from_slice(&table);
    p.extend_from_slice(&blob);

    let size = u32::try_from(p.len()).expect("profile size fits u32");
    p[0..4].copy_from_slice(&be32(size));
    p
}

/// The probe's `A2B0`, of type **`mft2` (`lut16Type`)** — the tag type that
/// ICC.1:2022 10.10 says uses the legacy PCSLAB encoding.
///
/// Structure (ICC.1:2022 10.10):
/// ```text
/// 0   'mft2'          16  e2          48  input table entries n (uInt16)
/// 4   reserved 0      20  e3          50  output table entries m (uInt16)
/// 8   input chans  3  ...             52  input tables  (3 · n · uInt16)
/// 9   output chans 3  44  e9          ..  CLUT (2³ · 3 · uInt16)
/// 10  grid points  2                  ..  output tables (3 · m · uInt16)
/// 11  reserved 0      12  e1
/// ```
///
/// - The 3×3 matrix is the identity. It is only meaningful when the *input*
///   space is XYZ (which it is not here), and lcms2 skips inserting a matrix
///   stage when it is the identity — so this contributes nothing but its
///   bytes.
/// - Input and output tables have the minimum legal 2 entries and run
///   `0x0000 → 0xFFFF`, i.e. exact identities. With 2 entries there is no
///   interpolation to get wrong.
/// - The CLUT is 2 grid points per input channel: 8 corners, addressed with
///   the **first channel varying slowest**, which is why corner (0,1,0) is
///   index 2.
///
/// **Everything is arranged so that each probe input lands exactly on a CLUT
/// corner.** The number that comes back is then a decoded table entry, full
/// stop — no interpolation, no resampling, nothing to attribute an unexpected
/// value to except the decoding rule, which is the point.
fn mft2_probe_tag() -> Vec<u8> {
    let mut t = Vec::new();
    t.extend_from_slice(b"mft2");
    t.extend_from_slice(&be32(0));
    t.push(3); // input channels
    t.push(3); // output channels
    t.push(2); // CLUT grid points per dimension
    t.push(0); // reserved
    for v in [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0] {
        t.extend_from_slice(&s15fixed16(v));
    }
    t.extend_from_slice(&be16(2)); // input table entries
    t.extend_from_slice(&be16(2)); // output table entries
    for _ in 0..3 {
        t.extend_from_slice(&be16(0x0000));
        t.extend_from_slice(&be16(0xFFFF));
    }
    // CLUT, index = ((r*2 + g)*2 + b). Corners not used by a probe are set to
    // mid-neutral so the table is well defined everywhere.
    const MID: [u16; 3] = [0x8000, 0x8000, 0x8000];
    let mut clut = [MID; 8];
    clut[0b111] = [0xFF00, 0x8000, 0x8000]; // P1 in = 255,255,255
    clut[0b000] = [0x0000, 0x8000, 0x8000]; // P2 in = 0,0,0
    clut[0b100] = [0x8000, 0x8000, 0x8000]; // P3 in = 255,0,0
    clut[0b010] = [0xFF00, 0xFF00, 0x0000]; // P4 in = 0,255,0
    for entry in clut {
        for c in entry {
            t.extend_from_slice(&be16(c));
        }
    }
    for _ in 0..3 {
        t.extend_from_slice(&be16(0x0000));
        t.extend_from_slice(&be16(0xFFFF));
    }
    debug_assert_eq!(t.len(), 124);
    t
}

/// `textDescriptionType` (ICC.1:2001-04 6.5.17) — the v2 form of `desc`.
/// ASCII count includes the terminating NUL; the Unicode and ScriptCode
/// sections are present but empty, and the 67-byte Macintosh ScriptCode
/// buffer is mandatory even when unused.
fn text_description_type(s: &str) -> Vec<u8> {
    let ascii = s.as_bytes();
    let mut t = Vec::new();
    t.extend_from_slice(b"desc");
    t.extend_from_slice(&be32(0));
    t.extend_from_slice(&be32(
        u32::try_from(ascii.len()).expect("description fits u32") + 1,
    ));
    t.extend_from_slice(ascii);
    t.push(0);
    t.extend_from_slice(&be32(0)); // Unicode language code
    t.extend_from_slice(&be32(0)); // Unicode count
    t.extend_from_slice(&be16(0)); // ScriptCode code
    t.push(0); // ScriptCode count
    t.extend(std::iter::repeat_n(0u8, 67)); // Macintosh ScriptCode description
    t
}

/// `textType` — 7-bit ASCII, NUL-terminated.
fn text_type(s: &str) -> Vec<u8> {
    let mut t = Vec::new();
    t.extend_from_slice(b"text");
    t.extend_from_slice(&be32(0));
    t.extend_from_slice(s.as_bytes());
    t.push(0);
    t
}

/// `multiLocalizedUnicodeType` (ICC.1:2022 10.15) — one `en-US` record whose
/// string is UTF-16BE. `offset` is measured from the **start of the tag data
/// element**, i.e. it includes this 28-byte prologue.
fn mluc_type(s: &str) -> Vec<u8> {
    let utf16: Vec<u8> = s.encode_utf16().flat_map(|u| u.to_be_bytes()).collect();
    let mut t = Vec::new();
    t.extend_from_slice(b"mluc");
    t.extend_from_slice(&be32(0));
    t.extend_from_slice(&be32(1)); // number of records
    t.extend_from_slice(&be32(12)); // record size
    t.extend_from_slice(b"en");
    t.extend_from_slice(b"US");
    t.extend_from_slice(&be32(
        u32::try_from(utf16.len()).expect("description fits u32"),
    ));
    t.extend_from_slice(&be32(28)); // 8 + 4 + 4 + 12
    t.extend_from_slice(&utf16);
    t
}

/// `XYZType` — one `XYZNumber`, three `s15Fixed16Number`.
fn xyz_type(x: f64, y: f64, z: f64) -> Vec<u8> {
    let mut t = Vec::new();
    t.extend_from_slice(b"XYZ ");
    t.extend_from_slice(&be32(0));
    t.extend_from_slice(&s15fixed16(x));
    t.extend_from_slice(&s15fixed16(y));
    t.extend_from_slice(&s15fixed16(z));
    t
}

// ===========================================================================
// The probes and their predictions
// ===========================================================================

struct Probe {
    name: &'static str,
    rgb: [f64; 3],
    /// The 16-bit PCS values the CLUT corner holds.
    encoded: [u16; 3],
}

const PROBES: [Probe; 4] = [
    Probe {
        name: "P1 L=0xFF00 (legacy full scale)",
        rgb: [255.0, 255.0, 255.0],
        encoded: [0xFF00, 0x8000, 0x8000],
    },
    Probe {
        name: "P2 L=0x0000 (rules agree on L, differ on a/b)",
        rgb: [0.0, 0.0, 0.0],
        encoded: [0x0000, 0x8000, 0x8000],
    },
    Probe {
        name: "P3 L=0x8000 (mid scale)",
        rgb: [255.0, 0.0, 0.0],
        encoded: [0x8000, 0x8000, 0x8000],
    },
    Probe {
        name: "P4 a=0xFF00 b=0x0000 (chroma extremes)",
        rgb: [0.0, 255.0, 0.0],
        encoded: [0xFF00, 0xFF00, 0x0000],
    },
];

/// Legacy 16-bit PCSLAB decoding — ICC.1:2022 Tables 42/43, the encoding
/// 10.10 assigns to `lut16Type` and 10.17 to `namedColor2Type`.
/// `L*` full scale is `0xFF00` = 65 280 = 652,80 per unit.
fn decode_legacy(v: [u16; 3]) -> [f64; 3] {
    [
        f64::from(v[0]) / 652.80,
        f64::from(v[1]) / 256.0 - 128.0,
        f64::from(v[2]) / 256.0 - 128.0,
    ]
}

/// General 16-bit PCSLAB decoding — ICC.1:2022 6.3.4.2 Tables 12/13.
/// `L*` full scale is `0xFFFF` = 65 535.
fn decode_general(v: [u16; 3]) -> [f64; 3] {
    [
        f64::from(v[0]) * 100.0 / 65535.0,
        f64::from(v[1]) * 255.0 / 65535.0 - 128.0,
        f64::from(v[2]) * 255.0 / 65535.0 - 128.0,
    ]
}

fn max_abs_diff(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b)
        .map(|(x, y)| (x - y).abs())
        .fold(0.0_f64, f64::max)
}

/// How close an observation must sit to a hypothesis to be attributed to it.
///
/// **Justification (`CLAUDE.md` rule 5).** The two hypotheses are separated by
/// ≥0,196 in `L*` at every probe used here and by ≈1,09 in `a*` at P4. The
/// noise floor is 16-bit quantisation of the PCS, which is 100/65535 ≈ 0,0015
/// in `L*` — plus `transicc`'s 4-decimal printing, 1×10⁻⁴. 0,01 sits about
/// 7× above the noise floor and about 20× below the smallest separation, so
/// no plausible rounding can move an observation from one hypothesis to the
/// other, and neither hypothesis can be "confirmed" by a sloppy bound. If an
/// observation matches *neither* within 0,01, the honest report is
/// "inconclusive" — which this program will say rather than picking the
/// nearer one.
const ATTRIBUTION: f64 = 0.01;

// ---------------------------------------------------------------------------
// Predicting the confound: lcms2's forced BPC, transcribed from its source
// ---------------------------------------------------------------------------

/// lcms2's perceptual reference medium black point, `cmsPERCEPTUAL_BLACK_*`
/// in `include/lcms2.h` at the pin. Transcribed, not remembered.
///
/// **Provenance matters here and is weaker than it looks.** These are lcms2's
/// constants, read out of lcms2's header. They are used here **only to predict
/// what lcms2 will do**, which is the one purpose for which an
/// implementation's own constants are the correct source. They are not a
/// sourced colorimetric value and must not migrate into `iccce-color`.
const LCMS2_PERCEPTUAL_BLACK: [f64; 3] = [0.00336, 0.0034731, 0.00287];

/// The D50 white lcms2 compensates about (`cmsD50_XYZ`), and the same D50 the
/// probe profiles declare.
const D50: [f64; 3] = [0.9642, 1.0, 0.8249];

/// The CIELAB transfer function, exact rational form — the same choice
/// `iccce-color` makes and names as deviation **NA-001**
/// (`NUMERIC_CLAIMS.md` §4). Written out here rather than imported because
/// this crate does not depend on `iccce-color`: the harness must not need the
/// code under test in order to run.
fn lab_f(t: f64) -> f64 {
    const LIMIT: f64 = (24.0 / 116.0) * (24.0 / 116.0) * (24.0 / 116.0);
    if t > LIMIT {
        t.cbrt()
    } else {
        (841.0 / 108.0) * t + 16.0 / 116.0
    }
}

fn lab_f_inv(f: f64) -> f64 {
    if f > 24.0 / 116.0 {
        f * f * f
    } else {
        (108.0 / 841.0) * (f - 16.0 / 116.0)
    }
}

/// Predict what `L*` becomes after lcms2's forced black point compensation.
///
/// Transcribed from `src/cmscnvrt.c`, `ComputeBlackPointCompensation`:
///
/// ```text
/// // This is a linear scaling in the form ax+b, where
/// // a = (bpout - D50) / (bpin - D50)
/// // b = - D50* (bpout - bpin) / (bpin - D50)
/// ```
///
/// applied **per XYZ channel**, with `bpin` = the perceptual reference black
/// and `bpout` = the destination black, which for the `*Lab4` destination is
/// (0,0,0).
///
/// Only the `Y` channel is predicted, hence only `L*`. `a*` and `b*` also move
/// (the `X` and `Z` scalings differ slightly from `Y`'s), and the observed
/// run shows exactly that — but predicting them would need the neutral's `X`
/// and `Z`, which the probe does not pin down as tightly. **Claiming only what
/// is predicted is the point**: one channel predicted to four decimals is
/// better evidence than three channels predicted loosely.
fn predict_bpc_lstar(l_star: f64) -> f64 {
    let y = lab_f_inv((l_star + 16.0) / 116.0) * D50[1];
    let bp_in = LCMS2_PERCEPTUAL_BLACK[1];
    let bp_out = 0.0;
    let t = bp_in - D50[1];
    let a = (bp_out - D50[1]) / t;
    let b = -D50[1] * (bp_out - bp_in) / t;
    let y2 = a * y + b;
    116.0 * lab_f(y2 / D50[1]) - 16.0
}

/// How close the prediction must land to count as identifying the mechanism.
///
/// **Justification.** The PCS is carried through lcms2's 16-bit pipeline, so
/// the finest `L*` step representable is `100/65535 ≈ 0,00153`; `transicc`
/// prints four decimals, adding 1×10⁻⁴. 0,005 is a little over three
/// quantisation steps — tight enough that only the right formula passes (the
/// effect being explained is ≈3,15 in `L*`, some 630× larger), loose enough
/// not to fail on the encoding grid.
const BPC_PREDICTION_TOL: f64 = 0.005;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Verdict {
    Legacy,
    General,
    Neither,
}

impl Verdict {
    fn tag(self) -> &'static str {
        match self {
            Verdict::Legacy => "LEGACY(0xFF00 full scale)",
            Verdict::General => "GENERAL(0xFFFF full scale)",
            Verdict::Neither => "INCONCLUSIVE",
        }
    }
}

fn main() {
    let out_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("out");
    if let Err(e) = std::fs::create_dir_all(&out_dir) {
        eprintln!("cannot create {}: {e}", out_dir.display());
        std::process::exit(2);
    }

    let specs: [(&str, u32, MetaStyle); 4] = [
        ("probe_v2_1.icc", 0x0210_0000, MetaStyle::V2Text),
        ("probe_v4_3.icc", 0x0430_0000, MetaStyle::V2Text),
        ("probe_v4_4.icc", 0x0440_0000, MetaStyle::V2Text),
        ("probe_v4_3_mluc.icc", 0x0430_0000, MetaStyle::V4Mluc),
    ];

    let mut written: Vec<(String, PathBuf, u32)> = Vec::new();
    for (name, version, meta) in specs {
        let bytes = build_profile(version, meta);
        let path = out_dir.join(name);
        if let Err(e) = std::fs::write(&path, &bytes) {
            eprintln!("cannot write {}: {e}", path.display());
            std::process::exit(2);
        }
        println!("note\twrote\t{}\t{} bytes", path.display(), bytes.len());
        written.push((name.to_string(), path, version));
    }

    // Prove the three version-varying profiles differ ONLY in the version
    // word. This is the experiment's control on its own apparatus: if it does
    // not hold, any difference between them could be caused by something
    // else, and the whole run means nothing.
    {
        let a = std::fs::read(&written[0].1).unwrap();
        let b = std::fs::read(&written[1].1).unwrap();
        let c = std::fs::read(&written[2].1).unwrap();
        let diffs_ab: Vec<usize> = (0..a.len()).filter(|&i| a[i] != b[i]).collect();
        let diffs_ac: Vec<usize> = (0..a.len()).filter(|&i| a[i] != c[i]).collect();
        println!(
            "note\tbyte-diff v2.1 vs v4.3: offsets {diffs_ab:?} (expected exactly [8, 9] — the version word)"
        );
        println!("note\tbyte-diff v2.1 vs v4.4: offsets {diffs_ac:?}");
        assert!(
            diffs_ab.iter().all(|&i| (8..12).contains(&i)),
            "profiles differ outside the version word; the experiment is invalid"
        );
        assert!(
            diffs_ac.iter().all(|&i| (8..12).contains(&i)),
            "profiles differ outside the version word; the experiment is invalid"
        );
    }

    let oracle = match Oracle::locate() {
        Ok(Some(o)) => o,
        Ok(None) => {
            eprintln!(
                "no transicc found; build it with fetch-lcms2.sh + build-lcms2.ps1, or set ICCCE_TRANSICC"
            );
            std::process::exit(2);
        }
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2);
        }
    };
    println!("note\toracle\t{}", oracle.path().display());
    match oracle.banner() {
        Ok(b) => println!("note\tbanner\t{b}"),
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2);
        }
    }

    // Run one arm: every probe against one profile, at one intent, with one
    // BPC setting. Returns the raw observations, so an arm can be compared
    // against another arm as well as against the two hypotheses.
    let run_arm = |path: &PathBuf, intent: Intent, bpc: Bpc| -> Vec<Vec<f64>> {
        PROBES
            .iter()
            .map(|probe| {
                let req = Request {
                    input: Space::profile(path.clone()),
                    output: Space::lab_v4(),
                    intent,
                    precalc: Precalc::Exact,
                    bpc,
                    values: probe.rgb.to_vec(),
                };
                match oracle.convert(&req, 3) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("oracle failed on {} / {}: {e}", path.display(), probe.name);
                        std::process::exit(2);
                    }
                }
            })
            .collect()
    };

    let mut verdicts: Vec<(String, Intent, Verdict)> = Vec::new();

    for (name, path, version) in &written {
        // Intent 1 is the measurement. Intent 0 is run and printed but
        // EXCLUDED from the verdict, because lcms2 forces BPC on there for v4
        // profiles — see the module header, "The confound this experiment ran
        // into". Dropping intent 0 entirely would hide a real finding;
        // letting it vote would corrupt the one being made.
        for intent in [Intent::RelativeColorimetric, Intent::Perceptual] {
            let confounded = intent == Intent::Perceptual;
            let observed = run_arm(path, intent, Bpc::Off);
            let mut per_probe = Vec::new();
            for (probe, got) in PROBES.iter().zip(&observed) {
                let legacy = decode_legacy(probe.encoded);
                let general = decode_general(probe.encoded);
                let d_legacy = max_abs_diff(got, &legacy);
                let d_general = max_abs_diff(got, &general);
                let v = if d_legacy <= ATTRIBUTION && d_general > ATTRIBUTION {
                    Verdict::Legacy
                } else if d_general <= ATTRIBUTION && d_legacy > ATTRIBUTION {
                    Verdict::General
                } else {
                    Verdict::Neither
                };
                println!(
                    "probe\t{name}\tversion=0x{version:08X}\tintent={}\t{}\tgot={got:?}\tlegacy={legacy:?}(d={d_legacy:.5})\tgeneral={general:?}(d={d_general:.5})\t{}{}",
                    intent.as_num(),
                    probe.name,
                    v.tag(),
                    if confounded {
                        "\tCONFOUNDED(lcms2 forces BPC for v4 at perceptual/saturation)"
                    } else {
                        ""
                    }
                );
                per_probe.push(v);
            }
            // A profile's verdict is only the shared verdict of all four
            // probes. Mixed probes would mean the effect is not the encoding.
            let all_same = per_probe.windows(2).all(|w| w[0] == w[1]);
            let v = if all_same {
                per_probe[0]
            } else {
                Verdict::Neither
            };
            println!(
                "verdict\t{name}\tversion=0x{version:08X}\tintent={}\t{}\t{}",
                intent.as_num(),
                v.tag(),
                if confounded {
                    "EXCLUDED-FROM-FINDING(confounded by forced BPC)"
                } else {
                    "counts toward the finding"
                }
            );
            if !confounded {
                verdicts.push((name.clone(), intent, v));
            }
        }
    }

    // -----------------------------------------------------------------
    // Confirming the confound instead of assuming it
    // -----------------------------------------------------------------
    //
    // Two tests, and the first one FAILS TO DECIDE — which is recorded rather
    // than dropped, because a reader who repeats it deserves to know why.
    //
    // Test 1 (inconclusive by construction). If the v4 intent-0 difference is
    // forced BPC, then asking for BPC explicitly on the byte-identical **v2**
    // profile (`-b`) ought to reproduce the v4 numbers. It does not: the v2
    // profile is unchanged by `-b`. That is NOT evidence against the
    // hypothesis, because the two arms differ in more than the flag. lcms2
    // must *find* a source black point before it can compensate, and
    // `cmsDetectBlackPoint` (cmssamp.c) reaches the fixed perceptual-reference
    // constant only through the branch guarded by
    // `cmsGetEncodedICCversion(hProfile) >= 0x4000000 && (perceptual ||
    // saturation)`. For the v2 profile it falls through to the darker-colorant
    // search, which this minimal fixture gives nothing to work with, and the
    // black point stays (0,0,0) — equal to the destination's, so lcms2 skips
    // the stage entirely (`if (BlackPointIn != BlackPointOut)`). So `-b` on v2
    // is a no-op here **for a reason that has nothing to do with the encoding**
    // and the arm comparison cannot settle anything.
    //
    // Test 2 (decisive). Predict the intent-0 v4 numbers *quantitatively* from
    // lcms2's own published BPC arithmetic and see whether they land. If a
    // formula transcribed out of `cmscnvrt.c` reproduces the observation to
    // four decimals, the mechanism is identified — no arm comparison needed.
    {
        let v2_bpc_on = run_arm(&written[0].1, Intent::Perceptual, Bpc::On);
        let v4_bpc_default = run_arm(&written[1].1, Intent::Perceptual, Bpc::Off);
        for (i, probe) in PROBES.iter().enumerate() {
            println!(
                "confound-test1\t{}\tv2.1 with -b = {:?}\tv4.3 without -b = {:?}",
                probe.name, v2_bpc_on[i], v4_bpc_default[i]
            );
        }
        println!(
            "confound-test1\tINCONCLUSIVE BY CONSTRUCTION — `-b` is a no-op on the v2 fixture because \
             cmsDetectBlackPoint finds no black point for it, so the two arms differ in more than the flag"
        );

        let mut worst = 0.0_f64;
        for (probe, got) in PROBES.iter().zip(&v4_bpc_default) {
            let pre_bpc = decode_legacy(probe.encoded)[0];
            let predicted = predict_bpc_lstar(pre_bpc);
            let d = (got[0] - predicted).abs();
            worst = worst.max(d);
            println!(
                "confound-test2\t{}\tL* pre-BPC={pre_bpc:.4}\tpredicted post-BPC={predicted:.4}\tobserved={:.4}\td={d:.5}",
                probe.name, got[0]
            );
        }
        println!(
            "confound-test2\tworst L* deviation from the predicted BPC transform = {worst:.5}\t{}",
            if worst <= BPC_PREDICTION_TOL {
                "CONFIRMED: the intent-0 v4 values ARE lcms2's forced BPC against the perceptual reference black \
                 (cmscnvrt.c ComputeBlackPointCompensation + cmssamp.c cmsPERCEPTUAL_BLACK_*). \
                 It is not the Lab encoding, and the intent-1 verdict stands untouched."
            } else {
                "NOT CONFIRMED: the intent-0 values are not explained by forced BPC. \
                 Do NOT quote the intent-0 rows until this is understood."
            }
        );
    }

    // -----------------------------------------------------------------
    // Interpretation
    // -----------------------------------------------------------------
    let control_ok = verdicts
        .iter()
        .filter(|(n, _, _)| n == "probe_v2_1.icc")
        .all(|(_, _, v)| *v == Verdict::Legacy);

    println!(
        "control\tv2.1 profile decodes as legacy: {}",
        if control_ok {
            "YES — the instrument can detect the effect"
        } else {
            "NO — INSTRUMENT INVALID, no verdict is possible from this run"
        }
    );
    if !control_ok {
        let mut o = std::io::stdout().lock();
        let _ = o.flush();
        std::process::exit(3);
    }

    let v4: Vec<Verdict> = verdicts
        .iter()
        .filter(|(n, _, _)| n.starts_with("probe_v4"))
        .map(|(_, _, v)| *v)
        .collect();
    let v4_all_legacy = v4.iter().all(|v| *v == Verdict::Legacy);
    let v4_all_general = v4.iter().all(|v| *v == Verdict::General);

    if v4_all_legacy {
        println!(
            "FINDING\tlcms2 2.19.1 applies the LEGACY encoding to an mft2 Lab tag in a v4 profile. \
             The selector is the TAG TYPE, not header.version. This AGREES with ICC.1:2022 6.3.4.2 NOTE 3 and 10.10, \
             and CONTRADICTS the corpus's claim that lcms2 keys on the profile version. \
             DL-011 predicts iccce and lcms2 will disagree here; on this pin they do NOT."
        );
    } else if v4_all_general {
        println!(
            "FINDING\tlcms2 2.19.1 applies the GENERAL encoding to an mft2 Lab tag in a v4 profile. \
             The selector is header.version. This CONTRADICTS ICC.1:2022 6.3.4.2 NOTE 3 and 10.10, \
             and CONFIRMS the corpus's claim. DL-011's predicted divergence is real and measured: \
             iccce will read L* about 0.39% higher than lcms2 on the majority of production CMYK profiles."
        );
    } else {
        println!(
            "FINDING\tINCONCLUSIVE — the v4 profiles did not agree with each other or with either hypothesis. \
             Do not record a verdict; investigate the apparatus first."
        );
    }
    let mut o = std::io::stdout().lock();
    let _ = o.flush();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The canonical byte pattern for the PCS illuminant. If `s15fixed16` is
    /// wrong, every number in the probe is wrong in a way that would be very
    /// hard to see, so it is checked against a value whose encoding appears
    /// in every real profile.
    #[test]
    fn s15fixed16_encodes_the_pcs_illuminant_canonically() {
        assert_eq!(s15fixed16(0.9642), [0x00, 0x00, 0xF6, 0xD6]);
        assert_eq!(s15fixed16(1.0000), [0x00, 0x01, 0x00, 0x00]);
        assert_eq!(s15fixed16(0.8249), [0x00, 0x00, 0xD3, 0x2D]);
    }

    /// The two decodings must be far enough apart at the chosen probes for the
    /// experiment to be able to tell them apart — 20× the attribution bound is
    /// the margin claimed in the doc comment, so assert it rather than assume.
    #[test]
    fn the_probes_actually_separate_the_two_hypotheses() {
        for p in &PROBES {
            let sep = max_abs_diff(&decode_legacy(p.encoded), &decode_general(p.encoded));
            assert!(
                sep > 20.0 * ATTRIBUTION,
                "probe {} separates the hypotheses by only {sep}",
                p.name
            );
        }
    }

    /// The three version-varying profiles must differ only in the version
    /// word. Asserted in the run too, but a unit test fails faster.
    #[test]
    fn version_is_the_only_variable() {
        let a = build_profile(0x0210_0000, MetaStyle::V2Text);
        let b = build_profile(0x0430_0000, MetaStyle::V2Text);
        assert_eq!(a.len(), b.len());
        let diffs: Vec<usize> = (0..a.len()).filter(|&i| a[i] != b[i]).collect();
        assert!(
            diffs.iter().all(|&i| (8..12).contains(&i)),
            "differences outside the version word: {diffs:?}"
        );
        assert!(!diffs.is_empty(), "the profiles are identical — no variable");
    }

    /// Header is exactly 128 bytes and the size field matches the file.
    #[test]
    fn header_size_and_declared_size_are_right() {
        let p = build_profile(0x0430_0000, MetaStyle::V4Mluc);
        assert_eq!(u32::from_be_bytes([p[0], p[1], p[2], p[3]]) as usize, p.len());
        assert_eq!(&p[36..40], b"acsp");
        assert_eq!(&p[20..24], b"Lab ");
    }

    /// Every tag begins on a 4-byte boundary, as clause 7.2 requires.
    #[test]
    fn tag_data_is_four_byte_aligned() {
        let p = build_profile(0x0430_0000, MetaStyle::V2Text);
        let n = u32::from_be_bytes([p[128], p[129], p[130], p[131]]) as usize;
        for i in 0..n {
            let base = 132 + 12 * i;
            let off = u32::from_be_bytes([p[base + 4], p[base + 5], p[base + 6], p[base + 7]]);
            assert_eq!(off % 4, 0, "tag {i} is not aligned");
        }
    }
}
