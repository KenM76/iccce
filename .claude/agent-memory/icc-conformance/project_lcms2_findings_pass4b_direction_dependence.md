---
name: project-lcms2-findings-pass4b-direction-dependence
description: Three measured facts about lcms2 2.19.1 from Pass 4b — it forces TRILINEAR for any Lab-PCS output LUT (so NA-006's cost is zero in the B2A direction), its forced BPC is decided by the DESTINATION profile's version, and its sRGB inverse TRC is a 4096-entry u16 resampling that accounts for 100% of the gray residual.
metadata:
  type: project
---

All measured **2026-08-11** at pin `21c582a` during Pass 4b (the B2A `lut8`
direction, the v4 `mAB `/`mBA ` fixture, and the F.2 grayTRC model). Full
record: `tools/difftest/README.md` **§15**; tolerances in `docs/TOLERANCES.md`
**§3.4.4**; lessons in **§6.4** there.

**The unifying lesson: lcms2's behaviour is DIRECTION-dependent, and three
separate recorded findings turn out to be half-rules because nobody had
measured the other direction.**

**1. lcms2 forces TRILINEAR interpolation for any Lab-PCS output LUT.**
`cmsio1.c` `_cmsReadOutputLUT`: `if (cmsGetPCS(hProfile) == cmsSigLabData)
ChangeInterpolationToTrilinear(Lut);` — sets `CMS_LERP_FLAGS_TRILINEAR` on
*every* CLUT stage, bypassing `case 3`'s tetrahedral. Trilinear over 3 inputs
**is** iccce's n-linear. So:

- **NA-006's cost is ~1.6 ΔE2000 in the A2B direction and identically ZERO in
  the B2A direction.** Any statement of that cost that omits the direction is
  meaningless. The Pass 4 memory's "the 4-D scheme is a hybrid" is about
  `_cmsReadInputLUT` only.
- Measured: `sRGB → USWebCoatedSWOP` (`mft1` B2A, 33³, 8-bit) agrees to
  **1.33×10⁻⁴ device**, and the **tetrahedral counterfactual is 99–139× that**
  — run as the sensitivity control, so "the geometries agree" is not a claim
  about a comparison that could not tell.
- It is a *policy* (lcms2's own comment calls it "controversial stuff"); ICC.1
  is silent (A16). Agreement here is between two choices, not conformance.

**2. Forced BPC (DL-013 / corpus M2) is decided by the DESTINATION profile's
version.** `_cmsLinkProfiles` sets `BPC[i]` per profile, but
`DefaultICCintents` consumes it as `ComputeConversion(i, …, BPC[i], …)` — the
conversion **into** `hProfiles[i]`. Measured on one pair, both ways: v4 fixture
as **source** into a v2 destination is **bit-identical** between perceptual and
media-relative (the flag is set and never read); v2 source into the v4 fixture
as **destination** moves `K` at black 99.6094 % → 96.4721 %. **M2 as written
would mislead anyone using it to decide whether a comparison is confounded.**

**3. lcms2's sRGB destination inverse TRC is a 4096-entry `u16` resampling**
(`BuildRGBOutputMatrixShaper` → `cmsReverseToneCurve` →
`cmsReverseToneCurveEx(4096, ·)`), chorded between the forward table's 1024
knots and then evaluated through the float path that rounds input and output to
1/65535. Reimplementing it turns the gray-axis residual from an observation into
a reproduction: **9.686×10⁻⁵ device → 2.121×10⁻⁷, a 457× collapse, below
`transicc`'s print floor.** This is the term that dominates **any** comparison
whose destination is a matrix/TRC profile with tabulated TRCs — it is why
`fixture → sRGB` needed a different tolerance from `sRGB → fixture`.

**4. One divergence found, and it is iccce's clamp against lcms2's:** at the
`mAB ` fixture's `K = 0` the 3×4 matrix's `+1/256` offset puts the encoded `L*`
at **1.00390625**. iccce clamps (clause 10.18's curve domain, in `Trc::eval`) →
`L* = 100`; lcms2 does not (a `count=0` `curv` is a γ=1 parametric segment,
domain ±10²²) → `L* = 100.390625`. **Cost 0.61 ΔE2000.** UNSETTLED —
REPORTED, NOT GRADED, question written verbatim in README §15.3.3, dispatch to
`icc-spec-librarian` **OWED** (not made: a librarian task was already running in
the corpus tree).

**Method lessons that earned their keep (also in TOLERANCES §6.4):**

- **A tolerance of `0.0` is only available when the two sides are the same
  operations in the same order.** "Every geometry reproduces an affine function
  exactly" is true in ℝ and false in `f64` (~16 ulp). A derivation ending in
  *exactly* must say algebra or arithmetic.
- **Where in the grid the maximum sits is part of the derivation.** A ΔE
  tolerance derived at white failed at black by 2.2×: below sRGB's linear
  breakpoint a *device* difference is amplified into `a*`/`b*` by
  `4038/12.92 = 313` versus 69.9 for `L*`, so the max is **chromatic and near
  black even on a neutral axis** — the mirror image of Pass 3's "near black the
  device metric explodes while ΔE stays small".
- **Two comparisons sharing a fixture do not share a tolerance.** `sRGB →
  fixture` ends at a CLUT; `fixture → sRGB` ends at an inverse TRC. Same files,
  budgets an order of magnitude apart.
- **Writing a guessed envelope into a tolerance's `why` before computing it is
  a trap**, even when the number is later corrected: three of Pass 4b's five
  first-draft constants had placeholder envelopes and two of them failed.
  Compute the envelope first, then write the constant.

Related: [[project-lcms2-findings-pass4-interpolation-and-v2-wtpt]],
[[project-lcms2-findings-pass3-quantisation-and-clamping]],
[[project-lcms2-findings-legacy-lab-and-forced-bpc]],
[[project-oracle-and-tolerance-state]].
