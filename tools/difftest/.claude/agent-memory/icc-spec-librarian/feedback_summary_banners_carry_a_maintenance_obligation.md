---
name: feedback-summary-banners-carry-a-maintenance-obligation
description: At the end of every pass, re-read the FIRST SCREEN of each file touched — a stale "single source / GAP / not cross-verified" banner is what other agents read and quote onward
metadata:
  type: feedback
---

**When a pass adds evidence to a corpus file, edit the file's top banner in the
same edit. At the end of every pass, re-read the first screen of every file
touched and ask whether it still describes the file's current state.**

**Why:** ICC_Spec defect **C7** (2026-08-17). `iec/iec__s__srgb.md` opened with
*"All values below come from **one** source: lcms2 … **NOT cross-verified** … A
second independent source is a recorded GAP."* On 2026-08-12 a pass added a
whole section cross-verifying the primaries, white point and both matrices
against a second publication — **and correctly said so in a status table at the
bottom of the same file.** The banner was never touched. On 2026-08-17 an
`icc-engineer` dispatch opened the file, quoted the banner **verbatim** as the
current state (*"[VERIFIED — I read the file this session, 246 lines]"*), and
commissioned a pass to close a gap that had been half-closed for five days.
**The verification was real; the thing verified was stale.** A summary banner is
a claim with a maintenance obligation — and it is the part most likely to be
read alone and quoted onward.

**How to apply:**
- Adding a section under a "single source / GAP / UNVERIFIED / not
  cross-verified" banner **without editing the banner is a defect, not an
  omission.**
- Prefer a **per-constant status table** near the top over a prose banner —
  it forces the update because there is a row to change.
- **Sweep beyond the file you edited.** Applying this check on 2026-08-17 caught
  three further stale sites in `icc__ref__ground_truth_availability.md` that the
  same pass would otherwise have left contradicting the new files. Grep the
  whole corpus for the retracted phrase, not just the file you were working in.
- Related but distinct from **C1b** ("the correction did not reach the claims").
  **C7 is "the correction did not reach the summary."**

Full write-up:
`D:\Dev\Rag-Specialized\ICC_Spec\icc\icc__ref__spec_defects.md` §22.
See also [[feedback-tool-limit-findings-need-the-invocation]].
