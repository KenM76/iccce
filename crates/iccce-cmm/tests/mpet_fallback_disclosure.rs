//! # The 8.10.2 step-a) fallback is disclosed, not silent
//!
//! ## What is being tested and why it needed testing at all
//!
//! ICC.1:2022 clause **8.10.2** is a `shall`-level ordered fallback.
//! Step **a)** prefers the `D2Bx`/`B2Dx` multiProcessingElements tags
//! *"except where this tag is not needed or supported by the CMM"*;
//! step **b)** falls back to `A2Bx`/`B2Ax`.
//!
//! iccce does not implement `mpet`, so it takes step b). **That is
//! conformant.** lcms2 does implement `mpet` and takes step a). **That
//! is also conformant.** The two then return materially different
//! colour — measured at **33.13 L\*** on ICC's own `Probev2_ICCv4.icc`,
//! a profile deliberately built to make a CMM's behaviour visible.
//!
//! **The clause permits declining. It does not require silence.** A
//! caller that cannot tell an author-preferred transform was present and
//! declined has no way to explain a 33 `L*` difference against another
//! engine, and no way to decide whether to route the document elsewhere.
//!
//! ## ★★ The disclosure did not exist, and the docs said it did
//!
//! `transform.rs`'s module header claimed the deviation was *"recorded
//! here and in the model's `notes`"*. **There was no `notes` field and
//! no disclosure of any kind.** The `D2Bx`/`B2Dx` tags appeared in
//! `inspect`'s tag dump like any other tag, with nothing to say they had
//! been declined.
//!
//! That is the third instance in one session of a documented behaviour
//! that was never implemented, and the mechanism is consistent: **the
//! assertion in the doc is *why* nobody looked.** A stated safeguard is
//! read as a present safeguard.
//!
//! ## ★ The row that matters is the CONTROL
//!
//! Three of the four checks below are satisfied by printing a string.
//! Only [`disclosure_is_SILENT_on_a_profile_without_mpet_tags`] proves
//! the disclosure is conditional — and **an unconditional notice
//! discloses nothing.** It is the reason this file is not merely
//! decorative.
//!
//! Corpus-absent → **SKIP loudly, assert nothing.**

use iccce_cmm::matrix_trc::Intent;
use iccce_cmm::transform::{Chain, Destination, Side, mpet_deviation_for};
use iccce_profile::Profile;

fn corpus_root() -> std::path::PathBuf {
    let root = std::env::var("ICCCE_PRIVATE_FIXTURES")
        .unwrap_or_else(|_| r"D:\Dev\iccce-private-fixtures".to_string());
    std::path::Path::new(&root).join("color-org")
}

fn load(name: &str) -> Option<Profile> {
    let p = corpus_root().join(name);
    if !p.is_file() {
        return None;
    }
    let bytes = std::fs::read(&p).ok()?;
    Profile::parse(&bytes).ok()
}

/// The only profile in either corpus carrying `mpet` tags. ICC's own CMM
/// probe, whose `B2D0/1/2` return red / green / blue precisely so that a
/// CMM taking step a) is unmistakable.
const WITH_MPET: &str = "Probev2_ICCv4.icc";
/// A profile with no `mpet` tags at all — the control.
const WITHOUT_MPET: &str = "sRGB2014.icc";

/// Checks 1–3: a disclosure exists, names the declined tags **by
/// signature**, and **cites the clause**.
///
/// The signature requirement is not pedantry. "This profile has
/// multiProcessingElements tags" does not tell a caller *which* — and a
/// profile may carry `D2B0` without `D2B1`, so which intents are
/// affected depends on exactly which tags are present.
///
/// The clause citation is what makes this a **conformant decline**
/// rather than a defect report. Without it a reader cannot tell whether
/// iccce is admitting a gap or accusing the profile.
#[test]
fn disclosure_names_the_declined_tags_and_cites_the_clause() {
    let Some(profile) = load(WITH_MPET) else {
        eprintln!(
            "SKIP disclosure_names_the_declined_tags_and_cites_the_clause: {WITH_MPET} \
                   absent. This test asserted NOTHING."
        );
        return;
    };

    let dev = mpet_deviation_for(&profile, Side::Source)
        .expect("a profile carrying D2Bx/B2Dx MUST produce a deviation disclosure");

    // 2. named by signature, all six.
    assert_eq!(
        dev.declined.len(),
        6,
        "expected all six mpet tags to be named, got {:?}",
        dev.declined
    );
    let rendered = dev.to_string();
    for sig in ["D2B0", "D2B1", "D2B2", "B2D0", "B2D1", "B2D2"] {
        assert!(
            rendered.contains(sig),
            "the disclosure must name {sig} by signature; got: {rendered}"
        );
    }

    // 3. cites the clause.
    assert!(
        dev.clause.contains("8.10.2"),
        "the disclosure must cite ICC.1:2022 8.10.2; got {:?}",
        dev.clause
    );
    assert!(
        rendered.contains("8.10.2"),
        "the rendered disclosure must carry the clause, not only the struct field"
    );

    // And it must say what was done INSTEAD — a disclosure that names
    // only what was declined leaves the caller unable to reason about
    // the result they actually got.
    assert!(
        rendered.contains("step b)"),
        "the disclosure must say which step was taken instead: {rendered}"
    );
    println!("{rendered}");
}

/// ★★★ **THE CONTROL, and the only check here with real power.**
///
/// A disclosure that fires on every profile is not a disclosure — it is
/// a banner, and readers learn to skip banners. This asserts that the
/// notice is **conditional on the tags actually being present**.
///
/// Same shape as Pass H's `the-version-word-ALONE-produces-the-same-refusal`
/// control: the interesting claim is not that the mechanism fires, it is
/// that it *discriminates*.
#[test]
#[allow(non_snake_case)]
fn disclosure_is_SILENT_on_a_profile_without_mpet_tags() {
    let Some(profile) = load(WITHOUT_MPET) else {
        eprintln!(
            "SKIP disclosure_is_SILENT_on_a_profile_without_mpet_tags: {WITHOUT_MPET} \
                   absent. This test asserted NOTHING."
        );
        return;
    };

    // Premise: this profile really has no mpet tags. If a corpus
    // reshuffle replaced it with one that does, the control would pass
    // for the wrong reason, so the premise is asserted rather than
    // assumed.
    let has_mpet = profile.tags.iter().any(|t| {
        matches!(
            t.sig.0,
            0x4432_4230 | 0x4432_4231 | 0x4432_4232 | 0x4232_4430 | 0x4232_4431 | 0x4232_4432
        )
    });
    assert!(
        !has_mpet,
        "{WITHOUT_MPET} now carries mpet tags — it is no longer a valid control and this test \
         has stopped discriminating"
    );

    assert!(
        mpet_deviation_for(&profile, Side::Source).is_none(),
        "the 8.10.2 disclosure fired on a profile with no mpet tags — an unconditional notice \
         discloses nothing"
    );
}

/// The disclosure survives into a built [`Chain`], on both the profile
/// path and the constructed-destination path.
///
/// ★ Tested separately from `mpet_deviation_for` because a caller
/// reaches it through the chain, and a detector that works while nothing
/// calls it is the defect this whole file exists to close.
#[test]
fn chain_carries_the_deviation_on_both_destination_paths() {
    let (Some(with), Some(without)) = (load(WITH_MPET), load(WITHOUT_MPET)) else {
        eprintln!(
            "SKIP chain_carries_the_deviation_on_both_destination_paths: corpus absent. \
                   This test asserted NOTHING."
        );
        return;
    };

    // Caller-supplied destination.
    let chain =
        Chain::with_destination(&with, Destination::Profile(&without), Intent::MediaRelative)
            .expect("builds");
    assert_eq!(
        chain.spec_deviations().len(),
        1,
        "exactly one deviation expected (source side only): {:?}",
        chain.spec_deviations()
    );
    assert_eq!(chain.spec_deviations()[0].side, Side::Source);

    // Constructed destination — the destination has no tags at all, so
    // it cannot contribute a deviation and must not invent one.
    let chain =
        Chain::with_destination(&with, Destination::None, Intent::MediaRelative).expect("builds");
    assert_eq!(
        chain.spec_deviations().len(),
        1,
        "the constructed destination has no tags and must not add a deviation"
    );

    // And the control, through the chain: a chain with no mpet anywhere
    // reports nothing.
    let clean = Chain::with_destination(&without, Destination::None, Intent::MediaRelative)
        .expect("builds");
    assert!(
        clean.spec_deviations().is_empty(),
        "a chain with no mpet tags on either side must report no deviations, got {:?}",
        clean.spec_deviations()
    );
}

/// ★ The deviation is a **disclosure, not an error**: the chain builds
/// and converts normally.
///
/// Worth asserting because the obvious over-correction — refusing a
/// profile whose preferred path iccce cannot take — would be strictly
/// worse than the silence it replaced. Step b) is conformant and the
/// result is usable.
#[test]
fn the_deviation_does_not_prevent_conversion() {
    let Some(with) = load(WITH_MPET) else {
        eprintln!(
            "SKIP the_deviation_does_not_prevent_conversion: corpus absent. \
                   This test asserted NOTHING."
        );
        return;
    };
    let chain = Chain::with_destination(&with, Destination::None, Intent::MediaRelative)
        .expect("a profile with mpet tags must still build via step b)");
    assert!(!chain.spec_deviations().is_empty());

    let input = vec![0.3_f64; chain.input_channels()];
    let out = chain.convert(&input).expect("and must still convert");
    assert_eq!(out.len(), 3);
    for v in &out {
        assert!(
            v.is_finite() && (-0.001..=1.001).contains(v),
            "conversion via step b) produced {out:?}"
        );
    }
    println!("{WITH_MPET}: {} channels in -> RGB {out:?}", input.len());
}
