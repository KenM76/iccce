//! Scratch probe for Pass 5c — reimplementing lcms2's
//! `cmsDetectDestinationBlackPoint` from the pinned source and comparing it
//! against ISO/CD 18619 4.2.5 on the same round trip.
//!
//! Exploratory only; the graded apparatus is `src/pass5c.rs`.

use std::path::Path;

use iccce_difftest::{Oracle, pass5c};

fn main() {
    let swop = Path::new(r"C:\Windows\System32\spool\drivers\color\USWebCoatedSWOP.icc");
    let synth = Path::new("D:/Dev/iccce/fixtures/synthetic/v4-rgb-mab-chromatic-black.icc");
    for f in [swop, synth] {
        match pass5c::probe(f) {
            Ok(text) => println!("{text}"),
            Err(e) => eprintln!("probe failed: {e}"),
        }
    }

    let oracle = match Oracle::locate() {
        Ok(Some(o)) => o,
        Ok(None) => {
            eprintln!("no oracle");
            return;
        }
        Err(e) => {
            eprintln!("oracle: {e}");
            return;
        }
    };
    for (arm, path) in [("swop", swop.to_path_buf()), ("synthetic", synth.to_path_buf())] {
    println!("===== ARM {arm} =====");
    match pass5c::analyse(&oracle, arm, &path) {
        Err(u) => eprintln!("unavailable: {u}"),
        Ok(a) => {
            println!("--- section B ---");
            println!("observed transicc CMYK    : {:?}", a.observed_device);
            println!("predicted from lcms2 black: {:?}", a.predicted_from_lcms2);
            println!("predicted from ISO   black: {:?}", a.predicted_from_iso);
            println!("shipped iccce --bpc  CMYK : {:?}", a.shipped_device);
            println!("shipped error             : {:?}", a.shipped_error);
            println!(
                "device residual  lcms2 = {:.9}   ISO = {:.9}",
                a.device_residual_lcms2(),
                a.device_residual_iso()
            );
            println!(
                "sensitivity d(dev)/d(L*)  = {:.9}  -> L* bound {:.6}",
                a.sensitivity,
                a.l_star_bound()
            );
            println!(
                "estimator divergence: {:.6} dE76  (dL* {:.6}, chroma {:.6})",
                a.estimator_divergence_de76(),
                a.divergence_lightness(),
                a.divergence_chroma()
            );
            println!(
                "pass5b recovered black    : L* {:.6} a* {:.6} b* {:.6}",
                a.pass5b_recovered.l, a.pass5b_recovered.a, a.pass5b_recovered.b
            );
            println!(
                "roundtrip of reimpl black : L* {:.6} a* {:.6} b* {:.6}  -> agree to {:.6} dE76",
                a.roundtrip_of_reimpl.l,
                a.roundtrip_of_reimpl.a,
                a.roundtrip_of_reimpl.b,
                a.recovery_explained()
            );
        }
    }
}
}
