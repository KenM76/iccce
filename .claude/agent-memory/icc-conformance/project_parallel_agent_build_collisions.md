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

3. **A second instance of THIS agent runs concurrently and writes the same
   memory directory.** On 2026-08-12 `project_pass4c_absolute_intent_findings.md`
   and an edited `MEMORY.md` appeared mid-session from another
   `icc-conformance`. **Re-read `MEMORY.md` immediately before editing it**, and
   never rewrite it wholesale — insert a line, do not replace the file.
4. **The collision now also happens inside `tools/difftest` itself**, not just
   in `crates/`. The other instance added `src/pass4c.rs`, `pub mod pass4c;` in
   `lib.rs` and a block in `main.rs`; their in-flight file called a `Profile`
   method that commit `95c04c1` had just sealed, and the whole crate stopped
   compiling for ~20 minutes. In a worktree, strip their module from the
   **worktree copy** of `lib.rs`/`main.rs` only — never from the main tree,
   which is theirs to finish. A conservative line-based filter is safer than a
   regex: a `.*p4c.*` sweep silently ate an unrelated line and produced a
   parse error 200 lines away.

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
