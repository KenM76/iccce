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
//! - **It cannot pretend a missing oracle is a pass.** See [`Outcome::Skip`]
//!   and [`Report::exit_code`].
//!
//! ## ★ What changed on 2026-08-11 (Pass 3): ΔE is now computable, deliberately
//!
//! Until Pass 3 this header said the harness *cannot* compute ΔE, on the
//! grounds that doing so would mean either depending on `iccce-color` — which
//! is grading iccce with iccce's own arithmetic — or writing a second ΔE2000
//! to get subtly wrong. It also said the coupling **"must be a deliberate,
//! documented decision rather than a convenience"**. Pass 3 needs a perceptual
//! statement about an iccce-vs-lcms2 disagreement, so the decision has been
//! taken and is recorded in `Cargo.toml`'s header and in README §13.2. In one
//! paragraph:
//!
//! - The metric (`iccce_color::delta_e_2000`) is graded against **all 34
//!   published pairs of Sharma, Wu & Dalal (2005)** at 1×10⁻⁴ — the one
//!   ground-truth row in `TOLERANCES.md` §3.1.1. It is a ruler checked
//!   against the literature, not against itself.
//! - **The claim is unchanged.** Every iccce-vs-lcms2 record is
//!   [`Kind::CrossCheck`] regardless of how well-validated the ΔE code is.
//!   A good ruler does not turn a cross-check into ground truth.
//! - **The answers still come from subprocesses.** iccce's numbers come from
//!   running the shipped `iccce transform` binary ([`Iccce`]), lcms2's from
//!   running `transicc` ([`Oracle`]). The linked crates are the *instrument*,
//!   never the *subject*.
//! - The instrument is itself cross-checked: `pass3` carries both sides'
//!   device outputs into Lab twice — once through iccce's own destination
//!   model, once through the oracle — and reports the disagreement between
//!   the two rulers.
//!
//! What the harness still cannot do is *invent* a ΔE for a comparison whose
//! output space has no colorimetric meaning. [`Metric`] therefore keeps its
//! absolute per-component variants, and `ARCHITECTURE.md` DL-005 still governs
//! encoding questions: **legacy-Lab correctness is asserted by exact-value
//! integer invariants, never by ΔE.**
//!
//! ## ★★★ What changed on 2026-08-12: candidate separation is an emitted field
//!
//! On 2026-08-12 this suite established (DL-033, README §19.10) that
//! **agreement with the oracle had been the symptom of one of our own
//! defects**. `bpc.rs`'s non-conformant 4.2.5.4 branch returned `outRamp[first]
//! = MinL = 16.489806`; lcms2's answer is `16.571474`. The wrong answer sat
//! **0.082 ΔE76** from the oracle and the right one sits **4.799** away, so a
//! **4.717441 `L*`** defect produced an 0.082 signal in the cross-check built
//! to catch it — the defect's own magnitude was **57.8× the divergence it was
//! blamed for.**
//!
//! The general statement, which is what this section exists to mechanise:
//!
//! > **A cross-check's power is bounded by the distance between the answer it
//! > observed and the answer it would have observed under a plausible rival
//! > reading.** Agreement to 0.08 is worth nothing if the wrong answers also
//! > sit within 0.08, and is strong evidence if they sit 5 apart. Nothing in
//! > this harness recorded that distance, so the difference between those two
//! > situations was invisible in every record we keep.
//!
//! [`Separation`] makes it visible. Every [`Record`] carries one, and it has
//! **three honest states** which are deliberately not collapsed:
//!
//! | state | meaning |
//! |---|---|
//! | [`Separation::Measured`] | a **named** rival candidate exists, and here is the value this row would have observed under it, and the distance |
//! | [`Separation::NoNamedAlternative`] | somebody looked and there is no rival to name — **a real statement**, and it carries the reason |
//! | [`Separation::Unstated`] | **nobody has considered this row yet.** The default, and it prints as `UNSTATED` rather than as a blank, because silence and "no alternative exists" are different claims |
//!
//! ### The blindness test, which is machine-detectable
//!
//! A row whose candidates are **closer together than its own tolerance** cannot
//! discriminate them: it passes whichever answer the code produced. That is
//! exactly the configuration that hid the 4.2.5.4 defect, and
//! [`Record::separation_power`] flags it as [`SepPower::Blind`] with no human
//! in the loop. Two guards on that test, both load-bearing:
//!
//! - **It is only applied when the separation is in the row's own metric's
//!   units** ([`SepUnits::SameAsMetric`]). Comparing a separation in device
//!   units against a tolerance on a dimensionless ratio is arithmetic about
//!   incommensurable quantities — Pass 6 row R5's lesson — so that case reports
//!   [`SepPower::Incommensurate`] and the number, and does **not** pretend to a
//!   verdict.
//! - **A row with an infinite tolerance is [`SepPower::Ungraded`], never
//!   "blind".** It cannot fail, so no separation could have saved it; saying
//!   `BLIND` there would blame the fixture for a decision the tolerance made.
//! - ★★ **A separation of exactly zero is [`SepPower::ZeroSeparation`]**, which
//!   outranks every other verdict: the two candidates *are the same number*, so
//!   the row cannot move at any tolerance, in any units. This is the state
//!   `pass5c/synthetic/*` is in — the authored fixture's `InitialLab` and
//!   `outRamp[first]` are both `L* 20` — and it is the machine-readable form of
//!   `ARCHITECTURE.md` DL-036. A blind row might be rescued by tightening a
//!   tolerance; a zero-separation row can only be rescued by **a different
//!   fixture**, which is why the two are counted and listed separately.
//!
//! **A flag is not a failure.** `BLIND` does not change `status` or the exit
//! code, and that is deliberate: a small separation can be legitimate (an exact
//! invariant, a null control), and auto-failing it would create pressure to
//! stop stating separations at all — the precise opposite of the point. It is
//! counted on the `separation` line and the offending ids are named there.
//!
//! ## Machine-readable output
//!
//! [`Report::emit`] writes one tab-separated record per check:
//!
//! ```text
//! check<TAB>id<TAB>status<TAB>kind<TAB>metric<TAB>tolerance<TAB>observed<TAB>separation<TAB>sep-power<TAB>detail
//! summary<TAB>pass=N<TAB>fail=N<TAB>skip=N<TAB>error=N
//! separation<TAB>unstated=N<TAB>no-named-alternative=N<TAB>incommensurate=N<TAB>ungraded=N<TAB>zero-separation=N<TAB>blind=N<TAB>discriminating=N<TAB>sep-broken=N<TAB>blind-ids=…<TAB>zero-separation-ids=…
//! ```
//!
//! TSV rather than JSON because the fields are all short, tab-free scalars,
//! so TSV needs no escaping rules, no quoting decisions and no dependency.
//! `detail` is free text with tabs and newlines stripped ([`sanitise`]), and it
//! stays **last** so the machine-readable fields remain contiguous.
//!
//! **The `summary` line is byte-for-byte what it always was.** The separation
//! aggregate is a *separate* line rather than four more fields on `summary`,
//! because `pass=N fail=N skip=N error=N` is quoted in commit messages and in
//! `SESSION_LOG.md` as a run's signature, and changing the shape of a line
//! people compare by eye is a way to make two identical runs look different.
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

pub mod pass3;
pub mod pass4;
pub mod pass4b;
pub mod pass4c;
pub mod pass5;
pub mod pass5b;
pub mod pass5c;
pub mod pass6;
// ★ Pass G — the Ghent v5.0 population sample. The first differential grading
// this project has done against profiles a *real document producer* embeds,
// rather than synthetic, OS-shipped or standards-body-issued ones. It reads a
// **licensed, uncommittable** corpus via `$ICCCE_PRIVATE_FIXTURES` and skips
// with a reason when it is absent, which is the permanent state in CI. The
// licensing rules that bind anyone editing it are in the module's own header.
//
// This is a `//` comment and not a `///` one deliberately: an outer doc comment
// here is merged with `passg.rs`'s `//!` header, and rustdoc then resolves that
// header's intra-doc links in THIS module's scope, where `passg`'s own types are
// not visible. The result is a doc build that fails on links which are correct.
pub mod passg;

// ★ Pass H — acceptance and refusal, over the ICC's own published profile set.
// Its subject is not a colour value (that corpus publishes transforms, never
// expected outputs — DL-041) but **which files are accepted, which are refused,
// and whether a refusal says why**. It carries the first `Kind::GroundTruth`
// rows in this crate, sourced from the ICC's published Probe-profile readme.
// Same licensing posture and same `$ICCCE_PRIVATE_FIXTURES` resolution as Pass
// G; same `//`-not-`///` reason as the comment above.
pub mod passh;

// ★ Pass I — ICC's PUBLISHED chromatic-adaptation matrix, graded cell by cell
// against `iccce_color::adaptation_matrix`. The project's third
// `published-ground-truth` subject and the first for chromatic adaptation,
// resting on the document ICC.1:2022 Annex E.4.2 points at and this project
// recorded as not obtained until 2026-08-17.
//
// Two things about it are unusual enough to state at the declaration. First, it
// takes **no `Oracle`**: no lcms2, no profile, no fixture, no environment
// variable — it runs identically on a bare CI machine, because a
// ground-truth-shaped row must not be hostage to an oracle. Second, its bounds
// are **exact-arithmetic predictions made before the pass was first run**, not
// observations rounded up; the residual against ICC's published matrix is
// non-zero and is the sum of two documented input differences (see the module
// header, which also records that a bound derived from the cone-matrix term
// alone — the derivation this pass was commissioned with — would have failed by
// 7.4x). Same `//`-not-`///` reason as the comments above.
pub mod passi;

// ★ Pass K — BLACK PRESERVATION, and the letter names the CHANNEL, not a
// position in the alphabet: this module's whole subject is the fourth colorant
// of a CMYK separation, and nothing in it depends on a Pass J existing.
//
// It is the instrument for a feature that does NOT EXIST YET. `crates/`
// contains no black-preservation code at the commit it was written against;
// the point of writing it first is that its numbers were chosen before anybody
// knew which numbers would be convenient.
//
// Three things about it belong at the declaration. First, it reaches lcms2's
// **non-ICC** rendering intents 10-15 through its own `KOnlyOracle` type rather
// than through `Intent`, so this crate's standing promise that it "cannot
// express a non-ICC rendering intent" is kept intact and every row from that
// oracle carries a mandatory caveat string. Second, its headline finding is
// about THIS harness as much as about the engine: a K-only build re-separated
// into a real CMYK destination comes back with up to 0.705 of chromatic ink
// while sitting 0.13 dE2000 from where it started, so **dE is blind to the
// defect black preservation exists to fix** and every preservation row here is
// in device units. Third, two rows were RED BY DESIGN until the feature landed
// on 2026-08-18; both are green now and NEITHER BOUND WAS TOUCHED - the
// tolerance on both is still exactly zero, written before the code existed.
// The licensed half still SKIPs in CI for ever; the committed half (§F) does
// not, and it is where the in-CI evidence lives. Same `//`-not-`///` reason as
// the comments above.
pub mod passk;

// Pass L - which reading of sRGB does lcms2 implement? Two currently-in-force
// standards define sRGB with different transfer-function constants (ICC_Spec
// A57, filed OPEN), the two curves are 4.8e-6 apart at most, and they are
// EXACTLY equal over all 256 8-bit codes. The pass measures which one lcms2
// implements - a fact about an implementation, NOT a resolution of A57 - and
// prices the choice in dE2000. Read its module header before quoting any of
// it: the maximum separation is INTERIOR and sits in a DIFFERENT PLACE for
// each output space, and the oracle is demonstrably NOT usable as the ruler
// for the destination half. Same `//`-not-`///` reason as the comments above.
pub mod passl;

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
            stdin.write_all(buf.as_bytes()).map_err(DiffError::Pipe)?;
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

    /// Run **one** `transicc` invocation over **many** input triples and
    /// return one output row per input row.
    ///
    /// ## Why batch, and why it is not just a loop
    ///
    /// `transicc` reads components from stdin until EOF and prints one line
    /// per completed tuple. A 133-point grid is therefore *one* process, not
    /// 133 — which matters less for speed than for **provenance**: every row
    /// in the returned grid came from a single transform lcms2 built once,
    /// so a per-invocation difference (a re-read profile, a re-linked
    /// pipeline) cannot vary between rows and masquerade as a colour effect.
    ///
    /// `req.values` must already be flattened: `components_per_row *
    /// row_count` numbers, one per line. The row count is derived, and a
    /// short or long final row is an [`DiffError::Arity`] rather than a
    /// silently truncated grid.
    ///
    /// **Stdin is written on a background thread.** With ~400 numbers the
    /// write is far larger than a pipe buffer, and `transicc` interleaves
    /// reading with writing; a single-threaded write-then-wait deadlocks the
    /// moment its stdout buffer fills. That deadlock is worse than a wrong
    /// number because it produces no output at all.
    pub fn convert_batch(
        &self,
        req: &Request,
        components_per_row: usize,
    ) -> Result<Vec<Vec<f64>>, DiffError> {
        self.convert_batch_shaped(req, components_per_row, components_per_row)
    }

    /// ★ **Added 2026-08-11 for Pass 4.** As [`Oracle::convert_batch`], but for
    /// a transform whose input and output arities **differ**.
    ///
    /// Pass 3 only ever converted RGB → RGB, so one width served for both and
    /// [`Oracle::convert_batch`] took a single `components_per_row`. A CMYK
    /// source into an RGB destination is 4 in and 3 out, and feeding that to
    /// the single-width version reports *"1364 input values is not a whole
    /// number of 3-component rows"* — which is the arity guard doing its job on
    /// the wrong quantity. Splitting the two widths is the fix; the guard
    /// stays, now applied to each side separately, because a short final row
    /// on either side is a real disagreement about the shape of the answer and
    /// must not be silently truncated.
    pub fn convert_batch_shaped(
        &self,
        req: &Request,
        in_components: usize,
        out_components: usize,
    ) -> Result<Vec<Vec<f64>>, DiffError> {
        assert!(
            in_components > 0 && out_components > 0,
            "widths must be > 0"
        );
        let components_per_row = out_components;
        if req.values.len() % in_components != 0 {
            return Err(DiffError::Internal(format!(
                "{} input values is not a whole number of {in_components}-component rows",
                req.values.len()
            )));
        }
        let rows_in = req.values.len() / in_components;
        let args = req.to_args();

        let mut child = Command::new(&self.exe)
            .args(&args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| DiffError::Spawn(self.exe.clone(), e))?;

        let mut buf = String::with_capacity(req.values.len() * 20);
        for v in &req.values {
            // `{v}` prints f64's shortest round-trip form; transicc parses
            // with atof, so nothing is lost on the way in.
            buf.push_str(&format!("{v}\n"));
        }
        let mut stdin = child
            .stdin
            .take()
            .ok_or_else(|| DiffError::Internal("child stdin was not piped".into()))?;
        let writer = std::thread::spawn(move || -> io::Result<()> {
            stdin.write_all(buf.as_bytes())?;
            stdin.flush()
            // `stdin` is dropped here, closing the pipe. transicc reads until
            // EOF; without this drop it would never finish.
        });

        let out = child
            .wait_with_output()
            .map_err(|e| DiffError::Spawn(self.exe.clone(), e))?;
        match writer.join() {
            Ok(Ok(())) => {}
            Ok(Err(e)) => return Err(DiffError::Pipe(e)),
            Err(_) => return Err(DiffError::Internal("stdin writer thread panicked".into())),
        }

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

        let rows =
            parse_rows(&stdout, components_per_row).ok_or_else(|| DiffError::Unparsable {
                args: args.clone(),
                stdout: stdout.clone(),
                stderr: stderr.clone(),
            })?;
        if rows.len() != rows_in {
            return Err(DiffError::Arity {
                expected: rows_in,
                got: rows.len(),
                stdout,
            });
        }
        Ok(rows)
    }
}

/// Every fully numeric line of `stdout`, parsed as `components_per_row`
/// floats each.
///
/// Lines that are not entirely numeric are **dropped, not failed** — that is
/// how the banner is skipped without hard-coding what the banner says. A
/// numeric line with the wrong component count *is* a failure (`None`),
/// because that is a real disagreement about the shape of the answer and
/// silently dropping it would shorten the grid.
///
/// Returns `None` if nothing parsed at all.
pub fn parse_rows(stdout: &str, components_per_row: usize) -> Option<Vec<Vec<f64>>> {
    let mut rows = Vec::new();
    for line in stdout.lines() {
        let t = line.trim();
        if t.is_empty() {
            continue;
        }
        let mut vals = Vec::new();
        let mut all_numeric = true;
        for tok in t.split_whitespace() {
            match tok.parse::<f64>() {
                Ok(v) => vals.push(v),
                Err(_) => {
                    all_numeric = false;
                    break;
                }
            }
        }
        if !all_numeric || vals.is_empty() {
            continue; // banner, copyright, prompt — not data.
        }
        if vals.len() != components_per_row {
            return None;
        }
        rows.push(vals);
    }
    if rows.is_empty() { None } else { Some(rows) }
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
// Driving iccce — the OTHER subprocess
// ===========================================================================

/// A located `iccce` executable: **the code under test, invoked exactly the
/// way the oracle is invoked.**
///
/// ## Why a subprocess and not a function call
///
/// This crate links `iccce-cmm` (see `Cargo.toml`'s header), so calling
/// `MatrixTrcTransform::convert` in-process would be one line shorter. It is
/// forbidden here, for two reasons that are the whole point of the harness:
///
/// 1. **Symmetry.** lcms2's answer crosses a process boundary, is printed as
///    text, and is parsed back. If iccce's answer did not, the two sides
///    would differ in more than the arithmetic — printing, rounding and
///    argument handling would all be exercised on one side only, and a bug in
///    the *shipped* surface (`iccce transform`) would be invisible to the very
///    test that exists to find it.
/// 2. **What is actually shipped.** `target/release/iccce.exe` is the
///    artefact a user runs. Testing a library call and reporting it as "iccce
///    agrees with lcms2" would be a claim about code that is one wrapper away
///    from the claim's subject.
///
/// The linked crates are used only as the *measuring instrument* (device →
/// PCS → Lab → ΔE), never to produce the answer under test.
///
/// ## The wire format, which is not the oracle's
///
/// `iccce transform` reads **one whitespace-separated triple per line, floats
/// in 0..1**, and writes one converted triple per line at **6 decimals**.
/// `transicc` reads **one component per line in the device's own range**
/// (0–255 for 8-bit RGB) and prints **4 decimals**. The two conventions are
/// different in *both* directions, and every comparison in `pass3` states the
/// scaling it applied. Mixing them silently rescales everything by 255, which
/// looks like a catastrophic colour error rather than a units bug and wastes
/// an afternoon.
#[derive(Debug, Clone)]
pub struct Iccce {
    exe: PathBuf,
}

/// ★ **Added 2026-08-17 for Pass H.** Everything one invocation of the shipped
/// binary said, kept apart so a grader can assert on each piece.
///
/// The three fields are three different claims and Pass H grades all three:
///
/// - `code` — **the bare exit status of the child process.** Not a pipeline's,
///   not a shell's. `None` when the process did not exit normally at all,
///   which is a distinct and gradeable outcome from "exited non-zero".
/// - `stdout` — for a refusal this must be **empty**. `CLAUDE.md` rule 6: the
///   parser reports, it does not repair, and a profile it declined to parse
///   must not have produced a partial dump that a caller could mistake for a
///   successful read.
/// - `stderr` — the **reason**, which is the deliverable. A refusal that says
///   "parse error" where it should say "iccMAX" is a different behaviour, and
///   a row that only checked the exit code would not see the difference.
#[derive(Debug, Clone)]
pub struct InspectRun {
    pub code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

impl InspectRun {
    /// The value of a `key: value` line in `iccce inspect`'s stdout, whose
    /// format is deliberately a diff surface rather than a human UI
    /// (`crates/iccce-cli/src/main.rs`).
    #[must_use]
    pub fn field(&self, key: &str) -> Option<&str> {
        self.stdout
            .lines()
            .find_map(|l| l.strip_prefix(key)?.strip_prefix(": "))
    }
}

/// The optional flags `iccce transform` accepts, as one value.
///
/// ## Why a struct and not a tail of positional parameters
///
/// The shipped binary has gained **one optional flag per major feature** —
/// `--bpc` when Pass 5 needed it, `--preserve-black` when Pass K's subject
/// landed — and there is no reason to expect that to stop. A positional
/// `bool, Option<&str>` tail reads acceptably at two flags, becomes unreadable
/// at three, and forces every call site in this crate to be edited when the
/// third arrives. Gathering them means a new flag is an added field with a
/// [`Default`], and existing callers keep compiling.
///
/// ★ **What it must NOT become is a second entry point.** The reason both
/// wrappers above funnel into one function is that Pass 5 and Pass K each
/// compare *the same probe set through the same code path with one flag
/// changed*, and a bit-identical result is the observation. Two functions
/// would be free to drift in the intent, the profile order or the stdin
/// encoding, and the difference would then be attributed to the flag. This
/// struct is what keeps the single path while keeping the signature readable.
#[derive(Debug, Clone, Copy, Default)]
pub struct TransformOpts<'a> {
    /// `--bpc`. Black point compensation, never forced; the binary refuses by
    /// name where it cannot estimate.
    pub bpc: bool,
    /// `--preserve-black <policy>`. **A policy NAME, not a flag**, because the
    /// binary has no default and refuses a bare `--preserve-black`: two
    /// published definitions of "preserve the black" disagree by up to
    /// `4.9e-2`, and a number measured under one is uninterpretable beside a
    /// number measured under the other.
    ///
    /// Held as a `&str` rather than an enum on purpose: the harness must stay
    /// able to send a policy name the binary does not know, because *"an
    /// unknown policy is refused and not substituted"* is itself a behaviour
    /// worth being able to drive.
    pub preserve_black: Option<&'a str>,
}

impl Iccce {
    /// Find the `iccce` binary, in this order:
    ///
    /// 1. `$ICCCE_BIN`, if set. As with `$ICCCE_TRANSICC`, a variable that is
    ///    set but names a non-file is an **error**, not a fall-through: an
    ///    operator who named a binary meant that binary.
    /// 2. `../../target/release/iccce{.exe}` — what
    ///    `cargo build --release -p iccce-cli` produces at the workspace root.
    /// 3. `../../target/debug/iccce{.exe}`.
    ///
    /// **The release build is preferred and that is deliberate**, so the
    /// numbers recorded describe the artefact users run. If only a debug build
    /// is present the caller should say so in its report — `f64` arithmetic is
    /// not supposed to differ between profiles, but "not supposed to" is the
    /// phrase this role exists to distrust.
    ///
    /// `Ok(None)` means no binary was found, which is a **skip**, not an
    /// error: a fresh checkout has not been built yet.
    pub fn locate() -> Result<Option<Iccce>, DiffError> {
        if let Some(v) = std::env::var_os("ICCCE_BIN") {
            let p = PathBuf::from(v);
            if p.is_file() {
                return Ok(Some(Iccce { exe: p }));
            }
            return Err(DiffError::OracleNamedButMissing(p));
        }
        let root = Path::new(env!("CARGO_MANIFEST_DIR"));
        for rel in [
            "../../target/release/iccce.exe",
            "../../target/release/iccce",
            "../../target/debug/iccce.exe",
            "../../target/debug/iccce",
        ] {
            let c = root.join(rel);
            if c.is_file() {
                return Ok(Some(Iccce { exe: c }));
            }
        }
        Ok(None)
    }

    /// The binary that will be invoked. Print it in any report, for the same
    /// reason [`Oracle::path`] is printed: "iccce agrees with lcms2" is not
    /// falsifiable unless the reader can tell which build said so.
    pub fn path(&self) -> &Path {
        &self.exe
    }

    /// `true` when the located binary came from `target/debug`.
    pub fn is_debug_build(&self) -> bool {
        self.exe
            .components()
            .any(|c| c.as_os_str().eq_ignore_ascii_case("debug"))
    }

    /// ★ **Added 2026-08-17 for Pass H.** Run `iccce inspect <profile>` and
    /// return **everything the process said**, unparsed and ungraded.
    ///
    /// ## Why this returns a struct instead of a `Result`
    ///
    /// Every other method on this type treats a non-zero exit as
    /// [`DiffError::NonZeroExit`], because for a *transform* a refusal means
    /// no answer came back. Pass H inverts that: **the refusal IS the answer**,
    /// and the thing being graded is its exit code, its message and the absence
    /// of any stdout. Folding a refusal into an error type would force every
    /// caller to unpick it out of a `Display` string, and would make "iccce
    /// refused, saying why" indistinguishable from "the harness could not run
    /// iccce at all". Those are different facts and this type keeps them apart:
    /// a genuine spawn failure is still `Err`.
    ///
    /// ## What the caller must NOT do with the result
    ///
    /// **Do not pipe.** `TOLERANCES.md` §5.6 exists because two false
    /// "all green" claims have shipped from reading an exit code through
    /// `grep`/`tail`, which report *their* status, not the child's.
    /// [`InspectRun::code`] is the child's own status and is the only thing a
    /// gate may use.
    pub fn inspect(&self, profile: &Path) -> Result<InspectRun, DiffError> {
        let args = vec!["inspect".to_string(), profile.display().to_string()];
        let out = Command::new(&self.exe)
            .args(&args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| DiffError::Spawn(self.exe.clone(), e))?;
        Ok(InspectRun {
            code: out.status.code(),
            stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        })
    }

    /// ★ **Added 2026-08-17 for Pass H.** Run `iccce bench` and return the raw
    /// exit status, for a row whose subject is **whether the process survives
    /// at all**.
    ///
    /// `Ok(None)` for `code` means the process was terminated without a normal
    /// exit status. On Windows an allocator abort surfaces as the *negative*
    /// status `0xC000_0409` (`STATUS_STACK_BUFFER_OVERRUN`, which is what
    /// Rust's `__fastfail` raises), and that is a value a caller must be able
    /// to see rather than have flattened into "non-zero".
    pub fn bench_status(&self, args: &[String]) -> Result<InspectRun, DiffError> {
        let mut full = vec!["bench".to_string()];
        full.extend_from_slice(args);
        let out = Command::new(&self.exe)
            .args(&full)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| DiffError::Spawn(self.exe.clone(), e))?;
        Ok(InspectRun {
            code: out.status.code(),
            stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        })
    }

    /// Run `iccce transform --src <src> --dst <dst>` over a grid of RGB
    /// triples **in 0..1**, returning one output triple per input triple.
    ///
    /// The intent is not a parameter because the shipped binary implements
    /// exactly one (media-relative colorimetric) and **refuses any other by
    /// name** rather than substituting. When Pass 4 adds intents this grows an
    /// argument; until then, passing one would document a capability that does
    /// not exist.
    ///
    /// Stdin is written on a background thread for the deadlock reason given
    /// on [`Oracle::convert_batch`].
    pub fn transform_grid(
        &self,
        src: &Path,
        dst: &Path,
        grid: &[[f64; 3]],
    ) -> Result<Vec<[f64; 3]>, DiffError> {
        let args = vec![
            "transform".to_string(),
            "--src".to_string(),
            src.display().to_string(),
            "--dst".to_string(),
            dst.display().to_string(),
        ];

        let mut child = Command::new(&self.exe)
            .args(&args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| DiffError::Spawn(self.exe.clone(), e))?;

        let mut buf = String::with_capacity(grid.len() * 32);
        for t in grid {
            // Full round-trip precision on the way in. `iccce transform`
            // parses with str::parse::<f64>, so this is lossless — the input
            // side of the comparison must not be where precision is lost.
            buf.push_str(&format!("{} {} {}\n", t[0], t[1], t[2]));
        }
        let mut stdin = child
            .stdin
            .take()
            .ok_or_else(|| DiffError::Internal("child stdin was not piped".into()))?;
        let writer = std::thread::spawn(move || -> io::Result<()> {
            stdin.write_all(buf.as_bytes())?;
            stdin.flush()
        });

        let out = child
            .wait_with_output()
            .map_err(|e| DiffError::Spawn(self.exe.clone(), e))?;
        match writer.join() {
            Ok(Ok(())) => {}
            Ok(Err(e)) => return Err(DiffError::Pipe(e)),
            Err(_) => return Err(DiffError::Internal("stdin writer thread panicked".into())),
        }

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

        let rows = parse_rows(&stdout, 3).ok_or_else(|| DiffError::Unparsable {
            args: args.clone(),
            stdout: stdout.clone(),
            stderr: stderr.clone(),
        })?;
        if rows.len() != grid.len() {
            return Err(DiffError::Arity {
                expected: grid.len(),
                got: rows.len(),
                stdout,
            });
        }
        Ok(rows.into_iter().map(|r| [r[0], r[1], r[2]]).collect())
    }

    /// ★ **Added 2026-08-11 for Pass 4.** Run `iccce transform` over a grid of
    /// **n-channel** source device values at a **named intent**, returning one
    /// output triple per input row.
    ///
    /// [`Iccce::transform_grid`] above is the Pass 3 shape: three components
    /// in, no intent, because the binary at commit `051707f` accepted neither.
    /// Commit **`490191b`** ("cli: transform upgraded to the Chain — N-channel
    /// input, four intents") changed both, so a CMYK source can now be pushed
    /// through the **shipped surface** instead of through a library call. That
    /// matters: it restores the symmetry this type's doc comment demands —
    /// both sides of every Pass 4 device-space comparison now cross a process
    /// boundary, are printed as text, and are parsed back.
    ///
    /// `rows` are source device values in **0..1**, `channels` wide (4 for
    /// CMYK). The output is assumed to be three components; a destination with
    /// a different channel count is an [`DiffError::Arity`], not a silent
    /// reshape.
    ///
    /// The intent is passed by **name** (`media-relative`, `perceptual`,
    /// `saturation`, `absolute`) because that is what the CLI accepts and
    /// because it refuses an unknown name rather than substituting one.
    ///
    /// Stdin is written on a background thread for the deadlock reason given
    /// on [`Oracle::convert_batch`].
    pub fn transform_rows(
        &self,
        src: &Path,
        dst: &Path,
        intent: Intent,
        rows: &[Vec<f64>],
    ) -> Result<Vec<[f64; 3]>, DiffError> {
        let out = self.transform_rows_shaped(src, dst, intent, rows, 3)?;
        Ok(out.into_iter().map(|r| [r[0], r[1], r[2]]).collect())
    }

    /// ★ **Added 2026-08-11 for Pass 4b.** As [`Iccce::transform_rows`], but
    /// for a destination whose channel count is **not three**.
    ///
    /// Pass 4 only ever converted *into* an RGB destination, so the output
    /// width could be hard-coded. Pass 4b's B2A direction converts **into
    /// CMYK** (3 in, 4 out) and its gray direction converts **out of** a
    /// 1-channel source, so both widths have to be parameters. The arity guard
    /// stays and is applied to the stated width: a short or long row is a real
    /// disagreement about the shape of the answer and must not be silently
    /// reshaped — that is how a 4-channel result gets quietly read as a
    /// 3-channel one and compared against the wrong oracle column.
    ///
    /// `rows` are source device values in **0..1**, any width; the return is
    /// one row of `out_channels` values in **0..1** per input row.
    pub fn transform_rows_shaped(
        &self,
        src: &Path,
        dst: &Path,
        intent: Intent,
        rows: &[Vec<f64>],
        out_channels: usize,
    ) -> Result<Vec<Vec<f64>>, DiffError> {
        self.transform_rows_shaped_bpc(src, dst, intent, rows, out_channels, false)
    }

    /// ★ **Added 2026-08-11 for Pass 5.** As [`Iccce::transform_rows_shaped`],
    /// but with the shipped binary's **`--bpc`** flag optionally set.
    ///
    /// ## Why the flag is a parameter and not a second entry point
    ///
    /// Pass 5's whole subject is the *difference* between two invocations that
    /// differ in exactly one flag. Expressing that as one function with a
    /// `bool` makes it impossible for the two arms to drift apart in some other
    /// respect — a second copy of this function would be free to pass a
    /// different intent, a different profile order, or a different stdin
    /// encoding, and the resulting difference would be attributed to BPC.
    ///
    /// ## What a refusal looks like, and why it is returned rather than hidden
    ///
    /// `iccce transform --bpc` **exits 1 with a named reason** when the chain
    /// is outside its black-point estimation subset (`ChainError::
    /// BpcNotApplicable` at ICC-absolute, `ChainError::BpcEstimationUnsupported`
    /// for e.g. a v2 LUT side). That surfaces here as
    /// [`DiffError::NonZeroExit`] carrying the child's `stderr`, and Pass 5
    /// **grades one of those refusals as a deliverable** rather than treating it
    /// as harness breakage: an engine that refuses by name where it cannot
    /// estimate is behaving as `CLAUDE.md` rule 6 requires, and the boundary of
    /// the subset is part of the coverage statement.
    pub fn transform_rows_shaped_bpc(
        &self,
        src: &Path,
        dst: &Path,
        intent: Intent,
        rows: &[Vec<f64>],
        out_channels: usize,
        bpc: bool,
    ) -> Result<Vec<Vec<f64>>, DiffError> {
        self.transform_rows_shaped_opts(
            src,
            dst,
            intent,
            rows,
            out_channels,
            TransformOpts {
                bpc,
                preserve_black: None,
            },
        )
    }

    /// ★ **Added 2026-08-18 for Pass K's post-feature repointing.** As
    /// [`Iccce::transform_rows_shaped_bpc`], but with the shipped binary's
    /// **`--preserve-black <policy>`** flag optionally set.
    ///
    /// ## Why the policy is a `&str` and not an enum
    ///
    /// The CLI's whole design point is that **the policy name is mandatory and
    /// has no default**, because two published definitions of "preserve the
    /// black" disagree by up to `4.9e-2` on a cross-press pair. Mirroring the
    /// name as a *string the harness types* keeps the harness able to drive a
    /// policy name the binary does not know — which is a behaviour Pass K
    /// grades (an unknown policy must be refused, not substituted). An enum
    /// here would make the harness structurally incapable of asking the
    /// question.
    ///
    /// ## Why this is a parameter and not a second entry point
    ///
    /// The same argument [`Iccce::transform_rows_shaped_bpc`] makes for
    /// `--bpc`, and it matters more here: Pass K's post-feature regression
    /// guards compare **the same probe set through the same function with the
    /// flag on and off**, and a bit-identical result is the observation. Two
    /// copies of this function would be free to drift in the intent, the
    /// profile order or the stdin encoding, and the difference would be
    /// attributed to the preservation path.
    ///
    /// ## What a refusal looks like
    ///
    /// `iccce transform --preserve-black` **exits 1 with a named reason** for
    /// each of its five refusals (absolute intent; either side not 4-channel;
    /// no destination `A2B`; a non-monotonic K ramp; an unimplemented
    /// mapping). That surfaces here as [`DiffError::NonZeroExit`] carrying the
    /// child's `stderr`, exactly as the BPC refusals do, and a caller that
    /// wants to grade the refusal reads the message off it.
    ///
    /// # Errors
    /// As [`Iccce::transform_rows_shaped_bpc`].
    pub fn transform_rows_shaped_preserve_black(
        &self,
        src: &Path,
        dst: &Path,
        intent: Intent,
        rows: &[Vec<f64>],
        out_channels: usize,
        policy: &str,
    ) -> Result<Vec<Vec<f64>>, DiffError> {
        self.transform_rows_shaped_opts(
            src,
            dst,
            intent,
            rows,
            out_channels,
            TransformOpts {
                bpc: false,
                preserve_black: Some(policy),
            },
        )
    }

    /// The one place `iccce transform` is actually spawned for a row grid.
    ///
    /// Every optional flag is a parameter of this single function so that two
    /// invocations differing in one flag cannot differ in anything else. See
    /// the two public wrappers above for why each flag is shaped the way it is.
    ///
    /// # Errors
    /// Spawn, pipe, non-zero exit, unparsable output, or an arity mismatch
    /// between the rows written and the rows read back.
    pub fn transform_rows_shaped_opts(
        &self,
        src: &Path,
        dst: &Path,
        intent: Intent,
        rows: &[Vec<f64>],
        out_channels: usize,
        opts: TransformOpts<'_>,
    ) -> Result<Vec<Vec<f64>>, DiffError> {
        let TransformOpts {
            bpc,
            preserve_black,
        } = opts;
        let intent_arg = match intent {
            Intent::Perceptual => "perceptual",
            Intent::RelativeColorimetric => "media-relative",
            Intent::Saturation => "saturation",
            Intent::AbsoluteColorimetric => "absolute",
        };
        let mut args = vec![
            "transform".to_string(),
            "--src".to_string(),
            src.display().to_string(),
            "--dst".to_string(),
            dst.display().to_string(),
            "--intent".to_string(),
            intent_arg.to_string(),
        ];
        if bpc {
            args.push("--bpc".to_string());
        }
        if let Some(policy) = preserve_black {
            args.push("--preserve-black".to_string());
            args.push(policy.to_string());
        }

        let mut child = Command::new(&self.exe)
            .args(&args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| DiffError::Spawn(self.exe.clone(), e))?;

        let mut buf = String::with_capacity(rows.len() * 40);
        for r in rows {
            let line: Vec<String> = r.iter().map(|v| format!("{v}")).collect();
            buf.push_str(&line.join(" "));
            buf.push('\n');
        }
        let mut stdin = child
            .stdin
            .take()
            .ok_or_else(|| DiffError::Internal("child stdin was not piped".into()))?;
        let writer = std::thread::spawn(move || -> io::Result<()> {
            stdin.write_all(buf.as_bytes())?;
            stdin.flush()
        });

        let out = child
            .wait_with_output()
            .map_err(|e| DiffError::Spawn(self.exe.clone(), e))?;
        match writer.join() {
            Ok(Ok(())) => {}
            Ok(Err(e)) => return Err(DiffError::Pipe(e)),
            Err(_) => return Err(DiffError::Internal("stdin writer thread panicked".into())),
        }

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
        let parsed = parse_rows(&stdout, out_channels).ok_or_else(|| DiffError::Unparsable {
            args: args.clone(),
            stdout: stdout.clone(),
            stderr: stderr.clone(),
        })?;
        if parsed.len() != rows.len() {
            return Err(DiffError::Arity {
                expected: rows.len(),
                got: parsed.len(),
                stdout,
            });
        }
        Ok(parsed)
    }
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
    /// ★ **Added 2026-08-11 (Pass 4b).** An expectation **derived by
    /// arithmetic** from (a) the specification's stated element order and
    /// encoding and (b) the bytes of a *synthetic* fixture, with **no
    /// implementation's output in it**.
    ///
    /// ## Why this is a separate variant and not [`Kind::GroundTruth`]
    ///
    /// It is **not a published value**. Nobody at the CIE or the ICC printed
    /// the number; a reader of this repository derived it from clause text.
    /// Calling it ground truth would overstate the chain of custody, and
    /// `TOLERANCES.md` §1's whole point is that a weak claim must not be
    /// quotable as a strong one.
    ///
    /// ## Why it is nevertheless stronger than [`Kind::CrossCheck`]
    ///
    /// A cross-check is defeated when both implementations share a misreading.
    /// A derived expectation is defeated only when **the derivation** shares
    /// the misreading — and the derivation is written down next to the number,
    /// in a form a spec reader can check against the standard without running
    /// anything. It is the only kind of expectation available for a v4 LUT
    /// path on a machine that has no v4 LUT profile.
    ///
    /// ## What it cannot do, stated as prominently
    ///
    /// The fixture and the derivation are read out of the **same corpus** by
    /// the same project. If `ICC_Spec`'s transcription of clause 10.12/10.13 is
    /// wrong, the fixture bytes and this expectation are wrong **together** and
    /// agree perfectly. That is exactly the failure mode a *third* reading —
    /// lcms2's — is retained to catch, which is why every row of this kind in
    /// Pass 4b is paired with a [`Kind::CrossCheck`] row over the same points.
    DerivedExpectation,
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
            Kind::DerivedExpectation => "derived-expectation",
            Kind::CrossCheck => "cross-check",
            Kind::SelfConsistency => "self-consistency",
            Kind::OracleReproducibility => "oracle-reproducibility",
        }
    }
}

/// How two vectors of numbers are compared.
///
/// **"ΔE" alone is not a metric** (`TOLERANCES.md` §3): every variant below
/// says which difference formula, over what, reduced how. A record that says
/// `dE2000-max` and a record that says `dE2000-mean` are answering different
/// questions and must never be quoted for each other — a mean over a grid
/// hides exactly the outlier a colour engine gets wrong.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Metric {
    /// Maximum absolute difference over components, in the output space's own
    /// units. For Lab that is `L*`/`a*`/`b*` units, **not** ΔE: the largest
    /// single-component error, which is the conservative reading.
    AbsMaxComponent,
    /// Maximum absolute difference over every component of every row of a
    /// grid, expressed in **normalised device units (0..1)**. The unit matters:
    /// `transicc` prints device values in 0..255, `iccce transform` in 0..1,
    /// and a number quoted without its scale is wrong by 255.
    DeviceAbsMaxNormalised,
    /// Mean absolute difference over every component of every row, normalised
    /// device units. Reported **alongside** the max, never instead of it.
    DeviceAbsMeanNormalised,
    /// Maximum CIEDE2000 over the rows of a grid, `kL=kC=kH=1`, computed in
    /// D50 CIELAB. The formula is `iccce_color::delta_e_2000` — validated
    /// against all 34 Sharma, Wu & Dalal (2005) pairs (`TOLERANCES.md`
    /// §3.1.1). Using a validated ruler does **not** make the record ground
    /// truth; see [`Kind`].
    DeltaE2000Max,
    /// Mean CIEDE2000 over the rows of a grid, `kL=kC=kH=1`, D50 CIELAB.
    DeltaE2000Mean,
    /// ★ **Added 2026-08-17 for Pass H.** A **count** — of files, of violated
    /// conditions, of points at which a predicate did not hold.
    ///
    /// It exists because Pass H's subject is *acceptance and refusal*, which
    /// has no colour units at all, and because emitting a count under
    /// [`Metric::AbsMaxComponent`] would put "abs-max-component" in the TSV
    /// beside a number that is not a difference of components in any space.
    /// A metric column that lies is worse than one that is coarse: a reader
    /// comparing two rows must be able to tell that they answer different
    /// questions, which is the whole reason this enum documents each variant.
    ///
    /// **A count row's tolerance is essentially always zero** — there is no
    /// instrument error in a count — so a non-zero bound on one of these
    /// deserves a very good `why`.
    IndicatorCount,
}

impl Metric {
    pub fn tag(self) -> &'static str {
        match self {
            Metric::AbsMaxComponent => "abs-max-component",
            Metric::DeviceAbsMaxNormalised => "device-abs-max-normalised(0..1)",
            Metric::DeviceAbsMeanNormalised => "device-abs-mean-normalised(0..1)",
            Metric::DeltaE2000Max => "dE2000-max(kL=kC=kH=1,D50)",
            Metric::DeltaE2000Mean => "dE2000-mean(kL=kC=kH=1,D50)",
            Metric::IndicatorCount => "indicator-count",
        }
    }

    /// Only the pairwise variant can be measured from two flat vectors. The
    /// grid variants are computed by [`crate::pass3`] from whole grids and
    /// arrive via [`Record`] already reduced; asking for one here is a
    /// programming error, not a runtime condition, so it panics rather than
    /// returning a plausible number.
    fn measure(self, got: &[f64], expected: &[f64]) -> f64 {
        match self {
            Metric::AbsMaxComponent => got
                .iter()
                .zip(expected)
                .map(|(g, e)| (g - e).abs())
                .fold(0.0_f64, f64::max),
            other => panic!(
                "{} is a grid metric: build a Record with the value already reduced, \
                 do not call Metric::measure",
                other.tag()
            ),
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

// ===========================================================================
// Candidate separation — how far away the wrong answer would have been
// ===========================================================================

/// Whether a stated separation may be compared against the row's tolerance.
///
/// This exists because the blindness test in [`Record::separation_power`] is a
/// comparison of two numbers, and two numbers may only be compared when they
/// are in the same units. Pass 6 row R5 and Pass 5c's `ATTRIBUTION` row both
/// record the same lesson from the other direction: **the unit the requirement
/// is stated in is the one that may carry the tolerance**, and a ratio computed
/// across incommensurable units is arithmetic, not evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SepUnits {
    /// The separation is expressed in exactly the units [`Record::metric`]
    /// reports, so separation and tolerance are directly comparable and the
    /// blindness test means what it says.
    SameAsMetric,
    /// Different units, **named**. The separation is still emitted — it is
    /// information — but no verdict is drawn from comparing it to the
    /// tolerance, and the record says so rather than quietly skipping the test.
    Other(&'static str),
}

impl SepUnits {
    pub fn tag(self) -> &'static str {
        match self {
            SepUnits::SameAsMetric => "same-as-metric",
            SepUnits::Other(u) => u,
        }
    }
}

/// **How far apart this row's two candidate answers are** — the quantity that
/// bounds what a cross-check can prove. See the module header for the incident
/// that made it a first-class field.
///
/// ## What "the alternative" means, precisely
///
/// It is *not* a rival tolerance and *not* an error bar. It is: **the value
/// this row's own `observed` quantity would have taken had a named,
/// plausible-at-the-time alternative reading been the one implemented.**
/// Naming it is the whole discipline — "some other answer might have been
/// closer" is not a separation. The alternatives that qualify are the ones a
/// reader can go and check:
///
/// - an **intermediate** the code already computes (`MinL`, `outRamp[first]`,
///   `InitialLab`) that could have been returned instead;
/// - a **branch** the dispatching implementation might have taken
///   (`BlackPointAsDarkerColorant` instead of `BlackPointUsingPerceptualBlack`);
/// - an **anchor** a neighbouring clause supplies for the same name;
/// - **the other document's reading** of one word — which is what the whole of
///   Pass 5c turned out to be about.
///
/// ## Why `Unstated` is a variant and not an `Option`
///
/// `None` reads as "nothing to say here", and that is the failure this type
/// exists to prevent. A row where somebody looked and found no rival is
/// [`Separation::NoNamedAlternative`] **with the reason attached**; a row
/// nobody has thought about is [`Separation::Unstated`] and prints as
/// `UNSTATED`. The two are different claims about the evidence and the report
/// keeps them apart.
#[derive(Debug, Clone)]
pub enum Separation {
    /// Nobody has considered a rival candidate for this row yet. Prints as
    /// `UNSTATED` — deliberately conspicuous, deliberately not a blank.
    Unstated,
    /// Considered, and there is no named rival. The `String` is the reason,
    /// and it is required: "no alternative" without a reason is
    /// indistinguishable from not having looked.
    NoNamedAlternative(String),
    /// A named rival exists, and here is what this row would have observed
    /// under it.
    Measured {
        /// What the alternative *is*, in words a reader can verify against a
        /// clause or a source file. Include the file and line if it is code.
        alternative: String,
        /// The value `observed` would have taken under that alternative.
        alt_observed: f64,
        /// The distance between the two candidate observations. Usually
        /// `|observed − alt_observed|`, but supplied rather than derived,
        /// because for a vector-valued candidate (two Lab points, say) the
        /// distance is a ΔE and not the difference of two scalars.
        distance: f64,
        units: SepUnits,
    },
}

impl Separation {
    /// The common case: two scalar candidate observations, distance is their
    /// difference.
    ///
    /// ## ★★ When this is the WRONG constructor, measured 2026-08-12
    ///
    /// **If the alternative is "the code under test returns the other
    /// candidate", do not use this — use [`Separation::against_distance`].**
    ///
    /// The distance here is derived as `|observed − alt_observed|`, which is
    /// correct while the code is right and **collapses to exactly zero the
    /// moment the code is wrong in the way the row is watching for**: `observed`
    /// *becomes* `alt_observed`. The row then fails, correctly, and simultaneously
    /// reports `ZERO-SEPARATION` — the mechanism disclaiming its power on the one
    /// run where it has just demonstrated it.
    ///
    /// This is not hypothetical. The proof-of-power run for
    /// `pass5c/floored/CLAUSE/4.2.5.4-returns-InitialLab-not-outRamp-first`
    /// injected the pre-`fd34a44` defect: the row failed at `2,500 019×10¹`
    /// against a `7,63×10⁻⁴` bound and printed `ZERO-SEPARATION` beside it.
    ///
    /// The test: **ask whether the distance is a property of the run or of the
    /// fixture.** A separation between two *candidate answers* is a property of
    /// the fixture and must be supplied, not derived — it is 25 `L*` whichever
    /// answer the library happens to return today. `against` is right only where
    /// the alternative is a different *reading applied to the same observation*,
    /// so that the two observations genuinely coexist.
    pub fn against(
        alternative: impl Into<String>,
        alt_observed: f64,
        observed: f64,
        units: SepUnits,
    ) -> Separation {
        Separation::Measured {
            alternative: alternative.into(),
            alt_observed,
            distance: (observed - alt_observed).abs(),
            units,
        }
    }

    /// As [`Separation::against`], but for a candidate whose distance is not
    /// the difference of the two scalars — a ΔE between two Lab points, for
    /// instance. The caller states the distance and is responsible for it
    /// being in the units it claims.
    pub fn against_distance(
        alternative: impl Into<String>,
        alt_observed: f64,
        distance: f64,
        units: SepUnits,
    ) -> Separation {
        Separation::Measured {
            alternative: alternative.into(),
            alt_observed,
            distance,
            units,
        }
    }

    pub fn none(reason: impl Into<String>) -> Separation {
        Separation::NoNamedAlternative(reason.into())
    }

    /// The distance, when there is one. Emitted in its own TSV column.
    pub fn distance(&self) -> Option<f64> {
        match self {
            Separation::Measured { distance, .. } => Some(*distance),
            _ => None,
        }
    }
}

/// What a row's separation says about its power to discriminate. Derived, never
/// stored: it is a function of the separation **and the tolerance**, and
/// deriving it means the two cannot drift apart.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SepPower {
    /// No separation has been stated for this row. Not a verdict — an absence,
    /// and one a reader should be able to count.
    Unstated,
    /// Stated, and there is no named rival candidate.
    NoAlternative,
    /// A separation exists but is in different units from the metric, so the
    /// blindness test is **not applied**. The number is still emitted.
    Incommensurate,
    /// The tolerance is infinite: the row is reported, not graded, so it has no
    /// discriminating power regardless of how far apart the candidates are.
    /// **This is not the fixture's fault and is not `Blind`.**
    Ungraded,
    /// ★★ **The candidates are the same number.** The row cannot move whatever
    /// the code does, at any tolerance, graded or not — so this outranks every
    /// verdict below and is checked before them.
    ///
    /// It is a distinct state from [`SepPower::Blind`] because the remedy is
    /// different: a blind row might be rescued by tightening its tolerance, and
    /// a zero-separation row can only be rescued by **a different fixture**.
    /// `pass5c/synthetic/*` is the worked example — that fixture's
    /// `InitialLab` and `outRamp[first]` are both `L* 20`, so its 5.000000 was
    /// unmoved by the correction that shifted the `swop` arm 58.8×
    /// (`NUMERIC_CLAIMS.md` NC-166, `ARCHITECTURE.md` DL-036).
    ZeroSeparation,
    /// ★ **The alarm.** The candidates are closer together than the tolerance,
    /// so this row would pass under either — it cannot see the difference it
    /// looks like it is testing.
    Blind,
    /// The candidates are further apart than the tolerance: a wrong candidate
    /// would have failed this row.
    Discriminating,
    /// The separation did not compute (NaN). Apparatus breakage, surfaced
    /// rather than silently classified as one of the above.
    Broken,
}

impl SepPower {
    pub fn tag(self) -> &'static str {
        match self {
            SepPower::Unstated => "UNSTATED",
            SepPower::NoAlternative => "NO-NAMED-ALTERNATIVE",
            SepPower::Incommensurate => "INCOMMENSURATE",
            SepPower::Ungraded => "UNGRADED",
            SepPower::ZeroSeparation => "ZERO-SEPARATION",
            SepPower::Blind => "BLIND",
            SepPower::Discriminating => "DISCRIMINATING",
            SepPower::Broken => "SEP-BROKEN",
        }
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
    Pass {
        observed: f64,
        got: Vec<f64>,
    },
    Fail {
        observed: f64,
        got: Vec<f64>,
    },
    /// Could not run. Carries the reason, which is always printed: a skip
    /// whose reason is not recorded is indistinguishable from a pass in a
    /// summary line, and that is how coverage silently goes to zero.
    Skip {
        reason: String,
    },
    /// The harness or the oracle failed.
    Error {
        detail: String,
    },
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

/// One finished, gradeable line of a report — **the unit `Report` stores.**
///
/// [`Check`] describes a comparison the harness knows how to *run* (one
/// `transicc` invocation, one expected vector). Not every comparison has that
/// shape: a Pass 3 grid comparison runs two different binaries over 133
/// triples and reduces the result to one number, and contorting `Check` to
/// express that would have made the simple case unreadable. `Record` is the
/// common denominator — id, kind, metric, tolerance, provenance, outcome —
/// and `Check` produces one via [`Record::from_check`].
///
/// The fields nobody may omit are the same three as ever: **kind** (how strong
/// is this claim), **tolerance-with-a-why** (`CLAUDE.md` rule 5), and
/// **source** (where the expectation came from). A `Record` cannot be built
/// without all three, which is the whole reason it is a struct and not a
/// tuple.
#[derive(Debug, Clone)]
pub struct Record {
    pub id: String,
    pub kind: Kind,
    pub metric: Metric,
    pub tolerance: Tolerance,
    /// **Where the expectation came from** — a citation for ground truth, a
    /// document section and date for a recorded oracle value, or a plain
    /// statement that both sides were computed in this run.
    pub source: String,
    /// Free text: what was compared, over what, with what settings. Tabs and
    /// newlines are stripped on emit.
    pub detail: String,
    /// ★ **How far away the wrong answer would have been** — see
    /// [`Separation`] and the module header. Defaults to
    /// [`Separation::Unstated`] on every constructor, which prints as
    /// `UNSTATED`: a row nobody has thought about must look different from a
    /// row where somebody looked and found nothing.
    pub separation: Separation,
    pub outcome: Outcome,
}

impl Record {
    /// Grade an already-reduced observation against a tolerance.
    ///
    /// Used by grid comparisons, which do their own reduction. The
    /// pass/fail decision stays here so it is made in exactly one place:
    /// `observed <= tolerance.value`, `<=` and not `<`, so a tolerance stated
    /// as "the printed precision" admits a difference of exactly that.
    pub fn graded(
        id: impl Into<String>,
        kind: Kind,
        metric: Metric,
        tolerance: Tolerance,
        observed: f64,
        source: impl Into<String>,
        detail: impl Into<String>,
    ) -> Record {
        let outcome = if observed.is_finite() && observed <= tolerance.value {
            Outcome::Pass {
                observed,
                got: Vec::new(),
            }
        } else {
            Outcome::Fail {
                observed,
                got: Vec::new(),
            }
        };
        Record {
            id: id.into(),
            kind,
            metric,
            tolerance,
            source: source.into(),
            detail: detail.into(),
            separation: Separation::Unstated,
            outcome,
        }
    }

    /// ★ Attach a [`Separation`] — the builder half of the mechanism described
    /// in the module header.
    ///
    /// It is a builder rather than a seventh positional argument on
    /// [`Record::graded`] for one reason: **most rows do not have a named
    /// alternative, and a required argument would have produced a corpus of
    /// invented ones.** The engineer's brief on 2026-08-12 was explicit about
    /// preferring a few real separations with the rest honestly marked over a
    /// scheme that manufactures one everywhere, and the default of
    /// [`Separation::Unstated`] is that preference expressed in the type.
    #[must_use]
    pub fn with_separation(mut self, separation: Separation) -> Record {
        self.separation = separation;
        self
    }

    /// ★ **Added 2026-08-17 for Pass H.** Restate the [`Metric`] a record was
    /// built with.
    ///
    /// [`Record::graded`] takes the metric positionally, which is right for a
    /// section where every row measures the same thing. Pass H mixes counts,
    /// device units and `L*` units inside one section, and a helper that fixed
    /// one metric for all of them would have put a wrong label in the TSV — the
    /// failure [`Metric`]'s own doc comment warns about. A builder keeps the
    /// helper and keeps the label honest.
    #[must_use]
    pub fn with_metric(mut self, metric: Metric) -> Record {
        self.metric = metric;
        self
    }

    /// The verdict [`Separation`] supports for this row, derived from the
    /// separation **and this row's own tolerance**. See [`SepPower`].
    ///
    /// The order of the guards is the argument:
    ///
    /// 1. no separation stated → nothing to say;
    /// 2. stated as absent → nothing to say, but somebody looked;
    /// 3. NaN → apparatus breakage, said out loud rather than classified;
    /// 4. **zero distance → `ZeroSeparation`**, checked before everything
    ///    below it because a row whose candidates are the same number has no
    ///    power at any tolerance and in any units;
    /// 5. incommensurable units → the number is emitted, the test is not run;
    /// 6. **infinite tolerance → `Ungraded`**, checked *before* the comparison,
    ///    because `d <= inf` is true for every finite `d` and would label every
    ///    reported row `BLIND` — blaming the fixture for a decision the
    ///    tolerance made;
    /// 7. otherwise the comparison, `<=` matching [`Record::graded`]'s own
    ///    pass rule so that "the separation is exactly the tolerance" is read
    ///    the same way in both places: as *not* discriminating.
    #[must_use]
    pub fn separation_power(&self) -> SepPower {
        match &self.separation {
            Separation::Unstated => SepPower::Unstated,
            Separation::NoNamedAlternative(_) => SepPower::NoAlternative,
            Separation::Measured {
                distance, units, ..
            } => {
                if distance.is_nan() {
                    SepPower::Broken
                } else if *distance == 0.0 {
                    SepPower::ZeroSeparation
                } else if !matches!(units, SepUnits::SameAsMetric) {
                    SepPower::Incommensurate
                } else if !self.tolerance.value.is_finite() {
                    SepPower::Ungraded
                } else if *distance <= self.tolerance.value {
                    SepPower::Blind
                } else {
                    SepPower::Discriminating
                }
            }
        }
    }

    /// The separation, rendered for the `detail` column: the alternative's
    /// name, its value, the distance, and — where the comparison is legitimate
    /// — how many multiples of the tolerance that distance is.
    ///
    /// The ratio is **formatted from what the record already holds** rather
    /// than typed anywhere, which is the rule §3.5.8.6 established after three
    /// claim-bearing literals in this crate went false inside a day.
    fn separation_detail(&self) -> String {
        match &self.separation {
            Separation::Unstated => "candidate separation: UNSTATED — no alternative candidate \
                                     has been considered for this row"
                .to_string(),
            Separation::NoNamedAlternative(why) => {
                format!("candidate separation: NO NAMED ALTERNATIVE — {why}")
            }
            Separation::Measured {
                alternative,
                alt_observed,
                distance,
                units,
            } => {
                let power = self.separation_power();
                let ratio = if power == SepPower::Discriminating || power == SepPower::Blind {
                    format!(
                        "; distance is {:.4}x this row's tolerance",
                        distance / self.tolerance.value
                    )
                } else {
                    String::new()
                };
                format!(
                    "candidate separation: {} — this row would have observed {:.6e} under that \
                     alternative, {:.6e} away in {}{} -> {}",
                    alternative,
                    alt_observed,
                    distance,
                    units.tag(),
                    ratio,
                    power.tag()
                )
            }
        }
    }

    /// A record that could not run. Kept distinct from a failure — a skip
    /// whose reason is not recorded is indistinguishable from a pass in a
    /// summary line, which is how coverage silently goes to zero.
    pub fn skipped(
        id: impl Into<String>,
        kind: Kind,
        metric: Metric,
        tolerance: Tolerance,
        source: impl Into<String>,
        reason: impl Into<String>,
    ) -> Record {
        let reason = reason.into();
        Record {
            id: id.into(),
            kind,
            metric,
            tolerance,
            source: source.into(),
            detail: reason.clone(),
            separation: Separation::Unstated,
            outcome: Outcome::Skip { reason },
        }
    }

    /// A record whose comparison broke — harness or oracle, not colour.
    pub fn errored(
        id: impl Into<String>,
        kind: Kind,
        metric: Metric,
        tolerance: Tolerance,
        source: impl Into<String>,
        detail: impl Into<String>,
    ) -> Record {
        let detail = detail.into();
        Record {
            id: id.into(),
            kind,
            metric,
            tolerance,
            source: source.into(),
            detail: detail.clone(),
            separation: Separation::Unstated,
            outcome: Outcome::Error { detail },
        }
    }

    fn from_check(check: &Check, outcome: Outcome) -> Record {
        let detail = match &outcome {
            Outcome::Pass { got, .. } | Outcome::Fail { got, .. } => format!(
                "{} | got={:?} expected={:?}",
                check.request.describe(),
                got,
                check.expected
            ),
            Outcome::Skip { reason } => reason.clone(),
            Outcome::Error { detail } => detail.clone(),
        };
        Record {
            id: check.id.to_string(),
            kind: check.kind,
            metric: check.metric,
            tolerance: check.tolerance,
            source: check.source.to_string(),
            detail,
            separation: Separation::Unstated,
            outcome,
        }
    }
}

/// A run of checks, and its machine-readable rendering.
#[derive(Debug, Default)]
pub struct Report {
    rows: Vec<Record>,
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
        self.rows.push(Record::from_check(&check, outcome));
    }

    pub fn push_record(&mut self, record: Record) {
        self.rows.push(record);
    }

    pub fn counts(&self) -> (usize, usize, usize, usize) {
        let mut c = (0, 0, 0, 0);
        for r in &self.rows {
            match &r.outcome {
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

    /// ★ Count the rows by [`SepPower`], and **name** the two kinds of row that
    /// cannot see: the blind ones and the zero-separation ones.
    ///
    /// The two id lists are kept apart because the remedies are different — a
    /// blind row can be rescued by a tighter tolerance, a zero-separation row
    /// only by a different fixture (see [`SepPower::ZeroSeparation`]).
    ///
    /// Returned as data rather than printed here so a caller (a report binary,
    /// a future CI gate) can act on it without re-deriving the classification
    /// and possibly deriving it differently.
    #[must_use]
    pub fn separation_counts(&self) -> (Vec<(SepPower, usize)>, Vec<&str>, Vec<&str>) {
        let order = [
            SepPower::Unstated,
            SepPower::NoAlternative,
            SepPower::Incommensurate,
            SepPower::Ungraded,
            SepPower::ZeroSeparation,
            SepPower::Blind,
            SepPower::Discriminating,
            SepPower::Broken,
        ];
        let mut counts: Vec<(SepPower, usize)> = order.iter().map(|p| (*p, 0)).collect();
        let mut blind = Vec::new();
        let mut zero = Vec::new();
        for r in &self.rows {
            let p = r.separation_power();
            if let Some(slot) = counts.iter_mut().find(|(k, _)| *k == p) {
                slot.1 += 1;
            }
            match p {
                SepPower::Blind => blind.push(r.id.as_str()),
                SepPower::ZeroSeparation => zero.push(r.id.as_str()),
                _ => {}
            }
        }
        (counts, blind, zero)
    }

    /// Write the TSV records described in the module header.
    pub fn emit<W: Write>(&self, w: &mut W) -> io::Result<()> {
        for n in &self.notes {
            writeln!(w, "note\t{}", sanitise(n))?;
        }
        for r in &self.rows {
            let observed = match &r.outcome {
                Outcome::Pass { observed, .. } | Outcome::Fail { observed, .. } => {
                    format!("{observed:.6e}")
                }
                Outcome::Skip { .. } | Outcome::Error { .. } => "-".to_string(),
            };
            let separation = match r.separation.distance() {
                Some(d) => format!("{d:.6e}"),
                None => "-".to_string(),
            };
            // The `why`, the `source` and now the candidate separation travel
            // on EVERY line, including skips and errors. A tolerance quoted
            // without its justification is the thing this whole role exists to
            // prevent, and a reader grepping one line out of a log must not
            // have to go and find it. The separation is in the prose as well as
            // in its own column for the same reason: the column is for a
            // machine, the sentence is for whoever is reading one line.
            let detail = format!(
                "{} | tolerance because: {} | expectation source: {} | {}",
                r.detail,
                r.tolerance.why,
                r.source,
                r.separation_detail()
            );
            writeln!(
                w,
                "check\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
                r.id,
                r.outcome.tag(),
                r.kind.tag(),
                r.metric.tag(),
                r.tolerance.value,
                observed,
                separation,
                r.separation_power().tag(),
                sanitise(&detail)
            )?;
        }
        let (pass, fail, skip, error) = self.counts();
        // Unchanged in shape since Pass 3 and deliberately so — this line is
        // quoted as a run's signature.
        writeln!(
            w,
            "summary\tpass={pass}\tfail={fail}\tskip={skip}\terror={error}"
        )?;
        let (counts, blind, zero) = self.separation_counts();
        let fields: Vec<String> = counts
            .iter()
            .map(|(p, n)| format!("{}={}", p.tag().to_ascii_lowercase(), n))
            .collect();
        let list = |v: &[&str]| {
            if v.is_empty() {
                "none".to_string()
            } else {
                v.join(",")
            }
        };
        writeln!(
            w,
            "separation\t{}\tblind-ids={}\tzero-separation-ids={}",
            fields.join("\t"),
            list(&blind),
            list(&zero)
        )?;
        Ok(())
    }
}

/// Strip tabs and newlines so a free-text field cannot break the TSV framing.
pub fn sanitise(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c == '\t' || c == '\n' || c == '\r' {
                ' '
            } else {
                c
            }
        })
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

    // -----------------------------------------------------------------
    // Candidate separation. These test the CLASSIFIER, not any colour: the
    // whole value of the mechanism is that the blindness verdict is derived
    // the same way every time, so the derivation is what is pinned.
    // -----------------------------------------------------------------

    fn row(tolerance: f64, sep: Separation) -> Record {
        Record::graded(
            "t/row",
            Kind::CrossCheck,
            Metric::AbsMaxComponent,
            Tolerance::new(tolerance, "test fixture"),
            0.0,
            "test fixture",
            "test fixture",
        )
        .with_separation(sep)
    }

    /// ★ The 2026-08-12 incident, reduced to its arithmetic. The observed
    /// divergence was 0.0817 and the rival candidate — the conformant
    /// `InitialLab` — sat 4.717 away; had the row been graded at 1.0 ΔE the
    /// candidates would still have been 4.7x apart and a wrong answer would
    /// have failed it. The same row graded at 5.0 could not have told them
    /// apart, and that is the state this classifier exists to name.
    #[test]
    fn a_separation_smaller_than_the_tolerance_is_blind() {
        let sep = || {
            Separation::against(
                "outRamp[first] = MinL",
                4.799109,
                8.1668e-2,
                SepUnits::SameAsMetric,
            )
        };
        assert_eq!(row(1.0, sep()).separation_power(), SepPower::Discriminating);
        assert_eq!(row(5.0, sep()).separation_power(), SepPower::Blind);
        // Exactly equal counts as blind: `Record::graded` passes on
        // `observed <= tolerance`, so a difference of exactly the tolerance is
        // admitted there and must not be claimed as discriminated here.
        assert_eq!(
            row(
                1.0,
                Separation::against("x", 1.0, 0.0, SepUnits::SameAsMetric)
            )
            .separation_power(),
            SepPower::Blind
        );
    }

    /// An infinite tolerance is `UNGRADED`, never `BLIND`. Reported rows
    /// (`REPORTED` in every pass module) cannot fail, so no separation could
    /// have rescued them; calling that blindness would blame the fixture for
    /// a decision the tolerance made.
    #[test]
    fn an_infinite_tolerance_is_ungraded_not_blind() {
        let r = row(
            f64::INFINITY,
            Separation::against("x", 10.0, 0.0, SepUnits::SameAsMetric),
        );
        assert_eq!(r.separation_power(), SepPower::Ungraded);
    }

    /// Different units means the number is emitted and the verdict is not
    /// drawn. Pass 6 row R5's lesson, enforced by the type.
    #[test]
    fn incommensurable_units_do_not_get_a_verdict() {
        let r = row(
            1.0,
            Separation::against("x", 10.0, 0.0, SepUnits::Other("normalised device (0..1)")),
        );
        assert_eq!(r.separation_power(), SepPower::Incommensurate);
    }

    /// Silence and "there is no alternative" are different claims, and the
    /// default is the one that admits nobody has looked.
    #[test]
    fn unstated_is_distinct_from_no_alternative() {
        let d = Record::graded(
            "t/row",
            Kind::CrossCheck,
            Metric::AbsMaxComponent,
            Tolerance::new(1.0, "test fixture"),
            0.0,
            "test fixture",
            "test fixture",
        );
        assert_eq!(d.separation_power(), SepPower::Unstated);
        assert_eq!(
            row(1.0, Separation::none("both terms are measured in this run")).separation_power(),
            SepPower::NoAlternative
        );
    }

    /// ★★ Zero separation outranks everything, including the infinite
    /// tolerance of a reported row: `pass5c/synthetic`'s two candidates are
    /// both `L* 20`, and no tolerance decision can give that row power.
    #[test]
    fn zero_separation_outranks_ungraded_and_incommensurate() {
        assert_eq!(
            row(
                f64::INFINITY,
                Separation::against(
                    "outRamp[first] = L* 20 = InitialLab",
                    5.0,
                    5.0,
                    SepUnits::SameAsMetric
                )
            )
            .separation_power(),
            SepPower::ZeroSeparation
        );
        assert_eq!(
            row(
                1.0,
                Separation::against_distance("x", 5.0, 0.0, SepUnits::Other("device"))
            )
            .separation_power(),
            SepPower::ZeroSeparation
        );
    }

    /// A separation that failed to compute is apparatus breakage and says so
    /// rather than being classified as a healthy verdict.
    #[test]
    fn a_nan_separation_is_broken_not_discriminating() {
        let r = row(
            1.0,
            Separation::against("x", f64::NAN, 0.0, SepUnits::SameAsMetric),
        );
        assert_eq!(r.separation_power(), SepPower::Broken);
    }

    /// The emitted line keeps `detail` last and carries both new columns, and
    /// the `summary` line's shape is unchanged.
    #[test]
    fn emit_carries_the_separation_columns_and_leaves_summary_alone() {
        let mut rep = Report::new();
        rep.push_record(row(
            1.0,
            Separation::against("the other branch", 5.0, 0.0, SepUnits::SameAsMetric),
        ));
        let mut buf = Vec::new();
        rep.emit(&mut buf).unwrap();
        let text = String::from_utf8(buf).unwrap();
        let check = text.lines().find(|l| l.starts_with("check\t")).unwrap();
        let f: Vec<&str> = check.split('\t').collect();
        assert_eq!(f.len(), 10, "check line is 10 fields: {check}");
        assert_eq!(f[7], "5.000000e0");
        assert_eq!(f[8], "DISCRIMINATING");
        assert!(f[9].contains("candidate separation: the other branch"));
        assert!(text.contains("summary\tpass=1\tfail=0\tskip=0\terror=0"));
        assert!(text.contains("separation\tunstated=0"));
        assert!(text.contains("blind-ids=none"));
    }

    /// TSV framing survives a detail string containing tabs and newlines.
    #[test]
    fn sanitise_removes_framing_characters() {
        let s = sanitise("a\tb\nc\rd");
        assert_eq!(s, "a b c d");
    }
}
