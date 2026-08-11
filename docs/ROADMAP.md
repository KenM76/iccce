# iccce — roadmap

Passes, in dependency order. Each is sized to be finishable and
verifiable; a Pass that cannot be demonstrated is too big.

**Pass 0 is done (2026-08-11). Passes 1–8 are plan, not record.**

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

No ICC at all. XYZ, xyY, Lab, LCh; standard illuminants and observers;
Bradford and von Kries adaptation; ΔE 76/94/CMC/2000.

**Done when**: every function matches published reference values. This
Pass's tests are the foundation of every later Pass's credibility, so
their expected values must come from the literature, never from the
code.

## Pass 2 — profile parsing (`iccce-profile`)

Header, tag table, and the tag types real profiles use: `XYZType`,
`curveType`, `parametricCurveType`, `textType`/`multiLocalizedUnicode`,
`lut8`/`lut16`/`lutAToB`/`lutBToA`, `namedColor2`, `s15Fixed16Array`.

Report malformations, repair nothing. Identify iccMAX and refuse it by
name.

**Done when**: every profile on the machine parses or is refused with a
reason, and a synthetic corpus covers each tag type.

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

## Open questions for the operator

Recorded rather than decided, because they are scope calls:

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
- **(b)** How far into HDR? BT.2100 and PQ/HLG are a real body of work
  and only matter if something needs them.
- **(c)** Is a profile *creator* ever wanted? Currently a firm no; it
  changes the shape of the project if it becomes a yes.
- **(d)** Should `iccce` be published to crates.io? A general-purpose
  MIT CMM in Rust is a thing the ecosystem lacks; that is a reason to,
  and a maintenance commitment.
