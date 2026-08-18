//! # `passk_probe` — print the RAMP, which a graded record cannot
//!
//! A [`iccce_difftest::Record`] carries **one number**. That is right for a
//! gate and wrong for this subject, because the whole finding in Pass K §A is
//! that the contamination of a K-only build **is not uniform**: it peaks in
//! chromatic ink at one end of the ramp and in *black loss* in the middle, and
//! a single reduced maximum hides both facts. This binary prints the shape.
//!
//! It is an **instrument, not a grader**. It has no tolerances, it decides
//! nothing, and its exit code is `0` whenever it managed to measure and `1`
//! when it did not. Everything it prints is also reduced into a graded or
//! reported row by `passk::run`; nothing here is a claim that exists only in
//! this file.
//!
//! ## Usage
//!
//! ```text
//! cargo run --release --bin passk_probe
//! ```
//!
//! Requires:
//!
//! * the pinned `transicc` (`$ICCCE_TRANSICC` or the vendored build), and
//! * the shipped `iccce` binary (`$ICCCE_BIN` or `target/release/iccce`), and
//! * the **licensed** Ghent v5.0 corpus via `$ICCCE_PRIVATE_FIXTURES`.
//!
//! Absent any of them it says which one and exits `1`. It never falls back to
//! a substitute fixture: a ramp printed from a different profile would look
//! exactly as convincing and mean something else.
//!
//! ## What the six blocks are
//!
//! 1. **The baseline ramp**, media-relative, `ISO Coated v2 300% (ECI)` into
//!    itself — the block the whole feature is being built to change.
//! 2. **The same ramp at the other three ICC intents**, reduced to one line
//!    each, because the saturation line is startling and belongs beside the
//!    others rather than in a footnote.
//! 3. **What a K-preserving answer looks like**, from lcms2's **non-ICC**
//!    intent 11. Every line of that block is an *implementation cross-check*
//!    against a **vendor extension outside the ICC intent numbering**; the
//!    block prints that sentence itself so a pasted excerpt carries it.
//! 4. **The transition**, walking cyan across one cell of lcms2's 17-node
//!    black-preserving CLUT. This is the block that shows preservation is an
//!    *edge* of a table and not a rule about neutrality.
//! 5. **The GWG two-leg comparison**, in device units and in ΔE2000, for two
//!    gray sources — the favourable one and the unfavourable one.
//! 6. **The intent sweep** over six real CMYK destinations.

use std::process::ExitCode;

use iccce_color::delta_e_2000;
use iccce_difftest::passk::{
    self, CELL, KOnlyOracle, as_cmyk, cell_ramp, corpus_dir, gray_ramp, k_ramp, max_chromatic,
    max_dev, max_tac, to_lab,
};
use iccce_difftest::{Bpc, Iccce, Intent, Oracle, Precalc, Request, Space};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("passk_probe: {e}");
            ExitCode::from(1)
        }
    }
}

#[expect(
    clippy::too_many_lines,
    reason = "this is a report generator: splitting it would scatter the printed layout across \
              functions and make the output harder to keep aligned with the rows it mirrors"
)]
fn run() -> Result<(), String> {
    let oracle = Oracle::locate()
        .map_err(|e| e.to_string())?
        .ok_or("the pinned transicc was not found — set $ICCCE_TRANSICC")?;
    let iccce = Iccce::locate()
        .map_err(|e| e.to_string())?
        .ok_or("the shipped iccce binary was not found — `cargo build --release -p iccce-cli`")?;
    let dir = corpus_dir();
    if !dir.is_dir() {
        return Err(format!(
            "the Ghent v5.0 corpus is not at {} — set $ICCCE_PRIVATE_FIXTURES. It is licensed \
             and cannot be committed, so absence is the normal case",
            dir.display()
        ));
    }
    let dst = dir.join("c6b4b62f07262437.icc"); // ISO Coated v2 300% (ECI)
    if !dst.is_file() {
        return Err(format!("corpus member missing: {}", dst.display()));
    }

    println!("passk_probe — black preservation, measured on the shipped engine");
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
    println!("  fixture: ISO Coated v2 300% (ECI), Ghent v5.0 (licensed, uncommittable)");
    println!();

    let ramp = k_ramp();
    let rows: Vec<Vec<f64>> = ramp.iter().map(|r| r.to_vec()).collect();
    let lab_konly = to_lab(&oracle, &dst, &ramp).map_err(|e| format!("{e:?}"))?;

    // ---- block 1: the baseline ramp ------------------------------------
    let mine = as_cmyk(
        iccce
            .transform_rows_shaped(&dst, &dst, Intent::RelativeColorimetric, &rows, 4)
            .map_err(|e| e.to_string())?,
    )
    .map_err(|e| format!("{e:?}"))?;
    let lab_mine = to_lab(&oracle, &dst, &mine).map_err(|e| format!("{e:?}"))?;

    println!("=== 1. THE BASELINE — (0,0,0,K) -> ISO Coated v2 300% (ECI), media-relative ===");
    println!("    the input carries NO chromatic ink at any point of this table");
    println!();
    println!(
        "    K_in       C        M        Y        K     max CMY     TAC      dK      dE2000"
    );
    for (i, (inp, out)) in ramp.iter().zip(&mine).enumerate() {
        println!(
            "    {:.3}   {:8.6} {:8.6} {:8.6} {:8.6}  {:8.6} {:7.4} {:+8.4}  {:7.4}",
            inp[3],
            out[0],
            out[1],
            out[2],
            out[3],
            out[0].max(out[1]).max(out[2]),
            out.iter().sum::<f64>(),
            out[3] - inp[3],
            delta_e_2000(lab_konly[i], lab_mine[i])
        );
    }
    println!();
    println!(
        "    max chromatic ink {:.6}   max TAC {:.6} (input TAC never exceeds 1.000000)",
        max_chromatic(&mine),
        max_tac(&mine)
    );
    println!(
        "    max |dK| {:.6}          max dE2000 from the K-only build {:.6}",
        mine.iter()
            .zip(&ramp)
            .map(|(o, r)| (o[3] - r[3]).abs())
            .fold(0.0_f64, f64::max),
        lab_konly
            .iter()
            .zip(&lab_mine)
            .map(|(a, b)| delta_e_2000(*a, *b))
            .fold(0.0_f64, f64::max)
    );
    println!(
        "    ** READ THOSE TWO LINES TOGETHER: the ink is wrong by 0.7 of a plate and the \
         COLOUR is right to a"
    );
    println!(
        "       fraction of the perceptibility threshold. No dE-based test can be the \
         instrument for this. **"
    );
    println!();

    // ---- block 2: the other three ICC intents ---------------------------
    println!("=== 2. THE SAME RAMP AT THE OTHER ICC INTENTS ===");
    for intent in [
        Intent::RelativeColorimetric,
        Intent::Perceptual,
        Intent::Saturation,
        Intent::AbsoluteColorimetric,
    ] {
        let out = as_cmyk(
            iccce
                .transform_rows_shaped(&dst, &dst, intent, &rows, 4)
                .map_err(|e| e.to_string())?,
        )
        .map_err(|e| format!("{e:?}"))?;
        let lab = to_lab(&oracle, &dst, &out).map_err(|e| format!("{e:?}"))?;
        println!(
            "    {:<16} max CMY {:8.6}   max TAC {:7.4}   max |dK| {:8.6}   max dE2000 {:7.4}",
            intent.name(),
            max_chromatic(&out),
            max_tac(&out),
            out.iter()
                .zip(&ramp)
                .map(|(o, r)| (o[3] - r[3]).abs())
                .fold(0.0_f64, f64::max),
            lab_konly
                .iter()
                .zip(&lab)
                .map(|(a, b)| delta_e_2000(*a, *b))
                .fold(0.0_f64, f64::max)
        );
    }
    println!(
        "    ** saturation is nearly K-only ON THIS VENDOR'S PROFILES ONLY — see block 6. **"
    );
    println!();

    // ---- block 3: what a K-preserving answer looks like ------------------
    let k = KOnlyOracle::new(&oracle);
    let preserved = k
        .convert_cmyk(&dst, &dst, KOnlyOracle::PRESERVE_K_ONLY_RELATIVE, &ramp)
        .map_err(|e| e.to_string())?;
    println!("=== 3. WHAT A K-PRESERVING ANSWER LOOKS LIKE ===");
    println!("    {}", KOnlyOracle::CAVEAT);
    println!();
    println!("    K_in       C        M        Y        K       K_out - K_in");
    for (inp, out) in ramp.iter().zip(&preserved).step_by(4) {
        println!(
            "    {:.3}   {:8.6} {:8.6} {:8.6} {:8.6}    {:+.6}",
            inp[3],
            out[0],
            out[1],
            out[2],
            out[3],
            out[3] - inp[3]
        );
    }
    println!(
        "    chromatic ink over the whole ramp: {:.6e}  (exactly zero — which is what makes a \
         tolerance of exactly zero defensible)",
        max_chromatic(&preserved)
    );
    for (name, s, d) in [
        ("-> itself", "c6b4b62f07262437.icc", "c6b4b62f07262437.icc"),
        (
            "-> Coated FOGRA39",
            "c6b4b62f07262437.icc",
            "da2b9b593e27cba2.icc",
        ),
        (
            "-> Coated FOGRA27",
            "c6b4b62f07262437.icc",
            "fb710c05e3fb5a96.icc",
        ),
        (
            "-> GWG_GenericCMYK",
            "c6b4b62f07262437.icc",
            "5bad92a6f018e726.icc",
        ),
    ] {
        let got = k
            .convert_cmyk(
                &dir.join(s),
                &dir.join(d),
                KOnlyOracle::PRESERVE_K_ONLY_RELATIVE,
                &ramp,
            )
            .map_err(|e| e.to_string())?;
        println!(
            "    K is RE-MAPPED, not copied: ISO Coated v2 300% {:<20} max |K_out - K_in| = \
             {:.6}",
            name,
            got.iter()
                .zip(&ramp)
                .map(|(o, r)| (o[3] - r[3]).abs())
                .fold(0.0_f64, f64::max)
        );
    }
    println!();

    // ---- block 4: the transition ---------------------------------------
    let cell = cell_ramp();
    let cell_rows: Vec<Vec<f64>> = cell.iter().map(|r| r.to_vec()).collect();
    let cell_pres = k
        .convert_cmyk(&dst, &dst, KOnlyOracle::PRESERVE_K_ONLY_RELATIVE, &cell)
        .map_err(|e| e.to_string())?;
    let cell_mine = as_cmyk(
        iccce
            .transform_rows_shaped(&dst, &dst, Intent::RelativeColorimetric, &cell_rows, 4)
            .map_err(|e| e.to_string())?,
    )
    .map_err(|e| format!("{e:?}"))?;
    println!("=== 4. THE TRANSITION — C from 0 to one CLUT cell ({CELL:.6}), M=Y=0, K=0.5 ===");
    println!(
        "    the K-only answer is an EDGE of a 17-node CLUT, not a rule about neutrality;"
    );
    println!("    it decays LINEARLY to the colorimetric answer over exactly one cell.");
    println!();
    println!(
        "       C           lcms2 t11 (K-only)                       iccce today \
         (media-relative)"
    );
    for i in (0..cell.len()).step_by(4) {
        let (p, m) = (cell_pres[i], cell_mine[i]);
        println!(
            "    {:9.6}  {:8.6} {:8.6} {:8.6} {:8.6}   {:8.6} {:8.6} {:8.6} {:8.6}",
            cell[i][0], p[0], p[1], p[2], p[3], m[0], m[1], m[2], m[3]
        );
    }
    // The claim "at one cell the two answers coincide" is about the ORACLE's
    // two answers, so both sides of it are measured from the oracle. Putting
    // iccce on one side would turn a statement about lcms2's grid into a
    // statement about the engine, which is a different claim entirely.
    let req = Request {
        input: Space::profile(&dst),
        output: Space::profile(&dst),
        intent: Intent::RelativeColorimetric,
        precalc: Precalc::Exact,
        bpc: Bpc::Off,
        values: cell.iter().flatten().map(|v| v * 100.0).collect(),
    };
    let colorimetric: Vec<[f64; 4]> = as_cmyk(
        oracle
            .convert_batch_shaped(&req, 4, 4)
            .map_err(|e| e.to_string())?,
    )
    .map_err(|e| format!("{e:?}"))?
    .into_iter()
    .map(|r| [r[0] / 100.0, r[1] / 100.0, r[2] / 100.0, r[3] / 100.0])
    .collect();
    println!(
        "    at C = {:.6} exactly, the oracle's K-only and colorimetric answers are the SAME \
         table entry (difference {:.6e})",
        CELL,
        colorimetric
            .last()
            .zip(cell_pres.last())
            .map_or(f64::NAN, |(a, b)| a
                .iter()
                .zip(b)
                .map(|(p, q)| (p - q).abs())
                .fold(0.0_f64, f64::max))
    );
    println!();

    // ---- block 5: the GWG two-leg comparison ----------------------------
    println!("=== 5. GWG 'four different grays' — BOTH LEGS ===");
    println!(
        "    LEG P = ISO 32000-1 §10.3.3 device rule (c=m=y=0, k=1-gray) — the PDF \
         consumer's, not ours"
    );
    println!("    LEG I = iccce's ICC leg: gray profile -> PCS -> ISO Coated v2 300% (ECI)");
    println!();
    let gs = gray_ramp();
    let leg_p: Vec<[f64; 4]> = gs.iter().map(|g| [0.0, 0.0, 0.0, 1.0 - g]).collect();
    let lab_p = to_lab(&oracle, &dst, &leg_p).map_err(|e| format!("{e:?}"))?;
    for (label, src) in [
        (
            "Schwarze Druckfarbe (the press's OWN black ink)",
            dir.join("5dae7984654a2c9f.icc"),
        ),
        (
            "synthetic gamma-2.2 gray (an ordinary one)     ",
            passk::synthetic_dir().join("v2-gray-curv-gamma.icc"),
        ),
    ] {
        if !src.is_file() {
            println!("    {label}: MISSING ({})", src.display());
            continue;
        }
        let grows: Vec<Vec<f64>> = gs.iter().map(|g| vec![*g]).collect();
        let leg_i = as_cmyk(
            iccce
                .transform_rows_shaped(&src, &dst, Intent::RelativeColorimetric, &grows, 4)
                .map_err(|e| e.to_string())?,
        )
        .map_err(|e| format!("{e:?}"))?;
        let lab_i = to_lab(&oracle, &dst, &leg_i).map_err(|e| format!("{e:?}"))?;
        println!("    source: {label}");
        println!(
            "       g        LEG I  C        M        Y        K     |  LEG P K   |  dE2000"
        );
        for i in (0..gs.len()).step_by(4) {
            println!(
                "     {:.2}         {:8.6} {:8.6} {:8.6} {:8.6}  |  {:8.6}  |  {:7.4}",
                gs[i],
                leg_i[i][0],
                leg_i[i][1],
                leg_i[i][2],
                leg_i[i][3],
                leg_p[i][3],
                delta_e_2000(lab_i[i], lab_p[i])
            );
        }
        println!(
            "       max |Δ| device between the legs {:.6}   max dE2000 {:.6}",
            max_dev(&leg_i, &leg_p),
            lab_i
                .iter()
                .zip(&lab_p)
                .map(|(a, b)| delta_e_2000(*a, *b))
                .fold(0.0_f64, f64::max)
        );
        println!();
    }

    // ---- block 6: the intent sweep --------------------------------------
    println!("=== 6. THE INTENT SWEEP — six REAL CMYK destinations, max chromatic ink ===");
    println!(
        "    destination                       media-rel  perceptual  saturation   sat dE2000"
    );
    for (name, f) in [
        ("ISO Coated v2 300% (ECI)", "c6b4b62f07262437.icc"),
        ("ISO Coated v2 (ECI)", "128dc02f7246cc38.icc"),
        ("Coated FOGRA39", "da2b9b593e27cba2.icc"),
        ("Coated FOGRA27", "fb710c05e3fb5a96.icc"),
        ("GWG_GenericCMYK", "5bad92a6f018e726.icc"),
        ("GWG_ICC_v4_testprofile (X-Rite)", "b5988983b6b3b7d4.icc"),
    ] {
        let p = dir.join(f);
        if !p.is_file() {
            println!("    {name:<33}  MISSING");
            continue;
        }
        let lab_ref = to_lab(&oracle, &p, &ramp).map_err(|e| format!("{e:?}"))?;
        let mut v = [0.0_f64; 3];
        let mut de = 0.0_f64;
        for (slot, intent) in [
            Intent::RelativeColorimetric,
            Intent::Perceptual,
            Intent::Saturation,
        ]
        .into_iter()
        .enumerate()
        {
            let out = as_cmyk(
                iccce
                    .transform_rows_shaped(&p, &p, intent, &rows, 4)
                    .map_err(|e| e.to_string())?,
            )
            .map_err(|e| format!("{e:?}"))?;
            v[slot] = max_chromatic(&out);
            if intent == Intent::Saturation {
                let lab = to_lab(&oracle, &p, &out).map_err(|e| format!("{e:?}"))?;
                de = lab_ref
                    .iter()
                    .zip(&lab)
                    .map(|(a, b)| delta_e_2000(*a, *b))
                    .fold(0.0_f64, f64::max);
            }
        }
        println!(
            "    {:<33} {:9.6}  {:9.6}   {:9.6}    {:8.4}",
            name, v[0], v[1], v[2], de
        );
    }
    println!();
    println!(
        "    ** 'use the saturation intent' works on two of six, and both are the same \
         vendor's. **"
    );

    Ok(())
}
