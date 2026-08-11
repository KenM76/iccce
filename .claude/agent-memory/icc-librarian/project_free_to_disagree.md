---
name: iccce-free-to-disagree
description: DL-023 — state what two implementations were FREE to disagree about, from their sources, before the run; and DL-022, the never-forced BPC policy as a measured, user-visible divergence
metadata:
  type: project
---

**DL-023 (`ARCHITECTURE.md` §5, filed 2026-08-11 at the Pass 5
completion filing).** *Before a cross-check is graded, state what the two
implementations were **free to disagree about** — derived from their
sources, before the run. A pre-registered negative result is a finding;
a small residual noticed afterwards is not.*

**Why.** Pass 5 read both sides' reach first — `Chain::with_bpc`'s
applicability subset against lcms2's six first-match-wins black-point
guards at pin `21c582a` — and the intersection produced a **negative
result in advance**: everywhere iccce does BPC at all, **lcms2's
estimator reduces to the same two values** (`XYZ (0,0,0)` on every
matrix/TRC or gray side, because every TRC in reach has `trc(0) = 0`;
the same **A41** triple on a v4 LUT side at perceptual). So Pass 5's
rows grade **the scaling map, the direction and the pipeline — never the
ESTIMATORS**. A session that measured first would have found six small
numbers and read them as six independent agreements about "BPC".

**Three obligations.** (a) Read both reaches before running anything.
(b) Publish the negative result **at the top of the coverage
statement**, not in a footnote. (c) **Name the instrument that would
close the gap** so it becomes work rather than a permanent hedge — Pass
5's is *a synthetic v4 RGB-or-gray LUT fixture with a NON-ZERO device
black*, which does not exist (38 `.icc` in `fixtures/synthetic/`, one v4
LUT, black zero).

**Its cheap companion: print the sensitivity ratio.** *"iccce and lcms2
agree to 1,1×10⁻⁴"* is empty until *"BPC itself moves this by 3,5159
ΔE2000"* sits beside it — **388×** and **682×** in Pass 5, and **free**,
because the BPC-off arm is already run as the baseline.

**Not covered by DL-018 or DL-021.** DL-018 is about the **gate** (a
prediction pin so deleting a requirement can't make it greener);
DL-021 is about **scope** (name the direction). **DL-023 is about what
the comparison can distinguish at all** — a property of the *scenario
set*, fixed before any tolerance exists. A suite can satisfy both older
rules and still consist of comparisons that could not have failed.

**DL-022, filed the same day.** *iccce **never forces** BPC; it is an
explicit caller act.* lcms2 forces it for a **v4 destination** at
perceptual/saturation, overwriting the caller's flag before it is read.
Measured unasked-vs-unasked at **3,137 348 `L*`** (NC-100), matching the
corpus's **D11** fingerprint to 1,1×10⁻⁴ and identifying **lcms2's M2
route, not iccDEV's** — the sign diagnosed by measuring the *opposite*
direction. **REPORTED, NOT GRADED** (DL-019): the enable policy rests on
an unread Adobe document and Maria (2013) is silent on it. Promoted out
of a paragraph inside NA-009 because it now has a measured size, a
graded posture and a **user-visible** consequence: two correct CMMs give
different pictures by default, silently, through a flag on a shipped
binary.

Related: [[iccce-pass-status]], [[iccce-direction-scoped-behaviour]],
[[iccce-tolerance-cannot-swallow-and-claim]],
[[iccce-gate-must-not-reward-deletion]],
[[iccce-refusal-discharged-by-fixture]].
