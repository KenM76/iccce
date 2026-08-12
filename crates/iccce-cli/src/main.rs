//! # iccce — the command-line shell
//!
//! The scriptable surface of the engine. Exists from Pass 0 because it
//! is what makes the library verifiable without a GUI: `tools/difftest`
//! drives conversions through this binary and diffs the numbers against
//! lcms2. Output is therefore **stable and machine-diffable** — one
//! value per line, no decorative formatting on the data lines.
//!
//! ## Subcommands (grow with the Passes)
//!
//! | Command | Pass | Purpose |
//! |---|---|---|
//! | `inspect <profile>` | 0 | Print the header and tag table. Malformations are printed, not hidden — the CLI is the parser's disclosure surface. |
//! | `transform` | 3+ | Convert values between profiles at a stated intent. |
//!
//! ## Exit codes
//!
//! - `0` — success.
//! - `1` — operational failure (unreadable file, unparseable profile,
//!   unsupported request). The reason goes to stderr.
//! - `2` — usage error (unknown subcommand, missing argument).

use std::process::ExitCode;

/// Usage text. Printed to stderr on usage errors (exit 2), because a
/// script piping stdout must never receive help text where it expected
/// data.
const USAGE: &str = "\
iccce — ICC colour management engine

USAGE:
  iccce inspect <profile>   Print the profile header and tag table.
  iccce transform --src <profile> --dst <profile> [--intent <i>] [--bpc]
                            Convert device values, source -> destination.
                            <i>: media-relative (default), perceptual,
                            saturation, absolute. --bpc opts in to black
                            point compensation (never forced; refused by
                            name at absolute or outside the estimation
                            subset). Reads one set of source device
                            values per line from stdin (floats in 0..1,
                            whitespace-separated; count = source channel
                            count); writes one converted set per line,
                            6 decimals.

  iccce bench --src <profile> --dst <profile> [--grid N] [--pixels N]
                            Time a page-sized conversion through the
                            compiled path. Defaults to a 300 DPI A4
                            raster (2481x3507 = 8,700,267 px) and a
                            17-point grid. Prints build time, convert
                            time, throughput and the compiled path's
                            off-node error against the reference path.

Profiles are read as raw bytes; any file (or stream dump) containing an
ICC profile is accepted. Malformations are reported, never repaired.
";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();

    match args.first().map(String::as_str) {
        Some("inspect") => {
            let Some(path) = args.get(1) else {
                eprintln!("iccce inspect: missing <profile> argument\n\n{USAGE}");
                return ExitCode::from(2);
            };
            cmd_inspect(path)
        }
        Some("transform") => cmd_transform(&args[1..]),
        Some("bench") => cmd_bench(&args[1..]),
        Some(other) => {
            eprintln!("iccce: unknown subcommand `{other}`\n\n{USAGE}");
            ExitCode::from(2)
        }
        None => {
            eprintln!("{USAGE}");
            ExitCode::from(2)
        }
    }
}

/// `iccce inspect <profile>` — read the file, parse header + tag table,
/// print both.
///
/// WHY this is the first command: ROADMAP Pass 0's done-when is "a real
/// profile from the system's colour directory can be inspected". It
/// exercises the byte-level parse with zero colour maths, which is
/// exactly the layering ARCHITECTURE.md §1 mandates.
fn cmd_inspect(path: &str) -> ExitCode {
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("iccce inspect: cannot read `{path}`: {e}");
            return ExitCode::from(1);
        }
    };

    let profile = match iccce_profile::Profile::parse(&bytes) {
        Ok(p) => p,
        Err(e) => {
            // A refusal is a result, not a crash — the reason is the
            // deliverable (the parser reports; it does not repair).
            eprintln!("iccce inspect: refused: {e}");
            return ExitCode::from(1);
        }
    };

    let h = &profile.header;
    // One `key: value` per line, stable order — this output is a diff
    // surface for tools/difftest, not a human UI.
    println!("header.size: {}", h.size);
    println!("header.cmm: {}", h.cmm_id);
    println!("header.version: {} (0x{:08X})", h.version, h.version.raw);
    println!("header.class: {}", h.device_class);
    println!("header.colorspace: {}", h.color_space);
    println!("header.pcs: {}", h.pcs);
    if h.date.is_unspecified() {
        println!("header.date: unspecified");
    } else {
        println!(
            "header.date: {:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
            h.date.year, h.date.month, h.date.day, h.date.hours, h.date.minutes, h.date.seconds
        );
    }
    println!("header.platform: {}", h.platform);
    println!("header.flags: 0x{:08X}", h.flags);
    println!("header.manufacturer: {}", h.manufacturer);
    println!("header.model: 0x{:08X}", h.model);
    println!("header.attributes: 0x{:016X}", h.attributes);
    let intent_name = match h.rendering_intent {
        0 => "perceptual",
        1 => "media-relative",
        2 => "saturation",
        3 => "absolute",
        _ => "UNKNOWN",
    };
    println!("header.intent: {} ({intent_name})", h.rendering_intent);
    println!(
        "header.illuminant: {:.4} {:.4} {:.4}",
        h.illuminant.x.to_f64(),
        h.illuminant.y.to_f64(),
        h.illuminant.z.to_f64()
    );
    println!("header.creator: {}", h.creator);
    let id_hex: String = h.profile_id.iter().map(|b| format!("{b:02x}")).collect();
    if h.profile_id.iter().all(|&b| b == 0) {
        // All-zero = not computed, which is NOT an error (corpus D4).
        println!("header.id: not computed");
    } else {
        println!("header.id: {id_hex}");
    }

    println!("tags: {}", profile.tags.len());
    for (i, t) in profile.tags.iter().enumerate() {
        let type_sig = t
            .type_sig
            .map_or_else(|| "unreadable".to_string(), |s| s.to_string());
        println!(
            "tag[{i}]: {} offset={} size={} type={type_sig}",
            t.sig, t.offset, t.size
        );
        // Pass 2: decoded view. One `tag[i].decoded:` line per known
        // type, plus one line per content issue — the CLI is the
        // parser's disclosure surface, so issues print unconditionally.
        match profile.decode_tag(t) {
            None => println!("tag[{i}].decoded: (data out of bounds, see malformations)"),
            Some(Err(e)) => println!("tag[{i}].decoded: REFUSED: {e}"),
            Some(Ok(decoded)) => {
                if let Some(line) = summarize(&decoded.data) {
                    println!("tag[{i}].decoded: {line}");
                }
                for issue in &decoded.issues {
                    println!("tag[{i}].issue: {issue}");
                }
            }
        }
    }

    // Disclosure surface: everything the file got wrong, verbatim.
    println!("malformations: {}", profile.malformations.len());
    for m in &profile.malformations {
        println!("malformation: {m}");
    }

    // A4c: a colorimetric inconsistency the PARSER cannot see (it
    // needs the matrix/TRC model to know what the colorants sum to),
    // so it is disclosed here rather than as a malformation. Silent
    // for every profile shape that cannot exhibit it.
    if let Ok(model) = iccce_cmm::MatrixTrc::from_profile(&profile) {
        if let Some(note) = model.white_point_note() {
            println!("note: {note}");
        }
    }
    ExitCode::SUCCESS
}

/// `iccce transform --src <p> --dst <p>` — Pass 3's scriptable surface.
///
/// WHY stdin/stdout triples: this is the interface `tools/difftest`
/// diffs against transicc, so the contract is the harness's, not a
/// human's — one triple per line, floats 0..1, output at 6 decimals
/// (one decimal more than transicc's 4, so the comparison is never
/// limited by iccce's print precision), no banner on stdout.
///
/// All four intents are accepted (Pass 4); an --intent naming
/// anything ELSE is refused by name rather than silently substituted —
/// a substituted intent produces plausible wrong colour. (An earlier
/// version of this comment said media-relative only and outlived the
/// code by three commits — caught by icc-librarian's live-source
/// audit, 2026-08-11.)
fn cmd_transform(args: &[String]) -> ExitCode {
    let mut src_path: Option<&String> = None;
    let mut dst_path: Option<&String> = None;
    let mut intent = iccce_cmm::matrix_trc::Intent::MediaRelative;
    let mut bpc = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--src" if i + 1 < args.len() => {
                src_path = Some(&args[i + 1]);
                i += 2;
            }
            "--dst" if i + 1 < args.len() => {
                dst_path = Some(&args[i + 1]);
                i += 2;
            }
            "--intent" if i + 1 < args.len() => {
                intent = match args[i + 1].as_str() {
                    "media-relative" => iccce_cmm::matrix_trc::Intent::MediaRelative,
                    "perceptual" => iccce_cmm::matrix_trc::Intent::Perceptual,
                    "saturation" => iccce_cmm::matrix_trc::Intent::Saturation,
                    "absolute" => iccce_cmm::matrix_trc::Intent::Absolute,
                    other => {
                        // Refuse an unknown name rather than substituting:
                        // a substituted intent produces plausible wrong
                        // colour.
                        eprintln!("iccce transform: unknown intent `{other}`\n\n{USAGE}");
                        return ExitCode::from(2);
                    }
                };
                i += 2;
            }
            "--bpc" => {
                bpc = true;
                i += 1;
            }
            other => {
                eprintln!("iccce transform: unknown argument `{other}`\n\n{USAGE}");
                return ExitCode::from(2);
            }
        }
    }
    let (Some(src_path), Some(dst_path)) = (src_path, dst_path) else {
        eprintln!("iccce transform: --src and --dst are required\n\n{USAGE}");
        return ExitCode::from(2);
    };

    let load = |path: &String| -> Result<iccce_profile::Profile, String> {
        let bytes = std::fs::read(path).map_err(|e| format!("cannot read `{path}`: {e}"))?;
        iccce_profile::Profile::parse(&bytes).map_err(|e| format!("`{path}` refused: {e}"))
    };
    let (src, dst) = match (load(src_path), load(dst_path)) {
        (Ok(s), Ok(d)) => (s, d),
        (Err(e), _) | (_, Err(e)) => {
            eprintln!("iccce transform: {e}");
            return ExitCode::from(1);
        }
    };
    // The Chain handles both source shapes (matrix/TRC and lut16 A2B)
    // with the sourced 8.10.2 fallback; the source's channel count
    // (3 for RGB, 4 for CMYK, …) sets the per-line input arity.
    let chain = match iccce_cmm::transform::Chain::new(&src, &dst, intent) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("iccce transform: cannot build transform: {e}");
            return ExitCode::from(1);
        }
    };
    let chain = if bpc {
        match chain.with_bpc() {
            Ok(c) => c,
            Err(e) => {
                // A refusal (absolute intent; estimation outside the
                // named subset) is the deliverable, not a crash.
                eprintln!("iccce transform: --bpc refused: {e}");
                return ExitCode::from(1);
            }
        }
    } else {
        chain
    };
    let channels = chain.input_channels();

    let stdin = std::io::stdin();
    for (lineno, line) in std::io::BufRead::lines(stdin.lock()).enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("iccce transform: stdin read error: {e}");
                return ExitCode::from(1);
            }
        };
        if line.trim().is_empty() {
            continue;
        }
        let vals: Result<Vec<f64>, _> = line.split_whitespace().map(str::parse::<f64>).collect();
        let device: Vec<f64> = match vals {
            Ok(v) if v.len() == channels => v,
            _ => {
                eprintln!(
                    "iccce transform: line {}: expected {channels} floats \
                     (source channel count), got `{line}`",
                    lineno + 1
                );
                return ExitCode::from(1);
            }
        };
        match chain.convert(&device) {
            Ok(out) => {
                // Destination channel count varies (3 RGB, 4 CMYK…).
                let line: Vec<String> = out.iter().map(|v| format!("{v:.6}")).collect();
                println!("{}", line.join(" "));
            }
            Err(e) => {
                eprintln!("iccce transform: line {}: {e}", lineno + 1);
                return ExitCode::from(1);
            }
        }
    }
    ExitCode::SUCCESS
}

/// `iccce bench` — Pass 6's done-when, on the shipped surface.
///
/// Reports, one `key: value` per line (diffable, like `inspect`):
/// grid build time, per-pixel conversion time over a page-sized
/// raster, throughput, and the compiled path's **off-node** error
/// against the reference path.
///
/// ★ The error line is deliberately measured OFF-NODE and labelled
/// `self-consistency`: at a grid node the two arms are identical by
/// construction (the node IS a reference evaluation), so an on-node
/// number would be a spectacular zero that measured nothing —
/// exactly the DL-023 trap the unit tests document.
fn cmd_bench(args: &[String]) -> ExitCode {
    let mut src_path: Option<&String> = None;
    let mut dst_path: Option<&String> = None;
    // 0 = "use the measured recommendation for this channel count"
    // (resolved once the source's arity is known). An explicit
    // --grid overrides it.
    let mut grid = 0usize;
    // 300 DPI A4: 8.268 x 11.693 in → 2481 x 3507 px.
    let mut pixels = 2481usize * 3507;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--src" if i + 1 < args.len() => {
                src_path = Some(&args[i + 1]);
                i += 2;
            }
            "--dst" if i + 1 < args.len() => {
                dst_path = Some(&args[i + 1]);
                i += 2;
            }
            "--grid" if i + 1 < args.len() => {
                let Ok(n) = args[i + 1].parse::<usize>() else {
                    eprintln!("iccce bench: --grid needs an integer >= 2");
                    return ExitCode::from(2);
                };
                grid = n;
                i += 2;
            }
            "--pixels" if i + 1 < args.len() => {
                let Ok(n) = args[i + 1].parse::<usize>() else {
                    eprintln!("iccce bench: --pixels needs an integer");
                    return ExitCode::from(2);
                };
                pixels = n;
                i += 2;
            }
            other => {
                eprintln!("iccce bench: unknown argument `{other}`\n\n{USAGE}");
                return ExitCode::from(2);
            }
        }
    }
    let (Some(src_path), Some(dst_path)) = (src_path, dst_path) else {
        eprintln!("iccce bench: --src and --dst are required\n\n{USAGE}");
        return ExitCode::from(2);
    };
    if grid == 1 {
        eprintln!("iccce bench: --grid must be >= 2");
        return ExitCode::from(2);
    }

    let load = |p: &String| -> Result<iccce_profile::Profile, String> {
        let bytes = std::fs::read(p).map_err(|e| format!("cannot read `{p}`: {e}"))?;
        iccce_profile::Profile::parse(&bytes).map_err(|e| format!("`{p}` refused: {e}"))
    };
    let (src, dst) = match (load(src_path), load(dst_path)) {
        (Ok(s), Ok(d)) => (s, d),
        (Err(e), _) | (_, Err(e)) => {
            eprintln!("iccce bench: {e}");
            return ExitCode::from(1);
        }
    };
    let chain = match iccce_cmm::transform::Chain::new(
        &src,
        &dst,
        iccce_cmm::matrix_trc::Intent::MediaRelative,
    ) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("iccce bench: cannot build transform: {e}");
            return ExitCode::from(1);
        }
    };
    let in_ch = chain.input_channels();
    if grid == 0 {
        grid = iccce_cmm::compiled::recommended_grid_points(in_ch);
    }
    let out_ch = chain.output_channels();

    let t0 = std::time::Instant::now();
    let compiled = match iccce_cmm::compiled::CompiledTransform::new(&chain, grid) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("iccce bench: cannot compile: {e}");
            return ExitCode::from(1);
        }
    };
    let build = t0.elapsed();

    // A synthetic raster with structure (not one repeated pixel,
    // which would be unrepresentatively cache-friendly).
    let mut raster = vec![0.0f64; pixels * in_ch];
    for p in 0..pixels {
        for c in 0..in_ch {
            #[allow(clippy::cast_precision_loss)]
            let v = ((p * 7 + c * 131) % 1024) as f64 / 1023.0;
            raster[p * in_ch + c] = v;
        }
    }
    let mut out = vec![0.0f64; pixels * out_ch];

    let t1 = std::time::Instant::now();
    if !compiled.convert_buffer(&raster, &mut out) {
        eprintln!("iccce bench: buffer shape mismatch");
        return ExitCode::from(1);
    }
    let convert = t1.elapsed();

    // The reference path over a bounded prefix of the SAME raster,
    // timed in-process so the comparison is transform-vs-transform
    // and not dominated by stdio (a first attempt measured the CLI's
    // text parsing and reported ~49k px/s, which said nothing about
    // either path).
    let ref_pixels = pixels.min(100_000);
    let t2 = std::time::Instant::now();
    let mut sink = 0.0f64;
    for p in 0..ref_pixels {
        let px = &raster[p * in_ch..(p + 1) * in_ch];
        if let Ok(r) = chain.convert(px) {
            sink += r[0]; // keep the loop from being optimised away
        }
    }
    let ref_time = t2.elapsed();
    std::hint::black_box(sink);

    // Off-node error against the reference path, on a sample of the
    // raster (the reference path is far too slow for every pixel —
    // which is the entire point of compiling).
    let mut worst = 0.0f64;
    let mut checked = 0usize;
    let step = (pixels / 512).max(1);
    let mut p = 0;
    while p < pixels {
        let px = &raster[p * in_ch..(p + 1) * in_ch];
        if let Ok(reference) = chain.convert(px) {
            for c in 0..out_ch {
                worst = worst.max((out[p * out_ch + c] - reference[c]).abs());
            }
            checked += 1;
        }
        p += step;
    }

    #[allow(clippy::cast_precision_loss)]
    let mpix_per_s = pixels as f64 / convert.as_secs_f64() / 1.0e6;
    println!("src: {src_path}");
    println!("dst: {dst_path}");
    println!("channels: {in_ch} -> {out_ch}");
    println!("grid.points_per_axis: {grid}");
    // in_ch is a profile channel count (≤ 15): cannot truncate.
    #[allow(clippy::cast_possible_truncation)]
    let dims = in_ch as u32;
    println!("grid.nodes: {}", grid.saturating_pow(dims));
    println!("build.seconds: {:.6}", build.as_secs_f64());
    println!("raster.pixels: {pixels}");
    println!("convert.seconds: {:.6}", convert.as_secs_f64());
    println!("throughput.megapixels_per_second: {mpix_per_s:.3}");
    #[allow(clippy::cast_precision_loss)]
    let ref_mpix = ref_pixels as f64 / ref_time.as_secs_f64() / 1.0e6;
    println!("reference.pixels: {ref_pixels}");
    println!("reference.seconds: {:.6}", ref_time.as_secs_f64());
    println!("reference.megapixels_per_second: {ref_mpix:.3}");
    println!(
        "speedup.compiled_over_reference: {:.2}",
        mpix_per_s / ref_mpix
    );
    println!("error.samples: {checked}");
    println!("error.max_device_offnode: {worst:.9}");
    println!(
        "error.class: self-consistency (compiled vs reference, same code; \
         worthless as correctness evidence — NUMERIC_CLAIMS.md §1)"
    );
    ExitCode::SUCCESS
}

/// One stable, diffable line per decoded tag. `None` for types not
/// yet decoded (`TagData::Unknown`) — printing nothing is more honest
/// than printing "unknown", which would suggest the type was examined
/// and not recognised rather than not yet implemented.
fn summarize(data: &iccce_profile::tag_types::TagData) -> Option<String> {
    use iccce_profile::tag_types::{Curve, TagData};
    Some(match data {
        TagData::Curve(Curve::Identity) => "curve identity".to_string(),
        TagData::Curve(Curve::Gamma(g)) => format!("curve gamma={}", g.to_f64()),
        TagData::Curve(Curve::Table(t)) => format!("curve table n={}", t.len()),
        TagData::ParametricCurve(p) => format!(
            "parametric funcType={} params={}",
            p.func_type,
            p.params
                .iter()
                .map(|v| format!("{:.6}", v.to_f64()))
                .collect::<Vec<_>>()
                .join(",")
        ),
        TagData::Text(t) => format!("text {:?}", t.to_string_lossy()),
        TagData::Mluc(m) => format!(
            "mluc records={} first={:?}",
            m.records.len(),
            m.records
                .first()
                .and_then(iccce_profile::tag_types::MlucRecord::to_string_lossy)
                .unwrap_or_default()
        ),
        TagData::TextDescription(d) => format!(
            "desc ascii={:?}",
            String::from_utf8_lossy(
                &d.ascii[..d
                    .ascii
                    .iter()
                    .position(|&b| b == 0)
                    .unwrap_or(d.ascii.len())]
            )
        ),
        TagData::NamedColor2(n) => format!(
            "ncl2 colors={} deviceCoords={}",
            n.entries.len(),
            n.n_device_coords
        ),
        TagData::Xyz(v) => format!(
            "xyz n={} [{}]",
            v.len(),
            v.iter()
                .map(|x| format!(
                    "{:.4} {:.4} {:.4}",
                    x.x.to_f64(),
                    x.y.to_f64(),
                    x.z.to_f64()
                ))
                .collect::<Vec<_>>()
                .join("; ")
        ),
        TagData::S15Fixed16Array(v) => format!(
            "sf32 n={} [{}]",
            v.len(),
            v.iter()
                .map(|x| format!("{:.6}", x.to_f64()))
                .collect::<Vec<_>>()
                .join(",")
        ),
        TagData::Lut8(l) => format!(
            "lut8 in={} out={} clutPoints={} matrixIdentity={}",
            l.input_chan,
            l.output_chan,
            l.clut_points,
            l.matrix_is_identity()
        ),
        TagData::Lut16(l) => format!(
            "lut16 in={} out={} clutPoints={} inputEnt={} outputEnt={} matrixIdentity={}",
            l.input_chan,
            l.output_chan,
            l.clut_points,
            l.input_ent,
            l.output_ent,
            l.matrix_is_identity()
        ),
        TagData::LutAToB(l) => format!("lutAToB {}", summarize_lut_ab(l)),
        TagData::LutBToA(l) => format!("lutBToA {}", summarize_lut_ab(l)),
        TagData::Unknown => return None,
    })
}

/// Shared element summary for the mAB /mBA pipelines: which elements
/// are present, and the CLUT's shape when there is one.
fn summarize_lut_ab(l: &iccce_profile::lut::LutAB) -> String {
    let curves = |c: &Option<Vec<iccce_profile::lut::CurveElement>>| match c {
        Some(v) => v.len().to_string(),
        None => "absent".to_string(),
    };
    let clut = match &l.clut {
        Some(c) => format!(
            "grid=[{}] prec={}",
            c.grid_points[..usize::from(l.input_chan).min(16)]
                .iter()
                .map(u8::to_string)
                .collect::<Vec<_>>()
                .join("x"),
            c.precision
        ),
        None => "absent".to_string(),
    };
    format!(
        "in={} out={} B={} matrix={} M={} clut={} A={}",
        l.input_chan,
        l.output_chan,
        curves(&l.b_curves),
        if l.matrix.is_some() { "3x4" } else { "absent" },
        curves(&l.m_curves),
        clut,
        curves(&l.a_curves)
    )
}
