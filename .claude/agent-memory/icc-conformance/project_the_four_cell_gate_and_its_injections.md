---
name: project-the-four-cell-gate-and-its-injections
description: 2026-08-18 — a gate of TWO conditions has FOUR cells and the corpus held one; the four are not equally licensed (quoted / inferred / silence / project choice). Also: an exact-string assertion cannot tell "the report vanished" from "the report gained a citation", and MANIFEST.md quotes emitted text with nothing keeping the quotation true.
metadata:
  type: project
---

Replaced `rendering_intent_high_bits_not_version_gated.rs` with
`crates/iccce-profile/tests/rendering_intent_report_is_edition_specific.rs`
when `icc-engineer` landed the edition gate. Five things below are durable.

## 1. ★★ A gate of two conditions has FOUR cells, and the corpus held one

`Header::parse` now branches on edition **and** on which half of
`renderingIntent` is out of range. That is a 2x2, and only `(v4, high)` had a
fixture. Added `v4-rendering-intent-low-half` and `v2-rendering-intent-low-half`
(both intent `0x00000004`, the **boundary** value — a distant `0xFFFF` exercises
the same branch while hiding an off-by-one at the top of the defined set).

**How to apply:** when a gate gains a condition, count its cells before writing
the test. "The feature is tested" and "each condition is separately falsifiable"
are different claims, and the second is the one an engineer asking *"which half
broke?"* actually wants.

## 2. ★★ The four cells are NOT equally licensed, and a red means something
different in each

| cell | licence | what a red means |
|---|---|---|
| v4 high | **QUOTED** — 7.2.15 "shall be set to zero" | the code is wrong |
| v4 low | **INFERRED, 2 steps** (`A56`) | check the inference is still the project's reading first |
| v2 high | 6.1.11 imposes nothing; 6.1.8's identical boilerplate makes the half vendor space | iccce is making a **FALSE report against a conformant file** — the worst of the four |
| v2 low | **nothing in ICC.1** | a project **policy** changed; allowed, but must not change silently |

That table is in the module doc *and* in the `IF_THIS_FAILED` panic text,
because nobody reads a module doc out of a CI log.

## 3. ★ An exact-string assertion cannot tell a report VANISHING from a report
gaining four words

The old file asserted `reports(V4) == vec!["…outside the defined 0..=3"]`. When
the gate landed, the `Display` impl gained `(ICC.1:2022 7.2.15 + Table 23)` and
**the v4 arm went red for a reason that had nothing to do with the fix.**

Shape adopted: **presence and rule** are asserted on the typed value
(`Malformation::UnknownRenderingIntent { value, rule }`, which is `PartialEq`);
**wording** is asserted in a separate test, by substring, and stated as a claim
about emitted text. Injection D below proves they are independent.

## 4. ★★ Injection results — each reversion lands on exactly one arm

Detached worktree, four injections, ten tests:

| injection | RED |
|---|---|
| A: gate collapsed to one condition (the pre-fix defect) | `cell3_v2_high_half_is_not_reported_at_all` — **and nothing else** |
| B: v2 arm deleted (v2 never reports) | `cell4_…`, `the_wording_…` |
| C: v4 arm masked to the low half | `cell1_…` |
| D: v2 wording collapsed to the v4 wording | `the_wording_…` — **all four cells stayed green** |

D is the one worth keeping: a behaviour-preserving change to the emitted text
is caught by exactly the arm that exists for it. Note that B reds *two* arms
because a missing report makes the wording arm's length assertion fail — that
is expected, not sloppiness.

Worktree note again: build a fresh `git worktree add --detach`, copy the
uncommitted `crates/` + `fixtures/` in, and tear down with `rm -rf` then
`git worktree prune` (`git worktree remove` still fails "Filename too long" on
the scratchpad path).

## 5. ★ MANIFEST.md QUOTES emitted text, and nothing kept the quotation true

`recipes.rs`'s `expect` fields are typed literals quoting `Malformation`'s
`Display` output. The v4 row went false the moment the citation was added —
the **fourth** instance of the family in
[[project-stale-claim-strings-in-emitted-records]].

Interpolation is unavailable here **by design**: `tools/gen-profiles` must never
link `iccce-profile`, or a fixture could inherit a bug from the code it tests.
The available substitute is to read the generated file **as text** from the test
crate — `the_manifest_quotes_the_report_text_that_is_actually_emitted` asserts
every emitted string appears verbatim in `MANIFEST.md`, plus that the silence
row says `ZERO malformations`. The arrow stays safe: the test reads the
manifest, the generator still reads nothing. It caught the stale row on its
first run.

Related: [[project-a-third-fixture-category-disputed]],
[[project-header-rendering-intent-finding]],
[[project-prove-the-arm-by-injecting-the-defect]],
[[project-malformation-channel-overclaims]].
