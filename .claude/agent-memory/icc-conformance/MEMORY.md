# icc-conformance — memory index

- [Oracle and tolerance state](project_oracle_and_tolerance_state.md) — harness now drives transicc AND the shipped iccce binary; ΔE allowed; §3.1/§3.3/§5 filled; what is still blank.
- [Synthetic fixture corpus + GP-001](project_synthetic_fixture_corpus_and_gp001.md) — fixtures/synthetic exists (38 profiles); iccce mis-counts mBA curve sets; three ICC_Spec gaps closable.
- [Pass 4 lcms2 findings](project_lcms2_findings_pass4_interpolation_and_v2_wtpt.md) — the 4-D CLUT scheme is a hybrid, not tetrahedral; D50 substituted for a v2 display profile's wtpt (11 ΔE at absolute).
- [Pass 3 lcms2 findings](project_lcms2_findings_pass3_quantisation_and_clamping.md) — 16-bit quantisation of tabulated tone curves in float; device values >1.0 when the dst TRC inverse is analytic.
- [Encoded white points differ between profiles](project_encoded_white_points_differ_between_profiles.md) — nominal chromaticity containment ≠ encoded gamut nesting; this falsified a tolerance justification.
- [Two measured lcms2 findings](project_lcms2_findings_legacy_lab_and_forced_bpc.md) — legacy Lab keys off tag type (corpus was wrong); lcms2 forces BPC on v4 perceptual/saturation.
- [lcms2's licence is not uniform](project_lcms2_licence_is_not_uniform.md) — MIT core, GPL-3.0 plugins; the badge lies. Re-verify on every pin move.
- [Doc editing conventions](project_doc_editing_conventions.md) — section-scoped edits on shared docs; verification records are dated and append-only.
