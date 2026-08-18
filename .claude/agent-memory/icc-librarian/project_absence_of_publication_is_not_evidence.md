---
name: iccce-absence-of-publication-is-not-evidence
description: DL-054 — an ACCESS boundary (color.org's robot bar) got recorded as an EXISTENCE fact ("NO document publishes them"), so the shipped sRGB file was promoted to ground truth by elimination; the ~12 ULP blue-Z residual was the FILE's, not iccce's, and the mis-attribution survived because the NUMBER was correct
metadata:
  type: project
---

**The absence of a published value is not evidence that the artifact you
hold IS the reference.** Filed 2026-08-17 as `ARCHITECTURE.md`
**DL-054**, `NUMERIC_CLAIMS.md` **§3.33** (NC-230 … NC-233, §3.33.4,
§3.33.5) and **§4's NA-011**.

**Why:** for the life of this project's sRGB work, reconstructing the
shipped HP 1998 profile's colorants from sourced chromaticities missed
`bXYZ.Z` by **~12 ULP**. Three routes failed to close it. The residual
was registered as **iccce's** rule-4 approximation and asserted to stay
in `11.0..13.0` ULP.

The operator then downloaded, **in a browser**, ICC's own **"How to
interpret the sRGB color space (specified in IEC 61966-2-1) for ICC
profiles"** (Jack Holm, ICC, 2015-04-27). **§B.2 publishes the
D50-adapted colorants — and ICC's recommended D65→D50 `chad` — at 15
decimal places.**

| | worst cell | `bXYZ.Z` |
|---|---|---|
| **iccce's from-constants construction** | **3.02 ULP** | **0.90 ULP** |
| the shipped HP 1998 / `sRGB2014.icc` file | **11.13 ULP** | **11.13 ULP** |

★★★ **The residual was the FILE's.** iccce is ~3.7× closer to ICC's
published values than the world's most-deployed sRGB profile is.

**Why it survived, and it is not "nobody checked": the NUMBER was
right.** Every measurement was correct; every failed route genuinely
failed. **Only the OWNER was wrong**, and no re-measurement could catch
that, because both candidate owners produce the same residual. **Only a
new document could settle it** — which is why the item that would settle
it must never be de-prioritised by reasoning.

**The mechanism, three parts:**

1. **An ACCESS boundary was recorded as an EXISTENCE fact.** The document
   sat behind `color.org`'s robot bar. *"No agent may fetch it"* →
   *"it is not fetched"* → *"nothing publishes these values."* ★★★ This
   is **DL-041's taxonomy failing in practice for the first time**
   ([[iccce-ground-truth-cannot-exist]]): *existence*, *availability* and
   *access terms* are three different blockers, and the LUT path's
   absence is **structural** while this one was merely **unfetched**.
2. **Both registers lived in ONE corpus file and the flat one
   propagated.** `ICC_Spec/iec/iec__s__srgb.md`'s **acquisition list**
   said *"no document **found so far** states"* and named the barrier, who
   could pass it, and that not fetching was *"a reported tool/permission
   limit, not an untaken action"* — exactly right. Its **status table**,
   100 lines away, said ***"NO document states them."*** **The status
   table is what got quoted**, into a doc comment, into
   `DEFAULT_DESTINATION.md`, and into the ledger. **Summaries propagate;
   that is what summaries are for.**
3. ★★★ **The expectation of an UNREAD document was revised DOWNWARD from
   evidence about a DIFFERENT artifact** — hours before the fetch, on the
   grounds that `sRGB2014.icc` was byte-identical to HP 1998, the corpus
   wrote *"★ EXPECTATION LOWERED … no longer likely to close the colorant
   gap on its own."* It closed it outright **and reversed an
   attribution.** You cannot estimate an unread document's contents from
   the files it describes.

**How to apply.**

- **Write the SEARCH claim, never the EXISTENCE claim.** *"No document
  found so far states X; here is what was looked at and what is barred"*
  is falsifiable, dated and invites the next fetch. *"No document states
  X"* is a claim about the literature nobody here can make.
- **A gap in the literature is not a licence to promote an implementation
  to ground truth.** If the only reference is an artifact, the row is
  `measured_file_behaviour` / `constructed-vs-reference-file` and the
  residual is **UNATTRIBUTED** — a distance between two things, not a
  cost of ours.
- **Re-audit the summary register, not just the careful one.** The
  careful sentence being present somewhere in the file did not help.
- ★★ **Same-day counter-example, and it is NOT a contradiction.** Pass H
  found ICC's `Probe2 Profile Readme` states in numbers what
  `Probev2_ICCv4.icc` does — **and the published claim is FALSE of the
  file it names**; those rows went to REPORTED at infinity. **A document
  stating intended VALUES is a definition and outranks any file; a
  document stating what a FILE does is an empirical claim and can be
  falsified by that file. Ask which kind it is before deciding what it
  outranks.**

**The consequences that are still live:** `ICC_Spec` still carries *"NO
document states them"* at three places (owed to `icc-spec-librarian`);
**a second reading of §B.2 is owed** (one agent, one transcription —
~~and the librarian could not open the PDF (`pdftoppm` absent)~~ ★
**CORRECTED 2026-08-18: that reason was FALSE. `pdftoppm` is absent but
`pypdfium2` renders and Read handles the PNG, so the second reading is
UNBLOCKED and merely undone — and it must be a RASTER, not a fourth text
extraction, because a 15-dp table in a Symbol-font document is exactly
where text engines fail TOGETHER.** See
[[iccce-inferred-environment-constraint-is-a-reading]]); the document
contains **two transcription defects in §B.1**; and **the ground-truth
row for chromatic adaptation moves from BLOCKED to
AVAILABLE-AND-UNMEASURED**, because ICC's recommended `chad` is in the
same §B.2 — DL-042's rule working ([[iccce-negative-finding-removes-its-auditor]]).

Related: [[iccce-pass-status]], [[iccce-ground-truth-cannot-exist]],
[[iccce-compatibility-not-certification]],
[[iccce-stale-citation-worse-than-stale-number]],
[[iccce-count-from-a-sample-is-not-the-population]].
