//! # `ghent_probe` — exploratory measurement over the Ghent v5.0 profile corpus
//!
//! **This binary is an instrument, not a gate.** It prints numbers; it grades
//! nothing and it never fails. Its job is to let a tolerance be *derived* from
//! measured structure before it is written into `passg.rs`, so that no number
//! in the graded suite was chosen by watching a comparison go green.
//!
//! ## What it needs
//!
//! - `$ICCCE_PRIVATE_FIXTURES` (or the default `D:\Dev\iccce-private-fixtures`)
//!   containing `ghent-v50\`. **Nothing from that directory may be committed to
//!   this repository and no value may be copied out of it into the repo** —
//!   the Ghent suite licence forbids redistribution, and the profiles inside
//!   carry Adobe's / ECI's / X-Rite's separate licences. See `LEGAL.md` §3 and
//!   `docs/NEXT_SESSION.md` §4.
//! - the pinned oracle (`ICCCE_TRANSICC` or `vendor/build-msvc/transicc.exe`);
//! - `target/release/iccce`.
//!
//! Absent any of them it prints a reason and exits 0 — an instrument that
//! cannot take a reading has not found a defect.

use std::path::{Path, PathBuf};

use iccce_color::{Lab, delta_e_2000};
use iccce_difftest::{Bpc, Iccce, Intent, Oracle, Precalc, Request, Space};

fn corpus() -> Option<PathBuf> {
    let root = std::env::var_os("ICCCE_PRIVATE_FIXTURES")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(r"D:\Dev\iccce-private-fixtures"));
    let g = root.join("ghent-v50");
    if g.is_dir() { Some(g) } else { None }
}

/// The corpus files, by content hash. **Names, not values** — a SHA-256 prefix
/// is a pointer to a licensed artifact, not a colour number, so naming one here
/// does not copy anything out of the private tree.
const SRGB: &str = "2b3aa1645779a9e6.icc";
const ADOBE: &str = "07c1e0738ba6068b.icc";
const ECIRGB_V2: &str = "4b55b697e41a8f29.icc";
const ECIRGB_V4: &str = "58c16e490b2751dc.icc";
const ISOCOATED_V2: &str = "128dc02f7246cc38.icc";
const FOGRA39: &str = "da2b9b593e27cba2.icc";
const XRITE_V4: &str = "b5988983b6b3b7d4.icc";
const GRAY_K: &str = "5dae7984654a2c9f.icc";
const TRAP_RGB: &str = "13b44969a980dcd1.icc";
const TRAP_CMYK: &str = "bbdfa02565c1c1e9.icc";

struct Stat {
    max: f64,
    mean: f64,
    at: usize,
}

fn stat(v: &[f64]) -> Stat {
    let mut max = 0.0f64;
    let mut at = 0;
    for (i, &x) in v.iter().enumerate() {
        if x > max {
            max = x;
            at = i;
        }
    }
    Stat {
        max,
        mean: v.iter().sum::<f64>() / v.len() as f64,
        at,
    }
}

/// Device scale `transicc` prints in, per colour space. RGB and gray are
/// `0..255`, ink spaces `0..100`. A flat `/100` inflates every RGB destination
/// by 2.55× — recorded in the oracle memory as a live hazard.
fn scale_for(components: usize, is_rgb_or_gray: bool) -> f64 {
    let _ = components;
    if is_rgb_or_gray { 255.0 } else { 100.0 }
}

#[allow(clippy::too_many_arguments)]
fn end_to_end(
    oracle: &Oracle,
    iccce: &Iccce,
    label: &str,
    src: &Path,
    dst: &Path,
    rows: &[Vec<f64>],
    src_scale: f64,
    out_channels: usize,
    dst_is_rgb_or_gray: bool,
) {
    for intent in [
        Intent::Perceptual,
        Intent::RelativeColorimetric,
        Intent::Saturation,
        Intent::AbsoluteColorimetric,
    ] {
        for bpc in [Bpc::Off, Bpc::On] {
            let req = Request {
                input: Space::profile(src),
                output: Space::profile(dst),
                intent,
                precalc: Precalc::Exact,
                bpc,
                values: rows.iter().flatten().map(|v| v * src_scale).collect(),
            };
            let theirs = match oracle.convert_batch_shaped(&req, rows[0].len(), out_channels) {
                Ok(v) => v,
                Err(e) => {
                    println!(
                        "{label}\t{}\tbpc={}\tORACLE-ERR\t{e}",
                        intent.name(),
                        bpc.name()
                    );
                    continue;
                }
            };
            let mine = match iccce.transform_rows_shaped_bpc(
                src,
                dst,
                intent,
                rows,
                out_channels,
                bpc == Bpc::On,
            ) {
                Ok(v) => v,
                Err(e) => {
                    let s = format!("{e}");
                    let first = s.lines().next().unwrap_or("").to_string();
                    println!(
                        "{label}\t{}\tbpc={}\tICCCE-REFUSED\t{first}",
                        intent.name(),
                        bpc.name()
                    );
                    continue;
                }
            };
            let scale = scale_for(out_channels, dst_is_rgb_or_gray);
            let mut diffs = Vec::with_capacity(rows.len());
            let mut clipped = 0usize;
            for (m, t) in mine.iter().zip(&theirs) {
                let mut worst = 0.0f64;
                for (a, b) in m.iter().zip(t) {
                    let b = b / scale;
                    if !(0.0..=1.0).contains(&b) {
                        clipped += 1;
                    }
                    worst = worst.max((a - b.clamp(0.0, 1.0)).abs());
                }
                diffs.push(worst);
            }
            let s = stat(&diffs);
            println!(
                "{label}\t{}\tbpc={}\tdev-max={:.6e}\tdev-mean={:.6e}\tat={}\tclip={clipped}",
                intent.name(),
                bpc.name(),
                s.max,
                s.mean,
                s.at
            );
        }
    }
}

fn main() {
    let Some(g) = corpus() else {
        println!("ghent-v50 corpus not present: set ICCCE_PRIVATE_FIXTURES");
        return;
    };
    let oracle = match Oracle::locate() {
        Ok(Some(o)) => o,
        Ok(None) => {
            println!("no oracle");
            return;
        }
        Err(e) => {
            println!("oracle error: {e}");
            return;
        }
    };
    let iccce = match Iccce::locate() {
        Ok(Some(i)) => i,
        Ok(None) => {
            println!("no iccce binary");
            return;
        }
        Err(e) => {
            println!("iccce error: {e}");
            return;
        }
    };
    println!("# oracle {}", oracle.path().display());
    println!("# iccce  {}", iccce.path().display());

    let rgb: Vec<Vec<f64>> = iccce_difftest::pass4b::rgb_grid()
        .into_iter()
        .map(|t| t.to_vec())
        .collect();
    let cmyk: Vec<Vec<f64>> = iccce_difftest::pass4::grid()
        .into_iter()
        .map(|t| t.to_vec())
        .collect();
    let gray: Vec<Vec<f64>> = iccce_difftest::pass4b::gray_axis()
        .into_iter()
        .map(|v| vec![v])
        .collect();

    println!("\n## end-to-end, device units (0..1), lcms2 output clamped");
    end_to_end(
        &oracle,
        &iccce,
        "srgb->isocoated",
        &g.join(SRGB),
        &g.join(ISOCOATED_V2),
        &rgb,
        255.0,
        4,
        false,
    );
    end_to_end(
        &oracle,
        &iccce,
        "adobergb->fogra39",
        &g.join(ADOBE),
        &g.join(FOGRA39),
        &rgb,
        255.0,
        4,
        false,
    );
    end_to_end(
        &oracle,
        &iccce,
        "xrite-v4->srgb",
        &g.join(XRITE_V4),
        &g.join(SRGB),
        &cmyk,
        100.0,
        3,
        true,
    );
    end_to_end(
        &oracle,
        &iccce,
        "ecirgb-v4->isocoated",
        &g.join(ECIRGB_V4),
        &g.join(ISOCOATED_V2),
        &rgb,
        255.0,
        4,
        false,
    );
    end_to_end(
        &oracle,
        &iccce,
        "gray->isocoated",
        &g.join(GRAY_K),
        &g.join(ISOCOATED_V2),
        &gray,
        255.0,
        4,
        false,
    );

    println!("\n## v2/v4 eciRGB self-consistency (iccce only, both encodings -> ISO Coated v2)");
    let a = iccce.transform_rows_shaped(
        &g.join(ECIRGB_V2),
        &g.join(ISOCOATED_V2),
        Intent::RelativeColorimetric,
        &rgb,
        4,
    );
    let b = iccce.transform_rows_shaped(
        &g.join(ECIRGB_V4),
        &g.join(ISOCOATED_V2),
        Intent::RelativeColorimetric,
        &rgb,
        4,
    );
    match (a, b) {
        (Ok(a), Ok(b)) => {
            let d: Vec<f64> = a
                .iter()
                .zip(&b)
                .map(|(x, y)| {
                    x.iter()
                        .zip(y)
                        .map(|(p, q)| (p - q).abs())
                        .fold(0.0f64, f64::max)
                })
                .collect();
            let s = stat(&d);
            println!(
                "ecirgb v2 vs v4 (iccce): dev-max={:.6e} mean={:.6e} at={}",
                s.max, s.mean, s.at
            );
        }
        (x, y) => println!("ecirgb pair failed: {:?} {:?}", x.err(), y.err()),
    }

    println!("\n## trap profiles");
    let probes = vec![
        vec![1.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0],
        vec![0.0, 0.0, 1.0],
    ];
    for (name, p) in [("trap-rgb", TRAP_RGB)] {
        match iccce.transform_rows_shaped(
            &g.join(p),
            &g.join(SRGB),
            Intent::RelativeColorimetric,
            &probes,
            3,
        ) {
            Ok(v) => {
                for (i, r) in v.iter().enumerate() {
                    println!("{name}\tin={:?}\tout={:.6?}", probes[i], r);
                }
            }
            Err(e) => println!("{name} failed: {e}"),
        }
    }

    println!("\n## eciRGB v2 (tabulated curv, v2.4) -> ISO Coated v2, ICC-ABSOLUTE");
    println!("## the wtpt-substitution discriminator: same vendor, same space, v2 vs v4 header");
    for (name, p) in [("ecirgb-v2", ECIRGB_V2), ("ecirgb-v4", ECIRGB_V4)] {
        let req = Request {
            input: Space::profile(g.join(p)),
            output: Space::profile(g.join(ISOCOATED_V2)),
            intent: Intent::AbsoluteColorimetric,
            precalc: Precalc::Exact,
            bpc: Bpc::Off,
            values: rgb.iter().flatten().map(|v| v * 255.0).collect(),
        };
        let theirs = oracle.convert_batch_shaped(&req, 3, 4).unwrap();
        let mine = iccce
            .transform_rows_shaped(
                &g.join(p),
                &g.join(ISOCOATED_V2),
                Intent::AbsoluteColorimetric,
                &rgb,
                4,
            )
            .unwrap();
        let d: Vec<f64> = mine
            .iter()
            .zip(&theirs)
            .map(|(m, t)| {
                m.iter()
                    .zip(t)
                    .map(|(a, b)| (a - b / 100.0).abs())
                    .fold(0.0f64, f64::max)
            })
            .collect();
        let s = stat(&d);
        println!(
            "{name}\tabsolute\tdev-max={:.6e}\tmean={:.6e}",
            s.max, s.mean
        );
    }

    println!("\n## X-Rite v4 mAB: interpolation-method envelope, computed from the CLUT ALONE");
    xrite_envelope(&g.join(XRITE_V4), &oracle);

    println!("\n## the BPC divergence: which black point does each side use?");
    for (name, p) in [
        ("isocoated-v2.4", ISOCOATED_V2),
        ("fogra39-v2.1", FOGRA39),
        ("xrite-v4.2", XRITE_V4),
    ] {
        match iccce_difftest::pass5c::Fixture::open(&g.join(p)) {
            Ok(fx) => {
                let (_dev, darkest, initial, iso_l) = fx.iso_black();
                let branch =
                    iccce_difftest::pass5c::branch_for(fx.is_output_class, fx.is_ink_space);
                let det = iccce_difftest::pass5c::detect_destination_black_point(
                    branch,
                    &|l| fx.bt_rel(l),
                    &|l| fx.bt_perc(l),
                    darkest,
                );
                println!(
                    "{name}\tdarkest-vertex L*={:.6}\tISO initial L*={:.6}\tISO black L*={:.6}\t\
                     lcms2 black L*={:.6}\tdelta L*={:.6}",
                    darkest.l,
                    initial.l,
                    iso_l,
                    det.black.l,
                    (iso_l - det.black.l).abs()
                );
            }
            Err(e) => println!("{name}: {e}"),
        }
    }
    println!("\n## A2B0 corner detail: where does the 1.1e-3 dE00 corner residual come from?");
    corner_detail(&g.join(XRITE_V4), &oracle);

    println!("\n## destination B2A structure: node slope and the tetrahedral counterfactual");
    for (name, p) in [
        ("isocoated-v2.4", ISOCOATED_V2),
        ("fogra39-v2.1", FOGRA39),
        ("trap-cmyk", TRAP_CMYK),
    ] {
        b2a_structure(&g.join(p), name);
    }
}

/// For every `B2A*` tag: the CLUT's steepest node-to-node step (the "one lsb of
/// encoded input costs this much device output" quantity) and the difference
/// between iccce's n-linear geometry and lcms2's *default* tetrahedral one over
/// the Lab grid. The second number is a **counterfactual**: lcms2 does not take
/// that path for a Lab-PCS output LUT (`_cmsReadOutputLUT` forces trilinear),
/// so it is the sensitivity control that says what this comparison could have
/// seen had it needed to.
/// Print, per corner, the harness's Lab and lcms2's, component by component,
/// for both `A2B0` (non-identity 2-entry B curve) and `A2B1` (identity).
fn corner_detail(path: &Path, oracle: &Oracle) {
    let bytes = std::fs::read(path).expect("read");
    let profile = Profile::parse(&bytes).expect("parse");
    let corners: Vec<[f64; 4]> = (0..16u32)
        .map(|m| {
            [
                f64::from((m >> 3) & 1),
                f64::from((m >> 2) & 1),
                f64::from((m >> 1) & 1),
                f64::from(m & 1),
            ]
        })
        .collect();
    for (name, sig, intent) in [
        ("A2B0", Signature(0x4132_4230), Intent::Perceptual),
        ("A2B1", Signature(0x4132_4231), Intent::RelativeColorimetric),
    ] {
        let lut = read_lut_ab(&profile, sig).expect("lut");
        let clut = lut.clut.as_ref().unwrap();
        let dims: Vec<usize> = (0..usize::from(lut.input_chan))
            .map(|i| usize::from(clut.grid_points[i]))
            .collect();
        let data: Vec<f64> = match &clut.samples {
            ClutSamples::U16(v) => v.iter().map(|&s| f64::from(s) / 65535.0).collect(),
            ClutSamples::U8(v) => v.iter().map(|&s| f64::from(s) / 255.0).collect(),
        };
        let hc = HarnessClut::new(dims, usize::from(lut.output_chan), data);
        let a = lut.a_curves.as_ref().unwrap();
        let b = lut.b_curves.as_ref().unwrap();
        let req = Request {
            input: Space::profile(path),
            output: Space::lab_v2(),
            intent,
            precalc: Precalc::Exact,
            bpc: Bpc::Off,
            values: corners
                .iter()
                .flat_map(|q| q.iter().map(|v| v * 100.0))
                .collect(),
        };
        let theirs = oracle.convert_batch_shaped(&req, 4, 3).unwrap();
        println!("-- {name}");
        for (i, q) in corners.iter().enumerate() {
            let ins: Vec<f64> = q
                .iter()
                .enumerate()
                .map(|(j, v)| curve_eval(&a[j], *v))
                .collect();
            let mut mid = vec![0.0; 3];
            hc.eval(&ins, Scheme::NLinear, &mut mid);
            // Encoded CLUT output, before B curves: at a corner this must be an
            // exact u16 fraction.
            let codes: Vec<f64> = mid.iter().map(|v| v * 65535.0).collect();
            let o: Vec<f64> = mid
                .iter()
                .enumerate()
                .map(|(j, v)| curve_eval(&b[j], *v))
                .collect();
            let mine = Lab {
                l: o[0] * 100.0,
                a: o[1] * 255.0 - 128.0,
                b: o[2] * 255.0 - 128.0,
            };
            let t = Lab {
                l: theirs[i][0],
                a: theirs[i][1],
                b: theirs[i][2],
            };
            println!(
                "  {i:2} clut-codes=({:.3},{:.3},{:.3}) mine=({:.5},{:.5},{:.5}) lcms2=({:.4},{:.4},{:.4}) d=({:+.5},{:+.5},{:+.5}) dE={:.3e}",
                codes[0],
                codes[1],
                codes[2],
                mine.l,
                mine.a,
                mine.b,
                t.l,
                t.a,
                t.b,
                mine.l - t.l,
                mine.a - t.a,
                mine.b - t.b,
                delta_e_2000(mine, t)
            );
        }
    }
}

fn b2a_structure(path: &Path, name: &str) {
    let bytes = std::fs::read(path).expect("read");
    let profile = Profile::parse(&bytes).expect("parse");
    for (tag, sig) in [
        ("B2A0", Signature(0x4232_4130)),
        ("B2A1", Signature(0x4232_4131)),
        ("B2A2", Signature(0x4232_4132)),
    ] {
        let Some(e) = profile.tags.iter().find(|t| t.sig == sig) else {
            continue;
        };
        let (dims, outs, data) = match profile.decode_tag(e) {
            Some(Ok(d)) => match d.data {
                TagData::Lut16(l) => {
                    let n = usize::from(l.clut_points);
                    (
                        vec![n; usize::from(l.input_chan)],
                        usize::from(l.output_chan),
                        l.clut
                            .iter()
                            .map(|&s| f64::from(s) / 65535.0)
                            .collect::<Vec<_>>(),
                    )
                }
                TagData::Lut8(l) => {
                    let n = usize::from(l.clut_points);
                    (
                        vec![n; usize::from(l.input_chan)],
                        usize::from(l.output_chan),
                        l.clut
                            .iter()
                            .map(|&s| f64::from(s) / 255.0)
                            .collect::<Vec<_>>(),
                    )
                }
                _ => continue,
            },
            _ => continue,
        };
        let hc = HarnessClut::new(dims.clone(), outs, data);
        let mut a = vec![0.0; outs];
        let mut b = vec![0.0; outs];
        let mut cf = 0.0f64;
        for l in iccce_difftest::pass4b::lab_grid() {
            let enc = [
                (l.l / 100.0).clamp(0.0, 1.0),
                ((l.a + 128.0) / 255.0).clamp(0.0, 1.0),
                ((l.b + 128.0) / 255.0).clamp(0.0, 1.0),
            ];
            hc.eval(&enc, Scheme::NLinear, &mut a);
            hc.eval(&enc, Scheme::Lcms2Default, &mut b);
            for (x, y) in a.iter().zip(&b) {
                cf = cf.max((x - y).abs());
            }
        }
        let step = hc.max_adjacent_step();
        let n = dims[0] as f64;
        println!(
            "{name}\t{tag}\tgrid={n}\tmax-adjacent-step={step:.6}\t\
             slope-per-encoded-unit={:.4}\tlsb-device-bound={:.4e}\ttetrahedral-counterfactual={cf:.6e}",
            step * (n - 1.0),
            step * (n - 1.0) * 3.0 / 65535.0
        );
    }
}

// ---------------------------------------------------------------------------
// The v4 vendor mAB pipeline, reimplemented in the harness with switchable
// CLUT geometry. NO lcms2 output enters this computation — it is a property of
// the profile's own bytes and of the two published algorithms, which is what
// makes it usable as a tolerance rather than as a fitted residual.
// ---------------------------------------------------------------------------

use iccce_difftest::pass4b::{HarnessClut, Scheme};
use iccce_profile::Profile;
use iccce_profile::lut::{ClutSamples, CurveElement, LutAB};
use iccce_profile::num::Signature;
use iccce_profile::tag_types::{Curve, TagData};

/// Evaluate a `curv` element. Only the shapes this corpus actually uses are
/// handled; anything else aborts loudly rather than returning a plausible
/// number.
fn curve_eval(c: &CurveElement, x: f64) -> f64 {
    match c {
        CurveElement::Curve(Curve::Identity) => x,
        CurveElement::Curve(Curve::Gamma(g)) => x.clamp(0.0, 1.0).powf(f64::from(g.0) / 256.0),
        CurveElement::Curve(Curve::Table(t)) => {
            let n = t.len();
            assert!(n >= 2);
            let x = x.clamp(0.0, 1.0);
            let pos = x * (n - 1) as f64;
            let i = (pos.floor() as usize).min(n - 2);
            let f = pos - i as f64;
            let a = f64::from(t[i]) / 65535.0;
            let b = f64::from(t[i + 1]) / 65535.0;
            a + (b - a) * f
        }
        CurveElement::Parametric(_) => {
            panic!("parametric curve in an mAB the probe does not model")
        }
    }
}

fn read_lut_ab(p: &Profile, sig: Signature) -> Option<LutAB> {
    let e = p.tags.iter().find(|t| t.sig == sig)?;
    match p.decode_tag(e) {
        Some(Ok(d)) => match d.data {
            TagData::LutAToB(l) | TagData::LutBToA(l) => Some(l),
            _ => None,
        },
        _ => None,
    }
}

fn xrite_envelope(path: &Path, oracle: &Oracle) {
    let bytes = std::fs::read(path).expect("read v4 profile");
    let profile = Profile::parse(&bytes).expect("parse v4 profile");
    for (name, sig) in [
        ("A2B0", Signature(0x4132_4230)),
        ("A2B1", Signature(0x4132_4231)),
    ] {
        let Some(lut) = read_lut_ab(&profile, sig) else {
            println!("{name}: not decodable");
            continue;
        };
        let clut = lut.clut.as_ref().expect("clut");
        let dims: Vec<usize> = (0..usize::from(lut.input_chan))
            .map(|i| usize::from(clut.grid_points[i]))
            .collect();
        let data: Vec<f64> = match &clut.samples {
            ClutSamples::U16(v) => v.iter().map(|&s| f64::from(s) / 65535.0).collect(),
            ClutSamples::U8(v) => v.iter().map(|&s| f64::from(s) / 255.0).collect(),
        };
        let hc = HarnessClut::new(dims.clone(), usize::from(lut.output_chan), data);
        let a = lut.a_curves.as_ref().expect("a curves");
        let b = lut.b_curves.as_ref().expect("b curves");

        let eval = |q: &[f64], scheme: Scheme| -> Lab {
            let ins: Vec<f64> = q
                .iter()
                .enumerate()
                .map(|(i, v)| curve_eval(&a[i], *v))
                .collect();
            let mut out = vec![0.0; 3];
            hc.eval(&ins, scheme, &mut out);
            let o: Vec<f64> = out
                .iter()
                .enumerate()
                .map(|(i, v)| curve_eval(&b[i], *v))
                .collect();
            // v4 PCSLAB decode (ICC.1:2022 Table 42): L* = 100·n,
            // a*/b* = 255·n − 128.
            Lab {
                l: o[0] * 100.0,
                a: o[1] * 255.0 - 128.0,
                b: o[2] * 255.0 - 128.0,
            }
        };

        let grid = iccce_difftest::pass4::grid();
        let mut env = Vec::with_capacity(grid.len());
        for q in &grid {
            let n = eval(q, Scheme::NLinear);
            let l = eval(q, Scheme::Lcms2Default);
            env.push(delta_e_2000(n, l));
        }
        let s = stat(&env);
        println!(
            "{name}\tmethod-envelope dE00 max={:.6} mean={:.6} at={} (grid {:?}, {} pts)",
            s.max,
            s.mean,
            s.at,
            dims,
            grid.len()
        );

        // Corners: every device component 0 or 1. With A-curve endpoints at
        // 0x0000/0xFFFF these land on exact CLUT nodes, where both geometries
        // agree identically.
        let corners: Vec<[f64; 4]> = (0..16)
            .map(|m| {
                [
                    f64::from((m >> 3) & 1),
                    f64::from((m >> 2) & 1),
                    f64::from((m >> 1) & 1),
                    f64::from(m & 1),
                ]
            })
            .collect();
        let cenv: Vec<f64> = corners
            .iter()
            .map(|q| delta_e_2000(eval(q, Scheme::NLinear), eval(q, Scheme::Lcms2Default)))
            .collect();
        println!(
            "{name}\tcorner method-envelope dE00 max={:.3e}   A-curve endpoints=({:.6},{:.6})",
            stat(&cenv).max,
            curve_eval(&a[0], 0.0),
            curve_eval(&a[0], 1.0)
        );

        // iccce (in process) and the harness, both n-linear: the apparatus row.
        // Then the harness under lcms2's geometry against the oracle: the row
        // that would claim agreement.
        for (pcs_name, pcs) in [("*Lab4", Space::lab_v4()), ("*Lab2", Space::lab_v2())] {
            let req = Request {
                input: Space::profile(path),
                output: pcs,
                intent: if name == "A2B0" {
                    Intent::Perceptual
                } else {
                    Intent::RelativeColorimetric
                },
                precalc: Precalc::Exact,
                bpc: Bpc::Off,
                values: grid
                    .iter()
                    .flat_map(|q| q.iter().map(|v| v * 100.0))
                    .collect(),
            };
            match oracle.convert_batch_shaped(&req, 4, 3) {
                Ok(theirs) => {
                    let mut raw = Vec::new();
                    let mut emu = Vec::new();
                    for (i, q) in grid.iter().enumerate() {
                        let t = Lab {
                            l: theirs[i][0],
                            a: theirs[i][1],
                            b: theirs[i][2],
                        };
                        raw.push(delta_e_2000(eval(q, Scheme::NLinear), t));
                        emu.push(delta_e_2000(eval(q, Scheme::Lcms2Default), t));
                    }
                    let r = stat(&raw);
                    let e = stat(&emu);
                    println!(
                        "{name}\t{pcs_name}\tharness-nlinear vs lcms2: max={:.6} mean={:.6} | \
                         harness-lcms2geom vs lcms2: max={:.6} mean={:.6}  (collapse {:.1}x)",
                        r.max,
                        r.mean,
                        e.max,
                        e.mean,
                        r.max / e.max
                    );
                }
                Err(err) => println!("{name}: oracle error {err}"),
            }
        }
    }
}
