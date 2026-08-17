---
name: project-ghent-compatibility
description: iccce targets compatibility with the Ghent PDF Output Suite 5.0; the suite lives outside the repo and its licence bars redistribution
metadata:
  type: project
---

**iccce is aiming at *compatibility* with the Ghent PDF Output Suite 5.0
(GWG), the graphic-arts PDF/X test corpus.** Set by Ken 2026-08-17,
alongside the request-channel work — Ghent is the standard the print
industry actually checks a PDF workflow against, and `pdfce` is the
consumer that would be checked.

**Why:** it is the closest thing available to a real-world acceptance
test for the colour half of a PDF engine, and — this is the part that
makes it worth doing — most of the ICC-relevant subset needs **no
measurement hardware**. See [[feedback-compatibility-not-compliance]].

**How to apply:**

- The suite sits at
  `D:\Dev\temp\pdfce\Ghent_PDF_Output_Suite_V50_Testpages\`. Its three
  categories are `1-CMYK`, `2-SPOT` and `3-ICC-CMS`; **only `3-ICC-CMS`
  is substantially ours** — the rest is overprint, transparency,
  softmasks, fonts and image codecs, which the channel's boundary table
  assigns to `pdfce`.
- ★ **Licence: the suite may not be used commercially or redistributed
  without GWG's written permission**, and the ICC profiles embedded in it
  carry Adobe's / ECI's / X-Rite's own separate terms. **Nothing derived
  from it may enter the MIT repository.** Extracted profiles live in
  `D:\Dev\iccce-private-fixtures\ghent-v50\` with the project's other
  restricted corpora, resolved by tests through `$ICCCE_PRIVATE_FIXTURES`
  and skipped when absent.
- The suite's own pass/fail signal is **visual** — a red X at 0.5 m
  viewing distance — and GWG states a faint X is not a failure. So the
  suite itself supplies **no numeric tolerance and no reference
  measurement**. Any number iccce states about it has to come from
  iccce's own apparatus, and must say so.
- ★ GWG ships **deliberate trap profiles** (`RGB mntr mtx X (Switch red
  green)`, `CMYK prtr lut X (Switch magenta cyan)`). They are unusually
  valuable because the correct answer is knowable *without an oracle*:
  the swap is the profile's declared content, so an engine that honours
  the source profile must return green for red. Do not lose these.

Durable findings live in `docs/` in git, never only in memory or in the
channel folder. Related: [[reference-request-channel-polling]].
