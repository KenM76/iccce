---
name: project-pass5c-estimator-branch-finding
description: Pass 5c — lcms2 has TWO black-point estimators at media-relative and the DESTINATION'S HEADER picks between them, which overturned Pass 5b's claim-1 verdict; the reimplementation removed the error bar; and transicc prints RGB 0..255 while CMYK is 0..100.
metadata:
  type: project
---

> ★★ **RE-MEASURED 2026-08-12 on the corrected 4.2.5.4 code — the `swop`
> divergence is now 4,799 109 ΔE76, not 8,166 8×10⁻², and the reason is a
> finding rather than a fix. See
> [[project-conformance-can-worsen-the-crosscheck]].**

Measured **2026-08-12** at oracle pin `21c582a`, iccce commit `95c04c1`.
Apparatus `tools/difftest/src/pass5c.rs`; record `tools/difftest/README.md`
**§19**; tolerances `docs/TOLERANCES.md` **§3.5.8**. Two arms:
`USWebCoatedSWOP.icc` and the new
`fixtures/synthetic/v4-rgb-mab-chromatic-black.icc`.

**★★★ lcms2 has TWO black-point estimators at relative colorimetric, and which
one runs is decided by the destination's DEVICE CLASS and COLOUR SPACE**
(`cmssamp.c` L370–374): output-class **and** an ink space →
`BlackPointUsingPerceptualBlack`, which **forces `a* = b* = 0`** at L174;
anything else → `BlackPointAsDarkerColorant`, which **keeps the chroma**.
`cmsDetectDestinationBlackPoint` then returns `Lab.a = InitialLab.a` (L590), so
**the branch IS the returned chroma**. "Does lcms2 keep its black's chroma?"
has no single answer.

| arm | destination | ISO black | lcms2 black | divergence |
|---|---|---|---|---|
| `swop` | v2.1 `prtr` **CMYK** | L\* 16,4898 neutral | L\* 16,5715 **neutral** | 0,0817 ΔE76, **100 % L\*** |
| `synthetic` | v4.4 `prtr` **RGB** | L\* 20 neutral | Lab(20 · 4 · −3) | 5,0000 ΔE76, **100 % chroma** |

**★★ This overturned Pass 5b.** 98,3 % of its 0,858 ΔE76 was the
`A2B1 ∘ B2A1` **recovery**; claim 1's "CONFIRMED" is **withdrawn** on the swop
arm and claim 3's "NOT ESTABLISHED" — the verdict Pass 5b declined to assert —
was right and is now settled both ways. **The error bar Pass 5b reported at
0,948 was not an error bar; it was the measurement.**

**★ Neither implementation fits a quadratic on either fixture.** Both take the
mid-range straightness short-circuit; Pass 5b's "precisely lcms2's method-4
territory" is wrong. What they disagree about is **what the short-circuit
returns**: lcms2 `InitialLab` (L536, from the *perceptual* round trip), ISO
`outRamp[first]` (from the *relative* one).

**★★ The apparatus trick that removed the bar, worth reusing.** BPC's second
constraint sends the source black *exactly* to the destination black, and sRGB's
black is `XYZ(0,0,0)` — so the device values any implementation emits at input
black **ARE `B2A1(its own detected black)`**. A candidate black can therefore be
predicted **forward** into device space and compared with `transicc` directly:
no recovery, no inversion. Residual 4,22×10⁻⁴ (CMYK) / 8,94×10⁻⁶ (RGB) against
rivals that miss by 5,8× / 6 400×.

**★★ THE HARNESS BUG THIS CAUGHT: `transicc` prints ink spaces `0..100` and
RGB/gray `0..255`.** Passes 5/5b/5c all divided by 100 because every
destination in reach had been CMYK. The first synthetic run reported a
9,98×10⁻² residual where the truth is 8,9×10⁻⁶. **It was visible only because
§B carries a second hypothesis** — both candidates missed by the same amount.
*A residual that is large under every hypothesis is an apparatus fault, not a
finding.*

**★ The v4 perceptual arm CANNOT be discriminated by any fixture** — Pass 5's
§16.8 item 4 asked for an impossible instrument. At perceptual/saturation on a
v4 profile both implementations return the fixed A41 constant **without reading
the profile** (`cmssamp.c` L432–446). What the new fixture does instead is
discriminate the **media-relative** arm on a **non-ink** destination. Measuring
the A41 constant's error against its real black (L\* 3,1 assumed vs L\* 20
actual) is **owed**.

**Pass 6 re-graded at the new default grid 33: the gate PASSES** — 1,677×10⁻¹
and 9,349×10⁻² against the unchanged 2,5×10⁻¹. Two things the transition added:
`pass6.rs`'s `DEFAULT_GRID` must track `recommended_grid_points` (the apparatus
row caught the drift by failing at 1,576×10⁻³), and at grid 33 the two probe
populations **stop agreeing** (1,79× apart), so quoting either alone is now a
population claim. Build cost ~14 s moves `iccce bench`'s break-even from
≈70 000 px to **≈1,19 M px**.

Related: [[project-pass5b-estimator-findings]],
[[project-pass6-compiled-path-findings]],
[[project-synthetic-fixture-corpus-and-gp001]],
[[project-oracle-and-tolerance-state]].
