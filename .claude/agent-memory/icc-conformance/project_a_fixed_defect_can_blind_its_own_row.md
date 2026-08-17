---
name: project-a-fixed-defect-can-blind-its-own-row
description: On iccce, Pass H's one RED row went green when the engineer fixed the abort it found — and the fix ALSO made the row unable to see that defect return, with no number moving and nobody editing the row; the remedy is to re-ask "which layer is in the loop" of the FIX, not just of the row, and to split.
metadata:
  type: project
---

**2026-08-17, same day as Pass H's filing.** `icc-engineer` fixed the defect
`passh/C/7clr/compiled-path-does-not-ABORT-the-process` found in shipped code.
Suite went **`pass=270 fail=1` bare exit 1** → **`pass=274 fail=0 skip=9
error=0` bare exit 0** *(re-measured by me, not taken on report)*.

## ★★★ The finding, which is about ROWS and not about the bug

The row observed *"is the bare exit outside {0, 1}?"*. The fix had **two**
independent parts in `crates/iccce-cmm/src/compiled.rs`:

1. a **SIZE** guard (`ChainError::GridExceedsBudget`,
   `MAX_COMPILED_GRID_BYTES = 64 MiB`) distinct from the `checked_pow`
   **OVERFLOW** guard, converting an abort into a named refusal;
2. `recommended_grid_points`' `_ => 33` catch-all replaced by a value
   **computed** from that budget for ≥5 channels (`7→6`).

**Each part alone makes the row observe 0.** At grid 6 the allocation is 6.4 MiB
and succeeds whether or not the guard exists — so **deleting
`MAX_COMPILED_GRID_BYTES` would have left the row GREEN.** A row that went red on
a real defect had silently become a row that could not see that defect's return.

★ **Nothing detectable happened.** No tolerance moved, nobody edited the row, the
verdict flipped in the direction you want. This is the *opposite* failure mode
from a widened tolerance and it leaves no trace in a change ledger.

## How to apply

- **Ask "which layer is in the loop" of the FIX, not only of the row.** Pass H
  already had that question for rows (injection I2: seven of §D's eight rows were
  blind to a *wiring* defect by construction). Apply it again at every green-ing:
  *if the remedy has n parts, does the row still fail with any one of them
  removed?* If not, the row now proves something narrower than it did.
- **Split, and put a different layer in each row's loop.** Four rows now:
  default survivable (`…does-not-ABORT…`), default *usable* + the library's
  recommendation matching the binary's behaviour
  (`default-grid-BUILDS-and-is-the-grid-the-library-RECOMMENDS`), **the guard
  itself through the CLI** (`oversized-grid-is-a-NAMED-refusal`, which forces
  `--grid 33` — the exact configuration that died), and the reported cost.
- **★ The end-to-end row is NOT redundant with the engineer's unit test.**
  `compiled::tests::oversized_grid_arithmetic_is_refused_not_aborted` asserts the
  guard's arithmetic **in process, deliberately never attempting the
  allocation** — correct, because a test that aborts the test process takes its
  siblings with it. It is therefore blind to the CLI wiring: exit code, stream
  routing, stdout suppression. Same claim, two layers, both needed.
- **★ Guard against a row going VACUOUS.** `oversized-grid-…` counts a violation
  if `MAX_COMPILED_GRID_BYTES` is ever raised above the allocation it forces.
  *A row that has quietly become vacuous is worse than one that fails, because it
  reports PASS.* Interpolate the numbers it matches (`33^7`, `× out_ch × 8`, the
  budget) from the library at run time so it tracks the guard instead of freezing
  yesterday's arithmetic.
- **A defect report with no number is the strongest kind of red.** The observable
  was a bare exit status: there was nothing to widen, so the only route to green
  was fixing the code. Prefer this shape when the subject admits it.
- **The measured 33 for 3-D/4-D was NOT shrunk to fit the byte budget**, and the
  resulting tension (33⁴ × 15 × 8 ≈ 136 MiB exceeds it) is **asserted in a test
  that fails if it ever disappears**. Grade that pattern as correct: the failure
  mode of a documented exception is silent removal with the explaining paragraph
  surviving.

Filed: `docs/TOLERANCES.md` §3.8.4 (rewritten, five subsections) and §4 ledger;
`tools/difftest/README.md` §23.6; apparatus `tools/difftest/src/passh.rs`.

Related: [[project-passh-acceptance-and-refusal]],
[[project-stale-claim-strings-in-emitted-records]],
[[project-prove-the-arm-by-injecting-the-defect]],
[[project-pass6-compiled-path-findings]].
