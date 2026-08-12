---
name: iccce-negative-finding-removes-its-auditor
description: DL-042 — a wrong REJECTION survives indefinitely because nobody re-tests a fixture they were told is broken; when an item has been owed for many cycles, re-audit the REASON it is owed, not just the item
metadata:
  type: project
---

**Rule (binds every `§7.x` status pass in `NUMERIC_CLAIMS.md`): an item
restated as owed across many filings gets its REASON re-read, not merely
its status re-stated. Restating a blocker is not re-testing it.**

**Why.** 2026-08-12: the eleven-filing *"no published ground truth for
any transform"* gap turned out to be **partly a corpus defect (C5)**.
The ICC.1:2022 Annex D data had been examined months earlier and
**REJECTED** — by **point-evaluating values that are intervals** (a 4-dp
display is an interval, not a point), which turned one typo into two and
condemned the whole black row. It was usable all along.

★ **The failure mode is specific to NEGATIVE findings.** The corpus's
four previous defects were all wrong **assertions** and **all were
caught within days** — an assertion gets used, and using it exposes it.
**This was a wrong REJECTION and it survived indefinitely: nobody
re-tests a fixture they have been told is broken.** A negative finding
**removes the very traffic that would have audited it**, and each
restatement reads as another confirmation when it is the same
observation, copied.

**And the librarian's share of it: eleven consecutive filings restated
the blocker and NONE re-read the entry that created it.** The corpus
published a finding; this ledger **propagated** it.

**How to apply.**

- After roughly **five restatements**, a `§7.x` line must either
  **re-read the source of the blocker** or record explicitly that it did
  not.
- **Distinguish *"nobody has done it"* from *"somebody looked and said
  no"*.** The second has a **finding** underneath it — with an author, a
  date, a method, and the possibility of being wrong.
- **When filing a rejection, record its METHOD.** *"Examined and
  unusable"* is unauditable; *"examined by point evaluation of the
  printed 4-dp values"* is the sentence that lets the next reader find
  the error.
- ★ **This ledger's own refusals are the same shape** — every
  `no-named-alternative`, every `REPORTED, NOT GRADED`, every *"not
  comparable at this intent"* skip is a negative finding that removed its
  own auditor. Their reasons must be **stated** (DL-039) *and* **re-read**.
- Related mechanical rule from the same incident: **a displayed value is
  an interval.** Re-running a point evaluation at higher precision is
  **not a second pass — it is the same pass.**

**Companion incident, same day, opposite direction:** this ledger's own
*"the corrected 4.2.5.4 clause is undefended"* — quoted as "the most
important sentence in the filing" — was **false**, and the tension was
visible **three lines below the heading in its own table**. ★ **A
claim's prominence is not evidence about it.** See
[[iccce-verify-own-draft-too]].

Related: [[iccce-ground-truth-cannot-exist]] (the item this was found
on), [[iccce-count-needs-its-apparatus]], [[iccce-pass-status]].
