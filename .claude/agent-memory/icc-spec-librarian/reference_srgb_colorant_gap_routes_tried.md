---
name: srgb-colorant-gap-routes-tried
description: The sRGB D50-colorant gap is CLOSED — ICC srgb.pdf (2015) §B.2 publishes the colorants AND the recommended D65→D50 chad; the ~12 ULP blue-Z residual turned out to be the HP FILE's error, not a reconstruction failure; also where the operator's private ICC fixture tree lives
metadata:
  type: reference
---

**★★★ CLOSED 2026-08-17 evening. `_sources/srgb_bt709/srgb_icc_specification_of_srgb_2015.pdf`
(`sha256 ceed855c…8ddf9f1f`, 4 pp) publishes BOTH.** Operator browser download;
the `color.org` bar was never touched. Corpus home:
`D:\Dev\Rag-Specialized\ICC_Spec\icc\icc__s__srgb_for_icc_profiles.md`.

**Colorants (§B.2, cols = rXYZ/gXYZ/bXYZ):**
`0.436030342570117 0.385101860087134 0.143067806654203 / 0.222438466210245
0.716942745571917 0.060618777416563 / 0.013897440074263 0.097076381494207
0.713926257896652`. **This is the document ICC.1:2022 E.4.2 cites for the `chad`.**

**★★★ THE REVERSAL, and it is the reusable part.** The corpus had concluded
*"the HP 1998 colorants cannot be reproduced … `bXYZ.Z` provenance unknown"* —
**correct arithmetic, wrong implication.** Measured vs ICC's own published
values: **iccce from-constants = 3,02 ULP worst / 0,90 in `bXYZ.Z`;
the shipped HP/`sRGB2014` file = 11,13 ULP, all in `bXYZ.Z`.** **The file does
not match ICC's own spec either.** Question reposed, not resolved.

**Provenance chain, recovered here in exact `Fraction` (NOT in any dispatch):**
W3C-1996 **4-dp** matrix → invert, round 7 dp = **§A.7** → `chad` = linear
Bradford from **`0,9505/1/1,0890`** (the ROUNDED white, stated in §A.4 as
`76,04/80/87,12`) to `0,9642/1/0,8249` → §B.2 = `chad × inv(§A.7)`, **0,00 ULP**.

**★★★ `chad`'s cone matrix recovered by EIGENDECOMPOSITION** (its left
eigenvectors ARE the `M_A` rows): **`M_A[0][0] = 0,8950`, where ICC.1:2022
Annex E.3 Eq. (E.1) prints `0,8951`.** `0,8951` → `0,371` ULP; `0,8950` →
`5,7×10⁻¹⁶`. **Two ICC documents, two Bradford matrices.** Below one encoding
step so no colour changes — but "recompute E.3 and you get the recommended
`chad`" is FALSE. **The discriminating digit was already in the corpus**
(E.1's row 0 sums to `1,0001`, the recovered one to `1,0000`) recorded as
reassurance.

**Two DEFECTS in ICC's own §B.1, three engines agreeing:** `BL = B/12.02` (for
`12.92`), and **all three power-branch lines read `R`** — the *identical* defect
as W3C 1996 eq. (1.7b), nineteen years apart. **`pdftotext -layout` is
DISQUALIFIED for matrices in this document class** — it dropped all three `chad`
minus signs and transposed a cell. Use `pypdf` + `pdfminer.six` char coords.

**Still open, as amended 2026-08-18:** `0,04045` from IEC's own text (ICC
restates it and misprints the restatement, so it does NOT promote) — **and the
purchase target is now exact: clause 5.2/5.3, pp. 21-25 of IEC publication
`6169`, CHF 210**, because IEC's free 15-page preview is held and contains
**zero constants**; ~~no worked sRGB triple anywhere~~ **★★★ RETRACTED (`C10`) —
CSS Color 4 publishes FOUR**; `srgb.xalter` / `registry.color.org` unfetched
(expectation LOW). **NEW: `COR1:2014` exists and is unobtained.**
→ [[iec-srgb-primary-sourcing-state]]

**Where the binaries are:** `D:\Dev\iccce-private-fixtures\color-org\` — the
operator's private ICC profile fixture tree. `sRGB2014.icc` colorants are
byte-identical to HP 1998's (that measurement stands) — **and are NOT ICC's own
published numbers**, which is sharper than the old note.

Related: [[icc-corpus-gap-vs-nonexistence-claim]], [[published-ground-truth-state]],
[[icc-tos-blocks-automated-access]], [[icc-pdf-symbol-font-sign-loss]].
