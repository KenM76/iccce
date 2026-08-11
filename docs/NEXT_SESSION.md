# NEXT SESSION — start here

**Written 2026-08-11 by `icc-librarian`, at the Pass 4b filing (the
tenth of the same calendar day).** Replaces the evaluation-surface
edition entirely. Overwrite this file once acted on.

Read order: this file → `docs/ROADMAP.md` (the **Pass 4b progress
block**, then the **second Pass 5 annotation**, then the Pass 6 and
Pass 7 annotations) → `docs/NUMERIC_CLAIMS.md` (**§2.7** → **§3.11**,
starting with its coverage box → **§3.11.5**, the overflow finding and
the corpus correction → the dated notes under **NA-006** and **NA-008** →
**NA-009/NA-010** → **§7.7**) → `docs/ARCHITECTURE.md` §5
(**twenty-one** entries; **DL-021** is new, and **DL-020** has a dated
status note) → `docs/SESSION_LOG.md` (ten entries, all 2026-08-11; the
tenth is this work) → `tools/difftest/README.md` **§15** and
`docs/TOLERANCES.md` **§3.4.4** and **§4** (the evidence, both
`icc-conformance`'s) → the corpus's
`icc__type__lutAtoB_lutBtoA.md` (**seventh pass**) and
`icc__ref__bpc.md` (**new**).

---

## Where the project actually is

**Pass 0 done. Pass 1 core complete. Pass 2 DONE. Pass 3 DONE.
★ Pass 4 IN PROGRESS — every direction is now MEASURED and the done-when
is still NOT met, by exactly two items.** All on 2026-08-11.

| | Commit *(all **reported** — no agent here has ever run git)* |
|---|---|
| Pass 0 · Pass 1 | `f976a0e` · `7313c5b` |
| Pass 2 batch 1 · difftest harness · batch 2 | `b35a12e` · `bfd6b1e` · `d40d601` |
| Pass 3 core · `transform` · audits · filing | `c4038eb` · `051707f` · `55772c6` · `a9618fe` |
| CLUT · PCS encodings · absolute intent · Pass 3 differential | `fc5ff58` · `0843094` · `6873df1` · `986dae6` |
| Pass 3 closure · stages 1–3 · CLI · doc catch-up · Pass 4 differential · the swept-in gen-profiles | `19a3b17` · `9aa1bca` · `63874f9` · `b3f4388` · `490191b` · `db60e92` · `d9e0b82` · `edcb60e` |
| gen-profiles + 38 fixtures + GP-001 found · GP-001 fixed + `mAB `/`mBA ` · grayTRC F.2 + filing | `7576cfa` · `2e98cfd` · `97ad9fa` |
| **the last filing + gray-through-`Chain` + the GP-001 banner** | **`9e2e29e`** |
| **the corpus's 7th pass in code: matrix-output clamp, `offsetB == 0`, `mluc` wording** | **`a0310c7`** |
| **★ the Pass 4b measurements** | **`3d0c183`** |

### The one thing to read before touching anything

**Pass 4b measured the three directions Pass 4 left blank — and the
useful sentence is not "everything is measured", it is *which two things
are not*.**

- **Saturation was not run in any of the three directions.** In A2B it
  shares tag data with perceptual (`A2B0`/`A2B2` are one block in SWOP),
  so it is the same bytes through the same code — **but `B2A2` is a
  genuinely distinct third table and has never been evaluated.**
- **ICC-absolute was not run in any of them either**, and it is
  **blocked on `ICC.1:2001-04`** via **A4b** (DL-019). **This is the one
  clause of Pass 4's done-when that this project cannot close for
  itself.**
- **The v4 evidence is one synthetic fixture.** A sweep of all **40**
  profiles in this machine's colour directory found **zero
  `mAB `/`mBA ` tags** *(reported)*. There is no real v4 LUT profile
  here at any price.
- **The gray comparison did NOT price NA-008.** §C runs gray as the
  **source**; NA-008 lives in the gray **destination** path.
- **Pass 4 still has no ground-truth row**, and `derived-expectation`
  is **not** one.

### The Pass 4b numbers — quote them with their gate and their direction

**§A — B2A** (sRGB → SWOP, `mft1`, 213 RGB + 258 Lab points, perceptual
and media-relative, `-c0`):

- **device vs lcms2: 1,330×10⁻⁴** against **5×10⁻⁴** — the gate is an
  envelope from lcms2's own roundings and the observation matches it to
  **0,02 %**;
- **with that arithmetic modelled: 3,10×10⁻⁵ = 2,03 lsb of 1/65535**,
  three times independently — **this is the row that claims agreement**;
- **counterfactual (tetrahedral): 99–139× larger** — the proof the
  comparison could see a geometry difference if there were one.

**§B — the v4 fixture** (`mAB ` 4→3 ragged, `mBA ` 3→4, media-relative):

- **iccce vs the closed form: 2,842×10⁻¹⁴ `L*` / 2,220×10⁻¹⁶ device** —
  `f64` noise, and **the strongest LUT claim in the ledger**;
- **lcms2 vs the same closed form: 2,325×10⁻³ / 1,873×10⁻⁵** — the third
  reading, which is what stops the fixture and the derivation being
  wrong together;
- **encoded-PCS overflow: 0,6117 ΔE2000 on 10 of 128 points, REPORTED
  NOT GRADED**;
- **forced BPC by destination version: 0,0 vs 3,137×10⁻²**.

**§C — gray** (`ewgray22.icm` → sRGB, 69 points):

- **9,686×10⁻⁵ device / 2,169×10⁻² ΔE2000**;
- **modelled: 2,121×10⁻⁷ — a 457× collapse, below the oracle's print
  floor.** The residual is *reproduced*, not merely bounded.

---

## ★★ DL-021, because it changes how three existing numbers must be quoted

*A measured implementation behaviour is a fact about the direction and
the path it was measured in, until it is measured in the others.* Three
instances, all lcms2, all in one file, all previously written here as
unqualified rules:

1. **Interpolation.** `_cmsReadOutputLUT` forces **trilinear** for any
   Lab-PCS LUT, and trilinear over three inputs **is** iccce's n-linear.
   **NA-006's 1,5741 ΔE2000 is an A2B number; the B2A envelope is
   exactly zero.** Never quote it without the direction.
2. **Forced BPC.** Keyed by the **destination** profile's version, not
   "the profile's". DL-013 and corpus **M2** are half-stated.
3. **The legacy Lab encoding.** Applied for `lut16Type` and **not** for
   `lut8Type`. iccce's `Lab8` codec agrees exactly — and the mistake
   would have cost **≈0,2 ΔE2000**, *below* the perceptibility anchor
   and invisible to any ΔE-graded suite.

**The defect is in this project's transcription, not in lcms2.** Each
behaviour has a rationale in its own place.

---

## Then: the work, in dependency order

### 1. ★ Finish Pass 4's cheap half — `icc-conformance`

- **Saturation in the B2A direction.** `B2A2` exists and is a third
  distinct table; §A's apparatus already runs, so this is a short run.
- **ICC-absolute through a LUT destination** — never exercised, and the
  one case where the D.6/D.7 composite is applied **before** the PCS is
  encoded rather than after. It will still be **reported, not graded**
  until A4b clears (DL-019), and running it is still worth it: it
  measures the *arithmetic*, which is separable from the *authority*.
- **The M3 out-of-gamut excursion count**, which §A's 48 saturated-hue
  Lab points could have produced and which **was not recorded**.
- **README §15.5's build-commit line** (it names `97ad9fa`, which
  predates the clamp change).

### 2. ★ A gray profile as a DESTINATION — the only thing that measures NA-008

Non-neutral PCS input, `Y/Yn` vs `L*/100` vs lcms2, in ΔE2000 after
re-expansion. `fixtures/synthetic/v2-gray-curv-gamma.icc` exists and
`Chain` selects `DestModel::Gray`. **On the neutral axis the cost is
exactly zero**, which is why every gray row so far is blind to it.

### 3. ★ Pass 5 — and its sourcing has ALREADY LANDED

**Do not re-dispatch for it, and do not re-write it.** The corpus
carries **`icc__ref__bpc.md`** and its headline is that **the BPC
scaling map is in ICC.1:2022 after all**, at clause **6.3.4.3** under
another name — so the scaling half cites the specification, not the
oracle. **And the code half is largely done**: `bpc.rs` (4 tests),
**`Chain::with_bpc()`**, and **`iccce transform --bpc`** on the shipped
binary, refusing **by name** at the absolute intent
(`BpcNotApplicable` — BPC presupposes both whites already at D50) and
outside the estimation subset (`BpcEstimationUnsupported`, *"notably v2
LUT sources, where lcms2 runs an unattributed Lab ridge search"*).
**What Pass 5 is missing is MEASUREMENT.** Specifically:

- **`TOLERANCES.md` §3.5's two blank rows are now a GAP**, not a correct
  absence, and **NA-009's and NA-010's costs are OWED** — reachable
  through the shipped binary is exactly the condition that makes a cost
  come due.
- **A tolerance**, remembering there is **no BPC conformance test with a
  fixed expected value** — the grade is agreement with lcms2, an
  `implementation-cross-check`, and rule 3 requires it labelled as
  weaker than ground truth however green it is.
- **★ iccce NEVER forces BPC and lcms2 sometimes does.** iccce's is *"an
  explicit caller act, which is itself a recorded policy difference from
  the oracle"*. **A comparison that does not account for it measures
  iccce's policy and calls the answer a tolerance.** One direction is
  already priced: **NC-078** (3,137×10⁻² device) and **NC-020**
  (≈3,15 `L*`).
- **The `-b`-on/`-b`-off pairing must name which side of the chain the
  v4 profile is on** (DL-021, instance 2).
- **The estimation subset is where the refusals will fire** — a v2 LUT
  source at anything but v4-perceptual returns
  `BpcEstimationUnsupported`. **That is correct behaviour, not a bug to
  route around**, and a Pass 5 run should record how often it fires
  rather than choosing profiles that avoid it.

### 4. Pass 6 — now legitimately next after Pass 5

Rule 8's precondition is much closer to satisfied than it has ever been:
**every evaluation path a compiled transform would compile has now been
measured against another implementation in the direction it will be
used.** Inherit two method rules: **DL-018** (an upper bound on a
deliberate cost needs a prediction pin *and* a sensitivity control) and
**DL-021** (a compiled-path error measured in one direction says nothing
about the other).

### 5. A4b — `icc-spec-librarian`, and the operator

**Unchanged: UNVERIFIED** *(verified this session)*. Only
**`ICC.1:2001-04`** settles it; the ICC errata are recorded as
**unreachable by compliant means**. It now gates **two** things: the
11 ΔE absolute-intent divergence (NC-053/NC-054) **and the last
unmeetable clause of Pass 4's done-when**.

---

## Owed work, carried explicitly

### 1. `icc-conformance`

- The four items in **§1** above, plus the **gray-as-destination** run.
- **Whether to re-grade NC-077** (the encoded-PCS overflow) now that the
  matrix-output half of its clause question is answered — **its file,
  its call**; see §3.11.5.
- **A synthetic `lut8` fixture wired into the suite** — every §A row
  skips without the Windows colour directory, and
  `fixtures/synthetic/v2-cmyk-mft1-lab.icc` exists unused.
- **A `cargo test --workspace` count.** Five filings on, **NC-057 …
  NC-061 still have no reported outcome at all**.
- **`tools/difftest/README.md` §14.7's record decomposition** (7 + 1 +
  28 graded, 31 emitted, 3 skipped = 36 — not 8/1/27/30).
- **`TOLERANCES.md` §3.2 (Pass 2)** and **§6's coverage table**.
- **An instrument check on iccce's ΔE ruler for the sRGB destination.**
  §C prices *lcms2's* sRGB destination model, which is a different
  quantity; the ruler bound is still the one measured on Adobe RGB.
- **A re-run of the Pass 2 machine sweep** against a post-GP-001 build,
  with per-tag-type counts. (Pass 4b's 40-profile sweep searched for
  `mAB `/`mBA ` tags and does not discharge it.)
- **A behavioural test of `ncl2`** legacy-Lab decoding — now that a
  consumer exists (`named_color.rs`), it is cheaper than it was.

### 2. `icc-spec-librarian`

- **★ The clamp question, NARROWED to its second half.** The
  matrix-output half is **answered** by the corpus's own seventh pass
  (10.12.5/10.13.3, `shall be clipped`, `primary_spec`). What remains:
  **must the final `B` curves' output be clipped to the encodable PCS
  range of 6.3.4.2, and does 10.18's domain bind the *evaluator* or only
  describe the stored samples?** Put that, not the original two-part
  question.
- **The M2 correction** — forced BPC is keyed by the **destination**
  profile's version.
- **A corpus row for the trilinear override** in `_cmsReadOutputLUT`,
  beside M4: *same file, opposite direction, opposite answer.*
- **★ A4b** — top corpus gap, now gating a done-when as well as a
  finding.
- **IEC 61966-2-1's sRGB primaries** — still **the** cheapest route to
  the project's first **ground-truth** row for a transform. **Nobody has
  dispatched for it.**
- **The ITU terms determination** before any BT.709 fetch (DL-007).

### 3. `icc-engineer`

- **`iccce-cmm/src/lib.rs`'s §Status is stale for the FOURTH time** —
  *"Still to come: **BPC (Pass 5)**"*, and `bpc` missing from the module
  list, in a crate that declares `pub mod bpc;` and wires it into
  `Chain` *(verified — read)*. **Its own standing instruction saved it**
  (*"if a module below contradicts it, trust the module"*), which is the
  strongest argument yet for that style of fix — but the line is still
  wrong.
- **Decide what `named_color.rs` is for** — it is Pass 7's core, in the
  tree, reachable from nothing *(verified — grepped with no limit)*.
- **A decision nobody has taken**: whether iccce should implement
  lcms2's `EvalNInputs` geometry at all. **DL-021 sharpens it** — the
  scheme is asymmetric in the first `N−3` inks **and** lcms2 overrides
  it to trilinear in the B2A direction, so "match lcms2" is not one
  choice, it is two.
- **A disclosure for A4c**, if wanted: the colorant-sum-vs-`wtpt`
  inconsistency is detectable from the file today (rule 6).
- **Two of the three previously-stale doc blocks are now accurate** —
  `transform.rs`'s §Scope (which names grayTRC and `mAB `/`mBA ` on both
  sides) and `cmd_transform`'s comment *(both verified)*. **`lib.rs`'s
  §Status is the one that regressed**, above.

### 4. `icc-librarian` / whoever files next

- **The DL-014 citation audit.** Its surface grew again: `bpc.rs`
  (6.3.4.3, Tables 14/15/16) and `named_color.rs` (10.17, Table 66) on
  top of `lut_ab.rs` and `gray_trc.rs`. **Spot-reading is not
  auditing**, and `iccce-color` / `iccce-profile` have **never** been
  swept. One live question already found: `bpc.rs` heads 6.3.4.3
  **"PRIMARY-SOURCED"** while `icc__ref__bpc.md`'s `evidence:` line
  grades that section **`cross_verified_2src`**.
- **Observed residuals** for Pass 1's rows and for NC-032.
- **A ground-truth row for chromatic adaptation** — still the largest
  hole in Pass 1; **NA-002 is still not due** (checked against the code
  at a **fifth** consecutive filing).
- **A Linux run of anything at all.**

### 5. The operator

- **★ `ICC.1:2001-04`** — the top item, and now the only thing standing
  between Pass 4 and its done-when that the project cannot do itself.
  It also settles A1b, A2, A34 and A39c, and is the only source for
  `textDescriptionType`.
- The other optional downloads below.

---

## Optional operator unblocks — cheap, each settles something named

**All are browser downloads by Ken, not agent retrievals.**

| Document | What it settles |
|---|---|
| **★ `ICC.1:2001-04` (v2)** | **A4b — the 11 ΔE question AND Pass 4's last done-when clause**, plus A1b, A2, A34, A39c, and `textDescriptionType` |
| **IEC 61966-2-1** | the **first ground-truth row for a transform**, and the end of the single-source sRGB/D65 lineage |
| **ICC's published D65→D50 `chad` values** (Annex E.4.2) | the one place the adaptation ground-truth hole could be partly filled |
| **`ICC.1:2010-12` (v4.3)** | A31 / D10 — `parametricCurveType` Table 68 across editions |
| **ITU-R BT.709** | a second source for sRGB primaries and D65 (blocked on the terms determination) |
| **The BPC document** (Adobe / ICC WP40 / ISO 18619) | the *estimation* half of Pass 5 — **NA-009's A42**. The *scaling* half no longer needs it |

**Each row is a claim about what a document contains.** Treat *"it would
settle A4b"* as a prediction until the document is open.

---

## Decisions already made — do not re-litigate

- **MIT**, dependencies permissive, **publishing is the operator's act**
  (rule 9). DL-009 records an *intent*; **intent is not authorisation.**
- **`iccce-color` depends on nothing** and contains no ICC. **The
  fixture generator depends on nothing either.**
- **The parser reports, it does not repair**; in the CMM the same
  instinct is **refuse by name, never substitute**.
- **lcms2 is the oracle, never a dependency** — subprocess only, pinned
  by commit hash (DL-001).
- **DL-003** duplicate tags · **DL-004** the ⚠ provisional 1.0 anchor ·
  **DL-005** exact invariants for legacy Lab · **DL-007** HDR in scope ·
  **DL-008** profile creation reversed into scope · **DL-010 / NA-001**
  the rational breakpoint · **DL-011 / DL-012** the tag-type selector ·
  **DL-013** lcms2's forced BPC *(now known to be keyed by the
  destination — see DL-021)* · **DL-014** the terms for citing
  ICC.1:2022 · **DL-015 / NA-004** the `pow` guard · **DL-016** exact
  values at sample points · **DL-017** the harness may path-depend on
  iccce's crates · **DL-018** a prediction pin for an upper-bound gate ·
  **DL-019** report-not-grade when the mechanism is known and the
  authority is not · **DL-020** refuse-don't-guess, discharged by an
  independently authored fixture *(its first revisit condition has now
  **fired** — the per-type transcription landed)*.
- **DL-021** *(new)* — **a measured implementation behaviour is a fact
  about the direction and path it was measured in**, until measured in
  the others. Scope lines name the **direction**; a mirrored twin is
  assumed to differ until measured; a cost is quoted **with its
  direction attached**; and when a method difference collapses to zero
  the comparison needs a **counterfactual** to show it could have seen
  one.

### Everything measured against lcms2 is scoped to commit `21c582a`

Moving the pin is a **licence** event (DL-001) **and** a behavioural
one. **Re-run, not re-read:** NC-019 … NC-021, NC-034 … NC-037, NC-040,
NC-041, NC-043, NC-044 … NC-050, NC-053 … NC-057 **and now
NC-062 … NC-083**. **The sharp ones are NC-050, NC-056 and NC-082** —
each is a *transcription* of lcms2's internals, so a retuned
interpolator or resampler invalidates them **silently**, by continuing
to reproduce the old lcms2 perfectly.

---

## Method reminders that stay load-bearing

1. **A wrong colour looks exactly like a right one.** This filing's live
   example: the `lut8` legacy-Lab confusion would have cost **≈0,2
   ΔE2000** — under the anchor, invisible to every ΔE gate in the suite,
   and caught only because the encoding is asserted exactly (DL-005).
2. **★ Say which direction.** Three lcms2 "rules" this project had
   written down turned out to hold in one direction and not the other
   (DL-021), and one of this project's own predictions was falsified the
   same way (NA-008: the gray differential ran the wrong way round).
3. **Verify against the live source, not the dispatch.** The dispatch
   carried the overflow clause question as open; **the corpus had
   already answered half of it, verbatim, and the code had already acted
   on it.**
4. **A tolerance that fails first is a chance to fix a derivation.**
   Four Pass 4b rows moved, none of them widened to pass: a guessed
   envelope, a derivation at the wrong end of the axis, a missing term,
   and **real arithmetic mistaken for floating point**.
5. **When a difference collapses to zero, the comparison got weaker.**
   Agreement between two implementations running the same algorithm is
   not evidence the algorithm is right — say so, and price what the
   comparison *could* have seen.
6. **Expected values come from the literature.** **Pass 4 still has no
   ground-truth row**, and `derived-expectation` is **not** one.
7. **Coverage is part of every claim.** *"Every direction is measured"*
   means **two of four intents**, **one synthetic v4 file**, and **gray
   in one direction only**.
8. **Do not assert unmeasured facts about the environment.** **No agent
   here has ever run a git command**; every commit hash is reported. The
   run report covers `pass4b_report`, **not** `cargo test`.
9. **Check your own last filing — and your own draft.** This session
   found that a prediction carried in three of this librarian's own
   documents was wrong about what a differential would measure, **and
   that this filing's own first draft said BPC was wired into nothing
   when the CLI has a `--bpc` flag.**
10. **★ A truncated search is not an inventory.** The `--bpc` miss came
    from a **head-limited grep**: the first N matches, not the file's
    whole story. Same family as *a count is not an inventory* — when the
    conclusion is **"X is referenced nowhere"**, the search must be
    unlimited, because an absence proved by a truncated list is not an
    absence at all.

---

## The agents

- **`icc-engineer`** — lead. Be this agent if orchestrating.
- **`icc-spec-librarian`** — the standards corpus. **Owes** the narrowed
  clamp question, the M2 correction, the trilinear-override row, A4b,
  IEC 61966-2-1 and the ITU terms. *(Reported busy on BPC sourcing when
  this filing was made — and that sourcing has **landed**.)*
- **`icc-conformance`** — the oracle, the fixtures, the tolerance
  budget. **Owes** saturation and ICC-absolute in B2A, gray as a
  destination, the NC-077 grading call, a `lut8` fixture in the suite,
  a `cargo test` count, §14.7's decomposition, the sRGB ruler check and
  the remaining `TOLERANCES.md` sections.
- **`icc-librarian`** — ROADMAP, decision log, session log, and
  `NUMERIC_CLAIMS.md`. **No shell** — a dispatch to it must carry its
  evidence.

Dispatch them freely, and in parallel on disjoint file sets; no
permission is needed to dispatch an agent to read, analyse or draft.
