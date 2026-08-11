# `tools/difftest` — the differential oracle

**Status: Pass 0, partially complete.** The oracle is pinned, built and
demonstrated to answer questions. The Rust harness that will drive it is
**not written yet**; this directory currently contains the pin, the
fetch/build scripts, and the recorded evidence that they work.

---

## 1. What this is, and why it is out of tree

`iccce` computes colour. A wrong colour looks exactly like a right one —
nothing about a 3 ΔE error announces itself, which is the founding
observation of this project (`CLAUDE.md` rule 1). So the engine needs an
independent answer to compare against, and that is what lcms2 is here.

**lcms2 is the oracle. It is not a dependency.** The distinction is the
whole design:

| | oracle | dependency |
|---|---|---|
| invoked as | a separate process, from a test | linked into the library |
| appears in | nothing that ships | `Cargo.toml` of a shipping crate |
| if it disappears | tests skip, engine unaffected | engine does not build |
| licence exposure | none — we ship no part of it | full |

Concretely: **no crate under `crates/` has, or may acquire, an lcms2
dependency — not in `[dependencies]`, not in `[dev-dependencies]`.** The
harness in this directory shells out to `transicc.exe` and parses its
stdout. This mirrors the pattern `pdfce` uses for `oxidize-pdf`.

Three things follow from that, and each is load-bearing:

1. **Independence.** An oracle you link is an oracle you can accidentally
   agree with. A subprocess boundary makes it impossible for our code to
   influence its answer.
2. **Licence isolation.** lcms2's core is MIT but two of its plugins are
   GPL-3.0 (see §3). We ship none of it and build neither plugin.
3. **Reproducibility.** The oracle is identified by a commit hash that is
   committed here, while its source is not (see §2).

### What "oracle" does and does not mean here

**lcms2 is an implementation, not the standard.** Agreement with it is
evidence that two independent implementations read the specification the
same way. That is *weaker* than a published CIE reference value, and it
is occasionally wrong in the same direction — two implementations can
share a misreading.

So every difftest must be labelled with what it is:

- **ground truth** — a published CIE or vendor value, transcribed with
  its source. Pass 1's colorimetry tests are these.
- **cross-check** — agreement with lcms2. Everything in this directory
  produces cross-checks and nothing else.

And when the two disagree, per `CLAUDE.md` rule 7: **that is a finding,
not a failure.** Settle it from the specification text (dispatch
`icc-spec-librarian`) and record the outcome either way. A case where
iccce is deliberately right and lcms2 is wrong is worth more written down
than silently tolerated.

---

## 2. The pin

The single source of truth is [`lcms2.pin`](lcms2.pin). At the time of
writing:

| Field | Value |
|---|---|
| Upstream | `https://github.com/mm2/Little-CMS.git` |
| Tag | `lcms2.19.1` |
| **Commit (this is the pin)** | **`21c582a594fe5279f90c0b93437c398f93bf62b0`** |
| Commit date | 2026-05-06 |
| Reported version | `LittleCMS 2.19` (`LCMS_VERSION 2190`) |
| Licence, as verified | MIT for what we build — see §3 |
| Licence checked | 2026-08-11 |

**The commit hash is the pin; the tag is a label.** `lcms2.19.1` is a
*lightweight* tag — a mutable pointer with no tagger and no signature. If
upstream moves it, `git clone --branch lcms2.19.1` would silently hand us
a different tree, and every result previously attributed to "lcms2.19.1"
would become unreproducible. `fetch-lcms2.sh` therefore checks
`git rev-parse HEAD` against `LCMS2_COMMIT` after cloning and **exits 4
on mismatch**. That check is not a nicety; it is the thing that makes
"verified against lcms2 2.19.1" a falsifiable claim.

### The source is not vendored

`tools/difftest/vendor/` is git-ignored. The pin is committed; the source
is fetched on demand. Reasons, in increasing order of importance:

1. lcms2 is a third-party codebase we neither own nor maintain, and an
   MIT colour engine should not carry a second colour engine in its
   history.
2. **GPL-3.0 source never enters this repository.** See §3.

---

## 3. Licence — verified, and not as simple as the badge says

Full record with verbatim transcriptions: **`docs/LEGAL.md` §4.**
Summary:

- **Top-level `LICENSE` is verbatim MIT**, "Copyright (c) 2023 Marti
  Maria Saguer", unmodified OSI wording, no extra clause. Read from the
  cloned tree at the pinned commit on 2026-08-11 — not from GitHub's
  licence badge.
- **`src/` and `include/` are MIT**, with the permission notice repeated
  inline in each file header.
- **`plugins/fast_float/` and `plugins/threaded/` are GPL-3.0-or-later.**
  Upstream says so itself in `plugins/README.1ST`: *"LittleCMS core is
  released under MIT, but plug-ins may be released under other license.
  fast_float and threaded are GPL3"*.
- `utils/jpgicc/iccjpeg.c` is under the IJG licence. We do not build
  `jpgicc`.

**Both GPL plugins are OFF in every build here** — explicitly, via
`-DLCMS2_WITH_FASTFLOAT=OFF -DLCMS2_WITH_THREADED_PLUGIN=OFF`, even
though upstream already defaults them off, so that the intent is recorded
rather than inherited.

`fast_float` would have to stay off **even if it were MIT**: it replaces
lcms2's floating-point pipeline with a faster approximate one, and an
oracle must be the reference implementation's most accurate path. If the
oracle is itself approximate, every disagreement becomes ambiguous.

**Moving the pin is a licence event.** See `LEGAL.md` §4.3 for the
required re-verification steps.

---

## 4. Layout

```
tools/difftest/
  README.md          this file
  lcms2.pin          tag + commit + licence status. The source of truth.
  fetch-lcms2.sh     clone at the pin, VERIFY THE HASH, or fail
  build-lcms2.ps1    Windows / MSVC build   (exercised — see §6)
  build-lcms2.sh     POSIX build            (NOT yet exercised — see §7)
  vendor/            git-ignored
    lcms2/           the clone
    build-msvc/      Windows build output; transicc.exe lives here
    build-posix/     POSIX build output
```

---

## 5. Use

```sh
sh tools/difftest/fetch-lcms2.sh              # clone + verify hash
```

```powershell
# Windows
pwsh tools/difftest/build-lcms2.ps1 -RunTestbed
```

```sh
# Linux / macOS
sh tools/difftest/build-lcms2.sh --run-testbed
```

Neither build script improvises. If a toolchain is missing it names
exactly what is missing and exits non-zero, rather than producing
something that looks like a build.

Exit codes are documented in the header comment of each script.

---

## 6. The build, as it actually happened — Windows, 2026-08-11

Recorded because "it builds" is a claim, and a claim needs the machine it
was made on.

| | |
|---|---|
| Host | Windows 11 Pro 10.0.26200, x86-64 |
| Toolchain | **MSVC**, `cl.exe` 19.44.35228.0, toolset 14.44.35207, x64 |
| Sourced from | Visual Studio **2022 Build Tools** at `C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools` |
| Generator | Ninja 1.12.1 (VS-bundled) |
| CMake | 3.31.6-msvc6 (VS-bundled) |
| Build type | `Release` |
| Result | **success**, 53/53 targets |

**There is no cmake, ninja, gcc, clang or `cl.exe` on this machine's
PATH.** All three of cmake, ninja and the compiler were found inside the
Build Tools installation; `build-lcms2.ps1` locates them via `vswhere`.
Note that `vswhere` is queried with
`-requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64`, because
this machine also has a Visual Studio 18 Community instance **with no C++
workload** — "a Visual Studio is installed" and "a C compiler is
installed" are different claims, and picking the wrong instance fails
confusingly several minutes into a configure.

One harmless message appears during configure and is expected:

```
'vswhere.exe' is not recognized as an internal or external command
```

That comes from `vcvars64.bat`'s own internal probing, not from our
script, and does not affect the result.

Artefacts produced in `vendor/build-msvc/`:

```
transicc.exe   310,272 bytes
linkicc.exe    276,480 bytes
psicc.exe      283,136 bytes
lcms2.lib      (static; no DLL — LCMS2_BUILD_SHARED=OFF)
testbed/testcms.exe
```

Static linking is deliberate: `transicc.exe` is self-contained, so no
test can accidentally pick up a different `lcms2.dll` from elsewhere on
PATH at the moment it runs.

### Is the oracle sound? — lcms2's own self-test

An oracle that fails its own test suite is not an oracle. Run once at pin
time:

```
D:\Dev\iccce\tools\difftest\vendor\build-msvc\testbed> .\testcms.exe
```

```
  exit code : 0
  checks Ok : 157
  failures  : none
```

This is **evidence, not proof**: it establishes that lcms2 2.19.1 is
internally self-consistent on this build, and says nothing about whether
lcms2 reads the ICC specification correctly. A deeper run is available as
`testcms.exe --exhaustive` (upstream warns it "will take a while"); it
has not been run.

---

## 7. The POSIX build has NOT been exercised

`build-lcms2.sh` is written but **has never been run**. This machine has
no POSIX C toolchain — no `gcc`, no `clang`, no `make`, no `cmake` in Git
Bash. ROADMAP Pass 0 requires Linux CI, and the sibling project's lesson
was that an unchecked platform quietly stops compiling; the script exists
so CI has something to call.

Stated plainly here so nobody mistakes "a script exists" for "the Linux
build works." Whoever first runs Linux CI should replace this section
with the result.

---

## 8. Smoke test — the oracle answers questions

The point of a smoke test is not that it passes. It is that the oracle
demonstrably ran, on a real profile, and produced numbers a human can
recognise as colour.

### 8.1 The profile

`C:\Windows\System32\spool\drivers\color\sRGB Color Space Profile.icm`

**Category (c) per `LEGAL.md` §3 — read locally, never committed.** Tests
that use system profiles must skip when the file is absent; they may
never be a required input. The path is Windows-specific and is expected
to be missing on the Linux CI runner.

### 8.2 Invocation

`transicc` reads triplets from stdin, one component per line, and writes
the converted values to stdout. `-n` gives terse output suitable for a
pipe; `-t1` selects media-relative colorimetric; `*Lab` is lcms2's
built-in D50 v4 CIELAB space.

```sh
cd tools/difftest/vendor/build-msvc
printf '255\n255\n255\n' | ./transicc.exe \
    -i "C:\Windows\System32\spool\drivers\color\sRGB Color Space Profile.icm" \
    -o "*Lab" -t1 -n
```

Output, verbatim:

```
LittleCMS ColorSpace conversion calculator - 5.1 [LittleCMS 2.19]
Copyright (c) 1998-2026 Marti Maria Saguer. See COPYING file for details.
99.9988 0.0188 -0.0173
```

Note the two-line banner on **stdout**, before the numbers. Any harness
parsing this must skip it — take the last non-empty line, do not assume
line 1.

### 8.3 The full set, and why these numbers are recognisable

sRGB (system profile) → `*Lab`, intent 1 (media-relative colorimetric):

| sRGB in | L\* | a\* | b\* | why this is the right shape of answer |
|---|---|---|---|---|
| 255 255 255 | 99.9988 | 0.0188 | −0.0173 | media white maps to the PCS white, L\*≈100, a\*≈b\*≈0. The residue is the profile's own white-point quantisation, not an error. |
| 128 128 128 | 53.5836 | 0.0113 | −0.0104 | 128/255 = 0.502 → sRGB EOTF → ≈0.216 linear → L\* ≈ 53.6. Neutral stays neutral. |
| 255 0 0 | 54.2900 | 80.8198 | 69.8956 | sRGB red. The familiar D65 value is ≈(53.24, 80.09, 67.20); this is D50-adapted, as the ICC PCS requires, and lands where a Bradford adaptation puts it. |
| 0 0 0 | 0.0000 | 0.0000 | 0.0000 | black to black. |

**These are a smoke test, not a tolerance.** They are recorded so that a
future change to the toolchain, the pin or the profile shows up as a
diff. They are **cross-check values from an implementation**, not
published reference data, and must never be transplanted into an
`iccce-color` unit test as though they were ground truth — see
`CLAUDE.md` rule 3 and `docs/TOLERANCES.md`.

### 8.4 The LUT path also answers — CMYK, all four intents

Matrix/TRC (§8.3) exercises the analytic path only. This exercises
`A2B`/`B2A`, which is where Pass 4's risk lives.

`USWebCoatedSWOP.icc` → `sRGB Color Space Profile.icm`, CMYK
`0 100 100 0` (process red):

```sh
printf '0\n100\n100\n0\n' | ./transicc.exe \
    -i "C:\Windows\System32\spool\drivers\color\USWebCoatedSWOP.icc" \
    -o "C:\Windows\System32\spool\drivers\color\sRGB Color Space Profile.icm" \
    -t<N> -n
```

| `-t` | intent | R | G | B |
|---|---|---|---|---|
| 0 | perceptual | 237.2607 | 28.3697 | 36.1440 |
| 1 | media-relative colorimetric | 237.6654 | 51.3502 | 55.8132 |
| 2 | saturation | 237.2607 | 28.3697 | 36.1440 |
| 3 | ICC-absolute colorimetric | 207.3930 | 43.9027 | 44.9844 |

**Intents 0 and 2 are byte-identical, and that is correct — do not
"fix" it.** It is a property of the profile, not of lcms2 and not of any
fallback rule. Reading the tag table of `USWebCoatedSWOP.icc` directly:

```
profile version 2.1, 10 tags
A2B0  offset 432    size 41478
A2B2  offset 432    size 41478     <-- same offset, same size
A2B1  offset 41912  size 41478
```

`A2B0` and `A2B2` are two tag-table entries pointing at **one shared
block of tag data**. Perceptual and saturation are literally the same
transform in this profile. A difftest that flags "perceptual ==
saturation" as suspicious would be flagging the fixture, not the code —
and this is exactly the kind of thing that gets misdiagnosed at 2am, so
it is written down here.

Intent 3 differing from intent 1 in the direction it does (a darker,
less bright red) is the expected signature of absolute colorimetric:
media white is no longer mapped to the destination white, so the paper
white of SWOP shows through.

---

## 9. `transicc` notes for whoever writes the harness

Gathered while running the above; each of these would otherwise cost
someone an hour.

- **Usage is printed by running with no arguments.** `-h` is *not* a
  valid flag — `transicc -h` exits non-zero with
  `[transicc fatal error]: Unknown option`.
- **Flags take their argument attached**, with no space: `-i<profile>`,
  `-t<n>`, `-v<0..3>`. `-i profile.icc` is not the same thing.
- **The two-line banner goes to stdout**, mixed in with the data. Parse
  the last non-empty line.
- **Input is one component per line on stdin**, not a space-separated
  triplet on one line.
- **Default number range is device-native**, i.e. 0–255 for 8-bit RGB and
  0–100 for CMYK percentages. `-e` switches to encoded representation,
  `-w` to 16-bit, `-x` to hex. Pick one convention in the harness and
  state it; mixing them silently rescales everything.
- **Built-in profiles** save fixture work: `*Lab` / `*Lab4` (D50 v4
  CIELAB), `*Lab2` (D50 **v2** CIELAB — note the v2/v4 encoding
  difference, ARCHITECTURE §2's named hazard, is selectable right here
  and deserves its own fixtures), `*XYZ`, `*sRGB`, `*Gray22`, `*Gray30`,
  `*null`, `*Lin2222`.
- **Intents** beyond the ICC four: 10–15 are lcms2's black-preserving
  extensions (`preserving black ink` / `black plane`). **These are not
  ICC intents.** iccce implements 0–3; a difftest must not wander into
  10–15 and call the result a specification conformance check.
- Other flags that will matter later: `-b` black point compensation
  (Pass 5), `-c<0..3>` precalculation/precision (relevant to Pass 6's
  "what does flattening cost in ΔE?"), `-d<0..1>` observer adaptation
  state (absolute colorimetric only), `-p`/`-m`/`-g` soft-proofing.
- **`-c` is a knob that changes the answer.** lcms2's precalculated
  transforms are themselves an approximation of its own exact path. When
  a difftest tolerance is set, it must state which `-c` mode produced the
  reference numbers, or the tolerance means nothing.

---

## 10. Not done yet

- The Rust difftest harness. Nothing drives `transicc` programmatically.
- CGATS file I/O — `transicc` accepts `[CGATS input] [CGATS output]`
  positionally, which is the efficient way to push a whole corpus through
  in one process rather than one triplet at a time. Worth using once the
  corpus exists.
- Linux build, and therefore Linux CI (§7).
- The fixture corpus (`tools/gen-profiles`, `fixtures/synthetic`).
- Any actual tolerance. `docs/TOLERANCES.md` is a skeleton with one
  provisional anchor; nothing here has been compared to anything yet.
