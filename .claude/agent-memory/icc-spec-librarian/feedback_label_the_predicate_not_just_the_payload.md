---
name: label-the-predicate-not-just-the-payload
description: When a corpus row's numbers are cross-verified, that says nothing about the conditional that decides when to use them — label the predicate separately, or a well-sourced file ships an unsourced rule
metadata:
  type: feedback
---

**A corpus entry's *constants* and its *selector rule* need separate evidence labels.** Cross-verifying the numbers does not cross-verify the condition under which they apply.

**Why:** on 2026-08-11 the ICC_Spec corpus's highest-value row (`icc__ref__v2_v4_divergence.md` D1/D2, the legacy 16-bit Lab encoding) had constants extracted independently from two codebases — genuinely cross-verified — while its *selector* ("the encoding attaches to the **profile version**") was an inference from lcms2's code structure. ICC.1:2022 clause 6.3.4.2 NOTE 3 says the opposite: it attaches to the **tag type**, `lut16Type` and `namedColor2Type` "and only those tag types", with no version condition anywhere. The row read as confident, symptom-rich, cross-verified prose, and **half of it was unsourced.**

**What saved it:** the first pass wrote "NOT SOURCED — flagged, do not guess" directly under the claim, raised it as **A1**, ranked A1 the most important open item in the corpus, and put it in the index's gaps table. So it was the first thing checked when the spec arrived, and no code had been written against it. **The labelling discipline worked even though the claim was wrong** — that is the whole return on the discipline, and it is worth remembering as a success, not only as a near-miss.

**How to apply:**
- When writing any corpus row of the form "use X when Y", ask separately: *what sources X?* and *what sources Y?* Give them separate evidence markers if they differ.
- Treat "this implementation does it this way and is self-consistent" as evidence about **the implementation**, never about **the standard** — even when the implementation is the field's reference CMM.
- Keep the retraction visible when one is proved wrong. The corpus keeps its own error in `icc__ref__spec_defects.md` §9 with a post-mortem, rather than quietly rewriting history — a corpus that only lists other people's errors is not being audited.
