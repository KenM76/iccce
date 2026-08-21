//! # Pass K — **black preservation**: the instrument, built before the feature
//!
//! **The letter names the channel, not a position in the alphabet.** Passes G,
//! H and I are a sequence; this one is *K* because its whole subject is the
//! fourth colorant of a CMYK separation. Nothing here depends on a Pass J
//! existing, and nothing here should be renumbered if one appears.
//!
//! ## What this module is for, and what it deliberately is not
//!
//! `crates/` contained **no black-preservation code** at the commit this was
//! written against. That was not an oversight in this file — it was the
//! premise. This module is the apparatus that measures the feature, written
//! first so that the numbers it would be graded by were chosen before anybody
//! knew which ones would be convenient.
//!
//! ### ★★★ THE FEATURE LANDED 2026-08-18, AND THE TOLERANCES DID NOT MOVE
//!
//! `crates/iccce-cmm/src/black_preserve.rs` implements
//! `KMapping::EqualLightness` behind `Chain::with_black_preservation`, exposed
//! as `iccce transform --preserve-black <policy>`. §E and §F were repointed at
//! that surface the same day. **What a reader should check first is not that
//! the rows are green but that the BOUNDS are the ones written before the code
//! existed** — `EXACT_ZERO` is still exactly `0`, `TABLE_INTERPOLATION` is
//! still one 16-bit quantum, `ORACLE_CHAIN` is still two, and `E4`'s bound is
//! still computed at run time from the fixture. Nothing was widened; two rows
//! moved from red to green because two numbers moved.
//!
//! | row | before | after |
//! |---|---|---|
//! | `E1` `k-only-in-implies-k-only-out` (licensed) | `0.705320` | **`0.000000`** |
//! | `F5` the same on the committed fixture (CI) | `0.420705` | **`0.000000`** |
//!
//! ★★ **Four rows were ADDED rather than any bound relaxed**, because the
//! landed feature made four questions askable that had no answer before:
//! `E7`/`F8` (the preservation path must not touch an input that does not
//! qualify — graded at exactly zero, `SelfConsistency`), `E8` (on a
//! same-profile pair the construction is provably the identity — a
//! `DerivedExpectation`, and **iccce is right where lcms2 is `6.1e-5` wrong**),
//! and `E9` (the only row in this suite that can tell *which* of the two
//! published definitions iccce implements).
//!
//! It therefore does three things and refuses a fourth:
//!
//! 1. **§A records the BASELINE** — what the shipped engine does *today*, with
//!    no feature, when a K-only build is re-separated into a real CMYK
//!    destination. This is the number nobody had.
//! 2. **§B–§C measure the two predicates the requirement is usually stated
//!    in** — the Ghent Output Suite's *Four different Grays* patch — as quantities, on both
//!    sides of a boundary this project does not own.
//! 3. **§D–§E build the discriminating probes**, one of which was **RED on
//!    purpose** until the feature existed, and is now the row that says the
//!    feature does what it is named for.
//! 4. It **does not implement, model, prototype or recommend** a
//!    black-preservation algorithm. Where it needs to know what a K-preserving
//!    answer looks like it asks **lcms2**, and every such row is labelled a
//!    cross-check against a **vendor extension outside the ICC intent
//!    numbering** — see "The oracle is a non-ICC intent" below.
//!
//! ### ★★★ §G ADDED 2026-08-19 — what the policy COSTS, in colour
//!
//! Everything above §G is in **device units**, for the reason the headline
//! finding below states: ΔE2000 is blind to the *defect* preservation exists to
//! fix. That left `NUMERIC_CLAIMS.md`'s **NA-012** carrying `UNMEASURED` in its
//! cost field — *"nobody has measured the ΔE2000 between the preserved answer
//! and the colorimetric one on a cross-press pair"* — and §G is that
//! measurement. Sixteen rows, of which **seven run in CI** and nine need the
//! licensed corpus.
//!
//! | | |
//! |---|---|
//! | the cost, `ISO Coated v2 300% (ECI)` → `GWG_GenericCMYK`, media-relative | `G1`/`G2`, licensed |
//! | the same over **every** ordered pair of the six real CMYK members | `G16`, licensed |
//! | the same on a **committed** pair, against a **closed form** | `G11`–`G15`, CI |
//! | why the pair §F already had **cannot** carry a ΔE row | `G9`/`G10`, CI |
//!
//! ★★ **Its evidence class is `SelfConsistency` and that is the ceiling, not a
//! shortcoming**: the question *"what does this policy cost relative to not
//! applying it"* is intrinsically a comparison of the engine with itself, and
//! `ICC_Spec` **A51** is a closed negative — no published value can exist for
//! what preservation *should* return. lcms2 appears only as a **ruler**, and
//! `G6` grades the claim that the ruler does not decide the answer.
//!
//! ★★ **A fixture had to be authored for it.** `v2-cmyk-chromatic-neutral`
//! separates the two candidate answers by `0.420705` of **ink** and is sound
//! for §F — and its `B2A0` is not the inverse of its `A2B0`, so a ΔE measured on
//! it prices the fixture. Worse, its black ink is **spectrally neutral**, which
//! makes a preserved answer at matched lightness a **metamer** of the
//! colorimetric one: a fixture can separate by half a unit of ink and still
//! report a cost of zero for a reason that has nothing to do with the policy.
//! `fixtures/synthetic/v2-cmyk-warm-black.icc` varies exactly that one
//! variable. See §G's own header for both traps and the table of which row
//! catches which.
//!
//! ## ★★★ THE HEADLINE FINDING, stated before anything else
//!
//! > **ΔE is blind to the defect black preservation exists to fix.**
//!
//! On `ISO Coated v2 300% (ECI)`, converting the K-only ramp `(0,0,0,K)` into
//! the *same* profile at media-relative colorimetric (41-point ramp, measured
//! 2026-08-17; every one of these is restated by a row at run time):
//!
//! | quantity | measured |
//! |---|---|
//! | max chromatic ink where there should be none | **`0.705320`** (cyan, at `K = 1.0`) |
//! | max total area coverage | **`2.753549`** (275 %), from an input TAC of `1.00` |
//! | max reduction of the black channel itself | **`0.360889`** (at `K = 0.60`) |
//! | max ΔE2000 between that build and the K-only build | **`0.136 090`** |
//! | max disagreement with lcms2 on the same points | **`6.3×10⁻⁵`** device |
//!
//! Read those five rows together and the design of this whole module follows:
//!
//! - The engine is **not wrong**. It agrees with the pinned oracle to
//!   `6.3×10⁻⁵`; the separation is the *profile's own* `B2A1` table doing
//!   exactly what its author built it to do. A row that "caught" this as an
//!   iccce defect would be reporting a defect that is not there.
//! - The colour is **preserved almost perfectly** — `0.13 ΔE2000`, an eighth
//!   of §2's perceptibility anchor. **A conformance suite that graded this
//!   subject in ΔE would pass it and report nothing.** §A row `A4` asserts
//!   exactly that, and its *passing* is the finding.
//! - What is wrong is a **device-space** fact: three plates of ink under what
//!   the document said was a single-plate black. That costs registration,
//!   moiré, text edge definition and press stability, none of which is a
//!   colour difference and none of which any ΔE can see.
//!
//! **Consequence for the tolerance budget, and it is the load-bearing one:
//! every graded row in this module that is about preservation is in
//! NORMALISED DEVICE UNITS.** The two ΔE rows here exist to bound how much
//! *colour* a preservation decision costs, never to detect whether
//! preservation happened.
//!
//! ## ★★ The second finding: the shortcut that works on exactly one vendor
//!
//! The obvious cheap answer to *"we need K-only neutrals"* is *"use the
//! saturation intent — print profiles build that table with heavy GCR."* §B
//! measures it on **six real CMYK destinations** and it is true of **two**
//! (max chromatic ink on the K-only ramp, 2026-08-17):
//!
//! | destination | media-relative | perceptual | saturation |
//! |---|---|---|---|
//! | `ISO Coated v2 300% (ECI)` | `0.705320` | `0.706155` | **`0.035996`** |
//! | `ISO Coated v2 (ECI)` | `0.773954` | `0.776090` | **`0.038759`** |
//! | `Coated FOGRA39` | `0.726101` | `0.730096` | `0.730096` |
//! | `Coated FOGRA27` | `0.756552` | `0.759334` | `0.759334` |
//! | `GWG_GenericCMYK` | `0.791232` | `0.783291` | `0.783291` |
//! | `GWG_ICC_v4_testprofile` (X-Rite) | `0.501787` | `0.531900` | `0.506564` |
//!
//! Three of the six alias `B2A0 ≡ B2A2`, so their "saturation" answer *is*
//! their perceptual answer — Pass G's vendor-specific intent-aliasing finding,
//! reappearing as the reason a shortcut fails. **A suite that measured this
//! subject on ISO Coated v2 alone would have concluded the feature was
//! unnecessary.** `B1` is written to refute the shortcut and goes red exactly
//! when the corpus stops containing a counterexample.
//!
//! ★ Note also what the ECI column costs: saturation reaches **`6.4151 ΔE2000`**
//! from the K-only build at full black (§B's per-profile rows print it). The
//! shortcut is not free even where it works.
//!
//! ## ★★ The third finding: the synthetic corpus cannot see this subject
//!
//! `fixtures/synthetic/v2-cmyk-mft2-lab.icc`'s `B2A0` is built by
//! `lab_to_cmyk_clut`, which emits `[0, 0, 0, k]` at every node. Its K-only
//! ramp comes back **K-only already** — max chromatic ink `0.000000`, whether
//! or not black preservation exists. In this module's own vocabulary that
//! fixture is **`ZERO-SEPARATION`** for this subject: the two candidate
//! answers are the same number, and no tolerance can make it discriminate.
//!
//! That is why every graded row in §A, §B, §C and §E resolves a **licensed,
//! uncommittable** profile through `$ICCCE_PRIVATE_FIXTURES` and **SKIPs in
//! CI, permanently, by design**. A green CI line for those sections says those
//! rows did not run. `TOLERANCES.md` §3.10.8 states the consequence rather
//! than hiding it, and named the fixture that would close it.
//!
//! ### ★★★ CLOSED, 2026-08-17, by §F — and the closure is narrower than it looks
//!
//! `fixtures/synthetic/v2-cmyk-chromatic-neutral.icc` (recipe
//! `v2-cmyk-chromatic-neutral`) is a committed, unlicensed, byte-reproducible
//! CMYK profile whose `B2A0` **separates a neutral into all four inks by
//! construction**. Its two candidate answers are **`0.420 705` apart** in
//! device units, against a floor of `4e-2` declared in advance, and **§F's
//! eight rows all run in CI** — including `k-only-in-implies-k-only-out`,
//! which was deliberately red until 2026-08-18 and is now the pass's only
//! in-CI evidence that the feature works at all.
//!
//! What that *does* buy: the predicate is graded where anybody can see it,
//! against expectations derived from the fixture's own bytes, and a regression
//! guard on 50 **chromatic grays** — points that are not K-only under any
//! definition — made the red attributable to the missing feature rather than
//! to a misread table, and now makes the green attributable to the feature
//! rather than to a fixture that stopped separating.
//!
//! What it does **not** buy, and this must not be blurred: §F measures a
//! **synthetic instrument**, not a press. Its models are affine by
//! construction precisely so that no interpolation envelope enters the
//! arithmetic — which is exactly what a real profile does not give you. §A's
//! `0.705 320`, §B's six-vendor sweep and §C's gray legs remain the only
//! evidence about **real ink**, they remain licensed, and they remain skipped
//! in CI. §F closes the *gradeability* gap, not the *population* gap.
//!
//! ## The oracle is a NON-ICC intent, and every row that uses it says so
//!
//! lcms2 offers rendering intents **10–15**:
//!
//! | number | `lcms2.h` name | what it preserves |
//! |---|---|---|
//! | 10, 11, 12 | `INTENT_PRESERVE_K_ONLY_{PERCEPTUAL, RELATIVE_COLORIMETRIC, SATURATION}` | the K-only *axis* |
//! | 13, 14, 15 | `INTENT_PRESERVE_K_PLANE_{…}` | the whole K *plane* |
//!
//! **These are vendor extensions, and lcms2 says so itself**: `include/lcms2.h`
//! comments the block `// Non-ICC intents`, and ICC.1 **Table 23 permits only
//! 0–3 in a profile header**, so 10–15 cannot even be stored in a profile.
//! [`crate::Intent`] has four variants and no escape hatch precisely so that a
//! difftest cannot wander into 10–15 and describe the result as conformance,
//! and **this module does not weaken that type.** It reaches the non-ICC
//! intents through [`KOnlyOracle`], a separate type that
//!
//! - builds the `transicc` argument vector itself rather than through
//!   [`crate::Request`], so no ICC-intent code path can ever emit a `-t11`;
//! - names its constants (`PRESERVE_K_ONLY_RELATIVE`, not a bare `11`);
//! - carries [`KOnlyOracle::CAVEAT`] as **data**, and [`k_source`] prepends it
//!   to the `source` string of every record built from it, so the disclaimer
//!   cannot be forgotten on a row somebody adds later.
//!
//! Every such row is [`Kind::CrossCheck`] or [`Kind::OracleReproducibility`].
//! **None is ground truth and none can become ground truth**, and that is now
//! a settled negative rather than an assumption.
//!
//! ## ★★★ ICC.1 IS SILENT, and the negative is CLOSED
//!
//! `icc-spec-librarian` searched **ICC.1:2022 and ICC.1:2001-04 whole-document,
//! two engines each**, for `black.?preserv`, `preserve.*black`, `GCR`,
//! `gr[ae]y component` and `K.only`: **zero hits in both**. Corpus file
//! `ICC_Spec/icc/icc__ref__black_preservation.md`; register row **A51**.
//!
//! Two **v2-only** sentences carry the entire ICC story, and v4 deleted both:
//!
//! - **§6.4.45, `ucrbgTag` (`'bfd '`)**, verbatim: *"This tag provides
//!   descriptive information only and is not involved in the processing
//!   model."* — **ICC's only black-generation construct disclaims itself.**
//! - **§6.3.3.1**, verbatim: *"Note: The output values are the control values
//!   and not the \"K\" (black) values."* — a monochrome profile's channel is
//!   **not** the K ink. `"control value"` has zero occurrences in ICC.1:2022.
//!
//! So there is no clause to grade against, no published ΔE (Cholewo 2000 prints
//! visual figures only; lcms2 *computes* the ΔE of its own approximation and
//! discards it — `// Error estimation (for debug only)`, and it is ΔE\*ab, not
//! ΔE2000), and **every expectation in this module is either an implementation
//! cross-check or a property of a fixture's own bytes.** If a normative text is
//! ever found, §3.10 gains ground-truth rows and **none of the numbers below
//! move** — the *kinds* change, which is the whole reason for keeping them
//! separate.
//!
//! ## ★★★ TWO different things are called "black preservation". Say which.
//!
//! | name | rule | layer |
//! |---|---|---|
//! | **K-only preservation** (lcms2's intents 10–12, and Cholewo 2000) | *a pixel that is already K-only stays K-only* | a **CMYK → CMYK** rule — **iccce's** |
//! | *"gray maps onto K alone"* | `c = m = y = 0`, `k = 1 − gray` | a **PDF device-space** rule — **`pdfce`'s** |
//!
//! **They are not the same requirement and they live on opposite sides of the
//! project boundary.** §A, §D and §E measure the first. §C measures the
//! *distance to* the second and grades nothing about it.
//!
//! ★ And even inside the first there are **two definitions under one name**:
//! **lcms2 maps K by equal `L*` on the K ramp**, while **Cholewo (2000) maps it
//! by the `K_MIN`/`K_MAX` ratio.** `E2`'s row names both as rivals. *State
//! which one iccce implements before any cross-check of the K value means
//! anything.*
//!
//! ## What lcms2's K-only preservation actually is, read out of the pin
//!
//! Not folklore — `tools/difftest/vendor/lcms2/src/cmscnvrt.c` at pin
//! `21c582a`, `BlackPreservingKOnlyIntents` and its sampler:
//!
//! ```text
//! if (In[0] == 0 && In[1] == 0 && In[2] == 0) {
//!     Out[0] = Out[1] = Out[2] = 0;
//!     Out[3] = cmsEvalToneCurve16(bp->KTone, In[3]);
//!     return TRUE;
//! }
//! bp->cmyk2cmyk->Eval16Fn(In, Out, bp->cmyk2cmyk->Data);
//! ```
//!
//! Three structural consequences, and §D grades all three:
//!
//! 1. **It is a CLUT resample, not a run-time test.** The sampler is run over
//!    a grid of `_cmsReasonableGridpointsByColorspace(cmsSigCmykData, 0)`
//!    points, which `cmspcs.c` returns as **17** for a 4-channel space. The
//!    K-only answer therefore occupies exactly the `C=M=Y=0` **edge** of a
//!    17-node hypercube, and any input off that edge is a *linear blend* of
//!    the K-only corner with the ordinary colorimetric corners.
//! 2. **The blend is one cell wide and the cell is `1/16 = 0.0625`.** Measured
//!    over 33 samples inside the first cell, the observed answer matches the
//!    linear model to **`1.259×10⁻⁵`**; at `C = 1/16` exactly the K-only answer
//!    and the plain colorimetric answer are **bit-identical**. Preservation is
//!    therefore **not** a "snap near-neutrals to black" rule, and an
//!    implementation that behaved that way would differ measurably. `D1` and
//!    `D2` are the rows that pin this down.
//! 3. **K is RE-MAPPED, not copied.** `_cmsBuildKToneCurve(…, 4096, …)` builds
//!    a 4096-entry curve from the *source's* K-only lightness ramp against the
//!    *destination's*, so `K_out = K_in` only when the two presses' black inks
//!    agree. Across this corpus the difference reaches **`4.889 9×10⁻²`**
//!    (`ISO Coated v2 300%` → `GWG_GenericCMYK`). That number is `D4`'s
//!    **named candidate separation**: *"K is copied through unchanged"* is the
//!    plausible-but-wrong implementation, and it sits `4.89×10⁻²` away in the
//!    row's own units. On the *same-profile* pair it sits `6.1×10⁻⁵` away —
//!    which is why the cross-press pairs, not the same-profile one, are where
//!    that rival is discriminated.
//!
//! ## ★ Why the tolerances here can be TIGHT, and where they cannot
//!
//! Pass 4 measured the CLUT interpolation-method envelope (NA-006) at up to
//! `1.57 ΔE2000` on a CMYK `A2B`, and Pass G's `SWEEP_DEVICE` had to be
//! `4×10⁻³` because of it. This module's graded agreement rows are two orders
//! tighter, and the reason is a **structural** property of the probe sets, not
//! a measurement somebody liked:
//!
//! - **The K-only ramp lies on an EDGE of the 4-D `A2B` CLUT.** With
//!   `C = M = Y = 0` exactly, every interpolation scheme — quadrilinear,
//!   Sakamoto tetrahedral, lcms2's `Eval4Inputs` hybrid — degenerates to the
//!   *same* 1-D linear interpolation along K, because all of them agree on the
//!   edges of the hypercube by construction. The envelope is **identically
//!   zero on this ramp**.
//! - **The `B2A` leg is trilinear on both sides.** lcms2's `_cmsReadOutputLUT`
//!   forces trilinear for a Lab-PCS output LUT (Pass 4b finding 2, and Pass
//!   G's `SWEEP_DEVICE` premise) and `iccce_cmm::clut` is n-linear with
//!   tetrahedral deliberately absent (NA-006). Envelope **identically zero**
//!   again.
//! - **§E's off-neutral regression probe is NODE-ALIGNED.** Its points are
//!   drawn from `j/15`, the `A2B`'s own 16-node grid, so the `A2B` leg does no
//!   interpolation at all. **`E5` is the control that earns this**: the *same*
//!   comparison over arbitrary, non-node points is **`1.750×10⁻³`**, 32×
//!   larger. Without `E5` a reader could not tell a tight bound from a lucky
//!   one.
//!
//! What is left once the envelope is gone is the **16-bit PCS quantum** —
//! lcms2 carries Lab between the two tables in 16 bits, iccce in `f64`. Its
//! device cost is not a constant: it is the destination `B2A`'s own slope, and
//! this module **measures it at run time** ([`pcs_quantum_sensitivity`]) by
//! asking the oracle what one quantum of `L*`, `a*` and `b*` is worth in ink at
//! the PCS points the probe set actually reaches. That makes the bound **a
//! function of the fixture** — Pass G's first tolerance lesson — and it cannot
//! go stale (DL-034).
//!
//! **Where a tolerance could NOT be derived, this module says so and reports
//! rather than grades.** `E2` — whether iccce's preserved K *value* matches
//! lcms2's — is REPORTED for ever, for the same reason §3.7 gives BPC: the K
//! tone curve is a **vendor construction with no normative text behind it**
//! (the A27/A42 posture), and gating it would gate a choice no standard makes.
//! Its number is still printed and its named rival is still computed.
//!
//! ## ★★★ The boundary, now SETTLED — and §C measures it anyway, on purpose
//!
//! The "four different grays" page puts a `DeviceGray`, a `DeviceCMYK`
//! `0/0/0/K`, a `DeviceN [/Black]` and a `Separation /Black` side by side and
//! requires them to match. **`icc-spec-librarian` settled where that is
//! discharged, and it is not here:**
//!
//! - `DeviceGray → DeviceCMYK` is `shall`-level **PDF**: `c = m = y = 0`,
//!   **`k = 1.0 − gray`** — **ISO 32000-1 §10.3.3**, ISO 32000-2 §10.4.2.3.
//! - `Separation /Black` and `DeviceN [/Black]` bind to the K colourant by
//!   **§8.6.6.4**'s `shall`.
//! - **Therefore all four grays agree inside the PDF processor BEFORE any
//!   colour conversion happens.** It is `pdfce`'s job, the same boundary class
//!   as overprint. **iccce owns only CMYK → CMYK and the non-CMYK-native
//!   device.**
//! - ★ **PDF, not ICC, also names the harm**: §8.6.5.7 NOTE 2, both editions —
//!   a 4 → 3 → 4 conversion *"results in a loss of fidelity in the black
//!   component"*. That sentence is the closest thing to a normative statement
//!   of why §A's baseline matters, and it is in the wrong standard for this
//!   project to cite as a requirement on itself.
//!
//! ★ **A premise this pass was commissioned with also failed.** *"GWG 23.0
//! (Four different Grays)"* is **not a GWG requirement id**: GWG 2022 uses
//! `Dxxx`/`Rxxx`, and the four-way equivalence exists as **`D0013 "Black
//! Colour"`** — a *definition consumed by the overprint requirements*, not a
//! rendering requirement. The `n.m` form matches the **Ghent PDF Output Suite
//! patch** numbering. Every occurrence in this module now says *"the Ghent
//! Output Suite's Four-different-Grays patch"*, which is what the artefact is.
//!
//! **§C is kept, and its rows are REPORTED rather than graded**, for two
//! reasons that are worth stating rather than deleting the section:
//!
//! 1. iccce still owns the **non-CMYK-native device** case — a gray *ICC
//!    profile* into a CMYK destination is iccce's leg whenever a consumer hands
//!    it one, and nothing in ISO 32000 covers that path.
//! 2. The distance between the two legs is the number a consumer needs in order
//!    to decide **which** leg to use, and nobody had it.
//!
//! Every row in §C names which leg it asserts in its own id:
//!
//! - **Leg P** — the PDF device rule. Computed here in one line of arithmetic,
//!   with no ICC machinery in it, because it *is* one line of arithmetic.
//! - **Leg I** — iccce's ICC leg: a *gray profile*, through the PCS, into the
//!   CMYK destination. **This is the only leg iccce owns, and §C's graded row
//!   asserts nothing about Leg P's correctness.**
//!
//! The result is the sharpest thing in the module:
//!
//! > On the Ghent corpus's own press-gray profile the two legs land
//! > **`0.716 386` apart in device space** and **`0.7516 ΔE2000` apart in
//! > colour.** They look the same and they are made of completely different
//! > ink.
//!
//! ★ **That colorimetric agreement is a property of WHICH GRAY PROFILE, and
//! the corpus contains a counterexample.** `Schwarze Druckfarbe - ISO Coated
//! v2 (ECI)` is literally *"black printing ink"* — the tone curve of that
//! press's black — so of course its `g` and the destination's `K` describe the
//! same colour. Substituting `fixtures/synthetic/v2-gray-curv-gamma.icc`, an
//! ordinary gamma-2.2 gray, moves the same measurement to
//! **`12.5958 ΔE2000`** (and `0.853 495` in device units). `C5` refutes *"the two legs are interchangeable"*
//! with a number rather than an argument.
//!
//! ## The `refutation row` pattern, defined once
//!
//! Several rows here assert *"the corpus contains a counterexample to shortcut
//! X"*. [`crate::Record::graded`] compares `observed <= tolerance`, which
//! cannot express *"at least one"* directly, so those rows observe **the number
//! of corpus members for which the shortcut HOLDS** and bound it **below the
//! population size**. `B1` bounds 6 members at 5; `C5` bounds 2 at 1. The bound
//! comes from the logic — *a shortcut is only sound if it holds for all of
//! them* — and not from the observation, and the row fails in exactly the
//! circumstance that would make the shortcut defensible. It is a
//! [`Metric::IndicatorCount`], and a count has no instrument error, which is
//! why the bound is an integer.
//!
//! ## Which layer is in the loop
//!
//! Every iccce number here comes from **running the shipped `iccce` binary** as
//! a subprocess ([`crate::Iccce`]), never from an in-process library call. Pass
//! H's lesson — *ask not what a row measures but which layer is in its loop* —
//! applies with force, because a black-preservation feature will need a **CLI
//! surface**, and a feature that exists in the library but is unreachable from
//! the binary must show up as a failure rather than as a green in-process row.
//!
//! Correspondingly, this header used to say: *when the feature lands, `E1` and
//! `E3` must be pointed at whatever surface exposes it.* **Discharged
//! 2026-08-18** — and the discharge was larger than the instruction, in a way
//! worth recording because the same mistake is available to any future pass:
//!
//! ★★★ **`E4`'s own text claimed it was where a leaking preservation path
//! `shows up and nowhere else`. That was FALSE of `E4` as written**, because
//! black preservation is **opt-in and applied never by default**: a row driving
//! the plain surface has no preservation code in its chain to leak. The
//! instruction named `E1` and `E3` — the rows about the *predicate* — and
//! missed the row about the *regression*, which is exactly the row a repointing
//! is least likely to touch and most needs to. `E4`, `E5`, `F4` and `F7` are
//! therefore all driven with `--preserve-black` now, and `E7`/`F8` grade the
//! on/off difference **directly, at exactly zero**, which is a sharper
//! instrument than either cross-check.
//!
//! Generalisation, and it is Pass H's lesson turned on its own remedy: *ask
//! which layer is in the loop of the FIX, not only of the row.*
//!
//! ### ★★★ 2026-08-21 — the leak guards had a FLOOR, and it was an accident
//!
//! `E7`/`F8` were then proved to fire by injection (`NC-267`) — and the sweep
//! that did it found something the single red run could not: **at an injected
//! widening of the qualifying test to `t = 0.04` the ENTIRE difftest suite was
//! green with the defect compiled in** (`DL-064`). A leak guard can only see a
//! widening that reaches one of its own probes, and the smallest chromatic ink
//! these two sets carried was `1.106777e-1` and `5.000000e-2` — while the
//! rival named in their own justification, and in `crates/iccce-cmm`'s module
//! doc, was **`1e-9` of cyan**. Seven-plus orders, both numbers on the page.
//!
//! The floor turned out to be **a free parameter and not a property of the
//! machine**: the guard's response *rises* to a constant as ink falls
//! (`3.17e-1` to `3.84e-1` device units, flat from about `1e-6` down), because
//! the unpreserved answer tends to the four-ink separation of a K-only input.
//! [`low_ink_decade_probes`] therefore walks 14 decades to `1e-12` and is
//! folded into **the same `leak` number** on both rows, taking both floors to
//! `1.000000e-12`. **No tolerance moved**; both rows are still exactly `0` and
//! still observe `0.000000e0`. Each now prints its own floor, and
//! [`probe_floor`] computes it, so `arbitrary_off_neutral`'s seed accident can
//! no longer move `E7`'s sensitivity silently. Instrument:
//! `bin/passk_leak_floor`. Derivation and coverage: `TOLERANCES.md`
//! §3.10.12.8.
//!
//! ## What this pass cannot measure, stated here and not only in the report
//!
//! - **Whether lcms2's equal-`L*` rule or Cholewo's `K_MIN`/`K_MAX` ratio is
//!   the RIGHT definition.** ICC.1 states neither (register entry A51, a closed
//!   negative), so nothing here can settle it and rule 7's remedy does not
//!   apply. iccce implements the first and **names it in a mandatory CLI
//!   argument**; `E9` grades that it implements the one it names, which is a
//!   different and much weaker claim, stated as such on the row.
//! - **The K value itself, on a same-press pair.** `E2` is REPORTED for ever
//!   and the reason is now MEASURED rather than argued: on `ISO Coated v2 300%`
//!   → itself the two candidate definitions coincide to `6.1e-5`, which is
//!   exactly the observation, so the row is **BLIND** — a bound iccce passed,
//!   "copy K through" would pass too. `E9` exists because a cross-press pair is
//!   not blind.
//! - **The width of the near-neutral transition.** `E3`/`F6` report iccce's
//!   zero against the oracle's `1/16` and grade neither. That gap is now a
//!   real, measured behavioural difference between two implementations of an
//!   unspecified policy, not an artefact of a missing feature — and it is
//!   stated rather than tuned toward.
//! - **Anything about the PDF leg's correctness.** ISO 32000-1 §10.3.3 settles
//!   that it is `pdfce`'s; §C measures the distance to it and grades nothing.
//! - **Ink cost, moiré, registration or text sharpness** — the reasons the
//!   requirement exists. Nothing in a CMM measures them; the closest proxies
//!   here are chromatic-ink coverage and TAC, and they are proxies.
//! - **Leg P's correctness.** §C measures it as arithmetic and compares it; it
//!   does not grade it, because it is `pdfce`'s.
//! - **Anything about REAL INK, in CI.** §A–§E need the licensed corpus and
//!   skip there permanently. §F grades the same predicate in CI on a committed
//!   synthetic instrument, which is a statement about the *predicate*, never
//!   about a press — see "the third finding" and its closure.

use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use iccce_color::{Lab, delta_e_2000};

use crate::{
    Bpc, DiffError, Iccce, Intent, Kind, Metric, Oracle, Precalc, Record, Request, SepUnits,
    Separation, Space, Tolerance,
};

// ===========================================================================
// Where the fixtures live
// ===========================================================================

/// The private-fixture root, resolved exactly as Pass G and Pass H resolve it:
/// **environment variable, then default path, then skip.** No bundled copy and
/// no third fallback — a corpus that cannot be redistributed must be *absent*
/// on a machine that has not been given it, and the suite must say so out loud
/// rather than quietly grading nothing.
#[must_use]
pub fn corpus_dir() -> PathBuf {
    std::env::var_os("ICCCE_PRIVATE_FIXTURES")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(r"D:\Dev\iccce-private-fixtures"))
        .join("ghent-v50")
}

/// The committed synthetic corpus, resolved relative to this crate's manifest.
#[must_use]
pub fn synthetic_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/synthetic")
}

/// Ghent corpus members used here, named by the first 16 hex digits of their
/// SHA-256 — a *pointer* to a licensed artifact, never any part of its content.
/// The `desc` strings in the comments are the profiles' own descriptions,
/// reproduced for identification only; they are not colour values.
mod file {
    /// ★ **The subject of §A.** `ISO Coated v2 300% (ECI)`, v2.4 `prtr` CMYK,
    /// `mft2` `A2B*` at grid **16**, `mft2` `B2A*` at grid **33**. This is the
    /// profile the Ghent suite embeds as the `DestOutputProfile` of its
    /// ICC-CMS patches.
    ///
    /// ★ A second corpus file (`cb5df02f9b9cea7e.icc`) carries the same `desc`
    /// and **byte-identical payloads for every tag except `desc` and `hd10`**,
    /// differing only in the header version word (v2.0 vs v2.4). It is a
    /// *version* isolator, **not** a separation isolator, and Pass K does not
    /// use it.
    pub const ISOCOATED300: &str = "c6b4b62f07262437.icc";
    /// `ISO Coated v2 (ECI)` — the 350 % TAC sibling, v2.4.
    ///
    /// ★ Its `A2B1` is **byte-identical** to [`ISOCOATED300`]'s (same press,
    /// same ink — only the separation differs) while its `B2A0`/`B2A1` differ.
    /// That makes the pair a clean **separation-direction isolator**, and it is
    /// why converting `ISO Coated v2 (ECI) → ISO Coated v2 300% (ECI)` at
    /// media-relative produces output *bit-identical* to `300% → 300%`: two
    /// sources putting identical PCS values into one destination table. A
    /// property of the files, verified by tag hash — not a bug.
    pub const ISOCOATED350: &str = "128dc02f7246cc38.icc";
    /// `Coated FOGRA39 (ISO 12647-2:2004)`, v2.1 `prtr` CMYK.
    pub const FOGRA39: &str = "da2b9b593e27cba2.icc";
    /// `Coated FOGRA27 (ISO 12647-2:2004)`, v2.1 `prtr` CMYK.
    pub const FOGRA27: &str = "fb710c05e3fb5a96.icc";
    /// `GWG_GenericCMYK`, v2.0 `prtr` CMYK.
    pub const GENERIC_CMYK: &str = "5bad92a6f018e726.icc";
    /// `GWG_ICC_v4_testprofile.icc` — X-Rite, ICC v4.2, `mAB `/`mBA `.
    pub const XRITE_V4: &str = "b5988983b6b3b7d4.icc";
    /// ★ `Schwarze Druckfarbe - ISO Coated v2 (ECI)` — v2.1 `prtr` **GRAY**,
    /// four tags, 256-entry `kTRC`. Literally *"black printing ink"*: the tone
    /// curve of the black ink of §A's destination press. §C's most favourable
    /// possible gray source, which is exactly why §C also runs a second one.
    pub const PRESS_GRAY: &str = "5dae7984654a2c9f.icc";
}

/// The committed synthetic gray used as §C's **unfavourable** source: an
/// ordinary gamma-2.2 `GRAY` profile with no relationship to any press.
const SYNTHETIC_GRAY: &str = "v2-gray-curv-gamma.icc";
/// The committed synthetic CMYK profile, used by `E6` to demonstrate — with a
/// number — that the synthetic corpus is `ZERO-SEPARATION` for this subject.
const SYNTHETIC_CMYK: &str = "v2-cmyk-mft2-lab.icc";

// ===========================================================================
// The non-ICC oracle — lcms2 intents 10..15, quarantined behind a type
// ===========================================================================

/// A `transicc` invocation at one of lcms2's **black-preserving intents
/// 10–15**, which are **not ICC rendering intents**.
///
/// ## Why this is a separate type instead of a variant on [`crate::Intent`]
///
/// `lib.rs`'s crate header states that this harness *"cannot express a non-ICC
/// rendering intent … a difftest that wandered into them and reported
/// 'conforms' would be reporting on something the specification does not
/// define."* That contract is worth keeping, and adding a fifth variant to
/// [`crate::Intent`] would silently retire it for every pass in the crate.
///
/// So the non-ICC intents live here, in a type that
///
/// * builds its own argument vector — [`crate::Request::to_args`] can still
///   only emit `-t0..-t3`;
/// * cannot be used without naming lcms2's constant at the call site;
/// * carries [`KOnlyOracle::CAVEAT`] as data, which [`k_source`] prepends to
///   every record's `source`.
pub struct KOnlyOracle<'a> {
    exe: &'a Path,
}

impl<'a> KOnlyOracle<'a> {
    /// `INTENT_PRESERVE_K_ONLY_RELATIVE_COLORIMETRIC` — `lcms2.h`, pin
    /// `21c582a`. Preserves the K-only *axis*; see the module header for the
    /// sampler that implements it.
    pub const PRESERVE_K_ONLY_RELATIVE: u8 = 11;

    /// The sentence that goes on the front of every record's `source`.
    pub const CAVEAT: &'static str = "ORACLE IS A NON-ICC INTENT: lcms2 rendering intent 11 \
         (INTENT_PRESERVE_K_ONLY_RELATIVE_COLORIMETRIC), a VENDOR EXTENSION outside the ICC \
         intent numbering. ICC.1 numbers four rendering intents, 0..3, and defines no \
         black-preserving intent. Every row from this oracle is an IMPLEMENTATION CROSS-CHECK \
         and can never be ground truth";

    #[must_use]
    pub fn new(oracle: &'a Oracle) -> KOnlyOracle<'a> {
        KOnlyOracle { exe: oracle.path() }
    }

    /// Convert CMYK rows (normalised `0..1`) `src → dst` at a non-ICC intent,
    /// returning CMYK rows in `0..1`.
    ///
    /// `transicc` speaks **0..100** for ink spaces on both sides; the scaling
    /// is applied here so that no caller can forget it — the `/100`-vs-`/255`
    /// hazard recorded on [`crate::Iccce`].
    ///
    /// # Errors
    /// Any spawn, pipe, exit-status or parse failure, as [`crate::DiffError`].
    pub fn convert_cmyk(
        &self,
        src: &Path,
        dst: &Path,
        intent: u8,
        rows: &[[f64; 4]],
    ) -> Result<Vec<[f64; 4]>, DiffError> {
        let args = vec![
            format!("-i{}", src.display()),
            format!("-o{}", dst.display()),
            format!("-t{intent}"),
            "-c0".to_string(),
            "-n".to_string(),
        ];
        let mut child = Command::new(self.exe)
            .args(&args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| DiffError::Spawn(self.exe.to_path_buf(), e))?;
        let mut buf = String::with_capacity(rows.len() * 48);
        for r in rows {
            for v in r {
                buf.push_str(&format!("{}\n", v * 100.0));
            }
        }
        let mut stdin = child
            .stdin
            .take()
            .ok_or_else(|| DiffError::Internal("child stdin was not piped".into()))?;
        let writer = std::thread::spawn(move || -> std::io::Result<()> {
            stdin.write_all(buf.as_bytes())?;
            stdin.flush()
        });
        let out = child
            .wait_with_output()
            .map_err(|e| DiffError::Spawn(self.exe.to_path_buf(), e))?;
        match writer.join() {
            Ok(Ok(())) => {}
            Ok(Err(e)) => return Err(DiffError::Pipe(e)),
            Err(_) => {
                return Err(DiffError::Internal("stdin writer thread panicked".into()));
            }
        }
        let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
        let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
        if !out.status.success() {
            return Err(DiffError::NonZeroExit {
                args,
                code: out.status.code(),
                stdout,
                stderr,
            });
        }
        let parsed = crate::parse_rows(&stdout, 4).ok_or_else(|| DiffError::Unparsable {
            args: args.clone(),
            stdout: stdout.clone(),
            stderr: stderr.clone(),
        })?;
        if parsed.len() != rows.len() {
            return Err(DiffError::Arity {
                expected: rows.len(),
                got: parsed.len(),
                stdout,
            });
        }
        Ok(parsed
            .into_iter()
            .map(|r| [r[0] / 100.0, r[1] / 100.0, r[2] / 100.0, r[3] / 100.0])
            .collect())
    }
}

/// Build a `source` string for a record whose expectation came from a non-ICC
/// lcms2 intent. **Always** used for such rows; see [`KOnlyOracle::CAVEAT`].
/// Build the text of a candidate ALTERNATIVE that is defined by the non-ICC
/// oracle, so that a separation string cannot quietly present a vendor
/// extension as though it were an ICC behaviour.
fn k_only_alt(what: &str) -> String {
    format!(
        "{what} - as realised by lcms2 rendering intent 11, a VENDOR EXTENSION outside the ICC \
         intent numbering (ICC.1 numbers four intents, 0..3)"
    )
}

fn k_source(what: &str) -> String {
    format!("{} — {what}", KOnlyOracle::CAVEAT)
}

// ===========================================================================
// Probe sets — deterministic, and each one's SHAPE is part of its tolerance
// ===========================================================================

/// The K-only ramp: `(0, 0, 0, j/40)` for `j = 0..=40`.
///
/// **`C = M = Y = 0` exactly** is the load-bearing property, twice over: it is
/// the input condition lcms2's sampler tests, and it is what puts the ramp on
/// an *edge* of the 4-D `A2B` CLUT, where every interpolation scheme agrees and
/// the method envelope is identically zero. A ramp built with `1×10⁻⁹` of cyan
/// "to avoid a boundary" would silently destroy both properties.
///
/// 41 points rather than one: the contamination is **not uniform** — it peaks
/// in chromatic ink at `K = 1.0` and in *black loss* at `K = 0.60` — and the
/// shape is the finding. `src/bin/passk_probe.rs` prints the whole ramp; the
/// records below reduce it, which is what a [`Record`] is for.
#[must_use]
pub fn k_ramp() -> Vec<[f64; 4]> {
    (0..=40)
        .map(|j| [0.0, 0.0, 0.0, f64::from(j) / 40.0])
        .collect()
}

/// The gray ramp for §C: `g = j/20`, `j = 0..=20`, in a 1-channel source.
#[must_use]
pub fn gray_ramp() -> Vec<f64> {
    (0..=20).map(|j| f64::from(j) / 20.0).collect()
}

/// **§D's cell probe.** `C = t/32 × 1/16`, `M = Y = 0`, `K = 0.5`, for
/// `t = 0..=32`: 33 samples spanning exactly **one cell** of lcms2's 17-node
/// black-preserving CLUT, endpoints included.
///
/// `1/16` is not a fitted number: `_cmsReasonableGridpointsByColorspace`
/// returns **17** for a 4-channel space, so the cell width is `1/(17−1)`. If
/// that constant ever changes in lcms2, `D2` — which requires the endpoints to
/// be **bit-identical** to the plain colorimetric answer — goes red, which is
/// the intended alarm.
#[must_use]
pub fn cell_ramp() -> Vec<[f64; 4]> {
    (0..=32)
        .map(|t| [f64::from(t) / 32.0 * CELL, 0.0, 0.0, 0.5])
        .collect()
}

/// One cell of lcms2's black-preserving CMYK CLUT: `1/(17−1)`.
pub const CELL: f64 = 1.0 / 16.0;

/// **§E's regression probe: 96 deterministic off-neutral points whose device
/// coordinates are `A2B` CLUT NODES.**
///
/// `ISO Coated v2 300% (ECI)`'s `A2B*` tags declare `clut_points = 16`, so the
/// nodes sit at `j/15`. Drawing every coordinate from that set means the `A2B`
/// leg performs **no interpolation at all**, which is what allows §E's bound to
/// be two orders tighter than Pass G's `SWEEP_DEVICE`. `E5` measures the same
/// comparison over *arbitrary* points and is the control that earns it.
///
/// Two filters, each with a reason:
/// * `max(C, M, Y) > 0` — a point with no chromatic ink is a K-ramp point and
///   belongs to §A. §E's whole job is the **off-neutral** half.
/// * `C + M + Y + K ≤ 3.0` — this destination is a 300 % profile; points above
///   its own TAC exercise a region no separation of it can produce, and a
///   regression guard should watch the region documents actually occupy.
///
/// The generator is a fixed-seed LCG written out here rather than pulled from a
/// crate, because a probe set that changes when a dependency changes is not a
/// regression guard.
#[must_use]
pub fn node_aligned_off_neutral() -> Vec<[f64; 4]> {
    let mut state: u64 = 0x2026_0817_0000_000B;
    let mut next = || {
        state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        f64::from(u32::try_from((state >> 33) % 16).unwrap_or(0)) / 15.0
    };
    let mut out = Vec::with_capacity(96);
    while out.len() < 96 {
        let p = [next(), next(), next(), next()];
        if p[0].max(p[1]).max(p[2]) > 0.0 && p.iter().sum::<f64>() <= 3.0 {
            out.push(p);
        }
    }
    out
}

/// **`E5`'s control: 96 deterministic points that are NOT node-aligned.**
///
/// Same generator, same filters, coordinates drawn from a continuum instead of
/// from `j/15`. Its only job is to make the interpolation-method envelope
/// visible as a number so that `E4`'s tightness reads as *derived* rather than
/// as *lucky*.
#[must_use]
pub fn arbitrary_off_neutral() -> Vec<[f64; 4]> {
    let mut state: u64 = 0x2026_0817_0000_00E5;
    let mut next = || {
        state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        #[expect(
            clippy::cast_precision_loss,
            reason = "a 21-bit integer mantissa is exact in f64; this is a probe coordinate"
        )]
        let v = ((state >> 43) as f64) / 2_097_152.0;
        v * 0.8
    };
    let mut out = Vec::with_capacity(96);
    while out.len() < 96 {
        let p = [next(), next(), next(), next()];
        if p[0].max(p[1]).max(p[2]) > 0.0 && p.iter().sum::<f64>() <= 3.0 {
            out.push(p);
        }
    }
    out
}

// ---------------------------------------------------------------------------
// The LOW-INK decade probe set — shared by E7 (§E) and F8 (§F)
// ---------------------------------------------------------------------------

/// The chromatic-ink levels the low-ink probe set walks, largest first.
///
/// ★★★ **Why the list ends at `1e-12` and not lower, measured rather than
/// chosen.** The named rival of `E7`/`F8` — the change a future contributor is
/// most likely to make — is *widening the qualifying test from exact zero to a
/// tolerance*, and the magnitude that rival has always been written with in
/// this project is **`1e-9` of cyan**. A probe set whose smallest ink is
/// `1e-12` sits **three decades below** it, so a widening anywhere at or above
/// the rival's own magnitude puts probes on the qualifying side of the test.
///
/// Extending the list further buys nothing, and that is a **measurement**, not
/// an argument: the guard's response (`bin/passk_leak_floor`, and the
/// `RESPONSE` block it prints) is *constant* from about `1e-6` downwards —
/// `3.589900e-1` on `v2-cmyk-chromatic-neutral`, `3.170750e-1` on
/// `v2-cmyk-warm-black` — all the way to the smallest positive subnormal
/// `4.940656e-324`. Below `1e-6` the answer no longer changes, so a decade
/// below `1e-12` would add a probe that measures exactly what `1e-12` already
/// measures.
///
/// ★★ **And there is a hard floor below which a probe stops being a probe.**
/// The harness writes each coordinate with `format!("{v}")` and the CLI parses
/// it with `str::parse::<f64>`, so any decimal that **underflows to `0.0`**
/// arrives at the shipped qualifying test as a *genuine* K-only input.
/// The preservation branch then fires **correctly**,
/// `on != off`, and the guard goes **RED against a correct engine**. Measured:
/// at `c = 4.940656e-324` (the smallest positive subnormal) the baseline leak
/// is `0.000000e0`; at `c = 1e-324`, which parses to `0.0`, it is
/// `3.589900e-1`. **A leak guard's probe floor must stay above the underflow
/// boundary, and `1e-12` is 312 decades clear of it.**
///
/// ★ **What the sub-`1.5e-5` decades are, and are not, for.** No device value
/// encoded in a 16-bit ICC table can be smaller than one quantum
/// (`1/65535 = 1.525902e-5`) and non-zero, so the last nine entries of this
/// list are **not** reachable from a document. They are reachable from a
/// *source edit*, which is the only thing this row exists to catch: the rival
/// is a change to a predicate written in floating point, and it is graded in
/// the units the predicate is written in.
pub const LOW_INK_DECADES: [f64; 14] = [
    5e-2, 1e-2, 5e-3, 1e-3, 5e-4, 1e-4, 1e-5, 1e-6, 1e-7, 1e-8, 1e-9, 1e-10, 1e-11, 1e-12,
];

/// **★★★ The low-ink probe set: 70 chromatic grays walking 14 decades of ink.**
///
/// `E7` and `F8` are the two rows that detect a widened qualifying test, and
/// **a leak guard can only see a widening that reaches one of its own probes**.
/// Before this set existed their floors were `1.106777e-1` and `5.000000e-2`
/// — seven-plus orders of magnitude *above* the rival their own justification
/// named — and at an injected widening of `t = 0.04` the entire difftest suite
/// was green with the defect compiled in.
///
/// ## The construction, and why the floor is STRUCTURAL
///
/// Each level `c` emits five probes `[c, (6/7)·c, r·c, j/8]` for `j = 0..5`,
/// with `r = (50 + 45·(6/7))/90 ≈ 9.841270e-1`. Both ratios are **strictly
/// below 1**, and a positive `f64` multiplied by a ratio below 1 and rounded
/// to nearest can never exceed it — so `max(C, M, Y) = c` on **every** probe
/// and the set's floor is `LOW_INK_DECADES`' last entry **by construction**.
/// It moves only if someone edits that array, and [`probe_floor`] prints it on
/// the row every run so an edit shows up as a changed number.
///
/// ★ The ratios are `chromatic_gray_probes`' own, deliberately: the two sets
/// then differ in exactly one variable — the ink level — so the low-ink half
/// of a leak measurement is comparable with the `5e-2`-and-up half rather than
/// being a second, differently-shaped experiment.
///
/// ★★ **The `K` values are `j/8`, not a ramp to 1.0.** The preserved answer is
/// `[0, 0, 0, map_k(K)]` whatever the chromatic input, so the *response* a
/// widening produces is `|map_k(K) − plain(c, K)|` and it is largest where the
/// two answers are furthest apart. Five values spanning `0 … 0.5` is what
/// `chromatic_gray_probes` uses and it is enough: the row's tolerance is
/// **exactly zero**, so one non-zero channel on one probe fails it.
#[must_use]
pub fn low_ink_decade_probes() -> Vec<[f64; 4]> {
    const M_OVER_C: f64 = 6.0 / 7.0;
    let y_over_c = (50.0 + 45.0 * M_OVER_C) / 90.0;
    let mut out = Vec::with_capacity(LOW_INK_DECADES.len() * 5);
    for &c in &LOW_INK_DECADES {
        for j in 0..5 {
            out.push([c, M_OVER_C * c, y_over_c * c, f64::from(j) * 0.125]);
        }
    }
    out
}

/// **A probe set's DETECTION FLOOR: `min over probes of max(C, M, Y)`.**
///
/// ★★★ This is the smallest widening of the qualifying test that the set can
/// see, and **the reason it is a function rather than a constant** is `E7`.
/// `arbitrary_off_neutral` draws from a fixed-seed LCG on `[0, 0.8)`;
/// construction bounds its floor only at `0.8/2²¹ ≈ 3.8e-7`, i.e. at nothing,
/// so the floor it actually has (`1.106777e-1`) is **an accident of the seed**
/// and **re-seeding would move `E7`'s sensitivity without changing one line of
/// intent, one comment or one tolerance**. Computing it here and printing it
/// on the row means a re-seed shows up as a changed number instead of as
/// silent drift.
///
/// Returns `f64::INFINITY` for an empty set — a set with no probes has no
/// floor, and reporting `0` would read as "sees everything".
#[must_use]
pub fn probe_floor(probes: &[[f64; 4]]) -> f64 {
    probes
        .iter()
        .map(|p| p[0].max(p[1]).max(p[2]))
        .fold(f64::INFINITY, f64::min)
}

// ===========================================================================
// Tolerances
// ===========================================================================

/// **REPORTED, not graded.** A tolerance of `∞` is a decision and this module
/// makes it in three circumstances, each named on the row that carries it:
///
/// 1. **The baseline rows (§A1–§A3, §B2–§B7).** There is no requirement today
///    that a baseline could violate; inventing one so the section had a gate
///    would be inventing the very thing the pass exists to derive.
/// 2. **`E2`, the preserved K *value*.** lcms2's `_cmsBuildKToneCurve` is a
///    **vendor construction with no normative text behind it** — the same
///    posture §3.7 takes for BPC (corpus A27/A42). Gating iccce against it
///    would gate a choice no standard makes.
/// 3. **Leg-P comparisons in §C.** The PDF device rule is `pdfce`'s; this
///    module measures the distance to it and does not grade it.
///
/// A reported row has **no discriminating power** and the report says so on the
/// line (`SepPower::Ungraded`).
pub const REPORTED: Tolerance = Tolerance::new(
    f64::INFINITY,
    "REPORTED, NOT GRADED. Either the row records a BASELINE, which no requirement yet \
     constrains; or its expectation is lcms2's black-ink tone curve, a vendor construction with \
     NO normative text behind it (the A27/A42 posture §3.7 takes for BPC); or it measures a \
     distance to ISO 32000-1 §10.3.3's device-space rule, which belongs to the PDF consumer and \
     not to this project. Grading any of the three would gate a choice no standard makes",
);

/// **§2's perceptibility anchor, used here for its INVERSE.**
///
/// Everywhere else in this document family `1.0 ΔE2000` bounds an error that
/// must be *invisible*. `A4` uses it to establish that a difference which
/// **is** a defect is nevertheless invisible — that the contaminated build and
/// the K-only build are within the threshold of perceptible difference for
/// adjacent patches, and therefore that **no ΔE-based test can be the
/// instrument for this subject**. The row passing is the finding.
pub const DE_PERCEPTIBLE: Tolerance = Tolerance::new(
    1.0,
    "the accepted threshold of perceptible difference for adjacent patches (TOLERANCES.md §2). \
     ★ Used here for its INVERSE: this row asserts that the CONTAMINATED build and the K-only \
     build are INDISTINGUISHABLE, which is what proves a dE-based test cannot detect the defect \
     black preservation exists to fix. The defect's own magnitude is in DEVICE units and is \
     stated in this row's detail beside the dE",
);

/// **Exactly zero, for a predicate about the encoded value zero.**
///
/// "K-only" is the statement that three channels carry the encoded value zero.
/// There is no instrument error in *is this channel zero*: lcms2's own K-only
/// intent returns `0.000000` in all three chromatic channels at every point of
/// the ramp, so a non-zero bound would be an allowance for ink the requirement
/// forbids rather than for noise.
pub const EXACT_ZERO: Tolerance = Tolerance::new(
    0.0,
    "K-ONLY MEANS K-ONLY. The predicate is 'these three channels carry the encoded value zero', \
     and a predicate about zero has no instrument error: lcms2's own K-only intent returns \
     0.000000 in all three chromatic channels at every point of this ramp. Any bound above zero \
     would be an allowance for ink the requirement forbids",
);

/// **The two printed floors, and nothing else** — `E8`'s bound.
///
/// `E8`'s expectation is not an implementation's output and not a
/// measurement: it is **algebra**. When the source and destination models are
/// the same model, the destination `K` whose `K`-only patch has the same `L*`
/// as the source's at `K_in` **is `K_in`** — the equal-lightness construction
/// is the identity on a same-profile pair, exactly, for any strictly
/// monotonic ramp. Nothing about a press, an encoding or an interpolation
/// scheme enters that statement.
///
/// So the only instrument between the claim and the observation is the
/// printing: `iccce transform` writes six decimals in `0..1` (`1e-6`), and the
/// probe's own `K` values are `j/40`, every one of which is exactly
/// representable in six decimals. One printed unit is therefore the whole
/// budget.
///
/// ★ **The premise is a property of the fixture and is stated rather than
/// assumed.** If this destination's `L*(K)` ramp ever contained a flat stretch
/// — ink saturating, which is a real thing a press profile does — the
/// inversion would be ill-posed there, `crates/iccce-cmm`'s inverter takes the
/// **lower** `K` by a documented choice, and the identity would fail *for a
/// correct implementation*. That would be a red row about the fixture, not
/// about the code, and a reader who sees this row go red must check the ramp
/// before touching the inverter.
pub const PRINT_FLOOR: Tolerance = Tolerance::new(
    1e-6,
    "ONE printed unit of `iccce transform`'s six decimals in 0..1. The expectation is ALGEBRA, not a measurement: on a same-profile pair the equal-lightness construction is the identity K_out = K_in for any strictly monotonic L*(K) ramp, so no press, encoding or interpolation term exists to allow for. The probe's own K values are j/40 and are exactly representable in six decimals. ★ The premise is the fixture's strict monotonicity; a flat stretch would make the inversion ill-posed and this row would go red about the FIXTURE",
);

/// **§D1: two 16-bit quanta**, the encoding's own precision.
///
/// `D1` compares lcms2's observed answer inside one CLUT cell against the
/// linear blend of that cell's two endpoints. Both endpoints are **16-bit CLUT
/// outputs**, so each carries up to half a quantum of encoding error, and the
/// blend carries the CLUT's own 16-bit output quantisation once more. Two
/// quanta of `2⁻¹⁶` bounds the sum. Nothing perceptual enters this number and
/// §2's anchor must not be cited in its support.
pub const CLUT_CELL_MODEL: Tolerance = Tolerance::new(
    2.0 / 65_536.0,
    "TWO 16-bit quanta. The model is a linear blend of the cell's two ENDPOINTS, both of which \
     are 16-bit CLUT outputs (half a quantum of encoding error each), and the blend itself is \
     requantised to 16 bits once more. 2 x 2^-16 = 3.0518e-5 bounds the sum. ARITHMETIC, from \
     the encoding's own precision; NOT perceptual, and §2's 1.0 dE anchor is irrelevant to it",
);

/// **§B1's refutation bound.** Six real CMYK destinations; the shortcut *"use
/// the saturation intent instead of building black preservation"* is sound only
/// if it holds for **all** of them, so the row fails at 6 and passes below it.
/// Derived from the logic of the claim, not from the observation (which is 2).
pub const SHORTCUT_SATURATION: Tolerance = Tolerance::new(
    5.0,
    "a REFUTATION row over a population of 6 real CMYK destinations. The shortcut it refutes - \
     'use the saturation intent instead of building black preservation' - is sound only if EVERY \
     destination's saturation B2A happens to be K-only on the neutral axis, so the bound is one \
     below the population size. It is derived from the logic of the claim and not from the \
     observation. The row fails in exactly the circumstance that would make the shortcut \
     defensible, and it also fails if the corpus ever loses its counterexamples",
);

/// **§C5's refutation bound.** Two gray sources; the shortcut *"the ICC leg and
/// ISO 32000-1 §10.3.3's device rule are interchangeable"* is sound only if it
/// holds for both.
pub const SHORTCUT_GRAY_LEG: Tolerance = Tolerance::new(
    1.0,
    "a REFUTATION row over a population of 2 gray source profiles. The shortcut it refutes - \
     'iccce's ICC leg and ISO 32000-1 §10.3.3's device-space rule are interchangeable' - is \
     sound only if EVERY gray source lands within the perceptibility anchor of the device rule, \
     so the bound is one below the population size. Derived from the logic, not the observation. \
     ★ The favourable member (the press's OWN black-ink gray profile) agrees; the unfavourable \
     one does not, and a suite that carried only the favourable member would have reported the \
     shortcut as sound",
);

/// **§A5/§E4's agreement bound — computed at RUN TIME from the fixture.**
///
/// The derivation is in the module header and has two halves:
///
/// * **the interpolation-method envelope is identically zero** on both probe
///   sets, by construction (edge of the `A2B` hypercube for the K ramp; `A2B`
///   nodes for §E; trilinear on both sides in the `B2A` leg);
/// * what remains is the **16-bit PCS quantum**, whose device cost is the
///   destination `B2A`'s own slope. `sensitivity` is that slope, measured with
///   the oracle at the PCS points this probe set actually reaches.
///
/// Plus `2×10⁻⁶` for the two print floors: `iccce transform` prints six
/// decimals in `0..1` and `transicc` prints four decimals in `0..100`, both
/// `1×10⁻⁶` in normalised units.
///
/// **This is a function and not a constant on purpose** (Pass G tolerance
/// lesson 1): the bound is a property of *which destination table is loaded*,
/// it prints its own premise on the line, and it cannot go stale.
#[must_use]
pub fn pcs_quantum_tolerance(sensitivity: f64) -> Tolerance {
    Tolerance {
        value: sensitivity + 2e-6,
        why: "the destination B2A's own device response to ONE 16-bit PCS quantum, MEASURED AT \
              RUN TIME on the PCS points this probe set reaches, plus 2e-6 for the two print \
              floors (iccce 6 decimals in 0..1; transicc 4 decimals in 0..100). The \
              interpolation-method envelope - Pass 4's NA-006, worth up to 1.57 dE2000 on a CMYK \
              A2B and the reason Pass G's SWEEP_DEVICE had to be 4e-3 - is IDENTICALLY ZERO on \
              both of this pass's probe sets: the K ramp lies on an EDGE of the 4-D A2B \
              hypercube where every scheme degenerates to the same 1-D interpolation, §E's \
              points are A2B CLUT NODES, and lcms2 forces trilinear for a Lab-PCS output LUT \
              while iccce is n-linear, so the B2A leg's envelope is zero on both sides. Row E5 \
              measures the same comparison OFF the nodes and is the control that earns this. \
              A FUNCTION of the fixture, not a constant",
    }
}

// ===========================================================================
// Small measurement helpers
// ===========================================================================

/// Max chromatic ink over a set of CMYK rows — the module's primary observable.
#[must_use]
pub fn max_chromatic(rows: &[[f64; 4]]) -> f64 {
    rows.iter()
        .map(|r| r[0].max(r[1]).max(r[2]))
        .fold(0.0_f64, f64::max)
}

/// Max total area coverage over a set of CMYK rows.
#[must_use]
pub fn max_tac(rows: &[[f64; 4]]) -> f64 {
    rows.iter()
        .map(|r| r.iter().sum::<f64>())
        .fold(0.0_f64, f64::max)
}

/// Max `|Δ|` over every component of every row of two equal-shaped grids.
#[must_use]
pub fn max_dev(a: &[[f64; 4]], b: &[[f64; 4]]) -> f64 {
    a.iter()
        .zip(b)
        .map(|(x, y)| {
            x.iter()
                .zip(y)
                .map(|(p, q)| (p - q).abs())
                .fold(0.0_f64, f64::max)
        })
        .fold(0.0_f64, f64::max)
}

/// Reshape `Vec<Vec<f64>>` (what the harness's subprocess helpers return) into
/// fixed-width CMYK rows. A row of the wrong width is a real disagreement about
/// the shape of the answer, so it is an error rather than a silent reshape.
/// Reshape the harness's `Vec<Vec<f64>>` into fixed-width CMYK rows.
///
/// # Errors
/// [`Unavailable::Error`] when a row is not four components wide — a real
/// disagreement about the shape of the answer, never a silent reshape.
pub fn as_cmyk(rows: Vec<Vec<f64>>) -> Result<Vec<[f64; 4]>, Unavailable> {
    rows.into_iter()
        .map(|r| {
            <[f64; 4]>::try_from(r.as_slice())
                .map_err(|_| Unavailable::Error(format!("expected 4 components, got {}", r.len())))
        })
        .collect()
}

/// Render a set of **device** CMYK values into Lab through a profile's `A2B1`,
/// using the oracle.
///
/// ## Why the oracle and not iccce
///
/// Both sides of §A4's ΔE are iccce *outputs*; the thing being asked is *how
/// far apart are these two ink builds in colour*, which needs a **model of the
/// press**, not a second opinion about the conversion. Using one fixed model —
/// the profile's own `A2B1`, evaluated by lcms2 — for both sides means the ΔE
/// is a property of the two device values and of the profile, and the model's
/// own error cancels. Using iccce for it would put the code under test on both
/// the subject and the ruler side.
/// # Errors
/// [`Unavailable::Error`] when the oracle cannot be driven.
pub fn to_lab(oracle: &Oracle, profile: &Path, rows: &[[f64; 4]]) -> Result<Vec<Lab>, Unavailable> {
    let req = Request {
        input: Space::profile(profile),
        output: Space::lab_v4(),
        intent: Intent::RelativeColorimetric,
        precalc: Precalc::Exact,
        bpc: Bpc::Off,
        values: rows.iter().flatten().map(|v| v * 100.0).collect(),
    };
    let out = oracle
        .convert_batch_shaped(&req, 4, 3)
        .map_err(|e| Unavailable::Error(e.to_string()))?;
    Ok(out
        .into_iter()
        .map(|r| Lab {
            l: r[0],
            a: r[1],
            b: r[2],
        })
        .collect())
}

/// **The tolerance's measured half**: the destination `B2A`'s device response
/// to **one 16-bit PCS quantum**, at the PCS points a probe set actually
/// reaches.
///
/// One quantum is `100/65535` in `L*` and `255/65535` in `a*`/`b*` — the
/// encoding ICC.1 Table 12 fixes for 16-bit PCSLAB, which is what lcms2 carries
/// between the two tables. The function perturbs each axis in turn and returns
/// the largest device movement seen.
///
/// ## What using the oracle here does and does not mean
///
/// The oracle is being used as a **ruler on the fixture**, never as an answer:
/// both the baseline and the perturbed evaluation come from the same
/// `transicc` invocation shape, so what is measured is the *table's slope*, not
/// lcms2's correctness. A defect in lcms2's `B2A` evaluation would inflate this
/// bound rather than bias it, and inflating a bound is the direction that
/// weakens a gate rather than the direction that invents a failure — which is
/// why the row also prints the number and the margin it leaves.
///
/// # Errors
/// [`Unavailable::Error`] when the oracle cannot be driven.
pub fn pcs_quantum_sensitivity(
    oracle: &Oracle,
    dst: &Path,
    labs: &[Lab],
) -> Result<f64, Unavailable> {
    const Q_L: f64 = 100.0 / 65_535.0;
    const Q_AB: f64 = 255.0 / 65_535.0;

    let eval = |pts: &[Lab]| -> Result<Vec<[f64; 4]>, Unavailable> {
        let req = Request {
            input: Space::lab_v4(),
            output: Space::profile(dst),
            intent: Intent::RelativeColorimetric,
            precalc: Precalc::Exact,
            bpc: Bpc::Off,
            values: pts.iter().flat_map(|p| [p.l, p.a, p.b]).collect(),
        };
        let out = oracle
            .convert_batch_shaped(&req, 3, 4)
            .map_err(|e| Unavailable::Error(e.to_string()))?;
        as_cmyk(out).map(|rows| {
            rows.into_iter()
                .map(|r| [r[0] / 100.0, r[1] / 100.0, r[2] / 100.0, r[3] / 100.0])
                .collect()
        })
    };

    let base = eval(labs)?;
    let mut worst = 0.0_f64;
    for axis in 0..3 {
        let moved: Vec<Lab> = labs
            .iter()
            .map(|p| match axis {
                0 => Lab {
                    l: p.l + Q_L,
                    a: p.a,
                    b: p.b,
                },
                1 => Lab {
                    l: p.l,
                    a: p.a + Q_AB,
                    b: p.b,
                },
                _ => Lab {
                    l: p.l,
                    a: p.a,
                    b: p.b + Q_AB,
                },
            })
            .collect();
        worst = worst.max(max_dev(&base, &eval(&moved)?));
    }
    Ok(worst)
}

// ===========================================================================
// Availability
// ===========================================================================

/// Why a section could not run. A missing licensed corpus is a **skip with a
/// reason**; a profile that is present but will not convert is an **error**,
/// because at that point something is wrong that a skip would conceal.
#[derive(Debug, Clone)]
pub enum Unavailable {
    Skip(String),
    Error(String),
}

impl Unavailable {
    fn reason(&self) -> &str {
        match self {
            Unavailable::Skip(s) | Unavailable::Error(s) => s,
        }
    }
    fn is_skip(&self) -> bool {
        matches!(self, Unavailable::Skip(_))
    }
}

/// Resolve one licensed corpus member, or say why it is absent.
///
/// # Errors
/// [`Unavailable::Skip`] when the file is not there — the normal case off the
/// operator's machine, and the permanent case in CI.
pub fn need_corpus(name: &str) -> Result<PathBuf, Unavailable> {
    let p = corpus_dir().join(name);
    if p.is_file() {
        Ok(p)
    } else {
        Err(Unavailable::Skip(format!(
            "ghent-v50 corpus member {name} not present at {} — set $ICCCE_PRIVATE_FIXTURES. \
             It is licensed and cannot be committed, so absence is the normal case and is \
             PERMANENT in CI by design",
            corpus_dir().display()
        )))
    }
}

fn need_synthetic(name: &str) -> Result<PathBuf, Unavailable> {
    let p = synthetic_dir().join(name);
    if p.is_file() {
        Ok(p)
    } else {
        Err(Unavailable::Error(format!(
            "committed synthetic fixture {name} is missing from {} — this one is NOT licensed \
             and its absence is a broken checkout, not a skip",
            synthetic_dir().display()
        )))
    }
}

// ===========================================================================
// §A — the baseline
// ===========================================================================

/// Everything §A measured, kept as data so `note()` and the records read the
/// same numbers and cannot drift.
#[derive(Debug, Clone)]
pub struct Baseline {
    /// Per ICC intent, in the order media-relative, perceptual, saturation,
    /// absolute: `(max chromatic ink, max TAC, max |ΔK|, max ΔE2000 from the
    /// K-only build)`.
    pub per_intent: Vec<(Intent, f64, f64, f64, f64)>,
    /// Where the maximum chromatic ink occurred, at media-relative.
    pub worst_k: f64,
    /// max `|Δ|` device, iccce vs lcms2 `-t1 -c0`, on the same ramp.
    pub vs_oracle: f64,
    /// lcms2's own colorimetric max chromatic ink — the **fixture's** property,
    /// used as `E1`'s candidate separation so the separation does not collapse
    /// when iccce becomes K-preserving.
    pub oracle_chromatic: f64,
    /// max `|K_out - K_in|` in the **oracle's K-preserving** answer on the same
    /// ramp. `A3`'s candidate separation: under a K-preserving path the black
    /// channel would have moved by this instead of by the observed amount.
    /// Taken from the oracle so that it stays a property of the FIXTURE rather
    /// than of the run — the `Separation::against` trap.
    pub oracle_konly_dk: f64,
    /// The run-time-measured PCS-quantum sensitivity used by `A5`.
    pub sensitivity: f64,
    pub points: usize,
}

fn analyse_baseline(oracle: &Oracle, iccce: &Iccce) -> Result<Baseline, Unavailable> {
    let dst = need_corpus(file::ISOCOATED300)?;
    let ramp = k_ramp();
    let rows: Vec<Vec<f64>> = ramp.iter().map(|r| r.to_vec()).collect();
    let lab_konly = to_lab(oracle, &dst, &ramp)?;

    let mut per_intent = Vec::new();
    let mut worst_k = 0.0;
    for intent in [
        Intent::RelativeColorimetric,
        Intent::Perceptual,
        Intent::Saturation,
        Intent::AbsoluteColorimetric,
    ] {
        let out = as_cmyk(
            iccce
                .transform_rows_shaped(&dst, &dst, intent, &rows, 4)
                .map_err(|e| Unavailable::Error(e.to_string()))?,
        )?;
        let chroma = max_chromatic(&out);
        if intent == Intent::RelativeColorimetric {
            worst_k = out
                .iter()
                .zip(&ramp)
                .max_by(|a, b| {
                    let f = |r: &[f64; 4]| r[0].max(r[1]).max(r[2]);
                    f(a.0).total_cmp(&f(b.0))
                })
                .map_or(0.0, |(_, r)| r[3]);
        }
        let tac = max_tac(&out);
        let dk = out
            .iter()
            .zip(&ramp)
            .map(|(o, r)| (o[3] - r[3]).abs())
            .fold(0.0_f64, f64::max);
        let lab_out = to_lab(oracle, &dst, &out)?;
        let de = lab_konly
            .iter()
            .zip(&lab_out)
            .map(|(a, b)| delta_e_2000(*a, *b))
            .fold(0.0_f64, f64::max);
        per_intent.push((intent, chroma, tac, dk, de));
    }

    // The oracle's own colorimetric answer, for A5 and for E1's separation.
    let req = Request {
        input: Space::profile(&dst),
        output: Space::profile(&dst),
        intent: Intent::RelativeColorimetric,
        precalc: Precalc::Exact,
        bpc: Bpc::Off,
        values: ramp.iter().flatten().map(|v| v * 100.0).collect(),
    };
    let theirs: Vec<[f64; 4]> = as_cmyk(
        oracle
            .convert_batch_shaped(&req, 4, 4)
            .map_err(|e| Unavailable::Error(e.to_string()))?,
    )?
    .into_iter()
    .map(|r| [r[0] / 100.0, r[1] / 100.0, r[2] / 100.0, r[3] / 100.0])
    .collect();
    let mine = as_cmyk(
        iccce
            .transform_rows_shaped(&dst, &dst, Intent::RelativeColorimetric, &rows, 4)
            .map_err(|e| Unavailable::Error(e.to_string()))?,
    )?;

    let sensitivity = pcs_quantum_sensitivity(oracle, &dst, &lab_konly)?;

    // The oracle's K-preserving answer, for A3's candidate separation. It is a
    // NON-ICC intent and the record that quotes it says so.
    let konly = KOnlyOracle::new(oracle)
        .convert_cmyk(&dst, &dst, KOnlyOracle::PRESERVE_K_ONLY_RELATIVE, &ramp)
        .map_err(|e| Unavailable::Error(e.to_string()))?;

    Ok(Baseline {
        oracle_konly_dk: konly
            .iter()
            .zip(&ramp)
            .map(|(a, r)| (a[3] - r[3]).abs())
            .fold(0.0_f64, f64::max),
        per_intent,
        worst_k,
        vs_oracle: max_dev(&mine, &theirs),
        oracle_chromatic: max_chromatic(&theirs),
        sensitivity,
        points: ramp.len(),
    })
}

// ===========================================================================
// §B — the intent sweep across the corpus's CMYK destinations
// ===========================================================================

/// One destination's K-ramp behaviour at three intents.
#[derive(Debug, Clone)]
pub struct SweepRow {
    pub name: &'static str,
    pub media_relative: f64,
    pub perceptual: f64,
    pub saturation: f64,
    /// max ΔE2000 between the saturation build and the K-only build — the cost
    /// of the shortcut where the shortcut works.
    pub saturation_de: f64,
}

/// The population §B sweeps. Six real CMYK destinations, named so a reader can
/// check which vendors are and are not represented.
const SWEEP: [(&str, &str); 6] = [
    ("ISO Coated v2 300% (ECI)", file::ISOCOATED300),
    ("ISO Coated v2 (ECI)", file::ISOCOATED350),
    ("Coated FOGRA39", file::FOGRA39),
    ("Coated FOGRA27", file::FOGRA27),
    ("GWG_GenericCMYK", file::GENERIC_CMYK),
    ("GWG_ICC_v4_testprofile (X-Rite)", file::XRITE_V4),
];

fn analyse_sweep(oracle: &Oracle, iccce: &Iccce) -> Result<Vec<SweepRow>, Unavailable> {
    let ramp = k_ramp();
    let rows: Vec<Vec<f64>> = ramp.iter().map(|r| r.to_vec()).collect();
    let mut out = Vec::new();
    for (name, f) in SWEEP {
        let p = need_corpus(f)?;
        let lab_konly = to_lab(oracle, &p, &ramp)?;
        let mut vals = [0.0_f64; 3];
        let mut sat_de = 0.0_f64;
        for (slot, intent) in [
            Intent::RelativeColorimetric,
            Intent::Perceptual,
            Intent::Saturation,
        ]
        .into_iter()
        .enumerate()
        {
            let got = as_cmyk(
                iccce
                    .transform_rows_shaped(&p, &p, intent, &rows, 4)
                    .map_err(|e| Unavailable::Error(e.to_string()))?,
            )?;
            vals[slot] = max_chromatic(&got);
            if intent == Intent::Saturation {
                let lab_out = to_lab(oracle, &p, &got)?;
                sat_de = lab_konly
                    .iter()
                    .zip(&lab_out)
                    .map(|(a, b)| delta_e_2000(*a, *b))
                    .fold(0.0_f64, f64::max);
            }
        }
        out.push(SweepRow {
            name,
            media_relative: vals[0],
            perceptual: vals[1],
            saturation: vals[2],
            saturation_de: sat_de,
        });
    }
    Ok(out)
}

/// The threshold below which a destination's saturation `B2A` counts as
/// "already K-only" for `B1`'s population count.
///
/// It is not a colour tolerance and must not be quoted as one: it separates
/// `0.034`/`0.035` (the two ECI profiles) from `0.507`/`0.730`/`0.759`/`0.783`
/// (the other four) — a gap of **an order of magnitude in each direction**, so
/// no value between `0.04` and `0.5` changes the count. `5×10⁻²` is stated
/// because a threshold must be stated, not because it was fitted.
pub const NEARLY_K_ONLY: f64 = 5e-2;

// ===========================================================================
// §C — the GWG "four different grays" predicate, both legs
// ===========================================================================

/// One gray source's ICC leg measured against ISO 32000-1 §10.3.3's device
/// rule.
#[derive(Debug, Clone)]
pub struct GrayLeg {
    pub name: &'static str,
    /// max `|Δ|` device between Leg I's output and Leg P's `(0, 0, 0, 1−g)`.
    pub device_distance: f64,
    /// max ΔE2000 between the two legs, both rendered through the
    /// destination's own `A2B1`.
    pub colorimetric_distance: f64,
    /// The same ΔE at the ramp midpoint `g = 0.5`.
    ///
    /// ★ This was documented as "GWG's own patch value" until
    /// 2026-08-18. **It is not.** Patch 23.0's panels are `DeviceGray`
    /// **25 %** and `DeviceCMYK` **0/0/0/75** — read from the patch's
    /// own content stream (`.25 g`, `0 0 0 .75 k`) and from its readme,
    /// which sets them in a FIGURE that text extraction silently omits.
    /// `1 − 0.25 = 0.75` is ISO 32000-1 clause 10.3.3 evaluated, which
    /// is itself evidence for DL-059.
    ///
    /// The midpoint is a perfectly good place to sample a ramp. Only
    /// the justification was false — the number never depended on it,
    /// which is exactly why no test could have caught this.
    pub at_half: f64,
}

fn analyse_gray_leg(
    oracle: &Oracle,
    iccce: &Iccce,
    name: &'static str,
    src: &Path,
    dst: &Path,
) -> Result<GrayLeg, Unavailable> {
    let gs = gray_ramp();
    let rows: Vec<Vec<f64>> = gs.iter().map(|g| vec![*g]).collect();
    let leg_i = as_cmyk(
        iccce
            .transform_rows_shaped(src, dst, Intent::RelativeColorimetric, &rows, 4)
            .map_err(|e| Unavailable::Error(e.to_string()))?,
    )?;
    // Leg P: ISO 32000-1 §10.3.3, `c = m = y = 0`, `k = 1 - gray`. One line,
    // no profile, no PCS. It is written here rather than imported because it
    // is not iccce's leg and must not appear to be.
    let leg_p: Vec<[f64; 4]> = gs.iter().map(|g| [0.0, 0.0, 0.0, 1.0 - g]).collect();

    let lab_i = to_lab(oracle, dst, &leg_i)?;
    let lab_p = to_lab(oracle, dst, &leg_p)?;
    let des: Vec<f64> = lab_i
        .iter()
        .zip(&lab_p)
        .map(|(a, b)| delta_e_2000(*a, *b))
        .collect();
    let half = gs.iter().position(|g| (g - 0.5).abs() < 1e-12);
    Ok(GrayLeg {
        name,
        device_distance: max_dev(&leg_i, &leg_p),
        colorimetric_distance: des.iter().copied().fold(0.0_f64, f64::max),
        at_half: half.map_or(f64::NAN, |i| des[i]),
    })
}

// ===========================================================================
// §D — what a K-preserving answer looks like, characterised on the oracle
// ===========================================================================

/// §D's measurements. **Every field here comes from a NON-ICC lcms2 intent**
/// and none of it is evidence about iccce.
#[derive(Debug, Clone)]
pub struct OracleModel {
    /// max `|observed − linear blend of the cell's endpoints|` over the 33
    /// samples of [`cell_ramp`].
    pub cell_model_residual: f64,
    /// max `|Δ|` between the K-only answer and the plain colorimetric answer
    /// **at exactly one cell width**. Must be zero: the sample is a CLUT node
    /// whose K-only corner weight is zero.
    pub coincide_at_cell: f64,
    /// max chromatic ink in the oracle's K-only answer on the K ramp. Must be
    /// exactly zero — it is what `EXACT_ZERO` is calibrated against.
    pub chromatic_on_ramp: f64,
    /// `(pair name, max |K_out − K_in|)` for each source→destination pair.
    pub ktone: Vec<(&'static str, f64)>,
    /// max `|Δ|` between the cell's two ENDPOINTS — the K-only answer at
    /// `C = 0` and the colorimetric answer at `C = 1/16`. It is the scale of
    /// the thing the linear model interpolates, and therefore `D1`'s candidate
    /// separation: under a rival grid size the model would be wrong by up to
    /// this much. **Measured, never typed** (§3.5.8.6).
    pub cell_endpoint_distance: f64,
}

const KTONE_PAIRS: [(&str, &str, &str); 4] = [
    (
        "ISO Coated v2 300% -> itself",
        file::ISOCOATED300,
        file::ISOCOATED300,
    ),
    (
        "ISO Coated v2 300% -> Coated FOGRA39",
        file::ISOCOATED300,
        file::FOGRA39,
    ),
    (
        "ISO Coated v2 300% -> Coated FOGRA27",
        file::ISOCOATED300,
        file::FOGRA27,
    ),
    (
        "ISO Coated v2 300% -> GWG_GenericCMYK",
        file::ISOCOATED300,
        file::GENERIC_CMYK,
    ),
];

fn analyse_oracle_model(oracle: &Oracle) -> Result<OracleModel, Unavailable> {
    let k = KOnlyOracle::new(oracle);
    let dst = need_corpus(file::ISOCOATED300)?;

    let cell = cell_ramp();
    let preserved = k
        .convert_cmyk(&dst, &dst, KOnlyOracle::PRESERVE_K_ONLY_RELATIVE, &cell)
        .map_err(|e| Unavailable::Error(e.to_string()))?;
    let req = Request {
        input: Space::profile(&dst),
        output: Space::profile(&dst),
        intent: Intent::RelativeColorimetric,
        precalc: Precalc::Exact,
        bpc: Bpc::Off,
        values: cell.iter().flatten().map(|v| v * 100.0).collect(),
    };
    let colorimetric: Vec<[f64; 4]> = as_cmyk(
        oracle
            .convert_batch_shaped(&req, 4, 4)
            .map_err(|e| Unavailable::Error(e.to_string()))?,
    )?
    .into_iter()
    .map(|r| [r[0] / 100.0, r[1] / 100.0, r[2] / 100.0, r[3] / 100.0])
    .collect();

    let (a, b) = (preserved[0], preserved[preserved.len() - 1]);
    let mut residual = 0.0_f64;
    for (i, obs) in preserved.iter().enumerate() {
        #[expect(clippy::cast_precision_loss, reason = "i is at most 32; exact in f64")]
        let t = i as f64 / 32.0;
        for c in 0..4 {
            residual = residual.max((obs[c] - (a[c] * (1.0 - t) + b[c] * t)).abs());
        }
    }
    let coincide = colorimetric
        .last()
        .zip(preserved.last())
        .map_or(f64::NAN, |(x, y)| {
            x.iter()
                .zip(y)
                .map(|(p, q)| (p - q).abs())
                .fold(0.0_f64, f64::max)
        });

    let ramp = k_ramp();
    let on_ramp = k
        .convert_cmyk(&dst, &dst, KOnlyOracle::PRESERVE_K_ONLY_RELATIVE, &ramp)
        .map_err(|e| Unavailable::Error(e.to_string()))?;

    let mut ktone = Vec::new();
    for (name, s, d) in KTONE_PAIRS {
        let sp = need_corpus(s)?;
        let dp = need_corpus(d)?;
        let got = k
            .convert_cmyk(&sp, &dp, KOnlyOracle::PRESERVE_K_ONLY_RELATIVE, &ramp)
            .map_err(|e| Unavailable::Error(e.to_string()))?;
        let worst = got
            .iter()
            .zip(&ramp)
            .map(|(o, r)| (o[3] - r[3]).abs())
            .fold(0.0_f64, f64::max);
        ktone.push((name, worst));
    }

    let endpoint = a
        .iter()
        .zip(&b)
        .map(|(p, q)| (p - q).abs())
        .fold(0.0_f64, f64::max);

    Ok(OracleModel {
        cell_endpoint_distance: endpoint,
        cell_model_residual: residual,
        coincide_at_cell: coincide,
        chromatic_on_ramp: max_chromatic(&on_ramp),
        ktone,
    })
}

// ===========================================================================
// §E — the predicates the FEATURE is graded by (written before it existed)
// ===========================================================================

/// **The policy `--preserve-black` is driven with throughout §E and §F.**
///
/// ★ Named as a constant rather than typed at four call sites because a number
/// measured under one policy is uninterpretable beside a number measured under
/// another: `crates/iccce-cmm`'s two variants disagree by up to `4.9e-2` on a
/// cross-press pair, which is an order above the loosest device bound this
/// document family has ever justified. Every §E and §F record interpolates
/// this string into its `why`; none types it.
///
/// It is `k-only-equal-lightness` and **not** `k-only-ratio` for the plain
/// reason that the latter is a named refusal at this commit — and that refusal
/// is itself the honest answer, not a gap: Cholewo (2000)'s `K_MIN`/`K_MAX`
/// determination is not held by this project.
pub const PRESERVE_POLICY: &str = "k-only-equal-lightness";

/// §E's measurements.
#[derive(Debug, Clone)]
pub struct FeatureGate {
    /// **`E1`.** max chromatic ink in **iccce's** answer on the K ramp,
    /// through the surface named by `surface` — which since 2026-08-18 is the
    /// **preserving** surface, `--preserve-black` [`PRESERVE_POLICY`]. Before
    /// the feature existed this row was deliberately red at `0.705320`; it is
    /// now the row that says the feature does what it is named for.
    pub chromatic: f64,
    /// ★ **`E1`'s candidate separation, and it is deliberately NOT
    /// [`FeatureGate::chromatic`].** The distance between the two candidate
    /// answers — *K-only* and *the profile's own colorimetric separation* — is
    /// a property of the **destination table**, so it is measured from
    /// **lcms2's** colorimetric answer. Using iccce's own observation would be
    /// [`Separation::against`]'s trap: the distance would collapse to exactly
    /// zero on the day the feature lands and the row went green, the mechanism
    /// disclaiming its power on the one run that demonstrates it.
    ///
    /// ★★ **That day has now come and this field is why the row survived it.**
    /// `chromatic` fell from `0.705320` to `0.000000`; this number did not
    /// move, because nothing about iccce is in it.
    pub oracle_chromatic: f64,
    /// The exact argument the row drove, printed so that a future reader knows
    /// which surface was measured.
    pub surface: String,
    /// **`E2`.** max `|K_iccce − K_oracle-t11|` over the whole 41-point ramp.
    pub k_vs_oracle: f64,
    /// **`E2`'s node-aligned half.** The same quantity restricted to the `K`
    /// values that land exactly on lcms2's **17-node** black-preserving CLUT
    /// (`K = m/16`, which on a `j/40` ramp is `j ∈ {0,5,10,…,40}`).
    ///
    /// ★★ Splitting the ramp this way is the measurement that changed what
    /// `E2` means: off those nodes lcms2 is interpolating **its own** table,
    /// not evaluating its own construction, and the two are 120×–351× apart
    /// on the four cross-press pairs measured (observed ratios 119.6, 140.7,
    /// 210.1, 351.5).
    pub k_vs_oracle_at_nodes: f64,
    /// **`E2`'s named rival.** max `|K_oracle-t11 − K_in|`, the distance
    /// between "re-map K" and "copy K through" for this pair.
    pub k_copy_rival: f64,
    /// **`E3`.** The width, in device units of cyan, of iccce's K-only region
    /// at `K = 0.5`: the largest `C` at which chromatic ink is still zero.
    pub transition_width: f64,
    /// ★ **`E3`'s disambiguator, and it did not exist before the feature.**
    /// Max chromatic ink at the cell ramp's **`C = 0` endpoint** — the point
    /// that is itself K-only.
    ///
    /// Without it `transition_width` is degenerate: it reads `0.000000` both
    /// when the K-only region is *one point wide* and when there is **no
    /// K-only output at all**, and those are opposite states. The pre-feature
    /// run reported the same `0.000000` for the second reason.
    pub cell_zero_chromatic: f64,
    /// **`E4`.** max `|Δ|` device, iccce vs lcms2, on node-aligned off-neutral
    /// points — **measured through the preserving surface** since 2026-08-18.
    pub node_aligned: f64,
    /// **`E5`.** the same, off the nodes — the control.
    pub arbitrary: f64,
    /// **`E4`'s** run-time bound input.
    pub sensitivity: f64,
    /// ★★ **`E7`.** max `|Δ|` between the **same** off-neutral probes run
    /// with `--preserve-black` and without it. Zero means the preservation
    /// path did not touch an input that does not qualify for it.
    ///
    /// ★★★ **Since 2026-08-21 the probe set is 192 + 70**: the 96 node-aligned
    /// and 96 arbitrary points this section has always used, **plus
    /// [`low_ink_decade_probes`]**. See [`FeatureGate::leak_floor`].
    pub leak: f64,
    /// ★★★ **`E7`'s DETECTION FLOOR, computed and printed every run.**
    ///
    /// `min over probes of max(C, M, Y)` over the whole set (`probe_floor`).
    /// **This is the smallest widening of the qualifying test the row can
    /// see**, and printing it is the entire remedy for a guard whose
    /// sensitivity used to be a property of an LCG seed nobody could audit.
    pub leak_floor: f64,
    /// ★ **The node-aligned set's floor alone** — a grid fact bounded below by
    /// `1/15`, with which grid point the seed reached an *observation*.
    pub leak_node_floor: f64,
    /// ★★ **The arbitrary set's floor alone — SEED-DEPENDENT and printed for
    /// exactly that reason.** Construction bounds it only at `0.8/2²¹`; the
    /// value it has is an accident. If this number moves, someone re-seeded
    /// `arbitrary_off_neutral` and `E7`'s pre-2026-08-21 sensitivity moved
    /// with it.
    pub leak_arb_floor: f64,
    /// ★ **The low-ink set's floor alone** — structural, `LOW_INK_DECADES`'
    /// last entry, and what actually carries `E7`'s reach today.
    pub leak_low_floor: f64,
    /// How many probes the leak comparison ran over.
    pub leak_points: usize,
    /// **`E8`.** max `|K_out − K_in|` on the same-profile pair, where the
    /// construction is provably the identity.
    pub identity_k: f64,
    /// **`E8`'s rival**: max `|K_oracle-t11 − K_in|` on the same pair, i.e.
    /// how far the *oracle's* answer sits from the value algebra requires.
    pub identity_oracle_rival: f64,
    /// **`E9`'s destination** — a genuinely different press, named so that the
    /// row's scope is on its own line.
    pub xp_dst: &'static str,
    /// **`E9`.** `|K_iccce − K_oracle-t11|` on the cross-press pair, at the
    /// oracle's own CLUT nodes only.
    pub xp_k_at_nodes: f64,
    /// **`E9`'s rival**: `|K_oracle-t11 − K_in|` at the same nodes — the
    /// distance to the "copy K through" candidate.
    pub xp_copy_rival_at_nodes: f64,
    /// **`E9`'s** run-time bound input: the cross-press destination's device
    /// response to one 16-bit PCS quantum at the K ramp's own PCS points.
    pub xp_sensitivity: f64,
    /// How many of the 41 ramp points are oracle CLUT nodes.
    pub xp_node_points: usize,
    /// **`E6`.** max chromatic ink on the committed synthetic CMYK fixture's
    /// K ramp: zero, which is what makes it useless for this subject.
    pub synthetic_chromatic: f64,
    pub node_points: usize,
}

/// The K values of [`k_ramp`] that are exact nodes of lcms2's **17-node**
/// black-preserving CLUT.
///
/// `_cmsReasonableGridpointsByColorspace` returns 17 for a 4-channel space, so
/// the nodes sit at `m/16`. The ramp samples `j/40`, and `j/40 = m/16` exactly
/// when `j` is a multiple of 5. **Derived from the two grid sizes, not chosen**
/// — if lcms2's constant ever changes this set silently becomes wrong, which is
/// why `D1`/`D2` grade that constant directly.
fn oracle_node_indices() -> Vec<usize> {
    (0..=40).filter(|j| j % 5 == 0).collect()
}

fn analyse_feature_gate(oracle: &Oracle, iccce: &Iccce) -> Result<FeatureGate, Unavailable> {
    let dst = need_corpus(file::ISOCOATED300)?;
    let ramp = k_ramp();
    let rows: Vec<Vec<f64>> = ramp.iter().map(|r| r.to_vec()).collect();
    let err = |e: DiffError| Unavailable::Error(e.to_string());
    let preserved = |src: &Path, d: &Path, r: &[Vec<f64>]| -> Result<Vec<[f64; 4]>, Unavailable> {
        as_cmyk(
            iccce
                .transform_rows_shaped_preserve_black(
                    src,
                    d,
                    Intent::RelativeColorimetric,
                    r,
                    4,
                    PRESERVE_POLICY,
                )
                .map_err(err)?,
        )
    };
    let plain = |src: &Path, d: &Path, r: &[Vec<f64>]| -> Result<Vec<[f64; 4]>, Unavailable> {
        as_cmyk(
            iccce
                .transform_rows_shaped(src, d, Intent::RelativeColorimetric, r, 4)
                .map_err(err)?,
        )
    };
    let oracle_cmyk =
        |s: &Path, d: &Path, probes: &[[f64; 4]]| -> Result<Vec<[f64; 4]>, Unavailable> {
            let req = Request {
                input: Space::profile(s),
                output: Space::profile(d),
                intent: Intent::RelativeColorimetric,
                precalc: Precalc::Exact,
                bpc: Bpc::Off,
                values: probes.iter().flatten().map(|v| v * 100.0).collect(),
            };
            Ok(
                as_cmyk(oracle.convert_batch_shaped(&req, 4, 4).map_err(err)?)?
                    .into_iter()
                    .map(|r| [r[0] / 100.0, r[1] / 100.0, r[2] / 100.0, r[3] / 100.0])
                    .collect(),
            )
        };

    // ★ THE REPOINTING, 2026-08-18. Everything §E says about the PREDICATE now
    // drives `--preserve-black`; the plain surface is still driven alongside
    // it, so that the two can be compared against each other (E7) rather than
    // only against lcms2.
    let mine = preserved(&dst, &dst, &rows)?;
    let k = KOnlyOracle::new(oracle);
    let theirs = k
        .convert_cmyk(&dst, &dst, KOnlyOracle::PRESERVE_K_ONLY_RELATIVE, &ramp)
        .map_err(err)?;

    // E3: how wide is iccce's K-only region at K = 0.5? Walk the same cell the
    // oracle's model occupies and find the last point at which chromatic ink is
    // still exactly zero. `cell_zero_chromatic` records the C = 0 endpoint
    // separately, because a width of zero has two opposite meanings without it.
    let cell = cell_ramp();
    let cell_rows: Vec<Vec<f64>> = cell.iter().map(|r| r.to_vec()).collect();
    let cell_out = preserved(&dst, &dst, &cell_rows)?;
    let mut width = 0.0_f64;
    for (inp, out) in cell.iter().zip(&cell_out) {
        if out[0].max(out[1]).max(out[2]) == 0.0 {
            width = inp[0];
        } else {
            break;
        }
    }
    let cell_zero_chromatic = cell_out
        .first()
        .map_or(f64::NAN, |r| r[0].max(r[1]).max(r[2]));

    // §E's two off-neutral probe sets, run BOTH ways. The preserved run is what
    // E4/E5 grade — the feature must be IN THE LOOP of the row that claims it
    // does not leak — and the difference between the two runs is E7.
    let node = node_aligned_off_neutral();
    let node_rows: Vec<Vec<f64>> = node.iter().map(|r| r.to_vec()).collect();
    let node_mine = preserved(&dst, &dst, &node_rows)?;
    let node_plain = plain(&dst, &dst, &node_rows)?;
    let node_theirs = oracle_cmyk(&dst, &dst, &node)?;

    let arb = arbitrary_off_neutral();
    let arb_rows: Vec<Vec<f64>> = arb.iter().map(|r| r.to_vec()).collect();
    let arb_mine = preserved(&dst, &dst, &arb_rows)?;
    let arb_plain = plain(&dst, &dst, &arb_rows)?;
    let arb_theirs = oracle_cmyk(&dst, &dst, &arb)?;

    // ★★★ THE LOW-INK ARM, 2026-08-21. `E7`'s reach used to stop at
    // `1.106777e-1` — the smallest chromatic coordinate a fixed LCG seed
    // happened to draw — while the rival its own justification named was
    // `1e-9`. These 70 probes walk 14 decades down to `1e-12` and are folded
    // into the SAME number, deliberately: a second row would let one of the two
    // go green while the other went red and still read as "the leak guard
    // passed". They enter the leak comparison ONLY. `E4`/`E5` keep their own
    // probe sets untouched, because those rows are cross-checks against lcms2
    // whose bound is an interpolation envelope derived for THOSE points.
    let low = low_ink_decade_probes();
    let low_rows: Vec<Vec<f64>> = low.iter().map(|r| r.to_vec()).collect();
    let low_mine = preserved(&dst, &dst, &low_rows)?;
    let low_plain = plain(&dst, &dst, &low_rows)?;

    let leak = max_dev(&node_mine, &node_plain)
        .max(max_dev(&arb_mine, &arb_plain))
        .max(max_dev(&low_mine, &low_plain));
    let leak_node_floor = probe_floor(&node);
    let leak_arb_floor = probe_floor(&arb);
    let leak_low_floor = probe_floor(&low);
    let leak_floor = leak_node_floor.min(leak_arb_floor).min(leak_low_floor);
    let leak_points = node.len() + arb.len() + low.len();

    let node_labs = to_lab(oracle, &dst, &node)?;
    let sensitivity = pcs_quantum_sensitivity(oracle, &dst, &node_labs)?;

    // §E9 — the CROSS-PRESS arm. On a same-press pair the two candidate K
    // answers (equal lightness, copy through) coincide to 6.1e-5, so the
    // section's own destination cannot tell them apart. GWG_GenericCMYK can.
    let xp = need_corpus(file::GENERIC_CMYK)?;
    let xp_mine = preserved(&dst, &xp, &rows)?;
    let xp_theirs = k
        .convert_cmyk(&dst, &xp, KOnlyOracle::PRESERVE_K_ONLY_RELATIVE, &ramp)
        .map_err(err)?;
    let nodes = oracle_node_indices();
    let xp_k_at_nodes = nodes
        .iter()
        .map(|&j| (xp_mine[j][3] - xp_theirs[j][3]).abs())
        .fold(0.0_f64, f64::max);
    let xp_copy_rival_at_nodes = nodes
        .iter()
        .map(|&j| (xp_theirs[j][3] - ramp[j][3]).abs())
        .fold(0.0_f64, f64::max);
    let ramp_labs = to_lab(oracle, &dst, &ramp)?;
    let xp_sensitivity = pcs_quantum_sensitivity(oracle, &xp, &ramp_labs)?;

    let syn = need_synthetic(SYNTHETIC_CMYK)?;
    let syn_out = preserved(&syn, &syn, &rows)?;

    // The fixture's OWN separation between the two candidate answers, taken
    // from lcms2's colorimetric result so that it does not move when iccce's
    // does. See `FeatureGate::oracle_chromatic`.
    let oracle_colorimetric = oracle_cmyk(&dst, &dst, &ramp)?;

    Ok(FeatureGate {
        chromatic: max_chromatic(&mine),
        oracle_chromatic: max_chromatic(&oracle_colorimetric),
        surface: format!(
            "iccce transform --src <ISO Coated v2 300% (ECI)> --dst <same> --intent {} \
             --preserve-black {} (repointed 2026-08-18, per this row's own pre-feature \
             instruction; the plain surface is still driven alongside it and the difference is \
             row passk/E/regression/preservation-does-not-touch-a-non-qualifying-input)",
            Intent::RelativeColorimetric.name(),
            PRESERVE_POLICY
        ),
        k_vs_oracle: mine
            .iter()
            .zip(&theirs)
            .map(|(a, b)| (a[3] - b[3]).abs())
            .fold(0.0_f64, f64::max),
        k_vs_oracle_at_nodes: nodes
            .iter()
            .map(|&j| (mine[j][3] - theirs[j][3]).abs())
            .fold(0.0_f64, f64::max),
        k_copy_rival: theirs
            .iter()
            .zip(&ramp)
            .map(|(o, r)| (o[3] - r[3]).abs())
            .fold(0.0_f64, f64::max),
        transition_width: width,
        cell_zero_chromatic,
        node_aligned: max_dev(&node_mine, &node_theirs),
        arbitrary: max_dev(&arb_mine, &arb_theirs),
        sensitivity,
        leak,
        leak_floor,
        leak_node_floor,
        leak_arb_floor,
        leak_low_floor,
        leak_points,
        identity_k: mine
            .iter()
            .zip(&ramp)
            .map(|(a, r)| (a[3] - r[3]).abs())
            .fold(0.0_f64, f64::max),
        identity_oracle_rival: theirs
            .iter()
            .zip(&ramp)
            .map(|(o, r)| (o[3] - r[3]).abs())
            .fold(0.0_f64, f64::max),
        xp_dst: "GWG_GenericCMYK",
        xp_k_at_nodes,
        xp_copy_rival_at_nodes,
        xp_sensitivity,
        xp_node_points: nodes.len(),
        synthetic_chromatic: max_chromatic(&syn_out),
        node_points: node.len(),
    })
}

// ===========================================================================
// Assembly
// ===========================================================================

/// Everything Pass K measured, for `note()` and for a caller that wants the
/// numbers rather than the records.
#[derive(Debug, Clone, Default)]
pub struct Bundle {
    pub baseline: Option<Baseline>,
    pub sweep: Option<Vec<SweepRow>>,
    pub gray: Option<Vec<GrayLeg>>,
    pub model: Option<OracleModel>,
    pub gate: Option<FeatureGate>,
    /// §F's file-only reading of the committed **separating** fixture. Present
    /// even when no `iccce` binary was found — it needs neither that nor the
    /// oracle.
    pub separating: Option<Separating>,
    /// §F's run of that fixture through the shipped binary and the oracle.
    pub separating_run: Option<SeparatingRun>,
    /// §G's licensed legs — the cross-press headline, its reversal, and the
    /// same-press control.
    pub cost: Option<Cost>,
    /// §G's committed leg, which runs in CI and exists to show why the
    /// headline cannot.
    pub cost_synthetic: Option<CostLeg>,
    /// §G's OTHER committed leg — the pair authored so that a ΔE cost row can
    /// run without a licence — and the largest departure of its measurement
    /// from the closed form derived from the two recipes.
    pub cost_warm_black: Option<(CostLeg, (f64, f64, f64))>,
    /// §G's population sweep over every ordered pair of the licensed CMYK
    /// members.
    pub cost_population: Option<CostPopulation>,
    pub unavailable: Vec<String>,
}

fn skip_or_error(
    records: &mut Vec<Record>,
    ids: &[(&str, Kind, Metric, Tolerance)],
    u: &Unavailable,
    source: &str,
) {
    for (id, kind, metric, tol) in ids {
        if u.is_skip() {
            records.push(Record::skipped(
                *id,
                *kind,
                *metric,
                *tol,
                source.to_string(),
                u.reason().to_string(),
            ));
        } else {
            records.push(Record::errored(
                *id,
                *kind,
                *metric,
                *tol,
                source.to_string(),
                u.reason().to_string(),
            ));
        }
    }
}

/// Run Pass K.
///
/// Nothing here panics on a missing corpus: every section that cannot run emits
/// **one labelled record per row it would have emitted**, so the report's row
/// count is stable and a reader can see exactly which claims did not run.
#[must_use]
pub fn run(oracle: &Oracle) -> (Bundle, Vec<Record>) {
    let mut b = Bundle::default();
    let mut records = Vec::new();

    // ★ §F's file-only rows come FIRST, before the binary is looked for. They
    // read the committed separating fixture and assert properties of its
    // BYTES, so they can run on a machine that has built nothing — and a
    // fixture that is not what its recipe says invalidates every §F row that
    // follows it, which a reader should learn before the skips rather than
    // after them.
    let separating = analyse_separating();
    match &separating {
        Ok(x) => records.extend(separating_file_records(x)),
        Err(u) => {
            b.unavailable.push(format!("§F (file): {}", u.reason()));
            skip_or_error(&mut records, &F_FILE_ROWS, u, "Pass K §F");
        }
    }

    let iccce = match Iccce::locate() {
        Ok(Some(i)) => i,
        Ok(None) => {
            b.unavailable.push(
                "the shipped iccce binary was not found; build with `cargo build --release \
                       -p iccce-cli` or set $ICCCE_BIN"
                    .to_string(),
            );
            let u = Unavailable::Skip(
                "the shipped iccce binary was not found — every remaining row in Pass K drives \
                 it as a subprocess, deliberately (a feature reachable only from the library is \
                 not reachable by a consumer). §F's three FILE rows are unaffected and have \
                 already been emitted: they read the committed fixture's bytes and need no \
                 binary"
                    .into(),
            );
            skip_or_error(
                &mut records,
                &ALL_ROWS,
                &u,
                "Pass K, tools/difftest/src/passk.rs",
            );
            skip_or_error(&mut records, &F_XFORM_ROWS, &u, "Pass K §F");
            skip_or_error(&mut records, &G_ROWS, &u, "Pass K §G");
            skip_or_error(&mut records, &G_SYNTHETIC_ROWS, &u, "Pass K §G");
            skip_or_error(&mut records, &G_WARM_ROWS, &u, "Pass K §G");
            skip_or_error(&mut records, &G_POPULATION_ROWS, &u, "Pass K §G");
            b.separating = separating.ok();
            return (b, records);
        }
        Err(e) => {
            b.unavailable.push(e.to_string());
            let u = Unavailable::Error(e.to_string());
            skip_or_error(
                &mut records,
                &ALL_ROWS,
                &u,
                "Pass K, tools/difftest/src/passk.rs",
            );
            skip_or_error(&mut records, &F_XFORM_ROWS, &u, "Pass K §F");
            skip_or_error(&mut records, &G_ROWS, &u, "Pass K §G");
            skip_or_error(&mut records, &G_SYNTHETIC_ROWS, &u, "Pass K §G");
            skip_or_error(&mut records, &G_WARM_ROWS, &u, "Pass K §G");
            skip_or_error(&mut records, &G_POPULATION_ROWS, &u, "Pass K §G");
            b.separating = separating.ok();
            return (b, records);
        }
    };

    match analyse_baseline(oracle, &iccce) {
        Ok(x) => {
            records.extend(baseline_records(&x));
            b.baseline = Some(x);
        }
        Err(u) => {
            b.unavailable.push(format!("§A: {}", u.reason()));
            skip_or_error(&mut records, &A_ROWS, &u, "Pass K §A");
        }
    }
    match analyse_sweep(oracle, &iccce) {
        Ok(x) => {
            records.extend(sweep_records(&x));
            b.sweep = Some(x);
        }
        Err(u) => {
            b.unavailable.push(format!("§B: {}", u.reason()));
            skip_or_error(&mut records, &B_ROWS, &u, "Pass K §B");
        }
    }
    match analyse_gray(oracle, &iccce) {
        Ok(x) => {
            records.extend(gray_records(&x));
            b.gray = Some(x);
        }
        Err(u) => {
            b.unavailable.push(format!("§C: {}", u.reason()));
            skip_or_error(&mut records, &C_ROWS, &u, "Pass K §C");
        }
    }
    match analyse_oracle_model(oracle) {
        Ok(x) => {
            records.extend(model_records(&x));
            b.model = Some(x);
        }
        Err(u) => {
            b.unavailable.push(format!("§D: {}", u.reason()));
            skip_or_error(&mut records, &D_ROWS, &u, "Pass K §D");
        }
    }
    match analyse_feature_gate(oracle, &iccce) {
        Ok(x) => {
            records.extend(gate_records(&x));
            b.gate = Some(x);
        }
        Err(u) => {
            b.unavailable.push(format!("§E: {}", u.reason()));
            skip_or_error(&mut records, &E_ROWS, &u, "Pass K §E");
        }
    }
    match &separating {
        Ok(f) => match analyse_separating_run(oracle, &iccce, f) {
            Ok(x) => {
                records.extend(separating_run_records(f, &x));
                b.separating_run = Some(x);
            }
            Err(u) => {
                b.unavailable.push(format!("§F: {}", u.reason()));
                skip_or_error(&mut records, &F_XFORM_ROWS, &u, "Pass K §F");
            }
        },
        Err(u) => {
            // The fixture itself did not read. Nothing downstream of it can be
            // graded, and the reason was already reported on the file rows.
            skip_or_error(&mut records, &F_XFORM_ROWS, u, "Pass K §F");
        }
    }
    b.separating = separating.ok();

    // §G. The COMMITTED leg is measured first, for the same reason §F's file
    // rows come first: it runs everywhere, and it is the row that says why the
    // licensed one has to exist. Each set of records is given the other leg so
    // that every cross-reference between them is INTERPOLATED from a
    // measurement rather than typed — and reads "[not measured in this run]"
    // when the other leg did not run, which is the honest state and not a
    // number somebody would have had to keep current.
    let cost_syn = match analyse_cost_synthetic(oracle, &iccce) {
        Ok(x) => Some(x),
        Err(u) => {
            b.unavailable
                .push(format!("§G (committed pair): {}", u.reason()));
            skip_or_error(&mut records, &G_SYNTHETIC_ROWS, &u, "Pass K §G");
            skip_or_error(&mut records, &G_WARM_ROWS, &u, "Pass K §G");
            skip_or_error(&mut records, &G_POPULATION_ROWS, &u, "Pass K §G");
            None
        }
    };
    match analyse_cost(oracle, &iccce) {
        Ok(x) => {
            records.extend(cost_records(&x, cost_syn.as_ref()));
            b.cost = Some(x);
        }
        Err(u) => {
            b.unavailable.push(format!("§G: {}", u.reason()));
            skip_or_error(&mut records, &G_ROWS, &u, "Pass K §G");
        }
    }
    match (analyse_cost_population(oracle, &iccce), b.cost.as_ref()) {
        (Ok(p), Some(c)) => {
            records.extend(cost_population_records(&p, &c.cross));
            b.cost_population = Some(p);
        }
        (Ok(_), None) => {
            // Unreachable in practice: the population sweep needs the same
            // corpus §G's headline does. Emitted rather than panicked on.
            let u = Unavailable::Error(
                "the population sweep ran but the headline pair did not, so the row has no pair \
                 to state its choice against"
                    .into(),
            );
            skip_or_error(&mut records, &G_POPULATION_ROWS, &u, "Pass K §G");
        }
        (Err(u), _) => {
            b.unavailable
                .push(format!("§G (population): {}", u.reason()));
            skip_or_error(&mut records, &G_POPULATION_ROWS, &u, "Pass K §G");
        }
    }
    if let Some(x) = &cost_syn {
        records.extend(cost_synthetic_records(x, b.cost.as_ref()));
    }
    match analyse_cost_warm_black(oracle, &iccce)
        .and_then(|leg| Ok((leg, warm_black_derivation_residual(oracle, &iccce)?)))
    {
        Ok((leg, residual)) => {
            records.extend(cost_warm_black_records(
                &leg,
                residual,
                b.cost.as_ref(),
                cost_syn.as_ref(),
            ));
            b.cost_warm_black = Some((leg, residual));
        }
        Err(u) => {
            b.unavailable
                .push(format!("§G (warm-black pair): {}", u.reason()));
            skip_or_error(&mut records, &G_WARM_ROWS, &u, "Pass K §G");
            skip_or_error(&mut records, &G_POPULATION_ROWS, &u, "Pass K §G");
        }
    }
    b.cost_synthetic = cost_syn;

    (b, records)
}

fn analyse_gray(oracle: &Oracle, iccce: &Iccce) -> Result<Vec<GrayLeg>, Unavailable> {
    let dst = need_corpus(file::ISOCOATED300)?;
    let press = need_corpus(file::PRESS_GRAY)?;
    let syn = need_synthetic(SYNTHETIC_GRAY)?;
    Ok(vec![
        analyse_gray_leg(
            oracle,
            iccce,
            "Schwarze Druckfarbe - ISO Coated v2 (ECI) [the destination press's OWN black ink]",
            &press,
            &dst,
        )?,
        analyse_gray_leg(
            oracle,
            iccce,
            "fixtures/synthetic/v2-gray-curv-gamma.icc [an ordinary gamma-2.2 gray]",
            &syn,
            &dst,
        )?,
    ])
}

// ---------------------------------------------------------------------------
// Row identifiers, declared once so a skip emits exactly the rows a run would
// ---------------------------------------------------------------------------

const CC: Kind = Kind::CrossCheck;
const OR: Kind = Kind::OracleReproducibility;
/// ★ **§F's kind.** An expectation derived by arithmetic from the bytes of a
/// SYNTHETIC fixture and the specification's stated encoding, with no
/// implementation's output in it — stronger than a cross-check, weaker than a
/// published value, and the only kind available for a predicate no standards
/// body has printed a number for. Every §F row carries it except the one that
/// is explicitly the third reading.
const DE_KIND: Kind = Kind::DerivedExpectation;
/// ★ **The kind of the two post-feature LEAK rows** (`E7`, `F8`). Both
/// sides are `iccce transform`, differing only in whether `--preserve-black`
/// was passed, so nothing outside this project is in the loop and the claim is
/// **self-consistency**: the weakest evidence class this suite emits. It is
/// used here anyway, and deliberately, because the *predicate* is exact —
/// "these two invocations printed the same bytes" — where every cross-check
/// available for the same question carries an interpolation envelope two
/// orders wider. A weak class with an exact predicate catches a leak that a
/// strong class with a loose bound would absorb.
const SELF: Kind = Kind::SelfConsistency;
const DEV: Metric = Metric::DeviceAbsMaxNormalised;
const DE: Metric = Metric::DeltaE2000Max;
const CNT: Metric = Metric::IndicatorCount;

const A_ROWS: [(&str, Kind, Metric, Tolerance); 8] = [
    (
        "passk/A/isocoated300/k-ramp/media-relative/chromatic-ink",
        CC,
        DEV,
        REPORTED,
    ),
    (
        "passk/A/isocoated300/k-ramp/media-relative/total-area-coverage",
        CC,
        DEV,
        REPORTED,
    ),
    (
        "passk/A/isocoated300/k-ramp/media-relative/black-channel-reduction",
        CC,
        DEV,
        REPORTED,
    ),
    (
        "passk/A/isocoated300/k-ramp/media-relative/dE-is-BLIND-to-the-defect",
        CC,
        DE,
        DE_PERCEPTIBLE,
    ),
    (
        "passk/A/isocoated300/k-ramp/media-relative/agrees-with-lcms2",
        CC,
        DEV,
        REPORTED,
    ),
    (
        "passk/A/isocoated300/k-ramp/perceptual/chromatic-ink",
        CC,
        DEV,
        REPORTED,
    ),
    (
        "passk/A/isocoated300/k-ramp/saturation/chromatic-ink",
        CC,
        DEV,
        REPORTED,
    ),
    (
        "passk/A/isocoated300/k-ramp/absolute/chromatic-ink",
        CC,
        DEV,
        REPORTED,
    ),
];

const B_ROWS: [(&str, Kind, Metric, Tolerance); 7] = [
    (
        "passk/B/saturation-is-NOT-a-general-k-preservation-substitute",
        CC,
        CNT,
        SHORTCUT_SATURATION,
    ),
    (
        "passk/B/isocoated300/saturation/chromatic-ink",
        CC,
        DEV,
        REPORTED,
    ),
    (
        "passk/B/isocoated350/saturation/chromatic-ink",
        CC,
        DEV,
        REPORTED,
    ),
    (
        "passk/B/fogra39/saturation/chromatic-ink",
        CC,
        DEV,
        REPORTED,
    ),
    (
        "passk/B/fogra27/saturation/chromatic-ink",
        CC,
        DEV,
        REPORTED,
    ),
    (
        "passk/B/generic-cmyk/saturation/chromatic-ink",
        CC,
        DEV,
        REPORTED,
    ),
    (
        "passk/B/xrite-v4/saturation/chromatic-ink",
        CC,
        DEV,
        REPORTED,
    ),
];

const C_ROWS: [(&str, Kind, Metric, Tolerance); 5] = [
    (
        "passk/C/leg-I-icc/press-gray/device-distance-from-leg-P",
        CC,
        DEV,
        REPORTED,
    ),
    (
        "passk/C/leg-I-icc/press-gray/colorimetric-distance-from-leg-P",
        CC,
        DE,
        REPORTED,
    ),
    (
        "passk/C/leg-I-icc/synthetic-gamma22-gray/device-distance-from-leg-P",
        CC,
        DEV,
        REPORTED,
    ),
    (
        "passk/C/leg-I-icc/synthetic-gamma22-gray/colorimetric-distance-from-leg-P",
        CC,
        DE,
        REPORTED,
    ),
    (
        "passk/C/the-two-legs-are-NOT-interchangeable",
        CC,
        CNT,
        SHORTCUT_GRAY_LEG,
    ),
];

const D_ROWS: [(&str, Kind, Metric, Tolerance); 7] = [
    (
        "passk/D/lcms2-intent-11/k-only-region-is-ONE-clut-cell-wide",
        OR,
        DEV,
        CLUT_CELL_MODEL,
    ),
    (
        "passk/D/lcms2-intent-11/coincides-with-colorimetric-at-one-cell",
        OR,
        DEV,
        EXACT_ZERO,
    ),
    (
        "passk/D/lcms2-intent-11/chromatic-ink-on-the-k-ramp-is-exactly-zero",
        OR,
        DEV,
        EXACT_ZERO,
    ),
    (
        "passk/D/lcms2-intent-11/ktone/isocoated300-to-itself",
        OR,
        DEV,
        REPORTED,
    ),
    (
        "passk/D/lcms2-intent-11/ktone/isocoated300-to-fogra39",
        OR,
        DEV,
        REPORTED,
    ),
    (
        "passk/D/lcms2-intent-11/ktone/isocoated300-to-fogra27",
        OR,
        DEV,
        REPORTED,
    ),
    (
        "passk/D/lcms2-intent-11/ktone/isocoated300-to-generic-cmyk",
        OR,
        DEV,
        REPORTED,
    ),
];

const E_ROWS: [(&str, Kind, Metric, Tolerance); 9] = [
    ("passk/E/k-only-in-implies-k-only-out", CC, DEV, EXACT_ZERO),
    (
        "passk/E/preserved-k-value-vs-the-oracle-tone-curve",
        CC,
        DEV,
        REPORTED,
    ),
    ("passk/E/near-neutral-transition-width", CC, DEV, REPORTED),
    (
        "passk/E/regression/node-aligned-off-neutral-agrees-with-lcms2",
        CC,
        DEV,
        REPORTED,
    ),
    (
        "passk/E/regression/off-node-envelope-is-NOT-zero",
        CC,
        DEV,
        REPORTED,
    ),
    (
        "passk/E/synthetic-cmyk-fixture-is-ZERO-SEPARATION-for-this-subject",
        CC,
        DEV,
        REPORTED,
    ),
    (
        "passk/E/regression/preservation-does-not-touch-a-non-qualifying-input",
        SELF,
        DEV,
        EXACT_ZERO,
    ),
    (
        "passk/E/preserved-k-is-the-IDENTITY-on-a-same-profile-pair",
        DE_KIND,
        DEV,
        PRINT_FLOOR,
    ),
    (
        "passk/E/cross-press/preserved-k-matches-the-oracle-at-its-own-clut-nodes",
        CC,
        DEV,
        REPORTED,
    ),
];

/// ★ **§F's file-only rows.** They need neither the oracle nor the shipped
/// binary: they read the committed fixture and assert properties of its bytes.
/// Emitted BEFORE the binary is looked for, deliberately — a corrupted fixture
/// must be reported on a machine that has built nothing, and every other row
/// in this module rests on the fixture being what its recipe says.
const F_FILE_ROWS: [(&str, Kind, Metric, Tolerance); 3] = [
    (
        "passk/F/synthetic-chromatic-neutral/b2a-is-a-b-INDEPENDENT-across-the-dead-band",
        DE_KIND,
        DEV,
        EXACT_ZERO,
    ),
    (
        "passk/F/synthetic-chromatic-neutral/b2a-neutral-column-matches-the-authored-model",
        DE_KIND,
        DEV,
        HALF_QUANTUM,
    ),
    (
        "passk/F/synthetic-chromatic-neutral/separation-is-above-the-declared-floor",
        DE_KIND,
        DEV,
        SEPARATION_FLOOR_MET,
    ),
];

/// **§F's transform rows** — the ones that drive the shipped binary. All four
/// run in CI: the fixture is committed and unlicensed, so nothing here skips
/// for want of a corpus.
const F_XFORM_ROWS: [(&str, Kind, Metric, Tolerance); 5] = [
    (
        "passk/F/synthetic-chromatic-neutral/chromatic-gray-round-trip-matches-the-derived-table",
        DE_KIND,
        DEV,
        TABLE_INTERPOLATION,
    ),
    (
        "passk/F/synthetic-chromatic-neutral/k-only-in-implies-k-only-out",
        DE_KIND,
        DEV,
        EXACT_ZERO,
    ),
    (
        "passk/F/synthetic-chromatic-neutral/near-neutral-transition-width",
        DE_KIND,
        DEV,
        REPORTED,
    ),
    (
        "passk/F/synthetic-chromatic-neutral/regression/chromatic-gray-round-trip-agrees-with-lcms2",
        CC,
        DEV,
        ORACLE_CHAIN,
    ),
    (
        "passk/F/synthetic-chromatic-neutral/regression/preservation-does-not-touch-a-non-qualifying-input",
        SELF,
        DEV,
        EXACT_ZERO,
    ),
];

/// Every row this pass can emit, for the "no iccce binary" case.
const ALL_ROWS: [(&str, Kind, Metric, Tolerance); 36] = {
    let mut out = [A_ROWS[0]; 36];
    let mut i = 0;
    while i < 8 {
        out[i] = A_ROWS[i];
        i += 1;
    }
    let mut j = 0;
    while j < 7 {
        out[8 + j] = B_ROWS[j];
        j += 1;
    }
    let mut k = 0;
    while k < 5 {
        out[15 + k] = C_ROWS[k];
        k += 1;
    }
    let mut l = 0;
    while l < 7 {
        out[20 + l] = D_ROWS[l];
        l += 1;
    }
    let mut m = 0;
    while m < 9 {
        out[27 + m] = E_ROWS[m];
        m += 1;
    }
    out
};

// ---------------------------------------------------------------------------
// Records
// ---------------------------------------------------------------------------

const SRC_BASELINE: &str = "Pass K §A — BOTH SIDES MEASURED IN THIS RUN. iccce's numbers come \
    from running the shipped `iccce transform` binary; lcms2's from the pinned transicc \
    (21c582a). Fixture: ISO Coated v2 300% (ECI), Ghent v5.0 corpus, licensed and uncommittable";

fn baseline_records(x: &Baseline) -> Vec<Record> {
    let mut out = Vec::new();
    let mr = x.per_intent[0];

    out.push(
        Record::graded(
            A_ROWS[0].0,
            CC,
            DEV,
            REPORTED,
            mr.1,
            SRC_BASELINE,
            format!(
                "★ THE BASELINE. {} K-only points (0,0,0,K), K = 0..1, converted ISO Coated v2 \
                 300% (ECI) -> ITSELF at media-relative colorimetric. Largest chromatic ink in \
                 the output: {:.6}, at K_in = {:.3}. The input carries NONE. This is not an \
                 iccce defect - see the agrees-with-lcms2 row, {:.6e} - it is the profile's own \
                 B2A1 separation, and it is the quantity black preservation exists to remove. \
                 The ramp is not flat and the shape is the finding: run \
                 `cargo run -p iccce-difftest --bin passk_probe` to print it",
                x.points, mr.1, x.worst_k, x.vs_oracle
            ),
        )
        .with_separation(Separation::none(
            "a baseline has no rival candidate: nothing is being read two ways. What the number \
             would be under a K-preserving path is measured separately by row \
             passk/E/k-only-in-implies-k-only-out, which states its own separation",
        )),
    );

    out.push(Record::graded(
        A_ROWS[1].0,
        CC,
        DEV,
        REPORTED,
        mr.2,
        SRC_BASELINE,
        format!(
            "Total area coverage of the output, max over the ramp: {:.6} ({:.1} %). The INPUT's \
             maximum TAC is 1.000000 (100 %), because a K-only build cannot exceed one plate. \
             ★ The output stays INSIDE the 300 % the profile's own name declares, so the \
             destination's separation is doing exactly what it was authored to do - which is the \
             point of this row: nothing here is a defect, and a conformant table has turned a \
             one-plate black into {:.2}x the ink the document asked for. Reported because ink \
             cost is the reason the requirement exists and no dE can express it",
            mr.2,
            mr.2 * 100.0,
            mr.2
        ),
    )
    .with_separation(Separation::none(
        "considered, and there is no rival READING of this observation: total area coverage is a \
         sum of four numbers that came out of one conversion, and nothing about it is read two \
         ways. What the figure would be under a K-preserving path is bounded by the K-only \
         build's own TAC, which cannot exceed 1.000000 by construction, and that comparison is \
         carried by row passk/E/k-only-in-implies-k-only-out",
    )));

    out.push(Record::graded(
        A_ROWS[2].0,
        CC,
        DEV,
        REPORTED,
        mr.3,
        SRC_BASELINE,
        format!(
            "★ The black channel is not merely diluted, it is REDUCED: max |K_out - K_in| = \
             {:.6}. A K-only 60 % black comes back with roughly a quarter of the black it went \
             in with, the rest of the darkness supplied by cyan, magenta and yellow. This is the \
             component of the baseline most likely to be missed by an eye check, because the \
             patch still LOOKS right",
            mr.3
        ),
    )
    .with_separation(Separation::against_distance(
        k_only_alt(
            "a K-preserving path, under which the black channel moves only by the destination's \
             own black-ink tone curve",
        ),
        x.oracle_konly_dk,
        (mr.3 - x.oracle_konly_dk).abs(),
        SepUnits::SameAsMetric,
    )));

    out.push(
        Record::graded(
            A_ROWS[3].0,
            CC,
            DE,
            DE_PERCEPTIBLE,
            mr.4,
            SRC_BASELINE,
            format!(
                "★★★ THIS ROW PASSING IS THE FINDING. max dE2000 between the K-only build and \
                 what iccce actually produces, both rendered through the destination's own A2B1: \
                 {:.6}. That is {:.0}x INSIDE the perceptibility anchor - the two builds are \
                 indistinguishable. The SAME comparison in device units is {:.6}, {:.0}x the \
                 anchor's numerical value in its own (incommensurable) units. A conformance \
                 suite that graded black preservation in dE would pass this configuration and \
                 report nothing, which is why every preservation row in this module is in \
                 DEVICE units",
                mr.4,
                if mr.4 > 0.0 {
                    1.0 / mr.4
                } else {
                    f64::INFINITY
                },
                mr.1,
                mr.1
            ),
        )
        .with_separation(Separation::against_distance(
            "the same two ink builds compared in NORMALISED DEVICE UNITS instead of dE2000 — the \
             quantity the feature must move, and the one this row deliberately cannot see",
            mr.1,
            mr.1,
            SepUnits::Other("normalised device units (0..1), not dE2000"),
        )),
    );

    let tol = pcs_quantum_tolerance(x.sensitivity);
    out.push(
        Record::graded(
            A_ROWS[4].0,
            CC,
            DEV,
            tol,
            x.vs_oracle,
            SRC_BASELINE,
            format!(
                "The PRECONDITION for reading §A at all: iccce's colorimetric answer on this \
                 ramp agrees with the pinned lcms2 to {:.6e} device, against a bound of {:.6e} \
                 computed at run time as the destination B2A's response to one 16-bit PCS \
                 quantum ({:.6e}) plus the two print floors. The contamination §A reports is \
                 therefore the PROFILE's, not iccce's",
                x.vs_oracle, tol.value, x.sensitivity
            ),
        )
        .with_separation(Separation::against_distance(
            "the contamination is iccce's own defect rather than the destination profile's \
             separation — under which this row would have observed the whole contamination, \
             because lcms2 would not have produced it",
            x.oracle_chromatic,
            x.oracle_chromatic,
            SepUnits::SameAsMetric,
        )),
    );

    for (i, (intent, chroma, tac, dk, de)) in x.per_intent.iter().enumerate().skip(1) {
        out.push(
            Record::graded(
                A_ROWS[4 + i].0,
                CC,
                DEV,
                REPORTED,
                *chroma,
                SRC_BASELINE,
                format!(
                    "The same ramp at {}: max chromatic ink {:.6}, max TAC {:.6}, max |K_out-K_in| \
                 {:.6}, max dE2000 from the K-only build {:.6}. ★ The saturation row is the one \
                 to read twice - on THIS vendor's profiles it is nearly K-only already, and §B \
                 shows on five other destinations that that is not a general property",
                    intent.name(),
                    chroma,
                    tac,
                    dk,
                    de
                ),
            )
            .with_separation(Separation::none(
                "considered: this is the SAME observable as the media-relative baseline row at a \
             different ICC intent, and the rival candidate for all four is the K-preserving \
             answer, stated once on rows passk/A/.../black-channel-reduction and \
             passk/E/k-only-in-implies-k-only-out. Restating it here would inflate the \
             discriminating count without adding a second reading of anything",
            )),
        );
    }
    out
}

const SRC_SWEEP: &str = "Pass K §B — iccce (shipped binary) into six REAL CMYK destinations from \
    the Ghent v5.0 corpus, licensed and uncommittable. The population is named in the module \
    header; no synthetic profile can join it, because a synthetic B2A this project authored \
    would be K-only by construction";

fn sweep_records(rows: &[SweepRow]) -> Vec<Record> {
    let mut out = Vec::new();
    #[expect(
        clippy::cast_precision_loss,
        reason = "a population count of 6 is exact in f64"
    )]
    let holds = rows
        .iter()
        .filter(|r| r.saturation <= NEARLY_K_ONLY)
        .count() as f64;
    let names: Vec<&str> = rows
        .iter()
        .filter(|r| r.saturation <= NEARLY_K_ONLY)
        .map(|r| r.name)
        .collect();
    out.push(
        Record::graded(
            B_ROWS[0].0,
            CC,
            CNT,
            SHORTCUT_SATURATION,
            holds,
            SRC_SWEEP,
            format!(
                "★★ A REFUTATION ROW (see the module header for the pattern). Of {} real CMYK \
                 destinations, {} have a saturation B2A that is already K-only on the neutral \
                 axis (max chromatic ink <= {:.0e}): {}. The other {} do not, so 'use the \
                 saturation intent instead of building black preservation' is refuted BY \
                 MEASUREMENT. Three of the six alias B2A0 = B2A2, so their saturation answer IS \
                 their perceptual answer - Pass G's vendor-specific intent-aliasing finding, \
                 reappearing as the reason the shortcut fails. ★ And where it does work it is \
                 not free: the ECI saturation build sits up to {:.4} dE2000 from the K-only \
                 build",
                rows.len(),
                names.len(),
                NEARLY_K_ONLY,
                if names.is_empty() {
                    "none".to_string()
                } else {
                    names.join(", ")
                },
                rows.len() - names.len(),
                rows.iter()
                    .filter(|r| r.saturation <= NEARLY_K_ONLY)
                    .map(|r| r.saturation_de)
                    .fold(0.0_f64, f64::max)
            ),
        )
        .with_separation(Separation::none(
            "considered, and there is no rival CANDIDATE — the thing that looks like one is not. \
             ★ The alternative worth naming, 'the corpus had contained only ISO Coated v2, the \
             destination §A uses, under which the count would have been the whole population and \
             the shortcut would have read as sound', changes the POPULATION rather than the \
             reading of a fixed observation. That is a COVERAGE statement (§3.10.8), and stating \
             it as a separation made this row report BLIND for a property it does not have — \
             measured on the first run and corrected. The rule it generalises: a rival TOLERANCE \
             is not a rival candidate (Pass 4c), and a rival CORPUS is not one either",
        )),
    );
    for (i, r) in rows.iter().enumerate() {
        out.push(Record::graded(
            B_ROWS[1 + i].0,
            CC,
            DEV,
            REPORTED,
            r.saturation,
            SRC_SWEEP,
            format!(
                "{}: max chromatic ink on the K-only ramp — media-relative {:.6}, perceptual \
                 {:.6}, SATURATION {:.6} (the graded value), and the saturation build sits \
                 {:.4} dE2000 from the K-only build",
                r.name, r.media_relative, r.perceptual, r.saturation, r.saturation_de
            ),
        )
        .with_separation(Separation::none(
            "considered: a per-destination baseline has no rival reading. §B's CLAIM is carried \
             by the refutation row above, whose own separation statement explains why a rival \
             CORPUS is not a rival candidate",
        )));
    }
    out
}

const SRC_GRAY: &str = "Pass K §C — LEG I is iccce's: a gray ICC profile through the PCS into \
    ISO Coated v2 300% (ECI), measured by running the shipped binary. LEG P is ISO 32000-1 \
    §10.3.3's DEVICE-SPACE rule (c=m=y=0, k=1-gray), computed here as arithmetic with no ICC \
    machinery in it, because that is what it is. ★ Whether the GWG requirement is discharged on \
    Leg P (in which case it is the PDF consumer's and not this project's) is OPEN and is being \
    settled by a separate icc-spec-librarian dispatch; §C measures both legs so that the answer \
    changes which row is quoted, not which numbers exist";

fn gray_records(rows: &[GrayLeg]) -> Vec<Record> {
    let mut out = Vec::new();
    for (i, r) in rows.iter().enumerate() {
        out.push(Record::graded(
            C_ROWS[i * 2].0,
            CC,
            DEV,
            REPORTED,
            r.device_distance,
            SRC_GRAY,
            format!(
                "Source {}. max |Δ| device between LEG I's output and LEG P's (0,0,0,1-g) over a \
                 21-point gray ramp: {:.6}. ★ The two legs are made of completely different ink. \
                 REPORTED and not graded because Leg P is the PDF consumer's arithmetic and \
                 grading iccce against it would grade iccce against a rule it does not implement",
                r.name, r.device_distance
            ),
        )
        .with_separation(Separation::none(
            "considered: the two legs ARE the two candidates, and this row's observation is the \
             distance between them rather than a reading that could have gone another way. Which \
             leg the GWG requirement is discharged on is an OPEN boundary question and a \
             coverage statement (§3.10.8), not a separation",
        )));
        out.push(Record::graded(
            C_ROWS[i * 2 + 1].0,
            CC,
            DE,
            REPORTED,
            r.colorimetric_distance,
            SRC_GRAY,
            format!(
                "Source {}. max dE2000 between the two legs, both rendered through the \
                 destination's own A2B1: {:.6}; at the ramp midpoint g = 0.5, {:.6} (★ NOT a                  GWG patch value — patch 23.0's gray panel is 25 %, its CMYK panel 0/0/0/75;                  the earlier attribution here was false and the number is unaffected). Read \
                 this beside the device row for the same source: same colour, different ink, or \
                 different colour, depending ENTIRELY on which gray profile is in the loop",
                r.name, r.colorimetric_distance, r.at_half
            ),
        )
        .with_separation(Separation::none(
            "considered: as for the device row above, the two legs are the two candidates and \
             this row reports the distance between them",
        )));
    }
    #[expect(
        clippy::cast_precision_loss,
        reason = "a population count of 2 is exact in f64"
    )]
    let holds = rows
        .iter()
        .filter(|r| r.colorimetric_distance <= DE_PERCEPTIBLE.value)
        .count() as f64;
    out.push(
        Record::graded(
            C_ROWS[4].0,
            CC,
            CNT,
            SHORTCUT_GRAY_LEG,
            holds,
            SRC_GRAY,
            format!(
                "★★ A REFUTATION ROW. Of {} gray source profiles, {} put iccce's ICC leg within \
                 the {:.1} dE2000 perceptibility anchor of ISO 32000-1 §10.3.3's device rule. \
                 The agreeing one is the destination press's OWN black-ink gray profile, where \
                 agreement is close to a tautology ({:.4} dE2000); the disagreeing one is an \
                 ordinary gamma-2.2 gray at {:.4} dE2000. 'The two legs are interchangeable' is \
                 therefore refuted by measurement, and a suite carrying only the favourable \
                 fixture would have reported it as sound",
                rows.len(),
                holds,
                DE_PERCEPTIBLE.value,
                rows.first().map_or(f64::NAN, |r| r.colorimetric_distance),
                rows.get(1).map_or(f64::NAN, |r| r.colorimetric_distance),
            ),
        )
        .with_separation(Separation::none(
            "as for §B's refutation row: the alternative worth naming — 'only the press's own \
             black-ink gray profile had been in the corpus' — changes the POPULATION rather than \
             the reading of a fixed observation, so it is a coverage statement (§3.10.8) and not \
             a separation. ★ This row's power comes instead from the two rows above it, whose \
             observations on the SAME destination differ by more than an order of magnitude in \
             dE2000",
        )),
    );
    out
}

fn model_records(x: &OracleModel) -> Vec<Record> {
    let mut out = Vec::new();
    out.push(
        Record::graded(
            D_ROWS[0].0,
            OR,
            DEV,
            CLUT_CELL_MODEL,
            x.cell_model_residual,
            k_source(
                "and BOTH SIDES ARE lcms2: the observation is transicc's output and the \
                 expectation is a model read out of the pinned source (cmscnvrt.c \
                 BlackPreservingKOnlyIntents; cmspcs.c _cmsReasonableGridpointsByColorspace \
                 returns 17 for a 4-channel space). It says NOTHING about colour correctness \
                 and nothing about iccce; its job is to establish what a K-preserving answer \
                 looks like BEFORE iccce has one, and to go red if the pin's grid ever moves",
            ),
            format!(
                "33 samples spanning exactly one CLUT cell (C = 0 .. 1/16, M = Y = 0, K = 0.5). \
                 max |observed - linear blend of the cell's two endpoints| = {:.6e}. ★ The \
                 K-only region is therefore NOT 'the neutral axis' and NOT a snap rule: it is \
                 the C=M=Y=0 EDGE of a 17-node hypercube, and the answer decays LINEARLY to the \
                 ordinary colorimetric answer over exactly one cell. An implementation that \
                 snapped near-neutrals to K-only, or that jumped discontinuously off the axis, \
                 would differ from this by a measurable amount",
                x.cell_model_residual
            ),
        )
        .with_separation(Separation::against_distance(
            "the grid were 33 nodes rather than 17 (cmsFLAGS_HIGHRESPRECALC gives 23; a caller \
             may also pack a grid size into the flags' high bits) — under which the cell would \
             be half as wide and the model evaluated at 1/16 would be wrong by the whole \
             endpoint distance",
            0.0,
            x.cell_endpoint_distance,
            SepUnits::SameAsMetric,
        )),
    );
    out.push(Record::graded(
        D_ROWS[1].0,
        OR,
        DEV,
        EXACT_ZERO,
        x.coincide_at_cell,
        k_source("both sides lcms2; see the preceding row's source"),
        format!(
            "At C = 1/16 exactly the K-only answer and the plain colorimetric answer differ by \
             {:.6e}. Zero is required, not tolerated: that sample is a CLUT NODE whose K-only \
             corner carries weight zero, so the two answers are the SAME table entry. A non-zero \
             value here means the grid is not 17 nodes and every model statement in this module \
             is stale",
            x.coincide_at_cell
        ),
    )
    .with_separation(Separation::none(
        "considered: the rival worth naming is 'the black-preserving CLUT is not 17 nodes', and \
         it is stated WITH a computed distance on the row above, which is the row that would \
         detect it. Restating it here would double-count one alternative",
    )));
    out.push(
        Record::graded(
            D_ROWS[2].0,
            OR,
            DEV,
            EXACT_ZERO,
            x.chromatic_on_ramp,
            k_source("both sides lcms2; this row is what EXACT_ZERO is calibrated against"),
            format!(
                "Over the whole 41-point K-only ramp the oracle's K-only answer carries {:.6e} \
             chromatic ink. This is the measurement that makes a tolerance of EXACTLY ZERO \
             defensible for row passk/E/k-only-in-implies-k-only-out: a real implementation of \
             this requirement returns the encoded value zero, not something small",
                x.chromatic_on_ramp
            ),
        )
        .with_separation(Separation::none(
            "considered, and there is genuinely no second candidate: the sampler either writes the \
         encoded value zero into the three chromatic channels or it does not, and there is no \
         reading of 'zero' that returns something else",
        )),
    );
    let worst = x.ktone.iter().map(|(_, v)| *v).fold(0.0_f64, f64::max);
    for (i, (name, v)) in x.ktone.iter().enumerate() {
        out.push(
            Record::graded(
                D_ROWS[3 + i].0,
                OR,
                DEV,
                REPORTED,
                *v,
                k_source(
                    "both sides lcms2. REPORTED for ever: _cmsBuildKToneCurve is a VENDOR \
                     construction and ICC.1 has no normative text about it, exactly the A27/A42 \
                     posture §3.7 takes for BPC",
                ),
                format!(
                    "{}: max |K_out - K_in| under the K-only intent = {:.6}. ★ K is RE-MAPPED, \
                     not copied: _cmsBuildKToneCurve builds a 4096-entry curve from the source's \
                     K-only lightness ramp against the destination's, so K_out = K_in only when \
                     the two presses' black inks agree. The largest re-mapping across these {} \
                     pairs is {:.6}, and THAT is the distance between a correct implementation \
                     and the plausible one that copies K through",
                    name,
                    v,
                    x.ktone.len(),
                    worst
                ),
            )
            .with_separation(Separation::against_distance(
                "K is copied through unchanged (K_out = K_in) instead of being mapped through \
                 the destination's black-ink tone curve — the plausible-but-wrong \
                 implementation",
                0.0,
                *v,
                SepUnits::SameAsMetric,
            )),
        );
    }
    out
}

const SRC_GATE: &str = "Pass K §E — the rows the FEATURE is graded by, and they were written \
    BEFORE it existed (2026-08-17) and repointed at it on 2026-08-18 without a bound moving. \
    iccce's numbers come from the shipped binary, driven with --preserve-black k-only-equal-lightness \
    where the row is about the preservation policy and BOTH ways where the row is about the \
    policy not being applied; the K-preserving reference comes from the non-ICC lcms2 intent 11 \
    and is labelled on every row that uses it";

fn gate_records(x: &FeatureGate) -> Vec<Record> {
    let mut out = Vec::new();
    out.push(
        Record::graded(
            E_ROWS[0].0,
            CC,
            DEV,
            EXACT_ZERO,
            x.chromatic,
            SRC_GATE,
            format!(
                "★★★ THE ROW THE FEATURE WAS BUILT AGAINST, REPOINTED 2026-08-18 AT THE SURFACE \
                 THAT NOW EXISTS. Observed max chromatic ink {:.6} against a required 0, over \
                 the 41-point K-only ramp. Surface driven: {}. ★ THE NUMBER MOVED BECAUSE THE \
                 CODE MOVED: this row read 0.705320 before black preservation existed and the \
                 tolerance has not been touched — it was and is exactly 0, written before \
                 anybody could see which value would be convenient. ★ This row still SKIPs in \
                 CI permanently (licensed corpus); its committed twin \
                 passk/F/synthetic-chromatic-neutral/k-only-in-implies-k-only-out is what runs \
                 there, and §3.10.8 records the coverage gap",
                x.chromatic, x.surface
            ),
        )
        .with_separation(Separation::against_distance(
            "a K-preserving path, which lcms2's non-ICC intent 11 shows returns exactly 0.000000 \
             chromatic ink at every point of this ramp. ★ The distance is taken from LCMS2'S \
             COLORIMETRIC ANSWER on this same ramp and not from iccce's observation, which is \
             why it did not collapse to zero on the run that turned the row green — the trap \
             Separation::against exists to name",
            0.0,
            x.oracle_chromatic,
            SepUnits::SameAsMetric,
        )),
    );
    out.push(
        Record::graded(
            E_ROWS[1].0,
            CC,
            DEV,
            REPORTED,
            x.k_vs_oracle,
            SRC_GATE,
            format!(
                "max |K_iccce - K_oracle| over the whole 41-point ramp = {:.6e}; over the {} \
                 points that are EXACT NODES of lcms2's own 17-node black-preserving CLUT it is \
                 {:.6e}. ★★ THE SPLIT IS THE FINDING AND IT IS WHY THIS ROW STAYS REPORTED. Off \
                 those nodes lcms2 is INTERPOLATING ITS OWN TABLE rather than evaluating its own \
                 construction, so the whole-ramp figure measures lcms2's grid, not either \
                 party's K mapping — the same node/off-node shape row \
                 passk/E/regression/off-node-envelope-is-NOT-zero reports at 32x for the \
                 colorimetric path. ★★ AND ON THIS PAIR THE ROW IS BLIND ANYWAY: source and \
                 destination are the same press, where the two published definitions coincide, \
                 so the named rival sits {:.6e} away — the SAME distance as the observation. A \
                 bound iccce passed here, 'copy K through' would pass too. The pair that CAN \
                 discriminate is graded at \
                 passk/E/cross-press/preserved-k-matches-the-oracle-at-its-own-clut-nodes; the \
                 exact answer this pair does admit is graded at \
                 passk/E/preserved-k-is-the-IDENTITY-on-a-same-profile-pair",
                x.k_vs_oracle, x.xp_node_points, x.k_vs_oracle_at_nodes, x.k_copy_rival
            ),
        )
        .with_separation(Separation::against_distance(
            "K is copied through unchanged instead of mapped through the destination's black-ink \
             tone curve — measured on this pair as the distance between the oracle's answer and \
             the input. ★★ Read the number: on a SAME-PRESS pair it equals the observation, so \
             this row cannot discriminate the rival and must not be quoted as though it had. ★ \
             The second named rival, Cholewo (2000)'s K_MIN/K_MAX ratio, is a NAMED REFUSAL in \
             crates/iccce-cmm at this commit rather than an implemented arm, so no distance to \
             it can be measured from any surface iccce exposes",
            x.k_copy_rival,
            x.k_copy_rival,
            SepUnits::SameAsMetric,
        )),
    );
    out.push(Record::graded(
        E_ROWS[2].0,
        CC,
        DEV,
        REPORTED,
        x.transition_width,
        SRC_GATE,
        format!(
            "The width of iccce's K-only region at K = 0.5, measured by walking C from 0 to 1/16 \
             and finding the last point at which chromatic ink is still exactly zero: {:.6}. ★★ \
             THAT ZERO NOW MEANS THE OPPOSITE OF WHAT IT MEANT BEFORE THE FEATURE, AND ONLY A \
             SECOND NUMBER CAN TELL THE TWO APART: chromatic ink at the C = 0 endpoint itself is \
             {:.6e}, so the K-only region EXISTS and is exactly one point wide. Before the \
             feature this row also read 0.000000 — because there was no K-only output at all. A \
             row whose observation is unchanged across the change it was written to detect is \
             the failure mode this pass's own memory calls a blinded row. ★★★ THE GAP TO THE \
             ORACLE IS NOW A REAL BEHAVIOURAL DIFFERENCE AND NOT AN ARTEFACT OF A MISSING \
             FEATURE. lcms2's K-only region is exactly one cell of its 17-node CLUT, {:.6}; \
             iccce's is zero by construction, because crates/iccce-cmm tests the three chromatic \
             channels against EXACT ZERO (matching lcms2's own In[0]==0 && In[1]==0 && In[2]==0) \
             while lcms2's width is a consequence of sampling that test into a CLUT and \
             interpolating it. ICC.1 says nothing about either (register entry A51, a closed \
             negative), so there is no text to settle it from and rule 7's remedy does not \
             apply: this is REPORTED, deliberately and permanently, and inventing a width so \
             that the section had a gate would invent the thing the pass exists to derive",
            x.transition_width, x.cell_zero_chromatic, CELL
        ),
    )
    .with_separation(Separation::against_distance(
        k_only_alt(
            "the oracle's own K-only region, which is exactly one cell of its 17-node \
             black-preserving CLUT",
        ),
        CELL,
        (CELL - x.transition_width).abs(),
        SepUnits::SameAsMetric,
    )));
    let tol = pcs_quantum_tolerance(x.sensitivity);
    out.push(
        Record::graded(
            E_ROWS[3].0,
            CC,
            DEV,
            tol,
            x.node_aligned,
            SRC_GATE,
            format!(
                "★ THE REGRESSION GUARD FOR THE NON-PRESERVED PATH, AND SINCE 2026-08-18 IT IS \
                 DRIVEN WITH --preserve-black {} SO THAT THE FEATURE IS ACTUALLY IN ITS LOOP. \
                 {} deterministic OFF-NEUTRAL points whose device coordinates are A2B CLUT nodes \
                 (j/15, grid 16), converted at media-relative: iccce agrees with lcms2's ORDINARY \
                 colorimetric answer to {:.6e} against a run-time bound of {:.6e} (PCS-quantum \
                 sensitivity {:.6e} + two print floors). ★★ WHY THE SURFACE HAD TO CHANGE: this \
                 row's pre-feature text said a leaking preservation path 'shows up here and \
                 nowhere else'. That was true only if the path were in the chain — and black \
                 preservation is OPT-IN and applied never by default, so a row driving the plain \
                 surface would have contained no preservation code to leak. Repointing it at the \
                 preserving surface is what makes the sentence true rather than aspirational. ★ \
                 An input with any chromatic ink is not K-only under the exact-zero rule, so a \
                 correct implementation must return bit-identical answers on both surfaces; row \
                 passk/E/regression/preservation-does-not-touch-a-non-qualifying-input grades \
                 that difference directly and at exactly zero, which is the sharper instrument. \
                 Read the two together: this row red and that one green means the ordinary path \
                 drifted; both red means the preservation path leaked",
                PRESERVE_POLICY, x.node_points, x.node_aligned, tol.value, x.sensitivity
            ),
        )
        .with_separation(Separation::against_distance(
            "a black-preservation path that leaked into the ordinary media-relative answer — for \
             which the nearest measured analogue is the off-node interpolation envelope this \
             section's control row reports, an order above this bound",
            x.arbitrary,
            x.arbitrary,
            SepUnits::SameAsMetric,
        )),
    );
    out.push(
        Record::graded(
            E_ROWS[4].0,
            CC,
            DEV,
            REPORTED,
            x.arbitrary,
            SRC_GATE,
            format!(
                "★★ THE CONTROL THAT EARNS THE PRECEDING ROW'S TIGHTNESS. The same comparison over \
             96 points that are NOT node-aligned, through the same preserving surface: {:.6e}, \
             which is {:.1}x the node-aligned figure. That difference IS the CLUT \
             interpolation-method envelope (NA-006) - the term Pass G's SWEEP_DEVICE had to \
             admit at 4e-3 and the reason a bound derived for one probe set is not a bound for \
             another. Without this row a reader could not tell a derived tolerance from a lucky \
             one. ★ The same 32x shape reappears in the K channel between iccce and lcms2's \
             black-preserving CLUT, at 351x, and is why \
             passk/E/preserved-k-value-vs-the-oracle-tone-curve reports two figures",
                x.arbitrary,
                if x.node_aligned > 0.0 {
                    x.arbitrary / x.node_aligned
                } else {
                    f64::INFINITY
                }
            ),
        )
        .with_separation(Separation::none(
            "considered: this row IS a control, and a control's rival is the row it controls. The \
         named alternative belongs to \
         passk/E/regression/node-aligned-off-neutral-agrees-with-lcms2 above, where it is stated \
         with a computed distance",
        )),
    );
    out.push(
        Record::graded(
            E_ROWS[5].0,
            CC,
            DEV,
            REPORTED,
            x.synthetic_chromatic,
            "Pass K §E — the COMMITTED synthetic fixture fixtures/synthetic/v2-cmyk-mft2-lab.icc, \
             which needs no licence and runs everywhere including CI",
            format!(
                "★★ WHY THIS PASS CANNOT RUN IN CI, stated as a number rather than as an excuse. \
                 The committed synthetic CMYK fixture's K-only ramp comes back with {:.6e} \
                 chromatic ink - its B2A0 is built by gen-profiles' `lab_to_cmyk_clut`, which \
                 emits [0,0,0,k] at every node, so it is K-only ALREADY and stays K-only whether \
                 or not black preservation exists. Its two candidate answers are the same \
                 number: ZERO-SEPARATION, the one state no tolerance can rescue. ★★ THE FEATURE \
                 HAS NOW LANDED AND THIS ROW IS UNCHANGED, WHICH IS THE PROOF OF THE POINT: it \
                 reads the same number with --preserve-black as without it, so a suite built on \
                 this fixture alone would have reported the identical figure before and after \
                 the work and graded nothing. ★ CLOSED 2026-08-17 by a NEW committed recipe, \
                 v2-cmyk-chromatic-neutral, whose B2A0 puts chromatic ink into neutrals by \
                 construction — see §F, whose rows grade the same predicate IN CI at a \
                 separation of 0.4207. THIS ROW IS NOT DELETED and its ZERO-SEPARATION verdict \
                 is not a defect to be tidied away: it is the measurement that says WHY the \
                 second fixture had to exist, and a future reader who repoints something at \
                 v2-cmyk-mft2-lab needs to find it",
                x.synthetic_chromatic
            ),
        )
        .with_separation(Separation::against_distance(
            "a K-preserving path — under which this fixture returns exactly the same numbers, \
             which is the point of the row",
            x.synthetic_chromatic,
            0.0,
            SepUnits::SameAsMetric,
        )),
    );
    out.push(
        Record::graded(
            E_ROWS[6].0,
            SELF,
            DEV,
            EXACT_ZERO,
            x.leak,
            SRC_GATE,
            format!(
                "★★★ THE LEAK GUARD, NEW 2026-08-18, AND THE SHARPEST INSTRUMENT IN THIS \
                 SECTION. The SAME {} probes ({} node-aligned + {} arbitrary + {} low-ink) \
                 run twice through the same function, differing in nothing but \
                 --preserve-black {}: max |on - off| = {:.6}, required exactly 0. \
                 ★★★ DETECTION FLOOR {:.6e} - the smallest widening of the qualifying test \
                 these probes can SEE, computed as min over probes of max(C, M, Y) and \
                 PRINTED rather than asserted. Its three parts: node-aligned {:.6e} (a j/15 \
                 grid fact, bounded below by 1/15 - which grid point a fixed seed reached \
                 is an OBSERVATION); arbitrary {:.6e} - ★★ SEED-DEPENDENT, construction \
                 bounds it only at 0.8/2^21 = 3.8e-7, so if this number moves someone \
                 re-seeded arbitrary_off_neutral; low-ink {:.6e} - STRUCTURAL, \
                 LOW_INK_DECADES' last entry, and what carries this row's reach today. \
                 ★★ WHY THE LOW-INK ARM EXISTS: until 2026-08-21 this row floor WAS the \
                 arbitrary set seed accident, seven-plus orders ABOVE the 1e-9 rival its \
                 own justification named, and an injected widening of t = 0.04 left the \
                 ENTIRE difftest suite green with the defect compiled in (DL-064). \
                 ★★ WHY EXACTLY ZERO IS THE RIGHT TOLERANCE AND NOT A TIGHT ONE. Every \
                 probe has at least one of C, M, Y strictly positive, so under the \
                 exact-zero qualifying rule crates/iccce-cmm documents (matching lcms2 In[0] \
                 == 0 && In[1] == 0 && In[2] == 0) NONE of them qualifies, the preservation \
                 branch returns None for every one, and the two invocations execute the \
                 identical arithmetic. This is not an agreement claim with an instrument \
                 error; it is the claim that a branch was not taken, and a branch is taken \
                 or it is not. ★ EVIDENCE CLASS IS SELF-CONSISTENCY, THE WEAKEST THIS SUITE \
                 EMITS - both sides are iccce and nothing outside this project is in the \
                 loop. It is used anyway because the predicate is EXACT where every \
                 available cross-check for the same question carries an interpolation \
                 envelope two orders wider, and a widening of the qualifying test to a \
                 tolerance - the single most plausible future change to this module - would \
                 move this row and might not move the cross-check at all",
                x.leak_points,
                x.node_points,
                x.node_points,
                x.leak_points - 2 * x.node_points,
                PRESERVE_POLICY,
                x.leak,
                x.leak_floor,
                x.leak_node_floor,
                x.leak_arb_floor,
                x.leak_low_floor
            ),
        )
        .with_separation(Separation::against_distance(
            "the qualifying test is widened from exact zero to a tolerance - the \
             alternative crates/iccce-cmm's own module doc names and rejects, and the \
             change a future contributor is most likely to make on the grounds that 1e-9 \
             of cyan 'is really K-only'. ★★★ THIS ROW NOW CATCHES THAT RIVAL AT ITS OWN \
             NAMED MAGNITUDE AND THREE DECADES BELOW IT: the low-ink probes reach 1e-12, \
             so any widening at or above 1e-12 puts probes on the qualifying side of the \
             test. ★ CORRECTED 2026-08-21 - this text previously read `under it the \
             arbitrary probe set smallest chromatic coordinate would begin to qualify`, \
             which was FALSE at the magnitude named in the same sentence: that \
             coordinate is 1.106777e-1, seven-plus orders ABOVE 1e-9, and a widening to \
             1e-9 would have moved nothing here (DL-064). When the branch does fire, the \
             difference becomes the whole distance between the preserved and \
             colorimetric answers, which this section's own baseline measures at \
             0.705320 on the K ramp. ★ The distance is bounded below by the off-node \
             envelope the control row reports, which is what a leak would at minimum \
             have to exceed to be visible to the cross-check instead",
            0.0,
            x.arbitrary,
            SepUnits::SameAsMetric,
        )),
    );
    out.push(
        Record::graded(
            E_ROWS[7].0,
            DE_KIND,
            DEV,
            PRINT_FLOOR,
            x.identity_k,
            "Pass K §E — an expectation from ALGEBRA, not from an implementation: on a \
             same-profile pair the equal-lightness construction is the identity. iccce's number \
             comes from the shipped binary; the oracle appears in this row only as the named \
             RIVAL, never as the expectation",
            format!(
                "★★★ THE ONE ROW IN THIS SECTION WHOSE EXPECTATION IS NOT AN IMPLEMENTATION'S \
                 OUTPUT. When the source and destination models are the same model, the \
                 destination K whose K-only patch has the same L* as the source's at K_in IS \
                 K_in — exactly, for any strictly monotonic L*(K) ramp, with no press, encoding \
                 or interpolation term in the statement. Observed max |K_out - K_in| over the \
                 41-point ramp = {:.6}, against one printed unit ({:.1e}). ★★★ AND IT IS A \
                 DISAGREEMENT WITH THE ORACLE IN ICCCE'S FAVOUR (project rule 7): lcms2's \
                 intent-11 answer on this same pair is {:.6e} away from K_in, because its K \
                 curve is sampled into a 17-node CLUT and read back, while iccce inverts the \
                 ramp directly. THE ORACLE IS WRONG HERE AND THE ENGINE IS RIGHT, by algebra \
                 rather than by preference, and the number is recorded rather than tolerated. ★ \
                 THE PREMISE IS THE FIXTURE'S: a flat stretch in this destination's L*(K) ramp \
                 (ink saturating) would make the inversion ill-posed, crates/iccce-cmm takes the \
                 LOWER K there by a documented choice, and the identity would fail FOR A CORRECT \
                 IMPLEMENTATION. A future red here is a question about the ramp before it is a \
                 question about the inverter",
                x.identity_k, PRINT_FLOOR.value, x.identity_oracle_rival
            ),
        )
        .with_separation(Separation::against_distance(
            "an implementation that reproduced the ORACLE's CLUT-quantised K instead of \
             evaluating the construction — i.e. one whose K came back through a 17-node table. \
             ★ The distance is lcms2's own measured departure from the algebraic answer on this \
             pair, so the rival is not hypothetical: it is a shipping CMM. ★★ The rival this row \
             does NOT have is 'K is copied through unchanged', which on a same-profile pair IS \
             the correct answer — ZERO-SEPARATION against that candidate, deliberately, and the \
             reason a second row exists on a cross-press pair",
            x.identity_oracle_rival,
            x.identity_oracle_rival,
            SepUnits::SameAsMetric,
        )),
    );
    let xp_tol = pcs_quantum_tolerance(x.xp_sensitivity);
    out.push(
        Record::graded(
            E_ROWS[8].0,
            CC,
            DEV,
            xp_tol,
            x.xp_k_at_nodes,
            k_source(
                "the CROSS-PRESS arm, added 2026-08-18: ISO Coated v2 300% (ECI) -> \
                 GWG_GenericCMYK, the pair on which the two published definitions of 'preserve \
                 the black' are furthest apart in this corpus",
            ),
            format!(
                "★★★ THE ONLY ROW ANYWHERE IN THIS SUITE THAT CAN TELL WHICH DEFINITION OF \
                 BLACK PRESERVATION ICCCE IMPLEMENTS, and that is the claim it supports — not \
                 'the K value is right', which no standard states. Source ISO Coated v2 300% \
                 (ECI), destination {}, at the {} ramp points that are EXACT NODES of lcms2's \
                 17-node black-preserving CLUT: |K_iccce - K_lcms2| = {:.6e} against a run-time \
                 bound of {:.6e} (the destination's own device response to one 16-bit PCS \
                 quantum at this ramp's PCS points, {:.6e}, plus two print floors). The rival \
                 answer — 'copy K through' — sits {:.6e} away at the same points, which is {:.0}x \
                 the observation. ★★ WHY ONLY AT THE NODES. Between them lcms2 interpolates its \
                 own 17-node table rather than evaluating its own construction, and the residual \
                 grows by two orders (up to 1.09e-2); a row graded over the whole ramp would be \
                 grading lcms2's grid density. This is the same node/off-node structure the \
                 colorimetric control row measures at 32x, in a different channel. ★ EVIDENCE \
                 CLASS: cross-check, and weaker than it looks — iccce implements lcms2's OWN \
                 construction by design, so agreement is expected and the row is not evidence \
                 that equal lightness is the right definition. It is evidence that iccce \
                 implements the definition it names, which is exactly what the mandatory \
                 --preserve-black policy argument promises a caller",
                x.xp_dst,
                x.xp_node_points,
                x.xp_k_at_nodes,
                xp_tol.value,
                x.xp_sensitivity,
                x.xp_copy_rival_at_nodes,
                if x.xp_k_at_nodes > 0.0 {
                    x.xp_copy_rival_at_nodes / x.xp_k_at_nodes
                } else {
                    f64::INFINITY
                }
            ),
        )
        .with_separation(Separation::against_distance(
            "K is copied through unchanged instead of mapped through the destination's black-ink \
             tone curve — the 'plausible-but-wrong implementation' this pass named before the \
             feature existed, and the value Cholewo's formula collapses to if its K_MIN/K_MAX \
             ranges turn out to be unconstrained on the pure-K axis. The distance is measured \
             from the ORACLE's answer and the ramp's own input, never from iccce's output",
            x.xp_copy_rival_at_nodes,
            x.xp_copy_rival_at_nodes,
            SepUnits::SameAsMetric,
        )),
    );
    out
}

// ===========================================================================
// §F — the COMMITTED fixture on which the predicate has two answers
// ===========================================================================
//
// ★★★ WHAT §F IS FOR, AND WHY IT IS NOT PART OF §E.
//
// §E grades the black-preservation predicate on `ISO Coated v2 300% (ECI)`,
// which is licensed. Every §E row therefore **skips in CI, permanently**, and
// row `E1`'s deliberate red — the whole point of the pass — is visible only to
// somebody holding a corpus that cannot be committed. Row `E6` states the
// obvious remedy and why it fails: the committed synthetic CMYK fixture
// `v2-cmyk-mft2-lab.icc` has a `B2A0` that emits `[0, 0, 0, k]` at every node,
// so its K ramp is K-only *already* and the two candidate answers are one
// number — `ZERO-SEPARATION`, the one state no tolerance can rescue.
//
// §F is the closure. `fixtures/synthetic/v2-cmyk-chromatic-neutral.icc`
// (recipe `v2-cmyk-chromatic-neutral` in `tools/gen-profiles`) is a committed,
// unlicensed, byte-reproducible CMYK profile whose `B2A0` **separates a
// neutral into all four inks by construction**. On it the K-only predicate has
// two distinct answers, and §F grades them **in CI**.
//
// ★★ WHY EVERY §F EXPECTATION IS A `DerivedExpectation` AND NOT A CROSS-CHECK.
//
// `CLAUDE.md` rule 3: an expectation that came from the code under test
// detects change, not error. §F never asks iccce what the fixture does. It
// **reads the committed bytes** — with its own tag-table walk and its own
// `mft2` decoder, in [`Mft2Bytes`], not through `iccce-profile` — and
// evaluates the two CLUTs itself. The expectation is arithmetic on those bytes
// plus clause 10.10's stated element order and interpolation; no
// implementation's output enters it.
//
// That is the strongest claim available here. It is **not** ground truth: no
// standards body printed `0.420 7`, and if this project's reading of clause
// 10.10 is wrong then the fixture and the expectation are wrong *together* and
// agree perfectly. That is exactly the failure a **third** reading catches,
// which is why `F7` pairs the same probe set against lcms2 and is labelled
// `CrossCheck` — the pairing `Kind::DerivedExpectation`'s own documentation
// asks for.
//
// ★★ THE THREE CONSTRUCTIONS THAT MAKE THE ARITHMETIC EXACT.
//
// They live in the recipe and are restated here because a reader of a row
// should not have to open another crate to know why its bound is two orders
// below the loosest bound in this document family.
//
// 1. **Both CLUT models are affine — no cross terms.** Every conformant
//    interpolation (n-linear, tetrahedral, prism, lcms2's 4-D hybrid) returns
//    a convex combination of a cell's corners, and a convex combination of an
//    affine function's corner values *is* that function at the point. So the
//    harness's n-linear evaluation and a consumer's tetrahedral one are the
//    same number wherever the model is unclamped — and `F4`/`F7`'s probes are
//    all chosen unclamped, corners included.
// 2. **`B2A0` is `a*`/`b*`-INDEPENDENT across a three-node dead band about the
//    neutral axis.** `a* = 0` encodes to `8000h` = 32 768 while node 4 of a
//    9-node axis sits at 32 767,5, so **the neutral axis is not a node** — it
//    falls `1.5e-5` of a cell inside the cell `[4, 5]`. With nodes 3, 4 and 5
//    carrying identical values every convex combination of them is that value,
//    and the neutral column is exact for *any* scheme. `F1` grades that
//    property against the file rather than asserting it in prose.
// 3. **The `B2A0` darkness variable is the ENCODED `L*` fraction**, so the
//    model never clamps on the neutral column — legacy PCSLAB puts `L* = 100`
//    at `FF00h`, and a model defined on `1 − L*/100` would clamp at the top
//    node, inside the very cell the K ramp's white end lands in.
//
// ★ WHAT §F DELIBERATELY DOES NOT DECIDE — read before adding a row here.
//
// §E2 records an **open fork**: what `K` value a black-preserving path should
// emit. lcms2's `_cmsBuildKToneCurve` maps `K` by equal `L*`; Cholewo (2000)
// maps it by the `K_MIN`/`K_MAX` ratio. Two definitions, one name, and the
// choice is the operator's. **No §F row grades the `K` channel.** `F4` and
// `F7` compare the three chromatic channels only, and `F5`'s predicate —
// `C = M = Y = 0` in implies `C = M = Y = 0` out — is definitionally
// unambiguous and needs no answer to the K question. `E2` keeps its posture
// exactly: REPORTED, with both rivals named.

/// The committed fixture §F is built on. Generated by `tools/gen-profiles`
/// recipe `v2-cmyk-chromatic-neutral`; `gen-profiles verify` in CI proves the
/// bytes on disk are the recipe's.
const SYNTHETIC_SEPARATING: &str = "v2-cmyk-chromatic-neutral.icc";

/// The `B2A0` composite-gray slope the fixture is authored with: at encoded
/// darkness `d` the neutral separation lays down `C = M = Y = 0.60 d`.
///
/// ★ **This literal is an EXPECTATION, not a transcription.** `F2` grades the
/// committed bytes against it, so a recipe change that moved the slope turns
/// `F2` red instead of silently re-basing every number in this section. It is
/// the one place in §F where a constant is typed rather than read, and it is
/// typed precisely so that the two copies must agree.
const CN_GRAY_SLOPE: f64 = 0.60;

/// The `B2A0` skeleton-black slope, `K = 0.40 d` at neutral.
///
/// ★ Graded by `F2` **as a property of the file** — that the bytes carry the
/// authored table — and by nothing else. No §F row asserts that a
/// black-preserving consumer should return this, or any other, `K`. See the
/// section header's last paragraph.
const CN_K_SLOPE: f64 = 0.40;

/// The dead-band node indices of the `B2A0` `a*` and `b*` axes: the three
/// lines about the centre across which the table is authored `a*`/`b*`
/// independent. Nine nodes per axis, so the centre is 4.
const CN_DEAD_BAND: [usize; 3] = [3, 4, 5];

/// **The declared floor on the fixture's candidate separation** — the number
/// `F3` grades, and the reason §F exists at all.
///
/// A fixture whose two candidate answers coincide is `ZERO-SEPARATION`, and no
/// tolerance can rescue it: that is the defect `E6` reports on
/// `v2-cmyk-mft2-lab`. Replacing one such fixture with another would be the
/// same bug under a fresh filename, so the separation is not merely *reported*
/// here — it is **graded against a floor declared in advance**.
///
/// **Where `4e-2` comes from, and it is not the observation.** The loosest
/// device-space tolerance anything in this document family has ever justified
/// is Pass G's `SWEEP_DEVICE` at `4e-3` (`TOLERANCES.md` §3.9). A separation
/// **ten times** that cannot be straddled by any bound this project has
/// written or could plausibly write: whatever tolerance a future row carries,
/// the two candidate answers stay on opposite sides of it. The floor is
/// derived from the tolerance budget, not from what the fixture happens to
/// measure — which is `0.420 7`, an order above the floor again.
const SEPARATION_FLOOR: f64 = 4.0e-2;

// ---------------------------------------------------------------------------
// The harness's OWN mft2 reader — deliberately not iccce's parser
// ---------------------------------------------------------------------------

/// A `lut16Type` tag, decoded from raw profile bytes by this harness.
///
/// ## Why this exists when `iccce-profile` can already parse an `mft2`
///
/// §F's whole claim is that its expectations come from the fixture's bytes and
/// **not** from the implementation under test. Decoding the tag with iccce's
/// own parser would put iccce inside the derivation: a parser that read the
/// CLUT wrongly would produce an expectation wrong in the same way as the
/// observation, and every §F row would agree perfectly while measuring
/// nothing. The same argument Pass 4 makes for `SourcePipeline`, taken one
/// layer further down.
///
/// ## What it does and does not implement
///
/// Enough of clause 10.10 to evaluate this corpus's tags and no more:
///
/// * the 128-byte header, the `uInt32` tag count at 128, and 12-byte
///   `signature / offset / size` directory entries;
/// * `mft2`'s fixed prologue — signature, 4 reserved bytes, `inputChan`,
///   `outputChan`, `clutPoints`, a pad byte, a 3×3 `s15Fixed16` matrix,
///   `inputEnt` and `outputEnt` — 52 bytes before the tables;
/// * the input tables, the CLUT and the output tables as `uInt16` arrays.
///
/// **The input and output tables are read and their identity is asserted, not
/// applied.** Every `mft2` in this corpus carries the two-entry
/// `0000h, FFFFh` table, which clause 10.10 permits as the minimum and which
/// is exactly the identity with nothing to interpolate. [`Mft2Bytes::decode`]
/// **refuses** any other table rather than silently ignoring it — a fixture
/// that grew a real transfer table would otherwise be evaluated as though it
/// had not, which is the silent-wrong-answer shape this project exists to
/// avoid.
///
/// The 3×3 matrix is likewise read and asserted to be the identity. Clause
/// 10.10 permits a non-identity matrix only for a PCSXYZ input and neither tag
/// here has one.
#[derive(Debug, Clone)]
pub struct Mft2Bytes {
    pub input_chan: usize,
    pub output_chan: usize,
    pub points: usize,
    /// CLUT samples, **normalised to `0..1`** by `/65535`, in clause 10.10's
    /// order: first input channel varies least rapidly, last most rapidly.
    pub clut: Vec<f64>,
}

impl Mft2Bytes {
    /// Locate `sig` in the tag table and decode it.
    ///
    /// # Errors
    /// A string naming what was wrong, for [`Unavailable::Error`]: a fixture
    /// that is present but undecodable is a broken checkout, never a skip.
    pub fn decode(bytes: &[u8], sig: &[u8; 4]) -> Result<Mft2Bytes, String> {
        let u32_at = |o: usize| -> Result<usize, String> {
            bytes
                .get(o..o + 4)
                .map(|s| u32::from_be_bytes([s[0], s[1], s[2], s[3]]) as usize)
                .ok_or_else(|| format!("profile truncated at byte {o}"))
        };
        let u16_at = |o: usize| -> Result<u16, String> {
            bytes
                .get(o..o + 2)
                .map(|s| u16::from_be_bytes([s[0], s[1]]))
                .ok_or_else(|| format!("profile truncated at byte {o}"))
        };
        let count = u32_at(128)?;
        let mut found = None;
        for i in 0..count {
            let e = 132 + 12 * i;
            let s = bytes.get(e..e + 4).ok_or("tag table truncated")?;
            if s == sig {
                found = Some((u32_at(e + 4)?, u32_at(e + 8)?));
                break;
            }
        }
        let (off, size) =
            found.ok_or_else(|| format!("tag {} absent", String::from_utf8_lossy(sig)))?;
        if bytes.get(off..off + 4) != Some(b"mft2") {
            return Err(format!("tag {} is not mft2", String::from_utf8_lossy(sig)));
        }
        let input_chan = usize::from(bytes[off + 8]);
        let output_chan = usize::from(bytes[off + 9]);
        let points = usize::from(bytes[off + 10]);
        // Clause 10.10: the matrix shall be the identity unless the input is
        // PCSXYZ. Asserted rather than applied — see the type's doc comment.
        for (i, m) in (0..9).map(|i| (i, u32_at(off + 12 + 4 * i))) {
            let want = if i % 4 == 0 { 0x0001_0000 } else { 0 };
            if m? != want {
                return Err(format!(
                    "tag {} has a non-identity matrix; §F evaluates neither",
                    String::from_utf8_lossy(sig)
                ));
            }
        }
        let input_ent = usize::from(u16_at(off + 48)?);
        let output_ent = usize::from(u16_at(off + 50)?);
        let mut p = off + 52;
        let take = |n: usize, p: &mut usize| -> Result<Vec<u16>, String> {
            let mut v = Vec::with_capacity(n);
            for _ in 0..n {
                v.push(u16_at(*p)?);
                *p += 2;
            }
            Ok(v)
        };
        let it = take(input_chan * input_ent, &mut p)?;
        let clut_len = points.pow(u32::try_from(input_chan).unwrap_or(0)) * output_chan;
        let clut = take(clut_len, &mut p)?;
        let ot = take(output_chan * output_ent, &mut p)?;
        if p - off != size {
            return Err(format!(
                "tag {} declares {size} bytes, its tables consume {}",
                String::from_utf8_lossy(sig),
                p - off
            ));
        }
        // Refuse, never ignore: a non-trivial transfer table would change the
        // answer and this evaluator does not apply one.
        for (name, t, ent, chan) in [
            ("input", &it, input_ent, input_chan),
            ("output", &ot, output_ent, output_chan),
        ] {
            if ent != 2 || (0..chan).any(|c| t[c * 2] != 0x0000 || t[c * 2 + 1] != 0xFFFF) {
                return Err(format!(
                    "tag {}'s {name} tables are not the two-entry identity; §F's evaluator \
                     applies no transfer table and must not pretend otherwise",
                    String::from_utf8_lossy(sig)
                ));
            }
        }
        Ok(Mft2Bytes {
            input_chan,
            output_chan,
            points,
            clut: clut.into_iter().map(|v| f64::from(v) / 65_535.0).collect(),
        })
    }

    /// One CLUT node, by per-axis index, as `output_chan` values in `0..1`.
    ///
    /// # Panics
    /// If `idx` has the wrong length or an out-of-range component — a harness
    /// bug, not a fixture property, and one that must not be absorbed.
    #[must_use]
    pub fn node(&self, idx: &[usize]) -> Vec<f64> {
        assert_eq!(idx.len(), self.input_chan, "node index rank");
        let mut flat = 0usize;
        for &i in idx {
            assert!(i < self.points, "node index out of range");
            flat = flat * self.points + i;
        }
        self.clut[flat * self.output_chan..(flat + 1) * self.output_chan].to_vec()
    }

    /// **n-linear interpolation** of the CLUT at `x` (each component `0..1`).
    ///
    /// ★ The scheme is chosen for a reason that is worth stating: n-linear is
    /// the *only* scheme whose weights are unambiguous from clause 10.10 alone
    /// — tetrahedral, prism and lcms2's 4-D hybrid are vendor choices about
    /// how to split a cell. It does not matter here, because **§F's probes lie
    /// where the table is affine**, and every one of those schemes returns a
    /// convex combination of the cell's corners with weights summing to one,
    /// which reproduces an affine function exactly. That equivalence is the
    /// reason a bound of one 16-bit quantum is defensible; it would not be on
    /// a table with cross terms.
    /// [`Mft2Bytes::eval`] with the CLUT's **index order reversed** — the
    /// first input channel varying most rapidly instead of least.
    ///
    /// ★ **This is not a utility; it is a named rival, and it exists so that
    /// `F4` and `F7` can state a separation with a computed distance instead
    /// of a plausible sentence.** Clause 10.10 fixes the order — *"the
    /// dimension corresponding to the first input channel varies least
    /// rapidly … the last input channel varies most rapidly"* — and reading it
    /// backwards is the classic `mft2` misimplementation: the tag still parses,
    /// every length still checks out, and the profile silently produces a
    /// channel-permuted image. Evaluating the *same committed bytes* the wrong
    /// way round gives the distance such a consumer would sit at.
    #[must_use]
    pub fn eval_reversed(&self, x: &[f64]) -> Vec<f64> {
        let mut r: Vec<f64> = x.iter().take(self.input_chan).copied().collect();
        r.reverse();
        self.eval(&r)
    }

    #[must_use]
    pub fn eval(&self, x: &[f64]) -> Vec<f64> {
        let last = self.points - 1;
        let mut base = Vec::with_capacity(self.input_chan);
        let mut frac = Vec::with_capacity(self.input_chan);
        for &v in x.iter().take(self.input_chan) {
            let u = v.clamp(0.0, 1.0) * grid_f64(last);
            #[expect(
                clippy::cast_possible_truncation,
                clippy::cast_sign_loss,
                reason = "u is non-negative and bounded by `last`, which is a uInt8 grid count"
            )]
            let i0 = (u as usize).min(last.saturating_sub(1));
            base.push(i0);
            frac.push(u - grid_f64(i0));
        }
        let mut out = vec![0.0; self.output_chan];
        for corner in 0..(1usize << self.input_chan) {
            let mut w = 1.0;
            let mut idx = Vec::with_capacity(self.input_chan);
            for d in 0..self.input_chan {
                let hi = corner >> d & 1 == 1;
                w *= if hi { frac[d] } else { 1.0 - frac[d] };
                idx.push(base[d] + usize::from(hi));
            }
            if w == 0.0 {
                continue;
            }
            for (o, v) in out.iter_mut().zip(self.node(&idx)) {
                *o += w * v;
            }
        }
        out
    }
}

/// A CLUT grid count or node index as an `f64`, converted through `u32` so the
/// lossy-cast lint is answered by an arithmetic guarantee rather than silenced.
///
/// Every caller passes a `clutPoints` value or an index below it, and
/// `clutPoints` is a **single `uInt8` field** of `lut16Type` (clause 10.10), so
/// the value can never exceed 255 and `f64` represents it exactly. A grid this
/// function could not convert would be a malformed tag, and the `expect` says
/// so rather than assuming it.
///
/// # Panics
/// If `n` exceeds `u32::MAX` — unreachable for a `uInt8` grid, and a panic is
/// the right outcome for a harness that has lost track of what it decoded.
#[must_use]
fn grid_f64(n: usize) -> f64 {
    f64::from(u32::try_from(n).expect("a lut16Type grid index fits in u32; clutPoints is a uInt8"))
}

/// **§F's off-neutral probe set: 50 CHROMATIC GRAYS.**
///
/// Each point is `(c, 6c/7, 0.984 127c, k)` — the family for which the
/// fixture's `A2B0` returns `a* = b* = 0` **exactly**, because
/// `a* = −60c + 70m = 0` at `m = 6c/7` and
/// `b* = −50c − 45m + 90y = 0` at `y = (50 + 45·6/7)c/90`.
///
/// ★★ **WHY THIS SHAPE, AND WHY NOT THE K RAMP.** `F4` and `F7` are the rows
/// that must **stay green when black preservation lands** — they are what
/// makes `F5`'s red attributable. A row measured on the K ramp cannot do that
/// job: the day the feature ships, iccce's answer there *should* change, and
/// the row would go red for the right reason and look like a regression.
///
/// A chromatic gray has `C`, `M` and `Y` all strictly positive, so it is **not
/// K-only under any definition** and no black-preservation path may touch it —
/// while its PCS image sits exactly on the neutral axis, inside the
/// `a*`/`b*` dead band where the derived expectation is exact for any
/// interpolation scheme. It is the strongest available regression guard and
/// the reason `F4`'s expectation can be derived rather than cross-checked.
///
/// ★ **Every probe is clamp-free, corners included.** The largest darkness any
/// probe reaches is `0.661 c + 0.70 k ≤ 0.661 × 0.50 + 0.70 × 0.50 = 0.680`,
/// and `A2B0`'s only clamp is `L* ≥ 0` at darkness above `1`; on the `B2A0`
/// side the dead-band nodes carry only `0.60 d` and `0.40 d`, inside `0..1`
/// for every `d`. Affinity therefore holds over every cell the probes occupy,
/// which is the condition the bound needs and not merely at the probes
/// themselves.
#[must_use]
pub fn chromatic_gray_probes() -> Vec<[f64; 4]> {
    const M_OVER_C: f64 = 6.0 / 7.0;
    let y_over_c = (50.0 + 45.0 * M_OVER_C) / 90.0;
    let mut out = Vec::with_capacity(50);
    for i in 1..=10 {
        let c = f64::from(i) * 0.05;
        for j in 0..5 {
            out.push([c, M_OVER_C * c, y_over_c * c, f64::from(j) * 0.125]);
        }
    }
    out
}

/// **§F's transition probe**: `C = t/32 × 1/(N−1)`, `M = Y = 0`, `K = 0.5`,
/// spanning exactly one cell of the fixture's 5-node `A2B0` grid.
#[must_use]
fn separating_cell_ramp(points: usize) -> Vec<[f64; 4]> {
    let cell = 1.0 / grid_f64(points - 1);
    (0..=32)
        .map(|t| [f64::from(t) / 32.0 * cell, 0.0, 0.0, 0.5])
        .collect()
}

// ---------------------------------------------------------------------------
// §F's measurements
// ---------------------------------------------------------------------------

/// Everything §F read out of the committed file, with no implementation in it.
#[derive(Debug, Clone)]
pub struct Separating {
    /// **`F1`.** max spread, over the 4 channels and every `L*` node, between
    /// the dead-band nodes `(ai, bi) ∈ {3,4,5}²` and the centre node `(4,4)`.
    /// Zero is required.
    pub dead_band_spread: f64,
    /// **`F2`.** max `|decoded − authored|` over the dead-band nodes, all four
    /// channels, against `C = M = Y = 0.60 d`, `K = 0.40 d`.
    pub column_residual: f64,
    /// **`F3`.** the fixture's candidate separation: the max chromatic ink the
    /// *table itself* returns on the K-only ramp, computed by evaluating both
    /// CLUTs from the committed bytes.
    pub separation: f64,
    /// The derived round trip on the K-only ramp, `(C, M, Y, K)` per point.
    pub derived_ramp: Vec<[f64; 4]>,
    /// The derived round trip on the 50 chromatic grays.
    pub derived_gray: Vec<[f64; 4]>,
    /// **`F4`/`F7`'s named rival, computed.** The max chromatic distance
    /// between the derived round trip and the *same committed bytes* evaluated
    /// with clause 10.10's CLUT index order **reversed** in both legs. See
    /// [`Mft2Bytes::eval_reversed`].
    pub index_order_rival: f64,
    /// One cell of the fixture's `A2B0` grid, `1/(N−1)`, for `F6`'s rival.
    pub a2b_cell: f64,
    /// Grid sizes, printed so a row names the table it measured.
    pub a2b_points: usize,
    pub b2a_points: usize,
}

/// Read the committed fixture and derive everything §F expects from it.
///
/// **Needs neither the oracle nor the iccce binary** — deliberately. `F1`–`F3`
/// are properties of the file, and a corrupted fixture must be reported on a
/// machine that has built nothing.
///
/// # Errors
/// [`Unavailable::Error`] throughout: this fixture is committed and
/// unlicensed, so its absence or undecodability is a broken checkout and never
/// a skip.
fn analyse_separating() -> Result<Separating, Unavailable> {
    let path = need_synthetic(SYNTHETIC_SEPARATING)?;
    let bytes = std::fs::read(&path)
        .map_err(|e| Unavailable::Error(format!("cannot read {}: {e}", path.display())))?;
    let a2b = Mft2Bytes::decode(&bytes, b"A2B0").map_err(Unavailable::Error)?;
    let b2a = Mft2Bytes::decode(&bytes, b"B2A0").map_err(Unavailable::Error)?;

    // ★ The shape §F needs, asserted before anything indexes into it.
    //
    // Every row below reads node `(li, 4, 4)` and its dead-band neighbours,
    // which requires a `B2A0` grid of at least 6 nodes per axis; `Mft2Bytes::node`
    // would otherwise PANIC on a smaller one. A panic here would be the worst
    // available outcome — the suite dies instead of reporting, and a reader
    // learns nothing about which fixture was wrong. Anyone who repoints
    // `SYNTHETIC_SEPARATING` at another profile (the sibling
    // `v2-cmyk-mft2-lab` has a 3-node grid and is the obvious mistake) gets a
    // named error instead.
    let want = (4usize, 3usize, 3usize, 4usize);
    let got = (
        a2b.input_chan,
        a2b.output_chan,
        b2a.input_chan,
        b2a.output_chan,
    );
    if got != want {
        return Err(Unavailable::Error(format!(
            "{}: §F needs A2B0 4->3 and B2A0 3->4, found {got:?}",
            path.display()
        )));
    }
    let biggest = CN_DEAD_BAND.iter().copied().max().unwrap_or(0);
    if b2a.points <= biggest || a2b.points < 2 {
        return Err(Unavailable::Error(format!(
            "{}: §F reads B2A0 dead-band node index {biggest}, so the grid must exceed it — \
             found B2A0 {} nodes, A2B0 {} nodes. A fixture with a smaller grid is not this \
             fixture; §F must report that, never index past the table",
            path.display(),
            b2a.points,
            a2b.points
        )));
    }

    // F1 — the dead band carries one value, so the neutral axis is exact for
    // ANY interpolation scheme rather than only for n-linear.
    let mut dead_band_spread = 0.0_f64;
    for li in 0..b2a.points {
        let centre = b2a.node(&[li, 4, 4]);
        for ai in CN_DEAD_BAND {
            for bi in CN_DEAD_BAND {
                for (p, q) in b2a.node(&[li, ai, bi]).iter().zip(&centre) {
                    dead_band_spread = dead_band_spread.max((p - q).abs());
                }
            }
        }
    }

    // F2 — the bytes carry the authored separation, so quoting 0.60/0.40
    // anywhere in this section is a checked claim and not a remembered one.
    let mut column_residual = 0.0_f64;
    for li in 0..b2a.points {
        let d = 1.0 - grid_f64(li) / grid_f64(b2a.points - 1);
        let want = [
            CN_GRAY_SLOPE * d,
            CN_GRAY_SLOPE * d,
            CN_GRAY_SLOPE * d,
            CN_K_SLOPE * d,
        ];
        for ai in CN_DEAD_BAND {
            for bi in CN_DEAD_BAND {
                for (got, w) in b2a.node(&[li, ai, bi]).iter().zip(want) {
                    column_residual = column_residual.max((got - w).abs());
                }
            }
        }
    }

    // The derived round trip, both legs evaluated from the committed bytes.
    let round_trip = |p: &[f64; 4]| -> [f64; 4] {
        let lab = a2b.eval(p);
        let out = b2a.eval(&lab);
        [out[0], out[1], out[2], out[3]]
    };
    let derived_ramp: Vec<[f64; 4]> = k_ramp().iter().map(round_trip).collect();
    let derived_gray: Vec<[f64; 4]> = chromatic_gray_probes().iter().map(round_trip).collect();

    // The rival, evaluated rather than asserted: the same bytes read with
    // clause 10.10's index order reversed in BOTH legs.
    let reversed = |p: &[f64; 4]| -> [f64; 4] {
        let lab = a2b.eval_reversed(p);
        let out = b2a.eval_reversed(&lab);
        [out[0], out[1], out[2], out[3]]
    };
    let index_order_rival = chromatic_gray_probes()
        .iter()
        .map(reversed)
        .zip(&derived_gray)
        .flat_map(|(w, r)| (0..3).map(move |j| (w[j] - r[j]).abs()))
        .fold(0.0_f64, f64::max);

    Ok(Separating {
        index_order_rival,
        dead_band_spread,
        column_residual,
        separation: max_chromatic(&derived_ramp),
        derived_ramp,
        derived_gray,
        a2b_cell: 1.0 / grid_f64(a2b.points - 1),
        a2b_points: a2b.points,
        b2a_points: b2a.points,
    })
}

/// What iccce and lcms2 return on the same committed fixture.
#[derive(Debug, Clone)]
pub struct SeparatingRun {
    /// **`F4`.** max `|iccce − derived|` over the three chromatic channels of
    /// the 50 chromatic grays. `K` is excluded on purpose.
    ///
    /// **Measured through the PRESERVING surface** since 2026-08-18: a
    /// chromatic gray has `C`, `M` and `Y` all strictly positive, so no
    /// definition of K-only admits it and the answer must be unchanged. A
    /// guard that did not drive the flag would contain no preservation code to
    /// guard against.
    pub gray_vs_derived: f64,
    /// **`F5`.** max chromatic ink iccce returns on the K-only ramp, through
    /// the preserving surface.
    pub chromatic: f64,
    /// **`F6`.** the width, in device units of cyan, of iccce's K-only region
    /// at `K = 0.5` on this fixture.
    pub transition_width: f64,
    /// ★ **`F6`'s disambiguator** — chromatic ink at the cell ramp's `C = 0`
    /// endpoint. Without it a width of `0.000000` reads the same whether the
    /// K-only region is one point wide or does not exist. See
    /// [`FeatureGate::cell_zero_chromatic`].
    pub cell_zero_chromatic: f64,
    /// **`F7`.** max `|iccce − lcms2|` over the same three channels and the
    /// same 50 points.
    pub gray_vs_oracle: f64,
    /// ★★ **`F8`.** max `|Δ|` over all four channels of the chromatic grays
    /// between the run with `--preserve-black` and the run without it. The
    /// committed-fixture twin of `E7`, and the only leak guard that runs in CI.
    ///
    /// ★★★ **Since 2026-08-21 the probe set is 50 + 70**: the 50 chromatic
    /// grays (floor `5.000000e-2`) **plus [`low_ink_decade_probes`]** (floor
    /// `1.000000e-12`). Before that addition an injected widening of `t = 0.04`
    /// left this row — and every other row of the suite — green with the
    /// defect compiled in.
    pub leak: f64,
    /// ★★★ **`F8`'s DETECTION FLOOR, computed and printed every run.** See
    /// [`FeatureGate::leak_floor`]; this is the CI-resident twin.
    pub leak_floor: f64,
    /// ★ **The chromatic-gray set's floor alone**, `5.000000e-2`, structural in
    /// `chromatic_gray_probes`' own loop.
    pub leak_gray_floor: f64,
    /// ★ **The low-ink set's floor alone**, structural.
    pub leak_low_floor: f64,
    /// How many probes the leak comparison ran over.
    pub leak_points: usize,
    pub gray_points: usize,
}

fn analyse_separating_run(
    oracle: &Oracle,
    iccce: &Iccce,
    f: &Separating,
) -> Result<SeparatingRun, Unavailable> {
    let path = need_synthetic(SYNTHETIC_SEPARATING)?;
    let err = |e: DiffError| Unavailable::Error(e.to_string());
    let preserved = |r: &[Vec<f64>]| -> Result<Vec<[f64; 4]>, Unavailable> {
        as_cmyk(
            iccce
                .transform_rows_shaped_preserve_black(
                    &path,
                    &path,
                    Intent::RelativeColorimetric,
                    r,
                    4,
                    PRESERVE_POLICY,
                )
                .map_err(err)?,
        )
    };

    let gray = chromatic_gray_probes();
    let gray_rows: Vec<Vec<f64>> = gray.iter().map(|r| r.to_vec()).collect();
    let mine = preserved(&gray_rows)?;
    let mine_plain = as_cmyk(
        iccce
            .transform_rows_shaped(&path, &path, Intent::RelativeColorimetric, &gray_rows, 4)
            .map_err(err)?,
    )?;

    // ★★★ THE LOW-INK ARM, 2026-08-21 — the CI-resident half of the fix.
    // `F8`'s floor was `5.0e-2`, and an injected widening of `t = 0.04` left
    // the WHOLE difftest suite green with the defect compiled in. These 70
    // probes take the floor to `1e-12`. They feed the leak comparison ONLY:
    // `F4` and `F7` keep the 50 chromatic grays, because their expectations
    // (the derived table, and lcms2) are stated for those points.
    let low = low_ink_decade_probes();
    let low_rows: Vec<Vec<f64>> = low.iter().map(|r| r.to_vec()).collect();
    let low_mine = preserved(&low_rows)?;
    let low_plain = as_cmyk(
        iccce
            .transform_rows_shaped(&path, &path, Intent::RelativeColorimetric, &low_rows, 4)
            .map_err(err)?,
    )?;
    let leak = max_dev(&mine, &mine_plain).max(max_dev(&low_mine, &low_plain));
    let leak_gray_floor = probe_floor(&gray);
    let leak_low_floor = probe_floor(&low);
    let leak_floor = leak_gray_floor.min(leak_low_floor);
    let leak_points = gray.len() + low.len();
    let chromatic_max = |a: &[[f64; 4]], b: &[[f64; 4]]| -> f64 {
        a.iter()
            .zip(b)
            .flat_map(|(p, q)| (0..3).map(move |j| (p[j] - q[j]).abs()))
            .fold(0.0_f64, f64::max)
    };
    let gray_vs_derived = chromatic_max(&mine, &f.derived_gray);

    let req = Request {
        input: Space::profile(&path),
        output: Space::profile(&path),
        intent: Intent::RelativeColorimetric,
        precalc: Precalc::Exact,
        bpc: Bpc::Off,
        values: gray.iter().flatten().map(|v| v * 100.0).collect(),
    };
    let theirs: Vec<[f64; 4]> = as_cmyk(oracle.convert_batch_shaped(&req, 4, 4).map_err(err)?)?
        .into_iter()
        .map(|r| [r[0] / 100.0, r[1] / 100.0, r[2] / 100.0, r[3] / 100.0])
        .collect();
    let gray_vs_oracle = chromatic_max(&mine, &theirs);

    let ramp = k_ramp();
    let ramp_rows: Vec<Vec<f64>> = ramp.iter().map(|r| r.to_vec()).collect();
    let ramp_out = preserved(&ramp_rows)?;

    let cell = separating_cell_ramp(f.a2b_points);
    let cell_rows: Vec<Vec<f64>> = cell.iter().map(|r| r.to_vec()).collect();
    let cell_out = preserved(&cell_rows)?;
    let mut transition_width = 0.0_f64;
    for (inp, out) in cell.iter().zip(&cell_out) {
        if out[0].max(out[1]).max(out[2]) == 0.0 {
            transition_width = inp[0];
        } else {
            break;
        }
    }
    let cell_zero_chromatic = cell_out
        .first()
        .map_or(f64::NAN, |r| r[0].max(r[1]).max(r[2]));

    Ok(SeparatingRun {
        gray_vs_derived,
        chromatic: max_chromatic(&ramp_out),
        transition_width,
        cell_zero_chromatic,
        gray_vs_oracle,
        leak,
        leak_floor,
        leak_gray_floor,
        leak_low_floor,
        leak_points,
        gray_points: gray.len(),
    })
}

// ---------------------------------------------------------------------------
// §F's tolerances
// ---------------------------------------------------------------------------

/// **Half a 16-bit quantum** — `F2`'s bound, and it is the encoding's own
/// arithmetic rather than a choice.
///
/// `F2` compares the committed `B2A0` samples against the model the recipe
/// says it wrote. The generator rounds each model value to the nearest
/// `uInt16`, and round-to-nearest is wrong by at most half a quantum. **The
/// extremal case is attained here** — the observed residual is exactly
/// `0.5/65 535` — which is not a coincidence and not a near miss: several
/// authored values (`0.525`, `0.450`, `0.375`, …) land precisely on a half
/// code. `Record::graded` admits `observed == tolerance` deliberately, so the
/// row passes at the encoding's own worst case and fails at anything a
/// *changed model* would produce.
pub const HALF_QUANTUM: Tolerance = Tolerance::new(
    0.5 / 65_535.0,
    "HALF a 16-bit quantum — the largest error round-to-nearest can introduce when the \
     generator encodes a model value into a uInt16 CLUT sample. ARITHMETIC, from the encoding; \
     nothing perceptual enters it and §2's 1.0 dE anchor is irrelevant to it. The observed \
     residual EQUALS this bound because several authored values land exactly on a half code, \
     which is the worst case the encoding admits and not a tuned fit",
);

/// **One 16-bit quantum** — `F4`'s bound, counted rather than chosen.
///
/// `F4` compares `iccce transform`'s answer against this harness's own
/// evaluation of the *same* CLUT samples, so the samples' own quantisation
/// cancels between the two sides and only the differences in how the two
/// evaluate the chain survive. Counted, in quanta of `2⁻¹⁶`:
///
/// | term | size |
/// |---|---|
/// | the PCS handed between the `A2B` and `B2A` legs is a 16-bit encoded Lab, so the two evaluations can differ by half a quantum there — carried into device output by the `B2A` model's `0.60` gray slope | `0.30` |
/// | a consumer may carry CLUT indices in 16-bit fixed point where this harness uses `f64` | `0.50` |
/// | `iccce transform` prints six decimals | `0.07` |
/// | **counted sum** | **`0.87`** |
///
/// The bound is **the next whole quantum above the counted sum**, because a
/// bound stated in fractional quanta claims a precision the encoding does not
/// have. Observed at the time of writing: **`4.965 520e-7`, `0.033` of a
/// quantum** — which incidentally says that neither side requantises the PCS,
/// so the first term above is an allowance for a *conformant consumer that
/// does*, not a description of this one. Stating it that way matters: a bound
/// whose largest term is not exercised is a bound whose margin is untested,
/// and a future consumer that does requantise must not read `0.033` as
/// headroom it can spend.
///
/// ★ **What licenses the counting** is that the fixture's models are affine
/// and every `F4` probe lies where they are unclamped, corners included — so
/// the *interpolation scheme* contributes nothing at all, and the terms above
/// are the complete list. On a table with cross terms the scheme's own
/// envelope would dominate and this bound would be indefensible; Pass 4's
/// `NA-006` prices that term at up to `1.57 ΔE2000` on a real CMYK `A2B`.
pub const TABLE_INTERPOLATION: Tolerance = Tolerance::new(
    1.0 / 65_535.0,
    "ONE 16-bit quantum, COUNTED: the PCS exchanged between the two mft2 legs is a 16-bit \
     encoded Lab (half a quantum, carried by the B2A model's 0.60 gray slope = 0.30), plus a \
     consumer's 16-bit fixed-point CLUT indexing where this harness uses f64 (0.50), plus \
     iccce transform's six printed decimals (0.07) — 0.87 quanta, rounded UP to the next whole \
     quantum because a bound in fractional quanta claims a precision the encoding does not \
     have. The interpolation SCHEME contributes zero because the fixture's models are affine \
     and every probe is unclamped with its cell corners; that is a property of THIS fixture and \
     must not be quoted for another",
);

/// **Two 16-bit quanta** — `F7`'s bound, the same counting with lcms2's
/// pipeline in place of this harness's.
///
/// lcms2 evaluates an `mft2` chain through `cmsPipelineEval16`, so **each
/// stage's output is requantised**, not just the PCS handoff: two stages at
/// half a quantum is a full quantum before anything else. Add the same
/// `0.30` for the PCS carried by the gray slope, and `transicc`'s four printed
/// decimals of a percentage (`0.03`). Counted sum `1.33`; the bound is the
/// next whole quantum, `2 × 2⁻¹⁶`. Observed at the time of writing:
/// `1.40e-5`, `0.92` of a quantum.
pub const ORACLE_CHAIN: Tolerance = Tolerance::new(
    2.0 / 65_535.0,
    "TWO 16-bit quanta, COUNTED with lcms2's pipeline in place of the harness's: \
     cmsPipelineEval16 requantises EACH stage's output, so two mft2 stages contribute a full \
     quantum before anything else; plus the PCS handoff carried by the 0.60 gray slope (0.30), \
     plus transicc's four printed decimals of a percentage (0.03) — 1.33 quanta, rounded up to \
     the next whole quantum. As with F4 the interpolation scheme contributes zero ONLY because \
     this fixture's models are affine over every cell the probes occupy",
);

/// **`F3`'s gate: the declared separation floor must be MET**, and the row's
/// observed value is the *shortfall below it*.
///
/// The mechanism is the one `§B1` uses for a refutation: `Record::graded`
/// compares `observed ≤ tolerance`, so a **lower** bound on a quantity is
/// expressed by grading its shortfall at zero. `observed =
/// max(0, floor − separation)`, which is `0` while the floor holds and rises
/// the moment it does not.
///
/// ★ **Why this is a graded row and not merely the `Separation` field.**
/// Every row already carries a candidate separation and the classifier already
/// prints `ZERO-SEPARATION` when it collapses — but a flag is never a failure
/// (that is deliberate, so that stating separations does not become
/// dangerous). `F3` is the one place where the collapse itself is graded,
/// because §F exists **because** a fixture collapsed, and a replacement that
/// quietly collapsed again would be the same defect under a fresh filename.
pub const SEPARATION_FLOOR_MET: Tolerance = Tolerance::new(
    0.0,
    "ZERO SHORTFALL against a separation floor declared in advance. The observed value is \
     max(0, floor - separation), so the row passes exactly while the fixture's two candidate \
     answers stay at least the declared distance apart and fails the moment they do not. The \
     floor itself is 10x Pass G's SWEEP_DEVICE (4e-3), the loosest device-space tolerance this \
     document family has ever justified — derived from the tolerance budget, NOT from what this \
     fixture happens to measure",
);

// ---------------------------------------------------------------------------
// §F's records
// ---------------------------------------------------------------------------

const SRC_SEP_FILE: &str = "Pass K §F — fixtures/synthetic/v2-cmyk-chromatic-neutral.icc, read as \
    RAW BYTES by this harness's own mft2 decoder (never through iccce-profile) and evaluated by \
    its own n-linear interpolator. No implementation's output enters these expectations; no \
    licence is needed; they run in CI";

const SRC_SEP_RUN: &str = "Pass K §F — the same committed fixture, driven through the SHIPPED \n    iccce binary as a subprocess and, where the row says so, through the pinned lcms2. Since \n    2026-08-18 every transform row is driven with --preserve-black k-only-equal-lightness, and \n    one row is driven BOTH ways so that the difference itself can be graded. The expectation is \n    the harness's own evaluation of the fixture's bytes; the oracle appears on exactly one row \n    and is labelled there";

fn separating_file_records(x: &Separating) -> Vec<Record> {
    let mut out = Vec::new();
    out.push(
        Record::graded(
            F_FILE_ROWS[0].0,
            DE_KIND,
            DEV,
            EXACT_ZERO,
            x.dead_band_spread,
            SRC_SEP_FILE,
            format!(
                "★★ THE ROW THAT MAKES EVERY OTHER §F NUMBER EXACT. Over all {} L* nodes, the \
                 9 dead-band nodes (a*,b* indices {:?}) differ from the centre node (4,4) by \
                 {:.6e} in every one of the four channels. ZERO IS REQUIRED, not tolerated: a* \
                 = 0 encodes to 8000h = 32768 while node 4 of a {}-node axis sits at 32767.5, so \
                 THE NEUTRAL AXIS IS NOT A NODE — it falls 1.5e-5 of a cell inside the cell \
                 [4,5]. Only because those nodes carry ONE value is every convex combination of \
                 them that value, and only then is the neutral column exact for tetrahedral, \
                 prism and lcms2's 4-D hybrid as well as for n-linear. A non-zero value here \
                 means the recipe's dead band has gone and F4's one-quantum bound is no longer \
                 defensible",
                x.b2a_points, CN_DEAD_BAND, x.dead_band_spread, x.b2a_points
            ),
        )
        .with_separation(Separation::none(
            "considered, and there is genuinely no second candidate: nine node values either \
             are one number or they are not, and no reading of 'identical' returns something \
             else",
        )),
    );
    out.push(
        Record::graded(
            F_FILE_ROWS[1].0,
            DE_KIND,
            DEV,
            HALF_QUANTUM,
            x.column_residual,
            SRC_SEP_FILE,
            format!(
                "The committed B2A0's dead-band nodes carry C = M = Y = {:.2} d and K = {:.2} d, \
                 where d is the node's ENCODED L* darkness 1 - li/{}: max |decoded - authored| \
                 = {:.6e} against half a 16-bit quantum ({:.6e}). ★ THIS IS WHAT LICENSES \
                 QUOTING {:.2} ANYWHERE ELSE IN §F. Without it the slope would be a number \
                 remembered from a recipe in another crate, which is exactly the shape of the \
                 stale literals §3.5.8.6 exists to prevent. ★ The K column is graded HERE, as a \
                 property of the FILE, and nowhere else: what K a black-preserving path should \
                 emit is an open fork (E2) and no §F row presupposes an answer",
                CN_GRAY_SLOPE,
                CN_K_SLOPE,
                x.b2a_points - 1,
                x.column_residual,
                HALF_QUANTUM.value,
                CN_GRAY_SLOPE
            ),
        )
        .with_separation(Separation::against_distance(
            "the generator TRUNCATED rather than rounded when encoding the model into uInt16 \
             samples — the plausible-but-wrong encoding, which would put the residual at a \
             whole quantum instead of half of one",
            1.0 / 65_535.0,
            (1.0 / 65_535.0 - x.column_residual).abs(),
            SepUnits::SameAsMetric,
        )),
    );
    let shortfall = (SEPARATION_FLOOR - x.separation).max(0.0);
    out.push(
        Record::graded(
            F_FILE_ROWS[2].0,
            DE_KIND,
            DEV,
            SEPARATION_FLOOR_MET,
            shortfall,
            SRC_SEP_FILE,
            format!(
                "★★★ THE ROW THAT SAYS THIS FIXTURE IS NOT THE LAST ONE AGAIN. Evaluating both \
                 CLUTs from the committed bytes, the K-only ramp comes back carrying {:.6} of \
                 chromatic ink — that is what the TABLE returns, with no implementation in it. A \
                 black-preserving path returns 0. THE TWO CANDIDATE ANSWERS ARE {:.6} APART, \
                 against a floor of {:.2e} declared in advance; shortfall {:.6}. On the sibling \
                 v2-cmyk-mft2-lab that distance is 0.000000 and row \
                 passk/E/synthetic-cmyk-fixture-is-ZERO-SEPARATION-for-this-subject reports it. \
                 A replacement fixture that quietly collapsed again would be the same defect \
                 under a fresh filename, so the collapse is GRADED here and not merely flagged \
                 by the classifier",
                x.separation, x.separation, SEPARATION_FLOOR, shortfall
            ),
        )
        .with_separation(Separation::against_distance(
            "the fixture had been built like its sibling v2-cmyk-mft2-lab, whose B2A0 emits \
             [0,0,0,k] at every node — under which this row's separation would be exactly 0.0 \
             and its shortfall the whole declared floor",
            0.0,
            x.separation,
            SepUnits::SameAsMetric,
        )),
    );
    out
}

fn separating_run_records(f: &Separating, x: &SeparatingRun) -> Vec<Record> {
    let x_index_order_rival = f.index_order_rival;
    let mut out = Vec::new();
    out.push(
        Record::graded(
            F_XFORM_ROWS[0].0,
            DE_KIND,
            DEV,
            TABLE_INTERPOLATION,
            x.gray_vs_derived,
            SRC_SEP_RUN,
            format!(
                "★★ THE ROW THAT MAKES F5's VERDICT ATTRIBUTABLE, AND IT HAS SURVIVED THE \
                 FEATURE. {} CHROMATIC GRAYS - (c, 6c/7, 0.984127c, k), the family for which \
                 this A2B0 returns a* = b* = 0 exactly - converted at media-relative WITH \
                 --preserve-black {}: iccce agrees with the harness's own evaluation of the same \
                 CLUT bytes to {:.6e} against a counted bound of {:.6e}. ★ REPOINTED AT THE \
                 PRESERVING SURFACE 2026-08-18, for the same reason its §E counterpart was: a \
                 guard against a leak has to have the leaking code in its loop, and black \
                 preservation is opt-in and applied never by default. A chromatic gray has C, M \
                 and Y all STRICTLY POSITIVE, so it is not K-only under any definition and no \
                 black-preservation path may touch it - which is why this guard survives the \
                 feature while a guard on the K ramp could not. If F5 disagreed with the fixture \
                 and THIS row were green, the disagreement would mean what it says; if both were \
                 red, the fault would be in reading the fixture. ★ Three chromatic channels \
                 only: the K channel is the open fork of E2",
                x.gray_points, PRESERVE_POLICY, x.gray_vs_derived, TABLE_INTERPOLATION.value
            ),
        )
        .with_separation(Separation::against_distance(
            "the CLUT index order is read BACKWARDS — first input channel varying most rapidly \
             instead of least, against clause 10.10's explicit statement. ★ The distance is not \
             asserted, it is EVALUATED: the harness reads the same committed bytes the wrong way \
             round in both legs and takes the difference over the same probes. ★★ A rival this \
             row does NOT have, and the reason is worth recording: 'iccce applied the general \
             PCSLAB encoding instead of the legacy one' (DL-005) is INVISIBLE here, because both \
             legs of the round trip would use it and this derivation works in encoded fractions \
             throughout — a SYMMETRIC misreading cancels exactly. That rival belongs to a row \
             that decodes the PCS, and this row does not",
            0.0,
            x_index_order_rival,
            SepUnits::SameAsMetric,
        )),
    );
    out.push(
        Record::graded(
            F_XFORM_ROWS[1].0,
            DE_KIND,
            DEV,
            EXACT_ZERO,
            x.chromatic,
            SRC_SEP_RUN,
            format!(
                "★★★ THE COMMITTED TWIN OF THE PASS'S HEADLINE ROW, REPOINTED 2026-08-18 AT \
                 --preserve-black {} — AND UNLIKE passk/E/k-only-in-implies-k-only-out THIS ONE \
                 RUNS IN CI. Observed max chromatic ink {:.6} on the 41-point K-only ramp \
                 against a required 0, through the COMMITTED fixture, which needs no licence and \
                 is byte-verified by gen-profiles. It read 0.420705 before the feature existed \
                 and the tolerance has not moved: it was and is exactly 0. ★ The Ghent row is \
                 NOT repointed here and its figure is printed on ITS OWN row and deliberately \
                 not restated: §F adds reach, it does not launder anything. ★★ This row's \
                 separation is taken from the FIXTURE'S OWN TABLE ({:.6}, row \
                 .../separation-is-above-the-declared-floor) and not from iccce's observation, \
                 which is precisely why it did not collapse to zero on the run that turned the \
                 row green — the injection experiment of 2026-08-17 showed that a collapsed \
                 fixture does not merely fail to inform, it MANUFACTURES this row's green",
                PRESERVE_POLICY, x.chromatic, f.separation
            ),
        )
        .with_separation(Separation::against_distance(
            "a black-preserving path, which returns exactly 0.000000 chromatic ink. The \
             distance is the fixture's OWN table evaluated from its committed bytes, so it is a \
             property of the file and not of any implementation",
            0.0,
            f.separation,
            SepUnits::SameAsMetric,
        )),
    );
    out.push(
        Record::graded(
            F_XFORM_ROWS[2].0,
            DE_KIND,
            DEV,
            REPORTED,
            x.transition_width,
            SRC_SEP_RUN,
            format!(
                "The width of iccce's K-only region at K = 0.5 on the committed fixture, \
                 measured by walking C from 0 to one A2B0 cell ({:.6}, grid {}) and finding the \
                 last point at which chromatic ink is still exactly zero: {:.6}. ★★ THAT ZERO \
                 NOW MEANS THE OPPOSITE OF WHAT IT MEANT BEFORE THE FEATURE, and the second \
                 number is what distinguishes them: chromatic ink at the C = 0 endpoint itself \
                 is {:.6e}, so the K-only region EXISTS and is exactly one point wide, where \
                 before it did not exist at all. ★★★ THE GAP TO THE ORACLE IS A REAL \
                 BEHAVIOURAL DIFFERENCE BETWEEN ICCCE AND LCMS2 AND NOT AN ARTEFACT OF A MISSING \
                 FEATURE: iccce's width is zero BY CONSTRUCTION because crates/iccce-cmm tests \
                 the three chromatic channels against exact zero (matching lcms2's own \
                 In[0]==0 && In[1]==0 && In[2]==0), while lcms2's own width of one CLUT cell is \
                 a consequence of sampling that same test into a 17-node table and interpolating \
                 it. ICC.1 contains no black-preservation construct at all (register entry A51, \
                 a CLOSED NEGATIVE), so there is no specification text to settle it from and \
                 rule 7's remedy does not apply. REPORTED, deliberately and permanently: \
                 inventing a width so that the section had a gate would invent the thing the \
                 pass exists to derive, and tuning iccce toward the oracle's width would be \
                 adopting a vendor's CLUT resolution as a colour requirement",
                f.a2b_cell, f.a2b_points, x.transition_width, x.cell_zero_chromatic
            ),
        )
        .with_separation(Separation::against_distance(
            "a K-only region one A2B0 CELL wide — the width derived from this fixture's own \
             grid, which is the shape lcms2's black-preserving CLUT has (row \
             passk/D/lcms2-intent-11/k-only-region-is-ONE-clut-cell-wide establishes it on the \
             oracle's 17-node table)",
            f.a2b_cell,
            (f.a2b_cell - x.transition_width).abs(),
            SepUnits::SameAsMetric,
        )),
    );
    out.push(
        Record::graded(
            F_XFORM_ROWS[3].0,
            CC,
            DEV,
            ORACLE_CHAIN,
            x.gray_vs_oracle,
            SRC_SEP_RUN,
            format!(
                "★ THE THIRD READING. The same {} chromatic grays, iccce (with --preserve-black \
                 {}) against the pinned lcms2's ordinary colorimetric answer: {:.6e} against a \
                 counted bound of {:.6e}. This row is a CROSS-CHECK and the rest of §F is not, \
                 and the difference is the point: a derived expectation is defeated when the \
                 DERIVATION shares a misreading with the fixture, and both are this project's \
                 reading of clause 10.10. lcms2 is a third party that read the same clause \
                 independently. Agreement here is what stops §F from being a closed loop; it is \
                 NOT ground truth, and a disagreement would be a finding to settle from the \
                 specification text (rule 7), not a bound to widen. ★ Driven through the \
                 preserving surface since 2026-08-18 so that it also witnesses the feature not \
                 reaching an input that does not qualify — the exact form of which is the next \
                 row",
                x.gray_points, PRESERVE_POLICY, x.gray_vs_oracle, ORACLE_CHAIN.value
            ),
        )
        .with_separation(Separation::against_distance(
            "one of the two implementations reads clause 10.10's CLUT index order backwards — \
             the same evaluated rival the preceding derived row carries, and the one a \
             cross-check between two independent readers of that clause is actually positioned \
             to catch",
            0.0,
            x_index_order_rival,
            SepUnits::SameAsMetric,
        )),
    );
    out.push(
        Record::graded(
            F_XFORM_ROWS[4].0,
            SELF,
            DEV,
            EXACT_ZERO,
            x.leak,
            SRC_SEP_RUN,
            format!(
                "★★★ THE LEAK GUARD THAT RUNS IN CI, NEW 2026-08-18. The same {} probes \
                 ({} chromatic grays + {} low-ink) run twice through the same harness \
                 function, differing in nothing but --preserve-black {}: max |on - off| over \
                 ALL FOUR channels = {:.6}, required exactly 0. ★★★ DETECTION FLOOR {:.6e} \
                 - the smallest widening of the qualifying test these probes can SEE, \
                 computed as min over probes of max(C, M, Y) and PRINTED rather than \
                 asserted. Its two parts: chromatic grays {:.6e} and low-ink {:.6e}, BOTH \
                 STRUCTURAL - every probe of both sets has max(C, M, Y) equal to its own ink \
                 level exactly, because the two ratios applied to C are strictly below 1. \
                 ★★ WHY THE LOW-INK ARM EXISTS: until 2026-08-21 this floor was 5.0e-2, and \
                 an injected widening of t = 0.04 left the ENTIRE difftest suite green with \
                 the defect compiled in, while the rival named in this row and in \
                 TOLERANCES.md was 1e-9 - seven-plus orders BELOW what the probes could see \
                 (DL-064). ★★ Note the channel count: this is the ONE §F row that \
                 includes K. Every other row here excludes it because the K value a \
                 preserving path should emit is E2 open fork - but the claim here is not \
                 about what K should be, it is that the preservation branch was NOT TAKEN, \
                 and a branch that was not taken leaves every channel alone. ★★ WHY \
                 EXACTLY ZERO. Each probe has C, M and Y all strictly positive, so under the \
                 exact-zero qualifying rule none of them qualifies and the two invocations \
                 execute identical arithmetic. This is not an agreement claim with an \
                 instrument error; it is the claim that a branch was not taken. ★ EVIDENCE \
                 CLASS IS SELF-CONSISTENCY, the weakest this suite emits: both sides are \
                 iccce. It earns its place because the predicate is EXACT where the \
                 cross-check on the same probes (the preceding row) carries a two-quantum \
                 bound - a leak smaller than 3.05e-5 would be invisible there and is visible \
                 here",
                x.leak_points,
                x.gray_points,
                x.leak_points - x.gray_points,
                PRESERVE_POLICY,
                x.leak,
                x.leak_floor,
                x.leak_gray_floor,
                x.leak_low_floor,
            ),
        )
        .with_separation(Separation::against_distance(
            "the qualifying test is widened from exact zero to a tolerance — the \
             alternative crates/iccce-cmm's module doc names and rejects, and the change \
             a future contributor is most likely to make. ★★★ AND THE PROBES NOW REACH \
             IT: the low-ink arm walks 14 decades to 1e-12, three below the 1e-9 this \
             rival has always been written with, so a widening anywhere at or above \
             1e-12 puts probes on the qualifying side and the difference becomes the \
             full distance between this fixture's two candidate answers, which §F \
             measures from the committed bytes. ★ CORRECTED 2026-08-21 - the sentence \
             `under it these probes would begin to qualify` was true only of a widening \
             above 5.0e-2, which the 50 chromatic grays alone could see; at the rival \
             own named 1e-9 nothing here moved (DL-064)",
            0.0,
            f.separation,
            SepUnits::SameAsMetric,
        )),
    );
    out
}

// ===========================================================================
// §G — WHAT THE POLICY COSTS, IN COLOUR, ON A PAIR OF DIFFERENT PRESSES
// ===========================================================================
//
// ★★★ THE QUESTION §A–§F DELIBERATELY DID NOT ANSWER.
//
// Every row above this one is in **device units**, for the reason §A measured
// and the module header states: ΔE2000 is blind to the *defect* black
// preservation exists to fix (`0.705320` of chromatic ink at `0.136090`
// ΔE2000). That is a statement about detecting the defect. It is **not** a
// statement about what applying the policy costs, and `NUMERIC_CLAIMS.md`
// registered the difference as **NA-012**'s `UNMEASURED` cost field: *nobody
// has measured the ΔE2000 between the preserved answer and the colorimetric
// one on a cross-press pair.* §G is that measurement.
//
// ★★★ THE EVIDENCE CLASS, STATED FIRST BECAUSE IT IS THE EASIEST THING TO
// OVERSTATE.
//
// **Every cost row here is `Kind::SelfConsistency` — iccce's preserved answer
// against iccce's own colorimetric answer.** That is the weakest class this
// suite emits and it is nevertheless the *right* class, because the question
// is intrinsically a comparison of the engine against itself: *what does
// applying this policy cost relative to not applying it?* No stronger class is
// available and none ever will be — `ICC_Spec` register entry **A51** is a
// closed negative (ICC.1 contains no black-preservation construct in either
// edition checked), so no published value can exist for what preservation
// *should* return, and lcms2's intent 11 is the construction iccce implements
// rather than an independent check of it.
//
// **lcms2 appears here only as a RULER**, never as an answer: both device
// answers are carried into Lab through the *destination's own* `A2B1`,
// evaluated by `transicc`. The same map is applied to both legs, so a ruler
// error largely cancels in the difference — and `G6` grades that claim rather
// than asserting it, by recomputing the headline through iccce's own `A2B1`
// evaluation and comparing the two.
//
// ★★★ THE TRAP THIS SECTION IS BUILT AROUND: A PAIR THAT DOES NOT SEPARATE
// MANUFACTURES A FALSE "THE POLICY IS NEARLY FREE".
//
// The policy is a **no-op** wherever the colorimetric answer is already
// K-only, and **nearly** a no-op wherever the two profiles describe the same
// press. Both are properties of the *fixture pair*, not of the engine, and
// both produce a small ΔE that reads exactly like good news. §G therefore
// measures the pair before it measures the policy, and four rows exist only to
// keep the headline honest:
//
// | row | what it defends against |
// |---|---|
// | `G3` | a destination whose `B2A` emits K-only anyway — the `E6` zero-separation failure, in device units |
// | `G4` | two files that are the *same press* — they must render the same device `K` values at least `1.0` ΔE2000 apart |
// | `G5` | a destination that cannot reproduce the source at all, so that the "colorimetric answer" being differenced is itself perceptibly wrong |
// | `G8` | the same measurement on a **same-press** pair, printed beside the headline so the two regimes cannot be confused |
//
// ★★ `G5` IS THE ROW THAT DISQUALIFIES THE COMMITTED SYNTHETIC PAIR, AND THAT
// IS WHY IT IS A GATE AND NOT A NOTE.
//
// `v2-cmyk-chromatic-neutral` separates the two candidate answers by
// `0.420705` **in device units** — §F grades exactly that, in CI, and it is
// sound. It cannot carry a **ΔE** row, and the reason is structural rather
// than incidental: **its `B2A0` is not the inverse of its `A2B0`.** The
// separation returns `0.60 d` of composite gray under `0.40 d` of black for a
// neutral of encoded darkness `d`, which the `A2B0` model reads back as
// darkness `0.70 d` — so the fixture's own colorimetric round trip is wrong by
// far more than any cost it could report. `G9`/`G10` run that pair in CI and
// print both numbers so the disqualification is a measurement rather than a
// paragraph. A reader who wants the cost row to run without a licence must
// first author a fixture whose two tables invert each other on the neutral
// axis; the corpus does not contain one today, and saying so is `G9`'s whole
// job.
//
// ★★ THE PAIR, AND WHY THIS ONE.
//
// `ISO Coated v2 300% (ECI)` → `GWG_GenericCMYK`, media-relative, over the
// **qualifying set** — `C = M = Y = 0`, `K = j/100`, `j = 0..=100`, the only
// inputs `KPreserve::apply` acts on at all. It is the largest cost of the
// **thirty ordered pairs** the six real CMYK members of the Ghent corpus admit
// (`passk_cost_probe` prints the whole matrix); the smallest cross-press pair
// is an order below it and the same-press pairs are two. Reporting the largest
// is deliberate: **a caller weighing an opt-in policy needs its worst case on
// a real pair**, and a mean over a matrix of profiles nobody uses together
// would hide it. `G2` reports the mean over the ramp beside it because
// `Metric`'s own doc comment forbids quoting one for the other.
//
// ★ THE DIRECTION IS PART OF THE CLAIM, and `G1` states it from a measurement
// rather than an assertion: the same pair reversed is a third licensed leg of
// this section, and it costs materially less. The policy's price is a property
// of the **destination's** black behaviour, so swapping the two profiles is a
// different measurement and not a check of the same one.
//
// ★★ THE INJECTION PROOF IS IN `tools/difftest/README.md` §26.8, and it is the
// reason to trust `G12` rather than the row count. A 5 % error injected into
// `KPreserve::map_k` turns `G12` red at **62x its bound** and `G8` red at 4x
// theirs; the four controls `G3`-`G6` do **not** move, which is correct
// because they are statements about the fixture pair rather than the engine;
// and `G16` stays green, because a population row of the shape *"no entitled
// pair finds the policy imperceptible"* is **one-sided** and cannot see a
// defect that makes the policy cost more. ★ The headline `G1` is `REPORTED`
// and can never go red: its protection is `G12` and `G8` failing beside it.
//
// ★ WHAT IS NOT MEASURED HERE, stated so a reader does not infer it: one
// intent (media-relative), no `--bpc`, one pair for the headline,
// `KMapping::EqualLightness` only — `KMapping::Ratio` is unimplemented and
// refused by name — and no weighting by the *frequency* with which a document
// contains K-only content, which is the term that decides whether the policy
// is worth its price and which belongs to the consumer.

/// **§G's probe: the qualifying set at 1 % resolution.**
///
/// `C = M = Y = 0` exactly, `K = j/100`. The predicate `KPreserve::apply`
/// tests is `C = M = Y = 0` **exactly**, so this ramp *is* the set on which
/// the policy does anything at all; a "broader sample" of qualifying inputs
/// can only be a finer ramp. Non-qualifying inputs cost exactly zero by
/// construction (`E7`/`F8` grade that at `0`), which is why the interesting
/// neighbouring measurement is not a sample but a **step** — see
/// [`CostLeg::boundary_step`].
///
/// 101 points rather than §A's 41 because the maximum is what is reported and
/// a coarse ramp can only under-state it.
#[must_use]
pub fn cost_ramp() -> Vec<[f64; 4]> {
    (0..=100)
        .map(|j| [0.0, 0.0, 0.0, f64::from(j) / 100.0])
        .collect()
}

/// Everything §G measured on **one ordered pair** of profiles.
///
/// Kept as data, like every other section's struct, so that the records and
/// the `note` line read the same numbers and cannot drift apart — the rule
/// three claim-bearing literals in this crate went false inside a day for.
#[derive(Debug, Clone)]
pub struct CostLeg {
    /// `source → destination`, in words, including which files.
    pub label: String,
    /// Points on the ramp.
    pub points: usize,
    /// ★ **The headline.** `max` over the ramp of
    /// `ΔE2000(A2B1_dst(colorimetric), A2B1_dst(preserved))`.
    pub cost_max: f64,
    /// Mean of the same quantity over the same ramp.
    pub cost_mean: f64,
    /// Minimum of the same quantity — always at or near the white end, where
    /// `K = 0` and the two answers coincide.
    pub cost_min: f64,
    /// The `K` at which `cost_max` occurred.
    pub cost_argmax_k: f64,
    /// `cost_max` recomputed with **iccce's own** `A2B1` evaluation as the
    /// ruler instead of lcms2's. `G6` grades the difference.
    pub cost_max_iccce_ruler: f64,
    /// The largest **per-point** disagreement between the two rulers. Reported
    /// rather than graded: nobody has measured this destination's own
    /// interpolation envelope, and NC-050's figure was measured elsewhere.
    pub ruler_gap: f64,
    /// **Separation, device units.** The most chromatic ink the *colorimetric*
    /// answer lays down anywhere on the ramp. Zero means the destination emits
    /// K-only anyway and the policy cannot change anything (`E6`).
    pub sep_device: f64,
    /// **Separation, ΔE2000.** How differently the two profiles render the
    /// *same* device `(0,0,0,K)` values. Zero means one press described twice.
    pub sep_press: f64,
    /// **Control.** How far the colorimetric answer itself lands from the
    /// colour the source asked for — the reference leg's own error. If this
    /// approaches the cost, the difference prices the fixture and not the
    /// policy.
    pub round_trip: f64,
    /// **The discontinuity.** `max` over the ramp of
    /// `ΔE2000(preserved(0,0,0,K), preserved(1/255,0,0,K))` — the step a
    /// consumer sees where a gradient leaves the qualifying set by one 8-bit
    /// code of cyan. Both legs are run with `--preserve-black`; only the
    /// input differs, so this is the policy's own edge and not a difference of
    /// flags.
    pub boundary_step: f64,
    /// The `K` at which `boundary_step` occurred.
    pub boundary_argmax_k: f64,
}

/// The three licensed legs §G measures.
#[derive(Debug, Clone)]
pub struct Cost {
    /// `ISO Coated v2 300% (ECI)` → `GWG_GenericCMYK` — the headline.
    pub cross: CostLeg,
    /// The same pair **reversed**, so that "the direction is part of the
    /// claim" is a measurement and not an assertion.
    pub reversed: CostLeg,
    /// `ISO Coated v2 (ECI)` → `ISO Coated v2 300% (ECI)` — two files whose
    /// `A2B1` tags are **byte-identical** (see the note on `ISOCOATED350` in
    /// the `file` module), i.e. one press with two separations. The regime
    /// NC-244's `1.360900e-1` belongs to.
    pub same_press: CostLeg,
}

/// Build a device → Lab evaluator from a profile's own `A2B1`, falling back to
/// `A2B0` for the two-tag synthetic fixtures.
///
/// This is **iccce's** reading of the destination table, used as §G's second
/// ruler. It is deliberately the reference model (`iccce_cmm`'s LUT types,
/// linked by the harness under the Pass 3 decision recorded in
/// `tools/difftest/Cargo.toml`) and not the shipped binary: the binary has no
/// device → PCS mode, and adding one for the benefit of a measurement would
/// put a feature in the product that no consumer asked for.
fn a2b_ruler(path: &Path) -> Result<crate::pass5c::TagModel, Unavailable> {
    let bytes = std::fs::read(path).map_err(|e| Unavailable::Error(e.to_string()))?;
    let p =
        iccce_profile::Profile::parse(&bytes).map_err(|e| Unavailable::Error(format!("{e:?}")))?;
    for want in [crate::pass5c::tag::A2B1, crate::pass5c::tag::A2B0] {
        let Some(e) = p.tags.iter().find(|t| t.sig == want) else {
            continue;
        };
        let Some(Ok(d)) = p.decode_tag(e) else {
            continue;
        };
        let m = match d.data {
            iccce_profile::tag_types::TagData::Lut16(l) => {
                iccce_cmm::lut_transform::Lut16Model::from_lut16(
                    &l,
                    false,
                    iccce_cmm::lut_transform::PcsKind::Lab,
                )
                .ok()
                .map(crate::pass5c::TagModel::Lut16)
            }
            iccce_profile::tag_types::TagData::Lut8(l) => {
                iccce_cmm::lut_transform::Lut16Model::from_lut8(
                    &l,
                    false,
                    iccce_cmm::lut_transform::PcsKind::Lab,
                )
                .ok()
                .map(crate::pass5c::TagModel::Lut16)
            }
            iccce_profile::tag_types::TagData::LutAToB(l) => {
                iccce_cmm::lut_ab::LutAbModel::from_lut_ab(
                    &l,
                    iccce_cmm::lut_transform::PcsKind::Lab,
                )
                .ok()
                .map(crate::pass5c::TagModel::Mab)
            }
            _ => None,
        };
        if let Some(m) = m {
            return Ok(m);
        }
    }
    Err(Unavailable::Error(format!(
        "{}: neither A2B1 nor A2B0 decodes to a Lab LUT model, so §G has no second ruler",
        path.display()
    )))
}

/// Evaluate a set of CMYK rows through an [`a2b_ruler`] model.
fn ruler_lab(m: &crate::pass5c::TagModel, rows: &[[f64; 4]]) -> Option<Vec<Lab>> {
    rows.iter()
        .map(|r| match m.device_to_pcs(r) {
            Some(iccce_cmm::lut_transform::PcsValue::Lab(l)) => Some(l),
            _ => None,
        })
        .collect()
}

/// Measure one ordered pair. Three `iccce transform` invocations — two of them
/// differing **only** in `--preserve-black` — and four `transicc` runs used as
/// the ruler.
fn analyse_cost_leg(
    oracle: &Oracle,
    iccce: &Iccce,
    label: &str,
    src: &Path,
    dst: &Path,
) -> Result<CostLeg, Unavailable> {
    let ramp = cost_ramp();
    let rows: Vec<Vec<f64>> = ramp.iter().map(|r| r.to_vec()).collect();

    let off = as_cmyk(
        iccce
            .transform_rows_shaped(src, dst, Intent::RelativeColorimetric, &rows, 4)
            .map_err(|e| Unavailable::Error(e.to_string()))?,
    )?;
    let on = as_cmyk(
        iccce
            .transform_rows_shaped_preserve_black(
                src,
                dst,
                Intent::RelativeColorimetric,
                &rows,
                4,
                PRESERVE_POLICY,
            )
            .map_err(|e| Unavailable::Error(e.to_string()))?,
    )?;

    let lab_off = to_lab(oracle, dst, &off)?;
    let lab_on = to_lab(oracle, dst, &on)?;
    // The colour the SOURCE asked for, and what the DESTINATION would make of
    // the same device values — the two halves of the pair's separation.
    let lab_src_in = to_lab(oracle, src, &ramp)?;
    let lab_dst_in = to_lab(oracle, dst, &ramp)?;

    let model = a2b_ruler(dst)?;
    let mine_off = ruler_lab(&model, &off);
    let mine_on = ruler_lab(&model, &on);

    let mut cost_max = 0.0_f64;
    let mut cost_min = f64::INFINITY;
    let mut cost_argmax_k = 0.0;
    let mut sum = 0.0;
    let mut sep_device = 0.0_f64;
    let mut sep_press = 0.0_f64;
    let mut round_trip = 0.0_f64;
    let mut ruler_gap = 0.0_f64;
    let mut cost_max_iccce_ruler = 0.0_f64;
    for (i, p) in ramp.iter().enumerate() {
        let c = delta_e_2000(lab_off[i], lab_on[i]);
        if c > cost_max {
            cost_max = c;
            cost_argmax_k = p[3];
        }
        cost_min = cost_min.min(c);
        sum += c;
        sep_device = sep_device.max(off[i][0].max(off[i][1]).max(off[i][2]));
        sep_press = sep_press.max(delta_e_2000(lab_src_in[i], lab_dst_in[i]));
        round_trip = round_trip.max(delta_e_2000(lab_src_in[i], lab_off[i]));
        if let (Some(a), Some(b)) = (&mine_off, &mine_on) {
            let mine = delta_e_2000(a[i], b[i]);
            cost_max_iccce_ruler = cost_max_iccce_ruler.max(mine);
            ruler_gap = ruler_gap.max((mine - c).abs());
        }
    }

    // The boundary step: the same PRESERVING invocation, one 8-bit code of
    // cyan away from the qualifying set.
    let just_off: Vec<Vec<f64>> = ramp
        .iter()
        .map(|r| vec![1.0 / 255.0, 0.0, 0.0, r[3]])
        .collect();
    let nb = as_cmyk(
        iccce
            .transform_rows_shaped_preserve_black(
                src,
                dst,
                Intent::RelativeColorimetric,
                &just_off,
                4,
                PRESERVE_POLICY,
            )
            .map_err(|e| Unavailable::Error(e.to_string()))?,
    )?;
    let lab_nb = to_lab(oracle, dst, &nb)?;
    let mut boundary_step = 0.0_f64;
    let mut boundary_argmax_k = 0.0;
    for (i, p) in ramp.iter().enumerate() {
        let d = delta_e_2000(lab_on[i], lab_nb[i]);
        if d > boundary_step {
            boundary_step = d;
            boundary_argmax_k = p[3];
        }
    }

    #[expect(
        clippy::cast_precision_loss,
        reason = "the ramp has 101 points; the count is exactly representable"
    )]
    let mean = sum / ramp.len() as f64;
    Ok(CostLeg {
        label: label.to_string(),
        points: ramp.len(),
        cost_max,
        cost_mean: mean,
        cost_min,
        cost_argmax_k,
        cost_max_iccce_ruler,
        ruler_gap,
        sep_device,
        sep_press,
        round_trip,
        boundary_step,
        boundary_argmax_k,
    })
}

/// §G's licensed legs.
fn analyse_cost(oracle: &Oracle, iccce: &Iccce) -> Result<Cost, Unavailable> {
    let iso300 = need_corpus(file::ISOCOATED300)?;
    let gwg = need_corpus(file::GENERIC_CMYK)?;
    let iso350 = need_corpus(file::ISOCOATED350)?;
    Ok(Cost {
        cross: analyse_cost_leg(
            oracle,
            iccce,
            "ISO Coated v2 300% (ECI) -> GWG_GenericCMYK [two different presses]",
            &iso300,
            &gwg,
        )?,
        reversed: analyse_cost_leg(
            oracle,
            iccce,
            "GWG_GenericCMYK -> ISO Coated v2 300% (ECI) [the same pair, reversed]",
            &gwg,
            &iso300,
        )?,
        same_press: analyse_cost_leg(
            oracle,
            iccce,
            "ISO Coated v2 (ECI) -> ISO Coated v2 300% (ECI) [ONE press, two separations: the \
             two files' A2B1 tags are byte-identical]",
            &iso350,
            &iso300,
        )?,
    })
}

/// §G's committed leg — the one that runs in CI, and the one whose job is to
/// show that it **cannot** carry the headline.
fn analyse_cost_synthetic(oracle: &Oracle, iccce: &Iccce) -> Result<CostLeg, Unavailable> {
    let src = need_synthetic(SYNTHETIC_CMYK)?;
    let dst = need_synthetic(SYNTHETIC_SEPARATING)?;
    analyse_cost_leg(
        oracle,
        iccce,
        "fixtures/synthetic/v2-cmyk-mft2-lab.icc -> \
         fixtures/synthetic/v2-cmyk-chromatic-neutral.icc [committed, unlicensed, runs in CI — \
         and DISQUALIFIED as a cost fixture by the row above]",
        &src,
        &dst,
    )
}

/// **`G4`'s floor: the two profiles must be perceptibly different presses.**
///
/// `1.0` ΔE2000 — §2's perceptibility anchor, used here as a **lower** bound
/// on the *fixture pair* rather than an upper bound on an error. If the two
/// profiles render the same device `(0,0,0,K)` within the threshold of
/// perceptible difference, they describe one press for this subject, the
/// preserved and colorimetric answers cannot differ by much whatever the
/// engine does, and a small cost is a property of the pair. The corpus
/// contains such a pair and `G8` runs it on purpose.
pub const PRESS_SEPARATION_FLOOR: f64 = 1.0;

/// `G4`'s gate, in the shortfall form `F3` established: `observed =
/// max(0, floor − separation)`, graded at zero.
pub const PRESS_SEPARATION_FLOOR_MET: Tolerance = Tolerance::new(
    0.0,
    "ZERO SHORTFALL against a PAIR separation floor of 1.0 dE2000 — TOLERANCES.md §2's \
     perceptibility anchor, applied as a LOWER bound on the fixture pair rather than an upper \
     bound on an error. Below it the two profiles are one press for this subject and any cost \
     they report is a property of the pair rather than of the policy. Derived from the anchor, \
     NOT from what this pair happens to measure — which at the time of writing is several times \
     the floor, and the margin is printed on the row",
);

/// **`G5`'s gate: the leg being differenced must itself be sound.**
///
/// A cost is a difference between two answers, and it prices the *policy* only
/// while the answer it is differenced against is the colour the source asked
/// for. `1.0` ΔE2000 — the perceptibility anchor again, and this time in its
/// ordinary direction: if the colorimetric answer is itself *perceptibly*
/// wrong, the difference measures the destination's gamut and its round trip
/// rather than the policy.
///
/// ★ **This bound bites, and the row exists because it did.** On the committed
/// synthetic pair the same quantity is more than an order of magnitude above
/// it — larger than the cost that pair would have reported — and `G9` prints
/// the number in CI rather than leaving it to this comment.
pub const REFERENCE_LEG_SOUND: Tolerance = Tolerance::new(
    1.0,
    "the accepted threshold of perceptible difference for adjacent patches (TOLERANCES.md §2), \
     applied to the leg the cost is differenced AGAINST. A cost prices the policy only while \
     the colorimetric answer is the colour the source asked for; once that leg is itself \
     perceptibly wrong the difference prices the destination's gamut and round trip instead. \
     The bound BITES: the committed synthetic pair is disqualified by it, which is why the \
     headline needs a licensed corpus",
);

/// **`G8`'s bound: on a same-press pair the policy is imperceptible.**
///
/// Same anchor, third use, and the claim is the one NA-012's *"what must NOT
/// be quoted as its cost"* field is about: where the two profiles describe one
/// press, K-only preservation changes the answer by less than the threshold of
/// perceptible difference — so quoting that number as the policy's price
/// understates it by more than an order of magnitude.
pub const SAME_PRESS_IMPERCEPTIBLE: Tolerance = Tolerance::new(
    1.0,
    "the perceptibility anchor (TOLERANCES.md §2). The claim being graded is that on a pair of \
     profiles describing ONE press the policy is invisible — which is why NC-244's same-profile \
     1.360900e-1 must never be quoted as the cost of the policy. A red row here would mean the \
     same-press regime is NOT the cheap one and the framing of NA-012's cost field is wrong",
);

/// **`G6`'s bound: the headline must not depend on which ruler carried the
/// device answers into Lab.**
///
/// `0.254 23` ΔE2000 — **NC-050's measured interpolation-method envelope for
/// the media-relative `A2B1` direction** on a real CMYK profile
/// (`NUMERIC_CLAIMS.md` §3.11; the `A2B0` figure is `1.5741` and is *not* the
/// one that applies here, because §G converts at media-relative). It is the
/// largest disagreement this project has measured between two conformant
/// evaluations of one CMYK `A2B` table, so a headline that moves by less than
/// it when the ruler changes has not been decided by the ruler.
///
/// ★ **What it is not.** It was measured on `ISO Coated v2 300% (ECI)`, not on
/// this destination, whose own envelope nobody has measured; the per-point
/// gap between the two rulers ([`CostLeg::ruler_gap`]) exceeds the borrowed
/// number on this pair. That is disclosed on the row rather than smoothed
/// over — the graded quantity is the **headline's** movement, because that is
/// the number NA-012's cost field will carry.
pub const RULER_INVARIANCE: Tolerance = Tolerance::new(
    0.254_23,
    "NC-050's MEASURED interpolation-method envelope for the media-relative A2B1 direction on a \
     real CMYK profile (NUMERIC_CLAIMS.md §3.11) — the largest disagreement this project has \
     measured between two conformant evaluations of one CMYK A2B table. BORROWED from a \
     different profile, which is stated on the row: this destination's own envelope has never \
     been measured and the per-point ruler gap here exceeds the borrowed figure. What is graded \
     is the movement of the HEADLINE when the ruler changes, because that is the number NA-012 \
     will carry",
);

const G_ROWS: [(&str, Kind, Metric, Tolerance); 8] = [
    (
        "passk/G/cost/isocoated300-to-generic-cmyk/media-relative/dE2000-max",
        SELF,
        DE,
        REPORTED,
    ),
    (
        "passk/G/cost/isocoated300-to-generic-cmyk/media-relative/dE2000-mean",
        SELF,
        Metric::DeltaE2000Mean,
        REPORTED,
    ),
    (
        "passk/G/separation/the-colorimetric-answer-DOES-lay-chromatic-ink",
        SELF,
        DEV,
        SEPARATION_FLOOR_MET,
    ),
    (
        "passk/G/separation/the-two-presses-render-the-same-K-DIFFERENTLY",
        OR,
        DE,
        PRESS_SEPARATION_FLOOR_MET,
    ),
    (
        "passk/G/control/the-leg-the-cost-is-differenced-against-is-SOUND",
        CC,
        DE,
        REFERENCE_LEG_SOUND,
    ),
    (
        "passk/G/control/the-headline-does-not-depend-on-the-RULER",
        CC,
        DE,
        RULER_INVARIANCE,
    ),
    (
        "passk/G/cost/boundary-step-at-one-8-bit-code-of-cyan",
        SELF,
        DE,
        REPORTED,
    ),
    (
        "passk/G/control/on-a-SAME-PRESS-pair-the-policy-is-imperceptible",
        SELF,
        DE,
        SAME_PRESS_IMPERCEPTIBLE,
    ),
];

/// §G's two committed rows. They need no licence and run in CI; neither is the
/// headline, and the second says so in its identifier.
const G_SYNTHETIC_ROWS: [(&str, Kind, Metric, Tolerance); 2] = [
    (
        "passk/G/synthetic-pair/control/the-reference-leg-is-NOT-sound",
        CC,
        DE,
        REPORTED,
    ),
    (
        "passk/G/synthetic-pair/cost/DISQUALIFIED-by-its-own-reference-leg",
        SELF,
        DE,
        REPORTED,
    ),
];

const SRC_COST: &str = "Pass K §G — the SHIPPED iccce binary run twice on the same input, \
    differing only in --preserve-black k-only-equal-lightness, both answers carried into Lab \
    through the DESTINATION's own A2B1 by the pinned lcms2 acting as a RULER. Self-comparison: \
    iccce's preserved answer against iccce's own colorimetric answer, which is the only class \
    the question admits (ICC_Spec A51 is a closed negative). Ghent v5.0, licensed, skips in CI";

const SRC_COST_SYN: &str = "Pass K §G — the same measurement on two COMMITTED synthetic CMYK \
    fixtures, so that the reason the headline needs a licence is a number in CI rather than a \
    claim in a comment";

/// A number from a leg that may not have run, rendered for a detail string.
/// Never a typed literal: a claim-bearing numeral that goes stale is the
/// failure this helper exists to make impossible.
fn opt_num(v: Option<f64>, digits: usize) -> String {
    v.map_or_else(
        || "[not measured in this run]".to_string(),
        |x| format!("{x:.digits$}"),
    )
}

fn cost_records(c: &Cost, synthetic: Option<&CostLeg>) -> Vec<Record> {
    let x = &c.cross;
    let s = &c.same_press;
    let r = &c.reversed;
    let mut out = Vec::new();

    out.push(
        Record::graded(
            G_ROWS[0].0,
            G_ROWS[0].1,
            G_ROWS[0].2,
            G_ROWS[0].3,
            x.cost_max,
            SRC_COST,
            format!(
                "★★★ THE COST OF THE POLICY, and the number NA-012's cost field was registered \
                 as UNMEASURED for. {}: the preserved answer differs from the colorimetric one \
                 by up to {:.6} dE2000, at K = {:.2}, over {} qualifying points at \
                 media-relative. Mean {:.6} (G2). REPORTED, never graded: no requirement bounds \
                 what an opt-in policy may cost, and grading it would gate the caller's choice. \
                 The pair separates — {:.6} of chromatic ink in the colorimetric answer (G3) and \
                 {:.4} dE2000 between the two presses' own rendering of the same device K (G4) \
                 — and the leg this is differenced against is itself within {:.4} dE2000 of the \
                 colour asked for (G5). ★ DIRECTION IS PART OF THE CLAIM: reversed, the same \
                 pair costs {:.6}, because the price is a property of the DESTINATION's black",
                x.label,
                x.cost_max,
                x.cost_argmax_k,
                x.points,
                x.cost_mean,
                x.sep_device,
                x.sep_press,
                x.round_trip,
                r.cost_max
            ),
        )
        .with_separation(Separation::against_distance(
            "the SAME measurement on a same-press pair — ISO Coated v2 (ECI) -> ISO Coated v2 \
             300% (ECI), whose A2B1 tags are byte-identical — which is the regime NC-244's \
             1.360900e-1 belongs to and the number NA-012 forbids quoting as the cost. It is a \
             property of the PAIR, so it is supplied and not derived: it does not collapse when \
             the engine changes",
            s.cost_max,
            x.cost_max - s.cost_max,
            SepUnits::SameAsMetric,
        )),
    );

    out.push(
        Record::graded(
            G_ROWS[1].0,
            G_ROWS[1].1,
            G_ROWS[1].2,
            G_ROWS[1].3,
            x.cost_mean,
            SRC_COST,
            format!(
                "The mean of the same distribution over the same {} points — its own row because \
                 a mean and a max answer different questions and Metric's own doc comment \
                 forbids quoting one for the other. The distribution is far from flat: {:.6} at \
                 its minimum, {:.6} at its maximum (K = {:.2}), where the destination's K ink \
                 alone can no longer reach the lightness its four-ink separation reaches. \
                 `passk_cost_probe` prints every point",
                x.points, x.cost_min, x.cost_max, x.cost_argmax_k
            ),
        )
        .with_separation(Separation::against_distance(
            "the same-press pair's mean over the same ramp",
            s.cost_mean,
            x.cost_mean - s.cost_mean,
            SepUnits::SameAsMetric,
        )),
    );

    let sep_shortfall = (SEPARATION_FLOOR - x.sep_device).max(0.0);
    out.push(
        Record::graded(
            G_ROWS[2].0,
            G_ROWS[2].1,
            G_ROWS[2].2,
            G_ROWS[2].3,
            sep_shortfall,
            SRC_COST,
            format!(
                "★★ THE ANTI-ARTEFACT ROW, in device units. The colorimetric answer lays down \
                 {:.6} of chromatic ink at its worst point on this ramp, against a floor of \
                 {:.0e} declared by §F in advance; the observed value is the shortfall below \
                 that floor. A destination whose B2A emits K-only anyway would report a cost of \
                 nearly zero and it would be a property of the fixture — which is exactly what \
                 E6 measured on v2-cmyk-mft2-lab, whose separation is 0.000000",
                x.sep_device, SEPARATION_FLOOR
            ),
        )
        .with_separation(Separation::against_distance(
            "the same quantity on the fixture E6 disqualified (v2-cmyk-mft2-lab as the \
             destination), where the colorimetric answer is already K-only, the separation is \
             exactly zero and the shortfall would be the whole floor",
            SEPARATION_FLOOR,
            x.sep_device,
            SepUnits::Other("device units of chromatic ink, not a shortfall"),
        )),
    );

    let press_shortfall = (PRESS_SEPARATION_FLOOR - x.sep_press).max(0.0);
    out.push(
        Record::graded(
            G_ROWS[3].0,
            G_ROWS[3].1,
            G_ROWS[3].2,
            G_ROWS[3].3,
            press_shortfall,
            "Pass K §G — the pinned lcms2 rendering the SAME device values through each of the \
             two profiles' own A2B1. iccce is absent from this row entirely: it is a statement \
             about the FIXTURE PAIR, with the oracle used as an instrument",
            format!(
                "★★ THE SECOND ANTI-ARTEFACT ROW, and the one a device-unit separation cannot \
                 make. The two profiles render the same (0,0,0,K) device values up to {:.4} \
                 dE2000 apart, against a floor of {:.1}; the observed value is the shortfall. \
                 Two files describing ONE press pass G3 — their separations still differ — and \
                 fail here, and the corpus contains such a pair: ISO Coated v2 (ECI) and ISO \
                 Coated v2 300% (ECI) have byte-identical A2B1 tags and separate by exactly \
                 {:.4}. G8 runs it on purpose",
                x.sep_press, PRESS_SEPARATION_FLOOR, s.sep_press
            ),
        )
        .with_separation(Separation::against_distance(
            "the same quantity on the same-press pair, which is the state this floor exists to \
             reject and which the corpus actually contains",
            s.sep_press,
            x.sep_press - s.sep_press,
            SepUnits::Other("dE2000 between two profiles' renderings, not a shortfall"),
        )),
    );

    out.push(
        Record::graded(
            G_ROWS[4].0,
            G_ROWS[4].1,
            G_ROWS[4].2,
            G_ROWS[4].3,
            x.round_trip,
            "Pass K §G — iccce's colorimetric answer carried back into Lab by the pinned lcms2 \
             and compared with the source profile's own rendering of the input. A CROSS-CHECK: \
             the two ends of this comparison come from different implementations",
            format!(
                "★★★ THE ROW THAT DISQUALIFIES A FIXTURE PAIR, and the reason §G needs a \
                 licensed corpus. The colorimetric answer — the leg the cost is differenced \
                 AGAINST — lands {:.6} dE2000 from the colour the source asked for: inside the \
                 perceptibility anchor, and {:.1}x below the cost it is used to price. On the \
                 committed synthetic pair the same quantity is {} (G9), which is why that pair \
                 cannot carry the headline however large a DEVICE separation it has",
                x.round_trip,
                x.cost_max / x.round_trip.max(f64::MIN_POSITIVE),
                opt_num(synthetic.map(|y| y.round_trip), 4)
            ),
        )
        .with_separation(Separation::against_distance(
            "the same control on the committed synthetic pair, whose destination's B2A0 is not \
             the inverse of its A2B0 — measured by G9 in CI, and reported here as unavailable \
             rather than typed when that leg did not run",
            synthetic.map_or(f64::NAN, |y| y.round_trip),
            synthetic.map_or(f64::NAN, |y| y.round_trip - x.round_trip),
            SepUnits::SameAsMetric,
        )),
    );

    let ruler_move = (x.cost_max - x.cost_max_iccce_ruler).abs();
    out.push(
        Record::graded(
            G_ROWS[5].0,
            G_ROWS[5].1,
            G_ROWS[5].2,
            G_ROWS[5].3,
            ruler_move,
            "Pass K §G — the SAME two device answers carried into Lab twice: once by the pinned \
             lcms2 (transicc, through the destination's A2B1) and once by iccce_cmm's own LUT \
             model of the same tag. A cross-check OF THE INSTRUMENT, not of the answer",
            format!(
                "The headline moves by {:.6} dE2000 when the ruler changes ({:.6} vs {:.6}), \
                 against NC-050's measured A2B1 interpolation envelope. ★ The per-point \
                 disagreement between the two rulers reaches {:.6}, ABOVE the borrowed bound — \
                 disclosed here rather than smoothed over, and the reason the bound is applied \
                 to the headline's movement and not to the ramp. It is also the answer to the \
                 obvious objection to a self-comparison ruled by lcms2: the same map is applied \
                 to both legs, so most of the ruler cancels in the difference",
                ruler_move, x.cost_max, x.cost_max_iccce_ruler, x.ruler_gap
            ),
        )
        .with_separation(Separation::against_distance(
            "the per-point ruler gap over the same ramp — what this row would observe if the two \
             legs were NOT differenced under one ruler",
            x.ruler_gap,
            (x.ruler_gap - ruler_move).abs(),
            SepUnits::SameAsMetric,
        )),
    );

    out.push(
        Record::graded(
            G_ROWS[6].0,
            G_ROWS[6].1,
            G_ROWS[6].2,
            G_ROWS[6].3,
            x.boundary_step,
            SRC_COST,
            format!(
                "★★ THE DISCONTINUITY, which the cost distribution does not show. The policy \
                 fires at C = M = Y = 0 EXACTLY, so a gradient that leaves the qualifying set by \
                 one 8-bit code of cyan steps by up to {:.6} dE2000 (at K = {:.2}) — both legs \
                 run WITH --preserve-black, so this is the policy's own edge and not a \
                 difference of flags. It is the number a consumer painting a K-only-to-rich ramp \
                 actually sees, and on this pair it is slightly LARGER than the cost itself \
                 ({:.6}). E7/F8 grade the other half of the same fact at exactly 0: the policy \
                 does not touch a non-qualifying input",
                x.boundary_step, x.boundary_argmax_k, x.cost_max
            ),
        )
        .with_separation(Separation::none(
            "no rival READING of a step: the two answers compared are one invocation shape at \
             two adjacent inputs, and the only alternative — a policy with a wider qualifying \
             region, as KMapping::Ratio's near-neutral band would be — is a different feature \
             rather than a different reading of this one",
        )),
    );

    out.push(
        Record::graded(
            G_ROWS[7].0,
            G_ROWS[7].1,
            G_ROWS[7].2,
            G_ROWS[7].3,
            s.cost_max,
            SRC_COST,
            format!(
                "★★★ THE ROW THAT KEEPS NC-244 IN ITS PLACE. {}: cost {:.6} dE2000 max, {:.6} \
                 mean — imperceptible, and {:.1}x smaller than the cross-press number G1 \
                 reports. The two files' A2B1 tags are byte-identical, so their press separation \
                 is {:.4}: this is ONE press with two separations, the regime NC-244's \
                 same-profile 1.360900e-1 belongs to. Quoting either as the policy's cost \
                 understates it by more than an order of magnitude, which is what NA-012's \
                 'what must NOT be quoted' field says and what this row measures",
                s.label,
                s.cost_max,
                s.cost_mean,
                x.cost_max / s.cost_max.max(f64::MIN_POSITIVE),
                s.sep_press
            ),
        )
        .with_separation(Separation::against_distance(
            "the cross-press pair G1 measures — the same policy, the same ramp, the same intent, \
             a genuinely different destination press",
            x.cost_max,
            x.cost_max - s.cost_max,
            SepUnits::SameAsMetric,
        )),
    );

    out
}

fn cost_synthetic_records(c: &CostLeg, licensed: Option<&Cost>) -> Vec<Record> {
    let mut out = Vec::new();
    out.push(
        Record::graded(
            G_SYNTHETIC_ROWS[0].0,
            G_SYNTHETIC_ROWS[0].1,
            G_SYNTHETIC_ROWS[0].2,
            G_SYNTHETIC_ROWS[0].3,
            c.round_trip,
            SRC_COST_SYN,
            format!(
                "★★★ WHY THE COST ROW NEEDS A LICENCE, as a number in CI. On {} the colorimetric \
                 answer lands {:.6} dE2000 from the colour the source asked for — {:.0}x G5's \
                 bound of {:.1}. The cause is structural and is in the recipe: \
                 v2-cmyk-chromatic-neutral's B2A0 returns 0.60d of composite gray under 0.40d of \
                 black for a neutral of encoded darkness d, and its own A2B0 reads that back as \
                 darkness 0.70d — the two tables do not invert each other. Sound for §F, which \
                 grades DEVICE values, and fatal for a dE row. REPORTED and not graded: a red \
                 row in CI would be a claim about the engine, and this is a claim about the \
                 fixture",
                c.label,
                c.round_trip,
                c.round_trip / REFERENCE_LEG_SOUND.value,
                REFERENCE_LEG_SOUND.value
            ),
        )
        .with_separation(Separation::against_distance(
            "the same control on the licensed cross-press pair (G5) — the state this fixture \
             would have to reach before it could carry the headline",
            licensed.map_or(f64::NAN, |l| l.cross.round_trip),
            licensed.map_or(f64::NAN, |l| c.round_trip - l.cross.round_trip),
            SepUnits::SameAsMetric,
        )),
    );
    out.push(
        Record::graded(
            G_SYNTHETIC_ROWS[1].0,
            G_SYNTHETIC_ROWS[1].1,
            G_SYNTHETIC_ROWS[1].2,
            G_SYNTHETIC_ROWS[1].3,
            c.cost_max,
            SRC_COST_SYN,
            format!(
                "★★ THE NUMBER THIS PAIR WOULD HAVE REPORTED, printed so that nobody derives it \
                 independently and quotes it: {:.6} dE2000 max, {:.6} mean. It is NOT the cost \
                 of the policy and must never be quoted as one — the row above measures a \
                 reference leg {:.6} dE2000 wrong, larger than this number, so what is \
                 differenced here is mostly the destination's failure to invert its own A2B. \
                 The identifier carries the disqualification so that a grep for 'cost' cannot \
                 return it without it. The licensed cross-press headline is {}",
                c.cost_max,
                c.cost_mean,
                c.round_trip,
                opt_num(licensed.map(|l| l.cross.cost_max), 6)
            ),
        )
        .with_separation(Separation::against_distance(
            "the licensed cross-press pair's cost (G1), which is the number NA-012 carries",
            licensed.map_or(f64::NAN, |l| l.cross.cost_max),
            licensed.map_or(f64::NAN, |l| (c.cost_max - l.cross.cost_max).abs()),
            SepUnits::SameAsMetric,
        )),
    );
    out
}

// ---------------------------------------------------------------------------
// §G, third part — the POPULATION, because one pair is a fixture
// ---------------------------------------------------------------------------
//
// ★★★ WHY A POPULATION ROW EXISTS AT ALL. `G1` reports one ordered pair, and
// a reader is entitled to ask whether that pair was chosen because it was
// large. It was: it is the largest cost among the pairs **entitled** to price
// the policy. `G16` is the row that makes that admission checkable, by
// running the same measurement over **every ordered pair** the six real CMYK
// members of the corpus admit and grading a statement about all of them:
//
// > **No pair entitled to price this policy finds it imperceptible.**
//
// A count, graded at zero. It is a weaker statement than the headline and a
// much harder one to have got lucky with — the headline could be one unusual
// destination; this cannot.
//
// ★★ "ENTITLED" IS THE SAME THREE GATES `G3`/`G4`/`G5` APPLY, and they are
// applied here **per pair** rather than to one chosen pair: the colorimetric
// answer must lay ink (§F's `4e-2` floor), the two profiles must render the
// same device `K` at least `1.0` ΔE2000 apart, and the reference leg must be
// sound to `1.0` ΔE2000. **Nineteen of the thirty pairs fail at least one**,
// and every one of them would have reported a comfortable "the policy is
// nearly free". Without the filter the same count is not zero — the row's
// separation says by how much, which is the number that shows the filter is
// load-bearing rather than decorative.
//
// ★ WHAT THE POPULATION IS NOT. Six profiles, one intent, one direction of
// each pair. It is a statement about **these six files**, not about presses;
// two of them (`ISO Coated v2 (ECI)` and its `300 %` sibling) share an `A2B1`
// byte for byte, so the six sources are five behaviours.

/// The six real CMYK members of the corpus, in the order `G16` walks them.
const POPULATION: [(&str, &str); 6] = [
    ("ISO Coated v2 300% (ECI)", file::ISOCOATED300),
    ("ISO Coated v2 (ECI)", file::ISOCOATED350),
    ("Coated FOGRA39", file::FOGRA39),
    ("Coated FOGRA27", file::FOGRA27),
    ("GWG_GenericCMYK", file::GENERIC_CMYK),
    ("GWG_ICC_v4_testprofile (X-Rite)", file::XRITE_V4),
];

/// What `G16` measured over every ordered pair.
#[derive(Debug, Clone)]
pub struct CostPopulation {
    /// Ordered pairs measured.
    pub pairs: usize,
    /// Of those, how many pass all three entitlement gates.
    pub entitled: usize,
    /// ★ **The graded quantity**: entitled pairs whose cost is at or below the
    /// perceptibility anchor. Zero is the claim.
    pub imperceptible_entitled: usize,
    /// The same count **without** the entitlement filter — the row's
    /// separation, and the number that shows the filter is load-bearing.
    pub imperceptible_all: usize,
    /// Smallest and largest cost among entitled pairs, with their names.
    pub smallest: (f64, String),
    pub largest: (f64, String),
}

/// Run the whole matrix. Thirty ordered pairs at three subprocess invocations
/// each; the whole block costs a few seconds, which is why it is a row rather
/// than a probe-only statement.
fn analyse_cost_population(oracle: &Oracle, iccce: &Iccce) -> Result<CostPopulation, Unavailable> {
    let mut out = CostPopulation {
        pairs: 0,
        entitled: 0,
        imperceptible_entitled: 0,
        imperceptible_all: 0,
        smallest: (f64::INFINITY, String::new()),
        largest: (0.0, String::new()),
    };
    for (sn, sf) in POPULATION {
        for (dn, df) in POPULATION {
            if sn == dn {
                continue;
            }
            let src = need_corpus(sf)?;
            let dst = need_corpus(df)?;
            let leg = analyse_cost_leg(oracle, iccce, &format!("{sn} -> {dn}"), &src, &dst)?;
            out.pairs += 1;
            if leg.cost_max <= SAME_PRESS_IMPERCEPTIBLE.value {
                out.imperceptible_all += 1;
            }
            let entitled = leg.sep_device >= SEPARATION_FLOOR
                && leg.sep_press >= PRESS_SEPARATION_FLOOR
                && leg.round_trip <= REFERENCE_LEG_SOUND.value;
            if !entitled {
                continue;
            }
            out.entitled += 1;
            if leg.cost_max <= SAME_PRESS_IMPERCEPTIBLE.value {
                out.imperceptible_entitled += 1;
            }
            if leg.cost_max < out.smallest.0 {
                out.smallest = (leg.cost_max, format!("{sn} -> {dn}"));
            }
            if leg.cost_max > out.largest.0 {
                out.largest = (leg.cost_max, format!("{sn} -> {dn}"));
            }
        }
    }
    Ok(out)
}

/// **`G16`'s bound: zero, and it is a UNIVERSAL statement.**
///
/// The graded quantity is a count of pairs, so there is no instrument error in
/// it — a pair either costs more than the perceptibility anchor or it does
/// not, and the anchor is `TOLERANCES.md` §2's, not a number chosen here. The
/// claim is *"no entitled pair finds this policy imperceptible"*, which **one
/// counterexample refutes**; a bound above zero would be an allowance for
/// counterexamples, which is the opposite of what a universal claim admits.
///
/// ★ It is deliberately **not** [`EXACT_ZERO`], whose `why` is about three ink
/// channels carrying the encoded value zero. Reusing that constant here would
/// have put a justification about ink beside a count of profile pairs — a
/// tolerance whose reason belongs to another row is the failure `Tolerance`'s
/// required `why` exists to prevent.
pub const NO_IMPERCEPTIBLE_ENTITLED_PAIR: Tolerance = Tolerance::new(
    0.0,
    "ZERO COUNTEREXAMPLES. The claim is universal - no pair ENTITLED to price this policy \
     finds it imperceptible - and a universal claim is refuted by one instance, so the only \
     bound it admits is zero. There is no instrument error in a count. The threshold each \
     pair is tested against is TOLERANCES.md \u{a7}2's 1.0 dE2000 perceptibility anchor and \
     not a number chosen here; the entitlement gates are G3's, G4's and G5's, applied per pair",
);

const G_POPULATION_ROWS: [(&str, Kind, Metric, Tolerance); 1] = [(
    "passk/G/population/no-ENTITLED-pair-finds-the-policy-imperceptible",
    SELF,
    CNT,
    NO_IMPERCEPTIBLE_ENTITLED_PAIR,
)];

fn cost_population_records(p: &CostPopulation, cross: &CostLeg) -> Vec<Record> {
    vec![
        Record::graded(
            G_POPULATION_ROWS[0].0,
            G_POPULATION_ROWS[0].1,
            G_POPULATION_ROWS[0].2,
            G_POPULATION_ROWS[0].3,
            #[expect(
                clippy::cast_precision_loss,
                reason = "a count of at most 30 pairs is exact in f64"
            )]
            {
                p.imperceptible_entitled as f64
            },
            "Pass K §G — the same self-comparison as G1, run over EVERY ordered pair \
             of the six real CMYK members of the Ghent v5.0 corpus. Licensed; skips in CI",
            format!(
                "★★★ THE POPULATION CLAIM, and it is the one that makes G1's single pair \
                 checkable. Of {} ordered pairs, {} are ENTITLED to price this policy (the \
                 colorimetric answer lays ink, the two profiles' K axes are at least {:.1} \
                 dE2000 apart, and the reference leg is sound to {:.1}), and {} of those find \
                 the policy imperceptible. Among the entitled the cost runs {:.6} ({}) to \
                 {:.6} ({}); G1 grades {:.6} on {}, chosen for continuity with §A-§E rather \
                 than for size — it is {:.6} below the largest. ★ The other {} pairs fail a \
                 gate, and every one of them would have read as 'the policy is nearly free' \
                 — which is what the separation prices",
                p.pairs,
                p.entitled,
                PRESS_SEPARATION_FLOOR,
                REFERENCE_LEG_SOUND.value,
                p.imperceptible_entitled,
                p.smallest.0,
                p.smallest.1,
                p.largest.0,
                p.largest.1,
                cross.cost_max,
                cross.label,
                p.largest.0 - cross.cost_max,
                p.pairs - p.entitled
            ),
        )
        .with_separation(Separation::against_distance(
            "the same count WITHOUT the entitlement filter — what this row would have observed \
             if a reader took any two CMYK profiles and measured. A property of the corpus, \
             supplied and not derived, and it is what the three gates are worth",
            #[expect(
                clippy::cast_precision_loss,
                reason = "a count of at most 30 pairs is exact in f64"
            )]
            {
                p.imperceptible_all as f64
            },
            #[expect(
                clippy::cast_precision_loss,
                reason = "a count of at most 30 pairs is exact in f64"
            )]
            {
                (p.imperceptible_all - p.imperceptible_entitled) as f64
            },
            SepUnits::SameAsMetric,
        )),
    ]
}

// ---------------------------------------------------------------------------
// §G, second half — the COMMITTED pair that CAN carry a ΔE row
// ---------------------------------------------------------------------------
//
// ★★★ WHAT THE FIRST HALF ESTABLISHED, AND WHY A FIXTURE WAS AUTHORED.
//
// `G9` measures, in CI, that the committed pair Pass K already had cannot
// price this policy: `v2-cmyk-chromatic-neutral`'s `B2A0` is not the inverse
// of its own `A2B0`, so the leg a cost is differenced *against* is itself more
// than twenty ΔE2000 wrong. A device-unit separation of `0.420705` does not
// rescue that — the two failures are independent, and a fixture must survive
// **both** to carry a ΔE row:
//
// | trap | detected by | the committed pair |
// |---|---|---|
// | the colorimetric answer is already K-only | `G3`-shaped device separation | **survives** — §F's whole point |
// | the reference leg is itself wrong | `G5`-shaped round-trip control | **fails**, at more than 20 ΔE2000 |
// | the two profiles are one press | `G4`-shaped pair separation | survives |
// | ★ the preserved answer is a **METAMER** of the colorimetric one | *nothing above* | **fails silently** |
//
// ★★ THE FOURTH ROW OF THAT TABLE IS THE ONE NOBODY WOULD HAVE PREDICTED, and
// it is the reason the new fixture varies the black ink's **chroma** rather
// than anything else. `chromatic_neutral_a2b`'s `K` appears in `L*` and in
// nothing else — a *spectrally neutral* black. On such a profile the
// preserved answer at matched lightness has the same `L*`, the same `a*` and
// the same `b*` as the four-ink separation it replaced: the two answers are
// **colorimetrically identical however much ink separates them**, and the cost
// is zero as a property of the model. A fixture can therefore separate by
// `0.42` of ink, pass every device-unit gate in §F, and still report a cost of
// zero for a reason that has nothing to do with the policy. `fixtures/synthetic`
// `/v2-cmyk-warm-black.icc` exists because that is not a defect a tolerance
// can catch.
//
// ★★ WHAT WAS VARIED, AND WHAT WAS HELD. Against its sibling the new recipe
// changes exactly two things: `K` carries chroma (`a* += 2K`, `b* += 6K`), and
// the `C M Y` coefficients of `a*` and `b*` each sum to zero so that a
// balanced composite gray is exactly neutral and the separation can be
// **solved** rather than chosen. The darkness coefficients — including
// `K = 0.70` — are the sibling's, unchanged. Paired source-to-destination the
// two profiles therefore differ in **one variable**, and the ΔE below is
// attributable to it rather than to a fixture that changed in three places at
// once.
//
// ★ THE CLOSED FORM, and why `G12` is a `DerivedExpectation` rather than a
// second self-comparison. Writing `ρ = 65 280/65 535` for the legacy-PCSLAB
// full-scale ratio and `k` for the input black:
//
// ```text
// source A2B0(0,0,0,k)  = (100(1 − 0.70k), 0, 0)          exactly, both edges affine
// destination separation reproduces that L* through the node coordinate
//                        = (ρ·100(1 − 0.70k), 0, 0)       the ρ gap and nothing else
// preserved answer       = (100(1 − 0.70K′), 2K′, 6K′)
// and K′ = k EXACTLY, because both profiles carry the same 0.70 darkness per
// unit of K and the equal-lightness construction is then the identity.
// ```
//
// So the expectation is `ΔE2000` between two Lab points **computed from the
// two recipes' constants**, with no implementation's output in it. That is the
// strongest class this subject admits — `ICC_Spec` **A51** rules out ground
// truth — and it is strictly stronger than the licensed headline, which is a
// self-comparison with no derivation available on a real press table.
//
// ★ WHAT THIS PAIR IS NOT. It is **not** a substitute for `G1`. Its number is
// a property of two authored models and says nothing about any press; `G1`'s
// is a property of two real press profiles and cannot be derived. They are
// different claims and the section keeps both.

/// The legacy-PCSLAB full-scale ratio, `65 280/65 535` — the gap between
/// `L* = 100` and full scale that clause 10.10's encoding leaves, and the only
/// term in `G12`'s closed form that is neither a recipe constant nor `1`.
const LEGACY_FULL_SCALE: f64 = 65_280.0 / 65_535.0;

/// `v2-cmyk-warm-black`'s darkness per unit of `K`, shared with its sibling —
/// which is what makes `K′ = k` exact.
const WB_K_DARKNESS: f64 = 0.70;
/// `v2-cmyk-warm-black`'s `a*` per unit of `K`.
const WB_K_A: f64 = 2.0;
/// `v2-cmyk-warm-black`'s `b*` per unit of `K`.
const WB_K_B: f64 = 6.0;

/// The committed warm-black destination, and the sibling used as the source.
const SYNTHETIC_WARM_BLACK: &str = "v2-cmyk-warm-black.icc";

/// **`G12`'s expectation, derived from the two recipes and nothing else.**
///
/// Returns the ΔE2000 a conformant consumer must observe between the
/// colorimetric and the preserved answer at input black `k`, on
/// `v2-cmyk-chromatic-neutral → v2-cmyk-warm-black` at media-relative.
///
/// See the section header for the derivation. No value here comes from
/// running anything.
#[must_use]
pub fn warm_black_expected_cost(k: f64) -> f64 {
    warm_black_expected_cost_scaled(k, LEGACY_FULL_SCALE)
}

/// The same derivation with the legacy full-scale ratio as a parameter, so
/// that `G12` can state what it would have observed under the **rival
/// reading** in which `L* = 100` is full scale (`scale = 1.0`) rather than
/// `FF00h`. That is a real misreading of clause 10.10 and the one this corpus
/// has caught before (DL-005, DL-011); it is worth `0.22` ΔE2000 at the white
/// end, twenty times `G12`'s bound.
#[must_use]
pub fn warm_black_expected_cost_scaled(k: f64, scale: f64) -> f64 {
    let l_src = 100.0 * (1.0 - WB_K_DARKNESS * k);
    let colorimetric = Lab {
        l: scale * l_src,
        a: 0.0,
        b: 0.0,
    };
    let preserved = Lab {
        l: l_src,
        a: WB_K_A * k,
        b: WB_K_B * k,
    };
    delta_e_2000(colorimetric, preserved)
}

/// **`G12`'s bound: one hundredth of a ΔE2000, COUNTED.**
///
/// The expectation is exact algebra, so everything between it and the
/// observation is encoding and printing. The two legs do not carry the same
/// terms and the count keeps them apart — the **colorimetric** leg passes
/// through the `B2A0` table and the **preserved** leg does not:
///
/// | term | leg | `L*` | `a*`/`b*` |
/// |---|---|---|---|
/// | half a legacy-PCSLAB sample quantum in the `A2B0` CLUT (`100/65 280` and `255/65 535`) | both | `7.7e-4` | `1.9e-3` |
/// | half a quantum of `B2A0` **device** output (`0.5/65 535`), carried by the `A2B0` coefficients — `1.40` in darkness, `140` and `190` in `a*` and `b*` | colorimetric | `1.1e-3` | `1.5e-3` |
/// | half a quantum on the PCS handed between the two legs | colorimetric | `7.7e-4` | — |
/// | the equal-lightness inversion reading the same 16-bit `L*(K)` ramp | preserved | `7.7e-4` | — |
/// | `iccce transform`'s six printed device decimals, through the same coefficients | colorimetric | `7.0e-5` | `1.0e-4` |
/// | `transicc`'s four printed decimals of Lab | both | `1.0e-4` | `1.0e-4` |
///
/// Colorimetric leg `≈2.7e-3` in `L*` and `≈3.5e-3` in each of `a*`, `b*`;
/// preserved leg `≈1.6e-3` and `≈2.0e-3`. Their difference is what ΔE2000
/// sees: `4.3e-3` in `L*`, `5.5e-3` in each chroma axis. Divided by ΔE2000's
/// own weighting (`S_L ≥ 1.29` and `S_C ≥ 1.15` everywhere on this ramp) and
/// combined in quadrature the counted sum is **`≈7.6e-3`**. The bound is **the
/// next power of ten above it**, because a bound stated to the precision of
/// the counting claims more than the counting supports.
///
/// ★ **The margin is thin, and that is the honest state.** Observed
/// `6.3e-3` at the time of writing, `0.63` of the bound — most of the count is
/// exercised, unlike `F4`'s, whose largest term was an allowance for a
/// conformant consumer that requantises the PCS. **A first draft of this bound
/// omitted the `B2A0` output-quantisation row and counted `6e-3`, below what
/// the run then observed.** It is corrected here rather than widened: a count
/// its own observation exceeds is not a bound, it is a coincidence.
pub const DERIVED_COST: Tolerance = Tolerance::new(
    1.0e-2,
    "ONE HUNDREDTH of a dE2000, COUNTED from the encoding and the printing, because the \
     expectation is exact algebra derived from two recipes' constants and nothing else can \
     stand between it and the observation. Per leg: half a legacy-PCSLAB sample quantum in the \
     A2B0 CLUT (7.7e-4 in L*, 1.9e-3 in a*/b*, BOTH legs); half a quantum of B2A0 DEVICE output \
     carried by the A2B0 coefficients (1.1e-3 and 1.5e-3, COLORIMETRIC leg only); half a \
     quantum on the PCS handoff (7.7e-4, colorimetric); the equal-lightness inversion's reading \
     of the same 16-bit ramp (7.7e-4, preserved); and the two programs' printing. Differenced, \
     divided by dE2000's own S_L >= 1.29 and S_C >= 1.15 on this ramp and combined in \
     quadrature: about 7.6e-3, rounded UP to the next power of ten. NOT perceptual: \
     TOLERANCES.md \u{a7}2's 1.0 anchor is two orders away and is irrelevant to it",
);

const G_WARM_ROWS: [(&str, Kind, Metric, Tolerance); 5] = [
    (
        "passk/G/synthetic-warm-black/cost/dE2000-max",
        SELF,
        DE,
        REPORTED,
    ),
    (
        "passk/G/synthetic-warm-black/cost/matches-the-DERIVED-closed-form",
        DE_KIND,
        DE,
        DERIVED_COST,
    ),
    (
        "passk/G/synthetic-warm-black/control/the-reference-leg-IS-sound",
        CC,
        DE,
        REFERENCE_LEG_SOUND,
    ),
    (
        "passk/G/synthetic-warm-black/separation/the-two-presses-render-the-same-K-DIFFERENTLY",
        OR,
        DE,
        PRESS_SEPARATION_FLOOR_MET,
    ),
    (
        "passk/G/synthetic-warm-black/separation/the-colorimetric-answer-DOES-lay-chromatic-ink",
        SELF,
        DEV,
        SEPARATION_FLOOR_MET,
    ),
];

const SRC_WARM: &str = "Pass K §G — fixtures/synthetic/v2-cmyk-chromatic-neutral.icc -> \
    fixtures/synthetic/v2-cmyk-warm-black.icc, both COMMITTED and unlicensed, driven through the \
    shipped iccce binary twice (differing only in --preserve-black) with the pinned lcms2 as the \
    ruler. Runs in CI. The expectation on the DERIVED row comes from the two recipes' constants \
    and no implementation's output";

/// §G's committed cost leg, measured on the fixture authored for it.
fn analyse_cost_warm_black(oracle: &Oracle, iccce: &Iccce) -> Result<CostLeg, Unavailable> {
    let src = need_synthetic(SYNTHETIC_SEPARATING)?;
    let dst = need_synthetic(SYNTHETIC_WARM_BLACK)?;
    analyse_cost_leg(
        oracle,
        iccce,
        "fixtures/synthetic/v2-cmyk-chromatic-neutral.icc -> \
         fixtures/synthetic/v2-cmyk-warm-black.icc [committed, unlicensed, runs in CI; the two \
         models differ in ONE variable, the chroma of the black ink]",
        &src,
        &dst,
    )
}

/// The largest departure of the measured cost from [`warm_black_expected_cost`]
/// over the same ramp `analyse_cost_leg` measured — recomputed here rather than
/// carried on [`CostLeg`], because it is meaningful for exactly one pair.
fn warm_black_derivation_residual(
    oracle: &Oracle,
    iccce: &Iccce,
) -> Result<(f64, f64, f64), Unavailable> {
    let src = need_synthetic(SYNTHETIC_SEPARATING)?;
    let dst = need_synthetic(SYNTHETIC_WARM_BLACK)?;
    let ramp = cost_ramp();
    let rows: Vec<Vec<f64>> = ramp.iter().map(|r| r.to_vec()).collect();
    let off = as_cmyk(
        iccce
            .transform_rows_shaped(&src, &dst, Intent::RelativeColorimetric, &rows, 4)
            .map_err(|e| Unavailable::Error(e.to_string()))?,
    )?;
    let on = as_cmyk(
        iccce
            .transform_rows_shaped_preserve_black(
                &src,
                &dst,
                Intent::RelativeColorimetric,
                &rows,
                4,
                PRESERVE_POLICY,
            )
            .map_err(|e| Unavailable::Error(e.to_string()))?,
    )?;
    let lab_off = to_lab(oracle, &dst, &off)?;
    let lab_on = to_lab(oracle, &dst, &on)?;
    let mut worst = 0.0_f64;
    let mut at_k = 0.0;
    let mut worst_rival = 0.0_f64;
    for (i, p) in ramp.iter().enumerate() {
        let seen = delta_e_2000(lab_off[i], lab_on[i]);
        let d = (seen - warm_black_expected_cost(p[3])).abs();
        if d > worst {
            worst = d;
            at_k = p[3];
        }
        worst_rival = worst_rival.max((seen - warm_black_expected_cost_scaled(p[3], 1.0)).abs());
    }
    Ok((worst, at_k, worst_rival))
}

fn cost_warm_black_records(
    c: &CostLeg,
    residual: (f64, f64, f64),
    licensed: Option<&Cost>,
    sibling: Option<&CostLeg>,
) -> Vec<Record> {
    let mut out = Vec::new();
    out.push(
        Record::graded(
            G_WARM_ROWS[0].0,
            G_WARM_ROWS[0].1,
            G_WARM_ROWS[0].2,
            G_WARM_ROWS[0].3,
            c.cost_max,
            SRC_WARM,
            format!(
                "★★★ THE COST OF THE POLICY, ON A COMMITTED PAIR, IN CI. {}: {:.6} dE2000 max at \
                 K = {:.2}, {:.6} mean, {:.6} minimum, over {} qualifying points at \
                 media-relative. Boundary step at one 8-bit code of cyan {:.6}. ★ It is NOT a \
                 substitute for the licensed headline ({}): this number is a property of two \
                 authored models and says nothing about any press. What it is: the same policy \
                 priced on a pair whose reference leg is sound (the row below), whose separation \
                 is real in BOTH senses, and whose answer is derivable in closed form",
                c.label,
                c.cost_max,
                c.cost_argmax_k,
                c.cost_mean,
                c.cost_min,
                c.points,
                c.boundary_step,
                opt_num(licensed.map(|l| l.cross.cost_max), 6)
            ),
        )
        .with_separation(Separation::against_distance(
            "the same measurement on the SIBLING fixture v2-cmyk-chromatic-neutral, whose black \
             ink is spectrally NEUTRAL: there the preserved answer at matched lightness is a \
             METAMER of the four-ink separation and the cost would be the encoding gap alone, \
             ~0.22, however much ink separates the two answers. It is a property of the FIXTURE \
             and is supplied, not derived",
            c.cost_min,
            c.cost_max - c.cost_min,
            SepUnits::SameAsMetric,
        )),
    );

    out.push(
        Record::graded(
            G_WARM_ROWS[1].0,
            G_WARM_ROWS[1].1,
            G_WARM_ROWS[1].2,
            G_WARM_ROWS[1].3,
            residual.0,
            SRC_WARM,
            format!(
                "★★★ THE ONLY ROW IN §G WHOSE EXPECTATION IS NOT AN IMPLEMENTATION'S OUTPUT. The \
                 measured cost departs from the closed form by at most {:.3e} (at K = {:.2}), \
                 against a counted bound of {:.0e}. The closed form is \
                 dE2000((rho*100(1-0.70k), 0, 0), (100(1-0.70k), 2k, 6k)) with rho = 65280/65535 \
                 — the colorimetric answer is the source's own lightness reproduced through the \
                 destination's node coordinate, the preserved answer is the destination's warm \
                 black at K' = k, and K' = k is EXACT because both recipes carry the same 0.70 \
                 darkness per unit of K. Nothing in the expectation was run",
                residual.0, residual.1, DERIVED_COST.value
            ),
        )
        .with_separation(Separation::against_distance(
            "the same comparison against the SIBLING's model — a spectrally neutral black, i.e. \
             WB_K_A = WB_K_B = 0 — under which the closed form collapses to the encoding gap \
             alone at every k. That is what this row would observe if the fixture's black ink \
             were not chromatic, and it is far outside the bound",
            (c.cost_max - c.cost_min).abs(),
            (c.cost_max - c.cost_min).abs() - residual.0,
            SepUnits::SameAsMetric,
        )),
    );

    out.push(
        Record::graded(
            G_WARM_ROWS[2].0,
            G_WARM_ROWS[2].1,
            G_WARM_ROWS[2].2,
            G_WARM_ROWS[2].3,
            c.round_trip,
            SRC_WARM,
            format!(
                "★★ THE GATE THE SIBLING FAILS, PASSED. The colorimetric answer lands {:.6} \
                 dE2000 from the colour the source asked for — against {} on the sibling pair \
                 (G9), which is the whole reason this fixture was authored. The residue is the \
                 legacy encoding's own 65280/65535 gap at the white end and nothing else: the \
                 destination's B2A0 neutral column is the SOLUTION of 'lay ink that reaches this \
                 darkness with a* = b* = 0', so the two tables invert each other by construction",
                c.round_trip,
                opt_num(sibling.map(|y| y.round_trip), 6)
            ),
        )
        .with_separation(Separation::against_distance(
            "the same control on the sibling pair (G9), which is what this fixture was authored \
             to move",
            sibling.map_or(f64::NAN, |y| y.round_trip),
            sibling.map_or(f64::NAN, |y| y.round_trip - c.round_trip),
            SepUnits::SameAsMetric,
        )),
    );

    let press_shortfall = (PRESS_SEPARATION_FLOOR - c.sep_press).max(0.0);
    out.push(Record::graded(
        G_WARM_ROWS[3].0,
        G_WARM_ROWS[3].1,
        G_WARM_ROWS[3].2,
        G_WARM_ROWS[3].3,
        press_shortfall,
        SRC_WARM,
        format!(
            "The two committed profiles render the same (0,0,0,K) device values up to {:.4} \
             dE2000 apart, against a floor of {:.1}; the observed value is the shortfall. The \
             separation IS the varied variable: the source's black is spectrally neutral, the \
             destination's is warm, and at K = 1 they differ by exactly the chroma (2, 6) the \
             recipe puts there",
            c.sep_press, PRESS_SEPARATION_FLOOR
        ),
    )
    .with_separation(Separation::against_distance(
        "a destination whose black ink were spectrally NEUTRAL, as the sibling recipe's is, with \
         the same 0.70 darkness: the two profiles' K axes would then COINCIDE, this quantity \
         would be exactly zero and the shortfall would be the whole floor. A property of the \
         fixture pair, so it is supplied and not derived",
        PRESS_SEPARATION_FLOOR,
        c.sep_press,
        SepUnits::Other("dE2000 between two profiles' renderings, not a shortfall"),
    )));

    let sep_shortfall = (SEPARATION_FLOOR - c.sep_device).max(0.0);
    out.push(Record::graded(
        G_WARM_ROWS[4].0,
        G_WARM_ROWS[4].1,
        G_WARM_ROWS[4].2,
        G_WARM_ROWS[4].3,
        sep_shortfall,
        SRC_WARM,
        format!(
            "The colorimetric answer lays down {:.6} of chromatic ink at its worst point, against \
             §F's declared floor of {:.0e}; the observed value is the shortfall. This is the \
             device-unit gate §F already had, restated on the new fixture so that a future edit \
             to the recipe cannot quietly turn the pair into a K-only-out destination and leave \
             the dE rows reporting zero for the wrong reason",
            c.sep_device, SEPARATION_FLOOR
        ),
    )
    .with_separation(Separation::against_distance(
        "the fixture E6 disqualified (v2-cmyk-mft2-lab as the destination), whose B2A0 emits \
         [0,0,0,k] at every node: there the colorimetric answer is already K-only, this quantity \
         is exactly zero and the shortfall would be the whole floor",
        SEPARATION_FLOOR,
        c.sep_device,
        SepUnits::Other("device units of chromatic ink, not a shortfall"),
    )));

    out
}

// ===========================================================================
// The one-line note
// ===========================================================================

/// One line for the report's `note` block. Every number in it is formatted from
/// [`Bundle`], never typed — §3.5.8.6's rule, after three claim-bearing
/// literals in this crate went false inside a day.
#[must_use]
pub fn note(b: &Bundle) -> String {
    let mut parts = Vec::new();
    if let Some(x) = &b.baseline {
        parts.push(format!(
            "BASELINE (ISO Coated v2 300% (ECI), {} K-only points, media-relative): chromatic \
             ink {:.6}, TAC {:.6}, |dK| {:.6}, dE2000 from the K-only build {:.4} — the colour \
             is right and the ink is not; vs lcms2 {:.3e}",
            x.points,
            x.per_intent[0].1,
            x.per_intent[0].2,
            x.per_intent[0].3,
            x.per_intent[0].4,
            x.vs_oracle
        ));
    }
    if let Some(s) = &b.sweep {
        let holds = s.iter().filter(|r| r.saturation <= NEARLY_K_ONLY).count();
        parts.push(format!(
            "saturation is already K-only on {}/{} real CMYK destinations",
            holds,
            s.len()
        ));
    }
    if let Some(g) = &b.gray {
        if let Some(a) = g.first() {
            parts.push(format!(
                "GWG legs: {:.6} apart in device, {:.4} dE2000 apart in colour on the press's \
                 own gray; {:.4} dE2000 on an ordinary gamma-2.2 gray",
                a.device_distance,
                a.colorimetric_distance,
                g.get(1).map_or(f64::NAN, |r| r.colorimetric_distance)
            ));
        }
    }
    if let Some(m) = &b.model {
        parts.push(format!(
            "oracle model: cell residual {:.3e}, K re-mapping up to {:.6}",
            m.cell_model_residual,
            m.ktone.iter().map(|(_, v)| *v).fold(0.0_f64, f64::max)
        ));
    }
    if let Some(g) = &b.gate {
        parts.push(format!(
            "GATE (repointed 2026-08-18 at --preserve-black {}): k-only-in-implies-k-only-out \
             observed {:.6} against a required 0; leak on/off over {} non-qualifying probes \
             {:.6}; K identity on a same-profile pair {:.6} (the oracle is {:.3e} away from \
             it); cross-press K at the oracle's own CLUT nodes {:.3e} with the copy-K rival \
             {:.3e} away; regression guard {:.3e} node-aligned vs {:.3e} off-node",
            PRESERVE_POLICY,
            g.chromatic,
            2 * g.node_points,
            g.leak,
            g.identity_k,
            g.identity_oracle_rival,
            g.xp_k_at_nodes,
            g.xp_copy_rival_at_nodes,
            g.node_aligned,
            g.arbitrary
        ));
    }
    if let Some(f) = &b.separating {
        parts.push(format!(
            "§F on the COMMITTED v2-cmyk-chromatic-neutral (no licence, runs in CI): the \
             fixture's own table separates the two candidate answers by {:.6} in device units \
             (floor {:.0e}), dead-band spread {:.1e}, column residual {:.3e}",
            f.separation, SEPARATION_FLOOR, f.dead_band_spread, f.column_residual
        ));
    }
    if let Some(r) = &b.separating_run {
        parts.push(format!(
            "§F GATE (repointed 2026-08-18, and this one runs in CI): \
             k-only-in-implies-k-only-out observed {:.6} against a required 0 on the \
             COMMITTED fixture; the chromatic-gray guard that had to survive the feature \
             is {:.3e} vs the derived table and {:.3e} vs lcms2 over {} points, and the \
             leak guard on the same points is {:.6}",
            r.chromatic, r.gray_vs_derived, r.gray_vs_oracle, r.gray_points, r.leak
        ));
    }
    if let Some(c) = &b.cost {
        parts.push(format!(
            "§G COST OF THE POLICY ({}, {} qualifying points, media-relative): {:.6} dE2000 max at K = {:.2}, {:.6} mean — against {:.6} on the SAME-PRESS pair and {:.6} on the same pair reversed. The pair separates: {:.6} chromatic ink in the colorimetric answer, {:.4} dE2000 between the two presses' own K rendering, reference leg sound to {:.4}. Boundary step at one 8-bit code of cyan {:.6}",
            c.cross.label,
            c.cross.points,
            c.cross.cost_max,
            c.cross.cost_argmax_k,
            c.cross.cost_mean,
            c.same_press.cost_max,
            c.reversed.cost_max,
            c.cross.sep_device,
            c.cross.sep_press,
            c.cross.round_trip,
            c.cross.boundary_step
        ));
    }
    if let Some(c) = &b.cost_synthetic {
        parts.push(format!(
            "§G on the COMMITTED pair (runs in CI): DISQUALIFIED as a cost fixture — its reference leg is {:.6} dE2000 wrong, larger than the {:.6} it would have reported",
            c.round_trip, c.cost_max
        ));
    }
    if let Some(p) = &b.cost_population {
        parts.push(format!(
            "\u{a7}G POPULATION: {} of {} ordered pairs of the six real CMYK members are ENTITLED to \
             price the policy, and {} of those find it imperceptible; among them the cost runs \
             {:.6} ({}) to {:.6} ({}). Without the entitlement filter {} of {} would read as \
             imperceptible",
            p.entitled,
            p.pairs,
            p.imperceptible_entitled,
            p.smallest.0,
            p.smallest.1,
            p.largest.0,
            p.largest.1,
            p.imperceptible_all,
            p.pairs
        ));
    }
    if let Some((c, res)) = &b.cost_warm_black {
        parts.push(format!(
            "§G on the COMMITTED warm-black pair authored for it (runs in CI): cost {:.6} dE2000 max at K = {:.2}, {:.6} mean; reference leg sound to {:.6}; press separation {:.4}; the measurement departs from the CLOSED FORM derived from the two recipes by {:.3e} at K = {:.2}, bound {:.0e}",
            c.cost_max,
            c.cost_argmax_k,
            c.cost_mean,
            c.round_trip,
            c.sep_press,
            res.0,
            res.1,
            DERIVED_COST.value
        ));
    }
    if parts.is_empty() {
        parts.push("nothing ran".to_string());
    }
    for u in &b.unavailable {
        parts.push(format!("UNAVAILABLE: {u}"));
    }
    parts.join("; ")
}
