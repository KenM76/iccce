# `tools/difftest` — the differential oracle

**Status: Pass 3 differential run, 2026-08-11.** The oracle is pinned, built
and demonstrated to answer questions; a Rust harness drives it
programmatically (**§11**); the first designed experiment (**§12**) settled
what `ARCHITECTURE.md` DL-011 left open and turned up a second, unrelated
version-keyed divergence on the way; and **§13 is the first comparison in this
repository between `iccce` and anything else.**

**Eight records are registered. One compares lcms2 against lcms2 (§11.3);
seven grade `iccce` (§13).** Of those seven, two are ungraded means whose
tolerance is literally `inf`. Said plainly here so that
`summary pass=8 fail=0` is not mistaken for eight independent proofs.

> **The two numbers ROADMAP Pass 3 asks for, up front** — full derivation and
> scope in §13:
>
> | | value | tolerance | kind |
> |---|---|---|---|
> | **iccce vs lcms2**, sRGB → Adobe RGB (1998) | **max 3.476×10⁻³ ΔE2000** (mean 5.114×10⁻⁴) | 2×10⁻² | cross-check |
> | **round trip** sRGB → Adobe RGB → sRGB, iccce alone | **max 1.8788×10⁻² ΔE2000** (mean 8.674×10⁻⁴) | 2.5×10⁻² | self-consistency |
>
> Scope, which travels with them: **one profile pair, one intent
> (media-relative colorimetric), one direction, 133 grid points, one platform
> (Windows 11 / MSVC), one lcms2 build.**

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
  Cargo.toml         the harness crate. NOT a workspace member — see §11
  src/
    lib.rs           drive transicc AND iccce, parse them, grade the answer
    pass3.rs         the Pass 3 experiment: grid, tolerances, records (§13)
    pass4.rs         Pass 4: CMYK→RGB through an mft2 A2B, all four intents,
                     plus the mft2 pipeline reimplemented twice (§14)
    pass4b.rs        Pass 4b: the B2A (mft1) direction, the v4 mAB/mBA
                     fixture, and the F.2 grayTRC model (§15). Also carries
                     the ragged-grid CLUT, lcms2's reverse tone curve, and
                     the two closed forms
    main.rs          the runner; registers the checks (§11, §13, §14, §15)
    bin/
      legacy_lab_probe.rs   the DL-011 experiment (§12) — also authors the
                            synthetic probe profiles byte by byte
      pass3_report.rs       the per-point Pass 3 record, and the two
                            experiments that TEST §13's justifications
      pass4_report.rs       the same for Pass 4, incl. the interpolation
                            envelope and the white-point experiment (§14)
      pass4b_report.rs      the same for Pass 4b, incl. the tetrahedral
                            counterfactual and the reverse-curve
                            attribution (§15)
  out/               git-ignored; generated probe profiles land here
  target/            git-ignored
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

Output, verbatim (as it appears on a terminal, both streams together):

```
LittleCMS ColorSpace conversion calculator - 5.1 [LittleCMS 2.19]
Copyright (c) 1998-2026 Marti Maria Saguer. See COPYING file for details.
99.9988 0.0188 -0.0173
```

> **⚠ Correction, 2026-08-11 — the banner is on STDERR, not stdout.**
> The sentence that stood here said the two-line banner goes to stdout. It
> does not. Redirecting the streams to separate files shows stdout carries
> **only** the data line (with a trailing space and a CRLF):
>
> ```sh
> printf '255\n255\n255\n' | ./transicc.exe -i"…" -o"*Lab" -t1 -n -c0 \
>     >stdout.txt 2>stderr.txt
> # stdout.txt: "99.9988 0.0188 -0.0173 \r\n"
> # stderr.txt: the two banner lines
> ```
>
> The original claim came from watching both streams interleaved in one
> terminal, which is exactly the observation a harness makes impossible to
> keep making. This is small, and it is the first thing the harness found.
>
> The advice that followed it is still right, for a different reason: **take
> the last non-empty line of stdout**, never line 1. That is correct under
> either arrangement and costs nothing, so `parse_values` does it.

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
  `[transicc fatal error]: Unknown option`. Note that running with **no**
  arguments **exits 0** (measured 2026-08-11), so exit status alone cannot
  distinguish "printed usage" from "converted something"; only a parsable
  stdout line can.
- **Flags take their argument attached**, with no space: `-i<profile>`,
  `-t<n>`, `-v<0..3>`. `-i profile.icc` is not the same thing.
- **The two-line banner goes to STDERR** — corrected 2026-08-11, see the
  boxed note in §8.2. stdout carries only the data line. Parse the last
  non-empty line of stdout anyway.
- **`-o*Lab2` does NOT expose the legacy encoding.** Measured 2026-08-11:
  `*Lab`, `*Lab4` and `*Lab2` all print the identical triplet for the same
  input, because `transicc`'s float output is in `L*`/`a*`/`b*` units and
  lcms2 normalises across the built-in Lab profile's own encoding. So the
  obvious "compare `*Lab2` against `*Lab4`" experiment silently measures
  nothing. The v2/v4 encoding question has to be asked through a *profile's
  own tag*, which is what §12 does.
- **Input is one component per line on stdin**, not a space-separated
  triplet on one line.
- **Default number range is device-native**, i.e. 0–255 for 8-bit RGB and
  0–100 for CMYK percentages. `-e` switches to encoded representation,
  `-w` to 16-bit, `-x` to hex. Pick one convention in the harness and
  state it; mixing them silently rescales everything.
  > **★ Refined 2026-08-11 (Pass 4), because the mechanism is not where this
  > sentence implies.** The CMYK 0–100 convention does **not** come from
  > `transicc`'s `InputRange`, which `ComponentNames` sets to **1** for
  > `cmsSigCmykData`. It comes from **`cmspack.c`**, where the double
  > formatters scale by `IsInkSpace(fmt) ? 100.0 : 1.0`. The distinction
  > matters because it means the convention is a property of **lcms2's float
  > formatters**, not of the utility, and it applies to every ink space — so it
  > will hold for a 6-ink profile too, where the utility's own table would
  > have to be consulted separately. Measured: `0 1 1 0` gives near-paper
  > white (1 % ink) and `0 100 100 0` gives process red (§14.3).
- **Output is UNBOUNDED by default.** `lUnbounded` initialises to `TRUE`, so
  `transicc` prints values outside the output range unless `-q`/quantise is
  asked for. That is why §13.4 could see 1.000 120 at all; a harness that
  assumed clamping would have mis-attributed it.
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

- CGATS file I/O — `transicc` accepts `[CGATS input] [CGATS output]`
  positionally. **Partly obsolete as a motivation**: §13 pushes a 133-point
  grid through one `transicc` process on stdin (`Oracle::convert_batch`),
  which gets the same one-process-per-comparison property without a file
  format. CGATS remains worth using if a corpus ever needs to live on disk.
- Linux build, and therefore Linux CI (§7). The harness runs on Linux —
  it is `std`-only — but with no oracle it exits **3 (nothing ran)**, and
  §13's checks additionally need the Windows colour directory, so on Linux
  they skip regardless.
- The general fixture corpus (`tools/gen-profiles`, `fixtures/synthetic`).
  §12's probe writes profiles byte by byte inside the harness because Pass
  2's generator does not exist; when it does, port the probe onto it.
  **§13 uses no synthetic profiles at all** — both its profiles are category
  (c) system files, which is why every §13 check skips off this machine.
- **The reverse direction, Adobe RGB → sRGB.** Spot-checked by hand on
  2026-08-11 (§13.4) and not in the suite. It is the direction that would
  exercise a *genuine* gamut clip rather than §13's 1-lsb excursions.
- ~~**Any intent but media-relative colorimetric**, and any LUT profile.
  Pass 4.~~ **★ Done 2026-08-11 — §14**: `USWebCoatedSWOP.icc` (`mft2` A2B,
  4-channel, `Lab ` PCS) → the system sRGB profile, **all four intents**, 341
  points. What is still not done is the **B2A** direction (`mft1` here),
  `lut8Type`, `lutAToBType`, and any v4 or synthetic LUT profile.
- **Distinguishing clamp-before-TRC⁻¹ from clamp-after** (§13.6). iccce
  clamps at three sites, so the normative F.8–F.16 *ordering* is currently
  unobservable at the shipped surface. Recorded as owed.

Done since this list was written: ΔE metrics (§11.2 and §13.2 record the
decision and its limits), and the first comparison against `iccce` (§13).

---

## 11. The harness — what it is and what it refuses to do

`Cargo.toml` + `src/` is a **standalone crate, deliberately outside the
workspace**. The root manifest lists four members and not this one;
because this directory sits under the workspace root, cargo would refuse
to build it without the empty `[workspace]` table in its manifest, which
is the standard idiom and is preferred here to an `exclude` line in the
root manifest (which belongs to a different agent). The invariant it
protects is §1's: **no shipping crate may reach lcms2, even through
`cargo tree`.**

**Zero THIRD-PARTY dependencies, as policy.** Everything the harness does
for itself is `std`. The temptations declined were `serde` (machine-readable
output is hand-emitted TSV) and a CLI parser. `LEGAL.md` §1 requires
classifying every dependency; the cheapest classification is the empty set.

**Three path dependencies on iccce's own crates were added 2026-08-11 for
Pass 3** — `iccce-color`, `iccce-profile`, `iccce-cmm`. The decision, its
four-part justification and its limits are in `Cargo.toml`'s header and in
§13.2. The one-line version: the arrow points *harness → code under test*,
which is the normal shape of a test harness and leaves §1's invariant
(no shipping crate may reach lcms2) exactly where it was.

### 11.1 Run it

```sh
cd tools/difftest
cargo test                        # the harness's own unit tests (18)
cargo run                         # the registered checks (8 records)
cargo run --bin legacy_lab_probe  # the §12 experiment
cargo run --bin pass3_report      # the §13 per-point record + experiments
```

`cargo run` needs **`cargo build --release -p iccce-cli` to have been run at
the workspace root**, or §13's seven records skip with that as their reason.

Output is TSV:

```
check<TAB>id<TAB>status<TAB>kind<TAB>metric<TAB>tolerance<TAB>observed<TAB>detail
summary<TAB>pass=N<TAB>fail=N<TAB>skip=N<TAB>error=N
```

| exit | meaning |
|---|---|
| 0 | at least one check ran and everything that ran passed |
| 1 | a failure |
| 2 | a harness/oracle error |
| 3 | **nothing ran** — everything skipped, or no oracle on this machine |

**3 is not 0, and that is the most important line in this section.** A
run with no oracle skips every check; reporting that as success is how a
suite quietly stops testing anything. `SKIP` and `ERROR` are likewise
kept distinct from `FAIL`, and each carries its reason in `detail`.

### 11.2 What the types make impossible

Design choices worth stating, because each one closes a way this harness
could have produced a claim it does not support:

- **`Tolerance` cannot be built from a bare number.** It carries a `why`
  string (`CLAUDE.md` rule 5).
- **Every check states its `Kind`** — ground truth / cross-check /
  self-consistency / **oracle-reproducibility**. The last is new and is
  the honest label for "both sides are lcms2".
- **`Intent` has exactly the four ICC intents.** lcms2's 10–15 are its
  own black-preserving extensions; they cannot be expressed here, so no
  result from this harness can accidentally be described as conformance
  to something ICC.1 does not define.
- **`Precalc` and `Bpc` are required fields**, never defaulted. `-c`
  changes the answer (§9), and — §12 — lcms2 turns BPC on by itself for
  v4 profiles at perceptual and saturation, so a record that does not say
  what was asked for cannot be interpreted later.
- **~~No ΔE.~~ ★ Superseded 2026-08-11 — see §13.2.** This entry read: *"The
  only metric is `abs-max-component`. Adding ΔE would mean either depending
  on `iccce-color` (grading iccce with iccce's own arithmetic — a coupling
  that must be a documented decision, not a convenience) or writing a second
  ΔE2000 to get subtly wrong."* Pass 3 needs a perceptual statement, so the
  documented decision has been taken and the coupling exists. The kept half
  of the original entry: `ARCHITECTURE.md` **DL-005 still governs encoding
  questions — legacy-Lab correctness is asserted by exact-value integer
  invariants, never by ΔE**, and §12's probe still uses no ΔE at all.
  `Metric` accordingly keeps its absolute variants alongside the new ΔE ones.

### 11.3 The registered check, and exactly what it proves

| | |
|---|---|
| **id** | `smoke/srgb-white-to-lab` |
| **What** | system sRGB → `*Lab4`, media-relative colorimetric, `-c0`, input `255 255 255` |
| **Expected** | `99.9988  0.0188  −0.0173` — from §8.2, recorded 2026-08-11 from this same pinned oracle |
| **Kind** | **oracle-reproducibility** — *both sides are lcms2* |
| **Metric / tolerance** | `abs-max-component` / **1×10⁻⁴** |
| **Why that tolerance** | `transicc -n` prints four decimals and the recorded expectation is itself a four-decimal print, so agreement cannot be asserted more tightly than the reference is printed. **Arithmetic-agreement, not perceptual** — the 1.0 ΔE2000 anchor is irrelevant to it. For scale: the v2/v4 encoding error this project cares most about moves `L*` at white by ≈0.39, some 3900× this bound. |
| **Result, 2026-08-11** | **PASS, observed deviation 0.000000e0** (exact agreement) on Windows 11 Pro 10.0.26200, MSVC, `LittleCMS 2.19` |

Two things it does **not** establish, stated because a green line invites
the opposite reading: it says nothing about whether lcms2 is right, and
nothing about `iccce`, which is not in the loop. **These numbers must
never be transplanted into an `iccce-color` or `iccce-cmm` unit test as
expected values** — at that moment the claim would silently change from
"the oracle still answers the same" to "iccce is correct" (`CLAUDE.md`
rule 3).

Note also that §8.2 recorded its numbers with no `-c` flag while the
check passes `-c0`. Verified 2026-08-11: all of (no flag), `-c0`, `-c1`,
`-c2`, `-c3` print the identical triplet for this transform, so the
substitution does not change what is being reproduced.

The check **skips** where the system sRGB profile is absent — category
(c) under `LEGAL.md` §3, so on Linux CI this runner exits 3.

---

## 12. ★ The DL-011 experiment — measured, 2026-08-11

`ARCHITECTURE.md` **DL-011** decided that the legacy 16-bit PCSLAB
encoding keys off the **tag type** (`lut16Type`, `namedColor2Type`) and
never off `header.version`, per ICC.1:2022 **6.3.4.2 NOTE 3** and
**10.10**. It recorded, from the corpus, that **lcms2 keys the same
decision on the profile version** — and flagged that claim as
**unverified**, owing `icc-conformance` a behavioural difftest.

**That difftest has now been run. The corpus's claim about lcms2 is
wrong.**

### 12.1 What was done

`src/bin/legacy_lab_probe.rs` authors **four synthetic profiles byte by
byte** (category (a), `LEGAL.md` §3) — `scnr` class, RGB device space,
**Lab PCS**, whose only transform tag is an `A2B0` of type **`mft2`
(`lut16Type`)** holding a 2×2×2 CLUT with chosen corner values:

| file | header version |
|---|---|
| `probe_v2_1.icc` | `0x02100000` |
| `probe_v4_3.icc` | `0x04300000` |
| `probe_v4_4.icc` | `0x04400000` |
| `probe_v4_3_mluc.icc` | `0x04300000`, with v4 `mluc` metadata |

**The first three are byte-identical except for the version word**, and
the program asserts that at run time (`byte-diff … offsets [8, 9]`)
before believing any result. The fourth exists only to rule out the
objection that the other three carry v2-era `desc`/`text` metadata in a
v4 profile.

Probes land exactly on CLUT corners, so nothing is interpolated; `-c0`
stops lcms2 flattening the pipeline. The two candidate decodings are
separated by ≥0.196 in `L*` and ≈1.09 in `a*` at the probes used, against
a 16-bit quantisation floor of 0.0015 — the attribution bound is 0.01,
about 7× the noise and 20× below the smallest separation, and an
observation matching neither is reported as inconclusive rather than
rounded to the nearer.

### 12.2 The result

At **media-relative colorimetric**, every profile — v2.1, v4.3, v4.4 and
the fully-v4 `mluc` variant — decodes **LEGACY**:

| probe | CLUT (L,a,b) | legacy predicts | general predicts | **lcms2 gave** |
|---|---|---|---|---|
| P1 | `FF00 8000 8000` | 100.0000, 0.0, 0.0 | 99.6109, −0.4980, −0.4980 | **100.0000, 0.0, 0.0** |
| P2 | `0000 8000 8000` | 0.0, 0.0, 0.0 | 0.0, −0.4980, −0.4980 | **0.0, 0.0, 0.0** |
| P3 | `8000 8000 8000` | 50.1961, 0.0, 0.0 | 50.0008, −0.4980, −0.4980 | **50.1961, 0.0, 0.0** |
| P4 | `FF00 FF00 0000` | 100.0, 127.0, −128.0 | 99.6109, 126.0078, −128.0 | **100.0, 127.0, −128.0** |

Worst deviation from the legacy prediction across all probes and all four
profiles: **2×10⁻⁵** (`transicc`'s printing precision). The control — the
v2.1 profile, where both rules agree — reads legacy, so the instrument
can detect the effect it is looking for.

**Corroborated by reading the pinned source**, which is where the
mechanism is visible rather than inferred. `src/cmsio1.c`,
`_cmsReadInputLUT`:

```c
// After reading it, we have now info about the original type
OriginalType =  _cmsGetTagTrueType(hProfile, tag16);
…
// We need to adjust data only for Lab16 on output
if (OriginalType != cmsSigLut16Type || cmsGetPCS(hProfile) != cmsSigLabData)
    return Lut;
…
// Add a matrix for conversion V2 to V4 Lab PCS
if (!cmsPipelineInsertStage(Lut, cmsAT_END, _cmsStageAllocLabV2ToV4(ContextID)))
```

**No version test.** The same tag-type test appears in
`_cmsReadOutputLUT` (line ~627) and `_cmsReadDevicelinkLUT` (line ~782),
and the `namedColor2Type` paths insert the stage unconditionally. The
scale factor is `65535.0/65280.0` (`cmslut.c`
`_cmsStageAllocLabV2ToV4`) — the same `1.00390625` DL-005 names.
`cmsGetEncodedICCversion` appears in `cmsio1.c` only for the media white
point and `chad` fix-ups on **v2 display** profiles.

### 12.3 What this changes

- **DL-011's rule is unchanged** — it comes from the specification text
  and never depended on lcms2. It is now *also* what the field's dominant
  CMM does.
- **The predicted divergence does not exist on this pin.** DL-011 says
  *"iccce follows the specification text and must log the divergence at
  runtime rather than silently differing from the field's dominant CMM."*
  On lcms2 2.19.1 there is nothing to log for `mft2`-in-v4: the two
  agree. Pass 4 should still implement the tag-type selector, and the
  runtime warning should be reconsidered rather than written on the
  strength of a divergence that has now been measured as absent.
- **The corpus's lcms2 claim needs retracting.** It named
  `cmsLabEncoded2FloatV2` and `_cmsReadInputLUT` "inserting V2→V4 Lab
  stages based on `cmsGetEncodedICCversion`". At this pin,
  `cmsLabEncoded2FloatV2` is called from `cmspack.c` only — a *pixel
  formatter* for callers who ask for a v2-encoded Lab buffer — and never
  from profile reading. A dispatch to `icc-spec-librarian` is owed.
- **Scope, stated honestly.** One tag (`A2B0`), one tag type (`mft2`),
  one direction (device→PCS), one PCS (Lab), two intents, four synthetic
  profiles, one platform, one lcms2 build. **`ncl2` (`namedColor2Type`)
  was not tested behaviourally** — the source reading says it always gets
  the legacy stage, which is agreement, but that is source reading, not
  measurement. B2A (`_cmsReadOutputLUT`) was not tested behaviourally
  either.

### 12.4 ★ The second finding — lcms2 forces BPC on v4 perceptual and saturation

The first run used both intent 0 and intent 1. At intent 0 the **v4**
profiles matched *neither* hypothesis: black came back at `L* = −3.1482`
instead of 0, while the byte-identical v2 profile was unaffected. That is
**not** the Lab encoding. `src/cmscnvrt.c`, `_cmsLinkProfiles`:

```c
// Check if black point is really needed or allowed. Note that
// following Adobe's document:
// BPC does not apply to devicelink profiles, nor to abs colorimetric,
// and applies always on V4 perceptual and saturation.
if (TheIntents[i] == INTENT_PERCEPTUAL || TheIntents[i] == INTENT_SATURATION) {
    // Force BPC for V4 profiles in perceptual and saturation
    if (cmsGetEncodedICCversion(hProfiles[i]) >= 0x4000000)
        BPC[i] = TRUE;
}
```

with the black point taken from a fixed constant in that case
(`cmssamp.c`: *"v4 + perceptual & saturation intents does have its own
black point… Black point tag is deprecated in V4"*,
`cmsPERCEPTUAL_BLACK_X/Y/Z` = 0.003 36 / 0.003 473 1 / 0.002 87).

**So lcms2 silently enables black point compensation for v4 profiles at
perceptual and saturation, on the authority of an Adobe document rather
than ICC.1** — the user asked for neither.

This was **confirmed quantitatively, not assumed**. Transcribing lcms2's
own `ComputeBlackPointCompensation` (`a = (bpout − D50)/(bpin − D50)`,
`b = −D50·(bpout − bpin)/(bpin − D50)`, per channel) and running the
legacy-decoded `L*` through it predicts the observation:

| probe | `L*` before BPC | predicted after | observed | Δ |
|---|---|---|---|---|
| P1 | 100.0000 | 100.0000 | 100.0000 | 0 |
| P2 | 0.0000 | **−3.1482** | **−3.1482** | 3×10⁻⁵ |
| P3 | 50.1961 | 49.8574 | 49.8574 | 3×10⁻⁵ |
| P4 | 100.0000 | 100.0000 | 100.0000 | 0 |

An earlier attempt to confirm it by re-running the v2 profile with `-b`
**failed to decide**, and is kept in the program's output labelled as
such: `-b` is a no-op on that fixture because `cmsDetectBlackPoint`
reaches the fixed perceptual constant only through the same
`>= 0x4000000` guard, and with source and destination black points equal
lcms2 skips the stage. Two arms that differ in more than the variable
cannot settle anything — worth recording, because a reader repeating it
would otherwise read the null result as a refutation.

**Consequences, which are larger than the finding that prompted them:**

1. **Pass 4 will disagree with lcms2 at perceptual and saturation on
   every v4 profile**, unless iccce copies a behaviour ICC.1 does not
   require. That disagreement is ≈3.15 `L*` at black — nothing like
   sub-perceptual.
2. **Pass 5 (BPC) inherits it.** Any lcms2 cross-check at perceptual or
   saturation against a v4 profile is measuring BPC whether or not `-b`
   was passed. A tolerance set without knowing that would be a tolerance
   set on the wrong quantity.
3. It is a plausible origin for the corpus's belief that lcms2 keys Lab
   decoding on the profile version. lcms2 **does** key a decision on the
   profile version — at perceptual intent. Just not that one.

Neither finding is ground truth about colour: both are
`implementation-cross-check`-class observations of one build of one
implementation (`NUMERIC_CLAIMS.md` §1). What they establish is what
iccce will be compared against, which is exactly what an oracle is for.

---

## 13. ★ Pass 3 — the matrix/TRC differential: iccce against lcms2

**Run 2026-08-11 by `icc-conformance`.** This is the **first comparison in
this repository between `iccce` and any other implementation.** Everything
before it either compared lcms2 to lcms2 (§8, §11.3) or graded
`iccce-color` against published CIE data with no oracle in the loop
(`TOLERANCES.md` §3.1).

Apparatus: `src/pass3.rs` (grid, tolerances, records) and
`src/bin/pass3_report.rs` (per-point record, and the two experiments that
**test** the tolerances' justifications rather than asserting them).

### 13.1 The profile pair, and why no substitution was needed

ROADMAP Pass 3's done-when names **sRGB → AdobeRGB**. Both profiles are
present on this machine in the Windows colour directory, so the done-when is
answered as written and **no substitution-with-reason is invoked**:

| | source | destination |
|---|---|---|
| file | `C:\Windows\System32\spool\drivers\color\sRGB Color Space Profile.icm` | `C:\Windows\System32\spool\drivers\color\AdobeRGB1998.icc` |
| category (`LEGAL.md` §3) | **(c)** — read locally, never committed | **(c)** — read locally, never committed |
| version / class / spaces | 2.1 · `mntr` · `RGB ` → `XYZ ` | 2.1 · `mntr` · `RGB ` → `XYZ ` |
| `desc` | `sRGB IEC61966-2.1` | `Adobe RGB (1998)` |
| `cprt` | Copyright (c) 1998 Hewlett-Packard Company | Copyright 2000 Adobe Systems Incorporated |
| colorants (r/g/b XYZ) | (0.4361, 0.2225, 0.0139) (0.3851, 0.7169, 0.0971) (0.1431, 0.0606, 0.7141) | (0.6097, 0.3111, 0.0195) (0.2053, 0.6257, 0.0609) (0.1492, 0.0632, 0.7446) |
| **TRC** | **1024-entry sampled `curv` tables** (one shared tag-data block at offset 1084 for all three) | **single-value `curv` gamma, γ = 2.19921875** (`u8Fixed8` `0x0233`, exact in binary) |
| malformations reported | 0 | 0 |

**The TRC row is why this is a good pair rather than merely the named one.**
The source's tone curve is a *sampled table* and the destination's is an
*analytic gamma*, so one run exercises `iccce-cmm::curve`'s table
interpolation **and** its analytic evaluation, its Annex-F.1 table inversion
**and** its analytic inversion. Had both been gammas, half of that crate
would have gone untested while the report said "sRGB → Adobe RGB verified".

**Both are v2.1**, which matters for a reason recorded in §12.4 and taken up
in §13.3.

### 13.2 The instrument, and the design decision that had to be taken to build it

Three of `iccce`'s own crates are now **path dependencies of this harness**.
§11.2 previously said the harness *cannot* compute ΔE precisely because that
coupling "must be a documented decision, not a convenience". This is the
decision. Four things make it safe, and all four are load-bearing:

1. **The direction is the safe one.** §1's invariant is *no crate under
   `crates/` may reach lcms2*. These arrows point **difftest → iccce**, i.e.
   harness → code under test. `cargo tree` on any shipping crate still
   cannot reach lcms2; this crate is still outside the workspace, so
   `cargo test --workspace` still cannot pull it in.
2. **The ruler is validated against the literature.**
   `iccce_color::delta_e_2000` is graded against **all 34 published pairs of
   Sharma, Wu & Dalal (2005)** at 1×10⁻⁴ — the single **ground-truth** row in
   `TOLERANCES.md` §3.1.1 (NC-001). It is not a ruler checked against itself.
3. **The claim is unchanged.** Every iccce-vs-lcms2 record here is
   **cross-check**, not ground truth, however good the ΔE code is. A good
   ruler does not upgrade a weak claim.
4. **The answers still come from subprocesses.** iccce's colours come from
   running the shipped **`iccce transform` binary**; lcms2's from running
   `transicc`. The linked crates are the *instrument*, never the *subject*.
   Calling `MatrixTrcTransform::convert` in-process would be one line shorter
   and would make the two sides asymmetric — printing, parsing and argument
   handling exercised on one side only — so `Iccce`'s doc comment forbids it.

**The one exception is labelled as one.** Record 7 (§13.5) is an *instrument
check*: it holds iccce's device→Lab model, called in-process, against lcms2's
rendering of the same profile. That record says so in its own `source` field.

#### The units trap

| | input | output |
|---|---|---|
| `iccce transform` | one whitespace-separated triple per line, floats **0..1** | 6 decimals, **0..1** |
| `transicc` (8-bit RGB) | one component per line, **0..255** | 4 decimals, **0..255** |

Different in *both* directions. `pass3.rs` works in normalised 0..1
throughout and converts only at the `transicc` boundary. A number quoted
without its scale is wrong by 255, which looks like catastrophic colour error
rather than a units bug.

### 13.3 Settings, and the trap that was avoided by construction

| setting | value | why |
|---|---|---|
| intent | **media-relative colorimetric** (`-t1`) | the only intent iccce implements in Pass 3; `iccce transform` refuses any other **by name** rather than substituting |
| precalculation | **`-c0`** (`cmsFLAGS_NOOPTIMIZE`) | an oracle must be the reference implementation's most accurate path — the same reason `fast_float` is never built (§3). Any other `-c` makes a disagreement ambiguous between "iccce is wrong" and "lcms2 approximated" |
| BPC | not requested | but see below |
| grid | 133 deterministic points | §13.7 |

**Perceptual and saturation were not compared, deliberately.** §12.4 /
`ARCHITECTURE.md` **DL-013**: lcms2's `_cmsLinkProfiles` sets `BPC = TRUE` on
its own authority ("following Adobe's document") whenever the intent is
perceptual or saturation **and** `cmsGetEncodedICCversion >= 0x4000000`.
Measured effect ≈**3.15 `L*` at black** — nothing like sub-perceptual. A
tolerance set at those intents without knowing that is a tolerance set on the
wrong quantity.

**This pair would have escaped that trap anyway** — both profiles are v2.1
(`0x02100000`), below the version gate. That is worth stating and worth *not*
relying on: escaping a trap by accident is not avoiding it. The intent is
pinned at media-relative **by construction** (iccce implements nothing else),
and the v2-ness is recorded as a *second, independent* reason the comparison
is clean.

### 13.4 ★ Finding — lcms2 does not clamp its float device output on the high side, and iccce does

**8 of 399 output components (2.01%) came back from `transicc` outside
`[0,1]`** — up to `1.000 120` — all of them on grid points whose maximum
channel is 1.0. iccce returns exactly `1.000 000` for the same inputs.

**Which one does the specification support?** ICC.1:2022 **Annex F.8–F.16**
is normative for the matrix/TRC PCS→device direction and clamps each linear
component to `[0,1]` **before** the inverse TRC; that is what
`crates/iccce-cmm/src/matrix_trc.rs` implements and cites. On that reading
iccce is right and lcms2 is permissive.

**But the mechanism suggests lcms2 has no policy here at all.** Measured the
same day in the *reverse* direction (Adobe RGB → sRGB, whose destination TRC
inverse is a **tabulated** reverse curve rather than an analytic gamma),
lcms2 **does** saturate: `0 1 0` → `0.000000 1.000000 0.000000`, no
excursion. So the difference tracks *which inversion path lcms2 took* —
`pow(1.000106, 1/γ)` is perfectly finite and nothing forces it back, whereas
a reverse table has nothing to return outside its range. That is an artefact,
not a stated position.

**Status: FINDING, not failure** (`CLAUDE.md` rule 7). It is recorded here
and **not** settled from the specification text, because settling it properly
means putting the question to `icc-spec-librarian`, and **no Agent tool was
available in the session that ran this** — so the dispatch could not be made
and is **owed**, not done. The specific question to put:

> Does ICC.1:2022 require a matrix/TRC PCS→device conversion to deliver a
> device value within `[0,1]`, or only to clamp the *linear* value before
> TRC⁻¹? `TOLERANCES.md` NA-003 records that clause 6.4 requires clipping
> "on a per-component basis" on **integer** conversion and *no* clipping for
> float32 encodings — which may make lcms2's float excursion conforming and
> iccce's clamp merely stricter. The two clauses need reading together.

**How it is handled in the numbers meanwhile.** The device-space check
compares against lcms2's output **clamped into `[0,1]`**, so it grades
*arithmetic* disagreement; the *unclamped* maximum (1.200×10⁻⁴) and the count
of excursions are reported on the same record as a separate quantity. The ΔE
check is structurally blind to this, and that is correct rather than a gap: a
device code outside `[0,1]` denotes no colour in that device space, so there
is no colour difference to measure. Stated in the record so nobody reads the
ΔE silence as agreement.

### 13.5 The seven records, and what each can catch

`cargo run` emits these on stdout as TSV. Reproduced in §13.9.

| # | id | kind | metric | tolerance | observed |
|---|---|---|---|---|---|
| 1 | `pass3/srgb-to-adobergb/device-vs-lcms2` | cross-check | device abs-max, 0..1 | **5×10⁻⁴** | **6.7059×10⁻⁵** |
| 2 | `pass3/srgb-to-adobergb/device-mean` | cross-check | device abs-mean, 0..1 | **∞ — reported, not graded** | 6.1672×10⁻⁶ |
| 3 | `pass3/srgb-to-adobergb/de2000-vs-lcms2` | cross-check | ΔE2000 max | **2×10⁻²** | **3.4762×10⁻³** |
| 4 | `pass3/srgb-to-adobergb/de2000-mean` | cross-check | ΔE2000 mean | **∞ — reported, not graded** | 5.1145×10⁻⁴ |
| 5 | `pass3/srgb-to-adobergb-to-srgb/roundtrip-de2000` | self-consistency | ΔE2000 max | **2.5×10⁻²** | **1.8788×10⁻²** |
| 6 | `pass3/roundtrip/white-clamp-cost-matches-prediction` | self-consistency | ΔE2000 max | **1×10⁻³** | **5.7392×10⁻⁶** |
| 7 | `pass3/instrument/adobergb-device-to-lab-ruler` | cross-check | ΔE2000 max | **5×10⁻²** | **8.7945×10⁻⁵** |

Two of the seven are **means with an infinite tolerance**. They pass because
there is nothing for them to fail. They exist so the distribution is on file
next to the max — **a mean over a grid hides exactly the outlier a colour
engine gets wrong**, and quoting one for the other is the misuse the `metric`
column is there to prevent.

Record 7 is the **instrument check**: iccce's Adobe RGB device→Lab model
(in-process) against `transicc -i<dst> -o*Lab4 -t1 -c0` over the same 133
device values. It exists because records 3–5 measure things *with a ruler
built partly out of the code under test*; if that ruler were wrong, their ΔE
would be systematically mis-scaled and the error would hide inside the metric
instead of appearing as a number. At 8.79×10⁻⁵ ΔE2000 — below `transicc`'s
own Lab print floor of ~1×10⁻⁴ — the two rulers are indistinguishable, so the
ΔE figures above are not resting on a bent one.

### 13.6 ★ The tolerances, and the two experiments that test their justifications

Full derivations live on the constants in `src/pass3.rs`. What follows is
what each number rests on and, more importantly, **what was done to check
that the reasoning was true rather than plausible**.

#### 13.6.1 Device-space, 5×10⁻⁴ — and the quantisation experiment

The bound is derived from **lcms2's own arithmetic**, not from iccce's:

> `cmsgamma.c`, `cmsEvalToneCurveFloat`:
> ```c
> // Check for 16 bits table. If so, this is a limited-precision tone curve
> if (Curve->nSegments == 0) {
>     In  = (cmsUInt16Number) _cmsQuickSaturateWord(v * 65535.0);
>     Out = cmsEvalToneCurve16(Curve, In);
>     return (cmsFloat32Number) (Out / 65535.0);
> }
> ```

The source profile's TRCs are exactly that case — 1024-entry sampled tables,
no analytic segments — so lcms2 rounds both the curve's **input** and its
**output** to 1/65535 where iccce interpolates in `f64` throughout. Each
rounding is ≤ ½ lsb = 7.63×10⁻⁶; the input term is amplified by the sRGB
EOTF's peak slope (≈2.275 at white) to ≈1.74×10⁻⁵; total ≈**2.5×10⁻⁵ in
source-linear**. That is then amplified by the destination inverse gamma,
`(1/γ)·L^(1/γ−1)`, **which is unbounded as `L` → 0** — so there is no finite
uniform device-space tolerance valid over the whole cube, and pretending
otherwise would be the dishonest part. Evaluated at *this grid's* darkest
non-zero step (1/16 device → 4.03×10⁻³ linear → ×11.6) the envelope is
2.9×10⁻⁴; **5×10⁻⁴** is that rounded up.

**The tolerance is therefore grid-dependent by construction**, and its `why`
string says so: a grid extended nearer black must **re-derive** it, never
re-tune it.

**★ The experiment.** An assertion in a `why` string is exactly the kind of
claim this role exists to distrust, so `pass3_report` §4 **tests** it by the
method §12.4 established — *predict the confound quantitatively from the
other implementation's own arithmetic*. It emulates lcms2's evaluation inside
iccce's model (`linear = Q(TRC(Q(device)))`, `Q(v) = round(v·65535)/65535`)
and re-measures against lcms2's actual output:

| residual against lcms2's measured output | max | mean |
|---|---|---|
| device (0..1), **iccce as shipped** | 6.705882×10⁻⁵ | 6.167183×10⁻⁶ |
| device (0..1), **with lcms2's 16-bit quantisation modelled** | **2.311449×10⁻⁷** | 1.448340×10⁻⁷ |
| ΔE2000, iccce as shipped | 3.476186×10⁻³ | 5.114460×10⁻⁴ |
| ΔE2000, with quantisation modelled | 8.412613×10⁻⁵ | 1.772019×10⁻⁵ |

**The device-space residual shrinks by a factor of 290, to 2.31×10⁻⁷ —
*below* `transicc`'s own print floor of 1×10⁻⁴/255 = 3.92×10⁻⁷.** The
disagreement is accounted for, essentially completely, by a named
approximation in the **oracle**. The justification stands, and it stands
because it was measured.

Two limits, stated so a partial collapse could not have been over-read: lcms2
interpolates its table in 16-bit fixed point (`LinLerp1D` +
`_cmsQuickSaturateWord`) while the emulation interpolates in `f64` and rounds
once; and lcms2 carries the pipeline in `f32`. A residual of a few lsb was
the expected floor. It came in below it.

#### 13.6.2 ΔE2000 cross-check, 2×10⁻²

Carrying the device value back through the destination model **undoes** the
inverse-gamma amplification that made §13.6.1 grid-dependent, so this one has
a finite ceiling over the whole cube. The same 2.5×10⁻⁵ source-linear error
becomes ≤2.5×10⁻⁵ in PCS XYZ (`‖M_src‖∞` = 1.0, the Y row, by construction
for a D50-referenced media-relative profile), and Lab's steepest
sensitivities are on `f`'s linear segment where `f'(t) = 7.787`:
`dL*/dY ≤ 903.3` and `da*/dX ≤ 4038`. Worst case, every term aligned at its
maximum at the most sensitive point in the space: **ΔE00 ≲ 0.28**.

**2×10⁻² is set deliberately *tighter* than that ceiling**, because 0.28 is a
pessimistic union bound and a residual that had quietly grown from 3×10⁻³ to
0.27 would still pass a 0.28 gate with nothing to show it —
`TOLERANCES.md` §3.1's boxed warning, applied. 50× below the (⚠ provisional)
1.0 ΔE2000 perceptibility anchor, whose ⚠ this inherits and can afford.

#### 13.6.3 ★ Round trip, 2.5×10⁻² — a tolerance that FAILED, and why the number moved

**This tolerance was 1×10⁻² for the length of one run, and that run failed at
1.8788×10⁻².** The sequence is kept because `TOLERANCES.md` §0 makes the
*order* of the diagnosis the point.

The original justification read: *"sRGB and Adobe RGB (1998) share their red
(0.64, 0.33) and blue (0.15, 0.06) primaries and Adobe's green is more
saturated, so the sRGB triangle is strictly contained, no grid point is
clipped, and the only losses are interpolation ones."* Every clause of that
is true **of the two colour spaces** and the conclusion is false **of the two
files**.

**★ The experiment** (`pass3_report` §5). A matrix/TRC profile's media white
is its colorant sum `M·(1,1,1)`, and the two files' colorants were authored
and rounded to `s15Fixed16` independently — HP in 1998, Adobe in 2000.
Measured from the tags:

```
media white = colorant sum M*(1,1,1), as ENCODED in each file:
  source (sRGB)      X=0.96427917 Y=0.99996948 Z=0.82508850
  dest   (AdobeRGB)  X=0.96420288 Y=1.00000000 Z=0.82490540
  difference         dX=+7.629e-5  dY=-3.052e-5  dZ=+1.831e-4
```

Those differences are **5, 2 and 12 units of `s15Fixed16`'s 1/65536 lsb**,
accumulated over three colorant tags. Consequently the source's device white
lands *outside* the destination's encoded cube:

```
source white through M_dst^-1 : R=1.00010586 G=0.99987297 B=1.00025354
clamped to [0,1] per F.8-F.16 : R=1.00000000 G=0.99987297 B=1.00000000
channels actually clipped     : [0, 2]   <-- the original justification said 'none'
```

**25 of the 133 grid points are clipped somewhere**, all on the high-value
faces of the cube. Predicting the round-trip ΔE at white from **the two
matrices and the clamp alone** — no tone curve (every TRC here is exactly 1
at 1), no lcms2, no measurement:

| | ΔE2000 |
|---|---|
| **predicted**, closed form | 1.878244×10⁻² |
| **observed**, two invocations of the shipped binary | 1.878818×10⁻² |
| relative agreement | **0.03 %** |

Mechanism established. The number is then re-derived from it:

| term | value | source |
|---|---|---|
| range clamp of the encoded white-point mismatch | 1.8782×10⁻² | closed form from the two files' colorant tags |
| 1024-entry table interpolation, forward + inverse | ≈1×10⁻³ | `h²·max(f'')/8`, `h = 1/1023`, `max(f'') ≈ 3.0`, ×903.3 `dL*/dY`, two non-cancelling evaluations |
| **sum** | ≈1.98×10⁻² | |
| **tolerance** | **2.5×10⁻²** | the sum with ~25 % headroom, because the closed form is evaluated at the white corner only and the other 24 clipped points were not separately predicted |

**This is a corrected justification, not a widened number.** The distinction
is the whole of `TOLERANCES.md` §0's procedure: step 4 ("is the tolerance
wrong?") is reachable only after steps 1–3, and it was reached because step 3
found a property of the *corpus* the original derivation did not know about.
The change is logged in `TOLERANCES.md` §4 with both justifications, per the
append-only rule. **The number is corpus-specific and says so** — a different
profile pair re-derives it from its own colorant tags.

#### 13.6.4 ★ The check that stops §13.6.3 rewarding a deleted requirement — 1×10⁻³

Record 5 is an **upper** bound on a quantity that is mostly a *deliberate
cost*. Remove iccce's range clamping and the round trip gets **better**: the
upper bound would go green while a normative requirement had been deleted. A
gate that rewards that is not a gate.

Record 6 refuses the trade. It pins |predicted − observed| at device white,
where the prediction is closed-form `f64` arithmetic on the two colorant
matrices and the clamp, and the observation crosses two subprocess
boundaries. Tolerance 1×10⁻³ = ten times the ~1×10⁻⁴ ΔE00 floor imposed by
`iccce transform`'s 6-decimal device print on each leg (±5×10⁻⁷ per
component × `dL*/d device ≈ 85` at white ÷ `S_L ≈ 1.75`).

**The sensitivity control** (`pass3_report` §5, printed): with no clamping at
all the round trip is the exact identity, the observation would be 0, and
this record's metric would read 1.878×10⁻² — **failing by 19×**. An apparatus
not shown able to detect the effect it is looking for is not an experiment.

**★ Scope, and it is narrower than it first looks.** A first draft of this
check claimed it made the normative **F.8–F.16 *ordering*** falsifiable.
That claim was wrong, and it is corrected here rather than deleted. Reading
`iccce-cmm::curve`, range clamping happens at **three** independent sites,
each with its own citation:

| site | clause | what it clamps |
|---|---|---|
| `MatrixTrc::pcs_to_device` | **F.8–F.16** | linear → `[0,1]` before TRC⁻¹ |
| `Trc::eval` | **10.18** (domain) | curve input → `[0,1]` |
| `Trc::eval_inverse` / `invert_table` | **F.1(b)** | `y` → the attainable range |

So record 6 catches a **wrong colorant matrix** and **clamping removed from
all three sites**, and does **not** catch the F.8–F.16 clamp being removed on
its own, because the other two make it redundant. For this profile pair the
clamp-before/clamp-after *ordering* is **unobservable at the shipped
surface**, and no test in this repository currently distinguishes them.
`matrix_trc.rs`'s module doc is right that the order is normative and right
about the symptom if a CMM got it wrong; it is `iccce-cmm`'s own
belt-and-braces clamping that makes it undetectable here. Distinguishing the
two orders needs a TRC whose inverse is defined outside `[0,1]`, which iccce
never permits. **Recorded as owed, not as covered.**

#### 13.6.5 Instrument check, 5×10⁻²

Dominated by `transicc`'s print precision rather than by either
implementation: Lab printed to 4 decimals gives a ΔE00 floor of ≈1×10⁻⁴
before any arithmetic. Above that, lcms2's `cmsD50X/Y/Z` and iccce's `D50`
agree to 4 decimals **by construction** (`illuminant.rs` cites both as its
sources) but not beyond, and a 1×10⁻⁴ white-point difference moves `L*` by
~0.01. **5×10⁻² is ~5× that** — loose enough not to fail on known,
understood differences, tight enough that a swapped colorant, a missing D50
adaptation, or the v2/v4 Lab encoding error (≈0.39 `L*`) could not pass.

### 13.7 The grid — 133 points, and what it does not cover

| block | count | why it is there |
|---|---|---|
| cube corners | 8 | black, white, three primaries, three secondaries — where clamping, gamut edges and TRC endpoints all live |
| neutral axis | 17 (`k/16`) | neutrals are where a wrong white point or a channel-asymmetric TRC shows up as a visible cast, and where nothing else in the cube can hide it |
| 4×4×4 lattice on `{0, 1/3, 2/3, 1}` | 64 | systematic interior coverage that cannot accidentally miss a face or an edge |
| primaries/secondaries at half | 6 | mid-tone saturated colour, which a 1/3–2/3 lattice approximates but does not hit |
| pseudo-random interior | 48 | a fixed-seed LCG (MMIX constants), mapped into `[0.02, 0.98]`. Systematic grids can sit exactly on table entries and never interpolate; these deliberately do not |
| **total after de-duplication** | **133** | 143 before; the corners recur in the lattice |

**Deterministic by construction** — no `rand` crate, no clock, no hash seed.
Two runs on two machines compare the same 133 colours or the comparison
between their reports means nothing. Pinned by five unit tests
(`pass3::tests`), including one that asserts the count, because a silently
changed grid silently changes the scope of every number above.

**What it does not cover, stated because "verified" without scope is the
claim this whole role exists to prevent:**

- **Nothing below 1/16 except exact zero.** The destination inverse gamma
  amplifies without bound as linear → 0, which is precisely where §13.6.1's
  device-space tolerance is least transferable.
- **No genuinely out-of-gamut input**, because sRGB ⊂ Adobe RGB in
  chromaticity makes it impossible in this direction. The clip path is
  exercised only by the 1-lsb white-point excursions of §13.6.3, not by a
  real gamut clip. The reverse direction would; it is not in the suite.
- **One profile pair, one intent, one direction, one platform, one lcms2
  build.** Both profiles are v2; **no v4 profile is exercised at all.**
- **No LUT profile, no CMYK, no grey, no `chad`, no absolute colorimetric.**
  Pass 4.

### 13.8 Coverage statement — what "Pass 3 verified" is allowed to mean

> **iccce's Annex F.3 matrix/TRC model agrees with lcms2 2.19.1 to a maximum
> of 3.476×10⁻³ ΔE2000 (mean 5.114×10⁻⁴) and 6.706×10⁻⁵ in normalised device
> units (0.0171 in 0..255), over 133 deterministic points, sRGB →
> Adobe RGB (1998), media-relative colorimetric, `-c0`, on Windows 11 Pro
> 10.0.26200 / MSVC. The residual is accounted for to a factor of 290 by
> lcms2's own 16-bit quantisation of tabulated tone curves. Round-tripping
> sRGB → Adobe RGB → sRGB through iccce alone costs a maximum of
> 1.8788×10⁻² ΔE2000 (mean 8.674×10⁻⁴), of which 1.8782×10⁻² is the range
> clamp discarding a 5/2/12-lsb difference between the two files' encoded
> media whites and is predicted in closed form from the colorant tags.**

Everything outside that sentence is **not** verified. In particular it says
nothing about v4 profiles, LUT profiles, any other intent, the absolute
colorimetric white-point adjustment, BPC, or any platform but this one — and,
per §1, agreement with lcms2 is evidence that two implementations read a
clause the same way, which two implementations can do while both being wrong.

### 13.9 The machine-readable lines, as emitted 2026-08-11

`cargo run` output, `detail` column elided for width. The full lines carry
the whole `why` and `source` text on **every** record — including skips — so
a tolerance grepped out of a log always arrives with its justification
attached.

```
note	oracle: D:\Dev\iccce\tools\difftest\vendor/build-msvc/transicc.exe
note	banner: LittleCMS ColorSpace conversion calculator - 5.1 [LittleCMS 2.19]
note	pass3: iccce=.../target/release/iccce.exe (release) grid=133 points clipped=25
check	smoke/srgb-white-to-lab	PASS	oracle-reproducibility	abs-max-component	0.0001	0.000000e0	...
check	pass3/srgb-to-adobergb/device-vs-lcms2	PASS	cross-check	device-abs-max-normalised(0..1)	0.0005	6.705882e-5	...
check	pass3/srgb-to-adobergb/device-mean	PASS	cross-check	device-abs-mean-normalised(0..1)	inf	6.167183e-6	...
check	pass3/srgb-to-adobergb/de2000-vs-lcms2	PASS	cross-check	dE2000-max(kL=kC=kH=1,D50)	0.02	3.476186e-3	...
check	pass3/srgb-to-adobergb/de2000-mean	PASS	cross-check	dE2000-mean(kL=kC=kH=1,D50)	inf	5.114460e-4	...
check	pass3/srgb-to-adobergb-to-srgb/roundtrip-de2000	PASS	self-consistency	dE2000-max(kL=kC=kH=1,D50)	0.025	1.878818e-2	...
check	pass3/roundtrip/white-clamp-cost-matches-prediction	PASS	self-consistency	dE2000-max(kL=kC=kH=1,D50)	0.001	5.739153e-6	...
check	pass3/instrument/adobergb-device-to-lab-ruler	PASS	cross-check	dE2000-max(kL=kC=kH=1,D50)	0.05	8.794459e-5	...
summary	pass=8	fail=0	skip=0	error=0
```

Environment for all of the above: Windows 11 Pro 10.0.26200 x86-64; lcms2
2.19.1 at pin `21c582a`, MSVC Release, static; `iccce` built with
`cargo build --release -p iccce-cli` at commit **`051707f`**.

### 13.10 What §13 owes

1. ~~**A dispatch to `icc-spec-librarian`** on §13.4's clamping question —
   clause 6.4's integer-vs-float32 clipping rule read together with Annex
   F.8–F.16. Not made: no Agent tool was available in the session that ran
   this.~~
   **★ DISCHARGED 2026-08-11 (later the same day).** The librarian's fifth
   pass settled it in
   `ICC_Spec\icc\icc__s__computational_models.md` §4 (**A39**), and the
   answer went **against §13.4's own working hypothesis**:
   - **Clause 6.4 is not about device values at all.** It is titled
     "Converting between PCSXYZ and PCSLAB encodings" and every quantity in
     it — including the "No clipping is performed" sentence §13.4 quoted —
     is a **PCS** value. The clause that governs device encoding is **6.5**,
     and its float32 permission is **doubly gated**: float32 encoding *and*
     `DToBx`/`BToDx` tags, which **8.3.3/8.4.3 do not permit in a matrix/TRC
     profile at all**. The escape hatch is structurally unreachable here.
   - **A conforming F.8–F.16 evaluation cannot emit a device value above
     1,0**, by entailment: the clamp puts `TRC⁻¹`'s argument in `[0,1]`, and
     `TRC⁻¹` returns a value in the curve's *domain*, which 10.6/10.18 fix at
     `[0,0 1,0]`. lcms2's 1,000 120 is therefore **arithmetically
     unreachable from the model** — it is evidence that the *input* clamp
     was skipped, not that the *output* was left unclamped.
   - **Two hedges survive and must travel with any restatement.** ICC.1:2022
     clause 5 binds a CMM only to *reading* profiles (**A39b**), so
     "non-conforming CMM" is not a sentence the standard supports —
     `divergence` is the right word. And whether the **v2** specification
     states the same clamp is **UNSOURCED** (**A39c**): the profile is
     v2.1.0 and ICC.1:2001-04 has not been obtained.
   - **Still unmeasured:** the *size* of the divergence under genuine
     out-of-gamut input. Every excursion §13.4 observed is 1-lsb boundary
     residue at white, because sRGB ⊂ Adobe RGB makes real clipping
     impossible in that direction. A destination *smaller* than the source
     would drive the F.10 branch hard, and has not been run — see item 3.
     **§14 does not close this either**: on SWOP → sRGB, which *does* clip
     genuinely, `transicc` returned **0 of 1023 components** outside `[0,1]`
     at every intent, because the destination TRC inverse there is a
     *tabulated* reverse curve — lcms2's saturating path (M3).
2. **A fixture that distinguishes clamp-before from clamp-after** (§13.6.4).
   Needs a TRC whose inverse is defined outside `[0,1]`.
3. **The reverse direction in the suite**, Adobe RGB → sRGB — the one that
   exercises a real gamut clip.
4. **A v4 profile pair**, so that the version-gated behaviours §12.4 found
   are exercised rather than merely avoided.
5. **A synthetic pair from `tools/gen-profiles`** (Pass 2's generator, which
   does not exist), so that §13 does not skip entirely on a machine without
   the Windows colour directory.
6. **A `NUMERIC_CLAIMS.md` mirror** of §13's numbers, and of NA-004 in
   `TOLERANCES.md` §5 — `icc-librarian`'s file, not this one's.

---

## 14. ★ Pass 4 — the LUT differential: CMYK → RGB, iccce against lcms2

**Run 2026-08-11 by `icc-conformance`**, after §13, on the same machine and
the same pin. This is the **first comparison in this repository that exercises
a CLUT**, a four-channel device space, a `Lab ` PCS, and **all four rendering
intents**.

Apparatus: `src/pass4.rs` (grid, tolerances, the mft2 reimplementation, the
records) and `src/bin/pass4_report.rs` (per-point record, and the three
experiments that **test** the tolerances' justifications rather than asserting
them). Run `cargo run --bin pass4_report`.

**Pass 3's shape is deliberately reused** — same `Record` types, same
"envelope predicted first, residual measured second" discipline, same refusal
to grade a mean. What is new is that Pass 4's dominant disagreement is **a
named approximation rather than a rounding difference**, which changes what a
tolerance can honestly claim; §14.5 is about that and is the most important
subsection here.

### 14.1 The profile pair, read from its bytes

| | source | destination |
|---|---|---|
| file | `C:\Windows\System32\spool\drivers\color\USWebCoatedSWOP.icc` | `C:\Windows\System32\spool\drivers\color\sRGB Color Space Profile.icm` |
| category (`LEGAL.md` §3) | **(c)** — read locally, never committed | **(c)** — read locally, never committed |
| version / class / spaces | **2.1.0** · `prtr` · `CMYK` → **`Lab `** | **2.1.0** · `mntr` · `RGB ` → `XYZ ` |
| tags | 10: `desc cprt wtpt A2B0 A2B2 A2B1 B2A0 B2A1 B2A2 gamt` | 17, incl. `rTRC/gTRC/bTRC` (1024-entry `curv`) and the three colorants |
| the tag that matters | **`A2B0` and `A2B2` share one block of tag data** (both offset 432, size 41478); `A2B1` is separate at 41912. All three are `mft2` | **no `B2A*` at all** |
| A2B structure | 4 in / 3 out, **9 CLUT points per axis** (9⁴ = 6561 nodes), 256-entry input tables (non-identity), 2-entry output tables (identity), identity 3×3 | — |
| `wtpt` as stored | (0.708 405, 0.735 947, 0.571 045) | **(0.950 455, 1.000 000, 1.089 050) — i.e. D65, not D50** |

**Two of those rows decide what the comparison means.**

1. **The destination has no `B2A*` tags.** If it had one, lcms2 would evaluate
   the destination through a LUT while `iccce-cmm`'s `Chain` used the colorant
   matrix, and every ΔE below would be comparing **two different models**
   rather than two implementations of one. It does not, so both sides take
   clause 8.10.2 step 4 on that side of the chain. This was checked in the run
   (the structure line is printed on every record), not assumed.
2. **The destination's `wtpt` holds D65** while its colorants are D50-adapted
   — a common v2-era encoding. That is inert at three intents and is the whole
   story at the fourth; §14.6.

### 14.2 ★ What lcms2 actually does with a 4-D CLUT — read at the pin

The expected deviation was stated in advance as *"iccce interpolates n-linear,
lcms2 tetrahedral"*. **For four inputs that is not what lcms2 does**, and
since the tolerances rest on it, the source was read rather than recalled.

```c
// cmsintrp.c, DefaultInterpolatorsFactory:
case 3:  // RGB et al
    ... TetrahedralInterpFloat / TetrahedralInterp16 (unless CMS_LERP_FLAGS_TRILINEAR)
case 4:  // CMYK lut
    if (IsFloat) Interpolation.LerpFloat = Eval4InputsFloat;
    else         Interpolation.Lerp16    = Eval4Inputs;

// cmsintrp.c, Eval4InputsFloat — verbatim comment:
// "For more that 3 inputs (i.e., CMYK) evaluate two 3-dimensional
//  interpolations and then linearly interpolate between them."
```

So lcms2's four-input scheme is a **hybrid**: *linear* along input channel 0
(C), *Sakamoto tetrahedral* in the remaining three (M, Y, K), with the two
3-D results blended by the first channel's fraction. Consequences worth
stating because none of them is what "tetrahedral" would have implied:

- **It is not symmetric in the four inks.** Reordering the channels would
  change the answer. iccce's n-linear (quadrilinear) *is* symmetric.
- **It is not pure tetrahedral either**, so a bound transcribed from the
  trilinear-vs-tetrahedral literature is not the bound that applies.
- **The float path does not use the float interpolator.** An `mft2` tag is
  read into a **16-bit** CLUT stage (`cmsStageAllocCLut16bitGranular`), whose
  float evaluator is `EvaluateCLUTfloatIn16`: quantise the stage input to
  `u16`, call `Interpolation.Lerp16` — i.e. **`Eval4Inputs`, the fixed-point
  twin** — and convert back. lcms2's CMYK pipeline in `transicc`'s default
  float mode therefore carries 16-bit quantisation at the CLUT boundary **as
  well as** inside the tabulated tone curves (§13.6.1's finding).
- **The index conventions differ at the top of each axis.** lcms2 takes
  `k0 = floor(pk)` unclamped — `points − 1` when the input is exactly 1.0,
  with `rest = 0` — and separately collapses the upper node
  (`K1 = K0 + (Input >= 1.0 ? 0 : opta)`). iccce clamps the cell index to
  `points − 2` and lets the fraction reach 1.0. **Both are correct with their
  own upper-node rule and catastrophically wrong when mixed**; the first draft
  of the emulation mixed them and returned node 0 for an input of 1.0. It was
  caught by a unit test written for exactly that
  (`both_schemes_reproduce_a_separable_function_exactly`), which is why the
  test exists rather than a comment.

**This is `impl_crosscheck` knowledge, not specification.** ICC.1 says
**nothing** about CLUT interpolation — corpus ambiguity **A16**, SILENT — which
is why iccce's n-linear is a *named choice* (**NA-006**) and why a
disagreement here is a **difference**, not an error on either side.

### 14.3 Settings, the units trap, and the confound that was checked rather than assumed

| setting | value | why |
|---|---|---|
| intents | **all four** (`-t0..-t3`) | Pass 4's scope. Pass 3 could not: the CLI implemented one |
| precalculation | **`-c0`** (`cmsFLAGS_NOOPTIMIZE`) | an oracle must be the reference implementation's most accurate path (§3, §13.3) |
| BPC | not requested — **and unreachable** | see below |
| grid | 341 deterministic CMYK points | §14.4 |
| iccce side | the **shipped `iccce transform` binary**, `--intent <name>` | commit `490191b` gave it N-channel input and four intents; both sides cross a process boundary, as in §13 |

**The forced-BPC confound (DL-013 / §12.4 / corpus M2) is unreachable for this
pair, and the run proves it rather than assuming it.** lcms2 sets `BPC = TRUE`
on its own authority at perceptual and saturation only when
`cmsGetEncodedICCversion(profile) >= 0x4000000`. Both profiles here carry
header version `0x02100000`. `pass4::analyse` reads both version words from
the parsed headers and **prints them on every record**, so a future
substitution of a v4 profile cannot silently reintroduce the confound. (Pass 3
made the same point about escaping a trap by accident; here the version check
is a printed quantity rather than a paragraph.)

**★ The CMYK units trap, which is a different trap from Pass 3's.** `transicc`
reads CMYK as **0..100 percentages**, and *not* for the reason §9 implies. Its
own `InputRange` for `cmsSigCmykData` is **1** (`ComponentNames` in
`transicc.c`); the 0..100 convention comes from `cmspack.c`, where the double
formatters scale by `IsInkSpace(fmt) ? 100.0 : 1.0`. Measured as well as read,
2026-08-11:

```
0 100 100 0  ->  237.6654  51.3502  55.8132     (process red)
0   1   1 0  ->  254.9455 251.2879 249.4669     (1 % ink — near paper white)
```

A harness that fed 0..1 here would compare full-ink colours against 1 %-ink
colours and produce a difference of ~100 ΔE that looks like a catastrophic
colour bug. The RGB output side is 0..255 as in §13, and `transicc`'s output
is **unbounded by default** (`lUnbounded = TRUE`; `-u` is not needed, and it is
`-n`/quantise that would clamp).

### 14.4 The grid — 341 points, and what it does not cover

| block | count | why it is there |
|---|---|---|
| hypercube corners | 16 | paper, 100 % K, the four single inks, 0/100/100/0 process red, 400 % total ink. **Every one is an exact CLUT node** — the interpolation-free control block |
| K ramp `(0,0,0,k/8)` | 9 | the black channel alone: where a CMYK profile's separation behaviour lives |
| CMY composite neutral `(v,v,v,0)` | 9 | the *other* neutral axis. A transposed ink shows up here and nowhere else |
| rich neutral `(v,v,v,v)` | 9 | all four channels together, into the deepest shadow the profile reaches |
| 4-D lattice on `{0, ⅓, ⅔, 1}` | 256 | systematic interior coverage; ⅓ and ⅔ are **not** CLUT nodes, so every one of these interpolates |
| pseudo-random interior | 64 | fixed-seed LCG (MMIX constants) into `[0.02, 0.98]⁴` |
| **total after de-duplication** | **341** | |

Deterministic by construction — no `rand`, no clock, no hash seed — and pinned
by unit tests including the count and the corner block's position.

**What it does not cover:**

- **No total-ink-limit realism.** Real SWOP separations rarely exceed ~300 %
  total ink; this grid goes to 400 %. Deliberate (the CLUT is defined there and
  the disagreements are larger), but it means **the mean over this grid is not
  the mean over printable colour** and must not be quoted as if it were.
- **Nothing below 1/8 in a single channel** except through the random block.
- **One profile pair, one direction.** The **B2A** direction is not exercised
  at all: SWOP's `B2A*` tags are `mft1` (`lut8Type`), and although
  `iccce-cmm`'s `Chain` grew a B2A destination path in `b3f4388`, this run's
  destination is matrix/TRC. **"Pass 4 verified" therefore does not include
  B2A.**
- **No v4 profile, and no synthetic fixture.** `tools/gen-profiles` did not
  exist when this ran; a generator crate appeared in the working tree during
  the same session, so this sentence is dated rather than permanent — the
  substantive point is that **every §14 record reads a category (c) system
  profile and therefore skips off this machine**. One platform, one lcms2
  build.

### 14.5 ★ The tolerances, and the three experiments that test them

Full derivations live on the constants in `src/pass4.rs`. What follows is what
each number rests on and what was done to check the reasoning.

#### 14.5.1 The problem Pass 4 has that Pass 3 did not

In Pass 3 the whole disagreement was lcms2's 16-bit rounding — a *defect of
precision*, so a tight tolerance was both derivable and meaningful. Here the
dominant term is **an interpolation-method difference between two schemes
ICC.1 does not choose between**. It is not an error in either implementation,
it is ~1.6 ΔE2000, and it will not go away.

That creates a trap, and NA-006 named it in advance: *"a tolerance wide enough
to swallow ~1 ΔE cannot also demonstrate agreement."* A single number cannot
both admit the method difference and show that the two `lut16` pipelines agree.
**So Pass 4 uses two, and says which is which:**

- a **wide, structural** gate whose value *is* the method envelope
  (`de2000-vs-lcms2`, `pcs-lab-vs-lcms2`), which can catch a wrong index
  order or a wrong Lab decode and **cannot** claim agreement;
- two **tight, arithmetic** gates with the method difference switched off —
  `pcs-lab-emulated-geometry` (100× tighter) and
  `pcs-lab-corners-interpolation-free` (2000× tighter) — which are where the
  agreement claim actually lives.

#### 14.5.2 Experiment 1 — the interpolation-method envelope

`SourcePipeline` reimplements the entire `mft2` A2B path inside the harness —
input tables, CLUT, output tables, legacy Lab decode — **twice**, differing in
exactly one component: iccce's n-linear CLUT, or lcms2's `Eval4Inputs`
geometry transcribed from `cmsintrp.c`. **No lcms2 output enters this
quantity**; it is computed from the CLUT and the two algorithms alone.

The **apparatus is graded before anything is concluded from it**: the n-linear
arm is held against `iccce_cmm::lut_transform::Lut16Model` on every grid point
at every intent, tolerance 10⁻⁹ in `L*`/`a*`/`b*` units. **Observed: 0.0
exactly** — bit-identical, which is what one should expect from the same
arithmetic in the same order and is worth having as a number rather than a
hope.

| tag (intent) | envelope max | envelope mean | propagated end-to-end (ΔE00) | propagated (device 0..1) |
|---|---|---|---|---|
| `A2B1` (media-relative) | **0.254 23** | 0.038 54 | 0.254 23 | 2.9012×10⁻³ |
| `A2B0` (perceptual, saturation) | **1.574 1** | 0.043 86 | 1.663 9 | 1.0751×10⁻² |

**The two tables are not equally smooth and the difference is a factor of
six.** The perceptual table's worst cell is at CMYK (0.541, 0.442, 0.744,
0.972) — deep shadow at near-full black, where the CLUT turns sharply and the
two schemes take different routes across the same cell. **A tolerance derived
from `A2B1` alone would have been wrong by 6× for precisely the intents Pass 3
never exercised**, which is the argument for running all four intents rather
than assuming the colorimetric one is representative.

The tolerances follow from that table: **2.0 ΔE2000** for the PCS and
end-to-end ΔE gates (the larger envelope, +20–27 %), **2×10⁻²** device
(the larger propagated envelope, +86 %).

#### 14.5.3 Experiment 2 — the attribution

The Pass 3 discipline: predict the confound from the other implementation's
own arithmetic, then measure what is left.

| intent | n-linear vs lcms2 (max / mean) | **lcms2 geometry emulated** vs lcms2 (max / mean) | shrink |
|---|---|---|---|
| media-relative | 0.254 65 / 0.039 00 | **4.5931×10⁻³ / 1.2988×10⁻³** | **55× / 30×** |
| perceptual, saturation | 1.571 5 / 0.044 30 | **4.8154×10⁻³ / 1.1091×10⁻³** | **326× / 40×** |

One point in full, the worst method-envelope point at media-relative
(`pass4_report` §8):

```
CMYK              (0.94978, 0.69367, 0.95021, 0.94748)
iccce n-linear    Lab (14.2965, -3.2319, 1.6226)
lcms2 geometry    Lab (14.3933, -3.4322, 1.6197)   [emulated in f64, this harness]
transicc -o*Lab4  Lab (14.3934, -3.4297, 1.6211)   [the oracle itself]
```

The emulation lands on the oracle to 1×10⁻⁴ in `L*` while iccce's shipped
n-linear sits 0.2 away in `a*`. **The disagreement is the geometry**, and
what remains after substituting it is the oracle's own quantisation — the
budget `DE_PCS_EMULATED`'s 2×10⁻² is built from: tabulated input curves
rounded to 1/65535 in and out, the CLUT stage input rounded to `u16`,
`Eval4Inputs` evaluated in s15.16 fixed point, `transicc`'s 4-decimal Lab
print. One 16-bit lsb of CLUT output is **1.53×10⁻³ in `L*` and 3.9×10⁻³ in
`a*`/`b*`** under the legacy decode this tag type mandates (652.8 and 256
codes per unit) — the `a*`/`b*` scale means a single lsb is *not* negligible
there.

#### 14.5.4 Experiment 3 — the interpolation-free control

The 16 corners are the only grid points where both implementations evaluate
the CLUT **at an exact node**: each `mft2` input table starts at `0x0000` and
ends at `0xFFFF`, so device 0 and 1 map to node 0 and node 8. There, n-linear
and tetrahedral agree *identically* — the harness prints the method envelope
restricted to the corners and it is **0.0 exactly**, as it must be.

And lcms2's quantisation terms **vanish rather than accumulate** at a node:
the CLUT input is an exact `u16`, the interpolated value *is* the stored
`u16`, the output tables are the identity. What is left is `transicc`'s
4-decimal Lab print, a ΔE00 floor of ≈1×10⁻⁴.

| intent | corner max | corner mean |
|---|---|---|
| media-relative | **5.9131×10⁻⁵** | 2.8954×10⁻⁵ |
| perceptual, saturation | **6.6558×10⁻⁵** | 2.8820×10⁻⁵ |

Exactly the print floor, and **70× below** the same comparison between nodes.
**Tolerance 1×10⁻³ — ten times the floor.** This is the tightest gate in Pass
4 and the one that makes the 2.0 gate defensible: without a node-only control,
a wide structural gate could hide a genuine 1.9 ΔE error. It would catch the
v2/v4 Lab encoding error (≈0.39 `L*` at white, far worse in `a*`/`b*`), a
swapped ink, an off-by-one in the node index — all ≥1000× this bound.

#### 14.5.5 The shared-tag identity, graded at exactly zero

`A2B0` and `A2B2` are one block of tag data in this file (§8.4's Pass 0
finding, re-verified here from the parsed tag table). Perceptual and saturation
are therefore the *same bytes through the same code*, and any difference at all
is an 8.10.2 tag-selection defect — there is no arithmetic that could produce a
small one. **Tolerance `0.0`, observed `0.0` on both sides.** A small epsilon
here would admit exactly the class of bug the record exists to catch.

### 14.6 ★★ Finding — at the ICC-absolute intent, iccce and lcms2 read *different destination media whites*, and it costs 11 ΔE2000

**The observation.** At `-t3`, iccce and lcms2 differ by **max 11.217 ΔE2000,
mean 4.670** (device: max 0.1580, mean 0.0485) — two orders of magnitude more
than at any other intent, and far beyond anything the interpolation envelope
(0.2542 for `A2B1`, which is the table absolute uses) could account for. The
worst points are the *lightest* ones: paper (0,0,0,0) at 10.6 ΔE00, 33 % C at
11.2.

**The mechanism, read at the pin and then measured.**

```c
// cmsio1.c, _cmsReadMediaWhitePoint:
//     ... reads cmsSigMediaWhitePointTag ...
//     // V2 display profiles should give D50
//     if (cmsGetEncodedICCversion(hProfile) < 0x4000000) {
//         if (cmsGetDeviceClass(hProfile) == cmsSigDisplayClass) {
//             *Dest = *cmsD50_XYZ(); return TRUE;
//         }
//     }
```

At `AdaptationState == 1.0` (the default) `ComputeAbsoluteIntent` builds the
diagonal `WhitePointIn / WhitePointOut` — the same D.6/D.7 composite iccce
implements. **The two implementations differ not in the formula but in what
they read for `WhitePointOut`:**

| | source white | destination white |
|---|---|---|
| **iccce** (NA-007: `wtpt` **as stored**) | SWOP's `wtpt` | the sRGB profile's `wtpt` = **D65** (0.950 455, 1.0, 1.089 050) |
| **lcms2** | SWOP's `wtpt` (a `prtr`, so the tag is used) | **D50** (0.9642, 1.0, 0.8249) — substituted because the profile is **v2** and **display class** |

The ratio between them is `D65/D50` = (0.9858, 1.0, 1.3202): a **32 % error in
`Z`**, applied to every colour. That is the 11 ΔE.

**Re-predicting lcms2's output with that one substitution** (plus the CLUT
geometry, so the two known differences are both modelled) collapses it:

| | max ΔE00 | mean ΔE00 |
|---|---|---|
| iccce as shipped vs lcms2 | 11.217 | 4.6705 |
| re-predicted (D50 destination white + lcms2 geometry) | **2.1677×10⁻²** | **3.4034×10⁻³** |
| **shrink** | **517×** | **1372×** |

**Which one is right? — NOT settled here, and that is the finding.** ICC.1:2022
specifies v4 profiles; what a **v2** profile's `wtpt` means is corpus **A4b**,
**UNVERIFIED**, because ICC.1:2001-04 has not been obtained. lcms2's
substitution is justified in its source by a comment, not by a clause.
`NUMERIC_CLAIMS.md` **NA-007** registers iccce's as-stored reading as a named
choice. **A dispatch to `icc-spec-librarian` is owed**, and the question to put
is:

> Does the v2 specification (ICC.1:2001-04, or ICC.1:1998-09) define
> `mediaWhitePointTag` for a **display-class** profile as the *adapted* PCS
> white (i.e. D50 by construction, making lcms2's substitution a correction of
> a widely-mis-authored field) or as the *measured, unadapted* device white
> (making it a substitution of the CMM's own guess for the file's data)? And
> does ICC.1:2022 6.2.3 / D.6 / D.7's absolute-colorimetric composite intend
> `wtpt` as stored in either reading?

**How it is handled in the numbers meanwhile.** The two raw absolute-intent
comparisons are **REPORTED, NOT GRADED** (tolerance ∞), and the **gate at that
intent is the white-point-policy record** at 5×10⁻². Both alternatives were
considered and rejected in writing:

- *Widen the tolerance to ~15 ΔE00 so it passes.* A number chosen because it
  passed; 15 ΔE00 is a different colour; and it would silently absorb any
  future arithmetic error in the absolute path.
- *Let it fail permanently.* A red line that never changes stops being read,
  and it would report the disagreement as unexplained when it is not.

The moment A4b is settled, one of the two implementations acquires a defect and
this becomes a graded row again. **This is the only place in the suite where a
known disagreement is deliberately not gated**, and it is labelled rather than
absorbed.

### 14.7 The records, as emitted

`cargo run` adds 30 Pass 4 records to the report (2 whole-run, 7 per intent
with 3 of them skipping at absolute where a 4th takes their place).

| id | kind | tolerance | observed |
|---|---|---|---|
| `pass4/apparatus/harness-nlinear-matches-iccce-cmm` | self-consistency | 1×10⁻⁹ | **0.0** |
| `pass4/swop/perceptual-equals-saturation` | cross-check | **0.0 exact** | **0.0** |
| `…/perceptual/device-vs-lcms2` | cross-check | 2×10⁻² | **1.0816×10⁻²** |
| `…/perceptual/de2000-vs-lcms2` | cross-check | 2.0 | **1.6590** |
| `…/perceptual/pcs-lab-vs-lcms2` | cross-check | 2.0 | **1.5715** |
| **`…/perceptual/pcs-lab-emulated-geometry`** | cross-check | 2×10⁻² | **4.8154×10⁻³** |
| **`…/perceptual/pcs-lab-corners-interpolation-free`** | cross-check | 1×10⁻³ | **6.6558×10⁻⁵** |
| `…/media-relative/device-vs-lcms2` | cross-check | 2×10⁻² | **3.0045×10⁻³** |
| `…/media-relative/de2000-vs-lcms2` | cross-check | 2.0 | **0.252 94** |
| `…/media-relative/pcs-lab-vs-lcms2` | cross-check | 2.0 | **0.254 65** |
| **`…/media-relative/pcs-lab-emulated-geometry`** | cross-check | 2×10⁻² | **4.5931×10⁻³** |
| **`…/media-relative/pcs-lab-corners-interpolation-free`** | cross-check | 1×10⁻³ | **5.9131×10⁻⁵** |
| `…/saturation/*` | — | — | **identical to perceptual, exactly** (shared tag data) |
| `…/icc-absolute/device-vs-lcms2` | cross-check | **∞ — reported** | 0.157 96 |
| `…/icc-absolute/de2000-vs-lcms2` | cross-check | **∞ — reported** | **11.217** |
| **`…/icc-absolute/white-point-policy-emulated`** | cross-check | 5×10⁻² | **2.1677×10⁻²** |
| `…/<intent>/device-mean`, `…/de2000-mean` | cross-check | **∞ — reported** | see §14.8 |

Means, recorded next to their maxima and **never to be quoted for them**:
device 4.6257×10⁻⁴ (perceptual/saturation), 4.1870×10⁻⁴ (media-relative);
ΔE2000 4.3126×10⁻² and 4.0107×10⁻².

**`transicc` returned 0 of 1023 output components outside `[0,1]` at every
intent** — no M3-style excursions on this pair, because the destination TRC
inverse here is a *tabulated* reverse curve, which is lcms2's saturating path.

`summary  pass=36  fail=0  skip=3  error=0` for the whole suite (8 Pass 3
records, 1 smoke, 27 graded Pass 4 records + 3 absolute-intent PCS skips).

Environment: Windows 11 Pro 10.0.26200 x86-64; lcms2 2.19.1 at pin `21c582a`,
MSVC Release, static; `iccce` built with `cargo build --release -p iccce-cli`
at commit **`b3f4388`**.

### 14.8 Coverage statement — what "Pass 4 verified" is allowed to mean

> **iccce's `lut16` A2B pipeline chained into a matrix/TRC destination agrees
> with lcms2 2.19.1 to within the CLUT interpolation-method difference between
> the two — n-linear versus lcms2's linear-in-C × tetrahedral-in-MYK — over
> 341 deterministic CMYK points, `USWebCoatedSWOP.icc` → the Windows system
> sRGB profile, at all four ICC intents, `-c0`, on Windows 11 Pro 10.0.26200 /
> MSVC. That difference is up to 1.659 ΔE2000 (mean 4.31×10⁻²) at
> perceptual/saturation and 0.2529 (mean 4.01×10⁻²) at media-relative, and it
> is accounted for to 0.3–0.5 % by an envelope computed from the CLUT and the
> two algorithms alone. With lcms2's own interpolation geometry substituted,
> the two pipelines agree to 4.82×10⁻³ ΔE2000; at the 16 CLUT-node corners,
> where no interpolation happens, to 6.66×10⁻⁵ — `transicc`'s print floor. At
> the ICC-absolute intent the two implementations use different destination
> media whites (iccce `wtpt` as stored, lcms2 D50 by its v2-display rule),
> costing 11.217 ΔE2000; substituting that one policy collapses the
> disagreement 517×.**

Everything outside that sentence is **not** verified. In particular it says
nothing about: the **B2A direction** (`mft1`, not exercised), **`lut8Type`**,
**`lutAToBType`/`mAB `**, any **v4** profile, any **synthetic** fixture, BPC,
soft-proofing, any other platform, or any **published** value — every Pass 4
record is a cross-check or a self-consistency check, and **Pass 4 has no
ground-truth row at all**. Per §1, agreement with lcms2 is evidence that two
implementations read a clause the same way, which two implementations can do
while both being wrong.

**Pass 4's done-when is therefore only partly answered.** These are its first
numbers, at all four intents, in the A2B direction. All-intents coverage in
the sense the ROADMAP means — B2A evaluation, `iccce-cmm` stage 3 — is not
measured here even though `b3f4388` landed the code for it.

### 14.9 What §14 owes

1. **A dispatch to `icc-spec-librarian`** on §14.6's v2 `wtpt` question
   (corpus **A4b**, **NA-007**). Until it is settled, neither implementation's
   absolute-intent output can be called right.
2. **The B2A direction**, now that `b3f4388` exists: SWOP's `B2A*` are `mft1`,
   so this also needs `lut8Type` evaluation. That is where "all intents" is
   actually completed.
3. **A ground-truth row.** Pass 4 has none. The most tractable candidate is a
   **synthetic** `mft2` whose CLUT stores an exactly-reproducible function (an
   affine one, where **every** interpolation scheme must agree exactly, so the
   expectation is arithmetic rather than an oracle's opinion), authored by
   `tools/gen-profiles`. That crate did not exist when this ran and appeared in
   the working tree during the same session; when it is usable, this row and
   the CI-skip problem close together.
4. **An instrument check for the sRGB destination model.** §13's record 7
   bounds the ruler on **Adobe RGB**; Pass 4 inherits that bound rather than
   re-measuring it on the profile it actually used.
5. **A `NUMERIC_CLAIMS.md` mirror** of §14's numbers, and of NA-006's cost —
   which this pass makes **measured** for the first time. `icc-librarian`'s
   file, not this one's.
6. **A corpus entry** for the two lcms2 behaviours read here: the 4-D
   interpolation hybrid, and the v2-display `wtpt` substitution. They belong in
   `icc__ref__lcms2_measured_behaviour.md` as M4/M5; `icc-spec-librarian`'s
   file, not this one's.

---

## 15. ★ Pass 4b — the three directions Pass 4 left unmeasured

**Run 2026-08-11 by `icc-conformance`**, after §14, on the same machine and the
same pin. Apparatus: `src/pass4b.rs`; per-point record and the experiments:
`cargo run --bin pass4b_report`.

§14.9 and `TOLERANCES.md` §3.4.3 list what Pass 4 did not touch. This section
closes three of those items and leaves the rest labelled:

| § | direction | tag type | first of |
|---|---|---|---|
| **A** | sRGB → `USWebCoatedSWOP`, **RGB→CMYK** | `mft1` (`lut8Type`), 3→4, 33³, 8-bit | the **B2A** direction; the first `lut8` evaluation compared to anything |
| **B** | `fixtures/synthetic/v4-cmyk-mab-lab.icc`, **both** directions | `mAB `/`mBA `, ragged 5×4×3×2 and 3³ | the first **v4** LUT; the first **derived** (non-oracle) expectation for a LUT transform; the first graded rows in this suite that do **not** need a system profile |
| **C** | `ewgray22.icm` → sRGB, **GRAY→RGB** | none — Annex **F.2** grayTRC | the first monochrome transform |

**Result: 28 records, `pass=28 fail=0`.** Whole suite after Pass 4b:
`summary pass=64 fail=0 skip=3 error=0`.

### 15.1 The method, restated because Pass 4 had to learn it

Pass 4 stated its expected divergence as *"iccce interpolates n-linear, lcms2
tetrahedral"* and then read `cmsintrp.c`, which showed the four-input scheme is
a hybrid that is **not tetrahedral at all** (§14.2). A tolerance derived from
the wrong algorithm is a number with a story attached.

So every deviation source below was **read out of lcms2 at pin `21c582a` before
any comparison was run** and — where it is arithmetic rather than geometry —
**modelled in the harness**, so that each tolerance is an envelope computed from
lcms2's own arithmetic with **no lcms2 output in it**, and the residual after
modelling is a separate, much tighter record. Three of Pass 4b's five in-advance
predictions turned out to be the opposite of what the naive expectation would
have been, and two of those are findings in their own right.

### 15.2 §A — the B2A direction

#### 15.2.1 The pair and the tag, read from its bytes

| | source | destination |
|---|---|---|
| file | `sRGB Color Space Profile.icm` | `USWebCoatedSWOP.icc` |
| category (`LEGAL.md` §3) | **(c)** | **(c)** |
| version / class / spaces | 2.1.0 · `mntr` · `RGB ` → `XYZ ` | 2.1.0 · `prtr` · `CMYK` → **`Lab `** |
| the tag that matters | `rTRC/gTRC/bTRC`, 1024-entry `curv` **tables** | **`B2A0` @83392, `B2A1` @228980** — `mft1`, 3 in / 4 out, **33 points per axis** (35 937 nodes), 256-entry 8-bit input and output tables, identity 3×3 |

**Unlike `A2B0`/`A2B2`, the three `B2A*` tags are three different blocks at three
different offsets**, so perceptual and media-relative are genuinely different
tables here and each gets its own records. Both profiles are v2.1, so the
forced-BPC confound is unreachable — and §15.3.4 shows it would not have fired
in this direction even if one of them had been v4.

#### 15.2.2 ★★ Finding — lcms2 does **not** use tetrahedral interpolation in the B2A direction, and the reason is a hard-coded override

```c
// cmsio1.c, _cmsReadOutputLUT — verbatim, including the comment:
// Now it is time for a controversial stuff. I found that for 3D LUTS using
// Lab used as indexer space,  trilinear interpolation should be used
if (cmsGetPCS(hProfile) == cmsSigLabData)
    ChangeInterpolationToTrilinear(Lut);
```

`ChangeInterpolationToTrilinear` sets `CMS_LERP_FLAGS_TRILINEAR` on **every**
CLUT stage of the pipeline, which sends `DefaultInterpolatorsFactory`'s `case 3`
down `TrilinearInterpFloat`/`TrilinearInterp16` instead of the tetrahedral
routines it would otherwise select. **Trilinear over three inputs is n-linear**,
which is exactly what `iccce-cmm`'s `Clut::eval` computes (NA-006).

Consequences, all of which matter more than the size of the number:

1. **The interpolation-method envelope that dominated Pass 4 — 1,57 ΔE2000 — is
   identically zero in the B2A direction**, for every Lab-PCS profile, which is
   every CMYK output profile in this machine's colour directory.
2. It is in `_cmsReadOutputLUT` only. **The A2B direction is unaffected**, which
   is why Pass 4 measured a large envelope and Pass 4b does not; the two results
   are not in tension.
3. **It is a policy, not a specification.** ICC.1 says nothing about CLUT
   interpolation (corpus **A16**, SILENT). lcms2's own comment calls it
   "controversial stuff" and offers a rationale ("Lab used as indexer space")
   rather than a clause. iccce's n-linear happens to agree with it; that is
   agreement between two choices, not conformance.
4. **It means a cross-check in this direction cannot show that iccce's
   interpolation is right** — only that it is the same. The counterfactual
   record below is what stops that being invisible.

#### 15.2.3 The remaining deviation sources, and one non-source

- **8-bit table data is NOT a divergence.** `Type_LUT8_Read` widens every stored
  byte with `FROM_8_TO_16(v) = (v<<8)|v = v·257`, and `257·255 = 65535`, so
  lcms2's normalised sample is `v/255` — bit-identical to `iccce-cmm`'s
  `f64::from(v)/255.0`. The 1/255 granularity of the table is *shared* and
  cancels in the difference. What it does is make the pipeline **sensitive**:
  the largest adjacent-node step in this CLUT is **0,2235** of the device range,
  so an input difference is multiplied by up to `0,2235 × 32 = 7,2`.
- **lcms2 quantises three times inside this pipeline**, all modelled in
  `B2aPipeline::eval`: the 256-entry input curves are
  `cmsBuildTabulatedToneCurve16`, so `cmsEvalToneCurveFloat` rounds their input
  *and* output to 1/65535 (§13.6.1's finding); `EvaluateCLUTfloatIn16` rounds
  the CLUT stage input to `u16` and returns `u16/65535`; the output curves round
  twice more. Plus the source's 1024-entry `curv` TRCs, same mechanism.
- **★ The Lab encoding is NOT a divergence, and this is the one worth an
  assertion rather than a comment.** `_cmsReadOutputLUT` inserts
  `_cmsStageAllocLabV4ToV2` **only when `OriginalType == cmsSigLut16Type`** —
  for a `lut8Type` tag it does not, so the pipeline keeps lcms2's internal
  v4-normalised Lab (`L*/100`, `(ab+128)/255`, from `cmspack.c`'s
  `UnrollLabDoubleToFloat`). iccce's `PcsCodec::Lab8` encodes `L/100`,
  `(ab+128)/255` — Tables 12/13's 8-bit column, corpus **A10**. **The two agree
  exactly**: the legacy 652,8 scale belongs to `lut16Type`, and neither
  implementation applies it here. Had iccce applied it, `L*` would be 0,39 % low
  — ≈0,2 ΔE2000, *below* the perceptibility anchor and invisible to any suite
  graded at it. The agreement is now measured in the direction where the mistake
  is easiest to make.

#### 15.2.4 The grid — 213 RGB points end to end, 258 Lab points PCS-side

RGB: 8 cube corners, the 17-step neutral axis, three 9-step primary ramps, a
`{0, ¼, ½, ¾, 1}³` lattice, 64 fixed-seed pseudo-random interior points. Lab:
**125 node-aligned points** (`L*`, `a*`, `b*` each on `{0,8,16,24,32}/32` of the
encoded axis — exact nodes of a 33-point table), a 21-step neutral axis, 48
saturated hues at `C* = 60` well outside any CMYK gamut, and 64 pseudo-random.
Deterministic by construction and pinned by tests.

**What it does not cover:** nothing between 0 and 1/16 in RGB except through the
random block — which is where the source EOTF's inverse slope and the XYZ→Lab
sensitivity are both largest; **saturation and ICC-absolute** (the former is a
third copy of the same shape, the latter would re-measure §14.6's white-point
divergence rather than the B2A path); and no out-of-`[0,1]` device value, which
the shipped CLI does not accept.

#### 15.2.5 The three experiments

**1 — the envelope.** `B2aPipeline` evaluates the whole `mft1` path twice,
differing only in whether lcms2's roundings are applied, with the source model
switched the same way. The maximum difference over the grid **is** the envelope,
and no lcms2 output enters it: **1,330×10⁻⁴** device units at media-relative,
**9,602×10⁻⁵** at perceptual. `DEVICE_B2A` is `5×10⁻⁴` — the larger with ~276 %
headroom for the two roundings deliberately not modelled (lcms2 interpolates its
curves and its CLUT in **16-bit fixed point**; the model uses `f64`).

**Observed 1,330×10⁻⁴ against an envelope of 1,330 241×10⁻⁴ — 0,02 %.** The
disagreement is not merely small, it is *accounted for*.

**2 — the attribution.** The modelled prediction against `transicc`'s actual
output: **3,101×10⁻⁵ (perceptual), 3,100×10⁻⁵ (media-relative), 3,097×10⁻⁵
(PCS-side)**. That number is **2,03 lsb of 1/65535**, three times independently.
What remains after modelling lcms2's arithmetic is lcms2's *fixed-point*
arithmetic, and nothing else. `DEVICE_B2A_MODELLED` is `5×10⁻⁵`.

**3 — the counterfactual, which is the sensitivity control.** The same table
evaluated tetrahedrally: **1,527×10⁻²** (perceptual), **1,311×10⁻²**
(media-relative) — **139× and 99× the observed disagreement**. Without it, "the
two agree to 10⁻⁴" would be a claim about a comparison that might not be able to
see a geometry difference at all. It can, by two orders of magnitude.

#### 15.2.6 §A's numbers

| record | tolerance | perceptual | media-relative |
|---|---|---|---|
| `…/apparatus-lut8-matches-iccce-cmm` | 1×10⁻⁹ | **0,0 exactly** | **0,0 exactly** |
| `…/device-vs-lcms2` | 5×10⁻⁴ | **1,100×10⁻⁴** | **1,330×10⁻⁴** |
| `…/device-mean` | ∞ reported | 2,362×10⁻⁵ | 2,546×10⁻⁵ |
| `…/device-lcms2-arithmetic-modelled` | 5×10⁻⁵ | **3,101×10⁻⁵** | **3,100×10⁻⁵** |
| `…/roundtrip-lab-de2000` | 5×10⁻² | **7,095×10⁻³** | **5,711×10⁻³** |
| `…/counterfactual-tetrahedral` | ∞ reported | 1,527×10⁻² | 1,311×10⁻² |
| `pass4b/lab-to-swop/media-relative/pcs-device-vs-lcms2` | 5×10⁻⁴ | — | **6,485×10⁻⁵** |
| `…/pcs-device-lcms2-arithmetic-modelled` | 5×10⁻⁵ | — | **3,097×10⁻⁵** |

The ΔE row carries both sides' CMYK back through **the same file's own `A2B1`**
— not a second opinion, the same table's forward direction — because four ink
components have no perceptual metric until they are in a space where a ΔE means
something. The PCS-side rows take the source model out of the picture entirely
and grade `iccce-cmm`'s `Lut16Model` **in process**, which their records say:
the shipped CLI has no Lab entry point, so those two rows grade the *model*, not
the binary.

### 15.3 §B — the synthetic v4 fixture

#### 15.3.1 The 40-profile sweep, and why a fixture was the only option

Every `.icc`/`.icm` in `C:\Windows\System32\spool\drivers\color\` was parsed
with `iccce inspect` and searched for `mAB `/`mBA ` tags. **40 profiles, zero
matches.** The one v4 profile that carries a LUT at all (`BlackWhite.icc`,
4.0.0, `prtr`, `GRAY`) carries a `B2A0` of type **`mft1`**. So on this machine
the entire v4 element-pipeline path — the ragged CLUT, the 3×4 matrix with
offsets, the five-element chain — **cannot be exercised against a real profile
at all**, and `tools/gen-profiles`' `v4-cmyk-mab-lab.icc` is not a convenience
but the only available instrument. It is also why §B's four derived rows are the
**first graded rows in this suite that do not skip on a machine without the
Windows colour directory**.

#### 15.3.2 ★ The closed forms, and why they are a new kind of row

Reading the generator's recipe shows both CLUTs store functions that are
**affine in one input and constant in the others**:

- `A2B0` (`mAB `, 4→3, grid 5×4×3×2): `L*` node = `100·(1 − K)`, `a*`/`b*` node =
  `0x8080`, independent of C, M, Y.
- `B2A0` (`mBA `, 3→4, grid 3³): `K` = `1 − L*` along the `L*` axis, `C=M=Y=0`,
  independent of `a*`, `b*`.

**Every interpolation geometry reproduces an affine function exactly**, so the
method difference that dominates Pass 4 is *provably* zero here — and it is
measured as such (`clut-is-affine-both-geometries-agree`) rather than asserted.
The output is then a closed form in the input, derived from ICC.1:2022
10.12.1/10.13.1 (element order), 10.12.5/10.13.4 (the 3×4 matrix and its offsets
`1/256`, `2/256`, `3/256`, applied in the *normalised* domain) and 6.3.4.2
Tables 12/13 (the **general** 16-bit PCSLAB encoding — `mAB `/`mBA ` are not in
NOTE 3's legacy set):

```
mAB:  L* = 100·(1 − K) + 0,390625      a* = 1,9921875      b* = 2,98828125
mBA:  C = M = Y = 0,   K = interp(1 → 32768/65535 → 0)  at  n_L = L*/100 + 1/256
```

Two details a reader should check rather than trust: the offsets are applied in
the normalised domain, so `+1/256` is `+0,390625` of `L*` and `+1,9921875` of
`a*`; and the mBA's **middle node is `round(0,5·65535) = 32768`, i.e.
`0,500 007 63`, not `0,5`** — an expectation using the idealised line would be
wrong by 7,6×10⁻⁶ and would look like an implementation defect.

**This is a new `Kind`: `derived-expectation`.** It is **not** ground truth —
nobody at the CIE or the ICC printed the number. It is stronger than a
cross-check, because a cross-check is defeated when both implementations share a
misreading whereas this is defeated only when *the derivation* shares it, and
the derivation sits next to the number in a form a spec reader can check without
running anything. And it has a stated weakness: the fixture and the derivation
are read out of the **same corpus** by the same project, so if `ICC_Spec`'s
transcription of 10.12/10.13 is wrong they are wrong together and agree
perfectly. That is precisely why every derived row is paired with an lcms2
cross-check over the same points — **the third reading**.

| record | kind | tolerance | observed |
|---|---|---|---|
| `pass4b/fixture/clut-is-affine-both-geometries-agree` | self-consistency | 1×10⁻¹⁴ | **1,110×10⁻¹⁶** |
| `…/mab/iccce-vs-derived-expectation` | **derived-expectation** | 1×10⁻¹² | **2,842×10⁻¹⁴** (`L*`) |
| `…/mab/lcms2-vs-derived-expectation` | **derived-expectation** | 1×10⁻² | **2,325×10⁻³** (`L*`) |
| `…/mba/iccce-vs-derived-expectation` | **derived-expectation** | 1×10⁻¹² | **2,220×10⁻¹⁶** (device) |
| `…/mba/lcms2-vs-derived-expectation` | **derived-expectation** | 1×10⁻⁴ | **1,873×10⁻⁵** (device) |
| `pass4b/srgb-to-fixture/media-relative/device-vs-lcms2` | cross-check | 1×10⁻⁴ | **5,200×10⁻⁵** |
| `pass4b/fixture-to-srgb/media-relative/device-vs-lcms2` | cross-check | 2,5×10⁻⁴ | **1,012×10⁻⁴** |

**iccce reproduces the closed form to `f64` noise in both directions**, which is
the strongest statement any LUT row in this repository has been able to make —
and it is a statement about **GP-001**'s fix as well, since the `mBA ` curve
counts (B=3, M=3, A=4 for a 3-in/4-out tag) are what make the chain evaluate at
all. lcms2 reproduces it to its own quantisation.

#### 15.3.3 ★★ Finding — the encoded PCS overflows; iccce clamps it and lcms2 does not, and it costs 0,61 ΔE2000

At `K = 0` the `mAB ` CLUT's `L*` node is full scale (`0xFFFF`, normalised 1,0)
and the matrix then adds `+1/256`, so the value handed to the `B` curves is
**1,003 906 25 — outside the range of the 16-bit PCS encoding it is about to be
read as**.

| | what it does | result |
|---|---|---|
| **iccce** | `Trc::eval` enforces clause **10.18**'s `[0,1]` curve domain, so the `B` curve clamps its input | `L* = 100` |
| **lcms2** | a `curv` with `count = 0` becomes a type-1 parametric curve (γ = 1) whose segment domain is ±10²², evaluated as `pow(x, 1)` — nothing forces it back | `L* = 100,390 625` |

Measured directly, `transicc -i<fixture> -o*Lab4` at `K = 0`:
**`100.3906  1.9922  2.9883`** — the unclamped closed form to four decimals.

**Cost: 0,6117 ΔE2000** over the 10 affected grid points, carried end to end
into sRGB (device 4,440×10⁻³). That is well above everything else in Pass 4b and
in the neighbourhood of §2's provisional 1,0 anchor — a difference that *would*
be visible next to its neighbour.

**Which is right is NOT settled here, and that is the finding.** The two
readings:

> *iccce's:* the `B` curves are `curveType` elements whose domain is `[0,1]`
> (10.18), and the result of an `mAB ` is a **PCS value in a defined encoding**
> whose range is 0…1 by construction (6.3.4.2, Tables 12/13). 1,003 906 25 has
> no 16-bit code. Clipping the PCS is what clause **6.4** requires — and note
> NA-003's correction, that 6.4 *is* the PCS clause (`TOLERANCES.md` §5.2).
>
> *lcms2's:* the float pipeline is unbounded by design, the elements are
> function compositions, and clipping is the business of the *encoder* at the
> end of the chain, not of every intermediate element.

**A dispatch to `icc-spec-librarian` is OWED**, and the question to put is:

> In `lutAToBType`/`lutBToAType` (ICC.1:2022 10.12/10.13), is the output of the
> **matrix** element required to be clipped to the domain of the curve element
> that follows it — and is the output of the final `B` curves required to be
> clipped to the encodable PCS range of 6.3.4.2 — when a CMM evaluates the chain
> in floating point rather than through the 16-bit encoding? Specifically: does
> 10.18's statement of a `curveType`'s domain bind the *evaluator*, or only
> describe the stored samples? And does clause 6.4's PCS clipping requirement
> apply to the result of an `mAB ` before it is a PCS value, or only once it is
> encoded?

**Until it is settled the affected points are REPORTED, NOT GRADED** — the
posture §14.6 takes with A4b, and for the same reasons: grading them would mean
either a ~0,7 ΔE tolerance chosen because it passed, or a permanent red line
that stops being read. The remaining 118 grid points are graded normally and the
record names the excluded set and why.

**One thing this finding is not:** a defect the fixture was designed to catch.
`v4-cmyk-mab-lab.icc` carries non-zero matrix offsets because *dropping* them is
the classic misread. That the same offsets also push a value past full scale is
an accident of the fixture — and the best argument in this document for
authoring fixtures with awkward values rather than tidy ones.

#### 15.3.4 ★ Finding — lcms2's forced BPC is decided by the **destination** profile's version, which refines M2

`ARCHITECTURE.md` DL-013 and corpus **M2** record that *"lcms2 forces BPC on v4
profiles at the perceptual and saturation intents"*. Measured here in **both
directions on the same pair of profiles**, that is half the rule:

| direction | perceptual vs media-relative, lcms2 against itself |
|---|---|
| v4 fixture as **source**, v2 sRGB destination | **0,0 — bit-identical** |
| v2 sRGB source, v4 fixture as **destination** | **3,137×10⁻²** device (`K` at black moves 99,6094 % → 96,4721 %) |

The mechanism, read at the pin: `_cmsLinkProfiles` sets `BPC[i]` per profile,
but `DefaultICCintents` consumes it as
`ComputeConversion(i, hProfiles, Intent, BPC[i], …)`, which builds the
conversion **from `hProfiles[i-1]` into `hProfiles[i]`**. The flag that decides
is therefore the **destination** profile's version; a v4 *source* into a v2
destination sets a flag nothing reads.

Both sides of that measurement are lcms2, so it says nothing about iccce. What
it says is that **anyone using M2 to decide whether a comparison is confounded
needs the direction, not just the version** — and that §B's choice of the
media-relative intent was necessary for the `mBA ` direction and
belt-and-braces for the `mAB ` one. It belongs in
`icc__ref__lcms2_measured_behaviour.md` as a correction to M2;
`icc-spec-librarian`'s file, not this one's.

### 15.4 §C — the gray axis

#### 15.4.1 Both implementations build the same model, from the same constants

```c
// cmsio1.c: the gray input pipeline is the TRC then a 1x3 matrix of D50.
static const cmsFloat64Number GrayInputMatrix[] =
    { (InpAdj*cmsD50X), (InpAdj*cmsD50Y), (InpAdj*cmsD50Z) };
```

`cmsD50X/Y/Z` are `0.9642 / 1.0 / 0.8249` (`lcms2.h`) and `iccce_color::D50` is
`0.9642 / 1.0000 / 0.8249` — **the same three literals** — so Annex F.2's white
multiplication cannot diverge. (`InpAdj` is lcms2's internal 1/1,99997 XYZ
encoding scale, undone by the matching `OutpAdj` on the destination side.) The
`kTRC` is a single-value `curv` (γ = 2,199 218 75), which lcms2 turns into a
type-1 parametric curve, so it is **analytic on both sides** and §13.6.1's
tabulated-curve quantisation does not apply to the source either.

Two consequences stated in advance, both confirmed:

- **`wtpt` is not read by either side at the media-relative intent**, so §14.6's
  divergence — lcms2 substituting D50 for a v2 *display* profile's `wtpt` —
  **cannot fire here even though `ewgray22.icm` is exactly such a profile**
  (v2.2, `mntr`, `wtpt` = D65 = 0,950 455 / 1,0 / 1,089 050). It would fire at
  ICC-absolute; that intent is out of §C's scope and stays attributed to §14.6.
- **Perceptual and media-relative must be the same transform**, because a
  monochrome profile has no `A2B*`/`B2A*` for 8.10.2 to select between and the
  destination is matrix/TRC. Graded at **exactly `0,0`**; observed 0,0 on both
  sides.

#### 15.4.2 ★ Finding — the whole residual is lcms2's 4096-entry reverse tone curve, attributed 457×

Because the source contributes nothing, §C is the cleanest measurement available
of lcms2's sRGB **output** model — which answers §14.9 item 4.
`BuildRGBOutputMatrixShaper` inverts each 1024-entry `curv` with
`cmsReverseToneCurve` = `cmsReverseToneCurveEx(4096, ·)`: a **4096-entry `u16`
resampling** of the inverse, built by chording between forward-table knots, then
evaluated through the float path that rounds input and output to 1/65535.
iccce inverts the stored table directly.

`ReverseCurve` reimplements it. The result:

| | max | mean |
|---|---|---|
| iccce as shipped vs lcms2, device | **9,686×10⁻⁵** | 1,782×10⁻⁵ |
| the same in ΔE2000 | **2,169×10⁻²** | 2,641×10⁻³ |
| envelope (exact vs modelled destination — **no lcms2 output**) | 9,680×10⁻⁵ | 1,780×10⁻⁵ |
| **modelled prediction vs lcms2's actual output** | **2,121×10⁻⁷** | — |
| **shrink** | **457×** | |

2,121×10⁻⁷ is **below `transicc`'s 4-decimal print floor** of 3,9×10⁻⁷ in
normalised units. The disagreement is not merely explained, it is *reproduced*.
Worst point `g = 2/255`: iccce `0,000300`, lcms2 `0,000397`, model `0,000397`.

Note the envelope is 0,06 % *below* the observation (9,680 vs 9,686×10⁻⁵), and
that is the expected direction: the envelope is computed between two `f64`
pipelines while the observation additionally carries `transicc`'s 4-decimal
print and `iccce transform`'s 6-decimal print. An envelope comfortably *above*
the observation would have meant the model was pessimistic about lcms2.

#### 15.4.3 ★ A sensitivity note that inverts one from Pass 3

§13.6 recorded *"near black the device metric explodes while ΔE stays small"* —
that is the *inverse* TRC's unbounded slope acting on a device comparison. Here
the comparison is already in device units and the amplification runs the other
way. Below sRGB's linear breakpoint a device difference `δ` becomes `δ/12,92` of
linear light, and CIELAB's **chromatic** sensitivity on *its* linear segment is
`da*/dX = 500 · 7,787 / X_n = 4038`, so with `δ = 9,68×10⁻⁵`:

- `ΔL* ≈ 903,3 × δ/12,92 = 69,9 δ` = 6,8×10⁻³
- `Δa* ≈ 4038 × (δ/12,92) × X_R = 136 δ` = 1,3×10⁻² (`X_R = 0,4361`)

and near neutral `S_C ≈ 1` while `S_L ≈ 1,75`, so **the chromatic term is the
larger by ~3× and the maximum ΔE is at the dark end, not the light one**. Union
≈2×10⁻²; observed 2,169×10⁻². `DE_GRAY` is `5×10⁻²`.

**The first draft of `DE_GRAY` said 1×10⁻² from a derivation taken at white, and
the row failed at 2,17×10⁻².** The code was not wrong (the mechanism is
attributed 457×) and the fixture was not wrong — the *derivation was looking at
the wrong end of the axis*. `TOLERANCES.md` §4 logs it as a corrected
justification with both texts kept.

### 15.5 The records, as emitted

```
pass4b/srgb-to-swop/perceptual/apparatus-lut8-matches-iccce-cmm        PASS  1e-9    0.0
pass4b/srgb-to-swop/perceptual/device-vs-lcms2                         PASS  5e-4    1.100000e-4
pass4b/srgb-to-swop/perceptual/device-mean                             PASS  inf     2.361502e-5
pass4b/srgb-to-swop/perceptual/device-lcms2-arithmetic-modelled        PASS  5e-5    3.101114e-5
pass4b/srgb-to-swop/perceptual/roundtrip-lab-de2000                    PASS  5e-2    7.095173e-3
pass4b/srgb-to-swop/perceptual/counterfactual-tetrahedral              PASS  inf     1.526949e-2
pass4b/srgb-to-swop/media-relative/apparatus-lut8-matches-iccce-cmm    PASS  1e-9    0.0
pass4b/srgb-to-swop/media-relative/device-vs-lcms2                     PASS  5e-4    1.330000e-4
pass4b/srgb-to-swop/media-relative/device-mean                         PASS  inf     2.546479e-5
pass4b/srgb-to-swop/media-relative/device-lcms2-arithmetic-modelled    PASS  5e-5    3.100458e-5
pass4b/srgb-to-swop/media-relative/roundtrip-lab-de2000                PASS  5e-2    5.710814e-3
pass4b/srgb-to-swop/media-relative/counterfactual-tetrahedral          PASS  inf     1.311299e-2
pass4b/lab-to-swop/media-relative/pcs-device-vs-lcms2                  PASS  5e-4    6.485006e-5
pass4b/lab-to-swop/media-relative/pcs-device-lcms2-arithmetic-modelled PASS  5e-5    3.097192e-5
pass4b/fixture/clut-is-affine-both-geometries-agree                    PASS  1e-14   1.110223e-16
pass4b/fixture/mab/iccce-vs-derived-expectation                        PASS  1e-12   2.842171e-14
pass4b/fixture/mab/lcms2-vs-derived-expectation                        PASS  1e-2    2.325000e-3
pass4b/fixture/mba/iccce-vs-derived-expectation                        PASS  1e-12   2.220446e-16
pass4b/fixture/mba/lcms2-vs-derived-expectation                        PASS  1e-4    1.873190e-5
pass4b/srgb-to-fixture/media-relative/device-vs-lcms2                  PASS  1e-4    5.200000e-5
pass4b/fixture-to-srgb/media-relative/device-vs-lcms2                  PASS  2.5e-4  1.012157e-4
pass4b/fixture/mab/encoded-pcs-overflow-divergence                     PASS  inf     6.117005e-1
pass4b/fixture/forced-bpc-is-decided-by-the-DESTINATION-version        PASS  inf     3.137300e-2
pass4b/gray-to-srgb/media-relative/device-vs-lcms2                     PASS  2.5e-4  9.686275e-5
pass4b/gray-to-srgb/media-relative/device-mean                         PASS  inf     1.782154e-5
pass4b/gray-to-srgb/media-relative/de2000-vs-lcms2                     PASS  5e-2    2.169482e-2
pass4b/gray-to-srgb/media-relative/device-lcms2-arithmetic-modelled    PASS  5e-6    2.121004e-7
pass4b/gray-to-srgb/perceptual-equals-media-relative                   PASS  0.0     0.0
```

`summary pass=64 fail=0 skip=3 error=0` for the whole suite.

Environment: Windows 11 Pro 10.0.26200 x86-64; lcms2 2.19.1 at pin `21c582a`,
MSVC Release, static; `iccce` built with `cargo build --release -p iccce-cli` at
commit **`97ad9fa`**.

### 15.6 Coverage statement — what "Pass 4b verified" is allowed to mean

> **(A)** iccce's `lut8` **B2A** pipeline, driven from a matrix/TRC source,
> agrees with lcms2 2.19.1 to **1,33×10⁻⁴ device units** (5,7×10⁻³ ΔE2000 when
> carried back through the same profile's `A2B1`) over 213 deterministic RGB
> points, `sRGB Color Space Profile.icm` → `USWebCoatedSWOP.icc`, at the
> perceptual and media-relative intents, `-c0`; and to **6,49×10⁻⁵** over 258
> Lab points with the source model removed. That disagreement is accounted for
> to **0,02 %** by an envelope computed from lcms2's own roundings, and what
> remains after modelling them is **2,03 lsb of 1/65535**. The
> interpolation-method difference is zero because `_cmsReadOutputLUT` forces
> trilinear for a Lab-PCS LUT; had it not, the disagreement would have been
> **99–139× larger**, which is measured and not assumed.
>
> **(B)** iccce's `mAB ` and `mBA ` evaluation reproduces a **closed form
> derived from ICC.1:2022 10.12/10.13 and Tables 12/13** to `f64` noise
> (2,8×10⁻¹⁴ `L*`, 2,2×10⁻¹⁶ device) over 128 CMYK and 258 Lab points on **one
> synthetic v4 fixture**, and agrees with lcms2 to 1,01×10⁻⁴ device end to end —
> **except at the 10 points where the 3×4 matrix pushes the encoded PCS above
> full scale**, where the two differ by **0,61 ΔE2000** and the specification
> question is unsettled.
>
> **(C)** iccce's Annex **F.2** grayTRC model agrees with lcms2 to
> **9,69×10⁻⁵ device / 2,17×10⁻² ΔE2000** over 69 points of the gray axis,
> `ewgray22.icm` → the system sRGB profile, at the media-relative and perceptual
> intents (which are bit-identical on both sides). The residual is **entirely**
> lcms2's 4096-entry reverse tone curve: modelling it collapses the
> disagreement **457×**, to below `transicc`'s print floor.

Everything outside those three paragraphs is **not** verified. In particular
Pass 4b says nothing about: the **saturation** or **ICC-absolute** intents in any
of its three directions; `lut8` with an **XYZ** PCS (`iccce-cmm` refuses it by
name — the 8-bit XYZ encoding is unsourced, corpus A10); any **real** v4 LUT
profile (there is none on this machine); a **devicelink**; BPC; soft-proofing;
any other platform; or any **published** value. §B's four derived-expectation
rows are the strongest claims here and they are still not ground truth — §15.3.2
states exactly what they can and cannot be defeated by.

### 15.7 What §15 owes

1. **The `icc-spec-librarian` dispatch on §15.3.3's clamp question.** Until it is
   settled, one of the two implementations has a defect at every `mAB ` whose
   matrix or curves can leave `[0,1]`, and neither can be called right. The
   question is written out in §15.3.3 and should be put verbatim. *(Not
   dispatched from this session: an `icc-spec-librarian` task was already running
   in the corpus tree and a second writer there would collide.)*
2. **A corpus correction to M2** — forced BPC is decided by the **destination**
   profile's version (§15.3.4). The current wording would mislead anyone using it
   to decide whether a comparison is confounded.
3. **A corpus entry for the trilinear override** (§15.2.2), alongside M4's 4-D
   hybrid: same file, opposite direction, opposite answer.
4. **Saturation and ICC-absolute in the B2A direction.** `B2A2` exists and is a
   third distinct table; absolute would exercise §14.6's white-point policy
   through a **LUT** destination for the first time, where the D.6/D.7 composite
   is applied before the PCS is encoded rather than after.
5. **A synthetic `lut8` fixture in the suite.** §A's graded rows all skip without
   the Windows colour directory; `fixtures/synthetic/v2-cmyk-mft1-lab.icc` exists
   and is not yet wired into anything.
6. **An out-of-gamut probe for the M3 divergence.** §13.10 item 1 records that
   the size of lcms2's out-of-`[0,1]` float device output under genuine
   out-of-gamut input is still unmeasured; §A's 48 saturated-hue Lab points are
   the first grid in this suite genuinely outside the destination gamut, and the
   excursion count was **not** recorded on this run.
7. **A `NUMERIC_CLAIMS.md` mirror** of §15's numbers, and of the fact that
   **NA-006's cost is zero in the B2A direction** — a materially different
   statement from the one Pass 4 recorded, and it belongs next to it.
   `icc-librarian`'s file, not this one's.
