---
name: iccce-pass-status
description: iccce status snapshot 2026-08-12 (ORIGINAL SCOPE COMPLETE — Passes 0–7 all DONE and filed; 52 commits, 9 pushes; next free NC-158; DL-029 is the newest decision) — what remains is Pass 8 in pdfce, four purchased/licensed documents, Passes 9–10, and standing debts
metadata:
  type: project
---

**Snapshot of 2026-08-12 (the estimator-discrimination filing — the
fourteenth overall, the FIFTH of the second calendar day). Verify before
relying on any of it** — read `docs/NEXT_SESSION.md`, then
`docs/ROADMAP.md`'s latest header block, `docs/NUMERIC_CLAIMS.md`
**§2.11 + §3.18 + §7.11**, newest `SESSION_LOG.md`.

**★★★ THE ORIGINAL SCOPE IS COMPLETE. Passes 0–7 are all DONE and
filed.** Pass 4 closed at the previous filing; Pass 5's one stated
boundary (the estimators) closed at this one; Pass 6's gate now **passes**
at a new default grid. **What remains is four kinds of thing and only one
is a task list**: (1) **Pass 8 — built in `pdfce`, not here**;
(2) blocked on documents nobody here can produce (**IEC 61966-2-1**,
`ICC.1:2010-12` for **A31**, ICC's `chad` values, ITU-R for Pass 9);
(3) **Passes 9/10** — operator scope calls, never sized, and Pass 10's
precondition (a ground truth that is not iccce) is unsolved;
(4) standing debts.

**★★★ THE DAY'S FINDING (DL-027): lcms2 has TWO black-point estimators
at media-relative**, selected by the **destination's device class +
colour space** (`cmssamp.c` L370–374) — ink+output → forces chroma to 0;
everything else → keeps it. Measured: SWOP (v2 `prtr` CMYK) **8,1668e-2
ΔE76, 100 % `L*`**; a new v4 RGB fixture **5,000000 ΔE76, 100 % chroma,
ΔL* exactly 0**. **The pre-registered prediction resolves in OPPOSITE
directions on the two arms** — *a session that ran only one arm would
have filed a confident wrong headline either way*. See
[[iccce-direction-scoped-behaviour]].

**★★ Pass 5b was 98,3 % apparatus**; its Q3 verdict WITHDRAWN, Q2
superseded, Q5's "not established" vindicated. **Three graded rows
inverted, none deleted.** Its error bar (0,8137 vs an effect of 0,85817,
ratio 0,948 reported as *marginal*) **was the measurement** — see
[[iccce-apparatus-fault-under-every-hypothesis]] (DL-028).

**★★ Pass 6 gate PASSES at grid 33** — 0,16773 (513 bench probes) and
0,093486 (Pass 4's 341 pts) against an **unchanged 2,5e-1**; the engineer
moved `recommended_grid_points` 17→33 (`189e732`). Carry two riders: the
two probe populations **stop agreeing** at 33 (1,79× apart), and
break-even moves **≈70 k → ≈1,19 M px**. **NC-105…NC-108 describe grid
17.**

**Also landed:** ΔE94 + ΔE CMC (`impl_crosscheck`, C probe against the
pinned lcms2, ten decimals, three pairs — **NOT ground truth**; a test
pins CMC's **asymmetry**); the ISO estimator **wired to a caller** (it
had none — *an unused capability is not a feature and not a measurement
either*); **four API soundness defects** incl. a **stale-inverse hazard
on a public field**; **DL-029** the API sealing split (*seal what decodes
our format, publish what implements someone else's specification*); and
**CI reported to have run and passed** — a **report**, with no run URL
and no statement of whether the Linux job ran, so **it does not discharge
the Linux debt**.

**★★★ ONE OPEN QUESTION CAN MAKE SHIPPED CODE WRONG.** At ISO/CD 18619
**4.2.5.4**'s short-circuit (both implementations take it; **neither
fits a quadratic on either fixture**), **iccce returns `outRamp[first]`,
lcms2 returns `InitialLab`** — that difference **is** the whole SWOP
divergence. **Dispatched to `icc-spec-librarian` 2026-08-12; unanswered.
If ISO names lcms2's, iccce is WRONG, not divergent.**

**Ledger: NC-129 … NC-157 filed; next free NC-158.** DL-027/028/029
filed; **29 decision-log entries.**

**Repository, verified from `.git/` plain-text files 2026-08-12:**
`HEAD` = `origin/master` = **`5cfee171`**; **52 reflog lines, no rewrite
entries** (agrees with the dispatch's 52 — the first time a dispatch
count and the file agree); **NINE pushes**, last 09:06:55 −04:00.
**Nothing records a go-ahead for pushes 3–9.** One push **reported** to
have failed with HTTP 408 and been retried over HTTP/1.1 — **a failed
push leaves no reflog line.**

**Counts, verified 2026-08-12:** **121 `#[test]` declarations across 19
files** under `crates/` (116 at the previous-but-one filing); **39
`.icc`** in `fixtures/synthetic/` (the new one is
`v4-rgb-mab-chromatic-black.icc`). **The last reported suite outcome is
`exit 0, 121 passed` at commit `95c04c1` — the matching 121s are
DIFFERENT QUANTITIES.** **No runner outcome exists for the shape of
`pass5c`/`pass6` filed today.**

**Holes that outlasted every Pass:** **zero `published-ground-truth`
rows for ANY transform** (ninth filing; IEC 61966-2-1 is the cheap route
and nobody has dispatched), and **no Linux run of anything, by anyone,
ever**. Plus `dechk.obj` still at the repo root (published), and commit
hygiene — **three bare-pathspec commits swept in other agents' unfinished
work in two days**.

Related: [[iccce-git-files-readable-without-shell]],
[[iccce-apparatus-fault-under-every-hypothesis]],
[[iccce-direction-scoped-behaviour]],
[[iccce-control-only-as-good-as-its-fixture]],
[[iccce-free-to-disagree]], [[iccce-refusal-discharged-by-fixture]],
[[iccce-verify-own-draft-too]],
[[iccce-tolerance-cannot-swallow-and-claim]],
[[iccce-predicted-divergence-must-be-measured]],
[[iccce-gate-must-not-reward-deletion]],
[[iccce-bound-cannot-catch-its-own-magnitude]],
[[iccce-verification-loop-runs-both-ways]], [[ken-terse-scope-decisions]].
