---
name: a-retraction-is-a-grep-not-a-paragraph
description: When retracting a corpus claim, grep the whole corpus for the claim's distinguishing tokens and fix every site — a partially-corrected file is more dangerous than an uncorrected one; C1b, C10's retraction leaking the SAME DAY, and ★ C12: the rule runs for PROMOTIONS too, and there grep the A-ID not the phrase
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

**★★ It recurred 2026-08-18, hours after the retraction was written — and the leaked site was the LEAST plausible one.** `C10` retracted *"no document publishes a worked sRGB input→output triple"*. The retraction was filed as a banner at the head of `w3c/w3c__s__css_color_4.md` — **and the body of that same file still carried the live bullet** *"No worked numeric sRGB triple (input→output) anywhere. The ground-truth gap is NOT closed by this document."* **In the file describing the document that publishes four of them.**

**Three things worth carrying from the recurrence:**

1. **The intuition "surely not in *that* file" is exactly backwards.** The file most likely to keep the retracted claim is the one *about the source that refutes it*, because that is where the claim was originally most confidently written.
2. **It was found by a grep run for an unrelated reason** (chasing a `wpt.fyi` mention while sourcing web-platform-tests), not by any review of `C10`. Same mechanism as everything in [[corpus-defects-are-caught-from-outside]] — the retraction had no auditor either.
3. **A banner is the specific failure mode.** A banner at the head of a file is the cheapest way to *look* corrected. `C1b`'s rule already said "correct in place"; the addition is: **when a retraction is filed as a banner, that is a signal to sweep, not a substitute for sweeping.**

**Sweep tokens that worked here**, recorded so they are re-runnable: `no worked sRGB triple`, `No worked numeric sRGB`, `no document publishes an sRGB input`, `no worked example`. Post-fix sweep: all remaining hits are inside retraction prose quoting the claim to strike it; **zero live sites.**


---

## ★★ C12 (2026-08-18) — the rule runs in the OTHER direction, and there the token list is useless

`C1b` and `C10` are **retractions** that failed to propagate. `C12` is a
**promotion** that failed to propagate: `A7` (are only the low 16 bits of the
header `renderingIntent` significant?) was resolved from the primary on
2026-08-11 and the register said so — but `icc__s__header.md`'s per-field note
still read **"NOT SOURCED, flagged A7"** on 2026-08-18, three screens below that
same file's own table carrying the resolved `A7` row.

**Why the existing sweep procedure would NOT have caught it.** The procedure
above says: list the claim's *distinguishing tokens* and grep for them. **A
promotion has no shared tokens with the claim it supersedes.** The register row
says `RESOLVED … VERBATIM 7.2.15`; the stale site said `NOT SOURCED`. There is
no phrase in common. A phrase-grep is structurally blind here.

**The addition, and it is one line:** **when an `A`-id (or any corpus id)
changes KIND, grep the corpus for the *id string* — `A7`, `C10`, `D1` — not for
the sentence.** Ids are short, unique, and present at every site that depends on
the row. Cheap, and it works for promotions and retractions alike.

**Consequence for consumers, which is why this is worse than cosmetic:** a
"NOT SOURCED" reads to an implementer as *"we guessed; be lenient"*. The
resolved text said the opposite — a `shall`, so a non-zero high half is a
**reportable malformation**. The stale line sat in the file an implementer opens
while writing exactly that validator.

**Also:** `C12` was found by a dispatch on an **unrelated** question (who
consumes the header rendering-intent field) that merely had to read the file in
passing — n = 14, and still **zero** found by re-reading a file on purpose. See
[[corpus-defects-are-caught-from-outside]].
