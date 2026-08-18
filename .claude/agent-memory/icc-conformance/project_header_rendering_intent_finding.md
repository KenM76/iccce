---
name: project-header-rendering-intent-finding
description: Measured 2026-08-18 — neither iccce nor lcms2 consumes a profile header's renderingIntent on a non-DeviceLink profile; lcms2 reads it in exactly ONE place (DeviceLink, cmsIsCLUT); the library API had no "unspecified" state so only the CLI arm of the test could catch the rival hypothesis.
metadata:
  type: project
---

Built the synthetic pair `v2-rgb-header-intent-perceptual.icc` /
`…-relative.icc` (byte-identical except **file offset 67**, the low byte of
`renderingIntent` at header offset 64) plus
`crates/iccce-cli/tests/header_rendering_intent_not_consumed.rs` and
`docs/TOLERANCES.md` §3.11.

**Why:** the operator measured the behaviour on two licensed Ghent profiles
and could not put any number from them into git
(`docs/GHENT_COMPATIBILITY.md` §2.3), so the finding had to be reproduced from
scratch on bytes this project owns.

**How to apply:** four things below are durable and none is derivable from the
code.

## 1. ★ lcms2 reads the header intent in exactly ONE place: DeviceLink

`grep HeaderRenderingIntent` over the pinned vendor tree: every other mention
**writes** the field (`cmsvirt.c`, `linkicc.c`, `cmscgats.c`). The single read
in the transform path is `src/cmsio1.c`, `cmsIsCLUT`, verbatim comment *"For
devicelinks, the supported intent is that one stated in the header"*.

So **"lcms2 ignores the header intent" is only true of non-DeviceLink
profiles**, and any future statement must carry that qualifier. It is also the
best lead for the outstanding `icc-spec-librarian` dispatch: the specification's
answer may be **device-class-dependent**, and the corpus has no `link` fixture
at all.

## 2. The two engines ignore the same field while defaulting to DIFFERENT intents

`transicc.c` declares `static cmsUInt32Number Intent = INTENT_PERCEPTUAL`;
iccce's CLI default is media-relative. So on the member whose header says **1**,
lcms2's no-flag run chose **0** — the header value was present, disagreed with
the default, and lost. **That is a stronger observation than either engine
alone**: a shared default could have been mistaken for the header being
honoured.

Measured (transicc 5.1 / LittleCMS 2.19, dst `v4-rgb-matrix-trc.icc`, in
`128 128 128`): both members identical at no-flag, `-t0`, `-t1`, `-t2`, `-t3`.

## 3. ★★ Where you put the test decides whether it can fail

`Chain::new(src, dst, intent)` takes intent as a **required parameter** — there
is no "unspecified" state at the library surface. So the library half of the
measurement **cannot** detect the rival hypothesis; proved it by injecting the
rival (CLI patched to honour the header when no `--intent` given): only the
arm that shells out to `env!("CARGO_BIN_EXE_iccce")` went red.

Consequence: the file lives in **`crates/iccce-cli/tests/`**, not
`iccce-cmm/tests/` where every other transform test is. When the observable is
"what happens when the caller says nothing", the only surface that has a
"nothing" is the CLI.

## 4. The zero-separation trap, reproduced live on a new fixture

Second injection: made the generator emit the same table for both intents.
**The headline test still PASSED** — vacuously — while the control failed at
`0.000000` separation. Same lesson as [[project-passk-f-separating-fixture]],
now demonstrated rather than argued, and worth re-running as the standard way
to show a control is load-bearing.

Design consequences for a separating fixture: the two tables differ at **every**
node (constant `a*`/`b*` offset, not just `L*`, because an `L*`-only split
crosses over and coincides at some input), and every entry sits **inside the
destination gamut** so a clip cannot erase the separation.

## Stale prose counts found on the way

`fixtures/synthetic/README.md` said **41** fixtures when there were 41 and had
said 38 before; `tools/gen-profiles/README.md` §1 still said **38**. Both are
typed numerals in *present-tense prose* and decay silently on every recipe
added. Fixed, with `gen-profiles list` named as authoritative — but §6's dated
verification records were **left alone**, because those are history.
Same failure family as [[project-stale-claim-strings-in-emitted-records]].

Related: [[project-doc-editing-conventions]],
[[project-prove-the-arm-by-injecting-the-defect]],
[[project-oracle-and-tolerance-state]].
