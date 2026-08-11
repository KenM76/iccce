---
name: icc-spec-corpus-sourcing-route
description: With the ICC spec PDF blocked, the ICC_Spec corpus is sourced from two permissively-licensed codebases on GitHub — ICC's own DemoIccMAX (BSD-3) and lcms2 (MIT) — cross-verified against each other
metadata:
  type: reference
---

**Corpus location:** `D:\Dev\Rag-Specialized\ICC_Spec\` (start at `index.md`, then `LEGAL_NOTE.md`).

**★ Since 2026-08-11 the PRIMARY source is available** — `_sources\ICC.1-2022-05.pdf`, human-downloaded by Ken (see [[icc-tos-automated-access-blocker]]). **Cite ICC.1:2022 clauses first.** The two-codebase route below is now a **cross-check**, not the primary route — but keep using it: it is what makes a disagreement with lcms2 detectable, and per project rule 7 such a disagreement is a finding. It also remains the only source for `textDescriptionType` ('desc'), which has no clause in ICC.1:2022.

**The two-codebase route** — two independent, permissively-licensed sources, fetched from `raw.githubusercontent.com`, which is **not** "the Services" under ICC's ToS:

| Source | Repo | Files that carry the facts | Licence |
|---|---|---|---|
| **ICC's own** `DemoIccMAX` → **now `iccDEV`** | **`InternationalColorConsortium/iccDEV`** (the `DemoIccMAX` path still redirects) | `IccProfLib/icProfileHeader.h` (header struct, all signature enums, tag-type structs), `IccProfLib/IccUtil.cpp` (D50, XYZ↔Lab, PCS scaling), `IccProfLib/IccTagLut.cpp` (parametric curve formulas), **`IccProfLib/IccCmm.h` + `IccCmm.cpp` (the CMM itself — perceptual black constants, the v2→v4 PCS black scaling of clause 6.3.4.3)** | BSD-3-Clause |

**★ Tool limit, verified 2026-08-11: GitHub *code search* returns ZERO results for `InternationalColorConsortium/iccDEV`** — even for symbols that demonstrably exist (`gh search code --repo … CIccCmm` → empty, while the identical query shape works on `mm2/Little-CMS`). The repo is not in the code-search index. **Use the contents API instead:** `gh api repos/InternationalColorConsortium/iccDEV/contents/<path> --jq '.content' | base64 -d`. Pin used for the BPC pass: `b5f8def112ff98764fdf64e12d5d948395d0b62c` (master moves daily).
| **Little-CMS** (v2.19) | `mm2/Little-CMS` | `src/cmspcs.c` (**the v2/v4 Lab encoding constants**), `src/cmsgamma.c` (parametric curves), `src/cmsvirt.c` (sRGB), `src/cmswtpnt.c` (**Bradford matrix**), `src/cmstypes.c` (mluc reader), `include/lcms2.h` (header struct, D50) | MIT |

**Ground-truth test data (published literature, real ground truth per project rule 3):** Sharma/Wu/Dalal 2005 CIEDE2000 — all 34 pairs at `https://hajim.rochester.edu/ece/sites/gsharma/ciede2000/dataNprograms/ciede2000testdata.txt` (HTTP 200). Already transcribed into `cie/cie__ref__delta_e.md`.

**Two caveats worth carrying:**
- **ICC's `icProfileHeader.h` is the iccMAX/v5 superset, not the ICC.1 header.** Its `icHeader` subdivides bytes 100–127 into v5 fields; for ICC.1 those are 28 reserved bytes. It also disclaims its own authority in its header comment. Cite it as *ICC-authored secondary*, never as clause text.
- **A C header encodes offsets and enums exactly and prose requirements not at all** — so every normative "shall", every required/optional tag rule, and every rendering-intent semantic is a gap on this route, not a fact.

**Tool limitations hit and worth not re-discovering:** `iso.org` returns **HTTP 403** to automated fetch; `brucelindbloom.com` fails SSL handshake (`HANDSHAKE_FAILURE_ON_CLIENT_HELLO`) — use CRAN `spacesXYZ`'s adaptation vignette as the independent Bradford corroboration instead.
