---
name: iccce-ground-truth-cannot-exist
description: DL-041/DL-042 — published ground truth for the LUT path CANNOT EXIST (structural, not availability); ICC.1 Annex D.6.3 is available AND a test now asserts its twelve integers (0bd76ad) — but NC-001 is STILL the only published-ground-truth LEDGER ROW. A test is not a row.
metadata:
  type: project
---

**The eleven-filing "no `published-ground-truth` row for any transform"
item was SPLIT on 2026-08-12, and both halves are easy to state wrongly.**

**Why:** `icc-engineer` dispatched `icc-spec-librarian` to *close the
gap or classify it as structural*. Verdict: **PARTIAL CLOSE + STRUCTURAL
LIMIT, and the halves must not be merged** (`ARCHITECTURE.md` **DL-041**;
corpus file `D:\Dev\Rag-Specialized\ICC_Spec\icc\icc__ref__ground_truth_availability.md`).

### ★★★ The half that must never be rounded up

**ICC.1:2022 Annex D.6.3** publishes an input, every intermediate and
**twelve exact integer PCS encodings — all twelve reproduce**;
**Table 16** is **normative** and its five value↔encoding pairs
reproduce too (its `0808h` independently confirms v4's `65 535` `L*`
scaling — v2 legacy would give `0800h`).

> **They were reproduced by the CORPUS's arithmetic, not by iccce's
> code. NOTHING IN THIS PROJECT HAS BEEN COMPARED TO EITHER.**
> The ground truth is **available**; the row is **buildable**; **the row
> does not exist. `NC-001` (ΔE2000 vs Sharma/Wu/Dalal) remains this
> project's only `published-ground-truth` row.**

> ★★★ **CORRECTED 2026-08-17 — the first sentence of the box above is
> STALE; the last one is STILL TRUE, and the gap between them is the
> point.** Commit **`0bd76ad`** landed
> `crates/iccce-cmm/tests/annex_d_ground_truth.rs` — *"the project's
> first ground-truth test for a transform path"*, asserting **twelve
> exact published integers** against iccce's own code *(verified — file
> read at the tip 2026-08-17)*. **So iccce HAS now been compared to
> Annex D.6.3.**
>
> ★★★ **But `NUMERIC_CLAIMS.md` §7.17 still records NC-001 as the only
> `published-ground-truth` ROW, and §7.15 newly-owed 1 is still owed.**
> **A TEST IS NOT A LEDGER ROW.** The test exists in `crates/`; the row —
> with its tolerance, coverage, corpus and evidence class — has never
> been filed, and the blocking operator decision (may published numbers
> live in an MIT repo as fixtures) is unchanged. ★ **Say "the test
> exists, the row does not", never "still nothing has been compared" and
> never "we now have a second ground-truth row."** Both are wrong in
> opposite directions.

Three qualifications travel with every citation: **Annex D is
INFORMATIVE** (ground truth *epistemically*, not normatively — a
disagreeing CMM is not thereby non-conforming); the exact part is the
**twelve integers**, since displayed precision is 4 dp; and **Table D.2's
black `X = 0,009 7` is defective — start the fixture at Table D.3**
(corpus register `A47`).

### ★★★ The half that is STRUCTURAL, not a gap to chase

**ICC.1 mandates no interpolation method**, so two conforming CMMs may
legitimately return different numbers for the same profile and the same
input ⟹ **no single value could be published as the expected result even
in principle.** Also: a CLUT is one vendor's fit (no "correct" CLUT);
perceptual is vendor-defined by design; out-of-gamut handling is
delegated; and ICC's conformance clause binds **reading**, not computing
(DL-026).

**Corroborated from the strongest direction: iccDEV — ICC's OWN
reference implementation — ships ZERO expected colour values.**
`RunTests.sh` compares nothing; `ApplyDataFiles/` holds inputs only;
both expectation manifests are structural (parse/validation), not
colorimetric. ⟹ **agreement cannot be promoted to ground truth by
finding a better implementation**; a differential against iccDEV would
be a *second `implementation-cross-check`*. The remedy for a large
divergence is **a second implementation lineage** (iccDEV, BSD-3;
**Argyll is BARRED, AGPL-3.0**).

### ★★ Keep the three blocker kinds apart

| Kind | Example | Remedy |
|---|---|---|
| **EXISTENCE** | LUT results, iccDEV's suite, an ECI/Fogra residual, sRGB worked triples | none — **not withheld, not produced** |
| **AVAILABILITY** | IEC 61966-2-1, ISO 15076-1, ISO 12647-x, CGATS TR001 | money / operator time |
| **ACCESS TERMS** | `color.org` (ToS names AI), `fogra.org` (`robots.txt`), `itu.int` (WAF) | **a human clicks** — three different mechanisms |

Conflating them is the error the table exists to prevent: "we do not
have it" has three completely different fixes. Same family as the ICC.1
PDF blocker — see [[iccce-icc1-pdf-operator-blocker]].

**How to apply.** When any filing touches the ground-truth item: say
**available**, never **measured**, until an iccce row exists. Building
the Annex D fixture is blocked on **one operator decision** — may
published numbers live in an MIT repo as fixtures (ICC's Annex D
values + CIE's CC BY-SA tables + ECI's self-contradicting `cprt` are
**one question**, not three). Relabel the LUT remainder **STRUCTURAL,
not OPEN**.

Related: [[iccce-negative-finding-removes-its-auditor]] (why this sat
owed for eleven filings), [[iccce-pass-status]],
[[iccce-agreement-can-be-the-symptom]], [[iccce-free-to-disagree]].
