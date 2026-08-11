# NEXT SESSION — start here

**Written 2026-08-11 by `icc-librarian`, at the Pass 5 completion
filing (the eleventh of the same calendar day).** Replaces the Pass 4b
edition entirely. Overwrite this file once acted on.

Read order: this file → `docs/ROADMAP.md` (the **Pass 5 completion
record**, then the **second Pass 6 annotation**, then Pass 7) →
`docs/NUMERIC_CLAIMS.md` (**§2.8** → **§3.12**, starting with its
coverage box, then **§3.12.2** the class judgement and **§3.12.3** the
negative result → the dated notes under **NA-009** and **NA-010** →
**§7.8**) → `docs/ARCHITECTURE.md` §5 (**twenty-three** entries;
**DL-022** and **DL-023** are new) → `docs/SESSION_LOG.md` (eleven
entries, all 2026-08-11; the eleventh is this work) →
`tools/difftest/README.md` **§16** and `docs/TOLERANCES.md` **§3.5** and
**§6.5** (the evidence, both `icc-conformance`'s) → the corpus's
`icc__ref__bpc.md`.

---

## Where the project actually is

**Pass 0 done. Pass 1 core complete. Pass 2 DONE. Pass 3 DONE.
★ Pass 4 IN PROGRESS — every direction measured, done-when NOT met, by
exactly two items. ★★ Pass 5 DONE — done-when MET on stated terms.**
All on 2026-08-11.

| | Commit *(all **reported** — no agent here has ever run git)* |
|---|---|
| Pass 0 · Pass 1 | `f976a0e` · `7313c5b` |
| Pass 2 batch 1 · difftest harness · batch 2 | `b35a12e` · `bfd6b1e` · `d40d601` |
| Pass 3 core · `transform` · audits · filing | `c4038eb` · `051707f` · `55772c6` · `a9618fe` |
| CLUT · PCS encodings · absolute intent · Pass 3 differential | `fc5ff58` · `0843094` · `6873df1` · `986dae6` |
| Pass 3 closure · stages 1–3 · CLI · doc catch-up · Pass 4 differential · gen-profiles | `19a3b17` · `9aa1bca` · `63874f9` · `b3f4388` · `490191b` · `db60e92` · `d9e0b82` · `edcb60e` |
| gen-profiles + 38 fixtures + GP-001 found · GP-001 fixed + `mAB `/`mBA ` · grayTRC F.2 + filing | `7576cfa` · `2e98cfd` · `97ad9fa` |
| the Pass 4b filing + gray-through-`Chain` · the corpus's 7th pass in code · the Pass 4b measurements | `9e2e29e` · `a0310c7` · `3d0c183` |
| **the Pass 4b filing committed + the `lib.rs` §Status fix** | **`8be1ed3`** |
| **★ BPC core — and TWO RED COMMITS, each corrected by the next** | **`70411dd` → `a36abaf`** · **`6ea1b3d` → `812a215`** |
| **the `--bpc` CLI · ★ the Pass 5 measurements** | **`46f16e8`** · **`df3a233`** |

### The one thing to read before touching anything

**Pass 5's done-when is MET — and the sentence that must travel with it
is not *"iccce's BPC matches lcms2's"*, it is *"on the MAP, the
DIRECTION and the PIPELINE — never on the ESTIMATORS."***

- **The estimators were never discriminated, and that was known in
  advance.** Everywhere iccce does BPC at all, lcms2's estimator reduces
  to the same two values: `XYZ (0,0,0)` on every matrix/TRC or gray side
  in reach (every TRC in the corpus has `trc(0) = 0`), and the same
  **A41** triple on a v4 LUT side at perceptual. **DL-023.**
- **The instrument that would close it does not exist**: a synthetic v4
  RGB-or-gray LUT fixture with a **non-zero device black**.
- **lcms2 silently does no BPC at all below `IsEmptyLayer`'s 0,002**
  (≈0,41 `L*` between the blacks). **iccce deliberately has no such
  threshold.** The figure is **solved for, not observed**.
- **iccce never forces BPC; lcms2 forces it for a v4 destination at
  perceptual.** 3,137 348 `L*`, **REPORTED NOT GRADED**, now **DL-022**.
- **Pass 4 did not move.** Saturation in B2A and ICC-absolute are
  exactly where they were. **Pass 5's saturation gap is a different
  item** — iccce's subset has no LUT saturation arm at all.

### The Pass 5 numbers — quote them with their scope

**§A — the map** (no profiles, no oracle; runs anywhere):

- **`BpcScale(0 → PB)` vs ICC.1:2022 6.3.4.3's printed equation, 1005
  PCS values: 1,110×10⁻¹⁶** (tol 1×10⁻¹⁴);
- **vs a Gaussian elimination on Maria (2013) §4.2's two constraints,
  20 000 draws: 3,331×10⁻¹⁶**; the constraints hold under iccce's own
  map to the same figure; equal blacks are the **exact** identity;
- **`IsEmptyLayer` discriminant 0,015 342 = 7,7× lcms2's threshold** —
  reported, **READ not RUN**.

**§B — S2, `PB → 0`** (fixture → sRGB, perceptual, 128 CMYK, 10
excluded):

- **BPC-off baseline 1,012 157×10⁻⁴** device (graded **first**, on
  purpose) · **BPC-on 1,110 588×10⁻⁴** (tol 2,5×10⁻⁴);
- **1,262 374×10⁻² ΔE2000** on, 1,962 920×10⁻² off;
- **direction: 0,0 exactly**, largest fall 4,304×10⁻² device =
  **3,5159 ΔE2000** ⇒ **sensitivity 388×**;
- **A41 priced in a pipeline: 0,050 201 ΔE2000 / 0,037 416 ΔE76 /
  0,005 364 ΔL\*** — the corpus corroborated to 2×10⁻⁵ by an
  independent route.

**§C — S3, `0 → PB`** (sRGB → fixture, perceptual, 213 RGB):

- **iccce `--bpc` vs BOTH lcms2 arms: 4,600×10⁻⁵** (tol 1×10⁻⁴) ⇒
  **sensitivity 682×**;
- **the lift at device black vs a CLOSED FORM: 9,504 522×10⁻⁸** —
  *below* the print floor its bound came from — **and lcms2 against the
  same closed form: 9,046 508×10⁻⁷**, the third reading;
- **the policy, unasked vs unasked: 3,137 3×10⁻² device = 3,137 348
  `L*`** — **reported, not graded**.

**§D/§E** — S1 and S4 null (S4 is corpus trap **T5**, and iccce reaches
the same no-op **by a different route**); S5 and S6 **refusals, graded
on their exact wording**.

---

## ★★ DL-022 and DL-023, because they change how the next Pass writes its numbers

- **DL-022 — iccce NEVER forces BPC.** A deliberate divergence with a
  **user-visible** consequence: two correct CMMs give different pictures
  by default, silently. Any comparison that does not account for it
  **measures iccce's policy and reports it as a tolerance**.
- **DL-023 — say what the two implementations were FREE to disagree
  about, from their sources, BEFORE the run.** Publish the negative
  result it produces; name the instrument that would close it; and
  **print the sensitivity ratio** (Pass 5's: 388× and 682×, free,
  because the off arm is the baseline).

---

## Then: the work, in dependency order

### 1. ★★ Pass 6 — performance. It is next, and its precondition is finally met

**Done when**: a 300 DPI A4 CMYK→RGB conversion completes in a stated
time, and the compiled path's error against the uncompiled one is
measured and stated.

**Why it is legitimately next.** Rule 8 says *optimise only after
correct*. **Every stage a compiled transform would fold in has now been
measured against another implementation in the direction it will be
used** — `mft2` A2B, `mft1` B2A, the v4 `mAB `/`mBA ` element pipeline,
matrix/TRC both directions, the F.2 grayTRC, **and now BPC**, which is
the stage that was still unmeasured when Pass 4b said this. **This is
the first Pass in the project whose precondition holds across the whole
transform surface.**

**What it inherits, and the third one is the sharpest:**

- **DL-018** — an upper bound on a *deliberate* cost needs a
  **prediction pin** and a **sensitivity control**, or deleting
  precision makes the gate greener. A compiled path is exactly that
  shape.
- **DL-021** — a compiled path measured in one direction says **nothing**
  about the other. Name the direction and the tag type on every claim.
- **★ DL-023** — *"the compiled path agrees with the reference path"* is
  the **most likely null-by-construction row this project will ever
  write**: a transform compiled by *sampling* the reference path can be
  identical to it over the sampled set by construction. **State what the
  two arms were free to differ on, before choosing a tolerance**, and
  **print the sensitivity ratio** — how much compilation *could* move
  the answer, beside how much it did.
- The error row is a **`self-consistency`** row (§1) — worthless as
  correctness evidence however small, and to be labelled so.

**What is NOT satisfied, and should be said in the Pass's own block**:
Pass 4's done-when is still open at ICC-absolute (**A4b**,
operator-blocked) and at saturation in B2A; **no ground-truth row exists
for any transform in this project**; and there is still no `cargo test`
outcome on record.

### 2. ★ The instrument Pass 5 named — `tools/gen-profiles`

**A synthetic v4 RGB-or-gray LUT fixture with a NON-ZERO device black.**
It is the **only** thing that can discriminate the two black-point
**estimators**, and therefore the only route to **NA-009's** cost. Every
profile in reach has `trc(0) = 0`; `fixtures/synthetic/` holds 38 `.icc`
and one v4 LUT, black zero *(verified — enumerated)*. Same shape as the
GP-001 arc (**DL-020**): a doubt the corpus cannot discharge, discharged
by bytes this project authors.

### 3. ★ Finish Pass 4's cheap half — `icc-conformance`

- **Saturation in the B2A direction** (`B2A2` is a distinct third
  table). **Not the same item as Pass 5's saturation gap**, which is a
  *capability* gap in iccce's BPC estimation subset.
- **ICC-absolute through a LUT destination** — never exercised. Still
  **reported, not graded** until A4b clears (DL-019); running it
  measures the *arithmetic*, which is separable from the *authority*.
- **A gray profile as DESTINATION**, over non-neutral PCS input — the
  only thing that measures **NA-008**. *(And note Pass 5 leaves the gray
  side of iccce's own BPC subset unexercised too — a second gray hole.)*
- **The M3 out-of-gamut excursion count**, not recorded on the Pass 4b
  run.
- **README §15.5's build-commit line** (names `97ad9fa`, which predates
  the clamp change).

### 4. ★ The apparatus gaps Pass 5 left — `icc-conformance`

- **`tools/difftest/src/pass5.rs` has NO `#[test]` declarations**
  *(verified — `tools/` grepped with no result limit; `pass3.rs` 7,
  `pass4.rs` 7, `pass4b.rs` 8, `pass5.rs` **none**)*. **Fourteen ledger
  rows rest on two grids that nothing pins.**
- **A reported runner result for §16.** The whole-suite `pass=90 fail=0
  skip=3 error=0` is at the README's head, but **§16 states no
  `pass=`/`fail=` line of its own**. Pass 5's 26-record count is
  currently a **subtraction**, not a report.
- **★ A `cargo test --workspace` count.** Six filings on, **NC-057 …
  NC-061 still have no reported outcome at all** — and this session
  produced two commits whose messages claimed one falsely. **103
  `#[test]` declarations exist under `crates/`** *(verified — counted)*;
  that is not a pass result.

### 5. A4b and the operator downloads — the two things this project cannot do for itself

**Unchanged: A4b is UNVERIFIED.** Only **`ICC.1:2001-04`** settles it,
and it gates the 11 ΔE absolute-intent divergence **and** Pass 4's last
done-when clause. **New this Pass**: **`AdobeBPC.pdf` / ICC WP40 / ISO
18619** is what would let **NC-100** move from *reported* to *graded* —
is BPC's *applicability* specified as a function of intent and version,
or only its *black-point detection*? Both are **browser downloads by
Ken**; agent tools are ToS-barred or blocked.

---

## Owed work, carried explicitly

### 1. `icc-conformance`

- Everything in §3 and §4 above.
- **Whether to re-grade NC-077** (the encoded-PCS overflow) now that the
  matrix-output half of its clause question is answered — **its file,
  its call**.
- **A synthetic `lut8` fixture wired into the suite** —
  `fixtures/synthetic/v2-cmyk-mft1-lab.icc` exists unused, and every §A
  row skips without the Windows colour directory.
- **`tools/difftest/README.md` §14.7's record decomposition** (7 + 1 +
  28 graded, 31 emitted, 3 skipped = 36 — not 8/1/27/30).
- **`TOLERANCES.md` §3.2 (Pass 2)** and **§6's coverage table**.
- **An instrument check on iccce's ΔE ruler for the sRGB destination** —
  §C of Pass 4b priced *lcms2's* sRGB destination model, a different
  quantity.
- **A re-run of the Pass 2 machine sweep** against a post-GP-001 build,
  with per-tag-type counts.
- **A behavioural test of `ncl2`** legacy-Lab decoding.

### 2. `icc-spec-librarian`

- **★ The forcing-policy question** (NC-100 / DL-022) — blocked on the
  operator download, but the question should be *framed and queued*.
- **★ The clamp question, NARROWED**: must the final `B` curves' output
  be clipped to the encodable PCS range of 6.3.4.2, and does 10.18's
  domain bind the *evaluator* or only describe the stored samples?
- **★ Four corpus rows now owed**: the **M2 correction** (destination
  version); the **trilinear override** in `_cmsReadOutputLUT` (beside
  M4: *same file, opposite direction, opposite answer*); **the
  `IsEmptyLayer` 0,002 threshold** (§7.2's list came from `cmssamp.c`,
  this constant is in `cmscnvrt.c`); and **A41's ΔE2000 = 0,050 201**
  (the corpus computed ΔE76 and ΔL* only).
- **★ The tier question that now decides a ledger CLASS**: is
  `icc__ref__bpc.md` §2/§3 `primary_spec` or `cross_verified_2src`? At
  `primary_spec`, **NC-084/NC-086 become `normative-rule-conformance`
  rows**. `bpc.rs` heads the clause **"PRIMARY-SOURCED"**; the corpus's
  `evidence:` line does not *(both verified)*.
- **★ A4b** — top corpus gap, gating a finding and a done-when.
- **IEC 61966-2-1's sRGB primaries** — still **the** cheapest route to
  the project's first **ground-truth** row for a transform. **Nobody has
  dispatched for it.**
- **The ITU terms determination** before any BT.709 fetch (DL-007).

### 3. `icc-engineer`

- **Decide what `named_color.rs` is for** — Pass 7's core, in the tree,
  reachable from nothing.
- **Whether iccce should implement lcms2's `EvalNInputs` geometry at
  all** — DL-021 makes it two choices, not one.
- **A disclosure for A4c**, if wanted (the colorant-sum-vs-`wtpt`
  inconsistency is detectable from the file today, rule 6).
- **★ A mechanical commit gate.** Two commits this session shipped red
  under green messages. The remedy in the personal_rag lesson is not
  attentional: `cargo test --workspace -q > log 2>&1; TESTS=$?` then
  gate on `$TESTS`, display separately. **Exit codes compose; a pipe
  does not.**
- *(Closed this session: `lib.rs`'s §Status, stale four times, is now
  correct — verified.)*

### 4. `icc-librarian` / whoever files next

- **The DL-014 citation audit**, whose surface grew again and which now
  **decides a ledger class** (above). `iccce-color` / `iccce-profile`
  have **never** been swept.
- **A re-read of README §15** — two §7.7 items were carried into §7.8
  without re-verification, and a carried item is a claim with a date.
- **Observed residuals** for Pass 1's rows and for NC-032.
- **A ground-truth row for chromatic adaptation** — the largest hole in
  Pass 1; NA-002 still not due.
- **A Linux run of anything at all.**

### 5. The operator

| Document | What it settles |
|---|---|
| **★ `ICC.1:2001-04` (v2)** | **A4b — the 11 ΔE question AND Pass 4's last done-when clause**, plus A1b, A2, A34, A39c, and `textDescriptionType` |
| **★ `AdobeBPC.pdf` / ICC WP40 / ISO 18619** | **whether BPC's applicability is specified at all** — the only thing that can move **NC-100 / DL-022** from *reported* to *graded*, in either direction. **`pdfa.org`'s App Note is a browser route** |
| **IEC 61966-2-1** | the **first ground-truth row for a transform**, and the end of the single-source sRGB/D65 lineage |
| **ICC's published D65→D50 `chad` values** (Annex E.4.2) | the adaptation ground-truth hole |
| **`ICC.1:2010-12` (v4.3)** | A31 / D10 — `parametricCurveType` Table 68 across editions |
| **ITU-R BT.709** | a second source for sRGB primaries and D65 (blocked on the terms determination) |

**Each row is a claim about what a document contains.** Treat *"it would
settle X"* as a prediction until the document is open.

---

## Decisions already made — do not re-litigate

- **MIT**, dependencies permissive, **publishing is the operator's act**
  (rule 9). DL-009 records an *intent*; **intent is not authorisation.**
- **`iccce-color` depends on nothing** and contains no ICC. **The
  fixture generator depends on nothing either.**
- **The parser reports, it does not repair**; in the CMM the same
  instinct is **refuse by name, never substitute** — and Pass 5 **grades**
  two refusals on their exact wording.
- **lcms2 is the oracle, never a dependency** — subprocess only, pinned
  by commit hash (DL-001).
- **DL-003** duplicate tags · **DL-004** the ⚠ provisional 1.0 anchor ·
  **DL-005** exact invariants for legacy Lab · **DL-007** HDR in scope ·
  **DL-008** profile creation in scope · **DL-010 / NA-001** the
  rational breakpoint · **DL-011 / DL-012** the tag-type selector ·
  **DL-013** lcms2's forced BPC *(keyed by the **destination** — DL-021,
  and now measured end-to-end — DL-022)* · **DL-014** the terms for
  citing ICC.1:2022 · **DL-015 / NA-004** the `pow` guard · **DL-016**
  exact values at sample points · **DL-017** the harness may path-depend
  on iccce's crates · **DL-018** a prediction pin for an upper-bound
  gate · **DL-019** report-not-grade when the mechanism is known and the
  authority is not · **DL-020** refuse-don't-guess, discharged by an
  independently authored fixture · **DL-021** a behaviour is a fact
  about **one direction and one path**.
- **DL-022** *(new)* — **iccce never forces BPC**; it is an explicit
  caller act, a deliberate divergence from the oracle worth 3,137 348
  `L*` at black, **reported not graded**, with a **user-visible**
  consequence.
- **DL-023** *(new)* — **state what the two implementations were free to
  disagree about, from their sources, before the run**; publish the
  negative result; name the instrument that would close it; print the
  sensitivity ratio.

### Everything measured against lcms2 is scoped to commit `21c582a`

Moving the pin is a **licence** event (DL-001) **and** a behavioural
one. **Re-run, not re-read:** NC-019 … NC-021, NC-034 … NC-037, NC-040,
NC-041, NC-043, NC-044 … NC-050, NC-053 … NC-057, NC-062 … NC-083
**and now NC-088 … NC-096, NC-099 … NC-102**. **The sharp ones are
NC-050, NC-056, NC-082 and now NC-088** — each is a *transcription* of
lcms2's internals, so a retuned interpolator, resampler or threshold
invalidates them **silently**, by continuing to reproduce the old lcms2
perfectly. **NC-084 … NC-087 are the only rows in the ledger the pin
cannot touch**, because no implementation is in them.

---

## Method reminders that stay load-bearing

1. **A wrong colour looks exactly like a right one.** Pass 5's instance:
   **lcms2 does no BPC at all below 0,002** — a future comparison with
   two close blacks will show a residual that looks like somebody's bug.
2. **★ Ask what the two sides were free to disagree about.** Pass 5's
   six agreements are about the map, the direction and the pipeline, and
   **about nothing else**. Derived from sources **before** the run
   (DL-023).
3. **★ Print the sensitivity ratio.** *"They agree to 1,1×10⁻⁴"* means
   nothing until *"BPC moves this by 3,5 ΔE2000"* sits beside it. **388×
   and 682×.**
4. **Say which direction.** DL-021, and Pass 5 used it twice — the
   forcing policy's *sign* was diagnosed by measuring the other
   direction.
5. **Verify against the live source, not the dispatch.** This filing's
   instance: the dispatch called 4,6×10⁻⁵ *"the policy arm"*. **It is
   the cross-check arm**; the policy row is 3,137 3×10⁻² and is
   **ungraded**.
6. **A gate figure is a claim.** **Two commits this session claimed a
   green suite falsely**, because a `grep` and then a `tail` supplied
   their own exit codes. **Exit codes compose; text matching does not.**
7. **A class is not raised by how good the number looks.** NC-084 is
   `derived-expectation` until a corpus tier line says otherwise.
8. **Coverage is part of every claim.** *"Pass 5 is done"* means **the
   map, the direction and the pipeline**, on **one synthetic fixture**,
   at **one intent**, in **two directions**, on **one platform**, at
   **one pin** — and **not the estimators**.
9. **Do not assert unmeasured facts about the environment.** **No agent
   here has ever run a git command**; every commit hash above is
   reported, and this session is the proof that a commit message can be
   wrong.
10. **Check your own draft.** The rule that has caught something at
    three consecutive filings.

---

## The agents

- **`icc-engineer`** — lead. Be this agent if orchestrating.
- **`icc-spec-librarian`** — the standards corpus. **Owes** the
  forcing-policy framing, the narrowed clamp question, four corpus rows,
  the tier question that decides a ledger class, A4b, IEC 61966-2-1 and
  the ITU terms.
- **`icc-conformance`** — the oracle, the fixtures, the tolerance
  budget. **Owes** saturation and ICC-absolute in B2A, gray as a
  destination, `pass5.rs`'s missing unit tests, a §16 runner result, a
  `cargo test` count, the NC-077 grading call, a `lut8` fixture in the
  suite, §14.7's decomposition, the sRGB ruler check and the remaining
  `TOLERANCES.md` sections.
- **`icc-librarian`** — ROADMAP, decision log, session log, and
  `NUMERIC_CLAIMS.md`. **No shell** — a dispatch to it must carry its
  evidence.

Dispatch them freely, and in parallel on disjoint file sets; no
permission is needed to dispatch an agent to read, analyse or draft.
