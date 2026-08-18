//! # Black preservation — K-only, and the two rival definitions of "preserve"
//!
//! A CMYK→CMYK conversion that routes every colour through the PCS
//! destroys the black separation. This module is the policy that stops
//! it doing so for the one case where the requirement is unambiguous:
//! **an input carrying black alone must produce an output carrying
//! black alone.**
//!
//! ## ★★★ This implements NO standard, and that is the first thing to know
//!
//! **ICC.1 contains no black-preservation construct.** Verified
//! exhaustively over ICC.1:2022 (v4.4) and ICC.1:2001-04 (v2.3), whole
//! document, two extraction engines: zero occurrences of any phrasing of
//! black preservation, GCR, grey-component replacement, or K-only.
//! Recorded as `ICC_Spec` ambiguity-register entry **A51**, a *closed
//! negative* rather than an open question.
//!
//! **The reason is structural and worth stating, because it explains why
//! no future ICC edition is likely to contain one either:** the PCS is
//! three components (ICC.1:2022 clauses 0.3, 6.3.2, Annex D). Every
//! device→device transform is therefore 4→3→4, and **K has no carrier**.
//! ICC could not express black preservation without introducing a new
//! connection space.
//!
//! ICC's only black construct ever was v2's `ucrbgTag` (`'bfd '`,
//! ICC.1:2001-04 clause 6.4.45), which disclaims its own participation —
//! *"This tag provides descriptive information only and is not involved
//! in the processing model"* — and was **deleted in v4**.
//!
//! The harm, by contrast, is named — but by PDF, not by ICC:
//! **ISO 32000-1/2 clause 8.6.5.7 NOTE 2**, a 4→3→4 conversion *"is
//! unnecessary and results in a loss of fidelity in the black
//! component."*
//!
//! So everything below is a **named policy** (project rule 4), not an
//! implementation of a specification, and the caller opts into it
//! explicitly. It is never applied by default and never inferred.
//!
//! ## What is settled and what is not
//!
//! The requirement splits cleanly in two, and only one half has an
//! unambiguous answer:
//!
//! | half | status |
//! |---|---|
//! | the **chromatic** channels: `C = M = Y = 0` in ⇒ `C = M = Y = 0` out | **settled by definition.** There is no reading of "K-only" under which the output carries chromatic ink |
//! | the **K value** to emit | ★ **NOT settled. Two published definitions disagree** |
//!
//! ### The two rivals, and why iccce implements both rather than choosing
//!
//! ```text
//!   EqualLightness  lcms2's construction: map K so the destination's
//!                   K-only ramp matches the source's in L*.
//!                   VENDOR. No normative text. It is also our ORACLE.
//!
//!   Ratio           Cholewo (2000): preserve the K_MIN/K_MAX ratio.
//!                   PUBLISHED, peer-reviewed, and NOT what lcms2 does.
//! ```
//!
//! **Two definitions share one name in the literature.** A cross-check
//! against lcms2 is only meaningful if both sides compute the same
//! quantity, so a library that silently picked one and called it "black
//! preservation" would make every future comparison uninterpretable —
//! agreement and disagreement would look identical.
//!
//! Measured distance between the two policies on this project's own
//! corpus (`docs/TOLERANCES.md` §3.10, Pass K §D):
//!
//! ```text
//!   ISO Coated v2 300% -> itself             6.1e-5    (same press)
//!   ISO Coated v2 300% -> Coated FOGRA39     1.165e-3
//!   ISO Coated v2 300% -> Coated FOGRA27     1.4296e-2
//!   ISO Coated v2 300% -> GWG_GenericCMYK    4.8899e-2 (cross press)
//! ```
//!
//! ★ **Read that table before assuming the choice is academic.** On a
//! same-press pair the two policies are three orders of magnitude closer
//! than on a cross-press pair. **A test corpus of same-press pairs would
//! make this look like a non-decision**, which is exactly why Pass K
//! grades it on four pairs and not one.
//!
//! ## The exact-zero test, and its cost
//!
//! [`KPreserve::apply`] recognises a K-only input by testing the three
//! chromatic channels against **exact zero**, with no tolerance. This
//! matches lcms2 (`cmscnvrt.c`, `BlackPreservingKOnlyIntents`: the test
//! is `In[0] == 0 && In[1] == 0 && In[2] == 0`).
//!
//! **It is a choice with a measurable consequence and it is stated
//! rather than buried.** An input at `C = 1e-9` is not K-only under this
//! rule and takes the ordinary colorimetric path, so the transform has a
//! **discontinuity at C = 0**. The alternative — snapping near-neutrals
//! to K-only over some width — is a *different* behaviour, not a
//! refinement of this one, and Pass K's
//! `near-neutral-transition-width` row exists precisely to tell the two
//! apart. lcms2's own K-only region is **exactly one cell of its 17-node
//! CLUT** (width `1/16`), which is a consequence of its CLUT
//! construction rather than a stated rule.
//!
//! iccce's width under this module is **zero** — the single point
//! `C = 0`. That is reported by Pass K, not asserted here.
//!
//! ## What this module does NOT do
//!
//! - **It is not `DeviceGray` → CMYK.** That mapping is normative **PDF**
//!   (ISO 32000-1 clause 10.3.3 = ISO 32000-2 clause 10.4.2.3,
//!   `c = m = y = 0`, `k = 1 − gray`), routed there even for an
//!   ICC-enabled processor by ISO 32000-2 clause 10.3.2. It belongs to
//!   the PDF processor and not to a CMM. See `ARCHITECTURE.md` DL-059,
//!   which records this project having mis-filed it in the direction of
//!   claiming it.
//! - **It is not K-plane preservation.** Preserving the K value of a
//!   *general* CMYK colour while re-computing CMY is a strictly larger
//!   problem (it needs the destination's admissible black range at the
//!   target colour) and is not attempted here. The narrow K-only case is
//!   implemented because its requirement is unambiguous; the general
//!   case is not, and guessing at it would be indistinguishable from
//!   implementing it.
//! - **It is not applied to the absolute intent.** See
//!   [`crate::transform::ChainError::BlackPreserveNotApplicable`].

use crate::transform::SourceModel;

/// Which definition of "preserve the black" to apply to the K channel.
///
/// The chromatic half of black preservation is identical under both
/// variants (`C = M = Y = 0` out); these differ **only** in what K value
/// accompanies it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KMapping {
    /// **lcms2's construction — equal lightness.**
    ///
    /// Map `K_in` to the `K_out` whose destination K-only patch has the
    /// same `L*` as the source's `K_in` patch. Implemented here by
    /// sampling both K ramps through the two profiles' own A2B
    /// directions and inverting the destination's.
    ///
    /// **Evidence class: vendor construction.** lcms2 builds this as a
    /// 4096-entry curve in `_cmsBuildKToneCurve` (`cmscnvrt.c`). There
    /// is **no normative text behind it** — the same posture
    /// `docs/TOLERANCES.md` §3.7 takes for BPC's estimation subset
    /// (A27/A42). lcms2's own tutorial says so directly: *"No, this does
    /// not belong to normal ICC workflow. ICC has tried to address such
    /// need but still there is nothing in the spec."*
    ///
    /// ★ Selecting this makes iccce a **second implementation of a
    /// vendor choice**, and it is the only variant under which a
    /// differential test against lcms2 measures agreement rather than
    /// two different quantities.
    EqualLightness,

    /// **Cholewo's construction — preserve the black ratio.**
    ///
    /// `K_d = K_MIN + (K_MAX − K_MIN)/(K_i,MAX − K_i,MIN) · (K_i − K_i,MIN)`
    ///
    /// Cholewo, T. J., *"Conversion between CMYK spaces preserving black
    /// separation"*, IS&T/SID CIC8, vol. 8, pp. 257–261 (2000),
    /// DOI `10.2352/cic.2000.8.1.art00047`.
    ///
    /// **Evidence class: published literature** — which under project
    /// rule 3 outranks [`Self::EqualLightness`]'s vendor provenance.
    ///
    /// ★★ **A named refusal, and now a SOURCED one.** The paper defines
    /// its own limits — the earlier "after Hung 1994" reading was a
    /// credit misread as a deferral — and the definition is what rules
    /// this out as a policy switch. Cholewo §2.4, verbatim:
    ///
    /// > *"For each color the maximum (K_MAX) and minimum (K_MIN)
    /// > amounts of black with which it can be reproduced are
    /// > determined … K_MAX and K_MIN are found by inverting the device
    /// > model by constraining the solution to have at least one of the
    /// > CMY components equal to zero, and by penalizing K > 0,
    /// > respectively."*
    ///
    /// **They are functions of the target colour against a printer
    /// model, not constants of the press** — four model inversions per
    /// colour, plus a fifth optimisation for the answer. An
    /// implementation that hoists them out of the per-colour loop is
    /// not implementing this method.
    ///
    /// Why it is not one setting of a shared function
    /// (`ICC_Spec` **A53**):
    ///
    /// - it needs a **fitted, continuous, differentiable forward
    ///   printer model in MLAB for both devices**; an ICC profile's
    ///   LUT is not that object;
    /// - `K_d` is a **soft target inside a weighted objective**, not an
    ///   assignment — the optimiser may miss it, and the paper concedes
    ///   it does for "unrealistic values of `K_d`";
    /// - **six weights, three unspecified**, so two faithful
    ///   implementations of the paper will not agree with each other;
    /// - it is an **offline device-link construction procedure**, not a
    ///   runtime transform: *"The obtained interpolation table can be
    ///   embedded in a printer or packaged as an ICC device link
    ///   profile."*
    ///
    /// ## ★★★ What this corrects about our own testing
    ///
    /// If source and destination are the **same** printing condition
    /// the four limits coincide and Eq. (1) reduces to `K_d = K_i`
    /// **for every colour**. So on a same-press pair, *"copy K
    /// through"* is **not a naive candidate — it is Cholewo's answer,
    /// exactly.** Any test naming copy-through as the
    /// plausible-but-wrong rival on a same-press fixture has mislabelled
    /// it, and the distance it reports is
    /// `D(equal-lightness, ratio)` rather than the error of a defect.
    ///
    /// Cross-press on the pure-K axis it does **not** copy through: an
    /// input `(0,0,0,k)` already satisfies the `K_MAX` constraint, the
    /// bracket cancels, and `K_d = K_MAX` of the destination — which
    /// reproduces the paper's own headline claim that black-only areas
    /// *"will be reproduced with the maximum possible black amount for
    /// a requested color."* A genuinely different behaviour from
    /// [`Self::EqualLightness`], not a refinement of it.
    ///
    /// ★ **And that is the sharpest reason this refuses rather than
    /// approximates:** on a same-press pair a wrong Cholewo and a right
    /// one are **indistinguishable by construction**, so the fixture
    /// this project exercises most has *zero* power to catch a bad
    /// approximation.
    ///
    /// Returns [`crate::transform::ChainError::KMappingNotAvailable`].
    Ratio,
}

impl KMapping {
    /// A stable identifier for logs, test row ids and doc comments.
    ///
    /// Deliberately not `Display`: this is the *policy name* that must
    /// appear alongside any measured number so that a reader can tell
    /// which of the two definitions produced it. A number reported
    /// without it is uninterpretable (module doc, "two definitions share
    /// one name").
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::EqualLightness => "equal-lightness (lcms2, vendor)",
            Self::Ratio => "ratio (Cholewo 2000, published)",
        }
    }
}

/// Which side of the chain a refusal is about.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    Source,
    Destination,
}

impl std::fmt::Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Source => "source",
            Self::Destination => "destination",
        })
    }
}

/// Number of samples along the K axis used to build the mapping.
///
/// **Why 1024 and not lcms2's 4096.** lcms2 stores its K tone curve as a
/// `cmsToneCurve` of 4096 entries and interpolates within it. This
/// implementation samples the two A2B models directly and interpolates
/// linearly between samples, so the sample count trades build cost
/// against interpolation error on a curve that is smooth and monotonic
/// by construction. 1024 samples put the linear-interpolation residual
/// on a smooth `L*(K)` ramp **below the 16-bit PCS quantum** that
/// bounds every other number in this crate, so a finer grid would refine
/// a term already dominated by the encoding.
///
/// ★ That is an argument, not a measurement. It is stated here so that
/// the first person to measure it can contradict it with a number.
const K_SAMPLES: usize = 1024;

/// A built K-only preservation policy, ready to apply per-pixel.
///
/// Built once at [`crate::transform::Chain::with_black_preservation`] time — the
/// sampling of both K ramps is a per-*chain* cost, never a per-pixel
/// one, which is the same shape lcms2 uses (it precomputes a tone curve
/// at transform-construction time).
#[derive(Debug, Clone)]
pub struct KPreserve {
    mapping: KMapping,
    /// `curve[i]` is the destination K for a source K of
    /// `i / (K_SAMPLES - 1)`. Monotonic non-decreasing by construction —
    /// [`KPreserve::build`] refuses to produce a non-monotonic one.
    curve: Vec<f64>,
}

impl KPreserve {
    /// The policy this was built under. Carry it into any reported
    /// number.
    #[must_use]
    pub const fn mapping(&self) -> KMapping {
        self.mapping
    }

    /// Build the equal-lightness mapping by sampling both K ramps.
    ///
    /// # Algorithm, and why it is shaped this way
    ///
    /// 1. Sample the **source** K-only ramp `(0, 0, 0, k)` through the
    ///    source's own device→PCS model, taking `L*`.
    /// 2. Sample the **destination** K-only ramp the same way, through
    ///    the destination profile's *A2B* (forward) direction — not its
    ///    B2A. A B2A answers "what device value makes this colour"; here
    ///    we need "what colour does this device value make", which is
    ///    the forward question.
    /// 3. For each source sample, find the destination K whose `L*`
    ///    matches, by linear interpolation between bracketing
    ///    destination samples.
    ///
    /// # Why `L*` and not `Y`
    ///
    /// `L*` is perceptually uniform; `Y` is not. Matching in `Y` would
    /// concentrate the mapping's error where the eye is most sensitive.
    /// This follows lcms2, which stores `1 − L*/100`.
    ///
    /// # Refusals rather than guesses
    ///
    /// Both ramps must be **monotonic in `L*`**. A non-monotonic ramp
    /// makes the inversion ill-posed — there is no single `K_out` for a
    /// given `L*` — and this returns `None` rather than picking one.
    /// lcms2 rejects the same case.
    ///
    /// Monotonicity is tested with a small negative tolerance rather
    /// than at exact zero: a real profile's K ramp can be flat over a
    /// stretch (ink saturating), and flatness is not non-monotonicity.
    /// The tolerance admits `1e-9` of backward movement, which is
    /// numerical noise on a value in `0..=100` and far below the 16-bit
    /// PCS quantum.
    pub(crate) fn build_equal_lightness(
        src: &SourceModel,
        dst_a2b: &SourceModel,
    ) -> Option<KPreserve> {
        let src_l = sample_k_ramp_lightness(src)?;
        let dst_l = sample_k_ramp_lightness(dst_a2b)?;

        // Both ramps run light (K=0) to dark (K=1), so L* is
        // non-INCREASING along each. Reject a ramp that turns around.
        if !is_monotonic_non_increasing(&src_l) || !is_monotonic_non_increasing(&dst_l) {
            return None;
        }

        let mut curve = Vec::with_capacity(K_SAMPLES);
        for &target in &src_l {
            curve.push(invert_lightness(&dst_l, target));
        }

        // The composition of two monotonic maps is monotonic; assert it
        // rather than assume it, because a silently non-monotonic K
        // curve would produce banding that looks like a LUT artefact
        // and would be hunted in the wrong place.
        if !is_monotonic_non_decreasing(&curve) {
            return None;
        }

        Some(KPreserve {
            mapping: KMapping::EqualLightness,
            curve,
        })
    }

    /// Map a source K to a destination K.
    ///
    /// Linear interpolation within the built curve. The curve is dense
    /// (`K_SAMPLES` entries over `0..=1`) and monotonic, so this is a
    /// bounded, branch-light lookup suitable for a per-pixel loop.
    // Every cast here is between a value bounded by K_SAMPLES (1024)
    // and f64. An f64 mantissa is 52 bits, so integers up to 2^53 are
    // represented EXACTLY — 1024 is not close to that bound, and the
    // f64->usize cast is of a value already clamped into [0, len-1], so
    // it can neither truncate meaningfully nor lose a sign.
    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    #[must_use]
    pub fn map_k(&self, k: f64) -> f64 {
        let k = k.clamp(0.0, 1.0);
        let x = k * (self.curve.len() - 1) as f64;
        let i = x.floor() as usize;
        if i + 1 >= self.curve.len() {
            return self.curve[self.curve.len() - 1];
        }
        let t = x - i as f64;
        self.curve[i] * (1.0 - t) + self.curve[i + 1] * t
    }

    /// Apply the policy to one device value, if it qualifies.
    ///
    /// Returns `Some(output)` when the input is K-only and the policy
    /// therefore governs; `None` when the ordinary colorimetric path
    /// must run instead.
    ///
    /// ★ The qualifying test is **exact zero** on the three chromatic
    /// channels — see the module doc for why, and for what it costs.
    #[must_use]
    pub fn apply(&self, device: &[f64]) -> Option<Vec<f64>> {
        if device.len() != 4 {
            return None;
        }
        if device[0] != 0.0 || device[1] != 0.0 || device[2] != 0.0 {
            return None;
        }
        Some(vec![0.0, 0.0, 0.0, self.map_k(device[3])])
    }
}

/// Sample a model's K-only ramp, returning `L*` at each of
/// [`K_SAMPLES`] points.
///
/// Returns `None` if the model is not 4-channel or any sample fails to
/// evaluate — a partially-sampled ramp is not a ramp, and filling the
/// hole would invent colorimetry.
#[allow(clippy::cast_precision_loss)] // i < K_SAMPLES = 1024, exact in f64
fn sample_k_ramp_lightness(m: &SourceModel) -> Option<Vec<f64>> {
    if crate::transform::a2b_channels(m) != 4 {
        return None;
    }
    let mut out = Vec::with_capacity(K_SAMPLES);
    for i in 0..K_SAMPLES {
        let k = i as f64 / (K_SAMPLES - 1) as f64;
        let lab = crate::transform::a2b_to_lab(m, &[0.0, 0.0, 0.0, k])?;
        out.push(lab.l);
    }
    Some(out)
}

/// Tolerance for monotonicity tests on an `L*` ramp (see
/// [`KPreserve::build_equal_lightness`]).
const MONOTONIC_EPS: f64 = 1e-9;

fn is_monotonic_non_increasing(v: &[f64]) -> bool {
    v.windows(2).all(|w| w[1] - w[0] <= MONOTONIC_EPS)
}

fn is_monotonic_non_decreasing(v: &[f64]) -> bool {
    v.windows(2).all(|w| w[0] - w[1] <= MONOTONIC_EPS)
}

/// Find the K (as a fraction of the ramp) whose `L*` equals `target`.
///
/// `ramp` is non-increasing. Values outside the ramp's range clamp to
/// its ends — a source black darker than the destination can reach maps
/// to the destination's maximum K, which is the honest answer: the
/// destination cannot go darker, and reporting a K above 1 would be
/// inventing ink that does not exist.
#[allow(clippy::cast_precision_loss)] // indices < K_SAMPLES = 1024, exact in f64
fn invert_lightness(ramp: &[f64], target: f64) -> f64 {
    let n = ramp.len();
    if target >= ramp[0] {
        return 0.0;
    }
    if target <= ramp[n - 1] {
        return 1.0;
    }
    // Binary search for the bracketing pair. The ramp is non-increasing,
    // so we want the first index whose value is <= target.
    let mut lo = 0usize;
    let mut hi = n - 1;
    while hi - lo > 1 {
        let mid = (lo + hi) / 2;
        if ramp[mid] > target {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let (a, b) = (ramp[lo], ramp[hi]);
    let span = a - b;
    let t = if span.abs() <= MONOTONIC_EPS {
        // A flat stretch: every K in the bracket produces this L*.
        // Take the LOWER K — the least ink that achieves the colour.
        // Stated because "any of them is correct" is true of the colour
        // and false of the ink, and this project's subject here is ink.
        0.0
    } else {
        (a - target) / span
    };
    (lo as f64 + t) / (n - 1) as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A synthetic non-increasing ramp: L* from 100 down to 0.
    #[allow(clippy::cast_precision_loss)] // test sizes are small and exact
    fn linear_ramp(n: usize) -> Vec<f64> {
        (0..n)
            .map(|i| 100.0 - 100.0 * i as f64 / (n - 1) as f64)
            .collect()
    }

    #[test]
    fn inverting_a_linear_ramp_is_the_identity() {
        // Expectation source: ARITHMETIC, not colorimetry. For a ramp
        // whose L* is exactly linear in K, the K at lightness `l` is
        // `1 - l/100` by construction. This tests the inverter, not any
        // colour claim.
        let ramp = linear_ramp(1024);
        for (target, want) in [(100.0, 0.0), (75.0, 0.25), (50.0, 0.5), (0.0, 1.0)] {
            let got = invert_lightness(&ramp, target);
            assert!(
                (got - want).abs() < 1e-9,
                "L*={target} -> K={got}, expected {want}"
            );
        }
    }

    #[test]
    fn targets_outside_the_ramp_clamp_rather_than_extrapolate() {
        let ramp = linear_ramp(64);
        assert_eq!(invert_lightness(&ramp, 150.0), 0.0);
        assert_eq!(invert_lightness(&ramp, -20.0), 1.0);
    }

    #[test]
    fn a_ramp_that_turns_around_is_not_monotonic() {
        assert!(is_monotonic_non_increasing(&[100.0, 50.0, 0.0]));
        assert!(!is_monotonic_non_increasing(&[100.0, 50.0, 60.0]));
        // Flat is monotonic: ink saturating is not a defect.
        assert!(is_monotonic_non_increasing(&[100.0, 50.0, 50.0, 0.0]));
    }

    #[test]
    fn apply_rejects_any_chromatic_ink_however_small() {
        // The exact-zero rule, asserted on OUTCOME rather than on the
        // shape of the test inside `apply`.
        let kp = KPreserve {
            mapping: KMapping::EqualLightness,
            curve: linear_ramp(16).iter().map(|l| 1.0 - l / 100.0).collect(),
        };
        assert!(kp.apply(&[0.0, 0.0, 0.0, 0.5]).is_some());
        assert!(kp.apply(&[1e-12, 0.0, 0.0, 0.5]).is_none());
        assert!(kp.apply(&[0.0, 1e-12, 0.0, 0.5]).is_none());
        assert!(kp.apply(&[0.0, 0.0, 1e-12, 0.5]).is_none());
        // Wrong channel count is not K-only either.
        assert!(kp.apply(&[0.0, 0.0, 0.0]).is_none());
    }

    #[test]
    fn a_qualifying_input_emits_exactly_zero_chromatic_ink() {
        // The requirement, asserted as a measured output: the three
        // chromatic channels carry the ENCODED VALUE ZERO, not something
        // small. This is the predicate Pass K grades with a tolerance of
        // exactly zero.
        let kp = KPreserve {
            mapping: KMapping::EqualLightness,
            curve: linear_ramp(16).iter().map(|l| 1.0 - l / 100.0).collect(),
        };
        for k in [0.0, 0.25, 0.5, 0.75, 1.0] {
            let out = kp.apply(&[0.0, 0.0, 0.0, k]).expect("K-only input");
            assert_eq!(out[0], 0.0);
            assert_eq!(out[1], 0.0);
            assert_eq!(out[2], 0.0);
        }
    }
}
