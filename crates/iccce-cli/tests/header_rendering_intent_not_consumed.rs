//! # The header's `renderingIntent` field is parsed, validated, and not
//! consumed — a DISCLOSURE, not a certification
//!
//! ## What is measured here
//!
//! Every ICC profile header carries a `renderingIntent` field: a
//! `uInt32Number` at header offset 64, clause 7.2.15, whose defined values
//! are 0 perceptual, 1 media-relative colorimetric, 2 saturation,
//! 3 ICC-absolute colorimetric (Table 23). A caller of a CMM also names an
//! intent. **When the caller does not name one, which of the two governs?**
//!
//! This file measures iccce's answer on a fixture pair that differs in
//! **exactly one byte** — file offset 67, the low byte of that field, `00h`
//! in one member and `01h` in the other — and finds:
//!
//! * iccce **parses** the field (`iccce-profile`'s header reader stores it,
//!   and `inspect` prints it),
//! * iccce **validates** it (a value outside `0..=3` is reported as a
//!   malformation — the fixture `rendering-intent-high-bits` covers that
//!   arm), and
//! * the CMM **never consumes it**. The intent comes from the caller and
//!   from nowhere else. Structurally: [`Chain::new`] takes `intent: Intent`
//!   as a required parameter, so there is no code path in which the header
//!   value could be reached, and `iccce transform`'s no-flag default is a
//!   constant in the CLI (`Intent::MediaRelative`), not a value read from
//!   the source profile.
//!
//! ## ★ THE OPEN QUESTION — what this file does NOT claim
//!
//! **This test records what iccce does. It does not claim that what iccce
//! does is correct.**
//!
//! Whether ICC.1 *requires*, *permits* or *forbids* a CMM to consume a
//! profile's header `renderingIntent` field when no intent is otherwise
//! specified is **UNSOURCED as of 2026-08-18**. No clause has been read on
//! the question. Nothing here was written from memory (project rule 2: never
//! write colour behaviour from memory), and no position is taken. An
//! **`icc-spec-librarian` dispatch to settle it from the text — clause
//! 7.2.15, which defines the field, and clause 8.10.2, which defines how a
//! CMM selects a transform — is OUTSTANDING.**
//!
//! Three answers are possible, and this file is deliberately compatible with
//! all three:
//!
//! | if the specification turns out to say | then |
//! |---|---|
//! | the field is informative; the caller's intent governs | the measured behaviour stands, and this file becomes its regression test, unchanged |
//! | the field **shall** be honoured when the caller specifies nothing | **iccce is wrong.** [`the_two_members_produce_bit_identical_output`] is the assertion that flips, and the fixture needed to prove the fix is already committed |
//! | the field **shall** be ignored by a CMM | the measured behaviour stands, and [`the_control_two_named_intents_do_change_the_output`] is what shows it is deliberate rather than accidental |
//!
//! **Why this is written as a disclosure.** The alternative was available
//! and is worse. A file named `header_intent_is_correctly_ignored` would
//! have contained the same code, gone green in the same way, and recorded a
//! claim nobody had sourced — and a recorded claim is precisely why nobody
//! re-checks. This project has been bitten by that mechanism already: a
//! module doc asserted an 8.10.2 disclosure that had never been implemented,
//! and *the assertion is why nobody looked*
//! (`crates/iccce-cmm/tests/mpet_fallback_disclosure.rs`).
//!
//! ## Evidence class
//!
//! **Self-comparison — the weakest of the three classes this project uses.**
//! Every number asserted below is iccce compared against iccce. There is no
//! published value here, so this is *not ground truth*, and nothing that runs
//! in `cargo test` here consults a second implementation. What
//! self-comparison can establish is a **behavioural** fact — that a byte
//! does or does not reach the output — and that is the entire claim. It
//! cannot establish that the behaviour is right, and nothing below should be
//! quoted as if it could.
//!
//! ## Cross-check against lcms2 — run out of band, 2026-08-18
//!
//! A cross-check against another implementation, which per project rule 7 is
//! **weaker than ground truth and cannot close the open question**: lcms2 is
//! an implementation, not the standard. It is recorded because it tells the
//! outstanding librarian dispatch where to look.
//!
//! Same two fixtures, same destination, against the pinned oracle in
//! `tools/difftest/vendor` (transicc 5.1 / LittleCMS 2.19):
//!
//! ```text
//! echo "128 128 128" | transicc -i <member> -o v4-rgb-matrix-trc.icc [-t0..-t3]
//! ```
//!
//! * **lcms2 does not consume the field either.** The two members produced
//!   identical output at every setting tried — no flag, `-t0`, `-t1`, `-t2`,
//!   `-t3`.
//! * **Its no-flag default is a program constant, and that constant is intent
//!   0.** `utils/transicc/transicc.c` declares
//!   `static cmsUInt32Number Intent = INTENT_PERCEPTUAL`. So for the member
//!   whose header says **1**, lcms2's default chose **0**: the header value
//!   was present, disagreed with the default, and lost. The two engines
//!   ignore the same field while defaulting to *different* intents (iccce
//!   media-relative, transicc perceptual), which is a sharper statement than
//!   either engine alone could make.
//! * **★ The one place lcms2 does read the field is DeviceLink profiles** —
//!   `src/cmsio1.c`, in `cmsIsCLUT`, verbatim: *"For devicelinks, the
//!   supported intent is that one stated in the header"*. It is the only read
//!   of `cmsGetHeaderRenderingIntent` anywhere in lcms2's transform path;
//!   every other mention writes the field. **Neither fixture here is a
//!   DeviceLink**, and that is the lead the librarian dispatch should start
//!   from: the answer may be device-class-dependent, and this file has
//!   measured only the non-DeviceLink case.
//! * Incidentally, the fixture separates under lcms2 too (`-t0` and `-t1`
//!   differ there), so its discriminating power is not an artefact of iccce's
//!   own tag selection.
//!
//! **Not asserted in code**, deliberately: `transicc` is out-of-tree and not
//! present in CI, so a test depending on it would be a test that silently
//! skips. The four bullets above are a dated observation, reproducible from
//! the pinned vendor tree by the command shown.
//!
//! ## Why the fixture is synthetic
//!
//! The behaviour was first observed on two profiles embedded in a licensed
//! third-party corpus. `docs/GHENT_COMPATIBILITY.md` §2.3 is absolute: no
//! value may be copied out of that directory into this repository — not into
//! a test, not into a doc, not into a comment. **Nothing here is derived
//! from those files.** The pair below is authored byte by byte by
//! `tools/gen-profiles` (recipes `v2-rgb-header-intent-perceptual` and
//! `v2-rgb-header-intent-relative`), is regenerable with
//! `gen-profiles verify`, and states its own provenance in its `cprt` tag.
//! The observation is reproduced from scratch, on bytes this project owns.
//!
//! ## Why there are five tests and not one
//!
//! The headline measurement — two profiles, one differing byte, identical
//! output — is **vacuous on its own**, in two distinct ways, and each way is
//! closed by one of the other tests:
//!
//! 1. *The pair might differ somewhere else too.* Then the test would be
//!    measuring an unknown combination of changes.
//!    [`the_pair_differs_in_exactly_one_byte_at_offset_67`] closes it,
//!    re-deriving the property **from the files on disk** rather than
//!    trusting the generator that wrote them.
//! 2. *The two intents might have nothing to choose between.* If the
//!    profile's `A2B0` and `A2B1` produced the same colour, then output
//!    identical under both header values would certify nothing at all — the
//!    mechanism could be fully live and invisible.
//!    [`the_control_two_named_intents_do_change_the_output`] closes it, and
//!    it is the only test here with real discriminating power.
//!
//! A third: *the byte might not be read at all*, in which case "not
//! consumed" would be the wrong description of the finding.
//! [`the_parser_reads_the_field_it_does_not_consume`] separates *ignored*
//! from *unparsed*.
//!
//! ## Proved capable of failing — by injection, 2026-08-18
//!
//! "Differential" is not the same as "load-bearing", and the only way to tell
//! them apart is to introduce the defect and watch. Both injections were made
//! locally, run, and reverted; neither is committed.
//!
//! **Injection 1 — the rival hypothesis.** `iccce transform` was patched to
//! honour the source header's `renderingIntent` when the caller named no
//! intent. Result: [`the_two_members_produce_bit_identical_output`] and
//! [`the_no_flag_default_is_media_relative`] both FAILED; the other three
//! passed. So the two assertions that carry the finding do detect exactly the
//! behaviour change the open question is about.
//!
//! ★ Note **which half** caught it. The library half of that test cannot: at
//! the library surface there is no "unspecified" state to change. The failure
//! came from the CLI arm, which is why this file lives in `iccce-cli`'s tests
//! and shells out to the binary rather than sitting in `iccce-cmm` where the
//! rest of the transform tests live.
//!
//! **Injection 2 — the vacuity.** `intent_split_mft2` was patched to emit the
//! same table for both intents (zero separation) and the two fixtures were
//! regenerated. Result: **[`the_two_members_produce_bit_identical_output`]
//! still PASSED** — vacuously, on a fixture that could not have shown a
//! difference — while [`the_control_two_named_intents_do_change_the_output`]
//! failed with *"the two intents are only 0.000000 apart"*. That is the
//! control doing precisely the job it exists for, and a demonstration that
//! without it a green result here would mean nothing.

use iccce_cmm::matrix_trc::Intent;
use iccce_cmm::transform::Chain;
use iccce_profile::Profile;
use std::path::PathBuf;
use std::process::{Command, Stdio};

/// The member whose header says 0 (perceptual).
const PERCEPTUAL: &str = "v2-rgb-header-intent-perceptual.icc";
/// The member whose header says 1 (media-relative colorimetric). Identical
/// to [`PERCEPTUAL`] except at file offset 67.
const RELATIVE: &str = "v2-rgb-header-intent-relative.icc";

/// The destination, held constant across every arm of every test below.
///
/// It is a committed synthetic fixture rather than iccce's built-in sRGB for
/// one reason: the destination is **not the subject of this experiment**, and
/// a destination that is a file in this repository can be regenerated and
/// diffed by the same `gen-profiles verify` that guards the source pair. Its
/// primaries are an arbitrary split of the encoded D50 white point and
/// describe no device — see the `recipes.rs` module header. Nothing here is a
/// colorimetric claim.
const DESTINATION: &str = "v4-rgb-matrix-trc.icc";

/// The offset of the byte that differs, and the whole reason the pair exists.
///
/// `renderingIntent` is a `uInt32Number` at header offset **64** (clause
/// 7.2.15). The values 0 and 1 differ only in the **low** byte of that
/// big-endian word, which is offset **67**.
const INTENT_LOW_BYTE: usize = 67;

/// `iccce transform`'s documented no-flag default, as a value.
///
/// ★ There is no "unspecified" state in the library API: [`Chain::new`]'s
/// third parameter is an `Intent`, not an `Option<Intent>`. **That is itself
/// the mechanism this file is documenting** — a required parameter cannot
/// fall back to a value read from the profile. "Intent unspecified" therefore
/// means, and can only mean, *whatever the caller's default is*, and for the
/// shipped CLI that default is this constant. The link between the two is not
/// assumed: [`the_no_flag_default_is_media_relative`] asserts it against the
/// binary.
const CLI_DEFAULT_INTENT: Intent = Intent::MediaRelative;

/// A device value in the interior of the source cube.
///
/// Chosen away from every CLUT node so the measured numbers are a trilinear
/// blend of all eight nodes of whichever table was selected — a value on a
/// node would read back one table entry and would be a weaker probe of the
/// selection.
const PROBE: [f64; 3] = [0.5, 0.5, 0.5];

/// The floor the control must clear, in device units on the destination's
/// 0..1 scale.
///
/// **This is a bound on the FIXTURE's power, not a tolerance on a colour
/// value.** It answers "is this pair capable of showing a difference at all?"
/// and nothing else.
///
/// Justification for the number: the fixture's two tables are 12 units apart
/// in both `a` and `b` at **every** CLUT node by construction (see
/// `intent_split_mft2` in `tools/gen-profiles/src/recipes.rs`), a separation
/// of roughly 17 in chroma — far outside any perceptibility threshold and
/// orders of magnitude outside f64 rounding. Measured separation at [`PROBE`]
/// is about 0,20 device units. The floor is set an order of magnitude below
/// **the measured value**, not just below the design intent, so it fails
/// loudly if the separation ever collapses toward zero — which is the failure
/// mode that would silently make the headline test vacuous — while not
/// pinning a specific number that a legitimate change to the destination
/// model would have to chase.
const SEPARATION_FLOOR: f64 = 0.02;

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/synthetic")
        .join(name)
}

fn read(name: &str) -> Vec<u8> {
    std::fs::read(fixture_path(name))
        .unwrap_or_else(|e| panic!("fixture {name} must be present and readable: {e}"))
}

fn parse(name: &str) -> Profile {
    Profile::parse(&read(name)).unwrap_or_else(|e| panic!("fixture {name} must parse: {e:?}"))
}

/// Run one conversion through the library at a named intent.
fn convert(src: &str, intent: Intent) -> Vec<f64> {
    let src = parse(src);
    let dst = parse(DESTINATION);
    Chain::new(&src, &dst, intent)
        .expect("the chain must build")
        .convert(&PROBE)
        .expect("the conversion must run")
}

/// Run the shipped binary, returning its stdout verbatim.
///
/// The CLI is exercised as well as the library because the behaviour was
/// observed at the CLI, and because "no `--intent` flag" is a state that only
/// exists at that surface.
fn run_cli(src: &str, extra: &[&str]) -> String {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_iccce"));
    cmd.arg("transform")
        .arg("--src")
        .arg(fixture_path(src))
        .arg("--dst")
        .arg(fixture_path(DESTINATION))
        .args(extra)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = cmd.spawn().expect("the iccce binary must be runnable");
    {
        use std::io::Write;
        let stdin = child.stdin.as_mut().expect("stdin");
        writeln!(stdin, "{} {} {}", PROBE[0], PROBE[1], PROBE[2]).expect("write probe");
    }
    let out = child.wait_with_output().expect("the binary must exit");
    assert!(
        out.status.success(),
        "iccce transform {extra:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).expect("stdout is UTF-8")
}

/// **Premise 1 — the apparatus isolates its variable.**
///
/// Re-derived from the bytes on disk, deliberately: the generator asserts the
/// same property on the bytes it produces, but a test that trusted the
/// generator would not notice a fixture that had been edited, truncated, or
/// regenerated from a changed recipe. `gen-profiles verify` is the other half
/// of that guard; this is the half that runs in `cargo test`.
///
/// A pair that differed in two places would make every other test in this
/// file prove something strictly weaker than it claims, without any of them
/// changing colour.
#[test]
fn the_pair_differs_in_exactly_one_byte_at_offset_67() {
    let a = read(PERCEPTUAL);
    let b = read(RELATIVE);
    assert_eq!(a.len(), b.len(), "the pair must be the same length");
    let diffs: Vec<usize> = (0..a.len()).filter(|&i| a[i] != b[i]).collect();
    assert_eq!(
        diffs,
        vec![INTENT_LOW_BYTE],
        "the pair must differ in exactly one byte, the low byte of renderingIntent at offset \
         {INTENT_LOW_BYTE}; got {diffs:?}"
    );
    assert_eq!(
        (a[INTENT_LOW_BYTE], b[INTENT_LOW_BYTE]),
        (0, 1),
        "the differing byte must be 0 (perceptual) vs 1 (media-relative)"
    );
    // The other three bytes of the uInt32Number are zero in both, so offset
    // 67 carries the whole value rather than part of it.
    assert_eq!(&a[64..INTENT_LOW_BYTE], &[0, 0, 0]);
    assert_eq!(&b[64..INTENT_LOW_BYTE], &[0, 0, 0]);
}

/// **Premise 2 — the field is READ. "Ignored" is not "unparsed".**
///
/// The distinction matters to anyone acting on this record. A parser that
/// never looked at offset 64 would produce identical output for the same
/// reason a parser that looked and declined would, and the two call for
/// entirely different work if the open question resolves against us.
///
/// Both members must also report **zero malformations**: intents 0 and 1 are
/// both defined values, so neither member is a defect fixture, and a
/// malformation appearing here would mean the pair had stopped being a clean
/// single-variable experiment.
#[test]
fn the_parser_reads_the_field_it_does_not_consume() {
    let p = parse(PERCEPTUAL);
    let r = parse(RELATIVE);
    assert_eq!(
        p.header.rendering_intent, 0,
        "the parser must read 0 from {PERCEPTUAL}"
    );
    assert_eq!(
        r.header.rendering_intent, 1,
        "the parser must read 1 from {RELATIVE}"
    );
    assert!(
        p.malformations.is_empty() && r.malformations.is_empty(),
        "both members are well-formed; got {:?} / {:?}",
        p.malformations,
        r.malformations
    );
}

/// **THE MEASUREMENT.** With the intent unspecified, the two members produce
/// bit-identical output.
///
/// "Unspecified" is [`CLI_DEFAULT_INTENT`] at the library surface and a
/// missing `--intent` flag at the CLI surface; both are asserted, because the
/// second is the one that was actually observed and the first is the one that
/// explains it.
///
/// The comparison is **bit-identical, not within a tolerance**. There is no
/// tolerance to justify here and none is wanted: the claim is that one byte
/// of input reached nothing, and any non-zero difference at all would falsify
/// it. A tolerance would only create room for the claim to become partly
/// false without failing.
///
/// ★ Read this result with the module header's open question attached. It
/// says the header value does not reach the output. It does not say that is
/// right.
#[test]
fn the_two_members_produce_bit_identical_output() {
    // Library surface.
    let p = convert(PERCEPTUAL, CLI_DEFAULT_INTENT);
    let r = convert(RELATIVE, CLI_DEFAULT_INTENT);
    assert_eq!(
        p, r,
        "the two members differ only in the header rendering-intent byte, and at an \
         unspecified intent they produced different output: {p:?} vs {r:?}"
    );

    // CLI surface — no --intent flag at all, which is the state that was
    // originally observed and cannot be expressed in the library API.
    let cp = run_cli(PERCEPTUAL, &[]);
    let cr = run_cli(RELATIVE, &[]);
    assert_eq!(
        cp, cr,
        "`iccce transform` with no --intent produced different output for the two members"
    );
    println!("unspecified intent, both members: {}", cp.trim_end());
}

/// ★★★ **THE CONTROL, and the only test here with real power.**
///
/// If this profile's `A2B0` and `A2B1` produced the same colour, the test
/// above would pass no matter what the CMM did with the header byte, and
/// would certify nothing. This asserts that the two intents named by the two
/// header values — 0 selects `A2B0`, 1 selects `A2B1`, per clause 8.10.2 b) —
/// **do** lead to materially different output when the caller asks for them
/// explicitly.
///
/// That makes the rival hypothesis concrete rather than abstract: *if* iccce
/// consumed the header field, the member whose header says 0 would have
/// produced the perceptual answer measured here, and the test above would
/// have failed by roughly the separation measured here. It did not.
///
/// Two ways this control could itself be fooled, both closed below:
///
/// * **Clipping at the destination.** Two different PCS values landing on the
///   same clipped device value would erase the separation. Asserted: no
///   output channel sits at either rail.
/// * **A separation too small to mean anything.** Asserted against
///   [`SEPARATION_FLOOR`], with the reasoning at that constant.
#[test]
fn the_control_two_named_intents_do_change_the_output() {
    let relative = convert(PERCEPTUAL, Intent::MediaRelative);
    let perceptual = convert(PERCEPTUAL, Intent::Perceptual);

    for (label, v) in [("media-relative", &relative), ("perceptual", &perceptual)] {
        assert!(
            v.iter().all(|c| *c > 0.001 && *c < 0.999),
            "the {label} result touches a device rail ({v:?}); a clipped output could hide the \
             separation this control exists to demonstrate"
        );
    }

    let separation = relative
        .iter()
        .zip(&perceptual)
        .map(|(a, b)| (a - b).abs())
        .fold(0.0_f64, f64::max);
    assert!(
        separation > SEPARATION_FLOOR,
        "the two intents are only {separation:.6} apart in device units, below the \
         {SEPARATION_FLOOR} floor — this fixture is no longer separating the two A2B tags, and \
         `the_two_members_produce_bit_identical_output` has become vacuous"
    );
    println!(
        "control: media-relative {relative:?} vs perceptual {perceptual:?}; \
         max separation {separation:.6} device units"
    );

    // The same on the CLI surface, since that is where the original
    // observation was made.
    assert_ne!(
        run_cli(PERCEPTUAL, &["--intent", "perceptual"]),
        run_cli(PERCEPTUAL, &["--intent", "media-relative"]),
        "the CLI produced the same output for two explicitly different intents"
    );
}

/// The CLI's no-flag default **is** media-relative colorimetric.
///
/// This is the bridge between the two surfaces, and it is asserted rather
/// than assumed because the module header's account of the mechanism — a
/// constant in the CLI, not a value read from the profile — depends on it. If
/// the default ever changed, the headline measurement would still pass while
/// the explanation attached to it had quietly become false, which is the
/// stale-claim failure this project keeps meeting.
#[test]
fn the_no_flag_default_is_media_relative() {
    assert_eq!(
        run_cli(PERCEPTUAL, &[]),
        run_cli(PERCEPTUAL, &["--intent", "media-relative"]),
        "`iccce transform` with no --intent must equal an explicit --intent media-relative"
    );
    // ...and it is NOT perceptual, which is the value this member's header
    // carries. Stated separately because it is the exact substitution the
    // open question is about.
    assert_ne!(
        run_cli(PERCEPTUAL, &[]),
        run_cli(PERCEPTUAL, &["--intent", "perceptual"]),
        "the no-flag default matched the intent named in the source profile's header — if that \
         is ever true, the open question in this file's header has been answered by accident"
    );
}
