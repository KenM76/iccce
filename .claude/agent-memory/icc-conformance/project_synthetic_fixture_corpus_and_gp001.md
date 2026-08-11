---
name: project-synthetic-fixture-corpus-and-gp001
description: fixtures/synthetic now exists (38 profiles from tools/gen-profiles, verify-able byte-for-byte); building it produced FINDING GP-001 — iccce mis-counts mBA curve sets — plus three ICC_Spec corpus gaps closable from the PDF.
metadata:
  type: project
---

**State as of 2026-08-11.** `tools/gen-profiles` (standalone, zero-dep, own
empty `[workspace]`) and `fixtures/synthetic/` (38 committable profiles:
12 well-formed, 26 one-defect-each malformed) exist. This **supersedes** the
"`tools/gen-profiles` still open" line in
[[project-oracle-and-tolerance-state]].

**Why:** Pass 2 done-when clause 2 was PARTIAL — in-test synthetics are
tag-level and unusable outside `cargo test`.

**How to apply:**

- `gen-profiles verify <dir>` regenerates in memory and compares byte for
  byte; it is the reason nothing in that crate may read a clock, an env var,
  or an RNG. `MANIFEST.md` is generated, never hand-edited.
- The crate must **never** gain a dependency on `iccce-*`. A fixture written
  with the parser's own encoder cannot detect a *shared* misreading of the
  spec. (`tools/difftest`'s path-dep on `iccce-color` is a different case and
  is justified in its manifest; do not reason by analogy.)
- Nothing in that corpus is a colorimetric reference. Colorants are an
  arbitrary split of the *encoded* D50 white chosen so the integers sum to it
  exactly — a structural invariant, not colorimetry.

**★ FINDING GP-001 (open at filing).** `crates/iccce-profile/src/lut.rs`
`decode_lut_ab` counts **B and M by `output_chan` and A by `input_chan` for
BOTH tag types**. Correct for `mAB `; wrong for `mBA `. ICC.1:2022 **10.13.2 /
10.13.4** say B and M are counted by **input** channels and **10.13.6** says A
by **output** — so a CMYK `B2A0` (3 in, 4 out) has B=3, M=3, A=4. iccce refuses
it with `curve chain broken at element 3 (byte 68)`. lcms2 agrees with the spec
(`Type_LUTB2A_Read`) and converts through the same fixture. **The defect is
invisible whenever `inputChan == outputChan`, i.e. on every square LUT, and
appears on every real CMYK B2A0** — which is exactly the population the
40-profile machine sweep lacked.

**★ Three ICC_Spec corpus gaps found while authoring bytes** (report to
`icc-spec-librarian`; do not edit that tree directly):

1. `icc__type__lutAtoB_lutBtoA.md` carries **one blanket sentence for both
   types** ("A = inputChan; B and M = outputChan") — the likely origin of
   GP-001. Needs 10.12.2/4/6 and 10.13.2/4/6 transcribed **per type**.
2. **A23 is closable**: 10.12.1/10.13.1 enumerate the permitted element
   combinations verbatim (`B`; `M,Matrix,B`; `A,CLUT,B`; `A,CLUT,M,Matrix,B`
   for `mAB `, mirrored for `mBA `), plus "At least one processing element
   shall be included".
3. **A25 is closable**: 10.15 states the `mluc` fallback (same language code,
   else the first record).

Also still stale in that tree: `icc__type__lut8_lut16.md` §"facts that gate
correctness" and `icc__s__pcs_encoding.md` §2 **still say the legacy-Lab
selector is `header.version`**, contradicting the A1 resolution at the top of
the same file. See [[project-lcms2-findings-legacy-lab-and-forced-bpc]].

**One divergence the corpus now pins:** lcms2's `transicc` **accepts** a major
version 5 profile (`iccmax-version.icc`); iccce refuses iccMAX by name. Not a
defect on either side — a deliberate difference that now has a fixture.
