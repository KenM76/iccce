---
name: lcms2-measured-behaviour-file
description: Where measured (executed, not read) lcms2 behaviour lives in the ICC_Spec corpus — the M-id namespace, the pin, and the forced-BPC quirk that contaminates every v4 perceptual cross-check
metadata:
  type: reference
---

**`D:\Dev\Rag-Specialized\ICC_Spec\icc\icc__ref__lcms2_measured_behaviour.md`** — the only place in the corpus where "lcms2 does X" may be stated as behaviour. Created 2026-08-11 after [[reading-source-is-not-observing-behaviour]] (C3). Id namespace **`M<n>`**; evidence tier `measured_impl_behaviour` (`LEGAL_NOTE.md` §3).

**Everything in it is pinned to lcms2 `21c582a594fe5279f90c0b93437c398f93bf62b0` (tag `lcms2.19.1`, 2.19), Windows/MSVC, 2026-08-11. Moving the pin invalidates every row until re-measured** — the tag is a mutable lightweight pointer, so the hash is the identity. Harness: `D:\Dev\iccce\tools\difftest` (README §12 is the run record; commit `bfd6b1e`); the pinned tree is on disk at `tools\difftest\vendor\lcms2` and can be re-grepped directly.

Rows as of 2026-08-11:

- **M1** — legacy Lab keys off **tag type** in lcms2 too. Spec and lcms2 **agree**; the corpus's predicted divergence is measured **absent**. `ncl2` and B2A were *not* measured (source-read only) — that residue is a gap row in the corpus index.
- **M2 ★** — **lcms2 forces BPC ON for v4 profiles at perceptual and saturation regardless of `-b`**, per "Adobe's document" not ICC.1, using a fixed `cmsPERCEPTUAL_BLACK` constant. **≈3.15 L\* at black.** **Consequence: any v4 perceptual/saturation comparison against lcms2 is partly measuring BPC** — check this before diagnosing a transform. Also the likely origin of the C1/C3 inference: lcms2 *does* key a decision on the profile version, just not the Lab one.

**Related corpus state:** BPC is RAG_PLAN Tier 2 and **not built** — neither Adobe's BPC document nor ICC's white paper has been obtained (needs an operator browser download), and ICC.1:2022 does not define BPC at all (**A28**). M2 records what the oracle does; it is not the algorithm's source.
