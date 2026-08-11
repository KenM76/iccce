---
name: iccce-verification-loop-runs-both-ways
description: On 2026-08-11 an iccce unit test caught an arithmetic error in the ICC_Spec corpus itself — a corpus "DERIVED" value is a calculation someone did, not a sourced fact
metadata:
  type: project
---

**The corpus is not automatically the more reliable side of a
disagreement.** Pass 1's D50-chromaticity consistency test failed on
first run. Per project rule 5 the arithmetic was checked before the code
was blamed — and the **corpus** was wrong.

`ICC_Spec\cie\cie__ref__colorimetry_core.md` derives D50's chromaticity
as `0.34567 / 0.35850`. Those are the chromaticities of the
**high-precision** D50 (0.96422 / 1 / 0.82521), not of the **4-figure ICC
triple (0.9642 / 1.0000 / 0.8249)** that the same file instructs the
project to use everywhere. Correct derivation: `0.9642 / 2.7891 =
0.345703`, `1 / 2.7891 = 0.358539`.

**Why:** the corpus committed the **mixing-precision trap that the same
section warns about**. The document warning about the trap fell into it.

**How to apply:**

- **A corpus value marked `DERIVED` is a calculation somebody did**, with
  the same error rate as any other calculation. It is a different kind of
  object from a value marked sourced/`primary_spec`, and it does not
  inherit the file's authority. Check the arithmetic before treating a
  derived number as an expectation.
- **When code and corpus disagree, rule 5's order still applies** — code
  first, expectation second — but the expectation genuinely can be the
  wrong one, and finding that is a *finding worth filing*, not an
  embarrassment to smooth over. Filed as `NUMERIC_CLAIMS.md` §3.4 and in
  the Pass 1 `SESSION_LOG.md` entry.
- **Corpus fixes are dispatched, not assumed.** A parallel dispatch went
  to `icc-spec-librarian`; **the erratum was still present when Pass 1
  was filed** *(verified by grep)*. Never record "a parallel dispatch
  fixed it" — record that one was sent, and re-check.
- `icc-librarian` must **not touch the corpus** — it is
  `icc-spec-librarian`'s. Report, do not repair. (Same shape as the
  project's own parser invariant.)

Related: [[iccce-pass-status]].
