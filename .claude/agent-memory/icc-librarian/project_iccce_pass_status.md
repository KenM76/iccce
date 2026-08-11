---
name: iccce-pass-status
description: iccce status snapshot 2026-08-11 (Pass 4b measured B2A/v4-fixture/gray; done-when open on saturation + A4b-blocked absolute) — next free NC-084, DL-021 filed, Pass 5 + Pass 7 code already in tree
metadata:
  type: project
---

**Snapshot of 2026-08-11 (the Pass 4b filing — the tenth of one calendar
day). Verify before relying on any of it** — read `docs/ROADMAP.md`
(Pass 4b block, the second Pass 5 annotation), `docs/NUMERIC_CLAIMS.md`
§2.7 + §3.11 + §7.7, `docs/NEXT_SESSION.md`, newest `SESSION_LOG.md`.

**Pass 0 done · Pass 1 core · Pass 2 DONE · Pass 3 DONE · Pass 4 IN
PROGRESS, done-when NOT met — by exactly two items.**

**★ Pass 4b measured all three previously-blank directions**, 28 records,
**`pass=28 fail=0`** *(reported; no `cargo test` count came with it, so
NC-057…NC-061 STILL have no outcome)*. B2A (`mft1`) **1,330e-4** device
vs 5e-4, envelope agrees **0,02 %**, modelled residual **2,03 lsb of
1/65535**; v4 `mAB `/`mBA ` fixture **2,842e-14 L\* / 2,220e-16 device**
vs a **closed form derived from clause text** (new class
**`derived-expectation`** — NOT ground truth); gray **9,686e-5** device,
attribution **457×** to below the print floor.

**Done-when remainder = (1) saturation in B2A (`B2A2` is a distinct
table, cheap), (2) ICC-absolute — blocked on A4b, i.e. on an OPERATOR
download of ICC.1:2001-04.** Pass 4 is **not closable by engineering**.
v2/v4 clause judged **MET on stated terms**: v4 = **one synthetic
fixture**, because a 40-profile sweep found **zero `mAB `/`mBA ` tags**.

**★ DL-021 (new):** *a measured implementation behaviour is a fact about
the direction/path it was measured in until measured in the others.*
Three lcms2 instances, same file: (1) `_cmsReadOutputLUT` forces
**trilinear** for Lab-PCS LUTs ⇒ **NA-006's 1,5741 ΔE is an A2B number,
B2A envelope is ZERO**; (2) forced BPC keyed by the **DESTINATION**
version (M2/DL-013 half-stated); (3) legacy Lab applied for `lut16Type`
**not** `lut8Type`. Also bit this project's own prediction: the gray
differential ran GRAY→RGB, so **NA-008 is STILL unmeasured** (it lives in
the gray *destination* path).

**★ My catch this filing:** the corpus's **seventh pass** answered half
the encoded-PCS-overflow clause question **before** the dispatch called
it open — 10.12.5/10.13.3 VERBATIM *"shall be clipped to the range 0,0
to 1,0"* at the **matrix output**, `primary_spec`. iccce's clamp is
normatively backed; the queued `icc-spec-librarian` dispatch should be
**narrowed to the final-B-curve half**. Also: **A23/A25 RESOLVED, A24
partial, C4 filed** ⇒ **DL-020's first revisit condition FIRED**.

**Ledger: NC-062 … NC-083 filed; next free NC-084.** New **NA-009**
(BPC black-point *estimation* subset, A42) and **NA-010** (perceptual
black 0.00336/0.0034731/0.00287 — lcms2 **and iccDEV** against ICC.1
Table 16, 0,037 ΔE76 corpus-derived, **exactly zero at 16-bit PCS**).

**★ Undispatched code in the tree (5th consecutive filing):**
`iccce-cmm/src/bpc.rs` (Pass 5, 4 tests) and `named_color.rs` (Pass 7,
2 tests). Corpus `icc__ref__bpc.md` landed: **the BPC scaling map IS in
ICC.1:2022, clause 6.3.4.3, under another name** — so "Pass 5 pending
sourcing" is false. **BPC is also WIRED**: `Chain::with_bpc()` +
`iccce transform --bpc`, refusing by name at absolute and outside the
estimation subset, and **iccce NEVER forces BPC** (a recorded policy
difference from lcms2, one direction already priced by NC-078). **Pass 5
is missing MEASUREMENT, not code**; TOLERANCES §3.5's blanks are now a
gap and NA-009/NA-010's costs are OWED. `NamedColors` genuinely is
reachable from nothing. *(My first draft said both were unwired — from a
head-limited grep. See [[iccce-verify-own-draft-too]] item 6.)*
**`iccce-cmm/src/lib.rs`'s §Status is stale a 4th time** ("Still to
come: BPC") — saved by its own "trust the module" instruction.

**Counts, verified:** **102** `#[test]` declarations across **18** files
under `crates/` (was 95/16). **Unresolved:** README §15.5 says the
binary was built at **`97ad9fa`**, which predates the clamp commit.

**Commits, ALL reported — no agent here has ever run git:** `9e2e29e`,
`a0310c7`, `3d0c183`.

Related: [[iccce-direction-scoped-behaviour]],
[[iccce-refusal-discharged-by-fixture]], [[iccce-verify-own-draft-too]],
[[iccce-tolerance-cannot-swallow-and-claim]],
[[iccce-predicted-divergence-must-be-measured]],
[[iccce-gate-must-not-reward-deletion]],
[[iccce-bound-cannot-catch-its-own-magnitude]],
[[iccce-verification-loop-runs-both-ways]], [[ken-terse-scope-decisions]].
