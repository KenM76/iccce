---
name: iccce-pass-status
description: iccce status snapshot 2026-08-11 (Pass 4 IN PROGRESS) — first CLUT differential, NA-006 priced at 1.5741 dE00, an 11.217 dE00 absolute-intent divergence pending A4b, DL-019 filed, B2A has zero measurements
metadata:
  type: project
---

**Snapshot of 2026-08-11 (the Pass 4 *progress* filing — the eighth of
one calendar day). Verify before relying on any of it** — read
`docs/ROADMAP.md` (Pass 4 progress block), `docs/NUMERIC_CLAIMS.md`
§3.9, `docs/NEXT_SESSION.md` and the newest `SESSION_LOG.md` entry.

**Pass 0 done · Pass 1 core · Pass 2 one scope decision from done ·
Pass 3 DONE · ★ Pass 4 IN PROGRESS, done-when NOT met.**

**The Pass 4 differential** (SWOP → system sRGB, 341 CMYK points, all
four A2B intents, both files v2.1.0, `pass=36 fail=0 skip=3`) produced
**NC-044 … NC-056**; next free number **NC-057**. **Quote a Pass 4
number only with WHICH GATE it came from** — they differ by four orders
of magnitude:
- **corners (exact CLUT nodes, no interpolation): 5.9131e-5 / 6.6558e-5
  dE00** vs 1e-3 — the strongest cross-check evidence the project has;
- **lcms2's geometry emulated: 4.5931e-3 / 4.8154e-3** vs 2e-2 — *this*
  is the row that claims agreement;
- **raw: 0.25294 (m-rel) / 1.6590 (perceptual)** vs 2.0 — **cannot claim
  agreement**; its value IS the method envelope.

**★ NA-006 is MEASURED at last: 1.5741 dE00** (perceptual table) /
0.25423 (colorimetric) — the corpus's "~1 dE" was an **underestimate**
on one table. Priced by an apparatus **in the harness**, not by shipping
a second interpolator, and that apparatus was graded against
`Lut16Model` at 1e-9 (**0.0 exactly**) before anything was concluded.

**★ 11.217 dE00 at the ICC-absolute intent — REPORTED, NOT GRADED.**
lcms2's `cmsio1.c` substitutes **D50** for the `wtpt` of a **v2
display-class** profile; iccce uses `wtpt` as stored (NA-007). The sRGB
profile's `wtpt` is **D65** → 32 % error in Z. Modelling the one
substitution collapses it **517×** (2.1677e-2, gated at 5e-2).
**Corpus A4b decides who is wrong and is UNVERIFIED** (verified in the
corpus at this filing; M4/M5 rows absent — only M1–M3 exist).

**New: `ARCHITECTURE.md` DL-019** — mechanism identified + authority
absent ⇒ report-not-grade the raw row, gate the **modelled** quantity,
**write both rejected alternatives down**, state the blocking question
to a named owner. Five conjunctive steps; an *unmodelled* disagreement
still gets a failing gate. Also recorded there: the per-depth `PcsCodec`
was **considered and deliberately NOT given an entry** (its rule is
DL-011's, its mechanism is self-documenting in a closed enum).

**Owed, and load-bearing:** **B2A has ZERO measurements** (code in
`b3f4388`; sRGB has no `B2A*` tag, so this run's destination was
matrix/TRC) — that also means **`lut8Type` evaluation and the `Lab8`
codec are untested against anything**. **`mAB `/`mBA ` are DECODED
(Pass 2 batch 2) and NOT EVALUATED** = stage 4. **Pass 4 has no
ground-truth row at all.** NA-002's Bradford cost **still not due**
(checked against code at three consecutive filings — `iccce-cmm` has no
reference to `adapt`).

**Things that changed under a carried claim:** `tools/gen-profiles`
**now exists** (28 tests) and `fixtures/synthetic/` holds **39 .icc
files** including `v4-cmyk-mab-lab.icc` — four filings said neither
existed. Nothing reads them yet. And **NA-003's clause citation was
WRONG** (6.4 governs the PCS, not device values) — corrected append-style
in `TOLERANCES.md` §5.2, which **inverts** the NC-043 finding: a
conforming F.8–F.16 evaluation **cannot** exceed 1.0.

**Commits, ALL reported — no agent in this project has ever run git:**
`19a3b17`, `9aa1bca` (stage 1), `63874f9` (stage 2), `490191b` (CLI:
N-channel + four intents), `b3f4388` (stage 3), `db60e92`, `d9e0b82`
(the differential), `edcb60e` (untracked gen-profiles swept in by a
cwd-relative pathspec — a process slip).

Related: [[iccce-verify-own-draft-too]],
[[iccce-tolerance-cannot-swallow-and-claim]],
[[iccce-predicted-divergence-must-be-measured]],
[[iccce-gate-must-not-reward-deletion]],
[[iccce-bound-cannot-catch-its-own-magnitude]],
[[iccce-verification-loop-runs-both-ways]], [[ken-terse-scope-decisions]].
