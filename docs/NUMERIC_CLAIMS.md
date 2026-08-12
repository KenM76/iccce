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

> **Dated correction, 2026-08-11 (Pass 2 / difftest filing).** The
> paragraph above is left as written, being the record of what was true
> at the Pass 1 filing. **Both sections have since been filled** by
> `icc-conformance`: `TOLERANCES.md` §3.1 carries Pass 1's tolerances
> (each mirroring a row here), §5 carries NA-001…NA-003, and §4 records
> both as *"first filling, not a change"*. §6.1 there additionally
> records the two findings behind §3.6 below. *(verified — read this
> session.)* **`TOLERANCES.md` §3.2 (Pass 2) and §3.3–§3.6 are still
> blank**, which is correct: those comparisons have not been run.

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
| **derived-expectation** *(class added 2026-08-11, Pass 4b filing — defined in `TOLERANCES.md` §3.4.4.1, rows in §3.11)* | The expected value is computed by **arithmetic** from (a) the specification's stated element order and encoding, as the corpus transcribes them, and (b) the bytes of a **synthetic fixture this project authored**. **No implementation's output is in it** — neither iccce's nor lcms2's. | **Not ground truth**: nobody at the CIE or the ICC printed the number; a reader of this repository derived it from clause text. **Stronger than an implementation-cross-check**, because a cross-check is defeated when two implementations share a misreading whereas this is defeated only when **the derivation** shares one — and the derivation sits next to the number in a form a specification reader can check without running anything. **What it cannot do:** the fixture and the derivation are read out of the **same corpus** by the same project, so a wrong corpus transcription makes them wrong **together** and they agree perfectly. That is why **every derived row in §3.11 is paired with an lcms2 cross-check over the same points** — the third reading. **The definition is `icc-conformance`'s** (`TOLERANCES.md` §3.4.4.1); this row exists because §1's own rule is that *a row without a class is not finished*, and four §3.11 rows carry this class. |
| **implementation-cross-check** | Agreement with lcms2 or another independent implementation. | Evidence that two implementations read a clause the same way. Two implementations can share a misreading (`TOLERANCES.md` §1). **Weaker than ground truth and must be labelled so.** |
| **oracle-behaviour-at-pin** *(class added 2026-08-11, Pass 2 filing — see §3.6)* | A measurement of **what the oracle does**, at a named commit, with **iccce not in the loop at all**. Either side of the comparison is lcms2 or a hand-transcription of lcms2's own arithmetic. | Establishes what iccce **will be compared against**, and nothing else. It is **not** evidence that iccce is correct (iccce did not participate) and **not** evidence that lcms2 is correct (the specification is the authority, not the implementation — rule 7). Every such row is scoped to one pin, and **the pin moving invalidates it**. |
| **normative-rule-conformance** *(class added 2026-08-11, Pass 3 filing — see §3.7)* | The expected **behaviour** is derived from **verbatim normative specification text** transcribed in the corpus at `primary_spec` tier — not from a published numeric dataset, not from another implementation, and not from iccce. | Proves the implementation does what the clause says, **as the corpus transcribes the clause**. It therefore inherits the **transcription risk**: one PDF extraction pipeline, cross-checked against others but not read from paper by anyone here. Weaker than **published-ground-truth** (whose datasets are adversarially designed to catch a wrong reading); **stronger than arithmetic-identity**, because the expectation comes from outside the code. Distinct from **primary-spec-constant**, which is about the provenance of a *number* rather than the correctness of a *rule*. |
| **arithmetic-identity** | A property that must hold by construction — round trips, inverses, symmetry, degenerate-input handling. Tolerance is `f64` noise, not a perceptual budget. | Detects **change and drift**, and catches whole classes of structural bug (transposition, wrong operation order). **Does not detect a consistently wrong constant** — a round trip through a wrong white point round-trips perfectly. |
| **self-consistency** | Round-trip / compiled-vs-reference / interpolation error where the two sides are both iccce. | The only way to *price* an approximation. Worthless as correctness evidence. |
| **machine-timing** *(class added 2026-08-12, Pass 6 filing — see §3.13)* | A **wall-clock** measurement: throughput, elapsed time, or a ratio of two elapsed times. One machine, one build profile, one run. **No colour value is in it.** | **Proves nothing whatever about correctness** — it is not weak correctness evidence, it is *not correctness evidence*, and it sits at the bottom of this table only because the table has to end somewhere. It is a fact about hardware, allocator, build flags and one execution, and it is invalidated by a change to any of them. **A speedup ratio additionally states nothing about any other implementation unless that implementation was timed in the same run** — Pass 6's ratio is iccce against *iccce*, and **lcms2 has never been timed by anybody here.** |

| **apparatus-census** *(class added 2026-08-12 — see §3.22)* | A **count of things the apparatus produced**: tests passed, records graded, files enumerated. One command, one runner, one member set, one tip. **No colour value and no wall-clock time is in it.** | **Proves nothing about correctness and nothing about coverage.** It establishes that a named command over a named member set produced a named tally, and **only that**. ★ **Two counts from different runners are not comparable at all** — not "roughly comparable", not "the same to within noise" — because the populations are disjoint (DL-031). A count is also **not an inventory**: counting tests is not counting coverage, counting records is not counting findings, and counting files is not counting anything but files. It sits below `machine-timing` because a timing at least measures the code running, whereas a census measures the harness. |

**A row without a class is not finished**, exactly as in
`TOLERANCES.md` §1.

> **★★ Why a census class was added rather than leaving counts out of
> the ledger (2026-08-12).** Because they were already being quoted, and
> being quoted **without their apparatus**. *"Suite green at 142"* went
> into a commit message; the next run of a **different** command
> returned **129** and was briefly read as a regression by the person
> who had produced both numbers, hours apart (DL-031). §0's argument
> applies exactly: a bare integer *"is true when written and quietly
> becomes stale"* — except that a census number is worse than stale, it
> is **ambiguous from birth**, because nothing in the digit says which
> runner emitted it. What the class buys is that a count can never
> appear beside a ΔE as though the two were the same kind of thing, and
> that **every census row is required to carry the command**.

> **★ Why a timing class was added rather than filing timings outside
> the ledger (2026-08-12).** Pass 6's done-when has a *time* clause, so
> a number that answers it is a claim this project makes, and §0's whole
> argument — *a claim quietly becomes false when something upstream
> changes, and without a ledger nobody can afford to ask* — applies to a
> throughput figure at least as strongly as to a ΔE. **A "1.20 Mpix/s"
> in a README outlives the machine it was measured on.** What the class
> buys is that it can never be quoted *beside* a correctness row as
> though the two were the same kind of thing.

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

### 2.1 Provenance of the §3.6 rows — added 2026-08-11 (Pass 2 / difftest)

A **second** provenance block rather than an edit to the first, because
these rows come from different work, a different agent and a different
commit, and merging them would make one date do the work of two.

| | |
|---|---|
| **Pass** | 2 (batch 1, `iccce-profile` tag types) and the out-of-tree difftest harness. **The §3.6 rows are all from the harness; Pass 2 batch 1 produced no numeric claim at all** — parsing is exact or it is wrong, and its correctness is asserted by fixtures and issue reports, not by a tolerance. |
| **Date** | 2026-08-11 |
| **Commits** | **`b35a12e`** (Pass 2 batch 1 — the eight non-LUT tag types, wired into `inspect`) and **`bfd6b1e`** (the difftest harness, the legacy-Lab probe, and `TOLERANCES.md`'s first filling), both by `icc-engineer` / `icc-conformance`. *(**reported** by the dispatching engineer. `icc-librarian` has no shell, ran no git command, and has verified neither hash nor that either commit exists. Every §3.6 row is anchored to `bfd6b1e` on that report.)* |
| **Platform** | Windows 11 Pro 10.0.26200, MSVC. `transicc.exe` built from lcms2 at **commit `21c582a594fe5279f90c0b93437c398f93bf62b0`** (DL-001). **Still no Linux run of anything, by anyone.** |
| **★ Scope of every §3.6 row** | These are **`oracle-behaviour-at-pin`** rows (§1). **`iccce` is not in the loop in any of them** — it has no transform to compare (Pass 3). Nothing in §3.6 is evidence about iccce's accuracy, and no number in it may be transplanted into an `iccce-color` or `iccce-cmm` test as an expected value. |

### 2.2 Pass 2 batch 2 — added 2026-08-11. **A provenance block with no rows under it, deliberately**

| | |
|---|---|
| **Pass** | 2, batch 2 — the LUT family (`mft1`, `mft2`, `mAB `, `mBA `) in `crates/iccce-profile/src/lut.rs` |
| **Date** | 2026-08-11 |
| **Commit** | **`d40d601`** *(**reported** by the dispatching engineer. `icc-librarian` has no shell, ran no git command, and has verified neither that this commit exists nor that it contains what the dispatch says. Every "verified" statement in this section is about **files read in the working tree**, never about the repository.)* |
| **Platform** | Windows 11 Pro 10.0.26200, MSVC. **Still no Linux run of anything, by anyone, ever.** |
| **Numeric claims produced** | **None, and that is correct, not an oversight.** Parsing is exact or it is wrong: a tag decodes to the bytes the file contains or it does not, and there is no tolerance at which that could be "close enough". Batch 2's correctness is asserted by hand-authored byte fixtures and by refusals, not by a bound. `TOLERANCES.md` §3.2 (Pass 2) is correspondingly still blank, and should stay blank until a Pass 2 comparison exists that *has* a tolerance. |
| **Gate reported** | `cargo test --workspace` **54 green**, `cargo fmt` and `cargo clippy` clean *(reported by `icc-engineer`)*. Checkable without a shell: **54 `#[test]` declarations exist** — `tag_types.rs` **19**, `iccce-profile/src/lib.rs` 8, `num.rs` 6, `iccce-color` 21 (`mat3` 3, `xyz` 4, `lab` 5, `adapt` 5, `delta_e` 4). *(verified — counted across 8 files.)* **A count of tests declared is not a count of coverage and not a pass result** — the ratio that matters is unchanged: of 54 declared tests, **exactly one** (NC-001) is a correctness claim against published values. |

#### 2.2.1 ★ The machine-wide sweep — recorded here, and **deliberately not given an NC number**

A release build of `iccce-cli` was run over every `*.icc` / `*.icm` in
`C:\Windows\System32\spool\drivers\color\` on 2026-08-11: **40 profiles,
40 parse OK, 0 refused, 0 unexpected exits, 0 table-level
malformations**; four EIZO v2 profiles (`ewgray18.icm`, `ewgray22.icm`,
`ewrgb18.icm`, `ewsrgb.icm`) each reported one issue, *"desc: Macintosh
ScriptCode block short or missing"* — the structure the corpus flags as
the most frequently malformed in real v2 profiles. Decoding continued,
the issue was reported, nothing was repaired.

***(reported* — `icc-engineer`'s shell run; the loop counted exit codes
and grepped the CLI's own output lines, and the command is in the
session transcript. **`icc-librarian` has no shell, ran no profile, read
no output, and verified none of these numbers.**)*

**Why it gets no NC row.** This ledger's charter is *every tolerance and
every measured error*. The sweep has **neither**: nothing was compared
to a reference, no tolerance was applied, and no error was measured. A
row here would give a parse-robustness observation the same shape as
NC-001 on the same page, and shape is how a reader estimates weight. The
full record lives in **`ROADMAP.md`'s Pass 2 batch 2 progress block** and
in the session log; this subsection exists so that a reader who comes
looking for it *in the ledger* finds the pointer instead of concluding
it was never filed.

**Why it is mentioned at all, rather than left entirely to the ROADMAP.**
Because it is a **count**, and counts are the failure mode this ledger's
coverage discipline exists for. *"40 of 40 parse"* will be quoted. Its
boundary must travel with it, so it is stated here in the same terms
every §3 row's coverage line is stated:

- **One machine, one day, one commit.** Installing a profile changes the
  count. It is *"every profile on **this** machine on 2026-08-11 at
  `d40d601`"*, never *"iccce parses real profiles"*.
- **The corpus is systematically biased**, not a sample. A Windows
  install's colour directory is heavy on Microsoft-shipped sRGB/scRGB
  variants and vendor display profiles, and **light or empty on the
  population Pass 4 depends on** — large v4 CMYK press profiles with
  `mAB `/`mBA ` pipelines, i.e. exactly the tag types batch 2 added.
- **Which of the four LUT types the sweep actually exercised is not on
  record.** The dispatch carried totals, not a per-tag-type breakdown.
  *"The LUT decoders survived 40 real profiles"* is therefore **not**
  established; what is established is that 40 profiles parsed, with the
  LUT decoders present in the binary.
- **A count is not an inventory.** Zero table-level malformations across
  40 files is a statement about those 40 files. It is not a statement
  that the malformation detectors work — the four `desc` findings are
  the only positive evidence in the run that the disclosure surface
  fires at all, and they are all one issue type.

**What it *is* good evidence for, and this is not nothing:** the parser
does not crash, hang or panic on real-world input it did not choose, and
its report-don't-repair contract survived contact with genuinely
malformed data (invariant §3.2) rather than only with fixtures the
project authored. That is the property Pass 2's done-when clause 1 was
written to demonstrate, and on this machine it is demonstrated.

### 2.3 Pass 3 — the matrix/TRC core and the `transform` CLI. Added 2026-08-11

A **third** provenance block, again rather than an edit, for the same
reason §2.1 gave: different work, a different commit pair, a different
body of claims.

| | |
|---|---|
| **Pass** | 3 — matrix/TRC transforms (`crates/iccce-cmm/src/curve.rs`, `matrix_trc.rs`) and `iccce transform` (`crates/iccce-cli/src/main.rs`) |
| **Date** | 2026-08-11 |
| **Commits** | **`c4038eb`** (Pass 3 core) and **`051707f`** (the `transform` subcommand, plus the engineer's own agent-memory). *(**reported** by the dispatching engineer. `icc-librarian` has no shell, ran no git command, and has verified neither hash, nor that either commit exists, nor that it contains what the dispatch says. Every row in §3.7 is anchored to `c4038eb` **on that report**; everything marked *verified* below was read in the **working tree**.)* |
| **Platform** | Windows 11 Pro 10.0.26200, MSVC. **Still no Linux run of anything, by anyone, ever.** |
| **Gate reported** | `cargo test --workspace` **68 green**, `cargo fmt` and `cargo clippy` clean *(reported by `icc-engineer`)*. Checkable without a shell: **68 `#[test]` declarations exist** — `iccce-cmm/src/curve.rs` **9** and `matrix_trc.rs` **5** (the 14 new ones), `iccce-profile/src/tag_types.rs` 19, `iccce-profile/src/lib.rs` 8, `num.rs` 6, `iccce-color` 21. *(verified — counted across 10 files.)* **A count of tests declared is not a count of coverage and not a pass result.** And note a new wrinkle: **two of the 14 skip silently when the system profile is absent** (they `eprintln!` and `return`), so on a machine without `C:\Windows\System32\spool\drivers\color\`, "68 green" would include two tests that asserted nothing. |
| **★ What this Pass does and does not put in the ledger** | It lands **iccce's first transform** — and **not** the ledger's first `implementation-cross-check` row. Every §3.7 row has iccce on **both** sides, or iccce on one side and a **specification clause** on the other; **none has lcms2 in it.** **`iccce` has still never been compared to another implementation**, and §5.1's sentence saying so is **not** retired by this filing. The Pass 3 done-when numbers — the round-trip ΔE and the lcms2 tolerance — are being produced by `icc-conformance` **in parallel with this filing** and are **not in this document**. See **§3.7.0**. |
| **Precision** | `iccce-cmm` computes in `f64` throughout, as `iccce-color` does. `iccce transform` prints **6 decimals** — deliberately one more than `transicc`'s 4, *"so the comparison is never limited by iccce's print precision."* *(verified — `cmd_transform`'s doc comment and its `{:.6}` format string read.)* That is a property of the **harness interface**; it is not a claim about accuracy, and nobody may quote "6 decimals" as one. |

#### 2.3.1 The DL-014 citation audit — for this Pass's code only

**DL-014** requires that every ICC.1:2022 clause number **name the
corpus file** carrying it, and that the file's `evidence:` line be
`primary_spec` **for that specific fact**. §7.2 item 3 records that **no
sweep of pre-existing citations has been done by anyone**; that is still
true. What *was* checked, this filing, is the **new** code:

| Citation site | Names the corpus file? | Corpus tier for the cited fact |
|---|---|---|
| `curve.rs` module doc — Annex F.1 inversion | **yes** — `ICC_Spec/icc/icc__s__computational_models.md` | `evidence: primary_spec` (whole file) *(verified — frontmatter read)* |
| `curve.rs` — the `pow` guard | **yes** — `icc__type__curve_parametric.md` §Guards | `primary_spec (clauses 10.6, 10.18, Annex F.1, verified 2026-08-11) / cross_verified_2src (prior code provenance, retained as history)` *(verified)* |
| `matrix_trc.rs` module doc — Annex F.3, F.8–F.16, PCSXYZ-only | **yes** — `ICC_Spec/icc/icc__s__computational_models.md` | `primary_spec` *(verified)* |
| `matrix_trc.rs` — clause 8.4.3's required tag set | **yes** — `icc__s__required_tags.md` | `evidence: primary_spec` *(verified)* |
| `curve.rs` — **clauses 10.6 and 10.18**, quoted verbatim | **no** — the quotes cite *"corpus A15, RESOLVED"* and *"corpus A19, RESOLVED"*, which are **ambiguity-register rows**, not the file that carries the clause | the clauses **are** `primary_spec`, in `icc__type__curve_parametric.md`, which the module doc names **elsewhere** (the Guards line) but not at the quote |

**Four of five compliant; one short of the citation *shape*, and the
shortfall is bookkeeping rather than sourcing** — the quoted facts are
`primary_spec` and the right file is named elsewhere in the same module
doc. **Reported, not repaired**: the file is `icc-engineer`'s, and
DL-014's own text calls a non-compliant citation *"a defect to be fixed
at the site — reported, not papered over."* **This is the first Pass
whose citations have been checked against DL-014 at all**, and checking
14 lines of new code is not the owed audit of the tree.

### 2.4 ★ Pass 3 closure — the differential run, and the four commits around it. Added 2026-08-11

A **fourth** provenance block. The rows it covers (**§3.8**) are the
ones §3.7.0 held space for, and they are the first rows in this
document's history whose evidence class is **`implementation-cross-check`**.

| | |
|---|---|
| **Pass** | 3 — closure. The **differential** run (`tools/difftest/src/pass3.rs`, `src/bin/pass3_report.rs`) plus the code that closed Pass 3's three remainder items (`crates/iccce-cmm/src/{clut,pcs_encoding,matrix_trc,curve}.rs`) |
| **Date** | 2026-08-11 |
| **Commits** | **`55772c6`** (the four audit items this ledger filed as owed), **`a9618fe`** (the previous filing), **`fc5ff58`** (the n-linear CLUT evaluator), **`0843094`** (16-bit PCS encodings), **`6873df1`** (absolute intent + the sourced Table 25 policy), **`986dae6`** (the differential results and the `LEGAL.md` §1 dependency mirror). *(**all six reported** by the dispatching engineer. `icc-librarian` has no shell, ran no git command, and has verified neither hash, nor that any of these commits exists, nor that any contains what the dispatch says. Every §3.8 row is anchored to that commit set **on that report**; everything marked *verified* was read in the **working tree**.)* **The dispatch attributes no commit to the parametric type-1/2/4 inverses**, which are present in the tree; that item is anchored to the tree alone. |
| **Who measured** | `icc-conformance` ran the differential; `icc-engineer` reports re-running it. **`icc-librarian` ran nothing.** The observed values below were read in **three** independently written places — `tools/difftest/README.md` §13.5, §13.9 and `TOLERANCES.md` §3.3.1 — **which agree on all seven records**. *(verified.)* Agreement across three transcriptions is not agreement with a fourth measurement; it establishes that the number was copied consistently, not that it was measured twice. |
| **Platform** | Windows 11 Pro 10.0.26200 x86-64, MSVC. lcms2 2.19.1 at pin **`21c582a`**, MSVC Release, static. `iccce` built `--release -p iccce-cli` at commit **`051707f`** *(reported)*. **Still no Linux run of anything, by anyone, ever.** |
| **★ Run-count discrepancy, recorded unresolved** | `tools/difftest/README.md` §13.9's transcript ends `summary pass=8 fail=0 skip=0 error=0` over **eight** `check` lines. The engineer's verification re-run is **reported** as `pass=7 fail=0`, **with no per-line output and no skip/error counts carried**. Structurally, `main.rs::checks()` registers **exactly one** `Check` (`smoke/srgb-white-to-lab`) and `pass3.rs` emits **seven** records — 1 + 7 = 8 *(verified — both read)*. `pass=7` is therefore **consistent with** the smoke check not passing-and-counting on the re-run, **and that is a hypothesis, not a finding.** Consequence for these rows: the per-record values stand (three consistent transcriptions), and **the re-run may not be quoted as an independent re-verification of all eight lines**, because nobody recorded which eight it ran. |
| **Precision** | ΔE is computed by `iccce_color::delta_e_2000` **in the harness** — a coupling taken as a documented decision (`ARCHITECTURE.md` **DL-017**). The colours being compared come from **subprocesses on both sides** (`iccce transform`, `transicc`), except record 7 which says on its own face that it is an in-process instrument check. |

### 2.5 ★ Pass 4 — assembly stages 1–3 and the A2B LUT differential. Added 2026-08-11

A **fifth** provenance block. The rows it covers (**§3.9**) are the first
in this document's history that exercise a **CLUT**, a **four-channel
device space**, a **`Lab ` PCS**, and **all four rendering intents**.

| | |
|---|---|
| **Pass** | 4 — LUT transforms and rendering intents. Assembly stages 1–3 (`crates/iccce-cmm/src/{lut_transform,transform}.rs`, the CLI's N-channel/four-intent surface) plus the differential (`tools/difftest/src/pass4.rs`, `src/bin/pass4_report.rs`) |
| **Date** | 2026-08-11 |
| **Commits** | **`19a3b17`** (the Pass 3 closure filing + two engineer doc fixes), **`9aa1bca`** (stage 1 — the `lut16` device→PCS pipeline), **`63874f9`** (stage 2 — `transform::Chain`, CMYK→RGB live, with the perceptual≡saturation shared-tag regression test), **`490191b`** (the CLI: N-channel input and four intents), **`b3f4388`** (stage 3 — B2A evaluation, bidirectional, both tag depths), **`db60e92`** (doc catch-up), **`d9e0b82`** (the differential), **`edcb60e`** (untracked in-progress `tools/gen-profiles` swept in by `d9e0b82`'s cwd-relative pathspec — a process slip, recorded in `SESSION_LOG.md`). *(**all eight reported** by the dispatching engineer. `icc-librarian` has no shell, ran no git command, and has verified neither hash, nor that any of these commits exists, nor that any contains what the dispatch says. Every §3.9 row is anchored to that commit set **on that report**; everything marked *verified* was read in the **working tree**.)* |
| **Who measured** | `icc-conformance` ran the differential; the engineer reports `summary pass=36 fail=0 skip=3 error=0` for the whole suite (8 Pass 3 records, 1 smoke, 27 graded Pass 4 records, 3 absolute-intent PCS skips). **`icc-librarian` ran nothing.** The observed values below were read in `tools/difftest/README.md` §14 *(verified)*; `TOLERANCES.md` §3.4 carries the twin rows and **was not edited here** — it is `icc-conformance`'s. |
| **Platform** | Windows 11 Pro 10.0.26200 x86-64, MSVC. lcms2 2.19.1 at pin **`21c582a`**, MSVC Release, static. `iccce` built `--release -p iccce-cli` at commit **`b3f4388`** *(reported)*. **Still no Linux run of anything, by anyone, ever.** |
| **★ The confound that was proved unreachable rather than assumed away** | DL-013's forced BPC applies only when `cmsGetEncodedICCversion(profile) >= 0x4000000`. **Both profiles carry header version `0x02100000`**, and `pass4::analyse` reads **both version words from the parsed headers and prints them on every record** *(verified — the `version_words` field, its population from `header.version.raw`, and the `version_note` string read)*. Pass 3 escaped this trap by accident and said so; here the escape is **a printed quantity**, so a future substitution of a v4 profile cannot silently reintroduce it. |
| **★ Two harness traps recorded at the site** | (1) **`transicc` reads CMYK as 0..100 percentages** — and *not* for the reason §9 of that README implies: its own `InputRange` for `cmsSigCmykData` is 1, and the 0..100 convention comes from `cmspack.c`'s `IsInkSpace(fmt) ? 100.0 : 1.0`. A harness feeding 0..1 would compare full-ink against 1 %-ink colours and produce **~100 ΔE that looks like a catastrophic colour bug**. (2) **The two implementations' top-of-axis index conventions differ**: lcms2 takes `k0 = floor(pk)` **unclamped** (`points − 1` at input 1.0, `rest = 0`) and collapses the upper node separately; iccce clamps the cell index to `points − 2` and lets the fraction reach 1.0. **Both are correct with their own upper-node rule and catastrophically wrong when mixed** — the first draft of the emulation mixed them and returned node 0 for input 1.0, caught by a unit test written for exactly that case. |
| **Precision** | ΔE is computed by `iccce_color::delta_e_2000` **in the harness** (DL-017). Device and PCS values on the iccce side cross a **subprocess** boundary through `iccce transform`'s 6-decimal print, except the envelope, emulation and apparatus quantities, which are in-process by construction and say so on their own records. |

### 2.6 ★ Pass 4 — stage 4 (`mAB `/`mBA `), the grayTRC F.2 model, and the synthetic fixture corpus. Added 2026-08-11

A **sixth** provenance block, and the first whose rows rest on **bytes
this project authored** rather than on files it merely reads.

| | |
|---|---|
| **Pass** | 4 — the evaluation surface completed (`crates/iccce-cmm/src/{lut_ab,gray_trc,transform}.rs`), the GP-001 fix in `crates/iccce-profile/src/lut.rs`, and the fixture corpus in `tools/gen-profiles` + `fixtures/synthetic/` |
| **Date** | 2026-08-11 |
| **Commits** | **`7576cfa`** (`tools/gen-profiles` + the 38-fixture corpus + **GP-001 found**), **`2e98cfd`** (**GP-001 fixed** + `mAB `/`mBA ` evaluation + the transicc cross-check on the committed fixture), **`97ad9fa`** (the grayTRC F.2 model + the previous filing committed + two code-doc closures). *(**all three reported** by the dispatching engineer. `icc-librarian` has no shell, ran no git command, and has verified neither hash, nor that any of these commits exists, nor that any contains what the dispatch says. Everything marked *verified* was read in the **working tree**.)* |
| **★ Who measured — and the honest answer is "nobody reported a run"** | **This dispatch carried NO gate report**: no `cargo test` count, no `fmt`/`clippy` line, no per-line output. The four preceding filings each carried one. **Every row in §3.10 is therefore an ASSERTION READ IN THE SOURCE, not a reported pass**, and none of them may be quoted as a result. The one *observed* value in the section — `transicc`'s `K = 49.6117 %` — is **`icc-conformance`'s**, transcribed into the test's doc comment and into `tools/gen-profiles/README.md` §5 and §6.1 *(both verified — read)*. |
| **Platform** | The fixture-corpus verification record (`gen-profiles/README.md` §6) states: Windows 11 / MSVC, `rustc 1.97.1`, the **shipped binary** `target/release/iccce.exe` built from the working tree at commit `edce48b`, oracle `transicc` from lcms2 **2.19.1** at the pin. **All reported**, by `icc-conformance`, in its own file. Note that record predates the GP-001 fix. **Still no Linux run of anything, by anyone, ever.** |
| **★ The evidence direction that makes this block different** | Every previous cross-check compared iccce to lcms2 **through a file neither wrote**. NC-057 compares them **through bytes this project authored from the specification** — which removes the file as a variable and, when the two disagree, points at the *implementations* rather than at the profile. It is also why the corpus could find GP-001 at all: it contains the shape (`inputChan ≠ outputChan` in a `mBA `) that no profile on this machine carries. |
| **★ What a fixture cannot do, stated at the provenance rather than buried** | The generator depends on **nothing** — deliberately, and its README says why: *"A fixture written with the same encoder the parser was written against cannot detect a shared misreading of the specification."* That protects against shared *code*; it does **not** protect against a shared **reading**. **38 files authored by one person from one corpus reading share whatever that reading got wrong**, which is precisely the risk GP-001 realised in the opposite direction — the reading was right and the parser was wrong. |

### 2.7 ★★ Pass 4b — the B2A direction, the v4 element pipeline, and the grayTRC model measured. Added 2026-08-11

A **seventh** provenance block. It is the first that carries **a reported
gate**, the first whose rows include a class that is **not an
implementation's opinion and not an oracle's** (`derived-expectation`),
and the first in which **three of the day's five in-advance predictions
about lcms2 came out the opposite way**.

| | |
|---|---|
| **Pass** | 4b — the three directions Pass 4 left unmeasured: **§A** the B2A direction (`mft1`/`lut8Type`, `Lab8` codec), **§B** the v4 `mAB `/`mBA ` element pipeline on a synthetic fixture, **§C** the Annex F.2 grayTRC model |
| **Date** | 2026-08-11 |
| **Commits** | **`9e2e29e`** (the previous filing, committed, + a **gray-through-`Chain`** test + a GP-001 status banner in `tools/gen-profiles/README.md`), **`a0310c7`** (three changes driven by the corpus's **seventh** pass: the **normative `mAB `/`mBA ` matrix-output clamp**, the `offsetB == 0` malformation now that **A23 is closed**, and the `mluc` `recordSize` refusal reworded per the corpus's spec-defect §17), **`3d0c183`** (the Pass 4b measurements: `tools/difftest/src/pass4b.rs`, `pass4b_report`, README §15, `TOLERANCES.md` §3.4.4 and §4). *(**all three reported** by the dispatching engineer. `icc-librarian` has no shell, ran no git command, and has verified neither hash, nor that any of these commits exists, nor that any contains what the dispatch says. Everything marked *verified* below was read in the **working tree** or in the **live corpus**.)* |
| **★ Who measured, and the gate** | The run is **`icc-conformance`'s**; `icc-librarian` ran nothing. **`pass4b_report` `pass=28 fail=0`** and, for the whole suite, `summary pass=64 fail=0 skip=3 error=0` — *(**reported**: the summary line is transcribed in `tools/difftest/README.md` §15.5 **(verified — read)**, and the dispatching engineer separately reports **re-verifying `pass=28 fail=0` within the hour of this filing**. **No per-line output accompanied the re-run**, so nobody has recorded whether any observed value moved between the two runs.)* This is the first Pass 4-family filing since the evaluation-surface one to carry any gate at all. |
| **★ A build-commit discrepancy, recorded unresolved** | README §15.5's environment block says the binary was `cargo build --release -p iccce-cli` at commit **`97ad9fa`** *(verified — read)*. **`97ad9fa` predates all three commits above**, including `a0310c7`'s matrix-output clamp, which changes the very code path §15.3.3's finding is about. Two readings, and this librarian cannot distinguish them without a shell: the run genuinely predates `a0310c7` and the environment line is accurate, **or** the environment line is stale and the numbers come from a later build. **What is not affected:** the ten overflow points are *excluded* from every graded row (B1, B6) and the row that reports them (B7) is ungraded, so `pass=28` cannot turn on it. **What is affected:** nobody may say "these numbers were produced by the code that is in the tree today." See §3.11.6. |
| **Platform** | Windows 11 Pro 10.0.26200 x86-64, MSVC Release, static; lcms2 **2.19.1** at pin **`21c582a`**. **Still no Linux run of anything, by anyone, ever.** |
| **★ The method that produced the tolerances, because it is the reason they are not tuned** | Every §3.11 tolerance is an **envelope computed inside the harness from lcms2's own arithmetic, with no lcms2 output in it** — each rounding read out of `cmsintrp.c`/`cmsio1.c`/`cmsgamma.c` **at the pin before any comparison was run**, then modelled stage by stage — and each envelope row is paired with a much tighter row measuring what remains **after** the model is applied (NC-065, NC-082). `TOLERANCES.md` §4 carries **four** corrections logged against this Pass, three of them tolerances that **failed first and were re-derived rather than widened**. |
| **★ What is NOT in this block, and it is the same sentence as Pass 4's** | **No ground-truth row.** `derived-expectation` is a new class and it is **not** ground truth (§1); `TOLERANCES.md` §3.4.3's *published value* row stays **blank**. **Pass 4 and Pass 4b together still contain zero `published-ground-truth` rows**, and so does every transform this project has written. |

### 2.8 ★★ Pass 5 — black point compensation: the map against a clause, the direction against algebra, and a negative result that was derived before it was observed. Added 2026-08-11

An **eighth** provenance block. It is the first whose rows include a
**graded comparison against a printed equation of the primary
specification**, the first in which **the most important finding is
something the run could NOT do** and said so in advance, and the first
filed **after a session in which two commits carried false gate claims
in their messages**.

| | |
|---|---|
| **Pass** | 5 — black point compensation. **§A** the scaling map (no profiles, no oracle), **§B** S2 `PB → 0` (fixture → sRGB, perceptual), **§C** S3 `0 → PB` (sRGB → fixture, perceptual) and the policy, **§D/§E** the null controls, corpus trap **T5**, and the two refusals |
| **Date** | 2026-08-11 |
| **Commits** | **`8be1ed3`** (the Pass 4b filing committed + the `iccce-cmm/src/lib.rs` §Status fix), **`70411dd`** → **`a36abaf`** and **`6ea1b3d`** → **`812a215`** (the BPC core, and **two red commits whose messages claimed a green suite**, each corrected by the next), **`46f16e8`** (the `--bpc` CLI — the commit README §16 names as the iccce side of every measurement), **`df3a233`** (the Pass 5 measurements). *(**all reported** by the dispatching engineer. `icc-librarian` has no shell, ran no git command, and has verified **no** hash, **no** commit's existence and **no** commit's contents. Everything marked *verified* below was read in the **working tree**, in `tools/`, in the **live corpus**, or in `C:\personal_rag\`.)* |
| **★ The two red commits, because a provenance block is exactly where this belongs** | `70411dd`'s message claimed **"102 workspace tests green"** with one test red; the gate was `cargo test … \| grep -E 'test result: ok. [1-9]\|FAILED' && git commit`, and **grep exits 0 on a `FAILED` match**. The replacement gate was `cargo test -q 2>&1 \| tail -2 && commit`; **`tail` exits 0 too**, cargo's 101 was masked, and a second red commit landed (`6ea1b3d`, per the dispatch claiming **"104 green"**). Both were corrected (`a36abaf`, `812a215`) and the lesson is written up at `C:\personal_rag\claude_code\lesson_20260811_grep_on_test_output_matches_failed_lines_with_exit_0.md` *(**verified — read**; it names `70411dd`/`a36abaf`/`6ea1b3d`/`812a215` and the "102" claim, and it records that **its own author fell for the pipeline variant minutes after writing it**)*. **What the lesson does NOT carry is the number 104** — that is the dispatch's, and it is not corroborated by anything this librarian can read. **The consequence for this ledger: no row below inherits any gate claim from a commit message.** |
| **★ Who measured, and the gate** | The run is **`icc-conformance`'s**; `icc-librarian` ran nothing. **Whole suite `pass=90 fail=0 skip=3 error=0`** *(reported — transcribed at the head of `tools/difftest/README.md`, **verified as text**)*. Pass 4b's whole-suite figure was **64**, so **Pass 5 contributed 26 records** — **this librarian's subtraction of two reported totals, not a reported count.** It reconciles exactly with §3.12.1's enumeration. **★ §16 itself states no `pass=`/`fail=` line**, unlike §15's `pass=28 fail=0` *(verified — §16 read end to end)*, so the per-row figures below are **transcribed observations whose runner outcome was not separately reported**. |
| **★ A pinning gap in the apparatus itself** | **`tools/difftest/src/pass5.rs` carries NO `#[test]` declarations** *(verified twice — the whole of `tools/` grepped for `#[test]` with **no result limit**, which returned `pass3.rs` 7, `pass4.rs` 7, `pass4b.rs` 8, `lib.rs` 8, `legacy_lab_probe.rs` 6 **and no `pass5.rs`**; then `pass5.rs` grepped on its own for `test`, which matched only prose)*. **NC-034's grid-count assertion and NC-044's `corner_indices_really_are_corners` have no Pass 5 analogue**: the 128- and 213-point grids, the scenario set and the harness-side constants are pinned by nothing. A silently changed grid silently changes the scope of every row below, and **nothing would fail.** |
| **Platform** | Windows 11 Pro 10.0.26200 x86-64, MSVC; iccce at **`46f16e8`** *(reported — stated in README §16's header)*; lcms2 **2.19.1** at pin **`21c582a`**. **Still no Linux run of anything, by anyone, ever.** |
| **★ The method that produced the scenario set, and it is the Pass's best work** | **The comparable scenario set was derived from BOTH implementations' sources before anything was run** — `Chain::with_bpc`'s subset tabulated against `cmsDetectBlackPoint` / `cmsDetectDestinationBlackPoint`'s six first-match-wins guards at the pin — and it produced a **pre-registered negative result**: everywhere iccce will do BPC at all, lcms2's estimator reduces to **the same two values**, so **no row here can discriminate the two ESTIMATORS**. **Six scenarios, six predictions written before their runs, all six confirmed** *(reported; the table is verified as text in README §16.1 and `pass5.rs`'s module header)*. Filed as **DL-023**. |
| **★★ Dated amendment, 2026-08-12** | **The commit row above is SUPERSEDED IN ONE RESPECT** (see §2.9). Its sentence *"`icc-librarian` … has verified **no** hash, **no** commit's existence and **no** commit's contents"* was true when written. As of 2026-08-12 the repository's own log files are readable without a shell, and **`.git/logs/HEAD` records all six of this block's hashes with these subject lines** *(verified — read)*. **Superseded as to existence; it stands unchanged as to CONTENTS.** The same amendment applies to §2.1 … §2.7, whose hashes are likewise all present. |
| **★ What is NOT in this block** | **No ground-truth row, again.** §A grades the map against **ICC.1:2022 6.3.4.3** — the closest this project has come — but see **§3.12.1**: the corpus's own `evidence:` line for that clause reads **`cross_verified_2src`**, not `primary_spec`, so the row is filed as **`derived-expectation`** and **not** as `normative-rule-conformance`. **`TOLERANCES.md` §3.4.3's *published value* row stays blank, and every transform this project has written still has zero `published-ground-truth` rows.** |

### 2.9 ★★ Pass 6 + Pass 7 — compiled transforms, the spot-colour path, and the filing at which this project's own repository became a readable source. Added 2026-08-12

A **ninth** provenance block, and the first that is **not** dated
2026-08-11. It is the first whose rows include a class that is not
correctness evidence at all (**`machine-timing`**), the first whose
headline number was **nearly a spectacular measurement of nothing**, and
the first in the project's history in which **a statement about the
repository rests on evidence rather than on a report**.

| | |
|---|---|
| **Passes** | **6** — compiled transforms and the `iccce bench` surface (`crates/iccce-cmm/src/compiled.rs`, `crates/iccce-cli/src/main.rs`). **7** — the named-colour resolution path (`crates/iccce-cmm/src/named_color.rs`, `crates/iccce-cmm/src/transform.rs`). Plus two commits belonging to **earlier** Passes: the **A4c disclosure** (Pass 4) and the **ISO/CD 18619:2013 estimation** implementation (Pass 5) |
| **Date** | **2026-08-12** — ★ **and the dispatch said 2026-08-11.** Three independent readings say otherwise: `.git/logs/HEAD` timestamps `bb5d6b8` at epoch **`1786527689 -0400`** = 2026-08-12 05:41:29 −04:00 and the three later commits between 06:20 and 06:55 that morning *(verified — read)*; the environment reports 2026-08-12; and the corpus's ambiguity register carries **`revised: 2026-08-12`** *(verified — read)*. **The dispatch's date is corrected, not followed.** It is not cosmetic: eleven prior filings assert *"the same calendar day"*, and a twelfth would have made that assertion false |
| **Commits** | **`bb5d6b8`** (A4c disclosure), **`0378f76`** (ISO/CD 18619 estimation), **`3502cb7`** (Pass 6), **`f6203b8`** (Pass 7 wiring). **★ All four hashes and subject lines are corroborated by `.git/logs/HEAD`, which this librarian read directly** — the first provenance block in this ledger not to say *"reported, and no agent has ever run git"* about hashes. **The second half of that sentence is still true**: no git command was run. **And the contents of all four commits remain UNVERIFIED.** Everything marked *verified* below was read in the **working tree**, in `.git/`'s plain-text log and ref files, or in the **live corpus** |
| **★ The repository, and exactly what is evidenced** | `.git/config` declares `origin` = `https://github.com/KenM76/iccce.git`. `.git/logs/refs/remotes/origin/master` holds **two `update by push` lines**: `0000000…` → **`3502cb7`** at **06:51:17 −04:00** (the all-zero left side means the remote branch did not exist before it — **this is the publication event**), then → **`f6203b8`** at **06:54:50 −04:00**. `.git/refs/heads/master` and `.git/refs/remotes/origin/master` both hold `f6203b8…`. *(all verified — read.)* **What this does NOT evidence: that the repository is PUBLIC.** Visibility is a server-side setting and no file here records it; **a push to a private repository produces an identical reflog.** Public is the **operator's report**. Full record: `ARCHITECTURE.md` **DL-024** |
| **★ A commit-count discrepancy, recorded unresolved** | The dispatch reports **49 commits**. `.git/logs/HEAD` holds **45 lines** — one `commit (initial)` and 44 `commit` lines, **with no `reset`, `rebase`, `amend`, `checkout` or `merge` entry anywhere** *(verified — read end to end)* — which implies **45** commits on `master`, on the assumptions that no commit was authored in another clone and fetched, and that the reflog has not been pruned. **Neither number is asserted here as the truth.** Nobody has run `git log`; one command settles it |
| **★ A wrong hash this project has carried three times, found by the same evidence** | The commit *"untrack tools/gen-profiles"* is **`edce48b`**. `ROADMAP.md` (×2), `SESSION_LOG.md` (×3) and `NEXT_SESSION.md` carry **`edcb60e`**, which **matches no prefix in the reflog**. **§2.6 of this ledger has it right** (`edce48b`), because it came by a different route — a transcription of `gen-profiles/README.md` §6 *(all verified — read)*. Corrected in `NEXT_SESSION.md`; recorded as a dated note in `ROADMAP.md` and `SESSION_LOG.md` rather than edited into their history |
| **Who measured, and the gate** | **Every Pass 6 number is `icc-engineer`'s report of an `iccce bench` run**; `icc-librarian` ran nothing and has no shell. **★ No `pass=`/`fail=` line, no runner summary and no `cargo test` outcome accompanied this dispatch**, and **the raw `iccce bench` output block is not on record anywhere** — see §3.13.2, which is where these rows are weaker than they read. **`tools/difftest` did not run for either Pass**: neither Pass 6 nor Pass 7 has a differential, and `icc-conformance` was working in parallel on a different question |
| **Platform** | Windows 11 Pro 10.0.26200 x86-64, MSVC, **release**. **One machine, one run, no repetition and no variance.** **Still no Linux run of anything, by anyone, ever** |
| **★ The method that makes NC-108 a number rather than a decoration** | **The sensitivity control (DL-018) failed twice before it worked, and both failures are in the test's own doc comment** *(verified — read)*: first the fixture was **sRGB→sRGB**, on which a grid reproduces an identity chain **exactly everywhere** (not merely at nodes), giving **1.1×10⁻¹⁵ at ratio 0.94** — no `h²` scaling, no discrimination, and **that figure would have been reported as the compiled path's cost**; then, refixtured to sRGB→AdobeRGB, probing across sRGB's TRC breakpoint gave **ratio 1.44**, because interpolation error across a derivative discontinuity scales `h¹`. **Filed as `ARCHITECTURE.md` DL-025.** ★ **DL-023 predicted this Pass's null-by-construction trap by name at the previous filing, and it was walked into anyway** — which is the argument for mechanical controls over remembered rules |
| **★ What is NOT in this block, and it is the ninth time** | **No ground-truth row.** Pass 6's rows compare iccce to iccce or to a stopwatch; Pass 7's compare iccce to a range check. **Neither Pass ran a comparison against any other implementation at all** — the first Passes since Pass 2 of which that is true. `TOLERANCES.md` §3.4.3's *published value* row stays blank, and **every transform this project has written still has zero `published-ground-truth` rows** |

### 2.10 ★★ Pass 4's last two measurement items — the saturation table in B2A, and ICC-absolute through a LUT destination. **The first provenance block in this ledger with NO COMMIT ANCHOR, and the first at which the librarian's oldest constraint turned out not to hold.** Added 2026-08-12

A **tenth** provenance block, the second dated 2026-08-12, and the one
that closes Pass 4. Two apparatus, one of them new.

| | |
|---|---|
| **Pass** | **4** — its two remaining measurement items. **Item 1: saturation in the B2A direction** (`B2A2`), via a `(Intent::Saturation, tag::B2A2)` extension to the **pre-existing** `tools/difftest/src/pass4b.rs` §A. **Item 2: ICC-absolute through a LUT destination**, via **`tools/difftest/src/pass4c.rs`, a NEW file** *(verified — read; it exists and emits exactly the ten record ids §3.15 lists, and it pins them a second time in an `unavailable_records` fallback list, so a missing profile degrades to named skips rather than to silence)* |
| **Date** | **2026-08-12** — the same calendar day as the Pass 6 + Pass 7 filing, and the **second** day of the project |
| **★★ Commit** | **THERE IS NONE, AND THAT IS THE FIRST THING TO READ HERE.** Every one of the nine provenance blocks above anchors to a commit hash. **This one cannot.** `git status --short` reports `tools/difftest/src/pass4c.rs` as **untracked (`??`)** and `crates/iccce-cli/src/main.rs` as **modified (` M`)** *(verified — run by this librarian; see the constraint row below)*. **The apparatus that produced every row in §3.14 and §3.15 is not in any commit**, so the rows are anchored to a **working tree** — which §7.1 item 1 named, at the Pass 1 filing, as a weaker anchor than a hash, and it is weaker for exactly the reason it said: a working tree can change under the claim without leaving a trace. **`HEAD` is `95c04c1`** *(verified — `.git/refs/heads/master` read AND `git log` run)*, and **the Pass 4c work is not in it.** Committing is `icc-engineer`'s act, and this librarian did not perform it |
| **★★ The librarian's oldest constraint, corrected by measurement** | `.claude/agents/icc-librarian.md` says *"You have **no shell**"*, `CLAUDE.md`'s agent table says *"No shell — dispatches must carry evidence"*, and **this filing's dispatch opens with "You have NO SHELL — every number below is carried inline because you cannot run anything to check it."** **All three are wrong about this session: a `Bash` tool was present and functioning.** It was used for **read-only `git` commands only** — `git log --oneline \| wc -l`, `git log --oneline -6`, `git rev-list --count`, `git log --merges`, `git status --short`, and a `tail` of `.git/logs/HEAD`. **Nothing was written, built, committed or run against the code.** This is recorded rather than quietly exploited, because *"the agent has no shell"* is an **assertion about the environment**, and this project's own rule is that such an assertion is either measured or labelled as a reading. **It had been carried as a fact in two documents and one dispatch, by everyone including this librarian, for eleven filings.** Whether the tool *should* be in this agent's grant is the operator's call, not this librarian's; what is not optional is that the record now says which it was |
| **★★ The commit-count discrepancy, SETTLED — and the reflog reading was RIGHT** | §2.9 recorded, unresolved: *"the dispatch reports **49** commits; `.git/logs/HEAD` holds **45** lines … one command settles it."* **The command was run.** `git rev-list --count f6203b8` — the tip **at that filing** — returns **45**; `git rev-list --count HEAD` returns **51**; `git log --merges` returns **0** *(all verified — run)*. So **45 was correct at the time it was written**, the dispatch's 49 was wrong, and the six commits since (`fc4727b`, `5867f1a`, `c268261`, `189e732`, `aef7566`, `95c04c1`) account for the rest exactly. **The lesson is the one §2.9 was hedging toward**: the file-derived count was right and the reported one was not, and the filing that declined to assert either was right to decline |
| **Who measured** | **`icc-conformance`.** This librarian ran no test, no build and no differential. **★ Every row in §3.14 and §3.15 is reported as reproducing BIT-IDENTICALLY across two independent invocations of the release runner** — the first time in this project's history that any measurement has been repeated at all, and it is worth more than its size suggests: §2.9's Pass 6 rows are *one run, no repetition, no variance*, and every Pass before it likewise |
| **Platform / pin** | Windows 11 Pro 10.0.26200 x86-64, MSVC, release. lcms2 pinned at **`21c582a`**, `transicc` banner *"LittleCMS ColorSpace conversion calculator - 5.1 [LittleCMS 2.19]"*. **Still no Linux run of anything, by anyone, ever** |
| **Gates, reported for the first time in seven filings** | `cargo test --workspace` — **exit code 0, 121 passed, 0 failed** (63 + 25 + 33 across three test binaries); `cargo fmt --check` on the **root workspace** exit 0; `cargo clippy --workspace --all-targets -- -D warnings` exit 0 *(all **reported** by `icc-conformance`, gated on `$?` rather than on text — which is the mechanical gate `NEXT_SESSION.md` item 0 and DL-024 both asked for)*. **What this discharges and what it does not** is in §7.10; the short form is that **NC-057 … NC-061 have a reported outcome for the first time**, and it is a **workspace-wide pass count, not per-row confirmation** |
| **★ A NEW apparatus gap, found by the same gate** | **`cargo fmt --check` FAILS in `tools/difftest`** — **109 diffs across 15 files**, all reported pre-existing *(reported)*. It matters because rule 10's gate is stated **workspace-wide** and **`tools/difftest` is deliberately not a workspace member** (DL-001 keeps the oracle out of the published artefact), **so `--workspace` cannot see it and never has.** The harness has evidently never passed `fmt --check`. `icc-conformance` formatted only `pass4c.rs` (its own, 8 diffs) and deliberately left the other 14 files alone, so a 101-diff reformat would not bury this session's change. **Owed, §7.10** |
| **★ The suite was IN FLUX during these runs, and not because of this work** | Reported: a first full run gave **`pass=134 fail=0 skip=3 error=0`, exit 0**; a second, ~5 minutes later, gave **`pass=140 fail=2 skip=3 error=0`, exit 1**, with `pass5c`'s record count moving **8 → 16** between them. **The two failures are both `pass5c`** — `…/FINDING/lcms2-destination-black-is-NEUTRAL-too` (tol 0, obs 5.000000e0) and `…/ATTRIBUTION/pass5b-recovery-was-the-round-trip` (tol 1, obs 1.062074e0) — i.e. **another agent's Pass 5c work, mid-flight** *(corroborated here independently: `tools/difftest/src/pass5c.rs` is **untracked**, and `TOLERANCES.md` has gained a **§3.5.8 "Pass 5c"** section naming iccce at commit `95c04c1` and **WITHDRAWING** row Q3's CONFIRMED verdict — verified, read, and neither was in this filing's dispatch)*. **Do not read this filing as reporting a green suite, and do not read those two failures as a regression from Pass 4c**: the sixteen rows below pass in **both** runs and reproduce bit-identically. **Whoever files the Pass 5c work reports its own outcome** |
| **★ What is NOT in this block, and it is the tenth time** | **No ground-truth row.** §3.14 and §3.15 are cross-checks against lcms2, self-consistency controls, and preconditions. **`TOLERANCES.md` §3.4.3's published-value row stays blank**, and **every transform this project has written still has zero `published-ground-truth` rows.** `TOLERANCES.md` §3.4.5 notes that `ICC.1:2022` Table D.2 *does* print an nCIEXYZ media white for SWOP within ~2×10⁻³ of this file's — **a published value for a white point is not a published value for a transform**, and it is a different characterization revision besides *(verified — read)* |

> **★★ DATED ADDENDUM TO §2.10, made MINUTES after the block above was
> written, because the tree moved underneath it — and the thing that
> moved it is the SAME PROCESS SLIP this filing had already recorded
> TWICE.**
>
> **The commit row above says "THERE IS NONE." That was true when
> written and is now false.** While this filing was being written,
> another agent committed **`5cfee17`** — *"difftest: the estimator
> discrimination — and lcms2 has TWO estimators"*, 2026-08-12 **09:06:21
> −04:00**, 23 files, +4 907 / −97 *(all verified — `git show --stat`
> run)*. **It contains `tools/difftest/src/pass4c.rs` (+1 027), the CLI
> help fix, `TOLERANCES.md` (+310) — and `docs/NUMERIC_CLAIMS.md`
> (+691), which is THIS FILING, mid-write.**
>
> **★ It was then PUSHED**: `origin/master` is now **`5cfee17`**
> *(verified — ref read)*, making **nine** `update by push` lines where
> DL-024 records two.
>
> **What is now true, stated exactly:**
> - **The Pass 4c apparatus IS committed**, at `5cfee17`. §3.15's rows
>   have a commit anchor after all, and the *"anchored to a working
>   tree"* warning above is **discharged for §3.14/§3.15**.
> - **This filing is SPLIT across a commit and a working tree.** The
>   ledger's §2.10/§3.14/§3.15/§3.16 text went in at `5cfee17`;
>   `ROADMAP.md`, `ARCHITECTURE.md`, `SESSION_LOG.md` and
>   `NEXT_SESSION.md` were still uncommitted at the time of writing.
>   **A single filing spanning two provenance states is a first, and it
>   is not a good first.**
> - **The commit message describes the estimator work and says nothing
>   about Pass 4c or about this ledger.** So **`git log` is now a
>   misleading index of when Pass 4c and its filing landed** — findable
>   only by `git log -- <path>`.
>
> **★★ THIRD INSTANCE OF ONE MECHANISM, IN ONE PROJECT, IN TWO DAYS.**
> This filing had already recorded two: **`edce48b`** (§2.6 — *"untracked
> in-progress `tools/gen-profiles` swept in by `d9e0b82`'s cwd-relative
> pathspec — a process slip"*) and **`dechk.obj`** (§7.10 item 1 — a
> stray object file swept into `aef7566` and published). **This is the
> third, and the victim is another agent's in-flight document.** A
> pattern that recurs three times in two days is **not a slip; it is the
> default behaviour of the commit command being used**, and it now has a
> demonstrated cost beyond untidiness: **it publishes work whose author
> has not finished checking it.** The ledger's own rule — *rows are
> never edited to make an old number look like a new one* — assumes the
> filing agent controls when its text becomes the record. **Here it did
> not.**
>
> **Owed to `icc-engineer`, and it is now the top process item:** commit
> with **explicit pathspecs**, never `-A` or a bare `.` from the
> repository root, while any other agent is working in the tree. **The
> repository is public and the push is automatic in practice** — three
> instances, three pushes, no review step between them.

### 2.11 ★★ The ESTIMATOR DISCRIMINATION (Pass 5b + Pass 5c), the Pass 6 gate re-graded at a new default grid, and Pass 1's last remainder. **The first block whose headline finding resolves in OPPOSITE DIRECTIONS on two arms of the same experiment.** Added 2026-08-12

An **eleventh** provenance block, the **third** dated 2026-08-12, and the
one that closes the project's original scope. It carries three kinds of
work that arrived together: a **conformance** result (the estimators,
finally discriminated), a **re-grade** of a Pass that had already been
filed (Pass 6, at a default the engineer moved rather than a tolerance he
moved), and the **closure of Pass 1's four-item remainder** at its
cheapest end.

| | |
|---|---|
| **Passes** | **5b** and **5c** — the black-point **estimators** (`tools/difftest/src/pass5b.rs`, `pass5c.rs`; `TOLERANCES.md` §3.5.7, §3.5.8; `tools/difftest/README.md` §17, §19). **6** — the compiled path **re-graded at the new default grid of 33** (`TOLERANCES.md` §3.6, README §18.2's re-grade box). **1** — ΔE94 and ΔE CMC, the last remainder item that was blocked on sourcing rather than on engineering (`crates/iccce-color/src/delta_e.rs`). Plus three pieces of engineering with no Pass of their own: the **ISO estimator wired to a caller**, **four API soundness defects**, and the **API sealing split** |
| **Date** | **2026-08-12** — the same calendar day as §2.9 and §2.10, and the project's **second** day |
| **Commits** *(hashes and subject lines corroborated by `.git/logs/HEAD`, read; **contents unverified**)* | **`fc4727b`** *"Pre-publication audit: four API soundness defects fixed, metadata, CI"* — ★ **committed at 07:08:16 −04:00, i.e. BEFORE the Pass 6 + Pass 7 filing commit `5867f1a` (07:21:36), and §2.9 does not mention it**; **`c268261`** *"cmm: wire the ISO estimator — it had no caller (Pass 5b finding)"*; **`189e732`** *"Pass 6 gate: default grid 17 -> 33, because the number would not move"*; **`aef7566`** *"color: dE94 and CMC — Pass 1's last remainder, honestly labelled"*; **`95c04c1`** *"api: seal the byte readers, keep the ISO surface public — a stated split"*; **`5cfee17`** *"difftest: the estimator discrimination — and lcms2 has TWO estimators"* — **the current tip** |
| **The repository, and exactly what is evidenced** | `.git/refs/heads/master` and `.git/refs/remotes/origin/master` both hold **`5cfee171…`** *(verified — both read)*. `.git/logs/HEAD` holds **52 lines**, all `commit`/`commit (initial)`, **no `reset`, `rebase`, `amend`, `checkout` or `merge` entry** *(verified — read end to end)*. **★ That corroborates the dispatch's "52 commits" by two independent routes**: the reflog line count, and §7.10's `git rev-list --count HEAD` = **51** at `95c04c1` plus the one commit since. **This is the first filing at which a dispatch's commit count and the file-derived count agree** — §2.9's disagreed (49 vs 45) and the file was right |
| **★ Nine pushes now, not eight — and one that FAILED** | `.git/logs/refs/remotes/origin/master` holds **nine** `update by push` lines, the last at **09:06:55 −04:00** carrying `95c04c1` → **`5cfee171`** *(verified — read)*. §7.10 item 2's observation is **unchanged and now larger**: **nothing in any document records a go-ahead for pushes three through nine**, and rule 9 plus DL-024 both say publishing is the operator's act. **Recorded as an observation, not an accusation.** ★ Separately, the engineer reports a **transient `HTTP 408` on one push**, retried successfully over **HTTP/1.1**. **A failed push leaves no reflog line**, so the failure is **reported** and only the success is **evidenced** — which is the cleanest small example in this project of the difference the two words carry |
| **★ THE LIBRARIAN HAD NO SHELL AT THIS FILING, AND HAD ONE AT THE LAST** | §2.10 corrected *"the librarian has no shell"* from a fact to a reading, having found a working `Bash` tool. **This session's tool grant contains no `Bash` tool at all** *(verified — the tool list)*. **So the correction does not generalise into a new standing fact**: shell availability is a property of a *session*, not of the agent, and both filings must be read with the date attached. Everything below marked *verified* was read in the working tree or in `.git/`'s plain-text files. **Three of §7.10's shell-derived items therefore could not be re-checked here and are labelled `unverified-this-filing`, not `owed`** (§7.10 item 8's own protocol, applied to itself the first time it could be) |
| **Who measured** | **`icc-conformance`** — every Pass 5b, Pass 5c and Pass 6 number below, via `TOLERANCES.md` §3.5.7 / §3.5.8 / §3.6 and `tools/difftest/README.md` §17 / §18 / §19 *(all four documents read here as the source; this librarian ran nothing)*. **`icc-engineer`** — the ΔE94/CMC transcription and its **C probe** compiled against the pinned lcms2, the ISO wiring, the API audit, and the CI run |
| **★ NO RUNNER OUTCOME accompanied this dispatch, and there is a specific reason to want one** | No `pass=`/`fail=` line and no `cargo test --workspace` count came with this filing. **The last runner outcome on record is §2.10's, and it was RED**: `pass=140 fail=2` with **both failures in `pass5c`**, mid-flight. Both have since been **re-formulated rather than widened** — the `…/FINDING/lcms2-destination-black-is-NEUTRAL-too` row is now `…/FINDING/divergence-chroma-follows-lcms2-BRANCH` (the needle moved from *"neutral"* to *"whatever the selected branch requires"*, which is the finding itself), and the attribution row is now **graded on the `swop` arm only**, on a stated units argument *(all verified — `TOLERANCES.md` §3.5.8.3 read)*. **This librarian's judgement: both re-formulations are defensible and neither is a moved tolerance** — the two constants (`0,0 exact` and `1,0`) are unchanged, and §3.5.8.3 gives the derivation for each. **But no run of the FINAL shape has been reported to anyone**, and eight ledger rows below rest on that shape |
| **Independently checkable without a shell** | **121 `#[test]` declarations across 19 files under `crates/`** *(verified — counted, no result limit; 116 across 19 at §2.9)*. §2.10 reported **121 passed**. ★ **The two numbers agreeing is a coincidence of two different quantities and must not be read as per-test confirmation** — one is a count of declarations in today's tree, the other is a pass count reported from a run at `95c04c1`, and one commit has landed since |
| **`dechk.obj`** | **Still present at the repository root** *(verified — the tree enumerated today)*. It is the **C probe's object file** from the ΔE94/CMC work — which is now traceable to its cause rather than merely to its commit. **Its TRACKED status and its presence in `origin/master` are §7.10's shell-verified findings and could not be re-verified here**; they are carried as **verified-then, unchecked-now** |
| **★ What is NOT in this block, and it is the eleventh time** | **No ground-truth row.** Pass 5c is a **reimplementation cross-check**; Pass 6's re-grade is `self-consistency`; ΔE94 and ΔE CMC are **`impl_crosscheck` by construction and say so in their own module doc**, because CIE 116-1995 and BS 6923 are paywalled and **no published worked example was obtained for either**. **NC-001 remains the project's only `published-ground-truth` row, and it is about a metric, not about a transform** |

### 2.12 ★★ The CONFORMANCE DEFECT IN OUR OWN CODE, a fifth crate that had been filed nowhere, and three green results that turn out to be three different instruments. Added 2026-08-12

A **twelfth** provenance block and the **fourth** dated 2026-08-12. It
is the first block whose headline is **a defect in iccce that the
specification settled against us**, and the first whose second-largest
item is **a documentation gap rather than a measurement**: a crate that
was in the build and in no document.

| | |
|---|---|
| **Passes** | **5c's open question, CLOSED** — ISO/CD 18619 4.2.5.4 (`crates/iccce-cmm/src/bpc.rs`). **10 pre-work** — `iccce-measure`, the CGATS/IT8.7 reader (`crates/iccce-measure/`). **6, weakened** — the throughput/speedup claim restated as a range. Plus the apparatus census that occasioned DL-031 |
| **Date** | **2026-08-12** — the same calendar day as §2.9, §2.10 and §2.11; the project's **second** day |
| **Commits** *(hashes and subject lines corroborated by `.git/logs/HEAD`, read; **contents unverified except where a file is named as read below**)* | **`fd34a44`** *"bpc: iccce was WRONG at 4.2.5.4 — lcms2 conformed, we did not"*; **`d5efd96`** *"Final filing + suite green at 142: the original scope is complete"*; **`2a2d616`** *"iccce-measure: CGATS/IT8.7 reader — Pass 10 pre-work, no hardware needed"* — **the current tip** *(verified — the last line of `.git/logs/HEAD`)* |
| **The repository, and exactly what is evidenced** | `.git/logs/HEAD` now holds **55 lines**, all `commit`/`commit (initial)` *(verified — read from line 40 to the end; no `reset`, `rebase`, `amend`, `checkout` or `merge` entry appears in the range read)*. **Three commits have landed since §2.11's tip `5cfee171`.** ★ **This filing did NOT read the refs or the push log**, so **nothing here evidences the tip having been pushed**, and §7.11's "nine pushes, seven unrecorded" is carried forward **unchecked** rather than restated |
| **★ NO SHELL AT THIS FILING** | The session's tool grant contains no `Bash` tool *(verified — the tool list)*. **Second consecutive filing without one, and the point of §2.11 stands**: shell availability is a property of a session. Every *verified* below was read in the working tree or in `.git/`'s plain-text files |
| **Who measured** | **`icc-engineer`** — all three runner outcomes, the `iccce bench` figures, and the 4.2.5.4 correction. **`icc-spec-librarian`** — the ISO/CD 18619 4.2.5.4 sourcing that settled the question. **This librarian ran nothing**, and read `bpc.rs`, `Cargo.toml`, `crates/iccce-measure/{Cargo.toml,src/lib.rs}`, `crates/iccce-color/Cargo.toml`, `docs/bench-2026-08-12.txt` and `TOLERANCES.md` §3.6 as the source |
| **★★ A RUNNER OUTCOME ARRIVED — three of them — and §7.11's item 2 is DISCHARGED** | `cargo test --workspace` → **129 passed, 0 failed**, bare exit 0. `cargo test` in `tools/difftest` → **36 passed**, exit 0. The conformance runner (`cargo run --release` in `tools/difftest`) → **pass=142 fail=0 skip=3 error=0**, **re-run today on current code**. §7.11 asked for *"one `pass=`/`fail=` line, and the `cargo test --workspace` exit code at the current tip"* and got both. **The twenty-four records of §3.18 and §3.19 now have a green run of their own shape** *(reported — see §3.22 for what corroborates it and what does not)* |
| **★★ Independently checkable without a shell, and this time it is a per-crate match** | **129 `#[test]` declarations across 20 files under `crates/`** *(verified — counted; 121 across 19 at §2.11, the difference being `iccce-measure`'s eight)*, distributed **cmm 63 · profile 33 · color 25 · measure 8 · cli 0**. **The reported pass counts are cmm 63, profile 33, color 25, measure 8, cli 0.** ★ **Every crate agrees exactly.** Unlike §2.11's coincidence of two totals, a **per-crate** agreement across five members is real corroboration that **no declared test was skipped, ignored or filtered out** — and it is **still a count of declarations, not of coverage** (§1.2), and it cannot corroborate that any test *passed*, only that the population is the one expected. Separately: **36 `#[test]` declarations across 6 files in `tools/difftest`** *(verified — counted)*, matching the reported 36 exactly |
| **★★★ Three claims in the dispatch that live source CONTRADICTED, and they are recorded because the rule that caught them is the reason this role exists** | **(1)** The dispatch stated that *"the manifest header still says `Four crates`"* and asked for that to be flagged as an owed correction. **It does not.** `Cargo.toml`'s header block reads *"Five crates, per docs/ARCHITECTURE.md §1"* and its `[workspace] members` lists all five *(verified — read)*. **Nothing is owed; the manifest was already correct.** **(2)** The dispatch stated that `ARCHITECTURE.md` §1 *"currently says 'Four crates'"*. **It did not say that either** — the string *"Four crates"* appeared nowhere in the file *(verified — searched)*; §1 carried an ASCII tree that **listed four crate directories and omitted the fifth**, which is the same defect but not the same text, and a filing that had corrected the quoted string would have corrected nothing. **(3)** The dispatch stated that *"the previous filing recorded 'suite green at 142'"*. **No filing did.** The string `142` occurs in `docs/` only as the CIE standard number **142-2001** *(verified — searched)*; *"suite green at 142"* is the **commit message** of `d5efd96`. ★ **The correction matters to the finding rather than merely to the record**: the number's ambiguity is worse than a filing's would have been, because it lives in **git history**, where nothing around it names an apparatus and no dated note can be appended to it |
| **★ What is NOT in this block, and it is the twelfth time** | **No ground-truth row.** The 4.2.5.4 correction is `normative-rule-conformance` against a **committee-draft** transcription; the census rows are `apparatus-census`; the throughput rows are `machine-timing`. **`iccce-measure` produces no row at all** — nothing in it has been compared to anything, deliberately. **NC-001 remains the project's only `published-ground-truth` row** |

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

### 3.6 Measurements **of the oracle** — added 2026-08-11 (difftest, commit `bfd6b1e`)

**Read §2.1 before quoting anything here.** All three rows are
**`oracle-behaviour-at-pin`**: they measure what lcms2 2.19.1 at commit
`21c582a` does. **`iccce` is not in the loop in any of them.** They are
in this ledger because DL-012 and DL-013 rest on them and because the
pin moving invalidates them — which is exactly the "is that still true?"
question §0 exists to make cheap.

#### ★ NC-019 — lcms2 keys the legacy 16-bit PCSLAB encoding off the **tag type**, not the profile version

| Field | Value |
|---|---|
| **What was compared** | `transicc` output (device→PCS through an `mft2` `A2B0`, Lab PCS, `-c0`, media-relative colorimetric) against **two hand-computed hypotheses**: legacy (ICC.1:2022 Tables 42/43 — `L* = v/652.80`, `a*,b* = v/256 − 128`) and general (6.3.4.2 Tables 12/13 — `L* = v·100/65535`, `a*,b* = v·255/65535 − 128`). |
| **Corpus** | **Four synthetic profiles authored byte by byte** by the probe itself (category (a), `LEGAL.md` §3): `probe_v2_1.icc` (`0x02100000`), `probe_v4_3.icc` (`0x04300000`), `probe_v4_4.icc` (`0x04400000`) — **byte-identical except the version word, asserted at run time as a byte diff at exactly offsets [8, 9]** — plus `probe_v4_3_mluc.icc`, a v4.3 with proper `mluc` metadata to close the "v2-era metadata in a v4 profile" objection. **4 probes each**, all on exact CLUT corners so nothing is interpolated. |
| **Coverage — part of the claim** | 4 profiles × 4 probes, **verdict taken at media-relative colorimetric only** (intent 0 is confounded — NC-020). **One tag (`A2B0`), one tag type (`mft2`), one direction (device→PCS), one PCS (Lab), one platform, one lcms2 build at one commit.** **`ncl2` was NOT tested behaviourally and neither was B2A** — for those, the claim is a *source reading*, which is a different and weaker object than a measurement and must not be merged into the same sentence. |
| **Tolerance** | **0.01**, as an *attribution* bound: how far an observation may sit from a hypothesis and still be attributed to it. |
| **Why that tolerance** | Justified from both ends, not picked. It is ~7× the 16-bit PCS quantisation floor (`100/65535 ≈ 0,0015`) plus `transicc`'s 4-decimal print, and ~20× **below** the smallest separation between the two hypotheses (≥0,196 in `L*`; ≈1,09 in `a*` at P4). No plausible rounding can move an observation from one hypothesis to the other, and an observation matching **neither** is reported as `INCONCLUSIVE` rather than snapped to the nearer. *(verified — `ATTRIBUTION` and its doc comment read in the probe source.)* |
| **Result** | **LEGACY, on all four profiles including v4.3, v4.4 and the `mluc` variant.** **Worst deviation from the legacy prediction across all probes and all four profiles: 2×10⁻⁵** — `transicc`'s printing precision. Note this is an **observed maximum**, not merely the asserted bound; §1.1's caveat does not apply to it, and it is the first row in this ledger that carries one. The **v2.1 control reads legacy**, so the instrument can detect the effect it is looking for. *(reported — the run is `icc-conformance`'s.)* |
| **Corroboration** | `src/cmsio1.c` `_cmsReadInputLUT` at the pin tests `_cmsGetTagTrueType` against `cmsSigLut16Type` and `cmsGetPCS` against `cmsSigLabData` and inserts `_cmsStageAllocLabV2ToV4` — **no version test**; same in `_cmsReadOutputLUT` / `_cmsReadDevicelinkLUT`; scale `65535/65280`. *(reported — transcribed in `tools/difftest/README.md` §12.2. No lcms2 source was read by this librarian; `vendor/` is git-ignored and not in this repository.)* |
| **Evidence class** | **oracle-behaviour-at-pin.** It does **not** establish that the tag-type rule is correct — that comes from ICC.1:2022 6.3.4.2 NOTE 3 and 10.10 (DL-011) and would stand unchanged had lcms2 disagreed (rule 7). |
| **★ A discrepancy found while filing, reported not repaired** | The probe's **module-doc** prediction table (`legacy_lab_probe.rs`, the "two predictions" table) and the same table in `tools/difftest/README.md` §12.1 disagree in two cells, both on the **rejected** hypothesis: for P3 the module doc prints general `L* = 50.0004` and for P4 general `a* = 125.9078`. Recomputing from the code's own `decode_general`: `32768·100/65535 = 50.000763 → 50.0008` and `65280·255/65535 − 128 = 126.007782 → 126.0078` — i.e. **the README's cells are right and the module doc's two cells are wrong.** *(Arithmetic done independently here; `decode_general` read in the source.)* **The verdict is unaffected**: the predictions are computed at run time by `decode_general`, not read from the prose, and both erroneous cells remain far outside the 0.01 attribution bound from the legacy values. It is a prose defect in a doc comment, it belongs to `icc-conformance`, and this librarian **did not edit that file** — reported here so it is findable. |
| **Where** | `tools/difftest/src/bin/legacy_lab_probe.rs`; `tools/difftest/README.md` §12.1–§12.3. |
| **Decision record** | `ARCHITECTURE.md` **DL-012** (which supersedes DL-011's *"live disagreement with lcms2"* clause; DL-011's rule itself stands). |
| **Invalidated by** | **The pin moving** (every number is scoped to `21c582a` — under DL-001 that is already a licence event; DL-012 makes it a behavioural one too). Also by any behavioural test of `ncl2` or B2A that disagrees with the source reading. |

#### ★ NC-020 — lcms2 forces BPC on for v4 profiles at perceptual and saturation: the mechanism, predicted and confirmed

| Field | Value |
|---|---|
| **What was compared** | `transicc` output at **intent 0 (perceptual)** through the same four probe profiles, against a **hand transcription of lcms2's own `ComputeBlackPointCompensation`** (`a = (bp_out − D50)/(bp_in − D50)`, `b = −D50·(bp_out − bp_in)/(bp_in − D50)`, per channel) applied to the legacy-decoded `L*`, with `bp_in` = lcms2's fixed perceptual reference black (`cmsPERCEPTUAL_BLACK_X/Y/Z` = 0.003 36 / 0.003 473 1 / 0.002 87) and `bp_out` = 0. |
| **Corpus / coverage** | The same 4 probes on the v4 profiles. **`L*` only** — deliberately: the probe pins down one channel to four decimals rather than three loosely. **One intent (0), one platform, one pin.** Saturation (intent 2) was **not** measured; that it is affected comes from the *source* (`_cmsLinkProfiles` tests both intents), which is a reading, not a measurement. |
| **Tolerance** | **0.005** asserted (`BPC_PREDICTION_TOL`). |
| **Why that tolerance** | ~3× the 16-bit `L*` quantisation step (`100/65535 ≈ 0,00153`) plus `transicc`'s 4-decimal print — tight enough that only the right formula passes, loose enough not to fail on the encoding grid. The effect being explained is ≈3,15 in `L*`, some 630× larger. *(verified — read in the probe source.)* |
| **Result** | Predicted matches observed on all four probes: `100.0000 → 100.0000` (Δ 0); **`0.0000 → −3.1482`** (Δ **3×10⁻⁵**); `50.1961 → 49.8574` (Δ 3×10⁻⁵); `100.0000 → 100.0000` (Δ 0). **Observed maximum 3×10⁻⁵**, against the 0.005 bound. *(reported.)* |
| **What it establishes** | That the ≈3.15 `L*` shift at black on v4 profiles at perceptual is **black point compensation**, forced on by lcms2 itself — not a Lab-encoding effect, not an interpolation artefact, and not something the user asked for (`-b` was not passed). |
| **The arm that did not decide, kept on the record** | Re-running the byte-identical **v2** profile at intent 0 **with `-b`** does *not* reproduce the v4 numbers: `-b` is a no-op there, because `cmsDetectBlackPoint` reaches the fixed perceptual constant only behind the same `>= 0x4000000` guard, and equal source/destination black points make lcms2 skip the stage. **Two arms differing in more than the variable settle nothing**, so it is recorded as inconclusive rather than as a refutation. |
| **Evidence class** | **oracle-behaviour-at-pin.** Not ground truth about colour, and **not** a finding that lcms2 is wrong: ICC.1 does not require the behaviour, and "not required" is not "prohibited". Note also that upstream's stated authority is *"Adobe's document"* — **which nobody in this project has obtained or read.** That is upstream's attribution, transcribed; it is not a citation this project can check. |
| **Where** | `tools/difftest/src/bin/legacy_lab_probe.rs` (`predict_bpc_lstar`, `LCMS2_PERCEPTUAL_BLACK`, `BPC_PREDICTION_TOL`); `tools/difftest/README.md` §12.4; `docs/TOLERANCES.md` §6.1 item 2. |
| **Decision record** | `ARCHITECTURE.md` **DL-013**. |
| **Consequences for future rows — this is why it is in the ledger** | Any future cross-check row at **perceptual or saturation against a v4 profile** is measuring a transform **with BPC in it**, whether or not `-b` was passed. **Pass 4** must either account for that explicitly or restrict its cross-check to the colorimetric intents and say so; **Pass 5**'s `-b`-on/`-b`-off pairing does **not** isolate the variable on v4 profiles at those intents. A tolerance set without knowing this is a tolerance set on the wrong quantity. |
| **Invalidated by** | The pin moving; any lcms2 change to `_cmsLinkProfiles` or to the perceptual black constants. |

#### NC-021 — the registered smoke check: the oracle still answers the same

| Field | Value |
|---|---|
| **What was compared** | **lcms2 against lcms2.** `smoke/srgb-white-to-lab`: system sRGB profile → `*Lab4`, media-relative colorimetric, `-c0`, input `255 255 255`, compared to `99.9988 0.0188 −0.0173` — a value recorded from **this same pinned oracle** in `tools/difftest/README.md` §8.2 on 2026-08-11. |
| **Kind, as the harness itself labels it** | **`oracle-reproducibility`** — the harness's own `Kind` enum carries this variant precisely so "both sides are lcms2" cannot be written down as anything stronger. *(verified — `Kind::OracleReproducibility` read in `tools/difftest/src/lib.rs` and in the registered check in `src/main.rs`.)* |
| **Metric / tolerance** | `abs-max-component` / **1×10⁻⁴**. |
| **Why that tolerance** | `transicc -n` prints four decimals and the recorded expectation is itself a four-decimal print, so agreement cannot be asserted more tightly than the reference is printed. **Arithmetic-agreement, not perceptual** — the 1.0 ΔE2000 anchor (DL-004) is irrelevant and must not be cited for it. *(verified — the `Tolerance` value carries its `why` string in the source; the type cannot be constructed from a bare number.)* |
| **Result** | **PASS, observed deviation `0.000000e0`** (exact agreement). *(reported.)* |
| **Coverage — and it is small** | **One profile, one direction, one input triplet, one intent, one precalc mode, one platform.** Exactly **one** check is registered in the whole harness. The profile is category (c) under `LEGAL.md` §3 (read locally, never committed), so the check **skips** where it is absent and the runner exits **3 (nothing ran)** — never 0. |
| **★ What it proves, in the harness's own words** | Harness-and-pin stability, **and nothing about colour**. It says nothing about whether lcms2 is right and nothing about `iccce`, which is not in the loop. These numbers **must never be transplanted into an `iccce-color` or `iccce-cmm` unit test as expected values** — at that moment the claim would silently change from "the oracle still answers the same" to "iccce is correct" (rule 3). |
| **Evidence class** | **oracle-behaviour-at-pin.** |
| **Where** | `tools/difftest/src/main.rs` (`checks()`, `PRINTED_PRECISION`); `tools/difftest/README.md` §11.3. |
| **Invalidated by** | The pin moving; a rebuild on a different toolchain; the system profile changing (it is not ours and is not committed). |

### 3.7 Pass 3 — the matrix/TRC path (commit `c4038eb`, 2026-08-11)

**Read §2.3 before quoting anything here.** Twelve rows, and **not one
of them has lcms2 in it.** Six are **`normative-rule-conformance`** — a
class added today, whose expectation comes from verbatim normative text
in the corpus at `primary_spec` tier. Five are `arithmetic-identity` or
`self-consistency`. One (NC-031) is a **fact about a file**.

#### 3.7.0 ★ The Pass 3 done-when numbers are NOT here, and this is where they will go

Pass 3's done-when reads: *"sRGB→AdobeRGB round-trips within a stated
ΔE, and matches lcms2 within a stated tolerance, with both numbers
written down."* **Neither number exists in this ledger as of this
filing.** `icc-conformance` was dispatched **in parallel with this
librarian** to produce them, on the operator's standing instruction to
run disjoint file sets in parallel. Stated exactly so nobody reads
§3.7's twelve rows as the done-when being met:

- **The round-trip ΔE across two different profiles** (sRGB→AdobeRGB→sRGB)
  is **not measured**. NC-032 is a round trip through **one** profile in
  **device units**, not a ΔE across a profile *pair*, and the two must
  not be conflated: NC-032 prices table quantisation and inversion,
  while the done-when's figure additionally prices the matrix chain and
  the gamut clipping between two different primary sets.
- **The lcms2 tolerance is not measured, not justified and not set.**
  Whatever arrives must be justified **before** the run, not fitted
  after it (rule 5, `TOLERANCES.md` §0), and the row that carries it
  will be this ledger's **first `implementation-cross-check`** row.
- **No NC number is reserved for either.** A reserved number is a
  promise, and an unfilled promise in a ledger reads like a lost row.
  The next free number after this filing is **NC-034**.
- **Whether the parallel run landed is `unverified` here.** A later
  session must check for the rows rather than assume the dispatch
  succeeded — the same rule that has now twice found an item carried as
  outstanding was in fact done (§7.1, §7.2).

Until those rows exist, **Pass 3's done-when is not met**, and
`ROADMAP.md`'s Pass 3 progress block says so in the same words.

> **★ Dated status of §3.7.0, 2026-08-11 (Pass 3 closure filing) — the
> held space is FILLED. Nothing above is edited.** The parallel
> `icc-conformance` dispatch **landed**, and this was established by
> **looking** (`tools/difftest/README.md` §13 and `TOLERANCES.md` §3.3
> read in the live tree) rather than by trusting the dispatch that said
> so. The rows are **§3.8**, numbered **NC-034 … NC-043** — starting at
> NC-034 exactly as this section predicted the next free number would be,
> because **no number was reserved**. Three specific statements above are
> now superseded and the way each was superseded matters:
>
> - *"The round-trip ΔE across two different profiles is not measured"* —
>   **now measured**, NC-038, and it is a **larger** quantity than NC-032
>   for exactly the reason stated here: it prices the matrix chain and the
>   clipping between two different primary sets. **1.8788×10⁻²** against
>   NC-032's `1×10⁻³`-bounded single-profile trip — an order of magnitude,
>   and the conflation this section warned against would have been a real
>   error.
> - *"The lcms2 tolerance is not measured, not justified and not set"* —
>   **set before the run and tested after it** (NC-036, NC-041). It was
>   justified from **lcms2's own arithmetic**, and the justification was
>   then **checked by emulating that arithmetic**, which is a stronger
>   procedure than this section asked for.
> - *"Whether the parallel run landed is `unverified` here"* — **it
>   landed.** That is now the **fourth** consecutive filing at which
>   checking beat assuming, in both directions: three items carried as
>   outstanding were done, and one prediction carried as fact was refuted
>   by the code.

#### 3.7.1 `normative-rule-conformance` — expectations taken from Annex F and clause 10.6

The corpus file for all six is
`ICC_Spec\icc\icc__s__computational_models.md` (`evidence:
primary_spec`, whole file) except NC-025, whose clause is carried by
`icc__type__curve_parametric.md` (`primary_spec` for clauses 10.6 /
10.18 / Annex F.1). *(both frontmatter lines verified — read
2026-08-11.)* **All six inherit the transcription risk named in §1**:
they prove iccce matches *the corpus's transcription of* the clause, and
the corpus's own C1/C2/C3 errata show transcriptions can be wrong.

| ID | The rule, and where the expectation comes from | Tolerance | Result | Coverage — part of the claim | Where (all *verified*) |
|---|---|---|---|---|---|
| **NC-022** | **F.1(a), first case.** A flat subdomain ending **before** the domain end inverts to the **highest** x. Table `[0, A, A, 40000, 65535]`, plateau on x ∈ [0.25, 0.5] → expected **0.5** | `< 1×10⁻¹²` | holds | **one table, one plateau, one probe.** The expectation is read off the verbatim rule, not off the code | `curve.rs::tests::f1_plateau_mid_domain_inverts_to_highest_x` |
| **NC-023** | **F.1(a), second case.** A flat subdomain **reaching** the domain end inverts to the **lowest** x. Table `[0, 30000, M, M, M]` → expected **0.5**. **The rule FLIPS between NC-022 and NC-023**, which is the whole reason both are tested | `< 1×10⁻¹²` | holds | one table, one plateau, one probe | `curve.rs::tests::f1_plateau_at_domain_end_inverts_to_lowest_x` |
| **NC-024** | **F.1(b).** An unattainable `y` clamps to the nearest attainable `y` and returns its `x`. Table with range ≈[0.1, 0.9]; `y = 1.0 → x = 1`, `y = 0.0 → x = 0` | `< 1×10⁻¹²` | holds | one table, two probes, both endpoints | `curve.rs::tests::f1b_out_of_range_clamps_to_attainable` |
| **★ NC-025** | **Clause 10.6 layout.** Entry *i* sits at `x = i/(n−1)` and decodes as `t[i]/65535`; between entries the rule is **linear** (A15, RESOLVED — normative, not a choice) | **`< 1×10⁻¹⁵`** at the samples; `< 1×10⁻¹²` at the midpoint | holds | one 4-entry table, **all 4 samples including both endpoints**, one interpolated midpoint | `curve.rs::tests::table_eval_exact_at_samples` |
| **NC-026** | **F.1 on a decreasing curve.** A monotonically *falling* table inverts with the internal mirroring undone. `[65535, 32768, 0]`: `y = 32768/65535 → x = 0.5`; `y = 1.0 → x = 0.0` | `< 1×10⁻¹²` | holds | **one falling table, two probes.** The probe value is the *exact encoding* `32768/65535`, **not 0.5** — the test's first draft assumed 0.5 and failed, an encoding slip **in the expectation**, not in the code | `curve.rs::tests::falling_table_inverts` |
| **NC-027** | **F.8–F.16 order.** The linear component is clamped to [0,1] **before** the inverse TRC. An out-of-gamut PCSXYZ drives red and blue negative; both come back **exactly `0.0`** (`assert_eq!`), and every channel is finite and in [0,1] | **exact** on the two clamped channels | holds | **one out-of-gamut XYZ, one synthetic model** (gamma 2.2, one matrix). **The test asserts the output value, not the code path** — which is what makes it a conformance row rather than a comment | `matrix_trc.rs::tests::out_of_gamut_clamps_before_inverse_trc` |

**★ Why NC-025 is starred.** It is the row that caught a real bug on
its first run — an off-by-one that returned `t[n−2]` at `x = 1.0` — and
**the two self-consistency checks in this same Pass would both have
passed with that bug in place.** The arithmetic, the counterfactual and
the method rule that follows are `ARCHITECTURE.md` **DL-016**; the
margin is restated in NC-032 below, where the bound that would have
absorbed it lives.

**★ Why NC-027 matters more than its size suggests.** Clamping *after*
the inverse TRC instead of before gives a different answer whenever
`TRC⁻¹` is non-identity near the endpoints — always, for a gamma curve.
The corpus's stated symptom is *"out-of-gamut saturated colours land on
the wrong device value; the gamut boundary is subtly the wrong shape"*,
and it flags it **Quiet**. This is a rule-1 defect shape: nothing about
it announces itself.

#### 3.7.2 Arithmetic identities — Pass 3

Same warning as §3.2: these prove the code is **structurally** sound and
has not drifted. **They cannot detect a consistently wrong constant**,
and a round trip through a wrong matrix round-trips perfectly.

| ID | What | Tolerance | Result | Coverage | Where (all *verified*) |
|---|---|---|---|---|---|
| **NC-028** | `(x^g)^(1/g) = x` for the `u8Fixed8`-exact gamma **2.19921875** | `< 1×10⁻¹²` | holds | **5 probes** — 0.0, 0.1, 0.5, 0.9, 1.0, both endpoints included | `curve.rs::tests::gamma_round_trip` |
| **NC-029** | Parametric **type 3** forward∘inverse round trip, **both branches** | `< 1×10⁻⁹` | holds | **6 probes** — 0, 0.02, **0.04045 (the branch point itself)**, 0.05, 0.5, 1.0. Parameters are the corpus's sRGB *shape* (γ=2.4, a=1/1.055, b=0.055/1.055, c=1/12.92, d=0.04045) used **as an arithmetic fixture, NOT as a claim about sRGB** — the test says so, and `iec__s__srgb.md` is single-source (§5) | `curve.rs::tests::parametric_type3_round_trip` |
| **NC-030** | Matrix/TRC device→PCS→device on a **synthetic** model (gamma 2.19921875, a well-conditioned 3×3) | `< 1×10⁻¹²` per channel | holds | **3 triples**: black, white, and (0.2, 0.5, 0.8). No profile bytes involved | `matrix_trc.rs::tests::round_trip_is_identity` |

#### 3.7.3 ★ NC-031 — the sRGB profile's colorant sum, and a tolerance re-justified after it failed

| Field | Value |
|---|---|
| **What was compared** | `MatrixTrc::device_to_pcs([1,1,1])` on the **real system sRGB profile** — i.e. the sum of the profile's `rXYZ`+`gXYZ`+`bXYZ` colorants, since `TRC(1) = 1` — against **ICC's 4-figure D50** (0.9642 / 1.0000 / 0.8249), the PCS white a well-formed media-relative profile's colorants should sum to. |
| **Corpus** | **One profile**, `C:\Windows\System32\spool\drivers\color\sRGB Color Space Profile.icm` — the 1998 HP/Microsoft profile. **Category (c) under `LEGAL.md` §3**: read locally, never committed, and **the test skips silently when it is absent.** On a machine without it, this row asserts nothing and the suite is still green. |
| **Tolerance** | **`1×10⁻²` on X and Z; `1×10⁻³` on Y.** |
| **★ The finding, and why the tolerance is what it is** | The test **failed on first run** at a `1×10⁻⁴` bound. Per rule 5 the arithmetic was checked before the code was blamed — and **the code was right**: this profile's colorant **Z sums to 0.825089**, i.e. **`1.9×10⁻⁴` away from 0.8249**. *(the sum is **reported** by `icc-engineer` from the failing run; `icc-librarian` has no shell and did not read the profile's bytes. The subtraction `0.825089 − 0.8249 = 1.89×10⁻⁴` **was** checked here.)* That is the **1998 author's own white-point rounding — a fact about the FILE**, not an error in iccce and not a quantisation floor. A tolerance derived from `s15Fixed16` quantisation would have been *"a claim the file never made"*. |
| **How the replacement bound is justified — by what it discriminates** | The check exists to catch a **D65-referenced colorant set** (the classic authoring error, and the shape of a transposed or unadapted matrix): D65's `Z ≈ 1.089`, which is **0.264 from 0.8249 — 26× the `1×10⁻²` bound** *(recomputed here from NC-018's derived D65)*. Authoring spread is ≈`2×10⁻⁴`, **50× inside** it. So the bound sits between the two by a factor of ~26 either way: **it cannot fail on a well-formed profile and cannot pass a wrong white.** This is a tolerance justified by its **discrimination**, which is the standard `TOLERANCES.md` §0 asks for, and it is the project's cleanest worked example of rule 5 to date — *the first question when a test fails is whether the code is wrong*, and here the answer was no, twice over (neither the code nor the file was wrong; the **tolerance's premise** was). |
| **Evidence class** | **arithmetic-identity** on iccce's side (it would catch a transposed colorant matrix — see NC-033), carrying a **reported fact about a file**. **It is NOT a correctness claim about iccce's colour**, and **not** a claim that the profile is accurate. |
| **Where** | `matrix_trc.rs::tests::system_srgb_profile_end_to_end`, the three white assertions and their comment. *(verified — read.)* |
| **Invalidated by** | The profile being replaced or updated on this machine (it is not ours and is not committed); any change to `illuminant.rs`'s D50; any change to how colorant tags are assembled into the matrix. |

#### 3.7.4 ★ NC-032 — the real-profile round trip, and the bound that would have hidden the bug

| Field | Value |
|---|---|
| **What was compared** | `pcs_to_device(device_to_pcs(rgb))` against `rgb`, on the **real system sRGB profile**, through its sampled TRC tables and its colorant matrix — **one profile, source and destination the same**. |
| **Tolerance** | **`1×10⁻³` device units** per channel. |
| **Result** | holds. Per §1.1 this is the **bound asserted**; the test comment states residuals are *"far below it"*, but **no observed maximum was carried in the dispatch and none is on record.** |
| **Coverage — part of the claim** | **3 triples** (black, white, and 0.25/0.5/0.75), **one profile, one machine**, and the test **skips silently if the profile is absent**. |
| **Evidence class** | **self-consistency** — both sides are iccce. It is the only way to *price* the table-quantisation + F.1-inversion approximation, and it is **worthless as correctness evidence**: a wrong matrix round-trips perfectly. |
| **★ Its stated justification is arithmetically off by a factor of two, and the bound is fine anyway** | The comment justifies `1×10⁻³` as *"~2× the table's input spacing (1/1023)"*. But `1/1023 = 9.775×10⁻⁴`, so `1×10⁻³` is **≈1.02×** that spacing, not ≈2×. The "~2×" reading holds only against the **half**-spacing (`1/2046 = 4.888×10⁻⁴`), which is the natural scale for a worst-case interpolation mismatch and is probably what was meant. **Reported, not repaired** (the file is the engineer's). It matters because in this project the *justification* is the claim — a bound whose stated derivation is off by 2× is a bound nobody can check by reading it. |
| **★★ This bound would have PASSED with the `eval_table` bug in place** | With that bug, `TRC(1.0)` returned the second-to-last table entry, and inverting it lands exactly one table spacing from 1.0 — **`9.775×10⁻⁴` for a 1024-entry table, inside the `1×10⁻³` gate with ~2 % of margin.** The error a spacing-derived bound must catch is *exactly the spacing*, so the two quantities are the same quantity and the bound cannot discriminate. Only NC-025's `1×10⁻¹⁵` exact-value assertion caught it. **This arithmetic was computed by `icc-librarian` from the code as written; nothing was run, and the 1024-entry table size is the engineer's statement in a comment, unverified here.** Full record and the method rule that follows: `ARCHITECTURE.md` **DL-016**. |
| **What it does NOT price** | The **cross-profile** path. Source and destination are the same profile here, so the matrix and its inverse cancel exactly and the row prices only the curve stack. The done-when's sRGB→AdobeRGB figure is a different and larger quantity (§3.7.0). |
| **Where** | `matrix_trc.rs::tests::system_srgb_profile_end_to_end`, the round-trip loop and its comment. *(verified — read.)* |
| **Invalidated by** | Any change to `eval_table`, `invert_table`, or the F.1 tie-break; the profile changing on this machine; a table-representation change (DL-016's revisit condition). |

#### 3.7.5 NC-033 — the synthetic twin of NC-031

| Field | Value |
|---|---|
| **What** | On the **synthetic** model, white `[1,1,1]` maps to the colorant **row sums**: `X = 0.4361 + 0.3851 + 0.1431` and `Y = 1.0`, within `1×10⁻¹²`. |
| **Why it earns a row** | It is the check that would catch a **transposed colorant matrix**. `M·[1,1,1]` is the row sums; a transposed assembly would return each colorant's own component sum instead, and the numbers differ. A transposed matrix is the canonical *"uniform colour cast that looks like a white-point problem"* — the shape `ARCHITECTURE.md` and the Pass 2 record both name as the wrong-colour-looks-right archetype. |
| **Evidence class** | **arithmetic-identity.** Two of the three components are asserted (`X`, `Y`); `Z` is not. |
| **Where** | `matrix_trc.rs::tests::white_maps_to_colorant_sum`. *(verified — read.)* |

#### 3.7.6 Refusals and degenerate-input guards — behavioural, not numeric

Recorded here so the ledger is a complete account of what Pass 3
asserts. **Every one of these is a refusal *by name*, never a
substitution** — the CMM-layer expression of invariant §3.2 and of the
rule that a plausible substitute is indistinguishable from a bug.
*(all verified — read in the live source.)*

- **A constant curve and a non-monotonic curve are DIFFERENT errors.**
  `ConstantNotInvertible` versus `NonMonotonicInverseUndefined` —
  because Annex F draws exactly that distinction (*"cannot"* be
  inverted versus the inverse is *"undefined"*), and merging them would
  erase it. The non-monotonic case is where the spec permits **anything**
  and iccce refuses instead of choosing silently.
  (`curve.rs::tests::constant_and_nonmonotonic_are_distinct_errors`)
- **Parametric inverses for types 1, 2 and 4 are refused by name**
  (`InverseUnsupported { func_type }`) rather than sampled. A sampled
  inverse would be an **unmeasured approximation**, which rule 4
  forbids. Types 0 and 3 are analytic. Recorded as a Pass 3 remainder in
  `ROADMAP.md`. (`curve.rs::tests::unsupported_parametric_inverse_refused`)
- **A Lab-PCS profile is refused by name** for the matrix/TRC model —
  `PcsNotXyzRefused`, whose message contains the string `"Annex F.3"`,
  **asserted** — on the strength of F.3's verbatim *"Only the PCSXYZ
  encoding can be used with matrix/TRC models."* Tested against the
  **real** `USWebCoatedSWOP.icc`, i.e. against the actual population the
  refusal exists for, not a fixture.
  (`matrix_trc.rs::tests::lab_pcs_profile_refused_by_name`)
- **A singular colorant matrix** yields `SingularMatrix` rather than
  infinities; **degenerate parametric parameters** yield
  `DegenerateParams { func_type }`; a **short parameter list** yields
  `ParametricUnevaluable`; a **curve table shorter than 2 entries**
  yields `TableTooShort` even though the profile layer should have
  refused it first.
- **Non-media-relative intents are refused by the CLI, by name**, with
  the message *"not implemented (Pass 3 implements media-relative
  only); refusing rather than substituting"* and exit code 1. There is
  **no test asserting this**; it was read in the source.

> **★ Dated status of §3.7.6, 2026-08-11 (Pass 3 closure filing).** Two
> of these refusals have since been **discharged by implementation**, and
> one has **not**, and the difference is worth stating rather than
> quietly re-writing the list:
>
> - **The parametric type-1/2/4 inverse refusal no longer exists.**
>   `invert_parametric` handles **all five** function types analytically
>   and the `InverseUnsupported { func_type }` variant is **gone from the
>   enum** *(verified — the whole function and the error type read)*. No
>   sampled inverse was introduced, so rule 4's "an approximation needs a
>   measured cost" was never engaged. Type 4's discontinuous-branch gap
>   returns the boundary `d`, cited as *"the F.1(b) posture applied to
>   the gap"* under corpus **A18** (the spec imposes no continuity at the
>   breakpoint) — a **named posture**, and the nearest thing in the new
>   code to a choice rather than a derivation.
> - **The absolute-intent refusal is gone from the library** and replaced
>   by the D.6/D.7 implementation (§3.8, and `ROADMAP.md`'s completion
>   record).
> - **The CLI's intent refusal is UNCHANGED.** `cmd_transform` still
>   refuses everything but `media-relative`, by name, with exit 1
>   *(verified — read)*. Since `tools/difftest` deliberately drives the
>   **binary**, **absolute intent cannot be cross-checked against lcms2
>   at all until the CLI exposes it** — the implementation currently
>   carries unit-test and corpus evidence only and **zero cross-check
>   evidence.** That is a coverage fact, not a defect, and it is the kind
>   that goes invisible fastest.
> - Still true, unchanged: **there is no test asserting the CLI's
>   refusal.**

### 3.8 ★★ Pass 3's done-when — the differential against lcms2. The ledger's FIRST `implementation-cross-check` rows

**Read §2.4 before quoting anything here**, and read the shared coverage
box below before quoting any single row. Ten rows, **NC-034 … NC-043**.
Four are `implementation-cross-check` — a class this ledger has never
carried until today. Two are `self-consistency`. One is
`oracle-behaviour-at-pin`. One is a fact about two files. Two are means
recorded **with an infinite tolerance**, so that the distribution sits
next to the maximum without ever being mistaken for it.

> **★ SHARED COVERAGE — part of every claim in this section, and it must
> travel with any row that is quoted.**
> **One profile pair**: the Windows system `sRGB IEC61966-2.1` (HP, 1998)
> → `Adobe RGB (1998)` (Adobe, 2000), **both v2.1**, both **category (c)**
> under `LEGAL.md` §3 (read locally, never committed — so **every row
> here skips** on a machine without the Windows colour directory, and the
> runner then exits **3, "nothing ran"**, not 0). **One intent**
> (media-relative colorimetric — the only one the CLI exposes). **One
> direction.** **133 deterministic grid points** (8 corners, 17 neutrals,
> a 4×4×4 lattice, 6 half-saturated primaries/secondaries, 48 fixed-seed
> pseudo-random interior points; no `rand`, no clock, count asserted by a
> unit test). **One platform**, **one lcms2 build at one pin**.
> **NOT covered, stated because "verified" without scope is the failure
> this document exists to prevent:** no v4 profile anywhere; no LUT
> profile, no CMYK, no grey, no `chad`; **no other intent, including the
> absolute intent this Pass implements**; nothing below 1/16 device
> except exact zero, which is precisely where the device-space bound is
> least transferable; **no genuinely out-of-gamut input**, because
> sRGB ⊂ Adobe RGB makes real clipping impossible in this direction.
> **The TRC pair is the one thing that is broad**: the source's curves
> are 1024-entry sampled tables and the destination's is an analytic
> `u8Fixed8` gamma (2.19921875), so one run exercises table
> interpolation **and** analytic evaluation, Annex-F.1 table inversion
> **and** analytic inversion — both curve paths in both directions. Had
> both been gammas, half of `iccce-cmm::curve` would have gone untested
> under a report saying *"sRGB → Adobe RGB verified"*.

#### 3.8.1 The seven emitted records, at a glance

| ID | Record | Kind | Metric | Tolerance | **Observed** |
|---|---|---|---|---|---|
| **NC-034** | `pass3/srgb-to-adobergb/device-vs-lcms2` | **implementation-cross-check** | device abs-max, normalised 0..1 | **5×10⁻⁴** | **6.7059×10⁻⁵** |
| **NC-035** | `pass3/srgb-to-adobergb/device-mean` | implementation-cross-check | device abs-mean, 0..1 | **∞ — reported, NOT graded** | 6.1672×10⁻⁶ |
| **★ NC-036** | `pass3/srgb-to-adobergb/de2000-vs-lcms2` | **implementation-cross-check** | ΔE2000 **max**, `kL=kC=kH=1`, D50 | **2×10⁻²** | **3.4762×10⁻³** |
| **NC-037** | `pass3/srgb-to-adobergb/de2000-mean` | implementation-cross-check | ΔE2000 mean | **∞ — reported, NOT graded** | 5.1145×10⁻⁴ |
| **★ NC-038** | `pass3/srgb-to-adobergb-to-srgb/roundtrip-de2000` | **self-consistency** | ΔE2000 max | **2.5×10⁻²** | **1.8788×10⁻²** |
| **★ NC-039** | `pass3/roundtrip/white-clamp-cost-matches-prediction` | self-consistency (**a prediction pin**) | \|predicted − observed\| ΔE2000 | **1×10⁻³** | **5.7392×10⁻⁶** |
| **NC-040** | `pass3/instrument/adobergb-device-to-lab-ruler` | implementation-cross-check (**instrument check**) | ΔE2000 max | **5×10⁻²** | **8.7945×10⁻⁵** |

**The two `∞` rows pass because there is nothing for them to fail**, and
that is deliberate: **a mean over a grid hides exactly the outlier a
colour engine gets wrong.** They exist so the distribution is on file,
and quoting one for the other is the misuse the `metric` column prevents.

**NC-040 is the instrument, not the subject.** It holds iccce's Adobe RGB
device→Lab model — called **in-process**, the single documented exception
to "answers come from subprocesses" — against `transicc -o*Lab4` over the
same 133 device values. It exists because NC-036 … NC-039 grade iccce
**with a ruler built partly out of the code under test**; a bent ruler
would mis-scale them systematically and the error would hide *inside the
metric* instead of appearing as a number. At 8.79×10⁻⁵ ΔE2000 — **below
`transicc`'s own ~1×10⁻⁴ Lab print floor** — the two rulers are
indistinguishable.

#### 3.8.2 ★★ NC-036 — iccce against lcms2 in ΔE2000. **The first `implementation-cross-check` row in this project's history**

| Field | Value |
|---|---|
| **What was compared** | The **shipped `iccce transform` binary** (`--src` sRGB, `--dst` AdobeRGB1998, media-relative) against **`transicc`** (`-t1 -c0`, lcms2's most accurate path) over the 133-point grid, both converted to D50 CIELAB and differenced with `iccce_color::delta_e_2000`. Both sides are **subprocesses**; neither implementation is called in-process. |
| **Tolerance** | **2×10⁻² ΔE2000 max.** |
| **★ Why that tolerance — and it was justified BEFORE the run** | Derived from **lcms2's own arithmetic, not iccce's**. Carrying the device value back through the destination model undoes the unbounded inverse-gamma amplification that makes NC-034 grid-dependent, so a finite ceiling exists over the whole cube: 2.5×10⁻⁵ source-linear → ≤2.5×10⁻⁵ in PCS XYZ (‖M_src‖∞ = 1.0, the Y row, by construction for a D50-referenced media-relative profile) → through Lab's steepest sensitivities (`f'(t) = 7.787` on the linear segment, `dL*/dY ≤ 903.3`, `da*/dX ≤ 4038`) → **worst case ≈0.28 ΔE00**. **2×10⁻² is set deliberately TIGHTER than the ceiling it derives**, because 0.28 is a pessimistic union bound and a residual that had quietly grown from 3×10⁻³ to 0.27 would still pass a 0.28 gate with nothing to show it. That is §3.1's boxed warning applied to a new row. It is **50× below** the 1.0 ΔE2000 perceptibility anchor and **inherits that anchor's ⚠ provisional status** (DL-004) — which it can afford at 50×. |
| **Result** | **max 3.4762×10⁻³, mean 5.1145×10⁻⁴** — an **observed maximum**, not merely the asserted bound. §1.1's caveat does **not** apply to this row. |
| **Evidence class** | **`implementation-cross-check`.** Per §1 and rule 7 this is evidence that **two implementations read Annex F.3 the same way** — nothing more. **It is not ground truth, and the shared-misreading risk is ELEVATED here, not merely present**: the corpus's sRGB constants rest on **lcms2 alone** (IEC 61966-2-1 paywalled) and **D65 is single-source** from `cmsvirt.c` (NC-018, the weakest constant in `iccce-color`). Nothing in Pass 3 reads those constants — the profiles supply their own colorants — but the *corpus* against which any future ground-truth check would be built shares an origin with the oracle. |
| **★ The justification was TESTED, not asserted** | See **NC-041**. Modelling lcms2's own 16-bit tone-curve quantisation inside iccce's model collapses the device-space residual by a factor of ~290. The disagreement is **accounted for**, not merely tolerated. |
| **Where** | `tools/difftest/src/pass3.rs` (the tolerance constant and its `why`), `src/bin/pass3_report.rs`; `tools/difftest/README.md` §13.5, §13.6.2, §13.9; `TOLERANCES.md` §3.3.1 row 3. |
| **Measured** | 2026-08-11 by `icc-conformance` *(reported)*; re-run reported by `icc-engineer` *(reported, and see §2.4's count discrepancy)*. **`icc-librarian` ran nothing** and read the numbers in three places. |
| **Invalidated by** | **The lcms2 pin moving off `21c582a`** (re-run, do not re-read); either profile changing on this machine; any change to `MatrixTrc`, `curve.rs`, or `iccce transform`'s print precision; **any change to the 133-point grid**, which changes the scope of the number and not merely its value. |

#### 3.8.3 NC-034 / NC-035 — device space, and a bound that is grid-dependent by construction

| Field | Value |
|---|---|
| **What** | The same two subprocess outputs compared **in normalised device units**, `lcms2`'s output first **clamped into `[0,1]`** so the row grades *arithmetic* disagreement rather than the clamping-policy difference NC-043 records separately. |
| **Tolerance / result** | **5×10⁻⁴**; observed **6.7059×10⁻⁵** max (0.0171 in 0..255), **6.1672×10⁻⁶** mean. |
| **★ Why the bound is what it is, and why it is honest about not generalising** | `cmsEvalToneCurveFloat` rounds a **segment-free (tabulated)** tone curve's input *and* output to 1/65535; the source profile's TRCs are exactly that case, so each rounding is ≤½ lsb = 7.63×10⁻⁶, the input term amplified by the sRGB EOTF's peak slope ≈2.275 → **≈2.5×10⁻⁵ in source-linear**. That is then amplified by the destination inverse gamma `(1/γ)·L^(1/γ−1)`, **which is unbounded as L → 0** — so **no finite uniform device-space tolerance exists over the whole cube**, and saying otherwise would be the dishonest part. Evaluated at *this grid's* darkest non-zero step (1/16 device → 4.03×10⁻³ linear → ×11.6) the envelope is 2.9×10⁻⁴; 5×10⁻⁴ is that rounded up. **A grid extended nearer black must RE-DERIVE this number, never re-tune it**, and the constant's `why` string says so. |
| **Class** | **`implementation-cross-check`**, arithmetic-agreement — **not perceptual**. DL-004's 1.0 ΔE2000 anchor is irrelevant to it and must not be cited in its support. |
| **Where** | `tools/difftest/README.md` §13.6.1; `TOLERANCES.md` §3.3.1 rows 1–2. |

#### 3.8.4 ★★ NC-038 — the round trip, and a tolerance whose derivation was CORRECTED after it failed

| Field | Value |
|---|---|
| **What was compared** | `sRGB → Adobe RGB → sRGB` through **two invocations of the shipped binary**, output against input, in ΔE2000. **Both sides are iccce.** |
| **Tolerance** | **2.5×10⁻² ΔE2000 max.** **Supersedes 1×10⁻²**, which stood for the length of exactly one run. |
| **Result** | **max 1.8788×10⁻², mean 8.674×10⁻⁴** (max device deviation 5.670×10⁻⁴). Observed, not merely bounded. |
| **Evidence class** | **`self-consistency`** — and it must be labelled so **however reassuring it looks**, because a wrong matrix round-trips perfectly. Its value is that it **prices** an approximation, not that it validates one. |
| **★★ Why the number moved, and why that is a CORRECTED DERIVATION rather than a widened tolerance** | The run **failed at 1.8788×10⁻²** against 1×10⁻². `TOLERANCES.md` §0's procedure was then followed **in order**, and step 4 was reached only because steps 1–3 were answered: **(1) is the code wrong?** No — the excess *is* the Annex F.8–F.16 clamp doing what the clause requires. **(2) is the expectation wrong?** There is no recorded expectation; both sides are computed in the run. **(3) is the fixture wrong?** **Yes, and this is where it was.** The original justification read *"sRGB and Adobe RGB share their red and blue primaries and Adobe's green is more saturated, so the sRGB triangle is strictly contained, no grid point is clipped."* **Every clause of that is true of the two colour SPACES and false of the two FILES.** **(4) only then, the tolerance.** |
| **★ The mechanism was PREDICTED, not asserted** | A matrix/TRC profile's media white **is** its colorant sum, and the two files' colorants were authored and rounded to `s15Fixed16` independently, 1998 and 2000 (**NC-042**). The white-corner cost was predicted **in closed form from the two matrices and the clamp alone** — no tone curve (every TRC here is exactly 1 at 1), no lcms2, no measurement: **1.878244×10⁻² predicted against 1.878818×10⁻² observed, agreeing to 0.03 %.** A justification that survives being computed independently is a different object from one that merely sounds right. |
| **How 2.5×10⁻² is then built** | 1.8782×10⁻² (the clamp of the encoded white mismatch, closed form) + ≈1×10⁻³ (1024-entry table interpolation, forward and inverse, `h²·max f''/8` with `h = 1/1023`, ×903.3 `dL*/dY`, two non-cancelling evaluations) ≈ 1.98×10⁻², **plus ~25 % headroom** because the closed form is evaluated **at the white corner only** and the other 24 clipped points were not separately predicted. |
| **★ The number is CORPUS-SPECIFIC and any restatement must carry the pair** | It is a property of *which two files* are being converted between, not a constant of the engine. **Two profiles with identical encoded whites would show ≈0 here.** A different pair re-derives it from its own colorant tags. |
| **Both justifications are preserved** | `TOLERANCES.md` §4 carries the change with the superseded reasoning quoted in full, per that document's append-only rule; `tools/difftest/README.md` §13.6.3 carries the sentence it replaced. **This ledger does not duplicate the tolerance's history** — that is §4's job there. What is recorded here is the **measured** number and its class. |
| **Where** | `tools/difftest/README.md` §13.6.3, §13.9; `TOLERANCES.md` §3.3.1 row 5 and §4. |
| **Invalidated by** | Either profile changing; any change to the clamp sites, `MatrixTrc`, or the curve stack; **the grid changing**; a re-derivation for any other profile pair (which produces a *new* row, not an edit to this one). |

#### 3.8.5 ★ NC-039 — the prediction pin: the check that stops NC-038's gate rewarding a deleted requirement

| Field | Value |
|---|---|
| **The problem it solves** | NC-038 is an **upper bound on a quantity that is mostly a deliberate cost.** Remove iccce's range clamping and the round trip gets **better** — the gate would go *greener* while a **normative requirement had been deleted**. A gate that rewards deleting the thing it is guarding is not a gate. |
| **What was compared** | \|predicted − observed\| at **device white**, where "predicted" is closed-form `f64` arithmetic on the two colorant matrices and the clamp, and "observed" crosses **two subprocess boundaries**. |
| **Tolerance / result** | **1×10⁻³**; observed **5.7392×10⁻⁶**. The bound is **10× the ≈1×10⁻⁴ ΔE00 floor** imposed by `iccce transform`'s 6-decimal device print on each leg (±5×10⁻⁷ per component × `dL*/d device ≈ 85` at white ÷ `S_L ≈ 1.75`). |
| **★ The sensitivity control — an apparatus not shown able to detect its effect is not an experiment** | With **no clamping at all** the round trip is the exact identity, the observation would be 0, and this record's metric would read **1.878×10⁻² — failing by 19×.** Printed by `pass3_report`. |
| **★★ SCOPE, and it is narrower than the check first looks — this is stated because a first draft claimed more** | A first draft claimed the check made the normative **F.8–F.16 *ordering*** falsifiable. **That claim was wrong and was corrected in place rather than deleted.** `iccce-cmm` clamps at **three independent sites**, each with its own citation: `MatrixTrc::pcs_to_device` (**F.8–F.16**, linear → [0,1] before TRC⁻¹); `Trc::eval` (**10.18**, curve input domain); `Trc::eval_inverse` / `invert_table` (**F.1(b)**, attainable range). So NC-039 catches **a wrong colorant matrix** and **clamping removed from all three sites**, and does **NOT** catch the F.8–F.16 clamp removed on its own — the other two make it redundant **at the shipped surface**. Distinguishing clamp-before from clamp-after needs a TRC whose inverse is defined outside [0,1], **which iccce never permits.** **OWED, NOT COVERED**, and `TOLERANCES.md` §3.3.3 carries it as a deliberately blank row. |
| **Relation to NC-027** | NC-027 asserts the *ordering* on measured output of a **synthetic** model, and remains the only assertion in the project that touches it. It is a unit test, not a differential; the ordering is still unobservable **through the binary**. |
| **Class** | **`self-consistency`**, and a new *shape* of self-consistency: a **prediction pin**. Its method generalisation is `ARCHITECTURE.md` **DL-018**. |
| **Where** | `tools/difftest/README.md` §13.6.4, §13.9; `TOLERANCES.md` §3.3.1 row 6 and §4. |

#### 3.8.6 ★ NC-041 — the quantisation experiment: the residual explained to a factor of 290

| Field | Value |
|---|---|
| **What was compared** | lcms2's **actual measured output** against **iccce's model with lcms2's own 16-bit tone-curve quantisation emulated inside it** — `linear = Q(TRC(Q(device)))`, `Q(v) = round(v·65535)/65535`. |
| **Result** | See the four-row table immediately below this one. |
| **What it establishes** | **The device-space residual shrinks by a factor of ~290, to below `transicc`'s own print floor of 1×10⁻⁴/255 = 3.92×10⁻⁷.** The iccce–lcms2 disagreement is accounted for, essentially completely, by **a named approximation in the ORACLE** — not by an unexplained margin absorbed into a tolerance. |
| **★ What it does NOT establish, and the experiment says so itself** | It grades an **emulation**, not the shipped binary. Two limits are stated so a partial collapse could not have been over-read: lcms2 interpolates its table **in 16-bit fixed point** (`LinLerp1D` + `_cmsQuickSaturateWord`) while the emulation interpolates in `f64` and rounds once; and lcms2 carries its pipeline in **`f32`**. A residual of a few lsb was the expected floor; **it came in below it**, which is a slightly *better* result than the model predicts and therefore not a place to stop asking questions. |
| **Class** | **`implementation-cross-check`**, on a **deliberately modified** iccce model. It is evidence about **where the disagreement comes from**, not about iccce's accuracy, and it may never be quoted as *"iccce agrees with lcms2 to 2.3×10⁻⁷."* |
| **Where** | `tools/difftest/src/bin/pass3_report.rs` §4; `tools/difftest/README.md` §13.6.1. |

**NC-041's measured residuals, against lcms2's actual output:**

| Residual against lcms2's measured output | max | mean |
|---|---|---|
| device (0..1), **iccce as shipped** | 6.705882×10⁻⁵ | 6.167183×10⁻⁶ |
| device (0..1), **with lcms2's 16-bit quantisation modelled** | **2.311449×10⁻⁷** | 1.448340×10⁻⁷ |
| ΔE2000, **iccce as shipped** | 3.476186×10⁻³ | 5.114460×10⁻⁴ |
| ΔE2000, **with quantisation modelled** | 8.412613×10⁻⁵ | 1.772019×10⁻⁵ |

#### 3.8.7 NC-042 — the two files' encoded media whites. A fact about **files**, in the shape of NC-031

| Field | Value |
|---|---|
| **What** | Each profile's media white **as encoded**, computed as the colorant sum `M·(1,1,1)` from the tags: **sRGB (0.96427917, 0.99996948, 0.82508850)**; **Adobe RGB (0.96420288, 1.00000000, 0.82490540)**; difference **(+7.629×10⁻⁵, −3.052×10⁻⁵, +1.831×10⁻⁴)** — **5, 2 and 12 units of `s15Fixed16`'s 1/65536 lsb**, accumulated over three colorant tags each. |
| **Consequence, and it is the whole of NC-038** | Source device white maps through `M_dst⁻¹` to **(1.00010586, 0.99987297, 1.00025354)** — *outside* the destination's encoded cube on **two** channels — and **25 of the 133 grid points clip somewhere**, all on the high-value faces. |
| **Class** | A **fact about two files**, reported from a run this librarian did not perform, in the same category as NC-031's 0.825089. **It is not a claim about iccce and not a claim that either file is wrong** — independent rounding by two authors two years apart is exactly what `s15Fixed16` does. |
| **Why it earns its own number** | Because NC-038's tolerance is **corpus-specific**, and a corpus-specific tolerance is only checkable if the corpus property it rests on is written down separately from it. Anyone re-deriving the round-trip bound for another pair starts here. |
| **Where** | `tools/difftest/README.md` §13.6.3 (the printed measurement); `TOLERANCES.md` §3.3.1 row 5. |

#### 3.8.8 ★ NC-043 — lcms2 emits device values above 1.0 on the analytic-inverse path. **A FINDING, not a failure, and not yet a verdict**

| Field | Value |
|---|---|
| **What was observed** | **8 of 399 output components (2.01 %)** came back from `transicc` **outside `[0,1]` — up to `1.000120`** — all on grid points whose maximum channel is 1.0. **iccce returns exactly `1.000000`** for the same inputs. |
| **★ The mechanism, and why it looks like an artefact rather than a policy** | Measured the same day in the **reverse** direction (Adobe RGB → sRGB, whose destination TRC inverse is a **tabulated** reverse curve rather than an analytic gamma), lcms2 **does saturate**: `0 1 0` → `0.000000 1.000000 0.000000`, no excursion. The behaviour therefore tracks **which inversion path lcms2 took** — `pow(1.000106, 1/γ)` is perfectly finite and nothing forces it back, whereas a reverse table has nothing to return outside its own range. |
| **Which implementation the specification supports — and why this row does NOT answer it** | **Annex F.8–F.16** is normative for the matrix/TRC PCS→device direction and clamps each linear component to [0,1] before the inverse TRC, which is what `matrix_trc.rs` implements and cites; on that reading iccce is right and lcms2 is permissive. **But** clause **6.4** requires out-of-range values to be clipped per component on **integer** conversion and requires **no clipping for float32 encodings** (NA-003), which may make lcms2's float excursion **conforming** and iccce's clamp merely **stricter**. **The two clauses must be read together, and nobody in this project has read them together yet.** |
| **Status** | **A recorded difference. NOT a verdict, in either direction** (rule 7: disagreement with lcms2 is a finding, settled from the specification text). |
| **★ The dispatch status, stated precisely because two sources disagree** | `tools/difftest/README.md` §13.10 item 1 records the question as **owed and NOT dispatched** — *"no Agent tool was available in the session that ran this"* *(verified — read)*. The dispatch that produced this filing reports the question **was** put to `icc-spec-librarian` in parallel. **Both can be true** (the README was written in the earlier session), and **whether the dispatch landed is `unverified` here**: the answer is not in the corpus file this librarian read. **Do not close this row on the strength of the dispatch having been sent.** |
| **How it is handled in the numbers meanwhile** | NC-034 compares against lcms2's output **clamped into [0,1]**, so it grades *arithmetic* disagreement; the **unclamped** maximum (1.200×10⁻⁴) and the excursion count are reported on the same record as a **separate quantity**. The ΔE rows are **structurally blind** to it, and that is correct rather than a gap: a device code outside [0,1] denotes no colour in that device space, so there is no colour difference to measure. **Stated in the record so nobody reads the ΔE silence as agreement.** |
| **Class** | **`oracle-behaviour-at-pin`** — it measures what lcms2 2.19.1 at `21c582a` does. **Invalidated by the pin moving**, like NC-019 … NC-021. |
| **Where** | `tools/difftest/README.md` §13.4, §13.10 item 1. |

#### 3.8.9 What §3.8 does **not** claim

- **Not that iccce is correct.** Four rows say two implementations read
  Annex F.3 the same way; two say iccce is self-consistent; one is a
  fact about the oracle; one about two files. **The shared-misreading
  case is live** (§1, `TOLERANCES.md` §1) and, uniquely here, its two
  sides share a sourcing origin.
- **Not that the matrix/TRC path is verified against published values.**
  It is not, anywhere. `TOLERANCES.md` §3.3.3 names this as **the largest
  evidential hole in Pass 3** and it is unfilled: IEC 61966-2-1's
  primaries and a D50-adapted matrix derived from them would supply the
  first such row, and **the corpus has not been asked**.
- **Not anything about the absolute intent**, which this Pass implements
  and which **no differential can reach** while the CLI refuses it.
- **Not anything about v4 profiles.** DL-013's forced-BPC behaviour was
  **avoided** here (both files are v2.1, below lcms2's version gate) —
  and the apparatus notes that *"escaping a trap by accident is not
  avoiding it"*; the intent is pinned at media-relative **by
  construction** because iccce implements nothing else, which is the
  reason that holds.
- **Not that the run was independently reproduced.** See §2.4's
  `pass=8` / `pass=7` discrepancy.

### 3.9 ★★ Pass 4 — the LUT differential. CMYK → RGB, a `Lab ` PCS, four intents, and the first row that PRICES a named approximation

**Read §2.5 before quoting anything here**, and read the shared coverage
box below before quoting any single row. Thirteen rows, **NC-044 …
NC-056**.

> **★ SHARED COVERAGE — part of every claim in this section, and it must
> travel with any row that is quoted.**
> **One profile pair**: `USWebCoatedSWOP.icc` (v2.1.0, `prtr`, CMYK →
> **`Lab `**, `A2B*` all `mft2`, 4-in/3-out, **9 CLUT points per axis**,
> 256-entry input tables, identity output tables and identity 3×3) → the
> Windows system `sRGB Color Space Profile.icm` (v2.1.0, `mntr`, RGB →
> XYZ). Both **category (c)** under `LEGAL.md` §3 — read locally, never
> committed, so **every row here skips** on a machine without the Windows
> colour directory. **341 deterministic CMYK points** (16 hypercube
> corners, a 9-step K ramp, a 9-step CMY composite neutral, a 9-step rich
> neutral, a 256-point 4-D lattice on {0, ⅓, ⅔, 1}, 64 fixed-seed
> pseudo-random interior points; no `rand`, no clock, count asserted by a
> unit test). **All four A2B intents.** **One platform**, **one lcms2
> build at one pin**.
> **NOT covered, stated because "verified" without scope is the failure
> this document exists to prevent:** **the B2A direction is not exercised
> at all** — SWOP's `B2A*` are `mft1` and this run's *destination* is
> matrix/TRC, sRGB having **no `B2A*` tag** — so **`lut8Type` evaluation
> and the `Lab8` codec have never been compared to anything**;
> **`mAB `/`mBA ` are decoded but not evaluated**; **no v4 profile
> anywhere**; no synthetic fixture; no BPC, no soft-proofing; **no
> ground-truth row of any kind — Pass 4 has none**. And two limits of the
> grid itself: it reaches **400 % total ink** where real SWOP separations
> rarely exceed ~300 %, so **the mean over this grid is not the mean over
> printable colour**; and nothing below 1/8 in a single channel except
> through the random block.
> **★ And one that is easy to miss:** SWOP's **`A2B0` and `A2B2` are one
> block of tag data** (both offset 432, size 41478 — the Pass 0 finding).
> Perceptual and saturation are therefore **the same bytes through the
> same code**. *"All four intents"* is honest about what ran; **the
> number of distinct A2B tables exercised is three, not four.**

#### 3.9.0 The design that makes this section readable — two kinds of gate, and only one of them may claim agreement

Pass 3's whole disagreement with lcms2 was the oracle's 16-bit rounding —
a *defect of precision*, so one tight tolerance was both derivable and
meaningful. **Pass 4's dominant term is an interpolation-method
difference between two schemes ICC.1 does not choose between.** It is not
an error in either implementation, it is ~1.6 ΔE2000, and it will not go
away. NA-006 named the trap in advance: *"a tolerance wide enough to
swallow ~1 ΔE cannot also demonstrate agreement."*

A single number cannot both admit the method difference and show that the
two `lut16` pipelines agree, so **this section carries two kinds of row
and labels which is which**:

| | Rows | What it is for |
|---|---|---|
| **Structural, wide** — the gate's value *is* the method envelope | NC-048, NC-049 | Catches a wrong index order, a wrong Lab decode, a swapped ink. **Cannot claim agreement**, and its own record text says so |
| **Arithmetic, tight** — the method difference switched off | **NC-044, NC-045** (interpolation-free corners), **NC-046, NC-047** (lcms2's geometry emulated) | **This is where the agreement claim lives.** 55–2000× tighter |

**Anyone quoting a Pass 4 number must say which of the two they are
quoting.** The 2.0 gate is not evidence of agreement and the 1×10⁻³ gate
is not evidence about interpolated colour.

#### 3.9.1 The thirteen rows, at a glance

| ID | Record | Class | Metric | Tolerance | **Observed** |
|---|---|---|---|---|---|
| **★ NC-044** | `pass4/swop-to-srgb/media-relative/pcs-lab-corners-interpolation-free` | **implementation-cross-check** | ΔE2000 max, 16 CLUT-node corners | **1×10⁻³** | **5.9131×10⁻⁵** |
| **★ NC-045** | `pass4/swop-to-srgb/perceptual/pcs-lab-corners-interpolation-free` | **implementation-cross-check** | ΔE2000 max, same 16 corners | **1×10⁻³** | **6.6558×10⁻⁵** |
| **★ NC-046** | `pass4/swop-to-srgb/media-relative/pcs-lab-emulated-geometry` | **implementation-cross-check** | ΔE2000 max, PCS Lab | **2×10⁻²** | **4.5931×10⁻³** |
| **★ NC-047** | `pass4/swop-to-srgb/perceptual/pcs-lab-emulated-geometry` | **implementation-cross-check** | ΔE2000 max, PCS Lab | **2×10⁻²** | **4.8154×10⁻³** |
| NC-048 | `pass4/swop-to-srgb/media-relative/{de2000,pcs-lab,device}-vs-lcms2` | implementation-cross-check (**structural**) | ΔE2000 max / ΔE2000 max / device abs-max | **2.0** / **2.0** / **2×10⁻²** | **0.252 94** / 0.254 65 / **3.0045×10⁻³** |
| NC-049 | `pass4/swop-to-srgb/perceptual/{de2000,pcs-lab,device}-vs-lcms2` | implementation-cross-check (**structural**) | same | **2.0** / **2.0** / **2×10⁻²** | **1.6590** / 1.5715 / **1.0816×10⁻²** |
| **★★ NC-050** | the interpolation-method envelope (`pass4_report` §§ envelope) | **self-consistency** (**NA-006's price**) | ΔE2000 max/mean, two algorithms on one CLUT | **not gated** — it *is* the budget the gates are built from | **1.5741** / 0.043 86 (`A2B0`) · **0.254 23** / 0.038 54 (`A2B1`) |
| NC-051 | `pass4/apparatus/harness-nlinear-matches-iccce-cmm` | self-consistency (**apparatus check**) | max abs in `L*`/`a*`/`b*` units | **1×10⁻⁹** | **0.0 exactly** |
| NC-052 | `pass4/swop/perceptual-equals-saturation` | **implementation-cross-check** (an **identity**, graded **on both sides**) | max abs device-output difference between the two intents | **0.0 exact** | **0.0** |
| **★★ NC-053** | `pass4/swop-to-srgb/icc-absolute/{de2000,device}-vs-lcms2` | **oracle-behaviour-at-pin** (**a FINDING**) | ΔE2000 max / mean | **∞ — REPORTED, NOT GRADED** | **11.217** / 4.670 (device 0.157 96 / 0.0485) |
| **★ NC-054** | `pass4/swop-to-srgb/icc-absolute/white-point-policy-emulated` | **implementation-cross-check** | ΔE2000 max, lcms2's `wtpt` policy modelled | **5×10⁻²** | **2.1677×10⁻²** (**517× collapse**) |
| NC-055 | `…/<intent>/device-mean`, `…/de2000-mean` | implementation-cross-check | means | **∞ — reported, NOT graded** | ΔE00 4.3126×10⁻² (perc/sat) · 4.0107×10⁻² (m-rel); device 4.6257×10⁻⁴ · 4.1870×10⁻⁴ |
| **★ NC-056** | lcms2's 4-D CLUT interpolation, **read at the pin** | **oracle-behaviour-at-pin** | — (a mechanism, with the attribution that measures it) | — | linear-in-C × **Sakamoto tetrahedral** in MYK; **the advance prediction was "tetrahedral" and was WRONG** |

**The `∞` rows pass because there is nothing for them to fail**, exactly
as in §3.8 — but **NC-053's `∞` is a different object from NC-055's**.
NC-055's means are ungraded because *a mean over a grid hides the outlier
a colour engine gets wrong*. **NC-053's is ungraded because the project
does not yet know which implementation is right**, and that distinction
is the entire content of §3.9.5 and of **DL-019**.

#### 3.9.2 ★★ NC-044 / NC-045 — the interpolation-free corners. **The strongest cross-check evidence this project has produced**

| Field | Value |
|---|---|
| **What was compared** | The shipped `iccce transform` binary against `transicc -o*Lab4`, restricted to the **16 hypercube corners** of the CMYK grid — paper, 100 % K, the four single inks, process red, 400 % total ink |
| **★ Why those 16 points are categorically different** | Each `mft2` input table starts at `0x0000` and ends at `0xFFFF`, so device 0 and 1 map to **node 0 and node 8 exactly**. There, **both implementations evaluate the CLUT at an exact node**: n-linear and lcms2's hybrid geometry return the *same stored value*, identically — the harness prints the method envelope restricted to the corners and it is **0.0 exactly, as it must be**. And lcms2's quantisation terms **vanish rather than accumulate**: the CLUT input is an exact `u16`, the interpolated value *is* the stored `u16`, the output tables are the identity |
| **Tolerance** | **1×10⁻³ ΔE2000 max — ten times the ≈1×10⁻⁴ ΔE00 floor** imposed by `transicc`'s 4-decimal Lab print, which is all that is left at a node. **The tightest gate in Pass 4** |
| **Result** | **5.9131×10⁻⁵** (media-relative, mean 2.8954×10⁻⁵) and **6.6558×10⁻⁵** (perceptual/saturation, mean 2.8820×10⁻⁵) — **observed maxima, exactly the print floor, and 70× below the same comparison taken between nodes** |
| **★ Why this is the strongest row in the project** | Every previous cross-check compared two implementations **through** an approximation neither is obliged to share. This one removes the approximation by construction rather than by modelling it: **at a CLUT node there is nothing left to disagree about except the arithmetic of the surrounding pipeline** — input tables, the tag-type Lab decode, the node indexing, the channel order. It is the row that makes the 2.0 structural gates defensible: **without a node-only control, a wide structural gate could hide a genuine 1.9 ΔE error.** It would catch the v2/v4 Lab encoding confusion (≈0.39 `L*` at white, far worse in `a*`/`b*`), a swapped ink, or an off-by-one in the node index — **all ≥1000× this bound** |
| **Class** | **`implementation-cross-check`.** Per §1 and rule 7 this remains evidence that two implementations read 10.10 and 6.3.4.2 NOTE 3 the same way — **not ground truth.** Pass 4 has no ground-truth row at all |
| **Where** | `tools/difftest/README.md` §14.5.4, §14.7; `tools/difftest/src/pass4.rs` (`DE_PCS_CORNERS` and its `why`); `TOLERANCES.md` §3.4 |
| **Invalidated by** | The lcms2 pin moving (**re-run, do not re-read**); either profile changing on this machine; any change to `lut_transform.rs`, `clut.rs`, `pcs_encoding.rs` or the CLI's print precision; **any change to the grid's corner block**, which is asserted by `corner_indices_really_are_corners` |

#### 3.9.3 ★ NC-046 / NC-047 — lcms2's own geometry substituted. **Where the agreement claim actually lives**

| Field | Value |
|---|---|
| **What was compared** | lcms2's **actual measured PCS output** against **iccce's `mft2` pipeline re-run with lcms2's `Eval4Inputs` geometry** in place of n-linear — the whole rest of the pipeline (input tables, output tables, legacy Lab decode) held identical. The Pass 3 discipline exactly: *predict the confound from the other implementation's own arithmetic, then measure what is left* |
| **Tolerance** | **2×10⁻² ΔE2000 max**, built from the oracle's residual quantisation: tabulated input curves rounded to 1/65535 in and out, the CLUT **stage input rounded to `u16`**, `Eval4Inputs` evaluated in **s15.16 fixed point**, and `transicc`'s 4-decimal Lab print. **One 16-bit lsb of CLUT output is 1.53×10⁻³ in `L*` and 3.9×10⁻³ in `a*`/`b*`** under the legacy decode this tag type mandates (652.8 and 256 codes per unit) — **the `a*`/`b*` scale means a single lsb is not negligible there**, which is the sort of thing that is obvious only after somebody computes it |
| **Result** | **4.5931×10⁻³** media-relative (mean 1.2988×10⁻³) and **4.8154×10⁻³** perceptual/saturation (mean 1.1091×10⁻³) — a **55× / 326× shrink** on the maxima against the raw comparison |
| **★ The single point that shows what was substituted** | At the worst method-envelope point at media-relative, CMYK (0.949 78, 0.693 67, 0.950 21, 0.947 48): iccce's n-linear gives Lab (14.2965, −3.2319, 1.6226); the emulated lcms2 geometry gives (14.3933, −3.4322, 1.6197); **`transicc` itself gives (14.3934, −3.4297, 1.6211)**. The emulation lands on the oracle to 1×10⁻⁴ in `L*` while the shipped n-linear sits **0.2 away in `a*`. The disagreement IS the geometry** |
| **Class** | **`implementation-cross-check`, on a deliberately modified iccce model** — the same qualification NC-041 carries. It is evidence about **where the disagreement comes from** and about the **rest** of the pipeline's arithmetic; **it may never be quoted as "iccce agrees with lcms2 to 4.8×10⁻³"** without the words *"with lcms2's own interpolation geometry substituted"* |
| **Where** | `tools/difftest/README.md` §14.5.3, §14.7; `pass4.rs` `DE_PCS_EMULATED` |

#### 3.9.4 NC-048 / NC-049 / NC-050 — the raw comparison, and the envelope that explains it

| Field | Value |
|---|---|
| **NC-048 / NC-049 — what they are** | The **raw** end-to-end and PCS comparisons against `transicc`, ungated in any meaningful sense: **the tolerance's value is the method envelope plus 20–27 % headroom** (2.0 ΔE2000 for the ΔE gates, 2×10⁻² device from the larger propagated envelope +86 %). Observed: media-relative **0.252 94** end to end, 0.254 65 in PCS, 3.0045×10⁻³ device; perceptual/saturation **1.6590**, 1.5715, 1.0816×10⁻² |
| **★ What they explicitly cannot claim** | **Agreement.** The record's own `why` string says the tolerance is *above* the provisional 1.0 ΔE00 perceptibility anchor (DL-004) and therefore cannot be read as a perceptual guarantee. They are **structural** gates: they catch a wrong index order, a wrong Lab decode, a swapped ink — defects that would move these numbers by ≥1000× |
| **★★ NC-050 — the envelope itself, and it is the number NA-006 has been owed since it was filed** | `SourcePipeline` reimplements the entire `mft2` A2B path **twice**, differing in **exactly one component**: iccce's n-linear CLUT, or lcms2's `Eval4Inputs` geometry transcribed from `cmsintrp.c`. **No lcms2 output enters this quantity** — it is computed from the CLUT and the two algorithms alone. **`A2B0` (perceptual/saturation): max 1.5741, mean 0.043 86. `A2B1` (media-relative): max 0.254 23, mean 0.038 54.** Propagated end to end: **1.6639 ΔE00** and 1.0751×10⁻² device (`A2B0`), 0.254 23 and 2.9012×10⁻³ (`A2B1`) |
| **★ The factor of six, and why it is an argument about method rather than a curiosity** | The perceptual table's worst cell is CMYK (0.541, 0.442, 0.744, 0.972) — **deep shadow at near-full black, where the CLUT turns sharply and the two schemes take different routes across the same cell.** The colorimetric table is six times smoother. **A Pass 4 tolerance derived from `A2B1` alone would have been wrong by 6× for precisely the intents Pass 3 never exercised.** Nothing about a smooth colorimetric result predicts a rough perceptual one, and the only way to know was to run all four |
| **NC-050's class** | **`self-consistency`** — it *prices* an approximation, which §1 names as the only thing self-consistency is good for, and neither arm is an oracle output. **But it carries one `oracle-behaviour-at-pin` liability**: the second arm is a **transcription of lcms2's geometry at commit `21c582a`**, so **the pin moving invalidates it too**, and a transcription is a source reading until something measures it. Here something does: NC-046/NC-047's 326× collapse is that measurement |
| **NC-051 — the apparatus, graded before anything was concluded from it** | The n-linear arm is held against `iccce_cmm::lut_transform::Lut16Model` on **every grid point at every intent**, tolerance 10⁻⁹ in `L*`/`a*`/`b*`: **observed 0.0 exactly — bit-identical.** Without this row the whole substitution experiment would rest on an *assertion* that the reimplementation is faithful to the code it is standing in for. **An apparatus that has not been shown to reproduce the thing it replaces is not an apparatus** |
| **NC-052 — the shared-tag identity** | `A2B0` and `A2B2` are one block of tag data in this file, so perceptual and saturation are the same bytes through the same code. **Tolerance `0.0` exact; observed `0.0` on both sides.** A small epsilon here would admit exactly the class of bug — an 8.10.2 tag-selection defect — that the record exists to catch, and there is no arithmetic that could produce a *small* one. A unit test in `transform.rs` makes the same assertion on the real files with `assert_eq!` *(verified — read)* |
| **Where** | `tools/difftest/README.md` §14.5.2, §14.5.4, §14.5.5, §14.7 |

#### 3.9.5 ★★ NC-053 / NC-054 — at the ICC-absolute intent the two implementations read **different destination media whites**, and it costs 11 ΔE2000

| Field | Value |
|---|---|
| **What was observed** | At `-t3`: **max 11.217 ΔE2000, mean 4.670** (device max 0.157 96, mean 0.0485) — **two orders of magnitude more than at any other intent**, and far beyond anything the interpolation envelope for `A2B1` (0.2542, the table absolute uses) could account for. **The worst points are the lightest**: paper (0,0,0,0) at 10.6, 33 % C at 11.2 |
| **★ The mechanism, read at the pin and then measured** | `cmsio1.c`'s `_cmsReadMediaWhitePoint` **substitutes D50 for the stored `wtpt`** when a profile is **v2 AND display-class** (`cmsGetEncodedICCversion < 0x4000000 && deviceClass == cmsSigDisplayClass`). The destination sRGB profile's `wtpt` as stored is **(0.950 455, 1.000 000, 1.089 050) — D65**, while its colorants are D50-adapted: a common v2-era encoding. **Both implementations build the same D.6/D.7 diagonal; they differ in what they read for `WhitePointOut`** — iccce takes `wtpt` **as stored** (**NA-007**), lcms2 takes **D50**. The ratio is D65/D50 = (0.9858, 1.0, 1.3202): **a 32 % error in `Z`, applied to every colour.** That is the 11 ΔE |
| **The measurement that confirms the attribution** | Re-predicting lcms2's output with **that one substitution** (plus the CLUT geometry, so both known differences are modelled) gives **max 2.1677×10⁻², mean 3.4034×10⁻³ — a 517× / 1372× collapse.** That is **NC-054**, and it is graded at **5×10⁻²** |
| **★ Which implementation is right — NOT settled, and that is the finding** | ICC.1:2022 specifies v4 profiles. **What a v2 profile's `wtpt` means is corpus ambiguity A4b, and A4b is UNVERIFIED** — ICC.1:2022 is *silent about version 2's convention*, confirmed there by full-text search, and **ICC.1:2001-04 has not been obtained** *(verified — `icc__ref__ambiguity_register.md`, `icc__s__rendering_intents.md` §A4b and `icc__ref__v2_v4_divergence.md` read 2026-08-11)*. **lcms2's substitution is justified in its source by a comment, not by a clause.** Per rule 7 this is **a recorded difference, not a verdict in either direction** |
| **★★ How the numbers handle it, and why this pattern is filed as DL-019** | The **two raw comparisons are REPORTED, NOT GRADED** (tolerance ∞) and the **gate at that intent is NC-054**, the modelled quantity. **Both rejected alternatives are written down** rather than one being chosen silently: *widen to ~15 ΔE00 so it passes* — a number chosen because it passed, 15 ΔE00 being a different colour, and it would **silently absorb any future arithmetic error in the absolute path**; *let it fail permanently* — **a red line that never changes stops being read**, and it reports the disagreement as unexplained when it is not. **This is the only place in the suite where a known disagreement is deliberately not gated**, and it is labelled rather than absorbed |
| **The dispatch status, stated precisely** | `tools/difftest/README.md` §14.9 item 1 records the A4b dispatch as **owed**; the dispatch that produced this filing reports it **sent in parallel**, together with corpus rows **M4/M5**. **Whether it has landed is `unverified` here**, and what *is* checkable says not yet: as of this filing `icc__ref__lcms2_measured_behaviour.md` carries **M1, M2, M3 and no M4 or M5**, and A4b is still listed **UNVERIFIED** *(verified — enumerated)*. **Do not close this row on the strength of the dispatch having been sent** |
| **Classes** | **NC-053: `oracle-behaviour-at-pin`** — its content is what lcms2 2.19.1 at `21c582a` does with a v2 display-class `wtpt`. **NC-054: `implementation-cross-check`, on a deliberately modified model**, like NC-046/NC-047 |
| **What settling A4b would do** | **One of the two implementations acquires a defect**, and this becomes a graded row again. If `wtpt` in a v2 display profile means the *adapted* PCS white, lcms2 is correcting a widely-mis-authored field and **iccce is wrong**; if it means the *measured, unadapted* device white, lcms2 is substituting its own guess for the file's data and **iccce is right**. Nothing in this ledger prefers either outcome |
| **Where** | `tools/difftest/README.md` §14.6, §14.7, §14.9; `pass4.rs`'s absolute-intent constants and their `why` strings |

> **★★ DATED CORRECTION, 2026-08-12 (the Pass 4c filing) — the "What
> settling A4b would do" row above is a PREDICTION, and it is FALSIFIED
> BY THE EVENT IT NAMED. The row is left standing; this note is the
> correction, per §3.9.6's convention.**
>
> **What the row predicted:** *"One of the two implementations acquires a
> defect, and this becomes a graded row again."* It then set out the two
> branches — if `wtpt` in a v2 display profile means the *adapted* PCS
> white then **iccce is wrong**; if it means the *measured, unadapted*
> device white then **lcms2 is right to be overridden** — and closed with
> *"nothing in this ledger prefers either outcome."*
>
> **What happened: A4b settled, and NEITHER implementation acquired a
> defect.** The prediction assumed the settling clause would *adjudicate
> between readers*. **It does not bind readers at all.**
> `ICC.1:2001-04` **A.3.1.1** addresses the profile's **author** and
> gates its recommendation on the **adaptation condition**;
> `ICC.1:2022` **9.2.36** gates on **device class with no version gate**.
> **lcms2's `version < 0x4000000 && class == 'mntr'` predicate reproduces
> no clause in either edition** — and that does not make lcms2 wrong
> either, because **the conformance clause binds READING profiles, not a
> CMM's computed output.** Say lcms2 **diverges**, never
> *"non-conforming"*. Full reasoning: **§3.15.6** and
> **`ARCHITECTURE.md` DL-026**.
>
> **So the row's third sentence is the one that survived and the first is
> the one that failed.** *"Nothing in this ledger prefers either
> outcome"* is now **permanently** true rather than provisionally true,
> and NC-053 becomes the **A16 / NC-056 pattern** — a **difference**
> between two named choices inside a specification silence, not a
> pending adjudication.
>
> **★ This is the project's THIRD falsified prediction, and the treatment
> is identical to the first two** (§3.9.6): **DL-011 → DL-012** (a
> predicted lcms2 disagreement over the legacy-Lab selector, measured
> **absent**) and **NA-006's *"tetrahedral"*** (a mechanism carried in
> three documents, falsified by reading the oracle). **Each time, the
> text stayed where it was written and a dated note named the evidence.**
>
> **★ And the shape of THIS one is new, which is why it is worth the
> space.** DL-012 falsified a prediction about **an implementation**;
> NA-006 falsified one about **an algorithm**. This one falsified a
> prediction about **what a document would do to the project's own
> record** — *"settling A4b will make this gradeable"*. **A prediction
> that a future source will resolve a disagreement is a claim about a
> document nobody has read, and this project has now made that exact
> claim wrong twice**: `NEXT_SESSION.md`'s operator table already warns
> *"treat 'it would settle X' as a prediction until the document is
> open"*, citing `ICC.1:2001-04` as the worked example of a document
> expected to adjudicate an 11 ΔE divergence that turned out to be
> **silent**. **The same document has now done it twice, one level up.**

#### 3.9.6 ★ NC-056 — lcms2's four-input CLUT interpolation, read at the pin. **And a prediction, filed in three documents, that reading it falsified**

| Field | Value |
|---|---|
| **What was read** | `cmsintrp.c`'s `DefaultInterpolatorsFactory`: for **3** inputs lcms2 selects tetrahedral; for **4** inputs it selects `Eval4Inputs` / `Eval4InputsFloat`, whose own verbatim comment reads *"For more that 3 inputs (i.e., CMYK) evaluate two 3-dimensional interpolations and then linearly interpolate between them."* So the four-input scheme is a **hybrid**: **linear along input channel 0 (C), Sakamoto tetrahedral in M, Y, K**, the two 3-D results blended by the first channel's fraction |
| **★★ The prediction it falsified** | **NA-006, `NEXT_SESSION.md` and `ROADMAP.md` all carried *"iccce interpolates n-linear, lcms2 tetrahedral"***, and the Pass 4 blocker was recorded as *"source lcms2's tetrahedral cube decomposition"*. **For four inputs that is not what lcms2 does.** Three consequences, none of which *"tetrahedral"* would have implied: (1) **lcms2's scheme is not symmetric in the four inks** — reordering the channels changes its answer, while iccce's quadrilinear **is** symmetric; (2) **it is not pure tetrahedral**, so **a bound transcribed from the trilinear-vs-tetrahedral literature is not the bound that applies** — and NA-006's ~1 ΔE was exactly such a bound; (3) **the float path does not use the float interpolator** — an `mft2` tag is read into a **16-bit** CLUT stage whose float evaluator quantises the stage input to `u16` and calls the fixed-point twin, so lcms2's CMYK pipeline in `transicc`'s default float mode carries 16-bit quantisation **at the CLUT boundary as well as** inside the tone curves |
| **How it was settled** | **By reading the oracle's source at the pin, not by recalling it** (rule 2 applied to a claim *about an implementation*, which is the case people forget rule 2 covers). And then by **measuring**: the emulation built from that reading collapses the residual **326×** (NC-046/NC-047), which is what makes the reading a finding rather than a paraphrase |
| **Class** | **`oracle-behaviour-at-pin`** — it establishes what iccce is compared against and nothing else. **It is not evidence that iccce is correct** (iccce did not participate) **and not evidence that lcms2 is correct** (ICC.1 says nothing about CLUT interpolation — ambiguity **A16**, SILENT, which is why iccce's n-linear is a *named choice* and a disagreement here is a **difference**, not an error on either side) |
| **Corpus status** | **Owed as corpus row M4** and **not present as of this filing** *(verified — `icc__ref__lcms2_measured_behaviour.md` carries M1, M2, M3 only)*. Until it is, this row is the project's only record of the reading |
| **Invalidated by** | **The pin moving.** `cmsintrp.c`'s factory is exactly the kind of thing a minor release changes |

**★ Where this project puts a falsified prediction, stated once so the
next one lands in the same place.** A prediction that is contradicted by
evidence is **left standing where it was written** and corrected by a
**dated note that names the evidence** — never edited away. This is the
project's **second** instance and the treatment is identical to the
first: DL-011 predicted a live disagreement with lcms2 over the
legacy-Lab selector, DL-012 measured it **absent**, and DL-011's text was
superseded rather than rewritten. Here: NA-006's *"tetrahedral"* wording
is corrected by the dated note in §4 and by this row; `NEXT_SESSION.md`
was rewritten (it is the one document in this set that is *not*
append-only, by its own header); and the ROADMAP's Pass 4 progress block
records the correction next to what was expected. **The failure mode
being guarded against is a prediction quietly becoming a citation** —
which is what *"lcms2 uses tetrahedral"* was three documents away from
being.

#### 3.9.7 What §3.9 does **not** claim

- **Not that iccce is correct.** Every row is a cross-check, a
  self-consistency check, or a measurement of the oracle. **Pass 4 has no
  ground-truth row at all**, and the most tractable candidate — a
  synthetic `mft2` whose CLUT stores an **affine** function, where every
  interpolation scheme must agree exactly and the expectation is
  therefore arithmetic rather than an oracle's opinion — is now
  buildable, because `tools/gen-profiles` exists.
- **Not that the B2A direction works.** `b3f4388` landed it; **nothing
  has measured it.** `lut8Type` evaluation and the `Lab8` codec have
  never been compared to anything.
- **Not anything about `mAB `/`mBA `.** Decoded since Pass 2 batch 2
  *(verified — `tag_types.rs` dispatches `sig::MAB`/`sig::MBA` to
  `lut::decode_lut_ab`)*, **evaluated by nothing**.
- **Not anything about v4 profiles.** DL-013's forced BPC was **proved
  unreachable** for this pair from the parsed version words, which is
  stronger than Pass 3's accidental escape — but proving a confound
  absent is not exercising the case it would have confounded.
- **Not that 1.659 ΔE2000 is iccce's error.** It is the **difference
  between two schemes ICC.1 does not choose between**, priced by NC-050
  and attributed by NC-046/NC-047. Neither implementation is wrong there
  on any evidence this project holds.
- **Not that the absolute intent is verified in any direction.** See
  NC-053.

#### 3.9.8 ★ The run count, reconciled against the code — and a NEW off-by-one, in the prose rather than in the arithmetic

§2.4 recorded the Pass 3 `pass=8` / `pass=7` discrepancy **unresolved**,
with the structural reading *"1 registered check + 7 `pass3` records"* as
a hypothesis. Pass 4's suite total lets that be checked, and this filing
counted the record emitters in the live source rather than trusting
either README *(verified — `pass3.rs`'s **seven** distinct `"pass3/…"`
ids, pinned by a unit test that asserts the same seven; `pass4.rs`'s
emitter loop read in full)*.

**What the code emits, per intent** (`pass4::records`): four records
always — `device-vs-lcms2`, `device-mean`, `de2000-vs-lcms2`,
`de2000-mean` — then a branch. When `lcms2_pcs` is `Some` (the three
non-absolute intents) three more: `pcs-lab-vs-lcms2`,
`pcs-lab-emulated-geometry`, `pcs-lab-corners-interpolation-free`. At
**`icc-absolute`** those three are pushed as **`Record::skipped`** with
the reason *"not comparable at this intent"*, and
`white-point-policy-emulated` is pushed **in their place**. Plus two
whole-run records (`apparatus/harness-nlinear-matches-iccce-cmm`,
`swop/perceptual-equals-saturation`).

| | records emitted | of which graded | of which skipped |
|---|---|---|---|
| smoke (`main.rs::checks()`) | 1 | 1 | 0 |
| Pass 3 (`pass3.rs`) | **7** | 7 | 0 |
| Pass 4 (`pass4.rs`) | 2 + 4×4 + 3×3 + (1 + 3) = **31** | **28** | **3** |
| **total** | **39** | **36** | **3** |

**36 and 3 are exactly the reported summary** *(`pass=36 fail=0 skip=3
error=0`, reported)*, which is the first time this project's record
counts have been reconciled to the emitters at all. **Two consequences,
and they point in opposite directions:**

1. **§2.4's hypothesis is confirmed structurally.** `pass3.rs` emits
   **seven**, so §13.9's `pass=8` is 7 + the smoke check. The engineer's
   re-run reporting `pass=7` remains unexplained and is **still not an
   independent re-verification of eight lines**, because no per-line
   output was ever recorded. **A count is not an inventory**, and this is
   the second filing to say so about the same summary line.
2. **★ `tools/difftest/README.md` §14.7's prose decomposition is wrong,
   in both terms, while its total is right.** It reads *"8 Pass 3
   records, 1 smoke, 27 graded Pass 4 records + 3 … skips"* and *"adds 30
   Pass 4 records"*. The code says **7**, 1, **28** and **31**. The two
   errors cancel in the total, which is exactly why nobody would notice:
   **a sum that comes out right is not evidence that its terms are
   right.** **Reported, not repaired** — §14 is `icc-conformance`'s file
   and this ledger does not edit it. Nothing measured moves; what moves
   is whether the sentence can be quoted for *which* checks ran.

### 3.10 ★ Pass 4 — the evaluation surface: `mAB `/`mBA `, grayTRC F.2, and the project's FIRST claim through bytes it authored itself

**Read §2.6 before quoting anything here** — in particular its second
row: **no test-run report accompanied the dispatch that produced this
section.** Five rows, **NC-057 … NC-061**, plus two findings and one
coverage observation that are **deliberately not given NC numbers**.

> **★ SHARED COVERAGE — part of every claim in this section.**
> **NC-057** is one **point** (`Lab(50, 0, 0)`) through one tag (`B2A0`,
> a `mBA `) of one **synthetic** profile this project authored
> (`fixtures/synthetic/v4-cmyk-mab-lab.icc`, category (a) under
> `LEGAL.md` §3 — committable, and **reported** committed).
> **NC-058 … NC-059** are synthetic arithmetic on hand-built `LutAB`
> structures, in-process, never through a file.
> **NC-060** reads one real EIZO profile from the Windows colour
> directory (category (c)) and **skips silently when it is absent**.
> **NC-061** is pure synthetic arithmetic.
> **NOT covered, and this is the sentence that matters**: **no B2A
> differential exists** — NC-057 is a *single recorded value*, not a run
> over a grid; **`mAB ` has never been evaluated against a real file**;
> **no gray transform has ever been compared to another
> implementation**; **nothing here traverses `transform::Chain`**, which
> is wired for both new models and exercised by no test; and there is
> still **no ground-truth row anywhere in Pass 4**.

#### 3.10.1 The five rows, at a glance

| ID | What | Class | Tolerance | Result |
|---|---|---|---|---|
| **★ NC-057** | `mBA ` evaluation of the committed fixture's `B2A0`: `Lab(50, 0, 0)` → CMYK, `K` against **`transicc`'s recorded 0.496117** | **implementation-cross-check** | **1×10⁻³** (justified in the test) | **asserted; the run was not reported** — see §2.6 |
| NC-058 | the `mAB ` 3×4 matrix's **offset terms arrive**: same matrix with and without `e03 = 0.25`, difference in `X` against the exact `0.25 × 65535/32768` | arithmetic-identity (**a trap's regression**) | **1×10⁻⁹** | asserted; not reported |
| NC-059 | `mAB ` full pipeline (A → CLUT → M → B, identity elements) decodes as **v4** Lab: `L* = n × 100`, `a*/b* = n × 255 − 128`; and device white → `L* = 100, a* = b* = 0` | arithmetic-identity | **1×10⁻⁹**, and **exact** (`assert_eq!`) for the white | asserted; not reported |
| **★ NC-060** | **grayTRC F.2 on a real profile** (`ewgray22.icm`): device white → the **full D50 triple** in X, Y **and** Z; plus a 5-point device round trip | normative-rule-conformance (the white) + **self-consistency** (the round trip) | **1×10⁻³** per component; **2×10⁻³** round trip | asserted; not reported; **skips silently** when the profile is absent |
| NC-061 | synthetic gamma-2.2 gray, XYZ PCS: forward at 0.5 equals `0.5^2.2 × D50` in X and Z, and the round trip returns 0.5 | arithmetic-identity | **1×10⁻¹²** | asserted; not reported |

#### 3.10.2 ★ NC-057 — the first claim this project has ever made through bytes it wrote itself

| Field | Value |
|---|---|
| **What was compared** | `iccce_cmm::lut_ab::LutAbModel::from_mba(...).pcs_to_device(Lab(50, 0, 0))`, on the `B2A0` tag decoded out of `fixtures/synthetic/v4-cmyk-mab-lab.icc`, against **`transicc`'s recorded conversion of the same input through the same tag of the same file**: CMYK(0, 0, 0, **49.6117 %**) |
| **Corpus and coverage — part of the claim** | **One point. One tag. One file.** `Lab(50, 0, 0)` is a mid-neutral; nothing off the neutral axis, nothing near either end of `L*`, no second intent, no grid. **This is a cross-check *point*, not a differential**, and the difference is the whole reason Pass 4's B2A clause is still unmet |
| **Tolerance** | **1×10⁻³** in normalised device units, **justified in the test's own doc comment** *(verified — read)*: `transicc` prints 4 decimals of percent (≈1×10⁻⁶ in 0..1), but its pipeline **quantises to `u16`** (≈1.5×10⁻⁵) and its **ragged-grid interpolation differs from n-linear away from nodes** (the fixture's CLUT is `5×4×3×2`, so this probe is emphatically *not* at a node). 1×10⁻³ admits those three and **still refuses a wrong curve count** — GP-001's symptom was a **refusal**, and a swapped count moves `K` by whole percent |
| **Why the tolerance is not tuned** | It is derived from **named mechanisms with magnitudes**, each smaller than the bound, and it is stated **against the failure mode it must catch** — the same shape as NC-031's re-justification. It is **not** the smallest number that passed, because nobody here has been told what the observed residual is |
| **Result** | **The assertion and its bound were read in the live source. No run was reported with this dispatch**, so per §1.1 this row records an asserted bound and **not** a pass |
| **Class** | **`implementation-cross-check`** — and a *narrow* one: both sides read **identical synthetic bytes**, so agreement bounds the two **evaluators**, not the specification. The test's own comment says exactly this. It is **not** ground truth; Pass 4 still has none |
| **★ What it is nevertheless worth** | It is the **only** number in this project that touches the B2A direction at all, and it is the regression that keeps GP-001 fixed. A future re-introduction of the `mAB ` curve convention on `mBA ` does not produce a small error here — it produces a **decode refusal**, and the test fails at `unwrap` rather than on the tolerance |
| **Where** | `crates/iccce-cmm/src/lut_ab.rs::tests::mba_fixture_matches_transicc_recorded_value`; the oracle value in `tools/gen-profiles/README.md` §5 and §6.1 *(all verified — read)* |
| **Invalidated by** | The lcms2 pin moving (**re-run, do not re-read** — the recorded 49.6117 % is one build's output); the fixture's bytes changing (the generator's `verify` is what detects that); any change to `lut_ab.rs`, `clut.rs`, `pcs_encoding.rs` or the `mBA ` curve counts; the fixture being absent, in which case the test **panics rather than skipping** — deliberately, because a committed fixture that is missing is a repository defect, not an environment one |

#### 3.10.3 NC-058 / NC-059 — the two `mAB ` traps, asserted on measured output

| Field | Value |
|---|---|
| **NC-058 — the 3×4 offsets** | The `mAB `/`mBA ` matrix is **nine coefficients then three offset terms**; reading 36 bytes and stopping produces *"a uniform colour cast that looks like a white-point problem"*. Pass 2 made the 36-byte read **unrepresentable in the decoder**; this row asserts the offsets **arrive in the output** of the evaluator: two models identical but for `e03 = 0.25`, differing in `X` by exactly `0.25 × 65535/32768 = 0.500 003 814…` within **1×10⁻⁹**. **Note the expected value is not `0.5`** — it carries the u1Fixed15 scale, and deriving it as ½ would be a ≈7.6 ppm error, which is the same trap the F.3 NOTE's `(32 768/65 535)` factor sets in the other direction |
| **NC-059 — the v4 encodings, not the legacy ones** | `mAB `/`mBA ` are **not** in 6.3.4.2 NOTE 3's legacy set, so their PCS side uses Tables 12/13. The pipeline test asserts the decode is `L* = n × 100` and `a*/b* = n × 255 − 128` within 1×10⁻⁹ through an identity A → CLUT → M → B chain, and a second test asserts device white → `L* = 100, a* = 0, b* = 0` **exactly** (`assert_eq!`). **DL-005's rule applied to the v4 side**: the legacy/general confusion is ≈0.39 `L*` at white and worse in `a*`/`b*` — below a ΔE gate's notice, above an exact-value test's |
| **Class** | **`arithmetic-identity`** for both. They prove the pipeline is structurally sound and that two specific known misreads are absent. **They cannot detect a wrong CLUT geometry, a wrong element order, or a shared misreading of 10.12** |
| **Where** | `lut_ab.rs::tests::{matrix_offsets_applied, mab_full_pipeline_identity_clut, b_only_v4_lab_decode_exact}` *(verified — the three test bodies read, and the names taken from the source rather than from the dispatch)* |

#### 3.10.4 ★ NC-060 / NC-061 — grayTRC F.2, and the green-cast trap made into a regression

| Field | Value |
|---|---|
| **The rule being asserted** | `icc__s__computational_models.md` §2, **primary_spec, verbatim**: the grayTRC connection value is a 0..1 scalar that **"shall be multiplied by the PCSXYZ or PCSLAB values of the PCS white point."** Clause 8.3.4 / 8.4.4 / 8.5.3 bind the model normatively *(the clause attributions are the module doc's, read here; **this librarian has not opened the PDF**)* |
| **★ Why a real profile was used, and what it catches** | Using the scalar directly as `Y` is right **only because `Y_white = 1.0`**; using it directly as `X` or `Z` is wrong by the **D50 chromaticity**, and the corpus names the symptom: *"a monochrome profile renders with a green cast."* **A test that checked only `Y` would pass with the bug present** — which is the same structural point as DL-016, in a different place. NC-060 asserts **all three components** against the D50 triple within 1×10⁻³ on `ewgray22.icm`, one of the four EIZO profiles the Pass 2 sweep flagged for a short `desc` block |
| **What the 1×10⁻³ bound is doing** | It admits the real curve's `TRC(1.0)` not being exactly 1.0 and the profile's own encoding, while the failure mode it must catch — `X = Z = t` instead of `t × D50` — is **0.036 and 0.175 away**, i.e. **36× and 175× the bound**. It cannot pass a green cast |
| **NC-061 — the synthetic twin** | Gamma-2.2, XYZ PCS: forward at 0.5 is `0.5^2.2 × D50` in `X` and `Z` within 1×10⁻¹², and the round trip returns 0.5 within 1×10⁻¹². **Arithmetic on the sourced formula**, with no file in it |
| **Class** | NC-060's white check is **`normative-rule-conformance`** (the expectation is the clause's, not the code's); its round trip and NC-061 are **self-consistency** / **arithmetic-identity**, and the round trip is worthless as correctness evidence however reassuring it looks |
| **★ What is NOT claimed** | **No gray value has ever been compared to lcms2**, and **no gray value has traversed `transform::Chain`** — the Chain wiring is verified to *exist* on both sides (`SourceModel::Gray`, `DestModel::Gray`, both reached as 8.10.2 step 4's second shape) and is exercised by **no test at all** *(verified — `transform.rs`'s two tests read; both are SWOP→sRGB)*. The dispatch's phrase *"neutrality through the chain"* is corrected here: the neutrality is measured **in the model** |
| **Where** | `crates/iccce-cmm/src/gray_trc.rs::tests::{real_gray_profile_white_maps_to_d50, synthetic_gray_forward_multiplies_full_white}` *(verified — read)* |

#### 3.10.5 ★★ GP-001 — a real parser defect, found by the fixture corpus. **Deliberately NOT given an NC number**

**Why no number:** this ledger's §2.1 and §2.2 rule stands — **parsing is
exact or it is wrong**; there is no tolerance, no measured value and no
ΔE, so an NC row would be a category error. It is recorded here because
the ledger is where a **falsified claim about iccce's own code** belongs,
and because the arc is the most instructive thing this project produced
today.

| | |
|---|---|
| **The defect** | `crates/iccce-profile/src/lut.rs::decode_lut_ab` counted curves by the **`mAB ` convention for both tag types** (B and M by `outputChan`, A by `inputChan`). On a CMYK `B2A0` (`inputChan = 3`, `outputChan = 4`) it expected **four** B curves where the specification puts **three**, walked into the matrix element, and reported `curve chain broken at element 3 (byte 68)` |
| **The rule, per type** | **10.12.2/4/6** (`mAB `): A = input, M = output, B = output. **10.13.2/4/6** (`mBA `): B = input, M = input, A = output. Generalised: **the curve set at the data's entry side is counted by `inputChan`, the exit side by `outputChan`** — which letter that is depends on the direction the tag runs. *(Conformance's **direct reads of the PDF**, quoted in `tools/gen-profiles/README.md` §5 and now in `lut.rs`'s own comment. **This librarian did not open the PDF**; both quotations were read as text.)* |
| **Blast radius** | **Every real CMYK `B2A0`.** Invisible whenever `inputChan == outputChan`, i.e. on every square LUT — which is why 40 profiles, 89 tests and a differential run had all passed over it |
| **Why the machine sweep could not find it** | The Pass 2 clause-1 record **predicted its own blind spot in writing**: the sweep is *"light or empty on the population Pass 4 depends on — large v4 CMYK press profiles with `mAB `/`mBA ` pipelines."* The fixture corpus **is** that population |
| **★ The refusal that preceded the finding by an hour** | The evaluator was written `mAB `-only and **refused `mBA ` on a curve-count contradiction found during design** — the corpus's one-sentence rule could not be reconciled with the tag's geometry, and the author declined to guess. **The doubt was the bug.** `lut_ab.rs`'s `LutAbModel` carries the note: *"The refusal was vindicated within the hour — GP-001: the guessed counts WOULD have been wrong."* *(verified — read.)* **A guess would have produced colour**, and a wrong colour looks exactly like a right one |
| **The cross-check, labelled** | lcms2's `Type_LUTB2A_Read` reads B and M with `inputChan`, A with `outputChan`; `transicc` evaluates the tag iccce refused. **Corroboration that two readers of the standard read it the same way** — weaker than the clause text, and not what settles it |
| **The corpus is the origin, and is still wrong** | `icc__type__lutAtoB_lutBtoA.md` carries **one blanket sentence for both types** (*"`A` curves = `inputChan`; `B` and `M` curves = `outputChan`"*), byte tables at `icc_secondary_code`, **A23 open**. **Still present as of this filing** *(verified — read 2026-08-11)*. Owed to `icc-spec-librarian`, together with **A23** (permitted element sets, enumerated verbatim in the generator's README) and **A25** |
| **Status** | **Fixed** in `2e98cfd` *(reported)*; the fix and its two clause triples are **verified in `lut.rs`**. **`tools/gen-profiles/README.md` §5 still says `Status: open`** and its §6.1 row still shows the refusal — **reported, not repaired**, that file being `icc-conformance`'s |
| **The regression** | **NC-057** |

#### 3.10.6 Two more observations with no NC number, and why each is right

1. **★ lcms2 does not refuse a major-version-5 profile; iccce refuses
   iccMAX by name.** `transicc` at the pin **accepts**
   `fixtures/synthetic/iccmax-version.icc` *(reported —
   `icc-conformance`'s run, recorded in `gen-profiles/README.md` §6.3)*.
   **No number, because it is a behavioural difference, not a
   measurement**; it is a **deliberate divergence, not a defect on
   either side** (Pass 2's plan text *requires* iccce to identify and
   refuse iccMAX by name); and it now has **a committed fixture** that
   will keep it visible. Precedent for recording it here without a
   number: §2.2.1, the machine-wide sweep.
2. **The fixture corpus's verification record is a coverage
   observation.** *(All **reported** by `icc-conformance` in its own
   file: `gen-profiles verify` **38 identical, 0 not identical**; the
   crate's own `cargo test` **28 passed**; the shipped binary reading
   **11 of 12** well-formed fixtures as specified with **one**
   specification-backed disagreement — GP-001 — and reporting **26 of
   26** authored defects exactly as intended.)* **No NC number**: it
   grades a parser, and parsing has no tolerance. What it establishes is
   what its own README says and no more — *"not 'iccce parses ICC
   profiles correctly', and it must never be rounded up to that."*

### 3.11 ★★ Pass 4b — the B2A direction, the v4 element pipeline, and the gray axis. **The ledger's first `derived-expectation` rows, and the tightest LUT claims it has ever carried**

**Read §2.7 before quoting anything here**, in particular its
build-commit discrepancy row. **Twenty-two rows, NC-062 … NC-083**,
mirroring the **28 emitted records** of `tools/difftest/README.md` §15
(the mapping is in §3.11.1 — six of the rows carry the same comparison
at two intents, which is why 22 rows cover 28 records). Every number
below was **read by this librarian** in README §15 and cross-read against
`TOLERANCES.md` §3.4.4, which agrees on all 28 *(verified — both read
2026-08-11; `TOLERANCES.md` is `icc-conformance`'s and was not edited)*.
**The run is `icc-conformance`'s; this librarian ran nothing.**

> **★ SHARED COVERAGE — part of every claim in this section.**
> **§A** is `sRGB Color Space Profile.icm` → `USWebCoatedSWOP.icc`
> (both **category (c)**, neither ours, both **v2.1.0**), **213
> deterministic RGB points** end to end plus **258 Lab points**
> PCS-side, at **perceptual and media-relative only**, `-c0`.
> **§B** is **one synthetic fixture** — `fixtures/synthetic/v4-cmyk-mab-lab.icc`,
> **category (a)**, bytes this project authored — **128 CMYK + 258 Lab
> points**, **media-relative only**.
> **§C** is `ewgray22.icm` → the system sRGB profile, **69 points of the
> gray axis**, perceptual and media-relative.
> **NOT covered, and these are the sentences that matter**: the
> **saturation** intent in *any* of the three directions; the
> **ICC-absolute** intent in *any* of them (§3.9's A4b posture is
> untouched); **any real v4 LUT profile** — a sweep of all **40**
> `.icc`/`.icm` in this machine's colour directory found **zero**
> `mAB `/`mBA ` tags *(reported — `icc-conformance`'s sweep, README
> §15.3.1)*, so §B's claims are about **one file this project wrote**;
> `lut8` with an **XYZ** PCS (still refused by name); the **grayTRC
> inverse**, because §C runs gray as the **source** — see §3.11.8;
> and **any published value**. **Pass 4 still has no ground-truth row.**

#### 3.11.1 The twenty-two rows, at a glance

| ID | What | Class | Tolerance | Observed |
|---|---|---|---|---|
| NC-062 | **§A apparatus**: the harness's own `lut8` arm vs `iccce_cmm::lut_transform::Lut16Model::pcs_to_device`, every Lab point, **both intents** (2 records) | self-consistency | 1×10⁻⁹ | **0,0 exactly** (bit-identical), both |
| **★ NC-063** | **§A** sRGB → SWOP through the shipped binary vs `transicc`, **device** max, both intents (2 records) | implementation-cross-check | 5×10⁻⁴ | **1,100×10⁻⁴** (perceptual) · **1,330×10⁻⁴** (media-relative) |
| NC-064 | …the same, **mean**, both intents (2 records) | implementation-cross-check | **∞ — REPORTED, NOT GRADED** | 2,361 502×10⁻⁵ · 2,546 479×10⁻⁵ |
| **★★ NC-065** | **§A with lcms2's own arithmetic modelled** — every rounding switched on in the harness's model (2 records) | implementation-cross-check | 5×10⁻⁵ | **3,101 114×10⁻⁵** · **3,100 458×10⁻⁵** — **2,03 lsb of 1/65535** |
| NC-066 | **§A** the same disagreement in ΔE2000, both sides' CMYK carried back through **the same file's `A2B1`** (2 records) | implementation-cross-check | 5×10⁻² | **7,095 173×10⁻³** · **5,710 814×10⁻³** |
| **★ NC-067** | **§A the sensitivity control** — the same B2A table evaluated **tetrahedrally** (2 records) | self-consistency (a counterfactual) | **∞ — REPORTED, NOT GRADED** | **1,526 949×10⁻²** · **1,311 299×10⁻²** = **139× and 99×** NC-063 |
| NC-068 | **§A PCS-side**: Lab → SWOP `B2A1` with the **source model removed**, iccce **in process** | implementation-cross-check | 5×10⁻⁴ | **6,485 006×10⁻⁵** |
| NC-069 | …the same, lcms2's arithmetic modelled | implementation-cross-check | 5×10⁻⁵ | **3,097 192×10⁻⁵** — the same 2,03 lsb, a **third** time |
| NC-070 | **§B precondition**: both interpolation geometries on the fixture's own **affine** CLUTs | self-consistency | 1×10⁻¹⁴ | **1,110 223×10⁻¹⁶** |
| **★★ NC-071** | **§B `mAB ` (CMYK→Lab): iccce vs the closed form derived from 10.12.1/10.12.5 and Tables 12/13** | **derived-expectation** | 1×10⁻¹² | **2,842 171×10⁻¹⁴** in `L*` |
| NC-072 | **§B `mAB `: lcms2 vs the same closed form** — the third reading | **derived-expectation** | 1×10⁻² | **2,325×10⁻³** in `L*` |
| **★★ NC-073** | **§B `mBA ` (Lab→CMYK): iccce vs the closed form** (10.13.1/10.13.4) | **derived-expectation** | 1×10⁻¹² | **2,220 446×10⁻¹⁶** in device |
| NC-074 | **§B `mBA `: lcms2 vs the same closed form** | **derived-expectation** | 1×10⁻⁴ | **1,873 190×10⁻⁵** in device |
| NC-075 | **§B** sRGB → fixture (`mBA `), shipped binary vs `transicc`, device | implementation-cross-check | 1×10⁻⁴ | **5,200×10⁻⁵** |
| NC-076 | **§B** fixture → sRGB (`mAB `), shipped binary vs `transicc`, device | implementation-cross-check | 2,5×10⁻⁴ | **1,012 157×10⁻⁴** |
| **★★ NC-077** | **§B the encoded-PCS overflow** — iccce clamps, lcms2 does not, over **10 of 128** points | implementation-cross-check | **∞ — REPORTED, NOT GRADED** | **0,611 700 5 ΔE2000** (device 4,440×10⁻³) |
| **★ NC-078** | **§B forced BPC measured in BOTH directions on one pair** — lcms2 against itself | oracle-behaviour-at-pin | **∞ — REPORTED, NOT GRADED** | v4 **source** → v2 dst: **0,0 bit-identical**; v2 src → v4 **destination**: **3,137 3×10⁻²** device |
| NC-079 | **§C** gray → sRGB, iccce vs lcms2, **device** max | implementation-cross-check | 2,5×10⁻⁴ | **9,686 275×10⁻⁵** |
| NC-080 | …the same, **mean** | implementation-cross-check | **∞ — REPORTED, NOT GRADED** | 1,782 154×10⁻⁵ |
| NC-081 | **§C** the same in ΔE2000 | implementation-cross-check | 5×10⁻² | **2,169 482×10⁻²** |
| **★★ NC-082** | **§C with lcms2's destination modelled** — `cmsReverseToneCurveEx(4096)` reimplemented | implementation-cross-check | 5×10⁻⁶ | **2,121 004×10⁻⁷** — a **457×** collapse, **below `transicc`'s print floor** |
| NC-083 | **§C** perceptual ≡ media-relative, **both sides** | implementation-cross-check | **0,0 — exact** | **0,0** |

**Record arithmetic, so the count is checkable**: rows NC-062 … NC-067
carry **two records each** (perceptual and media-relative) = 12;
NC-068/NC-069 = 14; NC-070 … NC-074 = 19; NC-075/NC-076 = 21;
NC-077/NC-078 = 23; NC-079 … NC-083 = 28. **28 records, `pass=28
fail=0` reported.** *(A count that comes out right is not evidence that
its terms are right — §3.9.8's lesson; the terms are enumerated above
for exactly that reason.)*

#### 3.11.2 ★★ NC-063 / NC-065 — the B2A direction, and a residual that is *reproduced* rather than bounded

| Field | Value |
|---|---|
| **What was compared** | `iccce transform --src <sRGB> --dst <SWOP>` — **the shipped binary on both sides** — against `transicc -c0` over 213 deterministic RGB points, at the perceptual and media-relative intents. SWOP's `B2A0`/`B2A1` are **`mft1`** (`lut8Type`), 3 in / 4 out, **33 points per axis** (35 937 nodes), 256-entry 8-bit tables, identity 3×3. **Unlike `A2B0`/`A2B2`, the three `B2A*` are three different blocks at three different offsets**, so perceptual and media-relative are genuinely different tables here |
| **Why the tolerance is not tuned** | 5×10⁻⁴ is **an envelope computed inside the harness from lcms2's own roundings, with no lcms2 output in it**: 256-entry input curves rounded to 1/65535 in *and* out, the CLUT stage input rounded to `u16` and its output returned as `u16/65535`, the output curves twice more, plus the source's 1024-entry `curv` TRCs — propagated through the actual B2A table it gives **1,330 241×10⁻⁴**. The observed **1,330×10⁻⁴** is **within 0,02 %** of it. The disagreement is not merely small, it is *accounted for* |
| **★ And what remains after the model is applied** | **NC-065: 3,101×10⁻⁵ = 2,03 lsb of 1/65535**, at both intents **and** on the PCS-side row (NC-069, source model removed entirely) — **three times independently**. What is left after modelling lcms2's `f64`-representable arithmetic is lcms2's **16-bit fixed-point** arithmetic, and nothing else |
| **★ The 8-bit table depth is NOT a divergence, and that is worth an assertion** | `Type_LUT8_Read` widens each stored byte with `(v<<8)\|v = v·257`, and `257·255 = 65535`, so lcms2's normalised sample is `v/255` — **bit-identical** to `iccce-cmm`'s `f64::from(v)/255.0`. The 1/255 granularity is *shared* and cancels. What it does do is make the pipeline **sensitive**: the largest adjacent-node step in this CLUT is 0,2235 of the device range, so an input difference is multiplied by up to **7,2** |
| **★ The `Lab8` codec agreed, in the direction where the mistake is easiest** | `_cmsReadOutputLUT` inserts `_cmsStageAllocLabV4ToV2` **only when `OriginalType == cmsSigLut16Type`** — for a `lut8Type` tag it does not. iccce's `PcsCodec::Lab8` encodes `L/100`, `(ab+128)/255` (Tables 12/13, corpus **A10**), and **the two agree exactly**. Had iccce applied the legacy 652,8 scale here, `L*` would be **0,39 % low ≈ 0,2 ΔE2000** — *below* §2's perceptibility anchor and **invisible to any ΔE-graded suite**. This is **DL-005's rule paying off in the `lut8` case**: the encoding is asserted where a ΔE gate could not see it |
| **Class** | **`implementation-cross-check`** for both. Agreement is evidence that two implementations read a clause the same way, **not** that either is right (rule 7) |
| **Coverage** | Two of four intents; one direction of one profile pair; **no out-of-`[0,1]` device value** (the shipped CLI does not accept one); nothing between 0 and 1/16 in RGB except through the 64-point random block — *"which is where the source EOTF's inverse slope and the XYZ→Lab sensitivity are both largest"* |
| **Where** | `tools/difftest/README.md` §15.2 and §15.5; `TOLERANCES.md` §3.4.4.2 rows A1/A3/A6 |
| **Invalidated by** | The lcms2 pin moving (**re-run, do not re-read**); either profile changing or being absent (**every §A row skips silently**); the 213/258-point grids changing; `iccce transform`'s print precision |

#### 3.11.3 ★★ NC-067, and the finding that makes **NA-006's cost direction-dependent**

`cmsio1.c`'s `_cmsReadOutputLUT` carries, verbatim including the
comment: *"Now it is time for a controversial stuff. I found that for 3D
LUTS using Lab used as indexer space, trilinear interpolation should be
used"* — and calls `ChangeInterpolationToTrilinear` whenever the PCS is
Lab *(read at the pin by `icc-conformance`; quoted in README §15.2.2 and
**verified as text** here — **this librarian has not built or read
lcms2**)*. **Trilinear over three inputs *is* n-linear**, which is
exactly what `iccce-cmm`'s `Clut::eval` computes.

**Consequences, and the first is the one that changes a claim this
ledger already carries:**

1. **The interpolation-method envelope that dominated Pass 4 —
   NA-006's 1,5741 ΔE2000 — is identically ZERO in the B2A direction**,
   for every Lab-PCS profile, which is every CMYK output profile in this
   machine's colour directory. **NA-006's measured cost is a fact about
   the A2B direction**, and the Pass 4 statement of it was **half a
   rule**. The dated note under NA-006 in §4 is the correction.
2. **It is a policy, not a specification.** ICC.1 is **SILENT** on CLUT
   interpolation (corpus **A16**, and the corpus's own seventh pass
   restates it for these tag types: *"the only clipping these clauses
   mandate is the matrix output clamp"*, interpolation unspecified)
   *(verified — `icc__type__lutAtoB_lutBtoA.md` §8 read)*. lcms2 offers a
   rationale, not a clause. **iccce's n-linear happens to agree with it;
   that is agreement between two choices, not conformance.**
3. **★ Therefore a cross-check in this direction cannot show that
   iccce's interpolation is right — only that it is the same.**
   **NC-067 is what stops that being invisible**: the same table
   evaluated tetrahedrally differs by **1,527×10⁻² / 1,311×10⁻²**,
   **139× and 99× the observed disagreement**. Without it, *"the two
   agree to 10⁻⁴"* would be a claim about a comparison that might not be
   able to see a geometry difference at all. **It can, by two orders of
   magnitude** — the same discipline **DL-018** required of Pass 3's
   round trip, applied to a method rather than to a deleted requirement.

**The generalisation is filed as `ARCHITECTURE.md` DL-021**, because
this is the **third** lcms2 behaviour in this project to turn out
direction- or path-dependent after being written down as a rule.

#### 3.11.4 ★★ NC-071 / NC-073 — the closed forms. **The strongest LUT rows in this ledger, and GP-001's real regression**

| Field | Value |
|---|---|
| **Why a fixture is the *only* instrument** | A sweep of every `.icc`/`.icm` in `C:\Windows\System32\spool\drivers\color\` — **40 profiles — found ZERO `mAB `/`mBA ` tags** *(reported)*. The single v4 profile carrying any LUT (`BlackWhite.icc`, `prtr`, `GRAY`) carries an **`mft1`**. **On this machine the entire v4 element-pipeline path cannot be exercised against a real profile at any price**, so `v4-cmyk-mab-lab.icc` is not a convenience — it is the instrument |
| **What was compared** | Both CLUTs in that fixture store a function **affine in one input and constant in the others** (`A2B0`: `L*` node `= 100·(1−K)`, `a*`/`b*` fixed; `B2A0`: `K = 1 − L*` along the `L*` axis). **Every interpolation geometry reproduces an affine function exactly**, so the method difference is *provably* zero here — and **NC-070 measures it rather than asserting it** (1,110×10⁻¹⁶). The output is then a **closed form in the input**, derived from **10.12.1/10.13.1** (element order), **10.12.5/10.13.4** (the 3×4 matrix and its offsets, applied in the **normalised** domain) and **6.3.4.2 Tables 12/13** (the **general** 16-bit PCSLAB encoding — `mAB `/`mBA ` are **not** in NOTE 3's legacy set) |
| **The expectations, written out so a reader can check them without running anything** | `mAB `: `L* = 100·(1 − K) + 0,390625`, `a* = 1,9921875`, `b* = 2,98828125`. `mBA `: `C = M = Y = 0`, `K = interp(1 → 32768/65535 → 0)` at `n_L = L*/100 + 1/256` |
| **★ Two details that are the whole value of the row** | (1) **The offsets are applied in the *normalised* domain**, so `+1/256` is `+0,390625` of `L*` and `+1,9921875` of `a*` — a derivation that applied them in `L*` units would be wrong by two orders of magnitude. (2) **The `mBA `'s middle node is `round(0,5·65535) = 32768`, i.e. 0,500 007 63, not 0,5** — an expectation built on the idealised line would be wrong by **7,6×10⁻⁶ and would look like an implementation defect**. This is the same trap as NC-058's `65535/32768` matrix scale, in the other direction |
| **Observed** | **iccce reproduces the closed form to `f64` noise in both directions: 2,842×10⁻¹⁴ in `L*`, 2,220×10⁻¹⁶ in device.** These are **the strongest statements any LUT row in this repository has been able to make** |
| **Class** | **`derived-expectation`** (§1, new). **Not ground truth.** Its specific weakness is stated in §1 and in `TOLERANCES.md` §3.4.4.1: **the fixture and the derivation are read out of the same corpus by the same project**, so a wrong transcription of 10.12/10.13 makes them wrong *together* and they agree perfectly. **That is why NC-072 and NC-074 exist** — lcms2 as the third reading, reproducing the same closed form to its own quantisation (2,325×10⁻³ `L*`, 1,873×10⁻⁵ device) |
| **★★ What NC-073 is *also*** | **GP-001's real regression.** NC-057 was one recorded point; NC-073 is the whole `mBA ` chain over **258 Lab points**, and the `mBA ` curve counts (**B=3, M=3, A=4** for a 3-in/4-out tag) are what make the chain evaluate at all. A reverted GP-001 fix does not produce a small error here — **it produces a decode refusal** |
| **Coverage — and it must not be rounded up** | **One synthetic fixture. One intent (media-relative). 128 CMYK + 258 Lab points.** It says nothing about any real v4 profile, because none exists here to say it about |
| **Where** | README §15.3.2; `TOLERANCES.md` §3.4.4.3 rows B0–B4 |
| **Invalidated by** | The fixture's bytes changing (`gen-profiles verify` is what detects that, and **nothing runs it automatically**); the corpus's transcription of 10.12/10.13 changing — **which would invalidate the expectation and the fixture at once**; `lut_ab.rs`, `clut.rs` or the curve counts changing; the pin moving (NC-072/NC-074 only) |

#### 3.11.5 ★★ NC-077 — the encoded-PCS overflow, and **a clause question that the corpus answered between the run and this filing**

**The mechanism.** At `K = 0` the `mAB ` CLUT's `L*` node is full scale
(`0xFFFF`, normalised 1,0) and the 3×4 matrix then adds `+1/256`, so the
value handed to the `B` curves is **1,003 906 25 — outside the range of
the 16-bit PCS encoding it is about to be read as**. **iccce clamps and
returns `L* = 100`; lcms2 does not and returns `L* = 100,390 625`**
(measured directly: `transicc -i<fixture> -o*Lab4` at `K = 0` prints
`100.3906 1.9922 2.9883`). **Cost 0,6117 ΔE2000 over 10 of 128 points**,
device 4,440×10⁻³ carried into sRGB — **the largest disagreement
anywhere in Pass 4b and in the neighbourhood of §2's ⚠ provisional 1,0
anchor.**

**Handled exactly as DL-019 requires**: the 10 points are **REPORTED,
NOT GRADED** and **excluded** from NC-071 and NC-076, whose records name
the excluded set and why. Both alternatives were rejected **in writing**
— a ~0,7 ΔE tolerance would be a number chosen because it passed, and a
permanent red line stops being read.

> **★★ A dated correction to the run's own framing, made by reading the
> live corpus rather than the dispatch, 2026-08-11.**
>
> README §15.3.3 states *"which is right is NOT settled, and that is the
> finding"*, and owes a dispatch to `icc-spec-librarian` asking **two**
> questions: (a) is the **matrix element's output** required to be
> clipped to the domain of the curve that follows it, and (b) is the
> **final `B` curves' output** required to be clipped to the encodable
> PCS range of 6.3.4.2. The dispatch that produced this filing carries
> the question as still owed and **queued**.
>
> **Question (a) is answered, and it was answered before this filing.**
> The corpus's **seventh** pass transcribed 10.12.5 and 10.13.3
> VERBATIM: *"The range of input values X1, X2 and X3 is 0,0 to 1,0. The
> resultant values Y1, Y2 and Y3 **shall be clipped to the range 0,0 to
> 1,0**"* — and used as inputs to the **"B" curves** (10.12.5) or the
> **"M" curves** (10.13.3). The corpus adds: *"Clipping here is
> normative and is one of the few places ICC.1 says where clipping
> happens. It is at the matrix output, before the next curve set, per
> component."* *(**verified** — `ICC_Spec\icc\icc__type__lutAtoB_lutBtoA.md`
> §5 read 2026-08-11; the file's `evidence:` line is
> **`primary_spec` (clauses 10.12, 10.13 and Tables 45–48 transcribed
> 2026-08-11, two extractors)**, which is what **DL-014** requires before
> a clause number may be cited.)*
>
> **The fixture's overflow arises exactly there** — it is the *matrix
> output* that is 1,003 906 25, and 10.12.5's sentence governs precisely
> that value. **So this instance is not, in fact, a question about
> 10.18's curve domain at all**, which is the clause README §15.3.3
> attributes iccce's behaviour to. **iccce's `L* = 100` is what the
> clause requires**, and lcms2's 100,390 625 is on the far side of a
> `shall`.
>
> **iccce's live code already reflects this**, and it is *not* the code
> the run described: `lut_ab.rs::apply_matrix_3x4` now clamps each
> component to `[0,1]`, with the comment *"the NORMATIVE matrix-output
> clamp captured in the corpus's per-type re-transcription of
> 10.12/10.13 (2026-08-11, seventh pass); it was absent from the first
> implementation of this function because the clause text had not been
> transcribed yet"* *(verified — read)*. **The result at these 10 points
> is unchanged** — both routes clamp the B-curve input to 1,0 — **but
> that is this librarian's arithmetic reading, not a measurement**, and
> the only evidence that the rows still hold is the engineer's reported
> re-run at `pass=28 fail=0` (§2.7).
>
> **Three things this correction does NOT do.** (1) It does **not**
> answer question (b): whether the **final** `B` output must be clipped
> to the encodable PCS range is still open, and the corpus says *"the
> only clipping these clauses mandate is the matrix output clamp"* —
> so the queued dispatch should be **narrowed to (b)**, not cancelled.
> (2) It does **not** make lcms2 "non-conforming": per **A39b** clause 5
> binds a CMM only to *reading* profiles, so the available word is
> **divergence** — the same hedge `TOLERANCES.md` §5.2 attached to the
> device-range finding. (3) It does **not** re-grade NC-077. **Grading
> it is `icc-conformance`'s call** on its own files, and it should be
> taken with the narrowed question answered rather than on this reading
> alone.

**One thing this finding is not:** a defect the fixture was designed to
catch. `v4-cmyk-mab-lab.icc` carries non-zero matrix offsets because
*dropping* them is the classic misread (NC-058's trap). That the same
offsets also push a value past full scale is **an accident of the
fixture — and the best argument in this project for authoring fixtures
with awkward values rather than tidy ones.**

#### 3.11.6 ★ NC-078 — forced BPC is decided by the **DESTINATION** profile's version, which makes DL-013 / corpus M2 half a rule

**Both sides of this row are lcms2**: its own media-relative output
against its own perceptual output, on one pair of profiles, in each
direction. **It says nothing whatever about iccce**, which is why its
class is `oracle-behaviour-at-pin` (`TOLERANCES.md` calls the same row
`oracle-reproducibility`; the two names describe the same thing and the
ledger uses its own §1 vocabulary).

| direction | lcms2 against itself |
|---|---|
| v4 fixture as **source**, v2 sRGB destination | **0,0 — bit-identical** |
| v2 sRGB source, v4 fixture as **destination** | **3,137 3×10⁻²** device — `K` at black moves **99,6094 % → 96,4721 %** |

**The mechanism, read at the pin**: `_cmsLinkProfiles` sets `BPC[i]` per
profile, but `DefaultICCintents` consumes it as
`ComputeConversion(i, hProfiles, Intent, BPC[i], …)`, which builds the
conversion **from `hProfiles[i−1]` into `hProfiles[i]`**. **The flag
that decides is the destination profile's**; a v4 *source* into a v2
destination sets a flag nothing reads.

**What it costs anyone who quotes M2 or DL-013 as written:** *"lcms2
forces BPC on for v4 profiles at perceptual and saturation"* is enough
to decide whether a comparison is confounded **only if you also know the
direction**. A corpus correction to M2 is owed to `icc-spec-librarian`
(README §15.7 item 2), and **it is not this librarian's file**. The
general rule is **DL-021**.

#### 3.11.7 ★★ NC-082 — the gray axis, and a residual **reproduced** to below the oracle's print floor

| Field | Value |
|---|---|
| **Why §C is unusually clean** | **The source cannot contribute, and that was established before the run rather than hoped.** Both implementations evaluate the same *analytic* γ = 2,199 218 75 (lcms2 turns a single-value `curv` into a type-1 parametric curve, so the tabulated-curve quantisation does not apply) and multiply by the **same D50 literals** — `cmsD50X/Y/Z` = 0.9642/1.0/0.8249 in `lcms2.h`, and `iccce_color::D50` is the same three. **Annex F.2's white multiplication cannot diverge**, so §C measures **lcms2's sRGB *destination* alone** — which is also the instrument check Pass 4 owed and inherited from Adobe RGB instead |
| **The whole residual, attributed** | lcms2 inverts each 1024-entry `curv` with `cmsReverseToneCurve` = `cmsReverseToneCurveEx(4096, ·)` — a **4096-entry `u16` resampling** of the inverse, chorded between forward-table knots, then evaluated through the float path that rounds input and output to 1/65535. iccce inverts the stored table directly. Reimplementing that one mechanism collapses the disagreement from **9,686×10⁻⁵ to 2,121×10⁻⁷ — 457×**, which is **below `transicc`'s 4-decimal print floor of 3,9×10⁻⁷**. Worst point `g = 2/255`: iccce `0,000300`, lcms2 `0,000397`, **model `0,000397`.** *The disagreement is not merely explained, it is reproduced* |
| **★ The envelope sits 0,06 % BELOW the observation, and that is the right direction** | 9,680×10⁻⁵ modelled against 9,686×10⁻⁵ observed: the envelope is computed between two `f64` pipelines while the observation additionally carries `transicc`'s 4-decimal print and `iccce transform`'s 6-decimal print. **An envelope comfortably *above* the observation would have meant the model was pessimistic about lcms2** — i.e. would have been the weaker result, not the stronger one |
| **★ NC-081's maximum is near BLACK, and it inverts a note this project carried from Pass 3** | §13.6 recorded *"near black the device metric explodes while ΔE stays small"* — true of a **device** comparison amplified by an inverse TRC. Here the comparison is *already* in device units and the amplification runs the other way: below sRGB's linear breakpoint a device difference `δ` becomes `δ/12,92` of linear light, and CIELAB's **chromatic** sensitivity on its own linear segment is `da*/dX = 500·7,787/X_n = 4038`, giving `Δa* ≈ 136 δ` against `ΔL* ≈ 69,9 δ`; with `S_C ≈ 1` and `S_L ≈ 1,75` **the chromatic term dominates by ~3× and the maximum is at the dark end** |
| **NC-083** | Perceptual ≡ media-relative, **graded at exactly 0,0 and observed 0,0 on both sides**. A monochrome profile carries **no `A2Bx`/`B2Ax` at all**, so clause 8.10.2 has nothing to select and both intents fall through to step 4's F.2 model. **No arithmetic in either chain could make the difference small** — any difference is an intent-dispatch defect, which is why exact equality is the only honest bound |
| **Class** | **`implementation-cross-check`** throughout |
| **Where** | README §15.4; `TOLERANCES.md` §3.4.4.4 rows C1–C5 |
| **Invalidated by** | The pin moving (**NC-082 especially: its content is a transcription of `cmsReverseToneCurveEx` — a retuned resampler would keep reproducing the *old* lcms2 perfectly and invalidate it silently**); `ewgray22.icm` or the system sRGB profile changing or being absent (**every §C row skips silently**); `gray_trc.rs` or `curve.rs`'s `invert_table` changing |

#### 3.11.8 What §3.11 does **not** claim

1. **No ground truth.** `derived-expectation` is **not** it. Four rows
   are derived from clause text this project transcribed, and if the
   transcription is wrong they are wrong. **Pass 4 and Pass 4b together
   contain zero `published-ground-truth` rows.**
2. **The saturation intent is unmeasured in all three directions**, and
   **ICC-absolute is unmeasured in all three**. `B2A2` exists and is a
   third distinct table; ICC-absolute through a **LUT destination** has
   never run at all, and it is the one case where the D.6/D.7 composite
   is applied *before* the PCS is encoded rather than after.
3. **★ The grayTRC INVERSE has still never been compared to anything,
   and NA-008 is still unmeasured.** §C runs gray as the **source**
   (`GRAY → RGB`), so `GrayTrc::device_to_pcs` is what was measured;
   `pcs_to_device` — the projection onto the achromatic channel that
   NA-008 registers — **was not reached**. `NEXT_SESSION.md`'s prediction
   that a gray differential *"would give NA-008 its first measurement"*
   is **falsified by the direction the differential actually ran**, and
   NA-008's dated note in §4 records it.
4. **§B is one file this project wrote.** It is not evidence about any
   real v4 profile, and none exists on this machine to be evidence about.
5. **Two rows grade a MODEL, not the shipped binary.** NC-068 and
   NC-069 run `iccce-cmm` **in process**, because the shipped CLI has no
   Lab entry point. Their records say so; nothing else in Pass 4b does
   this.
6. **Nothing here was run by this librarian**, nothing was run on Linux,
   and **no CI run has ever been observed by anyone**.

### 3.12 ★★ Pass 5 — black point compensation. **The ledger's first rows graded against a printed equation of the primary specification, and its first pre-registered NEGATIVE result**

**Read §2.8 before quoting anything here**, in particular its two rows
about the red commits and about `pass5.rs` carrying no tests.
**Twenty-one rows, NC-084 … NC-104**, mirroring `TOLERANCES.md` §3.5's
twenty comparisons **P1 … P20** and `tools/difftest/README.md` §16.
Every number below was **read by this librarian** in README §16 and
cross-read against `TOLERANCES.md` §3.5, **which agrees on all of them**
*(verified — both read 2026-08-11; `TOLERANCES.md` is
`icc-conformance`'s and was not edited)*. **The run is
`icc-conformance`'s; this librarian ran nothing.**

> **★ SHARED COVERAGE — part of every claim in this section.**
> **§A** uses **no profile and no oracle**: it is arithmetic against two
> documents, 1005 PCS values and 20 000 random black-point pairs.
> **§B (S2)** is `fixtures/synthetic/v4-cmyk-mab-lab.icc` → the system
> sRGB profile, **perceptual only**, **128 CMYK points with 10 excluded**
> as §3.11.5's encoded-PCS overflow, `-c0`.
> **§C (S3)** is the same pair reversed, **perceptual only**, **213 RGB
> points**.
> **§D/§E** are S1 (sRGB → Adobe RGB, media-relative), S4 (sRGB →
> `v4-rgb-matrix-trc.icc`, perceptual), S5 (sRGB → `USWebCoatedSWOP.icc`,
> media-relative) and S6 (two committed fixtures, ICC-absolute).
> **NOT covered, and these are the sentences that matter:** **any
> black-point ESTIMATOR** — see §3.12.3, and it is the boundary that
> most invites rounding up; the **saturation** intent (lcms2 forces BPC
> there, iccce's subset has no LUT arm for it, so **that comparison has
> no iccce half**); **any real v4 LUT profile** (the 40-profile sweep's
> zero stands — §B and §C are **one file this project wrote**); the
> **gray side of iccce's own subset**, implemented and unexercised;
> lcms2's **0,002 empty-layer threshold observed** (it is **solved for,
> not triggered**); **devicelink and abstract** profiles; **any other
> platform**; and **any published value for a BPC result** — there is
> none, for the same reason there is none for perceptual (**A27**).
> **Pass 5 has no ground-truth row, and neither does any other Pass.**

#### 3.12.1 The twenty-one rows, at a glance

| ID | What | Class | Tolerance | Observed |
|---|---|---|---|---|
| **★★ NC-084** | **§A `BpcScale(0 → PB)` vs ICC.1:2022 6.3.4.3's printed equation**, 1005 PCS values, different algebraic form | **derived-expectation** *(see §3.12.2 — the class was judged, not copied)* | 1×10⁻¹⁴ | **1,110×10⁻¹⁶** |
| **★ NC-085** | **§A `BpcScale(bs → bd)` vs a Gaussian elimination on Maria (2013) §4.2's two published constraints**, 20 000 random draws, different solution method | **derived-expectation** (`published_literature` source, retrieved compliantly) | 1×10⁻¹⁴ | **3,331×10⁻¹⁶** |
| NC-086 | **§A** the two constraints hold under iccce's own map (`apply(D50) = D50`, `apply(bs) = bd`) — catches a map anchored on the **wrong white** | derived-expectation | 1×10⁻¹⁴ | **3,331×10⁻¹⁶** |
| NC-087 | **§A** equal black points are the **exact** identity, 1001 values | self-consistency | **0,0 — exact** | **0,0** |
| **★ NC-088** | **§A** lcms2's **`IsEmptyLayer` discriminant** for the S2/S3 map — a constant this project had not recorded | derived-expectation, from lcms2's own inequality (**READ, not RUN**) | **∞ — REPORTED, NOT GRADED** | **0,015 342 = 7,7× the 0,002 threshold**; the threshold bites at ≈**0,41 `L*`** |
| NC-089 | **§B baseline — BPC OFF on both sides**, device max. **Graded first on purpose** | implementation-cross-check | 2,5×10⁻⁴ | **1,012 157×10⁻⁴** |
| **★★ NC-090** | **§B BPC ON on both sides**, device max — **the row the done-when's second clause rests on** | implementation-cross-check | 2,5×10⁻⁴ | **1,110 588×10⁻⁴** — **sensitivity 388×** |
| NC-091 | **§B** the same in ΔE2000 (2 records: on, and the off baseline) | implementation-cross-check | 5×10⁻² | **1,262 374×10⁻²** (on) · **1,962 920×10⁻²** (off) |
| **★ NC-092** | **§B the DIRECTION — nothing may rise.** A sign with an algebraic proof; **needs no tolerance at all** | self-consistency | **0,0 — exact** | **0,0** (largest fall **4,304×10⁻² device = 3,5159 ΔE2000**) |
| NC-093 | **§B** lcms2 `-b` vs no `-b` — **it does NOT force here** (v2 destination). The control that makes NC-095 mean something | oracle-behaviour-at-pin | **∞ — REPORTED, NOT GRADED** | **4,290 863×10⁻²** |
| **★ NC-094** | **§B A41 priced in a pipeline** — the map rebuilt with **ICC.1 Table 16's printed decimals** over this grid | derived-expectation | **∞ — REPORTED, NOT GRADED** | **0,050 201 ΔE2000 / 0,037 416 ΔE76 / 0,005 364 ΔL\*** |
| NC-095 | **§C** lcms2 forces BPC **unasked** into a v4 destination at perceptual — exact, because the flag is **overwritten before it is read** | oracle-behaviour-at-pin | **0,0 — exact** | **0,0** |
| **★★ NC-096** | **§C** iccce `--bpc` vs lcms2, device max, run against **both** lcms2 arms (`-b` and unasked) — 2 records | implementation-cross-check | 1×10⁻⁴ | **4,600×10⁻⁵** against each — **sensitivity 682×** |
| NC-097 | **§C the DIRECTION — no `K` may rise.** ★ Its first draft graded the negated **minimum** and failed at 3,1372×10⁻²; **the failure was the row, not the engine** | self-consistency | **0,0 — exact** | **0,0** |
| **★★ NC-098** | **§C the lift at device black vs a CLOSED FORM** — the one end-to-end row in Pass 5 with **no implementation's output in it** | **derived-expectation** | 5×10⁻⁶ | **9,504 522×10⁻⁸** — *below* the print floor its bound was derived from |
| **★ NC-099** | **§C lcms2's own forced-BPC `K` at black against the same closed form** — **the third reading** | **derived-expectation** | 1×10⁻⁴ | **9,046 508×10⁻⁷** — within one printed lsb |
| **★★ NC-100** | **§C THE POLICY** — iccce **without** `--bpc` vs lcms2 **without** `-b`; lcms2 lighter at black. **Neither is a defect; the number IS the policy** (2 records: the difference, and the D11 fingerprint) | implementation-cross-check | **∞ — REPORTED, NOT GRADED** (DL-019) | **3,137 300×10⁻² device = 3,137 348 `L*`** |
| NC-101 | **§D S1** — both implementations' BPC is a **no-op** on two v2 matrix/TRC files (2 records: lcms2, iccce). **NULL BY CONSTRUCTION and labelled so** | oracle-behaviour-at-pin · self-consistency | **0,0 — exact** | **0,0** and **0,0** |
| **★ NC-102** | **§D S4 — corpus trap T5 measured.** The configuration M2 says forces BPC, and it costs **exactly nothing** (2 records) — lcms2 via guard 3's **matrix-shaper escape**, iccce by a **different route** | oracle-behaviour-at-pin · self-consistency | **0,0 — exact** | **0,0** and **0,0** |
| NC-103 | **§E S5** — iccce **refuses by name** outside its estimation subset (v2 CMYK `prtr` destination at media-relative). **Graded, not merely reported** | self-consistency (behavioural, 0/1) | **exact wording** | **refused as required** |
| NC-104 | **§E S6** — iccce **refuses by name** at the ICC-absolute intent (Maria 2013 §4.1, verbatim). Runs on a machine with **no colour directory** | self-consistency (behavioural, 0/1) | **exact wording** | **refused as required** |

**Record arithmetic, so the count is checkable.** §A = 5 (NC-084 … NC-088);
§B = 7 (NC-089, NC-090, **NC-091 ×2**, NC-092, NC-093, NC-094); §C = 8
(NC-095, **NC-096 ×2**, NC-097, NC-098, NC-099, **NC-100 ×2**); §D/§E = 6
(**NC-101 ×2**, **NC-102 ×2**, NC-103, NC-104). **Total 26**, which is
exactly the whole-suite figure's movement from **64** (after Pass 4b) to
**90** (after Pass 5). ***That reconciliation is this librarian's
subtraction of two reported totals, and §16 states no Pass 5 record
count of its own*** — a sum that comes out right is not evidence that
its terms are right (§3.9.8), which is why the terms are enumerated.

#### 3.12.2 ★★ NC-084's CLASS was judged, not copied — and the answer is *not yet* `normative-rule-conformance`

The tempting sentence is *"the ledger's first primary-spec conformance
row for a transform."* **Three things are wrong with it**, and the row
is filed as **`derived-expectation`** accordingly.

1. **The tier label does not say `primary_spec`.** §1's
   `normative-rule-conformance` class requires *verbatim normative text
   transcribed in the corpus **at `primary_spec` tier***. The corpus file
   is `ICC_Spec\icc\icc__ref__bpc.md`, and its `evidence:` line grades
   **§2 and §3** as **`cross_verified_2src` (ICC.1:2022 by two engines)**
   — *not* `primary_spec` *(verified — the frontmatter read 2026-08-11)*.
   Compare §3.7.1, whose six rows earned the class off
   `icc__s__computational_models.md`, `evidence: primary_spec`, whole
   file.
2. **It is not the ledger's first anything of that kind.** **NC-022 …
   NC-027** (Pass 3, Annex F.1 / clause 10.6 / F.8–F.16) are
   `normative-rule-conformance` rows and have been since the Pass 3
   filing.
3. **It is not a transform.** NC-084 grades the **map function**
   `BpcScale` over PCS values. The **end-to-end** Pass 5 row is
   **NC-098**, whose expectation is a *derivation* with a fixture's
   stored bytes in it — precisely `derived-expectation`, precisely
   §3.11's class.

**What NC-084 genuinely is, stated at full strength and no higher:** the
expectation is **6.3.4.3's printed equation itself**, evaluated by a
different algebraic route, with **no fixture and no implementation's
output in it** — which makes it the *strongest-sourced* row in this
section and arguably a **new sub-shape** of `derived-expectation`: the
class's stated weakness is *"the fixture and the derivation come out of
the same corpus"*, and **NC-084 has no fixture at all**. Its residual
weakness is the transcription risk alone, shared with every
`normative-rule-conformance` row.

**★ Promotion is a one-line question for `icc-spec-librarian`, and it is
now load-bearing on a ledger class rather than on a doc-comment
heading.** `crates/iccce-cmm/src/bpc.rs` heads the map
**"PRIMARY-SOURCED"** *(verified — read)* while the corpus's own
`evidence:` line does not. **If the tier is `primary_spec`, NC-084 and
NC-086 become `normative-rule-conformance` rows and this section's
strength genuinely rises.** Until someone whose file that is says so,
**the weaker label stands** — a class is not raised by how good the
number looks. This is the **DL-014** audit item, and it has grown teeth.

#### 3.12.3 ★★ The negative result, which was DERIVED before it was OBSERVED — no row here discriminates the ESTIMATORS

BPC has three rules, each keyed on something different: an
**applicability set**, an **estimation method**, and a **forcing
policy**. Before anything ran, both sides' reach was read out of their
sources — iccce's `Chain::with_bpc` subset against lcms2's six
first-match-wins guards at the pin — and the intersection produced this,
in advance:

> **Everywhere iccce will do BPC at all, lcms2's estimator reduces to
> the same two values.** On a matrix/TRC or gray side, lcms2's guard 6
> darkest-colorant estimate is device black through the profile at a
> colorimetric intent — exactly iccce's `device_to_pcs(0)` — and **on
> every profile in reach that is exactly `XYZ (0,0,0)`, because every
> TRC in the corpus has `trc(0) = 0`**. On a v4 LUT side at perceptual,
> lcms2's guard 3 returns **the same A41 triple iccce hard-codes**.

**So NC-089 … NC-104 grade the SCALING MAP, the DIRECTION and the
PIPELINE the map sits in. They do not grade the ESTIMATION, and no row
here may be quoted as if they did.** **lcms2's methods 3 and 4 — the ink
round trip and the least-squares quadratic curve fit — are untested
against anything**, because iccce refuses rather than implementing them
(**NC-103**), and that refusal is a **coverage gap, not a bug**: lcms2
answers there, iccce does not, **so no comparison exists and Pass 5
claims none**.

**Why this is the section's most valuable paragraph.** A session that
measured first would have found six small numbers and read them as six
independent agreements about "BPC". **When two implementations agree,
the question is what they were free to disagree about** — answered from
their sources, not from the size of the residual. Filed as
`ARCHITECTURE.md` **DL-023**.

**What would close it:** **a synthetic v4 RGB-or-gray LUT fixture with a
non-zero device black.** `fixtures/synthetic/` holds **38 `.icc` files**
*(verified — enumerated)*; the only v4 LUT among them is
`v4-cmyk-mab-lab.icc`, and `icc-conformance` records that **every
profile in reach has `trc(0) = 0`**. The instrument does not exist, and
until it does **NA-009's cost stays unmeasured for a reason, not for
want of trying.**

#### 3.12.4 ★★ NC-090 / NC-096 — the agreement rows, and the ratio that makes a small number mean something

| Field | Value |
|---|---|
| **What was compared** | `iccce transform --bpc` — **the shipped binary** — against `transicc -b`, over 128 CMYK points (**§B**, 10 excluded as the encoded-PCS overflow) and 213 RGB points (**§C**), perceptual, `-c0`. **§C runs against BOTH lcms2 arms** (`-b` and unasked), so a reader need not trust that NC-095's forcing row makes them identical |
| **Why the tolerances are not tuned** | Both are **earlier Passes' envelopes carried forward deliberately**. NC-090's is row **B6**'s computed envelope (lcms2's 4096-entry `cmsReverseToneCurveEx` resampling at 9,68×10⁻⁵ + the fixture CLUT's `u16` lsb at 1,5×10⁻⁵ ≈ 1,15×10⁻⁴) **× the map's gain `a = 1,0035`**, plus the `f32` stage-boundary rounding of **the single matrix stage BPC inserts** (7,8×10⁻⁷): BPC adds **no table lookup and no `u16` rounding**, so it adds no quantisation of the kind the envelope models. NC-096 **keeps B5's constant unchanged on purpose** — *"a tolerance that moved when the only change is a linear stage would be a tolerance tracking the observation"* |
| **★ The inherited term, flagged before the run and then priced by it** | The derivation named its own weakest assumption: *where on the axis* lcms2's reverse tone curve resamples worst, because BPC moves the operating point into the shadow. **Switching BPC on moved the residual by 1,097× where the gain alone predicts 1,0035 — so the flagged risk is real and worth ~9,4 %**, and the envelope still bounds it. A derivation that names its weakest term and is then vindicated on it is worth more than one that quietly omits it |
| **★★ The sensitivity, which is what stops these being nulls from an instrument that could not tell** | **BPC itself moves §B by up to 3,5159 ΔE2000 (4,304×10⁻² device) while the two implementations disagree by 1,11×10⁻⁴ — 388×**; in §C it moves `K` by 3,137×10⁻² against a 4,6×10⁻⁵ disagreement — **682×**. This is the same argument NC-067's tetrahedral counterfactual makes for interpolation geometry, **with the advantage that here it is free**, because the BPC-off arm is already run as the baseline (NC-089). **Every future agreement claim in this project should carry its sensitivity ratio** |
| **Class** | **`implementation-cross-check`** — evidence that two implementations read a rule the same way, **never** that either is right (rule 7). And per §3.12.3, evidence about **the map and the pipeline**, not the estimators |
| **Coverage** | **One intent (perceptual), one synthetic fixture, one system profile, two directions, one platform, one pin.** Ten of §B's 128 points excluded. **No saturation arm exists on iccce's side at all** |
| **Where** | `tools/difftest/README.md` §16.3, §16.4; `TOLERANCES.md` §3.5.3 row **P6b**, §3.5.4 row **P12** |
| **Invalidated by** | The lcms2 pin moving (**re-run, do not re-read**); the fixture's bytes changing (`gen-profiles verify` is the only detector and nothing runs it automatically); the system sRGB profile changing or being absent (**both rows skip silently**); the grids changing — **and nothing pins them**, see §2.8 |

#### 3.12.5 ★★ NC-098 / NC-099 — the one end-to-end expectation with no implementation's output in it, and the third reading that keeps it honest

Everything else in Pass 5 that grades an end-to-end transform grades it
against lcms2. **NC-098 does not.** The chain, each step separately
justified:

1. `RGB (0,0,0) → XYZ (0,0,0)` **exactly** — every TRC is 0 at 0, and no
   quantisation can move zero.
2. BPC's **second constraint** sends that to the **destination black
   exactly**, i.e. to the A41 triple. **This is a premise here and a
   measurement in NC-086** (3,331×10⁻¹⁶), so it is not an assumption.
3. `L* = (841/108)·116 × 0,003 473 1 = 3,137 238` — CIELAB's **linear**
   segment; every value here is far below the knee, so the threshold
   question does not arise.
4. The fixture's `mBA ` closed form (§3.11's row B3, using the
   **stored** `u16` nodes — `32768/65535`, **not** an idealised ½) turns
   that into `K`.

giving predicted `K` = **0,964 721 905** with BPC and **0,996 093 810**
without: a predicted lift of **0,031 371 905** against an observed
**0,031 372 000**. The bound is the observation's **print floor and
nothing else** (6 decimals per arm, ±10⁻⁶ on the difference; 5×10⁻⁶ is
5×) and the residual — **9,5×10⁻⁸** — came in *below* it, the two arms'
print roundings having partly cancelled.

**★ It also discriminates the constant.** The wrong perceptual-black
triple's signature here is `ΔK ≈ 5,4×10⁻⁵`, **11× the bound**, so
NC-098 doubles as the **A41 discriminator** (see NA-010).

**★★ And the third reading.** **NC-099**: lcms2's own forced-BPC `K` at
device black, against a derivation it had no part in — **9,0×10⁻⁷,
within one printed lsb**. That is what stops the fixture and the
derivation from being wrong *together*, which §3.4.4.1 names as the
standing weakness of every derived expectation. **Two independent
implementations and one clause-derived closed form meeting at the same
number is the strongest structure this project has built**, and it is
still **not ground truth**.

#### 3.12.6 ★★ NC-100 — the policy difference, REPORTED NOT GRADED, and the D11 fingerprint answered

Same pair, same intent, **neither side asked for BPC**:

```text
   iccce (no --bpc)  vs  lcms2 (no -b)  :  3,1373×10⁻² device  =  3,137 348 L*
   lcms2 is LIGHTER at black (its K is lower)
```

**Neither implementation is wrong. The number is the policy.**

- **lcms2 forces BPC on** for a **v4 destination** at perceptual or
  saturation, overriding the caller, on the authority of a source
  comment attributing it to Adobe's document — **which nobody in this
  project has read**.
- **The one published BPC source this project holds** (Maria 2013)
  corroborates the *exclusion* set and is **silent on the enable
  policy** (`ICC_Spec` §7.1).
- **iccce declines to force**, and requires `--bpc`.

**Grading it would mean picking a winner without a clause**, so it is
handled exactly as §3.9.5's absolute-intent rows and §3.11.5's clamp
divergence are: **REPORTED, NOT GRADED (DL-019)**. The two available
gradings — a ~3,2 `L*` tolerance chosen because it passed, or a
permanent red line — were **both rejected in writing**.

**★ The corpus's D11 watch is answered rather than tolerated.** D11 says
a ≈3,14 `L*` deviation **with a sign** is the fingerprint of the v2/v4
perceptual-black mismatch, and that the sign says which CMM you were
matching:

```text
   observed policy difference        3,137 348 L*
   PRM black, Table 16's 08h         3,137 254 L*
   the A41 triple's L*               3,137 238 L*     match to 1,1×10⁻⁴ L*
```

**It matches lcms2's M2 route** — force for a v4 **destination**,
mapping the source's zero black **up** to the PRM black, so lcms2 is
*lighter*, which is the observed sign — **and not iccDEV's**, which
applies 6.3.4.3 to the v2 side's transform data at link time and inverts
on output. **The two are distinguishable in §B**, where the v2 profile
is the *destination*: iccDEV would map the PRM black **down** to zero
there, and lcms2 does nothing unless asked — **which is exactly what
NC-093 observed** (4,29×10⁻² between `-b` and no `-b`, and the unasked
arm matching iccce's unasked arm to 1,01×10⁻⁴). **The sign was diagnosed
from the mechanism and confirmed in both directions, not read off one
number** — DL-021's discipline, applied to a policy.

**This is now `ARCHITECTURE.md` DL-022.** It was previously carried as a
paragraph inside **NA-009** (*"recorded here rather than minted as its
own entry"*); what changed is that it now has **a measured size, a
graded posture and a user-visible consequence** — two correct CMMs give
different pictures by default, through a flag on a shipped binary.

#### 3.12.7 ★ NC-088 — a threshold lcms2 has and iccce deliberately lacks. **READ, not RUN**

`cmscnvrt.c` L327–348, `IsEmptyLayer`, sums the BPC matrix's deviation
from the identity plus its offsets (already divided by
`MAX_ENCODEABLE_XYZ`) and `AddConversion` inserts the stage **only when
that sum is ≥ `0,002`**. So **lcms2 silently performs no BPC at all**
once the two black points are within roughly **0,41 `L*`** of each
other; **iccce has no such threshold and applies the map however small
it is.**

- **For the S2/S3 map the discriminant is 0,015 342 — 7,7× the
  threshold**, so **nothing in this section is affected by it**.
- **The 0,41 `L*` figure is a solution of lcms2's own inequality, not an
  observation.** No profile pair in reach has blacks close enough to
  trigger it. **Recorded at that strength and no higher** — this is the
  same discipline NA-006's corpus-derived bound is held to.
- **`ICC_Spec` §7.2's list of unattributed constants does not contain
  it**, because that list was drawn from `cmssamp.c` (the estimation)
  and this constant is in `cmscnvrt.c` (the linking). **A corpus row is
  owed**, and it is the second time this Pass that a real lcms2 constant
  turned out to be filed under the wrong half of BPC.

**Why it matters later**: a future comparison whose two blacks happen to
be close will show iccce doing BPC and lcms2 doing nothing, and the
residual will look like a defect in one of them. **It will be this
threshold**, and this row is what a reader will find.

#### 3.12.8 ★ NC-094 — A41 priced in a pipeline, corroborating the corpus by an independent route

The corpus (`icc__ref__bpc.md` §3) derived **in Python, in two
independent passes**, that using ICC.1 Table 16's printed `PCSXYZ`
decimals instead of the triple lcms2 **and ICC's own iccDEV** both use
costs `ΔL* = 0,005 3` and `ΔE76 = 0,037 437`. Rebuilding the map with
Table 16's triple and evaluating it on this grid's PCS values — **Rust,
a different pipeline, a fixture's stored bytes** — gives:

| | corpus (Python, two passes) | Pass 5 (Rust, through a fixture) |
|---|---|---|
| ΔL* | 0,005 3 | **0,005 364** |
| ΔE76 | 0,037 437 | **0,037 416** |
| ΔE2000 | *(never computed)* | **0,050 201** |

**Corroborated to 2×10⁻⁵ ΔE76 by an independent route** — and this is
**the verification loop running the direction it ran at Pass 1**, when a
unit test caught an arithmetic error *in the corpus*. It did not this
time; the corpus was right.

**The ΔE2000 is new, and it carries a warning the corpus's framing does
not.** At **0,050** it is the **same order as §B's entire agreement
budget** (5×10⁻²). That does not contradict the corpus's *"invisible at
16-bit"* reading — both triples still encode to the same `u16` codes —
**it complements it: on a float path the choice of digits is not
negligible against the measurement noise**, and a difftest that adopted
the specification's printed decimals would carry a residue of exactly
that size, permanently, with no defect anywhere. **See the dated note
under NA-010**, whose cost this measures.

#### 3.12.9 NC-103 / NC-104 — two refusals, **graded** rather than reported

A refusal is behavioural, not numeric, and §3.7.6 already holds a block
of them. **These two get NC numbers because they are graded**, and
because the property they assert is one iccce claims out loud: **a build
that quietly substituted a zero black for an unestimable one would
produce plausible colour and pass every other row in this section.**

- **NC-103 (S5)** — a v2 CMYK `prtr` destination at media-relative is
  exactly where lcms2 runs the least-squares quadratic fit whose
  mathematics Maria 2013 forwards to the ToS-barred `AdobeBPC.pdf`
  (**A42**) and whose **six thresholds are unattributed even in lcms2's
  own source**. iccce prints *"black point not estimable within iccce's
  named subset (A42); refused, not guessed"*.
- **NC-104 (S6)** — the one exclusion Pass 5 can cite a **published**
  source for: Maria 2013 §4.1, verbatim, *"absolute colorimetric intent
  … does not apply"*. **BPC presupposes both media whites already at
  D50 — the exclusion and the D50 anchoring are the same fact.** Both
  its profiles are committed fixtures, so it is one of only two Pass 5
  rows that run on a machine with no colour directory.

**★ The needle is the exact wording, not the word "refused"** — a loose
needle would let an ICC-absolute row pass on an estimation-subset
refusal and nobody would know. *(The first draft used a paraphrase and
S6 failed; the failure was the needle, not the engine, and
`icc-conformance` recorded it rather than quietly rewriting it.)*

#### 3.12.10 What §3.12 does **not** claim

- **That either black-point ESTIMATOR is correct, or that they agree.**
  §3.12.3. This is the sentence most likely to be lost in a summary.
- **That "iccce's BPC matches lcms2's"** without its scope. It matches
  **on one synthetic v4 fixture against one system profile, at one
  intent, in two directions, on one platform, at one pin**, on the
  **map** and the **pipeline**.
- **That BPC is right.** There is **no BPC conformance test with a fixed
  expected value** anywhere — the same standing as perceptual under
  **A27**. NC-084 … NC-086 grade **the map** against a clause and a
  paper; nothing grades the **result**.
- **That lcms2 is wrong to force BPC, or that iccce is right not to.**
  **NC-100 is REPORTED, NOT GRADED**, and it stays that way until
  `AdobeBPC.pdf` / ICC WP40 / ISO 18619 is read.
- **That the 0,41 `L*` threshold was observed.** It was **solved for**.
- **That any of these rows ran on today's code, or that the suite was
  green when they were committed.** Two commits this session claimed a
  green suite falsely (§2.8). **The whole-suite `pass=90 fail=0` is
  reported and is the only gate figure this section rests on** — and
  §16 carries no Pass 5 runner line of its own.

### 3.13 ★★ Pass 6 + Pass 7 — the compiled path, the spot-colour path, and the ledger's first rows that are not correctness evidence of any kind

**Read §2.9 before quoting anything here**, in particular its rows about
the date correction, the commit-count discrepancy, and the fact that the
raw `iccce bench` output was never filed. **Eight rows, NC-105 …
NC-112.** **The run is `icc-engineer`'s; this librarian ran nothing.**

> **★ SHARED COVERAGE — part of every claim in this section, and it is
> narrow.**
> **Pass 6 (NC-105 … NC-110)**: **one machine** (Windows 11 Pro
> 10.0.26200 x86-64, MSVC **release**), **one run**, **no repetition and
> no variance**; **one direction and one tag-type pair** — SWOP `A2B1`
> (**`mft2`**, 4-D) → the system sRGB profile (**matrix/TRC**), at
> **media-relative**, on a **17-point** grid; the sensitivity control on
> a **different** pair (sRGB → AdobeRGB, matrix/TRC both sides).
> **Pass 7 (NC-111, NC-112)**: **one committed synthetic fixture** this
> project authored, **one destination** and it is **matrix/TRC**, the
> **`Lab ` arm only** of the two encodings Table 66 permits.
> **NOT covered, and these are the sentences that matter:** **no other
> implementation was run for either Pass** — **lcms2 was never timed and
> never asked to resolve a spot**, so **nothing here is a cross-check**;
> **no ΔE of any kind was computed** in either Pass; **no `cargo test`
> outcome exists**; **no LUT or gray destination was reached from a spot
> colour**, though both are reachable; **no real vendor `ncl2` profile
> was parsed by anything**; **BPC was not folded into the benchmarked
> chain**; **no B2A direction was compiled**; and **no published value
> exists for anything in this section** — for timings because the notion
> is meaningless, for the spot path because nobody has printed one.
> **Pass 6 and Pass 7 have no ground-truth row, and neither does any
> other Pass.**

#### 3.13.1 The eight rows, at a glance

| ID | What | Class | Tolerance | Observed |
|---|---|---|---|---|
| **★ NC-105** | **Pass 6 done-when, clause 1** — 300 DPI A4 CMYK→RGB through the **compiled** path: **8 700 867 px** (2481 × 3507) | **machine-timing** | **none — a STATED TIME is the clause; there is no gate** | **7.23 s = 1.20 Mpix/s** |
| NC-106 | **Grid build**, paid once: **83 521 chain evaluations** (17⁴) | machine-timing | none | **1.04 s** |
| NC-107 | **The reference (uncompiled) path** over the same raster, timed **in-process**, and the ratio | machine-timing | none | **0.084 Mpix/s**; **speedup 14.4×** *(and see §3.13.2 — the ratio does not reproduce from the two rounded figures)* |
| **★★ NC-108** | **Pass 6 done-when, clause 2** — compiled vs reference, **maximum OFF-NODE** difference. Direction and tag type named (DL-021): SWOP `A2B1` `mft2` 4-D → sRGB matrix/TRC, media-relative, 17-point grid | **self-consistency** — both arms are iccce | **∞ in the ledger's sense: the clause asks for a MEASUREMENT, not a bound.** The unit test's own gate is **2×10⁻² device** | **0.003589 device units** |
| **★★ NC-109** | **THE SENSITIVITY CONTROL (DL-018)** — `h²` scaling of the off-node error under a halved grid spacing, sRGB → AdobeRGB, probes in the smooth region `[0.2, 0.9]` | self-consistency *(an instrument check, not a claim about colour)* | **ratio ∈ [2.0, 8.0]** | **in band** *(the passing ratio was not carried)*. ★ **Its two FAILING drafts are the row's real content: 1.1×10⁻¹⁵ at ratio 0.94** (identity fixture) and **ratio 1.44** (across the TRC breakpoint) |
| NC-110 | **The node-identity check** — compiled and reference agree at a grid node | **self-consistency, and NULL BY CONSTRUCTION** | 1×10⁻¹² *(a structural gate)* | **passes — and it is NOT EVIDENCE.** The node's value **is** a reference evaluation. Filed so the identity is on record as something that could not have failed |
| **★ NC-111** | **Pass 7** — **every** spot in `fixtures/synthetic/v2-ncl2-named.icc` resolved into the **real system sRGB** profile; each output channel in `[0.0, 1.0]` | self-consistency (behavioural) | **range, not a number** | **all resolved, all in gamut** |
| NC-112 | **Pass 7** — an unknown spot name yields **`None`** (the `/Alternate` signal), not a guess | self-consistency (behavioural, 0/1) | **exact behaviour** | **`None` as required** |

**Record arithmetic.** Pass 6 = 6 (NC-105 … NC-110); Pass 7 = 2
(NC-111, NC-112). **Total 8.** ★ **Unlike §3.12, this total reconciles
against nothing** — `tools/difftest` did not run for either Pass, so
there is no whole-suite figure to subtract and **no runner outcome of
any kind to check these eight against.** That is stated rather than
papered over.

#### 3.13.2 ★★ The transcription-precision note — the one place these rows are weaker than they read

**`iccce bench` prints twelve `key: value` lines** *(verified — the
`println!` block read in `cmd_bench`)*, including
`convert.seconds` at **6 decimals**,
`throughput.megapixels_per_second` and
`reference.megapixels_per_second` at **3**,
`speedup.compiled_over_reference` at **2**, and
`error.max_device_offnode` at **9**.

**None of those lines is on record anywhere.** The dispatch carried
four figures at four different precisions, and they do not close:

- `8 700 867 / 7.23 = 1.2034` Mpix/s ✔ — consistent with the quoted
  **1.20**.
- **`1.2034 / 0.084 = 14.3`, and the dispatch says 14.4×.** This is
  **not** an error: `cmd_bench` divides the *unrounded* values, and
  `reference.megapixels_per_second` printing `0.084` means the true
  value lies in `[0.0835, 0.0845)`, which puts the ratio anywhere in
  **`[14.24, 14.41]`**. **14.4 is reachable only at the very bottom of
  that band**, and `7.23` is itself a rounding of a 6-decimal print,
  which widens it further.
- **`0.003589`** is quoted to 6 decimals where the program prints 9.

**What follows, and it is small but real.** A reader cannot reproduce
14.4 from the other quoted figures, and cannot tell whether the error
row's 7th–9th decimals were zero. **The remedy is twelve lines of text
that nobody filed**, and it is owed in §7.9. **Nothing here suggests a
figure is wrong** — it suggests that the *evidence* for these rows is a
transcription of a transcription, which is exactly the distinction §1.1
was written to keep visible.

#### 3.13.3 ★★ NC-108's class was judged, not assumed — and why a `self-consistency` row can still answer a done-when

The tempting sentence is *"the compiled path agrees with the reference
path to 0.0036 device units, so compilation is safe."* **Two things are
wrong with it.**

1. **Both arms are iccce.** §1: `self-consistency` is *"the only way to
   price an approximation. Worthless as correctness evidence."* **A grid
   built by sampling a wrong reference path reproduces the wrong answer
   to 0.003589 device units and this row is unmoved.** `cmd_bench`
   prints that sentence in its own output — `error.class:
   self-consistency (compiled vs reference, same code; worthless as
   correctness evidence — NUMERIC_CLAIMS.md §1)` *(verified — read)* —
   which is the right place for it, because the person most likely to
   quote a number out of context is the person running the tool.
2. **0.003589 is a DEVICE unit, and no perceptual translation of it
   exists.** Converting it to ΔE by intuition is precisely what
   **DL-004** forbids; the conversion factor depends on where in the
   destination's tone curve the error lands, and Pass 4b measured that
   the same device figure can mean wildly different ΔE at different ends
   of an axis (`TOLERANCES.md` §4's row C3 is the worked instance).
   **Owed.**

**So why is the row allowed to answer a done-when at all?** Because the
clause asks for something weaker and more honest than correctness: *"the
compiled path's error against the uncompiled one is **measured and
stated**."* **Pricing a deliberate approximation is exactly what
`self-consistency` is for** — the same footing as NA-006's interpolation
cost. **What the class forbids is the second sentence**, the one that
turns a price into a warrant.

#### 3.13.4 ★★ NC-109 and NC-110 — the pair that makes NC-108 mean anything, and the pair a reader should look at FIRST

**These two rows are not supporting material. They are the Pass.**

**NC-110 is null by construction, and is filed BECAUSE it is.** The
compiled grid is built by evaluating the reference chain at each node,
so **at a node the two arms are the same number by construction** — the
node's stored value *is* a reference evaluation. The test
`identical_at_nodes_by_construction` asserts that identity and its own
doc comment reads **"STRUCTURAL, NOT EVIDENCE … It must never be cited
as the compiled path's error"** *(verified — read)*. Its purpose is to
catch a **transposed indexing convention** — the first channel varies
slowest, matching the CLUT evaluator (corpus **A20**) — and nothing
else. **This is the same posture as NC-101 in Pass 5:** an exact zero,
labelled null, rather than an exact zero quoted as agreement.

**NC-109 is the reason NC-108 is a measurement.** **DL-018**'s rule is
that an upper bound on a *deliberate* cost must be paired with something
showing the instrument could have failed. `n`-linear interpolation error
scales as `h²`, so halving the spacing must cut the off-node error
roughly 4×; the gate is **2×–8×**, wide enough for a real curve's
varying curvature and narrow enough to fail if the compiled arm were
secretly the reference arm (**ratio → 1**) or the probes were
accidentally on-node (**both errors zero**).

**★ And the control caught its own instrument, twice, before it worked.
Both failures are in the test's doc comment** *(verified — read)*:

| Draft | What it returned | What was actually wrong |
|---|---|---|
| **1** — fixture **sRGB → sRGB** | **1.1×10⁻¹⁵**, ratio **0.94** | **The fixture nullified the control.** A grid does not merely match an identity chain at its nodes — **`n`-linear interpolation is exact on a linear function, so it matches it everywhere.** `f64` noise, no `h²` scaling, no discrimination. **Without the control, that 1.1×10⁻¹⁵ would have been reported as the compiled path's cost** |
| **2** — fixture sRGB → **AdobeRGB**, probes across the whole axis | ratio **1.44** | **Neither the code nor the fixture.** sRGB's TRC joins a linear segment to a power curve at **0.04045**; interpolation error across a **derivative discontinuity** scales `h¹`, not `h²`. **A correct control disagreeing with an incorrect expectation** — the same shape as `TOLERANCES.md` §4's corrected derivations, and **not** a tolerance being widened. Fixed by probing the **smooth region** `[0.2, 0.9]`, off-node for both the 5- and 9-point grids |

**★ DL-023 predicted this Pass's trap by name at the previous filing** —
the ROADMAP's second Pass 6 annotation calls a compiled-vs-reference row
*"the most likely null-by-construction row this project will ever
write"* — **and it was walked into anyway.** That is the argument for
mechanical controls over remembered rules, and it is filed as
**`ARCHITECTURE.md` DL-025**, together with the observation that this is
the **third** instrument in two days to catch something a competent
engineer was about to ship (after **DL-016** and **DL-020**).

**One thing NOT on record:** the **passing** ratio. The dispatch carried
the two failing values and not the value the control now returns. **A
control whose passing margin is unknown is a control nobody can tell is
near its band edge.** Owed in §7.9.

#### 3.13.5 ★ NC-107 — a discarded measurement that belongs in the ledger

**A first attempt at the reference timing measured the CLI end to end
and reported ≈49 000 px/s.** That number is **stdio text parsing**: the
`transform` subcommand reads and writes decimal text per pixel. It is
not a measurement of either transform. The reference path is now timed
**in-process** over a bounded prefix (`pixels.min(100_000)`) of the
**same** raster, and `cmd_bench` carries the reason in a comment
*(verified — read)*.

**Why a thrown-away number gets a subsection.** ≈49 k px/s and 84 k
px/s are the **same order of magnitude**. A speedup quoted against the
wrong denominator would have been **≈24× instead of 14.4×**, and
**nothing about 24× would have looked wrong.** This is project rule 1
wearing a stopwatch: *a wrong measurement looks exactly like a right
one.* The ledger records the discarded figure so that a future reader
who finds "49k" in a commit message or a scrollback knows what it was.

#### 3.13.6 ★ NC-111 / NC-112 — what the spot-colour rows actually establish, and the cross-check that was available and not taken

**What they establish.** That `NamedColors::resolve_to_device` executes
end to end on real bytes — a committed `ncl2` fixture into a real
system profile — and returns device values in range; and that an unknown
name returns `None` rather than a colour. **Before this, `NamedColors`
was referenced by nothing outside its own file**, a finding this project
filed at the Pass 4b annotation and repeated at the Pass 5 filing.
**That finding is closed.**

**What they do not establish, and the boundary is sharp.** **Neither row
contains an expectation from outside iccce.** NC-111 asserts a
**range**, not a colour: a resolution that was wrong by 10 ΔE but still
in `[0, 1]` passes it. **No spot colour's resolved value has ever been
compared to anything** — not to lcms2, not to a published value, not to
the profile's own stored device coordinates.

**★ And that last one was available.** An `ncl2` entry carries
`nDeviceCoords` — **the device values the profile's author recorded for
that spot** — alongside its PCS coordinates. Resolving a spot into *the
spot's own profile's* device space and comparing against those stored
values would be an expectation **written by someone other than iccce**,
on bytes iccce did not choose. **It is the cheapest genuine cross-check
on this path and it was not run.** Owed in §7.9.

**What is right about the design, and it is why the Pass is short.**
`resolve_to_device` → `Chain::convert_pcs_to_device` →
`Chain::pcs_to_destination`, **which is the same method
`Chain::convert` uses for its own destination half** — the duplicated
arm was removed in this commit *(verified — `Chain::convert` ends
`self.pcs_to_destination(xyz)`)*. So a spot inherits the sourced 8.10.2
fallback, the same model selection, the same refusals and the same
clamping **by construction**. The failure mode that a private path
invites is not a crash: it is a `Separation` rendering 0.4 % off from
every other object on the page, on some profiles only. **The module's
own sentence is the justification — spot colours are brand matching,
*"the least acceptable place in the whole system for a sub-perceptual
defect."***

**And the encoding is asserted the way DL-005 and DL-016 require**:
`0xFF00/0x8000/0x8000` must decode to `Lab(100, 0, 0)` **exactly**,
because the wrong (v4) decode gives **99.6109** — **invisible to any ΔE
gate, fatal to a brand colour** *(verified — read)*. **Never assert this
with ΔE.**

#### 3.13.7 What §3.13 does NOT claim

- **That the compiled path is correct.** Both arms are iccce (§3.13.3).
- **That 1.20 Mpix/s is a property of iccce.** It is a property of
  **iccce on one machine, in one build, on one run**, and
  **`machine-timing` exists to stop it being quoted otherwise.**
- **That 14.4× says anything about lcms2.** **lcms2 was never timed.**
  The ratio is iccce against **iccce**.
- **That 0.003589 device units is a small colour error.** **No ΔE
  translation has been measured.** DL-004 forbids supplying one by
  intuition.
- **That the sensitivity control passes comfortably.** **The passing
  ratio was not carried** (§3.13.4).
- **That a spot colour resolves *correctly*.** NC-111 asserts range.
- **That Pass 7 discharges the `ncl2` behavioural test owed since Pass
  2.** **NC-019's coverage line still rests on a source reading**, and a
  consumer existing does not change that.
- **That any test in either Pass passes.** **No `cargo test --workspace`
  outcome has been reported at any of the last seven filings**, and this
  project has already shipped two commits whose messages claimed a green
  suite while a test was red.
- **That either Pass ran on today's code, or that these numbers came
  from the committed build.** **The commits' contents are unverified**
  (§2.9).

---

#### 3.13.8 ★ Dated correction, 2026-08-12 — **the CLI's help text was wrong twice, in the SHIPPED, PUBLIC binary**, and one of the two errors is a number this section carries

**Neither figure below is a measurement. Both are things the program
told its users**, which is why they belong beside the rows they
misreport rather than in a code-review note.

`crates/iccce-cli/src/main.rs`'s `bench` help block said:

1. **"2481x3507 = 8,700,267 px"**. **2481 × 3507 = 8 700 867.** The `867`
   figure is the one **NC-105 carries**, the one `pass6.rs`'s
   `BENCH_PIXELS` computes as `2481 * 3507` *(verified — read)*, and the
   one the bench output prints. **The CLI help was the lone outlier
   across four places**, which is exactly why nobody caught it: a wrong
   digit in the one place that is prose rather than arithmetic.
2. **"a 17-point grid"**. `iccce_cmm::compiled::recommended_grid_points`
   returns **33** for 3- and 4-channel and **129** for 1- and 2-. The
   default moved to 33 in commit **`189e732`** — *"Pass 6 gate: default
   grid 17 -> 33, because the number would not move"* *(verified — the
   commit's subject line read; and `pass6.rs`'s `DEFAULT_GRID` doc
   comment records the same move and the same hash)* — **and the help
   text never followed.**

**Fixed by `icc-conformance`, and the fix is in the working tree**
*(verified — read: the block now says `8,700,867 px` and *"the
recommended grid for the source's channel count (33 for 3- and
4-channel, 129 for 1- and 2-)"*)*. **It is NOT committed** — see §2.10's
commit row.

> **★ Why this is filed in the ledger and not shrugged off.** The second
> error is the interesting one, and it is a **documentation instance of
> the exact hazard `pass6.rs` built a test to catch in the code.**
> `APPARATUS_BENCH` exists so that when the shipped default grid moves
> and the harness's copy of it does not, a row **fails loudly** — and it
> did, at 1.576×10⁻³, which `pass6.rs` records as *"not an error but the
> gap between the two grids' costs"* *(verified — read)*. **The help text
> is a third copy of the same constant with no such gate on it**, and it
> silently disagreed with the binary it documents. **The project is
> public** (DL-024), so this was a wrong statement made to strangers
> about a program's default behaviour. **The generalisable form:** a
> constant duplicated into prose is still a duplicated constant, and
> prose has no test.

### 3.14 ★ Pass 4 item 1 — **the SATURATION table (`B2A2`) in the B2A direction. Measured, and it had ALREADY been measured: what was missing was the filing, not the measurement**

**Read §2.10 first**, in particular the commit row (**there is no commit**)
and the suite-in-flux row. **Six rows, NC-113 … NC-118.** Apparatus: a
`(Intent::Saturation, tag::B2A2)` extension to the **pre-existing**
`tools/difftest/src/pass4b.rs` §A. `TOLERANCES.md` **§3.4.4.6** is
`icc-conformance`'s record of the same run and is the source these rows
were written from *(verified — read; this ledger did not compute them)*.

> **★★ THE CORRECTION THIS SECTION EXISTS TO MAKE, and it is about the
> project's own bookkeeping rather than about colour.**
>
> **`NEXT_SESSION.md` §3 says of this item: *"Cheap, unblocked, never
> run."* `ROADMAP.md`'s "what remains" block says *"Cheap, unblocked,
> and nobody has run it."*** *(both verified — read.)*
>
> **It had been run.** A prior `icc-conformance` session on **the same
> calendar day, 2026-08-12**, wired the saturation intent into
> `pass4b.rs` §A, measured it, and `icc-conformance` **wrote the full
> result into `TOLERANCES.md` §3.4.4.6** — six graded rows with
> justifications, a coverage paragraph, and two `§4` change-table rows.
> The session that produced this filing **re-ran it and reproduces every
> figure**.
>
> **The cause is recorded in the very document that carried the error.**
> `NEXT_SESSION.md` §4 and §7.9 both state that at the Pass 6 + Pass 7
> filing **`tools/` was deliberately not re-read**, because
> `icc-conformance` was working there in parallel — and §7.9 item 8 says
> in as many words that *"two carried items is one more than last
> time."* **The measurement was never missing. The filing was.**
>
> **The shape is worth more than the instance**, and this project has now
> seen it twice in two days: at the sibling-facing level, `CLAUDE.md`'s
> XFA item asked a question one of its own corpus files had already
> answered. Here, a handoff told the next session to run an experiment
> that a finished, documented run had already completed. **Both are the
> same failure: a finding that did not propagate to the document that
> needed it.** The remedy is not more diligence, it is the mechanical
> one — **grep the corpus before recording something as owed**, and
> treat *"carried, not re-verified"* as the loud warning §7.9 already
> labelled it.
>
> **★ And keep the two saturation items DISTINCT, as the handoff itself
> correctly insists.** *This* item is an **evaluation** gap, and it is
> now closed. **Pass 5's saturation gap is a *capability* gap** in
> iccce's BPC estimation subset, and **nothing here touches it.**

#### 3.14.1 The six rows

| ID | What | Class | Tolerance | Observed |
|---|---|---|---|---|
| **★ NC-113** | **THE PRECONDITION** — the three `B2A*` tags are three **distinct** blocks of file bytes. `pass4b/srgb-to-swop/b2a-tags-are-three-distinct-tables` | self-consistency | **0.0 — exact** (a count of integer comparisons on raw bytes, no parser in the way) | **0.000000e0**. `B2A0`@83 392, `B2A1`@228 980, `B2A2`@374 568, each 145 588 B; differing bytes **71.4 % / 66.2 % / 70.4 %** pairwise |
| **NC-114** | **Saturation, device space** — `…/saturation/device-vs-lcms2` | **implementation-cross-check** | **5×10⁻⁴** (`DEVICE_B2A`, **reused unchanged**) | **1.550000e-4** — **99.8 % of the computed 1.5525×10⁻⁴ envelope** |
| **NC-115** | **Saturation, the attribution** — `…/saturation/device-lcms2-arithmetic-modelled` | implementation-cross-check, on a modelled destination | **5×10⁻⁵** (`DEVICE_B2A_MODELLED`, unchanged) | **3.098962e-5** = **2.03 lsb of 1/65535** — the same figure as perceptual, media-relative and the PCS row to three significant figures |
| **NC-116** | **Saturation, ΔE2000 round trip** — `…/saturation/roundtrip-lab-de2000` | implementation-cross-check | **5×10⁻²** (`DE_B2A_ROUNDTRIP`, unchanged) | **7.062753e-3** |
| **NC-117** | **Saturation, apparatus** — `…/saturation/apparatus-lut8-matches-iccce-cmm` | self-consistency | **1×10⁻⁹** | **0.000000e0** — bit-identical |
| **NC-118** | **Saturation, the sensitivity control** — `…/saturation/counterfactual-tetrahedral` | implementation-cross-check | **∞ — REPORTED, NOT GRADED** | **2.960041e-2** = **191×** the observed residual |

#### 3.14.2 ★ NC-113 is the row that makes the other five measurements

**Saturation had been out of scope on a sentence, not on a reason.**
`pass4b.rs` §A's own doc comment said *"saturation adds a third copy of
the same shape"* — **an assumption**, and `TOLERANCES.md` §3.4.4.6 says
so in those words *(verified — read)*.

**In the A2B direction of this same file that assumption is TRUE**, and
this ledger already records it: **NC-052** grades `A2B0` and `A2B2` as
**one block of tag data at one offset**, at tolerance `0.0` exact, which
is why `pass4/swop/perceptual-equals-saturation` observes zero. **Had
`B2A0`/`B2A2` been laid out the same way, NC-114 … NC-118 would have
reproduced the perceptual rows bit for bit** and the suite would have
gained five green lines that measured nothing.

**They are not**: the least-distinct pair still differs in **two thirds
of 145 588 bytes**. **A null that would have been null by construction
was identified BEFORE it was collected**, which is DL-025's requirement
applied one Pass earlier than DL-025 was written, and it is the third
consecutive filing at which a precondition row has done more work than
the rows it precedes.

#### 3.14.3 What §3.14 does **not** claim

- **Not that saturation is verified.** **One** profile pair
  (`sRGB → USWebCoatedSWOP`), **one** tag type (`mft1`/`lut8`, 3→4,
  33 nodes), **213** RGB points, one machine, one pin, Windows/MSVC.
- **Nothing about saturation in the A2B direction**, where this file
  aliases `A2B0`/`A2B2` and the intent is therefore **untested by
  construction** (NC-052).
- **Nothing about saturation through a v4 `mAB `/`mBA ` pipeline.**
- **Not a ground-truth row.** NC-114 … NC-116 and NC-118 are agreement
  with lcms2 at a pin; NC-113 and NC-117 are self-consistency.

### 3.15 ★★ Pass 4 item 2 — **ICC-absolute through a LUT destination, with lcms2's `wtpt` substitution held at ZERO by the choice of profile pair.** The ledger's answer to the question NC-053 could not ask

**Read §2.10 first.** **Ten rows, NC-119 … NC-128.** Apparatus:
`tools/difftest/src/pass4c.rs` (**new, untracked**), 10 records, **all
pass**, reproduced bit-identically across two runs *(reported)*.
`TOLERANCES.md` **§3.4.5** is `icc-conformance`'s record of the same run
and is the source these rows were written from *(verified — read)*.

> **★★ THE METHOD IS THE FINDING, and it generalises past this Pass.**
>
> NC-053 measured **11.217 ΔE2000** at the ICC-absolute intent and
> attributed it to a **policy**: `cmsio1.c`'s `_cmsReadMediaWhitePoint`
> substitutes D50 for a stored `wtpt` when a profile is **version < 4
> AND class `'mntr'`**. Under **DL-019** the raw comparison was
> **REPORTED, NOT GRADED**, and the gate moved to **NC-054**, *a model*.
>
> **A model can absorb a genuine arithmetic error along with the policy
> difference it was built to isolate**, and until today nothing in the
> suite could tell the two apart. So the ICC-absolute path's only gate
> was a quantity that could not fail for the right reason.
>
> **What changed is not a document.** It is the recognition that lcms2's
> predicate is a **CONJUNCTION**, so breaking *either* half on *both*
> profiles makes the policy difference **structurally absent** rather
> than modelled or tolerated:
>
> | role | profile | version | class | which half fails |
> |---|---|---|---|---|
> | source | `fixtures/synthetic/v4-rgb-matrix-trc.icc` | **4.4.0** | `'mntr'` | the **version** half |
> | destination | `USWebCoatedSWOP.icc` | 2.1.0 | **`'prtr'`** | the **class** half |
>
> **Each profile fails a DIFFERENT half**, so the pair does not rest on
> one property — a point worth keeping, because a pair that failed the
> same half twice would be one profile-header edit away from measuring
> the policy again.
>
> **The second Pass 4 confound is zero here too**: lcms2 forces
> trilinear for any Lab-PCS output LUT, trilinear over three inputs
> **is** iccce's n-linear (**NA-006 = 0 here**, by NC-067's finding),
> and the source has no CLUT at all.
>
> **★ So the item was never blocked on a document. It was blocked on a
> PROFILE PAIR — and the pair was sitting in the committed fixture
> corpus the whole time.** `ROADMAP.md` and `NEXT_SESSION.md` carried it
> for three filings as *"blocked on a document only the operator can
> fetch"*, and then, once **A4b resolved**, as *"unblocked, the
> arithmetic can be measured now"*. **Both framings were about the
> wrong object.** The generalisable lesson, and it is the one to carry
> into Pass 8: **when a comparison is confounded by an implementation's
> conditional behaviour, read the CONDITION. If it is a conjunction, the
> confound may be removable by choosing inputs rather than by resolving
> the disagreement.**

#### 3.15.1 The ten rows

**§A — the confound-free pair** (`v4-rgb-matrix-trc.icc` →
`USWebCoatedSWOP.icc`, **729** RGB points = 9×9×9 on the 8-bit lattice
at levels 0/32/64/96/128/160/192/224/255, destination `B2A1`
`mft1`/`lut8` 3→4 33³, `-c0`, **no BPC either side**):

| ID | What | Class | Tolerance | Observed |
|---|---|---|---|---|
| **★ NC-119** | **THE PRECONDITION** — neither profile trips lcms2's `wtpt` gate. `pass4c/v4matrix-to-swop/precondition-neither-profile-trips-lcms2-wtpt-gate` | self-consistency | **0.0 — exact** (a count over 2 parsed headers, not a float) | **0.000000e0**. src `v04400000` `'mntr'` wtpt = (0.9642, 1.0000, 0.8249) = **D50 exactly**; dst `v02100000` `'prtr'` wtpt = (0.7084, 0.7359, 0.5710) |
| **★★ NC-120** | **THE MEASUREMENT** — ICC-absolute, device space. `…/absolute/device-vs-lcms2` | **implementation-cross-check** | **5×10⁻⁴** (`DEVICE_B2A`, **reused unchanged**) | **8.900000e-5** |
| **NC-121** | ICC-absolute, device **mean**. `…/absolute/device-mean` | implementation-cross-check | **∞ — reported** | **1.830178e-5** |
| **★★ NC-122** | **THE FLOOR** — media-relative on the **same pair and grid**. `…/media-relative/device-vs-lcms2` | **implementation-cross-check** | **5×10⁻⁴** (same constant) | **1.080000e-4** |
| **★★ NC-123** | **The counterfactual — EXACT, not modelled.** `…/absolute/counterfactual-wtpt-substituted` | self-consistency | **∞ — REPORTED, NOT GRADED** | **2.055760e-1** |
| **NC-124** | **The sensitivity floor** (DL-018/DL-025). `…/absolute/sensitivity-floor` | self-consistency | **0.0** on `max(0, 100 − r)` | **0.000000e0**; observed ratio **r = 2310×** |
| **NC-125** | **The degeneracy guard** — fraction of grid points the absolute scaling did **not** move. `…/absolute/degeneracy-guard-unmoved-fraction` | self-consistency | **0.05** | **1.371742e-3** = **1 point of 729** |

**§B — the same policy in the OTHER direction** (system sRGB `.icm` →
`USWebCoatedSWOP.icc`, same grid, **the v2 `'mntr'` profile now as
SOURCE**):

| ID | What | Class | Tolerance | Observed |
|---|---|---|---|---|
| **NC-126** | **The precondition, inverted** — **exactly one** profile trips the gate. `pass4c/srgb-to-swop/precondition-source-DOES-trip-lcms2-wtpt-gate` | self-consistency | **0.0 — exact** on `\|count − 1\|` | **0.000000e0** (src gate = true, dst gate = false) |
| **★ NC-127** | ICC-absolute with the source-side substitution **live**. `pass4c/srgb-to-swop/absolute/device-vs-lcms2` | **oracle-behaviour-at-pin** | **∞ — REPORTED, NOT GRADED** | **2.134240e-1** — **1654×** its own floor |
| **NC-128** | **§B's floor** — media-relative, same pair and grid. `…/media-relative/device-vs-lcms2` | implementation-cross-check | **5×10⁻⁴** (same constant) | **1.290000e-4** |

**Record arithmetic.** §A = 7 (NC-119 … NC-125); §B = 3
(NC-126 … NC-128). **Total 10**, matching the reported record count and
the ten ids read in `pass4c.rs` *(verified)*.

#### 3.15.2 ★★ NC-120 is BELOW NC-122, and that relation — not the number — is the claim

**Never quote 8.90×10⁻⁵ without 1.08×10⁻⁴ beside it.** A bare *"agrees
to 9×10⁻⁵"* says only that a number is small, and this ledger has
`self-consistency` rows four orders of magnitude smaller that establish
nothing.

**The claim is relational.** NC-122 is media-relative — **an intent with
no absolute scaling in it at all** — on the **same two files**, the
**same 729 points**, through the **same destination table**, by the
**same evaluator**. It isolates what this direction costs before the
ICC-absolute arithmetic is added: the **8-bit `lut8` quantisation
envelope** Pass 4b priced at **1.330×10⁻⁴ / 1.5525×10⁻⁴ / 9.602×10⁻⁵**
across its three intents (NC-063, NC-114's envelope, and the perceptual
figure).

**NC-120 (8.90×10⁻⁵) lands BELOW NC-122 (1.08×10⁻⁴) on that same pair.**
What that supports, stated exactly: **the ICC-absolute arithmetic adds
nothing detectable above the cost the direction already carries.** It
does **not** support *"the absolute path is correct"* — see §3.15.5.

**And no fresh tolerance was minted.** All four graded device rows
(NC-120, NC-122, NC-128, and §B's floor) **reuse `DEVICE_B2A` at
5×10⁻⁴ unchanged**, because they end in the same destination table by
the same evaluator in the same direction, so the envelope transfers with
its justification intact. **A constant fitted to this observation would
have been a number chosen because it passed** — §3.14's saturation rows
set that precedent one section earlier, where the constant stayed put
and only the `why` string moved.

#### 3.15.3 ★★ NC-123 is EXACT rather than modelled, and that is the row's headline

Every previous attempt to price lcms2's `wtpt` substitution has been a
**model**: NC-054 re-predicts lcms2's output *with the substitution
applied* and grades what is left. **NC-123 does not model anything.**

**Because the source profile's stored `wtpt` IS D50 — exactly, to the
four-figure ICC triple (0.9642, 1.0000, 0.8249) — substituting D50 for
the *destination's* `wtpt` collapses the whole `ICC.1:2022` 6.3.2.2
scaling diagonal to the identity.** Therefore **`absolute` vs
`media-relative` on this pair *IS* the NC-053 substitution priced on
this pair**, computed rather than assumed. **2.055760×10⁻¹.**

**NC-124 is what stops NC-120 being a magnificent measurement of
nothing** — the DL-025 obligation, discharged: sensitivity **2310×**,
against a graded floor of **100×**. **The floor is transcribed, not
chosen.** It comes from Pass 4b's already-accepted counterfactual band
on this same table and direction — **99×, 139×, 191×** (the last being
NC-118) — so **it would have been 100 had the observation been 105×**,
which is the only test that distinguishes a floor from a number fitted
to clear an observation.

#### 3.15.4 ★★ TWO nulls were guarded, not one — and the second is the one nobody asks about

DL-025 taught the project to ask *"what would this comparison return if
the effect were identically absent?"* **NC-124 answers that one.**

**NC-125 answers a different one, and it is the one this project had not
yet named: CLIPPING.** If the absolute scaling had pushed the grid
outside the destination's gamut, **both implementations would clamp to
the same boundary and agree perfectly while computing nothing.** That is
a *second* route to a vacuous green row, structurally unlike the first —
the effect is present, the arithmetic runs, and the comparison still
sees nothing because the output is pinned.

**Counted rather than argued: 1 of 729 points unmoved** — **device
black, the fixed point of any diagonal scaling**. That is arithmetic,
not a defect, and the budget (0.05) sits an order of magnitude above the
one expected fixed point.

**The generalisation for later Passes**, and it belongs beside DL-025's:
*a comparison can be vacuous because the effect is absent, or because
the output is saturated. The first is caught by a sensitivity ratio; the
second is not, and needs its own count.*

#### 3.15.5 ★ NC-127 — the policy is DIRECTION-SYMMETRIC, and the prediction that says so was written BEFORE the run

**This is a DL-021 row.** NC-053 is a fact about the v2 `'mntr'` profile
as **destination**. §B puts the same file in as **source**.

**Pre-registered (DL-023), before anything ran:** the divergence should
move to the source side and stay **large**, order 10⁻¹ device units,
because **iccce scales by `D65/D50 = (0.98579, 1.0, 1.32027)` where
lcms2 scales by identity**.

**It held: 2.134240×10⁻¹, i.e. 1654× its own media-relative floor of
1.290×10⁻⁴.**

**Record that the prediction held, and record why that is not a
formality.** This project's convention — DL-011 → DL-012 — treats a
predicted divergence as **unproven until measured**, because it has
already once predicted an lcms2 divergence in advance and **measured it
absent**. Here the prediction is confirmed, **in the one direction
NC-053 never covered**, and the confirmation is what licenses calling
the substitution a *policy* rather than a destination-side quirk.

**NC-127's class is `oracle-behaviour-at-pin`, not
`implementation-cross-check`**, and the distinction is deliberate: its
content is *what lcms2 does with a v2 display-class `wtpt`*, iccce
having merely supplied the contrast. **NC-128 exists so that
"0.21 device units" has something to be large compared to.** Without a
floor, a big number is as uninterpretable as a small one.

#### 3.15.6 ★★ The DL-019 judgement, made rather than deferred — **NC-053 stays ungraded, and is RE-BASED off DL-019**

**The full reasoning is `ARCHITECTURE.md` DL-026.** What the ledger
records:

| | |
|---|---|
| **NC-053** | **Stays REPORTED, NOT GRADED — and is now PERMANENTLY so, on a new basis.** It is **re-based off DL-019** |
| **NC-054** | **Stays GRADED at 5×10⁻²**, unchanged |
| **Why DL-019 no longer applies** | DL-019 is *"report-not-grade when the mechanism is known and the **authority does not exist**"* — **a HOLDING pattern**. **The authority now exists and has been read.** Leaving NC-053 filed under DL-019 asserts the project is **still waiting for a document**. It is not, and **an unmeasured assertion about the project's own state is precisely the error class this project keeps paying for** |
| **★ What replaced it, and it is STRONGER and PERMANENT** | Sourced by `icc-spec-librarian`, 2026-08-12: **`ICC.1:2022` 9.2.36** gates the `wtpt` rule on **device class, with NO version gate**; **`ICC.1:2001-04` A.3.1.1** gates it on the **adaptation condition**, not on class at all — *"If the viewer completely adapts to the white point of the medium (as is often the case with monitors) this tag should be set to Xi, Yi, Zi"*, where **monitors are the typical case, not the condition**. **So lcms2's `version < 0x4000000 && class == 'mntr'` predicate reproduces NO CLAUSE IN EITHER EDITION.** A.3.1.1 is additionally a **`should`**, and `ICC.1:2001-04` has **no defined verbal-form hierarchy** — deontic weight **qualified** |
| **★ Why a graded row is not merely undesirable but UNAVAILABLE** | The conformance clause (`ICC.1:2022` cl. 5, `ICC.1:2001-04` cl. 3) binds the ability to **READ** profiles. **A CMM's computed output is not constrained by either.** So **neither implementation can be graded against the standard here**, and grading iccce **against lcms2** would mean putting a budget on a quantity nobody controls — which is exactly the *"widen to ~15 ΔE00"* alternative **DL-019 already rejected**, reached by a different route |
| **★ What NC-053 becomes** | **The A16 / NC-056 pattern**: the standard is **SILENT**, two named choices exist, and the outcome is **a DIFFERENCE, not an error on either side**. NC-053 is therefore no longer a *pending* ungraded row awaiting adjudication — **it is a settled one**, and that is a change in kind |
| **★★ WORDING, binding on every document** | Say lcms2 **DIVERGES**. **Never "non-conforming"** — the conformance clause binds reading only. This mirrors the hedge `TOLERANCES.md` **§5.2** already carries for **NA-003 / A39b** *(verified — read)* |
| **★★ The judgement was only DEFENSIBLE because its cost was removed FIRST** | Before today the ICC-absolute path's only gate was **NC-054, a model**. Leaving NC-053 ungraded then would have left the arithmetic unwatched. **NC-120 is a raw, unmodelled, GRADED cross-check of that arithmetic**, so leaving NC-053 ungraded now costs nothing. **Record the dependency explicitly: this judgement is contingent on NC-120 existing.** If NC-120 is ever removed or invalidated (the pin moving would do it), **the judgement must be re-made, not inherited** |
| **★ A4c does NOT ride along** | Whether a profile's `wtpt` must agree with its **own colorants** is a **separate ambiguity and remains SILENT**. The corpus's position is *"Disclosure is the one option ICC.1 does not foreclose"*. **The system sRGB profile is exactly such a self-inconsistent file** — D65 `wtpt` beside D50-adapted colorants — and **Pass 4c neither adjudicates nor repairs it.** **A4c did not clear when A4b cleared, and it does not clear now** |

#### 3.15.7 ★ A citation correction with reach well beyond this Pass

**iccce's docs and code cite the ICC-absolute conversion as *"Annex D,
D.6/D.7"*.** Per `icc-spec-librarian`, 2026-08-12:

- **Annex D is the INFORMATIVE restatement.** The **normative** statement
  is **`ICC.1:2022` clause 6.3.2.2, Equations (4)–(6)**.
- **★ The label is NOT EDITION-STABLE.** In **`ICC.1:2001-04`** Annex D
  the equations are **(D.1)–(D.6)**, **there is no (D.7)**, and that
  edition's **(D.6) is the single `Z` component of the *inverse***.

**Why this was live rather than pedantic: every `wtpt` discussion in
this project concerns a v2 file.** A reader chasing *"D.7"* into the v2
specification finds nothing; a reader chasing *"D.6"* finds a different
equation entirely. **Recommended form, and it is what `TOLERANCES.md`
§3.4.5 now uses:** `ICC.1:2022 6.3.2.2 Eq (4)–(6) [restated verbatim,
D.6.1 Eq (D.7)]`. **A sweep for the bare label is owed** (§7.10) — this
is the **DL-014 citation audit's** problem in miniature, and the second
time in two filings that a citation has been found to name the right
words in the wrong place (`TOLERANCES.md` §5.2's NA-003 correction was
the first).

#### 3.15.8 What §3.14 and §3.15 do **not** claim

- **Not that iccce's absolute arithmetic is CORRECT.** NC-120 and
  NC-122 are **cross-checks**; two implementations can read 6.3.2.2 the
  same way and both be wrong. **Pass 4c creates no ground-truth row**,
  and neither does §3.14.
- **Not that the SOURCE-side absolute term is exercised in §A.** It is
  **identity by construction** — that is exactly what buys NC-123's
  exact counterfactual, **and it is a cost as well as a benefit**. §A
  measures the **destination-side** term, which is the term NC-053 got
  wrong. §B exercises the source side but is **ungraded**.
- **Not that lcms2 is non-conforming.** §3.15.6, wording row.
- **Not that either implementation's `wtpt` reading is right.** A16-shaped
  silence; a **difference**.
- **Not anything about A4c**, which is a different ambiguity and still
  SILENT.
- **Coverage, to travel with every row above:** **two profile pairs**,
  **one destination tag** (`B2A1`), **one grid** (729 points), **one
  intent pair** (absolute + its media-relative floor), **one machine**,
  Windows/MSVC, **one pin**, **two runs** (bit-identical), **no other
  implementation timed or consulted beyond lcms2**, and **no
  ground-truth row created**. For §3.14: one pair, one tag type, 213
  points, same machine and pin.

### 3.16 ★★ Three measurements filed WITHOUT NC numbers — and each is deliberate

This ledger has a standing practice, set at §2.2.1 (the machine-wide
sweep) and §3.10.5 (GP-001): **a real measurement that nothing in the
harness pins does not get an NC number**, because an NC number is a
promise that a later session can re-run the thing and compare. All three
below are **scratch probes** — run once, by hand, outside the runner,
**with nothing in the tree that would fail if they stopped reproducing**.

#### 3.16.1 ★★ M3 — the out-of-gamut excursion count. **The owed item was a NULL BY CONSTRUCTION; its replacement finds a magnitude four orders larger**

**Reported by `icc-conformance`, 2026-08-12, scratch probe, `oracle-behaviour-at-pin`.**

**(a) The owed measurement could not have produced evidence, and that is
its discharge.** `TOLERANCES.md` §3.4.4.5 carried, as owed, *the count
of out-of-`[0,1]` components `transicc` returned* on Pass 4b §A's **48
saturated-hue Lab points**. **Predicted before the run**: it must be 0,
because §A's destination is SWOP's `B2A1` — **a CLUT, whose outputs
*are* table entries in `[0,1]`** — so no arithmetic in that path can
leave the range. **Measured: 0 of 192 components** (48 points × 4 CMYK)
outside `[0,1]`, at **all three intents**, worst excursion
**0.000000e0**.

**Do not read that as a reassuring null.** The specified measurement was
**structurally incapable of showing the effect it was commissioned to
measure**. The item is **RETIRED, not satisfied**, and replaced by (b).

**(b) The case that CAN show it — a controlled A/B on ONE variable.**
Corpus **M3** says lcms2 **saturates** when a destination's inverse TRC
is a **tabulated** reverse curve, but an **analytic** inverse can return
values outside `[0,1]`. Same source (`USWebCoatedSWOP.icc`), same
**625-point CMYK grid** (0/25/50/75/100 % per channel — genuinely
outside a small RGB matrix gamut), same intent (media-relative), `-c0`;
**three destinations differing only in inverse-TRC kind**:

| destination | TRC kind | components outside `[0,1]` | worst excursion |
|---|---|---|---|
| `fixtures/synthetic/v4-rgb-matrix-trc.icc` | `para` funcType 0 (γ=2.0), **analytic** | **16 / 1875** | **1.380557e-1** |
| `fixtures/synthetic/v4-rgb-para-type3.icc` | `para` funcType 3, **analytic** | **137 / 1875** | **3.053984e0** |
| system sRGB `.icm` | **tabulated** | **0 / 1875** | **0.000000e0** |

**Raw spot check, so `transicc`'s 0..255 convention is not taken on
trust.** CMYK (0, 100, 100, 0) and (100, 0, 100, 0), media-relative:
→ `v4-rgb-para-type3` gives `233.2237 70.4852 **-252.5707**` and
`**-121.1480** 188.7322 88.3317`, i.e. normalised **−0.9905** and
**−0.4751** — **genuinely negative, not 1-lsb boundary residue**; the
**identical inputs** → system sRGB give `237.6654 51.3502 55.8132` and
`**0.0000** 168.4047 89.6148` — **saturated at zero**.

**(c) ★★ Why this matters to a claim already in this ledger: a hedge
written from method discipline is now VINDICATED BY MEASUREMENT.**
**NA-003's dated status** carries: *"every excursion observed was 1-lsb
boundary residue at white … `transicc` returned 0 of 1023 components
outside `[0,1]` at every intent, because that destination's TRC inverse
is a tabulated reverse curve, which is lcms2's saturating path (corpus
M3). So the observed cost of this divergence remains ≤1.2×10⁻⁴ device
units at white, **and that number must never be restated as a bound on
the divergence in general**."*

**The real magnitude on an analytic-inverse destination is up to 3.05
device units — roughly 2.5×10⁴ times the 1.2×10⁻⁴ that was carefully
fenced off.** **This project has more examples of hedges that were
merely prudent than of hedges that were later shown to be necessary.
This is one of the second kind, and it is worth more than the number**:
the sentence that saved it was written by someone who had only the small
number and refused to generalise from it.

**(d) Scope, and it is a real limit.** **Both arms measure lcms2
ALONE.** **iccce's side was not run in arm (b)**, so **this is NOT a
divergence measurement between the two implementations.** iccce clamps
by **NA-004**, so the *difference* would be the excursion itself — **but
that was not measured and must never be written as though it had
been.** One machine, one pin `21c582a`, Windows/MSVC, one run, one
intent in arm (b), `-c0` throughout.

#### 3.16.2 ★ NA-008's cross-check half — gray as a DESTINATION, probed

**Reported by `icc-conformance`, 2026-08-12, scratch probe.**
`sRGB → ewgray22.icm`, **729 RGB points**, **max 3.382353e-05 device**;
and the residual is **no larger off the neutral axis (3.247059e-05 over
720 points) than on it (3.382353e-05 over 9 points)**.

**This is the *cross-check* half of NA-008 and it is now reachable and
cheap.** **It is NOT the half that measures the named approximation** —
see §4's NA-008 dated note, and §3.16.3.

#### 3.16.3 ★★ NA-008's OTHER half has NO INSTRUMENT, and one is owed from `tools/gen-profiles`

**NA-008 is the choice between `Y / Yn` (PCSXYZ) and `L* / 100`
(PCSLAB)** as the projection that recovers the F.2 connection scalar
from a non-neutral PCS colour. **Discriminating those two requires a
PCSLAB gray profile, and every gray profile in reach is PCSXYZ:**

| profile | version / class | PCS |
|---|---|---|
| `ewgray18.icm`, `ewgray22.icm` | v2.2.0 `'mntr'` | **GRAY → XYZ** *(reported — headers read via `iccce inspect`)* |
| `BlackWhite.icc` | v4.0.0 `'prtr'` | **GRAY → XYZ** *(reported)* |
| `fixtures/synthetic/v2-gray-curv-gamma.icc` | v2.4.0 `'mntr'` | **GRAY → XYZ** *(**verified** — `fixtures/synthetic/MANIFEST.md` read: "v2.4.0.0 mntr GRAY monochrome, kTRC as the gamma shorthand, gamma 2,0")* |
| `fixtures/synthetic/v2-gray-curv-identity.icc` | v2.4.0 `'mntr'` | **GRAY → XYZ** *(**verified** — MANIFEST read)* |

**So `tools/gen-profiles` owes a PCSLAB gray fixture**, and it is
**exactly the same shape of owed instrument as Pass 5's non-zero device
black** (§7.8 item 1, still open): **sourcing is not measuring, and
agreeing with lcms2 is not measuring the projection choice either** —
lcms2 makes one of the two choices too, so a cross-check against it
cannot discriminate them. **Two named approximations in this register
are now blocked on a fixture that has never been written**, which makes
`tools/gen-profiles` the highest-leverage unwritten code in the
repository.

### 3.17 ★ Pass 5b — the first attempt to discriminate the ESTIMATORS. **Filed retrospectively, WITH the verdicts Pass 5c overturned, because a withdrawn verdict that is deleted cannot be learned from**

**Run 2026-08-12 by `icc-conformance`.** Apparatus:
`tools/difftest/src/pass5b.rs`. Full record: `tools/difftest/README.md`
**§17**; tolerances `TOLERANCES.md` **§3.5.7**. Fixture:
`USWebCoatedSWOP.icc` (v2.1, `prtr`, CMYK, `Lab ` PCS) as destination,
system sRGB as source, **media-relative**, 21-step neutral ramp.

> **★★ READ THIS BEFORE ANY NUMBER BELOW.** These rows are filed **after**
> the work that partly overturned them, and deliberately not filed as if
> they had come out right. **§3.12.3 said no row in this ledger
> discriminates the two estimators; Pass 5b was the first that tried.**
> It could not read lcms2's black point, so it **recovered** it through
> `A2B1 ∘ B2A1`, **said so**, graded the recovery's error at **95 % of
> the effect it was bounding**, and qualified its conclusions
> accordingly. **Pass 5c (§3.18) then reproduced lcms2's estimator from
> source and found that 98,3 % of NC-130's number WAS that recovery.**
> **NC-131's verdict is WITHDRAWN. NC-130 is SUPERSEDED by NC-142.**
> Neither row is deleted, and neither is edited: *what an instrument
> reported before a better instrument existed is the only evidence that
> the better instrument was needed.*

#### 3.17.1 The eight rows, at a glance

| ID | What | Class | Tolerance | Observed | Status now |
|---|---|---|---|---|---|
| **★ NC-129** | **The apparatus row** — lcms2's black is **recovered** through `A2B1 ∘ B2A1`, so the recovery's own error must be smaller than the effect it bounds. Ratio, no free parameter | self-consistency (**apparatus check**) | **1,0** | **0,948 24** — local residual **0,782 5** at the ISO black, **0,813 7** at the recovered lcms2 black | **STANDS as a fact about Pass 5b's apparatus**, and it is the row that made the withdrawal below findable. ★ **It passed by 5 % and was reported as marginal rather than quoted as green** |
| **NC-130** | **The two estimated black points, in Lab.** ISO **`L* 16,4898`, neutral**; lcms2 **`L* 17,2150 · a* 0,3472 · b* 0,3001`** | cross-check | **∞ — REPORTED, NOT GRADED** | **0,858 17 ΔE76** (`ΔL* −0,7252`, `Δa* −0,3472`, `Δb* −0,3001`) | **★★ SUPERSEDED BY NC-142.** **98,3 % of it was the recovery**, not the estimators. The true divergence on this fixture is **8,166 8×10⁻² ΔE76, entirely `L*`** |
| **★ NC-131** | Pre-registered **claim 1 — the MECHANISM**: the chroma of the divergence equals the detected black's chroma | cross-check (**structural on iccce's side, and labelled so**) | **1×10⁻¹²** | **0,0 exactly** | **★★ The VERDICT ("CONFIRMED") is WITHDRAWN** — lcms2's black on this fixture is **neutral**, so there was no chroma to confirm. **What the row still grades is untouched**: that **ISO 4.2.3 is implemented** (a build that kept the chroma fails it) |
| **★★ NC-132** | Pre-registered **claim 2 — the MAGNITUDE**: predicted band **2–6 ΔE76** | cross-check | **∞ — REPORTED** | **0,458 92 ΔE76** — the detected black's chroma, an **order of magnitude below the band** | **FALSIFIED, and it STANDS.** SWOP's darkest colorant is `Lab(11,77 · 0,766 · 0,328)`, **0,834 off neutral**: no estimator reading this file could have landed in the band. ★ **Robust to NC-129's error bar** — even if the entire 0,813 7 fell in chroma, `1,273` is still below the band |
| **NC-133** | Pre-registered **claim 3 — the SHAPE**: is the divergence *the chroma*? Ratio of the `L*` term to the chroma term | cross-check | **∞ — REPORTED** | **1,580 11×** (`L*` 0,725 2 vs chroma 0,458 9); oracle-free ramp sensitivity **0,054 3 `L*`**, **13× too small** to explain it | **★ "NOT ESTABLISHED" WAS THE CORRECT CALL, and §3.18 settles it**: FALSIFIED on `swop`, CONFIRMED on the synthetic arm. **The row declined to claim a falsification its evidence could not support, and the evidence it lacked was exactly the branch nobody had read yet** |
| **NC-134** | Pre-registered **claim 4 — the DECAY**: a black-point disagreement must vanish at device white, because BPC is anchored on D50 there exactly | cross-check | **5×10⁻² ΔE76** *(deliberately §3.11's own white-end constant, not a new one)* | **0,0**. Ramp ΔE76 at `k = 0/0,25/0,5/0,75/1`: **0,087 8 · 0,053 1 · 0,013 5 · 0,008 8 · 0,000 0** — monotone | **STANDS.** Unaffected by the branch finding |
| **★ NC-135** | **What survives end to end** at input black — the number an integrator sees, as against the number the estimators differ by | cross-check | **∞ — REPORTED** | **8,785×10⁻² ΔE76 / 5,92×10⁻² ΔE2000 / 2,464×10⁻³ device** | **STANDS, and it is the most reusable row in the section**: **90 % of the black-point disagreement does not survive** the destination's `B2A`, because both blacks are at or below this profile's **gamut floor**. ★ **A disagreement about a black point is not the same size as a disagreement about the output** |
| **NC-136** | **The shipped chain could not reach the ISO estimator** — `iccce transform --bpc` **refused** a v2 CMYK LUT destination at media-relative, by exact `Display` text | self-consistency (behavioural, 0/1) | **exact wording** | **refused as required** | **★ SUPERSEDED BY NC-144, and its PREMISE is gone**: the defect it graded was fixed at commit `c268261`. **The row is kept because it is the measurement of a real defect on a real day** |

**Record arithmetic.** Eight rows, eight `pass5b/…` record ids. **Two
are superseded, one verdict is withdrawn, five stand.** ★ **Nothing here
was deleted and nothing was edited into agreement with §3.18** — the
project's convention, fourth instance after **DL-011 → DL-012**,
NA-006's *"tetrahedral"*, and **A4b**'s expiry (DL-026).

#### 3.17.2 ★★ Why this section exists at all, when a shorter ledger would just carry §3.18

**Because the discipline that caught the error is visible only in the
pair.** NC-129 is a row whose entire job was to say *"this apparatus may
not be good enough"*, it said so at **0,948**, and the thing it warned
about is exactly the thing that went wrong. **A ledger that carried only
the corrected numbers would record that the project got the right
answer, and would lose the reason it got there.**

★ **And one sentence from §19.5.1 belongs in this ledger verbatim**,
because it is the most portable thing either Pass produced:

> **§17.3.1 measured the same residual at 0,813 7 and called it an error
> bar. *It was not an error bar; it was the measurement.***

**The general form: when an error bar is the same order as the effect,
the honest reading is not "the result is marginal" — it is "the
apparatus may be measuring ITSELF."** NC-129 is what makes that
checkable rather than a worry.

### 3.18 ★★★ Pass 5c — lcms2's estimator REIMPLEMENTED, and **the finding of the day: lcms2 has TWO black-point estimators at media-relative, chosen by the DESTINATION'S DEVICE CLASS AND COLOUR SPACE**

**Run 2026-08-12 by `icc-conformance`.** Apparatus:
`tools/difftest/src/pass5c.rs`. Full record: `tools/difftest/README.md`
**§19**; tolerances `TOLERANCES.md` **§3.5.8**. Oracle pin **`21c582a`**
(lcms2 2.19.1), iccce at commit **`95c04c1`**. **Kind:
`implementation-cross-check`, provenance a SOURCE READ** — the same
standing as Pass 4b §C's `cmsReverseToneCurveEx` model. **No lcms2 binary
produces a black point here**; `transicc` appears only in the validation
arm, where it checks the reimplementation end to end.

#### 3.18.1 ★★★ The finding, stated before any row

`cmsDetectDestinationBlackPoint` takes its `InitialLab` from
`cmsDetectBlackPoint`, and **that function branches before it reaches the
code Pass 5b had read** (`cmssamp.c` **L370–374**):

| branch | taken when | what it does to the chroma |
|---|---|---|
| `BlackPointUsingPerceptualBlack` (L146+) | `INTENT_RELATIVE_COLORIMETRIC` **and** device class `output` **and** an **ink** colour space | round-trips `Lab(0,0,0)` through the **perceptual** `B2A`, clips `L*` to 50, and **FORCES `a* = b* = 0`** (L174) |
| `BlackPointAsDarkerColorant` (L62+) | **anything else** | transforms the space's darkest colorant through `A2B`, clips `L*` to `[0,50]`, and **KEEPS `a*` and `b*`** |

`cmsDetectDestinationBlackPoint` then returns `Lab.a = InitialLab.a;
Lab.b = InitialLab.b` (L590–591), **so the branch IS the returned
chroma.**

> **★★★ "Does lcms2 keep its black point's chroma?" HAS NO ANSWER.** It
> has one answer for a CMYK press profile and **the opposite** answer for
> an RGB printer profile. **The only real LUT profile within reach of
> this machine is the first kind**, which is why eleven filings' worth of
> reading never saw it.

**This is `ARCHITECTURE.md` DL-021 — *a behaviour is a fact about the
direction and path it was measured in* — generalised from
direction/path to PROFILE CLASS, and it is filed as `DL-027`.**

#### 3.18.2 ★★ The two arms, and the pre-registered prediction resolving in OPPOSITE directions on them

| | **arm `swop`** | **arm `synthetic`** |
|---|---|---|
| destination | `USWebCoatedSWOP.icc` — v2.1 `prtr` **CMYK**, `mft2`/`mft1`; `LEGAL.md` §3 category **(c)**, never committed | `fixtures/synthetic/v4-rgb-mab-chromatic-black.icc` — v4.4 `prtr` **RGB**, `mAB `/`mBA `, 9³; category **(a)**, authored byte by byte, committed, regenerable |
| lcms2 branch | `BlackPointUsingPerceptualBlack` | `BlackPointAsDarkerColorant` |
| ISO 4.2.5 black | `L* 16,489 806`, neutral | `L* 20,000 000`, neutral |
| lcms2 black (**reimplemented**) | `L* 16,571 474`, **neutral** | `Lab(20 · 4 · −3)`, **chromatic** |
| **divergence** | **8,166 8×10⁻² ΔE76 — 100 % `L*`, chroma exactly 0** | **5,000 000 ΔE76 — 100 % chroma, `ΔL*` exactly 0** |
| claim 1 (mechanism) · claim 3 (shape) | **FALSIFIED · FALSIFIED** | **CONFIRMED · CONFIRMED** |

> **★★★ A session that ran only ONE arm would have filed a confident
> wrong headline EITHER WAY.** The variable that decides the verdict is
> **two header fields** — not the black, not the intent, not the profile
> version, not the tag type.

⚠ **The synthetic arm's 5,000 ΔE76 is evidence for the MECHANISM and for
nothing else.** That chroma is what **this project authored** into the
fixture (`recipes.rs`, `SYNTH_BLACK_A/B`). It happens to land inside the
corpus's pre-registered **2–6 ΔE76** band; **that is not a confirmation
of claim 2**, whose falsification (NC-132) stands on the arm where the
profile was **not ours to choose**. *A fixture cannot confirm a magnitude
prediction about the world; it can only demonstrate a mechanism.*

#### 3.18.3 The eight rows, at a glance

Every row emits **two records**, one per arm (`pass5c/{arm}/…`), so the
section is **16 records**.

| ID | What | Class | Tolerance | Observed |
|---|---|---|---|---|
| **★ NC-137** | **The apparatus** — the error bar must be smaller than the effect. **NC-129's constant and NC-129's derivation, unchanged; only the apparatus is new**: a *reimplemented* black bounded by its own device residual against `transicc`, converted to `L*` through a sensitivity `d(device)/d(L*)` measured on the same `B2A1` table | self-consistency (**apparatus check**) | **1,0** | **swop 3,043 1×10⁻¹** · **synthetic 2,195 3×10⁻⁴**. ★ **NC-129 scored 0,948 on the same fixture: the bar is 33× tighter on `swop` and 4 300× tighter on the synthetic arm.** *A constant carried unchanged across an apparatus replacement is the strongest available evidence that it was never fitted to an observation* |
| **★★★ NC-138** | **THE FINDING** — `chroma of the divergence − what the SELECTED BRANCH requires` | cross-check (**structural on the reimplementation's side, and labelled so**) | **0,0 — EXACT, not an epsilon** | **0,0 exactly on both arms.** `swop`: chroma 0, branch requires 0. `synthetic`: chroma 5,000 000, branch requires 5,000 000. **Taking the other branch moves it by the darkest colorant's whole chroma** — 0,834 on SWOP, 5,0 on the fixture — **which no rounding argument reaches** |
| **★ NC-139** | **Neither implementation fits a quadratic on either fixture** — both take the mid-range straightness short-circuit (`cmssamp.c` L521–545; `bpc.rs`'s 4.2.5.4 gate) | cross-check (0/1) | **0,0 — exact** | **0,0 on both arms**; `nearlyStraight = true`, shadow points **0**. ★ **Graded rather than reported because Pass 5b asserted the opposite** — §17.3's *"precisely lcms2's method-4 (quadratic-fit) territory"* is **wrong**, and every Pass 5b statement about the shadow window, the sample count or the root **describes code that did not run** |
| **★★ NC-140** | **The discrimination row** — residual under the *lcms2* hypothesis ÷ residual under the *ISO* hypothesis, in device units | cross-check | **1,0** *(no free parameter: below 1 the lcms2 model is the better explanation of lcms2's own output)* | **swop 1,714 7×10⁻¹** (4,224 9×10⁻⁴ against 2,463 9×10⁻³) · **synthetic 1,561 2×10⁻⁴** (8,938 3×10⁻⁶ against 5,725 1×10⁻²). ★ **Without it a small absolute residual would be evidence of nothing**: on `swop` the two candidates are only **0,082 `L*`** apart |
| **NC-141** | **The absolute device residual** against `transicc` at input black | cross-check | **∞ — REPORTED, NOT GRADED** | **swop 4,224 9×10⁻⁴** · **synthetic 8,938 3×10⁻⁶**. ★ **Deliberately NOT graded against Pass 4b §A's 1,330×10⁻⁴ envelope for the same `B2A1` table** — that is a maximum over Pass 4b's point set and this is one deep neutral shadow point outside it. **What remains in it is the PIPELINE difference**: lcms2 evaluates through 16-bit machinery, the harness in `f64` |
| **★★ NC-142** | **The two black points** — **supersedes NC-130** | cross-check | **∞ — REPORTED** | **swop 8,166 8×10⁻² ΔE76** (`ΔL*` 0,081 67, chroma **0**) · **synthetic 5,000 000 ΔE76** (`ΔL*` **exactly 0**, chroma 5,000 000) |
| **★★ NC-143** | **The attribution** — `BT(reimplemented black)` should land on the black Pass 5b *recovered*; this grades what is left over, against **this section's own `L*` bound** rather than a chosen constant | self-consistency | **1,0 on `swop`; ∞ — REPORTED elsewhere** | **swop 6,036 4×10⁻¹.** Pass 5b recovered `Lab(17,214 958 · 0,347 197 · 0,300 108)`; `BT(reimplemented)` = `Lab(17,199 985 · 0,346 780 · 0,299 265)`; **unexplained 1,500 2×10⁻² ΔE76 of NC-130's 8,582×10⁻¹ — 98,3 % accounted for as apparatus.** ★ **Graded on one arm for a UNITS reason, not a convenience one**: the numerator is a full ΔE76 and the denominator an `L*`-only bound, commensurable only where the divergence is `L*` |
| **★ NC-144** | **The shipped binary reaches the ISO estimator** and lands on the **same black** the library function does — **supersedes NC-136**, whose premise the fix removed | self-consistency | **1×10⁻⁶ device** *(the CLI's own six-decimal print floor)* | **swop 4,499 1×10⁻⁷** · **synthetic 4,277 9×10⁻⁷**. ★ **The bound cannot absorb a different black point**, which moves this quantity by **2,46×10⁻³** (swop) and **5,73×10⁻²** (synthetic) |

#### 3.18.4 ★★ The apparatus fault, and the method rule it earns

The synthetic arm's **first** run reported a device residual of
**9,98×10⁻²** where the truth is **8,9×10⁻⁶**, and would have been filed
as *"the reimplementation does not reproduce lcms2 on this fixture."*

**The cause:** `transicc` prints **ink** spaces as percentages (`0..100`)
and **RGB and gray as `0..255`**. **Every oracle output in Passes 5, 5b
and 5c had been divided by 100**, because until this section the only
destination in reach was CMYK.

**How it was caught — and it was not by reading:** the validation arm
carries **two independent hypotheses**, and **both candidates missed by
roughly the same amount**. NC-140, whose entire job is to ask whether the
experiment can discriminate at all, is where that shows.

> **★★ THE RULE, filed as `ARCHITECTURE.md` DL-028: a residual that is
> LARGE UNDER EVERY HYPOTHESIS is an apparatus fault, not a finding.** A
> section with one arm and one candidate **has no way to notice**. This
> is the same family as **DL-025** (a control is only as good as its
> fixture) and **NC-110** (a null by construction): *an instrument that
> cannot fail cannot inform, and an instrument that fails identically
> under every explanation is measuring itself.*

#### 3.18.5 ★ What Pass 5c does **not** claim

- **Not ground truth.** It reproduces **lcms2**, from lcms2's source, at
  one pin. **Both implementations can read ISO/CD 18619 the same way and
  both be wrong**, and the ISO document is a **committee draft** besides.
- **Not any intent but media-relative.** At perceptual and saturation on
  a v4 profile **both implementations return the fixed A41 constant
  without reading the profile** (`cmssamp.c` L432–446;
  `Chain::estimate_dst_black`), so **no fixture can discriminate them
  there**. ★ §3.5.7.4's *"the v4 perceptual arm"* asked for an
  **instrument that cannot exist** — and the standing request in
  `NEXT_SESSION.md`, `ROADMAP.md` and **NA-009** for *"a non-zero-black
  v4 LUT fixture to discriminate the estimators"* is now **answered in a
  different place than it was aimed**: the fixture discriminates the
  **media-relative** arm, and the perceptual arm is **undiscriminable by
  construction**.
- **Not lcms2's ink round trip as a value.** It is reimplemented and it
  feeds the `swop` arm's `InitialLab`; **nothing grades that intermediate
  on its own**.
- **Not a profile whose darkest colorant has `|a*|` or `|b*|` above 50**,
  where lcms2's clamp/return asymmetry (it clamps the ramp's chroma to
  ±50 and returns the **unclamped** `InitialLab.a/.b`) would finally
  bite. **Still READ, not RUN** — and deliberately so: *a fixture built to
  trigger one branch of one clamp is a fixture built to make a point.*
- **Not the `bkpt` tag.** `CMS_USE_PROFILE_BLACK_POINT_TAG` is **off** in
  the pinned build, so neither arm consults it.
- **Not any source but the system sRGB profile, and not any platform but
  Windows/MSVC.**

#### 3.18.6 ★★ The OPEN QUESTION this Pass hands to `icc-spec-librarian`, and iccce's answer may be the wrong one

**The whole of the `swop` arm's 8,167×10⁻² ΔE76 is one line of code.**
Both implementations take the 4.2.5.4 mid-range straightness
short-circuit (NC-139), and they **return different things from it**:

| | what it returns at the short-circuit |
|---|---|
| **iccce** (`bpc.rs`, `estimate_lut_destination_black`) | **`outRamp[first]`** — `min_l.clamp(0.0, 50.0)`, where `min_l` is `out_ramp[0]` after the monotonic pass *(verified — read)* |
| **lcms2** (`cmssamp.c` L536) | **`InitialLab`** — a value from a **different** round trip |

**The question, dispatched to `icc-spec-librarian` 2026-08-12: does
ISO/CD 18619 4.2.5.4 specify `outRamp[first]`, or does it specify the
`InitialLab` behaviour lcms2 implements?**

> **★★ If ISO names lcms2's, iccce is WRONG — not divergent — and the
> engineer changes the code.** That is stated in this direction on
> purpose. **Rule 7 (*disagreement with lcms2 is a finding, not a
> failure*) is not a licence to assume iccce is right**; it says the
> **specification** settles it. Until the answer arrives, **NC-142's
> `swop` figure is a measured difference whose ATTRIBUTION is open**, and
> no document may describe it as lcms2 departing from the standard.

### 3.19 ★★ Pass 6, RE-GRADED at the new default grid of 33 — **the gate passes, and the number that moved was the GRID, not the tolerance**

**Re-run 2026-08-12 by `icc-conformance`** after commit **`189e732`**
moved `compiled::recommended_grid_points` from **17** to **33** for 3-D
and 4-D *(verified — `compiled.rs` read: `3 => 33`, `_ => 33`, with the
failing 17 recorded in the constant's own doc comment)*. Apparatus:
`tools/difftest/src/pass6.rs`; `TOLERANCES.md` **§3.6**;
`tools/difftest/README.md` **§18.2**. Pair, probes and intent are
unchanged: SWOP `A2B1` (`mft2`, 4-D) → system sRGB, **media-relative**,
`iccce bench`'s own **513** raster probes.

> **★★ The tolerance `2,5×10⁻¹` ΔE2000 DID NOT MOVE, and that is the
> entire point of the section.** It is **Pass 4's measured
> iccce-vs-lcms2 figure on this exact pair (0,252 94, §3.9)** to one
> significant figure — a number with **no free parameter to tune**.
> §3.6.1 wrote *"the remedy is the grid, not the number"* while the suite
> was red, and the engineer changed the grid. **A tolerance that survives
> its own failure is the only kind worth having written down.**

| ID | What | Class | Tolerance | Observed |
|---|---|---|---|---|
| **★★ NC-145** | **THE GATE** — the compiled path's cost in ΔE2000 at the **shipped default grid 33**, over the benchmark's 513 probes | **self-consistency** — both arms are iccce | **2,5×10⁻¹** | **★ PASS — 1,677 3×10⁻¹**, 33 % inside the line *(grid 17, the then-default: **FAIL 2,970 17×10⁻¹**, 17 % over)* |
| **★ NC-146** | **The same gate on PASS 4's OWN 341-point grid** — because a maximum over one population is not a maximum over another | self-consistency | **2,5×10⁻¹** | **★ PASS — 9,348 6×10⁻²** *(grid 17: **FAIL 2,962 90×10⁻¹**)*. ★★ **At 17 the two populations agreed to 0,25 %; at 33 NC-145 is 1,79× NC-146**, because once the error is small enough **probe PLACEMENT dominates**. **Both are inside the line, and quoting either alone is now a POPULATION CLAIM** |
| **NC-147** | **The device cost** — the quantity `iccce bench` prints. **Supersedes NC-108's shipped-default reading** | self-consistency | **∞ — REPORTED, NOT GRADED** | **2,012 444×10⁻³** *(grid 17: 3,588 962×10⁻³ — **the 0,003589 NC-108 carries**)*. Still above the `1,84×10⁻³` shadow-derived bound the ΔE row implies, and ungraded for the same reason: **the same physical event has a different size in two units, and the unit the requirement is stated in is the one that may carry the tolerance** |
| **★ NC-148** | **The apparatus row** — the harness reproduces `iccce bench` to the CLI's nine-decimal print floor | self-consistency (**apparatus check**) | **1×10⁻⁹** | **2,739×10⁻¹⁰** at grid 33 (2,537×10⁻¹⁰ at 17). ★★ **THIS ROW IS WHAT CAUGHT THE DEFAULT MOVING**: when `recommended_grid_points` changed and the harness constant did not, it failed at **1,576×10⁻³** — *not an error, but the gap between two grids' costs*. **A cheap row that fails loudly when the two arms stop describing the same transform is worth more than an expensive one that averages over it** |
| **★★ NC-149** | **The sensitivity control (DL-018), re-derived** — paired median of `err(coarse)/err(fine)` **at the same probe**, over three halvings, against the band `[2, 8]`. **This is the PASSING value NC-109 never recorded** | self-consistency (**instrument check**) | **band violation 0,0 — exact** | **0,0.** Paired medians **5/9 = 2,69 · 9/17 = 2,47 · 17/33 = 2,51** — stable to ~1 % across three octaves ⇒ **observed convergence order `log₂ 2,5 = 1,32`, NOT 2** |
| **★ NC-150** | **The FALSIFIED estimator, kept on file** — the **max-of-max** ratio `compiled.rs`'s unit test uses | self-consistency | **∞ — REPORTED** | **band violation 6,144×10⁻¹**; ratios **5,57 → 1,39 → 1,78**, a factor of **4** of wander against the paired median's 2,69 → 2,47 → 2,51. ★ **A clamp attribution was written, tested and FALSIFIED here**: restricting to cells whose 16 corners are all in gamut and above sRGB's 0,040 45 breakpoint changed the ratios **not at all** (448/513 probes; 65/513 out of gamut) |
| **NC-151** | **The grid trade, reported** so it is visible rather than asserted | self-consistency | **∞ — REPORTED** | ΔE2000 max / build: **5: 7,284×10⁻¹ / 0,009 s** · **9: 4,046×10⁻¹ / 0,086 s** · **17: 2,970×10⁻¹ / 1,06 s** · **33 (default): 1,677×10⁻¹ / 14,0 s** |
| **★ NC-152** | **What the green cost** — `iccce bench`'s **break-even** raster size, at the old default and the new one | **machine-timing** | **none — a timing is not correctness evidence of any kind (§1)** | **≈70 000 px → ≈1,19 million px**, a **17× increase**, from a build of 1,06 s → ~14 s. **Compiling now pays for itself only on large rasters** |

#### 3.19.1 ★ What the re-grade does and does not do to §3.13's rows

- **NC-108 is NOT deleted and NOT edited.** Its **0,003589 device units**
  is a correct measurement of the compiled path **at grid 17**, which was
  the shipped default when it was taken. **NC-147 is the same quantity at
  the shipped default of today.** Both are true; only one describes what
  a user gets.
- **NC-105 / NC-106 / NC-107 (the timings) are now ABOUT AN OLD
  DEFAULT.** NC-106's *"83 521 chain evaluations (17⁴), 1.04 s"* is
  **1 185 921 nodes and ~14 s** at 33 *(reported)*. **NC-105's 1.20
  Mpix/s and NC-107's 14.4× are conversion-rate figures and are not
  themselves changed by the grid**, but **the price of reaching them
  is** — NC-152. **A speedup quoted without its build cost is now a
  materially incomplete claim.**
- **NC-109's band is REPLACED by NC-149's, and the replacement is a
  re-derivation, not a widening.** NC-109 asserted `h²` and never
  recorded its passing ratio; NC-149 asserts only that the order lies in
  `[1, 3]`, measures **1,32**, and says so. ★ **`TOLERANCES.md` §4
  records the change with its justification** *(verified — read)*.
- **★ The done-when is untouched.** Pass 6's clause 2 asked for a
  *measurement*, not a bound (§3.13.3), and it has one at both grids.

### 3.20 ★ Pass 1's last remainder — ΔE94 and ΔE CMC(l:c). **`impl_crosscheck` by construction, and the module says so before it says anything else**

**Landed 2026-08-12** at commit **`aef7566`**, closing the first of the
four items `ROADMAP.md`'s Pass 1 record listed as *"blocked on sourcing,
not on engineering"* since 2026-08-11. **CIE 116-1995 (ΔE94) and BS 6923
(CMC) are paywalled and NOT sourced, and no published worked example was
obtained for either** *(verified — `delta_e.rs`'s module doc read)*.

> **★ The strength table is IN THE MODULE, not only here** *(verified —
> read)*: ΔE2000 **ground truth** (Sharma 34 pairs); ΔE76 **exact**
> (closed form); **ΔE94 and ΔE CMC `impl_crosscheck`**. It ends *"Grade
> suites in ΔE2000. These exist because some published tolerances are
> stated in them, not because they are as trustworthy."* **That is rule 3
> written at the site, which is where it survives.**

| ID | What | Class | Tolerance | Observed |
|---|---|---|---|---|
| **★ NC-153** | **ΔE94** (graphic-arts `kL=kC=kH=1`, `K1=0,045`, `K2=0,015`) against **lcms2 `cmsCIE94DeltaE`**, on **three** Lab pairs. ★ **The expectations were produced by compiling a C probe against the PINNED lcms2 and printing 10 decimals** — because the oracle is a subprocess the unit tests cannot reach | **implementation-cross-check** — **NOT ground truth** | **1×10⁻⁹** | **matched to all TEN printed decimals on all three pairs, FIRST RUN** (1,408 310 081 4 · 68,911 643 645 3 · 1,844 619 451 0) |
| **★ NC-154** | **ΔE CMC(2:1)** and **ΔE CMC(1:1)** against **lcms2 `cmsCMCdeltaE`**, same three pairs, same probe | **implementation-cross-check** | **1×10⁻⁹** | **matched to all ten printed decimals, first run**, in both parameterisations (2:1 — 1,738 736 105 7 · 58,055 319 818 0 · 2,024 752 084 5; 1:1 — 1,738 736 105 7 · 92,094 183 238 0 · 2,024 878 928 7) |
| **★★ NC-155** | **CMC is ASYMMETRIC ON PURPOSE** — it weights by the **first** (reference) colour, so `cmc(a,b) ≠ cmc(b,a)`. Asserted as a **difference**, so nobody later "fixes" it into symmetry. ΔE94's symmetry is asserted in the same test | self-consistency (**structural, and a REGRESSION AGAINST A PLAUSIBLE "FIX"**) | asymmetry **> 1×10⁻⁶**; ΔE94 symmetry **< 1×10⁻¹²** | **both hold** |
| **NC-156** | **The reduction identities and the two carried guards**: ΔE94 = ΔE76 on neutrals at unit weights; CMC = `ΔE76 / sl`; **two black colours return exactly 0** (else `sl` divides by zero); **`L* < 16` pins `sl = 0,511`** | arithmetic-identity | **1×10⁻¹²**, and **exact** (`assert_eq!`) for the two blacks | **asserted; not reported** |

#### 3.20.1 ★ Exactly how weak this is, said plainly

**Agreement to ten decimals on the first run is what a faithful
transcription produces, and it is ALSO what two identical mistakes
produce.** The test's own doc comment says so *(verified — read)*:
*"which is expected for a transcription, and is why this is a weak test:
it would also pass if both were wrong the same way."*

**What NC-153/NC-154 establish:** the transcription is faithful to lcms2
at the pin. **What they do not:** that lcms2 reads CIE 116-1995 or
BS 6923 correctly. **Rule 3's distinction, in the one place it is easiest
to blur** — a ten-decimal match *looks* like the strongest row in the
ledger and is one of the weakest. ★ **NC-001 remains the only
`published-ground-truth` row in this project**, and Pass 1's remainder is
now **three items, not four**: the **von Kries (HPE) cone matrix**
(corpus digits marked DO NOT USE), **CAT02** (CIE 159 paywalled), and
**observer CMF tables** (not needed until spectral input exists).

### 3.21 ★★ The ISO estimator acquires a CALLER — the "unused capability" family, demonstrated on this project's own code

| ID | What | Class | Tolerance | Observed |
|---|---|---|---|---|
| **★★ NC-157** | **A v2 CMYK LUT destination at media-relative — the exact case ISO/CD 18619 4.2.5 exists for — now CONVERTS through `iccce transform --bpc` instead of being refused**, and BPC **moves** the answer at a dark probe. Regression for the defect `icc-conformance`'s Pass 5b sweep found | self-consistency (**behavioural, reachability**) | **`moved > 0,0`** — a **0/1 reachability assertion, NOT a colour claim** | **reached; the value moves** *(and the test SKIPS on a machine without this machine's colour directory — `eprintln!("skipped: system profiles absent")`)* |

**The defect, in one sentence:** `crates/iccce-cmm/src/bpc.rs` implemented
ISO/CD 18619 4.2.5 **in full**, was unit tested, was filed as **NA-009**,
was celebrated in a ROADMAP addendum — and **nothing outside its own test
module called it**, so the shipped binary went on refusing exactly the
case it existed for. Wired at commit **`c268261`** *(verified —
`transform.rs::estimate_dst_black` read: the LUT arm now builds
`InitialLab` per 4.2.5.2.1 and calls `estimate_lut_destination_black`,
with `// WIRED 2026-08-12` and the reason at the site)*.

> **★★ This is the third member of a family this ledger has now seen
> three times, and it deserves its name: `NamedColors` was *"referenced
> by nothing outside its own file"* for two filings (§3.13.6); the
> `mBA ` evaluator existed before anything drove it; and `bpc.rs`'s ISO
> estimator was SOURCED, IMPLEMENTED, DOCUMENTED and UNREACHABLE.**
> **An unused capability is not a feature — and, worse for this project,
> it is not a MEASUREMENT either**: NA-009's cost could not be taken
> because nothing consumed the code that would have produced it.
> ★ **The rule that follows is cheap: when a Pass files an
> implementation, the filing should state WHO CALLS IT** — and if the
> answer is *"its own tests"*, that is the finding, not a detail.

### 3.22 ★★ The APPARATUS CENSUS — three green results on one tree, from three runners, and the reason all three are written together

**Reported 2026-08-12 by `icc-engineer`, who ran all three at commit
`2a2d616`.** This librarian ran nothing.

> **★★ THE POINT OF THIS SECTION IS THE ADJACENCY.** Any one of these
> numbers alone is a trap. **129**, **36** and **142** are three green
> results describing three disjoint populations, and on the day they
> were taken two of them were briefly compared and read as a regression
> — **by the engineer who had produced both**. They are filed in one
> table, each beside its command, so that a future reader meets them
> together or not at all. `ARCHITECTURE.md` **DL-031**.

| ID | What | Class | Tolerance | Observed |
|---|---|---|---|---|
| **★ NC-158** | **The workspace unit suite** — `cargo test --workspace` at the repository root. Member set: the **five** crates of `ARCHITECTURE.md` §1 | **apparatus-census** | **none — a census is not correctness evidence of any kind (§1)**; the only assertion is the **exit code** | **129 passed, 0 failed**, **bare exit 0**. Per crate: **iccce-cmm 63 · iccce-profile 33 · iccce-color 25 · iccce-measure 8 · iccce-cli 0 · doc-tests 0** |
| **★ NC-159** | **The harness's own unit suite** — `cargo test` **in `tools/difftest`**, which is **deliberately not a workspace member** (DL-001's oracle isolation, DL-017's permitted path dependency), so **`--workspace` cannot see it and NC-158 does not include it** | **apparatus-census** | **none**; exit code only | **36 passed**, exit 0 |
| **★★ NC-160** | **The differential conformance runner** — `cargo run --release` in `tools/difftest`. **This is the oracle**: it drives lcms2 and grades every `pass{3,4,4b,4c,5,5b,5c,6,7}` record against `TOLERANCES.md`. **Re-run today on current code** | **apparatus-census** *(of records; each **record** carries its own class in §3)* | **none as a census.** The records it grades carry the tolerances; **the census asserts only `fail=0` and `error=0`** | **pass=142 fail=0 skip=3 error=0** — unchanged from the previous filing's report on the same shape |

#### 3.22.1 ★★ What corroborates these, and precisely what does not

**Checkable without a shell, and it came out unusually well:**

- **129 `#[test]` declarations across 20 files under `crates/`**
  *(verified — counted)*, distributed **cmm 63 · profile 33 · color 25
  · measure 8 · cli 0**. ★ **That is the per-crate breakdown of NC-158,
  exactly, on all five members.**
- **36 `#[test]` declarations across 6 files in `tools/difftest`**
  *(verified — counted)*, matching NC-159 exactly.

**What the agreement establishes:** the runner saw the population this
tree contains — **no declared test was filtered out, `#[ignore]`d,
`cfg`-ed away or silently skipped**. That is worth having, and it is a
stronger corroboration than §2.11's, where two *totals* agreed by
coincidence; **five per-crate figures agreeing is not a coincidence.**

**What it cannot establish, and the distinction is the whole of §1.2:**

- **Not that any test passed.** A declaration count cannot corroborate
  an outcome. *"129 passed"* is **`icc-engineer`'s report**, and the
  count corroborates its **denominator** only.
- **Not coverage.** ★ **A count of tests is not an inventory of what is
  tested.** `iccce-cli` contributes **0** — a shipped binary with a
  public surface and no unit test of its own — and NC-158's total is
  entirely insensitive to that. **A number that cannot fall when a
  crate has no tests is not measuring testing.**
- **Not the runner's coverage either.** **NC-160's `skip=3`** is
  reported and **not enumerated**: nobody has said which three records
  skipped or why. A skip is the runner declining to grade, which is
  invisible in `fail=0`.

#### 3.22.2 ★ The three numbers are not on a common scale, stated once so it need not be inferred

**129 counts `#[test]` functions. 36 counts `#[test]` functions in a
different crate that is not in the workspace. 142 counts
CONFORMANCE RECORDS** — comparisons against `TOLERANCES.md` rows, each
of which may involve thousands of probe points and none of which is a
Rust test function.

**The number 142 has no relationship of any kind to 129.** It is not
larger, not smaller, not a superset and not a trend. Its only valid
comparison is **to its own previous run**, which was **`pass=140
fail=2`** on a shape that no longer exists (§2.11) — so even that
comparison needs the shape attached.

★ **And the `142` is in a place no dated note can reach.** It is the
commit message of **`d5efd96`**, *"Final filing + suite green at 142"*.
**It appears in no document in `docs/`** *(verified — searched; the
only `142` in `docs/` is the CIE standard number 142-2001)*. Commit
messages are append-only in the strongest possible sense: **the record
can be corrected here and the message stays wrong forever.** That is
the strongest available argument for writing the command down the first
time.

### 3.23 ★★ Pass 6's throughput and speedup, WEAKENED FROM A FIGURE TO A RANGE — and the range is wider than any single filing suggested

**Occasioned 2026-08-12 by a third measurement of the same binary on the
same machine.** No code changed between the readings; **only the load
did** — and, in one reading, the grid.

> **★★ THE CLAIM BEING WEAKENED IS NOT WRONG. IT IS
> UNDER-SPECIFIED.** Each figure below was correctly measured and
> honestly reported on the day it was taken. What the accumulation
> shows is that **the quantity has a spread the size of the claim**, so
> **no single number can carry it** — and `machine-timing`'s definition
> already said this would happen (*"a fact about hardware, allocator,
> build flags and one execution"*). **This section is that class being
> taken at its word.**

#### 3.23.1 Every reading on record, with its apparatus

| # | Apparatus | Grid | Throughput (compiled) | Reference | Speedup | Where |
|---|---|---|---|---|---|---|
| 1 | `iccce bench` CLI, full 300 DPI A4 raster (8 700 867 px), **loaded machine** | 17 | **1,203 Mpix/s** | 0,084 Mpix/s | **14,4×** | **NC-105 / NC-107** — ★ **RETRACTED as a quotable figure** by this section |
| 2 | `iccce bench` CLI, same raster, **loaded machine** | 17 | **0,820 Mpix/s** | — | **12,18×** | `docs/bench-2026-08-12.txt` |
| 3 | `iccce bench` CLI, **2 Mpix raster, quiet machine**, ×3 | 17 | **1,477 / 1,466 / 1,475 Mpix/s** | — | **16,00 / 16,01 / 16,19×** | `docs/bench-2026-08-12.txt` |
| 4 | **The conformance runner's own bench line**, full A4 raster (8 700 867 px), build 12,154 s, convert 3,866 s | **33** | **2,251 Mpix/s** | — | **22,85×** | **This filing** *(reported)* |
| — | **`tools/difftest`'s Pass 6 apparatus** — a **fourth, differently shaped** set | 17 | **2,4–2,7 Mpix/s** | **0,076–0,091 Mpix/s** | **28–32×** | `TOLERANCES.md` §3.6.2 — ★ **NOT reconciled here; see §3.23.4** |

| ID | What | Class | Tolerance | Observed |
|---|---|---|---|---|
| **★★ NC-161** | **The compiled path's throughput, as a RANGE under stated load variance** — `iccce bench` arm only | **machine-timing** | **none — and a range is not a tolerance.** The interval is the observed envelope, not a bound anything is graded against | **0,820 – 2,251 Mpix/s**, a **2,7× spread**, on **one machine, one build, one binary, across two grids**. ★ **The quiet-machine triple agrees to 0,7 %**, so the *instrument* is stable and the *machine* is not |
| **★★ NC-162** | **The speedup, as a RANGE** — compiled against the in-process reference path. **SUPERSEDES NC-107's `14.4×` as a quotable figure** | **machine-timing** | **none** | **12,18× – 22,85×**. ★ **The honest published form is "12–23× on this machine, load-dependent"** — and the *speedup* is the more transportable of the two numbers, because **both arms suffer the same load**, which is why its spread (1,88×) is smaller than throughput's (2,7×) |
| **★ NC-163** | **The break-even raster size, as a RANGE** — the point at which compiling repays its build. **Moves with NC-161/NC-162 and with the grid, and SUPERSEDES nothing: it EXTENDS NC-152** | **machine-timing** | **none** | **≈70 000 px** (grid 17, build 1,06 s) → **≈1,19 million px** (grid 33, build ~14 s, NC-152) → **1 258 593 px** (grid 33, build 12,154 s, today). ★ **The two grid-33 figures differ by 5,7 % because the BUILD time differs**, which is the same load variance seen from the other end |

#### 3.23.2 ★★ What must change in the documents, and what must not

**Must not:** NC-105, NC-106 and NC-107 are **not deleted and not
edited**. Each is a correct measurement of one execution and the record
of *what was believed when* is the point of an append-only ledger.
§3.19.1 already scoped them to grid 17; this section additionally
scopes NC-105 and NC-107 to **one load condition**.

**Must:** ★ **anywhere a single figure is asserted as "the" throughput
or "the" speedup, it is now a materially incomplete claim.** Found by
search *(verified — `docs/` searched for the figures)*:

| Where | What it says | Status |
|---|---|---|
| `ROADMAP.md` Pass 6 header block | *"**1.20 Mpix/s**, **14.4× the reference path**"* | ★ **Superseded by NC-161/NC-162.** Corrected by a **dated addition** at the head of the document, not by rewriting the block — the block is the record of the Pass's own filing |
| `SESSION_LOG.md` Pass 6 entry | the same two figures | **Append-only; left exactly as written.** Its own entry already carries §3.13.2's arithmetic gap |
| `NUMERIC_CLAIMS.md` **NC-105 / NC-107** | the same two figures | **Superseded here, retained there** |
| `docs/bench-2026-08-12.txt` | *"12.2x-16.2x observed"* and *"quote a RANGE and the load condition, never a point figure"* | ★ **Already correct, and it predicted this section.** Its range is now **wider** (reading 4 was not yet taken) |
| **`TOLERANCES.md` §3.6.2** | *"2,4–2,7 Mpix/s … **28–32×**, break-even ≈63 000–75 000 px"* | ★★ **FLAGGED, NOT EDITED — `icc-conformance` owns that file** (§3.23.4) |
| `README.md` | **not checked at this filing** | ★ **Owed** — §7.12 |

#### 3.23.3 ★ Why a range is the honest form and an average is not

An average of these readings would be a number **no execution
produced**, presented with the authority of one that did. The readings
are not samples of a stable quantity with noise; they are **samples of
different conditions** — loaded versus quiet, grid 17 versus 33, a
2 Mpix raster versus an 8,7 Mpix one — and a mean over conditions
nobody enumerated is exactly the *"plausible-looking result"* rule 1
warns about, arriving in a unit where nobody thinks to check.

**The range, with its conditions named, is the whole of what is known.**

#### 3.23.4 ★★ The FOURTH set of figures, flagged and deliberately not reconciled

`TOLERANCES.md` §3.6.2 records **2,4–2,7 Mpix/s compiled against
0,076–0,091 Mpix/s reference, 28–32×, break-even ≈63 000–75 000 px**,
with *"the run-to-run spread across four invocations in one session was
~10 %"* *(verified — read)*. **Its apparatus is `tools/difftest/src/pass6.rs`,
not the `iccce bench` CLI**, and its compiled throughput is roughly
**double** the CLI's at what appears to be the same grid.

**Three things are true and this librarian can establish none of the
fourth:**

1. **The numbers are not mine to change.** `TOLERANCES.md` is
   `icc-conformance`'s. **Flagged, not edited** — and this filing does
   **not** assert that they are wrong.
2. **They are not in NC-161/NC-162's ranges**, so quoting *"12–23×"*
   alongside *"28–32×"* would be quoting two incompatible claims about
   one program.
3. **The discrepancy has a shape worth naming:** a ratio near **2×** on
   the compiled arm with a **similar** reference arm is what one would
   expect if the two harnesses time **different work** — for example
   with or without the per-pixel buffer marshalling the CLI does. ★
   **That is a HYPOTHESIS, stated so somebody can test it, and it is
   labelled as one.** Nobody has run the comparison.

★ **Until it is resolved, no document may quote a single speedup figure
at all**, because the project currently holds **two ranges that do not
overlap** and does not know why. **That is the most useful sentence in
this section.**

#### 3.23.5 What §3.23 does NOT claim

- **That lcms2 is slower than iccce by any factor.** ★ **lcms2 has
  never been timed by anybody here**, in any Pass. Every ratio above is
  **iccce against iccce**.
- **That any figure describes another machine.** `machine-timing`'s
  definition applies at full strength: a different CPU, allocator,
  build profile or thermal state **retires every row above**.
- **That the spread is understood.** *"Load"* is the reported cause of
  readings 1–3's spread and it is plausible; **nothing measured it**.
- **That correctness is affected.** No row here is correctness evidence
  of any kind, and the compiled path's **error** (NC-145's
  `1,677 3×10⁻¹` ΔE2000 at the shipped grid) is a separate, graded,
  machine-independent claim.

### 3.24 ★★★ The 4.2.5.4 CONFORMANCE DEFECT — in iccce, not in lcms2. **The ledger's first row recording that this project's shipped code did not conform, and it was measured before it was found**

**Corrected 2026-08-12 at commit `fd34a44`** — *"bpc: iccce was WRONG at
4.2.5.4 — lcms2 conformed, we did not"*. Sourced by
`icc-spec-librarian`; fixed by `icc-engineer`; **filed from
`crates/iccce-cmm/src/bpc.rs` read at the tip** *(verified — lines
174–199 and 251–273)*.

> **★★★ THIS CLOSES §3.18.6 AND §7.11's NEWLY-OWED ITEM 1, IN THE
> DIRECTION THEY NAMED AS POSSIBLE.** §3.18.6 asked *"does ISO/CD 18619
> 4.2.5.4 specify `outRamp[first]`, or the `InitialLab` behaviour lcms2
> implements?"* and pre-committed: *"**if ISO names lcms2's, iccce is
> WRONG — not divergent — and the engineer changes the code**"*. **ISO
> names lcms2's.** The clause's final paragraph, verbatim: *"If the mid
> range is straight (as determined above) then the DestinationBlackPoint
> **shall be the same as InitialLab**."* 4.2.5.1's control-flow summary
> says it a second time.

| ID | What | Class | Tolerance | Observed |
|---|---|---|---|---|
| **★★★ NC-164** | **The straightness short-circuit returns `InitialLab`, the whole triple, unchanged** — ISO/CD 18619 **4.2.5.4** final paragraph, transcribed verbatim in the corpus and quoted at the site. **`outRamp[first]`, which iccce returned, appears in the whole of clause 4.2.5 only as `MinL`** — a threshold and a `yRamp` anchor — **and in 4.2.5.3's validity test; it is not a black-point candidate in any branch** | **normative-rule-conformance** *(inheriting the transcription risk of §1's definition, **and additionally the risk that 18619 is a COMMITTEE DRAFT**)* | **behavioural, 0/1** — the branch returns `initial_lab` or it does not | **conforms as of `fd34a44`** *(verified — `bpc.rs` read: the branch is `return initial_lab;`, with the clause quoted immediately above it)*. **Before `fd34a44` it did not** |
| **★★ NC-164a** | **The cost of the defect, measured BEFORE it was found** | **implementation-cross-check** *(the figure is **NC-142's**, re-attributed rather than re-measured)* | **∞ — REPORTED** | **8,166 8×10⁻² ΔE76** on `USWebCoatedSWOP` (`ΔL* 0,081 67`, chroma **exactly 0**) — ★ **100 % of the two implementations' divergence on that arm.** It is now attributed: **the whole of it was iccce's defect** |

#### 3.24.1 ★★★ Re-attribution of NC-142, stated as a re-basing rather than an edit

**NC-142 is not edited.** Its number was and is correct. What changes is
the sentence beside it:

| | Before | Now |
|---|---|---|
| **NC-142's `swop` figure** | *"a measured difference whose **ATTRIBUTION IS OPEN**"* (§3.18.6) | **Attributed: iccce was non-conformant; lcms2 conformed. The divergence was our defect in its entirety** |
| **What may be said about lcms2** | *"no document may describe this difference as lcms2 departing from the standard"* | **The prohibition is discharged — and it resolved the other way.** lcms2 is the one that conformed |

★ **This is why §3.18.6 was worth writing.** A section that had filed
the divergence as a *finding against lcms2* would have had to be
retracted; a section that filed it as *a measured difference with one
identified line and no attribution* needed only a sentence added. **The
measurement was usable precisely because its interpretation was
withheld.**

#### 3.24.2 ★★ The corollary that came with the fix, which is NOT a bug fix

**The function's return type widened from `L*` to a full `Lab`.** The
reason is a second reading of the same clause family: **4.2.5.2.1 zeroes
chroma only for CMYK**, so on a **Gray or RGB** LUT destination ISO
itself yields a **chromatic** `DestinationBlackPoint` — and the
short-circuit is **the only branch of 4.2.5 that can return one**.
Neutralising at the return would have been *a second, quieter
departure* *(the phrase is the site's own; verified — read)*.

**Cost today: zero.** 4.2.6 ignores `a`/`b` downstream. ★ **And the
correctness is not zero**, which is exactly the distinction rule 4
exists to hold open: a departure whose current price is nil is still a
departure, and the price changes the day something consumes the chroma.

#### 3.24.3 ★★ What this says about the corpus, and it is not flattering

**The corpus did not catch this, and could not have.** 4.2.5.4 had
**not been transcribed verbatim**, so nothing in
`D:\Dev\Rag-Specialized\ICC_Spec\` said `outRamp[first]` was wrong. The
sequence was:

1. **The defect shipped** (Pass 5's `bpc.rs`), sourced against a corpus
   that was silent on the point.
2. **Pass 5c measured its consequence** — 8,167×10⁻² ΔE76 — **without
   being able to attribute it**, and named the single line it had to be
   (§3.18.6).
3. **The question was dispatched outward**, and the answer required
   going back to the document rather than to the corpus.

★ **A corpus gap and an implementation bug with the same root.** The
lesson is not *"transcribe more"* — it is that **the measurement is
what made the gap findable**: nobody would have re-read 4.2.5.4 if a
number had not been sitting there unexplained.

#### 3.24.4 What §3.24 does NOT claim

- **That lcms2 is an authority.** It conformed **here**, at **one
  clause**, at **one pin**. **DL-027** stands unchanged: lcms2 has two
  black-point estimators and a branch this project's first reading did
  not trace.
- **That the corrected code is verified.** ★ **NC-164 is a
  behavioural row read from source, not a differential run.** The
  runner's `pass=142 fail=0` (NC-160) was taken on the corrected code
  *(reported)*, but **no record in it grades the short-circuit's return
  value against the clause** — the rows that touch this area
  (`pass5c/…`) grade the *branch selection* and the divergence, and
  **NC-142's 8,167×10⁻² should now be expected to COLLAPSE**, which
  **nobody has re-measured and this filing does not assert**.
- **That the ISO/CD tier caveat is retired.** 18619 is a **committee
  draft** in this project's corpus and every consequence drawn from it
  inherits that, including this one.
- **That other clauses of 4.2.5 have been re-checked.** One paragraph
  was sourced and one branch corrected. **Nothing swept the rest**, and
  §4.6's known ISO internal contradiction is untouched.

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

> **Dated note, 2026-08-11 (Pass 3 filing): NA-003's other half has now
> landed, in the CMM layer where this entry said it belonged.**
> `MatrixTrc::pcs_to_device` clamps each linear component to [0,1]
> **before** the inverse TRC, per **F.8–F.16**, and the order is
> asserted on measured output — **NC-027**. So the layering decision has
> been discharged as designed: `iccce-color` still does not clamp, and
> the CMM does, at the point the normative text specifies. *(verified —
> read.)*

> **★★ Second dated note, 2026-08-11 (Pass 4 filing) — NA-003's CLAUSE
> CITATION WAS WRONG, and the correction inverts a finding that was built
> on it. This entry is not edited; this note is the correction.**
>
> The paragraph above cites **clause 6.4** for the proposition that
> out-of-range colours *"shall be clipped on a per-component basis"* on
> integer conversion while **no clipping** is performed for float32
> encodings, and says that rule *"binds the CMM/profile layers"*. **It
> was written from recollection**, and `icc-spec-librarian` has since
> read the primary text. `TOLERANCES.md` **§5.2** carries the full
> correction append-style and **is `icc-conformance`'s file — it is not
> duplicated here.** What this ledger must carry is the effect on its own
> rows *(verified — `TOLERANCES.md` §5.2 read 2026-08-11; the corpus
> file it rests on is `icc__s__computational_models.md` §4, ambiguity
> **A39** resolved)*:
>
> 1. **Clause 6.4 is titled *"Converting between PCSXYZ and PCSLAB
>    encodings"* and every quantity in it is a PCS value.** The words
>    NA-003 recalled are real; **the subject is not.** They are about the
>    PCS, not about device values.
> 2. **The clause that governs device encoding is 6.5**, and its float32
>    permission is **doubly gated** — it applies *"when encoding using
>    float32Number values in **DToBx and BToDx** tags"*, and 8.3.3/8.4.3
>    do not list those tags among the ones a three-component matrix-based
>    profile may contain. **The escape hatch is structurally unreachable
>    from a matrix/TRC model.**
> 3. **Therefore a conforming F.8–F.16 evaluation cannot emit a device
>    value above 1,0** — by entailment, not by a separate output-clamp
>    rule.
>
> **★ What that does to NC-043, which is the row that relied on it.**
> NC-043's *"Which implementation the specification supports"* field
> raises exactly this hypothesis — *"clause 6.4 … may make lcms2's float
> excursion conforming and iccce's clamp merely stricter"* — and **that
> hypothesis is now refuted. The direction inverts**: lcms2's measured
> 1,000 120 is **arithmetically unreachable from the mandated model**, so
> it is evidence that the **input** clamp (F.10/F.13/F.16) was not applied
> at all. **iccce is not "stricter" — there is no stricter available.**
> NC-043's row is **left exactly as written**, per this document's
> append-only rule; this note supersedes its third field.
> **Two hedges must survive every restatement**, and they are the reason
> this is still not a conformance verdict: **(A39b)** clause 5's entire
> conformance requirement on a consumer is *"shall have the ability to
> **read** the profiles as they are defined"*, and the computational-model
> `shall`s are phrased about the **profile**, so the word is
> **divergence**, not non-conformance (rule 7); **(A39c)** both profiles
> in that measurement are **v2.1.0** and **the v2 half is UNSOURCED** —
> Annex F's text is version-neutral and the corpus treats it as applying
> to both, **an assumption labelled as one**.
> **And what remains unmeasured is not small:** every excursion observed
> was **1-lsb boundary residue at white**, because sRGB ⊂ Adobe RGB makes
> real clipping impossible in that direction. Pass 4 did **not** close
> this either — on SWOP → sRGB, which *does* clip genuinely, `transicc`
> returned **0 of 1023 components** outside `[0,1]` at every intent,
> because that destination's TRC inverse is a **tabulated** reverse curve,
> which is lcms2's saturating path (corpus **M3**). **So the observed cost
> of this divergence remains ≤1.2×10⁻⁴ device units at white, and that
> number must never be restated as a bound on the divergence in general.**
> **The lesson, which is this project's own rule turned on itself:** the
> sentence was written from recollection of a clause number, in a document
> whose charter is that claims carry provenance, and it was then **relied
> on by a differential finding**. Rule 2 — *never write colour maths from
> memory* — extends to **clause numbers**, and DL-014's requirement to
> name the corpus file at the citation is what would have caught it.

> **★★ DATED STATUS, 2026-08-12 (the Pass 4c filing) — THE HEDGE ABOVE
> WAS NECESSARY, AND IS NOW SHOWN TO BE, BY MEASUREMENT. The note is not
> edited.**
>
> The sentence *"the observed cost of this divergence remains ≤1.2×10⁻⁴
> device units at white, **and that number must never be restated as a
> bound on the divergence in general**"* was written from **method
> discipline alone** — its author had only the small number and refused
> to generalise from it, reasoning that the destination's **tabulated**
> inverse TRC put lcms2 on its **saturating** path (corpus **M3**) and
> that a different destination might not.
>
> **That reasoning has now been tested with a controlled A/B on one
> variable, and it is right.** Same source, same 625-point CMYK grid,
> same intent, three destinations differing **only** in inverse-TRC kind:
> a **tabulated** destination gives **0 / 1875** components outside
> `[0,1]`, while **analytic** `para` inverses give **16 / 1875** (worst
> **1.380557e-1**) and **137 / 1875** (worst **3.053984e0**). Raw
> `transicc` output confirms the excursions are **genuinely negative**,
> not boundary residue. **Full record and scope: §3.16.1.**
>
> **The real magnitude is up to 3.05 device units — roughly 2.5×10⁴
> times the 1.2×10⁻⁴ that was fenced off.** Had the hedge not been
> written, the small number would by now be sitting in this register as
> *"the cost of NA-003's divergence"*, and it is wrong by four orders of
> magnitude.
>
> **★ Why this is filed as a finding about METHOD and not just about
> numbers.** This project collects instances of instruments that caught
> something (**DL-016**, **DL-020**, **DL-025**). **This is a different
> and rarer kind: a SENTENCE that caught something.** A hedge costs one
> clause, it is invisible when it is unnecessary, and there is no way to
> tell the two cases apart in advance — which is why the register's
> standing practice is to write it every time. **The project has many
> hedges that were merely prudent. This is the first that is demonstrably
> load-bearing**, and it is the argument for the practice.
>
> **What is still NOT measured, and the limit is real:** **both arms
> measure lcms2 ALONE** (`oracle-behaviour-at-pin`). **iccce's side was
> not run**, so **no divergence between the two implementations has been
> measured here.** iccce clamps by **NA-004**, so the difference *would*
> be the excursion itself — **but that is an inference, not an
> observation, and must not be written as one.**

### NA-004 — the `pow(negative, fractional)` guard. **A choice inside a stated non-requirement — NOT a deviation from normative text, and the register now distinguishes the two**

| Field | Value |
|---|---|
| **The choice** | `curve.rs::eval_parametric` routes every `pow` through `pow_guarded`: `if base > 0.0 { base.powf(exp) } else { 0.0 }`. A negative or zero base yields **0.0** instead of **NaN**. *(verified — read.)* |
| **Kind — stated in the row, because NA-001 and this are not the same object** | **A choice inside a hole the standard declares open.** NA-001 departs from **printed normative text** (ICC.1:2022 writes a decimal breakpoint; iccce uses the rational). This does not: clause **10.18** declares complex/undefined parameter combinations **explicitly undefined** — *a stated non-requirement, which the corpus rightly notes is stronger than silence* — and nothing specifies what `pow` does with a negative base under a fractional exponent. **Never restate this row as a conformance departure.** |
| **What it diverges from** | **ICC's own sample code**, which calls `pow` unguarded; **lcms2 guards**, and iccce follows lcms2. Corpus, VERBATIM: *"lcms2 additionally guards `e > 0` before `pow`; ICC's code does not. **A real behavioural difference between the two implementations on malformed/extreme parameters.** Follow lcms2 (guard), and record it as a deliberate divergence from ICC's sample code."* *(verified — `icc__type__curve_parametric.md` §Guards, `evidence: primary_spec` for clauses 10.6 / 10.18 / Annex F.1.)* |
| **Cost, and its exact status** | *"None on well-formed curves"* — consistent with the code (for `a > 0`, `b ≥ 0` the base is positive across the branch, so the guard never fires) and **analytic, not measured. Evidence class: corpus-derived-bound.** No test in this repository compares guarded against unguarded output. **Nobody may write "measured to cost nothing."** |
| **★ Two limits the module doc's wording does not carry** | (1) *"turns NaN into a defined, **reported** value"* — it is **defined; nothing reports it.** `Trc::eval` returns a bare `f64` with no diagnostic channel, so the substitution is **silent at the evaluation site**. That may well be right (invariant §3.2 binds the *parser*, and an undefined parameter combination is not automatically a malformation), but *"reported"* asserts a disclosure surface that does not exist. (2) The guard **also fires on one well-formed input**: parametric **type 0 with `g = 0`** is the constant curve `y = x⁰ = 1`, and at exactly `x = 0` the base is `0.0`, which is not `> 0.0`, so the result is `0.0` while every `x > 0` gives `1.0` — **a step at the origin on a degenerate constant curve**. Its *inverse* is already refused by name; its forward evaluation is not. **No test exercises `g = 0` forward.** *(both derived here from the code as written, 2026-08-11.)* |
| **Where** | `crates/iccce-cmm/src/curve.rs` — module doc §"Named divergence from ICC's sample code", `pow_guarded`. |
| **Decision record** | `ARCHITECTURE.md` **DL-015**. |
| **Revisit if** | ICC's sample code adds the guard, or a later edition specifies the case (either ends the divergence); or a difftest measures iccce and lcms2 disagreeing here, which would mean the two guards are not the same guard — and the corpus's statement about lcms2 is a **source reading**, not a measurement (the DL-011/DL-012 distinction). |

### NA-005 — the matrix/TRC model uses colorant tags **as stored** and never consults `wtpt` or `chad`

| Field | Value |
|---|---|
| **The assumption** | `MatrixTrc::from_profile` builds its matrix from `rXYZ`/`gXYZ`/`bXYZ` **exactly as the file stores them**, and `MatrixTrcTransform::convert` chains source-forward → destination-inverse with **no adaptation step anywhere**. `iccce-cmm` does not read `wtpt`, does not read `chad`, and **does not call `iccce_color::adapt` at all.** *(verified — the imports and both functions read, 2026-08-11.)* |
| **Why that is correct, and exactly how far** | ICC's PCS is D50-referenced, so a conformant profile's colorants are already media-relative and D50-adapted (the `chad` tag records the adaptation that **was already applied**, it is not an instruction to apply one). Chaining forward and inverse therefore **is** the media-relative colorimetric conversion. The module doc states this. **The assumption is: the profile is conformant on this point** — and nothing in the build path checks it. |
| **Cost** | **Zero on a conformant profile; unbounded on a non-conformant one** (a white-point-sized error, i.e. the loud kind rather than the quiet kind). **UNMEASURED**, and no code path measures it: the only place the property is checked anywhere in this project is **NC-031**, in a *test*, on **one** profile. |
| **What would close it** | A build-time check that the colorant sum is within a justified distance of the PCS white, reported as a diagnostic rather than repaired (invariant §3.2) — which is exactly the arithmetic NC-031 already performs and already has a justified bound for. Recorded as a candidate, not a decision. |
| **Why it is registered at all** | Because *"in a well-formed profile"* is an assumption, and an unstated assumption is indistinguishable from a bug (invariant §3.3). The module doc names it at the site; this row gives it a **status** (unmeasured) and a place to be invalidated from. |

> **★ Dated correction to a prediction this ledger and `ROADMAP.md`
> both made, 2026-08-11 (Pass 3 filing) — NA-002's cost has NOT come
> due.** §7.2 and the ROADMAP's Pass 3 annotation both said *"Pass 3 is
> the Pass that owes the measurement… sRGB→AdobeRGB adapts"*, and
> `NEXT_SESSION.md` carried it as live. **As implemented, Pass 3 does
> not adapt anything.** Chromatic adaptation is not in the matrix/TRC
> path at all (NA-005 above): both profiles' colorants are already
> D50-referenced, so no CAT is applied and **`iccce-color::adapt` is
> still not called by any transform in this project.** *(verified — the
> live source read; `iccce-cmm` imports only `Mat3` and `Xyz` from
> `iccce-color`.)* **NA-002 therefore remains an unexercised entry and
> its cost is still not owed** — the prediction was about what Pass 3
> would need, not about what it built. It becomes owed at the first
> transform that adapts, which is now most likely **Pass 4** (absolute
> intent, and any path where a `chad` is consulted). **The prediction is
> left standing in §7.2 and in the ROADMAP as the record of what was
> expected**; this note is how it gets corrected, per this document's
> own append-only rule.

### NA-006 ★ — CLUT interpolation is **n-linear**, a choice inside an ICC.1 SILENCE, and its cost is a **corpus-derived bound, NOT measured**

*(added 2026-08-11 at the Pass 3 closure filing — the code landed in
`fc5ff58` **(reported)**; the entry is filed here the day the
approximation entered the tree, not the day a transform uses it.)*

| Field | Value |
|---|---|
| **The approximation** | `crates/iccce-cmm/src/clut.rs` evaluates multi-dimensional colour lookup tables by **n-linear interpolation** (multilinear; trilinear in the 3-input case). *(verified — module doc and `Clut::eval` read.)* |
| **Kind — and it is a THIRD kind, distinct from NA-001's and NA-004's** | NA-001 departs from **printed normative text**. NA-004 is a choice inside a **stated non-requirement** (10.18 declares the case undefined). **This is a choice inside a SILENCE**: corpus ambiguity **A16**, confirmed SILENT against the primary spec by an exhaustive search — the only normative sentence on the subject is a constraint on the *profile author* for the two-grid-point case. ICC.1 **does not specify** the interpolation between CLUT grid points at all. The register now carries all three kinds and they must not be conflated. |
| **Why n-linear was chosen, per the module doc** | It is fully specified by its own definition (no scheme variants to pick between), it works at every input dimensionality, and it is **exact on the class of functions the spec's own two-grid-point sentence contemplates** — a property asserted by a test (`reproduces_multilinear_exactly`). |
| **★ Cost — and its exact status** | **Up to ~1 ΔE** between trilinear and tetrahedral in regions of high CLUT curvature. **Evidence class: corpus-derived-bound**, transcribed from `ICC_Spec\icc\icc__type__lut8_lut16.md`, which states the two *"differ by up to ~1 ΔE … at or above the perceptibility threshold, so this choice is measurable, not academic"* *(verified — the corpus line read)*. **iccce has NOT measured it, and cannot yet**: tetrahedral is **deliberately absent** (lcms2's cube decomposition has several published variants and the corpus does not carry lcms2's; it will be **sourced before it is written, not recalled**). **Nobody may restate this as "measured at ~1 ΔE."** |
| **★ A prose defect at the site, reported not repaired** | The module doc says *"iccce's choice, per rule 4 (**named and measured**): n-linear interpolation."* **It is named. It is not measured** — no test, no run, and no comparison against any other scheme exists in this repository. The claim as written asserts a discharge of rule 4 that has not happened. The file is the engineer's; this row is the correction. |
| **Why this matters more than NA-001's ~10⁻⁵** | This is the **first named approximation in the project whose bound sits AT the perceptibility anchor** rather than five orders of magnitude below it. NA-001 can never show up in colour; **this one can**, and it is the single largest expected iccce-vs-lcms2 deviation in Pass 4. |
| **Consequence for Pass 4, which is where it becomes owed** | Pass 4's differential tolerances **must budget for the interpolation-method difference and say so** — a Pass 4 tolerance set without it is set on the wrong quantity, exactly as DL-013's forced BPC would be. And the budget must not silently become the *justification*: a tolerance wide enough to swallow ~1 ΔE cannot also demonstrate agreement. |
| **Where** | `crates/iccce-cmm/src/clut.rs` — module doc §"The A16 silence, and iccce's named choice", `Clut::eval`. |
| **Revisit if** | lcms2's tetrahedral decomposition is sourced (then the cost becomes **measurable**, and measuring it is owed); or a later ICC edition specifies an interpolation (which would end the silence and make this conformance rather than choice). |

> **★★ Dated status, 2026-08-11 (Pass 4 filing) — NA-006's cost is
> MEASURED, and the mechanism this entry predicted was WRONG. The entry
> above is not edited; this note is the correction.**
>
> **1. The cost is no longer a corpus-derived bound.** `NC-050` prices
> the n-linear choice directly, from the CLUT and the two algorithms
> alone with **no lcms2 output in it**: **1.5741 ΔE2000 max (mean
> 0.043 86) on SWOP's `A2B0` perceptual table** and **0.254 23 (mean
> 0.038 54) on `A2B1`**, propagating to **1.6639 ΔE00** end to end
> through the sRGB destination. The corpus's *"up to ~1 ΔE"* was **the
> right order of magnitude and an underestimate on one of the two
> tables.** The entry's *"nobody may restate this as measured at ~1 ΔE"*
> is superseded by: **measured at 1.5741 on one table of one profile at
> one pin** — and the coverage words are part of the claim.
> **Consequence at the site:** `clut.rs`'s module doc says *"per rule 4
> (named and measured)"*, which the last filing reported as asserting an
> undischarged obligation. **It is now true.** The item is closed by fact.
>
> **2. The entry says the cost "cannot yet" be measured because
> tetrahedral is deliberately absent. That reasoning was wrong on both
> halves.** It was not necessary to *implement* tetrahedral in
> `iccce-cmm` to price the choice — reimplementing the comparison arm
> **inside the harness**, and grading that arm against `Lut16Model` at
> 10⁻⁹ (**NC-051**, observed **0.0 exactly**), was enough. **A named
> approximation can be priced by an apparatus that is not shipped**, and
> that is now the pattern to reach for.
>
> **3. ★ And "lcms2 tetrahedral" was a prediction, not a fact.** For
> **four** inputs lcms2 runs a **hybrid** — linear in C, Sakamoto
> tetrahedral in M/Y/K — **read at the pin** (**NC-056**). So the ~1 ΔE
> bound this entry transcribed, which comes from the
> trilinear-vs-tetrahedral literature, **is not the bound that applies to
> this comparison at all**, and the Pass 4 blocker recorded in three
> documents as *"source lcms2's tetrahedral cube decomposition"* was
> aimed at the wrong object. **The prediction is left standing** in the
> entry above, in `ROADMAP.md`, and wherever else it was carried; this
> note and NC-056 are how it is corrected.
>
> **4. What has NOT changed.** iccce's CLUT interpolation is still
> **n-linear**, still a choice inside an **ICC.1 silence** (A16), and
> still not a conformance question. Tetrahedral is still **not
> implemented and still not sourced** for iccce's own use — and the case
> for implementing it is now weaker than it looked, because matching
> lcms2 would mean matching a **channel-order-asymmetric** scheme. That
> is a design decision nobody has taken and this note does not take it.

> **★★ Third dated status, 2026-08-11 (Pass 4b filing) — NA-006's
> measured cost is a fact about ONE DIRECTION. In the B2A direction it
> is ZERO, and the Pass 4 statement of it was half a rule. The entry and
> the note above are not edited; this note is the correction.**
>
> **1. The number that was filed is an A2B number.** NA-006's cost —
> **1,5741 ΔE2000 on SWOP's `A2B0`, 0,254 23 on `A2B1`** (NC-050) — was
> measured against lcms2's **four-input hybrid** in the device→PCS
> direction. **In the PCS→device direction the same comparison has an
> envelope of exactly zero**, because `cmsio1.c`'s `_cmsReadOutputLUT`
> calls `ChangeInterpolationToTrilinear` for **every** Lab-PCS LUT, and
> **trilinear over three inputs *is* iccce's n-linear** (**NC-067**,
> §3.11.3). That is **every CMYK output profile in this machine's colour
> directory**. Anyone quoting *"n-linear costs up to 1,57 ΔE against
> lcms2"* must now also say **which direction**.
>
> **2. It does not make the choice safer — it makes the comparison
> blinder, and that is why NC-067 exists.** With the method difference
> at zero, a B2A cross-check **cannot show that iccce's interpolation is
> right, only that it is the same**. The counterfactual row prices what
> the comparison *could* have seen: **139× / 99×** the observed
> disagreement. An apparatus not shown able to detect the effect it is
> looking for is not an experiment (**DL-018**), and here the effect is
> a *method* rather than a deleted requirement.
>
> **3. What has NOT changed.** iccce's CLUT interpolation is still
> **n-linear**, still a choice inside an **ICC.1 silence** (A16 — and the
> corpus's seventh pass restates the silence for `mAB `/`mBA ` too,
> adding that it *"applies to **non-uniform** grids too, where the choice
> matters more"* *(verified — `icc__type__lutAtoB_lutBtoA.md` §8 read)*).
> lcms2's trilinear override is likewise **a policy, not a clause** — its
> own comment calls it *"controversial stuff"*. **Two choices agreeing is
> not conformance.**
>
> **4. The general rule that comes out of it is `ARCHITECTURE.md`
> DL-021**, filed with this instance and two others from the same day.

### NA-007 — the absolute intent uses `wtpt` **as stored**, and does not un-apply `chad`

*(added 2026-08-11 at the Pass 3 closure filing; code in `6873df1`
**(reported)**. The **sibling** of NA-005, which registers the same
posture for the colorant tags.)*

| Field | Value |
|---|---|
| **The assumption** | `MatrixTrcTransform::convert_with_intent`'s `Intent::Absolute` scales the PCS by the per-component ratio **`mw_src / mw_dst`**, where each `mw` is that profile's `mediaWhitePointTag` **exactly as the file stores it**. `chad` is **not** un-applied. *(verified — the function and the `Intent::Absolute` doc read.)* |
| **Why that is right, and exactly how far** | Sourced, not assumed: **6.2.1 NOTE 1 / Annex E.4** — `chad` is a **provenance record** of an adaptation that was already applied, not an instruction to reverse one. And **9.2.36**: in a **conforming v4** profile `wtpt` **shall** already be the D50-adapted value, from which the sourced consequence follows that **absolute ≡ media-relative for a conforming v4 display profile — not a bug**. |
| **★ Where the assumption actually bites** | **v2 profiles**, where the meaning of a **non-D50 `wtpt`** is corpus ambiguity **A4b — UNVERIFIED**. The code records that *"implementation consensus says use it as stored, which is what this code does with the fact recorded here"* *(verified — read)*. **Implementation consensus is not a specification reading**, and this row exists so that sentence cannot quietly become one. |
| **Cost** | **UNMEASURED, and currently unmeasurable through the shipped surface**: the CLI refuses every intent but media-relative, so **no differential can reach this code path** (§3.8.9). The only evidence is unit tests — a direction pin against the corpus's printed intermediates (0.7067/0.85 = **0.831412**, with the backwards reading 1.202773 asserted **absent**) and the two refusal tests. |
| **A design property worth recording, because it is deliberate and invisible** | The ratio is computed **first** (`mw_src.x / mw_dst.x`), so equal whites give **exactly 1.0** by IEEE division and the sourced 9.2.36 consequence is **bit-exact rather than within-rounding**. *(verified — the comment and the arithmetic order read.)* An implementation that multiplied by `mw_src` and then divided would agree to rounding and not to the bit, and the difference would never show in colour — which is precisely the class of thing this project writes down. |
| **Refusal, not default** | A missing or degenerate (`≤ 0` component) `wtpt` yields **`AbsoluteNeedsWtpt`**, refused by name. Substituting `Xi` would silently make absolute ≡ relative **for a profile where that may be false** — the substitution being indistinguishable from a correct answer, which is invariant §3.2 at the CMM layer. |
| **Where** | `crates/iccce-cmm/src/matrix_trc.rs` — the `Intent` enum doc, `convert_with_intent`, `ModelError::AbsoluteNeedsWtpt`. |
| **Revisit if** | A4b is resolved by a sourcing dispatch (v2 `wtpt` semantics); the CLI exposes the intent, at which point a differential becomes possible **and the cost becomes owed**; or any path is added that consults `chad` — which is also **NA-002's** revisit condition. |

> **★★ Dated status, 2026-08-11 (Pass 4 filing) — NA-007's cost is
> MEASURED, it is enormous, and the sentence this entry was written to
> protect is exactly the sentence that came due.**
>
> This entry recorded the assumption as *"unmeasured, and currently
> unmeasurable through the shipped surface"*, because the CLI refused
> every intent but media-relative. **`490191b` exposed all four**, and
> the first differential to reach the absolute intent found **11.217
> ΔE2000 max, mean 4.670** against lcms2 (**NC-053**) — two orders of
> magnitude more than at any other intent.
>
> **The cause is precisely where this entry said the assumption would
> bite.** The entry's own words: *"Where the assumption actually bites —
> **v2 profiles**, where the meaning of a non-D50 `wtpt` is corpus
> ambiguity **A4b — UNVERIFIED**… implementation consensus is not a
> specification reading, and this row exists so that sentence cannot
> quietly become one."* **The destination sRGB profile is v2 and
> display-class with a D65 `wtpt`; lcms2 substitutes D50 by its own v2
> rule; iccce uses the stored value.** The ratio is a **32 % error in
> `Z`**, and modelling lcms2's substitution collapses the disagreement
> **517×** (**NC-054**). **The register worked**: an assumption that was
> named a Pass early was found, by measurement, to be the whole of an
> 11 ΔE divergence, and nobody had to discover it as a mystery.
>
> **What is NOT decided.** Whether iccce's reading or lcms2's is right —
> **A4b is still UNVERIFIED** *(verified — corpus read this filing)*, and
> per rule 7 the ledger records the difference without a verdict. **The
> raw comparison is deliberately REPORTED, NOT GRADED**; the graded row
> at that intent is the modelled one. Decision record
> `ARCHITECTURE.md` **DL-019**.
>
> **What is still unmeasured here.** `chad` remains **not un-applied**,
> and nothing in Pass 4 exercised a profile carrying one, so that half of
> the entry is untouched. And **`iccce_color::adapt` is STILL called by
> no transform in this project** — checked against the live source at a
> **third** consecutive filing rather than carried: `transform.rs`
> imports `Xyz` and `D50` from `iccce-color` and nothing from `adapt`
> *(verified — the `use` block read)*. **NA-002's Bradford cost is still
> not due.**

### NA-008 — the grayTRC inverse recovers the connection scalar from the **achromatic channel of the profile's own PCS encoding**, and discards chromatic content. Cost **UNMEASURED**, and it is not a rounding cost

*(Filed 2026-08-11 at the Pass 4 evaluation-surface filing, by
`icc-librarian`, from a reading of the shipped code. **New with the
entry**, so per this register's own rule an unmeasured cost is permitted
only while that remains true.)*

| Field | Value |
|---|---|
| **The assumption** | `GrayTrc::pcs_to_device` computes the F.2 connection scalar as **`Y / Yn`** for a PCSXYZ gray profile and as **`L* / 100`** (of the D50-relative Lab) for a PCSLAB one, clamps it to `[0,1]`, and inverts the curve. **Any chromatic content in the PCS input is discarded** *(verified — read; the code says so in those words rather than hiding it)*. |
| **What IS specified, and what is not** | The **forward** direction is normative and iccce follows it exactly: `connection = grayTRC[device]`, then multiply by the PCS white **triple** — which is what NC-060 asserts. Annex F.2's inverse, `device = grayTRC⁻¹[connection]`, is likewise normative **for a connection value**. What no clause supplies is **how to obtain a connection value from a PCS colour that is not on the neutral axis** — a monochrome device cannot reproduce chroma, so *something* must be projected away, and the specification does not say what. |
| **Why it is a named choice rather than a triviality** | The two encodings project **differently**. `Y/Yn` is linear in luminance; `L*/100` is not. Feed the same non-neutral PCS colour to two gray profiles that differ **only** in PCS kind and they return **different device values** — each self-consistent with its own forward model, and neither wrong. The module doc's justification is that the achromatic channel is *"the same channel NOTE 1 says the tag is usually derived from"*, which is a **rationale**, not a rule. |
| **Cost** | **UNMEASURED.** And note what kind of quantity it is: **not a rounding error but a gamut-mapping decision**, so its magnitude is bounded only by how far the input sits off the neutral axis. On the neutral axis it is **exactly zero**, which is why every test in `gray_trc.rs` sees none of it — both tests feed neutrals. |
| **What would measure it** | A grid of **non-neutral** PCS colours converted to gray by (a) `Y/Yn`, (b) `L*/100`, and (c) lcms2, with the results compared in ΔE2000 after re-expansion to the PCS. That is also the first comparison of any gray path against another implementation, and Pass 4 owes it. |
| **Where** | `crates/iccce-cmm/src/gray_trc.rs::pcs_to_device`, and the module doc's inverse paragraph *(verified — read)*. |
| **Related rows** | **NC-060**, **NC-061** (both on the neutral axis, so both blind to this); **NA-005** (colorants used as stored — the same shape of entry: a stated reading of what a tag means). |
| **Revisit if** | ICC.1's F.2 is re-read and turns out to constrain the inverse's domain; a gray differential runs; or a Pass adds gamut mapping, at which point this becomes a special case of a general policy rather than a local choice. |

> **★ Dated status, 2026-08-11 (Pass 4b filing) — the gray differential
> RAN and NA-008 is STILL UNMEASURED, because it ran in the other
> direction. The entry is not edited; this note is the correction.**
>
> This entry's *"what would measure it"* row, `NEXT_SESSION.md`, and
> `ROADMAP.md` all recorded that a gray comparison against lcms2 would
> give NA-008 its first measurement. **Pass 4b's §C is that comparison**
> — `ewgray22.icm` **→** the system sRGB profile, 69 points, agreeing to
> **9,686×10⁻⁵ device / 2,169×10⁻² ΔE2000** (**NC-079 … NC-083**) — and
> it measured **`GrayTrc::device_to_pcs`**, the *forward* model.
> **NA-008 is a property of `pcs_to_device`**, which is reached only when
> a gray profile is the **destination**, and **no differential has ever
> put one there.** *(verified — README §15.2's own table gives §C's
> direction as `GRAY→RGB`.)*
>
> **So the cost remains UNMEASURED, and it remains a *gamut-mapping*
> quantity rather than a rounding one** — zero on the neutral axis,
> bounded only by how far the input sits off it. **What would measure it
> is unchanged and now cheap**: sRGB (or SWOP) **→ a gray profile**, over
> a grid of **non-neutral** PCS colours, comparing `Y/Yn` against
> `L*/100` against lcms2. `fixtures/synthetic/v2-gray-curv-gamma.icc`
> exists and `Chain` selects `DestModel::Gray`, so nothing blocks it.
>
> **The lesson is the one this project keeps re-learning:** *"a gray
> differential"* named a **comparison**, not a **direction**, and the
> entry that predicted what it would measure did not say which way it
> ran. Same shape as **DL-021**.

> **★★ SECOND DATED STATUS, 2026-08-12 (the Pass 4c filing) — NA-008
> SPLITS INTO TWO HALVES, one is now probed and the other HAS NO
> INSTRUMENT IN EXISTENCE. The entry and the note above are not edited.**
>
> Both notes above, and both *"what would measure it"* rows, treat
> NA-008 as **one** owed measurement. **It is two**, and only now is that
> visible, because the cheap half was finally run.
>
> **Half 1 — the CROSS-CHECK (gray as destination, against lcms2).
> REACHABLE, and PROBED.** `sRGB → ewgray22.icm`, **729 RGB points**,
> **max 3.382353e-05 device**; and — the part that matters — the residual
> is **no larger off the neutral axis (3.247059e-05 over 720 points) than
> on it (3.382353e-05 over 9)**. **SCRATCH PROBE, not a graded row**: it
> is not in the harness and **nothing pins it**, so it has no NC number
> (§3.16.2). What it establishes is that iccce's `pcs_to_device` gray arm
> **agrees with lcms2 on this profile**, which is a genuine first — no
> differential had ever put a gray profile in the destination slot.
>
> **Half 2 — the NAMED APPROXIMATION ITSELF. UNMEASURABLE TODAY, and
> agreeing with lcms2 does not touch it.** NA-008 is the choice between
> **`Y / Yn`** (PCSXYZ) and **`L* / 100`** (PCSLAB). **Discriminating
> them requires a PCSLAB gray profile, and EVERY gray profile in reach is
> PCSXYZ** — `ewgray18.icm`, `ewgray22.icm`, `BlackWhite.icc`
> *(reported — headers read via `iccce inspect`)*, and both
> `fixtures/synthetic/v2-gray-curv-*.icc` *(**verified** —
> `MANIFEST.md` read: both are "v2.4.0.0 mntr GRAY monochrome")*. The
> full table is at **§3.16.3**.
>
> **★ Two things follow, and the second is the sharper one.**
> **(1)** `tools/gen-profiles` **owes a PCSLAB gray fixture**, and it is
> the **same shape of owed instrument as Pass 5's non-zero device black**
> (**NA-009**, §7.8 item 1, unwritten since Pass 5): **sourcing is not
> measuring.**
> **(2) Agreeing with lcms2 is not measuring the projection choice
> either** — **lcms2 makes one of the two choices too**, so a
> cross-check against it is blind to the difference between them by
> construction. **Half 1 cannot be quoted as evidence about Half 2**, and
> the temptation to do so is exactly why this note exists: a 3.4×10⁻⁵
> agreement sitting next to an unmeasured approximation reads, to a
> hurried reader, as though the approximation had been priced.
>
> **So NA-008's cost remains UNMEASURED**, it remains a **gamut-mapping**
> quantity rather than a rounding one, and **two entries in this register
> (NA-008 and NA-009) are now blocked on fixtures `tools/gen-profiles`
> has never written** — which makes that crate the highest-leverage
> unwritten code in the repository.

### NA-009 — the black-point **estimation** step is a labelled *subset* re-implementation of lcms2's, inside a documented silence (corpus **A42**). Cost **UNMEASURED**; nothing consumes it yet

*(Filed 2026-08-11 at the Pass 4b filing by `icc-librarian`, **from a
reading of code the dispatch did not mention**. `crates/iccce-cmm/src/bpc.rs`
is in the working tree with **4 `#[test]` declarations** and is declared
in `iccce-cmm/src/lib.rs` — **verified**. Registered now because
`ARCHITECTURE.md` invariant 3 requires an approximation to be named the
moment it exists, not the moment a Pass measures it.)*

| Field | Value |
|---|---|
| **The approximation** | BPC needs the source's and destination's **black points**, and **no published document defines black-point *estimation*** (`bkpt` is untrustworthy — the corpus's own cross-verified finding). `bpc.rs` implements a **subset** of lcms2's: for a **v4 profile at the perceptual intent**, the fixed perceptual black (see NA-010); **otherwise**, the media-relative transform of device black — *"the escape route lcms2 itself takes for v4 matrix/TRC profiles"*. **lcms2's fuller estimation (a thresholded Lab ridge search) is NOT reproduced**, and the module doc says so and says why: *"its thresholds are unattributed even in its own source"* — corpus **A42, UNVERIFIED** *(verified — `bpc.rs`'s module doc read, and A42's status read in `icc__ref__ambiguity_register.md`)* |
| **What is NOT an approximation here, and the distinction matters** | The **scaling map** is sourced: ICC.1:2022 **6.3.4.3**'s `Xp = Xt·(1−Xb/Xi)+Xb`, which with the two constraints solves to `a = (D50−bd)/(D50−bs)`, `b = D50·(bd−bs)/(D50−bs)` — algebraically identical to lcms2's `ComputeBlackPointCompensation` and to Maria (2013)'s published derivation. **The corpus's own citation discipline is carried at the site**: 6.3.4.3 is cited **for the scaling map, not "for BPC"** — its actor and its known-vs-estimated black differ, and conflating them is the corpus's named **C1** failure mode |
| **★ A DL-014 point, raised as a question rather than a defect** | `bpc.rs` heads that map **"PRIMARY-SOURCED"**, while `icc__ref__bpc.md`'s `evidence:` line grades §2/§3 **`cross_verified_2src` (ICC.1:2022 by two engines)** — the same shape of extraction that `icc__type__lutAtoB_lutBtoA.md` grades **`primary_spec`**. The corpus's §2.2 does call 6.3.4.3 *"the primary specification"* in its prose. **Whether the two tier labels mean the same thing is `icc-spec-librarian`'s to say**; DL-014 requires the `evidence:` line to be read, and this row records that it was, and that it did not say `primary_spec` *(verified — both frontmatters read)* |
| **Cost** | **UNMEASURED — and, unlike NA-008, ALREADY OWED, because the path is reachable through the shipped binary.** `Chain::with_bpc()` is wired and `iccce transform` accepts **`--bpc`** *(verified — `transform.rs:154–388` and `iccce-cli/src/main.rs:31–39, 195, 223–226, 259–268` read; **an earlier draft of this row said the opposite, from a head-limited grep** — recorded in §7.7 rather than silently fixed)*. `TOLERANCES.md` §3.5 (Pass 5) is **blank**, which is now a **gap** rather than a correct absence: the code is reachable and nothing has measured it |
| **★ The refusals, which are the part worth copying** | `with_bpc()` refuses **by name** rather than estimating something plausible: `ChainError::BpcNotApplicable` at the **absolute** intent (*"BPC presupposes both whites already at D50 — Maria 2013's sourced exclusion"*) and `ChainError::BpcEstimationUnsupported` **outside the named subset** — *"notably v2 LUT sources, where lcms2 runs an unattributed Lab ridge search"*. **The unsourced case is a refusal, not a guess**, which is DL-020's clause 1 applied in the CMM |
| **★ A related POLICY difference, recorded here rather than minted as its own entry** | **iccce NEVER forces BPC**; it is *"an explicit caller act"*. The field doc states the reason at the site: lcms2 forces it for v4 perceptual/saturation on *"the authority of an unpublished reading (M2/DL-013, and its 'always' has no published corroboration)"*. **This is not an approximation — it is a policy difference from the oracle**, so it belongs beside NA-002 in kind rather than in a new NA row; and **it is already priced in one direction** by **NC-078** (3,137×10⁻² device, `K` at black 99,6094 % → 96,4721 %) and by **NC-020** (≈3,15 `L*`). **Every Pass 5 cross-check must account for it explicitly** or it will measure iccce's *policy* and call the result a tolerance |
| **What would measure it** | A Pass 5 differential against `transicc -b`, at which point rule 4 requires it. Note in advance what such a row can be: **there is no BPC conformance test with a fixed expected value** (the module doc says so, same standing as perceptual under corpus A27), so the grade is **agreement with lcms2** — an `implementation-cross-check`, never ground truth |
| **Where** | `crates/iccce-cmm/src/bpc.rs` — module doc §"The estimation step", `BpcScale::new` |
| **Revisit if** | A42 acquires a source; BPC is wired into `Chain`; or lcms2's estimation changes at a new pin (**re-run, not re-read**) |

> **★★ Dated note, 2026-08-11 (Pass 5 completion filing) — the cost is
> STILL UNMEASURED, and Pass 5 is why that is now a *stated* result
> rather than an omission.** The row above says the cost is *"owed,
> because the path is reachable"*. **The path was exercised and the cost
> still cannot be taken**, for a reason derived from both
> implementations' sources **before** anything ran (§3.12.3): everywhere
> iccce does BPC at all, **lcms2's estimator reduces to the same two
> values** — `XYZ (0,0,0)` on every matrix/TRC or gray side in reach
> (`trc(0) = 0` everywhere), and the **same A41 triple** on a v4 LUT
> side at perceptual. **So every Pass 5 row grades the map, the
> direction and the pipeline, and none of them discriminates the
> estimators.** The instrument that would is **a synthetic v4
> RGB-or-gray LUT fixture with a NON-ZERO device black**, which does not
> exist *(verified — `fixtures/synthetic/` enumerated: 38 `.icc`, one v4
> LUT, zero black)*. **Two things did move**, and neither is this cost:
> **(a)** the **scaling map** is now graded against ICC.1:2022 6.3.4.3
> and Maria (2013) at ~10⁻¹⁶ (**NC-084 … NC-086**), so the sourced half
> of this entry has measurements; **(b)** the **refusals** are graded
> (**NC-103**, **NC-104**), so *"refuses rather than guessing"* is
> asserted behaviour rather than a claim about code. **And the policy
> paragraph in the row above has been promoted out of it**: measured
> end-to-end at **3,137 348 `L*`** (**NC-100**) and filed as
> `ARCHITECTURE.md` **DL-022**. The paragraph stays here, unedited, as
> the record of where the decision was first written down. **One new
> neighbour to this approximation, which the row above could not have
> known**: lcms2 **drops BPC entirely** below an `IsEmptyLayer`
> discriminant of 0,002 (**NC-088**) — a difference in *applicability*,
> not in estimation, and iccce deliberately lacks it.

> **★★ Second dated note, 2026-08-12 (the estimator-discrimination
> filing) — the cost is MEASURED at last, on two arms, and the fixture
> the two notes above kept asking for turned out to answer a DIFFERENT
> question than the one it was requested for.**
>
> **The cost, stated with its scope** (§3.18, **NC-142**): against
> lcms2's estimator **reimplemented from `cmssamp.c` at pin `21c582a`**,
> iccce's ISO/CD 18619 4.2.5 estimate differs by **8,166 8×10⁻² ΔE76
> (100 % `L*`, chroma exactly 0)** on `USWebCoatedSWOP.icc`, and by
> **5,000 000 ΔE76 (100 % chroma, `ΔL*` exactly 0)** on
> `fixtures/synthetic/v4-rgb-mab-chromatic-black.icc`. **Both at
> media-relative, both with the black point validated in device units
> against `transicc` (NC-140, NC-141).** ★ **What survives to a
> user is much smaller than either**: **8,785×10⁻² ΔE76 / 2,464×10⁻³
> device** end to end on the CMYK arm (**NC-135**), because both blacks
> sit at or below that profile's gamut floor.
>
> **★★★ And the reason one number is `L*` and the other is chroma is the
> finding: lcms2 has TWO estimators at media-relative**, selected by the
> **destination's device class and colour space** (`cmssamp.c`
> L370–374) — ink + output class takes `BlackPointUsingPerceptualBlack`
> and **forces the chroma to zero**; everything else takes
> `BlackPointAsDarkerColorant` and **keeps it**. `ARCHITECTURE.md`
> **DL-027**.
>
> **Three things this note must not be read as saying.** **(1)** It is
> **not ground truth** — one implementation reproduced from source,
> against one committee-draft procedure. **(2)** It is **not the
> perceptual arm**: there, both implementations return the A41 constant
> **without reading the profile**, so *no fixture can discriminate them*
> — the instrument this register asked for twice **cannot exist**, and
> what the new fixture makes measurable instead is **how wrong the
> constant is** (`L* ≈ 3,1` against that device's real black of
> `L* 20`), which is **owed, not made**. **(3)** ★ **The attribution of
> the `swop` number is OPEN**: it is entirely the 4.2.5.4 short-circuit's
> return value, where **iccce returns `outRamp[first]` and lcms2 returns
> `InitialLab`**, and **which of the two ISO specifies is a question
> dispatched to `icc-spec-librarian` and not yet answered** (§3.18.6).
> **If ISO names lcms2's, this row's cost is iccce's DEFECT, not its
> approximation**, and this register entry changes shape rather than
> gaining a number.
>
> **What else moved:** the estimator is now **reachable from the shipped
> binary** (**NC-157**) — it had **no caller at all** until commit
> `c268261`, which is why this row could say *"nothing consumes it yet"*
> for two filings and be right.

> **★★★ THIRD dated note, 2026-08-12 (later the same day) — item (3)
> above is ANSWERED, and this entry CHANGES SHAPE exactly as it said it
> would.** ISO/CD 18619 **4.2.5.4** specifies **`InitialLab`**. **iccce
> was non-conformant and lcms2 conformed**; corrected at commit
> **`fd34a44`** *(verified — `bpc.rs` read at the tip)*. See **§3.24**,
> **NC-164**, and `ARCHITECTURE.md` **DL-030**.
>
> **What this does to NA-009, precisely:**
>
> - **The `swop` arm's 8,167×10⁻² ΔE76 was never an approximation cost
>   at all. It was a DEFECT**, and it is re-attributed to iccce in
>   full. **A named approximation's register must not carry a bug as
>   though it were a priced departure** — that is the failure mode this
>   note exists to prevent, and the entry above pre-committed to
>   catching it.
> - **NA-009's actual cost is therefore UNMEASURED AGAIN**, and more
>   honestly so than before: the number that had been standing in for
>   it belonged to a different phenomenon. ★ **It should now be
>   expected to COLLAPSE toward zero on the `swop` arm** — both
>   implementations returning `InitialLab` from the same branch — **and
>   nobody has re-measured it.** Owed, §7.12.
> - **The synthetic RGB arm's 5,000 000 ΔE76 is UNTOUCHED by this**, and
>   the reason is DL-027: that arm diverges through the **estimator
>   selection** (`BlackPointAsDarkerColorant` keeping chroma), not
>   through the short-circuit's return value. **One correction did not
>   fix both arms, and it was never going to** — which is itself
>   evidence the two-arm design was right.
> - **The subset boundary moved slightly**: the corrected function
>   returns a full `Lab`, so a Gray or RGB LUT destination can now
>   receive the **chromatic** black ISO specifies (§3.24.2). **Cost
>   today zero** — 4.2.6 ignores `a`/`b` — **and it is still a change to
>   what this register describes.**

### NA-010 — the fixed v4 perceptual black follows **the implementations' triple**, not ICC.1 Table 16's printed decimals. A deviation from printed spec text whose cost is **corpus-derived, not measured here**

*(Filed 2026-08-11 at the Pass 4b filing by `icc-librarian`, from the
same reading. The **fourth kind** of entry in this register: NA-001
departs from printed **normative** text; NA-004 chooses inside a stated
**non-requirement**; NA-006 chooses inside a **silence**; **this one
follows two implementations against the specification's own printed
constants** — including **ICC's own reference CMM**.)*

| Field | Value |
|---|---|
| **The departure** | `bpc::PERCEPTUAL_BLACK` is **0.003 36 / 0.003 4731 / 0.002 87** *(verified — read)*. **ICC.1:2022 Table 16 prints 0,003 357 / 0,003 479 / 0,002 869**, which is also the exact decode of its printed 16-bit codes `006Eh`/`0072h`/`005Eh` |
| **Who agrees with whom** | lcms2 `cmsPERCEPTUAL_BLACK_{X,Y,Z}` at the pin and **iccDEV `icPerceptualRefBlack{X,Y,Z}` — ICC's own code** — are **byte-identical to each other** and neither matches the specification's printed values. The corpus calls this *"the first place in this corpus where lcms2 and an ICC-authored codebase agree **against** ICC.1's text"* (corpus **A41**) |
| **What the implementations' triple actually is** | A **channel-inconsistent hybrid**, per the corpus's derivation: `Y` is obtained from Table 16's **`L*`** (`8/255 × 100`, inverted through the Lab **linear** branch, giving 0,003 473 118… = 0,0034731 to 1,8×10⁻⁸), while `X` and `Z` are Table 16's **XYZ** decimals rounded to three significant figures. **Neither triple is neutral in Lab** |
| **Cost — and its exact status** | **0,005 3 in `L*` ≈ 0,037 4 ΔE76** between the two triples in the M2 configuration, **27× under §2's ⚠ provisional 1,0 anchor** — and **exactly zero on any 16-bit PCS path**, because both triples round to the same codes (110, 114, 94). Only a float32 PCS or a float pipeline can distinguish them. **Evidence class: `corpus-derived-bound`, DERIVED by the corpus in two passes. iccce has NOT measured it**, and nobody may restate it as an iccce measurement |
| **Why iccce deviates anyway** | The corpus's recommendation, quoted as a recommendation: *"use 0.00336 / 0.0034731 / 0.00287… it is what **both** the oracle and ICC's own reference CMM use, so a difftest that adopts the spec's printed decimals will show a systematic 0,037 ΔE floor at black that is a constant choice, not an error — exactly the kind of residue that gets chased."* **That is a testing-hygiene argument, not a conformance one, and it must not be restated as conformance** |
| **Where** | `crates/iccce-cmm/src/bpc.rs::PERCEPTUAL_BLACK`, whose doc comment cites A41 by name *(verified — read)*; corpus `icc__ref__bpc.md` §3 |
| **Related** | **NA-009** (the estimation step this constant feeds); **NC-020**/**DL-013** and **NC-078** (lcms2's forced BPC, whose triple this is) |
| **Revisit if** | ICC issues an erratum on Table 16; a float32-PCS path is added, at which point the 0,037 ΔE76 becomes **observable and therefore owed as a measurement**; or iccce ever emits a profile carrying these values, where following implementations against printed text is a different decision from consuming them |

> **★★ Dated note, 2026-08-11 (Pass 5 completion filing) — THE COST IS
> NOW MEASURED, by an independent route, and it acquired a figure the
> corpus never computed.** The row above records the cost as
> **`corpus-derived-bound`, DERIVED by the corpus in two Python passes,
> and explicitly *not* an iccce measurement*. **NC-094** rebuilt the BPC
> map with **ICC.1 Table 16's printed decimals** and evaluated it on
> §B's grid — **Rust, a different pipeline, a fixture's stored bytes**:
>
> | | corpus (Python, two passes) | Pass 5 (Rust, through a fixture) |
> |---|---|---|
> | ΔL\* | 0,005 3 | **0,005 364** |
> | ΔE76 | 0,037 437 | **0,037 416** |
> | ΔE2000 | *(never computed)* | **0,050 201** |
>
> **Both corpus figures corroborated to 2×10⁻⁵ ΔE76.** What this changes
> in the row above: its **Cost** field's *"iccce has NOT measured it, and
> nobody may restate it as an iccce measurement"* is **superseded** — it
> has now been measured in a pipeline, and **NC-094 is the row to cite**,
> at class `derived-expectation`, **REPORTED, NOT GRADED**. **What it
> does NOT change:** the corpus's derivation stands and was not wrong;
> the *"exactly zero on any 16-bit PCS path"* clause stands (both triples
> still encode to codes 110/114/94); and **the reason for the deviation
> is still testing hygiene, not conformance.**
>
> **★ And a warning the corpus's framing does not carry.** At **0,050
> ΔE2000** the choice of printed digits is **the same order as §3.12's
> entire agreement budget** (5×10⁻²). On a **float** path this constant
> is *not* negligible against the measurement noise — a difftest that
> adopted the specification's printed decimals would show a residue of
> exactly that size, permanently, with no defect anywhere. That is the
> complement of *"invisible at 16-bit"*, not a contradiction of it.
>
> **★ A second, independent confirmation of the same constant**, from
> the other end of the Pass: **NC-098**'s closed form is built on the A41
> triple's `L* = 3,137 238`, and **the wrong triple's signature there is
> `ΔK ≈ 5,4×10⁻⁵` — 11× that row's 5×10⁻⁶ bound.** So NC-098 is also an
> **A41 discriminator**: had iccce used Table 16's decimals, that row
> would have failed rather than passed at 9,5×10⁻⁸.

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

### 5.1 Dated correction to §5, 2026-08-11 (Pass 2 / difftest filing)

**The bullets above are left exactly as written** — they were true of
Pass 1 and this document does not edit a claim to make an old statement
look like a new one. Two clauses in the lcms2 bullet are now **partly
superseded**, and precisely which halves matter:

- *"There is still **no Rust difftest harness**"* — **superseded.** One
  exists (`tools/difftest/`, commit `bfd6b1e` *(reported)*): a standalone
  crate deliberately outside the workspace, zero dependencies, whose
  `Tolerance` type cannot be built without a `why` string and whose
  `Intent` enum admits only the four ICC intents so no result from it can
  be described as conformance to something ICC.1 does not define.
  *(verified — read in `tools/difftest/src/lib.rs` and `src/main.rs`.)*
- *"There is not one `implementation-cross-check` row in this ledger"* —
  **still true, and deliberately so.** §3.6's three rows are
  **`oracle-behaviour-at-pin`**, a class added today: they measure the
  oracle, with iccce absent. An `implementation-cross-check` row requires
  iccce on one side of the comparison, which requires a transform, which
  is **Pass 3**. **`iccce` has still never been compared to anything.**
- *"No cross-check against lcms2 exists anywhere in `iccce-color`"* —
  **still true**, unchanged.

**Also still true and worth restating** now that a harness exists: the
harness has **exactly one registered check** and it compares lcms2 to
lcms2 (NC-021). A green run of it is not coverage of anything, and its
runner deliberately exits **3 ("nothing ran")** rather than 0 when every
check skips.

### 5.2 Dated status of §5 and §5.1, 2026-08-11 (Pass 3 filing)

**Neither list is edited.** What Pass 3 changes, and — more
importantly — what it does not:

- **★ *"iccce has still never been compared to anything"* is STILL
  TRUE.** A transform now exists, which is the precondition, but **no
  comparison has been run and no `implementation-cross-check` row exists
  in this ledger.** §6's row predicting that Pass 3 *"is the moment the
  ledger can gain its first"* — *can*, and has not. The measurement is
  `icc-conformance`'s, dispatched in parallel with this filing, and
  **whether it landed is unverified here** (§3.7.0).
- **The transform's *interface* to the oracle now exists**, which is
  new: `iccce transform` reads triples on stdin and prints them at 6
  decimals with no banner, *"the interface `tools/difftest` diffs
  against transicc."* *(verified — read.)* An interface is not a
  comparison.
- **"No claim about sRGB constants"** — **still true of the claims, and
  now needing one clarification.** `iccce-color` still contains no sRGB
  constants; NC-029 uses the corpus's sRGB *shape* as parameters to a
  **round-trip fixture**, and the test says in its own doc comment that
  this is **not** a claim about sRGB. Do not let a grep for "sRGB" in
  the test suite become a claim.
- **"No claim that these tests pass on Linux"** — unchanged. Nothing has
  run on Linux, by anyone, ever.
- **New, and specific to this Pass: two of Pass 3's fourteen tests skip
  silently** when the system profile is absent (NC-031, NC-032). On such
  a machine the suite is still green and those two rows assert nothing.
  That is the same hazard as the difftest harness's skip-to-exit-3
  behaviour, without the exit code — **`cargo test` cannot distinguish
  "passed" from "did not run"**, and only these two rows record it.

### 5.3 ★★ Dated retirement, 2026-08-11 (Pass 3 closure filing) — *"iccce has never been compared to anything"* is **no longer true**

**Nothing in §5, §5.1 or §5.2 is edited.** Each was true on the day it
was written, and the whole value of this section is that a reader can see
the sentence stand across four filings and then fall.

**The sentence falls today.** §5's bullet *"There is not one
`implementation-cross-check` row in this ledger"* — carried unchanged
through the Pass 2 filing (§5.1, *"still true, and deliberately so"*) and
the Pass 3 core filing (§5.2, *"★ STILL TRUE"*) — is **retired as of
2026-08-11**, by **§3.8**: rows **NC-034, NC-035, NC-036, NC-037** and
**NC-040** are `implementation-cross-check`, and **NC-041** is a
cross-check of a deliberately modified model. `iccce` has been compared
to lcms2.

**★ Exactly what replaced it, because a retirement is the easiest place
in this document for a claim to grow:**

- **One profile pair, one intent, one direction, 133 points, one
  platform, one lcms2 pin.** The full scope box is at the head of §3.8
  and **must travel with any quotation of these rows.**
- **The strongest single sentence anyone is entitled to say** is
  `tools/difftest/README.md` §13.8's coverage statement, quoted verbatim
  in `ROADMAP.md`'s completion record. Anything shorter than it is
  probably an over-claim.
- **It is not ground truth and does not become ground truth by being
  the first of its kind.** Two implementations agreeing is evidence they
  read a clause the same way (rule 7, §1) — and here **both draw on the
  same single-source sRGB/D65 corpus lineage**, which is the
  shared-misreading case at its strongest, not its weakest.
- **`iccce-color` is still un-cross-checked.** §5's bullet *"No
  cross-check against lcms2 exists anywhere in `iccce-color`"* remains
  **true**: the harness *uses* `iccce_color::delta_e_2000` as its ruler
  (validated against Sharma's 34 pairs, NC-001), but no row compares
  `iccce-color`'s own output to lcms2's.

**Two bullets of §5 that are NOT retired and are worth restating on the
day the neighbouring one falls:**

- **"No claim about sRGB constants."** Still true. Nothing in Pass 3
  reads the corpus's sRGB constants — the profiles supply their own
  colorants — and NC-029 still uses the sRGB *shape* only as an
  arithmetic fixture.
- **"No claim that these tests pass on Linux."** Still true. **Nothing
  has run on Linux, by anyone, ever**, and every §3.8 row additionally
  **skips** on any machine without the Windows colour directory — which
  includes CI.

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
| **The lcms2 pin moving off `21c582a`** *(added 2026-08-11)* | **NC-019, NC-020, NC-021 — all three, without exception.** Every §3.6 row is a statement about one build of one implementation at one commit. DL-001 already makes moving the pin a **licence** event; DL-012 and DL-013 make it a **behavioural** one, and these three rows must be **re-run, not re-read**. |
| A behavioural test of `ncl2` or B2A legacy-Lab decoding *(added 2026-08-11)* | **NC-019's coverage line** — it currently rests on a *source reading* for those two cases, and a measurement would either promote them or contradict them |
| lcms2 changing `_cmsLinkProfiles` or `cmsPERCEPTUAL_BLACK_*` *(added 2026-08-11)* | **NC-020**, and with it every Pass 4 / Pass 5 tolerance derived from it |
| **Pass 3 landing a transform** *(added 2026-08-11)* | Nothing here is invalidated — but it is the moment the ledger can gain its **first `implementation-cross-check` row**, and §5.1's "iccce has never been compared to anything" stops being true |
| *(status of the row above, 2026-08-11 Pass 3 filing)* | **The transform landed; the row did not.** *"Can gain"* was accurate and *"stops being true"* was premature — see **§5.2**. §5.1's sentence still stands |
| `eval_table` / `invert_table` / the F.1 tie-break *(added 2026-08-11)* | **NC-025, NC-026, NC-032**, and NC-032 in particular: its bound is derived from the table's own spacing and **cannot discriminate a one-sample indexing error** (DL-016) |
| The **system sRGB profile** on this machine changing, or being absent *(added 2026-08-11)* | **NC-031 and NC-032** — both read a file this project does not own and does not commit, and **both skip silently when it is absent**, taking the suite green with them |
| `illuminant.rs`'s **D50**, again *(added 2026-08-11)* | **NC-031** joins the D50 list — its expectation is the 4-figure triple, and its justification quotes D65's `Z` from **NC-018**, the weakest constant in the crate |
| The matrix/TRC path acquiring **any adaptation step**, or consulting `chad`/`wtpt` *(added 2026-08-11)* | **NA-005** (retire or narrow it) and **NA-002** (its unmeasured cost becomes owed *then*, not at Pass 3 — see the dated correction in §4) |
| The **parametric `pow` guard** changing, or ICC/lcms2 changing theirs *(added 2026-08-11)* | **NA-004**, and any future difftest row covering parametric curves |
| **Either Pass 3 profile changing on this machine, or being absent** *(added 2026-08-11, closure)* | **NC-034 … NC-042 — all nine.** They are not ours, not committed, and every one of them **skips silently** when the directory is absent, taking the run to exit 3 rather than 0. **NC-042 in particular is a fact about two specific files**, and NC-038's tolerance is derived from it |
| **The 133-point grid changing** *(added 2026-08-11, closure)* | **NC-034 … NC-041.** A silently changed grid silently changes the **scope** of every one of them — and NC-034's bound is **grid-dependent by construction** (its derivation is evaluated at the grid's darkest non-zero step), so a grid extended nearer black **re-derives** it. Pinned by five unit tests in `pass3::tests`, including one asserting the count |
| **The lcms2 pin moving off `21c582a`** *(extended 2026-08-11, closure)* | Now **NC-019, NC-020, NC-021 AND NC-034 … NC-037, NC-040, NC-041, NC-043.** Every cross-check row is a statement about one build of one implementation at one commit, and **NC-041's whole content is a statement about lcms2's internal arithmetic** at that commit. **Re-run, not re-read** |
| **`iccce transform`'s print precision, or its argument handling** *(added 2026-08-11, closure)* | **NC-034 … NC-039.** The 6-decimal device print sets the ≈1×10⁻⁴ ΔE00 floor that **NC-039's tolerance is ten times**; both sides of every row cross a subprocess boundary through it |
| **The CLI exposing intents beyond media-relative** *(added 2026-08-11, closure)* | **NA-007's cost becomes measurable and therefore owed**, and the absolute-intent path becomes reachable by a differential for the first time. Nothing is invalidated; a hole becomes fillable |
| **`clut.rs`'s interpolation, or tetrahedral being sourced and added** *(added 2026-08-11, closure)* | **NA-006** — sourcing tetrahedral makes its ~1 ΔE corpus bound **measurable**, at which point rule 4 requires it to be measured rather than cited |
| **`iccce_color::delta_e_2000`, or `SHARMA_34`** *(added 2026-08-11, closure)* | **NC-001 directly — and now, indirectly, every ΔE row in §3.8**, because the harness grades with it (DL-017 condition 2). A change to the ruler re-scales NC-036, NC-038, NC-039, NC-040 and NC-041 at once, and **NC-040 is the only row that would notice** |
| **Clause 6.4's float32 clipping rule being read against Annex F.8–F.16** *(added 2026-08-11, closure)* | **NC-043** — it is the pending verdict; and if lcms2's excursion turns out to be conforming, **NA-003/NA-004's framing of iccce's clamp as conformance narrows to "conforming and stricter"** |
| *(status of the row above, 2026-08-11 Pass 4 filing)* | **ANSWERED, and against the hypothesis.** 6.4 governs the **PCS**, not device values; 6.5's float32 permission is gated on `DToBx`/`BToDx` tags a matrix/TRC profile may not carry; **a conforming F.8–F.16 evaluation cannot exceed 1,0.** See the second dated note under **NA-003** and `TOLERANCES.md` §5.2. **NC-043's third field is superseded**; two hedges survive (**A39b** divergence-not-non-conformance, **A39c** the v2 half unsourced) |
| **Corpus ambiguity A4b being resolved** (v2 `wtpt` semantics) *(added 2026-08-11, Pass 4)* | **NC-053 and NC-054, and NA-007's whole framing.** One of the two implementations acquires a defect and the absolute-intent row becomes **graded** again. **DL-019** is the decision record for the interim posture |
| **`clut.rs`'s interpolation changing, or tetrahedral being added** *(restated 2026-08-11, Pass 4)* | **NA-006 and NC-050 — and NC-046/NC-047, whose emulation arm exists only to switch the current scheme off.** The earlier row above predicted that sourcing tetrahedral would make the cost measurable; **the cost was measured without it** (NC-050), and the comparison arm that matters is lcms2's **four-input hybrid**, not tetrahedral |
| **The lcms2 pin moving off `21c582a`** *(extended again 2026-08-11, Pass 4)* | Now additionally **NC-044 … NC-050 and NC-053 … NC-056.** **NC-050 and NC-056 are the sharp ones**: NC-056's whole content is a reading of `cmsintrp.c`, and NC-050's second arm is a **transcription** of it — a minor release that retunes the interpolator factory invalidates both **silently**, because the transcription would keep reproducing the *old* lcms2 perfectly. **Re-run, not re-read** |
| **Either Pass 4 profile changing on this machine, or being absent** *(added 2026-08-11, Pass 4)* | **NC-044 … NC-055 — all twelve.** Neither file is ours or committed; **every row skips** without the Windows colour directory. **NC-053/NC-054 in particular are facts about one destination profile's `wtpt`**, and NC-050 about one source profile's two CLUTs |
| **The 341-point grid changing** *(added 2026-08-11, Pass 4)* | **NC-044 … NC-051 and NC-055.** A silently changed grid silently changes the **scope** of each. **NC-044/NC-045 are the most fragile**: their whole content is that those 16 points are **exact CLUT nodes**, a property of the corner block that `corner_indices_really_are_corners` pins |
| **A `B2A` or `mAB `/`mBA ` differential being run** *(added 2026-08-11, Pass 4)* | **Nothing here is invalidated** — but §3.9's coverage box narrows, and the **`Lab8` codec and `lut8Type` evaluation acquire their first evidence of any kind.** Until then they are implemented and unmeasured |
| **`tools/gen-profiles` fixtures being used by a differential** *(added 2026-08-11, Pass 4)* | Nothing is invalidated; **the "every row skips off this machine" clause on §3.8 and §3.9 becomes removable**, and Pass 4's missing **ground-truth** row becomes buildable (an affine-CLUT `mft2`, where every interpolation scheme must agree exactly) |
| **The `mAB `/`mBA ` curve counts, or `decode_lut_ab`** *(added 2026-08-11, evaluation surface)* | **NC-057 directly**, and it fails **at the decode**, not at the tolerance — a reverted GP-001 fix makes the fixture's `B2A0` refuse. Also every future B2A row, since the counts sit upstream of all of them |
| **`fixtures/synthetic/v4-cmyk-mab-lab.icc`'s bytes, or the generator that writes them** *(added 2026-08-11)* | **NC-057.** The fixture is the *expectation's* other half — `transicc`'s 49.6117 % was recorded against **these** bytes. `gen-profiles verify` is what detects an edited fixture, and **nothing runs it automatically** |
| **The lcms2 pin moving off `21c582a`** *(extended again 2026-08-11, evaluation surface)* | Now additionally **NC-057**, whose expectation is one build's recorded output. **Re-run, not re-read** — and unlike the §3.9 rows this one is cheap to re-run, being a single conversion through a committed file |
| **`gray_trc.rs`'s scalar recovery, or the F.2 white multiplication** *(added 2026-08-11)* | **NC-060, NC-061 and NA-008.** NC-060 is the green-cast regression and would catch the `X = Z = t` failure at 36–175× its bound; **neither row can see NA-008's cost**, because both feed neutrals |
| **`ewgray22.icm` changing on this machine, or being absent** *(added 2026-08-11)* | **NC-060** — a category (c) file this project does not own, and the test **skips silently** when it is missing, taking the suite green with it |
| **`transform::Chain` acquiring a test that traverses `Gray` or `LutAb`** *(added 2026-08-11)* | Nothing is invalidated; **a wiring that is currently verified only to exist acquires evidence that it works**. As of this filing `transform.rs`'s two tests are both SWOP→sRGB and neither reaches either new model |
| **A gray transform being compared to lcms2** *(added 2026-08-11)* | Nothing is invalidated; **NA-008's cost becomes measurable, and rule 4 then requires it to be measured rather than carried** |
| *(status of the row above, 2026-08-11 Pass 4b filing)* | **The comparison ran and the row was WRONG about what it would do.** §C compares a gray **source**; NA-008 lives in the gray **destination** path. **NA-008 is still unmeasured**, and the row that replaces this one is the next one down |
| **A transform with a GRAY profile as its DESTINATION being compared to anything** *(added 2026-08-11, Pass 4b)* | **NA-008 — and only then.** The measurement wanted is non-neutral PCS input, `Y/Yn` vs `L*/100` vs lcms2, because on the neutral axis the cost is **exactly zero** and every existing gray row sits there |
| **`cmsio1.c`'s `_cmsReadOutputLUT`, or `ChangeInterpolationToTrilinear`** *(added 2026-08-11, Pass 4b)* | **NC-062 … NC-069 and NC-075 … NC-077**, and **NA-006's direction clause**. If lcms2 stops forcing trilinear for Lab-PCS LUTs, the B2A envelope stops being zero and every §A tolerance is derived from the wrong quantity. **NC-067 is the row that would show it**, and it is ungraded — so **nothing would fail; the numbers would just quietly mean something else** |
| **`lut_ab.rs`'s matrix-output clamp, or the corpus's transcription of 10.12.5/10.13.3** *(added 2026-08-11, Pass 4b)* | **NC-071, NC-076 and NC-077.** The clamp is **normative** (`shall be clipped`), and the ten overflow points are excluded from the first two and reported by the third. **A corpus retraction of that sentence would move a `shall` back into an open question** and NC-077 would return to being unadjudicable |
| **The `derived-expectation` derivations, or the corpus's 10.12/10.13 transcription** *(added 2026-08-11, Pass 4b)* | **NC-070 … NC-074 together.** This is the class's stated weakness made into a dependency row: the fixture's bytes and the expectation are read out of **the same corpus**, so a wrong transcription invalidates **both at once and they keep agreeing.** NC-072/NC-074 (lcms2, the third reading) are what would notice |
| **The lcms2 pin moving off `21c582a`** *(extended again 2026-08-11, Pass 4b)* | Now additionally **NC-063 … NC-069, NC-072, NC-074 … NC-078 and NC-079 … NC-083.** **NC-082 is the sharp one**: its content is a **transcription of `cmsReverseToneCurveEx(4096)`, and a retuned resampler would keep reproducing the *old* lcms2 perfectly** — the same silent-invalidation shape as NC-050/NC-056. **Re-run, not re-read** |
| **`fixtures/synthetic/v4-cmyk-mab-lab.icc`'s bytes** *(extended 2026-08-11, Pass 4b)* | Now **NC-057 and NC-070 … NC-077** — nine rows resting on one file that **`gen-profiles verify` is the only detector for, and nothing runs it automatically** |
| **`ewgray22.icm` or the system sRGB profile changing, or being absent** *(extended 2026-08-11, Pass 4b)* | Now additionally **NC-062 … NC-069** (§A needs both) **and NC-079 … NC-083** (§C needs both). **All of them skip silently**; only §B's four derived rows survive on a machine without the Windows colour directory |
| **BPC being wired into `Chain`, or `bpc.rs`'s estimation changing** *(added 2026-08-11, Pass 4b)* | **NA-009 and NA-010** — their costs become measurable and therefore owed; and **NC-020/NC-078** become the behaviour a Pass 5 cross-check is measured against, so both must be **re-run** at whatever pin Pass 5 uses |
| *(status of the row above, 2026-08-11 Pass 5 filing)* | **Half right, and the half it got wrong is the interesting one.** NA-010's cost **was** measurable and is now **measured** (**NC-094**). **NA-009's was NOT** — being reachable is not the same as being *discriminable*, and no scenario in reach lets the two estimators differ (§3.12.3). *"Wiring makes a cost come due"* needs a second clause: **and an instrument in which the two candidates can disagree** |
| **`bpc.rs`'s `BpcScale` algebra, or the corpus's transcription of 6.3.4.3** *(added 2026-08-11, Pass 5)* | **NC-084, NC-085, NC-086 — and NC-098, whose step 2 is NC-086's constraint used as a premise.** A corpus retraction of 6.3.4.3's equation would take the only clause-graded rows Pass 5 has with it, and would leave the map with **no source but lcms2** |
| **`icc__ref__bpc.md`'s `evidence:` tier for §2/§3** *(added 2026-08-11, Pass 5)* | **NC-084 and NC-086's CLASS.** At `primary_spec` they become **`normative-rule-conformance`** rows; at `cross_verified_2src` (its current value, verified) they stay **`derived-expectation`**. **A ledger class now turns on a corpus frontmatter line** — see §3.12.2 and the DL-014 audit |
| **`bpc::PERCEPTUAL_BLACK`, or ICC issuing an erratum on Table 16** *(added 2026-08-11, Pass 5)* | **NA-010, NC-094 and NC-098.** NC-098 is the sharp one: it is an **A41 discriminator** whose bound the wrong triple misses by 11×, so a change to the constant makes it **fail loudly** — the rare case where a wrong constant does *not* hide |
| **lcms2's `IsEmptyLayer` threshold, or `AddConversion`** *(added 2026-08-11, Pass 5)* | **NC-088 directly** — and, silently, **every row in §3.12**: if the threshold rises far enough to swallow the S2/S3 map's 0,015 342 discriminant, lcms2 stops doing BPC and **the agreement rows would compare iccce's BPC against no BPC at all**. NC-088 is **ungraded**, so nothing would fail |
| **The S2/S3 grids (128 CMYK, 213 RGB), or the scenario set** *(added 2026-08-11, Pass 5)* | **NC-089 … NC-102.** ★ **Nothing pins them**: `pass5.rs` carries **no `#[test]` declarations** *(verified)*, so unlike Pass 3's five grid tests and Pass 4's corner assertion, **a silently changed grid silently changes the scope of fourteen rows and no test notices** |
| **`fixtures/synthetic/v4-cmyk-mab-lab.icc`'s bytes** *(extended 2026-08-11, Pass 5)* | Now **NC-057, NC-070 … NC-077 and NC-089 … NC-102** — **twenty-three rows on one file** that `gen-profiles verify` is the only detector for, and nothing runs it automatically |
| **The lcms2 pin moving off `21c582a`** *(extended again 2026-08-11, Pass 5)* | Now additionally **NC-088 … NC-096 and NC-099 … NC-102**. **NC-088 is the sharp one this time**: its whole content is a **constant and a control-flow predicate read out of `cmscnvrt.c`**, and a retuned threshold would invalidate it **silently, by never firing**. **Re-run, not re-read** — and **NC-084 … NC-087 are the only Pass 5 rows the pin cannot touch**, because no implementation is in them |
| **A non-zero-black v4 LUT fixture being authored** *(added 2026-08-11, Pass 5)* | **Nothing is invalidated — a hole becomes fillable.** It is the only instrument that discriminates the two **ESTIMATORS**, and therefore the only thing that can make **NA-009's cost** measurable. Same shape as the GP-001 arc: a doubt the corpus cannot discharge, discharged by bytes this project authors (**DL-020**) |
| **iccce ever forcing BPC, or lcms2 ceasing to** *(added 2026-08-11, Pass 5)* | **NC-100 and DL-022.** The row is a measurement of a **policy**; changing either side's policy retires it rather than moving it. **And every unasked-arm comparison in this ledger** (NC-020, NC-078, NC-093, NC-095) is a statement about the same policy pair |
| **`iccce transform --bpc`'s refusal WORDING** *(added 2026-08-11, Pass 5)* | **NC-103 and NC-104.** Their needle is the **exact wording**, deliberately — a paraphrase would let an ICC-absolute row pass on an estimation-subset refusal. **A message reword is a test-breaking change**, and that is the intended design |
| **★ THE MACHINE** *(added 2026-08-12, Pass 6)* | **NC-105, NC-106, NC-107 — all three, entirely.** A different CPU, a different allocator, a debug build, a thermally throttled run, or another process on the box **retires them**. They are not "approximately still true" on other hardware; they are **statements about one execution** and the `machine-timing` class exists to say so. **NC-107's 14.4× is the most fragile**, being a ratio of two things that scale differently |
| **★ The grid density (17 points/axis), or `CompiledTransform::new`'s sampling** *(added 2026-08-12, Pass 6)* | **NC-108 directly** — the off-node error is a property of the spacing, and `h²` means a 9-point grid would roughly quadruple it. **NC-105 and NC-106 too, in opposite directions**: a denser grid costs build time (NC-106) and buys accuracy, and **a Pass that reported only NC-105 could make itself faster by making NC-108 worse.** That trade is exactly DL-018's shape and NC-109 is what prices it |
| **★★ The sensitivity control's FIXTURE** *(added 2026-08-12, Pass 6)* | **NC-108's entire meaning, via NC-109.** ★ **This is the sharpest row in this table** and it is new in kind: NC-109 ran on **sRGB → AdobeRGB**, and the *previous* fixture (sRGB → sRGB) made the control **identically null** while still passing as code. **Anyone "simplifying" that fixture back to a single profile silently converts NC-108 from a measurement into a number** — and **nothing fails**, because `identical_at_nodes_by_construction` (NC-110) would still pass and the control's own band check would be exercised on noise. **DL-025** |
| **★ The sRGB TRC's breakpoint at 0.04045, or the probe set `[0.2, 0.9]`** *(added 2026-08-12, Pass 6)* | **NC-109.** The band `[2, 8]` is an `h²` prediction and holds **only in the smooth region**. Moving a probe across the kink returns `h¹` (measured: ratio 1.44) and the control **fails while nothing is wrong** — which is a maintenance hazard, not a bug, and is why both the region and the reason are in the test's doc comment |
| **★ `fixtures/synthetic/v2-ncl2-named.icc`'s bytes** *(added 2026-08-12, Pass 7)* | **NC-111 and NC-112.** The file is `gen-profiles` output and **`gen-profiles verify` is the only detector for a change to it, and nothing runs it automatically** — the same exposure §6 already records for `v4-cmyk-mab-lab.icc`, now on a second file |
| **★ `Chain::pcs_to_destination`, and the fact that `convert` shares it** *(added 2026-08-12, Pass 7)* | **NC-111 — and, in the other direction, every end-to-end row in §3.8 … §3.13.** The de-duplication is the Pass's central decision: **a change to the destination half now moves the spot path and the ordinary path together, which is the point.** ★ **What would break it silently is re-introducing a private destination arm for named colours** — the code would work, every test would pass, and the guarantee that a spot cannot drift from the rest of the CMM would be gone with nothing to notice |
| **★ The system sRGB and SWOP profiles in `C:\Windows\System32\spool\drivers\color\`** *(added 2026-08-12, Passes 6 and 7)* | **NC-105 … NC-109 and NC-111.** Neither profile is committed (LEGAL §3), so **every one of these rows SKIPS on a machine without them** — and a skip is not a failure. Combined with "no Linux run, ever", **six of this section's eight rows are unreproducible off this one machine** |

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

### 7.1 Status of §7, re-checked 2026-08-11 at the Pass 2 / difftest filing

The list above is **not edited**; this is its dated status.

| Item | Status now |
|---|---|
| 1 — a commit hash for §2 | **Discharged.** §2 carries `7313c5b` for Pass 1, and §2.1 carries `b35a12e` / `bfd6b1e` for today's rows. All three hashes are **reported by the engineer, not verified** — this librarian has no shell and ran no git command. |
| 2 — observed residuals, not only asserted bounds | **Partly discharged, and only for the new rows.** NC-019 (2×10⁻⁵) and NC-020 (3×10⁻⁵) and NC-021 (`0.000000e0`) each carry an **observed** figure. **NC-001 and every Pass 1 identity still carry only the bound asserted** — a residual that grew from 10⁻¹² to 9×10⁻⁵ would still pass its gate and nothing would show it. Unchanged as owed work for Pass 1. |
| 3 — `TOLERANCES.md` §3.1 and §5 rows | **Discharged by `icc-conformance`.** Both are filled, dated 2026-08-11, with §4 carrying two "first filling, not a change" rows. *(verified — read this session. This librarian did not edit that file; it is not this agent's.)* Note §3.2 (Pass 2), §3.3–§3.6 remain blank, correctly. |
| 4 — a ground-truth row for chromatic adaptation | **Still owed. Still the largest evidential hole in Pass 1**, and nothing filed today touches it. |
| 5 — the corpus D50-chromaticity erratum | **Fixed.** `cie__ref__colorimetry_core.md` now derives **0.345703 / 0.358539** for the ICC 4-figure triple and carries an `errata:` line **C2** naming the change, with a post-mortem pointer. *(verified — grepped this session. §3.4's "still present at filing" line is left standing as the record of what was true then.)* |
| 6 — a Linux run | **Still owed, and nothing has changed.** No Linux build of lcms2, no Linux `cargo test`, no observed CI run — by anyone, ever, in this project. |

**Newly owed as of this filing:**

1. **`icc-conformance`** — two wrong cells in `legacy_lab_probe.rs`'s
   module-doc prediction table (NC-019's starred note). Prose only; the
   run is unaffected; not repaired here because the file is not this
   librarian's.
2. **`icc-conformance`** — a **behavioural** test of `ncl2` and of B2A
   legacy-Lab decoding, so those two cases stop resting on a source
   reading (NC-019 coverage).
3. **Pass 4 / Pass 5** — the decision, and its own decision-log entry, on
   whether iccce **copies** lcms2's forced-BPC behaviour (DL-013). Until
   it is made, no perceptual/saturation tolerance against a v4 profile
   can be justified.
4. **`icc-spec-librarian`** — retract the corpus's claim that lcms2 keys
   Lab decoding on `cmsGetEncodedICCversion` (DL-012). A dispatch is
   **reported** in flight; **unverified whether it lands.**
5. **`icc-spec-librarian`** — **DL-002's successor entry is still
   unfiled.** §5 now runs to **DL-013** and several entries and doc
   comments cite ICC.1:2022 clause numbers. *(verified — `ARCHITECTURE.md`
   §5 read in full this session.)* The condition has been materially met
   since the ingest; the entry has not been written.

### 7.2 Status of §7 and §7.1, re-checked 2026-08-11 at the Pass 2 batch 2 filing

Neither list above is edited. This is their dated status, plus what
batch 2 newly owes.

| Item | Status now |
|---|---|
| §7.1 item 4 — a ground-truth row for chromatic adaptation | **Still owed, and now on a clock.** Unchanged in substance, but Pass 3 is next and it adapts, so **NA-002's unmeasured cost stops being permitted** the moment a transform lands (§4's own wording: unmeasured *"only while the entry is new"*). See `ROADMAP.md`'s Pass 3 annotation. |
| §7.1 item 6 — a Linux run | **Still owed. Nothing has changed.** No Linux build, no Linux `cargo test`, no observed CI run — by anyone, ever, in this project. |
| §7 item 2 / §7.1 item 2 — observed residuals for Pass 1's rows | **Still owed, unchanged.** NC-001 and every Pass 1 identity still carry only the bound asserted. |
| **§7.1 newly-owed 1** — the two wrong cells in `legacy_lab_probe.rs`'s module-doc prediction table | **DISCHARGED by `icc-conformance`.** The table now prints P3 general `L* = 50.0008` and P4 general `a* = 126.0078`, matching this librarian's recomputation and `difftest/README.md` §12.1, and the file carries a dated correction note naming what was wrong, who found it and why no verdict moved: *"the wrong cells sat on the REJECTED hypothesis and run-time predictions are computed, not read from this table."* *(**verified** — `tools/difftest/src/bin/legacy_lab_probe.rs` lines 66–78 read this session.)* NC-019's starred note is left standing as the record of what was true when it was written. |
| **§7.1 newly-owed 4** — the corpus retraction of the lcms2 version-keying claim | **LANDED — and verified rather than assumed.** `icc__ref__v2_v4_divergence.md` now carries *"★ RETRACTED 2026-08-11 (C3) — there is NO divergence from lcms2 here. Do not cite this row as one"*, with a four-row table separating what was claimed from what was measured, and it records that at the pin `cmsLabEncoded2FloatV2` *"is not called anywhere inside lcms2"*. `ICC_Spec\index.md` carries the same retraction at the top level and files it as the corpus's **third self-defect, C3**, with a new evidence file `icc__ref__lcms2_measured_behaviour.md` (M1 the selector, M2 the BPC finding). *(**verified** — both files read this session.)* **This is the second time checking beat assuming**: the D50 erratum was recorded as outstanding across two filings and had in fact been fixed. |
| **§7.1 newly-owed 5** — DL-002's successor entry | **FILED, as `ARCHITECTURE.md` DL-014**, after being owed across three filings. ICC.1:2022 clause numbers may now be cited **where the corpus file carrying the clause is `primary_spec` for that specific fact**, and the citation **must name the corpus file**. The tier is **per-fact, not per-file** — eleven of the fifteen `primary_spec` files are only *partly* so, with split `evidence:` lines. The prohibition is unchanged for every unread document (ICC.1:2010, ICC.1:2001-04, ISO 13655, CIE 142 / 11664-6 / 15 / 159, IEC 61966-2-1, and "Adobe's document"). Filed by `icc-librarian` rather than `icc-spec-librarian` as DL-006 anticipated — a reassignment of the filing, not of the sourcing judgement, and DL-014 says so. |
| **§7.1 newly-owed 2** — behavioural tests of `ncl2` and B2A | **Still owed to `icc-conformance`, and cheaper now.** Batch 2 shipped the B2A-side decoder (`mBA `, and `mft2` in the output direction), so the fixture half of that test no longer needs writing from nothing. NC-019's coverage line still rests on a **source reading** for both cases. |
| **§7.1 newly-owed 3** — the Pass 4/5 decision on whether iccce copies lcms2's forced BPC | **Still owed and still undecided.** Nothing in batch 2 touches it. |

**Newly owed as of this filing:**

1. **A decision on Pass 2's done-when clause 2**, and it is a scope call,
   not a coding task: *do the in-test byte-authored fixtures satisfy "a
   synthetic corpus covers each tag type", or does the clause require
   `tools/gen-profiles/` and files in `fixtures/synthetic/`?* Stated
   exactly: **every implemented tag type has hand-authored synthetic
   byte fixtures inside the unit tests**, including hostile cases; and
   **`tools/gen-profiles/` does not exist** while `fixtures/synthetic/`
   holds only a `README.md` that says *"Nothing here yet: the generator
   does not exist."* *(verified — tree enumerated and README read.)* The
   in-test fixtures are **tag-level, not whole profiles**, so they
   cannot cover header/tag-table/tag-data interaction and cannot be used
   by a differential run, a fuzzer, or an external consumer. **This
   ledger does not decide it**; `ROADMAP.md`'s batch 2 block states both
   readings without recommending one.
2. **A per-tag-type breakdown of the sweep** (§2.2.1). Forty profiles
   parsed, but which of `mft1`/`mft2`/`mAB `/`mBA ` were actually present
   is unrecorded, so *"the LUT decoders survived real profiles"* is not
   established by it. A re-run that counts tag types would turn a
   robustness observation into a coverage statement — and it would also
   show whether this machine contains **any** profile exercising batch
   2's code at all.
3. **An audit of existing ICC.1:2022 citations against DL-014's terms.**
   DL-014 does not retroactively bless them: each is permitted only if
   it names its corpus file and the fact is `primary_spec` **in that
   file's split `evidence:` line**. `lut.rs` §Sourcing is the shape
   intended and satisfies it; **no sweep of the other citations has been
   done by anyone**, and doc comments in `iccce-color` and
   `iccce-profile` predate the terms.

### 7.3 Status of §7, §7.1 and §7.2, re-checked 2026-08-11 at the Pass 3 filing

No list above is edited. This is their dated status, plus what Pass 3
newly owes. **Every "still owed" line below was re-checked against the
live tree this session, not carried forward from the last filing** —
that rule has now caught three items that were quietly done.

| Item | Status now |
|---|---|
| §7 item 2 / §7.1 item 2 — **observed residuals, not only asserted bounds** | **Still owed, and Pass 3 makes it worse rather than better.** All twelve §3.7 rows carry **the bound asserted**, not the residual observed — including **NC-032**, where the residual is the whole point (the test comment says *"measured residuals are far below it"* and **no figure was carried in the dispatch**). Had NC-032's residual been on record, DL-016's counterfactual would be a measurement instead of a reconstruction. |
| §7.1 item 4 / §7.2 — **a ground-truth row for chromatic adaptation** | **Still owed — and NO LONGER on a clock.** §7.2 put it on one on the reasoning that Pass 3 adapts. **Pass 3 does not adapt** (§4's dated correction, NA-005). It is still the largest evidential hole in Pass 1; it is simply not Pass 3's debt. |
| §7.1 item 6 — **a Linux run** | **Still owed. Nothing has changed, by anyone, ever.** |
| §7.1 newly-owed 2 — **behavioural tests of `ncl2` and B2A** | **Still owed to `icc-conformance`.** Nothing in Pass 3 touches either; NC-019's coverage still rests on a source reading for both. |
| §7.1 newly-owed 3 — **the Pass 4/5 decision on copying lcms2's forced BPC** | **Still owed and still undecided.** Pass 3 implements **media-relative only** and refuses other intents by name, so the question has not yet been reached — which is a deferral, not a discharge. |
| §7.2 newly-owed 1 — **the Pass 2 done-when clause 2 scope decision** | **Still owed and still undecided.** `tools/gen-profiles/` still does not exist and `fixtures/synthetic/` still holds only its README *(verified — enumerated this session)*. **Pass 2 is therefore still IN PROGRESS while Pass 3 has landed**, which is worth saying plainly: the Passes are no longer completing in order. |
| §7.2 newly-owed 2 — **a per-tag-type breakdown of the sweep** | **Still owed.** Unchanged. |
| §7.2 newly-owed 3 — **an audit of existing ICC.1:2022 citations against DL-014** | **Partly discharged, for new code only.** §2.3.1 audits the **five** citation sites Pass 3 added: four compliant, one naming an ambiguity-register row instead of the file carrying the clause. **No sweep of the pre-existing citations in `iccce-color` and `iccce-profile` has been done by anyone.** |
| §7.1 item 3 — `TOLERANCES.md` §3.2 and §6 | **Owned by `icc-conformance`, not re-checked in detail this filing** beyond noting it is theirs. Pass 3's tolerances (§3.7) have **no twin rows there yet**, which is expected on the day they are filed here and becomes owed if it persists. |

**Newly owed as of this filing:**

1. **`icc-conformance` — the Pass 3 done-when numbers** (§3.7.0): the
   sRGB→AdobeRGB round-trip ΔE and the lcms2 tolerance, the latter
   justified **before** the run. Dispatched in parallel; **landing
   unverified here.** Until they exist, **Pass 3 is not done.**
2. **`icc-spec-librarian` — the ICC-absolute intent formula.** A **new
   named corpus gap**: the media-relative→absolute white-point
   adjustment is **not transcribed** in `ICC_Spec`, `iccce-cmm` refuses
   the intent rather than approximating it, and the module doc records
   that it *"will not be written from memory"* (rule 2). It is the one
   thing blocking absolute colorimetric, and it is a **sourcing** task,
   not an engineering one.
3. **`icc-engineer` — three prose defects at their sites**, reported not
   repaired because the files are not this librarian's: NC-032's
   *"~2×"* justification (it is ≈1.02× of the quantity named);
   NA-004's *"reported"* (nothing reports it); and
   `crates/iccce-cmm/src/lib.rs`'s §Status, which still reads **"Pass 0
   scaffold. Matrix/TRC transforms are Pass 3"** on a crate that now
   contains them *(verified — read)*.
4. **An observed residual for NC-032**, specifically. It is the cheapest
   single number in this ledger to obtain and it would convert DL-016's
   reconstruction into a measurement.
5. **Parametric inverses for types 1, 2 and 4**, and a decision on
   whether a sampled inverse is ever acceptable — it would be an
   approximation and would therefore need a measured cost (rule 4).

### 7.4 Status of §7, §7.1, §7.2 and §7.3, re-checked 2026-08-11 at the Pass 3 **closure** filing

No list above is edited. **Every line below was re-checked against the
live tree this session**, not carried forward — the rule that has now
caught **four** quietly-done items and **one** false carried prediction.

| Item | Status now |
|---|---|
| §7.3 newly-owed 1 — **the Pass 3 done-when numbers** | **★ DISCHARGED, and verified by looking rather than by trusting the dispatch that said so.** §3.8 carries them: **3.4762×10⁻³ ΔE2000** against lcms2 (tolerance 2×10⁻², **implementation-cross-check**) and **1.8788×10⁻²** round trip (tolerance 2.5×10⁻², **self-consistency**). `TOLERANCES.md` §3.3 carries the twin rows with full derivations *(verified — read; not edited, it is `icc-conformance`'s)*. |
| §7.3 newly-owed 2 — **the ICC-absolute intent formula** | **★ DISCHARGED — sourced AND implemented on the same day it was filed as a gap.** `ICC_Spec\icc\icc__s__rendering_intents.md` exists at `evidence: primary_spec` for all clause text and equations *(verified — frontmatter and the D.6/D.7 verbatim blocks read)*, and it carries **more than was asked**: the clause-6.2.3 backwards-prose defect, the stale-cross-reference audit, and the 9.2.36 consequence. **The prediction *"expected to be in clause 6.x or an Annex"* was correct — 6.3.2.2 and Annex D.6 — but it was a prediction when written and it is a fact now only because someone opened the document.** |
| §7.3 newly-owed 3 — **three prose defects at their sites** | **DISCHARGED, all three** *(verified — each read at its site)*: NC-032's justification now reads *"≈1.02× the table's input spacing (1/1023 = 9.775e-4) — i.e. roughly ONE spacing, which per DL-016 means this bound CANNOT [discriminate]"*, and **names the audit that corrected it**; NA-004's *"reported"* is now *"**silent** — `eval` returns a bare `f64` with no diagnostic channel"*; `iccce-cmm/src/lib.rs`'s §Status no longer says "Pass 0 scaffold". **★ But the §Status is stale AGAIN, in a new place**: it reads *"media-relative colorimetric only; the absolute intent awaits its sourced formula"* on a crate that now implements it, and its module list mentions neither `pcs_encoding` nor `lut_transform`. **Newly owed, below.** |
| §7.3 newly-owed 4 — **an observed residual for NC-032** | **Still owed, and now conspicuous.** §3.8 carries **observed maxima on every one of its ten rows**, so the ledger now has two classes of row sitting next to each other: the new ones state what was measured, the Pass 1 and Pass 3-core ones state only the gate. NC-032's residual remains the cheapest number in this ledger to obtain and would turn DL-016's counterfactual from a reconstruction into a measurement. |
| §7.3 newly-owed 5 — **parametric inverses for types 1, 2 and 4** | **DISCHARGED, analytically, and no sampled inverse was introduced** — so the "would need a measured cost" question never arose *(verified — `invert_parametric` read in full; the `InverseUnsupported` variant no longer exists)*. |
| §7 item 2 / §7.1 item 2 — **observed residuals for Pass 1's rows** | **Still owed, unchanged.** NC-001 and every Pass 1 identity still carry the **bound asserted** only. |
| §7.1 item 4 — **a ground-truth row for chromatic adaptation** | **Still owed. Still NOT on a clock** — and this filing re-checked *why*, rather than repeating last filing's answer: **absolute intent does not adapt either.** D.6/D.7 is a per-component diagonal scale, `chad` is explicitly not un-applied, and `iccce_color::adapt` is **still called by no transform in this project** *(verified — read)*. **This is the second consecutive filing at which the NA-002 prediction was re-tested against code instead of carried.** |
| §7.1 item 6 — **a Linux run** | **Still owed. Nothing has changed, by anyone, ever.** And §3.8 sharpens it: every differential row skips without the Windows colour directory, so **CI could not run them even if CI ran**. |
| §7.1 newly-owed 2 — **behavioural tests of `ncl2` and B2A** | **Still owed to `icc-conformance`**, and now **cheaper again**: `pcs_encoding.rs` gives the legacy/v4 decode a tested home, and `lut_transform.rs` (in the tree, uncommitted status unknown) evaluates `mft2` in the A2B direction. NC-019's coverage still rests on a **source reading** for both cases. |
| §7.1 newly-owed 3 — **the Pass 4/5 forced-BPC decision** | **Still owed and still undecided.** Pass 3 **avoided** the question rather than answering it: both profiles are v2.1, below lcms2's version gate, and the apparatus itself says *"escaping a trap by accident is not avoiding it."* |
| §7.2 newly-owed 1 — **the Pass 2 clause-2 scope decision** | **Still owed and still undecided.** `tools/gen-profiles/` still does not exist; `fixtures/synthetic/` still holds only its README *(verified — enumerated)*. **It now blocks something concrete**: without a synthetic pair, every Pass 3 differential row skips on any machine but this one. |
| §7.2 newly-owed 2 — **a per-tag-type breakdown of the sweep** | **Still owed.** Unchanged. |
| §7.2 newly-owed 3 / §2.3.1 — **the DL-014 citation audit of pre-existing code** | **Still owed for `iccce-color` and `iccce-profile`.** The **one non-compliant Pass 3 site is fixed** — `curve.rs`'s 10.6 and 10.18 quotes now name `icc__type__curve_parametric.md` at the quote *(verified)*. **The new code has not been audited by anybody**: `clut.rs`, `pcs_encoding.rs`, `lut_transform.rs` and `matrix_trc.rs`'s new intent block add citations to 10.10, 10.6, 6.3.4.2, Table 25, 8.10.2, 9.2.36, D.6/D.7, A16, A18, A20, A21, A22 and A4b. **Spot-reading suggests they follow the DL-014 shape** (each names its corpus file), **but "suggests" is not an audit and this filing did not perform one.** |

**Newly owed as of this filing:**

1. **`icc-engineer` — `iccce-cmm/src/lib.rs`'s §Status, stale again.** It
   says the absolute intent *"awaits its sourced formula"* on a crate
   that implements it, and its module list omits `pcs_encoding` and
   `lut_transform`. *(verified — read.)* **This is the second
   consecutive filing to report this file's §Status**, which is itself
   worth noticing: a status line in a doc comment goes stale at exactly
   the rate the crate moves.
2. **`icc-engineer` — `clut.rs`'s module doc says *"per rule 4 (named
   and measured)"* about an approximation that is named and **not**
   measured** (NA-006). One word, and it asserts a discharged obligation
   that is not discharged.
3. **`icc-conformance` / `icc-engineer` — resolve the `pass=8` vs
   `pass=7` discrepancy** (§2.4) by recording a run's **per-line
   output**, not a summary count. A count is not an inventory, and a
   summary line cannot say *which* checks ran.
4. **`icc-spec-librarian` — the NC-043 clamping question**, if it is not
   already answered: clause 6.4's integer-vs-float32 clipping rule read
   **together with** Annex F.8–F.16. `tools/difftest/README.md` §13.10
   records it as **not dispatched**; the dispatch reports it **was**;
   **the answer is not in the corpus**, which is the only fact of the
   three this librarian can check.
5. **`icc-conformance` — the four items `tools/difftest/README.md`
   §13.10 owes**: a fixture distinguishing clamp-before from
   clamp-after; the **reverse** direction (the one with a real gamut
   clip); a **v4** profile pair; and a synthetic pair so §13 stops
   skipping everywhere but this machine.
6. **A ground-truth row for the matrix/TRC path** — `TOLERANCES.md`
   §3.3.3's first blank row, and **the largest evidential hole in Pass
   3**. Everything in §3.8 is implementation-relative or
   self-referential. The dispatch to `icc-spec-librarian` for IEC
   61966-2-1's primaries **has not been made by anyone**.
7. **A DL-014 audit of the new Pass 3-closure / Pass 4-groundwork
   code** — see the table row above. It is a larger citation surface
   than Pass 3's core added, and nobody has swept it.

### 7.5 Status of §7 … §7.4, re-checked 2026-08-11 at the **Pass 4** filing

No list above is edited. **Every line below was re-checked against the
live tree or the live corpus this session**, not carried forward — the
rule that has now closed **six** quietly-done items and caught **two**
false carried claims, one of them in this filing's own dispatch.

| Item | Status now |
|---|---|
| §7.4 newly-owed 1 — **`iccce-cmm/src/lib.rs`'s §Status** | **★ STILL OWED, and stale in a NEW place for the THIRD consecutive filing.** The absolute-intent sentence was fixed and `pcs_encoding`/`lut_transform` were added; the replacement now reads *"(CMYK→RGB live; **B2A/lut8/mAB stages pending**)"* on a crate where `b3f4388` landed **B2A and lut8** — `lut_transform.rs`'s own module doc is headed *"stages 1+3"* and evaluates **both depths in both directions**. Only `mAB `/`mBA ` is pending *(verified — both files read)*. Three filings running is no longer a fact about one file: **a status line in a doc comment goes stale at exactly the rate the crate moves.** |
| §7.4 newly-owed 2 — **`clut.rs`'s *"named and measured"*** | **★ DISCHARGED — by the measurement arriving, not by the prose changing.** NA-006's cost is now measured (**NC-050**), so the sentence that asserted an undischarged obligation is true. Recorded as closed **by fact**, because "fixed" and "overtaken by events" are different histories. |
| §7.4 newly-owed 3 — **resolve `pass=8` vs `pass=7` with per-line output** | **PARTIALLY DISCHARGED, structurally, and this filing did the counting.** §3.9.8 reconciles the emitters to the reported `pass=36 skip=3`: 1 smoke + **7** Pass 3 + **28** Pass 4 graded, 3 skipped. §2.4's hypothesis is confirmed. **Still owed: an actual run recorded with per-line output** — the summary line still cannot say *which* checks ran, and the `pass=7` re-run is still unexplained. |
| §7.4 newly-owed 4 — **the NC-043 clamping question** | **★ DISCHARGED, and it went AGAINST the hypothesis it was asked to test.** `icc-spec-librarian` read 6.4 / 6.5 / 8.3.3 / 8.4.3 and **A39 is resolved**: 6.4 governs the PCS, 6.5's float32 permission is unreachable from a matrix/TRC model, and **a conforming F.8–F.16 evaluation cannot exceed 1,0.** The full correction is `TOLERANCES.md` §5.2 (`icc-conformance`'s, not edited here); its effect on this ledger is the second dated note under **NA-003**. **The hypothesis was built on a clause number written from memory in this very document.** |
| §7.4 newly-owed 5 — **`tools/difftest/README.md` §13.10's four items** | **Two moved, two did not.** *A synthetic pair*: **`tools/gen-profiles/` now exists** and `fixtures/synthetic/` holds **39 `.icc` files** *(verified — enumerated)*, but **no differential record reads any of them**, so §13 and §14 still skip off this machine. *A v4 pair*: **still owed** — Pass 4 proved DL-013's confound **unreachable** for its v2 pair rather than exercising the v4 case. *The reverse direction* and *a clamp-before/clamp-after fixture*: **unchanged, both still owed.** |
| §7.4 newly-owed 6 — **a ground-truth row for the matrix/TRC path** | **Still owed, and the hole is now bigger, not smaller.** **Pass 4 has no ground-truth row either** — every §3.9 record is a cross-check, a self-consistency check or a measurement of the oracle. The most tractable Pass 4 candidate is a **synthetic `mft2` whose CLUT stores an affine function**, where every interpolation scheme must agree exactly, and `gen-profiles` now makes it authorable. The dispatch to `icc-spec-librarian` for **IEC 61966-2-1** has still **not been made by anyone**. |
| §7.4 newly-owed 7 — **a DL-014 citation audit of the new code** | **Still owed, and the surface has grown again.** `transform.rs` and `lut_transform.rs` add citations to **10.10/10.11, 10.6, 6.3.4.2 NOTE 3, 8.10.2 a)–d), Tables 40/44, Tables 12/13**, and ambiguities **A10, A16, A21, A22, A27**. Spot-reading again suggests they follow the shape — each names its corpus file — and **suggesting is still not auditing.** Nobody has swept `iccce-color` or `iccce-profile` either. |
| §7.2 newly-owed 1 — **the Pass 2 clause-2 scope decision** | **★ The BLOCKER dissolved; the DECISION is still not recorded as taken.** A generator and 39 whole-profile fixtures exist *(verified)*, so *"in-test synthetics or files on disk?"* is no longer a question that gates anything. **But the operator was asked which reading the plan meant, and nothing in these documents records an answer** — and until a differential actually reads a fixture, the concrete consequence (every §3.8/§3.9 row skipping off this machine) is unchanged. |
| §7.1 newly-owed 2 — **behavioural tests of `ncl2` and B2A legacy-Lab decoding** | **Still owed, and the B2A half is now conspicuous**: `b3f4388` shipped a **B2A evaluator** and Pass 4 measured **nothing** in that direction. NC-019's coverage still rests on a **source reading** for both cases. |
| §7.1 newly-owed 3 — **the Pass 4/5 forced-BPC decision** | **Still owed and still undecided — but for the first time it is a decision about a *future* run rather than a live confound.** Pass 4 **proved** the gate unreachable for its pair from the **parsed version words, printed on every record**, so the two options DL-013 posed remain open for whenever a **v4** pair is introduced. |
| §7.3 newly-owed 4 — **an observed residual for NC-032** | **Still owed.** §3.9 carries observed values on every row, so the ledger now has three sections of measured rows sitting beside Pass 1's and Pass 3-core's asserted bounds. |
| §7 item 2 / §7.1 item 2 — **observed residuals for Pass 1's rows** | **Still owed, unchanged.** |
| §7.1 item 4 — **a ground-truth row for chromatic adaptation** | **Still owed. Still NOT on a clock**, re-tested against the code for the **third** consecutive filing rather than carried: `iccce-cmm` contains **no reference to `adapt` of any kind** *(verified — the whole crate grepped case-insensitively; the only hits are two prose comments)*. Pass 4 adapts nothing either; **NA-002's Bradford cost is still not due.** |
| §7.1 item 6 — **a Linux run** | **Still owed. Nothing has changed, by anyone, ever.** And it is worse than a gap: **every Pass 3 and Pass 4 record reads a category (c) system profile**, so CI could not run them even if CI ran, and no CI run has ever been observed. |

**Newly owed as of this filing:**

1. **★ `icc-spec-librarian` — corpus ambiguity A4b**, the v2 `wtpt`
   question. It now has **11.217 ΔE2000 of consequence attached to it**
   (NC-053) and it decides which of the two implementations acquires a
   defect. **Verified still UNVERIFIED in the corpus this session.**
2. **★ `icc-spec-librarian` — corpus rows M4 and M5**: lcms2's four-input
   CLUT hybrid, and its v2-display-class `wtpt` substitution.
   `icc__ref__lcms2_measured_behaviour.md` carries **M1, M2, M3 and no
   more** *(verified — enumerated)*. Until they land, **NC-056 and
   NC-053 are this project's only record of two readings of the oracle's
   source**, and a reading that lives in one place is a reading that can
   quietly become a paraphrase.
3. **★ `icc-conformance` — measure the B2A direction.** The evaluator
   exists; nothing has compared it to anything. It is where *"at every
   intent"* is actually completed, and it is the first thing that would
   exercise **`lut8Type` evaluation** and the **`Lab8` codec**.
4. **`icc-conformance` — an instrument check for the sRGB destination
   model.** Pass 3's record 7 bounds iccce's ΔE ruler on **Adobe RGB**;
   **Pass 4 inherited that bound rather than re-measuring it on the
   profile it actually used.** Every Pass 4 ΔE row is graded with a
   ruler validated on a different profile.
5. **`icc-engineer` — `cmd_transform`'s doc comment contradicts its own
   code**: *"Only media-relative colorimetric exists (Pass 3 scope); an
   `--intent` flag naming anything else is refused by name"*, directly
   above a `match` accepting `perceptual`, `saturation` and `absolute`
   *(verified — read)*. **Reported, not repaired.** A reader who trusts
   it concludes no differential can reach the absolute intent — which is
   how the 11 ΔE finding was, until this morning, impossible.
6. **`icc-conformance` — `tools/difftest/README.md` §14.7's record
   decomposition** (§3.9.8): 7/1/28/31, not 8/1/27/30. The total is
   right and both terms are wrong.
7. **A decision nobody has taken: whether iccce should implement
   lcms2's four-input geometry at all.** NC-056 makes it a real
   question rather than a default — matching lcms2 would mean adopting a
   scheme that is **not symmetric in the four inks**, which is a
   property, not a bug, and choosing it needs a stated reason.

### 7.6 Status of §7 … §7.5, re-checked 2026-08-11 at the **evaluation-surface** filing

No list above is edited. **Every line below was re-checked against the
live tree or the live corpus this session.** Two of them are corrections
to statements this ledger itself made four hours earlier.

| Item | Status now |
|---|---|
| §7.5 newly-owed 1 — **A4b** | **★ Still UNVERIFIED, and now *expensive to leave*.** The corpus's sixth pass put the **11,2 ΔE00 stake into the ambiguity register itself** and re-confirmed by full-text search that ICC.1:2022 contains no transitional clause. **Only ICC.1:2001-04 settles it**, and the corpus records the ICC **errata as unreachable by compliant means** *(verified — the register read)*. It is the **top operator item**, ahead of every other download |
| §7.5 newly-owed 2 — **corpus rows M4 and M5** | **★ DISCHARGED — both landed**, and M5 did more than transcribe. **The framing this project carried in three documents was wrong**: lcms2 does **not** "ignore" the stored `wtpt`; `_cmsReadCHAD` uses it under the **same guard** to synthesise a Bradford `chad`, so lcms2's v2-display model is **coherent** (`wtpt` = unadapted white, `chad` = synthesised, adapted white = D50). Also new: **DemoIccMAX reads `wtpt` as stored** — the two ICC-adjacent implementations **disagree with each other** and **iccce matches ICC's own code**; **M4 generalises to `EvalNInputs`** (linear in the first `N−3`, tetrahedral in the last 3, so hexachrome inherits the asymmetry); and **A4c is new and SILENT** — ICC.1 requires **no** colorant/`wtpt` self-consistency, discovered from the stock sRGB profile's own bytes (**colorants sum to D50, `wtpt` holds D65**). **A4c does not clear when A4b clears** |
| §7.5 newly-owed 3 — **measure the B2A direction** | **★ Barely moved, and the distance is worth stating.** There is now **one recorded cross-check point** (**NC-057**) through a **synthetic** `mBA `. There is **no differential**, no grid, no real file, and **`lut8Type` evaluation and the `Lab8` codec still have no evidence of any kind**. The one thing that changed is reachability: `Chain` now selects `mft1`/`mft2`/`mBA ` on the destination side, so the run is possible **through the shipped binary** |
| §7.5 newly-owed 4 — **an instrument check for the sRGB destination model** | **Still owed, unchanged.** Every Pass 4 ΔE row is still graded with a ruler validated on Adobe RGB |
| §7.5 newly-owed 5 — **`cmd_transform`'s doc comment** | **★ DISCHARGED** *(verified — read)*. It now reads *"All four intents are accepted (Pass 4)"* **and records its own history**: *"An earlier version of this comment said media-relative only and outlived the code by three commits."* |
| §7.5 newly-owed 6 — **§14.7's record decomposition** | **Still owed** — `tools/difftest/README.md` is `icc-conformance`'s and was not read again this session for this purpose |
| §7.5 newly-owed 7 — **whether iccce should adopt lcms2's four-input geometry** | **Still undecided**, and M4's consequence 5 enlarges it: the choice is not "match lcms2 on CMYK" but "adopt a family that is asymmetric in the first `N−3` inks, up to 15 channels" |
| §7.4 newly-owed 1 — **`iccce-cmm/src/lib.rs`'s §Status**, stale for three filings | **★ DISCHARGED, and fixed in the right way** *(verified — read)*. It now enumerates every module accurately **and carries a standing instruction**: *"this block has been stale twice before — if a module below contradicts it, trust the module."* A doc line that tells the reader how to survive its own staleness is a better fix than a doc line that is currently true |
| §7.2 newly-owed 1 — **the Pass 2 clause-2 scope decision** | **★ DISCHARGED — by the stronger reading being satisfied, not by an answer.** 38 whole profiles on disk, a generator with `verify`, a generated `MANIFEST.md`, and profile-level coverage of **every tag type the Pass's plan names** *(verified)*. `ROADMAP.md`'s new Pass 2 block records the judgement, its boundary, and the fact that **no operator answer exists and none is now needed** |
| §7.1 newly-owed 2 — **behavioural tests of `ncl2` and B2A legacy-Lab decoding** | **Half-moved.** The B2A half has **one point** (NC-057) — but note it exercises the **v4** encodings, not the **legacy** ones, so **NC-019's legacy-Lab coverage line is untouched**; `ncl2` is unchanged |
| §7.3 newly-owed 4 / §7 item 2 — **observed residuals** | **Still owed, and worse this filing**: §3.10's rows have **no reported outcome at all**, because the dispatch carried no gate report |
| §7.1 item 4 — **a ground-truth row for chromatic adaptation** | **Still owed. Still not on a clock**, re-tested against the code for the **fourth** consecutive filing: `iccce-cmm` still calls nothing in `adapt` *(verified — grepped)*. **NA-002 remains not due** |
| §7.1 item 6 — **a Linux run** | **Still owed. Nothing, by anyone, ever** — and the new tests add two more silent skips off this machine |

**Newly owed as of this filing:**

1. **★ `icc-conformance` — the B2A / `mAB ` / gray measurements**, all
   three now reachable through the shipped binary and all three with
   **zero or one** data points today. The gray one is the cheapest
   comparison available: `transicc` accepts every well-formed fixture.
2. **★ `icc-spec-librarian` — the per-type transcription of
   10.12.2/4/6 and 10.13.2/4/6** into
   `icc__type__lutAtoB_lutBtoA.md`, whose **blanket sentence is still
   there** *(verified)* and is the most likely origin of GP-001, **plus
   A23** (permitted element sets — quoted verbatim in
   `gen-profiles/README.md` §5) **and A25** (`mluc` record selection;
   the generator reports having re-read 10.15 for its own use). **Both
   still UNVERIFIED in the register** *(verified)*.
3. **`icc-engineer` — `transform.rs`'s §Scope paragraph**, stale one
   commit after the last stale doc block was fixed: it calls
   `mAB `/`mBA ` *"the remaining absentees"* in the file that wires them
   on both sides, and **omits grayTRC entirely** *(verified — read)*.
4. **`icc-conformance` — `tools/gen-profiles/README.md` §5's
   `Status: open`**, its §6.1 `B2A0 REFUSED` row, and §8's handover
   line, all describing a finding that is **fixed in the live source**.
   A reader of that file today concludes iccce cannot parse a real CMYK
   `B2A0`.
5. **A run report.** Four consecutive filings carried a
   `cargo test --workspace` count; this one carried none, so **five
   ledger rows exist with asserted bounds and no reported outcome.**
6. **NA-008's cost**, which is measurable the moment a gray differential
   exists, and which is a **gamut-mapping** quantity rather than a
   rounding one.
7. **A re-run of the Pass 2 machine sweep against a post-GP-001 build**,
   with per-tag-type counts — the sweep's *"40 of 40"* is a statement
   about a superseded parser.

### 7.7 Status of §7 … §7.6, re-checked 2026-08-11 at the **Pass 4b** filing

No list above is edited. **Every line below was re-checked against the
live tree or the live corpus this session.** This is the first status
block in which **more items were discharged than added** — and one of
them was discharged by a document, not by a measurement.

| Item | Status now |
|---|---|
| §7.6 newly-owed 1 — **the B2A / `mAB ` / gray measurements** | **★★ ALL THREE DISCHARGED.** 28 records, `pass=28 fail=0` *(reported)*; rows **NC-062 … NC-083**. The three holes the evaluation-surface filing named in one sentence are now three sections of §3.11. **What the discharge does not include** is stated in §3.11.8 and is not small: no saturation, no ICC-absolute, no real v4 profile, and **the gray comparison ran in the direction that does not reach NA-008** |
| §7.6 newly-owed 2 — **the per-type transcription of 10.12.2/4/6 and 10.13.2/4/6, plus A23 and A25** | **★★ DISCHARGED, and it did more than transcribe.** The corpus's **seventh** pass replaced the blanket sentence with **six verbatim clause sentences and an implementable per-type table**, retracted the old rule **verbatim** and filed it as spec-defect **C4**; **A23 RESOLVED** (the permitted element combinations enumerated), **A25 RESOLVED** (with **A40** split off as its genuinely silent residue), **A24 PARTLY RESOLVED** (CLOSED for `mBA `, PARTIAL for `mAB `). It also produced the **normative matrix-output clamp** that §3.11.5 turns on *(all verified — `icc__type__lutAtoB_lutBtoA.md` §§1, 2, 5, 8 and the ambiguity register read 2026-08-11)*. **DL-020's clause 5 is discharged for this instance** |
| §7.6 newly-owed 3 — **`transform.rs`'s §Scope paragraph** | **★ DISCHARGED** *(verified — read)*. It now names grayTRC on both sides and `mAB `/`mBA ` as present rather than *"the remaining absentees"* |
| §7.6 newly-owed 4 — **`gen-profiles/README.md` §5's `Status: open`** | **★ DISCHARGED** *(verified — §5 now reads **"Status: FIXED same day — commit `2e98cfd`"** with the per-type counts)*. A reader of that file no longer concludes iccce cannot parse a real CMYK `B2A0` |
| §7.6 newly-owed 5 — **a run report** | **★ Half-discharged, and the half matters.** `pass4b_report` **`pass=28 fail=0`** is reported, so §3.11's rows are results rather than assertions. **But no `cargo test --workspace` count, no `fmt`/`clippy` line and no per-line output came with this dispatch either** — so **NC-057 … NC-061 still have no reported outcome at all**, five filings on, and the re-run that would have given them one was not reported per-line |
| §7.6 newly-owed 6 — **NA-008's cost** | **★ Still owed, and now owed with a reason instead of a hope.** The gray differential ran **GRAY→RGB**; NA-008 lives in the gray **destination** path. See the dated note under NA-008 |
| §7.6 newly-owed 7 — **a re-run of the Pass 2 machine sweep** | **Still owed.** A 40-profile sweep *did* run in this Pass — but it **searched for `mAB `/`mBA ` tags** (finding zero) rather than recording parse outcomes per tag type, and **the build it ran against is not stated** *(verified — README §15.3.1 read)*. It is a coverage fact about the machine, not the re-run that was owed |
| §7.5 newly-owed 4 — **an instrument check for the sRGB destination model** | **★ Moved, and not as far as it looks.** README §15.4.2 records §C as answering it, and §C **is** the cleanest available measurement of **lcms2's** sRGB output model (the whole residual attributed 457×). **But Pass 3's record 7 was an instrument check on *iccce's ΔE ruler*, not on the destination model** — a different quantity — so **the ΔE-ruler bound is still the one measured on Adobe RGB and inherited**. Stated as this librarian's reading of two records, not as a correction to `icc-conformance`'s file |
| §7.5 newly-owed 1 / §7.6 — **A4b** | **Still UNVERIFIED** *(verified — the register read this session)*. Unchanged in substance, and now **load-bearing in a second place**: §3.11's coverage box excludes ICC-absolute in all three directions precisely because §14.6's posture is untouched |
| §7.6 — **§14.7's record decomposition** | **Not re-checked this session** (`tools/difftest/README.md` is `icc-conformance`'s and was read only at §15 for this filing). Carried forward unchanged rather than assumed fixed |
| §7.1 item 4 — **a ground-truth row for chromatic adaptation** | **Still owed. Still not due**, checked against the live source for the **fifth** consecutive filing: nothing in `iccce-cmm` calls `iccce_color::adapt` *(verified — grepped)*. **NA-002 remains not owed** |
| §7.1 item 6 — **a Linux run** | **Still owed. Nothing, by anyone, ever.** Pass 4b adds **more** silent skips off this machine, not fewer — though §B's four derived rows are **the first graded rows in this suite that do NOT need the Windows colour directory** |

**★ And something appeared in the tree that this dispatch did not
mention — the fifth consecutive filing at which that is true.**

**`crates/iccce-cmm/src/bpc.rs` (Pass 5) and
`crates/iccce-cmm/src/named_color.rs` (Pass 7) both exist, are declared
in `iccce-cmm/src/lib.rs`, and carry 4 and 2 `#[test]` declarations**
*(verified — read and counted)*. The corpus carries a matching
`icc__ref__bpc.md` with new ambiguity rows **A41/A42/A43** *(verified —
frontmatter and §§2–3 read)*. **Consequences, filed rather than
smoothed over:**

1. **The dispatch's *"next: Pass 5 BPC pending sourcing"* is wrong on
   live evidence.** The sourcing has **landed**, and a scaling map, a
   fixed perceptual black and an estimation subset are **already
   written**. What has *not* happened is wiring or measurement.
2. **Two new register entries were owed the moment that code existed**
   — **NA-009** (the estimation subset, A42) and **NA-010** (the
   perceptual-black constant, A41) — and they are filed above, by this
   librarian, from the code.
3. **★ BPC is NOT merely written — it is WIRED AND REACHABLE, and this
   librarian's first draft said the opposite.** `Chain::with_bpc()`
   exists, `Chain` carries an `Option<BpcScale>` and per-side black
   estimation keyed on the major versions, and **`iccce transform`
   accepts `--bpc`** with the refusal path printed to stderr and
   exit 1 *(verified — `transform.rs:154–388` and
   `iccce-cli/src/main.rs:31–39, 195, 223–226, 259–268` read)*. The
   draft's *"referenced by nothing outside its own file"* came from a
   **head-limited grep**, which returned the first N matches and not the
   file's whole story. **A truncated search is not an inventory** — the
   same rule as *a count is not an inventory*, in a new disguise, and it
   is recorded rather than quietly fixed.
4. **Two Pass 5 facts follow from that, and both are load-bearing.**
   **(a)** **NA-009's and NA-010's costs are now OWED, not merely
   registered** — the code path is reachable through the shipped binary,
   which is the exact condition NA-007's dated note says makes a cost
   come due. **(b)** **iccce NEVER forces BPC**, deliberately: the field
   doc says lcms2 forces it for v4 perceptual/saturation *"on the
   authority of an unpublished reading (M2/DL-013…)"* and iccce *"makes
   it an explicit caller act, which is itself a recorded policy
   difference from the oracle."* **That difference will appear in every
   Pass 5 comparison**, and **NC-078 has already priced one direction of
   it** (3,137×10⁻² device at black).
5. **`NamedColors`, by contrast, really is unreachable** — referenced by
   nothing outside `named_color.rs` *(verified — the whole tree grepped
   for `named_color|NamedColors` with **no result limit**)*.
   `TOLERANCES.md` §3.5 (Pass 5) is **blank**, which is still correct:
   wiring is not measurement.
6. **★ `iccce-cmm/src/lib.rs`'s §Status is STALE AGAIN — the fourth
   time.** It reads *"Still to come: **BPC (Pass 5)**, compiled
   transforms (Pass 6)"* and omits [`bpc`] from its module list, in a
   crate that declares `pub mod bpc;` and wires it into `Chain`
   *(verified — `lib.rs:32–55` read)*. **Reported, not repaired** — the
   file is the engineer's. Note what saved it: the same block carries
   the standing instruction *"this block has been stale twice before —
   if a module below contradicts it, **trust the module**"*, which is
   why a reader following it would not have been misled. **A doc line
   that tells you how to survive its own staleness keeps working while
   being wrong**, which is the strongest available argument for that
   fix.
4. **`named_color.rs` moves §7.1's `ncl2` item without discharging it.**
   The consumer now exists and decodes through
   `pcs_encoding::LabEncoding::Legacy`, citing **10.17 verbatim** (*"this
   tag uses the legacy 16-bit PCSLAB encoding … not the 16-bit PCSLAB
   encoding that is defined in 6.3.4.2"*) and Table 66, corpus **A26
   RESOLVED** *(verified — read)*. **NC-019's coverage line still rests
   on a source reading**: nothing has compared an `ncl2` decode to
   another implementation, and the module doc's own note that legacy
   `L* > 100` is *"shall not"* in 10.10 but *"should not"* in 10.17
   (spec defect §4) is a **validator** question nobody has taken.

**Newly owed as of this filing:**

1. **★ `icc-conformance` — the two intents Pass 4b left out**:
   **saturation** in the B2A direction (`B2A2` is a third distinct
   table) and **ICC-absolute** through a **LUT destination**, which no
   run has ever exercised and which is where the D.6/D.7 composite is
   applied *before* the PCS is encoded.
2. **★ `icc-conformance` — a gray profile as DESTINATION**, over
   non-neutral PCS input. It is the only thing that measures **NA-008**,
   and §C proved it is not what a "gray differential" gives you by
   default.
3. **★ `icc-spec-librarian` — the clamp question, NARROWED to its second
   half** (§3.11.5): the *matrix-output* half is answered by the
   corpus's own seventh pass; what remains is whether the **final `B`
   curves' output** must be clipped to the encodable PCS range, and
   whether 10.18's domain binds the *evaluator* or describes the stored
   samples.
4. **`icc-spec-librarian` — two corpus rows**: the **M2 correction**
   (forced BPC is keyed by the **destination** version) and a new row
   for the **trilinear override** in `_cmsReadOutputLUT`, which sits
   beside M4 as *"same file, opposite direction, opposite answer."*
5. **`icc-conformance` — README §15.5's build-commit line** (§2.7), and
   **the M3 out-of-gamut excursion count**, which §A's 48 saturated-hue
   Lab points could have produced and **was not recorded on this run**.
6. **`icc-librarian` — the DL-014 citation audit**, whose surface grew
   again: `bpc.rs` (6.3.4.3, Tables 14/15/16, A41/A42) and
   `named_color.rs` (10.17, Table 66, A26) on top of `lut_ab.rs` and
   `gray_trc.rs`. **Spot-reading is not auditing**, and `iccce-color` /
   `iccce-profile` have still never been swept.

### 7.8 Status of §7 … §7.7, re-checked 2026-08-11 at the **Pass 5 completion** filing

No list above is edited. **Every line below was re-checked against the
live tree, `tools/`, the live corpus or `C:\personal_rag\` this
session.**

| Item | Status now |
|---|---|
| §7.7 newly-owed 1 — **the two intents Pass 4b left out** (saturation in B2A, ICC-absolute through a LUT destination) | **Still owed, both, and Pass 5 adds a THIRD saturation item that is not the same one.** Pass 5's saturation gap is that **iccce's estimation subset admits only perceptual for a LUT side**, so lcms2's forcing there **has no iccce half at all** — a *capability* gap, where Pass 4b's is a *run* gap. Do not let one tick the other off |
| §7.7 newly-owed 2 — **a gray profile as DESTINATION** (the only thing that measures NA-008) | **Still owed, and Pass 5 did not touch it.** §3.12's coverage box records the gray side of iccce's **own BPC subset** as unexercised too — a second, independent gray hole in the same crate |
| §7.7 newly-owed 3 — **the narrowed clamp question** (final `B` curves' output) | **Still owed** *(not re-verified this session; `icc-spec-librarian` was not dispatched from this filing)*. Carried forward unchanged rather than assumed moved |
| §7.7 newly-owed 4 — **two corpus rows** (the M2 destination-version correction, the trilinear override) | **Still owed — and the corpus now owes TWO MORE from Pass 5**: the **`IsEmptyLayer` 0,002 threshold** (§3.12.7; `ICC_Spec` §7.2's list came from `cmssamp.c` and this constant is in `cmscnvrt.c`) and **A41's ΔE2000 = 0,050 201** (§3.12.8; the corpus computed ΔE76 and ΔL* only, and ΔE2000 is the unit a perceptibility budget is stated in) |
| §7.7 newly-owed 5 — **README §15.5's build-commit line**, and the **M3 excursion count** | **Not re-checked this session** (§15 was not re-read; §16 was). Carried forward |
| §7.7 newly-owed 6 — **the DL-014 citation audit** | **★ Still owed, and it has grown teeth.** `bpc.rs` still heads 6.3.4.3 **"PRIMARY-SOURCED"** *(verified — read)* while `icc__ref__bpc.md`'s `evidence:` line still reads **`cross_verified_2src`** for §2/§3 *(verified — frontmatter read)*. **That mismatch now decides a ledger CLASS** (§3.12.2), not just a heading. `iccce-color` / `iccce-profile` have still **never** been swept |
| §7.6 / §7.7 — **a `cargo test --workspace` count** | **★ Still owed — and this is the filing where it stopped being a formality.** **Two commits this session carried false green claims in their messages** (§2.8). Independently checkable without a shell: **103 `#[test]` declarations across 18 files under `crates/`** *(verified — counted, no result limit; 102 at the previous filing)*. **NC-057 … NC-061 still have no reported outcome at all**, six filings on |
| §7.5 / §7.7 — **A4b** | **Still UNVERIFIED** *(not re-read this session — carried, not re-verified)*. Pass 5 does not touch it, but it is the reason **NC-104** can cite a published exclusion for the absolute intent while **NC-053/NC-054** remain unadjudicable |
| §7.1 item 4 — **a ground-truth row for chromatic adaptation** | **Still owed. Still not due**, for the **sixth** consecutive filing *(not re-grepped this session; the previous filing's grep stands and `iccce-cmm` gained only `bpc.rs`, which computes a diagonal scale, not an adaptation)*. **Stated as a reading, not a re-verification** |
| §7.1 item 6 — **a Linux run** | **Still owed. Nothing, by anyone, ever.** Pass 5 improves the *shape* slightly — **NC-084 … NC-088 and NC-104 need neither the Windows colour directory nor the oracle** — but nobody has run anything anywhere else |
| §7.7 item — **NA-009's and NA-010's costs** | **★ SPLIT: one discharged, one is now a stated impossibility.** **NA-010's cost is MEASURED** (NC-094, corroborating the corpus to 2×10⁻⁵ ΔE76 by an independent route, plus a new ΔE2000). **NA-009's is NOT, and cannot be until a non-zero-black v4 LUT fixture exists** — §3.12.3. See both dated notes in §4 |

**Newly owed as of this filing:**

1. **★ `tools/gen-profiles` — a synthetic v4 RGB-or-gray LUT fixture
   with a NON-ZERO device black.** The single highest-value item Pass 5
   produced: it is **the only instrument that can discriminate the two
   black-point estimators**, and therefore the only route to NA-009's
   cost. Every profile in reach has `trc(0) = 0`.
2. **★ `icc-conformance` — unit tests for `tools/difftest/src/pass5.rs`,
   which has none** *(verified — `tools/` grepped for `#[test]` with no
   result limit; `pass3.rs` 7, `pass4.rs` 7, `pass4b.rs` 8, **`pass5.rs`
   absent**)*. Fourteen rows' scope rests on two grids that **nothing
   pins**, where Pass 3 pinned its grid with five tests and Pass 4
   asserted its corners really are corners.
3. **★ `icc-conformance` — a reported runner result for §16.** The
   whole-suite `pass=90 fail=0 skip=3 error=0` is transcribed at the
   README's head, but **§16 states no `pass=`/`fail=` line of its own**,
   unlike §15's `pass=28 fail=0` *(verified)*. Pass 5's record count is
   currently **this librarian's subtraction**, not a report.
4. **★ `icc-spec-librarian` — the forcing-policy question** (§3.12.6):
   is BPC's *applicability* specified as a function of intent and
   version, or only its *black-point detection*? **Blocked on an
   operator browser download**; it is what would let **NC-100** move
   from *reported* to *graded*, in either direction.
5. **`icc-spec-librarian` — the two corpus rows above** (the
   `IsEmptyLayer` threshold, A41's ΔE2000).
6. **`icc-librarian` / whoever files next — a re-read of §15** at the
   next filing: §7.7's items 5 and 3 were carried forward here without
   re-verification, and **a carried item is a claim with a date on it.**

### 7.9 Status of §7 … §7.8, re-checked 2026-08-12 at the **Pass 6 + Pass 7** filing

No list above is edited. **This is the first status pass taken on a
different calendar day from the item it re-checks**, and the difference
shows: three items moved, and one of them moved because the operator
did something, not because an agent did.

| Item | Status now |
|---|---|
| §7.8 item 1 — **the non-zero-black v4 LUT fixture** | **Still owed, untouched.** Neither Pass 6 nor Pass 7 went near `tools/gen-profiles`. `fixtures/synthetic/` holds **38 `.icc`** *(verified — enumerated 2026-08-12; unchanged in count from the Pass 5 filing)*. **NA-009's cost is still unmeasurable**, and the ISO/CD 18619 work (`0378f76`) does **not** change that: **sourcing an estimator is not measuring one** |
| §7.8 item 2 — **unit tests for `pass5.rs`** | **Still owed** *(not re-verified this session — `tools/` was not re-grepped, by instruction: `icc-conformance` is working in `tools/` in parallel and this librarian did not read into it)*. **Carried, not re-verified — and that is a claim with a date on it** |
| §7.8 item 3 — **a reported runner result for §16** | **Still owed, and now with company.** ★ **Neither Pass 6 nor Pass 7 ran `tools/difftest` at all**, so §3.13's eight rows have **no runner outcome of any kind** and reconcile against nothing (§3.13.1) |
| §7.8 item 4 / §7.5 / §7.7 — **A4b, and the BPC forcing-policy question** | **★★ SPLIT, and one half is DISCHARGED by the operator.** **A4b is RESOLVED**: the corpus register (`revised: 2026-08-12`) records A1b, A2, **A4b**, A34 and A39c moving UNVERIFIED → RESOLVED on `ICC.1-2001-04.pdf`, leaving **one UNVERIFIED row in the whole register (A31)** *(verified — frontmatter and movement table read)*. It resolved **by the clause being silent on readers**, so **NC-053/NC-054 are no longer blocked on a missing document** — whether they move from *reported* to *graded* is now **`icc-conformance`'s judgement to make**, not a deferral. **The forcing-policy half is ALSO addressed**: `A42` moves **UNVERIFIED → PARTLY RESOLVED** on **ISO/CD 18619:2013**. **NC-100 is NOT thereby graded** — see the next row |
| *(consequence of the above)* — **NC-100's posture** | **★ Still REPORTED, NOT GRADED, and this librarian did not move it.** ISO/CD 18619 supplies the **estimation procedure**; DL-022/NC-100 turn on **applicability** — whether BPC's *enablement* is specified as a function of intent and version. **Nothing read this session says it is.** Moving NC-100 is a call for `icc-spec-librarian` and `icc-conformance` off the document itself, not an inference from A42's movement. **A class is not raised by an adjacent document arriving** |
| §7.8 item 5 / §7.7 item 4 — **corpus rows owed** (`IsEmptyLayer` 0,002; A41's ΔE2000; M2; the trilinear override) | **Not re-checked this session** — the corpus's BPC files were not re-read, only the ambiguity register's frontmatter and movement table. **Carried, not re-verified.** ★ Note `bpc.rs` now records the `IsEmptyLayer` `0.002` as one of **three constants with no home in either BPC document** *(verified — read)*, which is a *code-side* record, not the corpus row that was owed |
| §7.8 item 6 / §7.7 item 6 — **the DL-014 citation audit** | **★★ Still owed, and it now underwrites a PUBLISHED claim.** DL-024's third pre-publication check — *"spec quotation is short-with-citation per LEGAL §2.1"* — **is carried as REPORTED**, because no audit of the tree's quotations exists. The audit's surface has also grown again: `compiled.rs`, `named_color.rs`'s Table 66 / 10.17 citations and `bpc.rs`'s new **ISO/CD 18619:2013 clause 4.2.x** citations, on top of everything previously listed. `iccce-color` / `iccce-profile` have still **never** been swept. **And `bpc.rs` still heads 6.3.4.3 "PRIMARY-SOURCED" while the corpus's `evidence:` line reads `cross_verified_2src`** — the mismatch that decides NC-084's class *(not re-verified this session on the corpus side; the code side was read)* |
| §7.8 / §7.6 / §7.7 — **a `cargo test --workspace` count** | **★ Still owed, for the SEVENTH consecutive filing, and the project is now public.** Independently checkable without a shell: **116 `#[test]` declarations across 19 files under `crates/`** *(verified — counted, no result limit; 103 across 18 at the Pass 5 filing)*. **That is a count of declarations. It is not coverage and it is not a pass result.** **NC-057 … NC-061 still have no reported outcome at all** |
| §7.1 item 4 — **a ground-truth row for chromatic adaptation** | **Still owed. Still not due** *(not re-grepped; `iccce-cmm` gained `compiled.rs`, which interpolates a grid, and no adaptation code changed)*. **Stated as a reading, not a re-verification** |
| §7.1 item 6 — **a Linux run** | **Still owed. Nothing, by anyone, ever — and it changed character today.** The project is **public**. A reader who finds a throughput figure and a test suite will assume both travel; **six of §3.13's eight rows skip without this machine's colour directory**, and nobody has run anything on any other platform |
| §7.1 item 1 — **"a commit hash for §2"** | **★ DISCHARGED, in a form nobody anticipated.** The original item asked for hashes because *"a working tree is a weaker anchor."* Every provenance block has carried them since — as **reports**. As of 2026-08-12 **`.git/logs/HEAD` is a readable file** and corroborates all of them *(verified — read)*, which is a **stronger** anchor than the item asked for. **What is still not verified is any commit's CONTENTS**, and no amount of reflog reading will supply that |

**Newly owed as of this filing:**

1. **★ `icc-engineer` / `icc-conformance` — the raw `iccce bench`
   output block, twelve lines, filed somewhere.** §3.13.2: three of the
   four quoted figures were transcribed at three different precisions,
   and **14.4× does not reproduce from the other two** (the band
   consistent with them is `[14.24, 14.41]`). **The evidence for a whole
   Pass is twelve lines of text nobody kept.**
2. **★ `icc-conformance` — a ΔE2000 translation of NC-108's 0.003589
   device units.** The compiled path is a **named approximation** and
   rule 4 requires its cost in ΔE. `TOLERANCES.md` §3.6 states its two
   rows **in ΔE2000** and both are blank *(verified — read; its file,
   its call)*, so the unit mismatch is already recorded there.
3. **★ The sensitivity control's PASSING ratio.** §3.13.4: the two
   *failing* values are on record (0.94 and 1.44) and the passing one is
   not. **A control whose margin is unknown is a control nobody can tell
   is near its band edge.**
4. **★ `icc-conformance` — the spot-colour cross-check that was
   available and not taken**: resolve a spot into **its own profile's**
   device space and compare against the entry's stored `nDeviceCoords`.
   It is the only expectation on this path that iccce did not write, and
   it is cheap.
5. **A spot resolved into a LUT destination and into a gray
   destination** (both reachable today, both unexercised), and a
   **PCSXYZ `ncl2` fixture** so Table 66's second permitted encoding is
   not carried on a source reading.
6. **★ A re-read of the Pass 5 chroma-divergence prediction's OUTCOME.**
   `bpc.rs` pre-registers it (2–6 ΔE76 at input black, decaying to zero
   at white, relative colorimetric with a LUT destination) *(verified —
   read)*; `icc-conformance` is **reported** to be measuring it in
   parallel. **No result is recorded here in either direction.**
   **DL-011/DL-012 is the precedent: a predicted divergence has already
   once been measured ABSENT in this project**, so the row is written
   when there is a number and not before.
7. **★ `icc-engineer` — settle the commit count** (§2.9: dispatch 49,
   reflog 45) with one `git log --oneline | measure` on a shell, and
   **decide whether `edcb60e` needs any further correction** beyond the
   dated notes filed today.
8. **`icc-librarian` / whoever files next — `tools/` was not re-read
   this session** (items §7.8 2 and 5 above are carried), because
   `icc-conformance` was working there in parallel. **Two carried items
   is one more than last time.**

### 7.10 Status of §7 … §7.9, re-checked 2026-08-12 at the **Pass 4c** filing

No list above is edited. **This is the second status pass of the same
calendar day, and the first ever taken by a librarian with a working
shell** (§2.10), so three items that had been *"nobody has run the one
command"* are simply **answered**.

| Item | Status now |
|---|---|
| §7.9 item 7 / §2.9 — **the commit count (dispatch 49 vs reflog 45)** | **★★ SETTLED, and the file-derived reading was RIGHT.** `git rev-list --count f6203b8` = **45** — the tip at that filing; `git rev-list --count HEAD` = **51**; `git log --merges` = **0** *(all verified — run)*. **The dispatch's 49 was wrong**, 45 was correct when written, and the six commits since account for the difference exactly. **A number derived from a file beat a number carried in a dispatch**, which is §2.9's own thesis |
| §7.9 item 1 — **the raw twelve-line `iccce bench` output** | **Still owed, untouched.** Pass 4c ran the differential, not the bench. **§3.13.2's precision gap is unchanged**, and it is now the *only* thing standing between §3.13's rows and reproducibility |
| §7.9 item 2 — **a ΔE2000 translation of NC-108's 0.003589** | **Still owed** *(not re-checked on `TOLERANCES.md` §3.6 this session beyond confirming `COMPILED_DE` did not move)*. `icc-conformance`'s file, its call |
| §7.9 item 3 — **the sensitivity control's PASSING ratio** | **★ Partially answered by a different Pass, and the answer changes the question.** Pass 6's own control is still unrecorded — **but `TOLERANCES.md` §4 now records that its `h²` justification was FALSIFIED and re-derived**: the measured convergence order is **1.32**, stable to 1 % across three octaves, and row R6 now grades a **paired median** rather than a max-of-max *(verified — read)*. So the missing number is a ratio against a band that has since been re-derived. **Still owed, on the new band** |
| §7.9 item 4 — **the spot-colour cross-check against stored `nDeviceCoords`** | **Still owed, untouched.** Still the cheapest genuine cross-check available anywhere in the project |
| §7.9 item 5 — **a spot into a LUT/gray destination; a PCSXYZ `ncl2` fixture** | **Still owed.** ★ **But the gray half is now cheaper than it was**: §3.16.2 put a gray profile in the *destination* slot for the first time (as a scratch probe), so the path is demonstrably reachable from a `Chain` |
| §7.9 item 6 — **the Pass 5 chroma-divergence prediction's OUTCOME** | **★ IN FLIGHT, and deliberately NOT filed here.** `pass5c.rs` is untracked and its record count moved **8 → 16** between this session's two runs; `TOLERANCES.md` has gained a **§3.5.8** naming a **new finding** (lcms2 has *two* black-point estimators at media-relative, selected by the destination's device class and colour space) and **WITHDRAWING** row Q3's CONFIRMED verdict *(verified — read)*. **That is another agent's work, mid-flight. Whoever files it reports its own outcome**, and this ledger has not pre-empted it |
| §7.9 item 8 / §7.8 items 2 and 5 — **`tools/` was not re-read** | **★★ DISCHARGED for this filing, and it is what the whole filing turned on.** `tools/` **was** re-read: `pass4c.rs` read and its ten record ids verified, `pass6.rs`'s `COMPILED_DE` and `DEFAULT_GRID` read, `pass4c.rs`'s untracked status measured. **This is exactly the directory whose un-reading caused §3.14's error** — nine statements across three documents saying *"never run"* about finished, documented work. **`pass5.rs` still has no `#[test]` declarations** *(not re-verified — another agent is editing `pass5*.rs` right now, so the same protocol that caused the error applies again, and it is labelled as a carried claim rather than an observation)* |
| §7.8 item 1 — **the non-zero-black v4 LUT fixture** (NA-009) | **Still owed** — ★ **and it now has a companion.** §3.16.3: **NA-008's second arm is blocked on a PCSLAB gray fixture that has never been written either.** Two named approximations, two unwritten fixtures, one crate |
| §7.9 — **the DL-014 citation audit** | **★★ Still owed, and its surface grew again, in a way that is no longer hypothetical.** §3.15.7 found a **live** citation defect: *"Annex D, D.6/D.7"* names the **informative** annex when the normative statement is **6.3.2.2 Eq (4)–(6)**, and **the label is not edition-stable** — `ICC.1:2001-04` has no (D.7), and its (D.6) is a different equation. **Every `wtpt` discussion in this project concerns a v2 file**, so the ambiguity was live wherever the bare label appeared. **A sweep for the bare label is owed**, and this is the second consecutive filing at which a citation has been found naming the right words in the wrong place |
| §7.9 — **`cargo test --workspace`** | **★★ REPORTED FOR THE FIRST TIME IN SEVEN FILINGS: exit 0, 121 passed, 0 failed**, plus `fmt --check` and `clippy -D warnings` clean on the root workspace *(reported, gated on `$?`)*. **NC-057 … NC-061 have a reported outcome at last.** **State the scope honestly: that is a workspace-wide pass count, not per-row confirmation** — no row was individually named in the output, and *"121 passed"* is no more an inventory than *"116 declarations"* was |
| §7.9 — **a Linux run** | **Still owed. Nothing, by anyone, ever.** Unchanged and unchanging |
| §3.11's **build-commit discrepancy** (README §15.5 names `97ad9fa`) | **★★ SETTLED IN BOTH HALVES, by two different agents using two different methods.** **(a) The consequence is DISCHARGED by re-running**: `icc-conformance` rebuilt release from the **current working tree** and re-ran Pass 4b in full — **35 records, 0 fail**, and **every recorded number reproduces to every printed digit** (0.6117 → `6.117005e-1`; 1.012×10⁻⁴ → `1.012157e-4`; 5.2×10⁻⁵ → `5.200000e-5`; 9.686×10⁻⁵ → `9.686275e-5`; 2.121×10⁻⁷ → `2.121004e-7`; 1.87319×10⁻⁵ → `1.873190e-5`) *(reported)*. **The sentence "nobody may say these numbers were produced by the code that is in the tree today" is no longer true.** **(b) The hash question is ANSWERED here**: `97ad9fa` **is** commit #29 and `a0310c7` #32, 25 minutes apart, and `git merge-base --is-ancestor 97ad9fa a0310c7` confirms the ordering *(verified — run)*. **So the flag was correct — the named build commit does predate the clamp change — and the re-run proves the clamp change moved no Pass 4b number.** ★ **The method is worth keeping: the discrepancy was resolved by RE-RUNNING, not by reading.** A hash is a *proxy* for provenance; re-execution answers the question the proxy stood for. Where an item is carried because *"we cannot verify the build"*, **rebuild and re-measure is usually cheaper than authenticating the record** |
| `TOLERANCES.md` §3.4.4.5's **M3 out-of-gamut excursion count** | **★★ RETIRED, NOT SATISFIED — and replaced by a measurement four orders of magnitude larger.** §3.16.1: the owed form was **structurally incapable** of showing the effect (its destination is a CLUT, whose outputs *are* in-range table entries), so its count of **0/192** is a **null by construction**. The replacement A/B measures **up to 3.05 device units** on an analytic-inverse destination. **Do not tick the old item off as "measured, count 0"** |

**Newly owed as of this filing:**

1. **★★ `icc-engineer` — `dechk.obj` IS IN THE PUBLIC REPOSITORY, and this is the most urgent item on the list.** A 5 933-byte **MSVC COFF object file** at the repository **root**. **It is tracked, it was added by commit `aef7566`, and `aef7566` is an ancestor of `origin/master`** *(all verified — `git ls-files`, `git log --diff-filter=A`, `git merge-base --is-ancestor` run)*. **`.gitignore` has no `*.obj` or `*.o` rule** *(verified — read; it covers only `target/`, `*.icc`/`*.icm` with `fixtures/**` negations, and `tools/difftest/vendor/`)*. ★ **This is the SAME SHAPE as `edce48b`**, which §2.6 records as *"untracked in-progress `tools/gen-profiles` swept in by `d9e0b82`'s cwd-relative pathspec — a process slip"* — **same root directory, same mechanism, and this time with a remote attached and the push already done.** Owed: a `*.obj`/`*.o` rule, removal from the tree, **and a decision about history** (it is small and benign, but *"benign"* is a judgement the operator makes about a published artefact, not one an agent makes for him).
2. **★★ `icc-engineer` / the operator — EIGHT pushes to `origin/master` exist, not two.** DL-024 records two, at 06:51:17 and 06:54:50. `.git/logs/refs/remotes/origin/master` now holds **eight**, the last at **08:19:21 −04:00**, and **`origin/master` == `HEAD` == `95c04c1`** *(all verified — read and run)*. **Nothing in any document records a go-ahead for pushes three through eight**, and `CLAUDE.md` rule 9 plus DL-024 both say publishing is the operator's act and *"he said yes on the 12th" is not standing permission*. **This is recorded as an observation, not an accusation** — the pushes are attributed to `KenM76` in the reflog, the operator may well have run or authorised every one, and **no file records authorisation either way**. It needs confirming, not assuming.
3. **★ `icc-engineer` — commit the Pass 4c work.** `pass4c.rs` is **untracked**; the CLI help fix is **uncommitted**. **§3.14 and §3.15's sixteen rows are anchored to a working tree**, which §7.1 item 1 identified at the Pass 1 filing as the weaker anchor, for the reason that applies here exactly.
4. **★ `icc-conformance` — wire the three scratch probes into the harness or accept they will rot.** §3.16's M3 A/B, the gray-destination probe, and the Pass 4b re-run are **real measurements that nothing pins**. Each is one `Record` away from being a graded row with an NC number.
5. **★ `icc-engineer` — `cargo fmt --check` in `tools/difftest`: 109 diffs across 15 files** *(reported)*. **Rule 10's gate is stated workspace-wide and `tools/difftest` is deliberately not a workspace member**, so `--workspace` has never seen it. Either bring the harness under the gate explicitly or state in `CLAUDE.md` that it is exempt — **the current position is that a binding rule silently does not apply to a quarter of the code**.
6. **★ `tools/gen-profiles` — a PCSLAB gray fixture** (§3.16.3, NA-008's second arm) **and** the non-zero-black v4 LUT fixture (NA-009). **Two named approximations, one crate, neither fixture written.**
7. **A sweep for the bare *"D.6/D.7"* citation label** (§3.15.7), folded into the DL-014 audit.
8. **★ `icc-librarian` / whoever files next — the protocol fix the sweep exposed.** When a filing skips a directory because another agent holds it, **record WHICH directory was skipped and mark every dependent item `unverified-this-filing` rather than `owed`.** The two are different claims and **only one of them is safe to act on**: *"owed"* invites the next session to do the work, and **§3.14 is what that costs when the work is already done.** This filing skipped nothing, but it read `pass5*.rs` only glancingly while another agent was editing it — **so the same label applies to `pass5.rs`'s missing tests, and it is applied above.**

### 7.11 Status of §7 … §7.10, re-checked 2026-08-12 at the **estimator-discrimination** filing

No list above is edited. **This is the third status pass of the same
calendar day and the fourteenth filing overall.** ★ **It is also the
first taken WITHOUT a shell since §2.10 discovered one** — see §2.11 —
so every item that §7.10 settled by running a command is re-stated here
as **verified-then, unchecked-now**, per §7.10 item 8's own protocol.

| Item | Status now |
|---|---|
| §7.9 item 2 — **a ΔE2000 translation of NC-108's 0.003589 device units** | **★★ DISCHARGED.** `TOLERANCES.md` §3.6 and §3.19 carry the compiled path's cost **in ΔE2000**: **2,970 17×10⁻¹** at grid 17 and **1,677 3×10⁻¹** at the shipped default of 33, both against a **2,5×10⁻¹** line derived from Pass 4's measured iccce-vs-lcms2 figure. **Rule 4's requirement — a named approximation priced in ΔE — is met for the compiled path.** ★ **And the translation reversed the verdict**: 0,003589 device *"looks negligible and is not"* — at grid 17 it was **17 % above the entire implementation-to-implementation spread** |
| §7.9 item 3 — **the sensitivity control's PASSING ratio** | **★★ DISCHARGED, and the band it passes against is a different one.** **NC-149**: paired medians **2,69 · 2,47 · 2,51** over three halvings, band `[2, 8]`, violation **0,0**. The `h²` justification NC-109 carried is **falsified and re-derived** — measured order **1,32** — and **NC-150** keeps the falsified max-of-max estimator on file with its 4× wander |
| §7.9 item 1 — **the raw twelve-line `iccce bench` output** | **★ Still owed, and now worth MORE than when it was first asked for**: the shipped default has moved, so §3.13's figures describe a grid the binary no longer uses, and **nobody has filed a single line of the program's own output at either grid.** §3.13.2's precision gap is unchanged |
| §7.9 items 4–5 — **the spot-colour cross-check against stored `nDeviceCoords`; a spot into a LUT/gray destination; a PCSXYZ `ncl2` fixture** | **Still owed, untouched.** ★ **The `nDeviceCoords` comparison remains the cheapest genuine cross-check available anywhere in this project**, and it has now been owed for three filings |
| §7.9 item 6 — **the Pass 5 chroma-divergence prediction's OUTCOME** | **★★★ FILED — §3.17 and §3.18, NC-129 … NC-144.** And it resolved in a way no one asked for: **claim 2 FALSIFIED on the real profile; claims 1 and 3 FALSIFIED on one arm and CONFIRMED on the other**, because **lcms2 has two estimators and the destination's header picks between them** (**DL-027**). §7.10 said *"whoever files it reports its own outcome"* — **this is that filing**, and its own outcome is in the runner-outcome row below |
| §7.10 item 1 — **`dechk.obj` in the public repository** | **★ Still present at the repository root** *(verified — the tree enumerated)*, **and its CAUSE is now known**: it is the object file of the **C probe** that produced NC-153/NC-154's expected values. **Its tracked status and its presence in `origin/master` were verified with a shell at the last filing and could NOT be re-verified here.** Carried as **verified-then, unchecked-now**. Unchanged: `.gitignore` has no `*.obj`/`*.o` rule, and *"benign"* is the operator's judgement about a published artefact, not an agent's |
| §7.10 item 2 — **pushes without a recorded go-ahead** | **★ NINE now, not eight** *(verified — the reflog read)*, the last at **09:06:55 −04:00** carrying **`5cfee171`**. **Nothing in any document records authorisation for pushes three through nine.** Needs **confirming, not assuming** |
| §7.10 item 3 — **commit the Pass 4c work** | **★★ DISCHARGED — BY ACCIDENT, AND THE ACCIDENT IS A LARGER ITEM THAN THE DEBT.** The previous filing recorded, from `git show --stat`, that commit **`5cfee17`** *"difftest: the estimator discrimination"* (09:06:21, 23 files, +4 907) **swept in `pass4c.rs`, the CLI help fix, `TOLERANCES.md` — and `docs/NUMERIC_CLAIMS.md` MID-WRITE — and was pushed** *(reported by that filing, which ran the command; **not re-verified here**, no shell)*. ★ **Third instance of one mechanism in two days**: `edce48b` swept in in-progress `gen-profiles` (§2.6), `aef7566` swept in and **published** `dechk.obj`, and `5cfee17` swept in another agent's **unfinished document**. **Three times is not a slip; it is the default behaviour of the command being used**, and the cost is no longer untidiness — **it publishes work whose author has not finished checking it.** ★ **A consequence for anyone reading history: `5cfee17`'s message mentions neither Pass 4c nor this ledger, so `git log` is a misleading index of when they landed — use `git log -- <path>`** |
| §7.10 item 4 — **wire the three scratch probes into the harness** | **Still owed, untouched** *(not re-read; `tools/` was read here only at §17/§18/§19 of the README and at `TOLERANCES.md`)* |
| §7.10 item 5 — **`cargo fmt --check` fails in `tools/difftest` (109 diffs)** | **Carried, `unverified-this-filing`.** No shell. The structural point stands and is not a measurement: **rule 10's gate is stated workspace-wide and `tools/difftest` is deliberately not a workspace member**, so `--workspace` cannot see it |
| §7.10 item 6 — **the two unwritten fixtures** | **★★ ONE IS WRITTEN, and it answered a different question than it was asked.** `fixtures/synthetic/v4-rgb-mab-chromatic-black.icc` **exists** *(verified — the directory enumerated: **39** `.icc`, was 38)*, device black `Lab(20 · 4 · −3)`. **It does NOT discriminate the v4 PERCEPTUAL arm — nothing can** (§3.18.5) — it discriminates the **media-relative** arm by being **RGB**, i.e. not an ink space, and therefore taking lcms2's other branch. **The PCSLAB gray fixture (NA-008's second arm) is still unwritten** |
| §7.10 item 7 — **the bare "D.6/D.7" citation sweep** | **Still owed**, folded into the DL-014 audit |
| §7.10 item 8 — **the `unverified-this-filing` protocol** | **★ APPLIED TO ITSELF, first time.** Five items above carry it, for a reason the protocol did not anticipate: **not another agent holding a directory, but the absence of a shell in this session's grant.** *"Who could check it"* turns out to be as much a property of the session as *"who is editing it"* |
| §7.9 / §7.10 — **`cargo test --workspace`** | **★ NO OUTCOME ACCOMPANIED THIS DISPATCH.** The last on record is §2.10's **exit 0, 121 passed** at commit `95c04c1`; **one commit has landed since** (`5cfee17`). Checkable without a shell: **121 `#[test]` declarations across 19 files** *(verified — counted; 116 at §2.9)*. **The two 121s are different quantities and their agreement is a coincidence** |
| §7.9 / §7.10 — **a runner outcome for the differential suite** | **★★ STILL MISSING, AND IT IS THE SHARPEST GAP IN THIS FILING.** The last reported run was **`pass=140 fail=2`, both failures in `pass5c`**, on a **shape that no longer exists** — both rows have since been re-formulated (§2.11). **Sixteen `pass5c` records and eight `pass6` records below are filed from `TOLERANCES.md` and `README.md` §19, with no `pass=`/`fail=` line for the final shape reported to anyone** |
| §7.1 item 4 — **a ground-truth row for chromatic adaptation** | **Still owed. Still not due** *(not re-grepped; no adaptation code changed)* |
| §7.1 item 6 / §7.9 — **a Linux run** | **Still owed. Nothing, by anyone, ever.** ★ **CI is now REPORTED to have run and passed** (below), which is the closest this project has come — and **it is a report, not an observation** |
| **The DL-014 citation audit** | **★ Still owed, ninth filing.** ★ **A new, cheap surface**: `delta_e.rs` now cites **CIE 116-1995** and **BS 6923** — both **explicitly as UNSOURCED and paywalled**, which is the correct form, and the audit should confirm that no later document quietly upgrades them |
| **`published-ground-truth` for any transform** | **★★ UNCHANGED, NINTH consecutive filing, and the gap widened in appearance rather than in fact.** §3.20 adds two metrics that match another implementation to **ten decimals** and are **`impl_crosscheck`**, and §3.18 adds a **reimplementation** of lcms2. **`IEC 61966-2-1` is still the cheapest route and still nobody has dispatched for it** |

**Newly owed as of this filing:**

1. **★★★ `icc-spec-librarian` — ISO/CD 18619 4.2.5.4's short-circuit
   return value.** Does the standard specify **`outRamp[first]`** (what
   iccce returns) or the **`InitialLab`** behaviour lcms2 implements
   (`cmssamp.c` L536)? **Dispatched 2026-08-12.** **The entire `swop`
   arm's 8,167×10⁻² ΔE76 is that one line**, and **if ISO names lcms2's,
   iccce is wrong rather than divergent and the engineer changes the
   code** (§3.18.6). **Until it is answered, no document may describe
   this difference as lcms2 departing from the standard.**
2. **★★ A RUNNER OUTCOME for the final shape of `pass5c` and `pass6`** —
   one `pass=`/`fail=` line, and the `cargo test --workspace` exit code
   at the current tip. **Twenty-four records in §3.18 and §3.19 have
   none.**
3. **★ `icc-conformance` — the A41 constant's error, measured.** The
   instrument now exists: at perceptual on
   `v4-rgb-mab-chromatic-black.icc` both implementations use
   `L* ≈ 3,1` where the device's real black is **`L* 20`**. **Owed, not
   made** — and it is the one number the new fixture makes possible that
   nobody has taken.
4. **★ `icc-engineer` — the CI evidence.** §2.11 records the run as
   **reported**. `ROADMAP.md`'s Pass 0 record, and several filings after
   it, carry *"no CI run has been observed by this librarian"*; **that
   caveat is retired by a REPORT, which is a weaker retirement than it
   reads.** A workflow run URL or a pasted summary would make it an
   observation. ★ **And the report's scope is unstated**: CI builds
   **Linux and Windows** by design (`.github/workflows/ci.yml`, Pass 0),
   so *"CI passed"* may be **the first Linux execution in this project's
   history** — **or it may not be**, and nobody has said which. **Do not
   let it silently discharge the Linux debt.**
5. **★★ NOTHING — and the reason this item survives as a numbered entry
   is that it is a WRONG FINDING this filing caught in its own draft, an
   hour after writing it.** A draft of this section carried: *"the Pass
   4c entry's 'Filed this session' table says `NEXT_SESSION.md` was
   rewritten; **it was not**"* — sourced from a read of `NEXT_SESSION.md`
   taken **at the start of this session**, which showed the **Pass 6 +
   Pass 7** edition. **A re-read before filing shows the file is headed
   *"at the Pass 4 completion filing — the second of the second calendar
   day, and the thirteenth overall"*** *(verified — read)*. **The Pass 4c
   filing did rewrite it. The claim was false, and it was false because
   the read was OLD, not because the read was careless.**
   > **★★ The lesson, and it belongs to this librarian rather than to
   > anyone else: *"verify against live source"* has a hidden clause —
   > **live means AT THE MOMENT OF FILING.** Two other agents were
   > writing in `docs/` during this session (the Pass 4c filing was still
   > landing; the edit tool reported `NUMERIC_CLAIMS.md` and `ROADMAP.md`
   > as changed on disk between two reads, and `ROADMAP.md` grew by the
   > Pass 4c header block **while this session was open**). **In a
   > concurrent session an early read is a DISPATCH, not a source** — it
   > carries exactly the authority of somebody else's report about a file.
   > **Re-read anything a claim rests on, immediately before asserting
   > it.**
   **What is genuinely owed from this**: nothing to another agent. The
   incident is the entry.
6. **A fixture whose darkest colorant has chroma above 50**, which would
   turn lcms2's clamp/return asymmetry from **READ** into **RUN**.
   **Deliberately not built** (§3.18.5) — recorded so the decision is
   visible rather than forgotten.
7. **★ A `NUMERIC_CLAIMS.md` mirror is no longer owed** — §19.9 item 5
   asked for it and **§3.18 is it**. Recorded here so the harness's own
   owed-list can be closed by whoever next reads it.

### 7.12 Status of §7 … §7.11, re-checked 2026-08-12 at the **4.2.5.4 correction + `iccce-measure`** filing

No list above is edited. **This is the fourth status pass of the same
calendar day and the fifteenth filing overall.** ★ **It is the second
consecutive filing taken WITHOUT a shell**, so every item §7.10 settled
by running a command remains **verified-then, unchecked-now**, and
§7.11's own unchecked items are carried without being restated as
though they had been re-examined.

| Item | Status now |
|---|---|
| §7.11 newly-owed 1 — **ISO/CD 18619 4.2.5.4's short-circuit return value** | **★★★ DISCHARGED, AND IT WENT AGAINST US.** ISO specifies **`InitialLab`**; **iccce was non-conformant and lcms2 conformed**; corrected at **`fd34a44`** *(verified — `bpc.rs` read)*. **§3.24**, **NC-164**, **DL-030**, and a third dated note on **NA-009**. ★ **The prohibition it carried is discharged in the opposite direction to the one a careless reader would have assumed**: it was never available to say lcms2 departed from the standard, and now it is settled that **we** did |
| §7.11 newly-owed 2 — **a runner outcome for the final shape of `pass5c` and `pass6`, and a `cargo test --workspace` exit code** | **★★ DISCHARGED — and it arrived as THREE numbers from three runners, which is the finding.** `pass=142 fail=0 skip=3 error=0`; `cargo test --workspace` **129 passed, 0 failed, exit 0**; `tools/difftest` unit suite **36 passed**. **§3.22**, **NC-158 … NC-160**, **DL-031**. ★ **`skip=3` is reported and NOT enumerated** — see the new owed list |
| §7.9 item 1 — **the raw `iccce bench` output** | **★★ DISCHARGED, and it did more than close the item.** `docs/bench-2026-08-12.txt` carries the full twelve-line output *(verified — read)* **plus a variance note the item did not ask for**, which is what makes **§3.23** possible. ★ **The file predicted this filing**: *"quote a RANGE and the load condition, never a point figure"* |
| §7.9 items 4–5 — **the spot-colour cross-check against stored `nDeviceCoords`; a spot into a LUT/gray destination; a PCSXYZ `ncl2` fixture** | **Still owed, untouched, FOURTH filing.** ★ **The `nDeviceCoords` comparison remains the cheapest genuine cross-check available anywhere in this project** and nothing has been done about it on any of the four |
| §7.11 newly-owed 3 — **the A41 constant's error, measured** (`L* ≈ 3,1` against a real `L* 20`) | **Still owed, untouched.** The fixture exists; the measurement does not |
| §7.11 newly-owed 4 — **the CI evidence** (a run URL or pasted summary; and whether Linux was among the jobs) | **Still owed, untouched.** ★ **Three runner outcomes arrived at this filing and none of them is CI.** The Linux debt is undischarged and the caveat *"no CI run has been observed by this librarian"* still holds |
| §7.10 item 1 — **`dechk.obj` in the public repository** | **Carried, `unverified-this-filing`.** ★ **The repository root was NOT enumerated at this filing** — this librarian read named files only — so **not even its presence is re-confirmed here**, let alone its tracked status. Weaker than §7.11's carry, and said so rather than repeated |
| §7.10 item 2 / §7.11 — **pushes without a recorded go-ahead (nine at the last count)** | **Carried, `unverified-this-filing`.** **The push log was not read at this filing**, so the count is neither confirmed nor updated, and **nothing here evidences the current tip having been pushed at all**. Rule 9 and **DL-024** are unchanged: publishing is the operator's act |
| §7.10 item 3 — **commits sweeping in other agents' unfinished work** | **★ No new instance observed — and nothing was checked.** Commit *contents* have never been verified by this librarian in any filing. The **three** recorded instances stand |
| §7.10 item 4 — **wire the three scratch probes into the harness** | **Still owed, untouched** *(not re-read)* |
| §7.10 item 5 — **`cargo fmt --check` in `tools/difftest`** | **Carried, `unverified-this-filing`.** ★ **The structural point sharpens with NC-159**: `tools/difftest` now demonstrably has its own green unit suite (36), and rule 10's gate **still cannot see it**, because the crate is deliberately outside the workspace. **Two runners, one gate** |
| §7.10 item 6 — **the PCSLAB gray fixture (NA-008's second arm)** | **Still unwritten.** ★ **NA-008's cost remains the oldest UNMEASURED entry in §4** |
| §7.10 item 7 / **the DL-014 citation audit** | **★ Still owed, TENTH filing** — and it acquires a new, cheap surface at this one: **§3.24 cites ISO/CD 18619 4.2.5.4 verbatim**, and the audit should confirm the corpus now carries that paragraph at the tier the citation implies, since **§3.24.3 records that it did not before** |
| §7.1 item 4 — **a ground-truth row for chromatic adaptation** | **Still owed. Still not due** *(no adaptation code changed; not re-grepped)* |
| §7.1 item 6 / §7.9 — **a Linux run** | **Still owed. Nothing, by anyone, ever.** Unchanged by three green runners on Windows |
| **`published-ground-truth` for any transform** | **★★ UNCHANGED, TENTH consecutive filing.** ★ **And this filing is the sharpest illustration of the gap yet available**: a clause of a **committee draft** was the sole arbiter of a defect in shipped colour code, and it was right. **`IEC 61966-2-1` is still the cheapest route to a real ground-truth row and still nobody has dispatched for it** |

**Newly owed as of this filing:**

1. **★★ `icc-conformance` — RE-MEASURE the `swop` arm's black-point
   divergence on corrected code.** **NC-142's 8,166 8×10⁻² ΔE76 should
   now COLLAPSE**, both implementations returning `InitialLab` from the
   same branch. **Nobody has re-run it, and §3.24 deliberately does not
   assert the collapse.** ★ **Until it is measured, NA-009's cost is
   UNMEASURED — the number that stood in for it has been re-attributed
   to a defect.** This is the highest-value item on the list.
2. **★★ Reconcile the TWO NON-OVERLAPPING SPEEDUP RANGES.** `iccce
   bench` gives **12–23×** (NC-162); `TOLERANCES.md` §3.6.2 gives
   **28–32×** on the `tools/difftest/src/pass6.rs` apparatus. **The
   project holds two ranges that do not overlap and does not know
   why.** A hypothesis is offered in §3.23.4 (the two harnesses may time
   different work) and **is labelled a hypothesis**. **Until it is
   settled, no document may quote a single speedup figure.**
3. **★ Enumerate NC-160's `skip=3`.** Which three records skipped, and
   why. **A skip is the runner declining to grade, and it is invisible
   in `fail=0`** — the one place a green census can hide something.
4. **★ Sweep `README.md` for a single-figure throughput or speedup
   claim.** §3.23.2 checked `docs/` and **did not check `README.md`**,
   which is the **user-facing** surface and therefore the one where an
   unsupportable number does the most damage. *(Claim-bearing copy: it
   is a claim, and it gets verified.)*
5. **★ A `TOLERANCES.md` §5 row for the corrected NA-009**, and a
   `TOLERANCES.md` row anywhere for **NC-164**. `icc-conformance` owns
   that file; this ledger has recorded the facts and edited nothing.
6. **★★ A regression test for the 4.2.5.4 branch, graded against the
   clause.** **NC-164 is a behavioural row READ FROM SOURCE, not run** —
   the corrected line has no test asserting that the short-circuit
   returns `initial_lab` unchanged. ★ **The defect shipped once through
   exactly this gap**, and `iccce-cmm`'s 63 tests did not catch it.
7. **★ Nothing is owed for the manifest.** Recorded as a numbered item
   because the dispatch **asked for an owed correction that does not
   exist**: `Cargo.toml` already reads *"Five crates"* and lists all
   five *(verified — read)*. **The entry exists so nobody files the
   correction twice**, and as the fourth instance in this project's
   short history of **the dispatch and the tree disagreeing** (§2.12).

---

## 8. Related

- `docs/TOLERANCES.md` — the tolerance budget (`icc-conformance`).
- `docs/ARCHITECTURE.md` §5 — the decision log; **DL-004** (the
  perceptual anchor), **DL-005** (v2 legacy Lab tested by exact
  invariants), **DL-010** (NA-001), **DL-011** (legacy Lab keys off tag
  type), **DL-012** (the predicted lcms2 disagreement measured **absent**
  — NC-019), **DL-013** (lcms2 forces BPC on v4 perceptual/saturation —
  NC-020), **DL-014** *(added 2026-08-11)* — **the terms on which an
  ICC.1:2022 clause number may be cited**, which now govern every
  citation in this ledger too: name the corpus file, and check that the
  file's `evidence:` line is `primary_spec` **for the specific fact**,
  not merely somewhere in the file. **DL-015** *(added 2026-08-11)* —
  the parametric `pow` guard (NA-004), a choice **inside a stated
  non-requirement** rather than a deviation from normative text.
  **DL-016** *(added 2026-08-11)* — sampled tables are asserted by
  **exact values at the sample points**, with the arithmetic showing
  that NC-032's self-consistency bound would have passed with the bug
  NC-025 caught. **DL-017** *(added 2026-08-11, closure)* — the
  **harness may path-depend on iccce's crates**, direction and four
  conditions stated; it is why §3.8's ΔE rows can exist at all, and
  condition 2 is why NC-001 is load-bearing for every one of them.
  **DL-018** *(added 2026-08-11, closure)* — **an upper-bound gate on a
  deliberate cost must be paired with a prediction pin**, or deleting
  the requirement makes the gate greener (NC-038 and NC-039 are the
  worked pair). **DL-019** *(added 2026-08-11, Pass 4)* — when a
  disagreement's **mechanism is identified but its authority does not
  exist**, the raw comparison is **REPORTED, NOT GRADED** and the gate
  moves to the modelled quantity (NC-053/NC-054, and now **NC-077** and
  **NC-078**). **DL-020** *(added 2026-08-11, evaluation surface)* — a
  rule the corpus cannot supply is **refused by name, not guessed**, and
  the refusal is discharged by an **independently authored fixture that
  can fail** (GP-001; regression **NC-057**, and the whole-chain
  regression is now **NC-073**). **DL-021** *(added 2026-08-11,
  Pass 4b)* — **a measured implementation behaviour is a fact about the
  direction and path it was measured in, until it is measured in the
  others**; it governs **NA-006's direction clause**, **NC-067**,
  **NC-078** and the `Lab8`-vs-legacy split in §3.11.2. **DL-022**
  *(added 2026-08-11, Pass 5)* — **iccce NEVER forces BPC**; the policy
  behind **NC-100**, reported not graded. **DL-023** *(added
  2026-08-11, Pass 5)* — **state what the two implementations were free
  to disagree about, from their sources, before the run**; it governs
  §3.12's negative result and **predicted §3.13's null-by-construction
  trap by name**. **DL-024** *(added 2026-08-12)* — **the project is
  PUBLIC**: the push event, the file-level evidence for it, the three
  pre-publication checks (**two verified, one reported**), the
  commit-count discrepancy and the wrong hash it caught, and the four
  things publication does **not** authorise. It is the reason §2.9 is
  the first provenance block whose hashes are corroborated rather than
  reported. **DL-025** *(added 2026-08-12, Pass 6)* — **a sensitivity
  control is only as good as its FIXTURE, and its scaling law must match
  the function's smoothness class**; both halves were learned from
  consecutive failures, and it is what makes **NC-108** a measurement
  rather than a number (**NC-109**, **NC-110**). **DL-026** *(added
  2026-08-12, Pass 4c)* — **DL-019's premise expired**, so NC-053 is
  **re-based off it** and is now **permanently** ungraded, because the
  conformance clause binds **reading** profiles and not a CMM's computed
  output; it also promotes the wording rule **"diverges", never
  "non-conforming"**. **DL-027** *(added 2026-08-12)* — **an
  implementation's behaviour can be keyed by the DESTINATION PROFILE'S
  CLASS**, not only by direction and path: lcms2 selects between two
  black-point estimators on device class + colour space, so the same
  pre-registered prediction is **FALSIFIED on one arm and CONFIRMED on
  the other** (**NC-138**, **NC-142**, and NC-131's withdrawn verdict).
  **DL-028** *(added 2026-08-12)* — **a residual that is large under
  EVERY hypothesis is an apparatus fault, not a finding**; filed with
  the `transicc` 0..100-vs-0..255 unit error that NC-140 caught and that
  a one-armed experiment could not have. **DL-029** *(added 2026-08-12)*
  — **the API sealing split: seal what decodes OUR format, publish what
  implements SOMEONE ELSE'S specification** (`iccce-profile::num` →
  `pub(crate)`; `bpc.rs`'s ISO/CD 18619 surface stays public), filed
  with the four pre-publication soundness defects including the
  **stale-inverse hazard on a public field** that rule 1 makes invisible.
  **DL-030** *(added 2026-08-12)* — ★★★ **iccce was NON-CONFORMANT at
  ISO/CD 18619 4.2.5.4 and lcms2 conformed**; the first time rule 7 has
  run in the direction it was written to be capable of running in, filed
  with the pre-commitment (§3.18.6) that made the outcome cheap to
  accept and with the corollary that widened the return type
  (**§3.24**, **NC-164**). **DL-031** *(added 2026-08-12)* — **an
  unlabelled test count is not a claim, because the APPARATUS is half
  the number**; filed with the day's three green results (**129**, **36**,
  **142**) from three runners, two of which were briefly compared by the
  engineer who produced both (**§3.22**, **NC-158 … NC-160**, and §1's
  new `apparatus-census` class). **DL-032** *(added 2026-08-12)* — **an
  EXPECTED warning is documented at the site with what "fixing" it would
  break**, filed with the near-miss it prevented on the same day:
  deleting `license-file` to silence a cargo warning would have shipped
  a tarball with **no MIT notice text**, invisibly. ★ **Rule 1 in a
  non-colour register — the clean build IS the defect.**
- `tools/difftest/README.md` — the oracle, its pin and its licence (§2–§3),
  the smoke record (§8), the harness and its one registered check (§11),
  and **§12, the legacy-Lab experiment and the BPC finding** — the
  evidence behind every §3.6 row. **§13** *(added 2026-08-11, closure)*
  is the evidence behind every **§3.8** row: the profile pair (§13.1),
  the instrument and its dependency decision (§13.2), the `>1.0`
  finding (§13.4), the seven records (§13.5), **the two experiments that
  TEST the tolerances' justifications** (§13.6), the grid and what it
  does not cover (§13.7), the coverage statement (§13.8), the emitted
  lines (§13.9) and what §13 still owes (§13.10). **§14** is the
  evidence behind every **§3.9** row (the A2B LUT differential), and
  **§15** *(added 2026-08-11, Pass 4b)* is the evidence behind every
  **§3.11** row: the B2A direction and the **trilinear override**
  (§15.2), the synthetic v4 fixture, the **closed forms** and the
  **encoded-PCS overflow** (§15.3), the gray axis and the **457×
  attribution** (§15.4), the 28 emitted lines (§15.5), the three-part
  coverage statement (§15.6) and what §15 owes (§15.7).
- `docs/SESSION_LOG.md` — 2026-08-11, Pass 1; 2026-08-11 (Pass 2 batch 1
  + difftest); 2026-08-11 (Pass 2 batch 2 + the sweep); and 2026-08-11
  (**Pass 3 core + the `transform` CLI**), which is where §3.7 comes
  from.
- `D:\Dev\Rag-Specialized\ICC_Spec\` — the standards corpus. Read a
  file's frontmatter `evidence:` line before citing it; the tiers are not
  equal.
