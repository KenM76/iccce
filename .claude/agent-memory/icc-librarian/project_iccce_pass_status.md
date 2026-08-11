---
name: iccce-pass-status
description: iccce status snapshot 2026-08-11 (end of Pass 3 core) — Pass 3 engine + transform CLI built, done-when NOT met (no lcms2 comparison yet); Pass 2 still open on one scope decision; DL-015/DL-016 filed
metadata:
  type: project
---

**Snapshot of 2026-08-11 (end of Pass 3 core + the `transform` CLI).
Verify before relying on any of it** — read `docs/ROADMAP.md`,
`docs/NUMERIC_CLAIMS.md`, `docs/NEXT_SESSION.md` and the newest
`docs/SESSION_LOG.md` entry.

**Commits, ALL reported by dispatches — no agent in this project has run
git:** Pass 0 `f976a0e`, Pass 1 `7313c5b`, Pass 2 batch 1 `b35a12e`,
difftest harness `bfd6b1e`, Pass 2 batch 2 `d40d601`, **Pass 3 core
`c4038eb`**, **`iccce transform` `051707f`**.

**Built:** `iccce-color` (Pass 1); `iccce-profile` header + tag table +
eight non-LUT tag types + the four LUT types; iccMAX refused by name
since Pass 0; `tools/difftest` + `legacy_lab_probe`; **`iccce-cmm` is no
longer a stub** — tone curves (evaluate + invert per **Annex F.1,
NORMATIVE**) and the **Annex F.3** matrix/TRC model, media-relative
only; `iccce transform` (stdin triples, 6 decimals, the difftest
interface). **68 `#[test]` declarations; still exactly ONE correctness
claim against published values (NC-001).**

**★ The headline is a negative: `iccce` has a transform and has STILL
never been compared to another implementation.** Zero
`implementation-cross-check` rows. **Pass 3's done-when is NOT met** —
it needs an sRGB→AdobeRGB round-trip ΔE and a justified lcms2 tolerance,
and `icc-conformance` was dispatched **in parallel** to measure both.
**Whether that landed is unverified; check `NUMERIC_CLAIMS.md` §3.7 (next
free number NC-034, none reserved) and `TOLERANCES.md` §3.3.**
**Pass 2 is also still in progress** (the clause-2 scope decision), so
the Passes are no longer completing in order.

**Two Pass 3 findings worth carrying:** an exact-value test caught
`TRC(1.0) = 0.998` (an off-by-one at the table endpoint) that **both
self-consistency checks would have passed** → **DL-016**; and the system
sRGB profile's colorant `Z` sums to **0.825089** (the 1998 author's own
white rounding), which forced a tolerance to be re-justified by **what
it discriminates** rather than by a quantisation claim the file never
made → **NC-031**.

**DL-015** — the parametric `pow(neg, frac)` guard follows lcms2 over
ICC's sample code. **NOT a deviation from normative text**: clause 10.18
declares those parameter combinations *explicitly undefined*, a stated
non-requirement. The register (`NUMERIC_CLAIMS.md` §4) now states the
**kind** of departure per row so NA-001 and NA-004 cannot be conflated.

**New evidence class:** `normative-rule-conformance` — expectation taken
from **verbatim normative text** at `primary_spec` tier. Stronger than an
arithmetic identity, weaker than a published dataset, and it **inherits
the corpus's transcription risk**.

**Corrected this session (a prediction this librarian had filed twice):
Pass 3 does NOT adapt.** `iccce-cmm` never calls `adapt.rs` and never
reads `chad`/`wtpt`; conformant colorants are already D50-referenced.
**NA-002's Bradford cost is therefore NOT owed by Pass 3** — it moves to
the first Pass that adapts (likely Pass 4 / absolute intent). New
**NA-005** registers *"colorants used as stored"* as a named assumption
with an unmeasured cost.

**New named corpus gap:** the **media-relative → ICC-absolute
white-point adjustment formula is not transcribed** in `ICC_Spec`. The
intent is refused by name rather than written from memory.
`icc-spec-librarian` owes it; *"it is probably in clause 6.x or an
Annex"* is a prediction until the document is open.

**Still open every session:** `TOLERANCES.md` §3.2 all `—`, §3.3 empty,
§6 still "2–8 not started"; twin rows for §3.7's twelve tolerances;
`ncl2`/B2A behavioural tests; the forced-BPC copy decision (DL-013); a
ground-truth row for chromatic adaptation; the DL-014 audit of
**pre-existing** citations (only Pass 3's five new sites were checked —
4 of 5 compliant); **nothing has ever run on Linux and no CI run has
ever been observed.**

Related: [[iccce-bound-cannot-catch-its-own-magnitude]],
[[iccce-predicted-divergence-must-be-measured]],
[[iccce-verification-loop-runs-both-ways]], [[icc1-pdf-operator-blocker]],
[[ken-terse-scope-decisions]], [[iccce-verify-own-draft-too]].
