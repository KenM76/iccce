---
name: project-pass6-compiled-path-findings
description: Pass 6 findings — the compiled path's cost is 0.297 dE2000 at the shipped grid 17, ABOVE the 0.253 iccce-vs-lcms2 line (suite deliberately RED); its convergence order is 1.32 not 2; and max-of-max is the wrong estimator for an h-squared control.
metadata:
  type: project
---

Measured **2026-08-12** at pin `21c582a`, iccce commit `3502cb7`/`5867f1a`.
Apparatus `tools/difftest/src/pass6.rs`; record `tools/difftest/README.md`
**§18**; tolerances `docs/TOLERANCES.md` **§3.6**. Pair: `USWebCoatedSWOP A2B1`
(`mft2`, 4-D, 9 CLUT nodes/axis) → system sRGB, media-relative, on `iccce
bench`'s own 513 sampled probes.

**★★ RESOLVED 2026-08-12 (same day).** Commit `189e732` moved `recommended_grid_points` from 17 to **33** for 3-D and 4-D. **Both gate rows now PASS — 1,677×10⁻¹ (bench probes) and 9,349×10⁻² (Pass 4 grid) — against the IDENTICAL 2,5×10⁻¹.** Two things the transition added: `pass6.rs`'s `DEFAULT_GRID` must track the shipped default (the apparatus row caught the drift by failing at 1,576×10⁻³, which is the gap between the grids, not an error), and at 33 the two probe populations **stop agreeing** — 1,79× apart — so quoting either alone is now a population claim. The 14 s build moves `iccce bench`'s break-even from ≈70 000 px to **≈1,19 M px**. See [[project-pass5c-estimator-branch-finding]].

**★★ THE GATE FAILS AND THE SUITE IS DELIBERATELY RED.** Tolerance
**2,5×10⁻¹ ΔE2000** = Pass 4's measured iccce-vs-lcms2 figure on this exact pair
(0,252 94), to one significant figure, **no free parameter**. Observed
**2,970 17×10⁻¹** on the bench probes and **2,962 90×10⁻¹** on Pass 4's own
341-point grid — the same within 0,25 %, so it is a property of the transform,
not of a probe set. **The remedy is the grid (33 measures 0,167 7), not the
number.** Do NOT widen it: it is Pass 4's number and has nothing in it to move.

**The device figure `iccce bench` prints looks negligible and is not.**
3,588 962×10⁻³ device carries to 17 % *above the entire
implementation-to-implementation spread*. That is the whole reason a device
number is not a colour claim.

**★★ The convergence order is 1,32, not 2.** Paired-median `err(coarse)/err(fine)`
**2,69 / 2,47 / 2,51** over 5→9→17→33, stable to ~1 %. Doubling the grid costs
**~15× the build** and buys **~2,5×**, not 4×. Cause: SWOP's `mft2` has
**256-entry input tables interpolated linearly** = 255 kinks per axis at `k/255`,
and `gcd(255, N) = 1` for `N ∈ {4,8,16,32}`, so **no grid in reach aligns with
them**.

**★ Two wrong instruments were caught, and both are on file:**

1. **Max-of-max is the wrong estimator for an h² control.** It wanders
   **5,57 → 1,39 → 1,78** where the paired median sits at 2,5. `h²` is about a
   *fixed point*; two maxima need not be at the same probe. `compiled.rs`'s own
   unit test uses max-of-max over 7 probes — its `[2,8]` band passed by luck of
   the fixture.
2. **A clamp/breakpoint attribution was written, tested and FALSIFIED.**
   Restricting to cells whose 16 corners are all in gamut and above sRGB's
   0,040 45 changed the ratios **not at all** (448/513 probes qualify). The
   hypothesis was good and wrong.

**The band was re-derived to assert only order ∈ [1,3]** (ratio 2–8): below 1 the
error is not grid-driven (ratio → 1 is the identity-chain trap), above 3 is
impossible for multilinear interpolation.

**Throughput is REPORTED and graded nowhere**: 2,4–2,7 Mpix/s vs 0,076–0,091
reference = 28–32× on this machine, ~10 % run-to-run spread across four
invocations, and ~2× from the engineer's 1,20 Mpix/s / 14,4× in an earlier
session. The stable figure is **break-even ≈63 000–75 000 px**.

> ★★ **SUPERSEDED 2026-08-12 — see [[project-throughput-is-not-a-claim-breakeven-is]].**
> Every figure in the paragraph above was measured at the **old default grid of
> 17**. The speedup is **withdrawn** (12,44–25,27× over ten same-session runs;
> the "~10 %" understated the variance by an order of magnitude), and the
> break-even at the shipped grid 33 is **≈1,3×10⁶ px**, 14,8× larger — entirely
> because build went 0,838 s → 12,444 s.

**Method lesson worth carrying:** *the same physical event has a different size
in two units, and the unit the requirement is stated in is the one that may
carry the tolerance.* The device row is ungraded because the bound its ΔE
sibling implies (2,5e-1 ÷ 136, sRGB's shadow sensitivity) is **tighter than the
observation** while the observed maximum is a **midtone**.

Related: [[project-oracle-and-tolerance-state]],
[[project-lcms2-findings-pass4-interpolation-and-v2-wtpt]].
