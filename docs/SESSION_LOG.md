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
