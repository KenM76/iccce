---
name: iccce-count-needs-its-apparatus
description: DL-031 — an unlabelled test/record count is not a claim because the apparatus is half the number; iccce has THREE runners (129 / 36 / 142) whose populations are disjoint, and two were briefly compared as a regression
metadata:
  type: project
---

**Never write a count without the command that produced it.** *"The
suite is green at N"* is a fact about **one invocation of one runner
over one member set**, not about the project.

**Why:** on 2026-08-12 `icc-engineer` ran `cargo test --workspace` at
the tip, got **129**, and briefly read it as a regression against a
*"suite green at 142"* he had written into commit `d5efd96`'s message
**hours earlier**. They are different instruments. If a count can be
misread by its own author within a day, it will be misread by everyone
else. Filed as `ARCHITECTURE.md` **DL-031**; the ledger gained an
**`apparatus-census`** evidence class so a count can never sit beside a
ΔE as though the two were comparable.

**The three runners in iccce (as of 2026-08-12):**

| Command | Population | Result |
|---|---|---|
| `cargo test --workspace` (repo root) | the **five** crates | **129 passed, 0 failed**, exit 0 |
| `cargo test` in `tools/difftest` | the harness — **deliberately NOT a workspace member** (DL-001/DL-017), so `--workspace` cannot see it | **36 passed** |
| `cargo run --release` in `tools/difftest` | the **conformance runner** — grades records against `TOLERANCES.md`, drives lcms2 | **pass=142 fail=0 skip=3 error=0** |

**142 counts CONFORMANCE RECORDS, not test functions.** Its only valid
comparison is to its own previous run (`pass=140 fail=2`) — **and even
that needs the record *shape* attached**, because rows get reformulated.

**How to apply:**

- Filing a count: give the command, the member set, and the tip.
- Reading a count in an older doc or a commit message: **treat it as
  uninterpretable until the apparatus is identified.** Do not infer a
  trend from two numbers.
- ★ **A commit message cannot be corrected.** The ambiguous number here
  lives in git history, where nothing names an apparatus and no dated
  note can ever be appended — the strongest argument for writing the
  command down the first time.
- **A count is not an inventory.** `iccce-cli` contributes **0** tests
  and the total cannot notice. Corroborating a *per-crate* breakdown
  against `#[test]` declaration counts (which this filing did, matching
  on all five) validates the **denominator** — nothing was filtered or
  `#[ignore]`d — and says nothing about pass/fail or coverage.
- **A green census can still hide things**: `skip=3` was reported and
  never enumerated. A skip is the runner declining to grade, and it is
  invisible in `fail=0`.

Related: [[iccce-pass-status]], [[iccce-verify-own-draft-too]],
[[iccce-git-files-readable-without-shell]].
