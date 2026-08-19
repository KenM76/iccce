//! # The two readings of sRGB are BOTH reachable through the public API,
//! and selecting the non-default actually changes the answer
//!
//! ## Why this file exists
//!
//! Two currently-in-force standards define sRGB's transfer function with
//! different constants (`ICC_Spec` register row **`A57`**, filed OPEN;
//! see [`iccce_cmm::builtin::SrgbTrc`] for the full account). The
//! operator's standing instruction, 2026-08-19, is that a source
//! disagreement ships as **both readings, selectable, with a reasoned
//! default** rather than one reading silently chosen.
//!
//! That instruction has a failure mode, and it is the reason for this
//! file: **an option that is never exercised rots into a trap.** A user
//! who selects it then gets whatever the last refactor left behind, which
//! is worse than never having offered the choice. Two specific ways the
//! option could be hollow while every other test in the suite stays
//! green:
//!
//! 1. **[`Destination::BuiltinSrgb`] could ignore its argument** and
//!    build the default either way. Nothing else would notice — the
//!    default path is what all 193 other tests exercise.
//! 2. **The two variants could collapse** into the same constants, making
//!    the enum decorative. `builtin.rs`'s own unit tests guard the
//!    constants; this file guards the **plumbing between the enum and a
//!    built [`Chain`]**, which is a different thing and is where an
//!    argument gets dropped.
//!
//! ## What is asserted
//!
//! | assertion | what it catches |
//! |---|---|
//! | `BuiltinSrgb(ValueContinuous)` is **bit-identical** to `None` | the default spelling silently drifting away from the documented equivalence |
//! | `BuiltinSrgb(SlopeContinuous)` **differs** from `None` | the argument being dropped — the hollow-option failure |
//! | the difference is **small but non-zero**, within a stated window | the variants collapsing, AND the option quietly becoming visible |
//! | provenance still reports `BuiltInSrgb` | the disclosure surviving; a substituted destination must stay disclosed however it was selected |
//!
//! ## ★ Evidence class, and what is NOT claimed
//!
//! **Self-comparison** — iccce's two curves against each other, through
//! iccce's own chain. That is the correct and only available class here:
//! the question is *"does selecting the option change the output, and by
//! how much"*, which is intrinsically a comparison of the library with
//! itself.
//!
//! ★ **Nothing here claims either reading is correct.** `A57` is OPEN,
//! nobody in this project has read IEC 61966-2-1's normative text, and
//! the fact that **lcms2 implements the default reading** (measured in
//! Pass L: `23.2×` closer, `0 of 204` resolvable probes favouring the
//! alternative) is a **fact about lcms2**, not a resolution of the
//! standards question.
//!
//! ## The window, and why it is a window rather than a bound
//!
//! The separation is asserted to lie **strictly inside** a range, not
//! merely below a ceiling. A lower bound is unusual in this project and
//! it is deliberate: without it, a regression that collapsed the two
//! variants into one would produce a separation of exactly zero and
//! **pass a ceiling-only assertion**. The upper bound catches the option
//! becoming perceptible; the lower bound catches it becoming decorative.
//! Both failures are silent, and the second is the more likely.

use iccce_cmm::builtin::SrgbTrc;
use iccce_cmm::matrix_trc::Intent;
use iccce_cmm::transform::{Chain, Destination, DestinationProvenance};
use iccce_profile::Profile;

/// A committed source profile that is known to build against the
/// built-in destination.
fn source() -> Profile {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/synthetic/v2-rgb-header-intent-relative.icc");
    let bytes = std::fs::read(&path)
        .unwrap_or_else(|e| panic!("fixture {} must be readable: {e}", path.display()));
    Profile::parse(&bytes).expect("the fixture must parse")
}

/// Convert a small deterministic probe set through a chain.
fn probe(dst: Destination<'_>) -> Vec<Vec<f64>> {
    let src = source();
    let chain = Chain::with_destination(&src, dst, Intent::MediaRelative)
        .expect("the chain must build against the built-in destination");
    assert_eq!(
        chain.destination_provenance(),
        DestinationProvenance::BuiltInSrgb,
        "however the built-in destination was selected, the substitution must \
         stay DISCLOSED — a silently substituted destination is the exact \
         failure this provenance field exists to prevent"
    );
    // Probes deliberately include values near the breakpoints, where the
    // two readings differ most, and mid-tones, where they barely differ.
    let mut out = Vec::new();
    for i in 0..=40u32 {
        let v = f64::from(i) / 40.0;
        out.push(
            chain
                .convert(&[v, v * 0.5, 1.0 - v])
                .expect("the conversion must run"),
        );
    }
    out
}

/// The documented equivalence: `None` IS
/// `BuiltinSrgb(ValueContinuous)`.
///
/// Bit-identical, not approximately equal — they are meant to be the same
/// code path, so anything less than exact equality means they have drifted
/// and the doc comment on [`Destination::BuiltinSrgb`] has become false.
#[test]
fn the_default_spelling_and_the_named_default_are_the_same_answer() {
    let a = probe(Destination::None);
    let b = probe(Destination::BuiltinSrgb(SrgbTrc::ValueContinuous));
    assert_eq!(a.len(), b.len());
    for (i, (x, y)) in a.iter().zip(b.iter()).enumerate() {
        for (j, (p, q)) in x.iter().zip(y.iter()).enumerate() {
            assert_eq!(
                p.to_bits(),
                q.to_bits(),
                "probe {i} channel {j}: `Destination::None` and \
                 `BuiltinSrgb(ValueContinuous)` must be the SAME path, got \
                 {p:.17e} vs {q:.17e}"
            );
        }
    }
}

/// ★ The load-bearing one: selecting the non-default **changes the
/// answer**, and by an amount inside a stated window.
///
/// If this fails with a separation of exactly zero, the argument is being
/// dropped somewhere between [`Destination::BuiltinSrgb`] and the
/// constructed curve, and the option is hollow.
#[test]
fn selecting_the_slope_continuous_reading_actually_changes_the_output() {
    let a = probe(Destination::None);
    let b = probe(Destination::BuiltinSrgb(SrgbTrc::SlopeContinuous));

    let mut worst = 0.0_f64;
    for (x, y) in a.iter().zip(b.iter()) {
        for (p, q) in x.iter().zip(y.iter()) {
            worst = worst.max((p - q).abs());
        }
    }

    assert!(
        worst > 1.0e-9,
        "separation {worst:.6e} is effectively zero — `Destination::BuiltinSrgb` \
         is ignoring its argument, or the two variants have collapsed. The \
         option is hollow, which is worse than not offering it"
    );
    assert!(
        worst < 1.0e-2,
        "separation {worst:.6e} is far larger than the two readings can \
         explain (the curves differ by at most 9.76e-6 in the encoded domain). \
         Something other than the transfer function changed"
    );
}
