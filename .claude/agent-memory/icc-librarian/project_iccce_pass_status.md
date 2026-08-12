---
name: iccce-pass-status
description: iccce status snapshot 2026-08-12 (PUBLIC on GitHub; Passes 6+7 DONE, original scope essentially complete; Pass 4 open on 2 items but A4b RESOLVED) — next free NC-113, DL-024/DL-025 filed, next real step is the pdfce bridge in another repo
metadata:
  type: project
---

**Snapshot of 2026-08-12 (the Pass 6 + Pass 7 filing — the twelfth
overall, and the FIRST of a second calendar day). Verify before relying
on any of it** — read `docs/ROADMAP.md` (the **"what remains"** block
under Pass 8 first), `docs/NUMERIC_CLAIMS.md` §2.9 + §3.13 + §7.9,
`docs/NEXT_SESSION.md`, newest `SESSION_LOG.md`.

**★★ THE PROJECT IS PUBLIC** — `master` pushed to
`github.com/KenM76/iccce` on the operator's explicit go-ahead ⇒
**DL-024**. **A push is evidence of a push; PUBLIC VISIBILITY IS THE
OPERATOR'S REPORT** (a private repo produces an identical reflog).
Publication authorises **nothing else**: no crates.io publish, no tag,
no release; crate-name availability still unchecked by anyone.

**Passes 0,1,2,3,5,6,7 DONE · Pass 4 is the ONLY original Pass still
open**, at (a) saturation in B2A and (b) ICC-absolute through a LUT
destination — **★ (b) is NO LONGER operator-blocked: A4b is RESOLVED**
(`ICC.1:2001-04` A.3.1.1 addresses the profile's AUTHOR and is **silent
on readers**), so whether NC-053/NC-054 stay REPORTED-NOT-GRADED is now
a judgement `icc-conformance` must make. **A4c (SILENT) does not clear
when A4b clears.** **Pass 8 = the pdfce bridge, built in pdfce, not
here.**

**Pass 6 numbers (one machine, one run, release):** 8 700 867 px in
7.23 s = **1.20 Mpix/s**; build 1.04 s (83 521 evals); reference 0.084
Mpix/s; **speedup 14.4×** — **lcms2 was NEVER timed, the ratio is iccce
vs iccce**. Off-node compiled error **0.003589 device**,
`self-consistency`, on SWOP `A2B1` `mft2` 4-D → sRGB matrix/TRC,
media-relative, 17-pt grid. **No ΔE translation exists.**

**★ Pass 6's real story: the sensitivity control caught its own
instrument TWICE** ⇒ **DL-025**. (1) fixture was sRGB→sRGB and a grid
reproduces an IDENTITY chain **exactly everywhere** ⇒ 1.1×10⁻¹⁵, ratio
0.94, no h² — **that would have been reported as the cost**. (2)
probing across sRGB's TRC breakpoint gives **h¹ not h², ratio 1.44** —
code and fixture both fine, the *expectation* was wrong. **DL-023 had
predicted this exact trap by name at the previous filing and it was
walked into anyway.** Third instrument in two days to catch a shippable
error, after DL-016 (off-by-one sample) and DL-020 (GP-001).

**Pass 7:** `resolve_to_device` → `Chain::convert_pcs_to_device` →
**`pcs_to_destination`, the SAME method `Chain::convert` uses** (arm
de-duplicated) ⇒ a spot cannot take a private path. Unknown name →
**`None`** (the `/Alternate` signal). Media-relative by construction
(Table 66). The "reachable from nothing" finding **filed twice is
closed**. **BUT NC-111 asserts a RANGE, not a colour** — no spot's
resolved value has ever been compared to anything, and the cheap
cross-check (resolve into the spot's OWN profile, compare stored
`nDeviceCoords`) was **available and not taken**.

**★ My judgements/findings this filing.** (1) **The date** — dispatch
said 2026-08-11; reflog + env + corpus register all say **2026-08-12**;
corrected, and it matters because eleven filings assert "the same
calendar day". (2) **A wrong hash**: `edcb60e` (in ROADMAP ×2,
SESSION_LOG ×3, NEXT_SESSION) is really **`edce48b`**;
`NUMERIC_CLAIMS.md` §2.6 had it right by a different route. (3) **New
evidence class `machine-timing`** — a timing is *not* correctness
evidence at all, and needed a class so it can never be quoted beside a
ΔE row. (4) **14.4× does not reproduce** from the quoted figures
(1.2034/0.084 = 14.3); not an error (unrounded division) but **the raw
12-line `iccce bench` output was never filed**. (5) **Commit count:
dispatch 49, reflog 45** — left open, nobody has run `git log`.

**Ledger: NC-105 … NC-112 filed; next free NC-113.** Neither Pass ran
`tools/difftest` at all, so §3.13 has **no runner outcome of any kind**.

**Counts, verified 2026-08-12:** **116** `#[test]` across **19** files
under `crates/` (was 103/18) — *a count of declarations, not coverage,
not a pass result*. **38 `.icc`** in `fixtures/synthetic/`;
`fixtures/reference/PROVENANCE.md` reads **"(none yet)"**.

**Two holes that outlasted every Pass:** **zero
`published-ground-truth` rows for ANY transform** (IEC 61966-2-1 is the
cheap route, nobody has dispatched for it, 7 filings running), and
**no `cargo test --workspace` outcome ever reported** (7 filings) —
while two commits in this history shipped red under green messages.

**Commits (hashes now CORROBORATED by `.git/logs/HEAD`, contents still
unverified):** `bb5d6b8` (A4c), `0378f76` (ISO/CD 18619 estimation,
A42 upgraded), `3502cb7` (Pass 6), `f6203b8` (Pass 7 = current tip of
master and origin/master).

Related: [[iccce-git-files-readable-without-shell]],
[[iccce-control-only-as-good-as-its-fixture]],
[[iccce-free-to-disagree]], [[iccce-direction-scoped-behaviour]],
[[iccce-refusal-discharged-by-fixture]], [[iccce-verify-own-draft-too]],
[[iccce-tolerance-cannot-swallow-and-claim]],
[[iccce-predicted-divergence-must-be-measured]],
[[iccce-gate-must-not-reward-deletion]],
[[iccce-bound-cannot-catch-its-own-magnitude]],
[[iccce-verification-loop-runs-both-ways]], [[ken-terse-scope-decisions]].
