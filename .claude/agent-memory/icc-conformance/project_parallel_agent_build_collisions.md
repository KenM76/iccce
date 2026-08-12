---
name: project-parallel-agent-build-collisions
description: On iccce, other agents' in-progress edits to crates/ routinely break the build mid-session, and the engineer sweeps this agent's uncommitted harness files into their own commits — build in a detached worktree and re-verify against the new HEAD before reporting.
metadata:
  type: project
---

Observed **2026-08-12** during the completion sweep, and it cost real time.

**Two things happen when several agents work this repo at once.**

1. **A parallel agent's half-finished edit to `crates/` breaks the whole
   workspace**, including `tools/difftest`, which path-depends on all three
   crates. On 2026-08-12 `crates/iccce-cmm/src/named_color.rs` called a
   `Chain::convert_pcs_to_device` that did not exist yet (Pass 7 in progress),
   and nothing in the tree compiled for most of the session.
2. **The engineer commits the whole working tree.** Files this agent created but
   had not finished (`tools/difftest/src/pass5b.rs`, `pass6.rs`, and edits to
   `pass4b.rs`, `lib.rs`, `main.rs`) were swept into commits `f6203b8` /
   `5867f1a` in the middle of the run.

**Why this matters:** neither is a problem to fix — the engineer owns commits and
the other agent owns `crates/` — but both invalidate a naive workflow.

**How to apply.**

- **Do not `git stash` or edit another agent's broken file.** Instead:
  `git worktree add --detach <scratchpad>/wt HEAD`, copy your own changed files
  in, and build/run there with
  `ICCCE_TRANSICC=D:/Dev/iccce/tools/difftest/vendor/build-msvc/transicc.exe`
  (the oracle lives in the main tree and `Oracle::locate` resolves relative to
  `CARGO_MANIFEST_DIR`, so a worktree finds nothing without the env var).
  `git worktree remove --force <path>` when done.
- **Before reporting numbers, `git log --oneline -5` again and re-run the suite
  in the MAIN tree.** HEAD may have moved several commits since session start,
  and a measurement taken at the old HEAD is a measurement of a build nobody has.
- **Do not assume your uncommitted files are still uncommitted.**
  `git status --short` late in the session is the only reliable statement of what
  is yours to hand over.

Related: [[project-doc-editing-conventions]],
[[project-oracle-and-tolerance-state]].
