---
name: project-conformance-can-worsen-the-crosscheck
description: Measured on iccce 2026-08-12 — fixing iccce's non-conformant ISO 4.2.5.4 branch made the lcms2 cross-check 58.8x WORSE (0.0817 to 4.799 dE76); the defect had been masking a real spec-vs-lcms2 disagreement, and "this defect explains the whole gap" is not the same claim as "fixing it ends the gap".
metadata:
  type: project
---

**The measurement.** `pass5c/swop/estimators/black-points-in-lab`, harness
`cc03f3d`, `iccce-cmm` carrying `fd34a44`, oracle pin `21c582a`.

| | before `fd34a44` | after |
|---|---|---|
| ISO/CD 18619 4.2.5 black | `L* 16.489806` | **`L* 11.772365`** |
| lcms2's black (reimplemented) | `L* 16.571474` | **unmoved** |
| divergence | 8.1668e-2 ΔE76 | **4.799109 ΔE76** |

**Why:** both sides take the mid-range straightness short-circuit and both now
return a quantity their own document calls `InitialLab`. ISO 4.2.2.2's is the
darkest **device vertex** neutralised; lcms2's is the **perceptual black round
trip** with chroma forced to 0. The whole divergence is that the two documents
mean different things by one name. Neither is a bug.

**How to apply.**

- **Agreement with an oracle can be the SYMPTOM of a defect.** iccce's
  non-conformant return (`outRamp[first] = MinL = 16.489806`, which is not a
  black-point candidate in any branch of 4.2.5) sat 0.082 `L*` from lcms2's
  answer; the conformant one sits 4.799 away. The defect's own magnitude is
  **4.717441 `L*` — 57.8x the divergence it was blamed for.** It was nearly
  invisible in the cross-check built to find it. This is `CLAUDE.md` rule 3
  with a number on it.
- **Separate the two attributions when writing them up.** *"This defect
  accounts for 100 % of the observed gap"* was measured and is true.
  *"Removing it will close the gap"* was never measured and was false.
  `NUMERIC_CLAIMS.md` NC-164a states the first; do not let anyone read it as
  the second.
- ★ **Watch ratio rows that improve.** T1 (`apparatus/error-bar-…`) went
  3.043e-1 → 5.179e-3 with an **unchanged error bar** — its effect grew 59x.
  T4 (`validation/reimplementation-beats-…`) went 1.715e-1 → 4.258e-2 with an
  **unchanged numerator** — its rival got 4.03x worse. Both look like progress
  in the TSV and neither is. Always print both terms of a ratio.
- **The synthetic fixture could not see this at all** (its `InitialLab` and
  `outRamp[first]` are both `L* 20`, so its 5.000000 is unchanged). The
  vendor profile was the only arm that could. Keep a real profile beside the
  authored ones.
- **NA-009's cost is now measurable**: 4.799109 ΔE76 (swop, all `L*`) and
  5.000000 (synthetic, all chroma), carrying to 9.921e-3 and 5.725e-2 device
  at input black. It is a cost **at the black point** and **relative to
  lcms2**, not relative to truth — no published black point exists for
  `USWebCoatedSWOP.icc`.

Filed in `docs/TOLERANCES.md` §3.5.8.6 and `tools/difftest/README.md` §19.10.

Related: [[project-pass5c-estimator-branch-finding]],
[[project-stale-claim-strings-in-emitted-records]],
[[project-lcms2-findings-pass5-bpc]].
