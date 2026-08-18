---
name: project-passk-grading-the-landed-feature
description: Grading black preservation after it landed — the repointing instruction named the predicate rows and MISSED the guard, which could not see the defect its own text claimed it was the only place to see; E2 is measurably unable to discriminate on a same-press pair; and the COMPILED path spreads the preservation over a whole cell at O(1) error.
metadata:
  type: project
---

**2026-08-18, on the engineer's dispatch.** `tools/difftest/src/passk.rs`
§E/§F repointed at `iccce transform --preserve-black k-only-equal-lightness`;
`docs/TOLERANCES.md` **§3.10.12**; `tools/difftest/README.md` **§25.13**; CI
floor `22 → 23`. Pass K **40 → 44 rows** (E7, E8, E9, F8); suite `pass=331 fail=2` →
**`pass=337 fail=0 skip=9 error=0`**; corpus-free (CI-shaped)
`pass=184 fail=0 skip=94`, of which §F contributes **eight**. **No tolerance
was widened** — `E1` and `F5` are still exactly `0`.

**How to apply.**

- ★★★ **ASK WHICH LAYER IS IN THE LOOP OF THE *FIX*.** The pre-feature header
  said *"when the feature lands, `E1` and `E3` must be pointed at the new
  surface"* — the rows about the **predicate**. `E4`, the **regression** row,
  said of itself that a leak *"shows up here and nowhere else in this module"*.
  **That was false of `E4` as written**: preservation is **opt-in and applied
  never by default**, so a row driving the plain surface has *no preservation
  code in its chain to leak* and would have stayed green through any leak
  whatever. A repointing instruction naturally names the rows that must
  *change*; the row that must *not* change is the one that most needs the new
  surface, and is the one it will omit. `E4`/`E5`/`F4`/`F7` now drive the flag.
- ★★★ **THE COMPILED PATH SPREADS THE PRESERVATION OVER A WHOLE CELL, and no
  row anywhere can see it.** `CompiledTransform::new` samples `chain.convert`
  (which preserves) onto a uniform grid, then **interpolates across the
  `C=M=Y=0` discontinuity**. Measured out of tree, ISO Coated v2 300% → itself:
  max `|compiled − reference|` within one cell of the K axis is **`0.617121`
  at grid 17 and `0.617148` at grid 33** — *unchanged* — while the control far
  from the axis halves (`1.138e-3` → `5.34e-4`). **`O(1)` beside `O(h^1.32)`**,
  which is exactly what Pass 6's `R6` band exists to detect. Direction is
  **over-application**: it preserves pixels that do not qualify. Unreachable
  from the CLI (`iccce bench` takes no `--preserve-black`); **reachable from
  the library**, which is where a per-pixel consumer lives. Filed §3.10.12.7,
  **not fixed** — two defensible remedies and choosing is not a conformance
  call.
- ★★★ **A SEPARATION RATIO OF EXACTLY `1.0` IS THE MEASURED FORM OF "THIS ROW
  CANNOT GRADE".** `E2` on a same-press pair: observation `6.1e-5`, rival
  ("copy K through") `6.1e-5`. That is a **stronger** reason to leave it
  REPORTED than the provenance argument the dispatch offered. ★ **The emitted
  verdict is `UNGRADED`, not `BLIND`** — the classifier only reaches `BLIND`
  for a row with a **finite** tolerance. I wrote `blind = 1` into two documents
  and the report falsified it within the hour: *the typed-numeral rule applies
  to typed NOUNS too* ([[project-stale-claim-strings-in-emitted-records]]).
- ★★★ **`E8`: the oracle is wrong and the engine is right, by algebra.** On a
  **same-profile** pair the equal-lightness construction **is the identity**
  for any strictly monotonic ramp — no implementation in the expectation, so
  `DerivedExpectation`, bound = **one printed unit** and nothing else. iccce
  `0.000000`; **lcms2 intent 11 is `6.1e-5` off**, because its K returns
  through a 17-node CLUT. ★ Its rival must be **the oracle's own answer**, not
  "copy K through" — which on this pair *is* correct and would be
  ZERO-SEPARATION.
- ★★ **lcms2 OFF ITS OWN NODES IS NOT LCMS2'S CONSTRUCTION.** Split the ramp by
  `K = m/16` (`_cmsReasonableGridpointsByColorspace` = 17): residual
  `1.4–3.1e-5` at the nodes, up to `1.089 5e-2` off them — **`120×`–`351×`**.
  A whole-ramp agreement figure grades the *vendor's grid density*. `E9` grades
  **only at the nodes**, bound from `pcs_quantum_tolerance` at run time
  (`1.09e-4`), rival `4.890e-2` → **`1577×`**. It is the **only** row that can
  say which definition iccce implements — which is what the mandatory
  `--preserve-black <policy>` argument promises a caller.
- ★★ **AN OBSERVATION THAT DOES NOT MOVE ACROSS THE CHANGE IT WAS WRITTEN TO
  DETECT NEEDS A SECOND NUMBER.** `E3`/`F6` read `0.000000` before *and* after,
  for **opposite** reasons (no K-only output at all → a one-point-wide region).
  Added `cell_zero_chromatic`, the ink at the `C = 0` endpoint. Same family as
  [[project-a-fixed-defect-can-blind-its-own-row]].
- ★★ **A `SelfConsistency` row with an EXACT predicate beats a `CrossCheck`
  with a loose bound.** `E7`/`F8` run the same probes twice through the same
  harness function, flag on/off, graded at **exactly `0`** — the claim is *a
  branch was not taken*, not *these agree to within X*. `F7`'s bound is
  `3.05e-5`, so a leak below that is invisible to the cross-check. ★ `F8` is
  the **only §F row that includes K**, deliberately: a branch not taken leaves
  every channel alone, which needs no answer to `E2`'s fork.
- ★ **The handover was right on all three claims** (exact zero chromatic on ten
  destinations; K genuinely re-mapped; the values). Two things it did not
  cover: **every named refusal** (all five, exit `1` for policy / `2` for
  usage), and one destination where **`K = 1.0 → 0.881462`** — darker than the
  source, so equal lightness lands *below* full ink instead of clamping.
- **Not done:** no injection of a widened qualifying test against `E7`/`F8`
  (needs a `crates/` edit in a detached worktree); no `--bpc` + preservation
  row; `KMapping::Ratio` unimplemented so untestable; the **perceptual** cost
  of preservation unmeasured.

Related: [[project-passk-black-preservation-baseline]],
[[project-passk-f-separating-fixture]],
[[project-prove-the-arm-by-injecting-the-defect]],
[[project-pass6-compiled-path-findings]], [[project-candidate-separation]].
