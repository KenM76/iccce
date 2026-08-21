---
name: icc-corpus-gap-vs-nonexistence-claim
description: C9 then C10 — never write "no document states X"; a corpus reports the boundary of its own search. C10 is the sharper twin: a source is never "checked", it is checked FOR SOMETHING, and a blocker may not be filed under EXISTENCE without a positive reason
metadata:
  type: feedback
---

**Never write "NO document states X", "nobody publishes this", or "this is a gap
in the literature, not in the search." Write "not found in the N sources
searched FOR <predicate>, and here is source N+1 that was not reached, and
why."**

**Why:** two corpus defects, one week apart, same shape, second one despite the
first being written down.

## `C9`, 2026-08-17 — the refutation was eight rows away in the same table

`iec__s__srgb.md` carried *"NO document states them"* about sRGB's D50-adapted
colorants, escalated over passes from a true statement (*"the four documents
this corpus holds do not state them"*) to a claim about the world. **ICC had
published them since 2015.** The refutation sat in the same status table: one
row said *"NO document publishes them"*, another said *"ICC registry `srgb.pdf`
— NOT FETCHED … the cheapest open item"*. **A corpus that says "the answer is
probably in the document we have not opened" cannot also say "no document has
the answer."**

## ★★★ `C10`, 2026-08-18 — worse, because the document HAD been opened

*"There is still no worked sRGB input→output triple, in this document or any
other"* / *"none, anywhere"* — **carried across nine filings, and false.**
**W3C CSS Color 4 publishes four**, and the corpus had **held, read,
transcribed and given its own file to** that document one day earlier
(`w3c__s__css_color_4.md`).

**C9 was a search that never reached the document. C10 is a document that was
mined for the wrong thing.** The file was searched for *constants*; the corpus
wanted *results*; and *"five sources checked"* was carried forward pass to pass
as though **"checked" were a property of the source rather than of a question.**

**⟹ C9's rule was necessary and insufficient.**

## How to apply — four checks, all mechanical

1. **An inventory line names its PREDICATE.** *"n sources searched for the
   transfer-function constants"*, never *"n sources checked"*. A source checked
   for constants has not been checked for examples. This is
   [[label-the-predicate-not-just-the-payload]] arriving in an **inventory**
   rather than in a value.
2. **★★★ A blocker may NOT be filed under EXISTENCE without a POSITIVE reason on
   record** — a structural argument (LUT results: ICC mandates no interpolation
   method, so no value *could* be published) or a publisher's own statement
   (ECI's info document makes no accuracy claim). **"We looked and did not find
   it" is an AVAILABILITY or ACCESS-TERMS finding, never an EXISTENCE one.**
   This is the expensive step: an availability blocker invites a retry, an
   **existence blocker forecloses it**, and `iccce` carried "no
   published-ground-truth transform row" for nine filings partly on a sentence
   promoted two categories beyond its evidence.
3. **If a file contains both a NOT-FOUND claim and a NOT-FETCHED row, the
   NOT-FOUND claim is provisional by construction.** Grep for the pair.
4. **★ A NEGATIVE FROM `grep` IS A NEGATIVE ABOUT ONE LINE, NOT ABOUT THE
   DOCUMENT.** Text dumps of standards wrap mid-sentence, so a phrase search
   for a sentence that exists returns nothing. **Twice on 2026-08-21** a
   cross-edition divergence was within one keystroke of being filed on exactly
   this: *"the computation shall be the same whether the colour space is
   additive or subtractive"* was declared a PDF 2.0 addition because the 2008
   dump broke it as `additive or` / `subtractive.`; and the §11.7.3
   conversion-timing sentence was declared 2.0-only for the same reason. **Both
   were present in 2008.** ⟹ before filing ANY "absent from edition X", search
   a **short distinctive fragment** (3–5 words, no line-break risk), and search
   **two engines' dumps** — `pypdf` and `poppler` wrap at different points, so
   agreement between them on a negative is worth far more than either alone.
5. **When a held source is re-read for a NEW question, record that in the
   source's own file.** `w3c__s__css_color_4.md` said nothing about examples
   either way, so nothing disclosed that the question had never been asked. A
   banner naming *which predicates this file has been searched under* is now on
   that file.

**Both defects were caught from OUTSIDE the file** ([[corpus-defects-are-caught-from-outside]],
n = 12). **C10 was caught by a dispatch asking a question the corpus's own
summary said was already answered** — the strongest argument on record for
treating an external question as a test rather than an interruption.

**Neither loosens the sourcing discipline.** The `color.org` bar held
throughout both. **The failure was in DESCRIBING the gap, not in respecting
it** — the correction is wording, never "try harder to fetch".

Full write-ups: `D:\Dev\Rag-Specialized\ICC_Spec\icc\icc__ref__spec_defects.md`
§26 (`C9`) and §28 (`C10`).
