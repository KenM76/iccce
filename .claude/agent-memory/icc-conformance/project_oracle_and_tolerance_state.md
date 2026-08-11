---
name: project-oracle-and-tolerance-state
description: iccce oracle state — lcms2 pinned/built (Windows only), a zero-dependency Rust difftest harness at tools/difftest drives it, and TOLERANCES.md §3.1/§5 now carry Pass 1's numbers; what is still deliberately blank.
metadata:
  type: project
---

**Superseded the 2026-08-11 morning entry that said "zero tolerances set".**
State as of **2026-08-11 (later)**:

- **The oracle exists.** lcms2 pinned by commit in
  `tools/difftest/lcms2.pin`, fetched by `fetch-lcms2.sh` into a
  git-ignored `vendor/`, built on Windows/MSVC by `build-lcms2.ps1`.
  POSIX build script still never run.
- **A harness exists.** `tools/difftest/` is now a standalone Rust crate
  (**not** a workspace member — it carries an empty `[workspace]` table;
  the root manifest is untouched). Zero dependencies, `std` only. It
  drives `transicc` as a subprocess, parses stdout, grades against a
  stated tolerance, emits TSV. **One registered check**, and it compares
  lcms2 to lcms2 (`Kind::OracleReproducibility`).
- **`TOLERANCES.md` §3.1 and §5 are filled**; §3.2–§3.6 are still blank
  and correctly so. Exactly **one** row in the whole document is a
  correctness claim (ΔE2000 vs the 34 Sharma pairs, 1×10⁻⁴). The other 16
  are arithmetic identities that cannot detect a consistently wrong
  constant.
- **Still open:** the POSIX build and Linux CI; the general fixture
  generator (`tools/gen-profiles`, Pass 2); any comparison at all between
  iccce and lcms2; CGATS batch I/O; observed residuals (only asserted
  bounds are on record).

**Design decisions in the harness worth not re-litigating:**
`Tolerance` cannot be constructed without a `why` string; `Intent` has
only the four ICC intents (lcms2's 10–15 are inexpressible by
construction); `Precalc` and `Bpc` are required fields, never defaulted;
there is no ΔE metric (would need either a dependency on the code under
test or a second ΔE2000 to get wrong); **exit code 3 means "nothing ran"
and is separate from 0**, so a machine with no oracle cannot report
success.

**How to apply:** before adding a tolerance, check the comparison was
actually *run*, and mirror `NUMERIC_CLAIMS.md` rather than choosing a new
number. When reporting a result, say the scope — Pass 1 is one machine,
Windows/MSVC, no Linux run observed by anyone.

Related: [[project-lcms2-findings-legacy-lab-and-forced-bpc]],
[[project-lcms2-licence-is-not-uniform]],
[[project-doc-editing-conventions]].
