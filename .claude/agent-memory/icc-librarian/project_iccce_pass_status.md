---
name: iccce-pass-status
description: iccce Pass 0 finished 2026-08-11 (scaffold, parser, lcms2 oracle, corpus); Pass 1 colorimetry is next and NUMERIC_CLAIMS.md is deliberately not created until its first measured claim
metadata:
  type: project
---

**Pass 0 completed 2026-08-11** — the project's first working session.
Four crates, a header/tag-table parser that reports and does not repair,
`iccce-cli inspect` demonstrated on a real system profile, lcms2 pinned
out-of-tree by commit hash and smoke-tested, a 21-file standards corpus,
and `docs/TOLERANCES.md` with one provisional anchor. **Pass 1
(colorimetry, `iccce-color`) is next.**

**Why this matters to a librarian specifically:** `docs/NUMERIC_CLAIMS.md`
— the ledger this agent exists to keep — **does not exist yet, and that
is deliberate.** Pass 0 produced no measured colour claim: `iccce-color`
and `iccce-cmm` are stubs, nothing in iccce has been compared to
anything, and the only numbers on record are lcms2's own smoke-test
outputs (cross-check values from an implementation, explicitly not
transplantable into a unit test). An empty ledger invites a first row
that is not a measurement, and makes "nothing has been measured" look
like "nothing has been filed."

**How to apply:**
- **Create `docs/NUMERIC_CLAIMS.md` with the first genuinely measured
  claim**, expected in Pass 1: the ΔE2000 arithmetic-agreement result
  against the 34 Sharma et al. (2005) pairs already transcribed in the
  corpus. Do not create it earlier to "have it ready."
- Every row carries what was compared, tolerance, measured value,
  **corpus and coverage**, commit, and date. Coverage is part of the
  claim — "verified on the 34 Sharma pairs" never becomes "verified."
- **Verify before relying on this:** read `docs/ROADMAP.md` for the
  current Pass status and check whether `docs/NUMERIC_CLAIMS.md` now
  exists. This memory is a snapshot of 2026-08-11.
- Pass 0's completion record in `ROADMAP.md` was filed **without a commit
  hash** (the work was uncommitted; the commit is the engineer's act).
  If that line still says so, it is worth filling in.

Related: [[icc1-pdf-operator-blocker]].
