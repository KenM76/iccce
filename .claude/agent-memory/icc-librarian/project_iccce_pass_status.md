---
name: iccce-pass-status
description: iccce status snapshot 2026-08-11 — Pass 0/1 done, Pass 2 BOTH batches built + 40-profile sweep, one scope decision from done; DL-014 filed (ICC.1 citation terms); Pass 3 next
metadata:
  type: project
---

**Snapshot of 2026-08-11 (end of Pass 2 batch 2). Verify before relying
on any of it** — read `docs/ROADMAP.md`, `docs/NUMERIC_CLAIMS.md`,
`docs/NEXT_SESSION.md` and the newest `docs/SESSION_LOG.md` entry.

**Commits, ALL reported by dispatches — no agent in this project has run
git:** Pass 0 `f976a0e`, Pass 1 `7313c5b`, Pass 2 batch 1 `b35a12e`,
difftest harness `bfd6b1e`, **Pass 2 batch 2 `d40d601`**.

**Built:** `iccce-color` (Pass 1); `iccce-profile` header + tag table +
**eight non-LUT tag types** + **the four LUT types** (`mft1`/`mft2`/
`mAB `/`mBA ` in `lut.rs`); iccMAX refused by name since Pass 0;
`tools/difftest` + `legacy_lab_probe`. **`iccce-cmm` is still a stub —
`iccce` has never been compared to anything, and the ledger has zero
`implementation-cross-check` rows.** 54 `#[test]` declarations; exactly
ONE is a correctness claim against published values (NC-001).

**Pass 2 done-when:** clause 1 **MET on this machine** — sweep of
`C:\Windows\System32\spool\drivers\color\`: 40 profiles, 40 OK, 0
refused, 0 crashes, 0 table-level malformations; 4 EIZO v2 profiles each
report *"desc: Macintosh ScriptCode block short or missing"*, the exact
structure the corpus predicted. **No per-tag-type breakdown was taken**,
so it does NOT establish the LUT decoders met real input. Clause 2
**PARTIAL**: every tag type has hand-authored byte fixtures *inside the
unit tests*, but `tools/gen-profiles/` does not exist and
`fixtures/synthetic/` holds only a README saying so. **Whether in-test
synthetics satisfy the clause is an open scope decision — filed as a
question, deliberately not decided by the librarian.**

**DL-014 filed** — DL-002's successor, owed across three filings.
ICC.1:2022 clause numbers may be cited; **the citation must name the
corpus file**; **the tier is per-fact, not per-file** (15 of 20 corpus
files are `primary_spec`, but 11 only *partly*, with split `evidence:`
lines). Prohibition unchanged for unread documents (ICC.1:2010,
ICC.1:2001-04, ISO 13655, CIE/IEC, "Adobe's document"). **Automated
retrieval from color.org is still prohibited.** DL-014 does **not**
retroactively bless existing citations and no audit has been done.

**Closed by CHECKING, not assuming (both had been carried as owed):**
the `legacy_lab_probe.rs` P3/P4 doc-comment arithmetic is fixed; the
corpus retraction of the lcms2 version-keying claim **landed** (C3 in
`icc__ref__v2_v4_divergence.md` + `index.md`, new file
`icc__ref__lcms2_measured_behaviour.md` M1/M2).

**Next: the clause-2 decision, then Pass 3 (matrix/TRC).** Pass 3 is
where the ledger gains its first `implementation-cross-check` row, where
**NA-002's Bradford cost comes due** (sRGB→AdobeRGB adapts; both
alternative CATs are unsourceable), and where the corpus's **sRGB/D65
single-source gap** (lcms2 only; BT.709 free but not fetched, and DL-007
requires reading ITU's terms first) means agreement with lcms2 may just
be shared provenance.

**Still open every session:** `TOLERANCES.md` §3.2 all `—` and §6 still
reads "2–8 not started"; `ncl2`/B2A behavioural tests; the forced-BPC
copy decision; a ground-truth row for chromatic adaptation; **nothing
has ever run on Linux and no CI run has ever been observed.**

Related: [[iccce-predicted-divergence-must-be-measured]],
[[iccce-verification-loop-runs-both-ways]], [[icc1-pdf-operator-blocker]],
[[ken-terse-scope-decisions]], [[iccce-verify-own-draft-too]].
