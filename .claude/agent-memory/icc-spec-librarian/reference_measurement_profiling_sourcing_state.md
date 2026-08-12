---
name: measurement-profiling-sourcing-state
description: Pass 10 / iccce-measure sourcing — FOGRA51 is EMBEDDED in PSOcoated_v3.icc's targ tag (dataset + reference profile in ONE 1.8 MB free download); ECI permissive, Fogra robots-barred, Argyll AGPL-BARRED; CIE data tables held under CC BY-SA; and the industry's D50 Z is 0,82521 not ICC's 0,8249.
metadata:
  type: reference
---

**Everything below is on local disk. Check `_sources\` before re-fetching anything.**

## The one fact that unblocks Pass 10

**`PSOcoated_v3.icc`'s `targ` tag CONTAINS the complete FOGRA51 characterisation
dataset** — 123 463 bytes of CGATS/ISO 28178, **1 617 patches**, CMYK + XYZ + Lab
— and the profile built from that data is in the same zip. **Dataset and
reference profile are the same file, so no correspondence argument is needed.**

`D:\Dev\Rag-Specialized\ICC_Spec\_sources\characterisation\pso-coated_v3.zip`,
1 783 124 bytes, `sha256 5c8ed32d40949c2e8b84a03642ca7aadc4e3f153237cf439d3cdddffffd4ed95`,
from `http://www.eci.org/lib/exe/pso-coated_v3.zip`, 2026-08-12.
Full filing: `ICC_Spec\cgats\cgats__ref__characterisation_data_sourcing.md`.

## Who may be fetched, and why — the three postures

| Site | Verdict |
|---|---|
| **`eci.org`** | **★ AGENT RETRIEVAL OK.** `robots.txt` is `User-agent: *` / `Disallow:` — fully permissive. Everything under `/lib/exe/`. `pso-uncoated_v3_fogra52.zip` and `eci_cmyk_v2.zip` are one fetch away and untaken. |
| **`cie.co.at` / `files.cie.co.at`** | **★ AGENT RETRIEVAL OK.** `robots.txt` permits `/data-tables`; **`/terms`, `/legal-notice`, `/imprint`, `/copyright` ALL 404 — there is no prose ToS to find.** Data is **CC BY-SA 4.0** per-file in its own metadata. |
| **`fogra.org`** | **★ DO NOT AGENT-FETCH.** Free and no login, but **`robots.txt` disallows `/fileadmin/` and every dataset link is `/fileadmin/files/7_downloads/…`**. Prose T&C is silent. **Operator browser download only.** |
| **`registry.color.org`** | **BARRED permanently** — see [[icc-tos-automated-access-blocker]]. Not consulted; ECI was used instead. |
| **`iso.org`** | 403 to every agent tool, unchanged. |

**★ Fogra and color.org are inverses and the pair is the rule: at color.org the
prose barred and robots permitted; at fogra.org robots bars and the prose is
silent. The restrictive signal governs. "One of the two permits it" is not the
test.**

## ★★★ Argyll CMS is AGPL-3.0 — a STANDING BAR, filed at `LEGAL_NOTE.md` §1d

**Do not read, cite, paraphrase, or "just check how Argyll does it."** It is the
most complete free CGATS/ISO-12642/profiling implementation in existence and
therefore the most tempting reference in this whole subject area — which is why
the bar was written *before* the area was worked. iccce is MIT; AGPL-3.0 is
network copyleft and a derived parser or target layout cannot be cured later.
Licence verified 2026-08-12 from `argyllcms.com` + `/doc/License.txt`,
**no source read**. Permitted references: **lcms2** (MIT), **iccDEV** (BSD-3),
and **real files**.

## ★★ The industry's D50 is not ICC's — and this changes CGATS ingestion, not the PCS

FOGRA51's XYZ and Lab columns are internally consistent with
**D50 = `96,422 / 100,000 / 82,521`** at **1617/1617 patches on all three
channels, max residual `0,0050` = exactly the 2-dp rounding half-ULP** — versus
**651/1617** under ICC's `96,42/100/82,49`.

- **Do NOT change iccce's D50.** `0,9642/1,0000/0,8249` is the PCS white by
  definition; a CMM using another is broken.
- **DO use a CGATS file's `LAB_*` columns and never recompute them from its
  `XYZ_*`.** Cost of getting this wrong: ≤`0,21 ΔE76` / `0,033` mean, systematic
  and blue-yellow.
- **A FOGRA51 round-trip test has a ~`0,03 ΔE76` mean floor** from the data's own
  2-dp quantisation. A tighter tolerance is unachievable regardless of code.
- **Only the Lab→XYZ direction discriminates.** XYZ→Lab does not — its 2-dp
  quantisation swamps the effect. Both were run.

**Where ICC's `0,8249` comes from is a permanent silence** → register row
**`A46`**: ICC.1:2022 **6.3.1** delegates the PCS white chromaticity to
**ISO 3664** (paywalled), and no integration of CIE's own tables reproduces it.

## CIE data tables — held, and verified four ways

`_sources\cie_datasets\` — 1931 2° CMFs (1 nm), spectrum-locus chromaticities,
D50 SPD, D65 SPD, daylight S₀/S₁/S₂, **plus CVRL's independent republication**.
**md5 + sha256 + CIE's `sumOfColumns` + `sampleRow` all exact, 5/5 files.**
CIE vs CVRL agree to `5×10⁻¹³` over 1 413 values.
→ `ICC_Spec\cie\cie__data__cmf_1931_2deg.md`.

**★ CC BY-SA 4.0 vs iccce's MIT is a share-alike CONFLICT if a table is checked
into the repo.** Reading here is fine. Embedding is an **operator decision**.
**Never resolve it by retyping the numbers — that is laundering, not licensing.**

## Traps worth remembering

- **`CIE_cc_1931_2deg.csv` is NOT a CMF table** — it is spectrum-locus
  chromaticities. Columns are `x(lambda)`, not `x_bar(lambda)`.
- **The `targ` payload ends with a NUL byte** (ICC `textType` is NUL-terminated).
  A CGATS parser fed the raw tag must strip it.
- **`B2A2` in `PSOcoated_v3.icc` is an `mft1` (8-bit) while the other five LUTs
  are `mft2`** — a free fixture proving tag-type is not uniform within a profile.
- **CIE's own metadata has two defects**: D65's `S_D65` column header says
  `wavelength_first: 360` where the file starts at 300; the daylight-components
  description says "360 nm to 830 nm, 1 nm" where the file is 300–830 at 5 nm.
  **In both, a structured field and a prose field disagree and the prose is
  wrong** — the same shape as ICC's own Table 47 / 10.13.3 defects.

Related: [[icc-tos-automated-access-blocker]], [[icc-spec-corpus-sourcing-route]],
[[dont-transcribe-numeric-tables]], [[derived-values-need-a-second-pass]]
