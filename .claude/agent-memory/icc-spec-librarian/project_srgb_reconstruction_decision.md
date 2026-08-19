---
name: srgb-reconstruction-decision
description: Ken decided 2026-08-19 NOT to buy IEC 61966-2-1 (CHF 210) — sRGB is reconstructed from free sources under the new `reconstructed_consensus` class; never re-file the purchase as a next step, and never let the reconstruction be cited as if it were the standard
metadata:
  type: project
---

**Ken decided on 2026-08-19 not to purchase IEC 61966-2-1** (CHF 210 for
pp. 16–51 of publication `6169`). His instruction, verbatim: *"you can just do
your best guess using what is available online, make a rag of the best guess of
the standard and use it, refine it if more information becomes available."*

**Why:** the price is not justified by what it would settle. Three independent
standards bodies already restate the constants; what the purchase would answer
is narrower than it looks — chiefly `A54c`, whether IEC **fixes** `0.055` /
`0.04045` or **derives** them from a continuity criterion.

**How to apply:**

1. **Never file "purchase IEC 61966-2-1" as an actionable next step again.** It
   is decided. The corpus records this in `_sources\README.md` and on `A54c`.
   Affected rows carry **`would_be_upgraded_by:`** instead, so the question
   answers itself if the document ever arrives by another route (library,
   standards subscription, employer account). *That field IS the mechanism for
   "refine it if more information becomes available" — it is not a good
   intention, it is a grep target.*
2. **The reconstruction has its own evidence class and it must never be promoted
   by accident.** `reconstructed_consensus`, defined canonically **once**, in
   `D:\Dev\Rag-Specialized\ICC_Spec\EVIDENCE_CLASSES.md`. The name was chosen so
   a grep for `ground_truth` never returns it. **Do not alias it, do not write
   `reconstructed_ground_truth`.**
3. **The correct citation form matters and is written down** at
   `iec\iec__ref__srgb_reconstruction.md` §10. *"three independent
   standards-body restatements"* — **never** *"per IEC 61966-2-1 clause 5"*.
4. **Ken accepted a deliberately weaker artifact in exchange for cost.** The
   honest limits are enumerated in that file's §8 (nine items). **Keep them
   enumerated.** The value of this decision depends entirely on the limits
   staying visible; a reconstruction whose caveats erode becomes a fabrication.

**The payoff to state when this comes up:** the reconstruction is **independent
of any implementation**, so it can catch an error `iccce` and lcms2 make
together — which an lcms2 cross-check structurally cannot. That is the specific
gap it closes, and the only one.

Related: [[iec-srgb-primary-sourcing-state]], [[published-ground-truth-state]],
[[icc-corpus-gap-vs-nonexistence-claim]],
[[corpus-defects-are-caught-from-outside]]
