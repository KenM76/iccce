---
name: iccce-tolerance-cannot-swallow-and-claim
description: DL-019 and the Pass 4 gate design — a tolerance wide enough to absorb a method difference cannot also demonstrate agreement; use a wide structural gate PLUS a tight gate with the difference switched off, and report-not-grade what nobody can adjudicate
metadata:
  type: project
---

**One number cannot both admit a difference and prove agreement.** When
a comparison's dominant term is *not error* — a method difference, or a
policy divergence — a single tolerance either swallows it (and then
proves nothing) or fails forever (and then stops being read).

**Why.** Pass 4 (2026-08-11) compared iccce's `lut16` CMYK pipeline to
lcms2. The dominant term was the **CLUT interpolation-method
difference** — n-linear vs lcms2's four-input hybrid — which ICC.1 does
not choose between (ambiguity **A16**, SILENT). It is **1.6590 ΔE2000**,
it is nobody's bug, and it will not go away. NA-006 had named the trap a
filing in advance: *"a tolerance wide enough to swallow ~1 ΔE cannot
also demonstrate agreement."*

**How to apply — the pattern that worked, in three parts:**

1. **A wide, STRUCTURAL gate whose value IS the envelope** (2.0 ΔE00
   here). It catches a wrong index order, a wrong Lab decode, a swapped
   ink — defects ≥1000× its own resolution. **Its record must say on its
   own face that it cannot claim agreement.**
2. **A tight, ARITHMETIC gate with the difference switched off.** Two
   forms, both used: **substitute the other implementation's algorithm**
   into your own pipeline and change nothing else (4.8154e-3 vs 2e-2 —
   *this* is the row that claims agreement, a 326× shrink); and find
   **inputs where the difference cannot arise at all** — here the 16
   CLUT-node corners, where both schemes read the same stored value and
   the oracle's quantisation vanishes rather than accumulates
   (**6.6558e-5 vs 1e-3**, the print floor). *Without a node-only
   control, a wide structural gate could hide a genuine 1.9 ΔE error.*
3. **Grade the apparatus first.** The harness's reimplementation was
   held against the shipped `Lut16Model` at 1e-9 → **0.0 exactly**.
   Otherwise the whole substitution experiment rests on an assertion
   that the reimplementation is faithful.

**★ And when nobody can say which side is RIGHT — DL-019.** At the
absolute intent iccce and lcms2 read different destination media whites
(iccce `wtpt` as stored; lcms2 substitutes D50 for a **v2 display-class**
profile) and differ by **11.217 ΔE2000**. The spec question (corpus
**A4b**) is unsourced. The posture, five **conjunctive** steps:
(a) emit the raw comparison with an **infinite** tolerance, labelled
**REPORTED, NOT GRADED**; (b) make the gate the **modelled** comparison —
the same run with the other implementation's policy substituted and
nothing else changed (517× collapse, gated 5e-2); (c) **write both
rejected alternatives down at the record** — widening to ~15 ΔE00 is *a
number chosen because it passed*, and a permanent red line *stops being
read*; (d) state the blocking question in full to a named owner;
(e) note that the ungraded status ends the moment the question is
answered, because then one implementation acquires a defect.

**The limit that keeps this honest:** report-not-grade is available
**only** when the mechanism is modelled and **the model is itself
gated**. An *unmodelled* disagreement is an unexplained one, and the
right response to that is a failing gate.

**Quoting discipline that follows:** a Pass 4 number is meaningless
without naming its gate — corners, emulated, or raw differ by four
orders of magnitude.

Related: [[iccce-gate-must-not-reward-deletion]],
[[iccce-bound-cannot-catch-its-own-magnitude]], [[iccce-pass-status]],
[[iccce-predicted-divergence-must-be-measured]].
