---
name: colour-space-signature-state
description: ICC.1:2022 Table 19 is TRANSCRIBED and primary — but it has NO component-count column; every named-space count is a join with Table 41. The header-vs-tag channel-count requirement does not exist (A48). '1CLR' is in ICC's own header and in no ICC.1 edition (A49).
metadata:
  type: reference
---

**Durable answer lives at
`D:\Dev\Rag-Specialized\ICC_Spec\icc\icc__s__colour_space_signatures.md`**
(written 2026-08-17 for an iccce public-API dispatch: a `Signature → component
count` accessor for `pdfce`'s `/ICCBased` `/N` check). Do not re-derive; grep it.

**The four facts most likely to be mis-remembered:**

1. **★ ICC.1:2022 Table 19 has three columns — type, signature, hex — and NO
   component count.** The eleven named spaces get their counts only from
   **Table 41 (clause 10.10, "lut16Type channel encodings")**, by *counting
   non-dash cells*. `'GRAY'` = 1 rests on one `K` and three dashes.
   **Any `Signature → count` map is a TWO-TABLE JOIN, not a transcription** —
   ambiguity **A50**. Label it that way wherever it is published.
2. **★★ The header↔tag channel-count requirement does not exist — A48.** Only
   `colorantOrderType` (10.4) and `colorantTableType` (10.5) carry
   "**shall** be in agreement with the data colour space signature of 7.2.6";
   `namedColor2Type` (10.17) carries a **should**; the LUT types carry only
   "each colour component **shall** be assigned … as shown in Table 41", which
   binds *assignment*, not *count*. **Clause 5 gives no conformance hook
   either** ([[icc-conformance-clause-binds-only-reading]]) — so the verdict is
   "report a warning", never "non-conforming".
3. **`'1CLR'` (`31434C52h`) is defined in ICC's own `icProfileHeader.h` and in
   lcms2, and is in NEITHER ICC.1:2022 Table 19 nor ICC.1:2001-04 Table 13.**
   `'FCLR'` = 15 is the ceiling; **there is no 16-channel ICC.1 signature.**
   iccMAX's `nc0000`…`ncFFFF` family (count in the low 16 bits) is a *different
   standard* — ICC.2, not held. → **A49**. Also lcms2-only: `'LuvK'`.
   Same family as the two known typos in ICC's header ([[icc-spec-corpus-sourcing-route]]).
4. **The PCS field is not a two-value enum.** 7.2.7, verbatim: in a
   **DeviceLink** profile it holds a **data colour space from Table 19**.
   Hard-coding `pcs ∈ {'XYZ ','Lab '}` rejects every conformant `'link'`
   profile — the class a print/PDF workflow actually ships.

**v2 vs v4: CHECKED, no divergence.** ICC.1:2001-04 Table 13 = ICC.1:2022
Table 19, all 25 rows, same hex; Table 48 = Table 41. Only the extension NOTE
narrowed. Filed in `icc__ref__v2_v4_divergence.md` as a **checked
non-divergence** so it is not re-opened.

**Cheap wins taken in the same pass** — `icc__ref__signatures.md`'s four hex
GAPs are closed from clause 9.2 (`'calt'` `63616C74h`, `'targ'` `74617267h`,
and `A2B*`/`B2A*`/`pre*` confirming the ASCII-derived values were right), and
`icc__s__header.md`'s `pcs` row no longer says "NOT SOURCED, flagged A6" — the
register had resolved A6 from clause **8.6** and nobody swept it back
([[a-retraction-is-a-grep-not-a-paragraph]], the same shape with a
*resolution* instead of a retraction).

Related: [[icc-spec-corpus-sourcing-route]], [[corpus-defects-are-caught-from-outside]],
[[icc-pdf-symbol-font-sign-loss]], [[icc-tos-automated-access-blocker]]
