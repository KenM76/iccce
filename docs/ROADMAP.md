# iccce — roadmap

Passes, in dependency order. Each is sized to be finishable and
verifiable; a Pass that cannot be demonstrated is too big.

**Pass 0 is done (2026-08-11). Pass 1's core is complete and validated
(2026-08-11, same day) with an explicit remainder — see its own status
block. Passes 2–10 are plan, not record.**

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

## Pass 3 — matrix/TRC transforms

The analytic path: RGB→XYZ→RGB through matrices and tone curves, with
adaptation. Covers sRGB, Adobe RGB, Display P3 — most display profiles.

**Done when**: sRGB→AdobeRGB round-trips within a stated ΔE, and matches
lcms2 within a stated tolerance, with both numbers written down.

## Pass 4 — LUT transforms and rendering intents

`A2B`/`B2A`, multi-dimensional interpolation, all four intents including
absolute-as-media-relative-plus-white-point. **v2 vs v4 Lab encoding
lives here** and is the Pass's main risk.

**Done when**: CMYK→RGB through a real press profile matches lcms2
within tolerance at every intent, and the v2/v4 cases are separately
covered.

## Pass 5 — black point compensation

**Done when**: BPC on and off differ in the documented direction, and
match lcms2's BPC within tolerance.

## Pass 6 — performance

Compiled transforms, caching, a benchmark on a page-sized raster. Only
now — optimising before Pass 4 is correct is how a fast wrong answer
gets locked in.

**Done when**: a 300 DPI A4 CMYK→RGB conversion completes in a stated
time, and the compiled path's error against the uncompiled one is
measured and stated.

## Pass 7 — named colours and spot

`namedColor2Type`. The Pass `pdfce` is waiting for, because it is what
makes `Separation` and `DeviceN` colorimetric rather than approximated.

## Pass 8 — the pdfce bridge

Built **in pdfce**, not here. `ICCBased`, output intents, and replacing
the `/Alternate` fallback with a real conversion.

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
