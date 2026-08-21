//! # `violations: N` — the line that separates a disclosure from a verdict
//!
//! ## What this defends, and the incident that produced it
//!
//! `iccce inspect` prints two counts over one unaltered set of
//! observations:
//!
//! ```text
//! malformations: N    every disclosure the parser has to make
//! violations:    M    the subset that breaches a stated requirement
//! ```
//!
//! **`M <= N` always, and `M < N` is the interesting case**, because it
//! is the one a single count cannot express.
//!
//! ★★★ **The incident.** `DL-063` (2026-08-19) established that
//! `malformations: N` counts *disclosures*, not violations — a v2
//! profile can be fully conformant and still have something to
//! disclose. That was recorded in three doc comments. **It was not
//! recorded anywhere a program could read**, so every consumer still had
//! only `.len()`.
//!
//! On 2026-08-21 the differential suite's row
//! `passh/B/acceptance/no-malformation-is-disclosed-on-any-accepted-file`
//! was found **red against five ICC-PUBLISHED profiles** —
//! `sRGB2014.icc`, `ITU-RBT709ReferenceDisplay.icc`,
//! `PSOsc-b_paper_v3_FOGRA54.icc`, `PSOuncoated_v3_FOGRA52.icc` and
//! `SC_paper_eci.icc`. Each discloses exactly one
//! `HeaderReservedNonZero`: they are **v2** files carrying an MD5 in
//! bytes `84..99`, where **v4** later placed `profileID`.
//!
//! ★★ **The row offered two hypotheses and the answer was neither.** Its
//! own text reads *"either iccce over-reports or a published ICC profile
//! is defective"*. In fact the engine was right and the files are
//! conformant: ICC.1:2001-04 Table 9's cell is the unmodalised *"44
//! bytes reserved for future expansion"* — the only mention in the
//! document — so a v2 profile breaches no `shall` by using it.
//! `Malformation::violation_status` already said so.
//! **The row was counting the wrong quantity**, because the right one
//! was not exposed.
//!
//! ## Why these assertions are on OUTPUT and not on structure
//!
//! The obvious test — "assert `violation_status` returns `NotAViolation`
//! for `HeaderReservedNonZero` on v2" — **would have passed throughout
//! the entire incident**, because that function was correct the whole
//! time. What was broken was that *nothing printed it*. A test asserting
//! the classifier's shape cannot see a consumer that never calls the
//! classifier.
//!
//! So every assertion below parses the **bytes the binary actually
//! wrote**, and the load-bearing one asserts the two counts **differ**
//! on a file where they should — which is the exact observation the old
//! output could not produce at any price.
//!
//! ## Evidence class
//!
//! **Self-consistency plus one sourced expectation.** The `v2` vs `v4`
//! asymmetry is not an iccce convention: it is ICC.1:2022 7.2.19's
//! *"shall be set to zero"* against ICC.1:2001-04 Table 9's silence,
//! cited on `Malformation`'s doc comment. No oracle is involved and none
//! is claimed.

use std::path::{Path, PathBuf};
use std::process::Command;

fn exe() -> PathBuf {
    // The integration-test binary sits beside the CLI it tests.
    let mut p = std::env::current_exe().expect("test exe path");
    p.pop();
    if p.ends_with("deps") {
        p.pop();
    }
    p.join(format!("iccce{}", std::env::consts::EXE_SUFFIX))
}

fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/synthetic")
        .join(name)
}

/// Run `iccce inspect` and return stdout, or `None` if the binary or the
/// fixture is not present — a missing fixture must skip, never fail, so
/// this suite stays runnable without the licensed corpus.
fn inspect(path: &Path) -> Option<String> {
    if !path.exists() {
        return None;
    }
    let out = Command::new(exe()).arg("inspect").arg(path).output().ok()?;
    Some(String::from_utf8_lossy(&out.stdout).into_owned())
}

/// Parse a `key: value` count line out of the printed report.
fn count_line(stdout: &str, key: &str) -> Option<usize> {
    stdout
        .lines()
        .find_map(|l| l.strip_prefix(key)?.trim().parse().ok())
}

#[test]
fn both_counts_are_printed_unconditionally_even_when_zero_and_even_when_equal() {
    // ★ The line must not be conditional on being interesting. A
    // disclosure that appears only when non-zero teaches a reader to
    // read its absence as "nothing to see" — and the whole value of
    // this line is the comparison, which needs both numbers present.
    let path = fixture("v2-rgb-matrix-trc-curv.icc");
    let Some(stdout) = inspect(&path) else {
        eprintln!("SKIP: fixture or binary not present");
        return;
    };
    assert!(
        count_line(&stdout, "malformations:").is_some(),
        "no `malformations:` line in:\n{stdout}"
    );
    assert!(
        count_line(&stdout, "violations:").is_some(),
        "no `violations:` line in:\n{stdout}"
    );
}

#[test]
fn violations_never_exceeds_malformations_because_it_is_a_subset() {
    // The structural invariant, asserted across every synthetic fixture
    // rather than one — a per-file check that holds vacuously on a
    // well-formed file proves nothing.
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/synthetic");
    let Ok(entries) = std::fs::read_dir(&dir) else {
        eprintln!("SKIP: fixture directory not readable");
        return;
    };
    let mut checked = 0usize;
    for e in entries.flatten() {
        let p = e.path();
        if p.extension().and_then(|s| s.to_str()) != Some("icc") {
            continue;
        }
        let Some(stdout) = inspect(&p) else { continue };
        let (Some(m), Some(v)) = (
            count_line(&stdout, "malformations:"),
            count_line(&stdout, "violations:"),
        ) else {
            // A file the parser refused outright prints neither; that is
            // a different disclosure and not this test's subject.
            continue;
        };
        assert!(
            v <= m,
            "{}: violations {v} exceeds malformations {m} — violations is \
             defined as a SUBSET of the disclosed set",
            p.display()
        );
        checked += 1;
    }
    assert!(
        checked > 0,
        "no fixture produced both counts; this test asserted nothing"
    );
}

#[test]
fn a_v2_reserved_block_discloses_without_accusing_and_a_v4_one_accuses() {
    // ★★★ THE LOAD-BEARING ONE. Same malformation variant, two
    // editions, and the counts must DIFFER. Under the pre-2026-08-21
    // output this observation was unobtainable: both files printed
    // `malformations: 1` and there was no second number.
    //
    // Sourced, not conventional: ICC.1:2022 7.2.19 says the v4 reserved
    // bytes "shall be set to zero"; ICC.1:2001-04 Table 9 says only "44
    // bytes reserved for future expansion" and says it once.
    let v4 = fixture("header-reserved-nonzero.icc");
    let Some(v4_out) = inspect(&v4) else {
        eprintln!("SKIP: v4 fixture or binary not present");
        return;
    };
    assert!(
        v4_out.contains("header.version: 4."),
        "fixture is not v4; this test's premise is gone:\n{v4_out}"
    );
    assert_eq!(
        count_line(&v4_out, "malformations:"),
        Some(1),
        "v4 reserved-nonzero fixture should disclose exactly one thing:\n{v4_out}"
    );
    assert_eq!(
        count_line(&v4_out, "violations:"),
        Some(1),
        "v4's reserved bytes carry a `shall be set to zero` (ICC.1:2022 7.2.19), \
         so the disclosure IS a violation:\n{v4_out}"
    );

    // The v2 arm needs a v2 profile whose reserved block is non-zero.
    // The licensed corpus has five ICC-PUBLISHED ones; without it, the
    // asymmetry cannot be demonstrated and this arm skips rather than
    // asserting something weaker and calling it the same test.
    let Some(v2_path) = licensed_v2_with_reserved_bytes() else {
        eprintln!(
            "SKIP: no v2 profile with a non-zero reserved block available. \
             The v4 arm above ran; the ASYMMETRY did not."
        );
        return;
    };
    let Some(v2_out) = inspect(&v2_path) else {
        eprintln!("SKIP: v2 profile not readable");
        return;
    };
    assert!(
        v2_out.contains("header.version: 2."),
        "expected a v2 profile:\n{v2_out}"
    );
    let m = count_line(&v2_out, "malformations:").expect("count printed");
    let v = count_line(&v2_out, "violations:").expect("count printed");
    assert!(
        m >= 1,
        "{}: expected the reserved block to be disclosed:\n{v2_out}",
        v2_path.display()
    );
    assert_eq!(
        v,
        0,
        "{}: a v2 profile using bytes 84..128 breaches no stated requirement — \
         ICC.1:2001-04 Table 9's cell is the unmodalised \"44 bytes reserved for \
         future expansion\". Reporting it as a violation would accuse an \
         ICC-published, conformant file:\n{v2_out}",
        v2_path.display()
    );
    assert!(
        m > v,
        "{}: the two counts are equal, so this file no longer demonstrates the \
         asymmetry and the test has stopped testing it",
        v2_path.display()
    );
}

/// One of the five ICC-published v2 profiles known to carry an MD5 in
/// the v2 reserved block, if the licensed corpus is present.
///
/// ★ Named individually rather than scanned for, so that a corpus whose
/// contents changed makes this **skip** rather than silently select some
/// other file and assert against it.
fn licensed_v2_with_reserved_bytes() -> Option<PathBuf> {
    const ROOT: &str = r"D:\Dev\iccce-private-fixtures";
    const NAMES: [&str; 5] = [
        "sRGB2014.icc",
        "ITU-RBT709ReferenceDisplay.icc",
        "PSOsc-b_paper_v3_FOGRA54.icc",
        "PSOuncoated_v3_FOGRA52.icc",
        "SC_paper_eci.icc",
    ];
    let root = Path::new(ROOT);
    if !root.exists() {
        return None;
    }
    fn find(dir: &Path, name: &str) -> Option<PathBuf> {
        for e in std::fs::read_dir(dir).ok()?.flatten() {
            let p = e.path();
            if p.is_dir() {
                if let Some(hit) = find(&p, name) {
                    return Some(hit);
                }
            } else if p.file_name().and_then(|s| s.to_str()) == Some(name) {
                return Some(p);
            }
        }
        None
    }
    NAMES.iter().find_map(|n| find(root, n))
}
