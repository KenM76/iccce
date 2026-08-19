---
name: malformation-modality-settled
description: The nine iccce Malformation variants have a SETTLED per-edition modality (Violation / NotAViolation / Unsourced) at icc__s__tag_table.md §7 — do not re-derive; three split by edition (header reserved bytes, rendering-intent value, tag overrun), two are Unsourced (A61/A62), and ★ v2 requires with "must" (76×) not `shall` (27×, mostly elsewhere)
metadata:
  type: reference
---

**Settled 2026-08-19 from ICC.1:2022 + ICC.1:2001-04, two text engines plus a
`pypdfium2` page raster on the three v2 sentences the verdicts turn on. Do not
re-derive. The table is `D:\Dev\Rag-Specialized\ICC_Spec\icc\icc__s__tag_table.md`
§7**, with per-edition clause and quoted operative words.

## The four facts that answer every future version of this question

1. **★★★ v2 REQUIRES WITH "must", NOT WITH `shall` — counted: `shall` 27×
   (3 in the copyright notice; substantively only clause 3, clause 6's
   structure sentence, Annex A, and "usage shall be as defined in Table 20")
   vs `must` 76×, and EVERY tag-table / header-field / tag-type requirement
   is in the second group.** Its change list item 14 concedes it *"does not
   meet all of the ISO/IEC drafting rules"*. **A modality predicate that greps
   for `shall` reads v2 as silent where it is emphatic.**
   Symmetrically, v2's *unmodalised* field definitions really are silent,
   because the drafters used "must" in the adjacent sentence — that
   parallel-construction argument is what makes `HeaderReservedNonZero` a
   confident `NotAViolation` in v2 rather than an `Unsourced`.
2. **Three variants SPLIT by edition** — and each split can make iccce accuse a
   conforming file: `HeaderReservedNonZero` (v4 7.2.19 `shall be set to zero`;
   **v2 Table 9 states no requirement at all**, and its block is **84..127**,
   not 100–127); `UnknownRenderingIntent` (`A56`); `TagOverrun` (v4 7.2.2
   derived; **v2 6.1.1 is one unmodalised sentence** → `A62`).
3. **Two `Unsourced` verdicts, both new rows.** **`A61`** — *no minimum tag
   data element size in either edition*, and each has a sentence pointing the
   other way (v4 10.1 vs **7.4** *"shall only be restricted by"*; v2 6.5 vs
   **6.2.3** *"An element may have any size"*). **`A62`** — v2's profile size
   field carries no modal verb. **Both prevent a false accusation rather than
   enabling a true one; that is why they are worth rows.**
4. **`DuplicateTagSignature` is a violation in BOTH editions** — v4 7.3.1
   `shall not`, **v2 6.2 *"must be unique; a profile cannot contain more than
   one tag with the same signature"***, dated by v2's change list item 5 to a
   **1998-03-15 resolution**. The *"Legality NOT SOURCED"* label was wrong
   (`C14`); what remains unsourced is only **which duplicate wins**.

## The trap that produced the defect, and it will recur

**`icc__s__tag_table.md`'s "Validation checks iccce should perform and REPORT"
table was a DERIVED CONVENIENCE — a list of checks worth *performing*, with no
clause per row — and two consumers read it as a list of conformance rules.** It
now carries a banner saying so. **A derived validation table is not a modality
source.** Same shape anywhere the corpus lists "checks", "rules of thumb" or
"what an implementation should do": if a row has no clause, it cannot support
the word *violation*.

**And the version-gate half:** clause 7.3.1 states four requirements, the
Foreword names **one** as new, and the corpus wrote *"these are new … so v2
profiles may legitimately violate all four"*. **A change list is evidence about
the requirement it quotes and nothing else.** Full write-up:
`icc__ref__spec_defects.md` §32 (`C14`).

## Practical notes for the next dispatch of this kind

- **`TrailingBytes` is `NotAViolation` for a POSITIVE reason**, not silence:
  both editions contemplate embedded profiles in the same sentence (v4 7.3.4
  NOTE 1 = v2 6.2.2). Say the reason; a bare "not forbidden" invites a re-check.
- **Verdict wording:** "this file breaches 7.2.19", never "this file is
  non-conforming" — [[icc-conformance-clause-binds-only-reading]].
- **v2 has NO padding, contiguity or overlap requirement at all** (`pad*` only
  in 6.2.3's "must not include any padding", which is about the size field;
  `overlap`/`contiguous`/`gap` = 0 hits). `A12` is **v4-only**.
- Divergence row: `icc__ref__v2_v4_divergence.md` **D13**. v2 clause text:
  `icc__s__v2_ICC1_2001_04.md` **§3b**.

Related: [[header-rendering-intent-status]] (`A56`, the v4 leg is a **two-step
derivation**, so citing "7.2.15's `shall`" alone overstates it),
[[corpus-defects-are-caught-from-outside]], [[a-retraction-is-a-grep-not-a-paragraph]].
