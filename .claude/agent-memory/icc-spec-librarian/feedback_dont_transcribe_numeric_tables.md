---
name: dont-transcribe-numeric-tables
description: Never transcribe a numeric data table through my own output — download it into _sources/ and write a corpus file that POINTS at it with provenance, checksums and spot-checks. Also, write each finding to disk AS IT IS ESTABLISHED, never in one pass at the end.
metadata:
  type: feedback
---

**Two rules, both adopted 2026-08-12 after a run of this agent died mid-flight.**

## 1. Do not transcribe numeric tables through my own output

**Download the source file verbatim into `_sources\` and write a corpus file
that POINTS at it** — recording provenance, licence terms, format, column
meanings, wavelength/index range and interval, and a handful of **spot-check
values** a future consumer can verify a replacement file against. **The corpus
file describes and cites; the data file IS the data.**

**Why:** the previous run died while generating an 81-row CMF table inline. That
was both *what killed it* and *the most error-prone possible route* for exactly
the data this project is most careful about — **a mis-typed tristimulus value is
the archetypal silent colour error**, indistinguishable from a correct one
without measurement. This is the same discipline already applied to
`_sources\ICC.1-2022-05.pdf`: the PDF is not transcribed, it is cited.

**How to apply:** any table over ~10 rows of numbers. Prefer a publisher that
ships **checksums and validation values** (CIE's `CIEmetaDigitalProduct` metadata
carries md5, sha256, `sumOfColumns` and `sampleRow`) — then the corpus file can
carry a *verification certificate* instead of a copy. Good spot-checks are
structural, not arbitrary: **ȳ = 1.0 at exactly 555 nm** proves column order;
**z̄ = 0 for all λ ≥ 700 nm** proves the file is not truncated or shifted.

**Corollary that bit on the same day:** an arithmetic claim written into a file
*from my own head* is the same failure. I wrote `Z = (1−x−y)/y = 0.825006` into
a corpus file and it was wrong (`0.825188`), caught minutes later by `decimal`.
**Run every number, including the ones that feel like mental arithmetic.**
See [[derived-values-need-a-second-pass]].

## 2. Write incrementally — a half-written file beats a perfect unwritten one

**Write each finding to its corpus file AS IT IS ESTABLISHED, not in one pass at
the end.** An "IN PROGRESS" banner on a partial file is worth infinitely more
than a complete file that never gets written.

**Why:** the run that died had accumulated all its research in context and **lost
every bit of it** — the retry started from an empty `cgats\` directory. Corpus
work is long, tool-heavy and interruptible by API error; context is not storage.

**How to apply:** as soon as a finding is *established* (verified, cross-checked),
write it. Then continue. Amend the file later rather than deferring the first
write. This also makes the file itself the working notes, so a retry resumes
instead of restarting.

Related: [[corpus-defects-are-caught-from-outside]],
[[derived-values-need-a-second-pass]], [[measurement-profiling-sourcing-state]]
