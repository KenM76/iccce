//! # Named colours — Pass 7's core (`namedColor2Type` consumption)
//!
//! The consumer that makes spot colour colorimetric rather than
//! approximated — what `pdfce` wants for `Separation`/`DeviceN`.
//!
//! ## The encoding rule, and why it is THE point of this module
//!
//! `ncl2` PCS coordinates use the **LEGACY 16-bit PCSLAB encoding in a
//! profile of ANY version** — ICC.1:2022 clause 10.17 verbatim via
//! `ICC_Spec/icc/icc__type__text_mluc_namedcolor2.md` (primary_spec;
//! corpus A26, RESOLVED): "this tag uses the legacy 16-bit PCSLAB
//! encoding … not the 16-bit PCSLAB encoding that is defined in
//! 6.3.4.2", and Table 66: "Only PCSXYZ and legacy 16-bit PCSLAB
//! encodings are permitted. PCS values shall be relative
//! colorimetric." Getting this wrong costs ~0.4% in L* — "spot
//! colours are brand matching … the least acceptable place in the
//! whole system for a sub-perceptual defect." The decode goes through
//! [`crate::pcs_encoding::LabEncoding::Legacy`] so the invariant tests
//! there cover this path.
//!
//! The corpus's normativity mismatch is noted for a future validator:
//! legacy L* > 100 is "shall not" in 10.10 but "should not" in 10.17
//! (spec defect §4) — this consumer decodes whatever is stored and
//! leaves judgement to the caller.

use crate::lut_transform::{PcsKind, PcsValue};
use crate::pcs_encoding::{LabEncoding, decode_pcs_xyz};
use iccce_color::{Lab, Xyz};
use iccce_profile::Profile;
use iccce_profile::num::Signature;
use iccce_profile::tag_types::{NamedColor2, TagData};

const NCL2: Signature = Signature(0x6E63_6C32); // 'ncl2' (tag AND type sig)
const PCS_LAB: Signature = Signature(0x4C61_6220); // 'Lab '

/// Errors building the table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NamedColorError {
    /// No `ncl2` tag, or it decodes to something else.
    NoUsableNcl2,
}

impl std::fmt::Display for NamedColorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "no usable namedColor2 (ncl2) tag")
    }
}

/// One resolved named colour.
#[derive(Debug, Clone, PartialEq)]
pub struct NamedColor {
    /// Full name: prefix + root + suffix, concatenated at lookup
    /// (clause 10.17: "the name is not stored whole anywhere"),
    /// rendered lossily from the NUL-terminated ASCII fields.
    pub name: String,
    /// The PCS value, decoded per the tag's rule (legacy Lab / XYZ);
    /// relative colorimetric per Table 66.
    pub pcs: PcsValue,
    /// Device coordinates as stored: full-range 0..1 (0x0000 → min,
    /// 0xFFFF → max — NOT the u1Fixed15 PCS encoding; "different
    /// scale in the same struct"). Empty when `nDeviceCoords == 0`
    /// ("this field is not given" — a normal spot-library shape).
    pub device: Vec<f64>,
}

/// A profile's named-colour table.
#[derive(Debug, Clone)]
pub struct NamedColors {
    colors: Vec<NamedColor>,
}

impl NamedColors {
    /// Build from a parsed profile (first `ncl2` per A13; PCS kind
    /// from the header).
    pub fn from_profile(profile: &Profile) -> Result<NamedColors, NamedColorError> {
        let entry = profile
            .tags
            .iter()
            .find(|t| t.sig == NCL2)
            .ok_or(NamedColorError::NoUsableNcl2)?;
        let ncl2 = match profile.decode_tag(entry) {
            Some(Ok(d)) => match d.data {
                TagData::NamedColor2(n) => n,
                _ => return Err(NamedColorError::NoUsableNcl2),
            },
            _ => return Err(NamedColorError::NoUsableNcl2),
        };
        let pcs_kind = if profile.header.pcs == PCS_LAB {
            PcsKind::Lab
        } else {
            PcsKind::Xyz
        };
        Ok(Self::from_ncl2(&ncl2, pcs_kind))
    }

    /// Build from an already-decoded tag (the `pdfce` bridge's entry
    /// point, where the profile arrives as a stream).
    pub fn from_ncl2(ncl2: &NamedColor2, pcs_kind: PcsKind) -> NamedColors {
        let field = |bytes: &[u8]| -> String {
            let end = bytes.iter().position(|&b| b == 0).unwrap_or(bytes.len());
            String::from_utf8_lossy(&bytes[..end]).into_owned()
        };
        let prefix = field(&ncl2.prefix);
        let suffix = field(&ncl2.suffix);
        let colors = ncl2
            .entries
            .iter()
            .map(|e| {
                let pcs = match pcs_kind {
                    // A26: legacy, ANY profile version — decoded via
                    // the same functions the exact-value invariant
                    // tests pin.
                    PcsKind::Lab => PcsValue::Lab(Lab {
                        l: LabEncoding::Legacy.decode_l(e.pcs_coords[0]),
                        a: LabEncoding::Legacy.decode_ab(e.pcs_coords[1]),
                        b: LabEncoding::Legacy.decode_ab(e.pcs_coords[2]),
                    }),
                    PcsKind::Xyz => PcsValue::Xyz(Xyz {
                        x: decode_pcs_xyz(e.pcs_coords[0]),
                        y: decode_pcs_xyz(e.pcs_coords[1]),
                        z: decode_pcs_xyz(e.pcs_coords[2]),
                    }),
                };
                NamedColor {
                    name: format!("{prefix}{}{suffix}", field(&e.root_name)),
                    pcs,
                    device: e
                        .device_coords
                        .iter()
                        .map(|&d| f64::from(d) / 65535.0)
                        .collect(),
                }
            })
            .collect();
        NamedColors { colors }
    }

    pub fn len(&self) -> usize {
        self.colors.len()
    }

    pub fn is_empty(&self) -> bool {
        self.colors.is_empty()
    }

    /// Exact-name lookup. Case-sensitive: spot names are identifiers
    /// in a vendor's catalogue, and "pantone" vs "PANTONE" being
    /// different entries is representable in the format — fuzzy
    /// matching is a caller policy, not a format fact.
    #[must_use]
    pub fn find(&self, name: &str) -> Option<&NamedColor> {
        self.colors.iter().find(|c| c.name == name)
    }

    pub fn iter(&self) -> impl Iterator<Item = &NamedColor> {
        self.colors.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Synthetic ncl2 with the legacy full-scale codes: the D1
    /// invariant carried into the spot-colour path — 0xFF00/0x8000/
    /// 0x8000 decodes to Lab(100, 0, 0) EXACTLY. The wrong (v4)
    /// decode would give 99.6109 — sub-perceptual, invisible to ΔE,
    /// fatal to a brand colour.
    #[test]
    fn ncl2_lab_uses_legacy_encoding_exactly() {
        let ncl2 = NamedColor2 {
            vendor_flag: 0,
            n_device_coords: 0,
            prefix: *b"ACME \0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            suffix: [0u8; 32],
            entries: vec![iccce_profile::tag_types::NamedColorEntry {
                root_name: {
                    let mut r = [0u8; 32];
                    r[..5].copy_from_slice(b"White");
                    r
                },
                pcs_coords: [0xFF00, 0x8000, 0x8000],
                device_coords: vec![],
            }],
        };
        let table = NamedColors::from_ncl2(&ncl2, PcsKind::Lab);
        let c = table.find("ACME White").expect("name assembled");
        match c.pcs {
            PcsValue::Lab(lab) => {
                assert_eq!(lab.l, 100.0);
                assert_eq!(lab.a, 0.0);
                assert_eq!(lab.b, 0.0);
            }
            ref other => panic!("{other:?}"),
        }
        assert!(c.device.is_empty()); // nDeviceCoords == 0: legal
    }

    /// The committed synthetic fixture parses into a usable table.
    #[test]
    fn fixture_ncl2_builds() {
        let path = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/synthetic/v2-ncl2-named.icc"
        );
        let Ok(bytes) = std::fs::read(path) else {
            panic!("committed fixture missing: {path}");
        };
        let profile = Profile::parse(&bytes).unwrap();
        let table = NamedColors::from_profile(&profile).unwrap();
        assert!(!table.is_empty());
        for c in table.iter() {
            assert!(!c.name.is_empty());
        }
    }
}
