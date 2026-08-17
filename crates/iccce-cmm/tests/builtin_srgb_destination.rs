//! # The built-in sRGB destination — behaviour, disclosure, and the
//! # measured cost of its one named approximation
//!
//! ## What this file is for
//!
//! `docs/DEFAULT_DESTINATION.md` records the operator's decision that
//! iccce constructs sRGB when a caller supplies no destination. That
//! document states three things this file has to demonstrate **in the
//! running code**, not in review:
//!
//! 1. **§1.1 — a supplied destination is used, always.** The fallback
//!    must not fire when a profile was given.
//! 2. **§3 — the fallback is disclosed, not silent.** A consumer must be
//!    able to ask whether the constructed destination was used.
//! 3. **§4.2 — the constructed sRGB is NOT byte-identical to any shipped
//!    sRGB profile, so it must be tested by a ΔE round-trip with a
//!    stated tolerance and a named oracle — never by byte equality.**
//!    Byte equality would be testing HP's 1998 arithmetic.
//!
//! ## ★ On the evidence class of everything in this file
//!
//! **None of these expectations is ground truth, and none is a
//! cross-check against another CMM.** They are comparisons of iccce's
//! constructed sRGB against **iccce's own evaluation of a third-party
//! sRGB profile file**. That makes the *reference* third-party while the
//! *machinery on both sides is ours* — a weaker claim than either of the
//! project's usual two, and it is written down here rather than left for
//! a reader to work out.
//!
//! What it therefore CAN establish: that our construction agrees with
//! the canonical published encoding of the same colour space to a
//! measured, bounded amount, and that the bound does not drift. What it
//! CANNOT establish: that either one is colorimetrically right. No
//! document publishes a worked sRGB input→output triple, and no document
//! publishes the D50-adapted colorants at all — so this is the strongest
//! check available, not a substitute for one that does not exist.
//!
//! ## Corpus resolution
//!
//! The reference profile is read at run time from the private fixtures
//! (`$ICCCE_PRIVATE_FIXTURES`, then the default path, then the Windows
//! system profile). **When none is found these tests SKIP loudly and
//! assert nothing** — a green run on a machine without the data is
//! evidence the check did not run, never evidence it passed.

use iccce_cmm::matrix_trc::{Intent, MatrixTrc};
use iccce_cmm::transform::{Chain, Destination, DestinationProvenance};
use iccce_color::{D50, Lab, delta_e_2000};
use iccce_profile::Profile;

/// Locate a shipped sRGB profile, or `None` → the caller skips.
///
/// ## ★★ The two candidates are NOT interchangeable — read before adding
/// ## a test here
///
/// The resolution order below can land on either ICC's `sRGB2014.icc` or
/// the 1998 HP `sRGB Color Space Profile.icm` that Windows ships,
/// depending on the machine. For the colorants and the TRCs that is
/// harmless — **they are byte-identical** (measured 2026-08-17; ICC's
/// 2015 profile reuses HP's nine numbers and all three curves exactly).
///
/// **For three other things they differ, and a test that touched any of
/// them would silently measure a different quantity on a different
/// machine:**
///
/// | | HP 1998 | ICC 2015 |
/// |---|---|---|
/// | `wtpt` | **D65** — non-compliant with ICC.1 A.3.1.1 | **D50** — compliant |
/// | `chad` | **absent** | **present** |
/// | `bkpt` | `0, 0, 0` | **`0.0024, 0.0025, 0.0021`** |
/// | declared version | **2.1.0** | **2.0.0** |
///
/// ★ Note the last row is backwards from intuition: **ICC's 2015 profile
/// declares a *lower* version than HP's 1998 one.** Anything keying on
/// the version to infer recency is wrong on this pair.
///
/// **So: do not add a black-point, BPC, absolute-intent or `wtpt` test
/// that uses this helper.** Those all read one of the differing fields,
/// and "the sRGB profile" is ambiguous for them. Pin the exact file you
/// mean instead, and say why.
///
/// The tests currently in this file are safe because they touch only the
/// colorants and TRCs — except the ΔE bound, which is **derived from
/// whichever file was resolved** rather than being a constant, and
/// therefore tracks the choice instead of being invalidated by it.
fn shipped_srgb() -> Option<(std::path::PathBuf, Vec<u8>)> {
    let mut candidates: Vec<std::path::PathBuf> = Vec::new();
    if let Ok(root) = std::env::var("ICCCE_PRIVATE_FIXTURES") {
        candidates.push(std::path::Path::new(&root).join("color-org/sRGB2014.icc"));
    }
    candidates.push(std::path::PathBuf::from(
        r"D:\Dev\iccce-private-fixtures\color-org\sRGB2014.icc",
    ));
    candidates.push(std::path::PathBuf::from(
        r"C:\Windows\System32\spool\drivers\color\sRGB Color Space Profile.icm",
    ));
    let path = candidates.into_iter().find(|p| p.is_file())?;
    let bytes = std::fs::read(&path).ok()?;
    Some((path, bytes))
}

/// A spread of device RGB values covering the places a colour transform
/// actually goes wrong.
///
/// ★ Chosen so that no two distinct quantities coincide (project rule
/// §5.3 / GP-002: round, symmetric fixture values make conceptually
/// distinct quantities collide, and coincidence destroys
/// discrimination). In particular the **near-black** entries matter most
/// — that is where the transfer function's linear segment lives and
/// where a wrong construction is least visible.
const PROBES: &[[f64; 3]] = &[
    [0.0, 0.0, 0.0],
    [1.0, 1.0, 1.0],
    [0.0117, 0.0235, 0.0039], // deep shadow, all three channels distinct
    [0.0392, 0.0431, 0.0353], // straddling the TRC breakpoint
    [0.2157, 0.4706, 0.7333],
    [0.8039, 0.1176, 0.3608],
    [0.5137, 0.6392, 0.1804],
    [0.9412, 0.7059, 0.0784],
    [0.1373, 0.8627, 0.5490],
    [0.6275, 0.2745, 0.9059],
];

/// §1.1 — a supplied destination is used, and the chain says so.
#[test]
fn supplied_destination_is_used_and_reported_as_caller_supplied() {
    let Some((path, bytes)) = shipped_srgb() else {
        eprintln!("SKIP: no shipped sRGB profile found. This test asserted NOTHING.");
        return;
    };
    let profile = Profile::parse(&bytes).unwrap_or_else(|e| panic!("{path:?} should parse: {e}"));
    let chain = Chain::with_destination(
        &profile,
        Destination::Profile(&profile),
        Intent::MediaRelative,
    )
    .expect("sRGB → sRGB chain builds");

    assert_eq!(
        chain.destination_provenance(),
        DestinationProvenance::CallerSupplied,
        "a supplied destination must never be reported as the built-in"
    );
    assert!(
        chain.destination_provenance().note().is_none(),
        "there is nothing to disclose when the caller supplied the destination"
    );
}

/// §3 — the fallback is disclosed, not silent.
///
/// ★ This asserts on the **observable output of the running chain**, not
/// on the shape of the code that produced it. A test that grepped for a
/// call site could be satisfied by a helper that is never reached.
#[test]
fn absent_destination_is_disclosed_not_silent() {
    let Some((_, bytes)) = shipped_srgb() else {
        eprintln!("SKIP: no shipped sRGB profile found. This test asserted NOTHING.");
        return;
    };
    let profile = Profile::parse(&bytes).expect("parses");
    let chain = Chain::with_destination(&profile, Destination::None, Intent::MediaRelative)
        .expect("chain against the constructed destination builds");

    assert_eq!(
        chain.destination_provenance(),
        DestinationProvenance::BuiltInSrgb
    );
    let note = chain
        .destination_provenance()
        .note()
        .expect("the built-in destination MUST carry a disclosure");
    // The disclosure has to be usable by a consumer writing a preflight
    // line, so it must actually say the two load-bearing things.
    assert!(
        note.contains("no destination profile was supplied"),
        "the note must state that no destination was supplied: {note}"
    );
    assert!(
        note.contains("NOT the document's declared output intent"),
        "the note must warn that this is not a declared output intent: {note}"
    );
}

/// ★★ **The rule-4 measurement.** How far is the constructed sRGB from
/// the canonical published encoding of sRGB, in ΔE2000?
///
/// ## Method
///
/// For each probe RGB, evaluate device→PCSXYZ through (a) the
/// constructed model and (b) the model parsed from the shipped profile,
/// convert both to Lab under D50, and take ΔE2000. Both sides are
/// media-relative, both are D50-referred, so the two quantities are
/// commensurate — the comparison is not being made across a scale
/// difference.
///
/// ## ★★ Tolerance — DERIVED from the reference file, not chosen
///
/// The first draft of this test asserted a flat `0.02 ΔE2000`, reasoning
/// from the 12-ULP `bXYZ.Z` residual that the answer would be "a few
/// hundredths". **It failed at `0.033013`, and investigating the failure
/// rather than moving the number produced a better test** (project rule
/// 5: when a row fails, the first question is whether the code is
/// wrong).
///
/// What the investigation found: **the worst probe is pure white, and at
/// white the difference is not a fact about iccce at all.**
///
/// - Our constructed model maps device white to **exactly ICC D50**, by
///   construction — the Bradford adaptation is aimed there and lands
///   there (asserted in `builtin`'s own tests to `1e-9`).
/// - The reference file's colorants sum to
///   `0.964279 / 0.999969 / 0.825089`, which is **not** ICC D50
///   `0.9642 / 1 / 0.8249`, and is not the 5-figure graphic-arts D50
///   either. The corpus records this: *"the profile's implied white lies
///   BETWEEN ICC's D50 and the 5-figure D50 and matches neither."*
///
/// So the white probe measures **the reference artifact's own deviation
/// from D50**. Asserting a constant against it would be encoding a
/// third-party file's authoring rounding into our tolerance.
///
/// **The bound is therefore computed at run time from the reference
/// file's own tags:**
///
/// > *no probe may differ from the reference by more than the
/// > reference's own white-point offset from D50.*
///
/// This is a real, falsifiable claim and a tight one. It says the entire
/// discrepancy between our construction and the published encoding is
/// **bounded by a known property of that encoding** — i.e. we are not
/// adding error of our own beyond what the file already carries. A
/// genuinely wrong construction (a missing adaptation, a transposed
/// matrix, a wrong primary) blows past it immediately, because those
/// defects are orders of magnitude larger than a white-point rounding.
///
/// A 5% headroom is allowed on the derived bound purely so that ΔE2000's
/// non-uniformity — it is not a metric, and its local scale varies with
/// chroma and hue — cannot make a mid-gamut probe marginally exceed a
/// bound derived at the neutral axis.
///
/// ## ★★ There are TWO margins here and they are not interchangeable
///
/// An earlier version of this comment claimed "the margin in hand is
/// 37%, not 5%". **That conflated two different quantities**, and
/// `icc-librarian` caught it reading the source. Both numbers are real;
/// they describe different probes.
///
/// | probe | observed | vs derived bound `0.034664` | margin |
/// |---|---|---|---|
/// | **white — the BINDING probe** | `0.033013` | the bound *is* derived from this quantity | **exactly 5%** |
/// | largest non-white (`0.0392, 0.0431, 0.0353`) | `0.020671` | non-binding | 40% |
///
/// **Why white is special, stated precisely:** the bound is
/// `ΔE2000(D50, from_file.matrix()·[1,1,1]) × 1.05`, and the white probe
/// evaluates `from_file.device_to_pcs([1,1,1])` — **the same vector**,
/// because a `curv`'s last entry maps 1.0 to 1.0. Our construction maps
/// device white to D50 exactly. So at that probe *observed ≡ the derived
/// quantity*, and the assertion reduces to `x ≤ 1.05·x`.
///
/// ## ★★★ THIS TEST IS ASYMMETRICALLY BLIND, and that is §5.2 in the flesh
///
/// It is tempting to read the 5% above as "a very tight white-point
/// gate". **I wrote exactly that, then injected white-point drift to
/// check it, and it was wrong.** The measured behaviour:
///
/// | injected drift in the constructed white's `Z` | max ΔE2000 | this test |
/// |---|---|---|
/// | `−1.0×10⁻³` | `0.101968` | **FAILS** ✔ |
/// | `−3.0×10⁻⁴` | `0.050149` | **FAILS** ✔ |
/// | **`+3.0×10⁻⁴`** | **`0.029008`** | **PASSES — and looks BETTER than correct** ✘ |
/// | `+2.0×10⁻³` | `0.146450` | FAILS ✔ |
///
/// **A defect in one direction makes this test greener.** The reason is
/// structural: the reference file's own white sits `+1.885×10⁻⁴` above
/// D50 in `Z`, so drifting our construction *upward* moves it **toward
/// the reference**. Anything up to about `+3.8×10⁻⁴` — the point where
/// we would land on the file's white exactly — reduces the measured
/// difference. At that point the test would report **zero**.
///
/// This is `docs/NEXT_SESSION.md` §5.2 exactly: *a differential test has
/// no power against an error that moves your answer toward the
/// reference.* The project already paid for this lesson once, with a
/// non-conformant black-point estimator that landed `0.082 ΔE76` from
/// lcms2 while carrying a `4.717 L*` defect — **the buggy build agreed
/// with the oracle better than the correct build did.** The same shape
/// recurs here for the same reason, and it is not fixable by tightening
/// anything: **a difference cannot detect a defect that shrinks it.**
///
/// ## What actually gates the white point, and it is not this test
///
/// `builtin::tests::constructed_colorant_sum_is_d50` — which compares
/// the constructed white against **`D50` itself, to `1e-9`**, with no
/// reference file anywhere in it. Verified by the same injection:
/// **`+3.0×10⁻⁴` fails it while this file's six tests all pass.**
///
/// ★ **So the division of labour is load-bearing and must not be
/// "simplified":**
///
/// - **An ABSOLUTE assertion** (`= D50`, no file) gates the white point,
///   in both directions, tightly.
/// - **A DIFFERENTIAL assertion** (this file, vs. a reference profile)
///   gates overall agreement — and is blind in one direction by
///   construction.
///
/// Deleting the absolute test as "redundant, the ΔE test covers it"
/// would remove the only thing with power against the blind direction,
/// **and every test would stay green while it happened.**
///
/// The non-white probes are where the 40% headroom lives, and they are
/// what the ΔE2000 non-uniformity allowance was actually for.
///
/// ## The second, independent assertion
///
/// Separately and on completely different grounds, the worst difference
/// must stay **below `1.0 ΔE2000`**, the widely used perceptibility
/// threshold. That is the practical claim a consumer cares about — *is
/// the picture different?* — and it is justified by perception rather
/// than by this file, so it is asserted separately rather than folded
/// into one number. Two claims, two justifications.
#[test]
fn constructed_srgb_matches_the_published_encoding_within_stated_delta_e() {
    let Some((path, bytes)) = shipped_srgb() else {
        eprintln!("SKIP: no shipped sRGB profile found. This test asserted NOTHING.");
        return;
    };
    let profile = Profile::parse(&bytes).expect("parses");
    let from_file = MatrixTrc::from_profile(&profile).expect("matrix/TRC model");
    let constructed = iccce_cmm::builtin::srgb();

    // The derived bound: the reference file's OWN white-point offset
    // from D50, expressed in the same metric. Computed from the file's
    // tags, so it tracks whichever reference profile was resolved.
    let fw = from_file.matrix().apply([1.0, 1.0, 1.0]);
    let file_white = iccce_color::Xyz {
        x: fw[0],
        y: fw[1],
        z: fw[2],
    };
    let reference_white_offset =
        delta_e_2000(Lab::from_xyz(D50, D50), Lab::from_xyz(file_white, D50));
    let derived_bound = reference_white_offset * 1.05;

    // Perceptibility, justified independently of anything above.
    const PERCEPTIBILITY_DE2000: f64 = 1.0;

    let mut worst = 0.0_f64;
    let mut worst_probe = [0.0; 3];
    for &rgb in PROBES {
        let a = Lab::from_xyz(constructed.device_to_pcs(rgb), D50);
        let b = Lab::from_xyz(from_file.device_to_pcs(rgb), D50);
        let de = delta_e_2000(a, b);
        if de > worst {
            worst = de;
            worst_probe = rgb;
        }
    }
    println!(
        "constructed sRGB vs {}:\n  max {worst:.6} ΔE2000 at RGB {worst_probe:?}\n  \
         reference file's own white offset from D50 = {reference_white_offset:.6} ΔE2000\n  \
         derived bound (offset × 1.05) = {derived_bound:.6}",
        path.display()
    );
    assert!(
        worst <= derived_bound,
        "constructed sRGB differs from the published encoding by {worst:.6} ΔE2000 at \
         RGB {worst_probe:?}, exceeding the bound {derived_bound:.6} derived from that file's \
         OWN white-point offset from D50 ({reference_white_offset:.6}). This means we are adding \
         error beyond what the reference already carries — check the CONSTRUCTION (adaptation \
         applied? once? right primaries?) before touching this test."
    );
    assert!(
        worst < PERCEPTIBILITY_DE2000,
        "constructed sRGB differs from the published encoding by {worst:.6} ΔE2000, which is \
         at or above the {PERCEPTIBILITY_DE2000} ΔE2000 perceptibility threshold — the \
         substitution would be VISIBLE"
    );
    // ★ And the other direction: the difference must not be ZERO either.
    // A zero here would mean the constructed model had somehow become
    // the file's model — i.e. that this test had stopped comparing two
    // independent things and was silently self-comparing.
    assert!(
        worst > 1e-9,
        "the constructed sRGB is bit-identical to the parsed profile ({worst:.3e} ΔE2000). \
         That is not possible from an independent construction — this test is no longer \
         comparing two different things."
    );
}

/// A full conversion actually runs end to end against the constructed
/// destination, and lands somewhere sane.
///
/// ★ The point of this test is that it exercises `Chain`, not
/// `MatrixTrc` — the previous test could pass while `Destination::None`
/// was wired to the wrong model, or to nothing.
#[test]
fn chain_converts_through_the_constructed_destination() {
    let Some((_, bytes)) = shipped_srgb() else {
        eprintln!("SKIP: no shipped sRGB profile found. This test asserted NOTHING.");
        return;
    };
    let profile = Profile::parse(&bytes).expect("parses");
    let via_builtin = Chain::with_destination(&profile, Destination::None, Intent::MediaRelative)
        .expect("builds");
    let via_file = Chain::with_destination(
        &profile,
        Destination::Profile(&profile),
        Intent::MediaRelative,
    )
    .expect("builds");

    let mut worst = 0.0_f64;
    for &rgb in PROBES {
        let a = via_builtin.convert(&rgb).expect("converts");
        let b = via_file.convert(&rgb).expect("converts");
        assert_eq!(a.len(), 3, "sRGB destination has 3 channels");
        for (x, y) in a.iter().zip(b.iter()) {
            worst = worst.max((x - y).abs());
        }
        // sRGB→sRGB through the file is an identity round trip, so the
        // built-in path must land near the input as well. A loose bound
        // on purpose: this asserts "the chain is wired to sRGB", not a
        // precision claim — that is the previous test's job.
        for (out, inp) in a.iter().zip(rgb.iter()) {
            assert!(
                (out - inp).abs() < 0.01,
                "sRGB → built-in sRGB should be near-identity: in {rgb:?}, out {a:?}"
            );
        }
    }
    println!("chain: max device-channel difference built-in vs file = {worst:.3e}");
}

/// ★ The contract's dangerous edge, asserted: a destination that FAILED
/// TO PARSE must never become the built-in.
///
/// `docs/DEFAULT_DESTINATION.md` §2 — *"doesn't exist" must mean absent,
/// never unresolved.* This test demonstrates that the type system
/// enforces it rather than a convention doing so: garbage bytes produce
/// a parse **error** at the caller, and there is no path from that error
/// into `Destination::None` that iccce can take on the caller's behalf.
///
/// The assertion is about the parse refusing. The structural half — that
/// `Destination` has no `From<Option<..>>` and no fallible constructor
/// that swallows an error — is enforced by the enum having exactly two
/// explicit variants, which a reviewer can see and a caller cannot
/// bypass accidentally.
#[test]
fn an_unparseable_destination_is_a_refusal_not_a_fallback() {
    let garbage = vec![0u8; 200];
    let parsed = Profile::parse(&garbage);
    assert!(
        parsed.is_err(),
        "200 zero bytes must not parse as a profile; if this ever succeeds the premise of \
         this test is gone"
    );
    // The caller now holds an Err. Reaching Destination::None from here
    // is a decision a human has to write on purpose — which is exactly
    // the design intent, and is why the API takes an enum rather than
    // an Option that a `?` or an `.ok()` would quietly flatten.
}

/// Diagnostic (not a gate): per-probe breakdown, used to decide where
/// the residual actually lives.
#[test]
fn diagnostic_per_probe_breakdown() {
    let Some((_, bytes)) = shipped_srgb() else {
        return;
    };
    let profile = Profile::parse(&bytes).expect("parses");
    let from_file = MatrixTrc::from_profile(&profile).expect("model");
    let constructed = iccce_cmm::builtin::srgb();
    let file_white = from_file.matrix().apply([1.0, 1.0, 1.0]);
    println!("file colorant sum = {file_white:?}");
    println!("ICC D50           = [{}, {}, {}]", D50.x, D50.y, D50.z);
    for &rgb in PROBES {
        let a = Lab::from_xyz(constructed.device_to_pcs(rgb), D50);
        let b = Lab::from_xyz(from_file.device_to_pcs(rgb), D50);
        println!(
            "  rgb {:?} -> dE2000 {:.6}   dL {:+.4} da {:+.4} db {:+.4}",
            rgb,
            delta_e_2000(a, b),
            a.l - b.l,
            a.a - b.a,
            a.b - b.b
        );
    }
}
