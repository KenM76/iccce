---
name: iccce-pass-status
description: iccce status snapshot to the 2026-08-18 first-use-of-the-raster filing (tip 1a0509b; Pass K LANDED as 846952f; the raster capability is now VERIFIED not REPORTED; GWG 23.0's row was wrong about its NUMBERS too — 25%/0/0/0/75, which is ISO 32000's formula evaluated; two false statements found in passk.rs PROSE, one printed into the report; next free NC-243/NA-012, DL-061 newest, 27 session-log entries) (ORIGINAL SCOPE COMPLETE — Passes 0–7 DONE; ICC's own sRGB document INVERTED the 12-ULP attribution — it is the FILE's error; NC-230 is the ledger's SECOND published-ground-truth row so "NC-001 is the only one" retires; a shipped crash fixed; crates.io names checked but only 5 of 6 and NOT a reservation; GWG 23.0 reclassified OUT of "ours" so six-genuinely-ours becomes FIVE and the suite is 51 patches not ~48; Pass K exists in the WORKING TREE and in NO commit; PDF page RENDERING turns out to be available via pypdfium2 so NC-230's second reading is unblocked; next free NC-243/NA-012, DL-060 newest, 26 session-log entries; Pass H filed at pass=274) — what remains is Pass 8 in pdfce, purchased documents, Passes 9–10, standing debts
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

**★★★ UPDATED 2026-08-12, EIGHTEENTH FILING (tip `6c7cda1`) — THE
"STANDING RISK" TWO PARAGRAPHS ABOVE IS RETRACTED, and the engineer
measured it, not this librarian.** *"Documented but not defended"* is
**false in both halves**: a full reversion of `fd34a44` makes
`cargo test -p iccce-cmm` **fail (exit 101, 62 passed / 2 failed)** —
the clause was defended at **unit** level on a **synthetic closure** all
along; and the same reversion turned **no conformance row red on ANY
machine**, so the vendor arm was not the detector either. ★ **The
corrected, narrower claim: what had no detector was the clause exercised
THROUGH A PARSED PROFILE** (`parse → LUT model → estimator`), which is
where a wiring defect lives. **Seventh dispatch-vs-tree instance and the
first the dispatcher caught.**

**★★ That gap is now CLOSED and its power PROVEN by injection.** Third
Pass 5c arm on a new fixture `fixtures/synthetic/v4-rgb-mab-floored-b2a.icc`
(**40 fixtures now**): `InitialLab` `L* 12,5` vs rival `outRamp[first]`
`L* 37,5` — **25 `L*` apart by construction**. Bound
**`7,629 511×10⁻⁴`** = **half one PCSLAB quantum and nothing else**; the
reversion fails it by **`3,28×10⁴`**, the **only** failure in the suite
with both vendor paths disabled. **Two surfaces** — the runner row and a
`pass5c` unit test — and since `2835d23` `tools/` is gated in CI on
**ubuntu + windows**. ★ **Still no CI run observed, eighteen filings; a
workflow file is not a run.**

**★★★ GROUND TRUTH: AVAILABLE ≠ MEASURED.** See
[[iccce-ground-truth-cannot-exist]]. **NC-001 is STILL the only
`published-ground-truth` row** — do not let any future filing round
this up.

**★★ Census at this tip:** `pass=157 fail=0 skip=3`; separation
`unstated=119 … discriminating=16` over **160 rows** — **41 state a
separation**, up from 16 of 145. ★ **`16` has changed meaning between
filings** (was rows-stating-a-separation, is now `discriminating`) —
**always carry the denominator.** `cargo test --workspace` **131
passed**, matching **131 `#[test]` declarations across 20 files**
*(counted)*. `gen-profiles verify` **40 identical**. ★ **The `129`
collision has broken on its own** (119 vs 131) — the rule stands anyway.

**Owed and worth remembering:** the Annex D.6.3 fixture (blocked on **one
operator decision** — may published numbers live in an MIT repo, covering
ICC + CIE + ECI together); **two named-and-owed instruments**
(inverse-polarity; a vertex lighter than `L* 95` reaching lcms2's
untested `if (Lab.L > 95) L = 0`); a **retro-audit of which older rows
use `Separation::against`**; and `.github/workflows/ci.yml`'s prose
**"43 tests"** against a counted **47** (candidate fifth DL-034).

**Ledger now: NC-179 … NC-191 filed; next free NC-192. DL-038 … DL-043
filed; 43 decision-log entries. `SESSION_LOG.md` has 18 entries.**

**★★ UPDATED 2026-08-17, NINETEENTH FILING (tip `e21154c`) — a REQUEST
CHANNEL to `pdfce` exists** (`D:\Dev\FeatureRequests\iccce_FeatureRequests\`,
in **no git repo** — nothing may exist only there). **DL-044**: `pdfce`
is a **named external consumer**; its three CI gates (wasm32, no
copyleft, no network client) are now **inputs to rule 9**. ★★ **pdfce
adopting iccce is a LATERAL move in evidence class** (its table is
pdfium-fitted, ours is lcms2-checked) — **the case is CONFORMANCE, never
accuracy.** ★ That filing was made by a **general-purpose stand-in**, not
`icc-librarian`, and **one of its `[VERIFIED — I ran it]` rows was
false** (a too-narrow grep: this codebase writes `impl std::fmt::Display
for` in **all sixteen** impls). `CLAUDE.md` gained **rule 10**.

**★★★ UPDATED 2026-08-17, TWENTIETH FILING (tip `e21154c`, tree already
dirty in 4 files) — a NEW STANDING WORKSTREAM: Ghent compatibility.**
See [[iccce-compatibility-not-certification]]. **`docs/GHENT_COMPATIBILITY.md`
is new and is `icc-engineer`'s, not mine.** **98 PDFs → 121 embeddings →
20 distinct profiles**, now the **fourth** private corpus
(`ghent-v50/`, the most restrictive terms of the four). ★★★ **NOT ONE
ACCURACY CLAIM** — the lcms2 differential was dispatched and had not
reported. **NC-001 is still the only `published-ground-truth` row.**

**Ledger now: NC-192 … NC-199 filed; next free NC-200.** ★ **TWO new
evidence classes** — `fixture-declared-categorical` and `acceptance`
(the only filing ever to add two at once; the justification for that is
in §1's boxed note). **DL-045 / DL-046 / DL-047 filed; 47 decision-log
entries. `SESSION_LOG.md` has 20 entries.** `TOLERANCES.md` untouched —
correctly, six of the eight new rows have no bound to justify.

**★★★ UPDATED 2026-08-17, THE PASS G FILING (twenty-first `SESSION_LOG`
entry, twenty-SECOND librarian filing — an intervening filing was scoped
to `NEXT_SESSION.md` alone and correctly logged nothing; §7.17 states
both integers).** **`pass=229 fail=0 skip=3 error=0`, exit 0** (from
157), **72 new graded rows** (`tools/difftest/src/passg.rs`),
`discriminating` **16 → 42**. **Corpus-absent = CI: `pass=157 skip=7`** —
★ **numerically identical to the pre-Pass-G total, and 4 skip RECORDS
stand in for 72 rows.**

**★★★ THE RESULT: on X-Rite's VENDOR-authored v4 `mAB ` profile the raw
iccce-vs-lcms2 disagreement IS the interpolation method and nothing
else** — lcms2's `Eval4Inputs` geometry substituted collapses it **179×**
/ **243×**; an envelope from the CLUT's own bytes (no lcms2 output in it)
accounts for the raw residual to **0.04 % / 0.22 %**. `TOLERANCES.md`
§3.4.3's *"any real v4 LUT profile"* gap is **CLOSED**. ★★★ **But the
structural gate (envelope × 1.25) CANNOT claim agreement — the agreement
claim is the substituted-geometry row alone at `2×10⁻²`. NEVER merge the
two arms into one "agrees with lcms2" sentence** (§3.31.2). ★ The three
PCS rows compare the **harness's** `mAB ` reimplementation to lcms2; the
link to iccce is the `1×10⁻⁹` apparatus row, proven by injection I1.

**★★ NA-006 measured a SECOND time** (§4's 2026-08-17 note): envelopes
**0.948 160 / 0.828 123** on X-Rite's two A2B tags against Pass 4's
**1.5741** on SWOP. ★ **Three tables of two files — NOT a range. Never
write "n-linear costs 0.83–1.57 ΔE".**

**★★★ NC-197 DOWNGRADED to a NEGATIVE result** (§3.30.10): the v2/v4
`eciRGB v2` pair is **not an instrument** — both encodings put `wtpt` AT
the PCS white, and the files also differ in **TRC representation**, so
two candidate causes. Regridded **1.01e-4** (iccce both sides) /
**2.29e-4** (lcms2 both sides = about the FILES). ★★ **1.13e-4 vs 1.01e-4
— two of the engineer's OWN runs, UNRECONCILED**, filed that way. ★ The
downgrade cost nothing because the ROW said *"none declared in advance,
not a gate"* — **the prose made the claim, not the row.**

**★★ DL-048 and DL-049 filed; 49 decision-log entries. Ledger: NC-200 …
NC-218; next free NC-219. `SESSION_LOG.md` has 21 entries.** DL-049 is
the new shape: **a disclosure field that GATES NOTHING caught a defect in
a TOLERANCE'S JUSTIFICATION on a GREEN row** (`BLIND` fired; the `2e-4`
encoding floor did not hold — Ghent's sRGB colorants sum to the PCS white
to ≈12 lsb). ★ The fix **imports no third white point** — reaching for
D65 would have put the oracle's own single-sourced constant under a
finding about third-party authorship.

**★★ Rule 7 ran against a THIRD PARTY (NC-213):** Ghent's Adobe-embedded
sRGB / Adobe RGB have D50-adapted PCS, an **unadapted `wtpt`**, no
`chad` — an authorship defect under ICC.1:2001-04 A.3.1.1, settling
ICC-absolute **in lcms2's favour**. `eciRGB v2` is the control. **No NA
registered, no code changed** — newly owed as an engineering call.

**What Pass G did NOT do:** **11 of 20 profiles** (§7.16's pre-registered
check paid off — it asked exactly this); **no attribution row for §B**
(no `mft2` B2A model, so its 17–63× margin is NOT agreement); no `mBA `
direction; **8 `--bpc` combinations refused by name = differentially
untested**; nothing rendered; **no published ground truth and none
possible** — **NC-001 is STILL the only such row.** ★ **UNSETTLED:
`ungraded` stayed at 8 while §3.7.3 records 12 §B rows taken out of
grading.**

**★★★ UPDATED 2026-08-17, THE CONSTRUCTED-DESTINATION + `/N` FILING
(twenty-SECOND `SESSION_LOG` entry, twenty-THIRD librarian filing).
★★ NOTHING COMMITTED, NOTHING PUSHED — every row has NO COMMIT ANCHOR;
and the CONFORMANCE RUNNER WAS NOT RUN** (`icc-conformance` held
`tools/` + `TOLERANCES.md` for a **concurrent Pass H**, both untouched).
**`pass=229` stays NC-218's dated observation at `e21154c`.**

**★★★ A CORPUS CLAIM FALSIFIED by the test written to honour it.**
`ICC_Spec/iec/iec__s__srgb.md`'s *"the wrong sRGB breakpoint affects
8-bit codes 10 and 11, and nothing else"* is **WRONG — no 8-bit code
lands in the window; the separation at 8-bit precision is EXACTLY
ZERO** (`10/255 = 0.039216` < `0.03928`; `11/255 = 0.043137` >
`0.04045`). Max anywhere = **7.55e-7** linear light. ★★ **The corrected
statement is WORSE NEWS**: a wrong breakpoint is invisible to every
image, every 8-bit vector, every round trip (the constant inverts
itself) **and every differential against an implementation that made the
same choice**. ★★★ **THE CORPUS STILL CARRIES THE WRONG VERSION** — owed
to `icc-spec-librarian`. **DL-048 with the polarity reversed**: a pointer
that survives its target's move vs a **target that survives its claim's
refutation** — both make arrival read as confirmation.

**★★★ DL-051 — THE SUITE HAD ZERO POWER AGAINST THE CONSTANT IT
DOCUMENTED MOST.** Five injections on `builtin.rs`: Bradford omitted
**3/6 red**, applied twice **3/6**, gamma-2.2 **1**, green primary
**2** — **breakpoint substitution: 6 of 6 PASSED.** ★★ **The LENGTH of
the doc comment is the mechanism — nobody audits a constant that is
visibly well-explained.** Sibling of §5.3 and NOT the same: there the
tests could not fail *at all*; here **every test could fail, just not for
this defect**. ★★★ **A suite's power is PER-DEFECT, not per-suite**, and
**it was found by INJECTION — inspection could not, by the person who had
just written the constant AND the tests.**

**★★ `sRGB2014.icc` is NOT a second source** — its `rXYZ`/`gXYZ`/`bXYZ`
and all three TRC tables are **BYTE-IDENTICAL** to the HP 1998 file's;
only header/`wtpt`/`bkpt`/`chad` differ. **Exactly one lineage for those
nine numbers; §3.8's gap is NOT closed.** ★ Applying the file's **own
`chad`** (more information than reconstructing from constants) goes
**12.0 → 5.35 ULP and no further** — the corpus's negative conclusion is
**strengthened**, and the better route was **pre-registered before it was
run**.

**★★★ A THIRD EVIDENCE CLASS: `constructed-vs-reference-file`** (§3.32.1)
— **neither ground truth nor a cross-check.** The *reference* is
third-party but **the machinery on BOTH SIDES is ours**: one
implementation evaluating two **inputs**, not two implementations. ★★★
**`sRGB2014.icc` being ICC-published is the artifact most likely to be
mis-promoted to `published-ground-truth`. A PROFILE FILE IS NOT A
PUBLISHED EXPECTED VALUE. NC-001 is STILL the only such row.**

**★★ TWO THINGS I FOUND BY READING `crates/`, NOT CARRIED BY THE
DISPATCH** — §5.5 running in the unusual direction:
(a) **NC-227's number is measured but NOT ASSERTED and the test CANNOT
FAIL** — the survivor count goes to a `println!`, and the in-loop assert
compares `components(sig).count()` with **its own result**. ★★ **DL-051
recurring INSIDE the same session, in the module documenting the hazard
most.** (b) **NC-221's margin was rounded up**: at the **binding** probe
(white) observed ≡ derived bound, so the assert reduces to `x ≤ 1.05x` —
**margin exactly 5 %**, not the 37 % that applies to non-binding probes.
★ Row not weakened (the coincidence holds *only at the correct answer*,
making it a tighter white-point gate) — **but never quote the two
interchangeably.**
★ (c) Wording: the dispatch asked to mark **NC-213 "SETTLED"** — **NC-213
is a MEASURED row and was never pending**; what was open is §7.17
newly-owed 4. **A measured row cannot be settled.**

**★★ DL-050** — the built-in sRGB destination is selected by a
**two-variant enum, NOT `Option<&Profile>`**: *an `Option` being `None`
cannot distinguish "there was none" from "I failed to get one", and only
the second must never trigger a fallback.* Disclosed via
`DestinationProvenance`. **DL-052** — `ChainError` gained
`std::error::Error`, found by a **compiled doc example** failing
`E0277`; *a refusal that is awkward to propagate is a refusal that gets
discarded.* **DL-053** — see
[[iccce-count-from-a-sample-is-not-the-population]].

**★ NC-213's open question DISCHARGED by decision:** iccce uses `wtpt`
**as stored** and **discloses** (A4c/NA-007) — **verified in the running
thing** (DL-046). **6 of 60 disclose; all six hand-audited TRUE
positives, zero false positives.** `D50_XYZ.icc` looks false and is not
(XYZ identity summing to **illuminant E**); `D65_XYZ.icc` correctly does
**not** fire — its `chad` explains the difference.

**★ ICC.1 SILENCES (the `/N` work):** **A48 — ICC.1:2022 is SILENT on
header/tag channel agreement** (only `colorantOrderType` 10.4 and
`colorantTableType` 10.5 carry `shall`-level count agreement), so iccce
**discloses** rather than declaring non-conformance — ***"silent" ≠
"requires agreement"***. **A50 — the component count is a TWO-TABLE JOIN
(Table 19 + Table 41); ICC.1 publishes NO `Signature → count` map** —
cite as **derived**. ★ **lcms2's `cmsChannelsOf()` returns 3 for an
UNRECOGNISED signature**; iccce returns `Unknown(sig)` — **read from
source at the pin, NOT executed: `impl_crosscheck` BY INSPECTION**, the
weakest lcms2 statement in the ledger.

**★★ The `color-org/` terms were written BEFORE the files landed** —
that folder's rule 3 discharged; **the `ghent-v50/` lapse was not
repeated.** ★ Worth recording because **a rule that works leaves no trace
unless someone writes one.** **23 distinct `cprt` strings across 46 files
in SIX licensing postures** (bare assertion → *"included in commercial
software"* → *"…and sold"* → ECI self-contradicting → literal `none` →
absent). **Restrictive reading applies folder-wide.** ★★★ **Ten files
carry a grant that would survive redistribution — a FACT NO AGENT MAY
ACT ON.** Operator question now spans **FIVE** corpora and **a "yes" here
would have to be FILE-BY-FILE, not folder-wide.**

**Gates (engineer ran all bare, `$?` read):** `cargo test --workspace`
**154 passed, exit 0** (was 132 at `0bd76ad`); clippy / `fmt` / `doc` /
**wasm32 (4 library crates)** exit 0. ★ **`fmt --all` is the WORKSPACE;
`tools/difftest` is a SEPARATE workspace** — not the same runner.
★★ **CI covers NONE of §3.32** (NC-221/222 skip without a resolvable
sRGB profile; NC-219/220/223 are CLI sweeps over private corpora).
**Twenty-two entries, still no CI run observed by anyone here.**

**Ledger now: NC-219 … NC-229 filed; next free NC-230. DL-050 … DL-053
filed; 53 decision-log entries. `SESSION_LOG.md` has 22 entries.**
`ROADMAP.md` gained a **Pass 8 RETROSPECTIVE** subsection — the built-in
destination and the `/N` accessor **shipped with no ROADMAP entry at
all**, and a completion record with no plan above it beats a capability
with no record.

**★★★ UPDATED 2026-08-17, THE SUPPLEMENTARY FILING (twenty-THIRD
`SESSION_LOG` entry, twenty-FOURTH librarian filing). STILL NOTHING
COMMITTED OR PUSHED — §3.32 AND §3.33 are now TWO anchorless sections.**

**★★★ AN ATTRIBUTION FILED HOURS EARLIER IS INVERTED.** The operator
downloaded (browser; robot bar intact) ICC's **"How to interpret the sRGB
color space (specified in IEC 61966-2-1) for ICC profiles"** (Jack Holm,
ICC, 2015-04-27, 4 pp). **§B.2 publishes the D50-adapted colorants AND
ICC's recommended D65→D50 `chad` at 15 dp.** iccce's construction is
**3.02 ULP worst / 0.90 in `bXYZ.Z`**; the shipped HP 1998 /
`sRGB2014.icc` file is **11.13 ULP**. ★★★ **The ~12 ULP blue-Z residual
is the FILE's, not iccce's** — see
[[iccce-absence-of-publication-is-not-evidence]] (**DL-054**). The
remaining 3.02 ULP is **fully explained**: ICC's `chad` × inv(their §A.7
matrix) = their colorants to **0.00 ULP**, so it is only *which D65
matrix each side starts from*; iccce builds it exactly from BT.709-6 and
**keeps** its route. ★ ICC's published `chad` × D65 misses their own
stated D50 by **`4.9×10⁻⁵`**.

**★★★ THE LEDGER HAS A SECOND `published-ground-truth` ROW.** **NC-230**,
ruled by this librarian at the engineer's explicit request, with **four
conditions that are part of the claim**: (a) it grades **nine numbers,
not a transform**; (b) **one transcription, one reader — a second reading
is OWED** ~~and the librarian could not open the PDF (`pdftoppm`
absent)~~ ★ **that REASON was retracted 2026-08-18 — `pypdfium2` renders
and Read reads the PNG, so (b) is UNBLOCKED-BUT-UNDONE, not barred;
see [[iccce-inferred-environment-constraint-is-a-reading]]**, and the
document has **two transcription defects in §B.1**; (c)
it does **not** discharge Annex D.6.3 or touch DL-041; (d) *published* is
**provenance, not physical exactness**. ★★ **"NC-001 is the only such
row" — carried by eleven filings — RETIRES.** ★ `published-ground-truth`
now exists in **THREE populations**: this ledger, `tools/difftest`'s
Pass H rows (from ICC's `Probe2 Profile Readme`), and the corpus tiers.

**★★★ A SHIPPED CRASH.** `iccce bench` **ABORTED the process**
(`0xC0000409`, *"memory allocation of 1022842631448 bytes failed"*) on
ICC's 7-channel APTEC profile — a `_ => 33` catch-all (`33⁷` ≈ **952.6
GiB**) plus `checked_pow`, **which catches WRAP not SIZE**. Fixed:
computed ≥5-channel grids + `MAX_COMPILED_GRID_BYTES = 64 MiB` +
`ChainError::GridExceedsBudget`. ★★★ **DL-055 — each half of the fix
ALONE makes the row observe zero, so deleting the guard would have left
it GREEN with NO NUMBER MOVED**; four rows now, one per layer. See
[[iccce-gate-must-not-reward-deletion]].

**★★★ DL-056 — the ΔE destination gate is ASYMMETRICALLY BLIND**
(`+3.0×10⁻⁴` drift **passes and looks better** than the correct build);
what has power is the **absolute** `constructed_colorant_sum_is_d50`.
**DL-057 — a refusal that named the WRONG CLAUSE** on `A2B`-only profiles
(4 in ICC's published set); see
[[iccce-wrong-clause-refusal-and-discarded-halves]].

**★★ THREE DISCHARGES, ONE BLOCKER MOVED.** §3.32.8's tautological test
is fixed **and proven** (lcms2's `Known(3)` behaviour now fails **4 of
6** tests); the corpus's *"8-bit codes 10 and 11"* claim is **retracted
as defect `C8`** *(verified by this librarian in `ICC_Spec`)*; §1 now
lists `constructed-vs-reference-file`. ★★★ **The ground-truth row for
chromatic adaptation — owed since Pass 1 — moves BLOCKED →
AVAILABLE-AND-UNMEASURED**: ICC's `chad` is in the same §B.2. Instrument
named; **the bound must be derived BEFORE running it.**

**★★ THREE THINGS FOUND BY READING, NOT CARRIED BY THE DISPATCH:**
(a) **`builtin.rs`'s own doc comment still ends *"no document publishes
them at all"*** seventy lines below the rewrite that names the document —
only the trailing clause is false, which is why it survived;
(b) **`DEFAULT_DESTINATION.md` §4.2** carries the same falsified claim
under a banner saying everything below is still correct;
(c) ★★★ **`ROADMAP.md` judged a done-when MET against a DOC COMMENT while
§4 — the register of named approximations — was EMPTY for the constructed
sRGB.** **Now NA-011.** *A doc comment explains an approximation; the
register is what makes it findable.*

**Gates:** `cargo test --workspace` **158 passed, exit 0** (was 154);
clippy / fmt / doc / wasm32 (4 library crates) exit 0. **Conformance
runner `pass=274 fail=0 skip=9 error=0`** — `icc-conformance`'s Pass H,
**twice-carried, corroborated in `TOLERANCES.md` §3.8.1**. ★★★ **`274` may
NOT be compared with `229` and `skip=9` not with `skip=3` — Pass H added
rows.** ★ Twenty-three entries, **still no CI run observed**; CI covers
none of §3.33 **except NC-230**, the one row that needs no corpus and
**cannot skip**.

**Ledger now: NC-230 … NC-242 filed; next free NC-243. NA-011 registered
(next free NA-012). DL-054 … DL-057 filed; 57 decision-log entries.
`SESSION_LOG.md` has 23 entries.** ★ **`ICC_Spec` still carries the
falsified *"NO document states them"* at three places** — owed to
`icc-spec-librarian`, who was the concurrent agent this filing.

**★ UPDATED 2026-08-17, THE TAIL-DEBT #7 FILING (twenty-FOURTH
`SESSION_LOG` entry, twenty-FIFTH librarian filing). Smallest filing in
the log by substance: one tail-debt row, no Pass, no code, NO LEDGER
ROW.** See [[iccce-artifact-existence-is-not-obligation-status]]
(**DL-058**).

**Tail debt #7 SPLIT, and only one half moved.** `icc-engineer` queried
crates.io: **`iccce`, `iccce-color`, `iccce-profile`, `iccce-cmm`,
`iccce-cli` all returned *"does not exist"* on 2026-08-17**
(`[REPORTED]` — the librarian has no shell). ★★★ **crates.io has NO
reservation mechanism**, so it is a **dated observation with an expiry**,
**not a reservation**, and **it authorises nothing** (rule 9 and DL-009
untouched). ★★ **It covered FIVE of SIX candidate names** — the query
used **DL-009's list of 2026-08-11**, written before `iccce-measure`
existed, so **the fifth crate was never queried** (DL-053 again).

**★★★ The OTHER half is where reading paid.** `THIRD_PARTY_LICENSES.md`
now exists at the repo root and **is** genuine cargo-about output (its
prose is byte-identical to `about.hbs`) — **but it names FOUR crates at
`0.0.1` against a FIVE-crate, `0.1.0` workspace**, and **`about.toml`
has no `[iccce-measure.clarify]` block**, so by its own written rationale
**regenerating today emits the generic SPDX MIT placeholder** for the
fifth crate — which that comment calls *"worse than publishing nothing."*
**Verdict: CARRIED WITH A CAVEAT — neither "still owed" nor
"discharged."** ★ **The dispatch explicitly said not to discharge it on
the filename, and that instruction paid for itself.**

**Ledger: UNCHANGED — next free still NC-243, NA-012; no row added and no
§7.20 opened** (deliberate, reasoned in DL-058: §7 has never carried a
crates.io item and opening a status pass would inflate an integer §7
itself tracks). **DL-058 filed; 58 decision-log entries.
`SESSION_LOG.md` has 24 entries.**

**★★★ UPDATED 2026-08-17, THE GHENT BOUNDARY RECLASSIFICATION FILING
(twenty-FIFTH `SESSION_LOG` entry, twenty-SIXTH librarian filing). NO
ROW ADDED, CHANGED OR INVALIDATED — next free still NC-243 / NA-012.**
See [[iccce-patch-named-for-what-it-looks-at]] (**DL-059**).

**★★★ GWG 23.0 "Four different Grays" LEAVES Tier A — it is `pdfce`'s.**
`GHENT_COMPATIBILITY.md` §3.1 called it *"K-only preservation … CMM
policy, engine plumbing."* **It is device-space channel routing, the same
boundary class as overprint.** All four gray definitions resolve to the
same single-channel device answer **inside PDF**: **ISO 32000-1 §10.3.3 =
ISO 32000-2 §10.4.2.3** (`c=m=y=0`, `k=1.0−gray`, a **`shall`**),
**§10.3.2** (which routes gray→CMYK to that rule **inside the ICC-enabled
branch** — the load-bearing clause, still `[REPORTED]`),
**§8.6.6.4/§8.6.6.5**, **§10.3.1**. ★ **The patch readme names
`DeviceCMYK` as its reference and says the file was made "without
performing color conversion"** — it is a **non-conversion test**, exactly
like GWG 8.2, which §3.3 had classified correctly one table earlier.

**★★ Counts corrected: 51 patches (27/8/16), not "~48"; 35 for the first
two categories, not 32; "SIX genuinely ours" → FIVE.** The *"sixteen
touch colour conversion"* figure **was right**. ★ The same enumeration
returns **98 PDFs**, agreeing exactly with §4.1's extractor-derived 98 —
two instruments, one number.

**★★★ Pass K EXISTS IN THE WORKING TREE AND IN NO COMMIT.**
`tools/difftest/src/passk.rs` (+ `passk_probe.rs`) is fully wired into
`main.rs`/`lib.rs`, and **`docs/` contains not one word about it** — no
ROADMAP entry, no ledger row, and **the `TOLERANCES.md` §3.10.8 it cites
does not exist** (§3 runs 3.1 … 3.9.8). Branch tip is **`506fcd3`**
*(read `.git/refs/heads/master` + `.git/logs/HEAD`)*. **Nothing from it
was quoted or filed.** ★ It **got the boundary right on its own** — it
refuses to assume, measures both legs — but **cites §8.6.4.4, which is
*DeviceCMYK Colour Space***, not the rule (DL-057 again; the PDF corpus
already carries a standing correction of the identical substitution at
`color__iccbased.md:15`, so **§8.6.4.4 is an attractor**).

**★★ There is NO GWG requirement "23.0".** GWG 2022 is current (**no
2023**); requirements are `Dxxx`/`Rxxx`; `n.m` is **Output Suite *patch***
numbering. Nearest construct **`D0013 "Black Colour"` is a definition
consumed by the overprint requirements R0009–R0015** — **GWG files this
under overprint too.** ⇒ the equivalence claim's authority is **patch
documentation, not the GWG specification**.

**★★ The dispatch's premise FAILED: *"GWG 23.0 demands…"* appears NOWHERE
in the repository** *(grepped the whole tree)*. What exists is §3.1's
**column heading** *"the capability it demands of a CMM"*. **No filing
was made against it** — DL-048 from the other end (a *correction* aimed
at text that was never written).

**★★★ NOT changed, and the misreading to refuse: CMYK→CMYK black
preservation remains GENUINELY OURS**, unimplemented, being built.
**ICC.1 contains no black-preservation construct in either edition
checked** (`ICC_Spec` **A51**/**A52**) — the PCS is three components, so
every device→device transform is 4→3→4 and **K has no carrier**.

**Ledger: NO row. New `NUMERIC_CLAIMS.md` §7.20** (six owed items —
the `passk.rs` clause, its dangling §3.10.8, re-deriving §10.3.2, the
*"name the clause and the layer"* sweep over the other five Tier-A rows,
registering the GWG-2022 finding in `ICC_Spec`, and the closed-by-record
phrase sweep). **DL-059 filed; 59 decision-log entries. `SESSION_LOG.md`
has 25 entries.** ★ **§3.1 was NOT rewritten** — dated supersession, same
as §4.3's and §4.5's withdrawals.

**★★★ UPDATED 2026-08-18, THE FIRST-USE-OF-THE-RASTER FILING
(twenty-SEVENTH `SESSION_LOG` entry — 27 `^## 202` headings *counted*).
NO ROW ADDED — next free still NC-243 / NA-012** *(verified: highest are
NC-242 and NA-011)*. See [[iccce-source-labelled-number]] (**DL-061**).

**★★ Tip moved: `1a0509b`** *(read `.git/refs/heads/master`)*, and the
two blocks above are overtaken — **Pass K LANDED as `846952f`**, the
DL-059 docs filing as `7950dca` *(reflog read)*. **`passk.rs`'s
§8.6.4.4 defect is DISCHARGED** (nine `§10.3.3` cites, zero `8.6.4.4`,
working tree).

**★★★ DL-060's capability is VERIFIED, one day after being filed
`[REPORTED]`** — `which` for `pdftoppm`'s absence, a **1225×1619** PNG
from `PdfDocument(…)[0].render(scale=2).to_pil()`, Read displayed it.
§7.21's owed item 3 discharged after **one** filing. ★ **Scale and index
are still NOT measured** (recipe says 3.2, the run used 2).

**★★★ The first use found §3.1's row wrong a SECOND way: 25 % ·
0/0/0/75 · 75 · 75, not 50/50.** `1 − 0.25 = 0.75` is **ISO 32000's
formula evaluated** ⇒ the artwork is DL-059's **third** evidence
direction, arriving *after* the decision. **The readme declares; the
patch PDF has never been opened** — owed.

**★★★ Two false statements found in `passk.rs` PROSE, neither carried by
the dispatch:** `:1342`/`:2446` call `g = 0.5` *"GWG's own patch value"*
**and print it into the report**; `:291` lists an **`ICCBased`** panel
the readme does not have. **The ΔE is right; only the justification is
false** ⇒ nothing recomputes, no test can fail, DL-051's shape.

**★★ A SECOND text-extraction failure mode:** both new facts are set in
a **figure**, so engine agreement is **vacuous, not merely correlated**
— shared silence reads as *absence of the fact*. §7.21's sweep widens to
*"glyph-sensitive OR possibly in a figure"*. `GHENT_COMPATIBILITY.md`
§9 gained a fourth class **`[QUOTED-FROM-RASTER]`**.

**Ledger: NO row. New §7.22** (three owed: the two `passk.rs` prose
defects, the patch content-stream read, the widened sweep).
**DL-061 filed; 61 decision-log entries.**

Related: [[iccce-source-labelled-number]],
[[iccce-inferred-environment-constraint-is-a-reading]],
[[iccce-patch-named-for-what-it-looks-at]],
[[iccce-artifact-existence-is-not-obligation-status]],
[[iccce-absence-of-publication-is-not-evidence]],
[[iccce-wrong-clause-refusal-and-discarded-halves]],
[[iccce-count-from-a-sample-is-not-the-population]],
[[iccce-ground-truth-cannot-exist]],
[[iccce-negative-finding-removes-its-auditor]],
[[iccce-measurement-under-the-guards]],
[[iccce-disclosure-is-not-enforcement]],
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
