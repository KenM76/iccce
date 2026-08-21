# The default destination — iccce constructs sRGB when the caller
# supplies none

**Decided by the operator, 2026-08-17.** This file is the durable record
of the decision, the conditions it carries, and the one edge that
determines whether it is safe. It answers the load-bearing design
question that `docs/NEXT_SESSION.md` §3.0 had listed as open, and that
`pdfce` asked in `request_iccbased_colour_spaces.md` under the heading
*"The one design question worth settling before code"*.

---

## ★★ STATUS: BUILT, 2026-08-17 — read this before the rest

**This block records what was implemented and the four things
measurement changed.**

> ★★ **Do NOT read this as vouching for everything below it.** It said
> *"Everything below is still the reference and still correct"* until
> 2026-08-17, and by the end of that day it was not: **§4.2 carried two
> claims this file's own later blocks had falsified**, and the blanket
> vouch is what kept them looking current. A supersession notice scoped
> to *"items 3 and 4"* under a header that vouches for *everything* leaves
> the rest endorsed. **Each section now carries its own status.**

| what | where |
|---|---|
| the constructed sRGB | `crates/iccce-cmm/src/builtin.rs` — `builtin::srgb()`, `SRGB_PRIMARIES_XY`, `SRGB_TRC_PARAMS`, `rgb_to_xyz()` |
| the API §5.1 declined to decide | `crates/iccce-cmm/src/transform.rs` — `Destination`, `DestinationProvenance`, `Chain::with_destination`, `Chain::destination_provenance` |
| the ΔE tests | `crates/iccce-cmm/tests/builtin_srgb_destination.rs` (6) and `builtin.rs`'s own module tests (8) |

**§5.1's API question is now decided, the way §2 argued:** a two-variant
`Destination` enum, **not** a bare `Option<&Profile>` — because an
`Option` being `None` cannot distinguish *"there was none"* from *"I
failed to get one"*, and the second must never trigger the fallback.
`Chain::new` is unchanged, so no existing caller moved.

### The four things measurement changed

1. **★ The ΔE cost is `0.033013 ΔE2000` maximum** against ICC's
   `sRGB2014.icc` over a 10-probe spread — 30× below the `1.0 ΔE2000`
   perceptibility threshold. **And the maximum is at pure white, where it
   is not a fact about iccce at all:** our construction lands on ICC D50
   exactly, and the reference file's colorant sum misses D50 by
   `1.9×10⁻⁴` in Z. Device **black is exact — `0.000000 ΔE2000`.**
2. **★★ The tolerance is DERIVED, not chosen.** The first draft asserted
   a flat `0.02` and **failed at `0.033`**. Investigating rather than
   moving the number (rule 5) produced a better test: the bound is now
   computed at run time as *the reference file's own white-point offset
   from D50*, and the assertion is that **no probe exceeds it** — i.e.
   iccce adds no error beyond what the reference already carries. That is
   falsifiable and tight; a flat number would have been neither.
3. **★★ §4.3's "do not let the two uses blur" acquired a sharper
   instance. `sRGB2014.icc` is not a second source.** It looks like the
   independent, better-authored reference the HP 1998 file is not — it
   has the compliant `wtpt` and the `chad`. **Measured: its `rXYZ`,
   `gXYZ`, `bXYZ` and all three TRC tables are BYTE-IDENTICAL to the HP
   profile's.** Only the header, `wtpt`, `bkpt` and `chad` differ. So the
   D50-colorant gap in `ICC_Spec/iec/iec__s__srgb.md` is **not** closed by
   it: there is exactly one **FILE** lineage for those nine numbers.
   ~~and no document publishes them.~~ **← FALSE within hours of being
   written; see item 1 of the SUPERSEDED block below.** ★ This is the
   *third* copy of that clause found still standing after the retraction
   — the others were in `builtin.rs` and in §4.2. **A claim repeated in
   three places is retracted in one and survives in two**, which is the
   argument for hunting the sentence rather than the paragraph.
   ★ A further route was tried and also fails: applying the file's **own
   disclosed `chad`** — strictly more information than the constants
   alone — to the BT.709 D65 matrix improves the residual from **12.0 ULP
   to 5.35 ULP and no further**, still concentrated in `bXYZ.Z`. The
   `chad` inverts to `xy = 0.312702 / 0.329020`, i.e. BT.709 D65 to
   quantisation, so it *is* a D65→D50 adaptation — just not linear
   Bradford to the ULP. **The residual's provenance remains unrecoverable
   from anything this project holds.**
4. **★★ The test suite had ZERO power against `0.03928`** — the constant
   §4.1 spends the most words on — and this was found by **injection**,
   not by inspection: swapping the breakpoint left all five original
   tests green. Two tests were added to close it, and the injection now
   goes red. In the course of writing them, a claim carried in the
   standards corpus was **falsified**: the error does *not* "affect 8-bit
   codes 10 and 11". `10/255 = 0.039216` is below both candidate
   breakpoints and `11/255 = 0.043137` is above both, so **no 8-bit code
   lands in the disagreement window and the separation at 8-bit precision
   is exactly zero.** The maximum separation anywhere is `7.55×10⁻⁷` in
   linear light. A wrong breakpoint is invisible to every image, every
   8-bit vector, every round-trip, **and every differential test against
   an implementation that made the same choice.**

### ★★★ SUPERSEDED SAME DAY — the operator obtained `srgb.pdf`, and it
### changes items 3 and 4 above

**On 2026-08-17 the operator downloaded ICC's own "How to interpret the
sRGB color space (specified in IEC 61966-2-1) for ICC profiles"** (Jack
Holm, ICC, 2015-04-27, 4 pp). Held at
`D:\Dev\Rag-Specialized\ICC_Spec\_sources\srgb_bt709\srgb_icc_specification_of_srgb_2015.pdf`.

**It closes two gaps this project had recorded as permanent, and it
reverses an attribution.**

**1. The D50-adapted colorants ARE published.** §B.2 prints the
Matrix/TRC matrix at **15 decimal places**. The corpus's standing claim
— *"NO document publishes them… only a real file does"* — was **false**.

**2. ICC's recommended D65→D50 `chad` IS published**, also at 15 dp,
§B.2. `ICC_Spec` recorded ICC.1:2022 E.4.2 as pointing at "a separate ICC
document, **not obtained**". **This is that document.**

**3. ★★★ The 12-ULP blue-Z residual is the FILE's error, not iccce's.**
Measured against the published values *(exact rational arithmetic,
2026-08-17)*:

| | worst cell | `bXYZ.Z` |
|---|---|---|
| **iccce's construction** | **3.02 ULP** | **0.90 ULP** |
| the shipped HP 1998 / `sRGB2014.icc` file | **11.13 ULP** | **11.13 ULP** |

**The most widely deployed sRGB profile in the world disagrees with ICC's
own published specification by 11 ULP in blue-Z, and iccce's
from-constants construction is four times closer to the published values
than the file is.** Every earlier route "failed to close" the residual
because it was being measured against an artifact that does not match the
specification either.

★ **The lesson, and it is the expensive one:** the residual was measured
against the only reference available at the time, and *the absence of a
published value was silently treated as evidence that the file was the
reference*. A gap in the literature is not a licence to promote an
implementation to ground truth. The mis-attribution survived because **the
number itself was correct** — we had the right residual and the wrong
owner for it.

**4. The remaining 3.02 ULP is fully explained.** ICC's construction is
exactly recoverable from ICC's own two printed matrices: their published
`chad` × the inverse of their §A.7 XYZ(D65)→RGB matrix reproduces their
published colorants to **`0.00 ULP`**. So the whole difference is *which
D65 matrix each side starts from* — ICC inverts their own matrix as
printed to 7 decimals; iccce builds it exactly from BT.709-6's
chromaticities. iccce's route carries no rounded intermediate and is
kept.

**5. ★ ICC's published `chad` does not quite reach ICC's own D50.**
`chad × D65 = (0.964150918938, 0.999997711611, 0.824943819994)` against a
stated `0.9642 / 1 / 0.8249` — off by `≈4.9×10⁻⁵`. Worth knowing before
treating the recommended matrix as exact.

**6. ★★ Two transcription defects in ICC's own document**, both of the
kind that propagate into implementations:
- **`BL = B/12.02`** in §B.1 — should be `12.92`. A typo in the
  *inverse* transfer function, in the blue channel only.
- **§B.1's three power-branch equations all read `R`**: `GL` and `BL` are
  both written as `((R + 0.055)/1.055)^2.4`. **This is the identical
  copy-paste error as W3C 1996 eq. (1.7b)**, which the corpus already
  records — two independent documents, same defect, same three lines.
  Anyone transcribing either one literally gets a greyscale transform.

### What is still open

- `IEC 61966-2-1` remains **paywalled and unobtained**. ICC's document
  *cites* it and restates its parameters, which is materially better than
  before but is still not the standard's own text.
- **No worked sRGB input→output triple exists in any source**, including
  this one. This remains a construction from constants checked against
  published *parameters*, never against a published *result*.
- `srgb.xalter` and the registry entry were not part of this download.
  Their remaining value is now much lower — the colorants they were
  wanted for have arrived.

---

## 1. The decision

> **"If the caller supplied destination doesn't exist then it should
> fallback to constructing sRGB internally."** — the operator,
> 2026-08-17.

Today the answer is the opposite, **by omission rather than by
decision**: `Chain::new` (`crates/iccce-cmm/src/transform.rs:246`)
demands two parsed `&Profile`s, and every `sRGB` anywhere in `crates/` is
a **test** reading the Windows system `.icm` off disk. No code path
constructs sRGB. That changes.

### 1.1 What it means, stated as a contract

- When a caller supplies a destination profile, **that profile is used.
  Always. No exception, no override, no "improvement".**
- When a caller supplies **no** destination, iccce constructs sRGB from
  published constants and uses it.
- The constructed sRGB is **computed, never loaded**. No file, no
  embedded profile blob, no redistribution of anybody's `.icc`.

### 1.2 Why this is the right call, on the consumer's evidence

`pdfce` renders to an sRGB `tiny_skia::Pixmap`. It does not know the
operator's monitor profile and — being a library that must run headless
in a CLI and in a **wasm32** web fork — mostly should not care. Its three
hard gates all favour a computed destination:

| gate | a shipped sRGB profile file | a computed sRGB |
|---|---|---|
| **wasm32-unknown-unknown builds** (CI-enforced) | needs the bytes reachable at runtime — `include_bytes!` or a file read | **no I/O at all** |
| **no copyleft in the tree** | depends whose profile you ship | **no dependency** |
| **no network client in `pdfce-core`/`pdfce-render`** — a clause the operator explicitly declined to narrow | irrelevant but tempting | **structurally impossible to need one** |

A computed destination also removes an item from `pdfce`'s portable
folder, which is a real deliverable and not a rounding error.

---

## 2. ★★ The edge that determines whether this is safe

**"Doesn't exist" must mean *absent*, never *unresolved*.**

This is the whole risk of the decision, and it is worth more words than
the decision itself.

A colour-managed document very often **declares** its destination. A
PDF/X file's `/OutputIntents` names a print condition and embeds the
destination profile; every ICC-CMS patch in the Ghent suite embeds `ISO
Coated v2 300% (ECI)` exactly this way
(`docs/GHENT_COMPATIBILITY.md` §3.4). If a caller hands iccce a document
whose declared destination **failed to parse**, or **was not looked
for**, or **was found and then dropped**, and iccce quietly substitutes
sRGB, the result is:

- a plausible-looking image,
- rendered to the wrong destination,
- **with the document's own declared print condition silently
  discarded**,
- and no error anywhere.

That is rule 1's exact subject — *a wrong colour looks exactly like a
right one* — and it would be caused by the fallback rather than caught by
it. GWG 22.1, the Output Intent Change Indicator, exists in the Ghent
suite **specifically to detect this failure in real workflows**.

**So the contract is:**

- The fallback fires when there is **genuinely no destination to use**.
- The fallback **must never fire as recovery from a failure to obtain
  one.** A destination that was declared and could not be parsed is a
  **named refusal** (`ChainError`), exactly as today. It is not a reason
  to substitute a different destination.
- Choosing between those two situations is the **caller's** judgement,
  because only the caller knows whether it looked. iccce's API must
  therefore make the caller *say* which case it is, rather than inferring
  it from an `Option` being `None` — an absent `Option` cannot
  distinguish "there was none" from "I failed to get it".

---

## 3. ★ The fallback is disclosed, not silent

**Rule 6 — *the parser reports; it does not repair* — has a CMM
analogue, and this is it.** A silent substitution of the destination is
the transform-layer version of a silently corrected tag: it hides a
material fact from the only layer that could disclose it.

`pdfce` independently asked for the same thing from the other side, in
item 3 of its request: *"a named refusal when the profile is unusable, so
pdfce can print **why** it fell back to `/Alternate` rather than silently
doing so. Your rule 6 and pdfce's rule 4 are the same rule from two
directions, and this is where they meet."*

**So a chain that used the constructed sRGB must say so, and the caller
must be able to ask.** The fallback is not an error — the operator
decided it should happen — but it is a fact about the conversion, and a
consumer must be able to log it, surface it in a preflight report, or
gate on it. A `pdfce` that cannot tell whether a page was rendered to a
document-declared destination or to iccce's default cannot honestly
report what it did.

This is the difference between a **default** and a **cover-up**, and it
costs one accessor to stay on the right side of it.

---

## 4. ★★ The sequencing gate — this cannot be built yet, and the reason
## is not bureaucratic

**The constants are not yet independently sourced.** As of 2026-08-17,
`D:\Dev\Rag-Specialized\ICC_Spec\iec\iec__s__srgb.md` carries, in its own
frontmatter:

> `evidence: impl_crosscheck (single source — NOT cross-verified)`
> **"IEC 61966-2-1 is paywalled and was NOT obtained. All values below
> come from one source: lcms2 (MIT). … A second independent source is a
> recorded GAP."**

**lcms2 is also this project's differential oracle.** Building a computed
sRGB on those constants today would put the oracle's own white point
underneath every conversion iccce then checks *against that oracle* —
and the resulting agreement would be evidence of nothing. That is exactly
the defect class in `NEXT_SESSION.md` §5.2: **agreement with the oracle
can be the symptom of a defect, not evidence against one.** A
non-conformant black-point estimator once landed 0.082 ΔE76 from lcms2's
answer while carrying a defect of 4.717 L\* — 57.8× the signal it
produced — and the differential had no power against it.

`NUMERIC_CLAIMS.md` records D65 as *"the weakest constant in
`iccce-color`"* and the shared-misreading risk as **ELEVATED** for this
precise reason.

**The remedy is already in flight.** At the operator's suggestion,
`icc-spec-librarian` was dispatched 2026-08-17 to source sRGB from three
free, independent routes:

1. **W3C**, `https://www.w3.org/Graphics/Color/sRGB.html` — the 1996
   Stokes/Anderson/Chandrasekar/Motta proposal that fed into IEC
   61966-2-1. **Not the standard**, but published by the standard's own
   authors.
2. **The ICC colour registry**, `https://registry.color.org/rgb-registry/srgb`
   — ICC's own registered characterisation.
3. **ITU-R BT.709**, free from the ITU — sRGB's primaries and white point
   *are* BT.709's, so this reaches the same chromaticities **from a
   different standards body entirely**.

### 4.1 ★★ SOURCING LANDED 2026-08-17 — the gate is lifted

**Plainly: the constants are now independently sourced enough to build
on.** Primaries and white point come from **ITU-R BT.709-6**, the
breakpoints from **W3C**, and neither is lcms2. **The oracle
contamination is gone.**

★★ **And the premise this section was written on was WRONG — mine, and
stated in the dispatch that went looking.** I wrote that "with the
rounded constants the two segments do not meet exactly, so some
formulations use a continuity-solved breakpoint instead." **Backwards.**
`0.04045` **is** the continuity-solved value. [VERIFIED — I re-derived
all of this at 50 decimal places, independently of the agent's report:]

| | condition solved | `a` | linear threshold | encoded threshold |
|---|---|---|---|---|
| **IEC / lcms2 / CSS** | **C⁰** — value continuity, `a` pinned at exactly `0.055` | `0.055` | `0.003130668442500634` → **`0.0031308`** | `0.040448236277108192` → **`0.04045`** |
| **W3C 1996** | **C¹** — value *and slope*, `a` solved too | `0.055010718947586597` | `0.0030412825601275209` → **`0.00304`** | `0.039293370676847569` → **`0.03928`** |

Measured discontinuity with `a` pinned at `0.055`:

- at `V = 0.04045` — **2.33×10⁻⁹**, some 6,500× *under* the 16-bit PCS
  quantum;
- at `V = 0.03928` — **−7.55×10⁻⁷**, **324× worse**.

**The two documents are each internally consistent because they solve
different equations.** The 1996 authors took the slope-matched breakpoint
*and* rounded `a` to `0.055` — two mutually inconsistent choices. IEC's
correction was to keep `a = 0.055` and re-solve for value continuity.
**Adopting `a = 0.0550107` would be a different curve, not a more precise
sRGB.** ★ `0.0031308` is therefore no longer `DERIVED` — it is the
rounded C⁰ solution and is now independently sourced.

**The W3C page carries a live erratum banner** — *"This document is
obsolete… During standardization, a small numerical error caused by
rounding error was corrected"* — and the sentence naming where to get the
correction is **commented out of the HTML**. A corpus pass had read past
it.

### 4.2 ★ The caveat that must travel into the implementation

> ### ★★★ CORRECTED 2026-08-17 — read this box before the section
>
> **This section was left standing when the `STATUS: BUILT` block at the
> top of this file was written, because that block scoped its
> supersession to "items 3 and 4 above". It did not reach here — and the
> header a reader passes on the way in says "everything below is still
> the reference and still correct."**
>
> That is the whole failure, and it is worth more than the correction:
> **a supersession notice that scopes itself narrowly, under a header
> that vouches broadly, leaves the stale text endorsed.** The reader is
> told twice that this is current, and once — in a block they may not
> reach — that part of it is not.
>
> **Two claims below are FALSE as of 2026-08-17:**
>
> 1. *"No document publishes the D50-adapted colorants at all — only a
>    file does."* **ICC's own "Specification of sRGB" (Holm, 2015) §B.2
>    publishes them at 15 decimal places.** See the `SUPERSEDED SAME DAY`
>    block at the top of this file.
> 2. *"Name the ~1.83×10⁻⁴ XYZ blue-Z difference as an approximation
>    under rule 4."* **That residual is the FILE's error, not iccce's.**
>    Measured against ICC's published values: iccce **3.02 ULP** worst
>    and **0.90 ULP** in `bXYZ.Z`; the shipped file **11.13 ULP**, all of
>    it in `bXYZ.Z`. The instruction above would have had the
>    implementation declare somebody else's defect as its own cost —
>    and it did, for most of a day, until the document arrived.
>
> **What survives unchanged, and is the reason this section is corrected
> rather than deleted:** the *"do not write a byte-equality test"* rule
> below is still right, and is now better justified — the file does not
> match ICC's published values either, so equality with it would be a
> **worse** claim, not a stricter one.

**A from-constants sRGB will NOT be byte-identical to the canonical HP
`sRGB IEC61966-2.1` profile**, and that is not a defect in either.

Reconstructing that profile's colorants from the sourced chromaticities
under ICC.1 Annex E.3 Bradford puts **8 of 9 cells within 2 ULP of
`s15Fixed16`; `bXYZ.Z` misses by 12 ULP (1.83×10⁻⁴)**, and no D50 tier
closes it. ~~**No document publishes the D50-adapted colorants at all** —
only a *file* does.~~ **← FALSE; see the box above.**

Consequences, binding:

- **Do not write a byte-equality test against the HP profile.** It would
  be testing HP's 1998 arithmetic, not iccce's correctness. **Still
  binding.**
- Test with a **ΔE round-trip**, stated tolerance, named oracle. **Still
  binding**, and now joined by a direct assertion against ICC's published
  colorants at a 4 ULP bound (3.02 observed).
- ~~**Name the ~1.83×10⁻⁴ XYZ blue-Z difference as an approximation under
  rule 4**, with what it costs in ΔE, in the doc comment.~~ **← FALSE;
  see the box above.** The approximation to name is the **3.02 ULP
  (`4.6×10⁻⁵` XYZ) difference from ICC's published colorants**, registered
  as **NA-011**.

★ And note the trap already recorded at §4.3 below: the Ghent corpus's
`sRGB IEC61966-2.1` files — and the canonical HP one — encode `wtpt` =
**D65** with **no `chad`**, which ICC.1 A.3.1.1 makes a defect. **The
most-deployed sRGB profile in the world does not comply.** It is a fine
test that iccce parses what real files contain and a **bad** reference
for what a constructed profile's white point should be.

**Build order, therefore:**

1. ~~Sourcing lands~~ — **done, 2026-08-17.**
2. The constants still single-sourced are named individually: **the D50
   colorants remain unpublished**, and `0.04045` from a standards *text*
   is still owed (CSS Color 4's own normative reference for sRGB is the
   IEC paywall, so it restates rather than corroborates). Both plausibly
   close with **one operator browser download** from `color.org` —
   `chardata/rgb/srgb.pdf`, `srgb.xalter`, and the registry entry.
3. The constructed sRGB is written, citing standard and clause per rule 2.
4. It is tested against a **parsed** sRGB profile with a stated tolerance
   and a named oracle — **and per §4.2, never by byte equality.**

★ **Still no worked sRGB triple exists in any of the four documents.**
That gap does not close, and it is why this remains a construction from
constants rather than a check against a published example.

★ **Step 4 has a trap worth naming now.** The Ghent corpus's `sRGB
IEC61966-2.1` files are **defectively authored**: they encode `wtpt` =
D65 while their colorants sum to D50 with **no `chad`**, which ICC.1
Annex A.3.1.1 makes a defect (`GHENT_COMPATIBILITY.md` §4.5b). They are
therefore a fine test that iccce *parses what real files contain*, and a
**bad** reference for what the constructed profile's white point should
be. Do not let the two uses blur.

---

## 5. What this does NOT decide

1. **The API shape.** Whether the entry point takes an
   `Option<&Profile>`, a distinct `Destination` enum, or a separate
   constructor is an engineering call, and §2 argues it should *not* be a
   bare `Option`. Not decided here.
2. **Whether sRGB is the right default for a *print* consumer.** It is
   right for `pdfce`, which rasterises to an sRGB pixmap. A caller
   converting for press wants its own output intent, and §2's rule is
   what protects that case.
3. ~~**Anything about `/N` validation** … **iccce currently exposes no
   public signature→component-count helper**~~ ★ **CLOSED, and this
   paragraph was stale.** Re-measured 2026-08-21: the helper exists as
   `iccce_profile::colour_space::components()`
   (`crates/iccce-profile/src/colour_space.rs:181`), returning a
   `ComponentCount`, and it does **not** require building a chain. The
   text above was written when it was true and was never revisited.
   **Left struck rather than deleted**, because the gap is cited from
   the request channel and a reader arriving from there needs to see
   that it closed rather than find no trace of it.
4. **Whether `pdfce` should route `/OutputIntents` CMYK through iccce**,
   which is the *other* owed design question and is a **conformance**
   argument, never an accuracy one.
5. ★★ **That this space is fit to COMPOSITE in. It is a destination, and
   the standard says it can be unsuitable as a blending space.**
   **ISO 32000-2:2020 §11.7.2 NOTE 4**: the CIE-based sRGB colour space
   *"is nonlinear and hence can be unsuitable for use as a group colour
   space"*, because (NOTE 3) compositing and blend functions compute
   **linear combinations** *"on the assumption that the component values
   themselves are linear"*. ★ **Edition-dependent**, and the difference
   decides whether a processor is deviating: the `should` is **body text
   in ISO 32000-2:2020** and sits **inside NOTE 3 in ISO 32000-1:2008** —
   normative in the newer edition, informative in the older.

   **Why it belongs in this document specifically.** §1's decision is
   that a caller with no destination gets *this* space; a PDF engine
   building a transparency-group buffer is exactly the caller that would
   reach for the destination it already holds. **The failure mode is a
   page that renders and looks plausible.** Disclosed at the definition
   site too (`crates/iccce-cmm/src/builtin.rs` module doc). ★ **It is a
   disclosure and not a guard** — nothing in iccce can see what a caller
   does with the model it returns, and nothing pretends to.

   Sourced 2026-08-21 from `PDF_Spec\_sources\`, printed p. 426,
   **verified from primary, two independent extraction engines**,
   errata-checked (no erratum against §11.7.2; `/Annots` scan plus
   `pdf-issues.pdfa.org`, two channels agreeing).

---

## 6. Provenance

| statement | source |
|---|---|
| the decision | operator, 2026-08-17, quoted verbatim |
| today's behaviour (`Chain::new` demands two profiles; every `sRGB` in `crates/` is a test) | `transform.rs:246`; `NEXT_SESSION.md` §0 |
| `pdfce`'s destination question, its three gates, and its item 3 and 4 | `open/request_iccbased_colour_spaces.md`, quoted |
| the corpus's single-source warning on sRGB | `ICC_Spec/iec/iec__s__srgb.md` frontmatter, read 2026-08-17 |
| the oracle-agreement defect class | `NEXT_SESSION.md` §5.2 |
| Ghent's sRGB files' authoring defect | `GHENT_COMPATIBILITY.md` §4.5b, from the Pass G run |
