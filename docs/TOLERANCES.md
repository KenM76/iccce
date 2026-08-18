# iccce — the tolerance budget

**Owner:** `icc-conformance`.

**Status, 2026-08-11: Pass 1's tolerances are recorded (§3.1), Pass 3's were
measured later the same day (§3.3), and Pass 4's LUT differential later still
(§3.4). Pass 2 and Passes 5–6 are still blank, and blank on purpose.** Exactly
**one** row in this document is a
correctness claim against published data — the CIEDE2000 row. Everything in
§3.1 besides it is an *arithmetic identity*: it detects drift and structural
error and **cannot detect a consistently wrong constant**. §3.3's and §3.4's
rows are *cross-checks and self-consistency*, which is a different and weaker
thing again. The distinction is drawn in §1 and is the difference between
"this is right" and "this has not changed".

**★ And §3.4 adds a distinction §3.3 never had to make.** Pass 3's
disagreement with lcms2 was a *rounding* difference, so one tight number could
both bound it and mean something. Pass 4's dominant disagreement is a **named
approximation** — the CLUT interpolation scheme, **NA-006**, a choice inside a
specification silence — worth up to 1.6 ΔE2000 and not going away. A single
tolerance cannot both admit that and demonstrate agreement. §3.4 therefore
carries **wide structural gates that explicitly cannot claim agreement** and
**tight arithmetic gates that can**, and every row says which it is. Quoting a
§3.4 row without that distinction is the specific misuse this document exists
to prevent.

**★ Superseded 2026-08-11 (later): this document now DOES grade `iccce`
against `lcms2`.** The paragraph here previously read *"No tolerance
anywhere in this document grades `iccce` against `lcms2`. There is still no
such comparison to grade: `iccce` has no transform (Pass 3)."* Pass 3
shipped the matrix/TRC model and `iccce transform`, the comparison was run,
and **§3.3 carries the first five graded iccce-vs-anything rows in the
project.** Their scope is one profile pair, one intent, one direction, 133
grid points, one platform — §6 states it in full, and no shorter statement
is a fair summary of it.

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

> **★ A fourth kind was added 2026-08-11 (later still) by `icc-conformance` —
> `derived-expectation`.** An expectation computed by **arithmetic** from the
> specification's stated element order and encoding plus the bytes of a
> *synthetic* fixture, with **no implementation's output in it**. It sits
> between ground truth and cross-check: it is **not** a published value, so it
> must never be called ground truth; but it is defeated only when *the
> derivation* shares a misreading, where a cross-check is defeated when both
> implementations do. **§3.4.4 defines it in full, states what it cannot do, and
> is the only place it is used.** The table above is left as it stood; extending
> it is owed, and is deliberately not done here because §1 is a shared
> definitional section and this pass owned §3.x and §4.

**Never promote a cross-check to ground truth by transplanting it.**
Numbers produced by lcms2 must not be pasted into an `iccce-color` unit
test as an expected value. `CLAUDE.md` rule 3: a test whose expectation
came from an implementation detects change, not error.

### 1.1 ★★★ Candidate separation — the second thing a row must state, added 2026-08-12

**The kind of a claim says how strong the *evidence* is. It says nothing about
how much *power* the row has**, and on 2026-08-12 this document learned the
difference the expensive way (§3.5.8.6, `ARCHITECTURE.md` DL-033).

> A cross-check's power is bounded by **the distance between the answer it
> observed and the answer it would have observed under a plausible rival
> reading.** Agreement to 0,08 proves nothing if the wrong answers also sit
> within 0,08; the same 0,08 is strong if they sit 5 apart. **Nothing in this
> budget recorded that distance**, so the two situations were indistinguishable
> in every row we keep.

The incident, in one line: `bpc.rs`'s non-conformant 4.2.5.4 branch sat
**0,082 `L*`** from lcms2's answer and the conformant one sits **4,799** away,
so a **4,717 `L*`** defect produced an 0,082 signal — its own magnitude was
**57,8× the divergence it was blamed for**, and the cross-check built to catch
it was very nearly blind.

`tools/difftest` now emits a **candidate separation** on every record
(`lib.rs`'s `Separation`; the `separation` and `sep-power` columns; README §20).
Three states, deliberately not collapsed:

| state | meaning |
|---|---|
| **measured** | a **named** rival candidate exists — an intermediate the code already computes, a branch the other implementation might have dispatched to, the other document's reading of one word — with the value this row would have observed under it, and the distance |
| **no named alternative** | somebody looked and there is none. **A real statement**, and it carries its reason |
| **unstated** | nobody has considered this row yet. Prints as `UNSTATED`, not as a blank |

And two automatic verdicts, both machine-detectable, neither of which changes a
row's pass/fail status:

- **`BLIND`** — the candidates are **closer together than the row's own
  tolerance**, so it passes under either. Precisely the configuration that hid
  the 4.2.5.4 defect.
- **`ZERO-SEPARATION`** — the candidates are the *same number*. The row cannot
  move at any tolerance. A blind row can be rescued by tightening a tolerance; a
  zero-separation row can only be rescued by **a different fixture**
  (`ARCHITECTURE.md` DL-036, `tools/gen-profiles/README.md` §4.1 — FINDING
  GP-002).

**Why a flag and not a failure.** A small separation is sometimes legitimate — an
exact invariant, a null control — and auto-failing it would create pressure to
stop stating separations at all, which is the opposite of the point. The counts
and the offending ids are emitted on their own `separation` line; the `summary`
line is unchanged.

**Coverage of this mechanism, stated as §6 requires: 16 of 145 emitted rows
carry a stated separation, all of them Pass 5c's**, on the run of 2026-08-12
(`pass=142 fail=0 skip=3 error=0`; `unstated=129 no-named-alternative=4
incommensurate=2 ungraded=3 zero-separation=1 blind=0 discriminating=6`). The
other 129 are marked `UNSTATED` and that is an honest absence, not a claim that
they are well separated. **A few real separations with the rest honestly marked
is the design**; a scheme that invented one for every row would be a worse
document than this one.

#### 1.1.1 Coverage as of the second run — 2026-08-12, later

**41 of 160 emitted rows carry a stated separation** — Pass 5c's 30 (three arms
now, §3.5.9) and Pass 4c's 10, plus one more from the third arm's fixture row.
Run: `pass=157 fail=0 skip=3 error=0`;
`unstated=119 no-named-alternative=12 incommensurate=3 ungraded=8
zero-separation=2 blind=0 discriminating=16 sep-broken=0`.

The 119 `UNSTATED` remain the honest absence. Of the 10 Pass 4c rows, **four
carry a named rival and six do not** — and the six are
`no-named-alternative` **with their reason**, which is the state this mechanism
exists to distinguish from "nobody looked". Two examples of a reason worth
having: the media-relative floor rows say *lcms2 consults the media white point
only for the ICC-absolute adjustment, so the predicate whose three readings this
module prices is never evaluated at that intent*; the sensitivity-floor row says
*the only alternative one could name is a different floor, and that is a
tolerance question, not a value the row could have observed.* **Conflating a
rival tolerance with a rival candidate is how a separation quietly becomes a
second, undocumented gate.**

#### 1.1.2 ★★ `Separation::against` derives a distance that COLLAPSES when the defect is present

Found by running the mechanism against an injected defect rather than by review,
2026-08-12.

`Separation::against(alternative, alt_observed, observed, units)` derives the
distance as `|observed − alt_observed|`. That is right when the alternative is a
different *reading applied to the same observation*, so that the two values
genuinely coexist. It is **wrong** when the alternative is *"the code under test
returns the other candidate"* — because then, on the run where the code actually
does return the other candidate, `observed` **becomes** `alt_observed` and the
derived distance is exactly zero.

Measured instance: with the pre-`fd34a44` behaviour injected,
`pass5c/floored/CLAUSE/4.2.5.4-returns-InitialLab-not-outRamp-first` failed at
`2,500 019×10¹` against a `7,629×10⁻⁴` bound — **and printed `ZERO-SEPARATION`
beside it.** The mechanism disclaimed its own power on the one run where it had
just demonstrated it.

**The test to apply before choosing a constructor: is the distance a property of
the RUN or of the FIXTURE?** A distance between two candidate *answers* is a
property of the fixture — 25 `L*` whichever answer the library returns today —
and must be supplied through `Separation::against_distance`, not derived. Three
rows were corrected under this rule (the clause row above and two `0/1`
indicator rows, whose candidate observations are `0` and `1` and are therefore
always one apart); the hazard is now recorded on `Separation::against`'s own doc
comment, which is where the next person will meet it.

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

**Filled 2026-08-11 by `icc-conformance`**, from the comparisons actually
run. Every number below **mirrors a row of `docs/NUMERIC_CLAIMS.md`** —
that document is the retrospective record of what was measured, this one
is the prospective budget, and where they state a number it must be the
same number. No tolerance here was invented for this table.

> **★ Read this before quoting any tolerance below.** A tolerance is
> **the bound the test asserts**, not the residual that was observed.
> `assert!((got − expected).abs() < 1e-4)` passing proves the error was
> below 1×10⁻⁴ *on that run*; it does not establish that the error was
> 3×10⁻⁶. **The observed maxima are not on record anywhere**
> (`NUMERIC_CLAIMS.md` §1.1, and §7 item 2 there records it as owed). A
> residual that has silently grown from 10⁻¹² to 9×10⁻⁵ still passes a
> 10⁻⁴ gate and nothing would show it.

#### 3.1.1 The one correctness claim

| Comparison | Kind | Metric | Tolerance | Justification | Measured |
|---|---|---|---|---|---|
| `delta_e_2000` vs the **34 published pairs** of Sharma, Wu & Dalal (2005), all 34, `kL=kC=kH=1` | **ground truth** | absolute, ΔE00 units | **1×10⁻⁴** | **The published data's own precision.** The paper prints ΔE00 to four decimal places, so agreement cannot be asserted more tightly than the reference is printed. This is an **arithmetic-agreement** tolerance: §2's 1.0 ΔE2000 perceptual anchor is *irrelevant* to it and must not be cited in its support. | 2026-08-11, `icc-engineer` — `NUMERIC_CLAIMS.md` **NC-001** |

Coverage is **part of the claim**: all 34 pairs, not a sample. The set is
adversarial by design (pairs 1–6 catch an omitted `R_T`, 9–16 sit on the
hue-angle discontinuity, 33–34 are very dark), so cherry-picking would
defeat it. `kL=kC=kH=1` only — nothing is claimed for other parametric
factors.

#### 3.1.2 Arithmetic identities — what they are worth, and what they are not

These assert properties that must hold **by construction**: round trips,
inverses, symmetry, degenerate inputs. Their tolerances are `f64` noise
floors, chosen as *the tightest bound the arithmetic can be expected to
meet*, and they are not perceptual budgets — quoting §2's anchor near any
of them would be a category error.

**What they cannot do**, stated as prominently as what they can: a round
trip through a *wrong* white point round-trips perfectly, and an
adaptation matrix built from a mis-transcribed cone matrix still maps its
own source white to its own destination white exactly. **A consistently
wrong constant survives every row in this table.**

| Comparison | Kind | Metric | Tolerance | Justification | Measured |
|---|---|---|---|---|---|
| ΔE2000 symmetry `ΔE(A,B) = ΔE(B,A)`, over all 34 Sharma pairs | arithmetic identity | absolute | **1×10⁻¹²** | The two evaluations differ only in operand order, so the residual is `f64` rounding on ~10 flops — bounded far below 10⁻¹². Anything larger is asymmetric mean-hue handling, not noise. | 2026-08-11 — **NC-002** |
| ΔE2000 of a colour with itself | arithmetic identity | absolute | **exact** (`assert_eq!` 0.0) | Every difference term is identically zero before any division; there is no rounding to allow for. An epsilon here would be an unjustified number. | 2026-08-11 — **NC-003** |
| ΔE76 on a (3,4,12) difference = 13.0 | arithmetic identity | absolute | **exact** | Pythagorean triple: the sum of squares is 169 exactly in binary floating point and its root is exact. | 2026-08-11 — **NC-004** |
| Bradford adaptation with `src == dst` is the identity matrix (9 elements) | arithmetic identity | absolute, per element | **1×10⁻¹⁴** | `M⁻¹·diag(1,1,1)·M` over a well-conditioned 3×3; a couple of ulp per element accumulated over ~30 flops. | 2026-08-11 — **NC-005** |
| Adaptation maps the source white exactly onto the destination white | arithmetic identity | absolute, per channel | **1×10⁻¹²** | The identity is exact in exact arithmetic; the bound is the rounding of one 3×3 inverse plus two matrix products. **One direction only: D65 → D50.** | 2026-08-11 — **NC-006** |
| D65 → D50 → D65 round trip | arithmetic identity | absolute, per channel | **1×10⁻¹²** | As above, twice. **One sample vector, one illuminant pair.** | 2026-08-11 — **NC-007** |
| `BRADFORD` row sums = 1.0001 / 1.0000 / 1.0000 | **transcription-guard** | absolute | **1×10⁻¹²** | Sums of three 4-decimal literals: exact to well within 10⁻¹² unless a digit is wrong. **This checks a typo, not a value** — it says nothing about whether the matrix is the right matrix. The 1.0001 in row 1 is real, not a typo. | 2026-08-11 — **NC-008** |
| `f_inv(f(t)) = t` across the Lab transfer function's breakpoint, 7 probes | arithmetic identity | absolute | **1×10⁻¹⁵** | The rational-form breakpoint makes `f` and `f⁻¹` *exact* mutual inverses (see §5, NA-001); the residual is a cube-root round trip, ~1 ulp. This tolerance is the property the A11 choice was made for. | 2026-08-11 — **NC-010** |
| XYZ → Lab → XYZ round trip, 2 samples (one per branch of `f`) | arithmetic identity | absolute, per channel | **1×10⁻¹²** | Cube root and cube, plus three divisions by the white point. | 2026-08-11 — **NC-011** |
| White maps to `L*=100, a*=b*=0` | arithmetic identity | absolute | **exact** | `X/Xn = Y/Yn = Z/Zn = 1` exactly by construction, `f(1)=1`, so `116·1−16 = 100` and the differences cancel exactly. | 2026-08-11 — **NC-012** |
| `Y = 0` maps to `L* = 0` | arithmetic identity | absolute | **exact** | Holds exactly **only because the linear segment exists**: a cube-root-only `f` gives `f(0)=0` and `L* = −16`. The identity most worth keeping, because it is what NA-001 is load-bearing for. | 2026-08-11 — **NC-013** |
| Lab ↔ LCh round trip, hue wrapped into `[0,360)`, 1 third-quadrant sample | arithmetic identity | absolute on `a*`, `b*` | **1×10⁻¹²** | `atan2`/`hypot` then `cos`/`sin`: a handful of ulp. The sample is chosen where `atan2` returns negative, so the wrap is exercised. | 2026-08-11 — **NC-014** |
| XYZ → xyY → XYZ round trip, 1 sample (D50) | arithmetic identity | absolute, per channel | **1×10⁻¹⁴** | Two divisions and two multiplications; no transcendental. | 2026-08-11 — **NC-015** |
| `Mat3::inverse`: `M · M⁻¹ = I`, 1 non-colour matrix | arithmetic identity | absolute, per element | **1×10⁻¹⁴** | Adjugate/determinant on a well-conditioned integer-ish matrix. **The Bradford inverse is exercised only indirectly, through NC-005.** | 2026-08-11 — **NC-016** |
| `D50.to_xyy()` vs the chromaticity derived from iccce's own sourced D50 triple | arithmetic identity / self-consistency | absolute | **5×10⁻⁷** | Six significant figures of a two-division derivation — the precision at which the derived value is stated. **Not a published expectation**; the corpus marks all such chromaticities DERIVED. This test **failed first and the corpus was wrong**, see NC-017. | 2026-08-11 — **NC-017** |
| D65 XYZ derived through `XyY::to_xyz()` vs the corpus's derived triple | arithmetic identity | absolute | **5×10⁻⁶** | The corpus states the derived triple to 6 significant figures; agreement cannot be claimed tighter than the value is written. **Rests on a single-source D65 chromaticity** (lcms2 only) — the weakest constant in the crate. | 2026-08-11 — **NC-018** |

#### 3.1.3 Ground truth that does **not** exist yet — deliberately blank

These are the rows the original skeleton listed as ground truth. They stay
blank because **no published expectation has been obtained**, and a
plausible number here would be worth less than the blank.

| Comparison | Kind | Metric | Tolerance | Justification | Measured |
|---|---|---|---|---|---|
| XYZ ↔ Lab against a **published worked example** | ground truth | absolute | — | no published worked example obtained; §3.1.2's round trips are identities, not this | — |
| A **complete chromatic adaptation** against a published worked example | ground truth | absolute | — | **the largest evidential hole in Pass 1** (`NUMERIC_CLAIMS.md` §7 item 4). Everything adaptation-related is a sourced matrix plus identities; a mis-transcribed digit that preserved the row sum would survive every test in the crate. | — |
| ΔE94 / ΔE CMC | ground truth | absolute | — | **not implemented**, deliberately: no citable formula transcription and no published worked examples, so an implementation today could only be lcms2-cross-checked | — |
| Standard illuminant white points against a **published** value | ground truth | absolute | — | D50's triple is sourced; its chromaticity is *derived* (NC-017), and D65 is single-source (NC-018). Neither is a published-value comparison. | — |

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

**Filled 2026-08-11 by `icc-conformance`** from comparisons actually run.
Apparatus and full derivations: **`tools/difftest/README.md` §13**, and the
tolerance constants in `tools/difftest/src/pass3.rs`, each of which carries
its derivation as a doc comment. Machine-readable records: §13.9 there.

> **★ Read this before quoting any row below.** Everything in this table is
> **one profile pair** — the Windows system `sRGB IEC61966-2.1` and
> `Adobe RGB (1998)`, both **v2.1**, both **category (c)** (`LEGAL.md` §3:
> read locally, never committed) — at **one intent** (media-relative
> colorimetric), in **one direction**, over **133 deterministic grid
> points**, on **one platform**. Every check **skips** on a machine without
> the Windows colour directory or without `target/release/iccce`, and the
> runner then exits **3 (nothing ran)**, not 0.
>
> **The two ROADMAP done-when numbers are rows 3 and 5.**

#### 3.3.1 The graded rows

| Comparison | Kind | Metric | Tolerance | Justification | Measured |
|---|---|---|---|---|---|
| **1.** sRGB → AdobeRGB, iccce vs lcms2, **device space** (lcms2 clamped into [0,1]) — `pass3/srgb-to-adobergb/device-vs-lcms2` | **cross-check** | abs-max per component, normalised device units 0..1 | **5×10⁻⁴** | Derived from **lcms2's own arithmetic, not iccce's**. `cmsEvalToneCurveFloat` rounds a segment-free (tabulated) tone curve's input *and* output to 1/65535; the source profile's TRCs are 1024-entry `curv` tables, so each rounding is ≤½ lsb = 7.63×10⁻⁶, the input term amplified by the sRGB EOTF's peak slope 2.275 → **2.5×10⁻⁵ in source-linear**. Amplified by the destination inverse gamma `(1/γ)L^(1/γ−1)`, **unbounded as L→0**, so no finite uniform bound exists over the whole cube; evaluated at this grid's darkest non-zero step (L = 4.03×10⁻³ → ×11.6) the envelope is 2.9×10⁻⁴, rounded up to 5×10⁻⁴. **GRID-DEPENDENT BY CONSTRUCTION** — a grid extended nearer black re-derives it, never re-tunes it. Arithmetic-agreement, **not** perceptual: §2's anchor is irrelevant here. | 2026-08-11, `icc-conformance` — **observed 6.7059×10⁻⁵** (0.0171 in 0..255) |
| **2.** …the same, **mean** — `pass3/srgb-to-adobergb/device-mean` | cross-check | abs-mean per component, 0..1 | **∞ — REPORTED, NOT GRADED** | A mean over a grid hides exactly the outlier a colour engine gets wrong. Recorded so the distribution sits on file next to the max; **must never be quoted as if it were the max.** | 2026-08-11 — observed 6.1672×10⁻⁶ |
| **3. ★ sRGB → AdobeRGB, iccce vs lcms2, ΔE2000** — `pass3/srgb-to-adobergb/de2000-vs-lcms2` | **cross-check** | ΔE2000 **max**, `kL=kC=kH=1`, D50 CIELAB | **2×10⁻²** | Carrying the device value back through the destination model undoes row 1's unbounded amplification, so a finite ceiling exists: the same 2.5×10⁻⁵ becomes ≤2.5×10⁻⁵ in PCS XYZ (‖M_src‖∞ = 1.0, the Y row), and Lab's steepest sensitivities (`f'(t)=7.787` on the linear segment → `dL*/dY ≤ 903.3`, `da*/dX ≤ 4038`) give a worst-case **ceiling of ≈0.28 ΔE00**. **2×10⁻² is set deliberately TIGHTER than that ceiling**, because 0.28 is a pessimistic union bound and a residual that grew from 3×10⁻³ to 0.27 would still pass it with nothing to show (§3.1's boxed warning). 50× below §2's ⚠ provisional 1.0 anchor, whose ⚠ it inherits. | 2026-08-11 — **observed 3.4762×10⁻³** |
| **4.** …the same, **mean** — `pass3/srgb-to-adobergb/de2000-mean` | cross-check | ΔE2000 mean | **∞ — REPORTED, NOT GRADED** | See row 2. | 2026-08-11 — observed 5.1145×10⁻⁴ |
| **5. ★ sRGB → AdobeRGB → sRGB round trip, iccce alone** — `pass3/srgb-to-adobergb-to-srgb/roundtrip-de2000` | **self-consistency** | ΔE2000 **max**, `kL=kC=kH=1`, D50 CIELAB | **2.5×10⁻²** | Dominated by the **range clamp discarding the two files' encoded white-point mismatch**, which is a fact about the FILES: sRGB's colorant sum is (0.964 279 17, 0.999 969 48, 0.825 088 50) and Adobe RGB's is (0.964 202 88, 1.0, 0.824 905 40) — 5/2/12 units of `s15Fixed16`'s 1/65536 lsb — putting source white at (1.000 106, 0.999 873, 1.000 254) in destination linear space, two channels outside [0,1]. **25 of the 133 grid points are clipped somewhere.** Closed-form prediction from the two matrices and the clamp alone: **1.8782×10⁻²**, against **1.8788×10⁻²** observed — 0.03 % agreement. Plus ≈1×10⁻³ for 1024-entry table interpolation (`h²·max f''/8`, `h=1/1023`, ×903.3). **2.5×10⁻² = that sum with ~25 % headroom.** **CORPUS-SPECIFIC**: another pair re-derives it. **SUPERSEDES a 1×10⁻² whose justification wrongly assumed nothing was clipped — see §4.** | 2026-08-11 — **observed 1.8788×10⁻²** (mean 8.674×10⁻⁴; max device deviation 5.670×10⁻⁴) |
| **6.** The clamp cost matches its closed-form prediction, at device white — `pass3/roundtrip/white-clamp-cost-matches-prediction` | **self-consistency** | \|predicted − observed\| ΔE2000 | **1×10⁻³** | **Exists because row 5 is an UPPER bound on a deliberate cost**: remove iccce's range clamping and the round trip *improves*, so row 5 would go green while a normative requirement had been deleted. This row pins the observed cost *to* the prediction. 1×10⁻³ = 10× the ≈1×10⁻⁴ ΔE00 floor from `iccce transform`'s 6-decimal device print on each leg (±5×10⁻⁷ × `dL*/d device ≈ 85` at white ÷ `S_L ≈ 1.75`). **Sensitivity control run**: with no clamping the metric would read 1.878×10⁻², failing by 19×. **SCOPE: it does NOT detect the F.8–F.16 clamp removed on its own** — iccce clamps at three sites (F.8–F.16, 10.18's domain, F.1(b)) so the *ordering* is unobservable at the shipped surface. Owed, not covered. | 2026-08-11 — **observed 5.7392×10⁻⁶** |
| **7.** **Instrument check** — AdobeRGB device → D50 Lab, `iccce-cmm`'s model (in-process) vs `transicc -o*Lab4` — `pass3/instrument/adobergb-device-to-lab-ruler` | **cross-check** | ΔE2000 max | **5×10⁻²** | **Rows 3–5 measure with a ruler built partly out of the code under test.** If iccce's destination forward model were wrong their ΔE would be systematically mis-scaled and the error would hide *inside the metric*. This row holds the ruler against a second ruler. Bound is dominated by `transicc`'s 4-decimal Lab print (ΔE00 floor ≈1×10⁻⁴) plus the fact that lcms2's `cmsD50X/Y/Z` and iccce's `D50` agree to 4 decimals *by construction* but not beyond (~0.01 in `L*`). 5×10⁻² is ~5× that — loose enough not to fail on understood differences, tight enough to catch a swapped colorant, a missing D50 adaptation, or the v2/v4 Lab encoding error (≈0.39 `L*`). **This grades the instrument, not the shipped binary**, and is the one place in Pass 3 where iccce is called in-process rather than as a subprocess. | 2026-08-11 — **observed 8.7945×10⁻⁵** |

#### 3.3.2 Why the ΔE metric may be computed by `iccce-color` at all

Rows 3–7 use `iccce_color::delta_e_2000` to grade `iccce`. That is a
coupling, and `tools/difftest`'s own module docs previously forbade it. It
was taken as a **deliberate, documented decision** on 2026-08-11 and rests on
four things, all of which must remain true:

1. **The arrow points harness → code under test.** The invariant that matters
   (`tools/difftest/README.md` §1) is *no crate under `crates/` may reach
   lcms2*, and it is untouched.
2. **The ruler is ground truth, not self-reference.** `delta_e_2000` is
   graded against **all 34 published pairs** of Sharma, Wu & Dalal (2005) at
   1×10⁻⁴ — §3.1.1's single correctness row (NC-001).
3. **The claim did not change.** Rows 3, 4 and 7 are **cross-check**; rows 5
   and 6 are **self-consistency**. A validated ruler does not promote either
   to ground truth (§1's rule against transplanting).
4. **The answers still come from subprocesses.** iccce's colours come from
   the shipped `iccce transform` binary, lcms2's from `transicc`. The linked
   crates are the instrument, never the subject — except in row 7, which says
   so on its own record.

#### 3.3.3 Still blank, and correctly so

| Comparison | Kind | Metric | Tolerance | Justification | Measured |
|---|---|---|---|---|---|
| sRGB → Lab against **published** sRGB primaries | **ground truth** | ΔE2000 | — | **The largest evidential hole in Pass 3.** Everything in §3.3.1 is implementation-relative: the cross-checks say iccce and lcms2 read Annex F.3 the same way, the self-consistency rows price approximations. **Nothing yet compares a matrix/TRC transform to a published value.** IEC 61966-2-1's primaries and the D50-adapted matrix derived from them would supply one; the corpus has not been asked. Dispatch owed to `icc-spec-librarian`. | — |
| AdobeRGB → sRGB (the **reverse** direction) vs lcms2 | cross-check | ΔE2000 | — | Not run in the suite. It is the direction that exercises a **genuine** gamut clip — sRGB ⊂ AdobeRGB makes real clipping impossible in the forward direction. Spot-checked by hand 2026-08-11 (`tools/difftest/README.md` §13.4) and not graded. | — |
| A **v4** matrix/TRC pair vs lcms2 | cross-check | ΔE2000 | — | Both Pass 3 profiles are v2.1. The version-gated lcms2 behaviours §6.1 records are **avoided, not exercised**. | — |
| A **synthetic** matrix/TRC pair (category (a)) vs lcms2 | cross-check | ΔE2000 | — | Needs `tools/gen-profiles`, which does not exist (Pass 2 remainder). Until it does, **every Pass 3 row skips on any machine without the Windows colour directory** — including CI. | — |
| Clamp-**before**-TRC⁻¹ distinguished from clamp-**after** | ground truth | exact | — | Annex F.8–F.16 makes the order normative and `matrix_trc.rs` implements it, but iccce clamps at three independent sites so the ordering is **unobservable at the shipped surface**. Distinguishing them needs a TRC whose inverse is defined outside [0,1], which iccce never permits. **Owed, not covered.** | — |

### 3.4 Pass 4 — LUT transforms and rendering intents

**Filled 2026-08-11 (later) by `icc-conformance`** from comparisons actually
run. Apparatus and full derivations: **`tools/difftest/README.md` §14**, and
the tolerance constants in `tools/difftest/src/pass4.rs`, each carrying its
derivation as a doc comment. Per-point record and the three experiments:
`cargo run --bin pass4_report`.

> **★ Read this before quoting any row below.** Everything here is **one
> profile pair** — `USWebCoatedSWOP.icc` (v2.1, `prtr`, `CMYK`→`Lab `, `mft2`
> A2B, 9⁴ CLUT) → the Windows system `sRGB IEC61966-2.1` (v2.1, `mntr`, **no
> `B2A*` tags**), both **category (c)** — over **341 deterministic CMYK
> points**, in the **A2B direction only**, on **one platform**. The **B2A
> direction is not exercised at all**, so "all four intents" here means all
> four *A2B* intents. §6 states the scope in full and no shorter statement is
> a fair summary of it.
>
> **★★ And read §3.4.1 before quoting rows 1–3.** Their tolerance is the
> *interpolation-method envelope*, which is at and above the perceptibility
> anchor. **They cannot demonstrate agreement and must never be quoted as if
> they did.** The agreement claim on this pass is rows 4 and 5.

#### 3.4.1 The graded rows

| Comparison | Kind | Metric | Tolerance | Justification | Measured |
|---|---|---|---|---|---|
| **0.** The apparatus: the harness's own `mft2` reimplementation vs `iccce-cmm`'s `Lut16Model`, every point, every intent — `pass4/apparatus/harness-nlinear-matches-iccce-cmm` | **self-consistency** | abs-max, `L*`/`a*`/`b*` units | **1×10⁻⁹** | **The precondition for believing anything else in Pass 4.** The interpolation experiments need the same pipeline evaluated two ways, differing in one component; that substitution cannot be made inside `crates/`, so the harness rebuilds the pipeline and this row holds its n-linear arm against the crate's evaluator. 1×10⁻⁹ is ~7 orders above `f64` noise on this arithmetic and ~6 below anything colorimetric: it can neither pass a real divergence nor fail on rounding. | 2026-08-11 — **observed 0.0 exactly** (bit-identical) |
| **1.** SWOP → sRGB, iccce vs lcms2, **device space** (lcms2 clamped into [0,1]) — `pass4/swop-to-srgb/<intent>/device-vs-lcms2` | **cross-check** | abs-max per component, normalised device units 0..1 | **2×10⁻²** | The **interpolation-method envelope propagated through the actual destination model point by point** — a quantity computed from the CLUT and the two algorithms alone, with **no lcms2 output in it**: 1.0751×10⁻² (perceptual/saturation), 2.9012×10⁻³ (media-relative). 2×10⁻² is the larger with ~86 % headroom, sized to admit lcms2's 16-bit quantisation *on top of* the geometry. A closed-form union bound was computed first and discarded as useless (1.6 device units — wider than the range). Bounded at all only because the sRGB inverse TRC's slope is ≤12.92 near black; a pure-gamma destination has none (cf. §3.3 row 1). **GRID-DEPENDENT BY CONSTRUCTION**; arithmetic-agreement, **not** perceptual. | 2026-08-11 — **observed 1.0816×10⁻²** (perceptual/saturation), 3.0045×10⁻³ (media-relative) |
| **2. ★ SWOP → sRGB, iccce vs lcms2, ΔE2000** — `pass4/swop-to-srgb/<intent>/de2000-vs-lcms2` | **cross-check** | ΔE2000 **max**, `kL=kC=kH=1`, D50 CIELAB | **2.0** | The same envelope, propagated end-to-end and expressed in ΔE2000: **1.6639** (perceptual/saturation), **0.254 23** (media-relative), against observed **1.6590** and **0.252 94** — agreeing to **0.3 %** and **0.5 %**, so the disagreement is *accounted for* by a named non-error mechanism (**NA-006**) rather than merely being small. 2.0 is the larger envelope with ~20 % headroom. **ABOVE §2's ⚠ provisional 1.0 ΔE2000 anchor: this row does NOT demonstrate perceptual agreement.** It detects structural error — a wrong CLUT index order, a wrong Lab decode, a transposed ink, a missing input table, all of which are tens of ΔE. | 2026-08-11 — **observed 1.6590** (perceptual/saturation), **0.252 94** (media-relative) |
| **3.** SWOP → sRGB, **PCS side only** (source A2Bx vs `transicc -o*Lab4`) — `pass4/swop-to-srgb/<intent>/pcs-lab-vs-lcms2` | **cross-check** | ΔE2000 max | **2.0** | The same envelope, measured where it lives — the source CLUT, with the destination model out of the picture: **1.5741** (`A2B0`), **0.254 23** (`A2B1`). **The two tags in one file differ by 6×**, so a tolerance derived from the colorimetric table alone would have been wrong by that factor for exactly the intents Pass 3 never ran. Same disclaimer as row 2: wide by construction, structural only. | 2026-08-11 — **observed 1.5715** (`A2B0`), **0.254 65** (`A2B1`) |
| **4. ★★ …the same, with lcms2's OWN CLUT geometry emulated** — `pass4/swop-to-srgb/<intent>/pcs-lab-emulated-geometry` | **cross-check** | ΔE2000 max | **2×10⁻²** | **This is the row that claims agreement.** With lcms2's `Eval4Inputs` geometry (linear in C × tetrahedral in M,Y,K — read at pin `21c582a`) substituted for n-linear, what remains is the oracle's own quantisation: tabulated input curves rounded to 1/65535 in *and* out, the CLUT stage input rounded to `u16` (`EvaluateCLUTfloatIn16`), `Eval4Inputs` evaluated in **s15.16 fixed point**, and `transicc`'s 4-decimal Lab print. One 16-bit lsb of CLUT output is **1.53×10⁻³ in `L*`** and **3.9×10⁻³ in `a*`/`b*`** under the legacy decode this tag type mandates — a ~10⁻² ΔE00 budget; 2×10⁻² is that. | 2026-08-11 — **observed 4.8154×10⁻³** (`A2B0`), **4.5931×10⁻³** (`A2B1`); a **326×** and **55×** collapse from row 3 |
| **5. ★★ …the same, at the 16 CLUT-node corners only** — `pass4/swop-to-srgb/<intent>/pcs-lab-corners-interpolation-free` | **cross-check** | ΔE2000 max | **1×10⁻³** | **The sensitivity control that makes rows 2–3 defensible.** At a corner both implementations evaluate the CLUT at an exact node (each `mft2` input table starts `0x0000`, ends `0xFFFF`), so the method difference is **identically zero** — measured as 0.0 — *and* lcms2's quantisation terms vanish rather than accumulate (exact `u16` in, stored `u16` out, identity output tables). What is left is `transicc`'s 4-decimal Lab print: a ΔE00 floor of ≈1×10⁻⁴. **1×10⁻³ is 10× that floor.** Without a node-only control, a 2.0 gate could hide a real 1.9 ΔE error. | 2026-08-11 — **observed 6.6558×10⁻⁵** (`A2B0`), **5.9131×10⁻⁵** (`A2B1`) — the print floor, and 70× below the between-node figure |
| **6.** Perceptual and saturation are the same transform — `pass4/swop/perceptual-equals-saturation` | cross-check | abs-max, normalised device 0..1 | **0.0 — exact** | `A2B0` and `A2B2` are **one shared block of tag data** in this file (same offset, same size). Perceptual and saturation are the same bytes through the same code, so any difference is an 8.10.2 tag-selection defect and **no arithmetic could make it small**. A small epsilon would admit exactly the bug this row exists to catch. Graded on the larger of the two sides. | 2026-08-11 — **observed 0.0** on iccce's side and 0.0 on lcms2's |
| **7. ★ ICC-absolute: the white-point-policy model** — `pass4/swop-to-srgb/icc-absolute/white-point-policy-emulated` | **cross-check** | ΔE2000 max | **5×10⁻²** | **The gate at the absolute intent** (rows 1–2 are ungraded there — §3.4.2). It grades a *model*: re-predict lcms2's absolute output with exactly two of iccce's choices replaced by lcms2's — the CLUT geometry, and the **destination media white** (D50, which `_cmsReadMediaWhitePoint` substitutes for the `wtpt` tag of a **v2 display-class** profile). **Weakest-justified number in the pass, and labelled as such**: 5×10⁻² is ~2.3× the observed maximum and is a *bracket*, not a derivation — no closed form was computed for how the destination leg amplifies a PCS residual in deep shadow. What makes it usable is the ratio it sits between: **500× below the divergence it must detect, 2× above the quantisation floor it must not trip on.** | 2026-08-11 — **observed 2.1677×10⁻²** (mean 3.4034×10⁻³), a **517×** collapse from the unmodelled 11.217 |

#### 3.4.2 The two rows that are deliberately NOT graded, and why

| Comparison | Kind | Metric | Tolerance | Justification | Measured |
|---|---|---|---|---|---|
| SWOP → sRGB at **ICC-absolute**, device and ΔE2000 — `pass4/swop-to-srgb/icc-absolute/{device,de2000}-vs-lcms2` | cross-check | device abs-max / ΔE2000 max | **∞ — REPORTED, NOT GRADED** | **iccce and lcms2 read different destination media whites here**, and the difference is a *policy* whose specification question is unsourced. iccce uses `wtpt` **as stored** (**NA-007**) — which in this v2 `mntr` file is **D65** (0.950 455, 1.0, 1.089 050) — while lcms2 substitutes **D50** for any v2 display-class profile (`cmsio1.c`). The ratio is `D65/D50` = (0.9858, 1.0, 1.3202): a **32 % error in `Z`** on every colour. **Which is correct is corpus A4b, UNVERIFIED** (ICC.1:2001-04 not obtained); lcms2's substitution is justified in its source by a comment, not a clause. Grading this would mean either **a ~15 ΔE00 tolerance chosen because it passed** — which would also silently absorb any future arithmetic error in the absolute path — or **a permanent red line that says nothing new**. Both were considered and rejected in writing (`tools/difftest/README.md` §14.6). **The gate at this intent is row 7**, which is tight and would catch a regression. **When A4b is settled, one implementation acquires a defect and this becomes a graded row.** | 2026-08-11 — **observed 11.217 ΔE2000** (mean 4.6705); device 0.157 96 (mean 4.8469×10⁻²) |
| Means: `…/<intent>/device-mean`, `…/<intent>/de2000-mean` | cross-check | device abs-mean / ΔE2000 mean | **∞ — REPORTED, NOT GRADED** | A mean over a grid hides exactly the outlier a colour engine gets wrong. Recorded so the distribution is on file next to the max; **never to be quoted as if it were the max.** Note additionally that **this grid's mean is not the mean over printable colour** — it runs to 400 % total ink, which SWOP separations do not. | 2026-08-11 — ΔE2000 mean 4.3126×10⁻² (perceptual/saturation), 4.0107×10⁻² (media-relative); device mean 4.6257×10⁻⁴ / 4.1870×10⁻⁴ |

#### 3.4.3 Still blank, and correctly so

| Comparison | Kind | Metric | Tolerance | Justification | Measured |
|---|---|---|---|---|---|
| v2 `lut16Type` Lab encoding | **ground truth** | absolute | — | **Still owed, and still deliberately ground truth.** `ARCHITECTURE.md` §2 names this as the single richest source of CMM bugs, and it is the case where "lcms2 agrees" is least reassuring: an encoding difference of exactly this kind would be *shared* by any implementation that read the clause the same way. §3.4.1 row 5 shows the two agree at the corners to 6.7×10⁻⁵ ΔE00 — which is **evidence of agreement, not of correctness**, and does not discharge this row. It must be settled from the specification text. | — |
| v4 `lutAToBType` Lab encoding | **ground truth** | absolute | — | Same reasoning. **No v4 profile and no `mAB ` tag is exercised anywhere in Pass 4.** | — |
| ~~CMYK → RGB through the **B2A** direction, any intent~~ | cross-check | ΔE2000 | — | ~~**The half of Pass 4's done-when that is NOT measured.**~~ **★ MEASURED 2026-08-11 (later still) — see §3.4.4 §A.** The direction was measured as what it actually is: PCS→device, i.e. **RGB → CMYK** with SWOP as the *destination*, through its `B2A0`/`B2A1` `mft1` tags, at the perceptual and media-relative intents. Saturation (`B2A2`) and ICC-absolute are still not run. | **2026-08-11 (later still)** — §3.4.4 |
| A **published** value for any LUT transform | **ground truth** | ΔE2000 | — | **Still blank, and it is important that it stays that way.** §3.4.4's four `derived-expectation` rows are the closest thing that exists — an expectation computed by arithmetic from clause text and a synthetic fixture's own bytes — and they are **not** this row. Nobody published the number; the derivation is this project's reading of 10.12/10.13, and if `ICC_Spec`'s transcription is wrong the fixture and the expectation are wrong together. A published worked example would be independent of that; nothing in the corpus supplies one. | — |
| ~~A **synthetic** LUT fixture (category (a)) vs lcms2~~ | cross-check | ΔE2000 | — | **★ MEASURED 2026-08-11 (later still) — see §3.4.4 §B.** `fixtures/synthetic/v4-cmyk-mab-lab.icc` is now driven in both directions against `transicc` *and* against a closed form. Four of its rows need **no system profile at all**, so they are the first graded rows in this suite that survive on a machine without the Windows colour directory. The `mft2`/`mft1` synthetic fixtures are still not wired in, so **every §3.4.1 row and every §3.4.4 §A and §C row still skips** there. | **2026-08-11 (later still)** — §3.4.4 |

> The v2/v4 Lab encoding rows are marked **ground truth** deliberately.
> `ARCHITECTURE.md` §2 names this as the single richest source of CMM
> bugs, and it is the case where "lcms2 agrees" is least reassuring: an
> encoding difference of exactly the kind at issue would be shared by any
> implementation that read the clause the same way. These must be settled
> from the specification text, not from the oracle.

### 3.4.4 Pass 4b — the B2A direction, the v4 element pipeline, and the grayTRC model

**Filled 2026-08-11 (later still) by `icc-conformance`** from comparisons
actually run. Apparatus and full derivations: **`tools/difftest/README.md`
§15**, and the tolerance constants in `tools/difftest/src/pass4b.rs`, each
carrying its derivation as a doc comment. Per-point record and the experiments:
`cargo run --bin pass4b_report`.

Three independent sections, three different corpora, and **they do not share a
scope statement**:

| § | what runs | skips without the Windows colour directory? |
|---|---|---|
| **A** | `sRGB Color Space Profile.icm` → `USWebCoatedSWOP.icc` (`mft1` B2A, 3→4, 33³, 8-bit), 213 RGB points end to end + 258 Lab points PCS-side, perceptual and media-relative, `-c0` | **yes** — both category (c) |
| **B** | `fixtures/synthetic/v4-cmyk-mab-lab.icc` (`mAB ` 4→3 ragged 5×4×3×2, `mBA ` 3→4 3³), 128 CMYK + 258 Lab points, media-relative | **no** for the four derived rows; yes for the two end-to-end rows, which need sRGB |
| **C** | `ewgray22.icm` → sRGB (Annex **F.2**, no LUT), 69 points on the gray axis, perceptual and media-relative | **yes** |

#### 3.4.4.1 ★ The new kind — `derived-expectation`, and what it is worth

§B introduces a fourth [`Kind`] (§1 carries a pointer to this subsection). An
expectation is a `derived-expectation` when it is computed by **arithmetic**
from (a) the specification's stated element order and encoding and (b) the bytes
of a **synthetic** fixture, with **no implementation's output in it**.

**Why it is not ground truth.** Nobody at the CIE or the ICC printed the number.
A reader of this repository derived it from clause text. Calling it ground truth
would overstate the chain of custody, and §1's whole point is that a weak claim
must not become quotable as a strong one.

**Why it is nevertheless stronger than a cross-check.** A cross-check is
defeated when both implementations share a misreading. A derived expectation is
defeated only when **the derivation** shares it — and the derivation is written
out next to the number, in a form a specification reader can check against the
standard without running anything.

**What it cannot do, stated as prominently.** The fixture and the derivation are
read out of the **same corpus** by the same project. If `ICC_Spec`'s
transcription of clause 10.12/10.13 is wrong, the fixture's bytes and this
expectation are wrong **together** and agree perfectly. That is exactly the
failure mode a *third* reading — lcms2's — is retained to catch, which is why
**every derived row below is paired with a cross-check row over the same
points**, and why the "published value" row of §3.4.3 stays blank.

#### 3.4.4.2 §A — the graded rows, RGB → CMYK through a `lut8` B2A

| Comparison | Kind | Metric | Tolerance | Justification | Measured |
|---|---|---|---|---|---|
| **A0.** The apparatus: the harness's own `lut8` reimplementation vs `iccce-cmm`'s `Lut16Model::pcs_to_device`, every Lab point — `pass4b/srgb-to-swop/<intent>/apparatus-lut8-matches-iccce-cmm` | **self-consistency** | abs-max, device 0..1 | **1×10⁻⁹** | **The precondition for believing anything else in §A**, and the same argument as §3.4.1 row 0: the experiments need one pipeline evaluated several ways differing in one component, and that substitution cannot be made inside `crates/`. 1×10⁻⁹ is ~7 orders above `f64` noise on this arithmetic and ~5 below anything colorimetric. | 2026-08-11 (later still) — **observed 0,0 exactly** (bit-identical), both intents |
| **A1.** sRGB → SWOP, iccce vs lcms2, **device space** — `pass4b/srgb-to-swop/<intent>/device-vs-lcms2` | **cross-check** | abs-max per component, normalised device 0..1 | **5×10⁻⁴** | **The quantisation envelope computed from lcms2's OWN arithmetic**, with no lcms2 output in it: the harness models every rounding lcms2 performs in this pipeline — 256-entry input curves rounded to 1/65535 in *and* out (`cmsEvalToneCurveFloat`, `nSegments == 0`), the CLUT stage input rounded to `u16` and its output returned as `u16/65535` (`EvaluateCLUTfloatIn16`), the output curves twice more, and the source's 1024-entry `curv` TRCs likewise — and propagates it through the actual B2A table: **1,330×10⁻⁴** (media-relative), **9,602×10⁻⁵** (perceptual). 5×10⁻⁴ is the larger with ~276 % headroom for the two roundings *not* modelled (lcms2 interpolates its curves and its CLUT in **16-bit fixed point**; the model uses `f64`). **The interpolation-method term is ZERO** — `_cmsReadOutputLUT` forces trilinear for a Lab-PCS LUT (README §15.2.2), and trilinear over three inputs *is* iccce's n-linear. **GRID-DEPENDENT BY CONSTRUCTION**; arithmetic-agreement, **not** perceptual. | 2026-08-11 (later still) — **observed 1,100×10⁻⁴** (perceptual), **1,330×10⁻⁴** (media-relative) — **within 0,02 % of the envelope** |
| **A2.** …the same, **mean** — `…/device-mean` | cross-check | abs-mean, 0..1 | **∞ — REPORTED, NOT GRADED** | A mean over a grid hides exactly the outlier a colour engine gets wrong. On file next to the max; **never to be quoted for it.** | 2026-08-11 — 2,362×10⁻⁵ / 2,546×10⁻⁵ |
| **A3. ★★ …with lcms2's own arithmetic modelled** — `…/device-lcms2-arithmetic-modelled` | **cross-check** | abs-max, 0..1 | **5×10⁻⁵** | **This is the row that claims agreement.** With every lcms2 rounding switched on in the harness's model, what must remain is `transicc`'s 4-decimal CMYK print floor (10⁻⁶ normalised) plus the two 16-bit **fixed-point** interpolations the `f64` model does not reproduce (~1,5×10⁻⁵ each). 5×10⁻⁵ is that sum, and it is **10× tighter than A1**. ★ Observed **3,100×10⁻⁵ = 2,03 lsb of 1/65535**, at both intents *and* on the PCS-side row — the residual is not merely under the bound, it is exactly the two roundings left out, three times independently. | 2026-08-11 (later still) — **3,101×10⁻⁵ / 3,100×10⁻⁵** |
| **A4.** …the same disagreement in ΔE2000, both sides' CMYK carried back through **the same file's `A2B1`** — `…/roundtrip-lab-de2000` | **cross-check** | ΔE2000 max, `kL=kC=kH=1`, D50 CIELAB | **5×10⁻²** | Four ink components have **no perceptual metric**; a device number in CMYK cannot be compared to §2's anchor until it is in a space where a ΔE means something. The route back is the same profile's own colorimetric table — a *round trip*, not a second opinion, and the record says so. Bound: `A2B1`'s steepest node-to-node step is ≈0,1 normalised `L*` per 1/8 device, so `dL*/d(device) ≲ 80`; 1,330×10⁻⁴ × 80 = 1,06×10⁻² `L*` per ink, ×2 because the four inks move together and add, ÷ `S_L ≈ 1,2` ≈ **1,8×10⁻² ΔE00**; 5×10⁻² is ~2,8× that. **20× below §2's ⚠ anchor**, whose ⚠ it inherits. | 2026-08-11 (later still) — **7,095×10⁻³** (perceptual), **5,711×10⁻³** (media-relative) |
| **A5. ★ The sensitivity control** — the same table evaluated **tetrahedrally** — `…/counterfactual-tetrahedral` | cross-check | abs-max, 0..1 | **∞ — REPORTED, NOT GRADED** | **A counterfactual, not a comparison**: computed from the B2A table and the two geometries alone, no lcms2 output. It exists because A1's headline result is *"the interpolation-method difference is zero"*, and a comparison that could not detect a geometry difference would report the same thing. It can: **1,527×10⁻² / 1,311×10⁻², i.e. 139× and 99× the observed disagreement.** Ungraded because there is nothing to grade — neither number is an error. | 2026-08-11 (later still) — 1,527×10⁻² / 1,311×10⁻² |
| **A6.** Lab → SWOP `B2A1` with the source model removed — `pass4b/lab-to-swop/media-relative/pcs-device-vs-lcms2` and `…-lcms2-arithmetic-modelled` | **cross-check** | abs-max, 0..1 | **5×10⁻⁴** / **5×10⁻⁵** | The same two rows with **no source profile in the chain at all** (`transicc -i*Lab4`), so the B2A tag is isolated from the sRGB TRC quantisation. **iccce is IN-PROCESS here and the record says so**: the shipped CLI has no Lab entry point, so these two rows grade the **model**, not the binary — the one place in Pass 4b where that is true. | 2026-08-11 (later still) — **6,485×10⁻⁵** and **3,097×10⁻⁵** |

#### 3.4.4.3 §B — the v4 element pipeline, and the first derived expectations

| Comparison | Kind | Metric | Tolerance | Justification | Measured |
|---|---|---|---|---|---|
| **B0.** Both interpolation geometries on the fixture's own CLUTs — `pass4b/fixture/clut-is-affine-both-geometries-agree` | **self-consistency** | abs-max, normalised | **1×10⁻¹⁴** | **The precondition for B1–B4.** Both CLUTs store a function affine in one input and constant in the others, and every geometry reproduces an affine function exactly **in exact arithmetic** — but the two algorithms reach that value by different sequences of `f64` operations, so they agree to *rounding*, not bit-identically. The n-linear arm sums 2⁴ = 16 products of values in [0,1], so ~16 ulp = 3,6×10⁻¹⁵; 1×10⁻¹⁴ is ~3× that and 11 orders below one `u16` lsb. **SUPERSEDES a 0,0 whose justification confused real arithmetic with floating point — see §4.** | 2026-08-11 (later still) — **1,110×10⁻¹⁶** |
| **B1. ★★ `mAB ` (CMYK→Lab): iccce vs the closed form** — `pass4b/fixture/mab/iccce-vs-derived-expectation` | **derived-expectation** | abs-max, `L*`/`a*`/`b*` units | **1×10⁻¹²** | The expectation is `L* = 100(1−K) + 0,390625`, `a* = 1,9921875`, `b* = 2,98828125`, derived from **10.12.1** (element order), **10.12.5** (the 3×4 matrix, offsets applied in the *normalised* domain) and **6.3.4.2 Tables 12/13** (the **general** 16-bit PCSLAB encoding — `mAB ` is not in NOTE 3's legacy set), plus the fixture's own stored nodes. iccce evaluates in `f64` with no intermediate quantisation, so the residual is a few ulp; an ulp of `L*` near 100 is 1,4×10⁻¹⁴, so 1×10⁻¹² is ~70 of them — and **7 orders below one `u16` lsb, so a single-lsb error in a stored node still fails.** iccce is **IN-PROCESS** (`LutAbModel`). **EXCLUDES the 10 encoded-PCS-overflow points** (B5). | 2026-08-11 (later still) — **2,842×10⁻¹⁴** |
| **B2.** `mAB `: **lcms2** vs the same closed form — `…/mab/lcms2-vs-derived-expectation` | **derived-expectation** | abs-max, `L*` units | **1×10⁻²** | **The third reading** — what stops the fixture and the derivation being wrong together. lcms2's residual against an exact form is its own quantisation: CLUT input rounded to `u16` (½ lsb = 7,6×10⁻⁶ of the axis), output returned as `u16/65535` (1 lsb = 1,5×10⁻⁵), plus fixed-point interpolation; in `L*` units (`L* = 100n`) ≈3×10⁻³. 1×10⁻² is ~3× that — **still 40× below the 0,390625 `L*` matrix offset this row confirms is applied**, and 100× below the 0,39 % legacy-vs-general PCSLAB confusion it would also catch. Uses the **unclamped** reading, which is what lcms2 computes. | 2026-08-11 (later still) — **2,325×10⁻³** |
| **B3. ★★ `mBA ` (Lab→CMYK): iccce vs the closed form** — `…/mba/iccce-vs-derived-expectation` | **derived-expectation** | abs-max, device 0..1 | **1×10⁻¹²** | Mirror of B1 from **10.13.1/10.13.4**: `C=M=Y=0`, `K` interpolated along the `L*` axis alone at `n_L = L*/100 + 1/256`. ★ The expectation uses **the stored nodes including their `u16` rounding** — the middle node is `round(0,5·65535) = 32768`, i.e. 0,500 007 63, **not** 0,5; an idealised `1 − L` would be wrong by 7,6×10⁻⁶ and would look like an implementation defect. **This row is also the regression for GP-001**: the `mBA ` curve counts (B=3, M=3, A=4 for a 3-in/4-out tag) are what make the chain evaluate at all. | 2026-08-11 (later still) — **2,220×10⁻¹⁶** |
| **B4.** `mBA `: **lcms2** vs the same closed form — `…/mba/lcms2-vs-derived-expectation` | **derived-expectation** | abs-max, device 0..1 | **1×10⁻⁴** | B2's mechanism stated in the units *this* row is measured in rather than converting an `L*` bound into device units, which nobody would be able to check: 1 lsb of CLUT output is 1,5×10⁻⁵ of the device range and the `K` axis has unit slope, so nothing amplifies it; 1×10⁻⁴ is ~4×. | 2026-08-11 (later still) — **1,873×10⁻⁵** |
| **B5.** sRGB → fixture (`mBA `), shipped binary vs `transicc` — `pass4b/srgb-to-fixture/media-relative/device-vs-lcms2` | **cross-check** | abs-max, device 0..1 | **1×10⁻⁴** | The method envelope is zero by construction (B0), so what is left is lcms2's `u16` quantisation of the CLUT boundary carried into device units: 1 lsb = 1,5×10⁻⁵, unamplified because this table has unit slope in device per normalised PCS unit. 1×10⁻⁴ is ~6× that, covering fixed-point interpolation and `transicc`'s print floor. | 2026-08-11 (later still) — **5,200×10⁻⁵** |
| **B6.** fixture → sRGB (`mAB `), shipped binary vs `transicc` — `pass4b/fixture-to-srgb/media-relative/device-vs-lcms2` | **cross-check** | abs-max, device 0..1 | **2,5×10⁻⁴** | **Deliberately NOT B5's number, because the destination is not the same kind of thing.** Converting *into* the fixture ends at a CLUT; converting *out of* it ends at sRGB's inverse tone curves, which lcms2 builds as a 4096-entry `u16` resampling (`cmsReverseToneCurveEx`) — the term §C measures independently at **9,68×10⁻⁵ device on the same destination** — plus the fixture CLUT's own 1,5×10⁻⁵ carried through: ≈1,15×10⁻⁴, and 2,5×10⁻⁴ is ~2,2×. **SUPERSEDES sharing B5's 1×10⁻⁴, which failed at 1,012×10⁻⁴ because the derivation omitted the destination — see §4.** **EXCLUDES the 10 overflow points** (B7). | 2026-08-11 (later still) — **1,012×10⁻⁴** |
| **B7. ★★ The encoded-PCS overflow** — `pass4b/fixture/mab/encoded-pcs-overflow-divergence` | cross-check | ΔE2000 max | **∞ — REPORTED, NOT GRADED** | At `K = 0` the `mAB ` CLUT's `L*` node is full scale and the 3×4 matrix then adds `+1/256`, so the value handed to the `B` curves is **1,003 906 25 — outside the range of the encoding it is about to be read as**. **iccce clamps** (its `Trc::eval` enforces clause 10.18's `[0,1]` curve domain) → `L* = 100`. **lcms2 does not** (a `count = 0` `curv` becomes a γ=1 parametric segment with domain ±10²², evaluated unbounded) → `L* = 100,390625`. **Cost 0,6117 ΔE2000** over 10 points — the largest disagreement anywhere in Pass 4b and near §2's anchor. **Which the specification requires is UNSETTLED**; the question is written out verbatim in `tools/difftest/README.md` §15.3.3 and a dispatch to `icc-spec-librarian` is **owed**. Grading it would mean either a ~0,7 ΔE tolerance chosen because it passed or a permanent red line; both were rejected in writing, as with §3.4.2's absolute-intent rows. **When it is settled, one implementation acquires a defect and this becomes a graded row.** | 2026-08-11 (later still) — **observed 0,6117 ΔE2000**, device 4,440×10⁻³, over 10 of 128 points |
| **B8. ★ Forced BPC, measured in both directions** — `pass4b/fixture/forced-bpc-is-decided-by-the-DESTINATION-version` | **oracle-reproducibility** | abs-max, device 0..1 | **∞ — REPORTED, NOT GRADED** | **BOTH SIDES ARE lcms2** — its own media-relative output against its own perceptual output, on one pair of profiles, in each direction. It says nothing whatever about iccce. What it says is that **DL-013 / corpus M2 is half a rule**: `_cmsLinkProfiles` sets `BPC[i]` per profile, but `DefaultICCintents` consumes it as `ComputeConversion(i, …, BPC[i], …)` — the conversion **into** `hProfiles[i]` — so the **destination** profile's version decides. v4 fixture as *source* into a v2 destination: **0,0, bit-identical**. v2 source into the v4 fixture as *destination*: **3,137×10⁻²** (`K` at black 99,6094 % → 96,4721 %). Anyone using M2 to decide whether a comparison is confounded needs the direction, not just the version. | 2026-08-11 (later still) — **0,0 / 3,137×10⁻²** |

#### 3.4.4.4 §C — the grayTRC model (Annex F.2)

| Comparison | Kind | Metric | Tolerance | Justification | Measured |
|---|---|---|---|---|---|
| **C1.** `ewgray22.icm` → sRGB, iccce vs lcms2, **device** — `pass4b/gray-to-srgb/media-relative/device-vs-lcms2` | **cross-check** | abs-max, device 0..1 | **2,5×10⁻⁴** | **The source cannot contribute**, and that is established before the run rather than hoped: both implementations evaluate the same *analytic* γ = 2,199 218 75 (lcms2 turns a single-value `curv` into a type-1 parametric curve, so §3.4's tabulated-curve quantisation does not apply) and multiply by the **same D50 literals** — `cmsD50X/Y/Z` = 0.9642/1.0/0.8249 in `lcms2.h`, and `iccce_color::D50` is the same three. So this is the **destination alone**: lcms2 inverts each 1024-entry `curv` with `cmsReverseToneCurveEx(4096)`, a `u16` resampling whose knots do not coincide with the forward table's, then evaluates it through the float path that rounds input and output to 1/65535. Envelope computed from that model over this axis: **9,680×10⁻⁵**; 2,5×10⁻⁴ is ~2,6×. **SUPERSEDES a 1×10⁻⁴ whose stated envelope (3,45×10⁻⁵) was a pre-run guess — see §4.** Arithmetic-agreement, **not** perceptual. | 2026-08-11 (later still) — **9,686×10⁻⁵**, which is 0,06 % *above* the envelope (the observation additionally carries both binaries' print floors) |
| **C2.** …the same, **mean** — `…/device-mean` | cross-check | abs-mean, 0..1 | **∞ — REPORTED, NOT GRADED** | As A2. | 2026-08-11 — 1,782×10⁻⁵ |
| **C3.** …the same in ΔE2000 — `…/de2000-vs-lcms2` | **cross-check** | ΔE2000 max | **5×10⁻²** | ★ **The maximum is near BLACK, and the reason inverts §6.2's note.** §13.6 recorded that near black the *device* metric explodes while ΔE stays small — that is the inverse TRC's slope acting on a device comparison. Here the comparison is already in device units and the amplification runs the other way: below sRGB's linear breakpoint a device difference `δ` becomes `δ/12,92` of linear light, and CIELAB's **chromatic** sensitivity on *its* linear segment is `da*/dX = 500·7,787/X_n = 4038`. With the three channels carrying slightly different `δ` (independent reverse tables): `ΔL* ≈ 69,9 δ = 6,8×10⁻³` but `Δa* ≈ 136 δ = 1,3×10⁻²`, and near neutral `S_C ≈ 1` while `S_L ≈ 1,75`, so the chromatic term is larger by ~3×. Union ≈2×10⁻²; 5×10⁻² is ~2,4× that and **20× below §2's ⚠ anchor**, whose ⚠ it inherits. **SUPERSEDES a 1×10⁻² derived at white, which failed at 2,17×10⁻² — see §4.** | 2026-08-11 (later still) — **2,169×10⁻²** |
| **C4. ★★ The attribution** — lcms2's destination modelled — `…/device-lcms2-arithmetic-modelled` | **cross-check** | abs-max, 0..1 | **5×10⁻⁶** | **This is the row that claims agreement**, and it is the tightest gate in Pass 4b. With `cmsReverseToneCurveEx(4096)` reimplemented and both 1/65535 roundings applied, what must remain is `transicc`'s 4-decimal RGB print floor (3,9×10⁻⁷ normalised) plus the unmodelled parts (16-bit fixed-point table interpolation, `float32` matrix). 5×10⁻⁶ is ~13 print lsb and **50× tighter than C1**. ★ Observed **2,121×10⁻⁷ — below the print floor**: a **457×** collapse. The disagreement is not merely explained, it is *reproduced*. | 2026-08-11 (later still) — **2,121×10⁻⁷** |
| **C5.** Perceptual and media-relative are the same transform, both sides — `…/perceptual-equals-media-relative` | cross-check | abs-max, 0..1 | **0,0 — exact** | A monochrome profile carries **no `A2Bx`/`B2Ax` at all**, so clause 8.10.2's intent-indexed selection has nothing to select and both intents fall through to step 4's F.2 model; the destination is matrix/TRC with the same property. **No arithmetic in either chain could make the difference small** — any difference is an intent-dispatch defect. Exact equality is the only honest bound, as with §3.4.1 row 6. (ICC-absolute is excluded and *does* differ: it reads `wtpt`, which is §3.4.2's finding.) | 2026-08-11 (later still) — **0,0** on both sides |

#### 3.4.4.5 What Pass 4b did NOT measure

> **★ SUPERSEDED IN PART, 2026-08-12.** The first bullet's saturation half is
> now measured — see **§3.4.4.6**. The bullet is left standing because §4's
> convention is that a record of what was believed and when is not overwritten;
> everything else in it still holds, including ICC-absolute.

- **Saturation and ICC-absolute in any of the three directions.** `B2A2` exists
  and is a third distinct table; ICC-absolute through a **LUT destination** would
  exercise §3.4.2's white-point policy where the D.6/D.7 composite is applied
  *before* the PCS is encoded rather than after — a case Pass 4 could not reach.
- **`lut8` with an XYZ PCS.** `iccce-cmm` refuses it by name (the 8-bit XYZ
  encoding is unsourced in the corpus); nothing here changes that.
- **Any real v4 LUT profile.** A sweep of all **40** `.icc`/`.icm` files in this
  machine's colour directory found **zero** `mAB `/`mBA ` tags. §B's claims are
  about **one synthetic fixture**, and no wider statement is available on this
  machine at any price.
- **The M3 out-of-range divergence.** §A's 48 saturated-hue Lab points are the
  first grid in this suite genuinely outside the destination gamut, and the
  count of out-of-`[0,1]` components `transicc` returned was **not recorded** on
  this run. Still owed (`tools/difftest/README.md` §13.10 item 1).

#### 3.4.4.6 ★ §A extended 2026-08-12 — the SATURATION table (`B2A2`), and Pass 4's done-when clause closed

**Run 2026-08-12 by `icc-conformance`.** Apparatus: the same
`tools/difftest/src/pass4b.rs` §A, with `(Intent::Saturation, tag::B2A2)` added
to its intent loop. Full record: `tools/difftest/README.md` **§15.8**.

Pass 4's done-when clause explicitly failed on this: only perceptual and
media-relative had ever been run in the **B2A** direction, and §A's own doc
comment put saturation out of scope with the sentence *"saturation adds a third
copy of the same shape"*. **That sentence was an assumption and it was wrong**,
and the row that now precedes the intent runs is the one that says so.

| Comparison | Kind | Metric | Tolerance | Justification | Measured |
|---|---|---|---|---|---|
| **A0. ★ The three `B2A*` tags are three distinct tables** — `pass4b/srgb-to-swop/b2a-tags-are-three-distinct-tables` | self-consistency | count of byte-identical pairs | **0,0 — exact** | **The PRECONDITION for reading the saturation rows as measurements at all, and one of the few places `0,0` is honestly available**: the quantity is a count of integer comparisons on **raw file bytes**, with no parser in the way, so there is no rounding for a tolerance to absorb (§3.4.4 row B0's "same operations in the same order" rule is about arithmetic; this is not arithmetic). **What it catches is not hypothetical — it is true one direction away in the same file.** Pass 4 found `A2B0` and `A2B2` sharing **one** block at **one** offset and graded their equality at exactly zero for that reason; had `B2A0`/`B2A2` been laid out the same way, the three rows below would have reproduced the perceptual rows bit for bit and the suite would have gained green lines that measured nothing. | 2026-08-12 — **0**. `B2A0`@83 392, `B2A1`@228 980, `B2A2`@374 568, all 145 588 B. Differing bytes: 0-vs-1 **71,4 %**, 0-vs-2 **66,2 %**, 1-vs-2 **70,4 %**. The least-distinct pair still differs in two thirds of its bytes. |
| **A6. Saturation, device space** — `pass4b/srgb-to-swop/saturation/device-vs-lcms2` | **cross-check** | abs-max, device 0..1 | **5×10⁻⁴** (**`DEVICE_B2A`, unchanged**) | The same computed quantisation envelope as perceptual and media-relative, re-derived on **`B2A2`**, which is the **steepest of the three tables**: **1,552 5×10⁻⁴**, against 1,330×10⁻⁴ (media-relative) and 9,602×10⁻⁵ (perceptual). **The constant did not move; the `why` string did** — see §4. Headroom over the worst of the three is now ~222 % rather than ~276 %. | 2026-08-12 — **1,550 0×10⁻⁴**, i.e. **99,8 % of the envelope**. The same signature as the other two intents: the disagreement is *accounted for*, not merely small. |
| **A7. Saturation, the attribution** — `…/saturation/device-lcms2-arithmetic-modelled` | **cross-check** | abs-max, device 0..1 | **5×10⁻⁵** (`DEVICE_B2A_MODELLED`, unchanged) | Every lcms2 rounding modelled; what is left must be lcms2's **fixed-point** arithmetic and `transicc`'s print floor. | 2026-08-12 — **3,098 96×10⁻⁵**, i.e. **2,03 lsb of 1/65535** — the *same* figure as perceptual, media-relative and the PCS-side row, to three significant figures. Four independent comparisons landing on the same two roundings. |
| **A8. Saturation, ΔE2000 round trip** — `…/saturation/roundtrip-lab-de2000` | **cross-check** | ΔE2000 max, D50 | **5×10⁻²** (`DE_B2A_ROUNDTRIP`, unchanged) | Both sides' CMYK carried back through SWOP's own `A2B1`. | 2026-08-12 — **7,062 75×10⁻³** |
| **A9. Saturation, apparatus** — `…/saturation/apparatus-lut8-matches-iccce-cmm` | self-consistency | abs-max | **1×10⁻⁹** (`APPARATUS_B2A`, unchanged) | The harness's `lut8` pipeline must be the crate's. | 2026-08-12 — **0,0** |
| **A10. Saturation, the sensitivity control** — `…/saturation/counterfactual-tetrahedral` | **cross-check** | abs-max, device | **∞ — REPORTED, NOT GRADED** | What the disagreement *would* have been had `_cmsReadOutputLUT` not forced trilinear for a Lab-PCS LUT. | 2026-08-12 — **2,960 0×10⁻²**, **191×** the observed residual. The comparison could have seen a geometry difference two orders larger, so A6 is not a null from an instrument that could not tell. |

**Coverage, stated in the same breath as the numbers.** Saturation is now
verified in the **B2A** direction on **one** profile pair
(`sRGB → USWebCoatedSWOP`), **one** tag type (`mft1`/`lut8`, 3→4, 33 nodes),
**213** RGB points, Windows/MSVC, pin `21c582a`. It says nothing about
saturation in the **A2B** direction (where Pass 4 showed this file aliases
`A2B0`/`A2B2` and the intent is therefore untested by construction), about
saturation through a **v4** element pipeline, or about **ICC-absolute**, which
remains out of scope for the reason §3.4.4.5 gives.

### 3.4.5 ★★ Pass 4c — ICC-absolute through a LUT destination, measured with lcms2's `wtpt` substitution held at ZERO

**Run 2026-08-12 by `icc-conformance`**, apparatus
`tools/difftest/src/pass4c.rs`, 10 records, **all pass**. This closes the last
measurement item of Pass 4 and it is the row §3.4.4.5's first bullet said was
out of reach.

> **★ SUPERSEDES §3.4.4.5's ICC-absolute half.** That bullet is left standing
> per §4's convention. Two things in it are now wrong and both are worth
> naming: the composite is cited there as **"D.6/D.7"**, a label that is **not
> edition-stable** (in `ICC.1:2001-04` Annex D the equations are (D.1)–(D.6),
> there is **no (D.7)**, and that edition's (D.6) is the single `Z` component
> of the *inverse*); and ICC-absolute through a LUT destination was treated as
> permanently blocked, which it was not — it was blocked on a **profile pair**,
> not on a document. The edition-stable citation is **`ICC.1:2022` 6.3.2.2
> Eq (4)–(6), restated verbatim at D.6.1 Eq (D.7)**.

#### Why this could not be measured before, and what changed

Pass 4's ICC-absolute row (**NC-053**, 11,217 ΔE2000) is dominated by a
**policy** difference, not an arithmetic one: `cmsio1.c`'s
`_cmsReadMediaWhitePoint` substitutes D50 for a stored `wtpt` when a profile is
**both** version < 4 **and** class `'mntr'`. The only gate the ICC-absolute
path had was **NC-054**, which grades a *model*. **A model can absorb a genuine
arithmetic error along with the policy difference it was built to isolate**,
and nothing in the suite could tell the difference.

What changed is not a document — it is the recognition that lcms2's predicate
is a **conjunction**, and that breaking either half on **both** profiles makes
the policy difference *structurally absent* rather than modelled or tolerated:

| role | profile | version | class | why the gate is not taken |
|---|---|---|---|---|
| source | `fixtures/synthetic/v4-rgb-matrix-trc.icc` | **4.4.0** | `'mntr'` | fails the **version** half |
| destination | `USWebCoatedSWOP.icc` | 2.1.0 | **`'prtr'`** | fails the **class** half |

Each fails a *different* half, so the pair is not quietly resting on one
property. The second Pass 4 confound is zero here too: lcms2 forces trilinear
for any Lab-PCS output LUT and trilinear over three inputs **is** iccce's
n-linear (NA-006 = 0), and the source has no CLUT at all.

#### §A — the graded rows

| Comparison | Kind | Metric | Tolerance | Justification | Measured |
|---|---|---|---|---|---|
| **C0. ★ Neither profile trips lcms2's `wtpt` gate** — `pass4c/v4matrix-to-swop/precondition-neither-profile-trips-lcms2-wtpt-gate` | self-consistency | count over 2 profiles | **0,0 — exact** | **The PRECONDITION for reading every row below as a measurement of arithmetic.** A count of files satisfying `version < 0x4000000 AND class == 'mntr'`, read from the two parsed headers *of the files actually opened* — not asserted in a comment, because a profile can be replaced on a machine. Exact zero because it is a count, not a float. | 2026-08-12 — **0**. src v04400000 `'mntr'` wtpt=(0,9642, 1,0000, 0,8249); dst v02100000 `'prtr'` wtpt=(0,7084, 0,7359, 0,5710) |
| **C1. ★★ ICC-absolute, device space** — `…/absolute/device-vs-lcms2` | **cross-check** | abs-max, device 0..1 | **5×10⁻⁴** (**`DEVICE_B2A`, REUSED UNCHANGED**) | Same destination table (SWOP `B2A1`, `mft1`/`lut8`, 3→4, 33 nodes, 8-bit), same evaluator, same direction as Pass 4b §A, so the same quantisation envelope applies and **transfers with its justification intact**. **Minting a fresh constant fitted to this observation would have been a number chosen because it passed** — §3.4.4.6 set the precedent when saturation reused it. | 2026-08-12 — **8,900×10⁻⁵** over **729** RGB points |
| **C2. The FLOOR — media-relative on the same pair and grid** — `…/media-relative/device-vs-lcms2` | **cross-check** | abs-max, device 0..1 | **5×10⁻⁴** (same constant) | **This is the comparison C1 actually has to beat**, and it is the sharper claim: an intent with no absolute scaling in it at all, on the same two files and the same grid, isolating the 8-bit `lut8` cost of this direction. | 2026-08-12 — **1,080×10⁻⁴**. **C1 (8,90×10⁻⁵) is BELOW its own floor**: the ICC-absolute arithmetic adds nothing detectable above the cost the direction already carries. |
| **C3. ★★ The counterfactual — EXACT, not modelled** — `…/absolute/counterfactual-wtpt-substituted` | self-consistency | abs-max, device 0..1 | **∞ — REPORTED, NOT GRADED** | Because the source's stored `wtpt` **is** D50, substituting D50 for the *destination's* `wtpt` collapses the whole 6.3.2.2 diagonal to identity — so **absolute-vs-media-relative on this pair IS what lcms2's substitution would have cost here**, computed rather than assumed. It is the NC-053 mechanism, priced on this pair. | 2026-08-12 — **2,055 76×10⁻¹** |
| **C4. The sensitivity floor** — `…/absolute/sensitivity-floor` | self-consistency | violation `max(0, 100 − r)` | **0,0** | **DL-025's row.** The floor of 100 is **transcribed from Pass 4b's already-accepted counterfactual band (99×, 139×, 191×)** on this same table and direction — a measured band, not a number chosen to clear this observation. It is what stops 8,9×10⁻⁵ from being a magnificent measurement of nothing. | 2026-08-12 — **0,0**; observed ratio **2 310×**, twenty-three times the floor |
| **C5. The degeneracy guard** — `…/absolute/degeneracy-guard-unmoved-fraction` | self-consistency | fraction of grid | **0,05** | The guard against the *other* null: if the absolute scaling had pushed the grid out of the destination gamut, **both implementations would clamp to the same boundary and agree perfectly while computing nothing.** A diagonal scaling fixes the origin, so device black is expected and is arithmetic, not a defect — 1/729 = 1,4×10⁻³, and the budget sits an order of magnitude above that one expected fixed point. | 2026-08-12 — **1,371 74×10⁻³** = **1 point of 729** |

#### §B — the same policy, measured in the OTHER direction (DL-021)

NC-053 measured the substitution with the v2 `'mntr'` profile as
**destination**. §B measures it with the same file as **source**.

**The prediction was written before the run** (DL-023): the divergence should
move to the source side and stay **large**, order 10⁻¹ device units, because
iccce scales by `D65/D50 = (0,98579, 1,0, 1,32027)` where lcms2 scales by
identity. Had it come out **small**, NC-053's mechanism would have been wrong
about its own generality — the more interesting outcome, and one this project
has already had once (DL-011 predicted a divergence, DL-012 measured it
**absent**).

| Comparison | Kind | Metric | Tolerance | Measured |
|---|---|---|---|---|
| **C6. Exactly one profile trips the gate** — `pass4c/srgb-to-swop/precondition-source-DOES-trip-lcms2-wtpt-gate` | self-consistency | violation \|count − 1\| | **0,0 — exact**. Zero would mean §B measures nothing; two would mean the destination substitutes as well and the attribution is wrong | 2026-08-12 — **0,0** (src gate=true, dst gate=false) |
| **C7. ★ ICC-absolute, source-side substitution live** — `…/absolute/device-vs-lcms2` | **cross-check** | abs-max, device 0..1 | **∞ — REPORTED, NOT GRADED** | 2026-08-12 — **2,134 24×10⁻¹** over 729 points, **1 654×** its own media-relative floor. **The prediction held: the policy is direction-symmetric.** |
| **C8. §B's floor** — `…/media-relative/device-vs-lcms2` | **cross-check** | abs-max, device 0..1 | **5×10⁻⁴** (same constant) | 2026-08-12 — **1,290×10⁻⁴**. Without it, "0,21 device units" is a number with nothing to be large compared to. |

#### What §3.4.5 does NOT claim

- **Not that iccce's absolute arithmetic is correct.** Both C1 and C2 are
  cross-checks; two implementations can read 6.3.2.2 the same way and both be
  wrong. **Pass 4c creates no ground-truth row.** (`ICC.1:2022` Table D.2 does
  print an nCIEXYZ media white for SWOP — 0,706 7 · 0,734 6 · 0,570 3, within
  ~2×10⁻³ of this file's — but that is a published value for a **white point**,
  not for a transform, and a different characterization revision besides.)
- **Not that lcms2 is non-conforming.** That verdict is **not available**: the
  conformance clause (`ICC.1:2022` clause 5, `ICC.1:2001-04` clause 3) binds
  the ability to **read** profiles, not a CMM's computed output. Say lcms2
  **diverges**. *(Sourced by `icc-spec-librarian`, 2026-08-12.)*
- **Not that the source-side term is exercised in §A.** It is identity by
  construction — that is exactly what buys the exact counterfactual, and it is
  a cost as well as a benefit. §A measures the **destination-side** term, which
  is the term NC-053 got wrong.
- **Nothing about A4c.** Whether a profile's `wtpt` must agree with its own
  colorants is a separate ambiguity, still **SILENT**, and it did **not** clear
  when A4b cleared.
- **Coverage:** two profile pairs, one destination tag (`B2A1`), one grid
  (729 points, 9×9×9 on the 8-bit lattice), one machine, one pin, Windows/MSVC,
  one run, no repetition.

#### 3.4.5.1 ★ Candidate separations — all ten rows, 2026-08-12

Added by `icc-conformance` when §1.1's mechanism was extended past Pass 5c.
Every Pass 4c row now states one; **four are `Measured` and six are
`no-named-alternative` with their reason**, and the six are the more instructive
half.

**Three live readings of one predicate**, all of which this project has met, are
now priced by the two precondition rows instead of being argued in prose:

| reading | source | count on §A's pair | on §B's |
|---|---|---|---|
| `version < 0x4000000 AND class == 'mntr'` | `cmsio1.c` at the pin — what the **code** does | **0** | **1** |
| `class == 'mntr'` | **`ICC.1:2022` 9.2.36** — what the **standard** says, no version gate | 1 | 1 |
| `version < 0x4000000 OR class == 'mntr'` | the single-character misreading | 2 | 2 |

`Separation` holds **one** alternative, so each row names **the reading that is
the strongest threat to its own claim** and enumerates the others in the
alternative's text. That is not a convenience: on §A the class-only reading is
the threat (count 1 → the precondition fails → every number in the section is
measuring the policy again), while on §B it gives the *same* observation and the
threat is the disjunction instead. **Naming the rival that happens to flatter a
row is the tuning this mechanism exists to prevent**, so the choice is made on
threat and stated on the row.

| row | separation | verdict |
|---|---|---|
| `…/precondition-neither-profile-trips-lcms2-wtpt-gate` | class-only reading → count **1**; distance **1,0** against a tolerance of 0 | `DISCRIMINATING` |
| **`…/absolute/device-vs-lcms2`** — ★ the row the brief named | **the substitution having fired**, value **`2,055 76×10⁻¹`** = the counterfactual row's own number, which on this pair is **exact, not modelled**; distance `2,054 87×10⁻¹` against a tolerance of `5×10⁻⁴` | `DISCRIMINATING`, **411×** |
| `…/absolute/device-mean` | the same alternative reduced as a **mean** over the same 729 points (`1,175 27×10⁻¹`) — borrowing the max's counterfactual here would be Pass 6 row R4's population error | `UNGRADED` (tolerance ∞) |
| `…/absolute/counterfactual-wtpt-substituted` | the substitution firing on the **source** instead: **exactly 0**, because the source's stored `wtpt` already **is** D50. *That asymmetry is why this counterfactual is exact rather than modelled*, now stated on the row instead of in a paragraph | `UNGRADED` |
| `…/srgb-to-swop/precondition-source-DOES-trip-…` | the disjunction → count **2**; distance **1,0** | `DISCRIMINATING` |
| `…/srgb-to-swop/absolute/device-vs-lcms2` | **lcms2 *not* substituting** → the media-relative floor `1,29×10⁻⁴` measured on the same pair and grid in the same run. §A's separation mirrored | `UNGRADED` |
| both `media-relative/device-vs-lcms2` rows | **none** — lcms2 consults the media white point *only* for the ICC-absolute adjustment, so at this intent the predicate is never evaluated; and NA-006 is structurally zero here too. What is left is quantisation, and **quantisation has one value, not two** | `NO-NAMED-ALTERNATIVE` |
| `…/absolute/sensitivity-floor` | **none** — the only alternative nameable is a different **floor**, and that is a *tolerance* question answered in the row's `why` from Pass 4b's accepted 99×/139×/191× band. **Conflating a rival tolerance with a rival candidate is how a separation becomes a second, undocumented gate** | `NO-NAMED-ALTERNATIVE` |
| `…/degeneracy-guard-unmoved-fraction` | **none** — the `10⁻⁹` is a numerical-zero threshold, not an interpretation; and the null it guards against is the **hypothesis the row tests**, not a value it could have observed | `NO-NAMED-ALTERNATIVE` |

### 3.5 Pass 5 — black point compensation

**Filled 2026-08-11 (after Pass 4b) by `icc-conformance`** from comparisons
actually run. Apparatus and full derivations: **`tools/difftest/README.md`
§16**; the tolerance constants live in `tools/difftest/src/pass5.rs`, each
carrying its derivation as a doc comment. Per-scenario record, and **every
prediction printed next to its observation**: `cargo run --bin pass5_report`.

#### 3.5.1 ★ The comparable scenario set, and the one thing to read before any number below

Pass 5's rows do **not** grade "BPC". They grade three separable things, and
which one a row grades is stated on the row, because BPC has three rules — an
applicability set, an estimation method, and a forcing policy — each keyed on
something different.

The scenario set was derived from both implementations' sources **before
anything was run** (README §16.1 tabulates iccce's `Chain::with_bpc` subset
against lcms2's `cmsDetectBlackPoint` / `cmsDetectDestinationBlackPoint` guards
at pin `21c582a`), and it produced a consequence that has to be stated in
advance rather than inferred afterwards from a suspiciously small number:

> **★ Everywhere iccce will do BPC at all, lcms2's estimator reduces to the
> same two values.** On a matrix/TRC or gray side, lcms2's darkest-colorant
> estimate is device black through the profile at a colorimetric intent — which
> is exactly what iccce computes — and on every profile in reach that is exactly
> `XYZ (0,0,0)`, because every TRC in the corpus has `trc(0) = 0`. On a v4 LUT
> side at perceptual, lcms2's guard 3 returns the same A41 triple iccce
> hard-codes. **So the cross-check rows below grade the SCALING MAP, the
> DIRECTION and the pipeline the map sits in. They do NOT discriminate the two
> ESTIMATORS, and no row here may be quoted as if they did.**

| # | pair | intent | map | runs without the Windows colour directory? |
|---|---|---|---|---|
| **§A** | none — arithmetic against two documents | — | — | **yes**, and without the oracle too |
| **S1** | sRGB → Adobe RGB (1998), both v2 matrix/TRC | media-relative | identity | no |
| **S2** | `fixtures/synthetic/v4-cmyk-mab-lab.icc` → sRGB | perceptual | `PB → 0`, lowers | no |
| **S3** | sRGB → the same fixture | perceptual | `0 → PB`, raises | no |
| **S4** | sRGB → `fixtures/synthetic/v4-rgb-matrix-trc.icc` | perceptual | identity | no |
| **S5** | sRGB → `USWebCoatedSWOP.icc` | media-relative | iccce refuses | no |
| **S6** | two committed matrix fixtures | ICC-absolute | iccce refuses | **yes** |

#### 3.5.2 §A — the map, the only primary-specification rows Pass 5 has

| Comparison | Kind | Metric | Tolerance | Justification | Measured |
|---|---|---|---|---|---|
| **P1. ★★ `BpcScale(0 → PB)` vs ICC.1:2022 6.3.4.3's printed equation**, 1005 PCS values — `pass5/map/iccce-vs-icc1-6.3.4.3` | **derived-expectation** | abs-max, XYZ | **1×10⁻¹⁴** | **The only clause of the primary specification Pass 5 can cite.** `ICC_Spec/icc/icc__ref__bpc.md` §2 established that the BPC *scaling map* is in ICC.1:2022 after all — 6.3.4.3's v2→v4 perceptual-black adjustment `Xp = Xt(1 − Xb/Xi) + Xb` **is** the general map specialised to source black zero. The bound is arithmetic: the two forms are identical **in ℝ** and differ in `f64` only by rounding — ≈9 roundings on the longest path, each ≤1 ulp of 1,0 (2,22×10⁻¹⁶), amplified ≤1,04 by the division by `Xi − Xs ≈ 0,96`, so ≤**2,1×10⁻¹⁵**. 1×10⁻¹⁴ is ~4,8× that and **10 orders below one `u16` lsb of the encoded PCS**, so a dropped offset or a transposed numerator still fails. **NOT 0,0** — the two routes are not the same operations in the same order, which §4's B0 row identifies as the condition for 0,0. **The ESTIMATION has no such row and cannot (A42).** | 2026-08-11 (Pass 5) — **1,110×10⁻¹⁶** |
| **P2. `BpcScale(bs → bd)` vs a Gaussian elimination on Maria (2013) §4.2's two constraints**, 20 000 random draws — `…/iccce-vs-maria-two-constraint-solve` | **derived-expectation** | abs-max, XYZ | **1×10⁻¹⁴** | Generalises P1, which 6.3.4.3 can only state for `bs = 0`. The expectation is the paper's own pair of constraints solved by **elimination with the pivot on the white row** — deliberately a different sequence of `f64` operations from the closed form under test, so agreement is evidence about the algebra and not about a shared line of code. Same derivation as P1. **`published_literature`, retrieved compliantly (`littlecms.com/robots.txt` = `Allow: /`)**; not the standard. | 2026-08-11 — **3,331×10⁻¹⁶** |
| **P3. The two constraints hold under iccce's own map** (`apply(D50) = D50`, `apply(bs) = bd`) — `…/constraints-hold-under-iccce` | **derived-expectation** | abs-max, XYZ | **1×10⁻¹⁴** | Catches a map that is self-consistent but **anchored on the wrong white** — the failure the ICC-absolute exclusion exists to prevent, and the one a comparison against another implementation would miss if both were anchored wrongly. Also the step P8's end-to-end expectation rests on: "the source black maps to the destination black *exactly*" is a premise there and a measurement here. | 2026-08-11 — **3,331×10⁻¹⁶** |
| **P4. Equal black points are the exact identity**, 1001 values — `…/equal-blacks-are-the-exact-identity` | self-consistency | abs-max, XYZ | **0,0 — exact** | This is what makes S1's and S4's null results **interpretable**: with `bd = bs` the numerator `Xi − Xd` and denominator `Xi − Xs` are the same expression on the same bits, so `a = 1,0`, and `Xi(Xd − Xs)` is `Xi × 0`, so `b = 0,0`. No arithmetic could make the difference small rather than absent, so a non-zero value there is an **estimation** defect and not rounding. One of the two places in Pass 5 where 0,0 is available. | 2026-08-11 — **0,0** |
| **P5. ★ lcms2's `IsEmptyLayer` discriminant** — `…/lcms2-empty-layer-threshold` | derived-expectation | abs-max | **∞ — REPORTED, NOT GRADED** | **A constant this project had not recorded.** `cmscnvrt.c` L327–348 sums the BPC matrix's deviation from the identity plus its offsets (already divided by `MAX_ENCODEABLE_XYZ`) and **drops the entire stage below `0,002`**, so lcms2 silently performs no BPC once the two blacks are within roughly **0,41 `L*`**; iccce has no such threshold. For the S2/S3 map the discriminant is **0,015 342, 7,7× the threshold**, so nothing here is affected by it. **READ, not RUN** — no pair in reach has blacks close enough to trigger it. `ICC_Spec` §7.2's list of unattributed constants does not contain it (that list came from `cmssamp.c`); adding it is owed. | 2026-08-11 — **0,015 342** |

#### 3.5.3 §B — S2, the `PB → 0` direction (fixture → sRGB, perceptual)

128 CMYK points, **10 excluded** as §3.4.4's row-B7 encoded-PCS overflow.

| Comparison | Kind | Metric | Tolerance | Justification | Measured |
|---|---|---|---|---|---|
| **P6a. The baseline — BPC OFF on both sides** — `pass5/S2/fixture-to-srgb/perceptual/bpc-off-device-vs-lcms2` | **cross-check** | abs-max, device 0..1 | **2,5×10⁻⁴** | **Graded first on purpose.** A BPC-on agreement figure means nothing unless the same pair with BPC off is already known to agree; otherwise a residual that was there anyway is attributed to BPC. It reproduces row **B6**'s number exactly, which it should — it *is* B6, at an intent that selects the same tables. | 2026-08-11 — **1,012 157×10⁻⁴** |
| **P6b. ★★ BPC ON on both sides** — `…/bpc-on-device-vs-lcms2` | **cross-check** | abs-max, device 0..1 | **2,5×10⁻⁴** | **The row Pass 5's done-when clause 2 rests on.** Tolerance = **row B6's computed envelope × the map's own gain, and nothing else, because BPC adds no quantisation of the kind that envelope models**: (1) B6's envelope is lcms2's 4096-entry `cmsReverseToneCurveEx` resampling, measured independently by §3.4.4 row C1 at 9,68×10⁻⁵, plus the fixture CLUT's `u16` lsb of 1,5×10⁻⁵ ≈ **1,15×10⁻⁴**; (2) BPC inserts **one `cmsStageAllocMatrix`** between two stages `AddConversion` already had — no table lookup, no `u16` rounding — contributing only `f32` stage-boundary rounding, ≈6×10⁻⁸ relative, amplified ≤12,92 by sRGB's inverse TRC below its breakpoint = **7,8×10⁻⁷**; (3) the map multiplies the PCS by `a = Xi/(Xi − Xb) = 1,0035`. Envelope **1,16×10⁻⁴**; 2,5×10⁻⁴ is ~2,2×, deliberately B6's own headroom factor, since the term it covers (16-bit **fixed-point** interpolation) is unchanged by BPC. **★ SENSITIVITY: BPC moves this transform by up to 3,5159 ΔE2000 (4,30×10⁻² device) and the two implementations disagree by 1,11×10⁻⁴ — the comparison is 388× more sensitive than the effect it grades**, so this is not a null from an instrument that could not tell. | 2026-08-11 — **1,110 588×10⁻⁴** |
| **P7. The same in ΔE2000, BPC on** — `…/bpc-on-de2000-vs-lcms2` (and `…/de2000-baseline-bpc-off`) | **cross-check** | ΔE2000 max, `kL=kC=kH=1`, D50 | **5×10⁻²** | Row **C3**'s amplification chain re-run with §B's device envelope: below sRGB's breakpoint a device difference `δ` becomes `δ/12,92` of linear light and CIELAB's chromatic sensitivity on its own linear segment is `da*/dX = 4038`, so `Δa* ≈ 136 δ` against `ΔL* ≈ 69,9 δ`; with `S_C ≈ 1` and `S_L ≈ 1,75` the chromatic term dominates ~3×, giving ≈**2,4×10⁻²** at `δ = 1,16×10⁻⁴`. 5×10⁻² is ~2,1× that and **20× below §2's ⚠ anchor**, whose ⚠ it inherits. Derived at the **unclamped** shadow, which is conservative: BPC maps the darkest inputs below zero where both sides clamp to 0 and agree exactly. | 2026-08-11 — **1,262 374×10⁻²** (on) and **1,962 920×10⁻²** (off baseline) |
| **P8. ★ The direction — nothing may rise** — `…/direction-nothing-rises` | self-consistency | signed max, device 0..1 | **0,0 — exact** | **Pass 5's done-when clause 1, and it needs no tolerance at all.** `out − in = (a − 1)X + b = (Xd − Xs)/(Xi − Xs) · (Xi − X)`, whose second factor is `≥ 0` for any in-gamut PCS value, so the sign of the shift is the sign of `Xd − Xs` at **every** point; in S2 the destination black is zero and the source's is the A41 triple, so every channel must fall, and the destination's tone curves are monotone increasing so the sign survives into device space. The observed quantity is the largest **signed increase**; any positive value is a direction defect. | 2026-08-11 — **0,0** (largest fall 4,304×10⁻² device, 3,5159 ΔE2000) |
| **P9. lcms2 does NOT force here** — `…/lcms2-does-not-force-here` | oracle-reproducibility | abs-max, device | **∞ — REPORTED, NOT GRADED** | **Both sides are lcms2**: `-b` against no `-b`. Non-zero, and that is the point — the destination is **v2** sRGB and the forcing is keyed on the **destination** version (§3.4.4 row B8), so nothing is forced and the flag is the only thing turning BPC on. It is the control that makes S3's forcing row mean something. | 2026-08-11 — **4,290 863×10⁻²** |
| **P10. ★ A41 priced in a pipeline** — `pass5/S2/a41-cost-measured-in-a-pipeline` | **derived-expectation** | ΔE2000 max | **∞ — REPORTED, NOT GRADED** | The same map rebuilt with **ICC.1 Table 16's printed decimals** instead of the triple lcms2 and ICC's own `iccDEV` both use, over this grid's PCS values. **Both corpus figures corroborated by an independent route** (Rust, a fixture's stored bytes, a different pipeline, against the corpus's two Python passes): **ΔL* 0,005 364** vs 0,005 3, **ΔE76 0,037 416** vs 0,037 437 — agreeing to 2×10⁻⁵. The **ΔE2000 is new at 0,050 201**, and it carries a warning the corpus's framing does not: that is the **same order as §B's entire agreement budget**, so on a *float* path the choice of digits is not negligible against the measurement noise. Not a contradiction of "invisible at 16-bit" (both triples still encode to the same codes) but its complement — and the reason iccce follows the implementations and `bpc.rs` says so. | 2026-08-11 — **0,050 201 ΔE2000 / 0,037 416 ΔE76 / 0,005 364 ΔL*** |

#### 3.5.4 §C — S3, the `0 → PB` direction, and the policy (sRGB → fixture, perceptual)

213 RGB points.

| Comparison | Kind | Metric | Tolerance | Justification | Measured |
|---|---|---|---|---|---|
| **P11. lcms2 forces BPC unasked** — `pass5/S3/srgb-to-fixture/perceptual/lcms2-forces-bpc-unasked` | oracle-reproducibility | abs-max, device | **0,0 — exact** | **Both sides are lcms2**, into a v4 destination at perceptual. Exactly zero because the flag is **overwritten before it is read** (`_cmsLinkProfiles` sets `BPC[i] = TRUE` unconditionally here), so asking and not asking must produce the same bytes. Corpus **M2** re-measured in the direction row B8 showed is the one that matters. | 2026-08-11 — **0,0** |
| **P12. BPC on, iccce vs lcms2** — `…/bpc-on-device-vs-lcms2` and `…/bpc-on-vs-lcms2-unasked` | **cross-check** | abs-max, device 0..1 | **1×10⁻⁴** | **Row B5's envelope, deliberately unchanged.** B5's derivation: the interpolation-method term is zero by construction (the fixture's CLUTs are affine, row B0), leaving lcms2's `u16` CLUT-boundary quantisation of **1,5×10⁻⁵**, unamplified because this table has unit slope in device per normalised PCS unit; × the BPC gain 1,0035 + the `f32` term 7,8×10⁻⁷ = **1,6×10⁻⁵**. 1×10⁻⁴ is ~6,4×, covering fixed-point interpolation and `transicc`'s 10⁻⁶ print floor. **Kept at B5's constant on purpose: a tolerance that moved when the only change is a linear stage would be a tolerance tracking the observation.** Run against **both** lcms2 arms, so the reader need not trust that the forcing row makes them identical. **Sensitivity 682×.** | 2026-08-11 — **4,600×10⁻⁵** against each arm |
| **P13. The direction — no `K` may rise** — `…/direction-K-never-rises` | self-consistency | signed max, device | **0,0 — exact** | P8's argument with `Xd − Xs > 0`: every PCS value **rises**, and this destination's `K` falls as `L*` rises, so the *device* test reads "nothing may rise" again. **That coincidence of wording is why P14 exists** — a sign test that reads the same in both directions cannot by itself show the two directions differ. ★ The first draft of this row graded the negated **minimum** and failed at 3,1372×10⁻², asserting "nothing may fall" on a scenario whose whole point is that `K` falls. **The failure was the row, not the engine**; recorded rather than quietly rewritten. | 2026-08-11 — **0,0** |
| **P14. ★★ The lift at device black vs a closed form** — `…/lift-at-black-matches-closed-form` | **derived-expectation** | abs-max, device 0..1 | **5×10⁻⁶** | **The one place Pass 5 grades an end-to-end transform against something other than lcms2.** `RGB (0,0,0) → XYZ (0,0,0)` exactly (every TRC is 0 at 0); BPC's **second constraint** — graded at 3,3×10⁻¹⁶ by P3, so not an assumption — sends that to the destination black exactly, i.e. to the A41 triple, whose `L*` is `(841/108)·116 × 0,003 473 1 = 3,137 238` on CIELAB's linear segment; the fixture's `mBA ` closed form (row B3, using the **stored** `u16` nodes, not an idealised ½) gives `K`. Predicted `K` **0,964 721 905** with BPC and **0,996 093 810** without, a lift of **0,031 371 905**. The bound is the observation's print floor and nothing else: 6 decimals per arm = ±5×10⁻⁷ each, ±10⁻⁶ on the difference; 5×10⁻⁶ is 5×. **It also refuses the wrong perceptual-black triple**, whose signature here is `ΔK ≈ 5,4×10⁻⁵` — **11× this bound** — so the row doubles as the A41 discriminator. | 2026-08-11 — **9,504 522×10⁻⁸**, *below* the print floor it was derived from |
| **P15. ★ lcms2 against the same closed form** — `…/lcms2-black-matches-closed-form` | **derived-expectation** | abs-max, device 0..1 | **1×10⁻⁴** | **The third reading**, and it is what stops the fixture and the derivation being wrong together — the standing weakness §3.4.4.1 attaches to every derived expectation. lcms2's own forced-BPC `K` at device black against a derivation it had no part in: **within one printed lsb**. Graded at B5's constant rather than P14's because lcms2 carries its own `u16` quantisation and `transicc`'s 4-decimal print floor. | 2026-08-11 — **9,046 508×10⁻⁷** |
| **P16. ★★ THE POLICY DIFFERENCE** — `…/POLICY-iccce-never-forces` and `pass5/S3/D11-fingerprint` | **cross-check** | abs-max, device / `L*` | **∞ — REPORTED, NOT GRADED** | iccce **without** `--bpc` against lcms2 **without** `-b`, same pair, same intent: **3,1373×10⁻² device = 3,137 348 `L*`**, lcms2 lighter. **Neither is a defect; the number IS the policy.** lcms2 forces BPC for a v4 destination at perceptual on the authority of a document nobody in this project has read — the claim is a source comment attributed to Adobe, and the one published BPC paper (Maria 2013) corroborates the *exclusion* set while being **silent on the enable policy** (`ICC_Spec` §7.1). iccce declines to force. **Grading this would mean picking a winner without a clause**, and the two available gradings — a ~3,2 `L*` tolerance chosen because it passed, or a permanent red line — were both rejected in writing, as with §3.4.2 and §3.4.4 rows B7/B8. **★ D11 answered:** 3,137 348 `L*` against the PRM black's 3,137 254 and the A41 triple's 3,137 238 — a match to 1,1×10⁻⁴, and the sign matches **lcms2's M2 route** (force for a v4 *destination*, mapping zero **up** to the PRM black), **not iccDEV's** (apply 6.3.4.3 to the v2 side at link time, inverting on output) — which the two directions distinguish, because in S2 iccDEV would map the PRM black **down** on the v2 output side and lcms2 does nothing unless asked, which is what S2 observed. Settled by `AdobeBPC.pdf` / ICC WP40 / ISO 18619. | 2026-08-11 — **3,137 300×10⁻² device / 3,137 348 `L*`** |

#### 3.5.5 §D/§E — the null controls, the trap, and the refusals

| Comparison | Kind | Metric | Tolerance | Justification | Measured |
|---|---|---|---|---|---|
| **P17. S1, both implementations' BPC is a no-op** — `pass5/S1/srgb-to-adobergb/media-relative/{lcms2,iccce}-bpc-is-a-no-op` | oracle-reproducibility / self-consistency | abs-max, device | **0,0 — exact** | **NULL BY CONSTRUCTION, and recorded as such.** Both files are v2 matrix/TRC with `trc(0) = 0`, so both implementations estimate `XYZ (0,0,0)` on both sides; lcms2's `BlackPointIn != BlackPointOut` test fails and no stage is inserted, and `BpcScale` with equal blacks is the exact identity (P4). **INCONCLUSIVE as evidence that the two BPCs agree** — an arm-comparison that comes back null may be null by construction, and this one is (the lesson README §12 records). What it *does* establish is that lcms2's darkest-colorant estimate on these files really is zero, which is a **premise** of S2's and S3's predictions. | 2026-08-11 — **0,0** and **0,0** |
| **P18. ★ S4, forced BPC costs exactly nothing** — `pass5/S4/srgb-to-v4-matrix/perceptual/forced-bpc-costs-nothing` and `…/iccce-agrees-it-is-a-no-op` | oracle-reproducibility / self-consistency | abs-max, device | **0,0 — exact** | **Corpus trap T5, measured.** The configuration M2 says forces BPC — v4 destination, perceptual — and it costs nothing, because `cmsDetectBlackPoint`'s guard 3 takes the **matrix-shaper escape** to `BlackPointAsDarkerColorant` at *media-relative* and returns `XYZ (0,0,0)`, equal to the source's. Anyone expecting M2's ≈3,15 `L*` on *every* v4 perceptual profile would read this correct null as an anomaly. **iccce reaches the same no-op by a different route** — its subset sends a matrix/TRC side to device black regardless of version or intent, so it never consults the A41 constant here — and a shared answer from different reasoning is stronger than one from shared reasoning. | 2026-08-11 — **0,0** and **0,0** |
| **P19. S5, iccce refuses outside its estimation subset** — `pass5/S5/srgb-to-swop/media-relative/refuses-outside-the-subset` | self-consistency | 0/1 | **0,0 — exact** | **A refusal is graded, not merely reported**, because refusing where it cannot estimate is a property iccce claims: a build that quietly substituted a zero black for an unestimable one would produce plausible colour and pass every other row here. A v2 CMYK `prtr` destination at media-relative is exactly where lcms2 runs the least-squares quadratic fit whose mathematics Maria 2013 forwards to the ToS-barred `AdobeBPC.pdf` (**A42**) and whose six thresholds are unattributed even in lcms2's own source. **lcms2 answers there; iccce does not; so no comparison exists for that case and Pass 5 claims none — a COVERAGE GAP, not a bug.** | 2026-08-11 — **refused as required** |
| **P20. S6, iccce refuses BPC at ICC-absolute** — `pass5/S6/absolute-intent/refuses-bpc` | self-consistency | 0/1 | **0,0 — exact** | The one exclusion Pass 5 can cite a published source for — Maria 2013 §4.1, **verbatim**: *"absolute colorimetric intent (either the new ICC-absolute or the old V2-absolute) does not apply"* — and lcms2 enforces the same exclusion twice over. BPC presupposes both media whites already at D50, which is what media-relative means and what ICC-absolute undoes: **the exclusion and the D50 anchoring are the same fact.** The only Pass 5 row besides §A that runs on a machine with no colour directory, both its profiles being committed fixtures. The needle is the **exact wording**, not "refused" — a loose needle would let this row pass on an estimation-subset refusal. | 2026-08-11 — **refused as required** |

#### 3.5.6 What Pass 5 did NOT measure

- **Any black-point ESTIMATOR.** Every scenario in reach has both
  implementations arriving at the same black (§3.5.1), so no row discriminates
  iccce's named subset from lcms2's four methods. **lcms2's methods 3 and 4 —
  the ink round trip and the quadratic curve fit — are untested against
  anything**, because iccce does not implement them and refuses instead. Closing
  this needs a synthetic v4 **LUT** fixture with a **non-zero** device black;
  every profile in reach has `trc(0) = 0`.
- **The saturation intent.** lcms2 forces BPC there too; iccce's subset admits
  only perceptual for a LUT side, so that arm has no iccce half.
- **Any real v4 LUT profile.** §3.4.4.5's finding stands — a 40-profile sweep of
  this machine found **zero** `mAB `/`mBA ` tags. S2 and S3 are about **one
  synthetic fixture**.
- **The gray side of iccce's subset.** It is implemented and no scenario
  exercises it, because every gray profile in reach would be another null.
- **lcms2's `0,002` empty-layer threshold *observed*.** It is solved for from
  lcms2's own inequality, not triggered; no pair in reach has blacks close
  enough.
- **Whether forcing is conformant.** Unsettled, and it needs `AdobeBPC.pdf` /
  ICC WP40 / ISO 18619 (`ICC_Spec` §11's operator download list).
- **Any published value.** There is none for a BPC result, for the same reason
  there is none for perceptual (**A27**): no obtained normative text to grade
  against.

### 3.5.7 ★★ Pass 5b — the ESTIMATORS, and a pre-registered prediction measured

**Run 2026-08-12 by `icc-conformance`.** Apparatus:
`tools/difftest/src/pass5b.rs`. Full record: `tools/difftest/README.md` **§17**.

> ⚠ **PARTIALLY SUPERSEDED THE SAME DAY BY §3.5.8.** Pass 5b could not read
> lcms2's black point and **recovered** it through `A2B1 ∘ B2A1`; it said so,
> graded the recovery's error at 95 % of the effect (row Q1, 0,948), and
> qualified its conclusions accordingly. Pass 5c reimplemented lcms2's
> estimator from source and found that **98,3 % of row Q2's 0,858 17 ΔE76 was
> that recovery**, and that lcms2's black on this fixture is **neutral** —
> so row Q3's "claim 1 CONFIRMED" is **withdrawn**. Rows Q1, Q4, Q6 and Q7 are
> unaffected; Q5's *"NOT ESTABLISHED"* was the correct call and is now settled.
> Q8 has been **inverted** — see §4. Read §3.5.8 before quoting any number
> below.

§3.5.6's first bullet — *"Any black-point ESTIMATOR … no row discriminates
iccce's named subset from lcms2's four methods"* — is the gap this section
closes, and it closes it **partially and on a real profile**, not on the
synthetic fixture §16.8 item 4 asked for.

#### 3.5.7.1 ★★ THE FINDING THAT COMES BEFORE ANY NUMBER — the ISO estimator has no caller

`crates/iccce-cmm/src/bpc.rs` implements ISO/CD 18619 4.2.5 in full and is
unit tested. **Nothing outside its own test module calls it.**
`Chain::estimate_dst_black` in `transform.rs` still carries the pre-ISO subset —
a LUT destination is accepted only when the profile is v4 *and* the intent is
perceptual, where it returns the fixed A41 triple, and is otherwise
`BpcEstimationUnsupported`. So **a v2 CMYK LUT destination at media-relative —
the exact case ISO 4.2.5 exists for, and the exact case row P19 recorded as a
coverage gap — is still refused by the shipped binary.** Row Q5 grades that
refusal rather than leaving it implied.

**Consequence for what every row below is allowed to claim:** rows Q1–Q4 and Q6
grade `iccce_cmm::bpc`'s **library function**, driven in process. They do **not**
grade `iccce transform --bpc`, which cannot reach this path. That is the same
distinction §3.4.4's PCS-side row draws, and it is on every record.

#### 3.5.7.2 The two estimators, read at the pin before anything ran

| | ISO/CD 18619 4.2.5 (iccce) | lcms2 2.19.1 `cmssamp.c` |
|---|---|---|
| ramp chroma | **ramps to zero**, `(t·100, ka(1−t), kb(1−t))` | **held constant** at `clamp(±50, InitialLab.a/.b)`, L455–500 |
| monotonic pass, sample count, **root of the quadratic** | identical | identical — so iccce's "root not vertex" correction of *Adobe* is **not** a divergence from lcms2 |
| returned chroma | **`(L, 0, 0)`, neutral** (4.2.3) | **`Lab.a = InitialLab.a; Lab.b = InitialLab.b`**, L592 — retained |

★ A difference nobody predicted, from the same two pages: lcms2 clamps the
chroma to ±50 *for the ramp* and returns the **unclamped** `InitialLab.a/.b`.
Two different numbers inside one function. **READ, not RUN** — no profile in
reach has a darkest colorant with |a\*| or |b\*| above 50.

#### 3.5.7.3 The rows

Fixture: `USWebCoatedSWOP.icc` (v2.1, `prtr`, CMYK, `Lab ` PCS) as destination,
system sRGB as source, **media-relative**, 21-step neutral ramp.
Measured black points: ISO **`L* 16,4898`, neutral**; lcms2 **`L* 17,2150`,
`a* 0,3472`, `b* 0,3001`**, chroma **0,4589**. Divergence **0,858 17 ΔE76**.

| Comparison | Kind | Metric | Tolerance | Justification | Measured |
|---|---|---|---|---|---|
| **Q1. ★ The apparatus — the recovery error must be smaller than the effect** — `pass5b/apparatus/recovery-error-is-smaller-than-the-effect` | self-consistency | ratio | **1,0** | `transicc` cannot print a black point, so lcms2's is **recovered** from its own output: with BPC on and a source black of `XYZ(0,0,0)`, the second constraint sends PCS zero to the destination black exactly (row P3, 3,33×10⁻¹⁶), so lcms2's CMYK there is `B2A1(black)` and `A2B1` carries it back. `A2B1 ∘ B2A1` is not the identity, so that has an error. **The bound is a ratio with no free parameter: an error bar is readable exactly when it is smaller than what it bounds.** At or above 1 every row below sits inside its own uncertainty and the section is *void*, not merely worse. **★ THIS ROW FAILED TWICE BEFORE IT PASSED** — see §4. | 2026-08-12 — **0,948 24**. Local residual **0,782 5** at the ISO black and **0,813 7** at the recovered lcms2 black. **★ It passes by 5 %.** That is stated here rather than buried: the recovery error is 95 % of the effect, so Pass 5b's black-point comparison is **marginal**, and which of its conclusions survive is decided row by row below rather than by the fact that this row is green. |
| **Q2. The two black points, in Lab** — `pass5b/estimators/black-points-in-lab` | **cross-check** | ΔE76 | **∞ — REPORTED, NOT GRADED** | **★ The first row in this suite that discriminates the two ESTIMATORS.** §3.5.1's warning — "no row here may be quoted as if they did" — was true of Pass 5 and is no longer true of Pass 5b. | 2026-08-12 — **0,858 17 ΔE76**; `ΔL* −0,7252`, `Δa* −0,3472`, `Δb* −0,3001`. |
| **Q3. ★ Prediction claim 1 of 4 — the MECHANISM — CONFIRMED** — `pass5b/PREDICTION/1-mechanism-CONFIRMED-chroma-component` | **cross-check** | abs residual | **1×10⁻¹²** | The chroma component of the divergence equals the detected black's chroma. **Labelled STRUCTURAL on iccce's side and that is the point of the row**: ISO 4.2.3 returns a neutral black, so `Δa* = −a*_lcms2` identically. **What it grades is that clause 4.2.3 is implemented** — a build that had quietly kept the chroma fails it — **not** that the prediction's substance was right. | 2026-08-12 — **0,0** exactly |
| **Q4. ★★ Prediction claim 2 of 4 — the MAGNITUDE — FALSIFIED** — `pass5b/PREDICTION/2-magnitude-FALSIFIED` | **cross-check** | ΔE76 | **∞ — REPORTED** | The prediction's band was **2–6 ΔE76**. The detected destination black's chroma is **0,458 92** — an order of magnitude below it. **The band assumed a chromatic printer black and this profile has not got one**: SWOP's darkest colorant is `Lab(11,77 · 0,766 · 0,328)`, i.e. **0,834 off neutral**, so *no* estimator reading this file could have produced a number in the predicted band. **★ ROBUST TO Q1's ERROR BAR**: even if the entire 0,813 7 recovery error fell in chroma, `0,459 + 0,814 = 1,273` is still below the band's lower edge. | 2026-08-12 — **0,458 92 ΔE76** |
| **Q5. Prediction claim 3 of 4 — the SHAPE — NOT ESTABLISHED** — `pass5b/PREDICTION/3-shape-NOT-ESTABLISHED-lightness-term-unattributed` | **cross-check** | ratio | **∞ — REPORTED** | The prediction says the divergence **is** the chroma, which needs the two estimators to agree on `L*`. The measured `L*` term is **1,58×** the chroma term — but **this is not a falsification and the row does not claim one.** (a) The `L*` term (0,725) is *inside* Q1's error bar (0,814). (b) The obvious mechanism — lcms2 holds the ramp's chroma constant while ISO ramps it to zero — was measured **oracle-free**, by running the same ISO function on the *unneutralised* darkest colorant: it moves the fitted root from **16,4898 to 16,5441**, i.e. **0,054 3 `L*`, 13× too small**. So the `L*` term is **unattributed and most likely apparatus**. Recording that is worth more than a third "FALSIFIED" headline the evidence does not support. | 2026-08-12 — **1,580 11×** (`L*` 0,725 2 vs chroma 0,458 9); oracle-free ramp sensitivity **0,054 3 `L*`** |
| **Q6. ★ Prediction claim 4 of 4 — the DECAY — CONFIRMED** — `pass5b/PREDICTION/4-decay-to-white-CONFIRMED` | **cross-check** | ΔE76 at device white | **5×10⁻²** | BPC is anchored on `D50` **exactly** at the white end (row P3, 3,33×10⁻¹⁶), so a black-point disagreement **must** vanish there; had it not, the divergence would not be in the black point and every other row here would be attributing it to the wrong thing. **Deliberately the same constant as §3.4.4 row A4**, because the quantity at white *is* that row and a different number here would be a tolerance tracking an observation. | 2026-08-12 — **0,0**. Ramp ΔE76 at `k = 0 / 0,25 / 0,5 / 0,75 / 1`: **0,087 8 · 0,053 1 · 0,013 5 · 0,008 8 · 0,000 0** — monotone. |
| **Q7. What actually survives end to end** — `pass5b/estimators/end-to-end-divergence-at-input-black` | **cross-check** | ΔE76 / ΔE2000 / device | **∞ — REPORTED** | **The number an integrator cares about, and it is much smaller than the black-point divergence.** Of 0,858 ΔE76 between the two estimated blacks, **90 % does not survive** the trip through the destination's `B2A` and back: both blacks are at or below this profile's **gamut floor** (`A2B1(B2A1(Lab(0,0,0)))` returns `L* 16,4898`, which is the ISO estimate to four decimals), so the destination clips them toward the same ink combination. **A disagreement about the black point is not the same size as a disagreement about the output**, and on a CMYK destination the gamut boundary is the thing that decides which. | 2026-08-12 — **8,785×10⁻² ΔE76 / 5,92×10⁻² ΔE2000 / 2,464×10⁻³ device** |
| **Q8. The shipped chain cannot reach the ISO estimator** — `pass5b/coverage/shipped-chain-cannot-reach-the-iso-estimator` | self-consistency | 0/1 | **0,0 — exact** | **Graded, not reported**, for row P19's reason: a build that quietly substituted a zero black for an unestimable one would produce plausible colour and pass every other row in this suite. The needle is the **exact `Display` text**, not the variant name — row P19/P20's lesson, not repeated. | 2026-08-12 — **refused as required**: `--bpc refused: black point not estimable within iccce's named subset (A42); refused, not guessed` |

#### 3.5.7.4 What Pass 5b did NOT measure, and it is not small

- **The shipped surface.** Every row above drives a library function. Wiring
  `estimate_lut_destination_black` into `Chain::estimate_dst_black` is
  engineering, not conformance, and until it happens `iccce transform --bpc`
  has exactly the coverage Pass 5 recorded.
- ~~**lcms2's estimator reimplemented.**~~ **DONE 2026-08-12 — see §3.5.8**,
  and it overturned this section's own claim 1. lcms2's black was *recovered*
  through `A2B1 ∘ B2A1`; it is now *reproduced* from `cmssamp.c` at the pin.
  **98,3 % of row Q2's 0,858 17 ΔE76 turns out to be that recovery.** The true
  divergence on this fixture is **8,167×10⁻² ΔE76, entirely `L*`**, and
  lcms2's black here is **neutral**, because a CMYK output profile at relative
  colorimetric reaches `BlackPointUsingPerceptualBlack`, which forces
  `a* = b* = 0`. Row Q3's "CONFIRMED" verdict on the prediction's mechanism is
  **WITHDRAWN**; what Q3 still grades — that ISO 4.2.3 is implemented — is
  unaffected.
- **lcms2's methods 3 and 4 separately.** SWOP at media-relative exercises
  method 4 (the quadratic fit). The **ink round trip** is still untested
  against anything.
- ~~**The v4 perceptual arm.**~~ **REFRAMED 2026-08-12.** The fixture exists —
  `fixtures/synthetic/v4-rgb-mab-chromatic-black.icc`, device black
  `Lab(20 · 4 · −3)` — but the arm it was asked for **cannot be
  discriminated by any fixture**: at perceptual and saturation on a v4 profile
  both implementations return the fixed A41 constant *without reading the
  profile* (`cmssamp.c` L432–446). What the fixture does instead is
  discriminate the **media-relative** arm, on a destination that is **not** an
  ink space and therefore takes lcms2's other branch. See §3.5.8.
- **Any profile but one.** Q4's falsification is about a band that assumed a
  chromatic black; a profile that *has* one would be a genuinely different
  test, and none is in reach.

### 3.5.8 ★★ Pass 5c — lcms2's estimator REIMPLEMENTED, and §3.5.7's claims settled

**Run 2026-08-12 by `icc-conformance`.** Apparatus:
`tools/difftest/src/pass5c.rs`. Full record: `tools/difftest/README.md` **§19**.
Oracle pin `21c582a` (lcms2 2.19.1), iccce at commit `95c04c1`.

§3.5.7.4 named this and called it *"the single highest-value item left in
Pass 5's family"*:

> **lcms2's estimator reimplemented.** lcms2's black is *recovered*, not
> reproduced. A harness reimplementation of `cmsDetectDestinationBlackPoint`
> … would remove Q1's error bar entirely and turn Q5 from *unattributed* into
> a finding either way.

It is now built, from `src/cmssamp.c` at the pin, and it runs on **two** arms.
**Kind: `impl_crosscheck`, provenance source-read** — the same standing as
Pass 4b §C's `cmsReverseToneCurveEx` model. No lcms2 binary is executed to
produce a black point; `transicc` appears only in §B, where it validates the
reimplementation end to end.

#### 3.5.8.1 ★★★ THE FINDING — lcms2 has TWO black-point estimators at media-relative, and which one runs is decided by the DESTINATION'S DEVICE CLASS AND COLOUR SPACE

`cmsDetectDestinationBlackPoint` takes its `InitialLab` from
`cmsDetectBlackPoint`, and that function branches **before** it reaches the
code §3.5.7.2's table described (`cmssamp.c` L370–374):

```c
// If output profile, discount ink-limiting and that's all
if (Intent == INTENT_RELATIVE_COLORIMETRIC &&
    (cmsGetDeviceClass(hProfile) == cmsSigOutputClass) &&
    (isInkColorspace(cmsGetColorSpace(hProfile))))
    return BlackPointUsingPerceptualBlack(BlackPoint, hProfile);
...
return BlackPointAsDarkerColorant(hProfile, Intent, BlackPoint, dwFlags);
```

The two branches disagree about **exactly the thing the pre-registered
prediction was about**:

| branch | taken when | what it does to the chroma |
|---|---|---|
| `BlackPointUsingPerceptualBlack` (L146+) | output class **and** an ink space | round trips `Lab(0,0,0)` through the **perceptual** `B2A` and the relative `A2B`, clips `L*` to 50, and **forces `a* = b* = 0`** (L174) |
| `BlackPointAsDarkerColorant` (L62+) | anything else | transforms the space's darkest colorant through `A2B`, clips `L*` to `[0,50]`, and **keeps `a*` and `b*`** |

`cmsDetectDestinationBlackPoint` then returns `Lab.a = InitialLab.a;
Lab.b = InitialLab.b` (L590–591) — so the branch **is** the returned chroma.

**Consequences, in order of how much they change what is on file:**

1. **§3.5.7's row Q3 ("claim 1 — the MECHANISM — CONFIRMED") is WITHDRAWN for
   the SWOP arm.** `USWebCoatedSWOP.icc` is `prtr` + CMYK, so lcms2 takes the
   *first* branch and returns a **neutral** black. The `a* 0,347 / b* 0,300`
   Q2 recorded was chroma the `A2B1 ∘ B2A1` **recovery** introduced. Q1's
   error bar existed to warn about exactly this and it was reported as
   marginal (0,948) rather than quoted as green — the discipline worked.
2. **Claim 3 (the SHAPE) is no longer "NOT ESTABLISHED"; it is FALSIFIED on
   one arm and CONFIRMED on the other**, and the two are different profiles.
3. **§3.5.7.2's row "ramp chroma held constant … `clamp(±50, InitialLab.a/.b)`"
   was true as written and irrelevant as applied**: on SWOP that clamp acts on
   a zero.
4. **Neither implementation fits a quadratic on either fixture.** Both take the
   mid-range straightness short-circuit (`cmssamp.c` L521–545;
   `bpc.rs`'s 4.2.5.4 gate). §17.3's sentence *"this configuration is precisely
   lcms2's method-4 (quadratic-fit) territory"* is **wrong**, and every Pass 5b
   statement about the shadow window or the root describes code that did not
   run. What the two implementations actually disagree about there is **what
   the short-circuit RETURNS**: lcms2 returns `InitialLab` (L536), a value from
   a *different* round trip; ISO returns `outRamp[first]`.

#### 3.5.8.2 The two arms, and neither is the answer alone

| | **arm `swop`** | **arm `synthetic`** |
|---|---|---|
| destination | `USWebCoatedSWOP.icc` — v2.1 `prtr` **CMYK** | `fixtures/synthetic/v4-rgb-mab-chromatic-black.icc` — v4.4 `prtr` **RGB**, `mAB `/`mBA `, 9³ |
| lcms2 branch | `BlackPointUsingPerceptualBlack` | `BlackPointAsDarkerColorant` |
| ISO 4.2.5 black | `L* 16,489 806`, neutral | `L* 20,000 000`, neutral |
| lcms2 black (reimplemented) | `L* 16,571 474`, **neutral** | `Lab(20,000 000 · 4,000 000 · −3,000 000)`, **chromatic** |
| divergence | **8,166 8×10⁻² ΔE76 — 100 % `L*`** | **5,000 000 ΔE76 — 100 % chroma, `ΔL*` exactly 0** |
| claim 1 (mechanism) | **FALSIFIED** | **CONFIRMED** |
| claim 3 (shape) | **FALSIFIED** | **CONFIRMED** |

**A session that ran only one arm would have filed a confident wrong headline
either way.** The variable that decides the verdict is not the black, the
intent, the profile version or the tag type — it is two header fields.

⚠ **The synthetic arm's 5,000 ΔE76 is evidence for the MECHANISM and for
nothing else.** That chroma is what this project *authored* into the fixture
(`recipes.rs`, `SYNTH_BLACK_A/B`). It happens to sit inside the corpus's
pre-registered 2–6 ΔE76 band; **that is not a confirmation of claim 2**, whose
falsification stands on the SWOP arm where the profile was not ours to choose.

#### 3.5.8.3 The rows

| Comparison | Kind | Metric | Tolerance | Justification | Measured |
|---|---|---|---|---|---|
| **T1. ★ The apparatus — the error bar must be smaller than the effect** — `pass5c/{arm}/apparatus/error-bar-is-smaller-than-the-effect` | self-consistency | ratio | **1,0** | **Deliberately §3.5.7 row Q1's constant and Q1's derivation, unchanged**: *an error bar is readable exactly when it is smaller than the effect it bounds.* What changed is the error bar. Q1 bounded a **recovered** black by the `A2B1 ∘ B2A1` round trip; T1 bounds a **reimplemented** black by its own device residual against `transicc`, converted back to `L*` through a sensitivity `d(device)/d(L*)` measured on the same `B2A1` table — so the bound is in the unit the claim is made in. | 2026-08-12 — **swop 3,043 1×10⁻¹** (residual 4,224 9×10⁻⁴ ÷ sensitivity 1,700 0×10⁻² = `L*` bound 2,485×10⁻², effect 8,167×10⁻²); **synthetic 2,195 3×10⁻⁴**. **Q1 scored 0,948 on the same fixture — the bar is 33× tighter on the swop arm and 4 300× tighter on the synthetic one.** · **2026-08-12 (later, re-measured on the corrected 4.2.5.4 code at `cc03f3d`) — swop 5,178 5×10⁻³.** ★ **The bar did not move and the row got 59× greener.** Residual, sensitivity and the resulting `L*` bound of 2,485×10⁻² are all unchanged; the **effect** grew from 8,167×10⁻² to 4,799 109. *A ratio row can improve because its denominator improved or because its numerator got worse, and only one of those is good news* — here it is the second. See **§3.5.8.6**. |
| **T2. ★★★ THE FINDING — the divergence's chroma follows lcms2's BRANCH** — `pass5c/{arm}/FINDING/divergence-chroma-follows-lcms2-BRANCH` | **cross-check** | abs residual | **0,0 — exact** | Graded quantity: `chroma of the divergence − what the selected branch requires`. **Exact, not an epsilon**: both sides assign literals — ISO 4.2.3 returns a neutral black, lcms2 returns `InitialLab.a/.b` verbatim — so the residual is `0` or it is a branch error. Taking the *other* branch moves it by the darkest colorant's whole chroma (0,834 on SWOP, 5,0 on the synthetic fixture), which no rounding argument reaches. **STRUCTURAL on the reimplementation's side and labelled so; T4/T5 are what make it evidence.** | 2026-08-12 — **0,0 exactly on both arms.** swop: chroma 0, branch requires 0. synthetic: chroma 5,000 000, branch requires 5,000 000. |
| **T3. ★ Neither implementation fits a quadratic here** — `pass5c/{arm}/FINDING/neither-implementation-fits-a-quadratic-here` | **cross-check** | 0/1 | **0,0 — exact** | **Graded rather than reported because §3.5.7 asserted the opposite.** Both estimators take the mid-range straightness short-circuit on both fixtures, so no shadow window is collected and no root is taken. A build in which either side stopped short-circuiting would change what this whole section is about, which makes the branch a precondition and not a footnote. | 2026-08-12 — **0,0 on both arms**; `nearlyStraight = true`, shadow points **0**. The divergence is therefore entirely in **what the short-circuit returns**: lcms2 `InitialLab` (L536), ISO `outRamp[first]`. · **2026-08-12 (later, corrected code at `cc03f3d`) — 0,0 on both arms, unchanged; the branch selection is not what moved.** ★ **But the sentence beside it is:** since commit `fd34a44`, ISO 4.2.5.4 returns **its own `InitialLab`** too (`outRamp[first]` was never a black-point candidate in any branch of 4.2.5). **Both sides now return a quantity their own document calls `InitialLab`, and the entire divergence is that the two documents mean different things by that name** — ISO's is 4.2.2.2's darkest *device vertex* neutralised, lcms2's is the *perceptual black* round trip. §3.5.8.6. |
| **T4. ★★ The reimplementation beats the rival candidate** — `pass5c/{arm}/validation/reimplementation-beats-the-rival-candidate` | **cross-check** | ratio, device | **1,0** | BPC's second constraint sends the source black **exactly** to the destination black (§3.5 row P3, 3,33×10⁻¹⁶) and this source's black is `XYZ(0,0,0)`, so the device values an implementation emits at input black **are** `B2A1(its own detected black)` and nothing else. **That is why a black point can be validated in device units with no round trip anywhere in the comparison.** The row divides the residual under the *lcms2* hypothesis by the residual under the *ISO* hypothesis: without it, a small absolute residual would be evidence of nothing, because on the swop arm the two candidates are only 0,082 `L*` apart. No free parameter — below 1 the lcms2 model is the better explanation of lcms2's own output. | 2026-08-12 — **swop 1,714 7×10⁻¹** (4,224 9×10⁻⁴ against 2,463 9×10⁻³); **synthetic 1,561 2×10⁻⁴** (8,938 3×10⁻⁶ against 5,725 1×10⁻²). · **2026-08-12 (later, corrected code at `cc03f3d`) — swop 4,258 5×10⁻²** (4,224 9×10⁻⁴ against **9,921 1×10⁻³**); **synthetic 1,561 2×10⁻⁴, unchanged.** The lcms2-model residual is identical to six figures; the **ISO-model** residual grew **4,03×** because the ISO candidate moved 4,72 `L*` away. The row is 4× sharper and **nothing about the reimplementation improved** — its whole gain is the rival getting worse. ★ **The justification's clause "the two candidates are only 0,082 `L*` apart" is retired**: it is 4,799 `L*` now, and the apparatus prints the separation rather than quoting it (see §3.5.8.6 on why the literal was there at all). |
| **T5. The absolute device residual** — `pass5c/{arm}/validation/device-residual-against-transicc` | **cross-check** | abs-max, device 0..1 | **∞ — REPORTED, NOT GRADED** | **Deliberately not graded against Pass 4b §A's 1,330×10⁻⁴ envelope for the same `B2A1` table**, because that number is a maximum over Pass 4b's own point set and this is one deep neutral shadow point outside it — §3.6 row R4's lesson about maxima over different populations, applied to a row that could have quietly borrowed a constant. What is left in the residual is the **pipeline** difference: lcms2 evaluates its tables through the 16-bit machinery, this harness in `f64`. | 2026-08-12 — **swop 4,224 9×10⁻⁴** · **synthetic 8,938 3×10⁻⁶** |
| **T6. The two black points** — `pass5c/{arm}/estimators/black-points-in-lab` | **cross-check** | ΔE76 | **∞ — REPORTED** | The headline number, and it **supersedes §3.5.7 row Q2's 0,858 17 ΔE76**, which was 95 % apparatus. | 2026-08-12 — **swop 8,166 8×10⁻²** (`ΔL*` 0,081 67, chroma 0) · **synthetic 5,000 000** (`ΔL*` 0, chroma 5,000 000) · ★★ **2026-08-12 (later, re-measured on the corrected 4.2.5.4 code at `cc03f3d`) — swop 4,799 109** (`ΔL*` 4,799 109, chroma **0**) · **synthetic 5,000 000, unchanged.** **THE PREDICTED COLLAPSE DID NOT HAPPEN: THE FIGURE GREW 58,8×.** ISO now `L* 11,772 365` (4.2.2.2's darkest vertex `CMYK(1,1,1,1)` → `Lab(11,7724 · 0,7656 · 0,3281)`, neutralised); lcms2 `L* 16,571 474`, unmoved. **§3.5.8.6 is the finding.** |
| **T7. ★ Pass 5b's recovery WAS the round trip** — `pass5c/{arm}/ATTRIBUTION/pass5b-recovery-was-the-round-trip` | self-consistency | ratio | **1,0 on the `swop` arm; ∞ — REPORTED elsewhere** | The attribution row: `BT(reimplemented black)` should land on the black §3.5.7 recovered, and this grades what is left over. The denominator is **this section's own `L*` bound** rather than a chosen constant — *an explanation accounts for an effect when what is left over is inside the uncertainty of the explanation* — and it is deliberately strict, since the true uncertainty on `BT(black)` also contains the two implementations' disagreement about the tables. **Graded on `swop` only, and the reason is a units mismatch rather than a convenience**: the numerator is a full ΔE76 in Lab and the denominator is an `L*`-only bound, which are commensurate only where the divergence is `L*`. On the synthetic arm the divergence is 100 % chroma, both quantities sit at ~1,1×10⁻³, and the resulting 1,06 is arithmetic about incommensurable units. §3.6 row R5's lesson, second occurrence. | 2026-08-12 — **swop 6,036 4×10⁻¹.** Pass 5b recovered `Lab(17,214 958 · 0,347 197 · 0,300 108)`; `BT(reimplemented)` = `Lab(17,199 985 · 0,346 780 · 0,299 265)`; **unexplained 1,500 2×10⁻² ΔE76 of the 8,582×10⁻¹ Pass 5b published — 98,3 % of Q2's number is now accounted for as apparatus.** |
| **T8. The shipped binary reaches the ISO estimator** — `pass5c/{arm}/shipped/binary-reaches-the-iso-estimator` | self-consistency | abs-max, device 0..1 | **1×10⁻⁶** | **SUPERSEDES §3.5.7 row Q8**, whose premise is gone: Q8 graded `iccce transform --bpc` *refusing* this case because `bpc::estimate_lut_destination_black` had no caller. Pass 5b found the missing caller, commit `c268261` wired it, and the binary converts. What is worth grading now is that the wired path reaches **the same black point the library function does** — a wiring that passed a differently-derived `InitialLab`, or the perceptual `BT` instead of the relative one, would still convert and still look plausible. The bound is the CLI's own print floor: device values are printed to **six** decimals. It cannot absorb a different black point, which moves this quantity by 2,46×10⁻³ on the swop arm and 5,73×10⁻² on the synthetic one. | 2026-08-12 — **swop 4,499 1×10⁻⁷** · **synthetic 4,277 9×10⁻⁷** |

#### 3.5.8.4 ★ The apparatus fault this section caught, and how

The synthetic arm's first run reported a device residual of **9,98×10⁻²** —
where the truth is 8,9×10⁻⁶ — and would have been filed as *"the
reimplementation does not reproduce lcms2 on this fixture"*.

`transicc` prints ink spaces as percentages (`0..100`) and **RGB and gray as
`0..255`**. Every oracle output in Pass 5/5b/5c had been divided by 100,
because until this section the only destination in reach was CMYK.

**It was caught because §B carries a second, independent hypothesis.** Both
the lcms2 candidate *and* the ISO candidate missed by roughly the same amount,
and T4's ratio — the row that exists to ask whether the experiment can
discriminate at all — was the thing that made that visible. **A residual that
is large under every hypothesis is an apparatus fault, not a finding**, and a
section with only one arm has no way to notice.

#### 3.5.8.5 What Pass 5c did NOT measure

- **Any intent but media-relative.** At perceptual and saturation on a v4
  profile **both implementations return the fixed A41 constant without reading
  the profile** (`cmssamp.c` L432–446; `Chain::estimate_dst_black`), so no
  fixture can discriminate them there. §3.5.7.4's "the v4 perceptual arm"
  bullet asked for an instrument that **cannot exist**; what the new fixture
  makes possible instead is *measuring how wrong the constant is* — its `L*` is
  ≈3,1 against this device's real black of `L* 20` — and that measurement is
  **owed, not made**.
- **lcms2's ink round trip (`BlackPointUsingPerceptualBlack`) as a value.**
  It is reimplemented and it feeds the swop arm's `InitialLab`, but nothing
  grades that intermediate on its own.
- **A profile whose darkest colorant has `|a*|` or `|b*|` above 50**, where
  lcms2's clamp/return asymmetry (§3.5.7.2's ★) would finally bite. Still
  READ, not RUN — the synthetic fixture's black is chroma 5,0 and could have
  been authored past 50, but a fixture built to trigger one branch of one
  clamp would be a fixture built to make a point.
- **Any source but sRGB, and any black-point-bearing tag.**
  `CMS_USE_PROFILE_BLACK_POINT_TAG` is off in the pinned build, so `bkpt` is
  never consulted by either arm; a profile carrying one is untested.

#### 3.5.8.6 ★★★ Re-measured on the corrected 4.2.5.4 code — the divergence did not collapse, it grew 58,8×

**Measured 2026-08-12** by `icc-conformance`, harness at `cc03f3d`,
`crates/iccce-cmm` carrying commit `fd34a44`'s correction to
`bpc::estimate_lut_destination_black`. Same apparatus, same two fixtures, same
pin (`21c582a`), nothing in `pass5c.rs`'s measurement path changed. **This was
requested as a confirmation and came back a refutation.**

##### What was expected, on the record, and what happened

`NUMERIC_CLAIMS.md` §3.24.4 wrote that **NC-142's `8,167×10⁻²` should now be
expected to COLLAPSE**, and was careful to add that nobody had re-measured it.
The re-measurement:

| | before `fd34a44` | after `fd34a44` |
|---|---|---|
| ISO/CD 18619 4.2.5 black (`iccce_cmm::bpc`) | `L* 16,489 806` | **`L* 11,772 365`** |
| lcms2's black (reimplemented from `cmssamp.c`) | `L* 16,571 474` | `L* 16,571 474` — **unmoved** |
| **divergence (T6)** | **8,166 8×10⁻² ΔE76** | **★ 4,799 109 ΔE76 — 58,8× LARGER** |

The `synthetic` arm is **unchanged at 5,000 000** and could never have moved:
there ISO's `InitialLab` and `outRamp[first]` are both `L* 20,000 000`, so the
defect is invisible on it. **The fixture this project authored cannot see the
defect; only the real profile can** — which is the strongest argument in this
document for keeping a vendor profile in the corpus alongside the synthetic
ones.

##### Why, in one sentence, and it is not a bug

**Both implementations take the mid-range straightness short-circuit, and
since `fd34a44` both return a quantity their own document calls
`InitialLab` — so the whole divergence is that the two documents mean
different things by that name.**

| | `InitialLab` is… | on `USWebCoatedSWOP` |
|---|---|---|
| ISO/CD 18619 4.2.2.2 → 4.2.3 | the darkest **device vertex** carried through `A2B1` and neutralised | `CMYK(1,1,1,1)` → `Lab(11,7724 · 0,7656 · 0,3281)` → `L* 11,772 365` |
| lcms2 `cmsDetectBlackPoint` (ink+output branch) | `BlackPointUsingPerceptualBlack` — `Lab(0,0,0)` through the **perceptual** `B2A` and the relative `A2B`, chroma forced to 0 | `L* 16,571 474` |

These are two different constructions of "the darkest colour this profile can
make", not two readings of one construction. **4,80 `L*` is the honest size of
the disagreement between ISO/CD 18619 and lcms2 on this profile**, and until
now the project had never seen it, because its own defect had been standing in
front of it.

##### ★★★ The methodological finding, which is the part worth keeping

**Agreement with the oracle was the symptom of our defect, and conforming to
the clause made the cross-check worse.**

- The non-conformant code returned `outRamp[first] = MinL = 16,489 806` — a
  quantity that appears nowhere in 4.2.5 as a black-point candidate. It landed
  **0,082 `L*`** from lcms2's answer.
- The conformant code returns `InitialLab = 11,772 365`. It lands **4,799 `L*`**
  from lcms2's answer.
- **The defect's own magnitude is `|16,489 806 − 11,772 365| = 4,717 441 L*`
  — 57,8× the divergence it was blamed for.** It was very nearly invisible in
  the cross-check that was supposed to be able to see it.

`NUMERIC_CLAIMS.md` **NC-164a** records *"the cost of the defect, measured
before it was found: 8,166 8×10⁻², 100 % of the two implementations'
divergence"*. That attribution was **right about the cause and wrong about the
consequence**, and the distinction is worth stating precisely rather than
softening: 100 % of the *observed gap* was produced by that branch, and
removing the branch did not remove the gap — it **revealed a gap 59× larger
that the defect had been masking**. *"This defect accounts for the whole of
the disagreement"* and *"fixing this defect will end the disagreement"* are
different claims, and only the first was measured. **The cost of the defect is
`4,717 441 L*` on this fixture** — measured after the fact, by differencing the
two code paths, and that number belongs beside NC-164a's.

★ **This is the cleanest available demonstration of `CLAUDE.md` rule 3.**
Agreement with lcms2 is evidence that two implementations read a standard the
same way. Here they did not read it the same way at all; one of them was not
reading it, and the coincidence that made them agree to 0,08 `L*` was worth
**less than nothing**, because it is exactly the observation that would have
retired the question.

##### What this does NOT establish

- **It does not make lcms2 wrong.** lcms2 does not implement ISO/CD 18619 and
  never claimed to; `cmsDetectBlackPoint` is its own algorithm and it is
  self-consistent. There is **no ground truth in this comparison at all** — no
  published black point exists for `USWebCoatedSWOP.icc`. Every number here is
  **cross-check**, and 18619 is a **committee draft** in this project's corpus.
- **It does not say which black point a user should want.** §B measures only
  that lcms2's own output at input black is predicted by lcms2's own candidate
  (residual `4,225×10⁻⁴` device) and not by ISO's (`9,921×10⁻³`, 23× worse).
  That is a statement about **whose output is being reproduced**, and it would
  read identically if ISO's were the better colour.
- **Coverage: two profiles, one intent (media-relative), one direction, one
  pin.** The `swop` arm is the only one that exercises the corrected branch at
  all.

##### The consequence for NA-009, which is now measurable

`NUMERIC_CLAIMS.md` **NA-009** has carried *"cost UNMEASURED"* through two
sessions, and the librarian's filing correctly reset it when NC-142's figure
turned out to be our own defect. **It is measured now, on the corrected code:**

> **Choosing ISO/CD 18619's black-point estimator over lcms2's costs
> `4,799 109 ΔE76` (100 % `L*`) on `USWebCoatedSWOP.icc` at media-relative,
> and `5,000 000 ΔE76` (100 % chroma) on the synthetic v4 RGB fixture. At the
> input black those carry to `9,921×10⁻³` and `5,725×10⁻²` of device range
> respectively** — that is, ~1 % of ink on the SWOP arm.

Two cautions on that number, both load-bearing: it is the cost **at the black
point**, not over a population of colours (BPC's effect tapers away from the
shadow end and nothing here measures the taper); and it is a cost **relative to
lcms2**, not relative to truth.

##### A second, smaller finding: three claim-bearing strings had gone stale

The re-measurement was slowed by the apparatus asserting things that had
stopped being true, in `source` and context strings **emitted into every
record**:

- `pass5c.rs` printed *"ISO returned exactly `outRamp[first]` (11,772 365
  against MinL 16,489 806)"* — **self-refuting inside one sentence**, since it
  printed two different numbers while asserting they were the same.
- `DISCRIMINATES`'s justification asserted *"the two candidates are only 0,082
  `L*` apart"*, typed as a literal on the morning it was true.
- `pass6.rs` asserted *"17 is the shipped default"* in the grid-cost records,
  a day after the default became 33.

All three are now **formatted from the values the apparatus already computes**
(`{:.6} L* apart`, `{DEFAULT_GRID}`) rather than spelled out beside them. The
rule this establishes, and it belongs in every future record: **a claim-bearing
number that the harness can compute must be interpolated at run time, never
written into the prose next to the code that computes it.** A stale comment
misleads a reader; a stale string in an emitted conformance record misleads the
evidence.

### 3.5.9 ★★ Pass 5c's third arm — the 4.2.5.4 clause, graded against an AUTHORED constant on a COMMITTED fixture

**Added 2026-08-12 by `icc-conformance`.** Apparatus:
`tools/difftest/src/pass5c.rs` §C; fixture recipe
`tools/gen-profiles/src/recipes.rs` → `v4-rgb-mab-floored-b2a`; the finding that
prompted it is `tools/gen-profiles/README.md` §4.1 (GP-002).

#### 3.5.9.1 What was actually wrong, measured before anything was built

The brief was that a 4.2.5.4 regression was invisible on a machine without the
Windows colour directory. **The measurement is worse than that and also
narrower**, and both halves matter:

- **Worse.** A full reversion of `fd34a44`, injected into `bpc.rs` in a detached
  worktree at the same HEAD, turned **no row of `tools/difftest` red on any
  machine.** The `swop` arm's numbers moved; none of its graded rows crossed a
  bound (`apparatus` `5,18×10⁻³` against 1, `validation` `4,26×10⁻²` against 1),
  because the row that carries the finding —
  `estimators/black-points-in-lab` — is `REPORTED`. "The arm is differential"
  and "the arm is load-bearing" are different properties.
- **Narrower.** `cargo test -p iccce-cmm` **does** fail on that reversion:
  `straight_midrange_short_circuits_at_relative_only` and
  `straight_midrange_carries_chromatic_initial_lab_whole`, verified by running
  them against the injected defect. The clause was defended *as a function*, on
  a synthetic closure, the whole time. What had **no** committed instrument was
  the clause exercised **through a parsed profile** — which is where a wiring
  defect between `Chain::estimate_dst_black` and the estimator would live, and
  where a unit test on a closure cannot reach.

#### 3.5.9.2 The fixture, and the rule it is the first application of

`fixtures/synthetic/v4-rgb-mab-floored-b2a.icc` — `LEGAL.md` §3 category (a),
18 656 bytes, `gen-profiles verify` reports **40 identical, 0 not identical**
(the 39 existing fixtures are unchanged to the byte).

It is the sibling `v4-rgb-mab-chromatic-black` with **one** structural
difference: its `B2A` floors `G` at `25/87,5` for *every* input, not only
out-of-gamut ones, which lifts the round-trip floor to `L* 37,5` while leaving
`A2B(0,0,0)` alone at `L* 12,5`. That asymmetry **is** the separation.

**The GP-002 generalisation — every conceptually distinct quantity gets a
distinct value** — is applied deliberately and for the first time:

| quantity | value | why this one |
|---|---|---|
| `InitialLab` (4.2.2.2 → 4.2.3) | `Lab(12,5 · 0 · 0)` | **not** the sibling's 20, so a figure quoted without its arm is obviously wrong rather than plausibly right |
| lcms2's chroma-retaining `InitialLab` | `Lab(12,5 · 6 · −8)`, chroma **10,0** | twice the sibling's 5,0 and a different `a*`/`b*` pair |
| `outRamp[first]` — the rival return value | `Lab(37,5 · 0 · 0)` | **25,0 `L*`** away: a quarter of the lightness range, four orders above any encoding argument |
| the returned `DestinationBlackPoint` | `= InitialLab` | 4.2.5.4 states an **identity**, and that identity is what is graded |

Pairwise separations `10,0 / 25,0 / 26,93` ΔE76 — three distinct, non-zero
distances, so no two of the four can be substituted without a row moving.

**What it still cannot separate, stated rather than hoped.** lcms2's black and
ISO's `InitialLab` share their `L*` **unavoidably on an RGB fixture**:
`BlackPointAsDarkerColorant` reads the same vertex through the same `A2B`, so
only the chroma can differ. Two instruments would separate them and neither
exists — an **inverse-polarity** fixture (ISO 4.2.2.2 NOTE 2: ISO searches,
lcms2 uses a fixed `_cmsEndPointsBySpace` constant, so they would return
opposite ends of the device range), and a fixture whose darkest vertex is
**lighter than `L* 95`**, which reaches lcms2's otherwise-unexercised
`if (Lab.L > 95) Lab.L = 0;` while ISO 4.2.3 clips to 50.

#### 3.5.9.3 The two new tolerances, derived

| row | bound | derivation |
|---|---|---|
| `pass5c/<arm>/CLAUSE/4.2.5.4-returns-InitialLab-not-outRamp-first` | **`7,629 5×10⁻⁴`** | half **one** general-PCSLAB `L*` quantum (`100/65 535`). `A2B1` at device `(0,0,0)` is a CLUT **corner** read through identity curves — *no interpolation happens*, so no interpolation term exists; no oracle term, because no oracle is consulted; the chroma terms are exactly zero because 4.2.3 assigns neutral literally and the estimator carries `InitialLab` through without arithmetic. **No free parameter.** Observed `1,907×10⁻⁴` — the generator's own rounding of `12,5`, which is not exactly encodable and decodes to `12,500 190 7` |
| `pass5c/<arm>/FIXTURE/candidates-are-separated-as-designed` | **`2,288 9×10⁻³`** | **three** named half-quanta, one per encoding the number passes through: `InitialLab`'s encode; the round-trip floor's, read back out of two interpolated `A2B` nodes each within half a quantum; and the `B2A`'s stored `u16` `G` floor converted to `L*` through `dL*/dG`, bounded by **100** — the whole range — rather than this fixture's `87,5`, because a fixture-specific slope inside a tolerance goes stale when a constant moves. Observed `5,821×10⁻⁹` |

Both are `derived-expectation`, not `cross-check`: the expected value is a named
constant in `recipes.rs` put through a clause. **They run on a bare checkout** —
no oracle, no system profile, no shipped binary — which is deliberate and is the
point: a derived expectation must not be hostage to an oracle. They are emitted
outside `analyse` for exactly that reason.

The second row is the one GP-002 demands. **The separation mechanism can report
that a row is blind; only a graded row can stop it becoming blind**, and the
collapse arrives as a consequence of reasonable-looking edits rather than as a
mistake anyone makes on purpose.

#### 3.5.9.4 Proof of power — run, not asserted

Detached worktree at the same HEAD; the pre-`fd34a44` return value injected into
`bpc.rs`; **both** category (c) profile constants repointed at a non-existent
drive to simulate a clean machine.

```text
summary      pass=129   fail=1   skip=30   error=0
pass5c rows skipped for want of a system profile: 27

pass5c/floored/CLAUSE/4.2.5.4-returns-InitialLab-not-outRamp-first
    FAIL  observed 2.500019e1  tolerance 7.629511e-4  sep 2.500019e1  DISCRIMINATING
pass5c/synthetic/CLAUSE/4.2.5.4-returns-InitialLab-not-outRamp-first
    PASS  observed 0.000000e0  tolerance 7.629511e-4  sep 0.000000e0  ZERO-SEPARATION
```

The failing row was the **only** failure in the suite. The sibling arm's
identical row stayed green with `ZERO-SEPARATION` beside it — GP-002
demonstrating itself on the same run rather than being asserted in a README.

#### 3.5.9.5 ★ A third arm made §B's own precondition explicit, and the apparatus row caught it

On the first run of the third arm,
`pass5c/floored/apparatus/error-bar-is-smaller-than-the-effect` **failed at
`3,775×10⁹`** against its bound of 1. The row was right. §B converts a device
residual into an `L*` bound by dividing by `d(device)/d(L*)` measured on `B2A1`,
and on this fixture that derivative is **zero by construction** — the floor makes
every `Lab` below `L* 37,5` map to one device value. Measured: `1,11×10⁻¹⁶`.
**§B is void on that arm**, and the row whose whole job is to say when §B is void
said so on its first exposure to a fixture that made it true.

The response was **not** to widen `APPARATUS_RATIO`. Its constant `1.0` is
unchanged and still applies wherever the conversion it depends on exists. What
was added is a **declaration**:

- `DEVICE_OBSERVABLE` — a table, one line per arm, saying whether that fixture
  makes the destination black observable in device space. `swop` true,
  `synthetic` true, `floored` **false by design**. Authored, reviewable in a
  diff, *not inferred at run time* — a row that demoted itself from graded to
  reported whenever a measured quantity came out small would disable exactly the
  check that would catch a real collapse.
- `pass5c/<arm>/apparatus/black-is-device-observable-as-declared` — graded at
  **exactly 0/1**, the measurement against the declaration, so the table cannot
  drift from reality in either direction. Cutoff `10⁻⁶` normalised device per
  `L*`, **derived from the shipped surface**: the CLI prints six decimals, so
  below that one whole `L*` of black-point error moves the printed output by
  less than one printed digit. Margins are not close — `swop` `1,7×10⁻²`,
  `synthetic` `8,1×10⁻³`, `floored` `1,1×10⁻¹⁶`.

#### 3.5.9.6 ★ Why `estimators/black-points-in-lab` stays `REPORTED`

Asked directly: the row named for the whole finding is `UNGRADED`, and a
`4,717 441` separation now exists — does that supply the derivation basis for a
real tolerance? **No.**

1. **There is nothing for a bound on that row to mean.** Since `fd34a44` both
   sides return a quantity their own document calls `InitialLab`, and the two
   documents mean different things by the name. **No clause requires them to
   agree**; grading their difference is grading iccce against lcms2's reading of
   a document iccce does not implement, which §1 and `CLAUDE.md` rule 7 both
   forbid.
2. **A bound derived from the separation is a bound fitted to one known
   defect.** Anything below `4,717 441` would have failed the pre-`fd34a44`
   build and anything above would not; nothing else constrains it. And it could
   not be *one* number — the three arms observe `4,799`, `5,000` and `10,000`,
   so it would be three constants each fitted to its own fixture. That is a
   tuned tolerance arrived at from the other end.
3. **The defect it would have caught now has a row with a real derivation** —
   §3.5.9.3, proved in §3.5.9.4.

**The generalisation:** a large separation on an `UNGRADED` row is a request for
**a fixture and a graded row elsewhere**, not a licence to grade that row. Ask
what clause the number would be graded against; if the answer is *"none, but it
would have caught the bug"*, the bound is fitted to the bug.

### 3.6 Pass 6 — performance, and the price of speed

**Filled 2026-08-12 by `icc-conformance`** from comparisons actually run.
Apparatus: `tools/difftest/src/pass6.rs`. Full record:
`tools/difftest/README.md` **§18**.

Pair: `USWebCoatedSWOP.icc` `A2B1` (`mft2`, 4-D, 9 CLUT nodes per axis) → the
system sRGB profile, **media-relative** — `iccce bench`'s own default pair, at
its own default grid, over its own **513** sampled probes, reproduced exactly
so that every number below is a *translation of what the shipped binary
prints* rather than a differently-shaped measurement of the same subject.

> ★★ **RE-GRADED 2026-08-12 AT THE NEW DEFAULT GRID OF 33 — THE GATE NOW
> PASSES, AND THE TOLERANCE DID NOT MOVE.** §3.6.1 said the remedy was the
> grid and not the number; commit `189e732` changed
> `compiled::recommended_grid_points` from 17 to 33 for 3-D and 4-D, and rows
> R3 and R4 went green **against the same `2,5×10⁻¹` derived from Pass 4's own
> measurement**. The `Measured` column below carries **both** dates: the
> grid-17 observations that failed and the grid-33 observations that pass.
> Nothing in the `Tolerance` or `Justification` columns changed, which is the
> only outcome that makes a tolerance worth having written down.

**Every row here is `self-consistency`.** Both arms are iccce and the compiled
grid is *built by sampling* the reference path, so no row is evidence that
either arm is correct however small it is (§1). That is why §A's tolerance is
derived from Pass 4's **cross-check** figure and from nothing measured here.

| Comparison | Kind | Metric | Tolerance | Justification | Measured |
|---|---|---|---|---|---|
| **R1. Apparatus — the harness reproduces `iccce bench`** — `pass6/apparatus/harness-reproduces-bench` | self-consistency | abs-max, device | **1×10⁻⁹** | The CLI prints `error.max_device_offnode` to **nine** decimals, so one printed lsb is 10⁻⁹, and the bound is that and nothing else. **The precondition for R3 being a translation of the benchmark**: it cannot absorb a different probe set, grid or intent, each of which moves this quantity by ≥10⁻⁴. | 2026-08-12 (grid 17) — **2,537×10⁻¹⁰**; 2026-08-12 (grid 33) — **2,739×10⁻¹⁰**. ★ **This row is what caught the default moving.** When `recommended_grid_points` changed and the harness constant did not, it failed at **1,576×10⁻³** — which is not an error but the gap between the two grids' costs. A cheap row that fails loudly when the two arms stop describing the same transform is worth more than an expensive one that averages over it. |
| **R2. Structural — identical at nodes, 4-D** — `pass6/structural/identical-at-nodes-4d` | self-consistency | abs-max, device | **1×10⁻¹²** | **STRUCTURAL, NOT EVIDENCE (DL-023).** At a node the compiled value *is* a stored reference evaluation, so equality is by construction; the row grades only that the 4-D index arithmetic in `CompiledTransform::new` and `Clut::eval` share a channel order. `compiled.rs`'s own node test runs on a **3**-channel identity chain where a transposition of a symmetric grid can hide. **Must never be cited as the compiled path's error.** | 2026-08-12 — **0,0** over 251 lattice nodes |
| **R3. ★★ THE GATE — the compiled path's cost in ΔE2000, graded at the shipped default grid (33 since `189e732`; 17 before it)** — `pass6/swop-to-srgb/media-relative/compiled-cost-de2000` | self-consistency | ΔE2000 max, D50 | **2,5×10⁻¹** | **The derivation has no free parameter, and the derivation that was rejected is worth as much as the one chosen.** *Rejected:* "an order of magnitude below §2's provisional 1,0 anchor" — that presumes the engine's approximations sum below the anchor, and **NA-006 alone was measured at 1,574 ΔE2000** on `A2B0` of this same file, so it is a budget derived from a total already exceeded. *Chosen:* **compiling must not move the result further than the two implementations already differ on the same transform.** Pass 4 measured iccce vs lcms2 on this exact pair at media-relative: **0,252 94 ΔE2000** (§3.4.1). This is that number to one significant figure — no headroom, no multiple, no anchor. Failing it means compiling is the **dominant** error term on this transform. **Not** a perceptibility claim; §2's ⚠ is neither cited nor inherited. **GRID-DEPENDENT IN ITS APPLICABILITY, NOT IN ITS DERIVATION — corrected 2026-08-12, see §4.** There is **no compiled grid anywhere in this bound's derivation**: Pass 4 compares the *reference* path against the oracle over 341 CMYK points and never builds a `CompiledTransform`, so there is nothing in the number that a grid change can make stale — which is why it survived the default moving without being touched. What the grid governs is where an *observation* may be compared to it: the graded quantity is `O(h^1,32)` (R6), so a measurement is comparable only at the grid it was taken on, and that is **33** since `189e732`. | 2026-08-12 (grid 17, the then-default) — **★ FAIL, 2,970 17×10⁻¹**, 17 % over; maximum at CMYK `(0,0196 0,1476 0,2757 0,4037)`, reference `L* 62,53`, mean **5,359×10⁻²**. 2026-08-12 (grid 33, the default since `189e732`) — **★ PASS, 1,677 3×10⁻¹**, 33 % inside the line. **The constant did not move.** |
| **R4. ★ The same gate on PASS 4's OWN grid** — `…/compiled-cost-de2000-on-pass4-grid` | self-consistency | ΔE2000 max, D50 | **2,5×10⁻¹** | R3's antecedent (0,252 94) was a maximum over Pass 4's **341-point** CMYK grid; R3's observation is a maximum over the benchmark's **513** raster probes. **A maximum over one population is not a maximum over another**, so the line is checked on both and the verdict does not rest on a population mismatch. | 2026-08-12 (grid 17) — **★ FAIL, 2,962 90×10⁻¹**, within 0,25 % of R3, so the failure was a property of the transform and not of either probe set. 2026-08-12 (grid 33) — **★ PASS, 9,348 6×10⁻²**. ★ Note that at grid 33 the two populations no longer agree to 0,25 %: R3 is **1,79×** R4, because the benchmark's raster probes and Pass 4's CMYK grid stop being interchangeable once the error is small enough for probe placement to dominate. **Both are inside the line, and quoting either alone would now be a population claim.** |
| **R5. The device cost** — `…/compiled-cost-device` | self-consistency | abs-max, device 0..1 | **∞ — REPORTED, NOT GRADED** | The quantity `iccce bench` prints. **Deliberately ungraded, and the arithmetic that made it ungradeable is the point.** The device bound implied by R3 is `2,5×10⁻¹ ÷ 136 = 1,84×10⁻³`, using sRGB's *shadow* sensitivity (§3.4.4 row C3's chain) — **tighter than the observation**, while the observed maximum is a **midtone**. Grading it there would fail runs R3 passes and assert something neither row means. *The same physical event has a different size in two units, and the unit the requirement is stated in is the one that may carry the tolerance.* | 2026-08-12 (grid 17) — **3,588 962×10⁻³** · (grid 33) — **2,012 444×10⁻³**. Still above the `1,84×10⁻³` shadow-derived bound the ΔE row implies, and still ungraded for the same reason. |
| **R6. ★★ The sensitivity control (DL-018)** — `pass6/control/error-scales-with-grid-spacing` | self-consistency | band violation | **0,0 — exact** | Graded on the **paired median** of `err(coarse)/err(fine)` **at the same probe**, over the halvings 9→17 and 17→33, against the band `[2, 8]`. **The band asserts only that the observed convergence order lies in `[1, 3]`** — below order 1 the error is not grid-driven and no number from this instrument is evidence (ratio → 1 is exactly what `compiled.rs`'s own control hit on an identity chain); above order 3 is impossible for multilinear interpolation and would mean the probes are collapsing onto nodes. **It deliberately does NOT assert order 2.** `0,0` is honest because the quantity is a `max(0, ·)` of a band test, not a residual. **★ BOTH THE ESTIMATOR AND THE JUSTIFICATION WERE WRONG ON THE FIRST RUN** — see §4. | 2026-08-12 — **0,0**. Paired medians **5/9 = 2,69 · 9/17 = 2,47 · 17/33 = 2,51** — stable to ~1 % across three octaves, i.e. **convergence order `log₂2,5 = 1,32`, not 2**. |
| **R7. The falsified estimator, kept on file** — `pass6/control/max-of-max-is-the-wrong-estimator` | self-consistency | band violation | **∞ — REPORTED** | The band violation on the **max-of-max** ratio — the estimator `compiled.rs`'s unit test uses. It divides one maximum by another and the two are not at the same probe, so as the grid refines it measures *where the worst point moved* as much as the law: **5,57 → 1,39 → 1,78**, a factor of 4 of wander, against the paired median's 2,69 → 2,47 → 2,51. **A clamp attribution was written, tested and falsified here**: restricting to cells whose 16 corners are all in gamut and above sRGB's 0,040 45 breakpoint changed the ratios **not at all**. | 2026-08-12 — **6,144×10⁻¹**; 65/513 probes out of gamut, 448/513 in cells smooth at 9, 17 **and** 33 |
| **R8. The grid trade, reported** — `pass6/grid-{5,9,17}/compiled-cost-de2000` | self-consistency | ΔE2000 max | **∞ — REPORTED** | 33 is the shipped default since `189e732` and the only grid R3 is graded at; these say what the alternatives cost, so the trade is visible rather than asserted. **The row's membership follows the default** — 17 moved from being the graded grid to being a reported alternative on the same day. | 2026-08-12 — grid **5: 7,284×10⁻¹** (build 0,009 s) · **9: 4,046×10⁻¹** (0,086 s) · **17: 2,970×10⁻¹** (1,06 s) · **33, the default: 1,677×10⁻¹** (14,0 s) |

#### 3.6.1 ★★ The verdict on R3/R4, and what must NOT happen next

**At the shipped default grid of 17, compiling this transform costs more than
iccce and lcms2 differ by on the same transform** — 0,297 against 0,253 — and
that holds on both probe populations. `iccce bench`'s device figure of
3,589×10⁻³ looks negligible and is not: carried into a space where a ΔE means
something it is **17 % above the entire implementation-to-implementation
spread**.

Three things follow, in this order:

1. **The remedy is the grid, not the number.** Grid **33** measures
   **1,677×10⁻¹**, comfortably inside the line, for a build cost of 13,8 s and
   1 185 921 nodes. Whether that trade is acceptable is the engineer's and the
   operator's call, not this document's; what is not available is moving
   `2,5×10⁻¹`, because it is Pass 4's measured number and has no free parameter
   to move.
2. **Refining is dearer than it looks.** R6 measures the convergence order at
   **1,32**, not 2. Doubling the grid density costs **~15× the build** and buys
   **~2,5×** the accuracy, not 4×. Anyone budgeting a default grid from an `h²`
   assumption will overestimate what refinement buys, by a lot.
3. **A red suite is the correct state here.** `TOLERANCES.md` §0's procedure was
   followed in order and stopped at step 1: the code is not wrong (R1 reproduces
   the shipped binary to 2,5×10⁻¹⁰, R2 is exactly zero, R6's control passes), no
   expectation is involved, and the fixture is the benchmark's own. **The suite
   is red because a shipped default does not meet a justified line**, which is
   what a conformance suite is for.

#### 3.6.2 What Pass 6 did NOT measure

- **Any direction but `A2B`, any intent but media-relative, any pair but one.**
  DL-021: an error measured compiling a CMYK→RGB `A2B` path says nothing about
  the `B2A` path on the same two files.
- **The compiled path with BPC in the chain.** `CompiledTransform` folds
  whatever the `Chain` contains, and every row above folds a chain with BPC
  **off**.
- **Throughput as a graded claim.** Reported in the record's context field and
  graded nowhere; a wall-clock figure from one machine is not a
  tolerance-bearing claim. **What is quotable, and what is not, is re-filed in
  §3.6.3 — the numbers that stood here were measured at the OLD default grid
  of 17 and one of them was never reproducible at any grid.**
- **`convert_buffer`'s shape refusals** and the `GridTooLarge` path, both of
  which `compiled.rs`'s unit tests cover and this suite does not.

#### 3.6.3 ★★ Throughput, re-filed 2026-08-12 — a break-even carries its grid, and the speedup is withdrawn

**What this subsection replaces.** §3.6.2 carried, until this filing:
*"2,4–2,7 Mpix/s compiled against 0,076–0,091 Mpix/s reference (**28–32×**,
break-even ≈63 000–75 000 px) … the run-to-run spread across four invocations
in one session was ~10 %."* Every figure in that sentence was measured while
`compiled::recommended_grid_points` still returned **17**. Commit `189e732`
made it **33**. Two separate things then went wrong with the sentence, and
they have different causes and different remedies.

##### (a) The break-even was never grid-free, and is now stated with its grid

Break-even `N` solves `build + N/compiled = N/reference`, i.e.

> `N = build ÷ (1/reference − 1/compiled)`

**`build` is in the numerator, and `build` is what the grid moved.** Nothing
else in the expression depends on the grid: measured below, the compiled and
reference throughputs at 17 and at 33 are indistinguishable. So the whole of
the 14× shift in break-even is the 4-D build going from ~0,84 s to ~12,4 s,
and the old figure was not "out of date" so much as **incomplete** — *a
break-even without a grid is like a tolerance without units*.

**Measured 2026-08-12, ten `iccce bench` invocations in one session**, same
binary (`cc03f3d`, release, MSVC), same pair (`USWebCoatedSWOP A2B1` → system
sRGB, media-relative), same 8 700 867-px raster, five at each grid:

| grid | build (s) | compiled (Mpix/s) | reference (Mpix/s) | **break-even (px)** |
|---|---|---|---|---|
| **33 — the shipped default** | 12,05 – 12,91 | 1,18 – 2,46 | 0,092 – 0,099 | **1,23×10⁶ – 1,39×10⁶** |
| 17 — the previous default | 0,82 – 0,94 | 1,23 – 1,36 | 0,092 – 0,099 | **8,3×10⁴ – 9,9×10⁴** |

- **The figure to quote is `≈1,3×10⁶ px at grid 33`**, and only ever with the
  grid attached. The suite's own run the same day printed **1 169 350 px** and
  an earlier invocation **1 258 593 px**, so across the day the observation is
  **1,17–1,39 Mpix**, a ±9 % band.
- **Ratio of the two medians: 85 900 → 1 273 800 = 14,8×.** Ratio of the two
  median build times: 0,838 → 12,444 = **14,8×**. They agree to three figures,
  which is the arithmetic saying the shift is *entirely* the build.
- **Concretely:** at grid 33 compiling pays for itself at about a 1140 × 1140
  image. `iccce bench`'s own A4-at-300-DPI raster is 8,7 Mpix, so it pays
  ~7× over there; a 1024 × 768 thumbnail sheet (0,79 Mpix) it does not.
- ★ **Break-even is a far more stable statistic than speedup on this machine,
  and that is structural rather than lucky.** Since `1/compiled ≪ 1/reference`
  (the compiled path is >12× faster), `N ≈ build × reference_rate` and the
  noisy term barely enters: over the five grid-33 runs the compiled rate spans
  **2,08×** while the break-even computed from those same runs spans **1,13×**.
  A quantity that is insensitive to the arm that varies is the one to publish.

##### (b) ★ The speedup is WITHDRAWN as a documented range

`28–32×` is not reproducible and the "~10 % run-to-run spread" that was
recorded beside it understated the variance by an order of magnitude. In the
ten invocations above, **on one machine, one binary, one session**:

| | grid 17 | grid 33 | both |
|---|---|---|---|
| speedup, five runs each | 12,72 – 14,67× | **12,44 – 25,27×** | **12,44 – 25,27×** |
| spread (max ÷ min) | 1,15× | **2,03×** | **2,03×** |

Adding the earlier sessions on the same machine — 28–32× (2026-08-12 morning),
22,85× (2026-08-12 midday), 21,35× (this suite run), 14,4× (an earlier session
reported by the engineer) — the observed range is **12,4× to 32×, a factor of
2,6**, with no change of grid, code or workload to attribute it to. It is
machine load.

**Decision (`icc-conformance`, 2026-08-12): this project does not carry a
speedup figure.** The defensible sentence is *"compiling amortises its build
after roughly N pixels at grid G"*; *"N× faster"* is not, because the
denominator is a wall clock on a loaded desktop and the numerator is the same
wall clock a second later. The `speedup.compiled_over_reference` line stays in
`iccce bench`'s output — it is a diagnostic a user runs on **their** machine —
and it is not restated in any project document as a property of the engine.
Anywhere a speedup is quoted it must be labelled *observed on one Windows box
under unknown load, range 12,4–32×*, which is a sentence that argues against
quoting it.

##### (c) The reference arm's recorded band was wrong, but the reference arm is not the unstable one

The old **0,076–0,091 Mpix/s** band does not contain today's observations
(**0,092–0,099** across all ten runs, at both grids). It should not be read as
"the reference arm has become unstable": **within this session the reference
arm is the *tightest* quantity measured**, ±4 %, against ±35 % for the
compiled arm. The band was a four-sample range from one session being quoted
as if it were a property of the machine. **Recorded here as the same class of
error as (a): a spread measured in one sitting is an observation of that
sitting.** The reference arm times only the first 100 000 px of the raster
(`iccce-cli` bounds it, since the reference path is ~13× slower), which is
~1,1 s of work — long enough that this is not sampling noise.

##### (d) Coverage of this subsection, stated

**One machine** (Windows 11, MSVC, release, single-threaded), **one pair**,
**one intent**, **one raster size**, **fifteen invocations across three
sessions on 2026-08-12**. No other CPU, no other OS, no multi-threading, no
other profile pair. Nothing here is a claim about the engine's performance in
general, and the only quantity that survives contact with the variance is the
break-even, at a named grid.

---

### 3.7 ★★★ Pass G — the Ghent v5.0 population sample

**Filed 2026-08-17 by `icc-conformance`, measured at tip `e21154c`, release
build, Windows 11 / MSVC, oracle pin `21c582a` (lcms2 2.19.1).** Code:
`tools/difftest/src/passg.rs`. Instrument that derived the envelopes:
`tools/difftest/src/bin/ghent_probe.rs` — **which grades nothing and never
fails**, deliberately, so that no number below was chosen by watching a
comparison go green.

**Whole-suite result with Pass G registered: `pass=229 fail=0 skip=3 error=0`,
exit 0** (baseline before it: `pass=157`). **72 rows, and every one of them
states a candidate separation** — the separation aggregate's `unstated` count
is unchanged at 119, so Pass G contributed **zero** unstated rows, `blind=0`,
`sep-broken=0`.

#### 3.7.0 What is new about this pass, and what is not

Every profile graded before it was **synthetic** (`tools/gen-profiles`),
**OS-shipped** (the Windows colour directory) or **standards-body-issued**
(FOGRA51). Pass G grades against 20 profiles extracted from the **Ghent PDF
Output Suite 5.0** — written by Adobe InDesign CS6, imposed by Callas
pdfToolbox, and embedded 121 times across 98 production PDF/X files. It is the
first time this project has measured itself against *what a real document
producer actually embeds*.

★ **It is a compatibility exercise, not a conformance certification.** Nothing
was proofed on a press or measured with an instrument; the operator has ruled
that path out. The strongest claim any oracle row here makes is *"iccce and
lcms2 read this profile the same way"*, which §1 ranks below the eight
`derived-expectation` rows in this section and far below published ground
truth, of which Pass G has **none**.

★★ **The corpus is licensed and uncommittable.** The Ghent suite's licence
forbids redistribution, and the profiles carry Adobe's, ECI's and X-Rite's
separate licences. It lives in `D:\Dev\iccce-private-fixtures\ghent-v50\`,
is resolved through `$ICCCE_PRIVATE_FIXTURES`, and **every row SKIPs with a
reason when it is absent**, which is the permanent state in CI. A green CI line
for these rows says they **did not run**. No value from that directory appears
in this repository: every number in the records is formatted at run time from
the file on the operator's disk, and the only corpus identifiers in source are
SHA-256 prefixes and file names.

#### 3.7.1 §A — the v4 vendor `mAB ` path (the row this pass exists for)

**Subject:** X-Rite's `GWG_ICC_v4_testprofile.icc`, ICC **v4.2.0**, `prtr`
CMYK → `Lab `, `mAB ` `A2B0/1/2` at **7×7×7×7** with 4096-entry A curves, `mBA `
`B2A0/1/2` at 17³, plus `gamt` and four `gbd` tags.

**Why it matters more than its row count suggests: every v4 LUT profile iccce
had ever been graded against came from `tools/gen-profiles`, i.e. from us.** A
shared misreading between the fixture generator and the engine would have been
invisible to §3.4.4's `derived-expectation` rows, because the fixture and the
derivation come out of one corpus (§3.4.4.1 says so in its own text). X-Rite
authored this one.

| row | kind | tolerance | where the number comes from | observed |
|---|---|---|---|---|
| `…/{A2B0,A2B1}/apparatus-harness-mab-matches-iccce-cmm` | self-consistency | **1×10⁻⁹** | ~7 orders above `f64` noise, ~5 below anything colorimetric | **0.0** both |
| `…/{A2B0,A2B1}/pcs-lab-vs-lcms2` | cross-check | **envelope × 1.25**, computed per tag at run time | the CLUT method envelope from *this tag's own* 7⁴ table under the two published geometries; **no lcms2 output in it** | **0.950 274** (A2B0, tol 1.185 199) / **0.828 444** (A2B1, tol 1.035 154) |
| **`…/{A2B0,A2B1}/pcs-lab-emulated-geometry`** | cross-check | **2×10⁻²** | lcms2's own quantisation once its geometry is substituted (see below) | **3.912×10⁻³** / **4.624×10⁻³** |
| `…/{A2B0,A2B1}/pcs-lab-corners-interpolation-free` | cross-check | **1×10⁻³ or 2×10⁻³**, chosen from the tag's B curves | see §3.7.2 — this one was wrong on its first run | **1.112×10⁻³** (A2B0, tol 2×10⁻³) / **6.074×10⁻⁵** (A2B1, tol 1×10⁻³) |
| `…/a2b1-equals-a2b2-byte-identical` | derived-expectation | **0.0** | an integer comparison on file bytes, not a residual | **0.0**, separation **255** |
| `…/perceptual-black-equals-its-own-B-curve-floor` | derived-expectation | **1×10⁻⁴** | `transicc`'s 4-decimal `L*` print | **4.510×10⁻⁵** |
| `passg/xrite-v4-to-srgb/<intent>/…` | cross-check | **propagated envelope × 1.25 + 1×10⁻⁴** | the method envelope pushed through the *actual* destination model point by point | **1.488×10⁻²** (perceptual, tol 1.872×10⁻²), **8.984×10⁻³** (media-relative and saturation, tol 1.130×10⁻²) |

★ **The headline result: the raw disagreement on the v4 `mAB ` path IS the
interpolation method, and nothing else.** With lcms2's own `Eval4Inputs`
geometry substituted into the harness pipeline, `A2B1`'s residual collapses
**0.828 444 → 4.624 5×10⁻³, a factor of 179**, and `A2B0`'s
**0.950 274 → 3.912 3×10⁻³, a factor of 243**. The method envelope computed
from the CLUT bytes alone is **0.828 123** and **0.948 160** — i.e. the raw
residual is accounted for to **0.04 %** and **0.22 %**. This is the same
signature Pass 4 established on a `lut16` profile, now reproduced on a
vendor-authored v4 one, and it is the first time §3.4.3's "any **real** v4 LUT
profile" gap (recorded as open since 2026-08-11) has been closed.

★ **What §A's PCS rows do and do not compare, stated because it is easy to
misread.** The three PCS rows compare **the harness's `mAB ` reimplementation**
to lcms2, not iccce to lcms2 — the substitution they depend on cannot be made
inside `crates/`, because the shipped engine has one interpolation scheme by
design. The link to iccce is the **apparatus row**, and it is graded at
`1×10⁻⁹`. Injection I1 (§3.7.6) confirms the linkage empirically: corrupting
iccce's v4 PCSLAB decode turns the apparatus rows red and leaves the three PCS
rows green.

★★ **lcms2's forced BPC fires on this profile, and §A defeats it with a
FIXTURE rather than subtracting it with a model.** The profile's `A2B0` `L*`
B curve is a 2-entry `curv` `(0x0808, 0xFFFF)` — its declared perceptual black,
`2056/65535 × 100 = 3.137 254` `L*`. Into a **v2** PCS lcms2 prints exactly
that for full ink; into a **v4** PCS it prints **0.0**, because
`_cmsLinkProfiles` forces BPC for perceptual when the *destination* is v4
(Pass 4b finding 2, §3.4.4). §A therefore runs the perceptual arm against
`*Lab2`. That is the Pass 4c lesson applied again: **a fixture that keeps the
gate shut is better evidence than a model that subtracts what the gate did**,
because a model can absorb an arithmetic error along with the policy difference
it was built to isolate. The `perceptual-black-equals-its-own-B-curve-floor`
row grades the whole mechanism against **the profile's own bytes**, with the
v4-PCS answer carried as its named separation.

★ **Intent-tag aliasing is a VENDOR choice, and this corpus contains three
different ones.** X-Rite aliases `A2B1 ≡ A2B2` (media-relative and saturation
are one block at two offsets); both ECI profiles alias `A2B0 ≡ A2B2`
(perceptual and saturation); the GWG CMYK trap aliases `B2A0 ≡ B2A2` as well.
**An engine — or a test suite — that hard-coded one pairing would be wrong on
the others.** §A grades the aliasing at exactly `0.0` and does **not** give
`A2B2` an arm of its own, because a saturation arm on this profile would
reproduce the media-relative arm bit for bit and add green lines that measure
nothing. §3.4.4's rule stands: a null that is null by construction must be
identified before it is collected, never explained afterwards.

#### 3.7.2 ★★ The corner tolerance was WRONG on its first run, and the term was found rather than the number moved

Its first draft was a single constant `1×10⁻³`, justified by: *"at a node
lcms2's quantisation terms vanish rather than accumulate — the CLUT input is an
exact `u16`, the interpolated value **is** the stored `u16`, and the 2-entry B
curves are affine; what remains is `transicc`'s 4-decimal print, a ΔE00 floor
of ≈1×10⁻⁴, and 1×10⁻³ is 10× that."*

**The `A2B1` arm measured 6.074×10⁻⁵ — the print floor, exactly as predicted.
The `A2B0` arm FAILED at 1.111 856×10⁻³.** §0's procedure, in order:

1. **Is the code wrong?** No. The per-corner dump (`ghent_probe`) shows the
   disagreement sits entirely in `L*`, at up to 0.001 16 — **0.76 of one lsb of
   `1/65535`** — and only on the tag whose `L*` B curve is *not* the identity.
2. **Is the expectation wrong?** There is none; both sides are computed.
3. **Is the fixture wrong?** No.
4. **The derivation was wrong**, and specifically one clause of it: *"the
   2-entry B curves are affine"* is true of both tags and **irrelevant**. What
   matters is whether they are the **exact identity** `(0x0000, 0xFFFF)`.
   `A2B1`'s are, so lcms2's `cmsEvalToneCurveFloat` `u16` round trip is
   lossless. `A2B0`'s `L*` curve is `(0x0808, 0xFFFF)`, a *non-identity* affine
   map, and lcms2 evaluates it through `cmsEvalToneCurve16` — rounding **twice**
   (input to `u16`, lossless at a node; output to `u16`, not) where iccce and
   the harness use `f64`. Two roundings of ≤½ lsb is ≤1 lsb; one lsb of encoded
   output is `100/65535 = 1.526×10⁻³` in `L*`; `S_L ≥ 1` off mid-lightness.
   Bound `≈1.526×10⁻³`; observed 0.73 of it.

**The fix is not a wider constant — it is a tolerance that is a FUNCTION of the
tag's own B curves** (`passg::corner_tolerance`, selected at run time by
`is_identity_curve` reading the stored `u16`s). A tag with identity B curves
still gets the tight `1×10⁻³` floor and does not inherit an allowance it does
not need. **Injection I3 proves the selection is load-bearing**: forcing the
identity branch turns exactly one row red — the `A2B0` corner row, at the same
1.111 856×10⁻³ — and nothing else.

The guess is preserved verbatim above so the change is auditable, per the
precedent set by §3.4.4 row C1.

#### 3.7.3 §B — the population sweep, and the honest limit of its claim

Five pairs (`sRGB→ISO Coated v2`, `Adobe RGB→Coated FOGRA39`,
`eciRGB v2 v4.2→ISO Coated v2`, `eciRGB v2 v2.4→ISO Coated v2`,
`Gray→ISO Coated v2`) × **four intents** × **±BPC**, 213 RGB / 69 gray points,
both sides in their own process.

**`SWEEP_DEVICE = 4×10⁻³`, and it is derived from the DISCRIMINATION
REQUIREMENT, not from the observation.** Every destination here is a Lab-PCS
**output** LUT, so `_cmsReadOutputLUT` forces trilinear and the
interpolation-method envelope is **identically zero** in this direction
(Pass 4b finding 1). That removes the dominant term *and* the obvious way to
derive a bound:

- **A closed-form union bound was tried and discarded as useless**, exactly as
  Pass 4 records. lcms2's tabulated-TRC rounding is ≤1.53×10⁻⁵ in linear RGB;
  `da*/dX ≤ 4038` carries that to ≈2.4×10⁻⁴ in *encoded* `a*`; the steepest of
  the six destination tables measured has a node-to-node slope of **14.836 2**
  per encoded unit; the product over three channels is ≈1.1×10⁻² — **wider than
  the rival it is supposed to discriminate.** A bound that cannot separate the
  two candidates is a formality, not a bound.
- **So the number is set by the rival instead.** The named alternative on every
  §B row is *"lcms2 had NOT forced trilinear and had used its default
  tetrahedral geometry"*, computed at run time from each destination's own
  `B2A1` table. Measured over this sweep: **1.235×10⁻² to 3.306×10⁻²**.
  `4×10⁻³` leaves every row **at least 3×** of discrimination against it, and
  the report's `separation` column states that multiple per row rather than the
  document asserting it.
- **It also catches** the v2/v4 legacy Lab decode error (`ARCHITECTURE.md` §2's
  named hazard): ≈0.39 `L*` at white and 0.4 % on the encoded `a*`/`b*` scale,
  which through slopes of 5–15 is ≥10⁻² device — 2.5× this gate at minimum.

★★ **What §B does NOT do, stated as prominently as what it does: it does not
claim agreement.** There is no attribution row for §B, because the harness has
no `mft2` B2A model to substitute lcms2's arithmetic into — Pass 4b built one
for `mft1`/`lut8` only. §B is a **structural gate with a stated rival**, and
the honest reading of a green §B row is *"no structural error, and not the
tetrahedral rival"*. **This gap is owed work, and recording it here is not the
same as closing it.** The observations sit 17–63× below the gate
(**2.7×10⁻⁵ to 2.36×10⁻⁴** across the twenty graded rows), and that margin is
**not** an agreement claim: §5.2's rule that an unexplained small residual is
*unexplained*, not *successful*, applies with full force.

**Two circumstances take a §B row out of grading, and each says so on the
line:**

| circumstance | rows | why not gated |
|---|---|---|
| ICC-absolute out of `sRGB` or `Adobe RGB (1998)` | 2 | the two implementations use **different destination media whites**; the mechanism is graded separately, on the profile's own bytes, in §3.7.4. Observed **2.066×10⁻¹** and **2.051×10⁻¹** device |
| anything with `--bpc` | 10 | iccce estimates the destination black by ISO/CD 18619 4.2.5, lcms2 by `cmsDetectDestinationBlackPoint`; **ICC.1 has no normative BPC text at all** (A27/A42). Gating this would gate a choice neither standard makes |

★ **ICC-absolute IS graded for `eciRGB v2` (both encodings) and for the gray
profile**, at the full `4×10⁻³`, and passes at **1.16×10⁻⁴ / 1.82×10⁻⁴ /
7.2×10⁻⁵**. Those sources encode `wtpt` **at** the PCS white, so lcms2's
substitution is a no-op and the gate cannot fire — the fixture defeats the
confound. That is the first time this suite has had a *graded, unmodelled*
ICC-absolute row through a real CMYK destination out of a real display profile.

★ **The `--bpc` mechanism, measured on three real vendor print profiles.** The
`passg/bpc-mechanism/*` rows report iccce's ISO estimator against `pass5c`'s
reimplementation of lcms2's: **2.010 883 `L*`** (ISO Coated v2 v2.4),
**0.823 487** (Coated FOGRA39 v2.1), **2.084 405** (X-Rite v4.2). What the
Ghent corpus adds to §3.5.7/§3.5.8 is that **the divergence is not an artefact
of synthetic or OS-shipped fixtures** — it reproduces, at the same order, on
profiles a real producer embeds. The device consequence is the 3.5×10⁻² to
7.6×10⁻² seen on the media-relative `--bpc` rows above.

★ **iccce refuses `--bpc` by name on eight of the twenty `--bpc` combinations**,
and those refusals are **graded as deliverables at exactly `0.0`** — the
quantity is *"did the engine decline by name"*, not a residual. `CLAUDE.md`
rule 6. The boundary of the estimation subset is part of the coverage
statement, and §3.7.5 states it.

#### 3.7.4 ★★ The authoring finding, and why it settles the ICC-absolute divergence

`COLORANTS_SUM = 2×10⁻⁴` — the `s15Fixed16` encoding floor: one lsb is
`1/65536 = 1.526×10⁻⁵`, three colorants sum three of them (4.6×10⁻⁵), and
Table 14 states the PCS white to four decimals so it is known only to ±5×10⁻⁵
per component. `2×10⁻⁴` is ≈2× that sum. It is **not perceptual and not fitted
to any observation** — it is the precision of the encoding the claim is made in.

★ **The two authoring rows are DIFFERENT CLAIMS, and the first draft conflated
them.** Where a profile's `wtpt` disagrees with its own colorant sum the
question is *which of the two is out of step*, and that is decidable **without
any external white point**: the colorant sum is compared to the normative PCS
white against a bound that is **half the distance to the profile's own rival
candidate**, so the row asks a *classification* question that cannot be tuned.
Where the two agree, that question is meaningless — and such a profile is the
**negative control**, so it gets its own row, the encoding-floor bound, and an
honest `NO-NAMED-ALTERNATIVE`.

★★★ **The finding.** Ghent's `sRGB IEC61966-2.1` and `Adobe RGB (1998)` — the
copies Adobe InDesign CS6 actually embedded — have colorants summing to the PCS
white (**1.885×10⁻⁴** and **5.396×10⁻⁶**), a `wtpt` sitting **0.264 15** away
from it, and **no `chad` tag**. Their PCS data *is* D50-adapted and their
`wtpt` is *not*. **ICC.1:2001-04 Annex A.3.1.1, VERBATIM:**

> *"If chromatic adaptation is being applied to the PCS values, the adaptation
> should be applied to the mediaWhitePointTag values as well."*

On that clause these are **defects of authorship, not a second reading of the
standard** — which is what the corpus's A4b resolution already records for the
Windows system sRGB profile (`ICC_Spec\icc\icc__s__v2_ICC1_2001_04.md` §1.1;
note §1.3's caution that this document has **no defined verbal-form hierarchy**,
so its "should" must not be cited as an ISO-directives *should*). It settles the
ICC-absolute divergence **in lcms2's favour**: iccce uses the encoded value and
lcms2 substitutes D50, and it is lcms2 that is following the clause.

**What the population sample adds, and it is the reason §B was worth running:
this is not a one-off system profile.** It is what a real producer embeds, in
98 PDF/X files, 121 times. `eciRGB v2` in both its encodings is the control that
stops the finding being read as *"every v2 display profile in the wild is
mis-authored"*: its `wtpt` and colorant sum agree to **1.526×10⁻⁵**, one lsb.

★ **`NA-00x` is NOT registered for this and no code changed.** Deciding whether
iccce should follow lcms2 here is an engineering call with a `ROADMAP` cost, not
a tolerance; this section records the evidence and the clause, and leaves the
decision where it belongs.

#### 3.7.5 §C and §D — the weakest row in this document, and the strongest

**§C (`eciRGB v2`, v2.4 against v4.2) is the weakest evidence class Pass G
produces, and it is labelled so on every line.** `Kind::SelfConsistency`:
**both sides are iccce**, on two files from **one vendor**. It prices a
representation change. It is **not** evidence that either answer is right, and
it is weaker than a cross-check because there is only one lineage in it. Graded
at `SWEEP_DEVICE` — the same number as §B, reused deliberately, because a
*different* tolerance for this comparison would be a number chosen for this
comparison. Observed **1.01×10⁻⁴** over 213 points. A companion row runs the
identical comparison with **lcms2 on both sides** (**2.29×10⁻⁴**), so a green
line says something about the two *files* and not about iccce's handling of
them.

★★ **And a NEGATIVE result, recorded because the pair looks like an instrument
it is not.** The v2/v4 pair **cannot** isolate lcms2's `wtpt` version gate:
both encodings put `wtpt` **at** the PCS white (**1.526×10⁻⁵** and
**5.396×10⁻⁶**), so the substitution is a no-op for either and the version leg
of the conjunction is never exercised. **No pair in this corpus differs only in
version while encoding a non-PCS white.** Nor is the pair a clean *version*
isolator at all: the two files differ in **TRC representation** as well —
a 700-entry tabulated `curv` against a `para` type 3 — so a disagreement
between them has **two** candidate causes and this fixture separates neither.
§5.4's rule applies: a negative result removes its own auditor, so it is
written down at the same length as a positive one.

**§D (the two GWG trap profiles) carries the only rows in this section whose
correct answer is known WITHOUT measurement.** `RGB mntr mtx X (Switch red
green)` declares the swap as its own content, so *"red in, green out"* is
checkable against the profile's own tags rather than against an oracle:
ICC.1:2022 6.3.4 / Annex F.3 gives PCS `XYZ` for device `(1,0,0)` as
`rXYZ × TRC_r(1)`, and a `curv`'s last entry maps to 1.0, **so the expected
answer IS the `rXYZ` tag's own three numbers.** `Kind::DerivedExpectation`,
graded at `SWAP_EXACT = 1×10⁻⁶` — ~1/15 of the `s15Fixed16` lsb, tighter than
the encoding **on purpose**, legitimate only because both sides decode the same
stored integers and the comparison is of two decodings of one number rather
than of two measurements. **Observed 0.0 on both rows.**

★ **The alternative candidate is named, and it is the one DL-033 asks for:**
*an engine that ignores the declared source profile and reads the colorants in
conventional order* — the failure the GWG suite designed these files to expose,
and the one its test page renders as a visible red X. Its distance is
**0.472 229** in the row's own metric (`|rXYZ − gXYZ|∞`), **472 229× the
tolerance**, and it is **supplied, not derived**, because it is a property of
the profile's two colorant tags and must not collapse to zero on the run where
the engine takes the wrong candidate. That the swap is colorimetric and not a
naming convention is graded too: the `rXYZ` tag's chromaticity has `y − x > 0`
(a **green** primary) and `gXYZ`'s has `x − y > 0` (**red**).

★★ **Injection I2 confirmed the separation was exactly right:** transposing
`rXYZ`/`gXYZ` in `MatrixTrc::from_profile` moved both rows to **0.472 229** —
*the stated separation, to six figures.* A separation that predicts the
magnitude of the injected failure is doing the job DL-033 defined for it.

Two further §D rows: the corpus contains **two files with the same `desc` and
different SHA-256s**, checked to agree at `0.0` with their colour-bearing tags
verified byte-identical in the same run (a corpus with two files under one
description is a place for a silent substitution to hide); and the CMYK trap's
`B2A0 ≡ B2A2` aliasing, the third pattern of §3.7.1.

#### 3.7.6 ★★★ Injection proof — every claim in this section was broken on purpose

§5.3 of `docs/NEXT_SESSION.md`: *a test that cannot fail is not evidence.* Four
injections were run **in a detached `git worktree`**, never in the main tree,
each reverted before the next. Baseline in that worktree reproduced the main
tree exactly (`pass=229 fail=0 skip=3 error=0`).

| # | defect injected | where | result |
|---|---|---|---|
| **I1** | v4 PCSLAB `a*`/`b*` decode scale `255 → 254` | `crates/iccce-cmm/src/lut_ab.rs` | **`pass=218 fail=11`.** Pass G red: both `apparatus` rows (at **0.894**, 894 million× the `1×10⁻⁹` bound) and all three graded `xrite-v4-to-srgb` rows. **The three PCS rows stayed green — correctly**, since they compare the harness to lcms2; the apparatus row is what ties them to iccce, and it fired. Six Pass 4b/5 rows also fired, confirming a real defect |
| **I2** | `rXYZ`/`gXYZ` transposed | `crates/iccce-cmm/src/matrix_trc.rs` | **`pass=177 fail=52`.** Both §D derived rows red at **0.472 229 = the stated separation exactly**; every §B graded row red at ~1.0; §A's end-to-end rows red. ★ `passg/trap-rgb/the-two-same-desc-files-agree` and `passg/ecirgb-v2-vs-v4/*` stayed **green — correctly**: they are self-consistency rows and are blind to a *symmetric* defect by construction, which is what their `kind` says |
| **I3** | the corner tolerance always takes the identity branch | `passg::analyse_xrite` | **`pass=228 fail=1`.** Exactly one row red: `passg/xrite-v4/A2B0/pcs-lab-corners-interpolation-free` at 1.111 856×10⁻³ against 1×10⁻³. **The run-time selection of §3.7.2 is load-bearing, not decoration** |
| **I4** | the emulated-geometry arm silently keeps iccce's geometry | `passg::analyse_xrite` | **`pass=227 fail=2`.** Both `pcs-lab-emulated-geometry` rows red at **0.828 444 / 0.950 274** against `2×10⁻²` — ★ **and their separations stayed `DISCRIMINATING` at 0.828 / 0.948 rather than collapsing to `ZERO-SEPARATION`.** That is the `Separation::against` hazard (`lib.rs`, 2026-08-12) avoided by construction: the distance is supplied as a property of the fixture, so the mechanism does not disclaim its power on the one run where it demonstrates it |

#### 3.7.7 Coverage of this section, stated

**Three vendors** (Adobe, ECI, X-Rite) and **one workgroup** (GWG); **11 of
the corpus's 20 profiles** touched, **9 not**; **one destination CMYK profile
for four of the five sweep pairs**; **one machine** (Windows 11, MSVC,
release), **one oracle pin**, **one day** (2026-08-17), **one tip**
(`e21154c`). 341 CMYK / 213 RGB / 69 gray points; 16 corners.

**Not covered, and named rather than implied:** the `mBA ` (B2A) direction of
the X-Rite v4 profile — §A grades its `A2B` only, and `B2A0`'s tabulated
4096-entry B curve is a shape nothing in this suite evaluates; `gamt` and
`gbd*` (iccce does not implement gamut tags); the 9 untouched profiles,
including the Thunderbolt display profile with `vcgt`/`mmod`/`ndin`; **eight
`--bpc` combinations that iccce refuses by name and are therefore NOT
differentially tested at all**; any perceptual claim (nothing here is measured
against a press or an instrument); and **any published ground truth** — Pass G
has none and cannot have any, for the structural reason §1 and
`icc__ref__ground_truth_availability.md` record: ICC.1 mandates no
interpolation method, so no published expected value for a LUT path can exist
even in principle.

---

### 3.8 ★★★ Pass H — acceptance and refusal, over the ICC's own published profile set

**Filed 2026-08-17 by `icc-conformance`.** Apparatus
`tools/difftest/src/passh.rs`, instrument
`tools/difftest/src/bin/passh_probe.rs`, operational notes
`tools/difftest/README.md` §23.

**Subject: which files iccce accepts, which it refuses, and whether a refusal
says why.** Not a colour value — and that is a structural limit, not an
omission. The corpus publishes *transforms*, never expected *outputs*, so no ΔE
computed on it could be ground truth (DL-041). What it can prove is broader
than anything else this project holds, because a **refusal population** and an
**acceptance population** are things a generator cannot manufacture: every file
in them was authored by somebody else, for their own purposes, before this
repository existed.

**Corpus:** `D:\Dev\iccce-private-fixtures\color-org\` — **50 `.icc` files**
downloaded by the operator from `color.org` on 2026-08-17, terms in that
folder's own `README.md` under `### color-org/`. 23 distinct `cprt` strings,
six licensing postures, **restrictive reading applies to the whole folder**.
Uncommittable; resolved through `$ICCCE_PRIVATE_FIXTURES`; **every row SKIPs
with a reason when it is absent**, which is CI's permanent state. ★ A green CI
line is evidence that nothing here ran.

**Measured population** (harness's own reading of bytes 8..12, 2026-08-17):
**40 accepted, 10 refused.** Versions `2.0.0 (5) 2.1.0 (4) 2.4.0 (6) 4.0.0 (8)
4.1.0 (1) 4.2.0 (15) 4.3.0 (1)`; classes `prtr (24) mntr (7) scnr (6)
spac (3)`; colour spaces `CMYK (23) RGB (16) 7CLR (1)`.

★ **`docs/NEXT_SESSION.md` said "two of them" were iccMAX. Two were *tested*;
ten are *present*.** The correction is filed in §4 below.

#### 3.8.1 The rows

| id | kind | metric | tolerance | observed | what it can catch |
|---|---|---|---|---|---|
| `passh/A/refusal/every-iccMAX-file-is-refused-by-name-with-its-own-version` | derived-expectation | indicator-count | **0** | 0 | a refusal that says "parse error" instead of naming iccMAX, does not quote the file's own version word, exits ≠1, or prints anything on stdout |
| `passh/A/refusal/stdout-is-empty-nothing-was-parsed-anyway` | derived-expectation | indicator-count | **0** | 0 | a partial header/tag dump escaping from a file that was refused (rule 6) |
| `passh/A/control/the-version-word-ALONE-produces-the-same-refusal` | derived-expectation | indicator-count | **0** | 0 | the ten real refusals being caused by their exotic **content** rather than by their version |
| `passh/A/gate/harness-reading-of-byte-8-predicts-iccce-on-every-file` | derived-expectation | indicator-count | **0** | 0 | any disagreement between the harness's own reading of the version word and iccce's verdict, over all 50 |
| `passh/B/acceptance/every-non-iccMAX-file-is-accepted-exit-0` | derived-expectation | indicator-count | **0** | 0 | a published, conformant v2/v4 profile that iccce cannot read |
| `passh/B/acceptance/no-malformation-is-disclosed-on-any-accepted-file` | derived-expectation | indicator-count | **0** | 0 | over-reporting by iccce **or** a defect in a published profile — the row cannot tell which, and says so |
| `passh/B/acceptance/iccce-and-lcms2-reach-the-same-verdict-on-every-file` | cross-check | indicator-count | **0** | 0 | one implementation reading a file the other cannot |
| `passh/B/acceptance/header-fields-iccce-printed-match-the-raw-bytes` | derived-expectation | indicator-count | **0** | 0 | "it parsed" without "it read the right bytes" |
| `passh/C/7clr/shipped-binary-converts-a-7-channel-source` | derived-expectation | indicator-count | **0** | 0 | the shipped binary failing or reshaping a seven-channel conversion |
| **`passh/C/7clr/pcs-corners-vs-lcms2`** | cross-check | `L*` | **2×10⁻³** | **4.900435×10⁻⁵** | ★ the first graded seven-channel row this project has ever had |
| `passh/C/7clr/end-to-end-device-corners-vs-lcms2` | cross-check | device (0..1) | **∞** | 1.191176×10⁻⁴ | reported; carries the destination's reverse-tone-curve term (see §3.8.3) |
| `passh/C/7clr/end-to-end-device-interior-vs-lcms2` | cross-check | device (0..1) | **∞** | 1.687373×10⁻³ | reported; dominated by the unlegislated interpolation method (A16) in 7 dimensions |
| **`passh/C/7clr/compiled-path-does-not-ABORT-the-process`** | derived-expectation | indicator-count | **0** | **0** (was **1 — RED** on 2026-08-17) | ★★ the process aborting instead of returning or refusing. **The defect it found is fixed; its subject has narrowed** — see §3.8.4 |
| **`passh/C/7clr/default-grid-BUILDS-and-is-the-grid-the-library-RECOMMENDS`** | derived-expectation | indicator-count | **0** | **0** | ★ added 2026-08-17 (later). A default that *refuses* rather than builds; and `recommended_grid_points` drifting apart from what `iccce bench` does with it |
| **`passh/C/7clr/oversized-grid-is-a-NAMED-refusal`** | derived-expectation | indicator-count | **0** | **0** | ★★ added 2026-08-17 (later). **The regression detector for the abort**: re-runs `--grid 33`, the exact configuration that died, and requires exit 1, empty stdout, and stderr naming all three quantities |
| `passh/C/7clr/compiled-vs-reference-at-the-default-grid` | **self-consistency** | device (0..1) | **∞** | **2.952005×10⁻³** | added 2026-08-17 (later). REPORTED for ever — **both arms are iccce** (§3.8.4's closing note on why this is not the number to grade) |
| `passh/C/coverage/6clr-evidence-is-zero` | derived-expectation | indicator-count | **∞** | 0 | reported coverage: there is none |
| `passh/E/coverage/population-breakdown` | derived-expectation | indicator-count | **∞** | — | reported coverage census |

Section D, per Probe profile (`probe-v1-icc-v2`, `probe-v1-icc-v4`,
`probe-v2-icc-v4`):

| id suffix | kind | metric | tolerance | v1/v2 | v1/v4 | v2/v4 |
|---|---|---|---|---|---|---|
| `b2a/off-colorant-channels-are-exactly-zero` | see §3.8.2 | device (0..1) | **0** (∞ on v2/v4) | 0 | 0 | 9.969315×10⁻¹ **claim FALSE** |
| `b2a/a-and-b-are-ignored` | see §3.8.2 | device (0..1) | **1.5259×10⁻⁵** (∞ on v2/v4) | 3.330669×10⁻¹⁶ | 4.440892×10⁻¹⁶ | 9.176524×10⁻¹ **claim FALSE** |
| `b2a/tint-is-monotone-decreasing-in-L` | see §3.8.2 | device (0..1) | **1.5259×10⁻⁵** (∞ on v2/v4) | 0 | 0 | 2.682589×10⁻³ **claim FALSE** |
| **`b2a/the-published-colorant-dominates-at-every-point`** | see §3.8.2 | indicator-count | **0** | 0 | 0 | **0** |
| **`a2b/the-three-published-bands-are-disjoint-and-ordered`** | see §3.8.2 | `L*` | **0** | 0 | 0 | **0** |
| `a2b/published-band-containment` | see §3.8.2 | `L*` | **∞** | 2.263736 | 2.263736 | 3.000687 |
| `a2b/vs-lcms2-through-the-same-tags` | cross-check | `L*` | **2×10⁻³** | 8.843702×10⁻⁴ | 8.843702×10⁻⁴ | SKIP (§3.8.5) |
| `a2b/encoded-pcs-clamp-divergence` | cross-check | `L*` | **∞** | 0 | **2.374×10⁻¹** | SKIP |
| `tags/mpet-is-present-and-NOT-decoded` | see §3.8.2 | indicator-count | **0** | SKIP | SKIP | **0** |
| `tags/mpet-selection-divergence-from-lcms2` | see §3.8.2 | `L*` | **∞** | SKIP | SKIP | **3.313383×10¹** |
| **`shipped/intent-selects-the-published-colorant`** | see §3.8.2 | indicator-count | **0** | 0 | 0 | **0** |

**Whole suite at first filing (2026-08-17, tip `e21154c`): `pass=270 fail=1
skip=9 error=0`,** bare exit `1`, the single failure being the compiled-path
abort of §3.8.4.

**Whole suite after the fix and the row split (2026-08-17, later the same day):
`pass=274 fail=0 skip=9 error=0`, bare exit `0`** — re-measured by
`icc-conformance` rather than taken on report, `cargo run --release` redirected
to a file with no pipe anywhere in the gate (§5.6's rule about the harness's own
exit code). `+4` on the pass count is `−1` failure becoming a pass and `+3` new
rows.

#### 3.8.2 ★★★ The first `ground-truth` rows in `tools/difftest`, and exactly what they are ground truth ABOUT

`Probev2.zip` ships **`Probe2 Profile Readme June 1.pdf`**, in which the ICC
states, in numbers, what `Probev2_ICCv4.icc` does:

> *"The rendering intent transforms (BToA tags or BToD tags) of the probe
> profile ignore the a\* and b\* components of incoming PCS colors, and map the
> L\* components directly to monotone tints of process colorants. (L\* = 0 is
> rendered as maximum colorant coverage, and L\* = 100 is rendered as unmarked
> media.) The B2A0 tag (perceptual rendering intent transform) renders the L\*
> values as tints of pure cyan. The B2A1 tag (relative colorimetric intent
> transform) renders them as tints of pure magenta, and the B2A2 tag
> (saturation intent transform) renders them as tints of pure yellow."*

> *"For the perceptual (A2B0) tag, the output is set such that the measured L\*
> values are scaled and offset into the range 70 to 100. For the relative
> colorimetric (A2B1) tag, the L\* values are scaled and offset into the range
> 30 to 70. For the saturation (A2B2) tag, they are scaled to the range 0 to
> 30."*

That is a **published vendor statement about a named file, transcribed with its
source, containing no implementation's output** — §1's definition of
`ground-truth`, satisfied for the first time in this crate.

**What it is ground truth ABOUT, and the limit matters more than the claim.**
It is ground truth about **rendering-intent tag selection** and about **the
lightness band a tag's output lies in**. Nobody measured a patch; there is no
published `L*a*b*` triple anywhere in it. A row here can catch iccce selecting
`B2A1` when asked for `B2A0`, evaluating the wrong element order, mis-decoding
PCSLAB, or transposing an ink — each of which a cross-check against lcms2 can
only catch if lcms2 happens not to share it. **It cannot certify that any
number is the right colour, and no row here says otherwise.**

**Kinds, and why they are not uniform.** Only `Probev2_ICCv4.icc` carries
`ground-truth`: it is the file the readme names. The readme describes
`Probev1_ICCv4.icc` only as *"the previous probe profile"* whose purpose the
v2 profile's is *"similar to … with the addition of optional tags based on the
MultiProcessingElement tag type"*, so applying the v2 table to a v1 file is a
**reading of that sentence**. The `Probev1_*` rows are graded — a reading that
cannot fail is not a reading — as **`derived-expectation`**, and a red one is
ambiguous between *"iccce is wrong"* and *"the reading is wrong"*. That
ambiguity is the whole reason they are not ground truth.

★★★ **AND THE PUBLISHED CLAIM IS FALSE OF THE FILE THE DOCUMENT NAMES.** This
is the most surprising result in the pass and it inverts the expected
arrangement:

- On **`Probev1_ICCv2`** and **`Probev1_ICCv4`**, which the readme does *not*
  describe, the "pure cyan / magenta / yellow" design is realised **exactly** —
  off-colorant channels are `0.0` to the bit, `a*`/`b*` change the answer by
  `3.3×10⁻¹⁶` (the `f64` arithmetic of the interpolation weights, eleven orders
  below one 16-bit device code), the ramp is monotone with **zero** violations,
  `L* 0 → 1.000000` (maximum coverage) and `L* 100 → 0.003890` (unmarked
  media). lcms2 agrees: `50.1945 %` against iccce's `50.193793 %` at `L* 50`.
- On **`Probev2_ICCv4`**, the file the readme names, the `BToA` tags produce a
  **near-neutral CMYK build with the intent's colorant raised** — at
  `L* 50, a*=b*=0` the perceptual tag returns `(0.645, 0.619, 0.619, 0.023)`,
  the media-relative tag `(0.630, 0.652, 0.630, 0.030)`, the saturation tag
  `(0.645, 0.645, 0.681, 0.041)`. Two chromatic channels equal, the readme's
  colorant third and highest. Off-colorant maximum `0.9969`; `a*`/`b*` move the
  answer by up to `0.9177`.

**What was done about it, and what was deliberately not done.** Once a
published premise is shown false, continuing to grade iccce against it grades
iccce against the document's error. The three rows that depend on the *strict*
form of the sentence are therefore emitted on `Probev2_ICCv4` as **REPORTED —
tolerance infinity, claiming nothing** — each carrying a loud prefix in its own
detail text (`★★★ THE PUBLISHED CLAIM IS FALSE OF THIS FILE, AND THIS ROW'S
'PASS' MEANS ONLY THAT ITS TOLERANCE IS INFINITE …`) so a green line cannot be
quoted as confirmation.

★★ **They are relaxed to infinity, not to a finite number the observation
happens to satisfy.** A bound of `0.98` chosen because the measurement came out
at `0.9969` would be exactly the tuning §0 exists to prevent, and it would read
in a report as a claim. Infinity reads as what it is.

**What survives, and IS graded on all three files, is the weaker statement the
sentence still entails:** *the published colorant is strictly the largest of
the three chromatic channels.* That row (`b2a/the-published-colorant-dominates-at-every-point`,
tolerance 0, observed 0 everywhere) is the one that catches an intent-to-tag
mis-wiring, and §3.8.6 shows by injection that it is the *only* in-process row
that does.

**The A2B side behaves the same way**: no profile meets the published band
(`2.26`–`3.00 L*` of excursion, at heavy-ink corners of the device cube that
are outside any real press gamut and are extrapolated in the table), **and
lcms2 reproduces the excursion to `8.8×10⁻⁴ L*`** — so it is a statement about
the artefact, not about either engine. Containment is therefore REPORTED. What
is **graded** is the property the readme's three bands actually support and
which no quantisation argument can erode: **the three realised bands are
disjoint and in the published order.** Overlap `0` on all three files; the
tightest gap on `Probev2_ICCv4` is `+1.03 L*`.

**Two further caveats the readme itself supplies, both load-bearing.** (a) It
states the profile is deliberately non-compliant — *"the media relative
colorimetric intent tags are not based on real measurement data, as is required
for v4 profiles"* — which is a defect of **content**, not of encoding, so it
does not touch anything §D grades; but a green §D must not be quoted as
evidence that iccce handles a *conformant* profile correctly. (b) Its own
colour-code table contains transcription defects (the `B2A2` block is
duplicated; the prose describes `B2D0/1/2` where the table says `D2A0/1/2`; one
paragraph names `A2B1`/`A2B2` while describing `D2B1`/`D2B2`). **§D grades only
the two prose paragraphs quoted above.** The table is not used.

#### 3.8.3 ★★ A bound that was written, failed, and was WITHDRAWN rather than widened

`SEVEN_CORNER = 5×10⁻⁵` was derived for the end-to-end seven-channel device
comparison: at a device-cube corner every interpolation scheme returns the
stored node, so the (unlegislated, A16) method difference is identically zero,
and what remains is lcms2's quantisation — CLUT input rounded to `u16`
(`7.63×10⁻⁶`), CLUT evaluated in s15.16 (`1.53×10⁻⁵`), *"the destination's
16-bit reverse tone curve"* (`1.53×10⁻⁵`), `transicc`'s 4-decimal print in
0..255 (`1.96×10⁻⁷`) — summing to `3.82×10⁻⁵`.

**It failed at `1.191176×10⁻⁴`, 2.4× over.** §0's procedure in order:

1. **Is the code wrong?** No. Re-run on the **PCS** side, where the destination
   is not in the loop at all, the same 128 corners agree to **`4.900435×10⁻⁵`
   `L*`** — 40× inside `ORACLE_LAB`. The disagreement was never in the
   seven-channel path.
2. **Is the expectation wrong?** There is none; both sides are computed.
3. **Is the fixture wrong?** No.
4. **The derivation was wrong.** The destination chosen for the row,
   `sRGB2014.icc`, carries **1024-entry tabulated `curv` TRCs**, and lcms2
   inverts a tabulated curve through a **4096-entry reverse tone curve**
   (`cmsgamma.c`) — the same term Pass 4b measured at `9.68×10⁻⁵` and used to
   collapse a residual 457×. The line *"the destination's 16-bit reverse tone
   curve, 1.53×10⁻⁵"* silently **assumed an analytic inverse** and is simply
   the wrong term for this destination.

**The response was not to widen `5×10⁻⁵` until the observation fitted.** The
graded claim moved to the PCS side, where the subject of the row — a
seven-channel `mAB ` tag — is isolated, and the end-to-end device numbers are
REPORTED with the missing term named. Widening would have produced a green line
whose justification still did not mention the biggest thing in it. The withdrawn
constant and its derivation are preserved as a comment in `passh.rs` so the
change is auditable.

★ **This is the third instance of the same failure in this document** — Pass
4b's `DEVICE_B2A`/`B6`, Pass G's `SWEEP_DEVICE` applied in the wrong direction,
and now this. **The generalisation: when a tolerance's `why` contains a clause
about a component the row does not own — a destination, a direction, a fixture
property — that clause is where the missing term will be.**

#### 3.8.4 ★★★ The row that went RED, the defect it found, and why one row became four

##### 3.8.4.1 What was measured on 2026-08-17 at tip `e21154c` — HISTORICAL, DATED, NOT LIVE

`iccce bench --src <the 7CLR profile> --dst <sRGB2014>` **aborted the process**.
Bare exit status **`-1073740791` = `0xC0000409`** (`STATUS_STACK_BUFFER_OVERRUN`,
which is what Rust's `__fastfail` raises on an allocator abort); stderr
*"memory allocation of 1022842631448 bytes failed"*; stdout empty.

The arithmetic, computed at run time by the row and not typed:
`iccce_cmm::compiled::recommended_grid_points(7)` returned **33** — its
`_ => 33` catch-all, whose doc comment reasoned only about 3-D and 4-D — so
`CompiledTransform::new` sampled `33⁷ = 42 618 442 977` nodes × 3 outputs × 8
bytes = **1 022 842 631 448 bytes ≈ 952.6 GiB**. The `checked_pow` guard the
constructor documented protects against **wrap**, not against **size**, so the
allocation was attempted.

**The tolerance is 0 on an indicator whose definition is: the bare exit status
must be `0` (it worked) or `1` (a NAMED refusal).** That is the shipped CLI's
own contract — every other decline in this product is a named refusal on stderr
(`--bpc` outside the estimation subset, `lut8` with an XYZ PCS, iccMAX). **A
process abort is neither a result nor a refusal; a caller cannot distinguish it
from a crash in their own code.**

★ **There was no number to move, and that is why this is worth re-reading.** The
observable was a bare exit status. The pass went red, stayed red, and went green
when the *code* changed — which is the entire proposition §0 asserts and this is
the first time in this document it has been demonstrated on shipped code rather
than on an injected defect.

★ **The corpus found this. Nothing synthetic would have**: `tools/gen-profiles`
has never produced a device space with more than four channels, because nobody
thought to.

##### 3.8.4.2 What fixed it — two changes in `crates/iccce-cmm/src/compiled.rs`

Made by `icc-engineer`, verified here by `icc-conformance` running the shipped
binary rather than reading the diff.

1. **A SIZE guard, distinct from the OVERFLOW guard.** New
   `ChainError::GridExceedsBudget { nodes, bytes, budget_bytes }`, bounded by a
   new public `iccce_cmm::compiled::MAX_COMPILED_GRID_BYTES = 64 MiB`.
   `ChainError::GridTooLarge` stays, and stays meaning the true `checked_pow`
   overflow case. ★ **The two are not merged, and should not be**: one says *this
   node count is not a number*, the other says *this node count is a perfectly
   ordinary number and cannot be allocated*. On a 64-bit machine the second is
   nearly every interesting case and the first is almost none of them.
2. **`recommended_grid_points` no longer has a `_ => 33` catch-all.** 1–2 stay
   129; **3 and 4 stay the measured 33**; ≥5 is **computed** as the largest grid
   that fits the budget at the worst output width ICC.1 permits (15, `FCLR`,
   Table 19). Yields `5→14, 6→9, 7→6, 8→5, 9→4, 10–12→3, 13–15→2`.

**Verified end to end** [`icc-conformance`, 2026-08-17, this machine]:

| command | bare exit | result |
|---|---|---|
| `iccce bench --src <7CLR> --dst sRGB2014.icc --pixels 10000` | **0** | grid **6**, **279 936** nodes, `6 718 464` bytes (6.407 MiB), build 3.56 s |
| the same with `--grid 33` | **1** | stdout **empty**; stderr *"compiled grid would need 42618442977 nodes = 1022842631448 bytes, over iccce's 67108864-byte budget; refused rather than allowed to abort the process. Pass a smaller --grid, or use the reference path (`transform`), which has no grid at all"* |

##### 3.8.4.3 ★★★ Why ONE row stopped being enough the moment the defect was fixed

This is the part worth carrying to other passes.

The original row observed *"did the bare exit fall outside {0, 1}?"*. **Each of
the two fixes above independently makes that observation zero.** Change (2)
alone — the smaller default — means the allocation on this file is 6.4 MiB and
succeeds whether or not the guard in change (1) exists. So:

> **Deleting `MAX_COMPILED_GRID_BYTES` entirely would leave the original row
> GREEN.**

A row that went red on a real defect can become a row that cannot see that
defect's return, without anybody editing it and without any number moving. The
Pass H question — *not "what does this row measure" but "which layer is in the
loop"* — is the one that catches it. Four rows, four layers:

| row | what is in the loop | what its going red would mean |
|---|---|---|
| `compiled-path-does-not-ABORT-the-process` | the shipped binary at the **default** grid | the default became unsurvivable again |
| `default-grid-BUILDS-and-is-the-grid-the-library-RECOMMENDS` | the binary **and** `recommended_grid_points` | the default refuses rather than builds, **or** the library's recommendation and the binary's behaviour have drifted apart |
| `oversized-grid-is-a-NAMED-refusal` | **the size guard itself**, through the CLI | the guard was removed, or stopped naming its numbers, or stopped exiting 1, or let something escape on stdout |
| `compiled-vs-reference-at-the-default-grid` | nothing — **REPORTED** | (never; it is not graded) |

★ **`oversized-grid-is-a-NAMED-refusal` is not redundant with the engineer's own
unit test.** `compiled::tests::oversized_grid_arithmetic_is_refused_not_aborted`
asserts the guard's **arithmetic**, in process, deliberately without attempting
the allocation — *"a test that aborts the test process proves nothing and takes
its siblings with it"*, which is right. But an in-process test is blind to the
CLI wiring: whether `bench` propagates the `Err` as exit 1, whether the message
reaches stderr, whether anything partial escapes on stdout. **This is the same
finding Pass H's injection I2 produced** — seven of §D's eight per-profile rows
evaluated a tag *by signature in process* and were blind to a *wiring* defect by
construction. The unit test and the row are the same claim at two layers, and
both are needed.

★ **The three numbers stderr must name are computed by the row, never typed.**
`33⁷`, `× 3 × 8`, and `MAX_COMPILED_GRID_BYTES` are all read from the library at
run time and matched as substrings. If the guard's arithmetic changes, the row
tracks it; if the message stops naming one of them, the row goes red. A sixth
violation counter fires if the budget is ever raised above this allocation —
**a row that has quietly become vacuous is worse than a row that fails**, because
it reports PASS.

##### 3.8.4.4 ★★ The measured 33 was NOT weakened to fit the budget, and the tension is asserted

`33⁴ × 15 × 8` is ~136 MiB, so the **measured** 4-channel 33 does *not* fit
`MAX_COMPILED_GRID_BYTES` at the worst-case output width. It was not shrunk.

- **A measured value is not weakened to satisfy a memory bound.** The 33 for 3-D
  and 4-D is gated on Pass 4's iccce-vs-lcms2 agreement on a real profile pair
  (§3.6; 17 was rejected because it failed that gate by 17 %). Shrinking it to
  fit a byte budget would discard evidence in favour of convenience.
- **The budget still protects the process**, because the guard in
  `CompiledTransform::new` uses the **actual** output width, not the worst case.
  CMYK → RGB at 33 is ~27 MiB and builds. Only CMYK → 15-channel exceeds, and
  that is then a named refusal, which is all the budget was ever for.

So worst-case sizing applies where there is no measurement to protect, and the
measurement wins where there is one. ★ **The tension is asserted in a test that
fails if it ever disappears**, so the doc cannot go stale claiming a conflict
that no longer exists. `icc-conformance` grades that as the right call: the
failure mode of a documented exception is that the exception is silently removed
and the paragraph explaining it survives.

##### 3.8.4.5 ★★ Why the ≥5-channel grids are REPORTED and not GRADED — `icc-conformance`'s judgement

`compiled-vs-reference-at-the-default-grid` observes **2.952005×10⁻³** in
normalised device units at grid 6 over 527 off-node probes. It is **REPORTED,
permanently**, and the request to consider grading it is declined. Four reasons,
in descending order of how hard they are to remove:

1. **Both arms are iccce.** It is self-comparison — `NUMERIC_CLAIMS.md` §1's
   weakest class, *"worthless as correctness evidence"*. No bound derived from it
   could distinguish a correct engine from a consistently wrong one. This reason
   alone is dispositive and does not depend on the other three.
2. **The gate that made 33 defensible does not exist at seven inputs.** The 3-D
   and 4-D 33 is gated on a *measured iccce-vs-lcms2 agreement*. At 7 inputs
   there is no equivalent: **ICC.1 legislates no interpolation method (A16)**, and
   **lcms2's >4-input CLUT geometry has not been read out of the pinned source**.
   The two end-to-end 7-channel rows against lcms2 are themselves REPORTED for
   exactly this reason.
3. **n = 1.** `APTEC_CMYKOGV_Coated_LinearCTV_2025.icc` is the only >4-channel
   profile this project has ever been given. A bound fitted to it would be a
   population of one — the shape Pass G's `SWEEP_DEVICE` withdrawal warned about.
4. **The number is not stable under the thing it would be quoted about.** 2.95×10⁻³
   is at grid 6 on one destination; the grid is now a *computed* function of a
   memory budget, so a change to `MAX_COMPILED_GRID_BYTES` moves it with no
   colour reasoning involved at all.

**What is graded instead is structure**, and that is not a compromise — it is
the correct partition given the evidence available. Exit codes, refusal wording,
stdout emptiness and recommendation-vs-behaviour agreement are indicator counts
with tolerance exactly **0**: no instrument error, nothing to absorb, and each of
them catches a real regression.

★ **What would unblock a graded colour row at seven inputs**, stated so the
decision is reversible rather than permanent: (a) lcms2's n>4 interpolation
geometry read out of pin `21c582a` and modelled, as Pass 4 did for the 4-D
hybrid; **and** (b) at least a second 7-channel profile, so the bound is not
fitted to one file. Until both, the honest report is a number with no claim
attached to it.

#### 3.8.5 ★★★ Two conformant CMMs, two different colours, from one file — and the Probe profile was built to show it

`Probev2_ICCv4.icc` carries `D2B0/1/2` and `B2D0/1/2` (`multiProcessElements`)
**in addition to** its `A2B*`/`B2A*` tags. **ICC.1:2022 clause 8.10.2**,
verbatim:

> a) Use the BToD0Tag, BToD1Tag, BToD2Tag, BToD3Tag, DToB0Tag, DToB1Tag,
> DToB2Tag, or DToB3Tag designated for the rendering intent if the tag is
> present, **except where this tag is not needed or supported by the CMM** (if a
> particular processing element within the tag is not supported the tag is not
> supported).
> b) Use the BToA0Tag, BToA1Tag, BToA2Tag, AToB0Tag, AToB1Tag, or AToB2Tag
> designated for the rendering intent if present, when the tag in a) is not
> used.

(Transcribed in
`D:\Dev\Rag-Specialized\ICC_Spec\icc\icc__s__required_tags.md` §5.)

iccce does not implement `multiProcessElementsType` — the six tags are present
and decode to `TagData::Unknown`, which the row
`tags/mpet-is-present-and-NOT-decoded` grades at **0 decoded**. The tag is
therefore *"not supported by the CMM"*, step (a)'s own proviso applies, and
iccce proceeds to step (b). **That is conformant.** lcms2 supports `mpet` and
takes step (a). **That is also conformant.**

**Measured size of the divergence: `33.133830 L*`** at unmarked media
(`CMYK 0,0,0,0`) at the perceptual intent — iccce returns **`99.000534`** from
`A2B0`, lcms2 returns **`65.866700`** from `D2B0`.
In the `BToA` direction the same divergence is
visible as pure colour: asked for `L* 50` at perceptual, iccce returns cyan
`(0.645, 0.619, 0.619, 0.023)` from `B2A0` and lcms2 returns **red**
`(0, 0.754, 0.754, 0)` from `B2D0` — and **the readme's own colour code
identifies which tag each engine used**: *"The B2D0 tag … renders the L\* values
as tints of pure red (a combination of magenta plus yellow). The B2D1 tag …
pure green … B2D2 … pure blue."* lcms2's three intents return red, green and
blue exactly.

★★ **The profile was built to make this visible and it did.** Its stated
purpose is *"to enable visual determination of the rendering intent **and
processing element type** used"*. This is the cleanest possible demonstration
of the standard's own designed-in divergence (ambiguity **A33**).

**Consequences for this pass, all of them stated rather than absorbed:**

- The row `a2b/vs-lcms2-through-the-same-tags` **SKIPs** on `Probev2_ICCv4`
  with that reason. There is no second reading of the *same* tag to compare
  against, and a comparison of two *different* tags would be a number with no
  meaning.
- The divergence itself is a **REPORTED** row. No clause requires the two to
  agree, so there is nothing for a bound to mean (the §3.5.9.6 rule).
- ★ **What is NOT settled, and is owed to the engineer:** iccce takes step (b)
  **silently**. Neither `inspect` nor `transform` tells a caller that an
  author-preferred transform was present and declined. Clause 8.10.2 permits
  declining; it does not require silence, and `CLAUDE.md` rule 6 is about
  exactly that kind of undisclosed substitution. **A 33 `L*` difference that a
  caller cannot see coming is a disclosure defect even where the selection is
  conformant.**
- ★★ **STATUS 2026-08-17 (later): agreed, not yet fixed, and the gate is
  pre-registered.** `icc-engineer` accepts it as a disclosure defect and has
  **deliberately not implemented it in this session** — it changes a public
  surface and the operator has not seen it. `icc-conformance` has therefore
  **specified the graded row in advance, before the implementation exists**, so
  the target cannot be fitted to whatever gets built: full text in
  `tools/difftest/README.md` §23.5. In outline —
  `passh/D/probe-v2-icc-v4/tags/8102-fallback-is-DISCLOSED`, indicator count,
  tolerance **exactly 0**, four counters: a disclosure exists; it **names the
  declined tag** by signature; it **cites clause 8.10.2**, which is what makes it
  a conformant decline rather than a defect report; and **the control** — the
  same disclosure must *not* appear on a profile with no `mpet` tags. ★ **The
  control is the row.** The first three are satisfied by printing a string; an
  unconditional notice discloses nothing. ■ The row will **not** claim iccce
  should take step (a) — it should not — and
  `tags/mpet-selection-divergence-from-lcms2` stays REPORTED at `33.13 L*`
  either way.

#### 3.8.6 Proof of power — three injections, each in a detached worktree

Baseline reproduced first (`pass=270 fail=1 skip=9`), each injection reverted
before the next, `git worktree remove --force` at the end.

| # | injected defect | predicted | measured |
|---|---|---|---|
| **I1** | `crates/iccce-profile/src/lib.rs`: the `version_raw >> 24 >= 5` gate made unreachable | §A red, §B's cross-check red, everything else green | **exactly that.** `pass=265 fail=6`. A1 failed at **10**, its stated separation `1.000000×10¹`; the control row A3 failed at **4**, its stated separation `4.000000×10⁰`. **Two separations predicted their own failure magnitude to the digit.** §B's acceptance rows, §C and all of §D stayed green |
| **I2** | `crates/iccce-cmm/src/transform.rs`: the destination intent→`B2A` map rotated by one (perceptual→`B2A1`, media-relative→`B2A2`, saturation→`B2A0`) | §D's shipped rows red | **`pass=238 fail=33`.** All three `shipped/intent-selects-the-published-colorant` rows failed at **exactly 81**, their stated separation `8.100000×10¹` (27 source points × 3 intents). Pass 4b/4c/5c/G's `B2A`-direction rows also went red, correctly |
| **I3** | `crates/iccce-cmm/src/lut_ab.rs`: `decode_v4_pcs` given the **legacy** 16-bit Lab scale instead of clause 10.13's | `ORACLE_LAB`'s stated sensitivity, `0.39 L*` at white | **`pass=252 fail=19`.** `passh/C/7clr/pcs-corners-vs-lcms2` failed at **`3.906250×10⁻¹`** = `100 × (65535/65280 − 1)` **exactly**, and `probe-v1-icc-v4/a2b/vs-lcms2-through-the-same-tags` at `3.843075×10⁻¹`. ★ The `probe-v1-icc-v2` rows stayed **green** — that file's tags are `mft2` and go through `Lut16Model`, not `decode_v4_pcs`, so the suite localised the injection to the v4 files |

★★★ **Five separations predicted the magnitude of their own injected failure to
the digit** (10, 4, and 81 three times), and a sixth reproduced a tolerance's
stated sensitivity claim exactly. DL-033's purpose, discharged.

★★ **The most useful thing I2 found is what did NOT go red.** Seven of §D's
eight per-profile rows are **blind** to an intent-to-tag mis-wiring, because
they evaluate a tag *by signature* in process — they test the tag, not the
wiring. **Only `shipped/intent-selects-the-published-colorant` sees it**, which
is why that row exists and why it drives the CLI rather than the library. *An
in-process library test cannot see a CLI-to-`Chain` mis-wiring.*

#### 3.8.7 Two named honesty limits in this pass

1. **`passh/B/acceptance/no-malformation-is-disclosed-on-any-accepted-file` is
   the weakest-argued row here, and its own `why` says so.** Its expectation —
   *a profile published as conformant contains nothing for a conformant parser
   to disclose* — is not derived from any implementation, and it is also not
   guaranteed by anything. **A non-zero result is an adjudication between two
   hypotheses (iccce over-reports / a published profile is defective) that this
   corpus cannot settle**, because `transicc` emits no malformation list to
   compare against. It is graded at 0 anyway, with the instruction attached that
   a non-zero is a finding to adjudicate and **never** answered by widening.
2. **The version gate's rival reading is untestable on this corpus.**
   `harness-reading-of-byte-8-predicts-iccce-on-every-file` prints
   **ZERO-SEPARATION**, correctly: the rival is *"the gate compares the whole
   4-byte version word against `0x05000000` rather than the major byte alone"*,
   and every iccMAX file here encodes exactly `0x05000000` (minor and bugfix
   zero), so the two readings are indistinguishable on all 50 files. **A v5.1
   profile would be needed and the corpus has none.** That is a coverage
   statement the mechanism produced without anybody asking for it.

#### 3.8.8 Coverage — what Pass H does NOT establish

- **No colour value.** Structurally impossible on this corpus (DL-041). Every
  ΔE-shaped number in §3.8.1 is an `L*` or device difference between two
  implementations or against a published *band*, never against a published
  *value*.
- **No `GRAY`, `Lab` or `XYZ` colour-space coverage — IN THIS CORPUS.** ★ The
  accepted population declares **`CMYK (23)`, `RGB (16)` and `7CLR (1)` only**.
  `D50_XYZ.icc`, `D55_XYZ.icc` and `D65_XYZ.icc` declare
  `colourSpace = 'RGB '`, not `'XYZ '`. Any claim of `GRAY`/`Lab`/`XYZ`
  coverage **from this corpus** is false, and the §E row says so in the report.
  ★★ **The denominator is now named in the row itself, because a second census
  exists and reads like a rival claim.** Across **both** private corpora —
  `color-org` (40 accepted) plus `ghent-v50` (20, Pass G) — the engineer's sweep
  gives **`CMYK 33, RGB 25, GRAY 1, 7CLR 1 = 60`** (`NUMERIC_CLAIMS.md`
  **NC-220**). **The two reconcile exactly**: `23+16+1 = 40` here, `10+9+1 = 20`
  there. **The single `GRAY` profile is in `ghent-v50`, not here.** There is no
  contradiction, only two populations — but a coverage number quoted without its
  corpus is not a coverage number, and the §E row now carries its own
  denominator so the pair cannot be read as a disagreement. **iccce does have
  `GRAY` evidence; Pass H is not where it lives.**
- **No ΔE claim of any kind for the compiled path above four input channels.**
  `compiled-vs-reference-at-the-default-grid` reports `2.952005×10⁻³` in device
  units at grid 6, and that is **self-comparison** — both arms are iccce. The
  ≥5-channel grid recommendations are a **memory** result, not an accuracy one;
  §3.8.4.5 states what would be needed to make one of them a graded colour claim
  and why neither condition is met today.
- **No `6CLR` evidence of any kind.** The corpus's only six-channel file,
  `SixChanCameraRef.icc`, is iccMAX and is refused at the version gate. Nothing
  about six channels may be inferred from the seven-channel rows: a `7CLR`
  `mAB ` and a `6CLR` tag share no code path this pass exercised.
- **No `namedColor2` behaviour.** Both `nmcl` files in the corpus
  (`NamedColor.icc`, `FluorescentNamedColor.icc`) are iccMAX and refused.
- **The CMYK print profiles are parsed, not transformed.** `CGATS21_CRPC1/3`,
  `GRACoL2006/2013`, `SWOP2006/2013`, `PSOuncoated_v3_FOGRA52`,
  `PSOsc-b_paper_v3_FOGRA54`, `SC_paper_eci`, `SNAP2007`, the two
  `Coated_Fogra39L_VIGC_*`, the two `Uncoated_Fogra47L_VIGC_*` and the five
  `APTEC_*` are covered by §B's acceptance rows and by nothing else. **No
  differential colour row exists for any of them**, and `NEXT_SESSION.md`'s
  queue item 6 is therefore **not** discharged by this pass.
- **One machine, one toolchain, one day.** Windows/MSVC, 2026-08-17, lcms2 pin
  `21c582a` (2.19.1).
- **`mpet` is not implemented and is not graded as colour anywhere.** §D
  measures the *consequence* of not implementing it; it does not evaluate a
  single `mpet` element.

---

### 3.9 ★★★ Pass I — ICC's **published** chromatic-adaptation matrix

**Filed 2026-08-17.** Apparatus `tools/difftest/src/passi.rs`, instrument
`tools/difftest/src/bin/passi_probe.rs`. **19 rows, `pass=19 fail=0 skip=0
error=0`, and none of them can skip** — Pass I invokes no oracle, reads no
profile, resolves no fixture and consults no environment variable. It is the
only section of §3 that grades the same rows on a bare CI machine as on the
operator's, which is the posture a ground-truth row should have: *a
ground-truth-shaped row must not be hostage to an oracle.*

**Subject:** `iccce_color::adaptation_matrix(&BRADFORD, D65, D50)` against the
nine cells ICC prints at fifteen decimal places in

> ICC, *How to interpret the sRGB color space (specified in IEC 61966-2-1) for
> ICC profiles*, Jack Holm, **2015-04-27**, **§B.2**. Corpus
> `ICC_Spec/icc/icc__s__srgb_for_icc_profiles.md`; PDF
> `ICC_Spec/_sources/srgb_bt709/srgb_icc_specification_of_srgb_2015.pdf`,
> obtained by the operator 2026-08-17.

This is the document `ICC.1:2022` Annex E.4.2 points at, and which this project
recorded as **not obtained** until that date. Pass I is the repository's
**third `published-ground-truth` subject** (after NC-001/Sharma and the sRGB
colorants) and the **first for chromatic adaptation** — the error class
`RAG_PLAN.md` names as the canonical from-memory mistake.

#### 3.9.0 ★★★ What these rows do NOT claim, stated before what they do

**`ICC.1` mandates no chromatic-adaptation transform at all** (corpus **A29**;
**NA-002**). A profile's `chad` stores a *resulting matrix*, not a method, so
Bradford is iccce **policy**. Every row in this section grades exactly one
sentence:

> *iccce's Bradford-derived D65→D50 matrix agrees with the matrix ICC
> recommends, to the extent that the two constructions' published inputs
> entail.*

It does **not** say *"iccce's chromatic adaptation is correct"*. No
specification text exists against which that sentence could be graded, and a
reader who quotes these rows for it has been handed the wrong number. Pass I
also does **not** discharge NA-002, whose cost is Bradford *against another
CAT* and whose alternatives remain unsourceable.

#### 3.9.1 ★★★ The bound was derived before the pass was run — and the derivation it was commissioned with was incomplete

This pass was dispatched with a bound to derive from the **cone-matrix**
difference alone, propagated through `M_A⁻¹ · D · M_A`. That term is real:

- **ICC.1:2022 Annex E.3 Eq. (E.1) prints Bradford `M_A[0][0] = 0,8951`**, and
  that is what `iccce_color::BRADFORD` carries, because it is what the
  specification prints.
- **ICC's published `chad` was computed with `0,8950`** — recovered by
  `icc-spec-librarian` by eigendecomposition (a von-Kries matrix has the rows of
  `M_A` as its left eigenvectors) and confirmed by exact reconstruction:
  `0,8951` leaves `5,661×10⁻⁶`, `0,8950` leaves `5,7×10⁻¹⁶`. The distinguishing
  digit had been in the corpus for six days as a sanity-check footnote — E.1's
  first row sums to `1,0001`, the recovered one to exactly `1,0000`.

Isolated exactly, that term is **`5,661 341 564 633 735×10⁻⁶`**. **A bound
derived from it would have failed this pass at 7,4× its value**, because of a
second difference the brief did not contain:

- **ICC's `chad` adapts a ROUNDED white.** `chad⁻¹ · D50` returns
  `(0,9505, 1,0000, 1,0890)` exactly — §A.4's `76,04/80` and `87,12/80`.
  **iccce's D65 comes from BT.709-6 item 1.4's chromaticities** `(0,3127,
  0,3290)`, giving `(0,950 455 927…, 1, 1,089 057 751…)`. The two whites differ
  by `−4,407×10⁻⁵` in X and `+5,775×10⁻⁵` in Z, and that propagates to
  **`4,453 187 573 657 197×10⁻⁵`** in the matrix — **7,9× the cone term.**

The two terms **partially cancel**, and the exact prediction for iccce as
shipped is **`4,164 936 613 631 601×10⁻⁵`** at cell `(0,0)` — `2,730` ULP of
`s15Fixed16`. Every §B bound is that prediction, **per cell**, plus one
numerical allowance.

> **★★ The generalisation, and it is the third instance of one failure shape in
> this document** (§3.4's `B6`, §3.7.2's `SWEEP_DEVICE`, §3.8.3's
> `SEVEN_CORNER`): *when a tolerance's derivation names only the components the
> row owns, the missing term is in a component it does not own.* Here the
> derivation named the **cone matrix**, which the row is *about*, and omitted
> the **white point**, which the row merely *uses*. All four instances were
> found the same way — by writing the derivation down before running.

#### 3.9.2 The one numerical allowance, `F64_NOISE = 1×10⁻¹²`

Every bound in §3.9 is `an exactly-derived prediction + F64_NOISE`, so this
constant is the only place a failing row here could be made to pass and it has
to stand on its own.

**Derivation.** The compared computation is one 3×3 adjugate inverse (nine 2×2
minors, one determinant, nine divisions), two 3×3 products (three-term dot
products) and three cone-ratio divisions, all at magnitudes of order 1 with no
leading-digit cancellation (largest intermediate `1,7135`). A conservative worst
case is ≈50 ulp of that magnitude: `50 × 2,220×10⁻¹⁶ × 1,72 ≈ 1,9×10⁻¹⁴`.
`1×10⁻¹²` is **50× that headroom**.

**Why it cannot mask what the section exists to detect.** The smallest defect
any row here is designed to see is the cone-cell substitution, worth
`5,663×10⁻⁶` — **5,7×10⁶ times the allowance**. There is no value of this
constant between `10⁻¹⁴` and `10⁻⁸` that changes any verdict in this section,
which is the property a numerical allowance should have and a tuned tolerance
never does.

**What would justify moving it:** a measured f64 residual above `10⁻¹³` on any
platform — which would be *a finding about floating-point accumulation, recorded
as one*, not a licence to widen. Observed values are emitted in every record's
detail so this can be checked rather than assumed. Measured 2026-08-17 on
Windows/MSVC: `4,44×10⁻¹⁶`, `2,67×10⁻¹⁷`, `2,82×10⁻¹⁶`, and **exactly `0`** —
four orders below the allowance at worst.

#### 3.9.3 The rows

| # | Row | Kind | Metric | Tolerance | Why that number | Observed 2026-08-17 |
|---|---|---|---|---|---|---|
| **A1** | `passi/A/harness-CAT-reproduces-ICC-published-chad-from-Bradford-0.8950` | **ground-truth** | abs-max-component | **1×10⁻¹²** | **The instrument check, and a ground-truth row in its own right.** The harness's own CAT — its own typed digits, its own adjugate inverse, **no iccce code in the loop** — is given ICC's own inputs (`M_A[0][0] = 0,8950`, source `0,9505/1/1,0890`, destination `0,9642/1/0,8249`) and must return the nine cells ICC printed. Exact reconstruction leaves `5,668×10⁻¹⁶`, the fifteen-decimal print floor; the rest is §3.9.2. **If A1 is ever red, nothing else in §3.9 means anything** — which is why it is graded first and at the tightest bound. | **`4,440 892×10⁻¹⁶`** — the print floor, and an independent second-route confirmation of the `0,8950` finding, in a second language from an independent transcription |
| **A2** | `passi/A/E.3-Bradford-does-NOT-reproduce-the-published-chad` | derived-expectation | abs-max-component | **1×10⁻¹²** | Graded quantity is `|measured − exact prediction|`, not a colour difference: `5,661 341 564 633 735×10⁻⁶` is exact rational arithmetic over published constants, so the only admissible discrepancy is round-off. **Stops A1 from being a tautology** — it demonstrates the harness can tell the two variants apart. | **`2,672 050×10⁻¹⁷`** |
| **A3** | `passi/A/typed-exact-predictions-still-hold` | derived-expectation | abs-max-component | **1×10⁻¹²** | **The stale-constant guard.** §B's per-cell bounds are typed exact values; the harness recomputes all nine in f64 from the published inputs and grades the agreement, so an edited published digit or a superseded derivation **fails loudly here** instead of quietly re-basing every bound in the section. | **`2,819 671×10⁻¹⁶`** |
| **B1–B9** | `passi/B/chad-cell-r{i}c{j}` | **ground-truth** | abs-max-component | **the cell's own exact prediction + 1×10⁻¹²** (`4,164 937×10⁻⁵` … `2,692 510×10⁻⁷`) | **A prediction, not an observation.** Each residual is the sum of the two terms in §3.9.1, both computable in exact rational arithmetic from published constants alone — which is why these nine numbers could be written down before the pass was first run. **ONE-SIDED BY CONSTRUCTION**, and that is stated on every row: a change moving iccce *toward* ICC's own construction passes silently here. §C is the two-sided gate. | **every cell exactly at its prediction**; worst `4,164 937×10⁻⁵` = `2,730` ULP |
| **B10** | `passi/B/chad-max-over-nine-cells` | **ground-truth** | abs-max-component | **`4,164 936 713 631 601×10⁻⁵`** (max prediction + allowance) | The headline number for the pass and the one to quote; the per-cell rows are what make it a test rather than a summary. | **`4,164 937×10⁻⁵`** = **`2,730` ULP of `s15Fixed16`** |
| **C** | `passi/C/iccce-matches-the-independent-prediction-two-sided` | derived-expectation | abs-max-component | **1×10⁻¹²** | **The regression gate, and the row with power in BOTH directions.** Two f64 implementations of one construction over identical published inputs may differ only by round-off. It catches a corrupted `BRADFORD` digit, a corrupted `D65_XY` or `D50`, an inverted operand order (`M_A · D · M_A⁻¹`) or a transposition — every one of which yields a matrix that still looks like an adaptation matrix. | **exactly `0`** — see the honesty limit in §3.9.6 |
| **D1** | `passi/D/two-ICC-publications-print-different-Bradford-matrices` | ground-truth | abs-max-component | **∞ — REPORTED** | **There is no clause under which one of two ICC publications is the wrong one.** Annex E is informative and ICC.1 mandates no CAT (A29), so neither value is required of anybody; iccce follows the printed specification. Grading this would mean *this project* deciding which ICC document is authoritative, which is not a decision a conformance suite is entitled to make. | `5,661 342×10⁻⁶` = `0,371` ULP |
| **D2** | `passi/D/encoded-chad-cells-differing-from-ICC-published` | ground-truth | indicator-count | **∞ — REPORTED** | A count of encoding differences is not a requirement: iccce writes no `chad` today and no clause requires a profile's `chad` to equal ICC's recommended one. Emitted because it is the unit a profile author cares about — and because it **corrects an inference this project made in writing** (§3.9.5). | **6 of 9**, largest **3 LSB** |
| **E1** | `passi/E/shipped-srgb-colorants-vs-ICC-published` | **ground-truth** | abs-max-component | **`4,607 402×10⁻⁵` + allowance** (`3,020` ULP) | The exactly-derived worst-cell residual of the **shipped** construction — `chad(0,8951, chromaticity D65) × rgb_to_xyz(BT.709-6 primaries)` — against ICC's printed colorants. It matches the figure `builtin.rs` declares as the model's one named approximation, **and until this row nothing in the repository measured it**: grepped 2026-08-17, the digits `0,436 030…` appear in **no** source file under `crates/`, so the doc comment's *"asserted in the tests"* was not true. | **`4,607 402×10⁻⁵`** = **`3,020` ULP** |
| **E2** | `passi/E/colorant-residual-attribution` | ground-truth | abs-max-component | **∞ — REPORTED** | The subject is an **attribution** — which of two inputs a known residual came from — and no clause and no published value grades an attribution. The number it carries is graded by E1. | `3,787 988×10⁻⁵`; see §3.9.4 |
| **E3** | `passi/E/published-colorant-rows-sum-to-D50` | ground-truth | abs-max-component | **1×10⁻⁷** | **A transcription guard on the REFERENCE DATA, not on iccce**, and the size of the permitted miss is derivable rather than observable: ICC's colorants are `chad × inv(§A.7)`, §A.7 is printed to **seven** decimals, so the implied white of `inv(§A.7)` sits `1,060 763×10⁻⁷` above `1,0890` in Z, and the published `chad` carries that to `7,946 512×10⁻⁸` in the row sums — **exact arithmetic, closing to every printed digit.** The bound is the next power of ten above it, stated in the unit of §A.7's own print precision. Discrimination: one mistyped digit in the third decimal of any published cell moves this by ~`10⁻³`, four orders above the bound. | **`7,946 512×10⁻⁸`** |

#### 3.9.4 ★★ A finding for the engineer: the built-in sRGB has TWO named approximations, not one

`crates/iccce-cmm/src/builtin.rs` states that the `3,02` ULP colorant residual
*"is entirely accounted for by **which D65 matrix each side starts from**"* —
ICC inverting its own §A.7 matrix as printed to 7 decimals, iccce building it
exactly from BT.709-6's chromaticities. **Exact decomposition, 2026-08-17:**

```text
iccce − ICC  =  (chad_iccce − chad_ICC) · M_d65        [the CHAD term]
             +   chad_ICC · (M_d65 − inv(§A.7))        [the PRIMARIES term]
```

| term | worst cell | on `bXYZ.Z` |
|---|---|---|
| **chad term** | **`2,482` ULP** | **`−2,482` ULP** |
| **primaries term** | **`2,480` ULP** | **`+1,586` ULP** |
| total (graded by E1) | `3,020` ULP | `−0,897` ULP |

The two terms are **the same size**, and the word *"entirely"* is false. Worse
for the doc comment's argument: on `bXYZ.Z` — the cell that section is
specifically about — the small total (`−0,897` ULP, presented as evidence the
construction is close) is a **cancellation between two errors five times its
size**, one of which is the Bradford variant the sentence does not mention. The
`3,02`/`11,13` comparison against the shipped HP file is unaffected and remains
correct; what is wrong is the attribution of the remainder to a single cause.
**Registered as NA-010.**

#### 3.9.5 ★ Sub-ULP does not mean identical bytes — a corollary corrected

`icc__s__srgb_for_icc_profiles.md` records that a `chad` recomputed from E.3
differs from ICC's by `0,371` ULP and concludes *"That is below one encoding
step, so **the written tag bytes are identical** and nothing observable
changes."* Measured, in exact arithmetic:

- for that very case (`0,8951` at ICC's own rounded white, `0,371` ULP),
  **3 of 9 cells still encode to a different `s15Fixed16` word**;
- for **iccce as shipped**, `2,730` ULP, **6 of 9 cells differ, largest 3 LSB**.

A sub-ULP difference near a half-ULP rounding boundary still flips the LSB.
*Below one ULP* bounds the encoding difference at **one** LSB; it does not make
it zero. The consequence is narrow but real: if iccce ever writes a `chad` tag
for its built-in sRGB, **it will not byte-match ICC's recommended one**, and no
clause requires it to.

#### 3.9.6 Proof of power — three injections, each in a detached worktree

Per `README.md` §21 and DL-018, the arm is proven by breaking it, not by
argument. Each defect was injected into a detached worktree at `aece12b`, the
pass re-run, the worktree destroyed.

| injected defect | predicted | **measured** |
|---|---|---|
| **`BRADFORD[0][0]` `0,8951` → `0,8950`** (adopt ICC's own cone cell) | B fails on 6 of 9 cells + B10 at `4,453 158×10⁻⁵`; C fails at `5,662 962×10⁻⁶`; E1 fails at `4,686 594×10⁻⁵`; §A untouched | **exactly that — `pass=10 fail=9`, every figure to the digit** |
| **operand order `M_A · D · M_A⁻¹`** (the classic transposition of this construction) | all of §B, C and E1 fail by orders | **`pass=7 fail=12`**, worst cell `9,724 514×10⁻²` — 2 300× its bound |
| **`D65_XY` → CIE's 5-figure `0,312 72 / 0,329 03`** (the "precision upgrade" trap `illuminant.rs` warns about) | §C fails; §B partially | **`pass=10 fail=9`** — and **three §B cells PASSED because the substitution moved them TOWARD ICC** (`r0c0` `2,443×10⁻⁵` against a `4,165×10⁻⁵` bound). **C failed at `1,784×10⁻⁴`, eight orders over.** |

★★ **The third injection is the argument for §C existing.** It is the same
shape as `builtin.rs`'s `constructed_colorant_sum_is_d50` finding and as
`NEXT_SESSION.md` §5.2: *a one-sided test has no power against an error that
moves your answer toward the thing you are comparing to.* §B is the
ground-truth claim and is one-sided; §C is the gate. Quoting §B as the
regression protection would be a mistake this table exists to prevent.

★ **What the injections did NOT move:** §A (all three rows) and §D on every
run. That is correct and designed — §A grades the *harness* against published
digits and contains no iccce code, and §D is REPORTED. **A row that never moves
under any injection of the subject is either an instrument row or a dead one,
and the difference must be stated**: A1–A3 are instrument rows, and they would
go red on a mistyped published constant in `passi.rs`, which is the defect they
exist for.

#### 3.9.7 The `BLIND` flags in §B, and why they are not being tuned away

Ten §B rows report `BLIND`: the candidate distance (`5,662 962×10⁻⁶`, iccce vs
the `0,8950` rival) is smaller than the bound (up to `4,164 937×10⁻⁵`), so the
mechanism's distance test declines to claim discriminating power. **That verdict
is conservative and correct as a distance test, and it understates these rows**
— because the observation sits *exactly at* its bound by construction, the rival
breaches **6 of the 9** per-cell bounds anyway (worst exceedance `1,621×`),
which the injection above confirms.

**Nothing was adjusted to remove the flag.** The honest reading is recorded on
the rows themselves: §B's power against the cone cell is real but is an artefact
of where the observation sits, and the row that carries that power without
needing an argument is **§C**, where the same separation is `DISCRIMINATING` by
`5,66×10⁶`. This is the same shape as §1.1's finding that the row named
`estimators/black-points-in-lab` was `UNGRADED` while the suite's power lived
elsewhere: **the row whose name matches the finding is often not the row that
would catch its regression.**

#### 3.9.8 Coverage of this section, stated

- **One illuminant pair** (D65→D50), **one cone matrix family** (Bradford),
  **one direction**. Nothing here says anything about D50→D65, about any other
  illuminant pair, or about any CAT other than Bradford.
- **No `chad` tag is parsed anywhere in this pass.** The subject is a *computed*
  matrix. A profile that carries a `chad` takes an entirely different code path
  (`iccce-profile`), and Pass I has **zero** power over it.
- **In-process library calls, not the shipped binary.** §A–§D call
  `iccce_color::adaptation_matrix` directly; §E calls
  `iccce_cmm::builtin::srgb()`. **A wiring defect between the `iccce`
  executable and either is invisible to every row in this file** — the Pass H
  lesson (*ask which layer is in the loop*), applied in advance. §E is the layer
  closest to the product that this subject can reach without an oracle.
- **No ΔE anywhere.** Every number is an XYZ-space matrix-cell difference. The
  perceptual cost of `4,16×10⁻⁵` in these cells is **not measured by this pass**
  and must not be inferred from it.
- **One machine, one toolchain, one day.** Windows/MSVC, 2026-08-17, at
  `aece12b`.

---

### 3.10 ★★★ Pass K — **black preservation**, measured before the feature exists

**`tools/difftest/src/passk.rs`; instrument `src/bin/passk_probe.rs`;
README §23. 33 rows filed 2026-08-17 at tip `506fcd3` (suite
`pass=325 fail=1 skip=9 error=0`); **§F added the same day brought it to 40
rows** and the suite to `pass=331 fail=2 skip=9 error=0`, both failures
deliberate and both the SAME predicate on two profiles.

> ### ★★★ 2026-08-18 — THE FEATURE LANDED AND NO TOLERANCE MOVED
>
> `crates/iccce-cmm/src/black_preserve.rs` implements
> `KMapping::EqualLightness`, reachable as
> `iccce transform --preserve-black <policy>`. §E and §F were repointed at that
> surface, **44 rows**, suite **`pass=337 fail=0 skip=9 error=0`**. Graded in
> §3.10.12, which is where a reader should start.
>
> **The two deliberate reds are green and the two bounds are unchanged** —
> both were and are exactly `0`. Nothing in §3.10 was widened, softened or
> re-derived to accommodate the feature; **four rows were added** because the
> feature made four questions askable that had no answer before: E7, E8, E9
> and F8.

`crates/` contained **no black-preservation code** at tip `506fcd3`. That was
the premise of the section, not a defect in it: the numbers below were chosen
before anybody could see which numbers would be convenient. The capability
being anticipated is the one the Ghent Output Suite's *Four different Grays*
patch demands.

**★★★ ICC.1 IS SILENT, and the negative is CLOSED rather than pending.**
`icc-spec-librarian` searched **ICC.1:2022 and ICC.1:2001-04 whole-document,
two engines each** for `black.?preserv`, `preserve.*black`, `GCR`,
`gr[ae]y component`, `K.only`: **zero hits in both**. Corpus file
`ICC_Spec/icc/icc__ref__black_preservation.md`, register row **A51**. Two
**v2-only** sentences carry the entire ICC story and v4 deleted both — §6.4.45's
`ucrbgTag` *"provides descriptive information only and is not involved in the
processing model"* (**ICC's only black construct disclaims itself**) and
§6.3.3.1's *"the output values are the control values and not the "K" (black)
values"*. **There is no clause to grade against and no published ΔE**: Cholewo
(2000) prints visual figures only, and lcms2 computes the ΔE of its own
approximation and discards it (`// Error estimation (for debug only)`, and it
is ΔE\*ab, not ΔE2000). If a normative text is ever found, §3.10 gains
ground-truth rows and **none of the numbers below move — the `kind` column
changes.**

**★★★ TWO different things are called "black preservation", and they sit on
opposite sides of the project boundary.**

| name | rule | layer |
|---|---|---|
| **K-only preservation** (lcms2 intents 10–12; Cholewo 2000) | a pixel that is already K-only stays K-only | a **CMYK → CMYK** rule — **iccce's** |
| *"gray maps onto K alone"* | `c = m = y = 0`, `k = 1 − gray` | a **PDF device-space** rule — **`pdfce`'s** |

§A, §D and §E measure the first. §C measures the *distance to* the second and
grades nothing about it. ★ And inside the first there are **two definitions
under one name**: **lcms2 maps K by equal `L*` on the K ramp; Cholewo maps it
by the `K_MIN`/`K_MAX` ratio.** Which one iccce implements must be stated before
`E2`'s number means anything (§3.10.6).

#### 3.10.0 ★★★ THE FINDING THAT DETERMINES EVERY TOLERANCE IN THIS SECTION

> **ΔE is blind to the defect black preservation exists to fix.**

`ISO Coated v2 300% (ECI)` — the profile the Ghent suite embeds as the
`DestOutputProfile` of every ICC-CMS patch — converting the K-only ramp
`(0,0,0,K)` into **itself** at media-relative colorimetric, 41 points:

| quantity | measured |
|---|---|
| max chromatic ink where the input had none | **`7.053 20×10⁻¹`** (cyan, at `K = 1.0`) |
| max total area coverage | **`2.753 549`** — 275 %, from an input TAC that cannot exceed 100 % |
| max reduction of the black channel itself | **`3.608 89×10⁻¹`** (at `K = 0.60`) |
| max ΔE2000 between that build and the K-only build | **`1.360 90×10⁻¹`** |
| max disagreement with lcms2 on the same points | **`6.3×10⁻⁵`** device |

Read together those five say something none of them says alone:

- **The engine is not wrong.** It agrees with the pinned oracle to `6.3×10⁻⁵`.
  The separation is the destination profile's own `B2A1` table doing exactly
  what its author built it to do, and the output stays **inside** the 300 % the
  profile's name declares. Nothing here is non-conformant anywhere.
- **The colour is preserved almost perfectly** — `0.136` ΔE2000, an *eighth* of
  §2's perceptibility anchor.
- **The defect is a device-space fact**: three plates of ink under what the
  document said was a single-plate black, costing registration, moiré, text
  edge definition and press stability — none of which is a colour difference
  and none of which any ΔE can see.

**Consequence, and it is the whole tolerance policy of this section: every
graded row about preservation is in NORMALISED DEVICE UNITS.** The two ΔE rows
exist to *price* the colour cost of a preservation decision, never to detect
whether preservation happened. Row `A4` grades the contaminated build against
§2's `1.0 ΔE2000` anchor and **passes**, and its passing is the finding: a
conformance suite that graded this subject in ΔE would report nothing at all.

`A4` therefore carries its separation in **`SepUnits::Other`** and prints
`INCOMMENSURATE` — the same discipline Pass 6 row R5 and Pass 5c's
`ATTRIBUTION` row record from the other direction. The device figure is emitted
beside the ΔE so a reader sees both, and no ratio is computed across the two.

#### 3.10.1 The rows

`REPORTED` = tolerance `∞`. Three circumstances, all named on the row: a
**baseline**, which no requirement yet constrains; an expectation that is
lcms2's **black-ink tone curve**, a vendor construction with no normative text
behind it (the A27/A42 posture §3.7 takes for BPC); or a distance to **PDF
32000-1 §10.3.3**, which belongs to the PDF consumer and not to this project.

| id | kind | metric | tolerance | why the number is that number | observed |
|---|---|---|---|---|---|
| **A1** | cross-check | device-abs-max | **∞ — REPORTED** | the baseline. Nothing yet requires anything of it | `7.053 20×10⁻¹` |
| **A2** | cross-check | device-abs-max | **∞ — REPORTED** | ink cost, the reason the requirement exists, has no colour units | `2.753 549` |
| **A3** | cross-check | device-abs-max | **∞ — REPORTED** | as A1 | `3.608 89×10⁻¹` |
| **A4** | cross-check | dE2000-max | **`1.0`** | §2's perceptibility anchor, **used for its inverse** — see §3.10.0 | `1.360 90×10⁻¹` |
| **A5** | cross-check | device-abs-max | **`1.24×10⁻⁴`, computed at run time** | §3.10.2 | `6.3×10⁻⁵` |
| **A6–A8** | cross-check | device-abs-max | **∞ — REPORTED** | the same ramp at the other three ICC intents | `7.061 55×10⁻¹`, `3.599 6×10⁻²`, `7.053 20×10⁻¹` |
| **B1** | cross-check | indicator-count | **`5`** | §3.10.3 — a refutation row over a population of 6 | `2` |
| **B2–B7** | cross-check | device-abs-max | **∞ — REPORTED** | per-destination baselines | `3.6×10⁻²` … `7.83×10⁻¹` |
| **C1–C4** | cross-check | device-abs-max / dE2000-max | **∞ — REPORTED** | the distance to a rule this project does not own | `7.163 86×10⁻¹`, `7.516 16×10⁻¹`, `8.534 95×10⁻¹`, `1.259 583×10¹` |
| **C5** | cross-check | indicator-count | **`1`** | §3.10.3 — a refutation row over a population of 2 | `1` |
| **D1** | oracle-reproducibility | device-abs-max | **`2 × 2⁻¹⁶ = 3.0518×10⁻⁵`** | §3.10.4 | `1.259 375×10⁻⁵` |
| **D2** | oracle-reproducibility | device-abs-max | **`0` exactly** | at `C = 1/16` the sample is a CLUT **node** whose K-only corner carries weight zero: the two answers are the *same table entry* | `0` |
| **D3** | oracle-reproducibility | device-abs-max | **`0` exactly** | §3.10.5 — this row is what `EXACT_ZERO` is calibrated against | `0` |
| **D4–D7** | oracle-reproducibility | device-abs-max | **∞ — REPORTED** | lcms2's `_cmsBuildKToneCurve` is a vendor construction; A27/A42 | `6.1×10⁻⁵`, `1.165×10⁻³`, `1.4296×10⁻²`, `4.8899×10⁻²` |
| **E1** | cross-check | device-abs-max | **`0` exactly** | §3.10.5. Red by design until 2026-08-18; **the bound never moved** | `7.053 20×10⁻¹` → **`0`** |
| **E2** | cross-check | device-abs-max | **∞ — REPORTED for ever** | §3.10.6 — and the reason is now **measured**, not argued: on this pair the row is `BLIND` | `3.608 89×10⁻¹` → `6.1×10⁻⁵` (whole ramp); `6.1×10⁻⁵` at the oracle's own nodes |
| **E3** | cross-check | device-abs-max | **∞ — REPORTED** | §3.10.12.3 — a real behavioural difference from lcms2, not a missing feature | `0` (and the `C = 0` endpoint is now K-only, which is the opposite of what `0` meant before) |
| **E4** | cross-check | device-abs-max | **`2.00×10⁻⁴`, computed at run time** | §3.10.2; repointed at the **preserving** surface 2026-08-18 (§3.10.12.2) | `5.4×10⁻⁵` |
| **E5** | cross-check | device-abs-max | **∞ — REPORTED** | the **control** that earns E4's tightness | `1.750×10⁻³` |
| **E6** | cross-check | device-abs-max | **∞ — REPORTED** | §3.10.7 | `0` |
| **E7** | **self-consistency** | device-abs-max | **`0` exactly** | §3.10.12.2 — a branch is taken or it is not | `0` |
| **E8** | **derived-expectation** | device-abs-max | **`1×10⁻⁶`** (one printed unit) | §3.10.12.4 — the expectation is ALGEBRA; **lcms2 is `6.1×10⁻⁵` wrong here and iccce is right** | `0` |
| **E9** | cross-check | device-abs-max | **`1.09×10⁻⁴`, computed at run time** | §3.10.12.5 — the only row that can say WHICH definition iccce implements | `3.1×10⁻⁵` (rival `4.890×10⁻²`, `1577×`) |
| **F1** | **derived-expectation** | device-abs-max | **`0` exactly** | §3.10.11 — nine node values are one number or they are not | `0` |
| **F2** | **derived-expectation** | device-abs-max | **`0.5 × 2⁻¹⁶ = 7.629 511×10⁻⁶`** | §3.10.11 — round-to-nearest, and the worst case is **attained** | `7.629 511×10⁻⁶` |
| **F3** | **derived-expectation** | device-abs-max | **`0` shortfall against a floor of `4×10⁻²`** | §3.10.11 — the floor is `10×` §3.7's `SWEEP_DEVICE`, declared in advance | `0` (separation `4.207 049×10⁻¹`) |
| **F4** | **derived-expectation** | device-abs-max | **`1 × 2⁻¹⁶ = 1.525 902×10⁻⁵`** | §3.10.11 — counted quanta; the interpolation scheme contributes **zero** | `4.965 520×10⁻⁷` |
| **F5** | **derived-expectation** | device-abs-max | **`0` exactly** | §3.10.5. Red by design **in CI** until 2026-08-18; **the bound never moved** | `4.207 050×10⁻¹` → **`0`** |
| **F6** | **derived-expectation** | device-abs-max | **∞ — REPORTED** | as E3 (§3.10.12.3) | `0` (and the `C = 0` endpoint is now K-only) |
| **F7** | cross-check | device-abs-max | **`2 × 2⁻¹⁶ = 3.051 804×10⁻⁵`** | §3.10.11 — the **third reading**, with lcms2's `cmsPipelineEval16` in the counting; repointed at the preserving surface 2026-08-18 | `1.400 000×10⁻⁵` |
| **F8** | **self-consistency** | device-abs-max | **`0` exactly** | §3.10.12.2 — the leak guard that runs **in CI**, and the only §F row that includes the K channel | `0` |

**Separation coverage: 44 of 44 rows state one; `unstated = 0`, `blind = 0`.**

★★ **`E2` IS THE ROW A READER SHOULD LOOK AT ANYWAY, AND THE EMITTED VERDICT
UNDERSTATES IT.** Its separation distance (`6.1×10⁻⁵`) **equals its
observation** — ratio `1.0`, which is the definition of a blind row. The
classifier prints **`UNGRADED`** rather than `BLIND` because `BLIND` is only
reachable for a row that carries a finite tolerance, and `E2`'s is infinite by
the decision in §3.10.6. So the suite-level tally is honest and *insufficient*:
the number that says this row cannot discriminate is on the row, not in the
tally. **`E9` exists because a cross-press pair is not blind** (§3.10.12.5).

★ That distinction was got wrong in this document's first draft of this
paragraph, on 2026-08-18, and the emitted report is what corrected it — the
same discipline §3.5.8.6 imposes on numerals, applied to a **noun**.

★ **§F's eight rows are `derived-expectation` except `F7` and `F8`, and the
exceptions are the point.** A derived expectation is defeated when *the derivation* shares a
misreading with the fixture, and here both are this project's reading of clause
10.10. `F7` is a third party that read the same clause independently, over the
same probe set. `Kind::DerivedExpectation`'s own documentation asks for exactly
that pairing.

#### 3.10.2 ★★ Why A5 and E4 can be two orders tighter than §3.7's `SWEEP_DEVICE`

Pass 4 measured the CLUT interpolation-method envelope (**NA-006**) at up to
`1.57 ΔE2000` on a CMYK `A2B`, and §3.7's `SWEEP_DEVICE` had to be `4×10⁻³`
because of it. A5 and E4 are bounded near `10⁻⁴`. That is **structural, not
observational**:

- **The K-only ramp lies on an EDGE of the 4-D `A2B` CLUT.** With
  `C = M = Y = 0` *exactly*, quadrilinear, Sakamoto tetrahedral and lcms2's
  `Eval4Inputs` hybrid all degenerate to the **same** 1-D linear interpolation
  along K, because every one of these decompositions agrees on the edges of the
  hypercube by construction. The envelope is **identically zero on this ramp**.
  A probe set built with `10⁻⁹` of cyan "to avoid a boundary" would silently
  destroy that property and the bound with it.
- **The `B2A` leg is trilinear on both sides.** lcms2's `_cmsReadOutputLUT`
  forces trilinear for a Lab-PCS output LUT (§3.4.4's finding 2, and
  `SWEEP_DEVICE`'s own premise) and `iccce_cmm::clut` is n-linear with
  tetrahedral deliberately absent (NA-006). Envelope **identically zero**
  again.
- **E4's off-neutral points are `A2B` CLUT NODES** (`j/15`, grid 16), so that
  leg does no interpolation at all.

What is left is the **16-bit PCS quantum** — lcms2 carries Lab between the two
tables in 16 bits and iccce in `f64` — whose device cost is the destination
`B2A`'s own slope. **Both rows measure it at run time**
(`passk::pcs_quantum_sensitivity`) by perturbing `L*`/`a*`/`b*` by one quantum
(`100/65535`, `255/65535`) *at the PCS points the probe set actually reaches*,
and add `2×10⁻⁶` for the two print floors (`iccce transform`: six decimals in
`0..1`; `transicc`: four decimals in `0..100`).

> **The bound is therefore a FUNCTION of the fixture, not a constant** — §3.7.2
> lesson 1 — it prints its own premise on the line, and it cannot go stale
> (DL-034).

**★ `E5` is the control that earns this, and without it the tightness would be
indistinguishable from luck.** The *same* comparison over 96 points that are
**not** node-aligned observes `1.750×10⁻³` — **32×** the node-aligned figure.
That difference *is* NA-006, made visible in the same section that excludes it.

★ Note what the oracle is doing in the sensitivity measurement and what it is
not: it is a **ruler on the fixture**. Baseline and perturbed evaluation come
from the same `transicc` invocation shape, so what is measured is the *table's
slope*. A defect in lcms2's `B2A` evaluation would **inflate** the bound rather
than bias it, and inflating a bound weakens a gate rather than inventing a
failure — which is why the row prints the sensitivity, the bound and the
observation, so the margin (`1.97×` on A5, `3.7×` on E4) is on the line.

#### 3.10.3 ★★ The `refutation row`, and the two shortcuts it kills

[`Record::graded`] compares `observed <= tolerance`, which cannot express *"at
least one counterexample exists"*. A refutation row therefore observes **the
number of corpus members for which a shortcut HOLDS** and bounds it **one below
the population size**. The bound comes from the logic — *a shortcut is only
sound if it holds for all of them* — never from the observation, and the row
fails in exactly the circumstance that would make the shortcut defensible. A
count has no instrument error, so the bound is an integer.

**B1 — *"use the saturation intent instead of building black preservation"*.**
Print profiles often build the saturation `B2A` with heavy GCR, so the shortcut
is plausible. Measured over six real CMYK destinations, max chromatic ink on
the K-only ramp:

| destination | media-relative | perceptual | saturation |
|---|---|---|---|
| `ISO Coated v2 300% (ECI)` | `0.705320` | `0.706155` | **`0.035996`** |
| `ISO Coated v2 (ECI)` | `0.773954` | `0.776090` | **`0.038759`** |
| `Coated FOGRA39` | `0.726101` | `0.730096` | `0.730096` |
| `Coated FOGRA27` | `0.756552` | `0.759334` | `0.759334` |
| `GWG_GenericCMYK` | `0.791232` | `0.783291` | `0.783291` |
| `GWG_ICC_v4_testprofile` (X-Rite) | `0.501787` | `0.531900` | `0.506564` |

**Two of six, and both are the same vendor's.** Three of the six alias
`B2A0 ≡ B2A2`, so their "saturation" answer *is* their perceptual answer —
§3.7's vendor-specific intent-aliasing finding, reappearing as the reason a
shortcut fails. ★ **A suite that measured this subject on ISO Coated v2 alone
would have concluded the feature was unnecessary.** And where the shortcut does
work it is not free: the ECI saturation build sits up to **`6.4151 ΔE2000`**
from the K-only build.

The `5e-2` threshold that decides "already K-only" is **not a colour
tolerance** and must not be quoted as one. It separates `0.0360`/`0.0388` from
`0.5066`/`0.7301`/`0.7593`/`0.7833` — an order of magnitude of clear air in
each direction, so no value between `0.04` and `0.5` changes the count. It is
stated because a threshold must be stated, not because it was fitted.

**C5 — *"iccce's ICC leg and ISO 32000-1 §10.3.3's device rule are
interchangeable"*.** See §3.10.9.

★★ **Neither refutation row states a candidate separation, and that is a
correction made on the first run.** The alternative that looks like one —
*"the corpus had contained only the favourable member"* — changes the
**population**, not the reading of a fixed observation. Stating it made both
rows report `BLIND` for a property they do not have. It is a **coverage**
statement (§3.10.10). The rule generalises §3.5.8's: *a rival TOLERANCE is not
a rival candidate, and a rival CORPUS is not one either.*

#### 3.10.4 What lcms2's K-only preservation actually is, and why D1's bound is `2 × 2⁻¹⁶`

Read out of the pin (`vendor/lcms2/src/cmscnvrt.c` at `21c582a`),
`BlackPreservingKOnlyIntents` samples

```text
if (In[0] == 0 && In[1] == 0 && In[2] == 0) {
    Out[0] = Out[1] = Out[2] = 0;
    Out[3] = cmsEvalToneCurve16(bp->KTone, In[3]);
    return TRUE;
}
bp->cmyk2cmyk->Eval16Fn(In, Out, bp->cmyk2cmyk->Data);
```

over a grid of `_cmsReasonableGridpointsByColorspace(cmsSigCmykData, 0)` points,
which `cmspcs.c` returns as **17** for a 4-channel space. Three consequences,
all graded:

1. **Preservation is an EDGE of a table, not a rule about neutrality.** The
   K-only answer occupies the `C=M=Y=0` edge of a 17-node hypercube.
2. **The blend to the ordinary answer is exactly one cell wide, `1/16`.**
   Over 33 samples inside the first cell the observed answer matches the
   **linear blend of the cell's two endpoints** to `1.259 375×10⁻⁵` (D1), and
   at `C = 1/16` the two answers are **bit-identical** (D2, tolerance `0`).
   **An implementation that snapped near-neutrals to K-only, or that jumped
   discontinuously off the axis, would differ from this measurably** — which is
   what makes `E3` a usable probe once there is a transition to measure.
3. **K is RE-MAPPED, not copied.** `_cmsBuildKToneCurve(…, 4096, …)` builds a
   curve from the *source's* K-only lightness ramp against the *destination's*.
   Across four pairs the re-mapping is `6.1×10⁻⁵` (same profile), `1.165×10⁻³`
   (→ FOGRA39), `1.4296×10⁻²` (→ FOGRA27) and **`4.8899×10⁻²`**
   (→ `GWG_GenericCMYK`). That last figure is D7's **named candidate
   separation**: *"K is copied through unchanged"* is the plausible-but-wrong
   implementation, and on a same-press pair it sits only `6.1×10⁻⁵` away —
   **so the cross-press pairs, not the same-profile one, are where that rival
   is discriminated.**

**D1's bound is `2 × 2⁻¹⁶ = 3.0518×10⁻⁵`, from the encoding's own precision.**
The model is a linear blend of two **16-bit CLUT outputs** (up to half a
quantum of encoding error each), requantised to 16 bits once more. Nothing
perceptual enters it and §2's anchor is irrelevant to it. D1's separation is
the **measured** endpoint distance (`4.008 09×10⁻¹`), so a rival grid size —
`cmsFLAGS_HIGHRESPRECALC` gives 23, and a caller may pack a size into the
flags' high bits — is separated by four orders.

#### 3.10.5 ★★★ Two tolerances of exactly zero, and one of them is RED on purpose

**"K-only" is the statement that three channels carry the encoded value zero,
and a predicate about zero has no instrument error.** D3 measures that lcms2's
own K-only intent returns `0.000000` in all three chromatic channels at **every**
point of the 41-point ramp. That is what makes `EXACT_ZERO` defensible rather
than merely strict: a real implementation of this requirement writes the
encoded zero, not something small, so **any bound above zero would be an
allowance for ink the requirement forbids** — not for noise.

**`E1` — `passk/E/k-only-in-implies-k-only-out` — was therefore RED, at
`7.053 20×10⁻¹` against a required `0`.** It was red because the feature did
not exist. **The remedy is the feature, not the number**, and this document
records no widening of it; the precedent is §3.6.1 and §3.8.4.

> ★★★ **DISCHARGED 2026-08-18 IN THE ONLY ACCEPTABLE WAY.** The observation is
> `0.000000`; the tolerance is still exactly `0`, the same constant
> (`EXACT_ZERO`) with the same `why` string it carried when it was written
> against a capability that did not exist. `F5`, the committed-fixture twin,
> went from `4.207 050×10⁻¹` to `0.000000` on the same commit and **in CI**.
> A reader auditing this should check the tolerance column and the constant,
> not the verdict: the verdict is the cheap half.

Two properties of E1 that must travel with it:

- **Its candidate separation is taken from lcms2's COLORIMETRIC answer
  (`7.053 18×10⁻¹`), not from iccce's observation.** Using the observation would
  be `Separation::against`'s trap (§3.5.8.6): the distance would collapse to
  exactly zero on the day the row goes green, the mechanism disclaiming its
  power on the one run that demonstrates it. Taken from the oracle it is a
  property of the **destination table** and stays put.
- **★ It SKIPs in CI, permanently.** The red is visible to whoever implements
  black preservation and invisible to CI. That is an honest consequence of the
  corpus's licence and it is recorded as a coverage gap in §3.10.10, not as a
  convenience.

#### 3.10.6 ★ One tolerance this section declines to derive, and says so

**`E2` — whether iccce's preserved K *value* matches lcms2's — is REPORTED for
ever.** The K value a preserving path should emit is `_cmsBuildKToneCurve`'s
construction: a **vendor choice with no normative text behind it**, exactly the
A27/A42 posture §3.7 takes for BPC. Gating iccce against it would gate a choice
no standard makes.

This is the honest answer to *"state the tolerance or say it is undetermined"*:
here it is **undetermined and unobtainable**, not undetermined and pending. The
number is still printed, and its named rival — *"K is copied through"* — is
still computed, so a future reader can see how far the two policies sit apart
before choosing one.

> ★★★ **2026-08-18 — THE ARGUMENT FOR LEAVING `E2` REPORTED IS NOW A
> MEASUREMENT, AND IT IS STRONGER THAN THE ARGUMENT WAS.**
>
> The pre-feature reason was provenance: lcms2's `_cmsBuildKToneCurve` is a
> vendor construction, so gating against it gates a choice no standard makes.
> That remains true. But the row is also **structurally incapable** of the
> gate, and this is the number that says so:
>
> | on `ISO Coated v2 300% (ECI)` → **itself** | value |
> |---|---|
> | observed `\|K_iccce − K_lcms2\|` over the 41-point ramp | `6.1×10⁻⁵` |
> | the named rival *"copy K through"*, measured from the oracle | `6.1×10⁻⁵` |
> | separation ratio | **`1.0` → `BLIND`** |
>
> **On a same-press pair the two published definitions coincide.** Any bound
> `E2` could carry that iccce passes, the rival passes too. Grading it would
> have produced a green row that discriminates nothing — the exact failure
> §3.5.8 exists to name, arriving through the tolerance column instead of
> through the separation column.
>
> ★★ **A second measurement changed what `E2`'s number MEANS.** Splitting the
> ramp by whether the `K` value lands on a node of lcms2's own 17-node
> black-preserving CLUT (`K = m/16`):
>
> | pair | at the oracle's nodes | off them | ratio |
> |---|---|---|---|
> | → `GWG_GenericCMYK` | `3.1×10⁻⁵` | `1.089 5×10⁻²` | **`351×`** |
> | → unnamed corpus CMYK (`bbdf…`) | `1.4×10⁻⁵` | `2.942×10⁻³` | `210×` |
> | → `Coated FOGRA27` | `2.3×10⁻⁵` | `2.751×10⁻³` | `120×` |
> | → `ISO Coated v2 (ECI)` 350 % sibling | `2.4×10⁻⁵` | `3.377×10⁻³` | `141×` |
>
> **Off its own nodes lcms2 is interpolating its table, not evaluating its
> construction.** A whole-ramp figure between the two implementations is
> therefore a measurement of lcms2's grid density, and a row graded on it would
> be gating iccce against a vendor's choice of `17`. That is why `E9` grades
> **only at the nodes**, and it is the same node/off-node structure `E5`
> measures at `32×` in a different channel — see §3.10.12.5.

#### 3.10.7 ★★ Why the committed synthetic corpus cannot grade this subject

`fixtures/synthetic/v2-cmyk-mft2-lab.icc`'s `B2A0` is built by `gen-profiles`'
`lab_to_cmyk_clut`, which emits `[0, 0, 0, k]` at every node. Its K-only ramp
comes back **K-only already**: `E6` observes `0` chromatic ink, and would
observe `0` whether or not black preservation existed. Its two candidate
answers are the same number — **`ZERO-SEPARATION`**, the one state no tolerance
can rescue (§1.1) — and `E6` is emitted precisely so that the fact is a number
in the report rather than a paragraph nobody reads.

**Owed work, and it is the single highest-value item this section leaves
open:** a new committed `gen-profiles` recipe whose `B2A` puts chromatic ink
into neutrals **by construction**, with a grid fine enough to carry a ramp. It
would make `E1`, `E3` and `D3`'s companion predicate runnable in CI on a
fixture whose *expected* K-only answer is derivable from the fixture's own
bytes — i.e. a `Kind::DerivedExpectation` rather than a cross-check.

> ★★★ **DONE, 2026-08-17 — recipe `v2-cmyk-chromatic-neutral`, graded by §F
> (§3.10.11).** The fixture's two candidate answers are **`4.207 049×10⁻¹`**
> apart, measured from its own committed bytes. **`E6` is NOT deleted and its
> `ZERO-SEPARATION` verdict is not tidied away** — it is the measurement that
> says why a second fixture had to exist, and a future reader who points
> something at `v2-cmyk-mft2-lab` needs to find it. Neither is `E1` repointed:
> the Ghent row stays exactly as it was, red and skipping in CI, measuring real
> ink. §F **adds reach; it does not launder the red into green.**

#### 3.10.8 ★★★ The boundary, SETTLED by the librarian — and why §C is kept anyway

`icc-spec-librarian` settled where the four-way equivalence is discharged, and
**it is not here**:

- `DeviceGray → DeviceCMYK` is `shall`-level **PDF** — `c = m = y = 0`,
  **`k = 1.0 − gray`** — **ISO 32000-1 §10.3.3**, ISO 32000-2 §10.4.2.3.
- `Separation /Black` and `DeviceN [/Black]` bind to the K colourant by
  **§8.6.6.4**'s `shall`.
- **So all four "grays" agree inside the PDF processor BEFORE any colour
  conversion happens.** It is `pdfce`'s job, the same boundary class as
  overprint. **iccce owns only CMYK → CMYK and the non-CMYK-native device.**
- ★ **PDF, not ICC, also names the harm**: §8.6.5.7 NOTE 2 (both editions) — a
  4 → 3 → 4 conversion *"results in a loss of fidelity in the black
  component"*. That is the closest thing to a normative statement of why §A's
  baseline matters, and it is in the wrong standard for this project to cite as
  a requirement on itself.

★ **A premise this pass was commissioned with also failed, and it is a
claim-bearing citation.** *"GWG 23.0 (Four different Grays)"* is **not a GWG
requirement id**: GWG 2022 uses `Dxxx`/`Rxxx`, and the four-way equivalence
exists as **`D0013 "Black Colour"`**, a *definition consumed by the overprint
requirements* rather than a rendering requirement. The `n.m` form matches the
**Ghent PDF Output Suite patch** numbering. Every occurrence in `passk.rs`,
this section and README §25 now names the artefact rather than the phantom
requirement.

**§C is kept, with every row REPORTED**, for two reasons:

1. iccce still owns the **non-CMYK-native device** case — a gray *ICC profile*
   into a CMYK destination is iccce's leg whenever a consumer hands it one, and
   nothing in ISO 32000 covers that path.
2. The distance between the two legs is the number a consumer needs in order to
   choose **which** leg to use, and nobody had it.

The result:

> On the corpus's own press-gray profile the two legs land **`0.716 386` apart
> in device space** and **`0.7516 ΔE2000` apart in colour.** They look the same
> and are made of completely different ink.

★★ **That colorimetric agreement is a property of WHICH GRAY PROFILE, and the
corpus contains a counterexample.** `Schwarze Druckfarbe - ISO Coated v2 (ECI)`
is literally *"black printing ink"* — the tone curve of the destination press's
black — so its `g` and the destination's `K` describe the same colour almost by
construction. Substituting `fixtures/synthetic/v2-gray-curv-gamma.icc`, an
ordinary gamma-2.2 gray, moves the same measurement to **`12.5958 ΔE2000`**
(`0.853 495` in device units). `C5` is the refutation row; **a suite carrying
only the favourable fixture would have reported the shortcut as sound.**

#### 3.10.9 Coverage of this section, stated

- **One destination profile carries §A, §C, §D and §E**: `ISO Coated v2 300%
  (ECI)`, v2.4, `mft2` `A2B` grid 16 / `B2A` grid 33. §B adds five more for one
  observable only.
- **§A runs all four ICC intents; §B runs three; §C, §D and §E run
  media-relative only.** No row anywhere in Pass K exercises `--bpc`.
- **Source and destination are the SAME profile in §A, §D and §E.** A
  cross-press re-separation is exercised **only** by §D's three cross-profile
  `KTone` rows. ★ The obvious pair `ISO Coated v2 (ECI) → ISO Coated v2 300%
  (ECI)` is a **separation-direction isolator** — their `A2B1` tags are
  byte-identical and only the `B2A` differs — and for that same reason its
  media-relative output is *bit-identical* to `300% → 300%`, so it adds nothing
  to §A and is not run there.
- **The corpus is LICENSED and uncommittable.** Every row of §A, §B, §C, §D and
  most of §E resolves through `$ICCCE_PRIVATE_FIXTURES` and **SKIPs, with a
  reason, everywhere else — permanently including CI.** A green CI line for
  Pass K says those rows did not run. §3.10.7 names what would close it.
- **Nothing here is proofed, printed or measured with an instrument.** Chromatic
  ink coverage and TAC are *proxies* for the reasons the requirement exists —
  registration, moiré, text sharpness, ink cost — and nothing in a CMM can
  measure those.
- **No ground truth, and none obtainable — established, not assumed** (A51;
  see §3.10 preamble).
- **No ground truth, and none obtainable today.** Every expectation is either an
  implementation cross-check against a **vendor extension outside the ICC intent
  numbering** (lcms2 intents 10–15, reached through `passk::KOnlyOracle`, whose
  `CAVEAT` string is prepended to the `source` of every record built from it) or
  a property of a fixture's own bytes.
- **`Intent` was NOT extended.** The crate's standing promise that it *"cannot
  express a non-ICC rendering intent"* is intact; the non-ICC intents live
  behind a separate type that builds its own argument vector.
- **One machine, one toolchain, one oracle pin, one day.** Windows/MSVC,
  2026-08-17, tip `506fcd3`, lcms2 `21c582a`.
- **NOT proven by injection — §A–§E.** §3.7.6, §3.8.6 and §3.9.6 each proved
  their arms by injecting the defect they watch for; §A–§E have not been through
  that, and for most of their rows the "injection" is the feature itself.
  **Still owed.** ★ **§F HAS been, for two of its three file arms** (§3.10.11),
  by mutating the committed fixture's bytes and restoring them — and the
  injection produced a result stronger than the design argued for: a collapsed
  fixture does not merely fail to inform, it **turns the headline row green**.
  §F's transform arms are not injected, because their rival is a consumer-side
  defect and injecting one means editing `crates/`; their rival is **evaluated**
  from the committed bytes instead.
- **WHICH K-mapping definition iccce will implement is unstated** — lcms2's
  equal-`L*` rule or Cholewo's `K_MIN`/`K_MAX` ratio. Until it is stated, `E2`
  can print a distance but cannot mean one.
- **§5 owes an entry when the feature lands.** A black-preservation path is a
  named approximation by construction — it trades colorimetric accuracy for a
  device-space property — and §3.10.0's `6.4151 ΔE2000` shows the trade is
  visible. `E2`'s posture (REPORTED, no normative text) is the shape that entry
  should take.
- ★★★ **§F changes exactly one of the bullets above and no others.** The
  licensed-corpus bullet now reads: **§A, §B, §C, §D and §E skip in CI
  permanently; §F's seven rows all run there**, on a committed, unlicensed,
  byte-verified fixture, and one of them is **red in CI by design**. §F is a
  statement about the *predicate* and never about a press — the *population*
  gap is untouched, and every number about real ink in this section is still
  unreachable without the corpus. See §3.10.11.

#### 3.10.11 ★★★ §F — the committed fixture on which the predicate has two answers

§3.10.7 recorded the hole and §3.10.9 recorded its consequence: every graded
row about black preservation rested on a **licensed** profile and therefore
**skipped in CI, permanently**, so `E1`'s deliberate red was visible only to
somebody holding a corpus that cannot be committed. This subsection is the
closure and the justification of its four new tolerances.

##### The fixture, and why the construction is the argument

`fixtures/synthetic/v2-cmyk-chromatic-neutral.icc` (recipe
`v2-cmyk-chromatic-neutral`, `tools/gen-profiles/src/recipes.rs`, 10 200 bytes,
byte-verified in CI by `gen-profiles verify`). A v2.4 `prtr` CMYK profile,
legacy PCSLAB, `mft2` `A2B0` on a `5⁴` grid and `mft2` `B2A0` on a `9³` grid.

Its `A2B0` gives black ink a darkness of only `0.70` — **K alone reaches
`L* 30` and no further** — so the remaining darkness of a dark neutral has to
come from a composite `C M Y` gray, exactly as on a real press. Its `B2A0`
supplies it: on the neutral axis the separation is `C = M = Y = 0.60 d` under a
skeleton `K = 0.40 d`, where `d` is the node's **encoded** `L*` darkness. The
round trip `B2A0(A2B0(0,0,0,k))` therefore returns

```
C = M = Y = 0.60 · (1 − (65280/65535)·(1 − 0.70 k))
```

which is **`4.207 049×10⁻¹` at `k = 1`**. A black-preserving consumer returns
`0`. **The two candidate answers are `4.207 049×10⁻¹` apart** — four orders
above any encoding argument that could be mistaken for it, and `0` on the
sibling `v2-cmyk-mft2-lab`.

##### ★★ Three construction choices, and each buys a term of the tolerance

They are not decoration. Each one removes a source of error that would
otherwise have to be *allowed for* in a bound, and a bound that allows for
something it could have eliminated is a bound nobody can defend.

1. **Both models are AFFINE — no cross terms.** Every conformant CLUT
   interpolation returns a convex combination of the cell's corners with
   barycentric weights summing to one, and a convex combination of an affine
   function's corner values *is* that function at the point. So n-linear,
   tetrahedral, prism and lcms2's 4-D hybrid all return the same number, and
   **`NA-006`'s interpolation-method envelope — worth up to `1.57 ΔE2000` on a
   real CMYK `A2B` (§3.10.2), and the term that forced §3.7's `SWEEP_DEVICE` to
   `4×10⁻³` — is identically zero here.** That is what makes a bound of one
   16-bit quantum arguable at all. **It is a property of THIS fixture and must
   not be quoted for another.**

2. **`B2A0` is `a*`/`b*`-INDEPENDENT across a three-node dead band about the
   neutral axis** (indices 3, 4, 5 of 9). The reason is a legacy-PCSLAB detail
   that would otherwise cost the derivation its exactness: `a* = 0` encodes to
   `8000h` = 32 768, while node 4 of a 9-node axis sits at
   `4 × 65 535 / 8` = 32 767,5. **The neutral axis is not a node.** It falls
   `1.5×10⁻⁵` of a cell inside the cell `[4, 5]`, so any `a*` dependence at all
   would hand the neutral axis a small interpolated contribution from node 5 —
   an error that is tiny, real, and impossible to state exactly. With those
   three node lines carrying **one** value, every convex combination of them is
   that value. **`F1` grades that property against the file rather than
   asserting it in prose**, at a tolerance of exactly zero, because nine node
   values either are one number or they are not.

3. **The `B2A0` darkness variable is the ENCODED `L*` fraction, not `L*`.**
   Legacy PCSLAB puts `L* = 100` at `FF00h`, so the axis's top node decodes to
   `L* = 100.390 6` and a model defined on `1 − L*/100` would be negative
   there — clamped, and the clamp would bite **inside the very cell the K
   ramp's white end lands in**, destroying affinity exactly where the
   derivation needs it. In the encoded coordinate the neutral column never
   clamps. The visible consequence is that the fixture's white returns a
   residual `2.33×10⁻³` of ink rather than exactly zero; that is the legacy
   encoding's own `255/65 535` gap made measurable, carried through the closed
   form rather than hidden.

##### The four tolerances, counted

- **`F2` — half a 16-bit quantum, `7.629 511×10⁻⁶`.** The generator rounds each
  model value to the nearest `uInt16`; round-to-nearest is wrong by at most
  half a quantum. ★ **The worst case is ATTAINED** — the observed residual
  *equals* the bound, because several authored values (`0.525`, `0.450`,
  `0.375`, …) land exactly on a half code. `Record::graded` admits
  `observed == tolerance` deliberately (§1), so the row passes at the encoding's
  own extremum and fails at anything a *changed model* would produce. This is
  the row that licenses quoting `0.60` anywhere else in §F; without it the
  slope would be a number remembered from a recipe in another crate, which is
  the exact shape of the stale literals §3.5.8.6 exists to prevent.

- **`F4` — one 16-bit quantum, `1.525 902×10⁻⁵`.** `F4` compares
  `iccce transform` against **this harness's own evaluation of the same CLUT
  samples**, so the samples' quantisation cancels between the two sides. What
  survives, in quanta:

  | term | size |
  |---|---|
  | the PCS handed between the two `mft2` legs is a 16-bit encoded Lab — half a quantum, carried into device output by the `B2A` model's `0.60` gray slope | `0.30` |
  | a consumer may carry CLUT indices in 16-bit fixed point where the harness uses `f64` | `0.50` |
  | `iccce transform` prints six decimals | `0.07` |
  | **counted sum** | **`0.87`** |

  The bound is **the next whole quantum above the counted sum**, because a
  bound stated in fractional quanta claims a precision the encoding does not
  have. Observed `4.965 520×10⁻⁷` — `0.033` of a quantum, which incidentally
  says that neither side requantises the PCS; the first term is an allowance
  for a conformant consumer that does, not a description of this one.

- **`F7` — two 16-bit quanta, `3.051 804×10⁻⁵`.** The same counting with
  lcms2's pipeline in place of the harness's. lcms2 evaluates an `mft2` chain
  through `cmsPipelineEval16`, which requantises **each stage's** output, so
  two stages contribute a full quantum before anything else; plus the same
  `0.30` for the PCS carried by the gray slope, plus `transicc`'s four printed
  decimals of a percentage (`0.03`). Counted sum `1.33`, rounded up. Observed
  `1.400 000×10⁻⁵`.

- **`F3` — zero shortfall against a separation floor of `4×10⁻²`, declared in
  advance.** `Record::graded` compares `observed ≤ tolerance`, so a **lower**
  bound is expressed by grading its *shortfall* at zero: `observed =
  max(0, floor − separation)`. **Where `4×10⁻²` comes from, and it is not the
  observation:** the loosest device-space tolerance anything in this document
  has ever justified is §3.7's `SWEEP_DEVICE` at `4×10⁻³`; ten times it cannot
  be straddled by any bound this project has written or could plausibly write.
  The floor is derived from the **tolerance budget**, not from what the fixture
  measures — which is `4.207 049×10⁻¹`, an order above the floor again.

##### ★★ Why `F3` exists when the classifier already prints `ZERO-SEPARATION`

Every row already carries a candidate separation and §1.1's classifier already
prints `ZERO-SEPARATION` when it collapses — **and a flag is never a failure**,
deliberately, so that stating a separation never becomes dangerous. `F3` is the
one place where the collapse itself is *graded*, and the reason is specific:
**§F exists because a fixture collapsed.** A replacement that quietly collapsed
again would be the same defect under a fresh filename, and the apparatus should
say so out loud rather than emit a verdict beside it.

##### ★★ `F4`'s probe set: 50 CHROMATIC GRAYS, and why not the K ramp

`F4` and `F7` are the rows that must **stay green when black preservation
lands** — they are what makes `F5`'s red *attributable*. A row measured on the
K ramp cannot do that job: the day the feature ships, iccce's answer there
*should* change, and the row would go red for the right reason and look like a
regression.

A **chromatic gray** is `(c, 6c/7, 0.984 127c, k)` — the family for which this
fixture's `A2B0` returns `a* = b* = 0` **exactly**, since `a* = −60c + 70m` and
`b* = −50c − 45m + 90y` both vanish there. Such a point has `C`, `M` and `Y`
all **strictly positive**, so it is not K-only under any definition and **no
black-preservation path may touch it**, while its PCS image sits on the neutral
axis inside the dead band where the derived expectation is exact. If `F5` is
red and `F4` is green, the red means what it says; if both are red, the fault is
in reading the fixture and not in the missing feature.

Every probe is clamp-free **with its cell corners**, which is the condition
affinity actually needs: the largest darkness any probe reaches is
`0.661 c + 0.70 k ≤ 0.680`, and `A2B0`'s only clamp is `L* ≥ 0` above darkness
`1`.

##### ★★★ PROVEN BY INJECTION — and the injection found something the design did not anticipate

§3.10.9 records that Pass K *"has not been through"* the injection discipline
§3.7.6, §3.8.6 and §3.9.6 each applied. **§F has been, for two of its arms**,
by mutating the committed fixture's bytes in place and restoring them
afterwards (`gen-profiles verify` re-run to `41 identical` each time).

**Injection A — the collapse `F3` exists to catch.** Every `B2A0` CLUT sample's
`C`, `M` and `Y` set to zero, `K` untouched: the sibling
`v2-cmyk-mft2-lab`'s construction, at this fixture's grid.

| row | before | under injection |
|---|---|---|
| `F2` neutral column matches the authored model | PASS `7.63×10⁻⁶` | **FAIL `6.00×10⁻¹`** |
| `F3` separation is above the declared floor | PASS, shortfall `0` | **FAIL, shortfall `4.00×10⁻²`** — the whole floor |
| `F5` **k-only-in-implies-k-only-out** | **FAIL `4.207 050×10⁻¹`** *(red by design)* | **PASS `0.000 000`** |
| `F6` near-neutral transition width | `0` — no K-only region | `2.500 000×10⁻¹` — a full cell |

★★★ **Read the last two rows again. A collapsed fixture does not merely fail to
inform — it turns the headline row GREEN and gives the transition-width row a
number that looks like a working feature.** The suite would have reported
`fail=1` instead of `fail=2`, and the one remaining failure would have been the
Ghent row that **skips in CI**. On a corpus-free runner the whole thing would
have gone green with black preservation still unimplemented.

That is a stronger result than the design argued for. §F was built because a
`ZERO-SEPARATION` fixture *cannot discriminate*; the injection shows it
**manufactures a false pass**. It is also the concrete answer to *"why grade the
separation when the classifier already flags it"* (above): the classifier's
`ZERO-SEPARATION` verdict would have been printed, in a column, beside a green
row, in a run whose summary said `fail=1`.

**Injection B — the dead band `F1` exists to protect.** One node,
`(li, ai, bi) = (0, 5, 4)`, given a cyan offset of `−0.01` — exactly what any
non-zero chroma slope inside the band would produce.

| row | before | under injection |
|---|---|---|
| `F1` `B2A0` is `a*`/`b*`-independent across the dead band | PASS `0` | **FAIL `9.994 659×10⁻³`** |
| `F2` neutral column matches the authored model | PASS `7.63×10⁻⁶` | **FAIL `9.994 659×10⁻³`** |
| `F4` chromatic-gray round trip vs the derived table | PASS `4.97×10⁻⁷` | PASS `4.97×10⁻⁷` |

★ **`F4` staying green under injection B is correct and worth stating**, because
it is the shape a reader will otherwise call a hole: `F4` compares iccce against
the harness's evaluation of *the same bytes*, so a corrupted table corrupts both
sides equally and `F4` is silent by design. Detecting a corrupted table is
`F1`/`F2`/`F3`'s job and `gen-profiles verify`'s; `F4`'s job is the **pipeline**.
Two different questions, two different rows — which is the arrangement that lets
`F5`'s red be attributed at all.

**What has NOT been injected, and why.** `F4` and `F7`'s real rival is a
*consumer-side* defect — an implementation that reads clause 10.10's CLUT index
order backwards, or applies a transfer table this fixture does not carry — and
injecting one means editing `crates/` in a detached worktree. **Not done.** The
rival is instead **evaluated** rather than asserted: the harness reads the same
committed bytes with the index order reversed in both legs and takes the
distance, `4.843 550×10⁻¹` — `31 742×` `F4`'s bound and `15 871×` `F7`'s.

★★ **One separation claim was WRONG and the evaluation is what caught it.**
`F4` and `F7` first carried *"iccce applied the general PCSLAB encoding of
6.3.4.2 instead of the legacy encoding clause 10.10 mandates, which would move
the derived gray by `0.60 × 255/65 535`"*. That is **false of this row**: §F's
derivation works in *encoded fractions* end to end and never decodes to `L*`, so
a consumer applying the general rule in **both** legs round-trips to exactly the
same numbers — **a symmetric misreading cancels**. The stated distance would
have been a plausible sentence attached to a rival the row cannot see. DL-005's
misreading belongs to a row that *decodes* the PCS; these two do not.

##### ★ What §F does NOT buy — read before quoting it

**§F closes the *gradeability* gap, not the *population* gap.** It measures a
**synthetic instrument**, not a press, and its models are affine by
construction *precisely so that no interpolation envelope enters the
arithmetic* — which is exactly what a real profile does not give you. §3.10.0's
`7.053 20×10⁻¹`, §B's six-vendor sweep and §C's gray legs remain the only
evidence about **real ink**; they remain licensed and they remain skipped in CI.

And §F **decides nothing about the K value**. §3.10.6's fork — lcms2's
equal-`L*` construction against Cholewo (2000)'s `K_MIN`/`K_MAX` ratio — is
untouched: `E2` keeps its posture exactly (REPORTED, both rivals named), and
**no §F row grades the `K` channel of a transform**. `F2` grades the `K` column
of the *file*, as a property of the bytes, and that is a different claim.
`F4`, `F5`, `F6` and `F7` are about `C`, `M` and `Y` only, because
`C = M = Y = 0` in implies `C = M = Y = 0` out is definitionally unambiguous
and needs no answer to the K question.

> ★ **2026-08-18: `F8` is the one exception and it is not a change of
> posture.** The leak guard compares all four channels because its claim is
> that the preservation **branch was not taken**, and a branch that was not
> taken leaves every channel alone. It still grades no opinion about what `K`
> a preserving path should emit. §3.10.12.2.

##### Consequence for CI, stated rather than discovered

`F5` was **red, in CI, until black preservation existed**. That was the
intended effect and not a side effect: `E1` had been red since Pass K was
written and nothing outside one machine could see it. The `oracle` job's
coverage floor is raised from `15` to `22` accordingly, and the floor step and
the summary re-emission are made `if: always()` so that a red suite does not
also silence the guard that watches CI's reach.

> ★ **2026-08-18: `F5` is green in CI and the floor rises to `23`** — §F now
> contributes eight rows there, not seven. The `if: always()` change is kept:
> it was made for a deliberately red suite, but a guard on CI's *reach* is
> exactly the guard that must survive a suite going green, or the day coverage
> silently collapses is the day nobody is watching.

#### 3.10.12 ★★★ Grading the feature — 2026-08-18, and what the repointing turned up

`crates/iccce-cmm/src/black_preserve.rs` implements `KMapping::EqualLightness`
(the source and destination K-only ramps sampled at 1024 points through both
profiles' `A2B` directions, the destination's inverted by binary search plus
linear interpolation, a non-monotonic ramp **refused** rather than branched on).
`KMapping::Ratio` is a **named refusal**. The surface is
`iccce transform --preserve-black <policy>`, and **the policy name is
mandatory** — there is deliberately no bare `--preserve-black`, because the two
published definitions disagree by up to `4.9×10⁻²` on a cross-press pair and a
default would be iccce choosing one silently.

**Suite: `pass=337 fail=0 skip=9 error=0`, 44 Pass K rows.** No tolerance in
§3.10 was widened, softened or re-derived. Four rows were added — E7, E8, E9 and F8.

##### 3.10.12.1 What was verified independently of the engineer's own report

The engineer's handover stated three things; all three were re-measured here
through the shipped release binary and all three hold.

| claim | verified |
|---|---|
| chromatic ink is exactly `0.000000` on every K-only input, on all **ten** CMYK destinations in the corpus | **yes** — and the ten were enumerated from the profile headers rather than taken on trust |
| `K` is genuinely re-mapped, not copied | **yes** — `0.500000` (same press, correct), `0.502608`, `0.461018`, `0.366689`; and one destination the handover did not name returns `0.881462` at `K = 1.0`, i.e. it is *darker* than the source and equal lightness lands below full ink rather than clamping to it |
| `fmt`, `clippy -D warnings`, `cargo test --workspace` clean | not re-run here; the difftest workspace builds and is clippy-clean |

Two further checks the handover did not make:

- **Every named refusal was exercised through the binary.** Absolute intent,
  either side not 4-channel, `k-only-ratio`, a bare `--preserve-black`, and an
  unknown policy name. Each refuses with a distinct message; the two policy
  refusals exit `1`, the two usage errors exit `2`. Nothing falls back to an
  unpreserved conversion, which is `CLAUDE.md` rule 6 applied to a policy.
- **`--preserve-black` on and off are bit-identical on non-qualifying input.**
  This is now rows `E7` and `F8`; see next.

##### 3.10.12.2 ★★★ The repointing instruction was RIGHT and INCOMPLETE — and the gap was in the guard

Pass K's pre-feature header said: *when the feature lands, `E1` and `E3` must
be pointed at whatever surface exposes it.* That names the rows about the
**predicate**. It does not name `E4`, the row about the **regression** — and
`E4`'s own text claimed it was where a leaking preservation path *"shows up and
nowhere else in this module"*.

**That claim was false of `E4` as written.** Black preservation is **opt-in and
applied never by default**, so a row driving the plain surface has no
preservation code in its chain to leak. `E4` would have stayed green through
any leak whatever, and its own sentence would have vouched for the silence.

This is the shape §3.5.8's injection work already named — *ask which layer is
in the loop* — arriving one level up: **ask which layer is in the loop of the
FIX.** The remedy:

- `E4`, `E5`, `F4` and `F7` are now all driven **with** `--preserve-black`, so
  the feature is inside the loop of every regression guard.
- **`E7` and `F8` are new and they are the sharp instrument.** Each runs the
  same probe set twice through the same harness function, differing in nothing
  but the flag, and grades `max |on − off|` at **exactly zero**.

**Why exactly zero is derived and not merely strict.** Every probe has at least
one of `C`, `M`, `Y` strictly positive, so under the exact-zero qualifying rule
(which matches lcms2's `In[0]==0 && In[1]==0 && In[2]==0`) none of them
qualifies, the preservation branch returns `None` for all of them, and the two
invocations execute the identical arithmetic. **This is not an agreement claim
with an instrument error; it is the claim that a branch was not taken**, and a
branch is taken or it is not.

**Their evidence class is `self-consistency`, the weakest this suite emits** —
both sides are iccce and nothing outside this project is in the loop. They earn
their place because the *predicate* is exact where every available cross-check
on the same question carries an interpolation envelope two orders wider: `F7`'s
bound is `3.05×10⁻⁵`, so a leak below that would be invisible to it and is
visible to `F8`. ★ And `F8` is the **only §F row that includes the `K`
channel**, deliberately: every other §F row excludes `K` because its value is
§3.10.6's open fork, but a branch that was not taken leaves *every* channel
alone, and that claim needs no answer to the K question.

★ The named rival for both is the change a future contributor is most likely to
make: **widening the qualifying test from exact zero to a tolerance**, on the
grounds that `10⁻⁹` of cyan "is really K-only". `crates/iccce-cmm`'s module doc
names and rejects it; these rows are what would catch it.

##### 3.10.12.3 ★★★ `E3`/`F6`: the transition width is now a REAL divergence from lcms2

**iccce's K-only region is zero wide — the single point `C = 0`. lcms2's is
exactly one cell of its 17-node CLUT, `1/16`.** Before the feature that gap was
an artefact of a missing capability. It is now a **measured behavioural
difference between two implementations**, and project rule 7 requires it be
stated rather than tuned toward.

Settling it from the specification is not available: **ICC.1 contains no
black-preservation construct at all** (register entry **A51**, a *closed
negative*, §3.10's opening). There is no clause under which either width is
correct. So the two rows stay **REPORTED**, permanently, and the reasons are
worth separating:

- inventing a width so the section had a gate would invent the thing the pass
  exists to derive;
- **tuning iccce toward `1/16` would be adopting a vendor's CLUT resolution as
  a colour requirement.** lcms2's width is not a stated rule; it is what
  happens when the same exact-zero test is sampled into a 17-node table and
  interpolated. `D1` grades that shape on the oracle and `_cmsReasonableGridpointsByColorspace`
  returning `17` is the only reason it is `1/16` rather than something else.

★★ **A degeneracy this row had and no longer has.** `E3`'s observation was
`0.000000` **before** the feature and is `0.000000` after it — for opposite
reasons. Before: there was no K-only output at all, so the walk broke at the
first probe. After: the `C = 0` probe qualifies and the next one does not.
A row whose observation does not move across the change it was written to
detect is a blinded row, and it took a **second number** to tell the two states
apart: `cell_zero_chromatic`, the chromatic ink at the `C = 0` endpoint itself,
which is now printed on both rows.

##### 3.10.12.4 ★★★ `E8` — the row where iccce is right and the ORACLE is wrong

**On a same-profile pair the equal-lightness construction is the identity**:
the destination `K` whose K-only patch has the same `L*` as the source's at
`K_in` **is** `K_in`, exactly, for any strictly monotonic `L*(K)` ramp. That is
algebra. No press, encoding or interpolation term appears in the statement, and
**no implementation's output appears in it either** — which is why the row's
kind is `derived-expectation` and not `cross-check`.

| | |
|---|---|
| iccce, `max\|K_out − K_in\|` over the 41-point ramp | **`0.000000`** |
| bound | `1×10⁻⁶` — one printed unit of `iccce transform`'s six decimals, and **nothing else**, because the probe's own `K` values are `j/40` and are exactly representable in six decimals |
| **lcms2 intent 11, same pair, same quantity** | **`6.1×10⁻⁵`** |

**lcms2 is wrong here by `61` printed units and iccce is exactly right**,
because lcms2's `K` comes back through a 17-node CLUT while iccce inverts the
ramp directly. Recorded under rule 7 as a case where the engine is deliberately
right and the oracle is not — and note the direction: **the oracle's own error
here is the same order as the whole `E2` observation on this pair**, which is
part of why `E2` cannot gate.

★ **The rival named is the oracle's own answer** (`6.1×10⁻⁵`), not "copy K
through" — because on a same-profile pair copy-through *is* the correct answer
and would give `ZERO-SEPARATION`. Naming it would have produced a row that
looked discriminating and was not.

★★ **The premise belongs to the fixture and is stated on the row.** If this
destination's `L*(K)` ramp ever contained a flat stretch — ink saturating,
which real press profiles do — the inversion would be ill-posed there,
`crates/iccce-cmm` takes the **lower** `K` by a documented choice, and the
identity would fail **for a correct implementation**. A future red here is a
question about the ramp before it is a question about the inverter.

##### 3.10.12.5 ★★★ `E9` — the only row that can say WHICH definition iccce implements

The mandatory `--preserve-black <policy>` argument promises a caller that iccce
computes the named construction. **Nothing in the suite tested that promise**
until this row: §F grades no `K` channel at all, and `E2` is `BLIND` on its own
pair (§3.10.6). A promise no row can check is decoration.

`E9` converts the K-only ramp `ISO Coated v2 300% (ECI)` → **`GWG_GenericCMYK`**
— the corpus pair on which the two definitions are furthest apart — and
compares `K` against lcms2 intent 11 **at the 9 ramp points that are exact
nodes of lcms2's own 17-node black-preserving CLUT**.

| | |
|---|---|
| observed | **`3.1×10⁻⁵`** |
| bound | **`1.09×10⁻⁴`, computed at run time** — the destination's own device response to one 16-bit PCS quantum at this ramp's PCS points (`1.07×10⁻⁴`) plus `2×10⁻⁶` for the two print floors. The same `pcs_quantum_tolerance` function `A5` and `E4` use, applied to a different probe set, so the bound is a **function of the fixture** (§3.7.2 lesson 1) |
| named rival, *"copy K through"*, measured from the oracle and the input | **`4.890×10⁻²`** |
| separation ratio | **`1577×`** |

**Why only at the nodes**, and it is the finding of §3.10.6's second table:
between them lcms2 interpolates its own 17-node table rather than evaluating
its own construction, and the residual grows by `351×` to `1.089 5×10⁻²`. A row
graded over the whole ramp would be grading lcms2's choice of grid density.

★ **Evidence class: cross-check, and weaker than it looks.** iccce implements
lcms2's *own* construction by design, so agreement is expected. The row is
**not** evidence that equal lightness is the right definition — no such
evidence exists, because ICC.1 states nothing. It is evidence that iccce
implements the definition it names.

##### 3.10.12.7 ★★★ A FINDING WITH NO ROW — the compiled path spreads the preservation over a whole cell

**This is the most consequential thing the grading turned up and no row in this
suite can see it**, which is why it is prose with numbers rather than a
tolerance.

`iccce_cmm::compiled::CompiledTransform::new(&chain, grid)` folds a `Chain`
into one uniform interpolable grid by sampling `chain.convert` at every node.
`Chain::convert` applies black preservation, so the **nodes** are right. But
the preservation is a **discontinuity at `C = M = Y = 0` exactly** — that is
its stated design (`crates/iccce-cmm/src/black_preserve.rs`, module doc) — and
**an interpolant cannot represent a discontinuity**. Every point within one
cell of the K axis gets a blend of one preserved corner and its
non-preserved neighbours.

Measured out of tree against `ISO Coated v2 300% (ECI)` → itself,
media-relative, `--preserve-black k-only-equal-lightness`:

| quantity | grid 17 | grid 33 |
|---|---|---|
| max chromatic ink **on** the K axis (41 points) | `0.000000` | `0.000000` |
| max `\|compiled − reference\|` within **one cell** of the K axis | **`0.617121`** | **`0.617148`** |
| the same measure **far** from the axis (control) | `1.138×10⁻³` | `5.34×10⁻⁴` |

★★★ **Read the two rows of the table against each other and the diagnosis is
unambiguous. Doubling the grid halves the control and does not move the
near-axis error at all.** That is `O(1)` beside `O(h^1.32)` — the exact
signature §3.6's row **R6** was built to detect, and R6's own band already says
what it means: *below order 1 the error is not grid-driven and no number from
this instrument is evidence.* At grid 33 the near-axis error is **1156×** the
control.

**The direction is over-application, not omission.** At `C = 3.906×10⁻³`,
`K = 1.0`, grid 33: compiled returns `(0.0896, 0.0763, 0.0725, 0.9815)` — very
nearly the preserved answer — where the reference chain returns
`(0.7068, 0.6115, 0.5862, 0.8498)`, the ordinary colorimetric separation. The
compiled path is **applying black preservation to pixels that do not qualify
for it**, across a band one cell wide around the whole K axis. That is `E7`'s
and `F8`'s defect, in a layer neither of them can reach.

**Exposure, stated precisely rather than alarmingly:**

- **Unreachable from the CLI today.** `iccce bench` is the only command that
  builds a `CompiledTransform` and it does not accept `--preserve-black`. No
  user of the shipped binary can produce this.
- **Reachable from the library, which is the point.** `CompiledTransform` is
  `pub`, and folding a chain into a grid once is exactly what a per-pixel
  consumer does — it is the reason the type exists. A consumer that opts into
  preservation and then compiles gets this silently.
- **§3.6's numbers are not falsified.** Pass 6 measured a chain with no
  preservation in it; nothing there is wrong. What is now known is that its
  premise — a smooth envelope with order in `[1, 3]` — is **false for a
  preserving chain**, and no row anywhere tests the combination.

**Not fixed here, deliberately.** The remedy is a `crates/` change and belongs
to the engineer, and there are at least two defensible ones which are not the
same decision: `CompiledTransform::new` **refuses** a chain carrying a
preservation policy (rule 6 — report, do not repair, and the caller learns the
two features do not compose), or it **applies the policy outside the grid**,
testing the input for K-only before the interpolation exactly as
`Chain::convert` does. The second costs one branch per pixel and reproduces the
reference path exactly; the first is more honest about a compiled transform
being a *different* transform. Choosing between them is not a conformance
decision.

★ **What this role owes when it is fixed:** a row. The shape is available —
`iccce bench` would have to accept `--preserve-black`, or Pass 6 would have to
gain a section that drives the library — and the observation is already
written: *max `|compiled − reference|` within one cell of the K axis*, graded
against the same `pcs_quantum_tolerance` shape as `E4`, with the control that
earns it being the same measure far from the axis. Until then this section
contains a **known unmeasured behaviour**, which is worth more written down
than discovered by a consumer.

##### 3.10.12.6 Coverage of this grading, stated

- **One source profile** for §E (`ISO Coated v2 300% (ECI)`), **one intent**
  (media-relative), **one policy** (`k-only-equal-lightness` — the other is a
  refusal at this commit), **two destinations** (itself, and `GWG_GenericCMYK`
  for `E9` alone). The ten-destination sweep in §3.10.12.1 was run by hand as a
  verification of the handover and is **not** a row: it has no oracle leg.
- **No `--bpc` anywhere**, and no row grades the two flags together. A chain
  carrying both is untested at any layer.
- **`KMapping::Ratio` has no row**, because it has no implementation. If it
  gains one, `E9`'s shape is what it needs — a cross-press pair at the oracle's
  nodes — and the oracle for it is **not lcms2**, which computes the other
  definition.
- **No injection proof for the leak rows.** `E7`/`F8` were verified to *pass*;
  neither has been shown to *fail* under an injected widening of the qualifying
  test, which would need a `crates/` edit in a detached worktree. §3.10.9's
  standing item, now with two more rows against it.
- **The perceptual cost of preservation is unmeasured.** Every row here is in
  device units by §3.10.0's finding; nobody has asked what `ΔE2000` the
  preserved answer sits from the colorimetric one on a cross-press pair, which
  is the number a caller weighing the policy would want;
- ★★★ **the COMPILED path is unmeasured by any row and is measurably wrong**
  — §3.10.12.7. It is listed here as well as there because a reader who skims
  only this list must not come away thinking the coverage gap is small.

---

## 4. Changes to tolerances — append only

Every change to a number in §3 gets a row here. **Never edit a tolerance
in place.** The history of a tolerance is the only defence against it
drifting one justification at a time.

| Date | Row | Old | New | Who | Why |
|---|---|---|---|---|---|
| 2026-08-11 | §3.1, all rows (**first filling, not a change**) | blank | as recorded in §3.1 | `icc-conformance` | Pass 1 ran; the comparisons exist, so the rows are no longer allowed to be blank. Every number mirrors a `NUMERIC_CLAIMS.md` row rather than being chosen here. **No tolerance was widened; there was nothing to widen.** |
| 2026-08-11 | §5, NA-001 / NA-002 / NA-003 (**first filling**) | "none registered yet" | as recorded in §5 | `icc-conformance` | `iccce-color` shipped one named deviation from normative text (NA-001) and one policy choice (NA-002). Registering them is required by `ARCHITECTURE.md` invariant 3 the moment they exist, not when they are measured. |
| 2026-08-11 (later) | §3.3, rows 1–4 and 6–7 (**first filling, not a change**) | blank | as recorded in §3.3.1 | `icc-conformance` | Pass 3 ran; the comparisons exist, so the rows are no longer allowed to be blank. **No tolerance was widened; there was nothing to widen.** |
| 2026-08-11 (later) | **§3.3 row 5** — sRGB → AdobeRGB → sRGB round trip | **1×10⁻² ΔE2000** | **2.5×10⁻² ΔE2000** | `icc-conformance` | **★ A CORRECTED JUSTIFICATION, NOT A WIDENED NUMBER — and the distinction is the reason this row is long.** The 1×10⁻² was set *before* the run, from this reasoning: *"sRGB and Adobe RGB (1998) share their red (0.64, 0.33) and blue (0.15, 0.06) primaries and Adobe's green is more saturated, so the sRGB triangle is strictly contained, no grid point is clipped, and the only losses are interpolation ones."* **The run failed at 1.8788×10⁻².** §0's procedure was then followed in order. **(1) Is the code wrong?** No: iccce applies the clamp Annex F.8–F.16 makes normative, and the failure is that clamp doing its job. **(2) Is the expectation wrong?** There is no recorded expectation — both sides are computed in the run. **(3) Is the fixture wrong?** **This is where it was.** Every clause of the original reasoning is true of the two *colour spaces* and false of the two *files*: a matrix/TRC profile's media white is its colorant sum, and HP (1998) and Adobe (2000) rounded their colorants to `s15Fixed16` independently, leaving the two encoded whites 5/2/12 lsb apart. Source device white therefore lands at (1.000 106, 0.999 873, 1.000 254) in destination linear space and **is** clipped, along with 24 other grid points. **(4) Only then, the tolerance.** The mechanism was not merely asserted: a closed-form prediction from the two matrices and the clamp alone — no tone curve, no lcms2, no measurement — gives **1.878 244×10⁻²** against **1.878 818×10⁻²** observed, agreeing to **0.03 %**. The new number is that, plus ≈1×10⁻³ for table interpolation, plus ~25 % headroom. **What was actually wrong was an analysis, and the analysis is now on record in `tools/difftest/README.md` §13.6.3 alongside the sentence it replaced.** |
| 2026-08-11 (later) | §3.3 row 6 (**new check, not a change to an existing one**) | did not exist | **1×10⁻³ ΔE2000** | `icc-conformance` | Added *because* of the row-5 correction. Row 5 is an upper bound on a quantity that is mostly a deliberate cost, so removing iccce's range clamping would make row 5 **pass more easily** while deleting a normative requirement. A gate that rewards that is not a gate. Row 6 pins the observed cost to its closed-form prediction, and a sensitivity control (printed by `pass3_report`) shows it would fail by 19× if clamping were removed. |
| 2026-08-11 (later) | §5, NA-004 (**first registration**) | did not exist | as recorded in §5 | `icc-conformance` | Pass 3 shipped a gamut-clipping policy at the CMM layer, which NA-003 explicitly deferred to "where it can be a named per-transform decision". It is now named, and — unusually for this register — **measured on the first day it existed**. |
| 2026-08-11 (later still) | **§5, NA-003 — the "But note" clause citation** | cited **ICC.1:2022 6.4** as requiring per-component clipping of *device* values on integer conversion and permitting none for float32 | **superseded; the sentence is preserved verbatim in the new §5.2 and the row now points there** | `icc-conformance` | **★ A WRONG CITATION, CORRECTED — not a tolerance change; no number moved.** The recalled words are real but belong to a clause about the **PCS**: 6.4 is titled "Converting between PCSXYZ and PCSLAB encodings". The clause governing device encoding is **6.5**, whose float32 permission is doubly gated to `DToBx`/`BToDx` tags — which **8.3.3/8.4.3 do not permit in a matrix/TRC profile at all**. Settled by `icc-spec-librarian`'s fifth pass (`ICC_Spec\icc\icc__s__computational_models.md` §4/§4.2, **A39** resolved). **The correction inverts a finding that had been built on it**: `tools/difftest/README.md` §13.4 used NA-003 to hypothesise that lcms2's out-of-range float device output might be conforming and iccce merely stricter. It is not — a conforming F.8–F.16 evaluation *cannot* exceed 1,0, so the observed 1,000 120 is evidence the **input** clamp was skipped. Two hedges are carried into §5.2 and must survive restatement: clause 5 binds a CMM only to **reading** profiles (**A39b**), so "non-conforming" is not available and the word is *divergence*; and the **v2** half is **UNSOURCED** (**A39c**). The **size of the divergence under genuine out-of-gamut input remains unmeasured** — every observed excursion is 1-lsb boundary residue. |
| 2026-08-11 (later still) | §3.4, rows 0–7 and §3.4.2 (**first filling, not a change**) | blank | as recorded in §3.4 | `icc-conformance` | Pass 4's LUT differential ran; the comparisons exist, so the rows are no longer allowed to be blank. **No tolerance was widened; there was nothing to widen.** Two things are worth flagging about *how* these numbers were arrived at, because both are the kind of thing that would otherwise look like tuning. **(1) The tolerances were derived from an envelope computed before any comparison against lcms2's output** — the CLUT and the two interpolation algorithms alone — and the observed residuals then landed 0.3–0.5 % below it. Had they landed above it, the tolerance would not have moved; the finding would have been that something other than interpolation was in play. **(2) The wide rows were split from the tight ones deliberately.** NA-006 warned in advance that "a tolerance wide enough to swallow ~1 ΔE cannot also demonstrate agreement", so rows 2–3 are structural-only at 2.0 ΔE00 and the agreement claim was moved to rows 4–5 at 2×10⁻² and 1×10⁻³, where the method difference is switched off by construction. |
| 2026-08-11 (later still) | §5, NA-006 (**first entry in THIS document**; already registered in `NUMERIC_CLAIMS.md` §4) | "**~1 ΔE, corpus-derived bound, NOT measured**" | **measured: max 1.5741 ΔE2000 on `A2B0`, 0.254 23 on `A2B1`** | `icc-conformance` | The n-linear CLUT choice was registered the day the code landed with an explicit "iccce has NOT measured it, and cannot yet". Pass 4 measured it — against **lcms2's actual 4-D scheme**, which turned out **not to be tetrahedral** but a hybrid (linear in C, tetrahedral in M/Y/K), so the corpus's trilinear-vs-tetrahedral bound was not the applicable one. The measured value is close to it and the *shape* of the claim is unchanged; what changed is that "unmeasured" comes off. **The cost is a property of the table's curvature, not a constant** — the two A2B tags in one file differ by 6×. |
| 2026-08-11 (later still, Pass 4b) | §3.4.4, all rows (**first filling, not a change**) | did not exist | as recorded in §3.4.4 | `icc-conformance` | Pass 4b measured the **B2A** direction, the **v4 `mAB `/`mBA `** element pipeline and the **F.2 grayTRC** model; the comparisons exist, so the rows are no longer allowed to be blank. **Three things about *how* these numbers were arrived at, because each is the kind of thing that would otherwise look like tuning.** **(1)** Every tolerance is an **envelope computed inside the harness from lcms2's own arithmetic, with no lcms2 output in it** — the roundings were read at pin `21c582a` and modelled stage by stage — and each is paired with a much tighter row measuring what is left *after* the model is applied (A3, C4). Where the envelope was written into the doc comment as a guess before it was computed, the guess was **replaced by the computed value and the tolerance re-derived from it**; three rows below record exactly that. **(2)** The wide/tight split of §3.4 is kept, but its *sense is inverted* here: in Pass 4 the wide row was wide because of a real method difference; in Pass 4b the method difference is **zero** (lcms2 forces trilinear for a Lab-PCS LUT), so A5 exists purely as a **sensitivity control** showing the comparison could see a geometry difference 99–139× larger if there were one. **(3)** A **fourth kind**, `derived-expectation`, was introduced for §B and is defined in §3.4.4.1 including what it cannot do. It is **not** ground truth and §3.4.3's published-value row stays blank. |
| 2026-08-11 (later still, Pass 4b) | **§3.4.4 row C1** — gray → sRGB, device | **1×10⁻⁴**, justified by an envelope of "3,45×10⁻⁵" | **2,5×10⁻⁴**, justified by a **computed** envelope of 9,680×10⁻⁵ | `icc-conformance` | **★ A CORRECTED ENVELOPE, NOT A WIDENED NUMBER — and the distinction is checkable.** The 3,45×10⁻⁵ was written into the constant's doc comment *before the envelope was computed*, from a hand estimate of the resampling error of a 4096-point reverse curve. The reimplementation of `cmsReverseToneCurveEx(4096)` then put the real envelope at **9,680×10⁻⁵**, and the row was observed at 9,686×10⁻⁵. §0's procedure in order: **(1) Is the code wrong?** No — and this is not an opinion: modelling lcms2's destination collapses the disagreement **457×**, to 2,121×10⁻⁷, which is *below* `transicc`'s print floor. The residual is reproduced, not merely bounded. **(2) Is the expectation wrong?** There is none; both sides are computed in the run. **(3) Is the fixture wrong?** No. **(4) Only then, the tolerance** — re-derived as 2,6× the computed envelope. **The guess is preserved here so the change is auditable**; a reader who suspects tuning can recompute the envelope, which contains no lcms2 output at all. |
| 2026-08-11 (later still, Pass 4b) | **§3.4.4 row C3** — gray → sRGB, ΔE2000 | **1×10⁻²**, derived at **white** | **5×10⁻²**, derived at **black** | `icc-conformance` | **★ A DERIVATION LOOKING AT THE WRONG END OF THE AXIS.** The original reasoning propagated the device envelope through `dL*/d(device) ≈ 85 near white`. The run failed at 2,17×10⁻². §0's procedure: the code is not wrong (C4 attributes the whole residual, 457×), so the **analysis** was. Near *black*, below sRGB's linear breakpoint, a device difference `δ` becomes `δ/12,92` of linear light and CIELAB's **chromatic** sensitivity on its own linear segment (`da*/dX = 4038`) makes `Δa* ≈ 136 δ` against `ΔL* ≈ 69,9 δ`; with `S_C ≈ 1` and `S_L ≈ 1,75` the chromatic term dominates by ~3×, giving ≈2×10⁻². **This inverts §6.2's carried-forward note** that "near black the device metric explodes while ΔE stays small" — that holds for a device comparison amplified by an inverse TRC, and the opposite holds for a ΔE computed *from* a device difference at the same place. Both texts are kept; the new one is not a relaxation of the old but a different calculation. |
| 2026-08-11 (later still, Pass 4b) | **§3.4.4 row B6** — fixture → sRGB, device | **1×10⁻⁴** (shared with B5) | **2,5×10⁻⁴** (its own constant) | `icc-conformance` | **★ A MISSING TERM IN A DERIVATION.** B5 and B6 were given one constant because they are "the same fixture against the same oracle". They are not the same comparison: B5 ends at a **CLUT**, B6 ends at **sRGB's inverse tone curves** — and lcms2 builds those as a 4096-entry `u16` resampling whose envelope §C measures independently at 9,68×10⁻⁵, an order of magnitude above B5's whole budget. The row failed at 1,012×10⁻⁴, which is that term and nothing else. Split into two constants with two derivations. **The fix is a second constant, not a bigger one**: B5 keeps 1×10⁻⁴ and still passes at 5,2×10⁻⁵, so the change cannot be a blanket relaxation. |
| 2026-08-11 (later still, Pass 4b) | **§3.4.4 row B0** — both geometries on an affine CLUT | **0,0 — exact** | **1×10⁻¹⁴** | `icc-conformance` | **★ REAL ARITHMETIC MISTAKEN FOR FLOATING POINT.** The justification — "every interpolation geometry reproduces an affine function exactly" — is **true**, and the tolerance derived from it was still wrong: the two algorithms reach that value by different sequences of `f64` operations, so they agree to *rounding*, not bit-identically. Failed at 1,110×10⁻¹⁶. The new bound is derived from the arithmetic rather than the algebra: the n-linear arm sums 2⁴ = 16 products of values in [0,1], so ~16 ulp = 3,6×10⁻¹⁵, and 1×10⁻¹⁴ is ~3× that — **still 11 orders below one `u16` lsb**, so the row remains the precondition for B1–B4 that it was written to be. A general lesson worth carrying: **"exact" in a spec-derived argument means exact in ℝ, and a tolerance of 0,0 is only available when the two sides are the same operations in the same order** (as at §3.4.1 row 6 and §3.4.4 row C5, which *are*, and are still graded at 0,0 and still observe it). |
| 2026-08-11 (Pass 5) | §3.5, all rows (**first filling, not a change**) | two placeholder rows with no numbers | as recorded in §3.5 | `icc-conformance` | Pass 5 measured black point compensation; the comparisons exist, so the rows are no longer allowed to be blank. **No tolerance was widened; there was nothing to widen.** **Four things about *how* these numbers were arrived at, because each is the kind of thing that would otherwise look like tuning.** **(1) The scenario set was derived from both implementations' sources BEFORE anything ran**, and the derivation produced a *negative* result that is stated in §3.5.1 rather than discovered afterwards: everywhere iccce does BPC, lcms2's estimator reduces to the same two values, **so no row here discriminates the two estimators and none may be quoted as if it did**. A session that had measured first would have found six small numbers and read them as six independent agreements. **(2) Every device tolerance is an EARLIER PASS'S COMPUTED ENVELOPE times the BPC map's own gain**, because BPC inserts one matrix stage between two stages the pipeline already had — no table lookup, no `u16` rounding — and the derivations say which envelope, why the multiplication is the whole correction, and (row P6b) which term is **inherited rather than recomputed**. That flagged term was then *priced by the observation*: switching BPC on moved the residual by 1,097× where the gain alone predicts 1,0035, i.e. the operating-point shift the derivation warned about is real and worth ~9,4 %. **The envelope still bounds it**, because row C1's figure was a maximum over the whole gray axis rather than over the BPC-off operating point. **(3) Two rows grade a SIGN with no tolerance at all** (P8, P13), because the shift is `(Xd − Xs)/(Xi − Xs)·(Xi − X)` and its sign is algebraic; and **one row grades an end-to-end transform against a closed form with no implementation's output in it** (P14), which is the strongest expectation Pass 5 has and the only one that is not a cross-check. **(4) Two rows are REPORTED, NOT GRADED and it is deliberate** — the forcing policy (P16) and the A41 constant (P10). Both have a number, a mechanism and a named document that would settle them; neither has a clause today. |
| 2026-08-11 (Pass 5) | **§3.5 row P13** — S3, the direction test | graded the **negated minimum** signed difference | grades the **maximum** signed difference | `icc-conformance` | **★ A TEST THAT ASSERTED THE OPPOSITE OF ITS OWN SCENARIO, CAUGHT BY FAILING.** The row was written to say "BPC's effect has the documented sign", and in S3 the PCS *rises* — but the destination is CMYK, whose `K` **falls** as `L*` rises, so the device-space form of "the PCS rose" is again "nothing rose". The first draft negated the minimum and therefore asserted "nothing may fall" on a scenario whose entire point is that `K` falls; it failed at 3,1372×10⁻², which is precisely the effect it was supposed to confirm. §0's procedure in order: **(1) Is the code wrong?** No — the engine moved `K` by exactly the amount row P14's closed form predicts, to 9,5×10⁻⁸. **(2) Is the expectation wrong?** **Yes, and that was the whole of it.** **(3) Is the fixture wrong?** No. **(4) The tolerance did not move** — it is still `0,0`; what moved is which quantity is compared against it. **Recorded rather than quietly rewritten**, because a direction test that reads the same in both directions is a real hazard and row P14 exists to cover it: only a *magnitude* against a closed form can show that the two directions are different. |
| 2026-08-11 (Pass 5) | **§3.5 row P20** — the ICC-absolute refusal | matched the refusal on the paraphrase `"BPC is not applicable"` | matches the **exact wording** iccce prints | `icc-conformance` | **★ A GATE THAT WOULD HAVE PASSED ON THE WRONG REFUSAL.** The needle was written from the `ChainError` variant's *name* rather than its `Display` text, so the row failed on the first run against a correct refusal. The fix is the exact string — and the reason it is worth a row here is the failure mode it prevents rather than the one it hit: a loose needle (`"refused"`) would have made the ICC-absolute row pass on an **estimation-subset** refusal, so a build that had lost the absolute exclusion entirely would still have been green. |
| 2026-08-12 | §3.4.4.6, rows A0 and A6–A10 (**first filling, not a change**) | did not exist | as recorded in §3.4.4.6 | `icc-conformance` | The **saturation** table (`B2A2`) was run in the B2A direction, closing the clause of Pass 4's done-when that had explicitly failed. **No tolerance was widened and none was added**: A6–A10 reuse `DEVICE_B2A`, `DEVICE_B2A_MODELLED`, `DE_B2A_ROUNDTRIP` and `APPARATUS_B2A` unchanged. One row is new — **A0**, which grades that the three `B2A*` tags are three distinct blocks of file bytes — and it exists because the *reason* saturation had been out of scope was the sentence "saturation adds a third copy of the same shape", which was an assumption. In the **A2B** direction of this same file it is *true* (`A2B0` and `A2B2` are one block at one offset, which is why `pass4/swop/perceptual-equals-saturation` is graded at exactly zero); in the **B2A** direction it is false by two thirds of 145 588 bytes. **A null that would have been null by construction was identified before it was collected rather than explained afterwards.** |
| 2026-08-12 | **§3.4.4 row A6 / `DEVICE_B2A`** — the `why` string | named envelopes of "1.330e-4 at media-relative and 9.602e-5 at perceptual, plus ~276% headroom" | names **1.5525e-4 at saturation** as well, and ~222 % headroom over the worst of the three | `icc-conformance` | **★ A CORRECTED JUSTIFICATION; THE NUMBER DID NOT MOVE.** `B2A2` is the steepest of the three tables, so its computed envelope is larger than either figure the string named. The tolerance stays at `5×10⁻⁴`. **The direction of travel is the diagnostic one**: the justification moved *toward* the observation while the constant stayed put. Had the constant moved instead, it would have been tuning. The observed saturation residual is 1,550 0×10⁻⁴ against the 1,552 5×10⁻⁴ envelope — 99,8 % accounted for, the same signature as the other two intents. |
| 2026-08-12 | §3.5.7, rows Q1–Q8 (**first filling**) | did not exist | as recorded in §3.5.7 | `icc-conformance` | Pass 5b measured the black-point **ESTIMATORS**, which §3.5.1 stated in advance that no Pass 5 row could discriminate. **Four things about how, because each would otherwise look like tuning. (1) The mechanism was read at the pin BEFORE the run** — `cmssamp.c` L592 `Lab.a = InitialLab.a` against ISO 4.2.3's neutral black — so the confirmation is not inferred from the size of a residual. **(2) The prediction was pre-registered and is graded claim by claim**, with three different verdicts: CONFIRMED (mechanism, decay), FALSIFIED (magnitude), NOT ESTABLISHED (shape). A single verdict on a four-part prediction would have been wrong whichever way it went. **(3) The one falsification that is asserted is shown to survive the error bar arithmetically** (0,459 + 0,814 < 2,0), and the one that does not survive it is labelled *unattributed* rather than promoted — including an **oracle-free** sensitivity showing the obvious mechanism is 13× too small to explain it. **(4) Row Q1 passes by 5 % and the row says so**, because a marginal apparatus that is quoted as green is how a whole section becomes unfalsifiable. |
| 2026-08-12 | **§3.5.7 row Q1** — the apparatus bound | v1: `2,0 ΔE76`, the round trip over `L* ∈ [0, 20]`. v2: the same over a 15-`L*` band above the black, as a **ratio** at `1,0` | v3: the **local** residual at the two estimated blacks, as a ratio at `1,0` | `icc-conformance` | **★ A TOLERANCE THAT FAILED TWICE, AND NEITHER FAILURE WAS THE NUMBER'S FAULT.** **v1 failed at 16,49.** §0's procedure: the code is not wrong, there is no expectation, and the **fixture** was — `USWebCoatedSWOP`'s black sits at `L* ≈ 16,5`, so most of `[0, 20]` is *outside its gamut* and the quantity being graded was the **gamut boundary**, not an inaccuracy. **v2 re-derived the bound as a ratio** ("an error bar is readable exactly when it is smaller than what it bounds" — zero free parameters) and **failed at 1,107**: a maximum over a 15-`L*` neighbourhood prices in curvature the recovery never touches. **v3 measures the residual at the two points the recovery actually evaluates** and passes at 0,948. **The bound never moved from 1,0 across all three versions — what moved was the probe.** All three are on the record because the first two are the ones a reader would otherwise repeat. |
| 2026-08-12 | §3.6, rows R1–R8 (**first filling**) | two placeholder rows with no numbers | as recorded in §3.6 | `icc-conformance` | Pass 6 graded the compiled path. **Three things about how. (1) The tolerance's REJECTED derivation is recorded beside the chosen one**: "an order of magnitude below the 1,0 perceptibility anchor" is unavailable here because NA-006 alone was measured at 1,574 ΔE2000 on this same file, so it would be a budget derived from a total already exceeded. The chosen line — *compiling must not move the result further than the two implementations already differ* — is Pass 4's measured 0,252 94 and has no free parameter. **(2) It FAILS at 0,297 on both probe populations, and the number is not moving.** §3.6.1 states the remedy (grid 33, measured at 0,168) and states that a red suite is the correct state when a shipped default does not meet a justified line. **(3) The device row is deliberately ungraded** and the arithmetic that made it ungradeable is recorded: the bound the ΔE tolerance implies is *tighter* than the observation while the observed maximum is a midtone, so grading it would assert something neither row means. |
| 2026-08-12 | **§3.6 row R6** — the `h²` sensitivity control | graded the **max-of-max** ratio, justified as "`h²` predicts 4×; the band `[2,8]` accommodates CLUT-node alignment" | grades the **paired median** ratio, justified as "the observed convergence order lies in `[1,3]`" | `icc-conformance` | **★ BOTH THE ESTIMATOR AND THE JUSTIFICATION WERE WRONG, AND THE SECOND ERROR IS THE MORE USEFUL.** **The estimator**: `h²` is a statement about a *fixed point* as `h` shrinks, and a ratio of two maxima is not — as the grid refines, *which* probe is worst moves. Over three halvings the max-of-max wanders **5,57 → 1,39 → 1,78** while the paired median sits at **2,69 → 2,47 → 2,51**. `compiled.rs`'s unit test uses the max-of-max over **7** probes; its `[2,8]` band passed there by luck of the fixture. **The justification**: the first draft explained the band by gamut clamping and sRGB's breakpoint cutting cells. **That was tested and falsified** — restricting to cells whose 16 corners are all in gamut and above 0,040 45 changed the ratios *not at all*. What the measurement actually says is that the convergence order is **1,32**, stable to 1 % across three octaves — the signature of a smooth envelope with unresolvable fine-scale kinks, which SWOP's `mft2` has by construction: its 256-entry input tables put **255 derivative discontinuities per axis at `k/255`**, and `gcd(255, N) = 1` for `N ∈ {4,8,16,32}`, so **no compiled grid in reach aligns with them**. The band was re-derived to assert only order ∈ `[1,3]`, which is what a band can honestly assert. |
| 2026-08-12 (later, Pass 5c) | §3.5.8, rows T1–T8 (**first filling**) | did not exist | as recorded in §3.5.8 | `icc-conformance` | lcms2's `cmsDetectDestinationBlackPoint` was **reimplemented** from `cmssamp.c` at pin `21c582a`, which §3.5.7.4 had named as the highest-value item left in Pass 5's family. **Four things about how, because each would otherwise look like tuning. (1) T1 is §3.5.7 row Q1's constant and Q1's derivation, unchanged** — what changed is the error bar, from a round-trip recovery (0,948, green by 5 %) to a device residual against `transicc` converted through a measured sensitivity (0,304 on the same fixture, 33× tighter). **A tolerance carried across an apparatus replacement is the strongest evidence available that it was never fitted to an observation.** **(2) The section runs on TWO arms that reach OPPOSITE verdicts on the same pre-registered claim**, and every record id is prefixed with the arm so neither can be quoted as the other. **(3) T4 exists so that T5's absolute residual is not evidence of nothing**: it divides the residual under the lcms2 hypothesis by the residual under the ISO one, and it is what caught an apparatus fault (§3.5.8.4) that would otherwise have been filed as a finding. **(4) T5 is REPORTED and deliberately NOT graded against Pass 4b's 1,330×10⁻⁴ envelope for the same table**, because that is a maximum over a different population — §3.6 row R4's lesson, applied to a row that could have quietly borrowed a constant. |
| 2026-08-12 (later, Pass 5c) | **§3.5.7 row Q3** — "claim 1, the MECHANISM, CONFIRMED" | **CONFIRMED** — the chroma component of the divergence equals the detected black's chroma | **WITHDRAWN as a claim about lcms2; the row is renamed `…/1-mechanism-SUPERSEDED-BY-PASS-5C-structural-only` and its tolerance is UNCHANGED at 1×10⁻¹²** | `icc-conformance` | **★★ A GREEN ROW WHOSE GREENNESS WAS NEVER THE CLAIM, AND THE LABEL IS WHAT SAVED IT.** Q3 was filed as *“STRUCTURAL on iccce's side and that is the point of the row — what it grades is that clause 4.2.3 is implemented, NOT that the prediction's substance was right”*. That reading is still exactly right and the row still passes at 0,0. What is withdrawn is the **headline**: the chroma it was compared against was not lcms2's black point's chroma but chroma the `A2B1 ∘ B2A1` **recovery** introduced. Pass 5c reimplemented the estimator and found lcms2 returns a **neutral** black on this fixture, because a CMYK **output-class** profile at relative colorimetric reaches `BlackPointUsingPerceptualBlack`, which forces `a* = b* = 0` (`cmssamp.c` L174) — a branch §3.5.7.2's table did not trace. **No tolerance moved; a claim did.** |
| 2026-08-12 (later, Pass 5c) | **§3.5.7 row Q5** — "claim 3, the SHAPE, NOT ESTABLISHED" | **NOT ESTABLISHED**, reported at 1,580 11× with the `L*` term inside the error bar | **SETTLED — renamed `…/3-shape-SETTLED-IN-PASS-5C`; still REPORTED, still 1,580 11×** | `icc-conformance` | **★ THE ONE VERDICT PASS 5b DECLINED TO ASSERT IS THE ONE THAT SURVIVED.** Q5 refused to call a falsification the evidence could not support and named the experiment that would settle it. That experiment is §3.5.8. The answer: on the `swop` arm the chroma term is **exactly 0** and 100 % of the divergence is `L*` — claim 3 **FALSIFIED**; on the synthetic RGB arm the `L*` term is **exactly 0** and 100 % is chroma — claim 3 **CONFIRMED**. Both are true, of different profiles, and the discriminating variable is the destination's device class and colour space. **Q5's number is a property of the recovery and is retained only so the two can be compared.** |
| 2026-08-12 (later, Pass 5c) | **§3.5.7 row Q8** — the shipped chain's refusal | **0,0 exact**: 0 if the binary REFUSED with the exact wording | **INVERTED, same constant**: 0 if the binary CONVERTS. Renamed `pass5b/coverage/shipped-chain-now-REACHES-the-iso-estimator` | `icc-conformance` | **★ A COVERAGE GAP THAT CLOSED, KEPT VISIBLE CLOSING.** Q8 graded `iccce transform --bpc` refusing a v2 CMYK LUT destination at media-relative, because Pass 5b had found `bpc::estimate_lut_destination_black` implemented, unit tested and **with no caller**. Commit `c268261` wired it into `Chain::estimate_dst_black` and the refusal is gone, so the row began failing on its own success. **Deleting it would erase the transition from the record stream**; inverting it keeps the history and still catches a regression that unwires the estimator. The **numeric** successor — does the wired path reach the *same* black point the library function does — is §3.5.8 row T8, deliberately not duplicated here. |
| 2026-08-12 (later, Pass 5c) | **§3.5 row S5** — `pass5/S5/…/refuses-outside-the-subset` | **0,0 exact**: iccce must REFUSE, and the row's prose said *“SO NO COMPARISON EXISTS FOR THIS CASE and Pass 5 claims none”* | **INVERTED, same constant**: iccce must CONVERT. Renamed `pass5/S5/…/SUPERSEDED-now-inside-the-subset` | `icc-conformance` | Both halves of the sentence stopped being true on the same day. The subset boundary moved when the ISO estimator was wired, and **§3.5.8 makes the comparison this row said did not exist** — the two estimators are 8,167×10⁻² ΔE76 apart on this pair, entirely in `L*`. Recorded here rather than silently deleted for the same reason as Q8: a coverage gap that closes should be visible closing. |
| 2026-08-12 (later, Pass 5c) | **§3.6 rows R3 and R4** — the compiled-path gate | **2,5×10⁻¹ ΔE2000**, observed **2,970×10⁻¹ / 2,963×10⁻¹ — FAIL** at the then-default grid of 17 | **2,5×10⁻¹ ΔE2000 — UNCHANGED**, observed **1,677×10⁻¹ / 9,349×10⁻² — PASS** at the new default grid of 33 | `icc-conformance` | **★★ NOT A TOLERANCE CHANGE. THE ROW IS HERE BECAUSE THE NUMBER DID NOT MOVE AND THE WORLD DID.** §3.6.1 said in terms: *the remedy is the grid, not the number — what is not available is moving 2,5×10⁻¹, because it is Pass 4's measured figure and has no free parameter to move.* Commit `189e732` changed `compiled::recommended_grid_points` from 17 to 33 for 3-D and 4-D, and both rows went green against the identical constant. **What a red suite is for, demonstrated end to end.** Two consequences are recorded rather than left implicit: **(a)** the harness's `DEFAULT_GRID` must track the shipped default, and row **R1 caught the drift by failing at 1,576×10⁻³** — the gap between the two grids' costs, not an error; **(b)** at grid 33 the two probe populations no longer agree to 0,25 % (R3 is **1,79×** R4), because once the error is small enough probe placement dominates, so **quoting either alone is now a population claim**. The build cost of the new default is **~14 s** against 1,06 s, which moves `iccce bench`'s break-even from ~70 000 px to **~1,19 million px** — reported by the binary, graded nowhere, and stated here because it is the price of the green. |
| 2026-08-12 (later, Pass 4c) | §3.4.5, rows C0–C8 (**first filling, not a change**) | did not exist | as recorded in §3.4.5 | `icc-conformance` | **★ NO NEW DEVICE TOLERANCE WAS MINTED, AND THAT IS THE POINT OF THE ROW.** All four graded device rows (C1, C2, C8 and the §B floor) **reuse `pass4b::DEVICE_B2A` at 5×10⁻⁴ unchanged**, because they end in the *same destination table* by the *same evaluator* in the *same direction*, so the envelope transfers with its justification intact. A fresh constant fitted to Pass 4c's own 8,90×10⁻⁵ would have been a number chosen because it passed — §3.4.4.6 set that precedent when the saturation table reused the same constant and only the `why` string moved. **Two rows are graded at exact zero and neither is arithmetic**: C0 and C6 are *counts of files* satisfying lcms2's substitution predicate, read from the parsed headers of the files actually opened, and they are the precondition without which every other number in §3.4.5 is measuring the policy again rather than the arithmetic. **The one number that could have been tuned and was not is C4's sensitivity floor of 100×**, which is **transcribed from Pass 4b's already-accepted counterfactual band of 99×/139×/191×** on this same table and direction; the observation is 2 310×, so the floor would have been identical had the observation been 105×. **And C5 exists because the obvious null is not the only null**: a comparison can also be vacuous by *clipping*, where both implementations clamp to the same gamut boundary and agree perfectly while computing nothing — C5 counts the points the absolute scaling did not move (**1 of 729**, device black, the fixed point of any diagonal) and budgets an order of magnitude above it. |
| 2026-08-12 (later, Pass 4c) | **§3.4.4.5's first bullet** — "saturation and ICC-absolute in any of the three directions" | ICC-absolute through a LUT destination recorded as unmeasured, citing the composite as **"D.6/D.7"** | **superseded in its ICC-absolute half by §3.4.5**; the citation corrected to **`ICC.1:2022` 6.3.2.2 Eq (4)–(6), restated at D.6.1 Eq (D.7)** | `icc-conformance` | **★ NOT A TOLERANCE CHANGE — a blocked item that was never blocked on what it said it was, plus a citation hazard.** The bullet treated ICC-absolute through a LUT destination as out of reach pending the A4b document question. **It was blocked on a PROFILE PAIR, not on a document**: lcms2's substitution predicate is a conjunction, and any pair that breaks either half on both profiles removes the confound structurally. That pair was available in the committed fixture corpus the whole time. **The citation is the second half and it is the kind that propagates**: "D.6/D.7" is **not edition-stable** — in `ICC.1:2001-04` Annex D the equations are (D.1)–(D.6), there is **no (D.7)**, and that edition's (D.6) is the single `Z` component of the *inverse*. Since every document in this project that discusses `wtpt` is discussing a **v2** file, the ambiguity was live wherever the bare label appeared. *(Both sourced by `icc-spec-librarian`, 2026-08-12, from `icc__s__rendering_intents.md` §3.1–§3.4, `evidence: primary_spec`.)* |
| 2026-08-12 (later still) | **§3.5.8 rows T1, T3, T4, T6** — the black-point section, **re-measured on the corrected 4.2.5.4 code** | T6 **8,166 8×10⁻² ΔE76** on `swop`; T4 **1,714 7×10⁻¹**; T1 **3,043 1×10⁻¹**; T3's note *"ISO returns `outRamp[first]`"* | T6 **4,799 109**; T4 **4,258 5×10⁻²**; T1 **5,178 5×10⁻³**; T3's note corrected — **both sides return their own `InitialLab`** | `icc-conformance` | **★★★ NO TOLERANCE MOVED. THE ROW IS HERE BECAUSE A PREDICTED COLLAPSE CAME BACK AS A 58,8× GROWTH, AND THREE GREEN ROWS GOT GREENER FOR A BAD REASON.** `NUMERIC_CLAIMS.md` §3.24.4 predicted NC-142's 8,167×10⁻² would collapse once commit `fd34a44` fixed iccce's non-conformant 4.2.5.4 return; it grew to **4,799 109 ΔE76**. **The non-conformant value (`outRamp[first] = MinL = 16,489 806`) sat 0,082 `L*` from lcms2's answer; the conformant one (`InitialLab = 11,772 365`) sits 4,799 `L*` from it.** The defect's own magnitude — the distance between the two code paths — is **4,717 441 `L*`, 57,8× the divergence it was blamed for**; it was nearly invisible in the cross-check meant to detect it. **T1 and T4 both improved, and neither improvement is good news**: T1's error bar did not move (its *effect* grew 59×) and T4's numerator did not move (its *rival* got 4× worse). **NC-164a's attribution is right about the cause and wrong about the consequence** — *"this defect accounts for the whole of the disagreement"* and *"fixing it will end the disagreement"* are different claims and only the first was measured. Full finding, coverage and the consequence for **NA-009** (whose cost is now measurable at 4,799 109 / 5,000 000 ΔE76): **§3.5.8.6**. |
| 2026-08-12 (later still) | **§3.6 row R3's justification** — the grid-dependence clause | *"**GRID-DEPENDENT**: the quantity is `O(h^1,32)` here, so the bound belongs to **grid 17** and to nothing else"* | *"grid-dependent in its **applicability**, not in its **derivation**"* — the derivation population contains no compiled grid at all; the applicable grid is **33** since `189e732` | `icc-conformance` | **★ A CONFLATION, NOT A STALE NUMBER — and the difference is why the row is worth writing.** `COMPILED_DE` is Pass 4's iccce-vs-lcms2 maximum over 341 CMYK points, and **Pass 4 never builds a `CompiledTransform`** — there is no grid in the bound to become stale, which is exactly why it survived the shipped default moving 17→33 untouched. What the grid governs is whether an *observation* may be compared to it, since the graded quantity is `O(h^1,32)`. Writing that as *"the bound belongs to grid 17"* invited the reading that the tolerance had a derivation population out of step with the shipped product; **it did not, and the corrected wording is what makes that checkable.** The row title now names the shipped default rather than freezing a number into it. |
| 2026-08-12 (later still) | **§3.6.2's throughput bullet** — re-filed as **§3.6.3** | *"2,4–2,7 Mpix/s compiled vs 0,076–0,091 reference (**28–32×**, break-even **≈63 000–75 000 px**) … spread across four invocations ~10 %"* | **break-even ≈1,3×10⁶ px AT GRID 33**, with the grid mandatory; **the speedup is WITHDRAWN as a documented figure** | `icc-conformance` | **★★ TWO DEFECTS WITH DIFFERENT CAUSES IN ONE SENTENCE, AND ONLY ONE OF THEM IS "OUT OF DATE".** **(a)** The break-even was measured at the old default grid of **17**. `N = build ÷ (1/ref − 1/comp)` puts `build` in the numerator, and `build` is the only term the grid moves: measured over ten invocations, the two throughputs are indistinguishable at 17 and 33, while build goes **0,838 s → 12,444 s (14,8×)** and break-even goes **85 900 px → 1 273 800 px (14,8×)** — agreeing to three figures. **A break-even without a grid is like a tolerance without units**, and it is now stated with one. **(b)** The speedup was never reproducible. Ten invocations in **one session, one machine, one binary** span **12,44×–25,27×** (2,03×); across the day's sessions, **12,4×–32×**. The recorded "~10 % run-to-run spread" understated the variance by an order of magnitude, because it was a four-sample range from one sitting quoted as a property of the machine. **A wall-clock ratio on a loaded desktop is not a claim this project can carry**; the break-even is, and it is structurally the stabler statistic — since `1/comp ≪ 1/ref`, `N ≈ build × ref_rate` and the noisy arm barely enters (the compiled rate spanned 2,08× over the five grid-33 runs while the break-even computed from those same runs spanned 1,13×). **(c)** The reference arm's recorded 0,076–0,091 band does not contain today's 0,092–0,099 — but the reference arm is the **tightest** quantity measured (±4 % against the compiled arm's ±35 %), so that is the same error as (b) and not evidence of instability. |
| 2026-08-12 (third filing) | **§5 NA-009 (new row, not a change)**; **§1.1 (new subsection)**; and **two justification STRINGS** in `tools/difftest/src/pass5c.rs` — `NEUTRAL_EXACT` and `SHIPPED_MATCHES_LIBRARY` | NA-009 absent from this table; `NEUTRAL_EXACT` asserted *"0,834 on USWebCoatedSWOP, 5,0 on the synthetic RGB fixture"*; `SHIPPED_MATCHES_LIBRARY` asserted *"the two candidate blacks are `2,46×10⁻³` apart, **three orders** above the bound"* | NA-009 registered with its measured cost; both strings now point at the row's **emitted candidate separation** instead of naming a figure | `icc-conformance` | **★★ NO TOLERANCE VALUE MOVED — `0,0` and `1×10⁻⁶` are unchanged, and the whole row is about what the justifications SAY.** The mechanism of §1.1 computes what those sentences asserted, and on its first run it caught a **fourth** stale literal to set beside §3.5.8.6's three: `2,46×10⁻³` was the **pre-`fd34a44`** device separation and the live value is `9,574×10⁻³`, so the claim was understated by 4× and the *"three orders"* was wrong by one. **The argument was never harmed — only the number was** (the separation is ~9 600× the bound, not ~2 500×), which is exactly why a claim-bearing figure the apparatus can compute must be interpolated and not typed. The `0,834`/`5,0` pair was still true and was replaced anyway: both are properties of *which fixture is loaded*, so a third arm would have falsified the sentence without touching it. |
| 2026-08-12 (fourth filing) | **§3.5.9, two new rows** (`CLAUSE/4.2.5.4-…` at `7,629 5×10⁻⁴` and `FIXTURE/candidates-are-separated-as-designed` at `2,288 9×10⁻³`) — **first filling, not a change**; plus a **third Pass 5c arm** and a new fixture `v4-rgb-mab-floored-b2a.icc` | ISO/CD 18619 4.2.5.4 had **no committed instrument in this suite**: a full reversion of `fd34a44` turned no row red on any machine | the reversion now fails exactly one row, on a **committed** fixture, with no oracle or system profile in the loop | `icc-conformance` | **★★★ THE FIRST TOLERANCE IN THIS DOCUMENT WHOSE DERIVATION HAS NO TERM IN IT AT ALL BEYOND ONE ENCODING QUANTUM — because the fixture was authored to remove the others.** `A2B1` at device `(0,0,0)` is a CLUT **corner** read through identity curves, so there is no interpolation term; no oracle is consulted, so there is no oracle term; 4.2.3 assigns neutral **literally**, so the chroma terms are exactly zero. What is left is the generator's own round-to-nearest into 6.3.4.2's general PCSLAB encoding — **half of `100/65 535`**, and nothing else. **The expectation is a named constant in `recipes.rs` put through a clause, so this is `derived-expectation` and not a cross-check**, and it deliberately runs outside `analyse`: *a derived expectation must not be hostage to an oracle.* **Three things about how, because each would otherwise look like tuning. (1) The power was PROVEN, not asserted** — §3.5.9.4: the pre-`fd34a44` return value injected in a detached worktree, both category (c) paths repointed at a non-existent drive, 27 `pass5c` rows skipped, and the new row the **only** failure at `2,500 019×10¹`. **(2) A failing row was NOT fixed by widening it.** The third arm made `apparatus/error-bar-is-smaller-than-the-effect` fail at `3,775×10⁹`, and it was **right**: the fixture's floor makes `d(device)/d(L*)` zero by construction and §B is void on that arm. `APPARATUS_RATIO` stays at `1.0` and still applies everywhere the conversion it needs exists; what was added is an **authored table** (`DEVICE_OBSERVABLE`) plus a row grading the measurement against the declaration, so the exemption cannot be acquired by a quantity coming out small. **(3) The brief's premise was corrected in both directions** — the clause was **not** undefended (`cargo test -p iccce-cmm` fails on the reversion, two tests, verified), and the vendor arm was **not** load-bearing either (its numbers moved and no row crossed a bound). What had no instrument was the clause exercised **through a parsed profile**. |
| 2026-08-12 (fourth filing) | **§3.4.5.1** — candidate separations on all ten Pass 4c rows; and **§1.1.2**, a defect in the separation mechanism itself | Pass 4c rows all `UNSTATED`; `Separation::against` used wherever two candidate observations existed | four rows `Measured`, six `no-named-alternative` **with reasons**; three rows moved to `against_distance` | `icc-conformance` | **★★ NO TOLERANCE MOVED; A MECHANISM WAS FOUND TO LIE IN ONE PLACE.** `Separation::against` derives its distance as `\|observed − alt_observed\|`, which **collapses to exactly zero on the run where the code actually returns the alternative** — measured: the new clause row failed at `2,500 019×10¹` and printed `ZERO-SEPARATION` beside it, the mechanism disclaiming its power on the one run where it had just demonstrated it. The test now recorded on the constructor's own doc comment: **is the distance a property of the RUN or of the FIXTURE?** Three rows corrected (one clause row, two `0/1` indicators whose candidates are always one apart). On Pass 4c the notable half is the **six** honest absences: the media-relative floors have no rival because lcms2 consults the media white point only for the ICC-absolute adjustment; the sensitivity-floor row has none because *the only alternative nameable is a different floor, and that is a tolerance question* — **conflating a rival tolerance with a rival candidate is how a separation becomes a second, undocumented gate.** Each precondition row names the reading that is **the strongest threat to its own claim** and enumerates the other two, because naming the rival that flatters a row is the tuning this mechanism exists to prevent. |
| 2026-08-12 (fourth filing) | **§3.5.8 row T6 / `estimators/black-points-in-lab`** — asked whether the `4,717 441` separation now justifies a real tolerance | `∞` — REPORTED, NOT GRADED | **`∞` — unchanged, and now argued rather than defaulted** (§3.5.9.6) | `icc-conformance` | **★ A TOLERANCE THAT WAS ASKED FOR, CONSIDERED, AND DECLINED — recorded because a declined change is evidence about the budget too.** Three reasons. **(1) There is nothing for a bound to mean**: since `fd34a44` both sides return a quantity their own document calls `InitialLab` and the two documents mean different things by the name; **no clause requires them to agree**, so grading the difference grades iccce against lcms2's reading of a document iccce does not implement. **(2) A bound derived from the separation is a bound fitted to one known defect** — anything below `4,717 441` would have failed the pre-`fd34a44` build and anything above would not, and it could not be one number: the three arms observe `4,799`, `5,000` and `10,000`. **(3) The defect now has a row with a real derivation** (§3.5.9.3). **The generalisation: a large separation on an `UNGRADED` row is a request for a fixture and a graded row elsewhere, not a licence to grade that row.** Ask what clause the number would be graded against; if the answer is *"none, but it would have caught the bug"*, the bound is fitted to the bug. |
| 2026-08-17 (Pass G) | **§3.7, all rows (first filling, not a change)** | did not exist | as recorded in §3.7 | `icc-conformance` | Pass G graded iccce against the **Ghent PDF Output Suite 5.0** profile corpus — the first differential grading in this suite whose inputs are profiles a **real document producer embeds** rather than synthetic, OS-shipped or standards-body-issued ones. **72 rows, `pass=229 fail=0 skip=3` for the whole suite.** Four things about *how* the numbers were arrived at, each of which would otherwise look like tuning. **(1)** Every tolerance is either an **envelope computed from the profiles' own bytes and two published algorithms, with no lcms2 output in it** (§A's structural and propagated gates), or a **discrimination requirement against a named rival computed the same way** (§B's `4×10⁻³`), or an **encoding floor** (§3.7.4), or a **classification bound built from the file's own two candidates** (§3.7.4's authoring rows). The instrument that computed them — `tools/difftest/src/bin/ghent_probe.rs` — **grades nothing and cannot fail**, deliberately. **(2)** The wide/tight split of §3.4 is kept: §A's structural rows admit the whole interpolation-method difference and say they cannot claim agreement; the agreement claim is the emulated-geometry row's, ≥40× tighter, and it collapses the residual **179×** and **243×**. **(3) §B has NO attribution row and the section says so** — the harness has no `mft2` B2A model — so §B is a structural gate with a stated rival and **not** an agreement claim, and its 17–63× margin is explicitly not offered as one. **(4)** All four sections were **proven by injection** (§3.7.6), including one injection whose only purpose was to show that a run-time tolerance *selection* is load-bearing. |
| 2026-08-17 (Pass G) | **§3.7.1, `…/{A2B0,A2B1}/pcs-lab-corners-interpolation-free`** | **`1×10⁻³`**, justified by *"the 2-entry B curves are affine"* | **`1×10⁻³` when the tag's B curves are the exact identity, `2×10⁻³` when they are not — selected at run time from the tag's own bytes** | `icc-conformance` | **★★ A TERM THAT WAS MISSING, FOUND — not a number that was widened, and the distinction is checkable by injection.** The first draft failed on `A2B0` at **1.111 856×10⁻³** while passing on `A2B1` at 6.074×10⁻⁵. §0's procedure in order: the code is not wrong (the disagreement is entirely in `L*` and only on the tag whose `L*` B curve is not the identity); there is no recorded expectation; the fixture is not wrong; **the derivation was wrong**. *"Affine"* is true of both tags and irrelevant — what matters is *"the exact identity `(0x0000, 0xFFFF)`"*, because lcms2 evaluates a non-identity 2-entry `curv` through `cmsEvalToneCurve16` and rounds **twice** where iccce uses `f64`. Two ≤½-lsb roundings is ≤1 lsb = `1.526×10⁻³` in `L*` through the v4 PCSLAB decode; observed 0.73 of that. **The remedy is a tolerance that is a FUNCTION of the tag, so a profile with identity B curves keeps the tight floor**, and `injection I3` (§3.7.6) shows that forcing the identity branch turns exactly that one row red and nothing else. The original justification is preserved verbatim in §3.7.2 so a reader who suspects tuning can audit the change. |
| 2026-08-17 (Pass G) | **§3.7.1, `passg/xrite-v4-to-srgb/<intent>/device-vs-lcms2`** | `4×10⁻³` (§B's `SWEEP_DEVICE`, reused) | **the method envelope propagated through the actual destination model, ×1.25, +1×10⁻⁴ — computed per tag at run time** | `icc-conformance` | **★ A TOLERANCE APPLIED IN THE WRONG DIRECTION, corrected.** `SWEEP_DEVICE` is derived for the **B2A** direction, where lcms2 forces trilinear and the interpolation-method envelope is **identically zero**; these rows are **A2B**, where it is the *dominant* term. Three rows failed at 8.98×10⁻³–1.49×10⁻² against 4×10⁻³ and they were **right to**: a bound that omits the dominant term is not a bound, however small its number looks. The replacement pushes the harness's **two** Lab answers — n-linear and lcms2's geometry — through `MatrixTrc::pcs_to_device` for the destination file, point by point, and takes the largest device difference; **no lcms2 output enters it.** Like §3.4's wide rows it admits the whole legitimate interpolation difference and therefore detects structural error only; the agreement claim for this profile stays in the PCS, where the destination model is not in the way. |
| 2026-08-17 (Pass G) | **§3.7.4, the authoring rows** | drafted as one row per profile, `|Σcolorants − PCS white|∞` against a `2×10⁻⁴` encoding floor | **two different rows for two different claims**: a **classification** bound (half the distance to the profile's own rival white) where `wtpt` and colorant sum disagree, and the **encoding floor** where they agree | `icc-conformance` | **★ THE APPARATUS CAUGHT IT, NOT A PERSON — the separation mechanism flagged `BLIND` on the `eciRGB v2` row.** Two faults, one visible and one not. **Visible:** for a profile whose `wtpt` *agrees* with its colorants there is no rival reading, so a "separation" of 5.4×10⁻⁶ against a 2×10⁻⁴ tolerance is a manufactured alternative and `BLIND` said so; that row is now `NO-NAMED-ALTERNATIVE` **with its reason**, and is the section's negative control. **Invisible until the first was fixed:** the `2×10⁻⁴` encoding-floor derivation does not actually hold for the disagreeing profiles — Ghent's sRGB colorants sum to the PCS white to **1.885×10⁻⁴**, i.e. ≈12 `s15Fixed16` lsb, because the *published* sRGB primaries do not sum to D50 to the encoding lsb. The row was passing **inside** a bound its own justification could not support, which is §5.2's shape exactly. The replacement asks a question with no free parameter — *is the colorant sum nearer the normative PCS white or nearer the profile's own encoded `wtpt`?* — and **imports no third white point**, D65 in particular, whose constant `docs/NEXT_SESSION.md` §0 records as the weakest in `iccce-color`. |
| 2026-08-17 (Pass H) | **§3.8, all rows (first filling, not a change)** | did not exist | as recorded in §3.8 | `icc-conformance` | Pass H graded **acceptance and refusal** over the ICC's own published profile set (50 files from `color.org`). **48 rows; whole suite `pass=270 fail=1 skip=9 error=0`, bare exit 1.** Four things about *how* the numbers were arrived at. **(1) Most of them are indicator counts with a tolerance of exactly zero**, and a new `Metric::IndicatorCount` was added so a count stops being emitted under the label `abs-max-component`; a count has no instrument error, so any bound above zero would be an allowance for a defect rather than for noise. **(2) The expectations are not this project's**: §A/§B derive the required verdict from the **harness's own reading of bytes 8..12** (clause 7.2.4) with the parser under test nowhere in the loop, and §D transcribes two paragraphs of a **published ICC document**. **(3) The one RED row is a defect report, not a tolerance question** (§3.8.4) — the observable is a bare exit status and there is no number that could be moved. **(4) All three arms were proven by injection** (§3.8.6), and **five separations predicted the magnitude of their own injected failure to the digit**. |
| 2026-08-17 (Pass H) | **§3.8.1, `passh/C/7clr/…-device-corners-vs-lcms2`** | drafted as `SEVEN_CORNER = 5×10⁻⁵`, an end-to-end device bound | **WITHDRAWN**; the graded claim moved to `passh/C/7clr/pcs-corners-vs-lcms2` at `2×10⁻³ L*` and the device rows are REPORTED | `icc-conformance` | **★★ A BOUND THAT FAILED AND WAS RETIRED RATHER THAN WIDENED, and the omitted term is nameable.** It failed at `1.191176×10⁻⁴`, 2.4× over. §0's procedure: the code is not wrong — re-run on the **PCS** side, where the destination is not in the loop, the same 128 corners agree to **`4.900435×10⁻⁵ L*`**, 40× inside `ORACLE_LAB` — so the disagreement was never in the seven-channel path. **The derivation was wrong**: the destination `sRGB2014.icc` carries **1024-entry tabulated `curv`** TRCs and lcms2 inverts a tabulated curve through a **4096-entry reverse tone curve**, the term Pass 4b measured at `9.68×10⁻⁵`; the withdrawn `why`'s line *"the destination's 16-bit reverse tone curve, 1.53×10⁻⁵"* silently assumed an **analytic** inverse. **Widening would have produced a green line whose justification still did not mention the biggest thing in it.** The constant and its derivation are preserved as a comment in `passh.rs`. ★ **Third instance of this failure in this document** (Pass 4b `B6`, Pass G `SWEEP_DEVICE`, this): *when a tolerance's `why` contains a clause about a component the row does not own — a destination, a direction, a fixture property — that clause is where the missing term will be.* |
| 2026-08-17 (Pass H) | **§3.8.1 §D, three rows on `Probev2_ICCv4`** (`b2a/off-colorant-channels-are-exactly-zero`, `b2a/a-and-b-are-ignored`, `b2a/tint-is-monotone-decreasing-in-L`) | drafted graded at `0` / `1.5259×10⁻⁵` — the readme's statement taken at face value | **REPORTED (∞) on that file only**, each carrying a mandatory `★★★ THE PUBLISHED CLAIM IS FALSE OF THIS FILE` prefix in its own emitted detail | `icc-conformance` | **★★★ THE PUBLISHED CLAIM IS FALSE OF THE FILE THE PUBLISHED DOCUMENT NAMES — and that inverts the pass's expected arrangement.** The ICC's `Probe2` readme says the `BToA` tags render *"tints of pure cyan / magenta / yellow"*. That is realised **exactly** on `Probev1_ICCv2` and `Probev1_ICCv4`, which the readme does **not** describe (off-colorant channels `0.0` to the bit; `a*`/`b*` change the answer by `3.3×10⁻¹⁶`); and it is **false of `Probev2_ICCv4`**, which the readme **does** name (off-colorant maximum `0.9969`; `a*`/`b*` worth up to `0.9177`). Once a published premise is shown false, continuing to grade iccce against it grades iccce against the document's error. ★★ **They were relaxed to INFINITY, not to a finite number the observation happens to satisfy** — `0.98` chosen because the measurement came out at `0.9969` would be exactly the tuning §0 exists to prevent, and would read in a report as a claim. **What survives and IS graded on all three files is the weaker statement the sentence still entails** — *the published colorant is strictly the largest of the three chromatic channels* — observed `0` violations everywhere, and §3.8.6 shows by injection that it is the only in-process row that catches an intent-to-tag mis-wiring. **No number moved on the two files where the claim holds.** |
| 2026-08-17 (Pass H) | **§3.8.1 §D, `a2b/vs-lcms2-through-the-same-tags`** | drafted over **all** device corners at `2×10⁻³ L*` | the same bound, over corners **less those where lcms2's `L*` exceeds the tag's representable ceiling**; the excluded points get their own REPORTED row `a2b/encoded-pcs-clamp-divergence` | `icc-conformance` | **★★ PASS 4b'S SYNTHETIC FINDING, REPRODUCED ON A REAL ICC-PUBLISHED FILE — and it is why the row was split, not widened.** It failed on `Probev1_ICCv4` at `2.374×10⁻¹` while passing on `Probev1_ICCv2` at `8.8×10⁻⁴`. The code is not wrong: **iccce clamps the encoded PCS at the B curve (clause 10.18's domain, via `Trc::eval`) and lcms2 does not** (its identity curve is an analytic gamma-1 segment, evaluated unbounded) — exactly `pass4b/fixture/mab/encoded-pcs-overflow-divergence`, which is REPORTED because **which behaviour the specification requires is UNSETTLED**. Pass 4b measured it on a fixture *this project authored*, so it could have been an artefact of our own fixture design; **it is not.** ★ The split predicate is evaluated on **lcms2's** output, never on iccce's: *"the file encodes a PCS value above what this tag's encoding can represent"* is a fact about the file, and the side that does **not** clamp is the one that can still show it — splitting on iccce's own clamp fixed-point would be splitting on the behaviour under test. ★ The two `Probev1` files make the mechanism unmistakable: the **same design**, encoded once as legacy `mft2` (ceiling `100.390625`, no overflow, `0`) and once as v4 `mAB ` (ceiling `100.0`, overflow, `0.2374`). **The overflow is caused by the ENCODING, not by the data** — the ICC's own v4 re-issue of its own v1 profile stored a value the v4 encoding cannot hold. |
| 2026-08-17 (later, after Pass H) | **§3.8.1 / §3.8.4, `passh/C/7clr/compiled-path-does-not-ABORT-the-process`** | one row, tolerance **0**, observed **1 — RED** | **the same row at the same tolerance 0, now observed 0 — plus THREE new rows**: `default-grid-BUILDS-and-is-the-grid-the-library-RECOMMENDS` (0), `oversized-grid-is-a-NAMED-refusal` (0), `compiled-vs-reference-at-the-default-grid` (**REPORTED**) | `icc-conformance` | **★★★ NO TOLERANCE MOVED. THE CODE MOVED — and then the ROW had to, for a reason worth naming.** `icc-engineer` fixed the abort in `crates/iccce-cmm/src/compiled.rs` with a SIZE guard (`ChainError::GridExceedsBudget`, `MAX_COMPILED_GRID_BYTES = 64 MiB`) distinct from the `checked_pow` OVERFLOW guard, and by replacing `recommended_grid_points`' `_ => 33` catch-all with a value **computed** from the budget for ≥5 channels (`7→6`). Re-measured here rather than taken on report: `pass=274 fail=0 skip=9 error=0`, **bare exit 0**, and both `iccce bench` invocations run directly. **★ Why three rows were added rather than none.** Each of the two fixes *independently* makes the original observation zero: at grid 6 the allocation is 6.4 MiB and succeeds whether or not the guard exists, so **deleting `MAX_COMPILED_GRID_BYTES` would leave the original row GREEN**. A row that went red on a real defect had become a row that could not see that defect return, with no number moving and nobody editing it. The split puts a different **layer** in each row's loop (§3.8.4.3): the default's survivability, the default's *usability* plus recommendation-vs-behaviour agreement, and — forcing `--grid 33`, the exact configuration that died — the guard itself through the CLI, requiring exit 1, empty stdout and stderr naming all three quantities, **every one of them computed at run time from the library rather than typed**. A sixth counter fires if the budget is ever raised above that allocation, because **a row that has quietly become vacuous is worse than one that fails: it reports PASS.** **★ Two stale-prose defects fixed in the same sweep, both DL-034's shape** — the row's `detail` mixed *computed* halves (which updated themselves correctly) with *typed* narrative (which did not), and the typed half went on asserting *"checked_pow guards against WRAP, not against SIZE, so the allocation is attempted and the allocator aborts"* **on a row reporting PASS**. Also `(~0.00 TiB)` — a unit chosen for `0.93 TiB` and left behind when the value fell five orders — now `human_bytes()`, which picks the unit from the value; and a trailing `stderr: ` that was indistinguishable from a truncated field, now `(empty)`. **A `detail` string that mixes computed and typed content inherits the weaknesses of the typed part.** **★ The grading decision, declined and reasoned:** `compiled-vs-reference-at-the-default-grid` stays REPORTED for ever — both arms are iccce (self-comparison, `NUMERIC_CLAIMS.md` §1), no lcms2 n>4 geometry has been read out of the pin, ICC.1 legislates no interpolation method (A16), and n = 1 profile. §3.8.4.5 states the two conditions that would reverse it. **★ The measured 4-channel 33 was NOT shrunk to fit the budget** (§3.8.4.4), and the resulting tension is asserted in a test that fails if it ever disappears — graded here as the right call, because the failure mode of a documented exception is silent removal with the paragraph surviving. |
| 2026-08-17 (Pass I) | **§3.9, all 19 rows (first filling, not a change)** | did not exist | as recorded in §3.9 | `icc-conformance` | Pass I graded `iccce_color::adaptation_matrix` against ICC's **published** D65→D50 `chad` — the repository's third `published-ground-truth` subject and the first for chromatic adaptation. **★★★ The bound this pass was COMMISSIONED with would have failed it at 7,4×.** The brief derived the tolerance from the cone-matrix difference alone (`0,8951` vs `0,8950`, exactly `5,661 342×10⁻⁶`); the residual is dominated by a second, unmentioned term — ICC's `chad` adapts the 4-dp-**rounded** white `0,9505/1/1,0890` while iccce derives D65 from BT.709-6's chromaticities, worth `4,453 188×10⁻⁵`, **7,9×** the cone term. The two partially cancel to `4,164 937×10⁻⁵`. **No number was moved after the fact**: the complete derivation was done in exact rational arithmetic before the pass was first run, and the bounds are per-cell predictions plus one f64 allowance. Same failure shape as §3.4's `B6`, §3.7.2's `SWEEP_DEVICE` and §3.8.3's `SEVEN_CORNER` — *the missing term is in the component the row does not own*. |
| 2026-08-17 (Pass I) | **§3.9.3 row E3, `passi/E/published-colorant-rows-sum-to-D50`** | drafted at **1×10⁻⁸**, justified as *"exact arithmetic over the printed fifteen decimals gives 9,3×10⁻⁹"* | **1×10⁻⁷**, justified from §A.7's **seven-decimal print** propagated through the published `chad` | `icc-conformance` | **★ The bound FAILED on its first run at `7,946 512×10⁻⁸` and the code was not wrong — the JUSTIFICATION's source was.** `icc__s__srgb_for_icc_profiles.md` prints all three row sums of ICC's published colorants and then summarises them as reproducing D50 *"to 9,3×10⁻⁹"*; that is the **X** row's residual quoted as though it were the maximum, and the **Z** row is `7,946 512×10⁻⁸`, **8,5× larger**. The replacement bound is derived, not observed: `inv(§A.7)`'s implied white sits `1,060 763×10⁻⁷` above `1,0890` in Z because §A.7 is printed to seven decimals, and the published `chad` carries that to `7,946 512×10⁻⁸` — closing to every printed digit. `1×10⁻⁷` is the next power of ten, stated in the unit of §A.7's own print precision. **The guard keeps its power**: a mistyped digit in the third decimal of any published cell moves this by ~`10⁻³`, four orders over. A corpus summary line was corrected as a result; this is the second time a bound derived from a corpus *summary* rather than the corpus's own *printed values* has failed. |
| 2026-08-17 (Pass I) | **§5, NA-010 (first registration)** | did not exist | as recorded in §5 | `icc-conformance` | **★★ `builtin.rs` declares ONE named approximation for the built-in sRGB and there are TWO.** Its doc comment attributes the `3,02` ULP colorant residual *"entirely"* to which D65 primaries matrix each side starts from. Exact decomposition (§3.9.4): the **chad** term reaches `2,482` ULP and the **primaries** term `2,480` ULP — the same size — and on `bXYZ.Z` the `−0,897` ULP total the doc presents as evidence of closeness is a **cancellation between `−2,482` and `+1,586`**. Registered on the day it was measured, which is the standard §5 sets. The `3,02`/`11,13` comparison against the shipped HP file is unaffected. |
| 2026-08-17 (Pass K) | **§3.10, all 33 rows (first filling, not a change)** | did not exist | as recorded in §3.10 | `icc-conformance` | **★★★ AN INSTRUMENT BUILT BEFORE THE THING IT MEASURES.** Pass K grades **black preservation**, a capability `crates/` does not have at tip `506fcd3`; the numbers were fixed before anyone could see which would be convenient. Five things about *how* they were arrived at. **(1) The section's whole tolerance policy follows from one measurement: a K-only build re-separated into `ISO Coated v2 300% (ECI)` comes back carrying `7.053 20×10⁻¹` of chromatic ink while sitting `1.360 90×10⁻¹ ΔE2000` from where it started — so ΔE is BLIND to this subject** and every preservation row is in device units. Row `A4` grades that ΔE against §2's `1.0` anchor and **passes on purpose**; a suite that graded this subject perceptually would report nothing. **(2) Two bounds are two orders tighter than §3.7's `SWEEP_DEVICE` and the reason is structural, not observational** — the K-only ramp lies on an *edge* of the 4-D `A2B` hypercube where every interpolation scheme coincides, §E's off-neutral points are `A2B` **nodes**, and the `B2A` leg is trilinear on both sides, so NA-006's envelope is **identically zero** on both probe sets. What remains is the 16-bit PCS quantum, and both rows **measure the destination's response to it at run time**, making the bound a function of the fixture (§3.7.2 lesson 1). **★ `E5` is the control that earns it**: the same comparison off the nodes is `1.750×10⁻³`, **32×** larger. **(3) One row is RED BY DESIGN at a tolerance of exactly zero** — `passk/E/k-only-in-implies-k-only-out`, observed `7.053 20×10⁻¹`. *K-only means K-only*, and D3 shows lcms2's own K-only intent returns the encoded zero at every point, so any bound above zero would be an allowance for ink the requirement forbids. **The remedy is the feature, not the number.** **(4) A new row shape, the REFUTATION row** (§3.10.3): `observed = the number of corpus members for which a shortcut holds`, bounded one below the population size, so the row fails exactly when the shortcut would be defensible. It kills two shortcuts with numbers — *"use the saturation intent"* (true of **2 of 6** real CMYK destinations, both the same vendor's) and *"the ICC leg and ISO 32000-1 §10.3.3's device rule are interchangeable"* (true of the press's own black-ink gray at `0.7516 ΔE2000`, false of an ordinary gamma-2.2 gray at `12.5958`). ★ **Both were corrected on the first run**: each had been given a candidate separation naming a rival *corpus*, which made them report `BLIND` for a property they do not have. **A rival CORPUS is not a rival candidate, just as a rival TOLERANCE is not** (§3.5.8). **(5) `Intent` was NOT extended to reach lcms2's black-preserving intents 10–15.** They are vendor extensions outside the ICC intent numbering; they are reached through a separate `passk::KOnlyOracle` that builds its own argument vector and carries a mandatory `CAVEAT` string prepended to the `source` of every record built from it. **Pass K contributes `unstated = 0` and `blind = 0`.** Suite: `pass=325 fail=1 skip=9 error=0`. |
| 2026-08-17 (Pass K §F, later the same day) | **§3.10, seven NEW rows F1–F7 (§3.10.11); §3.10.1's table and separation tally; §3.10.7's owed-work note; §3.10.9's injection bullet** | did not exist | as recorded in §3.10.11 | `icc-conformance` | **★★★ CLOSING §3.10.7's OWED ITEM — A FIXTURE, NOT A TOLERANCE.** §3.10.7 recorded that the committed synthetic CMYK fixture is `ZERO-SEPARATION` for black preservation, so every graded row had to run on the **licensed** corpus and skip in CI — including the one that is red on purpose. A new `gen-profiles` recipe, **`v2-cmyk-chromatic-neutral`**, has a `B2A0` that separates a neutral into all four inks by construction: **the two candidate answers are `4.207 049×10⁻¹` apart** and all seven new rows run in CI. **Five things about how the numbers were arrived at.** **(1) SIX OF SEVEN ROWS ARE `derived-expectation`, and the harness reads the bytes ITSELF** — `Mft2Bytes` walks the tag table and decodes `mft2` in `tools/difftest`, deliberately not through `iccce-profile`, because a parser that read the CLUT wrongly would otherwise produce an expectation wrong in the same way as the observation. `F7` is the paired **third reading** (lcms2) that `Kind::DerivedExpectation`'s own documentation asks for. **(2) THREE CONSTRUCTION CHOICES REMOVE TERMS FROM THE BOUNDS RATHER THAN ALLOWING FOR THEM** — both models affine with no cross terms (so `NA-006`'s interpolation envelope, the term that forced §3.7's `SWEEP_DEVICE` to `4×10⁻³`, is **identically zero**); `B2A0` `a*`/`b*`-independent across three node lines, because `a* = 0` (`8000h` = 32 768) is **not a node** — node 4 of a 9-node axis sits at 32 767,5; and darkness defined on the **encoded** `L*` fraction, because legacy PCSLAB's top node decodes to `L* = 100.390 6` and an `L*`-based model would clamp inside the cell the K ramp's white end lands in. `F1` **grades** the dead band against the file rather than asserting it. **(3) `F3` GRADES THE SEPARATION ITSELF, against a floor of `4×10⁻²` = 10× §3.7's `SWEEP_DEVICE`, declared in advance and derived from the tolerance budget rather than the observation.** The classifier already *flags* `ZERO-SEPARATION`, and a flag is never a failure — but §F exists **because** a fixture collapsed, and a replacement that collapsed again would be the same defect under a fresh filename. **(4) PROVEN BY INJECTION, AND THE INJECTION BEAT THE DESIGN ARGUMENT.** Zeroing the `B2A0` chromatic samples — the sibling's construction — turns `F5`, the headline row, **GREEN**, and gives `F6` a transition width that looks like a working feature; `F2` and `F3` fail and are the only things that say so. A collapsed fixture does not merely fail to inform, it **manufactures a false pass**. **(5) ONE OF THE AUTHOR'S OWN SEPARATION CLAIMS WAS FALSE AND EVALUATING IT IS WHAT CAUGHT IT.** `F4`/`F7` first named DL-005's legacy-vs-general PCSLAB misreading as their rival; that misreading is **invisible** to these rows, because the derivation works in encoded fractions end to end and a **symmetric** misreading cancels exactly. The rival is now clause 10.10's **CLUT index order read backwards**, and it is **evaluated from the same committed bytes** rather than asserted: `4.843 550×10⁻¹`, `31 742×` `F4`'s bound. **★ `E1` is NOT repointed and `E6` is NOT deleted** — §F adds reach, it does not launder the red into green, and `E6`'s `ZERO-SEPARATION` verdict is the measurement that says why a second fixture had to exist. **★ §F closes the *gradeability* gap, NOT the *population* gap:** its models are affine precisely so that no interpolation envelope enters the arithmetic, which is exactly what a real profile does not give you. **★ No tolerance was widened; four were newly derived, all by counting 16-bit quanta.** Pass K still contributes `unstated = 0`, `blind = 0`. Suite: `pass=331 fail=2 skip=9 error=0`, both failures deliberate. |
| 2026-08-18 (Pass K, grading the LANDED feature) | **§3.10.12 (new); §3.10's opening; §3.10.1's table (four new rows, E7–E9 and F8, and revised observations); §3.10.5; §3.10.6; §3.10.11's tail; CI floor 22 → 23** | `E1` `7.053 20×10⁻¹` (FAIL), `F5` `4.207 050×10⁻¹` (FAIL) | `E1` `0`, `F5` `0` — **at the same tolerances, both still exactly `0`** | `icc-conformance` | **★★★ NO TOLERANCE WAS WIDENED, SOFTENED OR RE-DERIVED; FOUR ROWS WERE ADDED.** `crates/iccce-cmm/src/black_preserve.rs` landed `KMapping::EqualLightness` behind `iccce transform --preserve-black <policy>` (the policy name mandatory, no default, because two published definitions disagree by up to `4.9×10⁻²`). §E and §F were repointed at that surface. Suite `pass=331 fail=2` → **`pass=337 fail=0 skip=9 error=0`**, 44 Pass K rows; corpus-free (CI-shaped) run `pass=184 fail=0 skip=94`, of which §F contributes eight. **Five things about how this was arrived at.** **(1) ★★★ THE REPOINTING INSTRUCTION WAS RIGHT AND INCOMPLETE, AND THE GAP WAS IN THE GUARD.** Pass K's own pre-feature text named `E1` and `E3` — the rows about the *predicate* — and `E4`, the row about the *regression*, claimed of itself that a leaking preservation path *“shows up here and nowhere else”*. **That was false of `E4` as written**: black preservation is opt-in and applied never by default, so a row driving the plain surface has no preservation code in its chain to leak, and `E4` would have stayed green through any leak whatever while its own sentence vouched for the silence. `E4`, `E5`, `F4` and `F7` are now driven WITH the flag, and **`E7`/`F8` grade `max |on − off|` at exactly zero** over probes that cannot qualify. Generalisation, one level above §3.5.8's: *ask which layer is in the loop of the FIX.* **(2) ★★ `E2` STAYS REPORTED AND THE REASON IS NOW MEASURED RATHER THAN ARGUED.** On the same-press pair §E uses, the observation (`6.1×10⁻⁵`) EQUALS the named rival's distance (`6.1×10⁻⁵`) — ratio `1.0`, **`BLIND`**. A bound iccce passed, “copy K through” would pass too. Grading it would have produced a green row discriminating nothing. `E9` was added on a cross-press pair (`GWG_GenericCMYK`), where the rival sits `4.890×10⁻²` away and the observation is `3.1×10⁻⁵` — **`1577×`**. It is the only row anywhere that can say WHICH of the two definitions iccce implements, which is exactly what the mandatory policy argument promises a caller. **(3) ★★ LCMS2 IS AN INTERPOLATION OF ITS OWN CONSTRUCTION, AND THAT CHANGED WHAT AN AGREEMENT NUMBER MEANS.** Split by whether the `K` value lands on a node of lcms2's 17-node black-preserving CLUT, the residual is `1.4–3.1×10⁻⁵` at the nodes and up to `1.089 5×10⁻²` off them — `120×` to `351×`. A whole-ramp figure measures lcms2's grid density, not either party's mapping, so `E9` grades **only at the nodes**. Same shape as `E5`'s `32×` control, in a different channel. **(4) ★★★ `E8` IS A ROW WHERE THE ORACLE IS WRONG AND THE ENGINE IS RIGHT (rule 7).** On a same-profile pair the equal-lightness construction is provably the identity — algebra, with no implementation in the expectation, so the kind is `derived-expectation`. iccce observes `0.000000` against a bound of one printed unit; **lcms2 intent 11 is `6.1×10⁻⁵` away from the algebraic answer** because its `K` returns through a 17-node CLUT. Its rival is named as the oracle's own answer, because “copy K through” IS correct on this pair and would have given `ZERO-SEPARATION`. **(5) ★★ `E3`/`F6`'s GAP TO LCMS2 IS NOW A REAL BEHAVIOURAL DIFFERENCE AND IS STATED, NOT TUNED TOWARD.** iccce's K-only region is zero wide by construction (exact-zero qualifying test); lcms2's is one CLUT cell, `1/16`. **ICC.1 contains no black-preservation construct at all** (register entry A51, a closed negative), so there is no text to settle it from and rule 7's remedy does not apply. Both rows stay REPORTED permanently — tuning toward `1/16` would be adopting a vendor's CLUT resolution as a colour requirement. ★ The rows also gained a **second number**, the chromatic ink at the `C = 0` endpoint, because `0.000000` meant “there is no K-only output at all” before the feature and means “the region exists and is one point wide” after it — an observation that does not move across the change it was written to detect is a blinded row. **★ Separation coverage `44 of 44`, `unstated = 0`, `blind = 0`** — and the tally is insufficient rather than wrong: `E2`'s separation distance EQUALS its observation (ratio `1.0`), which is what a blind row is, but the classifier only reaches `BLIND` for a row with a finite tolerance and `E2`'s is infinite, so it prints `UNGRADED`. **This paragraph first claimed `blind = 1`; the emitted report falsified it the same hour** — §3.5.8.6's rule about typed numerals, arriving as a typed NOUN. |

---

## 5. Named approximations

`ARCHITECTURE.md` invariant 3: *every approximation is named and
measured*. This is the register. Each entry states the departure from
exact colorimetry, and what it costs in ΔE — a cost of "unmeasured" is
permitted only while the entry is new.

**Filled 2026-08-11**, mirroring `NUMERIC_CLAIMS.md` §4. **NA-005 and NA-007
are registered there and are deliberately not duplicated here** — this table
carries the entries whose cost is a tolerance-budget question; NA-006 was added
to it on 2026-08-11 (later) because Pass 4 turned its cost from a corpus-derived
bound into a measurement, and a register entry whose "Measured?" column changes
is exactly what this table is for. **NA-009 was added on 2026-08-12 for the same
reason and only that reason** — it had carried *"cost UNMEASURED"* through four
filings, Pass 5c measured it (§3.5.8.6), and the permission this section grants
("*a cost of 'unmeasured' is permitted only while the entry is new*") had
expired. Costs are stated
in the units they were actually bounded in — **not** converted into ΔE2000
to make the column tidy, because a conversion nobody performed is a number
nobody can check.

| # | Approximation / deviation | Where | Cost | Evidence class | Measured? |
|---|---|---|---|---|---|
| **NA-001** | **The `f(t)` breakpoint uses the exact rational form** `(24/116)³ = 0,008 856 451 679…` (and `24/116` for `f⁻¹`) where **ICC.1:2022 6.4's normative text writes the decimal `0,008 856`**. iccce's first stated deviation from normative specification text. | `crates/iccce-color/src/lab.rs` — module doc §"Named DEVIATION", `f` / `f_inv` | **~10⁻⁷ in `f`, therefore ~10⁻⁵ in `L*`** | **corpus-derived-bound** | **NO — bounded analytically in the standards corpus; iccce has NOT measured it.** No test in this repository computes the difference between the two forms. Anyone restating this must write *"bounded analytically at ~10⁻⁵, unmeasured"*, never *"measured at 10⁻⁵"*. |
| **NA-002** | **Bradford is a policy choice, not a requirement.** iccce implements the general von Kries *method* and supplies Bradford cones; ICC.1 mandates no particular chromatic-adaptation transform (corpus ambiguity **A29**, resolved *recommended, not mandated*). **Must not be described as conformance.** | `crates/iccce-color/src/adapt.rs` | **UNMEASURED, and not yet exercised** — nothing in the repository adapts anything yet (`iccce-cmm` is a stub) | — | **NO.** An entry may carry an unmeasured cost *only while it is new*; this one **becomes owed the moment Pass 3 uses it**. Measuring it means comparing Bradford against at least one other CAT over a stated sample set, in ΔE2000, on a stated illuminant pair — and **both alternatives are currently unsourceable** (von Kries/HPE digits are a corpus placeholder marked DO NOT USE; CAT02's CIE 159 is paywalled). |
| **NA-003** | **No clamping in the colour layer.** `f_inv` deliberately does not clamp below the linear segment; gamut policy is left to the CMM layer where it can be a named per-transform decision. ICC's own reference code makes negative-XYZ clamping a *compile-time option*. | `crates/iccce-color/src/lab.rs::f_inv` | **not an approximation — no ΔE cost.** A layering decision, registered so Pass 4 does not meet it as a surprise. | — | n/a. **★ The "But note" that stood here was WRONG ABOUT WHICH CLAUSE — corrected 2026-08-11, see §5.2. The original sentence is preserved there verbatim rather than edited away.** The layering decision itself is unaffected: it is still true that `iccce-color` does not clamp and that the CMM layer does (NA-004), and still true that one must not conclude from this crate's silence that iccce clamps nowhere. |
| **NA-004** | **★ Gamut clipping at the CMM layer: `pcs_to_device` clamps each linear component to `[0,1]` before the inverse TRC** (ICC.1:2022 **Annex F.8–F.16**, normative), and `iccce-cmm::curve` clamps again at two further points (clause 10.18's domain in `Trc::eval`; F.1(b)'s attainable-range clip in `Trc::eval_inverse` / `invert_table`). **This is the named per-transform decision NA-003 deferred.** It is *conformance*, not an approximation — but it has a **cost**, because two profiles' encoded gamuts rarely nest exactly, and that cost is what is registered here. | `crates/iccce-cmm/src/matrix_trc.rs::pcs_to_device`; `crates/iccce-cmm/src/curve.rs::{eval, eval_inverse, invert_table}` | **1.8788×10⁻² ΔE2000** at device white for the sRGB → Adobe RGB (1998) pair, on **25 of 133** grid points overall. Closed-form prediction from the two colorant matrices and the clamp alone: **1.8782×10⁻²** — 0.03 % agreement. Driver: the two files' encoded media whites differ by 5/2/12 units of `s15Fixed16`'s 1/65536 lsb. | **measurement** (`tools/difftest`, §13.6.3) | **YES — measured 2026-08-11**, on **one profile pair, one direction, 133 points, one platform**. **The cost is corpus-specific**: it is a property of *which two files* are being converted between, not a constant of the engine, and any restatement must carry the pair. Two profiles with identical encoded whites would show ≈0 here. |
| **NA-006** | **★ CLUT interpolation is n-linear** (multilinear; quadrilinear for a CMYK A2B), a choice inside an **ICC.1 SILENCE** — corpus **A16**: the specification says nothing whatever about how to interpolate between CLUT grid points. Registered in full in `NUMERIC_CLAIMS.md` §4 on the day the code landed, with its cost as a **corpus-derived bound of ~1 ΔE and the explicit statement that iccce had NOT measured it**. **This row exists because Pass 4 measured it.** | `crates/iccce-cmm/src/clut.rs::Clut::eval`, reached through `lut_transform::Lut16Model` | **max 1.5741 ΔE2000 (mean 0.043 86) on `USWebCoatedSWOP.icc`'s `A2B0`, and max 0.254 23 (mean 0.038 54) on its `A2B1`**, against **lcms2 2.19.1's own 4-D scheme** — which is *not* pure tetrahedral but a hybrid: linear along input channel 0, Sakamoto tetrahedral in channels 1–3 (`cmsintrp.c` `Eval4Inputs`, read at pin `21c582a`). Propagated end-to-end through the sRGB destination: **1.6639 ΔE2000** / **1.0751×10⁻² device**. **At a CLUT node the two schemes agree identically** (measured: 0.0 at all 16 corners). | **measurement** (`tools/difftest` §14.5.2), computed from the CLUT and the two algorithms alone — **no lcms2 output enters the envelope** | **YES — measured 2026-08-11**, on **one profile, two of its three A2B tags, 341 CMYK points, one platform**. Three things must survive every restatement: (1) the cost is a property of *this CLUT's curvature*, not a constant — a smoother table shows less, and the two tags in this one file differ by **6×**; (2) it is measured against **lcms2's scheme**, not against "tetrahedral" generally, and not against the true colour, which nothing here knows; (3) **~1.6 ΔE2000 is at or above the perceptibility anchor**, so this is the project's first named approximation whose cost is *visible*. |
| **NA-009** | **★★ The black-point ESTIMATION step.** BPC needs a destination black point and **no published document defines how to estimate one**; `bkpt` is untrustworthy (the corpus's own cross-verified finding) and the silence is corpus **A42**. `iccce-cmm::bpc` implements **ISO/CD 18619 4.2.5** — a **committee draft** — where lcms2 implements its own unattributed procedure. Registered in full in `NUMERIC_CLAIMS.md` §4; **this row exists because the cost stopped being unmeasurable on 2026-08-12**, exactly as NA-006 joined this table when Pass 4 measured it. | `crates/iccce-cmm/src/bpc.rs::estimate_lut_destination_black`, reached through `Chain::estimate_dst_black` and the shipped `iccce transform --bpc` | **`4,799 109 ΔE76` (100 % `L*`) on `USWebCoatedSWOP.icc`** and **`5,000 000 ΔE76` (100 % chroma) on `v4-rgb-mab-chromatic-black.icc`**, both at media-relative. At the input black these carry to **`9,921×10⁻³`** and **`5,725×10⁻²`** of device range — ~1 % of ink on the SWOP arm. **The divergence is DEFINITIONAL, not an error by either side**: both implementations return a quantity their own document calls `InitialLab`, and ISO 4.2.2.2 means the darkest device **vertex** neutralised while lcms2's `cmsDetectBlackPoint` means the **perceptual black round trip** with chroma zeroed. | **cross-check** (`tools/difftest` §3.5.8.6, `README.md` §19.10) | **YES — measured 2026-08-12**, and **four caveats travel with the number, none optional.** (1) It is a cost **at the black point only** — BPC's effect tapers away from the shadow end and **nothing here measures the taper**. (2) It is relative to **lcms2, not to truth**. (3) ★ **There is no ground truth in this comparison at all**: no published black point exists for `USWebCoatedSWOP.icc`, and 18619 is a committee draft in this project's corpus — so this reads as an implementation-cross-check throughout and **must never be promoted**, however stable it looks. (4) **Coverage: two profiles, one intent, one direction, one pin, one platform** — and the `swop` arm is the only one with any power on the clause that produced the figure (`ZERO-SEPARATION` on the other; §1.1, DL-036). The register entry's previous *"UNMEASURED"* was correct while it stood and is superseded, not deleted. |
| **NA-010** | **★★ The Bradford VARIANT, and it is a second named approximation inside the built-in sRGB construction.** `iccce_color::BRADFORD` carries `M_A[0][0] = 0,8951` because **ICC.1:2022 Annex E.3 Eq. (E.1) prints it**. **ICC's own published D65→D50 `chad` was computed with `0,8950`** (recovered by eigendecomposition; exact reconstruction leaves `5,7×10⁻¹⁶` against `5,661×10⁻⁶` for E.1's variant). Two ICC publications, two Bradford matrices — **not an error by either side and not adjudicated here**; iccce follows the printed specification. Distinct from **NA-002**, which is *Bradford at all*; this is *which Bradford*. | `crates/iccce-color/src/adapt.rs::BRADFORD`, reached through `iccce_cmm::builtin::srgb()` | **`5,661 342×10⁻⁶` (`0,371` ULP of `s15Fixed16`) in the adaptation matrix, isolated.** In the shipped sRGB colorants the chad term reaches **`2,482` ULP** — the same size as the primaries term `builtin.rs` names, and on `bXYZ.Z` the two **cancel** to the `−0,897` ULP that doc comment presents as a small error (§3.9.4). Total shipped-colorant residual against ICC's published values: **`4,607 402×10⁻⁵` = `3,020` ULP**, graded by `passi/E`. | **measurement** in exact rational arithmetic over published constants (`tools/difftest/src/passi.rs`, §3.9), **no implementation's output in it** | **YES — measured 2026-08-17, on the day it was registered.** Four things travel with it. (1) It is an **XYZ-cell** difference; **no ΔE anywhere in Pass I**, and the perceptual cost is unmeasured. (2) **Adopting ICC's `0,8950` would make the colorant row WORSE, not better** — measured `4,686 594×10⁻⁵` against iccce's `4,607 402×10⁻⁵` — because the two error terms currently cancel; *this is not a defect with a known fix.* (3) The `chad` residual against ICC's recommendation is **dominated by the white point, not by this** (`4,453×10⁻⁵` vs `5,661×10⁻⁶`), so quoting NA-010 as the explanation of the `2,730` ULP `chad` difference is wrong by 7,9×. (4) **One illuminant pair, one direction, one machine.** |

### 5.1 Why NA-001's cost cannot be compared to §2's anchor, even though it is tempting

~10⁻⁵ in `L*` is roughly **five orders of magnitude below** the 1.0 ΔE2000
perceptibility anchor, so it is easy to write "negligible" and move on.
Two reasons not to leave it at that:

1. The anchor is itself ⚠ **provisional** (§2). A comparison against a
   provisional yardstick inherits the ⚠.
2. **Where NA-001 *can* show up is not perceptual at all**: bit-exact
   round-trip comparisons against an implementation that uses the decimal
   form. That is the only place it will ever appear, and it is precisely
   the kind of comparison a difftest makes. If a future difftest sees this
   deviation at a magnitude ~5 orders larger than the bound, the right
   conclusion is **a different bug**, not a wrong breakpoint.

An unstated approximation is indistinguishable from a bug. An
approximation stated with an unmeasured cost is at least an honest one —
provided the word "unmeasured" survives every restatement, which is what
the "Measured?" column above is for.

### 5.2 ★ NA-003's clause citation was wrong — the correction, 2026-08-11

**This subsection is an append, not an edit.** NA-003's own row now points
here; the sentence it used to carry is reproduced below in full, because the
history of a wrong citation is the only defence against re-making it.

#### What NA-003 said

> **But note**: ICC.1:2022 6.4 normatively requires out-of-range colours to be
> *"clipped on a per-component basis"* on integer conversion (and no clipping
> for float32 encodings). **That binds the CMM and profile layers, not this
> crate** — do not conclude from `iccce-color`'s silence that iccce clamps
> nowhere.

It was written from recollection of clause **6.4** and was then **relied on**:
`tools/difftest/README.md` §13.4 quoted it to raise a working hypothesis that
lcms2's out-of-`[0,1]` float device output might be *conforming* and iccce's
clamp *merely stricter*.

#### What the specification actually says

Settled by `icc-spec-librarian`'s fifth pass —
`ICC_Spec\icc\icc__s__computational_models.md` **§4** (ambiguity **A39**,
resolved) and §4.2 — from the primary text:

1. **Clause 6.4 is titled "Converting between PCSXYZ and PCSLAB encodings",
   and every quantity in it is a PCS value.** Its float32 sentence reads, in
   full: *"When converting to float32Number-based encodings, conversion
   between PCSXYZ and PCSLAB is performed and encoded using the float32Number
   encoding of PCS values as defined in 6.3.4.2. No clipping is performed."*
   **The words NA-003 recalled are real; the subject is not.** They are about
   the PCS, not about device values.
2. **The clause that governs device encoding is 6.5**, and its permission is
   **doubly gated**: it applies *"when encoding using float32Number values in
   **DToBx and BToDx** tags"*. Clauses **8.3.3 / 8.4.3** do not list
   `DToBx`/`BToDx` among the tags a three-component matrix-based profile may
   contain — while 8.3.2 / 8.4.2 list them explicitly for the N-component
   LUT-based classes. **The escape hatch is structurally unreachable from a
   matrix/TRC model**, not merely out of scope for it.
3. **Therefore a conforming F.8–F.16 evaluation cannot emit a device value
   above 1,0**, by entailment rather than by a separate output-clamp rule:
   F.8–F.16 hands `TRC⁻¹` an argument in `[0,1]`, and `TRC⁻¹` returns a value
   in the curve's **domain**, which 10.6 (`curveType`, declarative voice) and
   10.18 (`parametricCurveType`, `shall` twice) fix at `[0,0 1,0]`.

#### What that does to §13.4's finding

**It inverts its direction.** lcms2's measured 1,000 120 is not an unclamped
*output* that the standard might permit; it is **arithmetically unreachable
from the model**, because for that profile's γ = 2,199 curve `TRC⁻¹[1] = 1`
exactly and `TRC⁻¹[y] = y^(1/γ) ≤ 1` for every `y ∈ [0,1]`. It is evidence
that the **input** clamp (F.10/F.13/F.16) was not applied at all. **iccce is
not "stricter"** — there is no stricter available; it is on the interval the
mandated model entails.

#### Two hedges that must survive every restatement of this

1. **"Non-conforming CMM" is not a sentence ICC.1 supports** (**A39b**).
   Clause 5's entire conformance requirement on a consumer is *"shall have the
   ability to **read** the profiles as they are defined"*. The
   computational-model `shall`s at 8.3.3/8.4.3 are phrased about the
   **profile** — they fix what the data *means*, not what a CMM must compute.
   So the precise verdict is: **lcms2's output on that path is not the colour
   the profile denotes under the model ICC.1 makes mandatory for it, and
   ICC.1's conformance clause is too weak to convert that into a conformance
   failure.** The word is **divergence** (`CLAUDE.md` rule 7).
2. **The v2 half is UNSOURCED** (**A39c**). Both profiles in the measurement
   are **v2.1.0**; ICC.1:2022 specifies **4.4.0.0**. Whether the v2
   specification states the same three-branch clamp **has not been read**,
   because ICC.1:2001-04 has not been obtained. Annex F's text is
   version-neutral (it never mentions a profile version) and the corpus treats
   it as applying to both — **an assumption, labelled as one**. If v2 lacked
   the clamp, an argument exists that lcms2 is faithful to the version the
   file declares.

#### What is still unmeasured, and it is not small

**The size of the divergence under genuine out-of-gamut input is unknown.**
Every excursion §13.4 observed — 8 of 399 components, max 1,000 120 — is
**1-lsb boundary residue at white**, because sRGB ⊂ Adobe RGB makes real
clipping impossible in that direction. A destination *smaller* than the source
would drive the F.10 branch hard, and **has not been run**. Pass 4 does not
close this either: on SWOP → sRGB, which does clip genuinely, `transicc`
returned **0 of 1023 components** outside `[0,1]` at every intent — that
destination's TRC inverse is a *tabulated* reverse curve, which is lcms2's
saturating path (corpus M3). **So the observed cost of this divergence remains
≤1,2×10⁻⁴ device units at white, and that number must not be restated as a
bound on the divergence in general.**

---

## 6. Coverage — say the scope or say nothing

**"Verified" without scope is the claim this document exists to
prevent.** Every conformance statement must carry: how many profiles, of
which classes, at which intents, on which platform.

Current coverage, stated honestly, as of **2026-08-11 (after Pass 5)**:

| Pass | Status |
|---|---|
| 0 | oracle pinned, built (**Windows/MSVC only**) and smoke-tested on **2 profiles, 1 direction each**. A Rust harness drives it (`tools/difftest/README.md` §11); its own registered check is **oracle-reproducibility — both sides are lcms2**. |
| 1 | `iccce-color`: **1 correctness claim** (ΔE2000, 34/34 published pairs) and **16 arithmetic identities** (§3.1). Reported passing on **one machine, Windows 11 Pro 10.0.26200 / MSVC, `f64` throughout**. **No Linux run has been observed by anyone.** |
| 2 | `iccce-profile`: parsing records exist in `NUMERIC_CLAIMS.md`; **§3.2 of this document is still blank** and no tolerance here grades the parser. |
| 3 | **`iccce-cmm` matrix/TRC: 5 graded rows + 2 reported-only means (§3.3), run 2026-08-11.** Scope in the next paragraph. |
| 4 | **`iccce-cmm` `lut16` A2B → matrix/TRC: 8 graded rows + 4 reported-only (§3.4), run 2026-08-11 (later), at all four intents.** **A2B direction only** — the B2A half of the done-when is not measured. Scope below. |
| **4b** | **`lut8` B2A, the v4 `mAB `/`mBA ` element pipeline, and the F.2 grayTRC model: 23 graded rows + 5 reported-only (§3.4.4), run 2026-08-11 (later still).** **Two intents** (perceptual and media-relative) in §A and §C, **one** in §B; **saturation and ICC-absolute are not run anywhere in Pass 4b**. The v4 claims rest on **one synthetic fixture** because a 40-profile sweep of this machine found **zero** `mAB `/`mBA ` tags. Scope below. |
| **5** | **Black point compensation: 21 graded rows + 5 reported-only (§3.5), run 2026-08-11 (Pass 5).** Six scenarios, derived from both implementations' sources before anything ran. **The scaling map is graded against a clause of ICC.1:2022 (6.3.4.3) and against a published paper's two constraints; the ESTIMATION is not graded against anything and cannot be (A42).** ★ **No row discriminates the two implementations' black-point estimators** — see §3.5.1. **Perceptual only** where BPC is active. Scope below. |
| 6–8 | not started |
| **G** | ★ **The Ghent v5.0 population sample: 72 rows (§3.7), run 2026-08-17 at tip `e21154c`.** The first differential grading whose inputs are profiles a **real document producer embeds**. **All four intents, ±BPC, on five pairs**, plus a full Pass-4-style treatment of a **vendor-authored v4 `mAB `** profile — which closes §3.4.3's "any **real** v4 LUT profile" gap, open since 2026-08-11. **Every row states a candidate separation** (Pass G contributed 0 to the suite's `unstated` count); `blind=0`. **Compatibility, not certification** — nothing here is proofed or measured with an instrument, and Pass G has **no** ground-truth row and cannot have one. ★★ **Its corpus is LICENSED and cannot be committed**: resolved through `$ICCCE_PRIVATE_FIXTURES` and **skipped, with a reason, everywhere else — permanently including CI**. Scope in §3.7.7. |
| **H** | ★★★ **Acceptance and refusal over the ICC's own published profile set: 51 rows (§3.8), filed 2026-08-17 at tip `e21154c` with 48; three added the same day.** **50 files, 40 accepted, 10 refused.** Subject is NOT a colour value and cannot be (DL-041). Carries the **first `Kind::GroundTruth` rows in `tools/difftest`**, from the ICC's published `Probe2` readme — and ★★★ **that published statement is FALSE of the file it names**, so three rows are REPORTED with a mandatory `THE PUBLISHED CLAIM IS FALSE` prefix rather than graded (§3.8.2). ★★★ **One row was deliberately RED and was a defect report — `iccce bench` ABORTED the process on a 7-channel source. It is FIXED in `crates/iccce-cmm` and the row is now green, with no tolerance moved because there was no number to move** (§3.8.4). **The row was then SPLIT INTO FOUR**, because each of the two fixes independently satisfied the original observation and deleting the size guard would have left it green (§3.8.4.3). ★★ Two divergences with lcms2, **both conformant**: `mpet` tag selection under clause 8.10.2 worth `33.13 L*` (§3.8.5), and the encoded-PCS clamp, which Pass 4b found on a fixture we authored and this reproduces on a real ICC file. **Proven by three injections; five separations predicted their own failure magnitude to the digit** (§3.8.6). ★★ **Corpus LICENSED and uncommittable**; skips everywhere else including CI, permanently. Scope in §3.8.8. **Suite: `pass=274 fail=0 skip=9 error=0`, bare exit 0.** |
| **K** | ★★★ **Black preservation — the instrument was built BEFORE the feature and the feature then landed against it. 44 rows (§3.10); 33 filed 2026-08-17 at tip `506fcd3`, seven more (§F) the same day, four more on 2026-08-18 when the capability shipped.** ★★★ **NO TOLERANCE WAS EVER WIDENED FOR IT**: the two rows that were red by design (`E1` `7.053 20×10⁻¹`, `F5` `4.207 050×10⁻¹`) are `0` at the same bound of exactly `0` (§3.10.5, §3.10.12). ★★★ **Its central finding is still about the INSTRUMENT: ΔE2000 is blind to this subject** — the contaminated build sits `1.360 90×10⁻¹ ΔE2000` from the K-only build it should have been — so every preservation row is in **normalised device units** and the one ΔE row passes on purpose (§3.10.0). ★★ **The repointing found a guard that could not see the defect it named** (§3.10.12.2): `E4` claimed a leak *“shows up here and nowhere else”* while driving the surface with the feature switched OFF. Four rows repointed; `E7`/`F8` now grade on/off bit-identity at exactly zero. ★★ **`E2` cannot discriminate and that is MEASURED, not conceded** — on a same-press pair the two published definitions coincide to `6.1×10⁻⁵`, exactly the observation, so its separation ratio is `1.0`; the classifier prints `UNGRADED` rather than `BLIND` only because the row's tolerance is infinite. `E9` grades a cross-press pair at `1577×` separation and is the only row that can say WHICH definition iccce implements (§3.10.6, §3.10.12.5). ★★ **`E8` is a rule-7 row in iccce's favour**: on a same-profile pair the construction is provably the identity, iccce is exact and **lcms2 is `6.1×10⁻⁵` wrong** (§3.10.12.4). ★★ Two shortcuts refuted with numbers: *“use the saturation intent”* holds on **2 of 6** real CMYK destinations, both the same vendor's; *“the ICC leg and the PDF device rule are interchangeable”* **fails at `12.5958 ΔE2000`** on an ordinary gray (§3.10.3, §3.10.8). ★ **`Intent` was not extended** — lcms2's intents 10–15 are vendor extensions, quarantined behind `passk::KOnlyOracle`. ★★ **§A–§E's corpus is LICENSED and skips in CI permanently**; §F's **eight** rows run there on a committed `gen-profiles` fixture whose two candidate answers are **`4.207 049×10⁻¹` apart**, and §F's file arms are **PROVEN BY INJECTION** — which showed a collapsed fixture does not merely fail to inform, it turns the headline row green. ★ **`E1` was NOT repointed at the committed fixture and `E6` was NOT deleted**; §F closes the *gradeability* gap, never the *population* gap. **`unstated = 0`, `blind = 0`** (§3.10.6). **Suite: `pass=337 fail=0 skip=9 error=0`; corpus-free `pass=184 fail=0 skip=94`.** |

**Scope limits that must travel with any Pass G "verified"** — full record in
§3.7.7:

- **11 of 20 corpus profiles touched, 9 not.** Three vendors (Adobe, ECI,
  X-Rite) and one workgroup (GWG).
- **One destination CMYK profile for four of the five sweep pairs**; **one
  machine**, **one oracle pin**, **one day**, **one tip**.
- **The `mBA ` (B2A) direction of the X-Rite v4 profile is NOT graded** — §A
  covers its `A2B` only, and `B2A0`'s tabulated 4096-entry B curve is a shape
  nothing in this suite evaluates.
- **Eight `--bpc` combinations are refused by name by iccce and are therefore
  not differentially tested at all.** The refusals are graded as deliverables;
  the *conversions* behind them are not measured.
- **§B claims no agreement** — there is no attribution row for an `mft2` B2A,
  so its rows detect structural error against a named rival and nothing more.
- **`gamt` and `gbd*` are untouched** (iccce implements no gamut tags), as is
  the display profile carrying `vcgt`/`mmod`/`ndin`.

**Scope limits that must travel with any Pass 1 "verified":** adaptation
is exercised in **one direction (D65 → D50), one sample vector**; the
round trips use **1–2 samples each**; `Mat3::inverse` is tested on **one
matrix, and it is not a colour matrix**; there is **no ground-truth row
for chromatic adaptation at all**; and **no comparison of any kind against
lcms2 exists in `iccce-color`.**

**Scope limits that must travel with any Pass 3 "verified"** — full record
in `tools/difftest/README.md` §13.7–§13.8:

- **One profile pair.** Windows system `sRGB IEC61966-2.1` → `Adobe RGB
  (1998)`. Both **v2.1**; **no v4 profile is exercised at all**, so the
  version-gated lcms2 behaviours in §6.1 are *avoided*, not tested.
- **One intent** (media-relative colorimetric), **one direction**. Perceptual
  and saturation were deliberately not compared (§6.1 item 2); the reverse
  direction was spot-checked by hand and is **not** in the suite.
- **133 grid points**, deterministic; **nothing below 1/16 device except
  exact zero**, which is precisely where §3.3 row 1's device-space tolerance
  is least transferable.
- **No genuinely out-of-gamut input.** sRGB ⊂ Adobe RGB in chromaticity, so
  the clip path is exercised only by 1-lsb white-point excursions, not by a
  real gamut clip.
- **No LUT profile, no CMYK, no grey, no `chad`, no absolute colorimetric.**
- **One platform, one lcms2 build** (Windows 11 Pro 10.0.26200 / MSVC; lcms2
  2.19.1 at pin `21c582a`), **one `iccce` build** (release, commit `051707f`).
- **Both profiles are category (c)** (`LEGAL.md` §3): read locally, never
  committed, absent on the Linux runner. **Every Pass 3 check skips there**,
  and the runner exits **3 (nothing ran)**, not 0. There is **no synthetic
  Pass 3 fixture** because `tools/gen-profiles` does not exist.
- **No ground-truth row exists for Pass 3.** §3.3.3 records that as the
  largest evidential hole: nothing yet compares a matrix/TRC transform to a
  published value, only to lcms2 and to itself.

**Scope limits that must travel with any Pass 4 "verified"** — full record in
`tools/difftest/README.md` §14.4 and §14.8:

- **One profile pair.** `USWebCoatedSWOP.icc` → the Windows system
  `sRGB IEC61966-2.1`. Both **v2.1**, both category (c). **No v4 profile and
  no synthetic fixture** — every Pass 4 record reads a system profile, so all
  of them skip off this machine and the runner exits **3 (nothing ran)**.
- **The A2B direction only.** SWOP's `B2A*` tags are `mft1` (`lut8Type`) and
  are **not evaluated**; the destination is matrix/TRC because the sRGB
  profile has no `B2A*` at all — which was **checked in the run**, not
  assumed, because a LUT destination would have made every ΔE a comparison of
  two different *models*.
- **All four intents — of the A2B tags.** Two distinct tables:
  `A2B1` (colorimetric, and the one absolute uses) and `A2B0` = `A2B2` (one
  shared block of tag data, so perceptual and saturation are byte-identical
  transforms and are measured as exactly equal).
- **341 deterministic CMYK points**, running to **400 % total ink** — beyond
  what SWOP separations use, deliberately, which means **the mean over this
  grid is not the mean over printable colour**.
- **The dominant disagreement is a named approximation, not an error**
  (NA-006, the CLUT interpolation scheme), so §3.4 rows 1–3 are **structural
  gates that cannot claim agreement**. The agreement claim is rows 4–5.
- **At ICC-absolute the two implementations disagree by 11.217 ΔE2000** for a
  reason that is understood and unsettled (destination media white; corpus
  **A4b** unverified). That intent's raw rows are **reported, not graded** —
  the only place in the suite where a known disagreement is deliberately
  ungated, and it is labelled rather than absorbed.
- **One platform, one lcms2 build** (Windows 11 Pro 10.0.26200 / MSVC; lcms2
  2.19.1 at `21c582a`), **one `iccce` build** (release, commit `b3f4388`).
- **No ground-truth row exists for Pass 4** (§3.4.3), and **no instrument
  check was run on the sRGB destination model** — Pass 3's record 7 bounds the
  ruler on *Adobe RGB* and Pass 4 inherits that bound rather than re-measuring
  it on the profile it used.

**Scope limits that must travel with any Pass 4b "verified"** — full record in
`tools/difftest/README.md` §15.6:

- **Three unrelated corpora, three scopes, and no shared statement.** §A is one
  system profile pair (`sRGB Color Space Profile.icm` → `USWebCoatedSWOP.icc`,
  both v2.1, both category (c)); §B is **one synthetic fixture**; §C is one
  system gray profile into the same system sRGB. A sentence that says "Pass 4b
  verified the B2A direction" without saying which of the three is a claim
  about a corpus that does not exist.
- **Two intents in §A and §C, one in §B.** **Saturation and ICC-absolute are
  not run anywhere in Pass 4b.** `B2A2` is a third distinct table and is not
  touched; ICC-absolute through a *LUT destination* — where D.6/D.7 is applied
  before the PCS is encoded rather than after — has never been measured at all.
- **The v4 element pipeline rests on one file, and there is no alternative on
  this machine.** All **40** `.icc`/`.icm` files in the Windows colour
  directory were parsed and searched: **zero** carry `mAB ` or `mBA `. The only
  v4 profile with a LUT (`BlackWhite.icc`) carries an `mft1`.
- **§B's four derived rows are `derived-expectation`, not ground truth**
  (§3.4.4.1). They are defeated if `ICC_Spec`'s transcription of 10.12/10.13 is
  wrong, because the fixture and the derivation share it. **§3.4.3's
  published-value row is still blank and Pass 4b does not close it.**
- **10 of §B's 128 CMYK points are excluded from every graded row** — the
  encoded-PCS overflow (§3.4.4 row B7), where iccce and lcms2 differ by 0,61
  ΔE2000 and the specification question is unsettled. That exclusion is stated
  on the records themselves, and the excluded set is reported ungated rather
  than absorbed.
- **§A and §C skip entirely without the Windows colour directory**; only §B's
  four derived rows survive there. The runner still exits **3 (nothing ran)**
  if everything skips, but for the first time it would not: four rows would
  run, which is a change in what a green CI line means and is worth knowing.
- **iccce is IN-PROCESS on six rows** (§A's two PCS-side rows and §B's four
  derived rows), because neither a Lab input nor a Lab output exists at the
  shipped CLI. Those rows grade the **model**, not the binary, and say so.
- **One platform, one lcms2 build** (Windows 11 Pro 10.0.26200 / MSVC; lcms2
  2.19.1 at `21c582a`), **one `iccce` build** (release, commit `97ad9fa`).

**Scope limits that must travel with any Pass 5 "verified"** — full record in
`tools/difftest/README.md` §16.7:

- **★ THE ESTIMATORS ARE NOT COMPARED, AND THAT IS THE FIRST THING TO SAY.**
  Every scenario in reach has both implementations arriving at the **same** black
  point — zero on a matrix/TRC or gray side, the A41 triple on a v4 LUT side at
  perceptual — so Pass 5 grades the **scaling map**, the **direction** and the
  **policy**, and nothing it contains discriminates iccce's named subset from
  lcms2's four estimation methods. **lcms2's methods 3 and 4 (the ink round trip
  and the least-squares quadratic fit) are untested against anything**, because
  iccce refuses rather than implementing them. This is a property of the corpus,
  not of the apparatus: every profile in reach has `trc(0) = 0`.
- **One synthetic v4 fixture and one system sRGB profile carry S2 and S3**, which
  are the only two scenarios where BPC does anything at all. §3.4.4.5's finding
  stands — no real `mAB `/`mBA ` profile exists on this machine.
- **One intent where BPC is active.** Perceptual only. **Saturation is not run**
  (lcms2 forces BPC there too, and iccce's subset admits only perceptual for a
  LUT side, so the arm has no iccce half). ICC-absolute appears **only** as a
  refusal.
- **10 of S2's 128 CMYK points are excluded from every graded row**, the same
  encoded-PCS overflow §3.4.4 row B7 excludes and for the same reason.
- **Two of the six scenarios need neither a system profile nor the oracle** (§A's
  five map rows, and S6's refusal) — the first graded rows in this suite that
  survive a machine with no colour directory *and* no lcms2 build. The other
  four skip with a reason.
- **Two rows are REPORTED, NOT GRADED and both have named settling documents**:
  the forcing policy (**3,137 348 `L*`**, needs `AdobeBPC.pdf` / WP40 /
  ISO 18619) and the A41 constant (**0,050 ΔE2000**, needs ICC.1 to say which of
  Table 16's two representations governs).
- **lcms2's `0,002` empty-layer threshold is READ, not RUN.** No pair in reach
  has blacks close enough to trigger it, so the derived "lcms2 does no BPC below
  ≈0,41 `L*`" is a solution of its own inequality, not an observation.
- **One platform, one lcms2 build** (Windows 11 Pro 10.0.26200 / MSVC; lcms2
  2.19.1 at `21c582a`), **one `iccce` build** (release, commit `46f16e8`).
- **No ground-truth row exists for Pass 5 and none can today** (**A27**/**A42**):
  no normative BPC text has been obtained, so there is nothing published to grade
  a BPC *result* against. §A's rows are the strongest available — a
  primary-specification clause for the **map**, and a peer-reviewed paper for the
  **applicability set**.

The Pass 0 smoke test is recorded in `tools/difftest/README.md` §8. It
used `sRGB Color Space Profile.icm` and `USWebCoatedSWOP.icc` from the
Windows colour directory — **category (c) under `LEGAL.md` §3, read
locally, never committed, and absent on the Linux runner.** On a runner
without them the harness exits **3 (nothing ran)**, not 0.

### 6.1 Two measured findings about lcms2 that will shape later tolerances

Recorded here, not only in `tools/difftest/README.md` §12, because both
change what a future cross-check tolerance is measuring:

1. **The legacy PCSLAB encoding selector.** Measured on four synthetic
   profiles differing only in the version word: lcms2 2.19.1 applies the
   **legacy** encoding to an `mft2` Lab tag **regardless of profile
   version**, keying on the tag type exactly as ICC.1:2022 6.3.4.2 NOTE 3
   and 10.10 require. The divergence `ARCHITECTURE.md` DL-011 predicted
   between iccce and lcms2 **does not exist on this pin**, and §3.4's two
   Lab-encoding rows stay **ground truth** regardless — they must be
   settled from the specification, not from the agreement.
2. **lcms2 forces black point compensation on for v4 profiles at
   perceptual and saturation**, on the authority of an Adobe document
   rather than ICC.1, whether or not `-b` was passed. Confirmed
   quantitatively (predicted `L*` matches observed to 3×10⁻⁵). **Any
   §3.4 or §3.5 tolerance set at perceptual or saturation against a v4
   profile is measuring a transform with BPC in it.** A tolerance set
   without knowing that is a tolerance set on the wrong quantity, and the
   disagreement it would absorb is ≈3.15 `L*` at black — nothing like
   sub-perceptual.

### 6.2 Two things Pass 3 found that are worth carrying forward

1. **lcms2 quantises tabulated tone curves to 16 bits even in its float
   pipeline** (`cmsEvalToneCurveFloat` rounds a segment-free curve's input
   *and* output to 1/65535). This accounts for **essentially all** of the
   iccce-vs-lcms2 disagreement on this pair: modelling it shrinks the
   device-space residual from 6.71×10⁻⁵ to **2.31×10⁻⁷**, a factor of 290 and
   below `transicc`'s own print floor. **Any future cross-check whose source
   profile has sampled `curv` TRCs is measuring this**, and a tolerance set
   without knowing it is set on the wrong quantity — the same shape of
   problem as §6.1's forced BPC.
2. **lcms2 returns device values outside `[0,1]` in float** when the
   destination TRC's inverse is analytic (up to 1.000 120 here), and
   saturates when it is tabulated — an artefact of which inversion path it
   took, not a stated range policy. iccce clamps, per Annex F.8–F.16.
   ~~**Recorded as a FINDING; the specification question is OWED to
   `icc-spec-librarian`**~~ **★ SETTLED 2026-08-11 (later) — and the answer
   went against the hypothesis that had been built on NA-003's wrong clause
   citation. See §5.2 in full.** A conforming F.8–F.16 evaluation **cannot**
   exceed 1,0, so iccce is not "stricter" and lcms2's excursion is evidence
   its *input* clamp was skipped. Two hedges survive: clause 5 binds a CMM only
   to **reading** profiles (A39b), so the word is **divergence**, not
   non-conformance; and the **v2** half is **unsourced** (A39c). The size of
   the divergence under genuine out-of-gamut input is **still unmeasured** —
   Pass 4 did not close it either (0 of 1023 excursions on SWOP → sRGB, whose
   destination inverse is tabulated).

### 6.3 Three things Pass 4 found that are worth carrying forward

1. **lcms2's 4-D CLUT interpolation is not tetrahedral.** For four inputs it
   is a **hybrid** — linear along input channel 0, Sakamoto tetrahedral in
   channels 1–3 (`cmsintrp.c` `Eval4Inputs` / `Eval4InputsFloat`, pin
   `21c582a`) — and therefore **not symmetric in the four inks**. Worse for a
   naive tolerance: an `mft2` tag is read into a **16-bit** CLUT stage, whose
   float evaluator quantises the stage input to `u16` and runs the
   **fixed-point** twin. **Any tolerance derived from published
   trilinear-vs-tetrahedral figures is derived from the wrong algorithm.**
2. **lcms2 substitutes D50 for the `wtpt` of a v2 display-class profile**
   (`cmsio1.c` `_cmsReadMediaWhitePoint`), regardless of what the tag says.
   Where the tag holds D65 — common in v2 monitor profiles, including the
   Windows system sRGB one — that is a **32 % difference in `Z`** and worth
   **11.2 ΔE2000** at the absolute intent. **Any absolute-colorimetric
   cross-check against a v2 display profile is measuring this**, and a
   tolerance set without knowing it is set on the wrong quantity — the same
   shape of problem as §6.1's forced BPC and §6.2's tone-curve quantisation,
   and the third instance of it in two passes.
3. **A control that switches the dominant term OFF is worth more than a
   tighter gate.** Pass 4's 16 CLUT-node corners cost nothing to add, agree to
   `transicc`'s print floor (6.7×10⁻⁵ ΔE00), and are what make a 2.0 ΔE00
   structural gate defensible instead of embarrassing. The general form:
   **when a known non-error dominates a comparison, find the subset of the
   corpus where it is identically zero and grade that subset separately.**

### 6.4 Four things Pass 4b found that are worth carrying forward

1. **★ lcms2's interpolation geometry depends on the DIRECTION, not just the
   channel count.** `_cmsReadOutputLUT` calls `ChangeInterpolationToTrilinear`
   on **every CLUT stage** of any profile whose PCS is `Lab ` — so in the
   **B2A** direction lcms2 is *trilinear*, i.e. n-linear, i.e. iccce's choice,
   and §6.3 item 1's warning does not apply. The same file, the other
   direction, opposite answer. **NA-006's cost is therefore ~1,6 ΔE2000 in the
   A2B direction and identically ZERO in the B2A direction**, and any statement
   of that cost that does not say which direction is meaningless. lcms2's own
   comment calls it "controversial stuff" and gives a rationale rather than a
   clause: it is a **policy**, so the agreement is between two choices, not
   conformance.
2. **★ Forced BPC is decided by the DESTINATION profile's version.** §6.1 and
   corpus M2 record that lcms2 forces BPC on v4 profiles at perceptual and
   saturation. Measured in both directions on one pair: `BPC[i]` is consumed by
   `ComputeConversion(i, …)`, the conversion **into** `hProfiles[i]`, so a v4
   *source* into a v2 destination sets a flag nothing reads — **0,0,
   bit-identical** — while the reverse direction moves `K` at black by 3,1 %.
   **Anyone using M2 to decide whether a comparison is confounded needs the
   direction, not just the version.**
3. **★ A tolerance of `0,0` is only available when the two sides are the same
   operations in the same order.** "Both algorithms reproduce an affine
   function exactly" is a true statement about **ℝ** and a false one about
   `f64`: the two reach the same value by different sequences of operations and
   agree to ~16 ulp. A derivation that ends in the word *exactly* must say
   whether it means exact in the algebra or exact in the arithmetic. (§3.4.1
   row 6 and §3.4.4 row C5 legitimately mean the latter — the same bytes
   through the same code — and both still observe 0,0.)
4. **★ Where in a comparison the maximum sits is part of the derivation, not a
   detail.** §3.4.4 row C3's first tolerance was derived at white and failed at
   the dark end by 2,2×: below sRGB's linear breakpoint a *device* difference
   is amplified into `a*`/`b*` by `da*/dX = 4038 / 12,92 = 313` while `L*` gets
   only 69,9, so the ΔE maximum is **chromatic and near black** even on a
   neutral axis. This is the mirror image of §6.2's "near black the device
   metric explodes while ΔE stays small" — both are true, of different
   comparisons, and **which one applies depends on which side of the inverse
   TRC the difference is measured**.

### 6.5 Four things Pass 5 found that are worth carrying forward

1. **★ Derive the comparable scenario set from both sides' sources BEFORE
   measuring, and publish the negative result it produces.** Pass 5's most
   important finding is a *prediction*: everywhere iccce does BPC, lcms2's
   estimator reduces to the same two values, **so no cross-check row can
   discriminate the two estimators** (§3.5.1). A session that had measured first
   would have found six small numbers and read them as six independent
   agreements about "BPC". The general rule: **when two implementations agree,
   ask what they were free to disagree about**, and answer it from their sources
   rather than from the size of the residual. Its companion is the older lesson
   this document already carries — an arm-comparison that comes back null may be
   null *by construction* — and §3.5's rows P17 and P18 are labelled that way.
2. **★ A tolerance can legitimately be an EARLIER PASS'S ENVELOPE, provided the
   derivation says which term it inherited and the run then prices that term.**
   BPC inserts one matrix stage between two stages that were already in the
   pipeline: no table lookup, no `u16` rounding, so no new quantisation of the
   kind Pass 4b's envelopes model. What *does* change is **where on the axis the
   pipeline operates**, and row P6b said so before the run. The observation moved
   the residual by 1,097× where the map's gain alone predicts 1,0035 — the
   flagged term is real and worth ~9,4 %. **A derivation that names its weakest
   assumption and is then vindicated on it is worth more than one that omits
   it**, and this is the shape to reuse whenever a new pass extends an old
   pipeline rather than replacing it.
3. **★ A direction test that reads the same in both directions is not a
   direction test.** §3.5 row P13's first draft failed because "the PCS rises"
   and "the PCS falls" both become "no device component may rise" once the
   destination is CMYK, whose `K` runs opposite to `L*`. A *sign* is cheap and
   exact — `out − in = (Xd − Xs)/(Xi − Xs)·(Xi − X)` needs no tolerance at all —
   but only a **magnitude against a closed form** (row P14) shows that the two
   directions are different things. **Grade the sign for free; grade the size to
   mean anything.**
4. **★ Ratio the effect to the disagreement, and print it.** "iccce and lcms2
   agree to 1,1×10⁻⁴" says nothing until it sits beside "BPC itself moves this
   transform by 3,5 ΔE2000". Pass 5's two cross-check rows are **388×** and
   **682×** more sensitive than the effects they grade, and that ratio is the
   same argument §3.4.4 row A5's tetrahedral counterfactual makes for
   interpolation geometry — with the advantage that here it is free, because the
   BPC-off arm is already being run as the baseline. **Every future agreement
   claim should carry its sensitivity ratio**; a comparison that cannot state one
   has not shown it could have failed.

### 6.6 ★★ Five things Pass G found that are worth carrying forward

1. **★★★ A REAL vendor profile is a different instrument from a synthetic one,
   and the difference is not that it is "more realistic".** It is that the
   *shapes it contains were not chosen by us*. Three of §3.7's findings exist
   only because X-Rite and Adobe made choices `tools/gen-profiles` would never
   have made: a **non-identity 2-entry B curve** encoding a declared perceptual
   black (which broke a tolerance derivation — §3.7.2); an **intent-tag aliasing
   pattern that differs by vendor** (X-Rite `A2B1≡A2B2`, ECI `A2B0≡A2B2`, the
   GWG trap `B2A0≡B2A2`); and a **`wtpt` that contradicts its own colorants**
   (§3.7.4). **A generator writes the fixtures its author already understands.**
   Where a corpus of real files is legally obtainable, it buys coverage no
   amount of care in a generator can.
2. **★★ "Both files come from one vendor and describe one colour space" is a
   WEAKER premise than it sounds, and the fixture must be interrogated before
   the claim is written.** The `eciRGB v2` v2.4/v4.2 pair looks like a clean
   isolator for the ICC version gate. It is not: **both encode `wtpt` at the PCS
   white**, so the gate they were meant to exercise is a no-op for either; and
   they differ in **TRC representation** as well as version, so any disagreement
   has two candidate causes the pair cannot separate. The general form: **before
   describing a fixture as isolating a variable, enumerate the OTHER things that
   differ between its two arms.** Recording that a fixture *cannot* do a job is
   as load-bearing as recording that it can (§5.4).
3. **★★ A tolerance may need to be a FUNCTION of the fixture, not a constant** —
   and the tell is a derivation containing a clause about the fixture's
   *contents*. §3.7.2's first draft said "the 2-entry B curves are affine"; the
   property that mattered was "the exact identity `(0x0000, 0xFFFF)`", which is
   true of one tag in a file and false of another **in the same file**. A
   constant would have had to be the looser of the two everywhere, weakening the
   arm that did not need it. **When a `why` string asserts something about the
   fixture, ask whether the code can read it instead**; a run-time-selected
   tolerance cannot go stale (DL-034) and states its own premise on the line.
4. **★★ A gate derived for one direction is not a gate in the other, however
   small its number.** §B's `4×10⁻³` is defensible because lcms2 forces
   trilinear for a Lab-PCS *output* LUT and the method envelope is identically
   zero there; reusing it on §A's `A2B` end-to-end rows — where that envelope is
   the *dominant* term — put a bound on the table that omitted the biggest term
   in it, and three rows correctly failed. **Direction is part of a tolerance's
   identity in exactly the way §6.4 says it is part of a finding's.** The
   remedy was to propagate the envelope through the actual destination model
   point by point, which is Pass 4's method reused rather than a new one.
5. **★★★ The separation mechanism found two faults nobody was looking for, and
   the second was invisible until the first was fixed.** `BLIND` on the
   `eciRGB v2` authoring row said, correctly, that the "rival reading" was
   manufactured — the profile's `wtpt` and colorants agree, so nothing is out of
   step. Fixing that exposed the real defect: the `2×10⁻⁴` encoding-floor
   justification **did not hold for the profiles it was actually gating**
   (Ghent's sRGB colorants sum to the PCS white to ≈12 `s15Fixed16` lsb, because
   the *published* primaries do not sum to D50 to the lsb). The row had been
   **passing inside a bound its own justification could not support** — §5.2's
   shape, arrived at from the opposite direction. **A separation is not only a
   statement about power; it is a second, independent reading of what the row
   believes it is testing, and the two disagreeing is a finding.**

### 6.7 ★★★ Six things Pass H found that are worth carrying forward

1. **★★★ A published statement about a published artefact can be false, and
   the way you find out is by measuring it.** The ICC's `Probe2` readme says
   `Probev2_ICCv4.icc`'s `BToA` tags render pure single-colorant tints. They do
   not; they render a near-neutral CMYK build with the intent's colorant raised.
   The same sentence is realised **to the bit** on the two `Probev1` profiles
   the readme does *not* describe. **Ground truth is a provenance, not a
   guarantee** — §1 ranks `ground-truth` above `cross-check` because of where
   the expectation came from, and that ranking says nothing about whether the
   artefact honours it. Every ground-truth row this project ever adds must be
   run before it is believed, and the response to a falsified premise is to stop
   claiming (tolerance ∞ with a loud prefix), never to find a finite number the
   observation satisfies.
2. **★★★ Two conformant CMMs can return colours `33 L*` apart from one file,
   and ICC.1 says so.** Clause 8.10.2 step (a) prefers `DToBx`/`BToDx` *"except
   where this tag is not needed or supported by the CMM"*, and step (b) is the
   fallback. lcms2 supports `multiProcessElements` and takes (a); iccce does not
   and takes (b). **Both are conformant** (ambiguity A33), and the Probe profile
   was built to make exactly this visible. **The open question is not the
   selection but the SILENCE**: nothing in `inspect` or `transform` tells a
   caller that an author-preferred transform was present and declined, and a
   `33 L*` difference a caller cannot see coming is a disclosure defect even
   where the selection is right.
3. **★★ A synthetic fixture proves what it was written to prove, and no more.**
   Two instances in one pass. (a) `fixtures/synthetic/iccmax-version.icc`
   established that lcms2 *"does not refuse a major-version-5 profile"*
   (`NUMERIC_CLAIMS.md` §3.10.6) — true, and it does **not** generalise:
   **lcms2 declines all ten real iccMAX files**, for their content rather than
   their version. The fixture isolates the version field, which is its whole
   value; reading it as "lcms2 processes iccMAX" would have been wrong. (b) The
   encoded-PCS clamp divergence Pass 4b found on a fixture *this project
   authored* now reproduces on the ICC's own published v4 probe — so it was
   never an artefact of our fixture design, and the unsettled clause question
   behind it is load-bearing on real files.
4. **★★ An in-process library test cannot see a CLI-to-`Chain` mis-wiring, and
   the injection proved which rows can.** Rotating the destination intent→`B2A`
   map by one turned **exactly one** of §D's eight per-profile rows red — the
   one that drives the shipped binary. The other seven evaluate a tag *by
   signature* in process and are blind to it by construction. **Ask of every row
   not "what does it measure" but "which layer is in the loop".**
5. **★ `transicc` exits 0 when it fails.** It prints `[transicc]: Couldn't link
   the profiles`, converts nothing, and returns status 0 — measured on all ten
   iccMAX members of this corpus. **Any oracle-side acceptance test that keys on
   the exit code will record lcms2 as accepting everything.** The observable
   Pass H uses is *"did any numbers come out"*. This sits beside §5.6's rule
   about the *harness's* own exit code: the oracle's is not a gate either.
6. **★ A count is not a difference, and a metric label that lies is worse than
   one that is coarse.** `Metric::IndicatorCount` was added because Pass H
   grades counts of files and of violated conditions, and emitting those under
   `abs-max-component` would have put a wrong unit in the TSV beside a number
   that is not a difference of components in any space. **A count row's
   tolerance is essentially always zero**; a non-zero one deserves a very good
   `why`.

### 6.8 ★★★ Five things Pass I found that are worth carrying forward

1. **A bound derived from the component the row is ABOUT missed the term from
   the component the row merely USES.** The cone-matrix difference
   (`5,661×10⁻⁶`) was the subject; the white-point difference
   (`4,453×10⁻⁵`, **7,9×** larger) was the dominant term and was not in the
   brief. Fourth instance of this shape in this document. **The countermeasure
   is not vigilance, it is arithmetic: write the complete derivation down and
   run it before the pass exists.** Both terms here were computable in exact
   rational arithmetic from published constants alone.

2. **★★ Two ICC publications print different Bradford matrices.** ICC.1:2022
   Annex E.3 Eq. (E.1) prints `M_A[0][0] = 0,8951`; ICC's own sRGB guidance
   computed its recommended `chad` with `0,8950`. *"Recompute E.3's Bradford
   and you get ICC's recommended `chad`"* is **false at full precision**. This
   is **recorded, not adjudicated** — Annex E is informative, ICC.1 mandates no
   CAT (A29), and deciding which ICC document is authoritative is not a
   decision a conformance suite is entitled to make. Registered **NA-010**.

3. **★ Sub-ULP does not mean identical bytes.** The corpus inferred from a
   `0,371` ULP difference that *"the written tag bytes are identical"*. Measured:
   **3 of 9** cells still encode to different `s15Fixed16` words in that very
   case, and **6 of 9** for iccce as shipped. *Below one ULP* bounds an encoding
   difference at **one LSB**; it does not zero it. Any future claim of the form
   "the difference is sub-ULP so nothing is observable" must be **measured
   through the encoder**, not inferred from the magnitude.

4. **★★ A one-sided ground-truth row cannot be the regression gate.** §B grades
   `|iccce − published| ≤ predicted`, which is the claim worth making and has
   **no power against a change that moves iccce toward ICC**. The injection
   proved it: substituting CIE's 5-figure D65 left three §B cells **passing**
   because they got closer to ICC's numbers, while §C — the two-sided
   `|iccce − independent prediction| ≤ round-off` row — failed by eight orders.
   **Every published-ground-truth row in this project should be paired with a
   two-sided derived-expectation row**, and the pairing stated, because the
   ground-truth row is the one people quote and the derived one is the one that
   holds.

5. **★ A tolerance derived from a corpus SUMMARY rather than the corpus's
   printed VALUES failed on its first run.** E3's `1×10⁻⁸` came from a corpus
   sentence reading *"to `9,3×10⁻⁹`"*; the same corpus paragraph prints three
   row sums whose worst is `7,946×10⁻⁸`. The summary quoted the first row as
   though it were the maximum. **Derive from the printed numbers, then check
   that the surrounding sentence agrees with them** — and when it does not, the
   sentence is the thing to fix.

---

## 7. Related

- `tools/difftest/README.md` — the oracle, its pin, its licence, and the
  smoke test.
- **`tools/difftest/README.md` §13 — the Pass 3 differential in full**: the
  profile pair, the settings, the two experiments that *test* §3.3's
  justifications, the machine-readable records, and what §13 owes.
- **`tools/difftest/src/pass3.rs`** — every §3.3 tolerance as a `const`
  carrying its own derivation. **The constant and the row must state the
  same number**; if they ever differ, the source is authoritative about what
  ran and this document is authoritative about what was intended, and the
  discrepancy is a finding.
- **`tools/difftest/src/bin/pass3_report.rs`** — run it (`cargo run --bin
  pass3_report`) to see the per-point record, the worst offenders, the
  quantisation experiment and the white-point clamp experiment with its
  sensitivity control.
- **`tools/difftest/README.md` §14 — the Pass 4 LUT differential in full**:
  lcms2's 4-D interpolation as read at the pin, the three experiments that
  test §3.4's justifications, the ICC-absolute white-point finding, the
  coverage statement, and what §14 owes.
- **`tools/difftest/src/pass4.rs`** — every §3.4 tolerance as a `const` with
  its derivation, plus the `mft2` reimplementation that makes the
  interpolation-substitution experiment possible (and the apparatus check that
  holds it against `iccce-cmm`). **`cargo run --bin pass4_report`** prints the
  per-point record, the envelope, the attribution, the corner control and the
  absolute-intent white-point experiment.
- **`ICC_Spec\icc\icc__s__computational_models.md` §4** — the clause reading
  that corrects NA-003 (§5.2 here), and **A39 / A39b / A39c**.
- `docs/LEGAL.md` §4 — lcms2 licence verification.
- `docs/LEGAL.md` §5 — reference values are facts; transcribe the source
  alongside the value.
- `CLAUDE.md` rules 3, 4, 5, 7.
