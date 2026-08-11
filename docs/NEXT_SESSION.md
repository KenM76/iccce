# NEXT SESSION — start here

**Written 2026-08-11 by `icc-librarian`, at the close of Pass 0.**
Overwrite this file once acted on.

Read order: this file → `docs/ROADMAP.md` (Pass 0's completion record and
Pass 1's done-when) → `docs/ARCHITECTURE.md` §5 (five decisions were
logged on 2026-08-11 and they constrain Pass 1) → `docs/TOLERANCES.md`
§1–§2 → `docs/SESSION_LOG.md` (2026-08-11).

---

## ★★★ The one thing only the operator can do — ~2 minutes

**Download `ICC.1-2022-05.pdf` in a browser and save it to:**

```
D:\Dev\Rag-Specialized\ICC_Spec\_sources\ICC.1-2022-05.pdf
```

It is offered free at
`https://archive.color.org/specification/ICC.1-2022-05.pdf`.

**Why an agent did not do it.** color.org's Terms of Service (effective
2026-01-01) prohibit *"using any robot, spider, or other automated device
to access the Services for any purpose, including monitoring, copying, or
training artificial intelligence or machine learning models, without
prior written consent"* — a clause that names AI/ML explicitly.
`icc-spec-librarian` stopped at the three pages needed to read the terms
and retrieved nothing else. A human downloading a document a body
publishes for free is entirely outside that clause. Full record:
`LEGAL.md` §2.2; decision: `ARCHITECTURE.md` **DL-002**.

**What it unblocks.** Roughly **15 UNVERIFIED rows** in the corpus's
ambiguity register, and **the whole required/optional tag column**. The
corpus is currently built from cross-verified C headers, which encode
signatures, offsets and enumerations exactly and **encode prose
requirements not at all**. Consequence, in the corpus librarian's own
words: **a parser is defensible on this evidence and a validator is
not.**

Until it lands: **no claim in this project may cite an ICC.1 clause
number.** Cite corpus filenames, as `iccce-profile` already does.

---

## Where the project actually is

**Pass 0 is done (2026-08-11).** Four crates, a header/tag-table parser
that reports and does not repair, a CLI that inspects a real profile, an
lcms2 oracle pinned by commit hash and demonstrated on two system
profiles, a 21-file standards corpus, and a tolerance budget with one
provisional anchor and no tolerances in it.

**What is NOT true, and is easy to assume:**

- **No colour maths exists.** `iccce-color` and `iccce-cmm` are stubs.
- **Nothing in iccce has been compared to anything.** Every number
  recorded so far is either lcms2's own output or a byte-level read of a
  profile. There is **no measured claim about iccce's accuracy**, which
  is why `docs/NUMERIC_CLAIMS.md` does not exist yet.
- **The Rust difftest harness does not exist.** Nothing drives `transicc`
  programmatically (`tools/difftest/README.md` §10).
- **The Linux lcms2 build has never run** (§7 of the same file), so CI
  cannot run a difftest on Linux even once one is written.
- **CI has never been observed to run.** The workflow file exists and was
  read; its execution history was not checked by anyone.

---

## Pass 1 — colorimetry. The whole project's credibility rests here.

`iccce-color`: XYZ, xyY, Lab, LCh; standard illuminants and observers;
Bradford (and possibly von Kries) adaptation; ΔE 76/94/CMC/2000.
**No ICC anywhere in this crate. It depends on nothing.**

`ROADMAP.md`'s done-when: *every function matches published reference
values.* The expected values come from the literature, **never from the
code** — a test whose expectation came from the code under test detects
change, not error.

### The good news: the ground truth is already sitting in the corpus

| What | Where | Standing |
|---|---|---|
| **Sharma, Wu & Dalal (2005) — all 34 ΔE2000 test pairs, verbatim** | `ICC_Spec\cie\cie__ref__delta_e.md` | **Real published ground truth.** *Color Research & Application* 30(1):21–30, DOI `10.1002/col.20070`. The set exists precisely because ΔE2000's hue-angle discontinuities are easy to get subtly wrong. Grade against it at **~1×10⁻⁴ arithmetic agreement** — this is an *implementation* test, so DL-004's perceptual anchor is **irrelevant** to it. |
| **D50 = (0.9642, 1.0000, 0.8249)** | `ICC_Spec\cie\cie__ref__colorimetry_core.md` | **Cross-verified, 2 independent sources** (ICC `IccUtil.cpp`, lcms2). Use ICC's 4-figure value **everywhere** — mixing it with a higher-precision D50 (0.96422…0.82521) in different parts of the pipeline gives a small uniform untraceable cast. |
| **Bradford forward matrix `M_A`** | `ICC_Spec\cie\cie__ref__chromatic_adaptation.md` | **Cross-verified, 2 independent sources** (lcms2 `cmswtpnt.c`, R `spacesXYZ`). Rows are cone responses, columns are XYZ; **row-major, applied to a column vector**; `M = M_A⁻¹ · D · M_A`. The order and the transposition are both easy to invert and both produce *plausible* output. Free sanity check: with source == destination the composed matrix must be **exactly** the identity. |

**Cite these to code, not to CIE.** A doc comment must read *"D50 =
(0.9642, 1.0000, 0.8249) per ICC `IccUtil.cpp` and lcms2, cross-verified;
CIE 15 not sourced"* — **not** *"per CIE 15"*. Citing a paywalled
document nobody read is a failure this project family has already paid
for once.

### The gaps that will bite in Pass 1 — read these before writing code

- **von Kries is a placeholder. DO NOT USE the digits in the corpus.**
  `cie__ref__chromatic_adaptation.md` records the Hunt–Pointer–Estévez
  matrix **not sourced this session**, and flags a second problem: "von
  Kries" names *both* the general diagonal-in-cone-space method (of which
  Bradford is an instance) *and* that specific matrix. Settle which is
  meant before implementing anything called von Kries.
- **The Bradford inverse `M_A⁻¹` is NOT SOURCED.** Both reference
  codebases compute it numerically. Published digits circulate; the
  corpus records them and explicitly says **do not use them**. **Invert
  the sourced forward matrix at runtime, in `f64`.**
- **ΔE94 and ΔE CMC are not transcribed.** The lcms2 functions were
  located (`cmsCIE94DeltaE`, `cmsCMCdeltaE`) but not extracted, and **no
  published worked examples were obtained**. So a ΔE94/CMC implementation
  can only be **cross-checked against lcms2** — a strictly weaker claim
  than ground truth, and it must be **labelled as such** in the test, the
  doc comment and the ledger. ΔE2000 is the only metric with real ground
  truth; that is a further reason to grade the suite in ΔE2000.
- **sRGB is single-source.** `ICC_Spec\iec\iec__s__srgb.md` is
  `impl_crosscheck` tier — every value comes from lcms2 alone, because
  IEC 61966-2-1 is paywalled and was not obtained. A second independent
  source is a recorded gap (ITU-R BT.709 is free from ITU-T and was not
  fetched). **Do not present sRGB constants as cross-verified**; they are
  not, unlike D50 and Bradford.
- **The `f(t)` breakpoint has a genuine source conflict — ambiguity
  A11.** Exact rational `(24/116)³` (lcms2) versus decimal `0.008856`
  (ICC), and ICC's own forward and inverse thresholds are mutually
  inconsistent to ~4×10⁻⁷. **iccce's recorded choice is the rational
  form**, and rule 4 requires that the choice be *stated and measured*:
  cost is ~10⁻⁵ in `L*`, which matters for round-trip invariants and not
  for colour. Settling it properly needs CIE 15 / ISO 11664-4 (paywalled,
  not obtained).
- **No clause number in the corpus's CIE files is real** — the CIE
  documents were never obtained. The files say so in their own
  frontmatter. Read the frontmatter before citing anything.

### Create `docs/NUMERIC_CLAIMS.md` in this Pass

It does not exist yet, deliberately — Pass 0 produced no measured colour
claim, and an empty ledger invites a first row that is not a measurement.
**Its first row should be the ΔE2000 implementation result against the
Sharma 34 pairs.**

Every row carries: what was compared, at what tolerance, the measured
value, **the corpus and its coverage**, the commit, and the date. Coverage
is part of the claim — "verified on the 34 Sharma pairs" never becomes
"verified". The ledger exists so that when a later Pass changes something
upstream, the claims it invalidates are **findable** rather than quietly
stale.

### Method reminders that are load-bearing here specifically

1. **A wrong colour looks exactly like a right one.** Nothing about a
   3 ΔE error announces itself.
2. **Never write colour maths from memory.** Dispatch
   `icc-spec-librarian`; cite the corpus file in the doc comment.
3. **Disagreement with lcms2 is a finding, not a failure** — settle it
   from specification text and write the outcome down either way.
4. **Tolerances are justified, not tuned.** When a test fails, the first
   question is whether the code is wrong. `TOLERANCES.md` §4 is
   append-only, and widening a tolerance is an event that leaves a
   record.

---

## After Pass 1

`ROADMAP.md` Pass 2 (full profile parsing) is the natural next step and
the Pass 0 parser is its foundation. Two Pass-0 artefacts are worth
carrying forward before the LUT work in Pass 4:

- **DL-005** — v2 legacy Lab encoding must be tested by **exact-value
  invariants, not ΔE**: getting it wrong costs ≈0.3–0.5 ΔE, *below* the
  1.0 anchor, so a ΔE-graded test passes while the encoding is wrong.
- **Ambiguity A16** — the specification is **SILENT** on interpolation
  between CLUT grid points, and trilinear versus tetrahedral differ by up
  to ~1 ΔE, at or above perceptibility. **Two conformant CMMs can produce
  visibly different colour from the same profile and neither is wrong.**
  iccce must state its choice and measure the difference (rule 4).

---

## Decisions already made — do not re-litigate

From `NEXT_SESSION.md`'s previous edition, all still standing:

- **MIT**, dependencies permissive, **publishing is the operator's act.**
- **`iccce-color` depends on nothing** and contains no ICC.
- **The parser reports, it does not repair.**
- **No profile creation, no iccMAX execution, no display calibration.**
  Identify iccMAX and refuse it by name.
- **lcms2 is the oracle, never a dependency** — subprocess only.
- **The `pdfce` bridge is built in `pdfce`.** `iccce` must not know what
  a PDF is.

Added 2026-08-11, in `ARCHITECTURE.md` §5 — read them there, in full:

- **DL-001** — lcms2 pinned by **commit hash** (the tag is lightweight
  and therefore mutable); GPL-3.0 plugins excluded in three layers.
  Moving the pin is a **licence event**, not a version bump.
- **DL-002** — the corpus has **no `primary_spec` tier** and cites code,
  not clauses, until the PDF above lands.
- **DL-003** — duplicate tag signatures: keep both, consumers take the
  first, report the duplicate (ambiguity A13).
- **DL-004** — the 1.0 ΔE2000 anchor is a conservative **design choice**,
  ⚠ provisional; anything derived from it inherits the ⚠.
- **DL-005** — v2 Lab encoding tested by exact-value invariants.

---

## Open questions for the operator — (a)–(d), still standing

Listed in `ROADMAP.md`; repeated because they shape early work.

- **(a) Separate repository, or alongside `pdfce`?** — **de facto
  answered, not formally decided.** `D:\Dev\iccce` is its own git working
  tree and the workspace manifest declares
  `repository = "https://github.com/KenM76/iccce"`. That is a declaration
  in a file: **it is not evidence that the remote exists or that anything
  has been pushed**, neither of which was checked. What still needs an
  answer is whether that remote should be public — which is (d).
- **(b) How far into HDR** (BT.2100, PQ/HLG)? Real work; only matters if
  something needs it.
- **(c) Is a profile *creator* ever wanted?** Currently a firm no.
- **(d) Publish to crates.io?** The Rust ecosystem lacks a general MIT
  CMM — an argument for, and a maintenance commitment. **Nothing may be
  pushed, tagged or released without an explicit current go-ahead**
  (rule 9).

---

## The agents

- **`icc-engineer`** — lead. Be this agent if orchestrating.
- **`icc-spec-librarian`** — the standards corpus. Dispatch for *every*
  sourcing question. Owns the ICC.1 unblock above.
- **`icc-conformance`** — the oracle, the fixtures, the tolerance budget.
  Separate from the engineer on purpose: whoever builds a transform is
  the worst person to also decide what counts as proof it works.
- **`icc-librarian`** — ROADMAP, decision log, session log, and the
  numeric-claims ledger. **No shell** — a dispatch to it must carry its
  evidence.

Dispatch them freely; no permission is needed to dispatch an agent to
read, analyse or draft.

---

## Lessons carried from the sibling project

Written here because they were expensive there and are all more dangerous
in a numeric domain:

- **Verify in the running thing, not in the code you read.** A grep for
  direct writes cannot see a shared helper — that produced a confident
  bug report about a defect that did not exist.
- **A test asserting code shape can certify the bug it was written to
  catch.** Assert measured output. Prefer assertions that a thing is
  PRESENT: an absence-assertion goes quiet when it loses its subject and
  keeps passing while checking nothing.
- **A corpus paraphrase is not spec text.** Citing a RAG's own
  reconstructed heading as though it were the standard produced the right
  answer by an invalid route — worse than being wrong, because the method
  is invisible. **This is the live hazard right now**, given DL-002.
- **Do not assert unmeasured facts about the environment.** One false
  sentence about a repository's own state survived a day.
- **An unused capability is not a feature.**
