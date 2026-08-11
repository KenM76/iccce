---
name: iccce-predicted-divergence-must-be-measured
description: A predicted disagreement with another implementation is a prediction, not a finding — iccce's DL-011 predicted an lcms2 divergence that DL-012 then measured absent, and the measurement turned up a bigger unrelated one
metadata:
  type: project
---

**A disagreement inferred from reading two texts is not a divergence. It
becomes one only when it is measured — and it may not survive.**

**Why:** on 2026-08-11 `ARCHITECTURE.md` **DL-011** recorded that iccce's
tag-type-based legacy-PCSLAB selector put it *"in a live disagreement
with lcms2"*, on the corpus's claim that lcms2 keys off the profile
version. DL-011 was careful — it labelled the claim unverified and named
the difftest that would settle it. When `icc-conformance` ran that probe
(four synthetic profiles, three byte-identical except the version word),
**lcms2 keyed off the tag type too**: the divergence did not exist at the
pin, and `cmsio1.c` has no version test on that path. **DL-012**
supersedes only the disagreement clause; DL-011's rule, which came from
the specification text, was never at risk.

The same probe then found a *real* version-keyed divergence nobody was
looking for — lcms2 forcing BPC on for v4 perceptual/saturation, ≈3.15
`L*` at black (**DL-013**). The unplanned finding was worth more than the
planned one.

**How to apply:**

- **Never let a predicted divergence acquire a consequence before it is
  measured.** DL-011 had already ordered a runtime warning to be written
  for a divergence that turned out not to exist. If a filing must record
  an unmeasured disagreement, record the *owed measurement* with it and
  keep the consequence conditional.
- **When the measurement lands, file a NEW entry that supersedes the
  specific clause** — never rewrite the old entry, and never let
  "supersedes the disagreement" be read as "reverses the rule".
- **Agreement with an implementation is never the ground for a
  conformance choice.** The spec is the authority; agreement is
  `implementation-cross-check` evidence at best (rule 3, rule 7).
- **Coverage travels with the measurement.** "lcms2 keys off the tag
  type" is really `A2B0`/`mft2`/device→PCS/Lab/one intent/four
  synthetics/one pin — with `ncl2` and B2A resting on a *source reading*,
  which must not be merged into the same sentence.
- **A result matching neither hypothesis is the interesting one.**
  Refusing to round it to the nearer candidate is what produced DL-013.
  Keep inconclusive arms in the record too, labelled — a null result from
  arms that differ in more than the variable refutes nothing.
- **Findings scoped to an oracle pin are invalidated by the pin moving.**
  In iccce that was already a licence event (DL-001); it is now a
  behavioural one, and the affected ledger rows must be **re-run, not
  re-read**.

Related: [[iccce-pass-status]], [[iccce-verification-loop-runs-both-ways]].
