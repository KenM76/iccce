# NEXT SESSION — start here

**Written 2026-08-11 by `icc-librarian`, at the close of Pass 1.**
Replaces the Pass 0 edition entirely. Overwrite this file once acted on.

Read order: this file → `docs/ROADMAP.md` (Pass 1's completion record and
Pass 2's plan, including the dated annotation under Pass 2) →
`docs/NUMERIC_CLAIMS.md` (**new** — the ledger, and especially §1 evidence
classes and §5 "what Pass 1 does not claim") → `docs/ARCHITECTURE.md` §5
(**eleven** entries now; DL-010 and DL-011 are today's) →
`docs/TOLERANCES.md` §1–§2 → `docs/SESSION_LOG.md` (three entries, all
2026-08-11; the third is Pass 1).

---

## Where the project actually is

**Pass 0 done. Pass 1's core complete and validated — both on
2026-08-11.**

`iccce-color` exists and is real: XYZ/xyY, Lab/LCh, D50 + D65, the von
Kries method with Bradford cones, ΔE76 and CIEDE2000. The project has its
**first published-ground-truth measurement**: CIEDE2000 agrees with **all
34 Sharma, Wu & Dalal (2005) pairs within 1×10⁻⁴** at k = 1:1:1
(`NUMERIC_CLAIMS.md` **NC-001**).

`docs/NUMERIC_CLAIMS.md` **now exists**, created with that claim as its
first row.

### What is easy to over-read, so read it here first

- **"35 tests green" is not coverage.** 21 of them are in
  `iccce-color`, and **exactly one** is a correctness claim against
  published values. The rest are arithmetic identities, which detect
  drift and structural error and **cannot detect a consistently wrong
  constant** — a round trip through a wrong white point round-trips
  perfectly.
- **There is no cross-check against lcms2 anywhere.** Not one row of the
  ledger is an `implementation-cross-check`, because there is still **no
  Rust difftest harness** — nothing drives `transicc` programmatically
  (`tools/difftest/README.md` §10).
- **Chromatic adaptation has no ground-truth row at all.** No published
  worked example of a complete adaptation was obtained. It rests on a
  primary-sourced matrix plus identities. **Pass 1's largest evidential
  hole.**
- **Nothing has run on Linux, and no CI run has ever been observed.**
- **Pass 1 is uncommitted.** Its numbers are anchored to a working tree,
  not a hash. Whoever commits fills the hash into `ROADMAP.md`'s Pass 1
  record and `NUMERIC_CLAIMS.md` §2.
- **The corpus D50-chromaticity erratum was still present at filing** —
  a fix was dispatched; verify before assuming.

---

## Pass 2 — full tag-type parsing. **And a validator is now defensible.**

`ROADMAP.md` Pass 2: header, tag table, and the tag types real profiles
use — `XYZType`, `curveType`, `parametricCurveType`,
`textType`/`multiLocalizedUnicode`, `lut8`/`lut16`/`lutAToB`/`lutBToA`,
`namedColor2`, `s15Fixed16Array`. **Report malformations, repair
nothing.** Identify iccMAX and refuse it by name.

**Done when**: every profile on the machine parses or is refused with a
reason, and a synthetic corpus covers each tag type.

### The evidence position changed — this is the headline for Pass 2

Pass 0 filed DL-002's consequence in the librarian's own words: **"a
parser is defensible on this evidence and a validator is not."** That was
true of a corpus built from C headers, which encode signatures and
offsets exactly and prose requirements not at all.

**The ICC.1:2022 ingest has landed** *(verified by this librarian —
corpus files read on 2026-08-11)*. The corpus now carries
`evidence: primary_spec`, real clause numbers, verbatim normative text,
per-extractor agreement records, tag layouts and **the required/optional
tag column**. **On that evidence a validator is defensible**, and Pass 2
may plan for one — with the usual discipline: a conformance assertion
cites the clause it enforces, and where ICC.1 is SILENT the register says
so and iccce asserts nothing.

**Read the corpus's own frontmatter before citing any file.** The tiers
are still not equal — `primary_spec`, `cross_verified_2src`,
`impl_crosscheck` and `not_sourced` all coexist in there, sometimes
within one file.

### Three traps the corpus already knows about — do not rediscover them

1. **★ The legacy Lab selector is the TAG TYPE, never the version.**
   **DL-011**, filed today, before any code exists to get it wrong.
   6.3.4.2 NOTE 3: the legacy encoding belongs to `lut16Type` and
   `namedColor2Type` *"and only those tag types"*. **The corpus's own
   first pass claimed the opposite and is retracted.** A
   `version < 0x04000000` test is wrong **in both directions**, and the
   common wrong case — an `mft2` Lab tag in a **v4** profile, i.e. most
   production CMYK output profiles — is **sub-perceptual**
   (ΔE 0.3–0.5, `L*` 0.39 % low). Thread the encoding choice with the tag
   type at the point the tag is decoded. **DL-005 is untouched:** test it
   with exact-value integer invariants, never with ΔE.
2. **★ `parametricCurveType` Table 68 CHANGED between ICC.1:2010 (v4.3)
   and ICC.1:2022 (v4.4)** — corpus divergence **D10**. Two conformant
   CMMs reading different editions can evaluate **the same `'para'` tag
   differently.** **What changed is NOT SOURCED — do not guess it**; see
   the operator-download list below.
3. **★ Clause cross-references inside ICC.1:2022 are stale.** The
   document's own surviving references still use ICC.1:2010 numbering:
   it says `lut16Type` is "10.8" when 10.8 is `dateTimeType` and
   `lut16Type` is **10.10**. Corpus file
   `icc__ref__spec_defects.md` §1 lists them. **Follow the tag-type
   clause table, not the prose cross-references.**

### Bookkeeping that Pass 2 should not step around

**DL-002's clause-citation prohibition has no filed successor.**
`ARCHITECTURE.md` §5 ran to DL-009 before today; DL-006 said the
prohibition lifts only when `icc-spec-librarian` files DL-002's successor
entry, and **it has not.** Meanwhile `crates/iccce-color/src/adapt.rs`
already cites "ICC.1:2022 Annex E.3" and today's DL-011 cites 6.3.4.2 and
10.10. The *condition* is materially met; the *entry* is missing.
**Dispatch `icc-spec-librarian` to file it** — it should say on what
terms a clause number may be cited, and what a doc comment must state
alongside one (which extractor, which corpus file, what tier).

---

## Owed work, carried explicitly

### 1. `icc-conformance` — the legacy-Lab-in-v4 behavioural difftest ★

**The one owed item with a named owner and a defined method.** iccce
follows the clause text (tag type); **lcms2 keys on the profile
version**. On the letter of 6.3.4.2 NOTE 3 and 10.10 the version test is
the wrong selector — but **whether lcms2 is behaviourally wrong on real
files is NOT established**, and must not be asserted. The two selectors
agree on the common `mft2`-in-v2 case; they diverge on `mft2`-in-v4 and
on `ncl2`, and no lcms2 tree was read in the corpus pass that found this.

**The test:** build a synthetic v4 profile containing an `mft2` Lab
`A2B0`, push a known `L*` through `transicc`, and see whether lcms2 used
**652.8** or **655.35**. Project rule 7 — disagreement with lcms2 is a
finding, not a failure. Whichever way it lands, write it down. Until it
is run, iccce follows the specification **and logs the divergence at
runtime** rather than silently differing from the field's dominant CMM.

### 2. Pass 1's remainder — all blocked on sourcing, none on engineering

Land each **when, and only when, a citable source arrives.** Implementing
any of them today produces a claim that must be labelled weaker than it
looks, which is worse than an honest absence.

| Item | Blocked on | If done anyway |
|---|---|---|
| **ΔE94**, **ΔE CMC(l:c)** | formulas not transcribed from a citable source; **no published worked examples obtained** (the ingest session ran out of budget) | lcms2-cross-check only — strictly weaker than ground truth, and rule 3 requires labelling it so in the test, the doc comment **and** the ledger |
| **von Kries (HPE) cone matrix** | corpus digits are a placeholder marked **DO NOT USE**; "von Kries" is ambiguous between the general method (implemented) and this matrix (absent) | a nine-digit act of faith |
| **CAT02** | CIE 159 paywalled | — (not needed for ICC.1) |
| **Observer CMF tables** | **not blocked — not needed.** No Pass plans spectral input. | listed only so "standard illuminants and observers" is not read as delivered in full |

### 3. Ledger hygiene (`NUMERIC_CLAIMS.md` §7)

A commit hash for §2; **observed residuals rather than only asserted
bounds** (a residual that grew from 10⁻¹² to 9×10⁻⁵ still passes a 10⁻⁴
gate and nothing would show it); `TOLERANCES.md` §3.1 and §5 rows, which
are **`icc-conformance`'s** and are still blank; a ground-truth row for
adaptation; the corpus erratum; a Linux run.

---

## Optional operator unblocks — cheap, and each settles something named

Listed by `icc-spec-librarian` in the corpus (`LEGAL_NOTE.md` §1c,
`index.md`). **All three are browser downloads by Ken, not agent
retrievals.** color.org's ToS prohibition on automated access is
**unchanged and still standing** — the ICC.1:2022 file was cleared by
*human* retrieval, which is outside the robot clause; that did not create
a route for agents. **Do not re-attempt automated retrieval of any
color.org / archive.color.org document.**

**None of these blocks Pass 2.** Each removes a specific unknown:

| Document | What it settles | Why it matters, concretely |
|---|---|---|
| **`ICC.1:2010-12` (v4.3)** | **A31 / D10** — *what* changed in `parametricCurveType` **Table 68** between v4.3 and v4.4 | Directly Pass 2 and Pass 3 material. Two CMMs on different editions can evaluate **the same `'para'` tag differently**; right now the corpus knows *that* it changed and explicitly **does not know what** changed, and says do not guess |
| **`ICC.1:2001-04` (v2)** | **A1b, A2, A34** — whether v2 scoped the legacy encoding the same way; **what occupies header bytes 84–99 in v2** (ICC.1:2022 has exactly one, version-agnostic header table, so "reserved in v2" is currently an *inference*); whether clause 8's requirements apply to v2 at all. It is also **the only normative home of `textDescriptionType`** | Pass 2 parses v2 profiles constantly. Note the current standing rule: **parse v2 profiles; do not declare them non-conformant against clause 8** |
| **ICC's published D65→D50 `chad` values** (cited by Annex E.4.2) | writing D65-referenced profiles without ambiguity | Pass 10 (profile creation) territory, and a second check on the adaptation path — the one place Pass 1's biggest hole could be partly filled from published values |

**Every one of those is a claim about what a document contains**, made by
the agent that read the corpus — not something anyone here has read.
Treat "it would settle A2" as a prediction until the document is open.

---

## Decisions already made — do not re-litigate

- **MIT**, dependencies permissive, **publishing is the operator's act**
  (rule 9). DL-009 records an *intent* to publish; **intent is not
  authorisation**, and no document here may be read as a go-ahead.
- **`iccce-color` depends on nothing** and contains no ICC. Still true
  after Pass 1 — check it stays true.
- **The parser reports, it does not repair.**
- **No iccMAX execution, no display calibration.** (The
  *profile-creation* refusal was **reversed by the operator** on
  2026-08-11 → Pass 10, DL-008, with the validation-hardware problem
  carried forward as its precondition.)
- **lcms2 is the oracle, never a dependency** — subprocess only, pinned
  by **commit hash** (DL-001; the tag is lightweight and therefore
  mutable). Moving the pin is a **licence event**.
- **The `pdfce` bridge is built in `pdfce`.** `iccce` must not know what
  a PDF is.
- **DL-003** — duplicate tag signatures: keep both, consumers take the
  first, report the duplicate.
- **DL-004** — the 1.0 ΔE2000 anchor is a conservative **design choice**,
  ⚠ provisional; anything derived from it inherits the ⚠. Note that
  **no Pass 1 row is graded against it** — the Sharma tolerance is
  arithmetic agreement, not perceptibility.
- **DL-005** — v2 legacy Lab tested by **exact-value invariants, not
  ΔE**: the error is ≈0.3–0.5 ΔE, *below* the anchor, so a ΔE-graded test
  passes while the encoding is wrong.
- **DL-007** — HDR in scope (Pass 9), transfer functions and primaries
  only; blocked on ITU-R documents, and on `icc-spec-librarian` first
  establishing that `itu.int`'s terms permit retrieval. *"It is a free
  download"* is not *"automated retrieval is permitted."*
- **DL-010** *(new)* — the Lab `f(t)` breakpoint uses the **exact
  rational** form: iccce's **first stated deviation from normative spec
  text**, cost **bounded analytically** at ~10⁻⁵ in `L*` and **never to
  be restated as measured**.
- **DL-011** *(new)* — legacy Lab encoding keys off the **tag type**;
  the lcms2 disagreement and the owed difftest above.

---

## Method reminders that stay load-bearing

1. **A wrong colour looks exactly like a right one.** The
   `mft2`-in-v4 case above is the live example: 0.39 % on `L*`,
   sub-perceptual, on most production CMYK profiles.
2. **Never write colour maths from memory.** Dispatch
   `icc-spec-librarian`; cite the corpus file, and now the clause where
   one genuinely exists.
3. **Expected values come from the literature.** A test whose expectation
   came from the code under test detects change, not error. Where only
   lcms2 is available, label it a **cross-check** — the ledger has a
   class for exactly that, and currently **zero rows in it**.
4. **Every approximation is named and measured** — `NUMERIC_CLAIMS.md`
   §4, and a cost of "unmeasured" is permitted only while the entry is
   new. **NA-002** (Bradford as policy) is on that clock: its cost
   becomes owed the moment a Pass 3 transform adapts anything.
5. **Tolerances are justified, not tuned.** When a test fails, the first
   question is whether the code is wrong. Pass 1's D50-chromaticity
   failure is the worked example: the code was right, the arithmetic was
   checked, and **the corpus was wrong**.
6. **Coverage is part of every claim.** "Verified on the 34 Sharma pairs"
   never becomes "verified".
7. **Do not assert unmeasured facts about the environment.** This
   project's documents distinguish *verified* / *reported* /
   *unverified* on purpose, and the distinction has already caught
   real errors.

---

## The agents

- **`icc-engineer`** — lead. Be this agent if orchestrating.
- **`icc-spec-librarian`** — the standards corpus. Dispatch for *every*
  sourcing question. **Owes DL-002's successor entry** and the D50
  chromaticity erratum fix.
- **`icc-conformance`** — the oracle, the fixtures, the tolerance budget.
  **Owes the legacy-Lab-in-v4 difftest**, and `TOLERANCES.md` §3.1/§5.
- **`icc-librarian`** — ROADMAP, decision log, session log, and
  `NUMERIC_CLAIMS.md`. **No shell** — a dispatch to it must carry its
  evidence.

Dispatch them freely; no permission is needed to dispatch an agent to
read, analyse or draft.
