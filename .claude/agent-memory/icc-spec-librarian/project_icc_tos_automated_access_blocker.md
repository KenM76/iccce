---
name: icc-tos-automated-access-blocker
description: color.org's ToS bans automated/AI access — never agent-fetch color.org. The ICC.1:2022 PDF was human-downloaded by Ken 2026-08-11 and is now in _sources/; acquisition is still not a redistribution licence.
metadata:
  type: project
---

**Rule: never fetch anything from `color.org` or `archive.color.org` with curl / WebFetch / any agent tool.** Report the constraint rather than routing around it.

**Why:** the ToS (Effective 2026-01-01, checked 2026-08-11) prohibits "using any robot, spider, or other automated device to access the Services for any purpose, including **monitoring, copying, or training artificial intelligence or machine learning models**, without prior written consent from ICC." The clause names AI/ML explicitly, so it is not a strained reading of a generic anti-scraping term. Recorded deliberately as a conflicting signal: `www.color.org/robots.txt` does *not* disallow the specification index and `archive.color.org` serves no robots.txt at all — **the machine-readable permission and the prose contract point opposite ways, and the prose contract was taken as binding.**

**How to apply:**

- **The spec is already here — do not re-fetch it.** Ken browser-downloaded `ICC.1-2022-05.pdf` (905 961 bytes, ICC.1:2022, v4.4.0.0, 126 pages) to `D:\Dev\Rag-Specialized\ICC_Spec\_sources\` on **2026-08-11**. The corpus was rebuilt against it the same day (second pass) and cites it as `primary_spec`.
- **If another ICC document is needed, ask Ken to download it** — that is the only route. Currently wanted, in priority order **as revised 2026-08-11 (6th pass)**:
  1. **★ ICC.1:2001-04 (the v2 spec) — now the top gap, and no longer archaeology.** It is the only document that can settle **A4b**: whether a **v2 display** profile's `wtpt` is the adapted or the unadapted white. **Stake: a measured 11.2 ΔE2000 absolute-intent divergence** on the stock Windows sRGB profile — lcms2 substitutes D50, ICC's own DemoIccMAX reads the tag as stored, and the corpus cannot say which is right. Also settles A1b/A2/A34/A39c and `textDescriptionType`.
  2. **ICC.1:2010-12** — what changed in `parametricCurveType` Table 68 (D10/A31). **It will probably NOT settle A4b**: the 2022 Foreword's change list does not touch `wtpt`, so 9.2.36's display rule is already in the 2010 text.
  3. ICC's published **D65→D50 `chad`** values cited by Annex E.4.2; Adobe's BPC document / ICC's BPC white paper (A28, Tier 2).
- **ICC's errata and technical-note pages are equally barred**, and ICC publishes them nowhere else — so "check the errata" is not an available move. **Report that as a tool limit; do not go looking for a mirror.**
- **Acquisition ≠ redistribution, and this did not change.** The ToS grants no reproduction right and there is no permission notice anywhere on the site. **Summarise and cite; short verbatim quotation with a clause citation only; no bulk verbatim clause text leaving the corpus.**
- **Keep the corpus off networked storage** — the ToS bars reproduction in "any other web site or networked computer environment". Not `R:\` (Dropbox-synced), not any share, never a git remote. `D:\` is correct.

Full detail: `D:\Dev\iccce\docs\LEGAL.md` §2.1–§2.4 and `D:\Dev\Rag-Specialized\ICC_Spec\LEGAL_NOTE.md` §1–§1c.

Related: [[icc-spec-corpus-sourcing-route]], [[icc-pdf-symbol-font-sign-loss]]
