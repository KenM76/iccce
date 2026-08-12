//! # iccce-profile — ICC profile parsing and representation
//!
//! Parses ICC v2 and v4 profiles (`ICC.1`, ISO twin `ISO 15076-1`) from
//! bytes into a faithful in-memory representation: the 128-byte header,
//! the tag table, and every tag type real profiles use.
//!
//! ## Contracts
//!
//! - **INVARIANT: no colour maths.** This crate represents what the file
//!   *says*; interpreting it is `iccce-cmm`'s job. The split exists so
//!   that parsing is testable byte-for-byte and the maths is testable
//!   numerically, without either test smearing into the other.
//! - **INVARIANT: the parser reports; it does not repair.** A malformed
//!   tag is represented as malformed, with a diagnostic. A silently
//!   corrected tag is a malformation hidden from the only layer that
//!   could disclose it. (`docs/ARCHITECTURE.md` §3.2.)
//! - **Input is a byte slice, not a file.** The first consumer (`pdfce`)
//!   receives profiles as embedded PDF streams; there may be no
//!   filesystem anywhere near the call site. (`docs/ARCHITECTURE.md` §4.)
//! - **iccMAX (ICC.2) is identified and refused by name**, never
//!   mistaken for corruption and never executed. (`README.md` scope.)
//! - **Structure layouts are cited.** Every field offset and encoding in
//!   this crate names its clause in ICC.1, sourced via the `ICC_Spec`
//!   corpus at `D:\Dev\Rag-Specialized\ICC_Spec\`.
//!
//! ## Status
//!
//! Pass 0: header + tag-table parsing, with malformation reporting and
//! iccMAX refusal. Full tag-type coverage is Pass 2 (`docs/ROADMAP.md`).
//!
//! ## Sourcing
//!
//! All structure layouts cite `D:\Dev\Rag-Specialized\ICC_Spec\`. That
//! corpus is currently built by cross-verifying two independent
//! codebases (ICC's own `icProfileHeader.h`, BSD-3, and `lcms2.h`, MIT)
//! — **not** the ICC.1 PDF, whose retrieval is blocked pending the
//! operator's manual download (`docs/LEGAL.md` §2). Consequently no
//! citation here claims a clause number, and details the corpus marks
//! NOT SOURCED are held opaque (e.g. `Header::attributes`) rather than
//! guessed.

pub mod diag;
pub mod header;
pub mod lut;
pub mod num;
pub mod tag_table;
pub mod tag_types;

pub use diag::{Malformation, ParseError};
pub use header::{Header, ProfileVersion};
pub use num::Signature;
pub use tag_table::TagEntry;

/// A parsed profile: the header, the tag directory, and every rule
/// violation the file carries — reported, never repaired.
///
/// Owns a copy of the profile bytes so tag *data* (Pass 2) can be
/// decoded later without re-reading anything; input is a byte slice
/// because the first consumer receives profiles as embedded PDF streams
/// with no filesystem in sight (`docs/ARCHITECTURE.md` §4).
#[derive(Debug, Clone)]
pub struct Profile {
    pub header: Header,
    pub tags: Vec<TagEntry>,
    /// Every malformation found, in discovery order. Empty means the
    /// checked rules all held — it does not mean "valid": conformance
    /// requirements (which tags a class REQUIRES) are not yet sourced
    /// (corpus ambiguity A30) and are deliberately not asserted.
    pub malformations: Vec<Malformation>,
    bytes: Vec<u8>,
}

impl Profile {
    /// Parse a profile from bytes.
    ///
    /// Fatal refusals (in check order — each earlier check makes the
    /// next meaningful): too short → bad magic → iccMAX → truncated →
    /// tag directory overflows file. Everything else the file gets
    /// wrong is a [`Malformation`] on the returned profile.
    pub fn parse(bytes: &[u8]) -> Result<Profile, ParseError> {
        // 132 = 128-byte header + 4-byte tagCount. `icc__s__header.md`:
        // "The header is 128 bytes even when fields are unused."
        if bytes.len() < 132 {
            return Err(ParseError::TooShort {
                actual: bytes.len(),
            });
        }

        let magic = Signature::read(bytes, 36).expect("length checked");
        if magic != Signature::ACSP {
            return Err(ParseError::BadMagic { found: magic });
        }

        // iccMAX: major version ≥ 5 (`icc__s__header.md` — v5 reclaims
        // the v4 reserved region; parsing it with v4 semantics would be
        // silently wrong, so refuse by name instead).
        let version_raw = num::u32_be(bytes, 8).expect("length checked");
        if version_raw >> 24 >= 5 {
            return Err(ParseError::IccMaxRefused { version_raw });
        }

        let declared = num::u32_be(bytes, 0).expect("length checked");
        if (declared as usize) > bytes.len() {
            return Err(ParseError::Truncated {
                declared,
                actual: bytes.len(),
            });
        }

        // Attacker-controlled count: bound BEFORE allocating
        // (`icc__s__tag_table.md`).
        let tag_count = num::u32_be(bytes, 128).expect("length checked");
        if 132 + 12 * u64::from(tag_count) > bytes.len() as u64 {
            return Err(ParseError::TagCountOverflowsFile {
                tag_count,
                actual: bytes.len(),
            });
        }

        let mut malformations = Vec::new();
        let header = Header::parse(bytes, &mut malformations);
        if (header.size as usize) < bytes.len() {
            malformations.push(Malformation::TrailingBytes {
                declared: header.size,
                actual: bytes.len(),
            });
        }
        let tags = tag_table::parse(bytes, header.size, &mut malformations);

        Ok(Profile {
            header,
            tags,
            malformations,
            bytes: bytes.to_vec(),
        })
    }

    /// The raw data of one tag, when its directory entry is in-bounds.
    /// `None` for an entry whose offset/size overrun the profile — the
    /// entry itself is still listed, because representing the file
    /// faithfully includes representing its broken directory.
    pub fn tag_data(&self, entry: &TagEntry) -> Option<&[u8]> {
        let start = entry.offset as usize;
        let end = start.checked_add(entry.size as usize)?;
        self.bytes.get(start..end)
    }

    /// Decode one tag's data into its typed representation (Pass 2).
    /// `None` when the entry's bytes are out of bounds (that fact is
    /// already a reported [`Malformation`] on the profile); `Some(Err)`
    /// when the bytes exist but the layout is undecodable.
    pub fn decode_tag(
        &self,
        entry: &TagEntry,
    ) -> Option<Result<tag_types::DecodedTag, tag_types::TagDecodeError>> {
        self.tag_data(entry).map(tag_types::decode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a minimal, well-formed synthetic profile: 128-byte header,
    /// one tag ('wtpt', XYZType-shaped, 20 bytes). Category (a) fixture
    /// per LEGAL.md §3 — authored byte-by-byte here, from the layouts
    /// in `icc__s__header.md` / `icc__s__tag_table.md`.
    fn minimal_profile() -> Vec<u8> {
        let mut b = vec![0u8; 164];
        // size = 164 (128 header + 4 count + 12 entry + 20 tag data)
        b[0..4].copy_from_slice(&164u32.to_be_bytes());
        // version 4.3.0 = 04 30 00 00 (icc__s__header.md offset 8)
        b[8..12].copy_from_slice(&[0x04, 0x30, 0x00, 0x00]);
        b[12..16].copy_from_slice(b"mntr");
        b[16..20].copy_from_slice(b"RGB ");
        b[20..24].copy_from_slice(b"XYZ ");
        b[36..40].copy_from_slice(b"acsp"); // magic, offset 36
        // rendering intent 1 = media-relative (offset 64)
        b[64..68].copy_from_slice(&1u32.to_be_bytes());
        // tagCount = 1 at offset 128
        b[128..132].copy_from_slice(&1u32.to_be_bytes());
        // entry: 'wtpt', offset 144, size 20
        b[132..136].copy_from_slice(b"wtpt");
        b[136..140].copy_from_slice(&144u32.to_be_bytes());
        b[140..144].copy_from_slice(&20u32.to_be_bytes());
        // tag data at 144: type 'XYZ ' + 4 reserved zero + 12 bytes value
        b[144..148].copy_from_slice(b"XYZ ");
        b
    }

    /// A deterministic 32-bit LCG. Numerical Recipes' constants
    /// (`a = 1664525`, `c = 1013904223`, modulus 2^32).
    ///
    /// Hand-rolled rather than pulled in, because this crate's
    /// `[dependencies]` is empty by invariant and a robustness test is
    /// not a reason to spend that. Determinism is the requirement, not
    /// statistical quality: a failure here must be reproducible from
    /// the seed alone, on any machine, forever. A `rand` crate whose
    /// algorithm changes between versions would make a past failure
    /// unreproducible, which is the one property this test cannot lose.
    struct Lcg(u32);
    impl Lcg {
        fn next(&mut self) -> u32 {
            self.0 = self.0.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            self.0
        }
    }

    /// ★ THE PARSER MUST NOT PANIC ON ANY INPUT, EVER.
    ///
    /// `Profile::parse` reads **untrusted files**. A malformed profile
    /// arrives from a customer's press, an email attachment, an
    /// embedded stream inside a PDF — none of it authored by us, some
    /// of it hostile. The contract this crate advertises is *report,
    /// do not repair*: a malformation becomes a `Malformation` entry or
    /// an `Err`. **A panic is neither.** It is an unhandled third
    /// outcome that takes the caller's process down and, in a library
    /// that a GUI or a server might embed, converts a bad file into an
    /// availability bug.
    ///
    /// This matters here more than in most parsers because
    /// `header.rs`'s field reads are deliberately `.expect("caller
    /// guarantees >= 132 bytes")` — assertions documenting a contract
    /// rather than handling a reachable case. **That is sound only for
    /// as long as the contract actually holds on every path**, and
    /// nothing before this test checked that claim against anything but
    /// hand-written examples. An `expect` justified by a comment is a
    /// proof obligation, not a proof.
    ///
    /// WHAT THIS IS AND IS NOT. It is a deterministic, seeded mutation
    /// sweep — not a fuzzer. It cannot prove absence of panics; it can
    /// only fail. Real coverage of that claim wants `cargo-fuzz`, which
    /// needs nightly and is not wired into this project. **Stated
    /// plainly so nobody reads a green tick here as "the parser is
    /// proven total."** What it does buy is that the cheap, dense,
    /// highest-yield region of the input space — truncations and single
    /// byte flips around a *valid* profile, which is exactly where
    /// length/offset arithmetic breaks — is swept on every CI run.
    ///
    /// THE ASSERTION IS THE ABSENCE OF A PANIC. A panic in a Rust test
    /// fails it, so calling `parse` is itself the check; there is
    /// deliberately no `assert!` on the *result*, because both `Ok` and
    /// `Err` are correct answers here and asserting which one would be
    /// asserting something this test does not know.
    #[test]
    fn parser_never_panics_on_mutated_input() {
        let seed = minimal_profile();

        // (1) Every truncation, including empty. Truncation is the
        //     single most productive malformation for a format built
        //     from offset+size pairs: every bound check gets exercised
        //     against a buffer that ends before the structure does.
        for n in 0..=seed.len() {
            let _ = Profile::parse(&seed[..n]);
        }

        // (2) Every byte position, against values chosen to hit the
        //     edges of the arithmetic rather than the middle: 0, 1,
        //     0x7f, 0x80, 0xff make u32/i32 boundaries and sign flips
        //     out of length and offset fields.
        for i in 0..seed.len() {
            for v in [0x00u8, 0x01, 0x7f, 0x80, 0xff] {
                let mut m = seed.clone();
                m[i] = v;
                if let Ok(p) = Profile::parse(&m) {
                    // Walk the tags too — a panic hiding in tag_data's
                    // slicing would otherwise never be reached, since
                    // parse() alone does not read every tag's payload.
                    for t in &p.tags {
                        let _ = p.tag_data(t);
                    }
                }
            }
        }

        // (3) Multi-byte random damage, seeded. Catches interactions
        //     that single-byte edits cannot: a plausible tag count
        //     together with a plausible-but-wrong offset, for instance.
        let mut rng = Lcg(0x1CCC_E000);
        for _ in 0..4096 {
            let mut m = seed.clone();
            let edits = 1 + (rng.next() % 8) as usize;
            for _ in 0..edits {
                let i = (rng.next() as usize) % m.len();
                m[i] = (rng.next() >> 24) as u8;
            }
            // Sometimes truncate as well, so the two mutation classes
            // compose rather than only ever appearing alone.
            if rng.next() % 4 == 0 {
                let n = (rng.next() as usize) % (m.len() + 1);
                m.truncate(n);
            }
            if let Ok(p) = Profile::parse(&m) {
                for t in &p.tags {
                    let _ = p.tag_data(t);
                }
            }
        }
    }

    #[test]
    fn minimal_profile_parses_clean() {
        let p = Profile::parse(&minimal_profile()).unwrap();
        assert_eq!(p.header.version.to_string(), "4.3.0");
        assert_eq!(p.header.device_class.to_string(), "'mntr'");
        assert_eq!(p.header.pcs.to_string(), "'XYZ '");
        assert_eq!(p.tags.len(), 1);
        assert_eq!(p.tags[0].sig.to_string(), "'wtpt'");
        assert_eq!(p.tags[0].type_sig.unwrap().to_string(), "'XYZ '");
        assert_eq!(p.malformations, vec![]);
        assert_eq!(p.tag_data(&p.tags[0]).unwrap().len(), 20);
    }

    #[test]
    fn bad_magic_is_refused_not_guessed() {
        let mut b = minimal_profile();
        b[36] = b'x';
        assert!(matches!(
            Profile::parse(&b),
            Err(ParseError::BadMagic { .. })
        ));
    }

    #[test]
    fn iccmax_is_refused_by_name() {
        let mut b = minimal_profile();
        b[8] = 0x05; // major version 5 = iccMAX
        let err = Profile::parse(&b).unwrap_err();
        assert!(matches!(err, ParseError::IccMaxRefused { .. }));
        // The refusal must NAME iccMAX — "refuse it by name" is the
        // scope rule; an anonymous rejection reads as corruption.
        assert!(err.to_string().contains("iccMAX"));
    }

    #[test]
    fn truncated_is_refused() {
        let b = minimal_profile();
        assert!(matches!(
            Profile::parse(&b[..150]),
            Err(ParseError::Truncated { .. })
        ));
    }

    #[test]
    fn hostile_tag_count_is_bounded_before_allocation() {
        let mut b = minimal_profile();
        b[128..132].copy_from_slice(&u32::MAX.to_be_bytes());
        assert!(matches!(
            Profile::parse(&b),
            Err(ParseError::TagCountOverflowsFile { .. })
        ));
    }

    #[test]
    fn trailing_bytes_reported_not_erased() {
        let mut b = minimal_profile();
        b.extend_from_slice(&[0u8; 3]); // container padding
        let p = Profile::parse(&b).unwrap();
        assert!(p.malformations.iter().any(|m| matches!(
            m,
            Malformation::TrailingBytes {
                declared: 164,
                actual: 167
            }
        )));
        // ...and the header still says what the FILE says.
        assert_eq!(p.header.size, 164);
    }

    #[test]
    fn tag_overrun_reported_entry_retained() {
        let mut b = minimal_profile();
        b[140..144].copy_from_slice(&9999u32.to_be_bytes()); // entry size
        let p = Profile::parse(&b).unwrap();
        assert!(
            p.malformations
                .iter()
                .any(|m| matches!(m, Malformation::TagOverrun { index: 0, .. }))
        );
        // Reported, not repaired: the entry keeps the file's value.
        assert_eq!(p.tags[0].size, 9999);
        assert_eq!(p.tag_data(&p.tags[0]), None);
    }

    #[test]
    fn nonzero_header_reserved_reported_verbatim() {
        let mut b = minimal_profile();
        b[100] = 0xAB;
        let p = Profile::parse(&b).unwrap();
        assert!(
            p.malformations
                .iter()
                .any(|m| matches!(m, Malformation::HeaderReservedNonZero))
        );
        assert_eq!(p.header.reserved[0], 0xAB); // kept, not zeroed
    }
}
