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
| 2026-08-11 (later still) | **§5, NA-003 — the "But note" clause citation** | cited **ICC.1:2022 6.4** as requiring per-component clipping of *device* values on integer conversion and permitting none for float32 | **superseded; the sentence is preserved verbatim in the new §5.2 and the row now points there** | `icc-conformance` | **★ A WRONG CITATION, CORRECTED — not a tolerance change; no number moved.** The recalled words are real but belong to a clause about the **PCS**: 6.4 is titled "Converting between PCSXYZ and PCSLAB encodings". The clause governing device encoding is **6.5**, whose float32 permission is doubly gated to `DToBx`/`BToDx` tags — which **8.3.3/8.4.3 do not permit in a matrix/TRC profile at all**. Settled by `icc-spec-librarian`'s fifth pass (`ICC_Spec\icc\icc__s__computational_models.md` §4/§4.2, **A39** resolved). **The correction inverts a finding that had been built on it**: `tools/difftest/README.md` §13.4 used NA-003 to hypothesise that lcms2's out-of-range float device output might be conforming and iccce merely stricter. It is not — a conforming F.8–F.16 evaluation *cannot* exceed 1,0, so the observed 1,000 120 is evidence the **input** clamp was skipped. Two hedges are carried into §5.2 and must survive restatement: clause 5 binds a CMM only to **reading** profiles (**A39b**), so "non-conforming" is not available and the word is *divergence*; and the **v2** half is **UNSOURCED** (**A39c**). The **size of the divergence under genuine out-of-gamut input remains unmeasured** — every observed excursion is 1-lsb boundary residue. |
| 2026-08-11 (later still) | §3.4, rows 0–7 and §3.4.2 (**first filling, not a change**) | blank | as recorded in §3.4 | `icc-conformance` | Pass 4's LUT differential ran; the comparisons exist, so the rows are no longer allowed to be blank. **No tolerance was widened; there was nothing to widen.** Two things are worth flagging about *how* these numbers were arrived at, because both are the kind of thing that would otherwise look like tuning. **(1) The tolerances were derived from an envelope computed before any comparison against lcms2's output** — the CLUT and the two interpolation algorithms alone — and the observed residuals then landed 0.3–0.5 % below it. Had they landed above it, the tolerance would not have moved; the finding would have been that something other than interpolation was in play. **(2) The wide rows were split from the tight ones deliberately.** NA-006 warned in advance that "a tolerance wide enough to swallow ~1 ΔE cannot also demonstrate agreement", so rows 2–3 are structural-only at 2.0 ΔE00 and the agreement claim was moved to rows 4–5 at 2×10⁻² and 1×10⁻³, where the method difference is switched off by construction. |
| 2026-08-11 (later still) | §5, NA-006 (**first entry in THIS document**; already registered in `NUMERIC_CLAIMS.md` §4) | "**~1 ΔE, corpus-derived bound, NOT measured**" | **measured: max 1.5741 ΔE2000 on `A2B0`, 0.254 23 on `A2B1`** | `icc-conformance` | The n-linear CLUT choice was registered the day the code landed with an explicit "iccce has NOT measured it, and cannot yet". Pass 4 measured it — against **lcms2's actual 4-D scheme**, which turned out **not to be tetrahedral** but a hybrid (linear in C, tetrahedral in M/Y/K), so the corpus's trilinear-vs-tetrahedral bound was not the applicable one. The measured value is close to it and the *shape* of the claim is unchanged; what changed is that "unmeasured" comes off. **The cost is a property of the table's curvature, not a constant** — the two A2B tags in one file differ by 6×. |
| 2026-08-11 (later still, Pass 4b) | §3.4.4, all rows (**first filling, not a change**) | did not exist | as recorded in §3.4.4 | `icc-conformance` | Pass 4b measured the **B2A** direction, the **v4 `mAB `/`mBA `** element pipeline and the **F.2 grayTRC** model; the comparisons exist, so the rows are no longer allowed to be blank. **Three things about *how* these numbers were arrived at, because each is the kind of thing that would otherwise look like tuning.** **(1)** Every tolerance is an **envelope computed inside the harness from lcms2's own arithmetic, with no lcms2 output in it** — the roundings were read at pin `21c582a` and modelled stage by stage — and each is paired with a much tighter row measuring what is left *after* the model is applied (A3, C4). Where the envelope was written into the doc comment as a guess before it was computed, the guess was **replaced by the computed value and the tolerance re-derived from it**; three rows below record exactly that. **(2)** The wide/tight split of §3.4 is kept, but its *sense is inverted* here: in Pass 4 the wide row was wide because of a real method difference; in Pass 4b the method difference is **zero** (lcms2 forces trilinear for a Lab-PCS LUT), so A5 exists purely as a **sensitivity control** showing the comparison could see a geometry difference 99–139× larger if there were one. **(3)** A **fourth kind**, `derived-expectation`, was introduced for §B and is defined in §3.4.4.1 including what it cannot do. It is **not** ground truth and §3.4.3's published-value row stays blank. |
| 2026-08-11 (later still, Pass 4b) | **§3.4.4 row C1** — gray → sRGB, device | **1×10⁻⁴**, justified by an envelope of "3,45×10⁻⁵" | **2,5×10⁻⁴**, justified by a **computed** envelope of 9,680×10⁻⁵ | `icc-conformance` | **★ A CORRECTED ENVELOPE, NOT A WIDENED NUMBER — and the distinction is checkable.** The 3,45×10⁻⁵ was written into the constant's doc comment *before the envelope was computed*, from a hand estimate of the resampling error of a 4096-point reverse curve. The reimplementation of `cmsReverseToneCurveEx(4096)` then put the real envelope at **9,680×10⁻⁵**, and the row was observed at 9,686×10⁻⁵. §0's procedure in order: **(1) Is the code wrong?** No — and this is not an opinion: modelling lcms2's destination collapses the disagreement **457×**, to 2,121×10⁻⁷, which is *below* `transicc`'s print floor. The residual is reproduced, not merely bounded. **(2) Is the expectation wrong?** There is none; both sides are computed in the run. **(3) Is the fixture wrong?** No. **(4) Only then, the tolerance** — re-derived as 2,6× the computed envelope. **The guess is preserved here so the change is auditable**; a reader who suspects tuning can recompute the envelope, which contains no lcms2 output at all. |
| 2026-08-11 (later still, Pass 4b) | **§3.4.4 row C3** — gray → sRGB, ΔE2000 | **1×10⁻²**, derived at **white** | **5×10⁻²**, derived at **black** | `icc-conformance` | **★ A DERIVATION LOOKING AT THE WRONG END OF THE AXIS.** The original reasoning propagated the device envelope through `dL*/d(device) ≈ 85 near white`. The run failed at 2,17×10⁻². §0's procedure: the code is not wrong (C4 attributes the whole residual, 457×), so the **analysis** was. Near *black*, below sRGB's linear breakpoint, a device difference `δ` becomes `δ/12,92` of linear light and CIELAB's **chromatic** sensitivity on its own linear segment (`da*/dX = 4038`) makes `Δa* ≈ 136 δ` against `ΔL* ≈ 69,9 δ`; with `S_C ≈ 1` and `S_L ≈ 1,75` the chromatic term dominates by ~3×, giving ≈2×10⁻². **This inverts §6.2's carried-forward note** that "near black the device metric explodes while ΔE stays small" — that holds for a device comparison amplified by an inverse TRC, and the opposite holds for a ΔE computed *from* a device difference at the same place. Both texts are kept; the new one is not a relaxation of the old but a different calculation. |
| 2026-08-11 (later still, Pass 4b) | **§3.4.4 row B6** — fixture → sRGB, device | **1×10⁻⁴** (shared with B5) | **2,5×10⁻⁴** (its own constant) | `icc-conformance` | **★ A MISSING TERM IN A DERIVATION.** B5 and B6 were given one constant because they are "the same fixture against the same oracle". They are not the same comparison: B5 ends at a **CLUT**, B6 ends at **sRGB's inverse tone curves** — and lcms2 builds those as a 4096-entry `u16` resampling whose envelope §C measures independently at 9,68×10⁻⁵, an order of magnitude above B5's whole budget. The row failed at 1,012×10⁻⁴, which is that term and nothing else. Split into two constants with two derivations. **The fix is a second constant, not a bigger one**: B5 keeps 1×10⁻⁴ and still passes at 5,2×10⁻⁵, so the change cannot be a blanket relaxation. |
| 2026-08-11 (later still, Pass 4b) | **§3.4.4 row B0** — both geometries on an affine CLUT | **0,0 — exact** | **1×10⁻¹⁴** | `icc-conformance` | **★ REAL ARITHMETIC MISTAKEN FOR FLOATING POINT.** The justification — "every interpolation geometry reproduces an affine function exactly" — is **true**, and the tolerance derived from it was still wrong: the two algorithms reach that value by different sequences of `f64` operations, so they agree to *rounding*, not bit-identically. Failed at 1,110×10⁻¹⁶. The new bound is derived from the arithmetic rather than the algebra: the n-linear arm sums 2⁴ = 16 products of values in [0,1], so ~16 ulp = 3,6×10⁻¹⁵, and 1×10⁻¹⁴ is ~3× that — **still 11 orders below one `u16` lsb**, so the row remains the precondition for B1–B4 that it was written to be. A general lesson worth carrying: **"exact" in a spec-derived argument means exact in ℝ, and a tolerance of 0,0 is only available when the two sides are the same operations in the same order** (as at §3.4.1 row 6 and §3.4.4 row C5, which *are*, and are still graded at 0,0 and still observe it). |

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

Current coverage, stated honestly, as of **2026-08-11 (after Pass 4b)**:

| Pass | Status |
|---|---|
| 0 | oracle pinned, built (**Windows/MSVC only**) and smoke-tested on **2 profiles, 1 direction each**. A Rust harness drives it (`tools/difftest/README.md` §11); its own registered check is **oracle-reproducibility — both sides are lcms2**. |
| 1 | `iccce-color`: **1 correctness claim** (ΔE2000, 34/34 published pairs) and **16 arithmetic identities** (§3.1). Reported passing on **one machine, Windows 11 Pro 10.0.26200 / MSVC, `f64` throughout**. **No Linux run has been observed by anyone.** |
| 2 | `iccce-profile`: parsing records exist in `NUMERIC_CLAIMS.md`; **§3.2 of this document is still blank** and no tolerance here grades the parser. |
| 3 | **`iccce-cmm` matrix/TRC: 5 graded rows + 2 reported-only means (§3.3), run 2026-08-11.** Scope in the next paragraph. |
| 4 | **`iccce-cmm` `lut16` A2B → matrix/TRC: 8 graded rows + 4 reported-only (§3.4), run 2026-08-11 (later), at all four intents.** **A2B direction only** — the B2A half of the done-when is not measured. Scope below. |
| **4b** | **`lut8` B2A, the v4 `mAB `/`mBA ` element pipeline, and the F.2 grayTRC model: 23 graded rows + 5 reported-only (§3.4.4), run 2026-08-11 (later still).** **Two intents** (perceptual and media-relative) in §A and §C, **one** in §B; **saturation and ICC-absolute are not run anywhere in Pass 4b**. The v4 claims rest on **one synthetic fixture** because a 40-profile sweep of this machine found **zero** `mAB `/`mBA ` tags. Scope below. |
| 5–8 | not started |

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
