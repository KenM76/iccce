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
  never enumerated for three filings. ★ **Enumerated 2026-08-12 (tip
  `e26d9ba`): three rows, ONE cause** — the Pass 4 `icc-absolute`
  PCS-isolation rows, withheld because `transicc` applies the D.6/D.7
  media-white scale on lcms2's side while iccce's A2Bx is media-relative
  by construction. Grading them would mean **modelling the oracle rather
  than measuring it**. **A principled refusal to grade, not a concealed
  failure** — and `fail=0` could never have said so. **Caveat that
  survives: those are the skips of the CURRENT tip; the item was opened
  against an earlier run's `skip=3`, and same-count-same-cause is not
  same-rows.**

**★★ UPDATE 2026-08-12 (seventeenth filing) — A NUMBER COLLISION THIS
ENTRY PREDICTED.** The new candidate-separation aggregate prints
**`unstated=129`**. The workspace unit suite is also **129**. **Two
apparatus, two unrelated quantities, one tree, one day apart.** Nothing
may be inferred from their equality and it will not survive the next
test being written. ★ **This is the exact failure this entry exists to
prevent, arriving from a direction nobody watched** — not a stale count
re-quoted, but a *fresh* count that happens to equal an old one. **Name
the runner in the same sentence as the number, always.** And in that
same dispatch the runner producing `pass=142` and the 145-row aggregate
was **not named at all** — the attribution to `tools/difftest` is an
inference from the row ids (`pass4/…`, `pass5c/…`), which is recorded in
the ledger as a gap rather than glossed.

**★★ UPDATE 2026-08-17 (twentieth filing) — A THIRD COLLISION, and this
one caused a WRONG CITATION in an outbound document.** An outbound
request to `pdfce` cited `NUMERIC_CLAIMS.md:2529` as the basis of
iccce's **33-node recommended grid**. That line carries a **different
33** — `USWebCoatedSWOP.icc`'s **own `lut8` CLUT node count**, a vendor
file's tag. (The companion citation `:2164` states the grid as **17**.)
The real homes are **§3.19 / NC-145**, **§3.27**, and
`crates/iccce-cmm/src/compiled.rs:77` (`recommended_grid_points`). ★ **A
reader following the citation would have concluded the recommendation is
a property of somebody else's profile.** ★★ **The argument was right and
the citation was wrong** — which is the combination that survives review,
and the reason to grep a number's real home before citing a line number.
**Running tally of collisions in this project: `129`, `16`, `33`.**

Related: [[iccce-pass-status]], [[iccce-verify-own-draft-too]],
[[iccce-compatibility-not-certification]],
[[iccce-git-files-readable-without-shell]].
