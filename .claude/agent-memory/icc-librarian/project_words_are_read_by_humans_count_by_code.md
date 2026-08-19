---
name: iccce-words-humans-count-code
description: DL-063 — the emitted STRINGS were already careful and that is WHY the mis-designed channel survived; only the COUNT crosses into a caller's control flow, and two files with identical bytes give the same count where one is conformant
metadata:
  type: project
---

**`malformations: N` counts DISCLOSURES, not violations — and the reason
nobody caught the falsified doc comment is that the part a REVIEWER
reads was scrupulous while the part a PROGRAM reads had no room for the
disclaimer.**

**Why:** filed 2026-08-19 as **DL-063**
(`docs/ARCHITECTURE.md` §5), drafted by `icc-engineer`. The
`Malformation` type's doc comment said *"a rule violation the file
carries"*; **two of its own variants carry no violation**:

- `TrailingBytes` — its emitted text already said *"(normal for
  container-embedded profiles)"*. A profile embedded in a PDF stream or
  an ICC-tagged image routinely has bytes past its declared size, and
  **ICC.1 is not breached by a caller passing a longer buffer**.
- `UnknownRenderingIntent{IntentRule::V2Undefined}` — its emitted text
  already said ICC.1:2001-04 6.1.11 / Table 18 *"do not forbid others"*.
  **The sentence states in its own words that nothing is forbidden.**

★★★ **The sharpest demonstration, and the thing to quote:**
`v2-rendering-intent-low-half` and `v4-rendering-intent-low-half` carry
**the same four header bytes `0x00000004`**, both print
**`malformations: 1`**, and **only one of the two files is
non-conforming.** *(Counts `[REPORTED]` by `icc-engineer` at the CLI;
this role verified the branch logic at `header.rs:190`–`:202` — v4 tests
`intent > 3`, v2 tests `intent & 0xFFFF > 3`, so a **v2 high-half value
is not reported at all**, which is why the v2 high-bits fixture is `0`.)*

**The durable reading, which belongs in any answer about
`malformations`:**

- **`N == 0` means iccce found nothing to say — NOT a conformance
  certificate.** iccce checks the constraints it has *implemented*, not
  every clause of ICC.1.
- **`N > 0` means there is something worth reading — NOT that the file
  is non-conforming.**
- **A verdict requires matching on VARIANTS** (and on the `IntentRule`
  inside `UnknownRenderingIntent`), never on `.len()`.

**The alternative recorded as REJECTED** so it is not re-litigated:
splitting into `Violation` / `Observation`. A public API break with **no
numeric benefit** that **moves** the ambiguity — an
`Observation::TrailingBytes` beside an `Observation::ReservedNonZero`
**under-states the second exactly as much as the present name
over-states the first**, and ICC.1's own language has three states
(`shall`, silence, *"reserved"*), not two. **The mixed channel is real,
so it is documented rather than renamed.**

**Two things this filing found by reading that the dispatch did not
carry** — both in `icc-engineer`'s tree, **reported not fixed**, and
both worth re-checking before assuming the retraction is complete:
`diag.rs:14` (module header still says *"the file violates a rule"*) and
`iccce-cli/src/main.rs:237` (*"everything the file got wrong"*).
★ **A retraction is not complete while the retracted sentence still has
copies.**

★★ **And the fact was already correct in a THIRD document the whole
time:** `fixtures/synthetic/MANIFEST.md:312` says *"THIS FILE VIOLATES
NOTHING"* and names the cost exactly. **The filing did not discover the
finding; it filed it** — nothing in this project's workflow reads a
fixture manifest against a type's doc comment, which is
[[iccce-documented-is-not-tested]]'s gap on a new axis.

**How to apply:** never quote `malformations: N` — or any count — as a
conformance statement, and when reviewing a disclosure feature, **read
the type and the count, not just the emitted string**. Care applied at
the layer a reviewer reads does not transfer to the layer a program
reads. Related: [[iccce-disclosure-is-not-enforcement]] (a field that
gates nothing), [[iccce-count-needs-its-apparatus]] (a bare count is not
a claim), [[stale-citation-worse-than-stale-number]] (this entry
discharged a forward reference from `diag.rs:147`–`:150` that had no
destination), [[iccce-pass-status]].
