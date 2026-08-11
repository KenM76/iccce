---
name: iccce-pass-status
description: iccce status snapshot 2026-08-11 (Pass 5 DONE — done-when met on stated terms, estimators never discriminated; Pass 4 still open on saturation + A4b) — next free NC-105, DL-022/DL-023 filed, Pass 6 is next
metadata:
  type: project
---

**Snapshot of 2026-08-11 (the Pass 5 completion filing — the eleventh of
one calendar day). Verify before relying on any of it** — read
`docs/ROADMAP.md` (Pass 5 completion record, second Pass 6 annotation),
`docs/NUMERIC_CLAIMS.md` §2.8 + §3.12 + §7.8, `docs/NEXT_SESSION.md`,
newest `SESSION_LOG.md`.

**Pass 0 done · Pass 1 core · Pass 2 DONE · Pass 3 DONE · Pass 4 IN
PROGRESS (done-when open on exactly two items: saturation in B2A, and
ICC-absolute blocked on operator-only `ICC.1:2001-04` via A4b) ·
★★ Pass 5 DONE, done-when MET on stated terms.**

**Pass 5's headline is a BOUNDARY, not a number.** Done-when met:
direction **0,0 exactly** (a sign with an algebraic proof, no tolerance);
lcms2 match **1,110 588×10⁻⁴** device out of the fixture / **4,600×10⁻⁵**
into it, baseline (BPC off) **1,012 157×10⁻⁴** graded first; **map vs
ICC.1:2022 clause 6.3.4.3's printed equation 1,110×10⁻¹⁶**, vs Maria
(2013) §4.2 **3,331×10⁻¹⁶**. Six scenarios **pre-registered from both
sources before running, all six confirmed** *(reported)*. **BUT the two
ESTIMATORS were never discriminated** — a negative result derived in
advance ⇒ **DL-023**; the only instrument is **a non-zero-black v4 LUT
fixture, which does not exist** (owed to gen-profiles). Also: lcms2 does
**no BPC at all** below `IsEmptyLayer`'s **0,002** (≈0,41 `L*`,
**solved for, not observed**) — iccce deliberately lacks it; and
**iccce NEVER forces BPC** ⇒ **DL-022**, 3,137 348 `L*`, **reported not
graded**.

**★ My judgements this filing.** (1) **Class**: the map row **NC-084 is
`derived-expectation`, NOT `normative-rule-conformance`** — the corpus's
`evidence:` line for `icc__ref__bpc.md` §2/§3 reads
**`cross_verified_2src`**, the ledger's first normative-rule rows are
NC-022…NC-027 (Pass 3), and it grades a **map function**, not a
transform. `bpc.rs` still heads it "PRIMARY-SOURCED" — **the DL-014
audit now decides a ledger class.** (2) **Dispatch correction**: it
called 4,6×10⁻⁵ *"the policy arm"*; that is **NC-096** (cross-check),
the policy is **NC-100** at 3,137 3×10⁻², **ungraded**. (3) `pass5.rs`
has **NO `#[test]`** (pass3 7, pass4 7, pass4b 8) — 14 rows on unpinned
grids; and **§16 states no `pass=` line**, so Pass 5's 26 records is
**my subtraction** (whole suite 90 − 64), reconciling with 5+7+8+6.

**★ The double red-commit incident** (recorded in §2.8 + session log):
`70411dd` claimed *"102 workspace tests green"* with one red — gate was
`… | grep -E '…|FAILED' && commit`, **grep exits 0 on a FAILED match**;
corrected `a36abaf`. Then `6ea1b3d` (dispatch says *"104 green"* — **the
lesson file does not carry that number**) via `… | tail -2 && commit`,
**tail exits 0**; corrected `812a215`. Lesson at
`C:\personal_rag\claude_code\lesson_20260811_grep_on_test_output_matches_failed_lines_with_exit_0.md`
*(read)*, **hardened with its own author's recidivism**. Consequence:
**no ledger row inherits a gate claim from a commit message.**

**Ledger: NC-084 … NC-104 filed; next free NC-105.** NA-010's cost is
now **MEASURED** (NC-094: ΔL* 0,005 364 / ΔE76 0,037 416 / **ΔE2000
0,050 201** — corroborates the corpus to 2×10⁻⁵ by an independent
route). **NA-009's cost is still unmeasured and now for a stated
reason** (reachable ≠ discriminable).

**Counts, verified:** **103** `#[test]` across **18** files under
`crates/` (was 102). **`lib.rs`'s §Status is FIXED** at last (was stale
four times). `named_color.rs` (Pass 7) still reachable from nothing.
**Pass 6 (performance) is next** and is the first Pass whose rule-8
precondition holds across the whole transform surface.

**Commits, ALL reported — no agent here has ever run git:** `8be1ed3`,
`70411dd`/`a36abaf`, `6ea1b3d`/`812a215`, `46f16e8`, `df3a233`.

Related: [[iccce-free-to-disagree]], [[iccce-direction-scoped-behaviour]],
[[iccce-refusal-discharged-by-fixture]], [[iccce-verify-own-draft-too]],
[[iccce-tolerance-cannot-swallow-and-claim]],
[[iccce-predicted-divergence-must-be-measured]],
[[iccce-gate-must-not-reward-deletion]],
[[iccce-bound-cannot-catch-its-own-magnitude]],
[[iccce-verification-loop-runs-both-ways]], [[ken-terse-scope-decisions]].
