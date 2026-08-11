---
name: project-oracle-and-tolerance-state
description: iccce oracle/harness state — the harness drives transicc and the shipped iccce binary (now with --bpc), computes ΔE2000 via a path dep on iccce-color, and TOLERANCES.md §3.1/§3.3/§3.4/§3.5/§5 are filled through Pass 5; what is still deliberately blank.
metadata:
  type: project
---

**★★★ Updated 2026-08-11 (after Pass 5).** Everything below still holds; add:

- **`pass5` exists** (`tools/difftest/src/pass5.rs`, `bin/pass5_report.rs`,
  README **§16**, `TOLERANCES.md` **§3.5** and **§6.5**): six scenarios
  (S1–S6) plus a §A that grades the BPC **scaling map** against **ICC.1:2022
  clause 6.3.4.3** and against a Gaussian solve of Maria 2013's two
  constraints. **26 records, all pass.** Whole suite:
  **`pass=90 fail=0 skip=3 error=0`**.
- **`Iccce::transform_rows_shaped_bpc(.., bpc: bool)`** was added to `lib.rs`;
  `transform_rows_shaped` delegates to it. A refusal surfaces as
  `DiffError::NonZeroExit` carrying stderr, and Pass 5 **grades two of them**.
- **§A and S6 need neither a system profile nor the oracle** — the first graded
  rows in this suite that survive a machine with no colour directory *and* no
  lcms2 build.
- **`TOLERANCES.md` §3.5 is filled**; §3.2 and every *published* ground-truth
  row remain blank, and for Pass 5 they can never be filled (A27/A42 — no
  normative BPC text obtainable).
- Findings: [[project-lcms2-findings-pass5-bpc]].

**★★ Updated 2026-08-11 (later still, after Pass 4b).** Everything below still
holds; add:

- **`pass4b` exists** (`tools/difftest/src/pass4b.rs`, `bin/pass4b_report.rs`,
  README **§15**, `TOLERANCES.md` **§3.4.4**): **three independent sections** —
  §A the **B2A** direction (`sRGB → USWebCoatedSWOP`, `mft1`/`lut8`, 3→4, 33³,
  213 RGB + 258 Lab points, perceptual and media-relative), §B the **v4
  `mAB `/`mBA ` synthetic fixture** in both directions, §C the **F.2 grayTRC**
  model (`ewgray22.icm → sRGB`, 69 points). **28 records, all pass.** Whole
  suite: **`pass=64 fail=0 skip=3 error=0`**.
- **A fourth `Kind` exists: `DerivedExpectation`.** An expectation computed by
  arithmetic from clause text + a synthetic fixture's own bytes, with no
  implementation's output in it. **Not** ground truth (nobody published it; the
  fixture and the derivation share a corpus), but stronger than a cross-check.
  Defined in `TOLERANCES.md` §3.4.4.1; §1's kind table was deliberately *not*
  rewritten (shared section) — it carries a pointer, and extending it is owed.
- **The §3.4.3 "still blank" rows for the B2A direction and for a synthetic LUT
  fixture are now MEASURED.** The **published-value** row is still blank and
  Pass 4b does not close it.
- **`Iccce::transform_rows_shaped`** (n in / m out) was added — Pass 4 could
  hard-code 3 outputs; a CMYK destination and a gray source cannot.
- **Still blank and correctly so:** §3.2; any *published* ground-truth row for
  Passes 3/4/4b; saturation and ICC-absolute anywhere in Pass 4b; `lut8` with
  an XYZ PCS (refused by name); any **real** v4 LUT profile (a 40-profile sweep
  of this machine found **zero** `mAB `/`mBA `); the M3 out-of-range excursion
  count, which §A's saturated-hue block could have recorded and did not.

New findings: [[project-lcms2-findings-pass4b-direction-dependence]].

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

**Still open:** POSIX build and Linux CI; any ground-truth row for Pass 3;
the reverse direction; the other three intents.
**No longer open (2026-08-11):** `tools/gen-profiles` and
`fixtures/synthetic/` now exist — 38 committed profiles including v4, LUT
and CMYK shapes, so a Pass 3/4 comparison need no longer depend on this
machine's installed profiles. See
[[project-synthetic-fixture-corpus-and-gp001]].

**How to apply:** before adding a tolerance, check the comparison was
actually *run*, and state the scope in the same breath as the number.
Related: [[project-lcms2-findings-pass3-quantisation-and-clamping]],
[[project-encoded-white-points-differ-between-profiles]],
[[project-lcms2-findings-legacy-lab-and-forced-bpc]],
[[project-lcms2-licence-is-not-uniform]],
[[project-doc-editing-conventions]].
