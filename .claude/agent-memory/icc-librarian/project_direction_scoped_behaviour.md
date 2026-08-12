---
name: iccce-direction-scoped-behaviour
description: DL-021 + DL-027 — a measured implementation behaviour is a fact about one DIRECTION, one PATH and one PROFILE CLASS until measured in the others; four lcms2 instances, incl. two black-point estimators picked by the destination's header
metadata:
  type: project
---

**DL-021 (`ARCHITECTURE.md` §5, filed 2026-08-11 at the Pass 4b
filing).** *A measured implementation behaviour is a fact about the
direction and the path it was measured in, until it is measured in the
others.*

**Why:** three lcms2 behaviours, **all in `cmsio1.c`-adjacent code**,
had been written into this project's documents as unqualified rules and
each turned out to hold in **one direction or one tag type only**:

1. `_cmsReadOutputLUT` calls `ChangeInterpolationToTrilinear` for **any
   Lab-PCS LUT** ⇒ **NA-006's measured 1,5741 ΔE2000 is an *A2B*
   number; the B2A interpolation envelope is exactly ZERO.**
2. Forced BPC is keyed by the **DESTINATION** profile's version
   (`DefaultICCintents` consumes `BPC[i]` as the conversion *into*
   `hProfiles[i]`) — DL-013 / corpus M2 as written are half a rule.
3. The legacy 16-bit PCSLAB stage is inserted for **`lut16Type` only**,
   not `lut8Type`. Getting that wrong costs **≈0,2 ΔE2000** — *under*
   the perceptibility anchor, invisible to every ΔE gate.

**It bit this project's own writing too:** three documents predicted a
gray differential would price **NA-008**. The differential ran
**GRAY→RGB**, and NA-008 lives in the gray **destination** path — so it
is still unmeasured. *"A gray differential"* named a comparison, not a
direction.

**How to apply.** (a) Every record's scope line names the **direction**
(device→PCS or PCS→device, source or destination, and the tag type), not
just profile/pin/intent — an omitted direction is as incomplete as an
omitted pin. (b) ICC.1 is built of mirrored pairs (`A2Bx`/`B2Ax`,
`mAB `/`mBA `, `lut8`/`lut16`, source/destination): **assume the twin
differs until measured.** (c) Quote a cost with its direction attached,
permanently. (d) **When a method difference collapses to zero the
comparison gets weaker, not stronger** — agreement between two
implementations running the *same* algorithm proves sameness, not
correctness, so a **counterfactual** must price what the comparison
could have seen (Pass 4b's tetrahedral arm: **99–139×**). That is
DL-018's discipline moved from a deleted requirement to a method.

**★★★ DL-027 (filed 2026-08-12) adds a THIRD axis: the CLASS OF
PROFILE.** `cmsDetectBlackPoint` branches **before** the darkest-colorant
code everyone here had been reading (`cmssamp.c` L370–374): at relative
colorimetric, an **output-class profile in an INK colour space** goes to
`BlackPointUsingPerceptualBlack`, which **forces the chroma to zero**;
**anything else** goes to `BlackPointAsDarkerColorant`, which **keeps
it** — and `cmsDetectDestinationBlackPoint` returns `InitialLab`'s
`a`/`b` verbatim, so **the branch IS the returned chroma**.

> ***"Does lcms2 keep its black point's chroma?"* has NO ANSWER.** One
> answer for a CMYK press profile, the opposite for an RGB printer
> profile — and **the only real LUT profile on this machine is the first
> kind**, which is why eleven filings of reading never saw it.

**The cost, and why it is the strongest version of this lesson:** the
corpus's **pre-registered** prediction resolves **FALSIFIED on the CMYK
arm and CONFIRMED on the RGB arm** (8,1668e-2 ΔE76 100 % `L*` vs
5,000000 ΔE76 100 % chroma). **A session that ran only one arm would have
filed a confident wrong headline either way, supported by a clean,
tight, honestly bounded measurement.**

**How to apply the new axis.** Before writing a behaviour down as a rule,
**read the CONDITION that selects it**; if the condition names **header
fields** (device class, colour space, version), the rule needs a **second
arm chosen to fail that condition** — and if the corpus has no profile of
the second class, that is a coverage gap to state, not a rule to
generalise. *(Related: DL-026's conjunction trick — when a condition is a
conjunction, a confound may be removable by choosing INPUTS.)*

**The defect is in the transcription, not in lcms2** — each behaviour
has a rationale in its own place.

**Corpus twin:** [[iccce-refusal-discharged-by-fixture]] (DL-020) says a
blanket sentence over a mirrored pair is a defect class. **DL-020
governs how a *specification* is transcribed; DL-021 governs how an
*implementation's behaviour* is.**

Related: [[iccce-pass-status]], [[iccce-gate-must-not-reward-deletion]],
[[iccce-predicted-divergence-must-be-measured]],
[[iccce-verify-own-draft-too]].
