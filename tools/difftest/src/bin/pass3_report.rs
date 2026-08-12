//! # `pass3_report` — the per-point record behind Pass 3's two numbers
//!
//! ```sh
//! cd tools/difftest && cargo run --bin pass3_report
//! ```
//!
//! The suite runner (`cargo run`) emits **seven TSV lines** for Pass 3: five
//! graded comparisons and two reported-only means. That is the right output
//! for a gate. It is the wrong output for the question a human actually asks
//! when a number surprises them, which is **"where, and why?"**
//!
//! This binary answers that. It re-runs exactly the same experiment
//! (`crate::pass3::analyse`) and prints:
//!
//! 1. **Provenance** — which `iccce`, which `transicc`, which profiles.
//! 2. **The reductions**, max and mean together, in both device units and
//!    ΔE2000.
//! 3. **The ten worst points of each comparison**, with their inputs and both
//!    implementations' outputs. A max with no locus is a number nobody can
//!    act on.
//! 4. **The out-of-range report** — every lcms2 output component outside
//!    `[0,1]`, which is §13.4's finding rather than an error.
//! 5. **★ The quantisation experiment** (§4 below) — is lcms2's disagreement
//!    explained by lcms2's own documented approximation?
//! 6. **★ The white-point clamp experiment** (§5 below) — why the round trip
//!    is not the identity, predicted in closed form and checked, plus the
//!    sensitivity control that shows the apparatus can detect the effect.
//!
//! §§4–5 are the reason this is a binary rather than a `--verbose` flag: they
//! **test the justifications** that `pass3.rs`'s tolerances assert, and a
//! justification nobody tested is a justification nobody can defend.
//!
//! ## ★ Why the quantisation experiment is here
//!
//! `pass3::DEVICE_CROSSCHECK`'s justification asserts that the dominant
//! iccce-vs-lcms2 disagreement on this pair is **lcms2's own 16-bit
//! quantisation of tabulated tone curves** (`cmsEvalToneCurveFloat` rounds
//! both the input to and the output of a segment-free curve to 1/65535).
//!
//! An assertion in a tolerance's `why` string is exactly the kind of claim
//! this role exists to distrust, so it is **tested rather than asserted**, by
//! the method the DL-013 finding established (`README.md` §12.4): *when a
//! confound appears, predict it quantitatively from the other
//! implementation's own arithmetic.*
//!
//! The prediction is built by emulating lcms2's evaluation inside iccce's
//! model:
//!
//! ```text
//!   linear_i  =  Q( TRC_src,i ( Q( device_i ) ) )        Q(v) = round(v·65535)/65535
//!   XYZ       =  M_src · linear
//!   out       =  dst.pcs_to_device(XYZ)                   (unchanged)
//! ```
//!
//! and then comparing **that** against lcms2's measured output. Two outcomes
//! are informative and they are not the same:
//!
//! - The residual **collapses** toward `transicc`'s print floor → the model
//!   is confirmed, the tolerance's justification stands, and the disagreement
//!   is attributable to a named approximation in the oracle rather than to
//!   anything of iccce's.
//! - The residual **does not collapse** → the justification is wrong and must
//!   be rewritten before the tolerance is quoted anywhere. It would not mean
//!   the tolerance is wrong; it would mean nobody knows why it is right,
//!   which is the same thing as not having one.
//!
//! **Two limits of the emulation, stated up front so a partial collapse is
//! not over-read.** (i) lcms2 interpolates its 1024-entry table in 16-bit
//! fixed point (`cmsInterpFunction` → `LinLerp1D` with
//! `_cmsQuickSaturateWord`), while this emulation interpolates in `f64` and
//! rounds once at the end; the two can differ by ±1 lsb. (ii) lcms2 carries
//! the pipeline in `cmsFloat32Number` between stages, iccce in `f64`, which
//! is a further ~6×10⁻⁸ of relative noise. So a residual of a few lsb is the
//! *expected* floor of this experiment, not a failure of it.
//!
//! ## Exit codes
//!
//! `0` the analysis ran; `3` it could not (skip — a category (c) profile or
//! the `iccce` binary absent); `2` something errored. **3 is not 0**, for the
//! reason `lib.rs` gives.

use std::io::Write;

use iccce_cmm::{MatrixTrc, Trc};
use iccce_color::{D50, Lab, delta_e_2000};
use iccce_difftest::pass3::{self, ADOBE_RGB, Analysis, SRGB, Unavailable};
use iccce_difftest::{Oracle, Record, Report};
use iccce_profile::Profile;

fn main() {
    let mut out = std::io::stdout().lock();

    let oracle = match Oracle::locate() {
        Err(e) => {
            eprintln!("pass3_report: {e}");
            std::process::exit(2);
        }
        Ok(None) => {
            println!(
                "note\tno transicc found: set ICCCE_TRANSICC, or run fetch-lcms2.sh + \
                 build-lcms2.{{ps1,sh}}"
            );
            println!("summary\tpass=0\tfail=0\tskip=6\terror=0");
            std::process::exit(3);
        }
        Ok(Some(o)) => o,
    };

    let analysis = match pass3::analyse(
        &oracle,
        std::path::Path::new(SRGB),
        std::path::Path::new(ADOBE_RGB),
    ) {
        Ok(a) => a,
        Err(u) => {
            let code = match &u {
                Unavailable::Skip(_) => 3,
                Unavailable::Error(_) => 2,
            };
            let mut report = Report::new();
            for r in pass3::unavailable_records(&u) {
                report.push_record(r);
            }
            report.emit(&mut out).expect("stdout");
            let _ = out.flush();
            std::process::exit(code);
        }
    };

    print_header(&analysis);
    print_reductions(&analysis);
    print_worst(&analysis);
    print_out_of_range(&analysis);
    quantisation_experiment(&analysis);
    white_point_clamp_experiment(&analysis);

    println!("\n=== the seven machine-readable check lines ===\n");
    let mut report = Report::new();
    report.note(format!("oracle: {}", oracle.path().display()));
    report.note(format!("banner: {}", analysis.oracle_banner));
    report.note(format!("iccce:  {}", analysis.iccce_exe.display()));
    for r in pass3::records(&analysis) {
        report.push_record(r);
    }
    report.emit(&mut out).expect("stdout");
    let _ = out.flush();
    std::process::exit(report.exit_code());
}

fn print_header(a: &Analysis) {
    println!("=== Pass 3 — matrix/TRC differential, iccce vs lcms2 ===\n");
    println!("source profile : {}", a.src_path.display());
    println!("dest   profile : {}", a.dst_path.display());
    println!("intent         : media-relative colorimetric (the only one iccce implements)");
    println!("precalc        : -c0 (cmsFLAGS_NOOPTIMIZE) — lcms2's most accurate path");
    println!("bpc            : not requested (and unreachable: media-relative, v2.1 profiles)");
    println!("grid           : {} points, deterministic", a.grid.len());
    println!("iccce binary   : {}", a.iccce_exe.display());
    if a.iccce_is_debug {
        println!("               : ⚠ DEBUG BUILD — not the shipped artefact");
    }
    println!("oracle banner  : {}", a.oracle_banner);
    println!();
}

fn max_mean(v: &[f64]) -> (f64, f64) {
    let max = v.iter().copied().fold(0.0_f64, f64::max);
    #[allow(clippy::cast_precision_loss)]
    let mean = v.iter().sum::<f64>() / v.len() as f64;
    (max, mean)
}

fn print_reductions(a: &Analysis) {
    println!("=== 1. reductions ===\n");
    let (dm, dmean) = max_mean(&a.device_dev_clamped);
    let (drm, drmean) = max_mean(&a.device_dev_raw);
    let (em, emean) = max_mean(&a.de_crosscheck);
    let (rm, rmean) = max_mean(&a.de_roundtrip);
    let (rdm, rdmean) = max_mean(&a.device_roundtrip);
    let (im, imean) = max_mean(&a.de_instrument);

    println!("{:<52} {:>14} {:>14}", "comparison", "max", "mean");
    println!("{}", "-".repeat(82));
    println!(
        "{:<52} {:>14.6e} {:>14.6e}",
        "iccce vs lcms2, device (0..1), lcms2 clamped", dm, dmean
    );
    println!(
        "{:<52} {:>14.6e} {:>14.6e}",
        "  ... same, lcms2 as printed (unclamped)", drm, drmean
    );
    println!(
        "{:<52} {:>14.4} {:>14.4}",
        "  ... same, expressed in 0..255 device units",
        dm * 255.0,
        dmean * 255.0
    );
    println!(
        "{:<52} {:>14.6e} {:>14.6e}",
        "iccce vs lcms2, dE2000 (D50 Lab)", em, emean
    );
    println!(
        "{:<52} {:>14.6e} {:>14.6e}",
        "round trip sRGB->AdobeRGB->sRGB, dE2000", rm, rmean
    );
    println!(
        "{:<52} {:>14.6e} {:>14.6e}",
        "round trip, device (0..1)", rdm, rdmean
    );
    println!(
        "{:<52} {:>14.6e} {:>14.6e}",
        "INSTRUMENT: Lab ruler, iccce vs lcms2, dE2000", im, imean
    );
    println!();
}

/// The `n` largest entries of `v`, as `(index, value)`, descending.
fn worst(v: &[f64], n: usize) -> Vec<(usize, f64)> {
    let mut idx: Vec<(usize, f64)> = v.iter().copied().enumerate().collect();
    idx.sort_by(|x, y| y.1.total_cmp(&x.1));
    idx.truncate(n);
    idx
}

fn print_worst(a: &Analysis) {
    println!("=== 2. worst points ===\n");

    println!("--- device-space disagreement, iccce vs lcms2 (top 10) ---");
    println!(
        "{:>4}  {:<26} {:<26} {:<26} {:>12}",
        "#", "input RGB (0..1)", "iccce out (0..1)", "lcms2 out (0..1)", "max |d|"
    );
    for (i, d) in worst(&a.device_dev_clamped, 10) {
        println!(
            "{:>4}  {:<26} {:<26} {:<26} {:>12.4e}",
            i,
            fmt3(a.grid[i]),
            fmt3(a.iccce_out[i]),
            fmt3(a.lcms2_out[i]),
            d
        );
    }
    println!();

    println!("--- dE2000 disagreement, iccce vs lcms2 (top 10) ---");
    println!(
        "{:>4}  {:<26} {:<26} {:<26} {:>12}",
        "#", "input RGB (0..1)", "iccce out (0..1)", "lcms2 out (0..1)", "dE2000"
    );
    for (i, d) in worst(&a.de_crosscheck, 10) {
        println!(
            "{:>4}  {:<26} {:<26} {:<26} {:>12.4e}",
            i,
            fmt3(a.grid[i]),
            fmt3(a.iccce_out[i]),
            fmt3(a.lcms2_out[i]),
            d
        );
    }
    println!();

    println!("--- round trip sRGB->AdobeRGB->sRGB, iccce alone (top 10 by dE2000) ---");
    println!(
        "{:>4}  {:<26} {:<26} {:>12} {:>12}",
        "#", "input RGB (0..1)", "round-tripped (0..1)", "dE2000", "max |d|dev"
    );
    for (i, d) in worst(&a.de_roundtrip, 10) {
        println!(
            "{:>4}  {:<26} {:<26} {:>12.4e} {:>12.4e}",
            i,
            fmt3(a.grid[i]),
            fmt3(a.roundtrip[i]),
            d,
            a.device_roundtrip[i]
        );
    }
    println!();

    println!("--- INSTRUMENT: Lab of the same device value, iccce's model vs lcms2 (top 10) ---");
    println!(
        "{:>4}  {:<26} {:<30} {:<30} {:>12}",
        "#", "AdobeRGB device (0..1)", "iccce Lab", "lcms2 Lab", "dE2000"
    );
    let dst_model = model(&a.dst_path);
    for (i, d) in worst(&a.de_instrument, 10) {
        let li = dst_model
            .as_ref()
            .map(|m| Lab::from_xyz(m.device_to_pcs(a.iccce_out[i]), D50));
        println!(
            "{:>4}  {:<26} {:<30} {:<30} {:>12.4e}",
            i,
            fmt3(a.iccce_out[i]),
            li.map_or_else(|| "-".into(), fmt_lab),
            fmt_lab(a.lcms2_lab_of_iccce_out[i]),
            d
        );
    }
    println!();
}

fn print_out_of_range(a: &Analysis) {
    println!("=== 3. lcms2 outputs outside [0,1] — a FINDING, not an error ===\n");
    if a.lcms2_out_of_range.is_empty() {
        println!("none.\n");
        return;
    }
    // Counts are ~10^2; the cast is exact in f64.
    #[allow(clippy::cast_precision_loss)]
    let pct = 100.0 * a.lcms2_out_of_range.len() as f64 / (a.grid.len() * 3) as f64;
    println!(
        "{} of {} components ({pct:.2}%) fell outside [0,1].",
        a.lcms2_out_of_range.len(),
        a.grid.len() * 3,
    );
    println!(
        "\nICC.1:2022 Annex F.8-F.16 (NORMATIVE) clamps each linear component to [0,1] BEFORE"
    );
    println!("the inverse TRC. iccce does; lcms2 evidently does not on the high side when the");
    println!("destination TRC's inverse is ANALYTIC (a gamma), because pow(1.0001, 1/g) is");
    println!("perfectly finite and nothing forces it back. Measured 2026-08-11 in the other");
    println!("direction (AdobeRGB -> sRGB, whose inverse TRC is a TABULATED reverse curve),");
    println!("lcms2 DOES saturate — so this is an artefact of which inversion path it takes,");
    println!("not a stated range policy. See README §13.4.\n");
    println!("{:>4} {:>4} {:>16} {:>16}", "#", "chan", "value (0..1)", "excess");
    for &(i, c, v) in a.lcms2_out_of_range.iter().take(20) {
        let excess = if v > 1.0 { v - 1.0 } else { -v };
        println!("{i:>4} {c:>4} {v:>16.8} {excess:>16.4e}");
    }
    if a.lcms2_out_of_range.len() > 20 {
        println!("... {} more", a.lcms2_out_of_range.len() - 20);
    }
    println!();
}

// ===========================================================================
// ★ The quantisation experiment
// ===========================================================================

/// lcms2's 16-bit quantum, applied the way `_cmsQuickSaturateWord` does:
/// round to nearest, saturate into `[0, 65535]`.
fn q16(v: f64) -> f64 {
    (v * 65535.0).round().clamp(0.0, 65535.0) / 65535.0
}

fn model(path: &std::path::Path) -> Option<MatrixTrc> {
    let bytes = std::fs::read(path).ok()?;
    let profile = Profile::parse(&bytes).ok()?;
    MatrixTrc::from_profile(&profile).ok()
}

/// See the module header. Predicts lcms2's output by emulating its 16-bit
/// tabulated-curve evaluation inside iccce's model, then reports how much of
/// the measured disagreement that accounts for.
fn quantisation_experiment(a: &Analysis) {
    println!("=== 4. ★ the quantisation experiment ===\n");
    let (Some(src), Some(dst)) = (model(&a.src_path), model(&a.dst_path)) else {
        println!("skipped: could not rebuild the profile models.\n");
        return;
    };

    // Is the hypothesis even applicable? It applies only to a SAMPLED table
    // (lcms2's `nSegments == 0` case). Assert that rather than assume it —
    // if the source TRCs were parametric, this whole experiment would be
    // measuring nothing and would still print a plausible number.
    let tabulated = src
        .trc
        .iter()
        .all(|t| matches!(t, Trc::Table(v) if v.len() > 1));
    println!(
        "precondition: all three source TRCs are sampled tables (lcms2's nSegments==0 path)? {}",
        if tabulated { "YES" } else { "NO — experiment is inapplicable" }
    );
    if let Trc::Table(t) = &src.trc[0] {
        println!("              source table length: {}", t.len());
    }
    let dst_analytic = dst.trc.iter().all(|t| matches!(t, Trc::Gamma(_)));
    println!(
        "              destination TRCs are analytic gammas (no table quantisation \
         on the way out)? {}",
        if dst_analytic { "YES" } else { "NO" }
    );
    if !tabulated {
        println!("\nnot run.\n");
        return;
    }

    let mut before = Vec::with_capacity(a.grid.len());
    let mut after = Vec::with_capacity(a.grid.len());
    let mut de_before = Vec::with_capacity(a.grid.len());
    let mut de_after = Vec::with_capacity(a.grid.len());

    for (i, x) in a.grid.iter().enumerate() {
        // Emulate lcms2: quantise the curve input, evaluate, quantise the
        // curve output. Everything downstream is iccce's own arithmetic, so
        // the ONLY variable is the quantisation.
        let linear = [
            q16(src.trc[0].eval(q16(x[0]))),
            q16(src.trc[1].eval(q16(x[1]))),
            q16(src.trc[2].eval(q16(x[2]))),
        ];
        let xyz_v = src.matrix().apply(linear);
        let xyz = iccce_color::Xyz {
            x: xyz_v[0],
            y: xyz_v[1],
            z: xyz_v[2],
        };
        let Ok(pred) = dst.pcs_to_device(xyz) else {
            continue;
        };

        let meas = a.lcms2_out[i];
        let icc = a.iccce_out[i];

        // "before" = the real disagreement (iccce as shipped vs lcms2).
        // "after"  = what is LEFT once lcms2's quantisation is modelled.
        // Both compare against the SAME measured lcms2 output, clamped the
        // same way, so the two numbers are commensurable.
        let cl = |v: f64| v.clamp(0.0, 1.0);
        before.push(
            (0..3)
                .map(|c| (icc[c] - cl(meas[c])).abs())
                .fold(0.0_f64, f64::max),
        );
        after.push(
            (0..3)
                .map(|c| (pred[c] - cl(meas[c])).abs())
                .fold(0.0_f64, f64::max),
        );

        let lab = |t: [f64; 3]| Lab::from_xyz(dst.device_to_pcs(t), D50);
        de_before.push(delta_e_2000(lab(icc), lab(meas)));
        de_after.push(delta_e_2000(lab(pred), lab(meas)));
    }

    let (b_max, b_mean) = max_mean(&before);
    let (a_max, a_mean) = max_mean(&after);
    let (dbe_max, dbe_mean) = max_mean(&de_before);
    let (dae_max, dae_mean) = max_mean(&de_after);

    println!("\nresidual against lcms2's MEASURED output:\n");
    println!("{:<46} {:>14} {:>14}", "", "max", "mean");
    println!("{}", "-".repeat(76));
    println!(
        "{:<46} {:>14.6e} {:>14.6e}",
        "device (0..1), iccce as shipped", b_max, b_mean
    );
    println!(
        "{:<46} {:>14.6e} {:>14.6e}",
        "device (0..1), with lcms2's 16-bit quantisation modelled", a_max, a_mean
    );
    println!(
        "{:<46} {:>14.6e} {:>14.6e}",
        "dE2000, iccce as shipped", dbe_max, dbe_mean
    );
    println!(
        "{:<46} {:>14.6e} {:>14.6e}",
        "dE2000, with quantisation modelled", dae_max, dae_mean
    );

    let shrink = if a_max > 0.0 { b_max / a_max } else { f64::INFINITY };
    println!("\ndevice-space max residual shrank by a factor of {shrink:.1}.");
    println!(
        "transicc's print floor for this comparison is 1e-4/255 = {:.3e}; lcms2 carries the \n\
         pipeline in f32 (~6e-8 relative) and interpolates its table in 16-bit fixed point \n\
         (±1 lsb = {:.3e}), so a residual of a few lsb is this experiment's expected floor, \n\
         not a failure of it.",
        1e-4 / 255.0,
        1.0 / 65535.0
    );
    println!(
        "\nreading: {}",
        if shrink >= 3.0 {
            "the quantisation model ACCOUNTS for the bulk of the disagreement — \
             DEVICE_CROSSCHECK's justification stands."
        } else {
            "★ the quantisation model does NOT account for the disagreement. \
             DEVICE_CROSSCHECK's justification is WRONG and must be rewritten before \
             the tolerance is quoted anywhere."
        }
    );
    println!();
}

// ===========================================================================
// ★ The white-point clamp experiment
// ===========================================================================

/// ★ **Why the round trip is not the identity, established rather than
/// assumed.**
///
/// The first run of `DE_ROUNDTRIP` FAILED (observed 1.879×10⁻², tolerance
/// 1×10⁻²), and its `why` string asserted the reason it could not: *"sRGB's
/// gamut is strictly inside Adobe RGB (1998)'s … so nothing is clipped"*.
/// That sentence is true of the two spaces' **nominal chromaticities** and
/// false of the two **files**, which is the distinction this experiment
/// exists to make.
///
/// A matrix/TRC profile's media white is its **colorant sum** `M·(1,1,1)`,
/// and each profile's colorants were authored and rounded to `s15Fixed16`
/// independently — sRGB's by Hewlett-Packard in 1998, Adobe RGB's by Adobe in
/// 2000. They therefore do **not** agree exactly. Where the source white
/// exceeds the destination white in any channel, the source's device white
/// lands *outside* the destination's encoded cube, `pcs_to_device` clamps it
/// per the **normative** F.8–F.16 order, and the clamped-away part cannot be
/// recovered on the way back.
///
/// The prediction below uses **only the two matrices and the clamp** — no
/// TRC (both are exactly 1 at 1), no lcms2, no measurement. If it matches the
/// observed round-trip ΔE at white, the mechanism is established and
/// `DE_ROUNDTRIP` can be re-derived from a fact about the files. If it does
/// not, the failure is something else and the tolerance must not move at all.
///
/// **This is a correction of a justification, not a widening of a number.**
/// The distinction is the whole of `TOLERANCES.md` §0's procedure: step 4
/// ("is the tolerance wrong?") is only reachable after steps 1–3, and it is
/// reached here because the mechanism turns out to be a property of the
/// corpus that the original derivation did not know about.
fn white_point_clamp_experiment(a: &Analysis) {
    println!("=== 5. ★ the white-point clamp experiment ===\n");
    let (Some(src), Some(dst)) = (model(&a.src_path), model(&a.dst_path)) else {
        println!("skipped: could not rebuild the profile models.\n");
        return;
    };

    let sw = src.matrix().apply([1.0, 1.0, 1.0]);
    let dw = dst.matrix().apply([1.0, 1.0, 1.0]);
    println!("media white = colorant sum M*(1,1,1), as ENCODED in each file:\n");
    println!("  source (sRGB)      X={:.8} Y={:.8} Z={:.8}", sw[0], sw[1], sw[2]);
    println!("  dest   (AdobeRGB)  X={:.8} Y={:.8} Z={:.8}", dw[0], dw[1], dw[2]);
    println!(
        "  difference         dX={:+.3e} dY={:+.3e} dZ={:+.3e}",
        sw[0] - dw[0],
        sw[1] - dw[1],
        sw[2] - dw[2]
    );
    println!(
        "\nICC's own 4-figure D50 is X=0.9642 Y=1.0000 Z=0.8249. Neither file's colorant\n\
         sum equals it exactly, and the two files do not equal each other. That is the\n\
         1998/2000 authors' s15Fixed16 rounding — a fact about the FILES, not about iccce."
    );

    // Predict the round trip at device white, using ONLY the matrices and
    // the clamp. TRC(1) = 1 exactly for a table whose last entry is 0xFFFF
    // and for any gamma, so no tone curve enters this prediction.
    let lin_dst = dst
        .matrix()
        .inverse()
        .expect("destination matrix is invertible")
        .apply(sw);
    let clamped = [
        lin_dst[0].clamp(0.0, 1.0),
        lin_dst[1].clamp(0.0, 1.0),
        lin_dst[2].clamp(0.0, 1.0),
    ];
    let back = dst.matrix().apply(clamped);
    println!(
        "\nsource white through M_dst^-1 : R={:.8} G={:.8} B={:.8}",
        lin_dst[0], lin_dst[1], lin_dst[2]
    );
    println!(
        "clamped to [0,1] per F.8-F.16 : R={:.8} G={:.8} B={:.8}",
        clamped[0], clamped[1], clamped[2]
    );
    let clipped: Vec<usize> = (0..3).filter(|&i| lin_dst[i] > 1.0 || lin_dst[i] < 0.0).collect();
    println!(
        "channels actually clipped     : {}",
        if clipped.is_empty() {
            "none".to_string()
        } else {
            format!("{clipped:?}  <-- the original justification said 'none'")
        }
    );

    let lab_src_white = Lab::from_xyz(
        iccce_color::Xyz {
            x: sw[0],
            y: sw[1],
            z: sw[2],
        },
        D50,
    );
    let lab_back = Lab::from_xyz(
        iccce_color::Xyz {
            x: back[0],
            y: back[1],
            z: back[2],
        },
        D50,
    );

    // Recomputed here from the printed intermediates so a reader can follow
    // the arithmetic, then cross-checked against the library function the
    // GRADED record uses. If these two ever disagree, the printed narrative
    // and the pass/fail line have come apart, which is worse than either
    // being wrong.
    let local = delta_e_2000(lab_src_white, lab_back);
    let predicted = pass3::predicted_white_clamp_de(&src, &dst);
    assert!(
        (local - predicted).abs() < 1e-12,
        "the narrative arithmetic and pass3::predicted_white_clamp_de disagree: \
         {local:.12e} vs {predicted:.12e}"
    );
    let white_idx = a
        .grid
        .iter()
        .position(|t| *t == [1.0, 1.0, 1.0])
        .expect("the grid contains device white");
    let observed = a.white_clamp_observed;
    debug_assert!((observed - a.de_roundtrip[white_idx]).abs() < 1e-15);

    println!("\n  Lab of the source white        : {}", fmt_lab(lab_src_white));
    println!("  Lab after clamp and return     : {}", fmt_lab(lab_back));
    println!("  PREDICTED round-trip dE2000    : {predicted:.6e}   (matrices + clamp only)");
    println!("  OBSERVED  round-trip dE2000    : {observed:.6e}   (grid index {white_idx})");
    let rel = if observed > 0.0 {
        (predicted - observed).abs() / observed
    } else {
        f64::INFINITY
    };
    println!("  relative agreement             : {:.2}%", rel * 100.0);
    println!(
        "\nreading: {}",
        if rel < 0.05 {
            "★ MECHANISM ESTABLISHED. The round-trip maximum is the range clamp \
             discarding the two files' encoded white-point difference. iccce is doing what \
             the normative annex requires; the ORIGINAL TOLERANCE'S JUSTIFICATION was wrong \
             (it assumed nothing was clipped), and re-deriving it from this fact is a \
             correction, not a widening."
        } else {
            "the prediction does NOT match. Something other than the white-point clamp is \
             driving the round-trip maximum: DO NOT move the tolerance."
        }
    );

    // ---- the sensitivity control -----------------------------------------
    //
    // "An apparatus not shown able to detect the effect it is looking for is
    // not an experiment" (README §12.4's method note). WHITE_CLAMP_PREDICTION
    // claims it would fail if iccce stopped clamping. Show the number that
    // claim rests on, rather than asserting it: with NO clamping anywhere,
    // M_dst · (M_dst^-1 · W_src) is the identity and the observed cost is
    // exactly zero, so the check's metric would read |predicted - 0|.
    let no_clamp_observed = 0.0_f64;
    let would_read = (predicted - no_clamp_observed).abs();
    println!("\n--- sensitivity control: what if iccce did not clamp at all? ---");
    println!("  round trip without any clamping is the exact identity, so the observed");
    println!("  cost would be {no_clamp_observed:.6e} and the check's metric would read {would_read:.6e},");
    println!(
        "  against a tolerance of {:.1e} — it would FAIL by {:.0}x. The apparatus can",
        iccce_difftest::pass3::WHITE_CLAMP_PREDICTION.value,
        would_read / iccce_difftest::pass3::WHITE_CLAMP_PREDICTION.value
    );
    println!("  detect the effect it is looking for.");
    println!("\n  SCOPE, and it is narrower than it first looks: iccce clamps at THREE");
    println!("  independent sites (F.8-F.16 in pcs_to_device, 10.18's domain in Trc::eval,");
    println!("  F.1(b) in eval_inverse/invert_table). Removing the F.8-F.16 clamp ALONE");
    println!("  changes nothing observable here, because the other two catch it. This");
    println!("  control demonstrates sensitivity to the NET policy, not to the ordering.");

    // How many grid points are clipped at all, and where. A single clipped
    // corner would be a curiosity; a pattern is the thing to characterise.
    let m_inv = dst.matrix().inverse().expect("invertible");
    let mut clipped_points = Vec::new();
    for (i, x) in a.grid.iter().enumerate() {
        let lin_src = [
            src.trc[0].eval(x[0]),
            src.trc[1].eval(x[1]),
            src.trc[2].eval(x[2]),
        ];
        let l = m_inv.apply(src.matrix().apply(lin_src));
        let worst_excess = (0..3)
            .map(|c| {
                if l[c] > 1.0 {
                    l[c] - 1.0
                } else if l[c] < 0.0 {
                    -l[c]
                } else {
                    0.0
                }
            })
            .fold(0.0_f64, f64::max);
        if worst_excess > 0.0 {
            clipped_points.push((i, worst_excess, a.de_roundtrip[i]));
        }
    }
    clipped_points.sort_by(|p, q| q.1.total_cmp(&p.1));
    println!(
        "\n{} of {} grid points are clipped somewhere in the destination's linear space.",
        clipped_points.len(),
        a.grid.len()
    );
    println!("{:>4} {:<26} {:>14} {:>14}", "#", "input RGB", "clamp excess", "round-trip dE");
    for &(i, e, d) in clipped_points.iter().take(12) {
        println!("{:>4} {:<26} {:>14.4e} {:>14.4e}", i, fmt3(a.grid[i]), e, d);
    }
    println!();
}

fn fmt3(t: [f64; 3]) -> String {
    format!("{:.6} {:.6} {:.6}", t[0], t[1], t[2])
}

fn fmt_lab(l: Lab) -> String {
    format!("{:.4} {:.4} {:.4}", l.l, l.a, l.b)
}

/// Unused today, but the type is imported for the `Record` re-export path;
/// keeping the import honest rather than adding an `#[allow]`.
#[allow(dead_code)]
fn _assert_record_type(_r: Record) {}
