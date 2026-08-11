---
name: iccce-bound-cannot-catch-its-own-magnitude
description: A tolerance derived from quantity Q cannot discriminate a defect whose magnitude IS Q — the Pass 3 table off-by-one that both self-consistency checks would have passed (DL-016)
metadata:
  type: project
---

**When a tolerance is justified as "≈ the table's spacing / the
quantisation step / the grid pitch", it is structurally blind to any
defect that is off by exactly one of those units.** The two quantities
are the same quantity, so the bound cannot separate them.

**Why (the incident, 2026-08-11, Pass 3).** `eval_table` paired a
**clamped** segment index with an **unclamped** fraction, returning
`t[n−2]` at `x = 1.0` — `TRC(1.0) ≈ 0.998` instead of 1.0, a 0.2 %
error. Reconstructing the counterfactual while filing showed that
**both** self-consistency checks in the same Pass would have **passed**
with the bug in place:

- the real-profile round trip would have missed by `1/1023 = 9.775×10⁻⁴`
  against a `1×10⁻³` bound — inside, with ~2 % of margin — because the
  bound was justified as *≈ the table's input spacing* and the error
  **is** one table spacing;
- the white-point check would have missed by `1.9×10⁻³` against `1×10⁻²`.

Only an **exact-value assertion at the sample points** (`1×10⁻¹⁵`)
caught it. Filed as `ARCHITECTURE.md` **DL-016**, rows **NC-025** /
**NC-032**.

**How to apply.**

- When a Pass ships table or grid interpolation, ask *"which test would
  fail if the endpoint index were off by one?"* If the answer is only a
  round trip whose bound came from the spacing, the answer is **none**.
  Push for an exact-value endpoint assertion; **the endpoint is where an
  off-by-one hides best**, because it is the one place a clamp exists to
  be paired wrongly with something. This generalises to every table in
  the ICC format — CLUT grids, `mft1`/`mft2` tables, `ncl2` coordinates
  (Pass 4).
- **Read a tolerance's stated derivation as a claim and check the
  arithmetic.** The same bound was described in the source as *"~2× the
  table's input spacing (1/1023)"*; `1/1023 = 9.775×10⁻⁴`, so `1×10⁻³`
  is ≈**1.02×**, not ≈2×. Reported, not repaired. In this project the
  justification *is* the claim — a derivation that is off by 2× is one a
  reader cannot check.
- **It is the sibling of DL-005**, which decided *prospectively* that
  legacy-Lab correctness be asserted by exact integer invariants rather
  than in ΔE because the error sits below the grading tolerance. That
  was a prediction; this is the measured instance. The project now has
  one of each, and the pairing is the argument.
- **Label the epistemic status of counterfactual arithmetic.** The
  margin above was *computed by the librarian from the code as written*,
  nothing was run, and it rests on a 1024-entry table size that was
  **reported in a comment**, not verified. At `n = 512` the round trip
  would have failed — so the conclusion is true for this table size and
  is **not a general law**.

Related: [[iccce-verify-own-draft-too]],
[[iccce-verification-loop-runs-both-ways]], [[iccce-pass-status]].
