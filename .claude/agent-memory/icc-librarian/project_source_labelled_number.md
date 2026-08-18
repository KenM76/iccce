---
name: iccce-source-labelled-number
description: DL-061 — a number carrying a SOURCE LABEL makes two claims and only the value is ever tested; "GWG's own patch value" was PRINTED INTO A REPORT for a figure the patch does not contain, and the artefact that would falsify it keeps its numbers in a FIGURE where no text tool can reach them
metadata:
  type: project
---

**A number labelled with its source makes TWO claims — the value, and
where it came from — and only the first is ever tested. Where a value is
chosen *because* an external artefact uses it, the label is part of the
claim and gets verified against the artefact, at the line that states
it.** Corollary: **if the artefact states its values in a FIGURE, no text
extraction can contradict the label. The check is a render, or there is
no check.**

**Why:** on 2026-08-18 `tools/difftest/src/passk.rs:1342` documented its
`at_half` field as *"the same ΔE at `g = 0.5` — **GWG's own patch
value**"*, and **`:2446` printed that phrase into the Pass K report**.
**The GWG 23.0 patch's gray panel is 25 %, not 50 %.** The ΔE at
`g = 0.5` is measured correctly and is a fine sample of the ramp —
**only the reason given for choosing it is false.**

**★★★ Why it is worse than a bare number, and not a typo:**

- **Nothing recomputes.** No ΔE, no tolerance. **A change ledger has
  nothing to record** — [[iccce-gate-must-not-reward-deletion]]'s DL-055
  shape, arriving in a doc comment.
- **The label is what made it credible.** *"GWG's own patch value"* is
  the sentence that stops a reader looking it up. **DL-057 said a wrong
  clause is worse than a vague one because the citation makes the
  argument persuasive** — same mechanism, applied to a number
  ([[iccce-wrong-clause-refusal-and-discarded-halves]]).
- **It LEAVES THE REPOSITORY.** DL-031's finding — a bare count in
  emitted text can never be corrected afterwards — applies to an
  attribution exactly as to a count ([[iccce-count-needs-its-apparatus]]).
- **No test can fail on it.** DL-051 established documentation is not
  tested; **an injection harness perturbs values, and this defect has no
  value to perturb** ([[iccce-documented-is-not-tested]]).

**★★ Where the wrong pair most plausibly came from — a mechanism, not an
accusation.** `GHENT_COMPATIBILITY.md` **§3.3, one table after the wrong
row, attributes "50 % K, 50 % Gray and 50 % spot black" to GWG 3.0** and
calls it the deceptive gray-equivalence lookalike. ⇒ **a number can
migrate between adjacent rows of one table without anybody mistyping
anything, and it arrives wearing the destination row's subject.** Filed
as a *reading*, labelled as one.

**★★★ The second defect in the same module is the same class:**
`passk.rs:291` lists the patch's fourth panel as an **`ICCBased` gray**;
the readme says **`DeviceN`**. §C's reasoning survives on its own terms,
**but not if line 291 is read as its warrant** — DL-049's shape, a defect
in a *justification* rather than in a candidate.

**Do not round up what was actually established.** Everything rests on
the patch's **README**, read from a raster. **The patch PDF's content
stream has never been opened**, and the corpus deliberately ships two
corrupted trap profiles. **Write "the readme declares", never "the patch
paints".**

**How to apply:** when a constant, an abscissa, a grid size or a
threshold is justified by *"X's own value"*, treat the attribution as a
claim needing the same verification as the number. Ask **which line of
which artefact states it** — and if the artefact's numbers live in a
figure, **render it**, because every text tool in this project will
return silence and the silence reads as absence.

Related: [[iccce-patch-named-for-what-it-looks-at]] (DL-059 — this is its
corroboration and its sequel), [[iccce-inferred-environment-constraint-is-a-reading]]
(DL-060 — the route that made the figure readable, and the second,
broader failure mode it exposed), [[iccce-documented-is-not-tested]],
[[iccce-disclosure-caught-a-bad-justification]],
[[iccce-stale-citation-worse-than-stale-number]], [[iccce-pass-status]].
