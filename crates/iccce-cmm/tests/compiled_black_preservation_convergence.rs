//! # The compiled path must not interpolate across the preservation step
//!
//! ## The defect this exists to prevent, stated as it was measured
//!
//! [`CompiledTransform`] samples a chain onto a uniform lattice and
//! interpolates between nodes. Black preservation introduces a **step
//! discontinuity at `C = M = Y = 0`** — a qualifying input is answered
//! from a different rule than its immediate neighbours.
//!
//! The first implementation sampled the *preserving* conversion. That
//! put preserved values on the `C = 0` face of the grid and ordinary
//! colorimetric values on the next node in, so every point between them
//! came back a **blend of two answers to different questions**.
//! `icc-conformance` measured it on 2026-08-18:
//!
//! ```text
//!                                        grid 17     grid 33
//!   max |compiled - reference| near axis  0.617121    0.617148
//!   the same, far from the axis (control) 1.138e-3    5.34e-4
//! ```
//!
//! ★★★ **The control halved and the near-axis error did not move.** That
//! is `O(1)` beside `O(h^1.32)`, and it is the whole diagnosis: an error
//! that does not shrink under grid refinement **is not an interpolation
//! error**. At grid 33 it was 1156× the control. The direction was
//! over-application — the compiled path preserved pixels that did not
//! qualify.
//!
//! ## Why this test asserts CONVERGENCE and not a bound
//!
//! A fixed threshold here would be a number someone chose. The defect's
//! signature was **failure to converge**, so convergence is what is
//! asserted: refining the grid must reduce the near-axis disagreement.
//! Under the defect that was impossible — `0.617121 → 0.617148` is a
//! *rise*. Under the fix the near-axis behaviour is ordinary
//! interpolation and must improve like everything else.
//!
//! This is deliberately a weaker claim than "the error is below X" and a
//! much harder one to satisfy accidentally: a wrong constant passes a
//! threshold test, and nothing passes a convergence test by luck.
//!
//! ## Evidence class
//!
//! **Self-consistency.** Both arms are iccce — the compiled path against
//! the reference chain path. No oracle, no published value, and this
//! asserts nothing about whether either arm is colorimetrically right.
//! It asserts that compiling a transform does not change its answer,
//! which is a different and narrower claim.

use iccce_cmm::black_preserve::KMapping;
use iccce_cmm::compiled::CompiledTransform;
use iccce_cmm::matrix_trc::Intent;
use iccce_cmm::transform::Chain;
use iccce_profile::Profile;

/// The committed synthetic CMYK profile whose `B2A` puts chromatic ink
/// into neutrals by construction — the fixture built precisely so this
/// subject is measurable without the licensed corpus.
fn fixture() -> Option<Profile> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/synthetic/v2-cmyk-chromatic-neutral.icc");
    let bytes = std::fs::read(path).ok()?;
    Profile::parse(&bytes).ok()
}

/// Largest disagreement between the compiled and reference paths over a
/// set of probes.
fn max_disagreement(chain: &Chain, compiled: &CompiledTransform, probes: &[[f64; 4]]) -> f64 {
    let mut worst: f64 = 0.0;
    let mut out = vec![0.0f64; compiled.output_channels()];
    for p in probes {
        let reference = chain.convert(p).expect("reference path evaluates");
        assert!(compiled.convert(p, &mut out), "compiled path evaluates");
        for (a, b) in reference.iter().zip(out.iter()) {
            worst = worst.max((a - b).abs());
        }
    }
    worst
}

/// Probes within one cell of the neutral axis, where the step lives.
///
/// `c` is deliberately **never zero** — a qualifying input takes the
/// branch in both paths and would agree trivially. The interesting
/// region is the one the interpolator has to cross.
#[allow(clippy::cast_precision_loss)] // grid_points is 17 or 33, exact in f64
fn near_axis_probes(grid_points: usize) -> Vec<[f64; 4]> {
    let cell = 1.0 / (grid_points - 1) as f64;
    let mut v = Vec::new();
    for ki in 0..=8 {
        let k = ki as f64 / 8.0;
        for frac in [0.05, 0.15, 0.25, 0.5, 0.75, 0.95] {
            let c = cell * frac;
            v.push([c, 0.0, 0.0, k]);
            v.push([c, c, c, k]);
        }
    }
    v
}

/// Probes far from the axis: the control that says what ordinary
/// interpolation error looks like on this fixture.
fn far_probes() -> Vec<[f64; 4]> {
    let mut v = Vec::new();
    for ci in 1..=4 {
        for ki in 0..=4 {
            let c = ci as f64 / 5.0;
            let k = ki as f64 / 4.0;
            v.push([c, c * 0.7, c * 0.4, k]);
        }
    }
    v
}

#[test]
fn near_axis_error_converges_under_grid_refinement() {
    let Some(profile) = fixture() else {
        eprintln!("SKIP: committed synthetic fixture not readable");
        return;
    };
    let build = |points: usize| {
        let chain = Chain::new(&profile, &profile, Intent::MediaRelative)
            .expect("chain builds")
            .with_black_preservation(KMapping::EqualLightness)
            .expect("CMYK->CMYK, monotonic K ramp: preservation is applicable");
        let compiled = CompiledTransform::new(&chain, points).expect("compiles");
        let near = max_disagreement(&chain, &compiled, &near_axis_probes(points));
        let far = max_disagreement(&chain, &compiled, &far_probes());
        (near, far)
    };

    let (near17, far17) = build(17);
    let (near33, far33) = build(33);

    eprintln!("grid 17: near-axis {near17:.6e}  control {far17:.6e}");
    eprintln!("grid 33: near-axis {near33:.6e}  control {far33:.6e}");

    // The control must improve, or the fixture is not exercising
    // interpolation at all and the comparison below is meaningless.
    assert!(
        far33 < far17,
        "control did not converge ({far17:.6e} -> {far33:.6e}); \
         this test cannot distinguish anything"
    );

    // ★ The assertion. Under the defect this was 0.617121 -> 0.617148,
    // a RISE, while the control halved.
    assert!(
        near33 < near17,
        "near-axis disagreement did not converge under refinement: \
         {near17:.6e} -> {near33:.6e}. That is the signature of the \
         compiled grid interpolating across the preservation step \
         rather than carrying the policy outside it."
    );
}

#[test]
fn a_qualifying_input_is_preserved_by_the_compiled_path_too() {
    // The predicate itself, through the compiled path: the whole point
    // of carrying the policy outside the grid is that the exact test
    // still means what it says. Asserted on OUTPUT, at exact zero,
    // because "K-only" admits no small amount of chromatic ink.
    let Some(profile) = fixture() else {
        eprintln!("SKIP: committed synthetic fixture not readable");
        return;
    };
    let chain = Chain::new(&profile, &profile, Intent::MediaRelative)
        .expect("chain builds")
        .with_black_preservation(KMapping::EqualLightness)
        .expect("preservation applicable");
    let compiled = CompiledTransform::new(&chain, 17).expect("compiles");
    let mut out = vec![0.0f64; compiled.output_channels()];
    for ki in 0..=10 {
        let k = f64::from(ki) / 10.0;
        assert!(compiled.convert(&[0.0, 0.0, 0.0, k], &mut out));
        assert_eq!(out[0], 0.0, "cyan at K={k}");
        assert_eq!(out[1], 0.0, "magenta at K={k}");
        assert_eq!(out[2], 0.0, "yellow at K={k}");
    }
}

#[test]
fn without_the_policy_the_compiled_path_is_unchanged() {
    // The regression guard for everyone who does NOT ask for
    // preservation: splitting `convert` must not have altered the
    // ordinary compiled answer. Both arms here are unpreserved.
    let Some(profile) = fixture() else {
        eprintln!("SKIP: committed synthetic fixture not readable");
        return;
    };
    let chain = Chain::new(&profile, &profile, Intent::MediaRelative).expect("chain builds");
    let compiled = CompiledTransform::new(&chain, 17).expect("compiles");
    let mut out = vec![0.0f64; compiled.output_channels()];
    // On the K-only ramp with no policy, the compiled path must return
    // the ordinary colorimetric answer — chromatic ink and all.
    assert!(compiled.convert(&[0.0, 0.0, 0.0, 1.0], &mut out));
    let reference = chain.convert(&[0.0, 0.0, 0.0, 1.0]).expect("evaluates");
    for (a, b) in reference.iter().zip(out.iter()) {
        assert!(
            (a - b).abs() < 1e-9,
            "unpreserved compiled path diverged from the chain: {a} vs {b}"
        );
    }
    assert!(
        reference[0] > 0.0,
        "this fixture is supposed to contaminate neutrals; if it does not, \
         the test above proves nothing"
    );
}
