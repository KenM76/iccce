---
name: project-synthetic-fixture-corpus-and-gp001
description: fixtures/synthetic now exists (39 profiles from tools/gen-profiles, verify-able byte-for-byte); building it produced FINDING GP-001 (iccce mis-counts mBA curve sets), three ICC_Spec corpus gaps, and GP-002 — a round-valued fixture makes distinct quantities coincide and has zero power.
metadata:
  type: project
---

**State as of 2026-08-11.** `tools/gen-profiles` (standalone, zero-dep, own
empty `[workspace]`) and `fixtures/synthetic/` (39 committable profiles:
13 well-formed, 26 one-defect-each malformed) exist. This **supersedes** the
"`tools/gen-profiles` still open" line in
[[project-oracle-and-tolerance-state]].

**Why:** Pass 2 done-when clause 2 was PARTIAL — in-test synthetics are
tag-level and unusable outside `cargo test`.

**How to apply:**

- `gen-profiles verify <dir>` regenerates in memory and compares byte for
  byte; it is the reason nothing in that crate may read a clock, an env var,
  or an RNG. `MANIFEST.md` is generated, never hand-edited.
- The crate must **never** gain a dependency on `iccce-*`. A fixture written
  with the parser's own encoder cannot detect a *shared* misreading of the
  spec. (`tools/difftest`'s path-dep on `iccce-color` is a different case and
  is justified in its manifest; do not reason by analogy.)
- Nothing in that corpus is a colorimetric reference. Colorants are an
  arbitrary split of the *encoded* D50 white chosen so the integers sum to it
  exactly — a structural invariant, not colorimetry.

**★★ Updated 2026-08-12 (Pass 5c): the corpus is 39 profiles.** New recipe
`v4-rgb-mab-chromatic-black` — v4.4 `prtr` **RGB**, `mAB `/`mBA ` (3→3, 9³, the
`A,CLUT,B` combination: no matrix, no M curves), device black
`Lab(20 · 4 · −3)`, device white `Lab(100 · 0 · 0)`. **It is the first fixture in
the corpus with a non-zero, chromatic black**, and it was built RGB rather than
CMYK on purpose: lcms2 only keeps a black point's chroma when the destination is
**not** (output-class AND ink space). Its colour model is affine and the `B2A`
CLUT is its **exact closed-form inverse**, so multilinear interpolation
reproduces it exactly and NA-006 is not a confound. All three Lab values encode
**exactly** in the general 16-bit PCSLAB encoding. `verify` clean, 18 608 bytes,
`iccce inspect` reports 0 malformations. Being square (3→3) it **cannot** catch
GP-001, and its recipe says so. See
[[project-pass5c-estimator-branch-finding]].

**★★★ FINDING GP-002, 2026-08-12 — round, symmetric values make conceptually
distinct quantities COINCIDE, and coincidence is zero power.**
`v4-rgb-mab-chromatic-black.icc`'s `InitialLab` and `outRamp[first]` are **both
`L* 20`**, so ISO/CD 18619 4.2.5.4's two candidate return values are one number
on it. A real defect in that clause moved `USWebCoatedSWOP` by **4.717441 `L*`**
and moved this fixture by **exactly zero**; it would stay green through a full
reversion. **Not an authoring mistake** — it falls out of three properties each
chosen for a good reason: the model is affine, the `B2A` is its exact inverse,
and the black *is* the darkest vertex, so the round trip's floor equals the
neutralised vertex. On a real ink set none of the three holds exactly.

- **The right question is never "does this fixture exercise the path".** It is
  *"if the code returned the other candidate, would this fixture's number
  move?"*
- ★ **The only load-bearing detector here is category (c)** — the vendor
  profile, never committed. On a machine without the Windows colour directory
  those rows **skip** and everything else stays green. A green CI run on a
  Linux runner means less than it looks.
- **Roundness is a virtue for hand-checkability (§4's whole discipline) and a
  hazard for discrimination.** When they conflict, keep the round fixture and
  **add a second one**; do not make the readable one irregular.
- **Do not delete this fixture over it.** It is still the only profile in reach
  that reaches lcms2's `BlackPointAsDarkerColorant` branch at all. A
  distinct-values fixture is a NEW recipe (a `B2A` with a `G` floor for *every*
  input lifts the round-trip floor while leaving `A2B(0,0,0)` alone); editing
  these bytes would move NC-166's companion device figure `5.725e-2`.
- Now machine-visible: the harness emits `ZERO-SEPARATION` on that row — see
  [[project-candidate-separation]]. Filed as `tools/gen-profiles/README.md`
  §4.1, in the recipe doc, and in the generated `MANIFEST.md`.

**★★ Updated 2026-08-11 (later still, Pass 4b).** Three things:

1. **GP-001 is fixed and now has a regression that is not an oracle.**
   `v4-cmyk-mab-lab.icc` is driven in both directions in `difftest`'s §B, and
   iccce reproduces a **closed form derived from 10.12/10.13** to `f64` noise
   (2.8×10⁻¹⁴ `L*`, 2.2×10⁻¹⁶ device). The `mBA ` counts (B=3, M=3, A=4) are
   what make the chain evaluate at all, so that row *is* GP-001's regression.
2. **The fixture is not merely convenient — it is the only instrument.** All
   **40** `.icc`/`.icm` in `C:\Windows\System32\spool\drivers\color\` were
   parsed and searched: **zero carry `mAB ` or `mBA `.** The only v4 profile
   with a LUT (`BlackWhite.icc`, 4.0.0 `prtr` GRAY) carries an `mft1`. Without
   `tools/gen-profiles` the entire v4 element-pipeline path is unmeasurable on
   this machine at any price.
3. **★ The fixture caught a defect it was not designed for.** Its 3×4 matrix
   offsets exist because *dropping* them is the classic misread. They also push
   the encoded `L*` to **1.00390625** at `K = 0` — outside the encodable PCS
   range — where **iccce clamps and lcms2 does not, worth 0.61 ΔE2000**. The
   best argument on file for authoring fixtures with awkward values rather than
   tidy ones. Unsettled; see
   [[project-lcms2-findings-pass4b-direction-dependence]].

**★ FINDING GP-001 (open at filing).** `crates/iccce-profile/src/lut.rs`
`decode_lut_ab` counts **B and M by `output_chan` and A by `input_chan` for
BOTH tag types**. Correct for `mAB `; wrong for `mBA `. ICC.1:2022 **10.13.2 /
10.13.4** say B and M are counted by **input** channels and **10.13.6** says A
by **output** — so a CMYK `B2A0` (3 in, 4 out) has B=3, M=3, A=4. iccce refuses
it with `curve chain broken at element 3 (byte 68)`. lcms2 agrees with the spec
(`Type_LUTB2A_Read`) and converts through the same fixture. **The defect is
invisible whenever `inputChan == outputChan`, i.e. on every square LUT, and
appears on every real CMYK B2A0** — which is exactly the population the
40-profile machine sweep lacked.

**★ Three ICC_Spec corpus gaps found while authoring bytes** (report to
`icc-spec-librarian`; do not edit that tree directly):

1. `icc__type__lutAtoB_lutBtoA.md` carries **one blanket sentence for both
   types** ("A = inputChan; B and M = outputChan") — the likely origin of
   GP-001. Needs 10.12.2/4/6 and 10.13.2/4/6 transcribed **per type**.
2. **A23 is closable**: 10.12.1/10.13.1 enumerate the permitted element
   combinations verbatim (`B`; `M,Matrix,B`; `A,CLUT,B`; `A,CLUT,M,Matrix,B`
   for `mAB `, mirrored for `mBA `), plus "At least one processing element
   shall be included".
3. **A25 is closable**: 10.15 states the `mluc` fallback (same language code,
   else the first record).

Also still stale in that tree: `icc__type__lut8_lut16.md` §"facts that gate
correctness" and `icc__s__pcs_encoding.md` §2 **still say the legacy-Lab
selector is `header.version`**, contradicting the A1 resolution at the top of
the same file. See [[project-lcms2-findings-legacy-lab-and-forced-bpc]].

**One divergence the corpus now pins:** lcms2's `transicc` **accepts** a major
version 5 profile (`iccmax-version.icc`); iccce refuses iccMAX by name. Not a
defect on either side — a deliberate difference that now has a fixture.
