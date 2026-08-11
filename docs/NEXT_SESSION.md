# NEXT SESSION — start here

**Written 2026-08-11 by `icc-librarian`, at the CLOSE of Pass 3.**
Replaces the Pass 3-core edition entirely. Overwrite this file once acted
on.

Read order: this file → `docs/ROADMAP.md` (the **Pass 3 completion
record**, then the **Pass 4** annotation) → `docs/ARCHITECTURE.md` §5
(**eighteen** entries; **DL-017** and **DL-018** are new) →
`docs/NUMERIC_CLAIMS.md` (**§2.4** → the dated status on **§3.7.0** →
**§3.8**, the ten new rows, **starting with its coverage box** → §4's
**NA-006** and **NA-007** → **§5.3, the retirement** → §6 → **§7.4**) →
`docs/TOLERANCES.md` §3.3 → `tools/difftest/README.md` **§13** →
`docs/SESSION_LOG.md` (seven entries, all 2026-08-11; the seventh is this
work).

---

## Where the project actually is

**Pass 0 done. Pass 1 core complete. Pass 2 built, one scope decision
from done. ★ Pass 3 DONE — done-when met.** All on 2026-08-11.

**`iccce` has been compared to another implementation.** After four
filings in which *"iccce has never been compared to anything"* was the
headline, it is retired (`NUMERIC_CLAIMS.md` §5.3).

| | Commit *(all **reported** — no agent here has ever run git)* |
|---|---|
| Pass 0 | `f976a0e` |
| Pass 1 | `7313c5b` |
| Pass 2 batch 1 | `b35a12e` |
| difftest harness + probe | `bfd6b1e` |
| Pass 2 batch 2 — the LUT family | `d40d601` |
| Pass 3 core | `c4038eb` |
| `iccce transform` | `051707f` |
| audit items closed | `55772c6` |
| the Pass 3-core filing | `a9618fe` |
| **n-linear CLUT evaluator** | **`fc5ff58`** |
| **16-bit PCS encodings** | **`0843094`** |
| **absolute intent + Table 25 policy** | **`6873df1`** |
| **the Pass 3 differential + `LEGAL.md` §1** | **`986dae6`** |

### The two numbers, so nobody has to go looking

- **iccce vs lcms2, sRGB→AdobeRGB: max 3.4762×10⁻³ ΔE2000** (mean
  5.1145×10⁻⁴), tolerance **2×10⁻²**, class
  **`implementation-cross-check`**.
- **Round trip sRGB→AdobeRGB→sRGB: max 1.8788×10⁻² ΔE2000** (mean
  8.674×10⁻⁴), tolerance **2.5×10⁻²**, class **`self-consistency`**.

**Both are one profile pair, one intent, one direction, 133 points, one
platform, one lcms2 pin, and both files are v2.** The scope box at the
head of `NUMERIC_CLAIMS.md` §3.8 must travel with either number.

### What is easy to over-read, so read it here first

- **A cross-check is not ground truth**, and here the shared-misreading
  risk is *elevated*: the corpus's sRGB constants rest on **lcms2 alone**
  and **D65 is single-source**. `TOLERANCES.md` §3.3.3's first blank row
  — *nothing yet compares a matrix/TRC transform to a published value* —
  is **the largest evidential hole in Pass 3** and nobody has dispatched
  for it.
- **Absolute intent is implemented and cannot be cross-checked**, because
  `iccce transform` still refuses every intent but media-relative.
  Unit-test and corpus evidence only. (**NA-007**.)
- **`lut_transform.rs` is already in the tree** — *"Pass 4 assembly,
  stage 1"* — and the dispatch that closed Pass 3 did not mention it.
  **Whether it is committed is unknown.** Do not plan Pass 4 as though
  nothing exists; read it first.
- **The differential's `pass=8` / `pass=7` discrepancy is unresolved**
  (`NUMERIC_CLAIMS.md` §2.4). Structurally 1 registered check + 7
  records = 8. The re-run's per-line output was never recorded.
- **"87 tests" is not coverage.** Two skip silently without the system
  sRGB profile; **all seven differential records skip** without the
  Windows colour directory — including in CI, which has still never been
  observed running.
- **Pass 3 does NOT adapt, and absolute intent did not change that.**
  D.6/D.7 is a per-component diagonal scale; `chad` is deliberately not
  un-applied. **`iccce_color::adapt` is still called by no transform.**
  NA-002's Bradford cost is **still not due**. This has now been checked
  against the code at two consecutive filings; check it again rather than
  carrying it.
- **Pass 2 is still in progress**, and it now blocks something concrete
  (below).

---

## Then: **Pass 4 — LUT transforms and rendering intents**

`A2B`/`B2A`, multi-dimensional interpolation, all four intents. **Done
when**: CMYK→RGB through a real press profile matches lcms2 within
tolerance at every intent, and the v2/v4 cases are separately covered.

### What already exists — read before planning

| In the tree | What it is |
|---|---|
| `iccce-cmm/src/clut.rs` | n-linear CLUT evaluation, the **A16 named choice**, A20 index ordering pinned by an asymmetric-grid test, and the DL-016 bug class inherited-fixed and pinned by an input-1.0 test |
| `iccce-cmm/src/pcs_encoding.rs` | `LabEncoding {Legacy, V4}` — **no default**, the caller must say which — exact-value D1 invariants, exhaustive 65536-code round trips, and a discriminator asserting the **wrong** cross-decode equals lcms2's measured 99.6109 |
| `iccce-cmm/src/lut_transform.rs` | **`mft2` device→PCS pipeline** (input tables → \[3×3, PCSXYZ only, A21\] → CLUT → output tables → PCS decode). **Not in the closure dispatch's commit list; commit status unknown** |
| `iccce-cmm/src/matrix_trc.rs` | `Intent` enum, absolute per D.6/D.7 (**the corrected direction**), the sourced Table 25 policy |
| `iccce-profile/src/lut.rs` | all four LUT tag types, decoded (Pass 2 batch 2) |

**So Pass 4 is assembly, not invention.** What it needs:

1. **A sourcing decision on lcms2's tetrahedral interpolation.**
   `clut.rs` deliberately omits tetrahedral — the cube decomposition has
   several published variants and the corpus carries none of them, so it
   *"will be sourced before it is written, not recalled"* (rule 2).
   **This is the single largest expected iccce-vs-lcms2 deviation in
   Pass 4, at up to ~1 ΔE** — at the perceptibility threshold, not below
   it. Either source it, or **budget for the interpolation-method
   difference in every Pass 4 tolerance and say so**. What is not
   available is a tolerance wide enough to swallow ~1 ΔE also being
   offered as evidence of agreement. (**NA-006**.)
2. **The v2 `lut16` assembly** — finish what `lut_transform.rs` starts:
   the B2A direction, and the source→destination chain.
3. **Intents on LUT profiles per the now-sourced 8.10.2 fallback.**
   `icc__s__rendering_intents.md` carries the `shall`-level a)–d)
   fallback order and Table 25. Path selection follows **the specified
   order**, never an invented reasonable one.
4. **A v4 profile pair**, which Pass 3 avoided rather than exercised.

### What Pass 4 inherits, all live from its first commit

1. **Its done-when is known to be underspecified.** *"At every intent"*
   collides with **DL-013**: against a **v4** profile lcms2 forces BPC on
   at perceptual and saturation, ≈3.15 `L*` at black. Account for it
   explicitly, or restrict to the colorimetric intents **and say which**.
   Widening a tolerance to swallow it is not available.
2. **The v2/v4 Lab encoding selector is settled: the TAG TYPE**
   (DL-011), lcms2 agrees at the pin (DL-012, measured), and
   `pcs_encoding.rs` now implements it with **exact-value** invariants
   per **DL-005**. Never ΔE — the confusion costs only ≈0.3–0.5 ΔE, below
   the anchor, so a ΔE-graded suite **cannot detect it**.
3. **DL-016 generalises to every table Pass 4 touches.** The endpoint is
   where an off-by-one hides best. `clut.rs` already inherits the fix and
   pins it at input 1.0; `mft1`/`mft2` input and output tables need the
   same treatment.
4. **DL-018 is new and comes due in Pass 5 especially.** An upper-bound
   gate on a **deliberate cost** must be paired with a **prediction
   pin** plus a sensitivity control — BPC is exactly that shape.
5. **NA-002 and NA-005 and NA-007 all come due at the first path that
   adapts, or the first differential that reaches absolute intent.**
6. **The F.3 NOTE's `(32 768/65 535)` scale factor** applies when a
   matrix/TRC model is expressed as a `lutAToBType`. It is
   `0.500 003 8…`, **not** ½; deriving it as ½ is a ≈7.6 ppm error,
   invisible in colour forever and fatal to bit-exact comparison.

---

## Owed work, carried explicitly

### 1. `icc-conformance`

- **The four items `tools/difftest/README.md` §13.10 owes**: a fixture
  distinguishing **clamp-before from clamp-after** (needs a TRC whose
  inverse is defined outside [0,1]); the **reverse** direction
  AdobeRGB→sRGB, the one with a real gamut clip; a **v4** pair; and a
  **synthetic** pair so §13 stops skipping everywhere but this machine.
- **A ground-truth row for the matrix/TRC path** — `TOLERANCES.md`
  §3.3.3's first blank row.
- **`TOLERANCES.md` §3.2 (Pass 2)**, **§6's coverage table**, and **twin
  rows for §3.7's twelve Pass 3-core tolerances** (§3.3 now covers the
  differential's seven, not those).
- **A behavioural test of `ncl2` and of B2A** legacy-Lab decoding —
  cheaper again now that `pcs_encoding.rs` exists.
- **The Pass 4/5 forced-BPC decision** (DL-013), deferred three filings
  running.
- **An observed residual for NC-032** — still the cheapest number in the
  ledger, and it would turn DL-016's reconstruction into a measurement.
- **A run recorded with per-line output**, to settle `pass=8` vs
  `pass=7`.

### 2. `icc-spec-librarian`

- **★ lcms2's tetrahedral cube decomposition**, as measured /
  implementation-reference material — the Pass 4 blocker.
- **The NC-043 clamping question**: clause **6.4**'s integer-vs-float32
  clipping rule read **together with Annex F.8–F.16**. Two sources
  disagree about whether this was dispatched; **the answer is not in the
  corpus**.
- **IEC 61966-2-1's sRGB primaries**, which would give the project its
  first ground-truth row for a transform *and* strengthen every
  cross-check from *"agrees, possibly for the same wrong reason"*.
- **The ITU terms determination** before any BT.709 fetch (DL-007).
  *"Free download"* is not *"automated retrieval permitted."*
- **Corpus ambiguity A4b** — the meaning of a non-D50 `wtpt` in a **v2**
  profile, currently resting on implementation consensus (**NA-007**).

### 3. `icc-engineer`

- **`iccce-cmm/src/lib.rs`'s §Status is stale again** — it says the
  absolute intent *"awaits its sourced formula"* on a crate that
  implements it, and omits `pcs_encoding` and `lut_transform` from its
  module list. Second consecutive filing reporting this file.
- **`clut.rs`'s module doc says *"per rule 4 (named and measured)"*** of
  an approximation that is named and **not** measured (**NA-006**).
- **Expose intents through `iccce transform`**, or absolute intent stays
  permanently outside differential reach.

### 4. `icc-librarian` / whoever files next

- **A DL-014 citation audit of the closure and Pass 4-groundwork code.**
  `clut.rs`, `pcs_encoding.rs`, `lut_transform.rs` and `matrix_trc.rs`'s
  intent block add citations to 10.10, 10.6, 6.3.4.2, Table 25, 8.10.2,
  9.2.36, D.6/D.7 and eight ambiguity rows. **Spot-reading suggests they
  follow the shape; suggesting is not auditing.**
- **The pre-existing audit** of `iccce-color` and `iccce-profile`
  citations, still untouched by anyone.
- **A per-tag-type breakdown of the Pass 2 sweep.**
- **Observed residuals for Pass 1's rows** (§1.1).
- **A ground-truth row for chromatic adaptation** — still the largest
  hole in Pass 1, still not on a clock.
- **A Linux run of anything at all.**

### 5. The operator

- **The Pass 2 clause-2 scope decision** is still open and now blocks
  something concrete: without `tools/gen-profiles/`, **every Pass 3
  differential row skips** on any machine but this one, CI included.
  Both readings are stated in `ROADMAP.md`'s batch 2 block, neither
  recommended.

---

## Optional operator unblocks — cheap, each settles something named

**All are browser downloads by Ken, not agent retrievals.**

| Document | What it settles |
|---|---|
| **`ICC.1:2010-12` (v4.3)** | **A31 / D10** — what changed in `parametricCurveType` **Table 68** between editions. Two conformant CMMs on different editions can evaluate the same `'para'` tag differently |
| **`ICC.1:2001-04` (v2)** | **A1b, A2, A34** — the only normative home of `textDescriptionType`; and **A4b**, the v2 `wtpt` question NA-007 rests on |
| **ICC's published D65→D50 `chad` values** (Annex E.4.2) | the one place the adaptation ground-truth hole could be partly filled from published values |
| **IEC 61966-2-1** | the **first ground-truth row for a transform**, and the end of the single-source sRGB/D65 lineage |
| **ITU-R BT.709** | a second source for sRGB primaries and D65 |

**Each row is a claim about what a document contains.** Treat *"it would
settle A2"* as a prediction until the document is open.

---

## Decisions already made — do not re-litigate

- **MIT**, dependencies permissive, **publishing is the operator's act**
  (rule 9). DL-009 records an *intent*; **intent is not authorisation.**
- **`iccce-color` depends on nothing** and contains no ICC.
- **The parser reports, it does not repair**; in the CMM the same
  instinct is **refuse by name, never substitute**.
- **No iccMAX execution, no display calibration.** Profile *creation*
  was reversed by the operator → Pass 10, DL-008.
- **lcms2 is the oracle, never a dependency** — subprocess only, pinned
  by commit hash (DL-001). `tools/difftest` stays outside the workspace.
- **DL-003** duplicate tags · **DL-004** the ⚠ provisional 1.0 anchor ·
  **DL-005** exact invariants for legacy Lab · **DL-007** HDR in scope ·
  **DL-010 / NA-001** the rational breakpoint · **DL-011 / DL-012** the
  tag-type selector, the predicted disagreement measured **absent** ·
  **DL-013** lcms2's forced BPC · **DL-014** the terms for citing
  ICC.1:2022 · **DL-015 / NA-004** the `pow` guard · **DL-016** exact
  values at sample points.
- **DL-017** *(new)* — **`tools/difftest` may path-depend on iccce's
  crates.** Harness → code under test; four conditions, all load-bearing;
  the no-crate-reaches-lcms2 invariant untouched.
- **DL-018** *(new)* — **an upper-bound gate on a deliberate cost must be
  paired with a prediction pin** and a sensitivity control, or deleting
  the requirement makes the gate greener. **Its scope limit is part of
  the entry**: the Pass 3 pin does *not* make the F.8–F.16 **ordering**
  falsifiable, because iccce clamps at three sites.

### Everything measured against lcms2 is scoped to commit `21c582a`

Moving the pin is a **licence** event (DL-001) **and** a behavioural one
(DL-012, DL-013): **NC-019, NC-020, NC-021 and now NC-034 … NC-037,
NC-040, NC-041, NC-043 must be re-run, not re-read.**

---

## Method reminders that stay load-bearing

1. **A wrong colour looks exactly like a right one.** This Pass's live
   examples: an absolute-intent ratio inverted (clause 6.2.3's own prose
   has it backwards) produces perfectly plausible colour; a
   clamp-ordering error makes *"the gamut boundary subtly the wrong
   shape"*, which the corpus flags **Quiet**.
2. **Never write colour maths from memory.** The absolute-intent formula
   was **refused until sourced**, and it was sourced the same day.
   Tetrahedral interpolation is now the live instance of the same rule.
3. **Expected values come from the literature**, and a cross-check is a
   **weaker** claim that must be labelled as one — especially here,
   where both implementations draw on the same single-source lineage.
4. **Every approximation is named and measured.** **NA-006 is named and
   NOT measured**, and the doc comment that says otherwise is reported.
   The ledger records the *kind* of departure in the row: from normative
   text (NA-001), inside a stated non-requirement (NA-004), inside a
   **silence** (NA-006).
5. **Tolerances are justified, not tuned.** Two worked examples landed
   this Pass: a justification **tested by emulating the oracle's
   arithmetic** (290× collapse), and one **corrected after a failure**
   with the mechanism predicted in closed form to 0.03 %.
6. **Coverage is part of every claim.** *"Pass 3 verified"* means one
   pair, one intent, one direction, 133 points, one platform, both files
   **v2**. Everything outside `tools/difftest/README.md` §13.8's
   sentence is not verified.
7. **Do not assert unmeasured facts about the environment.** *verified* /
   *reported* / *unverified* are distinguished on purpose. **No agent
   here has ever run a git command**; every commit hash is reported.
8. **Check the live source — including your own last filing, and
   including the tree's shape.** This session found a source file that
   the dispatch did not mention and that an earlier enumeration in the
   same session did not show. **A tree can move while it is being
   described.**

---

## The agents

- **`icc-engineer`** — lead. Be this agent if orchestrating.
- **`icc-spec-librarian`** — the standards corpus. Dispatch for *every*
  sourcing question. **Owes** tetrahedral, the NC-043 clamping question,
  IEC 61966-2-1, the ITU terms, and A4b.
- **`icc-conformance`** — the oracle, the fixtures, the tolerance
  budget. **Owes** §13.10's four items, the ground-truth row, the
  remaining `TOLERANCES.md` sections, the `ncl2`/B2A tests, and the
  forced-BPC decision.
- **`icc-librarian`** — ROADMAP, decision log, session log, and
  `NUMERIC_CLAIMS.md`. **No shell** — a dispatch to it must carry its
  evidence.

Dispatch them freely, and in parallel on disjoint file sets; no
permission is needed to dispatch an agent to read, analyse or draft.
