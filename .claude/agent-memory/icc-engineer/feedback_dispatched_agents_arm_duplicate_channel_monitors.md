---
name: dispatched-agents-arm-duplicate-channel-monitors
description: Tell dispatched iccce subagents NOT to arm their own request-channel Monitor — each one duplicates the lead's poll and multiplies notifications
metadata:
  type: feedback
---

When dispatching `icc-conformance`, `icc-librarian` or `icc-spec-librarian`,
**state in the dispatch prompt that the lead already polls
`D:\Dev\FeatureRequests\iccce_FeatureRequests\open\` and the agent must not
arm its own `Monitor` on it.**

**Why:** the project `CLAUDE.md` rule 10 tells *every* session to check the
request channel first, and the agent definitions inherit it. Each dispatched
agent reads that as an instruction to arm a persistent poll of its own. On
2026-08-19 a single session ended up with **three** monitors on the same
folder (`bcex6a036`, `bsxt2pp57`, `b8bcc9id7`) after two agent dispatches —
so one outbound note produced three identical notifications. With a real
inbound request the multiplier is the same, and the operator had explicitly
asked to conserve tokens that session.

★ **They cannot be cleaned up mid-session.** `TaskStop` was not in the
session's toolset and `ToolSearch` found no deferred match, so once armed the
duplicates persist until the session ends. **Prevention is the only control**
— which is why this belongs in the dispatch prompt, not in a later fix.

**How to apply:** add one line to every agent dispatch — *"Do not arm a
Monitor on the request channel; the lead session already polls it."*
Distinguish this from the legitimate case: the lead session **should** re-arm
its own poll every session, because monitors die with the session (see
[[reference_request_channel_polling]]).

Related: the channel echoes the lead's **own writes** back, since filenames
carry no direction — always read the `**from:**` header before treating an
event as inbound. That is a separate trap and it is documented in
`docs/NEXT_SESSION.md`.
