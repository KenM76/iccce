---
name: iccce-compatibility-not-certification
description: DL-045/046/047 — "cannot be certified" was being read as "cannot be checked"; a third-party corpus can supply a CATEGORICAL expectation with no oracle and no instrument, but never a numeric one; and a [REPORTED] claim is promoted only by re-running the shipped code WITH a control
metadata:
  type: project
---

**Certification and capability are different claims with different
evidence. An item may not be parked because certification is
unreachable, when the capability underneath it is measurable without an
instrument.** Filed as `ARCHITECTURE.md` **DL-045**, on the operator's
instruction of 2026-08-17: *"We aren't going to aim for compliance like
that. Just aim for compatibility."*

**Why:** the habit was **not** caution about hard measurements, it was a
**category error** — *"this cannot be certified"* read as *"this cannot
be checked"*, and then filed beside real blockers. **DL-041** already
separates existence / availability / access-terms; this adds a fourth
kind, **an ORGANISATIONAL fact about a certification programme**, which
is not an engineering blocker at all. The parked work turned out to be
cheap: the strongest rows it produced need no oracle, no instrument, no
published dataset and no clause transcription.

**★★★ It changes WHAT IS CLAIMED, not how well a claim must be
supported.** Rules 1, 3, 4, 5 untouched. If a future filing ever uses
*"compatibility"* as grounds for a looser tolerance, that is this entry
being misread, and the misreading is the risk it was written to prevent.

### The three things worth carrying

**1. A third-party corpus can supply a CATEGORICAL expectation — and it
is a rare, strong shape.** GWG ships **deliberately corrupted** profiles
(red/green swapped; cyan/magenta swapped) as discriminators. The correct
answer is a fact about the **file's own declared content**, so there is
no oracle to share a misreading with and no transcription to get wrong.
New evidence class **`fixture-declared-categorical`**. ★★★ **What it
licenses:** *the declared source profile is USED rather than dropped for
a device alternate* — the separation being the full gamut width, because
the named rival is what `pdfce` does today (ISO 32000-1 Table 66
`/Alternate`). ★★★ **What it never licenses: any sentence with a number
in it.** A CMM wrong by 20 ΔE2000 passes it, provided it swaps.

**2. But that corpus can never supply a NUMBER (DL-047).** The Ghent
suite states **no tolerance, no reference measurement, no expected
colour value anywhere**; its criterion is *"a clear X"* at 0.5 m and
*"A faint X is NOT a failure"*. ★ **It also contradicts itself exactly
on our topic** — GWG 13.0 flags a rendering-intent failure with a
**faint green X**, the symbol already ruled not-a-failure — so **intent
handling is untested by its own criterion** and any intent claim must be
graded against ICC.1's text. Also new: class **`acceptance`** (the code
ran on real input and did not refuse), which is **negative-capable only
if the corpus was not filtered**.

**3. Verify in the RUNNING THING, and bring the control (DL-046).** The
CMYK swap arrived `[REPORTED]` from a byte-scan and was re-derived
through the shipped binary **with a genuine profile from the same patch
as a control**. ★ **The control is what turned "the values changed" into
"exactly two channels exchanged and nothing else moved"** — without it
the observation is equally consistent with the transform mangling
everything. Six further leads from the same scan were **not** promoted.
Diagnostic: *did anything execute, and was there a second case that
should NOT have moved?*

### How to apply

- **Never let a Ghent-measured number be quoted as ground truth or as a
  justified tolerance.** It inherits the class of the apparatus.
- **Watch for a numeric aside attached to a categorical row** — that is
  how a row acquires a bound nobody derived. One arrived this session
  (*"unchanged to 3 decimal places"* for a difference of `2.455×10⁻³`)
  and was corrected in the flattering direction.
- ★★★ **No public artifact may say "Ghent" without GWG's WRITTEN
  permission** — README, release notes, crates.io metadata. Certification
  is sold to print service providers; developers are directed to a
  separate programme reachable only by contacting GWG. This is a
  claim-bearing-copy matter and an **operator** decision.
- **The `ghent-v50/` fixtures are the fourth private corpus and have the
  most restrictive terms** (no commercial use, no redistribution, an
  **affirmative** notice obligation, plus unassessed per-profile
  licences). ★ A *"yes"* to the standing operator question about
  published numbers in an MIT repo **would not extend to this one**.

Related: [[iccce-pass-status]],
[[iccce-ground-truth-cannot-exist]],
[[iccce-agreement-can-be-the-symptom]],
[[iccce-control-only-as-good-as-its-fixture]],
[[iccce-refusal-discharged-by-fixture]],
[[iccce-count-needs-its-apparatus]],
[[iccce-verify-own-draft-too]].
