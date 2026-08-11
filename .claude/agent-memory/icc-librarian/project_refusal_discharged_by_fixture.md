---
name: iccce-refusal-discharged-by-fixture
description: DL-020 — a rule the corpus can't supply is refused by name, not guessed, and only an independently authored fixture can discharge the doubt (the GP-001 arc)
metadata:
  type: project
---

**DL-020, filed 2026-08-11 from the GP-001 arc.** When a structural rule
the code must obey cannot be established from the corpus at the tier the
code needs: **refuse the case by name, say what could not be settled, and
discharge it with an independently authored fixture that can fail** —
never with a second reading of the same sentence.

**Why:** the `mAB `/`mBA ` evaluator refused `mBA ` on a curve-count
contradiction it could not reconcile with the corpus's **one blanket
sentence covering both tag types**. **An hour later the fixture corpus
found the parser bug on that exact doubt** — `decode_lut_ab` used the
`mAB ` convention for both, so every real CMYK `B2A0` refused, while
every **square** LUT hid it. A guess would have produced CMYK, and a
wrong colour looks exactly like a right one.

**How to apply:**
- **Provenance order when fixture and code disagree:** primary clause
  text → the independently authored fixture → the code. **Never edit a
  fixture to make a test pass.**
- **A blanket corpus sentence over a mirrored pair** (`mAB `/`mBA `,
  `A2Bx`/`B2Ax`, forward/inverse) **is a defect class** — it is silently
  right in the symmetric case, which is what everyone tests with. Per
  type, with per-type clause numbers, or marked UNVERIFIED.
- **A coverage claim that names the population it lacks has written the
  next fixture's specification.** The Pass 2 sweep record said in
  advance it was "light or empty on large v4 CMYK profiles with
  `mAB `/`mBA ` pipelines"; that sentence sat there for hours and acting
  on it took one file.
- **File it as ONE decision-log entry, not three.** The refusal, the
  fixture and the report-don't-repair parser are one causal chain and
  break together; three entries would all rest on the same instance.
- The generator must depend on **nothing** — a fixture written with the
  encoder the parser was written against cannot detect a shared
  misreading. That guards the code, **not** the reading: 38 files from
  one person's corpus reading share whatever it got wrong.

Related: [[iccce-pass-status]], [[iccce-verify-own-draft-too]],
[[iccce-predicted-divergence-must-be-measured]].
