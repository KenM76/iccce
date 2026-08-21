---
name: project-handoff-carries-ref-hashes-not-a-push-count
description: docs/NEXT_SESSION.md deliberately stopped carrying "N commits ahead of origin" as of 2026-08-21 — carry the two ref hashes instead; do not reintroduce the count
metadata:
  type: project
---

`docs/NEXT_SESSION.md` **no longer carries a "N commits ahead of
`origin/master`" line, and must not regain one.** Carry the two ref hashes
(`.git/refs/heads/master` and `.git/refs/remotes/origin/master`) instead.

**Why:** the count went stale **five times** in this project, and instance 5
sat in the row *directly below* instance 4's own warning about staleness
(`DL-062`, escalated to `DL-068` on 2026-08-21). `icc-librarian` recommended
retiring it and the reasoning is stronger than "it decays":

- **Nothing in any handoff ever depended on it.** Pushing needs a current
  operator go-ahead regardless of what the count says, so the line carried no
  decision and the project's single highest staleness risk.
- **`4` and `0` look identical in authority.** A wrong count reads exactly
  like a right one — this project's rule 1 (*a wrong answer looks exactly like
  a right one*) applied to its own paperwork.
- **A hash becomes *historical* when the world moves, and says so by not
  matching.** That is a self-disclosing representation; a count is not.

**How to apply:**

- When writing a handoff, print both refs. ★ Also print the caveat: **a ref
  file records what the last fetch or push OBSERVED, never the remote's
  present contents.** It supports "these matched at that moment" and supports
  no publication claim beyond it.
- ★ **Never carry a push-state claim forward from a previous handoff.**
  Re-derive it, or omit it. This is the same discipline as
  [[project-a-green-census-is-evidence-only-about-its-own-tree]] and it costs
  two seconds.
- The same reasoning generalises to any *status* in a handoff: prefer a
  representation that visibly stops matching reality over one that silently
  keeps looking authoritative.

Related: the retirement itself is recorded in `docs/ARCHITECTURE.md` as
`DL-068`, and the 2026-08-21 handoff block explains it in place for whoever
reads the document rather than this memory.
