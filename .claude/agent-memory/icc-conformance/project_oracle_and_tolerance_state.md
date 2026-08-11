---
name: project-oracle-and-tolerance-state
description: iccce oracle/harness state — as of Pass 3 the harness drives BOTH transicc and the shipped iccce binary, computes ΔE2000 via a path dep on iccce-color, and TOLERANCES.md §3.1/§3.3/§5 are filled; what is still deliberately blank.
metadata:
  type: project
---

**Supersedes the earlier 2026-08-11 entry that said "no comparison between
iccce and lcms2 exists".** State as of **2026-08-11 (Pass 3, commit
`051707f` of the code under test)**:

- **The oracle exists and is pinned.** lcms2 `21c582a` (2.19.1),
  `tools/difftest/lcms2.pin`, MSVC build only. POSIX build script still
  never run.
- **The harness now drives two subprocesses**, not one: `Oracle` →
  `transicc`, and **`Iccce` → `target/release/iccce.exe`**. Both sides of
  every Pass 3 comparison cross a process boundary, deliberately — an
  in-process call on iccce's side would make the two sides asymmetric.
  Batch mode (`convert_batch` / `transform_grid`) pushes a whole grid
  through **one** process; stdin is written on a **background thread**
  because a single-threaded write-then-wait deadlocks once the child's
  stdout buffer fills.
- **★ The "no ΔE" policy was deliberately reversed.** `tools/difftest`
  now path-depends on `iccce-color`, `iccce-profile`, `iccce-cmm`. The
  four-part justification is in `tools/difftest/Cargo.toml`'s header and
  README §13.2 — arrow points harness→code-under-test; the ruler
  (`delta_e_2000`) is ground-truth-validated against 34/34 Sharma pairs;
  the CLAIM stays cross-check; answers still come from subprocesses. There
  is also an **instrument check** that holds iccce's Lab ruler against
  lcms2's, so a bent ruler shows up as a number.
- **`TOLERANCES.md` §3.1, §3.3, §4, §5 (NA-001..NA-004) and §6 are
  filled.** §3.2 (Pass 2) and §3.4–§3.6 remain blank and correctly so.
- **Pass 3's two done-when numbers:** iccce vs lcms2 **max 3.476e-3
  ΔE2000** (tol 2e-2); round trip **max 1.8788e-2 ΔE2000** (tol 2.5e-2).
  Scope: sRGB→AdobeRGB1998 (both Windows system, both **v2.1**),
  media-relative only, 133-point deterministic grid, Windows/MSVC.

**Design decisions worth not re-litigating:** `Tolerance` still cannot be
built without a `why`; `Intent` still has only the four ICC intents;
`Precalc`/`Bpc` still required; **exit 3 = "nothing ran" ≠ 0**. New:
`Report` stores `Record` (not `Check`) so grid comparisons that reduce a
whole grid to one number fit without contorting `Check`; the `why` and
`source` strings are emitted on **every** line including skips.

**Still open:** POSIX build and Linux CI; `tools/gen-profiles` (so Pass 3
has no synthetic fixture and skips entirely off this machine); any
ground-truth row for Pass 3; the reverse direction; any v4 profile; LUTs
and the other three intents.

**How to apply:** before adding a tolerance, check the comparison was
actually *run*, and state the scope in the same breath as the number.
Related: [[project-lcms2-findings-pass3-quantisation-and-clamping]],
[[project-encoded-white-points-differ-between-profiles]],
[[project-lcms2-findings-legacy-lab-and-forced-bpc]],
[[project-lcms2-licence-is-not-uniform]],
[[project-doc-editing-conventions]].
