---
name: project-a-fixed-defect-goes-stale-in-someone-elses-doc
description: A document that says "X is broken and someone else owns the fix" goes stale the moment that someone else commits — the two commits were 28 seconds apart and nothing in the fixing commit's review touched the doc; also the correction's bar (keep the numbers as dated history, split the two orders, do not read as closed).
metadata:
  type: project
---

**A stale STATUS decays faster than a stale numeral, and it decays
silently.** Recorded 2026-08-18 after correcting `docs/TOLERANCES.md`
§3.10.12.7 / §3.10.12.6 and `tools/difftest/README.md` §25.13.7 / §25.13.6.

**Why:** §3.10.12.7 was written while the compiled-path black-preservation
defect was live, headed *"A FINDING WITH NO ROW"*, and said **"Not fixed
here, deliberately… what this role owes WHEN it is fixed."** The remedy
landed in `a05476c` at 02:40:05 -0400; the conformance text was committed in
`a1bd818` at 02:40:33 — **twenty-eight seconds later**. From the document's
point of view the fixing commit is an unrelated change in another directory,
so **nothing in its review touches the document that made the claim**. A
wrong *number* invites re-derivation the next time someone measures; a wrong
*status* invites a reader to act on an obligation already discharged, or to
distrust working code. This one was false in the **unusual** direction — the
document understated the code — and it was still a false claim built to be
quoted onward.

**How to apply.**

- **Before quoting your own prior filing, re-run the thing and `git log` the
  crate it names.** If a section names another role as the owner of a
  remedy, that section has an expiry date set by *their* commit, not yours.
  Grep your own docs for "not fixed here", "belongs to the engineer", "when
  it is fixed", "is measurably wrong" at the start of any currency check.
- **The correction's bar, which is not "delete the wrong sentence":**
  1. **Keep the defect's numbers**, re-labelled as dated history with the
     fixing commit named. `0.617121` / `0.617148` are the measured signature
     of a real defect and the evidence for why the *structural* remedy was
     the right one of two candidates — deleting them erases why the decision
     went that way. Same posture §4 already takes toward superseded
     envelopes.
  2. **Split anything the defect shared a sentence with.** The reading was
     *"`O(1)` beside `O(h^1.32)`"*: the `O(1)` half is the defect's
     signature and is gone, `O(h^1.32)` is Pass 6's still-live measured
     order (DL-025, NC-149). Quoting the pair unsplit reads as though both
     were retracted or both still live.
  3. **Do not let it read as CLOSED.** The fix discharged the defect, not
     the debt: there is still **no difftest row** for the compiled path, and
     a `crates/` test carries no tolerance from `TOLERANCES.md`, is not
     separation-graded, and never reaches a `summary` line. What changed is
     the row's *purpose* — **disclosure → regression guard**.
  4. **Date it, attribute it, append a §4 change row**, and quote the
     retracted wording verbatim inside the correction so the document does
     not read as though it was never wrong.

Related: [[project-stale-claim-strings-in-emitted-records]] (the numeral
form of the same decay), [[project-a-fixed-defect-can-blind-its-own-row]]
(the *test* form — a fix that makes a row unable to see the defect return),
[[project-passk-grading-the-landed-feature]], [[project-doc-editing-conventions]].
