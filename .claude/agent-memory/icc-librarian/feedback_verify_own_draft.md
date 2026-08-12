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

5. **Three more, all at the Pass 4 filing (2026-08-11), and one of them
   was in the dispatch itself.** (a) The dispatch described
   `mAB `/`mBA ` as *"undecoded-unevaluated"*; they have been **decoded
   since Pass 2 batch 2** (`tag_types.rs` → `lut::decode_lut_ab`), and
   only the *evaluator* is missing. Filing it as written would have
   understated what Pass 2 shipped and mis-sized what Pass 4 has left.
   (b) `tools/difftest/README.md` §14.7's record decomposition —
   *"8 Pass 3, 1 smoke, 27 Pass 4"* — is **wrong in both terms while its
   total (36) is right**; counting the emitters in the source gives
   7 + 1 + 28. **A sum that comes out right is not evidence that its
   terms are right.** (c) A sentence this librarian had carried through
   **four** filings — *"`tools/gen-profiles/` does not exist and
   `fixtures/synthetic/` holds only its README"* — had become **false**:
   the crate exists with 28 tests and the directory holds **39 `.icc`
   fixtures**. A negative claim about the tree decays exactly as fast as
   a positive one.

6. **★ A TRUNCATED SEARCH IS NOT AN INVENTORY (Pass 4b filing,
   2026-08-11).** Drafting the Pass 4b block, this librarian wrote that
   `bpc.rs` and `named_color.rs` were *"referenced by nothing outside
   their own files"*, from a grep run with `head_limit: 40`. **`bpc.rs`
   is fully wired** — `Chain::with_bpc()` and an `iccce transform
   --bpc` flag — and the `--bpc` matches were **in the truncated tail**.
   Caught before the filing was reported, and recorded in
   `NUMERIC_CLAIMS.md` §7.7 and `SESSION_LOG.md` rather than fixed
   silently. **The rule: when the conclusion is *"X is referenced
   nowhere"*, the search must be UNLIMITED** — an absence proved by a
   truncated list is not an absence. Same family as *a count is not an
   inventory*. (`named_color.rs` was re-checked with no limit and the
   claim held for it.)

7. **A prediction of this project's own, falsified by a DIRECTION
   (same filing).** Three documents said a gray differential would give
   **NA-008** its first measurement. The differential ran **GRAY→RGB**,
   and NA-008 lives in the gray **destination** path — so it is still
   unmeasured. *"A gray differential"* named a comparison, not a
   direction. See [[iccce-direction-scoped-behaviour]].

8. **★ A COMMIT MESSAGE IS A CLAIM, AND IT CAN BE FALSE (Pass 5
   filing, 2026-08-11).** Two commits in one session shipped with a
   failing test under messages asserting a green suite: `70411dd`
   (*"102 workspace tests green"*, gated on a `grep` that **exits 0 on a
   `FAILED` match**) and `6ea1b3d` (gated on the "fixed" pipeline
   `… | tail -2 && commit`, where **`tail`'s 0 masked cargo's 101**);
   corrected in `a36abaf` and `812a215`. **Both were corrected honestly
   and a lesson written** to `C:\personal_rag\claude_code\`, which
   records its own author falling for the second variant minutes after
   writing the first. **The filing rule that follows: no ledger row may
   inherit a gate claim from a commit message**, and a provenance block
   is where the incident belongs, because that is where a reader decides
   what a number is worth. Note also **what the dispatch carried that
   the evidence did not**: the *"104 green"* figure appears in the
   dispatch and **not** in the lesson file — filed as the dispatch's,
   uncorroborated.

9. **★★ "LIVE SOURCE" MEANS *AT THE MOMENT OF FILING* (estimator
   filing, 2026-08-12).** A draft of `NUMERIC_CLAIMS.md` §7.11 accused
   the previous filing of claiming it had rewritten `NEXT_SESSION.md`
   when it had not. **The accusation was false.** It rested on a read of
   that file taken **at the start of the same session**, showing the
   *previous* edition; a re-read minutes before filing showed the
   thirteenth-filing edition, exactly as claimed. **Another librarian's
   filing was still landing while this session was open** — `ROADMAP.md`
   grew its Pass 4 header block mid-session, and the edit tool twice
   reported a file changed on disk between reads. **In a concurrent
   session an early read is a DISPATCH, not a source**: it carries
   exactly the authority of somebody else's report. The near-miss was
   worse than an ordinary error because it would have put a **false
   statement about another agent's honesty** into an append-only
   document. **Cheapest guard: re-read the file a sentence is about, in
   the minute before writing the sentence.**

10. **★ Shell availability is a property of the SESSION, not of the
    agent (same filing).** The Pass 4c filing found a working `Bash`
    tool and corrected *"the librarian has no shell"* from a fact to a
    reading — **and the very next session had no `Bash` tool at all.**
    So neither *"it has no shell"* nor *"it has one"* may be inherited:
    **check the tool grant, per session**, and label items
    `unverified-this-filing` **with the reason** — a held directory and a
    missing shell are different reasons with different fixes.

11. **★★★ THE DISPATCH NAMED A STRING TO CORRECT THAT DID NOT EXIST —
    twice in one filing (4.2.5.4 / `iccce-measure` filing,
    2026-08-12).** Three claims in one dispatch failed against live
    source. (a) *"The manifest header still says `Four crates` — flag it
    as an owed correction"*: `Cargo.toml` **already read "Five crates"**
    and listed all five, so **nothing was owed**. (b) *"`ARCHITECTURE.md`
    §1 currently says `Four crates`"*: the string appeared **nowhere in
    the file** — §1 carried an **ASCII tree that listed four directories
    and omitted the fifth**. Same defect, different text, and **a filing
    that corrected the quoted string would have corrected nothing.**
    (c) *"The previous filing recorded 'suite green at 142'"*: **no
    filing did** — the phrase is a **commit message** (`d5efd96`), and
    the only `142` in `docs/` is the CIE standard number 142-2001. ★
    **The third correction improved the finding, not just the record**:
    the ambiguous number lives in git history, where nothing names an
    apparatus and **no dated note can ever be appended**.
    **Generalisation: when a dispatch says "document X says S, fix it",
    search for S before editing.** A dispatch describing a defect
    accurately can still quote the text wrongly, and the quoted text is
    the part an edit acts on.

**How to apply.**

- ★ **A dispatch's quoted string is a claim about a file — search for
  it.** "Change S to T" is two claims: that S is wrong, and that S is
  there. The second fails more often than the first.
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
- **Never prove a negative with a limited search.** `head_limit` is for
  sampling, never for *"nothing else matches"*.

Related: [[iccce-verification-loop-runs-both-ways]],
[[iccce-predicted-divergence-must-be-measured]], [[iccce-pass-status]].
