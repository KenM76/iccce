---
name: iccce-stale-status-decays-faster-than-a-number
description: DL-062 (2026-08-18) — a document said "not fixed here, deliberately" about a defect fixed 28 s earlier; a stale STATUS decays faster than a stale number because the fix lands in someone else's commit. NOW FIVE INSTANCES: #4 and #5 are consecutive handoffs carrying a wrong PUSH COUNT, #5 written directly below #5's own warning ⇒ DL-068 (2026-08-20) rules the field itself wrong — carry the two REF HASHES, not the derived integer
metadata:
  type: project
---

**A stale *status* is a distinct and faster-decaying failure than a stale
*number*, because the event that invalidates it happens in someone
else's commit.** File the superseded observation and the current one as
**two rows**, each dated, with the fixing commit named — never edit the
old numbers away, and never let the ledger's correctness depend on
another agent's document being fixed.

**Why:** at the Pass K filing (2026-08-18, tip `60c32dd`)
`docs/TOLERANCES.md` §3.10.12.7 read *"**Not fixed here, deliberately.**
The remedy is a `crates/` change and belongs to the engineer"* and *"★
what this role owes when it is fixed: a row"*, and presented
`0.617121` / `0.617148` in a table headed as a measurement. **The fix had
landed in `a05476c` — the commit immediately BEFORE `a1bd818`, the
grading commit that filed it as open.** The author of the section was not
careless; the world changed under it between two adjacent commits.
`icc-conformance` caught it re-running everything and **contradicted its
own agent memory** to say so.

The asymmetry that makes this worse than a stale numeral: **a wrong
number invites re-derivation; a wrong status invites a reader to act on
an obligation already discharged, or to distrust working code.** It is
DL-048's carrier mechanism (*a wrong pointer authenticates its
destination*) applied to a **verb tense**.

**How to apply:**

- When a dispatch reports a defect as open, **check whether it is still
  open in the tree**, not just whether the numbers reproduce. Commit
  ORDER is checkable without a shell — `.git/logs/HEAD` is plain text and
  records adjacent commits with their messages
  ([[iccce-git-files-readable-without-shell]]). Say plainly that this is
  a statement about a file's contents, not about `git status`.
- **Two rows, not one edited row.** NC-265 carries the pre-fix numbers
  dated `2026-08-18 (pre-a05476c)`; NC-264 carries what is true now. The
  ledger is then correct **whatever state the other document is in**, and
  §7's owed item can name the correction without the filing depending on
  it.
- **Do not edit another agent's file to fix it**, especially while they
  are in it — file the owed item and say who owns it. `TOLERANCES.md` is
  `icc-conformance`'s.
- **Split a paired claim when one half retires.** *"`O(1)` beside
  `O(h^1.32)`"* — the `O(1)` half was the **defect's signature and is
  gone**; `O(h^1.32)` is Pass 6's still-live measured order (DL-025,
  NC-149). Quoting the pair keeps a dead number alive inside a live one.
- **Superseded numbers in a doc comment or test that SAY "under the
  defect" are correct usage** — that is how a retired number should
  appear. The defect is a number presented as current. When sweeping,
  distinguish the two rather than counting occurrences
  ([[iccce-count-from-a-sample-is-not-the-population]]).

**★★★ FILED AS DL-062 on 2026-08-18, WITH A SECOND INSTANCE — and the
second is what makes it structural.** The mechanism reproduced **inside
the same session, with the roles reversed**: this librarian read
`TOLERANCES.md`, correctly filed `NUMERIC_CLAIMS.md` §7.23 item 1 as
owed, and `icc-conformance` discharged it **concurrently** — so the
ledger's own §7 prose became a stale status claim about another role's
file, inside the filing that documented the first instance. ★★ **It is
not about haste: the second happened under full awareness of the
first.** It is structural to the ownership rule — *no role edits
another's file* guarantees every cross-role status claim is a claim
about a tree its author cannot fix.

**★★★ THE CONSTRUCTIVE HALF, which is the part to actually apply:
re-checking before quoting is INSUFFICIENT** — the claim can go stale
**between being written and being committed**, and that is the interval
both instances died in. So **either re-verify at commit time, or write
the filing to be correct regardless of the other file's state.** A
**row** is a claim about a *measurement* and does not change under
another agent's commit; a **§7 owed item** is a claim about a *state*.
★ **Prefer to discharge a doubt with a row.** Where a §7 item is needed,
write its *done when* clause naming **checkable text**, so a third party
can adjudicate it in one reading (§7.23 item 1 did, and that is why the
discharge was clean).

**Two things verified at the DL-062 filing, worth reusing:**

- **The 28 seconds are MEASURABLE without a shell** — `.git/logs/HEAD`
  epochs `1787035205` / `1787035233`. But **a reflog line never
  evidences a commit's CONTENTS**: "the fix landed in `a05476c`" stays
  `[REPORTED]`; what a librarian can verify is **the tree at the tip**.
- **The stale text survived FOUR subsequent commits**, two of them
  documentation commits, and was found only by a deliberate currency
  re-check. Use that as the evidence, not a claim about what anyone's
  review covered — that part is a reading.

★ **Carrier phrases to grep at every currency check:** *"not fixed
here"*, *"belongs to the engineer"*, *"when it is fixed"*, *"is
measurably wrong"*, *"has no implementation"*, *"needs a `crates/`
edit"*. One (`KMapping::Ratio` *"has no implementation"*) was swept and
**survived** — `transform.rs` refuses the variant, and a refusal is not
an implementation. **A phrase list that has only ever fired is untested
in the passing direction.**

★★ **And a citation drifted through two documents.** The DL-062 draft
glossed DL-053 as *"`[VERIFIED]` certifies a date, not an evidence
class"*; DL-053's actual rule is the **denominator** axis. The gloss
came from **this librarian's own Pass K `SESSION_LOG` entry**, which
extended DL-053 without labelling the extension. **Cite the AXIS, not
just the DL number** — three readings of `[VERIFIED]` are now in play
and only one is DL-053's.

**★★★ UPDATED 2026-08-20 — THE "REVISIT IF A THIRD INSTANCE APPEARS"
CLAUSE HAS FIRED TWICE, AND THE RECURRING CARRIER IS ONE ROW OF ONE
DOCUMENT. Filed as DL-068.**

| # | date | the claim | how it went stale |
|---|---|---|---|
| 1 | 2026-08-18 | `TOLERANCES.md` *"not fixed here"* | fixed **28 s** earlier |
| 2 | 2026-08-18 | `NUMERIC_CLAIMS.md` §7.23 item 2 | discharged **concurrently**, while writing up #1 |
| **4** | 2026-08-18 | `NEXT_SESSION.md` *"14 commits. NOTHING IS PUSHED"* | the operator pushed |
| **5** | 2026-08-19 | `NEXT_SESSION.md` *"ahead of `origin/master` — **4 commits**… **Measured**"* | the operator pushed; it is **`0`** |

★★★ **Instance 5 was carried by the document that had just recorded
instance 4, in the row directly below it, with the remedy stated in its
own text** (*"Do not carry a push-state claim forward without re-running
the count"*). **The count WAS re-run. It was right when written. It went
stale anyway.**

> **A field that goes wrong twice in a row under full awareness is not
> being filled carelessly. It is the WRONG FIELD.**

**Why a push count specifically fails BOTH of DL-062's constructive
clauses:** its subject is **the one tree no agent may touch** (pushing is
the operator's act, rule 9), so it changes exactly when nobody here is
looking; the interval it must survive is **the longest in the project**
(written at session end, read at the next session's start — which is when
the operator pushes); and ★★★ **nothing in any handoff DEPENDS on it** —
pushing is unauthorised regardless. **No decision, highest staleness
risk: the worst possible ratio.**

**THE DECISION (DL-068): a handoff records the two REF HASHES, not a
derived count.**

```text
local  refs/heads/master           0a88ad6…
remote refs/remotes/origin/master  0a88ad6…   (as of the last fetch/push this repo observed)
```

★ **A hash is a dated READING** — when the world moves it becomes
*historical* and says so by not matching what the reader finds. **`4` and
`0` look identical in authority.** Same move as NC-264/NC-265 surviving a
concurrent correction by being dated measurements with commit anchors.

**Three cheap supporting rules:** state the **method** beside the hashes
(loose ref files vs `git rev-list --count` — they fail differently);
**never write *"nothing is pushed"* as a standalone sentence**, which is
the form both instances decayed into; and **add to the grep list**:
`ahead of origin`, `NOTHING IS PUSHED`, `unpushed`, `commits this
session`.

★★ **Caveat that must travel with the hashes:** `refs/remotes/origin/master`
records **what the last fetch or push observed**, **never** the remote
server's present contents. It supports *"`rev-list --count` returns 0"*;
it supports **no** claim about publication (rule 9).

★ **DL-068 does NOT build the mechanical guard DL-062 imagined.** It
removes **one recurring carrier** by changing what gets written. A
pre-commit phrase-grep for cross-role status claims is still unbuilt and
unowned, and must not be read as done.

Related: [[iccce-pass-status]],
[[iccce-artifact-existence-is-not-obligation-status]] (an artifact's
existence is not the obligation's status — this is its twin: a
*document's* claim about status is not the status),
[[iccce-stale-citation-worse-than-stale-number]],
[[iccce-gate-must-not-reward-deletion]].
