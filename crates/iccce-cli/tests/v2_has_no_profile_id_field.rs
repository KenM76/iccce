//! # A v4-only field must not be read out of a v2 profile — the third
//! instance of one mechanism, and the first that FABRICATED a value
//!
//! ## The defect this pins, measured before it was fixed
//!
//! `profileID` was added in **v4**. In **ICC.1:2001-04 Table 9** the
//! header's bytes `84..127` are a single 44-byte block, *"44 bytes
//! reserved for future expansion"* — there is no identifier there, and
//! that sentence is the **only** mention of the block in the document.
//!
//! iccce read `84..100` as a `profileID` **regardless of edition**. So a
//! v2 profile carrying `0xDEADBEEF…` in its reserved space printed:
//!
//! ```text
//! header.id: deadbeefdeadbeefdeadbeefdeadbeef
//! malformations: 0
//! ```
//!
//! ★★ **That is worse than a false accusation.** The rendering-intent
//! defect fixed on 2026-08-18 accused a conforming file — loud, and
//! wrong in a way a careful reader could argue with. This one
//! **invented a checkable identity claim out of bytes that mean
//! something else**, and reported nothing at all. A consumer would
//! reasonably conclude the profile carries an MD5 profile ID it could
//! verify. The file carries no such field.
//!
//! And simultaneously it under-reported: checking only `100..128` on a
//! v2 profile **misses 16 bytes of that edition's reserved block**.
//!
//! ## The mechanism, which is the reason this file exists
//!
//! **Three instances in two days, all the same shape — a v4-only
//! concept applied to a v2 profile:**
//!
//! | date | what | how it showed |
//! |---|---|---|
//! | 2026-08-18 | rendering-intent report unconditional | v2 accused in v4's words |
//! | 2026-08-19 | `Malformation`'s doc comment | non-violations described as violations |
//! | 2026-08-19 | **this** | a fabricated `profileID`, and 16 reserved bytes unchecked |
//!
//! Each was found by asking *"does this edition actually say that?"*
//! rather than by a failing test. **A test cannot ask that question**,
//! which is why this file pins the outcome rather than pretending to
//! guard the class.
//!
//! ## What is asserted
//!
//! | assertion | catches |
//! |---|---|
//! | v2 with dirty `84..100` **reports** a malformation | the 16-byte blind spot returning |
//! | the report **names `84..128`**, not `100..128` | the range silently reverting to v4's |
//! | v4 with dirty `100` reports and names `100..128` | over-correcting, i.e. applying v2's range to v4 |
//! | v2's status is `NotAViolation`, v4's is `Violation` | the modality split collapsing |
//!
//! ## Sourcing
//!
//! - **v4**: ICC.1:2022 **7.2.19** — reserved bytes *"shall be set to
//!   zero"*. A requirement a file can breach ⇒ `Violation`.
//! - **v2**: ICC.1:2001-04 **Table 9** — *"44 bytes reserved for future
//!   expansion"*, **no modal verb**, encoding column empty.
//!   ⇒ `NotAViolation`. ★ v2 states requirements with **"must"** (76
//!   occurrences) rather than `shall`, so an unmodalised v2 sentence
//!   really is silent — the drafters used "must" in adjacent sentences.
//!   A `shall`-grep gets that edition backwards.
//!
//! Sourced by `icc-spec-librarian` on 2026-08-19 from both editions'
//! primary text; quotations in `ICC_Spec/icc/icc__s__tag_table.md` §7.
//! **Evidence class: specification text**, read rather than inferred.

use iccce_profile::Profile;
use iccce_profile::diag::{Malformation, ViolationStatus};

fn fixture(name: &str) -> Vec<u8> {
    let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/synthetic")
        .join(name);
    std::fs::read(&path)
        .unwrap_or_else(|e| panic!("fixture {} must be readable: {e}", path.display()))
}

/// Find the reserved-block report, if any, and return the range it names.
fn reserved_report(p: &Profile) -> Option<usize> {
    p.malformations.iter().find_map(|m| match m {
        Malformation::HeaderReservedNonZero { first_byte } => Some(*first_byte),
        _ => None,
    })
}

/// ★ The defect itself: bytes v2 reserves, dirtied, must be disclosed.
///
/// Before the fix this profile reported **zero** malformations, because
/// iccce was looking only at `100..128`.
#[test]
fn a_v2_profile_discloses_its_whole_reserved_block_including_84_to_100() {
    let mut b = fixture("v2-rgb-header-intent-relative.icc");
    assert_eq!(
        b[8], 2,
        "this fixture must be v2 for the test to mean anything"
    );
    b[84..100].copy_from_slice(&[0xAB; 16]);

    let p = Profile::parse(&b).expect("the profile must still parse");
    let first = reserved_report(&p).expect(
        "a v2 profile with non-zero bytes at 84..100 MUST be disclosed — those bytes are \
         reserved in ICC.1:2001-04 Table 9. Reporting nothing is the 16-byte blind spot \
         this test pins",
    );
    assert_eq!(
        first, 84,
        "the report must name v2's range 84..128, not v4's 100..128 — naming the wrong \
         range understates the block by 16 bytes"
    );
}

/// ★★ The fabrication: those same bytes must not be presented as an
/// identifier.
///
/// The parser still *stores* them (rule 6 — report, never repair), so
/// this asserts on what a consumer is TOLD, which is where the harm was.
#[test]
fn the_v2_reserved_bytes_are_not_presented_as_a_profile_id() {
    let mut b = fixture("v2-rgb-header-intent-relative.icc");
    b[84..100].copy_from_slice(&[0xAB; 16]);

    let out = std::process::Command::new(env!("CARGO_BIN_EXE_iccce"))
        .arg("inspect")
        .arg({
            let tmp = std::env::temp_dir().join("iccce-v2-profileid-test.icc");
            std::fs::write(&tmp, &b).expect("temp fixture must be writable");
            tmp
        })
        .output()
        .expect("the iccce binary must be runnable");
    let stdout = String::from_utf8(out.stdout).expect("stdout is UTF-8");

    let id_line = stdout
        .lines()
        .find(|l| l.starts_with("header.id:"))
        .expect("inspect must say something about header.id");

    assert!(
        !id_line.contains("abababab"),
        "the reserved bytes are being printed as a profileID — a field v2 does not have. \
         This FABRICATES a checkable identity claim from bytes that mean something else. \
         Got: {id_line}"
    );
    assert!(
        id_line.contains("n/a"),
        "a v2 profile must say the field does not exist rather than print a value or stay \
         silent. Got: {id_line}"
    );
}

/// The modality split: the same condition is a breach in v4 and not in
/// v2.
///
/// Without this, `violation_status` could return `Violation` for both
/// and every other assertion here would still pass — which would put
/// iccce back to accusing a conforming v2 file, in a different place.
#[test]
fn the_reserved_block_is_a_violation_in_v4_and_not_in_v2() {
    let mut v2 = fixture("v2-rgb-header-intent-relative.icc");
    v2[84..100].copy_from_slice(&[0xAB; 16]);
    let p2 = Profile::parse(&v2).expect("v2 must parse");
    let m2 = p2
        .malformations
        .iter()
        .find(|m| matches!(m, Malformation::HeaderReservedNonZero { .. }))
        .expect("v2 must report");
    assert_eq!(
        m2.violation_status(p2.header.version),
        ViolationStatus::NotAViolation,
        "ICC.1:2001-04 Table 9 is unmodalised — \"44 bytes reserved for future expansion\" \
         — so a v2 profile with non-zero reserved bytes BREACHES NOTHING. Calling it a \
         violation accuses a conforming file"
    );

    let mut v4 = fixture("rendering-intent-high-bits.icc");
    assert_eq!(v4[8], 4, "this fixture must be v4");
    v4[100] = 0xAB;
    let p4 = Profile::parse(&v4).expect("v4 must parse");
    let m4 = p4
        .malformations
        .iter()
        .find(|m| matches!(m, Malformation::HeaderReservedNonZero { .. }))
        .expect("v4 must report");
    assert_eq!(
        m4.violation_status(p4.header.version),
        ViolationStatus::Violation,
        "ICC.1:2022 7.2.19 says these bytes \"shall be set to zero\""
    );
    assert_eq!(
        reserved_report(&p4),
        Some(100),
        "v4's range is 100..128 — 84..100 is its profileID"
    );
}
