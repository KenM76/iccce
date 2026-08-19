---
name: no-paid-standards-use-reconstruction
description: Ken declined to buy IEC 61966-2-1 (CHF 210); build a reconstruction RAG from free sources instead and refine it over time — do not re-open the purchase question
metadata:
  type: project
---

**Decided 2026-08-19.** Ken **will not buy IEC 61966-2-1** (CHF 210 for
pp. 16–51, the only remaining route to `published-ground-truth` for a real
transform). Instead: *"do your best guess using what is available online,
make a rag of the best guess of the standard and use it, refine it if more
information becomes available."*

He also accepted, explicitly, that iccce's honest public claim stays
**"matches lcms2 within a stated tolerance"** rather than "provably
correct" — offered as a plain-language trade and chosen deliberately.

**Why:** the cost is not justified for a project at this stage. It is a
one-off ~CHF 210 for a claim upgrade, not for any working functionality;
nothing is blocked by its absence.

**How to apply:**

- **Do not re-open the purchase as a recommendation.** It stays recorded as
  an available upgrade path in `would_be_upgraded_by:` fields, and that is
  the right place for it. Raise it again only if Ken signals iccce is
  becoming something third parties depend on.
- ★ **The reconstruction gets its OWN evidence class, never
  `published-ground-truth`.** A value assembled from secondary sources is
  not read from the normative text. The class is weaker than ground truth
  but **not** weaker than cross-check-against-an-implementation, and is
  stronger in one specific way: being implementation-independent, it can
  catch an error lcms2 and iccce make *together*. That is the entire value
  of the exercise and the reason it is worth doing at all.
- **Strength scales with source independence**, so the source count and
  whether the sources actually derive from one another is part of every
  claim. W3C CSS Color 4 is downstream of the same paywall and must not be
  counted as independent.
- **Disagreements between sources are the most valuable output**, because
  they mark exactly where a from-memory implementation goes wrong.
- ★ Values from the ICC's own sRGB document (Jack Holm, 2015) are
  **already** cited as `ground-truth` in Pass I. **Do not demote them** by
  folding them into the reconstruction.

Related: [[feedback_compatibility_not_compliance]] — the same instinct, that
Ken does not want work stalled on an unobtainable certification artefact.
The reconstruction RAG is that principle applied to a paywall rather than to
measurement hardware.
