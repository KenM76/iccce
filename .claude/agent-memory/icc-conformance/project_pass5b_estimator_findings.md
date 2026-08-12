---
name: project-pass5b-estimator-findings
description: Pass 5b (PARTIALLY SUPERSEDED by Pass 5c) — the ISO/CD 18619 estimator is implemented but has NO CALLER; the corpus's pre-registered prediction split 2 CONFIRMED / 1 FALSIFIED / 1 NOT ESTABLISHED; and a CMYK destination's gamut boundary absorbs 90% of a black-point disagreement.
metadata:
  type: project
---

Measured **2026-08-12** at pin `21c582a`. Apparatus
`tools/difftest/src/pass5b.rs`; record `tools/difftest/README.md` **§17**;
tolerances `docs/TOLERANCES.md` **§3.5.7**. Fixture: `USWebCoatedSWOP.icc` as
**destination**, system sRGB source, **media-relative** — lcms2's method-4
territory and the case Pass 5 row P19 recorded as a gap.

**★★ THE ISO ESTIMATOR HAS NO CALLER.** `bpc::estimate_lut_destination_black`
is implemented, documented and unit tested; **nothing outside its own `mod
tests` calls it**. `Chain::estimate_dst_black` in `transform.rs` still carries
the pre-ISO subset (v4 + perceptual → the A41 constant, else
`BpcEstimationUnsupported`). So the shipped `iccce transform --bpc` still
refuses a v2 CMYK LUT destination at media-relative — Pass 5b **grades that
refusal** and drives the library function in process for everything else. Check
this is still true before quoting any Pass 5b row as being about the binary.

**The two estimators, read at the pin (`cmssamp.c`):** the ramp, the monotonic
pass, the sample count and **the root of the quadratic are identical** — so
iccce's "root not vertex" correction is a correction of *Adobe*, not of lcms2.
Two things differ: lcms2 **holds the ramp's chroma constant** where ISO 4.2.5.2.2
ramps it to zero, and lcms2 **returns `Lab.a = InitialLab.a`** (L592) where ISO
4.2.3 returns a neutral black. ★ Unpredicted third difference: lcms2 clamps the
chroma to ±50 *for the ramp* and returns the **unclamped** value — READ, not RUN.

**Measured black points:** ISO **L\* 16,4898 neutral**; lcms2 **L\* 17,2150,
a\* 0,3472, b\* 0,3001** (chroma 0,4589). Divergence **0,858 17 ΔE76**.

**★★ The corpus's pre-registered prediction, claim by claim:**

| claim | verdict |
|---|---|
| mechanism (ISO drops chroma, lcms2 keeps it) | **CONFIRMED** — but *structural* on iccce's side; it grades that 4.2.3 is implemented, not that the prediction was right |
| magnitude 2–6 ΔE76 | **FALSIFIED**, robustly — 0,4589, and even + the whole error bar = 1,27 < 2,0. The band assumed a *chromatic* printer black; SWOP's darkest colorant is 0,834 off neutral |
| shape ("the divergence IS the chroma") | **NOT ESTABLISHED** — L\* term 0,725 is 1,58× the chroma term but is *inside* the 0,814 error bar, and the only mechanism available is 13× too small (oracle-free: unneutralising the InitialLab moves the root only 0,0543 L\*) |
| decay to zero at white | **CONFIRMED** — 0,0878 → 0,0531 → 0,0135 → 0,0088 → **0,0000**, monotone |

**A four-part prediction needs four verdicts.** A single headline would have been
wrong whichever way it went.

**★ The finding neither Pass 5 nor the prediction contains:** of 0,858 ΔE76
between the two estimated blacks, **only 0,0878 survives end to end — the
destination absorbs 90 %**. `A2B1(B2A1(Lab(0,0,0)))` returns L\* 16,4898, this
profile's **gamut floor**, which is the ISO estimate to four decimals; both
blacks are at or below it and clip to the same ink combination. *A disagreement
about the black point is not the same size as a disagreement about the output,
and on a CMYK destination the gamut boundary decides which.*

**★ The apparatus row failed twice before it passed, and the bound never moved.**
lcms2's black must be *recovered* (`transicc` cannot print one) through
`A2B1(B2A1(·))`. v1 probed `L* ∈ [0,20]` — mostly **outside this profile's
gamut** — and failed at 16,49 **on the gamut boundary**. v2 re-derived the bound
as a **ratio at 1,0** ("an error bar is readable exactly when it is smaller than
what it bounds", zero free parameters) and failed at 1,107 because a maximum over
a 15-L\* band prices in curvature the recovery never touches. v3 measures the
**local** residual at the two estimated blacks: **0,948, a 5 % margin, and the
row says so.** A marginal apparatus quoted as green makes a whole section
unfalsifiable.

**★★ PARTIALLY SUPERSEDED 2026-08-12 by [[project-pass5c-estimator-branch-finding]].** lcms2's estimator was reimplemented from source: **98,3 % of the 0,858 ΔE76 above was the recovery**, lcms2's black on SWOP is **neutral**, claim 1's CONFIRMED is **withdrawn**, claim 3 is settled both ways, and **neither implementation fits a quadratic here** — both take the straightness short-circuit. The true divergence on this fixture is 0,0817 ΔE76, entirely L\*. Claims 2 and 4 and the 90 %-absorption finding stand.

**Owed, highest value first:** a harness reimplementation of
`cmsDetectDestinationBlackPoint` (constant-chroma ramp + its own
`BlackPointAsDarkerColorant`), which removes the error bar entirely and settles
claim 3 either way. Then wiring the ISO estimator into `Chain`. The **v4
perceptual arm still needs the synthetic v4 LUT fixture with a non-zero device
black — NOT built, still owed.**

Related: [[project-lcms2-findings-pass5-bpc]],
[[project-oracle-and-tolerance-state]],
[[project-synthetic-fixture-corpus-and-gp001]].
