---
name: project-passg-ghent-population-findings
description: Pass G graded iccce against 20 REAL producer-embedded profiles (Ghent v5.0) — the first vendor-authored v4 mAB; three findings a synthetic corpus could not produce (a non-identity B curve that broke a tolerance derivation, vendor-specific intent-tag aliasing, Adobe's shipped sRGB/AdobeRGB having wtpt=D65 with D50 colorants); and the eciRGB v2/v4 pair is NOT the version isolator it looks like.
metadata:
  type: project
---

**Built 2026-08-17 at tip `e21154c`.** `tools/difftest/src/passg.rs` (72 rows),
instrument `src/bin/ghent_probe.rs`, bounds `docs/TOLERANCES.md` **§3.7**,
operational notes `tools/difftest/README.md` **§22**. Suite went
`pass=157` → **`pass=229 fail=0 skip=3 error=0`**. **Pass G contributed ZERO
`unstated` separations** — every row states one; `blind=0`.

**Why: this corpus is not synthetic, OS-shipped or standards-body-issued.** It
is what Adobe InDesign CS6 actually embedded, 121 times across 98 PDF/X files.
Licensed, uncommittable, resolved via `$ICCCE_PRIVATE_FIXTURES`, **skips
everywhere else including CI permanently**.

**How to apply.**

- ★★★ **A real vendor profile is a different instrument, and the difference is
  that its shapes were not chosen by us.** Three findings exist only for that
  reason and a generator would never have produced any of them:
  1. X-Rite's `A2B0` `L*` B curve is a **non-identity** 2-entry `curv`
     `(0x0808,0xFFFF)` — its declared perceptual black, `L* 3.137254`. It
     **broke a tolerance derivation** (see below).
  2. **Intent-tag aliasing differs BY VENDOR**: X-Rite aliases `A2B1≡A2B2`
     (media-relative/saturation), both ECI profiles alias `A2B0≡A2B2`
     (perceptual/saturation), the GWG CMYK trap also aliases `B2A0≡B2A2`.
     **An engine or suite that hard-codes one pairing is wrong on the others.**
  3. Ghent's **`sRGB IEC61966-2.1` and `Adobe RGB (1998)` encode `wtpt` = D65
     while their colorants sum to D50, with no `chad`** — the M5 shape, now
     shown to be the **population norm**, not a one-off system profile.
     ICC.1:2001-04 A.3.1.1 makes that a **defect of authorship**, so the
     ICC-absolute divergence settles **in lcms2's favour**. (Do not cite that
     "should" as an ISO-directives *should* — the document has no verbal-form
     hierarchy; `icc__s__v2_ICC1_2001_04.md` §1.3.)
- ★★ **The v4 `mAB ` disagreement IS the interpolation method and nothing
  else.** With lcms2's `Eval4Inputs` geometry substituted, `A2B1` collapses
  `0.828444 → 4.6245e-3` (**179x**) and `A2B0` `0.950274 → 3.9123e-3`
  (**243x**); the envelope computed from the CLUT bytes alone is `0.828123` /
  `0.948160`, accounting for the raw residual to **0.04 % / 0.22 %**. Same
  signature as Pass 4 on `lut16`. This closes §3.4.3's "any REAL v4 LUT
  profile" gap.
- ★★ **lcms2's forced BPC fires here and a FIXTURE defeats it.** Into `*Lab4`
  lcms2 prints `L* 0` for full ink at perceptual; into `*Lab2` it prints the
  profile's own declared `3.1373`. Gate is on the **destination's** version
  (Pass 4b finding 2). §A runs the perceptual arm against `*Lab2` — the Pass 4c
  lesson again: **a fixture that keeps the gate shut beats a model that
  subtracts what the gate did.**
- ★★ **The `eciRGB v2` v2.4/v4.2 pair is NOT the version isolator it looks
  like.** Both encode `wtpt` **at** the PCS white, so lcms2's substitution is a
  no-op for either and the version leg is never exercised; **no pair in this
  corpus differs only in version while encoding a non-PCS white.** And the two
  files differ in **TRC representation** (700-entry `curv` vs `para` type 3) as
  well as version, so a disagreement has two causes the pair cannot separate.
  Its rows are `SelfConsistency` — the **weakest class in the module**, weaker
  than a cross-check because there is one lineage on both sides.
- ★ **The BPC estimator divergence reproduces on REAL print profiles**:
  ISO vs lcms2 black `L*` differ by **2.010883** (ISO Coated v2 v2.4),
  **0.823487** (FOGRA39 v2.1), **2.084405** (X-Rite v4.2) — device cost
  3.5e-2 to 7.6e-2. Not a synthetic-fixture artefact. Still ungradeable:
  A27/A42, no normative BPC text.
- **iccce refuses `--bpc` by name on 8 of 20 combinations** (v4 `mAB ` source
  outside the estimation subset). Graded as deliverables at exactly `0.0`.
  ★ Note an unexplained asymmetry worth chasing: it **accepts** `--bpc` at
  perceptual for that source and **refuses** at media-relative and saturation.

Related: [[project-passg-tolerance-lessons]],
[[project-lcms2-findings-pass4b-direction-dependence]],
[[project-pass4c-absolute-intent-findings]],
[[project-lcms2-findings-pass4-interpolation-and-v2-wtpt]],
[[project-candidate-separation]].
