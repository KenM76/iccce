# `tools/difftest` — the differential oracle

**Status: Pass 0 harness exists, 2026-08-11.** The oracle is pinned, built
and demonstrated to answer questions; a minimal Rust harness now drives it
programmatically (**§11**), and the first designed experiment has been run
against it (**§12**) — settling the question `ARCHITECTURE.md` DL-011 left
open and turning up a second, unrelated version-keyed divergence on the way.

**One check is registered and it compares lcms2 against lcms2.** Nothing in
this directory has yet compared anything to `iccce`, because `iccce` has no
transform to compare (Pass 3). Said plainly here so that a green run is not
mistaken for coverage.

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
    lib.rs           drive transicc, parse it, grade the answer
    main.rs          the runner; registers the checks (§11)
    bin/
      legacy_lab_probe.rs   the DL-011 experiment (§12) — also authors the
                            synthetic probe profiles byte by byte
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
  positionally, which is the efficient way to push a whole corpus through
  in one process rather than one triplet at a time. Worth using once the
  corpus exists.
- Linux build, and therefore Linux CI (§7). The harness runs on Linux —
  it is `std`-only — but with no oracle it exits **3 (nothing ran)**.
- The general fixture corpus (`tools/gen-profiles`, `fixtures/synthetic`).
  §12's probe writes profiles byte by byte inside the harness because Pass
  2's generator does not exist; when it does, port the probe onto it.
- **Any comparison against `iccce`.** Every number here is lcms2's.
- ΔE metrics in the harness (§11 explains why absolute-only, for now).

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

**Zero dependencies, as policy.** Everything is `std`. The temptations
declined were `serde` (machine-readable output is hand-emitted TSV) and a
CLI parser. `LEGAL.md` §1 requires classifying every dependency; the
cheapest classification is the empty set.

### 11.1 Run it

```sh
cd tools/difftest
cargo test                      # the harness's own unit tests (12)
cargo run                       # the registered checks
cargo run --bin legacy_lab_probe  # the §12 experiment
```

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
- **No ΔE.** The only metric is `abs-max-component`. Adding ΔE would mean
  either depending on `iccce-color` (grading iccce with iccce's own
  arithmetic — a coupling that must be a documented decision, not a
  convenience) or writing a second ΔE2000 to get subtly wrong. The
  comparisons available today are exact-encoding questions, for which
  `ARCHITECTURE.md` DL-005 says ΔE is the wrong instrument anyway.

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
