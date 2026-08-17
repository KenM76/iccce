---
name: feedback-tool-limit-findings-need-the-invocation
description: Never record "site X blocks agents" without the exact curl invocation (tool, flags, User-Agent, status AND response size) — and never send a bare Mozilla/5.0 UA, which is itself what gets blocked
metadata:
  type: feedback
---

**Never write a "site X is blocked / unreachable / bars agents" finding without
recording the exact invocation that produced it: the tool, the exact flags, the
User-Agent if one was set, the HTTP status **and** the response size. And do not
send `-A "Mozilla/5.0"` — the bare truncated string is a canonical bot
fingerprint and is frequently the *cause* of the block.**

**Why:** ICC_Spec defect **C6** (2026-08-17). A 2026-08-12 pass recorded
*"`itu.int` rejects every agent request at the WAF … operator browser download
is the only route"*, promoted it into a **taxonomy** ("a new third access
posture, distinct from `color.org`'s prose ban and `fogra.org`'s robots ban"),
and propagated it to four corpus documents. **It was false.** The rejection was
produced by the agent's own `Mozilla/5.0` header. Measured: `curl`'s default UA
and a *full* Chrome UA both return **HTTP 200 / 644 291 B**; the bare
`Mozilla/5.0` returns **HTTP 200 with a 245-byte rejection body**. ITU-R BT.709
— free, no paywall, the normative source of sRGB's primaries — **sat one `curl`
away for five days.** And because the old record never said what UA was sent,
whether ITU changed the rule or we sent the wrong header **is now permanently
undeterminable.**

**How to apply:**
- Before recording any block, **vary the UA and retry** — one command.
- **Record the response SIZE.** A WAF reject here was a *200* with a 245-byte
  body; a status-only note reads "fine" or "blocked" and is misleading both ways.
- Beware promoting an observation to a *taxonomy*. An "explained" observation
  never gets retested. This is the negative-finding trap (`DL-042`) with a new
  cause.
- **Keep genuine access bars strictly separate from tooling artifacts.** The
  discriminator: *did the publisher say it, or did I infer it from a response to
  a request I composed?* `color.org` (prose ToS naming AI/ML) and
  `fogra.org` (`robots.txt`) are real and stand — see
  [[reference-color-org-agent-bar-is-permanent]].
- A tool-limit claim has **no source to re-read**, which is the same audit hole
  as the `DERIVED` tier. It can only be checked by doing it again — so it must
  carry the recipe.

Full write-ups: `D:\Dev\Rag-Specialized\ICC_Spec\icc\icc__ref__spec_defects.md`
§21, and the cross-project lesson
`C:\personal_rag\claude_code\lesson_20260817_a_waf_block_diagnosed_from_your_own_user_agent_is_not_a_fact_about_the_publisher.md`.
