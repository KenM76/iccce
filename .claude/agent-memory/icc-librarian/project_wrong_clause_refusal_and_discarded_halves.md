---
name: iccce-wrong-clause-refusal-and-discarded-halves
description: DL-057 — a refusal that names the WRONG clause is worse than a vague one because the citation makes it persuasive; and a path that reuses machinery "and discards part of the result" inherits the discarded half's failure modes, since that half's error is what the caller sees
metadata:
  type: project
---

**A code path that reuses machinery *and discards part of the result*
inherits every failure mode of the part it discards — because the
discarded half cannot fail harmlessly. Its error is what the caller
sees.** Filed 2026-08-17 as `ARCHITECTURE.md` **DL-057**,
`NUMERIC_CLAIMS.md` **§3.33 / NC-237**, §3.33.7.

**Why:** `Chain::with_destination(src, Destination::None, ..)` obtained
the source model by building a **scaffold chain `src → src`** and
discarding the destination half. That works whenever the source can also
serve as a destination — which every profile tested at the time could. It
fails for a profile with an `A2B` tag and **no `B2A`, no colorant matrix,
no `grayTRC`** — a conformant shape, and **four such profiles are in
ICC's own published set** (colour-vision-deficiency simulation profiles:
`scnr`, Lab PCS, one-directional by design). Found by scanning both
private corpora **for the shape**, not by review.

**The symptom is the filing-worthy part.** It reported:

```text
matrix/TRC model requires PCSXYZ (Annex F.3, normative); profile PCS is 'Lab '
```

**True. Correctly clause-cited. And about a model iccce was about to
throw away.** A caller reads it as *"my source profile is unusable"* —
false.

> ★★★ **A refusal that names the WRONG clause is worse than a vague one,
> because the citation makes it persuasive.** A vague refusal invites
> investigation; a precise, clause-cited one invites acceptance. **This
> is the project's founding hazard (rule 1 — a wrong colour looks exactly
> like a right one) arriving in the ERROR SURFACE rather than in a colour
> value**, and it is DL-048's mechanism in a third register: a pointer
> that is individually correct and points at the wrong thing, so arrival
> reads as confirmation ([[iccce-stale-citation-worse-than-stale-number]]).

**Corollary for rule 6.** *The parser reports and does not repair* gains a
second half: **a report must be ABOUT THE THING THE CALLER ASKED ABOUT.**
A truthful report about an internal scaffold is not a report — it is a
leak.

**How to apply.**

- When auditing a path that calls a constructor for a side effect and
  drops part of its result, the question is **not** *"is this correct?"*
  but ***"whose error message does the caller see if the discarded half
  fails?"***
- **Fix by extraction, not by special case.** `derive_source_model()` is
  now **shared** with `new_inner`, so the ICC.1 **8.10.2** fallback
  dispatch — the most intricate logic in the crate — has one copy and
  cannot drift. A local special case removes the instance and leaves the
  class.
- ★ **Pair the fix with the test that stops an over-broad future fix.**
  Beside *"these profiles now work as sources"* sits *"these same
  profiles are still correctly REFUSED as destinations"*; without it the
  first test can go green **for the wrong reason** — DL-020's discipline
  ([[iccce-refusal-discharged-by-fixture]]) applied to a pair of tests.
- **Coverage, stated:** four profiles, one class, found by a shape scan
  of two private corpora. **No claim that four is all of them.**

Related: [[iccce-pass-status]], [[iccce-absence-of-publication-is-not-evidence]],
[[iccce-gate-must-not-reward-deletion]],
[[iccce-agreement-can-be-the-symptom]].
