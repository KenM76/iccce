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
