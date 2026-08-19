---
name: project-passk-g-cost-of-preservation
description: Pass K §G measured what black preservation COSTS (3.681203 dE2000 on a real cross-press pair, retiring NA-012's UNMEASURED) — and the load-bearing lesson is the SECOND trap: a device-unit separation can be large while the two answers are METAMERS, so the cost is zero for a reason that has nothing to do with the policy and no tolerance can catch it.
metadata:
  type: project
---

**Built 2026-08-19 at tip `2369037`** (measured first at `400179b`; re-run at the
new HEAD, numbers identical). `tools/difftest/src/passk.rs` **§G** (16 rows, 7 in
CI), instrument `src/bin/passk_cost_probe.rs`, operational notes
`tools/difftest/README.md` **§26**, new fixture
`fixtures/synthetic/v2-cmyk-warm-black.icc` + recipe in `tools/gen-profiles`.
Suite `pass=353 fail=0 skip=9 error=0`; fmt/clippy/test all green.

**The number NA-012 was owed:** `3.681203 ΔE2000` max, `1.580674` mean, at
`K = 1.00`, over 101 qualifying points, `ISO Coated v2 300% (ECI) →
GWG_GenericCMYK`, **media-relative**, `k-only-equal-lightness`. Class:
**self-comparison** (iccce preserved vs iccce colorimetric), which is the
ceiling — `ICC_Spec` A51 is a closed negative.

**How to apply.**

- ★★★ **A separation in DEVICE units does not license a ΔE row.** Two
  independent ways a fixture fakes "the policy is nearly free":
  (a) **the reference leg is wrong** — `v2-cmyk-chromatic-neutral`'s `B2A0` is
  not the inverse of its `A2B0` (returns `0.60d` gray under `0.40d` black, read
  back as `0.70d`), so its colorimetric round trip is **`21.218992` ΔE2000**,
  *larger* than the `19.394947` cost it would report; (b) ★ **the two answers
  are METAMERS** — its `K` appears in `L*` and nothing else, so a preserved
  answer at matched lightness has the same `L*a*b*` as the four-ink separation
  **however much ink separates them**. (b) is invisible to every device-unit
  gate §F has, and **no tolerance can catch it** — it is a fixture property.
  Generalisation: *before differencing two answers, gate the leg you difference
  AGAINST.*
- ★★ **`v2-cmyk-warm-black` varies ONE variable** against its sibling: `K`
  carries chroma (`a* += 2K`, `b* += 6K`) and the `CMY` coefficients of `a*`/`b*`
  each sum to zero so the neutral column is **solved** (Cramer, in the recipe)
  rather than chosen. Same darkness coefficients, same dead band. CI cost
  **`5.825550`**, reference leg `0.223097` (the `65280/65535` gap only), and the
  measurement matches a **closed form** derived from the two recipes to
  `6.281370e-3` against a counted `1e-2`. `K′ = k` exactly because both recipes
  carry `0.70` darkness per unit K.
- ★★★ **The injection proof (README §26.8): a 5 % error in `map_k` turns `G12`
  red at 62× its bound and `G8` at 4×; `G3`–`G6` do NOT move (correct — they are
  about the fixture pair), and `G16` stays green because a population row of the
  form "no entitled pair finds it imperceptible" is ONE-SIDED.** ★ `G1`, the
  headline, is `REPORTED` and **can never go red**; its protection is those two
  rows failing beside it.
- ★★ **The population is stronger than the headline**: of **30** ordered pairs
  of six real CMYK profiles, **11 are ENTITLED** (ink separation ≥ `4e-2`, press
  separation ≥ `1.0` ΔE00, reference leg ≤ `1.0` ΔE00) and **0** find the policy
  imperceptible; among them the cost runs `2.023698` – `3.686985`. Without the
  filter, **9 of 30** would read as imperceptible. `G1`'s pair is `5.782e-3`
  below the largest and was chosen for continuity with §A–§E, stated on the row.
- ★★ **The same-press trap is IN the corpus and is not hypothetical.**
  `ISO Coated v2 (ECI) → ISO Coated v2 300% (ECI)` (byte-identical `A2B1`) costs
  **`0.159500`**, 23.1× smaller and imperceptible — the regime NC-244's
  `1.360900e-1` belongs to. Direction matters too: the headline pair reversed
  costs `2.432964`, because the price is a property of the **destination's**
  black.
- ★ **The boundary step is bigger than the cost**: `3.712251` ΔE00 between the
  preserved answer at `(0,0,0,K)` and the preserved-path answer one 8-bit code
  of cyan away. A consumer painting a K-only→rich ramp sees the **step**, not
  the cost.
- ★ **Two rulers, and the gap is disclosed**: lcms2's `A2B1` vs `iccce_cmm`'s own
  model of the same tag differ by up to `0.317063` per point — **above** NC-050's
  borrowed `0.25423` — but the *headline* moves only `6.643135e-3`, because the
  same map is applied to both legs and cancels. Grade the headline's movement,
  not the ramp.
- ★ **A counted bound whose own observation exceeds it is a coincidence, not a
  bound.** `DERIVED_COST`'s first draft counted `6e-3` and the run observed
  `6.28e-3`; the missing term was the `B2A0` **device-output** quantisation
  carried by the `A2B0` chroma coefficients (`1.5e-3`). Corrected by counting,
  not by widening — and the draft error is recorded in the doc comment.

Related: [[project-passk-black-preservation-baseline]],
[[project-passk-f-separating-fixture]], [[project-passk-icc1-is-silent-on-black]],
[[project-prove-the-arm-by-injecting-the-defect]],
[[project-stale-claim-strings-in-emitted-records]],
[[project-parallel-agent-build-collisions]].
