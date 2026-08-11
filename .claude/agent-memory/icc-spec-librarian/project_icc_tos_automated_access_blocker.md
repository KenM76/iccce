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
  3. **★ `https://www.color.org/adobebpc.pdf` — "Adobe Systems' Implementation of Black Point Compensation" (2006, 12 pp). START AT ITS §7.2.** Settles **A42**, the Tier 2 blocker on Pass 5 (the black-point *estimation* algorithm, incl. the least-squares quadratic fit) and whether lcms2's "BPC is forced on at v4 perceptual/saturation" (`M2`) is Adobe's rule or lcms2's. **★ Note the host: this is an ADOBE document served from `color.org`, so it is ToS-barred like everything else there — it is NOT on `adobe.com`, and `adobe.com` is separately unreachable from this environment (curl HTTP 000, WebFetch timeout), so Adobe's own terms are UNKNOWN rather than permissive.** Also wanted, same page: **ICC White Paper 40** (`www.color.org/BlackPointCompensation.pdf` / `archive.color.org/files/WP40-Black_Point_Compensation_2010-07-27.pdf`) — Maria 2013 says it *"includes corrections that weren't addressed in the original Adobe paper"*, **so it may disagree with the Adobe document, and that disagreement is itself the finding.**
  4. **`https://pdfa.org/wp-content/uploads/2018/09/PDF20_AN001-BPC.pdf`** — PDF 2.0 App Note 001 on BPC (`UseBlackPtComp`). **Not ToS-barred; `pdfa.org` simply returns 403 to every agent tool.** A browser should work. Tier 4.
  5. ICC's published **D65→D50 `chad`** values cited by Annex E.4.2.
- **ICC's errata and technical-note pages are equally barred**, and ICC publishes them nowhere else — so "check the errata" is not an available move. **Report that as a tool limit; do not go looking for a mirror.**
- **Acquisition ≠ redistribution, and this did not change.** The ToS grants no reproduction right and there is no permission notice anywhere on the site. **Summarise and cite; short verbatim quotation with a clause citation only; no bulk verbatim clause text leaving the corpus.**
- **Keep the corpus off networked storage** — the ToS bars reproduction in "any other web site or networked computer environment". Not `R:\` (Dropbox-synced), not any share, never a git remote. `D:\` is correct.

Full detail: `D:\Dev\iccce\docs\LEGAL.md` §2.1–§2.4 and `D:\Dev\Rag-Specialized\ICC_Spec\LEGAL_NOTE.md` §1–§1c.

Related: [[icc-spec-corpus-sourcing-route]], [[icc-pdf-symbol-font-sign-loss]], [[icc-bpc-sourcing-state]]
