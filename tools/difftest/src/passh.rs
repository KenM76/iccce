//! # Pass H — acceptance and refusal, graded over the ICC's own published profile set
//!
//! Every earlier pass in this suite grades a **colour value**. Pass H does not,
//! and cannot: the corpus it runs on publishes *transforms*, never expected
//! *outputs*, so no ΔE in it could ever be ground truth (`DL-041`). What it
//! grades instead is broader than anything else the project holds — **which
//! files iccce accepts, which it refuses, and whether a refusal says why.**
//!
//! ## The corpus
//!
//! `D:\Dev\iccce-private-fixtures\color-org\` — 50 `.icc` files downloaded by
//! the operator from `color.org` on 2026-08-17, catalogued in that folder's
//! `README.md` under `### color-org/`. **Read those terms before touching
//! anything here.** 23 distinct `cprt` strings across the loose files and six
//! different licensing postures; the restrictive reading applies to the whole
//! folder. Therefore, and without exception:
//!
//! - **No file from it may be committed to this repository.**
//! - **No value read out of one may be copied into this repository.** Every
//!   number Pass H reports is computed at run time from the operator's disk and
//!   printed. The only identifiers this module holds are **file names** —
//!   pointers to a licensed artifact, not content of it — and structural facts
//!   (a version word, a channel count) which are format metadata.
//! - The corpus resolves from `$ICCCE_PRIVATE_FIXTURES`, then the default path,
//!   and **every row SKIPs with a reason when it is absent.** CI is permanently
//!   in the skipping case. ★ A green CI run is **not** evidence that anything
//!   here passed; it is evidence that nothing here ran.
//!
//! ## Why acceptance and refusal are worth a Pass of their own
//!
//! `CLAUDE.md` rule 6 — *"the parser reports; it does not repair"* — had until
//! now been demonstrated **only on profiles this project authored**
//! (`tools/gen-profiles`). A synthetic fixture asserting that iccce refuses
//! iccMAX proves that iccce refuses *the file we wrote to be refused*. The ten
//! iccMAX files in this corpus were written by the ICC, by X-Rite and by Kodak
//! for their own purposes, and none of them was shaped by anything in this
//! repository. That is the whole value, and it is not obtainable synthetically.
//!
//! ## The five sections
//!
//! | § | subject | needs |
//! |---|---|---|
//! | **A** | the **version gate**: 10 real iccMAX files refused by name, with their own version in the message, `stdout` empty, exit 1 — plus the committed synthetic control that isolates the version byte from the exotic content | corpus + shipped binary |
//! | **B** | the **acceptance population**: 40 real profiles across seven ICC versions, four device classes and three colour spaces, exit 0 and `malformations: 0` — cross-checked against lcms2's own accept/refuse verdict on the same 50 | corpus + shipped binary + oracle |
//! | **C** | the **N-channel population**: the first `7CLR` profile this project has ever seen, and the compiled path's behaviour on it | corpus + shipped binary |
//! | **D** | the **Probe profiles**, graded against the ICC's own **published** statement of what they do — the first `Kind::GroundTruth` rows in this crate | corpus (+ oracle for the second-implementation arm) |
//! | **E** | **coverage, reported not graded**: what the population actually contains, and what it does not | corpus |
//!
//! ## ★★★ §D is the first ground truth in this crate, and here is why it counts
//!
//! `Probev2.zip` ships `Probe2 Profile Readme June 1.pdf`, in which the ICC
//! states what `Probev2_ICCv4.icc` **does**, in numbers:
//!
//! > *"The rendering intent transforms (BToA tags or BToD tags) of the probe
//! > profile ignore the a\* and b\* components of incoming PCS colors, and map
//! > the L\* components directly to monotone tints of process colorants.
//! > (L\* = 0 is rendered as maximum colorant coverage, and L\* = 100 is
//! > rendered as unmarked media.) The B2A0 tag (perceptual rendering intent
//! > transform) renders the L\* values as tints of pure cyan. The B2A1 tag
//! > (relative colorimetric intent transform) renders them as tints of pure
//! > magenta, and the B2A2 tag (saturation intent transform) renders them as
//! > tints of pure yellow."*
//!
//! > *"For the perceptual (A2B0) tag, the output is set such that the measured
//! > L\* values are scaled and offset into the range 70 to 100. For the
//! > relative colorimetric (A2B1) tag, the L\* values are scaled and offset
//! > into the range 30 to 70. For the saturation (A2B2) tag, they are scaled
//! > to the range 0 to 30."*
//!
//! Those are **published vendor statements about a named file**, transcribed
//! with their source, containing no implementation's output. That is exactly
//! what [`Kind::GroundTruth`] is for, and it is the strongest evidence class
//! available anywhere in this suite.
//!
//! **What it is ground truth ABOUT, stated precisely, because overstating it
//! would be the failure this role exists to prevent.** It is ground truth about
//! **rendering-intent tag selection** and about **the lightness band a tag's
//! output lies in**. It is *not* a published colorimetric value: nobody
//! measured a patch and printed a `L*a*b*` triple. A row here can catch iccce
//! selecting `B2A1` when it was asked for `B2A0`, evaluating the wrong element
//! order, mis-decoding PCSLAB, or transposing an ink — every one of which is a
//! defect a cross-check against lcms2 could only catch if lcms2 happened not to
//! share it. It cannot certify that any number is *the right colour*.
//!
//! **Three caveats the readme itself supplies, and all three are load-bearing:**
//!
//! 1. The readme is titled for **Probe 2** and names `Probev2_ICCv4.icc`. It
//!    describes `Probev1_ICCv4.icc` only as *"the previous probe profile"* whose
//!    purpose the v2 profile's is *"similar to … with the addition of optional
//!    tags based on the MultiProcessingElement tag type"*. Applying the v2 table
//!    to a v1 file is therefore a **reading of that sentence**, not a statement
//!    the document makes. The v1 rows are graded — a reading that cannot fail is
//!    not a reading — but they are [`Kind::DerivedExpectation`], and a red one
//!    is ambiguous between *"iccce is wrong"* and *"the reading is wrong"*.
//!    That ambiguity is the entire reason they are not ground truth.
//! 2. The readme states the profile is **deliberately non-compliant**:
//!    *"Technically, this is non-compliant with the v4 ICC specification,
//!    because (obviously) the media relative colorimetric intent tags are not
//!    based on real measurement data."* That non-compliance is in the tags'
//!    **content**, not their **encoding**, so it does not touch anything §D
//!    grades — but a reader must not quote a green §D as evidence that iccce
//!    handles a *conformant* profile correctly.
//! 3. The readme's own colour-code table contains transcription defects (the
//!    `B2A2` block is duplicated; the prose describes `B2D0/1/2` where the table
//!    says `D2A0/1/2`; one paragraph names `A2B1`/`A2B2` while describing
//!    `D2B1`/`D2B2`). §D grades **only the prose statements quoted above**,
//!    which are internally consistent and unambiguous. The table is not used.
//!
//! ## Evidence class of every section, stated because they differ
//!
//! - §A's rows are [`Kind::DerivedExpectation`]: the harness reads the version
//!   word **out of the file's own bytes** and derives the required verdict from
//!   ICC.1:2022 §7.2.4's header layout plus this project's declared scope. No
//!   implementation's output is in the expectation.
//! - §B's accept/refuse agreement row is [`Kind::CrossCheck`] — two
//!   implementations reaching the same verdict about the same 50 files.
//! - §B's malformation row is [`Kind::DerivedExpectation`] and it is the
//!   weakest-argued row in the pass; its `why` says so.
//! - §C's rows are [`Kind::DerivedExpectation`] against the CLI's own stated
//!   contract (a refusal is *named*; a process abort is neither a result nor a
//!   refusal).
//! - §D's `Probev2_ICCv4` rows are [`Kind::GroundTruth`]; its `Probev1_*` rows
//!   are [`Kind::DerivedExpectation`] for the reason in caveat 1.
//! - §E is reported, never graded.

use std::path::{Path, PathBuf};

use iccce_cmm::lut_ab::LutAbModel;
use iccce_cmm::lut_transform::{Lut16Model, PcsKind, PcsValue};
use iccce_color::Lab;
use iccce_profile::Profile;
use iccce_profile::num::Signature;
use iccce_profile::tag_types::TagData;

use crate::{Kind, Metric, Record, SepUnits, Separation, Tolerance};

// ===========================================================================
// Where the corpus lives
// ===========================================================================

/// The corpus root, resolved the way every private-fixture section in this
/// crate resolves one: **environment variable, then default path, then skip.**
///
/// There is deliberately no third fallback and no bundled copy. A corpus that
/// cannot be redistributed must be *absent* on a machine that has not been
/// given it, and the suite must say so out loud rather than quietly grading
/// nothing.
#[must_use]
pub fn corpus_dir() -> PathBuf {
    std::env::var_os("ICCCE_PRIVATE_FIXTURES")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(r"D:\Dev\iccce-private-fixtures"))
        .join("color-org")
}

/// How many device channels a colour-space signature denotes.
///
/// ## Why this lives in the harness and not in `iccce-profile`
///
/// As of 2026-08-17 **no such table exists in the crates under test** — the
/// channel count is taken from whatever LUT tag is being evaluated, never from
/// `header.colorSpace`. A harness that asked the code under test how many
/// channels a signature has would be asking the defendant to write the charge
/// sheet. This table is built from the signature convention itself:
///
/// - The `nCLR` family (`'2CLR'` … `'FCLR'`) encodes the count in its **first
///   character**, hexadecimal, per ICC.1:2022 Table 19. `'6CLR'` is six,
///   `'7CLR'` is seven, `'ACLR'` is ten.
/// - The named spaces are enumerated: three for `RGB `/`Lab `/`XYZ `/`YCbr`/
///   `Yxy `/`HSV `/`HLS `/`CMY `, four for `CMYK`, one for `GRAY`, two for
///   `2CLR`.
///
/// `None` means "not a signature this table covers", which for this corpus is
/// the iccMAX spectral spaces — and that is a *reportable* answer, not a
/// guessed one.
///
/// ★ `icc-spec-librarian` is sourcing the normative table into
/// `D:\Dev\Rag-Specialized\ICC_Spec\` concurrently with this pass. When it
/// lands, this function should be checked against it and this comment updated
/// with the clause number. Until then it is the **convention**, read off the
/// signatures, and it is used only to *describe* the population — nothing
/// graded depends on it.
#[must_use]
pub fn channels_of(sig: u32) -> Option<u32> {
    let b = sig.to_be_bytes();
    // The nCLR family: first byte is a hex digit, the rest is "CLR".
    if &b[1..] == b"CLR" {
        let d = b[0] as char;
        if let Some(n) = d.to_digit(16) {
            if (2..=15).contains(&n) {
                return Some(n);
            }
        }
    }
    match &b {
        b"GRAY" => Some(1),
        b"CMYK" => Some(4),
        b"RGB " | b"Lab " | b"XYZ " | b"YCbr" | b"Yxy " | b"HSV " | b"HLS " | b"CMY " => Some(3),
        _ => None,
    }
}

/// The major version byte, read **straight out of the file** at offset 8.
///
/// ICC.1:2022 §7.2.4: the profile version field is a `u32` at byte offset 8
/// whose most significant byte is the major version. ICC.2:2023 (iccMAX) uses
/// major version 5. This function is the harness's *independent* reading of
/// that byte — the whole point of §A is that the expectation must not come
/// from the parser under test.
///
/// `None` for a file too short to have a header field at all, which is itself
/// a fact worth reporting rather than a panic.
#[must_use]
pub fn raw_version(bytes: &[u8]) -> Option<u32> {
    bytes
        .get(8..12)
        .map(|s| u32::from_be_bytes(s.try_into().expect("4 bytes")))
}

// ===========================================================================
// §D — the published Probe statements, transcribed with their source
// ===========================================================================

/// The three lightness bands the ICC's Probe 2 readme states for the proofing
/// (`AToB`) tags, in the readme's own order.
///
/// **Source, transcribed verbatim:** *"For the perceptual (A2B0) tag, the
/// output is set such that the measured L\* values are scaled and offset into
/// the range 70 to 100. For the relative colorimetric (A2B1) tag, the L\*
/// values are scaled and offset into the range 30 to 70. For the saturation
/// (A2B2) tag, they are scaled to the range 0 to 30."* — `Probe2 Profile
/// Readme June 1, 2007`, page 1, inside `Probev2.zip` as distributed by
/// `color.org`.
///
/// These are **published numbers about a named file**. They are the reason §D
/// can carry [`Kind::GroundTruth`] where nothing else in this crate can.
pub const A2B_BANDS: [(u32, &str, f64, f64); 3] = [
    (0x4132_4230, "perceptual", 70.0, 100.0),
    (0x4132_4231, "media-relative", 30.0, 70.0),
    (0x4132_4232, "saturation", 0.0, 30.0),
];

/// The colorant each rendering-intent (`BToA`) tag renders `L*` as a tint of,
/// as a **zero-based index into the profile's CMYK device channels**.
///
/// **Source, transcribed verbatim:** *"The B2A0 tag (perceptual rendering
/// intent transform) renders the L\* values as tints of pure cyan. The B2A1
/// tag (relative colorimetric intent transform) renders them as tints of pure
/// magenta, and the B2A2 tag (saturation intent transform) renders them as
/// tints of pure yellow."* — same document, same page.
///
/// The mapping colorant→index is the CMYK channel order of ICC.1:2022 Table 19
/// (`'CMYK'` is cyan, magenta, yellow, black in that order), not a convention
/// invented here.
pub const B2A_COLORANT: [(u32, &str, &str, usize); 3] = [
    (0x4232_4130, "perceptual", "cyan", 0),
    (0x4232_4131, "media-relative", "magenta", 1),
    (0x4232_4132, "saturation", "yellow", 2),
];

// ===========================================================================
// Evaluating one LUT tag, whichever family stores it
// ===========================================================================

/// One decoded LUT tag, built through `iccce-cmm`'s own models.
///
/// **Both storage families appear in this corpus's three Probe profiles** —
/// `Probev1_ICCv2` stores its tags as `mft2` (`lut16Type`), the two v4 probes
/// as `mAB `/`mBA ` (`lutAToBType`/`lutBToAType`) — so a §D that handled only
/// one family would silently grade one profile and skip two.
///
/// ★ This is an **in-process** evaluation of `iccce-cmm`, not the shipped
/// binary, and every record built from it says so. The shipped binary has no
/// surface that emits PCS `L*a*b*`; its `transform` takes a destination
/// *profile*. §D therefore carries **both** arms — this one for the PCS-side
/// claims, and a shipped-binary arm ([`shipped_intent_selection`]) for the
/// device-side ones, which is the arm that can see a wiring defect between the
/// CLI and the library.
pub enum TagEval {
    Ab(LutAbModel),
    Lut16(Box<Lut16Model>),
}

impl TagEval {
    /// `None` when the tag is absent, undecodable, or of a family this
    /// evaluator does not cover — all three of which are *reportable* states,
    /// never silently-zero ones.
    #[must_use]
    pub fn build(p: &Profile, sig: Signature) -> Option<TagEval> {
        let e = p.tags.iter().find(|t| t.sig == sig)?;
        let d = match p.decode_tag(e) {
            Some(Ok(d)) => d,
            _ => return None,
        };
        // Every Probe profile declares `header.pcs == 'Lab '`; the caller is
        // responsible for having checked that, and `device_to_lab` refuses an
        // XYZ result rather than reinterpreting it.
        let pcs = PcsKind::Lab;
        match d.data {
            TagData::LutAToB(l) => LutAbModel::from_lut_ab(&l, pcs).ok().map(TagEval::Ab),
            TagData::LutBToA(l) => LutAbModel::from_mba(&l, pcs).ok().map(TagEval::Ab),
            TagData::Lut16(l) => Lut16Model::from_lut16(&l, false, pcs)
                .ok()
                .map(|m| TagEval::Lut16(Box::new(m))),
            _ => None,
        }
    }

    #[must_use]
    pub fn device_channels(&self) -> usize {
        match self {
            TagEval::Ab(m) => m.device_channels(),
            TagEval::Lut16(m) => m.input_channels(),
        }
    }

    #[must_use]
    pub fn device_to_lab(&self, dev: &[f64]) -> Option<Lab> {
        let v = match self {
            TagEval::Ab(m) => m.device_to_pcs(dev)?,
            TagEval::Lut16(m) => m.device_to_pcs(dev)?,
        };
        match v {
            PcsValue::Lab(l) => Some(l),
            PcsValue::Xyz(_) => None,
        }
    }

    #[must_use]
    pub fn lab_to_device(&self, lab: Lab) -> Option<Vec<f64>> {
        match self {
            TagEval::Ab(m) => m.pcs_to_device(PcsValue::Lab(lab)),
            TagEval::Lut16(m) => m.pcs_to_device(PcsValue::Lab(lab)),
        }
    }
}

// ===========================================================================
// Deterministic sample sets — stated here so a reader can reproduce them
// ===========================================================================

/// The device grid §D walks through an `AToB` tag: five levels per channel,
/// every combination. For CMYK that is `5^4 = 625` points and it includes
/// every one of the 16 CLUT corners, so the extrema it finds are not an
/// interior-sampling artefact.
#[must_use]
pub fn device_grid(channels: usize) -> Vec<Vec<f64>> {
    let steps = [0.0_f64, 0.25, 0.5, 0.75, 1.0];
    let mut out = Vec::new();
    let mut idx = vec![0usize; channels];
    loop {
        out.push(idx.iter().map(|&i| steps[i]).collect());
        let mut c = 0;
        loop {
            if c == channels {
                return out;
            }
            idx[c] += 1;
            if idx[c] < steps.len() {
                break;
            }
            idx[c] = 0;
            c += 1;
        }
    }
}

/// The `(a*, b*)` pairs §D holds fixed while sweeping `L*`.
///
/// The readme's claim is that the `BToA` tags **ignore** `a*` and `b*`. Testing
/// that needs at least two chromatic points, and they must be far apart: a pair
/// of near-neutrals would agree for a reason that has nothing to do with the
/// claim. These four span the encodable `a*`/`b*` range in all four quadrants.
pub const AB_PROBES: [(f64, f64); 4] = [(0.0, 0.0), (-60.0, 40.0), (50.0, -70.0), (80.0, 80.0)];

/// The `L*` sweep: 101 points, one per integer `L*`.
#[must_use]
pub fn lightness_sweep() -> Vec<f64> {
    (0..=100).map(f64::from).collect()
}

// ===========================================================================
// Tolerances — each one derived, each one stating what it is derived FROM
// ===========================================================================

/// **Exact.** An indicator row — a count of files, or a count of violations —
/// has no measurement error to absorb, so any bound above zero would be an
/// allowance for a defect rather than for noise.
pub const EXACT_COUNT: Tolerance = Tolerance::new(
    0.0,
    "an indicator: a COUNT of files (or of violated conditions), not a measurement. \
     There is no instrument error to absorb and no rounding to admit, so any bound above \
     zero would be an allowance for a defect rather than for noise",
);

/// **The shipped binary's own print resolution**, for a row whose published
/// expectation is *"this channel carries none of the colorant"*.
///
/// `iccce transform` writes **6 decimals** (`crates/iccce-cli/src/main.rs`), so
/// the smallest non-zero value it can express is `1e-6` and a true zero and a
/// value below `5e-7` are indistinguishable in its output. The bound is one
/// printed unit rather than the half-ulp so that a last-digit rounding
/// difference is not a failure.
///
/// **This is not a perceptual tolerance and the 1.0 ΔE2000 anchor is
/// irrelevant to it** (`TOLERANCES.md` §2). It is the resolution of the
/// instrument doing the reading.
///
/// **What it is sensitive enough to catch**, which is the real test: if iccce
/// selected `B2A1` when asked for perceptual, the cyan channel would read
/// **0** and the magenta channel would carry the entire tint — up to `1.0`,
/// which is `10^6` times this bound. Intent mis-selection is not a near miss
/// here; it is six orders of magnitude.
pub const SHIPPED_PRINT_FLOOR: Tolerance = Tolerance::new(
    1e-6,
    "iccce transform prints 6 decimals, so 1e-6 is one printed unit and a true zero is \
     indistinguishable from anything below 5e-7; the bound is the unit rather than the \
     half-ulp so a last-digit rounding difference is not a failure. Arithmetic-resolution, \
     NOT perceptual. Sensitivity: selecting the wrong B2A tag moves this channel by up to \
     1.0, which is 1e6 times the bound",
);

// ★★★ WITHDRAWN 2026-08-17: `BAND_EDGE`, a `5e-3` `L*` bound for grading a
// realised A2B lightness band against the readme's published endpoints. Kept as
// a comment for the same reason `SEVEN_CORNER` below is — but note that this is
// a **different** failure from that one, and the difference is the whole point.
//
// **Its derivation, which is still correct:** the readme states a *design*
// target ("scaled and offset into the range 70 to 100"); what is in the file is
// that design quantised to the 16-bit PCS encoding the tag's storage uses, then
// read back through the tag's B curves. Three terms and only three —
// quantisation of an endpoint sample to one code (`100/65535 = 1.5259e-3` for
// `mAB ` per clause 10.13, `100/65280 = 1.5319e-3` for the legacy `mft2`
// encoding of 6.3.4.2 NOTE 3); the B curve's own table quantisation, ≤ one code
// on the same scale; and **zero** for interpolation, because an interpolant of
// samples that all lie in `[lo, hi]` is a convex combination and lies in
// `[lo, hi]`, so only the *nodes* can be outside. Two codes on the wider scale
// is `3.0637e-3`, rounded up to `5e-3`.
//
// **What was wrong was not the arithmetic but the PREMISE.** All three Probe
// profiles miss their published bands by `2.263736`–`3.000687` `L*` — three
// orders above any encoding budget — at heavy-ink corners of the device cube
// that are outside any real press gamut and are extrapolated in the table. And
// **lcms2 reproduces the excursion to `8.8e-4 L*`**, so it is a statement about
// the artefact, not about either engine. The published band is a design
// intention the published files do not realise.
//
// ★ **`SEVEN_CORNER` failed because a term was missing from OUR derivation;
// this failed because a premise was false about THEIR file.** The remedies
// differ accordingly: that one moved its graded claim to where the missing term
// is absent, this one **stopped claiming** — `a2b/published-band-containment`
// is REPORTED — and the graded claim moved to the property the readme's three
// bands actually support and no quantisation argument can erode: that the three
// realised bands are **disjoint and in the published order**. Overlap is zero
// on all three files, with the tightest gap `+1.033036 L*`.

/// **Reported, never graded.** Used for the coverage rows of §E and for
/// measurements this pass deliberately declines to gate.
pub const REPORTED: Tolerance = Tolerance::new(
    f64::INFINITY,
    "REPORTED, NOT GRADED. This row records a measured quantity for which this pass has no \
     defensible bound; a green line here is the absence of a claim, not the presence of one",
);

// ===========================================================================
// Small helpers the sections share
// ===========================================================================

/// The standard separation for a row whose only rival reading is one this
/// corpus cannot distinguish.
pub(crate) fn zero_sep(alternative: impl Into<String>, observed: f64) -> Separation {
    Separation::against_distance(alternative, observed, 0.0, SepUnits::SameAsMetric)
}

/// A byte count in the largest binary unit that leaves it >= 1, to three
/// significant figures, always beside the exact count and never instead of it.
///
/// ## Why this exists rather than a fixed `{:.2} TiB`
///
/// ★ The Pass H record filed on 2026-08-17 printed `(~0.00 TiB)` for a
/// **6 718 464-byte** allocation. The format string was written when the number
/// it was formatting was `1_022_842_631_448` (0.93 TiB) and the unit was chosen
/// for that magnitude; when the engineer's fix moved the number down by five
/// orders, the *number* in the record updated itself — it was interpolated —
/// and the *unit* did not, because a unit typed into a format string is prose.
///
/// **A fixed unit is a typed claim about magnitude**, and it goes stale by the
/// same mechanism as a typed numeral (see the `stale claim strings` note in
/// this pass's agent memory). Choosing the unit from the value removes the
/// claim. The exact byte count is always emitted next to it, because the human
/// unit is a courtesy and the byte count is the evidence.
pub(crate) fn human_bytes(bytes: u128) -> String {
    const UNITS: [&str; 6] = ["B", "KiB", "MiB", "GiB", "TiB", "PiB"];
    #[allow(clippy::cast_precision_loss)]
    let mut v = bytes as f64;
    let mut u = 0;
    while v >= 1024.0 && u + 1 < UNITS.len() {
        v /= 1024.0;
        u += 1;
    }
    if u == 0 {
        format!("{bytes} B")
    } else {
        format!("{v:.3} {}", UNITS[u])
    }
}

/// A captured stderr, rendered for a single-line TSV field — and rendered so
/// that **empty is visibly empty**.
///
/// ★ The 2026-08-17 record ended with the literal text `stderr: ` and nothing
/// after it, because the trailing `{}` interpolated an empty string. A reader
/// cannot tell that from a truncated field. `(empty)` can only mean one thing.
pub(crate) fn show_stderr(stderr: &str) -> String {
    let t = stderr.trim();
    if t.is_empty() {
        "(empty)".to_string()
    } else {
        t.replace('\n', " / ")
    }
}

// ===========================================================================
// The run
// ===========================================================================

use std::process::{Command, Stdio};

use crate::{Bpc, DiffError, Iccce, InspectRun, Intent, Oracle, Precalc, Request, Space};

/// The corpus member names §C and §D reach for by name.
///
/// These are **file names, not content** — pointers into a licensed folder.
/// Nothing about a name is a colour value, and nothing else from those files
/// is stored in this repository.
mod file {
    /// ★ The corpus's only `7CLR` profile, and the first N>4 device space this
    /// project has ever been given: APTEC's CMYK+orange+green+violet press
    /// characterisation, v4.2 `prtr`, `mAB ` A2B0/A2B1 (A2B2 **aliased to
    /// A2B0** at the same offset) and `mBA ` B2A0/B2A1 (B2A2 likewise).
    pub const SEVEN: &str = "APTEC_CMYKOGV_Coated_LinearCTV_2025.icc";
    /// The destination §C converts into: ICC's own v2.0 sRGB, chosen because it
    /// is in the same folder (so §C skips or runs as one unit) and because a
    /// matrix/TRC destination adds no LUT of its own to the comparison.
    pub const SRGB: &str = "sRGB2014.icc";
    /// ICC's v2 CMM probe (`mft2` throughout).
    pub const PROBE_V1_V2: &str = "Probev1_ICCv2.icc";
    /// ICC's v4 CMM probe (`mAB `/`mBA `, **no** `mpet` tags).
    pub const PROBE_V1_V4: &str = "Probev1_ICCv4.icc";
    /// ★ ICC's v4 CMM probe *2* — the file the published readme names, and the
    /// only one in the corpus carrying `D2B0/1/2` and `B2D0/1/2` (`mpet`).
    pub const PROBE_V2_V4: &str = "Probev2_ICCv4.icc";
}

/// The committed, category-(a) RGB source §D drives the shipped binary from.
///
/// **Why a committed synthetic fixture and not a corpus profile:** §D's
/// shipped-binary rows grade *which destination tag was selected*, and nothing
/// about the source should be able to influence that. A fixture whose bytes are
/// in this repository keeps the source side of the comparison inspectable by a
/// reader who has no access to the licensed corpus.
fn synthetic_rgb_source() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/synthetic/v2-rgb-matrix-trc-curv.icc")
}

/// The committed control fixture that isolates **the version byte** from
/// everything else an iccMAX file contains.
///
/// `fixtures/synthetic/iccmax-version.icc` is, apart from its version word, an
/// ordinary v2-shaped RGB matrix/TRC profile authored by `tools/gen-profiles`.
/// It is the reason §A can claim that the ten real refusals are caused by the
/// version and not by their exotic colour spaces, spectral tag types or
/// `namedColor2` content: **the same message comes out of a file that has none
/// of those things.**
fn iccmax_control() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/synthetic/iccmax-version.icc")
}

/// What one corpus file looked like to the harness and to the shipped binary.
pub struct FileVerdict {
    pub name: String,
    /// The version word the **harness** read out of bytes 8..12, independent of
    /// the parser under test.
    pub raw_version: Option<u32>,
    /// The refusal the harness's own reading of that word requires.
    pub harness_says_refuse: bool,
    pub run: InspectRun,
    /// `true` when lcms2 could build a transform out of this profile. `None`
    /// when the oracle was not available.
    pub lcms2_usable: Option<bool>,
}

impl FileVerdict {
    /// The major version byte, or 0 for a file too short to hold one.
    #[must_use]
    pub fn major(&self) -> u32 {
        self.raw_version.map_or(0, |v| v >> 24)
    }

    /// `true` when the shipped binary accepted the profile: exit status
    /// **exactly 0**, read bare.
    #[must_use]
    pub fn accepted(&self) -> bool {
        self.run.code == Some(0)
    }

    /// The `malformations:` count the CLI printed, or `None` if it printed
    /// none (which for an accepted profile is itself a defect and is graded).
    #[must_use]
    pub fn malformations(&self) -> Option<u32> {
        self.run.field("malformations")?.trim().parse().ok()
    }

    /// The subset of the disclosed set that breaches a stated
    /// requirement, as printed by `iccce inspect`'s `violations:` line.
    ///
    /// ★★ **This is the quantity a conformance question wants, and it
    /// did not exist until 2026-08-21.** `malformations` counts
    /// *disclosures*; `DL-063` established that a v2 profile can be
    /// fully conformant and still have something to disclose. Until the
    /// CLI printed this second number there was nothing for a row to
    /// grade except the mixed count — which is how
    /// `passh/B/acceptance/...` came to accuse five ICC-published files.
    ///
    /// `None` when the line is absent, which on a profile the parser
    /// accepted is itself a defect and is counted as one by the caller —
    /// **not** silently treated as zero.
    #[must_use]
    pub fn violations(&self) -> Option<u32> {
        self.run.field("violations")?.trim().parse().ok()
    }

    /// Every condition §A requires of a refusal, as a list of the ones that
    /// FAILED — so a red row names what was wrong rather than only that
    /// something was.
    #[must_use]
    pub fn refusal_defects(&self) -> Vec<&'static str> {
        let mut out = Vec::new();
        if self.run.code != Some(1) {
            out.push("exit status is not 1");
        }
        if !self.run.stderr.contains("iccMAX") {
            out.push("the message does not contain the string \"iccMAX\"");
        }
        let want = format!("0x{:08X}", self.raw_version.unwrap_or(0));
        if !self.run.stderr.contains(&want) {
            out.push("the message does not quote the profile's own version word");
        }
        if !self.run.stdout.trim().is_empty() {
            out.push("stdout is not empty — something was parsed and printed anyway");
        }
        out
    }
}

/// Everything Pass H measured, for the run's `note` line.
pub struct Analysis {
    pub structure: String,
    pub files: usize,
    pub accepted: usize,
    pub refused: usize,
}

/// Ask lcms2 whether it can build a transform out of `profile`.
///
/// ## ★ Why this does not use the exit code, and why that matters
///
/// **`transicc` exits 0 even when it prints `[transicc]: Couldn't link the
/// profiles` and converts nothing** — measured 2026-08-17 on all ten iccMAX
/// members of this corpus. An acceptance test that keyed on the oracle's exit
/// status would therefore have recorded lcms2 as *accepting* every one of
/// them, which is the exact opposite of what happened. The observable used
/// here is the only honest one available: **did any numbers come out.**
///
/// `channels` is the harness's own reading of the colour-space signature; when
/// it is unknown the profile is fed three values, which is enough for the
/// question being asked ("did lcms2 produce anything at all") and is recorded
/// as an approximation in the row's detail.
fn lcms2_usable(oracle: &Oracle, profile: &Path, channels: usize) -> Result<bool, DiffError> {
    let mut child = Command::new(oracle.path())
        .arg(format!("-i{}", profile.display()))
        .arg("-o*Lab2")
        .arg("-n")
        .arg("-c0")
        .arg("-t1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| DiffError::Spawn(oracle.path().to_path_buf(), e))?;
    let buf = "0\n".repeat(channels.max(1));
    let mut stdin = child
        .stdin
        .take()
        .ok_or_else(|| DiffError::Internal("child stdin was not piped".into()))?;
    let writer = std::thread::spawn(move || {
        use std::io::Write;
        let _ = stdin.write_all(buf.as_bytes());
    });
    let out = child
        .wait_with_output()
        .map_err(|e| DiffError::Spawn(oracle.path().to_path_buf(), e))?;
    let _ = writer.join();
    let stdout = String::from_utf8_lossy(&out.stdout);
    // A produced conversion is a line whose first token parses as a number.
    Ok(stdout.lines().any(|l| {
        l.split_whitespace()
            .next()
            .is_some_and(|t| t.parse::<f64>().is_ok())
    }))
}

// ===========================================================================
// The remaining tolerances — each derived, each stating what it derives FROM
// ===========================================================================

/// **One 16-bit device code.** The device side of every LUT tag in this corpus
/// is stored at 16-bit resolution (`mft2` output tables are `uInt16Number`;
/// `mAB `/`mBA ` `curv` entries likewise), so **a genuine dependence of the
/// output on an input the profile is said to ignore would have to be at least
/// one code**. Anything below one code cannot be a stored dependence; it is the
/// arithmetic of the interpolation, whose weights differ in the last bits of an
/// `f64` even when the samples they combine are identical.
///
/// `1/65535 = 1.5259e-5`. Measured on the two `Probev1` profiles: `4.44e-16`,
/// which is eleven orders below the bound — and that margin is itself the
/// evidence that the bound is not doing any work it should not.
pub const ONE_DEVICE_CODE: Tolerance = Tolerance::new(
    1.5259e-5,
    "one 16-bit device code (1/65535). The device side of every LUT tag here is stored at \
     16-bit resolution, so a genuine dependence on an input the profile is said to ignore \
     would have to be at least one code; below one code it cannot be stored and must be the \
     f64 arithmetic of the interpolation weights. Arithmetic-resolution, NOT perceptual",
);

/// **Exactly zero**, for the channels the published statement says carry *no*
/// colorant.
///
/// This is not a measurement bound and there is no noise for it to absorb. The
/// off-colorant channels of a `Probev1` `BToA` tag hold CLUT samples that are
/// authored zero; interpolation is a sum of products of those zeros with
/// non-negative weights, and the output curve is then evaluated at zero. The
/// IEEE-754 result is exactly `0.0` — **measured exactly `0.0`**, not
/// approximately.
///
/// A bound above zero here would admit a *structurally different answer* (the
/// wrong tag, a transposed colorant, a CLUT read at the wrong stride) rather
/// than measurement noise. **Selecting the wrong `B2A` tag moves this quantity
/// from 0 to about 0.5** — there is no near miss to protect.
pub const ZERO_COLORANT: Tolerance = Tolerance::new(
    0.0,
    "EXACTLY zero, and not a noise bound: the off-colorant CLUT samples are authored zero, \
     interpolation is a sum of products of zeros with non-negative weights, and the output \
     curve is evaluated at zero - the IEEE-754 result is exactly 0.0. A bound above zero \
     would admit a structurally different answer (wrong tag, transposed colorant, wrong \
     CLUT stride), never noise; the wrong-tag answer is about 0.5, so there is no near miss",
);

/// **Agreement with `transicc` on a PCS `L*` read out of the same tag.**
///
/// Terms, all of them the oracle's, at pin `21c582a`:
///
/// | term | size in `L*` | source |
/// |---|---|---|
/// | one legacy 16-bit Lab code | `100/65280 = 1.5319e-3` | 6.3.4.2 NOTE 3 (`mft2`) |
/// | one v4 PCSLAB code | `100/65535 = 1.5259e-3` | clause 10.13 (`mAB `) |
/// | `transicc` prints `L*` to 4 decimals | `+/-5e-5` | README section 9 |
///
/// One code plus the print floor is `1.58e-3`; **`2e-3`** is that rounded up.
///
/// **What it is sensitive enough to catch** — the test that matters: reading a
/// `mAB ` tag with the *legacy* PCSLAB decode instead of the v4 one (the single
/// richest source of CMM bugs, `ARCHITECTURE.md` section 2) moves `L*` at white
/// by about `0.39`, which is **195x** this bound.
///
/// ★ **This row must be run against `*Lab2`, not `*Lab4`.** lcms2 forces black
/// point compensation on when the **destination** profile is v4 and the intent
/// is perceptual or saturation (Pass 4b finding 2). Measured on
/// `Probev1_ICCv4` into `*Lab4`: perceptual reads `67.5743` where `*Lab2` reads
/// `67.7359` — a `0.16` shift that would have been attributed to iccce. The
/// Pass 4c lesson applies unchanged: **a fixture that keeps the gate shut beats
/// a model that subtracts what the gate did.**
pub const ORACLE_LAB: Tolerance = Tolerance::new(
    2e-3,
    "one 16-bit PCS Lab code (100/65280 = 1.5319e-3 legacy per 6.3.4.2 NOTE 3, \
     100/65535 = 1.5259e-3 v4 per clause 10.13) plus transicc's 4-decimal print floor \
     (5e-5) = 1.58e-3, rounded up to 2e-3. Arithmetic-agreement, NOT perceptual. \
     Sensitivity: decoding a mAB PCS with the LEGACY encoding instead of the v4 one moves \
     L* at white by 0.39, which is 195x this bound. Run against *Lab2 so lcms2's forced \
     BPC (destination-version gated) stays shut",
);

// ★★★ WITHDRAWN 2026-08-17: `SEVEN_CORNER`, an end-to-end device-corner bound
// of 5e-5 for the 7-channel comparison. It is recorded here as a comment rather
// than deleted, because a bound that was written, failed, and was **retired**
// teaches something a bound that never existed does not.
//
// **Its derivation was:** at a device-cube corner every interpolation scheme
// returns the stored node, so the (unlegislated, A16) method difference is
// identically zero; what remains is lcms2's quantisation — CLUT input rounded
// to `u16` (7.63e-6), CLUT evaluated in s15.16 (1.53e-5), "the destination's
// 16-bit reverse tone curve" (1.53e-5), `transicc`'s 4-decimal print in 0..255
// (1.96e-7) — summing to 3.82e-5, rounded up to 5e-5.
//
// **It failed at 1.191176e-4, 2.4x over.** The first question was whether the
// code was wrong. It was not:
//
// - the destination chosen for the row, `sRGB2014.icc`, carries **1024-entry
//   tabulated `curv` TRCs**, and lcms2 inverts a tabulated curve by building a
//   **4096-entry reverse tone curve** (`cmsgamma.c`; Pass 4b measured the same
//   term and collapsed a residual 457x by reproducing it). The line "the
//   destination's 16-bit reverse tone curve, 1.53e-5" **assumed an analytic
//   inverse** and is simply the wrong term for this destination;
// - re-run on the **PCS** side, where the destination is not in the loop at
//   all, the same 128 corners agree to **4.900435e-5 `L*`** — 40x inside
//   [`ORACLE_LAB`]. The disagreement was never in the 7-channel path.
//
// **So the bound omitted the dominant term, which is the same failure Pass G
// §2 and Pass 4b's `DEVICE_B2A` both record.** The response was NOT to widen
// 5e-5 until the observation fitted: it was to move the graded claim to the
// PCS side, where the subject of the row (a seven-channel `mAB ` tag) is
// isolated, and to REPORT the end-to-end device numbers with the missing term
// named. Widening would have produced a green line whose justification still
// did not mention the biggest thing in it.

// ===========================================================================
// The row registry - one place, so the SKIP path and the RUN path cannot drift
// ===========================================================================

/// Every non-Probe row Pass H emits, with the kind and tolerance it emits
/// under.
///
/// ## Why this is a table rather than literals at each call site
///
/// A section that emits `n` graded rows when it runs and `m` skip rows when it
/// cannot is a section whose coverage silently changes with the machine. Both
/// paths are generated from **this one list**, so "the corpus is absent"
/// produces exactly the same row identities as "the corpus is present", and a
/// reader diffing two runs sees only outcomes change.
const ROWS: &[(&str, Kind, Tolerance)] = &[
    // --- section A: the version gate ------------------------------------
    (
        "passh/A/refusal/every-iccMAX-file-is-refused-by-name-with-its-own-version",
        Kind::DerivedExpectation,
        EXACT_COUNT,
    ),
    (
        "passh/A/refusal/stdout-is-empty-nothing-was-parsed-anyway",
        Kind::DerivedExpectation,
        EXACT_COUNT,
    ),
    (
        "passh/A/control/the-version-word-ALONE-produces-the-same-refusal",
        Kind::DerivedExpectation,
        EXACT_COUNT,
    ),
    (
        "passh/A/gate/harness-reading-of-byte-8-predicts-iccce-on-every-file",
        Kind::DerivedExpectation,
        EXACT_COUNT,
    ),
    // --- section B: the acceptance population ---------------------------
    (
        "passh/B/acceptance/every-non-iccMAX-file-is-accepted-exit-0",
        Kind::DerivedExpectation,
        EXACT_COUNT,
    ),
    (
        "passh/B/acceptance/no-VIOLATION-is-disclosed-on-any-accepted-file",
        Kind::DerivedExpectation,
        EXACT_COUNT,
    ),
    // ★ REPORTED sibling, added 2026-08-21 with the repointing above.
    // The disclosure count it carries used to BE the graded row; it is
    // kept so the corpus fact survives the fix rather than being tidied
    // away by it.
    (
        "passh/B/acceptance/lawful-disclosures-on-accepted-files",
        Kind::DerivedExpectation,
        REPORTED,
    ),
    (
        "passh/B/acceptance/iccce-and-lcms2-reach-the-same-verdict-on-every-file",
        Kind::CrossCheck,
        EXACT_COUNT,
    ),
    (
        "passh/B/acceptance/header-fields-iccce-printed-match-the-raw-bytes",
        Kind::DerivedExpectation,
        EXACT_COUNT,
    ),
    // --- section C: the N-channel population ----------------------------
    (
        "passh/C/7clr/shipped-binary-converts-a-7-channel-source",
        Kind::DerivedExpectation,
        EXACT_COUNT,
    ),
    // ★ the GRADED 7-channel row, on the PCS side, where the subject is
    // isolated from the destination's tone curve. See the withdrawn
    // `SEVEN_CORNER` comment above for how it got here.
    (
        "passh/C/7clr/pcs-corners-vs-lcms2",
        Kind::CrossCheck,
        ORACLE_LAB,
    ),
    (
        "passh/C/7clr/end-to-end-device-corners-vs-lcms2",
        Kind::CrossCheck,
        REPORTED,
    ),
    (
        "passh/C/7clr/end-to-end-device-interior-vs-lcms2",
        Kind::CrossCheck,
        REPORTED,
    ),
    // ★★ The four compiled-path rows. They were ONE row until 2026-08-17,
    // and the split is not cosmetic — see the block comment above C4 in
    // `section_c` for why a single row stopped being load-bearing the moment
    // the defect it found was fixed.
    (
        "passh/C/7clr/compiled-path-does-not-ABORT-the-process",
        Kind::DerivedExpectation,
        EXACT_COUNT,
    ),
    (
        "passh/C/7clr/default-grid-BUILDS-and-is-the-grid-the-library-RECOMMENDS",
        Kind::DerivedExpectation,
        EXACT_COUNT,
    ),
    (
        "passh/C/7clr/oversized-grid-is-a-NAMED-refusal",
        Kind::DerivedExpectation,
        EXACT_COUNT,
    ),
    (
        "passh/C/7clr/compiled-vs-reference-at-the-default-grid",
        Kind::SelfConsistency,
        REPORTED,
    ),
    (
        "passh/C/coverage/6clr-evidence-is-zero",
        Kind::DerivedExpectation,
        REPORTED,
    ),
    // --- section E: coverage ---------------------------------------------
    (
        "passh/E/coverage/population-breakdown",
        Kind::DerivedExpectation,
        REPORTED,
    ),
];

/// The three Probe profiles section D grades, with the short name its row ids
/// use and the [`Kind`] its rows carry.
///
/// ★ **Only `Probev2_ICCv4` is [`Kind::GroundTruth`]**, because only that file
/// is named by the published readme. See caveat 1 in the module header for why
/// the other two are graded against the same statements but cannot claim the
/// same evidence class.
const PROBES: &[(&str, &str, Kind)] = &[
    (file::PROBE_V1_V2, "probe-v1-icc-v2", Kind::DerivedExpectation),
    (file::PROBE_V1_V4, "probe-v1-icc-v4", Kind::DerivedExpectation),
    (file::PROBE_V2_V4, "probe-v2-icc-v4", Kind::GroundTruth),
];

/// Section D's per-profile row suffixes and their tolerances.
const PROBE_ROWS: &[(&str, Tolerance)] = &[
    ("b2a/off-colorant-channels-are-exactly-zero", ZERO_COLORANT),
    ("b2a/a-and-b-are-ignored", ONE_DEVICE_CODE),
    ("b2a/tint-is-monotone-decreasing-in-L", ONE_DEVICE_CODE),
    (
        "b2a/the-published-colorant-dominates-at-every-point",
        EXACT_COUNT,
    ),
    (
        "a2b/the-three-published-bands-are-disjoint-and-ordered",
        EXACT_COUNT,
    ),
    ("a2b/published-band-containment", REPORTED),
    ("a2b/vs-lcms2-through-the-same-tags", ORACLE_LAB),
    ("a2b/encoded-pcs-clamp-divergence", REPORTED),
    (
        "shipped/intent-selects-the-published-colorant",
        SHIPPED_PRINT_FLOOR,
    ),
    ("tags/mpet-is-present-and-NOT-decoded", EXACT_COUNT),
    ("tags/mpet-selection-divergence-from-lcms2", REPORTED),
];

/// Every row identity Pass H can emit, in emission order.
fn all_row_ids() -> Vec<(String, Kind, Tolerance)> {
    let mut out: Vec<(String, Kind, Tolerance)> = ROWS
        .iter()
        .map(|(id, k, t)| ((*id).to_string(), *k, *t))
        .collect();
    for (_, short, kind) in PROBES {
        for (suffix, tol) in PROBE_ROWS {
            out.push((format!("passh/D/{short}/{suffix}"), *kind, *tol));
        }
    }
    out
}

/// The four conditions a refusal must satisfy — an **authored** design
/// constant, not a measured one.
///
/// (1) exit status exactly 1; (2) the message contains the string `iccMAX`;
/// (3) the message quotes the profile's **own** version word; (4) `stdout` is
/// empty. [`FileVerdict::refusal_defects`] returns the ones that failed, so a
/// file that was **accepted** fails all four.
///
/// A typed design number is safe in a way a typed measurement is not: it is a
/// premise the code enforces, and it fails loudly if the code stops matching
/// it. A typed *measurement* rots silently (DL-034).
const REFUSAL_CONDITIONS: f64 = 4.0;

/// ★ Where section D deliberately stops claiming, and why that is not tuning.
///
/// ## The measurement that forced this function to exist
///
/// The readme's sentence *"The B2A0 tag … renders the L\* values as tints of
/// **pure cyan**"* is **true of `Probev1_ICCv2` and `Probev1_ICCv4` to the last
/// bit** — their off-colorant channels are exactly `0.0` and their output does
/// not depend on `a*`/`b*` beyond `f64` noise — and **false of
/// `Probev2_ICCv4`, the file the readme actually names.** That profile's
/// `BToA` tags produce a near-neutral CMYK build with the intent's colorant
/// *raised*, not a pure single-colorant ramp. Both implementations were asked;
/// see the module header.
///
/// ## What is done about it, and what is deliberately NOT done
///
/// Once a published premise is shown false, continuing to grade iccce against
/// it grades iccce against the document's error. So on `Probev2_ICCv4` the
/// three rows that depend on the *strict* form of the sentence are emitted
/// **[`REPORTED`] — tolerance infinity, claiming nothing** — with the measured
/// value in the detail.
///
/// ★★ **They are relaxed to infinity, not to a finite number the observation
/// happens to satisfy.** A bound of `0.98` chosen because the measurement came
/// out at `0.9791` would be exactly the tuning this whole document family
/// exists to prevent, and it would read in a report as a claim. Infinity reads
/// as what it is: *this pass measured the quantity and declines to gate it.*
///
/// The claim that **survives** on that file is the weaker one the sentence
/// still entails — *the named colorant is the dominant chromatic channel* —
/// and it is graded, at zero violations, in
/// `b2a/the-published-colorant-dominates-at-every-point`. That row is the one
/// that catches an intent-to-tag mis-wiring, which is what section D is for.
fn probe_tolerance(short: &str, suffix: &str, default: Tolerance) -> Tolerance {
    if is_falsified_claim(short, suffix) {
        REPORTED
    } else {
        default
    }
}

/// `true` for the (profile, row) pairs where the readme's statement is
/// **demonstrably false of the file the readme names**. See
/// [`probe_tolerance`].
fn is_falsified_claim(short: &str, suffix: &str) -> bool {
    short == "probe-v2-icc-v4"
        && matches!(
            suffix,
            "b2a/off-colorant-channels-are-exactly-zero"
                | "b2a/a-and-b-are-ignored"
                | "b2a/tint-is-monotone-decreasing-in-L"
        )
}

/// ★★ The banner that goes at the FRONT of a row whose published claim is
/// false.
///
/// A row that reads `PASS ground-truth inf 9.969315e-1` is a trap: the `PASS`
/// is the tolerance being infinite, the `ground-truth` is the *provenance of
/// the expectation*, and neither says that **the expectation was not met**. A
/// reader skimming a report must not be able to quote that line as
/// confirmation of the published statement. This prefix makes that impossible
/// to do by accident.
fn falsified_prefix(short: &str, suffix: &str) -> &'static str {
    if is_falsified_claim(short, suffix) {
        "★★★ THE PUBLISHED CLAIM IS FALSE OF THIS FILE, AND THIS ROW'S `PASS` MEANS ONLY THAT ITS \
         TOLERANCE IS INFINITE. The ICC's readme names Probev2_ICCv4.icc and says its BToA tags \
         render pure single-colorant tints; measurement says they render a near-neutral CMYK \
         build with the intent's colorant raised. Both implementations were asked. What survives \
         - and IS graded - is the dominance row. || "
    } else {
        ""
    }
}

/// The RGB source points section D drives the shipped binary from: a 3x3x3
/// cube, 27 points, spanning the source's own lightness range.
fn shipped_source_rows() -> Vec<Vec<f64>> {
    let s = [0.0_f64, 0.5, 1.0];
    let mut out = Vec::with_capacity(27);
    for r in s {
        for g in s {
            for b in s {
                out.push(vec![r, g, b]);
            }
        }
    }
    out
}

/// The 2^n corners of an n-channel device cube — the only points at which the
/// interpolation-method difference is identically zero, and therefore the only
/// points [`SEVEN_CORNER`] may be quoted for.
pub fn device_corners(channels: usize) -> Vec<Vec<f64>> {
    (0..(1_usize << channels))
        .map(|m| {
            (0..channels)
                .map(|c| f64::from(u8::from(m >> c & 1 == 1)))
                .collect()
        })
        .collect()
}

/// A deterministic interior grid for the 7-channel comparison: values chosen so
/// that **no coordinate lands on a CLUT node for any plausible grid size**, so
/// the row reports the interpolation-method difference rather than hiding it.
///
/// `0,37` and `0,61` are irrational-looking thirds-and-fifths avoiders: for a
/// grid of `g` nodes a coordinate lands on a node only when `x*(g-1)` is an
/// integer, and neither value does that for any `g` from 2 to 64.
fn interior_grid_7() -> Vec<Vec<f64>> {
    let a = [0.37_f64, 0.61];
    let mut out = Vec::new();
    for i in 0..(1_usize << 7) {
        out.push((0..7).map(|c| a[i >> c & 1]).collect());
    }
    out
}

// ===========================================================================
// run
// ===========================================================================

/// Run Pass H. Returns the analysis (for the report's `note` line) and one
/// [`Record`] per row of [`all_row_ids`], in that order, whatever happened.
///
/// **Nothing here propagates an error.** A missing corpus, an unbuilt binary
/// or an oracle failure produces *labelled* `SKIP`/`ERROR` records, because a
/// section that emits nothing when it cannot run is indistinguishable, in a
/// log, from a section that was never wired up.
pub fn run(oracle: &Oracle) -> (Option<Analysis>, Vec<Record>) {
    let dir = corpus_dir();

    let iccce = match Iccce::locate() {
        Ok(Some(i)) => i,
        Ok(None) => {
            return (
                None,
                skip_everything(
                    &dir,
                    "the shipped iccce binary is not built on this machine \
                     (cargo build --release -p iccce-cli)",
                ),
            );
        }
        Err(e) => {
            return (
                None,
                skip_everything(&dir, format!("could not locate the shipped binary: {e}")),
            );
        }
    };

    if !dir.is_dir() {
        return (
            None,
            skip_everything(
                &dir,
                format!(
                    "the private color.org corpus is absent at {} — set $ICCCE_PRIVATE_FIXTURES. \
                     THIS PASS DID NOT RUN; a green suite line on this machine says nothing about it",
                    dir.display()
                ),
            ),
        );
    }

    // --- inventory --------------------------------------------------------
    let mut paths: Vec<PathBuf> = std::fs::read_dir(&dir)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("icc"))
        .collect();
    paths.sort();

    let mut verdicts: Vec<FileVerdict> = Vec::with_capacity(paths.len());
    for path in &paths {
        let bytes = std::fs::read(path).unwrap_or_default();
        let raw = raw_version(&bytes);
        let major = raw.map_or(0, |v| v >> 24);
        let run = match iccce.inspect(path) {
            Ok(r) => r,
            Err(e) => {
                return (
                    None,
                    skip_everything(&dir, format!("could not run the shipped binary: {e}")),
                );
            }
        };
        // The channel count for the oracle probe comes from the harness's own
        // reading of the colour-space signature, never from the parser under
        // test; `None` (an iccMAX spectral space) falls back to three, which is
        // enough for the only question being asked.
        let channels = bytes
            .get(16..20)
            .and_then(|s| channels_of(u32::from_be_bytes(s.try_into().ok()?)))
            .unwrap_or(3) as usize;
        let lcms2 = lcms2_usable(oracle, path, channels).ok();
        verdicts.push(FileVerdict {
            name: path.file_name().unwrap_or_default().to_string_lossy().into_owned(),
            raw_version: raw,
            harness_says_refuse: major >= 5,
            run,
            lcms2_usable: lcms2,
        });
    }

    let accepted = verdicts.iter().filter(|v| v.accepted()).count();
    let refused = verdicts.len() - accepted;

    let mut recs: Vec<Record> = Vec::new();
    section_a(&iccce, &verdicts, &mut recs);
    section_b(&verdicts, &mut recs);
    section_c(&iccce, oracle, &dir, &verdicts, &mut recs);
    section_e(&verdicts, &mut recs);
    section_d(&iccce, oracle, &dir, &mut recs);

    let analysis = Analysis {
        structure: format!(
            "{} .icc files at {} | iccce accepted {accepted}, refused {refused} \
             (bare exit status per file, never through a pipe) | \
             rows {} (run `cargo run --release --bin passh_probe` for the Probe instrument)",
            verdicts.len(),
            dir.display(),
            recs.len()
        ),
        files: verdicts.len(),
        accepted,
        refused,
    };
    (Some(analysis), recs)
}

/// A one-line note for `main.rs`.
#[must_use]
pub fn note(a: &Option<Analysis>) -> String {
    match a {
        None => "did not run (corpus or shipped binary absent) — every row SKIPped".to_string(),
        Some(a) => a.structure.clone(),
    }
}

fn skip_everything(_dir: &Path, reason: impl Into<String>) -> Vec<Record> {
    let reason = reason.into();
    all_row_ids()
        .into_iter()
        .map(|(id, kind, tol)| {
            Record::skipped(
                id,
                kind,
                Metric::AbsMaxComponent,
                tol,
                "the private color.org corpus (see D:\\Dev\\iccce-private-fixtures\\README.md \
                 section `color-org/`) and the ICC's published `Probe2 Profile Readme \
                 June 1, 2007`",
                reason.clone(),
            )
        })
        .collect()
}

/// Build a graded record with this pass's standard provenance string.
fn graded(
    id: &str,
    kind: Kind,
    tol: Tolerance,
    observed: f64,
    source: impl Into<String>,
    detail: impl Into<String>,
) -> Record {
    Record::graded(
        id,
        kind,
        Metric::AbsMaxComponent,
        tol,
        observed,
        source,
        detail,
    )
}

/// The provenance string every section A/B row carries.
const SRC_BYTES: &str = "the expectation is the HARNESS's own reading of the profile version word \
     at byte offset 8 (ICC.1:2022 clause 7.2.4), taken from the file's raw bytes with the parser \
     under test nowhere in the loop, plus iccce's declared scope (ICC v2/v4 only, no iccMAX \
     execution — README/ARCHITECTURE). NO implementation's output is in it";

// ===========================================================================
// Section A — the version gate
// ===========================================================================

/// The refusal population, graded four ways.
///
/// The ten iccMAX members of this corpus were authored by the ICC and by
/// vendors for their own purposes. Nothing in this repository shaped them,
/// which is what makes them a stronger demonstration of `CLAUDE.md` rule 6
/// than the committed synthetic fixture — and the synthetic fixture is
/// nevertheless kept, as the **control** that isolates the cause.
fn section_a(iccce: &Iccce, v: &[FileVerdict], out: &mut Vec<Record>) {
    let expect_refuse: Vec<&FileVerdict> = v.iter().filter(|f| f.harness_says_refuse).collect();
    let n = expect_refuse.len() as f64;

    // --- A1: refused by name, with its own version ------------------------
    let mut defective = 0.0_f64;
    let mut notes = Vec::new();
    for f in &expect_refuse {
        let d = f.refusal_defects();
        if !d.is_empty() {
            defective += 1.0;
            notes.push(format!("{}: {}", f.name, d.join("; ")));
        }
    }
    let names: Vec<&str> = expect_refuse.iter().map(|f| f.name.as_str()).collect();
    out.push(
        graded(
            "passh/A/refusal/every-iccMAX-file-is-refused-by-name-with-its-own-version",
            Kind::DerivedExpectation,
            EXACT_COUNT,
            defective,
            SRC_BYTES,
            format!(
                "{} real ICC-published iccMAX files, each required to satisfy ALL FOUR of: \
                 bare exit status exactly 1; the message contains \"iccMAX\"; the message quotes \
                 the file's OWN version word; stdout empty. Files: {}. Failures: [{}]",
                expect_refuse.len(),
                names.join(", "),
                notes.join(" | ")
            ),
        )
        .with_separation(Separation::against_distance(
            "the refusal is caused by something OTHER than the version word - an unknown \
             colour-space signature, an undecodable spectral tag type, a namedColor2 payload. \
             Under that reading a different error fires, its message names neither \"iccMAX\" \
             nor the version, and every one of these files fails condition 2 and 3",
            n,
            n,
            SepUnits::SameAsMetric,
        ))
        .with_metric(Metric::IndicatorCount),
    );

    // --- A2: nothing was parsed anyway ------------------------------------
    let noisy = expect_refuse
        .iter()
        .filter(|f| !f.run.stdout.trim().is_empty())
        .count() as f64;
    out.push(
        graded(
            "passh/A/refusal/stdout-is-empty-nothing-was-parsed-anyway",
            Kind::DerivedExpectation,
            EXACT_COUNT,
            noisy,
            SRC_BYTES,
            format!(
                "CLAUDE.md rule 6 - the parser reports, it does not repair. A profile iccce \
                 declined to parse must not also have produced a partial header/tag dump that a \
                 caller could mistake for a successful read. {} files checked",
                expect_refuse.len()
            ),
        )
        .with_separation(Separation::none(
            "there is no rival reading of \"how much of a refused profile's content should reach \
             stdout\". Rule 6 admits one answer and its negation is not a position anybody holds; \
             a manufactured alternative here would be worse than none",
        ))
        .with_metric(Metric::IndicatorCount),
    );

    // --- A3: the control that isolates the version word -------------------
    let ctrl = iccmax_control();
    let rec = if !ctrl.is_file() {
        Record::skipped(
            "passh/A/control/the-version-word-ALONE-produces-the-same-refusal",
            Kind::DerivedExpectation,
            Metric::IndicatorCount,
            EXACT_COUNT,
            SRC_BYTES,
            format!("the committed control fixture is missing at {}", ctrl.display()),
        )
    } else {
        match iccce.inspect(&ctrl) {
            Err(e) => Record::errored(
                "passh/A/control/the-version-word-ALONE-produces-the-same-refusal",
                Kind::DerivedExpectation,
                Metric::IndicatorCount,
                EXACT_COUNT,
                SRC_BYTES,
                format!("could not run the shipped binary on the control fixture: {e}"),
            ),
            Ok(r) => {
                let bytes = std::fs::read(&ctrl).unwrap_or_default();
                let f = FileVerdict {
                    name: "fixtures/synthetic/iccmax-version.icc".to_string(),
                    raw_version: raw_version(&bytes),
                    harness_says_refuse: true,
                    run: r,
                    lcms2_usable: None,
                };
                let d = f.refusal_defects();
                graded(
                    "passh/A/control/the-version-word-ALONE-produces-the-same-refusal",
                    Kind::DerivedExpectation,
                    EXACT_COUNT,
                    d.len() as f64,
                    "fixtures/synthetic/iccmax-version.icc - a COMMITTED, category (a) fixture \
                     authored by tools/gen-profiles: an ordinary v2-shaped RGB matrix/TRC profile \
                     whose ONLY iccMAX property is its version word",
                    format!(
                        "★ this row is what lets section A attribute the ten real refusals to \
                         the VERSION rather than to the exotic content those files also carry. \
                         The control has no spectral colour space, no iccMAX tag type and no \
                         namedColor2 payload, and it produces the same named refusal. \
                         Failures: [{}]",
                        d.join("; ")
                    ),
                )
                .with_separation(Separation::against_distance(
                    "the ten real refusals are caused by their exotic CONTENT, not by their \
                     version. Under that reading this control - which has none of that content - \
                     would be ACCEPTED, and would then fail all four refusal conditions at once",
                    REFUSAL_CONDITIONS,
                    REFUSAL_CONDITIONS,
                    SepUnits::SameAsMetric,
                ))
                .with_metric(Metric::IndicatorCount)
            }
        }
    };
    out.push(rec);

    // --- A4: the harness's prediction against iccce, over the whole set ----
    let mut wrong = 0.0_f64;
    let mut wrong_names = Vec::new();
    for f in v {
        if f.harness_says_refuse == f.accepted() {
            wrong += 1.0;
            wrong_names.push(f.name.clone());
        }
    }
    let raw_words: std::collections::BTreeSet<String> = v
        .iter()
        .filter(|f| f.harness_says_refuse)
        .map(|f| format!("0x{:08X}", f.raw_version.unwrap_or(0)))
        .collect();
    out.push(
        graded(
            "passh/A/gate/harness-reading-of-byte-8-predicts-iccce-on-every-file",
            Kind::DerivedExpectation,
            EXACT_COUNT,
            wrong,
            SRC_BYTES,
            format!(
                "over all {} files: the harness reads byte 8 itself and predicts REFUSE iff the \
                 major version is >= 5; iccce is then asked. Disagreements: [{}]",
                v.len(),
                wrong_names.join(", ")
            ),
        )
        .with_separation(zero_sep(
            format!(
                "the gate compares the whole 4-byte version WORD against 0x05000000 rather than \
                 the major BYTE alone. Every iccMAX file in this corpus encodes exactly {} \
                 (minor and bugfix zero), so the two readings give identical answers on all {} \
                 files: THIS CORPUS CANNOT DISTINGUISH THEM, and a v5.1 profile would be needed to",
                raw_words.iter().cloned().collect::<Vec<_>>().join("/"),
                v.len()
            ),
            wrong,
        ))
        .with_metric(Metric::IndicatorCount),
    );
}

// ===========================================================================
// Section B — the acceptance population
// ===========================================================================

fn section_b(v: &[FileVerdict], out: &mut Vec<Record>) {
    let expect_accept: Vec<&FileVerdict> = v.iter().filter(|f| !f.harness_says_refuse).collect();

    // --- B1: exit 0 --------------------------------------------------------
    let bad: Vec<String> = expect_accept
        .iter()
        .filter(|f| !f.accepted())
        .map(|f| format!("{} (exit {:?})", f.name, f.run.code))
        .collect();
    out.push(
        graded(
            "passh/B/acceptance/every-non-iccMAX-file-is-accepted-exit-0",
            Kind::DerivedExpectation,
            EXACT_COUNT,
            bad.len() as f64,
            SRC_BYTES,
            format!(
                "{} real profiles published by the ICC and by press-standards bodies, spanning \
                 seven ICC versions and four device classes. Bare exit status per file, never \
                 through a pipe (TOLERANCES.md section 5.6). Failures: [{}]",
                expect_accept.len(),
                bad.join(", ")
            ),
        )
        .with_separation(Separation::none(
            "the expectation is binary and its only rival is its negation. \"iccce should refuse \
             some of these\" is not a reading anybody holds: every one is published as a \
             conformant ICC v2/v4 profile and iccce's declared scope is ICC v2/v4",
        ))
        .with_metric(Metric::IndicatorCount),
    );

    // --- B2: violations, and the disclosure count reported beside it -------
    //
    // ★★★ REPOINTED 2026-08-21, and the bound did NOT move. It was `0`
    // before and it is `0` now. What changed is the QUANTITY: this row
    // graded `malformations:` — a count of DISCLOSURES — and therefore
    // went red against five ICC-PUBLISHED profiles (`sRGB2014.icc`,
    // `ITU-RBT709ReferenceDisplay.icc`, `PSOsc-b_paper_v3_FOGRA54.icc`,
    // `PSOuncoated_v3_FOGRA52.icc`, `SC_paper_eci.icc`), each disclosing
    // one `HeaderReservedNonZero` because they are **v2** files carrying
    // an MD5 in bytes 84..99 where **v4** later placed `profileID`.
    //
    // ★★ The old row's own text offered two hypotheses — "either iccce
    // over-reports or a published ICC profile is defective" — and the
    // answer was NEITHER. ICC.1:2001-04 Table 9's cell is the
    // unmodalised "44 bytes reserved for future expansion", the only
    // mention in the document, so a v2 file breaches no `shall` by using
    // it. `Malformation::violation_status` had said so since DL-063; the
    // CLI simply printed no number carrying that judgement, so this row
    // had nothing to grade but the mixed count.
    //
    // ★ THIS IS NOT RULE 5 BEING BENT. Widening would be moving `0` to
    // `5` and leaving the subject alone. This moves the SUBJECT to the
    // quantity the claim was always about and leaves the bound at zero —
    // and the old quantity is not discarded, it is REPORTED below, so
    // the corpus fact (five published files use reserved space) stays
    // visible rather than being tidied away by the fix.
    let mut violations_total = 0.0_f64;
    let mut violators = Vec::new();
    let mut disclosed_total = 0u32;
    let mut disclosers = Vec::new();
    let mut silent = Vec::new();
    for f in &expect_accept {
        // A file the parser accepted must print BOTH counts. A missing
        // line is a defect in its own right and is counted, never read
        // as zero.
        match (f.malformations(), f.violations()) {
            (Some(m), Some(v)) => {
                if v > 0 {
                    violations_total += f64::from(v);
                    violators.push(format!("{} ({v})", f.name));
                }
                if m > 0 {
                    disclosed_total += m;
                    disclosers.push(format!("{} ({m})", f.name));
                }
            }
            _ => silent.push(f.name.clone()),
        }
    }
    out.push(
        graded(
            "passh/B/acceptance/no-VIOLATION-is-disclosed-on-any-accepted-file",
            Kind::DerivedExpectation,
            EXACT_COUNT,
            violations_total + silent.len() as f64,
            "the expectation is that a profile published as conformant breaches no STATED \
             requirement of the edition it declares. ★ It is NOT derived from any implementation. \
             It is materially better argued than the disclosure count this row graded until \
             2026-08-21: a disclosure can be lawful (ICC.1:2001-04 Table 9 attaches no modal verb \
             to the v2 reserved block), whereas a violation names the `shall` it breaks",
            format!(
                "★ a non-zero is STILL a FINDING to ADJUDICATE - either iccce over-accuses or a \
                 published ICC profile breaches a stated requirement - and it is NEVER answered \
                 by widening the bound (CLAUDE.md rule 5). {} files; violating: [{}]; files that \
                 printed no `malformations:`/`violations:` pair at all (itself a defect, \
                 counted): [{}]",
                expect_accept.len(),
                violators.join(", "),
                silent.join(", ")
            ),
        )
        .with_separation(Separation::none(
            "there is no rival READING of this quantity. A non-zero result is an adjudication \
             between two hypotheses - iccce over-accuses, or a published profile breaches a \
             stated requirement - and this corpus cannot settle it alone; a second parser's \
             violation list would be needed, and transicc does not emit one",
        ))
        .with_metric(Metric::IndicatorCount),
    );
    // ★ The old quantity, kept and REPORTED rather than deleted. Five
    // ICC-published files using the v2 reserved block is a real fact
    // about the corpus and it should not vanish because it stopped being
    // a failure. Ungraded on purpose: nothing guarantees a published
    // file has nothing to disclose, which is exactly the weakness the
    // superseded row's own justification confessed to.
    out.push(
        graded(
            "passh/B/acceptance/lawful-disclosures-on-accepted-files",
            Kind::DerivedExpectation,
            REPORTED,
            f64::from(disclosed_total),
            "REPORTED, never graded, and it must stay that way: NOTHING guarantees a published \
             file has nothing to disclose. That was the confessed weakness of the graded row this \
             replaced, and promoting this one to graded would reinstate the same defect under a \
             new id",
            format!(
                "{} of {} accepted files disclose {} observations between them, of which {} are \
                 VIOLATIONS (graded on the row above). Disclosing: [{}]. ★ A disclosure is not an \
                 accusation - see DL-063 and `iccce_profile::diag::Malformation`. This row exists \
                 so that the corpus fact survives the 2026-08-21 repointing instead of being \
                 tidied away by it",
                disclosers.len(),
                expect_accept.len(),
                disclosed_total,
                violations_total,
                disclosers.join(", ")
            ),
        )
        .with_separation(Separation::none(
            "an ungraded baseline has no rival reading to separate from: it records what the \
             corpus contains, and any value it takes is the answer rather than evidence for one \
             hypothesis over another",
        ))
        .with_metric(Metric::IndicatorCount),
    );

    // --- B3: the cross-check against lcms2 ---------------------------------
    let mut disagree = Vec::new();
    let mut unknown = 0usize;
    for f in v {
        match f.lcms2_usable {
            None => unknown += 1,
            Some(l) => {
                if l != f.accepted() {
                    disagree.push(format!(
                        "{} (iccce {}, lcms2 {})",
                        f.name,
                        if f.accepted() { "accepts" } else { "refuses" },
                        if l { "uses" } else { "declines" }
                    ));
                }
            }
        }
    }
    out.push(
        graded(
            "passh/B/acceptance/iccce-and-lcms2-reach-the-same-verdict-on-every-file",
            Kind::CrossCheck,
            EXACT_COUNT,
            disagree.len() as f64,
            "both sides computed in this run: iccce's bare exit status, and whether transicc \
             produced any numbers. ★ transicc EXITS 0 even when it prints \"Couldn't link the \
             profiles\" and converts nothing (measured 2026-08-17), so its exit code is NOT the \
             observable and a row that used it would have recorded lcms2 as accepting every \
             iccMAX file",
            format!(
                "{} files, {} with no oracle verdict. Disagreements: [{}]",
                v.len(),
                unknown,
                disagree.join(", ")
            ),
        )
        .with_separation(Separation::against_distance(
            "★ MEASURED, not hypothetical: on fixtures/synthetic/iccmax-version.icc - a file \
             whose only iccMAX property is its version word - the two implementations DISAGREE. \
             lcms2 builds a transform from it; iccce refuses it by name. So lcms2 does not gate \
             on the version at all, and its refusal of the ten real files is caused by their \
             content. This row would observe 1 on that file",
            1.0,
            1.0,
            SepUnits::SameAsMetric,
        ))
        .with_metric(Metric::IndicatorCount),
    );

    // --- B4: printed header fields against the raw bytes -------------------
    let mut mismatch = Vec::new();
    for f in &expect_accept {
        let Some(printed) = f.run.field("header.version") else {
            mismatch.push(format!("{}: no header.version line", f.name));
            continue;
        };
        let want = format!("0x{:08X}", f.raw_version.unwrap_or(0));
        if !printed.contains(&want) {
            mismatch.push(format!("{}: printed {printed}, bytes say {want}", f.name));
        }
    }
    out.push(
        graded(
            "passh/B/acceptance/header-fields-iccce-printed-match-the-raw-bytes",
            Kind::DerivedExpectation,
            EXACT_COUNT,
            mismatch.len() as f64,
            SRC_BYTES,
            format!(
                "\"it parsed\" is a much weaker claim than \"it parsed and read the right \
                 bytes\". The harness reads bytes 8..12 itself and requires the CLI's own \
                 `header.version` line to quote the same word. {} files. Mismatches: [{}]",
                expect_accept.len(),
                mismatch.join(", ")
            ),
        )
        .with_separation(Separation::none(
            "the version word is a fixed-position big-endian u32 (clause 7.2.4); there is no \
             second reading of four bytes at a known offset",
        ))
        .with_metric(Metric::IndicatorCount),
    );
}

// ===========================================================================
// Section E — coverage, reported and never graded
// ===========================================================================

fn section_e(v: &[FileVerdict], out: &mut Vec<Record>) {
    use std::collections::BTreeMap;
    let mut ver: BTreeMap<String, usize> = BTreeMap::new();
    let mut cls: BTreeMap<String, usize> = BTreeMap::new();
    let mut spc: BTreeMap<String, usize> = BTreeMap::new();
    for f in v.iter().filter(|f| f.accepted()) {
        *ver.entry(
            f.run
                .field("header.version")
                .unwrap_or("?")
                .split_whitespace()
                .next()
                .unwrap_or("?")
                .to_string(),
        )
        .or_default() += 1;
        *cls.entry(f.run.field("header.class").unwrap_or("?").to_string())
            .or_default() += 1;
        *spc.entry(f.run.field("header.colorspace").unwrap_or("?").to_string())
            .or_default() += 1;
    }
    out.push(
        graded(
            "passh/E/coverage/population-breakdown",
            Kind::DerivedExpectation,
            REPORTED,
            0.0,
            "computed at run time from this machine's copy of the corpus",
            format!(
                "★ COVERAGE, STATED. Accepted population of THIS corpus (color-org): versions \
                 {ver:?}; classes {cls:?}; colour spaces {spc:?}. ★ WHAT IS ABSENT FROM THIS \
                 CORPUS and must not be inferred: there is NO GRAY profile, NO Lab-colour-space \
                 profile and NO XYZ-colour-space profile in the accepted population - the three \
                 files named `D50/D55/D65_XYZ.icc` declare colourSpace 'RGB ', not 'XYZ '. Any \
                 claim of GRAY/Lab/XYZ coverage FROM THIS CORPUS is false. \
                 ★★ THE DENOMINATOR IS NAMED BECAUSE A SECOND CENSUS EXISTS AND LOOKS LIKE A \
                 RIVAL CLAIM. Across BOTH private corpora - color-org (40 accepted) plus \
                 ghent-v50 (20, Pass G) - the engineer's own sweep gives CMYK 33, RGB 25, GRAY 1, \
                 7CLR 1 = 60 (NUMERIC_CLAIMS.md NC-220). That RECONCILES with this row exactly: \
                 23+16+1 = 40 here, 10+9+1 = 20 there. The single GRAY profile is in ghent-v50, \
                 NOT here. ■ There is therefore no contradiction between the two counts, only two \
                 different populations - and a coverage number quoted without its corpus is not a \
                 coverage number. iccce DOES have GRAY evidence; Pass H is not where it lives"
            ),
        )
        .with_separation(Separation::none(
            "a coverage census has no rival candidate; it is a count of what is on the disk",
        ))
        .with_metric(Metric::IndicatorCount),
    );
}

// ===========================================================================
// Section C — the N-channel population
// ===========================================================================

/// The corpus's `7CLR` profile is the **first device space with more than four
/// channels this project has ever been given**, and its `6CLR` profile is
/// unreachable behind the version gate. Both facts are graded or reported
/// here; neither is inferred.
fn section_c(
    iccce: &Iccce,
    oracle: &Oracle,
    dir: &Path,
    v: &[FileVerdict],
    out: &mut Vec<Record>,
) {
    let src = dir.join(file::SEVEN);
    let dst = dir.join(file::SRGB);
    // ★ The skip carries the row's OWN `Kind` from the registry rather than a
    // hard-coded `DerivedExpectation`. A skipped row is still a row in the
    // emitted evidence, and a skip filed under the wrong evidence class is a
    // small lie in the same column a reader uses to weigh the rest.
    let skip = |id: &str, kind: Kind, tol: Tolerance, why: String| {
        let metric = if id.ends_with("compiled-vs-reference-at-the-default-grid") {
            Metric::DeviceAbsMaxNormalised
        } else {
            Metric::IndicatorCount
        };
        Record::skipped(id, kind, metric, tol, SRC_BYTES, why)
    };

    if !src.is_file() || !dst.is_file() {
        for (id, kind, tol) in ROWS.iter().filter(|(id, _, _)| id.contains("/C/7clr/")) {
            out.push(skip(
                id,
                *kind,
                *tol,
                format!(
                    "the 7-channel profile or its destination is not present in {}",
                    dir.display()
                ),
            ));
        }
    } else {
        // --- C1: the shipped binary converts a 7-channel source ------------
        let corners = device_corners(7);
        let got = iccce.transform_rows_shaped(&src, &dst, Intent::RelativeColorimetric, &corners, 3);
        match &got {
            Err(e) => {
                out.push(Record::errored(
                    "passh/C/7clr/shipped-binary-converts-a-7-channel-source",
                    Kind::DerivedExpectation,
                    Metric::IndicatorCount,
                    EXACT_COUNT,
                    SRC_BYTES,
                    format!("the shipped binary did not convert a 7-channel source: {e}"),
                ));
            }
            Ok(rows) => {
                let nonfinite = rows
                    .iter()
                    .flatten()
                    .filter(|x| !x.is_finite())
                    .count() as f64;
                let arity = f64::from(u8::from(rows.len() != corners.len()));
                out.push(
                    graded(
                        "passh/C/7clr/shipped-binary-converts-a-7-channel-source",
                        Kind::DerivedExpectation,
                        EXACT_COUNT,
                        nonfinite + arity,
                        "the shipped binary's own contract: `iccce transform` writes one converted \
                         set per input line (crates/iccce-cli/src/main.rs). No published value is \
                         involved and none is claimed",
                        format!(
                            "★ FIRST EVIDENCE: {} device corners of a SEVEN-channel source \
                             (CMYK+orange+green+violet, mAB A2B with 7 inputs) pushed through the \
                             shipped binary into a matrix/TRC RGB destination. {} rows returned, \
                             {nonfinite} non-finite components. This row says the path RUNS and \
                             returns the right shape; it says nothing about the colour",
                            corners.len(),
                            rows.len()
                        ),
                    )
                    .with_separation(Separation::none(
                        "the claim is liveness and arity; there is no second candidate for \"how \
                         many rows should come back from n input rows\"",
                    ))
                    .with_metric(Metric::IndicatorCount),
                );
            }
        }

        // --- C2: the GRADED row, on the PCS side ---------------------------
        //
        // ★ The subject is a SEVEN-CHANNEL `mAB ` tag, so the comparison is
        // made where that tag is the only thing in the loop. Driving it
        // end-to-end into an RGB destination puts the destination's tone-curve
        // inversion into the residual, and on `sRGB2014.icc` — 1024-entry
        // tabulated `curv` — lcms2's 4096-entry reverse curve is the dominant
        // term. That is a fact about the destination and it belongs in the
        // reported rows below, not in a claim about seven channels.
        let a2b1 = std::fs::read(&src)
            .ok()
            .and_then(|b| Profile::parse(&b).ok())
            .and_then(|p| TagEval::build(&p, Signature(0x4132_4231)));
        let corners7 = device_corners(7);
        let req = Request {
            input: Space::profile(src.clone()),
            output: Space::lab_v2(),
            intent: Intent::RelativeColorimetric,
            precalc: Precalc::Exact,
            bpc: Bpc::Off,
            values: corners7.iter().flatten().map(|x| x * 100.0).collect(),
        };
        let rec = match (a2b1, oracle.convert_batch_shaped(&req, 7, 3)) {
            (Some(ev), Ok(rows)) if rows.len() == corners7.len() => {
                let mut max = 0.0_f64;
                for (q, r) in corners7.iter().zip(&rows) {
                    if let Some(l) = ev.device_to_lab(q) {
                        max = max.max((l.l - r[0]).abs());
                    }
                }
                graded(
                    "passh/C/7clr/pcs-corners-vs-lcms2",
                    Kind::CrossCheck,
                    ORACLE_LAB,
                    max,
                    "both sides computed in this run: iccce-cmm's LutAbModel IN PROCESS on the \
                     profile's own A2B1 tag (the shipped binary has no surface that emits PCS \
                     L*a*b*), and transicc into *Lab2",
                    format!(
                        "★★ THE FIRST GRADED SEVEN-CHANNEL ROW THIS PROJECT HAS EVER HAD. {} \
                         corners of a 7-D device cube (CMYK+orange+green+violet), A2B1 `mAB `, \
                         media-relative, -c0. Corners because there every interpolation scheme \
                         returns the stored node, so the interpolation-method difference - which \
                         ICC.1 does not legislate (A16) and which lcms2's >4-input geometry has \
                         not been read out of the pinned source for - is identically zero. The \
                         end-to-end device rows below carry that term and are reported, not graded",
                        corners7.len()
                    ),
                )
                .with_separation(Separation::none(
                    "no rival READING is in play - two implementations of the same clause on the \
                     same bytes. The interpolation-method difference is a rival METHOD, which \
                     belongs in the tolerance's why and not in a candidate separation (Pass 4c's \
                     rule), and at a corner it is zero in any case",
                ))
                .with_metric(Metric::AbsMaxComponent)
            }
            (ev, r) => Record::errored(
                "passh/C/7clr/pcs-corners-vs-lcms2",
                Kind::CrossCheck,
                Metric::AbsMaxComponent,
                ORACLE_LAB,
                "both sides computed in this run",
                format!(
                    "the PCS-side 7-channel comparison did not run: A2B1 decoded = {}, \
                     oracle = {:?}",
                    ev.is_some(),
                    r.err().map(|e| e.to_string())
                ),
            ),
        };
        out.push(rec);

        // --- C3/C4: end to end, reported ------------------------------------
        for (id, points, tol, what) in [
            (
                "passh/C/7clr/end-to-end-device-corners-vs-lcms2",
                device_corners(7),
                REPORTED,
                "★ REPORTED, NOT GRADED, and the reason is a WITHDRAWN BOUND: this row was \
                 graded at 5e-5 and failed at 1.191176e-4 (measured 2026-08-17, preserved here \
                 as a DATED historical value, not a live one). The code is not wrong - the destination \
                 `sRGB2014.icc` carries 1024-entry TABULATED curv TRCs, and lcms2 inverts a \
                 tabulated curve through a 4096-entry reverse tone curve (cmsgamma.c; Pass 4b \
                 measured the same term). The withdrawn bound's own why said \"the destination's \
                 16-bit reverse tone curve\" and assumed an ANALYTIC inverse, so it omitted the \
                 dominant term. It was retired, not widened",
            ),
            (
                "passh/C/7clr/end-to-end-device-interior-vs-lcms2",
                interior_grid_7(),
                REPORTED,
                "★ REPORTED, NOT GRADED: an INTERIOR grid, where the difference is dominated by \
                 the interpolation method. ICC.1 legislates none (A16), lcms2's >4-input geometry \
                 has not been read out of the pinned source, and no envelope has been computed - \
                 so there is no defensible bound and this pass declines to invent one",
            ),
        ] {
            let mine =
                iccce.transform_rows_shaped(&src, &dst, Intent::RelativeColorimetric, &points, 3);
            // transicc takes ink percentages 0..100 and prints RGB in 0..255.
            let req = Request {
                input: Space::profile(src.clone()),
                output: Space::profile(dst.clone()),
                intent: Intent::RelativeColorimetric,
                precalc: Precalc::Exact,
                bpc: Bpc::Off,
                values: points.iter().flatten().map(|x| x * 100.0).collect(),
            };
            let theirs = oracle.convert_batch_shaped(&req, 7, 3);
            match (mine, theirs) {
                (Ok(a), Ok(b)) if a.len() == b.len() => {
                    let mut max = 0.0_f64;
                    for (ra, rb) in a.iter().zip(&b) {
                        for (x, y) in ra.iter().zip(rb) {
                            max = max.max((x - y / 255.0).abs());
                        }
                    }
                    out.push(
                        graded(
                            id,
                            Kind::CrossCheck,
                            tol,
                            max,
                            "both sides computed in this run: the shipped iccce binary (prints \
                             0..1, 6 decimals) and transicc (prints 0..255, 4 decimals). The \
                             /255 rescale is stated because a number quoted without its scale is \
                             wrong by 255",
                            format!(
                                "{} points, {what}. 7-channel source -> v2 matrix/TRC RGB, \
                                 media-relative, -c0",
                                points.len()
                            ),
                        )
                        .with_separation(Separation::none(
                            "no rival READING is in play - this is two implementations of the same \
                             clause evaluated on the same bytes. The interpolation-method \
                             difference is a rival METHOD, which belongs in the tolerance's why \
                             and not in a candidate separation (the Pass 4c rule)",
                        ))
                        .with_metric(Metric::DeviceAbsMaxNormalised),
                    );
                }
                (a, b) => out.push(Record::errored(
                    id,
                    Kind::CrossCheck,
                    Metric::DeviceAbsMaxNormalised,
                    tol,
                    "both sides computed in this run",
                    format!(
                        "the 7-channel comparison did not produce two comparable grids: \
                         iccce {:?}, oracle {:?}",
                        a.err().map(|e| e.to_string()),
                        b.err().map(|e| e.to_string())
                    ),
                )),
            }
        }

        // ===================================================================
        // C4 — the compiled path, four rows
        // ===================================================================
        //
        // ★★ THIS WAS ONE ROW UNTIL 2026-08-17, AND THE SPLIT IS THE WHOLE
        // LESSON OF THE FIX.
        //
        // The original single row observed: *did `iccce bench` on this file
        // exit with a bare status outside {0, 1}?* On 2026-08-17 at tip
        // `e21154c` it observed **1** — the process aborted — and that was a
        // real defect in shipped code (`docs/TOLERANCES.md` §3.8.4).
        //
        // The engineer fixed it in `crates/iccce-cmm/src/compiled.rs` with two
        // changes, and **each one independently makes the single row green**:
        //
        //   1. a SIZE guard (`ChainError::GridExceedsBudget`, bounded by
        //      `MAX_COMPILED_GRID_BYTES`) distinct from the `checked_pow`
        //      OVERFLOW guard, which converts the abort into a named refusal;
        //   2. a COMPUTED rather than tabulated recommendation for >= 5
        //      channels, which makes the DEFAULT grid small enough that the
        //      guard is never reached on this file at all.
        //
        // ★ That is exactly the condition under which a single row stops being
        // load-bearing: it is now satisfied by (2) alone, so **deleting (1)
        // would leave it GREEN**. Asking of a row "what does it measure" is not
        // enough; the Pass H question is *which layer is in the loop*. Four
        // rows, four different layers:
        //
        //   C4a  the default path does not ABORT           (the historic defect)
        //   C4b  the default path BUILDS, at the grid the library recommends
        //   C4c  an oversized grid is a NAMED REFUSAL      (guard (1) itself)
        //   C4d  what the default grid COSTS, reported     (self-comparison)
        let grid = iccce_cmm::compiled::recommended_grid_points(7);
        let budget = iccce_cmm::compiled::MAX_COMPILED_GRID_BYTES as u128;
        let out_ch: u128 = 3; // the destination is sRGB2014, a matrix/TRC RGB profile
        let nodes = (grid as u128).pow(7);
        let bytes = nodes * out_ch * 8;
        let bench_args = |extra: &[&str]| {
            let mut a = vec![
                "--src".to_string(),
                src.display().to_string(),
                "--dst".to_string(),
                dst.display().to_string(),
                "--pixels".to_string(),
                "10000".to_string(),
            ];
            a.extend(extra.iter().map(|s| (*s).to_string()));
            a
        };
        let st = iccce.bench_status(&bench_args(&[]));

        // --- C4a: the default path must not abort the process --------------
        let rec = match &st {
            Err(e) => Record::errored(
                "passh/C/7clr/compiled-path-does-not-ABORT-the-process",
                Kind::DerivedExpectation,
                Metric::IndicatorCount,
                EXACT_COUNT,
                SRC_BYTES,
                format!("could not run `iccce bench`: {e}"),
            ),
            Ok(r) => {
                // 0 = it worked; 1 = a NAMED refusal, which is the CLI's own
                // documented way of declining. Anything else is neither.
                let bad = f64::from(u8::from(!matches!(r.code, Some(0) | Some(1))));
                graded(
                    "passh/C/7clr/compiled-path-does-not-ABORT-the-process",
                    Kind::DerivedExpectation,
                    EXACT_COUNT,
                    bad,
                    "the shipped CLI's own contract: every decline in this product is a NAMED \
                     refusal on stderr with a non-zero exit (CLAUDE.md rule 6 and the CLI's usage \
                     text). A process abort is neither a result nor a refusal - it is the absence \
                     of both, and a caller cannot distinguish it from a crash in their own code",
                    format!(
                        "★ THE DEFECT THIS ROW FOUND IS FIXED; THE ROW IS KEPT AND ITS SUBJECT \
                         HAS NARROWED. LIVE, computed this run: `iccce bench` on the corpus's \
                         7-channel source exited {:?}, so the indicator (a bare exit outside \
                         {{0, 1}}) observes {bad}. \
                         iccce_cmm::compiled::recommended_grid_points(7) returns {grid}, so \
                         CompiledTransform::new samples {grid}^7 = {nodes} nodes x {out_ch} \
                         outputs x 8 bytes = {bytes} bytes ({}), against \
                         iccce_cmm::compiled::MAX_COMPILED_GRID_BYTES = {budget} bytes ({}). \
                         ■ HISTORICAL, DATED, NOT LIVE - at tip `e21154c` on 2026-08-17 this row \
                         was RED: recommended_grid_points carried a `_ => 33` catch-all, so the \
                         same call sampled 33^7 = 42618442977 nodes x 3 x 8 = 1022842631448 bytes \
                         (0.93 TiB); `checked_pow` guarded WRAP and not SIZE, the allocation was \
                         attempted, and the allocator ABORTED the process - bare exit -1073740791 \
                         (0xC0000409), stderr \"memory allocation of 1022842631448 bytes failed\", \
                         stdout empty. ★★ WHAT THIS ROW NOW PROVES, AND WHAT IT NO LONGER DOES: \
                         it proves the SHIPPED DEFAULT is survivable on a real 7-channel file. It \
                         no longer exercises the size guard at all - at grid {grid} the allocation \
                         is {}, which would succeed with no guard present - so DELETING \
                         MAX_COMPILED_GRID_BYTES would leave this row green. The guard is held by \
                         `passh/C/7clr/oversized-grid-is-a-NAMED-refusal`, which forces the exact \
                         grid that aborted. stderr: {}",
                        r.code,
                        human_bytes(bytes),
                        human_bytes(budget),
                        human_bytes(bytes),
                        show_stderr(&r.stderr),
                    ),
                )
                .with_separation(Separation::against_distance(
                    "the alternative here is not hypothetical and was not modelled - it was \
                     MEASURED. At tip `e21154c` the same command on the same file aborted, and \
                     this indicator observed 1. The rival state is 'the size guard is absent AND \
                     the >=5-channel recommendation is again a `_ => 33` catch-all', which is the \
                     code as it stood on 2026-08-17",
                    1.0,
                    1.0,
                    SepUnits::SameAsMetric,
                ))
                .with_metric(Metric::IndicatorCount)
            }
        };
        out.push(rec);

        // --- C4b: the default path BUILDS, at the recommended grid ---------
        //
        // C4a is satisfied by a polite refusal. That is not what a caller
        // wants from the DEFAULT: a library whose recommended grid always
        // refused would satisfy C4a for ever. This row requires exit 0
        // specifically, and requires the grid the binary actually used to be
        // the grid the library recommends.
        //
        // ★ The second half of that is NC-148's shape one layer up. Pass 6
        // found the harness's `DEFAULT_GRID` and the shipped default drifting
        // apart; the same drift is possible between `recommended_grid_points`
        // and what `iccce bench` does with it, and nothing else in this suite
        // would see it.
        let rec = match &st {
            Err(e) => Record::errored(
                "passh/C/7clr/default-grid-BUILDS-and-is-the-grid-the-library-RECOMMENDS",
                Kind::DerivedExpectation,
                Metric::IndicatorCount,
                EXACT_COUNT,
                SRC_BYTES,
                format!("could not run `iccce bench`: {e}"),
            ),
            Ok(r) => {
                let printed_grid = r.field("grid.points_per_axis").map(str::trim);
                let printed_nodes = r.field("grid.nodes").map(str::trim);
                let v_exit = u8::from(r.code != Some(0));
                let v_grid = u8::from(printed_grid != Some(grid.to_string().as_str()));
                let v_nodes = u8::from(printed_nodes != Some(nodes.to_string().as_str()));
                let v_err = u8::from(!r.stderr.trim().is_empty());
                let violations = f64::from(v_exit + v_grid + v_nodes + v_err);
                graded(
                    "passh/C/7clr/default-grid-BUILDS-and-is-the-grid-the-library-RECOMMENDS",
                    Kind::DerivedExpectation,
                    EXACT_COUNT,
                    violations,
                    "two contracts of the shipped product, neither of them a colour claim. (1) \
                     `iccce bench` with no --grid uses compiled::recommended_grid_points(inputs) \
                     (crates/iccce-cli/src/main.rs) and prints what it used; a recommendation the \
                     binary does not honour is a recommendation that documents nothing. (2) A \
                     DEFAULT that refuses is not a default - C4a is satisfied by a polite refusal \
                     for ever, so this row requires exit 0 and an empty stderr specifically",
                    format!(
                        "★ THE DEFAULT IS USABLE, NOT MERELY NON-FATAL. Violations: exit != 0 \
                         ({v_exit}, saw {:?}); printed grid != recommended_grid_points(7) = {grid} \
                         ({v_grid}, saw {printed_grid:?}); printed nodes != {grid}^7 = {nodes} \
                         ({v_nodes}, saw {printed_nodes:?}); stderr not empty ({v_err}). \
                         ★ This row is the one that would catch the library's recommendation and \
                         the binary's behaviour drifting apart - the failure Pass 6's R1 caught \
                         between the harness's DEFAULT_GRID and the shipped default, one layer up. \
                         ■ NOT A COLOUR CLAIM: that the grid builds says nothing about what it \
                         costs. See `compiled-vs-reference-at-the-default-grid`, which reports the \
                         cost and grades nothing",
                        r.code,
                    ),
                )
                .with_separation(Separation::against_distance(
                    "the named rival is 'the default is small enough not to abort but the binary \
                     refuses it anyway, or silently uses some other grid'. Under that reading at \
                     least one of the four conditions is violated and this row observes >= 1",
                    1.0,
                    (1.0 - violations).abs(),
                    SepUnits::SameAsMetric,
                ))
                .with_metric(Metric::IndicatorCount)
            }
        };
        out.push(rec);

        // --- C4c: an oversized grid is a NAMED refusal, not an abort -------
        //
        // ★★ THIS IS THE ROW THAT KEEPS THE DEFECT FIXED. It re-runs the
        // EXACT configuration that aborted on 2026-08-17 - `--grid 33` on this
        // file - and requires the named refusal that replaced it.
        //
        // Why it is not redundant with the engineer's unit test
        // (`compiled::tests::oversized_grid_arithmetic_is_refused_not_aborted`):
        // that test asserts the GUARD'S ARITHMETIC in process, deliberately
        // without attempting the allocation. It cannot see the CLI wiring -
        // whether `bench` propagates the `Err` as exit 1, whether the message
        // reaches stderr, whether anything partial escapes on stdout. This row
        // is the shipped binary end to end, which is the layer a caller meets.
        //
        // Every number matched below is COMPUTED here, never typed: if the
        // guard's arithmetic changes the row recomputes with it, and if the
        // message stops naming one of the three numbers the row goes red.
        const FORCED: u128 = 33;
        let f_nodes = FORCED.pow(7);
        let f_bytes = f_nodes * out_ch * 8;
        let forced = iccce.bench_status(&bench_args(&["--grid", "33"]));
        let rec = match forced {
            Err(e) => Record::errored(
                "passh/C/7clr/oversized-grid-is-a-NAMED-refusal",
                Kind::DerivedExpectation,
                Metric::IndicatorCount,
                EXACT_COUNT,
                SRC_BYTES,
                format!("could not run `iccce bench --grid 33`: {e}"),
            ),
            Ok(r) => {
                let se = r.stderr.replace(['\u{a0}', ','], "");
                // If the budget ever rose above this allocation the row would
                // be vacuous - it would be testing a grid that now fits. Count
                // that as a violation so the row cannot go quietly hollow.
                let v_vacuous = u8::from(f_bytes <= budget);
                let v_exit = u8::from(r.code != Some(1));
                let v_stdout = u8::from(!r.stdout.trim().is_empty());
                let v_nodes = u8::from(!se.contains(&f_nodes.to_string()));
                let v_bytes = u8::from(!se.contains(&f_bytes.to_string()));
                let v_budget = u8::from(!se.contains(&budget.to_string()));
                let violations =
                    f64::from(v_vacuous + v_exit + v_stdout + v_nodes + v_bytes + v_budget);
                graded(
                    "passh/C/7clr/oversized-grid-is-a-NAMED-refusal",
                    Kind::DerivedExpectation,
                    EXACT_COUNT,
                    violations,
                    "CLAUDE.md rule 6 at the allocation layer, and the shipped CLI's own contract. \
                     A refusal must (a) exit 1, (b) print NOTHING on stdout so no partial result \
                     escapes, and (c) NAME the quantities that caused it - the node count, the \
                     byte count and the budget - because a refusal a caller cannot act on is only \
                     a politer abort. All three numbers are computed by this row, not typed, so \
                     the row tracks the guard's arithmetic instead of freezing yesterday's",
                    format!(
                        "★★ THE REGRESSION DETECTOR FOR THE 2026-08-17 DEFECT. Re-runs the exact \
                         configuration that aborted: `iccce bench --grid 33` on the 7-channel \
                         source. Required: exit 1, empty stdout, and stderr naming {f_nodes} \
                         nodes, {f_bytes} bytes ({}) and the {budget}-byte budget ({}). \
                         Violations: budget no longer exceeded so the row would be vacuous \
                         ({v_vacuous}); exit != 1 ({v_exit}, saw {:?}); stdout not empty \
                         ({v_stdout}); node count absent ({v_nodes}); byte count absent \
                         ({v_bytes}); budget absent ({v_budget}). ★ NOT REDUNDANT with \
                         crates/iccce-cmm's own \
                         `compiled::tests::oversized_grid_arithmetic_is_refused_not_aborted`: that \
                         test asserts the guard's arithmetic IN PROCESS and deliberately never \
                         attempts the allocation, so it is blind to the CLI wiring - exit code, \
                         stream routing, stdout suppression - which is the layer a caller meets. \
                         stderr: {}",
                        human_bytes(f_bytes),
                        human_bytes(budget),
                        r.code,
                        show_stderr(&r.stderr),
                    ),
                )
                .with_separation(Separation::against_distance(
                    "the named rival is the code as it stood at tip `e21154c`: no size guard, so \
                     this allocation is attempted and the process dies. Under that reading exit is \
                     not 1 and stderr names none of the three numbers, so the row observes at \
                     least 4 - and, being an abort, it observes them by not returning at all",
                    4.0,
                    (4.0 - violations).abs(),
                    SepUnits::SameAsMetric,
                ))
                .with_metric(Metric::IndicatorCount)
            }
        };
        out.push(rec);

        // --- C4d: what the default grid COSTS, reported and never graded ---
        //
        // ★ The weakest evidence class in this file, labelled as such. Both
        // arms are iccce: `iccce bench` prints the maximum device-space
        // difference between the COMPILED grid and the REFERENCE chain, off
        // the grid nodes. Same code, same profile, same intent - it can show
        // that interpolation costs something, and it CANNOT show that either
        // arm is right (NUMERIC_CLAIMS.md §1).
        //
        // It is here because `MAX_COMPILED_GRID_BYTES`'s own doc comment says
        // the >= 5-channel recommendations "carry NO ΔE claim", and a stated
        // absence of a claim is worth more when the thing not being claimed
        // has a number beside it. REPORTED, permanently: see the note in
        // §3.8.4 of TOLERANCES.md on why this is not the number to grade.
        let rec = match &st {
            Err(e) => Record::errored(
                "passh/C/7clr/compiled-vs-reference-at-the-default-grid",
                Kind::SelfConsistency,
                Metric::DeviceAbsMaxNormalised,
                REPORTED,
                "both arms are iccce",
                format!("could not run `iccce bench`: {e}"),
            ),
            Ok(r) => {
                let off = r
                    .field("error.max_device_offnode")
                    .and_then(|s| s.trim().parse::<f64>().ok());
                let samples = r.field("error.samples").map(str::trim).unwrap_or("?");
                match off {
                    None => Record::errored(
                        "passh/C/7clr/compiled-vs-reference-at-the-default-grid",
                        Kind::SelfConsistency,
                        Metric::DeviceAbsMaxNormalised,
                        REPORTED,
                        "both arms are iccce",
                        format!(
                            "`iccce bench` exited {:?} without printing \
                             error.max_device_offnode",
                            r.code
                        ),
                    ),
                    Some(v) => graded(
                        "passh/C/7clr/compiled-vs-reference-at-the-default-grid",
                        Kind::SelfConsistency,
                        REPORTED,
                        v,
                        "there is no expectation and this row states none. Both arms are the same \
                         codebase, so no bound derived from this number could distinguish a \
                         correct engine from a consistently wrong one",
                        format!(
                            "★ THE PRICE OF THE DEFAULT GRID, REPORTED, NEVER GRADED, AND THE \
                             WEAKEST CLASS IN THIS PASS. Maximum device-space (0..1) difference \
                             between the compiled grid at {grid} points per axis and the reference \
                             chain, over {samples} off-node probes: {v:.9}. ■ SELF-COMPARISON - \
                             both arms are iccce, so this is worthless as correctness evidence \
                             (NUMERIC_CLAIMS.md §1). It is recorded because \
                             MAX_COMPILED_GRID_BYTES's own doc says the >=5-channel \
                             recommendations carry NO ΔE claim, and an absence of a claim is more \
                             useful with a number beside it than without. ★ WHY THIS IS NOT \
                             PROMOTED TO A GRADED ROW: the 33 for 3-D and 4-D is gated on Pass 4's \
                             MEASURED iccce-vs-lcms2 agreement on a real pair; no equivalent \
                             exists at 7 inputs, because lcms2's >4-input CLUT geometry has not \
                             been read out of the pinned source and ICC.1 legislates none (A16). \
                             A bound fitted to this one file would be a population of one"
                        ),
                    )
                    .with_separation(Separation::none(
                        "a self-comparison has no rival READING - there is only one reading, and \
                         it is being compared with itself at a different sampling density",
                    ))
                    .with_metric(Metric::DeviceAbsMaxNormalised),
                }
            }
        };
        out.push(rec);
    }

    // --- C5: 6CLR coverage is zero -----------------------------------------
    let six: Vec<&str> = v
        .iter()
        .filter(|f| {
            f.run
                .field("header.colorspace")
                .is_some_and(|s| s.contains("6CLR"))
        })
        .map(|f| f.name.as_str())
        .collect();
    let six_refused: Vec<&str> = v
        .iter()
        .filter(|f| !f.accepted() && f.name.contains("SixChan"))
        .map(|f| f.name.as_str())
        .collect();
    out.push(
        graded(
            "passh/C/coverage/6clr-evidence-is-zero",
            Kind::DerivedExpectation,
            REPORTED,
            six.len() as f64,
            "computed at run time from this machine's copy of the corpus",
            format!(
                "★ COVERAGE, STATED HONESTLY. Accepted profiles declaring a 6CLR colour space: \
                 {} [{}]. The corpus's only six-channel file is [{}], which is iccMAX and is \
                 refused at the version gate - so iccce has ZERO evidence of any kind about \
                 6-channel handling, and none may be inferred from the 7-channel rows above: a \
                 7CLR mAB and a 6CLR tag share no code path that this pass exercised",
                six.len(),
                six.join(", "),
                six_refused.join(", ")
            ),
        )
        .with_separation(Separation::none(
            "a coverage census has no rival candidate; it is a count of what is on the disk",
        ))
        .with_metric(Metric::IndicatorCount),
    );
}

// ===========================================================================
// Section D — the Probe profiles, against the ICC's published statement
// ===========================================================================

/// The provenance string every section D row carries.
const SRC_README: &str = "ICC, `Probe2 Profile Readme June 1, 2007`, distributed inside \
     Probev2.zip from color.org and read 2026-08-17. A PUBLISHED VENDOR STATEMENT about a named \
     file, transcribed with its source; no implementation's output is in it. It is ground truth \
     about RENDERING-INTENT TAG SELECTION and about the lightness band a tag's output lies in - \
     NOT a published colorimetric value";

fn section_d(iccce: &Iccce, oracle: &Oracle, dir: &Path, out: &mut Vec<Record>) {
    for (fname, short, kind) in PROBES {
        let path = dir.join(fname);
        let push_skip = |out: &mut Vec<Record>, why: String| {
            for (suffix, tol) in PROBE_ROWS {
                out.push(Record::skipped(
                    format!("passh/D/{short}/{suffix}"),
                    *kind,
                    Metric::IndicatorCount,
                    probe_tolerance(short, suffix, *tol),
                    SRC_README,
                    why.clone(),
                ));
            }
        };
        let Ok(bytes) = std::fs::read(&path) else {
            push_skip(out, format!("{fname} is not present in {}", dir.display()));
            continue;
        };
        let Ok(profile) = Profile::parse(&bytes) else {
            push_skip(out, format!("{fname} was refused by iccce"));
            continue;
        };
        probe_rows(iccce, oracle, &path, &profile, short, *kind, out);
    }
}

#[allow(clippy::too_many_lines)]
fn probe_rows(
    iccce: &Iccce,
    oracle: &Oracle,
    path: &Path,
    profile: &Profile,
    short: &str,
    kind: Kind,
    out: &mut Vec<Record>,
) {
    let id = |suffix: &str| format!("passh/D/{short}/{suffix}");
    let tol = |suffix: &str, default: Tolerance| probe_tolerance(short, suffix, default);

    // -----------------------------------------------------------------
    // B2A: the published colorant assignment
    // -----------------------------------------------------------------
    let sweep = lightness_sweep();
    let mut off_max = 0.0_f64;
    let mut spread_max = 0.0_f64;
    let mut mono_max = 0.0_f64;
    let mut dominance_failures = 0.0_f64;
    let mut dominance_margin = f64::INFINITY;
    let mut missing = Vec::new();
    let mut endpoint_note = Vec::new();
    let mut points = 0usize;

    for (sig, intent, colorant, sel) in B2A_COLORANT {
        let Some(ev) = TagEval::build(profile, Signature(sig)) else {
            missing.push(format!("{intent}/{colorant}"));
            continue;
        };
        let mut prev: Option<f64> = None;
        let (mut at0, mut at100) = (f64::NAN, f64::NAN);
        for &l in &sweep {
            let mut first = f64::NAN;
            let (mut lo, mut hi) = (f64::INFINITY, f64::NEG_INFINITY);
            for (a, b) in AB_PROBES {
                let Some(dev) = ev.lab_to_device(Lab { l, a, b }) else {
                    continue;
                };
                points += 1;
                for (i, x) in dev.iter().enumerate() {
                    if i != sel {
                        off_max = off_max.max(x.abs());
                    }
                }
                // Dominance: the published colorant must be strictly the
                // largest of the three CHROMATIC channels. Black is excluded
                // deliberately - the readme's sentence names colorants, and a
                // black component is not one of the three it distinguishes.
                let rivals = (0..3.min(dev.len()))
                    .filter(|i| *i != sel)
                    .map(|i| dev[i])
                    .fold(f64::NEG_INFINITY, f64::max);
                let margin = dev[sel] - rivals;
                if margin <= 0.0 {
                    dominance_failures += 1.0;
                }
                dominance_margin = dominance_margin.min(margin);
                if first.is_nan() {
                    first = dev[sel];
                }
                lo = lo.min(dev[sel]);
                hi = hi.max(dev[sel]);
            }
            if hi >= lo {
                spread_max = spread_max.max(hi - lo);
            }
            if let Some(p) = prev {
                mono_max = mono_max.max(first - p);
            }
            prev = Some(first);
            if l == 0.0 {
                at0 = first;
            }
            if l == 100.0 {
                at100 = first;
            }
        }
        endpoint_note.push(format!(
            "{intent}/{colorant}: L*0 -> {at0:.6} (readme: maximum coverage), \
             L*100 -> {at100:.6} (readme: unmarked media)"
        ));
    }

    out.push(
        graded(
            &id("b2a/off-colorant-channels-are-exactly-zero"),
            kind,
            tol("b2a/off-colorant-channels-are-exactly-zero", ZERO_COLORANT),
            off_max,
            SRC_README,
            format!(
                "{}readme: \"The B2A0 tag renders the L* values as tints of PURE CYAN. The B2A1 \
                 tag renders them as tints of PURE MAGENTA, and the B2A2 tag ... PURE YELLOW.\" \
                 Graded quantity: the largest value on any channel the readme says carries no \
                 colorant, over {points} evaluations ({} L* levels x {} (a*,b*) probes x 3 tags). \
                 Endpoints: {}. Missing tags: [{}]",
                falsified_prefix(short, "b2a/off-colorant-channels-are-exactly-zero"),
                sweep.len(),
                AB_PROBES.len(),
                endpoint_note.join(" | "),
                missing.join(", ")
            ),
        )
        .with_separation(Separation::against_distance(
            "the intent-to-tag map is rotated by one - perceptual reads B2A1, media-relative \
             reads B2A2, saturation reads B2A0. Under that reading the \"off\" channel this row \
             watches carries the ENTIRE tint, whose maximum is the profile's own maximum \
             coverage",
            1.0,
            1.0,
            SepUnits::SameAsMetric,
        ))
        .with_metric(Metric::DeviceAbsMaxNormalised),
    );

    out.push(
        graded(
            &id("b2a/a-and-b-are-ignored"),
            kind,
            tol("b2a/a-and-b-are-ignored", ONE_DEVICE_CODE),
            spread_max,
            SRC_README,
            format!(
                "{}readme: \"the rendering intent transforms ... IGNORE THE a* AND b* COMPONENTS \
                 of incoming PCS colors\". Graded quantity: the largest spread of the selected \
                 colorant across {} (a*,b*) probes spanning all four chroma quadrants, at fixed \
                 L*, maximised over {} L* levels and 3 tags",
                falsified_prefix(short, "b2a/a-and-b-are-ignored"),
                AB_PROBES.len(),
                sweep.len()
            ),
        )
        .with_separation(Separation::none(
            "there is no rival reading of \"ignore\"; the alternative is simply that the \
             statement is false of the file, which is what the measurement reports",
        ))
        .with_metric(Metric::DeviceAbsMaxNormalised),
    );

    out.push(
        graded(
            &id("b2a/tint-is-monotone-decreasing-in-L"),
            kind,
            tol("b2a/tint-is-monotone-decreasing-in-L", ONE_DEVICE_CODE),
            mono_max,
            SRC_README,
            format!(
                "{}readme: \"map the L* components directly to MONOTONE tints ... L* = 0 is \
                 rendered as maximum colorant coverage, and L* = 100 is rendered as unmarked \
                 media\". Graded quantity: the largest INCREASE of the selected colorant between \
                 consecutive L* levels, i.e. the largest violation of the stated polarity, over \
                 {} steps and 3 tags. Endpoints: {}",
                falsified_prefix(short, "b2a/tint-is-monotone-decreasing-in-L"),
                sweep.len() - 1,
                endpoint_note.join(" | ")
            ),
        )
        .with_separation(Separation::against_distance(
            "the polarity is reversed - L* 0 rendered as unmarked media and L* 100 as maximum \
             coverage. Under that reading every step is an increase and this row observes the \
             full step size of the ramp",
            1.0 / (sweep.len() as f64 - 1.0),
            1.0 / (sweep.len() as f64 - 1.0),
            SepUnits::SameAsMetric,
        ))
        .with_metric(Metric::DeviceAbsMaxNormalised),
    );

    out.push(
        graded(
            &id("b2a/the-published-colorant-dominates-at-every-point"),
            kind,
            EXACT_COUNT,
            dominance_failures,
            SRC_README,
            format!(
                "★ the WEAKENED form of the readme's colorant sentence, and the form that \
                 survives on every file in this corpus: the published colorant must be STRICTLY \
                 the largest of the three chromatic channels. Black is excluded because the \
                 readme's sentence distinguishes three colorants and black is not one of them. \
                 Smallest margin observed: {dominance_margin:.6}. This is the row that catches an \
                 intent-to-tag mis-wiring",
            ),
        )
        .with_separation(Separation::against_distance(
            "the intent-to-tag map is rotated by one. Under that reading the named colorant is \
             not the largest at any point and this row observes every evaluation",
            points as f64,
            points as f64,
            SepUnits::SameAsMetric,
        ))
        .with_metric(Metric::IndicatorCount),
    );

    // -----------------------------------------------------------------
    // A2B: the published lightness bands
    // -----------------------------------------------------------------
    let nch = TagEval::build(profile, Signature(A2B_BANDS[0].0)).map_or(4, |e| e.device_channels());
    let grid = device_grid(nch);
    let mut bands: Vec<(f64, f64, f64, f64, &str)> = Vec::new();
    for (sig, intent, lo, hi) in A2B_BANDS {
        let Some(ev) = TagEval::build(profile, Signature(sig)) else {
            continue;
        };
        let (mut mn, mut mx) = (f64::INFINITY, f64::NEG_INFINITY);
        for d in &grid {
            if let Some(l) = ev.device_to_lab(d) {
                mn = mn.min(l.l);
                mx = mx.max(l.l);
            }
        }
        bands.push((mn, mx, lo, hi, intent));
    }

    // Ordering: the perceptual band must sit entirely above the
    // media-relative one, which must sit entirely above saturation. The
    // graded quantity is the OVERLAP, which is zero when they are disjoint.
    let mut overlap = 0.0_f64;
    let mut gaps = Vec::new();
    for w in bands.windows(2) {
        let gap = w[0].0 - w[1].1;
        gaps.push(format!("{} over {}: {gap:+.6} L*", w[0].4, w[1].4));
        overlap = overlap.max((-gap).max(0.0));
    }
    let band_text: Vec<String> = bands
        .iter()
        .map(|(mn, mx, lo, hi, i)| {
            format!("{i}: realised {mn:.6}..{mx:.6} vs published {lo}..{hi}")
        })
        .collect();

    out.push(
        graded(
            &id("a2b/the-three-published-bands-are-disjoint-and-ordered"),
            kind,
            EXACT_COUNT,
            overlap,
            SRC_README,
            format!(
                "readme: A2B0 -> L* 70..100, A2B1 -> 30..70, A2B2 -> 0..30 - three DISJOINT \
                 bands in a published order. Graded quantity: the L* OVERLAP between adjacent \
                 realised bands, zero when they are disjoint. {} device points per tag. \
                 Bands: {}. Gaps: {}",
                grid.len(),
                band_text.join(" | "),
                gaps.join(", ")
            ),
        )
        .with_separation(Separation::against_distance(
            "the intent-to-tag map is rotated by one, so two intents read the same table and the \
             bands coincide. Under that reading the overlap is the full width of a band, which \
             the readme publishes as 30 L*",
            30.0,
            30.0,
            SepUnits::SameAsMetric,
        ))
        .with_metric(Metric::AbsMaxComponent),
    );

    let excursion = bands
        .iter()
        .map(|(mn, mx, lo, hi, _)| (lo - mn).max(0.0).max((mx - hi).max(0.0)))
        .fold(0.0_f64, f64::max);
    out.push(
        graded(
            &id("a2b/published-band-containment"),
            kind,
            REPORTED,
            excursion,
            SRC_README,
            format!(
                "★ REPORTED, NOT GRADED, and the reason is a FINDING: the ICC's published band is \
                 NOT met by the ICC's own file. Largest excursion outside a published endpoint: \
                 {excursion:.6} L*. Bands: {}. The excursions occur at heavy-ink corners of the \
                 device cube, which are outside any real press gamut and are extrapolated in the \
                 table; and lcms2 reproduces them (see the vs-lcms2 row), so this is a statement \
                 about the ARTEFACT, not about either engine. It is reported rather than gated \
                 because no bound derivable from the encoding admits a 2-3 L* excursion, and a \
                 bound chosen to admit the observation would be tuning",
                band_text.join(" | ")
            ),
        )
        .with_separation(Separation::none(
            "the published endpoints are the only candidate; there is no second published band \
             to weigh them against",
        ))
        .with_metric(Metric::AbsMaxComponent),
    );

    // -----------------------------------------------------------------
    // The mpet rows — clause 8.10.2's precedence question
    // -----------------------------------------------------------------
    let mpet: Vec<&iccce_profile::TagEntry> = profile
        .tags
        .iter()
        .filter(|t| t.type_sig == Some(Signature(0x6D70_6574)))
        .collect();
    let mpet_decoded = mpet
        .iter()
        .filter(|t| {
            matches!(
                profile.decode_tag(t),
                Some(Ok(d)) if !matches!(d.data, TagData::Unknown)
            )
        })
        .count() as f64;

    if mpet.is_empty() {
        for suffix in ["tags/mpet-is-present-and-NOT-decoded", "tags/mpet-selection-divergence-from-lcms2"] {
            out.push(Record::skipped(
                id(suffix),
                kind,
                Metric::IndicatorCount,
                EXACT_COUNT,
                SRC_README,
                "this profile carries no DToBx/BToDx (multiProcessElements) tags, so clause \
                 8.10.2 step a) cannot arise and both implementations necessarily read the same \
                 AToBx/BToAx tags. The precondition this row tests for does not exist here"
                    .to_string(),
            ));
        }
    } else {
        let sigs: Vec<String> = mpet.iter().map(|t| t.sig.to_string()).collect();
        out.push(
            graded(
                &id("tags/mpet-is-present-and-NOT-decoded"),
                kind,
                EXACT_COUNT,
                mpet_decoded,
                "ICC.1:2022 clause 8.10.2, VERBATIM: \"a) Use the BToD0Tag ... DToB3Tag \
                 designated for the rendering intent if the tag is present, EXCEPT WHERE THIS TAG \
                 IS NOT NEEDED OR SUPPORTED BY THE CMM ... b) Use the BToA0Tag ... AToB2Tag \
                 designated for the rendering intent if present, when the tag in a) is not used.\" \
                 (transcribed in D:\\Dev\\Rag-Specialized\\ICC_Spec\\icc\\icc__s__required_tags.md \
                 section 5)",
                format!(
                    "★★ THE PRECONDITION FOR A CONFORMANT DIVERGENCE. This profile carries {} \
                     mpet tags [{}] and iccce decodes {mpet_decoded} of them. Because the tag is \
                     not supported by this CMM, clause 8.10.2's own proviso sends iccce to step \
                     b) and the AToBx/BToAx tags - which is CONFORMANT. lcms2 supports mpet and \
                     takes step a), which is ALSO conformant. Two conformant CMMs, two different \
                     colours, from one file: the standard's own designed-in divergence (A33). \
                     ★ What is NOT settled by this row: iccce takes step b) SILENTLY. Nothing in \
                     `inspect` or `transform` tells a caller that an author-preferred transform \
                     was present and declined - and CLAUDE.md rule 6 is about exactly that kind \
                     of undisclosed substitution",
                    mpet.len(),
                    sigs.join(", ")
                ),
            )
            .with_separation(Separation::against_distance(
                "iccce decodes mpet and takes step a) like lcms2. Under that reading this row \
                 observes every mpet tag in the file, and the profile's answer changes completely \
                 - see the divergence row",
                mpet.len() as f64,
                mpet.len() as f64,
                SepUnits::SameAsMetric,
            ))
            .with_metric(Metric::IndicatorCount),
        );

        // The magnitude of the divergence, measured through the oracle.
        let rec = match (
            TagEval::build(profile, Signature(A2B_BANDS[0].0)),
            oracle_lab(oracle, path, Intent::Perceptual, &[0.0; 4]),
        ) {
            (Some(ev), Ok(their_l)) => {
                let mine = ev.device_to_lab(&[0.0; 4]).map_or(f64::NAN, |l| l.l);
                graded(
                    &id("tags/mpet-selection-divergence-from-lcms2"),
                    kind,
                    REPORTED,
                    (mine - their_l).abs(),
                    "both sides computed in this run; the clause that makes both conformant is \
                     ICC.1:2022 8.10.2 (see the sibling row)",
                    format!(
                        "★★★ THE SIZE OF A CONFORMANT DISAGREEMENT. Unmarked media (CMYK 0,0,0,0) \
                         at the perceptual intent: iccce reads AToB0 and returns L* {mine:.6}; \
                         lcms2 reads DToB0 and returns L* {their_l:.6}. The readme identifies the \
                         two by their published colour codes, so the divergence is not a mystery \
                         to be diagnosed - the profile was BUILT to make it visible, and it did. \
                         Reported, never graded: no clause requires the two to agree",
                    ),
                )
                .with_separation(Separation::none(
                    "both readings are named, both are conformant under 8.10.2, and no clause \
                     prefers one - so there is no \"alternative\" here in the sense this field \
                     means. The two candidates are the row's subject, not its rival",
                ))
                .with_metric(Metric::AbsMaxComponent)
            }
            (_, e) => Record::errored(
                id("tags/mpet-selection-divergence-from-lcms2"),
                kind,
                Metric::AbsMaxComponent,
                REPORTED,
                SRC_README,
                format!("could not measure the divergence: {:?}", e.err()),
            ),
        };
        out.push(rec);
    }

    // -----------------------------------------------------------------
    // The oracle arm on the A2B tags — only where both read the same tags
    // -----------------------------------------------------------------
    if mpet.is_empty() {
        // ★★ The encoded-PCS ceiling, computed from the tag's OWN storage
        // family, and the predicate that splits this comparison.
        //
        // iccce clamps the encoded PCS to the representable range at the B
        // curve (clause 10.18's domain, via `Trc::eval`); lcms2 does not (its
        // identity curve is an analytic gamma-1 segment, evaluated unbounded).
        // Pass 4b found this on `fixtures/synthetic/v4-cmyk-mab-lab.icc` and
        // recorded it REPORTED, not graded, because **which behaviour the
        // specification requires is unsettled**
        // (`pass4b/fixture/mab/encoded-pcs-overflow-divergence`).
        //
        // ★ The predicate is evaluated on **lcms2's** output, not on iccce's:
        // "the file encodes a PCS value above what this tag's encoding can
        // represent" is a fact about the file, and the side that does *not*
        // clamp is the one that can still show it. Splitting on iccce's own
        // clamp fixed-point would be splitting on the behaviour under test.
        let ceiling = match TagEval::build(profile, Signature(A2B_BANDS[0].0)) {
            // v4 PCSLAB (clause 10.13): L* = 100 x code/65535, max 100.0.
            Some(TagEval::Ab(_)) => 100.0_f64,
            // legacy 16-bit Lab (6.3.4.2 NOTE 3): L* = code/652.8, and code
            // 0xFFFF decodes to 100 x 65535/65280 = 100.390625.
            _ => 100.0 * 65535.0 / 65280.0,
        };

        let mut worst = 0.0_f64;
        let mut detail = Vec::new();
        let mut failed = None;
        let mut clamp_worst = 0.0_f64;
        let mut clamp_points = 0usize;
        let mut clamp_example = String::new();
        for (sig, intent, _, _) in A2B_BANDS {
            let Some(ev) = TagEval::build(profile, Signature(sig)) else {
                continue;
            };
            let it = match intent {
                "perceptual" => Intent::Perceptual,
                "media-relative" => Intent::RelativeColorimetric,
                _ => Intent::Saturation,
            };
            let pts = device_corners(nch);
            let req = Request {
                input: Space::profile(path.to_path_buf()),
                output: Space::lab_v2(),
                intent: it,
                precalc: Precalc::Exact,
                bpc: Bpc::Off,
                values: pts.iter().flatten().map(|x| x * 100.0).collect(),
            };
            match oracle.convert_batch_shaped(&req, nch, 3) {
                Err(e) => failed = Some(e.to_string()),
                Ok(rows) => {
                    let mut m = 0.0_f64;
                    for (p, r) in pts.iter().zip(&rows) {
                        if let Some(l) = ev.device_to_lab(p) {
                            let d = (l.l - r[0]).abs();
                            if r[0] > ceiling {
                                clamp_points += 1;
                                if d > clamp_worst {
                                    clamp_worst = d;
                                    clamp_example = format!(
                                        "{intent} at device {p:?}: iccce {:.6}, lcms2 {:.6}",
                                        l.l, r[0]
                                    );
                                }
                            } else {
                                m = m.max(d);
                            }
                        }
                    }
                    detail.push(format!("{intent}: {m:.6e} L*"));
                    worst = worst.max(m);
                }
            }
        }

        out.push(
            graded(
                &id("a2b/encoded-pcs-clamp-divergence"),
                Kind::CrossCheck,
                REPORTED,
                clamp_worst,
                "both sides computed in this run. NOT GRADED because which behaviour the \
                 specification requires is UNSETTLED - the same question Pass 4b left open at \
                 `pass4b/fixture/mab/encoded-pcs-overflow-divergence`, owed to icc-spec-librarian",
                format!(
                    "★★★ PASS 4b's SYNTHETIC FINDING, REPRODUCED ON A REAL ICC-PUBLISHED FILE. \
                     iccce clamps the encoded PCS at the B curve (clause 10.18's domain, via \
                     Trc::eval); lcms2 does not (its identity curve is an analytic gamma-1 \
                     segment, evaluated unbounded). Pass 4b measured this on a fixture THIS \
                     PROJECT AUTHORED, so it could have been an artefact of our own fixture \
                     design - it is not. {clamp_points} of the graded corners encode a PCS L* \
                     above this tag's representable ceiling ({ceiling:.6}); largest divergence \
                     {clamp_worst:.6} L*. Worst: [{clamp_example}]. These points are EXCLUDED \
                     from the sibling cross-check row by a predicate evaluated on LCMS2's output \
                     (the side that does not clamp), never on iccce's",
                ),
            )
            .with_separation(Separation::none(
                "both behaviours are named and neither is settled by any clause this project has \
                 sourced; the two candidates are the row's subject, not its rival",
            ))
            .with_metric(Metric::AbsMaxComponent),
        );

        let rec = match failed {
            Some(e) => Record::errored(
                id("a2b/vs-lcms2-through-the-same-tags"),
                Kind::CrossCheck,
                Metric::AbsMaxComponent,
                ORACLE_LAB,
                "both sides computed in this run",
                format!("the oracle arm did not run: {e}"),
            ),
            None => graded(
                &id("a2b/vs-lcms2-through-the-same-tags"),
                Kind::CrossCheck,
                ORACLE_LAB,
                worst,
                "both sides computed in this run: iccce-cmm's LutAbModel/Lut16Model IN PROCESS \
                 (the shipped binary has no surface that emits PCS L*a*b*), and transicc into \
                 *Lab2. ★ *Lab2 and not *Lab4: lcms2 forces BPC on when the DESTINATION is v4 and \
                 the intent is perceptual or saturation (Pass 4b finding 2), and into *Lab4 this \
                 same comparison reads 0.16 L* worse at perceptual for that reason alone",
                format!(
                    "★ THE CROSS-CHECK that keeps the ground-truth rows honest: a published band \
                     is a coarse target, and two implementations agreeing to a fraction of a \
                     16-bit code on the same tag is a much finer instrument. {} device corners per \
                     intent, all three A2B tags, less {clamp_points} points excluded as \
                     encoded-PCS overflows (see the sibling clamp row - the exclusion predicate \
                     is `lcms2's L* exceeds this tag's representable ceiling {ceiling:.6}`, a \
                     fact about the FILE evaluated on the side that does not clamp). \
                     Per intent: {}",
                    device_corners(nch).len(),
                    detail.join(", ")
                ),
            )
            .with_separation(Separation::none(
                "no rival READING is in play - two implementations of the same clause on the same \
                 bytes. The one rival that WAS in play, lcms2's forced BPC, is defeated by the \
                 fixture choice (*Lab2) rather than modelled, so it is in the tolerance's why",
            ))
            .with_metric(Metric::AbsMaxComponent),
        };
        out.push(rec);
    } else {
        for (suffix, tol) in [
            ("a2b/vs-lcms2-through-the-same-tags", ORACLE_LAB),
            ("a2b/encoded-pcs-clamp-divergence", REPORTED),
        ] {
            out.push(Record::skipped(
                id(suffix),
                Kind::CrossCheck,
                Metric::AbsMaxComponent,
                tol,
                "both sides would be computed in this run",
                "★ lcms2 does not read this profile's AToBx tags AT ALL - it takes the DToBx \
                 (mpet) tags per clause 8.10.2 a), which iccce declines under the same clause's \
                 \"not supported by the CMM\" proviso. There is therefore no second reading of \
                 the SAME tag to compare against, and a comparison of the two DIFFERENT tags \
                 would be a number with no meaning. The size of that divergence is reported by \
                 the mpet-selection row"
                    .to_string(),
            ));
        }
    }

    // -----------------------------------------------------------------
    // The shipped-binary arm — intent to tag, through the product
    // -----------------------------------------------------------------
    let src = synthetic_rgb_source();
    let rows = shipped_source_rows();
    let rec = if !src.is_file() {
        Record::skipped(
            id("shipped/intent-selects-the-published-colorant"),
            kind,
            Metric::IndicatorCount,
            SHIPPED_PRINT_FLOOR,
            SRC_README,
            format!("the committed RGB source fixture is missing at {}", src.display()),
        )
    } else {
        let mut failures = 0.0_f64;
        let mut margin = f64::INFINITY;
        let mut shipped_off = 0.0_f64;
        let mut err = None;
        for (sig, intent, colorant, sel) in B2A_COLORANT {
            let _ = sig;
            let it = match intent {
                "perceptual" => Intent::Perceptual,
                "media-relative" => Intent::RelativeColorimetric,
                _ => Intent::Saturation,
            };
            match iccce.transform_rows_shaped(&src, path, it, &rows, 4) {
                Err(e) => err = Some(format!("{intent}/{colorant}: {e}")),
                Ok(got) => {
                    for r in &got {
                        let rivals = (0..3)
                            .filter(|i| *i != sel)
                            .map(|i| r[i])
                            .fold(f64::NEG_INFINITY, f64::max);
                        if r[sel] - rivals <= 0.0 {
                            failures += 1.0;
                        }
                        margin = margin.min(r[sel] - rivals);
                        for (i, x) in r.iter().enumerate() {
                            if i != sel {
                                shipped_off = shipped_off.max(x.abs());
                            }
                        }
                    }
                }
            }
        }
        match err {
            Some(e) => Record::errored(
                id("shipped/intent-selects-the-published-colorant"),
                kind,
                Metric::IndicatorCount,
                SHIPPED_PRINT_FLOOR,
                SRC_README,
                format!("the shipped binary did not convert into this profile: {e}"),
            ),
            None => graded(
                &id("shipped/intent-selects-the-published-colorant"),
                kind,
                EXACT_COUNT,
                failures,
                SRC_README,
                format!(
                    "★★ THE MOST VALUABLE ROW IN THIS PASS: the SHIPPED PRODUCT's intent-to-tag \
                     wiring, graded against a published statement. `iccce transform` from a \
                     COMMITTED synthetic RGB fixture into this probe at each of the three named \
                     intents; the readme's colorant for that intent must be strictly the largest \
                     of C, M, Y in the printed output. {} source points x 3 intents. Smallest \
                     margin {margin:.6}; largest value on a channel the readme says carries no \
                     colorant, {shipped_off:.6} (the shipped print floor is 1e-6). An in-process \
                     library test cannot see a CLI-to-Chain mis-wiring; this can",
                    rows.len()
                ),
            )
            .with_separation(Separation::against_distance(
                "the CLI's intent names are wired to the tags in a rotated order - --intent \
                 perceptual reaching B2A1, and so on. Under that reading the named colorant is \
                 never the largest and this row observes every point",
                rows.len() as f64 * 3.0,
                rows.len() as f64 * 3.0,
                SepUnits::SameAsMetric,
            ))
            .with_metric(Metric::IndicatorCount),
        }
    };
    out.push(rec);
}

/// One `L*` out of the oracle for an n-channel device value, into `*Lab2`.
///
/// `*Lab2` and not `*Lab4` for the reason [`ORACLE_LAB`] records: lcms2 forces
/// black point compensation on for a v4 **destination** at the perceptual and
/// saturation intents, and a v2 Lab destination keeps that gate shut.
fn oracle_lab(
    oracle: &Oracle,
    profile: &Path,
    intent: Intent,
    device: &[f64],
) -> Result<f64, DiffError> {
    let req = Request {
        input: Space::profile(profile.to_path_buf()),
        output: Space::lab_v2(),
        intent,
        precalc: Precalc::Exact,
        bpc: Bpc::Off,
        values: device.iter().map(|x| x * 100.0).collect(),
    };
    let rows = oracle.convert_batch_shaped(&req, device.len(), 3)?;
    rows.first()
        .map(|r| r[0])
        .ok_or_else(|| DiffError::Internal("oracle returned no rows".into()))
}
