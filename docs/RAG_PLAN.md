# The standards RAG — what `icc-spec-librarian` builds

**Location: `D:\Dev\Rag-Specialized\ICC_Spec\`**, a sibling of the
existing `PDF_Spec`, `Acrobat_Features` and `Inkscape_Features` corpora.
Same conventions, same house style, so anyone who can search one can
search all of them.

This is a **private development reference**. It is never shipped and
never committed to any product repository.

---

## Why this corpus is worth building before the code

Colour is a field where the plausible answer and the correct answer look
identical until measured. An engineer writing a chromatic adaptation from
memory will produce a matrix that is *nearly* Bradford, and the output
will look fine, and it will be wrong by an amount nobody notices until a
customer's brand colour is off on press.

The `pdfce` project has already paid for this lesson in a cheaper
currency — its spec RAG caught a case where the standard's own *prose*
contradicted its *table*, and building from the sentence would have
silently dropped watermarks from printed output. Colour has more of those
per page than PDF does.

---

## Format

Identical to `PDF_Spec`: LLM-optimized, not human-readable. Dense,
schema-consistent, grep-first. No narrative, no restating context a
reader already has. **If a sentence does not add a fact a future lookup
needs, cut it.**

Each file carries frontmatter with `standard`, `clause`, `keywords`, an
`evidence` tier, and an `iccce_relevance` line saying what in this
project depends on it. One finding per file where practical; a file per
clause where the clause is the unit.

**Mark verbatim standard text as verbatim, and paraphrase as
paraphrase.** This is not pedantry — an implementation decision resting
on a corpus paraphrase is resting on the librarian's reading, and the
distinction has already mattered in the sibling project, where a
reconstructed section *heading* was cited as though it were spec text.

---

## What to ingest, in build order

Order is by what blocks code, not by importance.

### Tier 1 — nothing can be built without these

| Standard | Why it blocks |
|---|---|
| **ICC.1:2022** (and/or ICC.1:2010-12 v4.3) — the profile format | Every byte the parser reads. The whole header, every tag type, the PCS definitions. |
| **ISO 15076-1** | The ISO twin of ICC.1. Record where they differ, if they do. |
| **CIE 15 — Colorimetry** | XYZ, standard illuminants, standard observers. The definitions everything else assumes. |
| **ICC v2 vs v4 differences** | Not one document — a synthesis the librarian must assemble. The single richest source of wrong-but-plausible colour, especially Lab PCS encoding in `lut16Type`. **Give this its own file and treat it as a first-class deliverable.** |

### Tier 2 — needed for correct transforms

| Standard | Why |
|---|---|
| **Black point compensation** (Adobe's published algorithm; ICC white paper) | Not in ICC.1. An implementation detail everyone implements and the base standard does not define. |
| **CIEDE2000 (CIE 142 / ISO 11664-6)** | The error metric the test suite is graded in. Getting ΔE2000 wrong makes every other test unreliable. |
| **Chromatic adaptation — Bradford, von Kries, CAT02** | Which one, and where ICC mandates it versus leaves it open. |
| **IEC 61966-2-1 — sRGB** | The default everything falls back to; its transfer function has a linear segment people routinely omit. |

### Tier 3 — needed for real-world files

| Standard | Why |
|---|---|
| **ITU-R BT.709 / BT.2020 / BT.2100** | Display and video primaries; BT.2100 for HDR transfer functions. |
| **Adobe RGB (1998), Display P3, ProPhoto** | Profiles that show up constantly. |
| **ISO 12647 / ISO 12640 (SCID) / ISO 12642 (IT8.7)** | Print condition and characterisation targets. |
| **ISO 13655** | Measurement geometry and computation — how the numbers in a profile were obtained. |
| **ISO 3664** | Viewing conditions. Explains why "correct" colour still looks wrong under the wrong light. |

### Tier 4 — the consumer's side

| Standard | Why |
|---|---|
| **ISO 32000-1 §8.6.5.5** | How PDF embeds ICC. Already partly in `PDF_Spec` — **cross-reference, do not duplicate.** |
| **ISO 15930 (PDF/X) output intents** | The destination profile for a print job. |
| **PDF/A colour requirements** | Constrains what a conforming file may contain. |

---

## Deliverables beyond ingestion

The librarian is not only a transcriber. Three synthesised artifacts are
worth more than any single clause file:

1. **`icc__ref__v2_v4_divergence.md`** — every place the versions differ,
   with the symptom each difference produces when got wrong. This is the
   file the engineer will open most.
2. **`icc__ref__tag_coverage_matrix.md`** — every tag type, whether it is
   required/optional, which profile classes use it, and iccce's
   implementation status. The same dated-status-table discipline
   `PDF_Spec` arrived at, from the start rather than after three
   corrections.
3. **`icc__ref__ambiguity_register.md`** — where the standard is silent
   or self-contradictory, each with an id, so an implementation choice
   can cite one. Colour has many: interpolation method between LUT grid
   points, out-of-gamut handling, what "perceptual" actually means
   (it is vendor-defined by design).

---

## Sourcing and legality

**The ICC specifications are published by the International Color
Consortium and are freely downloadable.** Confirm the current licence
terms before bulk-extracting, and record what they permit in
`LEGAL.md §2` — do not assume "freely available" means "freely
redistributable".

CIE and ISO standards are **paywalled**. Do not attempt to obtain them
improperly. Where a paywalled standard is needed, record the gap
explicitly and look for the freely-published equivalent (many CIE
definitions are reproduced with permission in vendor documentation and
academic sources, and the ICC's own specs restate a good deal).

**A recorded gap is worth more than a confident guess**, and this corpus
should say "not sourced" without embarrassment.
