# iccce — session log

**Append-only.** A session's entry is written once and not edited
afterwards; if a later session discovers that an earlier entry was wrong,
it says so **in its own entry**, naming the entry it corrects. An edited
history cannot be used as evidence, which is the only thing a history is
for.

Owned by `icc-librarian`, who **has no shell** — every measured statement
below arrived in a dispatch carrying its evidence, or was read out of a
file in the working tree. Statements are labelled with which:

| Label | Means |
|---|---|
| **verified** | The librarian read it, in the live source, this session. |
| **reported** | An agent ran it and carried the result. Not re-run here. |
| **unverified** | Neither. Recorded as an open question, never as a fact. |

Entry format: date, what changed, what was measured, what was decided,
and what the next session must not assume.

---

## 2026-08-11 — Pass 0: scaffold, parser, oracle, corpus

**First working session of the project.** The tree entered the day
containing a plan and an agent roster and no code.

### What was built

- **Workspace** — four crates per `ARCHITECTURE.md` §1
  (`iccce-color`, `iccce-profile`, `iccce-cmm`, `iccce-cli`),
  `unsafe_code = "deny"` workspace-wide, lossy-cast clippy lints at
  `warn`. `tools/difftest` is **deliberately not a workspace member**, so
  the shipping crates cannot link the oracle even by accident.
  *(verified — `Cargo.toml`, read.)*
- **CI** — `.github/workflows/ci.yml` builds and tests on
  `ubuntu-latest` **and** `windows-latest`, with `fmt` + `clippy` on
  Linux and `RUSTFLAGS: -D warnings` in CI only. *(verified — file read.
  **Whether it has ever run is unverified**; no run history was checked
  and this librarian cannot check one.)*
- **`iccce-profile` Pass 0 parser** — 128-byte header, tag table,
  malformation reporting, iccMAX identified and **refused by name**,
  hostile `tagCount` bounded *before* allocation. Every layout cites a
  corpus file (`icc__s__header.md`, `icc__s__tag_table.md`,
  `icc__s__number_encodings.md`) and **no ICC.1 clause number** — see
  DL-002. *(verified — `lib.rs`, `tag_table.rs`, `diag.rs` read.)*
- **`iccce-cli inspect`** — prints header, tag table and every
  malformation, one `key: value` per line, as a stable diff surface
  rather than a human UI. *(verified — `main.rs` read.)*
- **The oracle** — lcms2 pinned, built (MSVC), and demonstrated on real
  profiles. *(reported by `icc-conformance`; the recorded evidence in
  `tools/difftest/README.md` §6–§9 was verified as present and
  internally consistent.)*
- **The corpus** — 21 files at `D:\Dev\Rag-Specialized\ICC_Spec\`.
  *(verified — 21 `.md` files enumerated; contents of the
  chromatic-adaptation, ΔE, colorimetry-core, sRGB, divergence and
  ambiguity-register files read.)*

### Pass 0's done-when, met

1. `iccce inspect "…\sRGB Color Space Profile.icm"` → header (`'Lino'`
   CMM, v2.1.0, `mntr`/RGB/XYZ), 17 tags, 0 malformations, with
   `rTRC`/`gTRC`/`bTRC` all at offset 1084. *(reported.)*
2. `transicc` invoked on the same profile → `99.9988 0.0188 −0.0173` for
   white at intent 1, with the full command line recorded.
   *(reported; the record is verified present in `difftest/README.md`
   §8.2.)*

Filed in `ROADMAP.md` as the Pass 0 completion record, **without a commit
hash** — the work was uncommitted when this was written and the commit is
the engineer's act. The record says so and asks whoever commits to fill
it in.

### Gate results — carried, not measured here

`cargo test --workspace` 14/14 pass; `cargo fmt --check` and
`cargo clippy` clean. *(reported by `icc-engineer`, run on this machine.)*
The one thing checkable from the tree without a shell: **14 `#[test]`
declarations exist** — 8 in `crates/iccce-profile/src/lib.rs`, 6 in
`src/num.rs`. *(verified.)* That is a count of tests declared. It is
**not** a count of coverage and **not** a pass result; it is recorded
only because it is consistent with the reported figure.

### Findings that changed decisions

- **lcms2 is not uniformly MIT.** Core and headers are verbatim MIT;
  `plugins/fast_float` and `plugins/threaded` are **GPL-3.0-or-later**,
  stated by upstream in `plugins/README.1ST`. A licence badge would have
  said "MIT" and been incomplete. → **DL-001**.
- **The `lcms2.19.1` tag is lightweight**, therefore mutable, therefore
  not a pin. The commit hash `21c582a…` is the pin, and
  `fetch-lcms2.sh` hard-fails on mismatch. → **DL-001**, and a
  cross-project RAG lesson (below).
- **color.org's ToS blocks automated retrieval**, naming AI/ML training
  explicitly, so the ICC.1 PDF was **not** downloaded — while the site's
  own `robots.txt` permits the specification index. The two point
  opposite ways; the prose contract was taken as binding and the conflict
  recorded rather than resolved silently. → **DL-002**.
- **A2B0 and A2B2 share one tag-table offset (432) in
  `USWebCoatedSWOP.icc`**, so perceptual and saturation are
  byte-identical through that profile. Written into
  `difftest/README.md` §8.4 so it is never misdiagnosed as an engine
  bug at 2 a.m. *(reported, with the tag-table dump.)*
- **The v2 legacy Lab encoding costs ≈0.3–0.5 ΔE — below the 1.0
  anchor**, so a ΔE-graded test would pass while the encoding is wrong.
  → **DL-005**.
- **Duplicate tag signatures**: specification SILENT, observed in the
  wild, so the parser had to choose and the choice had to be visible.
  → **DL-003**.

### One measured verification closed

The corpus's **derived** illuminant hex for D50 (`0xF6D6` / `0xD32D`) was
confirmed byte-for-byte against the system sRGB profile: bytes 68–79 =
`0000F6D6 00010000 0000D32D`. *(reported, with the `xxd` output.)* This
promotes a value the corpus had *derived* to one *observed in a real
file* — a genuine strengthening, and worth noting that it is
**observation of one profile**, not a published constant. A parallel
dispatch was updating the corpus file; **this librarian did not verify
whether that edit landed**, and a later session should not assume it did.

### Deliberately NOT created: `docs/NUMERIC_CLAIMS.md`

Per `NEXT_SESSION.md`, the numeric-claims ledger is created **with the
first measured claim**. Pass 0 produced **no measured colour claim**:
`iccce-color` and `iccce-cmm` are stubs, no transform exists, and nothing
in iccce has been compared to anything. The numbers this session
produced are lcms2 smoke-test outputs (cross-check values from an
implementation, recorded in `difftest/README.md` §8, and explicitly not
transplantable into a unit test) and a byte-level hex confirmation —
neither is a claim about iccce's own accuracy.

**An empty ledger is worse than no ledger**: it invites the first entry
to be something that is not a measurement, and it makes "nothing has been
measured yet" look like "nothing has been filed yet." The ledger gets
created by Pass 1, with the ΔE2000 arithmetic-agreement result against
the Sharma 34 pairs as its first row.

### Filed this session

| Where | What |
|---|---|
| `ROADMAP.md` | Pass 0 marked done (2026-08-11) with an evidence-bearing completion record, a `NOT delivered` list, and a dated annotation on open question **(a)**. Plan text unchanged. |
| `ARCHITECTURE.md` §5 | **DL-001** … **DL-005**, appended to a previously empty log. |
| `SESSION_LOG.md` | Created — this entry. |
| `NEXT_SESSION.md` | Overwritten for Pass 1. |
| `D:\dev\rag\rust\` | `a_lightweight_git_tag_is_a_mutable_label_not_a_pin.md` + index entry. |

Not touched, by instruction and by ownership: `LEGAL.md`,
`TOLERANCES.md` (owned by `icc-conformance` / `icc-spec-librarian`), and
the corpus itself.

### Left for the next session to not assume

- **`README.md` §Status still says "Nothing is built."** It also says of
  lcms2's licence *"Verify that before relying on it"* — which was done
  this session (`LEGAL.md` §4). Both are stale. `README.md` is not the
  librarian's file; flagged for the engineer.
- The Linux build of lcms2, and therefore Linux CI's ability to run any
  difftest, is **unproven** — the script has never executed.
- No `primary_spec` tier in the corpus. **No claim in this project may
  cite an ICC.1 clause number** until DL-002's blocker clears.
