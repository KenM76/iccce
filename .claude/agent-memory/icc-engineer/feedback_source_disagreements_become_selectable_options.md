---
name: source-disagreements-become-selectable-options
description: When authoritative sources disagree on a colour constant, expose BOTH as runtime options with a reasoned default — never silently pick one
metadata:
  type: feedback
---

**Standing instruction from Ken, 2026-08-19:** *"if there are source
disagreements then you'll make both choices an option, with the default
being your best guess of which one is correct."*

**Why:** picking one reading and discarding the other destroys the finding.
A user whose output disagrees with another tool then has no way to discover
*why*, and this project's core hazard is that a wrong colour looks exactly
like a right one. Keeping both readings turns an unresolvable documentation
conflict into something a user can test against reality.

**How to apply — five obligations, and the third is the one that makes this
worth doing:**

1. **Both variants ship.** Named, documented, selectable. Neither is
   deleted, and the non-default is not a hidden debug flag.
2. **The default is an ARGUMENT, not a coin flip.** Record *why* that
   reading was judged more likely correct — source authority, source
   independence, internal consistency, agreement with the ICC's own
   documents. A default with no recorded reasoning will be flipped by a
   future session on equally little reasoning.
3. ★★★ **MEASURE THE ΔE BETWEEN THE VARIANTS.** This is the number that
   says whether the disagreement matters at all. Two sources differing at
   `1e-9` is a curiosity to be documented and forgotten; two sources
   differing at `2 ΔE2000` is a real fork with real consequences for a
   customer's press. **Without this number nobody can tell which kind they
   are looking at**, and the whole option becomes noise. File it in
   `NUMERIC_CLAIMS.md`.
4. ★★ **Every ΔE claim in the ledger is measured under ONE variant.**
   Selecting the other silently invalidates them. So a non-default
   selection must be **disclosed in the output/diagnostics**, not just
   accepted — otherwise the option becomes a silent-wrong-answer surface,
   which is precisely the failure mode this project exists to prevent.
5. **The non-default must be tested too**, or it rots into a trap: an
   untested option that a user selects is worse than no option.

★ **The unexpected payoff, and it is large.** Once both variants exist, the
lcms2 oracle can be run against *each* — which **measures which reading
lcms2 implements**. That converts an unresolvable documentation
disagreement into an empirical fact about the ecosystem, and it is
something the paid standard would not have told us. A disagreement is
therefore not a defeat; it is a measurement opportunity.

Related: [[project_no_paid_standards_use_reconstruction]] — this rule is
what makes that decision safe, because it means the reconstruction never
has to pretend to a certainty it does not have.
