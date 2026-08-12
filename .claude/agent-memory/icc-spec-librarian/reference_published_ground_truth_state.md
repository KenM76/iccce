---
name: published-ground-truth-state
description: Published transform ground truth EXISTS — ICC.1:2022 Annex D.6.3, held and audited — but only for the colorimetric/encoding chain; for the LUT path it is STRUCTURALLY impossible because ICC.1 mandates no interpolation method. Includes what was checked and found empty, so the survey is not re-run.
metadata:
  type: reference
---

**Do not re-run this survey. It was done 2026-08-12 (13th pass) and the answers are here.** Full file: `ICC_Spec\icc\icc__ref__ground_truth_availability.md`.

## ★★★ YES, once — and this corpus had already found it and thrown it away

**`ICC.1:2022 Annex D.6.3`** prints an input, every intermediate, and **twelve exact integer PCS encodings**. Audited fixture: **`ICC_Spec\icc\icc__data__annex_d_worked_example.md`**.

- **Covers:** media-white scaling (Eq. D.3) → PCSXYZ → PCSLAB → 16-bit and 8-bit encodings, at paper white and 4-colour black. **Annex D is INFORMATIVE — say so on every citation.**
- **★ START THE FIXTURE AT TABLE D.3, never Table D.2.** Table D.2's black `X = 0,009 7` is defective (register **`A47`**); from D.3 all **12/12** of Table D.5's integers reproduce exactly.
- **★ Table D.4's black `b*` is `−0,3`. `pdftotext` prints `0,3`** (`U+F02D`); `pypdf` keeps it; D.5's `32 819` settles the sign with no extractor at all.
- **Second, NORMATIVE encoding check: ICC.1:2022 Table 16** — five published value↔encoding pairs, all five reproduced. Its `0808h` confirms the **v4 `65535`** L\* scaling (v2 legacy would be `0800h`).
- **The corpus's own `C5`:** §13.2 had disqualified this example since the 5th pass by point-evaluating interval-valued inputs. **Eleven filings of "no published-ground-truth row" rested on that error.** → [[derived-values-need-a-second-pass]], [[corpus-defects-are-caught-from-outside]].

## ★★★ NO, and it is EXISTENCE not availability — the LUT path

**ICC.1 mandates no interpolation method between CLUT grid points, so two conforming CMMs may legitimately return different numbers for the same profile and input. No single value COULD be published as the expected result.** Add: a profile is a vendor artifact; "perceptual" is vendor-defined by design; out-of-gamut handling is delegated; **and ICC's conformance clause binds only *reading*, so there is no clause a published output could be normative under.**

**Corroborated from the strongest possible direction: `iccDEV/Testing` (ICC's OWN reference implementation, BSD-3) ships ZERO expected colour values.** `RunTests.sh` compares nothing — its only stated expectation is the comment *"return of zero's indicates that something bad happened"*. `ApplyDataFiles/` holds **inputs only**. Both `.tsv` manifests are **structural validation verdicts**. **⟹ you cannot promote lcms2 agreement to ground truth by finding a better implementation.**

**Right words for the ledger: not "not found" but "the specification is written so that none can exist."** The right response to a no-oracle incident is a **second implementation lineage** (iccDEV, BSD-3 — ICC's own, different lineage from lcms2). **Argyll stays BARRED (AGPL-3.0).**

## Checked and empty — do not re-check

| Route | Finding |
|---|---|
| **ECI / Fogra residual for `PSOcoated_v3` vs FOGRA51** | **Does not exist.** `PSOcoated_v3_info.pdf` **is inside the held zip** and states only: Heidelberg **Color Tool 17**, black length 9, black width 10, **TAC 300 %**, max K 96 %. **No accuracy claim of any kind.** |
| ISO 12647-x / GRACoL / SWOP / Fogra tolerances | **Category error, not an access problem.** They bound a *press* against an aim, never a *transform* against an input. **Do not spend money or operator time here.** |
| ISO 15076-1 | ICC.1 verbatim. Availability-blocked **and** redundant. |
| sRGB worked triples | **None published.** The free 1996 W3C/HP document has parameters only — and a **different breakpoint** (`0,003 04` / `0,039 28` vs IEC's `0,003 130 8` / `0,040 45`). **★ Every equation in it is a GIF; a text scrape yields zero numbers.** It *did* cross-verify the primaries and both matrices. |
| CGATS TR001 (would settle `A47`) | Via `color.org/chardata/` (**barred**) or purchase from APT. **Untried lead: the `targ`-tag trick — any freely-hosted profile built from TR001 carries the data inside it.** |

## ★★ What the FOGRA51 / `PSOcoated_v3` pairing CAN prove — measured, not argued

**Not ground truth for a CMM** (at a non-corner patch the profile's unpublished fitting error and the CMM's interpolation error are inseparable) — **but the 16 CLUT corners are interpolation-free by construction, and FOGRA51 contains all 16.**

**Measured 2026-08-12 from the held zip alone: max `0,5086` / mean `0,0754 ΔE76`.** The 8 corners with `K=0` land within `0,005 4`. **The sole outlier is CMYK 400 %, which is outside the profile's own declared 300 % TAC** — ECI's document explains it. ⟹ **a bounded cross-check: a CMM missing a corner by > ~`0,55 ΔE76` has an error of its own.** Call the tier `bounded-cross-check`, never `published-ground-truth`.

**★★ Two facts that fell out of the same measurement and are load-bearing elsewhere:**
1. **`D1` (the legacy `0xFF00` Lab encoding) is now confirmed against REAL PUBLISHED MEASUREMENTS.** Re-decoding with v4's `65535`: mean `0,0754 → 0,7642 ΔE76`, **10×** — and still **below the perceptibility line, i.e. quiet**. `PSOcoated_v3.icc` is **v2.4.0** (`02400000`), `mft2`, `desc`, no `chad`.
2. **The ICC-absolute direction `XYZ_abs = XYZ_rel × (mediaWhite / D50)` is confirmed empirically** — it returns the measured white to `0,005 4 ΔE76`; the inverse is off by ≈12. `wtpt` = FOGRA51's white patch verbatim.

## ★ Two NEW site postures (the ToS memory now has four shapes)

- **`itu.int` — WAF rejects every agent request.** HTTP 200 with body `The requested URL was rejected`. Nothing forbids it; the server just refuses. **BT.709 is operator-browser-only.**
- **`printtechnologies.org` (APT) — `robots.txt` is 24 lines of Cloudflare content-signal boilerplate with ZERO directives and no `Content-Signal:` line.** By its own clause (c) it *"neither grants nor restricts"*. **No permission inferred; nothing fetched.**
- **`w3.org` — permissive** (`robots.txt` disallows only WordPress/blog internals). `/Graphics/` fetched.

Related: [[icc-tos-automated-access-blocker]], [[measurement-profiling-sourcing-state]], [[derived-values-need-a-second-pass]], [[corpus-defects-are-caught-from-outside]], [[icc-conformance-clause-binds-only-reading]], [[icc-absolute-intent-clause-trap]]
