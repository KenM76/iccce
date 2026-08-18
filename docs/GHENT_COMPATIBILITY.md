# Ghent PDF Output Suite 5.0 — compatibility, and what that word is
# allowed to mean here

**Opened 2026-08-17 by `icc-engineer`, at the operator's instruction.**
Companion to `docs/NUMERIC_CLAIMS.md` (which holds the ledger rows) and
`docs/TOLERANCES.md` (which holds the graded bounds). This file holds the
*reasoning*: what the Ghent suite is, which of its demands are this
project's, which are the consumer's, and precisely which claims the
evidence gathered so far will and will not support.

---

## 1. The instruction, and why the wording matters

> *"I know some things you stopped work on because they required physical
> testing that we don't have. We aren't going to aim for compliance like
> that. Just aim for compatibility."* — the operator, 2026-08-17.

Two words that are routinely conflated, and this document depends on
keeping them apart:

| | what it means | can this project claim it? |
|---|---|---|
| **compliance / certification** | a conformance body has assessed a workflow against its published criteria, generally by printing the suite and assessing or measuring the result | **No.** It needs a press or proofing device and, for anything colorimetric, an instrument. Neither exists here, and the operator has ruled the path out. |
| **compatibility** | the software accepts what the corpus contains and behaves as the corpus's design says a correct engine behaves, in the parts that can be checked without an instrument | **Yes, in a bounded subset** — and that subset is larger than it first appears. §4 measures it. |

The habit this instruction corrects is real: work has been parked in this
project because its verification looked unreachable. Certification being
unreachable does not make *capability* unreachable, and the two got
merged. The corrective is not to loosen the evidence rules — rule 3 still
governs every number below — but to notice which questions an instrument
was never needed for.

---

## 2. What the Ghent PDF Output Suite 5.0 is

A corpus published by the Ghent Workgroup (GWG) in 2016 for the graphic
arts industry, to let a user determine whether a PDF workflow behaves
conformingly to the PDF/X standards. It is a set of small PDF *patches*,
each testing one property, plus assembled test pages. The patches were
authored in Adobe InDesign CS6, exported to PDF/X-1a, PDF/X-3 or PDF/X-4,
and imposed with Callas pdfToolbox 9.

Local copy: `D:\Dev\temp\pdfce\Ghent_PDF_Output_Suite_V50_Testpages\`.

Three categories:

| category | contents | whose problem |
|---|---|---|
| `1-CMYK` | `DeviceCMYK` only — overprint, fonts, shadings, optional content, transparency blend modes, softmasks, JPEG2000/JBIG2, 16-bit images, `DeviceN` overprint | almost entirely **`pdfce`** |
| `2-SPOT` | `DeviceCMYK` plus a spot colour — spot/CMYK overprint, `DeviceN` support at 4/5/6 colorants, white overprint and knockout | **`pdfce`**, with a named-colour edge (§3) |
| `3-ICC-CMS` | `ICCBased` colours and colour management — ICC source profiles, ICC **v4** profiles as image sources, 16-bit `ICCBased` Gray/RGB/CMYK images, colour-conversion and output-intent change indicators, four different gray definitions | **the only category substantially ours** |

### 2.1 ★ The suite's own pass/fail signal is a human eye, and it states no number

This is the single most important property of the corpus for a project
whose rule 1 is *a wrong colour looks exactly like a right one*.

Each patch is built so that an incorrectly-rendering engine reveals a
**red X**. `Ghent_PDF-Output-Suite-V50_ReadMeFirst.pdf` instructs the
assessor to *"examine the pages with a viewing distance of 0.5 m / 20
inches"*, and then draws the line that matters:

> *"For some patches a faint X may appear due to rounding of values or
> different color management engines beeing used. This is not a failure of
> the tested features: A clear X indicates a failure of this feature. A
> faint X is NOT a failure of this feature."*

**So the suite supplies no tolerance, no reference measurement, and no
expected colour values.** It distinguishes "engines disagree slightly"
from "this feature is broken" by *visual salience*, not by ΔE. That has
two consequences, and both are binding on how this file may be written:

1. **Ghent can never be the source of a numeric expectation.** Any number
   in §4 comes from this project's own apparatus and inherits that
   apparatus's evidence class — it does not become ground truth by being
   measured on a Ghent file.
2. **Ghent *can* be the source of a categorical expectation**, and that
   turns out to be worth more than it sounds. §3.1's trap profiles are
   the case: the expected outcome there is a *swap*, which is a fact about
   the profile's declared content, not a measurement of anything.

### 2.2 Licence — binding, and it constrains where the bytes may live

`ReadMeFirst.pdf` and every per-patch readme carry the same notice:

> *"Without express written permission of the Ghent Workgroup it is not
> permitted to use this PDF Output Suite for anything but its intended
> purpose of testing workflow setup. The Ghent PDF Output Suite cannot be
> sold or used in any commercial context without previous written
> permission by the Ghent Workgroup."*

Using it locally to test a workflow is the stated intended purpose and is
therefore fine. **Redistributing it, or anything substantially derived
from it, is not.** On top of that, the ICC profiles embedded *inside* the
patches are third-party works with their own separate terms — Adobe's,
ECI's, Heidelberg's, X-Rite's.

Two further conditions, found on a closer reading and **both of which
bite harder than the sentence above**:

1. **The notice condition is affirmative, not merely a prohibition:**
   *"This legal notice must be included in all copies containing the whole
   or substantial portions of the Ghent PDF Output Suite."* So vendoring
   the patches into `tests/fixtures/` would oblige this repository to
   carry GWG's notice — and would still leave the "commercial context"
   restriction attached to files sitting inside an MIT distribution. That
   is not a licence conflict this project should create.
2. ★ **"Testing workflow setup" is narrower than "testing our library",
   and narrower still than "saying so in public."** The certification
   release directs *"solution vendors, developers or system integrators"*
   to a separate Compliancy programme negotiated with GWG (§7). Using
   the suite privately to check iccce's behaviour is plainly within the
   spirit. **Publishing a pass/fail claim, a screenshot, or the word
   "Ghent" in this project's `README.md` or any release material is not
   covered and would need GWG's written permission first.** Under the
   global claim-bearing-copy rule, any public "compatible with the Ghent
   suite" statement is a claim requiring that permission — it is an
   operator decision, listed in §8.

**Consequence, and it is the same posture as the project's other three
restricted corpora (`docs/NEXT_SESSION.md` §4):**

- The suite is **not** in this repository and must not enter it.
- Profiles extracted from it live in
  `D:\Dev\iccce-private-fixtures\ghent-v50\`.
- Any test that uses them resolves the directory through
  `$ICCCE_PRIVATE_FIXTURES`, then the default path, then **SKIPs**. CI is
  permanently in the skipping case, by design.
- ★ **No value may be copied out of that directory into this repository**
  — not into a test, not into a doc, not into a comment. A green run on a
  machine without the fixtures is evidence that those checks *did not
  run*, not that they passed.

### 2.3 ★ The one place Ghent contradicts itself, and it is on our topic

GWG 13.0 is the suite's only statement about rendering intent:

> *"If you see a any red X, then the defined source ICC profile for the
> above noted object type was not respected. **In case of a faint green X,
> the defined rendering intent "Perceptual" was not respected.**"*

But `ReadMeFirst.pdf` has already ruled that *"A faint X is NOT a failure
of this feature."* **So the suite signals an intent failure with the
symbol it has separately declared not to be a failure**, and the
practical effect is that intent handling is untested by the suite's own
pass criterion.

This is worth writing down rather than smoothing over. It means a
workflow can be "Ghent-clean" while ignoring the declared rendering
intent entirely — and it means that if iccce wants a claim about intent
selection, **Ghent cannot supply the criterion**; the claim has to be
graded against ICC.1's own text about which `A2Bx`/`B2Ax` table an intent
selects.

Three more readmes pre-excuse engine-to-engine colour disagreement in the
same way — the ICCBased blend-mode readme says *"A faint X is due to
differences in the CMM and does not indicate a failure"*, the JPEG2000
readme says only the fill colour should be judged, and the shadings
readme concedes its own reference image *"is a screenshot … the colors
and gradation can be slightly off"*. **Ghent is a smoke test with a very
well-designed smoke detector. It is not, and does not claim to be, a
colorimetric criterion.**

---

## 3. The boundary — which Ghent patches are actually this project's

The request channel's `README.md` already draws the `iccce`/`pdfce` line;
applied to the Ghent categories it disposes of most of the suite quickly.
Overprint, blend modes, knockout and isolated groups, softmasks,
transparency, optional content, font embedding and substitution,
shadings, JPEG2000 and JBIG2 decoding, and *finding* a profile inside a
PDF are all **`pdfce`'s**. They are compositing, structure and decoding —
not conversion.

**Sixteen of the ~48 patches touch colour conversion at all. Six are
genuinely ours.** That count, and the two tables below, come from a
readme-by-readme sweep dispatched 2026-08-17; the ownership calls are
mine.

> ### ★★ Dated correction, 2026-08-17 (later the same day) — **the sentence above is superseded on BOTH of its numbers. It is left standing because a corrected count with no visible predecessor cannot be audited**
>
> | | as first written | corrected | how |
> |---|---|---|---|
> | patches in the suite | *"~48"* | **51** | `1-CMYK` **27**, `2-SPOT` **8**, `3-ICC-CMS` **16** *(verified — `icc-librarian` enumerated `…\Categories\*\Patches\*.pdf` on disk, 2026-08-17)* |
> | touching colour conversion | 16 | **16 — unchanged, and it was exactly right** | it is the whole of `3-ICC-CMS` |
> | **genuinely ours** | **six** | ★ **five** | **GWG 23.0 is not ours** — §3.5, and **DL-059** |
>
> ★ **The `~` was doing real work and should not have been there.** An
> approximate denominator under an exact numerator (*"sixteen of the
> ~48"*) reads as a measured ratio and is not one. The 16 was counted;
> the 48 was estimated. **DL-053's mechanism in miniature** — the
> verification was real, the denominator was not.
>
> ★ **A corroboration worth recording, because it costs nothing and
> closes a loop.** The same enumeration returns **98 PDFs** across the
> whole tree — 51 patches, 2 font-report PDFs, 3 assembled test pages,
> 38 readmes and 4 top-level documents. **§4.1's independently obtained
> *"98 PDFs scanned"* — which came from `tools/ghent/extract_icc.py`
> walking the tree, not from a file count — agrees exactly.** Two
> different instruments, one number.
>
> **Neither the 51 nor the 5 is a numeric *claim* in this project's
> sense** (no tolerance, no measured error, no oracle), which is why
> `NUMERIC_CLAIMS.md` gains no row. It gains an **owed item**, **§7.20**.

### 3.1 Tier A — genuinely a CMM's problem

> ### ★★★ SUPERSEDED IN PART, 2026-08-17 — **the GWG 23.0 row below is WRONG and is left in place deliberately. See §3.5.**
>
> **This table has five rows, not six.** GWG 23.0 is device-space channel
> routing — the same boundary class as overprint — and it is `pdfce`'s.
> The row is **not edited**, because the wrong classification is the only
> record of how the error looked from inside, and because this project's
> practice is dated supersession rather than silent rewriting (§4.3,
> §4.5). **Do not quote the 23.0 row.** Full reasoning and the clause
> evidence: **§3.5** and **`ARCHITECTURE.md` §5, DL-059**.
>
> ★ **The other five rows are NOT re-adjudicated by that correction**,
> and their surviving Tier-A status should be read as *"not yet checked
> against the same test"*, not as *"checked and confirmed"*. The test —
> **name the clause and the standard that assigns the behaviour to a
> layer** — was never applied to this table when it was built. Applying
> it to the remaining five is owed (`NUMERIC_CLAIMS.md` **§7.20**).
>
> ★ **Read the second column's heading with its provenance attached.**
> *"the capability it demands of a CMM"* is **this document's** phrasing
> over **patch-readme** source material (§9 records §3.1 as
> `[REPORTED]`). It is not GWG requirement text, and §3.5 explains why
> that distinction is load-bearing rather than pedantic.

| patch | what GWG says it tests | the capability it demands of a CMM |
|---|---|---|
| **GWG 13.0** `ICC_Source_Profile` | *"proper handling of PDF files that contain color managed objects"*, using `ICCBased` RGB and `ICCBased` CMYK | parse an embedded source profile and **actually use it** rather than falling back to the `/Alternate` space. Demands ICC v2 `mft2` (4→3, grid 16) *and* v2 matrix + `curv`. ★ A **red X** means the source profile was not respected; a **faint green X** means the declared Perceptual intent was not — but see §2.3, which is why the intent half of this patch is not a usable criterion |
| **GWG 20.5** `ICC-V4-CMYK-Image` | ICC **v4** CMYK profile as an image source space | parse ICC v4.2 `lutAToBType`/`lutBToAType` (`mAB `/`mBA `), `mluc`, `chad`, `dictType`, and **tolerate four `gbd` gamut-boundary tags of ~245 kB each without choking** |
| **GWG 20.6** `ICC-V4-RGB-Image` | ICC **v4** RGB profile as an image source space | parse `parametricCurveType` (`para`), **function type 3 specifically**, plus `mluc`; evaluate matrix/TRC |
| **GWG 18.2** `16Bit_Images_ICCbasedGray` | 16-bit image in an `ICCBased` **Gray** space | parse a **4-tag** `prtr`/GRAY/XYZ profile whose entire transform is a 256-entry `kTRC` `curv` plus `wtpt`, and build a 1-D gray→PCS transform with **no CLUT at all** |
| **GWG 18.4** `16Bit_Images_ICCbasedCMYK` | 16-bit image in an `ICCBased` CMYK space at Relative Colorimetric | a CMYK→PCS→CMYK chain between two `mft2` profiles, honouring the declared RelCol intent (`A2B1`/`B2A1`) |
| **GWG 23.0** `Four different Grays` | `DeviceGray`, `DeviceCMYK`, `Separation/Black` and `DeviceN[/Black]` *"shall show the same visual result"*, for both line art and an image | **K-only preservation.** DeviceGray 50 % and DeviceCMYK 0/0/0/50 must land on the same output K. This is the classic black-preservation trap, and it is the one where a CMM that routes everything through the PCS fails visibly. **CMM policy, engine plumbing** |

### 3.2 Tier B — a conversion is in the path, the failure is the engine's

`ICCBased` overprint (13.2/13.3 — the check fires on `OPM` handling),
`ICCBased` blend modes (16.1/16.4), `ICCBased` softmasks (16.7),
JPEG2000 in `ICCBased` RGB (17.2), 16-bit `ICCBased` RGB (18.0, split:
sample decode engine, matrix/TRC ours), 16-bit `DeviceGray` (18.3), and
the two indicators (22.0/22.1). These are `pdfce` features that happen to
be *painted in* an `ICCBased` space.

★ **22.0 and 22.1 are worth understanding rather than dismissing.** They
detect *whether a conversion happened*, not whether it was right — GWG
says so directly: *"NOTE: This does not necessarily mean that the output
workflow is wrong!"* They are the concrete reason the `/OutputIntents`
question in the request channel is a **conformance** question and not an
accuracy one.

22.1 also supplies the **only explicit colorimetric number in the entire
suite**: a PDF `/Lab` space with `/WhitePoint [0.964203 1.0 0.824905]`,
`/BlackPoint [0.0 0.0 0.0]`, `/Range [-128 127 -128 127]` — D50. That is
a usable fixture for a PDF `/Lab` → OutputIntent-CMYK path.

### 3.3 The rest

The whole of `1-CMYK` and `2-SPOT` — 32 patches — has nothing to do with
this project. Two look deceptive and are not: GWG 3.0 compares 50 % K,
50 % Gray and 50 % spot black, which reads like a gray-equivalence test,
but every sub-test is worded as *"the overprints have not been honored"*;
and GWG 8.2's checkmarks appear **because** DeviceN can overprint CMYK
and vanish if DeviceN is converted — it tests *non*-conversion.

> **Dated correction, 2026-08-17: 32 → 35.** `1-CMYK` holds **27**
> patches and `2-SPOT` holds **8** *(verified — enumerated on disk;
> §3's correction block)*. The ownership call is unchanged; only the
> count was wrong.
>
> ★★ **And this section had the right instinct one table too late.**
> *"GWG 8.2 tests **non**-conversion"* is the correct reading, and it is
> the identical argument that disposes of **GWG 23.0** — a patch whose
> pass condition is that a route was **left alone**. §3.5 is this
> paragraph's reasoning applied to a patch that happened to sit in
> `3-ICC-CMS`, and the fact that it was not applied there is what
> **DL-059** is about: the category folder, not the mechanism, decided
> the classification.

### 3.4 ★ The intents the patches actually declare

Not stated in any readme; read out of the patch files. Recorded because
it is the closest thing the corpus has to a specification of what a
consumer will ask for:

| patch | declared intent | OutputIntent condition |
|---|---|---|
| 13.0 | **Perceptual** (both `/Intent` and the `ri` operator) | ISO Coated v2 300% (ECI) |
| 16.1 (ICC) | **Saturation**, ×18 | `FOGRA27` ⚠ |
| 16.7, 17.2, 18.4, 20.5, 20.6 | RelativeColorimetric | `FOGRA27` ⚠ / `Custom` / ISO Coated v2 300% |
| 18.0, 18.2 | Perceptual | `Custom` |
| 22.1 | **both Perceptual ×2 and RelativeColorimetric ×2, in one file** | ISO Coated v2 300% (ECI) |
| 16.4 (ICC) | none declared | `Custom`, empty RegistryName |

Every ICC-CMS patch embeds **ISO Coated v2 300% (ECI)** as its
`DestOutputProfile`. Two observations that matter to a consumer:

- **GWG 16.1 is the only patch in the suite exercising the Saturation
  intent — and it is also the patch whose readme pre-excuses CMM
  disagreement.** The one place saturation is tested is the one place a
  wrong answer is declared acceptable.
- ⚠ **Do not trust `OutputConditionIdentifier`.** 16.1 and 16.7 declare
  `FOGRA27` while their embedded `DestOutputProfile` is ISO Coated v2
  300% (ECI), a FOGRA39-class condition. 16.4 embeds Coated FOGRA39 as
  its `ICCBased` *source* while its output intent identifier is `Custom`.
  **Read the profile bytes, not the identifier string** — which is
  precisely rule 6's posture (report what is there; do not reconcile it),
  and is worth passing to `pdfce`, whose job the identifier is.

★ **One demand does land on this project's API rather than its
colorimetry**: several of these patches use **16-bit images**. The whole
of iccce's evaluation surface is `f64` (`docs/NEXT_SESSION.md` §0), so a
consumer feeding it 16-bit samples widens every one of them. That is an
API finding, it was already known from the channel, and Ghent
independently corroborates that it is not hypothetical.

### 3.5 ★★★ GWG 23.0 "Four different Grays" — **reclassified out of Tier A. It is device-space channel routing, and it is `pdfce`'s**

**Opened 2026-08-17 by `icc-librarian`. Full decision and the
generalisable rule: `ARCHITECTURE.md` §5, DL-059.** This section holds
the evidence; the decision log holds the reasoning about *why the error
was possible*.

#### What the row said, and what is actually true

§3.1 filed 23.0 as **Tier A**, calling it *"K-only preservation … the
classic black-preservation trap, and the one where a CMM that routes
everything through the PCS fails visibly. CMM policy, engine plumbing."*

> **There is no colour conversion in this patch's path at all.** All four
> gray definitions resolve to the same single-channel device answer
> **inside PDF**, by clauses of ISO 32000, before any CMM is reached.

#### The clause evidence — four legs, four clauses, none of them ICC's

Every clause number below was **re-settled by this librarian against
`D:\Dev\Rag-Specialized\PDF_Spec\`**, independently of the dispatch that
requested this filing:

| the patch's leg | governing clause | what it requires |
|---|---|---|
| `DeviceGray` → CMYK | **ISO 32000-1:2008 §10.3.3** = **ISO 32000-2:2020 §10.4.2.3** | `c = m = y = 0`, `k = 1.0 − gray`. A **`shall`** *(verified — `PDF_Spec\color\color__cie_based.md:549`)* |
| the same, **in a colour-managed workflow** | **ISO 32000-2:2020 §10.3.2** | *"If the native device colour space is CMYK, then converting colours in the DeviceGray colour space to that CMYK **should follow the method described in 10.4.2.3**"* — **and this sentence sits inside the ICC-enabled branch** `[REPORTED]` — `icc-spec-librarian`, not re-derived here |
| `Separation /Black` | **ISO 32000-1 §8.6.6.4** | where the device *has* the named colourant, the reader **shall ignore** `alternateSpace` and `tintTransform` (clause identity verified — `PDF_Spec\color\color__separation.md:4,13`; the ignore-rule `[REPORTED]`) |
| `DeviceN [/Black]` | **ISO 32000-1 §8.6.6.5** | the same disposal, for the `DeviceN` form `[REPORTED]` |
| `DeviceCMYK 0/0/0/K` | **ISO 32000-1 §10.3.1** | passed through unconverted `[REPORTED]` |

★★ **The second row is the one that would have been missed, and it is
the whole reclassification.** It is easy to accept the device rule for an
unmanaged workflow and assume ICC takes over once a profile is present.
**ISO 32000-2 routes gray→CMYK to the device rule inside its own
ICC-enabled branch.** The equivalence is therefore not an achievement of
a CMM; it is a property of the consumer's colour-space resolution.

#### The patch's own readme agrees, and names `DeviceCMYK` as the reference

`[QUOTED]` — extracted by `icc-engineer` with `pdftotext -layout` from
`…\3-ICC-CMS\ReadMes\GWG230_Four_different Grays_README.pdf`:

- *"PDF offers 4 color spaces for black or gray objects **that only
  render in the Black color channel**."*
- *"No matter in which color space a black or gray object is defined, the
  final rendiering shall show the same visual result."* (sic —
  *"rendiering"*)
- *"This PDF was created with InDesign using the export to PDF/X-1a
  **without performing color conversion**."*
- *"Usually, the object defined in DeviceCMYK should render as expected.
  If an X appears, the color definition… is handeld differently than the
  DeviceCMYK the object."* (sic)

★ **Read the last two together.** The file was authored *without colour
conversion*, and the **unconverted `DeviceCMYK` object is the reference**
the other three are compared against. That makes 23.0 a **non-conversion
test** — structurally the same as GWG 8.2, which §3.3 had already
classified correctly one table earlier.

#### ★★ The premise that failed: there is no GWG requirement "23.0"

`icc-spec-librarian` retrieved the **GWG 2022 specification** — the
current edition; **there is no GWG 2023** — and reports `[REPORTED]`:

- requirement identifiers are **`Dxxx` / `Rxxx`**; **no "23.0" exists**;
- the nearest construct is **`D0013 "Black Colour"`**, which *is* the
  four-way equivalence — but it is a **definition consumed by the
  overprint requirements R0009–R0015**. ★ **GWG's own specification files
  this under overprint**, which is the same boundary call this section
  reaches by a different route;
- GWG's actual `DeviceGray` handling is **`R0011`: ban it** for small
  black text, *because overprint will not always be honoured for it*.

⇒ **`n.m` is Output Suite *patch* numbering, not GWG requirement
numbering.** The *"shall show the same visual result"* quotation is
genuine and correctly transcribed — but its authority is **patch
documentation, not the GWG specification**. Those are different strengths
of claim, exactly as ground truth, cross-check and self-comparison are
(§9, and `NUMERIC_CLAIMS.md` §1).

> **A `shall` in a test patch's readme is the patch author's `shall`.**

#### ★ The sweep for *"GWG 23.0 demands…"* — the phrase does not exist

A sweep was requested for that phrasing, believed to have been *"written
repeatedly"* in this project. **It appears nowhere in this repository**
*(verified — the whole tree grepped for `GWG ?23`, `GWG230` and
`Four.different.Gray`)*. Every occurrence of `23.0` in the repository is:

| where | what it is | disposition |
|---|---|---|
| `GHENT_COMPATIBILITY.md` §3.1 | the Tier-A row | **superseded by this section** |
| `GHENT_COMPATIBILITY.md` §4.6, §5.6, §6 | *"not attempted"* statements | annotated in place, below |
| `NUMERIC_CLAIMS.md` §7.16 ×2 | a **de-duplication** audit item about §6's table, already discharged | **not affected** — it is about a repeated table row, not about ownership |
| `tools/difftest/src/passk.rs`, `passk_probe.rs` | in-flight code — see below | owed to whoever lands it |

**No attribution of a `shall` to GWG was found in any document.** What
exists is §3.1's **column heading**, *"the capability it demands of a
CMM"*, applied to all six rows — almost certainly what was remembered.
★ **Recording this is the point, not a formality:** a correction aimed at
a string a document does not contain is DL-048's failure arriving from
the other end, and the cure is the same — **read the destination before
filing against it.**

#### ★★★ What is NOT affected — the correction must not over-reach

1. **The ICC-side finding stands, untouched and independent.** **ICC.1
   contains no black-preservation construct in either edition checked**,
   verified exhaustively; the structural reason is that the PCS is three
   components, so every device→device transform is 4→3→4 and **K has no
   carrier**. Sources: `ICC_Spec` register entries **A51** and **A52**,
   and `icc__ref__black_preservation.md`.
2. ★★ **CMYK→CMYK black preservation remains genuinely this project's.**
   It is unimplemented and being built. **It is simply not what GWG 23.0
   tests.** Conflating the two would turn a boundary correction into a
   scope cut, and **no scope is cut here.**
3. **No number in §4 moves**, no tolerance changes, no ledger row is
   invalidated. §4.6's gray-source measurement is unaffected — see the
   note appended there.

#### ★★ A live wrong clause in in-flight code, owed rather than fixed

`tools/difftest/src/passk.rs` **reaches this same conclusion
independently and deserves the credit**: it refuses to assume the
boundary, measures *both* legs, and states that *"if that is the
operative rule, the leg belongs to `pdfce` and not to this project at
all."* **§3.5 is the answer to the question that module declined to
answer for itself.**

**Its clause citation is wrong**, and it is the citation the boundary
argument rests on:

- `passk.rs:227` cites **PDF 32000-1 §8.6.4.4**.
- **§8.6.4.4 is *DeviceCMYK Colour Space***, not a conversion rule
  *(verified — `PDF_Spec\iso32000\iso32000__s__8.6.md:52,150`)*.
- The correct clause is **§10.3.3** *(verified —
  `PDF_Spec\color\color__cie_based.md:549`)*.

★ **§8.6.4.4 is a known attractor.** The PDF corpus carries a standing
correction of the identical substitution for a different subject —
*"this material is §8.6.5.5, not §8.6.4.4"*
(`PDF_Spec\color\color__iccbased.md:15`). It is where a reader reaches
when the topic is "device colour spaces" and the rule actually lives in
§10.

★★ **Why this is owed and not corrected here.** At the time of filing
**`passk.rs` is in no commit** — the branch tip is **`506fcd3`**
*(verified — `.git/refs/heads/master` and `.git/logs/HEAD` read
directly)* — and `docs/` contains **no mention of Pass K, of black
preservation, or of the `TOLERANCES.md` §3.10.8 that the module cites**
*(verified — grepped; `TOLERANCES.md` §3 runs 3.1 … 3.9.8 and has no
§3.10)*, though the module is wired into `tools/difftest/src/main.rs` and
`lib.rs`. **Pass K is in flight in a concurrent session; nothing from it
is quoted or claimed anywhere in this document.** Registered as owed at
`NUMERIC_CLAIMS.md` **§7.20**.

★ **One adjacent trap, so the correction is not over-applied.** ISO
32000-1 **contradicts itself** between §11.5.3 NOTE 3 and §10.3.3 — but
on the **CMYK → gray** direction, for soft-mask luminosity
*(`PDF_Spec\iso32000\iso32000__s__11.5.md:292-298`)*. **That is the other
direction and a separate problem.** The **gray → CMYK** rule relied on
here is not disputed by it.

---

## 4. What was measured

**All figures below are dated observations at tip `e21154c`, branch
`master`, measured 2026-08-17 with the `--release` build of
`crates/iccce-cli`, by `icc-engineer`.** They are recorded here as
evidence with a date, per DL-034; they are not descriptions that a later
run will restate for itself.

### 4.1 The corpus, and how it was obtained

`tools/ghent/extract_icc.py` walks a tree of PDFs, decodes every stream
object, and keeps those whose bytes 36..40 are the ASCII `acsp` — the ICC
profile file signature, the one header field with a fixed value.
Deliberately, it does **not** model PDF colour-space syntax: which
reference an embedded profile hangs off (`/ICCBased`, an image's
`/ColorSpace`, `/DestOutputProfile`, an alternate, a group) is `pdfce`'s
half of the boundary, and the signature test finds all of them regardless.
Output is content-addressed by SHA-256 so byte-identical embeddings
collapse to one file, with a manifest recording every (PDF, object number)
that produced each.

Run over the whole suite:

**98 PDFs scanned → 121 embeddings → 20 distinct profiles.**

They are real-world producer output, which is what makes them worth
having — every profile this project had tested against previously was
synthetic (`tools/gen-profiles`), OS-shipped, or standards-body-issued:

- **CMYK output (`prtr`, PCS Lab)**: ISO Coated v2 (ECI) ×2, ISO Coated v2
  300% (ECI) ×2, ISO Coated, Coated FOGRA39 (ISO 12647-2:2004), Coated
  FOGRA27 (ISO 12647-2:2004), GWG_GenericCMYK
- **RGB display (`mntr`, PCS XYZ)**: sRGB IEC61966-2.1 ×2, Adobe RGB
  (1998) ×2, eciRGB v2, a Thunderbolt display profile
- **ICC v4.2.0, two of them**: `eciRGB v2 ICCv4` (matrix/TRC), and
  `GWG_ICC_v4_testprofile.icc` (X-Rite, CMYK `prtr`, 1.36 MB, 18 tags —
  `mAB` A2B0/1/2 at grid 7×7×7×7, `mBA` B2A0/1/2 at 17×17×17, plus
  `gamt`, four `gbd`, `view` and a `dict`-typed `meta`)
- **Gray output**: "Schwarze Druckfarbe - ISO Coated v2 (ECI)", GRAY→XYZ
- ★ **two deliberate trap profiles**: `RGB mntr mtx X (Switch red green)`
  and `CMYK prtr lut X (Switch magenta cyan)`

### 4.2 Parsing — 20 of 20, zero malformations

`iccce inspect` on each of the 20: **exit 0 in every case, and
`malformations: 0` in every case**, including both v4 profiles and the
1.36 MB X-Rite v4 CMYK profile, whose full tag table decodes.

**What this claim is:** the parser accepts, and reports no malformation
in, every ICC profile embedded in this corpus. **What it is not:** any
statement that the parsed values are *right*. It is an acceptance result,
and the honest strength of an acceptance result is that it would have
caught a refusal, a panic or a spurious malformation report — nothing
more. Its value is that the sample is somebody else's, which no previous
parsing evidence in this project could say.

★ **A negative result would have been the more interesting one**, and it
is worth recording that the sweep was capable of producing one: the
extractor deliberately does not filter out truncated or malformed
candidates (a profile whose declared size disagrees with its buffer is
passed straight through), precisely so that the parser gets the chance to
report a malformation rather than the script hiding it. None occurred.

### 4.3 ★ The trap profiles — a categorical expectation, no oracle needed

`RGB mntr mtx X (Switch red green)` is GWG's discriminator for GWG 13.0:
an engine that *ignores* the declared `ICCBased` source profile renders it
unswapped, and the X appears. The expected behaviour is a fact about the
profile's own tags, so this is one of the rare colour checks where the
correct answer is knowable **without an instrument and without a second
implementation**.

`iccce transform --src "RGB mntr mtx X (Switch red green)" --dst "sRGB IEC61966-2.1"`,
media-relative:

| input (source RGB) | output (sRGB) |
|---|---|
| `1 0 0` (red) | `0.000000 0.991915 0.000000` — **green** |
| `0 1 0` (green) | `1.000000 0.000000 0.000000` — **red** |
| `0 0 1` (blue) | `0.000000 0.224217 1.000000` |

The swap is honoured. The profile's own primaries corroborate it: its red
primary carries the largest `Y` and its green primary the largest `X` —
the two matrix columns are literally exchanged, and the three `X` values
still sum to ≈0.9642, so the profile is otherwise well-formed and
correctly normalised.

**The CMYK trap behaves the same way, and this arm is the stronger one
because it comes with a control.** `CMYK prtr lut X (Switch magenta
cyan)` (v2.2.0, `mft2` grid 16) and the genuine `ISO Coated v2 300%
(ECI)` from the same patch, both → sRGB, media-relative:

| CMYK in | through the **trap** | through **ISO Coated v2 300%** (control) |
|---|---|---|
| `1 0 0 0` (cyan) | `1.000000 0.000000 0.484125` — **magenta** | `0.000000 0.628503 0.892175` — cyan |
| `0 1 0 0` (magenta) | `0.000000 0.566646 0.865191` — **cyan** | `0.900907 0.047617 0.501718` — magenta |
| `0 0 1 0` (yellow) | `1.000000 0.929322 0.000000` | `1.000000 0.931777 0.000000` |
| `0 0 0 0` (paper) | `0.999910 1.000000 0.999878` | `0.999910 1.000000 0.999878` |

C and M exchange. The control is what makes this worth more than the swap
alone: it rules out the transform simply mangling everything.

★ **But the claim first written here — that "Y and paper white do not
move measurably" — was wrong, and wrong in the flattering direction.**
It was corrected on the same day by `icc-librarian`, from the numbers in
this very table. Accurately:

- **Paper white is identical to six decimals** between trap and control.
- **Yellow is not.** R and B match exactly; **G differs by
  2.455×10⁻³** (`0.929322` vs `0.931777`). That is **21.7× the
  1.13×10⁻⁴ that §4.5 treats as a meaningful quantity**, so it cannot be
  waved away as noise.

The corrected claim is still strong, and it is now the *right* claim:
the C/M exchange is of order 0.6–0.9 in channel value, roughly **250×
larger** than yellow's residual. Two different profiles are not obliged
to agree on yellow at all, so a small residual there is expected; what
the control establishes is that the C/M difference is categorical while
everything else is a residual two orders of magnitude smaller.

**Why this correction is left visible rather than silently applied.**
NC-194/195's whole value is that it is **categorical** — an expectation
taken from the profile's declared content, with no oracle and no bound.
*"Does not move measurably"* is a numeric aside, and a numeric aside is
exactly how a categorical row quietly acquires a bound that nobody
derived and no clause grades. The error was caught by re-reading the
table's own figures, not by any new measurement.

**Evidence class: the expectation is categorical and comes from the
fixture's declared content, not from an oracle and not from a
measurement.** It is a strong claim of a narrow kind. It establishes that
the source profile is being *used* — the very thing GWG 13.0 exists to
detect — and says nothing at all about how *accurately*.

The alternative candidate answer is named and genuinely distinguished: an
engine that dropped the source profile and treated the values as its
device alternate would return red for red and cyan for cyan.

**These are now permanently graded rows** (`icc-conformance`, Pass G,
2026-08-17), and the grading is stronger than the hand-check above:

- The expectation is taken from **ICC.1 6.3.4 / F.3**, which makes the
  expected PCS XYZ for device `(1,0,0)` **the `rXYZ` tag itself** — so
  the row is graded against the profile's own declared content, class
  `DerivedExpectation`. **Observed 0.0 at a 1×10⁻⁶ tolerance.**
- **The swap is established colorimetrically, not by reading the
  `desc` string**: the `rXYZ` tag has y − x > 0, which is a green
  primary. A row that trusted the profile's name would be asserting
  GWG's spelling, not the file's content.
- The named rival — an engine that ignores the source profile — has
  separation **0.472229 in the row's own metric, 472,229× the
  tolerance**, and it is *supplied* rather than derived from the run, so
  it cannot collapse to zero on the defect run it exists to detect
  (the `Separation::against` defect of §3.4 in `NEXT_SESSION.md`).
- **Injection-proven:** transposing `rXYZ`/`gXYZ` turns 52 rows red, and
  the trap rows fail at **0.472229 — the stated separation, to six
  figures**. The self-consistency rows stayed green, correctly: they are
  blind to a symmetric defect by construction.

★ **And this is the mechanism that makes a Ghent Level-2 check
automatable.** The artwork inside GWG 13.0 was authored *pre-swapped*, so
a CMM that correctly applies the profile cancels the swap and the X
disappears. A renderer can therefore sample the X region against its
surround and assert they match — no press, no proof, no instrument, no
human at 0.5 m. That is the joint exercise in §6's last row.

### 4.4 ICC v4 evaluation on a vendor-authored profile

`GWG_ICC_v4_testprofile.icc` (X-Rite, CMYK, v4.2.0) → sRGB, **perceptual
intent**, through the `mAB` A2B0 pipeline:

| CMYK in | sRGB out |
|---|---|
| `0 0 0 0` | `0.999910 1.000000 0.999878` |
| `0 0 0 1` | `0.103214 0.101364 0.096422` |
| `0.2 0.3 0.4 0.1` | `0.747153 0.643538 0.549868` |

Every v4 LUT profile iccce had previously evaluated came from
`tools/gen-profiles` — i.e. from us, which means a shared misreading of
the `mAB` clause would have been invisible. This is the first v4 LUT
evaluation on a profile authored by somebody else.

**Evidence class when first written: capability only** — nothing was
compared to anything, and plausibility is not evidence.

**That has since been raised.** `icc-conformance` ran the lcms2
differential on 2026-08-17 and built **Pass G, 72 graded rows**, taking
the suite from `pass=157` to **`pass=229 fail=0 skip=3 error=0`, exit 0**
[VERIFIED — I re-ran the gate bare and read the exit code myself].
Discriminating rows went **16 → 42**.

★★ **It ran over 11 of the 20 profiles, not over the corpus.** [VERIFIED
— I enumerated the SHA-prefixed identifiers in `passg.rs` and
`ghent_probe.rs` twice, the second time with a deliberately broader token
match in case my first pattern assumed a filename form.] **§4.1 and §4.2
are claims about 20 profiles; everything in this subsection is a claim
about 11.** The two must not be quoted as one coverage figure.

**The nine never put through the oracle**, by `desc`: `Coated FOGRA27`,
`ISO Coated` (v2.0.0), `ISO Coated v2 300% (ECI)` — **both copies** —
`ISO Coated v2 (ECI)` (the second copy), `GWG_GenericCMYK`, the second
`sRGB IEC61966-2.1`, the second `Adobe RGB (1998)`, and the Thunderbolt
`Display` profile.

Two observations, in opposite directions, and both belong here:

- **It is less serious than 11-of-20 sounds.** Several of the nine are
  near-duplicates of profiles that *were* exercised — the same `desc`
  differing only in the header's rendering-intent field. Coverage of
  distinct colour behaviour is higher than the file count implies.
- **It is not nothing.** **Four distinct CMYK print profiles**
  (`Coated FOGRA27`, `ISO Coated`, `ISO Coated v2 300%`,
  `GWG_GenericCMYK`) have no differential row at all, and
  `ISO Coated v2 300% (ECI)` is the `DestOutputProfile` of **every
  ICC-CMS patch in the suite** (§3.4). The one profile a Ghent-driven
  consumer is most certain to meet is among the unexercised.

★ **This was caught by a pre-registered check, not by review.** The
previous filing recorded, as an owed item, the question *"did the
differential run over the same 20 profiles?"* — written down **before**
the answer was known. Nobody re-reads a coverage claim they have already
accepted; writing the question down in advance is what made the gap
cheap to find. That is DL-042's mechanism used deliberately rather than
suffered.

★ **The result that matters, and it is a clean one.** The raw
disagreement between iccce and lcms2 on this v4 `mAB ` profile **is the
interpolation method and nothing else**. With lcms2's own CLUT geometry
substituted, the residual collapses:

| tag | raw | with lcms2's geometry | ratio |
|---|---|---|---|
| `A2B1` | 0.828444 | 4.6245×10⁻³ | **179×** |
| `A2B0` | 0.950274 | 3.9123×10⁻³ | **243×** |

And the size of the raw residual was **predicted from the CLUT's own
bytes plus two published interpolation algorithms, with no lcms2 output
in the calculation** — 0.828123 and 0.948160, accounting for what was
observed to **0.04 %** and **0.22 %**. That is the difference between
"we differ by 0.83 and don't know why" and "we differ by 0.83 because
ICC.1 mandates no interpolation method, and here is the arithmetic".

This closes a gap open since 2026-08-11: `TOLERANCES.md` §3.4.3 wanted
"any *real* v4 LUT profile", and until this corpus arrived there was
none — every v4 LUT iccce had evaluated came from `tools/gen-profiles`,
i.e. from us, so a shared misreading of the `mAB ` clause would have been
invisible.

★ **What still cannot be claimed.** The *structural* gate is the method
envelope × 1.25 and therefore **admits the whole legitimate difference —
it explicitly cannot claim agreement**, and is labelled that way. The
agreement claim rests on the substituted-geometry arm at a 2×10⁻²
tolerance, derived from lcms2's own quantisation floor (4096-entry curves
at ±1/65535, `u16` CLUT input, s15.16 evaluation, 4-decimal print), which
is ≥40× tighter than the gate it replaces.

### 4.5 ★★ The v2/v4 pair — a NEGATIVE result. It is not the instrument
### I claimed it was

**This section originally read "an instrument this project has not had"
and called it "the one finding here that is worth more than the corpus it
came from." That was wrong, and the correction is the most useful thing
in this file.** `icc-conformance` established on 2026-08-17 that the pair
cannot do the job I assigned it, for two independent reasons:

1. **The version gate is never exercised.** Both encodings put `wtpt`
   **at** the PCS white (residuals 1.526×10⁻⁵ and 5.396×10⁻⁶), so the
   v2-vs-v4 white-point handling that differs between the two ICC
   versions is a no-op for either file. ★ **No pair anywhere in this
   corpus differs only in version while encoding a non-PCS white** — so
   the corpus cannot test the thing the pair appeared to test.
2. **The two files differ in more than their version.** Their TRCs are
   differently *represented* — a 700-entry `curv` table versus a `para`
   function type 3. So any disagreement has **two candidate causes and
   the pair cannot separate them**. Under DL-033 a comparison whose
   candidates are not separable has unknown power, which is precisely the
   state this project treats as "not evidence".

**Why the error is worth leaving on the page.** The pair *looks* like a
controlled experiment — same vendor, same colour space, one variable
changed. It is the shape of a good instrument, and I filed it as one
before checking whether the variable it appeared to isolate was isolated.
**A plausible-looking instrument is as dangerous as a plausible-looking
colour**, and it fails the same way: silently, while producing a number.

The measurement itself stands, with a narrower claim attached: gridded
properly, **max 1.01×10⁻⁴** (iccce on both sides) and **2.29×10⁻⁴**
(lcms2 on both sides — so that arm is a statement about *the files*, not
about either engine). It is labelled `SelfConsistency` on every emitted
line, the weakest class in the module.

★ **My own first measurement of this gave 1.13×10⁻⁴ over 2,197 points
against `ISO Coated v2 (ECI)`, and the harness's gives 1.01×10⁻⁴. The two
runs differ in destination and grid, and I have NOT reconciled them.**
Recorded as unreconciled rather than quietly replaced: §5.2's rule is
that an unexplained *small* difference is unexplained, not agreement, and
that applies to two of my own runs exactly as it applies to an oracle.

The historical framing is kept below for the record.

Both were transformed to the same destination (`ISO Coated v2 (ECI)`,
media-relative, no BPC) over a **13×13×13 = 2,197-point uniform RGB
grid**, and the destination CMYK device coordinates compared:

> **max |Δ| = 0.000113 in destination device coordinates (0..1), over
> 2,197 points.** Worst point: v2 encoding
> `[0.431796, 0.871526, 0.994334, 0.907564]` vs v4 encoding
> `[0.431800, 0.871413, 0.994341, 0.907562]`.

**Evidence class — read this before quoting the number.** This is a
**self-consistency check between two files supplied by one vendor**. It is
*weaker* than a cross-check against another implementation (both arms are
iccce, so a defect common to both encodings' code paths moves both arms
together and is invisible), and far weaker than ground truth. What it
does establish is narrow and real: the v2 `curv`/matrix path and the v4
path do not disagree about a colour space that both files claim to
describe, to within 1.13e-4 device units on this destination.

It also does **not** state a ΔE. The comparison is in destination device
coordinates because that is what the CLI emits; converting 1.13e-4 CMYK
device units into a perceptual difference would require the destination
profile's own B2A behaviour and has not been done. **Do not restate this
figure as a ΔE.**

### 4.5b ★ Adobe's shipped sRGB and Adobe RGB, as embedded here, are
### defectively authored — and this is rule 7 running the other way

`icc-conformance` found that **`sRGB IEC61966-2.1` and `Adobe RGB (1998)`
as embedded in these PDFs encode `wtpt` = D65 while their colorant tags
sum to D50, with no `chad` tag to reconcile them.** ICC.1:2001-04 Annex
A.3.1.1 makes that a **defect of authorship**, and it produces a 0.207 /
0.205 device divergence at the ICC-absolute intent — **which settles in
lcms2's favour.**

Three reasons this is worth its own subsection:

1. **Rule 7 is symmetric and this is the direction nobody plans for.**
   The rule says disagreement with lcms2 is a finding, settled from the
   specification text. It is written in the expectation that iccce might
   be right. Here the specification says **lcms2 is right and the widely
   shipped file is wrong**, and recording that honestly is the same act.
2. **This is not a one-off system profile.** It is what a real producer
   embedded, **121 times across 98 files**. A CMM that "corrects" it
   silently would be hiding a defect from the only layer that could
   disclose it — rule 6, applied one level up from the parser.
3. **It is a consumer-facing hazard.** `pdfce` will encounter these exact
   bytes in real documents, and the visible symptom at absolute intent
   would look like a colour bug in whichever engine reported it.

### 4.5c ★ Why `--bpc` is accepted at perceptual and refused at
### media-relative — the inverse of what you would guess

`icc-conformance` flagged as unexplained that iccce **refuses `--bpc` at
media-relative and saturation for the v4 `mAB ` source, but accepts it at
perceptual.** That looks backwards. It is deliberate, documented, and
correct [VERIFIED — I reproduced all four intents and read
`transform.rs:447-470` and `bpc.rs:80`].

The refusal is by name, not by silence:

```
--bpc refused: black point not estimable within iccce's named subset
(A42); refused, not guessed
```

The reason the asymmetry runs this way:

- **At v4 perceptual there is an agreed fixed black** — the perceptual
  reference medium black — so nothing needs estimating. iccce uses the
  A41 triple.
- **At media-relative with a LUT source there is not.** The black must be
  *estimated*, and lcms2 does it with an **unattributed Lab ridge
  search**. iccce declines to re-implement an algorithm it cannot cite.

So the availability of BPC tracks **whether the black point is *defined*
or must be *guessed***, not how "colorimetric" the intent sounds. **The
intent where the answer is fixed is the one where BPC is available.**

★ Two things not to lose. First, the A41 triple is *"deliberately the
lcms2/iccDEV triple, not Table 16's text"* — so even the available case
rests on a **cross-check-class constant, not ground truth**, and a claim
built on it inherits that. Second, this is the refusal-not-repair posture
extended from the parser to the CMM, and it is exactly the behaviour that
looks like a missing feature until you ask what the alternative would
have been: a number with no provenance, in a place where a wrong colour
looks exactly like a right one.

### 4.6 Gray source

"Schwarze Druckfarbe - ISO Coated v2 (ECI)" (GRAY→XYZ, `prtr`) → ISO
Coated v2, media-relative: builds and evaluates, exit 0, monotone in the
expected direction (input `1` → `0.000000 0.000011 0.000005 0.000000`,
i.e. paper white; input `0` → heavy four-colour black). Capability only,
same caveat as §4.4. The GWG 23.0 "four different grays" question —
whether K-only content survives every route identically — is **not**
answered by this and is not claimed to be.

> **Note added 2026-08-17 (§3.5, DL-059).** That last sentence is still
> true and its *reason* has changed: GWG 23.0's question is **not this
> project's to answer**, because all four of its routes are resolved by
> ISO 32000 clauses inside the consumer. **The measurement above is
> unaffected** — it is a gray *profile* driven through the PCS, which is
> a real iccce capability; it is simply not the patch's mechanism.

---

## 5. What is NOT claimed

Written out explicitly because a compatibility document is exactly the
kind of artifact whose claims inflate on re-reading.

1. **Not certified, not compliant, not "passes the Ghent suite".** No
   patch has been *rendered* by anything. The suite's own pass criterion
   is a rendered page assessed by eye, and this project renders nothing.
2. **★ No unqualified accuracy claim, and the qualification is
   structural.** §4.4 now carries a real cross-check against lcms2 — but
   its *agreement* arm holds only **with lcms2's own CLUT geometry
   substituted**, and its structural arm admits the whole legitimate
   difference by design. The reason is not effort: **ICC.1 mandates no
   interpolation method**, so two conforming implementations may differ
   by the amount measured and neither is wrong. §4.6 is capability, §4.5
   is self-consistency and a **negative** result, §4.3 is categorical.
   ★ **`docs/NUMERIC_CLAIMS.md` NC-001 remains this project's only
   ground-truth row.** Nothing on this corpus changed that.
3. **No claim about the 16-bit path**, beyond noting that the corpus
   demands one and iccce's evaluation surface is `f64`.
4. **No claim about `Separation`, `DeviceN` or spot colour**, which is
   `pdfce`'s structure to model; `namedColor2Type` support in iccce is a
   separate question this corpus does not exercise.
5. **No claim that "0 malformations" means the profiles are well-formed.**
   It means the parser reported none. A malformation the parser does not
   yet look for produces the same output.
6. **No claim that the six Tier-A patches (§3.1) pass.** Their *profiles*
   parse and their *transforms* evaluate; four of the six (18.2, 18.4,
   20.5, 20.6) additionally need 16-bit sample handling and image
   plumbing that is `pdfce`'s, and GWG 23.0's K-only question has not been
   attempted at all. "The colour transform each patch depends on can be
   built and evaluated" is what §4 supports. It is less than "the patch
   renders correctly", and much less than "the patch passes".
   > **Amended 2026-08-17 (§3.5, DL-059): read "the six" as *five*.**
   > GWG 23.0 is not a Tier-A patch and never was; *"has not been
   > attempted at all"* was true and was **describing a debt this project
   > does not owe**. ★ That is the precise hazard DL-059 names: an
   > over-claimed boundary produces a *"not attempted"* line that looks
   > like diligence and can be carried for ever, because **nothing fails
   > when you do not do work that is not yours.** The clause *"much less
   > than the patch passes"* is unchanged and still governs the five.
7. **No claim derived from §3.4's declared intents.** That table records
   what the patches *ask for*; nothing here checks that iccce selects the
   corresponding `A2Bx`/`B2Ax` table when asked. §2.3 explains why Ghent
   cannot be the criterion for that check even in principle.

---

## 6. What would raise each claim, cheapest first

| claim | today | what would raise it |
|---|---|---|
| v4 LUT evaluation (§4.4) | **done** — cross-check against lcms2, Pass G, 72 rows | the remaining ceiling is structural: ICC.1 mandates no interpolation method, so a second *lineage* (iccDEV, `NEXT_SESSION.md` §2.3) is the only thing that raises it further |
| the X-Rite `mBA ` (B2A) direction | **ungraded** | its `B2A0` carries a tabulated 4096-entry B curve that nothing in the suite currently evaluates |
| §B's `mft2` B2A rows | graded, but **no agreement claim** | the harness has no `mft2` B2A model, so there is no attribution row; its 17–63× margin is explicitly *not* offered as evidence (§5.2) |
| whether lcms2's forced BPC at v4 perceptual is policy or requirement | unsettled | a librarian dispatch on ICC.1's text — `icc-conformance` had no `Agent` tool and could not raise it |
| v2/v4 consistency (§4.5) | **negative result** — not a usable instrument | needs a pair differing *only* in version while encoding a **non-PCS white**. No such pair exists in this corpus |
| parsing (§4.2) | acceptance, no malformations | a v4 profile with a *known* defect, to show the sweep can report one on this corpus and not only on synthetic input |
| intent selection (§3.4) | not checked | grade it against **ICC.1's** clause on which table an intent selects — not against Ghent, which §2.3 shows cannot supply the criterion |
| the `FOGRA27` identifier mismatch (§3.4) | **[REPORTED]**, not re-derived | re-read the two patches' `/OutputIntents` here before it is passed to `pdfce` as fact |
| ~~K-only preservation (GWG 23.0)~~ **WITHDRAWN 2026-08-17** | **not this project's claim to raise** | ★ **Nothing.** §3.5 / DL-059: all four routes are resolved by ISO 32000 clauses inside the consumer, so there is no iccce claim here to raise. The row is struck rather than deleted so the withdrawal is auditable. **CMYK→CMYK black preservation is a different subject and remains ours** |
| the whole suite | not rendered | a `pdfce` render of the assembled test pages, compared patch-by-patch against `Ghent_PDF-Output-Test-V50_ALL_REFERENCE.pdf`. **This is the real prize and it is a joint exercise**, which is what the request channel is for |

★ §6's last row is the one to put to `pdfce`, and §4.3 shows it is
tractable rather than aspirational. The suite ships an `ALL_REFERENCE`
PDF alongside `ALL_X4`, and GWG's trap profiles make the judgement
mechanical:
the artwork is authored pre-swapped, so a correct conversion makes the X
vanish into its surround and an incorrect one leaves it visible. A
renderer that can produce both files has a self-contained visual
regression harness for the entire colour-managed pipeline — no press, no
instrument, no certification, no human at 0.5 m.

---

## 7. Certification — what it actually is, and why it is closed to us

The operator's instruction ruled certification out on the grounds that it
needs physical testing. Reading
`PR_Ghent-PDF-Output-Suite-5-Conformance-Certification.pdf` shows the
conclusion is right and the *reason* is only half right, which is worth
recording so nobody re-opens it on the wrong grounds.

**The document is a news release dated 23 November 2021 — 99 lines. It is
not a procedure, a criteria list or a test protocol.** It contains no
pass threshold, no measurement method and no submission process.

What it does establish:

- **Two levels, and level 2 is exactly our category.** *"Conformance
  Level 1: pages 1–4 which includes CMYK and Spot color tests /
  Conformance Level 1+2: pages 1–4 AND pages 5–6 which includes ICC based
  color tests. Conformance Level 2 involves color management and is
  therefore more demanding."*
- ★ **The programme is not open to a library.** *"The certification
  program is for companies not individuals and targeted to end users
  (e.g., print and packaging service providers). Solution vendors,
  developers or system integrators who wish to participate … should
  contact the GWG."* A component like iccce is categorically outside the
  certification programme; there is a separate, undocumented developer
  Compliancy programme reachable only by contacting GWG.
- **There is no single standard to conform to.** *"The certification is
  not directly offered by the Ghent Workgroup but by participating GWG
  members… They also determine the cost of the certification for their
  candidates."* Ten members each run their own.
- **The object under test is a workflow**, one that can *"process PDF/X-4
  print files"* — not a component.

**Does it require printing, proofing or measurement?** The release never
says so in as many words — a stated absence. But `ReadMeFirst.pdf` makes
physical output the intended medium (*"processed like regular PDF/X-4
print jobs (e.g. imposed to a sheet for offset printing…)"*) and
recommends proofing (*"it's also highly recommended to output the Ghent
PDF Output Suite test pages on a proofing device"*).

★ **The precise correction: what Ghent asks for is a *proof*, not a
*measurement*.** Nothing in the corpus asks for a spectrophotometer, and
§2.1 establishes there would be nothing to compare a measurement against
if you had one. So the barrier to certification is organisational (it is
sold to print service providers by third parties) and procedural (a
rendered page judged by eye), **not metrological**.

**The honest internal statement, and the form any external one must
take:** *iccce can demonstrate that it evaluates the Ghent Level-2
ICC-CMS patches' colour transforms without failure; it cannot be
Ghent-certified; and Ghent supplies no numeric criterion by which
"without failure" could be tightened into a colorimetric claim.*

---

## 8. Owed to the operator — decisions this session did not take

1. **Any public mention of Ghent needs GWG's written permission first**
   (§2.2). This includes `README.md`, release notes and crates.io
   metadata. Nothing has been published; nothing should be until asked.
2. **Whether to pursue the developer Compliancy programme** by contacting
   GWG (§7). It is the only route by which a public claim could become
   supportable, and contacting an external body is an operator act.
3. **Whether the joint render-and-compare exercise with `pdfce` is worth
   scoping** (§6, last row). It is the largest available win and it is
   not iccce's to start alone.

---

## 8.1 ★ An engineering decision I owe, and why it is waiting

`icc-librarian` filed NC-213 against me: **does iccce follow lcms2 and
substitute D50 for a mis-authored `wtpt`?** It registers no measurement
and changes no code, so **leaving it undecided is itself a position, and
today it is the shipped behaviour.** That framing is correct and is the
reason it is written here rather than left implicit.

The case is §4.5b's: Adobe's `sRGB IEC61966-2.1` and `Adobe RGB (1998)`,
as embedded 121 times in this corpus, encode `wtpt` = D65 while their
colorants sum to D50 with no `chad`. lcms2 substitutes; iccce does not;
the visible consequence is a 0.207 / 0.205 device divergence at the
ICC-absolute intent, and the specification settles it **in lcms2's
favour**.

**My provisional answer is: do not substitute — report.** Rule 6 says
the parser reports and does not repair, and a silent substitution is
exactly a repair: it takes a file whose two white-point statements
contradict each other, picks one, and destroys the evidence that they
disagreed. The right shape is a **named malformation** the caller can
see, so a preflight tool can say *"this profile is mis-authored"* rather
than a renderer quietly producing a different colour.

★ **But I am not filing that as the decision yet, for a reason that is
not caution.** The `icc-spec-librarian` dispatch now running is sourcing
sRGB's white point from W3C, the ICC registry and ITU-R BT.709 — which is
**the same subject**: what an sRGB profile's white point should be, and
what a conforming reader does when a file's `wtpt` disagrees with its
colorants. Deciding this before that lands would mean deciding it from
the one source the dispatch exists to stop depending on. **Two related
questions, one of them already being sourced — settle them together or
risk settling the second against the first.**

Note also the asymmetry that makes "report" the cheaper mistake: if
iccce reports and should have substituted, a consumer sees a diagnostic
and asks. If iccce substitutes and should have reported, **a defect in
121 embedded files becomes invisible and the colour is quietly
different** — and a wrong colour looks exactly like a right one.

---

## 9. Provenance of every statement in this file

Three different strengths appear below and are not interchangeable.
**`[VERIFIED]`** = `icc-engineer` ran it this session and read the output.
**`[QUOTED]`** = lifted verbatim from a document extracted with
`pdftotext -layout`. **`[REPORTED]`** = produced by a dispatched agent's
byte-level scan of the patch PDFs and **not independently re-derived
here** — which is a weaker claim, and is flagged wherever it is load-
bearing.

| § | statement | source |
|---|---|---|
| 1 | the instruction | operator, 2026-08-17 **[QUOTED]** |
| 2 | what the suite is, categories, patch list | `Ghent_PDF-Output-Suite-V50_ReadMeFirst.pdf` **[QUOTED]** |
| 2.1 | the 0.5 m rule, the faint-X rule | *ibid.* **[QUOTED]** |
| 2.2 | licence, incl. the affirmative notice condition | *ibid.* and every per-patch readme **[QUOTED]** |
| 2.3 | the intent/faint-X contradiction; the three pre-excusing readmes | `GWG130_ICC_Source_Profile_README.pdf`, `GWG161-164_…`, `GWG172_…`, `GWG060-061_Shading_ReadMe.pdf` **[QUOTED]** |
| 3.1 | the six Tier-A patches and their tag-level demands | readme sweep + patch-byte scan, dispatched 2026-08-17 **[REPORTED]**; ownership calls are `icc-engineer`'s. ★ **One of the six ownership calls was wrong — §3.5** |
| 3 / 3.3 | the patch counts **51 / 27 / 8 / 16**, and **98 PDFs** | `icc-librarian` enumerated `…\Categories\*\Patches\*.pdf` and the whole tree on disk, 2026-08-17 **[VERIFIED]**. The 98 independently agrees with §4.1's extractor-derived figure |
| 3.5 | **§10.3.3 / §10.4.2.3** (gray→CMYK), **§8.6.6.4** (`Separation` clause identity), and **§8.6.4.4 = *DeviceCMYK Colour Space*** | re-derived by `icc-librarian` from `D:\Dev\Rag-Specialized\PDF_Spec\`, cited to file and line **[VERIFIED]** — deliberately **not** taken from the dispatch that requested the filing |
| 3.5 | **§10.3.2**'s ICC-branch sentence, **§10.3.1**, **§8.6.6.5**, the `Separation` ignore-rule, and the **GWG 2022** `D0013`/`R0011` findings | `icc-spec-librarian` dispatch, 2026-08-17 **[REPORTED]** — **not re-derived here**. ★ §10.3.2 is the load-bearing one and is the one still owed a re-derivation (`NUMERIC_CLAIMS.md` **§7.20**) |
| 3.5 | the GWG 23.0 readme quotations | `GWG230_Four_different Grays_README.pdf` via `pdftotext -layout`, `icc-engineer` **[QUOTED]** |
| 3.5 | *"`passk.rs` is in no commit; tip is `506fcd3`"* | `.git/refs/heads/master` and `.git/logs/HEAD` read directly by `icc-librarian` **[VERIFIED]**. ★ This librarian has **no shell**; it is a statement about two files' contents, not about `git status`, and it does not survive the next commit |
| 3.2 | GWG 22.0's *"does not necessarily mean the workflow is wrong"* | `GWG220_ColorConversionIndicator_README.pdf` **[QUOTED]** |
| 3.2 | the `/Lab` D50 parameters in GWG 22.1 | patch bytes **[REPORTED]** — not re-derived here |
| 3.4 | declared intents, `DestOutputProfile`s, the FOGRA27 mismatches | patch bytes **[REPORTED]** — not re-derived here. ★ The mismatch claim is the one most worth re-deriving before it is passed to `pdfce` as fact |
| 4.1 | 98 PDFs → 121 embeddings → 20 distinct profiles; every `desc` | `tools/ghent/extract_icc.py` + `iccce inspect` **[VERIFIED]** |
| 4.2 | 20/20 exit 0, `malformations: 0` | `iccce inspect` **[VERIFIED]** |
| 4.3 | both trap profiles' swap, and the ISO Coated v2 300% control | `iccce transform` **[VERIFIED]** — this began as a **[REPORTED]** byte-level reading and was deliberately re-derived through the running code, because it is the section's strongest claim |
| 4.3 | the swapped profile's primaries summing to ≈0.9642 | profile bytes **[REPORTED]** — corroborative only; the swap itself is **[VERIFIED]** |
| 4.4–4.6 | every number | `iccce transform`, `--release`, tip `e21154c` **[VERIFIED]** |
| 7 | certification scope, levels, who may apply, who administers | `PR_Ghent-PDF-Output-Suite-5-Conformance-Certification.pdf` **[QUOTED]** |

★ **The re-derivation in §4.3 is the pattern to repeat.** A dispatched
agent reported that the CMYK trap swaps cyan and magenta. That report was
not treated as a finding until the same claim had been put through
`iccce transform` **with a control profile from the same patch** — and
the control is what turned "the values changed" into "exactly two
channels exchanged and nothing else moved". Verify in the running thing,
not in the report.
