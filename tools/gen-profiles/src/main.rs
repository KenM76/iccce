//! # `gen-profiles` — the synthetic ICC fixture generator
//!
//! ## What this program is for
//!
//! `docs/ROADMAP.md` records Pass 2's done-when clause 2 — *"a synthetic
//! corpus covers each tag type"* — as **PARTIAL**, because the only synthetic
//! bytes the project had were authored inside `iccce-profile`'s unit tests.
//! Those are tag-level, not profile-level: they cannot cover header/tag-table
//! /tag-data interaction, cross-tag consistency, or anything a consumer would
//! open, and nothing outside `cargo test` can use them. This program is the
//! other half: whole profiles, on disk, in `fixtures/synthetic/`.
//!
//! ## The rule everything here serves
//!
//! From `fixtures/synthetic/README.md`:
//!
//! > a synthetic fixture that cannot be regenerated is just a binary blob with
//! > better branding.
//!
//! Hence:
//!
//! * **Nothing reads a clock, an environment variable, or a random number.**
//!   The profile creation date is the constant
//!   [`iccce_gen_profiles::profile::FIXTURE_DATE`]. Byte-for-byte
//!   reproducibility is a property this corpus has, not one it hopes for.
//! * **`verify` is a first-class subcommand**, not a script. It regenerates
//!   every recipe in memory and compares against the files on disk. A fixture
//!   that has drifted from its generator — edited by hand, corrupted by a
//!   transfer, or left behind by a generator change — is a hard failure with a
//!   named byte offset.
//! * **The manifest is generated**, so the invocation recorded beside a
//!   fixture is by construction the invocation that produced it.
//!
//! ## Usage
//!
//! ```text
//! gen-profiles list                 # every recipe: name, category, coverage
//! gen-profiles <recipe> <out.icc>   # write one fixture
//! gen-profiles all <dir>            # write every fixture into <dir>
//! gen-profiles verify <dir>         # regenerate and compare, byte for byte
//! gen-profiles manifest             # emit fixtures/synthetic/MANIFEST.md
//! ```
//!
//! ## Exit codes
//!
//! * `0` — success; for `verify`, every fixture matched.
//! * `1` — operational failure (unwritable path, unreadable directory) **or a
//!   verification mismatch**. The reason goes to stderr.
//! * `2` — usage error (unknown recipe, missing argument). Distinguished from
//!   `1` so a script can tell "I asked wrongly" from "the corpus is wrong".
//!
//! Data goes to stdout; diagnostics to stderr; a script piping stdout never
//! receives help text where it expected content.

mod bytes;
mod profile;
mod recipes;
mod tags;

use std::path::Path;
use std::process::ExitCode;

const USAGE: &str = "\
gen-profiles — byte-by-byte synthetic ICC profile generator (iccce)

USAGE:
  gen-profiles list                 List every recipe with its coverage.
  gen-profiles <recipe> <out.icc>   Write one fixture.
  gen-profiles all <dir>            Write every fixture into <dir>.
  gen-profiles verify <dir>         Regenerate every fixture and compare it,
                                    byte for byte, with the file on disk.
  gen-profiles manifest             Emit MANIFEST.md on stdout.

Every fixture is deterministic: the same recipe always produces the same
bytes. Nothing here reads a clock or an environment variable.
";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("list") => cmd_list(),
        Some("manifest") => cmd_manifest(),
        Some("all") => match args.get(1) {
            Some(dir) => cmd_all(Path::new(dir)),
            None => usage("all: missing <dir>"),
        },
        Some("verify") => match args.get(1) {
            Some(dir) => cmd_verify(Path::new(dir)),
            None => usage("verify: missing <dir>"),
        },
        Some(name) => match (recipes::find(name), args.get(1)) {
            (Some(r), Some(out)) => cmd_one(&r, Path::new(out)),
            (Some(_), None) => usage(&format!("{name}: missing <out.icc>")),
            (None, _) => usage(&format!("unknown recipe `{name}`")),
        },
        None => usage("no subcommand"),
    }
}

fn usage(msg: &str) -> ExitCode {
    eprintln!("gen-profiles: {msg}\n\n{USAGE}");
    ExitCode::from(2)
}

/// One tab-separated line per recipe. TSV rather than a table because this is
/// a diff surface for tooling, the same choice `tools/difftest` makes and for
/// the same reason: no escaping rules, no crates.
fn cmd_list() -> ExitCode {
    println!("name\tcategory\tbytes\tcovers");
    for r in recipes::all() {
        let n = (r.build)().len();
        println!("{}\t{}\t{}\t{}", r.name, r.category.label(), n, r.covers);
    }
    ExitCode::SUCCESS
}

fn cmd_one(r: &recipes::Recipe, out: &Path) -> ExitCode {
    let bytes = (r.build)();
    match std::fs::write(out, &bytes) {
        Ok(()) => {
            println!("{}\t{}\t{} bytes", r.name, out.display(), bytes.len());
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("gen-profiles: cannot write {}: {e}", out.display());
            ExitCode::from(1)
        }
    }
}

fn cmd_all(dir: &Path) -> ExitCode {
    if let Err(e) = std::fs::create_dir_all(dir) {
        eprintln!("gen-profiles: cannot create {}: {e}", dir.display());
        return ExitCode::from(1);
    }
    for r in recipes::all() {
        let path = dir.join(format!("{}.icc", r.name));
        if cmd_one(&r, &path) != ExitCode::SUCCESS {
            return ExitCode::from(1);
        }
    }
    ExitCode::SUCCESS
}

/// Regenerate every recipe and compare with the file on disk.
///
/// Reports the **first differing byte offset**, not just "differs": a fixture
/// that has drifted is a question about which byte changed, and answering it in
/// the failure message is the difference between a two-minute diagnosis and an
/// afternoon with a hex editor.
fn cmd_verify(dir: &Path) -> ExitCode {
    let mut bad = 0usize;
    let mut ok = 0usize;
    for r in recipes::all() {
        let path = dir.join(format!("{}.icc", r.name));
        let expected = (r.build)();
        match std::fs::read(&path) {
            Err(e) => {
                println!("MISSING\t{}\t{}: {e}", r.name, path.display());
                bad += 1;
            }
            Ok(actual) if actual == expected => {
                println!("OK\t{}\t{} bytes", r.name, actual.len());
                ok += 1;
            }
            Ok(actual) => {
                let at = actual
                    .iter()
                    .zip(&expected)
                    .position(|(a, b)| a != b)
                    .map_or_else(|| "length only".to_string(), |i| format!("byte {i}"));
                println!(
                    "DIFFERS\t{}\ton disk {} bytes, generator {} bytes, first difference at {at}",
                    r.name,
                    actual.len(),
                    expected.len()
                );
                bad += 1;
            }
        }
    }
    println!("verify\t{ok} identical\t{bad} not identical");
    if bad == 0 {
        ExitCode::SUCCESS
    } else {
        eprintln!(
            "gen-profiles: {bad} fixture(s) do not match their generator — \
             a fixture that cannot be regenerated is just a binary blob"
        );
        ExitCode::from(1)
    }
}

/// Emit `fixtures/synthetic/MANIFEST.md`.
///
/// Generated rather than written, so the recorded invocation cannot drift from
/// the code that produces the bytes. The prose that surrounds the tables is
/// held here in one place for the same reason a header comment is: it explains
/// what a reader is looking at, and it has to change when the tables do.
fn cmd_manifest() -> ExitCode {
    let all = recipes::all();
    println!("# fixtures/synthetic — MANIFEST");
    println!();
    println!("**Generated by `gen-profiles manifest`. Do not edit by hand** — it is");
    println!("regenerated from `tools/gen-profiles/src/recipes.rs`, which is where the");
    println!("bytes actually come from. Editing this file would let the record and the");
    println!("artefact drift, which is the one thing a manifest exists to prevent.");
    println!();
    println!("Every fixture here is **category (a)** per `docs/LEGAL.md` §3: authored");
    println!("byte by byte by this project, no third-party content, unrestricted. Each");
    println!("carries that statement in its own `cprt` tag, so a fixture that escapes the");
    println!("repository still states its provenance.");
    println!();
    println!("## Reproducing the corpus");
    println!();
    println!("```text");
    println!("cd tools/gen-profiles");
    println!("cargo run -- all ../../fixtures/synthetic");
    println!("cargo run -- verify ../../fixtures/synthetic     # byte-for-byte");
    println!("```");
    println!();
    println!("Generation is deterministic — no clock, no RNG, no environment. The");
    println!("creation date in every header is the constant 2026-08-11T00:00:00Z.");
    println!("`verify` regenerates each recipe in memory and compares it with the file on");
    println!("disk, naming the first differing byte; it exits 1 if anything has drifted.");
    println!();
    for (cat, heading, blurb) in [
        (
            recipes::Category::WellFormed,
            "## Well-formed fixtures",
            "These assert that the parser **reads** correctly. Each must parse with zero \
             malformations and produce the stated decoded summaries.",
        ),
        (
            recipes::Category::Malformed,
            "## Malformed fixtures",
            "These assert that the parser **reports** correctly — `docs/ARCHITECTURE.md` §3.2 \
             makes reporting, not repairing, the parser's contract, and a contract with no \
             failing input has no test. Each fixture is one well-formed base plus **one** named \
             mutation: a fixture broken in two ways cannot tell you which one the consumer \
             reported.",
        ),
        (
            recipes::Category::Disputed,
            "## Disputed fixtures \u{2014} required consumer behaviour NOT yet writable",
            "\u{2605} **Read nothing in this section as a requirement on a consumer.** Both \
             sections above state what a conformant consumer *must* do. A fixture lands here \
             when the specification has not been read on the exact point the fixture probes, \
             so neither 'must report' nor 'must be silent' can be written down without \
             inventing the answer. The `Expected of a consumer` row therefore carries a \
             **dated measurement of what iccce does today** and names the outstanding \
             sourcing question. When that question is answered the fixture moves into one of \
             the two sections above and its row becomes a real expectation \u{2014} that \
             move is the visible event this section exists to produce.",
        ),
    ] {
        println!("{heading}");
        println!();
        println!("{blurb}");
        println!();
        // \u{2605} An empty category prints an explicit sentence rather than a
        // heading with nothing under it. A bare heading reads as a generator
        // bug or as a section someone forgot to fill; the sentence says the
        // set is empty ON PURPOSE and names what would put something in it.
        // The disputed section is the one this exists for \u{2014} it emptied on
        // 2026-08-18 when its only member's sourcing landed.
        if !all.iter().any(|r| r.category == cat) {
            println!(
                "**No fixtures are currently in this category.** That is a statement, not an \
                 omission: every fixture in this corpus has a settled category. A fixture \
                 lands here only when a dispatch to `icc-spec-librarian` is outstanding on \
                 the exact point the fixture probes \u{2014} not when iccce's behaviour is \
                 doubted (that is a defect, and the fixture is filed under what the standard \
                 says), and not when the text has been read and licenses more than one \
                 consumer behaviour (that is settled, and the project's choice is recorded \
                 in the row as a choice). The last member was \
                 `v2-rendering-intent-high-bits`, which moved to well-formed on 2026-08-18 \
                 when ICC.1:2001-04 6.1.11 was read."
            );
            println!();
        }
        for r in all.iter().filter(|r| r.category == cat) {
            let bytes = (r.build)();
            println!("### `{}.icc`", r.name);
            println!();
            println!("| | |");
            println!("|---|---|");
            println!("| Invocation | `gen-profiles {} {}.icc` | ", r.name, r.name);
            println!("| Size | {} bytes | ", bytes.len());
            println!("| Covers | {} |", r.covers);
            println!("| Contents | {} |", r.what);
            println!("| Expected of a consumer | {} |", r.expect);
            println!();
        }
    }
    ExitCode::SUCCESS
}
