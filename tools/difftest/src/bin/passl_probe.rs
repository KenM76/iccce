//! # `passl_probe` — Pass L on its own, with the tables the suite cannot print
//!
//! The suite runner emits one TSV row per [`iccce_difftest::Record`]. Pass L's
//! subject is a pair of curves that differ by `4.8×10⁻⁶`, and the things a
//! reader actually wants — *where* the two instruments' maxima are, *how many*
//! printed quanta each one buys, and the probe-by-probe residual against both
//! candidate readings — are tables, not scalars. They are printed here.
//!
//! ```text
//! cargo run --release --bin passl_probe
//! ```
//!
//! **Exit code:** the `Report`'s — `0` if every Pass L row passed. Do not read
//! a `0` here as the suite passing; it says nothing about Passes 3–K.
//!
//! ★ It runs §C even with **no oracle on the machine**, because §C is iccce
//! against iccce and needs none. §A and §B then skip with a reason.

use std::io::Write;

use iccce_difftest::{Oracle, Report, passl};

fn main() {
    let oracle = match Oracle::locate() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("passl_probe: {e}");
            None
        }
    };
    if oracle.is_none() {
        eprintln!(
            "passl_probe: no transicc found - section A (which reading lcms2 implements) and \
             section B (the precision analysis) will SKIP. Section C (the cost of the choice) \
             needs no oracle and still runs."
        );
    }

    let (a, records) = passl::run(oracle.as_ref());

    println!("=== Pass L - the two readings of sRGB ================================");
    println!(
        "{:<18} {:>24} {:>24}",
        "parametric [g,a,b,c,d]", "C0 (ICC / W3C / Khronos)", "C1 (H.273 clause 8.2)"
    );
    for (i, label) in ["g", "a", "b", "c", "d"].iter().enumerate() {
        println!("{label:<18} {:>24.17} {:>24.17}", a.p_c0[i], a.p_c1[i]);
    }
    println!();
    println!("=== where the curves separate, per instrument ========================");
    println!(
        "{:<26} {:>16} {:>12} {:>12}",
        "instrument", "max separation", "at code", "printed q"
    );
    println!(
        "{:<26} {:>16.6e} {:>12.5} {:>12.1}",
        "Lab (L*, quantum 1e-4)",
        a.l_sep_max,
        a.l_sep_code,
        a.l_sep_max / 1.0e-4
    );
    println!(
        "{:<26} {:>16.6e} {:>12.5} {:>12.1}",
        "XYZ (Y x 100, q 1e-4)",
        a.y_sep_max * 100.0,
        a.y_sep_code,
        a.y_sep_max * 100.0 / 1.0e-4
    );
    println!(
        "  -> the two maxima are {:.1} codes apart; neither is at a breakpoint",
        (a.l_sep_code - a.y_sep_code).abs()
    );
    println!();

    if let Some(i) = &a.lab {
        println!("=== §A - the Lab instrument ==========================================");
        println!("probes                                  {}", i.n);
        println!(
            "max |lcms2 - C0|                        {:.6e} L*  at code {:.5}",
            i.resid_c0_max, i.resid_c0_code
        );
        println!(
            "max |lcms2 - C1|                        {:.6e} L*  at code {:.5}",
            i.resid_c1_max, i.resid_c1_code
        );
        println!(
            "ratio (how much further the rival sits) {:.1}x",
            i.resid_c1_max / i.resid_c0_max
        );
        println!(
            "at the C1 breakpoint (zero-separation)  |-C0| {:.6e}   |-C1| {:.6e}",
            i.at_c1_break.0, i.at_c1_break.1
        );
        println!(
            "at the L* interior maximum             |-C0| {:.6e}   |-C1| {:.6e}",
            i.at_l_max.0, i.at_l_max.1
        );
        println!(
            "probes resolvable at >= 2 quanta        {} (C0 {} / C1 {})",
            i.votes_n, i.votes_c0, i.votes_c1
        );
        println!(
            "L* signal at the LINEAR-LIGHT maximum   {:.6e} = {:.0}% of the best available",
            i.l_sep_at_y_max,
            100.0 * i.l_sep_at_y_max / a.l_sep_max
        );
        println!();
    }
    if let (Some(c0), Some(c1)) = (a.xyz_resid_c0, a.xyz_resid_c1) {
        println!("=== §A - the XYZ instrument (independent) ============================");
        println!("|lcms2 - C0| {c0:.6e}   |lcms2 - C1| {c1:.6e}   (Y x 100 units)");
        println!();
    }
    match a.source_constants_missing {
        Some(0) => println!(
            "=== §A - whitebox: cmsvirt.c Build_sRGBGamma carries all four C0 constants ===\n"
        ),
        Some(n) => {
            let msg = format!("{n} of 4 C0 constants MISSING - CHECK THE PIN");
            println!("=== §A - whitebox: {msg} ===");
            println!();
        }
        None => println!("=== §A - whitebox: vendor/lcms2 not on disk ===\n"),
    }
    if let Some((runs, n, span)) = a.staircase {
        println!("=== §B - the oracle is NOT a usable destination ruler =================");
        println!(
            "one ink swept {span:.4e} % ({:.2} sixteen-bit quanta) in {n} samples -> {runs} \
             distinct L*",
            span * 65_535.0 / 100.0
        );
        println!(
            "tread ~{:.2e} % ink; the reading choice moves a destination ink by at most ~6.4e-3 %",
            span / (f64::from(u32::try_from(runs).unwrap_or(1)) - 1.0).max(1.0)
        );
        println!();
    }

    println!("=== §C - what the choice costs (self-comparison, iccce vs iccce) =====");
    println!(
        "PCS dE2000  max {:.6e}  at rgb ({:.9}, {:.9}, {:.9}) = codes ({:.4}, {:.4}, {:.4})",
        a.pcs_de_max,
        a.pcs_de_arg[0],
        a.pcs_de_arg[1],
        a.pcs_de_arg[2],
        a.pcs_de_arg[0] * 255.0,
        a.pcs_de_arg[1] * 255.0,
        a.pcs_de_arg[2] * 255.0
    );
    println!(
        "PCS dE2000  mean {:.6e} over {} probes",
        a.pcs_de_mean, a.pcs_de_n
    );
    println!(
        "PCS dE2000  neutral ramp only, max {:.6e} at code {:.4} = {:.0}% of the true max",
        a.pcs_de_neutral_max,
        a.pcs_de_neutral_code,
        100.0 * a.pcs_de_neutral_max / a.pcs_de_max
    );
    println!(
        "one 16-bit PCS L* quantum at that point is {:.6e} dE2000 -> the max is {:.2}x a quantum",
        a.pcs_quantum_de,
        a.pcs_de_max / a.pcs_quantum_de
    );
    println!();
    println!(
        "{:<24} {:>6} {:>14} {:>9} {:>10} {:>14}",
        "destination", "src", "dev sep (0..1)", "8-bit", "16-bit", "e2e dE2000"
    );
    for d in &a.dests {
        println!(
            "{:<24} {:>6} {:>14.6e} {:>9} {:>10} {:>14}",
            d.name,
            if d.committed { "fix" } else { "sys" },
            d.device_sep_max,
            format!("{}/{}", d.codes_changed_8, d.n),
            format!("{}/{}", d.codes_changed_16, d.n),
            d.e2e_de_max
                .map_or_else(|| "-".to_string(), |v| format!("{v:.6e}"))
        );
    }
    println!();

    let mut report = Report::new();
    report.note(format!("passl: {}", passl::note(&a)));
    for r in records {
        report.push_record(r);
    }
    let mut out = std::io::stdout().lock();
    report.emit(&mut out).expect("stdout");
    let _ = out.flush();
    std::process::exit(report.exit_code());
}
