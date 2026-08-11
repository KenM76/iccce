---
name: iccce-verify-own-draft-too
description: The verify-against-live-source rule applies to the librarian's OWN draft text, to totals carried in a dispatch, and to the tree's own shape — a file enumeration is a claim with a timestamp
metadata:
  type: feedback
---

**Verify every claim about the tree that reaches a filing — including
the ones this librarian wrote itself, and including a total that arrives
looking like evidence.**

**Why.** Three concrete incidents, all 2026-08-11:

1. Drafting the Pass 2 batch 2 ROADMAP block, this librarian wrote
   *"iccMAX identification and refusal by name — not delivered by either
   batch"* into an owed-work list. **It was false.** `Profile::parse`
   has refused major version ≥ 5 with `ParseError::IccMaxRefused` since
   **Pass 0**, and `iccmax_is_refused_by_name` asserts the message
   contains the literal string `"iccMAX"`. A grep caught it before the
   edit landed. The dispatch never claimed otherwise — **the wrong claim
   was the librarian's own inference from the Pass's plan text.** A
   plan item that nobody checked and a plan item that is done look
   identical in a to-do list.
2. The machine-wide sweep arrived as **"40 profiles, 40 parse OK, 0
   malformations."** That is a *total*, and it was dispatched in the
   same breath as a batch that added four LUT tag types — inviting
   *"the LUT decoders survived 40 real profiles."* **The dispatch
   carried no per-tag-type breakdown**, and a Windows colour directory
   is the wrong population for `mAB `/`mBA ` anyway. The total
   establishes no-crash robustness; it establishes nothing about which
   code ran.

3. **A prediction filed twice, refuted by the code.** The Pass 3
   annotation in `ROADMAP.md` and the carried list in
   `NEXT_SESSION.md` — **both written by this librarian** — said
   *"NA-002's Bradford cost comes due at Pass 3, because sRGB→AdobeRGB
   adapts."* When Pass 3 landed, reading the source showed **it does not
   adapt at all**: `iccce-cmm` imports only `Mat3` and `Xyz`, never
   touches `adapt.rs`, never reads `chad`/`wtpt`. The dispatch did not
   contradict the prediction; **the code did.** Left standing as the
   record of what was expected, corrected by a dated note. **A forecast
   about a future Pass becomes a claim about the tree the moment that
   Pass lands, and it has to be re-checked then — not carried forward.**

4. **The TREE ITSELF moved mid-filing, and a `Glob` result was a claim
   with a timestamp.** At the Pass 3 closure filing (2026-08-11) an
   enumeration of `crates/**/*.rs` did not list
   `iccce-cmm/src/lut_transform.rs`; a grep minutes later found it, with
   four tests and a module doc reading *"Pass 4 assembly, stage 1"*.
   **The closure dispatch's six-commit list never mentioned it.** Either
   an agent was writing concurrently or the first enumeration was stale
   — **and with no shell there is no way to tell which.** The filing said
   so instead of picking the tidier reading. Lesson: *"I enumerated the
   tree"* is evidence about **the moment of enumeration**, and in a
   parallel-dispatch session that moment expires quickly. Re-check
   anything load-bearing at the end of the filing, and when a file
   appears that no dispatch claimed, **record its existence and its
   unknown commit status** rather than absorbing it into the narrative.

**How to apply.**

- **Re-verify every carried-forward prediction on the Pass that was
  predicted about.** A "what Pass N inherits" annotation is written
  before Pass N exists; the day it exists, it is a testable claim.
- Before writing *"X is not delivered"* or *"X does not exist"*, grep
  for X. The cost is one tool call; the cost of the alternative is a
  permanent document asserting a false fact about the repository.
- When a correction happens mid-draft, **record the correction rather
  than silently deleting the wrong line** — a reader needs to be able
  to tell "checked and done" from "never checked."
- A count in a dispatch is a count. Ask what breakdown would turn it
  into coverage, and if the dispatch did not carry one, say so in the
  filing and add it to owed work.

Related: [[iccce-verification-loop-runs-both-ways]],
[[iccce-predicted-divergence-must-be-measured]], [[iccce-pass-status]].
