# icc-conformance — memory index

- [Pass 4c absolute-intent findings](project_pass4c_absolute_intent_findings.md) — lcms2's wtpt gate is a CONJUNCTION, so a fixture can defeat it instead of a model subtracting it; 8.90e-5 over 729 pts, sensitivity 2310x; the policy is direction-symmetric; NA-008's second arm has no fixture.
- [Pass 5c estimator-BRANCH finding](project_pass5c_estimator_branch_finding.md) — lcms2 has TWO estimators and the destination's HEADER picks; this overturned Pass 5b's claim 1; transicc prints RGB 0..255.
- [Pass 6 compiled-path findings](project_pass6_compiled_path_findings.md) - the gate FAILED at grid 17 (0.297 vs 0.253) and PASSES at the new default 33; order is 1.32 not 2; max-of-max is the wrong estimator.
- [Pass 5b estimator findings](project_pass5b_estimator_findings.md) - the ISO estimator has NO CALLER; the pre-registered prediction split 2/1/1; the gamut absorbs 90%.
- [Parallel-agent build collisions](project_parallel_agent_build_collisions.md) - crates/ breaks mid-session and the engineer sweeps your files into their commits; use a detached worktree.
- [Oracle and tolerance state](project_oracle_and_tolerance_state.md) — harness now drives transicc AND the shipped iccce binary; ΔE allowed; §3.1/§3.3/§5 filled; what is still blank.
- [Synthetic fixture corpus + GP-001](project_synthetic_fixture_corpus_and_gp001.md) — fixtures/synthetic exists (38 profiles); iccce mis-counts mBA curve sets; three ICC_Spec gaps closable.
- [Pass 5 BPC findings](project_lcms2_findings_pass5_bpc.md) — lcms2 drops BPC below 0.002/~0.41 L* (constant not in corpus); T5 measured; the estimators could not be discriminated (superseded by Pass 5b).
- [Pass 4b lcms2 findings](project_lcms2_findings_pass4b_direction_dependence.md) — lcms2 forces trilinear in the B2A direction (NA-006's cost is zero there); forced BPC is the DESTINATION's version; the 4096-entry reverse TRC.
- [Pass 4 lcms2 findings](project_lcms2_findings_pass4_interpolation_and_v2_wtpt.md) — the 4-D CLUT scheme is a hybrid, not tetrahedral; D50 substituted for a v2 display profile's wtpt (11 ΔE at absolute).
- [Pass 3 lcms2 findings](project_lcms2_findings_pass3_quantisation_and_clamping.md) — 16-bit quantisation of tabulated tone curves in float; device values >1.0 when the dst TRC inverse is analytic.
- [Encoded white points differ between profiles](project_encoded_white_points_differ_between_profiles.md) — nominal chromaticity containment ≠ encoded gamut nesting; this falsified a tolerance justification.
- [Two measured lcms2 findings](project_lcms2_findings_legacy_lab_and_forced_bpc.md) — legacy Lab keys off tag type (corpus was wrong); lcms2 forces BPC on v4 perceptual/saturation.
- [lcms2's licence is not uniform](project_lcms2_licence_is_not_uniform.md) — MIT core, GPL-3.0 plugins; the badge lies. Re-verify on every pin move.
- [Doc editing conventions](project_doc_editing_conventions.md) — section-scoped edits on shared docs; verification records are dated and append-only.
