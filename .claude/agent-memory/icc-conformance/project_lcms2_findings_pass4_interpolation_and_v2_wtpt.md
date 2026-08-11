---
name: project-lcms2-findings-pass4-interpolation-and-v2-wtpt
description: Two measured facts about lcms2 2.19.1 from Pass 4 — its 4-D CLUT scheme is a hybrid (linear in C x tetrahedral in M/Y/K) evaluated in 16-bit fixed point, and it substitutes D50 for the wtpt of a v2 DISPLAY profile, which costs 11 dE2000 at the absolute intent.
metadata:
  type: project
---

Both measured **2026-08-11** at pin `21c582a` during the Pass 4 CMYK→RGB
differential (`USWebCoatedSWOP.icc` → Windows system sRGB, 341 points, all
four intents). Full record: `tools/difftest/README.md` **§14**; tolerances in
`docs/TOLERANCES.md` §3.4.

**1. lcms2's 4-D CLUT interpolation is NOT tetrahedral — it is a hybrid, and
it is not symmetric in the inks.** `cmsintrp.c`, `Eval4Inputs` /
`Eval4InputsFloat`: **linear along input channel 0**, **Sakamoto tetrahedral in
channels 1–3**, the two 3-D results blended by channel 0's fraction. Pure
tetrahedral is the *3-input* case only. Consequences:

- A tolerance transcribed from published trilinear-vs-tetrahedral figures is
  derived from **the wrong algorithm** for any CMYK profile.
- **The float path does not use the float interpolator.** An `mft2` tag becomes
  a 16-bit CLUT stage (`cmsStageAllocCLut16bitGranular`), whose float evaluator
  `EvaluateCLUTfloatIn16` quantises the input to `u16` and calls the
  **fixed-point** twin. So CMYK carries 16-bit quantisation at the CLUT
  boundary *as well as* in the tone curves (the Pass 3 finding).
- **Index conventions differ at the top of each axis**: lcms2 takes
  `k0 = floor(pk)` unclamped (`points−1` at 1.0, `rest = 0`) and collapses the
  upper node; iccce clamps the index to `points−2` and lets the fraction reach
  1.0. Each is correct **with its own upper-node rule** and catastrophically
  wrong when mixed.

**Measured cost of the method difference (this is NA-006, now measured):** max
**1.5741 ΔE2000** on SWOP's `A2B0`, **0.254 23** on its `A2B1` — the two tables
in one file differ by **6×**. Emulating lcms2's geometry in the harness
collapses the iccce-vs-lcms2 residual by **326× / 55×** to ~4.8×10⁻³ ΔE00.

**2. lcms2 substitutes D50 for the `wtpt` of a v2 DISPLAY-class profile.**
`cmsio1.c` `_cmsReadMediaWhitePoint`: if `version < 0x4000000` and class is
`mntr`, return `cmsD50_XYZ()` **whatever the tag says**. The Windows system
sRGB profile's `wtpt` holds **D65** (0.950455, 1.0, 1.089050). iccce uses
`wtpt` as stored (NA-007). At ICC-absolute both apply the same D.6/D.7
diagonal `WPin/WPout` — with different `WPout`, differing by `D65/D50` =
(0.9858, 1.0, 1.3202), **a 32 % error in Z**. Measured **11.217 ΔE2000 max,
4.67 mean**; re-predicting lcms2 with that one substitution collapses it
**517×** to 2.17×10⁻². **Which is right is corpus A4b, UNVERIFIED** —
ICC.1:2001-04 not obtained. Dispatch to `icc-spec-librarian` is **owed**.

**This is the third time in two passes that a version- or class-gated lcms2
policy has silently changed what a cross-check was measuring** (forced BPC on
v4 perceptual/saturation; 16-bit tone-curve quantisation; now this). Assume
there is a fourth.

**Method lessons that earned their keep:**

- **Compute the expected divergence from the fixture and BOTH algorithms
  BEFORE comparing against the oracle**, and make the tolerance *be* that
  envelope. Observed then landed 0.3–0.5 % under it. Had it landed above, the
  tolerance would not have moved — the finding would have been that something
  else was in play.
- **When a known non-error dominates, find the subset of the corpus where it is
  identically zero and grade that subset separately.** The 16 CLUT-node corners
  cost nothing, agree to `transicc`'s print floor (6.7×10⁻⁵ ΔE00), and are what
  make a 2.0 ΔE00 structural gate defensible instead of embarrassing.
- **Split wide from tight and say which claims what.** A gate at the
  perceptibility anchor cannot demonstrate agreement; the agreement claim
  belongs to the emulated-geometry and corner rows, 100× and 2000× tighter.
- **Grade the apparatus.** The harness reimplements the `mft2` pipeline so the
  geometry can be substituted; its n-linear arm is held against
  `iccce-cmm`'s evaluator on every point (observed 0.0, bit-identical) before
  any conclusion is drawn from it.
- **A clause citation written from recollection is a claim, and later findings
  inherit its error.** NA-003 cited ICC.1:2022 6.4 for a *device*-value
  clipping rule; 6.4 is about the PCS, the device clause is 6.5, and the Pass 3
  §13.4 finding had been built on the mis-citation. Corrected in
  `TOLERANCES.md` §5.2, append-style.

Related: [[project-oracle-and-tolerance-state]],
[[project-lcms2-findings-pass3-quantisation-and-clamping]],
[[project-lcms2-findings-legacy-lab-and-forced-bpc]].
