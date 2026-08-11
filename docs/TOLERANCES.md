# iccce — the tolerance budget

**Owner:** `icc-conformance`.

**Status, 2026-08-11: Pass 1's tolerances are recorded (§3.1); Passes 2–6
are still blank, and blank on purpose.** Exactly **one** row in this
document is a correctness claim — the CIEDE2000 row, graded against
published data. Everything else in §3.1 is an *arithmetic identity*: it
detects drift and structural error and **cannot detect a consistently
wrong constant**. The distinction is drawn in §1 and is the difference
between "this is right" and "this has not changed".

**No tolerance anywhere in this document grades `iccce` against `lcms2`.**
There is still no such comparison to grade: `iccce` has no transform
(Pass 3). The harness that will run those comparisons now exists
(`tools/difftest/README.md` §11) and its one registered check compares
lcms2 to lcms2.

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
| 2026-08-11 | §3.1, all rows (**first filling, not a change**) | blank | as recorded in §3.1 | `icc-conformance` | Pass 1 ran; the comparisons exist, so the rows are no longer allowed to be blank. Every number mirrors a `NUMERIC_CLAIMS.md` row rather than being chosen here. **No tolerance was widened; there was nothing to widen.** |
| 2026-08-11 | §5, NA-001 / NA-002 / NA-003 (**first filling**) | "none registered yet" | as recorded in §5 | `icc-conformance` | `iccce-color` shipped one named deviation from normative text (NA-001) and one policy choice (NA-002). Registering them is required by `ARCHITECTURE.md` invariant 3 the moment they exist, not when they are measured. |

---

## 5. Named approximations

`ARCHITECTURE.md` invariant 3: *every approximation is named and
measured*. This is the register. Each entry states the departure from
exact colorimetry, and what it costs in ΔE — a cost of "unmeasured" is
permitted only while the entry is new.

**Filled 2026-08-11**, mirroring `NUMERIC_CLAIMS.md` §4. Costs are stated
in the units they were actually bounded in — **not** converted into ΔE2000
to make the column tidy, because a conversion nobody performed is a number
nobody can check.

| # | Approximation / deviation | Where | Cost | Evidence class | Measured? |
|---|---|---|---|---|---|
| **NA-001** | **The `f(t)` breakpoint uses the exact rational form** `(24/116)³ = 0,008 856 451 679…` (and `24/116` for `f⁻¹`) where **ICC.1:2022 6.4's normative text writes the decimal `0,008 856`**. iccce's first stated deviation from normative specification text. | `crates/iccce-color/src/lab.rs` — module doc §"Named DEVIATION", `f` / `f_inv` | **~10⁻⁷ in `f`, therefore ~10⁻⁵ in `L*`** | **corpus-derived-bound** | **NO — bounded analytically in the standards corpus; iccce has NOT measured it.** No test in this repository computes the difference between the two forms. Anyone restating this must write *"bounded analytically at ~10⁻⁵, unmeasured"*, never *"measured at 10⁻⁵"*. |
| **NA-002** | **Bradford is a policy choice, not a requirement.** iccce implements the general von Kries *method* and supplies Bradford cones; ICC.1 mandates no particular chromatic-adaptation transform (corpus ambiguity **A29**, resolved *recommended, not mandated*). **Must not be described as conformance.** | `crates/iccce-color/src/adapt.rs` | **UNMEASURED, and not yet exercised** — nothing in the repository adapts anything yet (`iccce-cmm` is a stub) | — | **NO.** An entry may carry an unmeasured cost *only while it is new*; this one **becomes owed the moment Pass 3 uses it**. Measuring it means comparing Bradford against at least one other CAT over a stated sample set, in ΔE2000, on a stated illuminant pair — and **both alternatives are currently unsourceable** (von Kries/HPE digits are a corpus placeholder marked DO NOT USE; CAT02's CIE 159 is paywalled). |
| **NA-003** | **No clamping in the colour layer.** `f_inv` deliberately does not clamp below the linear segment; gamut policy is left to the CMM layer where it can be a named per-transform decision. ICC's own reference code makes negative-XYZ clamping a *compile-time option*. | `crates/iccce-color/src/lab.rs::f_inv` | **not an approximation — no ΔE cost.** A layering decision, registered so Pass 4 does not meet it as a surprise. | — | n/a. **But note**: ICC.1:2022 6.4 normatively requires out-of-range colours to be *"clipped on a per-component basis"* on integer conversion (and no clipping for float32 encodings). **That binds the CMM and profile layers, not this crate** — do not conclude from `iccce-color`'s silence that iccce clamps nowhere. |

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

---

## 6. Coverage — say the scope or say nothing

**"Verified" without scope is the claim this document exists to
prevent.** Every conformance statement must carry: how many profiles, of
which classes, at which intents, on which platform.

Current coverage, stated honestly, as of **2026-08-11**:

| Pass | Status |
|---|---|
| 0 | oracle pinned, built (**Windows/MSVC only**) and smoke-tested on **2 profiles, 1 direction each**. A Rust harness now drives it (`tools/difftest/README.md` §11) with **exactly one registered check**, whose kind is **oracle-reproducibility — both sides are lcms2**. |
| 1 | `iccce-color`: **1 correctness claim** (ΔE2000, 34/34 published pairs) and **16 arithmetic identities** (§3.1). Reported passing on **one machine, Windows 11 Pro 10.0.26200 / MSVC, `f64` throughout**. **No Linux run has been observed by anyone.** |
| 2–8 | not started |

**Scope limits that must travel with any Pass 1 "verified":** adaptation
is exercised in **one direction (D65 → D50), one sample vector**; the
round trips use **1–2 samples each**; `Mat3::inverse` is tested on **one
matrix, and it is not a colour matrix**; there is **no ground-truth row
for chromatic adaptation at all**; and **no comparison of any kind against
lcms2 exists in `iccce-color`.**

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

---

## 7. Related

- `tools/difftest/README.md` — the oracle, its pin, its licence, and the
  smoke test.
- `docs/LEGAL.md` §4 — lcms2 licence verification.
- `docs/LEGAL.md` §5 — reference values are facts; transcribe the source
  alongside the value.
- `CLAUDE.md` rules 3, 4, 5, 7.
