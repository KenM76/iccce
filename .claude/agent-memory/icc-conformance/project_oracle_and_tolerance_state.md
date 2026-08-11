---
name: project-oracle-and-tolerance-state
description: iccce Pass 0 — the lcms2 oracle is pinned/built/smoke-tested (Windows only) and docs/TOLERANCES.md exists as a skeleton with zero tolerances set; what is deliberately still open.
metadata:
  type: project
---

As of **2026-08-11**, the `icc-conformance` half of ROADMAP Pass 0 is
stood up but deliberately incomplete:

- **The oracle exists.** lcms2 pinned by commit in
  `tools/difftest/lcms2.pin`, fetched by `fetch-lcms2.sh` into a
  git-ignored `vendor/`, built on Windows/MSVC by `build-lcms2.ps1`.
  `transicc.exe` has been demonstrated on real profiles. Evidence is in
  `tools/difftest/README.md` §6 and §8.
- **No tolerance has been set, and that is correct.**
  `docs/TOLERANCES.md` is a skeleton: every numeric cell is blank
  because no comparison between iccce and anything has been made — iccce
  does not compute colour yet. The one seeded entry is the 1.0 ΔE2000
  perceptual *anchor*, which is a yardstick, not a tolerance, and it
  carries a ⚠ PROVISIONAL flag.
- **Still open:** the Rust difftest harness (nothing drives `transicc`
  programmatically), the POSIX build (written, never run — no C
  toolchain on this machine), Linux CI, and the synthetic fixture
  corpus.

**Why:** ROADMAP Pass 0 requires the oracle before Pass 1's colorimetry,
and rule 5 forbids inventing tolerances for comparisons not yet made.
Getting the oracle credible early is what makes every later "verified"
claim mean something.

**How to apply:** when asked to "add a tolerance", first check whether
the comparison has actually been *run*. If it has not, the honest answer
is a blank row, not a plausible number. When a Pass completes, add its
measured tolerances to §3 with a date and a justification, and record
coverage scope in §6 — `docs/TOLERANCES.md` §6 is where "verified on N
profiles, M intents, on which platform" lives.

Related: [[project-lcms2-licence-is-not-uniform]],
[[project-doc-editing-conventions]].
