# icc-engineer memory index

- [Loop pacing and parallel dispatch](feedback_loop_pacing_and_parallel_dispatch.md) — Ken wants ~300s loop ticks and parallel agents; serialize only on real file conflicts
- [Request channel polling cadence](reference_request_channel_polling.md) — poll the pdfce↔iccce `open/` folder every ~15 min all session, not just at startup
- [Compatibility, not certified compliance](feedback_compatibility_not_compliance.md) — never stall on a standard because certification needs measurement hardware
- [Ghent PDF Output Suite 5.0 target](project_ghent_compatibility.md) — the compatibility target, where it lives, and why none of it may enter the MIT repo
- [Check _sources before accepting a corpus gap](feedback_check_sources_before_accepting_corpus_gap.md) — RAG digests report gaps the primary PDFs in `_sources/` do not have
- [Fixture separation and cancelling rivals](feedback_fixture_separation_and_cancelling_rivals.md) — a zero-separation fixture manufactures a false PASS; a rival that enters twice can cancel
