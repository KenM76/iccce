---
name: iccce-rule7-can-run-against-us
description: DL-030 — the first time "disagreement with lcms2 is a finding, not a failure" resolved against iccce; the pre-commitment written before the answer existed is what made it cheap, and withholding attribution is what made the measurement usable
metadata:
  type: project
---

**When a measured divergence's mechanism is identified but its
attribution is not, file the number and WITHHOLD the interpretation in
writing — including a pre-commitment to the outcome that goes against
this project.**

**Why:** on 2026-08-12 the ISO/CD 18619 **4.2.5.4** question came back
against iccce. The clause specifies the `DestinationBlackPoint` *"shall
be the same as **InitialLab**"*; iccce returned `outRamp[first]` (which
occurs in the whole clause only as `MinL`, a threshold — never a
black-point candidate). **lcms2 conformed. iccce shipped a
non-conformance.** Corrected at `fd34a44`; cost **0,0817 ΔE76** on SWOP
= **100 % of NC-142's divergence**, *measured before it was found*.

**What made it cheap:** `NUMERIC_CLAIMS.md` §3.18.6 had written, before
any answer existed — *"if ISO names lcms2's, iccce is WRONG, not
divergent, and the engineer changes the code … **rule 7 is not a licence
to assume iccce is right**"* — and had forbidden any document from
describing the difference as lcms2 departing from the standard. So the
correction needed **one sentence added**, not a retraction. A section
that had filed it as a finding *against lcms2* would have been
publishing a wrong claim for a day.

**How to apply:**

- Project rule 7 says the **specification** settles a disagreement — not
  that ours is the right side. A rule that has only ever exonerated the
  project has not been tested.
- **A measurement whose attribution is left open is what makes a later
  answer actionable.** Name the single line the divergence must be, and
  stop there.
- **Re-attribute rather than edit.** NC-142's number was correct and
  stayed; only the sentence beside it changed. What moved was a *claim*,
  not a *number*.
- ★ **Watch what a re-attribution does downstream**: NA-009's "cost" had
  been that very number, so the correction left the approximation
  **UNMEASURED again** — more honestly than before. A named-approximation
  register must never carry a bug as though it were a priced departure.
- The corpus had **not transcribed 4.2.5.4 verbatim**, so it could not
  have caught this: **a corpus gap and an implementation bug with the
  same root.** The lesson is not "transcribe more" — it is that **the
  unexplained measurement is what made the gap findable.**

Related: [[iccce-pass-status]], [[iccce-free-to-disagree]],
[[iccce-predicted-divergence-must-be-measured]],
[[iccce-direction-scoped-behaviour]].
