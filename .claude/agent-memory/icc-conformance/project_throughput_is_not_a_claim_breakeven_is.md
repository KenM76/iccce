---
name: project-throughput-is-not-a-claim-breakeven-is
description: iccce's compiled-path speedup is withdrawn as a documented figure — 10 same-session runs span 12.44x-25.27x on one machine; the break-even pixel count IS quotable but only with its grid, because N = build/(1/ref - 1/comp) and the grid moves only `build`.
metadata:
  type: project
---

**Decision taken 2026-08-12 by this agent**, filed as `docs/TOLERANCES.md`
**§3.6.3** and `tools/difftest/README.md` **§18.4.1**.

**Ten `iccce bench` invocations, one session, one binary (`cc03f3d`), same
pair and raster, five at each grid:**

| grid | build (s) | compiled (Mpix/s) | reference (Mpix/s) | speedup | break-even (px) |
|---|---|---|---|---|---|
| **33 (shipped)** | 12.05–12.91 | 1.18–2.46 | 0.092–0.099 | **12.44–25.27x** | **1.23e6–1.39e6** |
| 17 (previous) | 0.82–0.94 | 1.23–1.36 | 0.092–0.099 | 12.72–14.67x | 8.3e4–9.9e4 |

**The speedup is not a claim this project carries.** 2.03x spread within one
session at a fixed grid; 12.4x–32x across the day. The previously filed
"28–32x, ~10 % run-to-run spread" was the top of a distribution and
understated the variance by an order of magnitude. `iccce bench` still prints
it — a diagnostic the user runs on *their* machine — and no document restates
it as a property of the engine.

**The break-even is, and it must carry its grid.** `N = build ÷ (1/ref −
1/comp)`. `build` is in the numerator and is the **only** term the grid moves:
both throughputs are indistinguishable at 17 and 33. Median build 0.838 →
12.444 s is **14.8x**; median break-even 85 900 → 1 273 800 px is **14.8x** —
agreeing to three figures, so the whole shift is structural and none of it is
load. *A break-even without a grid is like a tolerance without units.*

★ **Why break-even is structurally the stabler statistic:** the compiled path
is >12x faster, so `1/comp ≪ 1/ref` and `N ≈ build × ref_rate` — the noisy arm
barely enters. Over the same five runs the compiled rate spanned **2.08x**
while the break-even spanned **1.13x**. *Publish the quantity that is
insensitive to the arm that varies.*

**Do not read the reference arm as unstable.** Its old recorded band
(0.076–0.091) does not contain today's (0.092–0.099), but within a session it
is the **tightest** quantity measured (±4 %, against ±35 % compiled). The old
band was a four-sample range from one sitting quoted as a property of the
machine — the same error as the speedup.

**How to apply.** Any performance sentence about iccce must name the grid, the
machine, the pair and the number of invocations, and should be a break-even
rather than a ratio. If a ratio is unavoidable, label it *observed on one
Windows box under unknown load, 12.4–32x*.

Related: [[project-pass6-compiled-path-findings]],
[[project-stale-claim-strings-in-emitted-records]].
