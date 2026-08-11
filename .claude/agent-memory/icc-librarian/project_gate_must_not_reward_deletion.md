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

Related: [[iccce-bound-cannot-catch-its-own-magnitude]],
[[iccce-pass-status]], [[iccce-verify-own-draft-too]].
