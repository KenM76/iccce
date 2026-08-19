---
name: iccce-gate-in-input-units-cannot-certify-output
description: DL-065 — a separation gate stated in DEVICE units cannot certify a separation in COLOUR; a spectrally neutral black makes the preserved answer a METAMER, so the fixture lays 0.42 of ink, clears every gate, and reports a cost of zero. No tolerance can express it; the remedy is a fixture
metadata:
  type: project
---

**DL-065 (2026-08-19, Pass K §G): a gate expressed in the units of the
transform's INPUT cannot certify a property of its OUTPUT. The model
between them may annihilate exactly the quantity being measured —
silently, with the gate green.**

**The instance.** `fixtures/synthetic/v2-cmyk-chromatic-neutral.icc` was
about to be used to price black preservation in ΔE2000. Its **black ink
is spectrally neutral** — `K` appears in `L*` and in nothing else — so
the preserved answer at matched lightness has the **same `L*a*b*`** as
the four-ink separation it replaced. The two answers are
**colorimetrically identical however much ink separates them.** The
fixture separates by **`0.420705`** of chromatic ink, **21× the declared
`4.0e-2` floor**, and would still have reported a cost of **zero**.

**Why it is NOT the zero-separation trap already on file.** Pass K
already records that a *zero*-separation fixture manufactures a false
pass, remedied by **NC-259**, a floor declared in advance in device
units. **That remedy is sound and is untouched.** This is its sibling:

| | zero-separation trap | ★ metamer trap |
|---|---|---|
| the fixture lays | **no** chromatic ink | **plenty** of chromatic ink |
| the device-unit gate reads | `0` — visibly broken | **healthy, `21×` the floor** |
| the measured ΔE | collapses to `0` | collapses to `0` |
| detected by | NC-259 | ★★★ **nothing in this project** |
| class | a **bound** problem | ★★★ a **fixture-model** problem |

**The remedy was a fixture, not a number** — DL-020's shape again.
`v2-cmyk-warm-black` varies **exactly one variable** against its sibling
(the black carries chroma, `a* += 2K`, `b* += 6K`); darkness
coefficients, dead band and grid sizes unchanged, **so the `5.825550`
ΔE2000 it reports is attributable to that one variable**. A supporting
change — the `CMY` coefficients of `a*` and `b*` each summing to zero —
exists so the neutral column can be *solved* rather than chosen, which is
what makes an exact inverse expressible.

**★ It was found BEFORE a number was published**, by measuring the
fixture's *fitness* instead of its output. **No ledger row was ever wrong
because of it**, and nothing failed — which is exactly why it needed
writing down.

**★★ The second disqualification, which must not be conflated with the
first.** The same fixture's `B2A0` is also not the inverse of its `A2B0`
(round trip **`21.218992`**, larger than the `19.394947` it would have
reported) — and `G5`'s reference-leg bound **catches that**. A reader who
takes *"the fixture was replaced because its round trip was bad"* has
taken the half a tolerance already handles and dropped the half that
nothing does. **Fixing the inverse alone would have left the metamer trap
fully intact, with every gate green.**

**How to apply.** Before pricing anything in ΔE with a fixture, ask
**what the fixture's MODEL does to the quantity**, not just whether its
separation gate passes: *if the effect were identically absent, would
this gate still read healthy?* Device-unit gates are the right choice for
leak detection (they are what a leak is measured in, and Pass K's
`0`-tolerance rows depend on it) — **they are necessary and never
sufficient for a colour claim.** Any recipe whose black appears only in
`L*` is the easiest black to write and the one that nullifies this class
of measurement.

Related: [[iccce-pass-status]],
[[iccce-control-only-as-good-as-its-fixture]],
[[iccce-refusal-discharged-by-fixture]],
[[iccce-unfailable-row-protected-only-by-siblings]],
[[iccce-measurement-under-the-guards]],
[[iccce-documented-is-not-tested]].
