---
name: project-doc-editing-conventions
description: iccce project agents edit shared docs concurrently and are scoped to named sections; verification records are dated, append-only, and never edited in place.
metadata:
  type: project
---

Two conventions observed on this project, both learned the hard way
elsewhere:

**1. Section-scoped edits.** Multiple project agents work the same shared
documents at the same time. On 2026-08-11 `icc-conformance` was
instructed to edit `docs/LEGAL.md` **§4 only** while `icc-spec-librarian`
was concurrently writing §2. Both edits landed cleanly.

*Why:* `docs/LEGAL.md`, `docs/ROADMAP.md` and `docs/SESSION_LOG.md` have
several agent owners. A whole-file rewrite silently discards a
concurrent agent's work, and neither agent finds out.

*How to apply:* re-**Read** the file immediately before editing (it may
have changed since session start), then use a targeted `Edit` scoped to
the assigned section. Never `Write` a shared doc wholesale. If a task
requires touching a section you were not assigned, say so rather than
doing it.

**2. Verification records are dated and append-only.** `LEGAL.md` §2 and
§4 are structured as dated observations — "Terms as actually checked —
2026-08-11", "Verification as actually performed — 2026-08-11". A
re-verification adds a new dated subsection; it does not overwrite the
old one. Same rule for tolerance changes: `docs/TOLERANCES.md` §4 is an
append-only change log, because the history of a tolerance is the only
defence against it drifting one justification at a time.

*How to apply:* when re-checking anything previously recorded, append.
The old entry stays as a record of what was believed and when.

Related: [[project-lcms2-licence-is-not-uniform]],
[[project-oracle-and-tolerance-state]].
