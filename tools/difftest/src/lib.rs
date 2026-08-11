//! # `iccce-difftest` — driving the oracle, and grading what it says
//!
//! This library is the programmatic half of `tools/difftest`. The other half
//! — the pin, the fetch script and the build scripts — puts a known build of
//! lcms2 on disk. This library **runs it and grades the answer**.
//!
//! Read `tools/difftest/README.md` first; it explains why the oracle is a
//! subprocess rather than a linked library, what the pin means, and where the
//! licence boundary is. This file assumes all of that and documents only the
//! mechanics.
//!
//! ## What this library is for, stated as a contract
//!
//! *Given* a colour transform expressed as (input space, output space,
//! rendering intent, precalculation mode, input values), *invoke the pinned
//! `transicc` on it*, *parse the numbers it prints*, and *compare them to a
//! stated expectation at a stated tolerance of a stated kind*, emitting a
//! machine-readable pass/fail record.
//!
//! Everything in that sentence that sounds like ceremony is load-bearing:
//!
//! - **"stated tolerance"** — `CLAUDE.md` rule 5. A tolerance carries the
//!   reason it is that number ([`Tolerance::why`]). There is no constructor
//!   that lets you supply a bare `f64`.
//! - **"stated kind"** — `TOLERANCES.md` §1. Agreement with lcms2 is a
//!   *cross-check*, not ground truth. [`Kind`] makes every record say which
//!   it is, so a weak claim cannot be quoted as a strong one later.
//! - **"stated precalculation mode"** — `README.md` §9. lcms2's precalculated
//!   transforms are an approximation *of lcms2's own exact path*. A reference
//!   number that does not say which `-c` mode produced it is not reproducible.
//!   [`Precalc`] is therefore a required field of [`Request`], not a default.
//!
//! ## What this library deliberately cannot do
//!
//! - **It cannot express a non-ICC rendering intent.** lcms2 offers intents
//!   10–15 (its black-preserving extensions). They are not ICC intents, and a
//!   difftest that wandered into them and reported "conforms" would be
//!   reporting on something the specification does not define. [`Intent`] has
//!   four variants and no escape hatch.
//! - **It cannot compute ΔE.** Every metric here is absolute, per component.
//!   Computing ΔE would mean either depending on `iccce-color` (grading iccce
//!   with iccce's own arithmetic — acceptable for some purposes but a
//!   coupling that must be a deliberate, documented decision rather than a
//!   convenience) or re-implementing ΔE2000 here (a second implementation to
//!   get subtly wrong). Neither is worth it for the first version, when the
//!   only comparisons available are exact-encoding questions that ΔE is the
//!   wrong instrument for anyway — see `ARCHITECTURE.md` DL-005.
//! - **It cannot pretend a missing oracle is a pass.** See [`Outcome::Skip`]
//!   and [`Report::exit_code`].
//!
//! ## Machine-readable output
//!
//! [`Report::emit`] writes one tab-separated record per check:
//!
//! ```text
//! check<TAB>id<TAB>status<TAB>kind<TAB>metric<TAB>tolerance<TAB>observed<TAB>detail
//! summary<TAB>pass=N<TAB>fail=N<TAB>skip=N<TAB>error=N
//! ```
//!
//! TSV rather than JSON because the fields are all short, tab-free scalars,
//! so TSV needs no escaping rules, no quoting decisions and no dependency.
//! `detail` is free text with tabs and newlines stripped ([`sanitise`]).
//!
//! **`status` is one of `PASS`, `FAIL`, `SKIP`, `ERROR`, and they are four
//! different things.** `SKIP` means the check could not run (usually a
//! category (c) system profile absent, per `LEGAL.md` §3). `ERROR` means the
//! harness or the oracle failed — a broken pipe, a non-zero exit, output that
//! did not parse. Collapsing either into `PASS` is the failure mode this
//! whole role exists to prevent.
//!
//! ## Exit-code contract (see [`Report::exit_code`])
//!
//! | code | meaning |
//! |---|---|
//! | 0 | at least one check ran and every check that ran passed |
//! | 1 | at least one check failed |
//! | 2 | at least one check errored (and none failed) |
//! | 3 | **nothing ran** — every check skipped, or the oracle is absent |
//!
//! 3 is separate from 0 on purpose. A suite that skips everything is not a
//! suite that passed, and on a machine without the oracle that is exactly
//! what would otherwise be reported.

use std::fmt;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

// ===========================================================================
// Locating the oracle
// ===========================================================================

/// A located `transicc` executable — the oracle, ready to be invoked.
///
/// Construct with [`Oracle::locate`]. Holding one of these is a statement
/// that the binary existed at construction time; it is not a statement that
/// it is the *pinned* build. Verifying the pin is [`Oracle::banner`]'s job
/// and the caller's responsibility (see [`Oracle::check_banner`]).
#[derive(Debug, Clone)]
pub struct Oracle {
    exe: PathBuf,
}

impl Oracle {
    /// Find `transicc`, in this order:
    ///
    /// 1. `$ICCCE_TRANSICC`, if set — the escape hatch for a CI runner that
    ///    builds lcms2 somewhere else. If it is set and does not exist, that
    ///    is an error rather than a fall-through: an operator who set the
    ///    variable meant it, and silently using a different binary than the
    ///    one they named is exactly how an oracle stops being reproducible.
    /// 2. `vendor/build-msvc/transicc.exe` — what `build-lcms2.ps1` produces.
    /// 3. `vendor/build-posix/transicc` — what `build-lcms2.sh` would
    ///    produce. (As of 2026-08-11 that script has never been run; see
    ///    README §7.)
    ///
    /// Returns `Ok(None)` when no oracle is present, which is a **skip**
    /// condition and not an error: a machine with no C toolchain can still
    /// check out this repository.
    pub fn locate() -> Result<Option<Oracle>, DiffError> {
        if let Some(v) = std::env::var_os("ICCCE_TRANSICC") {
            let p = PathBuf::from(v);
            if p.is_file() {
                return Ok(Some(Oracle { exe: p }));
            }
            return Err(DiffError::OracleNamedButMissing(p));
        }

        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        for candidate in [
            root.join("vendor/build-msvc/transicc.exe"),
            root.join("vendor/build-posix/transicc"),
        ] {
            if candidate.is_file() {
                return Ok(Some(Oracle { exe: candidate }));
            }
        }
        Ok(None)
    }

    /// The path this oracle will invoke. Print it in any report: "verified
    /// against lcms2" is not falsifiable unless the reader can tell *which*
    /// binary answered.
    pub fn path(&self) -> &Path {
        &self.exe
    }

    /// The banner `transicc` prints before any data, e.g.
    /// `LittleCMS ColorSpace conversion calculator - 5.1 [LittleCMS 2.19]`.
    ///
    /// Obtained by running the binary with no arguments, which makes it print
    /// its usage text. **`-h` is not a valid flag** and would be a fatal
    /// error (README §9), so do not "improve" this.
    ///
    /// ## Two measured corrections to what README §8.2/§9 originally said
    ///
    /// Both established 2026-08-11 by redirecting the two streams to separate
    /// files, which is the only way to tell them apart:
    ///
    /// 1. **The banner goes to stderr, not stdout.** stdout carries the data
    ///    line and nothing else. The original record said stdout, because on
    ///    a terminal the two streams interleave and look like one.
    /// 2. **`transicc` with no arguments exits 0**, not non-zero. So exit
    ///    status cannot be used to distinguish "printed usage" from
    ///    "converted something"; only the presence of a parsable stdout line
    ///    can.
    ///
    /// `stderr` is read first here and `stdout` second, so this keeps working
    /// if a future lcms2 moves the banner.
    pub fn banner(&self) -> Result<String, DiffError> {
        let out = Command::new(&self.exe)
            .stdin(Stdio::null())
            .output()
            .map_err(|e| DiffError::Spawn(self.exe.clone(), e))?;
        let err_text = String::from_utf8_lossy(&out.stderr).into_owned();
        let text = if err_text.trim().is_empty() {
            String::from_utf8_lossy(&out.stdout).into_owned()
        } else {
            err_text
        };
        Ok(text.lines().next().unwrap_or("").trim().to_string())
    }

    /// Assert the located binary is the version the pin claims.
    ///
    /// This is a *weak* check by design: it reads the version string the
    /// binary prints, which tells us the source it was built from called
    /// itself 2.19. It does **not** establish the commit. The commit is
    /// established by `fetch-lcms2.sh`, which verifies `git rev-parse HEAD`
    /// against `lcms2.pin` and exits 4 on mismatch; that is the real check
    /// and this one only catches "the wrong transicc is on this machine".
    pub fn check_banner(&self, expect_substring: &str) -> Result<String, DiffError> {
        let banner = self.banner()?;
        if banner.contains(expect_substring) {
            Ok(banner)
        } else {
            Err(DiffError::UnexpectedBanner {
                expected: expect_substring.to_string(),
                got: banner,
            })
        }
    }

    /// Run one conversion and return the numbers `transicc` printed.
    ///
    /// ## The mechanics, because two of them are traps
    ///
    /// - **Flags take their argument attached, with no space** (`-i<path>`,
    ///   `-t<n>`, `-c<n>`). `-i path` is a different thing and fails
    ///   confusingly. Every argument is built by concatenation below.
    /// - **Input is one component per line on stdin**, not a whitespace
    ///   triplet on one line.
    /// - **The two-line copyright banner goes to STDERR** (measured
    ///   2026-08-11 — see [`Oracle::banner`]; README §8.2's original claim of
    ///   stdout was an artefact of watching both streams on one terminal).
    ///   stdout carries only the data line — with a trailing space and a CRLF
    ///   on Windows. [`parse_values`] still takes the *last* non-empty line
    ///   rather than the first, deliberately: that is correct under either
    ///   arrangement, and costs nothing.
    ///
    /// Non-zero exit, unparsable output, or a component count different from
    /// the caller's `expect_components` all produce a [`DiffError`] carrying
    /// the captured stdout and stderr. Diagnosing an oracle failure without
    /// its output costs an hour; carrying it costs a struct field.
    pub fn convert(&self, req: &Request, expect_components: usize) -> Result<Vec<f64>, DiffError> {
        let args = req.to_args();

        let mut child = Command::new(&self.exe)
            .args(&args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| DiffError::Spawn(self.exe.clone(), e))?;

        {
            // Scoped so the pipe is closed before we wait. transicc reads
            // until EOF; if the handle outlived this block the child would
            // never finish and the harness would hang, which is a far worse
            // failure than a wrong number because it has no output at all.
            let stdin = child
                .stdin
                .as_mut()
                .ok_or_else(|| DiffError::Internal("child stdin was not piped".into()))?;
            let mut buf = String::new();
            for v in &req.values {
                // `{}` on f64 prints a shortest round-trip representation.
                // transicc parses with atof, so this is lossless in practice
                // for the values a fixture supplies.
                buf.push_str(&format!("{v}\n"));
            }
            stdin
                .write_all(buf.as_bytes())
                .map_err(DiffError::Pipe)?;
        }

        let out = child
            .wait_with_output()
            .map_err(|e| DiffError::Spawn(self.exe.clone(), e))?;

        let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();

        if !out.status.success() {
            return Err(DiffError::NonZeroExit {
                args,
                code: out.status.code(),
                stdout,
                stderr,
            });
        }

        let values = parse_values(&stdout).ok_or_else(|| DiffError::Unparsable {
            args: args.clone(),
            stdout: stdout.clone(),
            stderr: stderr.clone(),
        })?;

        if values.len() != expect_components {
            return Err(DiffError::Arity {
                expected: expect_components,
                got: values.len(),
                stdout,
            });
        }

        Ok(values)
    }
}

/// Take the last non-empty line of `transicc` stdout and parse it as
/// whitespace-separated floats.
///
/// Returns `None` if there is no such line or any token fails to parse —
/// deliberately all-or-nothing, because a partially parsed line is how a
/// harness ends up comparing against a fragment of a banner.
pub fn parse_values(stdout: &str) -> Option<Vec<f64>> {
    let line = stdout.lines().rev().find(|l| !l.trim().is_empty())?;
    let mut out = Vec::new();
    for tok in line.split_whitespace() {
        out.push(tok.parse::<f64>().ok()?);
    }
    if out.is_empty() { None } else { Some(out) }
}

// ===========================================================================
// Describing a conversion
// ===========================================================================

/// One side of a transform: either a profile on disk or one of lcms2's
/// built-in spaces.
///
/// The built-ins save a great deal of fixture work and are named with a
/// leading `*` on the command line (`*Lab`, `*XYZ`, `*sRGB`). Note
/// [`Space::lab_v2`] versus [`Space::lab_v4`]: lcms2 exposes the v2 and v4
/// CIELAB encodings as separate built-in profiles, which makes the encoding
/// hazard `ARCHITECTURE.md` §2 names selectable from the command line.
#[derive(Debug, Clone)]
pub enum Space {
    /// An lcms2 built-in, given *without* the leading `*` — it is added here.
    Builtin(String),
    /// A profile file. May be a synthetic fixture (category (a)) or a local
    /// system profile (category (c), never committed, must skip when absent).
    Profile(PathBuf),
}

impl Space {
    /// D50 CIELAB with the **v4** 16-bit encoding (`0x0000..0xFFFF`).
    pub fn lab_v4() -> Space {
        Space::Builtin("Lab4".into())
    }
    /// D50 CIELAB with the **v2 legacy** 16-bit encoding
    /// (`0x0000..0xFF00` for `L*`).
    pub fn lab_v2() -> Space {
        Space::Builtin("Lab2".into())
    }
    /// lcms2's built-in sRGB. **Not** the same object as a system sRGB
    /// profile, and results from the two must never be conflated.
    pub fn srgb_builtin() -> Space {
        Space::Builtin("sRGB".into())
    }
    pub fn profile<P: Into<PathBuf>>(p: P) -> Space {
        Space::Profile(p.into())
    }

    fn as_arg(&self) -> String {
        match self {
            Space::Builtin(name) => format!("*{name}"),
            Space::Profile(p) => p.display().to_string(),
        }
    }

    /// True when this side is a file that does not exist — the skip
    /// condition for a category (c) system profile.
    pub fn missing_file(&self) -> Option<&Path> {
        match self {
            Space::Profile(p) if !p.is_file() => Some(p),
            _ => None,
        }
    }
}

/// The four ICC rendering intents, and only those.
///
/// lcms2 additionally accepts 10–15 (black-ink-preserving extensions). They
/// are **not ICC intents**; iccce implements 0–3, and a difftest that used
/// 10–15 could not describe its result as a conformance check. There is no
/// variant for them and no `from_u32`, so the omission cannot be undone by
/// accident.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Intent {
    Perceptual = 0,
    RelativeColorimetric = 1,
    Saturation = 2,
    AbsoluteColorimetric = 3,
}

impl Intent {
    pub fn as_num(self) -> u8 {
        self as u8
    }
    pub fn name(self) -> &'static str {
        match self {
            Intent::Perceptual => "perceptual",
            Intent::RelativeColorimetric => "media-relative-colorimetric",
            Intent::Saturation => "saturation",
            Intent::AbsoluteColorimetric => "icc-absolute-colorimetric",
        }
    }
}

/// lcms2's `-c` precalculation mode. **A required field, never defaulted.**
///
/// lcms2 can flatten a transform into a sampled grid before running it. That
/// is an approximation of lcms2's *own* exact path, so a reference number is
/// only reproducible if it says which mode produced it (README §9). For
/// oracle work the right answer is nearly always [`Precalc::Exact`]: an
/// oracle should be the reference implementation's most accurate path, for
/// the same reason `fast_float` is never built (README §3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Precalc {
    /// `-c0` → `cmsFLAGS_NOOPTIMIZE`: evaluate the pipeline as read, with no
    /// grid flattening. **Use this unless the question is about flattening.**
    Exact = 0,
    /// `-c1` → lcms2's default optimisation.
    Default = 1,
    /// `-c2` → `cmsFLAGS_HIGHRESPRECALC`.
    HighRes = 2,
    /// `-c3` → `cmsFLAGS_LOWRESPRECALC`.
    LowRes = 3,
}

impl Precalc {
    pub fn as_num(self) -> u8 {
        self as u8
    }
    pub fn name(self) -> &'static str {
        match self {
            Precalc::Exact => "exact(-c0,NOOPTIMIZE)",
            Precalc::Default => "default(-c1)",
            Precalc::HighRes => "highres(-c2)",
            Precalc::LowRes => "lowres(-c3)",
        }
    }
}

/// Black point compensation — lcms2's `-b`. **A required field.**
///
/// ## Why this is not a `bool` with a `false` default
///
/// BPC changes the answer, and — measured 2026-08-11, see
/// `src/bin/legacy_lab_probe.rs` — **lcms2 turns it on by itself, without
/// being asked, for v4 profiles at the perceptual and saturation intents**:
///
/// ```text
/// cmscnvrt.c, _cmsLinkProfiles():
///     // BPC does not apply to devicelink profiles, nor to abs colorimetric,
///     // and applies always on V4 perceptual and saturation.
///     if (TheIntents[i] == INTENT_PERCEPTUAL || TheIntents[i] == INTENT_SATURATION) {
///         // Force BPC for V4 profiles in perceptual and saturation
///         if (cmsGetEncodedICCversion(hProfiles[i]) >= 0x4000000)
///             BPC[i] = TRUE;
///     }
/// ```
///
/// So [`Bpc::Off`] means *"we did not ask for it"*, **not** *"it did not
/// happen"*. Any check at perceptual or saturation against a v4 profile is
/// measuring a transform with BPC in it whether it wanted to or not. Making
/// the field mandatory at least guarantees the report says what was asked
/// for, which is the first half of noticing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bpc {
    /// `-b` absent. lcms2 may still apply BPC — see the type doc.
    Off,
    /// `-b` present.
    On,
}

impl Bpc {
    pub fn name(self) -> &'static str {
        match self {
            Bpc::Off => "not-requested",
            Bpc::On => "requested(-b)",
        }
    }
}

/// A complete, reproducible description of one oracle invocation.
///
/// Every field is required. There is no `Default` impl: a request with a
/// defaulted intent or a defaulted precalculation mode is a result nobody can
/// reproduce, and this type is the place to make that impossible.
#[derive(Debug, Clone)]
pub struct Request {
    pub input: Space,
    pub output: Space,
    pub intent: Intent,
    pub precalc: Precalc,
    pub bpc: Bpc,
    /// Input components, one per line on stdin, **in the input space's own
    /// device range** — 0–255 for 8-bit RGB, 0–100 for CMYK percentages,
    /// 0–100 / −128–127 for Lab. `transicc`'s `-e`, `-w` and `-x` flags
    /// change that convention; this harness never passes them, so the
    /// convention is fixed and stated rather than silently inherited.
    pub values: Vec<f64>,
}

impl Request {
    /// The exact argument vector passed to `transicc`.
    ///
    /// `-n` sets verbosity 0 so only the numbers (and the unavoidable banner)
    /// reach stdout. Arguments are attached to their flags with no space, per
    /// README §9.
    pub fn to_args(&self) -> Vec<String> {
        let mut args = vec![
            format!("-i{}", self.input.as_arg()),
            format!("-o{}", self.output.as_arg()),
            format!("-t{}", self.intent.as_num()),
            format!("-c{}", self.precalc.as_num()),
            "-n".to_string(),
        ];
        if self.bpc == Bpc::On {
            args.push("-b".to_string());
        }
        args
    }

    /// A one-line, human-readable restatement of the whole invocation, for
    /// the `detail` column of a report.
    pub fn describe(&self) -> String {
        format!(
            "in={} out={} intent={} precalc={} bpc={} values={:?}",
            self.input.as_arg(),
            self.output.as_arg(),
            self.intent.name(),
            self.precalc.name(),
            self.bpc.name(),
            self.values
        )
    }
}

// ===========================================================================
// Grading the answer
// ===========================================================================

/// What kind of claim a check makes. `TOLERANCES.md` §1.
///
/// **A check that does not state its kind is not finished.** The variants are
/// ordered strongest to weakest as *correctness* evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// A published CIE or vendor value, transcribed with its source. The
    /// strongest claim available. **Nothing in this crate can produce one** —
    /// ground truth does not come out of an implementation. The variant
    /// exists so a harness check that compares an oracle result against a
    /// *published* number can say so.
    GroundTruth,
    /// Agreement between iccce and lcms2. Evidence that two implementations
    /// read a clause the same way; two implementations can share a misreading.
    CrossCheck,
    /// Both sides are iccce (round trip, compiled-vs-reference). Prices an
    /// approximation; worthless as correctness evidence.
    SelfConsistency,
    /// **Both sides are lcms2**: today's oracle output against a previously
    /// recorded oracle output. Says nothing whatever about colour
    /// correctness. It detects a changed pin, a changed toolchain, a changed
    /// profile, or a harness that has stopped driving the oracle properly —
    /// which is exactly what a smoke test is for, and exactly what it must
    /// not be mistaken for.
    OracleReproducibility,
}

impl Kind {
    pub fn tag(self) -> &'static str {
        match self {
            Kind::GroundTruth => "ground-truth",
            Kind::CrossCheck => "cross-check",
            Kind::SelfConsistency => "self-consistency",
            Kind::OracleReproducibility => "oracle-reproducibility",
        }
    }
}

/// How two vectors of numbers are compared.
///
/// Only one variant today. ΔE metrics are deliberately absent — see the
/// module header, "What this library deliberately cannot do".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Metric {
    /// Maximum absolute difference over components, in the output space's own
    /// units. For Lab that is `L*`/`a*`/`b*` units, **not** ΔE: the largest
    /// single-component error, which is the conservative reading.
    AbsMaxComponent,
}

impl Metric {
    pub fn tag(self) -> &'static str {
        match self {
            Metric::AbsMaxComponent => "abs-max-component",
        }
    }

    fn measure(self, got: &[f64], expected: &[f64]) -> f64 {
        match self {
            Metric::AbsMaxComponent => got
                .iter()
                .zip(expected)
                .map(|(g, e)| (g - e).abs())
                .fold(0.0_f64, f64::max),
        }
    }
}

/// A tolerance **and the reason it is that number**.
///
/// There is no way to build one without a justification, because the failure
/// mode this whole document family exists to prevent is a bare `0.5` in a
/// source file that nobody can defend and everybody can widen.
#[derive(Debug, Clone, Copy)]
pub struct Tolerance {
    pub value: f64,
    /// Why *this* number. "It passed" is not a justification; if that is the
    /// true reason, the honest thing is to leave the check unwritten.
    pub why: &'static str,
}

impl Tolerance {
    pub const fn new(value: f64, why: &'static str) -> Tolerance {
        Tolerance { value, why }
    }
}

/// One graded comparison: what to run, what to expect, how close is close
/// enough, and where the expectation came from.
#[derive(Debug, Clone)]
pub struct Check {
    /// Stable identifier, quoted in reports and in `TOLERANCES.md`.
    pub id: &'static str,
    pub kind: Kind,
    pub metric: Metric,
    pub tolerance: Tolerance,
    pub request: Request,
    /// The expected output components.
    pub expected: Vec<f64>,
    /// **Where the expectation came from.** A citation for ground truth; a
    /// document section and date for a recorded oracle value. A check whose
    /// expectation came from the thing under test detects change, not error
    /// (`CLAUDE.md` rule 3), and this field is where that becomes visible.
    pub source: &'static str,
}

/// The result of running a [`Check`]. Four outcomes, and they are four
/// different things — see the module header.
#[derive(Debug, Clone)]
pub enum Outcome {
    Pass { observed: f64, got: Vec<f64> },
    Fail { observed: f64, got: Vec<f64> },
    /// Could not run. Carries the reason, which is always printed: a skip
    /// whose reason is not recorded is indistinguishable from a pass in a
    /// summary line, and that is how coverage silently goes to zero.
    Skip { reason: String },
    /// The harness or the oracle failed.
    Error { detail: String },
}

impl Outcome {
    pub fn tag(&self) -> &'static str {
        match self {
            Outcome::Pass { .. } => "PASS",
            Outcome::Fail { .. } => "FAIL",
            Outcome::Skip { .. } => "SKIP",
            Outcome::Error { .. } => "ERROR",
        }
    }
}

impl Check {
    /// Run this check against `oracle`, or skip it if it cannot run.
    ///
    /// Skips (never failures) when either side of the transform names a
    /// profile file that is absent — the category (c) rule of `LEGAL.md` §3:
    /// a system profile may be read locally and must never be a required
    /// input.
    pub fn run(&self, oracle: &Oracle) -> Outcome {
        for side in [&self.request.input, &self.request.output] {
            if let Some(p) = side.missing_file() {
                return Outcome::Skip {
                    reason: format!(
                        "profile not present on this machine: {} (LEGAL.md §3 category (c): read locally, never committed, never a required input)",
                        p.display()
                    ),
                };
            }
        }

        match oracle.convert(&self.request, self.expected.len()) {
            Err(e) => Outcome::Error {
                detail: e.to_string(),
            },
            Ok(got) => {
                let observed = self.metric.measure(&got, &self.expected);
                if observed <= self.tolerance.value {
                    Outcome::Pass { observed, got }
                } else {
                    Outcome::Fail { observed, got }
                }
            }
        }
    }
}

/// A run of checks, and its machine-readable rendering.
#[derive(Debug, Default)]
pub struct Report {
    rows: Vec<(Check, Outcome)>,
    /// Free-text notes emitted as `note` records — the provenance of the run
    /// (which binary, which banner) belongs here.
    notes: Vec<String>,
}

impl Report {
    pub fn new() -> Report {
        Report::default()
    }

    pub fn note<S: Into<String>>(&mut self, s: S) {
        self.notes.push(s.into());
    }

    pub fn push(&mut self, check: Check, outcome: Outcome) {
        self.rows.push((check, outcome));
    }

    pub fn counts(&self) -> (usize, usize, usize, usize) {
        let mut c = (0, 0, 0, 0);
        for (_, o) in &self.rows {
            match o {
                Outcome::Pass { .. } => c.0 += 1,
                Outcome::Fail { .. } => c.1 += 1,
                Outcome::Skip { .. } => c.2 += 1,
                Outcome::Error { .. } => c.3 += 1,
            }
        }
        c
    }

    /// See the exit-code table in the module header. In particular **3 is
    /// "nothing ran"**, which is not success.
    pub fn exit_code(&self) -> i32 {
        let (pass, fail, _skip, error) = self.counts();
        if fail > 0 {
            1
        } else if error > 0 {
            2
        } else if pass == 0 {
            3
        } else {
            0
        }
    }

    /// Write the TSV records described in the module header.
    pub fn emit<W: Write>(&self, w: &mut W) -> io::Result<()> {
        for n in &self.notes {
            writeln!(w, "note\t{}", sanitise(n))?;
        }
        for (check, outcome) in &self.rows {
            let (observed, detail) = match outcome {
                Outcome::Pass { observed, got } | Outcome::Fail { observed, got } => (
                    format!("{observed:.6e}"),
                    format!(
                        "{} | got={:?} expected={:?} | tolerance because: {} | expectation source: {}",
                        check.request.describe(),
                        got,
                        check.expected,
                        check.tolerance.why,
                        check.source
                    ),
                ),
                Outcome::Skip { reason } => ("-".to_string(), reason.clone()),
                Outcome::Error { detail } => ("-".to_string(), detail.clone()),
            };
            writeln!(
                w,
                "check\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                check.id,
                outcome.tag(),
                check.kind.tag(),
                check.metric.tag(),
                check.tolerance.value,
                observed,
                sanitise(&detail)
            )?;
        }
        let (pass, fail, skip, error) = self.counts();
        writeln!(
            w,
            "summary\tpass={pass}\tfail={fail}\tskip={skip}\terror={error}"
        )?;
        Ok(())
    }
}

/// Strip tabs and newlines so a free-text field cannot break the TSV framing.
pub fn sanitise(s: &str) -> String {
    s.chars()
        .map(|c| if c == '\t' || c == '\n' || c == '\r' { ' ' } else { c })
        .collect()
}

// ===========================================================================
// Errors
// ===========================================================================

/// Everything that can go wrong driving the oracle. Each variant carries
/// enough context to diagnose it without re-running by hand.
#[derive(Debug)]
pub enum DiffError {
    /// `$ICCCE_TRANSICC` was set to a path that is not a file. Deliberately
    /// an error and not a fall-through — see [`Oracle::locate`].
    OracleNamedButMissing(PathBuf),
    Spawn(PathBuf, io::Error),
    Pipe(io::Error),
    NonZeroExit {
        args: Vec<String>,
        code: Option<i32>,
        stdout: String,
        stderr: String,
    },
    Unparsable {
        args: Vec<String>,
        stdout: String,
        stderr: String,
    },
    Arity {
        expected: usize,
        got: usize,
        stdout: String,
    },
    UnexpectedBanner {
        expected: String,
        got: String,
    },
    Internal(String),
}

impl fmt::Display for DiffError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiffError::OracleNamedButMissing(p) => write!(
                f,
                "ICCCE_TRANSICC is set to {} but that is not a file",
                p.display()
            ),
            DiffError::Spawn(p, e) => write!(f, "could not run {}: {e}", p.display()),
            DiffError::Pipe(e) => write!(f, "failed writing to transicc stdin: {e}"),
            DiffError::NonZeroExit {
                args,
                code,
                stdout,
                stderr,
            } => write!(
                f,
                "transicc {args:?} exited {code:?}; stdout={stdout:?} stderr={stderr:?}"
            ),
            DiffError::Unparsable {
                args,
                stdout,
                stderr,
            } => write!(
                f,
                "transicc {args:?} produced no parsable numeric line; stdout={stdout:?} stderr={stderr:?}"
            ),
            DiffError::Arity {
                expected,
                got,
                stdout,
            } => write!(
                f,
                "expected {expected} components, transicc printed {got}; stdout={stdout:?}"
            ),
            DiffError::UnexpectedBanner { expected, got } => write!(
                f,
                "transicc banner does not contain {expected:?}; got {got:?}"
            ),
            DiffError::Internal(s) => write!(f, "internal harness error: {s}"),
        }
    }
}

impl std::error::Error for DiffError {}

// ===========================================================================
// Tests — of the harness itself, not of any colour
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// The banner hazard, pinned as a test. README §8.2 records the real
    /// two-line banner; the numbers are on the last line, never the first.
    #[test]
    fn parse_values_takes_the_last_line_not_the_first() {
        let stdout = "LittleCMS ColorSpace conversion calculator - 5.1 [LittleCMS 2.19]\n\
                      Copyright (c) 1998-2026 Marti Maria Saguer. See COPYING file for details.\n\
                      99.9988 0.0188 -0.0173\n";
        let v = parse_values(stdout).expect("should parse");
        assert_eq!(v.len(), 3);
        assert!((v[0] - 99.9988).abs() < 1e-12);
        assert!((v[2] + 0.0173).abs() < 1e-12);
    }

    /// Trailing blank lines must not hide the data.
    #[test]
    fn parse_values_skips_trailing_blank_lines() {
        let v = parse_values("banner\n1 2 3\n\n\n").expect("should parse");
        assert_eq!(v, vec![1.0, 2.0, 3.0]);
    }

    /// All-or-nothing: a line that is partly text is not partly data.
    #[test]
    fn parse_values_refuses_a_partially_numeric_line() {
        assert!(parse_values("1 2 oops").is_none());
        assert!(parse_values("").is_none());
        assert!(parse_values("banner only\n").is_none());
    }

    /// Flags must be attached to their arguments with no space (README §9).
    #[test]
    fn args_attach_their_values() {
        let req = Request {
            input: Space::profile("X:/p.icc"),
            output: Space::lab_v4(),
            intent: Intent::RelativeColorimetric,
            precalc: Precalc::Exact,
            bpc: Bpc::Off,
            values: vec![255.0, 255.0, 255.0],
        };
        let args = req.to_args();
        assert_eq!(args[1], "-o*Lab4");
        assert_eq!(args[2], "-t1");
        assert_eq!(args[3], "-c0");
        assert_eq!(args[4], "-n");
        assert!(args[0].starts_with("-i"));
        assert!(!args[0].contains("-i "));
    }

    /// A run in which everything skipped is NOT a pass.
    #[test]
    fn all_skipped_is_exit_3_not_0() {
        let check = Check {
            id: "dummy",
            kind: Kind::OracleReproducibility,
            metric: Metric::AbsMaxComponent,
            tolerance: Tolerance::new(1e-4, "test fixture"),
            request: Request {
                input: Space::profile("X:/does-not-exist.icc"),
                output: Space::lab_v4(),
                intent: Intent::RelativeColorimetric,
                precalc: Precalc::Exact,
                bpc: Bpc::Off,
                values: vec![0.0],
            },
            expected: vec![0.0],
            source: "test fixture",
        };
        let mut r = Report::new();
        r.push(
            check,
            Outcome::Skip {
                reason: "absent".into(),
            },
        );
        assert_eq!(r.exit_code(), 3);
    }

    /// The metric is the largest single-component error, not a mean.
    #[test]
    fn abs_max_component_is_the_max_not_the_mean() {
        let m = Metric::AbsMaxComponent;
        assert!((m.measure(&[1.0, 2.0, 3.0], &[1.0, 2.0, 3.5]) - 0.5).abs() < 1e-15);
    }

    /// TSV framing survives a detail string containing tabs and newlines.
    #[test]
    fn sanitise_removes_framing_characters() {
        let s = sanitise("a\tb\nc\rd");
        assert_eq!(s, "a b c d");
    }
}
