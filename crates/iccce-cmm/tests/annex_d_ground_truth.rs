//! ICC.1:2022 Annex D.6.3 — the project's first ground-truth test for a
//! transform path.
//!
//! # Why this file exists, and why it is unlike every other test here
//!
//! Until 2026-08-12 this project had **exactly one** `published-ground-truth`
//! row: ΔE2000 against Sharma, Wu & Dalal (2005). That grades a *metric*.
//! Everything asserted about colour *transformation* was
//! `implementation-cross-check` against lcms2 — a real claim, and not the same
//! claim as "it is right". The same day showed why that distinction bites:
//! a conformance defect was found whose wrong answer sat **0.082 ΔE76 from
//! lcms2's**, because it returned an intermediate lcms2 also computes. A
//! cross-check is structurally blind to that class of error; only an
//! *independent published expectation* can see it.
//!
//! ICC.1:2022 Annex D.6.3 is such an expectation. It prints an input, every
//! intermediate, and **twelve exact integer encodings** for the
//! data→PCS media-relative colorimetric chain.
//!
//! # Three qualifications that must travel with every citation
//!
//! 1. **Annex D is INFORMATIVE.** This is ground truth *epistemically* —
//!    published numbers from the standards body rather than from an
//!    implementation — and **not normatively**: a CMM that disagrees is not
//!    thereby non-conforming. Say "informative" whenever this is cited.
//! 2. **It contains no LUT.** The example ends exactly where `AToB1Tag` would
//!    begin. It says nothing about interpolation, gamut mapping, or
//!    perceptual/saturation intent. For the LUT path no published ground truth
//!    *can* exist: ICC.1 mandates no interpolation method, so two conforming
//!    CMMs may legitimately differ and no single value could be published as
//!    expected. (Corroborated from the strongest direction: iccDEV, ICC's own
//!    reference implementation, ships zero expected colour values.)
//! 3. **The chain is anchored at Table D.3, never Table D.2.** D.2's black
//!    `X = 0,0097` cannot produce D.3's `0,0134` under any rounding — the
//!    attainable interval is `[0,013165 , 0,013304]`. That is a genuine
//!    erratum in the standard. Anchoring at D.2 would encode the erratum into
//!    the expectation.
//!
//! # The data is not in this repository, and that is deliberate
//!
//! The values are ICC © 2022 and the document grants no reproduction right.
//! The operator's decision (2026-08-12) was to keep them **local** and have
//! the test read them at run time, so the repository stays unambiguously MIT.
//!
//! Resolution order: `$ICCCE_PRIVATE_FIXTURES`, then
//! `D:\Dev\iccce-private-fixtures`, then **skip**.
//!
//! ★ **A green run on a machine without that folder is NOT evidence that this
//! check passed — it is evidence that it did not run.** CI holds no such data
//! and is permanently in the skipping case, by design. Stated here so a green
//! CI badge is never mistaken for a ground-truth claim.
//!
//! A **partially** installed folder fails rather than skips: if the directory
//! exists but the file is missing or malformed, that is loud. Half-installed
//! data is otherwise indistinguishable from a pass, which is the failure mode
//! this whole project is organised against.

use std::collections::BTreeMap;
use std::path::PathBuf;

use iccce_cmm::pcs_encoding::{LabEncoding, encode_pcs_xyz};
use iccce_color::{Lab, Xyz};

/// Locate the private fixture store, or `None` if it is simply absent.
fn fixture_dir() -> Option<PathBuf> {
    let p = std::env::var_os("ICCCE_PRIVATE_FIXTURES").map_or_else(
        || PathBuf::from(r"D:\Dev\iccce-private-fixtures"),
        PathBuf::from,
    );
    p.is_dir().then_some(p)
}

/// Parse the `key<TAB>value` fixture. Panics on a malformed file **by
/// design** — see the module header on half-installed data.
fn load(path: &PathBuf) -> BTreeMap<String, f64> {
    let text = std::fs::read_to_string(path).unwrap_or_else(|e| {
        panic!(
            "private fixture directory exists but {} could not be read: {e}. \
             A half-installed fixture must fail, not skip.",
            path.display()
        )
    });
    let mut map = BTreeMap::new();
    for (n, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let (k, v) = line.split_once('\t').unwrap_or_else(|| {
            panic!(
                "{}:{}: expected key<TAB>value, got {line:?}",
                path.display(),
                n + 1
            )
        });
        let parsed: f64 = v.trim().parse().unwrap_or_else(|e| {
            panic!(
                "{}:{}: value {v:?} is not a number: {e}",
                path.display(),
                n + 1
            )
        });
        map.insert(k.trim().to_owned(), parsed);
    }
    map
}

fn get(m: &BTreeMap<String, f64>, k: &str) -> f64 {
    *m.get(k)
        .unwrap_or_else(|| panic!("fixture is missing required key {k:?}"))
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn get_u16(m: &BTreeMap<String, f64>, k: &str) -> u16 {
    let v = get(m, k);
    assert!(
        v >= 0.0 && v <= 65535.0 && v.fract() == 0.0,
        "fixture key {k:?} should be an integer code, got {v}"
    );
    v as u16
}

#[test]
fn annex_d_6_3_pcs_encodings_match_the_published_integers() {
    let Some(dir) = fixture_dir() else {
        eprintln!(
            "\n★ SKIPPED: annex_d_6_3_pcs_encodings_match_the_published_integers\n\
             No private fixture store found (set $ICCCE_PRIVATE_FIXTURES, or place it\n\
             at D:\\Dev\\iccce-private-fixtures). THIS TEST ASSERTED NOTHING.\n\
             A green run without that data is not evidence about Annex D.\n"
        );
        return;
    };
    let f = load(&dir.join("icc-annex-d").join("annex_d_6_3.tsv"));

    // ---------------------------------------------------------------
    // 1. PCSXYZ -> 16-bit u1Fixed15. Six exact integers (Table D.5).
    // ---------------------------------------------------------------
    for (patch, comps) in [("white", ["x", "y", "z"]), ("black", ["x", "y", "z"])] {
        for c in comps {
            let value = get(&f, &format!("pcsxyz.{patch}.{c}"));
            let expected = get_u16(&f, &format!("enc16.xyz.{patch}.{c}"));
            assert_eq!(
                encode_pcs_xyz(value),
                expected,
                "Annex D.5 16-bit PCSXYZ {patch}.{c}: encoding {value} must give \
                 the published {expected}"
            );
        }
    }

    // ---------------------------------------------------------------
    // 2. PCSXYZ -> PCSLAB (Table D.3 -> D.4).
    //
    // ★ GRADED AS AN INTERVAL, NOT A POINT. D.4 prints L* to 1 dp and a*/b*
    // to 2 dp, so `11,8` asserts only that the true value lies in
    // [11.75, 11.85]. Demanding equality with the midpoint would be
    // demanding the standard print more digits than it did — and
    // point-evaluating an interval-valued published number is exactly the
    // error that made this data sit unused for eleven filings.
    // ---------------------------------------------------------------
    let white = Xyz {
        x: get(&f, "pcsxyz.white.x"),
        y: get(&f, "pcsxyz.white.y"),
        z: get(&f, "pcsxyz.white.z"),
    };
    for patch in ["white", "black"] {
        let xyz = Xyz {
            x: get(&f, &format!("pcsxyz.{patch}.x")),
            y: get(&f, &format!("pcsxyz.{patch}.y")),
            z: get(&f, &format!("pcsxyz.{patch}.z")),
        };
        let got = Lab::from_xyz(xyz, white);
        for (name, got_v, key, half_ulp) in [
            ("L*", got.l, format!("pcslab.{patch}.l"), 0.05),
            ("a*", got.a, format!("pcslab.{patch}.a"), 0.005),
            ("b*", got.b, format!("pcslab.{patch}.b"), 0.005),
        ] {
            let printed = get(&f, &key);
            assert!(
                (got_v - printed).abs() <= half_ulp,
                "Annex D.4 {patch} {name}: computed {got_v} is outside the \
                 published {printed} +/- {half_ulp} (its displayed half-ULP)"
            );
        }
    }

    // ---------------------------------------------------------------
    // 3. PCSLAB -> 16-bit v4 encoding. Six more exact integers.
    //
    // ★ The PRINTED D.4 values are the input here, not the values recomputed
    // in step 2, because the standard encoded its own displayed figures:
    // 11,8 x 655,35 = 7733,13 -> 7733, which is what D.5 prints, whereas the
    // full-precision 11,8232 would give 7748. Feeding our recomputed Lab
    // would therefore fail against the published integer while nothing was
    // wrong — a test disagreeing with the standard about which number the
    // standard encoded.
    //
    // ★ Note black b* is NEGATIVE (-0,3). pdftotext renders the U+F02D
    // Symbol-font minus as nothing; the sign is confirmed by the document's
    // own arithmetic, since +0,3 encodes to 32973 and -0,3 to the published
    // 32819. This assertion is the standing guard on that sign.
    // ---------------------------------------------------------------
    for patch in ["white", "black"] {
        let l = get(&f, &format!("pcslab.{patch}.l"));
        let a = get(&f, &format!("pcslab.{patch}.a"));
        let b = get(&f, &format!("pcslab.{patch}.b"));
        assert_eq!(
            LabEncoding::V4.encode_l(l),
            get_u16(&f, &format!("enc16.lab.{patch}.l")),
            "Annex D.5 16-bit L* for {patch}"
        );
        assert_eq!(
            LabEncoding::V4.encode_ab(a),
            get_u16(&f, &format!("enc16.lab.{patch}.a")),
            "Annex D.5 16-bit a* for {patch}"
        );
        assert_eq!(
            LabEncoding::V4.encode_ab(b),
            get_u16(&f, &format!("enc16.lab.{patch}.b")),
            "Annex D.5 16-bit b* for {patch} (negative for black; the sign is \
             the point of this assertion)"
        );
    }

    // ---------------------------------------------------------------
    // NOT TESTED, stated rather than silently omitted: the six 8-bit Lab
    // codes (D.5's third block). This crate exposes no public 8-bit Lab
    // codec — the 8-bit path is handled inline inside `lut_ab.rs`'s
    // normalisation — and writing the encoding here from memory would
    // violate the project's first rule. The values are held in the fixture,
    // unused, so the test can be extended the day a codec is exported.
    // ---------------------------------------------------------------
}
