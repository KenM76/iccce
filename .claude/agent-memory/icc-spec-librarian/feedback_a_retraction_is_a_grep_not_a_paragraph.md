---
name: a-retraction-is-a-grep-not-a-paragraph
description: When retracting a corpus claim, grep the whole corpus for the claim's distinguishing tokens and fix every site — a partially-corrected file is more dangerous than an uncorrected one; this is C1b
metadata:
  type: feedback
---

**Filing a retraction as a new heading is not filing a retraction. Sweep the corpus for the retracted claim's distinguishing tokens, fix or annotate every hit, and record the token list in the defect entry so a later reader can re-run the sweep.**

**Why:** the C1 retraction (legacy 16-bit PCSLAB is selected by **tag type**, not `header.version`) was written at the top of `icc__type__lut8_lut16.md`, in `icc__ref__v2_v4_divergence.md` D1/D2, and in `icc__ref__spec_defects.md` §9 — and the *bodies* of two files kept the retracted claim:

- `icc__type__lut8_lut16.md`, body bullet: *"the selector is `header.version`, not the tag type"* — **34 lines below a correct statement of the opposite rule in the same file.**
- `icc__s__pcs_encoding.md` §2: *"Selector: `header.version < 0x04000000` … Not the tag type"*, compounded by grouping `lut8Type` with `lut16Type` (6.3.4.2 NOTE 3 excludes it) and by table column headers reading "v2 / v3 (legacy) | v4", so the inversion survived in the labels too.

Filed as **C1b** (`icc__ref__spec_defects.md` §18), an instance of C1 rather than a new defect id.

**The part worth carrying:** *a partially-corrected file is a worse hazard than an uncorrected one.* An uncorrected file is wrong consistently, so a reader who checks one statement has effectively checked them all. A partially-corrected file is wrong locally, next to material that is right, and the visible correction **raises the file's apparent authority while leaving a wrong sentence in it.**

**How to apply:**
- Before closing any retraction: list the claim's distinguishing tokens (for C1: `header.version`, `0x04000000`, `version-dependent`, `selector is`), grep the whole corpus, and triage every hit into corrected / correct-as-is / historical-quotation.
- **Correct in place, quoting the retracted wording at the site**, rather than deleting it — a future reader arriving at that line needs to know it changed.
- Record the sweep, the tokens and the hit counts in the defect entry. That turns "I fixed it" into something re-runnable.
- Check table **column headers** and **section headings**, not just prose; they carry the claim too and grep for prose tokens misses them.
- **This was found by a second consumer of the corpus, not by the corpus** — same as C4. See [[corpus-defects-are-caught-from-outside]].
