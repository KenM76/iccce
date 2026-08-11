---
name: iccce-pass-status
description: iccce Pass 0 and Pass 1 core both done 2026-08-11; NUMERIC_CLAIMS.md now EXISTS with NC-001 (Sharma 34/34); Pass 2 next and a validator is now defensible
metadata:
  type: project
---

**Snapshot of 2026-08-11 (end of Pass 1). Verify before relying on any of
it** — read `docs/ROADMAP.md`, `docs/NUMERIC_CLAIMS.md` and the newest
`docs/SESSION_LOG.md` entry.

**Pass 0 done** (commit `f976a0e`). **Pass 1 core complete and validated
the same day** — `iccce-color` has XYZ/xyY, Lab/LCh, D50 + D65, von Kries
method with Bradford cones, ΔE76 and CIEDE2000. **Filed uncommitted**: no
Pass 1 hash exists; the ledger and ROADMAP both say so and ask whoever
commits to fill it in.

**`docs/NUMERIC_CLAIMS.md` EXISTS as of 2026-08-11** — created with the
first genuinely measured claim, exactly as planned. Supersedes the old
"deliberately uncreated" note.

- **NC-001**: CIEDE2000 vs **all 34** Sharma, Wu & Dalal (2005) pairs,
  within **1×10⁻⁴** (the data's own precision), k=1:1:1. Published ground
  truth — the project's first.
- Everything else is `arithmetic-identity`. **Zero
  `implementation-cross-check` rows** — no Rust difftest harness exists.
  **No ground-truth row for chromatic adaptation** (no published worked
  example obtained) — Pass 1's largest evidential hole.
- Ledger design: 7 evidence classes; §1.1 says a passing test records the
  **bound asserted, not the residual observed**; §4 named approximations;
  §5 what Pass 1 does **not** claim; §6 invalidation map; §7 owed.

**Pass 2 (full tag-type parsing) is next, and the evidence position
changed**: the ICC.1:2022 ingest landed, the corpus now has a
`primary_spec` tier with real clause numbers and the required-tags
column, so **a validator is now defensible** — Pass 0's "a validator is
not" line is stale and was annotated, not rewritten.

**Two new decision-log entries, 2026-08-11:**

- **DL-010** — Lab `f(t)` uses the exact rational form: iccce's **first
  stated deviation from normative spec text** (ICC.1:2022 6.4 prints the
  decimal `0,008 856`, but *delegates* `f(t)` to ISO 13655, which is
  paywalled and unobtained). Cost ~10⁻⁵ in `L*` is a **corpus-derived
  analytic bound, NOT an iccce measurement** — never restate it as
  measured.
- **DL-011** — legacy Lab encoding keys off the **TAG TYPE** (`mft2`,
  `ncl2`, "and only those tag types"), never `header.version`. The
  corpus's first pass claimed the opposite and is **retracted**. **lcms2
  keys on version** → live disagreement; owed: an `icc-conformance`
  behavioural difftest (`mft2` Lab `A2B0` in a synthetic v4 profile
  through `transicc`; 652.8 vs 655.35). Whether lcms2 is *behaviourally*
  wrong is **not established**.

**Open bookkeeping gap worth checking every session:** **DL-002's
clause-citation prohibition still has no filed successor** — §5 ran to
DL-009 before DL-010/011 — while code and DL-011 already cite ICC.1:2022
clause numbers. `icc-spec-librarian` owes that entry.

Also still open at filing: the corpus D50-chromaticity erratum (below),
`TOLERANCES.md` §3.1 and §5 both still empty, nothing ever run on Linux,
no CI run ever observed.

Related: [[iccce-verification-loop-runs-both-ways]],
[[icc1-pdf-operator-blocker]], [[ken-terse-scope-decisions]].
