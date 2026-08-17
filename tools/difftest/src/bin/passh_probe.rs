//! # `passh_probe` — the instrument behind Pass H
//!
//! Pass H's §D grades iccce against the **ICC's own published statement of what
//! its Probe profiles do** (`Probe2 Profile Readme June 1, 2007`, shipped inside
//! `Probev2.zip`). This binary is the exploratory instrument the tolerances in
//! [`iccce_difftest::passh`] were derived from, and it stays in the tree for the
//! same reason `ghent_probe` does: **a tolerance derived from a measurement
//! nobody can re-take is a tolerance nobody can check.**
//!
//! It prints; it grades nothing. Run it with the private corpus present:
//!
//! ```text
//! cd tools/difftest && cargo run --release --bin passh_probe
//! ```
//!
//! ## What it prints
//!
//! 1. **The realised A2B lightness bands** of each Probe profile, per intent
//!    tag, against the readme's published `70..100` / `30..70` / `0..30`.
//! 2. **The realised B2A colorant assignment** — per-channel extrema, the
//!    largest monotonicity violation over an `L*` sweep, and the largest spread
//!    produced by varying `a*`/`b*` at fixed `L*` (the readme says those are
//!    ignored).
//! 3. **The population breakdown** by version, class, colour space and channel
//!    count, and the accept/refuse verdict of the shipped binary on every file.
//!
//! ★ **No number printed here is stored in this repository.** The corpus is
//! licensed (`D:\Dev\iccce-private-fixtures\README.md` § `color-org/`) and Pass
//! H recomputes everything it grades at run time.

use std::collections::BTreeMap;

use iccce_color::Lab;
use iccce_profile::Profile;
use iccce_profile::num::Signature;

use iccce_difftest::passh::{
    A2B_BANDS, AB_PROBES, B2A_COLORANT, TagEval, channels_of, corpus_dir, device_grid,
    lightness_sweep, raw_version,
};

const PROBES: [&str; 3] = ["Probev1_ICCv2.icc", "Probev1_ICCv4.icc", "Probev2_ICCv4.icc"];

fn main() {
    let dir = corpus_dir();
    println!("corpus: {}", dir.display());
    if !dir.is_dir() {
        println!("ABSENT — set ICCCE_PRIVATE_FIXTURES or place the corpus at the default path");
        return;
    }

    for name in PROBES {
        let path = dir.join(name);
        let Ok(bytes) = std::fs::read(&path) else {
            println!("\n### {name}: not present");
            continue;
        };
        let Ok(p) = Profile::parse(&bytes) else {
            println!("\n### {name}: REFUSED by iccce");
            continue;
        };
        println!(
            "\n### {name}  version {} class {} space {} pcs {} tags {} malformations {}",
            p.header.version,
            p.header.device_class,
            p.header.color_space,
            p.header.pcs,
            p.tags.len(),
            p.malformations.len()
        );

        for (sig, intent, lo, hi) in A2B_BANDS {
            let label = format!("A2B {intent:<15} readme L* {lo:>5.1}..{hi:<6.1}");
            match TagEval::build(&p, Signature(sig)) {
                None => println!("  {label}: tag absent or undecodable"),
                Some(e) => {
                    let n = e.device_channels();
                    let grid = device_grid(n);
                    let (mut lmin, mut lmax) = (f64::INFINITY, f64::NEG_INFINITY);
                    let (mut amin, mut amax) = (f64::INFINITY, f64::NEG_INFINITY);
                    let (mut bmin, mut bmax) = (f64::INFINITY, f64::NEG_INFINITY);
                    let mut ok = 0usize;
                    let (mut argmin, mut argmax) = (Vec::new(), Vec::new());
                    for d in &grid {
                        if let Some(l) = e.device_to_lab(d) {
                            if l.l < lmin {
                                lmin = l.l;
                                argmin = d.clone();
                            }
                            if l.l > lmax {
                                lmax = l.l;
                                argmax = d.clone();
                            }
                            amin = amin.min(l.a);
                            amax = amax.max(l.a);
                            bmin = bmin.min(l.b);
                            bmax = bmax.max(l.b);
                            ok += 1;
                        }
                    }
                    println!(
                        "  {label}: realised L* {lmin:.6}..{lmax:.6}  \
                         (below {:.3e} / above {:.3e})  a* {amin:.4}..{amax:.4}  \
                         b* {bmin:.4}..{bmax:.4}  {ok}/{} pts, {n}-ch\n      \
                         argmin={argmin:?} (ink {:.0}%)  argmax={argmax:?} (ink {:.0}%)",
                        (lo - lmin).max(0.0),
                        (lmax - hi).max(0.0),
                        grid.len(),
                        argmin.iter().sum::<f64>() * 100.0,
                        argmax.iter().sum::<f64>() * 100.0,
                    );
                }
            }
        }

        for (sig, intent, colorant, sel) in B2A_COLORANT {
            let label = format!("B2A {intent:<15} readme pure {colorant:<8} (ch {sel})");
            match TagEval::build(&p, Signature(sig)) {
                None => println!("  {label}: tag absent or undecodable"),
                Some(e) => {
                    let mut off_max = 0.0_f64;
                    let mut ab_spread = 0.0_f64;
                    let mut mono = 0.0_f64;
                    let mut prev: Option<f64> = None;
                    let (mut at_black, mut at_white) = (f64::NAN, f64::NAN);
                    let (mut smin, mut smax) = (f64::INFINITY, f64::NEG_INFINITY);
                    let mut broke = false;
                    for l in lightness_sweep() {
                        let mut vals = Vec::new();
                        for (a, b) in AB_PROBES {
                            let Some(dev) = e.lab_to_device(Lab { l, a, b }) else {
                                broke = true;
                                break;
                            };
                            for (i, v) in dev.iter().enumerate() {
                                if i == sel {
                                    smin = smin.min(*v);
                                    smax = smax.max(*v);
                                } else {
                                    off_max = off_max.max(v.abs());
                                }
                            }
                            vals.push(dev[sel]);
                        }
                        if broke {
                            break;
                        }
                        let hi = vals.iter().copied().fold(f64::NEG_INFINITY, f64::max);
                        let lo = vals.iter().copied().fold(f64::INFINITY, f64::min);
                        ab_spread = ab_spread.max(hi - lo);
                        if l == 0.0 {
                            at_black = vals[0];
                        }
                        if l == 100.0 {
                            at_white = vals[0];
                        }
                        if let Some(pv) = prev {
                            mono = mono.max(vals[0] - pv);
                        }
                        prev = Some(vals[0]);
                    }
                    if broke {
                        println!("  {label}: evaluation refused mid-sweep");
                    } else {
                        let at50 = e.lab_to_device(Lab { l: 50.0, a: 0.0, b: 0.0 });
                        let at50c = e.lab_to_device(Lab {
                            l: 50.0,
                            a: -60.0,
                            b: 40.0,
                        });
                        println!(
                            "  {label}: sel {smin:.6}..{smax:.6}  off-max {off_max:.3e}  \
                             mono-violation {mono:.3e}  ab-spread {ab_spread:.3e}  \
                             L*0 -> {at_black:.6}  L*100 -> {at_white:.6}\n      \
                             dev(L*50,0,0)={at50:?}\n      dev(L*50,-60,40)={at50c:?}"
                        );
                    }
                }
            }
        }
    }

    // --- the population ---------------------------------------------------
    println!("\n### population (harness's own reading of byte 8..12 and the header)");
    let mut by_version: BTreeMap<String, usize> = BTreeMap::new();
    let mut by_class: BTreeMap<String, usize> = BTreeMap::new();
    let mut by_space: BTreeMap<String, usize> = BTreeMap::new();
    let mut by_channels: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let (mut accepted, mut refused, mut unreadable) = (0usize, 0usize, 0usize);

    let mut files: Vec<_> = std::fs::read_dir(&dir)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("icc"))
        .collect();
    files.sort();

    for path in &files {
        let name = path.file_name().unwrap().to_string_lossy().into_owned();
        let Ok(bytes) = std::fs::read(path) else {
            unreadable += 1;
            continue;
        };
        let raw = raw_version(&bytes);
        let major = raw.map(|v| v >> 24).unwrap_or(0);
        match Profile::parse(&bytes) {
            Ok(p) => {
                accepted += 1;
                *by_version
                    .entry(format!("{}", p.header.version))
                    .or_default() += 1;
                *by_class
                    .entry(format!("{}", p.header.device_class))
                    .or_default() += 1;
                let sp = format!("{}", p.header.color_space);
                *by_space.entry(sp.clone()).or_default() += 1;
                let ch = channels_of(p.header.color_space.0)
                    .map_or_else(|| "unknown".to_string(), |n| format!("{n}"));
                by_channels.entry(ch).or_default().push(name);
            }
            Err(e) => {
                refused += 1;
                println!(
                    "  REFUSED major={major} raw=0x{:08X}  {name}: {e}",
                    raw.unwrap_or(0)
                );
            }
        }
    }
    println!("  accepted {accepted}  refused {refused}  unreadable {unreadable}  total {}", files.len());
    println!("  versions: {by_version:?}");
    println!("  classes:  {by_class:?}");
    println!("  spaces:   {by_space:?}");
    for (ch, names) in &by_channels {
        println!("  {ch}-channel: {} file(s)", names.len());
        if ch != "3" && ch != "4" {
            for n in names {
                println!("      {n}");
            }
        }
    }
}
