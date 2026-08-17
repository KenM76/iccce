---
name: iccce-count-from-a-sample-is-not-the-population
description: DL-053 — "two iccMAX" was really TEN; the claim carried [VERIFIED] and every word was true of what was RUN. A [VERIFIED] tag certifies the measurement happened and certifies nothing about what it ranged over. Filed 2026-08-17.
metadata:
  type: project
---

**A count states its DENOMINATOR, or it is not a count.** *"Two failed"*
is a claim about a **sample**; *"two of fifty failed"* is a claim about a
**population**. The first may never be written where the second will be
read — **and a handoff document is always the second.**

**Why:** on 2026-08-17 `docs/NEXT_SESSION.md`'s resume-from-cold handoff
said *"★ **Two of them correctly FAILED** [VERIFIED by me]:
`sRGB_ISO22028.icc` and `sRGB_D65_colorimetric.icc` are **iccMAX**…"*.
**Two were TESTED; TEN of the 50 are iccMAX.** Every word was true of
what was run. Filed as `ARCHITECTURE.md` **DL-053**; the measured
population is `NUMERIC_CLAIMS.md` §3.32.10a / **NC-219** (40 parse with
`malformations: 0`, 10 refused by name).

### Why this is NOT [[iccce-count-needs-its-apparatus]] (DL-031)

DL-031 says an **unlabelled** count is not a claim, because the
**apparatus** is half the number. **Here the apparatus WAS labelled** and
the verification was genuine.

★★★ **What was missing is the phrase *"…of the two I tested."***
**A `[VERIFIED]` tag certifies that a measurement HAPPENED. It certifies
nothing about what the measurement RANGED OVER.** The reader supplies the
natural denominator, which is the corpus.

★★ **It landed in the worst possible document.** `NEXT_SESSION.md` is
what every session is instructed to read first, so the figure was
positioned to be **re-quoted rather than re-derived** — DL-048's carrier
mechanism with a count instead of a citation.

### How to apply

- When a dispatch gives you a count, ask **"of what?"** before filing it.
  If the dispatch does not say, **the row states the sample, not the
  population** — or the row does not go in.
- ★ **Re-deriving a denominator often UPGRADES the claim rather than
  merely correcting it.** Here the full sweep cost one loop and turned
  *"rule 6 demonstrated on two files"* into *"rule 6 demonstrated at
  population scale on real ICC-published files"*.
- The mirror of *"a count is not an inventory"*: this is a count that
  **is** an inventory — of a **smaller set than the reader will assume**.
- Applies to profiles, files, tests, findings, rows, corpora. Anywhere a
  sweep was cheaper than the sample that got run.

Related: [[iccce-count-needs-its-apparatus]],
[[iccce-stale-citation-worse-than-stale-number]],
[[iccce-documented-is-not-tested]], [[iccce-pass-status]].
