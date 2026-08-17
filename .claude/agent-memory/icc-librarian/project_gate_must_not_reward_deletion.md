---
name: iccce-gate-must-not-reward-deletion
description: DL-018 — an upper-bound gate on a DELIBERATE cost goes greener when the requirement is deleted; it must be paired with a prediction pin plus a sensitivity control (Pass 3's clamp, Pass 5's BPC)
metadata:
  type: project
---

**Some measured quantities are not error — they are the price of doing
the right thing. Grading one with an upper bound builds a gate with a
perverse gradient.** Filed as `ARCHITECTURE.md` **DL-018**, 2026-08-11.

**Why:** iccce's range clamping (Annex F.8–F.16, normative) discards the
difference between two profiles' encoded media whites, and that discard
**costs ΔE in a round trip**. The Pass 3 round-trip gate is 2.5e-2 and
the observation is 1.8788e-2 — but **delete the clamping and the round
trip becomes the exact identity, so the gate passes MORE comfortably
while a normative requirement has been removed.** The suite stays green,
the number *improves*, and the improvement is the symptom. Same failure
family as DL-016 (a bound cannot discriminate a defect whose magnitude
is its own justification), one level up: there the bound could not *see*
the error; here it **rewards** it.

**How to apply.** When a check's metric is dominated by a deliberate,
required cost, the upper-bound row is **not sufficient alone**. Add:

1. a **prediction pin** — assert |predicted − observed| where the
   prediction is computed **independently** (Pass 3: closed form from
   the two colorant matrices and the clamp alone, no tone curve, no
   lcms2, no measurement — 1.878244e-2 predicted vs 1.878818e-2
   observed, **0.03 %**); its tolerance comes from the **measurement
   chain's** precision (1e-3 = 10× the ~1e-4 ΔE00 floor of the binary's
   6-decimal print), never from the effect's size;
2. a **sensitivity control** — show the pin would fail without the
   requirement (Pass 3: **19×**). An apparatus not shown able to detect
   the effect it looks for is not an experiment.

**★ State the pin's scope, because the first draft of Pass 3's got it
wrong.** The pin was claimed to make the F.8–F.16 clamp **ordering**
falsifiable. It does not: iccce clamps at **three** independent sites
(F.8–F.16, 10.18's domain, F.1(b)'s attainable range), so the other two
make the first redundant **at the shipped surface**, and no test in the
repo distinguishes clamp-before from clamp-after through the binary.
Recorded as **owed, not covered** — corrected in place, not deleted, so
a reader can tell "checked and narrower than hoped" from "never
checked".

**Where it comes due next: Pass 5, BPC** — same shape and worse (BPC
exists to change the result; DL-013 records lcms2 forcing it on at
≈3.15 L*). Also any Pass 4 metric dominated by a clip or gamut-mapping
step. **Not** every self-consistency row: most price an approximation
whose removal makes the metric *worse*, and those are well-conditioned.

**★★★ UPDATED 2026-08-17 — DL-055 is the MIRROR of this, and it is worse
in one specific way: NOTHING MOVES.** A crash fix had **two layers** (a
computed ≥5-channel grid recommendation, and `MAX_COMPILED_GRID_BYTES =
64 MiB` behind a named `ChainError::GridExceedsBudget`), and **each layer
alone makes the conformance row observe zero**: the smaller default means
the allocation succeeds **whether or not the guard exists**, so
**deleting `MAX_COMPILED_GRID_BYTES` would have left the row GREEN.**

DL-018's hazard is a gate that gets **greener** when a costly requirement
is removed — a diff of tolerances can at least show that. **Here no
tolerance changes, no expectation changes, no test is deleted, no number
is edited.** A *second* change elsewhere quietly takes the first out of
the loop. ★★★ **A change ledger cannot record it, because there is
nothing to record.**

**The rule:** when a fix has more than one layer, ask of every row that
observed the defect ***"which layer is in the loop?"***, not *"what does
this row measure?"* A row must exist that **fails if any single layer is
removed** — `icc-conformance` split one row into **four, one per layer**
(`TOLERANCES.md` §3.8.4.3). ★ **An in-process unit test and a CLI row are
different layers**: the unit test asserts the guard's arithmetic without
attempting the allocation (right — *a test that aborts the test process
proves nothing*), and is blind to whether `bench` propagates the `Err` as
exit 1. **"Belt and braces" is the tell: two mechanisms are redundant only
if EACH has a defect it alone can catch.**

★★ The same shape in the differential-test register is **DL-056** —
[[iccce-agreement-can-be-the-symptom]]. Filed with **NC-234/NC-235**
(`iccce bench` **aborted the process**, `0xC0000409`, on ICC's
seven-channel APTEC profile: a `_ => 33` catch-all giving `33⁷ × 3 × 8 ≈
952.6 GiB`, and a guard using `checked_pow`, **which catches WRAP not
SIZE**). ★ **An abort is the worst available library failure** — not
catchable; the consumer's process goes with it. **Rule 6 at the
allocation layer.**

Related: [[iccce-bound-cannot-catch-its-own-magnitude]],
[[iccce-pass-status]], [[iccce-verify-own-draft-too]],
[[iccce-agreement-can-be-the-symptom]],
[[iccce-absence-of-publication-is-not-evidence]].
