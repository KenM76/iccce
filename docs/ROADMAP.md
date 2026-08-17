# iccce — roadmap

Passes, in dependency order. Each is sized to be finishable and
verifiable; a Pass that cannot be demonstrated is too big.

**Pass 0 is done (2026-08-11). Pass 1's core is complete and validated
(2026-08-11, same day) with an explicit remainder — see its own status
block. Passes 2–10 are plan, not record.**

**Updated 2026-08-11 (same day, later): Pass 2 is IN PROGRESS — batch 1
(the eight non-LUT tag types) landed at `b35a12e`; batch 2 (the LUT
family) is next and is now unblocked by a measurement. Passes 4 and 5
carry new dated annotations from a finding about lcms2 that changes what
their cross-checks measure.** See the Pass 2 progress block and the
annotations under Passes 4 and 5.

**Updated again 2026-08-11 (same day, later still): batch 2 — the LUT
family — landed at `d40d601`, and a machine-wide sweep demonstrated
Pass 2's done-when clause 1 on this machine's 40 profiles. Clause 2 (a
synthetic corpus covering each tag type) is PARTIAL, and Pass 2 is
therefore still IN PROGRESS pending one scope decision.** See the
**batch 2** progress block. `ARCHITECTURE.md` gains **DL-014**, the
long-owed successor to DL-002: **ICC.1:2022 clause numbers may now be
cited, on stated terms.**

**Updated again 2026-08-11 (same day, later still): Pass 3's core landed
at `c4038eb` and the `transform` CLI at `051707f`. Pass 3 is IN
PROGRESS — its done-when needs two measured numbers and neither exists
yet** (`icc-conformance` is producing them in a parallel dispatch).
**Pass 2 is also still in progress**, so the Passes are no longer
completing in order. `ARCHITECTURE.md` gains **DL-015** (the parametric
`pow` guard — a divergence from ICC's *sample code*, inside a case the
standard declares undefined) and **DL-016** (sampled tables are asserted
by **exact values at the sample points**; the self-consistency bound
would have passed with the bug the exact-value test caught). See the
**Pass 3 progress block**, which also corrects one prediction the Pass 3
annotation made: **Pass 3 does not adapt, so NA-002's cost has not come
due.**

**Updated again 2026-08-11 (same day, latest): ★ Pass 3 is DONE — the
done-when is MET, and for the first time in this project's history
`iccce` has been compared to another implementation.** sRGB→AdobeRGB
agrees with lcms2 to **3.4762×10⁻³ ΔE2000 max** (tolerance 2×10⁻²,
**implementation-cross-check**) and the sRGB→AdobeRGB→sRGB round trip
costs **1.8788×10⁻² ΔE2000 max** (tolerance 2.5×10⁻², a **corrected
derivation**, **self-consistency**). Both numbers are written down in
`NUMERIC_CLAIMS.md` **§3.8** and `TOLERANCES.md` §3.3.1, with the
apparatus in `tools/difftest/README.md` §13. **`NUMERIC_CLAIMS.md`
§5.1's sentence *"iccce has never been compared to anything"* retires
today** — §5.3 records exactly what replaced it and how narrow the
replacement is. Pass 3's three remainder items are closed (absolute
intent sourced **and** implemented; parametric inverses for types 1, 2
and 4 implemented analytically; the perceptual/saturation policy
**sourced** to Table 25). `ARCHITECTURE.md` gains **DL-017** (the
harness may path-depend on iccce's crates — direction and four
conditions) and **DL-018** (an upper-bound gate on a *deliberate* cost
must be paired with a prediction pin, or removing the requirement makes
the gate greener). **Pass 2 is still IN PROGRESS on one scope
decision**, and Pass 4 groundwork is already in the working tree — see
the completion record.

**Updated again 2026-08-11 (same day, later still): Pass 4 is IN
PROGRESS.** Assembly stages 1–3 are built (`lut_transform.rs`,
`transform::Chain`, the B2A/lut8 generalisation) and **the first LUT
differential has run** — `USWebCoatedSWOP.icc` → system sRGB, 341 CMYK
points, **all four A2B intents**, `pass=36 fail=0 skip=3` *(reported)*.
**The done-when is NOT met**, and the Pass 4 progress block below says
exactly which parts are and are not: the A2B side has corner,
emulated-geometry and envelope evidence at every intent; **the absolute
intent's raw comparison is deliberately REPORTED, NOT GRADED** pending
corpus **A4b**; and **the B2A direction — whose code landed in
`b3f4388` — has ZERO measurements.** Two things filed by this Pass are
worth reading before anything else: **NA-006's cost is MEASURED for the
first time** (and the advance prediction of its mechanism was **wrong** —
lcms2 is not tetrahedral for four inputs), and a **new named divergence
from lcms2 at 11.217 ΔE2000** whose cause is identified and whose
authority does not exist yet. `ARCHITECTURE.md` gains **DL-019**.

**Updated again 2026-08-11 (same day, latest): ★ Pass 4's EVALUATION
SURFACE is complete, and ★ Pass 2 is DONE.** Stage 4 (`mAB `/`mBA `
evaluation) and the **grayTRC F.2** model landed, so every LUT tag type
now evaluates in **both** directions and monochrome profiles are no
longer a hole. The synthetic fixture corpus — **38 whole profiles on
disk** *(verified — enumerated)* plus a standalone generator with a
`verify` subcommand and a generated `MANIFEST.md` — **discharges Pass 2's
done-when clause 2 on the stronger of its two readings**, so the scope
question this document has carried since batch 2 is **moot rather than
answered**; the judgement, and its boundary, are in the new Pass 2 block.
**The corpus's first run against the shipped binary found a real parser
bug** — **GP-001**, `mBA ` curve counts, which affected **every real CMYK
`B2A0`** and which no square LUT and no profile on this machine could
expose. It was **refused before it was found**: the evaluator declined to
guess the counts an hour earlier, on the exact doubt that proved real.
`ARCHITECTURE.md` gains **DL-020**, and it is the richest entry of the
day because three separate disciplines had to hold at once for the bug to
surface. `NUMERIC_CLAIMS.md` gains **§3.10**, rows **NC-057 … NC-061**
and **NA-008**. **Still unmeasured, and this must not be rounded up:
there is no B2A differential** (one fixture cross-check point exists, and
that is all), **no `mAB ` evaluation against any real file**, and **no
gray comparison against lcms2 at all**.

**Updated again 2026-08-11 (same day, latest): ★★ Pass 4b — all three of
those directions are now MEASURED.** 28 records, **`pass=28 fail=0`**
*(reported)*: the **B2A** direction (`mft1`/`lut8Type`, 213 RGB points
end to end, **1,330×10⁻⁴** device against lcms2, the disagreement
**accounted for to 0,02 %** by an envelope built from lcms2's own
roundings, residual **2,03 lsb of 1/65535**); the **v4 `mAB `/`mBA `**
element pipeline, where iccce reproduces a **closed form derived from
clause text** to **2,8×10⁻¹⁴ `L*` / 2,2×10⁻¹⁶ device** — a **new
evidence class, `derived-expectation`**, and the strongest LUT claim in
the ledger; and the **F.2 gray** model, whose residual is not merely
explained but **reproduced**, collapsing **457×** to below the oracle's
print floor. `NUMERIC_CLAIMS.md` gains **§3.11**, rows
**NC-062 … NC-083**, and **NA-009 / NA-010**. **Three findings, and one
of them makes a number this document already carries into half a
rule:** lcms2 forces **trilinear** for a Lab-PCS LUT, so **NA-006's
1,5741 ΔE2000 is an A2B fact and the B2A envelope is exactly zero**;
lcms2's forced BPC is keyed by the **destination** profile's version, so
DL-013/M2 are half-stated; and an **encoded-PCS overflow** costs
**0,6117 ΔE2000** on 10 points, **reported not graded** — though the
corpus's seventh pass has since answered half the clause question, in
iccce's favour, **verbatim**. `ARCHITECTURE.md` gains **DL-021**: *a
measured implementation behaviour is a fact about the direction and path
it was measured in, until it is measured in the others.* **Pass 4's
done-when is STILL NOT MET, and its remaining distance is now exactly
two items — saturation in B2A (cheap), and the ICC-absolute intent,
which is blocked on a document only the operator can fetch.** Also
recorded: **Pass 5's sourcing has landed AND its code is wired and
reachable** (`Chain::with_bpc()`, `iccce transform --bpc`) — what Pass 5
lacks is **measurement**, so `TOLERANCES.md` §3.5's blanks are now a gap
and **NA-009/NA-010's costs are owed**; and **Pass 7's
`named_color.rs` is in the tree too**, reachable from nothing. **Neither
was in the dispatch**, and a first draft of this block got the BPC half
wrong from a truncated grep — corrected in place, with the error
recorded in `NUMERIC_CLAIMS.md` §7.7.

**Updated again 2026-08-11 (same day, latest): ★★ Pass 5 is DONE — the
done-when is MET on stated terms, and the terms are unusually
important.** BPC on and off **differ in the documented direction** (a
sign test with an algebraic proof and no tolerance at all — nothing
rises in `PB → 0`, no `K` rises in `0 → PB`), and iccce's BPC **matches
lcms2's within tolerance** (**1,110 588×10⁻⁴** device out of the
fixture, **4,600×10⁻⁵** into it, against a BPC-off baseline of
**1,012 157×10⁻⁴** on the same points — comparisons **388×** and
**682×** more sensitive than the effect they grade). The **scaling map**
is graded against **ICC.1:2022 clause 6.3.4.3** — the map is in the
primary specification after all, under another name — at
**1,110×10⁻¹⁶**, and against **Maria (2013) §4.2**'s two published
constraints solved by a different method at **3,331×10⁻¹⁶**. **Every one
of the six scenarios was pre-registered from both implementations'
sources before anything ran, and every prediction was confirmed**
*(reported)*. **The boundary, which must travel with all of it: the two
ESTIMATORS were never discriminated.** That negative result was
*derived in advance* — everywhere iccce will do BPC at all, lcms2's
estimator reduces to the same two values — so Pass 5 grades **the map,
the direction and the pipeline**, never the estimation (corpus **A42**);
the instrument that would close it is **a synthetic v4 LUT fixture with
a non-zero device black**, which does not exist. Two further boundaries:
lcms2 **silently performs no BPC at all** below an `IsEmptyLayer`
discriminant of **0,002** (≈**0,41 `L*`** between the two blacks) —
**a difference iccce deliberately lacks**, and **solved for, not
observed**; and **iccce NEVER forces BPC** where lcms2 forces it for a
v4 destination at perceptual, worth **3,137 348 `L*`** on one pair and
**REPORTED, NOT GRADED** under DL-019, because no obtainable clause
settles it. `NUMERIC_CLAIMS.md` gains **§3.12**, rows
**NC-084 … NC-104**; `ARCHITECTURE.md` gains **DL-022** (the
never-forced policy, promoted from a note under NA-009 now that it has a
measured size and a user-visible consequence) and **DL-023** (a
cross-check must state what the two implementations were **free to
disagree about**, derived from their sources *before* the run). **Pass 4
remains open at exactly the same two items** — saturation in B2A, and
the ICC-absolute intent, blocked on an operator download.

**★★ Updated 2026-08-12 — a SECOND calendar day, and the project's
original scope is essentially complete. ★ THE PROJECT IS PUBLIC.**
`master` was pushed to `https://github.com/KenM76/iccce` on the
operator's explicit go-ahead; `ARCHITECTURE.md` gains **DL-024**, which
records the event, the file-level evidence for it (two `update by push`
lines in `.git/logs/refs/remotes/origin/master`, at 06:51:17 and
06:54:50 −04:00 — **the first time any statement this project makes
about its own repository rests on something other than a report**), the
three pre-publication checks with **two verified and one carried as
reported**, and the four things publication does **not** authorise —
starting with crates.io, which remains unexercised. **★ Pass 6 is DONE**
— the done-when's two clauses are answered with numbers: a 300 DPI A4
CMYK→RGB conversion (**8 700 867 px**) in **7.23 s** = **1.20 Mpix/s**,
**14.4× the reference path**, with the compiled path's **off-node**
error against the reference path at **0.003589 device units**
(`self-consistency`, and labelled so). **The Pass's real work was making
that number mean anything**: its sensitivity control **caught its own
instrument on the first run** — an sRGB→sRGB fixture made the compiled
and reference arms identical *everywhere*, not merely at nodes, giving
**1.1×10⁻¹⁵ with no `h²` scaling at all** — and a second failure taught
that probing across the sRGB TRC breakpoint gives `h¹` rather than `h²`.
Both failures are in the test's own doc comment, and both are filed as
**DL-025**, with the observation that this is the **third** time in two
days that an instrument built to fail caught something a competent
engineer was about to ship (after **DL-016**'s off-by-one-sample and
**DL-020**'s GP-001). **★ Pass 7 is DONE** —
`NamedColors::resolve_to_device(name, dst)` resolves a spot colour into
a destination profile's device values through
`Chain::convert_pcs_to_device` → **the ordinary destination machinery**
(the same sourced 8.10.2 fallback), so a spot cannot take a private
path; the duplicated destination arm in `Chain::convert` now calls the
one shared method. An unknown name returns **`None`** — the
`/Alternate` signal — **not a guess**. Media-relative by construction
(Table 66). **The "reachable from nothing" finding this document filed
twice is closed**: a test resolves every spot in the committed `ncl2`
fixture into the real system sRGB profile. Also landed: the **A4c
disclosure** (`ICC.1:2001-04` **A.3.1.1** resolved **A4b** — the clause
addresses the profile's **author** and is silent on readers, so iccce
keeps `wtpt` **as stored** and **discloses** the inconsistency; a sweep
of this machine found **seven** v2 display profiles with that exact
shape, making it the authoring **norm**, not an outlier), and the
**ISO/CD 18619:2013** black-point estimation procedure implemented,
upgrading corpus **A42** — with a **pre-registered** chroma-divergence
prediction still awaiting measurement. `NUMERIC_CLAIMS.md` gains
**§3.13**, rows **NC-105 … NC-112**, and a new evidence class,
**`machine-timing`**. **What remains is in the "what remains" block
under Pass 8**, and the short version is: **Pass 8 is built in `pdfce`,
not here.**

**★★ Updated again 2026-08-12 (same day, later) — ★ PASS 4 IS DONE. The
original scope of this project is COMPLETE: Passes 0–7 are all closed.**
Its two remaining items are measured. **Item 1, saturation in the B2A
direction** (`B2A2`): 6 records, `pass4b.rs` §A extended, device
agreement with lcms2 at **1,550 0×10⁻⁴** against a computed envelope of
1,552 5×10⁻⁴ (**99,8 % accounted for**), preceded by the row that makes
it a measurement at all — **the three `B2A*` tags are three distinct
tables**, differing in two thirds of 145 588 bytes, where the A2B
direction of the same file aliases `A2B0`/`A2B2` into one block.
**Item 2, ICC-absolute through a LUT destination**: 10 records, the new
`tools/difftest/src/pass4c.rs`, **8,900×10⁻⁵** device against lcms2 —
**below its own media-relative floor of 1,080×10⁻⁴ on the same pair**,
so the absolute arithmetic adds nothing detectable above what the
direction already costs. **The blocker was never the document it was
recorded as**: lcms2's `wtpt`-substitution predicate is a **conjunction**
(`version < 4` AND class `'mntr'`), and a pair in which **each profile
fails a different half** removes the confound *structurally* — that pair
was in the committed fixture corpus the whole time.
`ARCHITECTURE.md` gains **DL-026**, which **re-bases NC-053 off DL-019**:
the verdict (REPORTED, NOT GRADED) does not move, but its **basis** does,
because DL-019 is a holding pattern that asserts the project is waiting
for a document and **the project is not waiting any more** — the clauses
were read, and **lcms2's predicate reproduces no clause in either
edition**, while **the conformance clause binds READING profiles rather
than a CMM's computed output**, so a graded row is not merely
undesirable but **unavailable**. NC-053 is therefore **permanently**
ungraded, in the A16/NC-056 pattern: a **difference**, not an error.
**Say lcms2 *diverges*; never *non-conforming*.** `NUMERIC_CLAIMS.md`
gains **§2.10**, **§3.14**, **§3.15**, **§3.16** and rows
**NC-113 … NC-128**. Also measured and filed: the **M3 out-of-gamut
excursion count** — whose owed form turns out to be a **null by
construction** and is **retired rather than satisfied**, replaced by a
controlled A/B that finds excursions up to **3,05 device units**, ~2,5×10⁴
times the 1,2×10⁻⁴ that `NUMERIC_CLAIMS.md` NA-003 had carefully fenced
off (**a hedge written from method discipline, now vindicated by
measurement**); and **NA-008 splits into two halves**, one probed and one
with **no instrument in existence**. **Gates, reported for the first time
in seven filings:** `cargo test --workspace` **exit 0, 121 passed, 0
failed**, `fmt` and `clippy` clean on the root workspace.

> **★★ A NINE-SITE CORRECTION SWEEP, issued with this filing, and the
> defect it corrects is this document's own.** **Nine statements across
> three documents said saturation in B2A had *"never been run"* about
> work that was finished, measured and written up in
> `docs/TOLERANCES.md` **§3.4.4.6** on the same calendar day.** Per this
> document's convention **the old text is not edited** — this note is the
> correction, and it names every site so the sweep is checkable:
>
> | document | sites |
> |---|---|
> | **`ROADMAP.md`** | lines **126**, **171**, **2287**, **2292–2293**, **2387**, **3303–3304** — six |
> | **`NUMERIC_CLAIMS.md`** | **§7.7**'s newly-owed row and **§7.8**'s successor — two *(the second also correctly distinguishes this from Pass 5's saturation gap; **keep that distinction**)* |
> | **`NEXT_SESSION.md`** | **§3**, first bullet — one *(fixed by rewrite)* |
>
> **Nothing was wrong and nothing contradicted anything. The finding
> simply never propagated out of the file where it landed.** The
> proximate cause is recorded in `NEXT_SESSION.md` §4 and is **not
> carelessness**: at the Pass 6 + Pass 7 filing **`tools/` was
> deliberately not re-read**, because `icc-conformance` was working
> there. That is a **sound** protocol against write collisions, and it
> has a known cost — **anything finished in the un-read tree is
> invisible to the filing and gets carried forward as "owed."**
>
> **★ The guard, adopted from here on:** when a filing skips a directory
> because another agent holds it, **record WHICH directory was skipped
> and mark every dependent item `unverified-this-filing`, not `owed`.**
> **They are different claims and only one is safe to act on** — *"owed"*
> invites the next session to redo finished work, which is exactly what
> it did. **And keep the two saturation items distinct**: this was an
> **evaluation** gap and it is closed; **Pass 5's is a *capability* gap**
> in iccce's BPC estimation subset and is untouched.

**★ A dated correction issued with this filing, because the new evidence
found it.** The commit *"untrack tools/gen-profiles"* is **`edce48b`**,
not **`edcb60e`**. `edcb60e` matches no prefix in `.git/logs/HEAD`
*(verified — read)*, and it is carried in this document at the two
places below, in `SESSION_LOG.md` at three, and was carried in
`NEXT_SESSION.md` (now corrected there). **`NUMERIC_CLAIMS.md` §2.6 has
it right**, because it came by a different route. **The old text is not
edited** — this note is the correction, and the incident is in
`ARCHITECTURE.md` **DL-024**.

**★★★ Updated again 2026-08-12 (same day, latest) — OVERALL STATUS: THE
ORIGINAL SCOPE (PASSES 0–7) IS COMPLETE AND FILED. This document's
remaining content is Pass 8 (built in `pdfce`), Passes 9–10, and the
standing debts.** Five things landed after the Pass 4 closure, and the
first of them is the day's largest finding.

**★★★ 1. The black-point ESTIMATORS are discriminated at last — and
lcms2 has TWO of them at media-relative, selected by the DESTINATION'S
DEVICE CLASS AND COLOUR SPACE** (`cmssamp.c` L370–374: output class +
ink space → `BlackPointUsingPerceptualBlack`, which **forces the chroma
to zero**; everything else → `BlackPointAsDarkerColorant`, which **keeps
it**). Measured on two arms: `USWebCoatedSWOP.icc` (v2 `prtr` CMYK)
diverges from iccce's ISO/CD 18619 estimate by **8,166 8×10⁻² ΔE76,
100 % in `L*`, chroma exactly zero**; a **new** synthetic v4 **RGB**
fixture diverges by **5,000 000 ΔE76, 100 % chroma, `ΔL*` exactly
zero**. **★★★ The corpus's pre-registered prediction therefore resolves
in OPPOSITE DIRECTIONS on the two arms** — FALSIFIED on one, CONFIRMED
on the other — and *a session that ran only one arm would have filed a
confident wrong headline either way*. `ARCHITECTURE.md` gains
**DL-027**, which generalises **DL-021** from *direction and path* to
**profile class**. `NUMERIC_CLAIMS.md` gains **§3.17** and **§3.18**,
rows **NC-129 … NC-144**.

**★★ 2. Pass 5b's headline was 98,3 % APPARATUS, and three graded rows
are INVERTED rather than deleted.** Pass 5b could not read lcms2's black
point and **recovered** it through `A2B1 ∘ B2A1`; Pass 5c **reproduced**
it from source instead. Its **0,858 17 ΔE76** becomes **8,166 8×10⁻²**;
its *"mechanism CONFIRMED"* verdict is **WITHDRAWN**; its
*"shape NOT ESTABLISHED"* was **the correct call**; and its error bar of
**0,813 7** — reported at the time as *marginal, passing by 5 %* —
**was not an error bar at all, it was the measurement.** Everything is
kept with its old verdict visible (`NUMERIC_CLAIMS.md` §3.17).

**★★ 3. An APPARATUS FAULT, and the method rule it earns.** `transicc`
prints ink spaces as `0..100` and **RGB and gray as `0..255`**; every
oracle output in Passes 5, 5b and 5c had been divided by 100, correctly,
because until now every destination in reach was CMYK. It was caught
**only** because the validation arm carried a **second, independent
candidate** and both missed by the same amount. **`ARCHITECTURE.md`
DL-028: a residual that is large under EVERY hypothesis is an apparatus
fault, not a finding.**

**★★ 4. Pass 6's gate PASSES — at a new default grid, against an
UNCHANGED tolerance.** `TOLERANCES.md` §3.6.1 said *"the remedy is the
grid, not the number"*; commit **`189e732`** moved
`compiled::recommended_grid_points` from 17 to **33**, and the two red
rows went green against the same **2,5×10⁻¹** ΔE2000: **1,677 3×10⁻¹**
on the benchmark's 513 probes and **9,348 6×10⁻²** on Pass 4's 341-point
grid. **Two things must travel with that**: at grid 33 the two probe
populations **stop agreeing** (the first is **1,79×** the second), so
quoting either alone is now a *population claim*; and the green has a
price — `iccce bench`'s break-even moves from **≈70 000 px to ≈1,19
million px**. `NUMERIC_CLAIMS.md` gains **§3.19**, rows
**NC-145 … NC-152**.

**★ 5. Pass 1's last remainder is closed at its cheapest end, and three
pieces of engineering landed with no Pass of their own.** **ΔE94 and
ΔE CMC** are implemented, transcribed from lcms2 and cross-checked
against a **C probe compiled against the pinned library** — matching to
**all ten printed decimals on three pairs, first run** — and labelled
**`impl_crosscheck`, NOT ground truth**, with a standing strength table
in the module itself and a test asserting that **CMC is ASYMMETRIC on
purpose** so nobody "fixes" it (**§3.20**, NC-153 … NC-156). The **ISO
estimator was WIRED to a caller** — it had none, so the shipped binary
went on refusing the exact case ISO 4.2.5 exists for (**NC-157**;
*an unused capability is not a feature, and it is not a measurement
either*). **Four API soundness defects** were fixed, including a **rule-1
stale-inverse hazard on a public field**, and the **API sealing split**
was decided — *seal what decodes our format, publish what implements
someone else's specification* — filed as **DL-029**. And **CI has run
and passed** *(reported by `icc-engineer`; no agent has observed the
run, and whether it constitutes this project's first Linux execution is
**unstated** — see `NUMERIC_CLAIMS.md` §7.11)*.

> **★★★ ONE OPEN QUESTION IS DISPATCHED AND UNANSWERED, and iccce may be
> the one that is wrong.** The entire `swop` divergence above is a single
> line: at ISO/CD 18619 **4.2.5.4**'s mid-range straightness
> short-circuit — which **both** implementations take, so **neither fits
> a quadratic on either fixture** — **iccce returns `outRamp[first]`**
> and **lcms2 returns `InitialLab`** (`cmssamp.c` L536). **Dispatched to
> `icc-spec-librarian` 2026-08-12: which of the two does ISO/CD 18619
> 4.2.5.4 specify?** **If it names lcms2's, iccce is WRONG rather than
> divergent, and the engineer changes the code.** Until it is answered,
> **no document here may describe this difference as lcms2 departing
> from the standard** — rule 7 says a disagreement is settled from the
> specification, not from which implementation is ours.

**Updated again 2026-08-12 (same day, latest): ★★★ THE OPEN QUESTION
ABOVE IS ANSWERED AND IT WENT AGAINST US. `iccce-measure` lands as a
FIFTH crate. And a test count that looked like a regression turned out
to be a different instrument.** Three things, and the first is the one
that changes shipped behaviour.

**★★★ 1. iccce was NON-CONFORMANT at ISO/CD 18619 4.2.5.4; lcms2
conformed. The code is corrected at `fd34a44`.** The clause's final
paragraph says the `DestinationBlackPoint` *"shall be the same as
InitialLab"*; `outRamp[first]` — what iccce returned there — appears in
the whole of 4.2.5 only as `MinL`, a threshold and a `yRamp` anchor,
and **is not a candidate for the black point in any branch**. **The
blockquote above is therefore DISCHARGED, in the direction it named as
possible.** Cost of the defect: **0,0817 ΔE76** on `USWebCoatedSWOP` —
**100 % of the `swop` arm's measured divergence** (NC-142), *measured
before it was found*. A corollary travelled with the fix: the return
type widened to a full `Lab`, because the short-circuit is the **only**
branch of 4.2.5 that can return a **chromatic** black (4.2.5.2.1 zeroes
chroma for CMYK only), so a Gray or RGB LUT destination gets one from
ISO itself. `ARCHITECTURE.md` gains **DL-030**; `NUMERIC_CLAIMS.md`
gains **§3.24** and **NC-164**. ★ **Rule 7 has now run in the direction
it was written to be capable of running in**, and the pre-commitment
that made that cheap — *"rule 7 is not a licence to assume iccce is
right"* — was written before the answer existed.

**★★ 2. A FIFTH crate: `iccce-measure`, Pass 10 pre-work.** A
CGATS/IT8.7 measurement-file reader, landed at **`2a2d616`** on the
operator's authorisation of 2026-08-12. **INVARIANT: no ICC and no
colour maths**, so its tests need no ICC fixture and a future profiler
and a future measurement tool can share it. Zero dependencies. See the
**Pass 10** section for the full record and `ARCHITECTURE.md` §1 for the
crate's place in the layout. ★ **It had been in the build and in no
document**: at the time of this filing `grep -c iccce-measure` over
`ROADMAP.md`, `ARCHITECTURE.md` and `SESSION_LOG.md` returned **0, 0,
0** *(verified — re-run as a search over `docs/`, which found the crate
named only in `Cargo.toml`, `Cargo.lock` and its own two files)*.

**★★ 3. "Suite green at 142" was never a `cargo test` count, and this
document must never carry a bare one again.** Three green results exist
on this tree from **three different runners**: `cargo test --workspace`
→ **129 passed, 0 failed**, exit 0; `cargo test` in `tools/difftest`
(outside the workspace by design) → **36 passed**; the **conformance
runner** → **pass=142 fail=0 skip=3 error=0**. The 142 lives in commit
`d5efd96`'s *message* — **not in any document here** *(verified — the
string appears in `docs/` only as the CIE standard number 142-2001)* —
which is precisely why it could not be scoped by a reader who met it.
`ARCHITECTURE.md` gains **DL-031**: **an unlabelled test count is not a
claim, because the apparatus is half the number.** `NUMERIC_CLAIMS.md`
§3.22 carries all three with their commands.

> **★★ AND ONE STANDING CLAIM IS WEAKENED RATHER THAN UPDATED.** The
> Pass 6 throughput/speedup figure has now been measured **three times
> on the same machine and the same code** and the readings span
> **2,7×** on throughput and **12,18×–22,85×** on speedup. **No single
> figure is supportable and none is quoted below any more.** The honest
> form is **"12–23× on this machine, load-dependent"**, the break-even
> moves with it, and a **fourth**, differently-shaped set of figures
> exists in `TOLERANCES.md` §3.6.2 that this librarian does not own and
> has flagged rather than reconciled. Full treatment, with every
> reading and its apparatus: `NUMERIC_CLAIMS.md` **§3.23**.

**Updated again 2026-08-12 (same day, latest): ★★★ THE PREDICTION IN
THE BLOCK ABOVE IS FALSIFIED. The `swop` black-point divergence did NOT
collapse on the corrected 4.2.5.4 code — it GREW 58,8×. And the reason
is that agreement with the oracle had been the symptom of our
defect.** Filed at tip **`2881e19`** *(verified — `.git/refs/heads/master`
read)*; code corrected at `fd34a44`, harness at `cc03f3d`, oracle pin
`21c582a`. **The measurement is `icc-conformance`'s and is carried; the
mechanism is verified from source.**

**★★★ 1. The re-measure, and what it means.** Block 1 above recorded the
defect's cost as **0,0817 ΔE76 — 100 % of the `swop` arm's divergence**,
and `NUMERIC_CLAIMS.md` §7.12 then predicted the divergence would
**collapse** once both implementations returned `InitialLab`. It did the
opposite:

| | before `fd34a44` | after |
|---|---|---|
| ISO 4.2.5 black (iccce) | `L* 16,489 806` | **`L* 11,772 365`** |
| lcms2 (reimplemented, pin `21c582a`) | `L* 16,571 474` | **unmoved** |
| **the divergence** | `8,166 8×10⁻²` ΔE76 | **`4,799 109` ΔE76** |

**This is not a bug.** Both sides take the straightness short-circuit
and both return what their own document calls `InitialLab` — **ISO
4.2.2.2 means the darkest DEVICE VERTEX neutralised; lcms2's
`cmsDetectBlackPoint` means the PERCEPTUAL BLACK ROUND TRIP with chroma
zeroed.** Two documents meaning different things by one name: **rule 7 in
its sharpest form, and this time neither side is wrong.**

**★★★ 2. The methodological finding, which outlives the number.** The
non-conformant return was `outRamp[first] = MinL = 16,489 806`, and
**`MinL(lcms2) = MinL(ISO) = 16,489 806` exactly** — so the defective
code sat **0,082 `L*`** from lcms2's answer **for a reason unrelated to
being right**. The defect's own magnitude, **`4,717 441 L*`**, was
**57,8× the divergence it was blamed for.** ★ **The cross-check built to
catch exactly this was nearly blind to it, because being wrong in that
particular way looked like being right.** `ARCHITECTURE.md` gains
**DL-033**: *a cross-check's power is bounded by the separation of the
two CANDIDATE answers, not by the tightness of the residual it
reports.*

**★★ 3. NA-009's cost is MEASURED at last** — `4,799 109 ΔE76` (`swop`,
100 % `L*`) and `5,000 000` (synthetic, 100 % chroma) — after four
filings of "unmeasured". **Three caveats travel with it and are not
optional:** it is a cost **at the black point only**; it is measured
**relative to lcms2, not to truth**; and ★ **there is NO ground truth in
this comparison** — no published black point exists for
`USWebCoatedSWOP.icc` and 18619 is a **committee draft**. It reads as a
cross-check throughout and **must never be promoted**.

**★★ 4. The synthetic fixture could not see any of this.** Its
`InitialLab` and `outRamp[first]` are **both `L* 20`**, so swapping them
changed nothing and its `5,000 000` is *identically* unmoved. **The
vendor profile was the only arm with the power** — because nobody
designed it. `ARCHITECTURE.md` gains **DL-036**, the stated converse of
DL-020: an authored fixture discharges *the doubt it was authored for*
**and nothing adjacent to it.**

> **★★ AND THE SPEEDUP IS NOW WITHDRAWN OUTRIGHT — the block above's
> "12–23× on this machine, load-dependent" is no longer the honest
> form.** `icc-conformance` measured it ten more times and found it
> spans **2,03× within ONE session at ONE grid**; the decision at
> `TOLERANCES.md` §3.6.3(b) is that **this project does not carry a
> speedup figure at all**. What survives is the **break-even, stated
> with its grid**: **≈1,3×10⁶ px at grid 33**, whose `85 900 →
> 1 273 800 px` shift is **14,8×** — matching the median build time's
> `0,838 → 12,444 s` **to three figures**, which is the arithmetic
> saying the shift is *entirely* the build. `NUMERIC_CLAIMS.md`
> **§3.27**, **NC-170 … NC-173**. Two more decisions come out of the
> same sweep: **DL-034** (a claim-bearing number the harness can compute
> is **formatted at run time**, never typed into prose beside it) and
> **DL-035** (**an improvement whose cause is the denominator or the
> rival is not an improvement**).

---

### 2026-08-12, later still — **DL-033 gets its instrument, and the instrument's first act was to tell us our best row could never have failed**

*(Third filing of the day's third session, tip **`e26d9ba`**. **No plan
text and no earlier block is rewritten.** Row figures and the census are
**carried** from `icc-conformance` via the engineer; the mechanism, the
guards and the corrected literals are **verified from source**. This
librarian has no shell — fourth consecutive filing.)*

**★★★ 1. Candidate separation is now an emitted field on every
conformance record.** DL-033 said a cross-check's power is bounded by
the distance between the two **candidate** answers, not by the tightness
of its residual — and left that as prose nobody could enforce.
`tools/difftest` now prints, per row, the named rival candidate, the
value the row would have observed under it, and the distance. **Two
design guards are the entry** (`ARCHITECTURE.md` **DL-037**):
**`UNGRADED` is tested BEFORE the comparison**, because `d ≤ ∞` holds
for every finite `d` and a naive test would brand every DL-019
report-don't-grade row `BLIND` — blaming the fixture for a decision the
**tolerance** made; and **`ZERO-SEPARATION` outranks everything**,
because a blind row is rescued by tightening a number and a
zero-separation row only by **a different fixture**. ★ **`BLIND` does
not affect status or exit code, deliberately** — if disclosing a
weakness could turn a row red, the cheapest response is to stop
disclosing.

**★★★ 2. And the first run's finding is about our own instruments, not
about colour.** The row carrying the **entire 4.2.5.4 result** —
`pass5c/swop/estimators/black-points-in-lab` — **is graded at infinity**,
so it **could never have failed however far the candidates moved.** Its
separation is **`4,717 441`**, the defect magnitude to six figures, and
**no gate consumes it.** The suite's real power on that question lives
in **§B's *device* rows**, which is a far less obvious place than the
row named *"estimators"*. `NUMERIC_CLAIMS.md` **§3.28**, **NC-176**.

**★★ 3. Coverage, and it is not to be rounded up: 16 of 145 rows carry
a separation, all Pass 5c's.** `blind=0` is out of **16** — strictly,
out of the **six** rows that reached the comparison at all. **129 rows
have had no rival candidate considered.** DL-033's operational item is
**closed for Pass 5c and open everywhere else**, and it must not be read
as closed because the mechanism exists.

**★★★ 4. A RISK, not merely a coverage gap: the corrected 4.2.5.4
clause is documented but UNDEFENDED on any clean machine.** The only
differential arm with power is a **Windows system profile**. On CI — or
any machine without that directory — **those rows skip and everything
stays green through a full reversion of the fix.** A third, purpose-built
arm is **commissioned**, with the requirement that its power be
**demonstrated by injecting the reverted behaviour** rather than
asserted. ★ **This collides with the Linux debt owed since Pass 0**: the
one platform never run on is the platform where the detector is absent.

**★ 5. `skip=3` is enumerated at last**, owed for three filings and
blocked on a shell each time. **Three rows, one cause** — the Pass 4
`icc-absolute` PCS-isolation rows, withheld because `transicc` applies
the D.6/D.7 media-white scale on lcms2's side and iccce's A2Bx is
media-relative by construction. **Grading them would mean reproducing
lcms2's absolute handling, i.e. modelling the oracle rather than
measuring it.** ★ **A principled refusal to grade, not a concealed
failure** — and `fail=0` could never have said so.

**★ 6. A fourth stale literal, found by an apparatus rather than a
person** (**DL-034**'s fourth instance): a justification asserted the
candidates were `2,46×10⁻³` apart, *"three orders above the bound"*;
computed, `9,574 451×10⁻³` — **four**. **The argument was never harmed,
only the number**, and the stale figure **understated** the claim — which
is exactly why nobody notices a stale literal inside a justification.

---

### 2026-08-12, latest — **the block above's most load-bearing sentence was FALSE, the engineer measured it so, and the fixture that replaces it fails by 32 768×**

*(Fourth filing of the day's third session and the eighteenth overall,
tip **`6c7cda1`** *(verified — the last line of `.git/logs/HEAD`; it
says **nothing** about whether that tip has been pushed)*. **No plan
text and no earlier block is rewritten.** Measured figures are
**carried**; every constant, table and doc comment cited is **verified
from source**. This librarian has no shell — **fifth consecutive
filing**.)*

**★★★ 1. RETRACTION, and it is the dispatcher's own.** The block above
says *"the corrected 4.2.5.4 clause is documented but UNDEFENDED on any
clean machine"* and this document filed it as a **RISK**. **Both halves
are false, and `icc-engineer` measured them before this ledger could
repeat them.** A full reversion of `fd34a44` makes **`cargo test -p
iccce-cmm` fail — exit 101, 62 passed / 2 failed**: the clause was
**defended all along, as a function, on a synthetic closure.** And the
second half was an *understatement* — the same reversion turned **no
conformance row red on ANY machine**, not merely on machines without a
vendor profile, because the row carrying the finding is `REPORTED` at
`inf`. ★ **The corrected sentence is narrower and more useful: what had
no detector was the clause exercised THROUGH A PARSED PROFILE** — the
`parse → LUT model → estimator` path, where a wiring defect lives that a
closure test structurally cannot reach. `NUMERIC_CLAIMS.md` **§3.29.1**;
§3.28.5 is **amended in place, not rewritten**.

**★★★ 2. The third arm exists and its power was PROVEN, not asserted.**
`fixtures/synthetic/v4-rgb-mab-floored-b2a.icc` — `InitialLab` at
`L* 12,5`, the rival `outRamp[first]` at `L* 37,5`, **25 `L*` apart by
construction.** Its clause row is graded at **`7,629 511×10⁻⁴`**, which
is **half one PCSLAB encoding quantum and nothing else**: no
interpolation term (a CLUT corner through identity curves), no oracle
term (none is consulted), chroma terms exactly zero. With the defect
injected **and both vendor-profile paths repointed at a non-existent
drive**, it is **the only failure in the suite** — `pass=129 fail=1
skip=30` — at **`2,500 019×10¹`**, i.e. **`3,28×10⁴` times its bound**
*(derived here)*. **It also fails as a unit test**, and since `2835d23`
the `tools/` trees are gated in CI on **ubuntu and windows**. ★ **The
Linux collision this document recorded one filing ago no longer exists
in that form** — though **no CI run has yet been observed by anyone
here, and a workflow file is not a run.** **NC-179/NC-180**.

**★★★ 3. The separation mechanism was itself lying, exactly where it
mattered.** `Separation::against` derived its distance as `|observed −
alt_observed|` — which **collapses to zero on the very run where the
code returns the rival**. The proof-of-power run therefore printed
**`ZERO-SEPARATION` beside a row that was failing at `2,500 019×10¹`**:
the mechanism disclaiming its power in the instant it demonstrated it.
**`ARCHITECTURE.md` gains DL-038.** ★ **DL-037 recorded the guard order
as the design; this records that the measurement UNDER the guards had
the defect the guards were built to catch** — and it was found by *using*
the instrument on the case it was built for, one filing after this
document celebrated it. The diagnostic now lives on the constructor's
own doc comment: **is the distance a property of the RUN or of the
FIXTURE?**

**★★ 4. A control failed and was NOT accommodated.** The new arm made
`apparatus/error-bar-is-smaller-than-the-effect` come out at
**`3,775×10⁹`**, because the fixture's floor makes `d(device)/d(L*)`
zero by construction (**`1,11×10⁻¹⁶`**). **`APPARATUS_RATIO` was not
widened**; an authored `DEVICE_OBSERVABLE` table was added, plus a row
grading the measurement against the declaration, **so the exemption
cannot be acquired by a number coming out small.** **DL-043** — a worked
instance of *tolerances are justified, not tuned*.

**★★ 5. Pass 4c's separations, and a tolerance that was DECLINED.** Ten
of ten rows priced — **four real, six honest absences with reasons**;
**`unstated` 129 → 119, `discriminating` 6 → 16**, and **41 of 160 rows
now state a separation.** Two rules out of the six absences (**DL-039**):
*a rival tolerance is not a rival candidate*, and *name the rival that
most threatens the row*. And the question of whether NC-176's
`4,717 441` separation now justifies a real bound was answered **NO**
(**DL-040**): **a large separation on an `UNGRADED` row is a request for
a fixture and a graded row elsewhere, not a licence to grade that row.**

**★★★ 6. The eleven-filing ground-truth blocker is SPLIT — and part of
it was a wrong REJECTION, not a gap.** **ICC.1:2022 Annex D.6.3**
publishes twelve exact integer PCS encodings and **all twelve
reproduce**; **Table 16** is normative and its five pairs reproduce too.
**But for the LUT path the limit is STRUCTURAL:** ICC.1 mandates no
interpolation method, so two conforming CMMs may legitimately differ and
**no single value could be published as expected** — corroborated by
**iccDEV, ICC's own reference implementation, shipping zero expected
colour values.** ★★★ **NOTHING IN iccce HAS BEEN COMPARED TO ANY OF
IT.** The ground truth is *available*; the row is *buildable*; **the row
does not exist**, and **NC-001 remains this project's only
`published-ground-truth` row.** **DL-041.**

**★★★ 7. And the reason it sat owed for eleven filings is the finding
that should change how this document is maintained.** The Annex D data
had been examined months ago and **REJECTED** — by point-evaluating
values that are intervals. ★ **A negative finding removes its own
auditor: nobody re-tests a fixture they have been told is broken.** The
corpus's four previous defects were all wrong *assertions* and all were
caught within days; this was a wrong *rejection* and survived
indefinitely. **DL-042: when an item has been owed for many cycles,
re-audit the REASON it is owed, not just the item.** Eleven filings
restated this blocker and **none re-read the entry that created it.**

**★ 8. Open, and NOT an engineering question.** Whether **published
numbers** may live in an MIT repository as fixtures — ICC's Annex D
values, CIE's CC BY-SA tables and ECI's self-contradicting `cprt` —
**is one operator decision, not three**, and item 6's fixture is blocked
on it. Rule 9 unchanged.

---

**Updated 2026-08-17 (five days later, and the first update to this
document since 2026-08-12): ★★★ a NEW STANDING WORKSTREAM — Ghent
compatibility — and a change of POSTURE from the operator that reaches
back into work this document has parked.** *(Filed by `icc-librarian`
from an `icc-engineer` dispatch, at tip **`e21154c`** *(carried — this
librarian has no shell and did not run `git`)*. **No Pass status
changes. No plan text is rewritten.** Two earlier 2026-08-17 sessions
touched other documents and **deliberately made no `ROADMAP.md` entry**;
this is therefore this document's **first 2026-08-17 material**.)*

**★★★ 1. The operator's instruction, verbatim:** *"I know some things
you stopped work on because they required physical testing that we don't
have. We aren't going to aim for compliance like that. Just aim for
compatibility."* **Filed as `ARCHITECTURE.md` DL-045.** ★ **It changes
what is CLAIMED, not how well a claim must be supported** — rules 1, 3,
4 and 5 are untouched, and nothing in it licenses a looser tolerance.
The habit it corrects is a **category error**: *"this cannot be
certified"* was being read as *"this cannot be checked"*, and DL-041's
three kinds of blocker gain a fourth — **an item blocked on an
ORGANISATIONAL fact about a certification programme, which is not an
engineering blocker at all.**

**★★ 2. A new durable document, and it is not this librarian's.**
`docs/GHENT_COMPATIBILITY.md` *(verified — nine sections, read)*, written
by `icc-engineer`: the compliance/compatibility distinction, what the
suite is, its licence, the boundary split against `pdfce` patch by patch,
what was measured, **what is explicitly NOT claimed**, what would raise
each claim, certification, the operator decisions owed, and a
per-statement provenance table tagging every claim VERIFIED / QUOTED /
REPORTED.

**★★ 3. The first corpus this project has measured that was authored by
neither this project, an operating system, nor a standards body.**
**98 PDFs → 121 embeddings → 20 distinct ICC profiles**, from
**Adobe, ECI, Heidelberg, X-Rite and GWG**. `NUMERIC_CLAIMS.md` gains
**§3.30** and rows **NC-192 … NC-199**, plus **two new evidence classes**
— `fixture-declared-categorical` and `acceptance`. ★★★ **NOT ONE OF THEM
IS AN ACCURACY CLAIM**, and that is not modesty: the lcms2 differential
over this corpus was dispatched the same day and **had not reported**.
**NC-001 remains this project's only `published-ground-truth` row**
(DL-041 unchanged).

**★★★ 4. The strong result, and it is a shape this project has never had
before.** GWG ships two **deliberately corrupted** profiles — a
red/green-swapped RGB matrix profile and a cyan/magenta-swapped CMYK LUT
profile — as discriminators. iccce honours both swaps, **and the CMYK arm
was re-derived with a CONTROL** (the genuine `ISO Coated v2 300% (ECI)`
from the same patch). The expected answer is a fact about the file's own
declared content, so there is **no oracle and no instrument in the
claim**, and the **candidate separation is the full width of the gamut**
— the named alternative being an engine that drops the source profile
for ISO 32000-1 Table 66's `/Alternate`, **which is what `pdfce` does
today**. ★ **What it licenses:** *iccce applies the declared source
profile rather than falling back to a device alternate.* ★★★ **What it
does NOT license: any sentence containing a number.** A CMM wrong by
20 ΔE2000 passes it as cleanly as a correct one, provided it swaps.

**★★ 5. Ghent can never supply a numeric expectation — DL-047.** The
suite states **no tolerance, no reference measurement and no expected
colour value anywhere**; its criterion is *"a clear X"* at *"0.5 m"*,
and *"A faint X is NOT a failure"*. ★ **And it contradicts itself
exactly on our topic:** GWG 13.0 signals a rendering-intent failure with
a **faint green X** — the symbol already declared not to be a failure —
so **intent handling is untested by the suite's own criterion**, and any
iccce intent claim must be graded against **ICC.1's** text instead.

**★★ 6. A rule about promoting reports — DL-046.** The CMYK swap arrived
`[REPORTED]` from a byte-level scan and was **re-derived through the
shipped binary, with a control**, before becoming a finding. Six further
leads from the same scan were **not** re-derived and are filed as leads.
★ **The rule is visible only because both outcomes happened on the same
day: verify in the RUNNING THING, and where the claim is "X changed",
add the case that should NOT have moved.**

**★ 7. Three decisions are owed to the operator and no agent may take
any of them** — see the workstream block below and `NUMERIC_CLAIMS.md`
§7.16. The first is the binding one: **no public artifact of this
project may say "Ghent" without GWG's written permission**, so
*"passes the Ghent suite"* is **not available as README or crates.io
copy today**. Nothing has been published.

---

**Updated again 2026-08-17 (later the same day, third filing): ★★★ PASS G
— the lcms2 differential over the Ghent corpus LANDED, and with it this
project's first cross-check rows on a corpus a real document producer
embeds.** *(Filed by `icc-librarian` from an `icc-engineer` dispatch
carrying `icc-conformance`'s work, at tip **`e21154c`**. **No Pass status
changes** — Pass G is part of the **Ghent workstream**, not a Pass; see
that block for the detail and for what it did not do.)*

**★★ 1. The suite is bigger and it is green.** `cargo run --release` in
`tools/difftest` gives **`pass=229 fail=0 skip=3 error=0`, exit 0** — up
from `pass=157` — from **72 new graded rows in four sections**
(`tools/difftest/src/passg.rs`), **every one of which states a candidate
separation**. `discriminating` goes **16 → 42**. *(`icc-engineer`,
**[VERIFIED — ran the gate bare, redirected to a file, read `$?`]**.)*
★ **Corpus-absent — which is CI, permanently — it is `pass=157 skip=7`,
exit 0**, four labelled SKIP records standing in for all 72 rows. **A
green CI line means Pass G did not run**, and the corpus-absent pass
count is **numerically identical to the pre-Pass-G total**.

**★★★ 2. The headline result: the raw iccce-vs-lcms2 disagreement on a
VENDOR-AUTHORED v4 `mAB ` profile IS the interpolation method and nothing
else.** With lcms2's own `Eval4Inputs` geometry substituted, the residual
collapses **179×** (`A2B1`, `0.828 444 → 4.624 5×10⁻³`) and **243×**
(`A2B0`, `0.950 274 → 3.912 3×10⁻³`); the envelope computed **from the
CLUT's own bytes and two published algorithms, with no lcms2 output in
it**, accounts for the raw residual to **0.04 %** and **0.22 %**.
`TOLERANCES.md` §3.4.3's *"any **real** v4 LUT profile"* gap, open since
2026-08-11, is **closed**. ★★ **But the structural gate (envelope × 1.25)
explicitly CANNOT claim agreement and is labelled so** — the agreement
claim lives in the substituted-geometry row alone, at `2×10⁻²`. **Never
merge the two arms into one "agrees with lcms2" sentence**
(`NUMERIC_CLAIMS.md` §3.31.2).

**★★ 3. rule 7 ran against a third party's files.** Ghent's Adobe-embedded
`sRGB` and `Adobe RGB (1998)` have **D50-adapted PCS data, a `wtpt` that
is NOT adapted, and no `chad` tag** — a defect of **authorship** under
`ICC.1:2001-04` Annex A.3.1.1, which **settles the ICC-absolute divergence
in lcms2's favour**. `eciRGB v2` is the control that stops it being read
as *"every v2 display profile in the wild"*. ★ **No `NA` was registered
and no code changed** — whether iccce should follow lcms2 here is an
engineering call with a cost, and it is **newly owed** (`NUMERIC_CLAIMS.md`
§7.17 newly-owed 4).

**★★ 4. Two decision-log entries, and neither is about colour.**
**DL-048** — *a stale claim-bearing **citation** is worse than a stale
**number**, because a wrong number invites re-derivation while a wrong
pointer invites the reader to accept whatever is at the destination*;
**six of six** spot-checked ledger line-citations were stale, and the one
that escaped into an outbound `pdfce` request is corrected at both ends.
**DL-049** — *a disclosure field that gates nothing caught a defect in a
**tolerance's justification**, on a row that was green*: `BLIND` fired,
and the fix exposed that a `2×10⁻⁴` encoding-floor bound **did not hold
for the profiles it was gating**.

**★ 5. What Pass G did NOT do.** It touched **11 of the corpus's 20
profiles**; it has **no attribution row for §B** (the harness has no
`mft2` B2A model), so §B's 17–63× margin is **not** an agreement claim;
it grades **no `mBA ` (B2A) direction** of the X-Rite profile; **eight
`--bpc` combinations are refused by name and therefore differentially
untested**; and it produces **no published ground truth and cannot**
(DL-041/DL-047). **NC-001 remains this project's only
`published-ground-truth` row.** `NUMERIC_CLAIMS.md` gains **§3.30.10**
(two corrections to §3.30), **§3.31** with rows **NC-200 … NC-218**, a
second dated measurement under **NA-006**, and **§7.17**.

**★★★ Updated again 2026-08-17 (later the same day, latest): TWO
CONSUMER-FACING CAPABILITIES LANDED THAT HAD NO ROADMAP ENTRY AT ALL, an
lcms2-independent sRGB was constructed and measured, and a CORPUS CLAIM
WAS FALSIFIED by the test written to honour it.** Filed by
`icc-librarian`; **`NUMERIC_CLAIMS.md` §3.32 (NC-219 … NC-229) and §7.18**
carry the rows, and `ARCHITECTURE.md` gains **DL-050 … DL-053**.

**★★★ Read this first, because it bounds everything below it.**
**Nothing from this session is committed and nothing is pushed**
*(carried; this librarian has no shell)*, so **every row in §3.32 is
against an uncommitted working tree and has NO COMMIT ANCHOR.** ★★ **The
conformance runner was NOT run** — `icc-conformance` holds
`tools/difftest` and `docs/TOLERANCES.md` for a **concurrent Pass H**, and
both are **untouched by this filing**. **`pass=229` remains NC-218's dated
observation at `e21154c`; nothing here re-measures it, and a `pass=` line
must not be quoted on this filing's authority.**

**★★ 1. The built-in sRGB destination is BUILT, and the API question is
DECIDED.** `crates/iccce-cmm/src/builtin.rs` constructs sRGB from
**ITU-R BT.709-6 items 1.3/1.4** (primaries and white), **W3C CSS Color
4** (transfer function) and **ICC.1:2022 Annex E.3 Eq. (E.1)**
(Bradford) — **no I/O, no embedded blob, no dependency, and no lcms2
anywhere in the lineage.** Selection is a **two-variant enum, not
`Option<&Profile>`** (**DL-050**), because an `Option` being `None`
cannot distinguish *"there was none"* from *"I failed to get one"* and
**only the second must never trigger a fallback**; the substitution is
**disclosed** via `DestinationProvenance`. `Chain::new` is unchanged and
**no caller moved**. ★★★ **The measurement is in a THIRD and WEAKER
evidence class — `constructed-vs-reference-file`, neither ground truth
nor a cross-check** (§3.32.1): **max `0.033013` ΔE2000 at pure white**,
device **black exact**, and **the bound is DERIVED AT RUN TIME from the
reference file's own tags** rather than typed (§3.32.9).

**★★ 2. The `/N` accessor is BUILT, and ICC.1 turns out to be SILENT on
the thing it would have been natural to assume.**
`crates/iccce-profile/src/colour_space.rs` exposes `components()`,
`channel_agreement()`, `is_valid_pcs()`. **A48: ICC.1:2022 nowhere
requires a LUT tag's channel count to match the header's data colour
space, and nowhere states reader behaviour on mismatch** — so iccce
**discloses** and does **not** call it non-conformant, because *"silent"*
is a different claim from *"requires agreement"*. **A50: the component
count is a TWO-TABLE JOIN (Table 19 + Table 41), not a transcription** —
**ICC.1 publishes no `Signature → count` map**, so cite it as derived.
★ **Population: 60 real profiles, CMYK 33 / RGB 25 / GRAY 1 / 7CLR 1,
zero unrecognised signatures, zero PCS-field violations.**

**★★★ 3. A corpus claim was FALSIFIED, and the corrected version is worse
news.** `ICC_Spec/iec/iec__s__srgb.md` said the wrong sRGB breakpoint
*"affects only 8-bit codes 10 and 11"*. **No 8-bit code lands in the
window at all** — the separation at 8-bit input precision is **exactly
zero**, and the maximum anywhere is **`7.55×10⁻⁷`** in linear light. ★★
**So a wrong breakpoint is invisible to every image, every 8-bit test
vector, every round trip, and every differential against an
implementation that made the same choice.** ★★★ **OWED: the corpus still
carries the wrong version** (§7.18 newly-owed 1, `icc-spec-librarian`).

**★★★ 4. The suite had ZERO POWER against the constant it documented
most.** Five injected defects: Bradford omitted **3 of 6 red**, applied
twice **3 of 6**, gamma-2.2 substitution **1**, green primary **2** —
and **the breakpoint substitution: NOTHING, 6 of 6 passed.** **DL-051:**
*a suite that documents a constant at length while being unable to detect
its corruption is worse than one that says nothing, because it reads as
protection* — **and the gap was found by INJECTION, not by inspection, by
the person who had just written both the constant and the tests.**

**★★ 5. `sRGB2014.icc` is NOT a second source.** ICC's 2015 file has the
compliant `wtpt` and the `chad` the HP 1998 file lacks — and its `rXYZ`,
`gXYZ`, `bXYZ` and **all three TRC tables are BYTE-IDENTICAL** to it.
**⟹ exactly one lineage for those nine numbers; the gap is NOT closed.**
★ Applying the file's **own** `chad` to the BT.709 matrix improves the
residual from **12.0 to 5.35 ULP and no further**, so the corpus's
existing negative conclusion is **strengthened, not overturned.**

**★★ 6. Two owed decisions discharged and one carried figure corrected.**
**§7.17 newly-owed 4 is DISCHARGED**: iccce **does not** substitute D50
for a mis-authored `wtpt` — it uses `wtpt` as stored and **discloses**
(A4c / NA-007), verified **in the running thing**, and **6 of 60 profiles
disclose, all six hand-audited as true positives.** ★★★ **NC-213 the ROW
is untouched and was never pending** — a measured row cannot be
"settled". And **DL-053**: `NEXT_SESSION.md`'s *"two iccMAX"* was **a
count from a sample recorded as a count of the population** — **ten are
present**, and the corrected sweep (**40 parse clean, 10 refused by
name**) is a *stronger* claim than the sample was.

**★ 7. `ChainError` now implements `std::error::Error`**, found by a
**compiled doc example** refusing to build (**DL-052**) — *the cheapest
available consumer, and the only reviewer who has not already learned the
API's shape.* **Gates:** `cargo test --workspace` **154 passed, exit 0**
(was 132); clippy, `fmt`, `doc` and **`wasm32` over the 4 library crates**
all exit 0 — ★ **and iccce still does not GATE `wasm32` in CI**, which is
a **consumer's** gate (`CLAUDE.md` rule 10.2).

**★★★ Updated again 2026-08-17 (same day, LATEST — the supplementary
filing). ICC's own sRGB document arrived and REVERSED an attribution the
block above makes; a shipped crash was found and fixed; and two tests
were measured to be blind. Nothing is committed and nothing is pushed.**
Full record: `NUMERIC_CLAIMS.md` **§3.33** (NC-230 … NC-242), **§4's
NA-011**, **§7.19**; `ARCHITECTURE.md` **DL-054 … DL-057**.

**★★★ 1. Items 1 and 5 of the block above are SUPERSEDED in their
attribution, not merely extended.** The operator obtained, in a browser,
**"How to interpret the sRGB color space (specified in IEC 61966-2-1) for
ICC profiles"** (Jack Holm, ICC, 2015-04-27). **§B.2 publishes the
D50-adapted colorants — and ICC's recommended D65→D50 `chad` — at 15
decimal places.** The corpus's standing *"NO document publishes them"*
was **false; it was behind `color.org`'s robot bar.** Measured against
the published values: **iccce's construction 3.02 ULP worst / 0.90 in
`bXYZ.Z`; the shipped HP 1998 / `sRGB2014.icc` file 11.13 ULP.** ★★★
**The ~12 ULP blue-`Z` residual is the FILE's error, not iccce's** —
**NC-230**/**NC-231**, and the Pass 8 retrospective's done-when row is
corrected in place. ★★ **NC-225 is unaffected**: there is still exactly
one lineage among the *files*; what changed is that a *document* now sits
outside that lineage.

**★★★ 2. The ledger's SECOND `published-ground-truth` row in a year of
having one.** NC-230 carries it, under four stated conditions (§3.33.2) —
scope is **nine numbers, not a transform**; the transcription is **single-
reader and a second reading is owed**; and *published* does **not** mean
*physically exact*, since ICC's own published `chad` misses ICC's own
stated D50 by `4.9×10⁻⁵` (**NC-233**). ★★★ **The sentence *"NC-001 is the
only published-ground-truth row"*, carried by eleven filings, retires
today.**

**★★★ 3. DL-054 — an ACCESS boundary had been recorded as an EXISTENCE
fact, and the mis-attribution survived because THE NUMBER WAS CORRECT.**
The corpus held both registers in one file: its acquisition list said *"no
document **found so far** states"* and named the barrier; its status table
said *"NO document states them."* **The status table is what
propagated.** ★★ Worse, hours before the fetch the same file **lowered
its expectation of the unread document** from evidence about a different
artifact. **Write the SEARCH claim, never the EXISTENCE claim.**

**★★★ 4. A shipped crash: `iccce bench` ABORTED the process** on ICC's
published seven-channel `APTEC_CMYKOGV_Coated_LinearCTV_2025.icc` — bare
`0xC0000409`, *"memory allocation of 1022842631448 bytes failed"*. Two
causes: a **`_ => 33` catch-all** (`33⁷` ≈ **952.6 GiB**) and a guard
using **`checked_pow`, which catches wrap and not size**. Fixed with a
computed ≥5-channel recommendation and `MAX_COMPILED_GRID_BYTES = 64 MiB`
behind a named `ChainError::GridExceedsBudget` (**NC-234**). ★★ **An
abort is the worst available library failure — not catchable, and a
consumer's process goes with it.** ★★★ **And DL-055: each half of the fix
alone makes the conformance row observe zero, so deleting the guard would
have left it GREEN with no number moved and no edit to blame.**
`icc-conformance` split it into four rows, one per layer.

**★★★ 5. DL-056 — the ΔE destination gate is ASYMMETRICALLY BLIND.**
Injected white-point drift: `−3.0×10⁻⁴` fails, **`+3.0×10⁻⁴` PASSES and
reports `0.029008`, better-looking than the correct build's `0.033013`**;
blind to ≈`+3.8×10⁻⁴` (**NC-240**). **A difference cannot detect a defect
that shrinks it.** What has power is the **absolute** assertion
`constructed_colorant_sum_is_d50` — D50 itself, no file in it — which the
same injection fails (**NC-241**). ★★ **Deleting it as redundant would
open the blind spot and every remaining test would stay green while it
happened.**

**★★ 6. DL-057 — a refusal that named the WRONG CLAUSE.**
`Destination::None` on an `A2B`-only profile (four exist in ICC's
published set) reported *"matrix/TRC model requires PCSXYZ (Annex F.3,
normative)"* — true, correctly cited, **and about a model iccce was about
to discard**. Fixed by sharing `derive_source_model()` (**NC-237**). ★★★
**A code path that discards half its result inherits the discarded half's
failure modes, because that half's error is what the caller sees.**

**★ 7. Two owed items DISCHARGED and one blocker MOVED.** §7.18's
tautological-test item is closed and **proven by injection** (the lcms2
behaviour now fails 4 of 6 tests — **NC-238**); the corpus's *"8-bit codes
10 and 11"* claim is **retracted as corpus defect `C8`** *(verified by
this librarian in `ICC_Spec` at the tip)*. ★★★ **And the ground-truth row
for chromatic adaptation — owed since Pass 1's §7 item 4 — moves from
BLOCKED to AVAILABLE-AND-UNMEASURED**, because ICC's recommended `chad`
is in the same §B.2. The instrument is named and the bound must be
derived before it is run.

**Gates:** `cargo test --workspace` **158 passed, exit 0** (was 154);
clippy / `fmt` / `doc` / **`wasm32` over the 4 library crates** all exit
0. ★★ **`icc-conformance` shipped a concurrent Pass H — `pass=274 fail=0
skip=9 error=0`, their measurement, corroborated in `TOLERANCES.md`
§3.8.1** — and **`274` may not be compared with `229`**, because Pass H
added rows.

---

## Pass 0 — scaffold and the oracle

**STATUS: DONE — 2026-08-11.** Evidence in the block below the done-when.
The plan text is unchanged; nothing here was rewritten to match what was
built.

- Cargo workspace, four crates per `ARCHITECTURE.md` §1, MIT throughout.
- `tools/difftest` pinning **lcms2** out-of-tree, with the licence
  verified and recorded before it is relied on.
- CI that builds and tests on Linux as well as Windows. The sibling
  project discovered its `main` had not compiled on Linux for weeks
  because nothing checked; start with the check.
- `iccce-cli inspect <profile>` printing the header and tag table.

**Done when**: a real profile from the system's colour directory can be
inspected, and `difftest` can invoke lcms2 on the same file.

### Pass 0 completion record — filed 2026-08-11 by `icc-librarian`

**Commit:** `f976a0e` (root commit, 2026-08-11, "Pass 0: scaffold,
oracle, and header/tag-table parsing" — 47 files). Hash filled in by
`icc-engineer` immediately after committing, per this record's own
request; the record itself was filed one commit earlier in time but
lands in the same root commit.

**Done-when, clause 1 — a real profile is inspected.** Reported by
`icc-engineer` from a run on this machine (Windows 11 Pro 10.0.26200):

```
iccce inspect "C:\Windows\System32\spool\drivers\color\sRGB Color Space Profile.icm"
  → header: CMM 'Lino', version 2.1.0, class 'mntr', RGB → XYZ
  → tags: 17
  → malformations: 0
  → rTRC / gTRC / bTRC all at offset 1084
```

The shared-offset result is worth keeping: it is a **live confirmation**
of the rule the corpus states in `icc__s__tag_table.md` (two entries may
point at one block of tag data), and it is the same phenomenon as the
`A2B0`/`A2B2` case in `tools/difftest/README.md` §8.4. Both are
properties of real profiles that read as bugs if nobody wrote them down.

**Done-when, clause 2 — `difftest` invokes lcms2 on the same file.**
Recorded verbatim, with its command line, in `tools/difftest/README.md`
§8.2–§8.4: `transicc` at pin `21c582a…`, sRGB system profile → `*Lab`,
intent 1, `255 255 255` → `99.9988 0.0188 −0.0173`; plus a four-row sRGB
set and a four-intent CMYK set through `USWebCoatedSWOP.icc`.

**Also delivered in Pass 0, beyond the done-when:** the four-crate
workspace with `unsafe_code = "deny"`; Linux+Windows CI from the first
commit (`.github/workflows/ci.yml`); the header/tag-table parser with
malformation reporting and iccMAX refusal; lcms2 licence verification
including the GPL-plugin finding (`LEGAL.md` §4.2); the ICC ToS finding
and the sourcing route taken instead (`LEGAL.md` §2.1–§2.3); 21 corpus
files at `D:\Dev\Rag-Specialized\ICC_Spec\`; and `docs/TOLERANCES.md`
with one provisional anchor.

**What Pass 0 did NOT deliver** — recorded so "Pass 0 is done" is not
read wider than it is:

- **No Rust difftest harness.** Nothing drives `transicc`
  programmatically yet (`tools/difftest/README.md` §10).
- **The POSIX lcms2 build has never been run.** The script exists; this
  machine has no POSIX C toolchain (§7 of the same file). "A script
  exists" is not "the Linux build works."
- **No CI run has been observed by this librarian.** The workflow file is
  present and its content was read; whether GitHub Actions has ever
  executed it is unverified here.
- **Gate results are the engineer's report, not a librarian measurement.**
  `cargo test --workspace` 14/14, `fmt` and `clippy` clean, were run by
  `icc-engineer` on this machine. Independently checkable from the tree:
  **14 `#[test]` declarations** exist, in
  `crates/iccce-profile/src/lib.rs` (8) and `src/num.rs` (6) — which is a
  count of tests declared, not a measure of coverage, and not a pass
  result.
- **No colour maths exists.** `iccce-color` and `iccce-cmm` are stubs;
  Pass 0 produced **no measured colour claim**, which is why
  `docs/NUMERIC_CLAIMS.md` was deliberately not created (see
  `SESSION_LOG.md`, 2026-08-11).
- **The corpus has no `primary_spec` tier.** Every layout citation names
  a corpus file, never an ICC.1 clause number, because no ICC-published
  document was retrieved (`LEGAL.md` §2.2). Consequence, in the
  librarian's words: **a parser is defensible on this evidence and a
  validator is not.**

## Pass 1 — colorimetry (`iccce-color`)

**STATUS: CORE COMPLETE AND VALIDATED — 2026-08-11**, the same working
day as Pass 0. **Not "done"** — the plan text below lists four things
this Pass named that are not built, and they are enumerated in the
completion record rather than quietly dropped. The plan text itself is
unchanged.

No ICC at all. XYZ, xyY, Lab, LCh; standard illuminants and observers;
Bradford and von Kries adaptation; ΔE 76/94/CMC/2000.

**Done when**: every function matches published reference values. This
Pass's tests are the foundation of every later Pass's credibility, so
their expected values must come from the literature, never from the
code.

### Pass 1 completion record — filed 2026-08-11 by `icc-librarian`

**Commit:** `7313c5b` (2026-08-11, "Pass 1: colorimetry — XYZ/xyY,
Lab/LCh, Bradford, ΔE76/ΔE2000"). Hash filled in by `icc-engineer`
immediately after committing, as for Pass 0; the record was filed
against the working tree and lands in the same commit it anchors.

#### The done-when, answered exactly

The done-when reads *"every function matches published reference
values."* Stated plainly, without rounding up:

> **Every *implemented* function meets it wherever a published reference
> value exists.** Exactly one published reference set was in hand for
> this Pass, and the implementation is validated against the whole of it.
> For the rest of the crate **no published reference value was
> obtained**, so those functions are held to arithmetic identities and to
> the provenance of their constants — which is a genuinely weaker
> standard, and the done-when is therefore **met in the only place it
> could be met, and not met in the sense of a reader who assumes the
> whole crate is validated against literature.**

**Where the done-when *is* met, in full:**

- **CIEDE2000 against all 34 pairs of Sharma, Wu & Dalal (2005)**
  (*Color Research & Application* 30(1):21–30, DOI 10.1002/col.20070),
  agreeing within **1×10⁻⁴** — the published data's own precision — at
  `kL = kC = kH = 1`. The full 34, not a sample; the set is adversarial
  by design and cherry-picking defeats it. **This is the first genuinely
  measured numeric claim in the project's history** and it is why
  `docs/NUMERIC_CLAIMS.md` exists as of today. Filed there as
  **NC-001**. *(reported by `icc-engineer` from a `cargo test` run on
  this machine; the assertion, its tolerance and the 34 transcribed
  pairs were read in the live source by this librarian.)*

**Where no published reference value exists, and what stands in its
place** — each of these is a real evidential gap, not a formality:

- **Chromatic adaptation.** **No published worked example of a complete
  adaptation was obtained.** What exists is: the Bradford cone matrix now
  **primary-sourced from ICC.1:2022 Annex E.3**, agreeing exactly with
  the two prior independent code extractions (lcms2 `LamRigg`; CRAN
  `spacesXYZ`) — plus **arithmetic identities only**: `src == dst` gives
  the identity within 1×10⁻¹⁴, source white maps to destination white
  within 1×10⁻¹², D65→D50→D65 round-trips within 1×10⁻¹² on one sample
  vector. **A mis-transcribed digit that preserved the row sums would
  survive every test in the crate.** This is the largest evidential hole
  in Pass 1 and it is recorded as such (`NUMERIC_CLAIMS.md` §3.3, §7).
  Note also that **Annex E is *informative*** — "primary-spec" means the
  digits are printed in the specification, not that the specification
  mandates Bradford (ambiguity **A29**: recommended, not mandated).
- **XYZ / xyY / Lab / LCh.** Round trips, exact endpoints
  (white → `L* = 100`; `Y = 0` → `L* = 0`) and both branches of `f`.
  Arithmetic identities. They detect drift and structural error; they
  cannot detect a consistently wrong constant.
- **Illuminants.** D50 is the corpus's most solidly sourced constant
  (two independent codebases, and its encoded header form was confirmed
  byte-for-byte against a real profile in Pass 0). **D65 is
  single-source** (lcms2 alone; IEC 61966-2-1 paywalled) and is exposed
  as a *chromaticity* so the XYZ derivation stays visible rather than an
  unsourced triple being baked in.

#### Delivered

All in `crates/iccce-color/src/`; every file read by this librarian.

| Module | What |
|---|---|
| `mat3.rs` | 3×3 `f64` matrix: multiply, apply-to-column-vector, **runtime inverse**. The inverse exists at runtime because the corpus marks published Bradford-inverse digits **NOT SOURCED** and directs numerical inversion of the sourced forward matrix. Singularity is `det == 0.0` exactly — not an epsilon, which would be a tuned number with no citation. |
| `illuminant.rs` | D50 as the ICC 4-figure triple (0.9642 / 1.0000 / 0.8249), used **everywhere**, per the corpus's mixing-precision warning; D65 as a chromaticity (0.3127 / 0.3290), labelled single-source at the constant. |
| `xyz.rs` | XYZ ↔ xyY, with divide-by-zero guards **both reference codebases lack** (black has no chromaticity → `None`, not an infinity). |
| `lab.rs` | XYZ ↔ Lab ↔ LCh. Carries **iccce's first named deviation from normative specification text** — see below. |
| `adapt.rs` | von Kries *method* with **Bradford** cones, `M = M_A⁻¹ · D · M_A`. Degenerate whites refused rather than propagated. |
| `delta_e.rs` | ΔE76; CIEDE2000 with explicit `kL/kC/kH` plus a `k = 1` wrapper. lcms2's `180.000001` branch epsilons preserved verbatim and deliberately — they are what Sharma pairs 9–16 test. |

**Named deviation — the `f(t)` breakpoint.** `lab.rs` uses the exact
rational `(24/116)³` / `24/116` form. After the ICC.1:2022 ingest this is
**a stated deviation from the specification's own normative decimal
`0,008 856`**, not merely a pick between disagreeing implementations:
corpus ambiguity **A11** is now resolved-as-delegated — ICC.1 clause 6.4
says conversions *"shall use the equations of the form specified in
ISO 13655"* and does not define `f(t)` itself, while writing the decimal
in its own normative sentence. **ISO 13655 is the authority and is
paywalled and not obtained.** iccce deviates because the rational form
makes `f` and `f⁻¹` exact mutual inverses, which the decimal form cannot
be — ICC's own reference code demonstrates the inconsistency. **Cost:
~10⁻⁷ in `f`, ~10⁻⁵ in `L*` — an analytically derived bound from the
corpus, NOT an iccce measurement**, and it must never be restated as one.
Decision record **DL-010**; register entry `NUMERIC_CLAIMS.md` **NA-001**.

#### Gates — the engineer's report, and what is checkable without a shell

`cargo test --workspace` **35 tests green** (21 `iccce-color` +
14 `iccce-profile`); `cargo fmt` and `cargo clippy` clean.
*(reported by `icc-engineer`, run on this machine.)*

Independently checkable from the tree: **35 `#[test]` declarations
exist** — `mat3.rs` 3, `xyz.rs` 4, `lab.rs` 5, `adapt.rs` 5,
`delta_e.rs` 4, `iccce-profile/src/lib.rs` 8, `num.rs` 6. *(verified —
counted.)* **That is a count of tests declared. It is not a count of
coverage and it is not a pass result.**

#### A finding: a test caught an error in the corpus

The D50-chromaticity consistency test **failed on first run**. Per rule 5
the arithmetic was checked before the code was blamed, and the **corpus**
was wrong: its derived chromaticity (0.34567 / 0.35850) is the
chromaticity of the *high-precision* D50, not of the 4-figure triple the
same file instructs the project to use everywhere — **the
mixing-precision trap that section warns about, committed by the section
that warns about it.** Correct derivation: **0.345703 / 0.358539**. A
parallel dispatch went to `icc-spec-librarian`; **as of this filing the
erratum is still present in the corpus file** *(verified)*. Full record:
`NUMERIC_CLAIMS.md` §3.4.

#### What Pass 1 did NOT deliver — the explicit remainder

**Every item here is blocked on sourcing, not on engineering.** None is a
hard problem; each is waiting for a citable source, and implementing any
of them today would produce a claim this project would have to label as
weaker than it looks.

- **ΔE94 and ΔE CMC(l:c).** Formulas not transcribed from a citable
  source and **no published worked examples obtained** — the
  `icc-spec-librarian` ingest session ran out of budget before
  transcribing them. An implementation now could only be
  **lcms2-cross-checked**, which rule 3 requires labelling as strictly
  weaker than ground truth. Recorded as a gap in `delta_e.rs`'s module
  doc.
- **The von Kries (HPE) cone matrix.** The corpus's digits are a
  placeholder marked **DO NOT USE**. The general *method* is implemented;
  the specific matrix lands when sourced. ("von Kries" names both — that
  ambiguity is why the module doc separates them.)
- **CAT02.** CIE 159 paywalled, not sourced. Not needed for ICC.1.
- **Observer colour-matching-function tables.** **Not needed until
  spectral input exists**, which no Pass currently plans. Listed so the
  Pass-1 plan line "standard illuminants and observers" is not read as
  delivered in full.

Also not delivered, carried forward from Pass 0 and still true: **no Rust
difftest harness** (nothing drives `transicc` programmatically), **no
Linux run of anything**, and **no CI run observed by anyone**.

#### Filed with this Pass

`docs/NUMERIC_CLAIMS.md` — **created**, with NC-001 (the Sharma result)
as its first row, the arithmetic identities classified separately and
weakly, NA-001 (the `f(t)` deviation) in the approximation register, and
an explicit §5 of what Pass 1 does **not** claim.

### ★ Pass 1 addendum — **the remainder's first item is CLOSED: ΔE94 and ΔE CMC exist, and the record says out loud that they are weaker than everything around them.** Filed 2026-08-12 by `icc-librarian`

**The completion record above is unchanged**, including its sentence
*"an implementation now could only be **lcms2-cross-checked**, which
rule 3 requires labelling as strictly weaker than ground truth."* **That
sentence is why this addendum is short and its labelling is long: the
Pass predicted exactly what closing this item would be worth, and
closing it did not make the prediction less true.**

**Commit:** **`aef7566`** *"color: dE94 and CMC — Pass 1's last
remainder, honestly labelled"* *(hash and subject corroborated by
`.git/logs/HEAD`, read; contents unverified)*.

**What landed** *(all verified — `crates/iccce-color/src/delta_e.rs`
read)*:

- **`delta_e_94`** — CIE 1994 with the **graphic-arts** parametric
  factors, transcribed from lcms2 `cmsCIE94DeltaE`. ★ **The textiles
  variant (2:1:1, different `K`) is a different metric and is NOT
  offered rather than guessed** — DL-020's refuse-don't-substitute
  instinct, in the colour crate.
- **`delta_e_cmc(s, t, l, c)`** — transcribed from lcms2 `cmsCMCdeltaE`,
  with both of lcms2's guards carried verbatim and both load-bearing
  (two blacks return **exactly** 0, or `sl` divides by zero; `L* < 16`
  pins `sl = 0,511`).
- **A standing STRENGTH TABLE in the module doc**, not merely in this
  ledger: ΔE2000 **ground truth** (Sharma's 34 pairs) · ΔE76 **exact** ·
  **ΔE94 and ΔE CMC `impl_crosscheck`** · and the instruction *"Grade
  suites in ΔE2000. These exist because some published tolerances are
  stated in them, not because they are as trustworthy."*
- **The expectations came from a C probe compiled against the PINNED
  lcms2**, printing ten decimals — because the oracle is a subprocess
  the unit tests cannot reach. **Agreement was exact to all ten digits on
  three pairs, on the first run**, and the test's own doc comment says
  why that is *weak*: *"it would also pass if both were wrong the same
  way."*
- **★ A test that asserts CMC is ASYMMETRIC** — it weights by the first
  (reference) colour — *"so nobody later 'fixes' it into symmetry"*,
  with ΔE94's symmetry asserted beside it.

**`NUMERIC_CLAIMS.md` §3.20**, rows **NC-153 … NC-156**.

**What the remainder still holds — three items, and all three are still
blocked on sourcing rather than on engineering**: the **von Kries (HPE)
cone matrix** (the corpus's digits are marked **DO NOT USE**), **CAT02**
(CIE 159 paywalled), and **observer colour-matching-function tables**
(not needed until spectral input exists, which no Pass plans).

★ **And the thing this addendum must not be read as doing:** it does
**not** add a ground-truth row. **NC-001 is still the only
`published-ground-truth` row in the project**, it is about a *metric*
rather than a *transform*, and a ten-decimal match against another
implementation is one of the **weakest** kinds of row in this ledger
while looking like one of the strongest.

## Pass 2 — profile parsing (`iccce-profile`)

Header, tag table, and the tag types real profiles use: `XYZType`,
`curveType`, `parametricCurveType`, `textType`/`multiLocalizedUnicode`,
`lut8`/`lut16`/`lutAToB`/`lutBToA`, `namedColor2`, `s15Fixed16Array`.

Report malformations, repair nothing. Identify iccMAX and refuse it by
name.

**Done when**: every profile on the machine parses or is refused with a
reason, and a synthetic corpus covers each tag type.

> **Annotation, 2026-08-11 (`icc-librarian`) — the evidence position for
> Pass 2 has changed, and Pass 0's completion record above is now stale
> on exactly one point.** That record says, quoting DL-002: *"a parser is
> defensible on this evidence and a validator is not."* That was true of
> a corpus built from C headers alone. **The ICC.1:2022 ingest has since
> landed**: corpus files now carry `evidence: primary_spec`, real clause
> numbers, verbatim normative text, tag layouts, and the
> **required/optional tag column** that a C header cannot encode.
> *(verified — `ICC_Spec\index.md` and several `icc__*.md` files read by
> this librarian on 2026-08-11.)* **On that evidence a validator is now
> defensible**, and Pass 2 may plan for one. The Pass 0 record is left
> exactly as written, per this document's own rule; this annotation is
> how it gets corrected. **Still open:** DL-002's clause-citation
> prohibition has **no filed successor entry** in `ARCHITECTURE.md` §5 —
> see **DL-011**, which records that gap rather than closing it.

### Pass 2 progress — batch 1 of 2 landed, 2026-08-11 (`icc-librarian`)

**Status: IN PROGRESS. Batch 1 (the non-LUT tag types) is built; batch 2
(the LUT family) is not.** The plan text above is unchanged.

**Commit:** **`b35a12e`** — *(reported by the dispatching engineer.
`icc-librarian` has no shell, ran no git command, and has not verified
that this commit exists or contains what the dispatch says. What follows
is what was read in the working tree.)*

**Delivered — eight non-LUT tag types decode.** `curv`, `para`, `text`,
`mluc`, `desc`, `ncl2`, `XYZ `, `sf32`, in
`crates/iccce-profile/src/tag_types.rs`, whose module doc names itself
*"Pass 2, batch 1 (the non-LUT types)"* and says *"The LUT family
(`mft1`/`mft2`/`mAB `/`mBA `) is batch 2."* *(verified — the module doc,
the eight `sig::` constants and the eight arms of `decode()` read.)*

**Wired into `inspect`.** The CLI decodes each tag's data, prints a
one-line summary where the type has one, and prints **every
`TagIssue` unconditionally** — the parser's disclosure surface, per
invariant §3.2 (*report, do not repair*). *(verified —
`crates/iccce-cli/src/main.rs`, the decode/summarise/issue loop read.)*

**The report-don't-repair contract is visible in the type design**, which
is the part worth recording: a rule violation that leaves the layout
decodable becomes a `TagIssue` **alongside** the decoded value, while one
that makes the layout unknowable (short data, an `mluc` `recordSize` ≠ 12)
is an `Err` — *"there is no partial result to be tempted by."*
Attacker-controlled counts are bounded against the actual byte length
**before allocation**, the same rule the tag table already used.
*(verified — read.)*

**Verification, and its exact strength.** Reported: verified live on the
system sRGB profile, and `cargo test --workspace` **47 tests green**.
*(reported by `icc-engineer`.)* Independently checkable from the tree
without a shell: **47 `#[test]` declarations exist** — `tag_types.rs` 12,
`iccce-profile/src/lib.rs` 8, `num.rs` 6, `iccce-color` 21 (`mat3` 3,
`xyz` 4, `lab` 5, `adapt` 5, `delta_e` 4). *(verified — counted across 8
files.)* **That is a count of tests declared. It is not a count of
coverage and not a pass result**, and it is recorded only because it is
consistent with the reported figure.

**Pass 2 produced no numeric claim, and that is correct.** Parsing is
exact or it is wrong; `NUMERIC_CLAIMS.md` §2.1 says so explicitly rather
than leaving the absence to be read as an oversight. `TOLERANCES.md` §3.2
(Pass 2) is correspondingly still blank.

**The done-when is NOT met**, and neither half of it: *"every profile on
the machine parses or is refused with a reason"* — not attempted across
the machine's profiles; *"a synthetic corpus covers each tag type"* —
`tools/gen-profiles/` and `fixtures/synthetic/` do not exist. The only
synthetic profiles this project has authored are the four written inside
the difftest probe (`tools/difftest/README.md` §10 notes they should be
ported onto the generator when it exists).

**★ Batch 2 is unblocked by the difftest probe.** The LUT family is where
the PCSLAB encoding selector has to be threaded, and until today that
selector's status was *"the specification says tag type; lcms2 is
believed to say version; nobody has measured it."* It has now been
measured — **DL-012** — so batch 2 can be written against a settled
question rather than an open one:

- **Thread the legacy/general Lab encoding choice with the TAG TYPE**, at
  the point the tag is decoded. Never consult `header.version` for it.
  **DL-011** (the rule, from ICC.1:2022 6.3.4.2 NOTE 3 and 10.10) and
  **DL-012** (lcms2 measured to do the same at the pin).
- **Do not write the runtime divergence warning DL-011 called for.** The
  divergence it was meant to disclose has been **measured absent** for
  `mft2`-in-v4; DL-012 says reconsider it rather than write it.
- **`ncl2` already carries the rule in batch 1's representation** —
  `NamedColorEntry`'s PCS coordinates are held **raw**, with the doc
  comment recording that decoding them is the CMM's job and **must** use
  the legacy tables in a profile of any version. *(verified — read.)*
  Note that `ncl2` was **not** tested behaviourally against lcms2; that
  case rests on a source reading (NC-019's coverage line).
- **DL-005 is untouched:** assert legacy-Lab correctness with
  **exact-value integer invariants, never ΔE**. The error is ≈0.3–0.5 ΔE,
  below the anchor, so a ΔE-graded test passes while the encoding is
  wrong.

### Pass 2 progress — batch 2 landed and the machine-wide sweep run, 2026-08-11 (`icc-librarian`)

**Status: STILL IN PROGRESS, and by a narrower margin than before.
Batch 2 (the LUT family) is built. Done-when clause 1 is met on this
machine. Clause 2 is PARTIAL and needs one scope decision, not more
code.** The plan text above is unchanged.

**Commit:** **`d40d601`** — *(reported by the dispatching engineer.
`icc-librarian` has no shell, ran no git command, and has verified
neither that this commit exists nor that it contains what the dispatch
says. Everything below marked **verified** was read in the working
tree.)*

#### Delivered — the four LUT tag types decode

`crates/iccce-profile/src/lut.rs`, dispatched from `tag_types.rs`
(`sig::MFT1`/`MFT2`/`MAB`/`MBA` → `TagData::Lut8` / `Lut16` /
`LutAToB` / `LutBToA`) and summarised by the CLI. *(verified — the
module doc, the four `decode_*` functions, the four `decode()` arms at
`tag_types.rs:489–492`, and the CLI's `lut8` / `lut16` / `lutAToB` /
`lutBToA` summary arms at `iccce-cli/src/main.rs:229–277` read.)*

**Four design choices worth recording, because each makes a specific
known misread *unrepresentable* rather than merely tested against** —
this is the same "enforced by the type design, not by discipline"
property batch 1 established, applied to the format's most error-prone
structures:

1. **`Lut8` and `Lut16` are distinct structs, not one struct with a
   width flag.** `lut8Type` has **no `inputEnt`/`outputEnt` fields**
   (its tables are always exactly 256 entries), so reading the `mft2`
   layout onto an `mft1` **shifts everything by 4 bytes** — and the
   result still parses. Two types make that impossible. *(verified —
   `Lut8` carries `Vec<u8>` tables and no `*_ent` fields; `Lut16`
   carries `Vec<u16>` and both.)*
2. **One `LutAB` struct serves `mAB ` and `mBA `.** The storage layout is
   identical and **direction-blind**; only the traversal order differs
   (`mAB `: A → CLUT → M → Matrix → B, device→PCS; `mBA ` the reverse),
   and the direction is carried by the **tag's type signature**, kept by
   the caller through the two distinct `TagData` variants. `B` is always
   the PCS-side end in both. *(verified — the doc comment states exactly
   this, and both `TagData::LutAToB` and `TagData::LutBToA` wrap
   `lut::LutAB`.)*
3. **The `mAB `/`mBA ` matrix is a fixed `[S15Fixed16; 12]`.** It is
   3×4 — nine coefficients **then three offset terms** e03/e13/e23, 48
   bytes. Reading 36 and stopping leaves the offsets unapplied, which
   the corpus describes as *"a uniform colour cast that looks like a
   white-point problem"* — the canonical wrong-colour-looks-right shape.
   The fixed-size array makes the 36-byte read unrepresentable, **and
   the offset terms are asserted to arrive**: the test loads
   `m[9] = 9.0`, `m[11] = 11.0` with the comment *"the 36-byte misread
   would have lost them."* *(verified — `lut.rs` `LutAB::matrix`, and
   `tag_types.rs::tests::lut_ab_full_pipeline_with_3x4_matrix_and_per_dim_clut`
   lines 1277–1281.)*
4. **Curve chains fail *positionally*.** `mAB `/`mBA ` curve elements are
   stored back to back with **no count field**, each padded to a 4-byte
   boundary, so curve *n* must be parsed to find curve *n+1* and one
   malformed element makes everything after it **unreachable, not merely
   wrong**. The decoder returns `TagDecodeError::CurveChainBroken {
   element, position }` — naming which element and at what byte — rather
   than a generic short-data error. *(verified —
   `decode_curve_chain`, and the test asserting `element: 1, position:
   44`.)*

**Hostile-input guards, all refusing *before* allocation:** every size is
computed in **`u128`** and checked against the actual byte length
(`clut_nodes_hypercube` is `u128::checked_pow` — `clutPoints` and
`inputChan` are single attacker-controlled bytes and `255^255` must
refuse, not wrap), yielding `LutSizeOverflow` / `LutSizeExceedsTag`; and
a **CLUT `precision` outside {1, 2} is refused** (`ClutBadPrecision`)
because the sample width is otherwise unknowable — there is no partial
result to be tempted by. *(verified — read, and both cases have tests.)*

**The legacy-Lab rule is stated in the module doc as the TAG TYPE rule,
with both citations.** VERBATIM from `lut.rs`: *"a `lut16Type` with Lab
PCS data uses the **legacy 16-bit PCSLAB encoding in a profile of ANY
version** — the selector is the TAG TYPE. `lut8Type` is NOT in the
legacy set (\"and only those tag types\"): it uses the general 8-bit
encoding."* It cites **ICC.1:2022 6.3.4.2 NOTE 3, primary_spec** *and*
**"MEASURED in lcms2 at the pin, 2026-08-11 — tools/difftest"** — i.e.
DL-011's rule and DL-012's measurement, kept as two separate objects
rather than merged. It also says *"the consumer decodes; this module
only repeats the rule so the consumer cannot miss it"*, which keeps
invariant §3.1 (`iccce-profile` performs no colour maths) intact.
*(verified — read.)*

**Sourcing honesty is at the site.** The module doc records that the
`mAB `/`mBA ` **byte tables remain code-derived** — *"the corpus says
so, and so does this line; A23/A24 remain open there"* — while the
clause numbers and CLUT rules are `primary_spec`. That split matches the
corpus file's own split `evidence:` line and is exactly the discipline
**DL-014** now requires of every ICC.1 citation. *(verified — `lut.rs`
§Sourcing and `ICC_Spec\icc\icc__type__lutAtoB_lutBtoA.md`
frontmatter.)*

**Gates.** `cargo test --workspace` **54 green**, `cargo fmt` and
`cargo clippy` clean *(reported by `icc-engineer`)*. Checkable without a
shell: **54 `#[test]` declarations exist** — `tag_types.rs` **19** (12
at batch 1, so **+7**), `iccce-profile/src/lib.rs` 8, `num.rs` 6,
`iccce-color` 21. *(verified — counted across 8 files.)* **A count of
tests declared is not a count of coverage and not a pass result.**

#### ★ Done-when clause 1 — **met on this machine's 40 profiles**

*"Every profile on the machine parses or is refused with a reason."*

A release build of `iccce-cli` was run over every `*.icc` / `*.icm` in
`C:\Windows\System32\spool\drivers\color\`:

| | |
|---|---|
| Profiles | **40** |
| Parse OK | **40** |
| Refused | **0** |
| Unexpected exits (crash, hang, panic) | **0** |
| Table-level malformations | **0** |
| Content findings | **4 profiles, 1 issue each** |

*(**reported** — this is `icc-engineer`'s shell run of 2026-08-11; the
loop counted exit codes and grepped the CLI's own output lines, and the
command is in the session transcript. `icc-librarian` has no shell, ran
no profile, and read no output file. **Nothing in this table was
verified here.**)*

**The four content findings are one finding, four times.** `ewgray18.icm`,
`ewgray22.icm`, `ewrgb18.icm` and `ewsrgb.icm` — EIZO v2 profiles — each
report *"desc: Macintosh ScriptCode block short or missing"*. That is
**exactly the structure the corpus flags as the most frequently
malformed in real v2 profiles**, so the sweep found the thing the corpus
predicted it would find, in the population it predicted. Decoding
continued, the issue was reported, **nothing was repaired** — invariant
§3.2 exercised on real files rather than on fixtures.

**What clause 1 now claims, and its exact boundary.** *"Every profile on
**this** machine, on 2026-08-11, at commit `d40d601`: 40 of 40."* It is
**not** "iccce parses real profiles" and must never be rounded up to it.
Forty profiles from one Windows install is a narrow, systematically
biased corpus: heavy on Microsoft-shipped sRGB/scRGB variants and vendor
display profiles, **light or empty on the population Pass 4 depends
on** — large v4 CMYK press profiles with `mAB `/`mBA ` pipelines, which
are precisely the tag types batch 2 just added. **The sweep exercised
the LUT decoders on whatever this machine happens to contain and nobody
has recorded which of the four LUT types actually appeared in it.**
Installing one profile changes the count; the claim is dated for that
reason.

#### Done-when clause 2 — **PARTIAL**, and the gap is a scope decision

*"A synthetic corpus covers each tag type."*

Stated exactly:

- **Every implemented tag type has synthetic byte fixtures** — the unit
  tests in `tag_types.rs` author tag bytes **by hand, byte by byte**,
  including the hostile cases (`255^255` dimensions, `precision = 3`, a
  broken curve chain at a named position). Those are category (a)
  artefacts in `LEGAL.md` §3's sense: they cannot inherit a bug from the
  code under test, because a human wrote the bytes. *(verified — read.)*
- **The standalone generator and the fixture corpus do not exist.**
  `tools/gen-profiles/` is **absent** from the tree, and
  `fixtures/synthetic/` contains **only its own `README.md`**, which
  says so itself: *"Nothing here yet: the generator does not exist."*
  *(verified — directory enumerated and the README read, 2026-08-11.)*
  `fixtures/reference/` likewise holds only `PROVENANCE.md`.
- **In-test fixtures are tag-level, not profile-level.** They exercise a
  tag's bytes in isolation; they are not whole profiles, so they cannot
  cover header/tag-table/tag-data interaction, cross-tag consistency, or
  anything a consumer would open with `inspect`. The only **whole**
  synthetic profiles this project has ever authored are the **four**
  written inside the difftest probe, and `difftest/README.md` §10 says
  they should be ported onto the generator when it exists.

**★ Whether in-test synthetics satisfy the plan's intent is a real
question and this block does not decide it.** The plan wrote *"a
synthetic corpus covers each tag type"* at a time when
`ARCHITECTURE.md` §1 already listed `tools/gen-profiles/` and
`fixtures/synthetic/` as directories, which is evidence the author meant
**files on disk**, not assertions inside a test module. Against that:
in-test fixtures are byte-authored, versioned, and executed on every
`cargo test`, which is more than a directory of blobs guarantees. **The
two readings differ in what they buy** — a fixture corpus is what a
*differential* run and a *fuzzing* run and an *external* consumer can
use, and in-test bytes are none of those. Recorded as an open decision
for the next session, with a recommendation attached to neither side.

#### What Pass 2 still owes

1. **The clause-2 decision above**, and then either the generator or a
   written statement that in-test synthetics discharge it.
2. **`TOLERANCES.md` §3.2 (Pass 2) rows and §6's coverage table** — both
   `icc-conformance`'s; §6 still reads *"2–8 not started"*.
3. **A behavioural test of `ncl2` and B2A** legacy-Lab decoding, so
   those two cases stop resting on a source reading (NC-019's coverage
   line). Batch 2 has now shipped the B2A-side decoder, so the fixture
   side of that test is cheaper than it was.
4. **Nothing on iccMAX.** The Pass's plan text says *"identify iccMAX and
   refuse it by name"*, and this librarian drafted that as an
   outstanding item before checking — **it is already done, and was done
   in Pass 0.** `Profile::parse` refuses major version ≥ 5 with
   `ParseError::IccMaxRefused { version_raw }`, whose `Display` names
   iccMAX explicitly, and
   `iccce-profile/src/lib.rs::tests::iccmax_is_refused_by_name` asserts
   the message **contains the string `"iccMAX"`** with the comment
   *"'refuse it by name' is the requirement."* *(verified — `lib.rs:94–99,
   215–222` and `diag.rs:41–71` read.)* Recorded as a corrected draft
   rather than deleted, because "an item nobody checked" and "an item
   that is done" look identical in a to-do list. The sweep encountered
   no iccMAX profile, which is evidence about the machine, not about the
   refusal path.

**Pass 2 batch 2 produced no numeric claim, and that remains correct.**
Parsing is exact or it is wrong. `NUMERIC_CLAIMS.md` §2.2 records the
commit and says so explicitly rather than leaving the absence to be read
as an oversight; the sweep is recorded there as a **coverage
observation deliberately not given an NC number**, with the reasoning.

### ★ Pass 2 — done-when clause 2 judged MET; **Pass 2 is DONE**. Filed 2026-08-11 by `icc-librarian`

**Status: DONE.** The plan text, the annotation and both progress blocks
above are unchanged — including the sentence *"clause 2 is PARTIAL"*,
which was true when written. This block is how it is corrected.

**Commits** *(**reported** by the dispatching engineer; `icc-librarian`
has no shell, ran no git command, and has verified neither that these
commits exist nor that they contain what the dispatch says)*: **`7576cfa`**
(`tools/gen-profiles` + the 38-fixture synthetic corpus + GP-001 found),
**`2e98cfd`** (GP-001 fixed + `mAB `/`mBA ` evaluation), **`97ad9fa`**
(the grayTRC F.2 model + the previous filing committed + two code-doc
closures).

#### The question that was left open, and why it is now moot rather than answered

The batch 2 block asked the operator which reading of *"a synthetic
corpus covers each tag type"* the plan meant: **files on disk**, or
**byte-authored fixtures inside the unit tests**. **Nothing in these
documents records an operator answer** *(verified — searched)*, and none
is needed any more, because **the stronger reading is now satisfied**:

| The stronger reading wants | What exists, **verified in the tree 2026-08-11** |
|---|---|
| whole profiles on disk | **38 `.icc` files** in `fixtures/synthetic/`, plus `MANIFEST.md` and `README.md` *(verified — the directory enumerated; 38 `.icc`, not the 39 the previous filing recorded — see the correction below)* |
| covering each tag type | `tools/gen-profiles/README.md` §7 lists, as covered by a **well-formed** fixture: `curv` (all three `count` cases), `para` (funcTypes 0 and 3), `text`, `desc`, `mluc`, `XYZ `, `sf32`, `ncl2`, `mft1`, `mft2` (both directions, v2 and v4), `mAB `, `mBA `. **That is every tag type this Pass's plan text names** *(verified — the plan text and §7 read against each other)* |
| regenerable, not blobs | a standalone crate with `list` / `all` / `verify` / `manifest`, a fixed `FIXTURE_DATE`, **no clock, no environment, no RNG**, and a `verify` that regenerates in memory and **names the first differing byte** *(verified — `tools/gen-profiles/README.md` §§1–2 and the crate's module doc read)*. **28 `#[test]` declarations** exist in the crate *(verified — counted)* |
| usable outside `cargo test` | the fixtures are ordinary files; a differential run, a fuzzer or an external consumer can read them |

**The librarian's judgement, stated as a judgement:** clause 2 is **MET**,
and because clause 1 was met on this machine at `d40d601`, **Pass 2's
done-when is met and Pass 2 is DONE.** The scope question dissolves — it
asked which of two readings the plan meant, and **both are now
satisfied**, so no operator ruling is needed to close the Pass. If the
operator ever answers it, the answer changes nothing.

#### What "Pass 2 DONE" does **not** mean — the boundary, so it cannot be rounded up

- **Clause 1's sweep predates the GP-001 fix and has not been re-run.**
  It was run at `d40d601` against a parser that mis-counted `mBA ` curves
  *(reported; the sweep itself was never verified here)*. No profile on
  this machine exposed the defect, and the fix only *widens* what
  decodes — but *"40 of 40 parse"* is a statement about a superseded
  build, and a re-run is cheap.
- **The fixture corpus is one person's reading of one corpus.** Its own
  README says so: *"38 files authored by one person from one corpus
  reading share whatever that reading got wrong."* GP-001 is the proof
  that this matters in both directions — the reading was right and the
  parser was wrong, and it could as easily have gone the other way.
- **`desc` (`textDescriptionType`) has no ICC.1:2022 clause at all** —
  removed in v4, defined in ICC.1:2001-04, **not obtained**. A `desc`
  fixture is evidence about what implementations do with those bytes,
  **never about the standard** *(verified — `gen-profiles/README.md` §3's
  last row)*.
- **Named gaps remain, and they are named rather than implied**: `para`
  funcTypes 1/2/4, **8-bit (`precision = 1`) `mAB `/`mBA ` CLUTs**,
  multi-record and `count == 0` `mluc`, `ncl2` with `nDeviceCoords == 0`,
  every tag type iccce does not implement, and — deliberately — **any
  CLUT that stresses interpolation** (every documented probe point lands
  on a node; A16 is a Pass 4/5 question wanting its own fixtures).
- **No differential record reads a fixture yet**, so **every Pass 3 and
  Pass 4 record still skips** on a machine without the Windows colour
  directory. That is a CI problem, not a clause-2 problem, and it is
  owed to `icc-conformance`.
- **The fixtures' committed status is reported, not verified.**
  `gen-profiles/README.md` §6 records `git status --porcelain` listing
  all 38 as **untracked** at the time it was written, with
  `git check-ignore -v` showing the `!fixtures/**/*.icc` negation — i.e.
  **trackable**. The dispatch reports them as committed in `7576cfa`.
  **No agent here has run git**, so "committed" travels as a report.

#### ★ A correction to this librarian's own previous filing

The Pass 4 progress block above says *"`fixtures/synthetic/` now holds
**39** `.icc` fixtures"*, and `NUMERIC_CLAIMS.md` §7.5 repeats it.
**The live count is 38** *(verified — the directory enumerated; 38
`.icc`, plus `MANIFEST.md` and `README.md`)*, which is also what
`gen-profiles/README.md` §1 and §6 say (*"38 whole profiles"*, *"verify:
38 identical"*). The most likely origin is counting **directory entries**
rather than `.icc` files at a moment when `MANIFEST.md` had appeared.
**Left standing where it was written**, per this document's rule; this is
the correction. It is the second time in two filings that a **count**
taken from a directory listing has been wrong, and the lesson is the one
already on the record: **a listing is a timestamped observation, and a
count is not an inventory.**

#### What Pass 2 still owes — nothing that blocks the Pass

1. **A re-run of the machine-wide sweep** against a post-GP-001 build,
   with per-tag-type counts. *(The per-type breakdown was already owed.)*
2. **`TOLERANCES.md` §3.2 (Pass 2) and §6's coverage table** —
   `icc-conformance`'s, still not written. §7 of the generator's README
   is the material for them.
3. **A behavioural test of `ncl2` legacy-Lab decoding** — still resting
   on a source reading (NC-019's coverage line).

## Pass 3 — matrix/TRC transforms

The analytic path: RGB→XYZ→RGB through matrices and tone curves, with
adaptation. Covers sRGB, Adobe RGB, Display P3 — most display profiles.

**Done when**: sRGB→AdobeRGB round-trips within a stated ΔE, and matches
lcms2 within a stated tolerance, with both numbers written down.

> **Annotation, 2026-08-11 (`icc-librarian`) — Pass 3 is the next Pass,
> and two things that have been dormant become live the moment it
> starts.** The plan text is unchanged; this records what the Pass
> inherits.
>
> **1. Its done-when produces the ledger's first
> `implementation-cross-check` row.** *"Matches lcms2 within a stated
> tolerance"* requires **iccce on one side of a comparison**, which has
> never happened — `NUMERIC_CLAIMS.md` §5.1 records that **iccce has
> never been compared to anything**, and §3.6's rows are
> `oracle-behaviour-at-pin` with iccce absent. The moment a Pass 3 row
> lands, that sentence stops being true and the ledger gains a class it
> has never carried. **The tolerance must be justified before the run,
> not fitted after it** (rule 5, `TOLERANCES.md` §0), and the
> round-trip half is `self-consistency` — worthless as correctness
> evidence and must be labelled so even when it looks reassuring.
>
> **2. NA-002's cost becomes owed on the first transform that adapts.**
> `NUMERIC_CLAIMS.md` §4 registers **Bradford as a policy choice, not a
> conformance requirement** (corpus **A29**: ICC.1 recommends no
> particular chromatic-adaptation transform), with its **cost
> UNMEASURED** — permitted *"only while the entry is new"*, and it was
> new on 2026-08-11. **sRGB→AdobeRGB adapts**, so Pass 3 is the Pass
> that owes the measurement: Bradford against at least one other CAT,
> over a stated sample set, in ΔE2000, on a stated illuminant pair.
> **Both plausible alternatives are currently unsourceable** — the
> corpus's von Kries/HPE digits are a placeholder marked **DO NOT USE**
> and CIE 159 (CAT02) is paywalled and not obtained (§5). So the honest
> possibilities are: source one, or record in NA-002 that the cost
> cannot be measured yet and say why. **What is not available is
> letting it lapse quietly.**
>
> **3. The sRGB constants are single-source, and Pass 3 is built on
> them.** The corpus's sRGB file rests on **lcms2 alone** — IEC
> 61966-2-1 is paywalled and not obtained — and the same gap makes
> **D65 the weakest constant in `iccce-color`** (NC-018: chromaticity
> from `cmsvirt.c` alone, *not* cross-verified, unlike D50 and
> Bradford). The corpus records **ITU-R BT.709 as free from itu.int and
> NOT FETCHED** as the candidate second source. *(verified —
> `ICC_Spec\index.md` and NC-018 read, 2026-08-11.)* Two consequences:
> a Pass 3 sRGB↔XYZ result that agrees with lcms2 **may be agreeing
> because both took their primaries from the same place**, which is the
> shared-misreading case `TOLERANCES.md` §1 warns about and is the
> weakest possible form of cross-check; and **fetching BT.709 is
> blocked on the same determination DL-007 requires** — ITU's terms must
> be read before an agent fetches, because *"it is a free download"* is
> not *"automated retrieval is permitted."* That is
> `icc-spec-librarian`'s call, not Pass 3's.
>
> **4. Curve evaluation now has a normative home, which it did not when
> this plan was written.** The ICC.1:2022 ingest found **Annex F
> (NORMATIVE) fully specifies curve inversion**, and **10.6 mandates
> linear interpolation for `curveType`** — the corpus's A15/A17 were
> not merely unverified but **wrong**. *(verified —
> `ICC_Spec\index.md`.)* Pass 3's tone curves are therefore a
> **specification-following** job, not a choose-a-reasonable-method job,
> and **DL-014** now permits citing those clauses provided the corpus
> file is named and its `evidence:` line is read first. Note the
> asymmetry the corpus flags as its own finding: **A16, CLUT
> interpolation, is confirmed SILENT** — so Pass 4's interpolation
> remains a named, measured approximation while Pass 3's is not.

### Pass 3 progress — core and CLI landed, done-when numbers pending, 2026-08-11 (`icc-librarian`)

**Status: IN PROGRESS. The matrix/TRC engine and the scriptable
`transform` surface are built. The done-when is NOT met, and cannot be
met by this filing: neither of the two numbers it requires exists yet.**
The plan text above and the annotation above it are unchanged.

**Commits:** **`c4038eb`** (Pass 3 core — `crates/iccce-cmm/src/curve.rs`,
`matrix_trc.rs`) and **`051707f`** (`iccce transform`, plus the
engineer's own agent-memory). *(both **reported** by the dispatching
engineer. `icc-librarian` has no shell, ran no git command, and has
verified neither that these commits exist nor that they contain what the
dispatch says. Everything below marked **verified** was read in the
working tree.)*

#### ★ The done-when, answered exactly — it is NOT met

*"sRGB→AdobeRGB round-trips within a stated ΔE, and matches lcms2 within
a stated tolerance, with both numbers written down."*

**Neither number is written down, because neither has been measured.**
`icc-conformance` was dispatched **in parallel with this filing** to
produce them — the operator's instruction on 2026-08-11 was faster loop
ticks with parallel dispatch on disjoint file sets, which is why two
agents were writing at once. Stated so that twelve new ledger rows are
not mistaken for the done-when:

| The done-when wants | What exists today |
|---|---|
| a **round-trip ΔE** across sRGB→AdobeRGB→sRGB | a round trip through **one** profile, in **device units**, bounded at `1×10⁻³` — `NUMERIC_CLAIMS.md` **NC-032**. Source and destination are the **same** profile, so the matrix and its inverse cancel and it prices only the curve stack. **Not the same quantity** |
| a **stated, justified lcms2 tolerance** | **nothing.** `iccce` has still never been compared to another implementation; the ledger still has **zero** `implementation-cross-check` rows (`NUMERIC_CLAIMS.md` §5.2) |

**Whether the parallel run landed is `unverified` here.** A later
session must look for the rows rather than assume the dispatch
succeeded. **Until they exist, Pass 3 stays IN PROGRESS** — and note
that **Pass 2 is also still in progress**, on one scope decision, so the
Passes are no longer completing in order.

#### Delivered

| Module | What |
|---|---|
| `iccce-cmm/src/curve.rs` | The tone-curve engine. `curveType`'s three cases (identity / gamma / sampled table) and `parametricCurveType`'s five function types; **forward evaluation and inversion**. Sampled-table interpolation is **linear because clause 10.6 says so** (corpus A15, RESOLVED — normative, not a choice); parametric range is **clipped to [0,1] because 10.18 says so** (A19). Inversion follows **Annex F.1, which is NORMATIVE** — both plateau tie-break cases, the F.1(b) out-of-range clamp, and **`constant` kept distinct from `non-monotonic`** exactly as the spec draws them (*"cannot"* be inverted versus the inverse is *"undefined"*). *(verified — module doc, `eval`, `eval_inverse`, `eval_table`, `invert_table`, `eval_parametric`, `invert_parametric` read.)* |
| `iccce-cmm/src/matrix_trc.rs` | The **Annex F.3** computational model. Forward `TRC → M`; inverse `M⁻¹ → clamp → TRC⁻¹`, with **the clamp BEFORE the inverse TRC** per F.8–F.16 and asserted on measured output. **PCSXYZ only**, per F.3 verbatim — a Lab-PCS profile is **refused by name**, tested against the real `USWebCoatedSWOP.icc`. A source→destination transform is source-forward + destination-inverse, which **is** the media-relative colorimetric conversion for D50-referenced colorants. *(verified — read.)* |
| `iccce-cli` `transform` | `--src`/`--dst`, triples on stdin, **6 decimals** on stdout, no banner — *"the interface `tools/difftest` diffs against transicc"*, one decimal more than `transicc`'s four **so the comparison is never limited by iccce's print precision**. `--intent` naming anything but `media-relative` is **refused by name** with exit 1, *"refusing rather than substituting"*. *(verified — `cmd_transform` read.)* Smoke-tested as an sRGB→sRGB identity at sample points *(**reported** — there is no test in the repository asserting it)*. |

**Refusal, not approximation, is the pattern throughout** — and it is
worth naming as a pattern because it recurs four times in one Pass: the
Lab PCS, the three unimplemented parametric inverses, the three
unimplemented intents, and the non-monotonic curve whose inverse the
specification leaves free to be **anything**. In each case iccce reports
by name. A plausible substitute is indistinguishable from a bug, and in
this domain it is also invisible.

#### ★ Two findings from the first test run — rule 5 followed, code questioned first

**1. A real bug, caught by an exact-value test.** `eval_table` paired the
**clamped** segment index with the **unclamped** fraction, so at
`x = 1.0` it returned `t[n−2]` instead of `t[n−1]` — for a fine gamma
table, **`TRC(1.0) ≈ 0.998` instead of 1.0**: a 0.2 % error of exactly
the class this project is organised against. Fixed, with the finding
written at the site.

**What makes it worth a decision-log entry rather than a line here** is
the counterfactual: **the two self-consistency checks in the same Pass
would both have passed with the bug present.** The real-profile round
trip's residual would have been `1/1023 = 9.775×10⁻⁴` against a
`1×10⁻³` gate (~2 % of margin), and the white check's X would have been
off `1.9×10⁻³` against a `1×10⁻²` gate. **The error is exactly one table
spacing, and the round-trip bound was justified as ≈ the table's
spacing** — the same quantity, so it cannot discriminate. Only the
`1×10⁻¹⁵` exact-value assertion at the sample points caught it. Full
record, including that the arithmetic is `icc-librarian`'s
reconstruction and rests on the table having 1024 entries (**reported**,
in a comment): `ARCHITECTURE.md` **DL-016**, ledger rows **NC-025** and
**NC-032**.

**2. A fact about a real file, and a tolerance re-justified rather than
tuned.** The system sRGB profile's colorant **`Z` sums to 0.825089** —
`1.9×10⁻⁴` from ICC's 4-figure D50 `0.8249`, which is **the 1998
author's own white rounding**. The test's first bound (`1×10⁻⁴`, from
`s15Fixed16` quantisation) was *"a claim the file never made"*. The
replacement, `1×10⁻²`, is justified **by the failure mode it
discriminates**: D65-referenced colorants would put `Z` at ≈1.089,
**0.26 away — 26× the bound** — while authoring spread is ≈`2×10⁻⁴`,
**50× inside** it. **It cannot fail on a well-formed profile and cannot
pass a wrong white.** Ledger row **NC-031**. This is the project's
cleanest rule-5 worked example so far, and the fourth in a row where
the first question — *is the code wrong?* — had the answer *no*.

#### ★ Corrections to this Pass's own annotation, made by reading the code

The annotation above (filed at the Pass 2 batch 2 filing) predicted what
Pass 3 would inherit. One prediction is **wrong about what was built**,
and the annotation is left standing as the record of what was expected:

- **NA-002's cost has NOT come due, because Pass 3 does not adapt.** The
  annotation said *"sRGB→AdobeRGB adapts, so Pass 3 is the Pass that
  owes the measurement."* **`iccce-cmm` performs no chromatic adaptation
  at all** — it imports only `Mat3` and `Xyz` from `iccce-color`, never
  touches `adapt.rs`, and never reads `wtpt` or `chad`. Colorants as
  stored in a conformant profile are **already** D50-referenced, so
  chaining forward and inverse needs no CAT. *(verified — imports and
  both conversion functions read.)* **Bradford is still unexercised by
  any transform in this project.** The debt moves to the first Pass that
  adapts — most likely **Pass 4** (absolute intent, and any `chad`
  path). Full note: `NUMERIC_CLAIMS.md` §4, and the new **NA-005**,
  which registers *"colorants used as stored, `chad`/`wtpt` never
  consulted"* as a named assumption with an unmeasured cost.
- **The single-source sRGB/D65 warning stands and is now load-bearing in
  a new way.** Nothing in Pass 3 reads the corpus's sRGB constants — the
  profile supplies its own — but **NC-031's justification quotes D65's
  `Z` from NC-018**, the weakest constant in `iccce-color`. A tolerance
  justified against a single-source constant is only as good as that
  constant. It is 26× clear of the bound, so nothing turns on it here.
- **Curve work being specification-following rather than
  choose-something-reasonable: confirmed, and it changed the shape of
  the Pass.** Annex F.1's tie-break is a rule nobody would have guessed
  — *highest* x normally, *lowest* x when the plateau reaches the domain
  end — and getting it backwards is described in the corpus as *"a
  printer profile with a flat shadow shoulder inverts to the wrong ink
  limit."* Both cases are tested against the verbatim rule (**NC-022**,
  **NC-023**), which is why this Pass produced the ledger's first
  **`normative-rule-conformance`** rows.

#### A named divergence, filed as DL-015

`pow(negative, fractional)` is `NaN`. **lcms2 guards the base; ICC's own
sample code does not** — a real behavioural difference between the two
reference implementations. iccce follows lcms2. **This is NOT a
deviation from normative text** (clause 10.18 declares those parameter
combinations *explicitly undefined* — a stated non-requirement, which is
stronger than silence), and the register now distinguishes that kind of
choice from DL-010's kind. Cost: none on well-formed curves — **analytic
and unmeasured**. `ARCHITECTURE.md` **DL-015**, ledger **NA-004**, which
also records two limits the module doc's own wording does not carry.

#### Gates

`cargo test --workspace` **68 green**, `cargo fmt` and `cargo clippy`
clean *(reported by `icc-engineer`)*. Checkable without a shell: **68
`#[test]` declarations exist** — `curve.rs` **9** and `matrix_trc.rs`
**5** (the 14 new), `tag_types.rs` 19, `iccce-profile/src/lib.rs` 8,
`num.rs` 6, `iccce-color` 21. *(verified — counted across 10 files.)*
**A count of tests declared is not a count of coverage and not a pass
result** — and **two of the fourteen skip silently** when
`C:\Windows\System32\spool\drivers\color\` has no sRGB profile, in which
case "68 green" includes two tests that asserted nothing.

#### The Pass 3 remainder — three items, and only one is engineering

1. **★ ICC-absolute colorimetric intent — BLOCKED ON SOURCING, and it
   is a new named corpus gap.** The media-relative→absolute white-point
   adjustment formula **has not been transcribed into `ICC_Spec`**, the
   module doc records that it **will not be written from memory** (rule
   2), and the intent is refused rather than approximated. This is
   `icc-spec-librarian`'s to close — the clause is expected to be in
   ICC.1:2022 clause 6.x or an Annex, and **"expected to be" is a
   prediction until the document is open.** Everything else about
   absolute intent is downstream of it.
2. **Parametric inverses for function types 1, 2 and 4.** Types 0 and 3
   are analytic and implemented — the shapes real display profiles
   carry. The other three are **refused by name**
   (`InverseUnsupported { func_type }`) because a sampled inverse would
   be an approximation, and an approximation needs a measured cost
   (rule 4). Implementing them is analytic work, not sourcing work.
3. **A policy for perceptual and saturation intent on matrix/TRC
   profiles.** The module doc's expectation is that they are
   media-relative there, *"which is what lcms2 does with them too — but
   THAT equivalence is an unverified expectation here, not a claim; the
   differential test owns it."* *(verified — read.)* **That sentence is
   exactly the right shape** and the difftest must settle it. Note the
   standing hazard from **DL-013**: at perceptual and saturation against
   a **v4** profile, lcms2 is running a transform with **forced BPC** in
   it (≈3.15 `L*` at black), so the comparison is not the one it looks
   like.

**Also not delivered, carried forward and still true:** no comparison
against lcms2 of any kind; **no Linux run of anything**; **no CI run
observed by anyone**; `tools/gen-profiles/` still absent and
`fixtures/synthetic/` still holding only its README *(verified —
enumerated)*.

### ★ Pass 3 completion record — the done-when MET, filed 2026-08-11 by `icc-librarian`

**Status: DONE.** The progress block above is left exactly as written —
it was true when filed, and *"the done-when is NOT met"* becoming *"met"*
four hours later is the record, not an embarrassment to be tidied away.
This block does not edit it.

**Commits since the last filing** *(all six **reported** by the
dispatching engineer. `icc-librarian` has no shell, ran no git command,
and has verified neither that these commits exist nor that they contain
what the dispatch says. Everything marked **verified** below was read in
the **working tree**.)*:

| Commit | What the dispatch attributes to it |
|---|---|
| `55772c6` | the four audit items this librarian filed as owed at the last filing, closed by the engineer |
| `a9618fe` | the last filing itself, committed |
| `fc5ff58` | `iccce-cmm/src/clut.rs` — the n-linear CLUT evaluator, the **A16 named choice** |
| `0843094` | `iccce-cmm/src/pcs_encoding.rs` — the 16-bit PCS encodings, exhaustive round trips, the D1 discriminator |
| `6873df1` | absolute intent per D.6/D.7 + the **sourced** Table 25 intent policy |
| `986dae6` | the Pass 3 differential results (`tools/difftest` §13, `TOLERANCES.md` §3.3) and the `LEGAL.md` §1 dependency mirror |

#### ★ The done-when, answered exactly — it is MET

*"sRGB→AdobeRGB round-trips within a stated ΔE, and matches lcms2 within
a stated tolerance, with both numbers written down."*

| The done-when wants | The number, its class, its tolerance |
|---|---|
| **matches lcms2 within a stated tolerance** | **max 3.4762×10⁻³ ΔE2000** (mean 5.1145×10⁻⁴), against a tolerance of **2×10⁻²**. Class: **`implementation-cross-check`** — the first rows of that class in this ledger's history. `NUMERIC_CLAIMS.md` **NC-036** (max) and **NC-037** (mean) |
| **round-trips within a stated ΔE** | **max 1.8788×10⁻² ΔE2000** (mean 8.674×10⁻⁴), against a tolerance of **2.5×10⁻²**. Class: **`self-consistency`** — both sides are iccce, and it must be labelled so however reassuring it looks. `NUMERIC_CLAIMS.md` **NC-038** |

Both are **sRGB IEC61966-2.1 → Adobe RGB (1998)**, the pair the done-when
names — **no substitution was invoked**, both files being present on this
machine. Media-relative colorimetric, `-c0` (lcms2's most accurate path),
**133 deterministic grid points**, Windows 11 Pro 10.0.26200 / MSVC,
lcms2 2.19.1 at pin `21c582a`. *(verified — the numbers read in
`tools/difftest/README.md` §13.5, §13.8 and §13.9 and cross-read against
`TOLERANCES.md` §3.3.1, which agrees on all seven records. The **run** is
`icc-conformance`'s and is **reported**; this librarian ran nothing.)*

**Five further records were produced by the same run and are not
decoration** — a device-space cross-check at **6.7059×10⁻⁵** against
5×10⁻⁴; two means recorded with an **infinite** tolerance so the
distribution sits next to the max without ever being quoted *as* the max;
the prediction pin below; and an **instrument check** at 8.7945×10⁻⁵
holding iccce's own ΔE ruler against `transicc`'s Lab output, because
records 3–5 grade iccce with a metric built partly out of iccce.
Ledger rows **NC-034 … NC-043**.

#### ★ Why these two numbers are stronger than "a suite went green"

**1. The cross-check tolerance was tested, not asserted.** Its
justification is derived from **lcms2's own arithmetic** — `cmsgamma.c`
quantises a segment-free tone curve's input *and* output to 1/65535, and
the source profile's TRCs are exactly that case. Rather than leave that
as a plausible sentence in a `why` string, `pass3_report` **modelled
lcms2's quantisation inside iccce's model and re-measured**: the
device-space residual collapses from 6.705882×10⁻⁵ to
**2.311449×10⁻⁷, a factor of ~290, and below `transicc`'s own print
floor of 3.92×10⁻⁷**. The disagreement is accounted for essentially
completely by a named approximation **in the oracle**. Ledger
**NC-041**.

**2. The round-trip tolerance is a CORRECTED DERIVATION, not a widened
number, and the distinction is the whole of rule 5.** It was **1×10⁻²
before the run and the run failed at 1.8788×10⁻²**. `TOLERANCES.md` §0's
procedure was followed in order: the code was not wrong (the clamp is
Annex F.8–F.16 doing its job); there was no expectation to be wrong; the
**fixture's premise** was wrong. The original reasoning — *"sRGB ⊂ Adobe
RGB, so nothing is clipped"* — is true of the two **colour spaces** and
false of the two **files**: their encoded media whites (the colorant
sums) differ by **5, 2 and 12 units of `s15Fixed16`'s 1/65536 lsb**,
putting source white at **(1.000106, 0.999873, 1.000254)** in destination
linear space, and **25 of 133 grid points clip somewhere**. The mechanism
was then **predicted in closed form from the two matrices and the clamp
alone** — no tone curve, no lcms2, no measurement: **1.878244×10⁻²**
predicted against **1.878818×10⁻²** observed, **0.03 % agreement**. Both
justifications are preserved in `TOLERANCES.md` §4. Ledger **NC-038**,
**NC-042**.

**3. A seventh check exists specifically to stop the round-trip gate
rewarding a deleted requirement.** Record 5 is an *upper* bound on a
quantity that is mostly a **deliberate cost**: delete iccce's range
clamping and the round trip gets *better*, so the gate would go greener
while a normative requirement had been removed. Record 6 pins
|predicted − observed| at device white to **5.7392×10⁻⁶** against
1×10⁻³, and a **sensitivity control** shows the same metric would read
1.878×10⁻² — **failing by 19×** — with clamping removed. An apparatus
not shown able to detect the effect it is looking for is not an
experiment. Ledger **NC-039**; the method rule that generalises from it
is `ARCHITECTURE.md` **DL-018**.

**★ And its scope is stated honestly rather than rounded up.** Record 6
does **not** make the F.8–F.16 clamp *ordering* falsifiable — iccce
clamps at **three** independent sites (F.8–F.16's linear clamp, 10.18's
domain clamp in `Trc::eval`, F.1(b)'s attainable-range clip in
`invert_table`), so the other two make the first redundant at the shipped
surface. A first draft of the check claimed otherwise; the claim was
**corrected in place rather than deleted**. **Recorded as owed, not as
covered.**

#### A FINDING against lcms2, recorded as a finding (rule 7)

**8 of 399 output components (2.01 %) came back from `transicc` outside
`[0,1]`, up to `1.000120`**, all on grid points whose maximum channel is
1.0; iccce returns exactly `1.000000`. It appears **only on the analytic
inverse path** — measured the same day in the reverse direction, whose
destination inverse is a *tabulated* reverse curve, lcms2 **saturates**.
So it tracks which inversion path lcms2 took and looks like an artefact
rather than a stated position. **Annex F.8–F.16 supports iccce**; clause
6.4 requires per-component clipping on **integer** conversion and
**none** for float32 encodings, which may make lcms2's excursion
conforming and iccce's clamp merely stricter. **The two clauses need
reading together, and until they are this is a recorded difference, not
a verdict.** Ledger **NC-043**.

*A status distinction worth keeping straight:* the dispatch reports that
the question **was** put to `icc-spec-librarian` in a parallel dispatch;
`tools/difftest/README.md` §13.10 item 1 still reads *"Not made: no Agent
tool was available in the session that ran this"* *(verified — read)*.
Both can be true — the README was written in that session. **Whether the
dispatch landed is `unverified` here**, and the answer is not in the
tree.

#### The three Pass 3 remainder items — all closed, and how each was closed

| Remainder item (from the progress block above) | Status, **verified in the live source** |
|---|---|
| **1. ICC-absolute intent — blocked on sourcing** | **CLOSED, sourcing first and code second — the right order.** The corpus gained `icc__s__rendering_intents.md` (`evidence: primary_spec` for all clause text and equations, extracted by 2–3 independent engines) *(verified — frontmatter read)*, and `matrix_trc.rs::convert_with_intent` implements `Intent::Absolute` as the per-component diagonal scale of **D.6/D.7**, `Xa = (Xmw/Xi)·Xr` composed with `Xr' = (Xi/Xmw_dst)·Xa`, `Xi` cancelling to the composite **`mw_src / mw_dst`**. **The gap this librarian filed as new this morning closed the same day.** |
| **2. Parametric inverses for types 1, 2 and 4** | **CLOSED, analytically.** `invert_parametric` now handles **all five** function types; the `InverseUnsupported { func_type }` refusal **no longer exists as a variant** *(verified — the whole function read; §3.7.6's record of that refusal is superseded and a dated note says so in the ledger)*. Type 4's discontinuous-branch gap returns the boundary `d` as *"the F.1(b) posture applied to the gap"*, citing corpus A18 (the spec imposes no continuity at the breakpoint) — a named posture, not a silent guess. **The dispatch did not attribute this to a specific commit**, so it is anchored to the working tree and to the commit set as a whole. |
| **3. A policy for perceptual and saturation on matrix/TRC** | **CLOSED by SOURCING, which is better than the difftest settling it.** The progress block said *"the differential test owns it"*; what actually happened is that **ICC.1:2022 Table 25** was transcribed — the TRC/matrix column reads **"Colorimetric"** for Input and Display classes — so perceptual and saturation are served by the colorimetric model **because the specification says so**, not because lcms2 agrees. A measurement would have shown agreement; it would not have shown *authority*. `Intent::{Perceptual, Saturation}` map to the media-relative path and a test asserts **exact equality** *(verified)*. |

#### ★ Three things that were corrected while filing, by reading rather than transcribing

1. **The absolute-intent DIRECTION is the corrected one, and the
   correction came from the corpus catching the specification.** Clause
   **6.2.3's prose states the source/destination ratio backwards**; the
   equations govern, and the code cites the corpus's spec-defect §12 at
   the site. The direction is pinned by a test asserting the ratio
   **0.7067/0.85 = 0.831412** — *"the corpus's own printed
   intermediates"* — with the backwards reading (1.202773) **asserted
   absent** *(verified — the test read)*. A direction error here is the
   canonical quiet defect: every colour still looks like a colour.
2. **`iccce-cmm/src/lib.rs`'s §Status is stale again, in a new place.**
   The old *"Pass 0 scaffold"* line was fixed (one of the four audit
   items), but the replacement still reads *"media-relative colorimetric
   only; the absolute intent awaits its sourced formula"* on a crate
   whose `matrix_trc.rs` now implements absolute intent, and its module
   list mentions neither `pcs_encoding` nor `lut_transform` *(verified —
   read)*. **Reported, not repaired** — the file is the engineer's.
3. **Absolute intent is implemented in the library and NOT reachable
   through the shipped CLI.** `cmd_transform` still refuses any
   `--intent` but `media-relative`, by name, with exit 1 *(verified —
   read)*. Since `tools/difftest` deliberately drives the **binary**,
   **no differential test can exercise absolute intent until the CLI
   exposes it.** The implementation therefore has unit-test and
   corpus-derived evidence only, and **zero cross-check evidence** — a
   distinction that will be invisible in six weeks unless it is written
   here.

#### Gates, and a count that is not an inventory

`cargo test --workspace` and the differential run are **reported** by the
engineer; this librarian ran neither. Checkable without a shell:
**87 `#[test]` declarations now exist across 13 files** under `crates/`
— `tag_types.rs` 19, `curve.rs` **11**, `matrix_trc.rs` **9**,
`lib.rs` (profile) 8, `num.rs` 6, `clut.rs` **5**, `adapt.rs` 5,
`lab.rs` 5, `delta_e.rs` 4, `xyz.rs` 4, `pcs_encoding.rs` **4**,
`lut_transform.rs` **4**, `mat3.rs` 3 — against 68 at the last filing.
*(verified — counted.)* **A count of tests declared is not a count of
coverage and not a pass result**, and the hazard recorded last filing is
unchanged: **two tests skip silently** when the system profile is absent,
and **every one of the seven differential records skips** on a machine
without the Windows colour directory, the runner then exiting **3
("nothing ran")** rather than 0.

**★ A DISCREPANCY in the run counts, recorded unresolved.**
`tools/difftest/README.md` §13.9's transcript ends `summary pass=8
fail=0 skip=0 error=0` and carries **eight** `check` lines; the
engineer's verification re-run is **reported** as `pass=7 fail=0`.
Structurally, `main.rs::checks()` registers **exactly one** `Check`
(`smoke/srgb-white-to-lab`, the pre-existing oracle-reproducibility
row) and `pass3.rs` emits **seven** records, which is where 1 + 7 = 8
comes from *(verified — both files read)*. So `pass=7` is **consistent
with** the smoke check not passing-and-counting on the re-run — but the
dispatch carried **no per-line output and no skip/error counts**, so
that is a hypothesis and it is written here as one. **What is not
affected:** the seven per-record values, which agree across three
independently written places (README §13.5, §13.9 and `TOLERANCES.md`
§3.3.1). **What is affected:** the re-run cannot be quoted as an
independent re-verification of all eight lines, because nobody recorded
which eight it ran.

#### What "Pass 3 verified" is allowed to mean — the coverage statement, quoted

> iccce's Annex F.3 matrix/TRC model agrees with lcms2 2.19.1 to a
> maximum of 3.476×10⁻³ ΔE2000 (mean 5.114×10⁻⁴) and 6.706×10⁻⁵ in
> normalised device units, over **133 deterministic points**, sRGB →
> Adobe RGB (1998), **media-relative colorimetric**, `-c0`, on Windows 11
> Pro 10.0.26200 / MSVC.

**Everything outside that sentence is not verified**, and the exclusions
are specific: **no v4 profile is exercised at all** (both files are
v2.1); no LUT profile, no CMYK, no grey, no `chad`; **no other intent**
— including the absolute intent this Pass implements; nothing below
1/16 device except exact zero; **no genuinely out-of-gamut input**,
because sRGB ⊂ Adobe RGB makes real clipping impossible in this
direction; one direction, one platform, one lcms2 build. And per rule 7,
agreement with lcms2 is evidence that two implementations read a clause
the same way — **which two implementations can do while both being
wrong**, a risk that is *elevated* here because the corpus's sRGB
constants and D65 both rest on lcms2 alone.

#### Still open — carried honestly, none of it blocking Pass 3

- **The F.8–F.16 clamp ORDERING is owed, not covered** (§13.6.4 above).
  Distinguishing clamp-before from clamp-after needs a TRC whose inverse
  is defined outside `[0,1]`, which iccce never permits. `TOLERANCES.md`
  §3.3.3 carries it as a blank row, correctly.
- **The lcms2 `>1.0` verdict is pending** a specification reading
  (clause 6.4 integer-vs-float32 clipping, read together with Annex
  F.8–F.16). Until then it is a **recorded difference**.
- **NA-002's Bradford cost is still NOT due.** Pass 3 does not adapt,
  and the absolute intent does not change that: D.6/D.7 is a
  **per-component diagonal scale**, not a chromatic-adaptation
  transform, and the code explicitly does **not** un-apply `chad`
  (6.2.1 NOTE 1 / E.4 — it is a provenance record) *(verified — read)*.
  **`iccce_color::adapt` is still not called by any transform in this
  project.** This is the second consecutive filing to check that
  prediction against the code rather than carry it.
- **The largest evidential hole in Pass 3 is named in `TOLERANCES.md`
  §3.3.3 and is worth repeating here: nothing yet compares a matrix/TRC
  transform to a PUBLISHED value.** Every §3.3.1 row is
  implementation-relative or self-referential. IEC 61966-2-1's primaries
  would supply one; the corpus has not been asked.
- **Pass 2 is still IN PROGRESS** on the clause-2 scope decision.
  `tools/gen-profiles/` still does not exist and `fixtures/synthetic/`
  still holds only its README *(verified — enumerated)*, which is also
  why every Pass 3 differential row skips in CI.
- **Nothing has run on Linux and no CI run has ever been observed**, by
  anyone, ever.

#### ★ Pass 4 work is already in the working tree, and the dispatch did not mention it

`crates/iccce-cmm/src/lut_transform.rs` exists and is declared in the
crate's `lib.rs`: *"lut16Type evaluation pipeline — **Pass 4 assembly,
stage 1**"*, evaluating `mft2` as device → input tables → \[3×3 matrix\]
→ CLUT → output tables → PCS decode, citing clause 10.10, applying the
matrix **only** for PCSXYZ input (A21), and decoding Lab through the
**legacy** encoding for this tag type. It carries 4 tests. *(verified —
read.)*

**It is not in the dispatch's commit list, and it was absent from a
`Glob` of `crates/**/*.rs` run earlier in this same filing session.**
Two readings, and this librarian cannot distinguish them without a
shell: another agent is writing in the tree **concurrently with this
filing**, or the earlier enumeration was stale. Either way the
consequence is the same and is recorded rather than smoothed over:
**the tree this record describes was moving while it was being
described**, and **whether `lut_transform.rs` is committed at all is
unknown.** Nothing in this completion record depends on it — Pass 3's
done-when is met by the matrix/TRC path — but the next session must not
read *"Pass 4 needs the v2 lut16 assembly"* as *"none exists"*.

## Pass 4 — LUT transforms and rendering intents

`A2B`/`B2A`, multi-dimensional interpolation, all four intents including
absolute-as-media-relative-plus-white-point. **v2 vs v4 Lab encoding
lives here** and is the Pass's main risk.

**Done when**: CMYK→RGB through a real press profile matches lcms2
within tolerance at every intent, and the v2/v4 cases are separately
covered.

> **Annotation, 2026-08-11 (`icc-librarian`) — the done-when above is now
> known to be underspecified, and the plan text is deliberately left
> unchanged.** A measured finding (`ARCHITECTURE.md` **DL-013**,
> `NUMERIC_CLAIMS.md` **NC-020**) lands directly on the phrase *"at every
> intent"*: **lcms2 forces black point compensation ON for v4 profiles at
> perceptual and saturation**, whether or not `-b` was passed, on the
> authority of an Adobe document rather than ICC.1. Against a v4 profile,
> two of the four intents therefore compare iccce's transform to lcms2's
> transform **plus a BPC stage iccce has no ICC.1 obligation to run**.
>
> **Pass 4 must choose one of two things and say which**, before any
> tolerance at those intents means anything:
>
> 1. run perceptual and saturation **with the forced BPC explicitly
>    accounted for** — reproduced, or subtracted, and stated; or
> 2. take the cross-check at the **colorimetric intents only**, and
>    record that the other two are excluded and why.
>
> **What is not acceptable is comparing at all four and widening a
> tolerance until it passes**: the disagreement being absorbed is
> **≈3.15 `L*` at black**, which is not a tolerance question at all
> (`TOLERANCES.md` §0, and rule 5 — the first question when a test fails
> is whether the code is wrong).
>
> **Also settled since this plan was written, and it removes a risk
> rather than adding one:** the v2/v4 Lab encoding selector is the **tag
> type**, and lcms2 at the pin **agrees** — DL-011 (the rule) and
> **DL-012** (the measurement). The *"live disagreement with lcms2"* that
> DL-011 recorded has been **measured absent**, so Pass 4 implements the
> tag-type selector on the strength of the clause text and does **not**
> owe a runtime divergence warning. `TOLERANCES.md` §3.4's two
> Lab-encoding rows stay **ground truth** regardless: agreement with an
> implementation is exactly the reassurance a shared misreading would also
> produce.

### Pass 4 progress — assembly stages 1–3 built, the A2B differential run, 2026-08-11 (`icc-librarian`)

**Status: IN PROGRESS.** The plan text and the annotation above are
unchanged. This block records what was built, what was measured, and —
at more length than the rest, because it is the part that will be
misquoted — **exactly which clause of the done-when each number does and
does not answer.**

**Commits** *(all **reported** by the dispatching engineer.
`icc-librarian` has no shell, ran no git command, and has verified
neither that these commits exist nor that they contain what the dispatch
says. Everything marked **verified** below was read in the **working
tree**.)*:

| Commit | What the dispatch attributes to it |
|---|---|
| `19a3b17` | the Pass 3 closure filing committed, plus two engineer doc fixes |
| `9aa1bca` | `lut16` device→PCS pipeline — **assembly stage 1** |
| `63874f9` | `transform::Chain` — **stage 2**; CMYK→RGB live end to end |
| `490191b` | the CLI: **N-channel input and all four intents** |
| `b3f4388` | **B2A evaluation — stage 3**, bidirectional, both tag depths |
| `db60e92` | documentation catch-up |
| `d9e0b82` | the Pass 4 A2B differential (`tools/difftest/src/pass4.rs`, `pass4_report.rs`, README §14, `TOLERANCES.md` §3.4) |
| `edcb60e` | untracked in-progress `tools/gen-profiles` that `d9e0b82`'s cwd-relative pathspec swept in — **a process slip**, recorded in `SESSION_LOG.md` rather than smoothed over |

#### What was built — verified in the live source

| Module | What it is |
|---|---|
| `iccce-cmm/src/lut_transform.rs` | **Stages 1 + 3.** Evaluates `mft2` (`lut16Type`) **and `mft1` (`lut8Type`)** in **both** directions — A2B (device→PCS, decode at the end) and B2A (PCS→device, encode at the start), the stored pipeline being *that* direction and **evaluated forward: no inversion exists anywhere in the LUT path**. Pipeline order per 10.10/10.11; tables interpolate linearly (10.6); the 3×3 applies **only** for PCSXYZ input (A21); CLUT is n-linear (**NA-006**). *(verified — module doc and `from_lut16` read.)* |
| ★ `PcsCodec` | The **(tag type × PCS kind)** product as a closed enum: `Lab16Legacy` (6.3.4.2 NOTE 3, the legacy encoding `lut16` mandates), `Lab8` (Tables 12/13 — **corpus A10 resolved**, and `lut8` is explicitly **not** in the legacy set), `Xyz16` (u1Fixed15). **The fourth cell of the product is REFUSED BY NAME**: `lut8` with an XYZ PCS returns `LutModelError::Lut8XyzPcsUnsourced` because *"the 8-bit XYZ form has no verified row"* in the corpus. *(verified — the enum, the error variant and its `Display` read.)* **Refusing an unsourced encoding instead of interpolating a plausible one is rule 2 enforced by the type system**, and it is why the model generalised to `mft1` without anyone having to decide anything at the call site. |
| `iccce-cmm/src/transform.rs` | **Stage 2 — `Chain`.** Source device→PCS chained into destination PCS→device, with the **sourced 8.10.2 a)–d) fallback** (`icc__s__rendering_intents.md` §4). PCS unified through `Lab::to_xyz` at the ICC 4-figure D50. **`D2Bx`/`B2Dx` (`mpet`) is not implemented and the stage proceeds to step 2 — a DEVIATION from the `shall`-order, and the module doc says so in those words**: *"Skipping silently would be the sin; skipping loudly is the recorded state."* *(verified — read.)* |
| `iccce-cli` `transform` | `--intent media-relative\|perceptual\|saturation\|absolute`; the per-line arity is **`chain.input_channels()`**, so CMYK is four floats per line and an unknown intent is still refused by name with exit 2. *(verified — `cmd_transform` read.)* **This is what made the Pass 4 differential possible at all**: the closure filing recorded that absolute intent had *zero* cross-check evidence because the CLI refused it. It no longer does. |

**A real-file regression test came with stage 2, and it is the right
shape.** `swop_perceptual_equals_saturation_shared_tag` builds two
chains at perceptual and saturation, notes that SWOP's `A2B0` and `A2B2`
are **one block of tag data** (the Pass 0 finding, §8.4), and asserts
`assert_eq!` on the converted output — **exact equality, not a
tolerance.** *(verified — read.)* Any difference at all would be an
8.10.2 tag-selection defect, and there is no arithmetic that could
produce a small one.

#### ★ The A2B differential — the numbers, with their classes

`USWebCoatedSWOP.icc` (v2.1.0, `prtr`, CMYK → `Lab `) → the Windows
system sRGB profile (v2.1.0, `mntr`, RGB → XYZ), **341 deterministic
CMYK points**, **all four A2B intents**, `-c0`, lcms2 2.19.1 at pin
`21c582a`. Run by `icc-conformance`; **`icc-librarian` ran nothing** and
read the values in `tools/difftest/README.md` §14 and cross-read the
tolerance derivations there *(verified — read; `TOLERANCES.md` §3.4 is
`icc-conformance`'s and was not edited)*. Ledger rows **NC-044 …
NC-056** in `NUMERIC_CLAIMS.md` **§3.9**.

**Both profiles are v2.1.0, so DL-013's forced-BPC confound is
unreachable — and the run PROVES it rather than asserting it.**
`pass4::analyse` reads **both header version words from the parsed
headers and prints them on every record** *(verified — the
`version_words` field, its population from `header.version.raw`, and the
`version_note` string read)*. Pass 3 escaped that trap by accident and
said so; Pass 4 turned the escape into **a printed quantity**, so a
future substitution of a v4 profile cannot silently reintroduce it.

**Three kinds of number, and the difference between them is the whole
filing:**

| Kind | Records | Tolerance | Observed | What it can claim |
|---|---|---|---|---|
| **Interpolation-free control** — the 16 hypercube corners, every one an **exact CLUT node**, where n-linear and lcms2's geometry must agree identically | `…/pcs-lab-corners-interpolation-free` | **1×10⁻³** | **5.9131×10⁻⁵** (media-relative) · **6.6558×10⁻⁵** (perceptual/saturation) | **The strongest cross-check evidence this project has ever produced.** At a node the interpolation cancels *and* lcms2's quantisation terms vanish rather than accumulate; what is left is `transicc`'s 4-decimal Lab print floor (≈1×10⁻⁴ ΔE00). The two `lut16` pipelines are indistinguishable at the print floor |
| **Arithmetic gate with the method difference switched off** — iccce's pipeline re-run with lcms2's own `Eval4Inputs` geometry substituted | `…/pcs-lab-emulated-geometry` | **2×10⁻²** | **4.5931×10⁻³** (media-relative) · **4.8154×10⁻³** (perceptual/saturation) | **This is where the agreement claim actually lives.** 55× / 326× tighter than the raw comparison. What remains is the oracle's own quantisation (16-bit tables in and out, `u16` at the CLUT stage boundary, s15.16 in `Eval4Inputs`, a 4-decimal print) |
| **Structural gate whose value IS the method envelope** | `…/de2000-vs-lcms2`, `…/pcs-lab-vs-lcms2`, `…/device-vs-lcms2` | **2.0 ΔE00** / **2×10⁻²** device | ΔE00 **1.6590** perceptual · **0.252 94** media-relative; PCS 1.5715 / 0.254 65; device **1.0816×10⁻²** / **3.0045×10⁻³** | **Explicitly unable to claim agreement.** It can catch a wrong index order, a wrong Lab decode, a swapped ink; it cannot say the two implementations agree, because its value *is* a difference nobody has to explain away |

**★ The 6× fact, which is the argument for having run all four intents.**
The interpolation-method envelope — computed from the CLUT and the two
algorithms alone, **with no lcms2 output in it** — is **1.5741 max on the
`A2B0` (perceptual/saturation) table** and **0.254 23 on `A2B1`
(media-relative)**. The perceptual table's worst cell is deep shadow at
near-full black, where the CLUT turns sharply. **A Pass 4 tolerance
derived from the colorimetric intent alone would have been wrong by a
factor of six for exactly the intents Pass 3 never exercised.** The two
tables are not equally smooth, and nothing about a smooth colorimetric
result predicts a rough perceptual one.

**The apparatus was graded before anything was concluded from it.** The
harness's n-linear arm is held against `iccce_cmm::lut_transform::Lut16Model`
on every grid point at every intent, tolerance 10⁻⁹ in `L*`/`a*`/`b*`:
**observed 0.0 exactly, bit-identical.** Without that row the whole
substitution experiment would be an assertion that the reimplementation
is faithful. Ledger **NC-051**.

**Perceptual ≡ saturation, graded at exactly 0.0 and observed 0.0.**
Ledger **NC-052**.

#### ★ NA-006 is MEASURED — and the advance prediction of its mechanism was WRONG

**The cost of the A16 n-linear choice has been measured for the first
time.** From the Pass 3 closure filing to this one, NA-006's cost was a
**corpus-derived bound** — *"up to ~1 ΔE, transcribed, iccce has NOT
measured it"* — and three documents said the measurement was blocked on
sourcing lcms2's tetrahedral decomposition. It is now a **measured
self-consistency quantity**: **1.5741 ΔE2000 max on the perceptual
table, 0.254 23 on the colorimetric one**, propagating to **1.6639**
ΔE00 end to end. The corpus's *"~1 ΔE"* was the right order of magnitude
and **an underestimate on one of the two tables**.

**And the mechanism nobody measured was wrong.** `NUMERIC_CLAIMS.md`
NA-006, `NEXT_SESSION.md` and this ROADMAP all carried *"iccce
interpolates n-linear, lcms2 tetrahedral"*, and the Pass 4 blocker was
recorded as *"source lcms2's tetrahedral cube decomposition."* Rather
than recall it, `icc-conformance` **read `cmsintrp.c` at the pin**, and
for **four** inputs lcms2 does not run tetrahedral at all: it runs a
**hybrid** — *linear* along input channel 0 (C), **Sakamoto tetrahedral**
in the remaining three (M, Y, K), the two 3-D results blended by the
first channel's fraction. Consequences, none of which *"tetrahedral"*
would have implied:

- **It is not symmetric in the four inks.** Reordering the channels
  changes lcms2's answer. iccce's quadrilinear **is** symmetric.
- **It is not pure tetrahedral**, so a bound transcribed from the
  trilinear-vs-tetrahedral literature is **not the bound that applies** —
  which is precisely what NA-006's ~1 ΔE was.
- **The float path does not use the float interpolator.** An `mft2` tag
  is read into a **16-bit** CLUT stage, whose float evaluator quantises
  the stage input to `u16` and calls the fixed-point twin. lcms2's CMYK
  pipeline in `transicc`'s default float mode therefore carries 16-bit
  quantisation **at the CLUT boundary as well as** inside the tone
  curves.

**This is the second time in this project that a predicted disagreement
with lcms2 was settled by measuring instead of assuming, and the second
time the prediction was wrong in a way that mattered** (the first:
DL-011 predicted a live disagreement over the legacy-Lab selector and
DL-012 measured it **absent**). The prediction is left standing wherever
it was written; the ledger's **NC-056** and NA-006's dated status note
are how it is corrected.

#### ★ A FINDING against lcms2 at the absolute intent — 11.217 ΔE2000, cause identified, authority absent

At `-t3`, iccce and lcms2 differ by **max 11.217 ΔE2000, mean 4.670**
(device max 0.1580) — **two orders of magnitude more than at any other
intent**, and far beyond anything the interpolation envelope for the
table absolute uses (0.2542) could account for. The worst points are the
**lightest**: paper at 10.6, 33 % C at 11.2.

**The mechanism was read at the pin and then measured.** `cmsio1.c`'s
`_cmsReadMediaWhitePoint` **substitutes D50 for the stored `wtpt`** when
a profile is **v2 and display-class**. The destination sRGB profile's
`wtpt` holds **D65** (0.950 455, 1.0, 1.089 050) while its colorants are
D50-adapted — a common v2-era encoding. So the two implementations
differ **not in the formula** (both build the D.6/D.7 diagonal) but in
**what they read for the destination white**: iccce uses `wtpt` **as
stored** (**NA-007**), lcms2 uses **D50**. The ratio is D65/D50 =
(0.9858, 1.0, 1.3202) — **a 32 % error in `Z`, applied to every colour.**
Modelling that one substitution (together with the CLUT geometry, so
both known differences are accounted for) **collapses the disagreement
517×, to 2.1677×10⁻²**.

**Which implementation is right is NOT settled, and that is the
finding.** ICC.1:2022 specifies v4; what a **v2** profile's `wtpt` means
is corpus **A4b**, and **A4b is UNVERIFIED** *(verified — the corpus's
ambiguity register and `icc__s__rendering_intents.md` §A4b read
2026-08-11: ICC.1:2022 is silent on version 2's convention, confirmed by
full-text search, and ICC.1:2001-04 has not been obtained)*. lcms2's
substitution is justified **in its source by a comment, not by a
clause**. A dispatch to `icc-spec-librarian` **is reported to have gone
out in parallel with this filing**, carrying A4b and the two corpus rows
M4/M5; **whether it has landed is `unverified` here** — as of this
filing the corpus carries **M1, M2, M3 and no M4 or M5** *(verified —
`icc__ref__lcms2_measured_behaviour.md` enumerated)*.

**How the numbers handle it meanwhile, and this is the part worth
copying.** The two raw absolute-intent comparisons are **REPORTED, NOT
GRADED** (tolerance ∞) and the **gate at that intent is the
white-point-policy record** at 5×10⁻². **Both alternatives were
considered and rejected in writing** rather than one being chosen
silently: widening to ~15 ΔE00 would be a number chosen because it
passed, and would silently absorb any future arithmetic error in the
absolute path; letting it fail permanently produces a red line that
stops being read and reports the disagreement as unexplained when it is
not. **This is the only place in the suite where a known disagreement is
deliberately not gated**, and the method rule that generalises from it
is `ARCHITECTURE.md` **DL-019**.

#### ★ The done-when, answered exactly — it is NOT met

*"CMYK→RGB through a real press profile matches lcms2 within tolerance
at every intent, and the v2/v4 cases are separately covered."*

| Clause | Status |
|---|---|
| *CMYK→RGB through a real press profile* | **Met.** `USWebCoatedSWOP.icc` → system sRGB, through the shipped binary on both sides, 341 points |
| *matches lcms2 within tolerance* — **A2B, the three non-absolute intents** | **Met, on stated terms.** The claim lives in the **corner** (1×10⁻³ gate, ≈6×10⁻⁵ observed) and **emulated-geometry** (2×10⁻² gate, ≈4.8×10⁻³ observed) records. The **raw** ΔE00 records are 2.0-gated and **cannot claim agreement** — the record's own text says so |
| *…at every intent* — **the absolute intent** | **NOT met, and deliberately so.** The raw comparison is **reported, not graded**, pending **A4b**. What is graded there is the *modelled* comparison at 5×10⁻². **A2B0 and A2B2 being one block of tag data** in this file also means perceptual and saturation are the **same bytes through the same code** — genuine four-intent coverage of *distinct tables* is three, not four, on this pair |
| *…at every intent* — **the B2A direction** | **★ NOT met. ZERO measurements exist.** `b3f4388` landed bidirectional evaluation and `transform::Chain` grew a `Lut16B2a` destination model *(verified — `DestModel::Lut16B2a` read)*, but **this run's destination is matrix/TRC**: sRGB has **no `B2A*` tag at all**. Nothing in the repository has compared a B2A evaluation to anything. §14.8's coverage statement says so in its own words, and **"Pass 4 verified" does not include B2A** |
| *the v2/v4 cases are separately covered* | **PARTIAL.** See the next block |

**The v2/v4 Lab-encoding coverage, stated exactly** — because *"v2/v4
separately covered"* is the Pass's stated main risk and it is the
sentence most likely to be rounded up:

- **The v2 side is exercised on real files.** Both profiles in this run
  are v2.1.0 and the source's `A2B*` are `mft2`, so the **legacy 16-bit
  PCSLAB** path ran on every one of 341 points at every intent, against
  the oracle, and agreed at the corners to 6×10⁻⁵.
- **The v4-`mft2` side was measured by the probe, not by this run.**
  NC-019 (`oracle-behaviour-at-pin`, `bfd6b1e`) measured lcms2 keying the
  legacy encoding off the **tag type** in a **v4** profile; `pcs_encoding.rs`
  implements both encodings with exact-value invariants per **DL-005**.
  **No v4 profile appears in this differential at all.**
- **`mAB `/`mBA ` are DECODED and NOT EVALUATED, and the dispatch got
  this half-wrong.** The dispatch describes them as
  *"undecoded-unevaluated"*. **They have been decoded since Pass 2 batch
  2**: `tag_types.rs` dispatches `sig::MAB`/`sig::MBA` to
  `lut::decode_lut_ab`, producing `TagData::LutAToB` / `TagData::LutBToA`
  *(verified — read)*. What does not exist is an **evaluator** in
  `iccce-cmm`: `lut_transform.rs`'s own scope note says *"Still absent:
  `mAB `/`mBA ` evaluation"*, and `transform::ChainError::SourceTagUnsupported`
  exists precisely to name that case rather than fail generically
  *(verified)*. **That is stage 4, and it is what the v4 half of the
  done-when actually needs.**

#### Reported, not repaired — three prose defects, all in the engineer's files

1. **`iccce-cmm/src/lib.rs`'s §Status is stale AGAIN, for the third
   consecutive filing.** The absolute-intent sentence the last two
   filings reported was fixed; the replacement now reads *"(CMYK→RGB
   live; **B2A/lut8/mAB stages pending**)"* on a crate where `b3f4388`
   landed **B2A and lut8** — `lut_transform.rs`'s own module doc is
   headed *"stages 1+3"* and evaluates both depths in both directions.
   **Only `mAB `/`mBA ` is pending.** *(verified — both files read.)*
   Three filings running is no longer an observation about one file; it
   is evidence that **a status line in a doc comment goes stale at
   exactly the rate the crate moves.**
2. **`cmd_transform`'s doc comment contradicts its own code.** It reads
   *"Only media-relative colorimetric exists (Pass 3 scope); an
   `--intent` flag naming anything else is refused by name"* — directly
   above a `match` that accepts `perceptual`, `saturation` and
   `absolute`. *(verified — `crates/iccce-cli/src/main.rs`, the doc
   comment and the match arms read.)* This one is worse than a stale
   status line: **a reader who trusts it concludes that no differential
   can reach the absolute intent**, which was true this morning and is
   the reason the 11 ΔE finding exists.
3. **`clut.rs`'s *"per rule 4 (named and measured)"*** — reported as
   owed at the last filing — **is now true rather than aspirational**,
   because NA-006's cost has been measured. Recorded so the item is
   closed by fact rather than left on a list.

#### Gates, and a count that is still not an inventory

`summary pass=36 fail=0 skip=3 error=0` for the whole suite *(reported
by the dispatching engineer; `icc-librarian` ran nothing)*. **★ The
decomposition in `tools/difftest/README.md` §14.7 — *"8 Pass 3 records,
1 smoke, 27 graded Pass 4 records"* and *"adds 30 Pass 4 records"* — is
wrong in both terms while its total is right.** Counting the record
emitters in the live source gives **1 smoke + 7 Pass 3 + 28 graded
Pass 4 = 36**, with **31** Pass 4 records emitted and **3** skipped at
the absolute intent *(verified — `pass3.rs`'s seven distinct ids, pinned
by their own unit test, and `pass4.rs`'s emitter loop read in full)*.
**A sum that comes out right is not evidence that its terms are right.**
**Reported, not repaired** — §14 is `icc-conformance`'s file. Full
arithmetic in `NUMERIC_CLAIMS.md` **§3.9.8**, which also **confirms
§2.4's structural hypothesis** about the old `pass=8` / `pass=7`
discrepancy. Checkable without a shell: **89 `#[test]` declarations exist
across 14 files under `crates/`** — `tag_types.rs` 19, `curve.rs` 11,
`matrix_trc.rs` 9, `lib.rs` (profile) 8, `num.rs` 6, `adapt.rs` 5,
`clut.rs` 5, `lab.rs` 5, `delta_e.rs` 4, `lut_transform.rs` 4,
`pcs_encoding.rs` 4, `xyz.rs` 4, `mat3.rs` 3, `transform.rs` 2 —
against 87 at the last filing. A further **52 exist under `tools/`**,
of which **28 are in `tools/gen-profiles`**, which did not exist at the
last filing. *(verified — counted.)* **A count of tests declared is not
a count of coverage and not a pass result**, and the standing hazards
are unchanged: tests that read `C:\Windows\System32\spool\drivers\color\`
**skip silently** when it is absent, and **every Pass 3 and Pass 4
differential record skips** on such a machine.

#### ★ Something appeared in the tree that changes a carried claim

**`fixtures/synthetic/` now holds 39 `.icc` fixtures**, and
`tools/gen-profiles/` is a working crate with `list` / `all` / `verify`
/ `manifest` subcommands, a fixed `FIXTURE_DATE`, and 28 tests
*(verified — the directory enumerated and `main.rs`'s module doc read,
2026-08-11)*. Four filings have carried *"`tools/gen-profiles/` does not
exist and `fixtures/synthetic/` holds only its README"*, and **that
sentence is now false.** The fixtures include `v2-cmyk-mft2-lab.icc`,
`v2-cmyk-mft1-lab.icc`, **`v4-cmyk-mab-lab.icc`** and
`v4-rgb-mft2-lab.icc` — i.e. the population Pass 4's remaining work
needs and this machine's colour directory does not contain.

**What that does NOT establish**, stated because a directory listing is
the weakest kind of evidence: nobody has run `gen-profiles verify`
here, no differential record reads any of these files yet, **Pass 2's
clause-2 scope decision is not thereby answered** (the operator was
asked whether in-test synthetics discharge it, and a generator
appearing does not answer a question about intent), and the dispatch
reports this crate as **an agent's work in flight**, which is also how
`edcb60e` came to sweep an untracked working state into a commit. The
tree was moving while this block was written — **again** — and it is
recorded rather than absorbed.

#### What Pass 4 still owes

1. **★ B2A measurement.** The code exists; nothing has measured it.
   SWOP's `B2A*` are `mft1`, so this exercises `lut8Type` evaluation and
   the `Lab8` codec — **neither of which any comparison has touched.**
   This is where *"at every intent"* is actually completed.
2. **★ Stage 4 — `mAB `/`mBA ` evaluation.** Decoded since Pass 2 batch
   2, evaluated by nothing. It is what the **v4** half of the done-when
   needs, and `v4-cmyk-mab-lab.icc` now exists to point it at.
3. **A4b**, which decides whether iccce or lcms2 acquires a defect at
   the absolute intent. Until then neither implementation's
   absolute-intent output can be called right (**DL-019**).
4. **A ground-truth row. Pass 4 has none at all** — every record is a
   cross-check or a self-consistency check. The tractable candidate is a
   **synthetic `mft2` whose CLUT stores an affine function**, where
   *every* interpolation scheme must agree exactly and the expectation is
   therefore arithmetic rather than an oracle's opinion. `gen-profiles`
   now exists to author it.
5. **An instrument check for the sRGB destination model.** Pass 3's
   record 7 bounds iccce's ΔE ruler on **Adobe RGB**; Pass 4 **inherited
   that bound rather than re-measuring it on the profile it used.**
6. **Corpus rows M4 and M5** for the two lcms2 behaviours read here (the
   4-D hybrid; the v2-display `wtpt` substitution) — `icc-spec-librarian`'s
   file, and **not present as of this filing** *(verified)*.

### ★ Pass 4 progress — the EVALUATION SURFACE completed: stage 4 (`mAB `/`mBA `), grayTRC F.2, and the GP-001 arc. Filed 2026-08-11 by `icc-librarian`

**Status: STILL IN PROGRESS, and the done-when is still NOT met.** The
plan text, the annotation and the previous progress block are unchanged.
What changed is that **iccce can now evaluate every LUT tag type in both
directions, plus monochrome** — and that a **parser bug affecting every
real CMYK `B2A0`** was found, by the fixture corpus, and fixed.

**Commits** *(all **reported** by the dispatching engineer.
`icc-librarian` has no shell, ran no git command, and has verified
neither that these commits exist nor that they contain what the dispatch
says. Everything marked **verified** below was read in the **working
tree**.)*:

| Commit | What the dispatch attributes to it |
|---|---|
| **`7576cfa`** | `tools/gen-profiles` + the **38**-fixture synthetic corpus + **GP-001 found** |
| **`2e98cfd`** | **GP-001 FIXED** + `mAB `/`mBA ` evaluation (stage 4) + the transicc cross-check on the committed fixture |
| **`97ad9fa`** | the **grayTRC F.2** model + the previous filing committed + **two code-doc closures** |

#### What was built — verified in the live source

| Module | What it is |
|---|---|
| `iccce-cmm/src/lut_ab.rs` | **Stage 4.** `mAB ` (A → CLUT → M → Matrix → B, device→PCS) and `mBA ` (the reverse) compiled to an evaluable pipeline, absent elements simply not in it. **PCS side is the v4 encodings, not the legacy ones** — the module doc gives the reason at the site (6.3.4.2 NOTE 3's *"and only those tag types"*) and notes the exactness that follows (`L* = n × 100`, `a*/b* = n × 255 − 128`). The **3×4 matrix applies all twelve terms**, offsets included, with a test asserting their effect on measured output. A `Direction` field fixed at build **makes calling the wrong method a `None`, not a wrong number**. *(verified — module doc, `Direction`, `LutAbModel`, the four tests read.)* |
| `iccce-cmm/src/gray_trc.rs` | **The F.2 computational model**, both directions. Forward is `t = kTRC[device]` then **multiplication by the FULL PCS white triple** — the corpus's named trap honoured at the site: *"using the scalar directly as `X` or `Z` is wrong by the D50 chromaticity — a monochrome profile renders with a green cast."* Inverse recovers the connection scalar from the **achromatic channel** (`Y/Yn`, or `L*/100` for a Lab-PCS gray profile), clamps to `[0,1]`, and inverts per **F.1**. *(verified — read.)* |
| `iccce-cmm/src/transform.rs` | Both new models are wired into `Chain` **on both sides**: `TagData::LutBToA` → `DestModel::LutAb` and `TagData::LutAToB` → `SourceModel::LutAb` inside the existing 8.10.2 fallback loops; **grayTRC is step 4's second shape**, tried when `MatrixTrc::from_profile` fails, with the comment recording that *"clause 8's per-class requirements decide which tags exist."* *(verified — the destination loop at 150–211 and the source loop at 213–297 read.)* Consequence worth stating: **a B2A differential is now reachable through the shipped binary**, which is the position the absolute intent was in before `490191b`. |
| `iccce-profile/src/lut.rs` | **The GP-001 fix** — curve counts are now **per tag type**, with the two clause triples in the comment at the site and the reasoning for why the two readings coincide on square LUTs. *(verified — `decode_lut_ab`, lines 322–345 read.)* |

#### ★★ GP-001 — the full arc, and why it is the day's richest record

**The order of events is the point.** The `mAB `/`mBA ` evaluator was
written **mAB-only**, and it **refused `mBA ` on a curve-count
contradiction found during design** — the author could not make the
corpus's one-sentence rule consistent with the tag's own geometry, and
declined to guess. **An hour later the fixture corpus's first run against
the shipped binary found the bug**, on exactly that doubt. The refusal is
still recorded in the code as a HISTORY NOTE: *"The refusal was
vindicated within the hour — GP-001: the guessed counts WOULD have been
wrong."* *(verified — `lut_ab.rs`'s `LutAbModel` doc comment read.)*

**The defect.** `decode_lut_ab` used the `mAB ` convention for **both**
tag types — B and M counted by `outputChan`, A by `inputChan`. For a CMYK
`B2A0` (`inputChan = 3` Lab, `outputChan = 4` CMYK) that expects **four**
B curves where the specification puts **three**, so the decoder walked
past the third into the matrix element and reported
`curve chain broken at element 3 (byte 68)`.

**Why the specification says otherwise, per type** — `icc-conformance`'s
**direct reads of `_sources/ICC.1-2022-05.pdf`**, quoted in
`tools/gen-profiles/README.md` §5 *(verified — read there; **this
librarian has not opened the PDF**, so these are conformance's reads,
carried with attribution)*:

- **10.12.2 / 10.12.4 / 10.12.6** (`mAB `): A = **input**, M = **output**,
  B = **output**.
- **10.13.2 / 10.13.4 / 10.13.6** (`mBA `): B = **input**, M = **input**,
  A = **output**.

**The rule is not *"A goes with input"*.** It is: **the curve set at the
data's entry side is counted by `inputChan`, and the set at its exit side
by `outputChan`** — and which letter that is depends on which direction
the tag runs. It is the same fact as *"B is always the PCS-side end"*,
counted.

**Four properties of this bug that are worth more than the bug:**

1. **It was invisible on every square LUT.** The two readings agree
   whenever `inputChan == outputChan`, which is every RGB→RGB and
   Lab→Lab pipeline anyone would reach for while testing.
2. **It affected every real CMYK `B2A0`** — i.e. the tag a press profile
   uses to *print*, and the exact population the machine-wide sweep was
   recorded as being **light or empty on**. The Pass 2 clause-1 record
   predicted its own blind spot in those words; the fixture corpus is
   that population, and it found what the sweep structurally could not.
3. **The parser's disclosure surface is what made it findable.** The
   symptom was a **named refusal at a byte position** — not a wrong
   colour. Invariant §3.2 (*report, do not repair*) and the positional
   `CurveChainBroken` error, both filed at Pass 2 batch 2 as design
   choices, are the reason this arrived as a diagnosis instead of as a
   silent 11-ΔE-class defect. **A repairing parser would have resynced
   on the next plausible curve header and produced colour.**
4. **The corpus is where it came from, and the corpus is still wrong.**
   `icc__type__lutAtoB_lutBtoA.md` carries **one blanket sentence for
   both types** — *"`A` curves = `inputChan`; `B` and `M` curves =
   `outputChan`"* — with its byte tables at `icc_secondary_code` and
   **A23 open**. That sentence is right for `mAB ` and wrong for `mBA `.
   **It is still there as of this filing** *(verified — line 108 of the
   corpus file read 2026-08-11)*, and closing it is
   `icc-spec-librarian`'s.

**The cross-check, labelled as one.** lcms2 2.19.1 at the pin reads B and
M with `inputChan` and A with `outputChan` in `Type_LUTB2A_Read`
(`Type_LUTA2B_Read` the other way) — *reported* in the generator's README
from `icc-conformance`'s source read — and `transicc` converts
**Lab(50, 0, 0) → CMYK(0, 0, 0, 49.6117 %)** through this fixture's
`B2A0`. **Corroboration that two readers of the standard read it the same
way, which is weaker than the clause text and is not what settles it.**

**And it is now a regression test with a number attached.** `lut_ab.rs`'s
`mba_fixture_matches_transicc_recorded_value` parses the **committed**
fixture, evaluates its `B2A0`, and asserts `K` within **1×10⁻³** of
**0.496117**, with the tolerance justified **in the test** (transicc's
4-decimal percent print, its u16 quantisation, and the ragged-grid
interpolation difference — while still refusing a wrong curve count,
*"GP-001's symptom was a REFUSAL, and a swapped count shifts K by whole
percent"*). *(verified — read.)* Ledger row **NC-057**.

#### The done-when, re-answered — the two clauses that moved, and the three that did not

| Clause | Status now |
|---|---|
| *CMYK→RGB through a real press profile* | **Met**, unchanged (`d9e0b82`'s run) |
| *matches lcms2… A2B, non-absolute intents* | **Met on stated terms**, unchanged — the corner and emulated-geometry rows |
| *…at every intent — the absolute intent* | **Still NOT met, deliberately** (DL-019). **A4b is still UNVERIFIED** *(verified — the corpus's ambiguity register read 2026-08-11)* |
| *…at every intent — **the B2A direction*** | **★ Still NOT met, and the change is small but real.** There is now **exactly one B2A number in the project**: NC-057's single point through a **synthetic** fixture's `mBA `. **No differential exists**; `lut8Type` evaluation and the **`Lab8` codec** have still **never been compared to anything**; SWOP's `B2A*` (`mft1`) have still never been evaluated against the oracle |
| *the v2/v4 cases are separately covered* | **★ Moved from PARTIAL toward met, and still not met.** `mAB `/`mBA ` are now **decoded AND evaluated**, and a **v4** fixture (`v4-cmyk-mab-lab.icc`) exercises both — but **only through unit tests and one recorded transicc value**. **No v4 profile appears in any differential run** |

#### What the evaluation surface now covers — and the three holes in the same sentence

**Covered by code, with unit-level evidence:** `mft1` and `mft2` in both
directions; `mAB ` and `mBA `; grayTRC F.2 in both directions;
matrix/TRC at four intents; the 8.10.2 fallback on both sides, including
grayTRC as step 4's second shape.

**NOT covered by any measurement, stated because "the surface is
complete" is the sentence most likely to be over-read:**

1. **B2A differentials — none.** One cross-check *point* (NC-057), on a
   synthetic fixture, in a unit test.
2. **`mAB ` against any real file — none.** Every `mAB `/`mBA ` number
   in this project comes from bytes this project authored.
3. **Gray against lcms2 — none.** The gray evidence is one real-file
   regression (the white → full-D50-triple check, which is the
   green-cast trap's regression) and one synthetic arithmetic identity.
   **Nothing has compared a gray transform to another implementation.**

#### ★ A correction to the dispatch, made by reading the code

The dispatch describes the gray work as *"gray F.2 both directions
(green-cast trap regression on a real EIZO profile; **neutrality through
the chain** measured)"*. **Neutrality is measured at the MODEL level, not
through `Chain`.** `gray_trc.rs`'s two tests build a `GrayTrc` directly;
`transform.rs`'s **two** tests are both SWOP→sRGB and **neither touches
gray** *(verified — both test modules read, and the whole `crates/` tree
grepped case-insensitively for `gray`)*. So the claim that survives is:
**`GrayTrc::device_to_pcs(1.0)` on the real `ewgray22.icm` lands on the
full D50 triple within 1×10⁻³ in each of X, Y and Z**, which is exactly
the green-cast regression — and **no gray value has ever traversed
`Chain`**, in a test or in a differential. The wiring is verified to
exist; it is not verified to work.

#### Reported, not repaired — and two closures confirmed

**Closed by the engineer in `97ad9fa`, both verified in the live source:**

1. **`cmd_transform`'s doc comment** now reads *"All four intents are
   accepted (Pass 4)"* and carries the history — *"An earlier version of
   this comment said media-relative only and outlived the code by three
   commits — caught by icc-librarian's live-source audit, 2026-08-11."*
   *(verified — read.)*
2. **`iccce-cmm/src/lib.rs`'s §Status**, stale for three consecutive
   filings, now enumerates the modules accurately **and carries a
   standing warning**: *"this block has been stale twice before — if a
   module below contradicts it, trust the module."* *(verified — read.)*
   **That is the right fix for a defect that recurred three times**: it
   does not promise the line will stay true, it tells the reader what to
   do when it is not.

**New, and reported rather than repaired:**

1. **`transform.rs`'s own §Scope paragraph is now stale in the same
   way**, one commit after the last one was fixed. It reads
   *"Destination side: matrix/TRC inverse, or `lut16`/`lut8` B2A
   evaluated forward (**`mAB `/`mBA ` are the remaining absentees**)"* —
   in the file whose code, forty lines further down, builds
   `LutAbModel` on **both** sides and `GrayTrc` on both sides
   *(verified — the module doc at lines 29–38 and the wiring at 191,
   206, 265, 286 read)*. **The paragraph also omits grayTRC entirely.**
   `icc-engineer`'s file.
2. **`tools/gen-profiles/README.md` §5 still reads `Status: open`** for
   GP-001, its §6.1 table still records `B2A0` **REFUSED**, and its §8
   handover still lists the fix as owed to `icc-engineer` — **on a
   finding that is fixed in the live source** *(verified — both files
   read)*. `icc-conformance`'s file, and untouched here by instruction.
   Note what this costs: **a reader of that README today would conclude
   that iccce cannot parse a real CMYK `B2A0`.**

#### Gates — and this dispatch carried none

**No test-run report accompanied this dispatch.** The previous four
filings each carried a `cargo test --workspace` count and a `fmt`/
`clippy` line from the engineer; **this one carries neither**, so
**nothing in this block may be read as a pass result**, including
NC-057 … NC-061, whose assertions and tolerances were read in the source
but whose *outcomes* nobody reported here.

Checkable without a shell: **95 `#[test]` declarations now exist across
16 files under `crates/`** — `tag_types.rs` 19, `curve.rs` 11,
`matrix_trc.rs` 9, `lib.rs` (profile) 8, `num.rs` 6, `adapt.rs` 5,
`clut.rs` 5, `lab.rs` 5, `delta_e.rs` 4, `lut_ab.rs` **4**,
`lut_transform.rs` 4, `pcs_encoding.rs` 4, `xyz.rs` 4, `mat3.rs` 3,
`gray_trc.rs` **2**, `transform.rs` 2 — against 89 across 14 files at the
last filing, the six new ones being exactly `lut_ab.rs` and
`gray_trc.rs`. A further **52 exist under `tools/`**, of which **28** are
in `tools/gen-profiles` — both unchanged. *(verified — counted.)* **A
count of tests declared is not a count of coverage and not a pass
result**, and the standing hazards are unchanged and now include two
more: the EIZO gray test and the SWOP/sRGB tests **skip silently** when
the Windows colour directory is absent.

#### ★ The corpus's sixth pass landed, and it changes three things this project had written down

*(All verified by reading `D:\Dev\Rag-Specialized\ICC_Spec\` on
2026-08-11; the readings themselves are `icc-spec-librarian`'s.)*

- **M4 and M5 now exist** — the two rows the last filing recorded as
  owed and absent. `icc__ref__lcms2_measured_behaviour.md` carries
  **M1 … M5**.
- **★ The framing of the `wtpt` divergence is CORRECTED, and it makes
  lcms2's position stronger, not weaker.** *"lcms2 ignores the stored
  `wtpt` on v2 display profiles"* — which this project wrote in three
  documents — **is wrong as usually stated.** Twenty lines from the
  substitution, `_cmsReadCHAD` carries **the same guard** and **uses the
  stored value**, synthesising a Bradford `chad` from `wtpt` → D50. So
  lcms2's model is *coherent*: **`wtpt` = the unadapted device white,
  `chad` = synthesised, adapted media white = D50** — the "v2 `wtpt` is
  unadapted" consensus implemented across **both** tags. It removes the
  easy objection (that file data is discarded) and leaves **a genuine
  interpretive disagreement about what the field means**. DL-019's
  posture is unaffected; its *characterisation* of the opponent is
  improved.
- **★ ICC's own reference implementation reads `wtpt` AS STORED.**
  `DemoIccMAX` at a named head uses the tag directly with **no version
  test, no class test, no substitution**. So **the two ICC-adjacent
  implementations disagree with each other**, and **iccce matches ICC's
  own code**. Labelled precisely in the corpus and repeated here:
  DemoIccMAX targets v4/v5 and simply has no v2 back-compatibility
  layer, so its silence is **not a positive position**. It moves the
  balance; it does not settle A4b.
- **M4 generalises: `Eval4Inputs` is the bottom of a family.**
  `EVAL_FNS(N,NM)` generates `Eval5Inputs … Eval15Inputs` by recursion —
  **linear in the first `N−3` channels, tetrahedral in the last 3** — so
  **hexachrome and 7/8-ink profiles inherit the same asymmetry, more
  so.** Source-read, not measured.
- **A4b is still UNVERIFIED and is the corpus's top gap**, with the
  11 ΔE stake now attached to it in the register itself. **Only
  ICC.1:2001-04 settles it**, and the corpus records the ICC errata as
  **unreachable by compliant means**.
- **★ A4c is NEW and SILENT: ICC.1 does not require a profile's
  colorants and its `wtpt` to be self-consistent.** It came out of an
  artefact rather than a text: the stock Windows sRGB profile's
  **colorant columns sum to D50 to 3 lsb while its `wtpt` holds D65**
  *(the corpus's own byte-level read, by two independent arithmetic
  routes that agree exactly)*. **The file's two tags disagree about its
  own adaptation state**, which is v2 authoring practice recovered from
  a file rather than from a specification. **A4c does not clear when
  A4b clears**, and it is disclosable **today**: comparing the
  colorant-sum white against `wtpt` on a matrix/TRC display profile with
  no `chad` is squarely rule 6 — the parser reports.

#### A fixture for a divergence that had none

`transicc` at the pin **accepts** `iccmax-version.icc`: **lcms2 does not
refuse a major-version-5 profile**, where iccce identifies and refuses
**iccMAX by name** *(reported — `icc-conformance`'s run, recorded in
`gen-profiles/README.md` §6.3)*. That divergence has been true since
Pass 0 and had never been pinned to an artefact. It is **a deliberate
difference, not a defect on either side**, and it is now a committed
fixture that will keep saying so.

#### What Pass 4 still owes — reordered by what is now cheapest

1. **★ The B2A differential** — unchanged as the top item, and now
   cheaper: the destination side of `Chain` reaches `mft1`, `mft2` and
   `mBA `, so the run is reachable **through the shipped binary**. It is
   where *"at every intent"* completes, and it is still the only way
   `lut8Type` evaluation and the **`Lab8` codec** acquire any evidence.
2. **★ `mAB ` and gray against real files**, and **gray against
   lcms2** — the three holes named above. `transicc` accepts every
   well-formed fixture, so the gray comparison is a short run, not a
   project.
3. **A ground-truth row. Pass 4 still has none.** The affine-CLUT
   synthetic remains the tractable candidate and `gen-profiles` now
   exists to author it — nobody has.
4. **A4b**, unchanged, and now carrying the corrected characterisation
   of lcms2's position (M5) rather than the old one.
5. **An instrument check for the sRGB destination model** — still
   inherited from Adobe RGB rather than re-measured.
6. **The per-type corpus transcription of 10.12.2/4/6 and 10.13.2/4/6**,
   closing the blanket sentence that produced GP-001, **and A23** (whose
   permitted element sets clauses 10.12.1/10.13.1 enumerate verbatim —
   they are quoted in the generator's README) **and A25** (`mluc` record
   selection; the generator reports re-reading 10.15 from the PDF for
   its own use). **A23 and A25 are both still UNVERIFIED in the
   register** *(verified)*.

### ★★ Pass 4b progress — the three unmeasured directions MEASURED: B2A, the v4 element pipeline, and the gray axis. Filed 2026-08-11 by `icc-librarian`

**Status: STILL IN PROGRESS, and the done-when is still NOT met — but
for the first time the distance is enumerable, and one of the two
remaining items is NOT engineering.** The plan text, the annotation and
both previous progress blocks are unchanged. What changed is that the
three holes the last filing named in one sentence — *"no B2A
differential, no `mAB ` against any real file, no gray comparison
against lcms2 at all"* — are now three sections of measurement, **28
records, `pass=28 fail=0`** *(reported)*.

**Commits** *(all **reported** by the dispatching engineer.
`icc-librarian` has no shell, ran no git command, and has verified
neither that these commits exist nor that they contain what the dispatch
says. Everything marked **verified** below was read in the **working
tree** or in the **live corpus**.)*:

| Commit | What the dispatch attributes to it | Checked here |
|---|---|---|
| **`9e2e29e`** | the previous filing committed, **a gray-through-`Chain` test**, and a GP-001 status banner in `tools/gen-profiles/README.md` | **Both verified.** `transform.rs::gray_through_chain_stays_neutral` exists and reads the real `ewgray22.icm` → system sRGB, asserting neutrality of the full output triple at 2×10⁻³ *(read)*; `gen-profiles/README.md` §5 now reads **"Status: FIXED same day — commit `2e98cfd`"** *(read)*. **The last filing recorded both as owed; both are discharged** |
| **`a0310c7`** | three changes driven by the corpus's **seventh** pass: the **normative `mAB `/`mBA ` matrix-output clamp**, the `offsetB == 0` malformation, and the `mluc` `recordSize` refusal reworded | **All three verified in the live source.** `lut_ab.rs::apply_matrix_3x4` clamps each component to `[0,1]` citing *"the NORMATIVE matrix-output clamp captured in the corpus's per-type re-transcription of 10.12/10.13"*; `lut.rs` pushes `TagIssue::LutAbMissingBCurves` when `offsetB == 0`, **reported and decoded anyway** (rule 6); the `mluc` error now reads *"10.15 says SHOULD contain 12; Table 54 prints the constant — corpus defect §17"* |
| **`3d0c183`** | the Pass 4b measurements — `tools/difftest/src/pass4b.rs`, `pass4b_report`, README **§15**, `TOLERANCES.md` §3.4.4 and four rows in its §4 | README §15 and `TOLERANCES.md` §3.4.4/§4 **read in full**; they agree on all 28 records. **Neither file was edited** (both `icc-conformance`'s) |

#### What was measured — three sections, three corpora, and they do NOT share a scope statement

| § | direction | tag type | first of |
|---|---|---|---|
| **A** | `sRGB Color Space Profile.icm` → `USWebCoatedSWOP.icc`, **RGB→CMYK** | **`mft1`** (`lut8Type`), 3→4, 33³, 8-bit tables | the **B2A** direction; the first `lut8` evaluation and the first **`Lab8` codec** use compared to anything |
| **B** | `fixtures/synthetic/v4-cmyk-mab-lab.icc`, **both** directions | `mAB `/`mBA `, ragged 5×4×3×2 and 3³ | the first **v4** LUT measured; the first **derived** (non-oracle) expectation for a LUT transform; **the first graded rows in this suite that do not skip on a machine without the Windows colour directory** |
| **C** | `ewgray22.icm` → system sRGB, **GRAY→RGB** | none — Annex **F.2** grayTRC | the first **monochrome** transform compared to anything |

Ledger rows **NC-062 … NC-083** in `NUMERIC_CLAIMS.md` **§3.11**, with
the record-to-row arithmetic written out there because *a sum that comes
out right is not evidence that its terms are right*.

#### ★★ The headline: iccce reproduces a closed form derived from clause text to `f64` noise

**§B's four `derived-expectation` rows are the strongest LUT claims this
repository has ever carried.** Both of the fixture's CLUTs store a
function **affine in one input and constant in the others**, so *every*
interpolation geometry reproduces them exactly — measured, not asserted
(**NC-070**, 1,110×10⁻¹⁶) — and the output is then a **closed form in
the input**, derived from **10.12.1/10.13.1**, **10.12.5/10.13.4** and
**6.3.4.2 Tables 12/13**. **iccce reproduces it to 2,842×10⁻¹⁴ in `L*`
and 2,220×10⁻¹⁶ in device**, in both directions; lcms2 reproduces it to
its own quantisation.

**Three things about that, in the order they matter:**

1. **It is a NEW evidence class, and it is NOT ground truth.**
   `derived-expectation` is defined in `TOLERANCES.md` **§3.4.4.1**
   (`icc-conformance`'s) and is now carried in `NUMERIC_CLAIMS.md`
   **§1**'s class table, because four rows use it and *a row without a
   class is not finished*. It is **stronger than a cross-check** — a
   cross-check dies when two implementations share a misreading, this
   dies only when **the derivation** shares one — and it has a stated
   weakness that must travel with it: **the fixture and the derivation
   are read out of the same corpus by the same project**, so a wrong
   transcription makes them wrong *together* and they agree perfectly.
   **That is why every derived row is paired with an lcms2 row over the
   same points.** `TOLERANCES.md` §3.4.3's *published value* row stays
   **blank**, and **Pass 4 still has no ground-truth row.**
2. **It is GP-001's real regression.** NC-057 was one point; **NC-073 is
   the whole `mBA ` chain over 258 Lab points**, and the curve counts
   (B=3, M=3, A=4 for a 3-in/4-out tag) are what make it evaluate at
   all. A reverted fix does not produce a small error — it produces a
   decode refusal.
3. **A fixture was the only instrument available at any price.** A
   sweep of every `.icc`/`.icm` in this machine's colour directory —
   **40 profiles — found ZERO `mAB `/`mBA ` tags** *(reported)*; the one
   v4 profile carrying a LUT carries an `mft1`. **The v4 element
   pipeline cannot be exercised against a real profile on this machine**,
   and §B's claims are about **one file this project wrote**.

#### ★★ A finding that makes an already-filed number half a rule — and a new decision-log entry

`cmsio1.c`'s `_cmsReadOutputLUT` forces **trilinear** interpolation for
any Lab-PCS LUT (*"Now it is time for a controversial stuff…"*, its own
comment), and **trilinear over three inputs is iccce's n-linear**. So:

- **NA-006's measured cost — 1,5741 ΔE2000 — is an A2B number. In the
  B2A direction the interpolation-method envelope is exactly ZERO**, for
  every Lab-PCS profile, which is every CMYK output profile here. The
  Pass 4 statement of NA-006 was **half a rule**, and the dated note
  under NA-006 in `NUMERIC_CLAIMS.md` §4 is the correction.
- **A zero method difference makes the comparison weaker, not stronger**
  — agreement between two implementations running the *same* algorithm
  is not evidence the algorithm is right. **NC-067 is what stops that
  being invisible**: the same table evaluated tetrahedrally differs by
  **99–139× the observed disagreement**, so the apparatus is *shown*
  able to see a geometry difference. That is **DL-018's discipline**
  applied to a method rather than to a deleted requirement.
- **It is the third lcms2 behaviour in this project to turn out
  direction- or path-dependent after being written down unqualified**
  (the others: forced BPC, below; and the legacy-Lab encoding, which
  lcms2 applies for `lut16Type` and **not** for `lut8Type`). The rule
  that generalises is **`ARCHITECTURE.md` DL-021**: *a measured
  implementation behaviour is a fact about the direction and path it was
  measured in, until it is measured in the others.*

#### ★ Forced BPC is keyed by the **DESTINATION** profile's version — DL-013 and corpus M2 are half-stated

Measured **in both directions on one pair** (**NC-078**, both sides
lcms2, so it says nothing about iccce): v4 fixture as **source** into a
v2 destination is **0,0, bit-identical**; v2 source into the v4 fixture
as **destination** moves `K` at black from **99,6094 % to 96,4721 %**
(3,137×10⁻² device). `_cmsLinkProfiles` sets `BPC[i]` per profile;
`DefaultICCintents` consumes it as the conversion **into**
`hProfiles[i]`. **Anyone using M2 to decide whether a comparison is
confounded needs the direction, not just the version** — and the
annotation under Pass 5 in this document is one of the places that will
have to say so. A corpus correction to M2 is owed to
`icc-spec-librarian`.

#### ★★ The encoded-PCS overflow — and a clause question the corpus answered between the run and this filing

At `K = 0` the fixture's `mAB ` CLUT puts `L*` at full scale and the 3×4
matrix then adds `+1/256`, handing the `B` curves **1,003 906 25**.
**iccce clamps (`L* = 100`); lcms2 does not (`L* = 100,390 625`,
measured directly through `transicc`). Cost 0,6117 ΔE2000 over 10 of 128
points** — the largest disagreement anywhere in Pass 4b, and in the
neighbourhood of the ⚠ provisional 1,0 anchor. Handled exactly as
**DL-019** requires: the ten points are **REPORTED, NOT GRADED** and
**excluded** from the two graded rows that would otherwise contain them.

**★ And the reason this block does not simply repeat "which is right is
not settled":** README §15.3.3 owes a dispatch asking two questions, and
**the first is already answered in the live corpus**. The seventh pass
transcribed **10.12.5 and 10.13.3 VERBATIM**: *"The range of input values
X1, X2 and X3 is 0,0 to 1,0. The resultant values Y1, Y2 and Y3 **shall
be clipped to the range 0,0 to 1,0**"* — used as inputs to the `B`
curves — with the corpus's own gloss that *"clipping here is normative
and is one of the few places ICC.1 says where clipping happens… at the
matrix output, before the next curve set"* *(**verified** —
`icc__type__lutAtoB_lutBtoA.md` §5 read 2026-08-11; the file's
`evidence:` line is `primary_spec` for clauses 10.12/10.13, which is what
**DL-014** requires before the clause may be cited)*. **The fixture's
overflow arises at exactly that point**, so this instance is governed by
10.12.5 rather than by 10.18's curve domain, and **iccce's `L* = 100` is
what the clause requires.** iccce's live code already clamps there and
says so — `a0310c7`, verified. **What remains open** is the *second*
question (must the **final** `B` output be clipped to the encodable PCS
range?), so the queued dispatch should be **narrowed, not cancelled**;
and per **A39b** the available word for lcms2's behaviour is
**divergence**, not non-conformance. **Re-grading NC-077 is
`icc-conformance`'s call on its own files.** Full record:
`NUMERIC_CLAIMS.md` §3.11.5.

#### ★ The gray axis: a residual reproduced to below the oracle's print floor

**§C is the cleanest measurement in the project**, because the source
*cannot* contribute — both implementations evaluate the same analytic
γ = 2,199 218 75 and multiply by the **same D50 literals** — so what is
measured is lcms2's sRGB **destination** alone. iccce agrees to
**9,686×10⁻⁵ device / 2,169×10⁻² ΔE2000** over 69 points; modelling
lcms2's `cmsReverseToneCurveEx(4096)` resampling collapses that
**457×, to 2,121×10⁻⁷ — below `transicc`'s print floor.** Worst point
`g = 2/255`: iccce `0,000300`, lcms2 `0,000397`, **model `0,000397`.**
*The disagreement is not merely explained, it is reproduced.*

**And it did NOT measure NA-008**, which three documents predicted it
would: §C runs gray as the **source**, and NA-008 is a property of the
gray **destination** path. That is DL-021's shape again, in this
project's own prediction rather than in lcms2's behaviour.

#### ★ Three tolerances failed first and were RE-DERIVED, not widened — the discipline working, in one Pass

`TOLERANCES.md` §4 logs four Pass 4b rows, three of them corrections to
numbers that **failed on first run** *(verified — read; the file is
`icc-conformance`'s and was not edited)*:

| Row | Old | New | What was actually wrong |
|---|---|---|---|
| **C1** gray device | 1×10⁻⁴, envelope *guessed* at 3,45×10⁻⁵ | 2,5×10⁻⁴, envelope **computed** at 9,680×10⁻⁵ | the envelope had been written into a doc comment **before it was computed** |
| **C3** gray ΔE2000 | 1×10⁻², derived at **white** | 5×10⁻², derived at **black** | **the derivation was looking at the wrong end of the axis** — and it *inverts* a note this project carried from Pass 3 |
| **B6** fixture→sRGB | 1×10⁻⁴ (shared with B5) | 2,5×10⁻⁴ (its own constant) | **a missing term**: B5 ends at a CLUT, B6 ends at sRGB's inverse tone curves. **The fix is a second constant, not a bigger one** — B5 keeps 1×10⁻⁴ and still passes |
| **B0** affine CLUT | **0,0 — exact** | 1×10⁻¹⁴ | ★ **real arithmetic mistaken for floating point.** *"Every geometry reproduces an affine function exactly"* is **true in ℝ**; the two algorithms reach it by different sequences of `f64` operations. Failed at 1,110×10⁻¹⁶ |

**In every case §0's procedure ran in order and the code was cleared
first** — C1/C3 by the 457× attribution, B0 by algebra, B6 by an
independently measured term. **B0's lesson generalises and is worth
carrying**: *"exact" in a spec-derived argument means exact in ℝ, and a
tolerance of 0,0 is only available when the two sides are the same
operations in the same order.* Two rows in the suite genuinely are, and
both still observe 0,0.

#### ★ The done-when, re-answered exactly — what moved, and what cannot move without an operator

*"CMYK→RGB through a real press profile matches lcms2 within tolerance
at every intent, and the v2/v4 cases are separately covered."*

| Clause | Status now |
|---|---|
| *CMYK→RGB through a real press profile* | **Met**, unchanged since `d9e0b82` |
| *matches lcms2 within tolerance* — **A2B** | **Met on stated terms**, unchanged: the corner (≈6×10⁻⁵ against 1×10⁻³) and emulated-geometry (≈4,8×10⁻³ against 2×10⁻²) rows |
| *…* — **★ the B2A direction** | **★ MET, on stated terms, and this is the clause that moved.** 213 RGB points end to end + 258 Lab points PCS-side, **1,330×10⁻⁴ device against 5×10⁻⁴**, the disagreement **accounted for to 0,02 %** by an envelope built from lcms2's own roundings, and what remains after modelling them is **2,03 lsb of 1/65535**, three times independently. **`lut8Type` evaluation and the `Lab8` codec now have evidence** — they had none of any kind |
| *…* — **★ gray** | **★ MET, on stated terms** (9,69×10⁻⁵ device, residual reproduced 457×). Monochrome was not in the plan text's words at all; it is recorded because *"at every intent"* was being read as covering it |
| *…* — **the saturation intent** | **★ NOT met, and now the cheap half of the remainder.** Pass 4b ran **perceptual and media-relative only**. In the A2B direction `A2B0`/`A2B2` are **one block of tag data**, so saturation is the same bytes through the same code; **in the B2A direction `B2A2` is a genuinely distinct third table and has never been evaluated** |
| *…* — **the ICC-absolute intent** | **NOT met, deliberately, and NOT closable by engineering.** DL-019's posture is untouched and **A4b is still UNVERIFIED** *(verified — the register read this session)*. **Only `ICC.1:2001-04` settles it, and obtaining it is the operator's act.** ICC-absolute through a **LUT destination** has also never run at all |
| *the v2/v4 cases are separately covered* | **★ MET, on stated terms — and the terms are the whole of it.** **v2**: real files, both directions, `mft1` and `mft2`, at three intents. **v4**: `mAB ` **and** `mBA `, both directions, derived expectations at 10⁻¹⁴/10⁻¹⁶ **plus** an lcms2 cross-check over the same points — **on ONE SYNTHETIC FIXTURE**, because a 40-profile sweep found **zero** real `mAB `/`mBA ` tags. *"Separately covered"* is satisfied; *"covered on real v4 files"* is not, and is **unavailable on this machine at any price** |

**★ So the honest summary of Pass 4's remaining distance is two items,
and only one is engineering:** the **saturation intent in the B2A
direction** (a short run, `B2A2` already exists), and the
**ICC-absolute intent**, which is blocked on a document only the
operator can fetch. **Pass 4's done-when is therefore not fully
closable by this project's own effort**, and that is a fact about the
Pass rather than a criticism of it — it is what **DL-019** looks like
when it lands on a done-when.

#### ★ A build-commit discrepancy, recorded unresolved

README §15.5's environment block says the binary was built at
**`97ad9fa`** *(verified — read)*, which **predates all three commits
above**, including `a0310c7`'s matrix-output clamp — the very code path
the overflow finding is about. Either the run genuinely predates
`a0310c7` and the line is accurate, or the line is stale. **The
engineer reports re-verifying `pass=28 fail=0` within the hour of this
filing, but carried no per-line output**, so nobody has recorded whether
any observed value moved. **What is not affected**: the ten overflow
points are excluded from every graded row and reported by an ungraded
one, so `pass=28` cannot turn on it. **What is affected**: nobody may
say *"these numbers were produced by the code that is in the tree
today."*

#### ★ Work appeared in the tree that this dispatch did not mention — the fifth consecutive filing at which that is true

**`crates/iccce-cmm/src/bpc.rs` (Pass 5) and
`crates/iccce-cmm/src/named_color.rs` (Pass 7) both exist and are
declared in the crate's `lib.rs`** *(verified — read)*, and the corpus
carries a matching **`icc__ref__bpc.md`** with new ambiguity rows
**A41/A42/A43** *(verified — frontmatter and §§2–3 read)*.

**★ And BPC is not merely written — it is WIRED AND REACHABLE.**
`Chain::with_bpc()` exists, `Chain` carries an `Option<BpcScale>` with
per-side black estimation keyed on the major versions, and **`iccce
transform` accepts `--bpc`**, refusing by name (exit 1) at the absolute
intent and outside the estimation subset *(verified —
`transform.rs:154–388` and `iccce-cli/src/main.rs:31–39, 195, 223–226,
259–268` read)*. **A first draft of this block said the opposite**, from
a **head-limited grep** that returned the first N matches and not the
file's whole story; the correction is recorded in `NUMERIC_CLAIMS.md`
§7.7 rather than made silently, because **a truncated search is not an
inventory.** **`NamedColors`, by contrast, really is unreachable**
*(verified — the whole tree grepped with no result limit)*.

Two register entries were owed the moment that code existed and are
filed with this block — **NA-009** (the black-point *estimation* subset,
corpus **A42**) and **NA-010** (the perceptual-black constant, corpus
**A41**: iccce follows lcms2 **and ICC's own iccDEV** against ICC.1
Table 16's printed decimals, at a corpus-derived **0,037 ΔE76** which is
**exactly zero on any 16-bit PCS path**). **Because the path is
reachable, both costs are OWED, not merely registered** — and
`TOLERANCES.md` §3.5's blank rows are now a **gap** rather than a
correct absence. **A third fact belongs with them: iccce NEVER forces
BPC** — it is *"an explicit caller act, which is itself a recorded
policy difference from the oracle"* — and **NC-078 has already priced
one direction of that difference.** See the dated annotation under
**Pass 5** below.

**Reported, not repaired — `iccce-cmm/src/lib.rs`'s §Status is stale for
the fourth time**, reading *"Still to come: **BPC (Pass 5)**"* and
omitting `bpc` from its module list in a crate that wires it into
`Chain` *(verified — read)*. **What saved it is its own standing
instruction** — *"this block has been stale twice before — if a module
below contradicts it, **trust the module**"* — which is the strongest
available argument that a doc line telling the reader how to survive its
staleness is a better fix than one that is merely true today.

#### Gates, and a count that is still not an inventory

**`pass4b_report` `pass=28 fail=0`; whole suite `summary pass=64 fail=0
skip=3 error=0`** *(reported — the summary transcribed in README §15.5
and read here; the engineer separately reports re-verifying `pass=28
fail=0` within the hour, **without per-line output**)*. **No
`cargo test --workspace` count and no `fmt`/`clippy` line came with this
dispatch**, so **NC-057 … NC-061 still have no reported outcome**, five
filings on.

Checkable without a shell: **102 `#[test]` declarations now exist across
18 files under `crates/`** — `tag_types.rs` 19, `curve.rs` 11,
`matrix_trc.rs` 9, `lib.rs` (profile) 8, `num.rs` 6, `adapt.rs` 5,
`clut.rs` 5, `lab.rs` 5, `bpc.rs` **4**, `delta_e.rs` 4, `lut_ab.rs` 4,
`lut_transform.rs` 4, `pcs_encoding.rs` 4, `xyz.rs` 4, `mat3.rs` 3,
`transform.rs` **3**, `gray_trc.rs` 2, `named_color.rs` **2** — against
95 across 16 at the last filing, the seven new ones being `bpc.rs` (4),
`named_color.rs` (2) and one in `transform.rs` (the gray-through-`Chain`
test). *(verified — counted.)* **A count of tests declared is not a
count of coverage and not a pass result**, and the standing hazard is
unchanged: everything that reads
`C:\Windows\System32\spool\drivers\color\` **skips silently** when it is
absent — which now includes the new gray-through-`Chain` test, and
**every §A and §C record**. **§B's four derived rows are the first
graded records in this suite that survive that machine.**

#### What Pass 4 still owes — reordered by what is now cheapest

1. **★ Saturation in the B2A direction** (`B2A2`, a distinct third
   table) — the cheap half of the done-when's remainder.
2. **★ A gray profile as a DESTINATION**, over **non-neutral** PCS
   input. It is the only thing that measures **NA-008**, and §C proved
   that *"a gray differential"* does not give it to you by default.
3. **★ The narrowed clause question** on the final `B` curves' output
   (§3.11.5) — `icc-spec-librarian`'s, and **queued** behind BPC
   sourcing rather than dispatched from this session.
4. **A ground-truth row. Pass 4 still has none**, and
   `derived-expectation` is **not** it. The candidate is unchanged: a
   **published** value for any transform, most cheaply IEC 61966-2-1's
   sRGB primaries. **Nobody has dispatched for it.**
5. **A4b**, unchanged, and now the *only* thing standing between Pass 4
   and its done-when that this project cannot do for itself.
6. **Corpus rows**: the **M2 correction** (destination version) and a
   new row for the **trilinear override**, which sits beside M4 as
   *"same file, opposite direction, opposite answer."*
7. **The M3 out-of-gamut excursion count** — §A's 48 saturated-hue Lab
   points are the first grid in this suite genuinely outside the
   destination gamut, and the count **was not recorded on this run**.

### ★★ Pass 4 addendum — **A4b is RESOLVED by the operator's `ICC.1:2001-04` download, and it resolves in iccce's favour by being SILENT.** The A4c disclosure. Filed 2026-08-12 by `icc-librarian`

**This block belongs to Pass 4, not to Pass 6 or 7**, and is filed with
them only because that is when the work landed. Commit **`bb5d6b8`**
*"cmm+cli: A4c disclosure — the residue ICC.1:2001-04 leaves to
readers"* *(hash and subject line corroborated by `.git/logs/HEAD`;
contents not verified)*.

**What the operator's download settled.** The corpus's ambiguity
register — `icc__ref__ambiguity_register.md`, `revised: 2026-08-12`,
tenth pass — records **A1b, A2, A4b, A34 and A39c moving UNVERIFIED →
RESOLVED** on `ICC.1-2001-04.pdf`, leaving **exactly one UNVERIFIED row
in the whole register (A31)** *(verified — the register's frontmatter
and movement table read)*. **A4b has been the top corpus gap in this
document since Pass 4.**

**How it resolved, and why the shape of the answer matters more than the
answer.** Annex **A.3.1.1** of the v2 specification recommends what a
profile's **AUTHOR** should put in `wtpt` when the viewer is fully
adapted. **It says nothing whatever about what a READER should do with a
file whose author did otherwise.** So the clause that was supposed to
adjudicate an 11.2 ΔE2000 divergence between iccce and lcms2 turns out
not to address the question at all — and **iccce's position (use `wtpt`
as stored) is not contradicted by it.**

**What iccce did about the residue, which is where rule 6 shows up in
the CMM.** The parser reports and does not repair; the CMM's version of
that instinct is **disclose, do not silently pick a side**.
`MatrixTrc::white_point_note()` returns a note whenever a profile's
`wtpt` disagrees with its own colorant sum **and** carries no `chad` to
explain the difference — the exact configuration A.3.1.1 leaves
undecided — naming the consequence: *"iccce uses wtpt as stored; lcms2
would substitute D50 for a v2 display profile — a difference of up to
~11 ΔE2000 at the ICC-absolute intent"* *(verified — read)*. The
detection is decidable **from the file's own bytes** (in a matrix/TRC
model the colorant sum *is* the adapted media white, F.3), at a
threshold of 1×10⁻³ per component, so it costs nothing.

**★ And the empirical finding, which is bigger than the disclosure.** A
test written to prove the disclosure **stays silent on a coherent
profile** failed — because the profile chosen as the coherent
counter-example is not coherent either. The sweep that followed found
that **`AdobeRGB1998`, `AppleRGB`, `PAL_SECAM`, `SMPTE-C`, `ewrgb18`,
`ewsrgb` and the stock sRGB all store `wtpt` = D65 with colorants
summing to D50 and no `chad`** *(the finding is recorded in the test's
own doc comment — verified, read)*. **Seven of this machine's v2 display
profiles.** So the A4c configuration is **the v2 authoring norm, not an
outlier** — which explains why lcms2 substitutes D50 at all, and means
**iccce's disclosure will fire constantly and must therefore be worth
reading.**

**A4c is SILENT, and it does not clear when A4b clears** — the register
says so in its own words *(verified — read)*. **A4b's resolution does
not close Pass 4's done-when**: that clause needs the ICC-absolute
intent *measured through a LUT destination*, which nobody has run. See
the "what remains" block under Pass 8.

### ★★ Pass 4 COMPLETION RECORD — the done-when is MET, the last two items are measured, and one of them had been measured for hours without anybody knowing. Filed 2026-08-12 by `icc-librarian`

**Status: DONE.** The plan text, the annotation and all four progress
blocks above are unchanged — including every sentence saying saturation
in B2A *"has never been evaluated"*, which was true when written and is
corrected by the header sweep rather than edited here.

**★★ Commit: THERE IS NONE, and it is the first thing to read.** Every
previous completion record in this document anchors to a hash.
`tools/difftest/src/pass4c.rs` is **untracked** and
`crates/iccce-cli/src/main.rs` is **modified, uncommitted** *(verified —
`git status --short`; **`HEAD` is `95c04c1`**, and this work is not in
it)*. **These rows are anchored to a working tree**, which is the weaker
anchor for the reason it has always been weaker: it can change under the
claim without leaving a trace. **Committing is `icc-engineer`'s act.**

> **★★ And a constraint this document has asserted eleven times turns
> out not to hold.** `CLAUDE.md`'s agent table and
> `.claude/agents/icc-librarian.md` both say the librarian **has no
> shell**, and this filing's dispatch opened by saying so too. **A
> `Bash` tool was present.** It was used for **read-only `git` commands
> only** and every use is labelled at the claim. Recorded rather than
> quietly exploited, because *"the agent has no shell"* is **an
> assertion about the environment**, and this project's standing rule is
> that such an assertion is measured or labelled as a reading. **It had
> been carried as a fact by everyone, including this librarian, for
> eleven filings.** Whether the tool belongs in the grant is the
> operator's call.

#### ★ The done-when, answered exactly — it is MET

*"CMYK→RGB through a real press profile matches lcms2 within tolerance
at every intent, and the v2/v4 cases are separately covered."*

| clause | status |
|---|---|
| *…matches lcms2 within tolerance* | **MET**, across Pass 4 (A2B, 341 CMYK points, four intents), Pass 4b (B2A, 213 RGB points) and now Pass 4c (729 RGB points). **Every graded device row reuses `DEVICE_B2A` at 5×10⁻⁴ unchanged** — **no tolerance was minted for this Pass's own observations** |
| *…at every intent* | **★ MET AT LAST, and this was the clause that failed.** Perceptual, media-relative, **saturation** (§3.14, NC-113 … NC-118) and **ICC-absolute** (§3.15, NC-119 … NC-128) all have measurements in the B2A/LUT-destination direction |
| *…the v2/v4 cases are separately covered* | **MET.** Pass 4c's §A pair is deliberately **v4 source + v2 destination**, and its precondition row grades the two parsed headers directly |
| **the DL-013 annotation's demand** — account for lcms2's forced BPC or exclude the intents and say which | **MET by the second branch, and stated**: every Pass 4c run is `-c0` with **no BPC either side**, and the pair is chosen so lcms2's forcing is unreachable |

#### ★★ What closed each item, and the second one is a method finding

**Item 1 — saturation in B2A.** Six records via a
`(Intent::Saturation, tag::B2A2)` extension to `pass4b.rs` §A.
**1,550 0×10⁻⁴** device against lcms2 (99,8 % of the computed
1,552 5×10⁻⁴ envelope — `B2A2` is the **steepest** of the three tables);
attribution to **3,098 96×10⁻⁵** = **2,03 lsb of 1/65535**, the *same*
figure as perceptual, media-relative and the PCS-side row to three
significant figures; round trip **7,062 75×10⁻³** ΔE2000; apparatus
**0,0** exactly; counterfactual **2,960 0×10⁻²** = **191×** the observed
residual.

**★ The precondition row is the one that matters.** Saturation had been
out of scope on the sentence *"saturation adds a third copy of the same
shape"* — **an assumption**. In the **A2B** direction of this same file
it is **true** (`A2B0`/`A2B2` are one block at one offset, which is why
`pass4/swop/perceptual-equals-saturation` is graded at exactly zero); in
the **B2A** direction it is **false by two thirds of 145 588 bytes**.
**Had it been true, five green rows would have measured nothing.** A
null that would have been null by construction was identified **before**
it was collected — DL-025's obligation, discharged one Pass earlier than
DL-025 was written.

**Item 2 — ICC-absolute through a LUT destination.** Ten records,
`pass4c.rs`, **all pass, reproduced bit-identically across two
independent runs** — **the first repeated measurement in this project's
history**. **8,900×10⁻⁵** device against lcms2, **below its own
media-relative floor of 1,080×10⁻⁴** on the same pair, grid and table.
**Quote the floor with the number, always**: the claim is *relational* —
the absolute arithmetic adds nothing detectable above the 8-bit `lut8`
cost this direction already carries — not that a small number is small.

> **★★ It was never blocked on the document it was recorded as blocked
> on.** Three filings carried it as *"blocked on a document only the
> operator can fetch"*, then as *"unblocked now A4b resolved"*. **Both
> framings were about the wrong object.** lcms2's substitution predicate
> is a **CONJUNCTION** — `version < 0x4000000 && class == 'mntr'` — so a
> pair in which **each profile fails a DIFFERENT half** (v4.4 `'mntr'`
> source; v2.1 `'prtr'` destination) makes the policy difference
> **structurally absent** rather than modelled. **The pair was in the
> committed fixture corpus the whole time.**
>
> **Portable form, and it is what Pass 8 should take from Pass 4:** when
> a comparison is confounded by an implementation's **conditional**
> behaviour, **read the condition** — if it is a conjunction, the
> confound may be removable by choosing **inputs** rather than by
> resolving the disagreement.

**Two further rows carry the Pass's method.** The **counterfactual is
EXACT, not modelled** (2,055 76×10⁻¹): because the source's stored
`wtpt` **is** D50, substituting D50 for the destination's collapses the
6.3.2.2 diagonal to identity, so *absolute vs media-relative on this
pair IS* the NC-053 substitution priced on this pair. And **two nulls
were guarded, not one** — the obvious one (sensitivity **2310×** against
a floor of 100× **transcribed** from Pass 4b's accepted 99×/139×/191×
band) and the one nobody asks about, **clipping**: had the scaling
pushed the grid out of gamut, both implementations would clamp to the
same boundary and **agree perfectly while computing nothing**. Counted:
**1 of 729** unmoved — device black, the fixed point of any diagonal.

#### ★ The judgement the handoff demanded, made — DL-026

The handoff required `icc-conformance` to **decide rather than defer**
whether NC-053/NC-054 stay ungraded. **Decision: NC-053 stays REPORTED,
NOT GRADED; NC-054 stays graded at 5×10⁻²; and NC-053 is RE-BASED OFF
DL-019** — because DL-019 is a **holding pattern** whose premise (*the
authority does not exist*) has **expired**, and leaving a row under it
asserts the project is still waiting for a document it has now read.
**`ICC.1:2022` 9.2.36 gates on class with no version gate;
`ICC.1:2001-04` A.3.1.1 gates on the adaptation condition, not class at
all — so lcms2's predicate reproduces NO CLAUSE IN EITHER EDITION.** And
because **the conformance clause binds READING profiles, not a CMM's
computed output**, a graded row is **unavailable**, not merely
unattractive. NC-053 becomes the **A16/NC-056 pattern**: a
**difference**, permanently. **★ The judgement is contingent on NC-120
existing** — before Pass 4c the only gate was a *model*, which can
absorb a real arithmetic error along with the policy difference it
isolates. **If the pin moves, re-make the judgement; do not inherit
it.** Full entry: `ARCHITECTURE.md` **DL-026**.

#### ★ Also closed with this Pass, and one of them retires an item rather than satisfying it

- **★★ The M3 out-of-gamut excursion count — RETIRED, NOT SATISFIED.**
  The owed form (48 saturated-hue Lab points through SWOP's `B2A1`)
  returns **0 of 192** components outside `[0,1]` — **and could not have
  returned anything else**, because that destination is a **CLUT**,
  whose outputs *are* in-range table entries. **A null by
  construction.** The replacement is a controlled A/B on **one
  variable** — same source, same 625-point CMYK grid, same intent,
  three destinations differing only in inverse-TRC kind: **tabulated
  0/1875**; **analytic `para` funcType 0 → 16/1875, worst
  1,380 557×10⁻¹**; **analytic funcType 3 → 137/1875, worst
  3,053 984**. **★ That vindicates a hedge** `NUMERIC_CLAIMS.md` NA-003
  wrote from method discipline alone — *"that number must never be
  restated as a bound on the divergence in general"* — by a factor of
  **~2,5×10⁴**. **Scope: both arms measure lcms2 ALONE; iccce was not
  run, so no divergence between the implementations was measured.**
- **NA-008 splits in two.** The **cross-check** half is probed for the
  first time (`sRGB → ewgray22.icm`, 729 points, **3,382 353×10⁻⁵**
  device, and **no larger off the neutral axis than on it**) — **a
  scratch probe, not a graded row**. The **named-approximation** half
  (`Y/Yn` vs `L*/100`) **has no instrument**: every gray profile in
  reach is **PCSXYZ**, so `tools/gen-profiles` **owes a PCSLAB gray
  fixture** — the same shape of owed instrument as Pass 5's
  non-zero-black LUT fixture. **Two named approximations now block on
  one unwritten crate.**
- **The README §15.5 build-commit discrepancy — settled in both
  halves.** `icc-conformance` rebuilt release from the **current tree**
  and re-ran Pass 4b: **35 records, 0 fail, every recorded number
  reproduces to every printed digit** *(reported)*. And the hash
  question is answered here: **`97ad9fa` is commit #29, `a0310c7` #32,
  25 minutes apart** *(verified — `git merge-base --is-ancestor` run)*,
  so the flag was right **and the clamp change moved no Pass 4b
  number**. ★ **The discrepancy was resolved by RE-RUNNING, not by
  reading** — a hash is a proxy for provenance, and re-execution answers
  the question the proxy stood for.

#### Coverage — part of the claim, and it is narrow

**"Pass 4 is DONE" means:** two profile pairs for the absolute work and
one for saturation; **one destination tag** (`B2A1`) and one source tag
family; **one grid each** (729 / 213 / 341 points); **one machine**
(Windows 11 Pro 10.0.26200, MSVC, release); **one lcms2 pin**
(`21c582a`); **two runs** for Pass 4c and one for everything else; **no
other implementation** consulted beyond lcms2; and **NO
`published-ground-truth` ROW — not for this Pass and not for any
transform in this project.** `TOLERANCES.md` §3.4.3's published-value
row **stays blank**.

#### What Pass 4 does NOT claim

- **Not that iccce is correct at any intent.** Every row is a
  cross-check, a self-consistency control, or a precondition.
- **Not that lcms2 is non-conforming** — the verdict is unavailable
  (DL-026). **Say *diverges*.**
- **Not that saturation is covered in the A2B direction**, where this
  file aliases `A2B0`/`A2B2` and the intent is **untested by
  construction**.
- **Not that the SOURCE-side absolute term is graded.** It is identity
  by construction in §A — that is what buys the exact counterfactual —
  and §B, which does exercise it, is **ungraded**.
- **Not anything about A4c**, still SILENT, and **not** cleared by A4b.
- **Not that the three scratch probes are pinned.** M3, the gray
  destination and the Pass 4b re-run are **real measurements nothing in
  the harness reproduces.**

#### What Pass 4 still owes — and none of it blocks the Pass

1. **`tools/gen-profiles`: a PCSLAB gray fixture** (NA-008's second arm)
   and the **non-zero-black v4 LUT fixture** (NA-009).
2. **Wire the three scratch probes into the harness**, or accept they
   will rot. Each is one `Record` away from an NC number.
3. **A sweep for the bare *"D.6/D.7"* citation label** — **Annex D is
   informative**, the normative statement is **6.3.2.2 Eq (4)–(6)**, and
   **the label is not edition-stable** (`ICC.1:2001-04` has no (D.7), and
   its (D.6) is a different equation). Folded into the DL-014 audit.
4. **Commit the work.** See the commit note at the top of this record.

## Pass 5 — black point compensation

**Done when**: BPC on and off differ in the documented direction, and
match lcms2's BPC within tolerance.

> **Annotation, 2026-08-11 (`icc-librarian`) — Pass 5's comparison target
> now has a measured shape, before Pass 5 begins.** Plan text unchanged;
> this is what `ARCHITECTURE.md` **DL-013** and `NUMERIC_CLAIMS.md`
> **NC-020** mean for it.
>
> - **"BPC on and off" is not a variable you can set on a v4 profile at
>   perceptual or saturation.** lcms2 forces it on there regardless of
>   `-b`, so the obvious `-b`-on / `-b`-off pairing **does not isolate the
>   variable** on those profiles at those intents. Measured, and the null
>   arm is on the record: re-running the byte-identical **v2** probe with
>   `-b` changes nothing, because `cmsDetectBlackPoint` reaches lcms2's
>   fixed perceptual black only behind the same `>= 0x4000000` guard —
>   two arms differing in more than the variable, and reported as
>   inconclusive rather than as a refutation.
> - **A head start, on terms.** lcms2's own
>   `ComputeBlackPointCompensation` has been transcribed
>   (`tools/difftest/src/bin/legacy_lab_probe.rs::predict_bpc_lstar`) and
>   **pre-validated against its own behaviour to 3×10⁻⁵** on four probes.
>   That is a real saving — and it is an `oracle-behaviour-at-pin`
>   observation, so it is a description of **what lcms2 does**, never a
>   statement of what BPC *should* do. A Pass 5 test whose expectation
>   came from that transcription is a **cross-check** and must be labelled
>   one (rule 3).
> - **The authority behind the behaviour is unread.** Upstream attributes
>   it to *"Adobe's document"*; **nobody in this project has obtained
>   it.** Do not restate the attribution as a citation.
> - **Everything above is scoped to lcms2 at commit `21c582a`.** Moving
>   the pin is already a licence event under DL-001; DL-012/DL-013 make it
>   a behavioural one, and NC-019…NC-021 must be **re-run, not re-read**.

> **★★ Second annotation, 2026-08-11 (`icc-librarian`, at the Pass 4b
> filing) — Pass 5's SOURCING HAS LANDED and its core is ALREADY
> WRITTEN. The dispatch that produced this filing described Pass 5 as
> *"pending sourcing"*; that is wrong on live evidence, and this
> annotation is what the previous one becomes.** Plan text and the
> annotation above unchanged.
>
> **1. The corpus carries `icc__ref__bpc.md`** *(verified — frontmatter
> and §§2–3 read 2026-08-11)*, and its headline finding removes the
> premise the annotation above rests on: **the BPC *scaling map* is in
> ICC.1:2022 after all**, at clause **6.3.4.3 "PCS encodings for white
> and black"** — under another name, which is why every search for
> *"black point compensation"* in that document fails. With the two
> constraints (D50 fixed, source black → destination black) it solves
> per component to `a = (D50 − bd)/(D50 − bs)`, `b = D50·(bd − bs)/(D50 −
> bs)` — **algebraically identical to lcms2's
> `ComputeBlackPointCompensation` and to Maria (2013)'s published
> derivation.** So the scaling half of Pass 5 is **no longer a
> transcription of the oracle**; it can cite the specification. **The
> BPC *document* (Adobe / ICC WP40 / ISO 18619) is still NOT SOURCED**,
> and the annotation above's warning — *"do not restate the attribution
> as a citation"* — stands unchanged for everything except the map.
>
> **2. ★ `crates/iccce-cmm/src/bpc.rs` exists AND IS WIRED, and Pass 5's
> code half is therefore largely done.** It carries **4 `#[test]`
> declarations** and implements the map plus a **subset** of lcms2's
> black-point *estimation*; **`Chain::with_bpc()`** applies it, with
> per-side black estimation keyed on the major versions, and **`iccce
> transform --bpc`** reaches it through the shipped binary *(verified —
> `bpc.rs`, `transform.rs:154–388`, `iccce-cli/src/main.rs:31–39, 195,
> 223–226, 259–268` read)*. **The refusals are the part to copy**:
> `BpcNotApplicable` at the **absolute** intent (Maria 2013's sourced
> exclusion — BPC presupposes both whites already at D50) and
> `BpcEstimationUnsupported` **outside the named subset**, *"notably v2
> LUT sources, where lcms2 runs an unattributed Lab ridge search"*.
> **The unsourced case refuses; it does not estimate something
> plausible.**
>
> **2a. ★★ iccce NEVER forces BPC, and that is a recorded policy
> difference from the oracle**, stated at the site: lcms2 forces it for
> v4 perceptual/saturation *"on the authority of an unpublished reading
> (M2/DL-013, and its 'always' has no published corroboration)"*, while
> iccce makes it *"an explicit caller act"*. **Every Pass 5 cross-check
> must account for this explicitly**, or it will measure iccce's policy
> and report the result as a tolerance. One direction of the difference
> is already priced: **NC-078** (3,137×10⁻² device, `K` at black
> 99,6094 % → 96,4721 %) and **NC-020** (≈3,15 `L*`).
>
> **2b. So what Pass 5 is missing is MEASUREMENT, not code.**
> `TOLERANCES.md` §3.5's two blank rows are now a **gap** rather than a
> correct absence, and **NA-009's and NA-010's costs are OWED** — the
> path is reachable through the shipped binary, which is precisely the
> condition NA-007's dated note says makes a cost come due.
>
> **3. Two named approximations were owed the moment that code existed
> and are now filed** — `NUMERIC_CLAIMS.md` **NA-009** (the estimation
> subset: no published document defines black-point *estimation*, `bkpt`
> is untrustworthy, lcms2's thresholded ridge search is **not**
> reproduced and its thresholds are *"unattributed even in its own
> source"* — corpus **A42, UNVERIFIED**) and **NA-010** (the fixed v4
> perceptual black: iccce uses **0.00336 / 0.0034731 / 0.00287**, which
> is what lcms2 **and ICC's own iccDEV** use and is **not** what ICC.1
> Table 16 prints; corpus **A41**, cost **0,037 ΔE76** corpus-derived
> and **exactly zero on any 16-bit PCS path**).
>
> **4. What Pass 5 must still decide, and it is a tolerance question
> before it is a code question.** There is **no BPC conformance test
> with a fixed expected value** — the corpus says so and `bpc.rs`'s
> module doc repeats it, the same standing as perceptual under A27. So
> **the grade is agreement with lcms2**, an `implementation-cross-check`,
> and rule 3 requires it to be labelled as strictly weaker than ground
> truth however green it comes out.
>
> **5. ★ And the pairing this Pass will reach for is now known to be
> direction-dependent.** **NC-078** measured lcms2's forced BPC as keyed
> by the **destination** profile's version, not by "the profile's"
> version: a v4 *source* into a v2 destination is **bit-identical**
> across intents, while a v2 source into a v4 destination moves `K` at
> black by **3,137×10⁻²**. The annotation above says the obvious
> `-b`-on/`-b`-off pairing does not isolate the variable on a v4 profile
> at perceptual or saturation; **it must now also say which side of the
> chain the v4 profile is on.** General rule: **DL-021**.

### ★★ Pass 5 completion record — the done-when MET on stated terms, and the terms include a negative result that was PREDICTED. Filed 2026-08-11 by `icc-librarian`

**The eleventh filing of the same calendar day.** Plan text and both
annotations above are unchanged; this block is the record.

**Commits, all reported** *(no agent in this project has ever run a git
command)*: **`8be1ed3`** (the Pass 4b filing committed, plus the
`iccce-cmm/src/lib.rs` §Status fix), **`70411dd`** / **`a36abaf`** /
**`6ea1b3d`** / **`812a215`** (the BPC core and **two red commits with
false green claims in their messages**, both corrected — see the session
log), **`46f16e8`** (the `--bpc` CLI), **`df3a233`** (the Pass 5
measurements). **What is verified is the working tree**: `bpc.rs` is
wired, `iccce transform` accepts `--bpc`, `tools/difftest/src/pass5.rs`
and `src/bin/pass5_report.rs` exist, `README.md` §16 and
`TOLERANCES.md` §3.5 are filled, and **`lib.rs`'s §Status is fixed** —
it now reads *"chains, opt-in [`bpc`] via `with_bpc`), [`bpc`] (Pass 5…)
… Still to come: compiled transforms (Pass 6)"* and lists `pub mod bpc;`
*(verified — read)*. **That closes a staleness this document reported
four times.**

#### ★ The done-when, answered exactly — it is MET, on terms

The clause is *"BPC on and off differ in the documented direction, and
match lcms2's BPC within tolerance."* Both halves, with their scope:

| Clause | Verdict | The measurement, and what it covers |
|---|---|---|
| **"differ in the documented direction"** | **★ MET, and with no tolerance at all** | The direction is a **sign with an algebraic proof**: `out − in = (Xd − Xs)/(Xi − Xs)·(Xi − X)`, whose second factor is `≥ 0` for any in-gamut PCS value, so the sign of the shift is the sign of `Xd − Xs` **at every point**. Graded **exactly 0,0** in both directions — no component rises in `PB → 0` (128 CMYK points, largest fall 4,304×10⁻² device = **3,5159 ΔE2000**), no `K` rises in `0 → PB` (213 RGB points). **NC-092, NC-097** |
| **"match lcms2's BPC within tolerance"** | **★ MET on the map, the direction and the pipeline** | **1,110 588×10⁻⁴** device (tol 2,5×10⁻⁴) converting *out of* the v4 fixture and **4,600×10⁻⁵** (tol 1×10⁻⁴) converting *into* it, against **both** of lcms2's arms; **1,262 374×10⁻² ΔE2000** (tol 5×10⁻²). **The BPC-OFF baseline was graded first on purpose** — 1,012 157×10⁻⁴ on the same points — so a residual that was there anyway cannot be attributed to BPC. **NC-089, NC-090, NC-091, NC-096** |
| *(the same clause, the part that is NOT covered)* | **★★ THE ESTIMATORS WERE NEVER DISCRIMINATED** | See below. This is not a caveat discovered afterwards; it was **derived from both sources before anything ran** |

**And a third thing the done-when does not ask for but Pass 5 has**: the
**map itself graded against the primary specification** — **1,110×10⁻¹⁶**
against ICC.1:2022 **6.3.4.3**'s printed equation over 1005 PCS values,
**3,331×10⁻¹⁶** against a Gaussian elimination on **Maria (2013)
§4.2**'s two published constraints over 20 000 random draws, and the two
constraints holding under iccce's own map to the same figure
(**NC-084 … NC-086**). Three independent statements of one map agreeing
to ~1,5 ulp.

#### ★★ The honest boundary — the estimators, and why the negative result is the Pass's best work

**BPC has three rules, each keyed on something different**: an
applicability set, an **estimation** method, and a forcing policy. Pass
5 grades the first and third and **cannot grade the second**, and it
knew that before it measured:

> **Everywhere iccce will do BPC at all, lcms2's estimator reduces to
> the same two values.** On a matrix/TRC or gray side, lcms2's guard 6
> darkest-colorant estimate is device black through the profile at a
> colorimetric intent — exactly iccce's `device_to_pcs(0)` — and on
> **every profile in reach that is exactly `XYZ (0,0,0)`, because every
> TRC in the corpus has `trc(0) = 0`**. On a v4 LUT side at perceptual,
> lcms2's guard 3 returns **the same A41 triple iccce hard-codes.**

**Why that matters more than the six small numbers it explains.** A
session that had measured first would have found six agreements and read
them as six independent statements about "BPC". **When two
implementations agree, the question is what they were free to disagree
about** — and here the answer, read out of their sources, is *"almost
nothing"*. Filed as **DL-023**.

**The instrument that would close it does not exist.** It is **a
synthetic v4 RGB-or-gray LUT fixture with a NON-ZERO device black**;
`fixtures/synthetic/` holds **38 `.icc` files** *(verified —
enumerated)* and the only v4 LUT among them is `v4-cmyk-mab-lab.icc`,
whose black is zero. **Owed to `tools/gen-profiles`**, and it is the
exact shape of the GP-001 arc (DL-020): a doubt the corpus cannot
discharge, discharged by bytes this project authors.

#### ★ Two recorded differences that are NOT defects, and neither may be rounded into the agreement figures

1. **★ lcms2 silently performs NO BPC below a threshold, and iccce
   deliberately has no such threshold.** `cmscnvrt.c`'s `IsEmptyLayer`
   sums the BPC matrix's deviation from the identity plus its offsets
   and **drops the entire stage below `0,002`** — so lcms2 stops doing
   BPC once the two black points are within roughly **0,41 `L*`**.
   **For the S2/S3 map the discriminant is 0,015 342, 7,7× the
   threshold**, so nothing measured here is affected by it. **It is
   READ, not RUN**: the 0,41 `L*` figure is a solution of lcms2's own
   inequality, not an observation, and it is recorded at that strength
   and no higher (**NC-088**). **The constant was not in the corpus** —
   `ICC_Spec` §7.2's list of unattributed constants was drawn from
   `cmssamp.c` and this one lives in `cmscnvrt.c`. A corpus row is owed.
2. **★★ iccce NEVER forces BPC; lcms2 forces it for a v4 destination at
   perceptual.** Same pair, same intent, **neither side asked**:
   **3,137 3×10⁻² device = 3,137 348 `L*`**, lcms2 lighter at black.
   **Neither implementation is wrong; the number IS the policy.**
   Handled per **DL-019** — **REPORTED, NOT GRADED**, because grading it
   would mean picking a winner without a clause, and the two available
   gradings (a ~3,2 `L*` tolerance chosen because it passed, or a
   permanent red line) were **both rejected in writing**. The corpus's
   **D11** watch is answered: 3,137 348 `L*` against the PRM black's
   3,137 254 and the A41 triple's 3,137 238 — **a match to 1,1×10⁻⁴**,
   and the **sign identifies lcms2's M2 route, not iccDEV's**, which the
   two directions distinguish. **This is a standing divergence, promoted
   to `ARCHITECTURE.md` DL-022**, and it is **user-visible**: two
   correct CMMs give different pictures by default. **NC-100**

#### ★ What is NOT verified — the coverage statement, so it cannot be rounded up

- **No black-point ESTIMATOR is tested**, by either side (above).
  **lcms2's methods 3 and 4 — the ink round trip and the least-squares
  quadratic fit — are untested against anything**, because iccce refuses
  there instead (**S5**, a **coverage gap, not a bug**: lcms2 answers,
  iccce does not, so **no comparison exists and Pass 5 claims none**).
- **The saturation intent.** lcms2 forces BPC there too; iccce's subset
  admits **only perceptual** for a LUT side, so that arm **has no iccce
  half**. This is the *second* Pass in a row whose remainder includes
  saturation, and the two are different items.
- **Any real v4 LUT profile.** S2 and S3 are **one synthetic fixture**;
  the 40-profile sweep's zero stands.
- **The gray side of iccce's own subset** — implemented, and no scenario
  exercises it, because every gray profile in reach would be another
  null.
- **Whether forcing is conformant**, which needs `AdobeBPC.pdf` / ICC
  WP40 / ISO 18619 — **operator downloads**; agent tools are ToS-barred
  or blocked.
- **Any published value for a BPC result.** There is none, for the same
  reason there is none for perceptual (**A27**). **This project still
  has no `published-ground-truth` row for any transform.**

#### ★ Gates — and this is the filing where a gate line has to be read carefully

- **Whole suite `pass=90 fail=0 skip=3 error=0`** *(reported;
  transcribed at the head of `tools/difftest/README.md` — **verified as
  text**)*. Pass 4b's whole-suite figure was **64**, so **Pass 5 added
  26 records** — *this librarian's subtraction of two reported totals,
  not a reported count*. It reconciles exactly with §3.12's row
  enumeration (5 + 7 + 8 + 6 = 26), which is why the enumeration is
  printed there.
- **★ §16 states no `pass=`/`fail=` line of its own**, unlike §15's
  `pass=28 fail=0` *(verified — §16 read end to end)*. The per-row
  outcomes are transcribed observations; **nobody has reported a Pass 5
  runner result as such.**
- **★ `tools/difftest/src/pass5.rs` carries NO `#[test]` declarations**
  *(verified — the whole of `tools/` grepped for `#[test]` with no
  result limit, and `pass5.rs` grepped again on its own)*. `pass3.rs`
  has 7, `pass4.rs` 7, `pass4b.rs` 8 — **the grids, the scenario set and
  the harness-side constants of Pass 5 are pinned by nothing.** NC-034's
  grid-count assertion has no Pass 5 analogue. Owed to
  `icc-conformance`.
- **★ `cargo test --workspace` STILL has no reported outcome** — and
  this is the filing where that stops being a formality, because **two
  commits this session claimed one falsely**. Independently checkable
  without a shell: **103 `#[test]` declarations across 18 files under
  `crates/`** *(verified — counted, no result limit; was 102 at the Pass
  4b filing)*. **A count of declarations is not the runner's count and
  not a pass result**; `cargo test` also counts doc-tests, and
  `tools/difftest` and `tools/gen-profiles` are **not** workspace
  members *(verified — `Cargo.toml` read)*. **NC-057 … NC-061 still have
  no reported outcome at all**, six filings on.

#### ★ A labelling correction to the dispatch, recorded rather than absorbed

The dispatch describes the lcms2 match as *"map 1.11e-16 vs its own
primary clause 6.3.4.3; **policy arm 4.6e-5**"*. **4,600×10⁻⁵ is not the
policy arm.** It is **NC-096** — iccce `--bpc` against lcms2, in S3,
with BPC on. **The policy row is NC-100 at 3,137 3×10⁻², and it is
REPORTED, NOT GRADED.** The two differ by a factor of ~680 and by their
*posture*, which is the whole point of DL-019. The ambiguity is
understandable — §16.4 is titled *"S3, the `0 → PB` direction, **and the
policy**"* — and it is corrected here rather than silently, because a
graded cross-check and an ungradable divergence must never be quoted
with each other's status.

#### What Pass 5 still owes

1. **★ A non-zero-black v4 LUT fixture** (`tools/gen-profiles`) — the
   only instrument that discriminates the **estimators**.
2. **★ The `icc-spec-librarian` dispatch on the forcing policy**: is
   BPC's *applicability* specified as a function of intent and version,
   or only its *black-point detection*? Blocked on an **operator**
   browser download.
3. **Two corpus rows**: the `IsEmptyLayer` **0,002** threshold beside
   `ICC_Spec` §7.2, and **A41's ΔE2000 = 0,050 201** (the corpus
   computed ΔE76 and ΔL* only, and ΔE2000 is the figure a perceptibility
   budget is stated in).
4. **Harness unit tests for `pass5.rs`**, and a reported runner result
   for §16.
5. **The saturation intent**, if iccce's estimation subset is ever
   widened to admit it — at which point S3 acquires a second arm and
   lcms2's forcing can be measured in a second place.
6. **The DL-014 tier question, now load-bearing on a ledger CLASS**:
   `bpc.rs` heads 6.3.4.3 **"PRIMARY-SOURCED"** while
   `icc__ref__bpc.md`'s `evidence:` line grades §2/§3
   **`cross_verified_2src`** *(both verified — read this session)*. See
   `NUMERIC_CLAIMS.md` §3.12.1.

### ★★ Pass 5 addendum — **the estimation step is SOURCED: `A42` upgraded on ISO/CD 18619:2013, and every unattributed lcms2 constant has a clause.** With a pre-registered prediction still awaiting measurement. Filed 2026-08-12 by `icc-librarian`

**This block belongs to Pass 5.** Commit **`0378f76`** *"cmm: ISO/CD
18619 black-point estimation — A42's practical consequence closed"*
*(hash and subject line corroborated by `.git/logs/HEAD`; contents not
verified)*.

**What the operator's download turned out to be.** The file fetched as
`_sources\BlackPointCompensation.pdf` is **ISO/CD 18619:2013, not
WP40** — WP40 is its own superseded ancestor. `bpc.rs`'s module header
records that identification and, with it, **a binding citation form**:
*"ISO/CD 18619:2013 clause 4.2.x", never "ISO 18619"* — because a
committee draft carries **normative language with non-normative
status**, and its own cover forbids the short form *(verified — read)*.
**That distinction is not decoration.** A row graded against a CD is not
graded against an International Standard, and writing the short form
would quietly upgrade every claim built on it.

**What it supplies.** The whole estimation procedure in `shall`
language. **Every threshold this project had been carrying as "an
unattributed lcms2 constant" is in clause 4.2 verbatim**: `0.2`, `≥ 4`,
the shadow windows `[0.1,0.5)` / `[0.03,0.25)`, the `L* ≤ 50` clamps,
256 ramp samples, the ±50 chroma clamp, `1.0E-10`, `max(0,min(50,·))`,
`n < 3` *(the list is in the module header — verified, read)*. `A42`
moves **UNVERIFIED → PARTLY RESOLVED** in the corpus register, and stays
*partly* on the CD-not-IS technicality alone *(verified — the register's
movement table read)*.

**What iccce implemented, and the three places ISO corrects Adobe.**
`estimate_lut_destination_black` follows 4.2.5: the **darkest-colour
SEARCH** (`darkest_vertex`, over 4.2.2.2's verbatim vertex sets) instead
of a fixed device black; the **root** instead of the vertex; and the
monotonic and validity guards. `neutralise_and_clip` implements 4.2.3 —
**always** neutral `(Li, 0, 0)`, where Adobe neutralised for CMYK only
*(all verified — read)*.

**★ Three constants have NO home in either document, and iccce does not
copy them**: lcms2's `L* > 95 → 0`, its `IsEmptyLayer` `0.002`
stage-drop (corpus **M6**, the threshold Pass 5 solved for at ≈0.41
`L*`), and an `n < 4` fitter guard that **contradicts both ISO's `3` and
lcms2's own caller** *(verified — read)*. Naming what was *not* adopted
is the same discipline as naming an approximation.

**★ A pre-registered prediction, awaiting measurement — and this is the
part to not round up.** The module header records, **before anything was
run** (DL-023): ISO 4.2.6 says the black points' `a*`/`b*` *"are
ignored"*, while **lcms2 retains chroma and propagates it per-channel**.
At input black the difference should equal exactly the detected
destination black's `√(a*² + b*²)` — predicted **2–6 ΔE76** for a `b*`
of −2…−6, decaying to zero at white, **on relative colorimetric with a
LUT destination**. **iccce follows ISO.** *(verified — read.)*

> **★ A predicted divergence is not a finding.** **DL-011** predicted an
> lcms2 disagreement and **DL-012** measured it **absent**. This
> prediction is unmeasured as of this filing; `icc-conformance` is
> reported to be measuring it in a parallel dispatch, **and no result of
> that run is recorded here, in either direction.** It gets a
> `NUMERIC_CLAIMS.md` row when it has a number, and not before.

**What this does NOT do.** It does **not** discharge **NA-009**'s cost.
Pass 5's negative result (DL-023) stands unchanged: **everywhere iccce
does BPC at all, lcms2's estimator reduces to the same two values**, so
**the two estimators still cannot be discriminated** by anything in
reach. **Sourcing an estimator is not measuring one.** The instrument
that would close it — **a synthetic v4 RGB-or-gray LUT fixture with a
non-zero device black** — still does not exist. And **A27 still
stands**: ISO gives the procedure, **not worked numbers**, so there is
still **no BPC conformance test with a published expected value**, and
this project still has **zero `published-ground-truth` rows for any
transform**.

### ★★★ Pass 5 addendum 2 — **the ESTIMATORS are discriminated, the negative result DL-023 pre-registered is closed, and the answer is that lcms2 has TWO estimators and the destination's header picks between them.** Filed 2026-08-12 by `icc-librarian`

**Pass 5's completion record and its first addendum are unchanged.**
Both stated the boundary honestly — *"the two ESTIMATORS were never
discriminated"* — and both said what instrument would close it. **This
block records that the instrument was built, that it closed the question
in a way nobody predicted, and that one of the two documents' predictions
about what the instrument would show was wrong.**

**Two runs, by `icc-conformance`**, apparatus
`tools/difftest/src/pass5b.rs` and `pass5c.rs`, records in
`tools/difftest/README.md` **§17** and **§19**, tolerances
`TOLERANCES.md` **§3.5.7** and **§3.5.8**. Ledger: `NUMERIC_CLAIMS.md`
**§3.17**, **§3.18**, rows **NC-129 … NC-144**, and a second dated note
on **NA-009**.

#### ★★★ The finding

`cmsDetectBlackPoint` **branches before** the darkest-colorant code every
previous reading here had stopped at (`cmssamp.c` **L370–374**): at
`INTENT_RELATIVE_COLORIMETRIC`, an **output-class profile in an INK
colour space** goes to `BlackPointUsingPerceptualBlack`, which
**forces `a* = b* = 0`**; **everything else** goes to
`BlackPointAsDarkerColorant`, which **keeps the chroma**.
`cmsDetectDestinationBlackPoint` then returns `InitialLab`'s `a`/`b`
verbatim — **so the branch IS the returned chroma.**

> **★★★ *"Does lcms2 keep its black point's chroma?"* has NO ANSWER. It
> has one answer for a CMYK press profile and the opposite for an RGB
> printer profile — and the only real LUT profile on this machine is the
> first kind.**

#### ★★ The two arms, and the prediction resolving in opposite directions

| | `USWebCoatedSWOP.icc` (v2.1 `prtr` **CMYK**) | `v4-rgb-mab-chromatic-black.icc` (v4.4 `prtr` **RGB**, ours) |
|---|---|---|
| lcms2 branch | `BlackPointUsingPerceptualBlack` | `BlackPointAsDarkerColorant` |
| ISO 4.2.5 black | `L* 16,489 806`, neutral | `L* 20,000 000`, neutral |
| lcms2 black | `L* 16,571 474`, **neutral** | `Lab(20 · 4 · −3)`, **chromatic** |
| **divergence** | **8,166 8×10⁻² ΔE76 — 100 % `L*`** | **5,000 000 ΔE76 — 100 % chroma, `ΔL*` exactly 0** |
| the corpus's **mechanism** claim | **FALSIFIED** | **CONFIRMED** |

⚠ **The synthetic arm's 5,0 is evidence for the MECHANISM and nothing
else** — that chroma is what **this project authored** into the fixture.
It lands inside the pre-registered **2–6 ΔE76** band by coincidence;
**the magnitude claim's falsification stands on the arm where the profile
was not ours to choose** (SWOP's darkest colorant is only **0,834** off
neutral, so **no** estimator reading that file could have produced a
number in the band).

#### ★★ What it cost the record, and what that cost bought

- **Pass 5b's 0,858 17 ΔE76 was 98,3 % APPARATUS.** Its lcms2 black was
  *recovered* through `A2B1 ∘ B2A1`; Pass 5c *reproduced* it from source.
- **Its *"mechanism CONFIRMED"* verdict is WITHDRAWN**; the structural
  half of the row — that **ISO 4.2.3 is implemented** — is untouched.
- **Its *"shape NOT ESTABLISHED"* was the correct call**, and is now
  settled: FALSIFIED on one arm, CONFIRMED on the other.
- **Its error bar of 0,813 7 against an effect of 0,858 17 was not an
  error bar; it was the measurement.** ★ **The row that made this
  findable is the row whose whole job was to doubt its own apparatus** —
  it came back at **0,948** against a limit of 1,0 and was reported as
  **marginal rather than green.**
- **★ Neither implementation fits a quadratic on either fixture.** Both
  take the 4.2.5.4 short-circuit, so **every Pass 5b statement about the
  shadow window, the sample count and the root describes code that did
  not run.**
- **★ An apparatus fault was caught by a second candidate**, not by
  reading: `transicc` prints RGB and gray as `0..255` and ink as
  `0..100`, and three Passes had divided everything by 100. **DL-028.**

#### ★ What this addendum does NOT do

- **It does not make Pass 5's grade stronger.** Pass 5 graded **the map,
  the direction and the pipeline**; that is unchanged. What is new is a
  **cross-check of the ESTIMATION step against a reimplementation of
  lcms2** — one implementation reproduced from its own source, at one
  pin, **not ground truth**.
- **It does not close the v4 PERCEPTUAL arm, and nothing can.** There
  **both** implementations return the fixed A41 constant **without
  reading the profile**. ★ **The instrument this document, `NEXT_SESSION`
  and NA-009 have all asked for three times over — "a v4 LUT fixture with
  a non-zero device black, to discriminate the estimators" — was built
  and discriminates the MEDIA-RELATIVE arm instead, because the
  perceptual one is a null by construction.** What the fixture makes
  newly measurable is **how wrong the A41 constant is** (`L* ≈ 3,1`
  against that device's real black of `L* 20`) — **owed, not made.**
- **★ It does not establish that iccce is right.** The `swop` divergence
  is **entirely** the 4.2.5.4 short-circuit's return value — **iccce
  returns `outRamp[first]`, lcms2 returns `InitialLab`** — and **which
  the standard specifies is dispatched to `icc-spec-librarian` and
  unanswered.** **If ISO names lcms2's, iccce is wrong and the code
  changes.**
- **It does not cover the saturation intent, the `bkpt` tag, any source
  but sRGB, a darkest colorant with chroma above 50, or any platform but
  Windows/MSVC.**

**Also closed by the same work:** the Pass 5b finding that
`bpc.rs`'s ISO estimator **had no caller** — the shipped binary refused
exactly the case ISO 4.2.5 exists for. Wired at commit **`c268261`**,
with a regression test (**NC-157**), and the graded row that used to
assert the *refusal* (**NC-136**) is superseded by one asserting the
wired path reaches **the same black** the library function does
(**NC-144**, at the CLI's own 10⁻⁶ print floor).

## Pass 6 — performance

Compiled transforms, caching, a benchmark on a page-sized raster. Only
now — optimising before Pass 4 is correct is how a fast wrong answer
gets locked in.

**Done when**: a 300 DPI A4 CMYK→RGB conversion completes in a stated
time, and the compiled path's error against the uncompiled one is
measured and stated.

> **Annotation, 2026-08-11 (`icc-librarian`, at the Pass 4b filing) —
> rule 8's precondition is now MUCH closer to satisfied, and that is
> what makes this Pass legitimately near rather than merely next in the
> numbering.** Plan text unchanged.
>
> Rule 8 says *optimise only after correct*, and this Pass's own text
> says optimising before Pass 4 is correct *"is how a fast wrong answer
> gets locked in."* As of today **every evaluation path this Pass would
> compile has been measured against another implementation in the
> direction it will be used**: `mft2` A2B (Pass 4), `mft1` B2A, the v4
> `mAB `/`mBA ` element pipeline, matrix/TRC both directions, and the
> F.2 grayTRC. **That is the condition rule 8 was actually asking for**,
> and it did not exist before Pass 4b.
>
> **Two things this Pass inherits that are not obvious from its plan
> text:**
>
> - **Its done-when's second clause already has a template.** *"The
>   compiled path's error against the uncompiled one"* is a
>   **`self-consistency`** row (`NUMERIC_CLAIMS.md` §1) — worthless as
>   correctness evidence and to be labelled so however small it comes
>   out. The pattern to copy is **DL-018**: an upper bound on a
>   *deliberate* cost needs a **prediction pin** and a **sensitivity
>   control**, or deleting precision makes the gate greener. A compiled
>   path is exactly that shape.
> - **★ A grid or cache built for one direction says nothing about the
>   other.** **DL-021** is the rule, and Pass 4b is the instance: the
>   interpolation-method cost is 1,5741 ΔE2000 in A2B and **zero** in
>   B2A on the same profile. Any Pass 6 claim about a compiled
>   transform's error must name **which direction and which tag type**
>   it was measured on, and must not be generalised across the pair.
>
> **What is NOT satisfied**: Pass 4's done-when is still open at the
> ICC-absolute intent (**A4b**, operator-blocked) and at saturation in
> B2A; and **no ground-truth row exists for any transform in this
> project.** Optimising is defensible on measured cross-checks; it does
> not become defensible on ground truth that has never existed.

> **★ Second annotation, 2026-08-11 (`icc-librarian`, at the Pass 5
> completion filing) — the precondition is now met across the whole
> transform surface, including the one stage that was added after Pass
> 4b said so.** Plan text unchanged.
>
> The annotation above listed the evaluation paths a compiled transform
> would compile and said all of them had been measured. **BPC is a
> stage such a transform would fold in** — lcms2 folds it in as a single
> matrix between two stages it already had — and as of Pass 5 it too is
> measured, in **both** directions, against lcms2 **and** against a
> clause. **So Pass 6 is the first Pass in this project whose "correct
> first" precondition (rule 8) is satisfied for every stage it would
> touch.**
>
> **Three things it inherits, and the third is new:**
>
> - **DL-018** — an upper bound on a deliberate cost needs a prediction
>   pin and a sensitivity control.
> - **DL-021** — a compiled path measured in one direction says nothing
>   about the other.
> - **★ DL-023** — before grading "the compiled path agrees with the
>   reference path", state **what the two were free to disagree about**.
>   A compiled transform built by *sampling the reference path* is the
>   sharpest case of this in the whole project: the two arms can be
>   identical by construction over the sampled set, and a
>   `self-consistency` row that is null by construction is worth
>   nothing. **Pass 5's cheap instrument transfers directly**: state the
>   **sensitivity ratio** — how much the compilation *could* move the
>   answer, beside how much the two arms differ (Pass 5's were **388×**
>   and **682×**).

### ★★ Pass 6 completion record — the done-when MET with two numbers, and the Pass's real work was making the second number MEAN anything. Filed 2026-08-12 by `icc-librarian`

**The first filing of a second calendar day** (the previous eleven were
all 2026-08-11). Plan text and both annotations above are unchanged;
this block is the record.

> **★ A dated note on this filing's own date.** The dispatch that
> commissioned it was headed **2026-08-11**. It is **2026-08-12**:
> `.git/logs/HEAD` timestamps `bb5d6b8` at epoch `1786527689 -0400` =
> **2026-08-12 05:41:29 −04:00** and the three commits after it between
> 06:20 and 06:55 the same morning *(verified — read)*; the environment
> reports the date as 2026-08-12; and the corpus's ambiguity register
> carries `revised: 2026-08-12` *(verified — read)*. **Every Pass 6 and
> Pass 7 record in this project is dated 2026-08-12**, and the dispatch's
> header is corrected rather than followed. It matters more than it
> looks: eleven filings saying *"the same calendar day"* is a claim about
> this project's history, and a twelfth would have made it false.

**Commits.** **`3502cb7`** *"Pass 6: compiled transforms + the A4
benchmark — done-when measured"*. **★ For the first time, a commit hash
in a Pass record is corroborated by something in this repository rather
than reported alone**: `.git/logs/HEAD` line 44 records exactly that
hash and that subject line *(verified — read)*. **What is still not
verified is the commit's CONTENTS.** What *is* verified is the working
tree: `crates/iccce-cmm/src/compiled.rs` exists and is the type
described below, and `crates/iccce-cli/src/main.rs` carries `cmd_bench`
with the argument parsing, the timing and the twelve output lines quoted
below *(both verified — read end to end)*.

#### ★ The done-when, answered exactly — it is MET

The clause is *"a 300 DPI A4 CMYK→RGB conversion completes in a stated
time, and the compiled path's error against the uncompiled one is
measured and stated."*

| Clause | Verdict | The measurement, and its scope |
|---|---|---|
| **"a 300 DPI A4 CMYK→RGB conversion completes in a stated time"** | **★ MET** | **8 700 867 pixels** (2481 × 3507 — A4 at 300 DPI, and the constant is in `cmd_bench` where a reader can check the arithmetic) in **7.23 s** = **1.20 Mpix/s**, on **this machine**, **release** build. Grid build — **83 521 chain evaluations**, 17 points per axis over 4 input channels — **1.04 s**, paid once. Reference path **0.084 Mpix/s** over the same raster, timed in the same process; **speedup 14.4×**. **NC-105, NC-106, NC-107** |
| **"the compiled path's error against the uncompiled one is measured and stated"** | **★ MET, and the error is `self-consistency`** | **0.003589 device units**, maximum over **off-node** probes, on **SWOP `A2B1` (`mft2`, 4-D) → sRGB matrix/TRC, media-relative, 17-point grid**. Both arms are iccce, so per `NUMERIC_CLAIMS.md` §1 this is **worthless as correctness evidence however small**, and `cmd_bench` prints that sentence in its own output rather than leaving it to a reader. **NC-108** |

**Every figure above is `icc-engineer`'s report of an `iccce bench` run
on this machine.** `icc-librarian` has no shell and ran nothing. **The
raw `iccce bench` output block is not on record anywhere** — see the
transcription-precision note in `NUMERIC_CLAIMS.md` §3.13.2, which is
the one place these numbers are weaker than they look.

#### ★★ The Pass's real work: the control that caught its own instrument, twice

The done-when's second clause is the shape **DL-018** exists for, and
**DL-023** predicted its failure mode by name at the previous filing: a
transform compiled by *sampling* its reference path is *"the most likely
null-by-construction row this project will ever write."*

**The prediction was right and the trap was walked into anyway.**

1. **The control's fixture nullified it.** `error_scales_with_grid_spacing`
   was first written on **sRGB → sRGB**. A grid does not merely match an
   identity chain at its nodes — **n-linear interpolation is exact on a
   linear function, so it matches it everywhere.** Result: **1.1×10⁻¹⁵,
   ratio 0.94, no `h²` scaling.** A magnificent number measuring
   nothing, and **without the control it would have been reported as the
   compiled path's cost.** Refixtured to **sRGB → AdobeRGB**, whose
   differing TRCs make the composite genuinely curved.
2. **Then the scaling law did not match the function.** The refixtured
   control probed the whole axis and got **ratio 1.44** against an `h²`
   prediction of 4. **Neither the code nor the fixture was wrong**:
   sRGB's TRC joins a linear segment to a power curve at `0.04045`, and
   interpolation error across a derivative discontinuity scales `h¹`.
   The probes now sit in **`[0.2, 0.9]`**, off-node for both the 5- and
   9-point grids, where `h²` is the right prediction; the accepted band
   is **2×–8×**.

**Both failures are recorded in the test's own doc comment** *(verified
— read)*, which is where the next person to change the fixture will see
them. Filed as **DL-025**, together with the observation that this is
the **third** instrument in two days to catch something a competent
engineer was about to ship — after **DL-016** (the off-by-one-sample
curve, where the self-consistency round trip *would have passed*) and
**DL-020** (GP-001, refused an hour before it was found).

#### ★ The three inherited rules, and how each is discharged in the record

| Rule | How Pass 6 discharges it |
|---|---|
| **DL-023** — say what the two arms were free to disagree about | **At a grid node they were free to disagree about NOTHING** — the node's value *is* a reference evaluation. `tests::identical_at_nodes_by_construction` asserts that identity and is labelled **"STRUCTURAL, NOT EVIDENCE"** in its own doc comment *(verified — read)*; it exists to catch a transposed indexing convention and for no other reason. **Every error number in this Pass is off-node**, in the unit tests and in `iccce bench` alike, and `cmd_bench`'s doc comment says why *(verified — read)*. Filed as **NC-110**, ungraded on purpose |
| **DL-018** — an upper bound on a deliberate cost needs a prediction pin and a sensitivity control | The control is `error_scales_with_grid_spacing` and it is the subject of the section above. **It is the only reason NC-108 is a measurement rather than a decoration**, and its own two failures are the evidence that it can fail. **NC-109** |
| **DL-021** — a behaviour is a fact about one direction and one path | **Named in every row.** NC-108 is **SWOP `A2B1`, `mft2`, 4-D → sRGB matrix/TRC, media-relative**; NC-109 is **sRGB → AdobeRGB matrix/TRC**. **Neither says anything about the B2A direction of either pair**, and the module header states that in as many words *(verified — read)* |

#### ★ One measurement that was thrown away, and it belongs in the record

**A first attempt at the reference timing measured the CLI end to end
and reported ≈49 000 px/s.** That figure is **stdio text parsing**, not
either transform — the `transform` subcommand reads and writes decimal
text per pixel. The reference path is now timed **in-process**, over a
bounded prefix of the *same* raster, so the comparison is
transform-versus-transform. `cmd_bench` carries the reason in a comment
*(verified — read)*.

**Why a discarded number is worth a paragraph.** ≈49 k px/s and 84 k
px/s are the same order of magnitude. **A speedup quoted against the
wrong denominator would have been ~24× instead of 14.4×, and nothing
about it would have looked wrong** — which is project rule 1 wearing a
stopwatch. The recorded figure is the one whose denominator is a
transform.

#### Coverage — part of the claim, and it is narrow

- **One machine.** Windows 11 Pro 10.0.26200 x86-64, MSVC **release**.
  **No timing has ever been taken on any other machine, on any other
  platform, or in a debug build**, and a throughput figure is a fact
  about hardware, allocator and build flags before it is a fact about
  iccce. **Still no Linux run of anything, by anyone, ever.**
- **One run.** No repetition, no variance, no confidence interval, no
  warm-up policy stated. **A single wall-clock sample.**
- **One direction, one tag-type pair, one intent, one grid density.**
  CMYK `mft2` A2B → RGB matrix/TRC at media-relative, 17 points per
  axis.
- **Two real profiles, both from this machine's colour directory** —
  `USWebCoatedSWOP.icc` and the stock sRGB. Neither is committed
  (LEGAL §3), so **every Pass 6 measurement skips on a machine without
  them**, exactly as the unit tests do.
- **No comparison against any other implementation.** **lcms2 was not
  timed.** Nothing here says iccce is fast *relative to anything except
  its own reference path*, and no row may be quoted as if it did.
- **BPC is not in the compiled chain that was benchmarked** (the bench
  builds a plain `Chain` at media-relative). A compiled path *can* fold
  BPC in — that is what made rule 8's precondition interesting — but
  **it was not exercised here.**

#### What this Pass does NOT claim

- **Not that the compiled path is correct.** `self-consistency` compares
  iccce to iccce. **A grid built by sampling a wrong reference path
  reproduces the wrong answer to 0.003589 device units.**
- **Not that 0.003589 is a perceptual quantity.** It is device units,
  and **no ΔE2000 translation of it has been measured.** Converting it
  by intuition would be exactly the move DL-004 exists to forbid.
- **Not that the 0.02 gate in the unit test is the measurement.** The
  gate is a bound chosen to survive the CLUT's curvature while failing
  on a grid-indexing error; **the reported number is `iccce bench`'s.**
- **Not that any test passed.** **No `cargo test --workspace` outcome
  has been reported at any of the last seven filings.** 116 `#[test]`
  declarations now exist across 19 files under `crates/` *(verified —
  counted, no result limit; 103 across 18 files at the previous
  filing)*. **That is a count of declarations. It is not coverage and it
  is not a pass result.**

#### Owed by this Pass

1. **★ `icc-conformance` — `TOLERANCES.md` §3.6.** It exists with **two
   placeholder rows and every cell blank** *(verified — read; its file,
   its call)*, and its two rows are stated in **ΔE2000**, which is a
   unit **nothing in Pass 6 has measured**. Either the rows get a ΔE
   translation of NC-108 or they get re-stated in device units.
2. **★ The raw `iccce bench` output block, pasted somewhere.** Three
   figures were transcribed at three precisions and their ratio does not
   quite reproduce (§3.13.2). The evidence is twelve lines of text and
   nobody has filed them.
3. **A repeat run, and a second machine.** One wall-clock sample is the
   whole timing record.
4. **A ΔE2000 figure for the compiled path's cost**, so the
   approximation can be priced in the unit the project's budgets are
   stated in.
5. **A compiled path measured in the B2A direction** — DL-021 makes it a
   separate question, not a generalisation.
6. **A compiled chain WITH BPC folded in**, which is the configuration
   Pass 5 made legitimate and Pass 6 did not exercise.

### ★★ Pass 6 addendum — **the gate was graded, it FAILED, and the engineer changed the GRID rather than the number. It now passes against the identical tolerance.** Filed 2026-08-12 by `icc-librarian`

**The completion record above is unchanged.** It closed Pass 6 on a
done-when that asked for **measurements**, and it got them; what it did
not have was a **tolerance**, because `TOLERANCES.md` §3.6 was still two
placeholder rows with every cell blank — which that record listed as the
Pass's first owed item. **This block records what happened when the
tolerance arrived.**

**Filled and run by `icc-conformance`** (`tools/difftest/src/pass6.rs`,
`TOLERANCES.md` §3.6, README §18). **Re-graded the same day** after
commit **`189e732`** *"Pass 6 gate: default grid 17 -> 33, because the
number would not move"* *(hash and subject corroborated by the reflog;
`compiled.rs`'s `recommended_grid_points` verified as `3 => 33`,
`_ => 33`, **with the failing 17 recorded in the constant's own doc
comment**)*.

> **★★ Dated citation correction, 2026-08-17 (supplementary filing).**
> The parenthetical above is **true as a 2026-08-12 read and false as a
> statement of current state**, and it is not edited — this note is the
> correction. **`_ => 33` no longer exists as code**; it was removed on
> 2026-08-17 after being measured to make `recommended_grid_points(7)`
> return 33, i.e. `33⁷ × 3 × 8 ≈ 952.6 GiB`, which **aborted the
> process** (`NUMERIC_CLAIMS.md` §3.33 / **NC-234**). The only surviving
> occurrences in `crates/` are **doc comments describing its removal**
> *(verified — grepped at the tip)*. ★ **The 3-D and 4-D `33` are
> unchanged and are now asserted in `compiled::tests` as *measured
> values***, so this Pass's grading stands.

#### ★★ The tolerance, and why it could not be moved

**`2,5×10⁻¹` ΔE2000 — which is Pass 4's measured iccce-vs-lcms2 figure
on this exact pair (0,252 94) to one significant figure.** Derivation:
**compiling must not move the result further than the two
implementations already differ on the same transform.** **No headroom,
no safety multiple, no anchor, and nothing in it to tune.** ★ **The
rejected derivation is on record beside it and is worth as much**: *"an
order of magnitude below §2's 1,0 perceptibility anchor"* presumes the
engine's approximations sum below the anchor, **and NA-006 alone was
measured at 1,574 ΔE2000 on `A2B0` of this same file** — a budget
derived from a total already exceeded.

| | grid 17 (the then-default) | **grid 33 (the default since `189e732`)** |
|---|---|---|
| ΔE2000 max, 513 bench probes | **FAIL 2,970 17×10⁻¹** | **PASS 1,677 3×10⁻¹** |
| ΔE2000 max, Pass 4's 341-point grid | **FAIL 2,962 90×10⁻¹** | **PASS 9,348 6×10⁻²** |
| device max (reported, ungraded) | 3,588 962×10⁻³ *(the **0,003589** the completion record carries)* | 2,012 444×10⁻³ |
| build | 1,06 s | **~14 s** |

#### ★★ Four things this transition put on the record

1. **A red suite was the CORRECT state.** §0's procedure was followed and
   stopped at step 1: the code was not wrong, no expectation was
   involved, and the fixture was the benchmark's own. **The suite was red
   because a shipped default did not meet a justified line** — which is
   what a conformance suite is for.
2. **★★ At grid 33 the two probe populations STOP AGREEING.** At 17 they
   were within **0,25 %** of each other, which is what licensed the
   sentence *"the failure is a property of the transform, not of a probe
   set"*. At 33 the bench figure is **1,79×** the Pass 4-grid figure,
   because once the error is small enough **probe placement dominates**.
   **Both are inside the line; quoting either alone is now a POPULATION
   CLAIM.**
3. **★★ The green has a price and it is reported nowhere else:
   `iccce bench`'s break-even moves from ≈70 000 px to ≈1,19 million
   px** — a **17×** increase. **Compiling now pays for itself only on
   large rasters**, and a speedup quoted without its build cost is a
   materially incomplete claim.
4. **★ The apparatus row caught the default moving.** `pass6.rs`'s grid
   constant must track the shipped one; when it did not,
   `harness-reproduces-bench` failed at **1,576×10⁻³** — *not an error,
   but the gap between two grids' costs*. **A cheap row that fails
   loudly when the two arms stop describing the same transform is worth
   more than an expensive one that averages over it.**

#### ★ And the control was re-derived — the `h²` justification was FALSIFIED

DL-018's sensitivity control had asserted `h²` and never recorded its
**passing** ratio. It now grades a **paired median at the same probe**
(not a max-of-max, which wanders by a factor of 4 as *which* probe is
worst moves) and observes **2,69 · 2,47 · 2,51** across three octaves —
**convergence order 1,32, not 2**. ★ **Doubling the grid costs ~15× the
build and buys ~2,5× the accuracy, not 4×**; anyone budgeting a default
from an `h²` intuition will overestimate refinement badly. ★ **A clamp
attribution was written, tested and falsified**: restricting to cells
entirely in gamut and above sRGB's breakpoint changed the ratios **not
at all**.

**`NUMERIC_CLAIMS.md` §3.19**, rows **NC-145 … NC-152**, with §3.19.1 on
exactly what this does to §3.13's rows: **NC-108 is not deleted and not
edited** — it is a correct measurement *at grid 17* — and **NC-147 is
the same quantity at today's default.**

## Pass 7 — named colours and spot

`namedColor2Type`. The Pass `pdfce` is waiting for, because it is what
makes `Separation` and `DeviceN` colorimetric rather than approximated.

> **Annotation, 2026-08-11 (`icc-librarian`, at the Pass 4b filing) —
> this Pass's core is ALREADY IN THE TREE, undispatched and unmeasured.**
> Plan text unchanged.
>
> `crates/iccce-cmm/src/named_color.rs` exists, is declared in the
> crate's `lib.rs`, and carries **2 `#[test]` declarations** *(verified —
> read and counted 2026-08-11)*. It builds a `NamedColors` table from an
> `ncl2` tag and decodes its PCS coordinates through
> `pcs_encoding::LabEncoding::Legacy`, citing **10.17 verbatim** — *"this
> tag uses the legacy 16-bit PCSLAB encoding … not the 16-bit PCSLAB
> encoding that is defined in 6.3.4.2"* — and **Table 66** (*"Only
> PCSXYZ and legacy 16-bit PCSLAB encodings are permitted. PCS values
> shall be relative colorimetric."*), corpus **A26 RESOLVED**.
>
> **Three things to carry forward, none of them optional:**
>
> 1. **`NamedColors` is referenced by nothing outside its own file**
>    *(verified — grepped)*. The consumer exists; **no transform reaches
>    it, and no comparison has ever touched it.**
> 2. **The legacy-encoding rule here is the one DL-005 exists for.**
>    Getting it wrong costs **~0,4 % in `L*`** — below any ΔE gate's
>    notice and above an exact-value test's, and the module doc names
>    the stake: *"spot colours are brand matching … the least acceptable
>    place in the whole system for a sub-perceptual defect."* **Assert it
>    with exact-value integer invariants, never with ΔE.**
> 3. **The `ncl2` behavioural test this project has owed since Pass 2 is
>    still owed** — `NUMERIC_CLAIMS.md` NC-019's coverage line still
>    rests on a **source reading**, and a consumer existing does not
>    change that. The module also records a **normativity mismatch** for
>    a future validator: legacy `L* > 100` is *"shall not"* in 10.10 and
>    *"should not"* in 10.17 (corpus spec-defect §4).

### ★★ Pass 7 completion record — the spot colour reaches a real destination through the ORDINARY machinery, and the "reachable from nothing" finding this document filed TWICE is closed. Filed 2026-08-12 by `icc-librarian`

**The second filing of 2026-08-12.** Plan text and the annotation above
are unchanged; this block is the record.

**Commits.** **`40cf384`** *"cmm: named colours — Pass 7 core, the
pdfce-facing piece"* (2026-08-11, the core this document's annotation
found in the tree undispatched) and **`f6203b8`** *"Pass 7: named
colours wired — the pdfce-facing path is reachable"* (2026-08-12).
**Both hashes and subject lines are corroborated by `.git/logs/HEAD`**
*(verified — read)*; **`f6203b8` is the current tip of `master` and of
`origin/master`** *(verified — both ref files read)*. Contents are not
verified. What *is* verified is the working tree: `named_color.rs`
carries `resolve_to_device`, `transform.rs` carries
`convert_pcs_to_device` and `pcs_to_destination`, and `Chain::convert`
calls the latter *(all verified — read)*.

#### ★ The Pass has no numeric done-when, and that is worth saying out loud

**Pass 7's plan text states no done-when clause** — unlike Passes 1–6
and 9 — so **there is no clause to declare MET.** This record therefore
does not say *"the done-when is met"*; it says **what was built, what
was measured, and what was not**, and lets a reader judge. Writing a
done-when retrospectively and then satisfying it would be the same move
as tuning a tolerance.

**What "done" means here**, stated plainly so it can be argued with:
`namedColor2Type` is decoded (Pass 4 era), and **the operation `pdfce`
actually needs — resolve a spot name to a destination's device values —
exists, is reachable, and is exercised against real bytes.**

#### ★ What was built, and the single design decision that carries the Pass

`NamedColors::resolve_to_device(name, dst)` →
`Chain::convert_pcs_to_device(dst, pcs)` →
`Chain::new(dst, dst, MediaRelative)` → **`Chain::pcs_to_destination`**.

**The decision is the last arrow.** `pcs_to_destination` is *the same
method `Chain::convert` uses for its own destination half* — the
destination arm was **de-duplicated in this commit**, and `convert` now
calls the shared method with the comment *"ONE destination
implementation, shared with the PCS-side entry point a named colour
uses: a spot colour that took a different path from every other
conversion would be exactly the kind of quiet divergence this project
exists to avoid"* *(verified — read)*.

**Why that is the whole Pass.** A spot colour is the **least acceptable
place in the system for a sub-perceptual defect** — spot colours are
brand matching, and the module says so. The failure mode that a private
resolution path invites is not a crash; it is a `Separation` that
renders 0.4 % off from every other object on the page, **on some
profiles only**, and looks fine. Routing through the ordinary machinery
means the spot inherits, by construction: the **sourced 8.10.2
fallback** for choosing a destination model, the same B2A / `mAB ` /
matrix / gray selection, the same refusals, and the same clamping. **It
cannot drift from the rest of the CMM without the rest of the CMM
drifting too.**

**Three further properties, each a decision rather than an accident:**

1. **Media-relative BY CONSTRUCTION, with no intent parameter at all.**
   Table 66: *"PCS values shall be relative colorimetric."* So the spot's
   colorimetry **is** the source, no source model is built, and **no
   intent choice arises on the source side** — there is nothing for a
   caller to get wrong. *(verified — the reasoning is in the doc
   comment.)*
2. **An unknown name returns `None`, not a guess.** In PDF that is
   precisely the signal to fall back to the `/Alternate` space. **It is
   not an error and not a substituted colour** — the CMM's form of "the
   parser reports, it does not repair" (**DL-020**: refuse by name,
   never substitute).
3. **The legacy PCSLAB encoding, asserted by exact integer invariants
   and never by ΔE.** `0xFF00/0x8000/0x8000` must decode to
   `Lab(100, 0, 0)` **exactly**; the wrong (v4) decode gives **99.6109**
   — *"sub-perceptual, invisible to ΔE, fatal to a brand colour"*
   *(verified — read)*. This is **DL-005** and **DL-016** doing the job
   they were written for.

#### ★★ The finding this document filed TWICE is closed

The Pass 4b annotation recorded, and the Pass 5 filing repeated, that
`NamedColors` **was referenced by nothing outside its own file** — *"the
consumer exists; no transform reaches it, and no comparison has ever
touched it."*

**`tests::spot_colour_resolves_into_a_real_destination` is the first
thing outside `named_color.rs` to touch it** *(verified — read)*. It
reads the committed fixture `fixtures/synthetic/v2-ncl2-named.icc`,
parses the **real system sRGB profile** as the destination, and resolves
**every spot in the table**, asserting each output channel lies in
`[0.0, 1.0]`; then asserts that an unknown name yields `None`. **NC-111,
NC-112.**

**What that test is and is not.** It is a **behavioural
`self-consistency`** check: nothing outside iccce supplied an expected
value. **It is not a cross-check** — `transicc` was not run on a spot
colour, and no ledger row compares iccce's spot resolution to anything.
**It is not a ΔE claim**; it asserts range, not colour. **What it does
establish** is the thing that was missing: the path executes end to end
on real bytes and produces device values a caller could use.

#### Coverage — part of the claim, and narrower than "Pass 7 is done" sounds

- **One fixture**, and this project authored it. `v2-ncl2-named.icc` is
  `tools/gen-profiles` output. **No real vendor `ncl2` profile has ever
  been parsed by anything here** — no PANTONE library, no press
  profile's spot table.
- **One destination, and it is a matrix/TRC one.** The system sRGB
  profile. **The LUT (`B2A`/`mBA `) and gray destination arms of
  `pcs_to_destination` are reachable from a spot colour and have never
  been exercised from one**, which is the same shape of hole Pass 4b
  found on the gray axis.
- **The `Lab ` arm only.** Table 66 permits **PCSXYZ and legacy 16-bit
  PCSLAB**; `NamedColors::from_ncl2` implements both, and **only the Lab
  branch is exercised** by the fixture.
- **No `nDeviceCoords` comparison.** An `ncl2` entry carries the
  vendor's *own* device values alongside its PCS coordinates. **Nothing
  compares iccce's resolved device values against a profile's stored
  ones** — which, on a profile whose own device space is the
  destination, would be a genuine cross-check and is the cheapest one
  available.
- **The `ncl2` behavioural test owed since Pass 2 is STILL OWED.**
  **NC-019's coverage line still rests on a source reading of lcms2**,
  and a consumer existing does not change that. Do not let this Pass
  tick that item off.
- **No comparison against lcms2 on any part of this path.**

#### Owed by this Pass

1. **★ A cross-check that has an outside expectation in it.** The
   cheapest: resolve a spot into **the spot's own profile** and compare
   against the entry's **stored `nDeviceCoords`** — an expectation
   written by whoever authored the file, not by iccce.
2. **★ A spot resolved into a LUT destination and into a gray
   destination**, which are reachable today and unexercised.
3. **A PCSXYZ `ncl2` fixture**, so the second permitted encoding is not
   carried on a source reading.
4. **The Pass 2-era `ncl2` legacy-Lab behavioural test** (`NC-019`),
   still owed, now for the fourth filing.
5. **A `pdfce`-side consumer**, which is Pass 8 and lives in another
   repository.

## Pass 8 — the pdfce bridge

Built **in pdfce**, not here. `ICCBased`, output intents, and replacing
the `/Alternate` fallback with a real conversion.

### ★★ What remains, as of 2026-08-12 — the original scope is essentially complete, and the next real step is in another repository

**Filed by `icc-librarian` at the Pass 6 + Pass 7 filing.** This block
replaces nothing; it is the first time this document has stated the
*whole* remaining picture in one place, because it is the first time the
picture is small enough to fit.

**Where every Pass stands** *(and every "DONE" carries the coverage
statement in its own completion record — none of them mean "verified",
and none of them mean "verified against ground truth")*:

| Pass | Status |
|---|---|
| **0** scaffold + oracle | **DONE** (2026-08-11) |
| **1** colorimetry | **DONE** — the project's only `published-ground-truth` row (NC-001, all 34 Sharma pairs) lives here, and it is about ΔE2000, **not about a transform** |
| **2** profile parsing | **DONE** |
| **3** matrix/TRC transforms | **DONE** |
| **4** LUT transforms and intents | **★ STILL OPEN — two items, and only one of them is still blocked.** See below |
| **5** black point compensation | **DONE on stated terms** (the estimators were never discriminated — DL-023), and its sourcing is now upgraded (the Pass 5 addendum above) |
| **6** performance | **DONE** (2026-08-12) |
| **7** named colours and spot | **DONE** (2026-08-12) |
| **8** the pdfce bridge | **NOT STARTED, and it is built in `pdfce`, not here** |
| **9** HDR (BT.2100) | **NOT STARTED.** Blocked on ITU-R documents entering the corpus, and *before that* on `icc-spec-librarian` establishing that `itu.int`'s terms permit retrieval (DL-007). **"The file is free" has never implied "automated retrieval is permitted"** — DL-002 exists because that exact inference was available and wrong |
| **10** profile creation | **NOT STARTED, far-future, and its precondition is unsolved**: nothing has been chosen as a ground truth that is not iccce |

#### ★ Pass 4 is the only original Pass still open, and its shape changed today

Its done-when needs two things Pass 4b left unmeasured:

1. **Saturation in the B2A direction** (`B2A2` is a distinct third
   table). **Cheap, unblocked, and nobody has run it.** Not the same
   item as Pass 5's saturation gap, which is a *capability* gap in
   iccce's BPC estimation subset.
2. **The ICC-absolute intent through a LUT destination.** **★ This item
   is no longer operator-blocked.** `A4b` is **RESOLVED** by the
   operator's `ICC.1:2001-04` download (Pass 4 addendum above), and it
   resolved by the clause being **silent on readers**. What follows:
   **the arithmetic can be measured now**, and whether the raw
   comparison stays **REPORTED, NOT GRADED** under **DL-019** is a
   judgement `icc-conformance` must now actually make rather than defer
   — *"the authority does not exist"* was true yesterday and is not true
   in the same way today. **A4c** (the residue: no clause requires a
   profile's `wtpt` to agree with its colorants) is **SILENT**, and
   **does not clear when A4b clears**.

#### ★ Pass 8 is the real next step, and it is not this repository's work

`ARCHITECTURE.md` §4 fixes the boundary and it does not move: **a thin
bridge crate *in `pdfce`*, and `iccce` must not know what a PDF is.**
The bridge maps `/ICCBased` streams to `iccce_profile::Profile`,
`/Separation` and `/DeviceN` tint transforms to named-colour lookups,
and PDF/X `/OutputIntent` to a destination profile.

**What Pass 7 handed it**, precisely: `NamedColors::resolve_to_device`,
returning **`None`** for an unknown name — which is exactly the
`/Alternate` fallback signal a PDF consumer needs, and is deliberately
not an error. **What this repository owes the bridge and has not
delivered:** a spot resolved into a **LUT** destination (a press profile
is the normal `/OutputIntent`, and it is a LUT profile), and any
cross-check at all on the spot path.

**One consequence worth stating before the bridge is written:** a PDF
consumer will hand iccce **real-world profiles at scale**, including
malformed ones. Rule 6 — *the parser reports, it does not repair* — has
never been exercised by a caller that must keep going. **Nothing here
knows what `pdfce` will do with a reported malformation**, and that is a
`pdfce`-side design question this document should not answer for it.

#### ★ The tail items — what is owed here regardless of Pass 8

These are not a Pass. They are the standing debts, and **the first two
are the two largest holes in the project**:

1. **★★ No `published-ground-truth` row exists for any transform.** Not
   one, across Passes 3, 4, 4b, 5, 6 and 7. Every transform claim is a
   cross-check against lcms2, a derived expectation, or
   self-consistency. **The cheapest route remains `IEC 61966-2-1`'s sRGB
   primaries, and nobody has dispatched for it** — for the seventh
   consecutive filing.
2. **★★ No `cargo test --workspace` outcome has ever been reported.**
   Not at any of the last seven filings. **116 `#[test]` declarations
   exist across 19 files under `crates/`** *(verified — counted, no
   result limit)*; **that is a count of declarations, not coverage and
   not a pass result.** **NC-057 … NC-061 still have no reported
   outcome at all.** And this project has already shipped **two commits
   whose messages claimed a green suite while a test was red**.
3. **The DL-014 citation audit**, never swept for `iccce-color` or
   `iccce-profile`, which now **decides a ledger class** (NC-084) *and*
   underwrites a **published** compliance claim (DL-024's third
   pre-publication check).
4. **A Linux run of anything at all.** Still nothing, by anyone, ever —
   and the project is now public, where "works on Windows" is a
   narrower claim than a reader will assume.
5. **The non-zero-black v4 LUT fixture** (`tools/gen-profiles`), the
   only instrument that can discriminate the two black-point estimators
   and therefore the only route to **NA-009**'s cost.
6. **`pass5.rs` has no unit tests**, so fourteen ledger rows rest on two
   grids that nothing pins.
7. **crates.io**: **name availability is still unchecked** by anyone,
   and `THIRD_PARTY_LICENSES.md` via `cargo-about` is still owed before
   a first publish. **A public git repository is not a published
   crate** (DL-024).

#### ★★ Dated update, 2026-08-12 (later the same day) — **Pass 4 is CLOSED, so the table above now reads 0–7 DONE.** Three of the seven tail debts moved, and two NEW ones outrank most of the list

**The block above is not edited**; this is how it is corrected. Its Pass 4
row (*"STILL OPEN — two items"*) and both numbered items under it are
**superseded by the Pass 4 completion record**. **Both items are
measured.** ★ Item 1 was **already measured when that block was
written** — see the nine-site sweep in this document's header; the
proximate cause was the Pass 6/7 filing's deliberate decision not to read
`tools/`.

**Where the seven tail debts stand:**

| # | Debt | Status |
|---|---|---|
| 1 | **No `published-ground-truth` row for any transform** | **UNCHANGED, and now for the EIGHTH consecutive filing.** Pass 4c adds cross-checks, not ground truth. **`IEC 61966-2-1` is still the cheapest route and still nobody has dispatched for it.** **This is now unambiguously the largest hole in the project** |
| 2 | **No `cargo test --workspace` outcome ever reported** | **★★ DISCHARGED, first time in seven filings: exit 0, 121 passed, 0 failed** (63 + 25 + 33 across three test binaries), plus `fmt --check` and `clippy -D warnings` clean on the root workspace *(reported, gated on `$?` — the mechanical gate DL-024 asked for)*. **Scope, honestly: a workspace-wide pass count is not per-row confirmation**, and *"121 passed"* is no more an inventory than *"116 declarations"* was |
| 3 | **The DL-014 citation audit** | **STILL OWED, and it stopped being hypothetical.** §3.15.7 found a **live** defect — the *"D.6/D.7"* label names the **informative** annex and is **not edition-stable**. Second consecutive filing to find a citation naming the right words in the wrong place |
| 4 | **A Linux run of anything** | **UNCHANGED. Nothing, by anyone, ever** |
| 5 | **The non-zero-black v4 LUT fixture** (NA-009) | **STILL OWED — and it now has a companion.** **NA-008's second arm is blocked on a PCSLAB gray fixture that has never been written either.** **Two named approximations, two unwritten fixtures, one crate** |
| 6 | **`pass5.rs` has no unit tests** | **Carried, and labelled `unverified-this-filing` rather than `owed`** — another agent is editing `pass5*.rs` right now. **The two are different claims and only one is safe to act on** |
| 7 | **crates.io** | **UNCHANGED. Name availability still unchecked; `THIRD_PARTY_LICENSES.md` still owed** |

**★★ Two NEW debts, and the first outranks everything above it:**

- **★★ `dechk.obj` IS IN THE PUBLIC REPOSITORY.** A 5 933-byte **MSVC
  COFF object file** at the repository **root**. **Tracked; added by
  commit `aef7566`; and `aef7566` is an ancestor of `origin/master`**
  *(all verified — `git ls-files`, `git log --diff-filter=A`,
  `git merge-base --is-ancestor` run)*. **`.gitignore` has no `*.obj` or
  `*.o` rule** *(verified — read)*. ★ **This is the same shape as
  `edce48b`** — which `NUMERIC_CLAIMS.md` §2.6 records as *"untracked
  in-progress `tools/gen-profiles` swept in by `d9e0b82`'s cwd-relative
  pathspec — a process slip"* — **same root directory, same mechanism,
  and this time the push is already done.** Owed to `icc-engineer`: a
  `*.obj`/`*.o` rule, removal, **and a decision about history**. It is
  small and benign, but ***"benign" is a judgement the operator makes
  about a published artefact, not one an agent makes for him.***
- **★★ EIGHT pushes to `origin/master` exist, not two.** DL-024 records
  two (06:51:17, 06:54:50). The reflog now holds **eight**, the last at
  **08:19:21 −04:00**, and **`origin/master` == `HEAD` == `95c04c1`**
  *(verified — read and run)*. **Nothing in any document records a
  go-ahead for pushes three through eight**, and rule 9 plus DL-024 both
  say publishing is the operator's act and *"he said yes on the 12th"*
  is not standing permission. **Recorded as an observation, not an
  accusation** — the reflog attributes them to `KenM76`, the operator
  may have run or authorised every one, and **no file records
  authorisation either way.** It needs **confirming, not assuming**.

**And the standing debt this filing itself created:** ★ **the Pass 4c
work is UNCOMMITTED** — `pass4c.rs` untracked, the CLI help fix
modified-not-committed — so **sixteen ledger rows are anchored to a
working tree.**

---

#### ★★★ Dated update, 2026-08-12 (latest) — **the original scope is COMPLETE AND FILED, and what remains is four kinds of thing, none of which is "unfinished Pass work"**

**Neither block above is edited.** The Pass table still reads **0–7
DONE** and that is unchanged; what this note adds is the **shape** of
what is left, because *"the scope is complete"* is the sentence most
likely to be read as *"the project is finished"*, and it is not the same
claim.

**Four kinds, and the distinction is the point:**

| Kind | What is in it | Why it is not merely unfinished work here |
|---|---|---|
| **1. In another repository** | **Pass 8**, the `pdfce` bridge | `ARCHITECTURE.md` §4 fixes the boundary and it does not move: **a thin bridge crate *in `pdfce`*, and `iccce` must not know what a PDF is.** No amount of work in this repository advances it past what Pass 7 already handed it |
| **2. Blocked on something nobody here can produce** | **A `published-ground-truth` row for any transform** (needs **IEC 61966-2-1**, purchased or licensed); **A31** (needs `ICC.1:2010-12` v4.3); **the adaptation ground truth** (needs ICC's published D65→D50 `chad` values); **Pass 9's precondition** (ITU-R terms, then the documents) | These are **acquisitions, not tasks**. ★ **A31 is now the register's only UNVERIFIED row**, and no amount of reading the corpus resolves it — *the document does not exist in reach* |
| **3. Operator scope calls already answered but not yet started** | **Pass 9** (HDR — in scope by DL-007) and **Pass 10** (profile creation — in scope by DL-008) | Both were **added by operator decision** and neither has been sized. **Pass 10's precondition is unsolved and is not an engineering backlog item**: nothing has been chosen as a ground truth that is not iccce, and *"round-tripping a profile through its own inverse is the canonical test whose expected value came from the code under test"* |
| **4. Standing debts of the work already done** | The tail-debt list above, as re-scored below | These are the ones a session **can** act on today |

**Where the tail debts stand at this filing** *(the seven-row table two
blocks up is not edited; this is its successor)*:

- **1 — no `published-ground-truth` row for any transform: UNCHANGED,
  NINTH consecutive filing, and it now has a decoy beside it.** §3.20
  adds two metrics that agree with lcms2 to **ten decimals** and are
  **`impl_crosscheck`**; §3.18 adds a **reimplementation** of lcms2.
  **Neither is ground truth, and the ten-decimal one looks the most like
  it.**
- **2 — a `cargo test --workspace` outcome: LAST REPORTED AT THE
  PREVIOUS FILING** (exit 0, 121 passed, at commit `95c04c1`), **and one
  commit has landed since.** Checkable without a shell: **121 `#[test]`
  declarations across 19 files** *(verified — counted)*. ★ **The two
  121s are different quantities and their agreement is a coincidence.**
- **3 — the DL-014 citation audit: STILL OWED**, and `delta_e.rs`'s new
  **CIE 116-1995** and **BS 6923** citations are a cheap new surface for
  it *(both are marked UNSOURCED at the site, which is the correct
  form)*.
- **4 — a Linux run: STILL OWED.** ★ **CI is now REPORTED to have run
  and passed**, which is the nearest this project has come — **and it is
  a report, with no run URL, no summary, and no statement of whether the
  Linux job was among what passed.** **Do not let it silently discharge
  this.**
- **5 — the non-zero-black v4 LUT fixture: BUILT, and it answered a
  different question** (see the Pass 5 addendum 2 above).
  **`fixtures/synthetic/` now holds 39 `.icc`** *(verified —
  enumerated)*. **The PCSLAB gray fixture (NA-008's second arm) is still
  unwritten.**
- **6 — `pass5.rs` has no unit tests: carried as
  `unverified-this-filing`**, not as *owed* — `tools/` was read here only
  at README §17–§19 and `TOLERANCES.md`.
- **7 — crates.io: UNCHANGED.** Name availability **still unchecked**;
  `THIRD_PARTY_LICENSES.md` still owed. ★ **The API surface was
  deliberately narrowed for publication (DL-029) and that is not an
  authorisation either.**
- **`dechk.obj`: still present at the repository root** *(verified — the
  tree enumerated)*, and **its cause is now known** — it is the object
  file of the C probe that produced ΔE94/CMC's expected values. **Its
  tracked status was shell-verified at the previous filing and could not
  be re-checked here** (this session had **no shell**; §2.11).
- **Pushes: NINE now, not eight** *(verified — the reflog read)*, the
  last carrying **`5cfee171`**, which is also the tip of `master` and of
  `origin/master` *(both ref files read)*. **Nothing records a go-ahead
  for pushes three through nine.** ★ One push is **reported** to have
  failed with **HTTP 408** and to have been retried over HTTP/1.1 — **a
  failed push leaves no reflog line**, so the failure is a report and
  only the success is evidence.

**★★★ And one question is open, dispatched, and capable of making a
shipped behaviour wrong:** does **ISO/CD 18619 4.2.5.4**'s mid-range
short-circuit return **`outRamp[first]`** (iccce) or **`InitialLab`**
(lcms2, `cmssamp.c` L536)? **Dispatched to `icc-spec-librarian`
2026-08-12.** It is the whole of the `swop` arm's 8,167×10⁻² ΔE76.
**If ISO names lcms2's, iccce is wrong — not divergent — and the
engineer changes the code.**

> **★★★ ANSWERED the same day, and it was capable of making a shipped
> behaviour wrong because it DID.** ISO/CD 18619 4.2.5.4's final
> paragraph specifies **`InitialLab`**; **lcms2 conformed and iccce did
> not**; the code is corrected at commit **`fd34a44`** *(verified —
> `bpc.rs` read at the tip: the straightness branch now returns
> `initial_lab` unchanged, with the clause quoted verbatim beside it)*.
> The paragraph above is left standing because **the shape of the
> question is the record** — it was posed in the direction that could go
> against us, and it did. `ARCHITECTURE.md` **DL-030**;
> `NUMERIC_CLAIMS.md` **§3.24**, **NC-164**.
>
> **Two dated corrections to the four-kinds list above**, neither of
> which edits it:
> - **The remainder is smaller by one dispatched question and larger by
>   one crate.** `iccce-measure` (commit `2a2d616`) is a **fifth**
>   workspace member and Pass 10 pre-work — see the Pass 10 section.
> - **★ Every bare test count in the block above is unusable as
>   written.** The *"121 `#[test]` declarations across 19 files"* figure
>   is superseded by **129 across 20 files** *(verified — re-counted in
>   `crates/`, the increase being `iccce-measure`'s eight)*, and the
>   `cargo test --workspace` outcome that had no result is now **129
>   passed, 0 failed, exit 0**. ★ **The declaration count and the pass
>   count agree exactly, per crate** (cmm 63, profile 33, color 25,
>   measure 8, cli 0), which corroborates that **no declared test was
>   skipped or ignored** — and it is still **a count of declarations,
>   not of coverage** (§1.2). **The `142` in commit `d5efd96`'s message
>   is a CONFORMANCE-RUNNER record count and is not comparable to
>   either** — DL-031.

### ★★★ Pass 8 — RETROSPECTIVE entries for two capabilities that shipped with NO ROADMAP ENTRY. Filed 2026-08-17 by `icc-librarian`

**Why this subsection exists, stated plainly:** the two items below were
**built, tested and measured before this document said they were
planned.** `NEXT_SESSION.md` §0 had listed them among *"four gaps this
project now knows about and has NOT filed anywhere else … **none has a
`ROADMAP.md` entry**"*. They now have implementations, so they get
retrospective entries — **a completion record with no plan above it is
still better than a capability with no record at all**, and pretending
they were planned would be worse than both.

★★ **Both are properly Pass 8 material even though Pass 8 is built in
`pdfce`**, because both are things **this repository owes the bridge**.
Neither is the bridge. `ARCHITECTURE.md` §4's boundary is untouched:
**iccce still does not know what a PDF is.**

★★★ **Both were requested, in substance, by the consumer.** They came out
of `pdfce`'s `open/request_iccbased_colour_spaces.md` and the `/N`
validation need — **which is the request channel doing the one thing
`CLAUDE.md` rule 10.1 says it is for**, and the strongest available
evidence that a bidirectional channel is worth its overhead.

#### ★★ Retrospective 1 — the built-in sRGB destination. **DONE 2026-08-17 (uncommitted)**

**What it is:** `iccce_cmm::builtin::srgb()` plus
`Destination` / `DestinationProvenance` / `Chain::with_destination` /
`Chain::destination_provenance` in `crates/iccce-cmm/src/transform.rs`.
**Contract, conditions and build order are in
`docs/DEFAULT_DESTINATION.md`**, whose **STATUS: BUILT** block records
the four things measurement changed.

**Done-when, judged MET on these terms:**

| Clause | Status |
|---|---|
| Constructed from published constants, **no file, no blob, no dependency** | **MET** — BT.709-6 items 1.3/1.4, W3C CSS Color 4, ICC.1:2022 Annex E.3 |
| **No lcms2 in the lineage** | **MET** — this is the clause the whole item was blocked on, and `illuminant.rs`'s D65 comment is corrected to match (DL-051) |
| *"Doesn't exist"* means **absent**, never **unresolved** | **MET** — **DL-050**, a two-variant enum rather than `Option` |
| The fallback is **disclosed, not silent** | **MET** — `DestinationProvenance::note()` names the constants and says *"This is NOT the document's declared output intent"* |
| Tested by **ΔE round-trip, not byte-equality** against a shipped profile | **MET** — NC-221/NC-222, and `NEXT_SESSION.md` §3.2's explicit prohibition on byte-equality was honoured |
| The blue-`Z` difference **named as a rule-4 approximation** | **MET** — ~12 ULP, and `builtin.rs` **asserts the residual stays in `11.0..13.0` ULP**, so a change to the construction turns it red |

> ★★★ **CORRECTED THE SAME DAY, 2026-08-17 (supplementary filing). The
> row immediately above is WRONG IN BOTH HALVES and is left standing as
> the record of what was judged.**
>
> 1. **The ~12 ULP residual is the FILE's, not iccce's.** ICC's own
>    published D50-adapted colorants (Holm/ICC 2015 §B.2, 15 dp) put
>    **iccce's construction at 3.02 ULP worst / 0.90 in `bXYZ.Z`** and
>    **the shipped HP 1998 / `sRGB2014.icc` file at 11.13 ULP**
>    (`NUMERIC_CLAIMS.md` §3.33, **NC-230**/**NC-231**; **DL-054**).
> 2. **The `11.0..13.0` assertion no longer exists.** It was replaced by
>    `matches_icc_published_colorants_within_stated_ulps`, a **4 ULP**
>    bound against ICC's published values with a **discrimination
>    assertion** that fails if the bound ever stops telling our
>    construction apart from the file's *(verified — read at the tip;
>    `11.0` appears in `builtin.rs` only as `11.0 / 255.0` in the 8-bit
>    breakpoint test)*.
> 3. ★★★ **And the clause was judged MET against a DOC COMMENT while the
>    register was EMPTY.** *"Named as a rule-4 approximation"* is
>    discharged by an entry in `NUMERIC_CLAIMS.md` §4 — the register of
>    named approximations — and **§4 carried nothing for the constructed
>    sRGB at all**, running NA-001 … NA-010 with no entry for it. **Now
>    registered as NA-011**, with the measured 3.02 ULP, its cause, and
>    what it is not. ★ **A doc comment explains an approximation; the
>    register is what makes it findable**, and only the register can
>    answer *"what does iccce approximate?"*
>
> **Judged MET on the corrected terms**, with the evidence class
> **improved**: the clause is now discharged against
> `published-ground-truth` rather than against a file.

★★★ **What is NOT met, and it is not a defect — it is the evidence
class.** NC-221/NC-222 are **`constructed-vs-reference-file`**: the
reference is third-party but **the machinery on both sides is ours**.
**Nothing about the `0.033013` establishes that either side is
colorimetrically right** (`NUMERIC_CLAIMS.md` §3.32.1). ★ **And the
margin must be quoted correctly** — the binding probe (white) passes with
**exactly the 5 % headroom**, not the 37 % that applies to the non-binding
probes (§3.32.9a).

★★ **Coverage:** one reference file, ten probes, one direction, **matrix/
TRC path only. No LUT destination, no CMYK, no non-default intent, no
`f32` path.** DL-021 binds every sentence about it.

**What it does NOT deliver, and what a consumer must not infer:** it does
**not** make adopting iccce an *accuracy* gain for `pdfce` (DL-044/DL-047
— the case is **conformance**, and this changes nothing about that), and
it does **not** supply a destination for the CMYK output-intent path,
which is a LUT profile.

#### ★★ Retrospective 2 — the colour-space signature accessor (`/N`). **DONE 2026-08-17 (uncommitted)**

**What it is:** `crates/iccce-profile/src/colour_space.rs` —
`components()`, `channel_agreement()`, `is_valid_pcs()`, with
`ComponentCount` and `ChannelAgreement`; exported from the crate root and
surfaced by `iccce inspect` as `colorspace.components:`.

**Why it is not the same as `Chain::input_channels()`**, which already
existed: that reports the **tag's** count and needs a **built chain**.
`pdfce` needs the **header signature's** count **before** building, to
validate a PDF `/N`. ★ **And when header and tag disagree, that is itself
worth disclosing** — hence `channel_agreement()`.

**Done-when, judged MET:**

| Clause | Status |
|---|---|
| Sourced, **not written from memory** (rule 2 / §5.1) | **MET** — `icc-spec-librarian` from **ICC.1:2022 7.2.6 Table 19**, transcribed with **two independent PDF text engines** and matched against ICC's `icProfileHeader.h` and lcms2's `lcms2.h`: **four routes, no disagreement**. Corpus file `ICC_Spec/icc/icc__s__colour_space_signatures.md` |
| A public `Signature → count` accessor | **MET** |
| A header/tag cross-check | **MET** |
| Exercised on real profiles | **MET** — **60**, zero unrecognised, zero PCS-field violations (NC-220) |

**★★★ The three findings that outlast the code:**

- **A48 — ICC.1:2022 is SILENT on header/tag channel agreement.** The
  only `shall`-level count agreements in the document are
  `colorantOrderType` (10.4) and `colorantTableType` (10.5). ★★
  ***"Silent" is a different claim from "requires agreement"*, and the
  distinction was asked for explicitly** — so iccce **discloses** rather
  than declaring non-conformance. That is **DL-020's discipline applied
  to a SEVERITY** rather than to a rule.
- **A50 — the count is a TWO-TABLE JOIN, not a transcription.** Table 19
  has **no component-count column**; for the eleven named spaces the
  count exists only in **Table 41**, read off by **counting non-dash
  cells** (`GRAY` = 1 rests on one `K` and three dashes). **ICC.1
  publishes no `Signature → count` map at all** — cite it as **derived**,
  under DL-014's terms.
- **★ The trap that costs conformance:** treating the header PCS field as
  a two-value enum `XYZ `/`Lab ` **rejects every conformant DeviceLink
  profile** — **7.2.7** says a DeviceLink's PCS *shall* be a data colour
  space from Table 19. Tested.

★★ **A deliberate divergence from lcms2, asserted so it cannot regress:
`cmsChannelsOf()` returns `3` for an UNRECOGNISED signature; iccce returns
`Unknown(sig)` and refuses to guess.** ★★★ **Evidence class, and it must
travel with the claim: this was READ FROM lcms2 SOURCE at the pinned
commit and NOT EXECUTED** — `impl_crosscheck` **by inspection**, weaker
than every other lcms2 statement in this project, all of which came from
`transicc` runs.

★ **The hazard made concrete:** of the **1 020** single-byte corruptions
of `'CMYK'`, **exactly one lands on another valid signature — `'CMY '`,
which is 3 components.** ★★ **That number is MEASURED but NOT ASSERTED**
— it reaches a `println!`, and the test's in-loop assertion compares a
call with its own result and **cannot fail** (`NUMERIC_CLAIMS.md`
§3.32.8, §7.18 newly-owed 2). **The behaviour is protected; the
enumeration is not.**

#### ★ Still not filed, and now down to two

`NEXT_SESSION.md` §0's list of four unrecorded gaps is **half
discharged**. Remaining:

1. **The `f32`/`u8` evaluation surface.** Still `f64`-only, so an 8 Mpix
   CMYK page is **256 MB in / 192 MB out** through `convert_buffer`.
   ★ **This is the API finding a real consumer produced that no amount of
   internal review would have**, and it still has no entry.
2. **The four unexercised CMYK print profiles**, including
   `ISO Coated v2 300% (ECI)` — the `DestOutputProfile` of *every*
   ICC-CMS Ghent patch.

★ `ChainError`'s missing `std::error::Error` — the third item on that
list — **is discharged** (DL-052), and it was never cosmetic: without it
a caller had to hand-wrap or `.ok()` the result and **lose the named
refusal**.

---

## Passes added 2026-08-11 by operator decision

Passes 9 and 10 were added after Ken answered the open scope questions
below. **No existing Pass was renumbered**, so the numbers here are
**filing order, not schedule order** — Pass 9's dependency position is
stated in its own section. See `ARCHITECTURE.md` **DL-007** (HDR) and
**DL-008** (profile creation), both of which record exactly what the
operator said and label the interpretation as the engineer's.

## Pass 9 — HDR: BT.2100 transfer functions and wide-gamut primaries

**Dependency position: after Pass 7, independent of Pass 8.** It needs
the colorimetry of Pass 1 and the transform machinery of Passes 3–4; it
needs nothing from the `pdfce` bridge and the bridge needs nothing from
it. Do it before, after, or alongside Pass 8 as convenient. It is
numbered 9 only because 8 was already taken and renumbering a shipped
plan destroys the ability to read older records against it.

**In this Pass:**

- The **PQ** and **HLG** transfer functions of the ITU-R BT.2100 family,
  forward and inverse.
- **BT.2020 / BT.2100 primaries** and the matrices they imply, alongside
  the sRGB/Adobe RGB/Display P3 set that Pass 3 already handles.
- A **stated, measured** mapping between these encodings and the ICC PCS.

**Explicitly NOT in this Pass**, so the boundary is a decision rather
than a thing that grows: tone mapping, gamut-mapping algorithms, dynamic
metadata (ST 2094 and relatives), and any invention of new rendering
intents. Each is a separate call nobody has made.

**The hard part is not the curves.** ICC's PCS is media-relative and
derives from reflective print; PQ is an **absolute** encoding tied to
luminance in cd/m², and HLG is **scene-referred** with a
display-dependent OOTF. Connecting either to a PCS requires a stated
choice about reference luminance and about what counts as white. That is
an approximation under project rule 4: **name it and measure what it
costs in ΔE**, in `NUMERIC_CLAIMS.md`, or it is indistinguishable from a
bug. (This paragraph describes the shape of the problem. It is **not**
sourced from the ITU-R documents — nobody in this project has read
them.)

**Corpus precondition — this Pass cannot start without it.** Tier 3 of
`D:\Dev\Rag-Specialized\ICC_Spec\` must hold the relevant ITU-R
recommendations first. They are **reported** to be freely downloadable
from `itu.int`, described as a legitimate route. **That is a claim about
a third party's terms and it gets checked before anything is fetched** —
by `icc-spec-librarian`, by reading ITU's actual terms of use. "The file
is free" does not imply "automated retrieval is permitted"; DL-002 exists
because exactly that inference was available at color.org and would have
been wrong.

**Rule 2 applies at full strength here.** Not one PQ or HLG constant may
be written from memory. Every coefficient cites a sourced corpus file
naming its document.

**Done when**: PQ and HLG round-trip within a stated numeric tolerance
against **published** reference values (not against our own inverse); a
BT.2020 primary set converts to and from XYZ within a stated tolerance;
and the PCS-mapping choice is written down with its ΔE cost measured and
filed in `NUMERIC_CLAIMS.md`.

## Pass 10 — profile creation (far-future; scope reversed 2026-08-11)

**Scope status:** profile creation was *"out of scope, deliberately"* in
`README.md` with the rationale *"that is a profiler, a different product,
and it needs measurement hardware to validate."* **Ken reversed that on
2026-08-11.** It is now future scope. Full record, including what the
operator actually said and what is the engineer's reading of it:
`ARCHITECTURE.md` **DL-008**.

**Position:** after the `pdfce` bridge. **Sized and planned when
reached** — this section is a placeholder with one precondition attached,
not a plan.

**The precondition, which is the whole difficulty.** The old rationale
was an engineering fact, not a preference, and reversing the scope did
not make it false:

> A profiler whose output cannot be validated against physical
> measurement is project rule 1 in its worst form. lcms2 cannot be the
> oracle here — it can only confirm that a profile we wrote is parseable
> and self-consistent, which is exactly the reassurance a *wrong* profile
> would also produce. Round-tripping a profile through its own inverse is
> the canonical test whose expected value came from the code under test.

**Before any profile-creation work is called correct, the project must
state how its output is validated, naming a ground truth that is not
iccce.** Candidates, none chosen and none investigated: published
characterisation datasets carrying both measurement data and a reference
profile; an actual spectrophotometer; or a deliberately reduced scope.

**Worth separating now, because the two will be conflated otherwise:**
writing **synthetic** profile bytes whose intended contents are known by
construction — which `tools/gen-profiles/` in `ARCHITECTURE.md` §1
already implies, for fixtures — needs no measurement and was never out of
scope. The thing that was refused is **profile creation from measurement
data**. Pass 10 should be sized against that distinction.

### ★★ Pass 10 PRE-WORK landed 2026-08-12: `iccce-measure`, the CGATS/IT8.7 reader. Filed by `icc-librarian`

**Commit `2a2d616`** — *"iccce-measure: CGATS/IT8.7 reader — Pass 10
pre-work, no hardware needed"*, **authorised by the operator on
2026-08-12** *(reported by `icc-engineer`; the authorisation is the
operator's word and this librarian did not observe it)*. **The crate
exists in the build** *(verified — `Cargo.toml`'s `[workspace] members`
lists `crates/iccce-measure`, and the crate's own manifest and
`src/lib.rs` were read at the tip)*.

#### Why pre-work is available at all, when the Pass itself is blocked

**The precondition above blocks exactly one claim** — *"this profile
describes that printer"* — and that claim belongs to the **fitting**
half. Everything upstream of it is parsing, colorimetry and fitting
arithmetic, **none of which needs an instrument**. So the Pass splits
cleanly:

| Half | Needs hardware? | Status |
|---|---|---|
| **Read the measurement file** | **No.** Text in, structure out. | **`iccce-measure`, landed 2026-08-12.** |
| **Turn measurements into a profile, and validate it** | **Yes**, or a named ground truth that is not iccce. | **Still blocked**, on exactly the terms above. **Nothing here weakens that.** |

★ **This is the distinction the section already drew, used.** *"Writing
synthetic profile bytes whose intended contents are known by
construction … needs no measurement and was never out of scope"* — the
reader is the same argument applied one stage earlier. **It does not
move the boundary; it builds up to it.**

#### What the crate is

**Purpose:** read the CGATS/IT8.7 text files a spectrophotometer
produces and a profiler consumes — a header of `KEYWORD value`
properties, a `BEGIN_DATA_FORMAT` field list, and a table of measured
patches.

**INVARIANT: no ICC, and no colour maths** *(verified — read; stated in
both the crate manifest's header and `lib.rs`'s module doc)*. Two
consequences, and the second is the one that will matter later:

- **Its tests never need an ICC fixture.** Eight `#[test]` declarations,
  all text-in/structure-out *(verified — counted in `src/lib.rs`)*.
- **A future profiler and a future measurement tool can share it.** A
  measurement file is not a profile; had the reader gone into
  `iccce-profile`, the tool that only wants to *look at a target* would
  have had to depend on an ICC parser.

**Dependencies: none** *(verified — `[dependencies]` is empty but for a
comment)*. That makes **three of five** crates with an empty
dependency section, and per `ARCHITECTURE.md` §1 adding one here is an
architectural change rather than a convenience.

**Surface, as shipped** *(verified — read)*:

```
parse(text: &str) -> Result<MeasurementSet, ParseError>

MeasurementSet { properties, fields, rows, issues }
    .field_index(name) .column(name) .spectral_fields()

Value  ::= Number(f64) | Text(String)
Issue  ::= FieldCountMismatch { row, expected, actual }
         | NumberOfFieldsDisagrees { declared, counted }
         | NumberOfSetsDisagrees   { declared, counted }
         | UnterminatedBlock       { block }
         | DataBeforeFormat        { line }
```

#### ★ The `issues` vector is rule 6, applied to measurement data

**The parser reports; it does not repair.** A `NUMBER_OF_FIELDS` that
disagrees with the `DATA_FORMAT` block is **disclosed and never
corrected**; the declared fields win, because they are what the columns
actually are, and the disagreement is recorded rather than resolved. A
short row is kept as parsed beside a `FieldCountMismatch`.

**Why this matters more here than in an ICC parser, not less.** A
malformed profile usually fails visibly somewhere downstream. A
measurement file with one column too few **fits**: every value is a
plausible number, the profiler builds, and the error is delivered as
*colour*. **A silently repaired measurement file is rule 1 arriving
through the front door** — and the only layer that could have disclosed
it is this one.

#### Licence lineage, which is the part that must not be got wrong later

- **Derived from lcms2's `cmscgats.c`** for structure and keyword
  vocabulary. **lcms2 is MIT — the same licence as this project — so it
  is a permitted lineage**, on the same `impl_crosscheck` terms as every
  other implementation-derived piece of work here.
- **★ Argyll CMS is AGPL-3.0 and must NEVER be read or cited for this
  work.** It is by far the most tempting reference in this subject area.
  The prohibition is recorded at the crate site *and* here *and* in
  `ARCHITECTURE.md` §1 because the temptation recurs and a single
  mention is not a guard.
- **CGATS.17 itself is paywalled and is NOT sourced.** Where lcms2's
  reader is more permissive than the standard may be, this follows
  lcms2 **and says so** — which is a corpus gap, not a resolved
  question.

#### What this landing does NOT do

- **It does not start Pass 10.** The Pass's precondition — *state how a
  created profile's output is validated, naming a ground truth that is
  not iccce* — is **untouched, unaddressed and still the whole
  difficulty**.
- **It does not make a colour claim of any kind.** No value in this
  crate has been compared to anything. Its eight tests are structural,
  and **no `NUMERIC_CLAIMS.md` row derives from it** — deliberately, as
  there is nothing yet to grade.
- **It does not source spectral interpretation.** Turning
  `SPECTRAL_NM_380 …` into XYZ needs observer colour-matching
  functions, which are **Pass 1's third and last remainder item** and
  are not obtained. The crate returns spectral columns as numbers and
  stops.
- **It does not imply hardware is nearer.** The blocked half is blocked
  by the same sentence it was blocked by on 2026-08-11.

---

## Publication — crates.io (standing intent, not a Pass)

**Answered 2026-08-11: yes, publication to crates.io is intended.**
Recorded as `ARCHITECTURE.md` **DL-009**.

> **The intent is not an authorisation.** Rule 9 is unchanged: nothing
> may be pushed, tagged, released or published without an **explicit
> current go-ahead from Ken at the time.** "We decided in August that
> we'd publish eventually" is not that go-ahead, and no agent may treat
> this section as one.

Practical consequences, cheapest if done early:

- **Crate-name availability on crates.io is unchecked.** `iccce`,
  `iccce-color`, `iccce-profile`, `iccce-cmm`, `iccce-cli` — nobody has
  looked. Discover a squatted name before the API is public, not after.
  crates.io names are effectively permanent and are not released by a
  yank.
- **Manifest metadata must be complete and true** on every publishable
  crate: `description`, `license = "MIT"`, `repository`, `keywords`,
  `categories`, `readme`, `rust-version`. The declared `repository`
  is `https://github.com/KenM76/iccce`; **whether that remote exists has
  never been checked by anyone** (see question (a) below).
- **`THIRD_PARTY_LICENSES.md` via `cargo-about`, before the first
  publish.** It matters more here than usual: DL-001 means lcms2 is in
  the workflow while not being a dependency, so the licence story needs
  to be legible rather than trusted.
- **The oracle must stay out of the published artefact.**
  `tools/difftest` is not a workspace member and `vendor/` is
  git-ignored. That was engineered for licence insulation; it now serves
  publication too, and a future "tidy-up" that folds difftest into the
  workspace would silently undo both.
- **Publishing sets an API-stability expectation** the project has not
  earned yet. The natural first publish follows a Pass whose numbers are
  on the record in `NUMERIC_CLAIMS.md` — the ledger is what lets a
  stranger trust the crate.

---

## Ghent compatibility — **a standing workstream, NOT a Pass** (opened 2026-08-17)

**STATUS: OPEN, with a first measurement and no accuracy claim.**

★ **Why this is not a Pass.** Every Pass in this document is sized to be
finishable and has a done-when that can be met. **This has neither.**
There is no state in which "Ghent compatibility" is *done*: the suite is
a corpus, not a specification, it supplies **no numeric criterion**
(DL-047), and the one claim that would close it — *"passes the Ghent
suite"* — **cannot be made in public without GWG's written permission**
and is therefore not this project's to declare. Filing it as a Pass
would create a done-when that could only ever be met by weakening it.

**Origin:** the operator's instruction of 2026-08-17 (**DL-045**),
handed over with the suite itself.

**Where the material lives, and none of it is duplicated here:**

| What | Where |
|---|---|
| The reasoning, licence analysis, patch-by-patch boundary against `pdfce`, and per-statement provenance | **`docs/GHENT_COMPATIBILITY.md`** (nine sections) |
| The measured rows, their evidence classes, and what they do not claim | **`docs/NUMERIC_CLAIMS.md` §3.30**, rows **NC-192 … NC-199** |
| The two new evidence classes | **`docs/NUMERIC_CLAIMS.md` §1** — `fixture-declared-categorical`, `acceptance` |
| The posture, the promotion rule, and the no-numeric-expectation bound | **`ARCHITECTURE.md` §5 — DL-045, DL-046, DL-047** |
| The profiles themselves | `D:\Dev\iccce-private-fixtures\ghent-v50\` — **20 `.icc` files plus a manifest** *(verified — enumerated)*. **They must never enter this repository** |
| The extractor | `tools/ghent/extract_icc.py` *(verified — exists)* |
| The outbound asks | `open/note_ghent_output_suite.md`, `open/request_ghent_render_harness.md`, `open/request_profile_population_census.md` in the request channel *(verified — all three exist)*. ★ **The channel is in no git repository: nothing may exist only there** (DL-044) |

### What is measured today, stated at its real strength

- **Acceptance** — `iccce inspect` on all 20 profiles: **20/20 exit 0,
  `malformations: 0`**, including a 1.36 MB X-Rite **ICC v4.2.0** CMYK
  profile whose full 18-tag table decodes. ★ **This proves the parser did
  not refuse. It proves nothing about the parsed values.**
- **Categorical** — the two trap profiles, **with a control**. The
  strongest rows, and narrow: *the declared source profile is used at
  all.*
- **Self-consistency** — `eciRGB v2` as **v2.4.0 vs v4.2.0 from the same
  vendor**, 2,197 grid points, **max |Δ| = 0.000113 in destination device
  coordinates**. ★★ **Both arms are iccce and it is NOT a ΔE.** Do not
  restate it as one.
- **Capability only** — the X-Rite v4 CMYK profile at perceptual intent,
  and the 4-tag `kTRC`-only Gray profile, both evaluate. **Nothing is
  compared to anything.**

### What is NOT measured, and must not be rounded up

- **No accuracy claim of any kind on this corpus.** The lcms2
  differential was dispatched 2026-08-17 and had not reported.
- **Nothing was rendered.** No Ghent *patch* has been processed as a PDF
  by anything here; twenty profiles were extracted and driven directly.
- **No B2A direction, no saturation intent, no ICC-absolute** on this
  corpus.
- **No `tools/difftest` record**, so no row here is in any `pass=…`
  count and none has an **emitted** separation field.
- **20 profiles is a distinct-profile count**, not a count of patches
  exercised and not an inventory of features (DL-031).

### Next, in the order that buys the most

1. **The lcms2 differential over the same 20 profiles** — converts the
   capability rows into cross-checks. **Dispatched, not reported.**
2. **Re-derive the six `[REPORTED]` byte-scan leads** before any is told
   to `pdfce` as fact — the `FOGRA27` / `ISO Coated v2 300% (ECI)`
   output-intent mismatch first.
3. **Grade intent selection against ICC.1**, never against Ghent
   (DL-047).
4. **The joint render-and-compare harness with `pdfce`** — the largest
   available win, and **operator-gated** (below).

### ★★★ Updated 2026-08-17 (later the same day) — **Pass G landed. Item 1 of "Next" is DISCHARGED and the "no accuracy claim" statement above is SUPERSEDED**

**The two lists above are dated observations and are NOT edited.** This
subsection is what changed.

- **★★★ "Next" item 1 — the lcms2 differential — is DISCHARGED.**
  `tools/difftest/src/passg.rs`, **72 graded rows in four sections**,
  whole-suite **`pass=229 fail=0 skip=3 error=0`**. Rows, classes and
  coverage: **`NUMERIC_CLAIMS.md` §3.31, NC-200 … NC-218**; derivations:
  **`TOLERANCES.md` §3.7, §4's two Pass G rows and §6.6**; apparatus:
  **`tools/difftest/README.md` §22**.
- **★★ So *"What is NOT measured"* bullet 1 above — "no accuracy claim of
  any kind on this corpus" — is SUPERSEDED.** There are now **six
  `implementation-cross-check` rows** and six `derived-expectation` ones.
  ★ **Every other bullet in that list still stands**, including *nothing
  was rendered*, and the **B2A direction is still untouched** on the one
  profile that has an interesting one.
- **★ It ran over 11 of the 20 profiles, not 20.** `NUMERIC_CLAIMS.md`
  §7.16 pre-registered exactly this check — *"a differential over a
  different member set is a different claim (DL-031)"* — and it paid off.
  **§3.30 is a claim about 20 profiles; §3.31 is a claim about 11.**
- **★ "Next" item 2 (re-derive the six `[REPORTED]` leads) and item 3
  (grade intent selection against ICC.1) are UNTOUCHED**; item 4 (the
  joint render harness) remains operator-gated, below.
- **★★ Two new decision-log entries came out of the same day and neither
  is about colour: DL-048** (cite the ledger by §/NC, never by line —
  **six of six** line citations were stale) and **DL-049** (a disclosure
  field caught a defect in a *tolerance's justification*, on a green row).
- **★★★ Nothing here changes the operator block below.** More numbers do
  **not** make *"passes the Ghent suite"* available; that is a permission
  question, not an evidence one.

### ★★★ Blocked on the operator — three decisions, and no agent may take any of them

*(There was no "blocked-on-operator" section in this document before
today* *(verified — grepped)**; this is it. The same three are recorded
in `NUMERIC_CLAIMS.md` §7.16.)*

1. **★★★ No public artifact of this project may say "Ghent" without
   GWG's written permission.** Certification is sold to print service
   providers by individual GWG member companies, and *"solution vendors,
   developers or system integrators"* are directed to a **separate
   programme reachable only by contacting GWG**. ★ **So *"passes the
   Ghent suite"* is not available as `README.md`, release-note or
   crates.io copy today** — a **claim-bearing-copy** matter under the
   global rule, not a style choice. **Nothing has been published.**
2. **Whether to pursue that developer Compliancy programme at all.**
   Contacting an external body is an operator act (rule 9), and it is
   the only route by which a public claim could become supportable.
3. **Whether to scope the joint render-and-compare harness with
   `pdfce`** — rasterise `ALL_X4`, compare patch-by-patch against the
   suite's shipped `ALL_REFERENCE`. ★ **It is tractable rather than
   aspirational**: the trap artwork is authored **pre-swapped**, so a
   correct conversion makes the X vanish into its surround and the
   judgement becomes *sample the region against its neighbours* — **no
   press, no proof, no instrument, no human at 0.5 m.**

★ **A fourth decision is enlarged rather than new.** The standing
operator question — *may published numbers live in an MIT repository as
fixtures?* — now spans **four** private corpora rather than three, and
**Ghent's terms are the most restrictive**: no commercial use, no
redistribution without written permission, an **affirmative obligation
to reproduce GWG's notice**, **plus** the separate and individually
unassessed licences of the profiles inside. ★★ **A "yes" on the other
three would not extend to this one**, and that is the distinction most
likely to be lost if they are answered together.

---

## Open questions for the operator — **all four answered 2026-08-11**

Recorded rather than decided, because they were scope calls. **The
questions are left standing as written**; the answers are appended under
them. Nothing above this line was rewritten to look as though it had
always said what was later decided.

**How (b), (c) and (d) were answered, and the limits of that answer.**
The engineer put the items to Ken as *"(1) download the ICC.1 PDF; (2)
the open scope calls: HDR depth (b), profile creator (c — currently a
firm no), crates.io (d)"*. Ken replied, in full: **"1 is done. 2. do
all."** That is the entirety of the operator's statement. **Reading "do
all" as *adopt all three* is the engineer's interpretation**, recorded as
an interpretation throughout. The operator supplied no depth, no
priority, no schedule and no per-item rationale, and none is attributed
to him anywhere in these documents.

- **(a)** Is a separate repository wanted, or does this live alongside
  `pdfce` in one? Affects whether it is published independently.
  — *Annotation, 2026-08-11 (`icc-librarian`): **de facto answered, not
  formally decided.** `D:\Dev\iccce` is its own git working tree, and the
  workspace manifest declares
  `repository = "https://github.com/KenM76/iccce"`. That is a declaration
  in a file, not evidence that the remote exists or that anything has
  been pushed — neither was checked, and publishing remains the
  operator's act (rule 9). What still needs an operator answer is whether
  that remote should be **public**, which is question (d)'s territory.*
  — **ANSWERED IN PRACTICE, 2026-08-11, via (d).** A yes on crates.io
  means the crate source becomes publicly readable at first publish
  regardless of what the git remote does, so the project should be
  written as public-facing from now on. **Still unverified:** whether the
  GitHub remote exists at all, and whether it is public. No agent has
  checked, and this document does not claim otherwise.
- **(b)** How far into HDR? BT.2100 and PQ/HLG are a real body of work
  and only matter if something needs them.
  — **ANSWERED 2026-08-11: in scope.** Filed as **Pass 9** above;
  decision record `ARCHITECTURE.md` **DL-007**. The Pass covers transfer
  functions and primaries; tone mapping, gamut mapping and dynamic
  metadata are explicitly outside it. Blocked on ITU-R documents entering
  the corpus, and on `icc-spec-librarian` first establishing that
  `itu.int`'s terms permit the retrieval.
- **(c)** Is a profile *creator* ever wanted? Currently a firm no; it
  changes the shape of the project if it becomes a yes.
  — **ANSWERED 2026-08-11: the firm no is reversed by the operator.**
  It is now **future scope**, filed as **Pass 10** above; decision record
  `ARCHITECTURE.md` **DL-008**, which quotes the position it reverses
  rather than erasing it. `README.md`'s "Out of scope" list was updated
  the same day to match, and says the scope *changed* rather than
  pretending it had always been planned. **The validation-hardware
  problem that justified the original no is carried forward intact as an
  open engineering problem** — see Pass 10's precondition.
- **(d)** Should `iccce` be published to crates.io? A general-purpose
  MIT CMM in Rust is a thing the ecosystem lacks; that is a reason to,
  and a maintenance commitment.
  — **ANSWERED 2026-08-11: yes, publication is intended.** See
  *Publication — crates.io* above and `ARCHITECTURE.md` **DL-009**.
  **The intent authorises nothing**: the publish act, and any push, tag
  or release, still needs an explicit current go-ahead (rule 9).

### And the operator action that was blocking the corpus — done

*"1 is done."* — `ICC.1-2022-05.pdf` is in
`D:\Dev\Rag-Specialized\ICC_Spec\_sources\`. **Verified by
`icc-librarian`** by listing that directory on 2026-08-11; it previously
held only `README.md`. The retrieval time (11:12) and the fact that it
was a manual browser download are **reported**, on Ken's word, not
measured. Nothing else about the file — size, hash, page count, or that
it is the document its name claims — has been checked by anyone here.

`icc-spec-librarian` was dispatched in parallel to ingest it and **owns
`LEGAL.md` §2 this session**. **Whether the ingest has landed is
unverified**, and DL-002's rule — *no claim in this project may cite an
ICC.1 clause number* — **should be treated as still standing until that
agent files its successor entry.** A PDF nobody has read is not yet a
citable source. Pointer entry: `ARCHITECTURE.md` **DL-006**.
