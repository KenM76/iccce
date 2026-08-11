//! # Profile assembly — the 128-byte header, the tag table, the tag data
//!
//! ## Purpose
//!
//! Turn a [`ProfileSpec`] (a header description plus a list of tags) into a
//! complete, byte-exact ICC profile. This is the only place in the crate that
//! knows about offsets, padding, aliasing, or the profile size field.
//!
//! ## Sourcing
//!
//! * Header — `icc__s__header.md`, `evidence: primary_spec`, ICC.1:2022
//!   clause 7.2 and Table 17 (18 fields summing to exactly 128 bytes),
//!   verified against the PDF.
//! * Tag table — `icc__s__tag_table.md`, `evidence: primary_spec`, clauses
//!   7.1.2 (padding), 7.3 (the table), 7.4 (tag data).
//!
//! ## The four rules assembly has to get right, and how each is enforced
//!
//! 1. **The header is exactly 128 bytes** — there is no short header, even
//!    when every optional field is zero. Enforced by a `debug_assert` on the
//!    length before the tag table is appended, and by a unit test.
//! 2. **Tag offsets are from the start of the profile (byte 0)**, not from the
//!    tag table, and the two least-significant bits of every offset shall be
//!    zero (clause 7.3.4). Enforced by construction: the layout walks a
//!    4-aligned cursor and never rounds anything down.
//! 3. **Every tag's data is padded to a 4-byte boundary — including the last**
//!    (clause 7.1.2 c, with d requiring the pad bytes be NULL) — **but the
//!    size field excludes the padding** (clause 7.3.5: "shall be the number of
//!    actual data bytes and shall not include any padding"). Getting this pair
//!    backwards is the single most common way a hand-written profile comes out
//!    subtly wrong: the file parses, and the last tag's size is 1–3 bytes too
//!    large.
//! 4. **The profile size field at offset 0 must equal the actual file length**
//!    — and cannot be known until the layout is done, so it is patched at the
//!    end. A `debug_assert` re-reads it.
//!
//! ## Aliasing is supported deliberately
//!
//! Clause 7.3.1, verbatim: the tag table "may contain multiple tags signatures
//! that all reference the same tag data element offset … In such cases, both
//! the offset and size … shall be the same." Full aliasing is **explicitly
//! legal and encouraged**; *partial* overlap is explicitly illegal. The corpus
//! calls out the trap directly: "a parser that treats any offset collision as
//! an error rejects conformant profiles." [`TagBody::Alias`] exists so
//! `v2-rgb-shared-trc` can put that trap in front of the parser as a
//! **well-formed** fixture that must produce zero malformations.

use crate::bytes::{Buf, D50_ENCODED};

/// A tag's data: either its own bytes, or a reference to another tag's.
#[derive(Debug, Clone)]
pub enum TagBody {
    Own(Vec<u8>),
    /// Full aliasing — this entry gets the **same offset and the same size**
    /// as the named tag, which must appear earlier in the list.
    Alias([u8; 4]),
}

#[derive(Debug, Clone)]
pub struct Tag {
    pub sig: [u8; 4],
    pub body: TagBody,
}

impl Tag {
    #[must_use]
    pub fn own(sig: &[u8; 4], data: Vec<u8>) -> Self {
        Self {
            sig: *sig,
            body: TagBody::Own(data),
        }
    }

    #[must_use]
    pub fn alias(sig: &[u8; 4], of: &[u8; 4]) -> Self {
        Self {
            sig: *sig,
            body: TagBody::Alias(*of),
        }
    }
}

/// The header fields a recipe actually varies, plus its tags.
///
/// Fields **not** exposed are the ones every recipe holds constant and whose
/// constancy is load-bearing for reproducibility: preferred CMM (0 —
/// informational, and "never dispatch on it"), platform (0 = unspecified),
/// flags (0 = not embedded, usable independently), manufacturer/model/creator
/// (0, all legal), attributes (0), profile ID (all-zero = not computed, which
/// is **not** an error), and the 28 reserved bytes (zero). A malformed recipe
/// that needs one of these changed patches it afterwards with the helpers at
/// the bottom of this file, so the deviation is visible as an explicit,
/// commented act rather than as a field somebody set.
#[derive(Debug, Clone)]
pub struct ProfileSpec {
    /// BCD, e.g. `0x0430_0000` = 4.3.0.0. Bytes 10–11 are reserved zero
    /// (clause 7.2.4). **The v2/v4 selector for tag-type availability** — but
    /// NOT, per clause 6.3.4.2 NOTE 3, for the legacy PCSLAB encoding, which
    /// keys off the tag type.
    pub version: u32,
    /// `'scnr'`/`'mntr'`/`'prtr'`/`'link'`/`'abst'`/`'spac'`/`'nmcl'`
    /// (clause 7.2.5, Table 18).
    pub class: [u8; 4],
    /// The device-side data colour space (clause 7.2.6, Table 19).
    pub color_space: [u8; 4],
    /// `'XYZ '` or `'Lab '` only, except for `'link'` (clause 7.2.7).
    pub pcs: [u8; 4],
    /// 0 = perceptual, 1 = media-relative colorimetric, 2 = saturation,
    /// 3 = ICC-absolute colorimetric (clause 7.2.15, Table 23).
    pub rendering_intent: u32,
    pub tags: Vec<Tag>,
}

/// The creation date every profile in this corpus carries:
/// **2026-08-11T00:00:00Z**, as six `uInt16` (clause 7.2.8 / 4.2, UTC).
///
/// ★ **A constant, never a clock.** `fixtures/synthetic/README.md` requires
/// every fixture to be regenerable, and `gen-profiles verify` compares
/// regenerated bytes against the files on disk. One call to a system clock
/// would make every fixture differ from its own generator one second after it
/// was written, and would turn the strongest property this corpus has into
/// noise.
pub const FIXTURE_DATE: [u16; 6] = [2026, 8, 11, 0, 0, 0];

impl ProfileSpec {
    /// Assemble the profile.
    ///
    /// # Panics
    /// If an [`TagBody::Alias`] names a tag that has not already been placed —
    /// a forward alias would need a second pass, and silently tolerating one
    /// would produce a fixture whose table does not say what the recipe says.
    #[must_use]
    pub fn assemble(&self) -> Vec<u8> {
        let n = self.tags.len();
        let table_bytes = 4 + 12 * n;
        // 128 + 4 + 12n is always a multiple of 4, so tag data can begin
        // immediately after the table with no alignment gap.
        let mut cursor = 128 + table_bytes;
        debug_assert_eq!(cursor % 4, 0);

        let mut table = Buf::new();
        table.u32(u32::try_from(n).expect("tag count fits u32"));
        let mut blob = Buf::new();
        // Remembers where each signature's data landed, so an alias can be
        // resolved to the same (offset, size) pair clause 7.3.1 requires.
        let mut placed: Vec<([u8; 4], u32, u32)> = Vec::new();

        for t in &self.tags {
            let (offset, size) = match &t.body {
                TagBody::Own(data) => {
                    let offset = u32::try_from(cursor).expect("tag offset fits u32");
                    let size = u32::try_from(data.len()).expect("tag size fits u32");
                    blob.bytes(data).align4();
                    cursor = 128 + table_bytes + blob.len();
                    (offset, size)
                }
                TagBody::Alias(of) => {
                    let (_, offset, size) = *placed
                        .iter()
                        .find(|(s, _, _)| s == of)
                        .unwrap_or_else(|| panic!("alias target {of:?} not placed yet"));
                    (offset, size)
                }
            };
            placed.push((t.sig, offset, size));
            table.sig(&t.sig).u32(offset).u32(size);
        }

        let mut p = Buf::new();
        p.u32(0) //                       0 profile size — patched at the end
            .u32(0) //                    4 preferred CMM: none (informational)
            .u32(self.version) //         8 profile version, BCD
            .sig(&self.class) //         12 device class
            .sig(&self.color_space) //   16 data colour space
            .sig(&self.pcs); //          20 PCS
        for v in FIXTURE_DATE {
            //                           24 dateTimeNumber, 6 × uInt16, UTC
            p.u16(v);
        }
        p.sig(b"acsp") //                36 the ONLY reliable format check
            .u32(0) //                   40 primary platform: unspecified
            .u32(0) //                   44 flags: not embedded, independent
            .u32(0) //                   48 device manufacturer
            .u32(0) //                   52 device model
            .u32(0) //                   56 device attributes (high word)
            .u32(0) //                   60 device attributes (low word)
            .u32(self.rendering_intent) //64 rendering intent
            .s15_raw(D50_ENCODED[0]) //  68 PCS illuminant X — always D50
            .s15_raw(D50_ENCODED[1]) //  72 PCS illuminant Y
            .s15_raw(D50_ENCODED[2]) //  76 PCS illuminant Z
            .u32(0) //                   80 profile creator
            .zeros(16) //                84 profileID: all-zero = not computed
            .zeros(28); //              100 reserved, shall be zero
        debug_assert_eq!(p.len(), 128, "the header is 128 bytes, always");

        p.bytes(&table.done()).bytes(&blob.done());
        let mut out = p.done();
        let size = u32::try_from(out.len()).expect("profile size fits u32");
        out[0..4].copy_from_slice(&size.to_be_bytes());
        debug_assert_eq!(read_u32(&out, 0) as usize, out.len());
        out
    }
}

// ===========================================================================
// Patch helpers — how a malformed fixture is authored
// ===========================================================================
//
// Every malformed recipe is "a well-formed profile, plus one named,
// documented mutation". That shape is deliberate and it is the reason the
// malformed corpus is worth anything:
//
//   * The mutation is the ONLY difference. A fixture that is malformed in two
//     ways cannot tell you which one the consumer reported, and a consumer
//     that reports the wrong one still looks correct.
//   * The mutation is expressed in the vocabulary of the specification rule it
//     breaks ("set the magic to something other than 'acsp'"), not in the
//     vocabulary of file offsets, so a reader can check the intent without
//     recomputing a layout.
//   * The base is regenerated by the same code as the well-formed fixture, so
//     the two stay in step for free.

/// Read a big-endian `uInt32` at `off`.
///
/// # Panics
/// If the profile is too short to contain it.
#[must_use]
pub fn read_u32(p: &[u8], off: usize) -> u32 {
    u32::from_be_bytes(p[off..off + 4].try_into().expect("4 bytes available"))
}

/// Write a big-endian `uInt32` at `off`.
pub fn set_u32(p: &mut [u8], off: usize, v: u32) {
    p[off..off + 4].copy_from_slice(&v.to_be_bytes());
}

/// The index of the first tag-table entry carrying `sig`.
///
/// # Panics
/// If no such tag exists — a recipe that patches a tag it did not add is a
/// recipe that has drifted from its own base, and failing loudly is the point.
#[must_use]
pub fn tag_index(p: &[u8], sig: &[u8; 4]) -> usize {
    let n = read_u32(p, 128) as usize;
    (0..n)
        .find(|i| &p[132 + 12 * i..136 + 12 * i] == sig)
        .unwrap_or_else(|| panic!("no tag {} in the base profile", show_sig(sig)))
}

/// Byte offset of the `offset` field of tag `i`'s directory entry.
#[must_use]
pub fn tag_offset_field(i: usize) -> usize {
    132 + 12 * i + 4
}

/// Byte offset of the `size` field of tag `i`'s directory entry.
#[must_use]
pub fn tag_size_field(i: usize) -> usize {
    132 + 12 * i + 8
}

/// The `(offset, size)` a tag's directory entry declares.
#[must_use]
pub fn tag_location(p: &[u8], sig: &[u8; 4]) -> (u32, u32) {
    let i = tag_index(p, sig);
    (
        read_u32(p, tag_offset_field(i)),
        read_u32(p, tag_size_field(i)),
    )
}

/// Render a signature for a message, showing non-printable bytes as hex — the
/// corpus warns that signatures must be compared as `u32` and that some are
/// not valid UTF-8 at all, so no `String::from_utf8` appears anywhere here.
#[must_use]
pub fn show_sig(sig: &[u8; 4]) -> String {
    sig.iter()
        .map(|&b| {
            if b.is_ascii_graphic() || b == b' ' {
                (b as char).to_string()
            } else {
                format!("\\x{b:02X}")
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tags;

    fn tiny() -> ProfileSpec {
        ProfileSpec {
            version: 0x0440_0000,
            class: *b"mntr",
            color_space: *b"RGB ",
            pcs: *b"XYZ ",
            rendering_intent: 1,
            tags: vec![
                Tag::own(b"rTRC", tags::curv_identity()),
                // 14 bytes — deliberately NOT a multiple of 4, so the padding
                // rule is exercised by the fixture that tests it.
                Tag::own(b"gTRC", tags::curv_gamma(2.0)),
                Tag::alias(b"bTRC", b"gTRC"),
            ],
        }
    }

    #[test]
    fn header_is_128_bytes_and_the_size_field_matches_the_file() {
        let p = tiny().assemble();
        assert_eq!(read_u32(&p, 0) as usize, p.len());
        assert_eq!(&p[36..40], b"acsp");
        assert_eq!(&p[8..12], &[0x04, 0x40, 0x00, 0x00]);
        assert_eq!(&p[12..16], b"mntr");
        assert_eq!(&p[20..24], b"XYZ ");
        // Reserved 100..128 all zero (clause 7.2.19).
        assert!(p[100..128].iter().all(|&b| b == 0));
    }

    #[test]
    fn every_tag_starts_four_byte_aligned_and_inside_the_file() {
        let p = tiny().assemble();
        let n = read_u32(&p, 128) as usize;
        for i in 0..n {
            let off = read_u32(&p, tag_offset_field(i));
            let size = read_u32(&p, tag_size_field(i));
            assert_eq!(off % 4, 0, "tag {i} misaligned");
            assert!(off >= 132 + 12 * u32::try_from(n).unwrap(), "tag {i} in table");
            assert!(off + size <= read_u32(&p, 0), "tag {i} overruns");
        }
    }

    /// Clause 7.3.5: the size field excludes padding. Clause 7.1.2 c: the
    /// padding is nonetheless present, including after the last tag. The
    /// gamma curve is 14 bytes, so both halves of that pair are observable
    /// here and would not be in a fixture whose tags all happened to be
    /// 4-aligned already.
    #[test]
    fn size_excludes_padding_but_the_padding_is_present() {
        let p = tiny().assemble();
        let (off, size) = tag_location(&p, b"gTRC");
        assert_eq!(size, 14, "the declared size is the actual data length");
        assert_eq!(p.len() % 4, 0, "the last tag is padded too");
        // The two pad bytes after it are NUL (clause 7.1.2 d).
        assert_eq!(&p[(off + size) as usize..(off + size) as usize + 2], &[0, 0]);
    }

    /// Full aliasing: same offset AND same size (clause 7.3.1), which is what
    /// makes it legal rather than a partial overlap.
    #[test]
    fn an_alias_shares_the_offset_and_the_size_exactly() {
        let p = tiny().assemble();
        assert_eq!(tag_location(&p, b"gTRC"), tag_location(&p, b"bTRC"));
    }

    #[test]
    fn assembly_is_deterministic() {
        assert_eq!(tiny().assemble(), tiny().assemble());
    }
}
