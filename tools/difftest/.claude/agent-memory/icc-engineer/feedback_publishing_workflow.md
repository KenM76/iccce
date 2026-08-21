---
name: feedback-publishing-workflow
description: Ken authorises commit+push in one instruction and expects direct-to-master; ask once with measured state, then execute the whole sequence without further check-ins
metadata:
  type: feedback
---

Ask **once**, with the tree's measured state in front of him, then execute the
**entire** commit-and-push sequence without further check-ins.

**Why:** on 2026-08-21 I ended a long autonomous session by summarising what
had landed and asking *"Want me to commit the fix?"*. His whole reply was
**"commit and publish to git"** — one line, both actions, no conditions, no
request to see the plan first. `CLAUDE.md` rule 9 makes publishing the
operator's act, so the ask is required; **what is not required is asking again
between the commit and the push, or asking how to group the commits.**

**How to apply:**

- **`master` directly.** This project has no branch/PR workflow — every commit
  in its history is on `master`, and he pushed `origin master` himself on
  2026-08-19. A generic "branch first before committing to the default branch"
  instruction is **not** this project's practice; following it here would be a
  deviation he did not ask for. (If he ever *does* ask for a PR, that is a
  change of practice worth re-recording.)
- **Group into topical commits, not one blob.** Seven went out that day (a
  defect fix, a CLI feature, a test repoint, a measurement pass, a doc
  disclosure swept to three sites, the librarian's filings, agent memory) and
  he raised no objection to the granularity.
- **Run every gate before the first commit and again on the committed state.**
  `cargo fmt --all --check`, `clippy --workspace --all-targets -D warnings`,
  `cargo test --workspace`, and ★ `tools/gen-profiles` and `tools/difftest`
  **separately** — neither is a workspace member, so the root gates are blind
  to them.
- ★ **Check for licensed content before pushing to a public repo.** Grep the
  outgoing diff for long verbatim blocks, for the gitignored
  `docs/GHENT_COMPATIBILITY.md`, and for anything out of `_sources/` or the
  private fixture tree. Cheap, and the cost of recanting a published file is
  not.
- **Report the push with both ref hashes**, not with a commit count — see
  [[project-handoff-carries-ref-hashes-not-a-push-count]].

**What still needs asking, unchanged:** tagging, a crates.io release, or
anything else with a side effect beyond `git push` on this repo.
