---
name: icc-pdf-symbol-font-sign-loss
description: ICC.1-2022-05.pdf sets +/-/x/<=/>= in the Symbol font, so every text extractor silently drops minus signs — map the U+F0xx private-use range before reading any number out of it
metadata:
  type: reference
---

**`D:\Dev\Rag-Specialized\ICC_Spec\_sources\ICC.1-2022-05.pdf` sets mathematical signs in the Adobe Symbol font, which extracts into the Unicode private-use area.** All three extractors tested — `pypdf` 6.7.0, poppler `pdftotext -layout`, `pdfminer.six` — **drop them silently rather than erroring.**

**The damage is invisible and severe:** `−128,0` (Table 43, legacy PCSLAB a\*/b\*) extracts as `128,0`; `1,0 + (32 767/32 768)` extracts as `1,0  (32 767/32 768)`; and **the Bradford matrix in Annex E.3 extracts all-positive**, which is a plausible-looking, catastrophically wrong matrix — exactly the failure mode the ICC_Spec corpus exists to prevent.

**Mapping (Symbol charcode + 0xF000):**

```python
m = {0xf03d:'=', 0xf02b:'+', 0xf02d:'\u2212', 0xf0b4:'\u00d7', 0xf0be:'\u2192',
     0xf0de:'\u21d2', 0xf067:'\u03b3', 0xf03e:'>', 0xf03c:'<', 0xf072:'\u03c1',
     0xf062:'\u03b2', 0xf0a5:'\u221e', 0xf0b3:'\u2265', 0xf0a3:'\u2264',
     0xf0b7:'\u00b7', 0xf0ce:'\u2208', 0xf028:'(', 0xf029:')'}
# 0xf0e6-0xf0fe are multi-line bracket/brace piece glyphs from matrix layouts - drop them.
```

**Working recipe, for re-extraction:** `pypdf` for a whole-document grep corpus (fast, page-tagged); **poppler `pdftotext -layout`** for tables (best column reconstruction, but it scrambles equation cells); **`pdfminer.six`** for equations and anything glyph-critical — it is the only one that preserved `parametricCurveType` Table 68 in recoverable order. `pdftotext` lives at `/mingw64/bin/pdftotext` in the Git-Bash environment.

**Tool limitation worth not re-discovering:** the **Read tool cannot render PDF pages** here — it needs `pdftoppm`/poppler-utils, which is not installed for it, and fails with "pdftoppm is not installed". So visual page reading is unavailable and **cross-verification must be two *text* engines, not text-plus-vision.** Say so in the file when a passage is load-bearing.

**This hazard is also recorded in the corpus itself** (`LEGAL_NOTE.md` §1b, `LEGAL.md` §2.4, and inline warnings in `icc__ref__v2_v4_divergence.md` D1, `icc__s__number_encodings.md`, `cie__ref__chromatic_adaptation.md`) because a future session may extract from the PDF without reading memory first.

Related: [[icc-tos-automated-access-blocker]], [[icc-spec-corpus-sourcing-route]]
