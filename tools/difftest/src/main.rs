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
//! ## What is registered here, as of 2026-08-11 (Pass 3)
//!
//! **Eight records: one oracle-reproducibility smoke check and seven Pass 3
//! records.** The difference between them matters more than the count:
//!
//! ### 1 — the smoke check, which is lcms2 against lcms2
//!
//! - `smoke/srgb-white-to-lab` — the sRGB white → Lab conversion recorded in
//!   `README.md` §8.2, re-run and compared to the recorded numbers. Its kind
//!   is [`Kind::OracleReproducibility`]: **both sides are lcms2**. It
//!   establishes that the harness drives the oracle correctly and that the
//!   pin and toolchain still produce the recorded answer. It says nothing
//!   whatever about whether iccce is right; iccce is not in that loop.
//!
//! ### 7 — Pass 3, which is the first time iccce is graded at all
//!
//! Built by [`iccce_difftest::pass3`] over a 133-point deterministic grid,
//! sRGB → Adobe RGB (1998), media-relative colorimetric, `-c0`:
//!
//! | id | kind | what it can catch |
//! |---|---|---|
//! | `pass3/srgb-to-adobergb/device-vs-lcms2` | cross-check | arithmetic disagreement with lcms2 in device units |
//! | `pass3/srgb-to-adobergb/device-mean` | cross-check | **reported, not graded** (tolerance ∞) |
//! | `pass3/srgb-to-adobergb/de2000-vs-lcms2` | cross-check | the same disagreement, expressed perceptually |
//! | `pass3/srgb-to-adobergb/de2000-mean` | cross-check | **reported, not graded** (tolerance ∞) |
//! | `pass3/srgb-to-adobergb-to-srgb/roundtrip-de2000` | self-consistency | the cost of the approximations in the loop — an **upper** bound |
//! | `pass3/roundtrip/white-clamp-cost-matches-prediction` | self-consistency | a **missing** normative F.8–F.16 clamp, which the upper bound above would reward |
//! | `pass3/instrument/adobergb-device-to-lab-ruler` | cross-check | a wrong ruler underneath the ΔE numbers |
//!
//! **Two of the seven are ungraded means.** They pass because there is
//! nothing for them to fail, their tolerance prints as `inf`, and their `why`
//! string says so. A green line is not evidence; the `kind` and the tolerance
//! on the same line are what tell a reader what it is worth.
//!
//! ### 28 — Pass 4, the LUT differential (added later on 2026-08-11)
//!
//! Built by [`iccce_difftest::pass4`] over a 341-point CMYK grid,
//! `USWebCoatedSWOP.icc` → the system sRGB profile, at **all four intents**,
//! `-c0`. Full record: README §14.
//!
//! | id (per intent unless noted) | kind | what it can catch |
//! |---|---|---|
//! | `pass4/apparatus/harness-nlinear-matches-iccce-cmm` *(once)* | self-consistency | **the apparatus**: the harness's own mft2 reimplementation drifting from `iccce-cmm`'s. Everything else in Pass 4 is void if this fails |
//! | `pass4/swop/perceptual-equals-saturation` *(once)* | cross-check | an 8.10.2 tag-selection defect — graded at **exactly zero**, because `A2B0` and `A2B2` are one block of tag data |
//! | `…/device-vs-lcms2`, `…/de2000-vs-lcms2` | cross-check | structural error, at a tolerance that is the **interpolation-method envelope** and therefore cannot claim agreement |
//! | `…/device-mean`, `…/de2000-mean` | cross-check | **reported, not graded** (tolerance ∞) |
//! | `…/pcs-lab-vs-lcms2` | cross-check | the same, isolated to the source CLUT |
//! | **`…/pcs-lab-emulated-geometry`** | cross-check | **the record that claims agreement** — lcms2's own CLUT geometry substituted, gated 100× tighter |
//! | **`…/pcs-lab-corners-interpolation-free`** | cross-check | the sensitivity control: 16 exact CLUT nodes, gated at 10× `transicc`'s print floor |
//! | `icc-absolute/white-point-policy-emulated` | cross-check | the **mechanism** of the 11 ΔE00 absolute-intent divergence |
//!
//! **At `icc-absolute` the two raw comparisons are reported with an infinite
//! tolerance**, because iccce and lcms2 read *different destination media
//! whites* there and the specification question is unsourced (corpus A4b).
//! The gate at that intent is the white-point-policy record. README §14.6 and
//! `TOLERANCES.md` §3.4 state it in full; it is the one place in this suite
//! where a known disagreement is deliberately not gated, and it is labelled
//! rather than absorbed into a widened number.
//!
//! ### 28 — Pass 4b, the directions Pass 4 left unmeasured (added later still)
//!
//! Built by [`iccce_difftest::pass4b`] in **three independent sections**, so a
//! missing system profile in one does not silence the others. Full record:
//! README §15.
//!
//! | id | kind | what it can catch |
//! |---|---|---|
//! | `pass4b/srgb-to-swop/<intent>/apparatus-lut8-matches-iccce-cmm` | self-consistency | **the apparatus** — as Pass 4's row 0, for the `lut8` pipeline |
//! | `…/device-vs-lcms2`, `…/roundtrip-lab-de2000` | cross-check | arithmetic disagreement in the **B2A** direction, at a tolerance that is a computed quantisation envelope |
//! | **`…/device-lcms2-arithmetic-modelled`** | cross-check | **the row that claims agreement** — every lcms2 rounding modelled, gated 10× tighter |
//! | `…/counterfactual-tetrahedral` | cross-check | **reported**: the sensitivity control. lcms2 forces *trilinear* for a Lab-PCS LUT, so the method envelope is zero; this says what it would have been |
//! | `pass4b/fixture/{mab,mba}/{iccce,lcms2}-vs-derived-expectation` | **derived-expectation** | a wrong element order, a wrong PCSLAB encoding, dropped matrix offsets — against a **closed form**, not against an oracle |
//! | `pass4b/fixture/mab/encoded-pcs-overflow-divergence` | cross-check | **reported, not graded**: iccce clamps the encoded PCS and lcms2 does not, worth 0.61 ΔE2000, and the clause question is unsettled |
//! | `pass4b/fixture/forced-bpc-is-decided-by-the-DESTINATION-version` | oracle-reproducibility | **both sides are lcms2** — the size and the *direction-dependence* of the DL-013 confound |
//! | `pass4b/gray-to-srgb/…` | cross-check | the **F.2 grayTRC** model; the attribution row reproduces lcms2's 4096-entry reverse tone curve and collapses the residual 457× |
//!
//! **§B's four derived rows are the first graded rows in this suite that need
//! no system profile at all** — they read `fixtures/synthetic/v4-cmyk-mab-lab.icc`,
//! which is category (a) and committed. Everything else still skips.
//!
//! ## Everything here skips without the Windows colour directory
//!
//! Every check's input is a category (c) system profile (`LEGAL.md` §3), and
//! Pass 3 additionally needs `target/release/iccce`. On a runner missing
//! either, this binary exits **3, not 0**. That is the intended behaviour: a
//! suite that skipped everything has not passed.
//!
//! ## Why the smoke check's expectation is allowed to come from lcms2
//!
//! `CLAUDE.md` rule 3 forbids an expectation that came from the code under
//! test. In the smoke check there is no code under test — nothing of iccce's
//! runs. The expectation came from lcms2 and is compared against lcms2, which
//! is a *regression* claim about the toolchain, and the [`Kind`] on the record
//! says exactly that. **These numbers must never be transplanted into an
//! `iccce-color` or `iccce-cmm` unit test as expected values.** At that
//! moment they would become an expectation derived from an implementation,
//! and the claim would silently change from "the oracle still answers the
//! same" to "iccce is correct", which it would not support.
//!
//! The Pass 3 records have no recorded expectation at all: **both sides are
//! computed in the run**, which is why they are cross-checks and not
//! reproducibility checks, and why nothing in them can go stale.

use std::io::Write;

use iccce_difftest::{
    Bpc, Check, Intent, Kind, Metric, Oracle, Outcome, Precalc, Report, Request, Space, Tolerance,
    pass3, pass4, pass4b, pass5, pass5b, pass6,
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
            // Pass 3's records are emitted as skips too, so the report has the
            // same eight lines everywhere. A suite that emits nothing when it
            // cannot run is indistinguishable, in a log, from one that was
            // never wired up.
            for r in pass3::unavailable_records(&pass3::Unavailable::Skip(
                "oracle not built on this machine".into(),
            )) {
                report.push_record(r);
            }
            for r in pass4::unavailable_records(&pass4::Unavailable::Skip(
                "oracle not built on this machine".into(),
            )) {
                report.push_record(r);
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
            for r in pass3::unavailable_records(&pass3::Unavailable::Error(format!(
                "oracle version check failed: {e}"
            ))) {
                report.push_record(r);
            }
            for r in pass4::unavailable_records(&pass4::Unavailable::Error(format!(
                "oracle version check failed: {e}"
            ))) {
                report.push_record(r);
            }
            finish(&report);
            return;
        }
    }

    for c in checks() {
        let outcome = c.run(&oracle);
        report.push(c, outcome);
    }

    // Pass 3. `run` returns its own skip/error records rather than
    // propagating, so a missing profile or an unbuilt binary produces seven
    // labelled SKIP lines instead of an absence.
    let (analysis, records) = pass3::run(&oracle);
    if let Some(a) = &analysis {
        report.note(format!(
            "pass3: iccce={} ({}) grid={} points clipped={} \
             (run `cargo run --bin pass3_report` for the per-point record)",
            a.iccce_exe.display(),
            if a.iccce_is_debug {
                "DEBUG BUILD"
            } else {
                "release"
            },
            a.grid.len(),
            a.clipped_points
        ));
    }
    for r in records {
        report.push_record(r);
    }

    // Pass 4 — the LUT differential. Same contract: it returns its own
    // skip/error records rather than propagating, so a missing profile
    // produces labelled SKIP lines instead of an absence.
    //
    // Its end-to-end side drives the same shipped binary Pass 3 does, via the
    // N-channel `transform` of commit 490191b; only its PCS-side instrument is
    // in-process, and every record says which it is.
    let (p4, p4_records) = pass4::run(&oracle);
    if let Some(a) = &p4 {
        report.note(format!(
            "pass4: grid={} points x 4 intents, iccce={} ({}), {} \
             (run `cargo run --bin pass4_report` for the per-point record and the \
             interpolation experiment)",
            a.grid.len(),
            a.iccce_exe.display(),
            if a.iccce_is_debug { "DEBUG BUILD" } else { "release" },
            a.structure
        ));
    }
    for r in p4_records {
        report.push_record(r);
    }

    // Pass 4b — the three directions Pass 4 left unmeasured: the B2A
    // (`lut8`) direction, the v4 `mAB `/`mBA ` fixture, and the grayTRC
    // model. Same contract again; its three sections are independent, so a
    // missing system profile still leaves the **synthetic** fixture's records
    // graded — which is the first time any LUT row in this suite survives on a
    // machine without the Windows colour directory.
    let (p4b, p4b_records) = pass4b::run(&oracle);
    if let Some(a) = &p4b.b2a {
        report.note(format!("pass4b/A: {}", a.structure));
    }
    if let Some(a) = &p4b.mab {
        report.note(format!("pass4b/B: {}", a.structure));
    }
    if let Some(a) = &p4b.gray {
        report.note(format!("pass4b/C: {}", a.structure));
    }
    for r in p4b_records {
        report.push_record(r);
    }

    // Pass 5 — black point compensation. Its §A (the scaling map against
    // ICC.1 6.3.4.3 and against Maria 2013's two constraints) needs neither a
    // profile nor the oracle, so it is the first section in this suite that
    // grades something on any machine at all; S1–S5 need the Windows colour
    // directory and skip with a reason when it is absent, and S6 (the
    // ICC-absolute refusal) runs off committed fixtures.
    let (p5, p5_records) = pass5::run(&oracle);
    if let Some(a) = &p5 {
        report.note(format!(
            "pass5: {} (run `cargo run --bin pass5_report` for the scenario table, the \
             black-point predictions and the policy measurement)",
            a.structure
        ));
    }
    for r in p5_records {
        report.push_record(r);
    }

    // Pass 5b — the black-point ESTIMATORS, and the corpus's pre-registered
    // prediction about them. Needs the oracle (lcms2's estimator is one arm),
    // the two system profiles and the shipped binary; drives
    // `iccce_cmm::bpc`'s ISO/CD 18619 function IN PROCESS, because
    // `Chain::estimate_dst_black` has no call to it and the shipped binary
    // therefore refuses this case — which one of its records grades.
    let (p5b, p5b_records) = pass5b::run(&oracle);
    if let Some(a) = &p5b {
        report.note(format!("pass5b: {}", a.structure));
    }
    for r in p5b_records {
        report.push_record(r);
    }

    // Pass 6 — the compiled path, graded. **No oracle**: both arms are iccce,
    // so this section runs on a machine with no lcms2 build. It still needs
    // the two system profiles and the shipped binary (it drives `iccce bench`
    // as a subprocess so the numbers it grades are the numbers that command
    // prints) and skips with a reason when either is absent.
    let (p6, p6_records) = pass6::run();
    if let Some(a) = &p6 {
        report.note(format!(
            "pass6: {} | bench {:.3} Mpix/s vs reference {:.3} = {:.2}x              (run `cargo run --bin pass6_report` for the grid table)",
            a.structure,
            a.bench.megapixels_per_second,
            a.bench.reference_megapixels_per_second,
            a.bench.speedup
        ));
    }
    for r in p6_records {
        report.push_record(r);
    }

    finish(&report);
}

fn finish(report: &Report) {
    let mut out = std::io::stdout().lock();
    report.emit(&mut out).expect("stdout");
    let _ = out.flush();
    std::process::exit(report.exit_code());
}
