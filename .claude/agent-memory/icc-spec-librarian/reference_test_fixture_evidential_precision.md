---
name: test-fixture-evidential-precision
description: A test fixture's evidential precision is its ASSERTION TOLERANCE, not its printed precision — find the comparison function and read the epsilon before citing any number from any suite; established on web-platform-tests (prints 5-6 s.f., asserts at 0.01, and 47 of 69 components are wrong at the printed precision while all 69 pass); includes WPT's licence being BSD-3 and NOT MIT
metadata:
  type: reference
---

**Before citing any number out of any test suite — `web-platform-tests`, lcms2's `testbed`, `DemoIccMAX` regression data, anyone's fixtures — find the comparison function and read its epsilon. A fixture printed to 17 digits and asserted with `assert_approx(1e-2)` is a two-decimal claim in a seventeen-decimal costume.**

Filed in the corpus as the evidence modifier **`machine_checked_to_stated_tolerance`** (`ICC_Spec\LEGAL_NOTE.md` §3) — the mirror image of `publisher_checksum_verified`: that one certifies the held bytes are the published bytes, this one **bounds how much of a printed number was ever tested.**

## Why — the measurement that established it (2026-08-18)

`web-platform-tests` `css/css-color` was fetched, held and run against CSS Color 4's own published algorithm at `mpmath` dps=50, two independent routes.

- The files **print 5–6 significant figures.**
- The script tests **assert at `ε = 0.01`** — `css/support/color-testcommon.js`, `set_up_fuzzy_color_test`, verbatim: `if (!epsilon) { epsilon = 0.01; }`. Of 509 `fuzzy_test_*` calls in the color-mix file, **four** pass an explicit epsilon and it is `0.1`, looser still.
- The reftests compare **rendered 8-bit pixels** — 12 of 13 examined carry **no `fuzzy` metadata at all** (exact match, `1/255 ≈ 0.0039`); one carries `maxDifference=0-1`.
- **Result: 47 of 69 published components fall outside their own printed intervals. All 69 pass the assertion.** Worst rendered error across 17 vectors: **0.032 8-bit codes**, against a 1-code criterion.

**The sharpest single case:** `css/css-color/xyz-d50-004.html` **omits the chromatic adaptation entirely** — its reference is reproduced by the no-adaptation pipeline to `1.14e-4` and by the correct one only to `2.69e-2`, a **235×** discrimination — **and the test passes anyway**, because both candidate answers clip to identical 8-bit pixels. **A reftest that renders sRGB has literally zero power over an out-of-gamut expectation.**

## The tier consequence, stated so it is not re-litigated

A conformance-suite fixture is **not** `published_ground_truth` and is **not** simply `published_literature` either. Two independent reasons, and the second is the one usually missed:

1. **The precision argument above** — "machine-checked" is true of the *feature* and false of the *digits*.
2. **Its correctness criterion is agreement with implementations, not with the specification.** A value all engines produce is, for the suite's purpose, correct whether or not it is what the spec computes. Structurally that makes it a **cross-check against a CONSENSUS of implementations** (iccce project rule 3) — stronger evidence about the ecosystem than one implementation, and **no stronger at all about the standard.** Nearer to lcms2 than to ICC.1 Annex D.6.3.

**Symptom of getting this wrong: QUIET.** A suite that adopts a fixture value at a loose tolerance passes while enshrining a number nobody checked, and the next real regression of the same size hides behind it.

## Two facts about WPT specifically, worth not re-deriving

- **Licence is 3-Clause BSD and NOT MIT.** The widely repeated "WPT is MIT/BSD" description does not match the repository: root `LICENSE.md` is titled verbatim `# The 3-Clause BSD License` and contains only the three clauses; `CONTRIBUTING.md` line 1 says all contributions are BSD-3; **no `LICENSE`/`COPYING` exists under `css/`**; `README.md` has no licence section. **★ Clause 3 bars writing "validated against web-platform-tests"** in any README or release note — an endorsement claim.
- **What it does add that nothing else did:** `parsing/color-mix-out-of-gamut.html` holds **five PCS→device vectors that PRESERVE out-of-gamut negatives** (e.g. `lab(100 104.3 -50.9)` → `color(srgb 1.59343 0.58802 1.40564)`). Even at `±0.01` that is a real external witness that **no clipping happens in the transform**. Fetching is free: `raw.githubusercontent.com`, and GitHub is already cleared in `LEGAL_NOTE.md` §2.

## Lineage

This is corpus defect `C5` (*a displayed value is an INTERVAL*) and `C8` (*a claim about TEST POWER, not magnitude*) applied to the **assertion** rather than to the value or the stimulus. Full file: `D:\Dev\Rag-Specialized\ICC_Spec\w3c\w3c__data__wpt_css_color_vectors.md`.

Related: [[derived-values-need-a-second-pass]], [[published-ground-truth-state]], [[label-the-predicate-not-just-the-payload]], [[corpus-defects-are-caught-from-outside]].
