//! # `passi_probe` — Pass I on its own, with the full per-cell table
//!
//! The suite runner prints one summary line per pass and one row per `Record`.
//! Pass I's subject is a 3×3 matrix, so the thing a reader actually wants — all
//! nine cells of all four variants, side by side, in both XYZ units and
//! `s15Fixed16` ULP — would be nine columns of noise in the suite's TSV and is
//! emitted here instead.
//!
//! It is also the fastest way to re-check the pass after touching
//! `iccce_color::BRADFORD`, `D65_XY`, `D50` or `Mat3::inverse`: Pass I needs no
//! oracle and no fixture, so this binary runs in milliseconds on any machine
//! and its exit code is Pass I's alone.
//!
//! ```text
//! cargo run --release --bin passi_probe
//! ```
//!
//! **Exit code:** the `Report`'s — `0` if every Pass I row passed. Do not read
//! a `0` here as the suite passing; it says nothing about Passes 3–H.

use std::io::Write;

use iccce_difftest::{Report, passi};

fn main() {
    let (a, records) = passi::run();

    let mut report = Report::new();
    report.note(format!("passi: {}", passi::note(&a)));
    for r in records {
        report.push_record(r);
    }

    println!("=== Pass I — the four matrices, cell by cell =========================");
    println!(
        "{:<6} {:>22} {:>22} {:>14} {:>10}",
        "cell", "iccce", "ICC published", "difference", "ULP"
    );
    for i in 0..3 {
        for j in 0..3 {
            let got = a.iccce_chad[i][j];
            let want = passi::PUBLISHED_CHAD[i][j];
            println!(
                "r{i}c{j}   {got:>22.15} {want:>22.15} {:>14.6e} {:>10.3}",
                got - want,
                (got - want) * 65536.0
            );
        }
    }
    println!();
    println!("=== the shipped sRGB colorants vs ICC section B.2 ====================");
    println!(
        "{:<6} {:>22} {:>22} {:>14} {:>10}",
        "cell", "builtin::srgb()", "ICC published", "difference", "ULP"
    );
    for i in 0..3 {
        for j in 0..3 {
            let got = a.shipped_colorants[i][j];
            let want = passi::PUBLISHED_COLORANTS[i][j];
            println!(
                "r{i}c{j}   {got:>22.15} {want:>22.15} {:>14.6e} {:>10.3}",
                got - want,
                (got - want) * 65536.0
            );
        }
    }
    println!();

    let mut out = std::io::stdout().lock();
    report.emit(&mut out).expect("stdout");
    let _ = out.flush();
    std::process::exit(report.exit_code());
}
