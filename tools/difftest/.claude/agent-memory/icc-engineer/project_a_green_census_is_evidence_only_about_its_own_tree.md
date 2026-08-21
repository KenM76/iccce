---
name: project-a-green-census-is-evidence-only-about-its-own-tree
description: the difftest suite was red at HEAD for two days while every doc recorded it green — re-run it each session, never quote a recorded pass/fail as current
metadata:
  type: project
---

**Re-run `tools/difftest` at the start of any session that will reason about
it. Never quote a recorded `pass=N fail=0` from a document as the current
state.**

**Why:** on 2026-08-21 the suite was found **red at `HEAD`** —
`pass=372 fail=1 skip=9` at `0a88ad6` — while every document in the repository
recorded `pass=353 fail=0`. Nothing was wrong when that census was written: it
was measured at `3c93b62`, and the `profileID` fix that turned a row red
landed **three commits later**. Two days of sessions read the green number and
none re-ran it.

★ **The failing row was itself the interesting part.** It graded
`malformations:` — a count of *disclosures* — and so accused five
**ICC-published** profiles that violate nothing. Its own text offered two
hypotheses (*"either iccce over-reports or a published ICC profile is
defective"*) and **the answer was a third it did not name: the row was
counting the wrong quantity.** When a conformance row goes red, check what it
measures before adjudicating between the hypotheses it hands you.

**How to apply:**

- Run it. It is minutes, and it is the one claim in this project that cannot
  safely be inherited.
- When a row fails, **the first question is what quantity it grades**, not
  whether the engine or the corpus is at fault. `CLAUDE.md` rule 5 forbids
  widening a bound — but **repointing a row to the correct subject is not
  widening**, and that distinction is worth stating explicitly in the commit
  so nobody later reads it as a bent rule. (The `passh/B` repoint kept its
  bound at `0` before and after; only the subject moved.)
- When you repoint, **keep the old quantity as a `REPORTED` (ungraded) row.**
  A fact that stops being a failure has not stopped being true, and letting
  the fix delete it is how a finding vanishes into its own remedy.

Related: [[project-handoff-carries-ref-hashes-not-a-push-count]] — the same
disease applied to publication state rather than test state.
