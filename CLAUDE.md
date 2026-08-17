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

### 10. ★ FIRST, EVERY SESSION: check the request channel

**`D:\Dev\FeatureRequests\iccce_FeatureRequests\open\`**

List that directory at the start of every session, before anything else.
It is the communication channel between this session and the **`pdfce`**
session at `D:\Dev\pdfce\` — a PDF engine with no colour management at
all, and the consumer this project's own `README.md` names first. Created
by the operator 2026-08-17. Neither session can ask the other a question
in real time; this folder is how they do it asynchronously.

- **`open/` empty = nothing is owed.** That is the whole check, and it is
  cheap.
- **`INDEX.md` is the memory** — one row per *closed* exchange, naming
  where in a repository the durable answer lives. Grep it; do not read
  `archive/` unless a row points there.
- **`README.md` in that folder is the contract.** Read it before writing
  anything into the channel.

Four things about it that are load-bearing and will otherwise be
discovered late:

1. **Requests flow BOTH ways.** Unlike the GUI channel `pdfce` also runs,
   this one is not a one-way queue. **`iccce` may write
   `open/request_<topic>.md` asking `pdfce` things** — *"what shape does a
   PDF actually hand you an ICC profile in?"*, *"how many components does
   a real `DeviceN` carry in the wild?"*, *"is this API callable from a
   per-pixel loop?"*. A consumer's real usage is **the best available
   check on this library's API shape**, and that check only works if this
   side asks. Not asking is the failure mode, not a courtesy.
2. **`pdfce` has hard gates that constrain anything it adopts from here.**
   It must build for **`wasm32-unknown-unknown` (CI-enforced)**; it takes
   **no copyleft anywhere in a dependency tree** (it is MIT, same as this
   project); and **`pdfce-core`/`pdfce-render` must never gain a network
   client under any future decision**. iccce satisfies all three today —
   the whole `Cargo.lock` is five packages, all of them ours — but that is
   a *dated observation, not a standing guarantee*, and **iccce does not
   gate wasm32 in CI**. Adding a dependency here can silently break a
   consumer's CI gate. Classify against those three before adding
   anything, not just against rule 9.
3. **★ Overprint is `pdfce`'s problem, not this project's.** It is
   **compositing, not conversion** — it decides which colorant channels a
   paint operation writes to, in a device space, *before* any conversion
   to a display. This is the single row of the channel's boundary table
   most likely to be mis-filed, and
   `open/note_boundary_and_overprint.md` explains why. iccce supplies the
   conversion at the *end* of that pipeline and owns none of the
   compositing in front of it. The dependency runs one way: **iccce's half
   unblocks `pdfce`'s overprint work, and none of the overprint work is
   iccce's.**
4. **`pdfce`'s shipped `DeviceCMYK`→sRGB table is fitted to *pdfium*
   output** (`crates/pdfce-core/src/color/cmyk_table.rs`) — a 6×6×6×6
   lookup obtained by rendering patches and measuring them. That makes it
   a **cross-check against another implementation, not ground truth** —
   this project's own rule 3, applied to `pdfce`'s existing work. It
   matters because **replacing it with iccce is a *lateral* move in
   evidence class**, since iccce's oracle for a CMYK LUT path is lcms2,
   also a cross-check. Adopting iccce is defensible as a **conformance**
   gain (the document's declared output intent gets honoured at all) and
   **not** as an accuracy gain. Do not let it be written as the latter.

**The channel folder is in NO git repository, deliberately.** Consequence,
and it is binding: **nothing may exist only there.** A reply is a
*pointer plus an executive summary*; the durable finding lands in this
project's own `docs/` in git — a measurement in `docs/NUMERIC_CLAIMS.md`,
a tolerance in `docs/TOLERANCES.md`, a behaviour in a test, an API in a
doc comment. **One topic per file.** And the channel's own extra rule,
which exists because of rule 1 above: **a colour claim carries its
reference and its number, and names its oracle** — ground truth,
cross-check against another implementation, and self-comparison are three
different strengths of claim in that folder exactly as they are here.

## Cross-project knowledge bases

- `D:\Dev\Rag-Specialized\ICC_Spec\` — this project's standards corpus.
- `D:\Dev\Rag-Specialized\PDF_Spec\` — the PDF side. **Cross-reference
  §8.6, do not duplicate it.**
- `D:\dev\rag\rust\` — Rust/Cargo/packaging quirks that generalise.
- `C:\personal_rag\` — empirical lessons, all subjects.
