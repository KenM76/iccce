---
name: icc-bpc-sourcing-state
description: BPC is now SOURCED — Adobe 2006 and ISO/CD 18619:2013 are in _sources/. The forcing verdict is NO (neither document keys BPC to a profile version, so iccce's DL-022 never-force policy is corroborated); lcms2's estimation constants turn out to be ISO/CD 18619's verbatim; and a faithful ISO implementation will diverge from lcms2 at black by the black point's own chroma (2-6 dE76, LOUD)
metadata:
  type: reference
---

**Entry point: `D:\Dev\Rag-Specialized\ICC_Spec\icc\icc__ref__bpc.md`** (synthesis
+ verdicts, §15–§19 are the current material) →
**`icc\icc__s__bpc_algorithm.md`** (the two documents transcribed side by side).
**Do not re-do the sourcing sweep. Do not re-open §7.1/§7.2/§7.3 — they are
closed.**

## The four things that changed on 2026-08-12

1. **★ THE FORCING VERDICT — NO, and it does not change a shipped policy.**
   lcms2 forces `BPC = TRUE` at v4 perceptual/saturation *"following Adobe's
   document"*. **Adobe's document does not say that.** Of the three clauses in
   that source comment, **two are faithful** (devicelink excluded, absolute
   excluded) and the third is absent. Positively: Adobe **6.1** makes BPC
   user-deselectable, **6.2** says for perceptual *"BPC should not be
   necessary … available, however … to be used with malformed profiles"*,
   **6.3** declares profile version irrelevant, and ISO/CD 18619's **Annex A**
   calls BPC *"an optional feature that the user can enable or disable"*.
   **Exhaustive search of both documents: `ersion` 25 + 18 hits all read, zero
   `V4`/`force` — no version-keyed rule exists in either.**
   **⟹ iccce's `DL-022` (never force) is CORROBORATED; nothing reverses.**
   New register row **`A45`, RESOLVED-as-DELEGATED.** `M2` itself still stands
   — it was measured; only its **attribution** is retracted.
2. **★ `A42` UNVERIFIED → PARTLY RESOLVED, and lcms2 is a HYBRID.** ISO/CD 18619
   clause **4.2** specifies the whole estimation procedure in `shall` language,
   and **every threshold this corpus called "unattributed" is in it verbatim**
   (`0.2`, `≥ 4`, `[0.1,0.5)`, `[0.03,0.25)`, the `L* ≤ 50` clamps, 256 ramp
   samples, `±50` chroma clamp, `1.0E-10`, `max(0,min(50,·))`, `n < 3`).
   **Three constants are still lcms2's own and unattributed anywhere:
   `L* > 95 → 0`, `IsEmptyLayer`'s `0.002` (`M6`), and `n < 4` in the fitter
   (which contradicts ISO's `3` AND lcms2's own caller).**
   **lcms2 follows ISO for the destination curve fit and ADOBE for the darkest
   colour** — it has no `D()` vertex search.
   **Not RESOLVED** only because the held document is a **committee draft**.
   **Binding citation form: "ISO/CD 18619:2013 clause 4.2.x", NEVER
   "ISO 18619".**
3. **★ A LOUD prediction owed to `icc-conformance` — SOURCE-READ, not measured.**
   ISO 4.2.6 says the black points' `a`/`b` *"are ignored"*; **lcms2 retains
   chroma at three points and propagates it through a per-channel map.**
   DERIVED two ways: **at input black the divergence equals exactly
   `√(a*² + b*²)` of the detected destination black — 2 to 6 ΔE76** for `b*`
   between −2 and −6 — decaying to zero at white. **Scope: relative
   colorimetric, LUT destination, chromatic black.** If iccce implements ISO
   faithfully this WILL show up in a difftest and it is **not** an error.
4. **★ Vertex vs root, settled.** Adobe §7.2 Step 4 v takes the **VERTEX**
   (`−u/2t`) and openly calls it *"an approximation"*; ISO 4.2.5.5 takes the
   **ROOT** with a `|a| < 1.0E-10` linear branch and a `[0,50]` clamp; **lcms2
   does the root**, under a stale comment saying "vertex". **Implement the root**
   — the vertex is unbounded as the fit straightens, which is the common case.

## The tier answer (it gated NUMERIC_CLAIMS NC-084)

**`icc__ref__bpc.md` §2 (the scaling map) — YES, promotable.** Stated four ways
(ISO/CD 18619 4.2.6 · Adobe §7.3 · ICC.1 6.3.4.3 · lcms2) with **zero
disagreement**, proved symbolically (`sympy` → exactly `[0,0,0]`) and numerically
(`decimal` prec 50 → `1,0×10⁻⁴⁹`). **Cite ISO/CD 18619 4.2.6** — the only one
written as a runtime rule between two profiles.
**§3 (the `A41` perceptual black triple) — NO.** Neither BPC document mentions a
fixed perceptual black at all, so `A41` is untouched and anything resting on that
triple stays `derived-expectation`. **Two halves of one file, two tiers — saying
"the BPC file is primary now" would be false for half of it.**

## Document identity and licences — check before quoting

- **`_sources\BlackPointCompensation.pdf` is ISO/CD 18619, NOT WP40.** WP40 is
  its Bibliography `[1]` and is **superseded** — do not put it back on a wanted
  list. Lineage: Adobe 2006 → WP40 2010 → ISO/CD 18619 2013 → ISO 18619:2015.
- **`AdobeBPC.pdf` has TWO OPPOSITE grants: text reproduction PROHIBITED, but
  the ALGORITHM is patent-free and Adobe explicitly invites implementation.**
  Do not conflate them. Implementing is fine; carrying the prose is not.
- **`PDF20_AN001-BPC.pdf` is CC-BY-4.0** — the only freely quotable one.
  **PDF 2.0 binds `UseBlackPtComp` to ISO 18619:2015 by name**, and its default
  value `Default` means *"up to the PDF processor"* — a third corroboration of
  `A45`.
- Per-document provenance and terms: `D:\Dev\iccce\docs\LEGAL.md` **§2.5**.

## Extraction

**For the two BPC PDFs `pypdf` is the STRONGER engine and poppler the weaker** —
the inverse of ICC.1:2022. `pdftotext -layout` turns every non-Latin-1 glyph into
`U+FFFD` (20 / 49), losing `≤`, `×` and the **`U+2013` en-dash minus signs**.
**Run both anyway: their losses are complementary** (pypdf drops the inline
equation image `y = tx² + ux + c`; poppler destroys the `DecodeL` table's
row↔formula pairing). See [[icc-pdf-symbol-font-sign-loss]].

## Still true from before

- **`A28`**: ICC.1 states the *map* at **6.3.4.3** as an **authoring** fix-up —
  **do not cite 6.3.4.3 *as* BPC** (trap T2, the `C1` failure mode). But "no
  standard defines BPC" is **false**: the ICC standardised it outside ICC.1.
- **`A41`**: ICC.1 Table 16 prints `0,003 357 / 0,003 479 / 0,002 869`; lcms2 and
  iccDEV are byte-identical to each other at `0.00336 / 0.0034731 / 0.00287`.
  **Use the implementations' triple.** `0,037 ΔE76 / 0,050 ΔE2000` — exactly zero
  at 16-bit PCS, and on a float path the same order as a whole scenario budget,
  so **sub-perceptible ≠ negligible**.
- **`M6`**: `IsEmptyLayer` drops the stage below an L1 `0,002` ⟹ no BPC once the
  two blacks are within ≈`0,41 L*`; the same guard drops the ICC-absolute white
  stage. **SOURCE-READ, never run.** Still unattributed after this ingest.
- **`bkpt`**: parse and report it; do not use it.
- **`D11`/`A43` did NOT close.** Neither BPC document mentions clause 6.3.4.3,
  the PRM black, or any fixed/version-keyed black — so **both** the iccDEV and
  lcms2 routes are outside both documents. A43 stays SILENT for a
  better-understood reason.

Related: [[lcms2-measured-behaviour-file]], [[icc-tos-automated-access-blocker]],
[[verify-a-documents-identity-from-its-title-page]],
[[label-the-predicate-not-just-the-payload]]
