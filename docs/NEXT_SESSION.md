# NEXT SESSION — start here

**Written 2026-08-12 by `icc-librarian`, at the Pass 4 completion
filing — the second of the second calendar day, and the thirteenth
overall.** Replaces the Pass 6 + Pass 7 edition entirely. Overwrite this
file once acted on.

Read order: this file → `docs/ROADMAP.md` (the **header's Pass 4
paragraph and its nine-site correction sweep** first, then the **Pass 4
completion record**, then the **dated update** at the end of the "what
remains" block) → `docs/NUMERIC_CLAIMS.md` (**§2.10** — read its commit
row before quoting anything — then **§3.15**, **§3.14**, **§3.16**, then
**§7.10**) → `docs/ARCHITECTURE.md` §5 (**twenty-six** entries; **DL-026**
is new) → `docs/SESSION_LOG.md` (thirteen entries) → `docs/TOLERANCES.md`
**§3.4.4.6**, **§3.4.5** and the two 2026-08-12 rows in §4.

---

## ★★ Four things that are true today and were not true this morning

1. **★ PASS 4 IS DONE. Passes 0 through 7 are ALL closed — the original
   scope of this project is complete.** What remains is Pass 8 (built in
   `pdfce`), Passes 9–10, and the standing debts.
2. **★ The librarian's oldest constraint does not exist.** Two documents
   and the dispatch all said `icc-librarian` **has no shell**. **A
   `Bash` tool was present.** Used for **read-only `git` only**, labelled
   at every claim. **Three items carried across filings fell to five
   commands.**
3. **★★ `dechk.obj` — a 5 933-byte MSVC object file — IS IN THE PUBLIC
   REPOSITORY.** Tracked, added by `aef7566`, and `aef7566` is an
   ancestor of `origin/master`.
4. **★ There are NINE pushes to `origin/master`. DL-024 records two.**
   *(Eight when this file was drafted; a ninth landed during the
   drafting — see item 0's commit-hygiene entry.)*

---

## Where the project actually is

| Pass | Status |
|---|---|
| **0** scaffold + oracle · **1** colorimetry · **2** parsing · **3** matrix/TRC | **DONE** |
| **4** LUT transforms and intents | **★ DONE (2026-08-12)** — closed today |
| **5** black point compensation | **DONE on stated terms** (the estimators were never discriminated) |
| **6** performance · **7** named colours | **DONE** |
| **8** the pdfce bridge | **NOT STARTED — and it is built in `pdfce`, not here** |
| **9** HDR (BT.2100) | **NOT STARTED.** Blocked on the ITU terms determination *before* any download (DL-007) |
| **10** profile creation | **NOT STARTED, far-future**; its precondition — a ground truth that is not iccce — is unsolved |

**`HEAD` = `origin/master` = `95c04c1`; 51 commits; zero merges**
*(verified — `git log`, `git rev-list --count`, refs read)*. **At the
last filing the tip was `f6203b8` at 45 commits** — so **§2.9's
unresolved "45 vs 49" is SETTLED: 45 was right, the dispatch's 49 was
wrong.**

### ★★ The one thing to read before touching anything

**Nine statements across three documents said saturation in B2A had
"never been run". It had been run, measured, and fully written up in
`TOLERANCES.md` §3.4.4.6 — on the same calendar day.**

Nothing was wrong. Nothing contradicted anything. **The finding never
propagated out of the file where it landed.** The cause is in the
previous edition of this file, §4: at the Pass 6/7 filing **`tools/` was
deliberately not re-read**, because `icc-conformance` was working there.
That protocol is **sound**; its cost had never been named — **anything
finished in the un-read tree is invisible to the filing and is carried
forward as "owed."**

> **★ THE GUARD, now standing.** When a filing skips a directory because
> another agent holds it, **record WHICH directory was skipped and mark
> every dependent item `unverified-this-filing`, NOT `owed`.** They are
> different claims and **only one is safe to act on**: *"owed"* tells the
> next session to do work that may already be done — which is exactly
> what it did. **Grep before recording anything as owed.**

---

## ★★ Then: the work, in dependency order

### 0. ★★ Fifteen minutes with a shell — and item 1 is the only urgent thing in this file

- **★★ `dechk.obj`.** 5 933 bytes, MSVC COFF, at the **repository root**.
  **Tracked; added by `aef7566`; `aef7566` is an ancestor of
  `origin/master`** *(all verified — run)*. **`.gitignore` has no
  `*.obj`/`*.o` rule** *(verified — read)*. Add the rule, remove the
  file, **and decide about history**. ★ **Same shape as `edce48b`** —
  `NUMERIC_CLAIMS.md` §2.6: *"untracked in-progress `tools/gen-profiles`
  swept in by `d9e0b82`'s cwd-relative pathspec — a process slip"* —
  **same root directory, same mechanism, now with the push already
  done.** It is small and benign, but ***"benign" is a judgement the
  operator makes about a published artefact, not one an agent makes for
  him.***
- **★★ Confirm the pushes.** **Eight `update by push` lines exist**, the
  last at **08:19:21 −04:00**; **DL-024 records two.** Rule 9 and DL-024
  both say publishing is the operator's act and *"he said yes on the
  12th" is not standing permission*. **Recorded as an observation, not
  an accusation** — the reflog attributes them to `KenM76` and **no file
  records authorisation either way.** Confirm; do not assume.
- **★★ COMMIT HYGIENE — the top process item, on three instances in two
  days.** ~~Commit the Pass 4c work.~~ **It committed itself**: while
  this filing was being written, another agent's commit **`5cfee17`**
  (*"difftest: the estimator discrimination — and lcms2 has TWO
  estimators"*, 09:06:21, 23 files, +4 907) **swept in `pass4c.rs`, the
  CLI help fix, `TOLERANCES.md` — and `docs/NUMERIC_CLAIMS.md` mid-write
  — and was PUSHED** *(all verified — `git show --stat`, refs read)*.
  `origin/master` is now `5cfee17`, a **ninth** push.
  **★ That is the THIRD instance of one mechanism**: `edce48b` swept in
  in-progress `gen-profiles` (§2.6, *"a cwd-relative pathspec — a
  process slip"*), `aef7566` swept in `dechk.obj` and published it, and
  now `5cfee17` swept in another agent's unfinished document.
  **Three times in two days is not a slip; it is the default behaviour
  of the command being used** — and the cost is no longer untidiness:
  **it publishes work whose author has not finished checking it.**
  **Owed: commit with EXPLICIT PATHSPECS, never `-A` or a bare `.` from
  the repository root, while any other agent is working in the tree.**
  Consequence to be aware of when reading history: **`5cfee17`'s message
  mentions neither Pass 4c nor this ledger**, so `git log` is a
  misleading index of when they landed — use `git log -- <path>`.
- **Paste the twelve-line `iccce bench` output somewhere durable.** Owed
  since the Pass 6 filing; still the entire evidence for four rows.

### 1. ★★ Pass 8 — the `pdfce` bridge. The real next step, and NOT in this repository

`ARCHITECTURE.md` §4 fixes the boundary and it does not move: **a thin
bridge crate *in `pdfce`*, and `iccce` must not know what a PDF is.**
`/ICCBased` → `iccce_profile::Profile`; `/Separation` and `/DeviceN` →
named-colour lookups; PDF/X `/OutputIntent` → a destination profile.

**What Pass 7 handed it:** `NamedColors::resolve_to_device`, returning
**`None`** for an unknown name — the `/Alternate` fallback signal,
deliberately not an error.

**What this repository still owes the bridge:**

- **A spot resolved into a LUT destination.** A press profile is the
  normal `/OutputIntent` and it **is** a LUT profile. ★ The **gray**
  destination arm was finally exercised today (§3.16.2, a scratch
  probe), so the machinery is demonstrably reachable — **the LUT arm is
  still not.**
- **Any cross-check at all on the spot path** — item 2.
- **A statement of what a caller should do with a reported
  malformation.** Rule 6 says the parser reports and does not repair.
  **No caller that must keep going has ever exercised that**, and a PDF
  consumer will hand iccce real-world profiles at scale. **That is a
  `pdfce`-side design question this repository should not answer
  unilaterally** — but it should be asked before the bridge is written.

### 2. ★ The cheapest genuine cross-check in the project, skipped for two filings running

**Resolve a spot into ITS OWN profile's device space and compare against
the entry's stored `nDeviceCoords`.** An `ncl2` entry carries the device
values *the profile's author* recorded — **an expectation iccce did not
write, on bytes iccce did not choose.** The only such expectation
anywhere on the spot path, and it costs almost nothing.
`icc-conformance`.

### 3. ★★ The two fixtures that block two named approximations — `tools/gen-profiles`

**This is now the highest-leverage unwritten code in the repository.**

- **A PCSLAB gray fixture** → **NA-008's second arm.** The approximation
  is the choice between **`Y/Yn`** (PCSXYZ) and **`L*/100`** (PCSLAB),
  and **every gray profile in reach is PCSXYZ** — `ewgray18`,
  `ewgray22`, `BlackWhite`, and both synthetic `v2-gray-curv-*`.
  ★ **Agreeing with lcms2 cannot substitute**: lcms2 makes one of the two
  choices too, so a cross-check is blind to the difference by
  construction.
- **A v4 RGB-or-gray LUT fixture with a NON-ZERO device black** →
  **NA-009's cost.** Still the only instrument that can discriminate the
  two black-point estimators. **Sourcing the estimator on ISO/CD 18619
  did not change this — sourcing is not measuring.**

### 4. ★ The apparatus gaps

- **`tools/difftest/src/pass5.rs` still has no `#[test]` declarations** —
  **marked `unverified-this-filing`, not `owed`**, because another agent
  is editing `pass5*.rs` right now. **Fourteen ledger rows rest on two
  grids that nothing pins.**
- **★ `cargo fmt --check` FAILS in `tools/difftest`: 109 diffs across 15
  files**, all pre-existing *(reported)*. Rule 10's gate is stated
  **workspace-wide** and **`tools/difftest` is deliberately not a
  workspace member** (DL-001), **so `--workspace` has never seen it.**
  Either bring it under the gate or state the exemption in `CLAUDE.md` —
  **a binding rule silently does not apply to a quarter of the code.**
- **★ Three real measurements are pinned by nothing** (§3.16): the M3
  A/B, the gray-destination probe, the Pass 4b re-run. **Each is one
  `Record` away from a graded row with an NC number.**
- **A sweep for the bare *"D.6/D.7"* citation label.** **Annex D is
  INFORMATIVE**; the normative statement is **`ICC.1:2022` 6.3.2.2
  Eq (4)–(6)**; and **the label is not edition-stable** —
  `ICC.1:2001-04` has **no (D.7)** and its **(D.6) is a different
  equation**. **Every `wtpt` discussion here concerns a v2 file.** Fold
  into the DL-014 audit.

### 5. ★ The Pass 5c work is IN FLIGHT and is NOT filed

`pass5c.rs` is untracked; its record count moved **8 → 16** between this
session's two runs; two of its rows were **failing** at the second run;
and `TOLERANCES.md` has gained a **§3.5.8** naming a new finding —
**lcms2 has two black-point estimators at media-relative, selected by
the destination's device class and colour space** — and **withdrawing**
row Q3's CONFIRMED verdict. **None of it is in `NUMERIC_CLAIMS.md`.
Whoever files it reports its own outcome.** Do not fold it into a
Pass 4c summary and do not read its two red rows as a Pass 4c
regression.

### 6. The two holes that have outlasted every Pass

- **★★ No `published-ground-truth` row exists for ANY transform.** Not
  one, across Passes 3, 4, 4b, 4c, 5, 6, 7. **The cheapest route is
  still `IEC 61966-2-1`'s sRGB primaries, and nobody has dispatched for
  it — for the EIGHTH consecutive filing.** `icc-spec-librarian`.
  **This is now unambiguously the largest hole in the project.**
- **★★ A Linux run of anything at all.** Still nothing, by anyone,
  ever — and in public, where *"works on Windows"* is a narrower claim
  than a reader will assume.

---

## Owed work, carried explicitly

### 1. `icc-conformance`

- Items 2, 3, 4 and 5 above.
- **A ΔE2000 translation of NC-108's 0.003589 device units** — rule 4
  requires a named approximation's cost in ΔE, and `TOLERANCES.md`
  §3.6's own rows are written in that unit.
- **The Pass 6 sensitivity control's PASSING ratio**, ★ now against the
  **re-derived** band: §4 records that the `h²` justification was
  **falsified** and the measured convergence order is **1.32**, stable
  to 1 % across three octaves.
- **A compiled path measured in the B2A direction** (DL-021 makes it a
  separate question) and **a compiled chain with BPC folded in**.
- **A repeat timing run, and a second machine.**
- **Whether to re-grade NC-077** (the encoded-PCS overflow) — its file,
  its call, carried since Pass 5.
- **A synthetic `lut8` fixture wired into the suite**;
  `fixtures/synthetic/v2-cmyk-mft1-lab.icc` exists unused.
- **`TOLERANCES.md` §3.2 (Pass 2), a §3.7 (Pass 7), and §6's coverage
  table.**
- **An instrument check on iccce's ΔE ruler for the sRGB destination.**
- **A re-run of the Pass 2 machine sweep** against a post-GP-001 build.
- **A behavioural test of `ncl2` legacy-Lab decoding** — owed since
  Pass 2; **Pass 7 does NOT discharge it** (NC-019 still rests on a
  source reading).

### 2. `icc-spec-librarian`

- **★ `IEC 61966-2-1`** — still **the** cheapest route to the project's
  first ground-truth row for a transform. **Eighth filing; nobody has
  dispatched.**
- **★ The tier question that decides a ledger CLASS**: is
  `icc__ref__bpc.md` §2/§3 `primary_spec` or `cross_verified_2src`? At
  `primary_spec`, **NC-084/NC-086 become `normative-rule-conformance`
  rows**.
- **★ The forcing-policy question is NOT settled by ISO/CD 18619.** That
  document supplies **estimation**; **NC-100 / DL-022 turn on
  applicability** — whether BPC's *enablement* is specified as a
  function of intent and version. **NC-100 stays REPORTED, NOT GRADED**
  until something says it is.
- **The clamp question, narrowed**: must the final `B` curves' output be
  clipped to 6.3.4.2's encodable PCS range, and does 10.18's domain bind
  the *evaluator* or only describe the stored samples?
- **Corpus rows owed**: the **M2 correction**; the **trilinear
  override**; the **`IsEmptyLayer` 0,002 threshold**; **A41's
  ΔE2000 = 0,050 201**. ★ **And M3 now has a measured magnitude**
  (§3.16.1) that the corpus row does not carry.
- **`A31`** — the only UNVERIFIED row in the register. Needs
  `ICC.1:2010-12` (v4.3).
- **The ITU terms determination** before any BT.709/BT.2100 fetch
  (DL-007). **"The file is free" has never implied "automated retrieval
  is permitted"** — DL-002 exists because that inference was available
  at color.org and would have been wrong.

### 3. `icc-engineer`

- **All of item 0**, starting with `dechk.obj`.
- **The `tools/difftest` `fmt` exemption question** (item 4).
- **Whether iccce should implement lcms2's `EvalNInputs` geometry at
  all** — DL-021 makes it two choices, not one.
- **A PCSXYZ `ncl2` fixture**, so Table 66's second permitted encoding
  is not carried on a source reading.
- **crates.io, if and when the operator says so**: **name availability
  is still unchecked by anyone**, and **`THIRD_PARTY_LICENSES.md` via
  `cargo-about` is owed before a first publish**. **A public git
  repository is not a published crate** (DL-024).

### 4. `icc-librarian` / whoever files next

- **★★ The DL-014 citation audit.** It decides **NC-084's ledger class**,
  underwrites **DL-024's third pre-publication check** (a *published*
  compliance claim, still carried as *reported*), and now has a **live
  defect** to sweep (the D.6/D.7 label). `iccce-color` and
  `iccce-profile` have **never** been swept.
- **A re-read of README §15**, carried unverified three times now. ★ Its
  **§15.5 build-commit half is DISCHARGED** — the hash flag was correct
  (`97ad9fa` #29 predates `a0310c7` #32, verified) **and every Pass 4b
  number reproduces on a fresh build of the current tree** (reported).
- **Observed residuals** for Pass 1's rows and for NC-032.
- **A ground-truth row for chromatic adaptation** — NA-002, still not
  due.

### 5. The operator

| Document | What it settles |
|---|---|
| **IEC 61966-2-1** | the **first ground-truth row for a transform**, and the end of the single-source sRGB/D65 lineage. **The largest remaining hole in the project** |
| **`ICC.1:2010-12` (v4.3)** | **A31 — the register's ONLY unverified row** |
| **ICC's published D65→D50 `chad` values** (Annex E.4.2) | the adaptation ground-truth hole |
| **ITU-R BT.709 / BT.2100** | Pass 9's precondition — **blocked first on the terms determination**, not on the download |

**Each row is a claim about what a document contains.** Treat *"it would
settle X"* as a **prediction** until the document is open. ★ **The
worked example got worse today**: `ICC.1:2001-04` was expected to
adjudicate an 11 ΔE divergence and turned out **silent** — and the
ledger's prediction about *what its arrival would do to the record*
(*"one implementation acquires a defect"*) is **also now falsified**.
**Two levels of prediction, both wrong, about the same document.**

---

## Decisions already made — do not re-litigate

- **★ DL-026 (new) — NC-053 is RE-BASED OFF DL-019 and is PERMANENTLY
  ungraded.** The verdict did not move; **the basis did**, because
  DL-019 is a holding pattern asserting the project is waiting for a
  document, and **it is not waiting any more.** `ICC.1:2022` 9.2.36
  gates on **class with no version gate**; `ICC.1:2001-04` A.3.1.1 gates
  on the **adaptation condition**, not class — **so lcms2's predicate
  reproduces no clause in either edition.** And **the conformance clause
  binds READING profiles, not a CMM's computed output**, so a graded row
  is **unavailable**. ★ **The judgement is contingent on NC-120
  existing** — if the pin moves, **re-make it, do not inherit it.**
- **★★ WORDING: say lcms2 DIVERGES. Never "non-conforming."** The
  verdict is unavailable on a CMM's computed output, in either
  direction. Mirrors `TOLERANCES.md` §5.2's NA-003/A39b hedge.
- **★ A4c is SILENT and did NOT clear when A4b cleared**, and does not
  clear now. Whether a profile's `wtpt` must agree with its own
  colorants is a separate ambiguity; **disclosure is the one option
  ICC.1 does not foreclose**, and the system sRGB profile is exactly
  such a self-inconsistent file.
- **★ The project is PUBLIC** (**DL-024**) — and that authorises
  **nothing else**. No crates.io publish, no tag, no release, **and each
  push needs its own current go-ahead** (see item 0).
- **MIT**, dependencies permissive, **publishing is the operator's act**
  (rule 9).
- **`iccce-color` depends on nothing** and contains no ICC. **The
  fixture generator depends on nothing either.**
- **The parser reports, it does not repair**; in the CMM the same
  instinct is **refuse by name, never substitute** — and where a file is
  self-inconsistent in a way no clause adjudicates, **disclose** (A4c).
- **lcms2 is the oracle, never a dependency** — subprocess only, pinned
  by commit hash (DL-001). **It is also not a dependency of the
  published artefact**, and a tidy-up that folds `tools/difftest` into
  the workspace would break that **in public** — which is also why
  `fmt --check` has never seen it (item 4).
- **DL-003** duplicate tags · **DL-004** the ⚠ provisional 1.0 anchor ·
  **DL-005** exact invariants for legacy Lab · **DL-007** HDR in scope ·
  **DL-008** profile creation in scope · **DL-009** crates.io intent
  (**not an authorisation**) · **DL-010 / NA-001** the rational
  breakpoint · **DL-011 / DL-012** the tag-type selector, and a
  predicted divergence measured **absent** · **DL-013** lcms2's forced
  BPC, keyed by the **destination** · **DL-014** the terms for citing
  ICC.1:2022 · **DL-015 / NA-004** the `pow` guard · **DL-016** exact
  values at sample points · **DL-017** the harness may path-depend on
  iccce's crates · **DL-018** a prediction pin for an upper-bound gate ·
  **DL-019** report-not-grade when the mechanism is known and the
  authority is not (**now re-based for NC-053 — see DL-026**) ·
  **DL-020** refuse-don't-guess · **DL-021** a behaviour is a fact about
  **one direction and one path** · **DL-022** iccce never forces BPC ·
  **DL-023** say what the two sides were free to disagree about, before
  the run · **DL-024** the publication event · **DL-025** a sensitivity
  control is only as good as its **fixture**, and its scaling law must
  match the function's **smoothness class**.

### Everything measured against lcms2 is scoped to commit `21c582a`

Moving the pin is a **licence** event (DL-001) **and** a behavioural
one. **Re-run, not re-read:** NC-019 … NC-021, NC-034 … NC-037, NC-040,
NC-041, NC-043, NC-044 … NC-050, NC-053 … NC-057, NC-062 … NC-083,
NC-088 … NC-096, NC-099 … NC-102, **and NC-113 … NC-128**. **The sharp
ones are NC-050, NC-056, NC-082 and NC-088** — each is a *transcription*
of lcms2's internals, so a retuned interpolator, resampler or threshold
invalidates them **silently**, by continuing to reproduce the old lcms2
perfectly. ★ **NC-120 is now sharp in a new way**: **DL-026's judgement
depends on it**, so the pin moving does not merely invalidate a row — it
reopens a decision.

---

## Method reminders that stay load-bearing

1. **A wrong colour looks exactly like a right one** — **and so does a
   wrong measurement.**
2. **★ Grep before recording anything as owed.** Nine statements in
   three documents said *"never run"* about finished work.
   **`unverified-this-filing` ≠ `owed`.**
3. **★ An instrument is only as good as its fixture** (DL-025) — **and
   there are TWO ways for a comparison to be vacuous**, not one. The
   effect can be absent (caught by a sensitivity ratio), **or the output
   can be SATURATED** — both implementations clamping to the same
   boundary and agreeing perfectly while computing nothing. **The second
   needs its own count** (NC-125: 1 of 729).
4. **★ Read the CONDITION, not just the behaviour.** lcms2's `wtpt`
   substitution looked like a blocker for three filings; its predicate
   is a **conjunction**, and the confound was removable by **choosing
   inputs**. The blocker was a **profile pair**, never a document.
5. **★ A hedge is cheap and occasionally load-bearing.** NA-003's
   *"that number must never be restated as a bound in general"* was
   written from discipline alone and is now vindicated **by a factor of
   ~2,5×10⁴**.
6. **Print the sensitivity ratio**, and say what floor it clears and
   where the floor came from. NC-124's 100× is **transcribed** from an
   accepted band, not fitted.
7. **Say which direction and which tag type** (DL-021), in every row.
8. **Verify against the live source, not the dispatch.** This filing's
   instances: **the shell** (three documents said there was none), **the
   commit count** (45, not 49), **`dechk.obj`'s tracked status** (a peer
   correctly declined to guess; the guess would have been wrong).
9. **A gate figure is a claim.** **Exit codes compose; text matching
   does not.** Today's gates were gated on `$?` — **and "121 passed" is
   still not an inventory.**
10. **A class is not raised by how good the number looks**, and **a
    number is not a claim without the thing it is compared to.** Quote
    **8,90×10⁻⁵ with its 1,08×10⁻⁴ floor**, always.
11. **Coverage is part of every claim.** *"Pass 4 is DONE"* means two
    profile pairs, one destination tag, one grid each, one machine, one
    pin, and **no ground-truth row**.
12. **Do not assert unmeasured facts about the environment** — ★ **and
    check whether you can measure them before assuming you cannot.**
    Eleven filings asserted this agent had no shell.

---

## The agents

- **`icc-engineer`** — lead. Be this agent if orchestrating. **Owes all
  of item 0**, starting with **`dechk.obj` in the public repository**.
- **`icc-spec-librarian`** — the standards corpus. **Owes** IEC
  61966-2-1 (eighth filing), the tier question, the forcing-policy
  question, the narrowed clamp question, five corpus rows, A31, and the
  ITU terms.
- **`icc-conformance`** — the oracle, the fixtures, the tolerance
  budget. **Owes** the spot-path cross-check, the two missing fixtures,
  a ΔE for NC-108, `pass5.rs`'s tests, `TOLERANCES.md` §3.2/§3.7, and
  **the filing of its own in-flight Pass 5c work**.
- **`icc-librarian`** — ROADMAP, decision log, session log,
  `NUMERIC_CLAIMS.md`. ★ **Has a shell after all** — read-only git was
  used this filing and labelled at every claim. **Owes** the DL-014
  citation audit, which now underwrites a published claim **and** has a
  live defect to sweep.

Dispatch them freely, and in parallel on disjoint file sets; no
permission is needed to dispatch an agent to read, analyse or draft.
