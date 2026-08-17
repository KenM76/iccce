---
name: project-passi-published-chad-findings
description: Pass I graded iccce's chromatic adaptation against ICC's published D65-to-D50 chad — the bound the pass was COMMISSIONED with would have failed it at 7.4x because the dominant term was the white point, not the cone matrix; plus a one-sided ground-truth row proved powerless against an error moving toward the reference, and two clippy suggestions in tools/ would have injected defects.
metadata:
  type: project
---

**Built 2026-08-17 at `aece12b`.** `tools/difftest/src/passi.rs` (19 rows, all
green), instrument `src/bin/passi_probe.rs`, bounds `docs/TOLERANCES.md`
**§3.9**, operational notes `tools/difftest/README.md` **§24**, named
approximation **NA-010**. Suite `pass=274` → **`pass=293 fail=0 skip=9
error=0`**. Subject: `iccce_color::adaptation_matrix(&BRADFORD, D65, D50)` vs
ICC's `srgb.pdf` §B.2 (Jack Holm, 2015-04-27) — third `published-ground-truth`
subject, first for chromatic adaptation.

## How to apply

- **★★★ THE BOUND THE DISPATCH SUPPLIED WOULD HAVE FAILED THE PASS AT 7.4×.**
  The brief said to derive it from the cone-matrix difference (ICC.1 E.3 prints
  Bradford `0.8951`, ICC's own chad was built with `0.8950`) — exactly
  `5.661342e-6`. The **dominant** term is the *white point*: ICC's chad adapts
  the 4-dp-**rounded** `0.9505/1/1.0890` (= `chad⁻¹·D50`, exactly) while iccce
  derives D65 from BT.709-6's chromaticities, worth `4.453188e-5`, **7.9×**
  larger. They partially cancel to **`4.164937e-5` = 2.730 ULP**. *Fourth
  instance of one shape*: **when a derivation names only the components the row
  is ABOUT, the missing term is in a component it merely USES.** The
  countermeasure is not vigilance, it is doing the whole derivation in exact
  rational arithmetic **before writing the row** — which is also what let every
  bound be a prediction rather than an observation (all nine cells landed on
  their predictions to f64 noise).
- **★★★ A ONE-SIDED GROUND-TRUTH ROW IS NOT A REGRESSION GATE, and injection
  proved it.** §B grades `|iccce − published| ≤ predicted`, which is the claim
  worth publishing and has **no power against a change that moves iccce toward
  the reference**. Injecting CIE's 5-figure D65 (`0.31272/0.32903` — the
  "precision upgrade" trap) left **three §B cells PASSING because they got
  closer to ICC's numbers**; §C — the two-sided `|iccce − independent
  prediction| ≤ 1e-12` row — failed by eight orders. **Pair every
  published-ground-truth row with a two-sided derived-expectation row and say
  which is which**: the ground-truth row is the one people quote, the derived
  one is the one that holds. Same shape as `builtin.rs`'s
  `constructed_colorant_sum_is_d50` and NEXT_SESSION §5.2.
- **★★ Build the harness's prediction side out of its OWN transcription and its
  OWN arithmetic.** `passi.rs` types its own copies of the published constants
  and implements its own `inv3`/`cat`. Importing `iccce_color::BRADFORD` would
  move both sides together under a corrupted constant and every row would stay
  green. The price is a second transcription that can be mistyped — pay it with
  an **instrument row (§A) that reproduces the published matrix from the
  published inputs**, `4.44e-16` observed. That row also **re-derived
  `icc-spec-librarian`'s `0.8950` finding by a second route in a second
  language**, which is what makes it a ground-truth row and not plumbing.
- **★★ SUB-ULP DOES NOT MEAN IDENTICAL BYTES.** The corpus inferred from a
  `0.371` ULP difference that *"the written tag bytes are identical"*. Measured
  exactly: **3 of 9** cells still encode to different `s15Fixed16` words in that
  very case, **6 of 9** for iccce (max 3 LSB). Below one ULP bounds an encoding
  difference at **one LSB**; it does not zero it. Any *"sub-ULP so nothing is
  observable"* claim must be **measured through the encoder**.
- **★★ A tolerance derived from a corpus SUMMARY failed; the printed values in
  the same paragraph were right.** E3's first bound came from the corpus
  sentence *"reproduces D50 to 9.3e-9"* and failed at `7.946512e-8`. The same
  paragraph prints all three row sums — the summary quoted the **X** row as
  though it were the max and **Z** is 8.5× larger. Neither the code nor the
  published data was wrong. **Derive from the printed numbers, then check the
  sentence around them agrees**; the replacement bound is §A.7's 7-decimal print
  propagated through the published chad, closing to every digit.
- **★★ `builtin.rs` names ONE approximation for the built-in sRGB and there are
  TWO** (now NA-010). Its *"entirely accounted for by which D65 matrix each side
  starts from"* is false: exact decomposition gives a **chad** term of `2.482`
  ULP and a **primaries** term of `2.480` ULP, and on `bXYZ.Z` the celebrated
  `−0.897` ULP total is a **cancellation between `−2.482` and `+1.586`**. Also:
  that doc says the approximation is *"asserted in the tests"* — grepped, the
  published digits appear in **no** file under `crates/`. **A residual whose
  cause is stated confidently is worth decomposing; the total agreeing does not
  make the attribution right.** ★ And adopting ICC's `0.8950` would make the
  colorant row **worse** (`4.686594e-5` vs `4.607402e-5`) — *not a defect with a
  known fix.*
- **★ A pass with no oracle is possible and is the right posture for ground
  truth.** `passi::run()` takes no `Oracle`; nothing in it can SKIP, so a green
  CI line means those rows **ran** — unlike Passes G and H.
- **★★★ CLIPPY'S SUGGESTION CAN BE THE DEFECT.** Asked whether CI should lint
  `tools/` (37 findings in difftest, 12 in gen-profiles). **34 are `usize as
  f64` noise; 3 were semantic and on 2 of those clippy is WRONG**:
  `manual_clamp` proposed `d.l.clamp(0.0, 50.0)` for a line-for-line
  transcription of lcms2's `BlackPointAsDarkerColorant`, where `L > 95 → 0`, not
  50 — **a 50 L\* error injected into the code whose purpose is fidelity**; and
  `neg_cmp_op_on_partial_ord` proposed `a >= b` for `!(a < b)`, which differs on
  **NaN** (the negated form gives up; the tidy form carries NaN into a
  black-point estimate). **Never flip `-D warnings` on a new tree before
  triaging it** — a red gate creates pressure to apply the suggestion, and the
  gate becomes the mechanism that introduces the defect. All three are now
  `#[expect(…, reason)]`, which documents the fidelity. gen-profiles' 12 are the
  higher priority despite being fewer: `usize as u8` in **fixture byte
  emission**, where a silent truncation writes a wrong reference.
- **★ `tools/difftest` has never been rustfmt-clean** — 19 of 19 files differ.
  A `fmt` gate arrives as a large mechanical diff and must be **its own commit**.
- **Each tool workspace declares its own `[workspace]`, so
  `cargo clippy --workspace` from the repository root reaches NONE of them** and
  is green while 49 findings sit unseen. A CI job must `cd` into each.

Related: [[project-passh-acceptance-and-refusal]],
[[project-prove-the-arm-by-injecting-the-defect]],
[[project-candidate-separation]],
[[project-stale-claim-strings-in-emitted-records]],
[[project-parallel-agent-build-collisions]].
