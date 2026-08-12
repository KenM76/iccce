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
is exactly what this table is for. Costs are stated
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
