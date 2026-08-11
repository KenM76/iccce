---
name: shared-layout-is-not-shared-semantics
description: When two structures share a binary layout (or one C struct), transcribe their semantics separately per clause — the shared struct is an authoring convenience and says nothing about meaning; this is corpus defect C4, which caused iccce GP-001
metadata:
  type: feedback
---

**A shared binary layout is not shared semantics. Transcribe per type, per clause, even when you expect the sentences to be identical — and when they do turn out identical, write "both clauses state X" rather than stating X once.**

**Why:** on 2026-08-11 `icc__type__lutAtoB_lutBtoA.md` carried **one** curve-count sentence governing both `lutAToBType` and `lutBToAType` — *"`A` curves = `inputChan`; `B` and `M` curves = `outputChan`"* — under a heading asserting that the **only** difference between the types was traversal order. ICC.1:2022 states the count **per type, per curve set**, in six sub-clauses:

| | `mAB ` (10.12.2/4/6) | `mBA ` (10.13.2/4/6) |
|---|---|---|
| A curves | `inputChan` | `outputChan` |
| M curves | `outputChan` | `inputChan` |
| B curves | `outputChan` | `inputChan` |

The corpus sentence was 10.12 generalised: **correct for `mAB `, wrong for `mBA ` in all three positions.** The generalisation came from ICC's published C header, where `icLutAtoB` and `icLutBtoA` are **field-identical**. They are identical because 10.12.1's own NOTE says the tag stores elements out of processing order "to allow for simplified reading and writing of profiles" — an *authoring convenience*, read as a statement about meaning.

**Why it hid for two passes:** the two readings coincide whenever `inputChan == outputChan`. Every square LUT agrees; the disagreement appears only on non-square `mBA `, i.e. **every real CMYK `B2A0`** — a population the project's profile sweep was already recorded as thin on. Filed as corpus defect **C4** (`icc__ref__spec_defects.md` §14); it produced iccce finding **GP-001** in `crates/iccce-profile/src/lut.rs`.

**The labelling discipline did not catch this one and could not have.** The byte tables around it were correctly marked `icc_secondary_code`, and A23/A24 were correctly open. **The offending sentence carried no marker at all** — unlabelled prose sitting beside correctly-labelled material, which is the shape that makes an unlabelled claim invisible. [[label-the-predicate-not-just-the-payload]] assumed the predicate would be labelled *wrongly*; here it was not labelled.

**How to apply:**
- **Defect signature to grep for:** a sentence that governs *two or more* named things while citing *one* clause number. Also: any heading of the form "the ONLY difference between X and Y is …".
- Never derive per-type behaviour from a shared struct in `icProfileHeader.h`, DemoIccMAX, or lcms2. The struct is layout evidence only.
- Related, from the same pass: the same file's element-combination list had **three** entries where 10.12.1 enumerates **four**, and stated the implications backwards (`A ⇒ CLUT`, not `CLUT ⇒ A`). Same mechanism — one summary standing in for per-clause text.
- **What caught it:** two consumers of one corpus sentence disagreeing with each other (a fixture generator vs. the parser, both built from this corpus). See [[corpus-defects-are-caught-from-outside]].
