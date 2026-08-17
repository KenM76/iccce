//! # The built-in sRGB destination driven from a real CMYK press profile
//!
//! ## Why this file exists separately from `builtin_srgb_destination.rs`
//!
//! That file exercises sRGB → sRGB: a **matrix/TRC source** into the
//! constructed destination. This one exercises **a LUT-based CMYK press
//! profile** into it, which is a materially different code path and is
//! the one an actual consumer uses.
//!
//! The distinction is not pedantic. `Chain::with_destination` builds the
//! source half by delegating to the ordinary constructor with the source
//! standing in for both sides, then discarding the destination it
//! derived. For a matrix/TRC source that delegation is trivially fine.
//! For a **CMYK profile whose only usable model is an `A2B` LUT**, the
//! scaffold build has to succeed at source→source, and the PCS handed to
//! the constructed destination is **Lab**, not XYZ — so the chain's PCS
//! unification is genuinely exercised rather than bypassed.
//!
//! ★ A test that only ever ran RGB → RGB could have passed while this
//! path was broken, and the breakage would have surfaced first in a
//! consumer rendering a PDF/X page. That is the wrong place to find it.
//!
//! ## Evidence class — read before quoting any number from here
//!
//! **These are not colour-accuracy assertions and must not be read as
//! any.** No published ground truth exists for a CMYK LUT conversion:
//! ICC.1 mandates no interpolation method, and ICC's own reference
//! implementation ships zero expected colour values. What is asserted
//! here is **structural** — that the path runs, that outputs are finite
//! and in range, that the transform discriminates between distinct
//! inputs, that polarity is not inverted, and that swapping the
//! constructed destination for a parsed one changes little.
//!
//! Corpus-absent → **SKIP loudly, assert nothing.** A green run without
//! the fixtures is evidence this did not run.

use iccce_cmm::matrix_trc::Intent;
use iccce_cmm::transform::{Chain, Destination, DestinationProvenance};
use iccce_profile::Profile;

/// Resolve a real CMYK press profile and a reference sRGB profile.
///
/// Several CMYK candidates are tried so the test does not hinge on one
/// file surviving a corpus reshuffle.
fn corpus() -> Option<(Vec<u8>, Vec<u8>, String)> {
    let root = std::env::var("ICCCE_PRIVATE_FIXTURES")
        .unwrap_or_else(|_| r"D:\Dev\iccce-private-fixtures".to_string());
    let base = std::path::Path::new(&root).join("color-org");
    let srgb_path = base.join("sRGB2014.icc");
    if !srgb_path.is_file() {
        return None;
    }
    for name in [
        "SWOP2006_Coated5v2.icc",
        "GRACoL2006_Coated1v2.icc",
        "PSOuncoated_v3_FOGRA52.icc",
        "CGATS21_CRPC1.icc",
    ] {
        let cmyk_path = base.join(name);
        if cmyk_path.is_file() {
            return Some((
                std::fs::read(&cmyk_path).ok()?,
                std::fs::read(&srgb_path).ok()?,
                name.to_string(),
            ));
        }
    }
    None
}

/// Distinct CMYK tuples.
///
/// ★ Every component distinct within a tuple wherever the probe's
/// purpose allows (GP-002). A probe like `(0.5, 0.5, 0.5, 0.5)` cannot
/// distinguish a channel-order defect from a correct transform, because
/// every permutation of it is itself.
const CMYK_PROBES: &[[f64; 4]] = &[
    [0.0, 0.0, 0.0, 0.0],     // paper white
    [0.0, 0.0, 0.0, 1.0],     // 100 percent K
    [0.9, 0.1, 0.2, 0.05],    // cyan-dominant
    [0.15, 0.85, 0.25, 0.05], // magenta-dominant
    [0.1, 0.2, 0.95, 0.0],    // yellow-dominant
    [0.35, 0.45, 0.55, 0.15], // a muddled mid-tone
    [0.7, 0.6, 0.5, 0.4],     // heavy, near the profile's TAC
];

#[test]
fn cmyk_source_into_the_constructed_srgb_destination() {
    let Some((cmyk_bytes, srgb_bytes, name)) = corpus() else {
        eprintln!(
            "SKIP cmyk_source_into_the_constructed_srgb_destination: private fixtures absent. \
             This test asserted NOTHING."
        );
        return;
    };
    let cmyk = Profile::parse(&cmyk_bytes).expect("the press profile parses");
    let srgb = Profile::parse(&srgb_bytes).expect("sRGB2014 parses");

    // The path a consumer actually takes: it has the document's source
    // profile and no destination of its own.
    let via_builtin = Chain::with_destination(&cmyk, Destination::None, Intent::MediaRelative)
        .expect("CMYK -> constructed sRGB must build");
    assert_eq!(
        via_builtin.destination_provenance(),
        DestinationProvenance::BuiltInSrgb
    );
    assert!(
        via_builtin.destination_provenance().note().is_some(),
        "the built-in destination must disclose itself on a CMYK path too"
    );

    let via_file =
        Chain::with_destination(&cmyk, Destination::Profile(&srgb), Intent::MediaRelative)
            .expect("CMYK -> parsed sRGB must build");

    assert_eq!(
        via_builtin.input_channels(),
        4,
        "a CMYK source must present 4 input channels"
    );

    let mut outputs = Vec::new();
    let mut worst_channel_diff = 0.0_f64;
    for &cmyk_in in CMYK_PROBES {
        let a = via_builtin
            .convert(&cmyk_in)
            .unwrap_or_else(|e| panic!("built-in path failed on {cmyk_in:?}: {e}"));
        let b = via_file
            .convert(&cmyk_in)
            .unwrap_or_else(|e| panic!("file path failed on {cmyk_in:?}: {e}"));
        assert_eq!(a.len(), 3, "sRGB destination is 3 channels");

        // In range and finite. A LUT path that indexed off the end of a
        // table produces NaN or a wild value; both are caught here.
        for v in &a {
            assert!(
                v.is_finite() && (-0.001..=1.001).contains(v),
                "converted value {v} out of range for {cmyk_in:?} -> {a:?}"
            );
        }
        for (x, y) in a.iter().zip(b.iter()) {
            worst_channel_diff = worst_channel_diff.max((x - y).abs());
        }
        outputs.push((cmyk_in, a));
    }

    // ★ Discrimination: the transform must actually depend on its input.
    // A chain wired to a constant, or one that lost its CLUT and fell
    // through to something degenerate, still passes every assertion
    // above this line.
    let distinct: std::collections::BTreeSet<String> =
        outputs.iter().map(|(_, o)| format!("{o:.6?}")).collect();
    assert_eq!(
        distinct.len(),
        CMYK_PROBES.len(),
        "distinct CMYK inputs produced duplicate outputs — the transform is not discriminating: \
         {outputs:#?}"
    );

    // ★ An ordering fact that needs NO oracle: paper white must be
    // lighter than solid K in every channel. This is a property of ink
    // on paper, not a measurement, so it is checkable with nothing to
    // compare against — and it catches a polarity inversion, which is
    // the one CMYK defect loud enough to matter and easy to introduce.
    let white = &outputs[0].1;
    let black = &outputs[1].1;
    for i in 0..3 {
        assert!(
            white[i] > black[i],
            "paper white (CMYK 0,0,0,0 -> {white:?}) must be lighter than solid K ({black:?}) \
             in channel {i} — polarity looks inverted"
        );
    }

    // ★★ Hue ordering — the strongest oracle-free assertions available
    // on this path, and the reason this test has real power rather than
    // merely running.
    //
    // The discrimination check above proves only that distinct inputs
    // give distinct outputs. It would still pass if the CLUT's channels
    // were permuted, or if cyan and magenta were swapped — which is
    // precisely the defect the Ghent suite ships a deliberate trap
    // profile for ("Switch magenta cyan"). These assertions catch that,
    // and they need no oracle at all because they follow from what the
    // inks ARE:
    //
    //   cyan ink absorbs RED    → a cyan-dominant patch has B > R
    //   magenta ink absorbs GREEN → a magenta-dominant patch has R > G
    //   yellow ink absorbs BLUE  → a yellow-dominant patch has R > B
    //
    // These are definitional properties of subtractive primaries, not
    // measurements, so nothing here is a colour-accuracy claim — it is a
    // structural claim that happens to be checkable against physics.
    let cyan = &outputs[2].1;
    assert!(
        cyan[2] > cyan[0],
        "cyan-dominant CMYK {:?} gave RGB {cyan:?}: cyan absorbs red, so B must exceed R. \
         A channel permutation or a cyan/magenta swap would look like this.",
        outputs[2].0
    );
    let magenta = &outputs[3].1;
    assert!(
        magenta[0] > magenta[1],
        "magenta-dominant CMYK {:?} gave RGB {magenta:?}: magenta absorbs green, so R must \
         exceed G",
        outputs[3].0
    );
    let yellow = &outputs[4].1;
    assert!(
        yellow[0] > yellow[2],
        "yellow-dominant CMYK {:?} gave RGB {yellow:?}: yellow absorbs blue, so R must exceed B",
        outputs[4].0
    );

    println!(
        "{name} -> built-in sRGB: {} probes, max per-channel difference vs parsed sRGB2014 = \
         {worst_channel_diff:.3e}",
        CMYK_PROBES.len()
    );
    for (i, o) in &outputs {
        println!(
            "   CMYK {i:?} -> RGB [{:.6}, {:.6}, {:.6}]",
            o[0], o[1], o[2]
        );
    }

    // Swapping the constructed destination for the parsed one is a
    // destination swap, not a different rendering, so the two must stay
    // close. Stated in DEVICE units rather than ΔE on purpose: this
    // compares two device-side results, and the colorimetric claim
    // belongs in `builtin_srgb_destination.rs` where the ΔE machinery
    // and its derived tolerance already live. Two files, two claims.
    assert!(
        worst_channel_diff < 0.005,
        "built-in and parsed sRGB destinations differ by {worst_channel_diff:.4} in device \
         units — far more than the documented construction difference"
    );
}
