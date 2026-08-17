//! # Regression: an A2B-only source must work with the built-in
//! # destination
//!
//! ## The defect this pins, and why it was worth a file of its own
//!
//! `Chain::with_destination(src, Destination::None, ..)` originally
//! obtained the source model by building a scaffold chain `src → src`
//! and throwing away its destination half. That is fine whenever the
//! source can *also* serve as a destination — which every profile the
//! feature was first tested against could, because they were all sRGB or
//! press profiles carrying both `A2B` and `B2A`.
//!
//! **It fails for a profile with an `A2B` tag and no `B2A`, no colorant
//! matrix and no `grayTRC`.** That shape is perfectly conformant: clause
//! 8 requires a `B2A` only for classes that need the reverse direction,
//! and a one-directional `scnr`-class profile has no reason to carry one.
//!
//! ## ★ The symptom was the dangerous part
//!
//! The scaffold's *destination* derivation failed, and surfaced its own
//! error:
//!
//! ```text
//! matrix/TRC model requires PCSXYZ (Annex F.3, normative); profile PCS is 'Lab '
//! ```
//!
//! That message is **true, correctly clause-cited, and about a model
//! iccce was about to discard.** A caller reads it as *"my source profile
//! is unusable"*, which is false — the source was fine, and the
//! destination being complained about was never going to be used.
//!
//! **A refusal that names the wrong clause is worse than a vague one,
//! because the citation makes it persuasive.** This is the founding
//! hazard of the project — a wrong answer that looks exactly like a right
//! one — reappearing in the error surface rather than in a colour value.
//!
//! ## How it was found
//!
//! Not by review, and not by any test above: by **scanning both real
//! corpora for profiles matching the risky shape** (`A2B` present, `B2A`
//! absent, `rXYZ` absent, `kTRC` absent) and then running the path
//! against them. Four profiles matched, all four failed, and all four are
//! in ICC's own published set — the colour-vision-deficiency simulation
//! profiles, which are `scnr` class, Lab PCS and one-directional by
//! design.
//!
//! ★ **The lesson generalises past this bug:** a code path that reuses
//! machinery "and discards part of the result" inherits every failure
//! mode of the part it discards. The discarded half cannot fail
//! harmlessly, because its error is what the caller sees.
//!
//! Corpus-absent → **SKIP loudly, assert nothing.**

use iccce_cmm::matrix_trc::Intent;
use iccce_cmm::transform::{Chain, Destination, DestinationProvenance};
use iccce_profile::Profile;

/// The four A2B-only profiles in the `color-org` corpus, found by shape
/// rather than picked by name.
const A2B_ONLY: &[&str] = &[
    "CVDlut-deutan.icc",
    "CVDlut-protan.icc",
    "CVDcolormap-deutan.icc",
    "CVDcolormap-protan.icc",
];

fn corpus_root() -> std::path::PathBuf {
    let root = std::env::var("ICCCE_PRIVATE_FIXTURES")
        .unwrap_or_else(|_| r"D:\Dev\iccce-private-fixtures".to_string());
    std::path::Path::new(&root).join("color-org")
}

#[test]
fn a2b_only_source_builds_against_the_builtin_destination() {
    let base = corpus_root();
    let mut tested = 0;
    for name in A2B_ONLY {
        let path = base.join(name);
        if !path.is_file() {
            continue;
        }
        let bytes = std::fs::read(&path).expect("readable");
        let profile = Profile::parse(&bytes).unwrap_or_else(|e| panic!("{name} should parse: {e}"));

        // Premise check: these really are the risky shape. If a corpus
        // reshuffle replaces them with two-directional profiles this
        // test would silently stop testing anything, so the premise is
        // asserted rather than assumed.
        let has_b2a = profile
            .tags
            .iter()
            .any(|t| t.sig.0 & 0xFFFF_FF00 == 0x4232_4100);
        assert!(
            !has_b2a,
            "{name} now carries a B2A tag — it is no longer the shape this regression pins, \
             and the test has stopped exercising the defect"
        );

        let chain = Chain::with_destination(&profile, Destination::None, Intent::MediaRelative)
            .unwrap_or_else(|e| {
                panic!(
                    "{name} is an A2B-only source and MUST build against the built-in \
                     destination, but got: {e}\n\nIf this message mentions a destination model \
                     or PCSXYZ, the scaffold-chain defect has regressed: the error is about a \
                     model that is never used."
                )
            });

        assert_eq!(
            chain.destination_provenance(),
            DestinationProvenance::BuiltInSrgb
        );

        // And it must actually convert, not merely build. A chain that
        // builds and then fails on every input is not a fix.
        let input = vec![0.25_f64; chain.input_channels()];
        let out = chain
            .convert(&input)
            .unwrap_or_else(|e| panic!("{name} built but could not convert: {e}"));
        assert_eq!(out.len(), 3, "sRGB destination is 3 channels");
        for v in &out {
            assert!(
                v.is_finite() && (-0.001..=1.001).contains(v),
                "{name} produced out-of-range {out:?}"
            );
        }
        println!("{name}: {} channels in -> RGB {out:?}", input.len());
        tested += 1;
    }

    if tested == 0 {
        eprintln!(
            "SKIP a2b_only_source_builds_against_the_builtin_destination: none of {A2B_ONLY:?} \
             found under {}. This test asserted NOTHING.",
            base.display()
        );
    }
}

/// ★ The contrast case, which is what makes the test above meaningful:
/// the same profiles still **cannot** serve as a DESTINATION, and the
/// refusal for that is correct and should stay.
///
/// Without this, a future "fix" that made every profile buildable as a
/// destination would turn the test above green for the wrong reason.
#[test]
fn the_same_profiles_are_still_refused_as_a_destination() {
    let base = corpus_root();
    let srgb_path = base.join("sRGB2014.icc");
    if !srgb_path.is_file() {
        eprintln!("SKIP the_same_profiles_are_still_refused_as_a_destination: corpus absent.");
        return;
    }
    let srgb_bytes = std::fs::read(&srgb_path).expect("readable");
    let srgb = Profile::parse(&srgb_bytes).expect("parses");

    let mut tested = 0;
    for name in A2B_ONLY {
        let path = base.join(name);
        if !path.is_file() {
            continue;
        }
        let bytes = std::fs::read(&path).expect("readable");
        let profile = Profile::parse(&bytes).expect("parses");
        let result =
            Chain::with_destination(&srgb, Destination::Profile(&profile), Intent::MediaRelative);
        assert!(
            result.is_err(),
            "{name} has no B2A, no colorant matrix and no grayTRC, so it cannot be a \
             destination — building one must be a named refusal, not a silent success"
        );
        println!(
            "{name} as destination: correctly refused — {}",
            result.unwrap_err()
        );
        tested += 1;
    }
    if tested == 0 {
        eprintln!("SKIP: none of the A2B-only profiles found. This test asserted NOTHING.");
    }
}
