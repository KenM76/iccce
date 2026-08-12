---
name: iccce-pass-status
description: iccce status snapshot 2026-08-12 (ORIGINAL SCOPE COMPLETE — Passes 0–7 DONE; 4.2.5.4 corrected, re-measured, and now DOCUMENTED BUT UNDEFENDED on any clean machine; separation apparatus live at 16/145; next free NC-179; DL-037 newest) — what remains is Pass 8 in pdfce, purchased documents, Passes 9–10, standing debts
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

**★★★ UPDATED 2026-08-12, SIXTEENTH FILING — the block above's
"nobody has re-measured the collapse" is DISCHARGED and the prediction
is FALSIFIED.** The `swop` divergence did **not** collapse; it **GREW
58,8×**, `8,166 8×10⁻² → 4,799 109 ΔE76`. **NA-009's cost is MEASURED
at last** (`4,799 109` swop / `5,000 000` synthetic) with three
mandatory caveats: **black point only**, **relative to lcms2 not
truth**, and ★ **NO ground truth exists** (no published black point for
SWOP; 18619 is a committee draft) — **never promote it**. Full story:
[[iccce-agreement-can-be-the-symptom]] (DL-033), and DL-036 — the
authored fixture had **zero power**; the vendor profile was the only
arm that could see. **NC-164a SPLIT into NC-174 (measured, true) and
NC-175 (unmeasured, falsified).**

**★★ THE SPEEDUP IS NOW WITHDRAWN OUTRIGHT** (supersedes the "12–23×
range" paragraph above). It spans **2,03× within ONE session at grid
33** (1,15× at grid 17 — not uniform across grids).
`TOLERANCES.md` §3.6.3(b): *"this project does not carry a speedup
figure."* The two-non-overlapping-ranges item is **discharged by
WITHDRAWAL, not reconciliation** — §3.23.4's hypothesis (the two
harnesses time different work) is **still untested**. What survives:
**break-even `≈1,3×10⁶ px AT GRID 33`**; the `85 900 → 1 273 800 px`
shift is **14,8×**, matching median build `0,838 → 12,444 s` to three
figures. Also: **`COMPILED_DE` is not derived on any compiled grid** —
Pass 4's 341-point CMYK iccce-vs-lcms2 max (`2,529 411×10⁻¹`), and
`pass4.rs` never builds a `CompiledTransform` (**DL-034**).
**T1/T4 got greener for bad reasons** — DL-035.

**Ledger: NC-158 … NC-175 filed; next free NC-176.** New evidence class
**`apparatus-census`**. **DL-030 … DL-036 filed; 36 decision-log
entries.** ★ **`README.md` carries NO throughput/speedup/break-even
claim at all** *(verified by grep, 2026-08-12)*.

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

**★★★ UPDATED 2026-08-12, SEVENTEENTH FILING (tip `e26d9ba`) — DL-033's
gap now has an INSTRUMENT, and the most important thing it produced is a
RISK.** Candidate separation is an emitted field on every conformance
record (**DL-037**, [[iccce-disclosure-is-not-enforcement]]). ★ **Read
its coverage before quoting it: 16 of 145 rows, all Pass 5c's;
`blind=0` is out of 16 — strictly, out of the SIX that reached the
comparison; 129 print `UNSTATED`.**

★★★ **THE STANDING RISK TO CARRY INTO EVERY FUTURE SESSION: the
corrected 4.2.5.4 clause is DOCUMENTED BUT NOT DEFENDED on any clean
machine.** The only differential arm with power is a **Windows system
profile** (fixture category **(c)**); on CI or any machine without that
directory **those rows skip and a full reversion of `fd34a44` stays
green.** A third purpose-built arm is **commissioned, not built**, with
its power to be **demonstrated by injecting the reverted behaviour**.
This collides with the Linux debt (still: **no Linux run by anyone,
ever**). ★ **`skip=3` IS NOW ENUMERATED** — three Pass 4 `icc-absolute`
PCS rows, one cause, a **principled refusal to grade** (see
[[iccce-count-needs-its-apparatus]]).

**Ledger now: NC-176 … NC-178 filed; next free NC-179. DL-037 filed;
37 decision-log entries. `SESSION_LOG.md` has 17 entries.**
`TOLERANCES.md` **§5 now carries NA-009** (four caveats — the fourth is
**coverage**) and a new **§1.1** stating that *a row's KIND measures
evidence strength and says nothing about POWER*.

Related: [[iccce-disclosure-is-not-enforcement]],
[[iccce-count-needs-its-apparatus]],
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
