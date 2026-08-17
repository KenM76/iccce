---
name: stale-citation-worse-than-stale-number
description: Cite NUMERIC_CLAIMS.md by §/NC-number, never by line — 6/6 spot-checked line citations were stale, and a wrong pointer is worse than a wrong number because it authenticates the destination.
metadata:
  type: project
---

**Cite `NUMERIC_CLAIMS.md` by §-number and NC-number. Never by line
number.** Source files may be cited `path:line`, but give the **full path
from the repo root**, not a bare filename.

**Why:** DL-034 / `NEXT_SESSION.md` §5.7 covers a claim-bearing *number*
going stale. A claim-bearing *citation* going stale is a distinct and
worse failure mode: **a wrong number invites re-derivation** (the doubting
reader recomputes), whereas **a wrong pointer invites the reader to accept
whatever is at the destination** — arriving somewhere plausible reads as
confirmation, so the citation authenticates the wrong text rather than
failing.

The 2026-08-17 instance: `NEXT_SESSION.md` §3.0 cited iccce's 33-node
recommended grid as `NUMERIC_CLAIMS.md:2164, :2529`. `:2529` described
**`USWebCoatedSWOP.icc`'s own `lut8` CLUT, which also has 33 nodes — a
different 33**, a vendor file's tag rather than iccce's recommendation.
The line was copied unchecked into an outbound `pdfce` request. Real homes:
**§3.19 / NC-145** and **§3.27**; symbol
`iccce_cmm::compiled::recommended_grid_points`
(`crates/iccce-cmm/src/compiled.rs:77`, called from
`crates/iccce-cli/src/main.rs:421`). Correction filed at §3.30.7.

**It is structural, not a slip.** I read all six cited locations at tip
`e21154c`: of `:2164`, `:2529`, `:5788`, `:623`, `:976`, `:6488`, **six of
six did not carry the claim cited to them.** §3.30.7's own correction went
stale the same way within one filing. A ledger that grows by insertion
renumbers every line below the insertion, so line citations decay silently
everywhere they were copied. Code-file citations held up (4/4 exact) —
line drift is a property of the *append-heavy ledger*, not of citation.

**★ A SIXTH failure of a different kind, found 2026-08-17 at the Pass G
filing: a bare `§4.4` with NO DOCUMENT NAMED.** Nothing in `docs/` has a
§4.4 carrying the constant it was attached to; the two that exist —
`LEGAL.md` §4.4 (*"What is not claimed"*) and `GHENT_COMPATIBILITY.md`
§4.4 (*"ICC v4 evaluation on a vendor-authored profile"*) — are **both
plausible enough to be read as confirmation.** A §-pointer without its
document is the same disease without even a line number to blame.

**★★ And the decay was caught in the act.** §3.30.7 recorded `:2164` as
§3.13's Pass 6 coverage box (this librarian, before that filing's edits);
the corrected outbound census request calls the same line *"unrelated
(BPC material)"* (`icc-engineer`, later the same day). **Two readers, two
moments, one line number, two destinations, neither reading wrong.**

**STATUS as of 2026-08-17 (Pass G filing): DISCHARGED.** All six are
re-cited by §/NC in `NEXT_SESSION.md` §0 and §5.8; the outbound census
request was corrected by `icc-engineer`; and the decision-log entry is
filed as **`ARCHITECTURE.md` DL-048** (beside DL-034, as §5.8 asked).
Real homes for the record: `:5788` → the standing
`published-ground-truth` row of the **§7.x** tables (§7.11/§7.12/§7.14
name IEC 61966-2-1); `:623` → **§3.5 / NC-018**; `:976` → **§3.8.2 /
NC-036** (restated §3.8.9); `:6488` → **§3.29.6** / DL-041.

**How to apply:** when filing, rewrite any `NUMERIC_CLAIMS.md:NNNN` you
encounter into §/NC form rather than passing it along, **and give source
files their full path from the repo root** — the bare `diag.rs:83` was
right in line and content and still ambiguous, because **two distinct
`ParseError` types exist in this workspace and both implement `Error`**.
Treat a line-numbered ledger citation in any dispatch as unverified
regardless of how confident the dispatcher sounds — this is exactly the
[[verify-own-draft]] case, and the collision hazard is
[[count-needs-its-apparatus]]'s "always carry the denominator" in pointer
form.
