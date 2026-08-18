---
name: project-passk-black-preservation-baseline
description: Pass K measured black preservation BEFORE the feature existed — and the headline is about the instrument, not the engine: ΔE2000 is blind to the defect (0.705 of chromatic ink at 0.136 ΔE00), so the whole section is in device units; plus the saturation-intent shortcut works on 2 of 6 vendors, the synthetic corpus is ZERO-SEPARATION for this subject, and a rival CORPUS is not a rival candidate.
metadata:
  type: project
---

**Built 2026-08-17 at tip `506fcd3`.** `tools/difftest/src/passk.rs` (33 rows),
instrument `src/bin/passk_probe.rs`, bounds `docs/TOLERANCES.md` **§3.10**,
operational notes `tools/difftest/README.md` **§25**. Suite
`pass=229…274` → **`pass=325 fail=1 skip=9 error=0`**; the one failure is
deliberate. Pass K contributes **`unstated=0`, `blind=0`**, and clippy-clean.

**Why: the operator asked for the INSTRUMENT before the feature.** `crates/` had
no black-preservation code; the tolerances were fixed before anyone could see
which ones would be convenient.

**How to apply.**

- ★★★ **ΔE CANNOT MEASURE THIS SUBJECT, and that is the finding.** K-only ramp
  into `ISO Coated v2 300% (ECI)` (same profile, media-relative, 41 pts):
  **`0.705320` chromatic ink**, **TAC `2.753549`** from an input that cannot
  exceed `1.0`, **`|ΔK| 0.360889`** — and **`0.136090 ΔE2000`** from the K-only
  build. The engine is *right*: it agrees with lcms2 to **`6.3e-5`**, and the
  destination's own `B2A1` stayed inside its declared 300 %. **A conformance
  suite that graded this perceptually would report nothing.** Row `A4` grades
  the ΔE against §2's `1.0` anchor and **passes on purpose**; its separation is
  `SepUnits::Other` → `INCOMMENSURATE`. Generalisation: *before choosing a
  metric, ask whether the defect has a signature in it.*
- ★★ **Two orders tighter than `SWEEP_DEVICE` is possible when the envelope is
  ZERO BY CONSTRUCTION.** The K-only ramp lies on an **edge** of the 4-D `A2B`
  hypercube — every scheme (quadrilinear, tetrahedral, lcms2's `Eval4Inputs`)
  degenerates to the same 1-D interpolation there — and §E's off-neutral points
  are `A2B` **CLUT nodes** (`j/15`). Bound = **run-time-measured** device
  response to one 16-bit PCS quantum + 2e-6 print floors (`1.24e-4` / `2.00e-4`;
  observed `6.3e-5` / `5.4e-5`). ★ **`E5` is the control that earns it**: the
  same comparison off the nodes is **`1.750e-3`, 32×** larger. *Without the
  control, a tight bound is indistinguishable from a lucky one.*
- ★★ **The shortcut that works on exactly one vendor.** *"Use the saturation
  intent instead of building black preservation"* is true of **2 of 6** real
  CMYK destinations — both ECI's `ISO Coated v2` variants (`0.036`/`0.039`) —
  and false of FOGRA39/27, GWG_GenericCMYK and X-Rite (`0.51`–`0.78`), three of
  which alias `B2A0 ≡ B2A2`. **Measuring on ISO Coated v2 alone would have
  concluded the feature was unnecessary.** Where it works it costs up to
  **`6.4151 ΔE2000`**.
- ★★ **The committed synthetic CMYK fixture is `ZERO-SEPARATION` for this
  subject.** `v2-cmyk-mft2-lab.icc`'s `B2A0` is `lab_to_cmyk_clut`, which emits
  `[0,0,0,k]` at every node, so its K ramp is K-only **already**. Row `E6`
  emits that as a number rather than a paragraph. Consequence: **every graded
  row of §A–§E skips in CI, permanently.** ★ **CLOSED the same day by §F** —
  recipe `v2-cmyk-chromatic-neutral`, separation `0.420705`, seven rows all
  running in CI, one red there. `E6` is **kept** (it says why a second fixture
  had to exist) and `E1` is **not repointed**. See
  [[project-passk-f-separating-fixture]] — including the injection result, which
  shows the collapse is worse than "cannot discriminate".
- ★★ **A rival CORPUS is not a rival CANDIDATE** — the twin of Pass 4c's *a
  rival tolerance is not a rival candidate*. Both refutation rows first stated
  *"the corpus had contained only the favourable member"* as a separation and
  both reported **BLIND** for a property they do not have. Population changes
  are **coverage**, not separation.
- ★ **The `refutation row` shape.** `Record::graded` cannot express *"at least
  one counterexample"*, so observe **how many members the shortcut HOLDS for**
  and bound one below the population size. `IndicatorCount`; the bound comes
  from the logic, never the observation.
- ★ **`Intent` was NOT extended for lcms2's intents 10–15.** They live behind
  `passk::KOnlyOracle`, which builds its own `transicc` argv and carries a
  `CAVEAT` constant that `k_source()`/`k_only_alt()` prepend to every record —
  the disclaimer is **data**, not discipline.
- ★ **`E1` is RED by design at tolerance exactly `0`** (`0.705320` observed) and
  its separation is taken from **lcms2's colorimetric answer**, not from
  iccce's, so it does not collapse the day the row goes green
  ([[project-prove-the-arm-by-injecting-the-defect]] §1).
- **What is NOT done:** no injection proof (§22/§23/§24 all have one); one
  destination carries §A/§C/§D/§E; media-relative only outside §A; no `--bpc`
  anywhere; `E2` is REPORTED **for ever** because the K value is a vendor
  construction (A27/A42 posture).

Related: [[project-passk-icc1-is-silent-on-black]],
[[project-passg-tolerance-lessons]], [[project-candidate-separation]],
[[project-synthetic-fixture-corpus-and-gp001]],
[[project-oracle-and-tolerance-state]].
