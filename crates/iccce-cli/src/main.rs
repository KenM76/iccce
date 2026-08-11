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
