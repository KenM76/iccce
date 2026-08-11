//! # `pass4b_report` — the per-point record for Pass 4b, and the experiments
//!
//! ```text
//! cd tools/difftest && cargo run --bin pass4b_report
//! ```
//!
//! `cargo run` (the main runner) emits Pass 4b's graded TSV lines. This binary
//! prints what those lines are a *reduction of*, plus the experiments that test
//! whether the tolerances' justifications are true rather than merely stated.
//!
//! Sections:
//!
//! 1. Provenance — which binaries, which profiles, which structures.
//! 2. **§A, the B2A direction** — per-intent maxima, the worst points in full,
//!    the quantisation envelope, and **the counterfactual**: what the
//!    disagreement would have been had `_cmsReadOutputLUT` not forced trilinear
//!    interpolation for a Lab-PCS LUT. That ratio is the sensitivity control:
//!    it says the comparison *can* see a geometry difference, which is the only
//!    thing that makes "the geometries agree here" worth reporting.
//! 3. **§B, the v4 fixture** — the closed forms against both implementations,
//!    the affine check that licenses them, and the encoded-PCS-overflow
//!    divergence in full.
//! 4. **§C, the gray axis** — the disagreement along the axis, and the reverse
//!    tone curve experiment that attributes it.
//!
//! Exit code mirrors the runner's contract: 3 when nothing ran.

use std::io::Write;

use iccce_difftest::{Oracle, pass4b};

fn main() {
    let mut out = std::io::stdout().lock();
    let oracle = match Oracle::locate() {
        Err(e) => {
            let _ = writeln!(out, "oracle error: {e}");
            std::process::exit(2);
        }
        Ok(None) => {
            let _ = writeln!(out, "no oracle on this machine — nothing ran");
            std::process::exit(3);
        }
        Ok(Some(o)) => o,
    };

    println!("=== 1. provenance ===\n");
    println!("oracle:  {}", oracle.path().display());
    println!("banner:  {}", oracle.banner().unwrap_or_default());
    println!("fixture: {}", pass4b::fixture_path().display());
    println!();

    let (bundle, records) = pass4b::run(&oracle);

    // -- §A ----------------------------------------------------------------
    println!("=== 2. section A — the B2A direction (sRGB -> SWOP, mft1/lut8) ===\n");
    match &bundle.b2a {
        None => println!("not run.\n"),
        Some(a) => {
            println!("structure: {}", a.structure);
            println!(
                "grid: {} RGB points end-to-end, {} Lab points PCS-side",
                a.rgb_grid.len(),
                a.lab_grid.len()
            );
            println!(
                "CLUT max adjacent-node step (per output channel): {:.6}\n",
                a.max_step
            );
            for r in &a.runs {
                let (dev, dev_mean) = mm(&r.device_dev);
                let (modelled, _) = mm(&r.device_dev_modelled);
                let (env, env_mean) = mm(&r.envelope);
                let (de, de_mean) = mm(&r.de_roundtrip);
                let (cf, cf_mean) = mm(&r.counterfactual);
                println!("--- intent {} ---", r.intent.name());
                println!("  apparatus (harness lut8 vs iccce-cmm)   {:.6e}", r.apparatus);
                println!("  device max / mean                       {dev:.6e} / {dev_mean:.6e}");
                println!("  device, lcms2 arithmetic modelled       {modelled:.6e}");
                println!("  quantisation envelope max / mean        {env:.6e} / {env_mean:.6e}");
                println!("  round-trip through A2B1, dE00 max/mean  {de:.6e} / {de_mean:.6e}");
                println!("  COUNTERFACTUAL tetrahedral max / mean   {cf:.6e} / {cf_mean:.6e}");
                if dev > 0.0 {
                    println!(
                        "  ratio counterfactual : observed         {:.0}x  <- the sensitivity \
                         control: this is what a geometry difference would look like",
                        cf / dev
                    );
                }
                if modelled > 0.0 {
                    println!(
                        "  attribution shrink (observed/modelled)  {:.1}x",
                        dev / modelled
                    );
                }
                // The worst point, in full.
                let (i, _) = argmax(&r.device_dev);
                let t = a.rgb_grid[i];
                println!(
                    "  worst point RGB ({:.5}, {:.5}, {:.5}):",
                    t[0], t[1], t[2]
                );
                println!(
                    "    iccce   CMYK {:?}",
                    r.iccce_cmyk[i].iter().map(r6).collect::<Vec<_>>()
                );
                println!(
                    "    lcms2   CMYK {:?}",
                    r.lcms2_cmyk_100[i]
                        .iter()
                        .map(|v| r6(&(v / 100.0)))
                        .collect::<Vec<_>>()
                );
                println!(
                    "    modelled CMYK {:?}   [lcms2's arithmetic, this harness]",
                    r.modelled_cmyk[i].iter().map(r6).collect::<Vec<_>>()
                );
                println!(
                    "    exact    CMYK {:?}   [f64 throughout, this harness]",
                    r.exact_cmyk[i].iter().map(r6).collect::<Vec<_>>()
                );
                println!();
            }
            let (p, pm) = mm(&a.pcs_device_dev);
            let (pmod, _) = mm(&a.pcs_device_dev_modelled);
            let (pcf, _) = mm(&a.pcs_counterfactual);
            println!("--- PCS side: Lab -> SWOP B2A1 (iccce IN-PROCESS) ---");
            println!("  device max / mean                       {p:.6e} / {pm:.6e}");
            println!("  device, lcms2 arithmetic modelled       {pmod:.6e}");
            println!("  COUNTERFACTUAL tetrahedral              {pcf:.6e}");
            println!();
        }
    }

    // -- §B ----------------------------------------------------------------
    println!("=== 3. section B — the synthetic v4 fixture (mAB and mBA) ===\n");
    match &bundle.mab {
        None => println!("not run.\n"),
        Some(m) => {
            println!("structure: {}", m.structure);
            println!(
                "affine check: both geometries on the fixture's own CLUTs differ by {:.3e} \
                 (must be exactly 0 — it is what licenses the closed forms)\n",
                m.scheme_envelope
            );
            let n_over = m.mab_overflows.iter().filter(|o| **o).count();
            let keep: Vec<bool> = m.mab_overflows.iter().map(|o| !o).collect();
            println!("--- mAB: CMYK -> Lab, {} points ---", m.cmyk_grid.len());
            println!(
                "  iccce vs derived (clamped reading)      {:.6e}   [max |dL*|,|da*|,|db*|, \
                 excluding the {n_over} overflow points]",
                max_at(&m.mab_iccce_vs_derived, &keep)
            );
            println!(
                "  lcms2 vs derived (unclamped reading)    {:.6e}",
                mm(&m.mab_lcms2_vs_derived).0
            );
            println!(
                "  iccce vs lcms2                          {:.6e}  (all points) / {:.6e}  \
                 (excluding overflow)",
                mm(&m.mab_cross).0,
                max_at(&m.mab_cross, &keep)
            );
            println!("--- mBA: Lab -> CMYK, {} points ---", m.lab_grid.len());
            println!(
                "  iccce vs derived                        {:.6e}",
                mm(&m.mba_iccce_vs_derived).0
            );
            println!(
                "  lcms2 vs derived                        {:.6e}",
                mm(&m.mba_lcms2_vs_derived).0
            );
            if !m.e2e_mab_device.is_empty() {
                println!("--- end to end, shipped binary vs transicc ---");
                println!(
                    "  sRGB -> fixture (mBA), device max       {:.6e}",
                    mm(&m.e2e_mba_device).0
                );
                println!(
                    "  fixture -> sRGB (mAB), device max       {:.6e}  (all) / {:.6e}  \
                     (excluding overflow)",
                    mm(&m.e2e_mab_device).0,
                    max_at(&m.e2e_mab_device, &keep)
                );
                println!(
                    "  fixture -> sRGB (mAB), dE00 max         {:.6e}  (all) / {:.6e}  \
                     (excluding overflow)",
                    mm(&m.e2e_mab_de).0,
                    max_at(&m.e2e_mab_de, &keep)
                );
                println!(
                    "\n  ★ ENCODED-PCS-OVERFLOW DIVERGENCE: {:.4} dE2000 over {n_over} points.",
                    m.clamp_divergence
                );
                println!(
                    "    At K = 0 the mAB matrix's +1/256 offset puts the encoded L* at \
                     1.00390625."
                );
                println!(
                    "    iccce clamps it (clause 10.18's [0,1] curve domain, in Trc::eval) -> \
                     L* = 100."
                );
                println!(
                    "    lcms2 does not (identity curve = analytic gamma-1 segment) -> \
                     L* = 100.390625."
                );
                println!(
                    "    NOT GRADED: which the specification requires is unsettled. See \
                     README section 15.3.\n"
                );
                println!(
                    "  ★ FORCED BPC (DL-013 / corpus M2) MEASURED IN BOTH DIRECTIONS, device units:"
                );
                println!(
                    "    v4 fixture as SOURCE, v2 destination:   {:.4e}   <- the forced flag \
                     is never read",
                    m.forced_bpc_cost.0
                );
                println!(
                    "    v2 source, v4 fixture as DESTINATION:   {:.4e}",
                    m.forced_bpc_cost.1
                );
                println!(
                    "    _cmsLinkProfiles sets BPC[i] per profile; DefaultICCintents consumes it as"
                );
                println!(
                    "    ComputeConversion(i, .., BPC[i], ..) — the conversion INTO hProfiles[i] \
                     — so the"
                );
                println!("    DESTINATION profile's version decides. BOTH SIDES ARE lcms2.\n");
            }
        }
    }

    // -- §C ----------------------------------------------------------------
    println!("=== 4. section C — the gray axis (ewgray22 -> sRGB, Annex F.2) ===\n");
    match &bundle.gray {
        None => println!("not run.\n"),
        Some(g) => {
            println!("structure: {}", g.structure);
            let (dev, dev_mean) = mm(&g.device_dev);
            let (modelled, _) = mm(&g.device_dev_modelled);
            let (de, de_mean) = mm(&g.de);
            let (env, env_mean) = mm(&g.envelope);
            println!("  points                                  {}", g.axis.len());
            println!("  device max / mean                       {dev:.6e} / {dev_mean:.6e}");
            println!("  dE2000 max / mean                       {de:.6e} / {de_mean:.6e}");
            println!("  envelope (exact vs modelled destination) {env:.6e} / {env_mean:.6e}");
            println!("  device, lcms2 destination modelled      {modelled:.6e}");
            if modelled > 0.0 {
                println!("  attribution shrink                      {:.1}x", dev / modelled);
            }
            println!(
                "  perceptual vs media-relative: iccce {:.3e}, lcms2 {:.3e}",
                g.intent_identity_iccce, g.intent_identity_lcms2
            );
            let (i, _) = argmax(&g.device_dev);
            println!(
                "\n  worst point g = {:.6}:\n    iccce    {:?}\n    lcms2    {:?}\n    \
                 modelled {:?}",
                g.axis[i],
                g.iccce_rgb[i].iter().map(r6).collect::<Vec<_>>(),
                g.lcms2_rgb_255[i]
                    .iter()
                    .map(|v| r6(&(v / 255.0)))
                    .collect::<Vec<_>>(),
                g.modelled_rgb[i].iter().map(r6).collect::<Vec<_>>()
            );
            println!();
        }
    }

    // -- the records -------------------------------------------------------
    println!("=== 5. the graded records, as the runner emits them ===\n");
    let mut n_pass = 0;
    let mut n_fail = 0;
    for r in &records {
        println!(
            "{:<62} {:<8} {:<20} tol={:<10} obs={}",
            r.id,
            r.outcome.tag(),
            r.kind.tag(),
            fmt(r.tolerance.value),
            match &r.outcome {
                iccce_difftest::Outcome::Pass { observed, .. }
                | iccce_difftest::Outcome::Fail { observed, .. } => format!("{observed:.6e}"),
                _ => "-".into(),
            }
        );
        match r.outcome {
            iccce_difftest::Outcome::Pass { .. } => n_pass += 1,
            iccce_difftest::Outcome::Fail { .. } => n_fail += 1,
            _ => {}
        }
    }
    println!("\npass={n_pass} fail={n_fail} of {} records", records.len());
    let _ = out.flush();
    if n_fail > 0 {
        std::process::exit(1);
    }
    if n_pass == 0 {
        std::process::exit(3);
    }
}

fn mm(v: &[f64]) -> (f64, f64) {
    let f: Vec<f64> = v.iter().copied().filter(|x| x.is_finite()).collect();
    if f.is_empty() {
        return (f64::NAN, f64::NAN);
    }
    #[allow(clippy::cast_precision_loss)]
    let n = f.len() as f64;
    (
        f.iter().copied().fold(0.0f64, f64::max),
        f.iter().sum::<f64>() / n,
    )
}

fn max_at(v: &[f64], keep: &[bool]) -> f64 {
    v.iter()
        .zip(keep)
        .filter(|(x, k)| **k && x.is_finite())
        .map(|(x, _)| *x)
        .fold(0.0f64, f64::max)
}

fn argmax(v: &[f64]) -> (usize, f64) {
    let mut bi = 0;
    let mut bv = f64::NEG_INFINITY;
    for (i, &x) in v.iter().enumerate() {
        if x.is_finite() && x > bv {
            bv = x;
            bi = i;
        }
    }
    (bi, bv)
}

fn r6(v: &f64) -> String {
    format!("{v:.6}")
}

fn fmt(v: f64) -> String {
    if v.is_infinite() {
        "inf".into()
    } else {
        format!("{v:.1e}")
    }
}
