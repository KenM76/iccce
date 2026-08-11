---
name: project-lcms2-findings-pass3-quantisation-and-clamping
description: Two measured facts about lcms2 2.19.1 from Pass 3 — it quantises TABULATED tone curves to 16 bits even in float (explains ~all iccce-vs-lcms2 disagreement), and it returns device values >1.0 in float when the destination TRC inverse is analytic.
metadata:
  type: project
---

Both measured **2026-08-11** at pin `21c582a` (lcms2 2.19.1) during the
Pass 3 differential. Full record: `tools/difftest/README.md` §13.4 and
§13.6.1.

**1. lcms2 quantises tabulated tone curves to 16 bits IN ITS FLOAT
PIPELINE.** `cmsgamma.c`, `cmsEvalToneCurveFloat`: when a curve has
`nSegments == 0` (i.e. a sampled `curv` table, no analytic form) it does
`In = _cmsQuickSaturateWord(v*65535.0); Out = cmsEvalToneCurve16(...);
return Out/65535.0` — **rounding both the input and the output to
1/65535**, where iccce interpolates in `f64` throughout.

This is not a footnote; it is **essentially the whole** iccce-vs-lcms2
disagreement on a profile whose TRCs are tables. Emulating it inside
iccce's model (`Q(TRC(Q(x)))`, `Q(v)=round(v·65535)/65535`) shrank the
device-space residual from **6.71e-5 to 2.31e-7** — a factor of 290, and
*below* transicc's own print floor of 1e-4/255 = 3.92e-7.

**Consequence:** any cross-check whose source profile has sampled `curv`
TRCs is measuring this. The Windows system sRGB profile has 1024-entry
tables; Adobe RGB (1998) has single-value gammas (analytic, unaffected).
Check which before setting a tolerance.

**2. lcms2 returns device values OUTSIDE [0,1] in float** — measured up
to 1.000120 — when the destination TRC's inverse is **analytic** (a
gamma: `pow(1.000106, 1/γ)` is finite and nothing forces it back). In the
reverse direction, where the destination TRC inverse is a **tabulated**
reverse curve, it **does** saturate. So it tracks which inversion path
lcms2 took, not a stated range policy. iccce clamps per Annex F.8–F.16.
~~**Recorded as a FINDING; the spec question is OWED to
`icc-spec-librarian`.**~~ **★ SETTLED 2026-08-11 (later), and against the
hypothesis:** clause **6.4 is about the PCS**, not device values — the device
clause is **6.5**, whose float32 permission is doubly gated to `DToBx`/`BToDx`
tags that a matrix/TRC profile may not contain. A conforming F.8–F.16
evaluation **cannot** exceed 1,0, so lcms2's 1,000 120 means its *input* clamp
was skipped, and iccce is not "stricter". Hedges: clause 5 binds a CMM only to
**reading** profiles (**A39b**) so the word is *divergence*; the **v2** half is
**unsourced** (**A39c**). **The size of the divergence under real
out-of-gamut input is still unmeasured** — Pass 4 did not close it (0 of 1023
excursions; that destination's TRC inverse is tabulated). Corrected in
`TOLERANCES.md` §5.2 and `tools/difftest/README.md` §13.10 item 1.

**Method lessons that paid off again:**
- **Predict the confound quantitatively from the other implementation's
  own arithmetic**, then measure the residual. A tolerance's `why` string
  is a claim; test it.
- **Run the sensitivity control.** For any check that claims it would
  catch X, compute what its metric would read if X happened.
- **Report device-space AND ΔE.** They tell different stories: near black
  the device metric explodes (unbounded inverse-gamma slope) while ΔE
  stays small; at white the ΔE is structurally blind to out-of-range
  device codes while the device metric sees them.

Related: [[project-oracle-and-tolerance-state]],
[[project-lcms2-findings-legacy-lab-and-forced-bpc]],
[[project-encoded-white-points-differ-between-profiles]].
