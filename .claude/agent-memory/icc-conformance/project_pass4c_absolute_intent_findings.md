---
name: project-pass4c-absolute-intent-findings
description: Pass 4c measured ICC-absolute through a LUT destination by choosing a profile pair that defeats lcms2's wtpt-substitution CONJUNCTION — 8.90e-5 device over 729 points, sensitivity 2310x. Also — the NC-053 policy is direction-symmetric, the handoff's "saturation never run" was stale, and every gray profile in reach is PCSXYZ.
metadata:
  type: project
---

All measured **2026-08-12** at pin `21c582a`. Apparatus:
`tools/difftest/src/pass4c.rs` (new, 10 records, all pass, **reproduced
bit-identically across two runs**). Tolerances: `docs/TOLERANCES.md`
**§3.4.5**.

## 1. ★★ The method that unblocked a question deferred through eight filings

**lcms2's `wtpt` substitution predicate is a CONJUNCTION** —
`version < 0x4000000 AND class == 'mntr'` (`cmsio1.c`,
`_cmsReadMediaWhitePoint`). NC-053's 11 ΔE was treated for eight filings as
blocking any measurement of the ICC-absolute *arithmetic*, pending a
document (corpus A4b).

**It was never blocked on a document. It was blocked on a PROFILE PAIR**,
and the pair was sitting in the committed fixture corpus the whole time:

| role | profile | defeats the gate on |
|---|---|---|
| src | `fixtures/synthetic/v4-rgb-matrix-trc.icc` (v4.4.0, `'mntr'`) | the **version** half |
| dst | `USWebCoatedSWOP.icc` (v2.1.0, `'prtr'`) | the **class** half |

Each fails a *different* half, so the pair does not rest on one property.
The other Pass 4 confound is zero too: lcms2 forces trilinear for a Lab-PCS
output LUT (= iccce's n-linear, NA-006 = 0) and the source has no CLUT.

**Generalise this.** When a divergence is gated by a compound predicate,
look for a fixture that makes the predicate FALSE before building a model
that subtracts the divergence. A model can absorb a real error along with
the effect it was built to isolate; a structurally-absent confound cannot.

**Measured:** absolute **8,900×10⁻⁵** device max / 1,830×10⁻⁵ mean, 729
RGB points (9×9×9 on the 8-bit lattice). **Media-relative on the same pair
and grid: 1,080×10⁻⁴.** The absolute row is BELOW its own floor — the
ICC-absolute arithmetic adds nothing above what the direction already
costs. **No new tolerance was minted**; `pass4b::DEVICE_B2A` (5×10⁻⁴)
reused unchanged, same destination table.

## 2. ★ The counterfactual was EXACT, not modelled — and it is free

Because the source's stored `wtpt` **is** D50, substituting D50 for the
*destination's* `wtpt` collapses the 6.3.2.2 diagonal to identity — so
**`absolute vs media-relative` on this pair IS the NC-053 substitution
priced on this pair**, not an approximation of it. **2,055 76×10⁻¹**,
**2 310×** the observed residual (floor of 100 transcribed from Pass 4b's
accepted 99×/139×/191× band).

Two nulls were guarded, not one. The obvious one is "the effect is absent";
the other is **clipping** — if the absolute scaling pushes the grid out of
gamut, both implementations clamp to the same boundary and agree perfectly
while computing nothing. Counted: **1 of 729** points unmoved (device
black, the fixed point of any diagonal — arithmetic, not a defect).

## 3. The NC-053 policy is DIRECTION-SYMMETRIC (DL-021)

NC-053 had the v2 `'mntr'` profile as **destination** (11,217 ΔE2000).
`sRGB → SWOP` at absolute puts it as **source**: **2,134 24×10⁻¹** device
over 729 points, **1 654×** its own media-relative floor (1,290×10⁻⁴).
**Predicted before the run** and it held. REPORTED, NOT GRADED.

## 4. What the spec dispatch returned, and it is sharper than "silent"

- Normative cite is **`ICC.1:2022` 6.3.2.2 Eq (4)–(6)**; Annex D is the
  *informative* restatement. **"D.6/D.7" is NOT edition-stable** —
  `ICC.1:2001-04` Annex D has **no (D.7)**, and its (D.6) is the single `Z`
  of the *inverse*. Every v2-`wtpt` discussion in this project was carrying
  a live ambiguity.
- **lcms2's predicate reproduces no clause in either edition**: v4's 9.2.36
  is class-gated with **no version gate**; v2's A.3.1.1 is gated on the
  **adaptation condition**, not on class at all.
- **The conformance clause binds READING, not computed output.** So
  *"non-conforming"* is not an available verdict about a CMM. Say
  **diverges**. This binds every document in the project.
- Corpus action line, verbatim: *"keep NA-007 (use `wtpt` as stored) and
  REPORT the mismatch; do not adopt lcms2's substitution."*

## 5. ★ Things the docs claimed that measurement contradicted

- **`NEXT_SESSION.md` §3: "saturation in B2A — never run". STALE.** A prior
  2026-08-12 `icc-conformance` session wired `(Intent::Saturation,
  tag::B2A2)` into `pass4b.rs` §A, measured it (**1,550 0×10⁻⁴**, 213 RGB
  points) and filed `TOLERANCES.md` §3.4.4.6. The librarian filed while
  `icc-conformance` was working in `tools/`, so it never reached
  `NUMERIC_CLAIMS.md`. **The ledger is the thing still missing, not the
  measurement.**
- **`NUMERIC_CLAIMS.md` §3.9.5's *"What settling A4b would do: one of the
  two implementations acquires a defect"* is FALSIFIED.** A4b settled and
  **neither** did — the clause does not bind readers at all. Third
  falsified-prediction instance (after DL-011/DL-012 and NA-006's
  "tetrahedral").
- **`iccce-cli`'s help text was wrong twice**, in the *shipped, public*
  binary: `8,700,267 px` (2481×3507 = **8,700,867**) and *"17-point grid"*
  (`recommended_grid_points` returns **33**, moved in `189e732`). Fixed.

## 6. NA-008 splits in two, and one half has no instrument

Gray as **destination** is reachable and cheap. `sRGB → ewgray22.icm`, 729
RGB points, **3,382×10⁻⁵** device max — and **no larger off the neutral
axis (3,247×10⁻⁵, 720 pts) than on it (3,382×10⁻⁵, 9 pts)**. That is the
*cross-check* half of NA-008 and it is now known to be cheap.

**The other half has no instrument.** NA-008 is the choice between `Y/Yn`
(PCSXYZ) and `L*/100` (PCSLAB) as the projection. **Every gray profile in
reach is PCSXYZ** — `ewgray18`, `ewgray22`, `BlackWhite.icc`, and both
synthetic `v2-gray-curv-*` fixtures *(verified — enumerated, headers read)*.
So the two projections **cannot be compared on the same input** until
`tools/gen-profiles` writes a **PCSLAB gray** fixture. Same shape as Pass
5's owed non-zero-black instrument: sourcing is not measuring, and neither
is agreeing with lcms2.

## 7. Method lesson that cost me a wrong number

A first gray probe fed iccce `0.5` and `transicc` `128` (= 0,501 96) and
produced a **1,9×10⁻³ "divergence" that was entirely the driver's**. Both
sides must get the *same* input expressed in each side's own convention —
integer codes, quotient at full `f64` precision. `transicc` takes RGB in
0..255 and CMYK/gray in 0..100; `iccce transform` takes 0..1 throughout.

Related: [[project-lcms2-findings-pass4-interpolation-and-v2-wtpt]],
[[project-lcms2-findings-pass4b-direction-dependence]],
[[project-oracle-and-tolerance-state]],
[[project-parallel-agent-build-collisions]].
