# NEXT SESSION — start here

---

# ★★★ HANDOFF — 2026-08-18 (later), work PAUSED by the operator

**Read this block first. It supersedes every block below it and anything
else that conflicts; everything below is otherwise still true and still
the reference.** The operator asked for a release build, a `FEATURES.md`,
a handoff, and then for work to **pause**. All four were done. The tree
is clean, the suite is green, and nothing is half-applied.

## If the operator types only "continue"

1. **List `D:\Dev\FeatureRequests\iccce_FeatureRequests\open\`** (§0's
   standing rule). **Nothing was owed by us when work paused.**
2. **Re-arm the 15-minute channel poll.** Monitors die with the session,
   so this is gone and must be re-armed *every* session. Persistent
   `Monitor`, 900 s loop, baseline `stat -c '%n %Y %s' "$DIR"/*` into
   `prev`, `comm -13` against a fresh listing. Track **mtime and size**,
   not names — an edited request is new work.
   ★ **It will echo your OWN writes.** Requests flow both ways and
   filenames carry no direction; read the file's `**from:**` header
   before treating an event as inbound.
3. **Go to "WHAT TO DO NEXT" below.**

## State of the tree — measured after committing, not before

| | |
|---|---|
| tip | **`2a9e126`** |
| commits this session | **5** (`60c32dd..HEAD`) |
| working tree | **clean** — 0 modified, 0 untracked |
| **ahead of `origin/master`** | ★★ **14 commits. NOTHING IS PUSHED.** |
| `cargo test --workspace` | **185 passed, 0 failed** |
| `cargo fmt --all --check` / `clippy --workspace --all-targets -D warnings` | **exit 0** — measured, not asserted |
| release build | `target/release/iccce.exe`, 377 344 bytes, exit 0 |
| synthetic fixtures | **46** (18 well-formed / 28 malformed / 0 disputed) |

★ **`cargo fmt --all` does NOT cover `tools/gen-profiles`** — it is not a
workspace member. Check it separately (`cd tools/gen-profiles && cargo
fmt --check`); it was left clean, but the root gate is blind to it.

**Pushing is not authorised.** Fourteen unpushed commits is a state, not
a backlog to clear on your own initiative.

## ★★ TWO THINGS THE OPERATOR BELIEVES THAT THE TREE DOES NOT SUPPORT

Both were stated on 2026-08-18 and both were checked, not assumed. They
are first because acting on either without re-checking wastes a session.

1. **"pdfce consumes iccce by path."** ★ **It does not, as of
   2026-08-18.** `grep -rn 'iccce' /d/Dev/pdfce --include=Cargo.toml
   --include=Cargo.lock --include=*.rs` returns **nothing** — no path
   dependency, no lockfile entry, no source reference. So **no commit or
   build in this repository currently reaches `pdfce` or any GUI built
   from it.** Either the wiring is planned-but-not-done, or it lives
   somewhere neither search covered. **Re-run that grep before believing
   either story.**

2. **`docs/FEATURES.md` did not exist** before this session; it was
   created. `pdfce` has one at `D:\Dev\pdfce\docs\FEATURES.md`, which is
   the likely source of the expectation. The new file is a
   **consumer-facing capability inventory** — deliberately answering
   *"can I call this tomorrow?"* rather than `README.md` §Status's *"how
   far along is the plan?"*

## What landed this session

- **★★★ A real parser defect fixed** (`7f89829`). The rendering-intent
  malformation was reported **unconditionally**, so a **v2** profile was
  accused in the same words as a v4 one — for a requirement
  ICC.1:2001-04 imposes on **neither half** of the field. v2's *"the
  least-significant 16 bits are reserved for the ICC"* is the identical
  boilerplate 6.1.8 uses for the profile flags, where the high half is
  demonstrably vendor space: **a v2 profile with high bits set is using
  the field as its own edition invites.** `Malformation::UnknownRenderingIntent`
  now carries an `IntentRule`, and the emitted string differs by edition
  because *"outside the defined 0..=3"* is true of v4 and **false of
  v2**. ★ **This is a public API break** — consumers matching that
  variant will fail to compile, which is the good failure mode.
- **The header rendering-intent field is not consumed**, now sourced
  (ICC.1:2022 7.2.15's `shall` binds the *field*, not a CMM; 8.10.2 is
  **silent**) and tested on a synthetic pair differing in **exactly one
  byte**.
- **Pass K filed** — `NC-243`…`NC-266`, `NA-012`, ~24 claims across
  §3.34, and the ROADMAP entry it never had.
- **`DL-062`** — a stale *status* decays faster and more silently than a
  stale *number*. **Two instances, the second arising while writing up
  the first.**
- **`TOLERANCES.md` §3.10.12.7 and `difftest/README.md` §25.13.7
  corrected** — both asserted a compiled-path defect fixed **28 seconds
  before** the doc claiming it was not.
- **Five new fixtures**, closing all four cells of the two-condition
  gate; `Category::Disputed` invented, used, and retained empty.
- **Sourcing**: the IEC 61966-2-1 free preview and BT.709-3:1998 are now
  held; two corpus defects retracted (`C10`, `C11`).

## ★★ WHAT TO DO NEXT

**Nothing is half-done. These are choices, in the order I would take
them.**

1. **★ Verify the `pdfce` link before anything else** — see the box
   above. If a consumer genuinely needs these changes, the wiring is the
   blocker, not the code, and it is ten minutes of work that unblocks
   everything downstream.
2. **The CLI's `header.intent` line has no test.** It prints
   `65537 (UNKNOWN)` for the v2 high-bits fixture while reporting **zero
   malformations** — correct output, but from an **unnamed mechanism**.
   A later tidy-up masking to `& 0xFFFF` would print `media-relative`,
   silently deleting the disclosure, **with a green suite**. One file in
   `crates/iccce-cli/tests/`, five minutes. Deliberately not written,
   because writing it decides the question.
3. **★★ `Malformation`'s doc comment is falsified by two of its own
   variants.** It says *"A rule violation the file carries"*; both
   `TrailingBytes` and the v2 unrecognised-intent report carry none. The
   emitted **words** are careful; the **channel** is not, and
   `malformations: N` is a machine-readable count a consumer will read
   as a conformance verdict. File the named choice in
   `ARCHITECTURE.md` — *"`malformations: N` counts disclosures, not only
   violations"*. Not `TOLERANCES.md`; there is no number.
4. **Tail debt #1, now for the tenth filing: no `published-ground-truth`
   row for any transform.** ★ Every **free** route is now closed with a
   *positive structural reason*, not "we looked": W3C CSS Color 4 and
   web-platform-tests are both downstream of the same paywall, contain
   no matrix and no breakpoint, and assert three orders coarser than the
   question. **CHF 210 for IEC 61966-2-1 pp. 16–51 is the only remaining
   route, and it is an operator decision.**
5. **The perceptual cost of black preservation is unmeasured** — the
   ΔE2000 between the preserved and colorimetric answers on a
   cross-press pair, which is the number a caller weighing the policy
   actually wants. `NA-012`. ★ Do **not** substitute `NC-244`'s
   `1.360900e-1`: that is the same-profile pair, where the policy is
   nearly a no-op.
6. **Still no difftest row for the compiled path**, even though the
   defect is fixed. Its purpose changed from *disclosure* to *regression
   guard* — weaker reason, permanent one.
7. **`blind=11`**, all Pass I `chad` rows — judged **intrinsic** and to
   be left alone. Resist any tightening that makes them look green
   rather than honest.

## Owed to the operator — do not decide these

- **Pushing / tagging / releasing / crates.io.** Fourteen commits
  unpushed. All six crate names were unregistered on 2026-08-17 — **a
  dated observation, not a reservation**; re-check immediately before
  any publish.
- **CHF 210 for IEC 61966-2-1 clauses 3–5**, plus AMD1:2003 and the
  newly discovered **COR1:2014** (which means every pre-2014 restatement
  may be restating uncorrected text).
- **`ICC.1:2010-12` (v4.3)** remains unheld — and ISO 32000-2 cl. 10.3.1
  normatively requires ISO 15076-1:2010, which **is the same document
  under an ISO designation**: a purchase, not a scraping problem.
- **`KMapping::Ratio` (Cholewo)** stays a refusal unless a research
  implementation is wanted.

## ★★★ The traps this session paid for

1. **A compile error can MASK a designed red.** The conformance test was
   built to fail when the version gate landed — and it did, but only
   after I added `rule: _` to a pattern, because the enum gained a field
   and the file would not **compile**. *"The test went red as intended"*
   and *"the test could not run"* are indistinguishable in an exit code.
2. **`$?` after a pipe is the LAST command's exit code.** I ran
   `cargo fmt --all --check | tail -5; echo $?` and read `0` from
   `tail`. Caught only by re-running. This is the fifth-trap family from
   the previous handoff, recurring immediately.
3. **Check the edition before comparing formatter output.** I compared
   `rustfmt --edition 2021` against a crate that is **edition 2024** and
   got a phantom difference that looked like an agent had touched code
   it said it had not.
4. **A retraction filed as a BANNER leaves the body live.** The `C10`
   retraction was written at the head of a file whose body still carried
   the retracted bullet. ★ *"The banner is the failure mode; it makes a
   file LOOK checked."*
5. **An existence blocker without a positive reason stops future
   searching.** *"No worked sRGB triple anywhere"* was carried across
   nine filings **and filed under EXISTENCE** — while the corpus already
   held, had read, and had transcribed the document publishing four.
6. **★ A test fixture's evidential precision is its ASSERTION
   TOLERANCE, not its printed precision.** web-platform-tests prints 5–6
   significant figures and asserts at `epsilon = 0.01`: **47 of 69
   published components are wrong at printed precision while 69 of 69
   pass their assertion.** Find the comparison function and read the
   epsilon.
7. **n = 14 corpus defects, and all fourteen were caught from OUTSIDE
   the file containing them.** That is no longer bad luck; it is a
   property of how single-file edits work. Corollary from `C12`: **grep
   the id, not the sentence** — a resolution and the stale text it
   supersedes share no distinctive phrase.

## Channel state

`open/` held 16 files and **nothing was owed by us**. Two of our asks
remain outstanding **with pdfce**: `request_profile_population_census.md`
and `request_header_tag_channel_disagreement.md`.

★ **Not yet written, and it should be**: a note correcting pdfce's belief
that GWG 130 carries *"two deliberately corrupted profiles"*. It carries
**four**, iccce reports `malformations: 0` on **all** of them, and the
trap is **semantic, not structural** — the colorants are channel-permuted
(red↔green in RGB, magenta↔cyan in CMYK), which was confirmed in the
running transform, not read off a tag table. **GWG 130 is therefore
undiagnosable by profile validation**, which matters because it means
pdfce's failure is confirmation it is not applying the embedded
`ICCBased` profile at all — the right conclusion, for a different reason
than they recorded. ★ **None of those measured numbers may enter this
repository** (`GHENT_COMPATIBILITY.md` §2.3); the durable, corpus-free
part is the structural claim.

---

# ★★ SUPERSEDED — HANDOFF of 2026-08-18 (early), session ended cleanly

**Read this block first. It supersedes the 2026-08-17 handoff below it
and anything else that conflicts; everything below is otherwise still
true and still the reference.** Unlike the last one, this session ended
**deliberately** — the tree is clean, the suite is green, and nothing was
left half-applied.

## If the operator types only "continue"

1. **List `D:\Dev\FeatureRequests\iccce_FeatureRequests\open\`** (§0's
   standing rule). **Nothing is owed by us as of 2026-08-18 02:00.**
2. **Re-arm the 15-minute channel poll.** Monitors die with the session,
   so this is gone and must be re-armed *every* session. Persistent
   `Monitor`, 900 s loop, baseline `stat -c '%n %Y %s' "$DIR"/*` into
   `prev`, `comm -13` against a fresh listing. Track **mtime and size**,
   not names — an edited request is new work.
   ★ **It will echo your OWN writes.** Requests flow both ways and
   filenames carry no direction; read the file's `**from:**` header
   before treating an event as inbound.
3. **Go to "WHAT TO DO NEXT" below.**

## State of the tree — measured, not remembered

| | |
|---|---|
| tip | **`9dc9d70`** |
| commits this session | **8** (`506fcd3..HEAD`) |
| working tree | **clean** — 0 modified |
| **ahead of `origin/master`** | ★★ **8 commits. NOTHING IS PUSHED.** |
| `cargo test --workspace` | **170 passed, 0 failed** |
| `cargo fmt --all --check` / `clippy --workspace -D warnings` | clean |
| difftest, licensed corpus present | **`pass=337 fail=0 skip=9 error=0`** |
| difftest, corpus genuinely absent (CI shape) | **`pass=184 fail=0 skip=94 error=0`** |

★ **"Corpus absent" means an EMPTY `$ICCCE_PRIVATE_FIXTURES`, not an
unset one** — the resolver falls back to a default path that exists on
this machine, so `env -u` proves nothing. That mistake was made and
caught here; do not repeat it.

**Pushing is not authorised.** Rule 9 unchanged; eight unpushed commits
is a state, not a backlog to clear on your own initiative.

## What landed

- **★★★ K-only black preservation** (`crates/iccce-cmm/src/black_preserve.rs`).
  Opt-in named policy, five named refusals, `--preserve-black <policy>`
  on the CLI with **no default** (two published definitions disagree by
  up to `4.9e-2`, so a default would be iccce choosing one and reporting
  it under a name that means both). Exactly zero chromatic ink on all
  ten CMYK destinations; K genuinely re-mapped (`0.366689` at `K_in=0.5`
  on the furthest cross-press pair), which is what distinguishes it from
  "copy K through".
- **Pass K** (40 → 44 rows), built by `icc-conformance` **before** the
  feature existed, then repointed at it. Both deliberately-red rows are
  green and their separations never moved.
- **A false capability retracted**: PDF pages *can* be rasterised here
  via `pypdfium2`; only `pdftoppm` is missing. DL-060.
- **DL-059 / DL-061**: GWG patch 23.0 is PDF device routing, not a CMM
  problem — we had claimed it. Four independent confirmations.
- **Licence manifest fixed** — `iccce-measure` had no `clarify` block and
  would have shipped `Copyright (c) <year> <copyright holders>`.

## ★★ WHAT TO DO NEXT

**Nothing is half-done. These are choices, in the order I would take
them.**

1. **`NUMERIC_CLAIMS.md` is unfiled for the whole of Pass K**, including
   §F and the grading. Free ids: **`NC-243`**, **`NA-012`**. This is the
   largest bookkeeping debt and `icc-librarian` owns it.
2. **The `docs/` sweep owed by §7.21 / §7.22** — every row whose
   provenance is "agreement between text-extraction engines" is weaker
   than it reads, and the sweep now widens to *"glyph-sensitive **or
   possibly set in a figure**"*. The figure half **cannot be found by
   comparing extractions**; it needs rasters.
3. **pdfce's Ask 2** — a `convert_buffer_u8` / `_f32` surface.
   Deliberately deferred under rule 8. ★ The reply already tells them the
   thing worth remembering: **`u8` fixes the MEMORY ask
   (268 MB → 67 MB) and does nothing for the TIME ask**, because
   `1.4 Mpix/s` is the grid evaluation and that cost is per pixel however
   the pixel arrived.
4. **Tail debt #1, now for the ninth filing: no `published-ground-truth`
   row for any transform.** `IEC 61966-2-1`'s sRGB primaries remain the
   cheapest route and nobody has dispatched for it. **This is still the
   largest hole in the project.**
5. **`blind=11`**, all Pass I `chad` rows — judged **intrinsic** and to
   be left alone. Resist any tightening that makes them look green rather
   than honest.

## Owed to the operator — do not decide these

- **Pushing / tagging / releasing / crates.io.** All six crate names were
  unregistered on 2026-08-17 — **a dated observation, not a
  reservation**; re-check immediately before any publish.
- **`ICC.1:2010-12`** — and it got *more* valuable: **ISO 32000-2
  cl. 10.3.1 normatively requires ISO 15076-1:2010 (= ICC.1:2010)**, so
  our named consumer's own standard points at it with a `shall`. ★ A new
  acquisition route exists: **ISO 15076-1 is the same document under an
  ISO designation** — a purchase, not a scraping problem.
- **`KMapping::Ratio` (Cholewo)** stays a refusal unless the operator
  wants a research implementation. It needs a fitted differentiable
  printer model and five constrained optimisations *per colour*; its six
  weights are three-unspecified, so **two faithful implementations of the
  paper will not agree with each other**.

## ★★★ The traps this session paid for — all four are general

1. **A zero-separation fixture MANUFACTURES a false pass.** Measured by
   injection: the headline red row went **green at `0.000000`** and a
   transition-width row reported a number that *looks like a working
   feature*. Grade the separation as its own row with a floor declared in
   advance; a classifier verdict prints in a column beside a green row,
   and nobody reads columns.
2. **Ask which layer is in the loop of the FIX, not just of the test.**
   A guard whose own text said a leak *"shows up here and nowhere else"*
   was **inert** — the feature is opt-in, so a row driving the plain
   surface had no feature in its chain to leak. It would have stayed
   green through any leak, vouching for the silence.
3. **An interpolator cannot represent a step.** Black preservation is a
   discontinuity at `C=M=Y=0`; sampling the *preserving* conversion onto
   a grid gave **0.617 of wrong ink** within one cell of the axis, and
   **refining the grid did not move it** — `O(1)` beside `O(h^1.32)`. The
   fix is structural (sample the smooth conversion, carry the policy
   outside the grid), and the test asserts **convergence, not a bound**:
   a wrong constant passes a threshold test; nothing passes a convergence
   test by luck.
4. **A constraint an agent infers about its own environment is a
   READING, not a fact.** Third recorded instance. The useful finding is
   not the pattern but *why it recurred*: the rule from instance 2 lived
   in `ICC_Spec`'s notes and **had no counterpart in `docs/`**, so it did
   not bind. DL-060 closes the gap rather than restating the rule.

★ **And a fifth, about the engineer rather than the code:** three times
this session I measured the wrong thing and caught it only by
re-running — a `tail -25` that captured only doctests, a `grep` in a
stale working directory whose errors `2>/dev/null` swallowed, and an
`env -u` that did not disable what it appeared to. **Every one of them
looked like a result.**

## Channel state

`open/` holds 16 files. **Nothing is owed by us.** Two of our asks are
outstanding **with pdfce**: `request_profile_population_census.md` and
`request_header_tag_channel_disagreement.md` (the second is *one extra
column* in the first's corpus sweep — say so if they schedule them
apart).

★ **`reply_ghent_render_harness.md` is the highest-value thing in that
folder.** pdfce built the harness and measured: **22 of 51 Ghent patches
show a trap X, 20 of them pdfce's own features, and zero attributable to
an iccce defect.** Overprint is 10 of the 22 — the largest bucket by
nearly two, and the first *evidence* for the boundary note's claimed
ordering rather than an assertion of it.

---

# ★★ SUPERSEDED — HANDOFF of 2026-08-17, session paused mid-flight

**Read this block first. It supersedes anything below it that conflicts,
and everything below it is still true and still the reference.** Written
because the operator had to restart the machine (mouse became unusable —
**not caused by anything this session ran**; no input automation was used
at any point, and no build of ours was running when it happened).

## If the operator typed only "continue", do these three, in order

1. **List `D:\Dev\FeatureRequests\iccce_FeatureRequests\open\`** (§0's
   standing rule).
2. **Re-arm the 15-minute channel poll.** The operator asked on
   2026-08-17 for the channel to be polled *throughout* a session, not
   just at startup. Monitors die with the session, so **this is not
   running any more and must be re-armed every session**: a persistent
   `Monitor`, 900 s loop, baseline `stat -c '%n %Y %s' "$DIR"/*` into
   `prev`, then `comm -13` against a fresh listing each cycle. Track
   **mtime and size**, not just names — an edited request is new work.
3. **Go to "THE QUEUE AS OF NOW" below.** Do **not** start from §3 of the
   old handoff without reading it — §3's ordering is stale.

## State of the tree

★ **Everything from this session is UNCOMMITTED and nothing is pushed.**
A restart does not lose it — it is all on disk — but the first act of the
next session should be to review `git status` and decide with the
operator what to commit. **Committing was offered and never authorised.**
Commit by explicit path, never `git add -A` (§5.8).

Tip is still **`e21154c`**. Files touched this session:

| file | what |
|---|---|
| `docs/GHENT_COMPATIBILITY.md` | **NEW.** The Ghent workstream in full. Mine. |
| `docs/DEFAULT_DESTINATION.md` | **NEW.** The sRGB-fallback decision. Mine. |
| `tools/ghent/extract_icc.py` | **NEW.** Profile extractor. Mine. |
| `tools/difftest/src/passg.rs`, `src/bin/ghent_probe.rs`, `lib.rs`, `main.rs` | **NEW/edited** by `icc-conformance`. Pass G. |
| `docs/NUMERIC_CLAIMS.md`, `ARCHITECTURE.md`, `ROADMAP.md`, `SESSION_LOG.md`, `NEXT_SESSION.md`, `TOLERANCES.md`, `tools/difftest/README.md` | filed by `icc-librarian` / `icc-conformance` |
| `.claude/agent-memory/icc-engineer/*` | 3 new memories + index |
| `D:\Dev\iccce-private-fixtures\README.md` | 4th corpus's terms |

## What landed, in one line each

- **Ghent PDF Output Suite 5.0 is now a compatibility target** (operator,
  2026-08-17): *compatibility, never certification* — certification needs
  a press and is organisationally closed to a library anyway. See
  `docs/GHENT_COMPATIBILITY.md`.
- **20 real-world ICC profiles extracted** from 98 PDFs / 121 embeddings
  into `D:\Dev\iccce-private-fixtures\ghent-v50\`. **20/20 parse, 0
  malformations.** ★ Doubly licence-encumbered; **never commit, never
  copy a value into the repo.**
- **Pass G** built by `icc-conformance`: suite `pass=157` → **`pass=229
  fail=0 skip=3 error=0`, exit 0** [VERIFIED by me, run bare]. The v4
  `mAB ` disagreement with lcms2 **is the interpolation method and
  nothing else** — 179×/243× collapse with lcms2's geometry substituted,
  and the envelope was predicted from the CLUT's own bytes to 0.04 %.
- ★ **It ran over 11 of the 20 profiles, not the corpus.** Four distinct
  CMYK print profiles have no differential row, including
  `ISO Coated v2 300% (ECI)` — the `DestOutputProfile` of *every*
  ICC-CMS patch.
- **sRGB sourcing landed** (operator supplied the URLs). Primaries and
  white point from **ITU-R BT.709-6**, breakpoints from **W3C**; neither
  is lcms2. **The oracle-contamination blocker is GONE.**
- **The operator downloaded the color.org profile set** into
  `C:\Users\Ken\Downloads` (2026-08-17 14:34–14:45, ~60 files). ★ These
  are **still in Downloads and have not been moved, catalogued or
  licence-classified.** That is job 1 in the queue.

## ★★★ SUPERSEDED 2026-08-17 (later the same day) — READ THIS BEFORE THE QUEUE BELOW

**Filed by `icc-librarian`.** ★ **The queue below is a DATED OBSERVATION
and is not edited.** Items **1, 2, 3 and 4 are DISCHARGED** and item **6
is partly discharged**. **Go to "★★★ THE QUEUE, REWRITTEN" at the end of
this block.** Rows, classes and coverage: **`NUMERIC_CLAIMS.md` §3.32
(NC-219 … NC-229) and §7.18**; decisions: **`ARCHITECTURE.md`
DL-050 … DL-053**; completion records: **`ROADMAP.md`**'s Pass 8
retrospective subsection.

### ★★★ THE CORRECTION THIS BLOCK EXISTS FOR: "two iccMAX" is TEN

**Item 1 below says two of the downloads correctly failed as iccMAX.
TWO WERE TESTED; TEN ARE PRESENT.**

> **Measured over all 50 `.icc` files** *(`icc-engineer`, **[VERIFIED —
> header bytes 8..12 across all 50, then `iccce inspect` over all 50
> reading `$?` per file, bare, no pipe]**)*: **40 parse, exit 0,
> `malformations: 0`; 10 are REFUSED BY NAME, exit 1**, iccMAX
> `0x05000000`, with the version in every refusal message. **The ten:**
> `FluorescentNamedColor`, `NamedColor`, `Lab-D50_2deg`,
> `SixChanCameraRef`, the four `Spec400_10_700-*`,
> `sRGB_D65_colorimetric`, `sRGB_ISO22028`.

★★★ **The lesson is now `ARCHITECTURE.md` DL-053, and it is about THIS
DOCUMENT.** The original claim carried `[VERIFIED by me]` and **every word
of it was true of what was run.** What was missing is the phrase
***"…of the two I tested."*** **A `[VERIFIED]` tag certifies that a
measurement happened and certifies NOTHING about what it ranged over** —
and this is the resume-from-cold handoff **every session is instructed to
read first**, so the figure was positioned to be re-quoted rather than
re-derived. **That is DL-048's carrier mechanism with a count instead of
a citation.**

★ **Re-deriving the denominator upgraded the claim rather than merely
correcting it:** *"rule 6 demonstrated on real ICC-published files"* is
now a **population** result, **NC-219**, not two files.

### ★★★ Three things that bound anything you quote from today's work

1. **Nothing is committed and nothing is pushed.** Authorisation has not
   been given. **Every row in §3.32 has NO COMMIT ANCHOR.**
2. **★★★ The conformance runner was NOT run.** `icc-conformance` holds
   `tools/difftest` and `docs/TOLERANCES.md` for a **concurrent Pass H**,
   both **untouched**. **`pass=229` is NC-218's dated observation at
   `e21154c`** — **do not quote a `pass=` line on today's authority**, and
   **expect a Pass H filing that this one does not cover.**
3. **★★ CI covers none of it.** NC-221/NC-222 **SKIP** without a
   resolvable sRGB profile; NC-219/NC-220/NC-223 are **CLI sweeps over
   private corpora CI will never hold.**

### What landed, in one line each

- **★★ The constructed sRGB destination is BUILT** — `builtin.rs`, from
  **ITU-R BT.709-6**, **W3C CSS Color 4** and **ICC.1:2022 Annex E.3**.
  **No I/O, no blob, no dependency, no lcms2 in the lineage.**
  **DL-050:** selection is a **two-variant enum, not `Option<&Profile>`**
  — *an `Option` being `None` cannot distinguish "there was none" from "I
  failed to get one", and only the second must never trigger the
  fallback.* The fallback is **disclosed** via `DestinationProvenance`.
  ★★★ **Evidence class is a THIRD and WEAKER one** —
  `constructed-vs-reference-file`, **neither ground truth nor a
  cross-check** (§3.32.1). **Max `0.033013` ΔE2000 at white; black
  exact.**
- **★★ The `/N` accessor is BUILT** — `colour_space.rs`. **A48: ICC.1 is
  SILENT on header/tag channel agreement**, so iccce **discloses** and
  does not declare non-conformance. **A50: the count is a TWO-TABLE JOIN;
  ICC.1 publishes no `Signature → count` map** — cite as derived.
  **60 profiles, zero unrecognised, zero PCS-field violations.**
- **★★★ A CORPUS CLAIM WAS FALSIFIED.**
  `ICC_Spec/iec/iec__s__srgb.md`'s *"8-bit codes 10 and 11, and nothing
  else"* is **wrong**: **no 8-bit code lands in the window** and the
  separation at 8-bit precision is **exactly zero**. **The corpus still
  carries the wrong version** — owed to `icc-spec-librarian`.
- **★★★ The suite had ZERO POWER against the constant it documented
  most** — the breakpoint substitution passed **6 of 6** tests while four
  other injections went red. **DL-051**, and **it was found by injection,
  not inspection.**
- **★★ `sRGB2014.icc` is NOT a second source** — its colorants and all
  three TRC tables are **byte-identical** to the HP 1998 file's.
  **Exactly one lineage. The gap is NOT closed.**
- **★★ NC-213's open question is DISCHARGED by decision** — iccce uses
  `wtpt` **as stored** and **discloses** (A4c / NA-007), **verified in the
  running thing**. **6 of 60 disclose; all six hand-audited are true
  positives.** ★ **NC-213 the ROW is untouched and was never pending.**
- **★ `ChainError` implements `std::error::Error`** — found by a compiled
  **doc example** failing with `E0277` (**DL-052**).
- **Gates:** `cargo test --workspace` **154 passed, exit 0** (was 132);
  clippy / `fmt` / `doc` / **`wasm32` over 4 library crates** all exit 0.
  ★ **iccce still does not GATE `wasm32` in CI** — a **consumer's** gate.

### ★★★ THE QUEUE, REWRITTEN

**1. ★★★ Get the falsified corpus claim repaired.** Dispatch
`icc-spec-librarian` at `ICC_Spec/iec/iec__s__srgb.md`. **Until it is
fixed the corpus will confirm a wrong belief to the next reader who
checks it** — and the code now asserts the correct statement
(`no_eight_bit_code_lies_between_the_two_candidate_breakpoints`), so
**the corpus and the tests currently disagree.** *Done when: the corpus
says no 8-bit code lands in the window, and cites the measurement.*

**2. ★★★ Assert NC-227, or delete the number.** The 1 020-corruption
enumeration is real, but **the survivor count reaches a `println!` and
the in-loop assertion compares a call with its own result — it cannot
fail** (`crates/iccce-profile/src/colour_space.rs`). ★★ **This is
DL-051 recurring inside the same session, in the module that documents
the hazard at greatest length.** *Done when: the count is asserted and an
injection turns it red.*

**3. ★★ Expect and integrate `icc-conformance`'s Pass H.** It was
building `tools/` and `TOLERANCES.md` concurrently and **files its own**.
★ **Do not merge its numbers with §3.32's** — different runners, and
§3.32's rows are **not** in `pass=…` at all. §7.18 records that the
ledger now has rows in **three** runners.

**4. ★★ Answer `pdfce` — the channel still owes a reply**, and today's
work is most of the answer. `request_iccbased_colour_spaces.md`'s design
question (*does iccce construct sRGB, or demand a caller-supplied
destination?*) **is now ANSWERED**: **both**, with the distinction carried
by **DL-050**'s enum. ★★★ **Frame the adoption case as CONFORMANCE, never
accuracy** (DL-044/DL-047) — and say plainly that the new destination's
evidence class is **weaker** than a cross-check, not stronger.
`request_pdf_output_intent_cmyk.md` is still owed. **Nothing may exist
only in the channel.**

**5. ★ The remaining unfiled gaps — down from four to two.** The
**`f32`/`u8` evaluation surface** (`f64`-only, so an 8 Mpix CMYK page is
**256 MB in / 192 MB out**) and the **four unexercised CMYK print
profiles**, including `ISO Coated v2 300% (ECI)`. ★ Both are in the
`ROADMAP.md` Pass 8 retrospective now, so they are recorded rather than
merely known.

**6. ★★ Gate `wasm32` in CI, or stop restating that it passes.** Third
filing at which it is written as a dated observation. It is a
**consumer's** CI gate (`CLAUDE.md` rule 10.2), so the cost of it going
stale is paid in another repository.

**7. ★ Supply §3.32's commit anchor** when the operator authorises a
commit. §2.10 is the precedent and it took two filings to close.

**8. ★★ The long-standing debts are UNTOUCHED and one is at its seventh
filing.** The A41 constant's error; the two black-point fixtures; the
`Separation::against` retro-audit; the `skip`/`ungraded` enumeration
(fifth filing); the six `[REPORTED]` byte-scan leads (third filing —
**nothing may be told to `pdfce` as fact until they are re-read**); CI
evidence and a Linux run (**twenty-two filings, no CI run observed by
anyone here**). ★★ **DL-042 says re-audit the REASON an item is owed, not
just the item** — and it has now been recorded as a MISS for the third
consecutive filing.

---

## ★★ THE QUEUE AS OF NOW — **superseded; see the block above**

**1. Rescue the color.org downloads out of `Downloads`.** They are the
most valuable artifacts in the session and they are sitting in a folder
people empty. Move to `D:\Dev\iccce-private-fixtures\` (new subfolder),
**write the terms subsection FIRST** — that folder's own rule 3, which I
broke once this session and recorded rather than tidied. Contents
include: `sRGB2014.icc` (ICC's own, `wtpt` **D50** + `chad` present,
`dmdd` = *"IEC 61966-2-1 Default RGB Colour Space - sRGB"*), the two
`sRGB_v4_ICC_preference*` v4.2 profiles, `ITU-RBT709ReferenceDisplay.icc`,
`D50/D55/D65_XYZ.icc`, `Lab-D50_2deg.icc`, **`Probev1_ICCv2.zip` /
`Probev1_ICCv4.zip` / `Probev2.zip` (ICC's own CMM probe profiles + a
readme PDF)**, **`PRMG_v2.0.1_MR.icc`** (perceptual reference medium
gamut — relevant to the A41 perceptual-black constant), a 7-channel
**`APTEC_CMYKOGV`**, **`NamedColor.icc` / `FluorescentNamedColor.icc`**
(`namedColor2Type`), `SixChanCameraRef.icc`, CVD profiles, spectral
`Spec400_10_700-*`, and a large CMYK set (SWOP2006/2013, GRACoL 2006/2013,
CGATS21 CRPC1/3, PSOuncoated_v3 FOGRA52, PSOsc-b FOGRA54, SNAP2007,
Fogra39L/47L VIGC, APTEC).

**Missing, and the only thing that is:** `color.org/chardata/rgb/srgb.pdf`
— the sRGB *document*. `ROMMRGB.pdf` came instead. Operator-browser job.

★ **Two of them correctly FAILED** [VERIFIED by me]:
`sRGB_ISO22028.icc` and `sRGB_D65_colorimetric.icc` are **iccMAX (ICC.2,
version `0x05000000`)** and iccce refused by name, exit 1, no repair.
That is rule 6 demonstrated on real ICC-published files rather than
synthetic ones — **worth a graded row**, and it does not exist yet.

> ★★★ **CORRECTED 2026-08-17 — "two" is a SAMPLE, not the population.
> TEN of the 50 are iccMAX.** The paragraph above is left standing
> because **the shape of the error is the record** (`ARCHITECTURE.md`
> **DL-053**): it carried `[VERIFIED by me]`, every word was true of what
> was run, and **what was missing is *"…of the two I tested."*** The
> measured population is **40 parse with `malformations: 0`, 10 refused
> by name** — `NUMERIC_CLAIMS.md` **§3.32.10a / NC-219**, which is the
> graded row this paragraph said did not exist. **Do not quote "two".**

**2. Build the constructed sRGB destination.** The operator decided
2026-08-17: *"if the caller supplied destination doesn't exist then it
should fallback to constructing sRGB internally."* Full contract,
conditions and build order in **`docs/DEFAULT_DESTINATION.md`** — read it
before writing a line. The two things that decide whether it is safe:
**"doesn't exist" must mean *absent*, never *unresolved*** (a declared
destination that failed to parse stays a named refusal — do **not** key
this on an `Option` being `None`), and **the fallback must be disclosed,
not silent**. ★ Do **not** test it by byte-equality against any shipped
sRGB profile; use a ΔE round-trip and name the blue-`Z` difference as a
rule-4 approximation.

> **★★★ Dated correction, 2026-08-17 (supplementary filing). This item is
> BUILT, and the last sentence above is WRONG about whose difference it
> is. The paragraph is left standing as the record of what was
> instructed.** ICC's own **"How to interpret the sRGB color space …"**
> (Holm, ICC, 2015-04-27) **publishes the D50-adapted colorants at 15
> decimal places** (§B.2). Measured against them: **iccce's construction
> 3.02 ULP worst / 0.90 in `bXYZ.Z`; the shipped HP 1998 /
> `sRGB2014.icc` file 11.13 ULP.** ★★★ **The ~12 ULP blue-`Z` residual is
> the FILE's error, not iccce's** — `NUMERIC_CLAIMS.md` **§3.33 /
> NC-230, NC-231**, **§4's NA-011**, `ARCHITECTURE.md` **DL-054**. ★ **The
> two instructions that were RIGHT are still right**: no byte-equality
> against a shipped profile (better justified now — the file does not
> match ICC's published values either), and a rule-4 entry is owed —
> **which is NA-011, registered late because the obligation had been
> discharged against a doc comment while the register was empty.**

**3. NC-213 is owed BY ME, not by the librarian.** Does iccce follow
lcms2 and substitute D50 for a mis-authored `wtpt`? Today it does not;
**leaving it undecided is a position and it is the shipped behaviour.**
My provisional answer is **report, don't substitute** (rule 6). It was
deliberately held pending the sRGB sourcing — **which has now landed, so
it can be settled.** See `GHENT_COMPATIBILITY.md` §8.1 and §4.5b.

**4. The `/N` gap, re-scoped — my earlier scoping of it was wrong.**
`Chain::input_channels()` exists and is public (in
`crates/iccce-cmm/src/transform.rs` — ★ **cited by symbol, not by line;
the `:632` this once carried is now `:952`** *(verified at the tip,
2026-08-17)*, which is DL-048's decay caught a second time in one
document) but
reports the **tag's** count and needs a built chain. `pdfce` needs the
**header signature's** count, *before* building, to validate PDF `/N`.
And when header and tag disagree that is itself a malformation worth
disclosing. So this is a public `Signature → component count` accessor
plus a cross-check — and **the signature table is spec data, so rule 2
applies: dispatch `icc-spec-librarian`, do not write it from memory.**

**5. Repair the 5 remaining stale citations** in §0 below, per the new
§5.8 rule: **cite the ledger by § and NC-number, never by line.** All six
were verified stale by me; `:5788` is a blank line.

**6. Not started, and named so it is not forgotten:** the four unexercised
CMYK print profiles (see above); the `f32`/`u8` evaluation surface
(`f64`-only today, so an 8 Mpix CMYK page is 256 MB in / 192 MB out);
`ChainError` still does not implement `std::error::Error` while
`ParseError` does (`crates/iccce-profile/src/diag.rs:83`).

## Owed to the operator — do not decide these

1. **Commit / push authorisation.** Nothing committed, nothing pushed.
2. **No public artifact may say "Ghent"** without GWG's written
   permission — certification is sold to print service providers, and
   vendors are directed to a separate programme. Claim-bearing copy.
3. `color.org/chardata/rgb/srgb.pdf`, and `ICC.1:2010-12` (still the
   highest-value download outstanding).
4. Whether to scope the joint Ghent render-and-compare harness with
   `pdfce` (`open/request_ghent_render_harness.md`).

## Channel state — three files owed a reply BY pdfce, one owed BY us

Written by us and open: `note_ghent_output_suite.md`,
`request_ghent_render_harness.md`,
`request_profile_population_census.md`,
`reply_iccbased_colour_spaces.md`. Still owed **by us**: a reply to
`request_pdf_output_intent_cmyk.md` — answer it as a **conformance**
question, **never** an accuracy one.

## ★ Three process lessons this session paid for

- **A tool limit diagnosed from your own request is not a fact about the
  publisher.** `itu.int` was recorded as WAF-blocking all agents and that
  propagated to four documents. False: honest `curl` gets the PDF; a bad
  browser UA gets a 245-byte reject. It sat one `curl` away for five days.
- **A stale *citation* is worse than a stale *number*** — a wrong number
  invites re-derivation, a wrong pointer invites the reader to accept the
  destination. Now DL-048.
- **I quoted a corpus file's banner into a work order without reading the
  body**, and the banner had contradicted the body for five days. Read
  the file, not the header.

---

**Written 2026-08-12 by `icc-engineer` at the operator's request, as a
resume-from-cold handoff.** Replaces the eighteenth-filing edition
entirely. **Overwrite this file once acted on.**

> **You can begin work from this file alone.** If the operator has said
> only *"continue"*, do **§0** first — it is one directory listing — then
> go to **§3 THE QUEUE** and start at the top. Read §5 before touching
> any code — it is the set of rules this project learned the hard way,
> and several of them will look like over-caution until you see what they
> cost when skipped.

**Read order:** **§0 below (list one directory)** → this file →
`docs/ARCHITECTURE.md` §5 (the decision log; **DL-033 … DL-043** are the
ones that change how you work) → `docs/NUMERIC_CLAIMS.md` §3.25–§3.29 →
`docs/TOLERANCES.md` §1.1 and §3.5.9 → `docs/ROADMAP.md` header status
block → `tools/difftest/README.md` §20–§21.

---

## 0. ★★ FIRST, EVERY SESSION: CHECK THE REQUEST CHANNEL

```text
D:\Dev\FeatureRequests\iccce_FeatureRequests\open\
```

**List that directory before anything else. Empty means nothing is
owed** — that is the entire check and it costs one call. The standing
rule and the four facts that constrain how you answer are **`CLAUDE.md`
rule 10**; read it once, then this section is just the pointer.

The channel is between this session and the **`pdfce`** session
(`D:\Dev\pdfce\`), created by the operator **2026-08-17**. `pdfce` is a
PDF engine with **no colour management at all**, and it is the consumer
`README.md` names first. **Requests flow both ways** — writing
`open/request_<topic>.md` to ask `pdfce` about real-world profile shapes
is not just permitted, it is the only check this project has on whether
its API is callable from a real consumer's per-pixel loop.

**★ AS OF 2026-08-17 THE CHANNEL IS NOT EMPTY.** Four files, three of
them owed to us:

| file | what it is |
|---|---|
| `request_pdf_output_intent_cmyk.md` | `pdfce` ignores PDF/X `/OutputIntents` entirely and converts CMYK through a **pdfium-fitted baked table**. Asks whether iccce can take the document's own embedded destination profile instead |
| `request_iccbased_colour_spaces.md` | `pdfce` parses `ICCBased` in full and then renders through Table 66's `/Alternate` fallback. **Carries the one design question that must be settled before any code**: does iccce construct sRGB internally, or demand a caller-supplied destination profile every time? |
| `note_boundary_and_overprint.md` | informational, no reply owed. Splits a real failing conformance file by owner. **Overprint is `pdfce`'s** — compositing, not conversion |
| `reply_capability_status.md` | **written by this project, 2026-08-17.** Answers *only* what exists today, with `file:line` citations. **It deliberately does not answer either design question** |

**What that reply established, so it is in git and not only in a folder
that is in no repository:**

- **Parsing from `&[u8]` is built** — `crates/iccce-profile/src/lib.rs:80`.
  **Per-pixel evaluation is built** — `crates/iccce-cmm/src/compiled.rs:171`
  and `:180`, allocation-free. **Named refusal is built** — `ParseError`,
  `Malformation`, `ChainError`.
- **★★ iccce constructs NO sRGB destination.** `Chain::new`
  (`transform.rs:246`) demands two parsed `&Profile`s. Every `sRGB` in
  `crates/` is a **test** reading the Windows system `.icm` off disk. The
  blocker is not mechanical — a computed destination needs no file and no
  redistribution — it is **rule 2**, and it is **harder than one
  dispatch**. `IEC 61966-2-1` has never been dispatched for, restated as
  owed in the standing **"`published-ground-truth` for any transform"**
  row of `NUMERIC_CLAIMS.md`'s **§7.x** status tables — **§7.11**
  (*"ninth filing"*), **§7.12** (*"tenth"*) and **§7.14** (*"twelfth"*)
  each name the document explicitly *(verified — read 2026-08-17)* — **and the reason is that it is
  PAYWALLED and was not obtained** — `NUMERIC_CLAIMS.md` **§3.5 /
  NC-018**, whose own row reads *"Weakest constant in the crate — say so
  whenever D65 is quoted"*. So the corpus's sRGB constants **rest on
  lcms2 `cmsvirt.c` alone**, D65 is recorded there as **single-source**
  and **not cross-verified**, and **§3.8.2 / NC-036**'s evidence-class
  row warns that the **shared-misreading risk is ELEVATED, not merely
  present**, because *"the corpus against which any future ground-truth
  check would be built shares an origin with the oracle"* — restated at
  **§3.8.9**'s first bullet as the one place in that section where *"its
  two sides share a sourcing origin"*. **A computed sRGB built today would take its
  white point from the implementation we cross-check against.** Obtaining
  the document is an operator act (like `ICC.1:2010-12`, §2.2);
  **ITU-R BT.709 is free from ITU and is the un-fetched independent route
  for D65** — that, not a dispatch, is the cheap lever.
- **★ The whole evaluation surface is `f64`.** No `f32`, no integer path.
  An 8 Mpix CMYK page widens to **256 MB in / 192 MB out** through
  `convert_buffer`. This is the API finding a real consumer produced that
  no amount of internal review would have.
- **wasm32 PASSES** — `cargo build --target wasm32-unknown-unknown` over
  all four library crates, **exit 0, measured 2026-08-17 at tip
  `e21154c`**. It passes structurally: `Cargo.lock` is **five packages,
  all ours**, and every `std::fs` in a library crate is inside
  `#[cfg(test)]`. ★ **But iccce does not gate wasm32 in CI** — that is a
  dated observation, not a guarantee, and a future dependency could break
  a consumer's CI silently.
- **★★ Q5, the important one:** adopting iccce would be a **lateral move
  in evidence class** for `pdfce`, and the reason is **structural, not a
  gap effort would close.** ICC.1 mandates no interpolation method, so
  **no published ground truth for a LUT path can exist even in principle**
  — corroborated because **iccDEV, ICC's own reference implementation,
  ships zero expected colour values** (`NUMERIC_CLAIMS.md` **§3.29.6**,
  the boxed *"WHAT THIS IS NOT"* and the paragraph under it — *"`RunTests.sh`
  compares nothing; `ApplyDataFiles/` holds inputs only"* — generalised as
  **`ARCHITECTURE.md` DL-041**; and
  `crates/iccce-cmm/tests/annex_d_ground_truth.rs:29-32`). iccce's oracle for SWOP→sRGB is
  lcms2 (NC-048 **0.25294** ΔE2000 media-relative, NC-049 **1.6590**
  perceptual). **The honest case for adoption is CONFORMANCE — the
  document's declared output intent being honoured at all — never
  accuracy.** The structural remedy is §2.3's second lineage.

**Four gaps this project now knows about and has NOT filed anywhere else**
(scoping them is work; this session was told not to): no computed sRGB
destination; no `f32`/`u8` evaluation surface; **`ChainError` does not
implement `std::error::Error`** while `ParseError` does
(**`crates/iccce-profile/src/diag.rs:83`** — the full path matters:
**there are TWO distinct `ParseError` types in this workspace**,
`crates/iccce-measure/src/lib.rs:221` and this one, and **both** implement
`Error`, so a bare filename names neither the crate nor the type
*(`icc-engineer`, [VERIFIED — read this session]; `ChainError` has no
`Error` impl anywhere under `crates/`)*);
and **no public signature→component-count helper**, so
`Header::color_space` cannot be turned into `/N` without building a chain
first. **None has a `ROADMAP.md` entry.** If any becomes load-bearing for
`pdfce`, scope it.

**Binding, because the folder is in no git repository:** nothing may
exist only there. A reply is a pointer plus an executive summary; the
durable finding lands in `docs/`. One topic per file. A colour claim
carries its reference, its number, **and the name of its oracle**.

---

## 1. STATE, MEASURED — not remembered

At tip **`0bd76ad`**, branch `master`, **70 commits**. Every figure below
was produced by running the thing on 2026-08-12, with the runner named
(DL-031: **an unlabelled count is not a claim**).

| runner | command | cwd | result |
|---|---|---|---|
| workspace | `cargo test --workspace` | repo root | **132 passed**, exit 0 |
| harness | `cargo test --all-targets` | `tools/difftest` | **47 passed**, exit 0 |
| generator | `cargo test --all-targets` | `tools/gen-profiles` | **28 passed**, exit 0 |
| fixtures | `cargo run --release -- verify ../../fixtures/synthetic` | `tools/gen-profiles` | **40 identical, 0 not** |
| conformance | `cargo run --release` | `tools/difftest` | **`pass=157 fail=0 skip=3 error=0`**, exit 0 |
| lint | `cargo clippy --workspace --all-targets` / `cargo fmt --all --check` / `RUSTDOCFLAGS=-D warnings cargo doc` | repo root | exit 0 / 0 / 0 |

Separation aggregate (same conformance run):
`unstated=119 no-named-alternative=12 incommensurate=3 ungraded=8
zero-separation=2 blind=0 discriminating=16 sep-broken=0`.

> ★★ **SUPERSEDED 2026-08-17 by Pass G — the table above is a dated
> observation at `0bd76ad`, not the current state.** At tip **`e21154c`**
> the conformance runner gives **`pass=229 fail=0 skip=3 error=0`, exit
> 0** and the aggregate is `unstated=119 no-named-alternative=58
> incommensurate=3 ungraded=8 zero-separation=2 blind=0
> **discriminating=42** sep-broken=0` *(`icc-engineer`, **[VERIFIED — ran
> the gate bare, redirected to a file, read `$?`]**; this librarian ran
> nothing)*. **Corpus-absent behaviour: `pass=157 skip=7`, exit 0**, four
> labelled SKIP rows — so a machine without `$ICCCE_PRIVATE_FIXTURES`
> reproduces the old pass count exactly, and **a green line there means
> Pass G did not run.**
>
> ★ **The two aggregates are internally consistent** *(derived here, the
> only check available without a shell)*: both sum to their row counts,
> **160 → 232 = 160 + 72**, and the entire delta lands in
> `discriminating` **(+26)** and `no-named-alternative` **(+46)**, which
> is 72 exactly, with `unstated`, `incommensurate`, `ungraded`,
> `zero-separation` and `blind` all unchanged. That corroborates *"72
> rows, none unstated"*.
> ★★ **What it does NOT settle:** `TOLERANCES.md` §3.7.3 records **12**
> §B rows deliberately taken out of grading (2 ICC-absolute, 10 `--bpc`)
> — yet `ungraded` **did not move from 8**. Either those rows emit a
> different separation status or `ungraded` counts something narrower
> than "not gated". **UNSETTLED — do not quote `ungraded=8` as an
> enumeration of the rows this suite does not grade** (§7.14 newly-owed 4
> already owes the same enumeration for `skip=3`).

**★ `skip=3`** is three rows, one cause, and is a *principled refusal to
grade*, not a concealed failure:
`pass4/swop-to-srgb/icc-absolute/{pcs-lab-vs-lcms2,
pcs-lab-emulated-geometry, pcs-lab-corners-interpolation-free}`. `transicc
-o*Lab4 -t3` applies the D.6/D.7 media-white scale to the PCS on lcms2's
side while iccce's `A2Bx` evaluation is media-relative by construction, so
the two arms are not measuring the same quantity. Comparing them would
mean *modelling* the oracle rather than *measuring* it.

**★ 15 commits are UNPUSHED.** Pushing is the operator's act and has not
been authorised. This matters beyond bookkeeping: **CI's `harness` job and
its manual `oracle` job have never run anywhere**, and `build-lcms2.sh`
has never been executed on any machine. The first push is what exercises
them.

---

## 2. ★★★ BLOCKED ON THE OPERATOR — do not decide these

1. **Pushing / tagging / releasing / crates.io.** Never without an
   explicit *current* go-ahead. Publication metadata is ready; the act is
   not authorised.
2. **`ICC.1:2010-12`** — the highest-value download outstanding. It is the
   sole blocker on **both** remaining `UNVERIFIED` corpus register rows
   (`A31`, `A47`). `color.org`'s ToS bars automated retrieval, so this
   needs a human browser.
3. **A second implementation lineage.** `iccDEV` is **BSD-3 and ICC's
   own**. This is the strongest available answer to DL-033 (see §5) and
   the largest single improvement left to the verification apparatus.
   Adding a dependency-class artifact is an operator call.
   ★ **Argyll CMS is AGPL-3.0 and must never be read or cited.**
4. **Passes 9 (HDR) and 10 (profile creation)** — scope calls. Pass 10 is
   now *startable without measurement hardware* because FOGRA51 is held
   locally (see §4).

**Already decided, do not re-litigate:**

- **Published third-party numbers stay OUT of this MIT repository.** They
  live in `D:\Dev\iccce-private-fixtures\` and tests read them at run
  time. Decided 2026-08-12. See §4.
- **`D:\Dev` is a Google Drive mirrored sync root** (`.tmp.driveupload` /
  `.tmp.drivedownload`, `GoogleDriveFS` active). This was measured,
  disclosed, and **accepted by the operator**. The private-fixtures folder
  therefore protects against accidental *commit*, not against *sync*. Do
  not raise this again as a new discovery; if it ever needs closing, the
  fix is to move the folder outside `D:\Dev` (the `D:\` root is not itself
  a mirror) and repoint `$ICCCE_PRIVATE_FIXTURES`.

---

## 3. THE QUEUE — what *"continue"* means, in priority order

Each item says what it is, why it is worth doing, and how you will know
you are done. Take them from the top; they are independent unless noted.

### 3.0 ★ Two owed design answers in the request channel — UNSCOPED

**Listed first because §0 says three files are owed, not because a
priority call has been made.** Nobody has decided whether these preempt
§3.1; `reply_capability_status.md` answered *what exists* and explicitly
declined both design questions. Read §0, then choose.

1. **Does iccce construct sRGB, or demand a caller-supplied destination
   profile?** This is the load-bearing one and it **sets the API before
   a consumer shapes itself around whatever it finds**. Today it is the
   latter, by omission rather than by decision. ★ **Do not scope this as
   "one dispatch to `icc-spec-librarian`" — §0 explains why that reading
   is wrong.** `IEC 61966-2-1` is **paywalled and unobtained**, so the
   corpus's sRGB constants come from **lcms2 alone** and building on them
   would put the oracle's own white point underneath every conversion.
   **The cheap independent lever is ITU-R BT.709, free from ITU and never
   fetched.** And **do not write the constants from memory** (§5.1) —
   this is precisely the rule's case.
2. **Should `pdfce` route `/OutputIntents` CMYK through iccce at all?**
   Answer it as a **conformance** question (the document's declared print
   condition gets honoured) — **never as an accuracy question**, because
   §0 records why that framing is unsupportable on today's evidence.

★ **And the thing most likely to be skipped: ASK `pdfce` SOMETHING.**
Requests flow both ways and this project has never used that. The offer
already on the table is a census across `pdfce`'s ~6,000-file corpus —
`/N` distribution, v2-vs-v4, device class, and (ask for it explicitly)
**tag type**. The recommended-grid constant rests on **one profile
pair, one direction, one tag type** — `NUMERIC_CLAIMS.md` **§3.19 /
NC-145** (which grades the compiled path at the shipped default grid 33,
and records that grid 17 **failed**) and **§3.27**; the constant itself is
`iccce_cmm::compiled::recommended_grid_points` in
**`crates/iccce-cmm/src/compiled.rs`**, called from the `bench` path in
**`crates/iccce-cli/src/main.rs`** — and
DL-021 says direction and tag type are part of that claim.

> **★★ Citation repaired 2026-08-17 (supplementary filing), and the
> repair is a REMOVAL rather than an update.** This paragraph cited the
> call site as `main.rs:421`; **it is now line 459** *(verified —
> grepped at the tip)*. **DL-048's rule is to cite by symbol, never by
> line**, so updating `421` to `459` would have re-armed exactly the same
> decay. ★ **The constant's own citation is now by file and symbol too.**
> ★★★ **And the constant's neighbourhood changed on 2026-08-17**: the
> `_ => 33` catch-all was removed after it was measured to abort the
> process on a seven-channel profile (`NUMERIC_CLAIMS.md` §3.33 /
> **NC-234**), and `recommended_grid_points` now **computes** the ≥5-channel
> recommendation. **The 3-D and 4-D `33` are unchanged and are asserted as
> measured values**, so the census request below is unaffected. A population
distribution would say whether that constant is fitted to an
unrepresentative sample. **It costs `pdfce` almost nothing and it is the
strongest evidence class either project could add cheaply.**

**Done when:** each design question has an answer recorded in `docs/`
(not only in the channel), and the exchange is closed per the channel
README — both files moved to `archive/`, **one row added to `INDEX.md`
naming where the durable answer lives**.

### 3.1 Export an 8-bit Lab codec, and extend the ground-truth test

**Bounded, and it buys published ground truth.** The Annex D test
(`crates/iccce-cmm/tests/annex_d_ground_truth.rs`) currently asserts
**twelve** exact published integers. Table D.5 prints **six more** as
8-bit codes (`255,128,128` white / `30,128,128` black) which are held in
the fixture, **unused**, because this crate exposes no public 8-bit Lab
codec — the path is inline in `lut_ab.rs`'s normalisation.

Do **not** write the encoding from memory (§5.1). Dispatch
`icc-spec-librarian` for the clause, export the codec beside
`LabEncoding` in `pcs_encoding.rs`, then extend the test and delete its
closing "NOT TESTED" note.

**Done when:** 18 of 18 published integers assert, and the test still
fails when a value is perturbed.

### 3.2 Extend candidate separation to the remaining passes

`unstated=119` of **232** rows as of 2026-08-17 (it was 119 of 160; **the
numerator did not move and the denominator did** — Pass G added 72 rows
and **zero** unstated ones). Pass 5c, Pass 4c and Pass G are done;
**Pass 4, 4b, 5, 5b and 6 are not.** ★ **`119` is the same integer in
both aggregates and means the same thing; the RATIO is what changed.**
Carry the denominator (DL-031). DL-033 says a cross-check's power is bounded by the
separation of its two candidate answers; until a row states one, its power
is unknown.

★ **`unstated` going down is only progress where the separation is real.**
`no-named-alternative` **with its reason** is a legitimate and useful
answer. Prefer 45 true statements over 145 invented ones. Where several
rivals exist, name the one that **most threatens the row** — picking the
flattering rival is precisely the tuning this mechanism exists to prevent.

Dispatch `icc-conformance`; `tools/` and `TOLERANCES.md` are its files.

### 3.3 Two owed instruments for the floored fixture

`v4-rgb-mab-floored-b2a.icc` cannot separate lcms2's `L*` from
`InitialLab`'s, because `BlackPointAsDarkerColorant` reads the same vertex
through the same `A2B`. Two fixtures would, and neither exists:

- an **inverse-polarity** fixture (`vertex_set(3)` is a *search*; lcms2
  uses a constant);
- a vertex **lighter than `L* 95`**, which reaches lcms2's untested
  `if (Lab.L > 95) L = 0` branch.

### 3.4 Audit whether NC-176 … NC-178 used the defective `against`

`Separation::against` was found deriving distance as
`|observed − alt_observed|`, which collapses to zero on exactly the defect
run it exists to detect. Three rows were fixed. **Whether the three
earliest separation rows use the defective form was never established**
and must not be inferred. Owed by `icc-librarian` §7.15.

### 3.5 Larger, and the highest value of all: the second lineage

See §2.3. If the operator authorises `iccDEV`, this is the work that
converts "agrees with one implementation" into "agrees with two
independent lineages" — and it is the only structural answer to the defect
class described in §5.2.

---

## 4. WHERE THINGS LIVE

| what | where | note |
|---|---|---|
| the repository | `D:\Dev\iccce\` | MIT. Contains **no** third-party numbers. |
| private fixtures | `D:\Dev\iccce-private-fixtures\` | ★ **Never commit. Never copy a value out of it into the repo.** Read its `README.md` first — three items, three *different* licensing postures. |
| standards corpus | `D:\Dev\Rag-Specialized\ICC_Spec\` | Private dev reference. Never shipped, never committed, **never on `R:\`** (Dropbox). |
| the oracle | `tools/difftest/vendor/` | git-ignored; pinned by commit hash in `lcms2.pin`, not by tag. |
| **the request channel** | `D:\Dev\FeatureRequests\iccce_FeatureRequests\` | ★ **In NO git repository, deliberately** — transient coordination, not a record. **Nothing may exist only there.** `open/` is the working set (empty = nothing owed); `INDEX.md` is the memory; `archive/` is never read unless a row points at it. See §0 and `CLAUDE.md` rule 10. |
| the consumer | `D:\Dev\pdfce\` | The `pdfce` session's own tree. **Read-only from here** — this project does not edit it; it writes into the channel instead. |

**The private fixtures hold:** ICC.1:2022 Annex D.6.3's values (ICC ©,
no reproduction right); the CIE 1931 tables (**CC BY-SA 4.0 — an actively
incompatible share-alike grant**, the hardest of the three); and FOGRA51
via `pso-coated_v3.zip` (ECI's `cprt` **contradicts itself**; the
restrictive reading was taken).

**Tests resolve them via `$ICCCE_PRIVATE_FIXTURES`, then the default path,
then SKIP.** ★ A green run on a machine without them is **not** evidence
that those checks passed — it is evidence they did not run. CI is
permanently in the skipping case, by design and by written note in
`ci.yml`.

**Agents — dispatch freely, never ask permission:** `icc-spec-librarian`
(every sourcing question), `icc-conformance` (oracle, fixtures, tolerance
budget — owns `tools/` and `TOLERANCES.md`), `icc-librarian` (ROADMAP,
SESSION_LOG, decision log, numeric-claims ledger; **has no shell**, so
your dispatch *is* its source — see §5.5).

---

## 5. ★★★ THE RULES THAT WERE LEARNED, NOT ASSUMED

Read this section before touching code. Each item cost something.

### 5.1 Never write colour maths from memory

Adaptation matrices, transfer-function breakpoints, Lab encodings — all
things one half-remembers correctly. Dispatch `icc-spec-librarian`, cite
the standard and clause in the doc comment. This is not a style
preference: **a wrong colour looks exactly like a right one**, and a 3 ΔE
error reaches a customer's press without announcing itself.

### 5.2 ★ Agreement with the oracle can be the SYMPTOM of a defect

A differential test measures `|ours − theirs|` and has **no power against
an error that moves your answer *toward* the oracle's**. On 2026-08-12 a
non-conformant black-point estimator returned `outRamp[first] = MinL =
16.489806` — a value **lcms2 also computes** — landing 0.082 ΔE76 from
lcms2's answer. **The defect's own magnitude was 4.717 L\*, 57.8× the
signal it produced.** The buggy build agreed with the oracle *better than
the correct build does*. It was caught by **reading the clause**, never by
the diff.

Consequences: treat a suspiciously **small** disagreement as *unexplained*,
not as success. If you cannot say *why* a residual has the size it has,
you have observed a number, not verified anything.

### 5.3 A test that cannot fail is not evidence — prove it by injection

Repeatedly decisive this session. Every substantive test added was
verified by **injecting the defect it claims to catch** and watching it go
red while its siblings stayed green. Two real cases:

- a test asserting the whole `(L,a,b)` triple was **blind to chroma**
  because it passed `InitialLab = (0,0,0)` — every wrong answer also has
  `a=b=0`;
- the **synthetic fixture** had `InitialLab` and `outRamp[first]` **both
  `L* 20`**, so it could not move regardless of what the code did.

★ **Simple, round, symmetric fixture values make conceptually distinct
quantities coincide, and coincidence destroys discrimination.** When
authoring a fixture, give every distinct quantity a **distinct** value on
purpose (GP-002).

### 5.4 A displayed value is an INTERVAL, not a point

`0,0097` printed at 4 dp is `[0.00965, 0.00975]`. Point-evaluating it and
declaring an inconsistency is how ICC's Annex D worked example — the only
published ground truth this project has for a transform — got wrongly
**rejected** and sat unused for eleven filings.

★ And the general form (DL-042): **a negative finding removes its own
auditor.** Nobody re-tests a fixture they have been told is broken. Wrong
*assertions* get caught in days because everyone who relies on them audits
them; wrong *rejections* survive indefinitely. **When an item has been
"owed" for many cycles, re-audit the REASON it is owed, not just the
item.**

### 5.5 Your dispatch to a shell-less agent IS its source

`icc-librarian` cannot run anything. Six times this session a dispatch
disagreed with the tree — twice via **verbatim quotes of text that did not
exist**, once because the dispatcher **edited the tree after dispatching**.
A filing that dutifully "corrects" a string you invented corrects nothing
and reports success.

**Quote only what you read this session. Otherwise describe the defect.
Name the artifact class precisely — a commit message is not a filing is
not a doc. Tag every claim `[VERIFIED — I ran it]` or `[CARRIED — not
re-derived]`.** That tagging is what made the seventh instance cheap to
catch.

### 5.6 The gate is the bare exit code

This project has shipped **two** false "all green" claims from piping test
output through `grep`/`tail` — `grep` exits 0 on a *FAILED* match; `tail`
masks the real status. Run the gate bare, redirect to a file, read `$?`,
and summarise separately. The conformance runner encodes this properly:
`0` passed, `1` a check failed, `2` harness/oracle error, **`3` nothing
ran — which is not success.**

### 5.7 A claim-bearing number must be computed, not typed (DL-034)

**Five instances, the fifth authored by the same person who adopted the
rule, hours later.** A number typed into prose beside the code that
computes it goes stale silently. Format it at run time. Where that is
impossible (a YAML comment), **do not state a number the run will state
for you** — or anchor it as a *dated observation* with the tip it was
measured at. A number used as **evidence** gets a date and stands; a
number used as **description** should not be written down at all.

### 5.8 ★★ A stale CITATION is worse than a stale number — cite by section and NC-number, never by line

**§5.7 covers a claim-bearing *number* going stale. This is a
claim-bearing *pointer* going stale, and it fails in a worse direction.**
A wrong number invites a re-derivation — the reader who doubts it goes and
recomputes. **A wrong pointer invites the reader to accept whatever is at
the destination**, because arriving somewhere plausible reads as
confirmation. The citation authenticates the wrong text instead of
failing.

**The instance, 2026-08-17.** §3.0 cited the 33-node recommended-grid
constant as `NUMERIC_CLAIMS.md:2164, :2529`. Neither line carried it.
`:2529` described **`USWebCoatedSWOP.icc`'s own `lut8` CLUT, which happens
to have 33 nodes — a different 33**: a vendor file's tag, not iccce's
recommendation. A reader at `pdfce` following it would have concluded the
recommendation is a property of somebody else's profile. **The line was
copied out of this file into an outbound cross-project request without
being checked** — so the handoff document every session is instructed to
read first was the *carrier*, and would have reproduced the error on every
future read. Correction filed at `NUMERIC_CLAIMS.md` §3.30.7.

★★ **This is structural, not a slip.** Verified by this librarian
2026-08-17, reading each cited location at tip `e21154c`: of the **six**
`NUMERIC_CLAIMS.md` line-number citations spot-checked in this file
(`:2164`, `:2529`, `:5788`, `:623`, `:976`, `:6488`), **six did not carry
the claim cited to them.** §3.30.7's own correction is already stale in
the same way — it describes what `:2164`/`:2529` held *before that
filing's edits shifted them*, and they have since moved again. A ledger
that grows by insertion **renumbers every line below the insertion**, so a
line citation into it decays on the next filing, silently, everywhere it
was copied.

**The rule:** cite `NUMERIC_CLAIMS.md` by **§-number and NC-number**,
which are stable identifiers that move *with* their content. Never by
line. For **source files** a `path:line` citation is acceptable and has
held up — the same sweep found `lib.rs:80`, `compiled.rs:171`/`:180`,
`transform.rs:246` and `diag.rs:83` all exact — but **give the full path
from the repo root**: §0's bare `diag.rs:83` is right about
`crates/iccce-profile/src/diag.rs` and names neither the crate nor the
`src/`.

★★ **DISCHARGED 2026-08-17 at the Pass G filing — all five remaining
citations are re-cited above, and the DL entry is filed.** *(Which
filing that is by number depends on the population counted;
`NUMERIC_CLAIMS.md` §7.17 states both integers rather than choosing one,
because the filing that FOUND this defect was scoped to this file alone
and therefore appears in one population and not the other.)*

- `:5788` (*"owed across twelve consecutive filings"*) → the standing
  **"`published-ground-truth` for any transform"** row of the **§7.x**
  status tables; **§7.11 / §7.12 / §7.14** name `IEC 61966-2-1`
  explicitly.
- `:623` (*"paywalled and was not obtained"*) → **§3.5 / NC-018**.
- `:976` (*"shared-misreading risk ELEVATED"*) → **§3.8.2 / NC-036**'s
  evidence-class row, restated at **§3.8.9**.
- `:6488` (*"iccDEV ships zero expected colour values"*) → **§3.29.6**,
  generalised as **DL-041**.
- **`§4.4`** → **a sixth failure, and of a DIFFERENT KIND: a bare
  §-number with no document named.** Nothing in `docs/` has a §4.4
  carrying a recommended-grid constant; the **two** §4.4 sections that do
  exist are `LEGAL.md` §4.4 (*"What is not claimed"*) and
  `GHENT_COMPATIBILITY.md` §4.4 (*"ICC v4 evaluation on a vendor-authored
  profile"*) *(verified — grepped, 2026-08-17)*. **Both destinations are
  plausible enough to be read as confirmation**, which is this section's
  thesis arriving without even a line number to blame. Replaced in §3.0
  by the named homes.
- **`diag.rs:83`** was right in line and in content and **wrong in
  path**, and the reason to give the full path is now measured rather
  than stylistic: **two distinct `ParseError` types exist in this
  workspace and both implement `Error`.** §0 now names the crate.

★★ **And the decay was observed happening.** `NUMERIC_CLAIMS.md`
**§3.30.7** recorded `:2164` as §3.13's **Pass 6 shared-coverage box**
*(this librarian's reading, 2026-08-17, before that filing's own edits)*;
the corrected outbound census request, read at the tip the same day,
describes the same line as *"unrelated (BPC material)"*
*(`icc-engineer`'s reading — verified, file read)*. **Two readers, two
moments, one line number, two different destinations, and neither reading
is wrong.** That is the mechanism, not an anecdote.

**Filed as `ARCHITECTURE.md` §5 DL-048**, beside DL-034 as this section
asked. ★ **The outbound `open/request_profile_population_census.md` has
also been corrected by `icc-engineer`** *(verified — read; it now cites
§3.19 / NC-145 and §3.27 and keeps the wrong citations visible as a
correction note)*, which discharges `NUMERIC_CLAIMS.md` §7.16
newly-owed 3.

### 5.9 Other standing rules

- **The parser reports; it does not repair.** A silently corrected tag
  hides the malformation from the only layer that could disclose it.
- **Tolerances are justified, not tuned.** When a row fails, the first
  question is whether the *code* is wrong. If a row must be exempted,
  **declare the exemption and grade the declaration** — never let it be
  acquired by a number happening to come out small (DL-043).
- **A large separation on an UNGRADED row buys a fixture and a graded row
  elsewhere — not a licence to grade that row** (DL-040). Ask what clause
  the bound would be graded against; if the answer is "none, but it would
  have caught the bug", the bound is fitted to the bug.
- **Disagreement with lcms2 is a finding, not a failure.** It is an
  implementation, not the standard. Settle it from the specification text
  and write down the outcome. Say lcms2 **diverges**; never
  *non-conforming*.
- **Optimise only after correct.** A fast wrong answer is harder to fix
  than a slow one, because the speed becomes load-bearing.
- **Commit by explicit path, never `git add -A`.** Sibling agents hold
  `tools/`, `fixtures/` and `docs/` concurrently; a bare `-A` has already
  swept an agent's mid-write files once.

---

## 6. WHAT IS DONE

**Passes 0–7 are closed and filed** — the original scope is complete.
Parsing (v2 and v4, `mft1`/`mft2`/`mAB`/`mBA`, report-don't-repair),
colorimetry (ΔE2000 against all 34 Sharma pairs at `1e-4` — still the
project's **only** published-ground-truth metric row), matrix/TRC and LUT
transforms, all four rendering intents in both directions, black point
compensation (ISO/CD 18619 4.2.5, with a conformance defect of our own
found and fixed), the compiled fast path, and a CLI.

Added after the original scope, this session: `iccce-measure` (CGATS/IT8.7
reader, exercised on the real 1,617-patch FOGRA51 payload — zero issues);
**candidate separation** as a first-class emitted quantity; a third Pass 5c
fixture arm whose power was proven by injection; a **parser robustness
sweep** (the parser had 261 panic sites and nothing testing untrusted
input); CI coverage for `tools/` (**71 tests that gated nothing**); and the
**first ground-truth test for a transform path** (Annex D.6.3, twelve exact
integers).

**Pass 8 is built in `pdfce`, not here.**

---

## 7. THE ONE-SENTENCE SUMMARY OF WHERE THIS PROJECT STANDS

It parses, transforms and measures correctly to stated tolerances against
lcms2 and against one informative published example — **and the honest
form of that claim is "it agrees with another implementation, plus twelve
published integers", which is not yet the same as "it is right"**; closing
that gap is what §2.3 and §3.5 are for.
