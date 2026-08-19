//! # `inspect`'s `header.intent:` line is a DISCLOSURE surface, and this
//! file is the only thing stopping a tidy-up from deleting it silently
//!
//! ## The mechanism this file exists to catch
//!
//! `iccce inspect` prints the header's `renderingIntent` field (clause
//! 7.2.15, header offset 64, a `uInt32Number`) **verbatim, all 32 bits**,
//! followed by a parenthesised name or `(UNKNOWN)`:
//!
//! ```text
//! header.intent: 65537 (UNKNOWN)
//! ```
//!
//! `65537` is `0x00010001` — a value whose **low half is `1`, a perfectly
//! ordinary media-relative colorimetric**, and whose **high half is `1`,
//! which in a v2 profile is vendor space** (see
//! `header_rendering_intent_not_consumed.rs` and the `IntentRule` split in
//! `iccce-profile`). That is why the v2 member of the pair reports
//! **`malformations: 0`**: ICC.1:2001-04 6.1.11 reserves the low 16 bits and
//! says nothing that forbids the high half, so there is no violation to
//! report. The **only** trace that the profile carries anything unusual at
//! all is the number on that line.
//!
//! **The failure mode, concretely.** Someone tidies the print site to
//! `h.rendering_intent & 0xFFFF`, reasoning — not unreasonably — that the
//! low half is the intent and the high half is noise. The line then reads
//! `header.intent: 1 (media-relative)`. The profile still reports zero
//! malformations, because zero malformations was always correct. **The
//! output is now indistinguishable from a plain, unremarkable v2 profile,
//! and the whole suite stays green**, because before this file no test
//! asserted anything about that line's *text*.
//!
//! That is the shape of bug this project is most exposed to: not a wrong
//! number, but a **deleted disclosure** — the parser reporting less than it
//! knows, with nothing failing. Project rule 6: the parser reports; it does
//! not repair. Masking the high half would be a repair performed on the
//! *output* rather than the file, which is the same defect wearing a
//! cosmetic disguise.
//!
//! ## Why an assertion on printed text, not on structure
//!
//! A test that asserted `Profile::parse(..).header.rendering_intent ==
//! 65537` would pass unchanged after the masking edit, because the masking
//! would live in the CLI, not the parser. The claim being defended is about
//! **what a user is told**, so the assertion is made on the bytes the binary
//! writes to stdout. This is the project's standing preference for
//! assertions on measured output over assertions on code shape.
//!
//! ## What is asserted, and the four cells
//!
//! Two independent conditions govern the *malformation* report — profile
//! edition (v2 / v4) and which half of the field is non-default — and all
//! four cells are exercised, because the `IntentRule` split landed with the
//! v2/v4 wording fix and the CLI is where a consumer meets it:
//!
//! | fixture | version | field | `malformations:` | why |
//! |---|---|---|---|---|
//! | `v2-rendering-intent-high-bits` | 2.4.0 | `0x00010001` | **0** | v2 does not forbid the high half |
//! | `rendering-intent-high-bits` | 4.4.0 | `0x00010001` | 1 | v4 7.2.15 + Table 23 define the whole field |
//! | `v2-rendering-intent-low-half` | 2.4.0 | `0x00000004` | 1 | *unrecognised*, not *forbidden* — the wording differs |
//! | `v4-rendering-intent-low-half` | 4.4.0 | `0x00000004` | 1 | outside `0..=3` |
//!
//! The **wording** of the two low-half reports is asserted as well as the
//! count, because the count is identical in both and the edition-correct
//! distinction lives entirely in the words. A regression that collapsed
//! `IntentRule` back to one message would keep every count right.
//!
//! ## Evidence class — two different strengths, kept apart
//!
//! * **The number `65537` is not taken from iccce.** It is re-derived in
//!   [`the_printed_number_is_the_field_read_big_endian_from_disk`] by
//!   reading bytes 64..68 of the fixture and composing them big-endian per
//!   the `uInt32Number` encoding — arithmetic on the file, independent of
//!   the code under test. Project rule 3 in its strict form: an expectation
//!   produced by the function under test detects change, not error.
//! * **The surrounding text is self-comparison.** `(UNKNOWN)`, the
//!   `header.intent: ` prefix and the malformation sentences are iccce's own
//!   wording, asserted so that a change to them is a deliberate act rather
//!   than an accident. No claim of external correctness is made about them,
//!   and none should be read in.
//!
//! No colour value is converted here and no tolerance applies: every
//! assertion is on exact integers and exact strings.
//!
//! ## What this file does NOT claim
//!
//! It does not claim printing the raw 32-bit value is what ICC.1 requires —
//! ICC.1 says nothing about a diagnostic tool's output format. It claims
//! only that iccce *chose* to disclose the whole field, and that the choice
//! is now load-bearing enough to be defended by a test rather than by
//! whoever next reads the print site.

use std::path::PathBuf;
use std::process::Command;

/// Header offset of `renderingIntent` (ICC.1:2022 Table 18 / 7.2.15) and its
/// length. Named rather than inlined because the derivation in
/// [`the_printed_number_is_the_field_read_big_endian_from_disk`] is only
/// meaningful if the reader can see it is the spec's offset.
const INTENT_OFFSET: usize = 64;
const INTENT_LEN: usize = 4;

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/synthetic")
        .join(format!("{name}.icc"))
}

/// Run `iccce inspect <fixture>` and return stdout verbatim.
///
/// Verbatim, not parsed into a structure: the thing under test is the text,
/// so anything that normalised it would weaken the assertion.
fn inspect(fixture: &str) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_iccce"))
        .arg("inspect")
        .arg(fixture_path(fixture))
        .output()
        .expect("the iccce binary must be runnable");
    assert!(
        out.status.success(),
        "iccce inspect {fixture} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).expect("stdout is UTF-8")
}

/// The single line beginning `header.intent: `, without its newline.
///
/// Panics if absent, because an absent line is the most likely way this
/// disclosure disappears and it must fail loudly rather than vacuously pass.
fn intent_line(fixture: &str) -> String {
    let stdout = inspect(fixture);
    stdout
        .lines()
        .find(|l| l.starts_with("header.intent: "))
        .unwrap_or_else(|| {
            panic!("`inspect {fixture}` printed no `header.intent:` line at all:\n{stdout}")
        })
        .to_string()
}

/// The `malformations: N` count as an integer.
fn malformation_count(fixture: &str) -> usize {
    let stdout = inspect(fixture);
    stdout
        .lines()
        .find_map(|l| l.strip_prefix("malformations: "))
        .unwrap_or_else(|| {
            panic!("`inspect {fixture}` printed no `malformations:` line:\n{stdout}")
        })
        .trim()
        .parse()
        .expect("the malformation count is an integer")
}

/// Every `malformation: ...` line, in the order printed.
fn malformation_lines(fixture: &str) -> Vec<String> {
    inspect(fixture)
        .lines()
        .filter_map(|l| l.strip_prefix("malformation: ").map(str::to_string))
        .collect()
}

/// **The ground-truth arm.** The printed number equals the field composed
/// big-endian from the bytes on disk — derived from the file, not from
/// iccce.
///
/// If this and [`the_v2_high_bits_profile_discloses_the_whole_field`]
/// disagree, the masking edit has happened and this is the assertion that
/// says so in terms nobody can argue with: the file contains `0x00010001`,
/// so the tool must print `65537`.
#[test]
fn the_printed_number_is_the_field_read_big_endian_from_disk() {
    for fixture in [
        "v2-rendering-intent-high-bits",
        "rendering-intent-high-bits",
        "v2-rendering-intent-low-half",
        "v4-rendering-intent-low-half",
        "v2-rgb-header-intent-perceptual",
        "v2-rgb-header-intent-relative",
    ] {
        let bytes = std::fs::read(fixture_path(fixture))
            .unwrap_or_else(|e| panic!("fixture {fixture} must be readable: {e}"));
        let field = &bytes[INTENT_OFFSET..INTENT_OFFSET + INTENT_LEN];
        // `uInt32Number` is big-endian (ICC.1:2022 4.5). Composed by hand
        // rather than with `from_be_bytes` on a parsed struct so that the
        // expectation cannot inherit a parser bug.
        let expected = (field[0] as u64) << 24
            | (field[1] as u64) << 16
            | (field[2] as u64) << 8
            | (field[3] as u64);
        assert_eq!(
            intent_line(fixture),
            format!(
                "header.intent: {expected} ({})",
                match expected {
                    0 => "perceptual",
                    1 => "media-relative",
                    2 => "saturation",
                    3 => "absolute",
                    _ => "UNKNOWN",
                }
            ),
            "the printed intent for {fixture} must be the field as it lies on \
             disk, unmasked; bytes 64..68 are {field:02x?}"
        );
    }
}

/// **The disclosure.** A v2 profile using the high half prints the whole
/// value and reports no malformation — the number is the *only* signal.
///
/// Asserted as one literal string rather than as a substring so that a
/// change to either half of the line fails here.
#[test]
fn the_v2_high_bits_profile_discloses_the_whole_field() {
    assert_eq!(
        intent_line("v2-rendering-intent-high-bits"),
        "header.intent: 65537 (UNKNOWN)",
        "0x00010001 masked to its low half would read `1 (media-relative)`, \
         which is exactly the disclosure this test exists to preserve"
    );
    assert_eq!(
        malformation_count("v2-rendering-intent-high-bits"),
        0,
        "v2 does not forbid the high half (ICC.1:2001-04 6.1.11), so there is \
         nothing to report and the printed number carries the whole finding"
    );
}

/// The v4 member of the same byte pattern: same printed number, but the
/// value **is** a violation there, so the count rises.
///
/// The pair is the point. Identical field, identical printed line, different
/// verdict — the difference is the profile edition and nothing else.
#[test]
fn the_v4_high_bits_profile_prints_the_same_number_but_reports_it() {
    assert_eq!(
        intent_line("rendering-intent-high-bits"),
        "header.intent: 65537 (UNKNOWN)"
    );
    assert_eq!(
        malformation_count("rendering-intent-high-bits"),
        1,
        "v4's 7.2.15 + Table 23 define the whole field, so 0x00010001 is out \
         of range"
    );
    assert_eq!(
        malformation_lines("rendering-intent-high-bits"),
        vec![
            "rendering intent 0x00010001 is outside the defined 0..=3 \
             (ICC.1:2022 7.2.15 + Table 23)"
        ]
    );
}

/// The low-half cells: both report, and the **words differ by edition**.
///
/// This is the assertion that a collapse of `IntentRule` back to a single
/// message would fail. Counting alone would not catch it: both cells count
/// 1.
#[test]
fn the_low_half_reports_differ_in_wording_by_edition() {
    assert_eq!(
        intent_line("v2-rendering-intent-low-half"),
        "header.intent: 4 (UNKNOWN)"
    );
    assert_eq!(
        intent_line("v4-rendering-intent-low-half"),
        "header.intent: 4 (UNKNOWN)"
    );

    let v2 = malformation_lines("v2-rendering-intent-low-half");
    let v4 = malformation_lines("v4-rendering-intent-low-half");
    assert_eq!(v2.len(), 1, "one report expected, got {v2:?}");
    assert_eq!(v4.len(), 1, "one report expected, got {v4:?}");

    assert_eq!(
        v2[0],
        "unrecognised rendering intent value 0x00000004 (ICC.1:2001-04 6.1.11 \
         / Table 18 define only 0..=3 and do not forbid others)",
        "v2's report must say *unrecognised*: 2001-04 defines four values but \
         does not forbid a fifth, so calling it `outside the defined range` \
         would accuse the file of a rule it does not break"
    );
    assert_eq!(
        v4[0],
        "rendering intent 0x00000004 is outside the defined 0..=3 \
         (ICC.1:2022 7.2.15 + Table 23)"
    );
    assert_ne!(
        v2[0], v4[0],
        "the whole point of IntentRule is that these two sentences are not \
         the same sentence"
    );
}

/// **The control.** On well-formed profiles the line names the intent and
/// nothing is reported — so the `(UNKNOWN)` above is a property of those
/// files, not of the printer.
///
/// Without this, every assertion above would still pass if the CLI had been
/// broken into printing `(UNKNOWN)` unconditionally.
#[test]
fn the_control_well_formed_profiles_name_their_intent() {
    assert_eq!(
        intent_line("v2-rgb-header-intent-perceptual"),
        "header.intent: 0 (perceptual)"
    );
    assert_eq!(
        intent_line("v2-rgb-header-intent-relative"),
        "header.intent: 1 (media-relative)"
    );
    assert_eq!(malformation_count("v2-rgb-header-intent-perceptual"), 0);
    assert_eq!(malformation_count("v2-rgb-header-intent-relative"), 0);
}
