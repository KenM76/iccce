---
name: iccce-patch-named-for-what-it-looks-at
description: DL-059 (CORROBORATED 2026-08-18 by the artwork itself — the panels are 25%/0/0/0/75, i.e. ISO 32000's formula evaluated) — a test patch is named for what it LOOKS at, not for the layer that PRODUCES it; GWG 23.0 "Four different Grays" was filed as ours and is pdfce's, and the over-claiming direction of a boundary error is caught by NOTHING
metadata:
  type: project
---

**A conformance item's OUTPUT being a colour does not make it a
colour-conversion test. Before claiming one, name the CLAUSE and the
STANDARD that assign the behaviour to a layer.** If the clause is in
ISO 32000 it is `pdfce`'s; if it is in ICC.1 it is ours; if it is in
neither, nobody owns it and that is itself a finding.

**Why:** on 2026-08-17 `GHENT_COMPATIBILITY.md` §3.1 listed **GWG 23.0
"Four different Grays"** in **Tier A — "genuinely a CMM's problem"**,
glossed as *"K-only preservation … the classic black-preservation trap …
CMM policy, engine plumbing."* It is **device-space channel routing —
the same boundary class as overprint — and it is `pdfce`'s.** All four
gray definitions resolve to the same single-channel device answer
**inside PDF, with no ICC transform in the path**:

| leg | clause |
|---|---|
| `DeviceGray` → CMYK | **ISO 32000-1 §10.3.3 = ISO 32000-2 §10.4.2.3**, `c=m=y=0`, `k=1.0−gray`, a **`shall`** |
| the same, **colour-managed** | **ISO 32000-2 §10.3.2** — routes gray→CMYK to that same rule **inside the ICC-enabled branch**. ★ **The load-bearing clause and the one that gets missed** |
| `Separation /Black`, `DeviceN[/Black]` | **§8.6.6.4 / §8.6.6.5** — where the device has the colourant, ignore `alternateSpace`/`tintTransform` |
| `DeviceCMYK 0/0/0/K` | **§10.3.1** — passed through unconverted |

The patch's own readme settles it independently: the file was authored
*"without performing color conversion"* and names **`DeviceCMYK` as the
reference**. It is a **non-conversion test** — structurally identical to
GWG 8.2, which §3.3 of the same document had already classified
**correctly, one table earlier**. The category folder (`3-ICC-CMS`), not
the mechanism, decided the classification.

**★★★ The direction is the reusable part: this CLAIMED work that is not
ours.**

- An **under**-claim is caught by a consumer — `pdfce` hits the gap and
  files in the request channel.
- An **over**-claim is caught by **nothing**. There is no failing test
  for work you do not own. It produces a *"not attempted"* line that
  **looks like diligence** and can be carried for ever with **no number
  moving** (DL-055's shape in the scope register).
- ★ And it is not idle: building it would ship an **ICC path for a leg
  the standard routes around ICC** — a feature that would be *wrong to
  ship* and would *look right*, because its output is a gray that
  matches. Rule 1, one level above the pixel.

**★★ Second finding — attribution strength.** **There is no GWG
requirement "23.0".** GWG 2022 is current (**there is no 2023**);
requirements are `Dxxx`/`Rxxx`; **`n.m` is Output Suite *patch*
numbering**. The nearest construct, **`D0013 "Black Colour"`**, is a
**definition consumed by the overprint requirements R0009–R0015** — so
**GWG files this under overprint too**. ⇒ *"shall show the same visual
result"* is real but its authority is **patch documentation, not the GWG
specification**. **A `shall` in a test patch's readme is the patch
author's `shall`.**

**★★ Third finding — §8.6.4.4 is an ATTRACTOR.** `passk.rs:227` cites
**PDF 32000-1 §8.6.4.4** for the gray→CMYK rule; **§8.6.4.4 is
*DeviceCMYK Colour Space***, and the rule is **§10.3.3**. The PDF corpus
already carries a standing correction of the *identical* substitution for
a different subject (`PDF_Spec\color\color__iccbased.md:15`: *"this
material is §8.6.5.5, not §8.6.4.4"*). It is where a reader reaches when
the topic is "device colour spaces" and the rule lives in §10. **DL-057:
a wrong clause is worse than a vague one, because the citation is what
makes the argument persuasive.** ★ Credit where due — **`passk.rs`
reached the right boundary on its own**, refusing to assume and measuring
both legs; only its citation was wrong.

**★★ Fourth — the dispatch's own premise failed.** It asked to sweep the
phrasing *"GWG 23.0 demands…"*, said to be *"written repeatedly"*. **It
appears nowhere in the repository.** What exists is a **column heading**,
*"the capability it demands of a CMM"*. **No filing was made against it.**
DL-048 arriving from the other end: there a *citation* pointed at a
destination that did not hold the claim; here a *correction* was aimed at
text that was never written. **Both are settled by reading the
destination** — see [[iccce-stale-citation-worse-than-stale-number]].

**★★★ What this does NOT touch, and the misreading to refuse:**
**CMYK→CMYK black preservation remains genuinely this project's**,
unimplemented and being built. **ICC.1 contains no black-preservation
construct in either edition checked** (`ICC_Spec` **A51**/**A52**) — the
PCS is three components, so every device→device transform is 4→3→4 and
**K has no carrier**. GWG 23.0 is simply not what tests it. **A boundary
correction is not a scope cut.**

**How to apply:** whenever a corpus row, patch, or consumer request is
about to be filed as *"ours"*, ask **which clause, in which standard,
assigns the behaviour** — and write the answer into the row. Apply it
especially where the artifact **lives in a colour-managed folder** or has
a colour in its name; that is what made this one invisible. Treat a
`shall` quoted from test-suite documentation as **patch authority**, a
weaker class than specification authority, exactly as ground truth,
cross-check and self-comparison are three classes.

**★★★ CORROBORATED 2026-08-18 FROM A THIRD DIRECTION — the artwork.**
The readme's four panels, read off a raster: **`DeviceGray` 25 %**,
**`DeviceCMYK` 0/0/0/75**, **`Separation` 75**, **`DeviceN` 75**.
**`1 − 0.25 = 0.75` is ISO 32000's `k = 1.0 − gray` EVALUATED** — GWG
**authored the patch on the device-space rule**, so the equivalence it
tests is the PDF formula's own output and a CMM cannot be what produces
it. ★ **The order is what makes it worth having: the clause argument came
first and the artefact agreed afterwards.** Limit: **the README declares
this; the patch's content stream has never been read.**

**★★ It also made §3.1's row wrong TWICE** — it said *"DeviceGray 50 %
and DeviceCMYK 0/0/0/50"*. **Still not edited**; the supersession block
was extended instead. ★ **The wrong pair is the interesting half**, and a
checkable origin sits one table later: **§3.3 attributes "50 % K, 50 %
Gray, 50 % spot black" to GWG 3.0**, the patch that document itself flags
as the deceptive lookalike.

**★★ The `passk.rs` clause defect is DISCHARGED** (nine `§10.3.3` cites,
zero `8.6.4.4`; Pass K landed as `846952f`) — **and the discharge found
two NEW defects, both in PROSE**: `g = 0.5` called *"GWG's own patch
value"* **and printed into the report**, and an **`ICCBased`** panel the
readme does not list. See [[iccce-source-labelled-number]] (**DL-061**).

Related: [[iccce-source-labelled-number]] (DL-061),
[[iccce-compatibility-not-certification]] (DL-045/046/047, the
Ghent posture), [[iccce-wrong-clause-refusal-and-discarded-halves]]
(DL-057), [[iccce-stale-citation-worse-than-stale-number]] (DL-048),
[[iccce-gate-must-not-reward-deletion]] (DL-055, a change with no number
to record), [[iccce-count-from-a-sample-is-not-the-population]] (DL-053 —
*"sixteen of the ~48"* is an exact numerator over an estimated
denominator), [[iccce-pass-status]].
