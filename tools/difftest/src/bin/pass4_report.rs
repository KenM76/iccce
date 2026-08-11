//! # `pass4_report` — the per-point record behind Pass 4's numbers
//!
//! `cargo run --bin pass4_report`
//!
//! The graded records (`cargo run`) reduce a 341-point grid at four intents to
//! a handful of numbers. This binary prints what those numbers are made of,
//! and — more importantly — prints **the two experiments that test Pass 4's
//! tolerance justifications instead of asserting them**:
//!
//! | § | what it shows | why it exists |
//! |---|---|---|
//! | 1 | provenance and profile structure | a substituted profile must not pass unnoticed |
//! | 2 | the five named CMYK points at all four intents | directly comparable with README §8.4's Pass 0 oracle numbers, by eye |
//! | 3 | per-intent reductions | the graded numbers, with their means |
//! | 4 | **the interpolation-method envelope** | computed from the CLUT and the two algorithms ALONE — no lcms2 output enters it. This is what `DE_PCS_CROSSCHECK` and `DE_CROSSCHECK` *are* |
//! | 5 | **the attribution** | how much of the observed iccce-vs-lcms2 disagreement the method difference accounts for. The Pass 3 discipline: predict the confound quantitatively, then measure the residual |
//! | 6 | the corner control | the sixteen CLUT-node points, where the method difference is identically zero |
//! | 7 | worst offenders and out-of-range excursions | where to look first when a number moves |
//!
//! Nothing here is graded. Grading happens in `pass4::records`; this is the
//! evidence a reader needs to decide whether to believe it.

use iccce_difftest::pass4::{self, NAMED_POINTS, Scheme};
use iccce_difftest::{Intent, Oracle};

fn main() {
    let oracle = match Oracle::locate() {
        Ok(Some(o)) => o,
        Ok(None) => {
            eprintln!("no transicc on this machine — nothing to report");
            std::process::exit(3);
        }
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2);
        }
    };

    let a = match pass4::analyse(
        &oracle,
        std::path::Path::new(pass4::SWOP),
        std::path::Path::new(pass4::SRGB),
    ) {
        Ok(a) => a,
        Err(u) => {
            eprintln!("pass4 unavailable: {u}");
            std::process::exit(3);
        }
    };

    println!("=== 1. Provenance ===============================================");
    println!("oracle   : {}", a.oracle_banner);
    println!("source   : {}", a.src_path.display());
    println!("dest     : {}", a.dst_path.display());
    println!("structure: {}", a.structure);
    println!(
        "versions : src {:#010X}, dst {:#010X}  (lcms2 forces BPC at perceptual/saturation only \
         at >= 0x04000000 — DL-013/M2 — so it is UNREACHABLE here)",
        a.version_words.0, a.version_words.1
    );
    println!("grid     : {} CMYK points, deterministic", a.grid.len());
    println!(
        "iccce    : {} ({}) — the SHIPPED binary, N-channel transform (commit 490191b+); \
         only the PCS-side instrument is in-process",
        a.iccce_exe.display(),
        if a.iccce_is_debug { "DEBUG BUILD" } else { "release" }
    );
    println!(
        "apparatus: max |harness n-linear - Lut16Model| over all points/intents = {:.3e} \
         (L*/a*/b* units)",
        a.runs.iter().map(|r| r.self_check).fold(0.0f64, f64::max)
    );
    println!(
        "shared-tag identity: perceptual vs saturation, iccce {:.3e}, lcms2 {:.3e} \
         (normalised device units; A2B0 and A2B2 are one block of tag data)",
        a.per_vs_sat_iccce, a.per_vs_sat_lcms2
    );

    println!();
    println!("=== 2. The five named points (README §8.4 carries Pass 0 oracle numbers) ===");
    println!(
        "{:<38} {:<14} {:>25} {:>25}",
        "point", "intent", "iccce RGB (0..255)", "lcms2 RGB (0..255)"
    );
    for (p, name) in NAMED_POINTS {
        let idx = a.grid.iter().position(|q| *q == p).expect("named points are in the grid");
        for r in &a.runs {
            let i = r.iccce_rgb[idx];
            let l = r.lcms2_rgb_255[idx];
            println!(
                "{:<38} {:<14} {:>8.4}{:>8.4}{:>9.4} {:>8.4}{:>8.4}{:>9.4}",
                name,
                short_intent(r.intent),
                i[0] * 255.0,
                i[1] * 255.0,
                i[2] * 255.0,
                l[0],
                l[1],
                l[2]
            );
        }
    }

    println!();
    println!("=== 3. Per-intent reductions ====================================");
    println!(
        "{:<14} {:>11} {:>11} {:>11} {:>11} {:>11} {:>11}",
        "intent", "dev max", "dev mean", "dE00 max", "dE00 mean", "PCS max", "PCS emul"
    );
    for r in &a.runs {
        let (dev_max, dev_mean) = mm(&r.device_dev_clamped);
        let (de_max, de_mean) = mm(&r.de_end_to_end);
        let (pcs_max, _) = mm(&r.de_pcs);
        let (emu_max, _) = mm(&r.de_pcs_emulated);
        println!(
            "{:<14} {:>11.4e} {:>11.4e} {:>11.4e} {:>11.4e} {:>11} {:>11}",
            short_intent(r.intent),
            dev_max,
            dev_mean,
            de_max,
            de_mean,
            opt(pcs_max),
            opt(emu_max),
        );
    }
    println!(
        "(PCS columns are blank at icc-absolute by design — see the record's skip reason)"
    );

    println!();
    println!("=== 4. The interpolation-method envelope ========================");
    println!(
        "iccce's n-linear (NA-006) against lcms2's Eval4Inputs geometry (linear in C x tetrahedral"
    );
    println!(
        "in M,Y,K), BOTH evaluated in this harness on the SAME table. No lcms2 output enters this."
    );
    println!(
        "This quantity IS the tolerance for the PCS cross-check and, propagated, for the ΔE one."
    );
    println!();
    println!(
        "{:<14} {:>12} {:>12} {:>14} {:>14} {:>14}",
        "intent", "PCS max", "PCS mean", "e2e dE00 max", "e2e dev max", "observed dE00"
    );
    for r in &a.runs {
        let (env_max, env_mean) = mm(&r.de_method_envelope);
        let (e2e_max, _) = mm(&r.de_method_end_to_end);
        let (dev_max, _) = mm(&r.device_method_end_to_end);
        let (obs, _) = mm(&r.de_end_to_end);
        println!(
            "{:<14} {:>12.4e} {:>12.4e} {:>14.4e} {:>14.4e} {:>14.4e}",
            short_intent(r.intent),
            env_max,
            env_mean,
            e2e_max,
            dev_max,
            obs
        );
    }

    println!();
    println!("=== 5. Attribution — does the method difference EXPLAIN the residual? ===");
    println!(
        "The Pass 3 discipline (README §13.6.1): predict the confound from the other"
    );
    println!(
        "implementation's own arithmetic, then measure what is left. If substituting lcms2's"
    );
    println!(
        "interpolation geometry collapses the disagreement, the disagreement WAS the geometry."
    );
    println!();
    for r in &a.runs {
        if r.lcms2_pcs.is_none() {
            println!("{:<14} (PCS not comparable at this intent)", short_intent(r.intent));
            continue;
        }
        let (pcs_max, pcs_mean) = mm(&r.de_pcs);
        let (emu_max, emu_mean) = mm(&r.de_pcs_emulated);
        println!(
            "{:<14} n-linear vs lcms2: max {:.4e} mean {:.4e}   |   emulated geometry vs lcms2: \
             max {:.4e} mean {:.4e}   |   shrink factor {:.1}x (max) {:.1}x (mean)",
            short_intent(r.intent),
            pcs_max,
            pcs_mean,
            emu_max,
            emu_mean,
            pcs_max / emu_max,
            pcs_mean / emu_mean
        );
    }

    println!();
    println!("=== 5b. The absolute intent — the white-point-policy experiment ===");
    println!(
        "lcms2 substitutes D50 for the wtpt of a v2 DISPLAY profile (cmsio1.c,"
    );
    println!(
        "_cmsReadMediaWhitePoint); iccce uses wtpt AS STORED (NA-007), and this destination's"
    );
    println!(
        "wtpt tag holds D65. Re-predicting lcms2's absolute output with that ONE substitution"
    );
    println!("(plus the geometry) tests whether that is the whole mechanism.");
    println!();
    if let Some(r) = a.runs.iter().find(|r| r.intent == Intent::AbsoluteColorimetric) {
        let (raw_max, raw_mean) = mm(&r.de_end_to_end);
        let (wp_max, wp_mean) = mm(&r.de_wp_policy);
        println!(
            "  unmodelled : max {raw_max:.4e} mean {raw_mean:.4e} dE00   (iccce as shipped vs lcms2)"
        );
        println!(
            "  re-predicted: max {wp_max:.4e} mean {wp_mean:.4e} dE00   (D50 destination white + \
             lcms2 geometry)"
        );
        println!(
            "  shrink      : {:.0}x (max), {:.0}x (mean)",
            raw_max / wp_max,
            raw_mean / wp_mean
        );
        println!(
            "  NOT a verdict on WHICH policy is right: corpus A4b (v2 wtpt semantics) is \
             UNSOURCED and the dispatch is owed."
        );
    }

    println!();
    println!("=== 6. The corner control — 16 exact CLUT nodes ==================");
    println!(
        "At a node every interpolation scheme returns the stored sample, so the method difference"
    );
    println!(
        "is identically zero and what remains is index order, table lookup and the LEGACY Lab"
    );
    println!("decode. This is what makes a 2.0 dE00 gate on the PCS side defensible.");
    println!();
    let corners = pass4::corner_indices(&a.grid);
    for r in &a.runs {
        if r.lcms2_pcs.is_none() {
            continue;
        }
        let sub: Vec<f64> = corners.iter().map(|&i| r.de_pcs[i]).collect();
        let env: Vec<f64> = corners.iter().map(|&i| r.de_method_envelope[i]).collect();
        let (cmax, cmean) = mm(&sub);
        let (emax, _) = mm(&env);
        println!(
            "{:<14} corner dE00 max {:.4e} mean {:.4e}  |  method envelope AT corners {:.3e} \
             (must be ~0 by construction)",
            short_intent(r.intent),
            cmax,
            cmean,
            emax
        );
    }

    println!();
    println!("=== 7. Worst offenders, and lcms2's out-of-range output ==========");
    for r in &a.runs {
        println!();
        println!("--- {} ---", r.intent.name());
        let mut idx: Vec<usize> = (0..a.grid.len()).collect();
        idx.sort_by(|&i, &j| r.de_end_to_end[j].total_cmp(&r.de_end_to_end[i]));
        println!(
            "{:>28} {:>11} {:>11} {:>28}",
            "CMYK (0..1)", "dE00", "envelope", "iccce RGB / lcms2 RGB (0..255)"
        );
        for &i in idx.iter().take(6) {
            let q = a.grid[i];
            println!(
                "{:>7.3}{:>7.3}{:>7.3}{:>7.3} {:>11.4e} {:>11.4e}  {:>7.2}{:>7.2}{:>7.2} /{:>7.2}{:>7.2}{:>7.2}",
                q[0],
                q[1],
                q[2],
                q[3],
                r.de_end_to_end[i],
                r.de_method_end_to_end[i],
                r.iccce_rgb[i][0] * 255.0,
                r.iccce_rgb[i][1] * 255.0,
                r.iccce_rgb[i][2] * 255.0,
                r.lcms2_rgb_255[i][0],
                r.lcms2_rgb_255[i][1],
                r.lcms2_rgb_255[i][2],
            );
        }
        println!(
            "lcms2 components outside [0,1]: {} of {} (max deviation {:.3e}) — README §13.4 / \
             corpus M3, a FINDING about range policy, graded separately from arithmetic",
            r.lcms2_out_of_range.len(),
            a.grid.len() * 3,
            r.lcms2_out_of_range
                .iter()
                .map(|(_, _, v)| if *v > 1.0 { v - 1.0 } else { -v })
                .fold(0.0f64, f64::max)
        );
    }

    println!();
    println!("=== 8. A single point, both geometries, in full ==================");
    println!(
        "The worst method-envelope point at media-relative, so the two schemes can be inspected \
         side by side:"
    );
    if let Some(r) = a.runs.iter().find(|r| r.intent == Intent::RelativeColorimetric) {
        let i = (0..a.grid.len())
            .max_by(|&x, &y| r.de_method_envelope[x].total_cmp(&r.de_method_envelope[y]))
            .expect("non-empty grid");
        let q = a.grid[i];
        println!("  CMYK          {:?}", q);
        println!(
            "  iccce n-linear Lab  ({:.4}, {:.4}, {:.4})",
            r.iccce_pcs[i].l, r.iccce_pcs[i].a, r.iccce_pcs[i].b
        );
        println!(
            "  lcms2 geometry Lab  ({:.4}, {:.4}, {:.4})   [emulated in f64 by this harness]",
            r.emulated_pcs[i].l, r.emulated_pcs[i].a, r.emulated_pcs[i].b
        );
        if let Some(p) = &r.lcms2_pcs {
            println!(
                "  transicc -o*Lab4    ({:.4}, {:.4}, {:.4})   [the oracle itself]",
                p[i].l, p[i].a, p[i].b
            );
        }
        println!(
            "  scheme names: {:?} vs {:?}",
            Scheme::NLinear,
            Scheme::Lcms2Hybrid
        );
    }
}

fn mm(v: &[f64]) -> (f64, f64) {
    if v.is_empty() {
        return (f64::NAN, f64::NAN);
    }
    let max = v.iter().copied().fold(0.0f64, f64::max);
    #[allow(clippy::cast_precision_loss)]
    let mean = v.iter().sum::<f64>() / v.len() as f64;
    (max, mean)
}

fn opt(v: f64) -> String {
    if v.is_nan() {
        "-".to_string()
    } else {
        format!("{v:.4e}")
    }
}

fn short_intent(i: Intent) -> &'static str {
    match i {
        Intent::Perceptual => "perceptual",
        Intent::RelativeColorimetric => "media-relative",
        Intent::Saturation => "saturation",
        Intent::AbsoluteColorimetric => "icc-absolute",
    }
}
