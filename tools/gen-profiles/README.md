# `tools/gen-profiles` — the synthetic fixture generator

A standalone, zero-dependency Rust crate that writes ICC profiles **byte by
byte**, from layouts transcribed out of the specification corpus, and the
verification record for the corpus it produces in `fixtures/synthetic/`.

Owned by `icc-conformance`. Not shipped. Not a workspace member. Nothing under
`crates/` may depend on it, and it depends on nothing at all.

---

## 1. Why this exists

`docs/ROADMAP.md` records Pass 2's done-when clause 2 — *"a synthetic corpus
covers each tag type"* — as **PARTIAL**, and states the gap precisely: every
implemented tag type had byte-authored fixtures, but they lived **inside
`iccce-profile`'s unit tests**. Those are tag-level, not profile-level. They
cannot cover header/tag-table/tag-data interaction, cross-tag consistency, or
anything a consumer would open with `inspect`, and nothing outside
`cargo test` can use them — not a differential run, not a fuzzing run, not an
external consumer.

This crate is the other half. It produces **38 whole profiles on disk**, all
category (a) per `docs/LEGAL.md` §3 (authored here, no third-party content,
unrestricted, and each one says so in its own `cprt` tag).

### The property that makes the corpus worth having

From `fixtures/synthetic/README.md`:

> a synthetic fixture that cannot be regenerated is just a binary blob with
> better branding.

So:

* **Nothing reads a clock, an environment variable, or a random number.** The
  creation date in every header is a constant. Generation is deterministic and
  a unit test asserts it for every recipe.
* **`gen-profiles verify <dir>` is a subcommand, not a script.** It regenerates
  each recipe in memory, compares it with the file on disk, and names the first
  differing byte. A fixture edited by hand, corrupted in transit, or left behind
  by a generator change is a hard failure with a byte offset attached.
* **The manifest is generated** (`gen-profiles manifest`), so the invocation
  recorded beside a fixture is by construction the invocation that produced it.

### Why it has no dependencies — including on iccce

`tools/difftest` path-depends on `iccce-color`, and that coupling is justified
at length in its own manifest. **No equivalent justification exists here, and
none is intended.** A fixture written with the same encoder the parser was
written against cannot detect a *shared misreading of the specification* — the
two would agree, and the agreement would be worthless. `CLAUDE.md` rule 3 says
an expectation derived from the code under test detects change, not error; this
crate is the structural enforcement of that sentence.

---

## 2. Usage

```text
gen-profiles list                 # every recipe: name, category, size, coverage
gen-profiles <recipe> <out.icc>   # write one fixture
gen-profiles all <dir>            # write every fixture into <dir>
gen-profiles verify <dir>         # regenerate and compare, byte for byte
gen-profiles manifest             # emit fixtures/synthetic/MANIFEST.md
```

Regenerating the committed corpus:

```text
cd tools/gen-profiles
cargo run -- all ../../fixtures/synthetic
cargo run -- verify ../../fixtures/synthetic
cargo run -- manifest > ../../fixtures/synthetic/MANIFEST.md
```

Exit codes: `0` success (for `verify`, everything matched); `1` operational
failure **or a verification mismatch**; `2` usage error. The split between 1
and 2 lets a script distinguish "I asked wrongly" from "the corpus is wrong".

---

## 3. Where the bytes come from

Every layout is transcribed from `D:\Dev\Rag-Specialized\ICC_Spec\`, with the
clause number and the corpus file named in the doc comment at the site. The
evidence tier travels with it, because it is not uniform:

| Structure | Clause | Corpus file | Tier |
|---|---|---|---|
| Header, 128 bytes | 7.2, Table 17 | `icc__s__header.md` | `primary_spec` |
| Tag table, padding | 7.1.2, 7.3, 7.4 | `icc__s__tag_table.md` | `primary_spec` |
| Number encodings | 4.6–4.11, Tables 4–7 | `icc__s__number_encodings.md` | `primary_spec` |
| `curv`, `para` | 10.6 Table 35, 10.18 Tables 67/68 | `icc__type__curve_parametric.md` | `primary_spec` |
| `mft2`, `mft1` | 10.10 Table 40, 10.11 Table 44 | `icc__type__lut8_lut16.md` | `primary_spec` |
| `ncl2` | 10.17 Table 66 | `icc__type__text_mluc_namedcolor2.md` | `primary_spec` |
| PCSLAB encodings | 6.3.4.2 Tables 12/13; 10.10 Tables 42/43 | `icc__s__pcs_encoding.md` | `primary_spec` |
| `mluc` | 10.15 Table 54 | `icc__type__text_mluc_namedcolor2.md` | corpus code-derived; **re-read from the PDF for this crate** |
| `mAB `, `mBA ` | 10.12, 10.13, Tables 45/47 | `icc__type__lutAtoB_lutBtoA.md` | corpus byte tables **code-derived**; curve counts and permitted element sets **re-read from the PDF for this crate** (see §5) |
| `text`, `XYZ `, `sf32` | 10.24, 10.31, 10.22 | `icc__type__text_mluc_namedcolor2.md` | clause numbers verified; **layouts code-derived** |
| `desc` | ★ **none** | `icc__type__text_mluc_namedcolor2.md` §3 | ★ **no ICC.1:2022 clause exists** — removed in v4, defined in ICC.1:2001-04, **NOT OBTAINED** |

★ **`textDescriptionType` is the weakest thing in the corpus and it is used
anyway.** It is what every v2 profile on this machine actually carries, and the
`desc-short-mac-block` fixture turns a real-world finding into a regression
test. But its layout is unverifiable against any specification this project
holds, so a `desc` fixture must never be cited as evidence *about the standard*
— only about what implementations do with those bytes.

---

## 4. What the fixtures claim, and what they do not

★ **Nothing in this corpus is a colorimetric reference.**

The colorant columns are an arbitrary split of the *encoded* D50 white point,
chosen so their `s15Fixed16` integers sum to it exactly:

| | X | Y | Z |
|---|---:|---:|---:|
| `rXYZ` | 31 595 | 16 384 | 6 757 |
| `gXYZ` | 15 797 | 32 768 | 6 758 |
| `bXYZ` | 15 798 | 16 384 | 40 546 |
| **sum** | **63 190** = `F6D6h` | **65 536** = `00010000h` | **54 061** = `D32Dh` |

That gives a real *structural* invariant — device (1, 1, 1) reaches the PCS
white point exactly, with no rounding anywhere — and it describes no device.
Tone curves are exact powers of two or linear ramps; CLUTs are simple documented
functions of their grid indices. Every number was chosen to be checkable by
hand.

Using real sRGB primaries adapted to D50 was **rejected deliberately**:
`CLAUDE.md` rule 2 forbids writing colour maths from memory, the corpus holds
sRGB's chromaticities at `impl_crosscheck` tier only, and a Bradford adaptation
computed inside a fixture generator would be a numeric claim minted in the wrong
place. A number nobody can mistake for colorimetry is safer than a plausible
one.

**The one exception, flagged at its site:** `v4-rgb-para-type3` carries the
sRGB-shaped ICC type-3 parameters, provenance **one source** (lcms2 `cmsvirt.c`,
tier `impl_crosscheck`, because IEC 61966-2-1 is paywalled and was not
obtained). It is still **not an sRGB profile** — its primaries are the arbitrary
split above.

**Also worth knowing before using a fixture against lcms2:** lcms2's parametric
curve type = ICC `funcType` **+ 1**. `v4-rgb-para-type3` is ICC type 3, which is
lcms2 type 4.

---

## 5. ★ FINDING GP-001 — `mBA ` curve counts: iccce disagrees with ICC.1:2022 and with lcms2

**Status: FIXED same day — commit `2e98cfd` (2026-08-11): the profile
layer now counts per tag type (mBA B/M = input, A = output), the
fixture's `B2A0` decodes B=3/M=3/A=4 with 0 malformations, and a
`iccce-cmm` test cross-checks this README's own transicc value
(Lab(50,0,0) → K within 1e-3 of 49.6117%). Status line updated by
`icc-engineer`; the finding text below is preserved as filed.**

**Originally: open, filed 2026-08-11 by `icc-conformance`. This is a finding about
`crates/iccce-profile`, produced by the fixture corpus on its first run. The
fixture is correct and must not be changed to match the parser.**

### What happens

`fixtures/synthetic/v4-cmyk-mab-lab.icc` is a v4.4 CMYK Output profile whose
`B2A0` is a `lutBToAType` with `inputChan = 3` (Lab) and `outputChan = 4`
(CMYK). `iccce inspect` reports:

```text
tag[4].decoded: REFUSED: 'mBA ': curve chain broken at element 3 (byte 68); later elements unreachable
```

The same file's `A2B0` (`mAB `, 4 → 3) decodes correctly.

### Why the fixture is right

Read directly from `_sources/ICC.1-2022-05.pdf` on 2026-08-11 with
`pdftotext -layout`. VERBATIM:

* **10.13.2** — "There are the same number of "B" curves as there are **input**
  channels."
* **10.13.4** — "There are the same number of "M" curves as there are **input**
  channels."
* **10.13.6** — "There are the same number of "A" curves as there are
  **output** channels."

and, for the other direction:

* **10.12.2** — "A" curves = **input** channels;
  **10.12.4** — "M" curves = **output** channels;
  **10.12.6** — "B" curves = **output** channels.

So the rule is *not* "A goes with input". It is: **the curve set at the data's
entry side is counted by `inputChan` and the set at its exit side by
`outputChan`**, and which letter that is depends on the direction the tag runs.
For a CMYK `B2A0`: **B = 3, M = 3, A = 4.**

### Why iccce produces the refusal

`crates/iccce-profile/src/lut.rs`, `decode_lut_ab`, calls the chain decoder with
`output_chan` for both B and M and `input_chan` for A — **for both tag types**.
For `mAB ` that is correct. For `mBA ` it expects **four** B curves where the
specification puts three, so it walks past the third into the matrix element at
byte 68 and reports a broken chain. The two readings agree whenever
`inputChan == outputChan`, which is why nothing caught it before: the defect is
invisible on every square LUT and appears on **every real CMYK `B2A0`**.

`docs/ROADMAP.md` already predicted the blind spot in the Pass 2 clause-1
record — the 40-profile machine sweep is *"light or empty on the population
Pass 4 depends on — large v4 CMYK press profiles with `mAB `/`mBA `
pipelines"*. This fixture is that population, and it found the thing the sweep
could not.

### Where the corpus itself is insufficient (a named gap)

`icc__type__lutAtoB_lutBtoA.md` carries **one blanket sentence for both types**
— *"`A` curves = `inputChan`; `B` and `M` curves = `outputChan`"* — with its
byte layouts marked `icc_secondary_code` and **A23 open**. That sentence is
correct for `mAB ` and wrong for `mBA `, and it is the most likely origin of the
parser's behaviour. **The corpus file needs 10.12.2/4/6 and 10.13.2/4/6
transcribed per-type, and A23 can be closed at the same time** — clauses 10.12.1
and 10.13.1 enumerate the permitted element combinations verbatim, which A23
records as unsourced:

* `mAB `: `B`; `M, Matrix, B`; `A, CLUT, B`; `A, CLUT, M, Matrix, B`
* `mBA `: `B`; `B, Matrix, M`; `B, CLUT, A`; `B, Matrix, M, CLUT, A`
* both: "At least one processing element shall be included."

That is `icc-spec-librarian`'s file, so it is reported here rather than edited.

### Cross-check (weaker than the above, and consistent with it)

lcms2 2.19.1 at the pin, `src/cmstypes.c`: `Type_LUTB2A_Read` reads B and M with
`inputChan` and A with `outputChan`; `Type_LUTA2B_Read` reads A with `inputChan`
and M and B with `outputChan`. Behaviourally, `transicc` converts
**Lab(50, 0, 0) → CMYK(0, 0, 0, 49.6117 %)** through this fixture's `B2A0`,
i.e. it parses and evaluates the tag iccce refuses.

**This is a cross-check, not the ground truth.** The clause text above is the
ground truth; lcms2 agreeing with it is corroboration that two readers of the
standard read it the same way.

---

## 6. Verification matrix

**Scope of this record, stated so it cannot be rounded up.** One machine
(Windows 11, MSVC), one build of the consumer, one run, 2026-08-11. The
consumer is the **shipped binary** `target/release/iccce.exe`, built from the
working tree at commit `edce48b` (code identical to `d9e0b82`; the only working
-tree modification was `docs/ROADMAP.md`). `rustc 1.97.1`. The oracle column is
`transicc` from lcms2 **2.19.1** at the pin in `tools/difftest/lcms2.pin`.

Generator self-checks: `cargo test` **28 passed, 0 failed**;
`gen-profiles verify` **38 identical, 0 not identical**.

`git check-ignore -v` reports the negation `!fixtures/**/*.icc` for every
fixture and `git status --porcelain` lists all **38** as untracked (`??`) — i.e.
they are **trackable**, which is the property that matters and the one a bare
`check-ignore` exit code does *not* answer.

### 6.1 Well-formed — 11 of 12 fully as specified

| Fixture | iccce `inspect` | Verdict | transicc |
|---|---|---|---|
| `v4-rgb-matrix-trc` | exit 0, 0 malformations, 9 tags, `para funcType=0 params=2.000000`, colorants `0.4821/0.2500/0.1031` … | **PASS** | accepted |
| `v4-rgb-para-type3` | exit 0, 0 malformations, `para funcType=3 params=2.399994,0.947861,0.052139,0.077393,0.040451` | **PASS** | accepted |
| `v2-rgb-matrix-trc-curv` | exit 0, 0 malformations, 10 tags, `curve table n=9`, `sf32 n=9` identity, `desc`/`text` decoded | **PASS** | accepted |
| `v2-rgb-shared-trc` | exit 0, 0 malformations, `rTRC`/`gTRC`/`bTRC` all `offset=608 size=30` | **PASS** — legal full aliasing is not mistaken for an overlap | accepted |
| `v2-gray-curv-gamma` | exit 0, 0 malformations, `curve gamma=2` | **PASS** — the `count == 1` `u8Fixed8` trap | accepted |
| `v2-gray-curv-identity` | exit 0, 0 malformations, `curve identity` | **PASS** — the `count == 0` trap | accepted |
| `v2-cmyk-mft2-lab` | exit 0, 0 malformations, `lut16 in=4 out=3 clutPoints=3 …` and `lut16 in=3 out=4 …`, `matrixIdentity=true` | **PASS** | accepted; CMYK(0.1,0.2,0.3,0.4) → Lab 99.6002, −0.0000, −0.0000 |
| `v2-cmyk-mft1-lab` | exit 0, 0 malformations, `lut8 in=4 out=3 clutPoints=3` | **PASS** | accepted |
| `v4-rgb-mft2-lab` | exit 0, 0 malformations, `lut16 in=3 out=3 clutPoints=2` | **PASS** | accepted |
| `v2-rgb-mft2-lab` | exit 0, 0 malformations, identical summaries to the v4 twin | **PASS** — and the pair differs only in header bytes 8–11 (unit-tested) | accepted; both give Lab 0.1961 for the same input |
| `v2-ncl2-named` | exit 0, 0 malformations, `ncl2 colors=4 deviceCoords=4` | **PASS** | accepted |
| `v4-cmyk-mab-lab` | exit 0, 0 malformations, `lutAToB in=4 out=3 B=3 matrix=3x4 M=3 clut=grid=[5x4x3x2] prec=2 A=4`; **`B2A0` REFUSED** | ★ **FINDING GP-001** — `mAB ` correct, `mBA ` refused. Fixture is right per 10.13.2/4/6; see §5 | accepted **including `B2A0`**: Lab(50,0,0) → CMYK(0,0,0,49.6117 %) |

### 6.2 Malformed — 26 of 26 reported exactly as intended

Terminal refusals (exit 1, no partial result):

| Fixture | iccce message | Verdict |
|---|---|---|
| `bad-magic` | `not an ICC profile: magic at offset 36 is 'nope', expected 'acsp'` | **PASS** |
| `truncated-declared-size` | `truncated: header declares 712 bytes, only 648 present` | **PASS** |
| `too-short` | `not an ICC profile: 100 bytes, minimum is 132 (128-byte header + tag count)` | **PASS** |
| `iccmax-version` | `iccMAX (ICC.2) profile refused: version 0x05000000 is major version 5 — …` | **PASS** — refused **by name**, which is the requirement |
| `hostile-tag-count` | `tag count 4294967295 requires 51539607672 bytes of directory, file has 648` | **PASS** — refused before allocating; no OOM, no hang |

Reported-and-parsed (exit 0; the report is the deliverable):

| Fixture | Reported | Verdict |
|---|---|---|
| `header-reserved-nonzero` | 1 malformation: reserved bytes 100..128 not all zero | **PASS** |
| `trailing-bytes` | 1 malformation: 8 trailing bytes, noted as normal for container-embedded profiles | **PASS** |
| `rendering-intent-high-bits` | 1 malformation: intent `0x00010001` outside 0..=3 | **PASS** — read as 32 bits, not masked (ambiguity A7) |
| `tag-overrun` | 1 malformation: data extends past declared profile size | **PASS** |
| `tag-overlaps-table` | 2 malformations + 1 decode refusal | **PASS** — primary report correct; the extras are *entailed* by the same single mutation (see below) |
| `tag-misaligned` | 2 malformations + 1 issue | **PASS** — same entailment note |
| `tag-too-small` | 1 malformation + `REFUSED: tag data 4 bytes, minimum 8` | **PASS** |
| `tagbase-reserved-nonzero` | 1 malformation + 1 issue, **and the tag still decodes** | **PASS** — report alongside a value, per ARCHITECTURE §3.2 |
| `duplicate-tag-signature` | 1 malformation naming both indices and recording choice A13 | **PASS** |
| `mluc-record-size-16` | `REFUSED: mluc recordSize 16 (shall be 12): record layout unknown` | **PASS** |
| `mluc-odd-offset` | issue: `mluc record 0: odd string offset` | **PASS** |
| `curv-count-overflows-tag` | `REFUSED: 'curv': count 1000 exceeds tag data` | **PASS** — before allocation |
| `para-unknown-functype` | issue: `parametric funcType 9 not in 0..=4`, parameters kept raw | **PASS** |
| `text-not-ascii` | issue: `textType contains non-ASCII bytes (kept verbatim)` | **PASS** — reported, not transcoded |
| `text-unterminated` | issue: `textType lacks a terminating NUL` | **PASS** |
| `desc-short-mac-block` | issue: `desc: Macintosh ScriptCode block short or missing` | **PASS** — the *same* report the 40-profile sweep produced on this machine's EIZO profiles, now a regression test |
| `mft2-clut-size-exceeds-tag` | `REFUSED: 'mft2': LUT needs 107268620086877343846 bytes, tag has 68` | **PASS** — the needed size exceeds `u64`; the `u128` path is exercised, and nothing allocated |
| `mft2-pad-nonzero` | issue: `lut: pad byte(s) non-zero`, LUT still decoded | **PASS** |
| `mab-clut-precision-3` | `REFUSED: clut precision 3 (shall be 1 or 2)` | **PASS** |
| `mab-curve-chain-broken` | `REFUSED: 'mAB ': curve chain broken at element 2 (byte 56)` | **PASS** — position named (see the byte table in the recipe) |
| `xyz-trailing-bytes` | issue: `XYZType: 4 trailing byte(s) after last XYZNumber` | **PASS** |

**On the two fixtures that produce more than one report.** `tag-overlaps-table`
and `tag-misaligned` each carry exactly **one mutation**; the additional reports
are *entailed* by it and are derivable from the fixture's own bytes without
reference to any implementation. Pointing a tag at byte 132 means its "type
signature" is the first directory entry and its `icTagBase` reserved word is
that entry's offset field, which is non-zero; reading eight bytes from
`offset − 1` puts the trailing space of `'XYZ '` into the reserved word. Both
expectations were **refined after the first run against the bytes, not against
the output** — the distinction matters, and it is the difference between
correcting a prediction and tuning one.

**Also refined the same way:** `mab-curve-chain-broken` was predicted to break
at element 1/byte 44 and breaks at element 2/byte 56, because a truncated curve
element **swallows the following element's header** and comes out looking like a
valid identity curve. The byte-by-byte derivation is in the recipe's doc
comment. That cascade is a better fixture than the one intended, and it is
exactly why the report must name a byte position rather than an element alone.

### 6.3 The oracle column, and how much it is worth

`transicc` at the pin **accepts all 12 well-formed fixtures** and produces
plausible conversions through them. That is a cross-check: it says two
independent implementations read these layouts the same way, which is weaker
than the clause text and can be wrong in the same direction.

Its behaviour on the malformed set is **not** a verdict on iccce and is recorded
only as an observation, because `transicc` is a *conversion* tool that reads
lazily — a defect in a tag it never touches costs it nothing:

* Refused (exit 1): `bad-magic`, `too-short`, `hostile-tag-count`,
  `curv-count-overflows-tag`, `duplicate-tag-signature`, `para-unknown-functype`,
  `mab-clut-precision-3`, `mab-curve-chain-broken`,
  `mft2-clut-size-exceeds-tag`.
* Accepted (exit 0): the other 17 — including `truncated-declared-size`,
  `tag-overrun`, `tag-too-small`, `header-reserved-nonzero`,
  `mluc-record-size-16` and **`iccmax-version`**.

★ The last is worth one line on its own: **lcms2 does not refuse a major
version 5 profile**, where iccce identifies and refuses iccMAX by name. That is
a deliberate divergence, not a defect on either side, and `iccmax-version.icc`
is now the fixture that keeps it visible.

---

## 7. Coverage, stated honestly

**Covered, at profile level, by a well-formed fixture:** `curv` (all three
`count` cases), `para` (funcTypes 0 and 3), `text`, `desc`, `mluc`, `XYZ `,
`sf32`, `ncl2`, `mft1`, `mft2` (both directions, v2 and v4), `mAB `, `mBA `.
Classes `mntr`, `prtr`, `scnr`, `nmcl`. Spaces RGB, GRAY, CMYK. PCS XYZ and Lab.
Both PCSLAB encodings, on the tag types the specification assigns each to.
Legal full tag aliasing.

**Not covered, and named rather than implied:**

* **Tag types iccce does not implement**: `link`/`abst`/`spac` classes, `pseq`,
  `gamt`, `clrt`/`clot`, `meas`, `cicp`, `dict`, `mpet` (`D2Bx`/`B2Dx`), `ncol`
  (the obsolete v1 predecessor, which the corpus says to recognise and report).
  A generator can only usefully author what something can read.
* **`para` funcTypes 1, 2 and 4.** Types 3 and 4 are the discontinuous pair
  (nothing requires `cd + f == (ad + b)^g + e` at `X = d`, ambiguity A18) and a
  fixture carrying a *deliberately* discontinuous curve would be a good addition.
* **8-bit `mAB `/`mBA ` CLUTs** (`precision = 1`). Only `precision = 2` appears
  in a well-formed fixture; `precision = 3` appears in a malformed one.
* **Multi-record `mluc`** (two languages, shared string storage — explicitly
  legal), and `count == 0` (also explicitly legal).
* **`ncl2` with `nDeviceCoords == 0`** (legal; entry stride 38), and the
  `Ncl2DeviceCoordCountMismatch` issue, which needs a caller that holds the
  header.
* **Grids that stress interpolation.** Every CLUT here is small and every
  documented probe point lands on a grid node, deliberately: these fixtures test
  *parsing*, and ambiguity A16 (n-D interpolation is unspecified) is a Pass 4/5
  question that wants its own fixtures.
* **v2 conformance**. Clause 8's requirements are stated for 4.4.0.0 profiles
  and ICC.1:2022 does not restate v2's (**A34**), so the v2 fixtures here are
  built to *parse*, not to be conformance exemplars. `v2-cmyk-mft2-lab` in
  particular carries only `A2B0`/`B2A0` where clause 8.5.2 would require six
  transforms plus `gamt` of a v4 Output profile. A validator must not flag it.

**What the whole corpus does and does not establish.** It establishes that the
shipped `iccce` binary, on this machine, on 2026-08-11, reads 11 of 12
well-formed layouts as specified and reports 26 of 26 authored defects exactly
as intended, with one specification-backed disagreement recorded as GP-001. It
is **not** "iccce parses ICC profiles correctly" and must never be rounded up to
that: 38 files authored by one person from one corpus reading share whatever
that reading got wrong, which is precisely why the lcms2 column and the clause
citations are here.

---

## 8. Handover

* **GP-001** is `icc-engineer`'s to fix in `crates/iccce-profile/src/lut.rs` and
  `icc-spec-librarian`'s to close in `icc__type__lutAtoB_lutBtoA.md` (per-type
  curve counts; A23's permitted element sets). The fixture stays as it is.
* `docs/TOLERANCES.md` §3.2 (Pass 2) and §6's coverage table are
  `icc-conformance`'s and are **not** updated by this crate — §7 above is the
  material for them, and Pass 2 has no tolerance to state because parsing is
  exact or it is wrong.
* `tools/difftest`'s `legacy_lab_probe` can now be pointed at
  `v4-rgb-mft2-lab.icc` / `v2-rgb-mft2-lab.icc` instead of writing its own
  profiles into a git-ignored directory — the CLUT corner values are identical
  by construction, so DL-012's measurement stays reproducible from committed
  bytes. That port is deliberately **not** done here: `tools/difftest` belongs to
  another instance of this role and is being edited concurrently.
