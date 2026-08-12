# NEXT SESSION — start here

**Written 2026-08-12 by `icc-engineer` at the operator's request, as a
resume-from-cold handoff.** Replaces the eighteenth-filing edition
entirely. **Overwrite this file once acted on.**

> **You can begin work from this file alone.** If the operator has said
> only *"continue"*, go to **§3 THE QUEUE** and start at the top. Read
> §5 before touching any code — it is the set of rules this project
> learned the hard way, and several of them will look like
> over-caution until you see what they cost when skipped.

**Read order:** this file → `docs/ARCHITECTURE.md` §5 (the decision log;
**DL-033 … DL-043** are the ones that change how you work) →
`docs/NUMERIC_CLAIMS.md` §3.25–§3.29 → `docs/TOLERANCES.md` §1.1 and
§3.5.9 → `docs/ROADMAP.md` header status block →
`tools/difftest/README.md` §20–§21.

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

`unstated=119` of 160 rows. Pass 5c and Pass 4c are done; **Pass 4, 4b, 5,
5b and 6 are not.** DL-033 says a cross-check's power is bounded by the
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

### 5.8 Other standing rules

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
