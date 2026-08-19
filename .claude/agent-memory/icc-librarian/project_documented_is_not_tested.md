---
name: iccce-documented-is-not-tested
description: DL-051 — a suite that documents a constant at length while unable to detect its corruption is WORSE than one saying nothing; the doc comment's own thoroughness is what hides the gap. Five injections, the best-documented constant caught by NONE. Found by injection, not inspection. 2026-08-17.
metadata:
  type: project
---

**A test suite that documents a constant at length while being unable to
detect its corruption is worse than one that says nothing, because it
READS AS PROTECTION.** Filed as `ARCHITECTURE.md` **DL-051**;
`NUMERIC_CLAIMS.md` §3.32.4 / **NC-228**.

**Why:** `crates/iccce-cmm/src/builtin.rs` shipped a long, well-sourced
doc comment on why the sRGB breakpoint is `0.04045` and not `0.03928` —
C⁰ vs C¹ solutions, the standard each comes from, the reasoning — plus
five tests. Injection matrix:

| injected defect | caught by |
|---|---|
| Bradford adaptation omitted | 3 of 6 FAILED |
| adaptation applied twice | 3 of 6 FAILED |
| TRC → pure gamma 2.2 | 1 FAILED |
| **breakpoint `0.04045` → `0.03928`** | **NOTHING. 6 of 6 PASSED** |
| green primary `0.600` → `0.610` | 2 FAILED |

★★★ **The LENGTH of the documentation is the mechanism. Nobody audits a
constant that is visibly well-explained** — a reader scanning for weak
spots sees the sourcing paragraph and moves on. The documentation
performs a test's function without being one, **convincingly in
proportion to how well it is written.**

### Three distinctions that make this usable

- **★★ It is a SIBLING of "a test that cannot fail is not evidence"
  (`NEXT_SESSION.md` §5.3) and NOT the same thing.** There the tests
  could not fail *at all*. Here **every test could fail** — three did —
  **just not for this defect.** ★★★ **A suite's power is PER-DEFECT, not
  per-suite**; a green suite is a statement about the defects someone
  thought to inject.
- **★★★ Found by INJECTION; inspection could not have found it.** The
  same person wrote the constant, the comment and the tests in one
  sitting and read them back without seeing it. **Reading your own work
  is not an instrument** (§5.2's structure, outside the oracle).
- **★ The same class covers a STALE SOURCING CLAIM in a doc comment.**
  `illuminant.rs`'s `D65_XY` said *"SINGLE SOURCE — not cross-verified"*
  long after BT.709-6 made it false. **DL-048's class is not confined to
  `docs/`, and it decays with no line numbers involved** — the pointer
  stayed valid while the *characterisation of the source's strength* went
  stale.

### ★★ It recurred inside the same session, hours later

`crates/iccce-profile/src/colour_space.rs`'s corruption test carries a
doc comment explaining the failure mode *"stated as a property rather
than a single case"* — **and the property is not tested**: the survivor
count reaches a `println!` (invisible without `--nocapture`) and the
in-loop assertion compares `components(sig).count()` with **its own
result**, so it cannot fail. Found by this librarian reading the file at
the tip; **the dispatch did not carry it.**

### ★★★ The sequel, 2026-08-19 — DL-064 supplies the other half

DL-051 says *a passing test is not evidence until an injection turns it
red.* **DL-064 says a red injection is evidence at ONE magnitude and no
other.** Pass K's leak guards were turned red as this entry demands — and
the same sweep found that **below `5.0e-2` the whole suite is green with
the defect compiled in.** See
[[iccce-injection-measures-one-magnitude]]. ★ **Asking for "the injection
result" is now insufficient; ask for the smallest magnitude that still
PASSED.**

### How to apply

- When a dispatch reports a **well-documented** constant, treat the
  documentation as **zero evidence of test coverage** and ask for the
  injection result specifically — ★ **and for the sweep, not the single
  point** (DL-064).
- ★ Watch for the shape: **a long `why` in the doc comment + a short
  assertion**. That combination is where this hides.
- A ledger row for a constant should record **which injection turns it
  red**, not that it is well sourced.

Related: [[iccce-injection-measures-one-magnitude]],
[[iccce-count-from-a-sample-is-not-the-population]],
[[iccce-stale-citation-worse-than-stale-number]],
[[iccce-agreement-can-be-the-symptom]],
[[iccce-disclosure-caught-a-bad-justification]], [[iccce-pass-status]].
