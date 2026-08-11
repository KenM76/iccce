# iccce — project instructions

A from-scratch, MIT-licensed **ICC colour management module** in Rust.
Read `README.md` for scope, `docs/ARCHITECTURE.md` for shape,
`docs/ROADMAP.md` for the plan, and `docs/NEXT_SESSION.md` before doing
anything.

The global rules in `C:\Users\Ken\.claude\CLAUDE.md` apply — especially
documentation-first, dispatch-agents-freely, and the personal_rag
lesson-writing discipline. This file adds what is specific to colour.

## Project agents

| Agent | Role |
|---|---|
| `icc-engineer` | Lead engineer. **Be this agent** if you are orchestrating; read its file at session start. |
| `icc-spec-librarian` | Builds `D:\Dev\Rag-Specialized\ICC_Spec\`. Dispatch for EVERY sourcing question. |
| `icc-conformance` | The oracle, the fixtures, the tolerance budget. Owns "how do we know?" |
| `icc-librarian` | ROADMAP / SESSION_LOG / decision log / the numeric-claims ledger. No shell — dispatches must carry evidence. |

## The rules that are specific to this project

### 1. A wrong colour looks exactly like a right one

This is the whole reason the discipline below is heavier than it would be
for a parser. Nothing about a 3 ΔE error announces itself. Assume every
plausible-looking result is unverified until measured.

### 2. Never write colour maths from memory

Adaptation matrices, transfer-function breakpoints, Lab encodings — all
things one half-remembers correctly. Dispatch `icc-spec-librarian` and
cite the standard and clause in the doc comment.

### 3. Expected values come from the literature

A test whose expectation came from the code under test detects change,
not error. Where no published value exists, use lcms2 and label the
expectation a **cross-check against another implementation**, which is a
weaker claim than ground truth and must not be written as though it were
the same.

### 4. Every approximation is named and measured

A CMM is a stack of interpolations. State what each departure from exact
colorimetry is and what it costs in ΔE. An unstated approximation is
indistinguishable from a bug.

### 5. Tolerances are justified, not tuned

"Within 1 ΔE2000 because that is the accepted perceptibility threshold"
is a tolerance. "Within 0.5 because it passed" is a number someone moved
until the suite went green. When a test fails, the first question is
whether the code is wrong.

### 6. The parser reports; it does not repair

A silently corrected tag hides the malformation from the only layer that
could disclose it.

### 7. Disagreement with lcms2 is a finding, not a failure

lcms2 is an implementation, not the standard. When the two differ, settle
it from the specification text and record the outcome — a case where
iccce is deliberately right and lcms2 is wrong is worth writing down.

### 8. Optimise only after correct

A fast wrong answer is harder to fix than a slow one, because the speed
becomes load-bearing before anyone checks the arithmetic.

### 9. MIT, and publishing is the operator's act

Classify every dependency before adding it. Never push, tag or release
without an explicit current go-ahead.

## Cross-project knowledge bases

- `D:\Dev\Rag-Specialized\ICC_Spec\` — this project's standards corpus.
- `D:\Dev\Rag-Specialized\PDF_Spec\` — the PDF side. **Cross-reference
  §8.6, do not duplicate it.**
- `D:\dev\rag\rust\` — Rust/Cargo/packaging quirks that generalise.
- `C:\personal_rag\` — empirical lessons, all subjects.
