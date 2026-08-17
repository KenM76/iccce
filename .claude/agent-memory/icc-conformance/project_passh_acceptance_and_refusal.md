---
name: project-passh-acceptance-and-refusal
description: Pass H graded ACCEPTANCE and REFUSAL over the ICC's own 50-profile color.org set — the first Kind::GroundTruth rows in tools/difftest, and the published statement they rest on turned out to be FALSE of the file the document names; plus a real defect (the compiled path aborts on 7 channels), a conformant 33 L* divergence with lcms2 under clause 8.10.2, and a bound that was withdrawn rather than widened.
metadata:
  type: project
---

**Built 2026-08-17 at tip `e21154c`.** `tools/difftest/src/passh.rs` (filed with
48 rows, **51 now**), instrument `src/bin/passh_probe.rs`, bounds
`docs/TOLERANCES.md` **§3.8**, operational notes `tools/difftest/README.md`
**§23**. Suite went `pass=229` → **`pass=270 fail=1 skip=9 error=0`**, bare exit
**1** → **`pass=274 fail=0 skip=9 error=0`, bare exit 0** once the engineer fixed
the defect below (re-measured 2026-08-17; see
[[project-a-fixed-defect-can-blind-its-own-row]] for why the fix forced a row
split). Corpus:
`D:\Dev\iccce-private-fixtures\color-org\`, 50 files, uncommittable,
`$ICCCE_PRIVATE_FIXTURES`-gated, skips everywhere else forever.

**Subject is NOT a colour value** — that corpus publishes transforms, never
expected outputs (DL-041). It is *which files are accepted, which refused, and
whether a refusal says why*. **40 accepted / 10 refused**, measured.

## How to apply

- **★★★ GROUND TRUTH IS A PROVENANCE, NOT A GUARANTEE.** `Probev2.zip` ships
  the ICC's `Probe2 Profile Readme June 1, 2007`, which states in numbers what
  `Probev2_ICCv4.icc` does — the first thing in this project that satisfies
  §1's definition of `Kind::GroundTruth`. **And the statement is FALSE of the
  file it names.** *"tints of pure cyan/magenta/yellow"* is realised **to the
  bit** on the two `Probev1` profiles the readme does *not* describe
  (off-colorant channels exactly `0.0`, `a*`/`b*` worth `3.3e-16`) and not at
  all on `Probev2_ICCv4` (off-colorant max `0.9969`). **Run a ground-truth row
  before believing it.** Response: relax to **infinity with a loud
  `★★★ THE PUBLISHED CLAIM IS FALSE OF THIS FILE` prefix in the emitted
  detail**, never to a finite number the observation happens to satisfy — and
  grade instead the **weaker statement the sentence still entails** (*the named
  colorant is strictly the largest chromatic channel*), which is what actually
  catches an intent-to-tag mis-wiring.
- **★★★ Two conformant CMMs, `33.13 L*` apart, from one file.** ICC.1:2022
  **8.10.2 a)** prefers `DToBx`/`BToDx` *"except where this tag is not needed or
  supported by the CMM"*; **b)** falls back to `AToBx`/`BToAx`. iccce does not
  implement `mpet` → takes (b); lcms2 does → takes (a). **Both conformant**
  (A33). lcms2's three intents return red/green/blue — exactly what the readme
  says `B2D0/1/2` do — so the profile *identifies which tag each engine used*.
  ★ **Owed to the engineer: iccce takes step (b) SILENTLY.** Nothing in
  `inspect` or `transform` discloses that an author-preferred transform was
  present and declined. Clause permits declining; it does not require silence.
- **★★ A REAL DEFECT the corpus found and nothing synthetic would have —
  FIXED 2026-08-17, and the fix is a lesson of its own.** `iccce bench` on the
  corpus's `7CLR` profile **aborted**: bare exit `-1073740791` (`0xC0000409`),
  *"memory allocation of 1022842631448 bytes failed"*.
  `recommended_grid_points(7)` returned 33 (its `_ => 33` catch-all, documented
  only for 3-D/4-D) → `33^7 = 42 618 442 977` nodes ≈ 952.6 GiB; `checked_pow`
  guards wrap, not size. **The row was RED and there was no number to move** —
  the observable is an exit status, which is why this is the cleanest
  demonstration in the project of what a red suite is for. `transform` was
  unaffected (reference `Chain`, not the compiled grid). ★ Now green at grid 6,
  with a named refusal at `--grid 33` — and **the row had to be split into four**
  because either half of the fix alone satisfied it:
  [[project-a-fixed-defect-can-blind-its-own-row]].
- **★★ The ≥5-channel compiled path stays REPORTED, not graded, and the reasons
  are ranked.** `compiled-vs-reference-at-the-default-grid` = `2.952005e-3`
  device units at grid 6 over 527 probes. (1) **Both arms are iccce** —
  self-comparison, dispositive on its own. (2) The gate that justifies 33 for
  3-D/4-D is a *measured lcms2 agreement*; there is no counterpart at 7 inputs
  because ICC.1 legislates no interpolation method (A16) and lcms2's n>4 CLUT
  geometry has not been read out of the pin. (3) **n = 1 profile.** (4) The grid
  is now a function of a *memory budget*, so the number moves for reasons with no
  colour content. **What IS graded is structure** — exit codes, refusal wording,
  stdout emptiness, recommendation-vs-behaviour agreement — indicator counts at
  exactly 0. Unblocking conditions named in `TOLERANCES.md` §3.8.4.5.
- **★★ A bound WITHDRAWN rather than widened — the third instance of one
  failure shape.** `SEVEN_CORNER = 5e-5` failed at `1.191176e-4`. The code was
  fine: on the **PCS** side the same 128 corners agree to `4.900435e-5 L*`. The
  derivation's line *"the destination's 16-bit reverse tone curve, 1.53e-5"*
  assumed an **analytic** inverse; `sRGB2014.icc` has **1024-entry tabulated**
  TRCs and lcms2 inverts those through a 4096-entry reverse curve. **The
  generalisation: when a tolerance's `why` contains a clause about a component
  the row does not own — a destination, a direction, a fixture property — that
  clause is where the missing term will be.** (Pass 4b `B6`, Pass G
  `SWEEP_DEVICE`, this.)
- **★★ Pass 4b's encoded-PCS clamp divergence reproduces on a REAL file.**
  iccce clamps the encoded PCS at the B curve (10.18's domain, `Trc::eval`);
  lcms2 does not. Found on a fixture *we authored*; now `0.2374 L*` on
  `Probev1_ICCv4`. ★ The two `Probev1` files nail the mechanism: same design,
  legacy `mft2` ceiling `100.390625` (no overflow) vs v4 `mAB ` ceiling `100.0`
  (overflow). **The overflow is the ENCODING, not the data.** Split predicate
  evaluated on **lcms2's** output — the side that does not clamp — never on
  iccce's (splitting on iccce's clamp fixed-point would split on the behaviour
  under test).
- **★★ Injection localised, and five separations predicted their own failure
  magnitude to the digit** (10, 4, and 81 three times); a sixth reproduced
  `ORACLE_LAB`'s stated sensitivity `3.906250e-1 = 100 × (65535/65280 − 1)`
  exactly. **The most useful thing was what stayed GREEN:** rotating the
  intent→`B2A` map turned red **only** the row that drives the shipped binary —
  seven of §D's eight per-profile rows evaluate a tag *by signature* in process
  and are blind to a *wiring* defect by construction. *Ask of every row not
  "what does it measure" but "which layer is in the loop".*
- **★ `transicc` EXITS 0 WHEN IT FAILS.** It prints *"Couldn't link the
  profiles"*, converts nothing, returns 0 — on all ten iccMAX files. Any
  oracle-side acceptance test keyed on the exit code records lcms2 as accepting
  everything. Observable used: *did any numbers come out*. Companion to §5.6's
  rule about the harness's own exit code.
- **★ A synthetic fixture proves what it was written to prove and no more.**
  `fixtures/synthetic/iccmax-version.icc` showed lcms2 ignores the version word
  (`NUMERIC_CLAIMS.md` §3.10.6) — true, and it does **not** generalise: lcms2
  declines all ten *real* iccMAX files, for their content. Kept as §A's control
  precisely because it isolates the version byte.
- **★★ A COVERAGE NUMBER QUOTED WITHOUT ITS CORPUS IS NOT A COVERAGE NUMBER.**
  My §E census (`CMYK 23, RGB 16, 7CLR 1 = 40`, **no GRAY**) and the engineer's
  sweep (`CMYK 33, RGB 25, GRAY 1, 7CLR 1 = 60`, NC-220) looked like rival
  claims and are **two denominators**: `color-org` (40 accepted) vs **both**
  private corpora including `ghent-v50` (20). They reconcile exactly —
  `23+16+1` and `10+9+1`. **The single GRAY profile is in `ghent-v50`, not
  here.** The §E row now names its own denominator and points at the other.
  *iccce does have GRAY evidence; Pass H is not where it lives.*
- **★ Coverage the corpus does NOT have, and the population census says so:**
  **no `GRAY`, no `Lab `, no `XYZ ` colour space in the accepted 40**
  (`D50/D55/D65_XYZ.icc` declare `colourSpace = 'RGB '`); **zero `6CLR`
  evidence** (the only six-channel file is iccMAX and refused); no `namedColor2`
  behaviour (both `nmcl` files are iccMAX); and **no differential colour row for
  any of the 20+ CMYK print profiles** — `NEXT_SESSION.md` queue item 6 is *not*
  discharged.
- **`Metric::IndicatorCount` was added to `lib.rs`.** A count is not a
  difference; emitting one as `abs-max-component` puts a wrong unit in the TSV.
  A count row's tolerance is essentially always zero.

Related: [[project-passg-tolerance-lessons]],
[[project-passg-ghent-population-findings]],
[[project-lcms2-findings-pass4b-direction-dependence]],
[[project-prove-the-arm-by-injecting-the-defect]],
[[project-candidate-separation]],
[[project-parallel-agent-build-collisions]].
