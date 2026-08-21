---
name: pdf-spec-cross-corpus-state
description: The sibling PDF_Spec RAG HOLDS both ISO 32000-1:2008 (free) and ISO 32000-2:2020 primary PDFs — 32000-2 is licensed_primary_private_rag, its errata are unapplied ANNOTATIONS, pypdf emits INTRA-WORD SPACES so whitespace-normalised fragment search still needs two engines, and §11.7.2 (filed 2026-08-21) says sRGB — iccce's default — can be unsuitable as a transparency-group BLENDING space
metadata:
  type: reference
---

**`D:\Dev\Rag-Specialized\PDF_Spec\_sources\` holds both PDF primaries.** Go
there before concluding anything about PDF's colour/transparency model; the
digests under `iso32000\` are good but partial, and a digest's gap is not the
corpus's gap.

| File | Edition | Pages | `license_basis` |
|---|---|---|---|
| `PDF32000_2008.pdf` | ISO 32000-1:2008 | 756 | **`free_primary`** — quote freely |
| `ISO_32000-2_sponsored_EC3.pdf` | ISO 32000-2:2020, Errata Collection 3 (2026-06-01) | 1023 | **`licensed_primary_private_rag`** |

## ★ The licence, and it binds this corpus too

Footer on **every page**: *"Sold by the PDF Association to Ken Mantle 20699 |
… | Single user only, copying and networking prohibited."* Free of charge, to a
**named individual**, under ISO copyright. The rules in
`D:\Dev\Rag-Specialized\PDF_Spec\LEGAL_NOTE.md` (2026-08-12 update) are:

- **Short quotation with citation: yes, sparingly. Multi-paragraph verbatim: NO — hard.**
- Clause numbers, table numbers, parameter names, **modal verbs** and constants
  are **facts** and are freely usable.
- The file **must never leave `_sources\`** — no `R:\` (a Dropbox subst alias
  = "networking"), no repo, no release asset.
- **ICC_Spec and iccce are MIT and public; that changes nothing.** MIT is our
  licence on our text, not a licence on ISO's.

⟹ When a dispatch asks for ISO 32000-2 text, deliver **sentence-level quotes +
clause + modal verb**, and paraphrase the rest. Prefer the **2008** wording when
the two editions agree — that one is `free_primary` and quotable at length.

## ★★ The errata are ANNOTATIONS, not applied text

`extract_text()` returns ISO 32000-2:2020 **as printed** — every correction
silently omitted. A naive dump **is the uncorrected standard**.

**Before quoting any 32000-2 passage, scan `/Annots` on its page** for
`/StrikeOut`, `/Caret`, `/Text` with `/T` = `Issue #NNN`. Filter out `/Link` and
`/Popup` or the output drowns. Review `/State` distinguishes authority:
**`Completed`** = ISO TC 171 SC 2 WG 8; **`Accepted`** = PDF Association TWG
only. Record which.

**Second, independent channel, freely fetchable:**
`https://pdf-issues.pdfa.org/32000-2-2020/clause<NN>.html` (zero-padded). Used
2026-08-21 and it agreed with the annotation scan exactly. **`pdfa.org` itself
403s automated fetches; `pdf-issues.pdfa.org` and `pdfa-inc.org` do not** — the
same shape of finding as [[icc-tos-automated-access-blocker]], and note that
the bar on `color.org` does **not** generalise to these hosts.

## Extraction, measured on these two files 2026-08-21

`pypdf` **beats** `pdftotext -layout` here: it recovers the Unicode
Mathematical Italic run (`𝐵(𝐶𝑏, 𝐶𝑠)`, `𝛼𝑟`) that poppler drops to nothing —
the same class of loss as [[icc-pdf-symbol-font-sign-loss]] but with the
opposite winner, so **do not assume the ICC-side engine ranking transfers.**
Poppler in exchange glues marginal heading numbers into body text
(`11.7.3  for a transparency group.`) — an artifact, not spec text. Run both;
disagreement localises the artifact.

Recipe: per-page `=== PDFPAGE n ===` markers via `pypdf`, plus a
`pdftotext -layout` twin. See [[dont-transcribe-numeric-tables]] — dump into a
scratchpad and point at it, do not transcribe.

## ★ pypdf's SECOND failure mode: an INTRA-WORD SPACE (2026-08-21)

Beyond line wrapping, **pypdf emits `furthe r`, `s hou ld`, `loss ⏎ of`** in the
2008 file. ⇒ **whitespace normalisation is NOT enough**; a normalised fragment
search still returns a false 0. On §11.7.2 alone this would have produced **two
false negatives out of five fragments**, and poppler caught both. **Never file a
cross-edition negative from one engine.** ([[icc-corpus-gap-vs-nonexistence-claim]] rule 4.)

## §11.7.2 — what the group blending space says (filed 2026-08-21)

`iso32000__s__11.7.2.md`, both editions. **It had NO digest before that date**
(honestly disclosed in five files, so this was a *known* gap, not a hidden one —
the overstatement lived in the **filename** `iso32000__s__11.7.md`, which covers
only §11.7.3/.4).

- **`GCS-2`** — objects **shall** be converted to the group space and **all
  blending and compositing computations shall be done in that space**. Every
  blend formula in the corpus is a formula *in the group space*.
- **★★ `GCS-15`** — NOTE 4, **both editions**: **sRGB is nonlinear and "can be
  unsuitable for use as a group colour space."** That is **iccce's default
  destination**. Destination use fine; **blending** use is the flagged case.
- **`GCS-14`** — NOTE 3, the linearity acknowledgement. **The nearest thing in
  ISO 32000 to a non-commutativity statement, and it is NOT one** — do not quote
  it as one.
- **★★ `GCS-N1`** — 2008 NOTE 1's convert-once-at-the-end **rationale is DELETED
  in 2.0**; verified 1→0 on 5 fragments × 2 engines. With `GCS-D4`/`D5`/`D6`:
  **four deletions, all removing warnings about ad-hoc conversion, rules intact.**
- **★ `GCS-D3`** — "the group's colour space **should** be CIE-based" is **body
  text in 2.0** and **inside NOTE 3 in 1.7**: normative one edition, informative
  the other. **Modal verb alone is not enough; check whether it is in a NOTE.**
- **`GCS-20`** (2.0 only, §11.4.7) — page-group→device **shall** use
  `RelativeColorimetric` (unbounded escape hatch) and **BPC is
  implementation-dependent by name** → `A41`.
- §11.7.3 also diverges: **`SP-D1` "one of two things `shall` happen" →
  "`should` happen"**, with both branches still `shall` — filed in
  `iso32000__s__11.7.md` §11.7.3-Δ.

**Errata channel agreed with the `/Annots` scan again** (zero on §11.7.2/§11.4.7).

## What PDF normatively hands ICC

**ISO 32000-2:2020 §10.3.1**, verbatim: *"Conversion from a CIE-based source
colour to a CIE-based destination colour shall be performed based on
ISO 15076-1:2010 (ICC.1:2010)."* That is the only `shall` binding a PDF
processor to ICC.1, and it names **ICC.1:2010 (v4.3)**, not 2022.
