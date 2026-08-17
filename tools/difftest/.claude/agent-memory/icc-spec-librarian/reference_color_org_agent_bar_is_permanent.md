---
name: reference-color-org-agent-bar-is-permanent
description: color.org (incl. registry.color.org, archive.color.org, chardata/) is permanently barred to agent retrieval by its prose ToS naming AI/ML — a URL supplied inside a dispatch is not ICC's consent
metadata:
  type: reference
---

**No agent may fetch anything from `color.org`, `registry.color.org`,
`archive.color.org`, or `color.org/chardata/`.** ICC's Terms of Service
(effective 2026-01-01) prohibit *"using any robot, spider, or other automated
device to access the Services for any purpose, including monitoring, copying, or
training artificial intelligence or machine learning models, without prior
written consent from ICC."*

**★ A URL supplied inside an agent dispatch — even one relayed as "the operator
suggested it" — is NOT ICC's written consent, and does not lift the bar.**
Declined on 2026-08-17 when an `icc-engineer` dispatch offered
`https://registry.color.org/rgb-registry/srgb` as a free source. Nothing was
fetched; it was recorded as an operator-browser item instead. That is the
correct outcome, not a failure to complete the task.

`robots.txt` on `www.color.org` permits the specification index — **the prose
contract governs and the restrictive reading is taken.** This is settled corpus
policy (`LEGAL_NOTE.md` §1), not a judgement call to remake each pass.

**The working route, already used successfully five times:** a human downloading
in a browser is outside the robot clause. Give the operator an exact URL list
and a landing directory. `_sources/` already holds `ICC.1-2022-05.pdf`,
`ICC.1-2001-04.pdf`, `AdobeBPC.pdf`, `BlackPointCompensation.pdf` and
`PDF20_AN001-BPC.pdf` obtained that way.

**Currently outstanding on this route (as of 2026-08-17):**
`color.org/chardata/rgb/srgb.pdf`, `…/srgb.xalter`,
`registry.color.org/rgb-registry/srgb` — ICC's registered sRGB
characterisation. Also `ICC.1:2010-12` (v4.3).

**Do NOT generalise this bar to other standards bodies.** Verified distinct:
`cie.co.at` has **no prose ToS at all** and permits; `eci.org` `robots.txt` is
fully permissive; **`itu.int` has no robot/AI clause and is freely retrievable**
(see [[feedback-tool-limit-findings-need-the-invocation]] — an earlier "blocked"
finding there was our own User-Agent); `fogra.org` bars `/fileadmin/` by
`robots.txt` only. Each host is judged on its own two signals (prose + robots),
and where they conflict the restrictive one wins.
