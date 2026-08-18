---
name: project-passk-icc1-is-silent-on-black
description: ICC.1 says NOTHING about black preservation (closed negative, A51) — ICC's only black construct disclaims itself; the DeviceGray→CMYK rule is shall-level ISO 32000-1 §10.3.3 and belongs to pdfce; "GWG 23.0" is not a GWG id; and TWO different rules share the name "black preservation".
metadata:
  type: project
---

**Established by `icc-spec-librarian` 2026-08-17** and folded into Pass K.
Corpus file `ICC_Spec/icc/icc__ref__black_preservation.md`; register rows
**A51** (ICC.1 silent) and **A52**; divergence **D12**.

**How to apply — five things, and three of them corrected something already
written down.**

1. **The negative is CLOSED, not pending.** ICC.1:2022 **and** ICC.1:2001-04,
   whole-document, two engines each: `black.?preserv`, `preserve.*black`,
   `GCR`, `gr[ae]y component`, `K.only` → **zero hits in both**. Do not re-run
   the search. Two **v2-only** sentences carry the whole ICC story and v4
   deleted both: §6.4.45's `ucrbgTag` *"provides descriptive information only
   and is not involved in the processing model"* — **ICC's only black construct
   disclaims itself** — and §6.3.3.1's *"the output values are the control
   values and not the "K" (black) values"*.
2. ★★★ **THE BOUNDARY IS SETTLED AND IT IS NOT OURS.** `DeviceGray →
   DeviceCMYK` (`c=m=y=0`, **`k = 1.0 − gray`**) is `shall`-level **PDF**: ISO
   32000-1 **§10.3.3**, ISO 32000-2 **§10.4.2.3**; `Separation /Black` binds by
   **§8.6.6.4**. **All four "grays" agree inside the PDF processor before any
   conversion** — `pdfce`'s job, same boundary class as overprint. **iccce owns
   only CMYK→CMYK and the non-CMYK-native device.** ★ PDF also names the harm:
   §8.6.5.7 NOTE 2, *"results in a loss of fidelity in the black component"*.
   **I had cited "PDF 32000-1 §8.6.4.4"; that is wrong.**
3. ★★ **TWO different things are called "black preservation".** *K-only
   preservation* (lcms2 intents 10–12, Cholewo 2000) = *an already-K-only pixel
   stays K-only*, a **CMYK→CMYK** rule and ours. *"Gray maps onto K alone"* is
   the PDF device rule. **And inside the first there are two definitions under
   one name**: lcms2 maps K by **equal `L*`** on the K ramp; **Cholewo (2000) by
   the `K_MIN`/`K_MAX` ratio**. *State which iccce implements before any
   cross-check of the K value means anything.*
4. ★ **"GWG 23.0 (Four different Grays)" is not a GWG requirement id.** GWG 2022
   uses `Dxxx`/`Rxxx`; the four-way equivalence is **`D0013 "Black Colour"`**, a
   *definition consumed by the overprint requirements*. The `n.m` form is the
   **Ghent PDF Output Suite patch** numbering. **An engineer's citation label is
   a claim too** — this one arrived in the dispatch brief and propagated into a
   module header before it was checked.
5. **No published ΔE exists to grade against.** Cholewo prints visual figures
   only; lcms2 *computes* the ΔE of its own approximation and **discards** it
   (`// Error estimation (for debug only)`, and it is ΔE\*ab not ΔE2000). Zeng's
   SPIE papers are unread/paywalled, so "no number exists anywhere" is **not**
   claimed. lcms2's own header comments intents 10–15 `// Non-ICC intents`, and
   ICC.1 Table 23 permits only 0–3 in a header, so they cannot be stored in a
   profile at all.

Related: [[project-passk-black-preservation-baseline]],
[[project-lcms2-findings-pass5-bpc]] (a *different* "black" — do not merge).
