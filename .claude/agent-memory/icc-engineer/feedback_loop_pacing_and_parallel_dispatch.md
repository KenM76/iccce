---
name: loop-pacing-and-parallel-dispatch
description: Ken wants fast loop ticks and maximally parallel agent dispatch; serialize only on real file-ownership conflicts
metadata:
  type: feedback
---

Run the autonomous loop with SHORT wakeup delays (~300s, not 1800s) and
dispatch agents IN PARALLEL whenever their file sets are disjoint.

**Why:** Ken's instruction 2026-08-11 during the iccce kickstart loop:
"loop faster and run agents in parallel without conflict where
possible." The engineer had been serializing librarian dispatches to
avoid edit conflicts and idling 1800s between ticks; Ken wants
throughput.

**How to apply:** The iccce agent territories are naturally disjoint —
`icc-conformance` owns `tools/difftest/` + `docs/TOLERANCES.md`;
`icc-spec-librarian` owns `D:\Dev\Rag-Specialized\ICC_Spec\` +
`docs/LEGAL.md` §2; `icc-librarian` owns the other `docs/` records
(ROADMAP, ARCHITECTURE §5, SESSION_LOG, NEXT_SESSION, NUMERIC_CLAIMS);
`icc-engineer` owns `crates/`. Dispatch any combination of them
simultaneously; serialize ONLY when two dispatches would edit the same
file (e.g. two icc-librarian filings back-to-back, or anything touching
the same LEGAL.md section). State each agent's do-not-touch list in the
dispatch prompt, as already practised. Keep ScheduleWakeup delays near
the short end unless genuinely waiting on something slow.
