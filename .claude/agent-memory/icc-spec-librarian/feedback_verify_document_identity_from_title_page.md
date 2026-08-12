---
name: verify-a-documents-identity-from-its-title-page
description: Never inherit a document's identity from the filename or from the dispatch that asked for it — read the cover, the running head and the bibliography first. A file named BlackPointCompensation.pdf was presumed to be ICC WP40 and was actually ISO/CD 18619, which changed its citable tier, its licence terms and a gaps-table row.
metadata:
  type: feedback
---

**Rule: the first thing done to a newly-arrived source document is establish
what it actually is — from its own cover page, running head, document-stage
metadata and bibliography — before any clause is transcribed from it.** Record
the identity evidence in the file's frontmatter, not just the filename it
arrived under.

**Why.** On 2026-08-12 the dispatch said `_sources\BlackPointCompensation.pdf`
was *"presumed ICC WP40"* and asked for the WP40-vs-Adobe delta. **It is not
WP40.** Its cover reads *"the final approved ICC version of ISO 18619 … as
prepared by the ICC and TC130 in WG7"*; it is `ISO TC 130/WG7 N 063`, dated
`2013-05-2`, **document stage (30), COMMITTEE DRAFT** — and **WP40 appears in
its own Bibliography as reference `[1]`, a separate 2010-07-27 document.**

Four things depended on getting that right, and every one of them would have
been wrong:

1. **The citable tier.** WP40 is an ICC white paper. ISO/CD 18619 is a
   standards-committee draft with `shall` language — a *different* evidence
   tier (`published_draft_standard`), and a **higher** one.
2. **The citation form, which the document itself constrains.** Its warning
   page says *"may not be referred to as an International Standard."* Citing it
   as "ISO 18619" would have been wrong in a way that reads perfectly
   plausibly — the worst kind.
3. **The licence terms.** ICC terms and ISO terms differ, and the corpus
   records per-document provenance. A wrong identity puts a wrong licence line
   in `LEGAL.md`.
4. **A gaps-table row and a wanted-download list.** WP40 turned out to be
   **superseded** by the document in hand, so it dropped from "wanted" to
   archaeology — while the genuinely-still-wanted item (the *published* ISO
   18619:2015) had not been on the list at that priority at all.

**How to apply.**

- **Read page 1, the running head and the Bibliography before clause 1.** The
  bibliography is the cheapest identity check there is: **a document that cites
  X is not X.** That single observation settled this case.
- **Treat a filename as a hint and the dispatch's description as a hypothesis.**
  Neither is evidence. The operator downloaded a file from a link labelled one
  way; ICC served a different document under that name.
- **Capture document-stage metadata verbatim** (`ISO/CD`, `stage (30)`,
  `Committee Draft`, `N 063`, the date) into `standard:` and `clause:`. A draft
  and a published standard are not interchangeable and the difference is often
  a single word on a cover page.
- **When the identity differs from the dispatch's premise, say so first and
  loudly**, before the findings — it changes what the findings mean.

Related: [[label-the-predicate-not-just-the-payload]] (same shape one level up:
the *label* on a thing is a separate claim from the thing),
[[corpus-defects-are-caught-from-outside]], [[icc-bpc-sourcing-state]]
