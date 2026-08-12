---
name: iccce-disclosure-is-not-enforcement
description: DL-037 — candidate separation is an emitted field whose GUARD ORDER is the decision (UNGRADED before the comparison, ZERO-SEPARATION outranks all); BLIND deliberately does not gate, because a disclosure mechanism and an enforcement mechanism must not be the same mechanism
metadata:
  type: project
---

**A mechanism that discloses a weakness must not also punish it.** If
stating a weakness can turn a green row red, the cheapest response to a
weak row is **to stop stating weaknesses**.

**Why:** on 2026-08-12 `icc-conformance` built the instrument
[[iccce-agreement-can-be-the-symptom]] (DL-033) had asked for —
`tools/difftest` now emits a **candidate separation** on every record:
the named rival answer, the value the row would have observed under it,
and the distance. Its automatic `BLIND` verdict (candidates closer
together than the row's own tolerance — precisely the configuration that
hid the 4.2.5.4 defect) **deliberately does not affect status or exit
code.** `unstated` prints as `UNSTATED` rather than as a blank, so
declining to state one is **visible but never punished**. Filed as
`ARCHITECTURE.md` **DL-037**. ★ **This is DL-018 read from the other
end** — DL-018 says a gate can be made greener by deleting the
requirement it protects; here the incentive is removed at the source by
having no gate to make greener.

**★★★ The guard ORDER is the decision, not a detail:**

1. `Unstated` / `NoNamedAlternative` — nothing to compare (state 2
   records that a **person looked**, and carries the reason)
2. `NaN` → **`sep-broken`** — apparatus breakage said out loud, never
   classified into a verdict
3. distance `== 0` → **`ZERO-SEPARATION`**, **before everything below**
4. incommensurable units → number emitted, **test not run**
5. tolerance not finite → **`UNGRADED`**, ★ **BEFORE the comparison**
6. else `distance <= tolerance` → `BLIND`, else `DISCRIMINATING`

- **Why 5 must precede 6:** `d ≤ ∞` is true for **every** finite `d`, so
  comparing first would brand **every report-don't-grade row (DL-019)
  `BLIND`** — *blaming the fixture for a decision the TOLERANCE made*.
  The new mechanism's first output would have been a systematic slander
  of the corpus.
- **Why 3 outranks the rest:** the two states differ **in their remedy,
  not their severity**. A blind row is rescued by **tightening a
  tolerance**; a zero-separation row only by **a different fixture**.
  Collapsing them files a fixture-authoring job under "adjust a number".

**★★★ What it found on its first run, both about our own instruments:**

1. **The row carrying the entire 4.2.5.4 finding is `UNGRADED`** —
   tolerance `inf`, separation `4,717 441` (the defect's magnitude to
   six figures). **It could never have failed however far the candidates
   moved.** The `inf` was *correct* under DL-019; the finding is that
   *"we measured it"* and *"we could catch a regression in it"* had been
   indistinguishable in the record. The suite's real power lives in §B's
   **device** rows — **the row named `estimators` is the one that cannot
   grade estimators.**
2. **A fourth stale literal** — see [[iccce-pass-status]] and DL-034.
   Found **by an apparatus rather than by a person**.

**★★★ UPDATED 2026-08-12 (eighteenth filing) — the guards were right and
the ARITHMETIC UNDER THEM was wrong.** Guard 3 (`ZERO-SEPARATION`, the
one that outranks everything) fired on a row that was **failing at
`2,500 019×10¹` at that moment**, because the distance was *derived* as
`|observed − alt_observed|`. **DL-038 —
[[iccce-measurement-under-the-guards]].** Nothing in the guard order is
superseded; what changed is the number handed to it. **Note this is NOT
the "wrong rival" case DL-037 deferred — the rival was right — and that
case is still uncovered.**

**Coverage moved at the same filing: 41 of 160 rows now state a
separation** (was 16 of 145); **119 print `UNSTATED`**; **16 reached the
final comparison** (was 6). ★ **So `16` now names a different quantity
than it did one filing ago.**

**How to apply:**

- ★★ **Never quote `blind=0` — or `16` — without its denominator.** At
  the seventeenth filing **16 of 145 rows** carried a separation and
  `blind=0` was strictly out of the **SIX** that reached the comparison
  (145 → 16 stated → 12 measured → 6 compared). At the eighteenth it is
  **41 of 160**, with **16** *discriminating*. Preserve the engineer's
  phrasing *"out of 16, not out of 145"* **with its filing attached**,
  then sharpen it — **in that direction only.**
- **A stated separation is not a TRUE one.** The rival candidate is
  named by a human from the two implementations' sources; a wrong or
  missing rival yields a confidently wrong separation and nothing
  detects it. [[iccce-free-to-disagree]] (DL-023) guards the input and
  is unaided by this.
- **Do not sweep `unstated` → `no-named-alternative` in bulk.** Without
  a reason **per row** it destroys the field's meaning while making the
  aggregate look finished.

Related: [[iccce-agreement-can-be-the-symptom]],
[[iccce-count-needs-its-apparatus]], [[iccce-gate-must-not-reward-deletion]],
[[iccce-tolerance-cannot-swallow-and-claim]],
[[iccce-control-only-as-good-as-its-fixture]], [[iccce-pass-status]].
