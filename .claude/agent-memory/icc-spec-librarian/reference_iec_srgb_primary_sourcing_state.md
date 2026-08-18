---
name: iec-srgb-primary-sourcing-state
description: IEC 61966-2-1 primary text is PARTLY FREE — IEC's own 15-page preview is held and gives the clause map (5.2/5.3 normative, Annexes A-E informative) but ZERO constants; plus COR1:2014 newly discovered, the BT.709-3-vs-6 edition trap CLOSED, and where the four published sRGB worked examples are
metadata:
  type: reference
---

**Do not re-run this. Done 2026-08-18 (18th pass).** Files:
`ICC_Spec\iec\iec__s__srgb_iec_primary_preview.md` and
`ICC_Spec\w3c\w3c__data__css_color4_worked_examples.md`.

## ★★★ IEC publishes a FREE preview and nobody had looked

**`webstore.iec.ch` — `robots.txt` is HTTP 200 with a ZERO-BYTE body.** No
directives. **Not the `color.org` case.** Publication `6169` offers a **15-page
free preview of the 51-page standard**, agent-retrieved, held at
`_sources\srgb_bt709\iec_61966-2-1_1999_IEC_free_preview_15pp.pdf`
(`sha256 81da17ec…f06377a`).

**★ The URL is CONSTRUCTED, not guessable.** The product page's JS holds
`previewFile: '/pub/pdf/preview/info_iec61966-2-1{ed1.0}b.pdf'` and the fetch is
`https://webstore.iec.ch/en/iec_catalog/product/preview/?id=<base64 of that path
INCLUDING the leading /pub/pdf/>`. **Omit the prefix and you get the store front
page at HTTP 200** — a silent failure that looks like success. **The old
`webstore.iec.ch/preview/info_…{ed1.0}en.pdf` scheme is DEAD (301).**

**What it gives:** clause map — **clause 5 is NORMATIVE, 5.2 = RGB→XYZ,
5.3 = XYZ→RGB, pp. 21-25**; **"Annexes A, B, C, D and E are for information
only"** (verbatim); the normative-reference list; clause 1 Scope verbatim
(**rendering is out of scope** — a BY-DESIGN delegation, like ICC's
"perceptual"); the introduction's **"the simple exponent value of 2,2"** against
clause 5's `2.4` (Annex A exists *because* of that ambiguity).

**★★ What it does NOT give: any constant.** `12.92`, `1.055`, `0.055`, `2.4`,
`0.04045`, `0.0031308` — **0 grep hits each, both engines.** The preview stops
at the end of clause 2; clause 3 starts on p. 17. **Every paywall gap stands;
it is now LOCATED, not closed.** Only **CHF 210** answers whether 5.2 prints a
matrix and whether `0.04045` is fixed or derived (`A54c`).

**★★ NEW DOCUMENT: `IEC 61966-2-1:1999/COR1:2014`** (pub. `6170`, 2014-01-24).
The corpus did not know it existed. Foreword, verbatim: *"The contents of the
corrigendum of January 2014 have been included in this copy."* ⟹ **every
pre-2014 restatement may restate an uncorrected text** (`A54`). **AMD1:2003
(`6168`) and COR1:2014 have NO preview and NO published abstract** — six
candidate preview paths → HTTP 302. **The corpus records NO claim about AMD1's
content; "AMD1 adds sYCC" is not sourced and is not written down.**

## ★★★ BT.709 edition trap — raised and CLOSED the same day

IEC's clause 2 imports **ITU-R BT.709-*3*:1998, a DATED reference**, with the
rule verbatim: *"For dated references, subsequent amendments … do not apply."*
The corpus held only **BT.709-6**. **Fetched -3 in one `curl`** —
**★ the URL suffix is `-S`, not the `-I` that works for -6, which 404s**:
`https://www.itu.int/dms_pubrec/itu-r/rec/bt/R-REC-BT.709-3-199802-S!!PDF-E.pdf`
(230 146 B, `sha256 c52a7541…6ff738`, now held). **Items 1.3/1.4 are identical
to -6 digit for digit, stated TWICE (Part I and Part II), two engines. The OETF
too.** `A54b` RESOLVED. **The values never moved; the CITATION FORM did** — cite
BT.709-**3**:1998 as sRGB's normative source, -6 as corroboration.

## ★★★ Four published sRGB WORKED EXAMPLES exist, in CSS Color 4 — and half are wrong

**Retracts nine filings of "no worked sRGB triple anywhere" (`C10`).**
Verified component-by-component against each printed value's **rounding
interval** (`C5`), two computation routes agreeing to `<1e-30`:

| Example | Verdict |
|---|---|
| `#7654CD` ↔ `lab(44.36% 36.05 -58.99)` | ✔ **3/3** ← **the project's first published input→output expectation for an sRGB transform** |
| `#FFFFFF` ↔ `xyz-d50 0.9643 1 0.8251` / `xyz-d65 0.9505 1 1.089` | ✔ **6/6** |
| `color(srgb 0.691 0.139 0.259)` ↔ `srgb-linear 0.435 0.017 0.055` | ✔ **3/3** — but 2-3 s.f., **ZERO power over the breakpoint** |
| `#7654CD` ↔ `xyz-d65 0.21661 0.14602 0.59452` | ✗ **0/3** |
| `#7654CD` ↔ `xyz-d50 0.2005 0.14089 0.4472` | ✗ **1/3** |
| `lab(51.2345% -13.6271 16.2401)` → `color(srgb 0.41587 0.503670 0.36664)` (**PCS→device**) | ✗ **1/3** |

**Residuals `1.3e-5`…`1.6e-4`; `ΔE76 = 0.0123` — QUIET.** Five candidate
provenances tested, **none** reproduces the printed values; implied per-channel
exponents `2.40047/2.39966/2.39872` = noise, **not a rival definition.**
**Tier is `published_literature`, NOT `published_ground_truth`** — W3C restates
IEC, the examples are illustrative prose, 4-6 s.f., half are wrong.

**★ Use CSS's D50 (`0.9642957/1/0.8251046`, from the 4-dp chromaticity), NOT
ICC's `0.9642/1.0/0.8249`, when evaluating these** — otherwise the test fails
for a reason unrelated to the code.
**★ CSS's Bradford uses `0.8951`; ICC's own sRGB `chad` used `0.8950`.** The
`0.8951` reconstruction reproduces CSS's published `D65_to_D50` to all 17
printed digits.

**Cheapest untried lead:** the **web-platform tests** CSS Color 4 names beside
each example (`xyz-001.html`, `predefined-016.html`, `srgb-linear-001.html`, …).
Not held, not run. **If they carry higher precision they settle the three
failures for free.**

Related: [[published-ground-truth-state]], [[icc-corpus-gap-vs-nonexistence-claim]],
[[srgb-colorant-gap-routes-tried]], [[icc-tos-blocks-automated-access]],
[[derived-values-need-a-second-pass]], [[corpus-defects-are-caught-from-outside]]
