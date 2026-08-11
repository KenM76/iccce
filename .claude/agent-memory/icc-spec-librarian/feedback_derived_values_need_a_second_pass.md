---
name: derived-values-need-a-second-pass
description: A DERIVED evidence label states provenance, not correctness — recompute every value the corpus derives, twice, and print the intermediate; the C2 D50-chromaticity erratum
metadata:
  type: feedback
---

**Any value this corpus computes rather than transcribes gets the same two-independent-passes treatment as a transcription, and must print its intermediate so a reader can falsify it by inspection.**

**Status: BINDING, corpus-wide, 2026-08-11** — the lead engineer approved both open questions. `DERIVED` is now a defined row in the `LEGAL_NOTE.md` §3 tier table (previously it existed only as a passing clause in §4 rule 4), with the two-passes + printed-intermediate + tier-table requirements written as conditions of the mark, C2 named as the incident, and the rule that an incompletely-labelled `DERIVED` value is demoted to `not_sourced`. §4 rule 4 points at §3. Generalised lesson filed outside this corpus at `C:\personal_rag\claude_code\lesson_20260811_derived_is_the_only_rag_evidence_tier_with_no_verification_step.md` (agent-workflow finding, not a colour one).

**Why:** on 2026-08-11 `cie/cie__ref__colorimetry_core.md` §2 derived D50's chromaticity from ICC's sourced 4-figure triple (0.9642, 1.0000, 0.8249) and printed **0.34567 / 0.35850**. Correct is **0.345703 / 0.358539** (sum 2.7891). The printed pair is the *5-figure* tier's answer (0.96422/1.00000/0.82521, sum 2.78943) and the widely published D50 chromaticity — i.e. the corpus committed **the exact precision-mixing trap the same section's ★ paragraph warns about, two lines above the warning**. The line was correctly labelled `DERIVED, not sourced`, and §5 already forbade using corpus-derived values as test expectations. **That labelling bounded the damage and did not catch the error**: every other evidence tier is verified by re-reading a source, but the DERIVED tier had *no* verification step — its correctness rested on one unaudited act of arithmetic. Recorded as **C2** in `icc__ref__spec_defects.md` §10 (ids: C1 = the A1 retraction, §9). Sibling lesson: [[label-the-predicate-not-just-the-payload]].

**How to apply:**
- Writing any number with a `DERIVED` mark: compute it twice by different means (e.g. Python `decimal` at high precision *and* a second route), state that in the file, and **show the intermediate** (the sum, the reciprocal, the ASCII bytes).
- When a value has multiple published precision tiers, print a **tier table** rather than one number, and mark which tier the project uses. A single number invites the mix.
- Treat "this is the number everyone publishes" as a *source claim needing its own label* — never as a substitute for the derivation you said you performed. Under-recorded provenance is worse than wrong arithmetic; both were live readings of C2 and the record keeps both.
- **Expect implementation tests to audit this corpus.** C2 was the first corpus defect found by *running code*: an iccce test recomputed the derivation from the sourced inputs and failed by ~3×10⁻⁵. That test shape (`code recomputes the corpus's derivation from the corpus's sourced inputs`) has no external oracle so it proves no colour science, but it is the only automatic check the DERIVED tier has. Suggest one for every derived value the implementation touches.
- Keep corrected errors visible with a post-mortem, as C1 and C2 both are. A corpus that only lists other people's errors is not being audited.
