---
name: corpus-defects-are-caught-from-outside
description: With n=9 corpus-made errors (C1, C1b, C2, C3, C4, A28's overstatement, bpc.md §7.2's one-file constant sweep, and the un-transcribed ISO 4.2.5.4 return value, and C5's wrongly-DISQUALIFIED ground-truth fixture), not one was found by re-reading the corpus — arrange for external auditors (the primary document, an execution, a second consumer) rather than for more careful re-reading; includes the "a full-text search is evidence about vocabulary, not content" rule
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

**Note also what the ambiguity register did and did not do.** It tracked C1 correctly (as A1, flagged the whole time). It could not have tracked C4: the wrong sentence never became an `A` row because **it was never uncertain to its author**. A register only records the uncertainty someone thought to write down.

**How to apply:**
- When a corpus file is finished, ask *what would disagree with this, and how soon* — not *did I read it carefully*.
- Prefer arrangements with two consumers (a generator and a parser; a fixture and an oracle) over one, and treat any disagreement between them as a corpus question first, an implementation question second.
- Do not treat a labelling/evidence-tier pass as an audit. Labels route a claim to the check that can falsify it; **they are not the check.** (This sentence is already in `icc__ref__spec_defects.md` §11 and is worth keeping in memory.)
- **★ The pattern extended again 2026-08-12, without adding a `C` id — worth knowing because it shows the SHAPE holds for non-defects too.** Three open questions the corpus had carried for two passes (`icc__ref__bpc.md` §7.1/§7.2/§7.3: is lcms2's v4 BPC forcing Adobe's rule? are its constants Adobe's? vertex or root?) were **all three answered the moment the primary document arrived, and none of them by any amount of re-reading the corpus.** Two answers reversed the expected direction: the forcing rule is **not** in the document lcms2 cites, and the "unattributed" constants turned out to be **ISO/CD 18619's, verbatim**. **A well-labelled open question is the corpus working; it is not a substitute for the document.** See [[verify-a-documents-identity-from-its-title-page]] for what else arrived with it.
- Related: [[shared-layout-is-not-shared-semantics]], [[a-retraction-is-a-grep-not-a-paragraph]], [[label-the-predicate-not-just-the-payload]], [[derived-values-need-a-second-pass]], [[reading-source-is-not-observing-behaviour]].
