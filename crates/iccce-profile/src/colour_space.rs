//! # Colour-space signatures and their component counts
//!
//! ## Purpose
//!
//! Answers one question, for one caller need: **given a profile's
//! declared data colour space, how many components does it carry?**
//!
//! This exists because a consumer must be able to validate a container's
//! own declaration against the profile's *before* building a transform.
//! The motivating case is PDF: an `/ICCBased` colour space stream
//! carries an `/N` entry stating the number of components, and a PDF
//! engine has to check it against the embedded profile. Until now iccce
//! offered only `Chain::input_channels()`, which reports the *tag's*
//! count and requires a **built chain** — too late, and answering a
//! different question.
//!
//! ## ★ Sourcing — this is spec data, and it was not written from memory
//!
//! Project rule 2. The table below was produced by `icc-spec-librarian`
//! from **ICC.1:2022 clause 7.2.6, Table 19**, transcribed twice with
//! two independent PDF text engines (pypdf and poppler
//! `pdftotext -layout`), with the hex values additionally matched
//! against ICC's own published `icProfileHeader.h` and lcms2's
//! `lcms2.h`. **Four routes, no disagreement.** Full record:
//! `ICC_Spec/icc/icc__s__colour_space_signatures.md`.
//!
//! ### ★★ The component count is a TWO-TABLE JOIN, not a transcription
//!
//! This is the most important thing to know before trusting anything
//! here, and it is easy to state wrongly.
//!
//! **Table 19 has three columns — colour space type, signature, hex. It
//! has NO component-count column.** The counts come from two different
//! places:
//!
//! - For the fourteen `xCLR` rows, the count is **in the row's own
//!   label** ("5 colour", "10 colour") and is mechanically decodable as
//!   the hex value of the signature's first character.
//! - For the eleven **named** spaces (`XYZ `, `Lab `, `RGB `, `GRAY`,
//!   `CMYK`, …) the count appears only in **Table 41** (clause 10.10,
//!   "lut16Type channel encodings"), and is obtained by **counting
//!   non-dash cells in a row**. `GRAY` = 1 rests on a single `K` and
//!   three dashes.
//!
//! Table 41 is normative for every LUT type (10.10–10.13, 10.16) and for
//! 10.2 and 10.21, but **its stated subject is channel assignment and
//! order, not count.** So a `Signature → count` map is *derived from two
//! tables*, and ICC.1 does not publish it as such. That is recorded as
//! corpus ambiguity **A50**, and this module cites it rather than
//! implying the specification states these counts directly.
//!
//! ## Enumeration bounds — measured, not assumed
//!
//! - The `xCLR` family **starts at `2CLR`**. There is **no `1CLR` and no
//!   `0CLR`** in ICC.1:2022 *or* in ICC.1:2001-04.
//! - **`FCLR` = 15 is the ceiling**; there is **no 16-channel ICC.1
//!   signature**, because the family is one hex digit wide.
//! - ★ **`1CLR` (`0x31434C52`) nevertheless exists in ICC's own
//!   published `icProfileHeader.h`** (as `icSig1colorData`, aliased
//!   `icSigMCH1Data`) **and in lcms2 — and is in neither edition of
//!   ICC.1.** Which standard defines it is **not sourced** (corpus
//!   ambiguity **A49**). A component-count map built by transcribing
//!   ICC's C header — the obvious shortcut — silently accepts it as
//!   though ICC.1 had blessed it. [`components`] therefore recognises it
//!   and **flags it**, rather than either rejecting a signature real
//!   files may carry or laundering it into the standard's table.
//! - ★ iccMAX (ICC.2) has a separate `nc0000`…`ncFFFF` family carrying
//!   the count in the **low 16 bits**, so 16-and-more channels *are*
//!   expressible there. Every claim in this module is scoped to **ICC.1**
//!   — which is all iccce parses.
//!
//! ## ★★ The trap in the PCS field, which costs conformance if missed
//!
//! It is natural to assume the header's PCS field is a two-value enum,
//! `XYZ ` or `Lab `. **That assumption rejects every conformant
//! DeviceLink profile.** ICC.1:2022 clause 7.2.7, verbatim:
//!
//! > "For all profile classes (see Table 18), other than a DeviceLink
//! > profile, the PCS encoding shall be either PCSXYZ or PCSLAB… When
//! > the profile/device class is a DeviceLink profile, the value of the
//! > PCS shall be **the appropriate data colour space from Table 19**."
//!
//! [`is_valid_pcs`] encodes exactly that, keyed on the device class.
//!
//! ## ★ A deliberate divergence from lcms2, and why
//!
//! **lcms2's `cmsChannelsOf()` returns `3` for an unrecognised
//! signature.** (`cmsChannelsOfColorSpace()` returns `-1`; the
//! deprecated wrapper maps anything negative to 3.) So a profile with a
//! corrupted or mistyped colour-space signature becomes a silent
//! three-channel transform there — a wrong answer with no signal, which
//! is the exact defect class this project is organised against.
//!
//! *(Evidence class: read from lcms2's source at the pinned commit, not
//! executed — this is `impl_crosscheck`, not measured behaviour.)*
//!
//! **iccce's accessor is fallible.** An unknown signature returns
//! [`ComponentCount::Unknown`] carrying the signature, and the caller
//! decides. Rule 6, one layer up: report, do not repair. lcms2 also
//! carries `'LuvK'` (4 channels), which appears in no ICC document.

use crate::num::Signature;

/// A colour-space signature's component count, and what kind of answer
/// it is.
///
/// Three variants rather than an `Option<usize>` because "how many
/// components" has three genuinely different answers here and collapsing
/// them loses the one a caller most needs to act on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentCount {
    /// The signature is in ICC.1:2022 Table 19 and this is its count.
    Known(usize),
    /// The signature is **not in ICC.1 Table 19**, but is defined in
    /// ICC's own `icProfileHeader.h` and in lcms2, and its count is
    /// unambiguous. Currently only `1CLR`.
    ///
    /// Separated from [`Self::Known`] so a caller can choose: accept it
    /// (real files may carry it, and its meaning is not in doubt) while
    /// still being able to *report* that the profile uses a signature
    /// ICC.1 does not define. Merging it into `Known` would hide that;
    /// merging it into `Unknown` would refuse a file whose intent is
    /// perfectly clear. See corpus ambiguity **A49**.
    NotInIccOneTable19(usize),
    /// Not recognised. **iccce does not guess.**
    ///
    /// ★ This is where iccce and lcms2 part company: lcms2's
    /// `cmsChannelsOf()` would answer `3` here. Answering a plausible
    /// number to an unanswerable question is how a corrupt signature
    /// becomes a silently wrong picture.
    Unknown(Signature),
}

impl ComponentCount {
    /// The count when one is available, regardless of whether ICC.1
    /// defines the signature.
    ///
    /// ★ Use this only where the ICC.1-membership distinction genuinely
    /// does not matter. If you are producing a diagnostic, match on the
    /// variants instead — that distinction is the reason they exist.
    #[must_use]
    pub fn count(self) -> Option<usize> {
        match self {
            Self::Known(n) | Self::NotInIccOneTable19(n) => Some(n),
            Self::Unknown(_) => None,
        }
    }

    /// True when ICC.1:2022 Table 19 lists the signature.
    #[must_use]
    pub fn is_icc1_defined(self) -> bool {
        matches!(self, Self::Known(_))
    }
}

/// Component count for a data colour space signature.
///
/// Source: **ICC.1:2022 clause 7.2.6 Table 19** (the signatures) joined
/// with **Table 41** (the counts for named spaces) — see the module doc
/// on why that join is worth stating, and corpus ambiguity **A50**.
///
/// The `xCLR` family is decoded arithmetically from the signature rather
/// than enumerated, because the standard's own row labels are
/// arithmetic ("5 colour" ↔ `5CLR`) and fourteen hand-written rows would
/// be fourteen chances to typo a number that no test could distinguish
/// from a correct one.
///
/// ```
/// use iccce_profile::colour_space::{components, ComponentCount};
/// use iccce_profile::Signature;
///
/// assert_eq!(components(Signature(0x434D_594B)).count(), Some(4)); // 'CMYK'
/// assert_eq!(components(Signature(0x4752_4159)).count(), Some(1)); // 'GRAY'
/// assert_eq!(components(Signature(0x3743_4C52)).count(), Some(7)); // '7CLR'
/// assert!(matches!(
///     components(Signature(0x0000_0000)),
///     ComponentCount::Unknown(_)
/// ));
/// ```
#[must_use]
pub fn components(sig: Signature) -> ComponentCount {
    // The named spaces — ICC.1:2022 Table 19 for the signature, Table 41
    // for the count. Ordered as Table 19 prints them, so the two can be
    // read side by side.
    let named = match sig.0 {
        0x5859_5A20 => Some(3), // 'XYZ ' nCIEXYZ or PCSXYZ
        0x4C61_6220 => Some(3), // 'Lab ' CIELAB or PCSLAB
        0x4C75_7620 => Some(3), // 'Luv ' CIELUV
        0x5943_6272 => Some(3), // 'YCbr' YCbCr
        0x5978_7920 => Some(3), // 'Yxy ' CIEYxy
        0x5247_4220 => Some(3), // 'RGB '
        0x4752_4159 => Some(1), // 'GRAY' — Table 41: one 'K', three dashes
        0x4853_5620 => Some(3), // 'HSV '
        0x484C_5320 => Some(3), // 'HLS '
        0x434D_594B => Some(4), // 'CMYK'
        0x434D_5920 => Some(3), // 'CMY '
        _ => None,
    };
    if let Some(n) = named {
        return ComponentCount::Known(n);
    }

    // The xCLR family. Table 19 defines '2CLR'..'FCLR' = 2..15; the
    // count is the first character read as a hex digit. '1CLR' is
    // NOT in Table 19 (A49) and is reported as such; '0CLR' is in no
    // source at all and stays Unknown.
    let bytes = sig.0.to_be_bytes();
    if &bytes[1..] == b"CLR" {
        let n = match bytes[0] {
            b'0'..=b'9' => usize::from(bytes[0] - b'0'),
            b'A'..=b'F' => usize::from(bytes[0] - b'A') + 10,
            _ => return ComponentCount::Unknown(sig),
        };
        return match n {
            2..=15 => ComponentCount::Known(n),
            // '1CLR': defined in ICC's icProfileHeader.h and lcms2,
            // absent from both editions of ICC.1. A49.
            1 => ComponentCount::NotInIccOneTable19(1),
            _ => ComponentCount::Unknown(sig),
        };
    }

    ComponentCount::Unknown(sig)
}

/// Whether `pcs` is a permissible PCS field value for a profile of
/// device class `device_class`.
///
/// **ICC.1:2022 clause 7.2.7, verbatim:** *"For all profile classes (see
/// Table 18), other than a DeviceLink profile, the PCS encoding shall be
/// either PCSXYZ or PCSLAB… When the profile/device class is a
/// DeviceLink profile, the value of the PCS shall be the appropriate
/// data colour space from Table 19."*
///
/// ★ **The DeviceLink case is the one that gets missed.** Treating the
/// PCS field as a two-value enum rejects every conformant DeviceLink
/// profile — a `shall`-level conformance error produced by an
/// over-tight reader.
#[must_use]
pub fn is_valid_pcs(pcs: Signature, device_class: Signature) -> bool {
    const PCS_XYZ: Signature = Signature(0x5859_5A20);
    const PCS_LAB: Signature = Signature(0x4C61_6220);
    const CLASS_LINK: Signature = Signature(0x6C69_6E6B); // 'link'
    if device_class == CLASS_LINK {
        components(pcs).count().is_some()
    } else {
        pcs == PCS_XYZ || pcs == PCS_LAB
    }
}

/// The result of comparing a profile header's declared component count
/// against a tag's actual channel count.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelAgreement {
    /// Header and tag agree.
    Agree(usize),
    /// ★ They **disagree**, and ICC.1:2022 **states no requirement that
    /// they agree** — see [`channel_agreement`] for the full sourcing.
    /// This is a disclosure, not a conformance verdict.
    Disagree { header: usize, tag: usize },
    /// The header's colour space is not recognised, so no comparison is
    /// possible. Reported rather than assumed-agreeing.
    HeaderCountUnknown(Signature),
}

/// Cross-check a header's declared component count against a tag's
/// channel count.
///
/// ## ★★ What ICC.1 actually says about this: NOTHING
///
/// This was sourced specifically, because "the standard is silent" and
/// "the standard requires agreement" are different claims and only one
/// of them is true here.
///
/// **ICC.1:2022 nowhere requires a LUT tag's input/output channel count
/// to agree with the header's data colour space, and nowhere states what
/// a reader should do on mismatch.** The complete set of what *is*
/// written:
///
/// - **`shall`, binding the count** — exactly two clauses, and neither
///   is a LUT: **10.4** `colorantOrderType` and **10.5**
///   `colorantTableType`, both verbatim *"The 'count of colorants' shall
///   be in agreement with the data colour space signature of 7.2.6."*
/// - **`shall`, but binding assignment and order rather than count** —
///   10.2, 10.10, 10.11, 10.12, 10.13, 10.16, 10.21: *"Each colour
///   component shall be assigned to an input and output channel… as
///   shown in Table 41."*
/// - **`should`** — **10.17** `namedColor2Type`: *"This representation
///   should be consistent with the 'number of device coordinates'
///   field…"*
/// - Clause **8** defines profile types by their **required tag set**,
///   never by header colour space.
///
/// *(Method, stated because a sweep is only evidence about the paths it
/// covered: the corpus grepped ICC.1:2022 for `7.2.6`, `data colour
/// space`, `Table 19`, `in agreement`, `shall match`, `consistent with`,
/// `number of (input|output|device|colour) channels`, `number of
/// components`, `equal to the number`, `same as the number`,
/// `N-component` and `Monochrome`, and read clause 8 in full. Corpus
/// ambiguity **A48**.)*
///
/// ## What follows, for wording
///
/// Because clause 5 binds only the ability to **read** profiles, a
/// `CMYK` header with a three-channel `A2B0` is **not declared
/// non-conformant by any clause**. So iccce discloses the disagreement
/// and does **not** call it an error:
///
/// > *"header/tag channel-count disagreement; ICC.1:2022 states no
/// > requirement (A48)"*
///
/// ★ The one place a mismatch **is** a `shall` violation is
/// `colorantOrderType` / `colorantTableType`, per 10.4/10.5 above. A
/// caller checking those tags is entitled to a harder verdict than this
/// function gives; this function is for the LUT and general case.
///
/// ## ★★ Status: UNVALIDATED AGAINST A REAL POPULATION
///
/// **As of 2026-08-17 this cross-check has never been observed to fire
/// on a real file, and that is not evidence that it does not.** Every
/// corpus this project holds is curated — 20 profiles extracted from a
/// conformance suite and 40 published by ICC — and **all 60 agree**
/// (measured: CMYK 33, RGB 25, GRAY 1, 7CLR 1; zero unrecognised
/// signatures, zero PCS-field violations).
///
/// A curated population is exactly the wrong place to look for
/// malformation. `pdfce` has been asked
/// (`open/request_header_tag_channel_disagreement.md`) for the rate
/// across ~6,000 real-world PDFs, because only a population nobody
/// selected can answer it.
///
/// **This is stated rather than left implicit because the two failure
/// modes point opposite ways:** a disclosure that never fires is dead
/// code dressed as protection, and one that fires constantly is noise
/// people learn to skip. Which this is, is currently **unknown**, and a
/// reader deserves to know it is defending a *possibility* rather than
/// an *observation*.
///
/// What is measured, and is the reason to keep it regardless: of the
/// **1,020 single-byte corruptions of `'CMYK'`, exactly one lands on
/// another valid signature — `'CMY '`, which is 3 components.** A single
/// bit flip produces a perfectly plausible header. The tag is the only
/// thing that still disagrees.
#[must_use]
pub fn channel_agreement(header_space: Signature, tag_channels: usize) -> ChannelAgreement {
    match components(header_space).count() {
        Some(n) if n == tag_channels => ChannelAgreement::Agree(n),
        Some(n) => ChannelAgreement::Disagree {
            header: n,
            tag: tag_channels,
        },
        None => ChannelAgreement::HeaderCountUnknown(header_space),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every named row of ICC.1:2022 Table 19, with its Table 41 count.
    ///
    /// Expectation source: the specification, via
    /// `ICC_Spec/icc/icc__s__colour_space_signatures.md` — transcribed
    /// twice from the PDF with two independent text engines and matched
    /// against ICC's `icProfileHeader.h` and lcms2's `lcms2.h`. **Not
    /// produced by the function under test**, and not recalled.
    ///
    /// The signatures are written as byte literals so a reader can check
    /// them against the standard without decoding hex.
    #[test]
    fn table_19_named_spaces() {
        let cases: &[(&[u8; 4], usize)] = &[
            (b"XYZ ", 3),
            (b"Lab ", 3),
            (b"Luv ", 3),
            (b"YCbr", 3),
            (b"Yxy ", 3),
            (b"RGB ", 3),
            (b"GRAY", 1),
            (b"HSV ", 3),
            (b"HLS ", 3),
            (b"CMYK", 4),
            (b"CMY ", 3),
        ];
        for (sig, expect) in cases {
            let s = Signature(u32::from_be_bytes(**sig));
            assert_eq!(
                components(s),
                ComponentCount::Known(*expect),
                "{:?} should be {expect} components",
                std::str::from_utf8(*sig).unwrap()
            );
        }
    }

    /// The whole `xCLR` family and its exact bounds.
    ///
    /// ★ The bounds are the point. `2CLR` is the floor and `FCLR` = 15
    /// the ceiling in ICC.1; `1CLR` is outside the standard but real;
    /// `0CLR` is in no source at all.
    #[test]
    fn xclr_family_and_its_bounds() {
        for n in 2..=15usize {
            let c = char::from_digit(u32::try_from(n).unwrap(), 16)
                .unwrap()
                .to_ascii_uppercase();
            let sig = Signature(u32::from_be_bytes([c as u8, b'C', b'L', b'R']));
            assert_eq!(
                components(sig),
                ComponentCount::Known(n),
                "{c}CLR should be {n} components"
            );
        }
        // '1CLR' — in ICC's own C header and in lcms2, in NEITHER
        // edition of ICC.1. A49.
        assert_eq!(
            components(Signature(u32::from_be_bytes(*b"1CLR"))),
            ComponentCount::NotInIccOneTable19(1),
            "1CLR must be recognised but flagged as outside ICC.1 Table 19"
        );
        // '0CLR' is in no source.
        assert!(matches!(
            components(Signature(u32::from_be_bytes(*b"0CLR"))),
            ComponentCount::Unknown(_)
        ));
        // And there is no 16-channel ICC.1 signature: the family is one
        // hex digit wide, so 'GCLR' is simply not a signature.
        assert!(matches!(
            components(Signature(u32::from_be_bytes(*b"GCLR"))),
            ComponentCount::Unknown(_)
        ));
    }

    /// ★★ The deliberate divergence from lcms2, asserted so it cannot
    /// regress into a convenience default.
    ///
    /// lcms2's `cmsChannelsOf()` answers **3** for an unrecognised
    /// signature. If iccce ever does the same, a corrupted colour-space
    /// signature becomes a silent three-channel transform — a wrong
    /// answer that looks exactly like a right one.
    #[test]
    fn unknown_signature_is_refused_not_defaulted_to_three() {
        for bad in [
            0x0000_0000u32,
            0xFFFF_FFFF,
            u32::from_be_bytes(*b"CMYX"), // one byte off 'CMYK'
            u32::from_be_bytes(*b"rgb "), // wrong case
            u32::from_be_bytes(*b"LuvK"), // lcms2 carries this; no ICC document does
        ] {
            let got = components(Signature(bad));
            assert!(
                matches!(got, ComponentCount::Unknown(_)),
                "signature {bad:#010x} must be Unknown, got {got:?}"
            );
            assert_eq!(
                got.count(),
                None,
                "an unknown signature must yield no count — lcms2 would answer 3 here"
            );
        }
    }

    /// How many one-byte corruptions of `'CMYK'` still yield a component
    /// count — i.e. how large the silent-corruption surface actually is.
    ///
    /// ## ★★ This test was TAUTOLOGICAL until 2026-08-17. Read why.
    ///
    /// The first version enumerated all 1 020 corruptions correctly and
    /// then asserted:
    ///
    /// ```ignore
    /// if let Some(n) = components(sig).count() { survivors.push((s, n)); }
    /// // ...
    /// assert!(components(sig).count() == Some(*n));
    /// ```
    ///
    /// `n` **came from** `components(sig).count()`, so the assertion
    /// compared a call with its own result: **true by construction, for
    /// any implementation whatsoever.** The interesting quantity — the
    /// *number* of survivors — reached only a `println!`, invisible
    /// without `--nocapture`.
    ///
    /// So the behaviour was protected and the **enumeration was not**,
    /// in the module that documents this exact hazard at greatest
    /// length. Caught by `icc-librarian` reading the source, not by the
    /// suite. Recorded rather than quietly fixed, because
    /// *"expectations must not come from the code under test"* is easiest
    /// to violate in the test that looks most thorough.
    ///
    /// ## What is asserted now, and where the expectation comes from
    ///
    /// **Exactly one** of the 1 020 single-byte corruptions of `'CMYK'`
    /// lands on another signature ICC.1:2022 Table 19 defines: `'CMY '`,
    /// at 3 components. That is a fact about **Table 19's signature
    /// set**, derivable by hand from the standard — `'CMYK'` differs
    /// from `'CMY '` only in its last byte (`K` → space), and no other
    /// Table 19 signature is within one byte of `'CMYK'`. It is not
    /// produced by the function under test.
    ///
    /// ★ The consequence is the point: **a single bit flip in a PDF
    /// stream can turn a 4-channel CMYK profile into a perfectly
    /// plausible 3-channel CMY one**, and nothing in the header looks
    /// wrong afterwards. Only the *tag* still disagrees, which is what
    /// [`channel_agreement`] is for.
    #[test]
    fn exactly_one_single_byte_corruption_of_cmyk_is_another_valid_signature() {
        let good = *b"CMYK";
        let mut survivors: Vec<(String, usize)> = Vec::new();
        let mut examined = 0usize;
        for pos in 0..4 {
            for b in 0u8..=255 {
                let mut s = good;
                s[pos] = b;
                if s == good {
                    continue;
                }
                examined += 1;
                if let Some(n) = components(Signature(u32::from_be_bytes(s))).count() {
                    survivors.push((String::from_utf8_lossy(&s).into_owned(), n));
                }
            }
        }

        // Premise: the enumeration really covered the whole surface.
        // 4 positions x 256 values, minus the 4 that reproduce 'CMYK'.
        assert_eq!(examined, 1020, "the corruption enumeration is incomplete");

        // The expectation, stated independently of `components`.
        assert_eq!(
            survivors,
            vec![("CMY ".to_string(), 3usize)],
            "the set of single-byte corruptions of 'CMYK' that remain valid signatures has \
             changed. Expected exactly one — 'CMY ' at 3 components, which is the only Table 19 \
             signature within one byte of 'CMYK'. Got: {survivors:?}"
        );
    }

    /// 7.2.7 — the DeviceLink exception, which an over-tight reader
    /// breaks.
    #[test]
    fn devicelink_pcs_may_be_any_data_colour_space() {
        let xyz = Signature(u32::from_be_bytes(*b"XYZ "));
        let lab = Signature(u32::from_be_bytes(*b"Lab "));
        let cmyk = Signature(u32::from_be_bytes(*b"CMYK"));
        let link = Signature(u32::from_be_bytes(*b"link"));
        let mntr = Signature(u32::from_be_bytes(*b"mntr"));

        // Non-DeviceLink: XYZ or Lab only.
        assert!(is_valid_pcs(xyz, mntr));
        assert!(is_valid_pcs(lab, mntr));
        assert!(
            !is_valid_pcs(cmyk, mntr),
            "a monitor profile may not declare a CMYK PCS"
        );

        // DeviceLink: any Table 19 data colour space.
        assert!(
            is_valid_pcs(cmyk, link),
            "a DeviceLink profile's PCS SHALL be a data colour space — rejecting this rejects \
             every conformant DeviceLink profile (7.2.7)"
        );
        assert!(is_valid_pcs(xyz, link));
        assert!(
            !is_valid_pcs(Signature(0), link),
            "an unrecognised signature is still not a valid DeviceLink PCS"
        );
    }

    /// The cross-check reports rather than judges, and says so in its
    /// own type.
    #[test]
    fn channel_agreement_reports_all_three_outcomes() {
        let cmyk = Signature(u32::from_be_bytes(*b"CMYK"));
        assert_eq!(channel_agreement(cmyk, 4), ChannelAgreement::Agree(4));
        assert_eq!(
            channel_agreement(cmyk, 3),
            ChannelAgreement::Disagree { header: 4, tag: 3 }
        );
        assert!(matches!(
            channel_agreement(Signature(0xDEAD_BEEF), 3),
            ChannelAgreement::HeaderCountUnknown(_)
        ));
    }
}
