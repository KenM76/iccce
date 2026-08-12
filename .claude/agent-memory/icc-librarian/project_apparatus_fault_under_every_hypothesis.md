---
name: iccce-apparatus-fault-under-every-hypothesis
description: DL-028 — a residual that is large under EVERY hypothesis is an apparatus fault, not a finding; and an error bar the same order as its effect may BE the measurement (Pass 5b's 98.3%)
metadata:
  type: project
---

**DL-028 (`ARCHITECTURE.md` §5, filed 2026-08-12 at the
estimator-discrimination filing).** *A residual that is large under every
candidate explanation is an apparatus fault, not a finding — so carry a
SECOND, INDEPENDENT candidate wherever the point of the experiment is to
discriminate, and grade the RATIO, not the magnitude.*

**Why — the incident.** Pass 5c's synthetic arm's **first** run reported
a device residual of **9,98e-2** where the truth is **8,9e-6**, and would
have been filed as *"the reimplementation does not reproduce lcms2 on
this fixture."* The cause was not colour: **`transicc` prints ink spaces
as `0..100` and RGB/gray as `0..255`**, and Passes 5, 5b and 5c had all
divided by 100 — **correct code for every destination the project had
ever measured**, wrong the first time one was RGB.

**What caught it:** the validation arm carries **two** candidates (the
lcms2 model and the ISO model) and **both missed by roughly the same
amount**. The discrimination row — whose whole job is to ask *"can this
experiment tell the two apart?"* — is where that shows, because a unit
error inflates both numerators identically: the **absolute** numbers
merely looked disappointing, the **ratio** said the experiment had
stopped discriminating.

**★ The corollary, which is the sharper half.** Pass 5b bounded a
*recovered* black point with an error bar of **0,8137** against an effect
of **0,85817** and reported the ratio as **0,948 — marginal, passing by
5 %**. Pass 5c then showed the recovery **was** the effect: **98,3 % of
the published number was apparatus.**

> **When an error bar is the same ORDER as the effect, the honest reading
> is not "the result is marginal" — it is "the apparatus may be measuring
> ITSELF."** A 5 % margin on an error-bar row is a row saying, correctly,
> that the experiment is not yet built.

**How to apply.**

- In any filing that grades a **discrimination** (which of two
  explanations is right), check that a **ratio between candidates**
  exists. If the section has one arm and one candidate, say so in the
  coverage statement — it cannot detect its own apparatus.
- When a number is large and *every* explanation misses similarly,
  **suspect units, scaling, and the harness** before writing a finding.
- **A carried-unchanged tolerance across an apparatus replacement is
  strong evidence it was never fitted**: Pass 5c reused Pass 5b's `1,0`
  constant and its derivation verbatim and scored **0,3043** (33× tighter)
  — that is worth stating whenever a constant survives a rebuild.
- Do **not** delete a superseded row. Pass 5b is filed in
  `NUMERIC_CLAIMS.md` §3.17 **with its overturned verdicts visible**:
  *what an instrument reported before a better instrument existed is the
  only evidence that the better instrument was needed.*

**Family.** Third instance in two days of *the instrument, not the
reading, is what catches an error*: **DL-016** (exact values at sample
points), **DL-025** (a control is only as good as its fixture), **DL-028**
(this). In all three, re-reading the code was available and would not
have worked.

Related: [[iccce-control-only-as-good-as-its-fixture]],
[[iccce-direction-scoped-behaviour]], [[iccce-pass-status]],
[[iccce-verify-own-draft-too]], [[iccce-gate-must-not-reward-deletion]].
