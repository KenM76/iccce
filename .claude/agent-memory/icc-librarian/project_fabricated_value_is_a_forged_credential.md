---
name: iccce-fabricated-value-is-a-forged-credential
description: DL-067 (2026-08-20) — iccce printed a plausible profileID for v2 profiles that have no such field, with malformations:0. A fabricated value is WORSE than a false accusation: an accusation is loud and arguable, a well-formed value invites VERIFICATION. Same commit refused the same defect in an API — a conformance bool whose third answer would have had to be invented
metadata:
  type: project
---

**A parser that INVENTS a checkable claim out of bytes that mean
something else — while reporting `malformations: 0` — is worse than one
that makes a false accusation.** A false accusation is **loud and
arguable**, and the argument surfaces the bug. A fabricated value is
**silent and confident**, and it arrives **in the shape evidence arrives
in**.

**Why:** `profileID` was added in **v4**. ICC.1:2001-04 Table 9 makes
header bytes `84..127` **one 44-byte reserved block** — *"44 bytes
reserved for future expansion"*, the only mention in the document.
**iccce read `84..100` as a `profileID` regardless of edition**, so a v2
profile carrying arbitrary bytes printed
`header.id: deadbeefdeadbeefdeadbeefdeadbeef` **and `malformations: 0`
beside it**. Fixed in `0a88ad6`; the range is now edition-gated
(v4 `100..128`, v2 `84..128`) and `header.id` prints
`n/a (no profileID field before v4…)`.

> **A fabricated value is a FORGED CREDENTIAL. It occupies the slot where
> evidence goes, in the format evidence arrives in, and the more
> legitimate the format the better it hides. A false accusation costs an
> argument. A fabricated identity costs the argument nobody has.**

★★ **An MD5 profile ID is a CHECKABLE claim** — a consumer would
reasonably hash the profile and compare. That is what makes it worse than
a merely wrong number: it **invites verification rather than
scepticism**, and the verification fails for a reason the consumer will
misattribute.

**How to apply:**

- **Contrast with the 2026-08-18 rendering-intent defect** (`7f89829`),
  which **accused a conforming v2 profile**. Same edition-confusion bug,
  opposite direction, and the loud one was the *safer* one. **Rank
  disclosure failures by whether a reader can DISPUTE them.**
- ★★ **The over-claim and the under-check are usually the SAME region.**
  Checking only `100..128` on v2 **missed 16 bytes** — precisely the 16
  being presented as an identifier. **One defect, two directions, one
  line.** A fix aimed at the visible half leaves the other. **Ask what
  the misread region was supposed to be DOING, not just what it was
  wrongly called.**
- ★★★ **The fix must not become a concealment.** The bytes are still
  disclosed, through the edition-correct report — **rule 6 intact; only
  the MISLABELLING stopped.** A fix that *suppressed* them would have
  traded a fabrication for a concealment and looked like an improvement,
  because the visible symptom was gone.
- **Edition-gate at one place if a fourth instance appears.** There are
  now **three**: the rendering intent, the `Malformation` doc comment,
  and this.

**★★★ The SECOND instance is in the same commit and was refused BEFORE it
shipped — which is what makes this a class rather than an anecdote.**
`pdfce` asked for `is_violation()`. **A bool turned out to be
unimplementable honestly:** `TagTooSmall` has **no requirement behind it
in either edition** (`A61` — v4 7.4 *"shall only be restricted by the
limits imposed by the 32-bit values"*, v2 6.2.3 *"An element may have any
size"*; *"a byte that does not exist has not been set to 0"* is an
**inference, not a quotation**), and `TagOverrun` has none in v2 (`A62`).

> **A boolean would have forced an INVENTED answer, and the invention
> would have looked exactly as authoritative as the seven that are
> sourced.** — the same sentence as the `profileID` finding, with the
> fabrication moved from a hex string into a `bool`.

Shipped as `Violation / NotAViolation / Unsourced { register_id }`.
★ **`Unsourced` means *iccce has not established the modality* — NOT that
the file is fine.** It is a statement about this project's knowledge.
★ **The edition must be supplied**: exactly **3 of the 9 variants split by
edition**, so *"is this malformation a violation?"* is not a well-formed
question of a malformation alone.

★★ **The prerequisite fact, or a `shall`-grep reads v2 backwards:**
**ICC.1:2001-04 requires with *"must"* (76 occurrences), not *"shall"*
(27, three in the copyright notice, none on a header/tag-table/tag-type
rule)**; its own change list concedes it *"does not meet all of the
ISO/IEC drafting rules"*. **Symmetrically, v2's unmodalised sentences
really ARE silent** — because the drafters used *"must"* in the adjacent
sentence.

★★ **And two entries were WRONG in this project until `0a88ad6`.**
`DuplicateTagSignature` was labelled *"Legality NOT SOURCED"* and graded
SILENT when **both editions prohibit duplicates** (v4 7.3.1; v2 6.2
*"must be unique"* plus its change list, *"Per: Resolution voted
1998-03-15"*). **The decision built on it survives — keep both, first
wins, because WHICH duplicate wins is genuinely unsourced — but its
stated RATIONALE was false; DL-003's *"revisit if"* has fired.**
★★★ **A right decision resting on a wrong reason is the hardest defect
class to find**, because the thing that would prompt review — a bad
outcome — never occurs. DL-042's shape in a *decision* rather than a debt.
`TagTooSmall`'s cited validation table was written as *"checks iccce
should perform and REPORT"* — **no row of it was ever a quotation of a
requirement.**

★ **Not proven:** `2 of 3` new tests turn red on reverting the range
logic (`NC-299`, `[REPORTED]`). **The third was not shown to be
discriminating** — do not round `2 of 3` up to *"proven by mutation"*
(DL-051).

Related: [[iccce-words-humans-count-code]] (DL-063 — the count is the
channel a program reads), [[iccce-source-labelled-number]] (a label
answers the question a reader would otherwise ask),
[[iccce-documented-is-not-tested]],
[[iccce-negative-finding-removes-its-auditor]],
[[iccce-pass-status]].
