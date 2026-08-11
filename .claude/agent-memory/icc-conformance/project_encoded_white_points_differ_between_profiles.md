---
name: project-encoded-white-points-differ-between-profiles
description: Two profiles' ENCODED gamuts do not nest just because their nominal chromaticities do — colorant sums differ by s15Fixed16 lsb, so round trips get clipped. This falsified a Pass 3 tolerance justification.
metadata:
  type: project
---

**The fact.** A matrix/TRC profile's media white is its **colorant sum**
`M·(1,1,1)`, and each profile's colorants were authored and rounded to
`s15Fixed16` independently. Measured 2026-08-11 from the two Windows
system files:

| | X | Y | Z |
|---|---|---|---|
| sRGB IEC61966-2.1 (HP, 1998) | 0.96427917 | 0.99996948 | 0.82508850 |
| Adobe RGB (1998) (Adobe, 2000) | 0.96420288 | 1.00000000 | 0.82490540 |
| difference | +7.629e-5 | −3.052e-5 | +1.831e-4 |

Those are **5, 2 and 12 units of s15Fixed16's 1/65536 lsb**. Consequently
sRGB's device white lands at **(1.000106, 0.999873, 1.000254)** in Adobe
RGB's *linear* space — outside the cube in two channels — and the
normative Annex F.8–F.16 clamp discards the excess. **25 of 133 grid
points were clipped**, all on high-value faces.

**Why this matters and how it bit.** A Pass 3 round-trip tolerance was
set at 1e-2 ΔE2000 on the reasoning *"sRGB and Adobe RGB share their R and
B primaries and Adobe's G is more saturated, so sRGB ⊂ Adobe RGB, nothing
is clipped, and the only loss is table interpolation."* **Every clause is
true of the two COLOUR SPACES and the conclusion is false of the two
FILES.** The run failed at 1.8788e-2.

**How it was settled** — and this is the reusable part: predict the
round-trip ΔE at white from **the two colorant matrices and the clamp
alone** (no tone curve — every TRC here is exactly 1 at 1; no lcms2; no
measurement). Prediction **1.878244e-2** vs observed **1.878818e-2**:
**0.03% agreement**. Mechanism established, tolerance re-derived to
2.5e-2, and `TOLERANCES.md` §4 logs it as a *corrected justification*, not
a widened number, with both texts kept.

**How to apply:**
1. **Never claim gamut containment from chromaticities alone.** Compute
   `M_dst⁻¹ · M_src · (1,1,1)` and look at it. Cheap, decisive.
2. **A round-trip tolerance between two real profiles is corpus-specific.**
   The dominant term is a property of *which two files*, not of the
   engine. Restating it without the pair is meaningless; a new pair
   re-derives it.
3. **An upper-bound round-trip check rewards deleting the clamp** (the
   round trip gets *better*). Pair it with a check that pins the observed
   cost to the closed-form prediction, and print the sensitivity control.
4. **Scope caveat found the same day:** iccce clamps at **three** sites
   (F.8–F.16 in `pcs_to_device`, 10.18's domain in `Trc::eval`, F.1(b) in
   `eval_inverse`/`invert_table`), so removing the F.8–F.16 clamp *alone*
   is **undetectable** — the clamp-before/clamp-after ordering is
   unobservable at the shipped surface. Recorded as owed.

Related: [[project-oracle-and-tolerance-state]],
[[project-lcms2-findings-pass3-quantisation-and-clamping]].
