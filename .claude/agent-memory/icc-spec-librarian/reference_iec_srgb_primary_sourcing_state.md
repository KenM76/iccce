---
name: iec-srgb-primary-sourcing-state
description: sRGB sourcing state — ★ the IEC purchase is DECIDED AGAINST (2026-08-19), replaced by a reconstruction under the new `reconstructed_consensus` class; ITU-T H.273 DEFINES "IEC 61966-2-1 sRGB" WITH DIFFERENT CONSTANTS; Khronos KDF is a third clean source; plus the still-valid preview/BT.709-3/CSS-examples state
metadata:
  type: reference
---

**Do not re-run the sourcing search. Two passes done: 2026-08-18 (preview) and
2026-08-19 (reconstruction).**

## ★★★ THE PURCHASE IS A CLOSED QUESTION — do not re-file it

**2026-08-19: Ken decided NOT to buy IEC 61966-2-1** (CHF 210, pp. 16–51).
Verbatim: *"you can just do your best guess using what is available online, make
a rag of the best guess of the standard and use it, refine it if more
information becomes available."* **Recorded in `_sources\README.md` and on
`A54c`. Never recommend the purchase as a next step again** — every affected row
now carries a **`would_be_upgraded_by:`** field instead, so the question answers
itself if the document ever arrives another way.

## ★★★ NEW EVIDENCE CLASS — `reconstructed_consensus`, defined ONCE

**`ICC_Spec\EVIDENCE_CLASSES.md`** (new root file) is now the **canonical**
definition of every `evidence:` value; `_TEMPLATE.md` and `LEGAL_NOTE.md` §3
both point at it and it **wins** over them.

> *"the value is NOT read from the normative document; it is the value that N
> freely-available sources, which do not derive from one another, all print."*

★★ **Weaker than `published_ground_truth`; NOT weaker than `impl_crosscheck`,
and STRONGER in one respect — independent of any implementation, so it catches
an error lcms2 and iccce make TOGETHER.** That is the whole return.
★ **An implementation is never a countable source.** Several old sRGB "3 sources
/ 4 sources" counts were counting lcms2 and are one too high; corrected counts
live in `iec\iec__ref__srgb_reconstruction.md` §2.2.

## ★★★ THE FINDING: ITU-T H.273 gives "sRGB" DIFFERENT CONSTANTS

**Rec. ITU-T H.273 (V4) (07/2024) | ISO/IEC 23091-2** — free from itu.int, in
force, the CICP doc AVIF/HEIF/AV1/`colr` point at. `TransferCharacteristics = 13`
is labelled *"IEC 61966-2-1 sRGB"*, and clause 8.2 **defines** `α`/`β` as *"the
positive constants necessary for the curve segments … to have continuity of both
value and slope"* ⟹ **`0.0550107…` / `0.0392934…` / `0.0030413…`**, not
`0.055` / `0.04045` / `0.0031308`.

**Proved from inside H.273:** its own `TC=1` example prints
`1.099296826809442…`/`0.018053968510807…` where **BT.709 itself prints
`1.099`/`0.018`**. **Khronos KDF §13.2.3 derives BT.709's pair the same way and
pointedly does NOT apply it to sRGB.**
★★ **MEASURED: encoded max `9.76e-6`, ZERO of 256 8-bit codes change — no
experiment can decide it.** Register **`A57`**.
⚠ **`pdftotext` silently drops every `α`/`β`/`γ`** — use a `pypdfium2` raster.
⚠ **ITU-T download path is `rec/dologin_pub.asp?...&type=items`; the ITU-R
`dms_pub/itu-t/rec/t/...` path 404s.**

## Sources — what counts, what does not (all held in `_sources\srgb_bt709\`)

| Source | Counts? |
|---|---|
| **Khronos Data Format Spec 1.4.0** ★ NEW | ✔ whole constant set **clean**, exact matrix at 6 dp (§14.1), `255`-not-`256`, `80 cd/m²`, **scRGB named** as the negative-extension source. ⚠ cites `IEC/4WD` (a working draft) |
| ICC `srgb.pdf` (2015) | ✔ but ⚠ for `0.04045` (inside its six defective §B.1 lines) |
| W3C CSS Color 4 | ✔ |
| ITU-T H.273 | ✔ **and DISSENTS** |
| BT.709-**3**:1998 | ★ **primary_spec** — the DATED reference IEC clause 2 imports. **Primaries are BETTER than reconstruction**: BT.709-3 + a 2-source bridge (ICC §A.1 Note; Süsstrunk 1999) |
| **Süsstrunk/Buckley/Swen CIC7 1999** ★ NEW | ✔ **for STRUCTURE only, zero constants** — *"The encoding transformations do not take into account the veiling glare…"*, flare is in an **informative annex** |
| lcms2 | ✗ **excluded** |
| **Exif 2.32 (CIPA)**, **W3C PNG 3rd ed.** ★ NEW negatives | ✗ **checked FOR the constants and empty** — greps recorded, staged so nobody re-fetches |

## Consensus, honestly

**Only `12.92` and `2.4` have a FOUR-body consensus.** `1.055`/`0.055`/
`0.04045`/`0.0031308` have **three, with a named dissenter**.
**Viewing conditions: six of nine rows rest on ONE document** (the obsolete 1996
proposal). **Rounding rule (`A58`) and reference observer (`A60`): ZERO sources.**

## Still true from 2026-08-18, unchanged

IEC's **free 15-page preview** is held (clause map: **5.2 RGB→XYZ, 5.3 XYZ→RGB,
normative, pp. 21-25; Annexes A-E informative**; **zero constants**). ★ URL is
**base64-constructed and includes the `/pub/pdf/` prefix** — omit it and you get
the store front page at HTTP 200. **`COR1:2014` and `AMD1:2003` exist, no
preview, no abstract** ⟹ every pre-2014 restatement may be stale (`A54`).
**CSS Color 4 publishes FOUR worked sRGB examples and half are wrong.**

★ **W3C 1996 equations exist ONLY as PNGs, and `srgb14`–`srgb17` (eq 1.3–1.6,
the 8-bit encoding) were unread until 2026-08-19** — the corpus's own
equation→image map skipped all four. Read them by rendering at ×6 Lanczos.

Related: [[published-ground-truth-state]], [[icc-corpus-gap-vs-nonexistence-claim]],
[[srgb-colorant-gap-routes-tried]], [[icc-tos-blocks-automated-access]],
[[derived-values-need-a-second-pass]], [[corpus-defects-are-caught-from-outside]],
[[icc-pdf-symbol-font-sign-loss]], [[recovering-construction-from-published-matrices]]
