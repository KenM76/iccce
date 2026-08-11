# Agent memory — icc-spec-librarian

- [ICC ToS blocks automated access](project_icc_tos_automated_access_blocker.md) — never agent-fetch color.org; the ICC.1:2022 PDF is already in `_sources/` (Ken, 2026-08-11); ask him for any further ICC document
- [ICC_Spec corpus sourcing route](reference_icc_spec_corpus_sourcing_route.md) — DemoIccMAX (BSD-3) + lcms2 (MIT) on GitHub, cross-verified; plus Sharma ΔE2000 ground truth
- [ICC PDF drops minus signs](reference_icc_pdf_symbol_font_sign_loss.md) — Symbol-font glyphs land in U+F0xx and every extractor silently discards them; exhaustive map + which engine per structure; Read tool cannot render PDF pages here
- [ICC-absolute: clause 6.2.3 is backwards](reference_icc_absolute_intent_clause_trap.md) — the prose inverts the media-white ratio; implement Eq (1)–(6); `chad` is never a render-time step
- [Label the predicate, not just the payload](feedback_label_the_predicate_not_just_the_payload.md) — cross-verified constants say nothing about the rule selecting them; the A1 near-miss
- [DERIVED values need a second pass](feedback_derived_values_need_a_second_pass.md) — a provenance label is not an arithmetic audit; the C2 D50-chromaticity erratum, caught by an iccce unit test
- [Reading source is not observing behaviour](feedback_reading_source_is_not_observing_behaviour.md) — "lcms2 does X" needs a run, not a citation; the C3 retraction of a divergence that measurement found absent
- [lcms2 measured quirks live in one file](reference_lcms2_measured_behaviour_file.md) — `M<n>` ids at pin 21c582a; check M2 (forced BPC on v4 perceptual/saturation) before blaming any transform
- [ICC.1 conformance binds only *reading*](reference_icc_conformance_clause_binds_only_reading.md) — clause 5 never binds a CMM's computed output; verdicts say "diverges", not "non-conforming" (A39b); plus the clamp-the-input pattern and the 6.4-vs-6.5 device-range trap
