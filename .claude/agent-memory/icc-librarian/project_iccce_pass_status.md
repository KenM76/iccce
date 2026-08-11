---
name: iccce-pass-status
description: iccce status snapshot 2026-08-11 (Pass 2 DONE, Pass 4 evaluation surface complete but B2A/mAB/gray unmeasured) — next free NC-062, DL-020 filed, GP-001 fixed
metadata:
  type: project
---

**Snapshot of 2026-08-11 (the evaluation-surface filing — the ninth of
one calendar day). Verify before relying on any of it** — read
`docs/ROADMAP.md` (Pass 4 evaluation-surface block, Pass 2 DONE block),
`docs/NUMERIC_CLAIMS.md` §3.10 + §7.6, `docs/NEXT_SESSION.md` and the
newest `SESSION_LOG.md` entry.

**Pass 0 done · Pass 1 core · ★ Pass 2 DONE · Pass 3 DONE · Pass 4 IN
PROGRESS, done-when NOT met.**

**Pass 2 closed because the STRONGER reading of clause 2 was satisfied,
not because the operator answered** (he never did): 38 whole `.icc`
fixtures on disk + a standalone generator with `verify` + a generated
MANIFEST, covering every tag type the plan names. Boundary: clause 1's
40-profile sweep **predates the GP-001 fix** and was not re-run; `desc`
has **no ICC.1:2022 clause at all**.

**★ Pass 4's evaluation surface is COMPLETE in CODE only.** `lut_ab.rs`
(mAB/mBA, both directions, v4 encodings, all 12 matrix terms) and
`gray_trc.rs` (F.2 both directions) landed and are wired into `Chain` on
both sides. **B2A has ONE number (NC-057), `mAB ` has no real file, gray
has NO lcms2 comparison, and no test traverses `Chain` into either new
model.** Pass 4 still has **no ground-truth row**.

**Ledger: NC-057 … NC-061 filed; next free NC-062.** NC-057 = the mBA
fixture's B2A0, Lab(50,0,0) → K within 1e-3 of transicc's recorded
0.496117 — the project's first claim through **bytes it authored
itself**. **NA-008** new (grayTRC inverse projects onto the achromatic
channel; cost UNMEASURED and it is a *gamut-mapping* cost, zero on the
neutral axis where both gray tests sit). **No gate report accompanied
that dispatch** — those five rows are asserted bounds with no outcome.

**★ GP-001 (fixed in `2e98cfd`):** `decode_lut_ab` used the mAB curve
convention for BOTH types → every real CMYK B2A0 refused; invisible on
square LUTs. Per type: mAB 10.12.2/4/6 (B/M = output, A = input); mBA
10.13.2/4/6 (B/M = input, A = output). **The evaluator had refused mBA
an hour earlier on that exact doubt.** → `ARCHITECTURE.md` **DL-020**.
Still open elsewhere: the corpus's blanket sentence in
`icc__type__lutAtoB_lutBtoA.md`, and `tools/gen-profiles/README.md` §5
still saying `Status: open`.

**Corpus 6th pass:** M4/M5 landed. **lcms2 does NOT "ignore" v2 `wtpt`**
— `_cmsReadCHAD` synthesises a Bradford chad from it under the same
guard, so its v2-display model is coherent. **DemoIccMAX reads wtpt
as-stored ⇒ the two ICC-adjacent implementations disagree and iccce
matches ICC's own code.** M4 generalises to `EvalNInputs` (linear in
first N−3, tetrahedral in last 3). **A4b still UNVERIFIED** (only
ICC.1:2001-04 settles it; ICC errata unreachable by compliant means).
**A4c NEW/SILENT**: no colorant↔wtpt self-consistency required; the stock
sRGB profile's colorants sum to D50 while wtpt holds D65.

**Counts, verified:** 95 `#[test]` declarations across 16 files under
`crates/` (was 89/14); 52 under `tools/`, 28 in gen-profiles. **38**
`.icc` fixtures — my own prior filing said 39 and was wrong.

**Commits, ALL reported — no agent in this project has ever run git:**
`7576cfa` (gen-profiles + fixtures + GP-001 found), `2e98cfd` (GP-001
fixed + mAB/mBA evaluation), `97ad9fa` (grayTRC F.2 + the previous
filing + two code-doc closures).

Related: [[iccce-refusal-discharged-by-fixture]],
[[iccce-verify-own-draft-too]],
[[iccce-tolerance-cannot-swallow-and-claim]],
[[iccce-predicted-divergence-must-be-measured]],
[[iccce-gate-must-not-reward-deletion]],
[[iccce-bound-cannot-catch-its-own-magnitude]],
[[iccce-verification-loop-runs-both-ways]], [[ken-terse-scope-decisions]].
