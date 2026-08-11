---
name: iccce-pass-status
description: iccce status snapshot 2026-08-11 (Pass 3 CLOSED) — first-ever lcms2 cross-check landed (3.4762e-3 dE2000), round trip 1.8788e-2, DL-017/DL-018 filed, Pass 2 still open, Pass 4 groundwork already in tree
metadata:
  type: project
---

**Snapshot of 2026-08-11 (Pass 3 CLOSURE — the seventh filing of one
calendar day). Verify before relying on any of it** — read
`docs/ROADMAP.md` (Pass 3 completion record), `docs/NUMERIC_CLAIMS.md`
§3.8, `docs/NEXT_SESSION.md` and the newest `SESSION_LOG.md` entry.

**★ THE HEADLINE FLIPPED. `iccce` has now been compared to another
implementation.** For four filings the headline was a negative — *"zero
`implementation-cross-check` rows"*. `NUMERIC_CLAIMS.md` **§5.3** retires
that sentence, **dated**, and states exactly what replaced it.

**The two done-when numbers** (quote them WITH scope, never bare):
**iccce vs lcms2, sRGB→AdobeRGB: max 3.4762e-3 ΔE2000** (mean 5.1145e-4),
tol 2e-2, class `implementation-cross-check`; **round trip
sRGB→AdobeRGB→sRGB: max 1.8788e-2 ΔE2000**, tol 2.5e-2, class
`self-consistency`. **Scope: one profile pair, both v2.1, one intent,
one direction, 133 points, one platform, lcms2 pin `21c582a`.** Ledger
rows **NC-034 … NC-043**; next free number **NC-044**.

**Two things that make those numbers unusually strong, and both are
patterns worth reusing:** the cross-check tolerance was **tested by
emulating lcms2's own 16-bit tone-curve quantisation** — residual
collapses ~290× to 2.31e-7, below transicc's print floor; and the
round-trip tolerance **failed at 1e-2 and its DERIVATION was corrected**
(not widened) after the closed-form white-corner prediction matched
observation to **0.03 %**. The driver is a fact about *files*, not
spaces: the two encoded media whites differ by **5/2/12 s15Fixed16
lsb**, and 25/133 grid points clip.

**Commits, ALL reported — no agent in this project has ever run git:**
Pass 0 `f976a0e`, Pass 1 `7313c5b`, Pass 2 b1 `b35a12e`, difftest
`bfd6b1e`, Pass 2 b2 `d40d601`, Pass 3 core `c4038eb`, `transform`
`051707f`, audits `55772c6`, prior filing `a9618fe`, CLUT `fc5ff58`,
PCS encodings `0843094`, absolute intent `6873df1`, differential +
LEGAL §1 `986dae6`.

**New decision-log entries: DL-017** (`tools/difftest` may **path-depend
on iccce's crates** — harness→subject, four conditions, the
no-crate-reaches-lcms2 invariant untouched) and **DL-018** (**an
upper-bound gate on a deliberate cost must be paired with a prediction
pin** + sensitivity control, or deleting the requirement makes the gate
greener; **its scope limit is part of the entry** — the pin does NOT
make the F.8–F.16 clamp *ordering* falsifiable, because iccce clamps at
three sites).

**Three-document boundary, stated so nobody merges them:**
`TOLERANCES.md` §4 owns a tolerance's *number history*;
`NUMERIC_CLAIMS.md` owns the *measured values*; `ARCHITECTURE.md` §5 owns
the *method rule*. One event, three jobs.

**Still open / still true:** **Pass 2 is IN PROGRESS** (clause-2 scope
decision) and it now blocks something concrete — without
`tools/gen-profiles`, every differential row skips off this machine, CI
included. **Absolute intent is implemented but unreachable through the
CLI**, so it has ZERO cross-check evidence (NA-007). **NA-006** — the
A16 n-linear CLUT choice — is **named and NOT measured** (~1 ΔE
corpus-derived bound; tetrahedral deliberately absent until sourced, and
that sourcing is Pass 4's blocker). **Pass 3 does NOT adapt and absolute
intent did not change that** — D.6/D.7 is a diagonal scale, `chad` is
not un-applied, `iccce_color::adapt` is called by no transform, so
NA-002's Bradford cost is STILL not due (checked against code at two
consecutive filings). **Nothing has run on Linux; no CI run observed,
ever.**

**One unresolved discrepancy, recorded as such:** difftest README §13.9
shows `pass=8` over eight check lines; the engineer's re-run reports
`pass=7` with no per-line output. Structurally 1 registered check + 7
pass3 records = 8. *Consistent with* the smoke check not counting — a
hypothesis, not a finding.

Related: [[iccce-verify-own-draft-too]],
[[iccce-bound-cannot-catch-its-own-magnitude]],
[[iccce-predicted-divergence-must-be-measured]],
[[iccce-verification-loop-runs-both-ways]], [[icc1-pdf-operator-blocker]],
[[ken-terse-scope-decisions]].
