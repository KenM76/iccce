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
| **implementation-cross-check** | Agreement with lcms2 or another independent implementation. | Evidence that two implementations read a clause the same way. Two implementations can share a misreading (`TOLERANCES.md` §1). **Weaker than ground truth and must be labelled so.** |
| **oracle-behaviour-at-pin** *(class added 2026-08-11, Pass 2 filing — see §3.6)* | A measurement of **what the oracle does**, at a named commit, with **iccce not in the loop at all**. Either side of the comparison is lcms2 or a hand-transcription of lcms2's own arithmetic. | Establishes what iccce **will be compared against**, and nothing else. It is **not** evidence that iccce is correct (iccce did not participate) and **not** evidence that lcms2 is correct (the specification is the authority, not the implementation — rule 7). Every such row is scoped to one pin, and **the pin moving invalidates it**. |
| **normative-rule-conformance** *(class added 2026-08-11, Pass 3 filing — see §3.7)* | The expected **behaviour** is derived from **verbatim normative specification text** transcribed in the corpus at `primary_spec` tier — not from a published numeric dataset, not from another implementation, and not from iccce. | Proves the implementation does what the clause says, **as the corpus transcribes the clause**. It therefore inherits the **transcription risk**: one PDF extraction pipeline, cross-checked against others but not read from paper by anyone here. Weaker than **published-ground-truth** (whose datasets are adversarially designed to catch a wrong reading); **stronger than arithmetic-identity**, because the expectation comes from outside the code. Distinct from **primary-spec-constant**, which is about the provenance of a *number* rather than the correctness of a *rule*. |
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

> **Dated note, 2026-08-11 (Pass 3 filing): NA-003's other half has now
> landed, in the CMM layer where this entry said it belonged.**
> `MatrixTrc::pcs_to_device` clamps each linear component to [0,1]
> **before** the inverse TRC, per **F.8–F.16**, and the order is
> asserted on measured output — **NC-027**. So the layering decision has
> been discharged as designed: `iccce-color` still does not clamp, and
> the CMM does, at the point the normative text specifies. *(verified —
> read.)*

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
  NC-025 caught.
- `tools/difftest/README.md` — the oracle, its pin and its licence (§2–§3),
  the smoke record (§8), the harness and its one registered check (§11),
  and **§12, the legacy-Lab experiment and the BPC finding** — the
  evidence behind every §3.6 row.
- `docs/SESSION_LOG.md` — 2026-08-11, Pass 1; 2026-08-11 (Pass 2 batch 1
  + difftest); 2026-08-11 (Pass 2 batch 2 + the sweep); and 2026-08-11
  (**Pass 3 core + the `transform` CLI**), which is where §3.7 comes
  from.
- `D:\Dev\Rag-Specialized\ICC_Spec\` — the standards corpus. Read a
  file's frontmatter `evidence:` line before citing it; the tiers are not
  equal.
