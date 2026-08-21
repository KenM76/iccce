# iccce — an ICC colour management engine

A from-scratch, MIT-licensed **colour management module (CMM)** in Rust:
ICC profile parsing, transform construction, and pixel conversion. Built
as a standalone library with no PDF in it, because a CMM is a general
piece of infrastructure and tying it to one document format would be a
mistake that is hard to undo later.

Its first consumer is [`pdfce`](https://github.com/KenM76/pdfce), which
today has **no colour management at all** — that is the gap this project
exists to close.

---

## Why this is a separate project

`pdfce` needs ICC support for four things it currently cannot do:

- `ICCBased` colour spaces rendered as anything better than their
  `/Alternate` (which is the spec-sanctioned fallback, and visibly wrong
  on colour-managed artwork).
- `Separation` and `DeviceN` spot colours resolved through a real
  colorimetric path rather than a tint approximation.
- PDF/X **output intents** — the profile a print job is destined for,
  which is the whole point of PDF/X and currently ignored.
- Soft-proofing and separations preview, both of which need a CMM before
  they can mean anything.

It is separate rather than a `pdfce` crate for three reasons, in order of
weight:

1. **It is not PDF.** ICC profiles are used by image editors, print
   pipelines, cameras and displays. A CMM that can only be reached
   through a PDF library is a CMM nobody else can use, including the next
   project that needs one.
2. **The standards corpus is enormous and separate.** ICC.1, ICC.2
   (iccMAX), CIE colorimetry, the ITU-R display standards, the ISO
   measurement and target standards. Mixing that into `PDF_Spec` would
   make both harder to search.
3. **It has a different correctness bar and a different oracle.** PDF
   correctness is "does the file round-trip and render like Acrobat".
   Colour correctness is numeric: a transform is right or wrong to a
   measurable ΔE against reference values. That wants its own test
   apparatus.

---

## Scope

**In scope** — the engine:

- ICC v2 and v4 profile **parsing** (`ICC.1:2022`, and its ISO twin
  `ISO 15076-1`), every tag type a real profile uses.
- **Transform construction**: matrix/TRC profiles, LUT-based profiles
  (`lut8Type`, `lut16Type`, `lutAToBType`, `lutBToAType`), and the
  multi-profile chains a real conversion needs.
- All four **rendering intents** — perceptual, media-relative
  colorimetric, saturation, ICC-absolute colorimetric.
- **Black point compensation.**
- **Named colour profiles** (`namedColor2Type`) — directly relevant to
  spot colour, which is why `pdfce` wants this.
- The **CIE machinery** underneath: XYZ, Lab, chromatic adaptation
  (Bradford), ΔE (76, 94, CMC, 2000).
- Enough **performance** to be usable on a page-sized raster: cached
  transforms, and an evaluation path that does not re-walk the profile
  per pixel.

**Out of scope, deliberately** — say no now rather than drifting:

- **iccMAX (ICC.2) execution.** Parse and identify it so a file carrying
  one is not mistaken for corrupt; do not implement its calculator
  element. Revisit only if something real needs it.
- **Display calibration** — talking to hardware, loading video LUTs.
- **A colour picker, a UI, or anything user-facing.** This is a library.

**Moved out of "out of scope" — profile *creation***

This list previously read *"**Profile creation** from measurement data.
That is a profiler, a different product, and it needs measurement
hardware to validate."* **That firm no was reversed by the operator on
2026-08-11.** Profile creation is now **future scope**: planned as Pass
10 in `docs/ROADMAP.md`, positioned after the `pdfce` bridge, to be sized
when reached. Decision record, including the wording it reverses:
`docs/ARCHITECTURE.md` **DL-008**.

Recorded this way — as a change, with the date — rather than quietly
deleted, because a scope statement that silently contradicts an earlier
one is worse than either version of it.

**The rationale did not go away with the no.** "It needs measurement
hardware to validate" was an engineering fact, and it survives as the
open problem Pass 10 must solve first: a profiler's output is a claim
about a physical device, and neither self-consistency nor agreement with
lcms2 can tell you whether the profile describes the printer. Building
one before that is answered would produce exactly the plausible-looking
wrong result this project is organised against.

**Also added to scope on 2026-08-11:** HDR — the BT.2100 PQ/HLG transfer
functions and BT.2020/2100 primaries — as Pass 9 (`DL-007`); and
publication to crates.io as the intended end state (`DL-009`), which
authorises no push, tag or release on its own.

---

## The oracle

**Little-CMS (lcms2) is MIT-licensed**, which makes it usable both as a
differential-test oracle *and*, in principle, as a dependency. This
project builds its own engine anyway — that is the point — but lcms2 is
pinned out-of-tree as a reference implementation to diff against, the
same pattern `pdfce` uses for `oxidize-pdf`.

**That licence was verified 2026-08-11** (`docs/LEGAL.md` §4): the core
is verbatim MIT at the pinned commit — but not uniformly across the
repository. The `fast_float` and `threaded` plugins are GPL-3.0, per
upstream's own `plugins/README.1ST`. iccce is insulated three ways:
lcms2 is driven as a subprocess and never linked, neither plugin is
built, and the source is git-ignored so GPL code never enters this
tree. A licence claim nobody checked is exactly the kind of thing this
project family has been bitten by — this one was checked, and the check
found something.

---

## Status

**Early, and in active development.** Version `0.1.0` — read from the
workspace manifest on 2026-08-21, which is the authoritative source; this
line previously said `0.0.1` and had been wrong since the manifest moved.
**Nothing has been published to crates.io**, the API is not stable, and no
compatibility is promised between commits.

Work is organised as numbered Passes. `docs/ROADMAP.md` is the record and
states each Pass's verdict in its own words; the table below summarises
it. **Pass numbers are filing order, not schedule order.**

| Pass | Covers | State |
|---|---|---|
| 0 | Workspace, Linux+Windows CI, the lcms2 oracle pinned | **Done** |
| 1 | Colorimetry — `iccce-color` | **Core complete and validated**, deliberately not called "done" — see below |
| 2 | Profile parsing — `iccce-profile` | **Done** |
| 3 | Matrix/TRC transforms | **Done** |
| 4 | LUT transforms and rendering intents | **In progress** — saturation in `B2A`, and the ICC-absolute intent, remain |
| 5 | Black point compensation | **Done on stated terms**, and the terms are load-bearing — see below |
| 6 | Performance — compiled transforms | Code has landed; **its measurement is not yet filed in the ledger** |
| 7 | Named colours and spot colour | Code has landed and is reachable; **not yet measured** |
| 8 | The `pdfce` bridge — `ICCBased`, output intents | Built **in `pdfce`**, not in this repository |
| 9 | HDR — BT.2100 PQ/HLG, BT.2020/2100 primaries | Planned; blocked on standards sourcing |
| 10 | Profile creation from measurement data | Planned, not yet sized |

**What "not done" means, in the two places it is easy to over-read:**

- **Pass 1** is called core-complete rather than done because ΔE94 and
  ΔE CMC(l:c) are not implemented, the von Kries (HPE) cone matrix in the
  tree is a placeholder **marked DO NOT USE**, CAT02 is not sourced, and
  the observer colour-matching-function tables are absent. Bradford
  adaptation, XYZ/xyY/Lab/LCh, ΔE76 and CIEDE2000 are implemented.
- **Pass 5** means the BPC scaling map, its direction, and the pipeline
  it sits in — on **one synthetic fixture, at one intent, in two
  directions, on one platform, against one oracle pin**, and explicitly
  **not** the black-point *estimators*, which have never been
  discriminated.

## How correctness is established here, and what it is worth

`docs/NUMERIC_CLAIMS.md` is an append-only ledger of every stated
tolerance and every measured error, each tagged with an evidence class.
`docs/TOLERANCES.md` is the budget those numbers are graded against. Two
things a reader should take from them before trusting any figure:

- **Exactly one claim in this project rests on published ground truth**:
  CIEDE2000 agrees with all 34 pairs of the Sharma, Wu & Dalal (2005)
  dataset (*Color Research & Application* 30(1):21–30) to within the
  published data's own precision of 1×10⁻⁴.
- **No transform in this project has a ground-truth row.** Transform
  correctness is currently established by differential comparison against
  lcms2 at a pinned commit, by expectations derived from ICC.1:2022
  clause text, and by self-consistency checks — and the ledger labels
  which is which, because they are not worth the same. Two
  implementations agreeing is evidence that they read a clause the same
  way; it is not evidence that the clause was read correctly.

Every departure from exact colorimetry is named and its cost measured, in
`docs/TOLERANCES.md`. The deliberate divergences from lcms2 are recorded
the same way — including that **iccce never forces black point
compensation on**, where lcms2 does for a v4 destination at the
perceptual intent. Two defensible CMMs can produce different pictures by
default; this one says so rather than matching quietly.

## Repository map

| Path | What it is |
|---|---|
| `crates/iccce-color` | CIE colorimetry. Depends on nothing — no siblings, no third-party crates |
| `crates/iccce-profile` | ICC v2/v4 parsing. Reports malformations; repairs nothing |
| `crates/iccce-cmm` | Transform construction and evaluation; where every approximation lives |
| `crates/iccce-cli` | The `iccce` binary — `inspect`, `transform`, `bench` |
| `tools/difftest` | The lcms2 differential harness. **Deliberately not a workspace member**, so no shipping crate can link the oracle |
| `tools/gen-profiles` | The synthetic-fixture generator |
| `docs/` | `ROADMAP.md`, `ARCHITECTURE.md` (with the dated decision log), `NUMERIC_CLAIMS.md`, `TOLERANCES.md`, `LEGAL.md` |

**The workspace has no third-party dependencies.** `Cargo.lock` resolves
to the four workspace crates and nothing else.

## Licence

**MIT** — see `LICENSE`. Dependency licensing is covered by
`THIRD_PARTY_LICENSES.md`, generated by `cargo-about`; the analysis of
lcms2's licensing, including the finding that it is **not** uniformly
MIT, is in `docs/LEGAL.md` §4.
