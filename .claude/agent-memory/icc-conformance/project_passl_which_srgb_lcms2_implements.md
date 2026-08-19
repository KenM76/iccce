---
name: project-passl-which-srgb-lcms2-implements
description: Pass L — lcms2 implements the C0 (0.055/0.04045) reading of sRGB, measured at float precision; the interior maximum is in a DIFFERENT PLACE for each output space; the oracle is provably unusable as a destination ruler; and "0 of 256 8-bit codes change" is FALSE end-to-end.
metadata:
  type: project
---

Measured **2026-08-19**, oracle pin `21c582a` (lcms2 2.19.1, MSVC, Windows 11).
Apparatus `tools/difftest/src/passl.rs` + `src/bin/passl_probe.rs`; record
`tools/difftest/README.md` **§27**. **20 records, all pass.** Whole suite
`pass=373 fail=0 skip=9 error=0`.

**The subject.** `ICC_Spec` row **`A57`**: ICC/W3C/Khronos print sRGB with
`α = 1.055`, `d = 0.04045` (**C⁰**); `Rec. ITU-T H.273 (V4) | ISO/IEC 23091-2`
clause 8.2 *defines* `TransferCharacteristics = 13`'s constants by **slope**
continuity too (**C¹**, `α = 1.0550107189…`, `d = 0.0392933707`). `iccce` ships
both as `SrgbTrc`. **A57 stays OPEN** — Pass L measures an *implementation*.

**Verdict: lcms2 implements C⁰.** Blackbox `max |lcms2−C0| = 5.300706e-5 L*`
(below one printed quantum) against `max |lcms2−C1| = 1.230354e-3` — 23.2x —
and **204 of 204** resolvable probes favour C⁰. Whitebox agrees:
`src/cmsvirt.c` `Build_sRGBGamma()` L640-647.

---

## ★★★ The interior maximum is instrument-specific, not just interior

The known trap (`builtin.rs`'s
`tests::breakpoint_is_the_c0_solution_not_the_1996_value`) is "the max is
interior, not at the boundary". Pass L found a **second layer**: the max is in
a **different place for every output space**, and the number the FEATURE's own
doc comment prints is the wrong one.

| probe | `L*` sep | printed quanta |
|---|---|---|
| C¹ breakpoint, code 10.0198 | **1.4e-14 — exactly zero** | 0 |
| C⁰ breakpoint, code 10.3148 | 6.93e-4 | 6.9 |
| **`L*` max, code 23.5136** | **1.202916e-3** | **12.0** |
| **linear-light max, code 142.9057** | 4.376e-4 | 3.6 |

★ **The C¹ breakpoint has EXACTLY zero power and it is structural**: H.273
clause 8.2 *defines* `β` by value continuity with the same linear segment C⁰
uses, so **the two curves meet there by construction**. A "test at the
boundary" here is not weak, it is void.

★ **The two maxima are 119.4 codes apart**, and probing at the linear-light
maximum — `4.777e-6`, the number `SrgbTrc`'s doc comment prints — keeps only
**36 %** of the `L*` signal. *The quantity a feature advertises is the wrong
place to probe when the instrument reports in different units.* Guarded by a
row (`passl/A/design/l-signal-at-the-linear-light-max`, bound 0.5) rather than
by a paragraph.

## ★★ The oracle is NOT a usable ruler for a destination's `A2B`, and this
## was caught by a nonsense GAIN, not by a wrong number

lcms2's `EvaluateCLUTfloatIn16` (`src/cmslut.c` L445-456) calls
`FromFloatTo16` **before** `Lerp16`: a `lut16Type`/`mft2` CLUT **quantises its
float input to 16 bits** even in a float pipeline. Measured staircase on
`USWebCoatedSWOP.icc`: **7 distinct `L*` over 60 samples** spanning 2.95e-2 %
ink; tread ≈**4.9e-3 % ink**. The reading choice moves a destination ink by at
most 6.36e-3 % — **≈1.3 treads**.

★★ **How it was caught.** The first end-to-end attempt used the oracle as the
ruler and returned a plausible `6.58e-3` dE2000 — but the *gain* over the PCS
difference ranged **0.00 to 36.44 across neighbouring probes**. *An
amplification factor that varies by four orders of magnitude between adjacent
points is an apparatus fault, not a finding.* Same shape as §19's `/100` bug
(see [[project-pass5c-estimator-branch-finding]]): **a residual that is
implausible under every hypothesis is the apparatus.**

Consequence: §C runs **entirely in process, in f64**. It has to anyway — ★ the
shipped `iccce` binary **has no flag that selects `SrgbTrc`**, so the C¹
reading is reachable *only* from the library API. Same precedent as pass5b.

## ★★★ "0 of 256 8-bit codes change" is true of sRGB and FALSE end-to-end

`SrgbTrc`'s doc comment records **0 of 256** — correct, and it is about
**sRGB's own encoding**. Through a real destination:

| destination | dev sep (0..1) | 8-bit codes changed | 16-bit |
|---|---|---|---|
| `v2-cmyk-mft2-lab.icc` (committed) | 1.193e-5 = 1/329 code | **2 / 5169** | 1145 |
| `USWebCoatedSWOP.icc` | 6.361e-5 = 1/62 code | **14 / 5169** | 3022 |
| `AdobeRGB1998.icc` | 8.096e-6 = 1/484 code | **11 / 5169** | 2893 |

Half-step-offset grid gives 17 and 6 → **real, not a grid artefact**. Mechanism
is *not* amplification: the movement is a fraction of one code, but ≈0.3 % of
points sit on a rounding boundary. **A separation far below one code still
flips codes.**

Also false in the same doc comment: "below one 16-bit PCS quantum". True in the
**encoded** domain, false as dE2000 in the PCS — one 16-bit `L*` quantum *at
the argmax point* is `9.262e-4` dE00, so the max is **2.01x a PCS quantum**.
(Evaluate the quantum AT the point; dE2000's `SL` varies 1.6x from `L*` 6 to
50.)

## ★★ The cost of the choice, and where the gray ramp lies

Self-comparison, 55 938 probes + coordinate descent:

- **PCS max `1.857907e-3` dE2000** at rgb `(0.0392993, 0.0932081, 0.0392992)`
  = codes `(10.0213, 23.7681, 10.0213)`; mean `3.630359e-4`.
- **Neutral ramp only: `7.395940e-4` — 40 % of the true cost.** The maximum is
  **off-axis**, where R and B sit essentially ON the C¹ breakpoint (their own
  contribution zero) and G is at the `L*` junction. **dE2000's chroma/hue terms
  put the worst case where a 1-D probe cannot reach it** — and a transfer
  function is exactly the subject you would probe with a gray ramp.
- End-to-end `→ SWOP → sRGB → Lab`: **`2.207972e-3`**, i.e. the destination
  **AMPLIFIES by 1.52x**. The committed synthetic pair **attenuates by 0.51x**
  — so amplification is a property of the destination, and the record computes
  the word from the ratio rather than typing it (a first draft typed
  "AMPLIFIES" unconditionally and printed it beside `0.51x`; see
  [[project-stale-claim-strings-in-emitted-records]]).

## Apparatus notes worth reusing

- **`transicc` prints `%.4f` for every float** (`PrintFloatResults` L694-698).
  That is *the* quantum; no flag widens it. Deriving every tolerance from it
  means no tolerance in the pass is a chosen number.
- **XYZ output is printed ×100**; RGB/gray ×255; inks ×1 with the formatter
  dividing by 100. (Extends the §19 scaling lesson to XYZ.)
- **Model the ORACLE, not the standard.** §A uses lcms2's own `f()` and neutral
  probes only, so `Y_PCS = TRC(v)` identically and nothing else can be modelled
  wrong. The apparatus row is graded **where the two candidates are the same
  function** — validating the model while being structurally unable to answer
  the question, and it prints `ZERO-SEPARATION`, correctly.
- **A row that grades the INSTRUMENT.**
  `passl/A/lab/rival-reading-is-rejected-by-two-printed-quanta` is
  `(2·quantum)/residual_vs_rival ≤ 1`. It reds when the measurement loses
  discriminating power — earlier and different from lcms2 changing its answer.
- `cargo fmt --check` inside `tools/difftest` is **not clean at HEAD** (100+
  pre-existing sites under this rustfmt). Check only your own files; do not
  sweep.

Related: [[project-pass5c-estimator-branch-finding]],
[[project-candidate-separation]],
[[project-stale-claim-strings-in-emitted-records]],
[[project-oracle-and-tolerance-state]], [[project-passg-tolerance-lessons]].
