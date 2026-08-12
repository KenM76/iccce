# NEXT SESSION — start here

**Written 2026-08-12 by `icc-librarian`, at the estimator-discrimination
filing — the fifth of the second calendar day and the fourteenth
overall.** Replaces the Pass 4 completion edition entirely. Overwrite
this file once acted on.

> **★★★ THE ORIGINAL SCOPE IS DONE. Passes 0 through 7 are closed and
> filed.** This file is therefore shaped differently from every edition
> before it: it is **not** a work queue for finishing a plan. It says
> **what is complete**, **what remains and why each remaining thing is
> not merely unfinished work**, and **what to read first**.

**Read order** — this file → `docs/ROADMAP.md` (the **header's latest
status block**, then the **"what remains"** block's **dated update at
the end**, then the **Pass 5 addendum 2** and the **Pass 6 addendum**) →
`docs/NUMERIC_CLAIMS.md` (**§2.11**, then **§3.18** — read **§3.18.1**
and **§3.18.2** before any number — then **§3.17**, **§3.19**, **§3.20**,
**§3.21**, then **§7.11**) → `docs/ARCHITECTURE.md` §5 (**twenty-nine**
entries; **DL-027**, **DL-028**, **DL-029** are new) →
`docs/SESSION_LOG.md` (fourteen entries) → `docs/TOLERANCES.md`
**§3.5.7**, **§3.5.8**, **§3.6** and the 2026-08-12 rows in §4 →
`tools/difftest/README.md` **§19**.

---

## ★★★ The one thing to read before touching anything

**lcms2 has TWO black-point estimators at media-relative, and which one
runs is decided by the DESTINATION profile's device class and colour
space** (`cmssamp.c` L370–374). Ink space + output class →
`BlackPointUsingPerceptualBlack`, which **forces the chroma to zero**.
Anything else → `BlackPointAsDarkerColorant`, which **keeps it**.

| | `USWebCoatedSWOP.icc` (v2 `prtr` **CMYK**) | `v4-rgb-mab-chromatic-black.icc` (v4 `prtr` **RGB**, ours) |
|---|---|---|
| divergence from iccce's ISO/CD 18619 estimate | **8,166 8×10⁻² ΔE76 — 100 % `L*`, chroma exactly 0** | **5,000 000 ΔE76 — 100 % chroma, `ΔL*` exactly 0** |
| the corpus's pre-registered **mechanism** claim | **FALSIFIED** | **CONFIRMED** |

> **★★★ A session that ran only one arm would have filed a confident
> wrong headline EITHER WAY** — *"lcms2 keeps its black's chroma"* or
> *"the chroma prediction was imaginary"* — **and both would have been
> supported by a clean, tight, honestly bounded measurement.**

**Take three things from it:**

1. **DL-027 — a behaviour is a fact about the direction, the path AND
   the CLASS OF PROFILE it was measured on.** DL-021 named the first
   two because that is what one day's evidence supported. **The same
   shape of error recurred one axis over, in the same oracle.**
2. **DL-028 — a residual that is large under EVERY hypothesis is an
   apparatus fault, not a finding.** The synthetic arm's first run was
   out by four orders because `transicc` prints ink as `0..100` and
   **RGB and gray as `0..255`**, and three Passes had divided by 100.
   **Two candidates caught it; reading the code would not have.**
3. **★ An error bar the same order as its effect may BE the
   measurement.** Pass 5b's was **0,813 7** against an effect of
   **0,858 17**; **98,3 % of its headline was the apparatus.** It
   reported the ratio as **marginal (0,948 against 1,0)** rather than
   green, and that honesty is the only reason this was findable.

---

## What is COMPLETE

| Pass | Status |
|---|---|
| **0** scaffold + oracle · **1** colorimetry · **2** parsing · **3** matrix/TRC | **DONE** |
| **4** LUT transforms and intents | **DONE** (2026-08-12) |
| **5** black point compensation | **DONE**, and its one stated boundary — *"the estimators were never discriminated"* — **is now closed** (`ROADMAP.md` Pass 5 addendum 2) |
| **6** performance | **DONE**, and **its gate now PASSES** at the new default grid of 33, against an unchanged tolerance |
| **7** named colours and spot | **DONE** |

**And Pass 1's remainder is down to three items from four** — ΔE94 and
ΔE CMC exist, `impl_crosscheck`, honestly labelled.

**★ "DONE" means what each completion record says it means, and no
more.** Every one carries a coverage statement. **None of them means
*verified*, and none means *verified against ground truth*.**

---

## ★★★ What REMAINS — four kinds, and only one of them is a task list

### Kind 1 — it is in another repository

**Pass 8, the `pdfce` bridge.** `ARCHITECTURE.md` §4 fixes the boundary
and it does not move: **a thin bridge crate *in `pdfce`*, and `iccce`
must not know what a PDF is.** `/ICCBased` → `iccce_profile::Profile`;
`/Separation` and `/DeviceN` → named-colour lookups; PDF/X
`/OutputIntent` → a destination profile.

**What Pass 7 handed it:** `NamedColors::resolve_to_device`, returning
**`None`** for an unknown name — the `/Alternate` fallback signal,
deliberately not an error.

**What this repository still owes it** *(and this is the only part of
Pass 8 that can be done here)*:

- **A spot resolved into a LUT destination.** A press profile is the
  normal `/OutputIntent` and **is** a LUT profile. The **gray** arm was
  exercised once as a scratch probe (§3.16.2); **the LUT arm has never
  been reached from a spot.**
- **Any cross-check at all on the spot path.** See Kind 4 item 2.
- **A statement of what a caller should do with a reported
  malformation.** Rule 6 says the parser reports and does not repair;
  **no caller that must keep going has ever exercised that**, and a PDF
  consumer will hand iccce real-world profiles at scale. **That is a
  `pdfce`-side design question and this repository should not answer it
  unilaterally** — but it should be asked before the bridge is written.

### Kind 2 — it is blocked on a document nobody here can produce

**These are acquisitions, not tasks. No amount of work in this tree
advances them.**

| What is needed | What it unblocks |
|---|---|
| **IEC 61966-2-1** (purchased or licensed) | **★★ The project's FIRST `published-ground-truth` row for a transform.** There are **none**, across Passes 3, 4, 4b, 4c, 5, 6, 7 — **ninth consecutive filing.** The cheapest route, and **nobody has dispatched for it** |
| **`ICC.1:2010-12` (v4.3)** | **A31 — the ambiguity register's ONLY unverified row.** `parametricCurveType` Table 68 across editions |
| **ICC's published D65→D50 `chad` values** (Annex E.4.2) | the chromatic-adaptation ground-truth hole (**NA-002**) |
| **ITU-R BT.709 / BT.2100** | **Pass 9's precondition** — and **blocked first on the TERMS determination, not on the download.** *"The file is free"* has never implied *"automated retrieval is permitted"* (DL-002, DL-007) |

> **★ Each row is a CLAIM ABOUT WHAT A DOCUMENT CONTAINS. Treat *"it
> would settle X"* as a prediction until the document is open.** The
> worked example is `ICC.1:2001-04`: it was expected to adjudicate an
> 11 ΔE divergence and turned out **silent** — and the ledger's
> prediction about *what its arrival would do to the record* was **also**
> wrong. **Two levels of prediction, both wrong, about one document.**

### Kind 3 — it is an operator scope call, answered but never sized

- **Pass 9 — HDR (BT.2100 PQ/HLG, BT.2020 primaries).** In scope by
  **DL-007**. Its hard part is **not the curves**: ICC's PCS is
  media-relative and reflective-print-derived, PQ is **absolute** and
  HLG is **scene-referred**, so connecting either needs a **stated
  choice** about reference luminance and what counts as white — an
  approximation under rule 4, which must be **named and priced in ΔE**.
- **Pass 10 — profile creation.** In scope by **DL-008**, far-future,
  and **its precondition is unsolved and is not a backlog item**:
  nothing has been chosen as a ground truth that is not iccce.
  *"Round-tripping a profile through its own inverse is the canonical
  test whose expected value came from the code under test."* ★ Note the
  distinction Pass 10 must be sized against: **writing synthetic profile
  bytes** (what `tools/gen-profiles` already does) was **never out of
  scope**; **profile creation from measurement data** is the thing that
  was refused.

### Kind 4 — the standing debts of work already done. **This is the only actionable list.**

**0. ★★★ One open question can make a shipped behaviour WRONG.**
At ISO/CD 18619 **4.2.5.4**'s mid-range straightness short-circuit —
which **both** implementations take, so **neither fits a quadratic on
either fixture** — **iccce returns `outRamp[first]`** and **lcms2
returns `InitialLab`** (`cmssamp.c` L536). **Dispatched to
`icc-spec-librarian` 2026-08-12; unanswered.** It is **the whole** of the
`swop` arm's 8,167×10⁻² ΔE76. **If ISO names lcms2's, iccce is WRONG —
not divergent — and `icc-engineer` changes the code.** Until then, **no
document here may describe this difference as lcms2 departing from the
standard.**

**1. ★★ Commit hygiene — three instances of one mechanism in two days.**
`edce48b` swept in in-progress `gen-profiles`; `aef7566` swept in and
**published `dechk.obj`**; `5cfee17` swept in another agent's
**mid-write `NUMERIC_CLAIMS.md`** *(all reported from `git show --stat` /
`git log --diff-filter=A` by the previous filing, which had a shell)*.
**Owed: commit with EXPLICIT PATHSPECS, never `-A` or a bare `.` from
the repository root, while any other agent is working in the tree.**
**And `dechk.obj` — 5 933 bytes, MSVC COFF, at the repository root, the
object file of the ΔE94/CMC C probe — is still there** *(verified — the
tree enumerated)*, with **no `*.obj`/`*.o` rule in `.gitignore`**. Add
the rule, remove the file, **and decide about history**: it is small and
benign, but ***"benign" is a judgement the operator makes about a
published artefact, not one an agent makes for him.***

**2. ★ The cheapest genuine cross-check in the project, skipped for
three filings.** **Resolve a spot into ITS OWN profile's device space
and compare against the entry's stored `nDeviceCoords`** — an
expectation **iccce did not write**, on bytes **iccce did not choose**.
The only such expectation anywhere on the spot path.

**3. ★ Two measurements the new fixture makes possible and nobody has
taken.**
- **The A41 constant's error.** At perceptual, both implementations use
  `L* ≈ 3,1` where `v4-rgb-mab-chromatic-black.icc`'s real black is
  **`L* 20`**. **The instrument now exists; the row does not.**
- **A PCSLAB gray fixture** → **NA-008's second arm**, which still has
  **no instrument in existence**. Every gray profile in reach is PCSXYZ,
  and **agreeing with lcms2 cannot substitute** — lcms2 makes one of the
  two choices too, so a cross-check is blind to the difference by
  construction.

**4. ★★ Evidence that was never filed, for numbers that are.**
- **A runner outcome for the shape of `pass5c` and `pass6` filed
  today.** The last on record is `pass=140 fail=2` on a **shape that no
  longer exists**. **Twenty-four records have no `pass=` line.**
- **A `cargo test --workspace` outcome at the current tip.** The last is
  **exit 0, 121 passed** at `95c04c1`; one commit has landed since.
  **121 `#[test]` declarations exist across 19 files** *(verified —
  counted)* — **a different quantity that happens to match.**
- **The twelve-line `iccce bench` output**, owed since the Pass 6
  filing, and **worth more now** than when first asked for: the default
  grid has moved, so §3.13's figures describe a grid the binary no
  longer uses.
- **The CI evidence.** CI is **reported** to have run and passed, which
  retires a caveat repeated across these documents — **by a report**.
  **No run URL, no summary, and no statement of whether the Linux job
  was among what passed.** ★ **Do not let it discharge the Linux debt
  silently.**

**5. ★ Apparatus and hygiene, carried.**
- **`tools/difftest/src/pass5.rs` has no `#[test]` declarations** —
  **`unverified-this-filing`, not `owed`** (`tools/` was read here only
  at README §17–§19).
- **`cargo fmt --check` fails in `tools/difftest`: 109 diffs across 15
  files** *(reported)*. **Rule 10's gate is workspace-wide and
  `tools/difftest` is deliberately not a workspace member** (DL-001), so
  `--workspace` has never seen it. Bring it under the gate **or** state
  the exemption in `CLAUDE.md`.
- **Three real measurements pinned by nothing** (§3.16): the M3 A/B, the
  gray-destination probe, the Pass 4b re-run.
- **A sweep for the bare *"D.6/D.7"* citation label**, folded into the
  DL-014 audit — **Annex D is INFORMATIVE, and the label is not
  edition-stable.**
- **★ A Linux run of anything at all.** Still nothing, by anyone, ever.

---

## Owed work, by agent

### `icc-spec-librarian`

- **★★★ ISO/CD 18619 4.2.5.4's short-circuit return value** — Kind 4
  item 0. **Dispatched; it is the only open question that can make
  shipped code wrong.**
- **★★ `IEC 61966-2-1`** — the first ground-truth row for a transform.
  **Ninth filing; nobody has dispatched.**
- **The tier question that decides a ledger CLASS**: is
  `icc__ref__bpc.md` §2/§3 `primary_spec` or `cross_verified_2src`? At
  `primary_spec`, **NC-084/NC-086 become `normative-rule-conformance`.**
- **The forcing-policy question is NOT settled by ISO/CD 18619.** That
  document supplies **estimation**; **NC-100 / DL-022 turn on
  applicability**. **NC-100 stays REPORTED, NOT GRADED** until something
  says otherwise.
- **The clamp question, narrowed**: must the final `B` curves' output be
  clipped to 6.3.4.2's encodable PCS range, and does 10.18's domain bind
  the *evaluator* or only describe the stored samples?
- **Corpus rows owed**: the **M2 correction**; the **trilinear
  override**; the **`IsEmptyLayer` 0,002 threshold**; **A41's
  ΔE2000 = 0,050 201**; **M3's measured magnitude**; ★ **and a
  correction recording that the black-point prediction's mechanism is
  BRANCH-DEPENDENT and its magnitude band assumed a chromatic printer
  black that a coated CMYK profile has not got.**
- **`A31`**, and **the ITU terms determination** before any BT.709 /
  BT.2100 fetch.

### `icc-conformance`

- Kind 4 items 2, 3, 4 and 5.
- **A compiled path in the B2A direction** (DL-021 makes it a separate
  question) and **a compiled chain with BPC folded in**.
- **A repeat timing run, and a second machine.** ★ And note the timings
  now need re-stating anyway: **NC-105 … NC-108 describe grid 17**, and
  the default is **33**.
- **Whether to re-grade NC-077** (the encoded-PCS overflow) — its file,
  its call, carried since Pass 5.
- **A synthetic `lut8` fixture wired into the suite**;
  `fixtures/synthetic/v2-cmyk-mft1-lab.icc` exists unused.
- **`TOLERANCES.md` §3.2 (Pass 2), a §3.7 (Pass 7), and §6's coverage
  table.**
- **An instrument check on iccce's ΔE ruler for the sRGB destination**,
  and **a re-run of the Pass 2 machine sweep** against a post-GP-001
  build.
- **A behavioural test of `ncl2` legacy-Lab decoding** — owed since
  Pass 2; **Pass 7 does NOT discharge it** (NC-019 rests on a source
  reading).
- **A fixture whose darkest colorant has chroma above 50** — **only if
  something depends on the answer.** It would turn lcms2's clamp/return
  asymmetry from READ into RUN, and *a fixture built to trigger one
  branch of one clamp is a fixture built to make a point.*

### `icc-engineer`

- **Kind 4 items 0 (the code change, if ISO names lcms2's), 1 and 4.**
- **The `tools/difftest` `fmt` exemption question.**
- **Whether iccce should implement lcms2's `EvalNInputs` geometry at
  all** — DL-021 makes it two choices, not one.
- **A PCSXYZ `ncl2` fixture**, so Table 66's second permitted encoding
  is not carried on a source reading.
- **crates.io, if and when the operator says so**: **name availability
  is still unchecked by anyone**; **`THIRD_PARTY_LICENSES.md` via
  `cargo-about` is owed before a first publish**; and **a public git
  repository is not a published crate** (DL-024). ★ **DL-029 narrowed
  the API surface for publication and is not an authorisation either.**

### `icc-librarian` / whoever files next

- **★★ The DL-014 citation audit.** It decides **NC-084's ledger
  class**, underwrites **DL-024's third pre-publication check** (a
  *published* compliance claim still carried as *reported*), and has a
  **live defect** to sweep. `iccce-color` and `iccce-profile` have
  **never** been swept. ★ New cheap surface: `delta_e.rs`'s **CIE
  116-1995** and **BS 6923** citations — both correctly marked
  **UNSOURCED**; confirm nothing later upgrades them.
- **A re-read of README §15**, carried unverified three times.
- **Observed residuals** for Pass 1's rows and for NC-032.
- **★ Re-read anything a claim rests on IMMEDIATELY before asserting
  it.** This filing drafted a false statement about another agent's work
  from a read taken **at the start of the same session** (§7.11 item 5).
  **In a concurrent session an early read is a dispatch, not a source.**

### The operator

The four documents in **Kind 2**, plus:

- **Whether pushes three through nine were authorised.** **Nine
  `update by push` lines exist; DL-024 records two.** The reflog
  attributes them to `KenM76` and **no file records authorisation either
  way.** ★ Also reported: **one push failed with HTTP 408** and was
  retried over HTTP/1.1 — **a failed push leaves no reflog line**, so the
  failure is a report and only the success is evidence.
- **`dechk.obj` in the published history** — Kind 4 item 1.

---

## Decisions already made — do not re-litigate

- **★★★ DL-027 (new)** — **a behaviour can be keyed by the DESTINATION
  PROFILE'S CLASS**, not only by direction and path. Read the
  **condition** that selects a behaviour; **if it names header fields,
  the rule needs a second arm chosen to fail that condition.**
- **★★ DL-028 (new)** — **a residual large under EVERY hypothesis is an
  apparatus fault, not a finding.** Discrimination experiments carry a
  **second independent candidate** and grade the **ratio**, not the
  magnitude. **Corollary: an error bar the same order as its effect may
  be the measurement.**
- **★ DL-029 (new)** — **the API sealing split: seal what decodes OUR
  format (`iccce-profile::num` → `pub(crate)`), publish what implements
  SOMEONE ELSE'S specification (`bpc.rs`'s ISO surface).** Filed with
  four pre-publication soundness defects, one of them a **stale-inverse
  hazard on a public field** — *silently wrong colour with no signal*,
  which is rule 1 as an API shape.
- **DL-026** — **NC-053 is RE-BASED off DL-019 and is PERMANENTLY
  ungraded**: the conformance clause binds **READING** profiles, not a
  CMM's computed output, so a graded row is **unavailable**. ★ **The
  judgement is contingent on NC-120 existing — if the pin moves, re-make
  it, do not inherit it.**
- **★★ WORDING: say lcms2 DIVERGES. Never "non-conforming."**
- **★ A4c is SILENT and did NOT clear when A4b cleared.** Disclosure is
  the one option ICC.1 does not foreclose.
- **★ The project is PUBLIC** (**DL-024**) — and that authorises
  **nothing else**. No crates.io publish, no tag, no release, **and each
  push needs its own current go-ahead.**
- **MIT**, dependencies permissive, **publishing is the operator's act**
  (rule 9).
- **`iccce-color` depends on nothing** and contains no ICC; **the
  fixture generator depends on nothing either.**
- **The parser reports, it does not repair**; in the CMM, **refuse by
  name, never substitute**; where a file is self-inconsistent in a way
  no clause adjudicates, **disclose**.
- **lcms2 is the oracle, never a dependency** — subprocess only, pinned
  by commit hash (DL-001), and **not a dependency of the published
  artefact**. A tidy-up that folds `tools/difftest` into the workspace
  would break that **in public**.
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
  **DL-019** report-not-grade · **DL-020** refuse-don't-guess ·
  **DL-021** direction and path · **DL-022** iccce never forces BPC ·
  **DL-023** say what the two sides were free to disagree about, before
  the run · **DL-024** the publication event · **DL-025** a control is
  only as good as its **fixture**, and its scaling law must match the
  function's **smoothness class**.

### Everything measured against lcms2 is scoped to commit `21c582a`

Moving the pin is a **licence** event (DL-001) **and** a behavioural
one. **Re-run, not re-read:** NC-019 … NC-021, NC-034 … NC-037, NC-040,
NC-041, NC-043, NC-044 … NC-050, NC-053 … NC-057, NC-062 … NC-083,
NC-088 … NC-096, NC-099 … NC-102, NC-113 … NC-128, **and
NC-129 … NC-144 and NC-153 … NC-154**. **The sharp ones are NC-050,
NC-056, NC-082, NC-088 — and now NC-137 … NC-144, which are a
TRANSCRIPTION of `cmssamp.c` itself**: a retuned estimator invalidates
them **silently**, by continuing to reproduce the old lcms2 perfectly.
★ **NC-120 is sharp in a second way — DL-026's judgement depends on
it**, so the pin moving reopens a decision rather than merely
invalidating a row. ★ **NC-153/NC-154 are a transcription too**, and
their ten-decimal agreement would survive a change in lcms2 only by
being re-measured.

---

## Method reminders that stay load-bearing

1. **A wrong colour looks exactly like a right one — and so does a wrong
   measurement.**
2. **★★ Read the CONDITION, not just the behaviour.** If it names header
   fields, one arm is not an experiment (**DL-027**). If it is a
   conjunction, a confound may be removable by **choosing inputs**
   (DL-026).
3. **★★ A residual large under every hypothesis is an apparatus fault**
   (**DL-028**) — and **an error bar the same order as its effect may be
   the measurement.**
4. **★ An instrument is only as good as its fixture** (DL-025), and
   there are **three** ways for a comparison to be vacuous: the effect
   can be absent, the output can be **saturated** (both sides clamping
   to the same boundary), or the two sides can be **structurally
   identical** (a grid at its own node).
5. **Grep before recording anything as owed.** `unverified-this-filing`
   ≠ `owed`. ★ **And say WHY it is unverified** — a held directory and a
   missing shell are different reasons with different fixes.
6. **Verify against live source — and *live* means AT THE MOMENT OF
   FILING.** In a session with other agents writing, **an early read is
   a dispatch, not a source.**
7. **Say which direction, which tag type, and now which PROFILE CLASS.**
8. **Print the sensitivity ratio**, and say what floor it clears and
   where the floor came from.
9. **A gate figure is a claim.** **Exit codes compose; text matching
   does not.** *"121 passed"* is still not an inventory.
10. **A class is not raised by how good the number looks.** ★ **A
    ten-decimal agreement with another implementation is
    `impl_crosscheck`** — it is what a faithful transcription produces
    **and** what two identical mistakes produce.
11. **Coverage is part of every claim.** *"The estimators are
    discriminated"* means **two destination classes, one intent, one
    pin, one platform, and a reimplementation rather than a run.**
12. **Do not assert unmeasured facts about the environment** — **and
    check whether you can measure them.** ★ **Shell availability is a
    property of a SESSION, not of an agent**: the previous filing had
    one, this one did not. **Ask; never inherit.**

---

## The agents

- **`icc-engineer`** — lead. Be this agent if orchestrating. **Owes**
  commit hygiene and `dechk.obj`, the runner and CI evidence, and the
  4.2.5.4 code change **if** the specification names lcms2's behaviour.
- **`icc-spec-librarian`** — the standards corpus. **Owes ISO/CD 18619
  4.2.5.4 first** (it can make shipped code wrong), then IEC 61966-2-1
  (ninth filing), the tier question, the forcing-policy question, the
  narrowed clamp question, six corpus rows, A31, and the ITU terms.
- **`icc-conformance`** — the oracle, the fixtures, the tolerance
  budget. **Owes** the spot-path cross-check, the A41 constant's error,
  the PCSLAB gray fixture, `pass5.rs`'s tests, `TOLERANCES.md`
  §3.2/§3.7, and **a runner outcome for the rows filed today**.
- **`icc-librarian`** — ROADMAP, decision log, session log,
  `NUMERIC_CLAIMS.md`. **No shell this session** *(the previous session
  had one — do not inherit either state)*. **Owes** the DL-014 citation
  audit.

Dispatch them freely, and in parallel on disjoint file sets; no
permission is needed to dispatch an agent to read, analyse or draft.
