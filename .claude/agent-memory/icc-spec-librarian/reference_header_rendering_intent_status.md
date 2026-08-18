---
name: header-rendering-intent-status
description: The header renderingIntent field, both halves — WHO MAY CONSUME it (A55: advisory for six classes, normative only for `link`; 8.10.2 presupposes an intent it never designates; the only "default" wording is an informative NOTE saying perceptual) and WHAT IT MAY CONTAIN (A56: the value set is NOT closed in either edition, so "outside the defined 0..=3" is a FALSE statement about v2). The v2/v4 gate is FOUR differences, not one, and one of them runs the other way
metadata:
  type: reference
---

**Settled 2026-08-18 from ICC.1:2022 + ICC.1:2001-04, three extraction channels (the third is a `pypdfium2` page raster). Do not re-derive. Cite `A55` for *who consumes* the field; cite `A56` for *what it may contain*.**

Corpus: `D:\Dev\Rag-Specialized\ICC_Spec\icc\icc__s__rendering_intents.md` **§1b**
(full verbatim), `icc__ref__ambiguity_register.md` **A55**,
`icc__ref__v2_v4_divergence.md` "the header rendering-intent field — v2 vs v4"
(heading renamed 2026-08-18; the old "checked non-divergences" title is stale),
`icc__s__v2_ICC1_2001_04.md` **§1c/§1c.2**, `icc__s__header.md` (both version
gates), `icc__ref__ambiguity_register.md` **A56**.

## The four facts that answer every future version of this question

1. **7.2.15's `shall` is WRITER-SIDE.** *"The rendering intent field **shall
   specify** the rendering intent **which should be used** (or, in the case of a
   DeviceLink profile, **was used**)…"* — the `shall` binds the **field**; the
   content it mandates recording is a `should`. **A `shall` to record a `should`
   is not a `shall` to obey.** Same clause calls it *"this flag"* that *"may not
   have any meaning"*.
2. **8.10.2 never mentions the field.** Its lead-in is *"the precedence order of
   the tag usage **for a designated rendering intent** shall be the following"*.
   The intent is an **input**; no clause designates it. **No cross-reference
   between 8.10.2 and 7.2.15 in either direction.** → SILENCE, `A55`.
3. **Exactly THREE clauses in all of ICC.1:2022 reference the field:** 7.2.15
   (defines), **7.2.18** (zeroed before the MD5 profile ID — two profiles
   differing only here have the **same profile ID**; corroborating *inference*,
   not spec text), **8.6** (DeviceLink).
4. **`link` is normative — and still selects nothing.** 8.6: *"The rendering
   intent used is indicated in the header of the profile."* But **8.10.3 omits
   the phrase "for a designated rendering intent"** because a link has one
   transform. **Consuming vs ignoring produce IDENTICAL PIXELS; they differ only
   in whether a mismatch with the caller's request is DISCLOSED.** Over-reading
   this as "the CMM shall select by the header intent for links" is the trap.

## The default question — the answer is not what either engine does

ICC.1's **only** occurrence of the words "default rendering intent" is
**9.2.39 Table 27 NOTE 1**: *"Because the perceptual intent is the typical
default rendering intent…"* — **informative**, inside a gamut-signature tag
definition, about which reference gamut to build against. **Cannot be cited as a
requirement**, but it is the only textual support for any default and it favours
**0 (perceptual)** = lcms2's default, **not** iccce's `Intent::MediaRelative`.
Neither default is required or forbidden; **both must be disclosed**.

## v2 vs v4 — FOUR real differences (was "one"; amended 2026-08-18 by the `A56` dispatch)

Same *rule* in both editions; 8.6 and v2 6.3.4.1 share their key sentence word
for word. The divergence table in `icc__ref__v2_v4_divergence.md` is now
**11 rows, 4 real changes** — its heading was *"CHECKED NON-DIVERGENCES"* and
the blanket "it did not change" is **retracted for the low half**:

1. **High 16 bits** — v4 7.2.15 adds "most significant 16 bits shall be set to
   zero"; v2 6.1.11 has no requirement. High-bits report is **v4-only** (`A7`).
   **Stronger than "un-forbidden":** v2's sentence *"The least-significant 16
   bits are reserved for the ICC"* is **the same boilerplate v2 6.1.8 / v4
   7.2.11 use for the profile FLAGS field**, where the high half is
   demonstrably vendor space and is never zeroed. So in v2 the high half reads
   as **affirmatively vendor-available**.
2. **★ LOW 16 bits — the value set is NOT CLOSED in either edition (`A56`).**
   **v2 6.1.11 contains no `shall` at all**; its one normative-sounding
   sentence — *"…are the four intents **required to be supported**"* — binds
   **CMM support**, not field content, and Table 18 (2 cols × 4 rows, no
   "other values" row) is introduced by the descriptive *"The encoding is such
   that:"*. **v4 7.2.15 adds a DIRECTIONAL `shall`** — *"These shall be
   identified using the values shown in Table 23"* — binding intent→value
   only. **Consequence: "outside the defined 0..=3" is a FALSE statement about
   v2**; the honest v2 report is *"unrecognised value"*. **Both editions use
   "other values are reserved for future use" elsewhere (v2 Table 38 / v4
   Table 36) and neither uses it here.**
3. **Encoded type** — v2 **never states it**: Table 9's `64..67` cell reads
   *"see below"* and 6.1.11 names no type. Only v4 says *"The field is a
   uInt32Number"*.
4. **★ Running the OTHER way — v4 DELETED text.** **v2 6.3.3.2 (Color Output
   Profiles), normative:** *"The intent values described in these tags
   **directly correlate to the value of the rendering intent header flag of the
   source profile in the color modeling session** (See Table 18)."* Occurs
   **once in all of v2**, has **no v4 counterpart** (v4 moved those tables to
   informative Annex G). Descriptive, not a `shall`, and it points at the
   **value** table — so read it as a numbering correspondence. **It does not
   close `A55`**, but `A55`'s line *"v4 looks more specified than v2 and is
   exactly as unspecified"* is **withdrawn as a claim about the whole field**.

**No "unspecified"/absent sentinel in either edition.** "unspecified" occurs
**0×** in ICC.1:2001-04 and **2×** in ICC.1:2022 (both 9.2.x, reference-gamut
tags). And where v4 wants a null it says so — 7.2.12/7.2.13/7.2.17 all carry
*"If not used this field shall be set to zero"*; **7.2.15 does not**.
**`0` means PERCEPTUAL, never "not stated"** — do not parse a zero as absent.

**Still true:** v2 6.3 admitted *"the general fall back strategy of the CMM is
implementation dependent"* in words; v4 replaced it with 8.10.2's `shall`
**whose intent input it still never sources**.

## Coverage gap this left open

**No `link`-class fixture exists** — not in this corpus, not in iccce. The
DeviceLink verdict is **entirely from the text** and does not need one, but
*iccce's handling of a DeviceLink is untested*. That is an untested class, not a
wrong default. Related: [[icc-conformance-clause-binds-only-reading]] — the
right verdict word is "diverges", never "non-conforming".
