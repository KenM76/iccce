---
name: icc1-pdf-operator-blocker
description: RESOLVED 2026-08-11 — Ken hand-downloaded ICC.1-2022-05.pdf after color.org's ToS blocked agent retrieval; the generalisable lesson is that "free download" never implies "automated retrieval permitted"
metadata:
  type: project
---

**Status: CLEARED 2026-08-11.**
`D:\Dev\Rag-Specialized\ICC_Spec\_sources\ICC.1-2022-05.pdf` now exists —
*verified* by `icc-librarian` listing that directory. Ken downloaded it
manually (reported: 11:12). Recorded as `ARCHITECTURE.md` **DL-006**,
which deliberately records only the **trigger**, not the corpus decision.

**What was blocking it.** color.org's ToS (effective 2026-01-01) forbids
*"any robot, spider, or other automated device … including … training
artificial intelligence or machine learning models"* — naming AI/ML
explicitly, so an agent fetching the PDF was squarely inside it. The
site's own `robots.txt` pointed the *other* way. The prose contract was
taken as binding and the conflict written down rather than resolved
silently → **DL-002**, `LEGAL.md` §2.1–§2.3.

**How to apply:**

- **The DL-002 clause-citation prohibition may still be live.** It was
  never about the file existing on disk — it was about there being no
  *sourced clause text* to cite. It lifts only when
  `icc-spec-librarian` files DL-002's successor entry. **Check
  `ARCHITECTURE.md` §5 for an entry after DL-009 before letting anything
  cite an ICC.1 clause number.** As of this writing, none existed.
- **Nothing about the PDF beyond its existence has been checked** — not
  size, hash, page count, nor that it is the document its name claims.
- **The generalisable lesson, which now has a live next victim:** *"the
  document is offered free"* does **not** imply *"automated retrieval is
  permitted."* Pass 9 (HDR) needs ITU-R recommendations, reported as free
  from `itu.int`. That report is a **claim about a third party's terms**
  and gets checked before anything is fetched — the same way color.org's
  was. Do not let a free-download framing substitute for reading the
  terms.
- Still true: **do not re-attempt automated retrieval** of any
  color.org / archive.color.org document.

Related: [[iccce-pass-status]], [[ken-terse-scope-decisions]].
