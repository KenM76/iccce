---
name: iccce-measurement-under-the-guards
description: DL-038/DL-040/DL-043 — the separation DISTANCE must be a property of the FIXTURE not the RUN (it printed ZERO-SEPARATION on a row that was failing); a big separation on an UNGRADED row buys a fixture, not a bound; an exemption is declared, never acquired
metadata:
  type: project
---

**Three rules from 2026-08-12 about instruments that fail while looking
fine. All three are about the layer BELOW the thing everybody watches.**

### ★★★ DL-038 — the distance is a property of the FIXTURE, not the RUN

`Separation::against` derived `distance = |observed − alt_observed|`.
**Where the alternative is *"the code returns the other candidate"*,
`observed` BECOMES `alt_observed` on the defect run** and the distance
is exactly `0`. So on the proof-of-power run the new 4.2.5.4 clause row
**failed at `2,500 019×10¹` against a `7,629 511×10⁻⁴` bound AND printed
`ZERO-SEPARATION` beside it** — the mechanism disclaiming its power in
the instant it demonstrated it.

**Why it matters more than an ordinary bug:** **DL-037** recorded the
guard *order* as the design; this records that **the MEASUREMENT
UNDERNEATH the guards had the defect the guards were built to catch.**
The guards could not have caught it — they classify the number they are
handed, and the number was `0`.

★ **How it was found: by USING the instrument on the very case it was
built for, one filing after it was celebrated.** Re-reading the code
would not have found it; the formula is correct in every state except
one, and that state does not occur while the suite is green.

**The diagnostic, now on the constructor's own doc comment
(`tools/difftest/src/lib.rs`): is the distance a property of the RUN or
of the FIXTURE?** A separation between two *candidate answers* is the
fixture's (`against_distance`, caller supplies it); `against` is right
only where the rival is a different **reading applied to the same
observation**, so the two genuinely coexist. **Three rows were moved —
that is a count, not an inventory; NC-176…NC-178 were not re-audited.**

### ★★★ DL-040 — a large separation on an `UNGRADED` row buys a FIXTURE, not a bound

Asked whether NC-176's `4,717 441` separation now justifies grading that
row: **NO.** (1) No clause requires two implementations of two different
documents to agree, so **nothing for a bound to mean**; (2) any bound
below `4,717 441` is **fitted to one known defect**; (3) it could not be
one number anyway — the three arms observe `4,799` / `5,000` / `10,000`.

> **Test: ask what clause the number would be graded against. If the
> answer is *"none, but it would have caught the bug"*, the bound is
> fitted to the bug.**

★ **The constructive half is what actually happened:** the request was
honoured with a **fixture** whose expectation is an authored constant
put through a clause, bound = **half one PCSLAB quantum and nothing
else**. **The bug is now caught by a row that would exist even if the
bug never had.**

### ★★ DL-043 — an exemption is DECLARED IN ADVANCE and GRADED, never acquired

The third arm made a control (`apparatus/error-bar-is-smaller-than-the-effect`)
fail at **`3,775×10⁹`**, because the fixture's floor makes
`d(device)/d(L*)` zero by construction (`1,11×10⁻¹⁶`). **`APPARATUS_RATIO`
was NOT widened** — an authored `DEVICE_OBSERVABLE` table declares which
arms the conversion exists on, a further row **grades the measurement
against the declaration**, and a two-way test keeps the table's arms and
the runner's arms in step.

> ★ **An exemption acquired by a measurement coming out small is
> indistinguishable from a defect coming out large.** Widening the
> constant to `4×10⁹` would have been green, reasonable-looking, and
> would have destroyed the control **on every arm, forever**.
> **DL-018 in a new position: the cheapest route to green must never be
> the one that removes the evidence.**

**How to apply.** When a filing carries a new instrument: ask what the
instrument would print **on the run where the defect is present**, not
just on the green run. When a control fails on a new arm, the two
admissible moves are **declare the exemption** or **fix the arm** —
never widen the constant. When a separation is large on a row nothing
grades, file it as **owed fixture work**.

Related: [[iccce-disclosure-is-not-enforcement]] (DL-037, the guards),
[[iccce-agreement-can-be-the-symptom]] (DL-033, the rule underneath),
[[iccce-control-only-as-good-as-its-fixture]],
[[iccce-gate-must-not-reward-deletion]], [[iccce-pass-status]].
