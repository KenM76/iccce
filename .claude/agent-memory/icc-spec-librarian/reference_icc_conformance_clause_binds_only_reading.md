---
name: icc-conformance-clause-binds-only-reading
description: ICC.1:2022 clause 5 requires of a CMM only the ability to READ profiles — no clause binds computed transform output, so "implementation X is non-conforming" is almost never a sentence this corpus can write; say "diverges" instead
metadata:
  type: reference
---

**ICC.1:2022 clause 5 "Conformance", in full for consumers, VERBATIM:**
*"Any colour management system, application, utility or device driver
that claims conformance with this ICC specification **shall have the
ability to read the profiles as they are defined** in this ICC
specification."*

**Reading. That is the whole requirement.** Nothing in ICC.1:2022 binds a
CMM's *computed transform output* to the profile's computational model.
The computational-model `shall`s are phrased about the **profile** —
8.3.3 / 8.4.3: "The computational model **supported by** three-component
matrix-based … profiles shall be that defined in F.3." They fix what the
profile's data *means*, not what a consumer must compute from it.

**Consequence, and it recurs on every "settle it from the spec"
dispatch:** even when an implementation demonstrably computes something
the mandated model cannot produce, the available verdict is
**"diverges from clause N"**, not **"is non-conforming."** Write the
former. The distinction is not hedging — it is the difference between a
claim the document supports and one it does not.

Logged in the corpus as **A39b (SILENT)** in
`icc\icc__ref__ambiguity_register.md`, with its first use in
`icc\icc__s__computational_models.md` §4.4 and `M3`. Same family as
**A33** (two conformant CMMs may select different tags) and **A37** (no
duty to disclose an intent fallback): **ICC.1 specifies profiles
thoroughly and consumer behaviour barely.** When a dispatch asks "is
implementation X conforming?", check whether the standard even has a
clause capable of answering before reasoning about the arithmetic.

**Companion fact, same pass, worth having to hand:** ICC.1:2022 bounds
every processing stage by clamping its **input**, never its output
(F.8–F.16, 10.12.3, 10.13.3, 10.16.2.4) — outputs stay in range because
the next element's domain is `[0,1]`. **Clause 6.5's permission for
device values outside `[0,1]` is gated to float32 in `DToBx`/`BToDx`
only**, because `mpet` is the sole model with unbounded element output.
Do not read clause **6.4**'s "No clipping is performed" as a device-value
permission — 6.4 is PCSXYZ↔PCSLAB and every quantity in it is a PCS
value. (`iccce` `TOLERANCES.md` NA-003 makes exactly that
mis-attribution.)

Related: [[reading-source-is-not-observing-behaviour]],
[[lcms2-measured-behaviour-file]], [[label-the-predicate-not-just-the-payload]]
