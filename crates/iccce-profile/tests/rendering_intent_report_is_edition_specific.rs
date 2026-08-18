//! # The rendering-intent report is edition-specific — all four cells of the
//! gate, each separately falsifiable
//!
//! ## The one sentence
//!
//! `Header::parse` decides what to say about header bytes 64..68
//! (`renderingIntent`) by **two conditions, not one**, and this file measures
//! all four combinations of {v2, v4} × {high half set, low half undefined}
//! against the clause text that licenses each — one test per cell, named after
//! the cell, so a red says **which half broke**.
//!
//! ## What this file replaced, and why the name changed
//!
//! It supersedes `rendering_intent_high_bits_not_version_gated.rs`, written on
//! 2026-08-18 to measure a *suspected defect*: the check was
//! `rendering_intent > 3` with no version test, so a v2 profile was reported in
//! the same words as a v4 one. That file asserted the pre-fix behaviour on
//! purpose, so that the day a gate landed the suite would go red and whoever
//! landed it would have to come here and write down the new expectation. **It
//! did exactly that**, and this file is that writing-down.
//!
//! The old name described the **absence** of a gate. The gate exists, so the
//! name became a false present-tense statement about the code the moment it
//! landed — the same failure mode as a stale numeral in prose, and worse,
//! because a file name is what a reader sees first and never re-reads.
//!
//! ★ One thing the transition taught, recorded because it will recur: the v4
//! arm of the old file also went red, and **not because v4 behaviour changed**.
//! The emitted string gained its citation (`… (ICC.1:2022 7.2.15 + Table 23)`),
//! and an assertion written as an exact string equality could not tell "the
//! report is gone" from "the report gained four words". This file therefore
//! separates the two: **presence and rule** are asserted on the typed value,
//! **wording** is asserted separately and stated as a claim about emitted text.
//!
//! ## The four cells, and what licenses each
//!
//! Established by `icc-spec-librarian` from the primary text of both editions,
//! read and cross-verified through three independent extraction channels,
//! 2026-08-18; ambiguity register rows **`A7`** (high half) and **`A56`** (low
//! half).
//!
//! | # | edition | half | iccce | licence, and its strength |
//! |---|---|---|---|---|
//! | 1 | v4 | high | reports | **QUOTED.** ICC.1:2022 7.2.15: *"the most significant 16 bits shall be set to zero"* |
//! | 2 | v4 | low | reports | **INFERRED, two steps.** *"shall specify the rendering intent"* + *"These shall be identified using the values shown in Table 23"* (`A56`) |
//! | 3 | v2 | high | **silent** | ICC.1:2001-04 6.1.11 imposes nothing; its *"least-significant 16 bits are reserved for the ICC"* is the identical boilerplate 6.1.8 uses for the flags field, where the high half is vendor space |
//! | 4 | v2 | low | reports, in **weaker words** | **NOTHING IN ICC.1.** v2 defines four values and forbids none; the report is a disclosure, not an accusation |
//!
//! ★★ **The four are not equally strong claims, and a red in each calls for a
//! different response.** Cell 1 failing means iccce contradicts a quotation —
//! fix the code. Cell 2 failing means iccce contradicts an inference — check
//! that the inference is still the project's reading before touching anything.
//! Cell 3 failing means iccce has made a **false statement about a conformant
//! file**, which is the worst of the four and is argued below. Cell 4 failing
//! means a **project policy** changed, and no clause is violated either way;
//! what must not happen is that it changes silently.
//!
//! ## Why cell 3 is the one to care about — the direction of the error
//!
//! Project rule 6: *the parser reports; it does not repair.* That has a
//! consequence easy to state and easy to forget — **there is no layer above the
//! parser that can catch a report the parser should not have made.** A missed
//! malformation can still be caught downstream by something that chokes on the
//! bytes. A *false* malformation is terminal: it is emitted, it is believed,
//! and it looks exactly like diligence. Cell 3 is the only arm here that guards
//! that direction, and it is an assertion of **silence**, which no amount of
//! unrelated test-writing produces by accident.
//!
//! ## Evidence class
//!
//! **Mixed, and the two halves must not be conflated.**
//!
//! * The **clause text** in the table above is `primary_spec` — quoted verbatim
//!   from both held editions by `icc-spec-librarian`, three extraction
//!   channels. That is what makes cells 1–3 claims about the standard rather
//!   than claims about iccce.
//! * Every **measured value** below is **self-comparison** — iccce compared
//!   against iccce, on bytes this project authored. No published value and no
//!   second implementation is consulted. lcms2 is *not* an available oracle
//!   here: it does not report on this field at all when a profile is opened, so
//!   there is no cross-check to obtain, and a green suite is evidence that
//!   iccce says what this project decided it should say — no more.
//!
//! ## The apparatus, and why it isolates its variable
//!
//! Each of the four fixtures is **one well-formed base plus one mutation**: the
//! four bytes at header offset 64. The v2 pair is built on
//! `v2-rgb-matrix-trc-curv`, the v4 pair on `v4-rgb-matrix-trc`, and the two
//! pairs are **not** byte-identical across editions — deliberately. A
//! byte-identical cross-edition pair would have to carry `mluc` and
//! `parametricCurveType` (v4-only) inside a v2 file, i.e. two era violations in
//! one fixture, against the corpus's one-mutation rule
//! (`tools/gen-profiles/src/recipes.rs`, module doc).
//!
//! Isolation is instead carried two ways, both stronger here than byte
//! identity:
//!
//! * **Structurally.** `Malformation::UnknownRenderingIntent` is raised inside
//!   `Header::parse` from the 128 header bytes alone. Tag content is not in
//!   scope at that point, so no difference in tag types *can* reach the
//!   observable.
//! * **By controls one generator call away.** Each base differs from its mutant
//!   in exactly the four bytes at offset 64
//!   ([`control_each_fixture_differs_from_its_base_in_exactly_those_four_bytes`])
//!   and each base is reported clean
//!   ([`control_both_unmutated_bases_are_reported_clean`]). Without those, "the
//!   file reports" would be equally compatible with the base reporting too, and
//!   the measurement would say nothing about the intent field.
//!
//! ## Coverage, stated honestly
//!
//! Four synthetic fixtures, two editions, one field, one value per half
//! (`0x0001` high / `4` low). **No real-world profile is measured here** and no
//! vendor is known to ship either shape. [`the_census_exactly_four_fixtures_can_observe_this_gate`]
//! states that population as a runnable check rather than a sentence, so a
//! fifth fixture cannot be added without either updating this file or turning
//! it red.

use iccce_profile::{IntentRule, Malformation, Profile};
use std::path::PathBuf;

// ---------------------------------------------------------------------------
// The four cells, and their two controls
// ---------------------------------------------------------------------------

/// Cell 1 — v4, high half set. The oldest of the four (Pass 2) and the only one
/// whose report rests on a quotation. Its name carries no `v4-` prefix for
/// historical reasons; renaming a fixture is a corpus-wide edit and was not
/// worth it.
const V4_HIGH: &str = "rendering-intent-high-bits.icc";

/// Cell 2 — v4, low half undefined. Added 2026-08-18: until then the *inferred*
/// half of iccce's v4 claim had no input that could see it move.
const V4_LOW: &str = "v4-rendering-intent-low-half.icc";

/// Cell 3 — v2, high half set. Added 2026-08-18 because nothing in the corpus
/// could ask the version question. **This is the silence.**
const V2_HIGH: &str = "v2-rendering-intent-high-bits.icc";

/// Cell 4 — v2, low half undefined. Added 2026-08-18 with the gate, because a
/// gate of two conditions whose second condition has no fixture is a gate that
/// can be deleted without a single test noticing.
const V2_LOW: &str = "v2-rendering-intent-low-half.icc";

/// The v4 pair's unmutated base — control.
const V4_BASE: &str = "v4-rgb-matrix-trc.icc";

/// The v2 pair's unmutated base — control.
const V2_BASE: &str = "v2-rgb-matrix-trc-curv.icc";

/// The high-half probe value. The low half is 1 (media-relative colorimetric),
/// a value **both** editions define, so the high half is the only thing either
/// edition could be objecting to.
const HIGH_HALF_VALUE: u32 = 0x0001_0001;

/// The low-half probe value. The high half is zero, so the v4 quotation is not
/// engaged and only the *inferred* prohibition can be doing the work. `4` is
/// the **boundary** — the smallest value neither Table 18 nor Table 23 defines
/// — so an off-by-one at the top of the defined range stays visible. A distant
/// value such as `0xFFFF` would exercise the same branch and hide it.
const LOW_HALF_VALUE: u32 = 0x0000_0004;

/// The field's file offset, and its width in bytes.
const INTENT_OFFSET: usize = 64;
const INTENT_WIDTH: usize = 4;

// ---------------------------------------------------------------------------
// What a failure means — re-aimed 2026-08-18 when the gate landed
// ---------------------------------------------------------------------------

/// Appended to every failure message. **Nobody reads a module doc out of a CI
/// log**, so what to do about a red has to be in the panic text.
///
/// ★ This const previously said the opposite: it told a reader that a red here
/// was *the expected outcome of the fix*, because the file it lived in asserted
/// believed-possibly-wrong pre-fix behaviour. The gate has landed and is
/// sourced, so that text now describes history and would mislead the next
/// person to see this file red into treating a genuine regression as progress.
/// A stale disclosure left in place is worse than no disclosure, because it is
/// *read*.
///
/// ★ The superseded text is quoted here rather than left to `git log`,
/// **because it is not in `git log`** — the predecessor file was written and
/// deleted inside the same uncommitted working tree and was never committed, so
/// deleting it destroyed the only copy. Checked before writing this sentence;
/// the first draft asserted it was recoverable from history, which was false.
/// What it said, verbatim:
///
/// > IF THIS FAILED because a version gate landed in iccce-profile's header
/// > reader, that is the EXPECTED outcome of the fix and NOT a regression. This
/// > test records the pre-fix measurement on purpose. Update it, and record the
/// > clause that licensed the gate.
///
/// It did its job — the gate landed, this file is the record it demanded, and
/// the clause that licensed the gate is in the table above.
const IF_THIS_FAILED: &str = "\
IF THIS FAILED, iccce's report about header bytes 64..68 no longer matches the \
clause text that licenses it. The first question is whether the CODE is wrong, \
never whether the expectation can move (CLAUDE.md rule 5). The four arms of \
this file are NOT equally licensed and the right response differs by arm: cell \
1 (v4 high) contradicts a QUOTATION; cell 2 (v4 low) contradicts a labelled \
two-step INFERENCE (register A56) which is the project's reading and could \
legitimately be revisited; cell 3 (v2 high) means iccce is making a FALSE \
report against a CONFORMANT file, which no layer above the parser can catch; \
cell 4 (v2 low) means a project POLICY changed, licensed by no clause either \
way, which is allowed but must not happen silently. Read the clause before \
touching a number.";

// ---------------------------------------------------------------------------
// Apparatus
// ---------------------------------------------------------------------------

fn fixture_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/synthetic")
}

fn read(name: &str) -> Vec<u8> {
    let p = fixture_dir().join(name);
    std::fs::read(&p).unwrap_or_else(|e| panic!("fixture {} must be readable: {e}", p.display()))
}

fn parse(name: &str) -> Profile {
    Profile::parse(&read(name)).unwrap_or_else(|e| panic!("fixture {name} must parse: {e:?}"))
}

/// Every malformation rendered through `Display` — the exact strings a user
/// sees from `iccce inspect`, not the `Debug` form, because the emitted text
/// **is** the disclosure.
fn reports(name: &str) -> Vec<String> {
    parse(name)
        .malformations
        .iter()
        .map(ToString::to_string)
        .collect()
}

/// The one malformation a fixture must carry, or a panic naming what it carried
/// instead. Used by the three arms that expect a report; the silence arm
/// deliberately does not go through it.
fn sole_malformation(name: &str) -> Malformation {
    let m = parse(name).malformations;
    assert_eq!(
        m.len(),
        1,
        "{name} is one base plus one mutation and must produce exactly one finding — a \
         second finding means the fixture probes two things at once and neither \
         measurement is attributable; got {m:?}. {IF_THIS_FAILED}"
    );
    m.into_iter().next().expect("length asserted above")
}

/// The apparatus check: the four fixtures really are the four cells. If this
/// fails, every other assertion in the file is measuring something else and the
/// arm names are lies.
#[test]
fn the_apparatus_four_fixtures_are_the_four_cells() {
    for (name, major, value) in [
        (V4_HIGH, 4u8, HIGH_HALF_VALUE),
        (V4_LOW, 4, LOW_HALF_VALUE),
        (V2_HIGH, 2, HIGH_HALF_VALUE),
        (V2_LOW, 2, LOW_HALF_VALUE),
    ] {
        let p = parse(name);
        assert_eq!(
            p.header.version.major(),
            major,
            "{name} must be a v{major} profile, or it is in the wrong row of the matrix"
        );
        assert_eq!(
            p.header.rendering_intent, value,
            "{name} must carry the probe value for its cell"
        );
    }

    // Asserted against values read out of the fixtures' own bytes rather than
    // against the literals above — a literal-vs-literal assertion is a
    // tautology, and this way a future edit to a fixture cannot quietly change
    // what the cell probes.
    let high = parse(V2_HIGH).header.rendering_intent;
    assert!(
        high & 0xFFFF <= 3 && high >> 16 != 0,
        "the high-half probe must set the high half and leave a DEFINED low half, or it \
         probes two things at once; got 0x{high:08X}"
    );
    let low = parse(V2_LOW).header.rendering_intent;
    assert!(
        low >> 16 == 0 && low & 0xFFFF > 3,
        "the low-half probe must zero the high half and set an UNDEFINED low half, or \
         the v4 quotation is engaged and the inference is not what is being measured; \
         got 0x{low:08X}"
    );
}

// ---------------------------------------------------------------------------
// Cell 1 — v4, high half. The QUOTED prohibition.
// ---------------------------------------------------------------------------

/// ICC.1:2022 7.2.15, verbatim: *"The most significant 16 bits shall be set to
/// zero (0000h)."* This is the strongest claim iccce makes about this field and
/// the only one that needs no inference.
#[test]
fn cell1_v4_high_half_the_quoted_prohibition_is_reported() {
    assert_eq!(
        sole_malformation(V4_HIGH),
        Malformation::UnknownRenderingIntent {
            value: HIGH_HALF_VALUE,
            rule: IntentRule::V4Prohibited,
        },
        "ICC.1:2022 7.2.15 says the high 16 bits SHALL be zero, in as many words. \
         {IF_THIS_FAILED}"
    );
}

// ---------------------------------------------------------------------------
// Cell 2 — v4, low half. The INFERRED prohibition.
// ---------------------------------------------------------------------------

/// No sentence in ICC.1:2022 forbids a low-half value outside 0–3. The
/// prohibition is reached by chaining *"The rendering intent field shall
/// specify the rendering intent"* to *"These shall be identified using the
/// values shown in Table 23"* — a value naming none of the four specifies no
/// intent. Register `A56` records that as a two-step inference, and the
/// `expect` row in `MANIFEST.md` repeats the qualifier, because a manifest row
/// is what gets quoted.
///
/// ★ Note what a red here would and would not mean. It would **not**
/// automatically mean iccce is wrong: `A56` also records that ICC.1:2010-12
/// (v4.3) is **not held** by this corpus, and v4.x is exactly where the
/// high-half `shall` first appeared, so it is the plausible place for a
/// low-half wording change this project cannot currently exclude.
#[test]
fn cell2_v4_low_half_the_inferred_prohibition_is_reported() {
    assert_eq!(
        sole_malformation(V4_LOW),
        Malformation::UnknownRenderingIntent {
            value: LOW_HALF_VALUE,
            rule: IntentRule::V4Prohibited,
        },
        "the v4 report on an undefined LOW half rests on a labelled two-step inference \
         (register A56), not on a quotation. {IF_THIS_FAILED}"
    );
}

// ---------------------------------------------------------------------------
// Cell 3 — v2, high half. ★ THE SILENCE.
// ---------------------------------------------------------------------------

/// ★★ **The most important arm in this file, and the only one that asserts
/// nothing is said.**
///
/// ICC.1:2001-04 6.1.11's complete clause body is *"Perceptual, media-relative
/// colorimetric, saturation and ICC-absolute colorimetric are the four intents
/// required to be supported. The least-significant 16 bits are reserved for the
/// ICC."* plus Table 18's four rows. It imposes **nothing** on the high half —
/// and that second sentence is the *identical boilerplate* 6.1.8 uses for the
/// profile flags field, where the high half is demonstrably vendor space and
/// neither edition ever zeroes it. So these bytes are not merely un-forbidden
/// in v2; they are **invited**.
///
/// A report here would be a false statement about a conformant file, made by
/// the only layer that could have disclosed anything, with nothing above it to
/// catch the error. For part of 2026-08-18 — between this fixture being added
/// and the gate landing — iccce made exactly that report. This arm is what
/// stops it coming back.
///
/// It is asserted on the **whole malformation list**, not on the absence of one
/// variant, because a v2 file that acquired *any* finding from these four bytes
/// would be a regression of the same kind.
#[test]
fn cell3_v2_high_half_is_not_reported_at_all() {
    let m = reports(V2_HIGH);
    assert!(
        m.is_empty(),
        "★ {V2_HIGH} is a CONFORMANT v2 profile — ICC.1:2001-04 6.1.11 imposes no \
         requirement whatever on the high 16 bits of renderingIntent, and 6.1.8's \
         identical wording for the flags field shows the high half is vendor space. \
         iccce reported {m:?} against a file that breaks no rule. {IF_THIS_FAILED}"
    );
}

// ---------------------------------------------------------------------------
// Cell 4 — v2, low half. A PROJECT CHOICE, licensed by no clause.
// ---------------------------------------------------------------------------

/// The only report in this corpus that no clause requires — and, equally, that
/// no clause forbids. ICC.1:2001-04 defines four values and forbids none
/// (register `A56`; both editions use *"other values are reserved for future
/// use"* elsewhere and neither uses it here), and neither v2 clause 3 nor v4
/// clause 5 makes a profile carrying an undefined field value non-conforming.
///
/// iccce's choice is to disclose that it cannot interpret the value. This arm
/// asserts that choice, and asserts it **as a choice**: a future decision to go
/// silent here would be defensible, and would have to come through this test
/// rather than past it.
#[test]
fn cell4_v2_low_half_is_reported_as_unrecognised_not_as_forbidden() {
    assert_eq!(
        sole_malformation(V2_LOW),
        Malformation::UnknownRenderingIntent {
            value: LOW_HALF_VALUE,
            rule: IntentRule::V2Undefined,
        },
        "iccce's POLICY is to disclose an uninterpretable v2 intent value. No clause \
         requires this report and none forbids it. {IF_THIS_FAILED}"
    );
}

// ---------------------------------------------------------------------------
// The wording arm — separately falsifiable from presence
// ---------------------------------------------------------------------------

/// ★ **The emitted text differs by edition, and that is the substance of the
/// fix rather than a cosmetic detail.** *"outside the defined 0..=3"* is a true
/// statement about ICC.1:2022 and a **false statement about ICC.1:2001-04**,
/// which defines four values and forbids no others. A consumer reads the
/// string, not the enum.
///
/// This is a separate test from cells 2 and 4 on purpose. Those assert the
/// typed value — that a report exists, and under which rule. This asserts what
/// it *says*. The predecessor file conflated the two in a single string
/// equality, and when the v4 text gained its citation the failure was
/// indistinguishable from the report disappearing.
///
/// The two fixtures compared here carry the **identical four bytes**
/// (`0x00000004`), so the only variable is the edition.
#[test]
fn the_wording_identical_bytes_produce_different_text_in_the_two_editions() {
    let v4 = reports(V4_LOW);
    let v2 = reports(V2_LOW);
    assert_eq!(v4.len(), 1, "cell 2 must produce one report; got {v4:?}");
    assert_eq!(v2.len(), 1, "cell 4 must produce one report; got {v2:?}");

    assert!(
        v4[0].contains("outside the defined 0..=3") && v4[0].contains("ICC.1:2022 7.2.15"),
        "the v4 text must state a range violation and cite the edition that has one; got \
         {:?}. {IF_THIS_FAILED}",
        v4[0]
    );
    assert!(
        !v2[0].contains("outside the defined"),
        "★ THE POINT OF THE WHOLE FIX: 'outside the defined range' asserts a rule \
         ICC.1:2001-04 does not contain. A report that overstates the standard is as \
         wrong as the file it accuses. Got {:?}. {IF_THIS_FAILED}",
        v2[0]
    );
    assert!(
        v2[0].contains("unrecognised") && v2[0].contains("do not forbid others"),
        "the v2 text must say the value is unrecognised AND that the edition does not \
         forbid it — the second half is what keeps the report from being read as an \
         accusation; got {:?}. {IF_THIS_FAILED}",
        v2[0]
    );
    assert_ne!(
        v2[0], v4[0],
        "identical bytes, different editions, identical words would mean the wording \
         selection has been collapsed back to one branch. {IF_THIS_FAILED}"
    );
}

// ---------------------------------------------------------------------------
// Controls
// ---------------------------------------------------------------------------

/// Control 1 — each fixture differs from its own base in exactly the four bytes
/// of the `renderingIntent` field, and in nothing else.
///
/// Without this, "the fixture reports a malformation" is compatible with the
/// mutation being somewhere else entirely.
#[test]
fn control_each_fixture_differs_from_its_base_in_exactly_those_four_bytes() {
    for (base_name, mutant_name) in [
        (V4_BASE, V4_HIGH),
        (V4_BASE, V4_LOW),
        (V2_BASE, V2_HIGH),
        (V2_BASE, V2_LOW),
    ] {
        let base = read(base_name);
        let mutant = read(mutant_name);
        assert_eq!(
            base.len(),
            mutant.len(),
            "{mutant_name} must not change the length of {base_name}"
        );

        let differing: Vec<usize> = base
            .iter()
            .zip(&mutant)
            .enumerate()
            .filter(|(_, (x, y))| x != y)
            .map(|(i, _)| i)
            .collect();

        assert!(
            !differing.is_empty(),
            "{mutant_name} and {base_name} must actually differ, or this control is vacuous"
        );
        assert!(
            differing
                .iter()
                .all(|&i| (INTENT_OFFSET..INTENT_OFFSET + INTENT_WIDTH).contains(&i)),
            "every byte differing between {base_name} and {mutant_name} must lie inside \
             the renderingIntent field at {INTENT_OFFSET}..{}; differing offsets were \
             {differing:?}",
            INTENT_OFFSET + INTENT_WIDTH
        );
    }
}

/// Control 2 — both unmutated bases report nothing.
///
/// This is what makes each measurement attributable to the intent field. It is
/// also the assertion that catches the *opposite* failure from cell 3: an
/// edition gate implemented so broadly that it suppressed reports it should not
/// have.
#[test]
fn control_both_unmutated_bases_are_reported_clean() {
    for name in [V4_BASE, V2_BASE] {
        let m = reports(name);
        assert!(
            m.is_empty(),
            "{name} must be reported clean, or the measurements on its mutated twins \
             attribute to the wrong byte; got {m:?}"
        );
    }
}

// ---------------------------------------------------------------------------
// Coverage, as a runnable check
// ---------------------------------------------------------------------------

/// The scope statement, as a check rather than a sentence in a doc.
///
/// **Exactly four fixtures in the corpus carry an intent field outside 0..=3**,
/// and they are the four cells above. Every other fixture carries a zero high
/// half and a low half of 0 or 1, so the gate is invisible to all of them. That
/// is not a complaint about the corpus: it is the reason the gate could not
/// have been tested before these fixtures existed, and the reason a future
/// change to the gate must not be believed on the strength of a green suite
/// alone.
///
/// Asserted as an exact set, not a floor. A fifth fixture with an out-of-range
/// intent would be a fixture whose category this file's reasoning governs, and
/// adding one without coming here is the thing to prevent.
#[test]
fn the_census_exactly_four_fixtures_can_observe_this_gate() {
    let mut visible: Vec<(u8, String)> = Vec::new();

    let dir = fixture_dir();
    for entry in std::fs::read_dir(&dir).expect("the synthetic corpus must be present") {
        let path = entry.expect("readable directory entry").path();
        if path.extension().and_then(|e| e.to_str()) != Some("icc") {
            continue;
        }
        let bytes = std::fs::read(&path).expect("readable fixture");
        // Deliberate refusals (iccMAX, hostile tag count, …) never reach a
        // header and are not part of this population.
        let Ok(p) = Profile::parse(&bytes) else {
            continue;
        };
        if p.header.rendering_intent > 3 {
            visible.push((
                p.header.version.major(),
                path.file_name().unwrap().to_string_lossy().into_owned(),
            ));
        }
    }
    visible.sort();

    let mut expected = vec![
        (2u8, V2_HIGH.to_string()),
        (2, V2_LOW.to_string()),
        (4, V4_HIGH.to_string()),
        (4, V4_LOW.to_string()),
    ];
    expected.sort();

    assert_eq!(
        visible, expected,
        "exactly four fixtures carry an intent outside 0..=3 — two per edition, one per \
         half — so exactly four can observe the edition gate. Any other member of this \
         list is a fixture whose conformance category the sourcing in this file governs, \
         and it must be filed deliberately rather than inherited."
    );
}

/// ★ The claim-string guard: the words this test asserts are the words the
/// **manifest** publishes.
///
/// `MANIFEST.md` is generated from `tools/gen-profiles/src/recipes.rs`, whose
/// `expect` fields are **typed string literals quoting the emitted report**. A
/// typed quotation of another module's output is a claim with no mechanism
/// keeping it true, and this project has watched several go false within a day
/// of being written — including this very row: when the `Display` impl gained
/// its `(ICC.1:2022 7.2.15 + Table 23)` citation, the manifest's copy became a
/// quotation of text that no longer existed.
///
/// Interpolating is not available across the boundary — `tools/gen-profiles` is
/// deliberately dependency-free and **must never link `iccce-profile`**, or a
/// fixture could inherit a bug from the code it tests
/// (`tools/gen-profiles/Cargo.toml`). Reading the generated file as **text**
/// costs nothing and keeps the arrow pointing the safe way: the test crate
/// reads the manifest, the generator still reads nothing.
///
/// Scoped to the four rendering-intent fixtures on purpose. A corpus-wide
/// version is a much larger claim that would go red for unrelated reasons.
#[test]
fn the_manifest_quotes_the_report_text_that_is_actually_emitted() {
    let manifest = std::fs::read_to_string(fixture_dir().join("MANIFEST.md"))
        .expect("MANIFEST.md must be present — regenerate with `gen-profiles manifest`");

    for name in [V4_HIGH, V4_LOW, V2_LOW] {
        for report in reports(name) {
            assert!(
                manifest.contains(&report),
                "MANIFEST.md must quote {name}'s report verbatim, and does not. The \
                 emitted text is:\n  {report}\nRegenerate the manifest AND fix the \
                 `expect` literal in tools/gen-profiles/src/recipes.rs — a manifest row \
                 is read as authoritative, so a stale quotation there is a published \
                 false claim, not a cosmetic drift."
            );
        }
    }

    // Cell 3 publishes a silence, which has no string to quote. Assert the row
    // says so, so that the silence is documented rather than merely absent.
    assert!(
        manifest.contains("ZERO malformations"),
        "MANIFEST.md must state {V2_HIGH}'s expectation as an explicit zero — a fixture \
         whose whole purpose is a silence needs its row to SAY silence, or a reader \
         cannot tell an asserted silence from an unwritten one."
    );
}
