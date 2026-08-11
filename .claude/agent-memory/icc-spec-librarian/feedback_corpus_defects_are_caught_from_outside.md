---
name: corpus-defects-are-caught-from-outside
description: With n=5 corpus-made defects (C1, C1b, C2, C3, C4), not one was found by re-reading the corpus — arrange for external auditors (the primary document, an execution, a second consumer) rather than for more careful re-reading
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

**Note also what the ambiguity register did and did not do.** It tracked C1 correctly (as A1, flagged the whole time). It could not have tracked C4: the wrong sentence never became an `A` row because **it was never uncertain to its author**. A register only records the uncertainty someone thought to write down.

**How to apply:**
- When a corpus file is finished, ask *what would disagree with this, and how soon* — not *did I read it carefully*.
- Prefer arrangements with two consumers (a generator and a parser; a fixture and an oracle) over one, and treat any disagreement between them as a corpus question first, an implementation question second.
- Do not treat a labelling/evidence-tier pass as an audit. Labels route a claim to the check that can falsify it; **they are not the check.** (This sentence is already in `icc__ref__spec_defects.md` §11 and is worth keeping in memory.)
- Related: [[shared-layout-is-not-shared-semantics]], [[a-retraction-is-a-grep-not-a-paragraph]], [[label-the-predicate-not-just-the-payload]], [[derived-values-need-a-second-pass]], [[reading-source-is-not-observing-behaviour]].
