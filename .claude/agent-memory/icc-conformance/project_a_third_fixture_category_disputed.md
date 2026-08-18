---
name: project-a-third-fixture-category-disputed
description: 2026-08-18 — added Category::Disputed because the well-formed/malformed taxonomy is BINARY and both options are CLAIMS; it EMPTIED the same day when the sourcing landed, which is the category working. Includes what does NOT belong in it, and how to prove a test MOVES by injecting the FIX.
metadata:
  type: project
---

Built `v2-rendering-intent-high-bits` (recipe + fixture + 
`crates/iccce-profile/tests/rendering_intent_high_bits_not_version_gated.rs`)
to measure a latent parser defect: `Header::parse` checks
`rendering_intent > 3` with **no version gate**, and ICC.1:2001-04 6.1.11
imposes nothing on the high 16 bits that ICC.1:2022 7.2.15 requires to be zero.

Four things below are durable and none is derivable from the code.

## 0. ★★ STATUS: the category is EMPTY, and that is the outcome it was built for

Filed `v2-rendering-intent-high-bits` disputed in the morning; `icc-spec-librarian`
returned ICC.1:2001-04 6.1.11 the same day; the fixture moved to **WellFormed**
and its manifest row now asserts a **silence**. Nothing about the bytes changed
— only what could truthfully be claimed about them. Kept the variant rather
than deleting it: deleting removes the *option*, and the next fixture built
ahead of its sourcing faces the same forced guess with no third answer. An
unused enum variant costs one match arm. `cmd_manifest` now prints an explicit
"no fixtures are currently in this category" sentence instead of a bare heading
— an empty heading reads as a generator bug.

★ **What does NOT belong in Disputed**, both of which I nearly got wrong:

* *iccce's behaviour is believed wrong.* That is a **defect**: file the fixture
  under what the **standard** says, and let the suite go red. A category is not
  a place to park a bug.
* *the text has been read and licenses more than one consumer behaviour.* That
  is **settled**, not disputed — the answer is "ICC.1 does not determine this".
  File under what the file **is** and record the project's choice in the row
  **as a choice**. Worked example: `v2-rendering-intent-low-half`, which
  violates nothing and which iccce reports anyway, deliberately.

The one condition that DOES put a fixture here is about the *sourcing*: a
dispatch is outstanding and its answer would change the category.

## 1. ★ The two-category taxonomy forces a guess, and I added a third

`Category::WellFormed` and `Category::Malformed` are not descriptions — they
are **claims about what a conformant consumer must do**. `WellFormed` asserts
silence is correct; `Malformed` asserts a report is correct. A fixture built to
probe a rule whose text nobody has read *on that exact point* fits neither, and
the taxonomy makes its author pick one **inside `MANIFEST.md`, which is
generated and therefore read as authoritative**.

Added `Category::Disputed` (`tools/gen-profiles/src/recipes.rs`, plus a section
in `cmd_manifest`). Its `expect` field carries a **dated measurement of current
behaviour**, explicitly not a requirement. The fixture moves out of the
category when the sourcing lands, and **that move is the visible event the
category exists to produce**.

**How to apply:** when a fixture's conformance status is open, do not round it
to the nearer of two claims. Blast radius was small — enum variant, `label()`,
one tuple in the manifest loop, two README paragraphs, `MANIFEST.md`
regenerated.

## 2. ★ The counterpart fixture did NOT need to be byte-identical, and arguing
why is better than achieving it

The obvious design was "the v4 fixture with the version bytes flipped". It is
wrong: the v4 base carries `mluc` metadata and `parametricCurveType` TRCs, both
v4-only, so the flip produces **two era violations in one fixture**, against
the corpus's one-mutation rule. Built it on `v2_rgb_matrix_trc_curv_spec()`
instead.

Isolation is then carried two better ways:

- **Structurally** — the malformation is raised inside `Header::parse` from the
  128 header bytes alone, so no difference in tag types *can* reach the
  observable. That is a property of the code path, not an assurance.
- **By a control that is one generator call away** — `v2-rgb-matrix-trc-curv`
  is the fixture's own base, differs in exactly the four bytes at offset 64
  (asserted), and reports zero malformations (asserted).

Contrast `probe_profile` (`v2-rgb-mft2-lab`/`v4-rgb-mft2-lab`), which **is**
byte-identical: there the observable is a *tag decode* the version selects
between, so the tags had to be held constant and a metadata non-conformity in
the v4 member was the accepted price. **Byte identity is worth paying for when
the observable is downstream of tags, and not when it is in the header.**

## 3. ★★ Prove the test MOVES by injecting the FIX (the mirror of the usual trick)

[[project-prove-the-arm-by-injecting-the-defect]] injects a *defect* to prove a
row is load-bearing. Here the test asserts believed-possibly-wrong behaviour,
so the thing to inject is the **prospective fix**: added
`&& header.version.major() >= 4` in a detached worktree. Three arms went red,
three stayed green — and **which three stayed green is the finding**: the two
controls and the apparatus check are insensitive to the fix by design, and the
scope test (which counts fixtures by reading `header.rendering_intent`, not by
counting reports) stays green because it measures the corpus, not the parser.

**Trap that cost ten minutes:** the injected-run output was filtered with
`grep -v "license"` to suppress Cargo's SPDX warnings, and that **silently ate
every custom assertion message**, because the failure text ends
*"record the clause that licensed the gate."* The left/right values alone read
as if the wrong fixture had been measured. Filter Cargo noise by
`grep -v "SPDX\|manifest.html"` only — never on a word a test message can
contain.

Worktree note: `git worktree remove --force` fails with **"Filename too long"**
on the scratchpad path; `rm -rf` then `git worktree prune` works.

## 4. A test that asserts believed-wrong behaviour needs its warning IN the
failure message

Shape chosen: assert today's measurement so the suite goes **red the moment the
gate lands**. The alternative — a test that prints and always passes — leaves
the defect with no trip-wire at all, which is the
`mpet_fallback_disclosure` failure mode.

What makes it honest rather than a trap: a `const IF_THIS_FAILED` prefix
appended to every assertion message, saying a failure here is the *expected
outcome of the fix and not a regression*. **Nobody reads a module doc out of a
CI log**, so the disclosure has to be in the panic text. And the file is named
after the **absence of the gate** — a present-tense, checkable fact — never
after the correctness of anything.

Two clippy notes: `assert!(CONST & 0xFFFF <= 3)` trips
`assertions_on_constants`; rewriting it to assert on the value **read back from
the fixture** removes the warning *and* makes the check non-tautological.

Superseded in part by [[project-the-four-cell-gate-and-its-injections]], which
records what replaced the test named in §4 once the gate landed.

Related: [[project-header-rendering-intent-finding]],
[[project-passk-f-separating-fixture]], [[project-doc-editing-conventions]].
