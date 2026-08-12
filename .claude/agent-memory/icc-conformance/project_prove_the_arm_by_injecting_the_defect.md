---
name: project-prove-the-arm-by-injecting-the-defect
description: A conformance arm claimed to be load-bearing must be proven by injecting the defect in a detached worktree and watching it go red — doing that found that NO row of the suite caught a 4.2.5.4 reversion, that the separation mechanism reports ZERO-SEPARATION exactly when the defect is live, and that "differential" is not "load-bearing".
metadata:
  type: project
---

Built **2026-08-12** on `icc-engineer`'s dispatch (the third Pass 5c arm and
Pass 4c separation coverage). Filed in `docs/TOLERANCES.md` §3.5.9 / §3.4.5.1 /
§1.1.2, `tools/difftest/README.md` §21, `tools/gen-profiles/README.md` §4.1.

## 1. ★★★ The demonstration, and it must be the FIRST thing done, not the last

**Injecting the defect into a detached worktree and running the suite is cheap
and it repeatedly overturns what everyone believed.** Procedure that worked:

```bash
git worktree add --detach <scratch>/wt HEAD
cp <my changed files> <scratch>/wt/...
python  # patch crates/ to the pre-fix behaviour; repoint category (c) consts
        # at Z:\no-such-machine to simulate a clean machine
cd <scratch>/wt/tools/difftest
ICCCE_TRANSICC=D:/Dev/iccce/tools/difftest/vendor/build-msvc/transicc.exe cargo run --release
git worktree remove --force <path>
```

**Three things it found that reasoning had not.**

- ★★ **"Differential" ≠ "load-bearing".** The brief was that only the
  category (c) arm could see a 4.2.5.4 regression. Truth: **no row of the whole
  suite went red under a full reversion, on any machine.** The vendor arm's
  numbers moved; none of its graded rows crossed a bound, because the row that
  carries the finding is `REPORTED`. Always ask *does a row FAIL*, not *does a
  number MOVE*.
- ★ **The clause was not undefended.** `cargo test -p iccce-cmm` fails on that
  reversion (two `bpc.rs` tests). The real gap was narrower and more precise:
  the clause **exercised through a parsed profile**, where a wiring defect
  between `Chain` and a library function lives and a unit test on a closure
  cannot reach. **Check the unit tests before claiming CI is blind.**
- ★★ **`Separation::against` lies exactly when it matters.** It derives the
  distance as `|observed − alt_observed|`; when the alternative is *"the code
  returns the other candidate"*, `observed` **becomes** `alt_observed` on the
  defect run and the distance is 0. The new row failed at `2.500019e1` and
  printed `ZERO-SEPARATION` beside it. **Test: is the distance a property of the
  RUN or of the FIXTURE?** A distance between candidate *answers* belongs to the
  fixture — use `against_distance`. Same trap on every `0/1` indicator row.

## 2. A large separation on an UNGRADED row is a request for a FIXTURE

Asked directly whether the `4.717441` separation justified giving
`estimators/black-points-in-lab` a real tolerance. **No.** No clause requires two
implementations of two different documents to agree; a bound below the
separation is a bound fitted to the one defect; and it could not be one number
(the three arms observe `4.799 / 5.000 / 10.000`). **Ask what clause the number
would be graded against; if the answer is "none, but it would have caught the
bug", the bound is fitted to the bug.** The right response is a fixture plus a
graded row with a derivable bound elsewhere.

## 3. GP-002 generalised — how to author a fixture that cannot re-collapse

`fixtures/synthetic/v4-rgb-mab-floored-b2a.icc` (recipe `v4-rgb-mab-floored-b2a`,
18 656 bytes). Sibling + **one** structural change: `B2A` floors `G` for *every*
input, lifting the round-trip floor to `L* 37.5` while `A2B(0,0,0)` stays at
`L* 12.5`.

- **Change one property and every shared constant.** The floor is the variable;
  `12.5 / chroma 10.0` vs the sibling's `20 / chroma 5.0` means a figure quoted
  without its arm is *obviously* wrong rather than plausibly right.
- **Give the fixture its own graded row** (`FIXTURE/candidates-are-separated-as-designed`,
  measured separation vs *designed* separation). The separation mechanism can
  report that a row is blind; **only a graded row can stop it becoming blind**,
  and the collapse arrives as a consequence of reasonable-looking edits.
- **Derived-expectation rows must run OUTSIDE the oracle path.** `analyse` needs
  the system sRGB profile + `transicc` + the shipped binary; the clause rows need
  the committed fixture and nothing else. *A ground-truth-shaped row must not be
  hostage to an oracle.*
- **Compute the rival in the harness, not via the crate under test** — a broken
  build otherwise reports its own broken rival.
- Authored constants typed into the harness are **safe** in a way measured
  numbers are not: a design number typed into an assertion fails loudly,
  a measured number typed into prose rots silently. `gen-profiles verify` is what
  makes that true.

## 4. Two useful negatives that cost nothing to record

- **`neutralise_and_clip` clips at 50; lcms2's `BlackPointAsDarkerColorant` has
  an extra `if (L > 95) L = 0`.** A fixture with a vertex lighter than `L* 95`
  would give a 50-`L*` divergence out of one `if`. Untested, owed.
- **`vertex_set(3)` is `{(0,0,0), (1,1,1)}` only** — not all 8 corners — so an
  **inverse-polarity** fixture would separate ISO's *search* from lcms2's fixed
  `_cmsEndPointsBySpace` constant (ISO 4.2.2.2 NOTE 2). Also owed.
- A fixture whose `B2A` is floored makes `d(device)/d(L*)` **zero**, so any row
  converting a device residual to `L*` is void on it. Declare that in an
  **authored table**, and grade the measurement against the declaration — never
  let a row demote itself because a quantity came out small.

Related: [[project-candidate-separation]],
[[project-conformance-can-worsen-the-crosscheck]],
[[project-pass4c-absolute-intent-findings]],
[[project-parallel-agent-build-collisions]],
[[project-synthetic-fixture-corpus-and-gp001]].
