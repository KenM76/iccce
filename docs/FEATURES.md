# FEATURES — what iccce can actually do today

**Created 2026-08-18.** This file did not exist before; `README.md`
§Status carried a Pass-by-Pass summary, which answers *"how far along is
the plan?"* but not *"can I call this from my program tomorrow?"* This
file answers the second question, and it is written for a **consumer**
— principally `pdfce`, which is named first in `README.md` — deciding
what to adopt.

## ★ How to read this file, and the one thing that makes it different

Every row carries an **evidence class**, because in this project a
capability that works and a capability that is *known* to work are
different claims, and a wrong colour looks exactly like a right one.
The three classes, weakest to strongest, are the project's standing
vocabulary (`CLAUDE.md` rule 3):

| class | what it means | what it does NOT mean |
|---|---|---|
| **self-comparison** | iccce agreed with iccce — e.g. the compiled path matches the reference path | nothing at all about whether either is colorimetrically right |
| **cross-check** | iccce agreed with **lcms2**, our pinned oracle, within a stated tolerance | that either is right. lcms2 is an implementation, not the standard (rule 7) |
| **derived-expectation** | iccce agreed with a value computed **by algebra** from a fixture's own authored bytes | that a real profile behaves this way |
| **published-ground-truth** | iccce agreed with a number a standards body published | — |

★★★ **iccce has NO `published-ground-truth` row for any transform, and
that is the single most important sentence in this file.** Every
conversion claim below is a cross-check, a derived expectation, or a
self-comparison. The reasons are structural and are recorded in
`docs/NUMERIC_CLAIMS.md` (DL-041, and the 2026-08-18 sourcing pass):
ICC.1 mandates no interpolation method, so no expected LUT value could
be published even in principle; and IEC 61966-2-1's constants sit behind
a paywall that free restatements (W3C CSS Color 4, web-platform-tests)
cannot substitute for, because they are downstream of the same document
and assert three orders of magnitude coarser than the question needs.

**A consumer should therefore read "cross-check against lcms2" as: iccce
will not surprise you relative to the engine you were probably already
using — not as: this is measured against truth.**

---

## 1. Command-line surface

Built from `crates/iccce-cli`. Binary name `iccce`.

### `iccce inspect <profile>`

Prints the header field by field, then the tag table, decoding each tag
type it knows, then a malformation list.

- Reads **raw bytes** — any file or stream dump containing an ICC
  profile is accepted; there is no container parsing and no sniffing.
- **Reports malformations; never repairs them** (`CLAUDE.md` rule 6). A
  silently corrected tag would hide the malformation from the only layer
  able to disclose it.
- ★ **`malformations: N` is a count of DISCLOSURES, not of violations,
  and is not a conformance verdict.** At least two variants are
  documented non-violations (`TrailingBytes`, which is normal for
  container-embedded profiles; and the v2 unrecognised-intent report).
  A consumer must not treat `N > 0` as "this file is broken".

### `iccce transform --src <p> --dst <p> [--intent i] [--bpc] [--preserve-black policy]`

Reads device values from stdin, one set per line, floats in `0..1`,
whitespace-separated, count = source channel count. Writes one converted
set per line at 6 decimals.

- `--intent` — `media-relative` (default), `perceptual`, `saturation`,
  `absolute`.
- `--bpc` — black point compensation. **Opt-in and never forced**;
  refused **by name** at absolute intent and outside the estimation
  subset, rather than silently ignored.
- `--preserve-black <policy>` — **required argument, no default.** See
  §3.

### `iccce bench --src <p> --dst <p> [--grid N] [--pixels N]`

Times a page-sized conversion through the compiled path. Defaults to a
300 DPI A4 raster (2481×3507 = 8 700 867 px) and to the recommended grid
for the source's channel count (33 for 3- and 4-channel, 129 for 1- and
2-). Prints build time, convert time, throughput, **and the compiled
path's off-node error against the reference path** — the last of these
is the point, and is why the benchmark is not merely a timing tool.

---

## 2. Library surface, by crate

| crate | owns | depends on |
|---|---|---|
| `iccce-color` | CIE colorimetry | **nothing** — no siblings, no third-party crates. This is a load-bearing invariant, not an accident |
| `iccce-profile` | ICC v2/v4 parsing | — |
| `iccce-cmm` | transforms | `iccce-color`, `iccce-profile` |
| `iccce-measure` | CGATS/IT8.7 measurement files | — |
| `iccce-cli` | the shell | all of the above |

**The whole dependency tree is five packages and all of them are ours.**
That satisfies `pdfce`'s three hard gates today — no copyleft anywhere,
no network client, and nothing that would break a `wasm32` build. ★ But
that is a **dated observation, not a standing guarantee**, and **iccce
does not gate `wasm32` in CI** while `pdfce` does. Adding a dependency
here can silently break a consumer's CI gate.

### Colorimetry — `iccce-color`

**Implemented:** XYZ, xyY, Lab, LCh; Bradford chromatic adaptation;
ΔE76; ΔE2000 (CIEDE2000).

**Deliberately absent, and each absence is a decision on the record:**
ΔE94 and ΔE CMC(l:c) are not implemented; CAT02 is not sourced; the
observer colour-matching-function tables are absent; and **the von Kries
(HPE) cone matrix present in the tree is a placeholder marked DO NOT
USE.** That last one is a trap for anyone reading the source for
available matrices.

### Parsing — `iccce-profile`

Header and tag table for **ICC v2 and v4**. Tag types decoded include
`text`, `desc`, `mluc`, `XYZ`, `curv`, `para`, `mft1`, `mft2`, `mAB`/`mBA`,
`ncl2`. iccMAX (ICC.2) is **identified and refused**, not parsed.

★ **The rendering-intent report is edition-specific** (landed
2026-08-18). ICC.1:2022 7.2.15 requires the high 16 bits to be zero;
ICC.1:2001-04 6.1.11 imposes nothing on either half, and its high half
is vendor space by the same construction 6.1.8 uses for the profile
flags. So the same bytes are reported differently — and in v2 a
high-half value is **not reported at all**, because doing so would make
a false statement about that edition.

### Transforms — `iccce-cmm`

Matrix/TRC and LUT paths; all four rendering intents; a compiled
(grid-interpolated) fast path; black point compensation; K-only black
preservation. See §3 and §4.

**★ Three evaluation surfaces, and which one to call.** They are not
interchangeable and the difference matters at raster scale:

| call | shape | use it for |
|---|---|---|
| `Chain::convert` | `&[f64] -> Result<Vec<f64>, ChainError>` | **build time and diagnostics only.** It allocates a `Vec` per call, so it is *not* callable from a per-pixel loop |
| `CompiledTransform::convert` | `&[f64] -> &mut [f64]`, `-> bool` | one pixel, no allocation on the ordinary path |
| `CompiledTransform::convert_buffer` | `n` interleaved pixels | **the raster surface.** Returns `false` on a shape mismatch rather than panicking |

★★ **`convert_buffer` is `convert` in a loop, and that is a contract, not
an implementation detail.** It delegates rather than reproducing the
body. From its introduction until **2026-08-21** it did reproduce the
body, and consequently **never applied the K-preservation policy at
all** — a caller who asked for preservation received it one pixel at a
time and silently did not receive it for a buffer. Measured at the fix:
**`7.195269e-1` of ink** at `[0, 0, 0, 1]` on `v2-cmyk-chromatic-neutral`
at grid 17, with the non-qualifying control at **exactly `0.0`**.

★ **The reason it survived so long is worth a consumer's attention more
than the bug is:** the only in-repository caller of `convert_buffer` is
`iccce bench`, and `bench` cannot request black preservation — so the
defect was **unreachable from the CLI, and `difftest` drives the CLI**.
*A defect that only a library consumer can reach is invisible to a
CLI-driven suite, however green it is.* The guarantee is now asserted by
bit-equality in
`crates/iccce-cmm/tests/compiled_buffer_agrees_with_single_pixel.rs`.

### Measurement — `iccce-measure`

Reads CGATS/IT8.7 measurement-data files. No ICC, no colour maths.

---

## 3. Named policies — the things that are NOT conformance features

These are places where iccce does something ICC.1 does not specify. Each
is a **named choice with the alternative stated**, never a default that
quietly picks a side.

### K-only black preservation

`--preserve-black <policy>`, **and the argument is mandatory.**

There is no default **because two published definitions disagree by up
to `4.9e-2`**, so a default would be iccce choosing one and reporting it
under a name that means both.

| policy | status |
|---|---|
| `k-only-equal-lightness` | **Implemented.** lcms2's mapping — a vendor construction with no normative text behind it, and also this project's own oracle |
| `k-only-ratio` | **Refused by name, not silently absent.** Cholewo (2000), genuinely published — but its `K_MIN`/`K_MAX` determination is not held here, and approximating it would be indistinguishable from implementing it |

**Measured:** exactly zero chromatic ink on all ten CMYK destinations in
the licensed corpus, at a `5×10⁻⁷` observation floor and structurally
exact in the code. K is genuinely re-mapped rather than copied through —
`0.366689` at `K_in = 0.5` on the furthest cross-press pair.

★★★ **What the policy COSTS, measured 2026-08-19** *(corrected in place —
this paragraph previously read "the number a caller actually wants is
UNMEASURED", which was true when written and is now false)*:

> **`3.681203` ΔE2000 max, `1.580674` mean** between the preserved and
> the colorimetric answer, over the 101-point qualifying set, on
> **`ISO Coated v2 300% (ECI)` → `GWG_GenericCMYK`**, **media-relative**,
> **no black-point compensation**. `NC-269`, `NUMERIC_CLAIMS.md` §3.36;
> registered against `NA-012`.

**Read the number with all four of these, because each of them changes
what it means:**

- **Its oracle is ITSELF.** This is a **self-comparison** — iccce's
  preserved answer against iccce's own colorimetric answer. ICC.1
  specifies **no** black-preservation construct, so no published value
  for what preservation *should* return can exist. That is the ceiling
  the question admits, not a gap in the measurement.
- **It belongs to an ORDERED pair.** The same two profiles **reversed**
  cost `2.432964` — the price is a property of the **destination's**
  black, not of the pair.
- **A population, not one pair:** over all 30 ordered pairs of six real
  CMYK profiles, **11 are entitled to price the policy and none of them
  find it imperceptible** (cost `2.023698` … `3.686985`). ★ Nine of the
  thirty would have looked *"nearly free"* without the entitlement gates.
- ★★ **What you will actually SEE is the boundary step, `3.712251`** —
  larger than the cost. The policy fires at `C = M = Y = 0` **exactly**,
  so a K-only-to-rich-black gradient steps once, visibly, one 8-bit code
  of cyan outside the qualifying set.

★ **Do not substitute the same-press figure** (`0.159500` on two profiles
whose `A2B1` tags are byte-identical, or `1.360900e-1` on a
same-*profile* pair): that regime is **`23×` cheaper** and is where the
policy is nearly a no-op.

★★ **What is still unmeasured:** the cost at any other intent, with
`--bpc`, or off the `C = M = Y = 0` axis — and **any weighting by how
often a real document contains K-only content**, which is the term that
decides whether the policy is worth its price and which belongs to the
consumer, not to this library.

### The default rendering intent

iccce defaults to **media-relative**. ICC.1's only word on a default is
an **informative NOTE** (9.2.39) and it favours **perceptual**, which is
lcms2's choice. Neither is required or forbidden — so this is a
disclosed divergence, not a standard being followed.

### The profile header's rendering-intent field is not consumed

Two profiles differing in **exactly one byte** (the header
`renderingIntent`) produce **bit-identical** output. Intent comes from
the caller.

Sourced: ICC.1:2022 7.2.15's `shall` binds **the field**, not a CMM, and
the content it mandates recording is a `should`. Clause 8.10.2 — which
defines how a CMM selects a transform — is **silent** on where the
intent comes from. Corroborating: 7.2.18 excludes those bytes from the
profile ID, so two profiles differing only here share a fingerprint.

★ **Untested class: DeviceLink.** ICC.1 8.6 makes the field normative
for `link` profiles. It still selects nothing there (a DeviceLink has
one transform), so consuming it and ignoring it produce identical
pixels — they differ only in whether a caller/header **mismatch is
disclosed**. No `link` fixture exists in either repository.

---

## 4. What is measured, and to what

Full detail in `docs/NUMERIC_CLAIMS.md` (numbered claims through
**`NC-280`**) and `docs/TOLERANCES.md` (every tolerance with its
justification). Summary — ★ **each figure below is dated, because an
apparatus count goes stale silently:**

- **202 workspace tests**, green. *Counted 2026-08-21 by running
  `cargo test --workspace` and summing every `test result:` line — **not**
  carried forward from the previous figure. The stated `190` was itself
  measured on 2026-08-19 at `3c93b62` and had already decayed twice by
  the time it was re-checked: Pass L added tests after it was written,
  and four more landed with the `convert_buffer` fix above.*
  ★ This is the reason each figure here carries a date: **the number was
  right when written and wrong when read**, which is the failure mode
  the project's decision log calls `DL-062`.
- **Differential suite vs pinned lcms2 2.19**: **`pass=353 fail=0 skip=9
  error=0`** with the licensed corpus present *(2026-08-19, `3c93b62`;
  was `pass=337` at `60c32dd` before Pass K §G added 16 rows)*.
  ★ **Without the corpus the last measurement is `pass=184 fail=0
  skip=94`, dated `60c32dd` and NOT re-measured since** — §G added 7 rows
  that do run in CI, so that pair of numbers is known to be stale and is
  **not** silently arithmetically updated here. CI is permanently in the
  skipping case, by design.
- **47 synthetic fixtures** *(2026-08-19 — `gen-profiles verify` reports
  all 47 identical)*, byte-generated by `tools/gen-profiles` so every
  expectation is derivable from authored bytes rather than from a vendor
  file.

**Tolerances are justified, never tuned.** *"Within 1 ΔE2000 because that
is the accepted perceptibility threshold"* is a tolerance; *"within 0.5
because it passed"* is a number someone moved until the suite went
green. When a test fails here, the first question is whether the code is
wrong.

---

## 5. What iccce does NOT do — read this before adopting

- **Overprint. This is `pdfce`'s, not ours** — it is *compositing, not
  conversion*, deciding which colorant channels a paint operation writes
  to, in a device space, *before* any conversion. iccce supplies the
  conversion at the end of that pipeline and owns none of the
  compositing in front of it. This is the boundary most likely to be
  mis-filed.
- **Finding a profile inside a PDF.** Container parsing is the
  consumer's.
- **★★ Supply a space to COMPOSITE in.** The built-in sRGB destination
  is a **destination**, not a working space. **ISO 32000-2:2020 §11.7.2
  NOTE 4** says the CIE-based sRGB space *"is nonlinear and hence can be
  unsuitable for use as a group colour space"* — blending computes
  linear combinations and assumes linear components. ★ The `should` is
  **normative body text in 2.0 and an informative NOTE in 2008**, so
  whether a processor is deviating depends on the edition. **Disclosed,
  not guarded**: iccce cannot see what a caller does with the model it
  returns. Full statement at `crates/iccce-cmm/src/builtin.rs` and
  `docs/DEFAULT_DESTINATION.md` §5.5.
- **★ Evaluate in `f32` or `u8`.** Every evaluation surface is `f64` —
  measured 2026-08-21, `grep -rn "f32" crates/iccce-cmm/src` returns
  **nothing**. A consumer whose raster is `f32` (which a compositing
  buffer generally is) pays a widen on the way in and a narrow on the
  way out for every pixel. **This is a known, requested and unbuilt
  gap**, asked for by `pdfce` on 2026-08-18; three of the four
  capability gaps named in that exchange are now closed and this is the
  fourth. What a narrower surface would and would not buy is settled
  and is *not* a measurement: it **solves the memory** (268 MB → 67 MB
  in, 201 MB → 25 MB out for a 300 DPI A4 CMYK raster) and **does
  nothing for the time**, because the ~1.4 Mpix/s is grid evaluation
  and is per-pixel however the pixel arrived. Deferred to Pass 6 under
  the project's *optimise only after correct* rule.
- **Profile creation** from measurement data (Pass 10, not sized).
- **HDR** — BT.2100 PQ/HLG, BT.2020/2100 primaries (Pass 9, blocked on
  standards sourcing).
- **iccMAX / ICC.2** — identified and refused, not implemented.
- **Any published claim of Ghent PDF Output Suite compatibility.** The
  suite is used privately to check behaviour; **publishing a pass/fail
  claim, a screenshot, or the word "Ghent" in release material needs
  GWG's written permission** and is an operator decision.
- **Any claim of validation against web-platform-tests.** Its 3-Clause
  BSD licence clause 3 independently bars that wording.

---

## 6. Status and stability

**Version `0.1.0` in `Cargo.toml`.** ★ `README.md` §Status says `0.0.1`
as of 2026-08-18 — one of them is stale and the manifest is
authoritative.

**Nothing has been published to crates.io.** All six crate names were
unregistered when checked on 2026-08-17 — *a dated observation, not a
reservation*; re-check immediately before any publish. **The API is not
stable and no compatibility is promised between commits.** Publishing,
pushing and tagging are the operator's acts.

**Licence: MIT**, and every dependency is permissive because every
dependency is ours.
