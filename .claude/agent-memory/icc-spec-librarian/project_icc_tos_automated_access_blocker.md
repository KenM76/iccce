---
name: icc-tos-automated-access-blocker
description: color.org's ToS bans automated/AI access, so the ICC.1 spec PDF cannot be agent-downloaded; awaiting a human browser download from Ken to D:\Dev\Rag-Specialized\ICC_Spec\_sources\
metadata:
  type: project
---

**The ICC.1 specification PDF has NOT been retrieved, and an agent must not retrieve it.** color.org's Terms of Service (Effective 2026-01-01, checked 2026-08-11) prohibit "using any robot, spider, or other automated device to access the Services for any purpose, including **monitoring, copying, or training artificial intelligence or machine learning models**, without prior written consent from ICC."

**Why:** the clause names AI/ML explicitly, so it is not a strained reading of a generic anti-scraping term. On reading it during the 2026-08-11 corpus kickstart, retrieval stopped at the three legal/index pages needed to establish the terms. Note the conflicting signal, recorded deliberately: `www.color.org/robots.txt` does *not* disallow the specification index, and `archive.color.org` serves no robots.txt at all — **the machine-readable permission and the prose contract point opposite ways, and the prose contract was taken as binding.**

**How to apply:**
- **Do not fetch anything from `color.org` or `archive.color.org` with curl/WebFetch/WebSearch-fetch**, including the spec PDFs, news pages, or the v4spec pages. Report the constraint rather than routing around it.
- **The unblock is Ken downloading `ICC.1-2022-05.pdf` in a browser** (a human act, outside the robot clause) to `D:\Dev\Rag-Specialized\ICC_Spec\_sources\`. It takes ~2 minutes and clears ~15 UNVERIFIED rows in the ambiguity register plus the entire required/optional tag column (**A30**) — which is what currently makes an ICC *validator* undefensible while a *parser* is fine. **Worth raising with Ken proactively at the start of any ICC session.**
- **Acquisition still ≠ redistribution.** Even with the PDF in hand the ToS grants no reproduction right and there is no permission notice anywhere on the site: summarise and cite, short verbatim quotation with a clause citation only, no bulk verbatim clause text.
- **Keep the corpus off networked storage** — the ToS bars reproduction in "any other web site or networked computer environment". Not `R:\` (Dropbox-synced), not any share. `D:\` is correct.

Full detail: `D:\Dev\iccce\docs\LEGAL.md` §2.1–2.3 and `D:\Dev\Rag-Specialized\ICC_Spec\LEGAL_NOTE.md` §1.

Related: [[icc-spec-corpus-sourcing-route]]
