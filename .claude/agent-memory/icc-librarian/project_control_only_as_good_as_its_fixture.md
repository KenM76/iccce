---
name: iccce-control-only-as-good-as-its-fixture
description: DL-025 — a sensitivity control can be nullified by its own FIXTURE, and its scaling law must match the function's smoothness class; plus the three-instance pattern that instruments, not re-reading, catch this project's errors
metadata:
  type: project
---

**DL-025 (2026-08-12, Pass 6): a sensitivity control is only as good as
its FIXTURE, and the scaling law it asserts must match the SMOOTHNESS
CLASS of the function under test.**

**Why:** Pass 6's compiled-transform control failed twice before it
worked, and both failures would have shipped a number.
(1) The fixture was **sRGB→sRGB**. A compiled grid reproduces an
identity chain **exactly everywhere**, not merely at its nodes —
n-linear interpolation is exact on a linear function. Result
**1.1×10⁻¹⁵ at ratio 0.94**: no h² scaling, no discrimination, and
**without the control that figure would have been reported as "the
compiled path's cost."**
(2) Refixtured to sRGB→AdobeRGB, probing the whole axis gave **ratio
1.44** — because sRGB's TRC kinks at 0.04045 and error across a
derivative discontinuity scales **h¹, not h²**. Neither code nor fixture
was wrong; the *expectation* was. Fixed by probing the smooth region.
**DL-023 had predicted this Pass's null-by-construction trap by name at
the previous filing, and it was walked into anyway** — which is the
argument for mechanical controls over remembered rules.

**How to apply:** whenever a Pass reports an agreement or an error
bound, ask two questions **before** filing the number — *would this
control return the same thing if the effect were identically absent?*
(if yes, the fixture nullifies it) and *is the asserted scaling law
valid over the region probed?* Publish the control's **failures** in its
own doc comment; a correct control disagreeing with a wrong expectation
is evidence, not debris, and must never be answered by widening the
band. Also record the control's **passing margin** — Pass 6 recorded
both failing ratios (0.94, 1.44) and **not** the passing one.

**The pattern this is the third instance of.** In iccce the thing that
catches an error is **never** a re-reading of the code and **never** the
number looking wrong (10⁻¹⁵ looks magnificent) — it is always **an
apparatus built to fail**:

| Pass | About to ship | Caught by | Filed |
|---|---|---|---|
| 3 | a curve evaluator off by one sample | an **exact-value** test (the round trip would have passed) | DL-016 |
| 4 | an `mBA ` curve count the corpus couldn't supply | a **refusal by name**, discharged by an independently authored fixture (GP-001) | DL-020 |
| 6 | an error of 1.1×10⁻¹⁵ measuring nothing | a **sensitivity control**, which failed on its own fixture | DL-025 |

Rule 1's corollary, now stateable from three worked cases: **a wrong
measurement looks exactly like a right one.**

Related: [[iccce-pass-status]], [[iccce-free-to-disagree]],
[[iccce-gate-must-not-reward-deletion]],
[[iccce-bound-cannot-catch-its-own-magnitude]],
[[iccce-refusal-discharged-by-fixture]].
