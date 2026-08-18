# fixtures/synthetic

Profiles authored byte-by-byte by this project's own generator
(`tools/gen-profiles`, built during Pass 2). Unrestricted, category (a)
per `docs/LEGAL.md` §3 — **prefer these for everything**.

**41 fixtures: 15 well-formed and 26 deliberately malformed.** The second
group is the one usually missing from a fixture corpus and the more valuable
here, because `docs/ARCHITECTURE.md` §3.2 makes *reporting* — not repairing —
the parser's contract, and a contract with no failing input has no test. Each
malformed fixture carries exactly **one** named defect: a fixture broken in two
ways cannot tell you which one the consumer reported.

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

The verification record (what the shipped `iccce` binary and lcms2's `transicc`
do with each fixture, dated and scoped), the coverage statement, and the open
finding **GP-001** are in `tools/gen-profiles/README.md` §§5–7.
