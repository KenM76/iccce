---
name: iccce-pass-status
description: iccce status snapshot 2026-08-12 (ORIGINAL SCOPE COMPLETE — Passes 0–7 DONE; the ISO 4.2.5.4 question ANSWERED AGAINST US and the code corrected; a FIFTH crate iccce-measure; next free NC-165; DL-032 newest) — what remains is Pass 8 in pdfce, purchased documents, Passes 9–10, standing debts
metadata:
  type: project
---

**Snapshot of 2026-08-12 (the 4.2.5.4-correction + `iccce-measure`
filing — the FIFTEENTH overall, the SIXTH of the second calendar day).
Verify before relying on any of it** — read `docs/NEXT_SESSION.md`, then
`docs/ROADMAP.md`'s latest header block, `docs/NUMERIC_CLAIMS.md`
**§2.12 + §3.22–§3.24 + §7.12**, newest `SESSION_LOG.md`.

**★★★ THE ORIGINAL SCOPE IS COMPLETE. Passes 0–7 are all DONE and
filed.** What remains is four kinds of thing and only one is a task
list: (1) **Pass 8 — built in `pdfce`, not here**; (2) blocked on
documents nobody here can produce (**IEC 61966-2-1**, `ICC.1:2010-12`
for **A31**, ICC's `chad` values, ITU-R for Pass 9); (3) **Passes
9/10** — operator scope calls, never sized, and **Pass 10's
precondition (a ground truth that is not iccce) is unsolved**;
(4) standing debts.

**★★★ THE PREVIOUS SNAPSHOT'S "ONE OPEN QUESTION" IS ANSWERED AND IT
WENT AGAINST US (DL-030).** ISO/CD 18619 **4.2.5.4** specifies
**`InitialLab`**; **iccce shipped `outRamp[first]` and was
NON-CONFORMANT; lcms2 conformed.** Corrected at **`fd34a44`**. Cost
**0,0817 ΔE76** on SWOP = **100 % of NC-142's divergence**, *measured
before it was found*. The return type widened to a full `Lab` (only
that branch can return a **chromatic** black; 4.2.5.2.1 zeroes chroma
for CMYK only). ★ **Rule 7 finally ran in the direction it was written
to be capable of running in** — and it was cheap only because §3.18.6
pre-committed in writing *"rule 7 is not a licence to assume iccce is
right"* **before the answer existed**. **Consequence: NA-009's cost is
UNMEASURED AGAIN** — the number standing in for it was a defect — and
**nobody has re-measured the collapse.**

**★★ A FIFTH CRATE, `iccce-measure` (`2a2d616`), had been in the build
and in NO document** (grep returned 0/0/0 over the three docs).
CGATS/IT8.7 measurement-file reader, **Pass 10 pre-work**, operator-
authorised 2026-08-12. INVARIANT: **no ICC, no colour maths**; zero
deps; 8 tests needing no ICC fixture. Lineage **lcms2 `cmscgats.c`
(MIT — permitted)**; ★ **Argyll CMS is AGPL-3.0 — never read or cite
it for this work.** CGATS.17 is paywalled/unsourced. Its `issues`
vector is **rule 6 for measurement data**; a short column *fits* and
arrives as colour. **It makes no colour claim and produces no ledger
row.** Pass 10 itself is still blocked.

**★★ THREE GREEN RESULTS, THREE INSTRUMENTS (DL-031)** — see
[[iccce-count-needs-its-apparatus]]. `cargo test --workspace` **129
passed, exit 0**; `cargo test` in `tools/difftest` **36**; the
**conformance runner** `pass=142 fail=0 skip=3 error=0`. **142 was never
a cargo count** and lives only in commit `d5efd96`'s **message**, where
no dated note can reach it. `skip=3` has **never been enumerated**.

**★★ Pass 6's throughput/speedup is now a RANGE, not a figure.** Three
readings on one machine/binary: 1,203 / 0,820 / **2,251** Mpix/s and
14,4× / 12,18× / **22,85×**. Honest form: **"12–23×, load-dependent"**;
break-even ≈70 k → ≈1,19 M → **1 258 593 px**. ★★ **A FOURTH,
NON-OVERLAPPING set exists**: `TOLERANCES.md` §3.6.2 says **2,4–2,7
Mpix/s, 28–32×** on the `pass6.rs` apparatus. **Two ranges that do not
overlap and nobody knows why** — hypothesis (different work timed) is
labelled as one. **Until settled, quote NO single speedup figure.**

**★★ DL-032 — documentation-first prevented a real defect.** The
engineer nearly deleted `license-file` from three manifests to silence a
cargo warning; an **"★ EXPECTED WARNING — do not fix it by deleting
this"** comment stopped him. Removing it ships a tarball with **no MIT
notice text**, invisibly (`license = "MIT"` is only metadata). ★ **Rule
1 in a non-colour register: the clean build IS the defect.**

**Ledger: NC-158 … NC-164 filed; next free NC-165.** New evidence class
**`apparatus-census`**. **DL-030/031/032 filed; 32 decision-log
entries.**

**Counts verified 2026-08-12 (by reading, no shell):** **129 `#[test]`
declarations across 20 files** under `crates/` — cmm 63 · profile 33 ·
color 25 · measure 8 · cli 0, **matching the reported pass counts
per-crate on all five members** (corroborates the *denominator*, i.e.
nothing was filtered/ignored — **not** that anything passed, and **not**
coverage: cli has 0 tests and the total cannot notice). **36 across 6
files** in `tools/difftest`. `.git/logs/HEAD` = **55 lines**, tip
**`2a2d616`**.

**NOT verified at this filing (weaker than the previous snapshot):**
refs/push log **not read** — nothing evidences the tip being pushed, and
"nine pushes, seven unrecorded" is carried unchecked; repo root **not
enumerated**, so not even `dechk.obj`'s presence is re-confirmed.

**Holes that outlasted every Pass:** **zero `published-ground-truth`
rows for ANY transform** (tenth filing; IEC 61966-2-1 is the cheap route
and nobody has dispatched), and **no Linux run of anything, by anyone,
ever** (CI is a *report*). Plus: **no test asserts the corrected 4.2.5.4
branch** — the defect shipped through exactly that gap, past 63
`iccce-cmm` tests.

Related: [[iccce-count-needs-its-apparatus]],
[[iccce-git-files-readable-without-shell]],
[[iccce-apparatus-fault-under-every-hypothesis]],
[[iccce-direction-scoped-behaviour]],
[[iccce-control-only-as-good-as-its-fixture]],
[[iccce-free-to-disagree]], [[iccce-refusal-discharged-by-fixture]],
[[iccce-verify-own-draft-too]],
[[iccce-tolerance-cannot-swallow-and-claim]],
[[iccce-predicted-divergence-must-be-measured]],
[[iccce-gate-must-not-reward-deletion]],
[[iccce-bound-cannot-catch-its-own-magnitude]],
[[iccce-verification-loop-runs-both-ways]], [[ken-terse-scope-decisions]].
