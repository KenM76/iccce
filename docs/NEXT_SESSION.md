# NEXT SESSION — start here

**Written 2026-08-12 by `icc-librarian`, at the Pass 6 + Pass 7
completion filing — the first of a second calendar day, and the twelfth
filing overall.** Replaces the Pass 5 edition entirely. Overwrite this
file once acted on.

Read order: this file → `docs/ROADMAP.md` (the **"what remains"** block
under Pass 8 first — it is new and it is the whole picture — then the
**Pass 6** and **Pass 7 completion records**, then the **Pass 4** and
**Pass 5 addenda**) → `docs/NUMERIC_CLAIMS.md` (**§2.9** → **§3.13**,
starting with its coverage box, then **§3.13.2** the transcription
note and **§3.13.4** the control → **§7.9**) → `docs/ARCHITECTURE.md`
§5 (**twenty-five** entries; **DL-024** and **DL-025** are new) →
`docs/SESSION_LOG.md` (twelve entries; the twelfth is this work) →
`docs/LEGAL.md` §2.1 and §2.5 → the corpus's
`icc__ref__ambiguity_register.md`.

---

## ★★ Three things that are true today and were not true yesterday

1. **The project is PUBLIC.** `master` is pushed to
   `https://github.com/KenM76/iccce`. **DL-024.** Everything written
   from now on is written for strangers.
2. **The original scope is essentially complete.** Passes 0, 1, 2, 3,
   5, 6 and 7 are DONE. **Pass 4 is the only original Pass still open**,
   at two items, and **only one of them was ever operator-blocked — that
   block is gone.**
3. **★ This project's own repository is a readable source.** The
   plain-text files under `.git/` (`config`, `logs/HEAD`,
   `logs/refs/remotes/origin/master`, `refs/…`) can be read without a
   shell. **This is not "an agent ran git"** — no agent has, and the
   sentence *"no agent in this project has ever run a git command"* is
   still true. But every commit hash the eight earlier provenance blocks
   carried as a **report** is now **corroborated**, and the first time
   anyone looked, it **found a wrong one** (below).

---

## Where the project actually is

| | Commit *(hashes now **corroborated by `.git/logs/HEAD`**, read; **contents still unverified**)* |
|---|---|
| Pass 0 · Pass 1 | `f976a0e` · `7313c5b` |
| Pass 2 batch 1 · difftest harness · batch 2 | `b35a12e` · `bfd6b1e` · `d40d601` |
| Pass 3 core · `transform` · audits · filing | `c4038eb` · `051707f` · `55772c6` · `a9618fe` |
| CLUT · PCS encodings · absolute intent · Pass 3 differential | `fc5ff58` · `0843094` · `6873df1` · `986dae6` |
| Pass 3 closure · stages 1–3 · CLI · doc catch-up · Pass 4 differential · **`edce48b`** *(★ CORRECTED — every prior edition of this file said `edcb60e`, which matches nothing in the reflog)* | `19a3b17` · `9aa1bca` · `63874f9` · `b3f4388` · `490191b` · `db60e92` · `d9e0b82` · **`edce48b`** |
| `mAB ` eval · gen-profiles + 38 fixtures + GP-001 found · GP-001 fixed + `mBA ` · grayTRC F.2 + filing | `26e92b8` · `7576cfa` · `2e98cfd` · `97ad9fa` |
| **Pass 7 CORE (named colours), 2026-08-11** | **`40cf384`** |
| Pass 4b filing + gray-through-`Chain` · corpus 7th pass in code · Pass 4b measurements | `9e2e29e` · `a0310c7` · `3d0c183` |
| Pass 4b filing committed + `lib.rs` §Status fix | `8be1ed3` |
| BPC core — **two red commits, each corrected by the next** | `70411dd` → `a36abaf` · `6ea1b3d` → `812a215` |
| `--bpc` CLI · Pass 5 measurements · Pass 5 filing | `46f16e8` · `df3a233` · `ea9cbab` |
| **★ 2026-08-12: A4c disclosure · ISO/CD 18619 estimation · Pass 6 · Pass 7 wired** | **`bb5d6b8`** · **`0378f76`** · **`3502cb7`** · **`f6203b8`** |

**`f6203b8` is the tip of `master` AND of `origin/master`** *(verified —
both ref files read)*. **Two `update by push` lines exist**, at
06:51:17 and 06:54:50 −04:00 on 2026-08-12 *(verified — read)*.

**★ A discrepancy left open on purpose:** the dispatch reported **49
commits**; `.git/logs/HEAD` holds **45** lines with no history-rewriting
entry. **Nobody has run `git log`. One command settles it, and settling
it is item 0 below.**

### The one thing to read before touching anything

**Pass 6's headline number was nearly a spectacular measurement of
nothing, and the only reason it is not is that a control failed.**

- The compiled-vs-reference error is **0.003589 device units**,
  off-node, on **SWOP `A2B1` (`mft2`, 4-D) → sRGB matrix/TRC,
  media-relative, 17-point grid**. **`self-consistency` — both arms are
  iccce.**
- The sensitivity control's **first** draft used **sRGB → sRGB** and
  returned **1.1×10⁻¹⁵ at ratio 0.94**. A grid reproduces an identity
  chain **exactly everywhere**, not merely at nodes. **That number would
  have been reported as the compiled path's cost.**
- Its **second** draft probed across sRGB's TRC breakpoint and returned
  **ratio 1.44** — because error across a derivative discontinuity
  scales `h¹`, not `h²`. **Neither the code nor the fixture was wrong;
  the expectation was.**
- **DL-023 predicted this trap by name at the previous filing, and it
  was walked into anyway.** Filed as **DL-025**.

**Take from it what the entry takes:** in this project the thing that
catches an error is never a re-reading of the code and never the number
looking wrong — **10⁻¹⁵ looks magnificent**. It is always an apparatus
built to fail. Three instances in two days: **DL-016** (off-by-one
sample), **DL-020** (GP-001), **DL-025** (this).

---

## The numbers this filing added — quote them with their scope

**Pass 6** (`machine-timing` and `self-consistency`; **one machine, one
run, release, no repetition, no variance**):

- **8 700 867 px in 7.23 s = 1.20 Mpix/s**; grid build (83 521 chain
  evaluations) **1.04 s**; reference **0.084 Mpix/s**; **speedup
  14.4×** — **and lcms2 was never timed, so the ratio is iccce against
  iccce**.
- **Off-node compiled error 0.003589 device units.** **No ΔE
  translation exists.** Do not supply one by intuition (DL-004).
- **The control's PASSING ratio is not on record**; only its two failing
  ones are.
- ★ **14.4× does not reproduce from the other quoted figures**
  (`1.2034 / 0.084 = 14.3`). Not an error — `cmd_bench` divides
  unrounded values — but **the raw twelve-line output was never
  filed**. §3.13.2.

**Pass 7** (behavioural `self-consistency`):

- **Every spot in `fixtures/synthetic/v2-ncl2-named.icc` resolves into
  the real system sRGB profile, in gamut**; **an unknown name returns
  `None`** — the `/Alternate` signal, not a guess.
- **NC-111 asserts a RANGE, not a colour.** A resolution wrong by 10 ΔE
  but inside `[0, 1]` passes it. **No spot's resolved value has ever
  been compared to anything.**

---

## ★★ Then: the work, in dependency order

### 0. Five minutes with a shell, before anything else

The project has gone seven filings without these, and it is now public:

- **`cargo test --workspace`, and record the outcome.** **116 `#[test]`
  declarations across 19 files under `crates/`** *(verified —
  counted)*; **that is not a pass result.** **NC-057 … NC-061 have no
  reported outcome at all**, and **two commits in this repository's
  history shipped red under messages claiming green.** Use the
  mechanical gate, not the attentional one:
  `cargo test --workspace -q > log 2>&1; TESTS=$?` then gate on
  `$TESTS`. **Exit codes compose; a pipe does not.**
- **`git log --oneline | measure`** — settle 45 vs 49 (§2.9).
- **Paste the twelve-line `iccce bench` output somewhere durable.** It
  is the entire evidence for four ledger rows and nobody kept it.

### 1. ★★ Pass 8 — the `pdfce` bridge. It is the real next step and it is NOT in this repository

`ARCHITECTURE.md` §4 fixes the boundary and it does not move: **a thin
bridge crate *in `pdfce`*, and `iccce` must not know what a PDF is.**
`/ICCBased` → `iccce_profile::Profile`; `/Separation` and `/DeviceN` →
named-colour lookups; PDF/X `/OutputIntent` → a destination profile.

**What Pass 7 handed it:** `NamedColors::resolve_to_device`, returning
**`None`** for an unknown name — exactly the `/Alternate` fallback
signal, and deliberately not an error.

**What this repository owes the bridge and has not delivered:**

- **A spot resolved into a LUT destination.** A press profile is the
  normal `/OutputIntent` and it is a LUT profile. **The LUT and gray
  arms of `pcs_to_destination` are reachable from a spot today and have
  never been exercised from one.**
- **Any cross-check at all on the spot path** — see item 2.
- **A statement of what a caller should do with a reported
  malformation.** Rule 6 says the parser reports and does not repair.
  **No caller that must keep going has ever exercised that**, and a PDF
  consumer will hand iccce real-world profiles at scale. **That is a
  `pdfce`-side design question and this repository should not answer it
  unilaterally** — but it should be asked before the bridge is written.

### 2. ★ The cheapest genuine cross-check in the project, and it was skipped

**Resolve a spot into ITS OWN profile's device space and compare against
the entry's stored `nDeviceCoords`.** An `ncl2` entry carries the device
values *the profile's author* recorded. That is an expectation **iccce
did not write**, on bytes iccce did not choose — the only such
expectation available anywhere on the spot path, and it costs almost
nothing. `icc-conformance`.

### 3. ★ Finish Pass 4 — and re-read its second item, because it changed

- **Saturation in the B2A direction** (`B2A2` is a distinct third
  table). Cheap, unblocked, never run. **Not the same item as Pass 5's
  saturation gap**, which is a *capability* gap in iccce's BPC subset.
- **★ ICC-absolute through a LUT destination — NO LONGER
  OPERATOR-BLOCKED.** **A4b is RESOLVED**: `ICC.1:2001-04` **A.3.1.1**
  addresses the profile's **author** and is **silent on readers**. So
  the arithmetic can be measured now, and **whether NC-053/NC-054 stay
  REPORTED-NOT-GRADED under DL-019 is a judgement `icc-conformance` must
  actually make** rather than defer. *"The authority does not exist"*
  was true yesterday and is not true in the same way today.
  **A4c** — no clause requires a profile's `wtpt` to agree with its
  colorants — is **SILENT**, and **does not clear when A4b clears**.
- **A gray profile as DESTINATION**, over non-neutral PCS input — still
  the only thing that measures **NA-008**.
- **The M3 out-of-gamut excursion count**, still unrecorded.
- **README §15.5's build-commit line** (names `97ad9fa`, which predates
  the clamp change) — carried unverified through two filings now.

### 4. ★ The apparatus gaps, and the Pass 6/7 ones are new

- **`tools/difftest/src/pass5.rs` still has no `#[test]` declarations**
  *(carried — `tools/` was deliberately not re-read at this filing,
  because `icc-conformance` was working there)*. **Fourteen ledger rows
  rest on two grids that nothing pins.**
- **★ Neither Pass 6 nor Pass 7 ran `tools/difftest` at all.** §3.13's
  eight rows have **no runner outcome of any kind** and reconcile
  against nothing.
- **`TOLERANCES.md` §3.6 exists with two placeholder rows and every cell
  blank**, and both rows are stated in **ΔE2000** — a unit **nothing in
  Pass 6 measured** *(verified — read; `icc-conformance`'s file, its
  call)*.
- **A reported runner result for README §16**, still owed.

### 5. ★ The pre-registered prediction that is awaiting a number

`bpc.rs` records, **before the run**: ISO/CD 18619 4.2.6 says the black
points' `a*`/`b*` *"are ignored"*; **lcms2 retains chroma and propagates
it per-channel**. At input black the difference should equal exactly the
destination black's `√(a*² + b*²)` — **2–6 ΔE76** for a `b*` of −2…−6,
decaying to zero at white, **on relative colorimetric with a LUT
destination**. `icc-conformance` was reported to be measuring it in
parallel with this filing.

> **★ Read DL-011 and DL-012 before believing it.** This project has
> already once predicted an lcms2 divergence in advance and **measured
> it absent** — and the run that did so found a *bigger* divergence
> nobody had looked for. **A predicted divergence is not a finding.**
> Whatever the number is, it gets a `NUMERIC_CLAIMS.md` row **in either
> direction**, and the absent case is the more interesting one.

### 6. The two holes that have outlasted every Pass

- **★★ No `published-ground-truth` row exists for ANY transform.** Not
  one, across Passes 3, 4, 4b, 5, 6, 7. **The cheapest route is still
  `IEC 61966-2-1`'s sRGB primaries, and nobody has dispatched for it** —
  for the seventh consecutive filing. `icc-spec-librarian`.
- **★★ A Linux run of anything at all.** Still nothing, by anyone,
  ever — and now in public, where *"works on Windows"* is a narrower
  claim than a reader will assume. **Six of §3.13's eight rows skip
  without this machine's colour directory.**

### 7. The instrument Pass 5 named, still unbuilt

**A synthetic v4 RGB-or-gray LUT fixture with a NON-ZERO device black**
(`tools/gen-profiles`). It is the **only** thing that can discriminate
the two black-point **estimators**, and therefore the only route to
**NA-009's** cost. **Sourcing the estimator on ISO/CD 18619 did not
change this** — sourcing is not measuring. Every profile in reach has
`trc(0) = 0`; `fixtures/synthetic/` holds **38 `.icc`** *(verified —
enumerated)*, one v4 LUT, black zero.

---

## Owed work, carried explicitly

### 1. `icc-conformance`

- Items 2, 3, 4 and 5 above.
- **A ΔE2000 translation of NC-108's 0.003589 device units** — rule 4
  requires a named approximation's cost in ΔE, and §3.6's own rows are
  written in that unit.
- **The sensitivity control's passing ratio**, so its margin is known.
- **A compiled path measured in the B2A direction** (DL-021 makes it a
  separate question) and **a compiled chain with BPC folded in** — the
  configuration Pass 5 made legitimate and Pass 6 did not exercise.
- **A repeat timing run, and a second machine.**
- **Whether to re-grade NC-077** (the encoded-PCS overflow) — its file,
  its call, carried since Pass 5.
- **A synthetic `lut8` fixture wired into the suite**;
  `fixtures/synthetic/v2-cmyk-mft1-lab.icc` exists unused.
- **`TOLERANCES.md` §3.2 (Pass 2), §3.6 (Pass 6), a §3.7 (Pass 7), and
  §6's coverage table.**
- **An instrument check on iccce's ΔE ruler for the sRGB destination.**
- **A re-run of the Pass 2 machine sweep** against a post-GP-001 build.
- **A behavioural test of `ncl2` legacy-Lab decoding** — owed since
  Pass 2, and **Pass 7 does NOT discharge it**: NC-019's coverage line
  still rests on a source reading.

### 2. `icc-spec-librarian`

- **★ `IEC 61966-2-1`** — still **the** cheapest route to the project's
  first ground-truth row for a transform. **Nobody has dispatched for
  it.**
- **★ The tier question that decides a ledger CLASS**: is
  `icc__ref__bpc.md` §2/§3 `primary_spec` or `cross_verified_2src`? At
  `primary_spec`, **NC-084/NC-086 become `normative-rule-conformance`
  rows**. `bpc.rs` still heads 6.3.4.3 **"PRIMARY-SOURCED"** *(verified
  — read)*.
- **★ The forcing-policy question is NOT settled by ISO/CD 18619.**
  That document supplies **estimation**; **NC-100 / DL-022 turn on
  applicability** — whether BPC's *enablement* is specified as a
  function of intent and version. **NC-100 stays REPORTED, NOT GRADED**
  until something says it is.
- **The clamp question, NARROWED**: must the final `B` curves' output be
  clipped to 6.3.4.2's encodable PCS range, and does 10.18's domain bind
  the *evaluator* or only describe the stored samples?
- **Corpus rows owed**: the **M2 correction**; the **trilinear
  override**; **the `IsEmptyLayer` 0,002 threshold** (`bpc.rs` now
  records it code-side as one of three constants with no home in either
  BPC document — the *corpus* row is still owed); **A41's ΔE2000 =
  0,050 201**.
- **`A31`** — now **the only UNVERIFIED row in the whole register**
  *(verified — read)*. Needs `ICC.1:2010-12` (v4.3).
- **The ITU terms determination** before any BT.709 or BT.2100 fetch
  (DL-007). **"The file is free" has never implied "automated retrieval
  is permitted"** — DL-002 exists because that inference was available
  at color.org and would have been wrong.

### 3. `icc-engineer`

- **Item 0's five minutes with a shell.**
- **The mechanical commit gate.** Two commits in this history shipped
  red under green messages. **Exit codes compose; a pipe does not.**
- **Whether iccce should implement lcms2's `EvalNInputs` geometry at
  all** — DL-021 makes it two choices, not one.
- **A PCSXYZ `ncl2` fixture**, so Table 66's second permitted encoding
  is not carried on a source reading.
- **crates.io, if and when the operator says so**: **name availability
  is still unchecked by anyone**, and **`THIRD_PARTY_LICENSES.md` via
  `cargo-about` is owed before a first publish**. **A public git
  repository is not a published crate** (DL-024).

### 4. `icc-librarian` / whoever files next

- **★★ The DL-014 citation audit.** It now does two jobs: it decides
  **NC-084's ledger class**, and it is **the audit that would move
  DL-024's third pre-publication check from *reported* to *verified*** —
  a compliance claim that is now published. `iccce-color` and
  `iccce-profile` have **never** been swept.
- **A re-read of `tools/`** — two §7.8 items were carried into §7.9
  without re-verification because another agent was working there.
  **A carried item is a claim with a date on it, and this is the second
  filing in a row that carried some.**
- **A re-read of README §15**, carried unverified twice now.
- **Observed residuals** for Pass 1's rows and for NC-032.
- **A ground-truth row for chromatic adaptation** — NA-002 still not
  due.

### 5. The operator

| Document | What it settles |
|---|---|
| **IEC 61966-2-1** | the **first ground-truth row for a transform**, and the end of the single-source sRGB/D65 lineage. **The largest remaining hole in the project** |
| **`ICC.1:2010-12` (v4.3)** | **A31 — now the register's ONLY unverified row.** `parametricCurveType` Table 68 across editions |
| **ICC's published D65→D50 `chad` values** (Annex E.4.2) | the adaptation ground-truth hole |
| **ITU-R BT.709 / BT.2100** | Pass 9's precondition — **blocked first on the terms determination**, not on the download |

**Each row is a claim about what a document contains.** Treat *"it would
settle X"* as a prediction until the document is open. **`ICC.1:2001-04`
is the worked example**: it was expected to adjudicate an 11 ΔE
divergence and instead turned out to be **silent on the question**.

---

## Decisions already made — do not re-litigate

- **★ The project is PUBLIC** (**DL-024**) — and that authorises
  **nothing else**. No crates.io publish, no tag, no release, and the
  next push needs its own current go-ahead. **"He said yes on the 12th"
  is not standing permission**, for exactly the reason *"we decided in
  August"* was not.
- **MIT**, dependencies permissive, **publishing is the operator's act**
  (rule 9).
- **`iccce-color` depends on nothing** and contains no ICC. **The
  fixture generator depends on nothing either.**
- **The parser reports, it does not repair**; in the CMM the same
  instinct is **refuse by name, never substitute** — and where a file is
  self-inconsistent in a way no clause adjudicates, **disclose** (A4c).
- **lcms2 is the oracle, never a dependency** — subprocess only, pinned
  by commit hash (DL-001). **It is also not a dependency of the
  published artefact**, and a tidy-up that folds `tools/difftest` into
  the workspace would now break that **in public**.
- **DL-003** duplicate tags · **DL-004** the ⚠ provisional 1.0 anchor ·
  **DL-005** exact invariants for legacy Lab · **DL-007** HDR in scope ·
  **DL-008** profile creation in scope · **DL-009** crates.io intent
  (**not an authorisation**) · **DL-010 / NA-001** the rational
  breakpoint · **DL-011 / DL-012** the tag-type selector, and a
  predicted divergence measured **absent** · **DL-013** lcms2's forced
  BPC, keyed by the **destination** · **DL-014** the terms for citing
  ICC.1:2022 · **DL-015 / NA-004** the `pow` guard · **DL-016** exact
  values at sample points · **DL-017** the harness may path-depend on
  iccce's crates · **DL-018** a prediction pin for an upper-bound gate ·
  **DL-019** report-not-grade when the mechanism is known and the
  authority is not · **DL-020** refuse-don't-guess, discharged by an
  independently authored fixture · **DL-021** a behaviour is a fact
  about **one direction and one path** · **DL-022** iccce never forces
  BPC · **DL-023** say what the two sides were free to disagree about,
  before the run.
- **DL-025** *(new)* — **a sensitivity control is only as good as its
  FIXTURE, and its scaling law must match the function's SMOOTHNESS
  CLASS.** Both halves were learned by the control failing.

### Everything measured against lcms2 is scoped to commit `21c582a`

Moving the pin is a **licence** event (DL-001) **and** a behavioural
one. **Re-run, not re-read:** NC-019 … NC-021, NC-034 … NC-037, NC-040,
NC-041, NC-043, NC-044 … NC-050, NC-053 … NC-057, NC-062 … NC-083,
NC-088 … NC-096, NC-099 … NC-102. **The sharp ones are NC-050, NC-056,
NC-082 and NC-088** — each is a *transcription* of lcms2's internals, so
a retuned interpolator, resampler or threshold invalidates them
**silently**, by continuing to reproduce the old lcms2 perfectly.
**NC-084 … NC-087 and NC-105 … NC-112 are the only rows the pin cannot
touch** — **because no other implementation is in them at all**, which
in Pass 6 and Pass 7's case is a weakness rather than a virtue.

---

## Method reminders that stay load-bearing

1. **A wrong colour looks exactly like a right one** — **and so does a
   wrong measurement.** Pass 6's instance: **1.1×10⁻¹⁵ with a ratio of
   0.94.**
2. **★ An instrument is only as good as its fixture.** DL-025. Ask what
   the control would return if the effect were identically absent, and
   **check that its fixture is not that case**.
3. **★ A scaling law is a claim about a function's smoothness.** `h²`
   holds where the function is twice differentiable and nowhere else.
   A control failing across a kink is **right about the wrong region**.
4. **Ask what the two sides were free to disagree about.** DL-023 —
   and in Pass 6 the answer at a grid node is *nothing at all*.
5. **Print the sensitivity ratio.** Pass 5's were 388× and 682×;
   Pass 6's control band is 2×–8×, **and its passing value is not on
   record**.
6. **Say which direction and which tag type.** DL-021, in every Pass 6
   and Pass 7 row.
7. **Verify against the live source, not the dispatch.** This filing's
   instances: **the date** (dispatch said 2026-08-11; three sources say
   2026-08-12), **the hash** (`edcb60e` → `edce48b`, wrong in three
   documents), and **the arithmetic** (14.4× does not reproduce from the
   figures quoted beside it).
8. **A gate figure is a claim.** **Two commits in this history claimed a
   green suite falsely.** **Exit codes compose; text matching does
   not.**
9. **A class is not raised by how good the number looks.** NC-108 is
   `self-consistency` at 0.003589, and would still be if it were 10⁻¹⁸.
10. **Coverage is part of every claim.** *"Pass 6 is done"* means **one
    machine, one run, one direction, one tag-type pair, one intent, one
    grid density, and no other implementation timed.** *"Pass 7 is
    done"* means **one fixture this project wrote, one matrix/TRC
    destination, one of two permitted encodings, and a range assertion
    rather than a colour.**
11. **A count is not an inventory.** **116 `#[test]` declarations** is
    not coverage and not a pass result. **38 fixtures** is not 38
    checked behaviours.
12. **Do not assert unmeasured facts about the environment.** ★ This is
    the reminder that changed today: **some** repository facts are now
    measurable from files, and they must be labelled as read rather than
    inferred. **A push is evidence of a push. It is not evidence of
    public visibility**, and the difference is exactly the kind this
    project exists to keep.

---

## The agents

- **`icc-engineer`** — lead. Be this agent if orchestrating. **Owes**
  item 0's five minutes with a shell and the mechanical commit gate.
- **`icc-spec-librarian`** — the standards corpus. **Owes** IEC
  61966-2-1, the tier question that decides a ledger class, the
  forcing-policy question (**not** settled by ISO/CD 18619), the
  narrowed clamp question, four corpus rows, A31 and the ITU terms.
- **`icc-conformance`** — the oracle, the fixtures, the tolerance
  budget. **Owes** the spot-path cross-check, saturation and
  ICC-absolute in B2A, gray as a destination, a ΔE for NC-108,
  `pass5.rs`'s missing tests, `TOLERANCES.md` §3.6/§3.7, and the
  chroma-divergence measurement.
- **`icc-librarian`** — ROADMAP, decision log, session log, and
  `NUMERIC_CLAIMS.md`. **No shell** — a dispatch to it must carry its
  evidence. **Owes** the DL-014 citation audit, which now underwrites a
  published claim.

Dispatch them freely, and in parallel on disjoint file sets; no
permission is needed to dispatch an agent to read, analyse or draft.
