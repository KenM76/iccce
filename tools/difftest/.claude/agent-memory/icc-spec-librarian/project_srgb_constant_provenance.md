---
name: project-srgb-constant-provenance
description: State of sRGB's constants after the 14th pass (2026-08-17) — what is now cross-verified, the C0-vs-C1 explanation of the 0.03928 divergence, and the one thing still owed
metadata:
  type: project
---

**As of 2026-08-17 sRGB's constants are independently sourced well enough to
build a computed sRGB destination on. The oracle-contamination risk that blocked
`docs/NEXT_SESSION.md` §3.0 is resolved for the primaries, the white point, the
matrices and the transfer function.**

**Why:** the whole corpus previously took sRGB from lcms2, which is also
iccce's differential oracle — so a computed sRGB would have taken its white
point from the implementation it is meant to be checked against
(`NUMERIC_CLAIMS.md` shared-misreading risk = ELEVATED). Three non-lcms2
documents are now held: **ITU-R BT.709-6** (primaries + D65, `primary_spec`),
**W3C CSS Color 4** (both breakpoints, exact-rational matrices, permissive
licence), **W3C sRGB 1996** (superseded, self-disavowed).

**How to apply:**
- **Do not re-litigate the `0.03928` / `0.04045` divergence** — it is solved,
  not merely recorded. The 1996 pair is the **C¹** (value *and slope*
  continuity) solution with `a` rounded to `0.055`; IEC's is the **C⁰**
  (value-only) solution with `a` pinned at `0.055`. **`0.04045` IS the
  continuity-solved value**, closing to `2.33×10⁻⁹`. Anyone "fixing" sRGB with
  `a = 0.0550107` has silently left the standard.
- **Two things are still owed, and they are different in kind.**
  **(1)** `0.04045` from a *standards* text — CSS Color 4's own normative
  reference for sRGB is the IEC paywall, so it is a restatement.
  **(2) ★ NO document publishes sRGB's D50-adapted colorants.** Only the HP 1998
  profile does, and it is **not reconstructible** from the sourced
  chromaticities under ICC.1 Annex E.3 Bradford — 8 of 9 cells land within 2 ULP
  of `s15Fixed16`, `bXYZ.Z` misses by **12 ULP** at any known D50 tier.
- **Consequence for any sRGB-building work:** a from-constants sRGB is
  defensible, but **will not be byte-identical to the HP profile.** A
  byte-equality test against `sRGB Color Space Profile.icm` tests HP's 1998
  arithmetic, not iccce's. Use a ΔE round-trip and name the ~`2×10⁻⁴ XYZ`
  blue-Z construction difference as an approximation under project rule 4.
- **Compute the sRGB→XYZ matrix from the chromaticities; do not transcribe it.**
  The Grassmann construction reproduces CSS Color 4's published exact rationals
  *identically* (`Fraction` equality), so a unit test asserting that is a real
  external check.
- Both remaining items are plausibly closed by one operator browser download —
  see [[reference-color-org-agent-bar-is-permanent]].

Files: `D:\Dev\Rag-Specialized\ICC_Spec\iec\iec__s__srgb.md` (rewritten),
`itu\itu__s__bt709.md`, `w3c\w3c__s__css_color_4.md`.
