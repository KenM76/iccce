# NEXT SESSION — start here

**Written 2026-08-11 by `icc-librarian`, at the close of Pass 3's core
and the `transform` CLI.** Replaces the Pass 2 batch 2 edition entirely.
Overwrite this file once acted on.

Read order: this file → `docs/ROADMAP.md` (the **Pass 3 progress
block**, then the **Pass 4** and **Pass 5** annotations) →
`docs/ARCHITECTURE.md` §5 (**sixteen** entries; **DL-015** and **DL-016**
are new) → `docs/NUMERIC_CLAIMS.md` (§1 evidence classes, which gained
one → **§2.3** and **§2.3.1** → **§3.7**, starting with **§3.7.0**,
which is where the missing numbers go → §4's **NA-004**, **NA-005** and
the **dated NA-002 correction** → §5.2 → §7.3) → `docs/TOLERANCES.md` →
`tools/difftest/README.md` §12 → `docs/SESSION_LOG.md` (six entries, all
2026-08-11; the sixth is this work).

---

## Where the project actually is

**Pass 0 done. Pass 1 core complete. Pass 2 built, one scope decision
from done. Pass 3's engine and CLI are built and its done-when is NOT
met.** All on 2026-08-11.

**`iccce` now has a transform — and has still never been compared to
another implementation.** Those two facts are both true today and the
second is the one that matters.

| | Commit *(all **reported** — no agent here has ever run git)* |
|---|---|
| Pass 0 | `f976a0e` |
| Pass 1 | `7313c5b` |
| Pass 2 batch 1 | `b35a12e` |
| difftest harness + probe + `TOLERANCES.md` first filling | `bfd6b1e` |
| Pass 2 batch 2 — the LUT family | `d40d601` |
| **Pass 3 core — curve engine + matrix/TRC** | **`c4038eb`** |
| **`iccce transform`** | **`051707f`** |

- `iccce-color` — XYZ/xyY, Lab/LCh, D50 + D65, von Kries method with
  Bradford cones, ΔE76 and CIEDE2000. **One** published-ground-truth
  claim in the whole project (NC-001).
- `iccce-profile` — header, tag table, eight non-LUT tag types and the
  four LUT types. iccMAX refused by name since Pass 0.
- **`iccce-cmm` — no longer a stub.** Tone curves (evaluate + invert per
  Annex F.1) and the Annex F.3 matrix/TRC model, media-relative only.
- `iccce-cli` — `inspect` and **`transform`** (stdin triples, 6
  decimals, no banner: the difftest interface).
- `tools/difftest` — the harness, one registered check, and the
  legacy-Lab probe.

### What is easy to over-read, so read it here first

- **A transform existing is not a transform being right.** Zero
  `implementation-cross-check` rows. Every one of Pass 3's twelve ledger
  rows has iccce on both sides, or iccce on one side and a
  **specification clause** on the other.
- **"68 green" is not 68 assertions.** **Two of Pass 3's fourteen tests
  skip silently** when `C:\Windows\System32\spool\drivers\color\` has no
  sRGB profile — they `eprintln!` and return. `cargo test` cannot
  distinguish *passed* from *did not run*, and those two are the only
  tests that touch a real profile.
- **NC-032 is not the done-when's ΔE.** It is a round trip through
  **one** profile in **device units**, source and destination the same,
  so the matrix and its inverse cancel and it prices only the curve
  stack.
- **Pass 3 does NOT adapt.** The previous edition of this file said
  NA-002's cost came due here. **It does not** — `iccce-cmm` never calls
  `adapt.rs` and never reads `chad`/`wtpt`. Bradford remains unexercised
  by any transform. (New register entry **NA-005** covers the
  assumption that replaces it.)
- **Pass 2 is still in progress too.** The Passes are no longer
  completing in order; do not read "Pass 3 landed" as "Pass 2 closed".
- **Nothing has run on Linux, and no CI run has ever been observed.**

---

## The immediate next step: **close Pass 3**, which needs numbers, not code

### 1. ★ The two done-when numbers — dispatched, landing unverified

`icc-conformance` was dispatched **in parallel with the filing that
produced this file** to measure:

- **the sRGB→AdobeRGB round-trip ΔE**, and
- **the lcms2 agreement tolerance** — which will be **this ledger's
  first `implementation-cross-check` row** and the first time iccce has
  been compared to anything.

**Check whether they landed. Do not assume.** Look in
`NUMERIC_CLAIMS.md` §3.7 (the next free number is **NC-034**; none is
reserved) and `TOLERANCES.md` §3.3. Three consecutive filings have now
found an item carried as outstanding was in fact done — and this filing
found the reverse, a prediction carried as fact that the code refuted.
**Check the live source either way.**

Standing constraints on those numbers when they arrive:

- **The tolerance is justified before the run, never fitted after it**
  (rule 5, `TOLERANCES.md` §0). A number moved until the suite went
  green is not a tolerance.
- **The round-trip half is `self-consistency`** and must be labelled so
  even when it looks reassuring — both sides are iccce.
- **The sRGB agreement may be agreeing for the wrong reason.** The
  corpus's sRGB constants rest on **lcms2 alone** (IEC 61966-2-1
  paywalled) and **D65 is single-source** (NC-018). Two implementations
  taking their primaries from the same place is the **shared-misreading**
  case, and the weakest form of cross-check there is.
- **DL-016's lesson applies to whatever bound is chosen**: a bound
  derived from a table's own spacing cannot discriminate an error whose
  magnitude *is* that spacing.

### 2. The Pass 2 clause-2 scope decision — still open, still not code

*"A synthetic corpus covers each tag type."* Every implemented tag type
has hand-authored byte fixtures **inside the unit tests**;
`tools/gen-profiles/` still does not exist and `fixtures/synthetic/`
still holds only its README *(verified — enumerated 2026-08-11)*. Both
readings are stated in `ROADMAP.md`'s batch 2 block, neither
recommended. **If the answer is "in-test suffices", that is a
decision-log entry** — it narrows a done-when that was written to mean
something else.

---

## Pass 3's remainder — three items, and only one is engineering

### ★ 1. ICC-absolute intent — a NEW NAMED CORPUS GAP, blocked on sourcing

The **media-relative → absolute white-point adjustment formula is not
transcribed in `ICC_Spec`.** `iccce-cmm` refuses the intent by name and
its module doc records that the formula **will not be written from
memory** (rule 2). This is `icc-spec-librarian`'s to close.

**It is expected to be in ICC.1:2022 clause 6.x or an Annex — and
"expected to be" is a prediction until the document is open.** The same
caution the corpus's own record demands: a claim about what a document
contains is a claim.

Note the shape of the thing when it arrives: `ARCHITECTURE.md` §2
already records that **absolute is media-relative plus a white-point
adjustment, not a fourth table**, and `matrix_trc.rs` records that the
`wtpt`/`chad` machinery exists for exactly this. So this is where
**NA-005** ("colorants as stored; `chad`/`wtpt` never consulted") gets
revisited, and plausibly where **NA-002's** unmeasured Bradford cost
finally comes due.

### 2. Parametric inverses for function types 1, 2 and 4

Types 0 and 3 are analytic and implemented — the shapes real display
profiles carry. The others are **refused by name**
(`InverseUnsupported { func_type }`) because a sampled inverse is an
approximation and an approximation needs a **measured** cost (rule 4).
This one is analytic work, not sourcing work.

### 3. A policy for perceptual and saturation on matrix/TRC profiles

The module doc's expectation — that they equal media-relative there,
*"which is what lcms2 does with them too"* — is explicitly labelled *"an
unverified expectation here, not a claim; the differential test owns
it."* **That is the right shape; go and settle it.** And carry
**DL-013**'s hazard: at perceptual and saturation against a **v4**
profile, lcms2 is running a transform with **forced BPC** in it (≈3.15
`L*` at black), so the comparison is not the one it looks like.

---

## Then: **Pass 4 — LUT transforms and rendering intents**

`A2B`/`B2A`, multi-dimensional interpolation, all four intents. **Done
when**: CMYK→RGB through a real press profile matches lcms2 within
tolerance at every intent, and the v2/v4 cases are separately covered.

### What Pass 4 inherits, all live from its first commit

1. **Its done-when is known to be underspecified.** *"At every intent"*
   collides with **DL-013**: against a v4 profile, lcms2 forces BPC on at
   perceptual and saturation. Pass 4 must either account for it
   explicitly or restrict the cross-check to the colorimetric intents
   **and say which** — widening a tolerance to swallow ≈3.15 `L*` is not
   available.
2. **The v2/v4 Lab encoding selector is settled: the TAG TYPE**
   (DL-011), and lcms2 at the pin **agrees** (DL-012, measured). No
   runtime divergence warning is owed. **DL-005 still stands**: assert it
   with **exact-value integer invariants, never ΔE**.
3. **DL-016 generalises to every table Pass 4 touches.** CLUT grids,
   `mft1`/`mft2` input and output tables: **the endpoint is where an
   off-by-one hides best**, because it is the one place a clamp exists
   to be paired wrongly. Assert exact values at the sample points; a
   spacing-derived self-consistency bound will not catch it.
4. **CLUT interpolation is the asymmetry.** The corpus confirms **A16
   SILENT** — ICC.1 does not specify the interpolation between CLUT grid
   points. So Pass 4's interpolation is a **named, measured
   approximation** (rule 4, register it in `NUMERIC_CLAIMS.md` §4),
   whereas Pass 3's was specification-following and needed no such entry.
5. **NA-002 and NA-005 both come due here if any path adapts.**
6. **The F.3 NOTE's `(32 768/65 535)` scale factor** applies when a
   matrix/TRC model is expressed as a `lutAToBType`. It is `0.500 003 8…`,
   **not** ½; deriving it as ½ is a ≈7.6 ppm error that is invisible in
   colour forever and breaks bit-exact comparison against any correct
   implementation. *(corpus `icc__s__computational_models.md` §F.3
   NOTE.)*

---

## Owed work, carried explicitly

### 1. `icc-conformance`

- **The Pass 3 done-when numbers** (above). Until they exist Pass 3 is
  not done.
- **`TOLERANCES.md` §3.2's four Pass 2 rows** and **§3.3 for Pass 3**;
  **§6's coverage table** still read *"2–8 | not started"* at the last
  check.
- **Twin rows for §3.7's twelve tolerances** — every Pass 3 bound is
  recorded in the ledger and none has a budget entry yet.
- **A behavioural test of `ncl2` and of B2A** legacy-Lab decoding
  (NC-019's coverage still rests on a source reading for both).
- **The Pass 4/5 decision on whether iccce copies lcms2's forced BPC**
  (DL-013) — still undecided; Pass 3 deferred rather than discharged it.
- **An observed residual for NC-032**: the cheapest number in the ledger
  to obtain, and it would turn DL-016's reconstruction into a
  measurement.

### 2. `icc-spec-librarian`

- **★ The ICC-absolute white-point adjustment formula** — the new named
  gap, and the only thing blocking absolute intent.
- **The ITU terms determination** before any BT.709 fetch (DL-007).
  *"Free download"* is not *"automated retrieval permitted."*
- The standing operator-download list below.

### 3. `icc-engineer`

Three prose defects, **reported not repaired** because the files are
not the librarian's:

- `matrix_trc.rs` — the round-trip bound is justified as *"~2× the
  table's input spacing (1/1023)"*, but `1/1023 = 9.775×10⁻⁴`, so
  `1×10⁻³` is **≈1.02×** that. The *"~2×"* reading holds only against
  the **half**-spacing. The bound is fine; **its stated derivation is
  what a reader checks it by.**
- `curve.rs` — the `pow` guard is said to yield a *"defined, **reported**
  value"*. It is defined; **nothing reports it** (`Trc::eval` returns a
  bare `f64`).
- `iccce-cmm/src/lib.rs` §Status still reads **"Pass 0 scaffold.
  Matrix/TRC transforms are Pass 3"** on a crate that now contains them.

### 4. `icc-librarian` / whoever files next

- **An audit of the pre-existing ICC.1:2022 citations** against DL-014.
  §2.3.1 audited **only** Pass 3's five new sites; `iccce-color` and
  `iccce-profile` doc comments predate the terms and **nobody has swept
  them.**
- **A per-tag-type breakdown of the Pass 2 sweep.**
- **Observed residuals for Pass 1's rows** (§1.1).
- **A ground-truth row for chromatic adaptation** — still the largest
  evidential hole in the project, and **no longer on a clock** now that
  Pass 3 turns out not to adapt.
- **A Linux run of anything at all.**

---

## Optional operator unblocks — cheap, and each settles something named

**All are browser downloads by Ken, not agent retrievals.**

| Document | What it settles |
|---|---|
| **`ICC.1:2010-12` (v4.3)** | **A31 / D10** — what changed in `parametricCurveType` **Table 68** between editions. **NOT SOURCED; do not guess it.** Directly Pass 3/4 material: two conformant CMMs on different editions can evaluate the same `'para'` tag differently |
| **`ICC.1:2001-04` (v2)** | **A1b, A2, A34** — the only normative home of `textDescriptionType` |
| **ICC's published D65→D50 `chad` values** (Annex E.4.2) | the one place the adaptation ground-truth hole could be partly filled from published values |
| **ITU-R BT.709** | a **second source** for sRGB primaries and D65, both currently lcms2-only — which is exactly what would strengthen the Pass 3 lcms2 cross-check from *"agrees, possibly for the same wrong reason"* |

**Each row is a claim about what a document contains.** Treat *"it would
settle A2"* as a prediction until the document is open.

---

## Decisions already made — do not re-litigate

- **MIT**, dependencies permissive, **publishing is the operator's act**
  (rule 9). DL-009 records an *intent*; **intent is not authorisation.**
- **`iccce-color` depends on nothing** and contains no ICC.
- **The parser reports, it does not repair.** And in the CMM the same
  instinct is **refuse by name, never substitute** — four instances in
  Pass 3 alone.
- **No iccMAX execution, no display calibration.** Profile *creation*
  was reversed by the operator → Pass 10, DL-008, with the
  validation-hardware problem carried forward.
- **lcms2 is the oracle, never a dependency** — subprocess only, pinned
  by **commit hash** (DL-001). `tools/difftest` is deliberately not a
  workspace member; folding it in would undo both the licence insulation
  and the publication guard.
- **DL-003** — duplicate tag signatures: keep both, consumers take the
  first, report the duplicate. `MatrixTrc::from_profile` follows it.
- **DL-004** — the 1.0 ΔE2000 anchor is a conservative design choice,
  ⚠ provisional; anything derived from it inherits the ⚠.
- **DL-005** — v2 legacy Lab tested by **exact-value invariants, not ΔE**.
- **DL-007** — HDR in scope (Pass 9), blocked on ITU-R terms.
- **DL-010 / NA-001** — the Lab `f(t)` breakpoint uses the exact
  rational form; its cost is **bounded analytically** and must never be
  restated as measured.
- **DL-011 / DL-012** — legacy Lab keys off the **tag type**; the
  predicted lcms2 disagreement was **measured absent**.
- **DL-013** — lcms2 forces BPC on v4 perceptual/saturation. The
  standing caveat for every perceptual-intent comparison.
- **DL-014** — the terms for citing ICC.1:2022 clause numbers: **name
  the corpus file**, and the tier is **per-fact, not per-file**. It does
  **not** retroactively bless existing citations.
- **DL-015** *(new)* — the parametric `pow` guard follows lcms2 over
  ICC's sample code. **A choice inside a stated non-requirement, not a
  deviation from normative text** — the register (§4) now distinguishes
  the two kinds.
- **DL-016** *(new)* — sampled tables are asserted by **exact values at
  the sample points**. A self-consistency bound derived from the table's
  spacing **cannot** catch an off-by-one-sample error, because the error
  is exactly that spacing.

### Everything measured against lcms2 is scoped to commit `21c582a`

Moving the pin is a **licence** event (DL-001) **and** a behavioural one
(DL-012, DL-013): **NC-019, NC-020 and NC-021 must be re-run, not
re-read.**

---

## Method reminders that stay load-bearing

1. **A wrong colour looks exactly like a right one.** Pass 3's live
   examples: `TRC(1.0) = 0.998`; and clamping **after** the inverse TRC
   instead of before, whose symptom the corpus records as *"the gamut
   boundary is subtly the wrong shape"* — and flags **Quiet**.
2. **Never write colour maths from memory.** The absolute-intent formula
   is the current live instance: it is **refused** rather than guessed.
3. **Expected values come from the literature.** New this Pass: where
   the expectation is **verbatim normative text**, the class is
   `normative-rule-conformance` — stronger than an identity, weaker than
   a published dataset, and it **inherits the transcription risk**.
4. **Every approximation is named and measured.** NA-004 and NA-005 are
   new; **both carry unmeasured costs**, which §4 permits only while an
   entry is new.
5. **Tolerances are justified, not tuned.** Now with five worked
   examples, of which two are Pass 3's: the colorant-sum bound
   re-justified by **what it discriminates**, and the round-trip bound
   that **would have absorbed a real bug** (DL-016).
6. **Coverage is part of every claim.** *"68 green"* includes two tests
   that assert nothing on a machine without the system sRGB profile.
7. **Do not assert unmeasured facts about the environment.** *verified* /
   *reported* / *unverified* are distinguished on purpose. **No agent
   here has ever run a git command**; every commit hash is reported.
8. **Check the live source — including your own last filing.** This
   session's correction was to a prediction `icc-librarian` had filed
   **twice** (NA-002's cost coming due at Pass 3). The dispatch did not
   contradict it; **the code did.**

---

## The agents

- **`icc-engineer`** — lead. Be this agent if orchestrating.
- **`icc-spec-librarian`** — the standards corpus. Dispatch for *every*
  sourcing question. **Owes** the absolute-intent formula and the ITU
  terms determination.
- **`icc-conformance`** — the oracle, the fixtures, the tolerance
  budget. **Owes** the Pass 3 done-when numbers, `TOLERANCES.md` §3.2 /
  §3.3 / §6, the `ncl2`/B2A behavioural tests, and the forced-BPC
  decision.
- **`icc-librarian`** — ROADMAP, decision log, session log, and
  `NUMERIC_CLAIMS.md`. **No shell** — a dispatch to it must carry its
  evidence.

Dispatch them freely, and in parallel on disjoint file sets; no
permission is needed to dispatch an agent to read, analyse or draft.
