# iccce — the tolerance budget

**Owner:** `icc-conformance`.

**Status, 2026-08-11: Pass 1's tolerances are recorded (§3.1) and Pass 3's
were measured later the same day (§3.3). Passes 2 and 4–6 are still blank,
and blank on purpose.** Exactly **one** row in this document is a
correctness claim against published data — the CIEDE2000 row. Everything in
§3.1 besides it is an *arithmetic identity*: it detects drift and structural
error and **cannot detect a consistently wrong constant**. §3.3's rows are
*cross-checks and self-consistency*, which is a different and weaker thing
again. The distinction is drawn in §1 and is the difference between "this is
right" and "this has not changed".

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
| 2026-08-11 (later) | §3.3, rows 1–4 and 6–7 (**first filling, not a change**) | blank | as recorded in §3.3.1 | `icc-conformance` | Pass 3 ran; the comparisons exist, so the rows are no longer allowed to be blank. **No tolerance was widened; there was nothing to widen.** |
| 2026-08-11 (later) | **§3.3 row 5** — sRGB → AdobeRGB → sRGB round trip | **1×10⁻² ΔE2000** | **2.5×10⁻² ΔE2000** | `icc-conformance` | **★ A CORRECTED JUSTIFICATION, NOT A WIDENED NUMBER — and the distinction is the reason this row is long.** The 1×10⁻² was set *before* the run, from this reasoning: *"sRGB and Adobe RGB (1998) share their red (0.64, 0.33) and blue (0.15, 0.06) primaries and Adobe's green is more saturated, so the sRGB triangle is strictly contained, no grid point is clipped, and the only losses are interpolation ones."* **The run failed at 1.8788×10⁻².** §0's procedure was then followed in order. **(1) Is the code wrong?** No: iccce applies the clamp Annex F.8–F.16 makes normative, and the failure is that clamp doing its job. **(2) Is the expectation wrong?** There is no recorded expectation — both sides are computed in the run. **(3) Is the fixture wrong?** **This is where it was.** Every clause of the original reasoning is true of the two *colour spaces* and false of the two *files*: a matrix/TRC profile's media white is its colorant sum, and HP (1998) and Adobe (2000) rounded their colorants to `s15Fixed16` independently, leaving the two encoded whites 5/2/12 lsb apart. Source device white therefore lands at (1.000 106, 0.999 873, 1.000 254) in destination linear space and **is** clipped, along with 24 other grid points. **(4) Only then, the tolerance.** The mechanism was not merely asserted: a closed-form prediction from the two matrices and the clamp alone — no tone curve, no lcms2, no measurement — gives **1.878 244×10⁻²** against **1.878 818×10⁻²** observed, agreeing to **0.03 %**. The new number is that, plus ≈1×10⁻³ for table interpolation, plus ~25 % headroom. **What was actually wrong was an analysis, and the analysis is now on record in `tools/difftest/README.md` §13.6.3 alongside the sentence it replaced.** |
| 2026-08-11 (later) | §3.3 row 6 (**new check, not a change to an existing one**) | did not exist | **1×10⁻³ ΔE2000** | `icc-conformance` | Added *because* of the row-5 correction. Row 5 is an upper bound on a quantity that is mostly a deliberate cost, so removing iccce's range clamping would make row 5 **pass more easily** while deleting a normative requirement. A gate that rewards that is not a gate. Row 6 pins the observed cost to its closed-form prediction, and a sensitivity control (printed by `pass3_report`) shows it would fail by 19× if clamping were removed. |
| 2026-08-11 (later) | §5, NA-004 (**first registration**) | did not exist | as recorded in §5 | `icc-conformance` | Pass 3 shipped a gamut-clipping policy at the CMM layer, which NA-003 explicitly deferred to "where it can be a named per-transform decision". It is now named, and — unusually for this register — **measured on the first day it existed**. |

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
| **NA-004** | **★ Gamut clipping at the CMM layer: `pcs_to_device` clamps each linear component to `[0,1]` before the inverse TRC** (ICC.1:2022 **Annex F.8–F.16**, normative), and `iccce-cmm::curve` clamps again at two further points (clause 10.18's domain in `Trc::eval`; F.1(b)'s attainable-range clip in `Trc::eval_inverse` / `invert_table`). **This is the named per-transform decision NA-003 deferred.** It is *conformance*, not an approximation — but it has a **cost**, because two profiles' encoded gamuts rarely nest exactly, and that cost is what is registered here. | `crates/iccce-cmm/src/matrix_trc.rs::pcs_to_device`; `crates/iccce-cmm/src/curve.rs::{eval, eval_inverse, invert_table}` | **1.8788×10⁻² ΔE2000** at device white for the sRGB → Adobe RGB (1998) pair, on **25 of 133** grid points overall. Closed-form prediction from the two colorant matrices and the clamp alone: **1.8782×10⁻²** — 0.03 % agreement. Driver: the two files' encoded media whites differ by 5/2/12 units of `s15Fixed16`'s 1/65536 lsb. | **measurement** (`tools/difftest`, §13.6.3) | **YES — measured 2026-08-11**, on **one profile pair, one direction, 133 points, one platform**. **The cost is corpus-specific**: it is a property of *which two files* are being converted between, not a constant of the engine, and any restatement must carry the pair. Two profiles with identical encoded whites would show ≈0 here. |

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

Current coverage, stated honestly, as of **2026-08-11 (after Pass 3)**:

| Pass | Status |
|---|---|
| 0 | oracle pinned, built (**Windows/MSVC only**) and smoke-tested on **2 profiles, 1 direction each**. A Rust harness drives it (`tools/difftest/README.md` §11); its own registered check is **oracle-reproducibility — both sides are lcms2**. |
| 1 | `iccce-color`: **1 correctness claim** (ΔE2000, 34/34 published pairs) and **16 arithmetic identities** (§3.1). Reported passing on **one machine, Windows 11 Pro 10.0.26200 / MSVC, `f64` throughout**. **No Linux run has been observed by anyone.** |
| 2 | `iccce-profile`: parsing records exist in `NUMERIC_CLAIMS.md`; **§3.2 of this document is still blank** and no tolerance here grades the parser. |
| 3 | **`iccce-cmm` matrix/TRC: 5 graded rows + 2 reported-only means (§3.3), run 2026-08-11.** Scope in the next paragraph. |
| 4–8 | not started |

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
   **Recorded as a FINDING; the specification question is OWED to
   `icc-spec-librarian`** and is stated verbatim in
   `tools/difftest/README.md` §13.4.


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
- `docs/LEGAL.md` §4 — lcms2 licence verification.
- `docs/LEGAL.md` §5 — reference values are facts; transcribe the source
  alongside the value.
- `CLAUDE.md` rules 3, 4, 5, 7.
