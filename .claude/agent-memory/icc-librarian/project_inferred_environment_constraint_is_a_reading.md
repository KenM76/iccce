---
name: iccce-inferred-environment-constraint-is-a-reading
description: DL-060 (VERIFIED 2026-08-18, promoted from REPORTED) — "the Read tool cannot render PDF pages here" was inferred from ONE failing call and written into permanent docs as a fact; it was false (pdftoppm absent, but pypdfium2 renders and Read reads the PNG), and what it cost was the only INDEPENDENT check on a transcribed equation, because all three text engines drop Symbol-font glyphs TOGETHER
metadata:
  type: project
---

**A constraint an agent infers about its own environment is a READING,
not a fact, and gets labelled as one.** "Tool X failed once" licenses
*"invocation X, with these arguments, on this date, failed"* — never
*"this environment cannot do Y."* Before a capability claim enters a
permanent document, **name the exact invocation** and **state what was
not tried**.

**Why:** on 2026-08-18 `icc-spec-librarian` retracted *"the Read tool
cannot render PDF pages in this environment"*, which it had
**overgeneralised from a single failing invocation** and which had
propagated into seven of its corpus files and into `iccce`'s `LEGAL.md`,
`NUMERIC_CLAIMS.md` and `SESSION_LOG.md`, plus this agent's own memory.

**The narrow correct claim — do not round it up.** ★★★ ***"PDF pages can
be rasterised via `pypdfium2` and read"***, **NOT** *"PDF reading
works"*. `pdftoppm` really is absent and the Read tool really does refuse
a `.pdf` handed to it directly. A future session **will** meet that exact
error message; it must find the workaround, not re-derive the wrong
conclusion. Recipe: `p.PdfDocument(f)[i].render(scale=3.2).to_pil()`,
**0-based index**, crop, save PNG, Read it. Canonical copy
`ICC_Spec\LEGAL_NOTE.md` §1b; also in DL-060 because `docs/` must not
depend on an uncommitted corpus for a capability it asserts.

**★★ The cost was evidential, not convenience.** ICC.1:2022 sets `+`,
`−`, `×`, `≤`, `≥` in the **Symbol font**, and **all three text engines
(`pypdf`, poppler, `pdfminer.six`) drop them for the same reason.** So
multi-engine agreement is **correlated, not independent**, wherever
meaning turns on a sign or an inequality — the concordance measures the
shared failure mode, not the quantity. **A raster is the only independent
channel.** This is [[iccce-agreement-can-be-the-symptom]] arriving in the
sourcing layer.

**★★★ Three instances, not two** — and the third is the argument:

| # | where | inferred | actually |
|---|---|---|---|
| 1 | `pdfce` 2026-08-08 | "session forbids subagent dispatch", filed in 3 docs | subagents were in use throughout (global `CLAUDE.md`) |
| 2 | `ICC_Spec` 2026-08-12 (**C6**) | "`itu.int` WAF rejects every agent request" | it rejects the bare UA `Mozilla/5.0` and only that — one `curl` away for 5 days |
| 3 | here, 2026-08-11→17 | "cannot render PDF pages" | `pypdfium2` works |

**C6 already produced the right rule** (state tool, flags, UA, status,
response size) — **and instance 3 happened anyway, because that rule
lives in `ICC_Spec\LEGAL_NOTE.md` §4 and had no counterpart in
`docs/ARCHITECTURE.md` §5.** ★ **A rule recorded only in the corpus does
not bind this repository's documents.** That gap is what DL-060 closes,
and it is a better reason to file than the pattern.

**Not a duplicate of [[iccce-negative-finding-removes-its-auditor]]
(DL-042).** DL-042 is the remedy applied *later* — re-read the REASON an
item is owed. DL-060 is the check applied *at the moment of writing*.
They compose: DL-060 stops the false negative being filed, DL-042 catches
it if it is.

**How to apply:** when any dispatch or document asserts what this
environment can or cannot do, treat it as a claim needing an invocation.
When filing a *correction* to such a claim, **do not repeat the shape** —
this librarian has no shell, so DL-060 records the capability as
`[REPORTED]` with a named first use (Cholewo 2000 Eq. (1), promoted
RECONSTRUCTION → VERIFIED off a rasterised page, `≤` glyphs recovered),
and §7.21 owes the invocation to whoever renders next. See
[[iccce-verify-own-draft-too]].

**Live consequence:** NC-230's condition (b) — a second reading of ICC's
`srgb.pdf` §B.2 — was recorded as *barred*; it is **unblocked and merely
undone**, and must be a **raster**, not a fourth text extraction.

★ **The dispatch that carried this correction was itself wrong in a
specific:** it named a *"Sharma & Starr row in `TOLERANCES.md`"* with a
`DERIVED` `<` glyph. **`Starr` has zero hits in this repository** — the
paper is in the sibling's corpus, and the glyph it means is Cholewo Eq.
(1)'s `≤`, already discharged there.
[[stale-citation-worse-than-stale-number]] from the other end, for
the second filing running.

**★★★ PROMOTED TO VERIFIED, 2026-08-18 — one day later, by a shell.**
`icc-engineer` ran it: **`which`** confirms `pdftoppm` absent; a
**1225×1619** PNG from `PdfDocument(…)[0].render(scale=2).to_pil()`; Read
displayed it. `NUMERIC_CLAIMS.md` §7.21's owed item 3 is **discharged**.
★ **Not rounded up:** the dispatch elided the path, and the run used
**`scale=2`** where the recipe says `3.2` — **the call SHAPE is verified;
the scale and the 0-based index are not measured claims.**

**★★★ The stumble is the better half: `pypdfium2.V_PYPDFIUM2` RAISED
while the render SUCCEEDED.** This entry's own error, met *inside* the
act of verifying its retraction. **A capability is falsified by the
capability failing, never by its metadata failing** — and the version
probe is both the most natural first move against an unfamiliar library
and the call least entitled to speak for it.

**★★★ A SECOND failure mode, broader than the Symbol-glyph one.** The
first use read two facts off a page — GWG 23.0's four panel values and
its declared intent — that are **set in a FIGURE**. `pdftotext -layout`
returns the prose and omits them. ⇒ **for figure-borne content, engine
agreement is not correlated, it is VACUOUS: all three return nothing and
their shared silence reads as ABSENCE OF THE FACT.** The glyph mode
**corrupts a value you can see**; this one **hides that a value exists**,
and no amount of cross-engine comparison can surface it. §7.21's owed
sweep widens to *"glyph-sensitive **or possibly set in a figure**"*.
`GHENT_COMPATIBILITY.md` §9 gained a fourth provenance class,
**`[QUOTED-FROM-RASTER]`**, because `[QUOTED]` was defined by naming
`pdftotext -layout`.

**★ First use — keep the two straight.** The **first anywhere** remains
`icc-spec-librarian`'s Cholewo 2000 Eq. (1) promotion in its corpus.
**The 2026-08-18 one is the first in THIS repository, the first by a
shell-holding agent here, and the first whose product is a correction to
this project's own documents.** It is the better *demonstration* — the
corpus case recovers glyphs the engines mangle, this one recovers content
they never mention — but **it is not the earlier one, and the record is
not rewritten.**

Related: [[iccce-source-labelled-number]] (DL-061, what the first use
found), [[iccce-pass-status]],
[[iccce-absence-of-publication-is-not-evidence]],
[[iccce-count-from-a-sample-is-not-the-population]].
