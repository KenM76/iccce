//! # The buffer path and the single-pixel path must give the same answer
//!
//! ## What this exists to catch, and why nothing else could
//!
//! [`CompiledTransform`] offers two evaluation surfaces:
//!
//! - [`CompiledTransform::convert`] — one pixel, `&[f64] -> &mut [f64]`
//! - [`CompiledTransform::convert_buffer`] — `n` interleaved pixels
//!
//! They are documented as the same transform applied to a different
//! number of pixels. **A caller is entitled to assume that**, and a
//! renderer will reach for the second one, because that is what the
//! doc comment on it says it is for: *"a raster loop is exactly where a
//! panic is least welcome."*
//!
//! ★★★ **They disagreed.** `convert` applied the K-preservation policy
//! before the grid; `convert_buffer` called `grid.eval` directly and
//! never consulted the policy at all. So the same `CompiledTransform`,
//! built with [`Chain::with_black_preservation`], returned a **preserved
//! black** one pixel at a time and an **unpreserved, grid-interpolated
//! black** for the identical values passed as a buffer.
//!
//! The magnitude is not incidental. `NC-269` measured the perceptual
//! cost of black preservation at **3.681203 ΔE2000 max / 1.580674
//! mean** for a real press pair — that is the size of the answer the
//! buffer path was silently discarding. A caller who asked for
//! preservation, got a compiled transform, and rendered a page with it
//! received the answer they had explicitly asked not to have, and the
//! page looked entirely normal. Project rule 1.
//!
//! ## ★ Why the existing suite was green while this was true
//!
//! `compiled_black_preservation_convergence.rs` has three tests and a
//! `difftest` section drives the CLI. **Every one of them evaluates
//! through `convert`.** The buffer path had no test of any kind, and a
//! test that exercises a subject through one of its two entry points
//! covers one of them. This is the shape `pdfce` reported on
//! 2026-08-21 from the other direction — *"a test that exercises its
//! subject without covering it"* — and the instrument that finds it is
//! the same one: **compare two things that are supposed to agree, and
//! make the comparison the assertion.**
//!
//! ## Evidence class
//!
//! **Self-consistency, and deliberately so.** Both arms are iccce. This
//! test asserts nothing about whether either answer is colorimetrically
//! correct — that is `difftest`'s job against lcms2, and it is a
//! cross-check against another implementation rather than ground truth.
//! What is asserted here is narrower and stronger: **two functions on
//! one struct, documented as the same transform, return the same
//! numbers.** An equality claim needs no tolerance justification because
//! it admits no tolerance — the two paths run the same arithmetic on the
//! same inputs, so anything but bit-equality is a structural difference,
//! not a numerical one.
//!
//! That is why the assertion below is `==` and not a ΔE bound. A
//! tolerance here would be a place for the defect to hide.

use iccce_cmm::black_preserve::KMapping;
use iccce_cmm::compiled::CompiledTransform;
use iccce_cmm::matrix_trc::Intent;
use iccce_cmm::transform::Chain;
use iccce_profile::Profile;

/// The committed synthetic CMYK profile whose `B2A` puts chromatic ink
/// into neutrals by construction, so a preserved answer and an
/// unpreserved one are numerically far apart on the K ramp.
///
/// ★ `NC-278` disqualified this fixture for measuring the *perceptual*
/// cost of preservation, because its black is spectrally neutral and a
/// preserved answer is therefore a metamer of the four-ink separation —
/// colorimetrically identical however much ink separates them. **That
/// objection does not reach this test.** Nothing here is a colour
/// claim: this compares two functions' output *numbers* for equality.
/// A metamer has different ink values, and ink values are exactly what
/// is being compared.
fn fixture() -> Option<Profile> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/synthetic/v2-cmyk-chromatic-neutral.icc");
    let bytes = std::fs::read(path).ok()?;
    Profile::parse(&bytes).ok()
}

fn preserving_transform(points: usize) -> Option<CompiledTransform> {
    let profile = fixture()?;
    let chain = Chain::new(&profile, &profile, Intent::MediaRelative)
        .expect("chain builds")
        .with_black_preservation(KMapping::EqualLightness)
        .expect("CMYK->CMYK with a monotonic K ramp: preservation is applicable");
    Some(CompiledTransform::new(&chain, points).expect("compiles"))
}

/// The K ramp, which is where the policy fires and therefore where the
/// two paths can differ. Exact zeros in C, M and Y — "K-only" admits no
/// small amount of chromatic ink, per `KPreserve::apply`.
fn k_only_ramp() -> Vec<[f64; 4]> {
    (0..=10)
        .map(|ki| [0.0, 0.0, 0.0, f64::from(ki) / 10.0])
        .collect()
}

/// Probes that do NOT qualify, so both paths must run the grid. This is
/// the control: if it ever fails, the disagreement is not about the
/// policy and the diagnosis below is wrong.
fn non_qualifying_probes() -> Vec<[f64; 4]> {
    let mut v = Vec::new();
    for ci in 1..=4 {
        for ki in 0..=4 {
            let c = f64::from(ci) / 5.0;
            let k = f64::from(ki) / 4.0;
            v.push([c, c * 0.7, c * 0.4, k]);
        }
    }
    v
}

/// Evaluate `probes` both ways and return the largest absolute
/// disagreement, together with the input that produced it.
fn worst_disagreement(compiled: &CompiledTransform, probes: &[[f64; 4]]) -> (f64, [f64; 4]) {
    let inn = compiled.input_channels();
    let outn = compiled.output_channels();

    // One flat interleaved buffer holding every probe, which is how a
    // renderer would actually call this.
    let mut src = Vec::with_capacity(probes.len() * inn);
    for p in probes {
        src.extend_from_slice(&p[..inn]);
    }
    let mut buffered = vec![0.0f64; probes.len() * outn];
    assert!(
        compiled.convert_buffer(&src, &mut buffered),
        "convert_buffer accepted the shape"
    );

    let mut single = vec![0.0f64; outn];
    let mut worst = 0.0f64;
    let mut worst_at = probes[0];
    for (i, p) in probes.iter().enumerate() {
        assert!(
            compiled.convert(&p[..inn], &mut single),
            "convert evaluates"
        );
        for c in 0..outn {
            let d = (single[c] - buffered[i * outn + c]).abs();
            if d > worst {
                worst = d;
                worst_at = *p;
            }
        }
    }
    (worst, worst_at)
}

#[test]
fn buffer_path_applies_black_preservation_exactly_as_the_single_pixel_path_does() {
    let Some(compiled) = preserving_transform(17) else {
        eprintln!("SKIP: committed synthetic fixture not readable");
        return;
    };

    let (worst, at) = worst_disagreement(&compiled, &k_only_ramp());
    eprintln!("K-only ramp: worst |convert - convert_buffer| = {worst:.6e} at {at:?}");

    // ★ Equality, not a tolerance. Both paths run the same arithmetic
    // on the same inputs; any difference at all is a different code
    // path, which is the thing under test.
    assert_eq!(
        worst, 0.0,
        "the buffer path and the single-pixel path disagree by {worst:.6e} at input {at:?}. \
         On a transform built with black preservation that is the policy being applied by \
         one entry point and skipped by the other — the caller asked for preservation and \
         a raster loop silently did not get it."
    );
}

#[test]
fn the_buffer_path_preserves_a_qualifying_input_on_its_own_terms() {
    // Asserted on OUTPUT rather than by comparison, so this test still
    // means something if BOTH paths regress together. The predicate is
    // "C = M = Y = 0 exactly"; the promise is that the answer carries
    // that through.
    let Some(compiled) = preserving_transform(17) else {
        eprintln!("SKIP: committed synthetic fixture not readable");
        return;
    };
    let probes = k_only_ramp();
    let inn = compiled.input_channels();
    let outn = compiled.output_channels();

    let mut src = Vec::with_capacity(probes.len() * inn);
    for p in &probes {
        src.extend_from_slice(&p[..inn]);
    }
    let mut dst = vec![0.0f64; probes.len() * outn];
    assert!(compiled.convert_buffer(&src, &mut dst));

    for (i, p) in probes.iter().enumerate() {
        let o = i * outn;
        assert_eq!(dst[o], 0.0, "cyan out, buffer path, at K={}", p[3]);
        assert_eq!(dst[o + 1], 0.0, "magenta out, buffer path, at K={}", p[3]);
        assert_eq!(dst[o + 2], 0.0, "yellow out, buffer path, at K={}", p[3]);
    }
}

#[test]
fn the_control_holds_where_the_policy_never_fires() {
    // If this fails, the disagreement above is not about the policy and
    // the whole diagnosis needs redoing. It is here so a green result
    // above is attributable.
    let Some(compiled) = preserving_transform(17) else {
        eprintln!("SKIP: committed synthetic fixture not readable");
        return;
    };
    let (worst, at) = worst_disagreement(&compiled, &non_qualifying_probes());
    eprintln!("non-qualifying control: worst = {worst:.6e} at {at:?}");
    assert_eq!(
        worst, 0.0,
        "the two paths disagree by {worst:.6e} at {at:?} on an input that does NOT \
         qualify for preservation — so the difference is not the policy and this \
         test file's diagnosis is wrong"
    );
}

#[test]
fn the_two_paths_agree_when_no_policy_was_requested() {
    // The regression guard for every caller who never asks for
    // preservation: whatever is done to reunite the paths must not
    // change the ordinary answer.
    let Some(profile) = fixture() else {
        eprintln!("SKIP: committed synthetic fixture not readable");
        return;
    };
    let chain = Chain::new(&profile, &profile, Intent::MediaRelative).expect("chain builds");
    let compiled = CompiledTransform::new(&chain, 17).expect("compiles");

    let mut probes = k_only_ramp();
    probes.extend(non_qualifying_probes());
    let (worst, at) = worst_disagreement(&compiled, &probes);
    eprintln!("no policy: worst = {worst:.6e} at {at:?}");
    assert_eq!(worst, 0.0, "unpreserved paths disagree at {at:?}");
}
