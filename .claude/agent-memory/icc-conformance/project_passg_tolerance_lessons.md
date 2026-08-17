---
name: project-passg-tolerance-lessons
description: Four tolerance lessons from Pass G — a tolerance may need to be a FUNCTION of the fixture (a constant failed); a gate derived for one direction is not a gate in the other; the separation mechanism found a defect nobody was looking for and a second one that was invisible until the first was fixed; and the injection proof pattern that validated all of it.
metadata:
  type: project
---

**Learned 2026-08-17 building Pass G** (`tools/difftest/src/passg.rs`). Full
record `docs/TOLERANCES.md` §3.7.2, §3.7.3, §3.7.4, §3.7.6, §6.6 and the §4
change log. These generalise beyond Pass G.

**1. ★★ A tolerance may need to be a FUNCTION of the fixture, and the tell is a
derivation containing a clause about the fixture's CONTENTS.**

The corner tolerance's first draft was a constant `1e-3` justified in part by
*"the 2-entry B curves are affine"*. That clause is true of both tags in the
X-Rite v4 profile and **irrelevant**; the property that mattered was *"the exact
identity `(0x0000,0xFFFF)`"*, which is true of `A2B1` and **false of `A2B0` in
the same file**. lcms2 evaluates a non-identity 2-entry `curv` through
`cmsEvalToneCurve16` and rounds twice where iccce uses `f64` — one lsb =
`1.526e-3` in `L*`. **The `A2B1` arm passed at the print floor; the `A2B0` arm
FAILED at `1.111856e-3`.**

**How to apply:** when a `why` string asserts something about the fixture, ask
whether the code can *read* it instead. `passg::corner_tolerance(bool)` selects
from the tag's own bytes at run time; a constant would have had to be the looser
value everywhere, weakening the arm that did not need it. A run-time-selected
tolerance also cannot go stale (DL-034) and prints its own premise on the line.
**Injection I3 (force the identity branch) turns exactly that one row red** —
that is how you show a selection is load-bearing rather than decoration.

**2. ★★ A gate derived for one DIRECTION is not a gate in the other, however
small its number.**

`SWEEP_DEVICE = 4e-3` is defensible for `B2A` because `_cmsReadOutputLUT` forces
trilinear for a Lab-PCS output LUT and the interpolation-method envelope is
**identically zero** there. Reusing it on the `A2B` end-to-end rows — where that
envelope is the **dominant** term — put a bound on the table that omitted the
biggest term in it. Three rows correctly failed at 8.98e-3–1.49e-2. Remedy:
propagate the envelope through the **actual destination model point by point**
(Pass 4's method, reused). Direction is part of a tolerance's identity exactly
as it is part of a finding's.

**3. ★★★ The separation mechanism found a defect nobody was looking for, and a
second one that was invisible until the first was fixed.**

`BLIND` fired on `passg/authoring/ecirgb-v2/...`. Correct: that profile's `wtpt`
and colorant sum **agree**, so the "rival reading" was manufactured — there is
nothing out of step to attribute. Fixing it (→ `NO-NAMED-ALTERNATIVE` with its
reason, and that row is now the section's **negative control**) exposed the real
defect: the `2e-4` encoding-floor justification **did not hold for the profiles
it was actually gating** — Ghent's sRGB colorants sum to the PCS white to
`1.885e-4`, ≈12 `s15Fixed16` lsb, because the *published* sRGB primaries do not
sum to D50 to the lsb. **The row had been passing inside a bound its own
justification could not support.**

**How to apply:** a separation is not only a statement about power — it is a
**second, independent reading of what the row believes it is testing**, and the
two disagreeing is a finding. Also: the replacement bound asks a *classification*
question with no free parameter (*is the colorant sum nearer the normative PCS
white or nearer the profile's own encoded `wtpt`?*, tolerance = half the distance
between the file's own two candidates) and **imports no third white point** —
D65 in particular, whose constant `NEXT_SESSION.md` §0 records as the weakest in
`iccce-color`. Never put a weak constant underneath a conformance claim.

**4. The injection pattern that validated all of it** — four injections, each in
a **detached `git worktree`**, reverted before the next, baseline reproduced
first:

| what | expect |
|---|---|
| corrupt iccce's v4 PCSLAB decode (255→254) | apparatus + end-to-end red; **the PCS rows stay GREEN**, because they compare the HARNESS to lcms2 — the apparatus row is the only link to iccce |
| transpose `rXYZ`/`gXYZ` | trap rows red **at exactly the stated separation, to six figures**; self-consistency rows stay green because they are blind to a *symmetric* defect by construction |
| defeat a run-time tolerance *selection* | exactly one row red |
| defeat the geometry substitution in the harness arm | the agreement rows red **and their separations stay `DISCRIMINATING`** — proof that `against_distance` (not `against`) was used |

★ **A separation that predicts the magnitude of the injected failure is doing
the job DL-033 defined for it.** Injection 2 produced `0.472229` and the row's
stated separation was `0.472229`.

★ **State the separation in the ROW'S OWN METRIC.** A ΔE separation on a row
graded in XYZ prints `INCOMMENSURATE` and the blindness test is skipped. Compute
the distance in the metric as well; keep the ΔE in the detail text for a human.

Related: [[project-passg-ghent-population-findings]],
[[project-candidate-separation]],
[[project-prove-the-arm-by-injecting-the-defect]],
[[project-parallel-agent-build-collisions]].
