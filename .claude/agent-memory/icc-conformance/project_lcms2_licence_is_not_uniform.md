---
name: project-lcms2-licence-is-not-uniform
description: lcms2's top-level LICENSE is MIT but plugins/fast_float and plugins/threaded are GPL-3.0 — a licence badge would have missed it; re-verify on every pin move.
metadata:
  type: project
---

**lcms2 is not uniformly MIT.** Verified 2026-08-11 at commit
`21c582a` (tag `lcms2.19.1`) by cloning and reading files:

- top-level `LICENSE` — verbatim MIT, "Copyright (c) 2023 Marti Maria
  Saguer", no added clause
- `src/`, `include/`, `utils/transicc`, `utils/linkicc` — MIT
- **`plugins/fast_float/` and `plugins/threaded/` — GPL-3.0-or-later**
- `utils/jpgicc/iccjpeg.c` — IJG licence

Upstream states this itself in `plugins/README.1ST`. **GitHub's licence
badge says "MIT" and would have been incomplete.**

**Why this matters:** iccce is MIT and the project has already paid once
for a licence claim nobody checked. It is also a *correctness* issue, not
only a legal one — `fast_float` swaps in an approximate floating-point
pipeline, and an oracle must be the reference implementation's most
accurate path, or every disagreement becomes ambiguous.

**How to apply:**
- Never enable `LCMS2_WITH_FASTFLOAT` or `LCMS2_WITH_THREADED_PLUGIN`.
  Both build scripts set them OFF explicitly even though upstream
  defaults them OFF.
- **Moving the pin is a licence event, not a version bump.** Re-read the
  top-level `LICENSE`, re-run the `find -iname 'LICENSE*' -o -iname
  'COPYING*'` sweep (that sweep is how the GPL plugins were found), and
  re-check `plugins/README.1ST` for added plugins. Then append a new
  dated subsection to `docs/LEGAL.md` §4 — never edit an existing dated
  verification in place.
- More generally: **verify a dependency's licence by reading the tree,
  not the badge.** Per-directory licensing is common in C projects and
  invisible to classifiers.

Full record with verbatim transcriptions: `docs/LEGAL.md` §4.
Related: [[project-oracle-and-tolerance-state]].
