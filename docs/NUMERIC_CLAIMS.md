# iccce — the numeric-claims ledger

**Owner:** `icc-librarian`. **Created 2026-08-11**, at Pass 1, with the
first genuinely measured claim this project has ever produced. It was
deliberately *not* created during Pass 0 — see `SESSION_LOG.md`,
2026-08-11: an empty ledger invites a first row that is not a
measurement, and makes *"nothing has been measured"* look like
*"nothing has been filed."*

**Append-only in the same sense as `ARCHITECTURE.md` §5.** A claim that
is superseded gets a **new row** carrying a `supersedes:` reference. Rows
are never edited to make an old number look like a new one; the whole
value of the ledger is that a stale claim stays visible as a stale claim.

---

## 0. Why this document exists — and why it is this project's, not the sibling's

Colour work accumulates sentences like *"matches lcms2 within
0.8 ΔE2000"* across many Passes. Each is true on the day it is written.
Each quietly becomes false when something upstream of it changes — a
different interpolation, a corrected white point, a re-sourced constant,
a new clamping rule. Nothing announces the change. The claim keeps
sitting in a README, a doc comment, a commit message, and a
conversation, being cited.

Without a ledger, answering *"is that still true?"* costs a full re-run
of everything, so nobody asks, so the answer is never known. **This
document's only job is to make the question cheap.** When a later Pass
changes something, the claims it invalidates must be *findable* — which
is what §6's dependency notes are for.

Two related documents, and the boundary between them:

- **`docs/TOLERANCES.md`** (owned by `icc-conformance`) is the *budget*:
  what tolerance a comparison is allowed, and why that number is
  justified rather than tuned. It is prospective.
- **This file** is the *record*: what was actually compared, at what
  tolerance, with what result, over what corpus, on what date. It is
  retrospective.

A tolerance can exist in `TOLERANCES.md` with no row here (nothing has
been run yet). A row here must never exist without the tolerance being
justifiable — and where §3 below uses a tolerance that `TOLERANCES.md`
has not yet recorded, that gap is stated in the row, not papered over.
**As of this filing, `TOLERANCES.md` §3.1 (Pass 1) is still entirely
blank and §5 (named approximations) still reads "none registered yet".**
*(verified — read 2026-08-11.)* Those are `icc-conformance`'s rows to
fill; this ledger does not fill them and does not pretend they are
filled.

---

## 1. Evidence classes — every row states exactly one

The class is not decoration. It is the difference between *"this is
right"* and *"this has not changed."* Ordered strongest to weakest as
**correctness** evidence.

| Class | What it means | What it can and cannot prove |
|---|---|---|
| **published-ground-truth** | The expected value comes from a published, peer-reviewed or standardised source, transcribed with its citation. | Can prove the implementation is *correct*. The strongest claim available to this project. |
| **primary-spec-constant** | A numeric constant transcribed from the standard's own text, with clause. | Proves provenance of a constant, not the correctness of the maths using it. |
| **transcription-guard** | An assertion that a constant matches a *published check on that constant* (e.g. a row-sum stated by the source). | Catches a typo or a transposition between source and code. Says nothing about whether the value is the right value to use. |
| **corpus-derived-bound** | An analytic bound computed in the standards corpus (or here) from sourced values — **not run against code**. | Bounds an error. **Is not a measurement of iccce**, and must never be written as though iccce measured it. |
| **implementation-cross-check** | Agreement with lcms2 or another independent implementation. | Evidence that two implementations read a clause the same way. Two implementations can share a misreading (`TOLERANCES.md` §1). **Weaker than ground truth and must be labelled so.** |
| **arithmetic-identity** | A property that must hold by construction — round trips, inverses, symmetry, degenerate-input handling. Tolerance is `f64` noise, not a perceptual budget. | Detects **change and drift**, and catches whole classes of structural bug (transposition, wrong operation order). **Does not detect a consistently wrong constant** — a round trip through a wrong white point round-trips perfectly. |
| **self-consistency** | Round-trip / compiled-vs-reference / interpolation error where the two sides are both iccce. | The only way to *price* an approximation. Worthless as correctness evidence. |

**A row without a class is not finished**, exactly as in
`TOLERANCES.md` §1.

### 1.1 What a passing test actually establishes — read this before quoting a row

Every §3 row's "Result" column records **the bound the test asserts**,
not the residual that was observed. `assert!((got − expected).abs() <
1e-4)` passing proves the error was **below 1×10⁻⁴ on that run**. It
does **not** establish that the error was 3×10⁻⁶, and this ledger does
not say that it was.

**The observed maxima were not carried in the Pass 1 dispatch and are
therefore not on record anywhere.** That is a real (small) gap: a
residual that has silently grown from 10⁻¹² to 9×10⁻⁵ still passes a
10⁻⁴ gate, and nothing would show it. Recorded in §7 as owed work.

### 1.2 Who measured, and what this librarian verified

`icc-librarian` **has no shell** and ran nothing. For every row below:

- **The assertion, its tolerance, its inputs and its expected values were
  read by this librarian in the live source** on 2026-08-11, at the file
  and test name given in the row. *(verified.)*
- **That the test passes** is `icc-engineer`'s report of a
  `cargo test --workspace` run on this machine (Windows 11 Pro
  10.0.26200). *(reported — not re-run here.)*
- Independently checkable from the tree without a shell: **35 `#[test]`
  declarations exist** — 21 in `crates/iccce-color/src/` (`mat3.rs` 3,
  `xyz.rs` 4, `lab.rs` 5, `adapt.rs` 5, `delta_e.rs` 4) and 14 in
  `crates/iccce-profile/src/` (`lib.rs` 8, `num.rs` 6). *(verified — 35
  occurrences counted across 7 files.)* **That is a count of tests
  declared. It is not a count of coverage and not a pass result.**

---

## 2. Provenance of this Pass's rows

| | |
|---|---|
| **Pass** | 1 — colorimetry (`iccce-color`) |
| **Date** | 2026-08-11 |
| **Commit** | `7313c5b` (2026-08-11) — filled in by `icc-engineer` immediately after committing, per this row's own request. Every row below is anchored to that commit. |
| **Platform** | Windows 11 Pro 10.0.26200, MSVC toolchain. **No Linux run of these tests has been observed by anyone** — CI exists and its execution history has never been checked (`SESSION_LOG.md`, Pass 0). |
| **Precision** | Every computation in `iccce-color` is `f64` throughout (`mat3.rs` module doc: `C̄'⁷` alone overflows `f32`). No row below is an `f32` claim. |

---

## 3. The claims

### 3.1 ★ NC-001 — CIEDE2000 against the Sharma, Wu & Dalal 34-pair dataset

**This is the first real measured numeric claim in the project's
history**, and the reason this file exists as of today rather than
earlier.

| Field | Value |
|---|---|
| **What was compared** | `iccce_color::delta_e::delta_e_2000(s, t)` against the published ΔE00 value for each pair. |
| **Corpus** | **All 34 pairs** of Sharma, Wu & Dalal (2005), *"The CIEDE2000 color-difference formula: Implementation notes, supplementary test data, and mathematical observations"*, **Color Research & Application 30(1):21–30, DOI 10.1002/col.20070**. Transcribed into the test from `ICC_Spec\cie\cie__ref__delta_e.md`. |
| **Coverage — part of the claim** | **34 of 34 pairs. Not a sample; the whole published set.** The set is adversarial by design: pairs 1–6 catch an omitted `R_T` cross term, 7–8 assert asymmetry-freedom, 9–16 sit on the hue-angle discontinuity (where a 4th-decimal change in `b` legitimately changes the answer), 21–24 calibrate ΔE = 1.0 in four directions, 33–34 are very dark. Cherry-picking defeats the dataset's design and the test runs all 34 in one loop. |
| **Parametric factors** | `kL = kC = kH = 1` — the factors the published data is stated for. The `delta_e_2000_k` entry point takes them explicitly; the claim is **only** for 1:1:1. |
| **Tolerance** | **1×10⁻⁴**, asserted as `(got − expected).abs() < 1e-4`. |
| **Why that tolerance** | It is **the published data's own precision** — the paper states ΔE00 to four decimal places, so agreement cannot be asserted more tightly than the reference is printed. `cie__ref__delta_e.md` line 85: *"Use all 34. Cherry-picking defeats the design. Tolerance: **1×10⁻⁴** (the data's own precision)"*, and its tolerance table lists 1×10⁻⁴ against "the ΔE2000 **implementation** test". **This is an arithmetic-agreement tolerance, not a perceptual one** — DL-004's 1.0 ΔE2000 anchor is *irrelevant* here and must not be cited in support of it. |
| **Result** | **All 34 pairs agree within 1×10⁻⁴.** Per §1.1 this is the asserted bound; the observed maximum residual was not carried and is not on record. |
| **Evidence class** | **published-ground-truth** — the strongest class this project has. |
| **Note on provenance vs correctness** | The *formula transcription* is from lcms2 `cmsCIE2000DeltaE` (`impl_crosscheck` tier; CIE 142:2001 / ISO/CIE 11664-6 are paywalled and **not obtained**). **The claim's strength comes from the 34 published pairs, not from lcms2.** Had the transcription been wrong, the dataset was built to catch it. Do not restate this row as "cross-checked against lcms2" — that would understate it, and do not restate it as "conforms to CIE 142" — that would overstate it. |
| **Where** | `crates/iccce-color/src/delta_e.rs`, `tests::de2000_matches_all_34_sharma_pairs`, dataset constant `SHARMA_34`. *(verified — read.)* |
| **Measured** | 2026-08-11 by `icc-engineer` *(reported)*; assertions and data read by `icc-librarian` *(verified)*. |
| **Invalidated by** | Any edit to `delta_e_2000_k`; any change to `Lab`'s field semantics; any change to the transcription of `SHARMA_34` (which would be a change to the *expectation*, and must be justified from the paper, never from the code). |

### 3.2 Arithmetic identities — Pass 1

**Read the class before quoting any of these.** They are
`arithmetic-identity` rows: they prove the code is *structurally* sound
and has not drifted. **They cannot detect a consistently wrong
constant** — a round trip through a wrong white point round-trips
perfectly, and an adaptation matrix built from a mis-transcribed cone
matrix still maps its own source white to its own destination white
exactly. That is precisely why NC-001 is the only correctness claim on
this page.

| ID | What | Tolerance | Result | Coverage | Where (all *verified*) |
|---|---|---|---|---|---|
| **NC-002** | ΔE2000 symmetry: `ΔE(A,B) = ΔE(B,A)` | `< 1×10⁻¹²` | holds | **all 34 Sharma pairs** (the paper publishes the property for one pair; asserting it across the set costs nothing and catches asymmetric mean-hue handling anywhere in the gamut) | `delta_e.rs::tests::de2000_is_symmetric` |
| **NC-003** | ΔE2000 of a colour with itself | **exact** (`assert_eq!` to `0.0`) | holds | 1 colour, `L*50 a*2.5 b*0` | `delta_e.rs::tests::de2000_of_identical_is_zero` |
| **NC-004** | ΔE76 = 13.0 on a (3,4,12) difference | **exact** (`assert_eq!`) | holds | 1 pair | `delta_e.rs::tests::de76_pythagorean_identity` |
| **NC-005** | Bradford adaptation with `src == dst` is the identity matrix | `< 1×10⁻¹⁴` per element (9 elements) | holds | white = D50 | `adapt.rs::tests::same_white_gives_identity` |
| **NC-006** | Adaptation maps the source white exactly onto the destination white | `< 1×10⁻¹²` per channel | holds | **one direction only: D65 → D50**, D65 derived from the single-source chromaticity | `adapt.rs::tests::adaptation_maps_src_white_to_dst_white` |
| **NC-007** | D65 → D50 → D65 round trip recovers the input | `< 1×10⁻¹²` per channel | holds | **one sample vector `[0.4, 0.2, 0.7]`**, one illuminant pair | `adapt.rs::tests::adaptation_round_trip` |
| **NC-008** | Bradford row sums = 1.0001 / 1.0000 / 1.0000 | `< 1×10⁻¹²` | holds | the 9 sourced digits | `adapt.rs::tests::bradford_row_sums_match_corpus` — class is **transcription-guard**, see NC-009 |
| **NC-010** | `f_inv(f(t)) = t` across the Lab transfer function's breakpoint | `< 1×10⁻¹⁵` | holds | **7 probe points**: 0, half-breakpoint, exactly the breakpoint, just above it, 0.18, 0.5, 1.0 | `lab.rs::tests::f_round_trips_across_breakpoint` |
| **NC-011** | XYZ → Lab → XYZ round trip | `< 1×10⁻¹²` per channel | holds | **2 samples**, deliberately one per branch of `f` (0.20/0.30/0.40 cube-root branch; 0.004/0.005/0.003 linear branch) | `lab.rs::tests::xyz_lab_round_trip` |
| **NC-012** | White maps to `L* = 100, a* = b* = 0` | **exact** (`assert_eq!`) | holds | D50 | `lab.rs::tests::white_maps_to_l100_exactly` |
| **NC-013** | `Y = 0` maps to `L* = 0` | **exact** (`assert_eq!`) | holds | black, D50 white | `lab.rs::tests::black_maps_to_l0_exactly` — **holds exactly only because the linear segment exists**; a cube-root-only `f` gives `f(0) = 0` and `L* = −16`. That is the identity most worth keeping, because it is the one the A11 choice (§4, NA-001) is load-bearing for. |
| **NC-014** | Lab ↔ LCh round trip, hue wrapped into `[0, 360)` | `< 1×10⁻¹²` on `a*`, `b*` | holds | **1 sample**, third-quadrant (the case where `atan2` returns negative and the single wrap is exercised) | `lab.rs::tests::lab_lch_round_trip_and_hue_range` |
| **NC-015** | XYZ → xyY → XYZ round trip | `< 1×10⁻¹⁴` per channel | holds | **1 sample: D50** | `xyz.rs::tests::xyy_round_trip_is_identity` |
| **NC-016** | `Mat3::inverse` — `M · M⁻¹ = I` | `< 1×10⁻¹⁴` per element | holds | **1 matrix, and it is not a colour matrix** (`[[2,1,0.5],[0,3,1],[1,0,2]]`) — the Bradford inverse is exercised indirectly through NC-005 | `mat3.rs::tests::inverse_times_forward_is_identity` |

**Degenerate-input guards** (behavioural, not numeric, recorded here so
the ledger is a complete account of what Pass 1 asserts): black has no
chromaticity and returns `None` rather than dividing by zero
(`xyz.rs::tests::black_has_no_chromaticity`); a zero white point is
refused by `adaptation_matrix` rather than propagating infinities
(`adapt.rs::tests::zero_white_is_refused`); a singular matrix returns
`None` (`mat3.rs::tests::singular_matrix_returns_none`). The corpus notes
**both reference codebases can divide by zero** at the xyY conversion;
iccce does not. *(verified — guards read in source.)*

### 3.3 NC-009 — the Bradford cone matrix, and exactly how strong its sourcing is

| Field | Value |
|---|---|
| **Constant** | `BRADFORD` = `[[0.8951, 0.2664, −0.1614], [−0.7502, 1.7135, 0.0367], [0.0389, −0.0685, 1.0296]]`, row-major, rows are cone responses (ρ, γ, β), columns are (X, Y, Z), applied to a **column vector**. |
| **Evidence class** | **primary-spec-constant**, corroborated by two independent code extractions. |
| **Source** | **ICC.1:2022 Annex E.3, Equation (E.1)** — the corpus's `cie__ref__chromatic_adaptation.md` carries it VERBATIM and its frontmatter records `evidence: primary_spec (Bradford — ICC.1:2022 Annex E.3, verified 2026-08-11)`. It **agrees exactly** with both prior independent extractions (lcms2 `cmswtpnt.c` `LamRigg`; CRAN `spacesXYZ`). *(verified — corpus file and index read by this librarian 2026-08-11.)* |
| **★ A qualification the code's doc comment does not make** | **Annex E of ICC.1:2022 is informative, not normative** — the corpus's own clause line records it as *"ICC.1:2022 Annex E (informative)"*. So "primary-spec" here means *the constant is printed in the specification document*, which is a genuine promotion over code-only sourcing, and **not** *the specification mandates this matrix*. The corpus separately resolves ambiguity **A29** as *recommended, not mandated* — ICC.1 requires no particular chromatic-adaptation transform. Both statements are true and they are easy to conflate; see §4 NA-002. |
| **Extraction hazard, recorded because it nearly cost the signs** | ICC.1:2022 sets `−`, `+`, `×`, `≤`, `≥` in the **Symbol font**, which extracts into the Unicode private-use area. **All three extractors tested (`pypdf` 6.7.0, poppler `pdftotext -layout`, `pdfminer.six`) drop them silently — the Bradford matrix in Annex E.3 extracts all-positive.** *(verified — `ICC_Spec\LEGAL_NOTE.md`, read.)* The signs in `BRADFORD` carry over from the cross-verified code sources, which the Annex then confirms. Anyone re-sourcing this matrix from the PDF must map the private-use range first. |
| **What is asserted about it in code** | Only NC-008, the row-sum transcription guard (1.0001 / 1.0000 / 1.0000 — the first row's 1.0001 **is real, not a typo**). |
| **What is NOT asserted, and is the honest limit of Pass 1's adaptation work** | **No published worked example of a complete chromatic adaptation was obtained**, so there is **no ground-truth row for adaptation anywhere in this ledger.** Everything adaptation-related is NC-005 … NC-008: a sourced matrix plus arithmetic identities. A mis-transcribed digit that happens to preserve the row sum would survive every test in the crate. |
| **Inverse** | `M_A⁻¹` is **computed at runtime in `f64`** by adjugate/determinant, never transcribed: the corpus marks published inverse digits **NOT SOURCED** and directs exactly this. Singularity is tested as `det == 0.0` exactly rather than against an epsilon — deliberately, because an epsilon would be a tuned number with no citation. *(verified — `mat3.rs`.)* |

### 3.4 NC-017 — the D50 chromaticity derivation, and a corpus erratum this test caught

| Field | Value |
|---|---|
| **What was compared** | `D50.to_xyy()` against the chromaticity derived from **iccce's own sourced 4-figure D50 triple** (0.9642, 1.0000, 0.8249). |
| **Result** | `x = 0.345703`, `y = 0.358539`, asserted within `5×10⁻⁷`. |
| **Evidence class** | **arithmetic-identity / self-consistency.** The corpus marks all such chromaticities **DERIVED, not sourced**; this is a consistency check on arithmetic and **is not a published expectation.** |
| **★ The finding** | The test **failed on first run.** Per project rule 5 the arithmetic was checked before the code was blamed — and the *corpus* turned out to be wrong. `cie__ref__colorimetry_core.md` states the derived chromaticity as `x = 0.34567`, `y = 0.35850`. Those are the chromaticities of the **high-precision** D50 (0.96422 / 1 / 0.82521), not of the 4-figure ICC triple the same file instructs the project to use everywhere. **The corpus's own derivation committed the mixing-precision trap that the same section warns about.** Correct derivation from the sourced triple: `0.9642 / 2.7891 = 0.345703`; `1 / 2.7891 = 0.358539`. *(Arithmetic independently checked by `icc-librarian`.)* |
| **Status of the corpus fix** | A parallel dispatch was sent to `icc-spec-librarian` to correct the corpus file. **As of this filing the erratum is still present**: `cie__ref__colorimetry_core.md` line 60 still reads `x = 0.9642/(0.9642+1+0.8249) = 0.34567`, `y = 0.35850`. *(verified — grepped 2026-08-11.)* A later session must not assume the fix landed; check the file. |
| **Why it is in this ledger at all** | Because it is the verification loop running **in the direction nobody plans for**. The corpus is supposed to check the code. Here a code test checked the corpus and won. Two consequences worth carrying: a "DERIVED" value in the corpus is a *calculation someone did*, with the same error rate as any other calculation; and the mixing-precision trap is real enough that the document warning about it fell into it. |
| **Where** | `crates/iccce-color/src/xyz.rs::tests::d50_chromaticity_derives_from_sourced_triple` — whose doc comment records the finding at the site. *(verified.)* |

### 3.5 NC-018 — the D65 XYZ derivation

| Field | Value |
|---|---|
| **What** | D65 XYZ derived through `XyY::to_xyz()` from the chromaticity `(0.3127, 0.3290)`, checked against the corpus's derived `(0.95046, 1, 1.08906)` within `5×10⁻⁶`. |
| **Evidence class** | **arithmetic-identity** on shared arithmetic. **Not ground truth in any sense.** |
| **★ Weakest constant in the crate — say so whenever D65 is quoted** | The chromaticity is **single-source**: lcms2 `cmsvirt.c` alone, because IEC 61966-2-1 is paywalled and was not obtained. It is **not** cross-verified, unlike D50 and Bradford. The corpus records an open gap for an independent D65 source (ITU-R BT.709 is free from ITU and was not fetched). `illuminant.rs` exposes D65 as a **chromaticity** rather than an XYZ triple specifically so the derivation stays visible instead of an unsourced XYZ triple being baked in as though it were published. *(verified — `illuminant.rs`, corpus.)* |
| **Where** | `xyz.rs::tests::d65_xyz_matches_corpus_derivation`. |
| **Consequence for other rows** | **NC-006 and NC-007 use this D65.** Their arithmetic is exact; their *illuminant* rests on one source. Any future correction to D65 invalidates neither identity but does change the matrices they exercise. |

---

## 4. Named approximations and deviations

`ARCHITECTURE.md` invariant 3 and project rule 4: *every approximation is
named and measured.* This is the register of departures from exact
colorimetry, or from the specification's literal text, that `iccce-color`
makes as of Pass 1. `TOLERANCES.md` §5 is the tolerance-budget twin of
this section and **is still empty** — `icc-conformance` owes it rows;
this ledger records the facts in the meantime and does not edit that
file.

### NA-001 ★ — the `f(t)` breakpoint uses the exact rational form. **This is iccce's first stated deviation from normative specification text.**

| Field | Value |
|---|---|
| **The departure** | `iccce-color`'s Lab transfer function uses `LIMIT = (24/116)³ = 0,008 856 451 679…` for `f`, and `24/116` for `f⁻¹`, with linear branches `(841/108)·t + 16/116` and `(108/841)·(t − 16/116)`. **ICC.1:2022's own normative text writes the breakpoint as the decimal `0,008 856`.** |
| **Why this is a *deviation* and not merely a pick between disagreeing implementations** | Because the ICC.1:2022 ingest (2026-08-11) resolved corpus ambiguity **A11** and changed the character of the choice. VERBATIM, ICC.1:2022 **6.4**: *"Conversions between the PCSXYZ and PCSLAB encodings **shall use the equations of the form specified in ISO 13655**."* — so ICC.1 **delegates** `f(t)` and does not define it. But its own normative sentence writes the decimal: *"In order to calculate PCSLAB values from negative PCSXYZ values, the straight line portion of the PCSLAB colour component transfer function below **0,008 856** shall be extended linearly below zero."* **ISO 13655 is the actual authority and is paywalled and NOT obtained.** *(verified — `ICC_Spec\icc\icc__s__pcs_encoding.md`, read 2026-08-11; the corpus grades A11 "RESOLVED-as-DELEGATED".)* |
| **Why iccce deviates anyway** | The rational form makes `f` and `f⁻¹` **exact mutual inverses at the breakpoint**; the decimal form provably cannot be, and **ICC's own reference code demonstrates the resulting inconsistency** — its forward and inverse thresholds disagree by ~4×10⁻⁷. The rational is also the form lcms2 uses and, per the corpus, the modern CIE 15 / ISO 11664-4 statement (that clause itself remains paywalled and unsourced). NC-010 and NC-013 are the properties the choice was made *for*. |
| **Cost — and its exact status** | **~10⁻⁷ in `f`, therefore ~10⁻⁵ in `L*`.** **Evidence class: corpus-derived-bound.** This is an **analytic bound taken from the standards corpus. iccce has NOT measured it.** No test in this repository computes the difference between the two forms, and no row in §3 is a measurement of this quantity. Anyone restating it must not write "measured at 10⁻⁵" — write "bounded analytically at ~10⁻⁵, unmeasured". |
| **What the cost means in practice** | ~10⁻⁵ in `L*` is roughly **five orders of magnitude below the 1.0 ΔE2000 perceptibility anchor** (which is itself ⚠ provisional — DL-004). It cannot affect colour. It **can** affect bit-exact round-trip comparisons against an implementation that uses the decimal form, and that is the only place it will ever show up. |
| **Where** | `crates/iccce-color/src/lab.rs` — module doc §"Named DEVIATION", and `f` / `f_inv`. *(verified — read.)* |
| **Decision record** | `ARCHITECTURE.md` **DL-010**. |
| **Revisit if** | ISO 13655 or CIE 15 / ISO 11664-4 is obtained and states the breakpoint explicitly either way; or a difftest finds the choice visible in a comparison that matters (which would require an error ~5 orders of magnitude larger than the bound, i.e. would indicate a different bug). |

### NA-002 — Bradford is a *policy* choice, not a specification requirement

| Field | Value |
|---|---|
| **The approximation** | `iccce-color` implements the general von Kries **method** and supplies **Bradford** cones. When a profile carries no `chad` tag, a CMM that adapts at all must choose a transform, and iccce's choice is Bradford. |
| **Why it is named** | Corpus ambiguity **A29**, resolved by the ICC.1:2022 ingest as **recommended, not mandated**: ICC.1 requires no particular chromatic-adaptation transform, and a profile's `chad` tag stores the *resulting matrix* rather than naming a method. So the choice is citable as a policy under A29 and **must not be described as conformance.** *(verified — corpus index and `adapt.rs` module doc.)* |
| **Cost** | **UNMEASURED, and not yet exercised.** No transform in this repository currently adapts anything — `iccce-cmm` is still a stub. Per `TOLERANCES.md` §5 an approximation may carry an unmeasured cost *only while the entry is new*; this entry is new today and the cost becomes owed the moment Pass 3 uses it. |
| **What would measure it** | Comparing Bradford against at least one other CAT (von Kries/HPE, CAT02) over a stated sample set, in ΔE2000, on a stated illuminant pair. **Both alternatives are currently unsourceable** — see §5. |

### NA-003 — no clamping in the colour layer

`f_inv` deliberately does **not** clamp below the linear segment. ICC's
own reference code makes negative-XYZ clamping a **compile-time option**
(corpus A9/A11 — the reference implementation declines to decide), so
`iccce-color` computes the unclamped value and leaves gamut policy to the
CMM layer where it can be a named, per-transform decision. *(verified —
`lab.rs::f_inv` doc.)*

This is **not** an approximation with a ΔE cost; it is a layering
decision, registered here so that Pass 4 does not discover it as a
surprise. Note it now sits alongside a **normative** finding from the
ingest: ICC.1:2022 **6.4** states out-of-range colours *"shall be clipped
on a per-component basis"* on integer conversion, while **no clipping is
performed** for float32-based encodings. **That rule binds the CMM/profile
layers, not this crate** — but a future reader must not conclude from
`iccce-color`'s silence that iccce clamps nowhere.

---

## 5. What Pass 1 does **not** claim

Stated as prominently as the claims, because the failure mode of a ledger
is that its existence is read as coverage.

- **No claim of any kind about ΔE94 or ΔE CMC.** Not implemented.
  Deliberately: the corpus has not transcribed their formulas from a
  citable source and no published worked examples are in hand, so an
  implementation today could only be **lcms2-cross-checked** — a strictly
  weaker claim that rule 3 requires labelling as such. Recorded as a gap
  in `delta_e.rs`'s module doc. *(verified.)*
- **No claim about the von Kries (HPE) cone matrix.** Not implemented;
  the corpus's digits are a placeholder marked **DO NOT USE**. Note the
  name is ambiguous between the general *method* (implemented) and that
  specific *matrix* (absent).
- **No claim about CAT02.** CIE 159 is paywalled; not sourced, not needed
  for ICC.1.
- **No claim about observer colour-matching functions.** No CMF table
  exists in the crate; none is needed until spectral input does.
- **No cross-check against lcms2 exists anywhere in `iccce-color`.**
  There is still **no Rust difftest harness** — nothing drives `transicc`
  programmatically (`tools/difftest/README.md` §10, per the Pass 0
  record). Every number on this page is either published ground truth or
  iccce's own arithmetic. **There is not one `implementation-cross-check`
  row in this ledger.**
- **No claim about sRGB constants.** `iccce-color` contains none, and the
  corpus's sRGB file is single-source (lcms2 only; IEC 61966-2-1
  paywalled).
- **No claim about any ICC profile behaviour.** `iccce-color` contains no
  ICC concepts by invariant.
- **No claim that these tests pass on Linux.** They have been reported
  passing on Windows/MSVC on one machine.

---

## 6. Dependency notes — what future work invalidates what

The point of the ledger. When a Pass changes something upstream, these
are the rows to re-run or retire.

| If this changes… | …these rows need re-examination |
|---|---|
| The `f(t)` breakpoint form (ISO 13655 obtained; A11 revisited) | **NA-001** (retire and re-file), NC-010, NC-011, NC-013 (`L* = 0` exactness is a *consequence* of the linear segment) |
| The D50 triple, or a decision to use a higher-precision D50 | NC-005, NC-006, NC-007, NC-012, **NC-017**, NC-015 — and see NC-017's finding: mixing precisions is the exact trap |
| The D65 chromaticity (a second source arrives, or it is corrected) | NC-006, NC-007, **NC-018** |
| `BRADFORD`, or the adaptation method/order | NC-005, NC-006, NC-007, NC-008, **NC-009**, NA-002 |
| The CIEDE2000 implementation, or `Lab` semantics | **NC-001**, NC-002, NC-003 |
| `Mat3::inverse` (e.g. a different algorithm, or an epsilon singularity test) | NC-005, NC-016, and indirectly every adaptation row |
| A Pass 3/4 transform that adapts | **NA-002's cost becomes owed** |
| The 1.0 ΔE2000 anchor (DL-004 revisited) | **Nothing in this ledger.** No Pass 1 row is graded perceptually — which is itself worth knowing. |

---

## 7. Owed, as of 2026-08-11

1. **A commit hash for §2.** Filed uncommitted; every row is anchored to
   a working tree, which is a weaker anchor than a hash.
2. **Observed residuals, not just asserted bounds** (§1.1). Recording the
   maximum residual for NC-001 would turn a gate into a measurement and
   make regression visible before it crosses the gate.
3. **`TOLERANCES.md` §3.1 and §5 rows** — `icc-conformance`'s, untouched
   here by ownership.
4. **A ground-truth row for chromatic adaptation** (§3.3) — currently the
   largest evidential hole in Pass 1.
5. **The corpus D50-chromaticity erratum** (§3.4) — still present at
   filing.
6. **A Linux run of these tests.**

---

## 8. Related

- `docs/TOLERANCES.md` — the tolerance budget (`icc-conformance`).
- `docs/ARCHITECTURE.md` §5 — the decision log; **DL-004** (the
  perceptual anchor), **DL-005** (v2 legacy Lab tested by exact
  invariants), **DL-010** (NA-001), **DL-011** (legacy Lab keys off tag
  type).
- `docs/SESSION_LOG.md` — 2026-08-11, Pass 1.
- `D:\Dev\Rag-Specialized\ICC_Spec\` — the standards corpus. Read a
  file's frontmatter `evidence:` line before citing it; the tiers are not
  equal.
