---
name: project-passk-f-separating-fixture
description: Pass K §F closed the ZERO-SEPARATION hole with a committed gen-profiles fixture whose B2A contaminates neutrals by construction — and the injection proof showed a collapsed fixture does not merely fail to inform, it turns the headline red row GREEN; also caught one of my own separation claims being false because a symmetric misreading cancels.
metadata:
  type: project
---

**Built 2026-08-17 on the engineer's dispatch, closing the item Pass K left
open** ([[project-passk-black-preservation-baseline]]). Recipe
`v2-cmyk-chromatic-neutral` in `tools/gen-profiles/src/recipes.rs` §4.2;
`tools/difftest/src/passk.rs` §F (7 rows, 40 in Pass K); bounds
`docs/TOLERANCES.md` **§3.10.11**; notes `tools/difftest/README.md` **§25.12**.
Suite `pass=325 fail=1` → **`pass=331 fail=2 skip=9 error=0`**, both failures
the same predicate on two profiles. Separation `4.207 049×10⁻¹`.

**How to apply.**

- ★★★ **A `ZERO-SEPARATION` fixture does not merely fail to inform — it
  MANUFACTURES A FALSE PASS.** Injection A (zero the `B2A0` chromatic samples,
  i.e. the sibling's construction) turned `F5`
  *k-only-in-implies-k-only-out* **GREEN at `0.000000`** and gave the
  transition-width row a full cell, which *looks like a working feature*. Only
  `F2`/`F3` failed. **The design argument was "cannot discriminate"; the
  measurement is worse than the argument.** This is the concrete answer to
  *"why grade the separation when the classifier already flags it"* — the
  `ZERO-SEPARATION` verdict would have printed in a column beside a green row
  in a run whose summary said `fail=1`.
- ★★★ **The class you can reach without an oracle is `DerivedExpectation`, and
  it is only worth the name if the HARNESS reads the bytes.** §F's `Mft2Bytes`
  walks the tag table and decodes `mft2` in `tools/difftest`, deliberately not
  through `iccce-profile`: a parser that read the CLUT wrongly would otherwise
  make the expectation wrong *in the same way* as the observation. Six rows are
  `DerivedExpectation`; `F7` is the paired lcms2 **third reading** the enum's
  own doc asks for.
- ★★★ **A SYMMETRIC misreading cancels, so it cannot be a row's rival.** `F4`/
  `F7` first named DL-005's legacy-vs-general PCSLAB confusion. **False of
  those rows**: the derivation works in *encoded fractions* end to end, so a
  consumer applying the general rule in **both** legs round-trips to identical
  numbers. Replaced with clause 10.10's **CLUT index order read backwards**,
  **evaluated** from the same bytes (`eval_reversed`) rather than asserted:
  `4.843 550×10⁻¹`, `31 738×` the bound. *Generalisation: before naming a
  rival, ask whether the defect appears TWICE in the chain and cancels.*
- ★★ **Design the fixture so no interpolation scheme can matter, then say what
  that costs.** Three choices, each removing a term from a bound rather than
  allowing for it: (1) both models **affine, no cross terms** → `NA-006`'s
  envelope is identically zero, so `1/65535` is arguable; (2) `B2A0`
  **`a*`/`b*`-independent across node lines 3,4,5** — because `a* = 0`
  (`8000h` = 32 768) is **NOT a node**: node 4 of a 9-node axis sits at
  32 767,5; (3) darkness on the **encoded** `L*` fraction, because legacy
  PCSLAB's top node decodes to `L* = 100.390 6` and an `L*`-based model clamps
  **inside the cell the K ramp's white end lands in**. ★ The price: the fixture
  is *less like a press than anything in Ghent*. Say it as loudly as the gain.
- ★★ **Build the regression guard on inputs the FEATURE cannot legally touch.**
  `F4`/`F7` use 50 **chromatic grays** `(c, 6c/7, 0.984127c, k)` — the family
  for which this `A2B0` gives `a* = b* = 0` exactly. All of `C M Y` strictly
  positive ⇒ not K-only under any definition ⇒ **no preservation path may move
  them**, so the guard survives the feature while a guard on the K ramp cannot.
  Read with `F5`: red + green ⇒ the red means what it says; red + red ⇒ the
  fault is in reading the fixture.
- ★★ **A lower bound is graded as a SHORTFALL at zero** (`Record::graded` is
  `observed ≤ tolerance`): `F3` observes `max(0, floor − separation)`. The
  floor `4×10⁻²` is **10× Pass G's `SWEEP_DEVICE`**, the loosest device bound
  the family has justified — **derived from the tolerance budget, never from
  what the fixture measures.** Same shape as §B1's refutation row.
- ★ **A tolerance whose worst case is ATTAINED is not a near miss.** `F2`
  observes exactly `0.5/65535` because several authored values land on a half
  code. `<=` is deliberate in `Record::graded`; the row passes at the
  encoding's extremum and fails at any *changed model*.
- ★ **Guard the fixture's SHAPE or the harness panics instead of reporting.**
  `analyse_separating` checks channel counts and that the grid exceeds the
  dead-band index — the sibling has a 3-node grid and `node(&[li,4,4])` would
  have panicked. A panic is the worst outcome: the suite dies and nobody learns
  which fixture was wrong.
- ★ **`E1` NOT repointed, `E6` NOT deleted.** The Ghent row stays red and
  skipping; `E6`'s `ZERO-SEPARATION` is the measurement that says *why* a second
  fixture had to exist. §F closes the **gradeability** gap, not the
  **population** gap.
- **CI consequence, deliberate:** the `oracle` job is now **red permanently**
  until the feature lands. Floor raised `15 → 22`; the floor step and summary
  re-emission made `if: always()` so a deliberately red suite does not silence
  the guard on CI's reach.
- **Not done:** no injection of a *consumer-side* defect for `F4`/`F7` (needs a
  `crates/` edit in a detached worktree); §A–§E still uninjected; `NUMERIC_CLAIMS.md`
  unfiled for all of Pass K including §F (librarian's, free id `NC-243`).

Related: [[project-prove-the-arm-by-injecting-the-defect]],
[[project-candidate-separation]], [[project-synthetic-fixture-corpus-and-gp001]],
[[project-stale-claim-strings-in-emitted-records]],
[[project-passg-tolerance-lessons]].
