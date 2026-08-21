//! # `passk_leak_floor` — how little chromatic ink can a leak guard SEE?
//!
//! ★★★ **The question this binary exists to answer, and why a green suite
//! could not answer it.** Pass K's black-preservation leak guards `E7` and
//! `F8` compare the same probe set through `iccce transform` twice, differing
//! in nothing but `--preserve-black`, and require `max |on − off| = 0`
//! exactly. They were **proven to fire** by injection (`NC-267`) — but an
//! injection turns a test red at **the magnitude injected and at no other**
//! (`DL-064`), and the sweep that followed found this:
//!
//! > **At an injected widening of the qualifying test to `t = 0.04`, the
//! > ENTIRE difftest suite was green with the defect compiled in.**
//!
//! The cause was not the tolerance and not the transform. It was the **probe
//! set**: a leak guard can only see a widening that reaches one of its own
//! probes, and the smallest chromatic ink `E7` and `F8` carried was
//! `1.106777e-1` and `5.000000e-2` — while the rival their own justification
//! named, in `TOLERANCES.md` §3.10.12.2 and in the module doc of
//! `crates/iccce-cmm`, was **`10⁻⁹` of cyan**. Seven-plus orders of magnitude,
//! with both numbers on the page and nobody subtracting them.
//!
//! ## What this binary prints, in four blocks
//!
//! 1. **`FLOORS`** — each committed probe generator's *detection floor*,
//!    `min over probes of max(C, M, Y)`, **computed and printed rather than
//!    asserted**. ★ This is the whole remedy for `E7`, whose pre-2026-08-21
//!    floor was **a property of an LCG seed**: `arbitrary_off_neutral` draws
//!    on `[0, 0.8)` and construction bounds its floor only at `0.8/2²¹ ≈
//!    3.8e-7`, i.e. at nothing, so re-seeding would have moved `E7`'s
//!    sensitivity **with no line of intent, comment or tolerance changing**.
//!    Printing it means a re-seed shows up as a changed number.
//! 2. **`RESPONSE`** — the leak magnitude a widening `t ≥ c` would produce, at
//!    each of 14 decades of chromatic ink `c`, on each available CMYK pair.
//!    **This is the sensitivity floor as a function of ink**, which is the
//!    measurement `docs/NEXT_SESSION.md` item 1 asks for.
//! 3. **`BASELINE`** — the same probes' actual on/off difference with the
//!    shipped predicate. Every entry must be `0.000000e0`; a non-zero here is
//!    a live leak and `E7`/`F8` are already red.
//! 4. **`UNDERFLOW`** — the one place where a floor genuinely cannot be pushed
//!    lower, demonstrated rather than argued.
//!
//! ## ★★★ How `RESPONSE` is measured WITHOUT recompiling the engine
//!
//! A widened qualifying test `|C|,|M|,|Y| ≤ t` changes exactly one thing for a
//! probe whose ink `c ≤ t`: `KPreserve::apply` returns `Some` instead of
//! `None`. Its `Some` branch is `[0.0, 0.0, 0.0, map_k(K)]` — **it discards
//! the chromatic input entirely**. So the preserved answer under injection at
//! ink `c` is bit-for-bit the preserved answer the *shipped, uninjected*
//! binary already gives at the exact-zero input with the same `K`, and the
//! unpreserved answer is what the shipped binary already gives at ink `c`.
//! Therefore
//!
//! ```text
//! response(c) = max | preserve(0, 0, 0, K) − plain(c, r₁c, r₂c, K) |
//! ```
//!
//! ★★ **That is a derived expectation, so it was checked against the thing it
//! models.** On 2026-08-21 the qualifying test was injected in a detached
//! worktree at `t = c` for `c ∈ {5e-2, 1e-2, 1e-3, 1e-4, 1e-6, 1e-9, 1e-12,
//! 1e-20, 1e-100, 4.940656e-324}`, the engine rebuilt at each, and the guard's
//! real `max |on − off|` compared with the model above. **The two agreed
//! exactly — `==` on `f64`, not within a tolerance — at all ten magnitudes on
//! both committed fixtures.** The injection is not committed; this identity is
//! what makes the number re-derivable without it.
//!
//! ★ **It is still an instrument, not a grader.** It has no tolerances, it
//! decides nothing, and its exit code is `0` when it managed to measure and
//! `1` when it did not. The graded statements live on `E7` and `F8`, whose
//! probe sets now include [`low_ink_decade_probes`] and whose printed detail
//! carries the floor every run.
//!
//! ## Usage
//!
//! ```text
//! cargo run --release --bin passk_leak_floor
//! ```
//!
//! Requires the shipped `iccce` binary (`$ICCCE_BIN` or
//! `target/release/iccce`) and **nothing else** — the two fixtures it needs
//! are committed. The licensed Ghent corpus (`$ICCCE_PRIVATE_FIXTURES`) adds
//! one more pair, `ISO Coated v2 300% (ECI)`, which is `E7`'s own destination;
//! absent it, that block says so and the rest still runs.
//!
//! ## ★★ Why two committed fixtures and not one
//!
//! `v2-cmyk-chromatic-neutral` is `F8`'s own fixture and is the right one for
//! a **device-unit** claim. But its black ink is **spectrally neutral**
//! (`DL-065`, `NC-278`): it appears in `L*` and in nothing else, so a
//! preserved answer at matched lightness is a **metamer** of the four-ink
//! separation it replaced. That trap annihilates a ΔE measurement while every
//! device-unit gate reads healthy. It does **not** bite here — this binary
//! reports device units throughout and a leak is a device-unit fact — but a
//! reader is entitled to see the number on a fixture whose black carries
//! chroma, so `v2-cmyk-warm-black` is measured beside it and the two are
//! printed side by side.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use iccce_difftest::passk::{
    LOW_INK_DECADES, corpus_dir, low_ink_decade_probes, max_dev, probe_floor, synthetic_dir,
};
use iccce_difftest::passk::{
    arbitrary_off_neutral, chromatic_gray_probes, node_aligned_off_neutral,
};
use iccce_difftest::{DiffError, Iccce, Intent};

/// The policy name `E7`/`F8` drive, mirrored here so the two arms of every
/// measurement in this file are the two arms those rows compare.
const POLICY: &str = "k-only-equal-lightness";

/// The `K` values every probe row in this binary walks — `chromatic_gray_probes`'
/// own, so the low-ink block is comparable with `F8`'s `5e-2`-and-up block
/// rather than being a second, differently-shaped experiment.
const K_VALUES: [f64; 5] = [0.0, 0.125, 0.25, 0.375, 0.5];

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("passk_leak_floor: {e}");
            ExitCode::from(1)
        }
    }
}

/// One CMYK profile used as both source and destination, with a label.
struct Pair {
    label: &'static str,
    note: &'static str,
    path: PathBuf,
}

/// The five probes at one chromatic-ink level, built exactly as
/// [`low_ink_decade_probes`] builds them.
///
/// The two ratios are `chromatic_gray_probes`' and both are strictly below 1,
/// so `max(C, M, Y) = c` on every row — which is what makes the floor a
/// property of the *list of levels* and not of the arithmetic.
fn probes_at(c: f64) -> Vec<[f64; 4]> {
    const M_OVER_C: f64 = 6.0 / 7.0;
    let y_over_c = (50.0 + 45.0 * M_OVER_C) / 90.0;
    K_VALUES
        .iter()
        .map(|&k| [c, M_OVER_C * c, y_over_c * c, k])
        .collect()
}

/// The five exact-zero probes: `C = M = Y = 0` at the same `K` values.
fn k_only_probes() -> Vec<[f64; 4]> {
    K_VALUES.iter().map(|&k| [0.0, 0.0, 0.0, k]).collect()
}

fn rows(p: &[[f64; 4]]) -> Vec<Vec<f64>> {
    p.iter().map(|r| r.to_vec()).collect()
}

fn as4(v: Vec<Vec<f64>>) -> Vec<[f64; 4]> {
    v.into_iter().map(|r| [r[0], r[1], r[2], r[3]]).collect()
}

fn plain(iccce: &Iccce, f: &Path, p: &[[f64; 4]]) -> Result<Vec<[f64; 4]>, DiffError> {
    Ok(as4(iccce.transform_rows_shaped(
        f,
        f,
        Intent::RelativeColorimetric,
        &rows(p),
        4,
    )?))
}

fn preserved(iccce: &Iccce, f: &Path, p: &[[f64; 4]]) -> Result<Vec<[f64; 4]>, DiffError> {
    Ok(as4(iccce.transform_rows_shaped_preserve_black(
        f,
        f,
        Intent::RelativeColorimetric,
        &rows(p),
        4,
        POLICY,
    )?))
}

#[expect(
    clippy::too_many_lines,
    reason = "this is a report generator: splitting it would scatter the printed layout across \
              functions and make the four blocks harder to keep aligned with each other"
)]
fn run() -> Result<(), String> {
    let iccce = Iccce::locate()
        .map_err(|e| e.to_string())?
        .ok_or("the shipped iccce binary was not found — `cargo build --release -p iccce-cli`")?;

    // ---------------------------------------------------------------- FLOORS
    println!(
        "=== FLOORS — min over probes of max(C, M, Y), computed from the committed generators"
    );
    println!();
    println!("  the smallest widening of the qualifying test each probe set can SEE.");
    println!("  STRUCTURAL means the value follows from the generator's own arithmetic;");
    println!(
        "  SEED-DEPENDENT means it is an accident of a fixed LCG seed and moves if anyone re-seeds it."
    );
    println!();
    let node = node_aligned_off_neutral();
    let arb = arbitrary_off_neutral();
    let gray = chromatic_gray_probes();
    let low = low_ink_decade_probes();
    let f_node = probe_floor(&node);
    let f_arb = probe_floor(&arb);
    let f_gray = probe_floor(&gray);
    let f_low = probe_floor(&low);
    // Built as a table rather than four println!s so the "kind" column is data
    // and not a literal argument - and so a fifth generator cannot be added
    // without deciding, in one place, which kind it is.
    let sets: [(&str, usize, f64, &str); 4] = [
        (
            "node_aligned_off_neutral (E7)",
            node.len(),
            f_node,
            "grid j/15 - construction gives >= 1/15 = 6.666667e-2 ONLY; WHICH node the seed reached is an OBSERVATION",
        ),
        (
            "arbitrary_off_neutral (E7)",
            arb.len(),
            f_arb,
            "★★ SEED-DEPENDENT - LCG on [0, 0.8); construction bounds it only at 0.8/2^21 = 3.814697e-7",
        ),
        (
            "chromatic_gray_probes (F8)",
            gray.len(),
            f_gray,
            "STRUCTURAL - c = i x 0.05, i = 1..=10, both ratios < 1",
        ),
        (
            "low_ink_decade_probes (E7+F8)",
            low.len(),
            f_low,
            "STRUCTURAL - LOW_INK_DECADES' last entry, both ratios < 1",
        ),
    ];
    println!("  {:<34} {:>4}  {:>15}  kind", "probe set", "n", "floor");
    for (name, n, f, kind) in sets {
        println!("  {name:<34} {n:>4}  {f:>15.6e}  {kind}");
    }
    println!();
    println!(
        "  E7's effective floor = min(node, arbitrary, low-ink) = {:.6e}   over {} probes",
        f_node.min(f_arb).min(f_low),
        node.len() + arb.len() + low.len()
    );
    println!(
        "  F8's effective floor = min(gray, low-ink)            = {:.6e}   over {} probes",
        f_gray.min(f_low),
        gray.len() + low.len()
    );
    println!();
    println!(
        "  ★ arbitrary_off_neutral's floor at full f64 precision, so a re-seed is unmissable:"
    );
    println!("      {f_arb:?}");
    println!(
        "    (TOLERANCES.md and NUMERIC_CLAIMS.md recorded this as `1.106780e-1`, which is the"
    );
    println!("     six-DECIMAL rounding 0.110678 re-expressed in scientific notation; the sixth");
    println!("     significant figure is a 7, not a 0.)");
    println!();

    // ------------------------------------------------------------ the pairs
    let mut pairs = vec![
        Pair {
            label: "v2-cmyk-chromatic-neutral",
            note: "F8's own fixture — COMMITTED; ★ its black is SPECTRALLY NEUTRAL (DL-065)",
            path: synthetic_dir().join("v2-cmyk-chromatic-neutral.icc"),
        },
        Pair {
            label: "v2-cmyk-warm-black",
            note: "COMMITTED; its black carries chroma (a* += 2K, b* += 6K) — the metamer control",
            path: synthetic_dir().join("v2-cmyk-warm-black.icc"),
        },
    ];
    let iso300 = corpus_dir().join("c6b4b62f07262437.icc");
    if iso300.is_file() {
        pairs.push(Pair {
            label: "ISO Coated v2 300% (ECI)",
            note: "E7's own destination — LICENSED, absent in CI",
            path: iso300,
        });
    } else {
        println!(
            "  (ISO Coated v2 300% (ECI) not present — set $ICCCE_PRIVATE_FIXTURES to include E7's own pair)"
        );
        println!();
    }
    for p in &pairs {
        if !p.path.is_file() {
            return Err(format!("fixture missing: {}", p.path.display()));
        }
    }
    println!("=== PAIRS — each profile is used as BOTH source and destination, media-relative");
    println!();
    for p in &pairs {
        println!("  {:<26}  {}", p.label, p.note);
    }
    println!();

    // -------------------------------------------------------------- RESPONSE
    println!(
        "=== RESPONSE — the leak magnitude a widening t >= c would produce, per decade of ink"
    );
    println!();
    println!(
        "  response(c) = max | preserve(0,0,0,K) - plain(c, (6/7)c, 0.984127c, K) | over 4 channels, 5 K values."
    );
    println!(
        "  Measured on the SHIPPED, UNINJECTED binary; see this file's header for the identity"
    );
    println!(
        "  that licenses it and for the ten magnitudes at which it was checked against a real"
    );
    println!("  compile-time injection (equal on f64, not within a tolerance).");
    println!();
    println!(
        "  UNITS: normalised device (0..1), max over all four output channels — the same metric E7/F8 grade."
    );
    println!(
        "  A guard whose probe floor is <= c DETECTS a widening at c iff response(c) > 0; the tolerance is exactly 0."
    );
    println!();
    print!("  {:>14}", "ink c");
    for p in &pairs {
        print!("  {:>26}", p.label);
    }
    println!();
    let mut zero_ref: Vec<Vec<[f64; 4]>> = Vec::new();
    for p in &pairs {
        zero_ref.push(preserved(&iccce, &p.path, &k_only_probes()).map_err(|e| e.to_string())?);
    }
    for &c in &LOW_INK_DECADES {
        print!("  {c:>14.6e}");
        for (i, p) in pairs.iter().enumerate() {
            let pl = plain(&iccce, &p.path, &probes_at(c)).map_err(|e| e.to_string())?;
            print!("  {:>26.6e}", max_dev(&zero_ref[i], &pl));
        }
        println!();
    }
    println!();
    println!("  ★★★ READ THIS COLUMN-WISE, NOT ROW-WISE. The response does not decay with ink: it");
    println!(
        "  RISES to a constant as c -> 0, because plain(c) -> plain(0), which is the four-ink"
    );
    println!("  separation of a K-only input — the very quantity black preservation replaces. So");
    println!(
        "  there is NO numerical sensitivity floor in the engine, the harness or the encoding:"
    );
    println!("  a leak guard sees a widening at ANY ink level it has a probe at, and its floor is");
    println!("  therefore a FREE PARAMETER of the probe set. That is the answer to 'how little");
    println!("  chromatic ink can a leak guard see' — as little as it is asked to.");
    println!();

    // -------------------------------------------------------------- BASELINE
    println!("=== BASELINE — the same probes' ACTUAL on/off difference, shipped predicate");
    println!();
    println!(
        "  Every entry must be 0.000000e0. A non-zero is a LIVE LEAK, not a floor measurement,"
    );
    println!("  and E7/F8 are already red when it happens.");
    println!();
    print!("  {:>14}", "ink c");
    for p in &pairs {
        print!("  {:>26}", p.label);
    }
    println!();
    let mut worst = 0.0_f64;
    for &c in &LOW_INK_DECADES {
        print!("  {c:>14.6e}");
        for p in &pairs {
            let pr = probes_at(c);
            let on = preserved(&iccce, &p.path, &pr).map_err(|e| e.to_string())?;
            let off = plain(&iccce, &p.path, &pr).map_err(|e| e.to_string())?;
            let d = max_dev(&on, &off);
            worst = worst.max(d);
            print!("  {d:>26.6e}");
        }
        println!();
    }
    println!();
    println!("  worst baseline leak over all levels and pairs: {worst:.6e}");
    println!();

    // ------------------------------------------------------------- UNDERFLOW
    println!("=== UNDERFLOW — the one floor that is NOT a free parameter");
    println!();
    println!(
        "  The harness writes each coordinate with format!(\"{{v}}\") and the CLI parses it with"
    );
    println!(
        "  str::parse::<f64>. A decimal that UNDERFLOWS TO 0.0 therefore arrives at the shipped"
    );
    println!(
        "  qualifying test as a GENUINE K-only input. Preservation then fires CORRECTLY, on != off,"
    );
    println!("  and the guard goes RED against an engine that did nothing wrong.");
    println!();
    let tiniest = f64::from_bits(1);
    let underflowed = tiniest / 2.0;
    // ★ Printed with `{:e}`, NOT with `{}`. f64's Display is not shortest-round-trip
    // in the subnormal range — `format!("{}", f64::from_bits(1))` emits a 324-digit
    // decimal expansion. The harness writes that expansion and the CLI parses it back
    // to the same bit pattern (which is why the row above reads 0.000000e0), but it is
    // unreadable in a report and would swamp this block.
    println!(
        "  smallest positive f64 subnormal   c = {tiniest:.6e}   (f64::from_bits(1); round-trips through the CLI unchanged)"
    );
    println!(
        "  the next step down                c = {underflowed:.6e}   (writes as `{underflowed}` — it IS zero)"
    );
    println!();
    print!("  {:>34}", "baseline leak at c =");
    for p in &pairs {
        print!("  {:>26}", p.label);
    }
    println!();
    for (label, c) in [
        ("subnormal 5e-324", tiniest),
        ("underflowed to 0.0", underflowed),
    ] {
        print!("  {label:>34}");
        for p in &pairs {
            let pr = probes_at(c);
            let on = preserved(&iccce, &p.path, &pr).map_err(|e| e.to_string())?;
            let off = plain(&iccce, &p.path, &pr).map_err(|e| e.to_string())?;
            print!("  {:>26.6e}", max_dev(&on, &off));
        }
        println!();
    }
    println!();
    println!("  So the usable range of a leak-guard probe floor is (4.940656e-324, infinity), and");
    println!("  LOW_INK_DECADES stops at 1e-12 — three decades below the 1e-9 rival, 312 decades");
    println!(
        "  clear of the underflow boundary, and below the level at which RESPONSE stops changing."
    );
    println!();
    println!(
        "  ★ For scale, and NOT as a floor: one 16-bit device quantum is 1/65535 = 1.525902e-5."
    );
    println!("  No value in a 16-bit ICC table can be smaller than that and non-zero, so the last");
    println!(
        "  nine decades above are unreachable from a DOCUMENT. They are reachable from a SOURCE"
    );
    println!("  EDIT, which is the only thing E7/F8 exist to catch: the rival is a change to a");
    println!(
        "  predicate written in floating point, and it is graded in the units it is written in."
    );

    Ok(())
}
