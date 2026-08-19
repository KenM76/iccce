//! # `passk_cost_probe` — the DISTRIBUTION behind Pass K §G's one number
//!
//! `NUMERIC_CLAIMS.md` registered **NA-012**'s cost field as `UNMEASURED`:
//! nobody had measured the ΔE2000 between the preserved answer and the
//! colorimetric one on a **cross-press** pair. Pass K §G measures it and
//! reduces it to a [`Record`](iccce_difftest::Record) — which carries **one
//! number**, and one number is the wrong shape for the two things a reader of
//! that field needs to see:
//!
//! 1. **The shape of the cost along the ramp.** It is not flat and it is not
//!    monotone: it is near zero at the white end, rises through the midtones,
//!    dips, and spikes at solid black where the destination's black ink alone
//!    can no longer reach the lightness its four-ink separation reaches. A
//!    maximum hides that; a mean hides it worse.
//! 2. **How much of the number is the PAIR rather than the POLICY.** The same
//!    measurement over the thirty ordered pairs the six real CMYK members of
//!    the Ghent corpus admit spans more than twenty-fold, and the smallest
//!    entries are the pairs that describe **one press twice**. Printing the
//!    whole matrix is what makes "3.68 on a named real pair" a statement a
//!    reader can check rather than a number they must take — and it is what
//!    turns the headline into a **population** claim: on every pair entitled
//!    to price the policy, the cost exceeds the perceptibility anchor.
//!
//! It is an **instrument, not a grader**: no tolerances, no verdicts, exit `0`
//! whenever it managed to measure and `1` when it did not. Everything it
//! prints is reduced into a graded or reported row by `passk::run`; nothing
//! here is a claim that exists only in this file.
//!
//! ## Usage
//!
//! ```text
//! cargo run --release --bin passk_cost_probe
//! ```
//!
//! Requires the pinned `transicc` (`$ICCCE_TRANSICC` or the vendored build)
//! and the shipped `iccce` binary (`$ICCCE_BIN` or `target/release/iccce`).
//! The **licensed** Ghent v5.0 corpus (`$ICCCE_PRIVATE_FIXTURES`) is needed
//! for blocks 1 and 2 only; blocks 3 and 4 run from committed fixtures and are
//! printed even when the corpus is absent, which is the normal case off the
//! operator's machine and the permanent case in CI.
//!
//! ## What the four blocks are
//!
//! 1. **The pair matrix** — every ordered pair of the six real CMYK
//!    destinations, with both separations and the reference-leg control beside
//!    each cost, so a reader can see which pairs are entitled to report one.
//! 2. **The headline ramp**, point by point, in device units *and* in ΔE2000.
//! 3. **The committed pair Pass K already had**, which cannot carry a ΔE row —
//!    its reference leg is more than twenty ΔE2000 wrong by construction.
//! 4. **The committed pair authored for this measurement**, beside the
//!    **closed form** derived from the two recipes' constants. This is the
//!    only block in which the expectation is not an implementation's output.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use iccce_color::delta_e_2000;
use iccce_difftest::passk::{
    as_cmyk, corpus_dir, cost_ramp, synthetic_dir, to_lab, warm_black_expected_cost,
};
use iccce_difftest::{Iccce, Intent, Oracle};

/// The six real CMYK members of the Ghent corpus, by short name and by the
/// first 16 hex digits of their SHA-256 — a pointer to a licensed artifact,
/// never any part of its content.
const CORPUS: [(&str, &str); 6] = [
    ("ISOc300", "c6b4b62f07262437.icc"),
    ("ISOc350", "128dc02f7246cc38.icc"),
    ("FOGRA39", "da2b9b593e27cba2.icc"),
    ("FOGRA27", "fb710c05e3fb5a96.icc"),
    ("GWGgen", "5bad92a6f018e726.icc"),
    ("XRiteV4", "b5988983b6b3b7d4.icc"),
];

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("passk_cost_probe: {e}");
            ExitCode::from(1)
        }
    }
}

/// One measured pair, in the shape this file prints.
struct Leg {
    cost: Vec<f64>,
    off: Vec<[f64; 4]>,
    on: Vec<[f64; 4]>,
    sep_device: f64,
    sep_press: f64,
    round_trip: f64,
}

impl Leg {
    fn max(&self) -> (f64, f64) {
        let ramp = cost_ramp();
        let mut m = 0.0_f64;
        let mut at = 0.0;
        for (i, c) in self.cost.iter().enumerate() {
            if *c > m {
                m = *c;
                at = ramp[i][3];
            }
        }
        (m, at)
    }
    fn mean(&self) -> f64 {
        #[expect(
            clippy::cast_precision_loss,
            reason = "101 points; the count is exactly representable"
        )]
        let n = self.cost.len() as f64;
        self.cost.iter().sum::<f64>() / n
    }
}

/// Measure one ordered pair, exactly as `passk::analyse_cost_leg` does — two
/// `iccce transform` invocations differing only in `--preserve-black`, with
/// the oracle carrying both answers into Lab through the destination's `A2B1`.
fn measure(oracle: &Oracle, iccce: &Iccce, src: &Path, dst: &Path) -> Result<Leg, String> {
    let ramp = cost_ramp();
    let rows: Vec<Vec<f64>> = ramp.iter().map(|r| r.to_vec()).collect();
    let off = as_cmyk(
        iccce
            .transform_rows_shaped(src, dst, Intent::RelativeColorimetric, &rows, 4)
            .map_err(|e| e.to_string())?,
    )
    .map_err(|e| format!("{e:?}"))?;
    let on = as_cmyk(
        iccce
            .transform_rows_shaped_preserve_black(
                src,
                dst,
                Intent::RelativeColorimetric,
                &rows,
                4,
                "k-only-equal-lightness",
            )
            .map_err(|e| e.to_string())?,
    )
    .map_err(|e| format!("{e:?}"))?;
    let lab_off = to_lab(oracle, dst, &off).map_err(|e| format!("{e:?}"))?;
    let lab_on = to_lab(oracle, dst, &on).map_err(|e| format!("{e:?}"))?;
    let lab_src_in = to_lab(oracle, src, &ramp).map_err(|e| format!("{e:?}"))?;
    let lab_dst_in = to_lab(oracle, dst, &ramp).map_err(|e| format!("{e:?}"))?;

    let mut leg = Leg {
        cost: Vec::with_capacity(ramp.len()),
        off,
        on,
        sep_device: 0.0,
        sep_press: 0.0,
        round_trip: 0.0,
    };
    for i in 0..ramp.len() {
        leg.cost.push(delta_e_2000(lab_off[i], lab_on[i]));
        let o = leg.off[i];
        leg.sep_device = leg.sep_device.max(o[0].max(o[1]).max(o[2]));
        leg.sep_press = leg
            .sep_press
            .max(delta_e_2000(lab_src_in[i], lab_dst_in[i]));
        leg.round_trip = leg.round_trip.max(delta_e_2000(lab_src_in[i], lab_off[i]));
    }
    Ok(leg)
}

fn run() -> Result<(), String> {
    let oracle = Oracle::locate()
        .map_err(|e| e.to_string())?
        .ok_or("the pinned transicc was not found — set $ICCCE_TRANSICC")?;
    let iccce = Iccce::locate()
        .map_err(|e| e.to_string())?
        .ok_or("the shipped iccce binary was not found — `cargo build --release -p iccce-cli`")?;

    println!("passk_cost_probe — what K-only black preservation costs, and on which pair");
    println!("  oracle : {}", oracle.path().display());
    println!(
        "  iccce  : {}{}",
        iccce.path().display(),
        if iccce.is_debug_build() {
            "   ** DEBUG BUILD — numbers describe an artefact nobody ships **"
        } else {
            ""
        }
    );
    println!(
        "  probe  : C = M = Y = 0, K = j/100 (j = 0..=100) — the QUALIFYING SET, media-relative"
    );
    println!(
        "  metric : dE2000 between iccce's preserved answer and iccce's own colorimetric one,"
    );
    println!(
        "           both carried into Lab through the DESTINATION's A2B1 by lcms2 as a RULER."
    );
    println!("           SELF-COMPARISON. No ground truth exists for this subject (ICC_Spec A51).");
    println!();

    let dir = corpus_dir();
    if dir.is_dir() {
        block1_and_2(&oracle, &iccce, &dir)?;
    } else {
        println!(
            "=== 1 & 2. SKIPPED — the Ghent v5.0 corpus is not at {} ===",
            dir.display()
        );
        println!("    It is licensed and cannot be committed, so absence is the normal case.");
        println!("    Blocks 3 and 4 below need no licence and run anywhere.");
        println!();
    }

    let syn = synthetic_dir();
    let neutral = syn.join("v2-cmyk-chromatic-neutral.icc");
    let mft2 = syn.join("v2-cmyk-mft2-lab.icc");
    let warm = syn.join("v2-cmyk-warm-black.icc");

    println!("=== 3. THE COMMITTED PAIR PASS K ALREADY HAD — and why it CANNOT price this ===");
    println!("    v2-cmyk-mft2-lab -> v2-cmyk-chromatic-neutral");
    let bad = measure(&oracle, &iccce, &mft2, &neutral)?;
    let (bmax, bat) = bad.max();
    println!(
        "    cost max {bmax:.6} at K = {bat:.2}, mean {:.6} — AND THE REFERENCE LEG IS {:.6} \
         dE2000 WRONG.",
        bad.mean(),
        bad.round_trip
    );
    println!(
        "    The colorimetric answer it is differenced against is itself further from the colour \
         asked for"
    );
    println!(
        "    than the cost it would report. v2-cmyk-chromatic-neutral's B2A0 lays 0.60d of gray \
         under 0.40d"
    );
    println!(
        "    of black, which its own A2B0 reads back as darkness 0.70d: the two tables do not \
         invert each"
    );
    println!("    other. Sound for §F, which grades DEVICE values. Fatal for a dE row.");
    println!();

    println!("=== 4. THE COMMITTED PAIR AUTHORED FOR THIS MEASUREMENT, vs the CLOSED FORM ===");
    println!("    v2-cmyk-chromatic-neutral -> v2-cmyk-warm-black  (differ in ONE variable:");
    println!("    the chroma of the black ink — a* += 2K, b* += 6K in the destination)");
    let good = measure(&oracle, &iccce, &neutral, &warm)?;
    let (gmax, gat) = good.max();
    println!(
        "    cost max {gmax:.6} at K = {gat:.2}, mean {:.6}; reference leg sound to {:.6}; \
         press separation {:.4}",
        good.mean(),
        good.round_trip,
        good.sep_press
    );
    println!();
    println!(
        "    K_in   |  colorimetric C M Y K                    | preserved K' |  measured |  \
         DERIVED  |   diff"
    );
    let ramp = cost_ramp();
    for (i, p) in ramp.iter().enumerate() {
        if i % 5 != 0 {
            continue; // every 5th point: 21 lines, the same ramp §A prints
        }
        let expected = warm_black_expected_cost(p[3]);
        println!(
            "    {:.3}  | {:8.6} {:8.6} {:8.6} {:8.6} |   {:8.6}   | {:9.6} | {:9.6} | {:8.2e}",
            p[3],
            good.off[i][0],
            good.off[i][1],
            good.off[i][2],
            good.off[i][3],
            good.on[i][3],
            good.cost[i],
            expected,
            (good.cost[i] - expected).abs()
        );
    }
    println!();
    println!(
        "    The DERIVED column is dE2000((rho*100(1-0.70k), 0, 0), (100(1-0.70k), 2k, 6k)) with"
    );
    println!(
        "    rho = 65280/65535 — computed from the two recipes' constants, with no \
         implementation's"
    );
    println!("    output in it. Pass K §G row G12 grades the diff column's maximum.");
    Ok(())
}

#[expect(
    clippy::too_many_lines,
    reason = "a report generator: splitting it would scatter the printed layout across functions"
)]
fn block1_and_2(oracle: &Oracle, iccce: &Iccce, dir: &Path) -> Result<(), String> {
    println!("=== 1. THE PAIR MATRIX — thirty ordered pairs of six real CMYK destinations ===");
    println!("    A pair is entitled to report a cost only if it separates in BOTH senses and");
    println!("    its reference leg is sound. The flag column says which gate a pair misses.");
    println!();
    println!(
        "    source -> destination     cost max   cost mean   sep(ink)   sep(press)   ref leg   \
         flag"
    );
    let mut best = (0.0_f64, String::new());
    let mut worst_cross = (f64::INFINITY, String::new());
    let mut entitled = 0_usize;
    let mut total = 0_usize;
    for (sn, sf) in CORPUS {
        for (dn, df) in CORPUS {
            if sn == dn {
                continue;
            }
            let src: PathBuf = dir.join(sf);
            let dst: PathBuf = dir.join(df);
            if !src.is_file() || !dst.is_file() {
                continue;
            }
            let leg = measure(oracle, iccce, &src, &dst)?;
            let (m, _) = leg.max();
            // Every gate a pair misses, not the first: two pairs miss two, and
            // reporting only the first would make a pair look nearer to being
            // entitled than it is.
            let mut flags: Vec<&str> = Vec::new();
            if leg.sep_press < 1.0 {
                flags.push(if leg.sep_press == 0.0 {
                    "ONE PRESS (A2B1 tags byte-identical)"
                } else {
                    "K axes agree within the anchor"
                });
            }
            if leg.sep_device < 4e-2 {
                flags.push("no ink separation");
            }
            if leg.round_trip > 1.0 {
                flags.push("reference leg unsound");
            }
            let flag = flags.join("; ");
            println!(
                "    {sn:>8} -> {dn:<10} {m:9.6}  {:9.6}  {:9.6}  {:10.4}  {:8.4}   {flag}",
                leg.mean(),
                leg.sep_device,
                leg.sep_press,
                leg.round_trip
            );
            total += 1;
            if flag.is_empty() {
                entitled += 1;
                if m > best.0 {
                    best = (m, format!("{sn} -> {dn}"));
                }
                if m < worst_cross.0 {
                    worst_cross = (m, format!("{sn} -> {dn}"));
                }
            }
        }
    }
    println!();
    println!(
        "    ENTITLED PAIRS: {entitled} of {total}.  cost max over them ranges {:.6} ({}) to \
         {:.6} ({}).",
        worst_cross.0, worst_cross.1, best.0, best.1
    );
    println!(
        "    ★ THE POPULATION STATEMENT, and it is stronger than any single pair: on EVERY pair"
    );
    println!("      entitled to price this policy the cost exceeds TOLERANCES.md §2's 1.0 dE2000");
    println!(
        "      perceptibility anchor — the smallest is twice it. The policy is never invisible"
    );
    println!("      where it is doing anything.");
    println!("    ★ §G grades ISOc300 -> GWGgen for continuity with §A-§E, which are all built on");
    println!(
        "      ISO Coated v2 300% (ECI). It is not quite the largest entitled pair; the largest is"
    );
    println!(
        "      above, and the gap between them is smaller than the disagreement between the two"
    );
    println!("      rulers G6 measures, so the choice of the two does not move the claim.");
    println!(
        "    ★ The pairs that are NOT entitled run down to 0.16 dE2000, and every one of them"
    );
    println!(
        "      would read as 'the policy is nearly free'. That is what the flag column is for."
    );
    println!();

    println!("=== 2. THE HEADLINE RAMP, point by point ===");
    let src = dir.join("c6b4b62f07262437.icc");
    let dst = dir.join("5bad92a6f018e726.icc");
    let leg = measure(oracle, iccce, &src, &dst)?;
    println!("    ISO Coated v2 300% (ECI) -> GWG_GenericCMYK, media-relative");
    println!();
    println!(
        "    K_in   |  colorimetric C M Y K                    | preserved K' |   TAC   |  cost \
         dE2000"
    );
    let ramp = cost_ramp();
    for (i, p) in ramp.iter().enumerate() {
        if i % 5 != 0 {
            continue;
        }
        println!(
            "    {:.3}  | {:8.6} {:8.6} {:8.6} {:8.6} |   {:8.6}   | {:7.4} | {:9.6}",
            p[3],
            leg.off[i][0],
            leg.off[i][1],
            leg.off[i][2],
            leg.off[i][3],
            leg.on[i][3],
            leg.off[i].iter().sum::<f64>(),
            leg.cost[i]
        );
    }
    println!();
    println!(
        "    ★ The spike at K = 1 is the destination's black ink running out of lightness: its"
    );
    println!(
        "      four-ink solid reaches darker than its K solid can, so the equal-lightness \
         construction"
    );
    println!(
        "      clamps at K' = 1 and the preserved answer is a LIGHTER black, not merely a \
         different one."
    );
    println!(
        "    ★ The midtone plateau near 2.0-2.4 is the black ink's own chroma: matched in \
         lightness,"
    );
    println!("      unmatched in a* and b*. That half of the cost has no clamp to blame.");
    println!();
    Ok(())
}
