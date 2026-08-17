# Changelog

All notable changes to `iccce` are recorded here.

This project's changelog is written to the same standard as its code: it
says **why**, not only what, and it records **what a change does not
claim** wherever that is the part a reader would otherwise assume. A
colour management module can be wrong in ways that look right, so a
changelog entry that describes a capability without its evidence class is
worse than no entry.

Format loosely follows [Keep a Changelog](https://keepachangelog.com/).
Versioning is [SemVer](https://semver.org/); while the major version is
`0`, **the public API may change in any release**.

---

## [0.1.0] — 2026-08-17

The first tagged release. Passes 0–7 were complete before it; what this
version adds is the surface a real consumer needs, and — more usefully —
the measurements that say how far to trust it.

### Added

- **A built-in sRGB destination, constructed from published constants.**
  `iccce_cmm::builtin::srgb()`. No file, no embedded profile blob, no
  I/O, no dependency. Primaries and white point from **ITU-R BT.709-6**
  items 1.3/1.4, transfer function and both breakpoints from **W3C CSS
  Color 4**, chromatic adaptation from **ICC.1:2022 Annex E.3**.

  Used when a caller states it has no destination profile, via the new
  `Chain::with_destination`.

- **`Destination` — a two-variant enum, deliberately not
  `Option<&Profile>`.** `Profile(&p)` or `None`. The distinction is the
  whole safety story: *"doesn't exist"* must mean **absent**, never
  **unresolved**. If a document declares an output intent whose profile
  fails to parse, and that error is flattened into a `None` on the way
  in, a silent substitution would render a plausible picture to the wrong
  destination with no error anywhere. An `Option` being `None` cannot
  distinguish "there was none" from "I failed to get one"; only the
  caller can, so the type makes the caller say.

- **`DestinationProvenance` — the fallback is disclosed, not silent.**
  `Chain::destination_provenance()` reports whether the destination was
  caller-supplied or constructed, with ready-to-log wording. Rule 6's
  CMM analogue: a silent substitution of the destination is the
  transform-layer version of a silently corrected tag.

- **`SpecDeviation` — the 8.10.2 tags iccce declines are now named.**
  `Chain::spec_deviations()` and `transform::mpet_deviation_for()`.
  ICC.1:2022 8.10.2 a) prefers `D2Bx`/`B2Dx` multiProcessingElements;
  iccce does not implement them and takes step b), which the clause's own
  exception permits. lcms2 does implement them. **Both engines are
  conformant and they differ by 33.13 L\*** on ICC's own
  `Probev2_ICCv4.icc`. The clause permits declining; it does not require
  silence.

- **A colour-space signature → component count accessor.**
  `iccce_profile::components()`, `channel_agreement()`, `is_valid_pcs()`.
  Answers *how many components does this profile declare* **from the
  header, before any chain is built** — which is what a PDF engine needs
  to validate an `/ICCBased` stream's `/N`.

  Sourced from ICC.1:2022 clause 7.2.6 Table 19 joined with Table 41,
  transcribed twice with two independent PDF engines and matched against
  ICC's own `icProfileHeader.h` and lcms2's `lcms2.h`.

- **`std::error::Error` for `ChainError`, `ModelError` and
  `CurveError`**, with a `source()` chain. Without it a consumer could
  not `?` a chain build into `Box<dyn Error>` — the most common error
  shape in Rust — and had to hand-wrap or `.ok()` the result and **lose
  the named refusal entirely**.

- **`iccce inspect` now reports** the declared component count, an
  unrecognised colour-space signature, a PCS field invalid for the
  device class, and any declined 8.10.2 tags.

- **CI gates wasm32 and the dependency tree.** The library crates build
  for `wasm32-unknown-unknown` on every push, and every crate in
  `cargo tree` must be one of ours. Both matter to a downstream consumer
  that enforces a wasm gate and classifies every dependency.

### Fixed

- **★ A seven-channel profile aborted the process.** `iccce bench` on a
  real `7CLR` press profile died with `0xC0000409` — *"memory allocation
  of 1022842631448 bytes failed"* — because a grid constant justified for
  3-D and 4-D was applied to every higher dimension by a `_ => 33`
  catch-all (`33⁷` ≈ 952 GiB), and the guard used `checked_pow`, which
  catches **wrap** and not **size**.

  An abort is the worst available failure for a library: not an `Err`,
  not a catchable panic, it takes the consumer's process down. Now a
  named refusal (`ChainError::GridExceedsBudget`), and the ≥5-channel
  recommendations are computed rather than tabulated.

- **A conformant `A2B`-only profile was wrongly refused**, with an error
  naming the wrong clause. The built-in-destination path obtained its
  source model by building a scaffold `src → src` chain and discarding
  the destination half; for a one-directional profile that half failed
  and surfaced *"matrix/TRC model requires PCSXYZ"* — true,
  clause-cited, and about a model iccce was about to throw away. **A
  refusal that names the wrong clause is worse than a vague one, because
  the citation makes it persuasive.**

### Changed

- **The sRGB rule-4 approximation was mis-attributed, and is corrected.**
  ICC's own *"Specification of sRGB"* (Holm, 2015) publishes the
  D50-adapted colorants at 15 decimal places — values this project had
  recorded as published by no document. Measured against them:

  | | worst cell | `bXYZ.Z` |
  |---|---|---|
  | **this construction** | **3.02 ULP** | 0.90 ULP † |
  | the shipped HP 1998 / `sRGB2014.icc` file | **11.13 ULP** | **11.13 ULP** |

  The long-standing ~12 ULP blue-Z residual is **the file's error**, not
  iccce's. The gap in the literature had been silently treated as
  evidence that the file *was* the reference.

  > **† Corrected after tagging — compare worst cells, not `bXYZ.Z`.**
  > Both numbers are right; the juxtaposition is not. Measured by
  > substituting one side at a time, this construction's `0.90 ULP` in
  > blue-Z is **a partial cancellation of two terms of ≈2.5 ULP**
  > (`−2.483` from our chromatic adaptation, `+1.585` from our D65
  > matrix) — not a small error. **The defensible figure is the worst
  > cell, `3.02 ULP`**, which is what the test bounds and what the
  > release notes quote.
  >
  > Left in place rather than deleted: removing a number is worse than
  > annotating it, and *a figure that looks like accuracy and is actually
  > a coincidence of signs* is exactly the failure this project exists to
  > catch. It survived a day because it pointed the flattering way.
  > Registered as **NA-010**.

### Measured

Numbers a reader may quote, each with its evidence class:

| claim | value | class |
|---|---|---|
| constructed sRGB vs ICC's **published** colorants | **3.02 ULP** worst (bound 4) | `published-ground-truth` |
| constructed sRGB vs `sRGB2014.icc`, 10 probes | **0.033 ΔE2000** max; black exact | constructed-vs-reference-file |
| real-world profiles parsed, two corpora | **50 accepted, `malformations: 0`** | acceptance population |
| iccMAX refused **by name**, exit 1 | **10 of 10** | refusal population |
| 7-channel `mAB` PCS corners vs lcms2 | **4.9×10⁻⁵ L\*** | cross-check |
| conformance suite, operator machine | **`pass=274 fail=0 skip=9`** | mixed; see `TOLERANCES.md` |

### ★ What this release does NOT claim

- **Not that iccce is more accurate than any other CMM.** Its oracle for
  LUT paths is lcms2 — a cross-check against another implementation, not
  ground truth. **ICC.1 mandates no interpolation method, so no published
  ground truth for a LUT path can exist even in principle**, and ICC's
  own reference implementation ships zero expected colour values.
- **Not that the CI badge evidences the conformance suite.** On a
  corpus-free runner the suite executes **15 of 201 rows** — the rest
  need a Windows system profile or private corpora no runner holds. The
  numbers that matter come from the operator's machine. A coverage floor
  now makes that visible rather than silent.
- **Not that the API is stable.** `0.x`. Several types here were added
  the same day this was tagged, and no external consumer has used them
  yet.
- **Not certified against any conformance suite.** Certification requires
  measurement hardware and is organisationally closed to a library.

### Known limitations

- **The evaluation surface is `f64` throughout.** No `f32`, no integer
  path. An 8 Mpix CMYK page is **256 MB in / 192 MB out**. This is the
  most consequential open item and it was found by a real consumer, not
  by internal review.
- `multiProcessingElements` (`mpet`) is not implemented — declined per
  8.10.2 b), and now disclosed.
- iccMAX (ICC.2) is refused by design, by name.
- Above four input channels there is **no ΔE claim of any kind** for the
  compiled path.

[0.1.0]: https://github.com/KenM76/iccce/releases/tag/v0.1.0
