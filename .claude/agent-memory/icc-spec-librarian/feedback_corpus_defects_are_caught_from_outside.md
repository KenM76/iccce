---
name: corpus-defects-are-caught-from-outside
description: With n=7 corpus-made errors (C1, C1b, C2, C3, C4, A28's overstatement, and bpc.md §7.2's one-file constant sweep), not one was found by re-reading the corpus — arrange for external auditors (the primary document, an execution, a second consumer) rather than for more careful re-reading; includes the "a full-text search is evidence about vocabulary, not content" rule
metadata:
  type: feedback
---

**A corpus cannot audit itself. Optimise for how fast something *outside* it will disagree, not for how carefully it was re-read.**

**Why:** as of 2026-08-11 the ICC_Spec corpus has made five recorded defects, and every one was overturned by a different mechanism than the one that let it through — **none by re-reading the corpus**:

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

**Note also what the ambiguity register did and did not do.** It tracked C1 correctly (as A1, flagged the whole time). It could not have tracked C4: the wrong sentence never became an `A` row because **it was never uncertain to its author**. A register only records the uncertainty someone thought to write down.

**How to apply:**
- When a corpus file is finished, ask *what would disagree with this, and how soon* — not *did I read it carefully*.
- Prefer arrangements with two consumers (a generator and a parser; a fixture and an oracle) over one, and treat any disagreement between them as a corpus question first, an implementation question second.
- Do not treat a labelling/evidence-tier pass as an audit. Labels route a claim to the check that can falsify it; **they are not the check.** (This sentence is already in `icc__ref__spec_defects.md` §11 and is worth keeping in memory.)
- Related: [[shared-layout-is-not-shared-semantics]], [[a-retraction-is-a-grep-not-a-paragraph]], [[label-the-predicate-not-just-the-payload]], [[derived-values-need-a-second-pass]], [[reading-source-is-not-observing-behaviour]].
