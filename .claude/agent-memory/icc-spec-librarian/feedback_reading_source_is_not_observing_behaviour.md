---
name: reading-source-is-not-observing-behaviour
description: A claim that an implementation DOES X requires running it; citing its source files only supports "this text exists" — the C3 retraction, where the corpus predicted an lcms2 divergence that measurement found absent
metadata:
  type: feedback
---

**"Implementation Y does X" is a measurement. Citing Y's source is evidence that the text exists in the file, not that the program behaves that way.** Dead code, an upstream guard, an overridden default, or a comment that lies all sit between the two.

**Status: BINDING, corpus-wide, 2026-08-11** — `LEGAL_NOTE.md` §4 rule 7, plus a new evidence tier `measured_impl_behaviour` in §3 (conditions: commit pin not a mutable tag, platform, date, discrimination bound, stated scope) and a new id namespace **`M<n>`** living in `icc/icc__ref__lcms2_measured_behaviour.md` — a file whose *name* makes mis-citation as a rule difficult.

**Why:** `icc__ref__v2_v4_divergence.md` D2 asserted "lcms2 keys the legacy Lab encoding on the profile version", cited `_cmsReadInputLUT` and `cmsGetEncodedICCversion`, called it a **live disagreement with lcms2** under project rule 7, and required iccce to **log a runtime divergence warning**. `icc-conformance` then ran it (four synthetic `mft2`/Lab profiles, three byte-identical but for the version word): lcms2 2.19.1 at pin `21c582a` keys on **tag type**, exactly as ICC.1:2022 6.3.4.2 NOTE 3 says. Worst deviation 2×10⁻⁵. **Spec and lcms2 agree; the divergence never existed; the warning would have fired on agreement.** Filed as **C3** (`icc__ref__spec_defects.md` §11).

**The trap that produced it, worth knowing because it is still there:** `cmsio1.c` lines 357, 608, 763 each read `// Check profile version and LUT type` **directly above a test that checks only the tag type**. The word "version" appears three times in exactly the right place. The inference was not careless — it was the reading the comments invite. And lcms2 *does* key a v2/v4 decision on the version, just elsewhere: it **forces BPC on for v4 perceptual/saturation** (M2, ≈3.15 L\* at black), so a version-keyed effect genuinely exists in the neighbourhood.

**How to apply:**
- Before writing "lcms2 does X", ask: did anything execute? If not, write "lcms2's source at `<pin>` contains X" — a weaker and different sentence — and file it `impl_crosscheck`.
- **Never predict a divergence and write requirements against the prediction.** Project rule 7 engages on *observed* disagreement only.
- When a measurement lands, state the **scope not covered** as prominently as the result. M1 did not measure `ncl2` or B2A; that residue is now a gap row in `index.md`, not a silent assumption.
- Related: [[label-the-predicate-not-just-the-payload]] (same paragraph, C1 — the *rule* half) and [[derived-values-need-a-second-pass]] (C2). With n=3 the pattern is visible: **each defect was caught by a different check than the one that let it through, and two of the three were caught by executing code.** Labels route a claim to the check that can falsify it; they are not the check.
