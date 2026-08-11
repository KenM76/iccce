//! # The tag table — the directory at offset 128
//!
//! Structure per `ICC_Spec/icc/icc__s__tag_table.md`: a big-endian
//! `tagCount` at offset 128, then `tagCount` 12-byte entries of
//! (signature, offset, size), then tag data in arbitrary order.
//!
//! Rules from the corpus that shape this module:
//!
//! - `offset` is from the **start of the profile**, not the tag table.
//! - `size` excludes padding to the next 4-byte boundary.
//! - Tag data need not be in table order or contiguous — never infer a
//!   tag's extent from its neighbour.
//! - **Two entries may share one offset** — legal and common
//!   (`rTRC`/`gTRC`/`bTRC` sharing a curve). Entries here are plain
//!   `(offset, size)` views into the profile bytes, so shared data
//!   costs nothing and owns nothing (no double-free class of bug to
//!   have).
//! - The first 8 bytes at `offset` are the `icTagBase`: a type
//!   signature + 4 reserved-zero bytes. Content begins at `offset + 8`.

use crate::diag::Malformation;
use crate::num::{Signature, u32_be};

/// One directory entry, as the file states it. Whether the entry is
/// *usable* is a separate question answered by the malformation list —
/// an entry pointing off the end of the file is still faithfully
/// represented here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TagEntry {
    /// Tag signature (`'desc'`, `'wtpt'`, `'A2B0'`, …).
    pub sig: Signature,
    /// Byte offset of the tag's data from the start of the profile.
    pub offset: u32,
    /// Actual byte count of the tag's data, excluding alignment padding.
    pub size: u32,
    /// The tag TYPE signature from the `icTagBase` at `offset`, when
    /// the entry is in-bounds and large enough to have one. `None`
    /// means "could not be read", never "was zero".
    pub type_sig: Option<Signature>,
}

/// Parse the directory. The caller has verified `tagCount` fits the
/// file BEFORE this is invoked (the count is attacker-controlled and
/// multiplies by 12 — `icc__s__tag_table.md`), so allocation here is
/// bounded by real bytes.
pub(crate) fn parse(
    bytes: &[u8],
    declared_size: u32,
    malformations: &mut Vec<Malformation>,
) -> Vec<TagEntry> {
    let g = "caller guarantees the directory fits in `bytes`";
    let tag_count = u32_be(bytes, 128).expect(g) as usize;
    let table_end = 132 + 12 * tag_count;

    let mut entries: Vec<TagEntry> = Vec::with_capacity(tag_count);

    for i in 0..tag_count {
        let base = 132 + 12 * i;
        let sig = Signature::read(bytes, base).expect(g);
        let offset = u32_be(bytes, base + 4).expect(g);
        let size = u32_be(bytes, base + 8).expect(g);

        // Validation per the corpus's table — each failure is reported
        // and the entry is kept verbatim. Overflow-safe in u64: offset
        // and size are attacker-controlled u32s whose sum wraps.
        let end = u64::from(offset) + u64::from(size);
        let in_bounds = end <= u64::from(declared_size);
        if !in_bounds {
            malformations.push(Malformation::TagOverrun { index: i, sig });
        }
        if (offset as usize) < table_end {
            malformations.push(Malformation::TagOverlapsTable { index: i, sig });
        }
        if offset % 4 != 0 {
            malformations.push(Malformation::TagMisaligned { index: i, sig });
        }
        if size < 8 {
            malformations.push(Malformation::TagTooSmall { index: i, sig });
        }
        if let Some(prev) = entries.iter().position(|e| e.sig == sig) {
            malformations.push(Malformation::DuplicateTagSignature {
                first_index: prev,
                dup_index: i,
                sig,
            });
        }

        // Read the icTagBase (type signature + 4 reserved bytes) only
        // when the entry can legitimately contain one.
        let type_sig = if in_bounds && size >= 8 {
            let ts = Signature::read(bytes, offset as usize);
            if let Some(reserved) = bytes.get(offset as usize + 4..offset as usize + 8) {
                if reserved.iter().any(|&b| b != 0) {
                    malformations.push(Malformation::TagBaseReservedNonZero { index: i, sig });
                }
            }
            ts
        } else {
            None
        };

        entries.push(TagEntry {
            sig,
            offset,
            size,
            type_sig,
        });
    }
    entries
}
