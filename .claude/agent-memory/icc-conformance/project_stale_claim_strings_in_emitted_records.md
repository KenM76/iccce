---
name: project-stale-claim-strings-in-emitted-records
description: On iccce, claim-bearing numbers typed into difftest `source`/context strings go stale within a day and are emitted into every conformance record as false facts — format them from the values the harness already computes, never spell them out in the prose beside the code.
metadata:
  type: project
---

**Three found in one sweep on 2026-08-12**, all emitted into every run of
`tools/difftest`:

1. `pass6.rs` — *"17 is the shipped default and the only grid COMPILED_DE is
   derived for"*, still emitted a day after `189e732` made the default **33**.
   It asserted a false fact about the shipped product **inside the artefact
   whose purpose is to be durable evidence**.
2. `pass5c.rs` — *"ISO returned exactly `outRamp[first]` ({iso} against MinL
   {minl})"*, which after `fd34a44` printed **11.772365 against 16.489806**:
   self-refuting inside one sentence.
3. `pass5c.rs` `DISCRIMINATES` — *"the two candidates are only 0.082 `L*`
   apart"*, a literal typed on the morning it was true and false by the
   afternoon (4.799).

**The rule:** a claim-bearing number the harness can compute must be
interpolated at run time (`format!("{DEFAULT_GRID}")`, `{:.6} L* apart`),
never written into prose next to the code that computes it. *A stale comment
misleads a reader; a stale string in an emitted conformance record misleads
the evidence.*

★ **A second, subtler error rides along with the first.** pass6's string also
said *"the only grid `COMPILED_DE` is **derived** for"*, which was wrong on the
day it was written. `COMPILED_DE` is Pass 4's iccce-vs-lcms2 maximum over 341
CMYK points, and **Pass 4 never builds a `CompiledTransform`** — there is no
grid in the bound at all. The grid governs the bound's **applicability** (the
graded quantity is `O(h^1.32)`), never its derivation. When correcting a stale
number, check whether the sentence around it was also confused; *"derived for"*
vs *"graded at"* is the difference between "our tolerance's population is out
of step with the product" (a serious finding) and "a word was loose".

**How to apply.** Before filing any Pass, grep your own apparatus for typed
numerals in `source`/`why`/context strings and ask of each one: *is this a
value the code already holds?* If yes, interpolate it. If it is a historical
value being deliberately preserved, date it and name the commit that moved it.

Related: [[project-doc-editing-conventions]],
[[project-conformance-can-worsen-the-crosscheck]],
[[project-pass6-compiled-path-findings]].
