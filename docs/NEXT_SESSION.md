# NEXT SESSION — start here

**Written 2026-08-12 by `icc-librarian`, at the black-point re-measure
filing — the fifth of the second calendar day and the sixteenth
overall.** Replaces the estimator-discrimination edition entirely.
**Overwrite this file once acted on.**

> **★★★ THE ORIGINAL SCOPE IS DONE. Passes 0 through 7 are closed and
> filed.** This file is therefore shaped differently from every edition
> before it: it is **not** a work queue for finishing a plan. It says
> **what is complete**, **what remains and why each remaining thing is
> not merely unfinished work**, and **what to read first**.

**Read order** — this file → `docs/ROADMAP.md` (the **header's latest
status block**, then the **"what remains"** block's **dated update at
the end**) → `docs/NUMERIC_CLAIMS.md` (**§3.25** first — read
**§3.25.2** and **§3.25.3** before any number — then **§3.26**,
**§3.27**, then **§3.24**, then **§7.13**) → `docs/ARCHITECTURE.md` §5
(**thirty-six** entries; **DL-033 … DL-036** are new) →
`docs/SESSION_LOG.md` (sixteen entries; the latest first) →
`docs/TOLERANCES.md` **§3.6.3** and the 4.2.5.4 subsection →
`tools/difftest/README.md`.

---

## ★★★ The one thing to read before touching anything

**A cross-check's power is bounded by the SEPARATION OF THE TWO
CANDIDATE ANSWERS, not by the tightness of the residual it reports.**
On 2026-08-12 this project shipped a conformance defect that **agreed
with the oracle to 0,08 ΔE76** — and the agreement was the *symptom*.

| | before the fix (`fd34a44`) | after |
|---|---|---|
| iccce's ISO 4.2.5 black, `USWebCoatedSWOP.icc` | `L* 16,489 806` — **non-conformant** | **`L* 11,772 365`** |
| lcms2 (reimplemented, pin `21c582a`) | `L* 16,571 474` | **unmoved** |
| **the reported divergence** | **`8,166 8×10⁻² ΔE76`** | ★ **`4,799 109 ΔE76` — 58,8× LARGER** |
| **the defect's own magnitude** | — | **`4,717 441 L*`, i.e. 57,8× the divergence it was blamed for** |

**Why the instrument was nearly blind:** the non-conformant return was
`outRamp[first] = MinL`, and **`MinL(lcms2) = MinL(ISO) = 16,489 806`
exactly**. iccce was returning a quantity the oracle *also* computes, so
it landed close to lcms2's answer **for a reason that had nothing to do
with being right**.

> **★★★ A SESSION THAT READ ONLY THE SMALL RESIDUAL WOULD HAVE FILED
> "the two implementations agree well here" — supported by a clean,
> tight, honestly bounded measurement — WHILE THE CODE WAS WRONG.**

**Take four things from it:**

1. **DL-033 — a small residual is evidence of PROXIMITY, not of
   correctness**, and proximity has more than one cause. It is the
   mirror of DL-028 and the more dangerous half: DL-028's failure mode
   announces itself; **this one is silent.**
2. **DL-036 — the authored fixture had ZERO power here.**
   `v4-rgb-mab-chromatic-black.icc`'s `InitialLab` and `outRamp[first]`
   are **both `L* 20`**, so the defect swapped two equal numbers and
   changed nothing. **The vendor profile was the only arm that could
   see — because nobody designed it.** DL-020 is not weakened; its
   boundary is now stated.
3. **A row must state ONE claim.** NC-164a carried a **measured** claim
   (*the defect explains the whole gap* — true) and an **unmeasured**
   one (*fixing it ends the gap* — false) in one sentence, and the
   second inherited the first's authority **by adjacency**. Now split
   into **NC-174** and **NC-175** (§3.25.3).
4. **★ The remaining `4,799 109` is a DEFINITIONAL divergence, not an
   error by either side.** Both return what their own document calls
   `InitialLab`: **ISO 4.2.2.2 means the darkest device vertex,
   neutralised; lcms2 means the perceptual black round trip with chroma
   zeroed.** Rule 7 in its sharpest form, and **this time neither side
   is wrong.**

---

## What is COMPLETE

| Pass | Status |
|---|---|
| **0** scaffold + oracle · **1** colorimetry · **2** parsing · **3** matrix/TRC | **DONE** |
| **4** LUT transforms and intents | **DONE** (2026-08-12) |
| **5** black point compensation | **DONE**; its estimator boundary closed, and **its 4.2.5.4 defect found, corrected (`fd34a44`) and re-measured** |
| **6** performance | **DONE**, gate passing at grid 33 — **but its speedup figure is now WITHDRAWN outright** (§3.27.3) |
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
  exercised once as a scratch probe; **the LUT arm has never been
  reached from a spot.**
- **Any cross-check at all on the spot path.** See Kind 4 item 2.
- **A statement of what a caller should do with a reported
  malformation.** Rule 6 says the parser reports and does not repair;
  **no caller that must keep going has ever exercised that.** That is a
  `pdfce`-side design question and this repository should not answer it
  unilaterally — but it should be asked before the bridge is written.

### Kind 2 — it is blocked on a document nobody here can produce

**These are acquisitions, not tasks. No amount of work in this tree
advances them.**

| What is needed | What it unblocks |
|---|---|
| **IEC 61966-2-1** (purchased or licensed) | **★★ The project's FIRST `published-ground-truth` row for a transform.** There are **none**, across every Pass — **eleventh consecutive filing.** ★ **And today is the sharpest illustration yet**: NA-009's cost is now measured to six figures **with no ground-truth arm at all**, so the project can say the two implementations differ by `4,799 109 ΔE76` and **cannot say which is nearer the truth.** Cheapest route; **nobody has dispatched** |
| **`ICC.1:2010-12` (v4.3)** | **A31 — the ambiguity register's ONLY unverified row.** `parametricCurveType` Table 68 across editions |
| **ICC's published D65→D50 `chad` values** (Annex E.4.2) | the chromatic-adaptation ground-truth hole (**NA-002**) |
| **A published black point for any real profile** *(new, 2026-08-12)* | **NA-009's cost acquiring a ground-truth arm.** There is none for `USWebCoatedSWOP.icc`, and **ISO/CD 18619 is a committee draft** — so the whole of §3.25 is an implementation-cross-check and **must never be promoted** |
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
  ★ Note the distinction Pass 10 must be sized against: **writing
  synthetic profile bytes** (what `tools/gen-profiles` already does) was
  **never out of scope**; **profile creation from measurement data** is
  the thing that was refused. **`iccce-measure` (the CGATS/IT8.7
  reader, `2a2d616`) is the half that never needed hardware and it has
  landed.**

### Kind 4 — the standing debts of work already done. **This is the only actionable list.**

**0. ★★★ CANDIDATE-SEPARATION STATEMENTS ON THE CROSS-CHECK ROWS.**
*(New, and it is the successor to the 4.2.5.4 question that used to sit
in this slot — which is now **ANSWERED, against us**, and closed.)*
For every `implementation-cross-check` row whose two implementations
could compute a **shared intermediate**, state **how far apart the two
candidate answers were.** That, and not the residual, is what bounds the
check's power. **No row in the ledger currently does this.** The
founding instance is the whole of §3.25; **DL-033 states the rule and
does not supply the statements.**

**1. ★★ A regression that would FAIL if the 4.2.5.4 branch regressed,
on the arm that can see it.** `crates/iccce-cmm/src/bpc.rs` **L620–L703**
now carries two tests naming 4.2.5.4 — one asserting *"InitialLab
carried through"*, one asserting the whole triple survives on a
**chromatic** `InitialLab` *(verified — read at the tip)*. **Whether
that discharges the debt is `icc-conformance`'s call.** ★ **What is
certain is that the differential harness is half-blind here**: NC-166
shows the synthetic arm has **zero power**, so **only the `swop` arm is
load-bearing** and a regression would be invisible on the other.

**2. ★★ Commit hygiene — three instances of one mechanism in two days.**
`edce48b` swept in in-progress `gen-profiles`; `aef7566` swept in and
**published `dechk.obj`**; `5cfee17` swept in another agent's
**mid-write `NUMERIC_CLAIMS.md`** *(all reported from a prior filing
that had a shell)*. **Owed: commit with EXPLICIT PATHSPECS, never `-A`
or a bare `.` from the repository root, while any other agent is working
in the tree.** **`dechk.obj` at the repository root** —
`unverified-this-filing`, the tree was not enumerated — with **no
`*.obj`/`*.o` rule in `.gitignore`** at last check. Add the rule, remove
the file, **and decide about history**: it is small and benign, but
***"benign" is a judgement the operator makes about a published
artefact, not one an agent makes for him.***

**3. ★ The cheapest genuine cross-check in the project, skipped for five
filings.** **Resolve a spot into ITS OWN profile's device space and
compare against the entry's stored `nDeviceCoords`** — an expectation
**iccce did not write**, on bytes **iccce did not choose**. The only
such expectation anywhere on the spot path.

**4. ★ Measurements the fixtures make possible and nobody has taken.**
- **The A41 constant's error.** At perceptual, both implementations use
  `L* ≈ 3,1` where `v4-rgb-mab-chromatic-black.icc`'s real black is
  **`L* 20`**. **The instrument exists; the row does not.**
- **A PCSLAB gray fixture** → **NA-008's second arm**, which still has
  **no instrument in existence** and is now **the oldest UNMEASURED
  entry in §4**. Every gray profile in reach is PCSXYZ, and **agreeing
  with lcms2 cannot substitute** — lcms2 makes one of the two choices
  too, so a cross-check is blind by construction.
- **★ A non-zero-black v4 LUT fixture that separates `InitialLab` from
  `outRamp[first]`** — **owed for a NEW reason.** It used to be *"the
  only instrument that can make NA-009's cost measurable"*; that cost is
  now measured without it. **It is now the cheapest way to give the
  authored corpus any power at all on this branch** (DL-036's revisit
  clause).

**5. ★★ Evidence that was never filed, for numbers that are.**
- **★ Enumerate the conformance runner's `skip=3`.** Which three
  records, and why. **A skip is the runner declining to grade, and it is
  invisible in `fail=0`** — the one place a green census can hide
  something. **Eleventh filing; still nobody.**
- **The CI evidence.** CI is **reported** to have run and passed — **by
  a report**. No run URL, no summary, and **no statement of whether the
  Linux job was among what passed**. ★ **Do not let it discharge the
  Linux debt silently.**
- **★ The `iccce bench`-vs-`pass6.rs` harness comparison.** §3.23.4's
  hypothesis — *the two harnesses may time different work, e.g. the
  CLI's per-pixel buffer marshalling* — **was never tested**, and the
  withdrawal of the speedup **does not refute it**. A quantity too noisy
  to publish can still be **systematically** different between two
  harnesses.

**6. ★ Apparatus and hygiene, carried.**
- **`tools/difftest/src/pass5.rs` has no `#[test]` declarations** —
  `unverified-this-filing`.
- **`cargo fmt --check` in `tools/difftest`** *(reported: 109 diffs
  across 15 files)*. **Rule 10's gate is workspace-wide and
  `tools/difftest` is deliberately not a workspace member** (DL-001), so
  `--workspace` has never seen it — and **NC-159 shows the crate has its
  own green 36-test suite the gate also cannot see**. Bring it under the
  gate **or** state the exemption in `CLAUDE.md`.
- **Three real measurements pinned by nothing**: the M3 A/B, the
  gray-destination probe, the Pass 4b re-run.
- **A sweep for the bare *"D.6/D.7"* citation label**, folded into the
  DL-014 audit — **Annex D is INFORMATIVE, and the label is not
  edition-stable.**
- **★ A Linux run of anything at all.** Still nothing, by anyone, ever.

---

## ★★ Numbers that are NO LONGER QUOTABLE — check before restating any of these

| Do not quote | Because |
|---|---|
| **`8,166 8×10⁻² ΔE76`** as the current `swop` black-point divergence | **Superseded by `4,799 109` (NC-165).** It remains correct as *the pre-correction figure* and as **NC-174**, the defect's measured cost |
| **Any speedup figure or range** — `14,4×`, `12–23×`, `28–32×` | ★★ **WITHDRAWN OUTRIGHT** (NC-173, `TOLERANCES.md` §3.6.3(b)). It spans **2,03× within one session at one grid**. **This project does not carry a speedup figure** |
| **A break-even without its grid** | `85 900 px` is **grid 17**; **`≈1,3×10⁶ px` is grid 33** (NC-172). *A break-even without a grid is like a tolerance without units* |
| **`0,076–0,091 Mpix/s`** as the reference arm's band | Superseded by **`0,092–0,099`** over ten runs. ★ **And the old band was never evidence of drift** — it was a four-sample range from one sitting quoted as a machine property. Within a session the reference arm is the **tightest** quantity measured (±4 % vs ±35 %) |
| **`COMPILED_DE` as "derived for grid N"** | ★ **Wrong on the day it was written.** Its derivation population is Pass 4's **341-point CMYK** iccce-vs-lcms2 comparison, and **`pass4.rs` never builds a `CompiledTransform`**. The grid governs the bound's **applicability**, never its derivation (**DL-034**) |
| **T1's and T4's movements as "improvements"** | T1's error bar **did not change** (its effect grew 59×); T4's numerator **did not change** (its rival got 4,03× worse). **DL-035** |
| **Any bare test count** | Three runners, three disjoint populations: **129** (`cargo test --workspace`), **36** (`tools/difftest` units), **142** (the conformance runner). **DL-031** |

---

## Owed work, by agent

### `icc-spec-librarian`

- **★★ `IEC 61966-2-1`** — the first ground-truth row for a transform.
  **Eleventh filing; nobody has dispatched.** ★ Today's re-measure is
  the strongest argument yet: a **committee draft** was the sole arbiter
  of a defect in shipped colour code.
- **★ Whether ISO/CD 18619's `DestinationBlackPoint` and lcms2's
  `cmsDetectBlackPoint` are describing the same quantity at all** — the
  `4,799 109` divergence is now attributed to the two documents meaning
  different things by *one name*, and that attribution is **this
  project's reading, not a sourced finding**.
- **The tier question that decides a ledger CLASS**: is
  `icc__ref__bpc.md` §2/§3 `primary_spec` or `cross_verified_2src`? At
  `primary_spec`, **NC-084/NC-086 become `normative-rule-conformance`.**
- **The forcing-policy question is NOT settled by ISO/CD 18619.** That
  document supplies **estimation**; **NC-100 / DL-022 turn on
  applicability**. **NC-100 stays REPORTED, NOT GRADED.**
- **The clamp question, narrowed**: must the final `B` curves' output be
  clipped to 6.3.4.2's encodable PCS range, and does 10.18's domain bind
  the *evaluator* or only describe the stored samples?
- **Corpus rows owed**: the **M2 correction**; the **trilinear
  override**; the **`IsEmptyLayer` 0,002 threshold**; **A41's
  ΔE2000 = 0,050 201**; **M3's measured magnitude**; the
  branch-dependence correction on the black-point prediction; ★ **and
  4.2.5.4 itself, which §3.24.3 records the corpus did NOT carry when
  the defect shipped.**
- **`A31`**, and **the ITU terms determination** before any BT.709 /
  BT.2100 fetch.

### `icc-conformance`

- **Kind 4 items 0, 1, 3, 4, 5 and 6.**
- **★ A `TOLERANCES.md` §5 row for NA-009 now that its cost EXISTS.**
  The register carried "UNMEASURED" for four filings and that
  justification has expired.
- **A compiled path in the B2A direction** (DL-021 makes it a separate
  question) and **a compiled chain with BPC folded in**.
- **A second machine.** Everything timed here is one Windows box.
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
  Pass 2; **Pass 7 does NOT discharge it.**
- **A fixture whose darkest colorant has chroma above 50** — **only if
  something depends on the answer.** *A fixture built to trigger one
  branch of one clamp is a fixture built to make a point.*

### `icc-engineer`

- **Kind 4 items 1, 2 and 5.**
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

- **★★ The DL-014 citation audit.** Eleventh filing. It decides
  **NC-084's ledger class**, underwrites **DL-024's third
  pre-publication check**, and has a **live defect** to sweep.
  `iccce-color` and `iccce-profile` have **never** been swept. ★ New
  surface: **§3.24 cites ISO/CD 18619 4.2.5.4 verbatim** — confirm the
  corpus now carries that paragraph at the tier the citation implies,
  since **§3.24.3 records that it did not before.**
- **A re-read of README §15**, carried unverified four times.
- **Observed residuals** for Pass 1's rows and for NC-032.
- **★ Re-read anything a claim rests on IMMEDIATELY before asserting
  it** — including whether the document you are about to *correct*
  actually contains the thing you are correcting. ★ **That check paid
  out on 2026-08-12**: a dispatched correction about "reference-arm
  drift" turned out to have **no target** — no document here ever
  carried the claim (§3.27.5). **Fifth instance of the dispatch and the
  tree disagreeing, and the first caught before the filing.**
- **★ The librarian has had no shell for three consecutive filings.**
  Ask, per session; **never inherit.**
