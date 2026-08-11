# NEXT SESSION — start here

**Written 2026-08-11 by `icc-librarian`, at the close of Pass 2 batch 2
and the machine-wide sweep.** Replaces the batch-1 edition entirely.
Overwrite this file once acted on.

Read order: this file → `docs/ROADMAP.md` (the **Pass 2 batch 2 progress
block**, then the **Pass 3 annotation**, then the dated annotations under
**Pass 4** and **Pass 5**) → `docs/ARCHITECTURE.md` §5 (**fourteen**
entries; **DL-014** is new and governs how every clause number in this
project may be cited) → `docs/NUMERIC_CLAIMS.md` (§1 evidence classes →
§2.2 and **§2.2.1**, the sweep and why it has no NC number → §7.2) →
`docs/TOLERANCES.md` §1–§2, **§3.2** and **§6.1** →
`tools/difftest/README.md` §12 → `docs/SESSION_LOG.md` (five entries, all
2026-08-11; the fifth is this work).

---

## Where the project actually is

**Pass 0 done. Pass 1 core complete and validated. Pass 2 is built —
both batches — and one scope decision away from done. `iccce-cmm` is
still a stub: there is no transform, and `iccce` has still never been
compared to anything.** All on 2026-08-11.

| | Commit *(all **reported** — no agent here has run git)* |
|---|---|
| Pass 0 | `f976a0e` |
| Pass 1 | `7313c5b` |
| Pass 2 batch 1 | `b35a12e` |
| difftest harness + probe + `TOLERANCES.md` first filling | `bfd6b1e` |
| Pass 2 batch 2 — the LUT family | **`d40d601`** |

- `iccce-color` — XYZ/xyY, Lab/LCh, D50 + D65, von Kries method with
  Bradford cones, ΔE76 and CIEDE2000. **One** published-ground-truth
  claim in the whole project (NC-001, all 34 Sharma pairs within 1×10⁻⁴).
- `iccce-profile` — header, tag table, **eight non-LUT tag types**
  (`curv`, `para`, `text`, `mluc`, `desc`, `ncl2`, `XYZ `, `sf32`) and
  **the four LUT types** (`mft1`, `mft2`, `mAB `, `mBA `). iccMAX is
  identified and **refused by name** (since Pass 0). All wired into
  `inspect`, which prints every `TagIssue` unconditionally.
- `tools/difftest` — zero-dependency, out-of-workspace harness driving
  `transicc`, plus `legacy_lab_probe`.
- `iccce-cmm` — **still a stub.**

### What is easy to over-read, so read it here first

- **"54 tests green" is not coverage.** Exactly **one** test in the
  workspace is a correctness claim against published values. The rest
  are arithmetic identities and parser fixtures.
- **"40 of 40 profiles parse" is one machine, one day, one commit.** It
  is not *"iccce parses real profiles"*. **No per-tag-type breakdown was
  taken**, so it does not establish that the LUT decoders met real input
  at all — and a Windows colour directory is the wrong shape for
  `mAB `/`mBA `, which live in large v4 CMYK press profiles.
- **`iccce` has still never been compared to anything.** Zero
  `implementation-cross-check` rows in the ledger, correctly: such a row
  needs iccce on one side, which needs a transform, which is **Pass 3**.
- **The difftest harness still has exactly one registered check and it
  compares lcms2 to lcms2** (NC-021). With no oracle on the machine every
  check skips and the runner exits **3 ("nothing ran")** — never 0.
- **Chromatic adaptation still has no ground-truth row.** Unchanged since
  Pass 1, still the largest evidential hole — and **Pass 3 makes it
  urgent** (below).
- **Nothing has run on Linux, and no CI run has ever been observed.**

---

## The immediate next step is a **decision**, not code

### ★ Pass 2 done-when clause 2: does an in-test synthetic satisfy it?

Clause 1 (*"every profile on the machine parses or is refused with a
reason"*) is **met on this machine's 40 profiles**. Clause 2 (*"a
synthetic corpus covers each tag type"*) is **PARTIAL**, stated exactly:

- **Every implemented tag type has hand-authored synthetic byte
  fixtures** — inside the unit tests, hostile cases included (`255^255`
  CLUT dimensions, `precision = 3`, a curve chain broken at a named
  position). *(verified.)*
- **`tools/gen-profiles/` does not exist.** `fixtures/synthetic/`
  contains only a `README.md` that says so itself. `fixtures/reference/`
  holds only `PROVENANCE.md`. *(verified — tree enumerated.)*

**The two readings, neither recommended:**

| | For | Against |
|---|---|---|
| **In-test bytes suffice** | byte-authored (category (a), cannot inherit a bug from the code under test), versioned, executed on every `cargo test` — more than a directory of blobs guarantees | they are **tag-level, not whole profiles**: no header/tag-table/tag-data interaction, no cross-tag consistency, and unusable by a differential run, a fuzzer, or any external consumer |
| **The generator is required** | `ARCHITECTURE.md` §1 listed both directories before the plan was written, which is evidence the author meant files on disk; `difftest/README.md` §10 says the four probe profiles should be **ported onto the generator when it exists** | it is real work for a benefit Pass 3 does not need |

**Decide it, record it in `ROADMAP.md`, and if the answer is "in-test
suffices" that is a decision-log entry** — it narrows a done-when that
was written to mean something else, and a quietly narrowed done-when is
how a Pass gets called complete without being it.

---

## Then: **Pass 3 — matrix/TRC transforms**

The analytic path: RGB→XYZ→RGB through matrices and tone curves, with
adaptation. **Done when**: sRGB→AdobeRGB round-trips within a stated ΔE,
and matches lcms2 within a stated tolerance, with both numbers written
down.

### ★ Four things Pass 3 inherits, all live from its first commit

**1. It produces the ledger's first `implementation-cross-check` row.**
*"Matches lcms2"* puts **iccce on one side of a comparison for the first
time in the project's history.** `NUMERIC_CLAIMS.md` §5.1's sentence
*"iccce has never been compared to anything"* stops being true that day.
**Set and justify the tolerance before the run** (rule 5,
`TOLERANCES.md` §0) — a tolerance chosen after seeing the residual is a
number someone moved until the suite went green. The round-trip half is
**`self-consistency`** and must be labelled so: it is the only way to
*price* an approximation and it is worthless as correctness evidence.

**2. NA-002's cost comes due.** `NUMERIC_CLAIMS.md` §4 registers
**Bradford as a policy choice, not conformance** (corpus **A29** — ICC.1
mandates no chromatic-adaptation transform, and a `chad` tag stores the
*resulting matrix*, not the method), with its cost **UNMEASURED** —
permitted *"only while the entry is new."* **sRGB→AdobeRGB adapts.**
What would measure it: Bradford against at least one other CAT, over a
stated sample set, in ΔE2000, on a stated illuminant pair. **Both
alternatives are currently unsourceable** — the corpus's von Kries/HPE
digits are a placeholder marked **DO NOT USE**, and CIE 159 (CAT02) is
paywalled and not obtained. So either source one, or **write into NA-002
that the cost cannot be measured yet and why**. Letting it lapse quietly
is the one option that is not available.

**3. The sRGB and D65 constants are single-source, and Pass 3 is built
on them.** The corpus's sRGB file rests on **lcms2 alone** (IEC
61966-2-1 paywalled, not obtained), and NC-018 records **D65 as the
weakest constant in `iccce-color`** — chromaticity from `cmsvirt.c`
alone, *not* cross-verified, unlike D50 and Bradford. **The candidate
second source is ITU-R BT.709, recorded in the corpus as free from
itu.int and NOT FETCHED.** *(verified — `ICC_Spec\index.md`.)* Two
consequences: a Pass 3 sRGB result that agrees with lcms2 **may agree
because both took their primaries from the same place** — the
shared-misreading case `TOLERANCES.md` §1 names, and the weakest form of
cross-check there is; and **fetching BT.709 is blocked on DL-007's
determination**: ITU's terms must be read before any agent fetches,
because *"it is a free download"* is not *"automated retrieval is
permitted."* That is `icc-spec-librarian`'s call.

**4. Curve work is now specification-following, not
choose-something-reasonable.** The ICC.1:2022 ingest found **Annex F is
NORMATIVE and fully specifies curve inversion**, and **10.6 mandates
linear interpolation for `curveType`** — the corpus's A15/A17 were not
merely unverified but **wrong**. Cite them under **DL-014**'s terms
(below). Note the asymmetry the corpus itself flags: **A16, CLUT
interpolation, is confirmed SILENT**, so Pass 4's interpolation stays a
named, measured approximation while Pass 3's does not.

---

## ★ New this session: **DL-014** governs every clause citation you write

DL-002's prohibition — *"no claim in this project may cite an ICC.1
clause number"* — **is now lifted for ICC.1:2022 only, on terms**:

1. **Name the corpus file** alongside the clause. The corpus is the
   verification trail; a bare clause number is an assertion.
2. **The tier is per-fact, not per-file.** `ICC_Spec\index.md` records
   **15 of 20 files at `primary_spec` — 4 fully, 11 partly**. Read the
   file's `evidence:` line **every time**. Worked example, and it is the
   one batch 2 depends on: `icc__type__lutAtoB_lutBtoA.md` reads
   `evidence: primary_spec (clause numbers + the CLUT/interpolation
   rules) / icc_secondary_code (byte layouts — NOT re-transcribed this
   pass)`. Its clause numbers are citable; **its byte tables are not**,
   and A23/A24 stay open. `crates/iccce-profile/src/lut.rs` §Sourcing is
   the shape to copy.
3. **Still prohibited for every unread document** — ICC.1:2010,
   ICC.1:2001-04, ISO 13655, CIE 142 / ISO-CIE 11664-6 / CIE 15 / ISO
   11664-4 / CIE 159, IEC 61966-2-1, and **"Adobe's document"** (DL-013),
   which is an attribution transcribed from a code comment, not a
   citation anyone here can check.
4. **DL-014 does NOT retroactively bless existing citations**, and **no
   audit of them has been done by anyone.** Doc comments in
   `iccce-color` and `iccce-profile` predate the terms.
5. **DL-002's other half is untouched:** automated retrieval from
   color.org / archive.color.org remains prohibited. ICC.1:2022 was
   cleared by **human** retrieval, which created no route for agents.
   **Do not re-attempt it.**

---

## Owed work, carried explicitly

### 1. `icc-conformance`

- **`TOLERANCES.md` §3.2's four Pass 2 rows** — all still `—` in
  Tolerance, Justification and Measured, with both batches built.
  §3.2's own preamble says the numeric ones (`s15Fixed16Number` decode,
  curve evaluation) are listed *"so that they are not forgotten"*.
- **§6's coverage table still reads "2–8 | not started"** while Pass 2
  is built and 40 profiles have been swept. *(verified — read
  2026-08-11; not edited, by ownership.)*
- **A behavioural test of `ncl2` and of B2A** legacy-Lab decoding.
  NC-019's coverage still rests on a **source reading** for both, and
  batch 2 has now shipped the B2A-side decoder, so the fixture half is
  cheaper than it was.
- **The Pass 4/5 decision on whether iccce copies lcms2's forced BPC**
  (DL-013) — still undecided, and until it is made no
  perceptual/saturation tolerance against a v4 profile can be justified.

### 2. `icc-spec-librarian`

- **Both items previously owed here are DISCHARGED** — verified, not
  assumed: DL-002's successor is filed (as **DL-014**, by
  `icc-librarian` on dispatch), and **the corpus retraction landed**
  (`icc__ref__v2_v4_divergence.md` C3, `index.md`, and the new
  `icc__ref__lcms2_measured_behaviour.md` M1/M2).
- **New:** the **ITU terms determination** before any BT.709 fetch
  (above, and DL-007). And the standing operator-download list below.

### 3. `icc-librarian` / whoever files next

- **A per-tag-type breakdown of the sweep** — it would turn a robustness
  observation into a coverage statement, and would show whether this
  machine contains *any* profile exercising batch 2's code.
- **An audit of existing ICC.1:2022 citations** against DL-014's terms.
- **Observed residuals for Pass 1's rows** (§1.1) — NC-001 and every
  identity still carry only the bound asserted; a residual that grew from
  10⁻¹² to 9×10⁻⁵ would still pass its gate and nothing would show it.
- **A ground-truth row for chromatic adaptation** — still the largest
  evidential hole in the project.
- **A Linux run of anything at all.**

### 4. Pass 1's remainder — all blocked on sourcing, none on engineering

Unchanged. Land each **when, and only when, a citable source arrives**:
**ΔE94** and **ΔE CMC(l:c)** (formulas not transcribed from a citable
source, no published worked examples); the **von Kries (HPE) cone
matrix** (corpus digits are a placeholder marked **DO NOT USE**);
**CAT02** (CIE 159 paywalled, and not needed for ICC.1). **Observer CMF
tables are not blocked — they are not needed**; no Pass plans spectral
input.

---

## Optional operator unblocks — cheap, and each settles something named

**All are browser downloads by Ken, not agent retrievals.**

| Document | What it settles |
|---|---|
| **`ICC.1:2010-12` (v4.3)** | **A31 / D10** — *what* changed in `parametricCurveType` **Table 68** between editions. Two conformant CMMs on different editions can evaluate the same `'para'` tag differently, and **what changed is NOT SOURCED — do not guess it.** Directly Pass 3 material |
| **`ICC.1:2001-04` (v2)** | **A1b, A2, A34** — and it is the **only normative home of `textDescriptionType`**, which `desc` is decoded from a **code-derived** layout |
| **ICC's published D65→D50 `chad` values** (Annex E.4.2) | the one place Pass 1's biggest hole — no ground-truth row for adaptation — could be partly filled from published values. **Directly relevant to Pass 3** |
| **ITU-R BT.709** | a **second source for sRGB primaries and D65**, both currently lcms2-only. **Free from itu.int** — but see DL-007: the terms must be read before an *agent* fetches. A human download is outside that question entirely |

**Each row is a claim about what a document contains**, made by the agent
that read the corpus. Treat *"it would settle A2"* as a prediction until
the document is open.

---

## Decisions already made — do not re-litigate

- **MIT**, dependencies permissive, **publishing is the operator's act**
  (rule 9). DL-009 records an *intent* to publish; **intent is not
  authorisation.**
- **`iccce-color` depends on nothing** and contains no ICC. Check it
  stays true.
- **The parser reports, it does not repair.** Both batches enforce this
  in the type design; the sweep exercised it on real malformed files.
- **No iccMAX execution, no display calibration.** (Profile *creation*
  was **reversed by the operator** 2026-08-11 → Pass 10, DL-008, with the
  validation-hardware problem carried forward as its precondition.)
- **lcms2 is the oracle, never a dependency** — subprocess only, pinned
  by **commit hash** (DL-001). No crate under `crates/` may acquire an
  lcms2 dependency, not even a dev-dependency. `tools/difftest` is
  **deliberately not a workspace member**; a future "tidy-up" that folds
  it in would silently undo both the licence insulation and the
  publication guard.
- **The `pdfce` bridge is built in `pdfce`.** `iccce` must not know what
  a PDF is.
- **DL-003** — duplicate tag signatures: keep both, consumers take the
  first, report the duplicate.
- **DL-004** — the 1.0 ΔE2000 anchor is a conservative **design choice**,
  ⚠ provisional; anything derived from it inherits the ⚠.
- **DL-005** — v2 legacy Lab tested by **exact-value invariants, not
  ΔE**. The error is ≈0.3–0.5 ΔE, *below* the anchor, so a ΔE-graded test
  **passes while the encoding is wrong**.
- **DL-007** — HDR in scope (Pass 9), transfer functions and primaries
  only; blocked on ITU-R documents *and* on establishing that `itu.int`'s
  terms permit retrieval.
- **DL-010** — the Lab `f(t)` breakpoint uses the **exact rational**
  form; cost **bounded analytically** at ~10⁻⁵ in `L*` and **never to be
  restated as measured**.
- **DL-011** — legacy Lab keys off the **tag type**. **DL-012** — the
  disagreement that rule predicted is **measured absent** at the pin; the
  runtime warning does not get written. Both are now restated in
  `lut.rs`'s module doc, as two separate objects.
- **DL-013** — lcms2 forces BPC on v4 perceptual/saturation. The standing
  caveat for every perceptual-intent comparison; **any** differential
  measurement there is measuring a transform with BPC in it.
- **DL-014** *(new)* — the terms for citing ICC.1:2022 clause numbers.

### Everything measured against lcms2 is scoped to commit `21c582a`

Moving the pin was already a **licence** event (DL-001). DL-012 and
DL-013 make it a **behavioural** one: **NC-019, NC-020 and NC-021 must be
re-run, not re-read**, if the pin ever moves.

---

## Method reminders that stay load-bearing

1. **A wrong colour looks exactly like a right one.** Batch 2's live
   examples: the `mft1`/`mft2` 4-byte shift, and the `mAB ` 3×4 matrix
   read as 36 bytes — which drops the three offset terms and produces
   *"a uniform colour cast that looks like a white-point problem."* Both
   are now unrepresentable in the type system rather than merely tested.
2. **Never write colour maths from memory.** Dispatch
   `icc-spec-librarian`; cite under DL-014's terms.
3. **Expected values come from the literature.** lcms2-only makes it a
   **cross-check**. Where **neither side is iccce**, it is
   `oracle-behaviour-at-pin` and proves nothing about iccce.
4. **Every approximation is named and measured.** **NA-002 is on the
   clock** — Pass 3 adapts.
5. **Tolerances are justified, not tuned.** Three worked examples now:
   Pass 1's D50-chromaticity failure (**the corpus was wrong**); the
   probe's intent-0 result matching neither hypothesis (**refusing to
   round it produced DL-013**); and this session's drafted claim that
   iccMAX refusal was undelivered (**the live source refuted the
   librarian's own draft**).
6. **Coverage is part of every claim.** *"40 of 40 profiles parse"* is
   one machine, one day, one commit, with no per-tag-type breakdown. **A
   count is not an inventory.**
7. **Do not assert unmeasured facts about the environment.** *verified* /
   *reported* / *unverified* are distinguished on purpose. **No agent
   here has run a git command**; every commit hash in these documents is
   reported.
8. **Check the live source, do not trust the last filing's status.** Two
   consecutive filings have now found an item carried as outstanding was
   in fact done — the corpus D50 erratum, then the corpus retraction and
   the probe doc-comment fix.

---

## The agents

- **`icc-engineer`** — lead. Be this agent if orchestrating.
- **`icc-spec-librarian`** — the standards corpus. Dispatch for *every*
  sourcing question. **Owes** the ITU terms determination.
- **`icc-conformance`** — the oracle, the fixtures, the tolerance budget.
  **Owes** `TOLERANCES.md` §3.2 and §6, the `ncl2`/B2A behavioural tests,
  and the forced-BPC decision.
- **`icc-librarian`** — ROADMAP, decision log, session log, and
  `NUMERIC_CLAIMS.md`. **No shell** — a dispatch to it must carry its
  evidence.

Dispatch them freely; no permission is needed to dispatch an agent to
read, analyse or draft.
