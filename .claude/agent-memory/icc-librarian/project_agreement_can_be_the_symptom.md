---
name: iccce-agreement-can-be-the-symptom
description: DL-033 — a cross-check's power is bounded by the SEPARATION of the two candidate answers, not the tightness of the residual; iccce's 4.2.5.4 defect agreed with lcms2 to 0.08 dE76 while being 4.72 L* wrong, and fixing it made the cross-check 58.8x WORSE
metadata:
  type: project
---

**Agreement with the oracle was the symptom of our defect.** Filed
2026-08-12 as `ARCHITECTURE.md` **DL-033**, `NUMERIC_CLAIMS.md` §3.25,
**NC-165 … NC-167**.

**Why:** iccce's ISO/CD 18619 4.2.5.4 short-circuit returned
`outRamp[first]` where the clause says `InitialLab` (that defect is
DL-030 — see [[iccce-rule7-can-run-against-us]]). The cross-check built
to catch exactly this reported a divergence of **`8,166 8×10⁻² ΔE76`**
against lcms2 — small, clean, honestly bounded, and **read as
agreement**. The defect's own magnitude was **`4,717 441 L*`, i.e.
57,8× larger than the residual it was blamed for**. The reason is
exact, not lucky: the non-conformant return was
`outRamp[first] = MinL`, and **`MinL(lcms2) = MinL(ISO) = 16,489 806`
exactly** — iccce was returning a quantity the oracle *also* computes,
so it landed near lcms2's answer **for a reason unrelated to being
right**.

**Correcting the code made the cross-check WORSE: `8,166 8×10⁻²` →
`4,799 109 ΔE76`, 58,8× larger.** Neither number grades correctness —
the *clause* did that. The residual now measures a **definitional**
gap: ISO 4.2.2.2 means the darkest **device vertex** neutralised;
lcms2's `cmsDetectBlackPoint` means the **perceptual black round trip**
with chroma zeroed. **Two documents, one name, neither side wrong.**

**Three things this filing also established:**

- **A row must state ONE claim.** NC-164a carried *"the defect explains
  the whole gap"* (**measured, true** → NC-174) and *"so fixing it ends
  the gap"* (**never measured, false** → NC-175) in one sentence. The
  unmeasured inference inherited the measurement's authority **by
  adjacency**, and nothing flagged it because the grammar and the
  arithmetic were both fine. **Split, never edited.**
- **The predicted collapse was PRE-COMMITTED AND LEFT UNASSERTED**, so
  this was an observation rather than a retraction — the same move that
  made DL-030 cheap. See
  [[iccce-predicted-divergence-must-be-measured]].
- **DL-036 — the authored fixture had ZERO power.**
  `v4-rgb-mab-chromatic-black.icc`'s `InitialLab` and `outRamp[first]`
  are **both `L* 20`**, so the defect swapped two equal numbers and its
  `5,000 000 ΔE76` is *identically* unmoved. The **vendor** profile was
  the only arm that could see, **because nobody designed it.** The
  stated converse of DL-020 ([[iccce-refusal-discharged-by-fixture]]):
  an authored fixture discharges *the doubt it was authored for* **and
  nothing adjacent to it.**

**How to apply.** Before quoting any `implementation-cross-check` row
as evidence a value is right, ask **how far apart the two candidate
answers were**. A small residual is evidence of **proximity**, not
correctness. Where the two implementations share an intermediate
(`MinL` here), the check may have no power at all.

**★★ UPDATED 2026-08-12 (seventeenth filing) — the "none of them has
one" clause is now WRONG, and the correction has a fraction attached.**
`tools/difftest` emits a candidate separation on every record
(**DL-037**, [[iccce-disclosure-is-not-enforcement]]). ★ **The gap is
closed for Pass 5c and open everywhere else: 16 of 145 rows, all Pass
5c's; 129 print `UNSTATED`.** DL-033's *"Revisit if"* asked for a worked
positive example and now has one. ★★★ **And the instrument's first act
was to show that the row carrying THIS ENTIRE FINDING is graded at
INFINITY** — a DL-019 report-don't-grade row — **so it could never have
failed, whatever the candidates did.** Its separation is `4,717 441`,
the defect magnitude above. **DL-033 said the cross-check was "nearly
blind, now merely uninformative"; the emitted field says something
stricter — on that row it was never a check at all.**

DL-033 is the **mirror of DL-028**
([[iccce-apparatus-fault-under-every-hypothesis]]) and the more
dangerous half: DL-028's failure announces itself, this one is silent.

Related: [[iccce-pass-status]], [[iccce-rule7-can-run-against-us]],
[[iccce-control-only-as-good-as-its-fixture]],
[[iccce-bound-cannot-catch-its-own-magnitude]].
