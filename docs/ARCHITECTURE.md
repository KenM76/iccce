# iccce — architecture

Written before any code, so the shape is a decision rather than a
residue. Everything here is revisable; what is not revisable without
discussion is marked **INVARIANT**.

---

## 1. Crate layout

```
iccce/
  crates/
    iccce-profile/     parse and represent an ICC profile. No maths.
    iccce-color/       CIE colorimetry. No ICC, no I/O.
    iccce-cmm/         transforms: build them, evaluate them, cache them.
    iccce-cli/         a scriptable shell for inspection and conversion.
    iccce-measure/     CGATS/IT8.7 measurement files. No ICC, no maths.
  tools/
    difftest/          out-of-tree differential oracle against lcms2.
    gen-profiles/      synthetic profile generator for fixtures.
  fixtures/
    synthetic/         profiles this project authored, byte by byte.
    reference/         rights-cleared real profiles (see LEGAL.md §3).
```

**Five crates.** The workspace manifest is the authority for membership
and it lists all five *(verified 2026-08-12 — `Cargo.toml` read: its
header block says "Five crates, per docs/ARCHITECTURE.md §1" and
`[workspace] members` enumerates them)*. **Three of the five —
`iccce-color`, `iccce-profile`, `iccce-measure` — have an empty
`[dependencies]` section, and that emptiness is the architecture rather
than a coincidence.**

### `iccce-measure` — added 2026-08-12, and why it exists before its consumer does

**INVARIANT: `iccce-measure` contains no ICC and no colour maths.** It
reads the text a spectrophotometer emits — a header of `KEYWORD value`
properties, a declared field list, a table of measured patches — and
returns them as declared. Deciding that a column named `LAB_L` means
CIE `L*` under some observer is the **consumer's** act; a spectral→XYZ
integration additionally needs colour-matching functions this project
has not sourced.

It is **Pass 10 pre-work** (profile creation), landed at commit
**`2a2d616`** on the operator's authorisation of 2026-08-12, and its
position in the tree is an argument rather than a convenience:

- **A measurement file is not a profile.** Keeping the reader out of
  `iccce-profile` means a future profiler *and* a future measurement
  tool share one reader, and means **this crate's tests never need an
  ICC fixture** — its eight tests are text in, structure out.
- **It is the half of a profiler that needs no hardware.** `ROADMAP.md`
  Pass 10's precondition — *a profiler whose output cannot be validated
  against physical measurement is rule 1 in its worst form* — binds the
  **fitting** half. Parsing has no such block, so it could be built
  today and was.
- **The parser reports; it does not repair** (§3 invariant 2, here
  applied to measurement data rather than to ICC bytes). A
  `NUMBER_OF_FIELDS` that disagrees with the `DATA_FORMAT` block is
  disclosed as an `Issue` and the file stays readable; **nothing is
  silently corrected**, because the disagreement is exactly the thing a
  later stage would otherwise attribute to the instrument.

**Lineage, which is a licence fact and not a footnote:** structure and
keyword vocabulary derive from **lcms2's `cmscgats.c`**, which is MIT —
the same licence as this project, so it is a permitted lineage on the
same terms as every other implementation-derived piece of work here
(`impl_crosscheck` tier). **Argyll CMS is AGPL-3.0 and must never be
read or cited for this work.** It is the most tempting reference in this
subject area and it is copyleft; the hazard is recorded at the crate
site as well as here because the temptation recurs. CGATS.17 itself is
paywalled and **is not sourced** — where lcms2's reader is more
permissive than the standard may be, this follows lcms2 and says so.

### Why `profile` and `color` are separate from `cmm`

**INVARIANT: `iccce-color` depends on nothing.** It is CIE mathematics —
XYZ, Lab, chromatic adaptation, ΔE. It must be usable, and testable,
with no profile anywhere near it, because its correctness is checkable
against published reference values and that check should not require
constructing an ICC file.

**INVARIANT: `iccce-profile` performs no colour maths.** It parses bytes
into a faithful representation of what the file says, including things
that are wrong. A parser that silently corrects a malformed tag is a
parser that hides the malformation from the layer that could report it.

`iccce-cmm` is where the two meet, and where every approximation lives.

### Why a CLI exists from the start

Same reasoning as `pdfce`'s: it is the surface that makes the library
verifiable without a GUI, and it is where a differential test drives the
engine. `iccce-cli transform --in a.icc --out b.icc --intent perceptual`
producing numbers a script can diff is worth more than any amount of
internal assertion.

---

## 2. The pipeline, and where the hard parts are

```
  bytes ──▶ Profile ──▶ TransformPlan ──▶ CompiledTransform ──▶ pixels
            (parse)     (choose the      (flatten to a          (evaluate)
                         path)            fast form)
```

**Choosing the path is the subtle step.** For a source→destination
conversion at a given intent, the CMM must decide which tags to use, and
the rules are not obvious:

- A v4 profile may have `A2B0`/`A2B1`/`A2B2` for the three
  non-absolute intents; absolute is media-relative plus a white-point
  adjustment, **not a fourth table**.
- A matrix/TRC profile has no A2B at all and is handled analytically.
- A profile may lack the tag for the requested intent, and the fallback
  order is specified — implement the specified one, do not invent a
  reasonable one.
- **v2 and v4 differ in ways that produce plausible wrong colour**, most
  notoriously in `lut16Type` PCS encoding for Lab. This is the single
  richest source of CMM bugs and deserves its own RAG section and its own
  fixtures.

**Compiling is where the speed is.** A page-sized raster is tens of
millions of pixels; walking the profile structure per pixel is not
viable. The plan flattens to a form that evaluates without allocation —
and the flattening is an approximation whose error must be measurable and
documented, not assumed negligible.

---

## 3. Standing invariants

1. **`iccce-color` has no dependencies.** See above.
2. **The parser does not repair.** It reports.
3. **Every approximation is named and measured.** A CMM is a pile of
   interpolations; the difference between an engineering choice and a bug
   is whether the error is stated. Any place the engine departs from
   exact colorimetry carries a doc comment saying what the departure is
   and what it costs in ΔE.
4. **No `unsafe`** without a written justification and a named
   alternative that was rejected. Colour maths does not need it; SIMD
   might, later, and that is the conversation to have then.
5. **MIT**, and every dependency permissive. Same posture as `pdfce`;
   see `LEGAL.md`.
6. **Numeric tests use published reference values**, not values the
   engine produced on a day it seemed to work. A regression test whose
   expected value came from the code under test only detects change, not
   error.

---

## 4. How `pdfce` consumes it

Through a thin bridge crate *in `pdfce`*, not here. `iccce` must not
know what a PDF is.

The bridge maps `/ICCBased` streams to `iccce_profile::Profile`,
`/Separation` and `/DeviceN` tint transforms to named-colour lookups
where a profile provides them, and PDF/X `/OutputIntent` to a destination
profile. That mapping is PDF knowledge and belongs on the PDF side of the
line.

**Consequence worth stating:** `iccce` must be able to parse a profile
from a byte slice with no filesystem, because in a PDF it arrives as a
stream.

---

## 5. Decision log

Entries are dated, appended, and **never rewritten** — a reversed
decision gets a new entry that references the old one by its `DL-nnn`
identifier. Each entry states the decision, why, the evidence it rests
on, what follows from it, and what would reopen it. An entry with no
"revisit if" is a decision nobody can ever revisit on purpose.

---

### DL-001 — the lcms2 oracle is pinned by **commit hash**, and its GPL plugins are excluded in three independent layers

**Date:** 2026-08-11 · **Decided by:** `icc-conformance` · **Filed by:**
`icc-librarian`

**Decision.** `tools/difftest/lcms2.pin` names tag `lcms2.19.1`, but the
**pin is the commit `21c582a594fe5279f90c0b93437c398f93bf62b0`**;
`fetch-lcms2.sh` re-derives `git rev-parse HEAD` after cloning and exits
non-zero on mismatch. `plugins/fast_float` and `plugins/threaded` are
never built.

**Why the hash and not the tag.** `lcms2.19.1` is a **lightweight** tag —
a mutable ref with no tagger, no signature, and no tag object. Upstream
can move it. If it moved, `git clone --branch lcms2.19.1` would hand a
future session a *different* tree under the same name, and every result
previously attributed to "verified against lcms2 2.19.1" would become
unreproducible **with nothing in the record showing that it had changed.**
That is the specific failure this project cannot absorb: the oracle's
identity is what makes every cross-check claim falsifiable.

**Why the plugins are off — two reasons, and the second survives the
first.** (1) Both are **GPL-3.0-or-later**, stated by upstream itself in
`plugins/README.1ST`, in a tree whose top-level `LICENSE` is verbatim
MIT. A licence badge would have said "MIT" and been incomplete. (2) Even
if `fast_float` were MIT it would stay off: it substitutes an approximate
floating-point pipeline for lcms2's careful one, and **an oracle must be
the reference implementation's most accurate path** — against an
approximate oracle every disagreement is ambiguous between our error and
the plugin's, which is the one thing an oracle exists to prevent.

**Insulation is three layers, deliberately, because one layer is a
setting somebody can flip:** no crate links lcms2 at all (subprocess
only); both plugins are explicitly `OFF` at configure time even though
upstream already defaults them off; and `tools/difftest/vendor/` is
git-ignored, so GPL source never enters this repository's history —
a matter of fact rather than of a build flag.

**Evidence.** `docs/LEGAL.md` §4.1–§4.3 (licence text transcribed from
the cloned tree at the pinned commit, not from GitHub's badge);
`tools/difftest/README.md` §2–§3.

**Consequence.** Moving the pin is a **licence event, not a version
bump** — `LEGAL.md` §4.3 sets the re-verification procedure, and it
requires a new dated subsection rather than an edit to the old one.

**Revisit if:** upstream publishes annotated/signed tags (the hash pin
stays regardless — a signed tag would be additional evidence, not a
replacement); or a difftest genuinely needs a plugin, in which case the
accuracy objection must be answered before the licence one.

---

### DL-002 — the standards corpus is built from cross-verified permissive **code**, and carries **no `primary_spec` tier**, until the operator downloads ICC.1 by hand

**Date:** 2026-08-11 · **Decided by:** `icc-spec-librarian` · **Filed
by:** `icc-librarian`

**Decision.** No ICC-hosted specification PDF was retrieved. Tier 1 of
`D:\Dev\Rag-Specialized\ICC_Spec\` (21 files) is built by cross-verifying
**two independent, permissively licensed codebases** — ICC's own
`icProfileHeader.h` from `DemoIccMAX` (BSD-3-Clause) and `lcms2.h` (MIT)
— and **every file says so in its frontmatter**. Doc comments in
`iccce-profile` cite **corpus filenames, never ICC.1 clause numbers.**

**Why.** color.org's Terms of Service (effective 2026-01-01) prohibit
*"using any robot, spider, or other automated device to access the
Services for any purpose, including monitoring, copying, or training
artificial intelligence or machine learning models, without prior written
consent"*. An agent fetching the PDF is squarely inside that clause,
which **names AI/ML explicitly** — not a strained reading. Retrieval
stopped at the three pages needed to determine the terms.

**The disagreement is part of the record.** `www.color.org/robots.txt` —
the machine-readable statement of what automated agents may do —
disallows only `/accounts/`, `/dashboard/`, `/documents/`, and does not
disallow the specification index; `archive.color.org` serves no
`robots.txt` at all. The site's machine-readable permission and its prose
contract point opposite ways. **The prose contract binds, so the
restrictive reading was taken**, and the conflict is written down rather
than resolved silently in either direction.

**Consequence, stated plainly by the librarian who took the decision: a
parser is defensible on this evidence and a validator is not.** A C
header encodes signatures, offsets and enumerations exactly and encodes
**prose requirements not at all**. Which tags a profile class *requires*,
what a decoder *shall* do with an out-of-range value, the
rendering-intent semantics, and the interpolation rule between LUT grid
points are **not recoverable by this route** and are recorded as gaps
(ambiguity register A1–A30), not guessed. `iccce-profile` therefore holds
opaque what the corpus marks NOT SOURCED (e.g. `Header::attributes`)
rather than inventing a reading, and asserts no conformance requirement.

**Evidence.** `docs/LEGAL.md` §2.1 (ToS transcribed verbatim, all sources
HTTP 200 on 2026-08-11), §2.2 (the blocker and the three routes out),
§2.3 (the two sources and their standing);
`crates/iccce-profile/src/lib.rs` module doc.

**Revisit if:** Ken downloads `ICC.1-2022-05.pdf` in a browser to
`ICC_Spec\_sources\` — a human retrieval, entirely outside the robot
clause, and the cheap answer (~2 minutes). That clears roughly 15
UNVERIFIED rows in the ambiguity register and the whole required/optional
tag column. Or: written consent from ICC for automated access. **Until
then, no ICC-hosted document is a source for this corpus, and no claim in
this project may cite an ICC.1 clause number.**

---

### DL-003 — duplicate tag signatures: the table keeps **both**, consumers take the **first**, and the duplicate is a reported malformation

**Date:** 2026-08-11 · **Decided by:** `icc-engineer`, on corpus
ambiguity **A13** · **Filed by:** `icc-librarian`

**Decision.** When one tag table contains two entries with the same
signature, `iccce-profile` stores **both** entries verbatim, in file
order, and pushes a `Malformation::DuplicateTagSignature { first_index,
dup_index, sig }`. The recorded convention for *consumers* is **take the
first**.

**Why this needed a decision at all.** Ambiguity register **A13** grades
the specification as **SILENT** on whether duplicate signatures are
legal, and the case is **observed in the wild**. A silence is not a
permission and not a prohibition, so a parser has to choose — and the
choice must be visible, because all three plausible choices (first, last,
refuse) produce a profile that parses.

**Why *keep both*.** Invariant §3.2: the parser reports, it does not
repair. Dropping the duplicate at parse time would erase the only
evidence that the file was malformed, from the only layer positioned to
disclose it.

**Why *first* rather than last.** File order is the only ordering the
format gives, and "first wins" is the reading under which a truncated or
partially-read table yields the same answer as a fully-read one. This is
a **stated convention, not a specification requirement** — it is exactly
the sort of thing that must be written down instead of implied by an
index into a vector.

**Evidence.** `crates/iccce-profile/src/tag_table.rs` (the duplicate
check, and `TagEntry` as a plain `(offset, size)` view so shared tag data
costs nothing); `crates/iccce-profile/src/diag.rs:117–119, 152–155`
(the diagnostic names the choice: *"consumers take the first; recorded
choice A13"*);
`ICC_Spec\icc\icc__ref__ambiguity_register.md` row A13.

**Revisit if:** the ICC.1 text (DL-002) turns out to speak to duplicates
after all; or a real profile is found whose *intended* tag is the second
of a duplicated pair, which would make "first wins" the wrong reading of
the wild. Either reopens this as a new entry, not an edit.

---

### DL-004 — the **1.0 ΔE2000** perceptual anchor is a conservative **design choice**, not an empirical fact

**Date:** 2026-08-11 · **Decided by:** `icc-conformance` · **Filed by:**
`icc-librarian`

**Decision.** `docs/TOLERANCES.md` §2 records 1.0 ΔE2000 as the yardstick
most later tolerances will be expressed as a fraction of, and marks it
**⚠ PROVISIONAL — citation not yet verified from primary text.**

**Why it is not a fact.** CIEDE2000 is standardised (CIE 142-2001, then
ISO/CIE 11664-6) and those documents **define the formula and its
reference conditions — they do not declare a just-noticeable-difference
value.** The "1.0 = JND" figure is industry convention layered on top of
the standard. The measured psychophysical literature does not report a
single number: 50:50 perceptibility thresholds for ΔE00 span roughly
**0.8 to 2.3** depending on stimulus size, background, illuminant, edge
contact and observer population. Neither CIE 142-2001 nor ISO/CIE
11664-6 has been obtained (both paywalled).

**Why 1.0 anyway.** It sits at the **conservative** end of the measured
range, and a conservative anchor is the right kind of wrong for an engine
tolerance: holding ourselves to 1.0 when the true threshold is 2.0 spends
margin we did not have to spend, whereas picking 2.0 when the true
threshold is 1.0 ships visible error and calls it conformance.

**Consequence.** Any tolerance derived from the anchor **inherits its
⚠**. A tolerance quoted in ΔE00 without the parametric factors
(k_L, k_C, k_H = 1:1:1 only under the standard's reference conditions) is
underspecified, and those conditions are currently **unverified
recollection** in `TOLERANCES.md` §2 and flagged as such. Separately: the
anchor is **irrelevant** to the ΔE2000 *implementation* test, which is an
arithmetic-agreement check (~1×10⁻⁴) against the Sharma et al. (2005)
34-pair table — real ground truth, already transcribed into the corpus.

**Evidence.** `docs/TOLERANCES.md` §2 (including the Mahy et al. 1994
bibliographic record, whose 2.3 figure is recorded as *attributed, not
read out of the paper*); `ICC_Spec\cie\cie__ref__delta_e.md`.

**Revisit if:** `icc-spec-librarian` obtains CIE 142-2001 / ISO/CIE
11664-6 and a primary psychophysical source with its stimulus and viewing
conditions stated. Any change to the anchor is a **new entry here plus an
append-only row in `TOLERANCES.md` §4** — never an edit to the number in
place.

---

### DL-005 — v2 legacy Lab encoding is tested by **exact-value invariants**, not by ΔE

**Date:** 2026-08-11 · **Adopted from** `icc-spec-librarian`'s corpus
finding **D1** · **Filed by:** `icc-librarian`

**Decision.** Correctness of the v2 `lut16Type` legacy Lab encoding
(`0x0000 → 0xFF00` = 0 → 100 for `L*`, versus v4's `0x0000 → 0xFFFF`)
will be asserted with **exact encode/decode invariants on integer
values**, not with a ΔE tolerance. No such test exists yet — Pass 4 is
where it lands; this entry fixes the method before the test is written,
because the natural instinct will be to grade it in ΔE like everything
else.

**Why.** Getting the encoding wrong costs **≈0.3–0.5 ΔE** (a uniform
~0.39 % error on neutrals; the ratio `65535/65280 = 1.00390625` is
exactly the v2→v4 rescale). That is **below the 1.0 anchor of DL-004** —
so a ΔE-graded test **passes while the encoding is wrong**, and the
defect ships as a slight uniform darkening of neutrals that looks like
nothing at all. `ARCHITECTURE.md` §2 already names v2/v4 Lab encoding as
the single richest source of CMM bugs; this is the mechanism by which it
stays hidden.

**Related trap, recorded with it:** decoding a **v4** profile with the
**v2** rule reads `L*` 0.39 % high and lets values above `0xFF00` decode
to `L* > 100`, outside the legal Lab domain — which can produce NaN
downstream in the `Lab→XYZ` cube-root inverse if the implementation does
not clamp. A NaN is at least loud; the 0.4 % cast is not.

**Consequence.** `TOLERANCES.md` §3.4 marks both Lab-encoding rows
**ground truth** rather than cross-check, deliberately: an encoding
error of exactly this kind would be **shared** by any implementation that
read the clause the same way, so "lcms2 agrees" is least reassuring
precisely here. These must be settled from specification text — which
DL-002 says this project does not yet have.

**Evidence.** `ICC_Spec\icc\icc__ref__v2_v4_divergence.md` §D1
(cross-verified against two independent codebases); `docs/TOLERANCES.md`
§3.4 and its note.

**Revisit if:** a real v2 profile is found whose encoder used the v4
range (which would make the rule a heuristic rather than an invariant),
or ICC.1 text arrives and contradicts the reconstructed rule.

---

### DL-006 — **DL-002's revisit condition has fired**: `ICC.1-2022-05.pdf` is in the corpus `_sources`. This entry records the *trigger*, not the corpus decision that follows from it

**Date:** 2026-08-11 (later the same day than DL-001…DL-005) · **Trigger
by:** Ken (operator), by hand · **Filed by:** `icc-librarian`

**What happened.** DL-002 named exactly one cheap route out of the
no-`primary_spec`-tier position: *"Ken downloads `ICC.1-2022-05.pdf` in a
browser to `ICC_Spec\_sources\`."* That has been done.

**Evidence, and its exact strength.** `icc-librarian` enumerated
`D:\Dev\Rag-Specialized\ICC_Spec\_sources\` directly and the directory
now contains `ICC.1-2022-05.pdf` alongside its `README.md` *(verified —
directory listed by this librarian, this session; previously that
directory held only `README.md`)*. The **retrieval time of 11:12 on
2026-08-11, and the fact that it was a manual browser download, are
reported** by the dispatching engineer on Ken's word — not measured here.
No agent in this project has verified the file's size, hash, page count
or that it is the document its filename claims. Those are the
`icc-spec-librarian`'s to establish on ingest.

**What this entry does NOT decide.** Three things, deliberately left
open, because they belong to the agent that owns the corpus:

1. **Whether the corpus gains a `primary_spec` tier**, and which files
   are re-sourced against it.
2. **Which of the ~15 UNVERIFIED ambiguity-register rows the document
   actually settles.** "It unblocks roughly 15 rows" was always an
   estimate of what the document *should* contain; the count is a
   prediction until somebody reads it.
3. **Whether DL-002's prohibition — *no claim in this project may cite an
   ICC.1 clause number* — is lifted, and on what terms.** The prohibition
   was never about a file's presence on a disk; it was about there being
   no sourced clause text to cite. A PDF that nobody has ingested is not
   yet a citable source. **Until `icc-spec-librarian` files the successor
   entry, treat DL-002's citation rule as still standing.**

**Why it is filed at all, given that it decides nothing.** Because
DL-002 is append-only and reads, in isolation, as a live prohibition with
no expiry. A future session reading only §5 would have no way to know its
condition had been met. This entry is the pointer that makes the
successor findable.

**Status of the ingest.** `icc-spec-librarian` was dispatched in parallel
with this filing to ingest the document, and **owns `LEGAL.md` §2 this
session**; `icc-librarian` did not touch the corpus or `LEGAL.md`, and
**has not verified that the ingest has landed or succeeded.** A later
session must not read this entry as evidence that it did.

**Revisit if:** — not applicable; this entry is a record of an event.
The decision it anticipates is `icc-spec-librarian`'s, and gets its own
`DL-nnn`.

---

### DL-007 — **HDR is in scope**: BT.2100 PQ/HLG transfer functions and BT.2020/2100 primaries become a planned Pass

**Date:** 2026-08-11 · **Decided by:** Ken (operator) · **Filed by:**
`icc-librarian`

**Decision.** Open question **(b)** of `ROADMAP.md` — *"How far into HDR?
BT.2100 and PQ/HLG are a real body of work and only matter if something
needs them"* — is answered **yes**. HDR transfer functions and wide-gamut
primaries are planned work, filed as **Pass 9** in `ROADMAP.md`.

**What the operator actually said, verbatim, and what is inference.**
The engineer put three open scope calls to Ken as a numbered list —
*"(1) download the ICC.1 PDF; (2) the open scope calls: HDR depth (b),
profile creator (c — currently a firm no), crates.io (d)"* — and Ken
replied in full: **"1 is done. 2. do all."**

That is the whole of the operator's statement on the subject. **"Do all"
is read here as *adopt all three of (b), (c), (d)*, and that reading is
the engineer's interpretation, recorded as an interpretation and not as
the operator's words.** It is the only reading the sentence plausibly
carries in context, but it is still a reading. Specifically, the operator
supplied **no** scope depth, **no** priority, **no** schedule, and **no**
per-item rationale, and none should be attributed to him. Everything
below about *how far* into HDR is the project's own planning, revisable
without going back to Ken.

**Why this is worth having as a decision rather than a to-do.** Because
"HDR" is not one thing, and the boundary is the substance of the answer.
The Pass as planned covers the **transfer functions and primaries** — the
colorimetry — and does **not** commit the project to tone mapping,
gamut-mapping algorithms, dynamic metadata (ST 2094 and relatives), or
display-referred rendering intent invention. Those are separate calls
that this entry does not make.

**The conceptual hazard, named now so Pass 9 does not discover it late.**
ICC's PCS is a media-relative, reflective-print-derived space; PQ is an
**absolute** encoding tied to real luminance in cd/m², and HLG is
**scene-referred** with a display-dependent OOTF. Mapping either onto an
ICC PCS is not a change of curve — it involves a stated choice about
reference luminance and about what "white" means. **That choice is an
approximation in the sense of project rule 4 and must be named and
measured**, not absorbed into a helper function. This paragraph is a
statement of the shape of the problem; it is **not** sourced from the
ITU-R documents, which nobody in this project has read.

**Corpus dependency, and the trap to avoid repeating.** Pass 9 cannot
start until Tier 3 of the corpus holds the relevant ITU-R
recommendations. Those are **reported** (by the dispatching engineer) to
be freely downloadable from `itu.int`, and that is described as a
legitimate retrieval route. **That is a claim about a third party's
terms, and it gets checked the same way color.org's were** — by reading
ITU's actual terms of use before an agent fetches anything, not by
inferring permission from the fact that the file is free. DL-002 exists
precisely because a site's machine-readable permission and its prose
contract pointed opposite ways; assuming "free download" means
"automated retrieval permitted" is the same mistake with a different
domain name. **`icc-spec-librarian` owns that determination.**

**Consequence.** `ROADMAP.md` gains Pass 9. No existing Pass is
renumbered (see the numbering note in that file: Pass 9's *dependency*
position is after Pass 7 and independent of Pass 8, so its number is
filing order, not schedule order). Rule 2 applies with full force —
**not one PQ or HLG constant may be written from memory**; every
coefficient comes from a sourced corpus file with the document named.

**Revisit if:** the ITU-R documents turn out not to be retrievable on
terms this project accepts (which would stall the Pass, not cancel it);
or the scope boundary above needs widening to tone mapping or dynamic
metadata, which is a **new** entry and a new operator conversation, not
an expansion of this one.

---

### DL-008 — **profile *creation* moves from "out of scope, deliberately" to future scope**, by operator decision. This entry reverses a stated position; it does not erase it

**Date:** 2026-08-11 · **Decided by:** Ken (operator) · **Filed by:**
`icc-librarian`

**This is the significant entry of the three.** The other two open
questions were unanswered; this one had a stated answer, in writing, with
a rationale, and the operator has reversed it.

**The position being reversed, quoted from the live source.**
`README.md` §Scope, under *"Out of scope, deliberately — say no now
rather than drifting"*:

> - **Profile *creation*** from measurement data. That is a profiler, a
>   different product, and it needs measurement hardware to validate.

and `NEXT_SESSION.md` §"Decisions already made — do not re-litigate":

> - **No profile creation, no iccMAX execution, no display calibration.**

**Note on where the old position lived.** It was **not** a `DL-nnn` entry
— the decision log did not exist when the scope was written, and
`DL-001…DL-005` do not touch profile creation. The firm no lived in
`README.md` and in `NEXT_SESSION.md`'s carried-decisions list. This entry
therefore references those two documents rather than a prior log entry,
and that is stated so nobody later hunts for a `DL-00n` that was never
written.

**Decision.** Profile creation is **future scope, not out of scope**.
Filed as **Pass 10** in `ROADMAP.md`, sized and planned when reached,
positioned after the `pdfce` bridge. `README.md` has been edited to match
— see "Consequence" below.

**What the operator actually said.** As in DL-007: the engineer listed
*"profile creator (c — currently a firm no)"* among three items and Ken
replied **"1 is done. 2. do all."** He said **nothing** about the
validation-hardware rationale, nothing about timing, and nothing about
what a profiler in this project would encompass. **The reading that "do
all" reverses the firm no is the engineer's interpretation**, labelled as
such here. It is a well-founded reading — item (c) was presented to him
*with* its "currently a firm no" attached, so the reversal is what
adopting it means — but the operator did not himself say "reverse the
no", and this record does not put those words in his mouth.

**The old rationale is not withdrawn, and this is the load-bearing part
of the entry.** "It needs measurement hardware to validate" was not a
statement of disinterest that an operator's yes can dissolve. It is an
engineering fact, and it **survives the reversal intact as an open
problem that Pass 10 must solve before it ships anything**:

> **A profiler whose output cannot be validated against physical
> measurement is precisely the wrong-answer-looks-right trap of project
> rule 1, in its worst form.** A parser can be checked against bytes and a
> transform against a reference implementation, but a profile *built from
> measurement data* is a claim about a physical device, and no amount of
> internal consistency, self-round-tripping or agreement with lcms2 can
> tell you whether the profile describes the printer. lcms2 is useless as
> an oracle here — it can confirm that a profile we wrote is *parseable
> and self-consistent*, which is exactly the reassurance that a wrong
> profile would also produce. Round-tripping a profile through its own
> inverse is the canonical version of a test whose expected value came
> from the code under test (invariant §3.6).

**Consequently, Pass 10 carries a hard precondition, recorded now rather
than discovered later:** before any profile-creation work is called
correct, the project must state **how its output will be validated**, and
that statement must name a source of ground truth that is not iccce.
Candidate routes, none chosen and none investigated as of this entry:
published characterisation datasets with both measurement data and a
reference profile (so the profile can be built and compared to a
third-party build); an instrument (spectrophotometer) Ken has or acquires;
or a deliberately reduced scope — e.g. **synthetic** matrix/TRC profile
*writing* for fixtures, which needs no measurement at all and is a
genuinely different and much smaller problem. **That last one is worth
separating explicitly**: `tools/gen-profiles/` in `ARCHITECTURE.md` §1
already implies writing profile bytes for test fixtures, and that has
never been out of scope. "Profile creation from measurement data" is the
thing that was refused; writing a synthetic profile whose intended
contents are known by construction is not the same activity and should
not be conflated with it when Pass 10 is sized.

**Consequence — documents changed by this entry.** `README.md`'s
"Out of scope" list no longer states profile creation as a firm no; it
now records that the item **moved from out-of-scope to future scope by
operator decision on 2026-08-11**, points here, and carries the
validation-hardware problem forward as the open engineering question. The
edit was made because **a scope statement that contradicts the decision
log is the specific drift this librarian exists to prevent** — but the
honesty requirement cuts both ways, so the README says the scope
*changed* rather than reading as though profile creation had always been
planned. `NEXT_SESSION.md`'s carried-decisions list is likewise corrected
rather than silently dropped.

**Revisit if:** the validation problem above proves unsolvable on
acceptable terms — in which case the right outcome is a **new** entry
narrowing Pass 10 (e.g. to synthetic profile writing only), not a quiet
lapse of the Pass; or the operator scopes the profiler further, which is
also a new entry.

---

### DL-009 — **publication to crates.io is intended**. The intent is standing; the publish *act* is not authorised by it

**Date:** 2026-08-11 · **Decided by:** Ken (operator) · **Filed by:**
`icc-librarian`

**Decision.** Open question **(d)** — *"Should `iccce` be published to
crates.io?"* — is answered **yes**. Publication is the intended end
state, and the project should be built so that publishing is a small step
rather than a scramble.

**What the operator actually said**, again in full: **"1 is done. 2. do
all."** As in DL-007 and DL-008, the mapping from "do all" onto a yes for
(d) is **the engineer's interpretation**, recorded as one. The operator
set no date, no version, and no crate-splitting policy.

**The standing constraint is unchanged, and intent does not weaken it.**
Project rule 9 and `ARCHITECTURE.md`'s posture both hold: **nothing may
be pushed, tagged, released or published without an explicit current
go-ahead from Ken at the time.** *"We decided in August that we'd publish
eventually"* is **not** that go-ahead. This distinction is the entire
reason the entry says "intent" rather than "approval": an intent recorded
in a document is exactly the artefact a future agent could mis-read as
standing authorisation for a side-effecting act, and it is being written
down here in the same breath as its own limit.

**What follows in practice — hygiene that gets cheaper the earlier it is
done.** Recorded in `ROADMAP.md` under *Publication*; the substance:

- **Crate naming is now load-bearing.** Four published crates
  (`iccce-color`, `iccce-profile`, `iccce-cmm`, `iccce-cli`) plus the
  facade name `iccce` need to be available on crates.io, and **whether
  they are has not been checked by anyone** — a name squatted by an
  unrelated crate is discovered best *before* the API is public. Note
  also that crates.io names are effectively permanent and cannot be
  reused after a yank.
- **Manifest metadata** — `description`, `license = "MIT"`, `repository`,
  `keywords`, `categories`, `readme`, `rust-version` — must be complete
  and *true* on every publishable crate. The `repository` field currently
  declares `https://github.com/KenM76/iccce`; **whether that remote
  exists has still never been checked** (see the (a) annotation in
  `ROADMAP.md`), and publishing a manifest that points at a 404 is a
  small, avoidable embarrassment.
- **`THIRD_PARTY_LICENSES.md`, generated by `cargo-about`, before the
  first publish.** This is the sibling project's pattern and it matters
  more here than usual because of DL-001: lcms2 sits in this repository's
  workflow while **not** being a dependency, and the licence story is
  therefore "MIT crates, plus an out-of-tree GPL-plugin-excluded oracle
  that is never linked." A generated manifest of actual dependency
  licences is what keeps a reader from having to take that on trust.
- **The oracle must stay out of the published artefact.**
  `tools/difftest` is deliberately not a workspace member (Pass 0), and
  `vendor/` is git-ignored, so lcms2 source cannot reach a `.crate` file.
  That property was engineered for licence insulation; it now also serves
  publication, and **it must survive any future workspace
  reorganisation** — a Pass that "tidies" difftest into the workspace
  would silently undo both.
- **Publishing sets an API-stability expectation** that Passes 1–10 have
  not earned yet. Nothing here argues for publishing early; if anything,
  the correct reading is that the first publish should follow a Pass
  whose numeric claims are on the record in `NUMERIC_CLAIMS.md`, because
  the ledger is what a stranger would need in order to trust the crate.

**Interaction with open question (a).** A yes on (d) settles (a) in
practice: crate *source* becomes publicly readable on crates.io at the
first publish regardless of what the git remote does, so the project
should be written as public-facing from now on. It does **not** establish
that the GitHub remote exists or is public — still unverified, still not
something any agent here has checked.

**Revisit if:** a name collision on crates.io forces a rename (a
publication-mechanics decision, but one that touches every doc); or the
operator narrows publication to a subset of the crates — publishing
`iccce-color` alone is a coherent position, since it depends on nothing
and its correctness is checkable against published reference values.

---

### DL-010 — the Lab `f(t)` breakpoint uses the **exact rational** form. This is iccce's **first stated deviation from normative specification text**, and it is named and bounded rather than absorbed

**Date:** 2026-08-11 (Pass 1) · **Decided by:** `icc-engineer`, on corpus
ambiguity **A11** as resolved by the ICC.1:2022 ingest · **Filed by:**
`icc-librarian`

**Decision.** `crates/iccce-color/src/lab.rs` implements the CIE Lab
transfer function with the exact rationals — `f` breaks at
`(24/116)³` with linear branch `(841/108)·t + 16/116`, and `f⁻¹` breaks
at `24/116` with linear branch `(108/841)·(t − 16/116)`. **ICC.1:2022's
own normative text writes that breakpoint as the decimal `0,008 856`.**
iccce does not use the decimal, and this entry exists so that the
departure is a decision on the record rather than a constant somebody
chose.

**Why this needed an entry now, when A11 has existed since Pass 0.**
Because the ingest *changed what kind of choice it is*. Before, A11 was a
**source conflict** — lcms2 said one thing, ICC's reference code said
another, and picking either was picking between implementations. After
the ingest it is a **deviation from a printed normative constant**, which
is a different and heavier thing to do silently.

**What ICC.1:2022 actually says, and why it is not the last word.** Two
sentences, both verbatim in the corpus and both load-bearing:

> **6.4:** *"Conversions between the PCSXYZ and PCSLAB encodings **shall
> use the equations of the form specified in ISO 13655**."*

> **6.4:** *"In order to calculate PCSLAB values from negative PCSXYZ
> values, the straight line portion of the PCSLAB colour component
> transfer function below **0,008 856** shall be extended linearly below
> zero."*

So ICC.1 **delegates** the definition of `f(t)` to **ISO 13655** and does
not define it — while writing the decimal in a normative sentence of its
own. **ISO 13655 is the actual authority, it is paywalled, and it has NOT
been obtained.** The corpus grades A11 **RESOLVED-as-DELEGATED**. The
deviation is therefore from ICC.1's *printed constant*, and it is
**unknown** whether it is a deviation from the delegated authority at
all — which is stated here rather than resolved, because nobody has read
ISO 13655.

**Why deviate.** The rational form makes `f` and `f⁻¹` **exact mutual
inverses at the breakpoint**. The decimal form provably cannot be, and
**ICC's own reference code demonstrates the resulting internal
inconsistency** — its forward and inverse thresholds disagree by
~4×10⁻⁷, i.e. the reference implementation does not round-trip through
its own function. The rational is also lcms2's form and, per the corpus,
the modern CIE 15 / ISO 11664-4 statement (that clause likewise paywalled
and unsourced). Choosing self-consistency over a four-significant-figure
decimal is defensible; choosing it *without saying so* is what rule 4
forbids.

**The cost, and the exact status of the cost — this is the part most
likely to be mis-restated.** Versus the decimal-threshold form the
difference is **~10⁻⁷ in `f`, therefore ~10⁻⁵ in `L*`** — roughly five
orders of magnitude below the (⚠ provisional, DL-004) 1.0 ΔE2000
perceptibility anchor. **That figure is an analytic bound taken from the
standards corpus. iccce has not measured it.** No test in this repository
computes the difference between the two forms. Anyone quoting it writes
*"bounded analytically at ~10⁻⁵, unmeasured"*, never *"measured at
~10⁻⁵"*. It can matter only to bit-exact round-trip comparison against an
implementation using the other form; it cannot affect colour.

**What the choice buys, concretely, and it is not nothing.** `Y = 0` maps
to `L* = 0` **exactly** — an identity that holds *only* because the
linear segment is present (a cube-root-only `f` gives `f(0) = 0` and
`L* = −16`), and one that later Passes will lean on when black points and
clipping arrive. `NUMERIC_CLAIMS.md` NC-010, NC-011, NC-013 are the
properties this form was chosen for.

**Related, and deliberately separated:** `f_inv` does **not** clamp below
the linear segment. ICC's reference code makes negative-XYZ clamping a
**compile-time option** — the reference implementation declines to
decide — so `iccce-color` computes the unclamped value and leaves gamut
policy to the CMM layer where it can be a named, per-transform decision.
That is a **layering** decision, not an approximation with a ΔE cost, and
it now sits beside a genuinely normative finding from the same clause:
**6.4 requires per-component clipping on integer conversion, and no
clipping for float32-based encodings.** That binds the CMM and profile
layers, not this crate — and a reader must not infer from
`iccce-color`'s silence that iccce clamps nowhere.

**Evidence.** `crates/iccce-color/src/lab.rs` — module doc §"Named
DEVIATION", `f`, `f_inv` *(read in the live source by this librarian,
2026-08-11)*; `ICC_Spec\icc\icc__s__pcs_encoding.md` §"A11 RESOLVED-as-
DELEGATED" and §"A9 — the normative clipping rules, now sourced"
*(read)*; `docs/NUMERIC_CLAIMS.md` **NA-001**.

**Consequence.** iccce now has a register of stated deviations, and this
is entry one. Every future departure from printed normative text gets the
same treatment: named at the site, bounded in `NUMERIC_CLAIMS.md` §4,
logged here. **An unstated approximation is indistinguishable from a
bug** (invariant §3.3) — and an unstated *deviation from a standard* is
worse, because it also misrepresents conformance.

**Revisit if:** **ISO 13655** is obtained and states the breakpoint
explicitly (the cheapest thing that would settle this entirely); or
CIE 15 / ISO 11664-4 is obtained; or a difftest shows the choice visible
in a comparison that matters — which, at five orders of magnitude below
perceptibility, would indicate a *different* bug rather than this one.

---

### DL-011 — legacy Lab encoding keys off the **TAG TYPE**, not the profile version. Recorded **before** Pass 4 implements it, because it reverses the corpus's first-pass claim and puts iccce in deliberate disagreement with lcms2

**Date:** 2026-08-11 (Pass 1) · **Adopted from** `icc-spec-librarian`'s
ICC.1:2022 ingest, ambiguity **A1** and divergence **D1/D2** ·
**Filed by:** `icc-librarian` · **Relates to** DL-005, which is
**not** reversed by this entry — see below

**Decision, recorded now and implemented in Pass 4.** When `iccce`
decodes PCSLAB values, the choice between the **legacy** 16-bit encoding
(`0x0000 → 0xFF00` = 0 → 100 for `L*`) and the **general** 6.3.4.2
encoding (`0x0000 → 0xFFFF`) is a property of **the tag type alone**:

```
legacy Lab encoding  ⇔  tag type ∈ { lut16Type ('mft2'), namedColor2Type ('ncl2') }
                        AND the values are on the PCS side
                        AND that PCS side is PCSLAB
```

**`header.version` must not be consulted for this decision.** The
encoding choice is threaded with the tag type, decided at the point the
tag is decoded.

**Why this is filed in Pass 1, before the code that needs it exists.**
Because the natural instinct — and the corpus's own first-pass
recommendation, and the field's dominant CMM — is the *version* test, and
Pass 4 is a large Pass in which a decoder gets written quickly. A rule
recorded after the code is written is a rule the code has already had a
chance to violate.

**What the specification says, verbatim.** ICC.1:2022 **6.3.4.2 NOTE 3**:

> *"Both the lut16Type and the namedColor2Type tag types (**and only
> those tag types**) use a legacy 16-bit encoding of PCSLAB L\*, PCSLAB
> a\* and PCSLAB b\* which is retained for backwards compatibility with
> an earlier profile version (version 2)."*

and **10.10** (`lut16Type`):

> *"For colour values that are in the PCSLAB colour space on the PCS side
> of the tag, this tag uses the legacy 16-bit PCSLAB encoding defined in
> Tables 42 and 43, not the 16-bit PCSLAB encoding defined in 6.3.4.2."*

**"Retained for backwards compatibility with profile version 2" is the
historical *reason* the encoding exists. It is not a conditional.** The
clause says *this tag* uses it — unconditionally, present tense, in the
edition that defines version 4.4.0.0 profiles. The corpus records that it
searched the full document text for any sentence making the encoding
conditional on the version field (`legacy`, `FF00`, `65 280`, `652,8`,
`version 2`, `earlier version`) and **found none**, and that **two
independent PDF extractions agree character-for-character** on the NOTE 3
passage, with a third agreeing on NOTE 3.

**The corpus's first pass claimed the opposite, and is retracted.** The
ambiguity register marks **A1 RESOLVED ★★★** with *"The first pass
claimed the opposite and is retracted."* **This entry is therefore a
correction of a position this project's own knowledge base held**, and
that is why it is a decision-log entry rather than a note.

**Why it matters — the trap runs the *other* way from what everyone
assumes.** A version-based selector is wrong on two large, real
populations:

| Case | A version test does | The clause requires | Symptom |
|---|---|---|---|
| **`mft2` Lab tag inside a v4 profile** — legal, and the majority of production CMYK output profiles, because `mft2` stayed the interoperability choice long after v4 shipped | general 6.3.4.2 encoding | **legacy** encoding | `L*` reads **0.39 % low**, ΔE 0.3–0.5 — **quiet, and below the 1.0 anchor** |
| **`mAB `/`mBA ` (or any non-`mft2`/`ncl2`) Lab tag in a v2-numbered profile** | legacy encoding | general encoding | `L*` reads 0.39 % **high**; values above `0xFF00` decode to `L* > 100`, outside the legal Lab domain, which can produce **NaN** downstream in the `Lab→XYZ` inverse if unclamped — loud-ish |
| **`ncl2` in any profile** — usually not special-cased at all | (nothing) | **legacy** encoding | spot colours off by ΔE 0.3–0.5 |

The first case is the one that matters: it is most production CMYK
profiles, and the error is **sub-perceptual**, which is exactly the
wrong-colour-looks-right failure of project rule 1.

**★ The live disagreement with lcms2 — recorded as a finding, not a
verdict (project rule 7).** The corpus records that **lcms2 keys this
decision on the profile version** (`cmsLabEncoded2FloatV2` /
`_cmsReadInputLUT` inserting V2→V4 Lab stages based on
`cmsGetEncodedICCversion`). Stated with its limits, because this is
precisely where a librarian either labels the uncertainty or launders it:

- **What is certain:** the clause text keys on tag type and states no
  version condition; two independent extractions agree.
- **What is NOT certain and must not be asserted:** that lcms2 is
  *behaviourally* wrong on real files. For the overwhelmingly common case
  — an `mft2` Lab tag in a **v2** profile — both selectors agree. They
  diverge only on `mft2`-in-v4 and on `ncl2`. **Whether lcms2 has another
  code path that makes those cases come out right anyway was not
  verified**; no lcms2 tree was read in that corpus pass.
- **What would settle it — and it is owed to `icc-conformance`:** a
  **behavioural difftest**. Build a synthetic v4 profile containing an
  `mft2` Lab `A2B0`, push a known `L*` through `transicc`, and see which
  of `652.8` / `655.35` lcms2 used. **That test does not exist**, and
  until it does the disagreement is a reading of two texts, not a
  measured divergence.

**Consequence.** Until the difftest is run, **iccce follows the
specification text and must log the divergence at runtime** rather than
silently differing from the field's dominant CMM. A user whose colours
differ from every other tool's deserves to be told which rule iccce
applied and why.

**Interaction with DL-005 — unchanged, and now more important.** DL-005
requires that legacy-Lab correctness be asserted by **exact-value integer
invariants, never by ΔE**, because the error is ≈0.3–0.5 ΔE and passes a
1.0-anchored test. **That reasoning is untouched.** What DL-011 changes is
only the **selector**; the arithmetic DL-005 protects is confirmed
exactly by ICC.1:2022 Tables 42/43 and 12/13, and the corpus notes that
what its first pass got wrong was the selector, not the numbers.

**Two adjacent facts from the same clauses, recorded so Pass 4 does not
rediscover them:** conversion between the encodings is
`× (65 535 / 65 280)` with values outside the target range *"clipped on a
per-component basis"* (normative, and one of the few places ICC.1 states
a clipping policy at all); and the ranges are **asymmetric** — legacy
`L*` above 100,0 *"shall not be used"*, while `a*`/`b*` slightly above
127,0 **are valid PCS values**. `namedColor2Type` (10.17) states the same
`L*` rule with *"should"* instead of *"shall"* — a normativity mismatch
between two clauses stating one rule, which the corpus files as a spec
defect.

**★ A gap this entry does not close, stated plainly.** This entry cites
ICC.1:2022 clause numbers. **DL-002 prohibited exactly that** — *"no
claim in this project may cite an ICC.1 clause number"* — and DL-006 said
the prohibition lifts only when `icc-spec-librarian` files DL-002's
successor entry. **No such entry exists: `ARCHITECTURE.md` §5 ended at
DL-009 before this filing.** *(verified — read this session.)* What *is*
verified is that **the ingest landed**: corpus files now carry
`evidence: primary_spec`, verbatim normative quotations, real clause
numbers, and per-file records of which extractor produced what
*(verified — `ICC_Spec\index.md`, `icc__s__pcs_encoding.md`,
`icc__ref__v2_v4_divergence.md`, `icc__ref__ambiguity_register.md`,
`cie__ref__chromatic_adaptation.md`, `LEGAL_NOTE.md` read)*. So DL-002's
*condition* is materially met and its *successor* is unfiled — and
`crates/iccce-color/src/adapt.rs` already cites "ICC.1:2022 Annex E.3" in
a doc comment. **This is recorded as an open bookkeeping gap, not
resolved here**: the successor to DL-002 is `icc-spec-librarian`'s entry
to file, per DL-006, and it should state on what terms clause citation is
permitted and what a doc comment must say alongside a clause number.

**Evidence.** `ICC_Spec\icc\icc__ref__v2_v4_divergence.md` §"The selector
rule — VERBATIM, ICC.1:2022 clause 6.3.4.2, NOTE 3", §D2, and Tables
42/43; `ICC_Spec\icc\icc__ref__ambiguity_register.md` row **A1**
(RESOLVED ★★★, first pass retracted);
`ICC_Spec\icc\icc__ref__spec_defects.md` §1–§2. All read in the live
corpus by this librarian on 2026-08-11. **No code implements any of this
yet** — `iccce-profile` parses the header and tag table only.

**Revisit if:** the owed difftest shows lcms2 behaviourally agrees with
the clause on `mft2`-in-v4 and `ncl2` (which would remove the divergence
without changing the rule); or a real profile is found whose *intended*
encoding contradicts the tag-type rule (which would make the clause a
poor description of the wild, and is a new entry, not an edit); or a
later ICC edition adds the version condition the 2022 text does not
contain.

---

### DL-012 — **the disagreement DL-011 predicted does not exist**: lcms2 at pin `21c582a` keys legacy PCSLAB off the **tag type**, measured. DL-011's *rule* stands; its *"live disagreement with lcms2"* clause and the runtime-logging consequence that followed from it are **superseded**

**Date:** 2026-08-11 (later the same day than DL-011; Pass 2 / difftest
session) · **Measured by:** `icc-conformance` · **Filed by:**
`icc-librarian` · **Supersedes** the starred *"★ The live disagreement
with lcms2"* section of **DL-011** and its **Consequence** paragraph.
**DL-011 is not rewritten and is not reversed** — its selector rule is
unchanged and is now also what the oracle does.

**What DL-011 left open, in its own words.** It recorded the corpus's
claim that *"lcms2 keys this decision on the profile version"*, marked
that claim **unverified** (*"no lcms2 tree was read in that corpus
pass"*), and named what would settle it: *"a behavioural difftest. Build
a synthetic v4 profile containing an `mft2` Lab `A2B0`, push a known
`L*` through `transicc`, and see which of `652.8` / `655.35` lcms2
used."*

**That difftest has now been run, and the corpus's claim about lcms2 is
wrong.** At the pinned commit, lcms2 keys the legacy PCSLAB encoding off
the **tag type**, agreeing with ICC.1:2022 **6.3.4.2 NOTE 3** and
**10.10** — the same rule DL-011 adopted from the specification text.

**The instrument, because the result is only as good as it.**
`tools/difftest/src/bin/legacy_lab_probe.rs` authors **four synthetic
profiles byte by byte** (category (a), `LEGAL.md` §3 — the only kind that
cannot inherit a bug from the code under test): `scnr` class, RGB device
space, **Lab PCS**, whose only transform tag is an `A2B0` of type
**`mft2` (`lut16Type`)** holding a 2×2×2 CLUT with chosen corner values.
`probe_v2_1.icc` (`0x02100000`), `probe_v4_3.icc` (`0x04300000`) and
`probe_v4_4.icc` (`0x04400000`) are **byte-identical except for the
version word**, and the program **asserts that at run time** — a byte
diff expected to be exactly offsets `[8, 9]` — before believing any
result. `probe_v4_3_mluc.icc` is a v4.3 with proper
`multiLocalizedUnicodeType` metadata, existing solely to close the
objection that the other three carry v2-era `desc`/`text` metadata in a
v4 profile. **Probes land exactly on CLUT corners**, so nothing is
interpolated, and `-c0` stops lcms2 flattening the pipeline into a
resampling of itself. *(verified — the probe source, its four specs, its
byte-diff control and its two decode predicates read by this librarian,
2026-08-11.)*

**The control is half the experiment.** The v2.1 profile is the case
where *both* candidate rules predict legacy. It reads legacy — so the
apparatus demonstrably can detect the effect it is looking for. An
experiment whose apparatus was never shown able to detect its own effect
is not an experiment.

**The result.** At **media-relative colorimetric**, every profile —
v2.1, v4.3, v4.4 and the fully-v4 `mluc` variant — decodes **LEGACY**.
Worst deviation from the legacy prediction across all probes and all four
profiles: **2×10⁻⁵**, which is `transicc`'s printing precision. The
attribution bound was **0.01**, justified rather than picked: about 7×
the 16-bit PCS quantisation floor (`100/65535 ≈ 0,0015`) and about 20×
below the smallest separation between the two hypotheses (≥0,196 in `L*`,
≈1,09 in `a*` at the probes used), with an observation matching neither
hypothesis reported as **inconclusive** rather than rounded to the
nearer. *(reported — the run is `icc-conformance`'s; the predicates, the
bound and its justification were read in the source here.)*

**Corroborated by reading the pinned source**, which is where the
mechanism is visible rather than inferred. `src/cmsio1.c`,
`_cmsReadInputLUT`, tests `_cmsGetTagTrueType(...)` against
`cmsSigLut16Type` and `cmsGetPCS(...)` against `cmsSigLabData` before
inserting `_cmsStageAllocLabV2ToV4` — **no version test anywhere in the
path**. The same tag-type test appears in `_cmsReadOutputLUT` and
`_cmsReadDevicelinkLUT`, and the `namedColor2Type` paths insert the stage
unconditionally. The scale factor is `65535.0/65280.0` — the same
`1.00390625` DL-005 names. *(reported by `icc-conformance` from the
cloned tree at the pin; transcribed in `tools/difftest/README.md` §12.2.
No lcms2 source was read by this librarian, and `vendor/` is git-ignored,
so it is not in this repository to read.)*

**What is superseded, stated exactly.** DL-011 concluded: *"Until the
difftest is run, iccce follows the specification text and must log the
divergence at runtime rather than silently differing from the field's
dominant CMM."* The difftest has been run and **there is no divergence to
log** for `mft2`-in-v4 on this pin. **Pass 4 still implements the
tag-type selector** — but for the reason DL-011 gave (the clause text),
never because lcms2 agrees; agreement with an implementation is
`implementation-cross-check` evidence and cannot be the ground for a
conformance choice (rule 3). The runtime warning should be
**reconsidered, not written**, on the strength of a divergence now
measured absent.

**Coverage is part of this claim and must travel with it.** One tag
(`A2B0`), one tag type (`mft2`), one direction (device→PCS), one PCS
(Lab), **one intent for the verdict** (media-relative colorimetric —
intent 0 is confounded, see **DL-013**), four synthetic profiles, one
platform (Windows 11 Pro 10.0.26200 / MSVC), one lcms2 build at one
commit. **`ncl2` (`namedColor2Type`) was NOT tested behaviourally** — the
source reading says it always gets the legacy stage, which is agreement,
but source reading is not measurement. **B2A (`_cmsReadOutputLUT`) was
not tested behaviourally either.** "lcms2 keys off the tag type" is
therefore established for the case measured and read for the rest, and
the two must not be merged into one sentence.

**What this does NOT establish, and it is the important half.** It says
nothing about whether lcms2 is *right* — DL-011's rule rests on the
clause text and would stand unchanged if lcms2 had disagreed (rule 7).
It also does not repair **DL-005**: the arithmetic that decision protects
is untouched, and legacy-Lab correctness is still to be asserted by
**exact-value integer invariants, never by ΔE**.

**Evidence.** `tools/difftest/README.md` §12.1–§12.3 *(read)*;
`tools/difftest/src/bin/legacy_lab_probe.rs` module doc, `PROBES`,
`decode_legacy` / `decode_general`, `ATTRIBUTION`, and the byte-diff
control *(read)*; `docs/NUMERIC_CLAIMS.md` **NC-019**. Commit **`bfd6b1e`**
*(reported by the dispatching engineer; no git command was run by this
librarian, which has no shell)*.

**Consequence for the corpus, owed and not discharged here.** The corpus
named `cmsLabEncoded2FloatV2` and `_cmsReadInputLUT` as *"inserting V2→V4
Lab stages based on `cmsGetEncodedICCversion`"*. At this pin
`cmsLabEncoded2FloatV2` is called from `cmspack.c` only — a **pixel
formatter** for callers who explicitly ask for a v2-encoded Lab buffer —
and never from profile reading. **That claim needs retracting in
`ICC_Spec`.** A dispatch to `icc-spec-librarian` is **reported** to be in
flight in parallel with this filing; **whether it lands is unverified
here**, and a later session must check rather than assume.

**Revisit if:** the pin moves — which DL-001 already makes a *licence*
event and which this entry makes a **behavioural** event too, because
every sentence above is scoped to `21c582a`; or `ncl2` / B2A are tested
behaviourally and come out differently from the source reading; or a real
(non-synthetic) v4 profile with an `mft2` Lab tag shows something these
four synthetics cannot.

---

### DL-013 — **lcms2 forces black point compensation on for v4 profiles at perceptual and saturation**, on the authority of an Adobe document rather than ICC.1. Measured at ≈3.15 `L*` at black, and it changes what Pass 4's and Pass 5's cross-checks are measuring

**Date:** 2026-08-11 (Pass 2 / difftest session) · **Measured by:**
`icc-conformance`, as an unplanned finding of the DL-012 experiment ·
**Filed by:** `icc-librarian`

**How it was found, which is worth keeping.** The first run of the
legacy-Lab probe used **both** intent 0 (perceptual) and intent 1
(media-relative colorimetric). At intent 1 every profile gave a clean
answer. At intent 0 the **v4** profiles matched **neither** hypothesis —
black came back at `L* = −3.1482` instead of 0 — while the
**byte-identical** v2 profile was unaffected. A result matching neither
candidate is exactly the case an attribution bound exists to refuse to
round away; refusing to round it turned a confound into a second finding.

**The mechanism, read at the pin.** `src/cmscnvrt.c`, `_cmsLinkProfiles`,
with upstream's own comment:

```c
// Check if black point is really needed or allowed. Note that
// following Adobe's document:
// BPC does not apply to devicelink profiles, nor to abs colorimetric,
// and applies always on V4 perceptual and saturation.
if (TheIntents[i] == INTENT_PERCEPTUAL || TheIntents[i] == INTENT_SATURATION) {
    // Force BPC for V4 profiles in perceptual and saturation
    if (cmsGetEncodedICCversion(hProfiles[i]) >= 0x4000000)
        BPC[i] = TRUE;
}
```

with the black point itself taken from a fixed constant in that case
(`src/cmssamp.c`: *"v4 + perceptual & saturation intents does have its
own black point… Black point tag is deprecated in V4"*,
`cmsPERCEPTUAL_BLACK_X/Y/Z` = 0.003 36 / 0.003 473 1 / 0.002 87).
*(reported by `icc-conformance` from the cloned tree at the pin;
transcribed in `tools/difftest/README.md` §12.4 and in the probe's module
doc, both read here.)*

**So lcms2 silently enables BPC for v4 profiles at perceptual and
saturation, whether or not `-b` was passed, on the authority of an Adobe
document rather than ICC.1.** Note precisely what is and is not sourced:
the *behaviour* is measured, the *code comment* is transcribed, and **the
Adobe document itself has been obtained by nobody here.** "Following
Adobe's document" is upstream's attribution, not a citation this project
can check, and it must not be restated as though the document had been
read.

**Confirmed quantitatively rather than assumed** — and this is the part
that turns a hypothesis into a mechanism. Transcribing lcms2's own
`ComputeBlackPointCompensation` (`a = (bp_out − D50)/(bp_in − D50)`,
`b = −D50·(bp_out − bp_in)/(bp_in − D50)`, per channel) and running the
legacy-decoded `L*` through it predicts the observation on all four
probes: `100.0000 → 100.0000`; **`0.0000 → −3.1482`**;
`50.1961 → 49.8574`; `100.0000 → 100.0000`. **Predicted matches observed
to 3×10⁻⁵**, against an asserted bound of `0.005` (justified as ~3× the
16-bit `L*` quantisation step, with the effect being explained ≈630×
larger). Only the `Y`/`L*` channel is predicted, deliberately: one
channel predicted to four decimals is better evidence than three
predicted loosely. *(reported — the run; the transcription
`predict_bpc_lstar`, the constants and `BPC_PREDICTION_TOL` were read in
the source here.)*

**The arm that did NOT decide is kept, and that is deliberate.** An
earlier attempt re-ran the byte-identical **v2** profile at intent 0 with
`-b`, expecting it to reproduce the v4 numbers. It did not: `-b` is a
no-op on that fixture because `cmsDetectBlackPoint` reaches the fixed
perceptual constant only through the same `>= 0x4000000` guard, and with
source and destination black points equal lcms2 skips the stage. **Two
arms that differ in more than the variable cannot settle anything** — so
the null result is reported as inconclusive rather than as a refutation,
because a reader repeating it would otherwise read it as one.

**Consequences — larger than the finding that prompted them, and they
land on two planned Passes:**

1. **Pass 4's done-when is now underspecified as written.** It reads
   *"CMYK→RGB through a real press profile matches lcms2 within tolerance
   at every intent."* Against a **v4** profile, "every intent" includes
   two intents at which lcms2 is running a transform with BPC in it that
   iccce has no ICC.1 obligation to run. **Pass 4 must either (a) run
   perceptual and saturation with the forced BPC explicitly accounted for
   — reproducing it, or subtracting it, and saying which — or (b) take
   the cross-check at the colorimetric intents only and state that the
   other two are excluded and why.** Silently comparing at all four and
   tuning a tolerance until it passes would be `TOLERANCES.md` §0's
   failure mode exactly: a tolerance set on the wrong quantity. The
   disagreement it would absorb is **≈3.15 `L*` at black** — three orders
   of magnitude above the sub-perceptual errors this project worries
   about, and not the kind of thing a tolerance should be quietly wide
   enough to swallow.
2. **Pass 5's comparison target has a measured shape before Pass 5
   begins.** Its done-when is *"BPC on and off differ in the documented
   direction, and match lcms2's BPC within tolerance."* On v4 profiles at
   perceptual/saturation, **lcms2's "BPC off" is not BPC off** — so the
   `-b`-on/`-b`-off pair that looks like the natural experiment does not
   isolate the variable there. Pass 5 gets lcms2's exact BPC arithmetic
   transcribed and pre-validated to 3×10⁻⁵ (above) as a starting point,
   which is a real head start — provided it is used as an
   `implementation-cross-check` and never as ground truth for what BPC
   *should* do.
3. **If iccce does not copy the behaviour, disagreement at those intents
   is expected and is a finding under rule 7, not a failure.** Whether to
   copy it is a Pass 4/5 decision that this entry does **not** make: it
   is a choice between matching the field's dominant CMM and doing only
   what ICC.1 requires, and it deserves its own entry when it is made.
4. **It is a plausible origin for the corpus's retracted belief** that
   lcms2 keys Lab decoding on the profile version (DL-012). lcms2 **does**
   key a decision on the profile version — at perceptual and saturation
   intent. Just not that one.

**Evidence class, stated so the entry cannot be over-read.** This is an
`implementation-cross-check`-class observation of **one build of one
implementation at one pin** — `NUMERIC_CLAIMS.md` **NC-020**. It is
**not** ground truth about colour and **not** a statement that lcms2 is
wrong; ICC.1 does not require the behaviour, and "not required" is not
"prohibited". What it establishes is **what iccce will be compared
against**, which is precisely what an oracle is for.

**Evidence.** `tools/difftest/README.md` §12.4 *(read)*;
`tools/difftest/src/bin/legacy_lab_probe.rs` module doc §"The confound",
`predict_bpc_lstar`, `LCMS2_PERCEPTUAL_BLACK`, `BPC_PREDICTION_TOL`
*(read)*; `docs/TOLERANCES.md` §6.1 item 2 *(read — filed independently
by `icc-conformance`, and it says the same thing)*;
`docs/NUMERIC_CLAIMS.md` **NC-020**. Commit **`bfd6b1e`** *(reported)*.

**Revisit if:** the pin moves (every number here is scoped to
`21c582a`); or ICC.1 or a later edition is found to require the
behaviour after all, which would change it from an implementation quirk
into a conformance question; or the Adobe document upstream cites is
obtained, which would let this project read the authority rather than the
attribution; or Pass 4/5 decides whether iccce copies it, which is a new
entry.

---

### DL-014 — **DL-002's successor**: ICC.1:2022 clause numbers may now be cited, on stated terms. The prohibition lifts **only** for that document, **only** where the corpus carries the clause at `primary_spec` tier, and **only** with the corpus file named alongside it

**Date:** 2026-08-11 (Pass 2 batch 2 session, later the same day than
DL-013) · **Successor to** **DL-002**, whose revisit condition fired at
**DL-006** · **Filed by:** `icc-librarian`

**Why this entry is overdue, and by whom it was owed.** DL-006 said the
successor was `icc-spec-librarian`'s to file, and three consecutive
filings have recorded it as still unwritten while `ARCHITECTURE.md` §5
ran to DL-013 and **DL-010, DL-011, DL-012 and several `iccce-profile`
and `iccce-color` doc comments cited ICC.1:2022 clause numbers.** That
is a live contradiction between the decision log and the code: every one
of those citations was, on the letter of DL-002, prohibited. It is filed
here by `icc-librarian` on the engineer's dispatch — which is a
**reassignment of the filing, not of the sourcing judgement**. The
substance below rests on what the corpus itself records, read in the
live corpus by this librarian; §5 is this librarian's document, and
leaving a known contradiction in it across three filings was the worse
of the two errors available.

**The condition DL-002 set, in its own words:** *"Until then, no
ICC-hosted document is a source for this corpus, and no claim in this
project may cite an ICC.1 clause number."* — where *"then"* was
`ICC.1-2022-05.pdf` reaching `ICC_Spec\_sources\` by human retrieval.
**That happened** (DL-006, the file enumerated on disk by this
librarian), **and the ingest has since landed and been verified**:
`ICC_Spec\index.md` records **15 of 20 corpus files now carrying a
`primary_spec` tier — 4 fully and 11 partly** — against **0** before the
ingest, with 19 of 35 ambiguity rows resolved and the required/optional
tag column populated from clause 8 and Annex G. *(verified —
`ICC_Spec\index.md` read by this librarian, 2026-08-11.)*

**Decision — what is now permitted.**

1. **A clause number from ICC.1:2022 may be cited** in a doc comment, a
   test, a decision-log entry, `NUMERIC_CLAIMS.md`, or any other project
   document.
2. **The citation must name the corpus file that carries it.** The
   corpus is the verification trail, and it is the only thing a later
   reader can check without re-opening a PDF nobody may fetch
   automatically. *"ICC.1:2022 clause 10.10"* alone is an assertion;
   *"ICC.1:2022 clause 10.10, `icc__type__lut8_lut16.md`"* is a
   citation. `crates/iccce-profile/src/lut.rs`'s §Sourcing block is the
   shape intended. *(verified — read.)*
3. **The tier is per-fact, not per-file, and this is the part most
   likely to be got wrong.** Eleven of the fifteen files are **partly**
   `primary_spec`: their frontmatter splits the `evidence:` line
   explicitly. `icc__type__lutAtoB_lutBtoA.md` reads
   `evidence: primary_spec (clause numbers + the CLUT/interpolation
   rules) / icc_secondary_code (byte layouts — NOT re-transcribed this
   pass)`. *(verified — frontmatter read.)* So citing that file's
   **clause number** is permitted and citing its **byte table** as
   specification-sourced is not — the byte table is still code-derived,
   A23/A24 are still open, and `lut.rs`'s module doc says so at the site.
   **Read the `evidence:` line before citing, every time.**

**What remains prohibited, and it is most of the corpus's citable
surface.** No clause, table, page or requirement may be cited from a
document nobody in this project has read. Named, because each has
already been reached for at least once:

| Document | Standing |
|---|---|
| **ICC.1:2010-12** (v4.3) | not obtained. `parametricCurveType` **Table 68 changed** between it and ICC.1:2022 (divergence **D10**) and **what changed is NOT SOURCED.** Do not guess it and do not cite ICC.1:2010 clause numbers. |
| **ICC.1:2001-04** (v2) | not obtained. The **only normative home of `textDescriptionType`**, which batch 1 decodes from a **code-derived** layout that says so. |
| **ISO 13655** | paywalled, not obtained — and it is the **actual authority** ICC.1:2022 6.4 *delegates* `f(t)` to (DL-010). |
| **CIE 142-2001 / ISO/CIE 11664-6, CIE 15 / ISO 11664-4, CIE 159** | paywalled, not obtained (DL-004, NA-001, `NUMERIC_CLAIMS.md` §5). |
| **IEC 61966-2-1** | paywalled, not obtained — which is why the corpus's sRGB file and the D65 chromaticity are **single-source** (NC-018). |
| **"Adobe's document"** (lcms2's BPC authority, DL-013) | obtained by nobody here. It is an **attribution transcribed from a code comment**, and DL-013 already forbids restating it as a citation. |

**Two things this entry deliberately does not do.**

- **It does not retroactively bless the citations already in the tree.**
  Each pre-existing ICC.1:2022 citation is permitted **if** it satisfies
  clauses 1–3 above, and any that does not is a defect to be fixed at
  the site — reported, not papered over. No sweep of existing citations
  has been performed by anyone; that is owed work, recorded in
  `NUMERIC_CLAIMS.md` §7.2.
- **It does not touch DL-002's other half.** *"No ICC-hosted document is
  a source for this corpus"* by **automated retrieval** is unchanged and
  standing: color.org's robot clause is unaltered, ICC.1:2022 was
  cleared by **human** retrieval, and that created no route for agents.
  **Do not re-attempt automated retrieval of any color.org /
  archive.color.org document.** The same test applies before any ITU-R
  fetch (DL-007).

**Evidence.** `ICC_Spec\index.md` (the 0→15 `primary_spec` count, the
ambiguity-row and requirements-column changes); `ICC_Spec\icc\
icc__type__lutAtoB_lutBtoA.md` frontmatter (the split tier, quoted
above); `crates/iccce-profile/src/lut.rs` §Sourcing (the citation shape
this entry endorses). **All read in the live sources by `icc-librarian`,
2026-08-11.** No PDF was opened and no page count, hash or size of
`ICC.1-2022-05.pdf` has been verified by this librarian at any point —
that the ingest reflects the document it names is
`icc-spec-librarian`'s establishment, reported and relied on, not
re-derived here.

**Revisit if:** a corpus file's tier is **downgraded** (a re-check that
finds a transcription wrong would invalidate every citation resting on
it, and the corpus's own C1/C2/C3 errata show that happens); or a
further ICC-hosted document is obtained by human retrieval, which
extends the permission to that document by a **new** entry and never by
analogy to this one; or an agent is ever granted written consent for
automated access, which is a different decision about a different act.

---

### DL-015 — `pow(negative, fractional)` in a parametric curve is **guarded**, following lcms2 rather than ICC's own sample code. This is a divergence **inside a hole the specification declares open** — it is *not* a deviation from normative text, and the difference from DL-010 is the point of the entry

**Date:** 2026-08-11 (Pass 3) · **Decided by:** `icc-engineer`, on the
corpus's Guards section · **Filed by:** `icc-librarian`

**Decision.** `crates/iccce-cmm/src/curve.rs::eval_parametric` evaluates
every `pow` through a local closure:

```rust
let pow_guarded = |base: f64, exp: f64| if base > 0.0 { base.powf(exp) } else { 0.0 };
```

so a negative (or zero) base returns **0.0** instead of `NaN`.
*(verified — read in the live source by this librarian, 2026-08-11.)*
ICC's own reference implementation calls `pow` unguarded; lcms2 guards.
**iccce follows lcms2**, and the module doc says so under the heading
*"Named divergence from ICC's sample code."* *(verified — read.)*

**Why it needed a decision at all.** The two reference implementations
*behave differently* on the same input, so an implementer who reads
either one and copies it is making a choice without knowing it. The
corpus states the conflict and the direction to take, verbatim:
*"`pow(negative, fractional)` = NaN. lcms2 additionally guards `e > 0`
before `pow`; ICC's code does not. **A real behavioural difference
between the two implementations on malformed/extreme parameters.**
Follow lcms2 (guard), and record it as a deliberate divergence from
ICC's sample code."* *(verified —
`ICC_Spec\icc\icc__type__curve_parametric.md` §Guards, read.)*

**★ What kind of departure this is, stated precisely, because the
obvious mis-filing is to shelve it next to DL-010.** DL-010 is a
departure **from printed normative text** — ICC.1:2022 writes a decimal
breakpoint and iccce uses a rational. **This is not that.** ICC.1:2022
**10.18** declares the relevant parameter combinations **explicitly
undefined**: *"The domain and range of each function shall be [0,0
1,0]. Any function value outside the range shall be clipped to the
range of the function"*, with complex/undefined parameter combinations
called out as undefined — which the corpus rightly notes is *a stated
non-requirement, stronger than silence.* Table 68's formulas are
normative; **what `pow` does with a negative base under a fractional
exponent is not specified by anything.** So iccce is choosing inside a
hole the standard leaves open, and **this entry must never be restated
as a conformance departure.** It is registered in `NUMERIC_CLAIMS.md`
§4 as **NA-004** because rule 4 requires every named choice to be
registered — not because it is a deviation from a requirement.

**★ Two precisions the module doc's own wording does not carry, found
while filing and reported rather than repaired** (the file is the
engineer's):

1. **"turns NaN into a defined, reported value" — it is defined; it is
   not reported.** `Trc::eval` returns a bare `f64` and has no
   diagnostic channel; the substitution of `0.0` for `NaN` is **silent
   at the evaluation site**, and nothing anywhere in the workspace
   surfaces that it happened. That is not necessarily wrong — invariant
   §3.2 (*report, do not repair*) binds the **parser**, and an undefined
   parameter combination is not automatically a malformation, so there
   may be nothing for the parser to have reported. But *"reported"* is a
   claim about a disclosure surface, this project distinguishes those
   carefully, and the surface does not exist. **Do not carry the word
   forward.**
2. **The guard also fires on one well-formed input.** Parametric type 0
   with `g = 0` is the constant curve `y = x⁰ = 1`; at **exactly
   `x = 0`** the base is `0.0`, which is not `> 0.0`, so `pow_guarded`
   returns `0.0` while every `x > 0` returns `1.0`. The mathematical
   value at the origin is 1. So the guard introduces a **step at the
   origin on a degenerate constant curve** — a curve whose *inverse* is
   already refused by name (`CurveError::ConstantNotInvertible`) but
   whose forward evaluation is permitted. *(verified — derived from
   `eval_parametric` and `invert_parametric` as written; no test in the
   repository exercises `g = 0` forward.)* Cost is confined to that one
   point of that one degenerate curve.

**The cost, and its exact status.** *"None on well-formed curves"* —
which is consistent with the code (for `a > 0`, `b ≥ 0` the base is
positive across the whole branch, so the guard never fires) and is
**analytic, not measured**. No test in this repository compares guarded
against unguarded output, and `NUMERIC_CLAIMS.md` NA-004 records the
cost as **unmeasured**. Nobody may write *"measured to cost nothing."*

**Consequence.** iccce's register of named departures now holds two
kinds, and they are not interchangeable: **NA-001** (deviation from
printed normative text, DL-010) and **NA-004** (a choice inside a stated
non-requirement, this entry). A future reader auditing conformance needs
to be able to tell them apart at a glance, which is why the register
states the kind in the row rather than only in the prose.

**Evidence.** `crates/iccce-cmm/src/curve.rs` — module doc §"Named
divergence from ICC's sample code", `eval_parametric`, `pow_guarded`
*(all read in the live source by this librarian, 2026-08-11)*;
`ICC_Spec\icc\icc__type__curve_parametric.md` §Guards and its
frontmatter (`evidence: primary_spec (clauses 10.6, 10.18, Annex F.1,
verified 2026-08-11) / cross_verified_2src (prior code provenance…)`)
*(read — the citation therefore satisfies **DL-014**: the corpus file is
named and the cited facts are `primary_spec` in its split evidence
line)*; `docs/NUMERIC_CLAIMS.md` **NA-004**. Commit **`c4038eb`**
*(reported by the dispatching engineer; `icc-librarian` has no shell and
ran no git command)*.

**Revisit if:** ICC's sample code adds the guard, or a later ICC edition
specifies the undefined case (either would make this a non-divergence);
or a real profile is found whose intended output depends on the
unguarded branch, which would make the choice visible in colour rather
than only in NaN-avoidance; or a difftest ever measures iccce and lcms2
disagreeing *here*, which would mean the two guards are not the same
guard and the corpus's reading of lcms2 needs re-checking (a
source-reading, not a measurement — the same distinction DL-012 turned
on).

---

### DL-016 — sampled-table curves are asserted by **exact values at the sample points**, because the self-consistency round trip **would have passed with the off-by-one-sample bug in place**. A method decision, filed with the measured instance that justifies it

**Date:** 2026-08-11 (Pass 3) · **Found by:** `icc-engineer` on the
first run of Pass 3's tests · **Filed by:** `icc-librarian` ·
**Relates to** **DL-005**, which decided the same *shape* of thing
prospectively; this is the retrospective twin

**What happened.** `eval_table` (linear interpolation over a sampled
`curveType`, clause 10.6) paired the **clamped** segment index with the
**unclamped** fraction. At `x = 1.0` the fraction is 0 while the segment
index has clamped to `n − 2`, so the function returned `t[n − 2]`
instead of `t[n − 1]` — for a fine gamma table, **TRC(1.0) ≈ 0.998
instead of 1.0**. The test `table_eval_exact_at_samples` failed on the
first run; the code was fixed and the finding is written at the site.
*(verified — `crates/iccce-cmm/src/curve.rs`, the `frac` derivation and
its explanatory comment, and the test, read in the live source by this
librarian.)*

**Why this is a decision-log entry and not a session-log line — this
paragraph is the entry.** Because of what the *other* tests would have
done. Three checks in this Pass touch a real profile's sampled TRCs, and
**two of them pass with the bug present:**

| Check | Bound | Residual **with the bug** | Verdict |
|---|---|---|---|
| Real-profile device→PCS→device round trip | `1×10⁻³` device units | `1/1023 = 9.775×10⁻⁴` | **PASSES**, with ~2 % of margin |
| Real-profile white → colorant sum, `X` | `1×10⁻²` | `≈1.9×10⁻³` (X scaled by 0.998) | **PASSES** |
| `table_eval_exact_at_samples` | `1×10⁻¹⁵` at each sample | `≈2×10⁻³` | **FAILS** — the one that caught it |

**The first row is the load-bearing one, and the coincidence in it is
structural rather than bad luck.** With the bug, `eval(1.0)` returns the
second-to-last sample; inverting that value lands exactly on the
second-to-last sample's abscissa, `(n − 2)/(n − 1)`. **The error is
therefore exactly one table spacing** — and the round-trip bound was
justified as *"≈ the table's input spacing"*. **Any bound derived from
the table's spacing is by construction the wrong instrument for an
off-by-one-sample error**, because the two quantities are the same
quantity. A tolerance cannot discriminate a defect whose magnitude is
its own justification.

**★ The exact status of that arithmetic.** It was **computed by
`icc-librarian` from the code as written** — the buggy branch was
reconstructed from the comment at the site and from `invert_table`'s
segment search. **Nothing was run**; this agent has no shell. It rests
on the profile's TRC tables having **1024 entries**, which is the
engineer's statement in a test comment and has **not** been verified
here (the profile is a binary this librarian did not read). Anybody
re-deriving it should establish `n` first: at `n = 1024`,
`1/(n−1) = 9.775×10⁻⁴`, inside the `1×10⁻³` gate; at `n = 512` it would
be `1.96×10⁻³` and the round trip would have failed. **The conclusion
"the round trip would have passed" is therefore true for this profile's
table size and is not a general law** — which is itself the argument for
not depending on it either way.

**Decision.** Sampled-table curve evaluation is asserted by **exact
values at the sample points, at `f64`-noise tolerance**, endpoints
included and especially. A round-trip or self-consistency bound derived
from the table's own spacing **may not be relied on** to catch a
sample-indexing error, and a Pass that ships table interpolation without
an exact-value endpoint test has not tested it. Interpolation *between*
samples is a separate assertion (the midpoint check, at `1×10⁻¹²`)
because it tests a different thing: that the rule is linear (clause
10.6, normative — corpus **A15**), not that the endpoints are the right
entries.

**Why it generalises beyond curves.** Every table in this format is
read by the same shape of code — CLUT grids (Pass 4), `mft1`/`mft2`
input and output tables, `ncl2` PCS coordinates. **The endpoint of a
table is the place an off-by-one hides best**, because it is the one
place where a clamp exists to be paired wrongly with something.

**Relation to DL-005, which is not superseded and is strengthened.**
DL-005 decided *before any code existed* that legacy-Lab correctness
would be asserted with exact integer invariants rather than in ΔE,
because the error mode sits **below** the grading tolerance. That was a
prediction. This entry is the same principle with a **measured
instance**: an error that sits below its natural tolerance, caught only
by the exact-value assertion, with the margin computed. **A predicted
methodological hazard and a demonstrated one are different objects
(the DL-011/DL-012 lesson), and this project now has one of each.**

**What this entry does NOT claim.** It does not claim the test suite is
adequate, that other off-by-ones are absent, or that the bug ever
existed anywhere but in an uncommitted working tree during one
afternoon. **It shipped nothing.** The record exists because *"the
exact-value discipline paid"* is only demonstrable if the instance is
written down **with its counterfactual** — and the counterfactual, not
the bug, is the content.

**A second finding from the same first run is filed elsewhere,
deliberately.** The real-profile white-point tolerance was re-justified
after failing — the profile's colorant `Z` sums to **0.825089**, `1.9×10⁻⁴`
from ICC's 4-figure D50, which is the file's author's own white
rounding and a fact about the **file**. That is a rule-5 worked example
about *tolerances*, and it belongs in `NUMERIC_CLAIMS.md`
(**NC-031**), not here: no decision was taken, a number was justified.

**Evidence.** `crates/iccce-cmm/src/curve.rs` — `eval_table` and its
comment, `table_eval_exact_at_samples`, `invert_table`;
`crates/iccce-cmm/src/matrix_trc.rs` — `system_srgb_profile_end_to_end`
and its two bounds *(all read in the live source, 2026-08-11)*;
`docs/NUMERIC_CLAIMS.md` **NC-025**, **NC-031**, **NC-032**. Commit
**`c4038eb`** *(reported)*.

**Revisit if:** the table representation changes — e.g. tables
normalised to `f64` at parse time, or an interpolation rewrite in Pass 6
— which would move *where* the exactness must be asserted without
changing that it must be; or a table type appears whose endpoints are
not required to be attained (none is known).

### DL-017 — `tools/difftest` **may path-depend on iccce's own crates**, because the arrow points harness → code-under-test. The coupling is permitted by four named conditions, and the invariant it might have threatened is untouched

**Date:** 2026-08-11 (Pass 3 closure) · **Decided by:** `icc-conformance`
while building the Pass 3 differential · **Filed by:** `icc-librarian` ·
**Relates to** **DL-001** (the oracle is pinned and insulated), which
this entry deliberately does *not* weaken

**What was decided.** `tools/difftest/Cargo.toml` declares **three path
dependencies** on the shipping crates:

```toml
[dependencies]
iccce-color   = { path = "../../crates/iccce-color" }
iccce-profile = { path = "../../crates/iccce-profile" }
iccce-cmm     = { path = "../../crates/iccce-cmm" }
```

*(verified — read in the live tree, 2026-08-11.)* Mirrored in
`LEGAL.md` §1 *(verified — read)* and in `tools/difftest/README.md`
§13.2.

**Why it needed a decision at all.** `tools/difftest`'s own module docs
**previously forbade** exactly this, on the reasoning that any coupling
between the harness and the code under test *"must be a documented
decision, not a convenience."* The immediate need was real: computing
ΔE2000 without `iccce-color` means either a second, unvalidated
implementation of CIEDE2000 inside the harness — a ruler nobody has
checked, grading a colour engine — or no ΔE at all, which would have made
Pass 3's done-when unanswerable in the units it is written in.

**The four conditions, all of which must remain true.** They are recorded
here rather than only in the harness because a condition that lives only
next to the code it constrains is a condition that quietly lapses.

1. **The direction is the safe one.** The invariant that matters
   (`tools/difftest/README.md` §1, `LEGAL.md` §4) is *"no crate under
   `crates/` may reach lcms2."* These arrows point **difftest →
   iccce** — harness → subject, the ordinary direction of a test
   harness. `cargo tree` on any shipping crate still cannot reach lcms2;
   the harness is still **outside the workspace**, so
   `cargo test --workspace` still cannot pull it in, and the publication
   guard is unchanged.
2. **The ruler is validated against the literature, not against
   itself.** `iccce_color::delta_e_2000` is graded against **all 34
   published pairs of Sharma, Wu & Dalal (2005)** at 1×10⁻⁴ — **NC-001**,
   the single `published-ground-truth` row in this project. Using an
   unvalidated ΔE here would have hidden a systematic mis-scaling *inside
   the metric*, where it is invisible by construction.
3. **The claim does not change.** Every iccce-vs-lcms2 record is
   **`implementation-cross-check`**, however good the ΔE code is. **A
   good ruler does not promote a weak claim** — the same rule that
   forbids transplanting an oracle's numbers into a unit test as
   expectations (rule 3).
4. **The answer under test still comes from subprocesses.** iccce's
   colours come from running the **shipped `iccce transform` binary**;
   lcms2's from running `transicc`. Calling
   `MatrixTrcTransform::convert` in-process would be one line shorter and
   would make the two sides **asymmetric** — printing, parsing and
   argument handling exercised on one side only. The linked crates are
   the **instrument**, never the **subject**.

**The one exception, and it labels itself.** Record 7
(`pass3/instrument/adobergb-device-to-lab-ruler`, ledger **NC-040**) *is*
an in-process call: it holds iccce's device→Lab model against lcms2's
rendering of the same profile, to check the ruler. It says so in its own
`source` field, and at 8.79×10⁻⁵ ΔE2000 — below `transicc`'s Lab print
floor — the two rulers are indistinguishable.

**What this entry does NOT decide.** It does not permit a shipping crate
to depend on the harness (the reverse arrow, which would be the
dangerous one); it does not fold `tools/difftest` into the workspace;
and it does not licence in-process evaluation as the default — condition
4 is a constraint, not a preference, and `Iccce`'s doc comment forbids
the shortcut at the site.

**Evidence.** `tools/difftest/Cargo.toml` (the three path dependencies
and the ~50 lines of rationale above them); `tools/difftest/README.md`
§13.2; `docs/LEGAL.md` §1; `docs/TOLERANCES.md` §3.3.2 *(all read in the
live tree by this librarian)*. Commit **`986dae6`** *(reported — no
agent in this project has run git)*.

**Revisit if:** the harness ever needs to compare something
`iccce-color` cannot express (a second ruler would then need its own
validation row before use); or a shipping crate is proposed to depend on
anything under `tools/` (refuse, and re-read condition 1); or the
workspace membership of `tools/difftest` is ever proposed to change,
which would undo both the licence insulation and the publication guard
at once.

### DL-018 — an **upper-bound gate on a deliberate cost** must be paired with a **prediction pin**, because deleting the requirement makes the gate greener. Filed with the worked pair that demonstrates it, and with the scope limit that a first draft of that pair got wrong

**Date:** 2026-08-11 (Pass 3 closure) · **Found by:** `icc-conformance`
while deriving the round-trip tolerance · **Filed by:** `icc-librarian` ·
**Relates to** **DL-016** (a bound cannot discriminate a defect whose
magnitude is its own justification) — this is the same failure family,
one level up: there, the bound could not *see* the error; here, the bound
**rewards** it

**The general shape.** Some measured quantities are not error at all;
they are **the price of doing the right thing**. iccce's range clamping
(Annex F.8–F.16, normative) discards the difference between two
profiles' encoded media whites, and that discard **costs ΔE in a round
trip**. Grading such a quantity with an upper bound produces a gate with
a perverse gradient:

| | round-trip metric | the 2.5×10⁻² gate |
|---|---|---|
| iccce as shipped, clamping per F.8–F.16 | 1.8788×10⁻² | **passes** |
| **clamping deleted** | **0 (exact identity)** | **passes more comfortably** |

**A gate that goes greener when a normative requirement is removed is
not a gate.** Nothing about that failure announces itself: the suite
stays green, the number *improves*, and the improvement is the symptom.

**The decision.** When a check's metric is dominated by a **deliberate,
required cost**, the upper-bound row is **not sufficient on its own**. It
must be accompanied by a **prediction pin**: a second row asserting that
the observed cost matches an **independently computed prediction** of
that cost, to a tolerance derived from the measurement chain's own
precision — plus a **sensitivity control** demonstrating the pin would
fail if the requirement were removed. A pin without a sensitivity control
is an assertion that the apparatus works.

**The worked instance, in full, because the rule is only checkable
against one.**

- **The upper bound.** `pass3/srgb-to-adobergb-to-srgb/roundtrip-de2000`,
  tolerance 2.5×10⁻² ΔE2000, observed **1.8788×10⁻²** (ledger
  **NC-038**).
- **The prediction.** From the two files' colorant matrices and the
  clamp **alone** — no tone curve (every TRC in this pair is exactly 1
  at 1), no lcms2, no measurement — the white-corner cost is
  **1.878244×10⁻²** in closed form, against **1.878818×10⁻²** observed:
  **0.03 % agreement**.
- **The pin.** `pass3/roundtrip/white-clamp-cost-matches-prediction`
  asserts |predicted − observed| < **1×10⁻³**, observed **5.7392×10⁻⁶**
  (ledger **NC-039**). The bound is **10× the ≈1×10⁻⁴ ΔE00 floor** that
  `iccce transform`'s 6-decimal device print imposes on each leg — a
  tolerance derived from the measurement chain, not from the effect.
- **The sensitivity control.** With clamping removed the observation
  would be 0 and the pin's metric would read 1.878×10⁻² — **failing by
  19×**. Printed by `pass3_report`.

**★ The scope limit, and it is the most useful part of this entry
because a first draft got it wrong.** The pin was first claimed to make
the normative **F.8–F.16 clamp ORDERING** falsifiable. **It does not.**
`iccce-cmm` clamps at **three independent sites**, each separately
cited — `MatrixTrc::pcs_to_device` (F.8–F.16, linear → [0,1] before
TRC⁻¹), `Trc::eval` (clause 10.18, the curve's domain), and
`Trc::eval_inverse` / `invert_table` (F.1(b), the attainable range) — so
the other two make the first **redundant at the shipped surface**. The
pin catches **a wrong colorant matrix** and **clamping removed from all
three sites**; it does **not** catch the F.8–F.16 clamp removed alone,
and **no test in this repository distinguishes clamp-before from
clamp-after through the binary.** Distinguishing them requires a TRC
whose inverse is defined outside [0,1], **which iccce never permits**.
Recorded as **owed, not covered** (`TOLERANCES.md` §3.3.3, blank row;
`tools/difftest/README.md` §13.6.4). **The correction was made in place
rather than by deleting the claim**, which is why a reader can tell
"checked and narrower than hoped" from "never checked."

**Where this rule comes due next, and it is not hypothetical.**
**Pass 5, black point compensation**, is the same shape and worse: BPC
exists to *change* the result, its round-trip and gamut behaviour
generally *improve* under some metrics, and DL-013 records that lcms2
forces it on for v4 perceptual and saturation at ≈3.15 `L*`. A Pass 5
gate that is only an upper bound will reward both deleting BPC and
mis-scaling it. **Pass 4** meets it wherever a clip, a gamut-mapping
step or an intent-dependent adjustment dominates a metric.

**What this entry does NOT claim.** It does not say every
self-consistency row needs a pin — most price an approximation whose
removal would make the metric *worse*, and those are already
well-conditioned. It applies **only** where removing a requirement
**improves** the metric. And it does not make the pin a correctness
claim: NC-039 is `self-consistency`, and both its sides ultimately come
from iccce.

**A note on ownership, because the tolerance and the method are
different objects.** The *number* 2.5×10⁻², its supersession of 1×10⁻²,
and both justifications are logged in **`TOLERANCES.md` §4**, which is
`icc-conformance`'s append-only tolerance history, and are **not
duplicated here** — this entry is filed for the **method**, which is a
standing rule about how gates are built and therefore belongs to the
decision log. `NUMERIC_CLAIMS.md` **NC-038** and **NC-039** carry the
measured values. Three documents, three different jobs, one event.

**Evidence.** `tools/difftest/README.md` §13.6.3 and §13.6.4;
`tools/difftest/src/pass3.rs` (the tolerance constants and their `why`
strings) and `src/bin/pass3_report.rs` §5; `docs/TOLERANCES.md` §3.3.1
rows 5–6, §3.3.3 and §4; `docs/NUMERIC_CLAIMS.md` **NC-038**,
**NC-039**, **NC-042** *(all read in the live tree; **the run itself is
`icc-conformance`'s and is reported — this librarian ran nothing**)*.
Commit **`986dae6`** *(reported)*.

**Revisit if:** a fixture arrives that distinguishes the three clamp
sites (the scope limit above narrows, and the pin's coverage genuinely
widens); or a Pass 5 BPC gate is written — at which point this entry is
the checklist, not a precedent to be re-derived.

### DL-019 — when a disagreement's **mechanism is identified but its authority does not exist**, the raw comparison is **REPORTED, NOT GRADED** and the gate moves to the **modelled** quantity. Filed with the live instance: iccce reads a v2 `wtpt` **as stored**, lcms2 substitutes **D50**, and it costs **11.217 ΔE2000** pending corpus **A4b**

**Date:** 2026-08-11 (Pass 4, the A2B differential) · **Found by:**
`icc-conformance` while running all four intents for the first time ·
**Filed by:** `icc-librarian` · **Relates to** **DL-013** (a measured
lcms2 behaviour that changes what a cross-check measures), **DL-018**
(what to do when a gate's metric is dominated by something other than
error), and **NA-007** (the assumption this divergence lands on, named a
Pass before it bit)

#### The instance, stated first, because the rule is only checkable against one

At the **ICC-absolute** intent, `USWebCoatedSWOP.icc` → the Windows
system sRGB profile, iccce and lcms2 differ by **max 11.217 ΔE2000, mean
4.670** (device max 0.1580) — **two orders of magnitude more than at any
other intent**, and far beyond anything the CLUT interpolation envelope
for the table absolute uses (0.2542) could account for. **The worst
points are the lightest**: paper at 10.6, 33 % C at 11.2.

**The mechanism was read at the pin and then measured.** Both
implementations build the same **D.6/D.7** diagonal; **they differ in
what they read for the destination media white**:

| | source white | destination white |
|---|---|---|
| **iccce** (**NA-007**: `wtpt` **as stored**) | SWOP's `wtpt` | the sRGB profile's `wtpt` = **D65** (0.950 455, 1.0, 1.089 050) |
| **lcms2** | SWOP's `wtpt` (a `prtr`, so the tag is used) | **D50** (0.9642, 1.0, 0.8249), substituted by `cmsio1.c`'s `_cmsReadMediaWhitePoint` because the profile is **v2 AND display-class** |

The ratio is D65/D50 = (0.9858, 1.0, 1.3202) — **a 32 % error in `Z`,
applied to every colour.** Re-predicting lcms2's output with **that one
substitution** (plus the CLUT geometry, so both known differences are
modelled) collapses the disagreement **517×, to 2.1677×10⁻²**.

**Which implementation is right is NOT settled, and settling it is not
available to this project today.** ICC.1:2022 specifies v4; **what a v2
profile's `wtpt` means is corpus ambiguity A4b, UNVERIFIED** —
ICC.1:2022 is *silent* on version 2's convention (confirmed there by
full-text search) and **ICC.1:2001-04 has not been obtained**, being one
of the operator-download items. **lcms2's substitution is justified in
its source by a comment, not by a clause.** Per rule 7 this is a
**finding**, not a failure, and not a verdict in either direction.

#### The problem that forces a decision

A differential has to do *something* with a 11 ΔE disagreement, and the
two obvious things are both wrong:

| Option | Why it was rejected — **in writing, at the record** |
|---|---|
| **Widen the tolerance to ~15 ΔE00 so it passes** | A number chosen **because it passed** — `TOLERANCES.md` §0 and rule 5 exist to forbid exactly that. 15 ΔE00 is **a different colour**, not a tolerance. And it would **silently absorb any future arithmetic error in the absolute path**: the one place the suite would then be blind is the path with the least evidence behind it |
| **Let it fail permanently** | **A red line that never changes stops being read.** It also **misreports the state of knowledge**: the disagreement is not unexplained — its mechanism is known to a factor of 517 — so a failing gate would assert an ignorance the project does not have |

#### The decision

**When a cross-check disagreement has (a) an identified mechanism that
can be modelled, and (b) no available authority to say which side is
right, then:**

1. **The raw comparison is emitted with an INFINITE tolerance and the
   words *REPORTED, NOT GRADED*** — it stays on the record, at full
   size, where it will be seen.
2. **The gate at that point becomes the MODELLED comparison** — the same
   run with the other implementation's policy substituted and **nothing
   else changed** — with a tolerance derived from the residual
   measurement chain, not from the divergence.
3. **Both rejected alternatives are written down** at the record, not
   just the chosen one. A reader must be able to tell *"considered and
   rejected"* from *"never thought of"*.
4. **The blocking question is stated as a question, in full, addressed to
   a named owner** — here `icc-spec-librarian`, and the question is
   whether the v2 specification defines `mediaWhitePointTag` for a
   display-class profile as the *adapted* PCS white (making lcms2's
   substitution a correction of a widely mis-authored field) or as the
   *measured, unadapted* device white (making it the CMM substituting its
   own guess for the file's data).
5. **The row's ungraded status is temporary by construction.** The moment
   A4b is answered, **one of the two implementations acquires a defect**
   and the raw comparison becomes gradeable again.

**This is the only place in the suite where a known disagreement is
deliberately not gated**, and that scarcity is part of the rule: the
posture is available **only** when the mechanism is modelled and the
model is *itself* gated. Without step 2 this would be "we decided not to
check", which is the thing it must never become.

#### Why it is not DL-018 in different clothes

DL-018 concerns a metric dominated by a **deliberate, required cost** —
the answer there is to add a **prediction pin** so the gate stops
rewarding deletion of the requirement. Here the metric is dominated by a
**policy difference whose correctness is unknown**, and there is nothing
to pin a prediction *to*: neither side is established as the right
answer. The shared ancestor of both entries is the same instinct —
**when a number is not error, do not grade it as though it were** — and
the two differ in what replaces the grade: a pin against a computed
prediction (DL-018), or a gate on the modelled substitution plus a
stated open question (this entry).

#### What this entry does NOT decide

- **It does not decide A4b**, and nothing in it prefers iccce's reading.
  **NA-007 remains a named assumption resting on implementation
  consensus**, which the entry that registered it already said *"is not a
  specification reading"*.
- **It does not license reporting-not-grading as a general escape.**
  Steps 1–5 are conjunctive. A disagreement whose mechanism is *not*
  modelled is an **unexplained** disagreement, and the correct response
  to one of those is a failing gate.
- **It does not extend to v4 profiles.** In a **conforming v4** profile
  `wtpt` **shall** already be D50-adapted (9.2.36), from which the
  sourced consequence follows that absolute ≡ media-relative for a
  conforming v4 display profile. **The entire divergence is a v2
  phenomenon**, on files the specification in hand does not govern.

#### ★ One candidate considered at this filing and deliberately NOT filed

**The per-depth `PcsCodec` generalisation** — `lut_transform.rs`
carrying `Lab16Legacy` / `Lab8` / `Xyz16` as the closed (tag type × PCS
kind) product, with the fourth cell (**`lut8` + XYZ PCS**) **refused by
name** as `Lut8XyzPcsUnsourced` because the 8-bit XYZ encoding has no
verified corpus row — **is a good decision and does not need an entry
here.** The reasoning, recorded so the omission is a judgement rather
than an oversight: the rule it enacts is **already in the decision log**
(DL-011: the encoding keys off the **tag type**) and **already sourced**
(A10 resolved for the 8-bit Lab tables); what `b3f4388` added is the
*mechanism* that makes the rule unrepresentable-to-get-wrong, and **a
closed enum with a named refusal variant is self-documenting in a way a
log entry cannot improve on**. The decision log is for choices a reader
could not recover from the code; this one is legible **because** of the
code. `NUMERIC_CLAIMS.md` §2.5 and the ROADMAP's Pass 4 block record it
as delivered work.

**Evidence.** `tools/difftest/README.md` §14.6, §14.7, §14.9;
`tools/difftest/src/pass4.rs` (`ABSOLUTE_REPORTED` and
`WP_POLICY_EMULATED` and their `why` strings — **both read in the live
source by this librarian**, including the record text that names the
`cmsio1.c` mechanism and states A4b as unsourced);
`crates/iccce-cmm/src/matrix_trc.rs` (`Intent::Absolute`, and the
comment *"one is captured as stored (A4b: no adaptation
second-guessed)"*); `docs/NUMERIC_CLAIMS.md` **NC-053**, **NC-054**,
**NA-007**'s dated status; corpus `icc__s__rendering_intents.md` §A4b
and `icc__ref__v2_v4_divergence.md` *(both read this session — **A4b is
UNVERIFIED as of this filing**, and `icc__ref__lcms2_measured_behaviour.md`
carries **M1–M3 only**, so the M5 row this behaviour belongs in does not
exist yet)*. **The run itself is `icc-conformance`'s and is reported;
this librarian ran nothing.** Commits **`490191b`** (the CLI exposing the
intent, without which none of this was reachable) and **`d9e0b82`** (the
differential) *(reported)*.

**Revisit if:** **A4b is answered** — then step 5 fires, the raw rows
become gradeable, one implementation acquires a defect, and this entry
becomes the record of how the question was held open rather than a live
posture; or a **second** instance of the same shape appears, at which
point the rule should be checked against both rather than generalised
from one; or a v4 pair is introduced, where 9.2.36 makes the question
moot.

### DL-020 — a rule the corpus cannot supply at the tier the code needs is **REFUSED BY NAME, not guessed**; and the thing that discharges the refusal is an **independently authored fixture that can fail**, never a second reading. Filed with the arc that demonstrates all of it: **GP-001**, the `mBA ` curve counts, refused **an hour before** the bug the refusal predicted was found

**Date:** 2026-08-11 (Pass 4, the evaluation surface) · **Refusal by:**
`icc-engineer`, during the design of `lut_ab.rs` · **Finding by:**
`icc-conformance`, on the fixture corpus's first run against the shipped
binary · **Filed by:** `icc-librarian` · **Relates to** invariant **§3.2**
(the parser reports, it does not repair), **DL-014** (the terms on which
an ICC.1:2022 clause may be cited, and the evidence-tier discipline that
makes "the corpus cannot supply this" a checkable statement), **DL-005**
and **DL-016** (exact-value assertions, because a ΔE gate cannot see
either failure), and **DL-012** (the other case where a guess about
another party was replaced by a reading)

#### The instance, in order, because the order is the argument

1. **The doubt.** Writing the `mAB `/`mBA ` evaluator, the engineer could
   not reconcile the corpus's rule for curve counts — **one blanket
   sentence covering both tag types**, *"`A` curves = `inputChan`; `B`
   and `M` curves = `outputChan`"*, sitting on byte tables marked
   `icc_secondary_code` with **A23 open** — with the geometry of a tag
   that runs PCS→device. **The evaluator shipped `mAB `-only and refused
   `mBA ` by name.**
2. **The fixture.** Independently, `tools/gen-profiles` authored
   `v4-cmyk-mab-lab.icc` — a v4.4 CMYK Output profile whose `B2A0` is a
   `mBA ` with `inputChan = 3` (Lab) and `outputChan = 4` (CMYK) — from
   layouts transcribed out of the specification, by a crate that
   **depends on nothing**, least of all on iccce.
3. **The finding, within the hour.** The shipped binary refused that
   tag: `curve chain broken at element 3 (byte 68)`. The parser had used
   the `mAB ` convention **for both types**, so it expected four B curves
   where the specification puts three, and walked into the matrix
   element.
4. **The adjudication, in the right order.** The clause text was read
   **first**, from the PDF: **10.13.2/4/6** put `mBA `'s B and M at
   `inputChan` and A at `outputChan`, the mirror of **10.12.2/4/6**.
   lcms2's `Type_LUTB2A_Read` agrees — **recorded as corroboration, not
   as the authority**. **The fixture was not changed to match the
   parser**; the parser was fixed, and the fix carries both clause
   triples in a comment at the site.
5. **The vindication is on the record in the code**, not only in a log:
   *"The refusal was vindicated within the hour — GP-001: the guessed
   counts WOULD have been wrong."*

**Why the defect had survived everything.** The two readings **coincide
whenever `inputChan == outputChan`** — every square LUT — so 40 profiles,
89 declared tests and a full differential run had passed over it. What it
broke was **every real CMYK `B2A0`**: the tag a press profile uses to
print.

#### The decision — five clauses, and they are conjunctive

**When a structural rule the code must obey cannot be established from
the corpus at the tier the code needs:**

1. **Refuse the case by name, in the type system where possible, and
   record the doubt at the site.** Not a `TODO`, not a plausible default
   — a refusal a caller cannot mistake for an answer. In this domain a
   guess produces **colour**, and a wrong colour looks exactly like a
   right one.
2. **The refusal must name what could not be settled**, so that it is a
   *question someone can answer* rather than a limitation someone must
   rediscover. "Refused: curve counts contradictory for this direction"
   is a work item; "unsupported" is a dead end.
3. **A doubt is discharged by an artefact that can fail** — a fixture
   whose bytes were authored **independently of the code under test**,
   from the specification — and **not** by re-reading the same corpus
   sentence, and not by a second opinion from another implementation.
   Rule 3 in `CLAUDE.md` says an expectation taken from the code under
   test detects change, not error; this is the same sentence applied to
   the *sources* an expectation is built from.
4. **When fixture and code disagree, provenance decides which is
   presumed right, in this order: primary clause text, then the
   independently authored fixture, then the code.** **The fixture is not
   edited to make the suite pass.** `tools/gen-profiles/README.md` §5
   states this at the finding: *"The fixture is correct and must not be
   changed to match the parser."*
5. **The corpus sentence that produced the defect is filed as a corpus
   defect, with a named owner**, and stays open until it is transcribed
   per type. A defect fixed only in code leaves the next reader of the
   corpus to make the same mistake.

#### ★ The generalisation this project should carry out of it: a blanket sentence over a mirrored pair is a **defect class**

ICC.1 is full of mirrored pairs — `mAB `/`mBA `, `A2Bx`/`B2Ax`,
device→PCS and PCS→device. **A single sentence that covers both members
of such a pair is a hazard even when it is written carefully**, because
it is **silently right in the symmetric case** and the symmetric case is
what everyone tests with. The corpus rule that follows: **a mirrored pair
gets per-type text with per-type clause numbers, or it gets marked
UNVERIFIED — never one generalisation with two clause numbers appended.**

**The population argument is the second half of it.** The Pass 2 clause-1
record predicted its own blind spot **in writing** — the machine sweep is
*"light or empty on the population Pass 4 depends on — large v4 CMYK
press profiles with `mAB `/`mBA ` pipelines"* — and this fixture is
exactly that population. So: **a coverage claim that names the population
it lacks has, by that act, written the next fixture's specification.**
That sentence was on the record for hours before anything acted on it,
and acting on it took one file.

#### Why this is ONE entry and not three

The refusal discipline (clause 1–2), the fixture as the discharging
artefact (clause 3–4), and the parser's report-don't-repair surface are
**one causal chain**, and each is only checkable against the same
instance. Filing three entries would triple the log's surface while
leaving every one of them resting on GP-001 alone, and would obscure the
thing that actually happened: **the doubt, the artefact and the
disclosure had to hold simultaneously.** Break any one and the outcome
changes — a guess instead of a refusal gives wrong CMYK; no fixture
leaves the guess unfalsified; **a repairing parser resynchronises on the
next plausible curve header and returns colour**, which is the failure
this project is organised against.

#### Candidates considered at this filing and deliberately NOT filed

- **The grayTRC F.2 model.** It is **specification-following**, not a
  decision: the connection scalar times the **full** PCS white triple is
  what the clause says, and the trap it avoids (a green cast from using
  the scalar as `X`/`Z`) is named in the corpus and asserted by a test.
  Nothing here a reader could not recover from the code plus the clause.
  **Its one genuine choice is registered where choices with unmeasured
  costs belong** — `NUMERIC_CLAIMS.md` **NA-008**, the projection of
  non-neutral PCS colour onto the achromatic channel.
- **`LutAbModel`'s `Direction` field.** Same reasoning as DL-019's
  closing note on `PcsCodec`: the *rule* is already logged (the tag type
  carries the direction — Pass 2 batch 2's design note), and a field
  fixed at build time that turns a wrong-direction call into a `None`
  rather than a wrong number is **self-documenting in a way a log entry
  cannot improve on**.
- **The `Chain` wiring of gray and `mAB `/`mBA `.** Mechanical
  application of the already-sourced 8.10.2 fallback. It is recorded as
  delivered work in `ROADMAP.md`, and its evidential status — **wired,
  and exercised by no test** — is recorded in `NUMERIC_CLAIMS.md`
  §3.10.4.

#### What this entry does NOT claim

- **It does not claim the fixture corpus is a validator.** 38 files
  authored by one person from one corpus reading **share whatever that
  reading got wrong**; GP-001 is the case where the reading was right,
  and the opposite case is available. The lcms2 column and the clause
  citations in that README exist for exactly this reason.
- **It does not claim iccce now parses `mBA ` correctly in general.**
  What exists is: the clause text, one fixture, one cross-check point
  (`NUMERIC_CLAIMS.md` **NC-057**), and **no differential in the B2A
  direction at all**.
- **It does not license "refuse it" as a general answer to difficulty.**
  Clause 1 is scoped to a rule the corpus **cannot supply** — verified
  against the corpus, at the tier DL-014 requires — not to a rule
  somebody has not looked up yet.

**Evidence.** `crates/iccce-profile/src/lut.rs` (`decode_lut_ab`'s
per-type counts and the GP-001 comment carrying both clause triples);
`crates/iccce-cmm/src/lut_ab.rs` (the `LutAbModel` HISTORY NOTE, and
`mba_fixture_matches_transicc_recorded_value`, which asserts `K` within
1×10⁻³ of `transicc`'s recorded 0.496117 with its tolerance justified in
the test); `tools/gen-profiles/README.md` **§5** (the verbatim clause
quotations, the lcms2 corroboration, and the named corpus gap) and **§6**
(the verification matrix) — **all read in the live source by this
librarian**, which is also how the two stale statements in that README
(**§5's `Status: open`** and §6.1's `B2A0 REFUSED` row) were found and
recorded as *reported, not repaired*. **This librarian has not opened
`ICC.1-2022-05.pdf`**: every clause quotation above is
`icc-conformance`'s direct read, carried with that attribution. Corpus
`icc__type__lutAtoB_lutBtoA.md`'s blanket sentence **verified still
present**. Commits **`7576cfa`** (corpus + finding) and **`2e98cfd`**
(fix + evaluator) *(reported)*.

**Revisit if:** the corpus gains per-type transcriptions of 10.12.x /
10.13.x and closes **A23** — then clause 5 is discharged for this
instance and the entry becomes history rather than a live debt; or a
**second** doubt is refused and later settled by a fixture, at which
point clauses 1–4 should be checked against both instances rather than
generalised from one; or a fixture is ever edited to make a test pass, in
which case clause 4 has been broken and the reason must be recorded here.

> **★ Dated status, 2026-08-11 (Pass 4b filing): the first revisit
> condition has FIRED, and clause 5 is discharged for this instance.**
> The corpus's **seventh** pass replaced the blanket sentence with **six
> verbatim clause sentences** (10.12.2/4/6 and 10.13.2/4/6) and an
> implementable per-type table, **retracted the old rule verbatim** and
> filed it as spec-defect **C4**, and closed **A23** (permitted element
> combinations enumerated) and **A25**, with **A24** closed for `mBA `
> and partial for `mAB ` *(verified — `icc__type__lutAtoB_lutBtoA.md`
> §§1–2 and the ambiguity register read 2026-08-11)*. **The entry above
> is not edited**; this note is how it is corrected. The generalisation
> in the ★ section — *a blanket sentence over a mirrored pair is a defect
> class* — is **not** discharged by this; it is the rule the corpus now
> follows, and **DL-021** is its second instance from the other side of
> the same day.

### DL-021 — **a measured implementation behaviour is a fact about the direction and the path it was measured in, until it is measured in the others.** Filed with three instances from one day, all in the same oracle, all of which had already been written down as unqualified rules

**Date:** 2026-08-11 (Pass 4b) · **Measurements by:** `icc-conformance`
· **Filed by:** `icc-librarian` · **Relates to** **DL-012** (a predicted
disagreement measured *absent*), **DL-013** (the rule this entry shows to
be half-stated), **DL-018** (an apparatus must be shown able to see the
effect it looks for), **DL-020** (a blanket sentence over a mirrored pair
is a defect class — the corpus-side twin of this rule), and
`NUMERIC_CLAIMS.md` **NA-006**, **NC-067**, **NC-078**, §3.11.2–§3.11.3

#### The three instances, and what each of them had been written down as

| # | What this project had written | What was measured on 2026-08-11 | Where it was wrong |
|---|---|---|---|
| **1** | *"iccce interpolates n-linear, lcms2 tetrahedral"*, and later, after Pass 4 priced it: **"NA-006 costs up to 1,5741 ΔE2000 against lcms2."** | `cmsio1.c`'s `_cmsReadOutputLUT` calls **`ChangeInterpolationToTrilinear`** for **every Lab-PCS LUT**, and trilinear over three inputs **is** iccce's n-linear. **The interpolation-method envelope is identically ZERO in the B2A direction** (**NC-067**; the counterfactual prices what the comparison could have seen at **99–139×**). | The number is an **A2B** number. The same approximation costs 1,5741 in one direction and **exactly nothing** in the other, on the same profile, at the same pin. |
| **2** | **DL-013** and corpus **M2**: *"lcms2 forces BPC on for **v4 profiles** at perceptual and saturation."* | `_cmsLinkProfiles` sets `BPC[i]` per profile, but `DefaultICCintents` consumes it as `ComputeConversion(i, …, BPC[i], …)` — the conversion **into** `hProfiles[i]`. **The DESTINATION profile's version decides.** v4 source into a v2 destination: **0,0, bit-identical**. v2 source into a v4 destination: **3,137×10⁻²** device (**NC-078**). | *"v4 profiles"* names a **pair member without a role**. Anyone using M2 to decide whether a comparison is confounded needs the **direction**, and half of them would get it wrong. |
| **3** | *"lcms2 keys the legacy 16-bit PCSLAB encoding off the **tag type**"* (**DL-011**'s rule, **DL-012**'s measurement, both correct) | The same function inserts `_cmsStageAllocLabV4ToV2` **only when `OriginalType == cmsSigLut16Type`** — so on a **`lut8Type`** B2A the legacy scale is **not** applied, and iccce's `Lab8` codec agrees exactly. Had iccce applied the legacy scale there, `L*` would be **0,39 % low ≈ 0,2 ΔE2000** — *below* the perceptibility anchor and **invisible to any ΔE-graded suite**. | The rule was right and was **verified on one tag type**. Its correctness on the *other* member of the same family was an inference until Pass 4b measured it. |

**All three live in the same file of the same oracle. Two of them were
in this project's documents as unqualified rules for hours or days**, and
in each case the qualification was recoverable by one reading of the
source that nobody had done in that direction.

#### The decision

**When a behaviour of another implementation is measured or read, the
record states the direction and the path it was established on, and any
statement about the other direction or path is marked as an inference
until it too is measured.** Concretely:

1. **A record's scope line names the direction** — device→PCS or
   PCS→device, source or destination, and which tag type — **not just
   the profile, the pin and the intent.** A scope line that omits the
   direction is incomplete in the same way one that omits the pin is.
2. **When the object has a mirrored twin, assume the twin differs until
   measured.** ICC.1 is built out of mirrored pairs (`A2Bx`/`B2Ax`,
   `mAB `/`mBA `, `lut8`/`lut16`, source/destination), and an
   implementation is free to treat the two members differently — lcms2
   demonstrably does, in **three** places in one file.
3. **A cost measured in one direction is quoted with that direction
   attached**, permanently. *"n-linear costs 1,57 ΔE"* is not a claim
   this project may make; *"1,57 ΔE in the A2B direction on this
   profile's perceptual table, zero in the B2A direction because lcms2
   forces trilinear there"* is.
4. **When the method difference collapses to zero, the comparison gets
   weaker, not stronger — and a counterfactual must say by how much.**
   Agreement between two implementations that are running the *same*
   algorithm is not evidence that the algorithm is right. **NC-067 is
   the required shape**: the same table evaluated the other way, ungraded,
   showing the comparison could have seen a difference **99–139×** larger.
   This is **DL-018's rule** transplanted from a deleted requirement to a
   method.
5. **A prediction about the untested direction is filed as a
   prediction**, in the register, so that measuring it later either
   confirms or corrects something concrete. DL-012 and this entry are
   both cases where the prediction was the thing that turned out to be
   wrong.

#### Why this is one entry and not a note under NA-006

Because it is **not a fact about interpolation.** The same failure
appeared in three unrelated mechanisms — an interpolator factory, a link
flag, and an encoding stage — within one working day, and the common
cause is a habit of writing down *"lcms2 does X"* when what was observed
is *"lcms2 did X here."* A note under NA-006 would fix one number and
leave the habit, and the habit is what produced the other two.

#### What this entry does NOT claim

- **It does not say lcms2 is inconsistent.** Each of the three
  behaviours has a rationale in its own place (`_cmsReadOutputLUT`'s
  comment argues that a Lab indexer space wants trilinear; the link flag
  is consumed exactly where a conversion is built). **The defect is in
  this project's transcription of them, not in lcms2.**
- **It does not require every rule to be measured in every direction
  before it may be used.** It requires the untested direction to be
  **labelled** — the same standing as any other inference in these
  documents.
- **It does not apply only to lcms2.** The corpus's own
  `icc__type__lutAtoB_lutBtoA.md` produced GP-001 by exactly this route,
  which is why **DL-020's** mirrored-pair rule and this one are two
  halves of one lesson: **DL-020 governs how a *specification* is
  transcribed; DL-021 governs how an *implementation's behaviour* is.**

**Evidence.** `tools/difftest/README.md` **§15.2.2** (the verbatim
`ChangeInterpolationToTrilinear` quotation and its consequences),
**§15.2.3** (the `lut8`/`lut16` encoding split), **§15.3.4** (the
destination-version BPC measurement and its `_cmsLinkProfiles` /
`DefaultICCintents` mechanism) and **§15.5** (the 28 emitted records) —
**all read in the live source by this librarian**;
`docs/TOLERANCES.md` §3.4.4.2 row A5 and §3.4.4.3 row B8 agree on every
number *(verified — read; both files are `icc-conformance`'s and neither
was edited)*. **The lcms2 source readings are `icc-conformance`'s: this
librarian has neither built nor read lcms2**, and every C quotation above
is carried with that attribution. Ledger rows
`NUMERIC_CLAIMS.md` **NC-062 … NC-083**, and the dated note under
**NA-006**. Commits **`9e2e29e`**, **`a0310c7`**, **`3d0c183`**
*(reported — no agent in this project has ever run git)*.

**Revisit if:** a fourth instance appears (the entry should then stop
listing instances and simply be the rule); or lcms2's pin moves, at which
point **all three instances must be re-measured, not re-read** — a
retuned interpolator factory or a moved link flag would invalidate them
**silently**, because the transcriptions would keep reproducing the old
behaviour perfectly.

### DL-022 — **iccce NEVER forces black point compensation; it is an explicit caller act.** A deliberate, measured divergence from the oracle with a **user-visible** consequence, filed as a decision because it can no longer be carried as a note

**Date:** 2026-08-11 (Pass 5 completion) · **Measurement by:**
`icc-conformance` · **Filed by:** `icc-librarian` · **Relates to**
**DL-013** (lcms2's forcing, first recorded), **DL-019** (report-not-grade
when the mechanism is known and the authority is not), **DL-021**
(instance 2 — the forcing is keyed by the **destination**), and
`NUMERIC_CLAIMS.md` **NC-020**, **NC-078**, **NC-100**, **NA-009**

#### The decision

**`iccce` applies BPC if and only if the caller asks for it**
(`Chain::with_bpc()`, reached from the shipped binary as
`iccce transform --bpc`). **It does not force BPC on for any profile
version, any intent, or any combination of the two.** lcms2 does: for a
**v4 destination** at perceptual or saturation it sets the flag
unconditionally, **overriding the caller** — `_cmsLinkProfiles` writes
`BPC[i] = TRUE` before the caller's flag is ever read, which is why
asking and not asking produce **bit-identical** output there
(**NC-095**, graded at exactly 0,0).

#### Why this is a decision and not a defect on either side

| | |
|---|---|
| **What lcms2 rests on** | A source comment attributing the policy to **Adobe's document**. **Nobody in this project has read that document.** It is `AdobeBPC.pdf` / ICC WP40 / ISO 18619, and it is **ToS-barred or blocked to agent tools** — an operator browser download |
| **What the one published BPC source says** | **Maria (2013)** corroborates the **exclusion** set (absolute, devicelink, abstract — the ground for `BpcNotApplicable`, and the basis of **NC-104**) and is **silent on the enable policy**. It discusses the v4 fixed black only as the easy case of black-point *detection*, **never** as a reason to override a caller (`ICC_Spec` §7.1) |
| **What ICC.1:2022 says** | The **scaling map** is there, at **6.3.4.3**, under another name (**NC-084**). **The applicability is not.** There is no clause that says when BPC *shall* be applied |
| **What it costs, measured** | **3,137 348 `L*`** at black on one pair (sRGB → `v4-cmyk-mab-lab.icc`, perceptual), lcms2 **lighter**; **NC-100**. The sign is diagnosed, not tolerated: it matches the corpus's **D11** fingerprint to **1,1×10⁻⁴ `L*`** and identifies **lcms2's M2 route**, not **iccDEV's** — the two being distinguishable in the opposite direction, which was measured (**NC-093**) |

**So the disagreement's mechanism is fully identified and its authority
does not exist.** That is exactly **DL-019**'s condition, and the
consequence is the same: **NC-100 is REPORTED, NOT GRADED.** The two
available gradings were considered and **both rejected in writing** — a
~3,2 `L*` tolerance would be a number chosen because it passed (rule 5),
and a permanent red line would assert a verdict no obtainable document
supports (rule 7).

#### The three things that make it a decision-log entry rather than a note

1. **It has a user-visible consequence.** This is not an internal
   approximation. **Two correct CMMs give different pictures by
   default**, through a flag on a shipped binary: someone converting
   sRGB into a v4 profile at perceptual gets a lighter black from
   `transicc` than from `iccce`, with no error, no warning and nothing
   in either output to indicate why. **Rule 1 in its purest form** — and
   the reason the difference is *documented at the call site* rather
   than merely tolerated.
2. **It contaminates every comparison in the Pass.** *"A comparison that
   does not account for it measures iccce's **policy** and reports the
   answer as a **tolerance**."* Pass 5 accounts for it by running the
   cross-checks **`--bpc` against `-b`** and separately reporting the
   unasked-against-unasked arm. **Any future Pass that compares BPC
   without doing this will produce a wrong tolerance that looks fine.**
3. **It was already written down twice in weaker places and that was
   not enough.** It lived as a paragraph inside **NA-009** (*"recorded
   here rather than minted as its own entry"*) and as a sentence in
   `bpc.rs`'s field doc. **Neither is where a reader looks for a
   deliberate divergence from the oracle**, and neither could carry the
   measured size, because at the time there was none.

#### What this entry does NOT claim

- **It does not say lcms2 is wrong.** Its behaviour may be exactly what
  the unread document specifies. **Rule 7 cuts both ways**: a
  disagreement with lcms2 is a finding, and a finding is not a verdict.
- **It does not say iccce's default is better colour.** For a v4
  perceptual destination, forcing BPC may well produce the *more useful*
  result. The claim is narrower and is about **authority**: iccce will
  not override a caller on the strength of a document nobody here has
  read.
- **It does not settle whether BPC should be forced.** It records that
  the question is open, who could close it, and what it costs while it
  stays open.
- **It is not a claim about saturation.** iccce's estimation subset
  admits **only perceptual** on a LUT side, so the saturation arm has no
  iccce half at all — the policy difference there is **unmeasured**.

**Evidence.** `tools/difftest/README.md` **§16.4.2** (the policy, with
its mechanism), **§16.4.3** (the D11 fingerprint answered in both
directions), **§16.1** (both implementations' reach, tabulated from
their sources); `docs/TOLERANCES.md` **§3.5.4 row P16**, which agrees on
every figure *(verified — read; the file is `icc-conformance`'s and was
not edited)*; `crates/iccce-cmm/src/bpc.rs` and
`crates/iccce-cli/src/main.rs` *(verified — the `--bpc` flag and its
refusal path read)*. Ledger rows **NC-093**, **NC-095**, **NC-100**, and
the dated note under **NA-009**. Commits **`46f16e8`**, **`df3a233`**
*(reported — no agent in this project has ever run git)*.

**Revisit if:** `AdobeBPC.pdf` / ICC WP40 / ISO 18619 is obtained — at
which point **NC-100 becomes gradable in one direction or the other, and
this entry either gains a clause or is reversed by a new entry**; or
iccce's estimation subset widens to admit saturation (a second place to
measure the same policy); or lcms2's pin moves, since the forcing is a
behaviour at a pin (**re-run, not re-read**).

### DL-023 — **before a cross-check is graded, state what the two implementations were FREE to disagree about — derived from their sources, before the run.** A pre-registered negative result is a finding; a small residual discovered afterwards is not

**Date:** 2026-08-11 (Pass 5 completion) · **Method by:**
`icc-conformance` · **Filed by:** `icc-librarian` · **Relates to**
**DL-018** (an apparatus must be shown able to see the effect it looks
for), **DL-021** (a behaviour is a fact about one direction), **DL-012**
(a predicted disagreement measured absent), and `NUMERIC_CLAIMS.md`
§3.12.3, **NC-067**, **NA-009**

#### The decision

**Every comparison between iccce and another implementation states, in
advance and from both sides' sources, which of the quantities under test
the two were actually free to differ on.** Where the answer is *"none"*
for some component, **that component's agreement is not evidence and the
record says so** — as a **pre-registered negative result**, not as a
caveat appended after a suspiciously small number.

Concretely, three obligations:

1. **Read both reaches before running anything.** Pass 5 tabulated
   `Chain::with_bpc`'s applicability subset against lcms2's six
   first-match-wins black-point guards **at the pin**, and derived the
   scenario set from the intersection.
2. **Publish the negative result the intersection produces**, at the top
   of the coverage statement rather than in a footnote. Pass 5's is:
   **everywhere iccce will do BPC at all, lcms2's estimator reduces to
   the same two values** — `XYZ (0,0,0)` on every matrix/TRC or gray
   side in reach (every TRC in the corpus has `trc(0) = 0`) and the same
   **A41** triple on a v4 LUT side at perceptual. **So Pass 5's rows
   grade the scaling map, the direction and the pipeline; they do not
   discriminate the two ESTIMATORS, and no row may be quoted as if they
   did.**
3. **Name the instrument that would close the gap**, so the gap is an
   item of work rather than a permanent hedge. Pass 5's is **a synthetic
   v4 RGB-or-gray LUT fixture with a non-zero device black** — the same
   shape as **DL-020**: a doubt the corpus cannot discharge, discharged
   by bytes this project authors.

**And its cheap companion, which Pass 5 also demonstrates: state the
sensitivity ratio.** *"iccce and lcms2 agree to 1,1×10⁻⁴"* means nothing
until it sits beside *"BPC itself moves this transform by 3,5159
ΔE2000"* — **388×**, and **682×** in the other direction. **A comparison
that cannot state such a ratio has not shown that it could have
failed.** Where the effect being graded has a natural "off" arm, the
ratio is **free**, because the off arm is already run as the baseline
(**NC-089**).

#### Why this is not already covered by DL-018 or DL-021

- **DL-018** requires a **prediction pin** so that *deleting a
  requirement* cannot make a gate greener. It is about the **gate**.
- **DL-021** requires a measured behaviour to carry its **direction**.
  It is about **scope**.
- **DL-023 is about what the comparison is capable of distinguishing at
  all** — a property of the *scenario set*, fixed before any tolerance
  is chosen and before any number exists. **A suite can satisfy both
  earlier rules perfectly and still consist entirely of comparisons that
  could not have failed.** NC-067's counterfactual was the first sign of
  this (a method difference collapsing to zero); Pass 5 is the case
  where the collapse covers a whole *rule* of the feature, and where it
  was **predicted rather than discovered**.

#### What this entry does NOT claim

- **It does not devalue the agreeing rows.** NC-090 and NC-096 are real
  evidence about the map, the direction and the pipeline — and they are
  388× and 682× more sensitive than the effect they grade. **What they
  are not is evidence about estimation.**
- **It does not require an instrument to exist before a Pass may
  close.** Pass 5's done-when is **MET**; the gap is stated in the same
  breath. **A done-when is met on its terms, and the terms are part of
  the claim.**
- **It does not make "we agreed" suspicious by default.** It makes
  *unexamined* agreement suspicious. The remedy is a reading of two
  sources, done once, before the run — which in this Pass cost less than
  the run did.

**Evidence.** `tools/difftest/README.md` **§16.1** (the scenario set,
with a prediction column written before each run and **all six
confirmed** — *reported*), **§16.7** paragraph (C) (the coverage
statement carrying the negative result), **§16.8** item 4 (the fixture
owed); `docs/TOLERANCES.md` **§3.5.1** and **§6.5** item 1, which state
the same rule in `icc-conformance`'s own words *(verified — read; not
edited)*; `tools/difftest/src/pass5.rs`'s module header, which tabulates
both implementations' reach *(verified — read)*. Ledger §3.12.3.
Commit **`df3a233`** *(reported)*.

**Revisit if:** a Pass produces an agreement claim **without** stating
what the two sides were free to disagree about (the entry should then
gain the instance); or the non-zero-black fixture is authored, at which
point Pass 5's negative result becomes a **measurable** question and
**NA-009's cost comes due for the first time**.

---

### DL-024 — ★ **the project is published: `master` was pushed to `github.com/KenM76/iccce` on the operator's explicit go-ahead.** The event, the evidence for it, the pre-publication checks, and the sharp line between what was verified and what is the operator's word

**Date:** 2026-08-12 · **Authorised by:** the operator (Ken), explicit
current go-ahead *(reported by the dispatching engineer)* · **Performed
by:** `icc-engineer` · **Filed by:** `icc-librarian` · **Relates to**
**DL-009** (publication to crates.io is *intended*; the intent
authorises nothing), **DL-001** and **DL-002** (the two things that had
to stay out of an MIT repository), `docs/LEGAL.md` **§2.1** and **§3**,
`ROADMAP.md` *"Publication — crates.io"*, and project rule 9

#### The decision, and whose it was

**Rule 9 says publishing is the operator's act.** `ROADMAP.md` has
carried, since the scope questions were answered, the sentence *"'We
decided in August that we'd publish eventually' is not that go-ahead,
and no agent may treat this section as one."* That sentence is
unchanged and still governs. **This entry records that the go-ahead
itself arrived** — as a current, explicit instruction — and that the
push was performed under it.

**The entry does not claim the go-ahead's wording.** No agent here
holds the operator's exact words for it; the dispatching engineer
reports that it was given and was explicit. That is the same evidential
footing as *"1 is done"* in the scope-questions block, and it is
recorded at that strength and no higher.

#### ★ The event, from files this librarian read directly

**This is the first time in the project's history that any statement
about its own repository rests on something other than a report.**
`icc-librarian` has no shell and ran no git command; the following was
read out of plain-text files under `D:\Dev\iccce\.git\` with the ordinary
file reader, which is a different act from running `git`:

| What was read | What it says |
|---|---|
| `.git/config` | `[remote "origin"]` · `url = https://github.com/KenM76/iccce.git` — so the declared `repository` in the workspace manifest and the configured remote are the same URL. *(verified — read)* |
| `.git/logs/refs/remotes/origin/master` | **Exactly two lines, both `update by push`.** (1) `0000000…` → **`3502cb70…`** at epoch **`1786531877 -0400`** = **2026-08-12 06:51:17 −04:00**. The all-zero left-hand side means **the branch did not exist on the remote before this line** — *this line is the publication event*, and it carried the whole history up to and including Pass 6. (2) **`3502cb70…` → `f6203b8d…`** at epoch **`1786532090 -0400`** = **2026-08-12 06:54:50 −04:00** — Pass 7, pushed three seconds after it was committed. *(verified — read)* |
| `.git/refs/heads/master` and `.git/refs/remotes/origin/master` | Both hold **`f6203b8df878f5af3a9bd9f8269e9bc3a2cac8aa`** — local and remote-tracking tips are the same commit, so nothing local is unpushed as of this reading. *(verified — read)* |
| `.git/logs/HEAD` | **45 lines**: one `commit (initial)` (`f976a0e…`, epoch `1786458320 -0400` = 2026-08-11 10:25:20 −04:00) and 44 further `commit` lines. **No `reset`, `rebase`, `amend`, `checkout` or `merge` entry appears anywhere in the file.** *(verified — read end to end)* |

#### ★ What that evidence establishes, and three things it does not

**Establishes:** that a push to that URL succeeded, twice, at those two
moments, and that the remote branch was created by the first of them.

**Does NOT establish that the repository is public.** Repository
visibility is a setting on GitHub's servers. **No file in this working
tree records it**, and a push to a private repository produces exactly
the same reflog. **That `github.com/KenM76/iccce` is public is the
operator's report** and is carried at that strength everywhere in this
filing. Anyone who needs it settled must open the URL in a browser
while signed out.

**Does NOT establish the commit count.** The dispatch reports **49
commits**. The reflog records **45** HEAD advances from the initial
commit to `f6203b8`, with no history-rewriting entry — which implies 45
commits on `master`, *on the assumption* that no commit was authored in
another clone and fetched (nothing evidences another clone) and that the
reflog has not been pruned (implausible for a repository one day old).
**45 ≠ 49, nobody here has run `git log`, and this entry asserts
neither number as the truth.** It records the discrepancy so that
whoever next has a shell can settle it in one command. See
`NUMERIC_CLAIMS.md` §2.9.

**Does NOT retire "no agent here has ever run git."** It is still true.
What changed is narrower and worth stating precisely: **the repository's
own log files are now a readable source**, and they *corroborate* the
commit hashes that eight provenance blocks have carried as reports. That
is a real strengthening of the ledger's provenance — and it immediately
found an error (below).

#### ★ The first thing the new evidence caught: a wrong hash this project has carried three times

`.git/logs/HEAD` line 25 records the commit *"untrack tools/gen-profiles:
in-progress agent work committed early"* as **`edce48bf32c81d472f5361f37147cb65523e8e67`** —
**`edce48b`**. Three documents carry it as **`edcb60e`**
(`ROADMAP.md` lines 1433 and 1705, `SESSION_LOG.md` lines 1406, 1590 and
2052, `NEXT_SESSION.md`'s commit table), and one carries it correctly:
`NUMERIC_CLAIMS.md` §2.6's platform row says `edce48b`, because it came
by a different route — a transcription of `gen-profiles/README.md` §6.
**`edcb60e` matches no prefix in the reflog.** *(all verified — read.)*

**The correction is filed, not applied retroactively.** ROADMAP and
SESSION_LOG are append-only; their old text stands and a dated
correction sits with this filing. `NEXT_SESSION.md` is rewritten every
session and carries the right hash from now on. **The lesson is the one
this project keeps relearning:** a hash typed from a screen is a claim,
it looks exactly like a right one, and it survives three documents
because nothing could check it. Something can now.

#### The pre-publication checks — which were verified here and which were not

The dispatch reports three checks run before the push. **They are not
equal in how far this librarian could confirm them**, and rounding them
into one "checked" would be exactly the failure this project files
entries about.

| Check | Status here |
|---|---|
| **No vendor profiles tracked** | **★ VERIFIED, and stronger than the dispatch put it.** Every `.icc`/`.icm` in the tree was enumerated: **38 in `fixtures/synthetic/`**, all generated by `tools/gen-profiles`; the rest are under `tools/difftest/vendor/` (git-ignored by name) and `tools/difftest/out/` (caught by `.gitignore`'s `*.icc` rule, which is negated only for `fixtures/**`). **`fixtures/reference/PROVENANCE.md`'s table reads *"(none yet)"*** — no third-party profile has ever been admitted, so there is nothing for the check to have missed. *(verified — globbed and read.)* |
| **The corpus is not in the repo** | **VERIFIED, as far as the file tree goes.** `D:\Dev\Rag-Specialized\ICC_Spec\` is outside this working tree and no path under it appears in it. That is what LEGAL §2.1 requires (*"`ICC_Spec` must stay on local disk … never in a git remote"*, on the ToS phrase *"any other … networked computer environment"*). **What was not checked: whether any corpus file was ever committed and later removed** — a deleted file stays in history, and that is a `git log`-shaped question nobody here can ask. |
| **Spec quotation is short-with-citation (LEGAL §2.1)** | **REPORTED — and this is the one that must not be rounded up.** LEGAL §2.1's operative rule was verified as text: *"Summarise and cite. Short verbatim quotation with a clause citation is normal technical-reference practice … bulk verbatim transcription of clause text is not."* **That the whole tree complies with it has NOT been audited by this librarian**, and no such audit exists anywhere. It is a claim about every doc comment in `crates/`, every line of five documents in `docs/`, and `tools/difftest/README.md`. **DL-014's citation audit — owed since Pass 3 and still owed — is the audit that would settle it**, and it has now acquired a second reason to exist. |

#### What this entry does NOT authorise

- **It is not a crates.io publish, and does not become one.** DL-009's
  intent is unchanged and still unexercised: **no crate has been
  published**, crate-name availability on crates.io is **still
  unchecked** by anyone, and `THIRD_PARTY_LICENSES.md` via `cargo-about`
  is **still owed before a first publish**. A public git repository and
  a published crate are different acts needing different go-aheads.
- **It is not a tag or a release.** Neither exists; neither is
  authorised.
- **It does not lower rule 9 for anything else.** The next push, tag or
  release needs its own current go-ahead. *"He said yes on the 12th"* is
  not a standing permission, for exactly the reason *"we decided in
  August"* was not.
- **It does not make the numbers in this repository stronger.** A
  stranger can now read `NUMERIC_CLAIMS.md` — which is the point of it —
  and everything §1 says about evidence classes still holds. **The
  project still has zero `published-ground-truth` rows for any
  transform**, and publication does not change that by one row.

#### What publication changes about how this project writes

Three consequences, all of them now live rather than prospective:

1. **The audience is no longer the four agents and the operator.** Every
   hedge in these documents is now read by people who cannot ask what
   was meant. The ledger's habit of stating coverage inside the claim
   was written for exactly this reader.
2. **A wrong claim is now a published wrong claim.** The
   claim-bearing-copy discipline (verify the source, never invent a
   plausible policy) applies to `README.md` badges and prose from today,
   not from first publish.
3. **The oracle's insulation is now load-bearing in public.**
   `tools/difftest` is not a workspace member and `vendor/` is ignored,
   which keeps GPL-3.0 lcms2 plugin source out of an MIT repository
   (DL-001, LEGAL §4). **A future tidy-up that folds difftest into the
   workspace would now do that in public.**

**Evidence.** `.git/config`, `.git/logs/refs/remotes/origin/master`,
`.git/logs/HEAD`, `.git/refs/heads/master`,
`.git/refs/remotes/origin/master` *(all verified — read)*;
`.gitignore`, `fixtures/reference/PROVENANCE.md`, `docs/LEGAL.md` §2.1
*(all verified — read)*; the fixture enumeration *(verified — globbed)*.
The operator's go-ahead and the repository's public visibility are
**reported**. `NUMERIC_CLAIMS.md` §2.9; `SESSION_LOG.md`, 2026-08-12.

**Revisit if:** the repository's visibility is ever changed; a crate is
actually published to crates.io (which needs its own entry, not an
amendment to this one); a third-party profile is proposed for
`fixtures/reference/`; or the DL-014 audit finds a quotation that
exceeds LEGAL §2.1's rule — in which case the third pre-publication
check moves from *reported* to *failed*, in public, and this entry gains
the dated instance.

---

### DL-025 — ★ **a sensitivity control is only as good as its FIXTURE, and the scaling law it asserts must match the SMOOTHNESS CLASS of the function under test.** Filed with the two consecutive failures that taught both halves, in the one Pass whose headline number would otherwise have been a spectacular measurement of nothing

**Date:** 2026-08-12 (Pass 6 completion) · **Found by:** `icc-engineer`,
by its own control failing twice · **Filed by:** `icc-librarian` ·
**Relates to** **DL-018** (an upper-bound gate on a deliberate cost needs
a prediction pin and a sensitivity control), **DL-023** (state what the
two arms were free to disagree about, before the run), **DL-016**
(sampled tables asserted by exact values), **DL-020** (a refusal
discharged by a fixture that can fail), and `NUMERIC_CLAIMS.md` §3.13

#### The decision

**Two obligations, on top of DL-018's requirement that a control
exist.** A control is an experiment, and an experiment can be null for
reasons that have nothing to do with the code:

1. **The control's FIXTURE must be one on which the effect is
   present.** DL-018 asks *"would this gate get greener if the
   requirement were deleted?"* DL-023 asks *"what were the two arms free
   to disagree about?"* **Neither question is answered by the code alone
   — both are answered by the code *and the fixture together*.** A
   control run on a fixture where the effect is identically zero
   satisfies both rules on paper and measures nothing.
2. **The scaling law the control asserts must match the function's
   smoothness class.** An error model of the form *"halving `h` should
   cut the error 4×"* is a statement about a **twice-differentiable**
   function. Probing across a derivative discontinuity gives `h¹`, not
   `h²` — and the resulting failure is the *control being right about
   the wrong region*, not the code being wrong.

**Both are checked before the control's number is quoted, and both
belong in the control's own doc comment, in the form "this failed, here
is why" rather than as a rule stated in the abstract.**

#### The two failures, in the order they happened, because the pair is the whole entry

**Failure 1 — the fixture nullified the control.** The first draft of
`compiled.rs`'s `error_scales_with_grid_spacing` used **sRGB → sRGB**.
A compiled grid does not merely reproduce an identity chain *at its
nodes*; **n-linear interpolation is exact on a linear function, so it
reproduces it everywhere**. The measured error came out at
**1.1×10⁻¹⁵** with a spacing ratio of **0.94** — pure `f64` noise, no
`h²` scaling, no discrimination whatever.

> **★ Had the control not existed, that 1.1×10⁻¹⁵ would have been
> reported as "the compiled path's cost."** It is a spectacular number.
> It is also *precisely* the failure DL-023 predicted for this Pass —
> the ROADMAP's second Pass 6 annotation, written at the Pass 5 filing,
> says a compiled transform built by sampling its reference is *"the
> most likely null-by-construction row this project will ever write."*
> **The prediction was correct and the trap was walked into anyway**,
> which is the argument for mechanical controls over remembered rules.

The fixture was changed to **sRGB → AdobeRGB**: different TRCs, so the
composite is genuinely curved in device space and a grid can be wrong
about it.

**Failure 2 — the scaling law did not match the function.** The
refixtured control probed the whole axis and returned a ratio of
**1.44** against an `h²` prediction of 4. **The code was not wrong and
the fixture was not wrong.** sRGB's TRC joins a linear segment to a
power curve at `0.04045` — a derivative discontinuity — and
interpolation error across a kink scales as `h¹`. The remedy was to
probe the **smooth region** (`[0.2, 0.9]`, chosen off-node for both the
5- and 9-point grids), where `h²` is the right prediction to test
against. The accepted band is **2×–8×**: wide enough for a real curve's
varying curvature, narrow enough to fail if the compiled arm were
secretly the reference arm (ratio → 1) or the probes were accidentally
on-node (both errors zero).

**Both failures are recorded in the test's own doc comment** *(verified
— read)*, which is where the next person to touch the fixture will
actually see them.

#### ★ The observation this entry exists to make

**This is the third time in two days that a control or a test caught
something a competent engineer was about to ship, and each time the
catching instrument was of a different kind:**

| Instance | What was about to ship | What caught it | Filed as |
|---|---|---|---|
| Pass 3 | A sampled-curve evaluator **off by one sample** | An **exact-value test at the sample points**. The self-consistency round trip **would have passed with the bug in place** | **DL-016** |
| Pass 4 | An `mBA ` curve-count reading the corpus could not supply | A **refusal by name**, discharged an hour later by an **independently authored fixture** that could fail — **GP-001** | **DL-020** |
| Pass 6 | A compiled-path error of **1.1×10⁻¹⁵** that measured nothing | A **sensitivity control**, which failed on its own fixture | **this entry** |

**The generalisation is not "be careful."** It is that in this project
the thing that catches an error is **never** a re-reading of the code
that contains it, and **never** the number looking wrong — a 10⁻¹⁵ looks
magnificent. It is always an **instrument constructed to fail**: an
exact expected value, a fixture authored elsewhere, a scaling law with a
predicted magnitude. Project rule 1 — *a wrong colour looks exactly like
a right one* — has a corollary this project can now state from three
worked instances: **a wrong measurement looks exactly like a right one,
and the only defence is an apparatus that could have said no.**

#### Why this is not already covered by DL-018 or DL-023

- **DL-018** requires a sensitivity control to **exist**. Both of Pass
  6's failures happened *inside a control that existed and was run.*
- **DL-023** requires stating what the two arms were free to differ on
  **before the run**. Pass 6 stated it — the ROADMAP annotation says it
  in as many words — **and still shipped an identity fixture into the
  control.** The statement was about the *method*; the nullity was in
  the *fixture*.
- **What is new here** is that the control is itself an experiment with
  its own validity conditions, and that **its failures are evidence to
  be published rather than debris to be cleaned up**. Failure 2 in
  particular is a *correct* control disagreeing with an *incorrect*
  expectation — the same shape as `TOLERANCES.md` §4's corrected
  derivations, and it must not be mistaken for a tolerance being tuned.

#### What this entry does NOT claim

- **It does not claim the compiled path is correct.** The error it
  finally measures is a **`self-consistency`** row (`NUMERIC_CLAIMS.md`
  §1) — both arms are iccce — and is worthless as correctness evidence
  however small. What the control buys is only that the number is *about
  something*.
- **It does not generalise across direction or tag type.** DL-021 still
  binds: the control ran on **sRGB → AdobeRGB matrix/TRC**, and the cost
  row on **SWOP `A2B1` (`mft2`, 4-D) → sRGB, media-relative**. Neither
  says anything about the B2A direction of either pair.
- **It does not claim every control in this project has been checked
  this way.** It has not. The ones that should be re-read against both
  obligations are named as owed work in `NUMERIC_CLAIMS.md` §7.9.

**Evidence.** `crates/iccce-cmm/src/compiled.rs` — the module header's
*"What the two arms are free to disagree about (DL-023)"* section, and
the doc comments on `tests::error_scales_with_grid_spacing` (both
failures, with the 1.1×10⁻¹⁵/0.94 and 1.44 figures) and
`tests::identical_at_nodes_by_construction` (*"STRUCTURAL, NOT
EVIDENCE"*) *(all verified — read)*; `crates/iccce-cli/src/main.rs`
`cmd_bench`'s doc comment, which carries the same off-node reasoning to
the shipped surface *(verified — read)*. `NUMERIC_CLAIMS.md` §3.13,
rows **NC-108 … NC-110**. Commit **`3502cb7`** *(hash corroborated by
`.git/logs/HEAD`; its contents are not verified)*.

**Revisit if:** a control is quoted whose fixture has not been shown to
carry the effect; a scaling-law control fails and is answered by
widening its band rather than by asking which of the three things is
wrong; or a fourth instance of the pattern in the table above appears,
in which case the observation stops being an observation about three
days and becomes a property of the method.

### DL-026 — ★★ **DL-019's premise EXPIRED, so NC-053 is RE-BASED OFF IT rather than left under it: the raw ICC-absolute comparison stays ungraded PERMANENTLY, because the standard binds READING profiles and not a CMM's computed output.** Filed with the measurement that made the judgement affordable, and with the wording rule that must survive every restatement

**Date:** 2026-08-12 (Pass 4c) · **Judgement by:** `icc-conformance`,
which the Pass 6 + Pass 7 handoff required to *decide rather than defer*
· **Sourcing by:** `icc-spec-librarian`, same day · **Filed by:**
`icc-librarian` · **Relates to** **DL-019** (the entry being re-based),
**DL-021** (a behaviour is a fact about one direction until measured in
the others), **DL-023** (state the disagreement space before the run),
**DL-025** (a control is only as good as its fixture), and
`NUMERIC_CLAIMS.md` **§3.15.6**, **§3.9.5**'s dated correction, and
**NC-053 / NC-054 / NC-119 … NC-128**

#### The decision, in four parts

1. **NC-053 stays REPORTED, NOT GRADED.**
2. **NC-054 stays GRADED at 5×10⁻².**
3. **NC-053 is RE-BASED OFF DL-019.** It is no longer filed under
   *"report-not-grade while the authority does not exist"*.
4. **It is now PERMANENTLY ungraded**, on a different and stronger
   basis — the **A16 / NC-056 pattern**: the standard is silent, two
   named choices exist, the outcome is a **difference**, not an error.

**Parts 1 and 2 look like "no change". Part 3 is the entry.**

#### Why leaving it under DL-019 would have been WRONG even though the verdict is the same

DL-019 is a **holding pattern**. Its condition is *"the mechanism is
identified but its authority does not exist"*, and a holding pattern
asserts something about the world: **that the project is waiting for a
document.**

**It is not waiting any more.** The operator's `ICC.1:2001-04` download
landed, `A4b` resolved, and the corpus was read. **A row still filed
under DL-019 would assert an unmeasured fact about the project's own
state** — and that is precisely the error class this project has paid
for twice: `NEXT_SESSION.md` §3 telling a session to run an experiment
that was already finished (`NUMERIC_CLAIMS.md` §3.14), and the sibling
project's `CLAUDE.md` carrying *"still unverified"* about a question one
of its own corpus files had already answered.

**A verdict that does not change is not the same as a basis that does
not change**, and the basis is the part a future session reads to decide
whether the verdict still holds.

#### What replaced the missing authority — stronger, and permanent

*(Sourced by `icc-spec-librarian`, 2026-08-12.)*

| document | what it actually gates the `wtpt` rule on |
|---|---|
| **`ICC.1:2022` 9.2.36** | **device class — with NO version gate** |
| **`ICC.1:2001-04` A.3.1.1** | **the adaptation condition — not on class at all**: *"If the viewer completely adapts to the white point of the medium (as is often the case with monitors) this tag should be set to Xi, Yi, Zi"* — **monitors are the typical case, not the condition** |

**Therefore lcms2's predicate — `version < 0x4000000 && class ==
'mntr'` — reproduces NO CLAUSE IN EITHER EDITION.** It is not a stricter
reading of a rule; it is a **different rule**, assembled from one half
of each edition. A.3.1.1 is additionally a **`should`**, and
`ICC.1:2001-04` has **no defined verbal-form hierarchy**, so its deontic
weight is **qualified**.

**This is not "the authority is still missing."** It is **the authority
found, read, and observed to be silent on the question** — which is a
finding, and a permanent one. Documents do not become less silent.

#### ★★ Why a graded row is not merely undesirable but UNAVAILABLE

**The conformance clause (`ICC.1:2022` clause 5, `ICC.1:2001-04`
clause 3) binds the ability to READ profiles. A CMM's computed output is
not constrained by either.**

Two consequences, and both are load-bearing:

- **Neither implementation can be graded against the standard here.**
  There is no clause to grade against. A tolerance would be a budget
  with no derivation.
- **Grading iccce against lcms2 would put a budget on a quantity nobody
  controls.** That is exactly the *"widen to ~15 ΔE00 so it passes"*
  alternative **DL-019 already rejected and wrote down** — reached by a
  different route, and it must be rejected again for the same reason:
  15 ΔE00 is a different colour, and such a gate would **silently absorb
  any future arithmetic error in the absolute path.**

#### ★★ The judgement was only DEFENSIBLE because its cost was removed FIRST — record the dependency

**Before this Pass, the ICC-absolute path's ONLY gate was NC-054, a
MODEL.** A model can absorb a genuine arithmetic error **along with the
policy difference it was built to isolate**, and nothing in the suite
could distinguish the two. Declining to grade NC-053 in that world would
have left the absolute arithmetic **unwatched**.

**Pass 4c supplies a raw, unmodelled, GRADED cross-check of that
arithmetic** — **NC-120 at 8.900×10⁻⁵**, below its own media-relative
floor (**NC-122, 1.080×10⁻⁴**) on the same pair, grid and destination
table. **Leaving NC-053 ungraded now costs nothing.**

> **★ The dependency is part of the decision, not background to it. If
> NC-120 is ever removed or invalidated — the lcms2 pin moving would do
> it — THIS JUDGEMENT MUST BE RE-MADE, NOT INHERITED.**

**And the method that made NC-120 possible is the reusable part.**
lcms2's substitution predicate is a **CONJUNCTION**, so choosing a pair
in which **each profile fails a DIFFERENT half** makes the policy
difference **structurally absent** rather than modelled or tolerated:
the source (`fixtures/synthetic/v4-rgb-matrix-trc.icc`) is `'mntr'` but
**v4.4**, failing the version half; the destination
(`USWebCoatedSWOP.icc`) is v2.1 but **`'prtr'`**, failing the class
half.

> **★★ So the item was never blocked on a DOCUMENT. It was blocked on a
> PROFILE PAIR — and the pair was in the committed fixture corpus the
> whole time.** `ROADMAP.md` and `NEXT_SESSION.md` carried it for three
> filings as *"blocked on a document only the operator can fetch"*.
> **The generalisation, and it is the entry's most portable sentence:
> when a comparison is confounded by an implementation's CONDITIONAL
> behaviour, read the CONDITION — if it is a conjunction, the confound
> may be removable by choosing INPUTS rather than by resolving the
> disagreement.**

#### ★★ A WORDING RULE that binds every document this project writes

**Say lcms2 DIVERGES. Never "non-conforming."**

The conformance clause binds **reading**, so *"non-conforming"* is a
verdict **not available** on a CMM's computed output — in either
direction, about either implementation. This mirrors the hedge
`TOLERANCES.md` **§5.2** already carries for **NA-003 / A39b**, and it
is stated here as a rule rather than a footnote because it is now needed
in a second place, which is how a footnote becomes a rule.

#### What this entry does NOT decide

- **It does not decide who is right about `wtpt`.** Nobody is. The
  standard is silent; both readings are named choices; the outcome is a
  **difference**. Rule 7 applies unchanged.
- **★ It does not clear A4c, and A4c does not ride along.** Whether a
  profile's `wtpt` must agree with **its own colorants** is a separate
  ambiguity and remains **SILENT**; the corpus's position is that
  *"disclosure is the one option ICC.1 does not foreclose"*. **The
  system sRGB profile is exactly such a self-inconsistent file** — D65
  `wtpt` beside D50-adapted colorants — and **Pass 4c neither
  adjudicates nor repairs it.** A4c did not clear when A4b cleared and
  it does not clear now.
- **It does not claim iccce's absolute arithmetic is correct.** NC-120
  and NC-122 are **cross-checks**; two implementations can read 6.3.2.2
  the same way and both be wrong. **Pass 4c creates no ground-truth
  row.**
- **It does not generalise past what was measured.** Two profile pairs,
  one destination tag (`B2A1`), one grid (729 points), one machine, one
  pin, two runs. **DL-021 binds.**

#### ★ A citation hazard found while sourcing this, worth carrying past the entry

iccce's docs and code cite the absolute conversion as **"Annex D,
D.6/D.7"**. **Annex D is the INFORMATIVE restatement**; the normative
statement is **`ICC.1:2022` clause 6.3.2.2, Equations (4)–(6)**. Worse,
**the label is NOT edition-stable**: in `ICC.1:2001-04` Annex D the
equations are **(D.1)–(D.6)**, **there is no (D.7)**, and that edition's
**(D.6) is the single `Z` component of the INVERSE**. Since **every
`wtpt` discussion in this project concerns a v2 file**, the ambiguity
was live wherever the bare label appeared. Recommended form:
`ICC.1:2022 6.3.2.2 Eq (4)–(6) [restated verbatim, D.6.1 Eq (D.7)]`.
**A sweep is owed** (`NUMERIC_CLAIMS.md` §7.10), folded into the DL-014
audit.

**Evidence.** `docs/TOLERANCES.md` **§3.4.5** and its `§4` change-table
rows, written by `icc-conformance` *(verified — read)*;
`tools/difftest/src/pass4c.rs`, ten records *(verified — read; the ten
ids appear twice, once emitted and once in the skip-fallback list)*;
`NUMERIC_CLAIMS.md` **§3.15**. **★ There is NO COMMIT**: `pass4c.rs` is
**untracked** *(verified — `git status --short` run; see
`NUMERIC_CLAIMS.md` §2.10 on why a librarian was able to run it at
all)*.

**Revisit if:** the lcms2 pin moves (**NC-120 falls, and with it this
entry's affordability**); an ICC document is published that binds a
CMM's *computed output* rather than its ability to read profiles; a
graded row is ever proposed for NC-053 (it needs a **derivation**, and
this entry says where one would have to come from); or **A4c** acquires
an authority, which would be a different entry and not an extension of
this one.

---

### DL-027 — ★★★ **an implementation's behaviour can be keyed by the DESTINATION PROFILE'S CLASS, not only by the direction and the path: lcms2 has TWO black-point estimators at media-relative, and two header fields decide which one runs.** Filed with the pre-registered prediction that resolved in OPPOSITE directions on the two arms of one experiment

**Date:** 2026-08-12 (the estimator-discrimination filing, the
fourteenth) · **Found by:** `icc-conformance`, by **reimplementing**
`cmsDetectDestinationBlackPoint` from `src/cmssamp.c` at pin `21c582a`
rather than recovering its output · **Filed by:** `icc-librarian` ·
**Generalises** **DL-021** · **Relates to** **DL-012** (a predicted
divergence measured absent), **DL-023** (state what the two sides were
free to disagree about, before the run), **DL-025** (a control is only
as good as its fixture), **DL-028** (its sibling, from the same run), and
`NUMERIC_CLAIMS.md` **§3.17**, **§3.18**, **NC-129 … NC-144**, **NA-009**

#### The mechanism, because the rule is worthless without it

`cmsDetectBlackPoint` **branches before** it reaches the darkest-colorant
code every previous reading of this project had stopped at
(`cmssamp.c` **L370–374**):

```c
// If output profile, discount ink-limiting and that's all
if (Intent == INTENT_RELATIVE_COLORIMETRIC &&
    (cmsGetDeviceClass(hProfile) == cmsSigOutputClass) &&
    (isInkColorspace(cmsGetColorSpace(hProfile))))
    return BlackPointUsingPerceptualBlack(BlackPoint, hProfile);
...
return BlackPointAsDarkerColorant(hProfile, Intent, BlackPoint, dwFlags);
```

| branch | taken when | the chroma |
|---|---|---|
| `BlackPointUsingPerceptualBlack` | output class **and** ink space | **forced to 0** (L174) |
| `BlackPointAsDarkerColorant` | anything else | **retained** |

`cmsDetectDestinationBlackPoint` then returns `Lab.a = InitialLab.a;
Lab.b = InitialLab.b` (L590–591). **The branch IS the returned chroma.**

#### ★★★ The decision

> **A measured implementation behaviour is a fact about the DIRECTION,
> the PATH, *and the CLASS OF PROFILE* it was measured on. Before a
> behaviour is written down as a rule, read the CONDITION that selects
> it — and if the condition names header fields, the rule needs a second
> arm chosen to fail that condition.**

**DL-021 said direction and path because that is what one day's evidence
supported.** This entry adds the third axis **because the same shape of
error recurred in the same oracle, one axis over**: iccce had a rule
about lcms2's chroma handling that was **true of every profile within
reach of this machine and false of a whole class of profiles nobody
here owns.**

#### ★★★ The evidence, and why one arm would have been worse than none

`USWebCoatedSWOP.icc` is `prtr` + **CMYK** — an ink space, output class,
**first branch**. The synthetic `v4-rgb-mab-chromatic-black.icc` is
`prtr` + **RGB** — not an ink space, **second branch**.

| | `swop` | `synthetic` |
|---|---|---|
| divergence | **8,166 8×10⁻² ΔE76 — 100 % `L*`** | **5,000 000 ΔE76 — 100 % chroma, `ΔL*` exactly 0** |
| the corpus's pre-registered **mechanism** claim | **FALSIFIED** | **CONFIRMED** |
| its **shape** claim | **FALSIFIED** | **CONFIRMED** |

> **★★★ A session that ran only one arm would have filed a confident
> wrong headline EITHER WAY** — *"lcms2 keeps its black point's chroma
> and iccce neutralises it"* or *"the chroma prediction was imaginary"*
> — **and both sentences would have been supported by a clean, tight,
> honestly bounded measurement.**

#### ★★ What it cost the record, stated because the cost is the argument

Pass 5b had already discriminated the estimators **on the `swop` arm
alone** and filed the result. Pass 5c **withdrew** its mechanism verdict,
**superseded** its headline number (98,3 % of it was the apparatus —
**DL-028**), and **settled** the one verdict Pass 5b had declined to
call. **Three graded rows were INVERTED, and none was deleted**
(`NUMERIC_CLAIMS.md` §3.17 carries them with their old verdicts visible)
— the project's convention, fifth instance after DL-011 → DL-012,
NA-006's *"tetrahedral"*, A4b's expiry, and the corpus's M2 correction.

**★ The row that made the withdrawal findable was the one whose job was
to doubt its own apparatus.** Pass 5b's error bar came back at **0,948**
against a limit of 1,0, was reported as **marginal rather than green**,
and said in terms that *which conclusions survive is decided row by
row*. **They were, and the one that did not survive is exactly the one
the bar was too weak to support.**

#### What this entry does NOT decide

- **It does not say lcms2 is wrong.** Two branches are a *design*, not a
  defect, and rule 7's *"disagreement is a finding"* cuts both ways.
- **★ It does not say iccce is right.** The `swop` divergence is
  **entirely** the ISO/CD 18619 **4.2.5.4 short-circuit's return value**
  — iccce returns `outRamp[first]`, lcms2 returns `InitialLab` — and
  **which one the standard specifies is an open question dispatched to
  `icc-spec-librarian`.** **If ISO names lcms2's, iccce is WRONG, not
  divergent**, and the engineer changes the code.
- **It does not extend past media-relative.** At perceptual and
  saturation on a v4 profile **both implementations return the fixed A41
  constant without reading the profile**, so **no fixture can
  discriminate them there** — the instrument three documents asked for
  **cannot exist**, and asking again would be asking for a null by
  construction.
- **It does not license reading two arms as "enough".** Two arms
  falsified a one-arm rule. **A third class of destination — gray, or a
  v2 `mAB ` RGB output profile — is untested**, and this entry is
  exactly the kind of claim it warns about if it is read as complete.

**Revisit if:** the lcms2 pin moves (**the branch predicate is source,
and source moves**); a destination class outside {ink+output,
RGB-output} is measured; or ISO/CD 18619's short-circuit question is
answered — **which changes the attribution of the `swop` number without
touching this entry's mechanism.**

---

### DL-028 — ★★ **a residual that is LARGE UNDER EVERY HYPOTHESIS is an apparatus fault, not a finding — so an experiment that can only be checked against one candidate cannot be checked at all.** Filed with the unit error it caught, which had been in three Passes' worth of oracle output

**Date:** 2026-08-12 · **Caught by:** `icc-conformance`'s two-candidate
validation arm, on the **first** run of the synthetic arm · **Filed by:**
`icc-librarian` · **Relates to** **DL-018** (a gate needs a prediction
pin), **DL-025** (a control is only as good as its fixture), **DL-027**
(its sibling), and `NUMERIC_CLAIMS.md` **§3.18.4**, **NC-140**,
**NC-141**

#### The incident

The synthetic arm's first run reported a device residual of
**9,98×10⁻²**, where the truth is **8,9×10⁻⁶** — four orders out. **The
finding it would have produced is a real sentence somebody would have
believed:** *"the reimplementation does not reproduce lcms2 on this
fixture."*

**The cause was not colour at all.** `transicc` prints **ink** spaces as
percentages (`0..100`) and **RGB and gray as `0..255`**. Every oracle
output in **Passes 5, 5b and 5c** had been divided by **100**, because
**until this section the only destination in reach was CMYK** — so the
bug was correct code for every profile the project had ever measured,
and became wrong the first time a fixture was RGB.

#### ★★ The decision

> **When a comparison's residual is large under EVERY candidate
> explanation, suspect the apparatus before the subject. Therefore:
> carry a SECOND, INDEPENDENT candidate wherever the point of the
> experiment is to discriminate — and grade the RATIO, not the
> magnitude.**

**The ratio is what made it visible.** `NC-140` divides the residual
under the lcms2 hypothesis by the residual under the ISO hypothesis. Its
purpose is to ask *"can this experiment tell the two apart at all?"* —
and a unit error inflates **both** numerators identically, so the
**absolute** numbers looked merely disappointing while the **ratio**
said the experiment had stopped discriminating.

#### ★ Why "read the code again" would not have found it

This is the third instance in this project of the same shape, and the
list is worth keeping in one place:

| | what looked fine | what caught it |
|---|---|---|
| **DL-016** | a self-consistency round trip that **passed** | exact values at the sample points |
| **DL-025** | **1,1×10⁻¹⁵** — a magnificent number | a control asked what it would return if the effect were absent |
| **DL-028** | a plausible *"they disagree"* headline | a **second candidate**, and a ratio between them |

**In every one, re-reading the code was available and would not have
worked**, because the code did exactly what it said. **The instrument
was the only thing that could fail in a way a human notices.**

#### ★ The corollary, from the same run, and it is the sharper half

Pass 5b bounded its recovered black point with an error bar of
**0,813 7** against an effect of **0,858 17** and called the section
*marginal*. Pass 5c then showed the recovery **was** the effect:
**98,3 %** of the published number was the apparatus.

> **★★ When an error bar is the same ORDER as the effect, the honest
> reading is not "the result is marginal". It is "the apparatus may be
> measuring ITSELF."**

**A margin of 5 % on an error-bar row is not a pass with a small
margin — it is a row saying, correctly, that the experiment is not yet
built.**

#### What this entry does NOT say

- **Not that Pass 5b should have been skipped.** *"A recovery that is
  honestly bounded and then replaced is how the bound gets to be
  checked; a recovery quoted as a measurement is how it does not."*
- **Not that every comparison needs two candidates.** It binds
  **discrimination** experiments — those whose question is *which of two
  explanations is right* — not agreement checks with one arm and a
  tolerance.
- **Not that the unit bug is fully cleared.** It was **corrected where
  Pass 5c reads `transicc`**; whether every other `transicc` call site in
  `tools/difftest` scales correctly is **not something this filing
  measured**, and every destination in those Passes being CMYK is a
  reason it *probably* never mattered — **which is a hypothesis, not a
  check.**

**Revisit if:** a harness site is found dividing an RGB or gray
`transicc` output by 100; or a discrimination row is proposed with a
single candidate, at which point this entry is the objection.

---

### DL-029 — ★ **the API sealing split, stated as a rule rather than taken case by case: SEAL what decodes OUR file format; PUBLISH what implements SOMEONE ELSE'S published specification.** Filed with the four pre-publication soundness defects, one of which was rule 1 wearing a public field

**Date:** 2026-08-12 · **Done by:** `icc-engineer` (commits **`fc4727b`**
*"Pre-publication audit: four API soundness defects fixed, metadata,
CI"* and **`95c04c1`** *"api: seal the byte readers, keep the ISO surface
public — a stated split"*) · **Filed by:** `icc-librarian` **from the
tree, not from the dispatch** · **Relates to** **DL-009** (crates.io
intent), **DL-024** (publication authorises nothing else), and
`NUMERIC_CLAIMS.md` **§2.11**

#### The rule

> **A published crate's surface is a promise. Seal the parts whose only
> caller could be ourselves — the byte-level readers that decode ICC's
> layout — and keep public the parts that are a faithful implementation
> of a document a caller may legitimately want to drive: ISO/CD 18619's
> black-point procedure, the colour metrics, the models.**

**The distinction is stated at the site, in `bpc.rs`** *(verified —
read)*: the ISO vertex-set/darkest-vertex/neutralise functions are
*"PUBLIC DELIBERATELY, unlike the byte-level readers in
`iccce-profile::num` which were sealed to `pub(crate)` in the same
pre-publication pass … those decode **our** file format and a consumer
has no business calling them, whereas this and its three siblings are a
faithful implementation of a **published algorithm** … The clause
citations in these doc comments are the point of exposing them."*

#### ★ The four soundness defects — three carry an audit annotation, and the fourth is this entry's own subject

**Stated as this librarian's reconstruction from the tree, not as the
engineer's enumeration** *(all four verified — read; the dispatch says
"four" and does not list them)*:

1. **★ `MatrixTrc::matrix` was `pub`, beside a CACHED `matrix_inv`.** A
   consumer assigning to the public field would leave `pcs_to_device`
   using the **stale inverse** — **silently wrong colour with no
   signal**. Now private with read-only `matrix()` / `matrix_inverse()`.
   **This is CLAUDE.md rule 1 as an API shape**: *"making the pair
   unmutatable together is cheaper than documenting the trap."*
2. **`Clut`'s `grid`/`samples` were public**, while `Clut::new` is the
   only place the invariants (every entry ≥ 2;
   `samples.len() == Π grid × outputs`) are checked and `Clut::eval`
   indexes on their truth. Now private with accessors.
3. **`ChannelMismatch { expected: 3, actual: 3 }` was being returned as
   a stand-in for unrelated failures** — *"a public error saying '3
   channels expected, 3 given' is misinformation on the API surface."*
   Now `ChainError::EvaluationFailed { stage }`.
4. **`iccce-profile::num`'s big-endian readers and fixed-point types
   sealed to `pub(crate)`** — the half of the split this entry names.

#### ★★ Why an API entry belongs in a COLOUR project's decision log

Because three of the four are the **same failure mode this project
exists to catch, relocated**: a caller does something reasonable, the
library keeps working, and **the colour is wrong with no signal**. A
stale inverse produces a plausible image. A `Clut` with a mutated grid
produces a plausible image. An error that names the wrong cause sends a
maintainer to the wrong file. **None of them crashes.**

#### What this entry does NOT decide

- **It does not authorise a publish.** **DL-009** is intent, **DL-024**
  is a git push, and **neither is a crates.io release.** Name
  availability is **still unchecked by anyone**, and
  `THIRD_PARTY_LICENSES.md` via `cargo-about` is **still owed**.
- **It does not claim the surface is now stable.** No version policy has
  been stated; sealing is not the same as committing to what remains.
- **It does not claim the audit was exhaustive.** Four defects were
  found and fixed; **nothing swept the whole public surface**, and this
  filing did not either.

**Revisit if:** a crates.io publish is authorised (the split becomes a
compatibility promise); a consumer needs a sealed reader, which is
evidence the split was drawn in the wrong place; or a fifth defect of
the same shape is found, which would argue for a systematic sweep rather
than another entry.

---

### DL-030 — ★★★ **the open question of DL-027's family is ANSWERED and it went against us: iccce was NON-CONFORMANT at ISO/CD 18619 4.2.5.4 and lcms2 conformed.** Filed as a decision because rule 7 was written to be capable of this outcome and had never yet produced one

**Date:** 2026-08-12 · **Done by:** `icc-engineer` (commit **`fd34a44`**
*"bpc: iccce was WRONG at 4.2.5.4 — lcms2 conformed, we did not"*),
sourced by `icc-spec-librarian` · **Filed by:** `icc-librarian` **from
`crates/iccce-cmm/src/bpc.rs` read at the tip, not from the dispatch** ·
**Answers** the question `NUMERIC_CLAIMS.md` §3.18.6 dispatched and
§7.11 carried as newly-owed item 1 · **Relates to** **DL-027** (the two
estimators), **DL-020** (refuse rather than guess), and rule 7

#### The decision

> **When the specification settles a disagreement against us, the code
> changes and the record says which of the two implementations was
> wrong, by name.** ISO/CD 18619 **4.2.5.4**'s final paragraph reads, in
> the corpus's verbatim transcription: *"If the mid range is straight
> (as determined above) then the DestinationBlackPoint **shall be the
> same as InitialLab**."* **lcms2 (`cmssamp.c` L536) does that. iccce
> returned `outRamp[first]`. iccce was wrong.**

**What made it wrong rather than divergent** *(verified — `bpc.rs`
lines 174–199 and 266–273 read)*: `outRamp[first]` occurs in the whole
of clause 4.2.5 only as `MinL` — a **threshold** and a `yRamp` anchor —
and in 4.2.5.3's validity test. **It is not a candidate for the black
point in any branch.** There was no reading of the clause under which
the shipped behaviour was permitted; this is not a silence iccce filled
differently, which is what DL-015 and NA-006 are.

#### ★★ Why this belongs in the decision log and not merely in the ledger

**Because it is the first time rule 7 has run in this direction, and a
rule that has only ever exonerated the project has not been tested.**
The rule reads *"disagreement with lcms2 is a finding, not a failure"*
and the danger in it was always the second clause being heard as *"so
iccce is right"*. `NUMERIC_CLAIMS.md` §3.18.6 anticipated exactly this
and pre-committed in writing — *"if ISO names lcms2's, iccce is WRONG —
not divergent — and the engineer changes the code … rule 7 is not a
licence to assume iccce is right"* — **before the answer existed**.
That pre-commitment is what made the outcome cheap to accept.

#### The cost, and the one number that must travel with it

**0,0817 ΔE76 on `USWebCoatedSWOP`** — which is **100 % of the two
implementations' measured disagreement on that arm**
(`NUMERIC_CLAIMS.md` **NC-142**, `8,166 8×10⁻² ΔE76`, `ΔL* 0,081 67`,
chroma exactly 0). ★ **The defect was measured before it was found.**
Pass 5c measured the divergence, could not attribute it, and named the
one line it had to be; the sourcing then said which side of that line
was wrong. **A measurement whose attribution is left open is what makes
a later answer actionable** — had the divergence been filed as *"lcms2
departs from the standard"*, the correct answer would have had nothing
to attach to.

#### A corollary the same reading produces, which is NOT a bug fix

The function's **return type widened from `L*` to a full `Lab`**,
because 4.2.5.2.1 zeroes chroma **only for CMYK**, so on a Gray or RGB
LUT destination ISO itself yields a **chromatic** `DestinationBlackPoint`
and the short-circuit is **the only branch that can return one**.
Neutralising it at the return would have been a second, quieter
departure. **Cost today: zero** — 4.2.6 ignores `a`/`b` downstream —
**and the correctness is not zero**, which is the distinction rule 4
exists to keep visible.

#### What this entry does NOT decide

- **It does not make lcms2 an authority.** lcms2 conformed *here*, at
  one clause, at one pin. **DL-027** stands: lcms2 has two estimators
  and a branch §3.5.7's table did not trace.
- **It does not retire the ISO/CD tier caveat.** 18619 is a **committee
  draft** in this project's corpus, and every consequence drawn from it
  inherits that.
- **It does not claim the corpus caught this.** ★ It did not: the corpus
  had **not transcribed 4.2.5.4 verbatim**, so nothing in it said
  `outRamp[first]` was wrong. **The defect shipped, was measured, and
  was then caught from outside** — a corpus gap and an implementation
  bug with the same root.

**Revisit if:** a published edition of ISO 18619 changes 4.2.5.4's
final paragraph; or any further clause of 4.2.5 is transcribed and
disagrees with what `bpc.rs` now does.

---

### DL-031 — ★★ **an unlabelled test count is not a claim, because the APPARATUS is half the number.** Filed with the day's live instance: three green results on one tree read 129, 36 and 142, and two of them were briefly compared

**Date:** 2026-08-12 · **Occasioned by:** `icc-engineer`, who ran
`cargo test --workspace` at the tip, got **129**, and momentarily read
it as a regression against a *"suite green at 142"* carried in commit
**`d5efd96`**'s message · **Filed by:** `icc-librarian` · **Relates to**
`NUMERIC_CLAIMS.md` **§1** (evidence classes) and **§1.2** (a count of
tests declared is not a count of coverage)

#### The rule

> **Every test count is written with the command that produced it, or
> it is not written.** *"The suite is green at N"* is not a fact about
> this project; it is a fact about **one invocation of one runner over
> one member set**, and this repository has **three** runners whose
> member sets do not overlap.

The three, as of commit `2a2d616` *(reported by `icc-engineer`, who ran
them; see the ledger for what corroborates each)*:

| Apparatus | Command | Result |
|---|---|---|
| **The workspace unit suite** | `cargo test --workspace` at the repository root | **129 passed, 0 failed**, exit 0 |
| **The harness's own unit suite** | `cargo test` in `tools/difftest` — **deliberately not a workspace member** (DL-001, DL-017), so `--workspace` cannot see it | **36 passed**, exit 0 |
| **The differential conformance runner** | `cargo run --release` in `tools/difftest` — the oracle, which drives lcms2 | **pass=142 fail=0 skip=3 error=0** |

**142 was never a `cargo test` count.** It is a count of *conformance
records*, and the number it is comparable to is its own previous run
(`pass=140 fail=2`), not to 129 and not to 36.

#### ★ Why this is a decision and not a note

**Because the failure mode is silent and arrives late.** A bare number
in a commit message, a README or a filing survives the session that
produced it; the reader who next meets it has no way to know which
runner it came from, and **the two most natural things to do with two
numbers are to compare them and to infer a trend.** That is precisely
what happened here — by the engineer, on the same day, on numbers he had
produced himself. **If it can be misread by its own author within
hours, it will be misread by anyone else.**

The general form, which is the part worth carrying beyond this project:
**a count is a measurement, and a measurement without its apparatus is
not weak evidence — it is uninterpretable.** This is the same rule as
`NUMERIC_CLAIMS.md` §1's `machine-timing` class (a throughput figure is
a fact about one machine) and the same rule as §3.19's *"a maximum over
one population is not a maximum over another"*, applied to integers
instead of to reals.

#### What follows, concretely

1. **The ledger carries all three numbers with their commands** —
   `NUMERIC_CLAIMS.md` §3.22, `NC-158` … `NC-160`, in a new evidence
   class that exists to keep them from being quoted beside a ΔE.
2. **No document may write "the suite is green" without the command.**
   Where an older document does, it is corrected by a dated addition
   rather than by rewriting, so the ambiguity is visible.
3. **A count of tests is still not a count of coverage** (§1.2). This
   entry makes counts *interpretable*; it does not make them *strong*.

**Revisit if:** a fourth runner appears (a doc-test target with
contents, a `gen-profiles` suite, or CI reporting its own aggregate),
which would make a bare number ambiguous in a new way and is the
condition under which the table above must grow.

---

### DL-032 — ★★ **a warning that is EXPECTED is documented at the site with the consequence of "fixing" it, because the alternative is that some future agent silences it correctly and ships a defect.** Filed with the near-miss it prevented, on the same day

**Date:** 2026-08-12 · **Occasioned by:** `icc-engineer`, who was about
to delete `license-file` from three crate manifests to silence a
`cargo publish` warning and stopped on reading the comment above it ·
**Filed by:** `icc-librarian` **from the manifests, read at the tip** ·
**Relates to** **DL-009** (crates.io intent), **DL-024** (a push
authorises nothing further), and the global documentation-first
directive

#### The decision

> **Where a build emits a warning the project intends to keep, the
> manifest or source carries a comment saying (a) that the warning is
> expected, (b) what the obvious fix would silently break, and (c) the
> command that verifies the property being protected. A warning with no
> such comment is an invitation.**

**The live text, verbatim from `crates/iccce-color/Cargo.toml`**
*(verified — read; `iccce-profile`, `iccce-cmm`, `iccce-cli` and
`iccce-measure` carry a pointer to it rather than a copy)*:

> `★ EXPECTED WARNING — do not "fix" it by deleting `license-file`.`
> `cargo publish` prints *"only one of `license` or `license-file` is
> necessary"* because both are set. Both are set on purpose: `license =
> "MIT"` (inherited from the workspace) is the SPDX expression crates.io
> indexes and displays, and `license-file` is what actually puts the
> licence **TEXT** in the tarball. Removing `license-file` silences the
> warning and **silently stops shipping the notice** — verify with
> `cargo package --list -p iccce-color`, which must show `LICENSE`.

#### ★★ What the near-miss actually was, stated precisely

**Not a licensing opinion — a shipped artefact.** `license = "MIT"` is
metadata; **MIT itself requires the notice text to be included in
copies.** With `license-file` deleted, `cargo publish` is quiet, the
crates.io page still says *MIT*, the build still works, and **the
`.crate` tarball contains no licence text at all.** Nothing downstream
fails. The defect is invisible at every point where anyone would look
for it, and it is only detectable by listing the tarball's contents —
which is why the comment names that exact command.

**This is CLAUDE.md rule 1 in a non-colour register.** A wrong colour
looks exactly like a right one; **a tarball missing its notice looks
exactly like one that has it.** The project's whole discipline is built
for the class of defect that produces no signal, and this is one.

#### Why the entry, when the comment already exists

Because **the comment worked once and comments are load-bearing only
while they are read.** The event worth recording is not that a warning
exists; it is that *documentation-first prevented an engineer from
making a correct-looking change*, and that is the concrete return on a
directive whose cost is otherwise all up-front. **It is also the
counter-example to the reasonable-sounding instinct that a clean build
is an end in itself:** here the clean build **is** the defect.

#### What this entry does NOT decide

- **It does not authorise a publish.** DL-009 is intent; DL-024 is a git
  push; **neither is a crates.io release**, name availability is still
  unchecked and `THIRD_PARTY_LICENSES.md` is still owed.
- **It does not claim the tarball was verified today.** The `cargo
  package --list` verification is dated **2026-08-12** *in the manifest
  comment itself* and is `icc-engineer`'s; **no agent re-ran it at this
  filing** and this librarian has no shell.
- **It does not generalise to suppressing warnings.** The rule is to
  **document** an expected warning, not to add `#[allow]` or
  `--quiet`. A silenced warning teaches nobody anything.

**Revisit if:** cargo stops warning when both keys are set (the comment
becomes stale and should say so rather than be deleted); or a second
expected-warning site appears, at which point the pattern is worth a
short list somewhere rather than one comment per site.

---

### DL-033 — ★★★ **AGREEMENT WITH THE ORACLE WAS THE SYMPTOM OF OUR DEFECT.** A cross-check's power is bounded by the distance between the two CANDIDATE answers, not by the tightness of the residual it reports — so being wrong in the right way is invisible to it

**Date:** 2026-08-12 · **Occasioned by:** the re-measurement of the
`swop` black-point divergence on the code corrected at `fd34a44`, which
**grew 58,8× instead of collapsing** · **Measured by:**
`icc-conformance` · **Filed by:** `icc-librarian` from
`tools/difftest/src/pass5c.rs`, read at the tip `2881e19` *(verified —
the record's own `source` string carries every figure below)* ·
**Relates to** **DL-030** (the defect itself), **DL-028** (a residual
large under every hypothesis), **DL-023** (pre-registration), and
**NC-165 … NC-167**, **NC-174**, **NC-175**

#### The decision

> **A cross-check whose two candidate answers are close together cannot
> detect an error that moves the result between them. Before a
> cross-check is offered as evidence that a value is right, state the
> SEPARATION OF THE CANDIDATES it was capable of resolving — and where
> the defect's own magnitude exceeds the residual the cross-check
> reports, say so in the same sentence. A small residual is not evidence
> of correctness; it is evidence of proximity, and proximity has more
> than one cause.**

#### The arithmetic, which is the whole entry

`bpc.rs`'s straightness short-circuit returned `outRamp[first]`. Clause
4.2.5.4 of ISO/CD 18619 requires `InitialLab`. On
`USWebCoatedSWOP.icc`:

| Quantity | Value |
|---|---|
| the **non-conformant** return, `outRamp[first] = MinL` | `L* 16,489 806` |
| the **conformant** return, `InitialLab` | `L* 11,772 365` |
| **the defect's own magnitude** | **`4,717 441 L*`** |
| lcms2's answer (reimplemented from `cmssamp.c` at pin `21c582a`) | `L* 16,571 474` |
| the divergence the cross-check reported **before** the fix | **`8,166 8×10⁻² ΔE76`** |
| the divergence the cross-check reports **after** the fix | **`4,799 109 ΔE76`** |

★ **The defect's magnitude was 57,8× the divergence it was blamed for.**
And the reason is exact rather than coincidental: `MinL` **is the same
number in both documents** — `MinL(lcms2) = MinL(ISO) = 16,489 806` —
so a non-conformant implementation that returned `MinL` was returning a
quantity the oracle also computes, and landed `0,0817 L*` from the
oracle's answer **for a reason that had nothing to do with being
right**.

#### ★★ Why this is not the same entry as DL-028

**DL-028** says a residual that is *large under every hypothesis* is an
apparatus fault. **This is its mirror and it is the more dangerous
half**: a residual that is *small* is read as success by default, and
nothing in the number asks to be explained. DL-028's failure announces
itself; **this one is silent, and silence is the failure mode CLAUDE.md
rule 1 exists for.** A wrong colour looks exactly like a right one —
and here **a wrong colour agreed with the oracle to 0,08 ΔE76.**

#### ★★★ What this does NOT license, and the trap is right here

It does **not** license reading the *new*, larger figure as evidence
that the corrected code is worse, or the old, smaller one as evidence
that the defective code was better. **Neither number grades
correctness at all.** The clause graded correctness (**DL-030**); the
cross-check only ever measured distance to another implementation.
★ **The right reading is that the cross-check was nearly blind here and
is now merely uninformative here** — both implementations return what
their own document calls `InitialLab`, and the residual is two
documents meaning different things by one name (ISO 4.2.2.2: the
darkest **device vertex**, neutralised; lcms2's `cmsDetectBlackPoint`:
the **perceptual black round trip** with chroma zeroed). **There is no
ground truth in this comparison** — no published black point exists for
`USWebCoatedSWOP.icc`, and 18619 is a committee draft.

#### What follows operationally

1. **Every cross-check row in `NUMERIC_CLAIMS.md` that grades a value
   against lcms2 owes a candidate-separation statement** wherever the
   two implementations' *methods* could produce answers close together
   for unrelated reasons. This is owed and is **not** discharged by this
   entry.
2. **A pre-registered prediction of a collapse is a prediction, and gets
   measured.** §3.24 pre-committed that NC-142's figure *"should now be
   expected to COLLAPSE"* and explicitly **refused to assert it**
   (`NUMERIC_CLAIMS.md` §7.12 newly-owed 1). ★ **That refusal is the
   only reason this entry is an observation rather than a retraction.**

**Revisit if:** a published black-point value for any real profile
becomes available (the comparison acquires a ground-truth arm and this
entry's "no ground truth" clause narrows); or a cross-check is built
whose candidate separation is stated in advance, at which point this
rule has a worked positive example instead of only its founding
negative one.

> **★★ Dated note, 2026-08-12 (third filing of the session, tip
> `e26d9ba`) — the "Revisit if" clause above has FIRED, and operational
> item 1 is CLOSED FOR PASS 5c AND OPEN EVERYWHERE ELSE. Nothing above
> is edited.**
>
> `tools/difftest` now emits a candidate separation on every record,
> with three stated states and an automatic verdict *(verified —
> `lib.rs` read at the tip)*. **The worked positive example this entry
> asked for exists and is filed as `DL-037`**, which also records the
> two design guards that make the mechanism honest rather than merely
> present.
>
> ★★★ **The closure is PARTIAL and the fraction is part of it:
> `16 of 145` rows state a separation, and all 16 are Pass 5c's.** No
> Pass 0 – Pass 5b, Pass 6 or Pass 7 row states one — **129 rows print
> `UNSTATED`**, which is the mechanism working, not the mechanism
> finished. **Item 1 remains owed for every Pass but 5c**
> (`NUMERIC_CLAIMS.md` §7.14 newly-owed 1).
>
> ★★ **And the first thing the mechanism found was about this entry's
> own founding measurement**: the row carrying the whole 4.2.5.4
> finding is **`UNGRADED`** — tolerance `inf` — so **it could never
> have failed, whatever the candidates did.** Its separation is
> **`4,717 441`**, the defect magnitude in the table above, to six
> figures. **This entry said the cross-check was "nearly blind, and is
> now merely uninformative"; the emitted field says something stricter
> — on that row it was never a check at all.** `NUMERIC_CLAIMS.md`
> **§3.28**, **NC-176**.

---

### DL-034 — ★★ **a claim-bearing number the harness can COMPUTE is FORMATTED AT RUN TIME, never typed into prose beside the code that computes it.** A stale comment misleads a reader; a stale string inside an emitted conformance record misleads the EVIDENCE

**Date:** 2026-08-12 · **Adopted from:** `icc-conformance`, which found
the instances · **Filed by:** `icc-librarian` from
`tools/difftest/src/pass6.rs`, read at the tip *(verified — the
corrective comment at L1055–1072 and the `format!` at L1084–1099)* ·
**Relates to** **DL-031** (a count without its apparatus), **DL-018**
(a gate that rewards deletion), and `NUMERIC_CLAIMS.md` §0

#### The decision

> **If the harness already holds a value in a variable, the emitted
> record interpolates that variable. Writing the value out as a literal
> in the adjacent prose creates a second, unsynchronised copy of a fact
> the program already knows — and the copy lives inside the artefact
> whose entire purpose is to be durable evidence.**

#### The instance that named the rule

A Pass 6 record's `source` string read *"**17** is the shipped
default"*. It was true when written. Commit `189e732` moved
`compiled::recommended_grid_points(4)` to **33**, and the string went on
being emitted into **every record of every subsequent run** — so the
apparatus spent a day **asserting a false fact about the shipped product
inside its own evidence**. The string is now
`"{DEFAULT_GRID} is the shipped default"`, interpolated from the same
constant the code grades against *(verified — read)*.

#### ★★ The second instance, which is the sharper one

The same string also read *"the only grid `COMPILED_DE` is **DERIVED**
for"*. That was **wrong on the day it was written** — not stale, wrong.
`COMPILED_DE`'s derivation population is **Pass 4's 341-point CMYK
comparison of the REFERENCE path against lcms2 2.19.1**, and
`tools/difftest/src/pass4.rs` **never constructs a `CompiledTransform`
at all** *(verified — read; the file builds a 341-point CMYK `grid()`
and compares reference-path output against the oracle)*. There is no
compiled grid anywhere in that number to be stale. What is
grid-dependent is the bound's **applicability**, because the graded
quantity is `O(h^1,32)` in the grid spacing (**DL-025**).

★ **A stale number and a conflated concept were shipping in one
sentence, and only the stale one was visible.** Re-formatting the
string forced the sentence to be re-read, which is what found the other.

#### What this rule cannot do

- **It cannot help a number the harness does not compute.** A figure
  quoted from another document, another session or a commit message is
  outside its reach entirely; **DL-031** covers that case and this one
  does not.
- **It does not make the record self-explaining.** `DEFAULT_GRID`
  interpolated correctly still says nothing about *which runner* emitted
  it — the census problem is orthogonal and unsolved by this.
- **It is not a licence to delete prose.** The remedy is to interpolate
  the *number*, keeping the reasoning around it; a record with no source
  string is worse than one with a stale figure, because the stale figure
  at least gets audited.

**Revisit if:** a claim-bearing figure is found in an emitted record
that the harness genuinely cannot compute at run time (the rule needs an
explicit exception clause, stating how such a figure is dated instead).

> **★★ Dated note, 2026-08-12 (third filing of the session, tip
> `e26d9ba`) — a FOURTH instance, and it is the first found by an
> APPARATUS rather than by a person.** Nothing above is edited.
>
> `SHIPPED_MATCHES_LIBRARY`'s justification asserted the two candidate
> blacks were **`2,46×10⁻³` apart, "three orders above the bound"**.
> Computed by the new separation field on its first run:
> **`9,574 451×10⁻³`** — **four** orders. `2,46×10⁻³` was the
> **pre-`fd34a44`** figure *(verified — `pass5c.rs` L1409–L1431 read at
> the tip)*.
>
> ★★★ **The argument was never harmed, only the number — and the stale
> figure UNDERSTATED the claim by a factor of four.** That is why this
> kind is the hardest to find: **a literal inside a justification is
> load-bearing for the conclusion's direction, which stayed right, and
> wrong only in a magnitude nothing re-reads.** A stale number that
> weakens your own argument raises no suspicion at all.
>
> **A companion fix of a different kind, worth distinguishing:**
> `NEUTRAL_EXACT`'s `0,834` / `5,0` were replaced by the emitted field
> although **both were still true**. They are properties of *which
> fixture is loaded*, so **a third arm would have falsified the sentence
> with nobody touching a line of it** *(verified — `pass5c.rs`
> L1286–L1291)*. ★ **One literal was stale; the other was only
> accidentally true. This rule covers both, and the second case is the
> one the rule's original wording does not obviously reach** — it is
> not that the harness *can* compute the number, it is that the number
> is a property of the run rather than of the claim.
>
> **Count, with provenance:** `icc-conformance` calls this the fourth
> instance; `lib.rs` L1697–L1703 records *"three claim-bearing literals
> in this crate went false inside a day"* *(verified)*, and **this entry
> names two of them.** The fourth is named here; **the enumeration of
> the first three is not re-derived** — a count is not an inventory.

---

### DL-035 — ★★ **an improvement whose cause is the DENOMINATOR or the RIVAL is not an improvement.** A graded ratio is read with its numerator and its denominator separately, or its direction of travel is unreadable

**Date:** 2026-08-12 · **Occasioned by:** two conformance rows that went
green-er on the corrected 4.2.5.4 code for reasons opposite to what
their numbers imply · **Measured by:** `icc-conformance` · **Filed by:**
`icc-librarian` *(the two rows' figures are **CARRIED** — this librarian
has no shell and did not re-derive them; see `NUMERIC_CLAIMS.md`
§3.26)* · **Relates to** **DL-018**, **DL-028**, **DL-031**

#### The decision

> **Where a graded quantity is a ratio, the ledger records the movement
> of the numerator and of the denominator as separate facts, and a
> narrative sentence about "improvement" is only permitted once both are
> stated. A ratio that improves because its denominator grew, or
> because a rival candidate got worse, has recorded no improvement in
> the thing anyone cares about.**

#### The two rows that named it

| Row | Before | After | ★ What actually moved |
|---|---|---|---|
| **T1** — apparatus error bar against the effect | `3,043×10⁻¹` | `5,179×10⁻³` | **The error bar did not change.** The *effect* grew **59×**. The apparatus is exactly as good as it was |
| **T4** — the reimplementation against the rival candidate | `1,715×10⁻¹` | `4,258×10⁻²` | **The numerator did not change.** The **rival got 4,03× worse** |

★ **A reader seeing only the two numbers concludes the opposite of what
happened**: that the apparatus got 59× sharper and the reimplementation
4× better. Neither is true, and both rows are *correctly computed*.
**The defect is not in the arithmetic; it is in the ratio being
publishable on its own.**

#### Why this is filed as a decision and not a note

Because **both rows are green, and green is not audited.** DL-018
established that a gate can be made greener by deleting the requirement
it protects; this is the same structure one level up — **a ratio can be
made greener by an event in its denominator that nobody chose and
nobody would defend if it were stated plainly.** The remedy in both
cases is identical and cheap: **pin the parts, not only the quotient.**

#### What this does NOT say

- **It does not say the two rows are wrong or should be ungraded.**
  Both are the right quantity to gate on; T1's ratio is exactly the
  DL-028 guard that an error bar must be smaller than its effect.
- **It does not say a ratio must never be quoted.** It says a quoted
  ratio must arrive with its two components, in the same table.
- **It does not extend to ratios whose denominator is a constant** —
  a tolerance with no free parameter in it (`COMPILED_DE`) has nothing
  to hide behind.

**Revisit if:** a third instance appears whose components move in the
same direction, which would show the pattern is not confined to
compensating movements and may need a stronger form (e.g. recording the
components as their own ledger rows rather than as a note beside the
ratio).

---

### DL-036 — ★★ **a real vendor profile stays in the fixture set beside the authored ones, because on the measurement that mattered the AUTHORED fixture had no power at all.** The converse of DL-020, learned from the same clause

**Date:** 2026-08-12 · **Occasioned by:** the black-point re-measure, in
which the `synthetic` arm's figure **did not move by one part in 10⁶**
while the `swop` arm's grew 58,8× · **Filed by:** `icc-librarian`
*(figures **CARRIED** from `icc-conformance`; the mechanism is
**verified** from `tools/difftest/src/pass5c.rs` and `bpc.rs`, read at
the tip)* · **Relates to** **DL-020** (a refusal is discharged by an
independently authored fixture), **DL-027** (behaviour keyed by profile
class), **GP-001**

#### The decision

> **The fixture corpus keeps at least one real, third-party-authored
> profile alongside every synthetic one, and a measurement taken only on
> authored fixtures states that it was. An authored fixture is
> constructed to make a mechanism visible; it is therefore also
> constructed — accidentally — to make everything else invisible.**

#### Why the synthetic arm was blind, exactly

On `v4-rgb-mab-chromatic-black.icc`, `InitialLab.L*` and
`outRamp[first]` are **both `L* 20`**. The 4.2.5.4 defect swapped one
for the other. **Swapping two equal numbers changes nothing**, so the
arm's `5,000 000 ΔE76` was identical before and after the correction —
not approximately, *identically*, and its divergence remains **100 %
chroma with `ΔL*` exactly 0**. ★ **The fixture that was authored to
discriminate the two ESTIMATORS had, by construction, zero power to
detect a defect in the RETURN VALUE of one of them.**

The vendor profile had the power because nobody designed it: on
`USWebCoatedSWOP.icc` those two quantities are `11,772 365` and
`16,489 806`, and they differ **because a real ink set makes them
differ**.

#### ★ The relationship to DL-020, stated so neither entry is misread

**DL-020 is not weakened.** A doubt the corpus cannot discharge is still
discharged by bytes this project authors — that is how GP-001 worked
and how the estimator discrimination worked. **What is added is the
boundary:** an authored fixture discharges *the doubt it was authored
for* and **nothing adjacent to it**, because the author's model of the
mechanism is baked into the bytes. **A vendor profile is the only
member of the corpus that was not written by someone who already had a
hypothesis.**

#### What this does NOT decide

- **It does not make the vendor profile ground truth.** It has no
  published black point; it is a *fixture*, not an oracle
  (rule 3 unchanged).
- **It does not say synthetic fixtures are weak evidence.** They are the
  only instrument that can isolate a mechanism, and §3.11's
  derived-expectation rows are the strongest non-published rows in the
  ledger.
- **It does not license adding vendor profiles freely.** Licensing and
  redistribution of third-party `.icc` files is unexamined here; this
  entry is about **keeping** the arm that exists, not about acquiring
  more.

**Revisit if:** a second vendor profile is added (the "one real arm"
policy should become a stated minimum with a rationale for the number);
or a synthetic fixture is authored that *does* separate `InitialLab`
from `outRamp[first]`, which would give the authored corpus the power it
lacked here and is the cheapest way to make this entry's warning
concrete rather than historical.

> #### ★★★ Dated note, 2026-08-12 (fourth filing, tip `6c7cda1`) — **the second "Revisit if" has FIRED, and the measurement that fired it CORRECTS this entry's headline**
>
> **The fixture exists.** `fixtures/synthetic/v4-rgb-mab-floored-b2a.icc`
> *(verified — the file is on disk, and `tools/gen-profiles/src/recipes.rs`
> L1150–L1152 and L1253–L1264 author `InitialLab` at `L* 12,5` against an
> `outRamp[first]` floor at `L* 37,5`, **25,0 `L*` apart by
> construction**)*. The authored corpus now has the power this entry
> recorded it as lacking. `NUMERIC_CLAIMS.md` **§3.29**, **NC-179**.
>
> **★★★ And the entry's headline sentence — *"the vendor profile was the
> only arm with the power"* — is now measured, and it is FALSE in the
> sense a reader would take it.** `icc-conformance` reverted `fd34a44`
> in a detached worktree and ran the whole conformance suite
> *(**carried** — this librarian has no shell)*: **no row went red on any
> machine.** The `swop` arm is *differential* — its numbers move — but
> **not load-bearing**: the two rows that moved are graded against `1`
> and observed `5,18×10⁻³` and `4,26×10⁻²`, nowhere near their bounds,
> because the row that carries the finding is **`REPORTED`** (DL-019,
> and see **NC-176**).
>
> **The distinction this entry must now carry, and it is the whole
> lesson:** the vendor arm had the power to **OBSERVE** the divergence —
> that is true, was measured, and is why §3.25.4 was right to say the
> synthetic arm saw nothing. It never had the power to **CATCH A
> REGRESSION** in it, because observation and detection are different
> capabilities and only the second needs a bound the observation can
> cross. **DL-036 conflated them**, in the one entry written to warn
> about a fixture's power. See **DL-040**, which is the rule that
> separates them.
>
> **What is NOT weakened:** the *converse-of-DL-020* argument above, and
> the decision to keep the vendor profile in the corpus. **A synthetic
> fixture authored by someone who already has a hypothesis still cannot
> surprise its author**, and the new fixture is no exception — it was
> authored to separate two candidates it was already known ought to
> differ. What it can do is **fail**, which is DL-020's own test.

---

### DL-037 — ★★★ **candidate separation is an EMITTED FIELD with its own guard order, and the guards are the decision.** `UNGRADED` is tested **before** the comparison and `ZERO-SEPARATION` **outranks everything**, because otherwise the mechanism blames the fixture for a decision the tolerance made

**Date:** 2026-08-12 · **Occasioned by:** **DL-033**'s own *"Revisit
if"* clause — *"a cross-check is built whose candidate separation is
stated in advance, at which point this rule has a worked positive
example instead of only its founding negative one"* · **Designed and
built by:** `icc-conformance` · **Filed by:** `icc-librarian` from
`tools/difftest/src/lib.rs`, read at the tip `e26d9ba` *(verified — the
`SepPower` enum at L1441–L1476, the guard chain at L1653–L1695, and
`separation_counts` at L1865–L1902)* · **Relates to** **DL-033** (the
rule this implements), **DL-019** (report-don't-grade), **DL-018** (a
gate must not reward deletion), **DL-031** (a count needs its
apparatus), **DL-036** and `NUMERIC_CLAIMS.md` **§3.28**

#### The decision

> **Every conformance record states how far away the nearest plausible
> RIVAL answer was, in the same units as the metric, as a field the
> harness computes — not as prose. Three stated states, no fourth:
> `measured` (a named rival, its value, the distance), `no named
> alternative` (somebody looked, and the reason is carried), and
> `unstated` (nobody has considered this row). A blank is not one of
> them.**

#### ★★★ The guard order, which is the load-bearing part

The classification runs in this order, and **each position is an
argument** *(verified — `lib.rs::separation_power`)*:

| # | Guard | Why it sits there |
|---|---|---|
| 1–2 | `Unstated` / `NoNamedAlternative` | nothing to compare; state 2 records that a **person looked** |
| 3 | `NaN` → **`sep-broken`** | apparatus breakage is **said out loud**, never classified into a verdict |
| 4 | distance `== 0` → **`ZERO-SEPARATION`** | ★ **checked before everything below it**, because a row whose candidates are the same number has no power **at any tolerance and in any units** |
| 5 | incommensurable units → **`incommensurate`** | the number is emitted, **the test is not run** |
| 6 | tolerance not finite → **`UNGRADED`** | ★★★ **checked BEFORE the comparison** |
| 7 | `distance <= tolerance` → **`BLIND`**, else **`DISCRIMINATING`** | `<=` deliberately matches `Record::graded`'s own pass rule, so *"the separation is exactly the tolerance"* reads identically in both places: **not** discriminating |

**Why 6 must precede 7, stated as the arithmetic it is:** `d <= inf` is
true for **every** finite `d`. A naive implementation that compared
first would brand **every REPORTED row `BLIND`** — ★ **blaming the
fixture for a decision the TOLERANCE made.** Under **DL-019** this
project deliberately reports-and-does-not-grade wherever a disagreement's
authority does not exist, so that mislabelling would have fired on
exactly the rows DL-019 created, and the new mechanism's very first
output would have been a systematic slander of the corpus.

**Why 4 outranks the rest:** the two states differ **in their remedy,
not in their severity**. *A blind row can be rescued by tightening a
tolerance; a zero-separation row can only be rescued by a different
fixture.* Collapsing them would put a fixture-authoring job into a
bucket labelled "adjust a number". The two id lists are emitted
**separately** for the same reason.

#### ★★ `BLIND` does not affect status or exit code, and that is deliberate

A blind row **passes**, and the aggregate says so in a line nobody's CI
gate reads. **This is not an oversight and it is not weakness.**

> **If stating a separation could turn a green row red, the cheapest
> response to a blind row is to stop stating separations.** The field is
> voluntary in exactly the way that keeps it honest: **`unstated` is
> printed as `UNSTATED`, not as a blank**, so declining to state one is
> *visible* — but it is never *punished*.

★ **This is DL-018 read from the other end.** DL-018 says an upper-bound
gate on a deliberate cost must be paired with a prediction pin, because
otherwise **deleting the requirement makes the gate greener**. Here the
same pressure is removed at the source: there is no gate to make
greener, so there is no incentive to delete the evidence. **A
disclosure mechanism and an enforcement mechanism must not be the same
mechanism.**

#### What the first run found, and it was about our own instruments

1. **The row carrying the entire 4.2.5.4 finding is `UNGRADED`** —
   tolerance `inf`, separation `4,717 441`, **so it could never have
   failed however far the candidates moved.** The suite's real power on
   that question lives in §B's *device* rows, **which is not where
   anyone would look**: the row named `estimators` is the one that
   cannot grade estimators. `NUMERIC_CLAIMS.md` **NC-176**.
2. **A fourth instance of DL-034**, found **by the apparatus rather than
   by a person**, on the first run of the field that computes it
   (**NC-178**, §3.28.3).

#### What this entry does NOT decide

- **It does not close DL-033's operational item 1.** **16 of 145 rows**
  state a separation, all of them Pass 5c's. See the dated note under
  DL-033.
- **It does not make `blind=0` a statement about the suite.** It is a
  statement about the **six** rows that reached guard 7.
- **It does not make a stated separation TRUE.** The rival candidate is
  named by a human, from the two implementations' sources; **a wrong or
  missing rival produces a confidently wrong separation**, and nothing
  here detects that. **DL-023** — *state what the two implementations
  were free to disagree about, from their sources, before the run* — is
  the discipline that guards the input, and it is unchanged and
  unaided by this.
- **It does not apply to `machine-timing` rows**, where the rival
  candidate is not a different answer but a different run, and
  **NC-173**'s withdrawal is the precedent for what to do there instead.

**Revisit if:** a `sep-broken` row ever appears (the NaN state has never
fired and its handling is untested by anything but a unit test); or a
row's stated rival is later found to have been the wrong rival, which is
the failure mode this entry explicitly does not cover and would need its
own rule; or `BLIND` is ever proposed as a gating condition, at which
point the pressure argument above must be answered rather than
overlooked.

> #### ★★ Dated note, 2026-08-12 (fourth filing, tip `6c7cda1`) — **the guards were right and the ARITHMETIC UNDER THEM was wrong.** See **DL-038**
>
> Nothing in the guard order above is changed, superseded or regretted.
> What was found one filing later is that **the `distance` the guards
> classify was itself derived by a formula that collapses to zero on the
> exact run the row exists to catch** — so guard 4 (`ZERO-SEPARATION`,
> the one that outranks everything) fired on a row that was **failing at
> `2,500 019×10¹` at that moment**. **DL-038** records it. Note that
> this is **NOT** the *"a row's stated rival was the wrong rival"* case
> deferred in the "Revisit if" above: **the rival was right.** That case
> is still uncovered and still needs its own rule.

---

### DL-038 — ★★★ **a candidate separation must be a property of the FIXTURE, not of the RUN.** `|observed − alt_observed|` is the wrong formula wherever the alternative is *"the code returns the other candidate"*, because on the defect run `observed` **becomes** `alt_observed` and the mechanism disclaims its power in the instant it demonstrates it

**Date:** 2026-08-12 · **Occasioned by:** the proof-of-power run for the
third Pass 5c arm — `icc-conformance` injected the pre-`fd34a44` return
value and the new clause row **failed and printed `ZERO-SEPARATION` on
the same line** · **Found and fixed by:** `icc-conformance` · **Filed
by:** `icc-librarian` from `tools/difftest/src/lib.rs`, read at the tip
`6c7cda1` *(verified — the two constructors at L1405–L1435 and the
★★ doc block at L1382–L1404, which carries the diagnostic question in
the source itself)* · **Relates to** **DL-037** (the guard order this
sits underneath), **DL-033** (the rule both implement), **DL-034** (a
claim-bearing number is computed, not typed), **DL-025** (a control is
only as good as its fixture) and `NUMERIC_CLAIMS.md` **§3.29**

#### The decision

> **`Separation::against` — which derives `distance = |observed −
> alt_observed|` — may be used ONLY where the alternative is a different
> READING APPLIED TO THE SAME OBSERVATION, so that the two observations
> genuinely coexist on the same run. Where the alternative is a
> different ANSWER THE CODE COULD RETURN, the distance is a property of
> the fixture and must be SUPPLIED, via `Separation::against_distance`.**
>
> **The diagnostic, and it is now written on the constructor's own doc
> comment:** *ask whether the distance is a property of the RUN or of
> the FIXTURE.*

#### ★★★ Why the derived form is not merely imprecise but actively inverted

The row watches for one thing: the library returning `outRamp[first]`
where ISO/CD 18619 **4.2.5.4** requires `InitialLab`. While the code is
correct, `observed ≈ InitialLab`, `alt_observed = outRamp[first]`, and
the derived distance is the real `25 L*`. **The moment the code is wrong
in precisely the way the row is watching for**, `observed` *becomes*
`outRamp[first]` — the same number as `alt_observed` — and the derived
distance is **exactly `0`**.

So the field reports **`ZERO-SEPARATION`**, whose meaning under DL-037
is *"this row has no power at any tolerance and only a different fixture
can rescue it"*, **on the one run in the project's history where that
row had just proved its power by failing at `2,500 019×10¹` against a
`7,629 511×10⁻⁴` bound.**

★ **The separation is `25 L*` whichever answer the library happens to
return today.** It was authored into the bytes (`FLOORED_BLACK_L =
12,5`, `FLOORED_ROUNDTRIP_L = 37,5`) and it does not become smaller
because the code got worse. **A quantity that describes the fixture must
not be computed from the run.**

#### ★★ Why this belongs in the decision log and not in a commit message

**DL-037 recorded the guard ORDER as the design.** This records that the
**measurement underneath those guards had the same defect the guards
were built to catch** — a mechanism reporting a confident,
wrong-in-the-safe-direction verdict about its own power. The guards
could not have caught it: they classify the number they are handed, and
the number was `0`.

★ **And the way it was found is the reusable part: it was found by
USING the instrument on the very case it was built for, one filing after
the instrument was celebrated.** Reading the code again would not have
found it; the derived formula is *correct* in every state except one,
and that one state does not occur while the suite is green. This is
**DL-025's rule** — *a control is only as good as its fixture* — arriving
at the same place from the other side: an instrument is only as good as
the run you are willing to break on purpose.

#### Scope — three rows moved, and which kinds they were

*(**carried** from `icc-conformance`; this librarian read the two
constructors and the doc comment, not a diff)*

| Kind of row | Why the derived form was wrong there |
|---|---|
| the new `CLAUSE/4.2.5.4-…` row | the alternative is literally *"the estimator returns the other branch's value"* |
| two `0/1` indicator rows | their two candidates are **always exactly one apart** — a property of the encoding, not of any run |

**Rows NOT moved, and this is the boundary:** every row whose rival is a
*rival reading* — lcms2's `wtpt` predicate read three ways, a legacy-Lab
decode read as v2 or v4 — keeps `against`, because there the two
observations really do coexist: one run, one set of bytes, two
arithmetics applied to it.

#### What this does NOT decide

- **It does not make a supplied distance true.** `against_distance`
  moves the responsibility to the caller and says so in its own doc:
  *"the caller states the distance and is responsible for it being in
  the units it claims."* **Nothing checks it.** The unit test that
  guards the authored `25 L*` (`the_floored_fixture_separates_4254s_two_candidates`)
  is a check on the **fixture**, not on the constructor's argument.
- **It does not retro-audit the earlier separations.** The Pass 5c rows
  filed at the seventeenth filing (**NC-176 … NC-178**) were not
  re-derived here. **Which of them use which constructor is not on
  file**, and a count of rows moved is not an inventory of rows checked.
- **It does not cover a wrong rival.** DL-037 deferred that and it is
  still deferred.

**Revisit if:** any row is added whose alternative is *"the code returns
the other candidate"* and which uses `against` (the fix was three named
rows, not a mechanism that prevents the fourth); or a lint/test is
proposed that could enforce the FIXTURE-vs-RUN distinction structurally
— it currently lives entirely in a doc comment and a reviewer's
attention.

---

### DL-039 — ★★ **a rival TOLERANCE is not a rival CANDIDATE; and where several rivals exist, the row names the one that MOST THREATENS it.** Both halves exist to stop a disclosure field from turning into a second, undocumented gate — or into a flattering one

**Date:** 2026-08-12 · **Occasioned by:** writing candidate separations
for Pass 4c's ten rows, six of which turned out to have **no** named
rival · **Derived by:** `icc-conformance` · **Filed by:**
`icc-librarian` from `tools/difftest/src/pass4c.rs`, read at the tip
`6c7cda1` *(verified — the split-out `is_display_class` predicate at
L486–L495 with its stated reason, the three-reading rows at L754 and
L778–L788, and the unit test at L1244–L1280 asserting the three readings
give **different** counts)* · **Relates to** **DL-037** (the field this
governs), **DL-023** (state what the two implementations were free to
disagree about, from their sources, before the run), **DL-005** and
`NUMERIC_CLAIMS.md` **§3.29**

#### The decision, in two halves

> **(a) A rival TOLERANCE is not a rival CANDIDATE.** If the only
> alternative anyone can name for a row is *"the bound could have been a
> different number"*, the row has **no named alternative** and says so
> with that reason. Writing it into the separation field would make the
> field a second gate — one with no derivation, no owner and no entry in
> `TOLERANCES.md`.
>
> **(b) Where several rivals exist, name the one that MOST THREATENS the
> row**, and enumerate the others. Naming the rival that happens to
> flatter the row is exactly the tuning the mechanism exists to prevent.

#### Why (a) is not pedantry

A separation says *"the nearest plausible **answer** was this far
away"*. A tolerance says *"we will accept this much error"*. They have
the same units and they are not the same kind of thing. **Conflating
them lets a number that was never derived acquire the authority of one
that was**: a reviewer sees a figure in a column headed `separation`,
reads it as evidence about the fixture's power, and it is actually
somebody's opinion about the bound.

The live instance is Pass 4c's **sensitivity floor**: the only
alternative nameable for that row is a different floor. **That is a
tolerance question, and `TOLERANCES.md` §3.4.5 is where it is already
answered** — the floor of `100×` is transcribed from Pass 4b's accepted
counterfactual band, and the observation is `2 310×`. The row therefore
carries `no-named-alternative` **with that reason attached**, which is
DL-037's state 2 doing exactly the work it was defined for.

★ **Six of Pass 4c's ten rows are honest absences of this shape.** The
media-relative floors have no rival because lcms2 consults the media
white point **only** for the ICC-absolute adjustment — there is no
second reading for it to have. **`no-named-alternative` with a reason is
a finding; `no-named-alternative` as a way of clearing `UNSTATED` off a
report is vandalism**, and the difference is entirely in whether the
reason is real.

#### Why (b) needed to be written down

lcms2's `wtpt` substitution predicate has **three** live readings — the
conjunction lcms2 implements, `ICC.1:2022` **9.2.36**'s class-only rule,
and the disjunction. A row may price itself against any of them, and
**they do not give the same count on the same pair of profiles**
*(verified — the unit test at `pass4c.rs` L1244 asserts precisely that
they differ, so a fixture pair that made all three agree could not be
adopted silently)*.

★ **A row that prices itself against the reading it beats most easily
has disclosed nothing and looks rigorous.** The requirement is
therefore: **the rival named is the one that most threatens the row**,
and the others are enumerated in the same string so a reader can see
that the choice was made rather than defaulted.

#### The worked figure

The Pass 4c row `icc-librarian` was asked to note by name lands at
**`2,055 76×10⁻¹`** against a bound of **`5×10⁻⁴`** — **411×**, and ★
**exact rather than modelled**: it is computed from the two readings'
own arithmetic, not from an envelope. **NC-186.**

#### What this does NOT decide

- **It does not say a row with no rival is weak.** Six of ten is not a
  deficiency; on those rows there genuinely is nothing else the
  implementations were free to do (**DL-023**'s discipline, applied
  before the run rather than after it).
- **It does not give a procedure for finding rivals.** They are found by
  reading two implementations' sources. Nothing automates that and
  nothing checks that the search was thorough.
- **It does not apply to `machine-timing` rows** — DL-037's exclusion is
  unchanged.

**Revisit if:** a row is ever found stating a separation whose "rival"
is a bound (half (a) violated in practice rather than in theory); or a
fixture is adopted on which the three `wtpt` readings coincide, which
would silently remove the separation the unit test exists to protect.

---

### DL-040 — ★★★ **a large separation on an `UNGRADED` row is a REQUEST FOR A FIXTURE and a graded row elsewhere — never a licence to grade that row.** Ask what clause the number would be graded against; if the answer is *"none, but it would have caught the bug"*, the bound is fitted to the bug

**Date:** 2026-08-12 · **Occasioned by:** `icc-engineer` asking directly
whether **NC-176**'s `4,717 441` separation now justifies putting a real
tolerance on the row that carries the whole 4.2.5.4 finding · **Answer:
NO**, from `icc-conformance`, adopted by the engineer and filed here as
a rule · **Filed by:** `icc-librarian`; the reasoning is **carried** from
the dispatch and **corroborated** in `docs/TOLERANCES.md` §4's fourth-filing
row *(verified — read at the tip)* · **Relates to** **DL-019**
(report-don't-grade), **DL-026** (a permanently ungraded row), **DL-037**
(the field that made the question askable), **DL-036** and
`NUMERIC_CLAIMS.md` **§3.29**

#### The decision

> **When a `REPORTED, NOT GRADED` row turns out to carry a large
> candidate separation, the correct response is to build a fixture and a
> graded row SOMEWHERE ELSE — one whose expectation comes from a clause.
> The `inf` stays.**

#### The three reasons, and each of them generalises

1. **★ There is nothing for a bound to MEAN.** Since `fd34a44` both
   implementations return a quantity **their own document calls
   `InitialLab`**, and the two documents mean different things by the
   name. **No clause requires two implementations of two different
   documents to agree.** A bound here would grade iccce against lcms2's
   reading of a specification iccce does not implement — inventing an
   authority, which is the exact failure **DL-019** was written to
   prevent.
2. **★★ Any bound below `4,717 441` is fitted to ONE KNOWN DEFECT.**
   Anything under it fails the pre-`fd34a44` build; anything over it
   does not. **That is not a tolerance, it is a memory of a bug wearing a
   tolerance's clothes** — project rule 5 ("tolerances are justified, not
   tuned") violated in the one direction that feels virtuous.
3. **★ It could not be ONE number anyway.** The three arms observe
   **`4,799`**, **`5,000`** and **`10,000`** *(carried)*. A single
   constant across them would be a maximum-of-unlike-things; three
   constants would be three numbers each fitted to its own fixture.

#### ★★★ The test, stated so it can be applied without re-deriving all of this

> **Ask what clause the number would be graded against.**
> **If the answer is *"none, but it would have caught the bug"*, the
> bound is fitted to the bug.**

And the constructive half, which is what actually happened here: the
separation was a **request**, it was honoured, and the honoured form is
**a fixture whose expectation is a named constant put through a clause**
— tolerance `7,629 511×10⁻⁴`, derived from **one encoding quantum and
nothing else**, and the reversion now fails it by four orders of
magnitude (**NC-179**, **DL-043**'s sibling case). ★ **The bug is caught
by a row that would exist even if the bug never had.**

#### Why this is not the same as DL-019 or DL-026

- **DL-019** decides *when* to report rather than grade (the mechanism
  is known, the authority does not exist).
- **DL-026** decides that one specific such row stays ungraded
  **permanently**, because the standard binds reading profiles and not a
  CMM's computed output.
- **DL-040** decides what to do **when new evidence of that row's
  importance arrives** — which is the pressure DL-019 and DL-026 do not
  address, and the pressure is real: a big number in a column labelled
  `separation` reads like an argument for grading it.

#### What this does NOT decide

- **It does not say `inf` rows are fine as they are.** The row's power
  is real and **no gate consumes it**; the debt is a *fixture*, and it
  stays owed until the graded row exists.
- **It does not close NC-176.** NC-176 is unchanged, still `UNGRADED`,
  still the largest unconsumed separation in the ledger.
- **It does not authorise deleting a REPORTED row** to tidy the census.

**Revisit if:** a clause is ever published that requires the two
estimators' results to agree (at which point reason 1 evaporates and the
row becomes gradeable on an authority, not on a memory); or ISO 18619
leaves committee-draft status, which would change what "their own
document" means on iccce's side.

---

### DL-041 — ★★★ **published ground truth for the LUT path CANNOT EXIST — this is STRUCTURAL, not an availability gap — while the NON-LUT half is now available and unmeasured.** The item that has been "owed" for eleven filings is split, and the three kinds of blocker are kept apart

**Date:** 2026-08-12 · **Occasioned by:** `icc-engineer` dispatching
`icc-spec-librarian` to *close the standing gap or classify it as
structural* — a line this ledger has carried since Pass 1 ·
**Researched by:** `icc-spec-librarian` · **Filed by:** `icc-librarian`
from `D:\Dev\Rag-Specialized\ICC_Spec\icc\icc__ref__ground_truth_availability.md`,
read at the tip *(verified — the verdict table §1, the structural
argument §8, and the three-kinds table §7)* · **Relates to** **DL-002**
and **DL-014** (corpus tiers and citation terms), **DL-019**, **DL-026**
(the conformance clause binds reading, not computing), **DL-036**,
rule 3 (*expected values come from the literature*) and
`NUMERIC_CLAIMS.md` **§3.29** / **§5**

#### The decision, in the words the ledger must use

> **Not *"we have not found published ground truth for the LUT path"*
> but *"the specification is written so that none can exist."***

#### Why it cannot exist — the load-bearing clause is the second

1. **A profile is a vendor artifact.** A CLUT is one vendor's fit to
   measured data; there is no *correct* CLUT for a dataset, so there is
   no correct output to publish.
2. **★★★ Interpolation between grid points is UNSPECIFIED.** ICC.1
   mandates neither tetrahedral nor trilinear. **Two conforming CMMs are
   permitted to return different numbers for the same profile and the
   same input** ⟹ *no single value could be published as the expected
   result even in principle.*
3. **"Perceptual" is vendor-defined by design** — the standard says what
   the intent is *for*, not what it *computes*.
4. **Out-of-gamut handling is delegated** to the CMM.
5. **★ ICC's conformance clause binds only *reading*** (**DL-026**'s
   premise): there is no clause a published expected output could be
   normative under.

**Corroborated from the strongest available direction:** ICC's own
reference implementation, **iccDEV** (BSD-3, the renamed DemoIccMAX),
**ships zero expected colour values.** Its `RunTests.sh` compares
nothing; `ApplyDataFiles/` holds inputs only; its two expectation
manifests are structural (`invalid-saved` / `parse-fail` /
validation verdicts), not colorimetric. ★ **The consequence for this
project is sharp: `icc-conformance` cannot promote agreement to ground
truth by finding a better implementation.** Comparing against iccDEV
would produce a **second `implementation-cross-check` row**, not a
`published-ground-truth` one.

#### ★★ What is now AVAILABLE, and the line this entry refuses to cross

**ICC.1:2022 Annex D.6.3** prints an input, every intermediate, and
**twelve exact integer PCS encodings**; **all twelve reproduce to the
digit** — *reproduced by `icc-spec-librarian`'s own arithmetic in the
corpus, from the printed Table D.3 inputs*. **Table 16** is
**normative** and its five value↔encoding pairs also reproduce.

> **★★★ NOTHING IN iccce HAS BEEN COMPARED TO EITHER OF THEM.** The
> ground truth is **available**; the row is **buildable**; **the row
> does not exist.** `NUMERIC_CLAIMS.md` §5's sentence — *NC-001 is the
> project's only `published-ground-truth` row* — **remains true at this
> tip**, and this entry must never be cited as having closed it.

**Three qualifications that travel with every citation of Annex D.6.3:**
(a) **Annex D is INFORMATIVE** — it is ground truth *epistemically*
(numbers published by the standards body, not produced by an
implementation) and **not normatively**; a CMM that disagrees is not
thereby non-conforming; (b) displayed precision is 4 dp, so **the exact
part is the twelve integers**; (c) **Table D.2's black `X = 0,009 7` is
defective** and the fixture must start at **Table D.3** (corpus register
`A47`).

#### ★★ The three kinds of blocker, kept apart deliberately

| Kind | Where it applies | Statement |
|---|---|---|
| **EXISTENCE** | LUT/CLUT transform results; iccDEV's suite; an ECI/Fogra residual; sRGB worked triples | **Nobody publishes these. Not withheld — not produced.** |
| **AVAILABILITY** | IEC 61966-2-1; ISO 15076-1; ISO 12647-x; CGATS TR001 | Paywalled or purchase-only. **Money and operator time would obtain them.** |
| **ACCESS TERMS** | `color.org` (ToS naming AI), `fogra.org/fileadmin/` (`robots.txt`), `itu.int` (WAF rejection) | **Obtainable by a human, not by this agent** — and by *three different mechanisms*, which is why they cannot be one row |

★ **Conflating these is the error this table exists to prevent.** "We do
not have it" has three completely different remedies: build the fixture
differently, spend money, or ask the operator to click. **ICC.1 §7's
prior entry (the PDF blocker) is the precedent** — *"free download"
never implied *"automated retrieval permitted"*.

#### The consequence for the ledger's owed column

**The permanently-owed item is converted, not discharged.** It becomes:

- **STRUCTURAL** for the LUT path — *relabel, do not chase.* The correct
  arm is, and will remain, a differential; the correct response to a
  large divergence is **a second independent implementation lineage**
  (iccDEV, BSD-3 — **Argyll remains BARRED**, AGPL-3.0), so that
  agreement stops being a single observation.
- **OPEN AND CHEAP** for PCS encoding, XYZ↔Lab, media-relative white
  scaling and the ICC-absolute direction — where the numbers now exist
  and the fixture does not.

#### What this does NOT decide

- **It does not authorise putting published numbers in this repository.**
  Whether ICC's Annex D values, CIE's CC BY-SA tables and ECI's
  self-contradicting `cprt` may live in an MIT repo as fixtures is **one
  operator question, still open**, and rule 9 is unchanged.
- **It does not make the corpus's reproduction an iccce measurement.**
  See the boxed sentence above.
- **It does not close the ICC-absolute direction question by itself** —
  the empirical confirmation (`× mediaWhite / D50`) is a corpus
  measurement on a real profile, not a row in this project's suite.

**Revisit if:** CGATS TR001 is obtained (it settles register `A47` from
the source data); or a second CMM lineage enters the differential; or
ICC publishes conformance material with expected colour values, which
would falsify the structural argument outright and is the thing most
worth watching for.

---

### DL-042 — ★★★ **a negative finding removes its own auditor. When an item has been owed for many cycles, RE-AUDIT THE REASON IT IS OWED, not just the item**

**Date:** 2026-08-12 · **Occasioned by:** the discovery that the
eleven-filing "no published ground truth" gap was **partly a corpus
defect**: the ICC.1:2022 Annex D data had been examined months earlier
and **REJECTED**, by point-evaluating values that were intervals ·
**Found by:** `icc-spec-librarian` (its corpus defect **C5**) ·
**Filed by:** `icc-librarian` as a convention binding **this** role ·
**Relates to** **DL-041**, **DL-031** (a count is not an inventory),
**DL-034** (a stale literal), and every `§7.x` status pass in
`NUMERIC_CLAIMS.md`

#### The decision

> **An item that has been restated as owed across many filings gets its
> REASON re-read, not merely its status re-stated. Restating a blocker
> is not re-testing it.**

#### Why the failure mode is specific to NEGATIVE findings

The corpus's four previous defects were all wrong **assertions** — "X is
true" when X was false — and **all four were caught within days**,
because an assertion is used, and using it exposes it.

★ **This one was a wrong REJECTION**, and it survived indefinitely.
**Nobody re-tests a fixture they have been told is broken.** A negative
finding is self-sealing: it removes the very traffic that would have
audited it, and the longer it stands the more authoritative it looks —
each restatement is read as another confirmation when it is in fact the
same observation, copied.

The mechanism of the original error is worth carrying too, because it is
cheap to repeat: **a displayed value is an interval, not a point.** A
4-dp printed number is `x ± 5×10⁻⁵`; evaluating it as a point and
finding an inconsistency proves nothing. The corpus's own generalisation:
**re-running a point evaluation at higher precision is not a second
pass — it is the same pass.**

#### ★★ What this obliges THIS role to do, concretely

**Eleven consecutive filings restated *"no published ground truth for any
transform"*. None re-read the entry that created it.** That is a
librarian failure, not a corpus failure — the corpus published a
finding; this ledger **propagated** it without ever asking what it rested
on.

So, binding on the `§7.x` status passes:

1. **A "still owed, unchanged" line is not free.** After roughly five
   restatements, the status pass must either re-read the *source of the
   blocker* or record explicitly that it did not.
2. **Distinguish "nobody has done it" from "somebody looked and said
   no".** The second kind carries a **finding** underneath it, and a
   finding is a claim like any other — with an author, a date, a method,
   and the possibility of being wrong.
3. **Record the method of a rejection when filing one.** *"Examined and
   unusable"* is unauditable; *"examined by point evaluation of the
   printed 4-dp values, and the black row was inconsistent"* is the
   sentence that lets the next reader find the error.

#### What this does NOT decide

- **It does not say the corpus was careless.** The rejection was made
  with a real method that was wrong in one specific way, and the same
  agent found and retracted it.
- **It does not make an old negative finding suspect by default.** It
  makes an old negative finding **re-auditable on a schedule**, which is
  different.
- **It does not apply only to the corpus.** ★ **This ledger's own
  refusals are the same shape** — every `no-named-alternative`, every
  `REPORTED, NOT GRADED`, every *"not comparable at this intent"* skip
  is a negative finding that removed its own auditor. **DL-039** already
  requires their reasons to be stated; this entry is why the reasons must
  be re-read.

**Revisit if:** a second wrong rejection is found (two instances would
justify a mechanical review cadence rather than a judgement call); or a
`§7.x` status pass ever discharges an item by re-reading its blocker,
which would be the first worked positive instance of this rule.

---

### DL-043 — ★★ **an exemption from a control is DECLARED IN ADVANCE and GRADED against the declaration. It is never acquired by the measurement coming out small**

**Date:** 2026-08-12 · **Occasioned by:** the third Pass 5c arm making
an existing control row — `apparatus/error-bar-is-smaller-than-the-effect`
— fail at **`3,775×10⁹`**, because the new fixture's floor makes
`d(device)/d(L*)` zero by construction · **Designed by:**
`icc-conformance` · **Filed by:** `icc-librarian` from
`tools/difftest/src/pass5c.rs`, read at the tip `6c7cda1` *(verified —
`DEVICE_OBSERVABLE` at L1279–L1301, the lookup at L1303–L1313, the row
that grades measurement against declaration at L1642–L1665, the
unchanged `APPARATUS_RATIO` at L1337–L1356, and the two-way arm-coverage
test at L2771–L2795)* · **Relates to** **DL-018** (a gate must not
reward deletion), **DL-025** (a control is only as good as its fixture),
**DL-028** (an apparatus fault is not a finding), project **rule 5**
(tolerances are justified, not tuned) and `NUMERIC_CLAIMS.md` **§3.29**

#### The decision

> **When a control legitimately does not apply on some arm, the
> exemption is written into an AUTHORED TABLE naming that arm, and a
> further row grades the MEASURED property against the table's
> DECLARATION. The control's own constant does not move.**

#### The situation, and why the obvious fix was the wrong one

§B's apparatus check converts an error bar between units using
`d(device)/d(L*)`, **which presupposes that derivative is non-zero**.
The new `floored` fixture's `B2A1` has a floor on `G`, so on that arm the
derivative is **`1,1×10⁻¹⁶`** — zero by construction — and the ratio
explodes to `3,775×10⁹`. *(Measured values **carried**; the arm's
`false` declaration and the three arms' derivative magnitudes are
**verified** in `pass5c.rs` L1278 and L1290.)*

**`APPARATUS_RATIO` was NOT widened.** Its `1.0` is unchanged and still
applies **everywhere the conversion it needs exists**. What was added:

- **`DEVICE_OBSERVABLE`** — one authored line per arm, declaring whether
  a device-side observation of a black point's `L*` is possible at all
  (`swop` and `synthetic` **true**, `floored` **false**);
- **a row grading the measured `d(device)/d(L*)` against that
  declaration**, so a mismatch in **either** direction is a failure;
- **a two-way test** that every arm in the table is driven by `run()`
  and every arm `run()` drives has a line in the table — because a
  declaration table whose keys have drifted silently exempts nothing or
  everything.

#### ★★★ Why the distinction is the whole entry

**Widening `APPARATUS_RATIO` to `4×10⁹` would have made the suite green
and the control meaningless** — on every arm, forever, including the two
where the conversion is real and the check is load-bearing. And it would
have been *reasonable-looking*: the number that made it green is a
number the run produced.

> **★ An exemption acquired by a measurement coming out small is
> indistinguishable from a defect coming out large.** The declaration
> inverts the burden: the arm is exempt because someone WROTE DOWN that
> it is exempt and said why, and the code then checks that the world
> agrees with the writing.

★ **This is DL-018 in a new position.** DL-018 says a gate must not
reward deleting the requirement; here a gate must not reward the
requirement *evaporating on its own*. **Both are the same pressure: the
cheapest route to green must never be the one that removes the
evidence.**

#### What this does NOT decide

- **It does not say a zero derivative is a defect.** On the `floored`
  arm it is the fixture's **purpose** — the floor is what separates
  4.2.5.4's two candidate return values by `25 L*`.
- **It does not make the declaration true.** `DEVICE_OBSERVABLE` is
  authored; the grading row checks the *measurement* against it, which
  catches drift but **not an original mis-declaration**.
- **It does not generalise to "any failing row may be declared
  inapplicable."** The exemption is admissible **only** where the
  quantity the control needs **does not exist**, not where it exists and
  is inconvenient. That boundary is a judgement and this entry does not
  automate it.

**Revisit if:** a third arm is added and its `DEVICE_OBSERVABLE` line is
written **after** its first run rather than before (the declaration
would then be fitted to the observation, which is exactly what this
entry forbids, and nothing in the code can detect the ordering); or a
second control acquires a declaration table, at which point the pattern
should be factored rather than copied.

### DL-044 — ★★ **iccce has a NAMED EXTERNAL CONSUMER and a standing bidirectional channel to it. Rule 9's dependency classification gains a SECOND AXIS — a consumer's CI gates — and the adoption case for that consumer is CONFORMANCE, never accuracy**

**Date:** 2026-08-17 · **Occasioned by:** the operator creating
`D:\Dev\FeatureRequests\iccce_FeatureRequests\` on 2026-08-17 as a
bidirectional request channel to the `pdfce` session at `D:\Dev\pdfce\`,
and `pdfce` filing three items into it the same day · **Assessed by:**
`icc-engineer` · **Filed by:** a **general-purpose agent standing in for
`icc-librarian`** — the agent type could not be dispatched from a
session whose working directory is `D:\Dev\pdfce`, so `iccce`'s project
agents were not in its roster; it followed
`.claude/agents/icc-librarian.md` as its operating contract and **had a
shell**, which the real agent does not · **Relates to** project **rule 9**
(classify every dependency), **rule 3** (name the oracle), **rule 1** (a
wrong colour looks like a right one), **DL-024** (publication is the
operator's act), **DL-031** (a count is not an inventory), **DL-041**
(the LUT path's ground truth cannot exist), **DL-042** (re-audit the
reason an item is owed), and `CLAUDE.md` **rule 10**

#### The decision

> **This project's API shape is, from today, constrained by a REAL
> CALLER rather than only by its own judgement; and `pdfce`'s three hard
> gates — `wasm32-unknown-unknown` (CI-enforced there), no copyleft
> anywhere in a dependency tree, and no network client in
> `pdfce-core`/`pdfce-render` — become INPUTS to rule 9's classification
> of every future dependency, not merely observations about a sibling.**

#### What the channel is, and the one property that forces this entry

The folder is **in no git repository, deliberately** — transient
coordination, not a record. Its binding consequence: **nothing may exist
only there.** A reply is a pointer plus an executive summary; the
durable finding lands in this project's own `docs/` in git. **This entry
is that landing** for the registration itself.

**Requests flow BOTH ways** — unlike the GUI channel `pdfce` also runs,
where `pdfceGUI` asks and `pdfce` answers. `iccce` may write
`open/request_<topic>.md`. ★ **A consumer's real usage is the best
available check on this library's API shape, and that check only works
if this side asks. Not asking is the failure mode, not a courtesy.**

#### The constraint is SATISFIED TODAY and is NOT GATED HERE

| Gate | State at tip `e21154c` | Class |
|---|---|---|
| `wasm32-unknown-unknown` builds | **exit 0**, four `.rlib`s emitted for `iccce-cmm`, `iccce-profile`, `iccce-color`, `iccce-measure` | **dated observation** |
| No copyleft in the tree | `Cargo.lock` holds **5** `[[package]]` entries and **all five are the workspace crates** — zero third-party dependencies | **dated observation** |
| No network client | nothing to classify; the only non-test I/O in the workspace is `iccce-cli`, and every `std::fs` call in a library crate sits inside `#[cfg(test)]` | **dated observation** |

*(All three **verified by the filing agent, which ran them** — see the
`SESSION_LOG.md` entry of the same date for the runner and the
inventory.)*

★★ **This was the FIRST time any iccce code was built for wasm32**; the
target was not previously installed on this machine. **It is NOT gated
in this project's CI.** So the honest statement is *"it built once,
today, at one tip, uninstrumented"* — **not** *"wasm32 is supported"*.

> **★ The consequence that makes this a decision rather than a note:
> adding a dependency here can break a CONSUMER'S CI gate silently.**
> Nothing in this repository would go red. Adding
> `wasm32-unknown-unknown` to `.github/workflows/ci.yml` is the obvious
> remedy and **is not done** — `.github/` is not the librarian's to
> edit, and it is recorded as owed.

#### ★★★ The adoption finding, and the direction it must never be written in

`pdfce` today converts `DeviceCMYK` through a **6×6×6×6 baked lookup
table fitted to *pdfium* output** (`crates/pdfce-core/src/color/cmyk_table.rs`)
— obtained by rendering patches and measuring them. That is a
**cross-check against another implementation, not ground truth**: this
project's own **rule 3**, applied to a sibling's existing work.

**iccce's oracle for a CMYK LUT path is lcms2** — also a cross-check.
And per **DL-041** the ground-truth gap for the LUT path is
**structural**: ICC.1 mandates no interpolation method, so no published
value could exist even in principle, corroborated because **iccDEV,
ICC's own reference implementation, ships zero expected colour values**.

> **★★★ Therefore replacing pdfium's table with iccce is a LATERAL move
> in evidence class. The defensible case for adoption is CONFORMANCE —
> ISO 15930 / ISO 32000-1 §14.11.5, the document's declared output
> intent being honoured AT ALL — and NOT accuracy. Do not let a later
> restatement invert this; the inversion is the specific error this
> entry exists to prevent.**

#### ★ And the first thing the consumer asked for is the thing this project cannot supply with independent provenance

`pdfce` renders to an sRGB raster and asked whether iccce constructs
**sRGB internally** or demands a caller-supplied destination profile.
**Today it is the latter, by omission rather than by decision** —
`Chain::new` (`crates/iccce-cmm/src/transform.rs:246`) requires two
parsed `&Profile`s, and every sRGB reference in the tree is inside a
`#[cfg(test)]` block reading a file off local disk *(verified — grepped
and read)*.

**The blocker is not effort.** `NUMERIC_CLAIMS.md:623` records the D65
chromaticity as **single-source from lcms2 `cmsvirt.c` alone**, because
**IEC 61966-2-1 is paywalled and was not obtained** — the row's own
header calls it *"the weakest constant in the crate"*. `:976` records
that *"the shared-misreading risk is ELEVATED here, not merely
present … the corpus against which any future ground-truth check would
be built shares an origin with the oracle."* ★ **A computed sRGB
destination built today would take its white point from the very
implementation iccce cross-checks against, and would then sit underneath
every `ICCBased` conversion `pdfce` made.** Obtaining the document is an
**operator act**, not an agent dispatch; the cheap *independent* lever is
**ITU-R BT.709, free from ITU and never fetched**.

**This is propagation, not a new finding** — the rows have said it for
some time and it simply had not reached the place that needed it. **No
new ledger row was created and `NUMERIC_CLAIMS.md` was not touched.**

#### What this does NOT decide

- **It does not scope any Pass.** No `ROADMAP.md` entry was created; the
  four gaps found (no computed sRGB destination; no `f32`/`u8`
  evaluation surface; `ChainError` implements `Display`
  (`transform.rs:134-169`) but **not** `std::error::Error`, so it is the
  `Box<dyn Error>` path and not the printable-message path that is
  blocked; no public signature→component-count helper) are **recorded as
  found and deliberately not scoped.** *(This list said `ChainError`
  implemented "neither" on the day of filing; corrected the same day —
  see the `SESSION_LOG.md` entry of 2026-08-17.)*
- **It does not commit `pdfce` to adopting iccce**, nor iccce to
  supplying anything on a schedule. It records that a consumer exists
  and what constrains it.
- **It does not answer either design question in the channel.**
  `reply_capability_status.md` answered *what exists* and declined both
  by name.
- **It does not make wasm32 a standing guarantee** — see the boxed
  consequence above.
- **It does not close an exchange.** `INDEX.md` holds **no rows**; a row
  is added when an exchange closes and none has.

**Revisit if:** any third-party dependency is proposed (classify against
all three consumer gates **before** rule 9's own MIT test, and re-run
the wasm32 build in the same breath); or `wasm32-unknown-unknown` is
added to this project's CI (at which point the "dated observation"
wording above is superseded and must be replaced rather than left
standing); or `IEC 61966-2-1` or **ITU-R BT.709** is obtained, which
converts the sRGB gap from a provenance problem into an engineering one;
or the channel's first exchange closes, at which point `INDEX.md` gains
its first row and the durable answer must be named in this project's
`docs/`.

---

### DL-045 — ★★★ **COMPATIBILITY and CERTIFICATION are different things, and this project has been parking work on the second while the first was reachable all along. The posture changes; the EVIDENCE RULES DO NOT**

**Date:** 2026-08-17 · **Occasioned by:** the operator, handing over the
**Ghent PDF Output Suite 5.0** · **Assessed by:** `icc-engineer` ·
**Filed by:** `icc-librarian` (**no shell** — every figure carried,
every document claim read from source) · **Relates to** project **rule 1**
(a wrong colour looks like a right one), **rule 3** (expected values come
from the literature), **rule 5** (tolerances are justified, not tuned),
**rule 9** (publishing is the operator's act), **DL-020** (refuse what
the corpus cannot supply), **DL-036** (a real vendor profile earns its
place), **DL-041** (three kinds of blocker, kept apart), **DL-042**
(re-audit the *reason* an item is owed), **DL-044** (the named external
consumer), and **DL-046 / DL-047** filed alongside it

#### The instruction, verbatim

> *"I know some things you stopped work on because they required physical
> testing that we don't have. We aren't going to aim for compliance like
> that. Just aim for compatibility."* — the operator, 2026-08-17.

#### The decision

> **This project does not pursue conformance CERTIFICATION, and it does
> pursue demonstrable COMPATIBILITY. The two are distinct claims with
> distinct evidence, and an item may not be parked on the grounds that
> certification is unreachable when the capability underneath it is
> measurable without an instrument.**
>
> ★★★ **This changes WHAT IS CLAIMED. It does not change how well a
> claim must be supported.** Rules 1, 3, 4 and 5 are untouched. Nothing
> in this entry licenses a looser tolerance, an unlabelled evidence
> class, or a plausible-looking result standing in for a measured one.

| | what it means | available here? |
|---|---|---|
| **compliance / certification** | a conformance body assesses a *workflow* against published criteria, generally by printing and judging or measuring the result | **No.** It needs a press or proofing device, and it is sold to print service providers by third parties. The operator has ruled it out |
| **compatibility** | the software accepts what a real-world corpus contains, and behaves as that corpus's design says a correct engine behaves, **in the parts checkable without an instrument** | **Yes, in a bounded subset** — and the subset is larger than it looked. `NUMERIC_CLAIMS.md` **§3.30** measures it |

#### ★★ The failure mode this corrects, named precisely

The habit was **not** cowardice about hard measurements. It was a
**category error**: *"this cannot be certified"* was being read as
*"this cannot be checked"*, and the item was then filed under the same
heading as a genuine blocker. **DL-041 already drew the distinction that
was needed** — existence, availability, and access terms are three
different blockers — and this entry adds the fourth kind that DL-041 did
not have a name for: **an item blocked on an ORGANISATIONAL fact about a
certification programme, which is not a blocker on the engineering at
all.**

★ **The correction was worth making because the parked work turned out
to be cheap.** `NUMERIC_CLAIMS.md` §3.30's strongest rows (NC-194,
NC-195) need **no oracle, no instrument, no published dataset and no
clause transcription** — they read their expectation off a fixture's
declared content. That evidence was reachable on any day of this
project's history and nobody reached for it, because the corpus it lives
in was filed under *"needs a press."*

#### ★★★ The one thing this entry must not be read as licensing

**It does not license the word "compatible" in public.** The Ghent
suite's licence permits use *"for … testing workflow setup"* and
**nothing else without written permission**; the certification release
directs developers to a separate programme reachable only by contacting
GWG. **Any public statement containing the word "Ghent" — `README.md`,
release notes, crates.io metadata — is a claim requiring GWG's written
permission first, and is an operator decision** (global claim-bearing-
copy rule; `NUMERIC_CLAIMS.md` §7.16, operator item 1).

★ **The internal statement that IS supportable today**, in full:
*iccce evaluates the colour transforms of the Ghent Level-2 ICC-CMS
patches' embedded profiles without failure, and applies a declared
source profile rather than falling back to a device alternate; it is not
Ghent-certified; and Ghent supplies no numeric criterion by which
"without failure" could be tightened into a colorimetric claim.*

#### What this does NOT decide

- **It does not scope a Pass.** `ROADMAP.md` gains a **standing
  workstream** block, not a numbered Pass, and no done-when is asserted.
- **It does not admit the suite into this repository.** The bytes stay in
  `D:\Dev\iccce-private-fixtures\ghent-v50\`; the licence's affirmative
  notice condition and its no-commercial-use clause would both follow
  them into an MIT tree.
- **It does not weaken DL-020.** Refusing what the corpus cannot supply
  remains correct. What changes is the *test* for "cannot supply":
  **ask whether an instrument is genuinely required, or whether only a
  certificate is.**
- **It does not make anything in §3.30 an accuracy claim.** See DL-047.

**Revisit if:** the operator obtains GWG's written permission (at which
point a public claim becomes possible and the exact wording becomes a
claim-bearing-copy matter, not a technical one); or a second
compatibility corpus with different terms arrives, which would argue for
generalising this entry beyond Ghent; or any future filing is found to
have used *"compatibility"* as a reason to relax a tolerance — **that
would be this entry being misread, and the misreading is the specific
risk it was written to prevent.**

---

### DL-046 — ★★★ **VERIFY IN THE RUNNING THING, NOT IN THE REPORT — and add the CONTROL the report did not have. A `[REPORTED]` claim is promoted by RE-DERIVATION, never by being repeated confidently**

**Date:** 2026-08-17 · **Occasioned by:** the CMYK trap-profile swap,
which arrived from a dispatched agent's byte-level scan and was
**deliberately re-derived through the shipped `iccce transform` binary,
with a control profile added**, before it was allowed to become a
finding · **Assessed by:** `icc-engineer`, who asked for the
generalisation to be recorded · **Filed by:** `icc-librarian` ·
**Relates to** project **rule 1**, **rule 3**, **DL-025** (a control is
only as good as its fixture), **DL-031** (a count is not an inventory),
**DL-033** (candidate separation is where a check's power lives),
**DL-036**, and `NEXT_SESSION.md` **§5.3** (an absence-claim is only as
strong as the breadth of the search that failed to find it)

#### The decision

> **A claim produced by reading bytes, parsing a file, or summarising a
> document is `[REPORTED]`. It becomes a finding only when the claim is
> re-derived by RUNNING THE SHIPPED CODE on the case — and where the
> claim is of the form "X changed", the re-derivation MUST include a
> control that isolates what did NOT change.**
>
> ★★★ **The control is not diligence. It is what makes the claim
> different from a weaker one.** Without it, *"the values changed"* is
> equally consistent with the transform mangling everything. With it, the
> claim becomes *"exactly two channels exchanged, and the others moved by
> at most `2.455×10⁻³`"*, and the mangling hypothesis is eliminated
> rather than assumed away.

#### The worked instance

A dispatched agent reported that GWG's `CMYK prtr lut X (Switch magenta
cyan)` has its cyan and magenta channels exchanged. That report was
**true** — and filing it would still have been wrong, because the
project would have recorded a *property of a file* as though it were a
*behaviour of iccce*.

What was done instead: the profile **and** the genuine
`ISO Coated v2 300% (ECI)` embedded in the same patch were both driven
through `iccce transform` at the same four inputs. The swap appeared on
the trap and not on the control; the unswapped channels agreed
(paper exactly to six decimals, yellow to `2.455×10⁻³`).
`NUMERIC_CLAIMS.md` **NC-195** and **NC-196**.

★★ **And the same session shows the other half of the rule holding.**
Six further byte-scan leads — per-patch rendering intents, a
`FOGRA27` / `ISO Coated v2 300% (ECI)` output-intent mismatch, a PDF
`/Lab` white point — were **NOT re-derived**, and are filed as
`[REPORTED]` leads with re-derivation recorded as owed
(§3.30.6, §7.16 newly-owed 2). **The rule is only visible as a rule
because both outcomes occurred on the same day**: one claim was promoted
and six were not.

#### ★★ Why this is not already covered by "verify against live source"

The librarian's standing rule says *verify the dispatch against live
source*. **This entry is the same principle one layer down, and the
layer matters.** Reading the source of a profile — or of the code — is
still *reading*. A wiring defect between `parse → model → transform`
is invisible to every amount of reading and appears only when the
binary runs. §3.29.1 already recorded exactly this shape from the other
direction: a clause was **defended as a function on a synthetic
closure** while *"the clause exercised THROUGH A PARSED PROFILE"* had no
detector at all.

> ★ **The diagnostic question, phrased for reuse:** *did anything
> execute, and was there a second case that should NOT have moved?* If
> the answer to either half is no, the claim is `[REPORTED]` and is
> labelled so.

#### What this does NOT decide

- **It does not devalue `[REPORTED]` claims.** They are how leads
  arrive. It requires only that they be **labelled**, and that the label
  survive being passed to a consumer.
- **It does not require a control on every row.** A control is required
  where the claim's form is *"X changed"*. A categorical row whose
  candidates are a gamut apart (NC-194) is already discriminating.
- **It does not make a control sufficient.** **DL-025 stands**: a
  control whose fixture is degenerate nullifies itself — an identity
  fixture once returned `1.1e-15` and proved nothing.

**Revisit if:** a `[REPORTED]` lead is ever found to have been promoted
without re-derivation (record which, and what it cost); or a control is
found to have agreed with its subject **because both were broken the
same way**, which is DL-025's failure mode arriving in this rule's
territory and would need the diagnostic sharpened.

---

### DL-047 — ★★★ **The Ghent PDF Output Suite CANNOT SUPPLY A NUMERIC EXPECTATION. It supplies a CATEGORICAL one, which is worth more than it sounds — and it CONTRADICTS ITSELF exactly on rendering intent**

**Date:** 2026-08-17 · **Occasioned by:** reading the suite's own
`ReadMeFirst.pdf` and per-patch readmes for a pass criterion, and
finding that none exists · **Assessed by:** `icc-engineer` (`pdftotext
-layout` extraction) · **Filed by:** `icc-librarian`, which **has not
seen the PDFs** and carries the quotations from
`docs/GHENT_COMPATIBILITY.md` §2.1 and §2.3 · **Relates to** project
**rule 3** (expected values come from the literature), **rule 5**,
**rule 7** (disagreement with an implementation is a finding, not a
failure), **DL-021** (say which direction, and which profile class),
**DL-033**, **DL-041**, and **DL-045**

#### The finding, in the suite's own words

> *"For some patches a faint X may appear due to rounding of values or
> different color management engines beeing used. This is not a failure
> of the tested features: A clear X indicates a failure of this feature.
> A faint X is NOT a failure of this feature."*

with assessment *"at a viewing distance of 0.5 m / 20 inches"*. Three
further readmes pre-excuse CMM-to-CMM disagreement outright — the
`ICCBased` blend-mode readme says *"A faint X is due to differences in
the CMM and does not indicate a failure"*, and the shadings readme
concedes its own reference image *"is a screenshot … the colors and
gradation can be slightly off"*.

#### The decision

> **No number measured on a Ghent file may be recorded as ground truth,
> a reference value, or a justified tolerance. The suite states no
> tolerance, no reference measurement and no expected colour value
> anywhere; its pass criterion is VISUAL SALIENCE assessed by a human at
> half a metre. Every figure measured on this corpus inherits the
> evidence class of the APPARATUS that produced it, and Ghent adds
> nothing to that class.**
>
> ★★ **The converse is the useful half: Ghent CAN supply a CATEGORICAL
> expectation**, because the trap profiles' correct handling is a fact
> about the file's declared content rather than a measurement of
> anything. That is why `NUMERIC_CLAIMS.md` §1 gains
> **`fixture-declared-categorical`**, and why NC-194/NC-195 are the
> section's strongest rows despite containing no tolerance.

#### ★★★ The suite contradicts itself, and precisely on this project's topic

GWG 13.0 — the suite's **only** statement about rendering intent —
signals *"the defined rendering intent 'Perceptual' was not respected"*
with a **faint green X**. `ReadMeFirst.pdf` has already ruled that **a
faint X is not a failure.**

> **Consequence: intent handling is UNTESTED by the suite's own pass
> criterion.** A workflow can be "Ghent-clean" while ignoring the
> declared rendering intent entirely. ★★★ **Therefore any iccce claim
> about intent selection must be graded against ICC.1's own text on
> which `A2Bx`/`B2Ax` table an intent selects — never against Ghent**,
> which has no criterion to offer.

★ **This is rule 7's shape with a corpus in the defendant's chair
instead of an implementation.** The suite is not wrong; it is a smoke
test with a very well-designed smoke detector, and it does not claim to
be a colorimetric criterion. **Recording that is what stops a future
session from grading a tolerance against it.**

#### What this does NOT decide

- **It does not devalue the corpus.** The profiles inside it are real
  third-party output and are the reason `NUMERIC_CLAIMS.md` §3.30 exists
  at all.
- **It does not say the suite is unusable as a target.** Rendering
  `ALL_X4` and comparing against the shipped `ALL_REFERENCE` remains the
  largest available win — and §3.30.2 shows the trap patches make that
  judgement **mechanical** rather than visual, because the artwork is
  authored pre-swapped and a correct conversion makes the X vanish into
  its surround. **That is a joint exercise with `pdfce` and an operator
  decision to scope.**
- **It does not close the intent question.** It states where the
  criterion must come from, and **no such row exists today**.

**Revisit if:** GWG publishes numeric criteria or a reference
measurement (which would change the first half entirely); or a render-
and-compare harness lands, at which point the pass criterion becomes
*this* project's to define and DL-045's warning applies — **defining our
own criterion is exactly the situation rule 5 was written for.**

---

### DL-048 — ★★★ **A stale claim-bearing CITATION is worse than a stale claim-bearing NUMBER, and in one specific direction: a wrong number invites RE-DERIVATION, a wrong pointer invites the reader to ACCEPT WHATEVER IS AT THE DESTINATION.** Cite the ledger by §-number and NC-number; never by line

**Date:** 2026-08-17 · **Occasioned by:** a spot-check of the six
`NUMERIC_CLAIMS.md` line-number citations in `docs/NEXT_SESSION.md`,
undertaken while correcting one of them · **Measured by:**
`icc-librarian` (reading each cited location at tip `e21154c`) and
independently by `icc-engineer` (*"[VERIFIED — I read all six lines
myself at tip `e21154c` and confirm every one carries unrelated content;
`:5788` is a blank line]"*) · **Filed by:** `icc-librarian` ·
**Sits beside DL-034**, which is its sibling and covers the *number*
case · **Relates to** **DL-031** (a count needs its apparatus — this is
the same disease in pointer form), **DL-044** (the outbound channel that
carried it), and project **rule 3**

#### The finding

**DL-034 says a claim-bearing number must be computed, not typed, because
a typed number goes stale silently.** This entry is the **pointer**
case, and it fails **in a worse direction**:

> **A wrong NUMBER invites re-derivation.** The reader who doubts it goes
> and recomputes, and the doubt is productive.
>
> **A wrong POINTER invites the reader to ACCEPT WHATEVER IS AT THE
> DESTINATION.** Arriving somewhere plausible *reads as confirmation*.
> The citation does not fail — **it authenticates the wrong text.**

#### The instance, and it is structural rather than a slip

`NEXT_SESSION.md` §3.0 cited iccce's **33-node recommended grid** as
`NUMERIC_CLAIMS.md:2164, :2529`. `:2529` described
**`USWebCoatedSWOP.icc`'s own `lut8` CLUT, which also has 33 nodes — a
DIFFERENT 33**: a vendor file's tag rather than iccce's recommendation.
**A reader at `pdfce` following it would have concluded the
recommendation is a property of somebody else's profile.** The line had
been **copied unchecked out of the handoff document every session is
instructed to read first, into an outbound cross-project request**, so the
carrier was the one artifact guaranteed to be re-read.

**Six of six spot-checked line citations did not carry the claim cited to
them** (`:2164`, `:2529`, `:5788`, `:623`, `:976`, `:6488`). ★★ **The
correction filed for the first two, at `NUMERIC_CLAIMS.md` §3.30.7, went
stale the same way inside one filing** — and the decay was then *observed
happening*: §3.30.7 records `:2164` as §3.13's Pass 6 coverage box
(`icc-librarian`, before that filing's own edits), while the corrected
outbound request describes the same line as *"unrelated (BPC material)"*
(`icc-engineer`, later the same day). **Two readers, two moments, one line
number, two destinations, and neither reading is wrong.**

**The mechanism is arithmetic, not carelessness: a ledger that grows by
INSERTION renumbers every line below the insertion**, so a line citation
into it decays on the next filing, silently, **everywhere it was copied**.

#### The decision

> **1. `NUMERIC_CLAIMS.md` is cited by §-number and NC-number** — stable
> identifiers that **move with their content**. **Never by line.**
>
> **2. A `path:line` citation into a SOURCE FILE is acceptable** — the
> same sweep found `lib.rs:80`, `compiled.rs:171`/`:180`,
> `transform.rs:246` and `diag.rs:83` **all exact, 4 of 4**. Line drift is
> a property of the **append-heavy ledger**, not of citation as such.
>
> **3. But a source citation gives the FULL PATH FROM THE REPO ROOT.**
> The bare `diag.rs:83` was right about line *and* content and named
> neither the crate nor `src/` — and **this workspace contains two
> distinct `ParseError` types** (`crates/iccce-measure/src/lib.rs:221`
> and `crates/iccce-profile/src/diag.rs:83`), **both of which implement
> `std::error::Error`**, which is exactly the ambiguity a bare filename
> invites.
>
> **4. A §-number with NO DOCUMENT NAMED is the same failure without even
> a line number to blame.** §3.0's bare *"§4.4"* pointed at nothing that
> carries the constant; the **two** §4.4 sections that exist in `docs/`
> are `LEGAL.md` §4.4 (*"What is not claimed"*) and
> `GHENT_COMPATIBILITY.md` §4.4 (*"ICC v4 evaluation on a
> vendor-authored profile"*) — **both plausible enough to be read as
> confirmation.**
>
> **5. When filing, REWRITE any `NUMERIC_CLAIMS.md:NNNN` you encounter
> into §/NC form rather than passing it along**, and treat a
> line-numbered ledger citation in any dispatch as **unverified**
> regardless of how confident the dispatcher sounds.

#### What this does NOT decide

- **It does not say the citations were wrong when written.** Several
  almost certainly were right; that is the point — **they became wrong
  without anybody touching them.**
- **It does not extend to source files.** Rule 2 above is deliberate:
  banning `path:line` everywhere would cost real precision to fix a
  problem that has not occurred there.
- **It does not fix the outbound channel retrospectively.** The channel is
  in no git repository (DL-044); the census request was corrected by its
  author *(verified — read)*, and the durable homes live in the ledger.

**Revisit if:** the ledger ever acquires stable anchors that survive
insertion (an emitted index, or per-row identifiers used as link
targets), at which point a mechanical citation form becomes available and
this entry's rule 1 can be enforced rather than remembered.

---

### DL-049 — ★★ **A disclosure field that GATES NOTHING caught a defect in a TOLERANCE'S JUSTIFICATION — on a row that was green. The separation mechanism's second-order value is that it makes a fixture state a claim its `why` string has to survive**

**Date:** 2026-08-17 · **Occasioned by:** `BLIND` firing on a Pass G
authoring row, and the fix exposing a second, invisible fault ·
**Measured by:** `icc-conformance` (`TOLERANCES.md` §3.7.4) · **Filed
by:** `icc-librarian` *(carried; this librarian ran nothing and
transcribed from `TOLERANCES.md` §3.7.4, read)* · **Extends DL-037**
(separation is disclosure, not enforcement) and **DL-039** (a rival
tolerance is not a rival candidate) · **Relates to** **DL-043**,
**DL-033**, and `NUMERIC_CLAIMS.md` **§5.2**

#### The finding

**DL-037 established that candidate separation is DISCLOSURE and
deliberately does not gate** — `BLIND` prints, it does not fail a row.
Pass G shows that a field with no enforcement power can still catch a
defect **no gate could**, because of *where* it looks:

1. **The visible fault.** On a profile whose `wtpt` **agrees** with its
   own colorant sum there is **no rival reading**, so a stated
   "separation" of `5.4×10⁻⁶` against a `2×10⁻⁴` bound was a
   **manufactured alternative**. `BLIND` said so. That row became
   `NO-NAMED-ALTERNATIVE` **with its reason** and is now the section's
   negative control.
2. **★★★ The invisible fault, which only surfaced once the first was
   fixed.** The `2×10⁻⁴` **encoding-floor justification did not hold for
   the profiles it was gating**: Ghent's sRGB colorants sum to the PCS
   white only to `1.885×10⁻⁴` — **≈12 `s15Fixed16` lsb** — because the
   *published* sRGB primaries do not sum to D50 to the encoding lsb.
   **The row had been PASSING, inside a bound its own `why` could not
   support.**

#### The decision

> **A candidate separation is not only a statement about POWER (DL-033).
> It is a statement about the FIXTURE that the row's `why` string must
> survive. Where the two disagree, the `why` is the thing that is wrong
> — and a green row is no evidence at all that it is not.**
>
> **Practically:** when a separation reads as *manufactured*, absent, or
> implausibly small, **re-derive the tolerance's justification before
> touching the separation.** The separation is the symptom; the
> derivation is where the defect lives.
>
> **And the remedy takes the DL-039 shape:** replace the unsupportable
> bound with a question that has **no free parameter** — here,
> *is the colorant sum nearer the normative PCS white or nearer the
> profile's own encoded `wtpt`?*, bounded at **half the distance to the
> profile's own rival candidate**. A classification cannot be tuned.

★ **A constraint that came with it, and it is rule 1 in a sourcing
register:** the replacement **imports no third white point** — **D65 in
particular**, whose constant this project records as **the weakest in
`iccce-color`** and **single-sourced from the oracle's own `cmsvirt.c`**
(`NUMERIC_CLAIMS.md` §3.5 / NC-018). **A derivation that had reached for
D65 would have put the oracle's own unverified constant underneath a
finding about third-party authorship** — and the finding would still have
looked exactly the same.

#### Why this is a separate entry and not a footnote to DL-037

**DL-037 is about the guard ORDER and about disclosure not being
enforcement.** This is about **what the disclosure is evidence OF**.
DL-037 predicts that `BLIND` will identify rows whose *comparison* has
unknown power; **it did not predict that `BLIND` would identify rows
whose TOLERANCE is unsupported**, which is a different document, a
different agent's responsibility, and — until this instance — something
only a person re-reading a `why` string could find.

#### What this does NOT decide

- **It does not make separation a gate.** DL-037's guard order and the
  deliberate non-gating of `BLIND` are unchanged.
- **It does not say every `BLIND` row hides a bad tolerance.** One
  instance; the rule is *look there first*, not *assume*.
- **It does not license widening.** In this instance the bound was
  **replaced by a differently-shaped question**, not relaxed — and the
  other Pass G tolerance corrected the same day
  (`NUMERIC_CLAIMS.md` §3.31.3) was fixed by **finding a missing term**,
  which is DL-043's rule and remains the first thing to try.

**Revisit if:** a `BLIND` or `NO-NAMED-ALTERNATIVE` row is ever found
whose `why` string is sound — which would show the association is weaker
than one instance suggests — or if the separation field acquires gating
power, which would change what it can be trusted to disclose.

---

### DL-050 — ★★★ **the built-in sRGB destination is selected by a TWO-VARIANT ENUM, not by `Option<&Profile>`, because an `Option` being `None` cannot distinguish *"there was none"* from *"I failed to get one"* — and only the second must never trigger a fallback**

**Date:** 2026-08-17. **Status:** decided and implemented (uncommitted).
**Filed by:** `icc-librarian` from an `icc-engineer` dispatch.
**Evidence:** `crates/iccce-cmm/src/transform.rs:122-179` *(read at the
tip by this librarian)*; `docs/DEFAULT_DESTINATION.md`'s
**STATUS: BUILT** block; `NUMERIC_CLAIMS.md` §3.32.14.

#### The decision

The operator decided on 2026-08-17: *"if the caller supplied destination
doesn't exist then it should fallback to constructing sRGB internally."*
**The API through which that happens is:**

```
pub enum Destination<'a> {
    Profile(&'a Profile),   // used. Always.
    None,                   // the caller LOOKED and there is none.
}
```

**Not** `Option<&Profile>`, and the difference is not stylistic.

#### Why — and this is rule 1 wearing a type signature

*A wrong colour looks exactly like a right one.* The fallback's failure
mode is the founding hazard **caused** rather than caught: if a
destination **failed to parse**, or **was not looked for**, or **was
found and then dropped**, and iccce quietly substitutes sRGB, the result
is a plausible-looking image, rendered to the wrong destination, with the
document's own declared print condition silently discarded, **and no
error anywhere.**

**`None` in an `Option` carries no information about which of those
happened.** The caller is the only party that knows whether it *looked* —
so the API makes the caller **say**, and `Destination::None` is spelled
out rather than inferred from an absence.

★★ **A declared destination that could not be parsed never reaches this
type.** It stays a **named refusal** and the caller propagates the parse
error. That is rule 6 (*the parser reports; it does not repair*) surviving
one layer up, where it would otherwise have been quietly discarded.

#### The second half of the decision: the fallback is DISCLOSED

`DestinationProvenance` (`CallerSupplied` / `BuiltInSrgb`) is returned by
`Chain::destination_provenance()`, and its `note()` names the constants
used and says in terms *"This is NOT the document's declared output
intent"*.

> ★★★ **This is the difference between a default and a cover-up, and it
> costs one accessor to stay on the right side of it.** A renderer that
> cannot tell whether a page went to a document-declared destination or
> to iccce's default **cannot honestly report what it did** — and
> `pdfce`, the named consumer, is a preflight-adjacent tool where exactly
> that report is the product.

#### What this does NOT decide

- ★★ **It does not make the constructed sRGB accurate.** Its measurement
  (NC-221/NC-222) is in a **third and weaker evidence class**,
  `constructed-vs-reference-file` — **neither ground truth nor a
  cross-check against another implementation**
  (`NUMERIC_CLAIMS.md` §3.32.1).
- **`Chain::new` is unchanged and no caller moved.** The addition is
  purely additive to the public surface.
- **It says nothing about LUT destinations.** The construction is
  matrix/TRC only, measured in one direction (DL-021 binds).

**Revisit if:** a consumer is found that needs a third state (e.g.
*"a destination was declared, is unparseable, and I want the fallback
anyway with a recorded reason"*) — which would be a **new variant with
its own provenance value**, never a widening of `None`.

---

### DL-051 — ★★★ **a test suite that documents a constant at length while being unable to detect its corruption is WORSE than one that says nothing, because it reads as protection. The gap was found by INJECTION and could not have been found by INSPECTION**

**Date:** 2026-08-17. **Status:** adopted, with the measured instance.
**Filed by:** `icc-librarian`. **Evidence:** `NUMERIC_CLAIMS.md` §3.32.4
(NC-228, the five-injection matrix) and §3.32.3 (NC-224).

#### The instance

`crates/iccce-cmm/src/builtin.rs` shipped **a long, carefully sourced doc
comment on why the sRGB breakpoint is `0.04045` and not `0.03928`** — the
C⁰ versus C¹ solutions, the standards each comes from, the reason the
project chose one — and **five tests**. The constant was then substituted
and the suite re-run:

| injection | caught by |
|---|---|
| **A** Bradford adaptation omitted | 3 of 6 tests FAILED |
| **B** adaptation applied twice | 3 of 6 FAILED |
| **C** TRC replaced by pure gamma 2.2 | 1 FAILED |
| **★★★ D** breakpoint `0.04045` → `0.03928` | **NOTHING. 6 of 6 PASSED** |
| **E** green primary `0.600` → `0.610` | 2 FAILED |

#### The rule, and the mechanism that hides it

★★★ **The length of the documentation is what made the gap invisible.
Nobody audits a constant that is visibly well-explained.** A reader
scanning for weak spots sees the paragraph of sourcing and moves on — the
documentation performs the function of a test without being one, and it
performs it *convincingly in proportion to how well it is written.*

★★ **This is a SIBLING of `NEXT_SESSION.md` §5.3 (*a test that cannot
fail is not evidence*) and is not the same thing.** In §5.3's two
instances the tests could not fail **at all**. Here **every one of the
five tests could fail** — three of them do, under A, B and E — **just not
for this defect.**

> ★★★ **A suite's power is PER-DEFECT, not per-suite.** A green suite is
> a statement about the defects someone thought to inject, and *"the
> suite passes"* carries no information about a defect nobody enumerated.

#### The method half, which is the transferable part

★★★ **Inspection could not have found it.** The same person wrote the
constant, the doc comment and the tests **in one sitting**, and read them
back without seeing it. That is `NEXT_SESSION.md` §5.2's structure
appearing outside the oracle: **reading your own work is not an
instrument.**

**Two tests were then written specifically for D, and the injection now
goes red while its siblings stay green** — including
`no_eight_bit_code_lies_between_the_two_candidate_breakpoints`, which
**asserts NC-224's falsification of the corpus so it cannot go stale
again.**

#### ★★ The corroborating instance, from the same day and a different module

`crates/iccce-color/src/illuminant.rs`'s `D65_XY` doc comment read
**"SINGLE SOURCE — not cross-verified"** long after ITU-R BT.709-6 had
made that false. **The pointer was fine; the CHARACTERISATION OF THE
SOURCE'S STRENGTH had gone stale** — and it mattered, because while D65
rested on lcms2 alone, **anything built on it put the differential
oracle's own white point underneath every conversion iccce then checks
against that oracle.**

★★★ **So DL-048's class is not confined to `docs/`.** A doc comment
citing a source is a claim-bearing pointer with the same decay property
as a ledger citation — **and it decays with no line numbers involved at
all.** *(`NUMERIC_CLAIMS.md` §3.32.10b; DL-048 itself is not edited.)*

#### ★★★ And it recurred INSIDE the same session, in the module documenting the hazard most

`crates/iccce-profile/src/colour_space.rs`'s corruption test carries a
doc comment explaining the failure mode *"stated as a property rather
than a single case"* — and **the property is not tested**: the survivor
count reaches a `println!`, and the in-loop assertion compares
`components(sig).count()` with **its own result**, so it cannot fail.
**Found by this librarian reading the file at the tip, hours after this
entry's rule was adopted from a different module**
(`NUMERIC_CLAIMS.md` §3.32.8, §7.18 newly-owed 2).

**Revisit if:** an injection is ever found that a test suite catches
**only** because of a doc comment — which cannot happen, and is stated so
the entry's claim stays falsifiable in principle.

---

### DL-052 — ★★ **write the COMPILED DOC EXAMPLE before declaring a public API done: it is the cheapest available consumer, and it is the only reviewer who has not already learned the API's shape**

**Date:** 2026-08-17. **Status:** adopted, with the defect it found.
**Filed by:** `icc-librarian`. **Evidence:** `NUMERIC_CLAIMS.md` §3.32.11;
`crates/iccce-cmm/src/transform.rs:307`,
`crates/iccce-cmm/src/matrix_trc.rs:768` and `:780` *(read at the tip)*.

#### The instance

`ChainError` did **not** implement `std::error::Error` while `ParseError`
did. The asymmetry had been **noticed across sessions and carried as
cosmetic** — `NEXT_SESSION.md` §0 names it in a list of known gaps.

**It was not cosmetic.** Without the impl a consumer cannot write

```
let chain = Chain::with_destination(..)?;
```

into a `Result<_, Box<dyn Error>>` — **the most common error-handling
shape in Rust.** Callers had to hand-wrap or `.ok()` the result and
**lose the named refusal entirely.**

> ★★★ **A refusal that is awkward to propagate is a refusal that gets
> discarded** — and rule 6's entire value is that refusals reach the
> consumer. An ergonomics defect on an error type is a **correctness**
> defect one layer out.

`source()` now chains through `ModelError` to `CurveError` (both also
gained impls), so **the clause-level reason survives the boxing.**

#### How it was found, which is the rule

A `no_run` **doc example** was written for `Chain::with_destination`
doing the ordinary thing, and **the doctest refused to compile with
`E0277`.**

★★★ **A compiled doc example exercises the API as a STRANGER would, which
no amount of internal review does.** The author of an API is the one
person who cannot use it naively — they know which types compose, so they
never write the line that does not compile. The doctest does, and the
compiler is the reviewer.

★★ **It is also the cheapest consumer available.** The project's other
real consumer check is the `pdfce` request channel, which costs a
round trip measured in sessions. This one costs a paragraph and runs in
`cargo test`.

**Scope:** this is about **public** API. A doc example on a private item
proves nothing about consumability.

**Revisit if:** the workspace ever gains a genuine external integration
test that plays the stranger's role — which would make the doc example
the cheap check rather than the only one.

---

### DL-053 — ★★★ **a count taken from a SAMPLE, recorded as a count of the POPULATION. `[VERIFIED]` tagging does not protect against this, because the verification is real — what is missing is the DENOMINATOR**

**Date:** 2026-08-17. **Status:** adopted, with the live instance and its
correction. **Filed by:** `icc-librarian`. **Evidence:**
`NUMERIC_CLAIMS.md` §3.32.10a (NC-219); the superseded text in
`docs/NEXT_SESSION.md`, corrected by this filing.

#### The instance

`NEXT_SESSION.md`'s resume-from-cold handoff stated:

> *"★ **Two of them correctly FAILED** [VERIFIED by me]:
> `sRGB_ISO22028.icc` and `sRGB_D65_colorimetric.icc` are **iccMAX**…"*

**Every word of that is true of what was run.** Two files were tested and
two files refused. **Ten are present** — `FluorescentNamedColor`,
`NamedColor`, `Lab-D50_2deg`, `SixChanCameraRef`, the four
`Spec400_10_700-*`, `sRGB_D65_colorimetric`, `sRGB_ISO22028` — measured
by reading header bytes 8..12 across all 50 files and then running
`iccce inspect` over all 50, reading `$?` per file.

#### Why this is not DL-031, and why it is worse in one specific way

**DL-031** says an unlabelled count is not a claim, because the
**apparatus** is half the number. **Here the apparatus WAS labelled** —
the claim carried `[VERIFIED by me]`, and the verification was genuine.

★★★ **What was missing is the sentence *"…of the two I tested."*** The
sample was never named, so the reader supplies the natural denominator,
which is the corpus. **A `[VERIFIED]` tag certifies that the measurement
happened; it certifies nothing about what the measurement RANGED OVER.**

★★ **And it landed in the worst possible document.** `NEXT_SESSION.md` is
the resume-from-cold handoff that **every session is instructed to read
first**, so the figure was positioned to be re-quoted rather than
re-derived — the same carrier mechanism DL-048 recorded for a stale
citation.

#### The rule

> **A count states its denominator, or it is not a count.** *"Two
> failed"* is a claim about a sample. *"Two of fifty failed"* is a claim
> about a population. **The first may never be written where the second
> will be read**, and a handoff document is always the second.

★ **The corollary for this librarian:** *"a count is not an inventory"*
already governs findings and tests. **This adds the mirror case** — a
count that IS an inventory of something, but of a **smaller set than the
reader will assume.**

#### ★ The correction, and what it turned out to be worth

**The full population is a better finding than the sample was:** 50 files,
**40 parse with `malformations: 0` and 10 are refused by name** with the
iccMAX signature and version in the message. That is **rule 6
demonstrated on real ICC-published files at population scale** rather
than on two of them (NC-219) — so re-deriving the denominator did not
merely correct a number, **it upgraded the claim.**

**Revisit if:** never, as a rule. **As a practice**, revisit the moment a
sampled measurement is cheap to run over its whole population — in this
instance the full sweep cost one loop and replaced a corrected figure
with a stronger one.

### DL-054 — ★★★ **the ABSENCE of a published value is not evidence that the artifact you hold IS the reference. An ACCESS boundary got recorded as an EXISTENCE fact, and the mis-attribution survived because THE NUMBER WAS CORRECT**

**Date:** 2026-08-17 (supplementary filing). **Status:** adopted, with the
live instance, its correction, and the repair still outstanding in the
standards corpus. **Filed by:** `icc-librarian`. **Evidence:**
`NUMERIC_CLAIMS.md` §3.33 (NC-230 … NC-233), §3.33.4, §3.33.5, §4's
NA-011; `crates/iccce-cmm/src/builtin.rs`'s rewritten rule-4 section;
`ICC_Spec/iec/iec__s__srgb.md` lines 58, 582, 655-669, 705.
**Supersedes no entry** — it *corrects an attribution*, and the entry
that carried the attribution was a ledger row, not a decision.

#### The instance

For the whole life of this project's sRGB work, reconstructing the
shipped HP 1998 profile's colorants from sourced chromaticities landed 8
of 9 cells within 2 ULP of `s15Fixed16` and missed `bXYZ.Z` by **~12
ULP**. Three routes were tried to close it; all failed; the residual was
registered as **iccce's** rule-4 approximation and a test asserted it
stayed in `11.0..13.0` ULP.

On 2026-08-17 the operator downloaded ICC's own **"How to interpret the
sRGB color space (specified in IEC 61966-2-1) for ICC profiles"** (Jack
Holm, ICC, 2015-04-27) in a browser. **§B.2 publishes the D50-adapted
colorants at 15 decimal places.** Measured against them:

| | worst cell | `bXYZ.Z` |
|---|---|---|
| **iccce's from-constants construction** | **3.02 ULP** | **0.90 ULP** |
| the shipped HP 1998 / `sRGB2014.icc` file | **11.13 ULP** | **11.13 ULP** |

**The residual belongs to the FILE.** iccce's construction is ~3.7×
closer to ICC's published values than the most widely deployed sRGB
profile in the world is.

#### Why it survived, and it is not "nobody checked"

★★★ **The number was right.** Every measurement of the residual was
correct; every route that failed to close it genuinely failed. **What was
wrong was the OWNER**, and no re-measurement could have found that,
because both candidate owners produce the same residual. **Only a new
document could settle it** — which is precisely why the item that would
have settled it must never be de-prioritised by reasoning.

#### The mechanism, in three parts

1. **An ACCESS boundary was recorded as an EXISTENCE fact.** The
   document sat behind `color.org`'s robot bar. *"No agent may fetch
   it"* became *"it is not fetched"* became *"nothing publishes these
   values."* ★★★ **DL-041 built the taxonomy that prevents this** —
   *existence*, *availability*, *access terms* are three different
   blockers — and **this is that taxonomy's first observed failure in
   practice.**
2. **Both registers existed IN ONE FILE, and the flat one propagated.**
   `iec__s__srgb.md`'s **acquisition list** said *"the one thing no
   document **found so far** states"* and named the barrier, who could
   pass it, and that not fetching was *"a reported tool/permission limit,
   not an untaken action"* — **exactly right.** Its **status table**, 100
   lines away, said ***"NO document states them. Only a real file
   does."*** ★★ **The status table is what got quoted** — into a doc
   comment, into `DEFAULT_DESTINATION.md`, into `NUMERIC_CLAIMS.md`
   §3.8 and §3.32.5. **Summaries propagate; that is what summaries are
   for**, and a summary that collapses a *search* claim into an
   *existence* claim launders the difference.
3. ★★★ **The expectation of an UNREAD document was revised DOWNWARD from
   evidence about a DIFFERENT artifact.** Hours before the fetch, the
   corpus recorded *"★ EXPECTATION LOWERED … `sRGB2014.icc` turns out to
   carry HP's 1998 colorants byte-for-byte, so the registry page most
   likely restates the same nine numbers … **no longer likely to close
   the colorant gap on its own**."* **The document closed the gap
   outright and reversed an attribution.** You cannot estimate the
   contents of an unread document from the files it describes, and the
   practical effect of trying was to lower the priority of the one action
   that would have settled the question.

#### Why this is NOT DL-041, and the distinction is the whole entry

**DL-041** established that published ground truth for the LUT path
**cannot exist** — ICC.1 mandates no interpolation method, so no expected
value could be published *even in principle*. **That absence is
STRUCTURAL.** Here the absence was merely **UNFETCHED**. ★★★ **The two
were treated the same way**, and the ledger's own language did not
distinguish them: *"no document publishes X"* was written for both.

> **The rule.** ★★★ **Write the SEARCH claim, never the EXISTENCE claim.**
> *"No document found so far states X, and here is what has been looked
> at and what is barred"* is falsifiable, dated, and invites the next
> fetch. ***"No document states X"*** is a claim about the literature
> that nobody in this project is in a position to make, and it converts
> the best artifact available into the reference **by elimination**.
>
> ★★ **Corollary, which is where the cost actually landed:** a gap in the
> literature is **not a licence to promote an implementation to ground
> truth.** If the only available reference is an artifact, the row is
> `measured_file_behaviour` or `constructed-vs-reference-file` and the
> residual is **unattributed** — it is a distance between two things, and
> the ledger says so, rather than assigning it to whichever of them is
> ours.

#### ★★ The same day supplied the counter-example, and it is not a contradiction

`icc-conformance`'s Pass H found that ICC's **`Probe2 Profile Readme`**
states, in numbers, what `Probev2_ICCv4.icc` does — and **the published
claim is FALSE of the file the document names** (`TOLERANCES.md` §3.8.2).
Those rows were relaxed to **REPORTED, tolerance infinity**.

★★★ **Two ICC documents, one day, opposite rulings, and the principle is
the same in both:** *a document that states the intended VALUES is a
definition and outranks any file* (sRGB §B.2 — we ruled for the
document); *a document that states what a particular FILE does is an
empirical claim about an artifact and can be falsified by that artifact*
(the Probe2 readme — we ruled against the document). **Ask what kind of
claim the document is making before deciding what it outranks.**

**Revisit if:** a second reading of §B.2 disagrees with the transcription
now in `builtin.rs`; or ICC publishes the §A.7 matrix at more than 7
decimals, which would collapse NA-011's 3.02 ULP into a transcription
question rather than a construction one.

### DL-055 — ★★★ **a fix with two layers where EACH LAYER ALONE makes the gate observe zero. The guard can be deleted with NO NUMBER MOVED and NO EDIT TO BLAME — the change ledger cannot record it, because there is nothing to record**

**Date:** 2026-08-17 (supplementary filing). **Status:** adopted; the
remedy is already in place as `icc-conformance`'s four rows.
**Filed by:** `icc-librarian`, from a finding that was
**`icc-conformance`'s, not the engineer's**. **Evidence:**
`NUMERIC_CLAIMS.md` §3.33 (NC-234, NC-235, NC-236) and §3.33.6;
`TOLERANCES.md` §3.8.4.3 — read at the tip, in `icc-conformance`'s own
words — and `crates/iccce-cmm/src/compiled.rs`.

#### The instance

`iccce bench` **aborted the process** on ICC's published seven-channel
`APTEC_CMYKOGV_Coated_LinearCTV_2025.icc`: bare exit `0xC0000409`,
stderr *"memory allocation of 1022842631448 bytes failed"*, stdout empty.
Two independent causes — a **`_ => 33` catch-all** (`33⁷ × 3 × 8 ≈ 952.6
GiB`), and a guard using **`checked_pow`, which catches WRAP and not
SIZE**. The fix has two layers: a **computed** ≥5-channel recommendation,
and a **`MAX_COMPILED_GRID_BYTES = 64 MiB`** budget with a named
`ChainError::GridExceedsBudget`.

> ★★★ **Each layer alone makes the original conformance row observe
> zero.** The computed recommendation puts the default at grid 6, so the
> allocation succeeds **whether or not the guard exists** — **deleting
> `MAX_COMPILED_GRID_BYTES` entirely would have left the row GREEN.**

#### Why this is worse than DL-018, which it otherwise resembles

**DL-018** is *a gate must not reward deletion*: someone removes a costly
thing and the gate gets **greener**, which a diff of tolerances can at
least show. ★★★ **Here nothing moves.** No tolerance changes, no
expectation changes, no test is deleted, no number is edited. **A second
change, elsewhere, quietly takes the first change out of the loop.** A
row that went red on a real defect becomes a row that **cannot see that
defect return**, and there is no artifact anywhere that records the
transition.

#### The rule

> **When a fix has more than one layer, ask of every row that observed
> the defect: *which layer is in the loop?* — not *what does this row
> measure?*** A row must exist that **fails if any single layer is
> removed**, or the layers are load-bearing only by coincidence.

**Applied here** (`icc-conformance`'s split, four rows, one per layer):
the default-grid survival row; a row tying the binary's behaviour to
`recommended_grid_points`; a row exercising **the guard itself through
the CLI**; and one **REPORTED** row that grades nothing and says so.

★★ **An in-process unit test is not the same layer as a CLI row.**
`compiled::tests::oversized_grid_arithmetic_is_refused_not_aborted`
asserts the guard's **arithmetic** deliberately without attempting the
allocation — right, because *a test that aborts the test process proves
nothing and takes its siblings with it* — but it is **blind to the
wiring**: whether `bench` propagates the `Err` as exit 1, whether the
message reaches stderr, whether anything escapes on stdout. **Both are
needed and neither substitutes.**

#### ★★ Two subsidiary decisions recorded as decided

1. **An abort is the worst available library failure and converting it to
   a named refusal is rule 6 at the allocation layer.** Not an `Err`, not
   a catchable panic — **it takes the consumer's process down**, and
   `pdfce` (DL-044, wasm32-gated) has no defence at all.
2. **The measured 4-channel grid `33` was NOT shrunk to fit the byte
   budget** (`33⁴ × 15 × 8 ≈ 136 MiB` at the worst output width). A
   measured value is not weakened to satisfy a memory bound; the guard
   uses the **actual** output width. ★ **A test fails if the tension ever
   disappears**, because the failure mode of a documented exception is
   silent removal with the explaining paragraph surviving.

**Revisit if:** a fix is ever proposed as *"belt and braces"* — that
phrase is the tell. Two mechanisms are only redundant if **each** has a
row that fails without it.

### DL-056 — ★★★ **a DIFFERENTIAL test is blind in the direction that moves your answer TOWARD the reference. The ABSOLUTE assertion covering the blind direction always looks redundant, and deleting it leaves every test green**

**Date:** 2026-08-17 (supplementary filing). **Status:** adopted, with the
injection table that proves it and DO-NOT-DELETE notes at both sites.
**Filed by:** `icc-librarian`. **Evidence:** `NUMERIC_CLAIMS.md` §3.33
(NC-240, NC-241) and §3.33.8; `crates/iccce-cmm/src/builtin.rs:489-519`;
`crates/iccce-cmm/tests/builtin_srgb_destination.rs:260-307`.
**Relationship to DL-033:** DL-033 is the **diagnosis** (*agreement with
the oracle was the symptom of our defect*); **this is the design rule
that follows from it.**

#### The instance

`icc-engineer` wrote that NC-221's run-time-derived ΔE bound was *"a very
tight white-point gate"*, **then injected drift to check it**:

| injected drift in the constructed white's `Z` | max ΔE2000 | the ΔE test |
|---|---|---|
| `−1.0×10⁻³` | `0.101968` | FAILS ✔ |
| `−3.0×10⁻⁴` | `0.050149` | FAILS ✔ |
| **`+3.0×10⁻⁴`** | **`0.029008`** | **PASSES — and looks BETTER than the correct build's `0.033013`** |
| `+2.0×10⁻³` | `0.146450` | FAILS ✔ |

**A defect in one direction makes the test greener.** The reference
file's own white sits **`+1.885×10⁻⁴` above D50** in `Z`, so drifting
upward moves iccce **toward the reference**; up to ≈`+3.8×10⁻⁴` the test
would report **zero**.

★★★ **It is not fixable by tightening: a difference cannot detect a
defect that shrinks it.**

#### ★★ It corrected the librarian's own reasoning, filed hours earlier

`NUMERIC_CLAIMS.md` §3.32.9a — this librarian's independent correction of
the dispatch — argued that the observed-≡-derived coincidence at the
binding probe made the row *"a tighter white-point gate than any flat
constant would have been."* **True downward, false upward.** ★★★ **A
careful argument produced from re-reading was corrected by an injection**,
which is §5.2 (*reading your own work is not an instrument*) applied to
the agent whose job is reading.

#### The rule

> **Every differential gate names the ABSOLUTE assertion that covers its
> blind direction, or declares that none exists.** An absolute assertion
> compares against a **sourced constant with no reference artifact in
> it**; a differential one compares against something real and is blind,
> by construction, in the direction of that something.

**Here:** `builtin::tests::constructed_colorant_sum_is_d50` compares the
constructed white against **`D50` itself to `1e-9`, with no file anywhere
in it**. The same `+3.0×10⁻⁴` injection **fails it while all six
differential tests pass**.

★★★ **Deleting it as "redundant — the ΔE test covers it" would open the
blind spot AND EVERY REMAINING TEST WOULD STAY GREEN WHILE IT HAPPENED.**
Both files therefore carry a **DO-NOT-DELETE note containing the
injection table** — the table is the argument; a bare *"do not delete"*
would be an assertion of the kind this project does not accept.

★★ **This is DL-055's mechanism in the differential-test register.** Two
mechanisms, only one with power against a given defect, and removing the
powerful one is invisible. **The shared sentence: a redundancy is only a
redundancy if each member has a defect it alone can catch — otherwise it
is a single mechanism with a decoy beside it.**

**Revisit if:** a reference artifact is ever replaced by one that sits
*below* D50 — the blind direction flips, and any prose describing which
way the gate is blind becomes wrong while every test stays green.

### DL-057 — ★★★ **a code path that reuses machinery "and discards part of the result" inherits every failure mode of the discarded part — because the discarded half CANNOT FAIL HARMLESSLY. And a refusal that names the WRONG CLAUSE is worse than a vague one, because the citation makes it persuasive**

**Date:** 2026-08-17 (supplementary filing). **Status:** adopted; fixed by
extraction, with two regression tests. **Filed by:** `icc-librarian`.
**Evidence:** `NUMERIC_CLAIMS.md` §3.33 (NC-237) and §3.33.7;
`crates/iccce-cmm/src/transform.rs:457-498` (`derive_source_model` and
its doc), shared with `new_inner` at `:609` and `:724`.

#### The instance

`Chain::with_destination(src, Destination::None, ..)` obtained the source
model by building a **scaffold chain `src → src`** and **discarding the
destination half**. That works whenever the source can also serve as a
destination — which every profile tested at the time could.

It fails for a profile with an `A2B` tag and **no `B2A`, no colorant
matrix, no `grayTRC`** — a conformant shape. **Four such profiles are in
ICC's own published set**: the colour-vision-deficiency simulation
profiles, `scnr` class, Lab PCS, one-directional by design.

#### ★★★ The symptom is the filing-worthy part

The refusal read:

```text
matrix/TRC model requires PCSXYZ (Annex F.3, normative); profile PCS is 'Lab '
```

**True. Correctly clause-cited. And about a model iccce was about to
throw away.** A caller reads it as *"my source profile is unusable"* —
which is false; the source was fine and the destination being complained
about was never going to be used.

> ★★★ **This is the project's founding hazard — rule 1, *a wrong colour
> looks exactly like a right one* — arriving in the ERROR SURFACE rather
> than in a colour value.** A vague refusal invites investigation. **A
> precise, clause-cited refusal invites acceptance.** The citation is
> what makes it dangerous, which inverts the usual instinct that more
> precise diagnostics are strictly better.
>
> ★★ It is also **DL-048's mechanism in a third register**: a pointer
> that is individually correct and points at the wrong thing, so arrival
> reads as confirmation.

#### The rule

> **A path that reuses machinery and discards part of the result inherits
> every failure mode of the part it discards.** The discarded half cannot
> fail harmlessly — **its error is what the caller sees**, wearing the
> caller's own request as its subject line.
>
> ★ **Corollary for rule 6:** *the parser reports and does not repair*
> gains a second half — **a report must be ABOUT THE THING THE CALLER
> ASKED ABOUT.** A truthful report about an internal scaffold is not a
> report to the caller; it is a leak.

#### ★ Why the fix is an extraction and not a special case

`derive_source_model()` is **shared** with `new_inner`, so the ICC.1
**8.10.2** fallback dispatch — the most intricate logic in the crate —
has one copy and cannot drift. A local special case would have removed
the instance and left the class.

★★ **And the second regression test is load-bearing.** Beside *"these
profiles now work as sources"* sits *"these same profiles are still
correctly REFUSED as destinations"*. Without it, an over-broad future fix
turns the first test green **for the wrong reason** — DL-020's discipline
(a fixture discharges the doubt it was authored for and nothing adjacent)
applied to a pair of tests.

**Revisit if:** any other path is found calling a constructor for a side
effect and dropping part of its result. The audit question is not *"is
this correct?"* but *"whose error message does the caller see if the
discarded half fails?"*
