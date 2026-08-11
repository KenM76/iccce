---
name: project-lcms2-findings-pass5-bpc
description: Pass 5 (BPC) findings — lcms2 silently drops BPC below a 0.002 IsEmptyLayer threshold (~0.41 L*, a constant not in the corpus); the v4 matrix-shaper escape makes forced BPC cost exactly zero; and the structural fact that iccce's and lcms2's black-point ESTIMATORS cannot be discriminated by any scenario in reach.
metadata:
  type: project
---

All measured **2026-08-11** at pin `21c582a`, iccce at commit `46f16e8`. Full
record: `tools/difftest/README.md` **§16**; tolerances `docs/TOLERANCES.md`
**§3.5**; method lessons **§6.5** there. Suite after Pass 5:
`pass=90 fail=0 skip=3 error=0` (Pass 5 contributes 21 graded + 5
reported-only).

**★ THE HEADLINE IS A NEGATIVE RESULT, AND IT WAS PREDICTED BEFORE THE RUN.**
*Everywhere iccce will do BPC at all, lcms2's estimator reduces to the same two
values* — zero on a matrix/TRC or gray side (its darkest-colorant estimate is
device black through the profile, and every TRC in the corpus has `trc(0)=0`),
and the A41 triple on a v4 LUT side at perceptual (its guard 3 constant is the
one iccce hard-codes). **So Pass 5 grades the SCALING MAP, the DIRECTION and
the POLICY. No row discriminates the two ESTIMATORS.** lcms2's methods 3 and 4
(ink round trip, least-squares quadratic fit) are untested against anything.
Closing that needs a synthetic v4 **LUT** fixture with a **non-zero** device
black — none exists.

**1. lcms2 silently performs NO BPC below a threshold — a constant not in the
corpus.** `cmscnvrt.c` L327–348 `IsEmptyLayer` sums `Σ|m−I| + Σ|off|` (offsets
already ÷ `MAX_ENCODEABLE_XYZ`) and `AddConversion` inserts the stage only if
that is **≥ 0.002**. Solving: **lcms2 stops doing BPC once the two blacks are
within ≈0.41 `L*`.** iccce has no such threshold. `ICC_Spec` §7.2's list of
unattributed constants does **not** contain it — that list came from
`cmssamp.c`, this is in `cmscnvrt.c`. **READ, not RUN** (no pair in reach
triggers it; the S2/S3 map sits at 0.015342, 7.7× the threshold).

**2. Corpus trap T5 measured: forced BPC on a v4 matrix/TRC destination costs
EXACTLY ZERO.** Guard 3's matrix-shaper escape returns `XYZ(0,0,0)`, equal to
the source's, so `ComputeConversion`'s `BlackPointIn != BlackPointOut` test
fails and no stage is inserted. Anyone expecting M2's ≈3.15 `L*` on *every* v4
perceptual profile reads this correct null as an anomaly.

**3. The policy difference measured, and D11 answered with its sign.** iccce
never forces; lcms2 forces for a v4 **destination**. Same pair, neither side
asked: **3.1373e-2 device = 3.137348 `L*`**, lcms2 lighter — against the PRM
black's 3.137254 and the A41 triple's 3.137238, a match to 1.1e-4. **The sign
matches lcms2's M2 route, NOT iccDEV's**, and the two are distinguishable
because in the v4-source→v2-dst direction iccDEV would map PB **down** while
lcms2 does nothing unless asked — which is what the measurement showed.
**REPORTED, NOT GRADED**; settled only by AdobeBPC.pdf / WP40 / ISO 18619.

**4. A41 priced in a pipeline, and both corpus figures corroborated.** Rust,
through a fixture's stored bytes: **ΔL* 0.005364** (corpus 0.0053) and **ΔE76
0.037416** (corpus 0.037437) — agreeing to 2e-5 by an independent route. The
**ΔE2000 is new at 0.050201**, which is the *same order as that section's whole
agreement budget*: on a **float** path the choice of digits is NOT negligible
against measurement noise. Complement, not contradiction, of "invisible at
16-bit".

**Method lessons that earned their keep (also TOLERANCES §6.5):**

- **When two implementations agree, ask what they were free to disagree about —
  and answer it from their sources, not from the size of the residual.**
- **A tolerance may be an earlier Pass's computed envelope × a stated
  correction, provided the derivation names the term it INHERITED and the run
  then prices it.** BPC adds no quantisation (one matrix stage between stages
  already there) but moves the operating point; predicted gain 1.0035, observed
  1.097 — the flagged term is real and worth ~9.4 %, and the envelope still
  bounds it because the inherited maximum was taken over the whole axis.
- **A direction test that reads the same in both directions is not a direction
  test.** "PCS rises" and "PCS falls" both become "no device component may
  rise" once the destination is CMYK. Grade the sign for free (`out − in =
  (Xd−Xs)/(Xi−Xs)·(Xi−X)`, no tolerance needed); grade the **magnitude against a
  closed form** to mean anything.
- **Print the sensitivity ratio with every agreement claim.** Pass 5's two
  cross-checks are **388×** and **682×** more sensitive than the effects they
  grade. A comparison that cannot state one has not shown it could have failed.
- **A refusal is graded, and the needle must be the EXACT wording.** A loose
  needle (`"refused"`) lets an ICC-absolute row pass on an estimation-subset
  refusal. One Pass 5 row failed on a paraphrase taken from the error *variant
  name* rather than its `Display` text.

Related: [[project-lcms2-findings-pass4b-direction-dependence]],
[[project-lcms2-findings-legacy-lab-and-forced-bpc]],
[[project-oracle-and-tolerance-state]],
[[project-synthetic-fixture-corpus-and-gp001]].
