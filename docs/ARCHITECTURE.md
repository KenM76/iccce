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
  tools/
    difftest/          out-of-tree differential oracle against lcms2.
    gen-profiles/      synthetic profile generator for fixtures.
  fixtures/
    synthetic/         profiles this project authored, byte by byte.
    reference/         rights-cleared real profiles (see LEGAL.md §3).
```

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
