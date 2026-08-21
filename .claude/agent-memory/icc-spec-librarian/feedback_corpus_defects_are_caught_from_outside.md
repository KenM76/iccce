---
name: corpus-defects-are-caught-from-outside
description: With n=17 corpus-made errors (C15 added 2026-08-21: a register row's closing "nothing in EITHER corpus would settle it" was false the day it was written — the sibling corpus had the clause, as an ID the row did not name — found when a sibling PROJECT re-derived it and reported it as novel; a non-existence claim about a sibling corpus must be a GREP of it, a row that cites a file must name the ID carrying the evidence, and a report's novelty claim is itself checkable) (C14 added 2026-08-19: a change list naming ONE new requirement was summarised as naming FOUR and became a PERMISSION for old files — found by a consumer asking for the MODALITY of a rule the corpus had only recorded the CONTENT of; the same sweep's `NOT SOURCED` grep found three more stale labels, two of them 8 months old and one quoted in iccce's source) (C13 added 2026-08-19: a "this directory is EMPTY" banner survived seven days and 63 staged files, found by a pass that opened the file to WRITE to it — appending is the one moment a writer passes the top of a file, so treat it as a scheduled banner audit; and the correcting run nearly wrote its own stale counts from memory, fixed by running find) (C12 added 2026-08-18: a RESOLVED A-id never propagated back to the file that raised the gap, found by a dispatch on an unrelated question that merely read the file in passing) (C11 added 2026-08-18: the corpus's OWN recommended next step was executed and its premise did not exist — two passes running) (C8 added 2026-08-17: the refutation was thirty lines away in the same file and still needed an outside procedure to find) (C1, C1b, C2, C3, C4, A28's overstatement, bpc.md §7.2's one-file constant sweep, and the un-transcribed ISO 4.2.5.4 return value, and C5's wrongly-DISQUALIFIED ground-truth fixture), not one was found by re-reading the corpus — arrange for external auditors (the primary document, an execution, a second consumer) rather than for more careful re-reading; includes the "a full-text search is evidence about vocabulary, not content" rule
metadata:
  type: feedback
---

**A corpus cannot audit itself. Optimise for how fast something *outside* it will disagree, not for how carefully it was re-read.**

**Why:** as of 2026-08-12 the ICC_Spec corpus has made **nine** recorded defects, and every one was overturned by a different mechanism than the one that let it through — **none by re-reading the corpus**:

| id | The claim | Overturned by |
|---|---|---|
| **C1** | the standard's *rule*, inferred from an implementation | reading the primary document |
| **C1b** | that retraction, filed as a heading and never swept | a second consumer hitting the contradiction |
| **C2** | an *arithmetic derivation*, labelled but unaudited | running an implementation (an iccce unit test) |
| **C3** | an *implementation's behaviour*, inferred from its source | running that implementation |
| **C4** | a *per-type rule*, generalised from a shared binary layout | two consumers of the corpus disagreeing with each other |

C4 and C1b were both surfaced the same day by `icc-conformance`'s `tools/gen-profiles`: a fixture authored from the corpus was refused by a parser written from the same corpus. **Two independent readers of one sentence is a cheap and unusually effective audit** — cheaper than a third transcription pass, and it finds a different class of error.

**★ n=6, added 2026-08-11 (8th pass): `A28` said BPC is "not in ICC.1:2022 at
all — confirmed by full-text search". The scaling map is in clause 6.3.4.3.
It was found by reading the primary document for a DIFFERENT question** — a
Tier 2 sourcing sweep that had no reason to expect the answer to be inside
ICC.1. **The specific error is worth naming, because it will recur: a
full-text search is evidence about VOCABULARY, not about CONTENT.** The phrase
"black point compensation" really is absent; the mechanism is not. Any register
row whose justification is "confirmed by full-text search" is asserting a
negative about wording and should not be read as a negative about substance.
Same family as the `A9` "clipping" sweep, which found six normative rules only
after the search moved from the word to the operation.

**★ n=7, added 2026-08-11 (Pass 5 differential feedback): `icc__ref__bpc.md`
§7.2's "the unattributed constants" list was drawn from ONE source file
(`cmssamp.c`, the estimator) and therefore missed `cmscnvrt.c`'s `IsEmptyLayer`
`0,002` — the one constant that can switch BPC OFF entirely.** Found by the
Pass 5 differential reading the *linker* for a different question. **Same
family as the A28 full-text-search error, one level up: a SWEEP is evidence
about the PATHS IT COVERED, and a sweep that does not name them reads as
exhaustive.** Generalised rule now in the corpus: any list of "all the X in
implementation Y" must state the files/functions searched. Cheap fix, and it
converts a false negative into a stated scope.

**★ n=8, added 2026-08-12 (11th pass): the corpus SUMMARISED ISO/CD 18619
4.2.5.1's short-circuit return value (`icc__s__bpc_algorithm.md` §4.3, one
clause of one sentence — "mid-range straight → `InitialLab`") and never
TRANSCRIBED 4.2.5.4, so nothing in the corpus said `outRamp[first]` was wrong.
An implementation shipped `outRamp[first]`, and it was caught by
`icc-conformance` MEASURING the difference against lcms2** — `8,166 8×10⁻² ΔE76`
— **not by anyone re-reading the summary.** **The lesson is specific and
actionable: a summary of a normative sentence is not a substitute for the
sentence, precisely because a summary reads as complete.** The tell was
available and unnoticed: §4.3 mentioned the return value only in passing, inside
a section about a *different* document's scoping defect, so it was filed under
"Adobe's bug" rather than under "what the algorithm returns". **Where a clause
states a RETURN VALUE, transcribe it verbatim at the place an implementer will
grep for it, even if another section already paraphrased it.**

**★ n=9, added 2026-08-12 (13th pass): `C5`. `icc__ref__spec_defects.md` §13.2 point-evaluated a worked example whose printed inputs were intervals, called ONE typo TWO, and on the strength of the apparent pattern wrote "the black row is not ground truth" — disqualifying ICC.1:2022 Annex D.6.3, THE only published transform ground truth in the field.** Overturned when `icc-engineer` asked a **different question** ("does published ground truth exist *anywhere*?"), forcing the example to be re-derived from scratch by an agent that had not read §13.2 first. **The mechanism is the one this memory keeps predicting: a fresh derivation from the primary document, prompted by a question the original entry was not written to answer.** The tell had been sitting in §13.2's own table for eight passes — `Z` marked ✔ at the same `0,4 %` agreement `Y` was marked ✘ for — **and no amount of re-reading found it, because to its author the entry was not uncertain.**

**★ The new-shape lesson from C5, distinct from the interval arithmetic (that part lives in [[derived-values-need-a-second-pass]]): a corpus entry that DISQUALIFIES something is far more dangerous than one that asserts something, and is checked far less often.** An assertion invites the next consumer to test it. A disqualification tells them not to look — so it removes its own auditor. **C1, C1b, C2, C3, C4 were all wrong assertions and all were caught within days by someone using them. C5 was a wrong *rejection* and survived eight passes and eleven filings, because nobody uses a fixture they were told is broken.** **How to apply: any sentence of the form "X is not usable / not ground truth / not reliable" gets the same evidence bar as a load-bearing positive claim, plus an explicit statement of what it costs if the rejection is wrong.**

**★ 2026-08-17 — the shape held again, WITHOUT a new `C` id, and it is the sweep-scope family.** `icc__ref__spec_defects.md` §1b announced "**44** stale clause references" from a scripted audit whose regex was `(\w+Type)\s*\(see\s+10\.N\)`. A dispatch about *colour space signatures* — nothing to do with cross-references — walked into **two more, in clause 9.1 and 9.2.1, both bare numbers in running prose that the regex could not match**, plus **two stale TABLE references (Table 38 for Table 41)**, an axis the audit never covered at all. **§1b was right to name its regex; that is exactly why the extension cost one grep instead of a re-derivation.** The rule to keep: **a count produced by a pattern is a count of what the pattern matches**, and the fix is not a better regex but publishing the regex beside the number. Same family as `A28`'s full-text search and `bpc.md` §7.2's one-file sweep.

**★★★ n=10, added 2026-08-17 (15th pass): `C8`, and it is the STRONGEST case this memory has.** `iec/iec__s__srgb.md` claimed the wrong sRGB breakpoint *"affects 8-bit codes 10 and 11 only."* **No 8-bit code is affected at all** — separation at 8-bit precision is exactly zero, all 256 codes. Overturned by `icc-conformance` **injecting the wrong constant into iccce and watching all five of its tests pass** — a procedure, not a reading. **And the refuting number was printed THIRTY LINES ABOVE the false one in the same file** (`−7.55×10⁻⁷`, the discontinuity the separation cannot exceed); the contradiction is one subtraction. **It was internally available for six days and internally invisible.** ⟹ *internal availability does not produce catching; only an external consumer with a procedure does.* **Corollary worth holding: "could a careful re-read have caught this?" is the wrong question — C8 proves the answer can be yes and the outcome still no.** Full lesson, including why false precision repels checking, in [[derived-values-need-a-second-pass]].

**★★★ n=13, added 2026-08-18 (19th pass): `C11`, and it names a NEW external auditor — the corpus's own recommended next step.** Two corpus files closed with a "cheapest untried lead": *run the web-platform tests CSS Color 4 names beside each example.* A dispatch executed it. **The premise did not exist.** The `<wpt>` annotations in a CSS spec attach to a **section**, not to an **example**; the corpus had extracted test filenames from a ±2 500-character window around each example in the held HTML, seen them adjacent, and written the adjacency up as *"the associated tests … named beside each example"*. **None of the failing examples' numbers are anywhere in the repository** — six zero-hit code searches, plus a **positive control** (`0.12266` → `predefined-016.html`) without which the six zeroes would have proved nothing.

**★ The generalisation, and it is the one to keep: EXECUTING A CORPUS'S OWN RECOMMENDED NEXT STEP IS A TEST OF THAT RECOMMENDATION, and this corpus has now failed that test twice running** (`C10`, then `C11`). A "next steps" list is written at the end of a pass, when attention is lowest and nothing downstream consumes it — so it is the least-audited prose in any file **and it sets the premise of the next dispatch.** **How to apply: give a proposed next step the same evidence bar as a finding. If it asserts that a resource contains something, say how that was established; if it was inferred from a rendering, write "no stated relationship was found" instead of "the associated X".**

**Family:** `C11` is [[shared-layout-is-not-shared-semantics]] with "layout" being **document position** rather than a C struct. **Grep trigger:** *"the X associated with each Y"*, *"named beside"*, *"the corresponding Z"* — wherever the correspondence was read off a rendering (HTML proximity, PDF page adjacency, table-row alignment) rather than out of a statement.

**n=11 and n=12** are `C9` and `C10` (the gap-hardened-into-non-existence pair).

**★ n=14, added 2026-08-18: `C12`, and it is the WEAKEST auditor yet — which is the point.** `icc__s__header.md` still read *"NOT SOURCED, flagged A7"* seven days after `A7` was RESOLVED from the primary, three screens below that same file's own resolved-`A7` row. **No procedure found it. No execution found it. A dispatch on an unrelated question (who consumes the header rendering-intent field) had to open the file for context and the line was in the way.** Two things worth carrying: **(a)** the corpus's sweep procedure is written for *retractions* and is structurally blind to *promotions*, because a promotion shares no distinguishing phrase with the claim it supersedes — grep the **id**, not the sentence (see [[a-retraction-is-a-grep-not-a-paragraph]]); **(b)** if an auditor this incidental still finds one, the density of remaining stale sites is higher than the tally suggests, and the tally is a lower bound.

**★★ n=15, added 2026-08-19: `C13`, and it names a THIRD external mechanism — the act of WRITING to the file.** `_sources\README.md` line 3 read *"**Empty as of 2026-08-11.** This is a recorded gap, not an oversight"* for **seven days**, while the tables **below it in the same file** grew to 63 staged documents; and the table's first row still called `ICC.1-2022-05.pdf` *"★ MISSING — the corpus's top blocker"* **while that file sat in the directory being described.** Found by a pass that opened the file **to add four new sources**, not to review it.

**The mechanism generalises and is cheap:** appending is the one moment a writer's eye necessarily passes the **top** of a file, and the top is where summary banners live — the highest-influence, least-revisited prose in any document. ⟹ **treat "I am appending to this file" as a scheduled audit of its banner.** Same family as `iec\iec__s__srgb.md`'s five-day "single source" banner, which a dispatch then quoted back as current state.

**★ And the correcting run nearly wrote its own stale line.** Its first draft asserted directory counts from memory of a listing read earlier in the same session — **three of four wrong** — and stated the ICC.1 PDFs "must be dropped here by a human" while they were present. Caught by running `find` before the sentence stood. ⟹ **the fix for a stale count is not a better estimate, it is a command: any "current state" claim about a directory must be produced by a listing in the same action that writes it.**

**★★★ n=16, added 2026-08-19: `C14`, and it names a FOURTH external mechanism — a consumer asking for the MODALITY of a fact the corpus had only recorded the CONTENT of.**

`pdfce` asked iccce for `Malformation::is_violation()`; iccce asked this corpus, per variant, *"does a file carrying this BREACH a requirement?"* **The corpus had never been asked that, and had no place to put the answer.** What it had was `icc__s__tag_table.md`'s *"Validation checks iccce should perform and REPORT"* — a list of checks worth **performing**, with no clause per row, **read (reasonably) by two consumers as a list of conformance rules.** Two of its six rows have no requirement behind them in either edition.

The dispatch also surfaced the real defect one level up: clause 7.3.1 states **four** requirements, the ICC.1:2022 Foreword names **one** as new, and this corpus wrote *"the Foreword confirms **these** are new … **so v4.3 and v2 profiles may legitimately violate all four**."* **False for the uniqueness rule** — ICC.1:2001-04 **6.2** prohibits duplicate tag signatures in words, and v2's own change list dates the prohibition to a **1998-03-15 resolution**. The refuting document had been on disk for seven days.

**Three things to carry:**

1. **★ A version-gate sentence of the form *"older files may legitimately violate this"* is a PERMISSION granted on the corpus's own authority, and it forecloses its own check** — exactly `C5`'s disqualification shape and `C10`'s existence shape, pointed at conformance. **An `Unsourced` verdict invites a later look; "legitimately violate" does not.** Whenever a clause states *n* rules and a change list names *m < n*, the version gate is written **per rule**, and the unnamed rules get their own search in the older edition.
2. **★ A batch resolution leaves a batch of stale prose.** Widening the fix to a whole-corpus `NOT SOURCED` grep found **four** stale labels, not two: two in `icc__s__tag_table.md` **contradicting that same file's own banner for eight months** (`A12`, `A13`), and two — in `icc__ref__v2_v4_divergence.md` **D4** (`A2`) and `icc__s__required_tags.md` §7 (`A34`) — both resolved on 2026-08-12 **by the arrival of ICC.1:2001-04**. **A document acquisition closes register rows in a batch, and a batch is the update most likely to reach the register and not the prose files that feed it.** ⇒ *when a newly-acquired source resolves N rows, grep for each id AND for each row's phrase.* The register is the index; **the prose files are what a consumer reads** — and one of these stale labels was quoted verbatim in iccce's own source (`Malformation::DuplicateTagSignature`: *"Legality NOT SOURCED"*, citing the id of the row that resolved it).
3. **The recount script that was supposed to detect drift had the drift.** It bucketed rows by testing `RESOLVED` before `PARTLY`, so every `PARTLY RESOLVED` row counted as `RESOLVED`. **A self-check written in code is still a claim; test its ordering on a known case.**

**★★★ n=17, added 2026-08-21: `C15`, and it names a FIFTH external mechanism — a SIBLING PROJECT RE-DERIVING THE SAME CLAUSE AND REPORTING IT AS NOVEL.**

`A52` closed with *"What would settle the 2.0 residue: **nothing in either corpus** — it is `pdfce`'s reading to make."* **False, and false on the day it was written (2026-08-19).** The clause that settles most of it — ISO 32000-2:2020 §10.5 / ISO 32000-1:2008 §10.4, which calls `c=m=y=0` **"the normal conversion"** and hangs a `shall not` on it, **in both editions, outside §10.4.2 entirely** — was written into `PDF_Spec/iso32000/iso32000__s__10.md` **the same day**, as `REND-11` (§6) and evidence row `P5` (§7.1).

**The auditor was the `pdfce` session, which re-derived the clause independently on 2026-08-21 and reported it as *"a third clause neither of us cited."*** True of `A52`; **false of `PDF_Spec`**. So the two sessions reached it independently — genuine corroboration — and the only thing that actually failed was **the pointer**: `A52` cited the right file and named `§7.4 / REND-6 / REND-A1`, **not `REND-11`**, so the corpus's own strongest evidence was invisible from inside the register that summarised it. **Cost: a full duplicated derivation in another project.**

**Four things to carry, and the first two are new shapes:**

1. **★★ A NON-EXISTENCE CLAIM SCOPED TO A SIBLING CORPUS MUST BE A GREP OF THAT CORPUS.** *"Nothing in **either** corpus"* is [[icc-corpus-gap-vs-nonexistence-claim]] arriving at **cross-corpus** scope, and it is worse there: the author has *write* access to one corpus and only *recollection* of the other, yet the sentence reads with equal authority about both. **Grep the sibling before writing a sentence about what it does not contain.** Trigger phrases: *"nothing in either corpus"*, *"neither project has"*, *"not covered anywhere"*.
2. **★★ A ROW THAT CITES A FILE MUST NAME THE ID INSIDE IT THAT CARRIES THE EVIDENCE.** A bare file pointer degrades to "somewhere in 700 lines" and a *partial* ID list is worse than none — it reads as exhaustive. This is [[label-the-predicate-not-just-the-payload]] applied to a **pointer**: the pointer's predicate ("what in there answers THIS question?") went unrecorded.
3. **★ A REPORT'S NOVELTY CLAIM IS ITSELF CHECKABLE, AND CHECKING IT IS FREE.** When another session says *"here is something neither of us has"*, grep your own corpus **for the clause, not for their framing** — before accepting the framing. Here the substance was right and the attribution was wrong, and **filing the attribution unchecked would have recorded a corpus gap that did not exist**, which is the mirror image of `C9`/`C10`.
4. **★ RE-DERIVATION AND ADOPTION ARE DIFFERENT CLAIMS AND MUST NOT SHARE WORDING.** The dispatch required *"verified from primary / not found / partially confirmed"* plus **who reported it, when, and who re-derived it, when** — because `pdfce` had declined to adopt this corpus's reading on *its* say-so, on the ground that **"two projects agreeing because one relayed it to the other is not corroboration."** That is the correct rule and it is now reciprocal. **An adoption recorded in the words of a verification permanently destroys the distinction**, and nothing downstream can recover it.

**And the re-derivation found a THIRD thing neither session had said:** §10.5's `shall not` **binds the transfer-function step, not the conversion** — "the normal conversion" is a **presupposition**, so the clause is decisive evidence about what the standard treats as normal and is **not a third `shall`**. `PDF_Spec` had this right (`P5`'s modality column: *"`shall not`, on a presupposition"*); the incoming report did not. **Re-deriving a confirmed finding is not wasted work — it is where the qualifier appears.**

The running tally is **seventeen for seventeen caught from outside.**

**Note also what the ambiguity register did and did not do.** It tracked C1 correctly (as A1, flagged the whole time). It could not have tracked C4: the wrong sentence never became an `A` row because **it was never uncertain to its author**. A register only records the uncertainty someone thought to write down.

**How to apply:**
- When a corpus file is finished, ask *what would disagree with this, and how soon* — not *did I read it carefully*.
- Prefer arrangements with two consumers (a generator and a parser; a fixture and an oracle) over one, and treat any disagreement between them as a corpus question first, an implementation question second.
- Do not treat a labelling/evidence-tier pass as an audit. Labels route a claim to the check that can falsify it; **they are not the check.** (This sentence is already in `icc__ref__spec_defects.md` §11 and is worth keeping in memory.)
- **★ The pattern extended again 2026-08-12, without adding a `C` id — worth knowing because it shows the SHAPE holds for non-defects too.** Three open questions the corpus had carried for two passes (`icc__ref__bpc.md` §7.1/§7.2/§7.3: is lcms2's v4 BPC forcing Adobe's rule? are its constants Adobe's? vertex or root?) were **all three answered the moment the primary document arrived, and none of them by any amount of re-reading the corpus.** Two answers reversed the expected direction: the forcing rule is **not** in the document lcms2 cites, and the "unattributed" constants turned out to be **ISO/CD 18619's, verbatim**. **A well-labelled open question is the corpus working; it is not a substitute for the document.** See [[verify-a-documents-identity-from-its-title-page]] for what else arrived with it.
- Related: [[shared-layout-is-not-shared-semantics]], [[a-retraction-is-a-grep-not-a-paragraph]], [[label-the-predicate-not-just-the-payload]], [[derived-values-need-a-second-pass]], [[reading-source-is-not-observing-behaviour]].
