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

**★ Second instance, 2026-08-11 (Pass 4) — and this time the prediction
was about the oracle's ALGORITHM, not its disagreement.** Three
documents (NA-006, `NEXT_SESSION.md`, `ROADMAP.md`) carried *"iccce
interpolates n-linear, **lcms2 tetrahedral**"*, and the Pass 4 blocker
was filed as *"source lcms2's tetrahedral cube decomposition."*
`icc-conformance` **read `cmsintrp.c` at the pin** instead of recalling
it: for **four** inputs lcms2 runs a **hybrid** — linear in C, Sakamoto
tetrahedral in M/Y/K. Consequences none of which "tetrahedral" implies:
lcms2's scheme is **not symmetric in the four inks** (iccce's
quadrilinear is); it is **not pure tetrahedral**, so **NA-006's ~1 ΔE
bound — transcribed from the trilinear-vs-tetrahedral literature — was
not the bound that applied**; and its float path quantises to `u16` at
the CLUT boundary. Filed as **NC-056** with NA-006's dated status note.

**The generalisation, which is the reusable part:** *never write a claim
about an IMPLEMENTATION from memory either.* Project rule 2 says never
write colour maths from memory; the same day, a clause number written
from recollection (NA-003's "6.4") was found wrong and had already been
**relied on** by a differential finding. **Recollection about a spec, an
implementation, or a clause number are the same failure**, and the cost
of reading the source once is a single lookup.

Related: [[iccce-pass-status]], [[iccce-verification-loop-runs-both-ways]],
[[iccce-tolerance-cannot-swallow-and-claim]], [[iccce-verify-own-draft-too]].
