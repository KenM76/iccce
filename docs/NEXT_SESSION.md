# NEXT SESSION — start here

**Written 2026-08-11 by `icc-librarian`, at the Pass 4 *progress* filing
(the eighth of the same calendar day).** Replaces the Pass 3-closure
edition entirely. Overwrite this file once acted on.

Read order: this file → `docs/ROADMAP.md` (the **Pass 4 progress
block**, then the Pass 4 annotation above it) → `docs/NUMERIC_CLAIMS.md`
(**§2.5** → **§3.9**, starting with its **coverage box** and **§3.9.0**,
the two-kinds-of-gate preamble → **§3.9.5** the absolute-intent finding →
**§3.9.6** the falsified prediction → **§3.9.8** the run-count
reconciliation → §4's **NA-003's second dated note**, **NA-006's** and
**NA-007's** dated status → §6 → **§7.5**) → `docs/ARCHITECTURE.md` §5
(**nineteen** entries; **DL-019** is new) → `docs/TOLERANCES.md` §3.4 and
**§5.2** → `tools/difftest/README.md` **§14** → `docs/SESSION_LOG.md`
(eight entries, all 2026-08-11; the eighth is this work).

---

## Where the project actually is

**Pass 0 done. Pass 1 core complete. Pass 2 built, one scope decision
from done. Pass 3 DONE. ★ Pass 4 IN PROGRESS — assembly stages 1–3 built,
the A2B differential run, the done-when NOT met.** All on 2026-08-11.

| | Commit *(all **reported** — no agent here has ever run git)* |
|---|---|
| Pass 0 | `f976a0e` |
| Pass 1 | `7313c5b` |
| Pass 2 batch 1 · difftest harness · batch 2 | `b35a12e` · `bfd6b1e` · `d40d601` |
| Pass 3 core · `transform` · audits · filing | `c4038eb` · `051707f` · `55772c6` · `a9618fe` |
| CLUT · PCS encodings · absolute intent · Pass 3 differential | `fc5ff58` · `0843094` · `6873df1` · `986dae6` |
| Pass 3 closure filing + doc fixes | `19a3b17` |
| **`lut16` device→PCS — stage 1** | **`9aa1bca`** |
| **`transform::Chain` — stage 2** | **`63874f9`** |
| **CLI: N-channel + four intents** | **`490191b`** |
| **B2A evaluation — stage 3** | **`b3f4388`** |
| doc catch-up · **the Pass 4 differential** · the swept-in gen-profiles | `db60e92` · **`d9e0b82`** · `edcb60e` |

### The Pass 4 numbers, so nobody has to go looking — and each with the words that must travel with it

`USWebCoatedSWOP.icc` → the Windows system sRGB profile, **341 CMYK
points**, **all four A2B intents**, both files **v2.1.0**, `-c0`, lcms2
at pin `21c582a`. `pass=36 fail=0 skip=3` *(reported)*.

- **At the 16 CLUT-node corners, where no interpolation happens:
  5.9131×10⁻⁵ / 6.6558×10⁻⁵ ΔE2000** against a 1×10⁻³ gate. **The
  strongest cross-check evidence this project has.**
- **With lcms2's own CLUT geometry substituted: 4.5931×10⁻³ /
  4.8154×10⁻³** against 2×10⁻². **This is the row that claims
  agreement.**
- **Raw, unmodelled: 0.252 94 (media-relative) and 1.6590
  (perceptual/saturation) ΔE2000** against a 2.0 gate. **This row cannot
  claim agreement** — its value *is* the interpolation-method envelope.
- **NA-006's price, measured at last: 1.5741 ΔE2000** on the perceptual
  table, **0.254 23** on the colorimetric one.
- **At the ICC-absolute intent: 11.217 ΔE2000, REPORTED, NOT GRADED** —
  a white-point **policy** divergence pending corpus **A4b**; modelling
  lcms2's substitution collapses it **517×** to 2.1677×10⁻², which is the
  graded row there.

### What is easy to over-read, so read it here first

- **"Pass 4 matches lcms2" is not available as a sentence.** Say which
  gate: the corners, the emulated geometry, or the raw envelope. The
  three differ by four orders of magnitude and only two of them are
  evidence of agreement.
- **★ The B2A direction has ZERO measurements.** `b3f4388` landed
  bidirectional `mft1`+`mft2` evaluation and `Chain` grew a `Lut16B2a`
  destination model — **and this run's destination is matrix/TRC**,
  because the sRGB profile has **no `B2A*` tag at all**. **`lut8Type`
  evaluation and the `Lab8` codec have never been compared to
  anything.**
- **`mAB `/`mBA ` are DECODED (Pass 2 batch 2) and NOT EVALUATED.** Do
  not plan stage 4 as though the decoder were missing, and do not report
  them as decoded-and-working.
- **Perceptual and saturation are the same bytes on this pair.** SWOP's
  `A2B0` and `A2B2` share one block of tag data, so *"all four intents"*
  ran four intents over **three distinct tables**.
- **DL-013's forced BPC was proved unreachable, not exercised.** Both
  profiles are v2.1.0 and the run **prints both header version words on
  every record**. A v4 pair reopens the question the moment one is
  introduced.
- **NA-002's Bradford cost is STILL not due.** Checked against the code
  at a **third** consecutive filing: `iccce-cmm` contains no reference to
  `adapt` at all. Check it again rather than carrying it.
- **`tools/gen-profiles` and 39 synthetic fixtures now EXIST** — four
  filings said they did not. **Nothing reads them yet**, so every
  differential row still skips off this machine.

---

## Then: the Pass 4 remainder, in dependency order

### 1. ★ Stage 4 — `mAB `/`mBA ` evaluation

**Unblocked by the differential's verdict on stages 1–3**: the `lut16`
A2B pipeline agrees with lcms2 at the node corners to the print floor,
so the shared machinery under stage 4 — `clut.rs`, the input/output
table interpolation, the PCS codecs — is evidence-backed rather than
hopeful.

What stage 4 needs, and what already exists for it:

- **The decoder is done.** `iccce-profile::lut::decode_lut_ab` produces
  one `LutAB` for both directions; the **direction is carried by the tag
  type**, and `B` is always the PCS-side end. Traversal: `mAB ` is
  A → CLUT → M → Matrix → B (device→PCS), `mBA ` the reverse.
- **The 3×4 matrix has twelve coefficients** — nine, **then three offset
  terms**. Reading 36 bytes and stopping produces *"a uniform colour cast
  that looks like a white-point problem"*. The decoder already refuses
  that shape; the **evaluator** must apply all twelve.
- **`mAB `/`mBA ` are NOT in the legacy-Lab set.** They use the **v4**
  16-bit encodings. `pcs_encoding.rs` implements both and takes **no
  default** — the caller must say which.
- **The F.3 NOTE's `(32 768/65 535)` scale factor** applies when a
  matrix/TRC model is expressed as a `lutAToBType`. It is
  `0.500 003 8…`, **not** ½; deriving it as ½ is a ≈7.6 ppm error,
  invisible in colour forever and fatal to bit-exact comparison.
- **A fixture exists**: `fixtures/synthetic/v4-cmyk-mab-lab.icc`
  *(verified — the directory enumerated)*. Nobody has opened it.

### 2. ★ The B2A measurement, which is owed and is where "at every intent" completes

The code shipped in `b3f4388` and **nothing has measured it.** SWOP's
`B2A*` tags are **`mft1`**, so this exercises `lut8Type` evaluation and
the **`Lab8`** codec — the A10-resolved 8-bit Tables 12/13 encoding —
neither of which any comparison has touched. Note the fourth cell of the
codec product, **`lut8` + XYZ PCS**, is **refused by name**
(`Lut8XyzPcsUnsourced`) because the 8-bit XYZ encoding has no verified
corpus row; a B2A run against an XYZ-PCS destination will hit that
refusal, and that is correct behaviour, not a bug to route around.

### 3. A ground-truth row — Pass 4 has **none at all**

Every §3.9 record is a cross-check, a self-consistency check or a
measurement of the oracle. The tractable candidate is now buildable: a
**synthetic `mft2` whose CLUT stores an affine function**, where **every**
interpolation scheme must agree exactly, so the expectation is
**arithmetic** rather than an oracle's opinion. `tools/gen-profiles`
exists to author it, and doing so also closes the CI-skip problem.

### 4. A4b, and the two corpus rows

`icc-spec-librarian` is **reported** to have been dispatched in parallel
with this filing on **A4b** (the v2 `wtpt` question) and on corpus rows
**M4** (lcms2's four-input CLUT hybrid) and **M5** (the v2-display `wtpt`
substitution). **As of this filing neither has landed**: the ambiguity
register still lists **A4b UNVERIFIED**, and
`icc__ref__lcms2_measured_behaviour.md` carries **M1, M2, M3 and no more**
*(verified — read 2026-08-11)*. **Do not close DL-019 or NC-053 on the
strength of a dispatch having been sent.**

---

## Owed work, carried explicitly

### 1. `icc-conformance`

- **★ The B2A differential** (above) and **an instrument check for the
  sRGB destination model** — Pass 3's record 7 bounds iccce's ΔE ruler on
  **Adobe RGB**, and Pass 4 **inherited that bound** rather than
  re-measuring it on the profile it used.
- **`tools/difftest/README.md` §14.7's record decomposition is wrong in
  both terms while its total is right**: the code emits 7 Pass 3 + 1
  smoke + 28 graded Pass 4 (31 emitted, 3 skipped) = 36, not
  "8 + 1 + 27 / 30". See `NUMERIC_CLAIMS.md` §3.9.8.
- **A run recorded with per-line output.** The old `pass=8`/`pass=7`
  question is now settled *structurally*; it is still true that a summary
  count cannot say which checks ran.
- **The four items `tools/difftest/README.md` §13.10 owes**: the
  clamp-before/clamp-after fixture; the **reverse** direction; a **v4**
  pair; a **synthetic** pair (now cheap — the fixtures exist).
- **`TOLERANCES.md` §3.2 (Pass 2)**, **§6's coverage table**, and twin
  rows for §3.7's Pass 3-core tolerances.
- **A ground-truth row for the matrix/TRC path** — §3.3.3's first blank
  row, still the largest evidential hole in Pass 3.
- **A behavioural test of `ncl2`** legacy-Lab decoding.
- **An observed residual for NC-032**, still the cheapest number in the
  ledger.

### 2. `icc-spec-librarian`

- **★ A4b** — it now has **11.217 ΔE2000 of consequence attached** and
  decides which implementation acquires a defect.
- **★ Corpus rows M4 and M5.** Until they land, NC-056 and NC-053 are the
  project's only record of two readings of lcms2's source — and **a
  reading that lives in one place quietly becomes a paraphrase.**
- **IEC 61966-2-1's sRGB primaries** — the first ground-truth row for a
  transform, and the end of the single-source sRGB/D65 lineage. **Nobody
  has dispatched for it.**
- **The ITU terms determination** before any BT.709 fetch (DL-007).
  *"Free download"* is not *"automated retrieval permitted."*

### 3. `icc-engineer`

- **`iccce-cmm/src/lib.rs`'s §Status — stale for the THIRD consecutive
  filing**, now reading *"B2A/lut8/mAB stages pending"* on a crate where
  `b3f4388` landed B2A and lut8. Only `mAB `/`mBA ` is pending.
- **`cmd_transform`'s doc comment contradicts its own code** — *"Only
  media-relative colorimetric exists"* above a `match` accepting four
  intents. A reader who trusts it concludes the absolute intent is
  unreachable by any differential.
- **A decision nobody has taken:** whether iccce should implement
  lcms2's four-input geometry at all. Matching lcms2 means adopting a
  scheme that is **not symmetric in the four inks** — a property, not a
  bug — and choosing it needs a stated reason.

### 4. `icc-librarian` / whoever files next

- **The DL-014 citation audit**, now over a larger surface again:
  `transform.rs` and `lut_transform.rs` add 10.10/10.11, 10.6, 6.3.4.2
  NOTE 3, 8.10.2 a)–d), Tables 40/44, Tables 12/13, and A10/A16/A21/A22/A27.
  **Spot-reading is not auditing**, and `iccce-color`/`iccce-profile`
  have never been swept.
- **A per-tag-type breakdown of the Pass 2 sweep**, and **observed
  residuals for Pass 1's rows**.
- **A ground-truth row for chromatic adaptation** — still the largest
  hole in Pass 1, still not on a clock.
- **A Linux run of anything at all.**

### 5. The operator

- **The Pass 2 clause-2 scope decision.** The *blocker* has dissolved —
  a generator and 39 fixtures exist — but **the question that was asked
  was about intent** (files on disk vs in-test bytes), and nothing
  records an answer.
- The optional document downloads below.

---

## Optional operator unblocks — cheap, each settles something named

**All are browser downloads by Ken, not agent retrievals.**

| Document | What it settles |
|---|---|
| **★ `ICC.1:2001-04` (v2)** | **A4b — the 11 ΔE question.** Also A1b, A2, A34, and **A39c**, the unsourced v2 half of the F.8–F.16 clamp reading |
| **`ICC.1:2010-12` (v4.3)** | A31 / D10 — what changed in `parametricCurveType` Table 68 between editions |
| **ICC's published D65→D50 `chad` values** (Annex E.4.2) | the one place the adaptation ground-truth hole could be partly filled |
| **IEC 61966-2-1** | the **first ground-truth row for a transform**, and the end of the single-source sRGB/D65 lineage |
| **ITU-R BT.709** | a second source for sRGB primaries and D65 (blocked on the terms determination) |

**Each row is a claim about what a document contains.** Treat *"it would
settle A4b"* as a prediction until the document is open.

---

## Decisions already made — do not re-litigate

- **MIT**, dependencies permissive, **publishing is the operator's act**
  (rule 9). DL-009 records an *intent*; **intent is not authorisation.**
- **`iccce-color` depends on nothing** and contains no ICC.
- **The parser reports, it does not repair**; in the CMM the same
  instinct is **refuse by name, never substitute** — now including
  `Lut8XyzPcsUnsourced` and `ChainError::SourceTagUnsupported`.
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
  crates · **DL-018** an upper-bound gate on a deliberate cost needs a
  prediction pin **and** a sensitivity control.
- **DL-019** *(new)* — **when a disagreement's mechanism is identified
  but no authority exists to say which side is right**, the raw
  comparison is **REPORTED, NOT GRADED**, the gate moves to the
  **modelled** quantity, **both rejected alternatives are written down**,
  and the blocking question is stated in full to a named owner. **The
  five steps are conjunctive** — an *unmodelled* disagreement is an
  unexplained one, and the right response to that is a failing gate.

### Everything measured against lcms2 is scoped to commit `21c582a`

Moving the pin is a **licence** event (DL-001) **and** a behavioural one:
**NC-019 … NC-021, NC-034 … NC-037, NC-040, NC-041, NC-043 and now
NC-044 … NC-050, NC-053 … NC-056 must be re-run, not re-read.**
**NC-050 and NC-056 are the sharp ones**: their content is a reading of
`cmsintrp.c` and a transcription of it, so a retuned interpolator
invalidates them **silently** — the transcription would go on
reproducing the *old* lcms2 perfectly.

---

## Method reminders that stay load-bearing

1. **A wrong colour looks exactly like a right one.** This Pass's live
   example: a 32 % error in `Z` applied to every colour, which reads as
   *"a slightly different rendering"* and is 11 ΔE2000.
2. **Never write colour maths from memory — and that includes CLAUSE
   NUMBERS.** NA-003 cited 6.4 from recollection, the citation was
   **wrong**, and a differential finding was built on it before anyone
   checked. DL-014's name-the-corpus-file rule is what catches this.
3. **Never write a claim about an IMPLEMENTATION from memory either.**
   *"lcms2 uses tetrahedral"* was carried in three documents and is false
   for four inputs. Reading `cmsintrp.c` at the pin cost one lookup.
4. **Expected values come from the literature.** Pass 4 has **no
   ground-truth row at all**, and every number in §3.9 is
   implementation-relative or self-referential.
5. **Every approximation is named and measured.** NA-006 is now
   **measured** — and the way it was measured is the pattern: **an
   apparatus that is not shipped**, graded against the shipped code
   first (0.0 exactly), then used to switch one variable.
6. **Tolerances are justified, not tuned** — and when a number cannot
   honestly be gated, **say so on the record and gate the modelled
   quantity instead** (DL-019), never widen until green.
7. **Coverage is part of every claim.** *"Pass 4 verified"* means one
   pair, both v2.1.0, A2B only, 341 points, three distinct tables, one
   platform, one pin — and **not B2A, not `mAB `, not v4, not any
   published value**.
8. **Do not assert unmeasured facts about the environment.** *verified* /
   *reported* / *unverified* are distinguished on purpose. **No agent
   here has ever run a git command**; every commit hash is reported.
9. **Check the live source — including your own last filing, your
   dispatch, and the tree's shape.** This session the dispatch was wrong
   about `mAB ` being undecoded, a README's own arithmetic was wrong in
   both terms while its total was right, and a sentence four filings old
   about `gen-profiles` not existing had become false.

---

## The agents

- **`icc-engineer`** — lead. Be this agent if orchestrating.
- **`icc-spec-librarian`** — the standards corpus. Dispatch for *every*
  sourcing question. **Owes** A4b, corpus rows M4/M5, IEC 61966-2-1 and
  the ITU terms.
- **`icc-conformance`** — the oracle, the fixtures, the tolerance
  budget. **Owes** the B2A differential, the sRGB instrument check,
  §14.7's count correction, §13.10's four items, the remaining
  `TOLERANCES.md` sections and the `ncl2` test.
- **`icc-librarian`** — ROADMAP, decision log, session log, and
  `NUMERIC_CLAIMS.md`. **No shell** — a dispatch to it must carry its
  evidence.

Dispatch them freely, and in parallel on disjoint file sets; no
permission is needed to dispatch an agent to read, analyse or draft.
