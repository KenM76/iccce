# iccce — the tolerance budget

**Owner:** `icc-conformance`.
**Status: skeleton.** One provisional anchor is recorded. **No tolerance
has been set, because no comparison has been made.** Every numeric cell
below that is blank is blank on purpose.

---

## 0. Why this document exists at all

A colour engine's test suite is graded against a reference, at a
tolerance somebody chose, over a corpus somebody assembled. Each of those
three is a place a whole suite can be quietly meaningless — and the
tolerance is the easiest of the three to corrupt, because corrupting it
looks like fixing a test.

The rule (`CLAUDE.md` rule 5):

> **A tolerance nobody can justify is a tolerance that will be widened.**

"Within 1 ΔE2000 because that is the accepted threshold of perceptible
difference for adjacent patches" is a tolerance. "Within 0.5 because it
passed" is a number someone moved until the suite went green. The two
are indistinguishable in the source code and completely different in
what they mean.

### The procedure when a test fails

In this order, and the order is the point:

1. **Is the code wrong?** Assume yes until shown otherwise.
2. **Is the expectation wrong?** A transcription error in a reference
   value, a wrong illuminant, a v2/v4 encoding mix-up.
3. **Is the fixture wrong?** (See the `A2B0`/`A2B2` case in
   `tools/difftest/README.md` §8.4 — a "failure" that was a property of
   the profile.)
4. **Only then**: is the tolerance wrong? And if it is, the fix is a new
   row in §4 with a new justification and a new date — never an edit to
   the number in place.

**Widening a tolerance is an event that leaves a record.** §4 is
append-only.

---

## 1. Ground truth vs cross-check — every row must say which

This distinction governs the whole document.

| Kind | What it is | Strength |
|---|---|---|
| **Ground truth** | A published CIE or vendor value, transcribed with its source. | Strongest available. The standard, or a measurement of the world. |
| **Cross-check** | Agreement with lcms2 (or another independent implementation). | Weaker. Evidence that two implementations read the specification the same way — and two implementations can share a misreading. |
| **Self-consistency** | Round-trip error, compiled-vs-uncompiled error, interpolation error. | Weakest as *correctness* evidence — it detects drift, not error — but it is the only way to price an approximation, so it is where §5's numbers live. |

A row that does not state its kind is not finished.

**Never promote a cross-check to ground truth by transplanting it.**
Numbers produced by lcms2 must not be pasted into an `iccce-color` unit
test as an expected value. `CLAUDE.md` rule 3: a test whose expectation
came from an implementation detects change, not error.

---

## 2. The perceptual anchor

This is not itself a tolerance. It is the yardstick that most tolerances
below will be expressed as a fraction of, so it is recorded first and
separately.

| | |
|---|---|
| **Value** | **1.0 ΔE2000** |
| **Claim** | A colour difference of 1.0 ΔE00 is the conventional threshold of perceptibility for two large, uniform, directly-adjacent patches under controlled viewing. |
| **Status** | ⚠ **PROVISIONAL — citation not yet verified from primary text.** |
| **Recorded** | 2026-08-11 by `icc-conformance` |

### What is actually established, and what is convention

Stated carefully, because this number is going to be load-bearing and it
is more contested than its ubiquity suggests.

- **The formula is standardised.** CIEDE2000 is defined in **CIE
  142-2001**, subsequently **ISO/CIE 11664-6**. Those documents define
  ΔE00 and its reference conditions. **They do not declare a
  just-noticeable-difference value.** The "1.0 = JND" figure is *not* in
  the standard; it is industry convention layered on top of it.
- **The empirical literature does not report a single number.** Measured
  50:50 perceptibility thresholds for ΔE00 span roughly **0.8 to 2.3**
  depending on stimulus size, background, illuminant, edge contact and
  observer population. A frequently cited psychophysical result puts
  PT₀₀ near **1.0–1.2** with an acceptability threshold near **2.7**;
  other controlled studies land above 2.
- **For CIELAB ΔE\*ab**, the corresponding commonly cited JND is ≈2.3,
  attributed to **Mahy, Van Eycken & Oosterlinck (1994), "Evaluation of
  Uniform Color Spaces Developed after the Adoption of CIELAB and
  CIELUV", *Color Research & Application* 19(2):105–121, DOI
  10.1111/j.1520-6378.1994.tb00070.x**. The bibliographic record is
  confirmed; **the 2.3 figure itself has not been read out of the paper**
  and is recorded here as attributed, not verified.

**So why adopt 1.0 anyway?** Because it sits at the *conservative* end of
the measured range, and a conservative anchor is the right kind of wrong
for an engine tolerance. If the true threshold for a given stimulus is
2.0 and we hold ourselves to 1.0, we have spent margin we did not have to
spend. If we had picked 2.0 and the true threshold were 1.0, we would be
shipping visible error and calling it conformance.

That is the justification, and it is a *design* justification rather than
an empirical one — which is exactly the sort of thing that has to be
written down instead of implied by a magic number in a test.

### What would settle it

Dispatch **`icc-spec-librarian`** to obtain and quote:

1. **CIE 142-2001** / **ISO/CIE 11664-6** — the reference viewing
   conditions under which ΔE00 is defined. ΔE00's parametric factors
   (k_L, k_C, k_H) are only 1:1:1 under those conditions, and a tolerance
   quoted in ΔE00 without them is underspecified. The conditions are
   believed to specify a D65 simulator at ~1000 lx, a uniform achromatic
   L\*≈50 background, patch subtense >4°, and **direct edge contact with
   no separation** — **all of that is unverified recollection and must be
   read out of the standard before it is relied on.**
2. **Mahy et al. 1994** — the ΔE\*ab ≈ 2.3 figure, from the paper.
3. A primary psychophysical source for a ΔE00 perceptibility threshold,
   with its stimulus and viewing conditions stated.

Until then this section carries its ⚠ and any tolerance derived from it
inherits the ⚠.

---

## 3. The budget

Columns, and what each is for:

- **Comparison** — what is being compared to what. Must be specific
  enough to reproduce.
- **Kind** — ground truth / cross-check / self-consistency (§1).
- **Metric** — ΔE76, ΔE2000, or absolute (in stated units). "ΔE" alone is
  not a metric.
- **Tolerance** — the number.
- **Justification** — *why that number*. Not "it passed".
- **Measured** — the date the comparison was actually run, and by whom.
  Blank means it has never been run and the tolerance is a guess, which
  is a state a row is allowed to be in only while it is blank.

### 3.1 Pass 1 — `iccce-color` colorimetry

| Comparison | Kind | Metric | Tolerance | Justification | Measured |
|---|---|---|---|---|---|
| XYZ ↔ Lab, published CIE values | ground truth | absolute | — | — | — |
| Bradford adaptation, published matrix | ground truth | absolute | — | — | — |
| ΔE2000 implementation vs published test data | ground truth | absolute | — | — | — |
| ΔE76 / ΔE94 / ΔECMC | ground truth | absolute | — | — | — |
| Standard illuminant white points | ground truth | absolute | — | — | — |

> **Note on the ΔE2000 row.** The canonical verification set for a
> CIEDE2000 *implementation* is the 34-pair table in **Sharma, Wu &
> Dalal (2005), "The CIEDE2000 color-difference formula: Implementation
> notes, supplementary test data, and mathematical observations",
> *Color Research & Application* 30(1):21–30**, which exists precisely
> because the formula's hue-angle discontinuities are easy to get subtly
> wrong. That is ground truth and the tolerance will be an
> *arithmetic-agreement* tolerance (agreement to the published number of
> digits), not a perceptual one — §2's anchor is irrelevant to it.
> **Not yet set: the paper has not been obtained and the data not
> transcribed.**

### 3.2 Pass 2 — `iccce-profile` parsing

Parsing is exact or it is wrong; most rows here will be "byte-identical"
rather than a ΔE. Rows are listed so that the ones which *are* numeric
(s15Fixed16 round-tripping, curve evaluation) are not forgotten.

| Comparison | Kind | Metric | Tolerance | Justification | Measured |
|---|---|---|---|---|---|
| Header fields vs synthetic generator input | ground truth | exact | — | — | — |
| `s15Fixed16Number` decode | ground truth | absolute | — | — | — |
| `curveType` / `parametricCurveType` evaluation | cross-check | absolute | — | — | — |
| Malformation reports vs deliberately-broken fixtures | ground truth | exact | — | — | — |

### 3.3 Pass 3 — matrix/TRC transforms

| Comparison | Kind | Metric | Tolerance | Justification | Measured |
|---|---|---|---|---|---|
| sRGB → AdobeRGB → sRGB round trip | self-consistency | ΔE2000 | — | — | — |
| sRGB → AdobeRGB vs lcms2 | cross-check | ΔE2000 | — | — | — |
| sRGB → Lab vs published sRGB primaries | ground truth | ΔE2000 | — | — | — |

### 3.4 Pass 4 — LUT transforms and rendering intents

| Comparison | Kind | Metric | Tolerance | Justification | Measured |
|---|---|---|---|---|---|
| CMYK → RGB, perceptual, vs lcms2 | cross-check | ΔE2000 | — | — | — |
| CMYK → RGB, media-relative, vs lcms2 | cross-check | ΔE2000 | — | — | — |
| CMYK → RGB, saturation, vs lcms2 | cross-check | ΔE2000 | — | — | — |
| CMYK → RGB, ICC-absolute, vs lcms2 | cross-check | ΔE2000 | — | — | — |
| v2 `lut16Type` Lab encoding | ground truth | absolute | — | — | — |
| v4 `lutAToBType` Lab encoding | ground truth | absolute | — | — | — |

> The v2/v4 Lab encoding rows are marked **ground truth** deliberately.
> `ARCHITECTURE.md` §2 names this as the single richest source of CMM
> bugs, and it is the case where "lcms2 agrees" is least reassuring: an
> encoding difference of exactly the kind at issue would be shared by any
> implementation that read the clause the same way. These must be settled
> from the specification text, not from the oracle.

### 3.5 Pass 5 — black point compensation

| Comparison | Kind | Metric | Tolerance | Justification | Measured |
|---|---|---|---|---|---|
| BPC on vs off, direction of change | self-consistency | ΔE2000 | — | — | — |
| BPC vs lcms2 `-b` | cross-check | ΔE2000 | — | — | — |

### 3.6 Pass 6 — performance, and the price of speed

| Comparison | Kind | Metric | Tolerance | Justification | Measured |
|---|---|---|---|---|---|
| Compiled transform vs uncompiled reference path | self-consistency | ΔE2000 | — | — | — |
| Interpolation grid density vs exact evaluation | self-consistency | ΔE2000 | — | — | — |

---

## 4. Changes to tolerances — append only

Every change to a number in §3 gets a row here. **Never edit a tolerance
in place.** The history of a tolerance is the only defence against it
drifting one justification at a time.

| Date | Row | Old | New | Who | Why |
|---|---|---|---|---|---|
| — | — | — | — | — | — |

---

## 5. Named approximations

`ARCHITECTURE.md` invariant 3: *every approximation is named and
measured*. This is the register. Each entry states the departure from
exact colorimetry, and what it costs in ΔE — a cost of "unmeasured" is
permitted only while the entry is new.

| Approximation | Where | Cost (ΔE2000) | Measured |
|---|---|---|---|
| — none registered yet — | | | |

An unstated approximation is indistinguishable from a bug. An approximation
stated with an unmeasured cost is at least an honest one.

---

## 6. Coverage — say the scope or say nothing

**"Verified" without scope is the claim this document exists to
prevent.** Every conformance statement must carry: how many profiles, of
which classes, at which intents, on which platform.

Current coverage, stated honestly:

| Pass | Status |
|---|---|
| 0 | oracle pinned, built (**Windows/MSVC only**) and smoke-tested on **2 profiles, 1 direction each**; no comparison against iccce has been made because iccce does not exist yet |
| 1–8 | not started |

The Pass 0 smoke test is recorded in `tools/difftest/README.md` §8. It
used `sRGB Color Space Profile.icm` and `USWebCoatedSWOP.icc` from the
Windows colour directory — **category (c) under `LEGAL.md` §3, read
locally, never committed, and absent on the Linux runner.**

---

## 7. Related

- `tools/difftest/README.md` — the oracle, its pin, its licence, and the
  smoke test.
- `docs/LEGAL.md` §4 — lcms2 licence verification.
- `docs/LEGAL.md` §5 — reference values are facts; transcribe the source
  alongside the value.
- `CLAUDE.md` rules 3, 4, 5, 7.
