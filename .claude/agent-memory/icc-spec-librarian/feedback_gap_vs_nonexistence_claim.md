---
name: icc-corpus-gap-vs-nonexistence-claim
description: C9 — never write "no document states X"; a corpus can only report the boundary of its own search, and the refutation was eight rows away in the same table
metadata:
  type: feedback
---

**Never write "NO document states X", "nobody publishes this", or "this is a gap
in the literature, not in the search." Write "not found in the N sources held,
and here is source N+1 that was not reached, and why."**

**Why:** corpus defect **`C9`**, 2026-08-17. `iec__s__srgb.md` carried
*"NO document states them"* about sRGB's D50-adapted colorants, escalated over
passes from a true statement (*"the four documents this corpus holds do not
state them"*) to a starred claim about the world (*"this is a GAP in the
literature, not in the search"*). **ICC had published them since 2015.** The
claim propagated into `index.md` as a `★★★` section heading, into
`icc__ref__ground_truth_availability.md`, `_sources/README.md` and
`LEGAL_NOTE.md`.

**★ The refutation was eight rows away in the SAME status table.** One row said
*"NO document publishes them"*; another said *"ICC registry `srgb.pdf` — NOT
FETCHED … ★ the cheapest open item"*, and two sections below,
*"plausibly the only published home of the D50-adapted colorants."*
**A corpus that says "the answer is probably in the document we have not opened"
cannot also say "no document has the answer."** No external procedure was
needed — only reading two rows together. That makes C9 unusual in the family
(see [[corpus-defects-are-caught-from-outside]]): most need an outside
consumer, this one needed subtraction.

**How to apply:**
1. **Mechanical check, cheap, runs inside one file:** *if a file contains both a
   NOT-FOUND claim and a NOT-FETCHED row, the NOT-FOUND claim is provisional by
   construction.* **Grep for the pair** whenever writing or reviewing a gap
   statement.
2. **C9 is a NEW defect class: every stated fact was true and the error was
   entirely in the quantifier.** None of the corpus's existing guards look at
   quantifiers — they check tiers, labels, derivations and staleness.
   **When editing a summary line, check the scope word, not just the payload.**
3. **The bar does NOT loosen the sourcing discipline.** The `color.org` agent
   bar held throughout; the corpus never routed around it and the document
   arrived by exactly the operator route it had specified. **A recorded gap beat
   a confident guess. The failure was in DESCRIBING the gap, not in respecting
   it** — so the correction is wording, never "try harder to fetch".
4. Applied already to `icc__ref__ground_truth_availability.md` §7's EXISTENCE
   cell, which said *"Nobody publishes these"* about four other routes and now
   says *"Not found in any source held."*

Full write-up: `D:\Dev\Rag-Specialized\ICC_Spec\icc\icc__ref__spec_defects.md` §26.
