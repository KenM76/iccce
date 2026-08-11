---
name: project-oracle-and-tolerance-state
description: iccce oracle/harness state — the harness drives transicc and the shipped iccce binary, computes ΔE2000 via a path dep on iccce-color, and TOLERANCES.md §3.1/§3.3/§3.4/§5 are filled through Pass 4; what is still deliberately blank.
metadata:
  type: project
---

**★ Updated 2026-08-11 (later, after Pass 4).** Everything below still holds;
add:

- **`pass4` exists** (`tools/difftest/src/pass4.rs`, `bin/pass4_report.rs`,
  README **§14**): `USWebCoatedSWOP.icc` (`mft2` A2B, 4-ch, `Lab ` PCS) → the
  system sRGB profile, **341 CMYK points, all four intents**, `-c0`. Both sides
  are subprocesses again — commit **`490191b`** gave the CLI N-channel input
  and `--intent`, so the in-process fallback that had been planned was not
  needed. `Oracle::convert_batch_shaped(req, in, out)` was added because a
  4-in/3-out transform breaks the single-width `convert_batch`.
- **Pass 4's done-when numbers:** iccce vs lcms2 **max 1.6590 ΔE2000**
  (perceptual/saturation, tol 2.0 — a *structural* gate, not an agreement
  claim) and **0.252 94** (media-relative); with lcms2's own CLUT geometry
  emulated, **4.8154×10⁻³** (tol 2×10⁻²); at the 16 CLUT-node corners
  **6.6558×10⁻⁵** (tol 1×10⁻³). ICC-absolute is **reported, not graded** —
  11.217 ΔE2000, mechanism established (see the Pass 4 findings memory).
  Suite: `pass=36 fail=0 skip=3 error=0`.
- **`TOLERANCES.md` §3.4 and §5.2 are now filled**, NA-006 is **measured**, and
  NA-003's clause citation is **corrected** (6.4 is about the PCS; the device
  clause is 6.5, doubly gated to `DToBx`/`BToDx`).
- **Still blank and correctly so:** §3.2; any ground-truth row for Pass 3 *or*
  Pass 4; the **B2A** direction (`b3f4388` landed the code, nothing measures
  it); `lut8Type`/`mAB `; any v4 profile; any synthetic fixture
  (`tools/gen-profiles` appeared in the tree mid-session but nothing in the
  suite uses it yet).

Original entry follows.

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
