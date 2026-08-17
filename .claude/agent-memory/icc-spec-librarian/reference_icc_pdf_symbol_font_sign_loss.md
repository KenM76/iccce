---
name: icc-pdf-symbol-font-sign-loss
description: ICC.1-2022-05.pdf sets +/-/x/<=/>= in the Symbol font, so every text extractor silently drops minus signs — map the U+F0xx private-use range (exhaustive map inside, corrected 4th pass) before reading any number out of it; also which engine to use per structure
metadata:
  type: reference
---

**`D:\Dev\Rag-Specialized\ICC_Spec\_sources\ICC.1-2022-05.pdf` sets mathematical signs in the Adobe Symbol font, which extracts into the Unicode private-use area.** All three extractors tested — `pypdf` 6.7.0, poppler `pdftotext -layout`, `pdfminer.six` — **drop them silently rather than erroring.**

**The damage is invisible and severe:** `−128,0` (Table 43, legacy PCSLAB a\*/b\*) extracts as `128,0`; `1,0 + (32 767/32 768)` extracts as `1,0  (32 767/32 768)`; and **the Bradford matrix in Annex E.3 extracts all-positive**, which is a plausible-looking, catastrophically wrong matrix — exactly the failure mode the ICC_Spec corpus exists to prevent.

**Mapping (Symbol charcode + 0xF000) \u2014 CORRECTED 2026-08-11, 4th pass. The
earlier version of this memory had `0xf0be` wrong; see the note below.**

```python
# Test the bracket band FIRST, then the table. Reversing this order lets a
# speculative entry (e.g. 0xf0f7) fire on a bracket piece.
def fix_char(o):
    if 0xf0e6 <= o <= 0xf0fe: return ''          # multi-line bracket/brace pieces - DROP
    return m.get(o)                              # None => unmapped, make it visible

m = {0xf03d:'=', 0xf02b:'+', 0xf02d:'\u2212', 0xf0b4:'\u00d7',
     0xf0be:'\u2022',   # LIST BULLET - NOT an arrow. 77 occurrences, every one
                        # heading a list item (Foreword changes, 3.1.12, clause 8.2)
     0xf0de:'\u21d2', 0xf067:'\u03b3', 0xf03e:'>', 0xf03c:'<', 0xf072:'\u03c1',
     0xf062:'\u03b2', 0xf0a5:'\u221e', 0xf0b3:'\u2265', 0xf0a3:'\u2264',
     0xf0b7:'\u00b7', 0xf0ce:'\u2208', 0xf028:'(', 0xf029:')',
     0xf0bc:'\u2026', 0xf0dc:'\u21d0'}   # ... and the erroneous <= glyph
```

**That map is now EXHAUSTIVE for this PDF \u2014 zero unmapped private-use
codepoints.** The complete set present: `F028 F029 F02B F02D F03C F03D
F03E F062 F067 F072 F0A5 F0B3 F0B4 F0B7 F0BC F0BE F0CE F0DC F0DE` plus the
bracket band. **Do not extend the map speculatively from the standard
Symbol encoding** \u2014 `0xf0f7` is *not* `\u00f7` here, it is a bracket piece
(29 occurrences), and mapping it injects a division sign into every
bracketed equation.

**Wide tables: `pdftotext -layout` detaches cells from their rows.**
Confirmed on **Table 25** (profile class \u00d7 tag \u00d7 intent) and **Table D.1**.
For any table whose meaning is the cell\u2194row pairing, resolve with
**pdfminer.six character x/y coordinates** \u2014 that is the only reliable
route, and it is what makes an otherwise unreadable table quotable.

**Equations are stacked fractions built from bracket pieces** \u2014 they have
**no linear text form**. Anything read out of one is a **RECONSTRUCTION
from geometry**, not a verbatim quote, and must be labelled so. Annex D
is the exception: it prints the same relationships on single lines, which
is why the corpus quotes `(D.6)`/`(D.7)` verbatim and reconstructs
`6.3.2.2 (1)\u2013(6)`.

**★★ SAME HAZARD IN ICC's `srgb.pdf` (2015)** — `_sources/srgb_bt709/srgb_icc_specification_of_srgb_2015.pdf`. Fonts are Times New Roman only, but the Symbol glyphs ride in a Times encoding: `0xF02D` ×19 (minus), `0xF02B`, `0xF03D`, `0xF028`/`0xF029`, `0xF020`/`0xF0A0`, bracket band `0xF0E6`–`0xF0FB`. **`pypdf` PRESERVES them as U+F0xx here** (unlike on ICC.1:2022) — map, don't assume dropped. **★ AND a second, unmappable hazard: §A.8's minus is U+2013 EN DASH in Times**, so a regex for `-` matches nothing.

**★★★ poppler is DISQUALIFIED for matrices in this document class.** On `srgb.pdf` `pdftotext -layout` **dropped all three `chad` minus signs AND transposed a cell**, yielding a plausible all-positive matrix. It read the *single-line* equations correctly (it independently confirmed both §B.1 defects). **For any matrix the two engines must be `pypdf` + `pdfminer.six` character coordinates.**

**Working recipe, for re-extraction:** `pypdf` for a whole-document grep corpus (fast, page-tagged); **poppler `pdftotext -layout`** for tables and single-line equations (best column reconstruction, but it scrambles equation cells **and matrices**); **`pdfminer.six`** for equations, matrices and anything glyph-critical — it is the only one that preserved `parametricCurveType` Table 68 in recoverable order. `pdftotext` lives at `/mingw64/bin/pdftotext` in the Git-Bash environment.

**Tool limitation worth not re-discovering:** the **Read tool cannot render PDF pages** here — it needs `pdftoppm`/poppler-utils, which is not installed for it, and fails with "pdftoppm is not installed". So visual page reading is unavailable and **cross-verification must be two *text* engines, not text-plus-vision.** Say so in the file when a passage is load-bearing.

**This hazard is also recorded in the corpus itself** (`LEGAL_NOTE.md` §1b, `LEGAL.md` §2.4, and inline warnings in `icc__ref__v2_v4_divergence.md` D1, `icc__s__number_encodings.md`, `cie__ref__chromatic_adaptation.md`) because a future session may extract from the PDF without reading memory first.

Related: [[icc-tos-automated-access-blocker]], [[icc-spec-corpus-sourcing-route]]
