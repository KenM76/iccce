# fixtures/synthetic

Profiles authored byte-by-byte by this project's own generator
(`tools/gen-profiles`, built during Pass 2). Unrestricted, category (a)
per `docs/LEGAL.md` §3 — **prefer these for everything**.

**46 fixtures as of 2026-08-18: 18 well-formed, 28 deliberately malformed, and
0 disputed.** (`gen-profiles list` is the authoritative count; the figure here
is a typed numeral in prose and is dated for that reason — it has already been
wrong twice.) The malformed group is the one usually missing from a fixture
corpus and the more valuable here, because `docs/ARCHITECTURE.md` §3.2 makes *reporting* — not repairing —
the parser's contract, and a contract with no failing input has no test. Each
malformed fixture carries exactly **one** named defect: a fixture broken in two
ways cannot tell you which one the consumer reported.

★ **The third category is an admission rather than a claim, and it is
currently EMPTY.** A **disputed** fixture is one whose *required consumer
behaviour cannot yet be written down* — the specification has not been read on
the exact point the fixture probes, so neither “this must report” nor “this must
be silent” can be recorded without inventing the answer. Its `MANIFEST.md` row
therefore carries a **dated measurement of what iccce does today**, not a
requirement on a consumer.

★★ **The category emptied on the day it was created, and that is it working.**
Its only member was `v2-rendering-intent-high-bits.icc`, filed disputed on
2026-08-18 while the question *“does ICC.1:2001-04 restrict a v2 `renderingIntent`
field to 0..3?”* was outstanding. `icc-spec-librarian` returned the clause text
the same day: 6.1.11 defines four values, contains **no `shall`**, and never
uses the *“other values are reserved”* formula the same document uses elsewhere
when it means to close a set. The fixture moved to **well-formed** and its row
now asserts a **silence** — a report iccce must not make. Nothing about the
bytes changed; only what could truthfully be claimed about them. That
transition is the entire purpose of the category, and the empty state is the
record that it completed.

**What puts a fixture here** — one condition, and it is about the *sourcing*,
never about the *code*: the bytes exercise a rule whose text has not been read
on the exact point at issue, so a dispatch to `icc-spec-librarian` is
outstanding and its answer would change which real category the fixture belongs
to. **What does not**: (a) *iccce’s behaviour is believed wrong* — that is a
defect, and the fixture is filed under what the **standard** says, which is what
makes the suite go red; a category is not a place to park a bug. (b) *the text
has been read and licenses more than one consumer behaviour* — that is settled,
not disputed; the fixture is filed under what the file **is**, and the project’s
own choice is recorded in the row **as a choice**.
`v2-rendering-intent-low-half.icc` is the worked example of (b): it violates
nothing, and iccce reports it anyway, deliberately.

**Every fixture in this directory is reproducible from a generator invocation
recorded alongside it** — a synthetic fixture that cannot be regenerated is just
a binary blob with better branding. The invocations are in
[`MANIFEST.md`](MANIFEST.md), which is itself *generated* so the record cannot
drift from the artefact:

```text
cd tools/gen-profiles
cargo run -- all ../../fixtures/synthetic
cargo run -- verify ../../fixtures/synthetic     # byte-for-byte, names the
                                                 # first differing byte
cargo run -- manifest > ../../fixtures/synthetic/MANIFEST.md
```

Generation is deterministic: no clock, no RNG, no environment. **Do not edit a
fixture by hand** — `verify` will fail, which is the point.

★ **Nothing here is a colorimetric reference.** The colorant columns are an
arbitrary split of the encoded D50 white point (chosen so they sum to it
exactly), the tone curves are exact powers of two or linear ramps, and the CLUTs
are simple documented functions of their grid indices. These files are evidence
about **structure** — that bytes in a stated layout decode to stated values —
and never about colour. See `tools/gen-profiles/README.md` §4.

★★ **One fixture is an INSTRUMENT rather than a sample, and the distinction is
worth knowing before quoting it.** `v2-cmyk-chromatic-neutral.icc` exists so
that the predicate *"K-only in implies K-only out"* has **two different
answers** on a profile that needs no licence: its `B2A0` separates a neutral
into all four inks by construction, and a black-preserving consumer and a
non-preserving one differ by **`0.420 705`** in device units on its K ramp. Its
sibling `v2-cmyk-mft2-lab.icc` cannot do that — its `B2A0` emits `[0,0,0,k]` at
every node, so the two answers coincide. **Both are kept**: the second is the
measurement that says why the first had to exist.

Its two CLUT models are **affine with no cross terms**, and its `B2A0` is
`a*`/`b*`-**independent across three node lines about the neutral axis**, both
on purpose — those properties are what make every expectation taken from it
exact for *any* conformant interpolation scheme rather than only for the one a
particular consumer uses. That also means it is **less like a real press than
any profile in the Ghent corpus**, and a number measured on it is evidence
about the *predicate*, never about ink. `tools/gen-profiles/src/recipes.rs`
carries the full derivation; `docs/TOLERANCES.md` §3.10.11 carries the
tolerances that rest on it.

★★ **A second instrument, added 2026-08-18: a PAIR, and the pair is the
instrument.** `v2-rgb-header-intent-perceptual.icc` and
`v2-rgb-header-intent-relative.icc` are **byte-identical except for one byte,
at file offset 67** — the low byte of the header's `renderingIntent` field
(clause 7.2.15), `00h` against `01h`. Neither member means anything on its
own; the *difference* is the whole apparatus, and it exists so the question
*"does a profile's declared intent reach the transform when the caller names
none?"* has an answer that is a measurement rather than a reading of the code.

Two properties make it able to answer that, and both are choices:

* **Its `A2B0` and `A2B1` differ at every CLUT node** — 12 units apart in both
  `a*` and `b*`, so no input can produce equal output from the two tags. A pair
  whose two tables coincided would let "the outputs are identical" pass while
  the mechanism under test was fully live: a zero-separation fixture does not
  merely fail to inform, it turns a red result green.
* **Every table entry is well inside the sRGB gamut**, so the separation cannot
  be erased by clipping at a destination.

Measured behaviour, the open specification question it does **not** answer, and
the two defect injections that prove the test can fail are in
`crates/iccce-cli/tests/header_rendering_intent_not_consumed.rs` and
`docs/TOLERANCES.md` §3.11.

The verification record (what the shipped `iccce` binary and lcms2's `transicc`
do with each fixture, dated and scoped), the coverage statement, and the open
finding **GP-001** are in `tools/gen-profiles/README.md` §§5–7.
