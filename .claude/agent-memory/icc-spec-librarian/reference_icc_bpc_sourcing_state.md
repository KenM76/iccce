---
name: icc-bpc-sourcing-state
description: What the corpus holds on black point compensation — the scaling map IS in ICC.1 clause 6.3.4.3, the black-point ESTIMATION is not sourced at all (A42), ICC.1's printed black-point digits differ from both reference implementations' (A41, and on a float path that is NOT negligible), and lcms2 drops BPC entirely below a 0,002 threshold (M6)
metadata:
  type: reference
---

**`D:\Dev\Rag-Specialized\ICC_Spec\icc\icc__ref__bpc.md`** — built 2026-08-11
(8th pass). **Read it before any BPC work; do not re-do the sourcing sweep.**

**Three facts that change how BPC is cited:**

1. **The BPC SCALING MAP is in ICC.1:2022, at clause 6.3.4.3.** Its
   `should`-level v2→v4 perceptual black adjustment
   `Xp = Xt·(1 − Xb/Xi) + Xb` is **algebraically identical** to lcms2's
   `ComputeBlackPointCompensation` with source black 0 (exact in `decimal`;
   `1,11×10⁻¹⁶` over 50 000 float64 draws). **`A28` was overstated** — it had
   said BPC is *"not in ICC.1 at all — confirmed by full-text search"*, and the
   phrase is absent while the mechanism is not. Now **PARTLY RESOLVED**.
   **Do NOT cite 6.3.4.3 *as* BPC**: it is an authoring fix-up on one profile
   with a *known* source black, not a runtime link with *both* blacks estimated.
2. **The black-point ESTIMATION step is NOT SOURCED — `A42`, and it gates
   Pass 5.** Nothing normative was obtainable. **An iccce BPC written today is
   a re-implementation of lcms2's BPC and its doc comments must say so.**
2b. **★ lcms2 can decide to do NO BPC at all (`M6`, added 2026-08-11 from the
   Pass 5 differential).** `cmscnvrt.c`'s `IsEmptyLayer` drops the computed
   stage below an L1 `0,002` ⟹ **no BPC once the two blacks are within ≈0,41
   `L*`**, silently; **the same guard drops the ICC-absolute white-point
   stage**. **SOURCE-READ, never run.** `icc__ref__bpc.md` §7.2 now lists it —
   the original list had missed it because that sweep covered `cmssamp.c` only.
3. **`A41` — the perceptual black constant.** ICC.1 Table 16 prints
   `0,003 357 / 0,003 479 / 0,002 869`; **lcms2 (`cmsPERCEPTUAL_BLACK_*`) and
   ICC's own reference implementation (`icPerceptualRefBlack*`) are
   byte-identical to each other at `0.00336 / 0.0034731 / 0.00287` and both
   differ from the spec.** The implementations' triple is a per-channel hybrid:
   **Y is the exact Lab inverse of Table 16's 8-bit `L*`**, X and Z are its
   `PCSXYZ` decimals rounded to 3 s.f. **Cost 0,037 ΔE76 / ΔE2000 0,050 2 —
   and exactly zero on any 16-bit PCS path** (both encode to
   `006Eh/0072h/005Eh`). **Use the implementations' triple.** **★ Sharpened
   2026-08-11: Pass 5 reproduced the corpus's ΔL*/ΔE76 by an independent route
   (Rust, through a fixture, on a grid) to 2×10⁻⁵ and added the ΔE2000. At
   0,050 the digit choice is the SAME ORDER as a float-path scenario's whole
   agreement budget — so "27× under a perceptibility threshold" must NOT be
   repeated as "negligible". Zero at 16-bit, permanent residue on float: the
   two statements are complements.**

**Sourcing outcomes — do not retry the barred ones:**

- **`AdobeBPC.pdf` is on `color.org`, NOT on `adobe.com`.** The ToS bar
  ([[icc-tos-automated-access-blocker]]) covers it. `adobe.com` is separately
  **unreachable** from this environment (curl HTTP 000, WebFetch timeout), so
  Adobe's own terms are **unknown**, not permissive.
- **`pdfa.org` (PDF 2.0 App Note 001 on BPC) and `iso.org/obp` return 403 to
  every agent tool — a tool limit, not a ToS bar.** A browser will likely work.
- **RETRIEVED and usable: `https://www.littlecms.com/BlackPointCompensationTests.pdf`**
  — Maria (2013), *"Validating the black point compensation standardization"*,
  by lcms2's own author, about implementing **the ICC document**; `robots.txt`
  is `Allow: /`. It is the corpus's only BPC document. **It forwards the
  curve-fit mathematics — needed in 26 % of its own 714 cases — to the barred
  Adobe paper, in an unresolved editorial comment left in the published PDF.**
- **`DemoIccMAX` was RENAMED to `iccDEV`** (`InternationalColorConsortium/iccDEV`,
  still BSD-3). **GitHub *code search* returns zero results for that repo** even
  for symbols that exist — **use the contents API** (`gh api repos/…/contents/…`).
  Update [[icc-spec-corpus-sourcing-route]] usage accordingly.

**Top operator download for BPC: `https://www.color.org/adobebpc.pdf`, and
start at its §7.2** — the only clause-level pointer into that document that
exists anywhere in either codebase (lcms2 `cmssamp.c`, in
`cmsDetectDestinationBlackPoint`).

**Two traps to check before blaming an iccce transform:**
- **`D11`** — the perceptual black moved between v2 and v4, and **iccDEV
  applies 6.3.4.3 at link time while lcms2 does not** (it forces BPC on at v4
  instead — `M2`). **≈3,14 `L*`, opposite signs.** See
  [[lcms2-measured-behaviour-file]]. **★ STATUS 2026-08-11: the two directions
  are now DISTINGUISHED BY MEASUREMENT — Pass 5 observed `3,137 348 L*`
  matching lcms2's M2 route (v4 DESTINATION forces BPC; lcms2 lighter at black)
  to 1,1×10⁻⁴, with iccDEV's route excluded independently by the reverse
  scenario. WHICH IS CORRECT is still open: A43 (silent) + the unread
  AdobeBPC/WP40. Do not restate D11 as "who is right unknown" — restate it as
  "directions distinguished, correctness unresolved".**
- **`M2` does NOT apply to v4 matrix/TRC profiles** —
  `cmsDetectBlackPoint` has a matrix-shaper escape to media-relative
  darkest-colorant estimation.
