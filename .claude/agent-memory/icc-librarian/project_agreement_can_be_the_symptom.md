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

**★★★ UPDATED 2026-08-17 — DL-056 is the DESIGN RULE that follows from
DL-033, and it caught the librarian's own reasoning.** DL-033 is the
*diagnosis*; DL-056 says what to do about it.

**A DIFFERENTIAL test is blind in the direction that moves your answer
TOWARD the reference.** Measured, not argued — injected drift into the
constructed sRGB white's `Z`:

| drift | max ΔE2000 | the ΔE test |
|---|---|---|
| `−1.0×10⁻³` | `0.101968` | FAILS ✔ |
| `−3.0×10⁻⁴` | `0.050149` | FAILS ✔ |
| **`+3.0×10⁻⁴`** | **`0.029008`** | **PASSES — better-looking than the correct build's `0.033013`** |
| `+2.0×10⁻³` | `0.146450` | FAILS ✔ |

The reference file's own white sits **`+1.885×10⁻⁴` above D50**, so
drifting upward moves us *toward* it; blind to ≈`+3.8×10⁻⁴`, where it
would report **zero**. ★★★ **Not fixable by tightening: a difference
cannot detect a defect that shrinks it.** Same shape as the black-point
estimator above.

★★★ **And it corrected this librarian's own §3.32.9a**, filed hours
earlier, which argued the observed-≡-derived coincidence made the row *"a
tighter white-point gate than any flat constant."* **True downward, false
upward.** A careful argument from re-reading was corrected **by an
injection** — §5.2 applied to the agent whose job is reading
([[iccce-verify-own-draft-too]]).

> **The rule: every differential gate NAMES the ABSOLUTE assertion that
> covers its blind direction, or declares that none exists.** Absolute =
> compared against a **sourced constant with no reference artifact in
> it**. Here: `constructed_colorant_sum_is_d50` (vs `D50` itself, `1e-9`,
> no file) — the same `+3.0×10⁻⁴` **fails it while all six differential
> tests pass**. ★★ **Deleting it as "redundant, the ΔE test covers it"
> would open the blind spot AND EVERY REMAINING TEST WOULD STAY GREEN
> WHILE IT HAPPENED**, so both sites carry a DO-NOT-DELETE note
> **containing the injection table** — the table is the argument.

★★ **DL-056 is DL-055's mechanism in the differential-test register**
([[iccce-gate-must-not-reward-deletion]]). Shared sentence: **a redundancy
is only a redundancy if each member has a defect it alone can catch;
otherwise it is a single mechanism with a decoy beside it.** ★ Revisit if
a reference artifact is replaced by one sitting *below* D50 — the blind
direction flips and any prose about it goes wrong while every test stays
green.

Related: [[iccce-pass-status]], [[iccce-rule7-can-run-against-us]],
[[iccce-control-only-as-good-as-its-fixture]],
[[iccce-bound-cannot-catch-its-own-magnitude]],
[[iccce-gate-must-not-reward-deletion]],
[[iccce-absence-of-publication-is-not-evidence]].
