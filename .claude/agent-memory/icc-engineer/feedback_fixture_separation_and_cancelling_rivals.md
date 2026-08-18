---
name: feedback-fixture-separation-and-cancelling-rivals
description: A zero-separation fixture manufactures a false PASS, not merely an uninformative one; and a named rival that appears twice in a chain can cancel and be invisible to the row naming it
metadata:
  type: feedback
---

Two testing-methodology rules, both learned the same day (2026-08-17,
Pass K §F) and both generalisable well beyond colour.

## 1. A zero-separation fixture MANUFACTURES a false pass

**Before accepting any fixture, compute the distance between the two
candidate answers it is supposed to distinguish. If that distance is
zero, the fixture does not merely fail to inform — it reports GREEN for
a capability that does not exist.**

**Why:** `fixtures/synthetic/v2-cmyk-mft2-lab.icc` has a `B2A0` that
emits `[0,0,0,k]` at every node, so its K-only ramp is K-only *already*.
`icc-conformance` injected that construction into the replacement fixture
and measured the consequence: the headline red row **went green at
`0.000000`**, and the transition-width row reported `0.25` — *a number
that looks like a working feature*. Only the two rows grading the
fixture's own bytes caught it. **A corpus-free CI run would have gone
green with black preservation unimplemented.**

**How to apply:** grade the separation as its own row with a floor
declared in advance, rather than trusting a classifier to flag it — the
`ZERO-SEPARATION` verdict would have printed in a column *beside a green
row* in a run whose summary said `fail=1`, and nobody reads a column.
This is the concrete answer to "why grade what the classifier already
labels".

## 2. Before naming a rival, ask whether the defect CANCELS

**A candidate separation is only real if the named defect would actually
change the number. If the defect enters the chain twice with opposite
sign, a symmetric misreading round-trips to identical values and the row
is blind to the very rival it names.**

**Why:** two Pass K §F rows first named DL-005's legacy-vs-general
PCSLAB misreading as their rival. It is **invisible to them** — the
derivation works in encoded fractions end to end, so a consumer applying
the general rule in *both* legs cancels exactly. The agent caught its own
claim and replaced it with a rival that does not cancel (clause 10.10's
CLUT index order read backwards), then **evaluated** it from the
committed bytes rather than asserting it: `4.84e-1`, some 31 000× the
row's bound.

**How to apply:** for every named rival ask *"where does this defect
enter, and does it enter again?"* Round trips, encode/decode pairs and
symmetric transforms are where cancellation hides.

★ Both rules are instances of the project's standing trap — a test that
certifies the bug it was written to catch. See
[[feedback-check-sources-before-accepting-corpus-gap]] for the
same lesson applied to research rather than to fixtures.
