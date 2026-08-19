---
name: iccce-unfailable-row-protected-only-by-siblings
description: DL-066 — the number a register carries can be one that CANNOT fail (G1 is REPORTED by design), so its whole protection is two siblings failing beside it; and a population row is coverage, not a gate — G16 is one-sided and blind to every defect that makes the policy cost MORE
metadata:
  type: project
---

**DL-066 (2026-08-19, Pass K §G): the headline number a register carries
can be one that CANNOT go red. Its protection is entirely its siblings,
and nothing in the source records that dependency.**

**The instance.** `NA-012`'s cost field now carries **`3.681203`
ΔE2000** from row `G1`. `G1` is **`REPORTED`** — an infinite tolerance —
and **correctly so**: no requirement bounds what an **opt-in** policy may
cost, and inventing a bound would look like a requirement where ICC.1
specifies none (A51 is a closed negative). ★ **The classification is
right; the hazard is that nothing marks the rows that defend it.**

Measured by injection (5 % error in `KPreserve::map_k`, detached
worktree, reverted):

| row | result | what it means |
|---|---|---|
| **`G12`** derived closed form | **RED at 62× its bound** | ★ the detector: its expectation comes from **two recipe constants**, so an injection cannot be fitted to it — and it **runs in CI, no licence** |
| **`G8`** same-press pair imperceptible | **RED at 4.0×** | the only row that would notice NA-012's *"what must not be quoted"* framing going wrong |
| `G3`, `G4`, `G5` gates | **unmoved — CORRECT** | they are statements about the **fixture pair**, not the engine; `G5` is bit-for-bit unmoved because the colorimetric leg never enters the policy |
| **`G16`** population | **GREEN, and it cannot see this** | see below |
| `G1` headline | moved, **cannot go red** | `REPORTED` by design |

**★★★ The second half: a population row is COVERAGE, not a GATE.**
`G16` grades a universal — *no pair entitled to price this policy finds
it imperceptible*, `0` counterexamples over 30 ordered pairs — and stayed
green under injection. That is its **shape**, not a defect: only a defect
making the policy cost **less** can refute it, and **every natural defect
here makes it cost more.** What it genuinely guards is a policy that
silently became a no-op.

> **A one-sided claim is a gate in one direction and documentation in the
> other, and nothing in a row's identifier, bound or green output says
> which direction it faces.**

**★★ A green suite bounds only the GRADED rows.** §G is **ten graded, six
`REPORTED`**. `pass=353 fail=0` establishes `G12 ≤ 1e-2`, `G5 ≤ 1.0`,
`G6 ≤ 0.25423`, `G8 ≤ 1.0`, `G16 = 0` and the floors — **and nothing
whatever about the headline.** *The green suite corroborates every number
in the section except the one the register carries.*

**How to apply.** When filing a number into a register from a `REPORTED`
row: (1) **name the rows that WOULD fail** if the number went wrong, and
file the dependency (this one is now a `NUMERIC_CLAIMS.md` §6 row, so
deleting a sibling is visible as an act with a consequence); (2) **do not
invent a tolerance** to make a headline failable — a bound with no source
is worse than an unfailable row; (3) **counting green rows beside a
headline is not counting its protection** — only injection distinguishes
the two; (4) for any universal/count row, **state which direction it
faces on the row itself.**

Related: [[iccce-pass-status]],
[[iccce-injection-measures-one-magnitude]],
[[iccce-documented-is-not-tested]],
[[iccce-agreement-can-be-the-symptom]],
[[iccce-disclosure-is-not-enforcement]],
[[iccce-gate-must-not-reward-deletion]],
[[iccce-gate-in-input-units-cannot-certify-output]].
