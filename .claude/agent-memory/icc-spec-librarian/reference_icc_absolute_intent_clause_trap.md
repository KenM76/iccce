---
name: icc-absolute-intent-clause-trap
description: Where the ICC-absolute colorimetric formula lives in the corpus, and the ICC.1:2022 clause (6.2.3) that states its white-point ratio backwards — check before anyone implements absolute intent from spec prose
metadata:
  type: reference
---

**ICC-absolute colorimetric material lives in
`D:\Dev\Rag-Specialized\ICC_Spec\icc\icc__s__rendering_intents.md`** (built
2026-08-11, 4th pass, to close the remainder iccce's Pass 3 recorded).
It carries the equations, the `wtpt` clause, the `chad` answer, Table 25 in
full, and the `shall`-level intent-fallback order (8.10.2).

**The one thing worth holding in memory rather than looking up: ICC.1:2022
clause 6.2.3 states the composite white-point ratio INVERTED.** Its prose
says to scale PCS values by "the ratio of the **destination** profile
mediaWhitePointTag **to the source** profile mediaWhitePointTag". Chaining
the document's own Equations (4) then (1) gives **`mw_src / mw_dst`** — the
reciprocal. Confirmed by two independent derivations, by Annex D.6.2 f)
stating the direction correctly, and by lcms2's `ComputeAbsoluteIntent`
(source read, not measured).

**Why this needs to survive between sessions:** 6.2.3 is the *narrative*
paragraph — it is the natural thing to read and quote when someone asks
"how do I do absolute?", and it is wrong. The equations in 6.3.2.2 and
Annex D are right. **An implementation built from the prose fails silently
in one direction** (values clip to paper white; absolute looks identical to
relative, so it reads as an unimplemented feature) **and loudly backwards in
the other** (the proof comes out lighter and bluer than the target).

**How to apply:** if any future dispatch involves absolute-colorimetric
rendering, proofing, paper simulation, or `wtpt` scaling, point at
`icc__s__rendering_intents.md` §7 and `icc__ref__spec_defects.md` §12
*before* the engineer reads clause 6.2.3. Two related facts from the same
file that also get assumed wrongly: **`chad` is never applied at render
time** (6.2.1 NOTE 1, E.4), and **for a conforming v4 display profile
absolute ≡ media-relative** because `wtpt` `shall` equal the PCS
illuminant — so "absolute does nothing on my monitor profile" is correct
behaviour, not the bug above.

Related: [[icc-pdf-symbol-font-sign-loss]] (the equations are stacked
fractions — anything read out of them is a reconstruction),
[[label-the-predicate-not-just-the-payload]],
[[reading-source-is-not-observing-behaviour]] (the lcms2 corroboration in
§7.4 is a source read and is labelled as one).
