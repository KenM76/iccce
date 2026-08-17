---
name: feedback-compatibility-not-compliance
description: Target compatibility with print-industry standards, never certified compliance — do not stall work because certification would need measurement hardware
metadata:
  type: feedback
---

**When an industry standard's conformance programme requires physical
testing — a press run, a proof, a spectrophotometer — aim for
*compatibility* with the standard and keep going. Do not treat the
unavailable certification path as a reason to stop.**

**Why:** Ken said this on 2026-08-17 while handing over the Ghent PDF
Output Suite 5.0: *"I know some things you stopped work on because they
required physical testing that we don't have. We aren't going to aim for
compliance like that. Just aim for compatibility."* This project has a
standing habit of parking work whose verification looked unreachable;
that habit was over-applied. Certification and capability are different
things, and the second is achievable alone.

**How to apply:** split any such standard into the part that needs a
measurement instrument and the part that does not, and do the second
part properly. The line is usually sharper than it first looks — for the
Ghent suite, certification needs proofing hardware, but "does the parser
accept every profile real producers embed?", "is a declared source
profile actually honoured?" and "do a vendor's v2 and v4 encodings of one
space agree?" are all answerable on this machine with no instrument at
all. State the resulting claim honestly: *compatible with*, never
*certified against*.

This does **not** loosen the evidence rules. Rule 3 still holds — a
compatibility claim still names its oracle, and a self-consistency check
is still weaker than a cross-check, which is still weaker than ground
truth. Compatibility changes *what* gets claimed, not *how well* it has
to be supported.

Related: [[project-ghent-compatibility]].
