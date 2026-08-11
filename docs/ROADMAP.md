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
