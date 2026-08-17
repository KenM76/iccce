---
name: disclosure-caught-a-bad-justification
description: DL-049 — a candidate-separation field that gates nothing caught a defect in a TOLERANCE'S JUSTIFICATION on a row that was passing; when a separation looks manufactured, re-derive the `why` string before touching the separation.
metadata:
  type: project
---

**When a candidate separation reads as manufactured, absent, or
implausibly small, re-derive the TOLERANCE'S JUSTIFICATION before
touching the separation.** The separation is the symptom; the derivation
is where the defect lives. **A green row is no evidence that its `why`
string is sound.**

**Why:** Pass G, 2026-08-17. `BLIND` fired on an authoring row — for a
profile whose `wtpt` **agrees** with its own colorant sum there is no
rival reading, so the stated separation (`5.4e-6` against a `2e-4` bound)
was a **manufactured alternative**. Fixing that exposed the real fault,
which nothing could see until then: **the `2e-4` `s15Fixed16`
encoding-floor justification did not hold for the profiles it was
gating.** Ghent's sRGB colorants sum to the PCS white only to `1.885e-4`
— ≈**12 lsb** — because the *published* sRGB primaries do not sum to D50
to the encoding lsb. **The row had been PASSING inside a bound its own
justification could not support**, which is
[[iccce-agreement-can-be-the-symptom]]'s shape one level up.

**This EXTENDS [[iccce-disclosure-is-not-enforcement]] rather than
repeating it.** DL-037 is about the guard **order** and about `BLIND`
deliberately not gating. DL-049 is about **what the disclosure is
evidence OF**: DL-037 predicted the field would flag rows whose
*comparison* has unknown power; it did **not** predict it would flag rows
whose *tolerance* is unsupported — a different document, a different
agent's responsibility, and previously findable only by a person
re-reading a `why` string.

**The remedy shape (DL-039):** replace the unsupportable bound with a
question that has **no free parameter**. Here: *is the colorant sum
nearer the normative PCS white or nearer the profile's own encoded
`wtpt`?*, bounded at **half the distance to the profile's own rival
candidate**. A classification cannot be tuned. ★ **And it imported no
third white point** — reaching for **D65** would have put the oracle's
own single-sourced constant (§3.5 / NC-018, *"the weakest constant in
`iccce-color`"*) underneath a finding about third-party authorship, and
the finding would have looked identical.

**How to apply:** when filing a Pass, read the separation aggregate's
`BLIND` and `NO-NAMED-ALTERNATIVE` rows as **pointers to suspect
derivations**, not just as coverage bookkeeping — and check whether the
sibling fix was made by **finding a missing term** (DL-043,
[[iccce-measurement-under-the-guards]]) or by moving a number. One
instance only: the rule is *look there first*, not *assume*. Related:
[[iccce-count-needs-its-apparatus]] (`ungraded=8` did not move although
12 rows were taken out of grading — still unsettled).
