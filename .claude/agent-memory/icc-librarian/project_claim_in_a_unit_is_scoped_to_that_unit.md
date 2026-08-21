---
name: iccce-claim-in-a-unit-is-scoped-to-that-unit
description: Pass L (2026-08-20) — "no image changes, ever" and "below one 16-bit PCS quantum" were both TRUE where measured and both FALSE one transform later, because a claim stated in a UNIT is silently scoped to the domain that unit lives in. Also: Pass L's §C is SELF-COMPARISON, not a cross-check, and that weaker class was FORCED by measuring that the oracle cannot see the effect
metadata:
  type: project
---

**A claim stated in a UNIT is silently scoped to the domain that unit
lives in.** *"Below one 16-bit PCS quantum"* and *"no 8-bit code
changes"* name a **unit**, and the unit names a **domain** — and neither
sentence carries the domain with it when it is quoted.

**Why:** `SrgbTrc`'s doc comment (commit `4db44a1`) shipped two
statements, **both correct and narrowly scoped, both written as though
general**. Pass L (`ac921e2`) falsified both:

| as first written | what Pass L measured |
|---|---|
| *"no image changes, ever"* | **FALSE end-to-end** — **14 of 5169** probe points move an 8-bit ink code through `USWebCoatedSWOP`, **11 of 5169** through `AdobeRGB1998`, **2 of 5169** through the committed synthetic (**17** and **6** on a half-step-offset grid ⇒ real, not a grid artefact). **TRUE** of sRGB's own encoding: `0` of 256 codes, curves `9.76e-6` apart encoded |
| *"below one 16-bit PCS quantum"* | **FALSE as ΔE** — PCS max **`1.857907e-3`** ΔE2000 = **`2.01×`** one 16-bit `L*` quantum. **TRUE** in the encoded domain |

★★ **The mechanism is NOT amplification.** The device separation is at
most **`1/62` of an 8-bit code**. It is that a difference that small
**still flips a code whenever the value straddles a rounding boundary**,
which ≈`0.3 %` of points do. **Zero-in-its-own-encoding does not survive
a second transform.**

**How to apply:**

- **When a doc comment states a bound in a unit, write the domain into
  the same sentence.** *"Below one 16-bit PCS quantum **in the encoded
  domain**"* would never have been falsified.
- ★★ **Evaluate a quantum AT THE ARGMAX, never at a convenient point.**
  ΔE2000's `SL` term varies by `1.6×` between `L*` 6 and `L*` 50, so a
  quantum quoted at the wrong lightness gives whatever ratio the author
  wants. `passl.rs` bumps `L*` by `100/65535` **at the maximum's own
  point** and says why.
- **Correct in place, keep the original beside it.** *"Both overclaims
  are corrected here rather than quietly amended, because I made them and
  then repeated them to the operator."*
- ★ **Honest summary form:** *a curiosity, but NOT an invisibility* —
  `538×` below perceptibility **and it still moves 8-bit ink codes**,
  which is how it shows up in somebody else's regression suite.

**★★★ THE SECOND LESSON, and it corrected a dispatch: only Pass L's §A is
a cross-check.** The dispatch said *"every Pass L row's oracle is
cross-check against lcms2"*. **Every §C record is
`Kind::SelfConsistency`** — iccce against iccce, in process, in `f64`.
So **`14`/`11`/`2` of `5169` are one class WEAKER than a cross-check**,
and writing them otherwise would attribute to the oracle a measurement it
**provably cannot make**.

★★ **And the weaker class was FORCED BY A MEASUREMENT, not chosen for
convenience** — which is the part worth reusing. lcms2 evaluates an
`mft2` CLUT inside a float pipeline through `EvaluateCLUTfloatIn16`,
whose **first act quantises the float input to 16 bits**. A control sweep
returned **7 distinct `L*` values from 60 samples** — a staircase whose
tread is ≈`4.9e-3` % ink against a maximum effect of `6.36e-3` %.
**The oracle is blind to the quantity by roughly the size of the
quantity.** ★ A section that merely announced *"measured in process"*
would look like an unexplained weakening; **the row that proves the
oracle cannot see it is what makes the self-comparison defensible.**

★ **§A does not exercise iccce's transform code either.** The harness
evaluates both curves and transcribes lcms2's own `f()`; iccce supplies
only the **constants**. So §A cross-checks **constants and curve form**,
never `MatrixTrc` or `Chain` — and it inherits `oracle-behaviour-at-pin`'s
property that **the pin moving invalidates every row**.

**★★ Two probe-design facts from the same Pass, both general:**

- **The obvious breakpoint probe has EXACTLY ZERO power, structurally.**
  H.273 clause 8.2 defines `β` by continuity with the **same linear
  segment both readings share**, so the curves meet there **by
  derivation**. Third time in this project the obvious breakpoint probe
  has been measured powerless.
- ★★ **The maximum is INTERIOR and in a DIFFERENT PLACE per instrument** —
  the `Lab` and linear-light maxima are **119 codes apart**, and the
  linear-light one (*which the crate's own doc comment prints*) throws
  away **64 %** of the `L*` signal. ★ **A gray ramp understates the ΔE
  cost by `2.51×`** because the true max is **off-axis**, at codes
  `(10.0213, 23.7681, 10.0213)`: ΔE2000's chroma and hue terms put the
  worst case where a 1-D probe cannot reach.

★ **`A57` STAYS OPEN.** lcms2 implements the **C⁰** reading
(`5.300706e-5` vs `1.230354e-3` `L*`, `23.2×`, `0 of 204` probes favour
C¹) — **a fact about an implementation, not about what sRGB is.** Two
in-force standards disagree (ICC/W3C/Khronos vs H.273 cl. 8.2 for
`TransferCharacteristics = 13`), and **IEC 61966-2-1 is paywalled and
unobtained**.

Related: [[iccce-words-humans-count-code]] (DL-063 — this is its shape
applied to a unit instead of a count),
[[iccce-bound-cannot-catch-its-own-magnitude]],
[[iccce-agreement-can-be-the-symptom]],
[[iccce-compatibility-not-certification]],
[[iccce-fabricated-value-is-a-forged-credential]],
[[iccce-pass-status]].
