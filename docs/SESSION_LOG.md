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
