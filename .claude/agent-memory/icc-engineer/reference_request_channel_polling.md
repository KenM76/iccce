---
name: reference-request-channel-polling
description: Ken wants the pdfce↔iccce request channel polled every ~15 minutes during a session, not only at session start
metadata:
  type: reference
---

The cross-project request channel is
`D:\Dev\FeatureRequests\iccce_FeatureRequests\` — `open/` is the working
set, `INDEX.md` the memory, `archive/` read only when a row points there.
**The contract is `CLAUDE.md` rule 10 and the folder's own `README.md`;
do not restate it here.** What is *not* in those documents is the
cadence Ken asked for on 2026-08-17:

**Poll `open/` about every 15 minutes for the whole session**, not just
once at startup. Other agents in other projects drop requests into it
while a session is already running, and a start-of-session-only check
leaves them sitting for hours.

**How to apply:** arm a `Monitor` at the start of every session with a
persistent 900-second poll that emits one event per new-or-changed file
in `open/`, e.g. baseline `stat -c '%n %Y %s' "$DIR"/*` into `prev`,
loop `sleep 900`, `comm -13` against the fresh listing. Monitors die
with the session, so this is re-armed each time — it is not something a
previous session left running for you. Track **mtime and size**, not
just filenames: a request that gets *edited* in place is as much new work
as one that gets created.

Related: [[project-ghent-compatibility]].
