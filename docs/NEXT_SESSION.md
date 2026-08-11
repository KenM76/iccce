# NEXT SESSION — start here

**Written 2026-08-11 by `icc-librarian`, at the evaluation-surface
filing (the ninth of the same calendar day, and a catch-up: the previous
filing was committed as `97ad9fa` and three landings had overtaken it).**
Replaces the Pass 4-progress edition entirely. Overwrite this file once
acted on.

Read order: this file → `docs/ROADMAP.md` (the **Pass 4
evaluation-surface block**, then the **Pass 2 DONE block**, then — only
if you need the numbers — the Pass 4 *progress* block above them) →
`docs/NUMERIC_CLAIMS.md` (**§2.6** → **§3.10**, starting with its
coverage box → **§3.10.5**, the GP-001 record → **NA-008** → **§7.6**) →
`docs/ARCHITECTURE.md` §5 (**twenty** entries; **DL-020** is new) →
`docs/SESSION_LOG.md` (nine entries, all 2026-08-11; the ninth is this
work) → `tools/gen-profiles/README.md` §5 and §7 (**§5 is stale — see
below**) → the corpus's
`icc__ref__lcms2_measured_behaviour.md` **M4/M5**.

---

## Where the project actually is

**Pass 0 done. Pass 1 core complete. ★ Pass 2 DONE. Pass 3 DONE.
★ Pass 4 IN PROGRESS — the evaluation surface is COMPLETE and the
done-when is NOT met.** All on 2026-08-11.

| | Commit *(all **reported** — no agent here has ever run git)* |
|---|---|
| Pass 0 · Pass 1 | `f976a0e` · `7313c5b` |
| Pass 2 batch 1 · difftest harness · batch 2 | `b35a12e` · `bfd6b1e` · `d40d601` |
| Pass 3 core · `transform` · audits · filing | `c4038eb` · `051707f` · `55772c6` · `a9618fe` |
| CLUT · PCS encodings · absolute intent · Pass 3 differential | `fc5ff58` · `0843094` · `6873df1` · `986dae6` |
| Pass 3 closure filing · stages 1–3 · CLI · doc catch-up · Pass 4 differential · the swept-in gen-profiles | `19a3b17` · `9aa1bca` · `63874f9` · `b3f4388` · `490191b` · `db60e92` · `d9e0b82` · `edcb60e` |
| **gen-profiles + 38 fixtures + GP-001 found** | **`7576cfa`** |
| **GP-001 fixed + `mAB `/`mBA ` evaluation + the transicc cross-check** | **`2e98cfd`** |
| **grayTRC F.2 + the last filing + two code-doc closures** | **`97ad9fa`** |

### The one thing to read before touching anything

**"Pass 4's evaluation surface is complete" is a statement about CODE,
not about evidence.** Every LUT tag type now evaluates in both
directions, plus monochrome — and:

- **The B2A direction has exactly ONE number**: `K` within 1×10⁻³ of
  `transicc`'s recorded **0.496117**, one point, one tag, one
  **synthetic** file (**NC-057**). **There is no B2A differential.**
- **`mAB ` has never been evaluated against a real file.** Every
  `mAB `/`mBA ` number in this project comes from bytes this project
  authored.
- **Gray has never been compared to lcms2 at all.** Its evidence is one
  real-file regression (white → the **full D50 triple**, the green-cast
  trap) and one synthetic identity.
- **Nothing traverses `transform::Chain` in a test that reaches the new
  models.** `Chain` is wired for `LutAb` and `Gray` on both sides;
  `transform.rs`'s two tests are both SWOP→sRGB. **Verified to exist,
  not verified to work.**
- **No run was reported with the landing that produced NC-057 … NC-061.**
  Five ledger rows carry asserted bounds and **no outcome**.

### The Pass 4 numbers that already existed, unchanged — quote them with their gate

341 CMYK points, SWOP → system sRGB, both v2.1.0, `-c0`, lcms2 at pin
`21c582a`:

- **corners (exact CLUT nodes): 5.9131×10⁻⁵ / 6.6558×10⁻⁵** vs 1×10⁻³ —
  the strongest cross-check evidence the project has;
- **lcms2's geometry emulated: 4.5931×10⁻³ / 4.8154×10⁻³** vs 2×10⁻² —
  **this** is the row that claims agreement;
- **raw: 0.252 94 (m-rel) / 1.6590 (perceptual)** vs 2.0 — **cannot
  claim agreement**; its value **is** the method envelope (NA-006's
  price: **1.5741** / 0.254 23);
- **absolute intent: 11.217 ΔE2000, REPORTED NOT GRADED** pending
  **A4b**; the modelled row collapses it **517×** to 2.1677×10⁻².

---

## ★ GP-001, because the next reader needs the whole arc in one place

The `mAB `/`mBA ` evaluator was written **`mAB `-only** and **refused
`mBA ` on a curve-count contradiction found during design**. **An hour
later the fixture corpus found the bug on that exact doubt.**
`decode_lut_ab` had used the `mAB ` convention for both types, expecting
**four** B curves on a CMYK `B2A0` where the specification puts
**three** — invisible on every square LUT, **wrong on every real CMYK
`B2A0`**. Settled from the clause text per type (**10.12.2/4/6** vs
**10.13.2/4/6**: entry side counted by `inputChan`, exit side by
`outputChan`), with lcms2 as corroboration. Fixed in `2e98cfd`;
regression is **NC-057**; decision record **DL-020**.

**Three things about it are still open, and two are not in the code:**

1. **The corpus sentence that caused it is still there** —
   `icc__type__lutAtoB_lutBtoA.md` carries one blanket rule for both
   types *(verified 2026-08-11)*.
2. **`tools/gen-profiles/README.md` §5 still says `Status: open`**, and
   §6.1 still shows `B2A0` REFUSED. **A reader of that file today
   concludes iccce cannot parse a real CMYK `B2A0`.**
3. **The Pass 2 machine sweep predates the fix** and has not been
   re-run.

---

## Then: the work, in dependency order

### 1. ★ The B2A differential — `icc-conformance`

Now reachable **through the shipped binary**: `Chain`'s destination side
selects `mft1`, `mft2` and `mBA ` under the 8.10.2 fallback. SWOP's
`B2A*` are **`mft1`**, so this is the first exercise of **`lut8Type`
evaluation** and the **`Lab8` codec** (Tables 12/13, A10-resolved) —
neither has any evidence of any kind today. The fourth codec cell,
**`lut8` + XYZ PCS**, is **refused by name** (`Lut8XyzPcsUnsourced`); a
run against an XYZ-PCS destination hitting that refusal is **correct
behaviour, not a bug to route around**.

### 2. ★ `mAB ` against a real file, and gray against lcms2

`v4-cmyk-mab-lab.icc` is the fixture; a **real** v4 CMYK profile is what
is missing. And the gray comparison is the cheapest run available —
`transicc` accepts every well-formed fixture, and
`fixtures/synthetic/v2-gray-curv-gamma.icc` /
`v2-gray-curv-identity.icc` exist. It would also give **NA-008** its
first measurement.

### 3. A ground-truth row — Pass 4 still has **none at all**

The tractable candidate is unchanged and now cheap: a **synthetic `mft2`
whose CLUT stores an affine function**, where *every* interpolation
scheme must agree exactly, so the expectation is **arithmetic** rather
than an oracle's opinion. `gen-profiles` exists to author it; nobody has.
Doing it also removes the "every differential row skips off this machine"
problem.

### 4. A4b — `icc-spec-librarian`, and the operator

Unchanged in status (**UNVERIFIED**), changed in framing. **M5 corrected
a sentence this project carried in three documents**: lcms2 does **not**
ignore the stored `wtpt` — `_cmsReadCHAD` uses it under the same guard to
synthesise a Bradford `chad`, so lcms2's v2-display model is
**coherent** (`wtpt` = unadapted, `chad` synthesised, adapted white =
D50). **DemoIccMAX reads `wtpt` as stored**, so the two ICC-adjacent
implementations **disagree with each other** and **iccce matches ICC's
own code**. **Only ICC.1:2001-04 settles it**; the ICC errata are
recorded as **unreachable by compliant means**.

**New and separate: A4c (SILENT)** — ICC.1 requires **no** agreement
between a profile's colorants and its `wtpt`. The stock Windows sRGB
profile's colorants sum to **D50** while its `wtpt` holds **D65**.
**A4c does not clear when A4b clears**, and it is **disclosable today**:
comparing the colorant-sum white against `wtpt` on a matrix/TRC display
profile with no `chad` is squarely rule 6.

---

## Owed work, carried explicitly

### 1. `icc-conformance`

- **★ The B2A differential**, the **`mAB `** and **gray** measurements
  (above).
- **★ `tools/gen-profiles/README.md` §5's `Status: open`**, §6.1's
  `B2A0 REFUSED` row, and §8's handover line — all describe a finding
  that is **fixed in the live source**.
- **An instrument check for the sRGB destination model** — Pass 3's
  record 7 bounds iccce's ΔE ruler on **Adobe RGB**, and Pass 4
  inherited that bound.
- **`tools/difftest/README.md` §14.7's record decomposition**: 7 Pass 3
  + 1 smoke + 28 graded Pass 4 (31 emitted, 3 skipped) = 36 — not
  "8 + 1 + 27 / 30". See `NUMERIC_CLAIMS.md` §3.9.8.
- **A run recorded with per-line output**, and — new — **any run report
  at all** for the tests behind NC-057 … NC-061.
- **`TOLERANCES.md` §3.2 (Pass 2)** and **§6's coverage table**;
  `gen-profiles/README.md` §7 is the material.
- **§13.10's four items**: the clamp-before/clamp-after fixture, the
  reverse direction, a **v4** pair, a **synthetic** pair (now cheap).
- **A re-run of the Pass 2 machine sweep** against a post-GP-001 build,
  with per-tag-type counts.
- **A behavioural test of `ncl2`** legacy-Lab decoding.

### 2. `icc-spec-librarian`

- **★ The per-type transcription of 10.12.2/4/6 and 10.13.2/4/6** into
  `icc__type__lutAtoB_lutBtoA.md`, replacing the blanket sentence that
  produced GP-001 — **and A23** (permitted element sets; clauses
  10.12.1/10.13.1 enumerate them verbatim and they are quoted in
  `gen-profiles/README.md` §5) **and A25** (`mluc` record selection;
  the generator reports re-reading 10.15 for its own use). **Both still
  UNVERIFIED** *(verified)*.
- **★ A4b** — top corpus gap, 11 ΔE of consequence.
- **IEC 61966-2-1's sRGB primaries** — the first ground-truth row for a
  transform. **Nobody has dispatched for it.**
- **The ITU terms determination** before any BT.709 fetch (DL-007).

### 3. `icc-engineer`

- **`transform.rs`'s §Scope paragraph** — calls `mAB `/`mBA ` *"the
  remaining absentees"* in the file that wires them on both sides, and
  **omits grayTRC entirely** *(verified)*. Note the two fixes that
  worked: `lib.rs`'s §Status now says *"if a module below contradicts
  it, trust the module"*, and `cmd_transform`'s comment records that its
  predecessor *"outlived the code by three commits"*.
- **A decision nobody has taken:** whether iccce should implement
  lcms2's four-input geometry. **M4 enlarges it** — the family is
  `EvalNInputs`, linear in the first `N−3` channels and tetrahedral in
  the last three, so the choice is not "match lcms2 on CMYK" but "adopt
  an asymmetry up to 15 channels".
- **A disclosure for A4c**, if wanted: the colorant-sum-vs-`wtpt`
  inconsistency is detectable from the file today.

### 4. `icc-librarian` / whoever files next

- **The DL-014 citation audit**, now over `lut_ab.rs` (10.12/10.13,
  Tables 12/13, A23) and `gray_trc.rs` (F.2, 8.3.4/8.4.4/8.5.3) as
  well. **Spot-reading is not auditing**, and `iccce-color` /
  `iccce-profile` have never been swept.
- **Observed residuals** for Pass 1's rows and for NC-032.
- **A ground-truth row for chromatic adaptation** — still the largest
  hole in Pass 1; **NA-002 is still not due** (checked against the code
  at a **fourth** consecutive filing: `iccce-cmm` calls nothing in
  `adapt`).
- **A Linux run of anything at all.**

### 5. The operator

- **★ `ICC.1:2001-04`** — the top item, and the only thing that settles
  A4b. It also settles A1b, A2, A34 and **A39c**.
- The other optional downloads below.

---

## Optional operator unblocks — cheap, each settles something named

**All are browser downloads by Ken, not agent retrievals.**

| Document | What it settles |
|---|---|
| **★ `ICC.1:2001-04` (v2)** | **A4b — the 11 ΔE question**, plus A1b, A2, A34, A39c. Also the only source for **`textDescriptionType`**, which every v2 profile carries and no specification in hand defines |
| **`ICC.1:2010-12` (v4.3)** | A31 / D10 — `parametricCurveType` Table 68 across editions |
| **ICC's published D65→D50 `chad` values** (Annex E.4.2) | the one place the adaptation ground-truth hole could be partly filled |
| **IEC 61966-2-1** | the **first ground-truth row for a transform**, and the end of the single-source sRGB/D65 lineage |
| **ITU-R BT.709** | a second source for sRGB primaries and D65 (blocked on the terms determination) |

**Each row is a claim about what a document contains.** Treat *"it would
settle A4b"* as a prediction until the document is open.

---

## Decisions already made — do not re-litigate

- **MIT**, dependencies permissive, **publishing is the operator's act**
  (rule 9). DL-009 records an *intent*; **intent is not authorisation.**
- **`iccce-color` depends on nothing** and contains no ICC. **The
  fixture generator depends on nothing either** — deliberately, so a
  fixture cannot inherit the parser's misreading.
- **The parser reports, it does not repair**; in the CMM the same
  instinct is **refuse by name, never substitute**.
- **lcms2 is the oracle, never a dependency** — subprocess only, pinned
  by commit hash (DL-001).
- **DL-003** duplicate tags · **DL-004** the ⚠ provisional 1.0 anchor ·
  **DL-005** exact invariants for legacy Lab · **DL-007** HDR in scope ·
  **DL-008** profile creation reversed into scope · **DL-010 / NA-001**
  the rational breakpoint · **DL-011 / DL-012** the tag-type selector,
  the predicted disagreement measured **absent** · **DL-013** lcms2's
  forced BPC · **DL-014** the terms for citing ICC.1:2022 ·
  **DL-015 / NA-004** the `pow` guard · **DL-016** exact values at
  sample points · **DL-017** the harness may path-depend on iccce's
  crates · **DL-018** a prediction pin for an upper-bound gate ·
  **DL-019** report-not-grade when the mechanism is known and the
  authority is not.
- **DL-020** *(new)* — **a rule the corpus cannot supply is REFUSED BY
  NAME, not guessed**, and the refusal is discharged by an
  **independently authored fixture that can fail**, never by a second
  reading. Provenance order when fixture and code disagree: **clause
  text, then fixture, then code — and the fixture is never edited to
  make a test pass.** Generalisation: **a blanket corpus sentence over a
  mirrored pair (`mAB `/`mBA `, `A2Bx`/`B2Ax`) is a defect class**,
  because it is silently right in the square case.

### Everything measured against lcms2 is scoped to commit `21c582a`

Moving the pin is a **licence** event (DL-001) **and** a behavioural one:
**NC-019 … NC-021, NC-034 … NC-037, NC-040, NC-041, NC-043,
NC-044 … NC-050, NC-053 … NC-056 and now NC-057 must be re-run, not
re-read.** **NC-050 and NC-056 stay the sharp ones** — their content is a
reading and a transcription of `cmsintrp.c`, so a retuned interpolator
invalidates them **silently**.

---

## Method reminders that stay load-bearing

1. **A wrong colour looks exactly like a right one.** This filing's live
   example is the counterfactual: had the `mBA ` curve counts been
   *guessed* instead of refused, the result would have been **CMYK**,
   not an error.
2. **Refuse rather than guess, and say what could not be settled.** A
   refusal that names the doubt is a work item; "unsupported" is a dead
   end (DL-020).
3. **A doubt is discharged by an artefact that can fail** — an
   independently authored fixture — not by re-reading the sentence that
   caused it.
4. **Never write a claim about an IMPLEMENTATION from memory.** *"lcms2
   ignores the stored `wtpt`"* was carried in three documents and is
   **wrong**: it synthesises a `chad` from that very value. Reading the
   source cost one lookup, again.
5. **Expected values come from the literature.** Pass 4 still has **no
   ground-truth row at all**.
6. **Every approximation is named and measured.** **NA-008** is new and
   **unmeasured** — and it is a *gamut-mapping* cost, not a rounding
   one, so it is zero on the neutral axis where both gray tests sit.
7. **Coverage is part of every claim.** *"The evaluation surface is
   complete"* means **code**. B2A = one point; `mAB ` = no real file;
   gray = no cross-check.
8. **Do not assert unmeasured facts about the environment.** **No agent
   here has ever run a git command**; every commit hash is reported.
   **This dispatch carried no test-run report**, so nothing filed from
   it is a pass result.
9. **Check the live source — including your own last filing.** This
   session the dispatch was wrong about where gray neutrality is
   measured, and **this librarian's own "39 fixtures" was wrong: there
   are 38.**

---

## The agents

- **`icc-engineer`** — lead. Be this agent if orchestrating.
- **`icc-spec-librarian`** — the standards corpus. **Owes** the per-type
  10.12/10.13 transcription, A23, A25, A4b, IEC 61966-2-1 and the ITU
  terms.
- **`icc-conformance`** — the oracle, the fixtures, the tolerance
  budget. **Owes** the B2A / `mAB ` / gray measurements, its own
  README's GP-001 status, the sRGB instrument check, §14.7's count, the
  remaining `TOLERANCES.md` sections and the `ncl2` test.
- **`icc-librarian`** — ROADMAP, decision log, session log, and
  `NUMERIC_CLAIMS.md`. **No shell** — a dispatch to it must carry its
  evidence.

Dispatch them freely, and in parallel on disjoint file sets; no
permission is needed to dispatch an agent to read, analyse or draft.
