---
name: feedback-check-sources-before-accepting-corpus-gap
description: When an agent reports a specialised RAG lacks a clause, check the corpus's _sources/ primary PDFs before accepting the gap — the digests are incomplete, the sources often are not
metadata:
  type: feedback
---

**When a dispatched agent reports "the corpus does not hold clause X",
treat that as a statement about the corpus's *digests*, not about what is
on disk. Check `_sources/` before accepting it, and before spending
anything on acquiring the document.**

**Why:** on 2026-08-17 two separate agents — `icc-spec-librarian` and
`icc-librarian` — independently reported that
`D:\Dev\Rag-Specialized\PDF_Spec\` holds no clause 10 of ISO 32000, and
that the load-bearing clause of a whole boundary reclassification
(cl. 10.3.2, which decides whether `DeviceGray`→CMYK belongs to the PDF
processor or to the CMM) therefore stood `[REPORTED]` on a single
retrieval and could not be corroborated. **Both were right about the
digests and wrong about the corpus.**
`PDF_Spec\_sources\ISO_32000-2_sponsored_EC3.pdf` — the full ISO
32000-2:2020 text — was on disk the entire time, along with
`PDF32000_2008.pdf` and a dozen other primary standards. One
`pdftotext -layout` and one `grep` corroborated the clause verbatim and,
as a bonus, **resolved an ambiguity that had just been filed as
unresolvable** (cl. 10.4.2.1 turned out not to conflict with cl. 10.3.2 at
all once the full text was read, rather than a digest's summary of it).

The digests are curated by topic and are honest about their own gaps —
`iso32000__s__8.6.md` lists clause 10 as an open gap. That honesty is
what makes the failure mode subtle: **the corpus correctly reports a gap
in itself, and the gap is only in the layer that was searched.**

**How to apply:** before writing `[REPORTED]` against a clause, before
telling the operator a document must be acquired, and before asking a
consumer project to re-derive something — run one `ls` of the relevant
`_sources/`. It costs a single tool call. Agents with no shell
(`icc-librarian`) structurally cannot do this, which is a reason for the
orchestrator to do it rather than to delegate it again.

★ **Licence caution attached to the same folder:** the ISO 32000-2 copy
there is a **single-user PDF Association licence issued to the operator**,
watermarked *"copying and networking prohibited"*. Short quotation with a
clause citation is normal technical-reference practice and is what belongs
in a doc comment; **the file must never be redistributed and no bulk
transcription may enter an MIT repository.** Same posture as
[[project-ghent-compatibility]]'s corpus.

Related: [[reference-request-channel-polling]],
[[feedback-compatibility-not-compliance]].
