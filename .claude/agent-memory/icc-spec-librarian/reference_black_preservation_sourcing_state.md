---
name: black-preservation-sourcing-state
description: Cholewo’s K_MIN/K_MAX ARE defined (per-colour, in Cholewo’s own words) and that is exactly what makes his method un-implementable as a policy — plus: black/K-only preservation is NOT in ICC.1 (both held editions, exhaustively searched) — the gray→CMYK rule is normative in ISO 32000 §10.3.3/§10.4.2.3 and belongs to pdfce; lcms2's intents 10–15 are self-labelled non-ICC; no published ΔE exists; and "GWG 23.0" could not be verified as a requirement
metadata:
  type: reference
---

**Corpus file: `D:\Dev\Rag-Specialized\ICC_Spec\icc\icc__ref__black_preservation.md`.**
Register rows **A51** (ICC.1 silent) and **A52** (ISO 32000-2 clause 10
self-contradiction); divergence row **D12** (both v2-only K sentences).

**★ The negative is CLOSED, not pending.** ICC.1:2022 **and**
ICC.1:2001-04, whole-document, two engines each: `black.?preserv`,
`preserve.*black`, `GCR`, `gr[ae]y component`, `K.only` → **zero hits in
both**. Do not re-run this search. The only remaining unsearched edition
is **ICC.1:2010-12**, which is *already* the top operator-download item
for A31/A47 — no new blocker was created.

**Two v2-only sentences carry the whole ICC story, and v4 deleted both:**
- **§6.4.45 `ucrbgTag` `'bfd '`**, VERBATIM: *"This tag provides descriptive information only and is not involved in the processing model."* — ICC's only black construct disclaims itself.
- **§6.3.3.1**, VERBATIM: *"Note: The output values are the control values and not the "K" (black) values."* — a monochrome profile's channel is **not** the K ink. `"control value"` has zero occurrences in ICC.1:2022.

**★★★ The boundary verdict, and it is the reusable part.** The
`DeviceGray`→`DeviceCMYK` rule (`c=m=y=0`, **`k = 1.0 − gray`**) is
`shall`-level **PDF**: ISO 32000-1 **§10.3.3**, ISO 32000-2 **§10.4.2.3**.
`Separation /Black` and `DeviceN [/Black]` bind to the K colourant by
**§8.6.6.4**'s `shall`. **So all four GWG "grays" agree inside the PDF
processor before any conversion — pdfce's job, the same boundary class as
overprint.** iccce owns only CMYK→CMYK and the non-CMYK-native device.
**PDF, not ICC, also names the harm** — §8.6.5.7 NOTE 2 (both editions):
4→3→4 "results in a loss of fidelity in the black component".

**★ Filing debt, deliberate: `PDF_Spec` holds NEITHER ISO 32000 clause 10
NOR §8.6.5.6/§8.6.5.7** — its own `iso32000__s__8.6.md` lists them as open
gaps. So the ICC_Spec file carries PDF clauses it should not own.
**Recommend dispatching `pdfce-librarian`** before anyone treats that as
corpus drift.

**lcms2 is a vendor extension and says so itself** — `include/lcms2.h`
comments intents 10–15 **`// Non-ICC intents`**, and the tutorial p.29
says ICC *"has tried to address such need but still there is nothing in
the spec"*. ICC.1 Table 23 permits only 0–3 in a header, so 10–15 cannot
be stored in a profile. **Its algorithm was SOURCE-READ, never run — no
`M<n>` id was issued** (per [[reading-source-is-not-observing-behaviour]]).

**★ Two different things are both called "black preservation", and the
dispatch's definition of "K-only" was wrong.** lcms2/literature K-only =
*already-K-only pixels stay K-only* (a CMYK→CMYK rule). "Gray maps onto K
alone" is the PDF device rule, a different layer. And lcms2 maps K by
**equal L\*** on the K ramp, while **Cholewo 2000** maps it by the
**K_MIN/K_MAX ratio** — two definitions, one name. State which one iccce
implements before any cross-check means anything.

**Numeric expectation: NONE exists, and that was verified rather than
assumed.** Cholewo has no ΔE (visual figures only). lcms2 *computes* the
ΔE of its own approximation and discards it — `// Error estimation (for
debug only)`, never exposed, and it is ΔE\*ab not ΔE2000. Nearest
published is **Sharma & Starr, JIST 54(6):060504, 2010** (open access) —
but it measures *ink optimisation*, and the paper disqualifies its own
digital numbers as self-referential. **Zeng's three SPIE papers are
UNREAD (paywalled), so "no number exists anywhere" is not claimed.**

**★ Premise check that failed, and it is worth carrying: "GWG 23.0
(Four different Grays)" is not a GWG requirement.** GWG 2022 IDs are
`Dxxx`/`Rxxx`; there is no 23.0. The four-way equivalence exists as
**`D0013 "Black Colour"`** — a *definition consumed by the overprint
requirements*, not a rendering requirement. The `n.m` form matches the
**Ghent PDF Output Suite patch** numbering. Settling it needs the Output
Suite download. *(Lesson shape: an engineer's citation label is a claim
too — see [[verify-document-identity-from-title-page]].)*

Site postures for `gwg.org`, `library.imaging.org`, `spiedigitallibrary.org`
are now in [[icc-tos-automated-access-blocker]]. **Use
`api.crossref.org/works/<doi>` for exact citations** — SPIE's own pages
return empty content to WebFetch.


---

## ★★ 2026-08-18 follow-up — Cholewo’s `K_MIN`/`K_MAX`, ANSWERED. Register row **A53**.

**The paper is NOW HELD:
`_sources/literature/cholewo_2000_cic8_preserving_black_separation.pdf`**
(with Sharma & Starr beside it). Do not re-fetch. Eq. (1) is **VERIFIED**,
no longer a reconstruction — 3 text extractions **plus a visual read of the
page rasterised with `pypdfium2`** (`pdftoppm` is NOT installed here;
`pypdfium2` IS, and **the Read tool renders its PNG output fine** — that is
the working route for typeset maths, and it is how the `≤` glyphs, which
every text engine drops, were recovered).

**The limits are DEFINED, per-colour, by Cholewo himself — VERBATIM §2.4:**
*“For each color the maximum (K_MAX) and minimum (K_MIN) amounts of black
with which it can be reproduced are determined … **In our method** K_MAX and
K_MIN are found by inverting the device model by constraining the solution
to have at least one of the CMY components equal to zero, and by penalizing
K > 0, respectively.”* **So “Hung 1994” is the CONCEPT credit only — the
construction is Cholewo’s and Hung is NOT needed to implement.** *(Shape of
the near-miss: my own earlier file said “attributed to Hung 1994”, which
reads as a deferral. **A credit is not a deferral** — check whether the
citing paper then says “in our method”.)*

**★★★ The finding that matters, and it inverts a test’s name: when source
≡ destination printing condition the four limits coincide and Eq. (1)
reduces EXACTLY to `K_d = K_i`, for every colour.** So on a same-press
fixture **“copy K through” IS Cholewo’s answer, to the bit** — it is not
the plausible-but-wrong candidate iccce’s suite names it. A wrong Cholewo
and a right one are indistinguishable there **by construction**. Cross-press
on the pure-K axis it does not copy through: `K_i = K_iMAX` ⇒ `K_d = K_MAX`
(destination), which reproduces the paper’s own VERBATIM headline claim —
the algebra and the prose agree, and that agreement is the cross-check.

**Why it still cannot ship as a “policy” beside lcms2’s:** lcms2’s K rule is
a **1-D tone curve**; Cholewo’s `K_d` is a **per-colour setpoint inside a
solver** — **five constrained optimisations per colour** over a **fitted
continuous MLAB forward printer model for BOTH devices** (not recoverable
from an ICC LUT), **six weights the paper leaves unspecified** (“can vary
depending on the desired color LCh_d”), a **soft** target the optimiser may
miss, and an **unguarded division by zero** when `K_iMAX = K_iMIN`. **Named
refusal is the supported outcome.** Three prerequisites are SPIE-paywalled
and NOT held: `10.1117/12.186294` (Hung 1994), `10.1117/1.482716` (Hung
1999, MLAB), `10.1117/12.373415` (Cholewo EI2000, the inversion).

**Corpus defect found in passing, flagged not fixed:** the ambiguity
register’s `revised:` line claims **56 rows**; a code recount finds **58
before A53 / 59 after**. Pre-existing; the kind-bucket counts may share the
drift. See [[corpus-defects-are-caught-from-outside]].

Related: [[icc-spec-corpus-sourcing-route]], [[lcms2-measured-behaviour-file]],
[[icc-bpc-sourcing-state]] (different "black" — do not merge them),
[[gap-vs-nonexistence-claim]].
