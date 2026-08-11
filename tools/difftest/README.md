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
    main.rs          the runner; registers the checks (§11, §13)
    bin/
      legacy_lab_probe.rs   the DL-011 experiment (§12) — also authors the
                            synthetic probe profiles byte by byte
      pass3_report.rs       the per-point Pass 3 record, and the two
                            experiments that TEST §13's justifications
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
- **Any intent but media-relative colorimetric**, and any LUT profile.
  Pass 4.
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

1. **A dispatch to `icc-spec-librarian`** on §13.4's clamping question —
   clause 6.4's integer-vs-float32 clipping rule read together with Annex
   F.8–F.16. Not made: no Agent tool was available in the session that ran
   this.
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
