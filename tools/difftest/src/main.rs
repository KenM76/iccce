//! # `iccce-difftest` — the runner
//!
//! Runs the registered checks against the pinned lcms2 oracle and emits the
//! TSV report described in `lib.rs`. Usage:
//!
//! ```text
//! cd tools/difftest && cargo run
//! ```
//!
//! Exit codes are the contract in `lib.rs`: `0` all-passed (and at least one
//! ran), `1` a failure, `2` a harness/oracle error, `3` **nothing ran**.
//!
//! ## What is registered here, as of 2026-08-11
//!
//! **Exactly one check, and it compares lcms2 against lcms2.** That is worth
//! saying plainly, because a runner with one green line looks like coverage:
//!
//! - `smoke/srgb-white-to-lab` — the sRGB white → Lab conversion recorded in
//!   `README.md` §8.2, re-run and compared to the recorded numbers. Its kind
//!   is [`Kind::OracleReproducibility`]: **both sides are lcms2**. It
//!   establishes that the harness drives the oracle correctly, that the pin
//!   and toolchain still produce the recorded answer, and nothing whatever
//!   about whether iccce is right — iccce is not in the loop and has no
//!   transform to put in it yet (Pass 3).
//!
//! It also **skips** on any machine without the Windows colour directory,
//! because its input is a category (c) system profile (`LEGAL.md` §3). On the
//! Linux CI runner this runner therefore exits **3, not 0**. That is the
//! intended behaviour: a suite that skipped everything has not passed.
//!
//! ## Why the expectation is allowed to come from lcms2 here
//!
//! `CLAUDE.md` rule 3 forbids an expectation that came from the code under
//! test. There is no code under test here — nothing of iccce's runs. The
//! expectation came from lcms2 and is compared against lcms2, which is a
//! *regression* claim about the toolchain, and the [`Kind`] on the record
//! says exactly that. **These numbers must never be transplanted into an
//! `iccce-color` or `iccce-cmm` unit test as expected values.** At that
//! moment they would become an expectation derived from an implementation,
//! and the claim would silently change from "the oracle still answers the
//! same" to "iccce is correct", which it would not support.

use std::io::Write;

use iccce_difftest::{
    Bpc, Check, Intent, Kind, Metric, Oracle, Outcome, Precalc, Report, Request, Space, Tolerance,
};

/// The system sRGB profile used by `README.md` §8.2.
///
/// **Category (c) per `LEGAL.md` §3**: read from the local system, never
/// committed, and never a required input. Absent on the Linux runner by
/// construction — the path is Windows-specific.
const SYSTEM_SRGB: &str = r"C:\Windows\System32\spool\drivers\color\sRGB Color Space Profile.icm";

/// Agreement to the precision `transicc` prints, and no tighter.
///
/// **Justification (`CLAUDE.md` rule 5).** `transicc -n` prints four decimal
/// places. The expectation in `README.md` §8.2 is itself a four-decimal
/// print, so the reference is only known to ±5×10⁻⁵ and agreement cannot
/// honestly be asserted more tightly than the reference is printed. 1×10⁻⁴ is
/// that printed precision, taken as the bound rather than the half-ulp so
/// that a last-digit rounding difference between platforms is not a failure.
///
/// **This is an arithmetic-agreement tolerance, not a perceptual one.** The
/// 1.0 ΔE2000 anchor (`TOLERANCES.md` §2, `ARCHITECTURE.md` DL-004) is
/// irrelevant to it and must not be cited in its support.
///
/// **What it is sensitive enough to catch**, which is the real test of a
/// tolerance: the v2/v4 legacy Lab encoding error — the single richest source
/// of CMM bugs (`ARCHITECTURE.md` §2) — moves `L*` at white by ≈0.39, which
/// is about 3900× this bound. Any change that matters is enormous here.
const PRINTED_PRECISION: Tolerance = Tolerance::new(
    1e-4,
    "transicc -n prints 4 decimals and the recorded expectation is itself a 4-decimal print, \
     so agreement cannot be asserted tighter than the reference is printed; \
     arithmetic-agreement, NOT perceptual (the 1.0 dE2000 anchor is irrelevant here)",
);

fn checks() -> Vec<Check> {
    vec![Check {
        id: "smoke/srgb-white-to-lab",
        kind: Kind::OracleReproducibility,
        metric: Metric::AbsMaxComponent,
        tolerance: PRINTED_PRECISION,
        request: Request {
            input: Space::profile(SYSTEM_SRGB),
            output: Space::lab_v4(),
            intent: Intent::RelativeColorimetric,
            // README §8.2 recorded this with no -c flag, i.e. lcms2's default
            // (-c1). Exact (-c0) is used here because an oracle should be the
            // reference implementation's most accurate path. Verified
            // 2026-08-11: all five of (no flag), -c0, -c1, -c2, -c3 print the
            // identical triplet for this transform, so the substitution does
            // not change what is being reproduced.
            precalc: Precalc::Exact,
            bpc: Bpc::Off,
            values: vec![255.0, 255.0, 255.0],
        },
        expected: vec![99.9988, 0.0188, -0.0173],
        source: "tools/difftest/README.md §8.2, recorded 2026-08-11 from this same pinned \
                 oracle — an ORACLE VALUE, not a published reference; do not transplant",
    }]
}

fn main() {
    let mut report = Report::new();

    let oracle = match Oracle::locate() {
        Err(e) => {
            eprintln!("difftest: {e}");
            std::process::exit(2);
        }
        Ok(None) => {
            // No oracle on this machine. Every check skips, and the exit code
            // is 3 ("nothing ran"), never 0. A green CI line with no oracle
            // is the exact shape of a test suite that has quietly stopped
            // testing anything.
            report.note(
                "no transicc found: set ICCCE_TRANSICC, or run fetch-lcms2.sh + build-lcms2.{ps1,sh}",
            );
            for c in checks() {
                report.push(
                    c,
                    Outcome::Skip {
                        reason: "oracle not built on this machine".into(),
                    },
                );
            }
            finish(&report);
            return;
        }
        Ok(Some(o)) => o,
    };

    report.note(format!("oracle: {}", oracle.path().display()));
    match oracle.check_banner("LittleCMS 2.19") {
        Ok(b) => report.note(format!("banner: {b}")),
        Err(e) => {
            // A wrong-version oracle is an error, not a skip: results from it
            // would be attributed to the pin and would not be reproducible.
            report.note(format!("banner check FAILED: {e}"));
            for c in checks() {
                report.push(
                    c,
                    Outcome::Error {
                        detail: format!("oracle version check failed: {e}"),
                    },
                );
            }
            finish(&report);
            return;
        }
    }

    for c in checks() {
        let outcome = c.run(&oracle);
        report.push(c, outcome);
    }
    finish(&report);
}

fn finish(report: &Report) {
    let mut out = std::io::stdout().lock();
    report.emit(&mut out).expect("stdout");
    let _ = out.flush();
    std::process::exit(report.exit_code());
}
