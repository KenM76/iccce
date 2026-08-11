---
name: project-lcms2-findings-legacy-lab-and-forced-bpc
description: Two measured facts about lcms2 2.19.1 — it keys legacy PCSLAB decoding on the TAG TYPE (not version, contra the corpus), and it silently forces BPC on for v4 profiles at perceptual/saturation.
metadata:
  type: project
---

Both measured **2026-08-11** at pin `21c582a` (lcms2 2.19.1) with the
`legacy_lab_probe` binary in `tools/difftest`, and corroborated by reading
the pinned source. Full record: `tools/difftest/README.md` §12.

**1. The legacy PCSLAB encoding selector — the corpus was wrong about
lcms2.** `ARCHITECTURE.md` DL-011 recorded, unverified, that lcms2 keys
the legacy `0xFF00`-full-scale Lab decoding on `header.version`. It does
not. `cmsio1.c` `_cmsReadInputLUT` / `_cmsReadOutputLUT` /
`_cmsReadDevicelinkLUT` all test
`_cmsGetTagTrueType(...) != cmsSigLut16Type` and never consult the
version; `namedColor2Type` paths insert the stage unconditionally.
Measured on four synthetic profiles differing only in the version word:
all decode legacy. **So DL-011's predicted iccce-vs-lcms2 divergence does
not exist on this pin.** `cmsLabEncoded2FloatV2` is called only from
`cmspack.c` (a pixel formatter), never from profile reading.

**2. lcms2 forces BPC on for v4 profiles at perceptual and saturation.**
`cmscnvrt.c` `_cmsLinkProfiles`, on the authority of "Adobe's document",
sets `BPC = TRUE` when `cmsGetEncodedICCversion >= 0x4000000` and the
intent is perceptual or saturation — whether or not the caller asked. The
black point comes from the fixed `cmsPERCEPTUAL_BLACK_*` constants
(cmssamp.c). Effect measured at ≈**3.15 `L*` at black**, and confirmed
quantitatively by transcribing lcms2's own
`ComputeBlackPointCompensation` and predicting the observation to 3×10⁻⁵.

**Why these matter:** any lcms2 cross-check at perceptual or saturation
against a v4 profile is measuring a transform with BPC in it. A tolerance
set without knowing that is set on the wrong quantity. This lands on
Pass 4 (intents) and Pass 5 (BPC).

**Method lessons worth reusing:**
- Make the fixtures **byte-identical except the one variable**, and assert
  it at run time before believing any result.
- **Run the control** — the case where both hypotheses agree. An apparatus
  not shown able to detect the effect is not an experiment.
- When a confound appears, **predict it quantitatively** from the other
  implementation's own arithmetic. An arm-comparison that comes back null
  may be null *by construction* (the `-b`-on-v2 arm was), and that must be
  recorded as inconclusive rather than read as a refutation.
- `-o*Lab2` vs `-o*Lab4` in `transicc` prints **identical** values — the
  built-in v2/v4 Lab distinction is invisible at the float boundary, so
  the obvious version of this experiment measures nothing.

Related: [[project-oracle-and-tolerance-state]].
