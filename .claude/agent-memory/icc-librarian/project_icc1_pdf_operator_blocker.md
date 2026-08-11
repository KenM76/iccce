---
name: icc1-pdf-operator-blocker
description: iccce's standards corpus has no primary-spec tier because color.org's ToS bans automated retrieval; only Ken can unblock it by downloading ICC.1-2022-05.pdf in a browser
metadata:
  type: project
---

**As of 2026-08-11 (Pass 0 close): no ICC-published specification PDF has
ever been retrieved for this project, and the unblock is an operator
action nobody else can take.** `icc-spec-librarian` built the 21-file
corpus at `D:\Dev\Rag-Specialized\ICC_Spec\` by cross-verifying ICC's own
`icProfileHeader.h` (BSD-3) against `lcms2.h` (MIT) instead. Recorded as
decision **DL-002** in `docs/ARCHITECTURE.md` §5; full evidence in
`docs/LEGAL.md` §2.1–§2.3.

**Why:** color.org's Terms of Service (effective 2026-01-01) prohibit
using "any robot, spider, or other automated device to access the
Services … including … training artificial intelligence or machine
learning models" — a clause that names AI/ML explicitly, so an agent
fetching the PDF is squarely inside it. The site's own `robots.txt`
points the *other* way (it does not disallow the spec index); the prose
contract was taken as binding and the conflict recorded rather than
resolved silently. The unblock is Ken downloading
`https://archive.color.org/specification/ICC.1-2022-05.pdf` in a browser
to `ICC_Spec\_sources\` — a human retrieval, outside the robot clause,
~2 minutes.

**How to apply:**
- **Do not re-attempt automated retrieval** of any color.org / archive.color.org
  document, and do not treat the permissive `robots.txt` as settling it.
  This was decided with the evidence in hand, not by default.
- Until the PDF lands, **no claim in iccce may cite an ICC.1 clause
  number.** Cite corpus filenames (`icc__s__header.md` etc.), as the
  parser's doc comments already do. The consequence the corpus librarian
  stated: *a parser is defensible on this evidence and a validator is
  not.*
- If Ken asks what is blocking depth in the corpus, or why some tag is
  "NOT SOURCED", this is the answer — and it clears ~15 UNVERIFIED
  ambiguity rows plus the entire required/optional tag column.
- **Verify before relying on this:** check whether
  `D:\Dev\Rag-Specialized\ICC_Spec\_sources\ICC.1-2022-05.pdf` now
  exists. If it does, this memory is stale — the corpus can gain a
  `primary_spec` tier and a new decision-log entry should reference
  DL-002.

Related: [[iccce-pass-status]].
