# iccce — session log

**Append-only.** A session's entry is written once and not edited
afterwards; if a later session discovers that an earlier entry was wrong,
it says so **in its own entry**, naming the entry it corrects. An edited
history cannot be used as evidence, which is the only thing a history is
for.

Owned by `icc-librarian`, who **has no shell** — every measured statement
below arrived in a dispatch carrying its evidence, or was read out of a
file in the working tree. Statements are labelled with which:

| Label | Means |
|---|---|
| **verified** | The librarian read it, in the live source, this session. |
| **reported** | An agent ran it and carried the result. Not re-run here. |
| **unverified** | Neither. Recorded as an open question, never as a fact. |

Entry format: date, what changed, what was measured, what was decided,
and what the next session must not assume.

---

## 2026-08-11 — Pass 0: scaffold, parser, oracle, corpus

**First working session of the project.** The tree entered the day
containing a plan and an agent roster and no code.

### What was built

- **Workspace** — four crates per `ARCHITECTURE.md` §1
  (`iccce-color`, `iccce-profile`, `iccce-cmm`, `iccce-cli`),
  `unsafe_code = "deny"` workspace-wide, lossy-cast clippy lints at
  `warn`. `tools/difftest` is **deliberately not a workspace member**, so
  the shipping crates cannot link the oracle even by accident.
  *(verified — `Cargo.toml`, read.)*
- **CI** — `.github/workflows/ci.yml` builds and tests on
  `ubuntu-latest` **and** `windows-latest`, with `fmt` + `clippy` on
  Linux and `RUSTFLAGS: -D warnings` in CI only. *(verified — file read.
  **Whether it has ever run is unverified**; no run history was checked
  and this librarian cannot check one.)*
- **`iccce-profile` Pass 0 parser** — 128-byte header, tag table,
  malformation reporting, iccMAX identified and **refused by name**,
  hostile `tagCount` bounded *before* allocation. Every layout cites a
  corpus file (`icc__s__header.md`, `icc__s__tag_table.md`,
  `icc__s__number_encodings.md`) and **no ICC.1 clause number** — see
  DL-002. *(verified — `lib.rs`, `tag_table.rs`, `diag.rs` read.)*
- **`iccce-cli inspect`** — prints header, tag table and every
  malformation, one `key: value` per line, as a stable diff surface
  rather than a human UI. *(verified — `main.rs` read.)*
- **The oracle** — lcms2 pinned, built (MSVC), and demonstrated on real
  profiles. *(reported by `icc-conformance`; the recorded evidence in
  `tools/difftest/README.md` §6–§9 was verified as present and
  internally consistent.)*
- **The corpus** — 21 files at `D:\Dev\Rag-Specialized\ICC_Spec\`.
  *(verified — 21 `.md` files enumerated; contents of the
  chromatic-adaptation, ΔE, colorimetry-core, sRGB, divergence and
  ambiguity-register files read.)*

### Pass 0's done-when, met

1. `iccce inspect "…\sRGB Color Space Profile.icm"` → header (`'Lino'`
   CMM, v2.1.0, `mntr`/RGB/XYZ), 17 tags, 0 malformations, with
   `rTRC`/`gTRC`/`bTRC` all at offset 1084. *(reported.)*
2. `transicc` invoked on the same profile → `99.9988 0.0188 −0.0173` for
   white at intent 1, with the full command line recorded.
   *(reported; the record is verified present in `difftest/README.md`
   §8.2.)*

Filed in `ROADMAP.md` as the Pass 0 completion record, **without a commit
hash** — the work was uncommitted when this was written and the commit is
the engineer's act. The record says so and asks whoever commits to fill
it in.

### Gate results — carried, not measured here

`cargo test --workspace` 14/14 pass; `cargo fmt --check` and
`cargo clippy` clean. *(reported by `icc-engineer`, run on this machine.)*
The one thing checkable from the tree without a shell: **14 `#[test]`
declarations exist** — 8 in `crates/iccce-profile/src/lib.rs`, 6 in
`src/num.rs`. *(verified.)* That is a count of tests declared. It is
**not** a count of coverage and **not** a pass result; it is recorded
only because it is consistent with the reported figure.

### Findings that changed decisions

- **lcms2 is not uniformly MIT.** Core and headers are verbatim MIT;
  `plugins/fast_float` and `plugins/threaded` are **GPL-3.0-or-later**,
  stated by upstream in `plugins/README.1ST`. A licence badge would have
  said "MIT" and been incomplete. → **DL-001**.
- **The `lcms2.19.1` tag is lightweight**, therefore mutable, therefore
  not a pin. The commit hash `21c582a…` is the pin, and
  `fetch-lcms2.sh` hard-fails on mismatch. → **DL-001**, and a
  cross-project RAG lesson (below).
- **color.org's ToS blocks automated retrieval**, naming AI/ML training
  explicitly, so the ICC.1 PDF was **not** downloaded — while the site's
  own `robots.txt` permits the specification index. The two point
  opposite ways; the prose contract was taken as binding and the conflict
  recorded rather than resolved silently. → **DL-002**.
- **A2B0 and A2B2 share one tag-table offset (432) in
  `USWebCoatedSWOP.icc`**, so perceptual and saturation are
  byte-identical through that profile. Written into
  `difftest/README.md` §8.4 so it is never misdiagnosed as an engine
  bug at 2 a.m. *(reported, with the tag-table dump.)*
- **The v2 legacy Lab encoding costs ≈0.3–0.5 ΔE — below the 1.0
  anchor**, so a ΔE-graded test would pass while the encoding is wrong.
  → **DL-005**.
- **Duplicate tag signatures**: specification SILENT, observed in the
  wild, so the parser had to choose and the choice had to be visible.
  → **DL-003**.

### One measured verification closed

The corpus's **derived** illuminant hex for D50 (`0xF6D6` / `0xD32D`) was
confirmed byte-for-byte against the system sRGB profile: bytes 68–79 =
`0000F6D6 00010000 0000D32D`. *(reported, with the `xxd` output.)* This
promotes a value the corpus had *derived* to one *observed in a real
file* — a genuine strengthening, and worth noting that it is
**observation of one profile**, not a published constant. A parallel
dispatch was updating the corpus file; **this librarian did not verify
whether that edit landed**, and a later session should not assume it did.

### Deliberately NOT created: `docs/NUMERIC_CLAIMS.md`

Per `NEXT_SESSION.md`, the numeric-claims ledger is created **with the
first measured claim**. Pass 0 produced **no measured colour claim**:
`iccce-color` and `iccce-cmm` are stubs, no transform exists, and nothing
in iccce has been compared to anything. The numbers this session
produced are lcms2 smoke-test outputs (cross-check values from an
implementation, recorded in `difftest/README.md` §8, and explicitly not
transplantable into a unit test) and a byte-level hex confirmation —
neither is a claim about iccce's own accuracy.

**An empty ledger is worse than no ledger**: it invites the first entry
to be something that is not a measurement, and it makes "nothing has been
measured yet" look like "nothing has been filed yet." The ledger gets
created by Pass 1, with the ΔE2000 arithmetic-agreement result against
the Sharma 34 pairs as its first row.

### Filed this session

| Where | What |
|---|---|
| `ROADMAP.md` | Pass 0 marked done (2026-08-11) with an evidence-bearing completion record, a `NOT delivered` list, and a dated annotation on open question **(a)**. Plan text unchanged. |
| `ARCHITECTURE.md` §5 | **DL-001** … **DL-005**, appended to a previously empty log. |
| `SESSION_LOG.md` | Created — this entry. |
| `NEXT_SESSION.md` | Overwritten for Pass 1. |
| `D:\dev\rag\rust\` | `a_lightweight_git_tag_is_a_mutable_label_not_a_pin.md` + index entry. |

Not touched, by instruction and by ownership: `LEGAL.md`,
`TOLERANCES.md` (owned by `icc-conformance` / `icc-spec-librarian`), and
the corpus itself.

### Left for the next session to not assume

- **`README.md` §Status still says "Nothing is built."** It also says of
  lcms2's licence *"Verify that before relying on it"* — which was done
  this session (`LEGAL.md` §4). Both are stale. `README.md` is not the
  librarian's file; flagged for the engineer.
- The Linux build of lcms2, and therefore Linux CI's ability to run any
  difftest, is **unproven** — the script has never executed.
- No `primary_spec` tier in the corpus. **No claim in this project may
  cite an ICC.1 clause number** until DL-002's blocker clears.

---

## 2026-08-11 (continuation) — the operator answers the scope questions, and the ICC.1 blocker clears

Same calendar day as the entry above and the same session, filed
separately because it is a distinct event: Ken answered the four open
questions in `ROADMAP.md`, one of which **reverses a scope position this
project had put in writing**.

### What the operator said

The engineer put two things to him — *"(1) download the ICC.1 PDF; (2)
the open scope calls: HDR depth (b), profile creator (c — currently a
firm no), crates.io (d)"*. Ken replied, in full:

> **"1 is done. 2. do all."**

That is the complete text of the operator's decision, as carried in the
dispatch. *(reported — this librarian did not observe the exchange.)*

**Everything filed today rests on one interpretation of it**, and the
interpretation is recorded as one everywhere it appears: **"do all" is
read as *adopt all three of (b), (c), (d)*, and that reading is the
engineer's.** It is the only reading the sentence plausibly carries — the
items were presented to him as a list, and (c) was presented *with* its
"currently a firm no" attached, so adopting it is what reversing it means
— but a terse operator instruction expanded into three decision-log
entries is precisely where a librarian either labels the expansion or
silently launders it into the record. **The operator supplied no scope
depth, no priority, no schedule, and no per-item rationale.** None is
attributed to him in any document filed today; every rationale in
DL-007…DL-009 is the project's own reasoning and is revisable without
going back to him.

### The one thing measured here

**`D:\Dev\Rag-Specialized\ICC_Spec\_sources\ICC.1-2022-05.pdf` exists.**
*(verified — this librarian listed that directory; it previously
contained only `README.md`.)* That is the whole of what was measured.

Everything else about the file is **reported**: the 11:12 retrieval time,
that it was a manual browser download, and that it is the ICC.1:2022
specification. **Its size, hash, page count and actual identity have been
checked by nobody.** Those belong to `icc-spec-librarian`, which was
dispatched in parallel to ingest it and **owns `LEGAL.md` §2 this
session**. **Whether that ingest landed is unverified here** and a later
session must not read DL-006 as evidence that it did.

**The DL-002 citation rule is therefore recorded as still standing.** The
prohibition was never about a file being present on a disk — it was about
there being no sourced clause text to cite, and a PDF nobody has read is
not a citable source. It lifts when `icc-spec-librarian` files DL-002's
successor, and not before.

### The reversal, and why it got the most care

**`README.md` said, in writing, under *"Out of scope, deliberately — say
no now rather than drifting"*: "Profile creation from measurement data.
That is a profiler, a different product, and it needs measurement
hardware to validate."** *(verified — read in the live source before
editing.)* `NEXT_SESSION.md` carried the same no in its
do-not-re-litigate list. **The operator has reversed it.**

Two things were done deliberately, and both are the point of having a
decision log at all:

1. **The old position is quoted, not deleted.** DL-008 reproduces the
   README's exact wording and its rationale, and `README.md` now says the
   item **moved** from out-of-scope to future scope on 2026-08-11 rather
   than reading as though profile creation had always been planned. A
   scope statement that quietly contradicts an earlier one is worse than
   either version of it.
2. **The rationale survives the reversal.** "It needs measurement
   hardware to validate" was an engineering fact, not a preference, and
   an operator's yes does not make it false. It is carried forward as
   Pass 10's precondition: **a profiler's output is a claim about a
   physical device, and neither self-consistency nor agreement with
   lcms2 can tell you whether the profile describes the printer.** lcms2
   is specifically useless as an oracle here — it would confirm that a
   profile we wrote is parseable and self-consistent, which is exactly
   the reassurance a *wrong* profile also produces. Round-tripping a
   profile through its own inverse is the canonical test whose expected
   value came from the code under test (invariant §3.6). Pass 10 must
   name a ground truth that is not iccce before any of it is called
   correct.

Recorded with it, because the two will otherwise be conflated: **writing
synthetic profile bytes whose contents are known by construction — which
`tools/gen-profiles/` already implies, for fixtures — needs no
measurement and was never out of scope.** What was refused is profile
creation *from measurement data*.

### Also filed

- **HDR (Pass 9)** — transfer functions and primaries only; tone
  mapping, gamut mapping and dynamic metadata are explicitly outside it,
  so the boundary is a decision rather than something that grows. Named
  the conceptual hazard up front (ICC's PCS is media-relative; PQ is
  absolute, HLG scene-referred — connecting them requires a stated
  reference-luminance choice, which is an approximation under rule 4 and
  must be measured). **Nothing in that section is sourced from the ITU-R
  documents; nobody here has read them**, and the section says so.
- **crates.io (DL-009)** — recorded as *intent*, with its own limit
  written in the same entry: **intent is not authorisation**, rule 9
  unchanged. That pairing is deliberate. A recorded intent to publish is
  exactly the artefact a future agent could mistake for standing
  approval of a side-effecting act.
- **The ITU retrieval caveat.** The dispatch reported `itu.int` downloads
  as a legitimate route. That is a **claim about a third party's terms**,
  and it is recorded as one, to be checked by `icc-spec-librarian` before
  anything is fetched. DL-002 exists because "the file is free" looked
  like "automated retrieval is permitted" at color.org and was not.

### Filed this session (continuation)

| Where | What |
|---|---|
| `ARCHITECTURE.md` §5 | **DL-006** (DL-002's trigger fired — records the event, decides nothing), **DL-007** (HDR in scope), **DL-008** (profile creation reversed), **DL-009** (crates.io intended). Appended; DL-001…DL-005 untouched. |
| `ROADMAP.md` | **Pass 9** (HDR) and **Pass 10** (profile creation) added; a **Publication — crates.io** section; open questions (a)–(d) now all carry dated answers **appended beneath the original question text, which was not rewritten**; the ICC.1 unblock recorded with its verified/reported split. **No existing Pass renumbered** — the numbering note in that file says why Pass 9's number is filing order, not schedule order. |
| `README.md` | Profile creation removed from the "Out of scope, deliberately" bullet list and replaced by a dated **"Moved out of out-of-scope"** subsection that quotes the old wording, points at DL-008, and carries the validation problem forward. HDR and crates.io noted. |
| `NEXT_SESSION.md` | The starred operator-action section replaced by a **CLEARED** section listing what is verified, what is reported and what must not be assumed; DL-006…DL-009 added to the decisions list; the "no profile creation" bullet struck through and corrected in place; open questions marked answered; a **Pass 1 status caveat** added. |
| `SESSION_LOG.md` | This entry. |

Not touched, by ownership: `LEGAL.md` (`icc-spec-librarian` owns §2 this
session), `TOLERANCES.md`, the corpus, and `docs/NUMERIC_CLAIMS.md` —
which **still does not exist**, correctly: nothing measured reached this
librarian today. *(verified — `docs/` enumerated.)*

**Nothing was committed.** Instructed not to, and committing is the
engineer's act in any case. Every claim above about the *repository's*
state is therefore limited to what was read out of files in the working
tree; **no git command was run, by an agent that has no shell.**

### Left for the next session to not assume

- **That Pass 1 is or is not built.** `icc-engineer` reported it was
  implementing Pass 1 in-session, in parallel with this filing. **No Pass
  1 work was seen, measured or verified by this librarian**, and no
  completion record was filed. `NEXT_SESSION.md` was phrased to be
  corrected by a later filing rather than claiming a status it cannot
  know. **A later filing that contradicts it is right.**
- **That the ICC.1 ingest happened.** The file exists; the corpus's use
  of it is unverified. DL-002's clause-citation prohibition stands until
  its successor entry is filed.
- **That "do all" meant anything beyond (b), (c), (d).** It was an answer
  to a three-item list. It is not a general authorisation, and it is
  emphatically not a go-ahead to push, tag, release or publish.

---

## 2026-08-11 (Pass 1) — colorimetry, the first measured claim, and the ledger

**Third entry of the same calendar day and the same session.** The
previous entry closed by saying the next session must not assume Pass 1
was or was not built, and that *"a later filing that contradicts it is
right."* **This is that filing.** Pass 1's core was implemented by
`icc-engineer` in-session and is now on the record.

### What was built

All in `crates/iccce-color/src/`. **Every file below was read in the
live source by this librarian** — the dispatch's account of what changed
is a claim like any other. *(verified.)*

- **`mat3.rs`** — 3×3 `f64` matrix: `mul`, `apply` (row-major, column
  vector), and a **runtime `inverse`** by adjugate/determinant. The
  inverse exists at runtime for a sourcing reason, not a numerical one:
  the corpus marks published Bradford-inverse digits **NOT SOURCED** and
  directs inversion of the sourced forward matrix. Singularity is
  `det == 0.0` **exactly** — an epsilon would be a tuned number with no
  citation.
- **`illuminant.rs`** — D50 as the ICC 4-figure triple
  (0.9642 / 1.0000 / 0.8249), used **everywhere** per the corpus's
  mixing-precision warning; D65 as a **chromaticity** (0.3127 / 0.3290),
  single-source, exposed as chromaticity so the XYZ derivation stays
  visible rather than an unsourced triple being baked in. Each constant
  carries its evidence tier **on the constant**, because they are not
  equal.
- **`xyz.rs`** — XYZ ↔ xyY with **divide-by-zero guards both reference
  codebases lack**: black has no chromaticity and returns `None`.
- **`lab.rs`** — XYZ ↔ Lab ↔ LCh, carrying **iccce's first named
  deviation from normative specification text** (below).
- **`adapt.rs`** — the von Kries **method** with **Bradford** cones,
  `M = M_A⁻¹ · D · M_A`, degenerate whites refused. von Kries **HPE** and
  **CAT02** deliberately absent (placeholder marked DO NOT USE;
  paywalled).
- **`delta_e.rs`** — ΔE76 and CIEDE2000 (`kL/kC/kH` explicit, plus a
  `k = 1` wrapper). lcms2's `180.000001` branch epsilons preserved
  **verbatim and deliberately** — they are what the Sharma hue-
  discontinuity pairs exist to test. ΔE94 and CMC deliberately absent.

### The one thing measured — and it is the first of its kind here

**CIEDE2000 agrees with all 34 pairs of Sharma, Wu & Dalal (2005) within
1×10⁻⁴**, at `kL = kC = kH = 1`. *(reported — `cargo test` on this
machine, Windows 11 Pro / MSVC; the assertion, the tolerance and the 34
transcribed pairs were read in `delta_e.rs` by this librarian —
verified.)*

That is **the first genuinely measured numeric claim in this project's
history**, and it is *published ground truth*, not a cross-check: the
formula transcription came from lcms2, but the expectation came from the
paper. The dataset is adversarial by construction — pairs 1–6 catch an
omitted `R_T` cross term, 9–16 sit on the hue-angle discontinuity, 33–34
are very dark — and the test runs **all 34**, because cherry-picking
defeats the design.

**Everything else Pass 1 asserts is an arithmetic identity**, and the
entry says so rather than letting a green suite imply more: Bradford
`src == dst` → identity within 1×10⁻¹⁴; source white → destination white
within 1×10⁻¹²; D65→D50→D65 within 1×10⁻¹² on one sample vector;
XYZ→Lab→XYZ within 1×10⁻¹² on both branches of `f`; white → `L* = 100`
and `Y = 0` → `L* = 0` **exact**. Identities detect drift and structural
error. **They cannot detect a consistently wrong constant** — a round
trip through a wrong white point round-trips perfectly.

### The deviation, named

`lab.rs` uses the exact rational `(24/116)³` / `24/116` form for `f`.
After the ICC.1:2022 ingest, corpus ambiguity **A11** is resolved as
**delegated**: clause 6.4 says conversions *"shall use the equations of
the form specified in ISO 13655"* and does **not** define `f(t)` — while
writing the breakpoint as the decimal `0,008 856` in its own normative
sentence. ISO 13655 is the authority, is paywalled, and **has not been
obtained.**

So iccce's choice is now **a stated deviation from ICC.1's printed
normative constant**, not a pick between disagreeing implementations —
and whether it deviates from the *delegated authority* is **unknown**,
which is stated rather than resolved. Reason for deviating: the rational
form makes `f` and `f⁻¹` exact mutual inverses, which the decimal form
cannot be — **ICC's own reference code demonstrates the inconsistency**
(forward and inverse thresholds disagree by ~4×10⁻⁷). **Cost: ~10⁻⁷ in
`f`, ~10⁻⁵ in `L*` — an analytic bound from the corpus, NOT an iccce
measurement**, and it is labelled that way in all three places it now
appears. → **DL-010**, `NUMERIC_CLAIMS.md` **NA-001**.

### ★ A test caught an error in the *corpus*

The D50-chromaticity consistency test **failed on first run.** Per rule 5
the arithmetic was checked before the code was blamed — and the corpus
was wrong. `cie__ref__colorimetry_core.md` derives D50's chromaticity as
`0.34567 / 0.35850`; those are the chromaticities of the
**high-precision** D50 (0.96422 / 1 / 0.82521), **not** of the 4-figure
ICC triple the same file instructs the project to use everywhere. **The
corpus's own derivation committed the mixing-precision trap that the same
section warns about.** Correct derivation from the sourced triple:
`0.9642 / 2.7891 = 0.345703`, `1 / 2.7891 = 0.358539`. *(Arithmetic
independently checked here.)*

This is worth a sentence because it is **the verification loop running in
the direction nobody plans for.** The corpus is built to check the code;
here a code test checked the corpus and won. Two things follow. A value
the corpus marks **DERIVED** is *a calculation someone did*, carrying the
same error rate as any other calculation — it is not the same kind of
object as a sourced value. And the mixing-precision trap is real enough
that **the document warning about it fell into it.**

A parallel dispatch went to `icc-spec-librarian` to correct the corpus
file. **As of this filing the erratum is still present** —
`cie__ref__colorimetry_core.md` line 60 still reads `0.34567` / `0.35850`
*(verified — grepped this session)*. **A later session must not assume
the fix landed.** This librarian did not touch the corpus.

### Gate results — carried, and what is checkable without a shell

`cargo test --workspace` **35 green**, `fmt` and `clippy` clean.
*(reported by `icc-engineer`.)* Checkable from the tree: **35 `#[test]`
declarations exist** — `mat3.rs` 3, `xyz.rs` 4, `lab.rs` 5, `adapt.rs` 5,
`delta_e.rs` 4 (21 in `iccce-color`), plus `iccce-profile/src/lib.rs` 8
and `num.rs` 6 (14). *(verified — counted.)* **A count of tests declared
is not a count of coverage and not a pass result**; it is recorded only
because it is consistent with the reported figure.

### The corpus position has changed, and one Pass 0 statement is now stale

**The ICC.1:2022 ingest has landed.** *(verified — corpus files read:
`index.md`, `icc__s__pcs_encoding.md`, `icc__ref__v2_v4_divergence.md`,
`icc__ref__ambiguity_register.md`, `icc__ref__spec_defects.md`,
`cie__ref__chromatic_adaptation.md`, `LEGAL_NOTE.md`.)* Files now carry
`evidence: primary_spec`, real clause numbers, verbatim normative
quotations, per-extractor agreement records, and the required/optional
tag material a C header cannot encode.

**Consequences recorded today:**

- **Pass 0's completion record says, quoting DL-002, "a parser is
  defensible on this evidence and a validator is not." That is now
  stale.** The record is left exactly as written — this log and a dated
  annotation under `ROADMAP.md` Pass 2 are how it gets corrected. **On
  the present evidence a validator is defensible.**
- **Bradford is now primary-sourced** from **ICC.1:2022 Annex E.3**,
  agreeing exactly with both prior code extractions. Recorded with a
  qualification the code's doc comment does not make: **Annex E is
  *informative*.** "Primary-spec" means the digits are printed in the
  specification — **not** that Bradford is mandated (ambiguity **A29**:
  recommended, not mandated).
- **An extraction hazard worth carrying:** ICC.1:2022 sets `−`, `+`, `×`
  in the **Symbol font**, landing in the Unicode private-use area, and
  **all three extractors tested drop them silently — the Bradford matrix
  in Annex E.3 extracts all-positive.** The signs in iccce's constant
  come from the cross-verified code sources, which the Annex then
  confirms.
- **★ A bookkeeping gap, stated rather than closed.** DL-002 forbade any
  claim in this project from citing an ICC.1 clause number, and DL-006
  said the prohibition lifts only when `icc-spec-librarian` files
  DL-002's **successor entry**. **No successor exists — §5 ended at
  DL-009 before today's filing** *(verified)* — while
  `crates/iccce-color/src/adapt.rs` already cites "ICC.1:2022 Annex E.3"
  and today's **DL-011** cites 6.3.4.2 and 10.10. The *condition* is
  materially met; the *entry* is unfiled. Recorded inside DL-011 as an
  open item belonging to `icc-spec-librarian`, not silently treated as
  discharged.

### Filed forward: the legacy-Lab selector, before Pass 4 writes it

**DL-011** records that the legacy 16-bit PCSLAB encoding attaches to the
**TAG TYPE** — `lut16Type` and `namedColor2Type`, *"and only those tag
types"*, per 6.3.4.2 NOTE 3 — and **never** to `header.version`. Filed in
Pass 1 deliberately, because the version test is the natural instinct,
was **the corpus's own retracted first-pass claim**, and is what the
field's dominant CMM does.

**This puts iccce in a live disagreement with lcms2** (which keys on
version) and the entry states the disagreement with its limits: the
clause text is certain; that lcms2 is *behaviourally* wrong on real files
is **not**, because the two selectors agree on the common
`mft2`-in-v2 case and no lcms2 tree was read. **The owed resolution is a
behavioural difftest by `icc-conformance`** — a synthetic v4 profile with
an `mft2` Lab `A2B0`, a known `L*` through `transicc`, and which of
`652.8` / `655.35` comes back. **That test does not exist.** DL-005 is
untouched by this: what changes is the *selector*, not the arithmetic,
and the exact-value-invariant testing method DL-005 mandates is more
necessary than before.

### `docs/NUMERIC_CLAIMS.md` — created

Created today, with **NC-001** (the Sharma result) as its first row,
exactly as `NEXT_SESSION.md` specified: *with* the first measured claim,
not earlier to have it ready.

Its design, since it is this project's own artefact rather than a copy of
the sibling's: seven **evidence classes** ordered by what they can
actually prove, with `arithmetic-identity` explicitly demoted below
`published-ground-truth`; **coverage as part of every claim** (NC-001 is
"34 of 34 Sharma pairs at k=1:1:1", never "verified"); a §1.1 warning
that **a passing test records the bound it asserts, not the residual
observed**; a §4 register of named approximations and deviations; a §5 of
**what Pass 1 does not claim**; and a §6 dependency table so a later Pass
can find the rows it invalidates.

**Nine rows record things that would otherwise read as coverage and are
not**: there is **no `implementation-cross-check` row anywhere in the
ledger** — no Rust difftest harness exists, so not one number in it has
been compared against lcms2 — and **no ground-truth row for chromatic
adaptation**, which is Pass 1's largest evidential hole.

### Filed this session (Pass 1)

| Where | What |
|---|---|
| `docs/NUMERIC_CLAIMS.md` | **Created.** NC-001 (Sharma 34/34) plus 17 further rows classified by evidence class; NA-001 (the `f(t)` deviation), NA-002 (Bradford as policy, cost unmeasured), NA-003 (no clamping in the colour layer); §5 "what Pass 1 does not claim"; §6 invalidation map; §7 owed items. |
| `ROADMAP.md` | Pass 1 marked **core complete and validated**, *not* done, with the done-when answered exactly (met where a published value exists; what has none); the four remainder items each labelled **blocked on sourcing, not engineering**; a dated annotation under Pass 2 correcting the now-stale "a validator is not defensible" line. Plan text unchanged. |
| `ARCHITECTURE.md` §5 | **DL-010** (the `f(t)` rational-form deviation) and **DL-011** (legacy Lab keys off tag type; the lcms2 disagreement; the unfiled DL-002 successor). DL-001…DL-009 untouched. |
| `SESSION_LOG.md` | This entry. |
| `NEXT_SESSION.md` | Rewritten for Pass 2. |

Not touched, by instruction and by ownership: `LEGAL.md`,
`TOLERANCES.md` (`icc-conformance`), and the corpus.

**Nothing was committed** — instructed not to, and committing is the
engineer's act. **No git command was run**, by an agent that has no
shell; every statement above about the repository is limited to what was
read out of files in the working tree.

### Left for the next session to not assume

- **That Pass 1 is finished.** Its core is. ΔE94, ΔE CMC and the von
  Kries HPE matrix are **not built**, each blocked on a citable source;
  observer CMF tables are not needed until spectral input exists.
- **That the ledger's tolerances are the tolerance budget.**
  `TOLERANCES.md` §3.1 is **still entirely blank** and §5 still reads
  "none registered yet" *(verified)*. Those rows are
  `icc-conformance`'s; `NUMERIC_CLAIMS.md` records what the tests
  actually assert and does not fill them.
- **That Pass 1's numbers are commit-anchored.** They are anchored to an
  uncommitted working tree. Whoever commits fills the hash into
  `ROADMAP.md`'s Pass 1 record and `NUMERIC_CLAIMS.md` §2.
- **That the corpus D50-chromaticity erratum is fixed.** It was still
  present at filing.
- **That DL-002's citation prohibition has been formally lifted.** Its
  condition is materially met; its successor entry is unfiled. See
  DL-011's closing section.
- **That anything ran on Linux.** Nothing has, and no CI run has ever
  been observed by anyone in this project.

---

## 2026-08-11 (autonomous-loop continuation) — Pass 2 batch 1, the difftest harness, and a prediction that turned out to be wrong

**Fourth entry of the same calendar day and the same session.** Two
commits arrived since the Pass 1 filing, from two different agents, and
they are filed together because the second changes what the first's next
step has to do.

| Commit | What, per the dispatch |
|---|---|
| **`b35a12e`** | Pass 2 batch 1 — the non-LUT tag types decoded and wired into `inspect`; 47 workspace tests |
| **`bfd6b1e`** | the difftest harness, the legacy-Lab probe, and `TOLERANCES.md`'s first filling — by `icc-conformance` |

**Both hashes are *reported*.** `icc-librarian` has no shell, ran no git
command, and has verified neither that these commits exist nor that they
contain what the dispatch says. Everything below labelled **verified**
was read in the working tree by this librarian this session; the
dispatch's account of what changed is a claim like any other and was
checked against the live source rather than transcribed.

### What was built — Pass 2 batch 1

`crates/iccce-profile/src/tag_types.rs`: **eight non-LUT tag types**
decode — `curv`, `para`, `text`, `mluc`, `desc`, `ncl2`, `XYZ `, `sf32`.
The module doc names itself *"Pass 2, batch 1 (the non-LUT types)"* and
states that *"the LUT family (`mft1`/`mft2`/`mAB `/`mBA `) is batch 2"*.
*(verified — module doc, the eight `sig::` constants, the eight arms of
`decode()`.)* The CLI decodes each tag, prints a summary where the type
has one, and prints **every `TagIssue` unconditionally**, the parser's
disclosure surface. *(verified — `iccce-cli/src/main.rs`.)*

**The invariant is enforced by the type design, not by discipline**: a
violation that leaves the layout decodable becomes a `TagIssue`
*alongside* the decoded value; one that makes the layout unknowable is an
`Err` — *"there is no partial result to be tempted by."* Counts are
bounded against the actual byte length **before** allocation.
*(verified.)*

**Gates.** `cargo test --workspace` **47 green**, verified live on the
system sRGB profile *(reported by `icc-engineer`)*. Checkable without a
shell: **47 `#[test]` declarations exist** — `tag_types.rs` 12, `lib.rs`
8, `num.rs` 6, `iccce-color` 21. *(verified — counted across 8 files.)*
**A count of tests declared is not a count of coverage and not a pass
result.**

**Pass 2's done-when is not met and is not claimed to be:** no sweep of
the machine's profiles, and **no synthetic corpus** —
`tools/gen-profiles/` and `fixtures/synthetic/` still do not exist. The
only synthetic profiles this project has ever authored are the four
written inside the difftest probe.

### ★ The measurement that closed DL-011's open question — and it came out the other way

DL-011 was filed in Pass 1 **before** any code, recording that the legacy
16-bit PCSLAB encoding keys off the **tag type**, and that lcms2 was
**believed** to key it off the profile version — a belief the entry
explicitly marked unverified, owing `icc-conformance` a behavioural
difftest. **That difftest has been run, and the belief was wrong.**

Four synthetic profiles, authored byte by byte inside the probe, `mft2`
`A2B0`, Lab PCS, 2×2×2 CLUT; three of them **byte-identical except the
version word**, asserted at run time as a byte diff at exactly offsets
`[8, 9]` before any result is believed; a fourth with proper v4 `mluc`
metadata to close the metadata objection; probes on exact CLUT corners so
nothing is interpolated; `-c0` so lcms2 does not flatten the pipeline.
**Every profile — v2.1, v4.3, v4.4, `mluc` — decodes LEGACY**, worst
deviation from the legacy prediction **2×10⁻⁵**, against an attribution
bound of 0.01 justified as ~7× the quantisation floor and ~20× below the
smallest hypothesis separation. The **v2.1 control reads legacy**, so the
instrument can detect the effect it is looking for. *(reported — the run;
verified — the probe's specs, byte-diff control, decode predicates and
tolerance justifications were read in the source.)* Corroborated by
reading `cmsio1.c` at the pin, where `_cmsReadInputLUT` tests the tag
type and the PCS and **contains no version test** *(reported —
transcribed in `difftest/README.md` §12.2; no lcms2 source was read here,
and `vendor/` is git-ignored so it is not in this repository to read)*.

**What that changes, precisely.** DL-011's **rule stands** — it came from
ICC.1:2022 6.3.4.2 NOTE 3 and 10.10 and never depended on lcms2. What is
superseded is its *"live disagreement with lcms2"* clause and the
consequence that followed: **there is no divergence to log**, so Pass 4
implements the tag-type selector on the authority of the clause and
**does not** write the runtime warning. Filed as **DL-012**, which
references DL-011 and does not rewrite it.

**Coverage travels with the claim, and it is narrow:** one tag, one tag
type, one direction, one PCS, **one intent for the verdict**, four
synthetic profiles, one platform, one lcms2 build at one commit.
**`ncl2` and B2A were not tested behaviourally** — for those the claim is
a source reading, which is a weaker object and is not merged into the
same sentence.

### ★ The second finding, which is larger than the one that was asked for

The probe's first run used intent 0 as well as intent 1. At intent 0 the
**v4** profiles matched **neither** hypothesis — black came back at
`L* = −3.1482` instead of 0 — while the byte-identical v2 profile was
unaffected. **Refusing to round an observation that matches neither
candidate is what turned a confound into a finding.**

The mechanism, read at the pin in `cmscnvrt.c` `_cmsLinkProfiles`:
**lcms2 forces black point compensation ON for v4 profiles at perceptual
and saturation**, whether or not `-b` was passed, with upstream's own
comment attributing it to *"Adobe's document"* — **not to ICC.1, and not
to a document anybody here has read.** Confirmed **quantitatively**, not
assumed: transcribing lcms2's own `ComputeBlackPointCompensation` with
its fixed perceptual black predicts the observation to **3×10⁻⁵** across
all four probes, including the `0 → −3.1482` shift. The arm that did
**not** decide is kept and labelled — re-running the v2 profile with `-b`
is a no-op on that fixture, so the two arms differ in more than the
variable and settle nothing; a reader repeating it would otherwise read
the null result as a refutation. Filed as **DL-013**.

**Why it matters more than the question it interrupted.** It changes what
two later Passes' cross-checks are *measuring*:

- **Pass 4's done-when** says *"matches lcms2 within tolerance at every
  intent."* On a v4 profile, two of those intents compare against a
  transform with BPC in it. Pass 4 must either account for the forced BPC
  explicitly or compare at the colorimetric intents only **and say
  which** — the disagreement otherwise absorbed is **≈3.15 `L*` at
  black**, which is not a tolerance question.
- **Pass 5's** natural `-b`-on/`-b`-off experiment **does not isolate the
  variable** on v4 profiles at those intents. It also inherits a real
  head start: lcms2's BPC arithmetic transcribed and pre-validated to
  3×10⁻⁵ — usable as a cross-check, never as ground truth for what BPC
  *should* do.

It is also a plausible origin for the corpus's retracted belief: **lcms2
does key a decision on the profile version — at perceptual intent. Just
not that one.**

### A discrepancy found while filing, reported and not repaired

The probe's **module-doc** prediction table and the same table in
`tools/difftest/README.md` §12.1 disagree in two cells, both on the
**rejected** hypothesis: the module doc prints P3 general `L* = 50.0004`
and P4 general `a* = 125.9078`. Recomputing from the code's own
`decode_general`: `32768·100/65535 = 50.000763 → 50.0008` and
`65280·255/65535 − 128 = 126.007782 → 126.0078` — **the README is right
in both cells and the module doc is wrong in both.** *(Arithmetic done
independently here; `decode_general` read in the source.)* **The verdict
is unaffected** — the predictions are computed at run time, not read from
the prose, and both wrong cells are still far outside the attribution
bound from the legacy values. It is a doc-comment defect in
`icc-conformance`'s file; **this librarian did not edit it** and filed it
as owed work in `NUMERIC_CLAIMS.md` §7.1. Same shape as the project's own
parser rule: report, do not repair.

### Two things that were owed and have landed — verified, not assumed

- **`TOLERANCES.md` §3.1 and §5 are filled**, dated 2026-08-11 by
  `icc-conformance`, with §4 recording both as *"first filling, not a
  change"* and §6.1 recording the two findings above. *(verified — read
  this session.)* Pass 1's filing said both were blank; that is now
  stale, and `NUMERIC_CLAIMS.md` §0 carries a dated correction rather
  than an edit.
- **The corpus D50-chromaticity erratum is fixed.**
  `cie__ref__colorimetry_core.md` now derives **0.345703 / 0.358539** for
  the ICC 4-figure triple and carries an `errata:` line **C2** naming the
  change with a post-mortem pointer. *(verified — grepped this session.)*
  Two consecutive filings recorded it as still present; it is worth
  noting that the fix was found by **checking, not by assuming the
  dispatch landed**.

**One small staleness left alone, by ownership:** `TOLERANCES.md` §6's
coverage table still reads *"2–8 not started"* while Pass 2 batch 1 is
built. That file is `icc-conformance`'s and this librarian does not edit
it; flagged here so it is findable.

### Still open, and unchanged by today

**DL-002's successor entry is still unfiled.** §5 now runs to **DL-013**;
several entries and doc comments cite ICC.1:2022 clause numbers; the
*condition* has been materially met since the ingest and the *entry* has
not been written. *(verified — `ARCHITECTURE.md` §5 read in full this
session.)* It is `icc-spec-librarian`'s, per DL-006.

**A corpus retraction is owed and is not verified as landed:** the corpus
named `cmsLabEncoded2FloatV2` and `cmsGetEncodedICCversion` as lcms2's
Lab-decoding mechanism; at this pin `cmsLabEncoded2FloatV2` is a **pixel
formatter** called from `cmspack.c` only and never from profile reading.
A dispatch is **reported** in flight in parallel with this filing.
**Whether it lands is unverified**, exactly as with the D50 erratum —
which took two filings to be checked.

### Filed this session (continuation)

| Where | What |
|---|---|
| `ARCHITECTURE.md` §5 | **DL-012** (the DL-011 disagreement measured **absent**; supersedes DL-011's disagreement clause and its runtime-logging consequence, referencing it rather than rewriting it) and **DL-013** (lcms2 forces BPC on v4 perceptual/saturation; its consequences for Pass 4's done-when and Pass 5). DL-001…DL-011 untouched. |
| `docs/NUMERIC_CLAIMS.md` | A new evidence class **`oracle-behaviour-at-pin`** (§1); a second provenance block **§2.1**; **§3.6** with **NC-019** (legacy selector, worst deviation 2×10⁻⁵ across four profiles), **NC-020** (BPC prediction agreement 3×10⁻⁵), **NC-021** (the oracle-reproducibility smoke check, observed `0.000000e0`); **§5.1** correcting exactly which halves of Pass 1's "no cross-check" bullet are superseded; four new **§6** invalidation rows led by *the pin moving*; **§7.1** re-checking every owed item and adding five new ones; §0 and §8 dated updates. |
| `ROADMAP.md` | A **Pass 2 progress block** (batch 1 at `b35a12e`, the eight types, the done-when explicitly **not** met, batch 2 unblocked by DL-012 with its four rules); dated annotations under **Pass 4** and **Pass 5** carrying the BPC finding; the header status line updated. **No plan text rewritten.** |
| `SESSION_LOG.md` | This entry. |
| `NEXT_SESSION.md` | Rewritten for Pass 2 batch 2. |

Not touched, by instruction and by ownership: `LEGAL.md`,
`TOLERANCES.md` and `tools/difftest/README.md` (`icc-conformance`), and
the corpus (`icc-spec-librarian`, with a parallel dispatch reported in
flight).

**Nothing was committed** — instructed not to, and committing is the
engineer's act. **No git command was run**, by an agent that has no
shell.

### Left for the next session to not assume

- **That `b35a12e` and `bfd6b1e` exist or contain what is recorded
  here.** Both are the dispatch's report. Everything about the *files* is
  verified; everything about the *repository* is not.
- **That NC-019…NC-021 survive a pin change.** They are statements about
  one build of one implementation at commit `21c582a`. Moving the pin is
  already a licence event (DL-001); it is now a behavioural one too, and
  those rows must be **re-run, not re-read**.
- **That "lcms2 keys off the tag type" covers `ncl2` or B2A.** It does
  not: those rest on a source reading, and the measurement covers `A2B0`
  / `mft2` / device→PCS / Lab / intent 1 only.
- **That iccce will copy lcms2's forced BPC.** No such decision has been
  made. It is Pass 4/5's, and it gets its own entry.
- **That the corpus retraction landed**, or that anything else dispatched
  in parallel did.
- **That Pass 2 is nearly done because batch 1 is.** Batch 2 is the LUT
  family — the interpolation tables, the PCS-side encoding selector, and
  the largest tag types in the format — and neither half of the Pass's
  done-when has been attempted.
- **That anything ran on Linux.** Still nothing, still no CI run observed
  by anyone.

---

## 2026-08-11 (autonomous-loop continuation) — Pass 2 batch 2, the machine-wide sweep, and a decision that had been owed for three filings

**Fifth entry of the same calendar day and the same session.** One commit
arrived, plus one shell run that is not a commit, plus the closure of two
items that had been carried as outstanding.

| | |
|---|---|
| Commit | **`d40d601`** — Pass 2 batch 2, the LUT family, per the dispatch |
| Not a commit | a machine-wide sweep of `C:\Windows\System32\spool\drivers\color\` |

**The hash is *reported*.** `icc-librarian` has no shell, ran no git
command, and has verified neither that this commit exists nor that it
contains what the dispatch says. Everything below marked **verified** was
read in the working tree this session; **the dispatch's account of what
changed was checked against the live source rather than transcribed**,
and doing so caught one error in this librarian's own draft (below).

### What was built — the LUT family

`crates/iccce-profile/src/lut.rs`, dispatched from `tag_types.rs` and
summarised by the CLI. *(verified — the module doc, the four `decode_*`
functions, the four `decode()` arms, and the four CLI summary arms
read.)* **7 new tests, all in `tag_types.rs`** (12 → **19**); **54
`#[test]` declarations workspace-wide** *(verified — counted across 8
files)*, against a reported `cargo test --workspace` 54 green with
`fmt`/`clippy` clean *(reported)*.

**The interesting property is that batch 2 makes three known misreads
unrepresentable rather than merely tested against** — the format's most
error-prone structures, closed at the type level:

- **`Lut8` and `Lut16` are separate structs.** `lut8Type` has no
  `inputEnt`/`outputEnt` fields, so reading the `mft2` layout onto an
  `mft1` **shifts everything by four bytes and still parses**. Two types
  make that impossible to write.
- **The `mAB `/`mBA ` matrix is a fixed `[S15Fixed16; 12]`** — 3×4: nine
  coefficients **then three offset terms**, 48 bytes. Reading 36 and
  stopping loses the offsets, which the corpus describes as *"a uniform
  colour cast that looks like a white-point problem"*: the canonical
  wrong-colour-looks-right shape. The test loads distinct values into
  `m[9]` and `m[11]` and asserts they arrive, with the comment *"the
  36-byte misread would have lost them."*
- **One `LutAB` struct serves both `mAB ` and `mBA `** — same storage,
  reverse traversal, **direction carried by the tag's type signature**
  through two distinct `TagData` variants rather than by a boolean the
  caller could pass wrongly.

**Curve chains fail *positionally*.** There is **no count field**: each
element must be parsed to find the next, so one malformed element makes
everything after it **unreachable, not merely wrong**.
`CurveChainBroken { element, position }` says which and where, rather
than reporting a generic short read. **Every size is computed in `u128`
and refused before allocation** (`255^255` must refuse, not wrap), and a
CLUT `precision` outside {1, 2} is refused because the sample width is
otherwise unknowable. *(all verified — read, and each has a test.)*

**The legacy-Lab rule is stated in the module doc as the TAG TYPE rule
with both citations kept separate** — ICC.1:2022 6.3.4.2 NOTE 3
(`primary_spec`) *and* "MEASURED in lcms2 at the pin, 2026-08-11" — i.e.
DL-011's rule and DL-012's measurement, not merged into one sentence.
It also notes that `lut8Type` is **not** in the legacy set, and ends
*"the consumer decodes; this module only repeats the rule so the
consumer cannot miss it"*, which keeps invariant §3.1 intact. **Sourcing
honesty is at the site**: the `mAB `/`mBA ` **byte tables remain
code-derived** and the module doc says so, matching the corpus file's
own split `evidence:` line. *(verified — both read.)*

### ★ The sweep — done-when clause 1, met on this machine

**40 profiles, 40 parse OK, 0 refused, 0 unexpected exits, 0
table-level malformations.** Four EIZO v2 profiles each report one
issue — *"desc: Macintosh ScriptCode block short or missing"* — which is
**exactly the structure the corpus flags as the most frequently
malformed in real v2 profiles**. Decoding continued, the issue was
reported, nothing was repaired: invariant §3.2 exercised on files the
project did not author. *(**reported** — `icc-engineer`'s shell run; the
loop counted exit codes and grepped the CLI's own output. **No number in
this paragraph was verified here** — this librarian has no shell.)*

**What it claims and what it does not.** *"Every profile on **this**
machine, on 2026-08-11, at `d40d601`: 40 of 40"* — never *"iccce parses
real profiles"*. The corpus is one Windows install: heavy on
Microsoft-shipped sRGB variants and vendor display profiles, **light or
empty on large v4 CMYK press profiles with `mAB `/`mBA ` pipelines**,
which are precisely what batch 2 added. **Which LUT types the sweep
actually exercised is not on record**, so it does not establish that the
new decoders met real input at all. And zero malformations across 40
files says nothing about whether the detectors fire — the four `desc`
findings are the run's only positive evidence of that, and they are all
one issue type. **A count is not an inventory.**

### Done-when clause 2 — PARTIAL, and stated as a question rather than answered

*"A synthetic corpus covers each tag type."* **Every implemented tag
type has hand-authored synthetic byte fixtures — inside the unit
tests**, hostile cases included. **`tools/gen-profiles/` does not exist**
and `fixtures/synthetic/` holds only a `README.md` saying *"Nothing here
yet: the generator does not exist."* *(verified — tree enumerated, README
read.)*

**Whether in-test synthetics satisfy the clause is a real question and
this filing does not decide it.** For the strict reading:
`ARCHITECTURE.md` §1 already listed both directories when the plan was
written, which is evidence the author meant files on disk; and in-test
fixtures are **tag-level, not whole profiles**, so they cannot cover
header/tag-table/tag-data interaction and are unusable by a differential
run, a fuzzer, or an external consumer. For the loose reading: they are
byte-authored, versioned, and executed on every `cargo test`, which a
directory of blobs does not guarantee. Both readings are recorded in
`ROADMAP.md`; **neither is recommended**, because it is a scope call and
scope calls are not this librarian's to make quietly.

### ★ DL-014 — filed, after being owed across three filings

DL-002 prohibited citing ICC.1 clause numbers until the primary source
was read. **DL-006 recorded that its condition had fired, and named
`icc-spec-librarian` as the agent owing the successor.** Three
consecutive filings then recorded the entry as still unwritten **while
DL-010, DL-011, DL-012 and several doc comments cited ICC.1:2022 clause
numbers** — a live contradiction inside `ARCHITECTURE.md` §5, which is
this librarian's own document. It is filed now, by `icc-librarian` on
the engineer's dispatch: **a reassignment of the filing, not of the
sourcing judgement**, and DL-014 says so in its own text.

**The terms.** Clause numbers from **ICC.1:2022** may be cited; the
citation **must name the corpus file** carrying it, because the corpus
is the verification trail; and **the tier is per-fact, not per-file** —
`ICC_Spec\index.md` records **15 of 20 files at `primary_spec`, 4 fully
and 11 partly**, and a partly-`primary_spec` file has a split
`evidence:` line. The worked example is the one batch 2 depends on:
`icc__type__lutAtoB_lutBtoA.md` reads `evidence: primary_spec (clause
numbers + the CLUT/interpolation rules) / icc_secondary_code (byte
layouts — NOT re-transcribed this pass)`, so its clause numbers are
citable and its byte tables are not. `lut.rs` already writes it that
way. *(verified — index, frontmatter, and `lut.rs` §Sourcing read.)*
**The prohibition is unchanged for every document nobody has read** —
ICC.1:2010, ICC.1:2001-04, ISO 13655, the CIE and IEC documents, and
"Adobe's document", which remains an attribution transcribed from a code
comment. And DL-002's **other** half is untouched: automated retrieval
from color.org is still prohibited; ICC.1:2022 was cleared by *human*
retrieval, which created no route for agents.

### Two owed items closed — both by checking, not by assuming

- **The `legacy_lab_probe.rs` module-doc arithmetic is fixed.** P3
  general `L*` now reads **50.0008** and P4 general `a*` **126.0078**,
  matching this librarian's recomputation and README §12.1, with a dated
  correction note in the file naming what was wrong and why no verdict
  moved. *(verified — read.)*
- **The corpus retraction landed.** `icc__ref__v2_v4_divergence.md`
  carries *"★ RETRACTED 2026-08-11 (C3) — there is NO divergence from
  lcms2 here"*; `index.md` files it as the corpus's **third self-defect**
  with the generalising lesson attached — *"Reading a codebase's types is
  not observing its behaviour"* — and a new evidence file
  `icc__ref__lcms2_measured_behaviour.md` (M1 the selector, M2 the BPC
  finding). *(verified — read.)* **This is the second consecutive filing
  where an item carried as outstanding turned out to be done.** The rule
  that keeps paying: check the live source, do not trust the last
  filing's status.

### A wrong claim this librarian caught in its own draft

The ROADMAP's batch 2 block was drafted with *"iccMAX identification and
refusal by name — not delivered by either batch"* in its owed list.
**That is false, and the live source refutes it**: `Profile::parse`
refuses major version ≥ 5 with `ParseError::IccMaxRefused`, whose
`Display` names iccMAX, and `iccmax_is_refused_by_name` asserts the
message **contains the string `"iccMAX"`** with the comment *"'refuse it
by name' is the requirement."* It was delivered in **Pass 0**.
*(verified — `lib.rs:94–99, 215–222`, `diag.rs:41–71`.)* The block now
records the correction rather than deleting the item, because **"nobody
checked this" and "this is done" look identical in a to-do list** — and
because the same rule that governs dispatches governs drafts: an
unverified statement about the tree is a claim, whoever wrote it.

### Filed this session (continuation)

| Where | What |
|---|---|
| `ARCHITECTURE.md` §5 | **DL-014** — DL-002's successor. Terms for citing ICC.1:2022 clause numbers; per-fact tier; the unread-document list; what it deliberately does **not** do (no retroactive blessing, no change to the automated-retrieval prohibition). DL-001…DL-013 untouched. |
| `ROADMAP.md` | A **Pass 2 batch 2 progress block** (commit, the four design choices, the hostile-input guards, the sweep with its boundary, clause 2 stated as PARTIAL with both readings, and what Pass 2 still owes); a dated **Pass 3 annotation** (the first `implementation-cross-check` row, NA-002's cost coming due, the sRGB/D65 single-source gap and the BT.709 retrieval question, and Annex F/10.6 making curve work specification-following); header status line updated. **No plan text rewritten.** |
| `docs/NUMERIC_CLAIMS.md` | **§2.2**, a provenance block for `d40d601` with **no rows under it**, stating why Pass 2 produces no numeric claim; **§2.2.1**, the sweep recorded as a coverage observation **deliberately given no NC number**, with its boundary in the same terms as any §3 coverage line; **§7.2**, re-checking every owed item, closing two, and adding three; §8 updated for DL-014. |
| `SESSION_LOG.md` | This entry. |
| `NEXT_SESSION.md` | Rewritten for the Pass 2 remainder decision and Pass 3. |

Not touched, by instruction and by ownership: the corpus
(`icc-spec-librarian`), `LEGAL.md` and `TOLERANCES.md`
(`icc-conformance`). **Nothing was committed** — instructed not to, and
committing is the engineer's act. **No git command was run**, by an agent
that has no shell.

### Left for the next session to not assume

- **That `d40d601` exists or contains what is recorded here.** The files
  are verified; the repository is not.
- **That the sweep's 40/40 means the LUT decoders were exercised.** It
  does not — no per-tag-type breakdown was taken, and this machine's
  profile population is the wrong shape for `mAB `/`mBA `.
- **That Pass 2 is done.** Clause 1 is met on one machine; clause 2 is
  partial and blocked on a scope decision, not on code.
- **That DL-014 blesses the ICC.1 citations already in the tree.** It
  does not, explicitly. No audit of them has been performed by anyone.
- **That `TOLERANCES.md` has caught up with Pass 2.** It has not. §3.2's
  four Pass 2 rows all still carry **`—` in Tolerance, Justification and
  Measured**, and §6's coverage table still reads **"2–8 | not
  started"** while both batches are built and 40 profiles have been
  swept. *(verified — grepped and read this session, and **not edited**:
  that file is `icc-conformance`'s.)* Two of those four rows —
  `s15Fixed16Number` decode and curve evaluation — are the ones §3.2's
  own preamble says exist *"so that the ones which are numeric are not
  forgotten"*, and they are the natural first Pass 2 rows in this ledger
  if anyone fills them.
- **That anything ran on Linux, or that any CI run has ever been
  observed.** Still nothing, by anyone, ever.

---

## 2026-08-11 (autonomous-loop continuation) — Pass 3: iccce's first transform, a bug the exact-value tests caught, and a prediction of this librarian's own that the code refuted

**Sixth entry of the same calendar day and the same session.** Two
commits, twelve new ledger rows, two new decision-log entries — and
**still no comparison against lcms2**, which is the thing the Pass is
named for.

| | Commit *(all **reported**)* |
|---|---|
| Pass 3 core — `iccce-cmm/src/curve.rs`, `matrix_trc.rs` | **`c4038eb`** |
| `iccce transform` + the engineer's agent-memory | **`051707f`** |

**The hashes are *reported*.** This librarian has no shell, ran no git
command, and has verified neither that these commits exist nor that they
contain what the dispatch says. Everything below marked **verified** was
read in the working tree; **the dispatch's account was checked against
the live source rather than transcribed**, and doing so produced three
corrections — one of them to a prediction this librarian filed itself,
twice.

**This session ran in parallel with an `icc-conformance` dispatch**, on
the operator's instruction of 2026-08-11 (faster loop ticks, parallel
dispatch on disjoint file sets). Consequence for the record: **the Pass
3 done-when numbers are being produced elsewhere, at the same time, and
whether that landed is `unverified` here.** Nothing in this entry may be
read as evidence that it did.

### What was built

**A tone-curve engine and the Annex F.3 matrix/TRC model.** `curveType`
and `parametricCurveType` forward; **inversion per Annex F.1, which is
NORMATIVE**; the F.3 model with the inverse's **clamp before the inverse
TRC** (F.8–F.16) asserted on measured output; **PCSXYZ only**, with a
Lab-PCS profile refused by name and tested against the real SWOP press
profile. Plus `iccce transform` — triples on stdin, **6 decimals** out,
*"the interface `tools/difftest` diffs against transicc"*, one decimal
finer than `transicc` so the comparison is never limited by iccce's
print precision. **14 new tests**, `curve.rs` 9 and `matrix_trc.rs` 5;
**68 `#[test]` declarations workspace-wide** *(verified — counted across
10 files)*, against a reported 68 green with `fmt`/`clippy` clean.

**The pattern worth naming, because it recurs four times in one Pass:
refusal by name, never substitution.** The Lab PCS; parametric inverses
for types 1, 2 and 4; the three unimplemented intents; and the
non-monotonic curve, whose inverse the specification leaves free to be
**anything** and which iccce refuses rather than choosing silently. In
this domain a plausible substitute is not merely wrong, it is invisible.

### ★ Two findings from the first test run, and the first is the important one

**1. A real bug, caught by an exact-value test — and the round trip
would have missed it.** `eval_table` paired the **clamped** segment
index with the **unclamped** fraction, returning `t[n−2]` at `x = 1.0`:
**`TRC(1.0) ≈ 0.998`**, a 0.2 % error of exactly the class this project
exists to catch.

The counterfactual is the content. With the bug present, the
real-profile **round trip** would have missed by `1/1023 = 9.775×10⁻⁴`
against its `1×10⁻³` bound — **inside, with about 2 % of margin** — and
the white check would have missed by `1.9×10⁻³` against `1×10⁻²`, also
inside. **Both would have passed.** The reason is structural rather than
unlucky: the error is **exactly one table spacing**, and the round-trip
bound was justified as *≈ the table's input spacing*. **A tolerance
cannot discriminate a defect whose magnitude is its own
justification.** Only the `1×10⁻¹⁵` assertion at the sample points
caught it. Filed as **DL-016**, with the arithmetic labelled for what it
is: **`icc-librarian`'s reconstruction from the code as written, nothing
run, resting on a 1024-entry table size that is the engineer's statement
in a comment and unverified here.**

**2. A fact about a real file, and a tolerance re-justified instead of
tuned.** The system sRGB profile's colorant `Z` sums to **0.825089** —
`1.9×10⁻⁴` from ICC's 4-figure D50, **the 1998 author's own white
rounding**. The original `1×10⁻⁴` bound was a quantisation claim *the
file never made*. The replacement is justified by **what it
discriminates**: D65-referenced colorants would sit `0.26` away, **26×
the new `1×10⁻²` bound**, while authoring spread is **50× inside** it.
It cannot fail on a well-formed profile and cannot pass a wrong white.
Ledger **NC-031** — and the fourth consecutive occasion on which rule
5's first question, *is the code wrong?*, was answered **no**.

### ★ A prediction this librarian filed twice, refuted by the code

The Pass 3 annotation in `ROADMAP.md` and the carried list in
`NEXT_SESSION.md` both said **NA-002's cost comes due at Pass 3**,
because *"sRGB→AdobeRGB adapts."* **It does not.** `iccce-cmm` performs
**no chromatic adaptation at all** — it imports only `Mat3` and `Xyz`
from `iccce-color`, never touches `adapt.rs`, and never reads `wtpt` or
`chad`. Colorants in a conformant profile are **already** D50-referenced,
so source-forward + destination-inverse *is* the media-relative
conversion. *(verified — imports and both conversion functions read.)*
**Bradford is still unexercised by any transform in this project**, and
the debt moves to the first Pass that adapts — most likely Pass 4.

Both statements are **left standing** as the record of what was
expected; the correction is filed as a dated note in
`NUMERIC_CLAIMS.md` §4 and in the ROADMAP's progress block. **This is
the memory rule paying for itself**: the live-source check covers the
librarian's own inferences, not only the dispatch's account — and this
is now the second time it has caught one.

The same reading produced **NA-005**, a new register entry: *colorants
are used **as stored** and `chad`/`wtpt` are never consulted.* Correct
on a conformant profile, **unbounded** on a non-conformant one, nothing
checks it at build time, and the only place the property is verified
anywhere is a single test on a single file.

### DL-015 — a divergence that is deliberately **not** filed next to DL-010

`pow(negative, fractional)` is NaN; **lcms2 guards, ICC's own sample
code does not**, and iccce follows lcms2. The corpus states the conflict
verbatim and directs the choice. **What matters is the kind of
departure**: clause 10.18 declares those parameter combinations
**explicitly undefined** — *a stated non-requirement, stronger than
silence* — so this is a choice **inside a hole the standard leaves
open**, not a deviation from printed normative text like DL-010. The
register (`NUMERIC_CLAIMS.md` §4) now states the *kind* in the row, so
an auditor can tell **NA-001** and **NA-004** apart at a glance.

Two limits found while filing and **reported, not repaired** (the file
is the engineer's): the module doc says the guard yields a *"defined,
reported value"* — it is defined, and **nothing reports it**,
`Trc::eval` having no diagnostic channel; and the guard **also fires on
a well-formed input**, parametric type 0 with `g = 0` at exactly
`x = 0`, giving `0.0` where `x⁰ = 1`.

### The done-when is NOT met, and that is the headline

*"sRGB→AdobeRGB round-trips within a stated ΔE, and matches lcms2 within
a stated tolerance, with both numbers written down."* **Neither number
exists.** NC-032 is a round trip through **one** profile in **device
units** — source and destination the same, so the matrix and its inverse
cancel and it prices only the curve stack. It is **not** the done-when's
ΔE. And **`iccce` has still never been compared to another
implementation**: zero `implementation-cross-check` rows, on the day the
transform that makes one possible finally landed. **Pass 3 is IN
PROGRESS. So is Pass 2** (one scope decision), so the Passes are no
longer completing in order.

### Filed this session (continuation)

| Where | What |
|---|---|
| `ARCHITECTURE.md` §5 | **DL-015** (the `pow` guard; the kind-of-departure distinction; the two doc-comment limits) and **DL-016** (exact-value assertions at sample points, with the counterfactual arithmetic and its exact epistemic status). DL-001…DL-014 untouched. |
| `docs/NUMERIC_CLAIMS.md` | **§1** gains the **`normative-rule-conformance`** evidence class, with its transcription-risk caveat stated in the row. **§2.3** a provenance block for `c4038eb`/`051707f`; **§2.3.1** a **DL-014 citation audit of the new code only** — four of five compliant, one naming an ambiguity-register row instead of the file carrying the clause. **§3.7** twelve rows, **NC-022…NC-033**, with **§3.7.0** stating in advance where the pending done-when numbers will go and that **no NC number is reserved** for them. **§4** gains **NA-004**, **NA-005** and the dated NA-002 correction. **§5.2**, **§6** (six new dependency rows), **§7.3**, **§8**. |
| `ROADMAP.md` | A **Pass 3 progress block** — the done-when answered exactly as *not met*, what was delivered, the two findings, the corrections to its own annotation, DL-015, the gates, and a three-item remainder of which **only one is engineering**. Header status updated. **No plan text and no prior annotation rewritten.** |
| `SESSION_LOG.md` | This entry. |
| `NEXT_SESSION.md` | Rewritten for the pending conformance numbers and Pass 4. |

Not touched, by instruction and by ownership: `tools/difftest/` and
`TOLERANCES.md` (`icc-conformance` is working in them **in parallel**,
right now), the corpus (`icc-spec-librarian`), and `LEGAL.md`.
**Nothing was committed** — instructed not to, and committing is the
engineer's act. **No git command was run**, by an agent that has no
shell.

### Left for the next session to not assume

- **That `c4038eb` or `051707f` exist or contain what is recorded here.**
  The files are verified; the repository is not.
- **That the parallel `icc-conformance` run landed.** Look for the rows
  in `NUMERIC_CLAIMS.md` and `TOLERANCES.md`; do not infer them from
  this entry. Three filings running, checking has beaten assuming.
- **That Pass 3 is done.** It is not, and it cannot be until two
  measured numbers exist.
- **That "68 green" means 68 assertions ran.** Two of Pass 3's fourteen
  tests **skip silently** when the system sRGB profile is absent, and
  `cargo test` cannot distinguish *passed* from *did not run*.
- **That NC-032's residual is known.** Only its **bound** is on record.
  The residual is the cheapest number in the ledger to obtain and it
  would turn DL-016's reconstruction into a measurement.
- **That the absolute-intent formula is somewhere in the corpus.** It is
  **not transcribed**, it is a **new named gap**, and *"it is probably in
  clause 6.x or an Annex"* is a prediction until the document is open.
- **That anything ran on Linux, or that any CI run has ever been
  observed.** Still nothing, by anyone, ever.

---

## 2026-08-11 (autonomous-loop continuation) — ★ Pass 3 CLOSES: iccce is compared to another implementation for the first time, and a tolerance is corrected rather than widened

**Seventh entry of the same calendar day and the same session**, and the
one the previous six were building toward. Six commits *(reported)*, ten
new ledger rows, two new named approximations, two new decision-log
entries — and, for the first time in this project's history, **a number
that compares `iccce` to something other than `iccce`.**

| | Commit *(all **reported**; no agent in this project has ever run git)* |
|---|---|
| the four audit items this librarian filed as owed | **`55772c6`** |
| the previous filing, committed | **`a9618fe`** |
| n-linear CLUT evaluator (the A16 named choice) | **`fc5ff58`** |
| 16-bit PCS encodings, exhaustive round trips, the D1 discriminator | **`0843094`** |
| absolute intent (D.6/D.7) + the **sourced** Table 25 intent policy | **`6873df1`** |
| the Pass 3 differential results + the `LEGAL.md` §1 dependency mirror | **`986dae6`** |

**The hashes are reported.** This librarian has no shell and verified
none of them. Everything marked *verified* below was read in the
**working tree**, and the dispatch's account was **checked against the
live source rather than transcribed** — which again produced corrections,
listed below.

### ★ The done-when, met — both numbers, with their classes

*"sRGB→AdobeRGB round-trips within a stated ΔE, and matches lcms2 within
a stated tolerance, with both numbers written down."*

- **Against lcms2: max 3.4762×10⁻³ ΔE2000** (mean 5.1145×10⁻⁴) against a
  tolerance of **2×10⁻²**. Class **`implementation-cross-check`** —
  **the first rows of that class this ledger has ever carried.**
- **Round trip: max 1.8788×10⁻² ΔE2000** (mean 8.674×10⁻⁴) against
  **2.5×10⁻²**. Class **`self-consistency`**, and it stays labelled so
  however reassuring it looks.

sRGB IEC61966-2.1 → Adobe RGB (1998) — the pair the done-when names, both
present, **no substitution invoked** — media-relative, `-c0`, **133
deterministic points**, one platform, lcms2 at pin `21c582a`. Ledger
**§3.8**, rows **NC-034 … NC-043**.

**The pair is also a better instrument than "the named one" implies:**
the source's TRCs are **1024-entry sampled tables** and the
destination's is an **analytic gamma**, so one run exercises table
interpolation *and* analytic evaluation, Annex-F.1 table inversion *and*
analytic inversion. Had both been gammas, half of `iccce-cmm::curve`
would have gone untested under a report saying *"sRGB → Adobe RGB
verified."*

### ★ Why these numbers are worth more than a green suite — two experiments that tested the justifications

**1. The cross-check tolerance was derived from lcms2's own arithmetic,
and then that derivation was checked by emulating it.** `cmsgamma.c`
quantises a segment-free tone curve's input *and* output to 1/65535.
Modelling exactly that inside iccce's model collapses the device-space
residual from 6.705882×10⁻⁵ to **2.311449×10⁻⁷ — a factor of ~290, and
below `transicc`'s own print floor.** The disagreement is **accounted
for**, not absorbed. Ledger **NC-041**. *An assertion in a `why` string
is exactly the kind of claim this project exists to distrust, and this is
what distrusting one looks like when it survives.*

**2. The round-trip tolerance FAILED, and what moved was the
derivation.** It was 1×10⁻² for one run. `TOLERANCES.md` §0's procedure
was followed **in order** and step 4 was reached only because step 3
found something: the original reasoning — *"sRGB ⊂ Adobe RGB, so nothing
is clipped"* — is true of the two **colour spaces** and **false of the
two files.** Their encoded media whites differ by **5, 2 and 12 units of
`s15Fixed16`'s lsb**; source white lands at **(1.000106, 0.999873,
1.000254)** in destination linear space; **25 of 133 points clip.** The
white-corner cost was then **predicted in closed form from the two
matrices and the clamp alone** — **1.878244×10⁻² predicted vs
1.878818×10⁻² observed, 0.03 %.** Ledger **NC-038**, **NC-042**; both
justifications preserved in `TOLERANCES.md` §4.

**3. And a seventh check exists to stop the round-trip gate rewarding a
deleted requirement.** Removing iccce's clamping makes the round trip
*better*; a gate with that gradient is not a gate. The pin holds
|predicted − observed| to **5.7392×10⁻⁶** against 1×10⁻³, with a
sensitivity control showing it would **fail by 19×** without clamping.
Filed as the method rule **DL-018** — with the scope limit a first draft
got wrong stated in the entry: it does **not** make the F.8–F.16 clamp
*ordering* falsifiable, because iccce clamps at three sites and the
other two make that one redundant. **Owed, not covered.**

### A finding against lcms2, kept as a finding

**8 of 399 output components (2.01 %) came back above 1.0, up to
`1.000120`** — and **only on the analytic-inverse path**; the tabulated
reverse curve saturates. Annex F.8–F.16 supports iccce; clause 6.4
requires no clipping for float32 encodings and may make lcms2's
excursion conforming and iccce's clamp merely stricter. **Recorded as a
difference, not a verdict** (rule 7). Ledger **NC-043**. Note the
status collision, kept visible rather than resolved by preference:
`tools/difftest/README.md` §13.10 says the spec dispatch was **not
made**; the dispatch says it **was**; the answer is **not in the
corpus**, which is the only one of the three this librarian can check.

### The three Pass 3 remainder items — closed, and one of them closed the right way round

**Absolute intent was SOURCED and then implemented** — the corpus gap
this librarian filed this morning closed the same day, and
`icc__s__rendering_intents.md` came back carrying more than was asked:
the **D.6/D.7 equations**, the **9.2.36** consequence, and a defect
finding — **clause 6.2.3's prose states the source/destination ratio
BACKWARDS.** The code implements the corrected direction and pins it with
a test against the corpus's printed intermediates (**0.7067/0.85 =
0.831412**, the backwards reading asserted **absent**). **Parametric
inverses for types 1, 2 and 4** are implemented analytically — no sampled
inverse, so rule 4's measured-cost obligation never arose. And the
**perceptual/saturation policy was settled by SOURCING rather than by the
differential**: ICC.1:2022 **Table 25** marks the TRC/matrix column
*"Colorimetric"*. The progress block had said *"the differential test
owns it"*; a measurement would have shown agreement, **but it would not
have shown authority**.

### ★ Three things this filing corrected by reading rather than transcribing

1. **`lut_transform.rs` exists in the tree and the dispatch does not
   mention it** — *"lut16Type evaluation pipeline — Pass 4 assembly,
   stage 1"*, 4 tests, declared in the crate's `lib.rs`. **It was absent
   from a `Glob` of the crates tree run earlier in this same filing
   session.** Either an agent is writing concurrently or the earlier
   enumeration was stale; this librarian cannot tell without a shell.
   **The tree being described was moving while it was described**, and
   the next session must not read *"Pass 4 needs the lut16 assembly"* as
   *"none exists."*
2. **Absolute intent is implemented in the library and unreachable
   through the CLI**, which still refuses every intent but
   media-relative *(verified)*. Since the harness drives the **binary**,
   **no differential can exercise absolute intent** — it carries
   unit-test and corpus evidence and **zero cross-check evidence**.
   Registered as **NA-007**.
3. **Two doc-comment claims overstate what was done.**
   `iccce-cmm/src/lib.rs`'s §Status still says the absolute intent
   *"awaits its sourced formula"* (second consecutive filing to report
   that file's §Status); and `clut.rs` says its interpolation choice is
   *"named **and measured**"* when the ~1 ΔE figure is a **corpus-derived
   bound** and tetrahedral is deliberately absent, so **nothing has been
   measured**. Registered as **NA-006**. Both **reported, not
   repaired** — the files are the engineer's.

### The count discrepancy, recorded unresolved

`tools/difftest/README.md` §13.9's transcript ends **`pass=8`** over
eight `check` lines; the engineer's verification re-run is reported as
**`pass=7`**, with no per-line output. Structurally `checks()` registers
**one** check and `pass3.rs` emits **seven** — 1 + 7 = 8 *(verified —
both read)*. So `pass=7` is *consistent with* the smoke check not
counting, **and that is a hypothesis.** The seven per-record values are
unaffected: they agree across three independently written places. What is
affected is that **the re-run cannot be quoted as re-verifying all eight
lines, because nobody recorded which eight it ran.** A summary count is
not an inventory.

### Filed this session (closure)

| Where | What |
|---|---|
| `ROADMAP.md` | The **Pass 3 completion record** — the done-when quoted and answered with both numbers, their classes and their tolerances; the three remainder items with how each closed; the corrections; the coverage statement; what is still open; and the Pass 4 code already in the tree. Header status updated. **No plan text, no prior annotation and no prior progress block rewritten.** |
| `NUMERIC_CLAIMS.md` | **§2.4** provenance for the six-commit set and the run-count discrepancy; a **dated status** on §3.7.0 (the held space filled, three superseded statements named); **§3.8**, ten rows **NC-034 … NC-043** with a shared coverage box; a dated status on §3.7.6 (two refusals discharged, one not); **NA-006** (the A16 CLUT choice — a *third kind* of named departure: a choice inside a **silence**) and **NA-007** (`wtpt` as stored); **§5.3, the retirement of *"iccce has never been compared to anything"***; eight new §6 dependency rows; **§7.4** and seven newly-owed items; §8. |
| `ARCHITECTURE.md` §5 | **DL-017** (the harness may path-depend on iccce's crates — direction plus four conditions) and **DL-018** (an upper-bound gate on a deliberate cost needs a prediction pin, with the worked pair and the scope limit). DL-001…DL-016 untouched. |
| `SESSION_LOG.md` | This entry. |
| `NEXT_SESSION.md` | Rewritten for Pass 4. |

**Not touched, by instruction and by ownership:** `TOLERANCES.md`,
`tools/difftest/`, `LEGAL.md`, and the corpus. **Nothing was committed**
— instructed not to, and committing is the engineer's act. **No git
command was run**, by an agent that has no shell.

**A note on where the tolerance history lives, since three documents
describe the same event.** `TOLERANCES.md` §4 owns the *number's*
history (1×10⁻² → 2.5×10⁻², both justifications). `NUMERIC_CLAIMS.md`
NC-038/NC-039 own the *measured values*. `ARCHITECTURE.md` DL-018 owns
the *method rule* that generalises. **Deliberately not duplicated** —
three jobs, one event, and the boundary is stated in DL-018 itself so a
later reader does not "helpfully" merge them.

### Left for the next session to not assume

- **That any of the six commits exists or contains what is recorded
  here.** The files are verified; the repository is not.
- **That `lut_transform.rs` is committed**, or that the tree is stable
  while being read.
- **That the differential re-run verified all eight checks.** See the
  count discrepancy.
- **That absolute intent has been cross-checked.** It cannot be, through
  the current CLI.
- **That the NC-043 clamping question has been answered.** Two sources
  disagree about whether it was even dispatched; the answer is not in
  the corpus.
- **That Pass 2 is done.** Still one scope decision, and it now blocks
  something concrete: without `tools/gen-profiles`, every Pass 3
  differential row skips everywhere but this machine.
- **That "87 tests" means anything about coverage.** Two skip silently
  without the system profile; all seven differential records skip
  without the Windows colour directory.
- **That anything ran on Linux, or that any CI run has ever been
  observed.** Still nothing, by anyone, ever.

---

## 2026-08-11 (autonomous-loop continuation) — ★ Pass 4 in progress: the first CLUT differential, a named approximation finally priced, a prediction about lcms2 falsified by reading it, and an 11 ΔE divergence nobody can adjudicate yet

**Eighth entry of the same calendar day and the same session.** Eight
commits *(reported)*, thirteen new ledger rows, one new decision-log
entry, three dated corrections to entries this ledger already carried —
and, for the first time, **numbers that come out of a CLUT, a
four-channel device space, a `Lab ` PCS and all four rendering intents.**

| | Commit *(all **reported**; no agent in this project has ever run git)* |
|---|---|
| the Pass 3 closure filing committed + two engineer doc fixes | **`19a3b17`** |
| `lut16` device→PCS pipeline — assembly **stage 1** | **`9aa1bca`** |
| `transform::Chain` — **stage 2**; CMYK→RGB live, with the perceptual≡saturation shared-tag regression test | **`63874f9`** |
| the CLI: **N-channel input and four intents** | **`490191b`** |
| **B2A evaluation — stage 3**, bidirectional, both tag depths | **`b3f4388`** |
| documentation catch-up | **`db60e92`** |
| the Pass 4 A2B differential | **`d9e0b82`** |
| untracked in-progress `tools/gen-profiles`, swept in by the above | **`edcb60e`** |

**The hashes are reported.** This librarian has no shell and verified
none of them. Everything marked *verified* below was read in the
**working tree** or the **live corpus**, and — again — checking the
dispatch against the source produced corrections, including one to the
dispatch itself.

### ★ What the differential measured, and the shape that makes it readable

`USWebCoatedSWOP.icc` → the Windows system sRGB profile, **341
deterministic CMYK points**, **all four A2B intents**, `-c0`, lcms2
2.19.1 at pin `21c582a`; `summary pass=36 fail=0 skip=3 error=0`
*(reported)*. Ledger **§3.9**, rows **NC-044 … NC-056**.

**Pass 4 has a problem Pass 3 did not**, and the whole filing turns on
it. Pass 3's disagreement with lcms2 was the oracle's 16-bit rounding —
a *defect of precision*, so one tight tolerance was derivable and
meaningful. **Pass 4's dominant term is an interpolation-method
difference between two schemes ICC.1 does not choose between.** It is not
an error in either implementation, it is ~1.6 ΔE2000, and it will not go
away. NA-006 named the trap in advance: *"a tolerance wide enough to
swallow ~1 ΔE cannot also demonstrate agreement."*

**So the suite uses two kinds of gate and says which is which** — a
**wide, structural** one whose value *is* the method envelope (2.0
ΔE2000; catches a wrong index order, a wrong Lab decode, a swapped ink;
**explicitly cannot claim agreement**), and **tight, arithmetic** ones
with the method difference switched off:

- **The interpolation-free corners** — the 16 hypercube corners are
  **exact CLUT nodes** (each `mft2` input table starts `0x0000`, ends
  `0xFFFF`), where the two schemes must agree identically and lcms2's
  quantisation terms **vanish rather than accumulate**. Gate 1×10⁻³,
  observed **5.9131×10⁻⁵** and **6.6558×10⁻⁵** — `transicc`'s own print
  floor, **70× below the same comparison between nodes**. **This is the
  strongest cross-check evidence this project has produced**, and it is
  what makes the 2.0 gate defensible: without a node-only control, a wide
  structural gate could hide a genuine 1.9 ΔE error.
- **lcms2's own geometry substituted** — gate 2×10⁻², observed
  **4.5931×10⁻³** / **4.8154×10⁻³**, a **55× / 326×** shrink. *This is
  the record that claims agreement*, and the record says so on its own
  face.

**And the apparatus was graded before anything was concluded from it:**
the harness's n-linear arm against `iccce_cmm::lut_transform::Lut16Model`
on every point at every intent, tolerance 10⁻⁹, **observed 0.0 exactly**.
An apparatus not shown to reproduce the thing it stands in for is not an
apparatus.

### ★ NA-006 is measured — and running all four intents was not a formality

**The A16 n-linear choice has a price at last**: **1.5741 ΔE2000 max on
SWOP's `A2B0` table**, **0.254 23 on `A2B1`** — computed from the CLUT
and the two algorithms alone, with no lcms2 output in it. The corpus's
*"up to ~1 ΔE"* was the right order and **an underestimate on one of the
two tables**.

**The factor of six is the argument.** The perceptual table's worst cell
is deep shadow at near-full black, where the CLUT turns sharply; the
colorimetric table is six times smoother. **A Pass 4 tolerance derived
from the colorimetric intent alone would have been wrong by 6× for
exactly the intents Pass 3 never exercised.** Nothing about a smooth
colorimetric result predicts a rough perceptual one.

**Two things this closes.** `clut.rs`'s *"per rule 4 (named and
measured)"* — reported as an undischarged claim last filing — **is now
true**, closed by fact rather than by prose. And NA-006's *"cannot yet be
measured, because tetrahedral is deliberately absent"* was **wrong about
its own difficulty**: pricing the choice needed a comparison arm **in the
harness**, not a second interpolator in the shipped crate.

### ★★ A prediction carried in three documents, falsified by reading the oracle

**NA-006, `NEXT_SESSION.md` and `ROADMAP.md` all said *"iccce
interpolates n-linear, lcms2 tetrahedral"***, and the Pass 4 blocker was
recorded as *"source lcms2's tetrahedral cube decomposition."* Rather
than recall it, `icc-conformance` **read `cmsintrp.c` at the pin**: for
**four** inputs lcms2 runs a **hybrid** — *linear* in C, **Sakamoto
tetrahedral** in M/Y/K, blended by the first channel's fraction. So
lcms2's scheme **is not symmetric in the four inks** (iccce's
quadrilinear is), **is not pure tetrahedral** — hence **a bound from the
trilinear-vs-tetrahedral literature, which is exactly what NA-006's
~1 ΔE was, is not the bound that applies** — and **its float path does
not use the float interpolator**, an `mft2` tag being read into a 16-bit
CLUT stage that quantises the input to `u16` and calls the fixed-point
twin.

**This is the second time a predicted disagreement with lcms2 was
settled by measuring rather than assuming, and the second time the
prediction was wrong in a way that mattered** — the first being
DL-011/DL-012, where a *predicted* disagreement over the legacy-Lab
selector was measured **absent**. The prediction is **left standing
wherever it was written**; NC-056 and NA-006's dated note are the
correction. **The failure mode is a prediction quietly becoming a
citation**, and *"lcms2 uses tetrahedral"* was three documents away from
being one.

### ★★ A finding at the absolute intent: 11.217 ΔE2000, mechanism known, authority absent

At `-t3` the two implementations differ by **max 11.217 ΔE2000, mean
4.670** — two orders of magnitude more than at any other intent, worst
at the **lightest** points. Read at the pin: `cmsio1.c`'s
`_cmsReadMediaWhitePoint` **substitutes D50 for the stored `wtpt`** when
a profile is **v2 and display-class**, and the destination sRGB
profile's `wtpt` holds **D65** while its colorants are D50-adapted.
**Both implementations build the same D.6/D.7 diagonal; they read
different destination whites.** The ratio is a **32 % error in `Z`**, and
modelling that single substitution collapses the disagreement **517×**.

**Which one is right is not settled and cannot be settled here**: corpus
**A4b** — the meaning of a v2 `wtpt` — is **UNVERIFIED**, ICC.1:2022 is
silent on version 2's convention, and ICC.1:2001-04 has not been
obtained. **lcms2's substitution is justified in its source by a comment,
not by a clause.**

**What the suite does about it is the reusable part**, and it is filed as
**DL-019**: the raw comparisons are **REPORTED, NOT GRADED**, the gate at
that intent becomes the **modelled** comparison (5×10⁻² against
2.1677×10⁻² observed), and **both rejected alternatives are written down
at the record** — widening to ~15 ΔE00 (*"a number chosen because it
passed; 15 ΔE00 is a different colour"*, and it would silently absorb any
future arithmetic error in the absolute path) and letting it fail
permanently (*"a red line that never changes stops being read"*, and it
would report the disagreement as unexplained when it is not). **This is
the only place in the suite where a known disagreement is deliberately
not gated**, and the scarcity is part of the rule.

**NA-007 predicted this exact bite.** It was registered a filing ago
saying *"where the assumption actually bites: v2 profiles, where the
meaning of a non-D50 `wtpt` is A4b — UNVERIFIED… implementation consensus
is not a specification reading, and this row exists so that sentence
cannot quietly become one."* **The register worked**: an 11 ΔE divergence
arrived pre-explained instead of as a mystery.

### ★ Five things this filing corrected by reading rather than transcribing

1. **The dispatch says `mAB `/`mBA ` are *"undecoded-unevaluated"*. Half
   of that is wrong.** They have been **decoded since Pass 2 batch 2** —
   `tag_types.rs` dispatches `sig::MAB`/`sig::MBA` to
   `lut::decode_lut_ab`, producing `TagData::LutAToB`/`LutBToA`
   *(verified — read)*. What is absent is an **evaluator** in
   `iccce-cmm`, which `lut_transform.rs`'s scope note and
   `ChainError::SourceTagUnsupported` both name. **That is stage 4**, and
   describing it as undecoded would have understated what Pass 2 shipped
   and mis-sized what Pass 4 has left.
2. **`iccce-cmm/src/lib.rs`'s §Status is stale again — third consecutive
   filing.** It now reads *"B2A/lut8/mAB stages pending"* on a crate
   where `b3f4388` landed **B2A and lut8**; only `mAB `/`mBA ` is
   pending. **Reported, not repaired.**
3. **`cmd_transform`'s doc comment contradicts its own code**: *"Only
   media-relative colorimetric exists (Pass 3 scope); an `--intent` flag
   naming anything else is refused by name"*, sitting directly above a
   `match` that accepts `perceptual`, `saturation` and `absolute`
   *(verified — read)*. Worse than a stale status line: a reader who
   trusts it concludes **no differential can reach the absolute
   intent** — true this morning, and the reason the 11 ΔE finding was
   impossible until `490191b`.
4. **`tools/difftest/README.md` §14.7's record decomposition is wrong in
   both terms while its total is right.** It says *"8 Pass 3 records, 1
   smoke, 27 graded Pass 4"* and *"adds 30 Pass 4 records"*; counting the
   emitters in the source gives **7**, 1, **28** and **31** — and
   1 + 7 + 28 = **36**, the reported total, with **3** skips. **A sum
   that comes out right is not evidence that its terms are right.**
   **Reported, not repaired** — §14 is `icc-conformance`'s. As a
   by-product this **confirms §2.4's structural hypothesis** about the
   old `pass=8` line: `pass3.rs` emits seven records, so eight is seven
   plus the smoke check. Ledger §3.9.8.
5. **A carried claim about the tree is now false, and it was this
   librarian's.** Four filings said *"`tools/gen-profiles/` does not
   exist and `fixtures/synthetic/` holds only its README."*
   **`gen-profiles` is a working crate with 28 tests, and
   `fixtures/synthetic/` holds 39 `.icc` files** *(verified —
   enumerated and the module doc read)*, including
   `v4-cmyk-mab-lab.icc`, `v2-cmyk-mft1-lab.icc` and `v4-rgb-mft2-lab.icc`
   — precisely the population Pass 4's remaining work needs and this
   machine's colour directory lacks. **What that does not establish:**
   nobody has run `gen-profiles verify` here, **no differential record
   reads any of them**, and **Pass 2's clause-2 scope decision is not
   thereby answered** — the operator was asked a question about intent,
   and a generator appearing does not answer it.

### The process slip, recorded rather than smoothed over

**`edcb60e` exists because `d9e0b82` was committed with a cwd-relative
pathspec that swept in an untracked, in-progress `tools/gen-profiles`
working state** *(reported by the dispatching engineer; no agent here ran
git)*. Nothing measured is affected. It is logged because **the tree
moving underneath a filing is now a repeat phenomenon in this project** —
the Pass 3 closure entry recorded `lut_transform.rs` appearing mid-filing
from a `Glob` that had not shown it — and because a commit that contains
more than its message says is a hazard to every later reader who uses
`git log` as the record of what changed when. **A pathspec relative to
the cwd is not a statement about what is being committed.**

### The three-document boundary, again, because this filing tested it

`TOLERANCES.md` §5.2 carries **`icc-spec-librarian`'s correction of
NA-003's clause citation** — 6.4 governs the **PCS**, not device values;
6.5's float32 permission is unreachable from a matrix/TRC model;
**a conforming F.8–F.16 evaluation cannot exceed 1,0**, which **inverts**
the direction of the NC-043 finding. That file is `icc-conformance`'s and
**was not edited here.** What this librarian filed instead is a **second
dated note under NA-003** recording the effect on *this ledger's* rows —
because the wrong sentence was written **in `NUMERIC_CLAIMS.md`**, from
recollection of a clause number, and was then **relied on by a
differential finding**. **Rule 2 — never write colour maths from
memory — extends to clause numbers**, and DL-014's requirement to name
the corpus file at the citation is what would have caught it.

### Filed this session

| Where | What |
|---|---|
| `ROADMAP.md` | A header status paragraph, and the **Pass 4 progress block**: the eight commits, what stages 1–3 actually built (verified in source), the three kinds of number with their classes, NA-006's measurement and the falsified prediction, the absolute-intent finding, **the done-when answered clause by clause with B2A recorded as zero-measurement**, the v2/v4 coverage stated exactly, three reported prose defects, the gates, the gen-profiles appearance, and six owed items. **No plan text, no annotation and no earlier progress block rewritten.** |
| `NUMERIC_CLAIMS.md` | **§2.5** provenance (eight commits, the proved-unreachable BPC confound, **two harness traps**); **§3.9**, thirteen rows **NC-044 … NC-056** with a shared coverage box, the two-kinds-of-gate preamble, the falsified-prediction record and **§3.9.8's run-count reconciliation**; a **second dated note under NA-003** (the clause correction and its effect on NC-043); **dated status on NA-006** (measured; mechanism prediction wrong) and on **NA-007** (cost measured, cause exactly where it was predicted); eight new §6 dependency rows; **§7.5** with seven newly-owed items. |
| `ARCHITECTURE.md` §5 | **DL-019** — reported-not-graded plus a gate on the modelled quantity, with the live A4b-gated instance, why it is not DL-018 in different clothes, what it does not decide, and **one candidate considered and deliberately not filed** (the per-depth `PcsCodec`, whose rule is already DL-011's and whose mechanism is self-documenting in code). DL-001…DL-018 untouched. |
| `SESSION_LOG.md` | This entry. |
| `NEXT_SESSION.md` | Rewritten for the Pass 4 remainder. |

**Not touched, by instruction and by ownership:** `TOLERANCES.md`,
`tools/`, the corpus, and `LEGAL.md`. **Nothing was committed** —
instructed not to, and committing is the engineer's act. **No git command
was run**, by an agent that has no shell.

### Left for the next session to not assume

- **That any of the eight commits exists or contains what is recorded
  here.** The files are verified; the repository is not.
- **That "Pass 4 matches lcms2" is a thing anyone may say.** It matches
  **at the corners and with lcms2's geometry substituted**; the raw
  comparison carries a 1.66 ΔE method difference that **cannot claim
  agreement**, and the absolute intent is **not graded at all**.
- **That the B2A direction works.** Code, yes. Measurements, **zero**.
  Same for `lut8Type` evaluation and the `Lab8` codec.
- **That A4b was answered because a dispatch went out.** As of this
  filing the corpus still lists it **UNVERIFIED** and carries **M1–M3
  only** *(verified)*.
- **That `gen-profiles`' fixtures are in use.** They exist; nothing
  reads them; every differential row still skips off this machine.
- **That "89 tests" means coverage.** It is a count of declarations, and
  the profile-dependent ones still skip silently.
- **That anything ran on Linux, or that any CI run has ever been
  observed.** Still nothing, by anyone, ever.

---

## 2026-08-11 (autonomous-loop continuation) — ★ Pass 2 closes on a fixture corpus, Pass 4's evaluation surface completes, and a parser bug is found by a doubt that was refused an hour earlier

**Ninth entry of the same calendar day, and a catch-up filing**: the
previous filing was committed as `97ad9fa` and three landings had already
overtaken it. Filed by `icc-librarian` against the working tree.

**Commits, all reported** *(no agent in this project has ever run git)*:
**`7576cfa`** — `tools/gen-profiles` + the 38-fixture synthetic corpus +
**GP-001 found**; **`2e98cfd`** — **GP-001 fixed** + `mAB `/`mBA `
evaluation + the transicc cross-check on the committed fixture;
**`97ad9fa`** — the **grayTRC F.2** model + the previous filing +
**two code-doc closures**.

### ★★ GP-001 — the day's finding, and the order of events is the whole of it

The `mAB `/`mBA ` evaluator shipped **`mAB `-only**, refusing `mBA ` **on
a curve-count contradiction found during design**: the corpus's rule for
curve counts is **one blanket sentence covering both tag types**, and the
author could not reconcile it with the geometry of a tag that runs
PCS→device, so he declined to guess. **An hour later the fixture corpus's
first run against the shipped binary found the bug**, on exactly that
doubt.

`decode_lut_ab` had used the `mAB ` convention **for both types**, so on
a CMYK `B2A0` (`inputChan = 3`, `outputChan = 4`) it expected **four** B
curves where the specification puts **three**, walked into the matrix
element, and reported `curve chain broken at element 3 (byte 68)`. The
clause text settles it per type — **10.12.2/4/6** for `mAB `,
**10.13.2/4/6** for `mBA `, i.e. **entry side counted by `inputChan`,
exit side by `outputChan`**, which letter that is depending on the
direction — with lcms2's `Type_LUTB2A_Read` as **corroboration, not
authority**. *(The clause quotations are `icc-conformance`'s direct reads
of the PDF, carried with that attribution; this librarian has not opened
the PDF.)*

Four things about it are worth more than the bug:

- **It was invisible on every square LUT** (the two readings coincide
  when `inputChan == outputChan`), and **it affected every real CMYK
  `B2A0`** — the tag a press profile prints with.
- **The machine-wide sweep could not have found it**, and the Pass 2
  clause-1 record **said so in advance**: the sweep is *"light or empty
  on the population Pass 4 depends on — large v4 CMYK press profiles
  with `mAB `/`mBA ` pipelines."* The fixture **is** that population.
- **The parser's disclosure surface is what made it a diagnosis.** The
  symptom was a **named refusal at a byte position**, not a colour.
  A repairing parser resynchronises on the next plausible curve header
  and returns plausible CMYK.
- **The corpus is the origin and is still wrong** — the blanket sentence
  is **verified still present**, and closing it is
  `icc-spec-librarian`'s, along with **A23** and **A25**.

The fixture was **not** changed to match the parser; the parser was
fixed, and the fixture's `B2A0` now carries a regression with a number:
`K` within **1×10⁻³** of `transicc`'s recorded **0.496117**
(**NC-057**) — the project's first claim made through **bytes it authored
itself**. `ARCHITECTURE.md` gains **DL-020**, filed as **one** entry
rather than three because the refusal, the fixture and the
report-don't-repair surface are one causal chain and break together.

### ★ Pass 2 is DONE, and the scope question dissolved rather than being answered

The batch 2 block asked the operator which reading of *"a synthetic
corpus covers each tag type"* was meant — files on disk, or in-test
bytes. **No answer exists anywhere in these documents** *(verified)*, and
none is needed: **the stronger reading is now satisfied** — 38 whole
profiles on disk, a standalone generator with `verify` and a generated
`MANIFEST.md`, and profile-level coverage of **every tag type the plan
text names**. Clause 1 was met at `d40d601`; clause 2 is met now;
**Pass 2's done-when is met.** The boundary is written out in the new
ROADMAP block — most sharply that **clause 1's sweep predates the GP-001
fix and has not been re-run**, and that `desc` has **no ICC.1:2022 clause
at all**.

### ★ Pass 4's evaluation surface is complete — and three holes have the same shape

`lut_ab.rs` (stage 4, both directions, v4 encodings, all twelve matrix
terms) and `gray_trc.rs` (F.2, both directions) landed, and both are
wired into `Chain` on **both** sides. So every LUT tag type now
evaluates, in both directions, plus monochrome.

**What has no measurement:** **no B2A differential** (one cross-check
*point*, on a synthetic, in a unit test); **no `mAB ` against any real
file**; **no gray comparison against lcms2 at all**. And **nothing
traverses `Chain`** in a test — its two tests are both SWOP→sRGB and
neither reaches either new model.

### ★ Three corrections made by reading rather than transcribing

1. **The dispatch's *"neutrality through the chain measured"* is wrong
   about where.** Neutrality is asserted **in the model** —
   `GrayTrc::device_to_pcs(1.0)` on the real `ewgray22.icm` landing on
   the **full D50 triple** within 1×10⁻³ in X, Y **and** Z, which is the
   green-cast trap's regression. **No gray value has ever gone through
   `Chain`** *(verified — both test modules read, whole crate grepped)*.
2. **This librarian's own previous filing said `fixtures/synthetic/`
   holds *39* `.icc` fixtures. The live count is 38** *(verified —
   enumerated; 38 `.icc` + `MANIFEST.md` + `README.md`)*, which is what
   the generator's README says twice. Most likely origin: counting
   **directory entries**. Second wrong count from a directory listing in
   two filings; the lesson is unchanged — **a listing is timestamped and
   a count is not an inventory.**
3. **This dispatch carried NO gate report** — no `cargo test` count, no
   `fmt`/`clippy` line, where the previous four filings each carried
   one. So **five new ledger rows exist with asserted bounds and no
   reported outcome**, and this is recorded at the provenance rather
   than inferred away. Checkable without a shell: **95 `#[test]`
   declarations across 16 files** under `crates/` (89 across 14 last
   filing; the six new are exactly `lut_ab.rs` 4 and `gray_trc.rs` 2),
   plus 52 under `tools/`, 28 of them in `gen-profiles` — **unchanged**
   *(verified — counted)*.

### The engineer closed both items this librarian filed against his files

`cmd_transform`'s doc comment now says four intents are accepted **and
records that the earlier comment *"outlived the code by three
commits"***; `iccce-cmm/src/lib.rs`'s §Status, stale for three filings,
now enumerates the modules **and carries a standing instruction — *"if a
module below contradicts it, trust the module."*** That is the better fix
for a defect that recurred three times: it does not promise the line
stays true, it tells the reader what to do when it is not.

**And a new one appeared one commit later:** `transform.rs`'s own §Scope
paragraph calls `mAB `/`mBA ` *"the remaining absentees"* in the file
that wires them on both sides, and omits grayTRC entirely *(verified)*.
**Reported, not repaired.** Also stale, and worse in its consequence:
`tools/gen-profiles/README.md` §5 still reads **`Status: open`** for
GP-001 — a reader of that file today concludes iccce cannot parse a real
CMYK `B2A0`.

### The corpus's sixth pass, and one framing this project had wrong

**M4 and M5 landed** (owed and absent at the last filing), and M5
**corrects a sentence carried in three documents**: lcms2 does **not**
"ignore" the stored `wtpt` on v2 display profiles — `_cmsReadCHAD` uses
it, under the **same guard**, to synthesise a Bradford `chad`. lcms2's v2
model is therefore **coherent** (`wtpt` = unadapted white, `chad`
synthesised, adapted white = D50), which removes the easy objection and
leaves a genuine interpretive disagreement. Also: **DemoIccMAX reads
`wtpt` as stored**, so the two ICC-adjacent implementations **disagree
with each other** and **iccce matches ICC's own code**; **M4 generalises
to `EvalNInputs`** — linear in the first `N−3` channels, tetrahedral in
the last three, so hexachrome inherits the asymmetry; **A4b is still
UNVERIFIED**, settled only by **ICC.1:2001-04**, with the ICC errata
recorded as **unreachable by compliant means**; and **A4c is new and
SILENT** — ICC.1 requires **no** colorant/`wtpt` self-consistency, found
from the stock sRGB profile's own bytes (**colorants sum to D50 while
`wtpt` holds D65**), and **A4c does not clear when A4b clears**.

**And a divergence acquired a fixture:** `transicc` **accepts**
`iccmax-version.icc` — **lcms2 does not refuse major version 5**, where
iccce refuses iccMAX **by name**. True since Pass 0, pinned to an
artefact today.

### Filed this session

| Where | What |
|---|---|
| `ROADMAP.md` | A header status paragraph; a **Pass 2 block** judging clause 2 **MET** and Pass 2 **DONE**, with its boundary, a correction of this librarian's own 39-vs-38 count, and three remaining owed items; a **Pass 4 evaluation-surface block** — what was built (verified), the **GP-001 arc**, the done-when re-answered clause by clause, the three unmeasured holes, the dispatch correction, two closures and two new reported-not-repaired defects, the gate position (**none reported**), and the corpus's sixth pass. **No plan text, no annotation and no earlier block rewritten.** |
| `NUMERIC_CLAIMS.md` | **§2.6** provenance (three commits, **no gate report**, what a self-authored fixture can and cannot do); **§3.10** — **NC-057 … NC-061** with a shared coverage box, plus **GP-001** and **two observations deliberately given no NC number**; **NA-008** (the grayTRC inverse's achromatic projection, cost **unmeasured** and *not* a rounding cost); **eight** new §6 dependency rows; **§7.6** re-checking every prior owed item and adding seven. |
| `ARCHITECTURE.md` §5 | **DL-020** — refuse-don't-guess, discharged by an **independently authored fixture that can fail**, with the provenance order that forbids editing a fixture to match a parser, the **mirrored-pair corpus rule** it generalises to, why it is **one** entry and not three, and **three candidates considered and deliberately not filed**. DL-001…DL-019 untouched. |
| `SESSION_LOG.md` | This entry. |
| `NEXT_SESSION.md` | Rewritten for the post-evaluation-surface position. |

**Not touched, by instruction and by ownership:** `TOLERANCES.md`,
`tools/`, `fixtures/`, the corpus, `LEGAL.md`. **Nothing was
committed** — instructed not to, and committing is the engineer's act.
**No git command was run**, by an agent that has no shell.

### Left for the next session to not assume

- **That any of the three commits exists or contains what is recorded
  here.** The files are verified; the repository is not.
- **That "Pass 4's evaluation surface is complete" means anything is
  measured.** B2A has **one point**, `mAB ` has no real file, gray has
  **no cross-check at all**, and `Chain` is exercised by no test that
  reaches either new model.
- **That the new tests pass.** No run was reported. Five ledger rows
  carry asserted bounds and no outcome.
- **That GP-001 is closed everywhere.** The **code** is fixed; the
  **corpus sentence that caused it** is still there, and
  `gen-profiles/README.md` still says the finding is open.
- **That Pass 2 being DONE means the sweep is current.** It was run
  against a pre-GP-001 parser.
- **That A4b moved.** It did not — only the stake and the
  characterisation of lcms2's position did.
- **That anything ran on Linux, or that any CI run has ever been
  observed.** Still nothing, by anyone, ever.

---

## 2026-08-11 (autonomous-loop continuation) — ★★ Pass 4b: the three directions Pass 4 left unmeasured are measured, a closed form derived from clause text beats every LUT row in the ledger, and three lcms2 "rules" turn out to be half-rules about one direction

**Tenth entry of the same calendar day.** Filed by `icc-librarian`
against the working tree and the live corpus.

**Commits, all reported** *(no agent in this project has ever run git)*:
**`9e2e29e`** — the previous filing committed, a **gray-through-`Chain`**
test, and a GP-001 status banner in `tools/gen-profiles/README.md`;
**`a0310c7`** — three changes driven by the corpus's **seventh** pass
(the **normative `mAB `/`mBA ` matrix-output clamp**, the `offsetB == 0`
malformation now that **A23 is closed**, and the `mluc` `recordSize`
refusal reworded per the corpus's spec-defect §17); **`3d0c183`** — the
Pass 4b measurements (`tools/difftest/src/pass4b.rs`, `pass4b_report`,
README **§15**, `TOLERANCES.md` §3.4.4 and four rows in its §4).
**All three commits' contents were checked against the live source and
all three matched** — the first dispatch in several filings where that is
true of every item.

### ★★ What was measured — 28 records, `pass=28 fail=0`

*(reported; the summary line `summary pass=64 fail=0 skip=3 error=0` is
transcribed in README §15.5 and was read here, and the engineer
separately reports **re-verifying `pass=28 fail=0` within the hour** —
**without per-line output**. `icc-librarian` ran nothing.)*

- **§A — the B2A direction.** sRGB → `USWebCoatedSWOP.icc` through
  `mft1`/`lut8Type`, 213 RGB points end to end + 258 Lab points
  PCS-side, perceptual and media-relative. **1,330×10⁻⁴ device against a
  5×10⁻⁴ gate** — and the gate is an **envelope computed from lcms2's
  own roundings with no lcms2 output in it**, which the observation
  matches to **0,02 %**. Modelling those roundings leaves
  **3,10×10⁻⁵ = 2,03 lsb of 1/65535**, *three times independently*.
  **`lut8Type` evaluation and the `Lab8` codec had no evidence of any
  kind before today.**
- **§B — the v4 element pipeline.** One synthetic fixture, because a
  sweep of all **40** profiles in this machine's colour directory found
  **zero `mAB `/`mBA ` tags**. Its CLUTs are **affine**, so every
  geometry reproduces them exactly, and the output is a **closed form**
  derived from 10.12/10.13 and Tables 12/13. **iccce reproduces it to
  2,842×10⁻¹⁴ in `L*` and 2,220×10⁻¹⁶ in device.**
- **§C — the gray axis.** `ewgray22.icm` → sRGB, 69 points.
  **9,686×10⁻⁵ device / 2,169×10⁻² ΔE2000**, and modelling lcms2's
  `cmsReverseToneCurveEx(4096)` collapses it **457×** to
  **2,121×10⁻⁷ — below `transicc`'s print floor.** Worst point
  `g = 2/255`: iccce `0,000300`, lcms2 `0,000397`, **model `0,000397`.**

### ★★ A new evidence class, and the honest ceiling on it

§B introduces **`derived-expectation`** — an expectation computed by
arithmetic from the specification's stated element order and encoding
plus the bytes of a fixture this project authored, **with no
implementation's output in it**. It is **stronger than a cross-check**
(a cross-check dies when two implementations share a misreading; this
dies only when *the derivation* does) and it is **not ground truth**
(nobody at the CIE or the ICC printed the number). Its weakness is
stated wherever it appears: **the fixture and the derivation come out of
the same corpus**, so a wrong transcription makes them wrong *together*
and they agree perfectly — which is exactly why every derived row is
paired with an lcms2 row over the same points, **the third reading**.
The class was defined by `icc-conformance` in `TOLERANCES.md` §3.4.4.1;
this librarian's judgement was that a pointer was **not** enough and the
class belongs in `NUMERIC_CLAIMS.md` §1's table, because §1's own rule
is that **a row without a class is not finished** and four rows carry it.

**Pass 4 still has no ground-truth row**, and this does not become one.

### ★★ Three findings, and the third is why there is a new decision-log entry

1. **lcms2 does NOT use tetrahedral interpolation in the B2A
   direction.** `_cmsReadOutputLUT` calls
   `ChangeInterpolationToTrilinear` for **every** Lab-PCS LUT — its own
   comment calls it *"controversial stuff"* — and **trilinear over three
   inputs is iccce's n-linear**. So **NA-006's measured 1,5741 ΔE2000 is
   an A2B fact, and the B2A envelope is exactly zero.** A zero method
   difference makes the comparison **weaker**, which is why the
   counterfactual row exists: the same table evaluated tetrahedrally
   differs by **99–139×** the observed disagreement, so the apparatus is
   *shown* able to see a geometry difference (DL-018's discipline,
   applied to a method).
2. **Forced BPC is decided by the DESTINATION profile's version.**
   Measured in both directions on one pair, both sides lcms2: v4 as
   *source* into a v2 destination is **bit-identical**; v2 into a v4
   *destination* moves `K` at black **99,6094 % → 96,4721 %**.
   **DL-013 and corpus M2 as written are half a rule** — anyone using
   them to decide whether a comparison is confounded needs the
   direction.
3. **The encoded PCS overflows, and it costs 0,6117 ΔE2000 on 10 of 128
   points.** The 3×4 matrix adds `+1/256` to a full-scale `L*` node;
   iccce clamps, lcms2 does not (`100,390 625`, measured through
   `transicc`). Handled per **DL-019**: **reported, not graded**, and
   the ten points **excluded** from the graded rows that would otherwise
   contain them.

**The meta-finding, and it is the entry:** three lcms2 behaviours, all
in the same file, **all previously written down in this project as
unqualified rules**, each turning out to be a statement about **one
direction or one tag type**. (The third: the legacy-Lab encoding, which
lcms2 applies for `lut16Type` and **not** for `lut8Type` — where iccce's
`Lab8` codec agrees exactly, and where the mistake would have cost
**0,39 % in `L*` ≈ 0,2 ΔE2000**, *below* the perceptibility anchor and
invisible to any ΔE-graded suite.) Filed as **`ARCHITECTURE.md`
DL-021**: *a measured implementation behaviour is a fact about the
direction and the path it was measured in, until it is measured in the
others.* **The defect is in this project's transcription, not in
lcms2** — each behaviour has a rationale in its own place.

### ★ A clause question that the corpus answered between the run and this filing

README §15.3.3 records the overflow as unsettled and owes a dispatch
asking **two** questions. **The first is already answered.** The
corpus's seventh pass transcribed **10.12.5/10.13.3 VERBATIM** — *"The
resultant values Y1, Y2 and Y3 **shall be clipped to the range 0,0 to
1,0**"*, used as inputs to the `B` curves — and glosses it: *"clipping
here is normative and is one of the few places ICC.1 says where clipping
happens."* **The fixture's overflow is exactly a matrix output**, so
**iccce's `L* = 100` is what the clause requires**, and iccce's live
code now clamps there **citing that clause** (`a0310c7`, verified).
**What remains open is the second question** — whether the *final* `B`
output must be clipped to the encodable PCS range — so the queued
dispatch is **narrowed, not cancelled**; and per **A39b** the available
word for lcms2's behaviour is **divergence**, not non-conformance.
**Re-grading the row is `icc-conformance`'s call on its own file.**

*This is the librarian rule that keeps paying: **verify against the live
source rather than the dispatch.** The dispatch carried the question as
open; the corpus had closed half of it, and the engineer had already
acted on the closure in code.*

### ★ Three tolerances failed first and were RE-DERIVED, not widened

`TOLERANCES.md` §4 logs four Pass 4b rows *(read; not edited — the file
is `icc-conformance`'s)*. **C1**: an envelope written into a doc comment
**before it was computed** (3,45×10⁻⁵ guessed, 9,680×10⁻⁵ computed).
**C3**: **a derivation taken at the wrong end of the axis** — near
black, CIELAB's *chromatic* sensitivity dominates by ~3×, which
**inverts** a note this project carried from Pass 3. **B6**: **a missing
term** — B5 ends at a CLUT, B6 ends at sRGB's inverse tone curves; **the
fix is a second constant, not a bigger one**, and B5 keeps its number
and still passes. **B0**: ★ **real arithmetic mistaken for floating
point** — *"every geometry reproduces an affine function exactly"* is
true **in ℝ**, and the two algorithms reach it by different sequences of
`f64` operations; 0,0 → 1×10⁻¹⁴, failed at 1,110×10⁻¹⁶. **In every case
the code was cleared first**, per §0's procedure.

### ★ Two things in the tree that the dispatch did not mention — the fifth consecutive filing

**`crates/iccce-cmm/src/bpc.rs` (Pass 5) and `named_color.rs` (Pass 7)
both exist**, are declared in the crate's `lib.rs`, and carry 4 and 2
tests *(verified — read and counted)*. The corpus carries a matching
**`icc__ref__bpc.md`**, whose headline is that **the BPC scaling map is
in ICC.1:2022 after all**, at clause **6.3.4.3** under another name. So
**the dispatch's *"Pass 5 pending sourcing"* is wrong on live
evidence**: the sourcing landed **and Pass 5's code half is largely
done** — `Chain::with_bpc()` applies it and **`iccce transform --bpc`**
reaches it through the shipped binary, refusing by name at the absolute
intent and outside the estimation subset. **iccce NEVER forces BPC**,
deliberately, and the field doc calls that *"a recorded policy
difference from the oracle"*; **NC-078 has already priced one direction
of it**. **What Pass 5 is missing is measurement**: `TOLERANCES.md`
§3.5's blank rows are now a **gap**, not a correct absence. Two register
entries were owed the moment that code existed and are filed with this
session — **NA-009** (the black-point *estimation* subset, corpus A42)
and **NA-010** (the perceptual-black constant, corpus A41: iccce follows
lcms2 **and ICC's own iccDEV** against ICC.1 Table 16's printed
decimals, at a corpus-derived **0,037 ΔE76** that is **exactly zero on
any 16-bit PCS path**) — and **because the path is reachable, both costs
are OWED rather than merely registered.** **`NamedColors`, by contrast,
really is reachable from nothing** *(verified — the whole tree grepped
with no result limit)*.

**★ And this filing's own error, recorded rather than fixed silently:**
the first draft of that paragraph said **both** modules were wired into
nothing. It came from a **head-limited grep** — the first N matches, not
the file's whole story — and the `--bpc` flag was in the truncated tail.
**A truncated search is not an inventory**, which is the same rule as
*a count is not an inventory* wearing a different hat, and it is the
second time in three filings that this librarian's own draft has carried
a wrong statement about the tree. **The rule that caught it both times
is the same: check the live source, including your own sentences.**

**Also reported, not repaired:** `iccce-cmm/src/lib.rs`'s §Status is
stale for the **fourth** time — *"Still to come: **BPC (Pass 5)**"*, in
a crate that wires BPC into `Chain` — **but its own standing
instruction** (*"if a module below contradicts it, trust the module"*)
means a reader following it is not misled. **That is the strongest
argument yet for that style of fix.**

### ★ A prediction of this project's own, falsified by the direction a run took

Three documents predicted that a gray differential would give **NA-008**
its first measurement. **It ran, and it did not** — §C runs gray as the
**source**, and NA-008 is a property of the gray **destination** path.
*"A gray differential"* named a comparison, not a direction. **That is
DL-021's shape appearing in this project's own writing rather than in
lcms2's behaviour**, and it is recorded as a dated note under NA-008
rather than by editing the prediction.

### A discrepancy recorded unresolved, and a process slip recorded rather than smoothed over

- **The build commit.** README §15.5 says the binary was built at
  **`97ad9fa`**, which **predates all three commits above**, including
  the matrix-output clamp that touches the very code path the overflow
  finding is about. Either the run predates `a0310c7` and the line is
  accurate, or the line is stale. **The ten overflow points are excluded
  from every graded row**, so `pass=28` cannot turn on it — but **nobody
  may say these numbers came from the code in the tree today.**
- **The cwd-relative pathspec trap recurred.** The engineer reports
  hitting it a **second** time, from `tools/difftest` — the same shape
  that swept an untracked working state into `edcb60e` at the Pass 4
  filing — and **caught it before committing this time** *(reported)*.
  Recorded because a near-miss that is not written down is
  indistinguishable from a trap that has gone away.

### Filed this session

| Where | What |
|---|---|
| `ROADMAP.md` | A header status paragraph; a **Pass 4b progress block** — the three sections measured, the derived-expectation headline, the three findings, the four re-derived tolerances, **the done-when re-answered clause by clause** (B2A/gray **met on stated terms**, v2/v4 **met on stated terms**, saturation and ICC-absolute **not met**), the build-commit discrepancy, the undispatched modules, the gates and the remaining owed items; a **second Pass 5 annotation** (sourcing landed, `bpc.rs` written, NA-009/NA-010, and the direction-keyed BPC warning); a **Pass 6 annotation** (rule 8's precondition, and DL-018/DL-021 as its inherited method rules); a **Pass 7 annotation** (`named_color.rs` exists, unwired, and DL-005 applies to it). **No plan text, no annotation and no earlier block rewritten.** |
| `NUMERIC_CLAIMS.md` | **§1** — the **`derived-expectation`** class added to the table, with what it cannot do; **§2.7** provenance (three commits, the **first reported gate** in three filings, the build-commit discrepancy, and the method that produced the tolerances); **§3.11** — **NC-062 … NC-083**, a shared coverage box, the record-to-row arithmetic, and six subsections including the encoded-PCS overflow with **the corpus correction to its own framing**; dated notes under **NA-006** (direction-dependence) and **NA-008** (the differential ran the other way); **NA-009** and **NA-010**, both filed from code the dispatch did not mention; **ten** new §6 dependency rows; **§7.7** re-checking every prior owed item — **five discharged** — and adding six. |
| `ARCHITECTURE.md` §5 | **DL-021** — a measured implementation behaviour is a fact about **one direction and one path**, with the three instances tabulated against what this project had written, five conjunctive clauses, why it is one entry and not a note under NA-006, and what it does not claim. Plus a **dated status note under DL-020**: its first revisit condition has **fired** (the per-type transcription landed, A23/A25 resolved), so clause 5 is discharged for that instance. DL-001 … DL-019 untouched. |
| `SESSION_LOG.md` | This entry. |
| `NEXT_SESSION.md` | Rewritten for the post-Pass-4b position. |

**Not touched, by instruction and by ownership:** `TOLERANCES.md`,
`tools/`, `fixtures/`, the corpus, `LEGAL.md`. **Nothing was
committed** — instructed not to, and committing is the engineer's act.
**No git command was run**, by an agent that has no shell.

### Left for the next session to not assume

- **That any of the three commits exists or contains what is recorded
  here.** The files and the corpus are verified; the repository is not.
- **That `pass=28 fail=0` covers the tests.** It is the **differential
  runner's** result. **No `cargo test --workspace` count came with this
  dispatch**, so **NC-057 … NC-061 still have no reported outcome at
  all**, five filings on.
- **That the measured numbers came from today's code.** See the
  build-commit discrepancy.
- **That "the B2A direction is measured" includes every intent.**
  **Saturation and ICC-absolute were not run in any of the three
  directions**, and `B2A2` is a genuinely distinct third table.
- **That §B says anything about real v4 profiles.** It is **one file
  this project wrote**; there is **no real `mAB `/`mBA ` profile on this
  machine at any price**.
- **That the gray comparison priced NA-008.** It did not — wrong
  direction.
- **That agreement with lcms2 in B2A validates iccce's interpolation.**
  The method difference there is **zero by lcms2's own override**; the
  comparison shows sameness, not correctness.
- **That Pass 4's done-when can be closed by engineering.** The
  ICC-absolute clause is blocked on **A4b**, and only `ICC.1:2001-04`
  settles it — an **operator** download.
- **That anything ran on Linux, or that any CI run has ever been
  observed.** Still nothing, by anyone, ever.

## 2026-08-11 (autonomous-loop continuation) — ★★ Pass 5 CLOSES: the BPC map is graded against a clause of the primary specification, the best finding of the Pass is something the run could not do and said so first, and TWO commits shipped red under messages claiming a green suite

**Eleventh entry of the same calendar day.** Filed by `icc-librarian`
against the working tree, `tools/`, the live corpus and
`C:\personal_rag\`.

**Commits, all reported** *(no agent in this project has ever run git)*:
**`8be1ed3`** (the Pass 4b filing committed + the `iccce-cmm/src/lib.rs`
§Status fix), **`70411dd`** → **`a36abaf`** and **`6ea1b3d`** →
**`812a215`** (the BPC core, and the two red commits below),
**`46f16e8`** (the `--bpc` CLI — the iccce commit README §16 names),
**`df3a233`** (the Pass 5 measurements). **Verified in the working
tree**: `bpc.rs` wired, `--bpc` on the CLI, `tools/difftest/src/pass5.rs`
+ `src/bin/pass5_report.rs` present, README §16 and `TOLERANCES.md` §3.5
filled — **and `lib.rs`'s §Status now correct**, closing a staleness this
project reported **four times**.

### ★★ The done-when is MET, and the terms are the interesting part

- **"Differ in the documented direction"** — met **with no tolerance at
  all**. `out − in = (Xd − Xs)/(Xi − Xs)·(Xi − X)`, whose second factor
  is `≥ 0` in gamut, so the **sign** is provable at every point: **0,0
  exactly** in both directions (largest fall **4,304×10⁻² device =
  3,5159 ΔE2000**).
- **"Match lcms2's BPC within tolerance"** — **1,110 588×10⁻⁴** device
  out of the fixture, **4,600×10⁻⁵** into it (against **both** lcms2
  arms), **1,262 374×10⁻² ΔE2000**, on a **BPC-off baseline graded
  first** at 1,012 157×10⁻⁴.
- **And a third thing nobody asked for**: the **scaling map** against
  **ICC.1:2022 6.3.4.3**'s printed equation at **1,110×10⁻¹⁶**, and
  against a Gaussian elimination on **Maria (2013) §4.2** at
  **3,331×10⁻¹⁶**. Three independent statements of one map, ~1,5 ulp
  apart.

### ★★ The finding of the Pass is a NEGATIVE result, and it was derived before anything ran

Both implementations' reach was read out of their sources first —
`Chain::with_bpc`'s subset against lcms2's six first-match-wins guards
at the pin — and the intersection said, **in advance**:

> **Everywhere iccce will do BPC at all, lcms2's estimator reduces to
> the same two values** — `XYZ (0,0,0)` on every matrix/TRC or gray side
> in reach (`trc(0) = 0` everywhere), the **same A41 triple** on a v4
> LUT side at perceptual.

**So Pass 5 grades the map, the direction and the pipeline, and cannot
discriminate the two ESTIMATORS.** A session that had measured first
would have found six small numbers and read them as six independent
agreements about "BPC". **When two implementations agree, the question
is what they were free to disagree about** — and it is answerable from
their sources, before the run, more cheaply than the run itself. Filed
as **DL-023**, with its cheap companion: **print the sensitivity
ratio** (Pass 5's are **388×** and **682×**; the off arm is already
being run as the baseline, so it costs nothing).

**The instrument that would close it does not exist**: a synthetic v4
RGB-or-gray LUT fixture with a **non-zero device black**.
`fixtures/synthetic/` holds 38 `.icc` *(verified — enumerated)*, one v4
LUT, black zero. **Owed to `tools/gen-profiles`** — the same shape as
the GP-001 arc.

### ★★ Two recorded differences that are not defects

1. **lcms2 silently does NO BPC below a threshold.** `IsEmptyLayer`
   drops the whole stage when the BPC matrix's deviation from the
   identity plus its offsets is under **0,002** — so lcms2 stops doing
   BPC once the two blacks are within ≈**0,41 `L*`**. **iccce has no
   such threshold.** For the S2/S3 map the discriminant is **0,015 342,
   7,7× the threshold**, so nothing measured turns on it, and **the
   0,41 figure is SOLVED FOR, not observed** — recorded at that strength
   and no higher. **The constant is not in the corpus**, because
   `ICC_Spec` §7.2's list came from `cmssamp.c` and this one is in
   `cmscnvrt.c`.
2. **★ iccce NEVER forces BPC; lcms2 forces it for a v4 destination at
   perceptual.** Unasked against unasked: **3,137 348 `L*`**, lcms2
   lighter. **Neither is a defect — the number IS the policy**, and it
   is **REPORTED, NOT GRADED** under DL-019, because the enable policy
   rests on a document nobody here has read and the one published source
   (Maria 2013) is silent on it. The corpus's **D11** fingerprint is
   *answered*: the size matches the A41 triple's `L*` to 1,1×10⁻⁴ and
   **the sign identifies lcms2's M2 route, not iccDEV's** — distinguished
   by measuring the *other* direction, which is DL-021's discipline
   applied to a policy. **Promoted to `ARCHITECTURE.md` DL-022**, out of
   the paragraph in NA-009 where it had been living, because it now has
   a measured size, a graded posture and **a user-visible consequence**:
   two correct CMMs give different pictures by default, silently.

### ★★ The double red-commit incident, recorded with both false claims named

**Two commits this session shipped with a failing test under a message
claiming a green suite**, and the second happened *after* the lesson for
the first had been written:

| Commit | The false claim | The gate that let it through | Corrected in |
|---|---|---|---|
| **`70411dd`** | *"102 workspace tests green"* — **one was red** | `cargo test … \| grep -E 'test result: ok. [1-9]\|FAILED' && git commit` — **grep exits 0 on a `FAILED` match** | **`a36abaf`** |
| **`6ea1b3d`** | *"104 green"* *(the number is the **dispatch's**; the lesson file does not carry it)* | the "fixed" gate `cargo test -q 2>&1 \| tail -2 && commit` — **`tail` exits 0**, masking cargo's 101 | **`812a215`** |

**Both were corrected honestly rather than quietly**, and the lesson is
written up at
`C:\personal_rag\claude_code\lesson_20260811_grep_on_test_output_matches_failed_lines_with_exit_0.md`
*(**verified — read**)*. **It names all four commits and the "102"
claim, and it records that its own author fell for the pipeline variant
minutes after writing it** — which is why the remedy it states is
**mechanical, not attentional**: capture the exit code before any
display, `cargo test --workspace -q > log 2>&1; TESTS=$?`, gate on
`$TESTS`. It also cross-references the older
`lesson_20260807_pipeline_exit_status_belongs_to_the_last_element.md`,
whose own amendments already record **four** instances of the same idiom
in one day in the sibling project *(verified — read)*. **Exit codes
compose; text matching does not, and neither does a pipe.**

**What this costs the ledger, and it is not nothing.** Every gate figure
this project holds is *reported*, and this session demonstrated that a
reported figure can be produced by a gate that was never actually read.
**§2.8 records it in the provenance block** — the right place, because a
provenance block is where a reader decides how much a number is worth —
and **no §3.12 row inherits any gate claim from a commit message.** The
one gate figure §3.12 rests on is the whole-suite
**`pass=90 fail=0 skip=3 error=0`** transcribed at the README's head.

### ★ Three things this filing found by reading rather than transcribing

1. **★ A labelling error in the dispatch.** It described the lcms2 match
   as *"map 1.11e-16 …; **policy arm 4.6e-5**"*. **4,600×10⁻⁵ is not
   the policy arm** — it is **NC-096**, the BPC-on cross-check in S3.
   **The policy row is NC-100 at 3,137 3×10⁻², REPORTED NOT GRADED.**
   They differ by ~680× **and by their posture**, which is the whole
   point of DL-019. Understandable (§16.4 is titled *"…and the
   policy"*), corrected rather than absorbed.
2. **★ `tools/difftest/src/pass5.rs` has NO unit tests** *(verified
   twice — `tools/` grepped for `#[test]` with **no result limit**,
   returning `pass3.rs` 7, `pass4.rs` 7, `pass4b.rs` 8 and **no
   `pass5.rs`**; then `pass5.rs` grepped alone)*. Pass 3 pinned its grid
   with five tests and Pass 4 asserted its corners really are corners.
   **Fourteen of the new rows rest on two grids that nothing pins**, and
   a silently changed grid would silently change their scope with
   nothing failing.
3. **★ §16 states no `pass=`/`fail=` line of its own**, unlike §15's
   `pass=28 fail=0` *(verified — §16 read end to end)*. **Pass 5's
   record count is this librarian's subtraction** of two reported
   whole-suite totals (90 − 64 = 26), and it reconciles exactly with
   §3.12.1's row enumeration (5 + 7 + 8 + 6). **A reconciliation that
   comes out right is not a report**, and the block says so.

### ★ A class judgement, made rather than inherited

The dispatch suggested the map row might be *"the ledger's first
primary-spec-conformance row for a TRANSFORM"* and asked for a judgement.
**The answer is no, on three grounds**, and **NC-084 is filed as
`derived-expectation`** (§3.12.2): (a) §1's `normative-rule-conformance`
class requires the corpus at **`primary_spec`** tier, and
`icc__ref__bpc.md`'s `evidence:` line reads **`cross_verified_2src`**
for §2/§3 *(verified — frontmatter read)*; (b) the ledger's first
normative-rule-conformance rows are **NC-022 … NC-027**, filed at Pass
3; (c) **it is not a transform** — it grades the map function
`BpcScale`, while the end-to-end row is **NC-098**, whose expectation has
a fixture's bytes in it. **A class is not raised by how good the number
looks.** What NC-084 *is*, at full strength: a **derived expectation with
no fixture in it at all** — which removes the class's stated weakness
and leaves only transcription risk. **Promotion is one line from
`icc-spec-librarian`**, and the **DL-014 audit item now decides a ledger
class**, not just a doc-comment heading: `bpc.rs` still heads 6.3.4.3
**"PRIMARY-SOURCED"** *(verified — read)*.

### ★ One named approximation's cost discharged, one shown to be undischargeable

- **NA-010 — MEASURED.** NC-094 rebuilt the map with **ICC.1 Table 16's
  printed decimals** on §B's grid and reproduced the corpus's two Python
  passes **to 2×10⁻⁵ ΔE76** by an independent route (Rust, a fixture's
  stored bytes), **plus a ΔE2000 the corpus never computed: 0,050 201**
  — the **same order as §B's entire agreement budget**, so on a float
  path the constant is *not* negligible against the noise. **This is the
  verification loop running the direction it ran at Pass 1** — and this
  time the corpus was right.
- **NA-009 — still unmeasured, and now for a stated reason.** Being
  *reachable* is not being *discriminable*. The §6 dependency row that
  predicted *"wiring makes the cost come due"* was **half wrong**, and
  its dated status says so.

### Filed this session

| Where | What |
|---|---|
| `ROADMAP.md` | A header status paragraph; the **Pass 5 completion record** — the done-when answered clause by clause with the estimator boundary in the same table, the pre-registered negative result, the two non-defect differences (`IsEmptyLayer`, the never-force policy), the coverage statement, the gates including the three read-not-transcribed findings, the dispatch's labelling correction, and six owed items; a **second Pass 6 annotation** (rule 8's precondition now met for every stage a compiled transform would touch, and DL-023 as the sharpest inherited rule). **No plan text, no earlier block and no annotation rewritten.** |
| `NUMERIC_CLAIMS.md` | **§2.8** provenance (six commits, the red-commit row, the `pass=90` subtraction, the `pass5.rs` pinning gap); **§3.12** — **NC-084 … NC-104**, a shared coverage box, the record arithmetic, and ten subsections including the **class judgement**, the **pre-registered negative result**, the closed form and its third reading, the policy, the `IsEmptyLayer` threshold, A41 priced, the two graded refusals, and *what §3.12 does not claim*; **dated notes under NA-009** (cost still unmeasured, and why) **and NA-010** (cost MEASURED — its *"nobody may restate it as an iccce measurement"* clause superseded); **eleven** new §6 dependency rows; **§7.8** re-checking every prior owed item — **one discharged, one split** — and adding six. |
| `ARCHITECTURE.md` §5 | **DL-022** — iccce never forces BPC: the measured size, the three reasons it is a decision rather than a note (user-visible consequence, contaminates every comparison, previously written down twice in weaker places), and four things it does not claim. **DL-023** — state what the two implementations were **free to disagree about**, from their sources, before the run; publish the negative result; name the instrument that would close it; print the sensitivity ratio. DL-001 … DL-021 untouched. |
| `SESSION_LOG.md` | This entry. |
| `NEXT_SESSION.md` | Rewritten for the post-Pass-5 position. |

**Not touched, by instruction and by ownership:** `TOLERANCES.md`,
`tools/`, `fixtures/`, the corpus, `LEGAL.md`. **Nothing was
committed** — instructed not to, and committing is the engineer's act.
**No git command was run**, by an agent that has no shell.

### Left for the next session to not assume

- **That any of the six commits exists or contains what is recorded
  here.** The files, `tools/`, the corpus and the personal_rag lesson
  are verified; **the repository is not**, and this is the session that
  proved a commit message can be false.
- **That `pass=90 fail=0` covers `cargo test`.** It is the
  **differential runner's** whole-suite figure. **No `cargo test
  --workspace` outcome has been reported at any of the last six
  filings**, and **NC-057 … NC-061 still have none.** 103 `#[test]`
  declarations exist under `crates/` *(verified — counted)*; that is not
  a pass result.
- **That "iccce's BPC matches lcms2's" is a statement about BPC.** It is
  a statement about **the scaling map, the direction and the pipeline**,
  on **one synthetic fixture**, at **one intent**, in **two
  directions**. **Neither ESTIMATOR was tested, by either side.**
- **That the estimator gap is a hedge.** It is a **derived** result with
  a named instrument that does not exist yet.
- **That lcms2 always does BPC when asked.** Below `IsEmptyLayer`'s
  0,002 it does none at all — **solved for, never triggered here**.
- **That NC-100 is a defect in either implementation.** It is a policy,
  and it is ungradable until an operator fetches a document.
- **That Pass 4 moved.** It did not: **saturation in B2A** and
  **ICC-absolute** (blocked on **A4b**) are exactly where the Pass 4b
  filing left them, and Pass 5's saturation gap is a **different** item.
- **That anything ran on Linux, or that any CI run has ever been
  observed.** Still nothing, by anyone, ever.

---

## 2026-08-12 — ★★ THE PROJECT IS PUBLIC, Pass 6 and Pass 7 CLOSE, and the librarian's oldest constraint loosens: this project's own repository became a readable source, and the first thing it did was catch a wrong hash carried in three documents

**The first entry of a SECOND calendar day** — the previous eleven are
all 2026-08-11. Filed by `icc-librarian` against the working tree, the
plain-text log and ref files under `.git/`, and the live corpus.

> **★ The dispatch that commissioned this filing was headed
> 2026-08-11, and it is 2026-08-12.** Three independent readings:
> `.git/logs/HEAD` timestamps the first of the four commits at epoch
> `1786527689 -0400` = **2026-08-12 05:41:29 −04:00**, and the other
> three between 06:20 and 06:55 the same morning *(verified — read)*;
> the environment reports 2026-08-12; the corpus's ambiguity register
> carries `revised: 2026-08-12` *(verified — read)*. **Corrected, not
> followed.** It is not cosmetic: eleven filings assert *"the same
> calendar day"*, and a twelfth would have made that assertion false.

**Commits, and for the first time in this project's history the hashes
are corroborated by something in the repository rather than reported
alone:** **`bb5d6b8`** (the A4c disclosure), **`0378f76`** (ISO/CD
18619:2013 estimation), **`3502cb7`** (Pass 6), **`f6203b8`** (Pass 7
wiring). **All four appear in `.git/logs/HEAD` with these subject
lines** *(verified — read end to end)*. **No git command was run** — a
reflog is a text file, and reading one is not running `git`. **The
contents of all four commits remain unverified.** Verified in the
working tree: `compiled.rs`, `cmd_bench`, `resolve_to_device`,
`convert_pcs_to_device`, `pcs_to_destination` and `Chain::convert`'s
call to it, `white_point_note`, and `bpc.rs`'s ISO/CD 18619 header.

### ★★ Publication — the event, and the line between evidence and report

`master` was pushed to **`https://github.com/KenM76/iccce`** on the
operator's explicit go-ahead *(the go-ahead itself is **reported**; no
agent here holds its wording)*. Filed as **DL-024**.

**What was read, not reported:**

| File | What it says |
|---|---|
| `.git/config` | `origin` = `https://github.com/KenM76/iccce.git` |
| `.git/logs/refs/remotes/origin/master` | **Two lines, both `update by push`.** `0000000…` → **`3502cb7`** at **06:51:17 −04:00** — the all-zero left side means **the remote branch did not exist before it**, so *this line is the publication event* and it carried the entire history through Pass 6. Then → **`f6203b8`** at **06:54:50 −04:00**, three seconds after Pass 7 was committed |
| `.git/refs/heads/master` · `.git/refs/remotes/origin/master` | Both **`f6203b8d…`** — nothing local is unpushed |

**★ What this does NOT establish: that the repository is PUBLIC.**
Visibility is a server-side setting; **a push to a private repository
produces an identical reflog**, and no file in this tree records it.
**Public is the operator's report**, and is carried at that strength
throughout this filing.

**The three pre-publication checks, and they are not equal:**

1. **No vendor profiles tracked — ★ VERIFIED, and more strongly than
   claimed.** Every `.icc`/`.icm` in the tree was enumerated: **38 in
   `fixtures/synthetic/`**, all generator output; the rest under
   `tools/difftest/vendor/` (ignored by name) and `tools/difftest/out/`
   (caught by `.gitignore`'s `*.icc`, negated only for `fixtures/**`).
   **`fixtures/reference/PROVENANCE.md`'s table reads "(none yet)"** —
   no third-party profile was ever admitted, so there was nothing to
   miss *(verified — globbed and read)*.
2. **The corpus is not in the repo — VERIFIED as to the file tree.**
   `ICC_Spec` is outside it, which is what LEGAL §2.1 requires.
   **Not checked: whether any corpus file was ever committed and later
   removed** — history keeps deletions, and that is a `git log`-shaped
   question.
3. **Spec quotation is short-with-citation per LEGAL §2.1 — REPORTED,
   and this is the one not to round up.** §2.1's rule was verified as
   *text*. **That the whole tree complies has never been audited**, by
   anyone. **DL-014's citation audit is that audit**, it has been owed
   since Pass 3, and it now underwrites a **published** compliance
   claim.

### ★★ The first thing the new evidence caught: a wrong commit hash, carried three times

The commit *"untrack tools/gen-profiles"* is **`edce48b`**
*(`.git/logs/HEAD` line 25 — verified, read)*. **`ROADMAP.md` (×2), this
log (×3, at lines 1406, 1590 and 2052) and `NEXT_SESSION.md` all carry
`edcb60e`, which matches no prefix in the reflog.**
**`NUMERIC_CLAIMS.md` §2.6 has it right**, because it arrived by a
different route — a transcription of `gen-profiles/README.md` §6.

**This entry is the correction; the earlier entries stand unedited**, as
this log's own header requires. `NEXT_SESSION.md` is rewritten and
carries the right hash. **The lesson is the project's own, in a new
place:** a hash typed from a screen is a claim, **it looks exactly like
a right one**, and it survived three documents because nothing could
check it. Something can now.

**And a discrepancy left open, deliberately.** The dispatch reports **49
commits**; `.git/logs/HEAD` holds **45 lines** — one `commit (initial)`
and 44 `commit` lines, with **no `reset`, `rebase`, `amend`, `checkout`
or `merge` entry anywhere**. **Neither number is asserted here.**
Nobody has run `git log`; one command settles it.

### ★★ Pass 6 — the done-when is MET, and the Pass's real work was making the second number mean anything

- **Clause 1 — a stated time.** **8 700 867 px** (2481 × 3507, A4 at 300
  DPI) in **7.23 s** = **1.20 Mpix/s**; grid build (83 521 chain
  evaluations) **1.04 s**; reference path **0.084 Mpix/s** in-process;
  **speedup 14.4×**. **NC-105 … NC-107**, in a new evidence class,
  **`machine-timing`** — which is not weak correctness evidence, it is
  *not correctness evidence*, and **lcms2 was never timed**, so the
  ratio is iccce against **iccce**.
- **Clause 2 — a stated error.** **0.003589 device units**, maximum
  **off-node**, on **SWOP `A2B1` (`mft2`, 4-D) → sRGB matrix/TRC,
  media-relative, 17-point grid** (DL-021 named in the row).
  **`self-consistency`** — both arms are iccce — and `iccce bench`
  **prints that sentence in its own output**, which is the right place
  for it. **NC-108.**

**★ The sensitivity control caught its own instrument, twice**, and both
failures are in the test's doc comment *(verified — read)*:

| Draft | Returned | What was wrong |
|---|---|---|
| **1** — fixture **sRGB→sRGB** | **1.1×10⁻¹⁵**, ratio **0.94** | **The FIXTURE nullified the control.** A grid reproduces an identity chain **exactly everywhere**, not merely at nodes — `n`-linear interpolation is exact on a linear function. No `h²` scaling, no discrimination. **That 1.1×10⁻¹⁵ would have been reported as the compiled path's cost** |
| **2** — sRGB→**AdobeRGB**, whole axis | ratio **1.44** | **Neither code nor fixture.** sRGB's TRC kinks at 0.04045, and error across a derivative discontinuity scales `h¹`. **A correct control disagreeing with an incorrect expectation** — not a band being widened. Fixed by probing `[0.2, 0.9]`, off-node for both grids |

**★ DL-023 predicted this trap by name at the previous filing** — the
ROADMAP's second Pass 6 annotation calls a compiled-vs-reference row
*"the most likely null-by-construction row this project will ever
write"* — **and it was walked into anyway.** Filed as **DL-025**, with
the observation that this is the **third** instrument in two days to
catch something a competent engineer was about to ship:

| Pass | About to ship | Caught by | Filed |
|---|---|---|---|
| 3 | a curve evaluator **off by one sample** | an **exact-value** test; the round trip *would have passed* | **DL-016** |
| 4 | an `mBA ` curve count the corpus could not supply | a **refusal by name**, discharged by an **independently authored fixture** (GP-001) | **DL-020** |
| 6 | an error of **1.1×10⁻¹⁵** that measured nothing | a **sensitivity control**, which failed on its own fixture | **DL-025** |

**The generalisation is not "be careful."** In this project the thing
that catches an error is **never** a re-reading of the code and **never**
the number looking wrong — 10⁻¹⁵ looks magnificent. It is always **an
apparatus built to fail**. Rule 1's corollary: **a wrong measurement
looks exactly like a right one.**

**And one measurement thrown away, recorded because it matters.** A
first reference timing measured the CLI end to end at **≈49 000 px/s** —
that is **stdio text parsing**, not either transform. ≈49 k and 84 k are
the same order of magnitude, so **the speedup would have read ≈24×
instead of 14.4× and nothing about it would have looked wrong.** The
timing is now in-process. **NC-107 / §3.13.5.**

### ★★ Pass 7 — the spot reaches a real destination through the ORDINARY machinery, and a finding this project filed TWICE is closed

`NamedColors::resolve_to_device(name, dst)` →
`Chain::convert_pcs_to_device` → **`Chain::pcs_to_destination`**, *which
is the same method `Chain::convert` uses for its own destination half* —
the duplicated arm was removed, and `convert` now ends
`self.pcs_to_destination(xyz)` *(verified — read)*.

**That last arrow is the Pass.** A spot inherits the sourced 8.10.2
fallback, the same model selection, the same refusals and the same
clamping **by construction**, so **it cannot drift from the rest of the
CMM without the rest of the CMM drifting too**. The failure mode a
private path invites is not a crash — it is a `Separation` rendering
0.4 % off from every other object on the page, on some profiles only.
**Spot colours are brand matching**, which the module names as *"the
least acceptable place in the whole system for a sub-perceptual
defect."*

- **Unknown name → `None`**, the `/Alternate` signal, **not a guess**
  (**NC-112**).
- **Media-relative by construction** — Table 66 makes the spot's PCS
  relative colorimetric, so **no intent choice arises** and there is
  nothing for a caller to get wrong.
- **The legacy encoding asserted by exact integers, never ΔE**:
  `0xFF00/0x8000/0x8000` → `Lab(100, 0, 0)` **exactly**; the v4 decode
  gives **99.6109** — invisible to any ΔE gate, fatal to a brand colour
  (**DL-005**, **DL-016**).
- **`NamedColors` was referenced by nothing outside its own file** — a
  finding filed at the Pass 4b annotation and **repeated** at the Pass 5
  filing. **A test now resolves every spot in the committed `ncl2`
  fixture into the real system sRGB profile** (**NC-111**). **Closed.**

**★ What NC-111 is not.** It asserts a **range**, not a colour: a
resolution wrong by 10 ΔE but inside `[0, 1]` passes it. **No spot's
resolved value has ever been compared to anything.** And the cheapest
genuine cross-check was available and not taken — an `ncl2` entry
carries **`nDeviceCoords`**, the device values *the profile's author*
recorded, so resolving into the spot's **own** profile and comparing
would be an expectation iccce did not write. Owed.

### ★ Two commits belonging to earlier Passes, and the operator is why

- **`bb5d6b8` — A4c, and A4b is RESOLVED.** `ICC.1:2001-04` **A.3.1.1**
  addresses the profile's **AUTHOR** and is **silent on readers**, so
  the clause that was supposed to adjudicate an 11.2 ΔE2000 divergence
  **does not address the question**. iccce keeps `wtpt` **as stored**
  and **discloses** the inconsistency (`white_point_note`, detectable
  from the file's own bytes at 1×10⁻³ per component). **★ And the
  empirical finding is bigger than the disclosure**: a test written to
  show the note stays silent on a coherent profile **failed**, and the
  sweep that followed found **AdobeRGB1998, AppleRGB, PAL_SECAM,
  SMPTE-C, ewrgb18, ewsrgb and the stock sRGB** all storing `wtpt` = D65
  with colorants summing to D50 and no `chad`. **Seven profiles: the A4c
  configuration is the v2 authoring NORM**, which is why lcms2
  substitutes D50 and why iccce's disclosure will fire constantly and
  must be worth reading. **A4c is SILENT and does not clear when A4b
  clears.** The corpus register now has **one UNVERIFIED row in total
  (A31)** *(verified — read)*.
- **`0378f76` — `A42` upgraded on ISO/CD 18619:2013.** The operator's
  download turned out to be the **committee draft, not WP40**, and
  `bpc.rs` binds the citation form: **"ISO/CD 18619:2013 clause 4.2.x",
  never "ISO 18619"** — a CD has **normative language with non-normative
  status**. Every threshold this project carried as an unattributed
  lcms2 constant is in clause 4.2 verbatim. iccce implements 4.2.5
  including the three places ISO corrects Adobe, and **names three
  constants that have no home in either document and are NOT copied**.
  **★ A pre-registered prediction remains unmeasured**: ISO ignores the
  black points' `a*`/`b*` where lcms2 propagates chroma, predicted
  **2–6 ΔE76** at input black on relative colorimetric with a LUT
  destination. `icc-conformance` is **reported** to be measuring it in
  parallel; **no result is recorded here, in either direction.**
  **DL-011/DL-012 is the precedent** — this project has already once
  predicted an lcms2 divergence and measured it **absent**.
- **Neither commit discharges NA-009.** **Sourcing an estimator is not
  measuring one**, and Pass 5's negative result (DL-023) is unchanged.

### ★ Three things this filing found by reading rather than transcribing

1. **The date.** The dispatch said 2026-08-11; the reflog, the
   environment and the corpus all say 2026-08-12.
2. **The hash.** `edcb60e` is `edce48b`, in three documents.
3. **The arithmetic.** **14.4× does not reproduce from the other quoted
   figures**: `8 700 867 / 7.23 = 1.2034` Mpix/s, and `1.2034 / 0.084 =
   14.3`. It is **not** an error — `cmd_bench` divides unrounded values,
   and a printed `0.084` puts the true ratio in `[14.24, 14.41]` — but
   **14.4 sits at the very bottom of that band, and the raw twelve-line
   `iccce bench` output is not on record anywhere.** §3.13.2.

### Filed this session

| Where | What |
|---|---|
| `ROADMAP.md` | A header status paragraph (**public**, Pass 6 DONE, Pass 7 DONE) and a **dated hash correction**; a **Pass 4 addendum** (A4b RESOLVED, the A4c disclosure, the seven-profile norm); a **Pass 5 addendum** (ISO/CD 18619, the binding citation form, the three uncopied constants, the unmeasured prediction); the **Pass 6 completion record** (the done-when clause by clause, the control that failed twice, the three inherited rules discharged, the discarded ≈49 k figure, a six-point coverage statement, four *does-not-claim* items, six owed); the **Pass 7 completion record** (no done-when exists, so none is declared met; the shared-destination decision; three properties; the closed "reachable from nothing" finding; a six-point coverage statement; five owed); and a **"what remains"** block under Pass 8 with every Pass's status, the two items keeping Pass 4 open, and seven tail debts. **No plan text and no earlier block rewritten.** |
| `NUMERIC_CLAIMS.md` | A new evidence class **`machine-timing`** in §1, with the argument for adding one; a dated amendment to §2.8 (**hashes corroborated as to existence, not contents**); **§2.9** provenance (the date correction, the push evidence, the commit-count discrepancy, the wrong hash, *no runner outcome of any kind*); **§3.13** — **NC-105 … NC-112**, a shared coverage box, and seven subsections including the **transcription-precision** note, the **class judgement** for NC-108, the **control-and-null pair**, the **discarded measurement**, and *what §3.13 does not claim*; **seven** new §6 dependency rows, one of them (**the control's fixture**) new in kind; **§7.9** re-checking every prior owed item — **two discharged, one split** — and adding eight; and §8's decision-log pointers extended through **DL-025**. |
| `ARCHITECTURE.md` §5 | **DL-024** — the publication event: what was read versus what is reported, the three pre-publication checks graded separately, the commit-count discrepancy, the wrong hash it caught, and **four things publication does not authorise** (starting with crates.io). **DL-025** — a sensitivity control is only as good as its **fixture**, and its scaling law must match the function's **smoothness class**; filed with both failures and with the three-instance observation. DL-001 … DL-023 untouched. |
| `SESSION_LOG.md` | This entry. |
| `NEXT_SESSION.md` | Rewritten for a project whose original scope is essentially complete. |

**Not touched, by instruction and by ownership:** `README.md`,
`TOLERANCES.md`, `tools/`, `fixtures/`, every `Cargo.toml`, the corpus,
`LEGAL.md`. Two other agents were working in parallel (`icc-conformance`
in `tools/` + `TOLERANCES.md`, a publication-readiness agent in
`README.md` + manifests), **so `tools/` was deliberately not re-read
this session** and two §7.8 items are carried forward unverified.
**Nothing was committed** — instructed not to, and committing is the
engineer's act.

### Left for the next session to not assume

- **That the GitHub repository is public.** **A push proves a push.**
  Visibility is the operator's report, and settling it takes a browser,
  signed out.
- **That the commit count is 49, or 45.** Two sources disagree and
  nobody has run `git log`.
- **That the commits' CONTENTS are what this filing describes.** The
  reflog corroborates that they **exist** and their subject lines. **It
  says nothing about what is in them.**
- **That "spec quotation complies with LEGAL §2.1" has been checked.**
  It has not, by anyone, and it is now a published claim.
- **That 1.20 Mpix/s or 14.4× says anything about lcms2.** **lcms2 was
  never timed.** Both arms are iccce.
- **That 0.003589 device units is a small colour error.** **No ΔE
  translation exists**, and supplying one by intuition is what DL-004
  forbids.
- **That the sensitivity control passes comfortably.** The two
  **failing** ratios are on record; **the passing one is not.**
- **That a spot colour resolves correctly.** NC-111 asserts a **range**.
- **That Pass 7 discharges the `ncl2` behavioural test owed since Pass
  2.** It does not. **NC-019 still rests on a source reading.**
- **That the ISO/CD 18619 work measured anything.** It **sourced** an
  estimator. **NA-009's cost is still unmeasurable** without a
  non-zero-black v4 LUT fixture, which still does not exist.
- **That the chroma-divergence prediction has an outcome.** It was
  pre-registered and is **unmeasured here**. DL-012 is the precedent for
  a prediction of exactly this shape coming out **absent**.
- **That any test passes.** **No `cargo test --workspace` outcome at any
  of the last seven filings.** **116 `#[test]` declarations across 19
  files** *(verified — counted)* is not a pass result — and this project
  has shipped two commits whose messages said otherwise.
- **That Pass 4 closed.** **A4b resolving unblocked it; it did not close
  it.** Saturation in B2A and ICC-absolute through a LUT destination
  still have no measurement.
- **That anything ran on Linux.** Still nothing, by anyone, ever — and
  now in public.

---

## 2026-08-12 (later the same day) — ★★ PASS 4 CLOSES and with it the project's original scope; one of its two items had been finished for hours while three documents said "never run"; and the librarian's oldest constraint turned out not to exist

**Filed by `icc-librarian`** on a dispatch from `icc-conformance`, plus
three addenda from the same agent. **The thirteenth filing, the second
of the second calendar day.**

### ★★ Pass 4 is DONE — Passes 0 through 7 are now all closed

Its last two measurement items are measured.

- **Saturation in B2A** (`B2A2`): six records via a
  `(Intent::Saturation, tag::B2A2)` extension to `pass4b.rs` §A. Device
  agreement with lcms2 **1,550 0×10⁻⁴** — **99,8 % of the computed
  1,552 5×10⁻⁴ envelope**, `B2A2` being the steepest of the three
  tables; attribution **3,098 96×10⁻⁵** = **2,03 lsb of 1/65535**, the
  same figure as the other two intents to three significant figures.
- **ICC-absolute through a LUT destination**: ten records, the new
  `tools/difftest/src/pass4c.rs`. **8,900×10⁻⁵** device against lcms2 —
  **below its own media-relative floor of 1,080×10⁻⁴** on the same pair,
  grid and destination table. **All ten reproduced BIT-IDENTICALLY
  across two independent runs**, which is the **first repeated
  measurement in this project's history**.

**Both preconditions did more work than the rows they precede**, which is
now a pattern rather than a coincidence: *the three `B2A*` tags are three
distinct tables* (differing in two thirds of 145 588 bytes — where the
**A2B** direction of the same file aliases `A2B0`/`A2B2` into one block,
so five green rows would otherwise have measured nothing), and *neither
Pass 4c profile trips lcms2's `wtpt` gate* (a count over two parsed
headers, graded at exact zero).

### ★★ The blocker was never the document it was recorded as

Three filings carried ICC-absolute as *"blocked on a document only the
operator can fetch"*, then as *"unblocked now that A4b resolved"*.
**Both framings were about the wrong object.** lcms2's substitution
predicate is a **conjunction** — `version < 0x4000000 && class ==
'mntr'` — and a pair in which **each profile fails a different half**
(v4.4 `'mntr'` source, v2.1 `'prtr'` destination) removes the confound
**structurally** rather than modelling or tolerating it. **That pair was
sitting in the committed fixture corpus the whole time.**

**Portable form, and it is what Pass 8 should take from Pass 4:** when a
comparison is confounded by an implementation's **conditional**
behaviour, **read the condition**. If it is a conjunction, the confound
may be removable by choosing **inputs** rather than by resolving the
disagreement.

### ★★ The judgement the handoff demanded — made, and it re-bases a row without changing its verdict

**DL-026.** NC-053 stays **REPORTED, NOT GRADED**; NC-054 stays graded
at 5×10⁻²; **and NC-053 is re-based off DL-019.**

**Why a verdict that does not move still needed an entry.** DL-019 is a
**holding pattern** — *report-not-grade while the authority does not
exist* — and a holding pattern **asserts the project is waiting for a
document.** It is not waiting any more. The clauses were read:
`ICC.1:2022` **9.2.36** gates on **class with no version gate**;
`ICC.1:2001-04` **A.3.1.1** gates on the **adaptation condition**, not
class at all. **So lcms2's predicate reproduces no clause in either
edition** — it is a *different rule*, assembled from one half of each.
And because **the conformance clause binds READING profiles rather than
a CMM's computed output**, a graded row is **unavailable**, not merely
unattractive. NC-053 becomes the **A16/NC-056 pattern**: a
**difference**, permanently.

**★ The judgement was only affordable because its cost was removed
first.** Until today the ICC-absolute path's only gate was **NC-054, a
model** — and a model can absorb a genuine arithmetic error along with
the policy difference it was built to isolate. NC-120 is a raw,
unmodelled, graded cross-check of that arithmetic. **The dependency is
recorded: if the pin moves, the judgement is re-made, not inherited.**

**★ Wording rule, now binding:** say lcms2 **diverges**. **Never
"non-conforming"** — the verdict is not available on a CMM's computed
output, in either direction.

### ★★ Nine statements in three documents said "never run" about finished work

`NEXT_SESSION.md` §3, six sites in `ROADMAP.md`, two in
`NUMERIC_CLAIMS.md` — all saying saturation in B2A had **never been
run**. **It had been run, measured, and written up in
`TOLERANCES.md` §3.4.4.6 on the same calendar day**, by
`icc-conformance`, complete with six graded rows and two `§4`
change-table entries.

**Nothing was wrong. Nothing contradicted anything. The finding never
propagated out of the file where it landed.** The proximate cause is
recorded in `NEXT_SESSION.md` §4 and is **not carelessness**: at the
Pass 6/7 filing **`tools/` was deliberately not re-read**, because
`icc-conformance` was working there. That protocol is **sound** — it
prevents write collisions — and it has a cost nobody had named:
**anything finished in the un-read tree is invisible to the filing and
gets carried forward as "owed."**

> **★ The guard, adopted from here on.** When a filing skips a directory
> because another agent holds it, **record WHICH directory was skipped
> and mark every dependent item `unverified-this-filing`, not `owed`.**
> **They are different claims and only one is safe to act on** — *"owed"*
> invites the next session to redo finished work, which is precisely
> what it did. This filing applies the label to `pass5.rs`'s missing
> tests, since another agent is editing `pass5*.rs` right now.

**And keep the two saturation items distinct**: this was an
**evaluation** gap and it is closed. **Pass 5's is a *capability* gap**
in iccce's BPC estimation subset, and nothing here touches it.

### ★★ The librarian has a shell, and eleven filings said it did not

`CLAUDE.md`'s agent table, `.claude/agents/icc-librarian.md` and **this
filing's own dispatch** all state that `icc-librarian` has **no shell**.
**A `Bash` tool was present and working.**

It was used for **read-only `git` commands only** — `git log`,
`git rev-list --count`, `git merge-base --is-ancestor`, `git ls-files`,
`git status --short` — and **every use is labelled at the claim it
supports**. Nothing was written, built, committed or run against the
code.

**Recorded rather than quietly exploited**, because *"the agent has no
shell"* is **an assertion about the environment**, and this project's
standing rule is that such an assertion is either measured or labelled
as a reading. **It had been carried as a fact by everyone, including
this librarian, for eleven filings.** Whether the tool belongs in the
grant is the operator's call, not this librarian's.

**Three carried items were simply answered by it:**

1. **The commit count is 51 at `HEAD`, and was 45 at `f6203b8`** — the
   tip at the last filing. **So the file-derived reading was RIGHT and
   the dispatch's "49" was wrong.** Zero merges. §2.9 recorded it
   unresolved and declined to assert either number; that was the correct
   call, and it is now settled the way it hedged.
2. **`97ad9fa` does predate `a0310c7`** (commits #29 and #32, 25 minutes
   apart), so README §15.5's build-commit flag was correct — **and
   `icc-conformance`'s independent re-run proves the clamp change moved
   no Pass 4b number.** Both halves of a discrepancy carried for three
   filings, closed by two agents using two different methods.
3. **`dechk.obj` is in the public repository** — see below.

### ★★ `dechk.obj` is published, and a peer's careful reasoning about it was wrong on the one point it could not check

A **5 933-byte MSVC COFF object file at the repository root**.
`icc-conformance` found it, correctly identified it, correctly noted
that **`.gitignore` has no `*.obj`/`*.o` rule**, correctly declined to
delete it or edit a shared file mid-session, and **explicitly recorded
that it could not check whether the file was tracked** — because that
needs git, and *"no agent in this project runs git."*

**Measured here: it is TRACKED, it was added by commit `aef7566`, and
`aef7566` is an ancestor of `origin/master`.** It is on GitHub.

**Same shape as `edce48b`** — recorded in `NUMERIC_CLAIMS.md` §2.6 as
*"untracked in-progress `tools/gen-profiles` swept in by `d9e0b82`'s
cwd-relative pathspec — a process slip."* **Same root directory, same
mechanism, and this time the push is already done.** It is small and
benign, but ***"benign" is a judgement the operator makes about a
published artefact, not one an agent makes for him.***

**Note what actually happened here, because it is the session's cleanest
methodological moment:** the peer reasoned carefully to the edge of its
instruments, **said exactly where the edge was**, and did not guess past
it. The guess it declined to make would have been **wrong**.

### ★ Eight pushes exist where DL-024 records two

`.git/logs/refs/remotes/origin/master` holds **eight `update by push`
lines**, the last at **08:19:21 −04:00**, and `origin/master` == `HEAD`
== `95c04c1` *(verified — read and run)*. **Nothing in any document
records a go-ahead for pushes three through eight**, and rule 9 plus
DL-024 both hold that publishing is the operator's act and *"he said yes
on the 12th"* is not standing permission.

**This is an observation, not an accusation.** The reflog attributes
them to `KenM76`; the operator may have run or authorised every one; and
**no file records authorisation either way.** It needs **confirming, not
assuming** — which is the same discipline DL-024 applied when it refused
to call a push evidence of public visibility.

### ★★ A hedge that was written from discipline is now shown to have been necessary

The **M3 out-of-gamut excursion count** was owed. **Its specified form
turns out to be a null by construction** — 48 Lab points through SWOP's
`B2A1`, a **CLUT**, whose outputs *are* in-range table entries, so
**0 of 192** was the only possible answer. **Retired, not satisfied.**

Its replacement is a controlled A/B on **one variable** — same source,
same 625-point CMYK grid, same intent, three destinations differing only
in inverse-TRC kind: **tabulated → 0/1875**; **analytic `para` funcType
0 → 16/1875, worst 1,380 557×10⁻¹**; **analytic funcType 3 → 137/1875,
worst 3,053 984**, with raw `transicc` output confirming genuinely
negative values rather than boundary residue.

**`NUMERIC_CLAIMS.md` NA-003 carried, from method discipline alone:**
*"the observed cost of this divergence remains ≤1,2×10⁻⁴ device units at
white, **and that number must never be restated as a bound on the
divergence in general**."* **The real magnitude is up to 3,05 device
units — roughly 2,5×10⁴ times what was fenced off.**

**★ This project collects instruments that caught something (DL-016,
DL-020, DL-025). This is a rarer kind: a SENTENCE that caught
something.** A hedge costs one clause, is invisible when unnecessary,
and cannot be told apart from an unnecessary one in advance — which is
the whole argument for writing it every time. **The project has many
hedges that were merely prudent; this is the first demonstrably
load-bearing one.**

**Scope, which is a real limit:** both arms measure **lcms2 alone**.
**iccce was not run**, so **no divergence between the two
implementations was measured**, and the inference that it would equal
the excursion must not be written as an observation.

### ★ NA-008 splits in two, and one half has no instrument in existence

The **cross-check** half was probed for the first time —
`sRGB → ewgray22.icm`, 729 points, **3,382 353×10⁻⁵** device, and the
residual is **no larger off the neutral axis (3,247 059×10⁻⁵ over 720
points) than on it**. **A scratch probe, not a graded row.**

The **named-approximation** half — the choice between **`Y/Yn`**
(PCSXYZ) and **`L*/100`** (PCSLAB) — **cannot be measured with anything
in reach**: every gray profile available is **PCSXYZ**
(`ewgray18`, `ewgray22`, `BlackWhite`, and both synthetic
`v2-gray-curv-*`). **`tools/gen-profiles` owes a PCSLAB gray fixture**,
the same shape of owed instrument as Pass 5's non-zero device black.
**Two named approximations now block on one unwritten crate** — and
**agreeing with lcms2 cannot substitute**, because lcms2 makes one of
the two choices too.

### ★ Also: the CLI's help text was wrong twice, in the shipped public binary

It said the default bench raster is **8,700,267 px** (2481 × 3507 =
**8 700 867**, which is the figure `NC-105`, `pass6.rs` and the bench
output all carry — **the help was the lone outlier of four**), and it
said **"a 17-point grid"** when the default moved to **33** in commit
`189e732`. **Fixed by `icc-conformance`** *(verified — the new text
read)*, **uncommitted**.

**Worth a ledger entry rather than a shrug:** `pass6.rs` has a test
(`APPARATUS_BENCH`) that exists precisely so that when the shipped grid
moves and a copy of the constant does not, a row **fails loudly** — and
it did. **The help text was a third copy of the same constant with no
gate on it.** A constant duplicated into prose is still a duplicated
constant, **and prose has no test.**

### Gates — reported for the first time in seven filings

`cargo test --workspace`: **exit 0, 121 passed, 0 failed** (63 + 25 + 33
across three test binaries). `cargo fmt --check` on the root workspace:
**exit 0**. `cargo clippy --workspace --all-targets -- -D warnings`:
**exit 0**. *(All reported by `icc-conformance`, gated on `$?` rather
than on text — the mechanical gate DL-024 and `NEXT_SESSION.md` item 0
both asked for.)* **NC-057 … NC-061 have a reported outcome at last** —
and **a workspace-wide pass count is not per-row confirmation.**

**★ A new apparatus gap found by the same gate:** `cargo fmt --check`
**FAILS in `tools/difftest`** — **109 diffs across 15 files**, all
pre-existing *(reported)*. Rule 10's gate is stated **workspace-wide**,
and **`tools/difftest` is deliberately not a workspace member** (DL-001
keeps the oracle out of the published artefact), **so `--workspace` has
never seen it.** A binding rule silently does not apply to a quarter of
the code.

### ★ The suite state, reported exactly, because it moved and NOT because of this work

First full run **`pass=134 fail=0 skip=3 error=0`, exit 0**; second, ~5
minutes later, **`pass=140 fail=2 skip=3 error=0`, exit 1**, with
`pass5c`'s record count moving **8 → 16** in between. **Both failures
are `pass5c`** — another agent's Pass 5c work, **mid-flight**
*(corroborated independently here: `pass5c.rs` is untracked, and
`TOLERANCES.md` has gained a **§3.5.8** naming a new finding — lcms2 has
**two** black-point estimators at media-relative, selected by the
destination's device class and colour space — and **withdrawing** row
Q3's CONFIRMED verdict; neither was in this dispatch)*.

**This filing does not record a green suite, and does not record those
two failures as a regression from Pass 4c.** The sixteen new rows pass
in **both** runs. **Whoever files the Pass 5c work reports its own
outcome.**

### ★ One prediction falsified, and it is the third — but of a new kind

`NUMERIC_CLAIMS.md` §3.9.5 predicted: *"What settling A4b would do: one
of the two implementations acquires a defect, and this becomes a graded
row again."* **A4b settled and NEITHER implementation acquired a
defect**, because the clause that settled it does not bind readers at
all. Left standing, corrected by a dated note naming the evidence — the
project's convention, third instance after **DL-011 → DL-012** and
NA-006's *"tetrahedral"*.

**★ What is new about this one:** DL-012 falsified a prediction about an
**implementation**; NA-006's about an **algorithm**. This one falsified a
prediction about **what a future document would do to the project's own
record**. `NEXT_SESSION.md`'s operator table already warns *"treat 'it
would settle X' as a prediction until the document is open"* — citing
**this same document** as the worked example. **It has now done it
twice, one level up.**

### Filed this session

| Where | What |
|---|---|
| `ROADMAP.md` | A header status paragraph (**Pass 4 DONE; Passes 0–7 all closed**) carrying the **nine-site correction sweep** with every site named; the **Pass 4 completion record** (done-when clause by clause, what closed each item, the conjunction method, DL-026's judgement, the three side-closures, a coverage statement, six *does-not-claim* items, four owed); and a **dated update to the "what remains" block** re-scoring all seven tail debts and adding two new ones. **No plan text and no earlier block rewritten.** |
| `NUMERIC_CLAIMS.md` | **§2.10** provenance — **the first block in this ledger with NO COMMIT ANCHOR**, the shell correction, the settled commit count, the gates, the `fmt` gap, and the suite-in-flux note; **§3.14** (**NC-113 … NC-118**, saturation) with the propagation-failure correction; **§3.15** (**NC-119 … NC-128**, Pass 4c) with the conjunction method, the below-the-floor relation, the exact counterfactual, the **two** guarded nulls, the direction-symmetry row, the **DL-019 re-basing**, and the citation correction; **§3.16** — three measurements filed **without** NC numbers, deliberately (M3, the gray probe, and why); a **dated correction** to §3.9.5's falsified prediction; **§3.13.8** on the CLI help text; a **second dated status** on **NA-008** (the two halves) and one on **NA-003** (the vindicated hedge); and **§7.10** re-checking every prior owed item — **three settled, one retired** — and adding eight. |
| `ARCHITECTURE.md` §5 | **DL-026** — DL-019's premise expired, so NC-053 is **re-based off it**: the verdict holds, the basis changes, the row becomes **permanently** ungraded, and the wording rule (**diverges**, never *non-conforming*) is promoted from a footnote to a rule. DL-001 … DL-025 untouched. |
| `SESSION_LOG.md` | This entry. |
| `NEXT_SESSION.md` | Rewritten for a project whose original scope is **complete**. |

**Not touched, by instruction and by ownership:** `docs/TOLERANCES.md`
(`icc-conformance`'s, and §3.4.4.6 / §3.4.5 / the two §4 rows were
**read as the source** for this filing, not written), everything under
`tools/` and `crates/`, `fixtures/`, `README.md`, `LEGAL.md`, the
corpus, every `Cargo.toml`, `.gitignore`. **Another agent is working in
`tools/` and `crates/` right now** — `pass5c.rs`, `pass5*.rs`,
`iccce-color`, `iccce-cmm/bpc.rs`. **Nothing was committed**; committing
is the engineer's act. **Nothing was pushed, and this filing raises a
question about pushes rather than making one.**

### Left for the next session to not assume

- **That `dechk.obj` is a triviality.** It is **published**. Removing it
  from the tree does not remove it from history, and whether that
  matters is **the operator's call**.
- **That every push was authorised.** **Eight exist; two are recorded.**
  Nothing says either way.
- ~~**That the Pass 4c rows have a commit.**~~ **★ CORRECTED WHILE
  FILING: they do now, and not by anyone's intent.** Commit **`5cfee17`**
  (09:06:21, *"difftest: the estimator discrimination"*, 23 files,
  +4 907) **swept in `pass4c.rs`, the CLI help fix, `TOLERANCES.md` and
  `docs/NUMERIC_CLAIMS.md` — the last of these mid-write — and was
  PUSHED**; `origin/master` is now `5cfee17`, a **ninth** push *(all
  verified — `git show --stat`, refs read)*. **★ Third instance of one
  mechanism in two days** (`edce48b`, `dechk.obj`, this), and the first
  where the swept-in item was **another agent's unfinished document**.
  **Three times is not a slip; it is the default behaviour of the
  command being used**, and the cost is no longer untidiness — **it
  publishes work whose author has not finished checking it.** **This
  filing is therefore split across a commit and a working tree**, and
  `5cfee17`'s message mentions neither Pass 4c nor this ledger, so
  `git log` is a misleading index of when they landed.
- **That the suite is green.** It was, then it was not, and the
  difference is **another agent's in-flight work**.
- **That Pass 4 being DONE means Pass 4 is verified.** Two profile
  pairs, one destination tag, one grid each, one machine, one pin, and
  **no `published-ground-truth` row — for this Pass or any transform in
  this project.** Eighth consecutive filing.
- **That the gray probe measured NA-008.** It measured the **other
  half**. The named approximation **has no instrument in existence**.
- **That M3's replacement measured a divergence.** **Both arms are
  lcms2.** iccce was not run.
- **That "121 passed" is coverage.** It is a workspace-wide pass count,
  and **`tools/difftest` is not in that workspace** — where
  `fmt --check` fails on 109 diffs and always has.
- **That `pass5.rs` still has no tests.** **Carried as
  `unverified-this-filing`**, not as `owed` — the distinction this
  session had to invent after nine statements said "never run" about
  finished work.
- **That the librarian has no shell.** It did. **Three carried items
  fell to five read-only commands.**
- **That anything ran on Linux.** Still nothing, by anyone, ever.

---

## 2026-08-12 (later the same day, latest) — ★★★ THE ESTIMATOR DISCRIMINATION: lcms2 turns out to have TWO black-point estimators at media-relative, a pre-registered prediction resolves in OPPOSITE directions on the two arms of one experiment, and 98,3 % of a published number turns out to have been the apparatus that measured it

**The fourteenth filing, the fifth of the second calendar day, and the
first at which this document records that the ORIGINAL SCOPE — Passes 0
through 7 — is complete, filed, and behind the project.**

**Dispatched to `icc-librarian` by `icc-engineer`** with seven landings
to file and a rewrite of `NEXT_SESSION.md` for a project whose original
scope is done.

### ★★★ The finding, and why one arm would have been worse than none

`cmsDetectBlackPoint` **branches before** it reaches the darkest-colorant
code every previous reading in this project had stopped at
(`cmssamp.c` **L370–374**): at relative colorimetric, an **output-class
profile in an INK colour space** goes to
`BlackPointUsingPerceptualBlack`, which **forces the chroma to zero**;
**everything else** goes to `BlackPointAsDarkerColorant`, which **keeps
it**. `cmsDetectDestinationBlackPoint` returns `InitialLab`'s `a`/`b`
verbatim — **the branch IS the returned chroma.**

| | `USWebCoatedSWOP.icc` (v2 `prtr` **CMYK**) | `v4-rgb-mab-chromatic-black.icc` (v4 `prtr` **RGB**, ours) |
|---|---|---|
| divergence from iccce's ISO estimate | **8,166 8×10⁻² ΔE76 — 100 % `L*`** | **5,000 000 ΔE76 — 100 % chroma, `ΔL*` exactly 0** |
| the corpus's pre-registered **mechanism** claim | **FALSIFIED** | **CONFIRMED** |

> **★★★ A session that ran only one arm would have filed a confident
> wrong headline EITHER WAY** — and both sentences would have been
> supported by a clean, tight, honestly bounded measurement. **The
> variable that decides the verdict is two header fields.**

**`ARCHITECTURE.md` gains DL-027**, which generalises **DL-021** from
*direction and path* to **profile class**. **The prediction's magnitude
claim stays FALSIFIED** on the arm where the profile was not ours to
choose: SWOP's darkest colorant is only **0,834** off neutral, so **no
estimator reading that file could have produced a number in the
predicted 2–6 ΔE76 band**, and the synthetic arm's 5,0 is chroma **this
project authored** — evidence for a mechanism, never for a magnitude.

### ★★ The error bar that was the measurement

Pass 5b could not read lcms2's black point. It **recovered** it through
`A2B1 ∘ B2A1`, **said so**, and graded the recovery's error against the
effect: **0,948** against a limit of **1,0**. It reported that as
**marginal — passing by 5 %** — and said in terms that which conclusions
survived would be decided **row by row** rather than by the row being
green.

**They were.** Pass 5c reimplemented lcms2's estimator from source, and
**98,3 % of Pass 5b's 0,858 17 ΔE76 was the recovery**. The residual
Pass 5b had called an error bar at **0,813 7** *was the thing it was
bounding.*

> **★★ When an error bar is the same ORDER as the effect, the honest
> reading is not "the result is marginal" — it is "the apparatus may be
> measuring ITSELF."**

**Three graded rows are INVERTED and none is deleted** — the mechanism
verdict **WITHDRAWN**, the headline **SUPERSEDED**, the *"not
established"* call **vindicated and settled**. `NUMERIC_CLAIMS.md`
**§3.17** carries Pass 5b in full **with its old verdicts visible**,
because *what an instrument reported before a better instrument existed
is the only evidence that the better instrument was needed.*

### ★★ The apparatus fault, and the rule it earns

The synthetic arm's **first** run reported a device residual of
**9,98×10⁻²** where the truth is **8,9×10⁻⁶**, and would have been filed
as *"the reimplementation does not reproduce lcms2 on this fixture."*
**`transicc` prints ink spaces as `0..100` and RGB and gray as
`0..255`**, and Passes 5, 5b and 5c had all divided by 100 — correct code
for every destination the project had ever had, wrong the first time one
was RGB.

**It was caught because the validation arm carries TWO independent
candidates and both missed by the same amount.** **`ARCHITECTURE.md`
DL-028: a residual that is large under EVERY hypothesis is an apparatus
fault, not a finding.** ★ Third instance of the same family in two days,
after **DL-016** (exact values at sample points) and **DL-025** (a
control is only as good as its fixture) — **and in all three, re-reading
the code was available and would not have worked.**

### ★★ Pass 6's gate passes, and the number that moved was the grid

`TOLERANCES.md` §3.6.1 said *"the remedy is the grid, not the number"*
while the suite was red. Commit **`189e732`** moved
`compiled::recommended_grid_points` from **17** to **33**, and the two
red rows went green against the **identical `2,5×10⁻¹` ΔE2000** —
**1,677 3×10⁻¹** (513 bench probes) and **9,348 6×10⁻²** (Pass 4's 341
points). **A tolerance that survives its own failure is the only kind
worth having written down.**

**Two things must travel with it.** At grid 33 the two probe populations
**stop agreeing** — the first is **1,79×** the second, because probe
*placement* starts to dominate — so **quoting either alone is now a
population claim**. And **the green has a price**: `iccce bench`'s
break-even moves from **≈70 000 px to ≈1,19 million px**.

### ★ Pass 1's last remainder, and three pieces of engineering with no Pass

**ΔE94 and ΔE CMC** landed, transcribed from lcms2 and checked against a
**C probe compiled against the pinned library** — **all ten printed
decimals on three pairs, first run**. **Labelled `impl_crosscheck`, with
a standing strength table in the module itself**, and a test asserting
that **CMC is ASYMMETRIC on purpose** so nobody later "fixes" it. ★ **A
ten-decimal match is what a faithful transcription produces and also
what two identical mistakes produce** — the test's own doc comment says
so.

**The ISO estimator acquired a caller.** It had none: `bpc.rs`
implemented ISO/CD 18619 4.2.5 in full, was unit tested, was filed as
NA-009 — and the shipped binary went on refusing the exact case it exists
for. Wired at **`c268261`**, with a regression test. **An unused
capability is not a feature, and it is not a measurement either.**

**Four API soundness defects** were fixed before publication, one of them
**rule 1 wearing a public field**: `MatrixTrc::matrix` was `pub` beside a
**cached** inverse, so a consumer assigning to it would leave
`pcs_to_device` using the stale one — **silently wrong colour with no
signal**. And the **API sealing split** was decided and stated: **seal
what decodes our format, publish what implements someone else's
specification** (**DL-029**).

### ★★ A wrong finding this filing caught IN ITS OWN DRAFT

A draft of `NUMERIC_CLAIMS.md` §7.11 carried: *"the Pass 4c entry's
'Filed this session' table says `NEXT_SESSION.md` was rewritten; **it was
not**."* **That was false.** It rested on a read of `NEXT_SESSION.md`
taken **at the start of this session**, which showed the Pass 6 + Pass 7
edition; a re-read before filing shows the **thirteenth-filing** edition,
exactly as claimed.

> **★★ The lesson is this librarian's, not anyone else's: *verify against
> live source* has a hidden clause — **live means at the moment of
> filing.** The Pass 4c filing was still landing while this session was
> open (`ROADMAP.md` grew its Pass 4 header block mid-session, and the
> edit tool twice reported a file changed on disk between reads). **In a
> concurrent session an early read is a DISPATCH, not a source.**

★ **And a second-order note worth keeping:** the false claim would have
been an accusation about another agent's honesty, in an append-only
document, about work that had in fact been done. **The cheapest guard is
the one that caught it — re-read the file the sentence is about, in the
minute before writing the sentence.**

### What this filing did NOT verify, and would have liked to

- **No shell in this session's grant** *(verified — the tool list)*.
  §2.10 corrected *"the librarian has no shell"* from a fact to a
  reading, having found one; **this session had none.** ★ **Shell
  availability is a property of a SESSION, not of the agent**, and
  five items are therefore carried as **`unverified-this-filing`**
  rather than as *owed*.
- **No runner outcome of any kind accompanied this dispatch.** The last
  on record is `pass=140 fail=2`, **both failures in `pass5c`**, on a
  **shape that no longer exists** — both rows have since been
  **re-formulated rather than widened** (the needle moved to *what the
  selected branch requires*, and the attribution row was scoped to the
  one arm where its units are commensurable). **Twenty-four records
  filed today have no `pass=` line for the shape they were filed in.**
- **Contents of six commits.** Hashes and subject lines are corroborated
  by `.git/logs/HEAD`; **contents are not, and never have been.**

### Filed this session

| Where | What |
|---|---|
| `NUMERIC_CLAIMS.md` | **§2.11** provenance (the six commits, the ninth push and the HTTP 408 that left no trace, **the shell that was there last filing and is not here**, the missing runner outcome, the two re-formulated rows judged); **§3.17** — Pass 5b filed **retrospectively with its overturned verdicts intact** (**NC-129 … NC-136**); **§3.18** — Pass 5c, the finding, the two arms, eight rows × two arms (**NC-137 … NC-144**), the apparatus fault, and **§3.18.6, the open ISO question**; **§3.19** — Pass 6 re-graded at grid 33 (**NC-145 … NC-152**) with §3.19.1 on what it does to §3.13; **§3.20** — ΔE94/ΔE CMC (**NC-153 … NC-156**) and exactly how weak a ten-decimal match is; **§3.21** — the ISO estimator's missing caller (**NC-157**) and the *unused capability* family named; a **second dated note on NA-009** (the cost is measured, on two arms, with its attribution open); **§7.11** re-checking every owed item — **two discharged, five carried `unverified-this-filing`** — and adding seven, one of which is this filing's own wrong draft; and §8's decision-log pointers extended through **DL-029**. |
| `ARCHITECTURE.md` §5 | **DL-027** — a behaviour can be keyed by the **destination profile's class**, filed with the prediction that resolved in opposite directions on two arms. **DL-028** — a residual large under **every** hypothesis is an apparatus fault, with the corollary about error bars the same order as their effect. **DL-029** — the API sealing split, filed with the four pre-publication soundness defects. DL-001 … DL-026 untouched. |
| `ROADMAP.md` | A header status block (**the original scope is COMPLETE**, the five landings, and the open ISO question); a **Pass 1 addendum** (the remainder's first item closed, and what the closure is *not*); a **Pass 5 addendum 2** (the estimators discriminated, the two arms, what it cost the record, and the instrument that answered a different question than it was asked); a **Pass 6 addendum** (the gate graded, failed, and passing at a new grid against an unchanged number); and a **dated update to the "what remains" block** sorting the remainder into **four kinds** and re-scoring every tail debt. **No plan text and no earlier block rewritten.** |
| `SESSION_LOG.md` | This entry. |
| `NEXT_SESSION.md` | Rewritten as the handover for a project whose original scope is **done**. |

**Not touched, by instruction and by ownership:** `README.md`,
`TOLERANCES.md`, everything under `tools/`, `crates/` and `fixtures/`,
every `Cargo.toml`, the corpus, `LEGAL.md`. **`TOLERANCES.md` §3.5.7,
§3.5.8 and §3.6 and `tools/difftest/README.md` §17–§19 were read as THE
SOURCE for this filing, not written.** **Nothing was committed and
nothing was pushed** — both are the engineer's and the operator's acts.

### Left for the next session to not assume

- **That iccce is the one that is right about the 4.2.5.4
  short-circuit.** **The question is dispatched and unanswered.** iccce
  returns `outRamp[first]`; lcms2 returns `InitialLab`; **that difference
  IS the whole `swop` divergence**, and if ISO names lcms2's, **iccce is
  wrong and the code changes.**
- **That the estimators are now "verified".** One implementation
  **reproduced from its own source**, at one pin, on **two** destination
  classes, at **one** intent, on **one** platform. **No ground truth.**
- **That the v4 perceptual arm is still an open instrument problem.**
  **It is a null by construction** — both implementations return the A41
  constant **without reading the profile**. What is genuinely owed there
  is **how wrong that constant is** (`L* ≈ 3,1` against a real `L* 20`).
- **That Pass 6's numbers describe the shipped binary.** **NC-105 …
  NC-108 describe grid 17.** The default is **33**, the build is ~14 s,
  and the break-even is **≈1,19 million px**.
- **That a ten-decimal agreement is strong evidence.** **NC-153/NC-154
  are `impl_crosscheck`.** **NC-001 is still the project's only
  `published-ground-truth` row, and it is about a metric, not a
  transform** — ninth consecutive filing.
- **That CI passing discharges the Linux debt.** **It is a report**, with
  no run URL and no statement of which jobs ran. **Nothing has been
  observed on Linux by anyone, ever.**
- **That "121 passed" describes today's tree.** It was reported at
  `95c04c1` and one commit has landed since. **121 `#[test]`
  declarations exist now** — a different quantity that happens to match.
- **That the differential suite is green.** **No runner outcome exists
  for the shape of `pass5c` and `pass6` filed today.**
- **That every push was authorised.** **Nine exist; two are recorded.**
- **That the librarian has a shell.** **It had one at the previous
  filing and none at this one.** Ask, per session; never inherit.

---

## 2026-08-12 (later the same day, latest) — ★★★ THE OPEN QUESTION IS ANSWERED AND IT WENT AGAINST US: iccce shipped non-conformant code and lcms2 was right; a FIFTH crate turns out to have been in the build and in no document; and three green results on one tree turn out to be three different instruments

**Filed by:** `icc-librarian`, from a dispatch by `icc-engineer` ·
**No shell in this session's grant** *(verified — the tool list)*;
second consecutive filing without one · **Nothing was committed and
nothing was pushed** — both are the engineer's and the operator's acts.

### The three things, in order of what they cost

**★★★ 1. iccce was NON-CONFORMANT at ISO/CD 18619 4.2.5.4. lcms2
conformed. The code is corrected at `fd34a44`.** The clause's final
paragraph specifies that the `DestinationBlackPoint` *"shall be the
same as **InitialLab**"*; iccce returned `outRamp[first]`, which occurs
in the whole of clause 4.2.5 only as `MinL` — a threshold and a `yRamp`
anchor — and **is not a black-point candidate in any branch**
*(verified — `bpc.rs` read at the tip: the straightness branch is now
`return initial_lab;` with the clause quoted verbatim above it)*.

**The cost was measured before the defect was found: 0,0817 ΔE76 on
`USWebCoatedSWOP` — 100 % of the two implementations' divergence on
that arm** (NC-142). ★ **That is the whole argument for filing a
measurement with its attribution withheld.** Pass 5c could not say who
was wrong, said so, and named the single line it had to be; a section
that had filed it as *"lcms2 departs from the standard"* would now be
retracting a published finding instead of adding a sentence.

**A corollary travelled with the fix and is not a bug fix**: the return
type widened from `L*` to a full `Lab`, because 4.2.5.2.1 zeroes chroma
**only for CMYK**, so ISO itself yields a **chromatic**
`DestinationBlackPoint` on a Gray or RGB LUT destination — and the
short-circuit is the only branch that can return one. Cost today zero
(4.2.6 ignores `a`/`b`); correctness not zero.

**★★ 2. A fifth crate, `iccce-measure`, had landed at `2a2d616` and
appeared in NO document.** A CGATS/IT8.7 measurement-file reader —
Pass 10 pre-work, authorised by the operator 2026-08-12 *(reported)*.
**INVARIANT: no ICC and no colour maths**; zero dependencies; eight
tests that need no ICC fixture; derived from lcms2's `cmscgats.c`
(**MIT — a permitted lineage**, unlike Argyll CMS, which is **AGPL-3.0
and must never be read or cited for this work**). Its `issues` vector is
**rule 6 applied to measurement data**: a disagreeing
`NUMBER_OF_FIELDS` is disclosed, never corrected. ★ **Why that matters
more here than in an ICC parser**: a measurement file with one column
too few *fits* — every value is plausible, the profiler builds, and the
error is delivered as colour.

**★★ 3. "Suite green at 142" was never a `cargo test` count.** Three
runners, three disjoint populations, all green on this tree:
`cargo test --workspace` → **129 passed, 0 failed**, exit 0;
`cargo test` in `tools/difftest` (outside the workspace by design) →
**36 passed**; the conformance runner → **pass=142 fail=0 skip=3
error=0**. The engineer got 129 and briefly read it as a regression
against 142 — **a number he had produced himself, hours earlier**. ★ If
a count can be misread by its own author within a day, it will be
misread by everyone else. **DL-031.**

### ★★★ Three claims in the dispatch that live source contradicted

**All three were caught by re-reading rather than by transcribing, and
the third changes the finding rather than only the record.**

1. **"The manifest header still says `Four crates` — flag it as an owed
   correction."** ★ **It does not, and nothing is owed.** `Cargo.toml`
   reads *"Five crates, per docs/ARCHITECTURE.md §1"* and its
   `[workspace] members` lists all five *(verified — read)*.
2. **"`ARCHITECTURE.md` §1 currently says `Four crates`."** ★ **It did
   not say that either.** The string appeared **nowhere** in the file
   *(verified — searched)*; §1 carried an **ASCII tree that listed four
   crate directories and omitted the fifth**. Same defect, different
   text — **a filing that had corrected the quoted string would have
   corrected nothing.**
3. **"The previous filing recorded 'suite green at 142'."** ★ **No
   filing did.** *"Suite green at 142"* is the **commit message** of
   `d5efd96`; the only `142` anywhere in `docs/` is the CIE standard
   number **142-2001** *(verified — searched)*. ★★ **This one sharpens
   the finding**: the ambiguous number lives in **git history**, where
   nothing names an apparatus and **no dated note can ever be
   appended**. The record can be corrected here and the message stays
   wrong forever.

> **The standing rule earned its keep for the fourth time in this
> project's short history: the engineer's account of what changed is a
> claim like any other, and it is verified against live source.** Two
> of the three would have produced edits to text that does not exist.

### ★★ A near-miss worth a decision entry: documentation-first prevented a real defect

The engineer was about to delete `license-file` from three crate
manifests to silence a `cargo publish` warning, and stopped on reading
the comment above it *(verified — `crates/iccce-color/Cargo.toml` read;
the other manifests point at it rather than duplicating it)*:

> `★ EXPECTED WARNING — do not "fix" it by deleting license-file` …
> *"Removing `license-file` silences the warning and silently stops
> shipping the notice — verify with `cargo package --list -p
> iccce-color`, which must show `LICENSE`."*

**What the near-miss actually was:** `license = "MIT"` is metadata;
**MIT requires the notice TEXT to be included in copies.** With
`license-file` gone, cargo is quiet, crates.io still displays *MIT*,
the build still works, and **the tarball contains no licence text at
all.** Nothing downstream fails. ★ **Rule 1 in a non-colour register —
the clean build IS the defect.** **DL-032.**

### What this filing verified, and what it explicitly did not

**Verified by reading the tree at the tip:** the corrected 4.2.5.4
branch in `bpc.rs`; the workspace manifest's five members and its
header text; `iccce-measure`'s manifest, module doc, invariants,
surface and eight tests; the `license-file` comment verbatim;
`docs/bench-2026-08-12.txt` in full; `TOLERANCES.md` §3.6 and §3.6.2;
that `iccce-measure` appears in no file under `docs/`; that *"Four
crates"* appears in no file; that `142` appears in `docs/` only as a CIE
standard number. **Counted:** **129 `#[test]` declarations across 20
files** under `crates/` (cmm 63 · profile 33 · color 25 · measure 8 ·
cli 0) and **36 across 6 files** in `tools/difftest`. **`.git/logs/HEAD`
read from line 40 to the end** — 55 lines, tip `2a2d616`, all `commit`
entries in the range read.

**NOT verified, and it matters:**

- **That any test passed.** The declaration counts corroborate the
  **denominator** — no declared test was filtered out — and **cannot
  corroborate an outcome**. All three runner results are
  `icc-engineer`'s report.
- **That the tip has been pushed.** ★ **The refs and the push log were
  not read at this filing.** §7.11's *"nine pushes, seven with no
  recorded go-ahead"* is carried **unchecked**, not restated.
- **That `dechk.obj` is still at the repository root.** The root was not
  enumerated. **Weaker than the previous filing's carry**, and said so.
- **Any commit's contents.** Hashes and subject lines are corroborated
  by the reflog; **contents never have been, by any filing.**
- **That the `swop` divergence has collapsed.** It should — both
  implementations now return `InitialLab` from the same branch — and
  **nobody has re-measured it.** §3.24 deliberately does not assert it.

### ★★ Two numbers this filing weakened rather than updated

**Pass 6's throughput and speedup are now a RANGE.** Three readings of
the same binary on the same machine: **1,203 / 0,820 / 2,251 Mpix/s**
and **14,4× / 12,18× / 22,85×** — a **2,7×** spread on throughput and
**12,18–22,85×** on speedup. **No single figure is supportable.** The
honest form is **"12–23× on this machine, load-dependent"**, and the
break-even moves with it (≈70 000 px at grid 17 → ≈1,19 M → **1 258 593
px** today).

★★ **And a FOURTH, non-overlapping set exists.** `TOLERANCES.md` §3.6.2
records **2,4–2,7 Mpix/s, 28–32×, break-even ≈63 000–75 000 px** on the
`tools/difftest/src/pass6.rs` apparatus *(verified — read)*. **The
project holds two ranges that do not overlap and does not know why.**
A hypothesis is offered in `NUMERIC_CLAIMS.md` §3.23.4 and **is
labelled a hypothesis**; nobody has run the comparison. **`TOLERANCES.md`
is `icc-conformance`'s and was flagged, not edited.** ★ **Until it is
settled, no document may quote a single speedup figure at all.**

### Filed this session

| Where | What |
|---|---|
| `NUMERIC_CLAIMS.md` | **§1** — a new **`apparatus-census`** evidence class, with the reason it was added rather than leaving counts out. **§2.12** — the twelfth provenance block, including the three dispatch claims live source contradicted. **§3.22** — the apparatus census (**NC-158 … NC-160**), what the per-crate declaration match does and does not corroborate, and why the three numbers share no scale. **§3.23** — throughput and speedup **weakened to ranges** (**NC-161 … NC-163**), every reading with its apparatus, the document sweep, and **§3.23.4's two non-overlapping ranges**. **§3.24** — the 4.2.5.4 conformance defect (**NC-164**, **NC-164a**), NC-142 **re-attributed rather than edited**, the widened return type, and what the corpus gap says about itself. A **third dated note on NA-009** — its cost is **UNMEASURED again**, because the number standing in for it was a defect. **§7.12** — four owed items **discharged**, seven added. **§8** extended through **DL-032**. |
| `ARCHITECTURE.md` | **§1** — `iccce-measure` added to the crate layout with its invariants, its licence lineage and the Argyll prohibition; *"Five crates"* stated with the manifest as the authority. **§5** — **DL-030** (iccce was non-conformant; rule 7 ran against us), **DL-031** (an unlabelled test count is not a claim), **DL-032** (an expected warning is documented with what "fixing" it would break). DL-001 … DL-029 untouched. |
| `ROADMAP.md` | A dated header block (the open question **answered against us**, the fifth crate, the three runners, and the throughput claim weakened); a **Pass 10 pre-work record** for `iccce-measure` — why pre-work is available when the Pass is blocked, the surface, the report-don't-repair argument, the licence lineage, and four things the landing does **not** do; and a dated blockquote under the "what remains" block discharging the open question and correcting every bare test count in it. **No plan text and no earlier block rewritten.** |
| `SESSION_LOG.md` | This entry. |

**Not touched, by instruction and by ownership:** `README.md`,
`TOLERANCES.md`, `NEXT_SESSION.md`, everything under `tools/`,
`crates/` and `fixtures/`, every `Cargo.toml`, the corpus, `LEGAL.md`.
**`TOLERANCES.md` §3.6 and `docs/bench-2026-08-12.txt` were read as THE
SOURCE for this filing, not written.**

### Left for the next session to not assume

- **That the `swop` black-point divergence is now zero.** It **should**
  collapse and **nobody has re-measured it**. Until then **NA-009's
  cost is UNMEASURED**, and the number that used to stand in for it has
  been re-attributed to a defect.
- **That the corrected 4.2.5.4 branch is tested.** ★ **NC-164 was read
  from source, not run.** No test asserts that the short-circuit
  returns `initial_lab` unchanged — **and the defect shipped once
  through exactly that gap**, past 63 `iccce-cmm` tests.
- **That "the suite is green" means anything without a command.**
  Three runners: **129** (`cargo test --workspace`), **36**
  (`tools/difftest` units), **142** (the conformance runner). **They
  are not comparable.** And **`skip=3` has never been enumerated.**
- **That any single throughput or speedup figure is quotable.** Two
  non-overlapping ranges exist and the discrepancy is unexplained.
  **`README.md` has not been swept** for one.
- **That `iccce-measure` makes any colour claim.** Nothing in it has
  been compared to anything, deliberately. **It produces no ledger
  row.** And Pass 10 itself is **still blocked** on naming a ground
  truth that is not iccce — the reader is the half that never needed
  hardware.
- **That ISO/CD 18619 is a published standard.** It is a **committee
  draft**, and every consequence drawn from it — including DL-030 —
  inherits that.
- **That lcms2 conforming here makes it an authority.** One clause, one
  pin, one branch. **DL-027 stands**: it has two black-point estimators
  and a branch this project's first reading did not trace.
- **That the tip is pushed, or that any push was authorised.** Neither
  was checked at this filing. **Rule 9 and DL-024 are unchanged.**
- **That the librarian has a shell.** Two filings in a row without one.
  Ask, per session; never inherit.

---

## 2026-08-12 (later the same day, latest) — ★★★ THE COLLAPSE THAT DIDN'T: the divergence GREW 58,8× on the corrected code, because agreement with the oracle had been the SYMPTOM of our defect; a ledger row turns out to have carried a measured claim and an unmeasured one in one sentence; and the speedup is withdrawn outright

**Filed by:** `icc-librarian`, from a dispatch by `icc-engineer` ·
**Sixteenth filing overall, the fifth of this calendar day** ·
**No shell** — third consecutive filing without one · **Nothing was
committed and nothing was pushed;** both are the engineer's and the
operator's acts · **Tip `2881e19`** *(verified —
`.git/refs/heads/master` read: `2881e1903a85d8d531c5d573fb965f597c25354a`)*

> **★ THE DISPATCH ARRIVED FULLY TAGGED**, every claim marked
> `[VERIFIED — I ran it]` or `[CARRIED — conformance's, not
> re-derived]`, in explicit response to the last filing having caught
> three unsourced claims. **That discipline paid for itself inside this
> filing** — see "the correction that had nowhere to land" below.

### ★★★ 1. The re-measure reversed a prediction this librarian had filed

`NUMERIC_CLAIMS.md` §7.12 predicted, from the engineer, that NC-142's
`8,166 8×10⁻² ΔE76` **should collapse** once the 4.2.5.4 defect was
corrected and both implementations returned `InitialLab`.

| | before `fd34a44` | after |
|---|---|---|
| ISO 4.2.5 black (iccce) | `L* 16,489 806` | **`L* 11,772 365`** |
| lcms2 (reimplemented from `cmssamp.c`, pin `21c582a`) | `L* 16,571 474` | **unmoved** |
| **the divergence** (`swop`, T6) | `8,166 8×10⁻²` ΔE76 | **`4,799 109` ΔE76** |

**It grew 58,8×, and it is not a bug.** Both sides now return what
*their own document* calls `InitialLab`: **ISO 4.2.2.2 means the darkest
DEVICE VERTEX neutralised; lcms2's `cmsDetectBlackPoint` means the
PERCEPTUAL BLACK ROUND TRIP with chroma zeroed.** Two documents meaning
different things by one name — **rule 7 in its sharpest form, and this
time neither side is wrong.**

### ★★★ 2. The finding that outlives the number

> **AGREEMENT WITH THE ORACLE WAS THE SYMPTOM OF OUR DEFECT.**

The non-conformant return was `outRamp[first] = MinL = 16,489 806`, and
**`MinL(lcms2) = MinL(ISO) = 16,489 806` exactly.** So the defective
code landed **0,082 `L*`** from the oracle's answer **for a reason that
had nothing to do with being right** — it was returning a quantity the
oracle also computes. **The defect's own magnitude, `4,717 441 L*`, was
57,8× the divergence it was blamed for.**

★ **The cross-check built to catch exactly this class of error was
nearly blind to it**, and its power was two orders of magnitude below
the error it was pointed at. **CLAUDE.md rule 1 at its most literal: a
wrong colour agreed with the oracle to 0,08 ΔE76.** Filed as
**DL-033** — *a cross-check's power is bounded by the separation of the
two CANDIDATE answers, not by the tightness of the residual it
reports.* It is the **mirror of DL-028 and the more dangerous half**:
DL-028's failure mode announces itself, this one is silent.

### ★★★ 3. A ledger row that carried two claims — split, not corrected

**NC-164a** said the defect *"accounts for the whole observed gap"* AND
implied that fixing it would end the gap. **The first was measured and
is true. The second was never measured and is now false.** The engineer
asked for this to be recorded as **two claims with different evidence
status** rather than as one corrected number, which is the right call:
the failure mode *is* the conflation, and editing the row would delete
the evidence of it.

- **NC-174** — *the defect accounted for 100 % of the pre-correction
  divergence.* **implementation-cross-check. MEASURED, TRUE.**
- **NC-175** — *therefore fixing it ends the gap.* **No evidence class,
  because it was never measured. FALSIFIED.**

★ **The two were only ever equivalent if the residual had exactly one
cause, which nobody had established.** An unmeasured inference inherited
a measurement's authority **purely by adjacency**, and nothing flagged
it because nothing was wrong with the sentence's grammar or its
arithmetic. **§1 required a class per row; it did not require a row to
contain only one CLAIM. It does now.**

**§3.24 already did the hard half correctly and that is why this filing
is cheap:** it labelled the collapse as *expected* and explicitly
refused to assert it. ★ **Without that refusal this would be a
retraction instead of an observation** — the same argument §3.24.1 made
about withholding attribution, one level up.

### ★★ 4. The fixture that could not see it

`v4-rgb-mab-chromatic-black.icc`'s `InitialLab.L*` and `outRamp[first]`
are **both `L* 20`**. The defect swapped one for the other; **swapping
two equal numbers changes nothing**, so that arm's `5,000 000 ΔE76` is
*identically* unmoved. ★ **The fixture authored to discriminate the two
ESTIMATORS had, by construction, zero power to detect a defect in the
RETURN VALUE of one of them.** `USWebCoatedSWOP.icc` had the power
**because nobody designed it**. Filed as **DL-036**, the stated converse
of DL-020: an authored fixture discharges *the doubt it was authored
for* **and nothing adjacent to it**.

### ★★ 5. Two green rows got greener for bad reasons

| Row | Before | After | ★ What actually moved |
|---|---|---|---|
| **T1** — the DL-028 error-bar guard | `3,043×10⁻¹` | `5,179×10⁻³` | **The error bar did not change. The EFFECT grew 59×** |
| **T4** — reimplementation vs rival | `1,715×10⁻¹` | `4,258×10⁻²` | **The numerator did not change. The RIVAL got 4,03× worse** |

**A reader seeing only the numbers concludes the opposite of what
happened**, and both rows are correctly computed. **DL-035:** *an
improvement whose cause is the denominator or the rival is not an
improvement.* ★ **DL-018 one level up** — a gate can be made greener by
deleting the requirement it protects, and a ratio can be made greener by
an event in its denominator nobody chose.

### ★★ 6. The speedup is withdrawn outright; break-even survives with its grid

`icc-conformance` measured it ten more times and found it spans
**2,03× within ONE session at ONE grid** (grid 33; **at grid 17 the same
protocol spanned only 1,15×** — the instability is **not uniform across
grids**, which this librarian states because the dispatch's compression
implied both). The decision at `TOLERANCES.md` §3.6.3(b) is that **this
project does not carry a speedup figure at all.** §7.12's *"reconcile
the two non-overlapping ranges"* is therefore **discharged by
withdrawal, not by reconciliation** — ★ **and §3.23.4's labelled
hypothesis, that the two harnesses time different work, remains
UNTESTED and is re-listed as a question.**

**What survives is the break-even, always with its grid:**
**`85 900 → 1 273 800 px` = 14,8×**, matching the median build's
**`0,838 → 12,444 s` = 14,8×** to **three figures** — the arithmetic
saying the shift is *entirely* the build, since `N ≈ build ×
reference_rate` puts the noisy term where it barely enters (spread
**1,13×** over the same five runs).

**And `COMPILED_DE` is not derived on any compiled grid.** Its
derivation population is Pass 4's **341-point CMYK** iccce-vs-lcms2
comparison, and **`pass4.rs` never constructs a `CompiledTransform`**
*(verified — read)*. The old emitted string was wrong **twice**: `17`
was **stale**, and `derived for` was **a conflation wrong on the day it
was written**. **DL-034:** *a claim-bearing number the harness can
compute is formatted at run time, never typed into prose beside the code
that computes it* — **a stale comment misleads a reader; a stale string
in an emitted conformance record misleads the evidence.**

### ★★★ The correction that had nowhere to land

The engineer filed a correction **to himself**: that he had told this
librarian the **reference arm** had drifted outside its recorded
`0,076–0,091 Mpix/s` band, and that this was wrong.

**★ This librarian checked whether the ledger contained the claim before
writing a correction to it. It does not.** *(verified — `docs/` grepped
for `drift`/`drifted` and the band's figures.)* The two places
`NUMERIC_CLAIMS.md` mentions that band **both attribute it to
`TOLERANCES.md` §3.6.2 and flag it as unreconciled**. No document owned
by this librarian ever said the reference arm drifted.

What is true instead: **within a session the reference arm is the
TIGHTEST quantity measured — ±4 %, against ±35 % for the compiled arm**;
today's ten runs give `0,092–0,099 Mpix/s` at both grids. The old band
was **a four-sample range from one sitting quoted as a property of the
machine** — i.e. **the same error as the withdrawn speedup claim, not
evidence of drift.**

> **★★★ Fifth recorded instance of the dispatch and the tree
> disagreeing (§2.12), and the FIRST caught BEFORE the filing rather
> than after.** The four previous instances were errors in a dispatch
> that reached a document. This one would have produced **a correction
> to a statement no document contains** — precisely the failure the
> sibling project logged and the reason the verify-against-live-source
> rule is worded as it is. ★ **The engineer's own tagging discipline is
> what made it cheap:** the claim arrived labelled as a correction, so
> the first question was *"correcting what, where?"*, and the answer was
> *nowhere*.

### ★★ Two owed items discharged by this librarian's own reading

- **`README.md` carries NO throughput, speedup or break-even claim at
  all** *(verified — grepped at this filing for `Mpix`, `speedup`,
  `faster`, `×`, `break-even`; the only hit is an unrelated `1×10⁻⁴`)*.
  §7.12 newly-owed 4 is discharged **in the "there was nothing there"
  direction**. ★ **Recorded as a discharge rather than silently
  dropped**: *"we checked and found nothing"* is a different fact from
  *"we never checked"*, and only one of them is evidence.
- **`bpc.rs` L620–L703 now carries two tests naming 4.2.5.4**, one
  asserting `"InitialLab carried through"` and one asserting the whole
  triple survives on a **chromatic** `InitialLab` *(verified — read at
  the tip)*. §7.12 newly-owed 6 is **MOVED, not closed** — whether it
  discharges the item is `icc-conformance`'s call, not this
  librarian's.

### Filed this session

| Where | What |
|---|---|
| `NUMERIC_CLAIMS.md` | **§3.25** — the re-measure (**NC-165 … NC-167**), the methodological finding, **NC-164a SPLIT into NC-174/NC-175**, the blind fixture, **NA-009's cost measurable at last with its three caveats**, and the new one-claim-per-row convention. **§3.26** — the two rows that got greener for bad reasons (**NC-168**, **NC-169**). **§3.27** — `COMPILED_DE`'s derivation (**NC-170**), break-even's structural shift (**NC-171**, **NC-172**), the **speedup WITHDRAWN** (**NC-173**), the document sweep, the discharge-by-withdrawal of the two-ranges item, **§3.27.5's correction with nowhere to land**, and a coverage statement. A **fourth dated note on NA-009**. **§6** — five new dependency rows, including one that has **already fired**. **§7.13** — three items discharged, six added. **§8** extended through **DL-036**. |
| `ARCHITECTURE.md` | **§5** — **DL-033** (agreement with the oracle was the symptom of our defect), **DL-034** (claim-bearing numbers are formatted at run time), **DL-035** (an improvement caused by the denominator or the rival is not one), **DL-036** (a vendor profile stays in the fixture set). DL-001 … DL-032 untouched. |
| `ROADMAP.md` | A dated header block: the falsified prediction, the definitional divergence, DL-033, NA-009's measured cost with its caveats, the blind fixture, and the speedup's outright withdrawal with break-even stated in its place. **No plan text and no earlier block rewritten.** |
| `NEXT_SESSION.md` | Rewritten — it is this librarian's, and it said *"overwrite once acted on"*. Its previous headline table carried the now-superseded `8,166 8×10⁻²` as the live figure. |
| `SESSION_LOG.md` | This entry. |

**Not touched, by instruction and by ownership:** `README.md`,
`TOLERANCES.md`, everything under `tools/`, `crates/` and `fixtures/`,
every `Cargo.toml`, the corpus, `LEGAL.md`. ★ **`TOLERANCES.md` §3.6.3,
`tools/difftest/src/pass5c.rs`, `pass6.rs`, `pass4.rs` and
`crates/iccce-cmm/src/bpc.rs` were read as THE SOURCE for this filing,
not written** — the engineer's dispatch named them as already committed
and off-limits, and every figure attributed to them here was checked
against the file rather than transcribed from the dispatch.

### Left for the next session to not assume

- **That the corrected code is better because the number is bigger, or
  was better when it was smaller.** ★ **Neither figure grades
  correctness.** The **clause** graded it (NC-164, DL-030); the
  cross-check only ever measured distance to another implementation.
- **That NA-009's cost is a cost against truth.** It is **at the black
  point only**, **relative to lcms2 only**, and ★ **there is NO ground
  truth in this comparison** — no published black point exists for
  `USWebCoatedSWOP.icc` and **18619 is a committee draft**.
- **That the synthetic arm would catch a regression here.** It has
  **zero power** on this quantity, by construction. A 4.2.5.4
  regression would be **invisible on one of the two arms**.
- **That any cross-check row in the ledger states its candidate
  separation.** ★ **None of them do.** That is the highest-value item
  newly owed (§7.13), and DL-033 does not supply it.
- **That the two timing harnesses have been compared.** The speedup was
  withdrawn *around* the question, not by answering it.
- **That `skip=3` has been enumerated.** Eleventh filing; still blocked
  on a shell.
- **That a CI run has ever been observed here.** Sixteen filings, none.
  **The Linux debt is untouched.**
- **That the tip is pushed, or that any push was authorised.** ★ **The
  tip hash is now corroborated from `.git/refs/heads/master`** — and
  that says **nothing** about whether it was pushed. Rule 9 and DL-024
  unchanged.
- **That the librarian has a shell.** **Three filings in a row without
  one.** Ask, per session; never inherit.

---

## 2026-08-12 — the **candidate-separation apparatus** filing (third of the session, seventeenth overall)

**Tip:** `e26d9ba` *(**verified** — `.git/refs/heads/master` read; it
says **nothing** about whether that tip has been pushed)*.
**Filed by:** `icc-librarian`, **without a shell — fourth consecutive
filing.** **Dispatched by:** `icc-engineer`, with provenance tagged
throughout, which is what made this filing cheap.

### What arrived, and how it was treated

| Provenance | Content | What this librarian did |
|---|---|---|
| **VERIFIED by the engineer** (it ran the runner) | `pass=142 fail=0 skip=3 error=0`; the `separation` aggregate; three row figures; the `9 574×` margin | **Carried.** No shell here. Recorded as carried, per row |
| **VERIFIED by the engineer** (it enumerated them) | the three `skip` ids and their verbatim reason | **Carried — and the REASON's key phrases corroborated in `pass4.rs` L1210 and L1861**, read at the tip |
| **CARRIED** from `icc-conformance` | the apparatus design, GP-002, `TOLERANCES.md`'s new rows, the CI-power risk | **The MECHANISM was verified from source rather than accepted**: `lib.rs` L1441–L1476, L1653–L1695, L1865–L1902; `pass5c.rs` L1286–L1291, L1409–L1431; `TOLERANCES.md` §1.1 and §5; `tools/gen-profiles/README.md` §4.1 and `recipes.rs` L931/L1726 |

### The three findings worth carrying out of this session

1. **★★★ The row carrying the entire 4.2.5.4 finding is `UNGRADED`** —
   tolerance `inf`, separation `4,717 441`. **It could never have
   failed.** The `inf` was the *correct* call under DL-019; the finding
   is that a REPORTED row's separation was invisible, so *"we measured
   it"* and *"we could catch a regression in it"* were
   indistinguishable in the record. **They are now two printed states.**
2. **★★★ The corrected clause is documented, not defended, on any clean
   machine.** The only differential arm with power is a **Windows system
   profile**; on CI it skips and **a full reversion of `fd34a44` stays
   green.** A third arm is **commissioned, with its power to be
   demonstrated by injecting the reverted behaviour** — recorded as a
   commitment, **not as a result**.
3. **★★ A fourth stale literal, found by an apparatus rather than by a
   person** — and it **understated** its own argument by 4×, which is
   why nobody finds this kind by reading.

### ★ Two disagreements between the dispatch and the tree, both small, both recorded

- **The dispatch cited "§7.12 item 4"** for the skip enumeration.
  **§7.12's newly-owed 4 is the `README.md` throughput sweep**, already
  discharged at the sixteenth filing; the skip item is **newly-owed 3**
  *(verified — §7.12's numbered list read)*. **Nothing turns on it.
  Recorded because §2.12's tally is only useful if the small instances
  go in too** — this is the sixth.
- **"Open since the eleventh filing"** traces to **this librarian's own
  `SESSION_LOG.md` line** *(verified — *"Eleventh filing; still blocked
  on a shell"*)*. **It is our phrase, not the engineer's**, and this
  filing did not re-derive the count.

### ★★ What this librarian refused to round up

- **`blind=0` is out of 16, not out of 145** — the engineer's own
  phrasing, preserved verbatim on request. ★ **And this ledger sharpened
  it in the same direction, not the other:** strictly it is out of the
  **six** rows that reached the blind-vs-discriminating comparison; the
  other six measured rows were diverted by a guard before it.
- **The skip enumeration is discharged FOR THE CURRENT TIP.** The item
  was opened against **NC-160's** `skip=3`, at an **earlier** tip. Same
  count, structurally the same cause, **not established as the same
  three rows** — re-listed as newly-owed 4.
- **"The fourth instance of DL-034" is carried as a COUNT.** DL-034's
  entry names two; `lib.rs` records three. **The enumeration of the
  first three was not re-derived** — a count is not an inventory.
- **The runner is not named.** `pass=142` and the 145-row aggregate
  arrived **without a binary or command line**; the attribution to
  `tools/difftest` is an **inference from the row ids**. **DL-031 is the
  entry this violates**, and ★ **its founding instance included the
  number `129` — which the new aggregate now also prints, for a
  completely unrelated quantity.** Filed as a collision hazard before it
  can bite.

### Filed this session

| Where | What |
|---|---|
| `NUMERIC_CLAIMS.md` | **§3.28** — the apparatus's first run: **NC-176** (the UNGRADED row), **NC-177** (the ZERO-SEPARATION row, GP-002), **NC-178** (the healthy control at `9 574,451×` its bound); the coverage decomposition **145 → 16 → 12 → 6**; the `129` collision hazard; the fourth stale literal; the `skip=3` enumeration with its two limits; **§3.28.5's CI-power RISK**; GP-002 and the deliberate non-regeneration of the fixture; a coverage statement. A **fifth dated note on NA-009** carrying the `TOLERANCES.md` §5 pointer and its **fourth** caveat. **§7.14** — five items moved, five added. **§8** extended through **DL-037**. |
| `ARCHITECTURE.md` | **§5** — **DL-037** (candidate separation is an emitted field; the guard order is the decision; `BLIND` deliberately does not gate). **Dated notes appended to DL-033** (its "Revisit if" has fired; item 1 closed for Pass 5c, open elsewhere, 16/145) **and to DL-034** (the fourth instance, and the `NEUTRAL_EXACT` case the rule's original wording does not obviously reach). **DL-001 … DL-036 bodies untouched — appended to, never rewritten.** |
| `ROADMAP.md` | A dated block: the instrument, the UNGRADED row, the 16/145 coverage, the undefended-clause risk, the skip enumeration, the fourth stale literal. **No plan text and no earlier block rewritten.** |
| `SESSION_LOG.md` | This entry. |

**Not touched, by instruction and by ownership:** `README.md`,
`TOLERANCES.md`, everything under `tools/`, `crates/` and `fixtures/`,
every `Cargo.toml`, the corpus, `LEGAL.md`. ★ **`tools/difftest/src/lib.rs`,
`src/pass5c.rs`, `src/pass4.rs`, `tools/gen-profiles/README.md`,
`src/recipes.rs` and `docs/TOLERANCES.md` were read as THE SOURCE for
this filing, not written** — the dispatch named them as already
committed and off-limits, and every figure attributed to them here was
checked against the file.

### Left for the next session to not assume

- **That the suite would catch a 4.2.5.4 regression.** ★ **On a machine
  without a Windows system profile it would not** — the rows skip and
  everything stays green. **This is the most load-bearing sentence in
  the filing.**
- **That the third differential arm exists.** It is **commissioned**.
  Nothing has been measured, and this ledger asserts nothing about it.
- **That the separation field covers the suite.** **16 of 145**, all
  Pass 5c's. **129 rows print `UNSTATED`.**
- **That `blind=0` means the suite is not blind.** It is a statement
  about **six** rows.
- **That converting `unstated` to `no-named-alternative` is progress.**
  Without a reason **per row** it destroys the field's meaning while
  making the aggregate look finished (§7.14 newly-owed 1).
- **That NC-160's three skips are the three enumerated here.** Same
  count, same structural cause, **not established**.
- **That the `129` in the separation aggregate has anything to do with
  the `129` in `cargo test --workspace`.** Two apparatus, two
  quantities, one coincidence.
- **That a CI run has ever been observed here.** **Seventeen filings,
  none.** The Linux debt now has a named cost.
- **That the tip is pushed, or that any push was authorised.** The hash
  is corroborated from `.git/refs/heads/master`; that says **nothing**
  about pushing. Rule 9 and DL-024 unchanged.
- **That the librarian has a shell.** **Four filings in a row without
  one.** Ask, per session; never inherit.

> #### ★★★ Dated correction, 2026-08-12 (fourth filing, tip `6c7cda1`) — **the first bullet of this list is FALSE and the entry is not rewritten**
>
> *"That the suite would catch a 4.2.5.4 regression — on a machine
> without a Windows system profile it would not"*, called here **the
> most load-bearing sentence in the filing**, was measured the same day
> and **fails in both directions**: `cargo test -p iccce-cmm` **does**
> fail on a full reversion (**exit 101, 62 passed / 2 failed**), so the
> clause was defended at unit level all along; and **no conformance row
> went red on ANY machine**, so the vendor arm was not the detector
> either. ★ **A claim's prominence is not evidence about it.** The
> corrected pair is `NUMERIC_CLAIMS.md` **§3.29.1**; §3.28.5 carries an
> amendment box. **The third bullet's `16 of 145` is also superseded —
> it is `41 of 160` now, and `16` has become the name of a different
> quantity.**

---

## 2026-08-12 — the **third-arm + ground-truth** filing (fourth of the session, eighteenth overall)

**Tip:** `6c7cda1` *(**verified** — the last line of `.git/logs/HEAD`,
which now holds **68** lines; it says **nothing** about whether that tip
has been pushed)*. **Filed by:** `icc-librarian`, **without a shell —
fifth consecutive filing.** **Dispatched by:** `icc-engineer`, with
provenance tagged throughout.

### ★★★ The headline is a retraction, and it is not this librarian's catch

The seventeenth filing closed on a sentence sourced from the engineer's
dispatch and quoted here as *"the sentence to carry out of it"*:

> *"the 4.2.5.4 correction is documented but undefended on any clean
> machine, and the platform where the detector is absent is the one
> platform this project has never run on."*

**The engineer injected the defect, measured it, and refuted its own
claim before this ledger filed on it a second time.**

| Half | Verdict |
|---|---|
| *"undefended"* | **FALSE.** `cargo test -p iccce-cmm` on a full reversion of `fd34a44`: **exit 101, 62 passed / 2 failed** — both `bpc::tests` clause tests. **The clause was defended all along, as a function, on a synthetic closure** |
| *"absent on one platform"* | **AN UNDERSTATEMENT.** The reversion turned **no conformance row red on any machine**. `swop` moves (`5,18×10⁻³` and `4,26×10⁻²` against bounds of `1`) but nothing crosses, because the row carrying the finding is `REPORTED`. **There was no conformance detector anywhere** |

**The corrected pair, which is narrower and more useful:** the clause was
defended at **unit** level on a **closure**; what had no detector was the
clause exercised **through a parsed profile**, where a `Chain`/estimator
wiring defect lives that a closure test structurally cannot reach.
★ **Verified in the source, which is the strongest form available** —
`pass5c.rs` L2713–L2727 says exactly that, in the tree.

**Seventh dispatch-vs-tree instance in the tally, and the first the
dispatcher caught.** Its own framing, preserved: *"the same error class
as the six already in the tally: a claim about the tree that I did not
read the tree for."*

### What arrived, and how it was treated

| Provenance | Content | What this librarian did |
|---|---|---|
| **VERIFIED by the engineer** (it ran them) | the reversion outcome; `pass=157 fail=0 skip=3`; the separation aggregate; `40 identical`; `131 passed` | **Carried.** No shell here. ★ **Two corroborated without one**: **131 `#[test]` declarations across 20 files under `crates/`** matches `131 passed` **exactly**, and `fixtures/synthetic/` holds exactly **40** `.icc` files |
| **CARRIED** from `icc-conformance` | the proof-of-power run, the mechanism defect, `DEVICE_OBSERVABLE`, Pass 4c's ten separations | **The mechanisms were VERIFIED from source, not accepted**: `lib.rs` L1382–L1435; `pass5c.rs` L1279–L1356, L1642–L1665, L2255–L2373, L2660–L2795; `pass4c.rs` L272, L486–L495, L1244–L1280; `recipes.rs` L1150–L1173, L1253–L1264; `bpc.rs` L604–L709; `TOLERANCES.md` §4 |
| **CARRIED** from `icc-spec-librarian` | the ground-truth survey and corpus defect **C5** | **Read the corpus file itself** — `ICC_Spec\icc\icc__ref__ground_truth_availability.md` §1, §4, §7, §8, and the amendment box in the corpus index |

### The four findings worth carrying out of this session

1. **★★★ The clause now has an instrument whose expectation is a
   clause.** Bound `7,629 511×10⁻⁴` = **half one PCSLAB quantum and
   nothing else**; the reversion fails it by **`3,28×10⁴`**. **Proven by
   injection with the vendor paths disabled** — the only failure in the
   suite, `pass=129 fail=1 skip=30`.
2. **★★★ The separation mechanism was lying exactly where it mattered.**
   `|observed − alt_observed|` collapses to zero on the defect run, so
   the row **failed at `2,500 019×10¹` while printing
   `ZERO-SEPARATION`**. **DL-038.** DL-037 recorded the guards; this
   records that the measurement *under* them had the defect the guards
   were built to catch.
3. **★★★ The eleven-filing ground-truth blocker was partly a wrong
   REJECTION.** Annex D.6.3 was examined months ago and disqualified by
   point-evaluating intervals. ★ **A negative finding removes its own
   auditor.** **DL-042** binds every future `§7.x` pass: **re-audit the
   REASON, not just the item.**
4. **★★ A control failed at `3,775×10⁹` and was not accommodated.**
   `APPARATUS_RATIO` stayed at `1.0`; the exemption was **declared** in
   an authored table and **graded** against the measurement. **DL-043.**

### ★★ What this librarian refused to round up

- **The ground truth is AVAILABLE, not MEASURED.** Annex D.6.3's twelve
  integers and Table 16's five pairs were reproduced **by the corpus's
  arithmetic, not by iccce's code. Nothing in this project has been
  compared to either**, so **NC-001 remains the only
  `published-ground-truth` row** and §3.29 must never be cited as having
  closed that gap. ★ **This is the single easiest sentence in the filing
  to get wrong.**
- **The arm's power is proven against ONE injected defect.** A row that
  catches the bug it was written for is not a general detector of 4.2.5.4
  faults.
- **`pass=129 fail=1 skip=30` and `pass=157 fail=0 skip=3` are two
  APPARATUS STATES, not a trend** (DL-031) — the first ran with both
  category (c) paths repointed at a non-existent drive.
- **Three separation rows moved is a count, not an inventory.** Whether
  **NC-176 … NC-178** use the defective constructor **was not
  established**.
- **The fixture's blindness is stated with its power.** It cannot
  separate lcms2's `L*` from `InitialLab`'s — same vertex, same `A2B`.
  **Two further instruments are named and OWED**, and named-and-owed is
  not coverage.
- **CI is configured for ubuntu, not observed.** **Eighteen filings, no
  CI run seen.** A workflow file is not a run.
- **`NC-181`'s observation is not on file.** The `FIXTURE` row is
  recorded with a tolerance and **no measurement**.

### ★ A number found by counting, and owed to someone else

`.github/workflows/ci.yml` asserts **twice** that `tools/difftest` has
**43 tests**; the declaration count at this tip is **47**
*(verified — counted; `gen-profiles`' companion **28** still matches,
which is what makes the basis comparable)*. **A candidate fifth instance
of DL-034.** Which tests are new was **not** established. **`.github/`
is not this librarian's to edit — recorded as owed.**

### Filed this session

| Where | What |
|---|---|
| `NUMERIC_CLAIMS.md` | **§2.13** — the thirteenth provenance block. **§3.29** — the corrected pair (**NC-183**, **NC-184**), the third arm and its proof (**NC-179 … NC-182**), the declared exemption (**NC-185**), Pass 4c's separations (**NC-186**), the census with **both** number collisions (**NC-187**), `verify` and the workspace suite (**NC-188**, **NC-189**), the ground-truth survey (**NC-190**), the CI count (**NC-191**), and a coverage statement. **An AMENDMENT BOX over §3.28.5**, which is *not* rewritten. **§7.15** — four items discharged or converted, six added. **§8** extended through **DL-043**. |
| `ARCHITECTURE.md` | **§5** — **DL-038** (fixture, not run), **DL-039** (what counts as a rival), **DL-040** (an `UNGRADED` separation is a request for a fixture), **DL-041** (the LUT path's ground truth cannot exist), **DL-042** (a negative finding removes its own auditor), **DL-043** (a declared exemption). **Dated notes appended to DL-036** (its second *"Revisit if"* has **fired**, and its headline is corrected: the vendor arm could **observe** the divergence and never could **catch a regression** in it) **and to DL-037**. **DL-001 … DL-037 bodies untouched — appended to, never rewritten.** |
| `ROADMAP.md` | A dated block: the retraction, the proven third arm, the lying mechanism, the unwidened control, Pass 4c, the split ground-truth blocker, and the one operator question. **No plan text and no earlier block rewritten.** |
| `SESSION_LOG.md` | This entry, plus a **dated correction box** on the previous entry's first "do not assume" bullet. |

**Not touched, by instruction and by ownership:** `README.md`,
`TOLERANCES.md`, `.github/`, everything under `tools/`, `crates/` and
`fixtures/`, every `Cargo.toml`, the corpus, `LEGAL.md`. ★ **The files
listed in the provenance table above were read as THE SOURCE for this
filing, not written.**

### Left for the next session to not assume

- **That iccce has been compared to published ground truth.** ★ **It has
  not.** Annex D.6.3 and Table 16 are **available and unmeasured**;
  **NC-001 is still the only such row**, and the fixture is blocked on
  an **operator** decision about published numbers in an MIT repo.
- **That the 4.2.5.4 clause is now generally defended.** It is defended
  **at unit level on a closure** and **through one parsed profile
  against one injected defect**.
- **That the separation field covers the suite.** **41 of 160.** **119
  rows print `UNSTATED`.**
- **That `16` means what it meant last filing.** It was
  *rows-stating-a-separation out of 145*; it is now the
  **`discriminating`** count out of 160. **Always carry the
  denominator.**
- **That the earlier separation rows are safe from DL-038's defect.**
  **Three named rows were fixed. No retro-audit was done.**
- **That a CI run has ever been observed here.** **Eighteen filings,
  none** — ubuntu is now in the workflow *file*.
- **That the tip is pushed, or that any push was authorised.** The hash
  and four commit subjects are corroborated from `.git/logs/HEAD`; that
  says **nothing** about pushing. Rule 9 and DL-024 unchanged.
- **That an item restated as "still owed" has been re-tested.**
  **DL-042.** Restating a blocker is not re-testing it, and a *negative*
  finding is the kind nobody goes back to.
- **That the librarian has a shell.** **Five filings in a row without
  one.** Ask, per session; never inherit.

## 2026-08-17 — the **request-channel registration** filing (first of the session, nineteenth overall). **No Pass shipped, no code changed, no numeric claim changed**

**Tip:** `e21154c`, branch `master` *(**verified — I ran `git log` and
`git branch --show-current`**)*. **Working tree:** `CLAUDE.md` and
`docs/NEXT_SESSION.md` **modified, uncommitted** — the engineer edited
both after taking its own clean-tree measurement, so **do not restate
the tree as clean**; the dispatch said so and the disk agrees.
*(**verified — `git status --short`**)*.

**Filed by:** ★ **a general-purpose agent standing in for
`icc-librarian`.** The agent type could not be dispatched: the
dispatching session's working directory is `D:\Dev\pdfce`, so `iccce`'s
project agents are not in its roster (the Agent tool returned *"Agent
type 'icc-librarian' not found"* and listed only `pdfce-*` agents). The
stand-in read `.claude/agents/icc-librarian.md` and followed it as its
operating contract. **Dispatched by:** `icc-engineer`, with an addendum
mid-filing correcting one of its own framings.

> ★★ **ONE DELIBERATE DIFFERENCE FROM THE REAL AGENT: THIS FILING HAD A
> SHELL.** Six consecutive filings before it did not. **Every "carried"
> figure in the dispatch that could be re-derived on this machine WAS
> re-derived**, and the tags below say which. A filing that looks like
> the librarian's but was not made by it is exactly the provenance this
> project tracks — hence this paragraph rather than a footnote.

### What happened: a communication channel now exists, and it has a consumer at the other end

The operator created
**`D:\Dev\FeatureRequests\iccce_FeatureRequests\`** on 2026-08-17 as an
asynchronous request channel between this session and the **`pdfce`**
session at `D:\Dev\pdfce\` — a PDF engine with **no colour management at
all**, and the consumer this project's own `README.md` names first. The
`pdfce` session set the folder up and filed the first three items;
`icc-engineer` read all four files, assessed the crates, and wrote one
reply.

★ **Requests flow BOTH ways.** Unlike the GUI channel `pdfce` also runs
— where `pdfceGUI` asks and `pdfce` answers — this one is not a one-way
queue. **`iccce` may write `open/request_<topic>.md`.** A consumer's
real usage is the best available check on this library's API shape, and
that check only works if this side asks. **This project has never used
it.**

**The channel folder is in NO git repository, deliberately.** Binding
consequence: **nothing may exist only there.** That is why this filing
exists — the durable half lands here, in git.

| File in `open/` | Direction | What it is |
|---|---|---|
| `request_pdf_output_intent_cmyk.md` | pdfce → iccce | pdfce ignores PDF/X `/OutputIntents` entirely and converts `DeviceCMYK` through a **pdfium-fitted 6×6×6×6 baked lookup table** (`crates/pdfce-core/src/color/cmyk_table.rs`). Asks whether iccce can take a document-embedded destination profile instead |
| `request_iccbased_colour_spaces.md` | pdfce → iccce | pdfce parses `ICCBased` **in full and then does not use it**, rendering through ISO 32000-1 Table 66's `/Alternate` fallback. Carries the design question: **does iccce construct sRGB internally, or demand a caller-supplied destination profile in every case?** |
| `note_boundary_and_overprint.md` | pdfce → iccce | Informational, **no reply owed**. Draws the boundary; **overprint is pdfce's** — compositing, not conversion |
| `reply_capability_status.md` | iccce → pdfce | `icc-engineer`'s reply. Answers **only** current capability status plus the wasm32 and licence questions |

★★ **The reply deliberately does NOT answer either design question.**
Both need real engineering thought and this was a scoping dispatch. That
declination is stated in the reply by name, so nothing is read as
settled.

### The measured facts, and who measured them

★ **`[VERIFIED — I ran it]` below means the stand-in librarian ran the
command on this machine at this tip.** `[CARRIED]` means the engineer's
figure was accepted without re-derivation.

| Claim | Tag | Detail |
|---|---|---|
| `cargo test --workspace` → **132 passed, 0 failed, exit 0** | **`[VERIFIED — I ran it]`** | ★ **And the inventory, per DL-031**: `iccce-cli` main **0** · `iccce_cmm` lib **64** · `tests/annex_d_ground_truth` **1** · `iccce_color` **25** · `iccce_measure` **8** · `iccce_profile` **34** = **132**, plus **four doc-test targets running 0 tests each**. ★★ **`tools/difftest` is NOT in this number** — it is a separate tree and the workspace suite does not run it. **132 is a count of the library suite, not of coverage, and specifically not of the pdfce integration path, which has ZERO tests** |
| `cargo build --target wasm32-unknown-unknown -p iccce-cmm -p iccce-profile -p iccce-color -p iccce-measure` → **exit 0** | **`[VERIFIED — I ran it]`**, with one honest qualification | The re-run finished from a **warm `target/` in 0.02 s**, so the **compilation itself was the engineer's run**; what this filing verified independently is that the four artifacts exist — `target/wasm32-unknown-unknown/debug/libiccce_{cmm,color,measure,profile}.rlib` — and that cargo accepts the tree for that target at exit 0. `rustup target list --installed` shows `wasm32-unknown-unknown` present |
| **First time any iccce code has been built for wasm32** | **`[CARRIED]`** | The target was not previously installed; it was added via `rustup target add` to run this check. **This filing can verify the target is installed NOW; it cannot verify when.** ★ **It is NOT gated in this project's CI** |
| `grep -c '^\[\[package\]\]' Cargo.lock` → **5** | **`[VERIFIED — I ran it]`** | And **enumerated**, which is the part that matters: `iccce-cli`, `iccce-cmm`, `iccce-color`, `iccce-measure`, `iccce-profile`. **The whole dependency graph is the five workspace crates. Zero third-party dependencies** |
| Every `std::fs` call in a library crate sits inside `#[cfg(test)]` | **`[VERIFIED — I ran it]`** | Checked as line numbers against each module's `#[cfg(test)]` boundary: `compiled.rs` 209 > 202 · `gray_trc.rs` 134 > 124 · `lut_ab.rs` 469 > 312 · `matrix_trc.rs` 510/652/687/699 > 433 · `named_color.rs` 244/245/282 > 188 · `transform.rs` 752… > 740. **The only non-test I/O in the workspace is `iccce-cli`** (`main.rs` 91, 260, 398) |
| **iccce constructs no sRGB destination** | **`[VERIFIED — I ran it]`** | `Chain::new` (`crates/iccce-cmm/src/transform.rs:246`) requires **two parsed `&Profile`s**. Every sRGB reference in `crates/` is inside a `#[cfg(test)]` block reading `C:\Windows\System32\spool\drivers\color\sRGB Color Space Profile.icm` off local disk |
| The capability surface | **`[VERIFIED — I ran it]`** | `Profile::parse(&[u8])` → `iccce-profile/src/lib.rs:80` · `CompiledTransform::convert` → `iccce-cmm/src/compiled.rs:171` and `convert_buffer` → `:180`, both writing into `&mut [f64]` (**allocation-free per pixel**) · `ChainError` → `transform.rs:88` · `ParseError` → `iccce-profile/src/diag.rs:28`, which **implements `Display` at `diag.rs:53` and `std::error::Error` at `diag.rs:83`** · ★ **`ChainError` implements `Display` (`transform.rs:134-169`, a match arm per variant, each producing a printable sentence) but NOT `std::error::Error`** — ★★ **CORRECTED IN-FILING; the first version of this row said "NEITHER `std::error::Error` NOR `Display`" and that was FALSE.** See the boxed note below. ★ **The whole evaluation surface is `f64`**; `f32` appears in `crates/` only in doc comments warning that ΔE2000's `C̄'⁷` overflows it |
| ★ `Chain::convert` allocates | **`[VERIFIED — I ran it]`** | `transform.rs:643` returns `Result<Vec<f64>, ChainError>` — a `Vec` per call. **`CompiledTransform::convert` is the per-pixel-safe one**; the two are easy to confuse by name and the reply says so |
| The bench (`docs/bench-2026-08-12.txt`: 1.293 Mpix/s loaded, 1.466–1.477 quiet, 12.2×–16.2× speedup) and the conformance run `pass=157 fail=0 skip=3` | **`[CARRIED — not re-derived]`** | **Not re-run.** ★ **And coverage is part of the claim (DL-021):** the bench is **one profile pair, one direction, one tag type**, at **grid 17** while the recommended grid for CMYK is 33 |
| `NUMERIC_CLAIMS.md` **NC-048** (0.25294 ΔE2000) and **NC-049** (1.6590) | **`[CARRIED — not re-derived]`** | Read, not re-measured |
| The LUT-path ground-truth gap is **STRUCTURAL** | **`[VERIFIED — I ran it]`** | Read at `docs/NUMERIC_CLAIMS.md:6488` and `crates/iccce-cmm/tests/annex_d_ground_truth.rs` module doc (the clause spans roughly L27–L32, not exactly L29–L32 as dispatched — **substance identical**). ICC.1 mandates no interpolation method, so no published ground truth for a LUT path can exist **even in principle**, corroborated because **iccDEV, ICC's own reference implementation, ships zero expected colour values** |

### ★★ A `[VERIFIED — I ran it]` claim in this very entry was FALSE, and the shape is worth more than the fact

**The row above originally read *"`ChainError` implements NEITHER
`std::error::Error` NOR `Display`"*.** The `Display` half was wrong.
`crates/iccce-cmm/src/transform.rs:134` is
`impl std::fmt::Display for ChainError`, a match-arm block covering all
seven variants and ending at `:169`. **Caught by a peer session
cross-checking this filing, not by the filing agent.** Corrected in
place, in both files, the same day.

> ★★★ **THE CAUSE: an ABSENCE-CLAIM IS ONLY AS STRONG AS THE BREADTH OF
> THE SEARCH THAT FAILED TO FIND IT.** The grep behind it searched
> `impl fmt::Display` and `impl Display for` — **neither matches the
> fully-qualified `impl std::fmt::Display for` that this codebase
> actually uses in all sixteen of its `Display` impls** *(verified — I
> re-ran it broadly: `clut.rs:81`, `curve.rs:92`, `gray_trc.rs:46`,
> `lut_ab.rs:68`, `lut_transform.rs:99`, `matrix_trc.rs:126`,
> `named_color.rs:43`, `transform.rs:134`, `iccce-measure/src/lib.rs:182`
> and `:213`, `diag.rs:53` and `:126`, `header.rs:48`, `num.rs:114`,
> `tag_types.rs:113` and `:222`)*. **A pattern narrower than the claim it
> is used to rule out produces a negative finding that cannot fail** —
> **§5.3 in grep form**, and **DL-042's class**, because a negative
> finding is the kind nobody goes back to.

★ **Two consequences, and the second is the one to carry.** The
correct statement is narrower and **less alarming**: only the
`Box<dyn Error>` / `?`-chaining path is blocked; `pdfce`'s actual stated
need — *"a named refusal so pdfce can print **why** it fell back"* — is
**already served today**. The wrong version would have sent a consumer
looking for a capability it already had.

★★ **And note that this is the SECOND provenance wobble in a single
filing** — the first being the stand-in substitution itself. **Both were
caught by cross-checking, neither by the agent doing the work.** The
`[VERIFIED — I ran it]` tag records *that a command was run*; **it does
not certify that the command asked the right question.** *(Recorded here
deliberately and **not** as a decision-log entry: the rule it
demonstrates is `NEXT_SESSION.md` §5.3, which already exists.)*

### ★★★ The finding that must never be written the other way round

**pdfce adopting iccce would be a LATERAL move in evidence class.**
iccce's oracle for a CMYK LUT path is **lcms2**; pdfce's current table is
fitted to **pdfium**; both are **cross-checks against another
implementation**, and per **DL-041** the LUT-path ground-truth gap is
**structural**, not an availability gap.

> **★★★ The defensible case for adoption is CONFORMANCE** — ISO 15930 /
> ISO 32000-1 §14.11.5, the document's declared output intent being
> honoured **at all** — **NOT accuracy.** Filed as **DL-044** precisely
> because the inversion is the error the entry exists to prevent.

### ★ The engineer corrected its own framing mid-filing, and it sharpens the first gap

`icc-engineer` sent an addendum retracting a line from its first draft of
the reply that called the sRGB blocker *"one dispatch to
`icc-spec-librarian` wide"*. **That framing is wrong and was corrected in
the reply file and in `NEXT_SESSION.md` before this filing landed**
*(**verified — `NEXT_SESSION.md` §3.0 now reads "Do not scope this as
'one dispatch to `icc-spec-librarian`'"**)*.

The reason, **verified in the ledger this filing read**:

- **`NUMERIC_CLAIMS.md:623`** (NC-018) — the D65 chromaticity is
  **single-source: lcms2 `cmsvirt.c` alone, because IEC 61966-2-1 is
  PAYWALLED and was not obtained.** **Not** cross-verified, unlike D50
  and Bradford. The row's own header calls it **"the weakest constant in
  the crate"**, and it records **ITU-R BT.709 as free from ITU and never
  fetched** — an un-taken independent route.
- **`NUMERIC_CLAIMS.md:976`** — *"the shared-misreading risk is
  **ELEVATED** here, not merely present … the corpus against which any
  future ground-truth check would be built **shares an origin with the
  oracle**."*

★★ **So a computed sRGB destination built today would take its white
point from the very implementation iccce cross-checks against — and
would then sit underneath every `ICCBased` conversion pdfce made.**
Obtaining the document is an **operator act** (paywalled, same class as
`ICC.1:2010-12` in `NEXT_SESSION.md` §2.2), **not** an agent dispatch.

★ **This is DL-042's failure mode caught before it started: a gap whose
stated reason is wrong is a gap nobody re-audits.** Recorded with its
reason attached rather than as a bare absence. **No new ledger row was
created; `NUMERIC_CLAIMS.md` was not touched — the rows already say it
correctly and simply had not reached the place that needed them. This is
propagation, not a finding.** The twelve-filing *"IEC 61966-2-1 has never
been dispatched for"* chain is at `NUMERIC_CLAIMS.md` **5788**, 6189,
6279, 6356, 6429 *(**verified — read 5788**; the engineer's first draft
said 5766 and corrected itself)*.

### The four gaps found, with NO `ROADMAP.md` entry, deliberately not scoped

★ **None of these was scoped into a Pass, and that is the instruction,
not an oversight.** Recorded so a later session finds them without
re-deriving them.

1. **No computed sRGB destination.** Today a caller must supply both
   profiles, **by omission rather than by decision**. ★ **Its blocker is
   PROVENANCE, not effort** — see the section above. **Do not restate it
   as trivial.**
2. **No `f32` / `u8` evaluation surface.** Everything is `f64`. A
   consumer rasterising 8-bit pixels converts in and out on every call.
3. **`ChainError` implements `Display` (`transform.rs:134-169`) but NOT
   `std::error::Error`**, while `ParseError` implements both
   (`diag.rs:53` and `:83`). ★ **The narrow half is what is blocked: a
   caller cannot `?`-chain a `ChainError` into a `Box<dyn Error>`.** The
   **message text is present and printable today** — every one of the
   seven variants has a match arm producing a sentence — ★★ **so
   `pdfce`'s stated need, "a named refusal it can PRINT", is ALREADY
   SERVED.** Only the boxed-error path is not. *(This item said
   "neither" in the first version of this entry; see the boxed note
   below.)*
4. **No public signature→component-count helper.** `Header::color_space`
   is a bare `Signature` (`iccce-profile/src/header.rs:68`); every channel
   count in the tree (`CompiledTransform::input_channels`,
   `Chain::input_channels`, `LutAB::device_channels`) hangs off an
   **already-built** object. ★ **So `pdfce` cannot validate `ICCBased`'s
   `/N` against the profile without building a chain first** — which is
   exactly the check ISO 32000-1 §8.6.5.5 requires and real files get
   wrong.

### ★★ No exchange was closed; no `INDEX.md` row was added

**A row is added when an exchange closes, and none has.** `INDEX.md`
carries its placeholder row only *(**verified — read; it says "no
exchange has closed yet; the channel opened 2026-08-17"**)*. Both
requests remain in `open/` alongside the reply, because the reply
declined both design questions.

### Filed this session

| Where | What |
|---|---|
| `ARCHITECTURE.md` | **§5 — `DL-044`**, one entry: the named external consumer, the standing bidirectional channel, `pdfce`'s three hard gates becoming inputs to **rule 9**'s dependency classification, the dated wasm32 evidence with its **not-CI-gated** caveat, the **lateral-in-evidence-class** adoption finding, and the sRGB-provenance clause. **DL-001 … DL-043 untouched.** |
| `SESSION_LOG.md` | This entry. **No earlier entry rewritten or annotated.** |

**Not touched, by instruction:** `docs/ROADMAP.md` and
`docs/NUMERIC_CLAIMS.md` — **no Pass was scoped and no new tolerance or
measured error was produced**, so neither had anything to receive.
**Not touched, by ownership:** `CLAUDE.md` (project instruction file) and
`docs/NEXT_SESSION.md` (engineer-written, explicitly overwritable) —
both were **edited by `icc-engineer` this session** and are **referenced
here, not written**:

- **`CLAUDE.md`** gained **rule 10**, *"★ FIRST, EVERY SESSION: check the
  request channel"*, between rule 9 and the cross-project-RAG section
  *(**verified — read; it carries all four load-bearing facts:
  requests-flow-both-ways, pdfce's three hard gates, overprint-is-pdfce's,
  and the pdfium-fitted table being a cross-check not ground truth**)*.
- **`docs/NEXT_SESSION.md`** gained **§0** *"FIRST, EVERY SESSION: CHECK
  THE REQUEST CHANNEL"* before §1, queue item **§3.0**, two rows in §4's
  "WHERE THINGS LIVE" table, and an amended read order and opening
  blockquote *(**verified — read §0, §3.0 and the read-order block**)*.

Also **`TOLERANCES.md`, `README.md`, `.github/`, `tools/`, `crates/`,
`fixtures/`, every `Cargo.toml`, the corpus and `LEGAL.md`** — untouched.
★ **Everything in the provenance tables above was read as THE SOURCE for
this filing, not written.**

### ★ Two things found by running, and owed to someone else

- **`.github/workflows/ci.yml` does not gate `wasm32-unknown-unknown`**,
  and **a consumer's CI does**. So a dependency added here can break
  `pdfce`'s gate with **nothing in this repository going red**. Adding
  the target is the obvious remedy; **`.github/` is not the librarian's
  to edit — recorded as owed.** *(The absence is inferred from the
  dispatch and from §0's own statement that iccce does not gate wasm32;
  **the workflow file itself was not read this filing** — tagged so the
  next session re-checks rather than inherits.)*
- **All five `Cargo.toml`s declare BOTH `license.workspace = true` AND
  `license-file = "../../LICENSE"`**, so every build prints five cargo
  warnings *(**verified — I ran the build and read the warnings; and
  grepped the manifests**)*. Pre-existing, harmless, and **not the
  librarian's to edit**. Noted because **DL-032** says an expected
  warning is documented where it fires, and this one is currently
  documented nowhere.

### Left for the next session to not assume

- **That wasm32 is "supported".** ★ **It built once, today, at one tip,
  uninstrumented by CI, and this filing's own re-run was warm-cache.**
  Not a standing guarantee. **DL-044** says so in the entry itself.
- **That 132 passing tests is coverage.** It is the library suite only —
  **`tools/difftest` is not in it**, and **the pdfce integration path has
  zero tests**. **DL-031.**
- **That the sRGB gap is one dispatch away.** ★ **It is not, and the
  engineer retracted that framing itself.** Its blocker is a **paywalled
  document and an operator act**; the cheap independent lever is **ITU-R
  BT.709**, free and never fetched.
- **That either design question has been answered.** `reply_capability_status.md`
  answered *what exists* and **declined both by name.** Nobody has
  decided whether §3.0 preempts §3.1.
- **That the four gaps are on the roadmap.** ★ **They are not.** No
  `ROADMAP.md` entry exists for any of them.
- **That an exchange has closed.** **`INDEX.md` has no rows.**
- **That this project has ever ASKED pdfce anything.** ★ **It has not.**
  The offer on the table — a census across pdfce's ~6,000-file corpus for
  `/N` distribution, v2-vs-v4, device class and **tag type** — is
  unclaimed, and §4.4's recommended-grid constant rests on **one profile
  pair, one direction, one tag type**.
- **That the tree is clean.** It is **not** — `CLAUDE.md` and
  `docs/NEXT_SESSION.md` are modified and uncommitted *(verified)*, and
  **DL-044 and this entry make it three files.**
- **That the tip is pushed, or that any push was authorised.** Nothing in
  this filing went near a remote. **Rule 9 and DL-024 unchanged.**
- **That a CI run has ever been observed here.** **Nineteen filings,
  none.** A workflow file is a configuration, not a run.
- **That a `[VERIFIED — I ran it]` tag means the claim is right.** ★★
  **One in this very entry was false** — see the boxed note above. The
  tag certifies **that a command ran**, not that it **asked the right
  question**. For an absence-claim, **quote the pattern**, and for a
  trait impl search the **fully-qualified** form
  (`impl std::fmt::Display for`, `impl std::error::Error for`) — this
  codebase uses it in **all sixteen** of its `Display` impls.
- **That the librarian filed this.** ★★ **It did not.** A
  general-purpose agent followed `icc-librarian.md`. **Ask, per session;
  never inherit** — including whether the agent type is dispatchable
  from the session's working directory at all.

---

## 2026-08-17 — the **Ghent compatibility** filing (second of the session, twentieth overall). **A new corpus, a new durable doc, three decision-log entries, eight ledger rows — and NO accuracy claim**

**Tip:** `e21154c`, branch `master` **[CARRIED — this librarian has no
shell and did not run `git`]**. **Working tree: NOT clean, and it was not
clean before this filing began** — the dispatch reports
`docs/ARCHITECTURE.md`, `docs/NEXT_SESSION.md`, `docs/SESSION_LOG.md`
and `CLAUDE.md` **already modified and uncommitted**, 659 insertions from
the earlier session of the same day **[CARRIED — `git diff --stat`, run
by `icc-engineer`]**.

> ★ **One corroboration of that was available without a shell and was
> made:** the nineteenth filing's own *"Filed this session"* table names
> `ARCHITECTURE.md` and `SESSION_LOG.md` as written, and its closing
> notes name `CLAUDE.md` and `docs/NEXT_SESSION.md` as edited by
> `icc-engineer` — **the same four files the dispatch reports as
> modified** *(verified — read)*. **That corroborates WHICH files, not
> the insertion count and not the tree state now.**

**Filed by:** `icc-librarian` — **the real agent this time**, and
**without a shell**. **Dispatched by:** `icc-engineer`, whose dispatch
tagged every claim `[VERIFIED — I ran it this session]`,
`[QUOTED]` or `[REPORTED]`. ★★ **No `[REPORTED]` claim was promoted to
`[VERIFIED]` anywhere in this filing**, which was an explicit instruction
and is also DL-046, filed the same day.

> ★★ **The "consecutive filings without a shell" count is deliberately
> NOT stated.** §3.29.11 of the ledger said *"fifth consecutive"*; the
> **nineteenth** filing **had** a shell but was made by a general-purpose
> **stand-in**, not by this agent. *Filings* and *filings by
> `icc-librarian`* are now two different populations giving two different
> integers — **DL-031 arriving in this document's own provenance line.**
> It is left uncounted rather than counted wrongly.

### What happened: the operator changed the posture, and handed over a corpus

> *"I know some things you stopped work on because they required physical
> testing that we don't have. We aren't going to aim for compliance like
> that. Just aim for compatibility."* — the operator, 2026-08-17
> **[QUOTED]**

Said while handing over the **Ghent PDF Output Suite 5.0**.
`icc-engineer` extracted every ICC profile embedded in it, drove them
through the shipped CLI, wrote `docs/GHENT_COMPATIBILITY.md`, and filed
three requests into the channel. **This entry is the durable record of
that session's findings; the rows are in `NUMERIC_CLAIMS.md` §3.30.**

★★★ **The finding that reframes the parked work.** Certification and
capability are different things, and this project had been conflating
them: *"this cannot be certified"* was being read as *"this cannot be
checked."* The corrective is **not** a loosening of the evidence rules —
rule 3 still governs every number — it changes **what is claimed**.
Filed as **DL-045**.

### What was measured, and what class each result is

| Row | What | Class | Result |
|---|---|---|---|
| **NC-192** | extraction over the suite | `apparatus-census` | **98 PDFs → 121 embeddings → 20 distinct profiles** |
| **NC-193** | `iccce inspect` on all 20 | `acceptance` | **20/20 exit 0, `malformations: 0`** |
| **NC-194** | the red/green trap profile | `fixture-declared-categorical` | swap honoured |
| **NC-195** | the cyan/magenta trap **with a control** | `fixture-declared-categorical` | swap honoured; control does not swap |
| **NC-196** | the **unswapped** channels of that pair | `self-consistency` | paper exact to 6 dp; **yellow differs by `2.455×10⁻³`** |
| **NC-197** | `eciRGB v2` **v2.4.0 vs v4.2.0**, 2,197 points | `self-consistency` | **max abs. difference `0.000113` in device coordinates**, ★ **not a ΔE** |
| **NC-198** | X-Rite v4 CMYK → sRGB, perceptual | `acceptance` | evaluates |
| **NC-199** | 4-tag `kTRC`-only Gray → ISO Coated v2 | `acceptance` | evaluates |

★★★ **Not one of those is an accuracy claim, and the reason is dated:**
the lcms2 differential over this corpus was dispatched to
`icc-conformance` the same day and **had not reported when the dispatch
was written**. **NC-001 remains this project's only
`published-ground-truth` row.**

**Two new evidence classes were added to `NUMERIC_CLAIMS.md` §1** —
`fixture-declared-categorical` and `acceptance` — because no existing
class fits these rows without lying about them. ★ **Adding two classes
in one filing is unusual**; the fold into an existing class was tested
and rejected for a stated reason in each case, and the reasoning is in
§1's own boxed note.

### ★★ TWO CORRECTIONS THIS FILING MADE TO THE DISPATCH — both found by checking, neither reported by the sender

★ **This is the librarian's whole job and it is recorded here so the
next filing expects to do it too.** The dispatch was careful, honest, and
tagged throughout — **and two claims in it did not survive contact with
the source.**

**(1) A carried figure was corrected by arithmetic.** The dispatch wrote
that yellow and paper are *"unchanged between trap and control to 3
decimal places."* **The two triples the same dispatch carried do not
support the yellow half:** `0.929322` vs `0.931777` differ **at** the
third decimal place, by **`2.455×10⁻³`**. Paper is exact to six
decimals. ★★ **The error is in the flattering direction** — the real
difference is larger than claimed. **Filed as NC-196 with the corrected
figure**, and the reason it matters is filed with it: NC-195's value is
that it is **categorical**, and a numeric aside attached to a categorical
row is how a row acquires a bound nobody derived.
`docs/GHENT_COMPATIBILITY.md` §4.3 carries the same overstatement as
*"do not move measurably"*; **that file is `icc-engineer`'s and the
correction is owed there, not made here.**

**(2) A citation in this session's own outbound request points at the
wrong number — and the wrong number is the same integer.**
`open/request_profile_population_census.md` cites
`NUMERIC_CLAIMS.md:2164` and `:2529` as the basis of iccce's **33-node**
recommended grid. *(verified — all four locations read at the tip)*:

- **`:2164`** is Pass 6's coverage box and states the grid as **17**.
- **`:2529`** describes **`USWebCoatedSWOP.icc`'s own `lut8` CLUT, which
  has 33 nodes** — ★★ **a DIFFERENT 33**, a vendor file's tag, not
  iccce's recommendation.
- The real homes are **§3.19 / NC-145**, **§3.27**, and the code at
  **`crates/iccce-cmm/src/compiled.rs:77`**
  (`recommended_grid_points`), called at
  **`crates/iccce-cli/src/main.rs:421`** *(verified — grepped)*.

★★★ **The number collision is the finding, not the typo.** The ledger
already records two collisions (`16`, `129`) and the rule *always carry
the denominator*. **A reader at `pdfce` following `:2529` would conclude
the recommendation is a property of `USWebCoatedSWOP.icc`.** ★ **The
substance of the ask survives intact** — the recommendation genuinely
does rest on one profile pair, one direction and one tag type, which is
presumably why those two lines were reached for. **The argument was
right and the citation was wrong.** The channel file is outbound, in no
repository, and **`icc-engineer`'s to fix**; the durable homes are
recorded correctly in §3.30.7 either way.

### What was corroborated without a shell, and what was not

**Corroborated** *(verified — read, enumerated or grepped)*:
`D:\Dev\iccce-private-fixtures\ghent-v50\` holds **exactly 20 `.icc`
files plus `manifest.json`**; `tools/ghent/extract_icc.py` exists;
`docs/GHENT_COMPATIBILITY.md` exists with the nine sections claimed; the
**three** request-channel files exist; the private-fixtures `README.md`
carries the `ghent-v50/` terms subsection; `recommended_grid_points`
exists at the two locations above; **no prior `docs/` file mentioned
Ghent or GWG at all** before today.

**NOT corroborated, and carried outright:** **98** and **121**; the
SHA-256 deduplication; every transform output value; the `inspect`
results; the tag-table decode of the X-Rite profile; the `git diff
--stat`; and **every quotation from a Ghent PDF** — this librarian has
**not seen any of those documents** and read the quotations inside
`GHENT_COMPATIBILITY.md`.

★★ **A count of files is not a check of their bytes.** Twenty `.icc`
files on disk corroborates *twenty*; it does not corroborate *distinct*.

### The `[REPORTED]` leads that were NOT promoted

Six, from a dispatched agent's byte-level scan, filed as leads in
§3.30.6: the per-patch declared rendering intents (**16.1** carries
Saturation ×18 and is the suite's only Saturation; **22.1** carries two
intents in one file); the **`FOGRA27`** `OutputConditionIdentifier` on
GWG 16.1/16.7 against an embedded `ISO Coated v2 300% (ECI)`; and GWG
22.1's PDF `/Lab` `/WhitePoint [0.964203 1.0 0.824905]`.

★ **The `FOGRA27` mismatch is the one a consumer would act on**, and it
is the one to re-derive first. **It must not reach `pdfce` as fact.**

### Filed this session

| Where | What |
|---|---|
| `NUMERIC_CLAIMS.md` | **§1** — two new evidence classes with a boxed justification for adding two at once. **§3.30** — rows **NC-192 … NC-199**, the trap-profile analysis, the arithmetic correction, the citation defect, what is not claimed, and coverage. **§7.16** — the twentieth status pass, six newly-owed items, three operator decisions. ★ **No existing row edited; nothing above §3.30 rewritten.** |
| `ARCHITECTURE.md` | **§5 — DL-045** (compatibility vs certification: what is claimed changes, how well it must be supported does not), **DL-046** (verify in the running thing, and add the control the report did not have), **DL-047** (Ghent cannot supply a numeric expectation, and contradicts itself on rendering intent). **DL-001 … DL-044 untouched.** |
| `ROADMAP.md` | A dated **2026-08-17** update block — **the first 2026-08-17 material in this document**, since two earlier sessions that day deliberately made no entry — and a new **"Ghent compatibility — a standing workstream, NOT a Pass"** section carrying the operator-blocked decisions. ★ **No Pass status changed and no plan text was rewritten.** |
| `SESSION_LOG.md` | This entry. **No earlier entry rewritten or annotated.** |

**Not touched, by ownership:** `docs/GHENT_COMPATIBILITY.md` (engineer's
— **two corrections owed in it**), `docs/TOLERANCES.md`
(`icc-conformance`'s — correctly empty here: six of the eight rows have
no bound to justify), `docs/NEXT_SESSION.md`, `CLAUDE.md`, `README.md`,
`.github/`, `tools/`, `crates/`, `fixtures/`, `LEGAL.md`, the private
fixtures tree, and **every file in the request channel**.

★ **Why `ROADMAP.md` got a workstream and not a Pass:** every Pass here
has a done-when that can be met. **This has none** — the suite is a
corpus, not a specification; it supplies no numeric criterion; and the
claim that would close it cannot be made in public without GWG's
permission. **A done-when could only ever be met by weakening it.**

---

## 2026-08-17 — the **Pass G** filing (twenty-first `SESSION_LOG` entry). **72 new graded rows, the v4 LUT gap closed on a vendor's file, one row DOWNGRADED to a negative result, and two decision-log entries that are not about colour**

**Filed by `icc-librarian` from an `icc-engineer` dispatch carrying
`icc-conformance`'s completed work.** Tip **`e21154c`** *(carried — this
librarian has no shell and did not run `git`)*.

★ **Counting hazard, stated at the top because this entry would otherwise
create it (DL-031).** There are **two populations**: `SESSION_LOG`
entries and librarian *filings*. An intervening 2026-08-17 filing was
**scoped by its dispatch to `docs/NEXT_SESSION.md` alone** and therefore
correctly made **no entry here** — it is the filing that found the
stale-citation defect. **So this is the twenty-first entry and the
twenty-second filing**, and `NUMERIC_CLAIMS.md` §7.17 states both rather
than choosing.

### What the dispatch carried, and how it was tagged

`icc-engineer` tagged every claim **[VERIFIED — I ran/read it this
session]** or **[CARRIED — from an agent's report, not re-derived by
me]**, which is §5.5 of `NEXT_SESSION.md` working as designed. The
**[VERIFIED]** half: the bare-gate re-run and its exit code; the six
stale line citations re-read at the tip; `diag.rs`'s path and the two
`ParseError` types. Everything inside the run was **[CARRIED]**.

★★ **This librarian transcribed the in-run numbers from `TOLERANCES.md`
§3.7 rather than from the dispatch** *(verified — §3.7 and §4 read in
full)*. That is the only independent check available without a shell:
**it does not confirm the numbers, it confirms that two documents written
by different agents say the same numbers.**

### ★★★ The result — and the one sentence that must not be written

**`pass=229 fail=0 skip=3 error=0`, exit 0** (from `pass=157`), **72 new
graded rows** in `tools/difftest/src/passg.rs`, `discriminating` **16 →
42**. Corpus-absent: **`pass=157 skip=7`, exit 0**.

**The headline:** on X-Rite's vendor-authored **v4 `mAB `** profile the
raw iccce-vs-lcms2 disagreement **is the interpolation method and nothing
else** — substituting lcms2's own `Eval4Inputs` geometry collapses it
**179×** and **243×**, and an envelope computed **from the CLUT's own
bytes with no lcms2 output in it** accounts for the raw residual to
**0.04 %** and **0.22 %**. `TOLERANCES.md` §3.4.3's *"any **real** v4 LUT
profile"* gap, open since 2026-08-11, is **closed**.

> ★★★ **The structural gate (envelope × 1.25) explicitly CANNOT claim
> agreement, and is labelled so.** The agreement claim lives in the
> substituted-geometry row alone, at `2×10⁻²`, ≥40× tighter. **Merging
> them into one *"agrees with lcms2"* sentence would give the wide row
> the tight row's authority** — `NUMERIC_CLAIMS.md` §3.31.2 exists to
> prevent exactly that, and the dispatch asked for it in terms.

★★ **And the three PCS rows compare the HARNESS's `mAB `
reimplementation to lcms2, not iccce to lcms2.** The link to iccce is the
apparatus row at `1×10⁻⁹` (NC-200), and **injection I1 proved the linkage
empirically**: corrupting iccce's v4 PCSLAB decode turned the apparatus
rows red at **894 000 000× their bound** and left the three PCS rows
green — **correctly**.

### ★★ Two tolerances were wrong first, and both were fixed by finding a MISSING TERM

Recorded because a corrected derivation and a widened tolerance are
indistinguishable in a diff.

- **The corner tolerance** failed at `1.111 856×10⁻³` because its
  derivation said *"the 2-entry B curves are affine"* — **true and
  irrelevant.** The property that matters is the **exact identity
  `(0x0000, 0xFFFF)`**, which is true of `A2B1` and false of `A2B0`
  **in the same file**. The remedy is a tolerance that is a **function of
  the tag's own bytes**, and **injection I3** shows the run-time
  selection is load-bearing: forcing the identity branch turns exactly
  one row red.
- **Three end-to-end rows** failed at `8.98×10⁻³ … 1.49×10⁻²` because
  **§B's B2A-derived gate was reused on an A2B direction**, where the
  method envelope is the dominant term rather than identically zero.
  **A bound that omits the dominant term is not a bound, however small
  its number looks.**

### ★★★ The apparatus found a defect nobody was looking for — DL-049

**`BLIND` fired on an authoring row**, and fixing it exposed something
else: a **`2×10⁻⁴` encoding-floor justification did not hold for the
profiles it was gating** (Ghent's sRGB colorants sum to the PCS white to
≈**12 `s15Fixed16` lsb**). **The row had been passing inside a bound its
own `why` could not support.**

★ **That is a new shape.** DL-037 said separation is **disclosure, not
enforcement**; here a field that **gates nothing** caught a defect in a
**tolerance's derivation**, on a **green** row — something only a person
re-reading a `why` string could previously find. ★★ **And the
replacement imports no third white point**: reaching for **D65** would
have put the oracle's own single-sourced constant (§3.5 / NC-018)
underneath a finding about third-party authorship.

### ★★ Rule 7 ran against a third party, and no code changed

Ghent's Adobe-embedded `sRGB` and `Adobe RGB (1998)` have **D50-adapted
PCS data, an unadapted `wtpt`, and no `chad`** — an **authorship** defect
under `ICC.1:2001-04` Annex A.3.1.1, settling the ICC-absolute divergence
**in lcms2's favour**. `eciRGB v2` is the control that stops it being
read as a claim about every v2 display profile. ★ **No `NA` registered,
no code changed** — whether iccce should follow lcms2 is an engineering
call with a cost, filed as newly owed. ★ **The clause's *"should"* must
not be cited as an ISO-directives *should***: that edition has **no
defined verbal-form hierarchy**.

### ★★★ NC-197 DOWNGRADED — a negative result, and the ledger paid nothing for it

The v2/v4 `eciRGB v2` pair was dispatched to the twentieth filing as
*"an instrument this project has not had"*. **It is not an instrument.**
Both encodings put `wtpt` **at** the PCS white (`1.526×10⁻⁵` /
`5.396×10⁻⁶`), so the version gate never runs — and **no pair in the
corpus differs only in version while encoding a non-PCS white**. The two
files also differ in **TRC representation** (700-entry `curv` vs `para`
type 3), so a disagreement has **two candidate causes**: DL-033's unknown
power. Gridded properly: **`1.01×10⁻⁴`** (iccce both sides) and
**`2.29×10⁻⁴`** (lcms2 both sides — *about the files*).

★★★ **`icc-engineer`'s own earlier figure was `1.13×10⁻⁴` over 2,197
points against a different destination, and the two runs are NOT
reconciled.** Filed unreconciled: **§5.2's rule that an unexplained
*small* difference is unexplained applies between two of one agent's own
runs.**

> ★ **The downgrade cost this ledger nothing, and that is the lesson.**
> NC-197's row recorded its tolerance as *"none declared in advance — an
> exploratory comparison, not a gate"* and its class as
> `self-consistency`. **The row never claimed the thing being withdrawn;
> the prose did.** A retraction that unpicks a sentence is cheap; one
> that unpicks a row is not.

### ★★★ DL-048 — the five remaining stale citations, and a sixth of a different kind

The dispatch verified all six independently. Their real homes, now cited
by **§/NC**: `:5788` → the standing **`published-ground-truth`** row of
the **§7.x** tables (**§7.11 / §7.12 / §7.14** name the document);
`:623` → **§3.5 / NC-018**; `:976` → **§3.8.2 / NC-036**, restated at
**§3.8.9**; `:6488` → **§3.29.6** and **DL-041**.

★★ **The sixth is a bare `§4.4` with NO DOCUMENT NAMED** — and the two
§4.4 sections that exist in `docs/` (`LEGAL.md`, `GHENT_COMPATIBILITY.md`)
are **both plausible enough to be read as confirmation**
*(verified — grepped)*.

★★★ **The decay was observed happening.** `NUMERIC_CLAIMS.md` §3.30.7
recorded `:2164` as §3.13's Pass 6 coverage box *(this librarian, before
that filing's own edits)*; the corrected outbound census request calls the
same line *"unrelated (BPC material)"* *(`icc-engineer`, later the same
day — verified, file read)*. **Two readers, two moments, one line number,
two destinations, neither reading wrong.**

★ **`diag.rs:83` was right in line and content and wrong in path** — and
the reason to insist on the full path is now measured: **two distinct
`ParseError` types exist in this workspace and both implement `Error`.**

### Four §7.16 items DISCHARGED, and three by someone else

*(verified — each by reading the file at the tip)*: the **census
request's citations** and **`GHENT_COMPATIBILITY.md` §4.3's yellow
figure**, both corrected by `icc-engineer`; the **duplicated §6 table
row**, gone; and the **lcms2 differential**, which is this filing.
★★ **§7.16's pre-registered check on that last one paid off**: it asked
whether the differential ran over **the same 20 profiles**. **It ran over
11.**

### What Pass G did NOT do

**11 of 20 profiles.** **No attribution row for §B** — the harness has no
`mft2` B2A model, so §B's 17–63× margin below its gate is **not** an
agreement claim. **No `mBA ` direction** of the X-Rite profile, whose
`B2A0` has a 4096-entry tabulated B curve **nothing in this suite
evaluates**. **Eight `--bpc` combinations refused by name and therefore
differentially untested.** **Nothing rendered.** **No published ground
truth, and none possible** (DL-041/DL-047) — **NC-001 is still the only
such row.**

★★ **And a denominator hazard found while filing** *(derived here — the
only arithmetic check available)*: the two separation aggregates sum to
**160 → 232 = 160 + 72**, with the whole delta in `discriminating`
(**+26**) and `no-named-alternative` (**+46**). **But `ungraded` did not
move from 8** although `TOLERANCES.md` §3.7.3 records **12** §B rows taken
out of grading — **UNSETTLED**. And **`skip` counts RECORDS, not rows**:
corpus-absent `skip=7` is **4 records standing in for 72 rows**, while
`pass=157` reproduces the pre-Pass-G total exactly. **A `skip` count is
not an inventory of what was not tested.**

### Filed this session

| Where | What |
|---|---|
| `NUMERIC_CLAIMS.md` | **§3.30.10** — dated corrections: NC-196's sibling claim discharged, **NC-197 downgraded to a negative result** with the two runs left unreconciled. **§3.31** — Pass G, rows **NC-200 … NC-218**, the two-arms rule, the two corrected tolerances, DL-049's finding, the traps, the four injections, coverage, and what it does not claim. **§4 NA-006** — a **second dated measurement** of the n-linear cost, on a vendor v4 file, with an explicit ban on ranging it against the first. **§7.17** — the status pass, six newly-owed items, four discharges. ★ **No existing row edited.** |
| `ARCHITECTURE.md` | **§5 — DL-048** (a stale citation is worse than a stale number; cite by §/NC, full paths for source files) and **DL-049** (a disclosure field caught a defect in a tolerance's justification). **DL-001 … DL-047 untouched.** |
| `ROADMAP.md` | A dated **2026-08-17 (later the same day)** header block, and a **"Pass G landed"** subsection inside the Ghent workstream discharging its "Next" item 1 and superseding its *"no accuracy claim"* bullet. ★ **No Pass status changed.** |
| `NEXT_SESSION.md` | **§0** — the five stale citations re-cited by §/NC and the `diag.rs` path completed; **§1** — the superseded conformance figures marked as a dated observation with the new ones beside them; **§3.2** — the separation denominator; **§5.8** — discharged, with the sixth failure recorded. |
| `SESSION_LOG.md` | This entry. **No earlier entry rewritten or annotated.** |

**Not touched, by ownership:** `docs/TOLERANCES.md` and `tools/`
(`icc-conformance`'s — **finished, and read here, not written**),
`docs/GHENT_COMPATIBILITY.md` (`icc-engineer`'s — **read and
cross-referenced, deliberately not edited**), `CLAUDE.md`, `README.md`,
`LEGAL.md`, `.github/`, `crates/`, `fixtures/`, the private-fixtures
tree, and **every file in the request channel**.

★ **The one thing this entry cannot tell you:** whether any of it runs on
a machine that is not this one. **Twenty-one entries, and no CI run has
been observed by anyone here** — and Pass G makes that worse rather than
better, because its rows skip without the private fixtures and the
skipping run reports a pass count identical to the old total.

---

## 2026-08-17 — the **constructed-destination + `/N` accessor** filing (twenty-second `SESSION_LOG` entry). **A CORPUS CLAIM FALSIFIED by the test written to honour it, a suite measured to have ZERO POWER against the constant it documented most, `sRGB2014.icc` exposed as NOT a second source, and two capabilities that shipped with no ROADMAP entry**

**Filed by `icc-librarian` from an `icc-engineer` dispatch.** ★
**Twenty-second entry here; twenty-third librarian filing** — §7.17
established that the two populations differ by one (an intervening
2026-08-17 filing was scoped to `NEXT_SESSION.md` alone and correctly made
no entry here), and **neither integer may be quoted without its
population** (DL-031).

### ★★★ Three things that bound everything in this entry — read them before any number

1. **Nothing from this session is committed and nothing is pushed.
   Authorisation has not been given** *(carried from the dispatch; this
   librarian has no shell and ran no `git`)*. **Every row filed today has
   NO COMMIT ANCHOR.**
2. **★★★ The conformance runner was NOT run.** `icc-conformance` holds
   `tools/difftest` and `docs/TOLERANCES.md` for a **concurrent Pass H**,
   and the dispatch says so in terms: *"Do not state a `pass=` line on my
   authority."* **`pass=229` stands as NC-218's dated observation at
   `e21154c`.** Both files are **untouched by this filing.**
3. **★★ The usual independent check was unavailable.** At the Pass G
   filing this librarian transcribed in-run numbers from `TOLERANCES.md`
   rather than from the dispatch. **None of today's rows are
   conformance-suite rows**, so that route did not exist — **the checks
   that were available were reads of `crates/`**, and two of them found
   things the dispatch had not carried.

### ★★★ What this librarian found by reading source rather than the dispatch

**Both are recorded because §5.5 says the dispatch IS the source for a
shell-less agent — and it works in this direction too.**

- **★★★ NC-227's number is measured but NOT ASSERTED, and the test cannot
  fail.** `single_byte_corruptions_of_cmyk_do_not_silently_become_three_channel`
  enumerates all 1 020 corruptions correctly, then sends the survivor
  count to a **`println!`** — invisible without `--nocapture` — and its
  in-loop assertion compares `components(sig).count()` **with its own
  result**, which is true by construction. ★★ **This is the same session's
  own DL-051 recurring, hours later, in the module that documents the
  hazard at greatest length.** The *behaviour* is protected; the
  *enumeration* is not.
- **★★ NC-221's margin was rounded up.** The dispatch reported *"37 % in
  hand, not 5 %"*. At the **binding** probe (pure white) the observed
  quantity and the derived bound are **the same number**, because the
  construction maps device white to exactly D50 and the file's white is
  the colorant sum — so the assertion reduces to `x ≤ 1.05x` and the
  margin there is **exactly 5 %.** The 37 % is measured on probes that do
  not bind. ★ **The row is not weakened** — the coincidence holds only at
  the correct answer, making it a *tighter* white-point gate than any flat
  constant — **but the two margins are not interchangeable.**

★ **A third correction is one of wording, and it is DL-048's family.** The
dispatch asked for *"NC-213 marked SETTLED"*. **NC-213 is a measured row
with a stated result and was never pending** — what was open is **§7.17
newly-owed 4**, the engineering decision NC-213 *raised*. Marking a
measured row "settled" would tell a future reader its measurement had
been in doubt. **The row is untouched; the owed item is discharged.**

### ★★★ The headline: a corpus claim was FALSIFIED, and the correction is worse news than the error

`ICC_Spec/iec/iec__s__srgb.md` recorded that using `0.03928` instead of
`0.04045` *"affects only encoded values in `[0.03928, 0.04045]` — 8-bit
codes **10 and 11**, and nothing else"*. It was carried into a doc
comment, **then tested.**

**No 8-bit code lands in the window at all.** `10/255 = 0.039216` is
below the lower breakpoint; `11/255 = 0.043137` is above the upper one.
**The separation at 8-bit input precision is exactly zero**, and the
maximum anywhere in the `1.17×10⁻³`-wide window is **`7.55×10⁻⁷`** in
linear light.

> ★★★ **Why the corrected statement is worse.** *"Two codes are
> affected"* describes a small, findable defect. The truth is that a
> wrong breakpoint is **invisible to every image, every 8-bit test
> vector, every round trip** (the same wrong constant inverts itself)
> **and every differential against an implementation that made the same
> choice.** It surfaces only against a correctly built reference
> evaluated at non-8-bit precision. **A defect invisible to the entire
> apparatus is not a smaller defect than one visible in two codes.**

★★★ **OWED and unfixed: the corpus still carries the wrong version.**
Flagged to `icc-spec-librarian` (§7.18 newly-owed 1). **This is DL-048
with the polarity reversed** — DL-048 is a pointer that survives after its
target moves; this is a target that survives after the claim inside it was
refuted. **Both leave a reader arriving somewhere plausible and treating
arrival as confirmation.**

### ★★★ The suite had ZERO power against the constant it documented most — DL-051

`builtin.rs` shipped a long, well-sourced doc comment on why `0.04045`
and not `0.03928`, plus five tests. The constant was substituted and the
suite re-run.

| injection | caught by |
|---|---|
| **A** Bradford adaptation omitted | 3 of 6 FAILED |
| **B** adaptation applied twice | 3 of 6 FAILED |
| **C** TRC → pure gamma 2.2 | 1 FAILED |
| **★★★ D** breakpoint `0.04045 → 0.03928` | **NOTHING. 6 of 6 PASSED** |
| **E** green primary `0.600 → 0.610` | 2 FAILED |

★★★ **The length of the documentation is what made the gap invisible —
nobody audits a constant that is visibly well-explained.** ★★ **It is a
sibling of §5.3 and not the same thing:** in §5.3's instances the tests
could not fail *at all*; here **every test could fail, just not for this
defect.** A suite's power is **per-defect, not per-suite.**

★★★ **And it was found by INJECTION, not inspection**, by the person who
had written the constant, the comment and the tests in one sitting.
**Reading your own work is not an instrument.** Two tests were then
written for D and **the injection now goes red while its siblings stay
green.**

### ★★ `sRGB2014.icc` is NOT the second source it looks like

ICC's 2015 file has the ICC.1 A.3.1.1-compliant `wtpt` (D50) and the
`chad` the HP 1998 file lacks — **exactly the two properties NC-213 found
missing in the Ghent-embedded copies** — so it is the obvious independent
specimen for the colorants §3.8 records from **one** file.

**Its `rXYZ`, `gXYZ`, `bXYZ` and all three TRC tables are
BYTE-IDENTICAL** to the HP file's. Only the header, `wtpt`, `bkpt` and
`chad` differ.

> ★★★ **A re-headered file is not a second measurement.** The 2015 file
> corrects the *authoring* defect and changes **not one of the nine
> numbers**. **There is still exactly one lineage, and a future session
> must not think the gap is closed.**

★★ **And a better route was tried and also failed — pre-registered as
better before it was run** (DL-023's discipline). Applying the file's
**own disclosed `chad`** to the BT.709 D65 matrix improves the residual
from **12.0 ULP to 5.35 ULP and no further**, still in `bXYZ.Z`. The
`chad` inverts to `xy = 0.312702 / 0.329020` — BT.709 D65 to quantisation
— so it **is** a D65→D50 adaptation, just not linear Bradford to the ULP.
**The corpus's negative conclusion is strengthened, not overturned**, and
it now rests on two failed routes instead of one.

### ★★ What was built, and the API decision that carries the most weight

- **The constructed sRGB destination** — BT.709-6 primaries and white,
  W3C CSS Color 4 transfer function, ICC.1:2022 Annex E.3 Bradford. **No
  I/O, no blob, no dependency, and no lcms2 in the lineage** — the
  oracle-contamination blocker that held this item for eleven filings.
- **★★★ DL-050: a two-variant enum, NOT `Option<&Profile>`.** *An
  `Option` being `None` cannot distinguish "there was none" from "I
  failed to get one", and only the second must never trigger the
  fallback.* A declared-but-unparseable destination stays a **named
  refusal**; the substitution, when it happens, is **disclosed** through
  `DestinationProvenance`. `Chain::new` unchanged, no caller moved.
- **The `/N` accessor** — `components()`, `channel_agreement()`,
  `is_valid_pcs()`, sourced from **ICC.1:2022 7.2.6 Table 19** through
  **four independent routes with no disagreement**. **A48: ICC.1 is
  SILENT on header/tag channel agreement**, so iccce **discloses** rather
  than declaring non-conformance — *"silent" is a different claim from
  "requires agreement"*. **A50: the count is a two-table join; ICC.1
  publishes no `Signature → count` map**, so cite it as derived.
- **★ `ChainError` implements `std::error::Error`** — found by a compiled
  **doc example** refusing to build with `E0277` (**DL-052**). *A refusal
  that is awkward to propagate is a refusal that gets discarded*, and
  rule 6's whole value is that refusals reach the consumer.

### ★★ Two owed items discharged, one carried figure corrected

- **§7.17 newly-owed 4 — DISCHARGED by decision.** iccce does **not**
  substitute D50 for a mis-authored `wtpt`; it uses `wtpt` as stored and
  **discloses** (A4c / NA-007). ★★ **Verified in the running thing**
  (`iccce inspect` prints the note) — **DL-046 applied to our own
  behaviour**: the code existed and **the decision was what was missing.**
  ★ **6 of 60 profiles disclose; all six hand-audited are TRUE positives,
  zero false positives found.** `D50_XYZ.icc` looks like a false positive
  and is not — its colorants are an XYZ identity summing to **illuminant
  E** while `wtpt` says D50 — and `D65_XYZ.icc` correctly does **not**
  fire, because its `chad` explains the difference. **Precision is what
  makes a disclosure worth reading.**
- **★★★ DL-053 — a count from a SAMPLE recorded as a count of the
  POPULATION.** `NEXT_SESSION.md` said **two** downloads correctly failed
  as iccMAX. **Two were TESTED; TEN are PRESENT.** Every word of the
  original was true of what was run and it carried `[VERIFIED by me]` —
  **what was missing is "…of the two I tested."** ★★ **A `[VERIFIED]` tag
  certifies that the measurement happened and certifies nothing about
  what it ranged over**, and this landed in the resume-from-cold handoff,
  the document every session reads first. ★ **Re-deriving the denominator
  upgraded the claim rather than merely correcting it:** 50 files, **40
  parse with `malformations: 0`, 10 refused by name** with the iccMAX
  signature and version in the message — **rule 6 demonstrated at
  population scale on real ICC-published files.**

### ★★ The corpus terms were written BEFORE the files landed — the rule working

`D:\Dev\iccce-private-fixtures\color-org\`'s `README.md` terms subsection
was written **before** the 50 files were moved in, discharging that
folder's own rule 3. **The lapse recorded against `ghent-v50/` earlier the
same day was not repeated.** ★ Recorded because **a rule that works leaves
no trace unless someone writes one**, and §7's tables train readers to
look only at what failed.

★★★ **The terms finding is not trivial: 23 distinct copyright strings
across 46 files, in SIX different licensing postures** — bare assertion
with no grant; IDEAlliance's *"included in commercial software"*; APTEC's
*"…and sold without restriction"*; ECI's self-contradicting form; the
literal string `none`; and absent. **The restrictive reading applies to
the whole folder.** ★★ **Ten files carry a grant that would survive
redistribution — recorded as a FACT that NO AGENT MAY ACT ON.** It is an
**operator decision**, the standing question now spans **five** private
corpora, and **a "yes" on this one would have to be file-by-file rather
than folder-wide.**

### Measured state, with every runner named (DL-031)

| runner | result |
|---|---|
| `cargo test --workspace` (repo root) | **154 passed, 0 failed, exit 0** (was **132** at `0bd76ad`) |
| `cargo clippy --workspace --all-targets` | exit 0 |
| `cargo fmt --all --check` | exit 0 |
| `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` | exit 0 |
| `cargo build --target wasm32-unknown-unknown` (4 library crates) | exit 0 |
| **`tools/difftest` conformance runner** | ★★★ **NOT RUN.** Held by `icc-conformance` for Pass H |

*(All **[VERIFIED — `icc-engineer` ran each bare, redirected, read `$?`;
no pipe into `grep`/`tail`]** — §5.6. This librarian ran nothing.)*

★★ **`cargo fmt --all --check` is the WORKSPACE**, and `tools/difftest` is
a **separate workspace** — the two are not the same runner, and §7.10
item 5 is still `unverified-this-filing`.

### Filed this session

| Where | What |
|---|---|
| `NUMERIC_CLAIMS.md` | **§3.32** — rows **NC-219 … NC-229**; a **third evidence class** (`constructed-vs-reference-file`) with the table of labels it is *not*; the falsified corpus claim; the injection matrix; the `sRGB2014` non-source; the A4c precision audit; the derived tolerance and **§3.32.9a's margin correction**; the vacuous-assertion finding; the terms finding; the ICC.1 silences; the `Destination` decision; and coverage. **§7.18** — the status pass, six newly-owed items, one discharge. ★ **No existing row edited; NC-213 untouched.** |
| `ARCHITECTURE.md` | **§5 — DL-050** (the `Destination` enum, not `Option`), **DL-051** (a well-documented constant is not a tested constant; found by injection, not inspection), **DL-052** (the doc example as the cheapest consumer), **DL-053** (a count from a sample recorded as a count of the population). **DL-001 … DL-049 untouched.** |
| `ROADMAP.md` | A dated **2026-08-17 (latest)** header block, and a **Pass 8 RETROSPECTIVE** subsection giving the built-in destination and the `/N` accessor the completion records they shipped without. ★ **No Pass status changed.** |
| `NEXT_SESSION.md` | The queue rewritten: items **1, 2, 3, 4** discharged and **6** partially; the **"two iccMAX" figure corrected to ten**; §0's gap list halved. |
| `SESSION_LOG.md` | This entry. **No earlier entry rewritten or annotated.** |

**Not touched, by ownership:** `docs/TOLERANCES.md` and `tools/`
(`icc-conformance`'s — **held for a concurrent Pass H, and NOT read as
finished**), `docs/GHENT_COMPATIBILITY.md`, `docs/DEFAULT_DESTINATION.md`
(`icc-engineer`'s — **read and cross-referenced, deliberately not
edited**), `CLAUDE.md`, `README.md`, `LEGAL.md`, `.github/`, `crates/`,
`fixtures/`, `ICC_Spec/`, the private-fixtures tree, and **every file in
the request channel**.

★★★ **The one thing this entry cannot tell you:** whether any of it
survives a commit. **Nothing is committed, nothing is pushed, and every
number above is against a working tree that exists on one machine.**
★★ **And CI covers none of §3.32** — NC-221/NC-222 **SKIP** without a
resolvable sRGB profile, and NC-219/NC-220/NC-223 are **CLI sweeps over
private corpora CI will never hold.** **Twenty-two entries, and no CI run
has been observed by anyone here.**

### Left for the next session to not assume

- **That anything here is an accuracy claim.** ★★★ **Nothing is.** Until
  `icc-conformance`'s differential lands, §3.30 is acceptance,
  categorical and self-consistency. **And when it lands, check it ran
  over the same 20 profiles** — a differential over a different member
  set is a different claim.
- **That `0.000113` is a ΔE.** ★★ **It is not.** It is in **destination
  device coordinates** for **one destination profile**, and both arms are
  iccce.
- **That NC-194/NC-195 say iccce is accurate.** ★★★ **A CMM wrong by
  20 ΔE2000 passes them**, provided it swaps the two channels. They
  detect a **structural** failure and only that one.
- **That the parser is correct on this corpus.** `malformations: 0` on
  well-formed files is the **expected** result.
- **That 20 profiles is coverage.** It is a distinct-profile count after
  deduplication — **not** patches exercised, not features tested.
- **That the six byte-scan leads are established.** ★ **None was
  re-derived.**
- **That "Ghent" may be written in public.** ★★★ **It may not** — not in
  `README.md`, not in release notes, not in crates.io metadata — without
  GWG's **written permission**, which is an **operator** act.
- **That a "blocked-on-operator section" existed to file these in.** ★
  **It did not** *(verified — grepped)*. The dispatch asked for one by
  name; one was created in `ROADMAP.md` and mirrored in
  `NUMERIC_CLAIMS.md` §7.16, and **the absence is recorded rather than
  papered over.**
- **That the tree is clean, or that the tip is pushed.** ★ **It was
  already dirty in four files before this filing, which now touches
  three of them plus one more.** Nothing here went near a remote.
  **Rule 9 and DL-024 unchanged.**
- **That a CI run has ever been observed here.** ★ **Twenty filings,
  none.**
- **That the dispatch was right because it was careful.** ★★ **Two of
  its claims did not survive checking** — one arithmetic, one a
  citation — **and both were caught by reading the source rather than
  the dispatch.** That is the standing procedure, not this filing's
  achievement.

---

## 2026-08-17 — the **supplementary** filing (twenty-third `SESSION_LOG` entry). **ICC's own sRGB document arrives and REVERSES an attribution made hours earlier; a shipped crash that took the process down; a fix that could have deleted its own guard; and a gate measured to be blind in one direction**

**Filed by `icc-librarian` from a second `icc-engineer` dispatch of the
same day.** ★ **Twenty-third entry here; twenty-fourth librarian filing**
— the two populations still differ by one, and **neither integer may be
quoted without its population** (DL-031, §7.17).

### ★★★ Four things that bound everything below — read them before any number

1. **Nothing from this session is committed and nothing is pushed.
   Authorisation has not been given** *(carried; this librarian has no
   shell and ran no `git`)*. **§3.32 and §3.33 are now TWO anchorless
   sections awaiting one commit.**
2. **★★ The scope INVERTED since the last entry.** Last filing,
   `TOLERANCES.md` and `tools/` were held for a concurrent Pass H and the
   usual cross-check was unavailable. **Pass H has now filed**, so
   NC-235 and the `pass=` half of NC-242 were **read in
   `icc-conformance`'s own file** rather than taken on the dispatch's
   word. ★ **`icc-spec-librarian` is now the concurrent agent**, working
   in `ICC_Spec/`, and two newly-owed items land there.
3. **★★★ `pass=274 fail=0 skip=9 error=0` is `icc-conformance`'s
   measurement, reaching this entry through TWO hands** — corroborated in
   `TOLERANCES.md` §3.8.1. **It may NOT be compared with `pass=229`:**
   Pass H added rows, so the member set changed and the two integers
   describe different suites (DL-031 in the position where it is easiest
   to break).
4. **★★ The PDF at the centre of this entry could not be read by this
   librarian** — the Read tool refused it (*"pdftoppm is not
   installed"*). **Everything about the document's content is a single
   transcription by a single agent**, and that is stated as a limit on
   the strongest row filed today, not as a footnote.

### ★★★ The headline: an attribution filed this morning is INVERTED, and the mechanism is worth more than the number

The operator downloaded, **in a browser** (`color.org`'s robot bar is
intact; no agent fetched it), **"How to interpret the sRGB color space
(specified in IEC 61966-2-1) for ICC profiles"** — Jack Holm,
International Color Consortium, **2015-04-27**, 4 pages. **§B.2 publishes
the D50-adapted sRGB colorants at 15 decimal places, and ICC's recommended
D65→D50 `chad` at 15 dp beside them.**

**Two gaps this project had recorded as permanent close at once**, and one
attribution inverts:

| | worst cell | `bXYZ.Z` |
|---|---|---|
| **iccce's from-constants construction** | **3.02 ULP** | **0.90 ULP** |
| the shipped HP 1998 / `sRGB2014.icc` file | **11.13 ULP** | **11.13 ULP** |

★★★ **The ~12 ULP blue-`Z` residual is the FILE's error, not iccce's.**
The most widely deployed sRGB profile in the world disagrees with ICC's
own published values by 11 ULP, and **our construction is ~3.7× closer to
them than the file is.** Every earlier route "failed to close" the
residual because it was measuring against an artifact that does not match
the specification either.

★★ **The remaining 3.02 ULP is fully explained and is KEPT.** ICC's
published `chad` × the inverse of ICC's own §A.7 XYZ(D65)→RGB matrix
reproduces ICC's published colorants to **`0.00 ULP`** — so the entire
difference is *which D65 matrix each side starts from*. ICC inverts theirs
**as printed to 7 decimals**; iccce builds it **exactly** from BT.709-6
chromaticities. **iccce's route carries no rounded intermediate.**

★ **And a caution nobody asked for:** ICC's published `chad` applied to
D65 gives `(0.964150918938, 0.999997711611, 0.824943819994)` against a
stated `0.9642 / 1 / 0.8249` — **off by `≈4.9×10⁻⁵`**. ICC's recommended
matrix is itself slightly inconsistent with the illuminant it is meant to
reach, which is why *published* is not the same claim as *exact*.

### ★★★ DL-054 — an ACCESS boundary had been recorded as an EXISTENCE fact, and it survived because THE NUMBER WAS CORRECT

**This librarian went looking in the standards corpus and found the
mechanism in a sharper form than the dispatch carried.**
`ICC_Spec/iec/iec__s__srgb.md` held **both registers, in one file**:

- **Status table (lines 58, 582, 705):** *"**NO document states them.**
  Only a real file does."*
- **Acquisition list (lines 655-663), 100 lines later:** *"the one thing
  **no document found so far** states"* — and, on the same item, *"**All
  three are `color.org` and are therefore AGENT-BARRED** … **NOT FETCHED.
  This is a reported tool/permission limit, not an untaken action.** A
  human in a browser is outside the robot clause."*

★★★ **The acquisition list was exactly right and the status table
collapsed existence, availability and access into one flat sentence. The
status table is what got quoted** — into `builtin.rs`'s doc comment, into
`DEFAULT_DESTINATION.md`, into `NUMERIC_CLAIMS.md` §3.8 and §3.32.5.
**Summaries propagate; that is what summaries are for.**

★★ **And the worse half.** At line 664 the same file records, dated
2026-08-17 — *hours before the fetch* — **"★ EXPECTATION LOWERED"**: since
`sRGB2014.icc` turned out to carry HP's 1998 colorants byte-for-byte, the
registry page was judged **"no longer likely to close the colorant gap on
its own."** **It closed the gap outright and reversed an attribution.**
You cannot estimate the contents of an unread document from the files it
describes, and the effect of trying was to lower the priority of the one
action that would have settled it.

> **The rule: write the SEARCH claim, never the EXISTENCE claim.** *"No
> document found so far states X, and here is what was looked at and what
> is barred"* is falsifiable and invites the next fetch. *"No document
> states X"* is a claim about the literature nobody here can make — and it
> promotes the best available artifact to the reference **by
> elimination**.

★ **This is DL-041's taxonomy failing in practice for the first time.**
DL-041 kept *existence*, *availability* and *access terms* apart; the LUT
path's absence is **structural**, this one was merely **unfetched**, and
the ledger's language did not distinguish them.

### ★★ The ruling that was asked for: `published-ground-truth` is ACCEPTED, with four conditions

The dispatch asked explicitly whether the label overstates it. **It does
not.** NC-230 is the ledger's **second `published-ground-truth` row** —
the sentence *"NC-001 is the only one"*, carried by eleven filings,
**retires today**. The four conditions are part of the claim, not caveats
beside it: **(a)** it grades **nine numbers, not a transform**; **(b)** one
transcription, one reader, **a second reading is owed** — and the same
document contains **two verified transcription defects in §B.1**
(`BL = B/12.02` for `12.92`; all three power-branch equations written with
`R`), which do not touch §B.2's table but do lower the prior that printed
equals intended; **(c)** it does **not** discharge the Annex D.6.3 item or
touch **DL-041**; **(d)** *published* is a claim about **provenance**, not
about physical exactness.

★★ **The `published-ground-truth` label now exists in THREE populations
with three owners** — this ledger (NC-001, NC-230), `tools/difftest`'s
Pass H rows from ICC's `Probe2 Profile Readme`, and the corpus's own
tiers. **A count from one is not a count of another.**

### ★★★ Two ICC documents, one day, opposite rulings — and it is not a contradiction

Pass H found that **ICC's `Probe2 Profile Readme` states in numbers what
`Probev2_ICCv4.icc` does, and the published claim is FALSE of the file the
document names**; those rows went to **REPORTED, tolerance infinity**.
Today's sRGB ruling went the other way — the document outranks the file.

> ★★★ **The principle is the same in both. A document stating the intended
> VALUES is a definition and outranks any file; a document stating what a
> particular FILE does is an empirical claim about an artifact and can be
> falsified by that artifact. Ask what kind of claim the document makes
> before deciding what it outranks.**

### ★★★ A crash in shipped code — and a fix that could have deleted its own guard

`iccce bench` **aborted the process** on ICC's published seven-channel
`APTEC_CMYKOGV_Coated_LinearCTV_2025.icc`: bare `0xC0000409`, *"memory
allocation of 1022842631448 bytes failed"*, stdout empty. Two independent
causes — a **`_ => 33` catch-all** whose doc reasoned about 3-D and 4-D
and applied that conclusion to all higher dimensions (`33⁷ × 3 × 8` ≈
**952.6 GiB**), and a guard using **`checked_pow`, which catches WRAP and
not SIZE**.

★★ **An abort is the worst available failure for a library** — not an
`Err`, not a catchable panic. It takes the consumer's process down, and
`pdfce` (a named consumer, DL-044) has no defence. **Converting it to a
named refusal is rule 6 at the allocation layer.**

★★★ **And the lesson `icc-conformance` found, which the engineer did
not: each half of the fix ALONE makes the conformance row observe zero.**
The computed recommendation puts the default at grid 6, so the allocation
succeeds whether or not the guard exists — **deleting
`MAX_COMPILED_GRID_BYTES` would have left the row GREEN, with no number
moved and no edit to blame.** They split it into **four rows, one layer
each**, and the question that catches it is *"which layer is in the
loop?"* rather than *"what does this row measure?"* (**DL-055** — the
mirror of DL-018, and worse, because **a change ledger cannot record
it: there is nothing to record**).

★ **A deliberate tension, recorded as decided:** the measured 4-channel
`33` does **not** fit the byte budget at the worst output width **and was
not shrunk**. A measured value is not weakened to satisfy a memory bound;
the guard uses the *actual* output width. **A test fails if the tension
ever disappears**, because the failure mode of a documented exception is
silent removal with the explaining paragraph surviving.

### ★★★ The gate that is blind in one direction — and it corrected THIS librarian's own reasoning

`icc-engineer` wrote that the derived ΔE bound was a tight white-point
gate, **then injected drift rather than leaving it plausible**:

| injected drift in the constructed white's `Z` | max ΔE2000 | the ΔE test |
|---|---|---|
| `−1.0×10⁻³` | `0.101968` | FAILS ✔ |
| `−3.0×10⁻⁴` | `0.050149` | FAILS ✔ |
| **`+3.0×10⁻⁴`** | **`0.029008`** | **PASSES — and looks BETTER than the correct build's `0.033013`** |
| `+2.0×10⁻³` | `0.146450` | FAILS ✔ |

**A defect in one direction makes the test greener**, because the
reference file's own white sits `+1.885×10⁻⁴` above D50 and drifting
upward moves us *toward* it; up to ≈`+3.8×10⁻⁴` the test would report
**zero**. ★★★ **Not fixable by tightening: a difference cannot detect a
defect that shrinks it.**

★★★ **And §3.32.9a — this librarian's own correction, filed hours
earlier — was half wrong.** It argued the observed-≡-derived coincidence
made the row *"a tighter white-point gate than any flat constant."* **True
downward, false upward.** **A careful argument produced by re-reading was
corrected by an injection**, which is §5.2 applied to the agent whose job
is reading. **§3.32.9a is not edited; §3.33.8 is the correction.**

**What saves the suite is a division of labour that must not be
"simplified":** `constructed_colorant_sum_is_d50` compares against **D50
itself, `1e-9`, with no file anywhere in it**, and the same `+3.0×10⁻⁴`
**fails it while all six differential tests pass**. ★★ **Deleting it as
redundant would open the blind spot and every remaining test would stay
green while it happened** (**DL-056** — DL-055's mechanism in the
differential-test register; the shared sentence is *a redundancy is only a
redundancy if each member has a defect it alone can catch*).

### ★★ A refusal that named the wrong clause — rule 1 in the error surface

`Chain::with_destination(src, Destination::None, ..)` built a scaffold
chain `src → src` and discarded the destination half. It fails for a
profile with `A2B` and **no `B2A`, no colorant matrix, no `grayTRC`** — a
conformant shape, and **four such profiles are in ICC's own published
set** (the colour-vision-deficiency simulation profiles, `scnr`, Lab PCS,
one-directional by design), found by scanning both private corpora for the
shape.

The refusal read *"matrix/TRC model requires PCSXYZ (Annex F.3,
normative); profile PCS is 'Lab '"* — **true, correctly clause-cited, and
about a model iccce was about to throw away.** A caller reads it as *"my
source profile is unusable"*, which is false.

> ★★★ **A refusal that names the WRONG clause is worse than a vague one,
> because the citation makes it persuasive.** A vague refusal invites
> investigation; a precise one invites acceptance. **This is the project's
> founding hazard arriving in the ERROR SURFACE rather than in a colour
> value.**

★★ **DL-057's general form:** *a code path that reuses machinery **and
discards part of the result** inherits every failure mode of the part it
discards — the discarded half cannot fail harmlessly, because its error is
what the caller sees.* Fixed by extracting `derive_source_model()`, shared
with `new_inner` so the 8.10.2 dispatch has one copy. ★ **The second
regression test is load-bearing**: without *"these same profiles are still
correctly refused as DESTINATIONS"*, an over-broad future fix would turn
the first test green for the wrong reason.

### ★ Two owed items discharged, one blocker moved, and a test point whose REASON was wrong

- **§7.18 newly-owed 2 DISCHARGED and PROVEN.** The 1 020-corruption test
  now asserts the **survivor set** (`vec![("CMY ", 3)]`, sourced from
  Table 19's structure) with a premise check that the enumeration covered
  1 020 cases — and the lcms2-behaviour injection (`components()`
  returning `Known(3)` for unknowns) now **fails 4 of 6 tests in the
  module**. ★★ *"The assertion now exists"* is a claim about source;
  *"the defect now goes red"* is a measurement, **and only the second
  closes a §5.3 item.**
- **§7.18 newly-owed 1 DISCHARGED, verified by this librarian in the
  corpus.** `iec__s__srgb.md` carries a **second retraction, corpus defect
  `C8`** — *"zero separation at 8-bit, all 256 codes; max `7.5548×10⁻⁷` at
  `V = 0.039 302 447`"* — **and tells future readers which wrong strings
  to look for.** ★ **One filing, not eleven.**
- **★★★ The ground-truth row for chromatic adaptation — owed since Pass
  1's §7 item 4, eight filings — moves from BLOCKED to
  AVAILABLE-AND-UNMEASURED.** Its blocker was *"ICC's `chad` values, not
  obtained"*; they are in the same §B.2. **Named instrument:** iccce's
  Bradford-derived D65→D50 matrix against ICC's published `chad`, cell by
  cell. ★ **Not a freebie** — NA-002 records Bradford as *policy*, and
  NC-233 shows the published `chad` misses ICC's own D50, **so the bound
  must be derived before the row is run.** This is DL-042's rule working:
  *re-audit the REASON an item is owed.*
- **★★ A test point that was RIGHT with a stated reason that was WRONG.**
  `V = 0.0393` was documented as sitting at *"the maximum-separation
  **end** of the window"*. **The maximum is interior** (`V ≈ 0.039302447`);
  at the window's own edge `0.03928` the separation is **exactly zero**,
  because both constants take the linear branch. ★★★ **A reader "tidying"
  the number to the boundary would have produced a zero-power test that
  still passes.** Corrected with an explicit *do not move this number*.
  **DL-049's family — no new entry.**

### ★★ What this librarian found by reading, not carried by the dispatch

1. **★★★ `builtin.rs`'s own doc comment still contains the falsified
   claim it was rewritten to correct.** Seventy lines below the rewrite
   that names the document, the *"trap worth naming"* subsection still
   ends *"…and no document publishes them at all."* ★ **Its first half is
   still true** (one lineage among the files) — **only the trailing clause
   is false, which is why it survived the rewrite: the sentence reads as
   correct until its last six words.** Owed to `icc-engineer`.
2. **★★ `DEFAULT_DESTINATION.md` has the same shape at a larger scale.**
   Its `STATUS: BUILT` block says *"everything below is still the
   reference and still correct"*; its `SUPERSEDED SAME DAY` block scopes
   the supersession to *"items 3 and 4 above"* — and **§4.2 below still
   carries the falsified claim and the old rule-4 instruction.** Owed to
   `icc-engineer`; not edited here.
3. **★★★ `ROADMAP.md` judged a done-when MET against a DOC COMMENT while
   the register was EMPTY.** *"The blue-`Z` difference named as a rule-4
   approximation"* was marked **MET**, and **§4 of the ledger — the
   register of named approximations — carried no entry for the constructed
   sRGB at all.** ★★ **A doc comment explains an approximation; the
   register is what makes it findable.** Repaired: **NA-011** registered,
   with the measured 3.02 ULP against ICC's published values, its cause,
   and what it is *not*.
4. **★ The corpus's two registers** (the finding above, DL-054's evidence)
   — and **`ICC_Spec` still carries the falsified *"NO document states
   them"* at three places.** Owed to `icc-spec-librarian`.

### Measured state, with every runner named (DL-031)

| runner | result |
|---|---|
| `cargo test --workspace` (repo root) | **158 passed, 0 failed, exit 0** (was **154**, and **132** at `0bd76ad`) |
| `cargo clippy --workspace --all-targets` | exit 0 |
| `cargo fmt --all --check` | exit 0 |
| `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps` | exit 0 |
| `cargo build --target wasm32-unknown-unknown` (4 library crates) | exit 0 |
| **`tools/difftest` conformance runner** | ★★ **`pass=274 fail=0 skip=9 error=0`, bare exit 0** — **`icc-conformance`'s measurement**, `[CARRIED]` through two hands, **corroborated by this librarian in `TOLERANCES.md` §3.8.1** |

*(The first five are **[VERIFIED — `icc-engineer` ran each bare,
redirected, read `$?`]**. This librarian ran nothing.)*

★★★ **`274` vs `229` is NOT a trend and `skip=9` vs `skip=3` is NOT a
regression** — Pass H added rows, including the four-way split above, so
the member set changed between the two runs.

### Filed this session

| Where | What |
|---|---|
| `NUMERIC_CLAIMS.md` | **§3.33** — rows **NC-230 … NC-242**; the evidence-class **ruling** with its four conditions; the attribution reversal and exactly what it does *not* change; the corpus's two registers; the crash and the two-layer masking; the wrong-clause refusal; the blindness measurement; the discharge of §3.32.8; the three stale citations. **§1** — the `constructed-vs-reference-file` class finally listed (§7.18 newly-owed 6). **§3.19** — a dated citation correction. **§4 — NA-011 registered.** **§7.19** — the status pass, seven newly-owed items, three discharges. ★ **No existing row edited; §3.8, §3.32.5, NC-225 and NC-226 all stand as written.** |
| `ARCHITECTURE.md` | **§5 — DL-054** (an access boundary recorded as an existence fact), **DL-055** (each layer alone makes the gate observe zero), **DL-056** (a differential test is blind toward its reference), **DL-057** (a discarded half's error is what the caller sees). **DL-001 … DL-053 untouched.** |
| `ROADMAP.md` | A dated **2026-08-17 (latest)** header block; a **correction in place** to the Pass 8 retrospective's rule-4 row (wrong in both halves, left standing); a dated citation correction in the Pass 6 re-grade block. ★ **No Pass status changed.** |
| `SESSION_LOG.md` | This entry. **No earlier entry rewritten or annotated.** |

**Not touched, by ownership:** `docs/TOLERANCES.md` and `tools/`
(`icc-conformance`'s — **read and cited this time, deliberately not
edited**), `docs/GHENT_COMPATIBILITY.md`, `docs/DEFAULT_DESTINATION.md`
(`icc-engineer`'s — **read, and two defects in it recorded as owed**),
`CLAUDE.md`, `README.md`, `LEGAL.md`, `.github/`, `crates/`, `fixtures/`,
`ICC_Spec/` (`icc-spec-librarian`'s — **read, one discharge and one new
debt recorded**), the private-fixtures tree, and **every file in the
request channel**.

### Left for the next session to not assume

- **That the attribution reversal makes iccce's sRGB "correct".** ★★★ **It
  does not.** It makes it **3.02 ULP from ICC's published construction**,
  which is ICC's own arithmetic from their own printed matrices — **not an
  independent measurement of anything physical.**
- **That NC-221/NC-222 became ground truth.** ★★ **They did not.** Their
  reference is still a file and the machinery on both sides is still ours.
- **That the §B.2 transcription has been checked.** ★★★ **It has been read
  ONCE, by ONE agent**, and this librarian **could not open the PDF**.
- **That `sRGB2014.icc` is now a second source.** ★ **NC-225 stands** —
  one lineage among the *files*; what changed is that a *document* now
  sits outside that lineage.
- **That the crash class is closed.** ★★ **One profile, one channel count
  (7), one machine.** Nothing here enumerates which other published
  profiles have ≥5 channels.
- **That four is all the `A2B`-only profiles there are.** ★ **Four were
  found by a shape scan of two private corpora.** No claim is made about
  any wider population.
- **That the blindness is bounded.** ★★ **It is demonstrated, on ONE axis
  (`Z`) of ONE white point, with FOUR injected drifts.** Nothing measures
  blindness in `X` or `Y`.
- **That `pass=274` is comparable with anything.** ★★★ **It is not** —
  different member set, different day, uncommitted tree.
- **That a CI run has been observed.** ★ **Twenty-three entries, none.**
  And **CI covers none of §3.33 except NC-230**, which is the one row here
  that **needs no corpus and cannot skip**, because its expectation comes
  from a document rather than a file.
- **That the tree is clean or the tip is pushed.** ★ Nothing here went
  near a remote. **Rule 9 and DL-024 unchanged.**

---

## 2026-08-17 — the **tail-debt #7** filing (twenty-fourth `SESSION_LOG` entry). **A six-day-old "nobody has looked" is looked at — and the half of the same debt that LOOKED discharged, because a file with the right name exists, is the half that was wrong**

**Filed by `icc-librarian` from an `icc-engineer` dispatch.** ★
**Twenty-fourth entry here; twenty-fifth librarian filing** — the two
populations still differ by one and **neither integer may be quoted
without its population** (DL-031). **By entry count this is the seventh
filing of 2026-08-17.**

★ **Smallest filing in this log by substance, and deliberately so: one
tail-debt row, no Pass, no code, no ledger row.** It is here because the
fact it records has an expiry date, and a fact with an expiry date that
lives nowhere is worse than one nobody checked.

### ★★★ Four things that bound everything below

1. **No colour was measured, no code changed, no `NUMERIC_CLAIMS.md` row
   was added or edited.** See *"Not filed in the ledger"* below — that is
   a judgement, recorded as one.
2. **This librarian has no shell.** The crates.io result is
   **`[REPORTED]`**, carried with its command and its five verbatim
   response bodies. **Nothing was re-run, and no network request was made
   from this session.**
3. **Nothing here authorises anything.** **Rule 9 and DL-009 stand
   exactly as they did.** No publish, push, tag or release is authorised,
   and a free crate name is not an approval to take it.
4. **★★ No claim is made about git.** `THIRD_PARTY_LICENSES.md`,
   `about.toml` and `about.hbs` were **read in the working tree**;
   **whether any of them is tracked, committed or pushed was not
   checked** and is not asserted anywhere in this filing.

### The half that was checked — and it is a dated observation, not a reservation

`icc-engineer` queried `https://crates.io/api/v1/crates/<name>` for
**`iccce`, `iccce-color`, `iccce-profile`, `iccce-cmm`, `iccce-cli`**.
All five returned exactly ``{"errors":[{"detail":"crate `<name>` does not
exist"}]}``.

> ★★★ **crates.io has no reservation mechanism.** A name is claimed by
> the first successful publish, so this is a statement about **one
> instant on 2026-08-17** and about nothing after it. **It must be re-run
> immediately before a first publish, and must never be quoted later as a
> standing guarantee.** *"The names were checked"* is precisely the
> sentence that stays true, goes stale, and stays load-bearing — which is
> why the date is part of the claim rather than a footnote to it.

**★★ And it covered five of six candidate names.** The queried list is
**verbatim DL-009's list of 2026-08-11**, written **before
`iccce-measure` existed** (added 2026-08-12). `Cargo.toml`'s `members`
and `Cargo.lock` both name **five** crates *(verified — read)*.
**`iccce-measure` was not queried.** ★ A stale enumeration copied into a
query returns five clean results and **looks complete** — DL-053, where
the measurement is real and the denominator is missing. ★ `iccce`, which
*was* queried, is a **facade name that is not a workspace member**, so
the queried set is neither a subset nor a superset of the publishable
set.

### ★★★ What this librarian found by reading, not carried by the dispatch

**The dispatch said a file named `THIRD_PARTY_LICENSES.md` now exists,
said it had not read it, and said explicitly not to discharge the debt on
the filename. That instruction paid for itself.** All of the following
are *(verified — read)*:

1. **It IS genuine `cargo-about` output.** Its intro prose is
   **byte-identical to the prose in `about.hbs`**, and `about.toml` +
   `about.hbs` both exist with the full regeneration procedure. **The
   apparatus `LEGAL.md` §1 asks for is real and executable** — that half
   of the debt is genuinely built.
2. **★★ Its DATA is stale in two independent ways.** It lists **four**
   crates at **`0.0.1`**; the workspace is **five** crates at **`0.1.0`**
   (`[workspace.package] version`, and every package in `Cargo.lock`).
   **`iccce-measure` is absent entirely**, while the file's own prose
   asserts *"The **four** crates … **are this project**."*
3. **★★★ Regenerating it today would emit a DEFECTIVE entry, not merely a
   missing one.** `about.toml` carries four `[<crate>.clarify]` blocks and
   **none for `iccce-measure`** — and by **`about.toml`'s own written
   rationale**, a crate without one falls back to the **generic SPDX MIT
   placeholder** (*"Copyright (c) `<year>` `<copyright holders>`"*), which
   that same comment calls **"worse than publishing nothing."** ★ **An
   inference from the config's stated reasoning, not a measurement** — no
   agent has run `cargo about` at this tree state.
4. **`about.hbs`'s prose also hardcodes "four"**, under its own banner
   *"★ READ THIS BEFORE EDITING THE PROSE BELOW"*. **Regeneration alone
   will not fix it.**

> ★★★ **The rule, filed as DL-058: the existence of an artifact is not
> the status of the obligation that asked for it.** A generated file is a
> claim about a tree **at the moment of generation**, and it does not
> become malformed when the tree moves on — **it becomes quietly wrong
> while staying well-formed and confident.** ★★ **DL-048's mechanism in a
> fourth register**: the right name at the right path invites the reader
> to accept the destination.

★ **A note on the instrument.** `LEGAL.md` §1's *"generated by
`cargo-about`, never hand-written"* is a rule about **how**, with no
clause about **when**. It was fully honoured and still produced a stale
manifest. **A generation rule needs a regeneration trigger to be
complete** — and `about.toml`'s stated trigger is *"after ANY dependency
change"*, which **adding a workspace member is not**. That is the exact
gap `iccce-measure` fell through.

### Verdict on tail debt #7, and it is a split

| Half | Status |
|---|---|
| *"name availability is still unchecked by anyone"* | **DISCHARGED as to *has anyone looked*, 2026-08-17** — with an **expiry**, a **five-of-six coverage gap**, and **no authorisation of any kind** |
| *"`THIRD_PARTY_LICENSES.md` via `cargo-about` still owed"* | **★★ CARRIED WITH A CAVEAT — neither *"still owed"* nor *"discharged"* is now the right word.** The apparatus exists; the output is stale in version and in membership; **the config would regenerate a placeholder licence for the fifth crate** |

### Newly owed (all to `icc-engineer`; each needs a shell)

1. **Query `iccce-measure`** on crates.io, in the same shape, before the
   same publish.
2. **Add `[iccce-measure.clarify]` to `about.toml`** — pointing at
   `../../LICENSE` with the same sha256 as its four siblings. ★ **Before**
   item 4, or item 4 emits the placeholder.
3. **Update `about.hbs`'s "four crates" prose**, preferring wording that
   does not restate a count (its own warning).
4. **Regenerate** and confirm five crates at `0.1.0`:
   `cargo about generate --workspace about.hbs -o THIRD_PARTY_LICENSES.md`.
   ★ `LICENSE`'s sha256 is **pinned** in all four `clarify` blocks and
   dated 2026-08-12 in the comment — **a checksum hard-fail here is the
   pin working, not a bug to route around.**
5. **★ Re-run the name query immediately before any first publish**, and
   record *that* run's date. This entry is not that run.

### Not filed in the ledger — a judgement, recorded so it can be overturned

**No `NUMERIC_CLAIMS.md` row was added and no `§7.20` status pass was
opened.** The ledger's subject is **tolerances and measured errors**;
every row states a colour-or-numeric evidence class and an HTTP response
body has none. ★ **§7 has never carried a crates.io item**, and opening a
status pass for one would have moved an integer that section itself
tracks (*"twenty-third status pass … twenty-fourth librarian filing"*,
§7.19) — **inflating a tracked count to hold a non-numeric fact.** The
durable home is `ROADMAP.md`'s *Publication — crates.io* plus **DL-058**.
*(Overturn by argument, not by noticing the absence.)*

### Filed this session

| Where | What |
|---|---|
| `ROADMAP.md` | A dated **2026-08-17** subsection in *Publication — crates.io* (the full correction), and a **scoped** dated note after the three 2026-08-12 tail-debt roundups pointing at it. ★ **Nothing edited in place**; the stale *"nobody has looked"* wording is left standing at all four sites, superseded rather than rewritten. **No Pass status changed.** |
| `ARCHITECTURE.md` | **§5 — DL-058.** **DL-001 … DL-057 untouched**, including the four that carry the stale *"still unchecked"* line (**DL-009**, **DL-024**, **DL-029**, **DL-030**) — an append-only register is corrected by a new entry, not by editing the old ones. |
| `SESSION_LOG.md` | This entry. **No earlier entry rewritten or annotated.** |
| `NUMERIC_CLAIMS.md` | **Nothing.** Deliberate; reasoned above. |

**Not touched, by ownership:** `THIRD_PARTY_LICENSES.md`, `about.toml`,
`about.hbs`, `Cargo.toml`, `Cargo.lock` (**read as evidence, and the
three repairs recorded as owed rather than made** — they are
`icc-engineer`'s and two of them need a shell anyway), `docs/LEGAL.md`
(**read, §1 quoted, not edited**), `docs/TOLERANCES.md`, `tools/`,
`crates/`, `README.md`, `CLAUDE.md`, `.github/`, `fixtures/`, and **every
file in the request channel**.

### Left for the next session to not assume

- **That the names are available.** ★★★ **They were, at one instant on
  2026-08-17, and five of six were asked.** Nothing since has been
  checked by anyone.
- **That anything may now be published.** ★★★ **No.** Rule 9, DL-009 and
  DL-024 are untouched; a free name is not a go-ahead.
- **That tail debt #7 is closed.** ★★ **One half discharged, one half
  carried with three named repairs.**
- **That `THIRD_PARTY_LICENSES.md` is publish-ready.** ★★★ **It is not.**
  Four crates at `0.0.1`, against five at `0.1.0`.
- **That regenerating it is safe today.** ★★★ **It is not** — do item 2
  above first, or the fifth crate gets the placeholder licence.
- **That the six-name list is now settled.** ★ **A sixth workspace member
  would recreate both defects at once** — an unqueried name and an
  uncleared `about.toml`.
- **That this filing checked the GitHub remote.** ★ **It did not.**
  DL-009's separate *"whether that remote exists has never been checked"*
  is untouched; a crates.io query says nothing about it.
- **That the tree is clean, committed or pushed.** ★★ **Nothing here ran
  `git` or went near a remote**, and the tracked status of every file
  read this filing is unknown to it.

---

## 2026-08-17 — the **Ghent boundary reclassification** filing (twenty-fifth `SESSION_LOG` entry). **A patch this project called "genuinely ours" is not ours — and the error ran in the CLAIMING direction, which nothing catches**

**Filed by `icc-librarian` from an `icc-engineer` dispatch**, which in
turn carried an `icc-spec-librarian` sourcing result. ★ **Twenty-fifth
entry here; twenty-sixth librarian filing** — the two populations still
differ by one and **neither integer may be quoted without its
population** (DL-031). **By entry count this is the eighth filing of
2026-08-17.**

### ★★★ Five things that bound everything below

1. **No colour was measured. No code changed. `NUMERIC_CLAIMS.md` gained
   NO ROW** — next free identifiers are still **NC-243** and **NA-012**.
   It gained an **owed block, §7.20**, and that disposal is a librarian
   judgement recorded as one.
2. **This librarian has no shell.** Every `[VERIFIED]` mark below means a
   file was **read** with the Read/Grep/Glob tools. No command was run,
   no build, no test, no `git`.
3. **★★ The clause numbers were re-settled from a corpus, not taken from
   the dispatch.** `D:\Dev\Rag-Specialized\PDF_Spec\` was the arbiter,
   cited to file and line. **This caught a wrong clause** — see finding 3.
4. **★★★ Nothing here cuts scope.** **CMYK→CMYK black preservation
   remains genuinely this project's**, unimplemented and being built.
   Reading a scope cut into this filing is the one misreading it is
   written to prevent.
5. **★★ No claim is made about git state beyond two files' contents.**
   `.git/refs/heads/master` and `.git/logs/HEAD` were **read as text**.
   That is a statement about those bytes, **not** about `git status`, not
   about the index, and not about a remote.

### The reclassification

`docs/GHENT_COMPATIBILITY.md` §3.1 listed **GWG 23.0 "Four different
Grays"** in **Tier A — "genuinely a CMM's problem"**, glossed as *"K-only
preservation … the classic black-preservation trap … CMM policy, engine
plumbing."*

> **It is device-space channel routing — the same boundary class as
> overprint — and it is `pdfce`'s.** All four gray definitions resolve to
> the same single-channel device answer **inside PDF**, before a CMM is
> reached.

| leg | clause | status here |
|---|---|---|
| `DeviceGray` → CMYK | **ISO 32000-1 §10.3.3** = **ISO 32000-2 §10.4.2.3**, a **`shall`** | **[VERIFIED]** — `PDF_Spec\color\color__cie_based.md:549` |
| the same, **colour-managed** | **ISO 32000-2 §10.3.2**, and the sentence sits **inside the ICC-enabled branch** | **[REPORTED]** — ★ **the load-bearing clause, and the one still owed a re-derivation** (§7.20 item 3) |
| `Separation /Black` | **ISO 32000-1 §8.6.6.4** | clause identity **[VERIFIED]**; the ignore-rule **[REPORTED]** |
| `DeviceN [/Black]` | **§8.6.6.5** | **[REPORTED]** |
| `DeviceCMYK 0/0/0/K` | **§10.3.1** | **[REPORTED]** |

★ **The patch's own readme settles it independently**, and names
`DeviceCMYK` as the reference: *"created … **without performing color
conversion**"*, *"Usually, the object defined in DeviceCMYK should render
as expected"* **[QUOTED]**. A patch whose reference is an unconverted
channel is a **non-conversion test** — structurally identical to GWG 8.2,
which §3.3 had **already classified correctly one table earlier**.

### ★★★ Finding 1 — the direction of the error is the reusable part

**This project claimed work that is not its own**, and that asymmetry is
why DL-059 exists rather than a footnote:

- **An under-claim is caught by a consumer.** `pdfce` hits the gap and
  files in the request channel. The channel exists for that.
- **An over-claim is caught by nothing.** There is no failing test for
  work you do not own. It produced *"not attempted"* lines in §5 and §6
  that **looked like diligence** and could be carried for ever, with **no
  number moving** — DL-055's shape, in the scope register.
- ★★ **And it was not merely idle.** Building it would ship an ICC path
  for a leg the standard routes *around* ICC. **A feature that would be
  wrong to ship, and would look right, because its output is a gray that
  matches.** Rule 1, one level up from the pixel.

### ★★ Finding 2 — attribution strength: there is no GWG requirement "23.0"

`icc-spec-librarian` retrieved **GWG 2022** (the current edition; **there
is no 2023**) **[REPORTED]**: requirements are `Dxxx`/`Rxxx`, **no
"23.0"**; the nearest construct is **`D0013 "Black Colour"`** — a
**definition consumed by the overprint requirements R0009–R0015**; and
GWG's real `DeviceGray` rule is **`R0011`: ban it** for small black text
*because overprint is not always honoured for it*.

> ★★ **GWG's own specification files this under overprint.** Two
> independent routes, one boundary.

⇒ `n.m` is **Output Suite *patch*** numbering. The *"shall show the same
visual result"* quotation is genuine — but its authority is **patch
documentation, not the GWG specification**. **A `shall` in a test patch's
readme is the patch author's `shall`.**

★ **The document was better provenanced than the dispatch assumed.** §9
already recorded §3.1 as `[REPORTED]` from a readme sweep, *"ownership
calls are `icc-engineer`'s"*. **What was missing is that it was recorded
in §9 and not where the reader meets the claim, three hundred lines
earlier** — a provenance table at the back does not travel with a row
quoted out of the middle.

### ★★★ Finding 3 — a wrong clause, live, in in-flight code (and the module that got the boundary RIGHT)

`tools/difftest/src/passk.rs` **reaches the same conclusion
independently and deserves the credit**: it refuses to assume the
boundary, measures **both legs**, and states *"if that is the operative
rule, the leg belongs to `pdfce` and not to this project at all."*
**DL-059 is the answer to the question that module declined to answer for
itself.**

**Its clause is wrong**, and it is the citation the argument rests on:

- `passk.rs:227` cites **PDF 32000-1 §8.6.4.4**;
- **§8.6.4.4 is *DeviceCMYK Colour Space*** **[VERIFIED]**
  (`PDF_Spec\iso32000\iso32000__s__8.6.md:52,150`);
- the rule is **§10.3.3** **[VERIFIED]**.

★ **§8.6.4.4 is a known attractor** — the PDF corpus already carries a
standing correction of the identical substitution for a *different*
subject: *"this material is §8.6.5.5, not §8.6.4.4"*
(`color__iccbased.md:15`). It is where a reader reaches when the topic is
"device colour spaces" and the rule lives in §10. **DL-057 exactly: a
wrong clause is worse than a vague one, because the citation is what
makes it persuasive.**

★★★ **Why it is OWED and not corrected.** **`passk.rs` is in no commit.**
The branch tip is **`506fcd3`** ("conformance: Pass I") **[VERIFIED —
`.git/refs/heads/master` and `.git/logs/HEAD` read directly]**, and
`docs/` contains **no mention of Pass K, of black preservation, or of the
`TOLERANCES.md` §3.10.8 the module cites** — **`TOLERANCES.md` §3 runs
3.1 … 3.9.8 and has no §3.10** **[VERIFIED — grepped]** — although the
module **is** wired into `main.rs:121,510-511` and `lib.rs:252`.

> **Pass K is in flight in a concurrent session. Not one of its numbers
> is quoted, claimed or filed anywhere in this filing.** A librarian who
> found a passing measurement on disk and recorded it would be filing a
> claim from a tree state nobody has committed.

### ★★ Finding 4 — a premise check on the DISPATCH, which failed

The dispatch asked for a sweep of the phrasing ***"GWG 23.0 demands…"***,
described as *"written repeatedly"* by this project.

> **It appears nowhere in the repository** **[VERIFIED — whole tree
> grepped for `GWG ?23`, `GWG230`, `Four.different.Gray`]**. `ROADMAP.md`
> had never mentioned patch 23.0 at all.

What exists is §3.1's **column heading**, *"the capability it demands of
a CMM"*, applied to all six rows — almost certainly what was remembered.
★★ **No filing was made against a string the document does not contain.**
That is **DL-048 arriving from the other end**: there a *citation*
pointed at a destination that did not hold the claim; here a *correction*
was aimed at text that was never written. **Both are settled the same
way — read the destination.**

### ★ Finding 5 — the counts, and the `~` that was doing real work

| | was | is |
|---|---|---|
| patches in the suite | *"~48"* | **51** — `1-CMYK` **27**, `2-SPOT` **8**, `3-ICC-CMS` **16** |
| `1-CMYK` + `2-SPOT` | *"32"* | **35** |
| touching colour conversion | 16 | **16 — unchanged and exactly right** |
| **genuinely ours** | six | ★ **five** |

**[VERIFIED — this librarian enumerated `…\Categories\*\Patches\*.pdf` on
disk.]** ★★ *"Sixteen of the ~48"* pairs an **exact numerator with an
estimated denominator** and reads as a measured ratio. **DL-053 in
miniature**: the 16 was counted, the 48 was not, and nothing in the
sentence distinguished them.

★ **A corroboration that cost nothing.** The same sweep returns **98
PDFs** across the tree, and §4.1's *"98 PDFs scanned"* — obtained by
`extract_icc.py` walking the tree, **not** by counting files —
**agrees exactly.** Two instruments, one number.

### What was written

| document | what changed |
|---|---|
| **`ARCHITECTURE.md` §5** | **DL-059** — the reclassification, the *"name the clause and the layer"* rule, the claiming-direction asymmetry, the attribution-strength finding, the wrong-clause finding, and four explicit non-effects |
| **`GHENT_COMPATIBILITY.md`** | **new §3.5** (evidence, clauses, readme quotations, the GWG-2022 premise check, the `passk.rs` clause defect); **§3.1 marked SUPERSEDED IN PART and NOT rewritten**; §3 and §3.3 counts corrected in dated blocks; §4.6, §5.6 and §6's row annotated or struck; **six new provenance rows in §9** |
| **`NUMERIC_CLAIMS.md`** | **new §7.20** — no row, six owed items, and the explicit non-effects |
| **`ROADMAP.md`** | dated update inside the Ghent standing workstream |

★ **§3.1 was deliberately NOT rewritten.** The wrong classification is
the only record of how the error looked from inside, and this project's
practice is dated supersession — the same treatment §4.3's withdrawn
*"does not move measurably"* and §4.5's downgrade to a negative result
received.

### What is owed (`NUMERIC_CLAIMS.md` §7.20)

1. **★★★ The `passk.rs:227` clause correction** — `icc-conformance`.
2. **★★ `passk.rs`'s dangling `TOLERANCES.md` §3.10.8** — must not be
   committed pointing at nothing.
3. **★★ Re-derive ISO 32000-2 §10.3.2's ICC-branch sentence** —
   `icc-spec-librarian`. **The load-bearing clause of the whole
   reclassification, standing on one dispatch.**
4. **★★★ Apply the *"name the clause and the layer"* test to the other
   five Tier-A rows** — `icc-engineer`. They are **"not yet checked"**,
   not "checked and confirmed".
5. **★ Register the GWG-2022 finding in `ICC_Spec`** — it currently lives
   only in agent memory and in §3.5.
6. **★★ The *"GWG 23.0 demands"* sweep** — closed by being recorded; the
   phrase does not exist and nothing is owed by it.

### What a reader must NOT take from this filing

- **That black preservation is no longer this project's.** ★★★ **False.**
  **CMYK→CMYK black preservation is genuinely ours**, unimplemented, and
  being built. GWG 23.0 is simply not what tests it.
- **That ICC.1 was re-checked here.** ★ **It was not.** *"ICC.1 contains
  no black-preservation construct in either edition checked"* stands on
  `ICC_Spec` **A51**/**A52** and `icc__ref__black_preservation.md`, and
  is **untouched and independent** of everything above.
- **That the other five Tier-A patches are confirmed ours.** ★★ **They
  are unexamined**, and §3.1 now says so.
- **That any number moved.** ★★★ **None did.** §3.30's 20 profiles,
  §3.31's 11, every ΔE, every tolerance: unchanged.
- **That Pass K exists, landed, or measured anything citable.** ★★★
  **Nothing from it is filed.** The branch tip is `506fcd3` and contains
  no Pass K.
- **That the tree is clean, committed or pushed.** ★★ **Two `.git` files
  were read as text and nothing else.** No `git` was run; the tracked
  status of `passk.rs` beyond *"not in the commit `506fcd3` points at"*
  is unknown to this filing.
- **That anything about Ghent may now be said in public.** ★★★ **No.**
  §7.16's three operator decisions are untouched. **A clearer boundary is
  not a permission.**

---

## 2026-08-18 — the **PDF-rasterisation retraction** filing (twenty-sixth `SESSION_LOG` entry). **A capability this project recorded as absent was present all along; the false claim was inferred from ONE failing call; and what it cost was not convenience but the only INDEPENDENT check on any transcribed equation**

**Filed by `icc-librarian`.** **No code changed. No Pass shipped. No
numeric claim moved — no ΔE, no tolerance, no measured error, no coverage
figure.** Next free identifiers remain **NC-243** and **NA-012**.

### ★★★ Four things that bound everything below

1. **Nothing here was measured by this librarian.** It **has no shell**,
   ran nothing, rendered no page, and did not confirm `pdftoppm`'s
   absence. The capability claim is **`[REPORTED]` by
   `icc-spec-librarian`**, corroborated by reading its
   `ICC_Spec\LEGAL_NOTE.md` §1b **at the tip**.
2. **The dispatch was verified against live source, and one of its
   specifics was WRONG** — see "A premise check that failed" below.
   `docs/` was **swept**, not taken from the dispatch's list of three
   files.
3. **Nothing is committed and nothing is pushed.** No authorisation was
   sought or given, and this librarian ran no `git`.
4. **The correct claim is narrow.** ★★★ ***"PDF pages can be rasterised
   via `pypdfium2` and read"*** — **NOT** *"PDF reading works"*.
   `pdftoppm` really is missing and the Read tool really does refuse a
   `.pdf` handed to it directly.

### The retraction

`icc-spec-librarian` retracted, on its own initiative, the claim that the
Read tool **cannot render PDF pages in this environment**. It found the
claim had been **overgeneralised from a single failing invocation**.

| | as recorded | as measured |
|---|---|---|
| `pdftoppm` (poppler) | absent | **absent — true, and the only thing the failing call showed** |
| rasterising a page at all | impossible here | **possible** — `pypdfium2` is installed |
| Read on the resulting PNG | never attempted | **works** |

It fixed its own corpus (seven files) and placed the canonical recipe in
`ICC_Spec\LEGAL_NOTE.md` §1b. **It escalated the occurrences in this
repository rather than editing them**, which is the correct boundary and
is worth recording as such.

### ★★★ Why this is a decision-log entry and not a typo fix

**It is not a convenience issue.** ICC.1:2022 sets `+`, `−`, `×`, `≤`,
`≥` in the **Symbol font**. **All three text engines drop them, for the
same reason.**

> **This project has been treating agreement between text-extraction
> engines as corroboration. For that glyph class it is not: the engines
> agree BECAUSE THEY SHARE THE SAME WRONG ASSUMPTION. Their errors are
> correlated, so three-engine agreement is not independent evidence.**

**A rasterised page does not share the assumption.** So the retraction
restores **the only currently-available independent check on any
transcribed equation in this project** — and for six days that check was
believed unavailable.

★ **This is DL-033/DL-056's shape in the sourcing layer.** There, a
differential test was blind toward its own reference; here, three
instruments are blind in the same direction and their agreement was read
as confirmation.

### ★★ It is THREE instances, not two — and the third already had the rule

The dispatch offered this as a second instance after `pdfce`'s
2026-08-08. **There is a third, in this project's own supply chain**
*(verified — `ICC_Spec\LEGAL_NOTE.md` §4 rule 6, read at the tip)*:
incident **C6**, 2026-08-12 — *"`itu.int`'s WAF rejects every agent
request"* propagated to three corpus files, when in fact the WAF rejects
the bare UA `Mozilla/5.0` and **only** that. **The document was one
`curl` away for five days.**

★★★ **C6 produced the correct binding rule** — state the tool, the exact
flags, the UA, the status and the response size — **and instance 3 was
written anyway, because that rule lives in the corpus and had no
counterpart in this repository's decision log.** ★ **A rule recorded only
in `ICC_Spec` does not bind `docs/`.** That gap, more than the pattern,
is what **DL-060** closes.

### ★ A premise check on the dispatch, which failed

The dispatch asked that **`TOLERANCES.md`'s "Sharma & Starr" row** — an
inequality glyph read as `<` by derivation from a sibling glyph,
labelled `DERIVED` — be named as now-checkable.

> **No such row exists.** *(Verified — `Starr` returns **zero hits
> anywhere in this repository**. `TOLERANCES.md` §3.1.1's Sharma row is
> **Sharma, Wu & Dalal (2005)**, tolerance `1×10⁻⁴`, justified by the
> published data's printed precision — no glyph, no inequality, no
> `DERIVED` label.)*

**What was being reached for is in `icc-spec-librarian`'s corpus, not
ours, and is already discharged there:** *Sharma & Starr, JIST
54(6):060504, 2010* sits beside **Cholewo 2000**, whose **Eq. (1)** was
promoted RECONSTRUCTION → **VERIFIED** on 2026-08-18 **by a rasterised
page that recovered the `≤` glyphs**. **Nothing is owed to anyone by this
item.**

★★ **DL-048 for the second filing running** — §7.20 item 6 was the first.
**A correction aimed at a document that does not contain the thing being
corrected is settled the same way as a wrong citation: read the
destination.**

### What was written

| document | what changed |
|---|---|
| **`ARCHITECTURE.md` §5** | **DL-060** — the retraction, the narrow correct claim, the recipe, the correlated-error finding, the three-instance table, why **DL-042** does not already cover it, an explicit statement of what this librarian did **not** verify, and five non-effects |
| **`LEGAL.md`** | dated correction blocks above **§2.4** and **§2.5**; ★ **the original paragraphs left standing, not rewritten** — the wrong claim is the record. §2.5's *"figures not seen"* becomes *"not yet seen"* |
| **`NUMERIC_CLAIMS.md`** | **§3.33.1**'s blockquote corrected in place (the *fact* stands, the *reason* was wrong); **§7's owed item 2** marked **UNBLOCKED**; **new §7.21** |
| **`SESSION_LOG.md`** | this entry |

### ★ What was examined and deliberately NOT changed

- **`ARCHITECTURE.md` DL-014's *"No PDF was opened…"*** *(line 1343)* —
  a **dated scope statement about what one librarian did**, not a
  capability claim. **True when written, true now.** ★★ **Correcting it
  would be over-reach**, and the discipline that keeps this correction
  honest is the same one that made it necessary.
- **Entry 23 of this log (item 4, *"the PDF … could not be read"*)** —
  **this log is append-only.** The statement is corrected **by this
  entry**, not edited where it stands.
- **Every measured row.** No transform, ΔE, fixture or runner is touched.

### What is owed (`NUMERIC_CLAIMS.md` §7.21)

1. **★★★ NC-230's second reading of §B.2 — UNBLOCKED, still UNDONE**, and
   it must be a **raster**, not a fourth text extraction.
   `icc-spec-librarian`.
2. **★★ A sweep for rows resting on multi-engine agreement over a
   glyph-sensitive passage.** §7.21 names **two** places; **that is an
   identified instance, not an inventory.**
3. **★ Record the render route WITH ITS INVOCATION** the first time a
   shell-holding agent here uses it. `icc-engineer` / `icc-conformance`.

### Passed on as a courtesy, and it is not ours

`icc-spec-librarian` reports that its **ambiguity register's `revised:`
line claims 56 rows where a recount gives 58**, before its new entry. **It
flagged rather than silently corrected**, which is the right disposal.
**Nothing in this repository depends on the total**, and no `iccce`
document cites it. Recorded here only so the flag is not lost.

### What a reader must NOT take from this filing

- **That "PDF reading works".** ★★★ **It does not.** `pdftoppm` is
  absent; the route is `pypdfium2` → PNG → Read. A future session will
  meet that same error message and must find the workaround here rather
  than re-derive the wrong conclusion.
- **That this librarian confirmed the capability.** ★★ **It did not** —
  no shell, no render. `[REPORTED]` with a named first use.
- **That any sourcing permission widened.** ★★★ **None did.** DL-002's
  `color.org` bar and DL-007's ITU-R test stand. **A new way to READ a
  held document is not a new way to OBTAIN one.**
- **That a reproduction right appeared.** **No.** A rendered page may no
  more be pasted into `docs/` than extracted text may.
- **That any row was downgraded.** ★★ **None was.** Rows are
  **qualified**; two-engine agreement remains sound, it is simply no
  longer the ceiling and not independent for Symbol-font glyphs.
- **That NC-001 is affected.** ★★★ **It is not** — different document,
  different publisher, provenance is not engine agreement on ICC.1:2022.

---

## 2026-08-18 — the **first use of the rasterisation route** filing (twenty-seventh `SESSION_LOG` entry). **The capability is VERIFIED one day after being retracted into `[REPORTED]`; its first use in this repository found that a superseded row's NUMBERS were wrong too — and the right numbers are ISO 32000's formula already evaluated, which is DL-059 confirmed by the artwork itself**

**Filed by `icc-librarian`.** **No code changed. No Pass shipped. No
numeric claim moved — no ΔE, no tolerance, no measured error, no coverage
figure.** Next free identifiers remain **NC-243** and **NA-012**
*(verified — the highest in `NUMERIC_CLAIMS.md` are NC-242 and NA-011)*.

### ★★★ Three things that bound everything below

1. **This librarian still has no shell.** The capability promotion rests
   on evidence `icc-engineer` **carried** — a `which` result, a stated
   render call, a stated PNG size, and the Read tool displaying it.
   ★ **The panel values and the intent declaration were read by
   `icc-engineer` from that image, not by this librarian**, and they are
   filed as `[QUOTED-FROM-RASTER]` for exactly that reason.
2. **★★ "The readme declares" — never "the patch paints".** Every new
   fact here comes from the GWG 23.0 **README**. The **patch PDF has
   never been opened**, and on a corpus that deliberately ships two
   corrupted trap profiles that gap is not free. Owed at
   `NUMERIC_CLAIMS.md` §7.22 item 2.
3. **Everything below was re-read in the live source**, including two
   things the dispatch did not mention and one it got slightly wrong.

### What the dispatch carried, and what verifying it added

| | the dispatch | after reading the source |
|---|---|---|
| DL-060's capability | promote `[REPORTED]` → VERIFIED | **done** — and §7.21's owed item 3 discharged after **one** filing |
| §3.1's GWG 23.0 numbers | *"wrong twice over"* | **confirmed**, and §3.1 is **the only place in that file those values appear** *(verified — the whole file grepped)* |
| §3.4's intents table | *"incomplete, 23.0 absent"* | **confirmed** — ★ and the new datum is **readme-declared, not file-read**, so it is kept **out** of that table rather than added to it |
| the `passk.rs` wrong clause (DL-059, owed) | not mentioned | ★★ **DISCHARGED** — nine `§10.3.3` citations, **zero** `8.6.4.4` |
| — | not mentioned | ★★★ **`passk.rs:1342`/`:2446` call `g = 0.5` *"GWG's own patch value"* and PRINT it into the report.** The patch's value is **25 %** |
| — | not mentioned | ★★ **`passk.rs:291`** lists an **`ICCBased` gray** panel; the readme says **`DeviceN`** |
| where 50/50 came from | *"someone reasoning from 'a gray-equivalence test'"* | ★ **a checkable candidate in the same file**: **§3.3 attributes "50 % K, 50 % Gray, 50 % spot black" to GWG 3.0**, one table later, and calls it the deceptive lookalike |

### ★★★ The two findings that are new work

**1. `1 − 0.25 = 0.75`.** The readme's panels — **25 % · 0/0/0/75 · 75 ·
75** — are **ISO 32000-1 §10.3.3 / 32000-2 §10.4.2.3 evaluated**. GWG
**authored the patch on the device-space rule**, so the equivalence it
tests is the PDF formula's own output and **a CMM cannot be what
produces it**. That is DL-059's conclusion reached from a **third
direction** — after the clause text and the readme prose — and reached
**by the artefact speaking about itself**. ★ The order is what makes it
worth having: **the clause argument came first and the artwork agreed
afterwards.**

**2. A number labelled with its source is harder to check than a bare
one** — **DL-061**. *"GWG's own patch value"* is the phrase that stops a
reader looking it up, and it is **printed into the Pass K report** for a
value the patch does not contain. **The ΔE is correct; only the
justification is false**, so nothing recomputes, **no test can fail on
it**, and a change ledger has nothing to record.

### ★★ A second failure mode for text extraction, broader than DL-060's

DL-060 recorded three engines dropping Symbol-font glyphs **together** —
agreement that is **correlated, not independent**. Both facts read on
2026-08-18 are set in a **figure**, which no text engine returns at all.

> **For figure-borne content, engine agreement is not correlated — it is
> VACUOUS, and the shared silence reads as absence of the fact.** The
> first mode **corrupts a value you can see**; the second **hides that a
> value exists.**

⇒ §7.21's owed sweep widens from *"glyph-sensitive passages"* to
*"glyph-sensitive **or possibly set in a figure**"*, and **the second
half cannot be found by comparing extractions.**

★ **The stumble is filed too:** `pypdfium2.V_PYPDFIUM2` **raised** while
the render **succeeded**. **A capability is falsified by the capability
failing, never by its metadata failing** — DL-060's own error, met
inside the act of verifying its retraction.

### What was written

| document | what changed |
|---|---|
| **`ARCHITECTURE.md` §5** | **DL-061** (new); **dated addition to DL-059** (third-direction corroboration + the discharged clause + the two new prose defects); **promotion block appended to DL-060** (`[REPORTED]` → VERIFIED, the version-probe stumble, the figure/vacuity extension, and a precise first-use record) |
| **`GHENT_COMPATIBILITY.md`** | §3.1 supersession block **EXTENDED** — the row is wrong twice, its numbers too; **§3.5 dated addition** (panel values, the `1 − 0.25` arithmetic, what is still unknown, why a new provenance class); **§3.5's owed clause DISCHARGED with two new defects recorded**; **§3.4 marked INCOMPLETE** with 23.0's readme-declared intent kept deliberately out of the table; **§9 gains `[QUOTED-FROM-RASTER]` and five rows**, and its `passk.rs`/tip row gets a dated correction |
| **`NUMERIC_CLAIMS.md`** | §7.21 item 3 **struck and discharged**; **new §7.22** — no row, three owed items, six explicit non-effects |

★ **§3.1's row is STILL not edited.** It is now wrong in two independent
ways and stays in place under a dated block — the same practice §4.3's
withdrawal and §4.5's downgrade received.

### What is owed (`NUMERIC_CLAIMS.md` §7.22)

1. **★★★ `passk.rs`'s two prose defects** — `icc-engineer`. The emitted
   string is the urgent half: **it leaves the repository.**
2. **★★ A content-stream read of `GWG230_Four_different Grays_x1a.pdf`**
   — whoever holds a shell. **A raster will not do it.**
3. **★★★ The multi-engine sweep, with a widened criterion** —
   `icc-spec-librarian`.

### What a reader must NOT take from this filing

- **That the patch's painted values are known.** ★★★ **They are not.**
  The **readme** declares them.
- **That any Pass K number is in doubt.** ★★ **None is.** `TOLERANCES.md`
  §3.10 stands; **neither defect appears anywhere in `docs/`** *(verified
  — grepped)*. Both are in one source file, in prose.
- **That DL-059 was reopened.** ★★★ **It was corroborated.** The
  ownership call was right; only §3.1's numbers were also wrong.
- **That §3.3's `50 %` figures are wrong.** ★ **They are GWG 3.0's and
  correct there** — and are the most plausible origin of the wrong pair,
  which is recorded as **a reading, labelled as one**.
- **That "PDF reading works".** ★★ **The verified claim is narrower:**
  `pdftoppm` is **absent**, the Read tool still refuses a `.pdf`
  directly, and **`pypdfium2` → PNG → Read** is the route.
- **That the scale or the index is now measured.** ★ The verified run
  used **`scale=2`**; DL-060's recipe says `scale=3.2`. **The call shape
  is verified; those two parameters are not.**
- **That the tree is clean, committed or pushed.** ★★ **Two `.git` files
  were read as text.** They show the tip at **`1a0509b`** and Pass K
  landed as **`846952f`**; **`git status` was not run and nothing about
  the working tree's tracked state is claimed.** ★ The `passk.rs` lines
  cited above were read **in the working tree**, not in a commit's tree.
- **That anything about Ghent may now be said in public.** ★★★ **No.**
  §7.16's three operator decisions are untouched.

---

## 2026-08-18 — the **Pass K** filing (twenty-eighth `SESSION_LOG` entry). **The largest block of rows this ledger has ever taken in one filing — and not one of them is `published-ground-truth`, because for this subject none can exist; the ORACLE is measurably wrong on the Pass's best row; and the defect a document still described as open had been fixed one commit earlier**

**Filed by `icc-librarian`.** **No code changed.** Rows added:
**`NC-243 … NC-266`** and **`NA-012`**; next free **`NC-267`** /
**`NA-013`**. Files touched: `NUMERIC_CLAIMS.md` (new **§3.34**, new
**NA-012**, five new **§6** dependency rows, new **§7.23**),
`ROADMAP.md` (a retrospective section), this log.
★★ **`TOLERANCES.md` was NOT touched** — it is `icc-conformance`'s and it
was in it concurrently.

### ★★★ Three things that bound everything below

1. **This librarian has no shell and ran nothing.** Every number came
   from an evidence package `icc-conformance` produced by re-running
   everything at tip **`60c32dd`**. ★ **The package lived in a temp
   directory outside every git repository** — a transport, not a record —
   so this filing is what makes it durable (`CLAUDE.md` rule 10:
   *nothing may exist only there*).
2. **The package was verified against live source, not filed on its own
   authority.** Everything load-bearing held; **two numbers in its
   suggested allocation were not carried by its own evidence tables** and
   were checked (see below).
3. **Commit ORDER was corroborated from `.git/logs/HEAD` read as text.**
   That is a statement about a file's contents — **`git status` was not
   run and nothing about the working tree's tracked state is claimed.**

### ★★★ The four findings this filing exists to preserve

**1. Zero `published-ground-truth` rows, and none is possible.** ICC.1
specifies **no black-preservation construct at all** (`ICC_Spec` **A51**,
a closed negative — the PCS is three components, so every device→device
transform is 4→3→4 and **K has no carrier**). ★★★ **24 rows landed in the
ledger's weakest classes while the strongest class stayed empty**, and
the standing debt is at its **ninth-plus** consecutive filing. **A large
section is not a strong one**, and §7.23 says so rather than leaving it
to be inferred from a row count. ★ It is also a **different**
impossibility from DL-041's — that one is a silence *inside* a specified
construct; this is a silence where **the construct does not exist**.

**2. ★★★ Rule 7 in its strongest form yet: the ORACLE is wrong and the
engine is right, BY ALGEBRA.** On a same-profile pair the equal-lightness
construction **is the identity** for any strictly monotonic `L*(K)`.
iccce returns **`0.000000`**; **lcms2 intent 11 sits `6.100000e-5` from
it** — **61× that row's own tolerance** — because its K returns through a
**17-node CLUT** while iccce inverts the ramp directly. **NC-256** is the
only Pass K row whose expectation comes from **neither implementation**,
and therefore the only one that could ever have adjudicated between them.
★ It carries its own falsifier: a flat stretch in the ramp would make the
inversion ill-posed and the identity would fail *for a correct
implementation*.

**3. ★★★ Seven rows are about the ORACLE, not about this engine.** §D's
rows have **lcms2 on both legs** *(verified — `passk/D/lcms2-intent-11/…`
in `passk.rs`)*. Filed as **NC-248 / NC-249** with that stated in the row
itself, because a reader seeing them green in a suite summary would
otherwise attribute lcms2's properties to iccce — ★★ and the engine's
actual behaviour is **the opposite** of D1's: iccce's K-only region is
**zero wide by construction**, lcms2's is **1/16**.

**4. ★★★ A document described as open a defect that had been fixed one
commit earlier.** `TOLERANCES.md` §3.10.12.7 reads *"**Not fixed here,
deliberately**"* and prints `0.617121` / `0.617148` as a measurement
table. **The fix landed in `a05476c`, immediately before `a1bd818`, the
grading commit that filed it as open.** Today the same measurement
converges: near-axis **`6.234231e-7` → `3.330669e-16`** (≈1.5 ULP of 1.0
— noise, not a residual). ★★ **A stale STATUS decays faster than a stale
number, because the event that invalidates it lands in someone else's
commit** — a wrong number invites re-derivation, a wrong status invites a
reader to act on a discharged obligation or to distrust working code.
**Filed as TWO rows** — **NC-265** (history, dated, fixing commit named)
and **NC-264** (what is true now) — so the ledger is right **whatever
state that document is in when next read.**

### What verifying the package added

| | the package | after reading the source |
|---|---|---|
| the leak guards' probe counts | *"192 / 50"*, with **no source in its own evidence tables** for the `50` | ★ **UPGRADED, not rejected** — `passk.rs:3884` documents F8 as *"the 50 chromatic grays"* |
| E5, *"the control that earns E4's tightness (32×)"* | one ratio | ★ **`32.4×` the OBSERVATION; `8.75×` the TOLERANCE** — the same two-denominator trap the package itself flags for E9 (`1577×` vs `448.6×`). **Quote a ratio with its denominator or not at all** |
| the suggested `NC-262` | sweep **and** K re-mapping in one row | ★ **SPLIT.** Different rivals — the sweep's is *pre-feature ink*, the re-mapping's is *`K_out = K_in`* — so different rows. Downstream ids shifted by one |
| `passk.rs`'s wrong clause, its dangling §3.10.8, and two false GWG statements | not mentioned | ★★ **ALL THREE DISCHARGED** *(verified)* — zero `8.6.4.4`, `TOLERANCES.md:3163` is §3.10.8, and the *"GWG's own patch value"* attribution is repaired **inside the emitted report string itself**. ★ **The correction was carried, not deleted** |
| `ROADMAP.md` | not mentioned | ★★★ **It had NO Pass K entry and no black-preservation entry at all** — the **third** instance of a capability shipping with no roadmap record. A retrospective section was added |

### ★★ The absences, filed as rows' equals

**NC-251's separation EQUALS its own observation — ratio `1.0` — so it
can never discriminate**, and it prints `UNGRADED` rather than `BLIND`
only because `BLIND` requires a finite tolerance. ★★★ **So `blind = 0` is
true of the classifier and false of the suite**, demonstrated rather than
suspected for the first time. **NC-254 is `ZERO-SEPARATION` and REPORTED,
so nothing gates it.** **No injection proof for the leak rows** — DL-051
says a passing test is not evidence until one turns it red. **No `--bpc`
+ preservation row anywhere.** **`KMapping::Ratio` has no row because it
has no implementation.** **Eight of the ten swept destinations are graded
by nothing.** And ★★★ **the perceptual cost of preservation is
UNMEASURED** — the ΔE2000 between the preserved and colorimetric answers
on a cross-press pair, which is **the number a caller weighing the policy
actually wants**. That is **NA-012**'s stated cost, registered on the day
the code landed rather than late, as NA-011 was.

### What is explicitly NOT claimed

- **That Pass K improves this project's evidence position.** ★★★ It adds
  volume to the weakest classes. The ground-truth debt is unmoved.
- **That the ten-destination sweep is a row.** ★★ It has **no oracle leg**
  and is `self-comparison`; a `[VERIFIED]` tag on it certifies **when it
  ran**, never its evidence class (DL-053).
- **That the compiled path is covered.** ★★ **No difftest row reaches
  it**; a `crates` test is the only detector, and `iccce bench` cannot
  build the combination at all.
- **That `fmt` or `clippy` passed.** ★ **Neither was run.** The session
  handoff asserts all three gates; **only `cargo test --workspace` (170
  passed) is filed.**
- **That the equal-lightness definition is correct.** ★★★ Nothing can
  establish that — ICC.1 states nothing for it to be right against, and
  **the two published definitions disagree by up to `4.889900e-2`.**
- **That anything is committed or pushed.** ★ This filing fetched
  nothing, authorised nothing, and makes no claim about push state.

---

## 2026-08-18 — the **stale-status** filing (twenty-ninth `SESSION_LOG` entry). **A decision-log entry with TWO dated instances, the second of which arose inside the document that was filing the first one's evidence — and no numeric claim moves at all**

**Filed by `icc-librarian`.** Two jobs: discharge `NUMERIC_CLAIMS.md`
§7.23's newly-owed item 1, and file **DL-062**, drafted by
`icc-conformance` and handed here because the decision log is this
role's. **No row added, changed or invalidated. Next free identifiers
remain `NC-267` and `NA-013`.** `ROADMAP.md` untouched — no Pass moved,
nothing shipped, nothing measured.

### What was verified, and how

★★★ **Nothing below was accepted from either agent's report.** The
operator's dispatch carried an account of the correction; the drafter
carried an account of the commits. **Both were re-checked against live
source**, which is this role's standing rule and which paid twice.

- **The 28 seconds are MEASURED, not reported.** `.git/logs/HEAD` lines
  99–100: `a05476c` at epoch **`1787035205`** (`-0400` ⇒ **02:40:05**),
  `a1bd818` at **`1787035233`** (**02:40:33**). Difference **28**.
  Adjacent commits. *(A reflog file is plain text and reading one is not
  running `git`.)*
- **The fix is in the tree** — `crates/iccce-cmm/src/compiled.rs:218`
  declares `k_preserve` **outside** the grid, `:333` populates it, `:354`
  branches per pixel in `convert`. ★ **That it landed in `a05476c`
  specifically stays `[REPORTED]`**: a reflog line evidences a commit's
  existence, time and subject and **never its contents**, and this role
  has no shell. What is verified is the **tip**.
- **All three of the owed item's *done when* clauses**, adjudicated
  separately against `TOLERANCES.md`: the `pre-a05476c` table header
  (`:3757`, `:3762`); **both carrier phrases surviving only inside
  quotation** (`:3718`, `:3720`, `:3968` — grepped the whole file, not
  spot-checked at the line numbers supplied); and §3.10.12.6's bullet
  now reading *"unmeasured by any row of this suite"* with a dated
  `★ CORRECTED` note at `:3893`.
- **A second verbatim copy in `tools/difftest/README.md`** §25.13.7
  (`:5367`) and §25.13.6 (`:5446`), also corrected. ★ **Found by
  `icc-conformance`, not by me, and named in no file list the evidence
  package carried.** The owed item named one file; the defect was in
  two.
- **One carrier phrase from DL-062's grep list was swept and SURVIVED.**
  *"`KMapping::Ratio` … has no implementation"* is still true:
  `black_preserve.rs:223` declares the variant, `:238` names it, and
  `transform.rs:1097` **refuses** it. ★ A refusal is not an
  implementation. Recorded because **a phrase list that has only ever
  fired is untested in the passing direction.**

### ★★★ The finding, and it has two instances

**DL-062** — *a document that records "X is broken and someone else owns
the fix" has an expiry date set by someone else's commit.* The
conformance role wrote *"not fixed here, deliberately — the remedy
belongs to the engineer"*; the engineer had fixed it **28 seconds**
before that text was committed.

★★ **Why it is not DL-048's stale numeral.** A numeral goes stale when
the world moves and **invites re-derivation the next time anyone
measures** — the correction is triggered by the same act that caused the
drift. A **status** goes stale when **a different role commits in a
different directory**, and **no act triggers the correction**: the fixing
commit is, from the document's side, an unrelated change; the document
is, from the commit's side, not in the diff. ★ The stale text survived
**four subsequent commits** (`a1bd818`, `f3b6b87`, `9dc9d70`,
`60c32dd`, two of them documentation commits) and was found by a
**deliberate currency re-check**. ★★★ **And the polarity is unusual: the
document UNDERSTATED the code.** Almost every guard this project has is
aimed the other way.

★★★ **THE SECOND INSTANCE, and it is why this is a log entry rather than
a section note.** The mechanism **reproduced inside this session, in the
document filing the first instance's evidence, with the roles reversed**:
this librarian read `TOLERANCES.md`, correctly found it stale, filed
§7.23 item 1 as owed — and `icc-conformance` discharged it concurrently,
leaving **`NUMERIC_CLAIMS.md` carrying a stale status claim about another
role's file.** ★★ **Two instances, the second arising while writing up
the first, is a claim about STRUCTURE, not an anecdote** — and **it is
not about haste, because the second happened under full awareness of the
first.** It is structural to how this project divides file ownership:
no role edits another's file, which is a good rule that *guarantees*
every cross-role status claim is a claim about a tree its author cannot
fix.

### ★★★ The constructive half — the defence is in the WRITING, not the CHECKING

Re-checking before quoting is necessary and **insufficient**: the claim
can go stale **between being written and being committed**, which is the
interval both instances died in. A filing that assigns work to another
role must **either re-verify at commit time, or be written to be correct
regardless of the other file's state.**

★★ **The worked example is in this ledger and it held.** **NC-264**
(current: near-axis `6.234231e-7 → 3.330669e-16` across grids 17 and 33)
and **NC-265** (pre-fix `0.617121` / `0.617148`, dated `pre-a05476c`
with the fixing commit named) are **two dated rows, not one edited row**.
Both were correct before the correction landed and are correct after it.
**The rows survived the race; the §7 prose about the rows did not** —
because a row is a claim about a *measurement*, and measurements do not
change under another agent's commit, while a §7 owed item is a claim
about a *state*. ★ **Prefer to discharge a doubt with a row.**

### One thing found by reading that nobody dispatched

★★ **The draft's DL-053 cross-reference was wrong, and where it came
from is the better finding.** The draft glossed DL-053 as *"`[VERIFIED]`
certifies a date, not an evidence class"*; DL-053's own rule
(`ARCHITECTURE.md:5378`) is the **denominator** axis — *a tag certifies
that the measurement happened, never what it RANGED OVER.* ★★★ **But the
gloss traces to this librarian's own Pass K entry above** (`:6063`),
which extended DL-053 to *"certifies when it ran, never its evidence
class"* **without labelling the extension**. The drafter cited
accurately from a source that had already drifted — **DL-048's mechanism
running through two documents.** All three readings are true of
`[VERIFIED]`; only the first is DL-053's. **This log is append-only, so
the correction lives in DL-062, not above.**

### What is explicitly NOT claimed

- **That the compiled path is now covered.** ★★★ **The document is
  corrected; the DEBT IS NOT DISCHARGED.** There is still **no difftest
  row** for the compiled path — §7.23 newly-owed item **4**, which stays
  **OPEN**. What changed is the row's *purpose*: **disclosure →
  regression guard**. Reading item 1's discharge as covering the coverage
  gap is **DL-062's error running the other way**.
- **That any measurement was taken or re-run here.** ★ Every number in
  this entry is quoted from a source named beside it. **No row moves.**
- **That `a05476c`'s contents were read.** ★ `[REPORTED]`, per above.
- **That `fmt` or `clippy` has been run since Pass K.** ★ §7.23's
  newly-owed item 7 stands untouched; this filing ran nothing.
- **That anything was committed or pushed.** ★ This filing edited three
  documents in the working tree, fetched nothing and authorised nothing.
- **That another role's file was edited by this librarian.** ★ It was
  not — here or at the Pass K filing. `TOLERANCES.md` and
  `tools/difftest/README.md` were **read only**.
