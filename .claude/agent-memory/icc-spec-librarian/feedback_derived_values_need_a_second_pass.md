---
name: derived-values-need-a-second-pass
description: A DERIVED evidence label states provenance, not correctness — recompute every value the corpus derives, twice, print the intermediate, ALWAYS evaluate a printed value as an INTERVAL not a point (C5), and NEVER print a derivation unlabelled (C8, the manufactured claim); the C2 D50-chromaticity erratum, the C5 Annex-D ground-truth loss, and C8's "a SYMPTOM claim is a claim about test power"
metadata:
  type: feedback
---

**Any value this corpus computes rather than transcribes gets the same two-independent-passes treatment as a transcription, and must print its intermediate so a reader can falsify it by inspection.**

**Status: BINDING, corpus-wide, 2026-08-11** — the lead engineer approved both open questions. `DERIVED` is now a defined row in the `LEGAL_NOTE.md` §3 tier table (previously it existed only as a passing clause in §4 rule 4), with the two-passes + printed-intermediate + tier-table requirements written as conditions of the mark, C2 named as the incident, and the rule that an incompletely-labelled `DERIVED` value is demoted to `not_sourced`. §4 rule 4 points at §3. Generalised lesson filed outside this corpus at `C:\personal_rag\claude_code\lesson_20260811_derived_is_the_only_rag_evidence_tier_with_no_verification_step.md` (agent-workflow finding, not a colour one).

**Why:** on 2026-08-11 `cie/cie__ref__colorimetry_core.md` §2 derived D50's chromaticity from ICC's sourced 4-figure triple (0.9642, 1.0000, 0.8249) and printed **0.34567 / 0.35850**. Correct is **0.345703 / 0.358539** (sum 2.7891). The printed pair is the *5-figure* tier's answer (0.96422/1.00000/0.82521, sum 2.78943) and the widely published D50 chromaticity — i.e. the corpus committed **the exact precision-mixing trap the same section's ★ paragraph warns about, two lines above the warning**. The line was correctly labelled `DERIVED, not sourced`, and §5 already forbade using corpus-derived values as test expectations. **That labelling bounded the damage and did not catch the error**: every other evidence tier is verified by re-reading a source, but the DERIVED tier had *no* verification step — its correctness rested on one unaudited act of arithmetic. Recorded as **C2** in `icc__ref__spec_defects.md` §10 (ids: C1 = the A1 retraction, §9). Sibling lesson: [[label-the-predicate-not-just-the-payload]].

## ★★★ C5, 2026-08-12 — TWO PASSES AT HIGHER PRECISION IS ONE PASS. A displayed value is an INTERVAL.

**The rule, and it is the sharpest form of this memory:**

> **Any claim that a published number "does not follow" from other published
> numbers must be evaluated over the INTERVALS their display precision defines,
> not at their midpoints. Re-running a point evaluation at higher precision is
> not a second pass — it is the same pass.**

**Why.** `icc__ref__spec_defects.md` §13.2 (5th pass) checked ICC.1:2022 Annex D.6.3's worked example by applying Eq (D.6) to Table D.2's **displayed** 4-dp values, in `float64` **and** in `decimal` at 30 digits, printing every intermediate — **a genuinely careful audit that answered the wrong question.** Both passes evaluate the same point; `decimal` tests the multiplication, not whether the input is the number the authors used. It concluded **two** cells were defective. Under interval evaluation (`±0,000 05`, all endpoint combinations) only `X` is outside its attainable range `[0,013 165 , 0,013 304]`; **`Y`'s range `[0,013 680 , 0,013 818]` contains the printed `0,0138` — `Y` was never wrong.** `Y` was off by `0,4 %` — **exactly the size of a display rounding, which is the signature of NO error.** The tell was in the same table: `Z` was marked ✔ at the same `0,4 %` scale.

**What it cost, and this is why it is worth this much space.** Calling one typo two made it look like a *pattern*, and the pattern justified the conclusion *"the black row is not ground truth."* **That sentence disqualified ICC.1:2022 Annex D.6.3 — the ONLY published input→output numeric ground truth for any ICC transform** — and `icc-librarian` then carried *"there is no published-ground-truth row for any transform"* through **eleven consecutive filings.** Started at Table D.3, the example reproduces **12 of 12** of Table D.5's integers exactly. **The corpus did not fail to find the ground truth. It found it, and then threw it away with a rounding error.**

**How to apply:** every time this corpus compares one printed number against another — spectral integrations, `chad` values, the D50 chromaticity work, any future worked example. If the two agree to within their display precision, **that is agreement**, not a near-miss. Only a disagreement larger than the whole rounding interval is a finding. And **before writing off a fixture, ask what it costs if the write-off is wrong** — here it cost the project its only oracle for a year of filings.

## ★★★ C8, 2026-08-17 — an UNLABELLED derivation, and the first corpus defect that was MANUFACTURED

**The rule, and it is new:**

> **A `SYMPTOM` claim is a claim about TEST POWER, not about magnitude. Write
> "which classes of test detect this, and which have exactly zero power" — never
> only "the error is ≈ N".** A magnitude cannot be checked by running anything;
> a test-power claim can be falsified in one afternoon, and *was*.

**Why.** `iec/iec__s__srgb.md` carried *"using the wrong sRGB breakpoint `0.03928` affects **8-bit codes 10 and 11 only**; max linear error ≈ `1.3×10⁻⁴` ≈ `0.05 ΔE76`."* **Both halves false.** `10/255 = 0.039 216` is *below* `0.03928` and `11/255 = 0.043 137` is *above* `0.04045`, so **both codes take the same branch under both breakpoints — separation at 8-bit precision is EXACTLY ZERO, all 256 codes.** True maximum `7.5548×10⁻⁷` at `V = 0.039 302 447` (an **interior** maximum: at the knee itself both branches are linear and the separation is zero, so *a test placed at `V = 0.03928` also has zero power*). **172× smaller than claimed.**

**Three things make C8 a different species from C1–C7:**
1. **It was MANUFACTURED, not mis-read.** Every earlier defect mis-read something real. **No source in `_sources/` contains `0.03928` as text at all** — it was computed in-corpus and printed **without the `DERIVED` label** that a table twenty lines below it carries. **An unlabelled derivation is indistinguishable from a transcription: a reader cannot tell there is nothing behind it to check against.** *(The corpus is not under version control, so which pass wrote it is unrecoverable — recorded as "provenance not recoverable" rather than guessed.)*
2. **The refutation was thirty lines away and internally invisible.** The same file already tabulated the `0.03928` pairing's discontinuity as `−7.55×10⁻⁷`, and **the separation between two curves cannot exceed the gap one fails to close.** One subtraction. Six days, two files, nobody did it. ⟹ **internal availability does not produce catching** — see [[corpus-defects-are-caught-from-outside]].
3. **★ FALSE PRECISION REPELS CHECKING.** "8-bit codes 10 and 11" has the texture of a computed result, so no reader recomputed it. **A claim precise enough to look computed is the claim least likely to be recomputed. Vagueness invites checking; false precision repels it.** This is the inverse of the usual instinct to make claims more specific.

**What it cost:** `icc-conformance` carried it into a Rust doc comment and **five** tests, then **injected `0.03928` and all five passed.** The corrected form tells a reader that 8-bit vectors, round-trips, and differential tests against lcms2 *all* have zero power here — only a correctly-built reference at non-8-bit precision inside a `1.17×10⁻³` window detects it.

**How to apply:** any SYMPTOM/consequence paragraph in this corpus must (a) carry `DERIVED` if it is derived, (b) name the check that would falsify it, and (c) be sanity-bounded against any related magnitude already printed in the same file. The bound check is one subtraction and is the whole audit.

**How to apply:**
- Writing any number with a `DERIVED` mark: compute it twice by different means (e.g. Python `decimal` at high precision *and* a second route), state that in the file, and **show the intermediate** (the sum, the reciprocal, the ASCII bytes).
- When a value has multiple published precision tiers, print a **tier table** rather than one number, and mark which tier the project uses. A single number invites the mix.
- Treat "this is the number everyone publishes" as a *source claim needing its own label* — never as a substitute for the derivation you said you performed. Under-recorded provenance is worse than wrong arithmetic; both were live readings of C2 and the record keeps both.
- **Expect implementation tests to audit this corpus.** C2 was the first corpus defect found by *running code*: an iccce test recomputed the derivation from the sourced inputs and failed by ~3×10⁻⁵. That test shape (`code recomputes the corpus's derivation from the corpus's sourced inputs`) has no external oracle so it proves no colour science, but it is the only automatic check the DERIVED tier has. Suggest one for every derived value the implementation touches.
- Keep corrected errors visible with a post-mortem, as C1 and C2 both are. A corpus that only lists other people's errors is not being audited.
- **This applies to the corpus's OWN bookkeeping, not just to colour numbers.** 5th pass, 2026-08-11: while adding three rows to the ambiguity register I wrote that the previous pass's total of 41 "was itself short by two" — an assertion invented to explain a number, never checked. The reconciliation `44 − 3 new rows = 41` refuted it immediately and the sentence was replaced with the reconciliation itself. **Caught before commit only because the recount was done in code rather than by adding three.** Counts, tallies and "N rows" claims are derived values; the delta check costs one line and is the whole audit.
