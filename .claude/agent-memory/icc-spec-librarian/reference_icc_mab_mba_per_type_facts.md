---
name: icc-mab-mba-per-type-facts
description: The load-bearing per-type facts about lutAToBType/lutBToAType that a shared reading gets wrong — curve counts, the four permitted element combinations, the matrix e10 name collision, and 10.13.3's self-contradiction; plus the note that ICC.1:2022 transcription debt is now zero
metadata:
  type: reference
---

**Corpus file: `D:\Dev\Rag-Specialized\ICC_Spec\icc\icc__type__lutAtoB_lutBtoA.md`** — fully transcribed from ICC.1:2022 on 2026-08-11 (7th pass), two extractors. The five facts below are the ones a from-memory answer gets wrong.

**1. Curve counts are PER TYPE** (10.12.2/4/6 and 10.13.2/4/6):

| | `mAB ` | `mBA ` |
|---|---|---|
| A | `inputChan` | `outputChan` |
| M | `outputChan` | `inputChan` |
| B | `outputChan` | `inputChan` |

The rule is *entry side counted by `inputChan`, exit side by `outputChan`* — not "A goes with input". CMYK `B2A0` (3→4): **B=3, M=3, A=4**. The wrong reading is invisible on every square LUT. See [[shared-layout-is-not-shared-semantics]].

**2. Four permitted element combinations per type, and the list is closed** (10.12.1 / 10.13.1). `mAB `: `B` / `M, Matrix, B` / `A, CLUT, B` / `A, CLUT, M, Matrix, B`. `mBA ` is the mirror. The constraint sentences are `A ⇒ CLUT` and `M ⇒ Matrix`, **not the converse**. DERIVED: `B` is in all four, so `offsetB == 0` is a malformation.

**3. Two per-type normative constraints with no counterpart:**
- **10.13.1**: `mBA ` "shall only be used when the PCS field in the header specifies either PCSXYZ or PCSLAB". **10.12.1**: `mAB ` "may be used independent of the value of the PCS field". (Bites on DeviceLink profiles.)
- **10.13.3**: "The matrix is permitted only if the number of output channels, or "M" curves, is 3." **10.12.5 has no equivalent sentence at all.**

**4. ★ 10.13.3 contradicts 10.13.4 inside one sentence.** 10.13.4 counts `mBA ` M curves by **input** channels, so on a CMYK `B2A0` the "output channels" half forbids the matrix and the ""M" curves" half permits it. **Implement the "M" curves reading** — the literal one rejects conformant CMYK profiles. Filed `icc__ref__spec_defects.md` §16.

**5. ★ Matrix element name collision.** ICC.1:2022 numbers the twelve `s15Fixed16` values **e1…e12**, with **e10/e11/e12 the three offset terms**. Row/column naming (`e00…e22` then `e03/e13/e23`) makes **`e10` mean two elements 24 bytes apart**. Byte order is the same under both; use the spec's `e1…e12` in any doc comment. Matrix output is **normatively clipped to [0,0 , 1,0]** per component before the next curve set — one of the few places ICC.1 says where clipping happens.

**Byte tables (Tables 45/46/47/48) had ZERO delta** against the previously code-derived versions — offsets 8/9/10/12/16/20/24/28, data at 32; CLUT 0–15 grid, 16 precision, 17–19 pad, 20 data. What was missing was normative wording, not geometry.

**Corpus status note (2026-08-11):** with 10.12/10.13/10.15 transcribed, `icc__ref__ambiguity_register.md`'s **transcription debt is zero** — every remaining UNVERIFIED row (A1b, A2, A4b, A31, A34, A39c) needs a *document* this corpus does not hold, not a clause it has not read. Register is 46 rows: 27 RESOLVED / 3 PARTLY / 7 SILENT / 3 BY DESIGN / 6 UNVERIFIED.
