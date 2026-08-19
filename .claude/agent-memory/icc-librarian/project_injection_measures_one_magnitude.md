---
name: iccce-injection-measures-one-magnitude
description: DL-064 — an injection that turns a test RED measures it at ONE magnitude and no other; the sweep's GREEN run is where the finding is. Pass K's leak guards proved discriminating above 1.106780e-1 / 5.0e-2 and decorative below; the rival was named WITH a magnitude (1e-9) and the guard claimed WITHOUT one. NC-267/NC-268, 2026-08-19.
metadata:
  type: project
---

**An injection that turns a test red is evidence AT THE MAGNITUDE
INJECTED and at no other.** Filed as `ARCHITECTURE.md` **DL-064**;
`NUMERIC_CLAIMS.md` **§3.35** / **NC-267** (the sweep) and **NC-268**
(the floors).

**Why:** the ledger owed *"prove the leak guards by injection"* since the
Pass K filing — the direct consequence of
[[iccce-documented-is-not-tested]]. `icc-engineer` widened the qualifying
test in `KPreserve::apply` (`black_preserve.rs:404`) from
`C = M = Y = 0` to `max(C,M,Y) <= t` and ran the **full difftest suite at
three magnitudes**, not one:

| `t` | E7 | F8 | `passk` failures |
|---|---|---|---|
| baseline | PASS | PASS | 0 |
| `0.12` | **FAIL `2.620510e-1`** | **FAIL `3.458210e-1`** | 4 |
| `0.10` | PASS | **FAIL `3.458210e-1`** | 3 |
| `0.04` | PASS | PASS | ★★★ **0** |

**The debt was discharged by the red runs. The DEFECT was in the green
one:** at `t = 0.04` the entire suite passes with a broken predicate
compiled in.

> **The red runs discharge the debt; the green run is the measurement of
> the FLOOR.** A passing injection is not a null result — the smallest
> injection that still passes is the most informative run in the sweep.

### ★★★ The mechanism — a rival named WITH a magnitude, a guard claimed WITHOUT one

`TOLERANCES.md` §3.10.12.2 names its rival and **quantifies it**
(*"`10⁻⁹` of cyan"*), then asserts ***"these rows are what would catch
it"*** with **no quantity at all**. The floor is `5.0e-2` — **seven-plus
orders above the rival.** ★★ **Both numbers were on the page and nobody
subtracted them.**

**A justification that names a rival must name the magnitude at which the
guard catches it.** Otherwise it is a hope with a citation attached.

★ This is [[iccce-bound-cannot-catch-its-own-magnitude]] (DL-016) one
level up: there a *tolerance* was blind to a magnitude, here a *probe
set* is. Both fail by never quantifying the instrument itself.

### ★★ STRUCTURAL vs INCIDENTAL floors — the distinction that outlives the numbers

- **F8's `5.0e-2` is STRUCTURAL** — `chromatic_gray_probes` emits
  `c = i × 0.05` with both chromatic ratios `< 1`, so `max(C,M,Y) = c`
  and the minimum is `0.05` **by construction**. Re-derivable by anyone,
  forever. **Verified by this librarian from committed source.**
- **E7's `1.106780e-1` is INCIDENTAL** — a fixed-seed LCG on `[0, 0.8)`
  whose construction bounds the floor only at `≈3.8e-7`. **It is an
  accident of the seed; re-seeding moves the guard's sensitivity with no
  line of intent, comment or tolerance changing.** Not re-derivable by
  reading anything in the repo — `[REPORTED]`, and the enumeration bin
  that produced it was deleted with the worktree.

★ **A regression guard whose sensitivity is a property of a seed is a
guard nobody can reason about.**

### How to apply

- When a dispatch reports *"the injection turned it red, debt
  discharged"* — **ask for the smallest magnitude that still passed.**
  If only one magnitude was run, the floor is unmeasured and the guard's
  reach is unknown.
- **Never quote NC-255/NC-267 without NC-268's floor.** "Proven
  discriminating" is half the claim; "and decorative below `5.0e-2`" is
  the half a future session needs. Rounding up coverage is the standing
  prohibition.
- When a doc names a rival, **check whether the rival carries a number
  and the guard does not.** That asymmetry is the tell.
- ★ Distinguish *the runs bracket the floor* from *the runs measure it*:
  three points gave `E7 ∈ (0.10, 0.12]` and `F8 ∈ (0.04, 0.10]`; the
  exact values came from the **generators**. Two apparatus agreeing is
  corroboration, not a second measurement.
- ★ **A suite-wide "green below X" from ONE point is not an interval** —
  `t = 0.045` was never run ([[iccce-count-from-a-sample-is-not-the-population]]).

Related: [[iccce-documented-is-not-tested]],
[[iccce-bound-cannot-catch-its-own-magnitude]],
[[iccce-disclosure-caught-a-bad-justification]],
[[iccce-gate-must-not-reward-deletion]],
[[iccce-control-only-as-good-as-its-fixture]],
[[iccce-disclosure-is-not-enforcement]],
[[iccce-wrong-clause-refusal-and-discarded-halves]],
[[iccce-pass-status]].
