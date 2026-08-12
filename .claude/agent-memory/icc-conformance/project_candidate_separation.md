---
name: project-candidate-separation
description: Every difftest Record now emits a candidate separation (how far the named rival answer sits) with an auto verdict — BLIND / ZERO-SEPARATION / UNGRADED; on its first run it caught a fourth stale literal and showed the row carrying the 4.2.5.4 finding is UNGRADED.
metadata:
  type: project
---

**Built 2026-08-12 on `icc-engineer`'s dispatch, out of DL-033
([[project-conformance-can-worsen-the-crosscheck]]).** `tools/difftest`'s
`Record` carries a `Separation`; two TSV columns (`separation`, `sep-power`)
plus a `separation` aggregate line. The `summary` line is deliberately
byte-unchanged — it is quoted as a run's signature.

**The rule the mechanism encodes:** a cross-check's power is bounded by the
distance between the answer it observed and the answer it would have observed
under a **named** rival reading. Not the tolerance, not the kind — the
separation.

**How to apply.**

- **Three states, and `Unstated` is not `Option::None`.** "Nobody looked" and
  "somebody looked and there is no rival" are different claims about the
  evidence. 129 of 145 rows are `UNSTATED` and that is the intended state —
  **do not fill them with plausible-sounding rivals**; a named alternative a
  reader cannot go and check is worse than none.
- **Verdict order matters and each guard is an argument.** `ZERO-SEPARATION`
  outranks everything (a row whose candidates are one number cannot move at any
  tolerance); `UNGRADED` is checked *before* the comparison, because `d ≤ ∞`
  would label every REPORTED row `BLIND` and blame the fixture for a decision
  the tolerance made; the blindness test runs only in `SepUnits::SameAsMetric`.
- **A flag is never a failure.** Auto-failing `BLIND` would create pressure to
  stop stating separations at all.
- ★ **The row that carried the whole 4.2.5.4 finding is `UNGRADED`.**
  `pass5c/*/estimators/black-points-in-lab` is REPORTED (tolerance ∞), so its
  4.717441 separation could never have failed anything. **The suite's power on
  that question lives entirely in §B's device rows**, not in the row whose name
  says "estimators". Look for this shape elsewhere.
- ★ **A fourth stale literal, found by the apparatus, not by a person.**
  `SHIPPED_MATCHES_LIBRARY` asserted the candidates were "2.46e-3 apart, three
  orders above the bound"; computed, it is **9.574e-3** (2.46e-3 was the
  *pre-`fd34a44`* figure) and the margin is four orders. **The argument was
  never harmed — only the number was**, which is exactly why nobody notices a
  stale literal in a justification: it usually misstates a margin that is still
  fine. Also replaced a literal that was still *true* (`0.834`/`5.0` in
  `NEUTRAL_EXACT`) because both are properties of *which fixture is loaded* —
  a third arm would have falsified the sentence without anyone touching it.
- **Coverage, stated: 16 of 145 rows, all Pass 5c's.** `blind=0` is out of 16,
  not out of 145. Pass 4c is the obvious next candidate — its §A was built so
  lcms2's `wtpt` substitution *cannot* fire, and "it fired" is a named
  alternative with a computable value.
  ★ **DONE 2026-08-12: 41 of 160** (`pass=157 fail=0 skip=3`;
  `unstated=119 no-named-alternative=12 incommensurate=3 ungraded=8
  zero-separation=2 blind=0 discriminating=16`). Pass 4c: **4 `Measured`, 6
  `no-named-alternative` with reasons** — the named candidate came out at
  `2.05576e-1` against a `5e-4` tolerance, `DISCRIMINATING` 411×. Two rules the
  Pass 4c pass produced: **a rival TOLERANCE is not a rival CANDIDATE** (the
  sensitivity-floor row's only "alternative" is a different floor, which belongs
  in `why`), and where several real rivals exist, **name the one that most
  threatens the row and enumerate the others in its text** — picking the
  flattering one is the tuning the mechanism exists to prevent.
- ★★ **`Separation::against` is unsafe when the alternative is "the code
  returns the other candidate"** — see
  [[project-prove-the-arm-by-injecting-the-defect]] §1.
- **It cannot generate alternatives.** The 4.2.5.4 rival was identifiable only
  because somebody read the clause and saw two candidate return values.

Filed in `tools/difftest/README.md` §20 and `docs/TOLERANCES.md` §1.1.

Related: [[project-stale-claim-strings-in-emitted-records]],
[[project-synthetic-fixture-corpus-and-gp001]],
[[project-pass5c-estimator-branch-finding]].
