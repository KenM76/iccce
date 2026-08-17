---
name: reference-memory-corpus-lives-at-repo-root
description: The icc-conformance memory corpus for iccce lives at the REPOSITORY ROOT (D:\Dev\iccce\.claude\agent-memory\icc-conformance\), not under tools\difftest — 25+ files with a maintained MEMORY.md index; read that one first whatever path this session was handed.
metadata:
  type: reference
---

**Two paths exist and only one has content.**

| path | state |
|---|---|
| `D:\Dev\iccce\.claude\agent-memory\icc-conformance\` | ★ **THE CORPUS.** 25+ memories, `MEMORY.md` index actively maintained, in git, shared with the team |
| `D:\Dev\iccce\tools\difftest\.claude\agent-memory\icc-conformance\` | created empty by a session whose cwd was `tools\difftest`; holds this pointer and nothing else |

**How to apply.** At session start, regardless of which path the harness names
as "your memory", **read
`D:\Dev\iccce\.claude\agent-memory\icc-conformance\MEMORY.md` first** and write
new memories alongside it. Splitting the corpus across two directories is the
one outcome to avoid: the value of these files is that they are read together,
and an index that lists half of them is worse than no index.

Everything Pass 1–H learned about the oracle, the tolerance budget, the
fixture corpora and lcms2's behaviour is in that root directory. Highlights an
incoming session should not miss: `project_oracle_and_tolerance_state.md`,
`project_stale_claim_strings_in_emitted_records.md`,
`project_prove_the_arm_by_injecting_the_defect.md`, and
`project_a_fixed_defect_can_blind_its_own_row.md`.
