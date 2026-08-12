//! # Pass 4 — the LUT differential: CMYK → RGB, iccce against lcms2
//!
//! This module answers the **first** part of ROADMAP Pass 4's done-when: a
//! four-channel `lut16` A2B source profile, chained into a matrix/TRC
//! destination, compared against the pinned oracle at every intent iccce
//! implements, with the numbers and their tolerances written down.
//!
//! Read `tools/difftest/README.md` §14 for the prose record of what was run
//! and what it found. This file is the apparatus; the README is the finding.
//!
//! ## The shape of the experiment
//!
//! ```text
//!   grid (CMYK quadruples in 0..1, deterministic — see `grid()`)
//!     │
//!     ├──▶ iccce transform --src SWOP --dst sRGB --intent <n>   (subprocess)
//!     │        → A: destination device RGB, 0..1, 6 decimals
//!     │
//!     ├──▶ transicc -iSWOP -osRGB -t<n> -c0 -n        (subprocess)
//!     │        → B: destination device RGB, 0..255, 4 decimals
//!     │
//!     └──▶ transicc -iSWOP -o*Lab4 -t<n> -c0 -n       (subprocess)
//!              → C: the PCS side alone, D50 CIELAB
//!
//!   compare  A  vs  B/255       in DEVICE space                  → record 1
//!   carry both into D50 CIELAB, compare in ΔE2000                → record 3
//!   compare iccce's own A2Bx PCS against C                       → record 5
//!   the same, at the 16 CLUT-node corners only                   → record 6
//!   the same, with lcms2's OWN interpolation geometry emulated   → record 7
//! ```
//!
//! ## ★ Which side runs where — and a correction made mid-session
//!
//! **The end-to-end comparison drives the shipped `iccce transform` binary as
//! a subprocess**, exactly as Pass 3 does, so both sides cross a process
//! boundary, are printed as text and are parsed back ([`crate::Iccce`]'s doc
//! comment explains why that symmetry is worth the trouble).
//!
//! That was not a foregone conclusion, and the record of why is kept rather
//! than tidied away. This module was first written against the CLI as it stood
//! at commit `63874f9`, whose `transform` subcommand accepted **three**
//! components per line and one intent — no use at all for a CMYK source. The
//! plan was therefore an in-process library call, with the asymmetry declared
//! on every record. **Commit `490191b` ("cli: transform upgraded to the Chain
//! — N-channel input, four intents") landed while this was being written**,
//! which removed the need. The lesson worth keeping is the ordering: the
//! degraded option was going to be *taken and labelled*, not taken and left
//! for a reader to infer.
//!
//! **Two things here are still in-process, and both are labelled as
//! instruments rather than subjects:**
//!
//! 1. [`SourcePipeline`] — the harness's own reimplementation of the `mft2`
//!    pipeline, which exists so the CLUT interpolation geometry can be
//!    *substituted*. It is graded against `iccce-cmm`'s evaluator on every
//!    point before anything is concluded from it
//!    (`pass4/apparatus/harness-nlinear-matches-iccce-cmm`).
//! 2. `Lut16Model` and `MatrixTrc`, called directly for the PCS-side records
//!    and for the ΔE ruler — the same arrangement, and the same declared
//!    weakness, as Pass 3's record 7.
//!
//! ## What lcms2 actually does with a 4-D CLUT — read at the pin, not assumed
//!
//! The expected deviation source was stated in advance as "iccce interpolates
//! n-linear, lcms2 tetrahedral". **For four inputs that is not what lcms2
//! does**, and the tolerances below rest on what the source says:
//!
//! ```c
//! // cmsintrp.c, DefaultInterpolatorsFactory:
//! case 4:  // CMYK lut
//!     if (IsFloat) Interpolation.LerpFloat = Eval4InputsFloat;
//!     else         Interpolation.Lerp16    = Eval4Inputs;
//!
//! // cmsintrp.c, Eval4InputsFloat — "For more that 3 inputs (i.e., CMYK)
//! // evaluate two 3-dimensional interpolations and then linearly interpolate
//! // between them."
//! //   pk = fclamp(Input[0]) * p->Domain[0];  k0 = floor(pk); rest = pk - k0;
//! //   TetrahedralInterpFloat(Input + 1, Tmp1, &p1);   // at node k0
//! //   TetrahedralInterpFloat(Input + 1, Tmp2, &p1);   // at node k0+1
//! //   Output[i] = y0 + (y1 - y0) * rest;
//! ```
//!
//! So lcms2's 4-D scheme is a **hybrid**: *linear* along the first input
//! channel (C), *tetrahedral* (Sakamoto, 6 simplices) in the remaining three
//! (M, Y, K). It is neither pure tetrahedral nor pure n-linear, and it is
//! **not symmetric in the four channels** — swapping the ink order would
//! change the answer. iccce's n-linear is quadrilinear and *is* symmetric.
//!
//! **And the float path does not use the float interpolator.** An `mft2` tag
//! is read into a 16-bit CLUT stage (`cmsStageAllocCLut16bitGranular`), whose
//! float evaluator is `EvaluateCLUTfloatIn16`: it quantises the stage input to
//! `u16`, calls `Interpolation.Lerp16` — i.e. **`Eval4Inputs`, the fixed-point
//! twin** — and converts back. So lcms2's CMYK pipeline in `transicc`'s
//! default float mode carries 16-bit quantisation at the CLUT boundary as well
//! as inside the tabulated tone curves (the Pass 3 finding, README §13.6.1).
//!
//! This is `impl_crosscheck` knowledge — read out of the pinned source at
//! `21c582a`, not out of ICC.1. ICC.1 says **nothing** about CLUT
//! interpolation (corpus ambiguity **A16**, SILENT), which is why iccce's
//! n-linear is a named choice (**NA-006**) rather than a conformance question,
//! and why a disagreement here is a *difference*, not an error on either side.
//!
//! ## Why every intent is compared, and why that is safe here
//!
//! Pass 3 compared media-relative only, because lcms2 forces BPC on at
//! perceptual and saturation **for v4 profiles** (DL-013 / README §12.4,
//! memory M2). The gate is `cmsGetEncodedICCversion(profile) >= 0x4000000`.
//!
//! **Verified for this pair, at the byte level, in this module** — see
//! [`Analysis::version_words`]: `USWebCoatedSWOP.icc` and the system sRGB
//! profile both carry header version `0x02100000` (v2.1.0), below the gate.
//! The forced-BPC branch is therefore unreachable for this pair at every
//! intent, and the comparison at perceptual and saturation is measuring the
//! transform it says it is measuring. The check is *run*, not assumed: the
//! version words are read from the parsed headers and reported on every
//! record, so a future substitution of a v4 profile cannot silently
//! reintroduce the confound.
//!
//! ## The units trap, restated because it is a DIFFERENT trap from Pass 3's
//!
//! | | input | output |
//! |---|---|---|
//! | `iccce transform` | one whitespace-separated **quadruple** per line, floats **0..1** | 6 decimals, **0..1** |
//! | `transicc` (CMYK in) | one component per line, **0..100** | — |
//! | `transicc` (8-bit RGB out) | — | 4 decimals, **0..255** |
//! | `transicc` (`*Lab4` out) | — | 4 decimals, `L*` 0..100, `a*`/`b*` −128..127 |
//!
//! The CMYK 0..100 convention is **not** transicc's `InputRange` (which
//! `ComponentNames` sets to 1 for `cmsSigCmykData`); it comes from
//! `cmspack.c`, where the double formatters scale by `IsInkSpace(fmt) ? 100.0
//! : 1.0`. Measured 2026-08-11 as well as read: `0 1 1 0` gives near-paper
//! white (1 % ink) and `0 100 100 0` gives process red. A harness that fed
//! 0..1 here would silently compare full-ink colours against 1 %-ink colours
//! and would look like a catastrophic colour bug.

use std::path::{Path, PathBuf};

use iccce_cmm::MatrixTrc;
use iccce_cmm::lut_transform::{Lut16Model, PcsKind, PcsValue};
use iccce_cmm::matrix_trc::Intent as CmmIntent;
use iccce_cmm::transform::Chain;
use iccce_color::{D50, Lab, delta_e_2000};
use iccce_profile::Profile;
use iccce_profile::lut::Lut16;
use iccce_profile::num::Signature;
use iccce_profile::tag_types::TagData;

use crate::{
    Bpc, DiffError, Iccce, Intent, Kind, Metric, Oracle, Precalc, Record, Request, Space, Tolerance,
};

// ===========================================================================
// The corpus — category (c), read locally, never committed
// ===========================================================================

/// The source profile: U.S. Web Coated (SWOP) v2, as shipped in the Windows
/// colour directory.
///
/// **Category (c) per `LEGAL.md` §3** — read from the local system, never
/// committed, never a required input. Every record here **skips** when it is
/// absent, so a runner without it reports 3 ("nothing ran"), not 0.
///
/// As read from its bytes on 2026-08-11 (in this module, at run time — see
/// [`Analysis::structure`]): header version `0x02100000` (**v2.1.0**), class
/// `prtr`, `CMYK` → **`Lab `** PCS, 10 tags —
/// `desc cprt wtpt A2B0 A2B2 A2B1 B2A0 B2A1 B2A2 gamt`.
///
/// - **`A2B0` and `A2B2` share one block of tag data** (offset 432, size
///   41478, both). Perceptual and saturation are *literally the same
///   transform* in this file — the Pass 0 finding (README §8.4), and the
///   reason `pass4/swop/perceptual-equals-saturation` can be graded at
///   **exactly** zero.
/// - **`A2B1` is a separate `mft2`** at offset 41912.
/// - Both A2B tags: 4 in, 3 out, **9 CLUT points per axis** (9⁴ = 6561
///   nodes), 256-entry input tables (non-identity, mild ink curves), 2-entry
///   output tables (identity), identity 3×3 matrix.
/// - The `B2A*` tags are `mft1` (`lut8Type`) — **not exercised here**: this
///   pass runs the A2B direction only, and `iccce-cmm` does not evaluate
///   `mft1` yet (assembly stage 3).
pub const SWOP: &str = r"C:\Windows\System32\spool\drivers\color\USWebCoatedSWOP.icc";

/// The destination profile: the Windows system sRGB profile.
///
/// **Category (c) per `LEGAL.md` §3.** The same file Pass 3 used as its
/// *source*; here it is the destination, which exercises the other half of
/// `iccce-cmm::curve` on it (table **inversion** rather than table
/// evaluation).
///
/// **It has no `B2A*` tags** — verified in this module from the parsed tag
/// list ([`Analysis::structure`]): 17 tags, `cprt desc wtpt bkpt rXYZ gXYZ
/// bXYZ dmnd dmdd vued view lumi meas tech rTRC gTRC bTRC`. That matters more
/// than it looks: if it carried a `B2A0`, lcms2 would use the LUT path for the
/// destination while `iccce-cmm::transform::Chain` (whose destination side is
/// matrix/TRC only, stage 3 pending) used the colorant matrix, and every ΔE
/// below would be measuring **two different models**, not two implementations
/// of one. It does not, so both sides evaluate clause 8.10.2 step 4 on this
/// side of the chain.
pub const SRGB: &str = r"C:\Windows\System32\spool\drivers\color\sRGB Color Space Profile.icm";

// ===========================================================================
// The tolerances — every one of them, with the reason it is that number
// ===========================================================================

/// **Record 5/7 — the interpolation-method envelope, in ΔE2000, PCS side.**
///
/// This is the number the whole pass turns on, so its derivation is given
/// before any of the tolerances that use it.
///
/// ## What is being bounded
///
/// iccce evaluates the 9⁴ CLUT by **n-linear** (quadrilinear) interpolation —
/// the **NA-006** named choice inside ICC.1's silence (**A16**). lcms2 at pin
/// `21c582a` evaluates the same table by a **hybrid**: linear along input
/// channel 0, Sakamoto tetrahedral in channels 1–3 (module doc). At a CLUT
/// **node** the two agree exactly — both are interpolants that reproduce the
/// stored sample — so the entire difference lives *between* nodes, and with 9
/// points per axis the cells are 1/8 wide in CLUT-input space, which is coarse
/// enough for the difference to be perceptible. `NUMERIC_CLAIMS.md` NA-006
/// carries a **corpus-derived bound of ~1 ΔE** for trilinear-vs-tetrahedral
/// and states plainly that iccce has **not measured it**. This pass measures
/// it.
///
/// ## Where the number comes from — the table and the two algorithms, not the
/// observed disagreement
///
/// [`SourcePipeline`] reimplements the whole `mft2` A2B path in this harness
/// twice: once with iccce's n-linear CLUT, once with lcms2's geometry as read
/// from `cmsintrp.c`. **Neither run involves lcms2 the program**, and neither
/// involves the observed iccce-vs-lcms2 residual. The maximum ΔE2000 between
/// the two evaluations, over this grid, is the *envelope of the method
/// difference for this fixture*: what two correct implementations of two
/// unspecified-by-ICC.1 interpolation schemes are entitled to differ by here.
///
/// Measured 2026-08-11 over the 341-point grid: **max 0.254 23 ΔE2000, mean
/// 0.038 54** on `A2B1` (media-relative), and **max 1.574 1, mean 0.043 86**
/// on `A2B0` (perceptual and saturation, which share one block of tag data).
/// `pass4_report` §4 prints both, and §7 prints the per-point worst offenders
/// so the claim can be checked rather than believed.
///
/// ## ⚠ `0,254 23` is NOT `0,252 94`, and they are one crate apart
///
/// `pass6.rs`'s [`crate::pass6::COMPILED_DE`] is derived from **`0,252 94`
/// ΔE2000**. The two numbers agree to 0,5 %, are about the same profile, the
/// same intent and the same grid, and live in the same crate — so they are
/// **exactly the pair somebody will quote for one another**, and they are
/// different quantities:
///
/// | | `0,254 23` (here) | `0,252 94` (Pass 4's cross-check) |
/// |---|---|---|
/// | what | the **interpolation-method envelope** | the **observed iccce-vs-lcms2 residual** |
/// | computed from | the 9⁴ table and the two algorithms | two programs' outputs |
/// | is lcms2 the *program* in it? | **no** — no `transicc` runs | **yes** |
/// | what it bounds | what two correct implementations may differ by | what these two implementations *did* differ by |
///
/// The near-agreement is itself a finding — it says the residual we observe is
/// about the size of the method difference alone, i.e. that little else is
/// going on — but it is a *conclusion*, and quoting either number for the other
/// would turn that conclusion into an assumption. **Neither is wrong; the
/// hazard is entirely in the quoting.** `pass6.rs`'s doc carries the mirror of
/// this warning.
///
/// **The two tables are not equally smooth, and the difference is a factor of
/// six.** The perceptual table's worst cell sits at
/// CMYK (0.541, 0.442, 0.744, 0.972) — deep shadow at near-full black — where
/// the CLUT is turning sharply and the two schemes take different routes
/// across the same cell. A tolerance derived from `A2B1` alone would have been
/// wrong by 6× for the very intents Pass 3 never exercised.
///
/// **2.0 ΔE2000 is set**: the larger envelope (1.5741) with ~27 % headroom for
/// the facts that it is a maximum over 341 sampled points of a continuous
/// function that could be larger between them, and that lcms2's *fixed-point*
/// `Eval4Inputs` is emulated here in `f64` geometry only.
///
/// ## ★ Why a tolerance this wide is honest — and what it therefore cannot
/// claim
///
/// 2.0 ΔE2000 is **above** `TOLERANCES.md` §2's (⚠ provisional) perceptibility
/// anchor. A gate at the perceptibility threshold cannot be described as
/// demonstrating agreement, and NA-006 says so in advance: *"a tolerance wide
/// enough to swallow ~1 ΔE cannot also demonstrate agreement."* So this record
/// is doing exactly one job — **detecting a structural error**: a wrong CLUT
/// index order (A20's first-channel-slowest, which would be catastrophic), a
/// wrong Lab decode (legacy vs v4, ≈0.39 `L*` at white but far larger in the
/// `a*`/`b*` scale factor), a transposed ink, a missing input table. Those are
/// tens of ΔE, not tenths.
///
/// **The claim of agreement is made by [`DE_PCS_EMULATED`] instead**, which
/// removes the method difference by construction and gates the remainder at
/// two orders of magnitude tighter. Splitting the two is the whole design:
/// one number that is wide and structural, one that is tight and arithmetic,
/// and neither pretending to be the other.
pub const DE_PCS_CROSSCHECK: Tolerance = Tolerance::new(
    2.0,
    "the CLUT INTERPOLATION-METHOD envelope for this fixture, computed from the 9^4 table and \
     the two algorithms ALONE (no lcms2 run, no observed residual): iccce's n-linear (NA-006, \
     ICC.1 is SILENT on interpolation - A16) against lcms2's hybrid linear-in-C x tetrahedral-in-MYK \
     (cmsintrp.c Eval4Inputs at pin 21c582a) = max 0.25423 dE00 on A2B1, 1.5741 on A2B0 over this \
     grid; 2.0 is the larger with ~27% headroom for between-point maxima and for lcms2's \
     fixed-point arithmetic being emulated in f64 geometry. ABOVE the (PROVISIONAL) 1.0 dE00 \
     perceptibility anchor, therefore NOT a demonstration of agreement - it detects STRUCTURAL \
     error only (wrong CLUT index order, wrong Lab decode, transposed ink). The agreement claim \
     is DE_PCS_EMULATED's, 100x tighter",
);

/// **Record 7 — the same comparison with lcms2's own interpolation geometry
/// emulated. This is the record that actually claims agreement.**
///
/// Metric: max ΔE2000 between (a) the `mft2` pipeline evaluated in this
/// harness with **lcms2's** CLUT geometry in `f64`, and (b) `transicc
/// -iSWOP -o*Lab4 -t<n> -c0`.
///
/// ## What is left once the method difference is removed
///
/// Everything that is *not* the interpolation scheme:
///
/// | term | size | source |
/// |---|---|---|
/// | tabulated input curves quantised to 1/65535, in *and* out | ≤1.53×10⁻⁵ per channel in CLUT-input | `cmsgamma.c` `cmsEvalToneCurveFloat`, `nSegments == 0` branch (README §13.6.1) |
/// | CLUT stage input quantised to `u16` | ≤7.63×10⁻⁶ per channel | `cmslut.c` `EvaluateCLUTfloatIn16` → `FromFloatTo16` |
/// | `Eval4Inputs` runs in **s15.16 fixed point**, not `f64` | ~1 lsb of 1/65535 on the output | `cmsintrp.c` `EVAL_FNS(4,3)` / `TetrahedralInterp16` |
/// | CLUT stage output back to float, then 2-entry output curves | ≤7.63×10⁻⁶ | same |
/// | `transicc` prints `L*`/`a*`/`b*` to 4 decimals | ±5×10⁻⁵ each | README §9 |
///
/// A CLUT-output error of 1 lsb (1/65535) becomes, through the **legacy** Lab
/// decode this tag type mandates, `65535/652.8 × 1/65535` ≈ **1.53×10⁻³ in
/// `L*`** and `65535/256 × 1/65535` ≈ **3.9×10⁻³ in `a*`/`b*`** — the `a*`/`b*`
/// scale is 256 codes per unit, so a single 16-bit lsb is *not* negligible
/// there. Summing a few such terms with the print floor gives a ΔE00 budget of
/// order 10⁻². **2×10⁻² is set**, and the observed maxima were **4.5931×10⁻³**
/// (mean 1.2988×10⁻³) at media-relative and **4.8154×10⁻³** (mean
/// 1.1091×10⁻³) at perceptual/saturation — i.e. the emulation shrinks the
/// disagreement by **55×** and **326×** respectively.
///
/// **This is where "iccce and lcms2 agree" is earned**: with lcms2's own
/// interpolation geometry substituted for iccce's, the two implementations of
/// the `lut16` pipeline — input curves, index order, CLUT, output curves,
/// legacy Lab decode, PCS handling — agree to the arithmetic of the oracle's
/// own quantisation. What it does **not** establish is that either one is
/// *right*; both could read clause 10.10 the same way and both be wrong
/// (`TOLERANCES.md` §1). That is what the ground-truth row §3.4 still lacks.
pub const DE_PCS_EMULATED: Tolerance = Tolerance::new(
    2e-2,
    "with lcms2's OWN CLUT geometry emulated in f64, what remains is the oracle's quantisation: \
     tabulated input curves rounded to 1/65535 in and out (cmsEvalToneCurveFloat), the CLUT stage \
     input rounded to u16 (EvaluateCLUTfloatIn16) and evaluated in s15.16 fixed point, and \
     transicc's 4-decimal Lab print. One 16-bit lsb of CLUT output is 1.53e-3 in L* and 3.9e-3 in \
     a*/b* under the LEGACY decode this tag type mandates (652.8 and 256 codes per unit), so a \
     few such terms plus the print floor is a ~1e-2 dE00 budget; 2e-2 is that. \
     THIS is the record that claims agreement; DE_PCS_CROSSCHECK is wide by construction",
);

/// **Record 6 — the interpolation-free control: the 16 CLUT-node corners.**
///
/// Metric: max ΔE2000 between iccce's A2Bx PCS output and `transicc`'s, over
/// the **16 hypercube corners only**.
///
/// ## Why these 16 points are special, and why the control is necessary
///
/// The corners are the only grid points where **both** implementations
/// evaluate the CLUT at an exact node: each `mft2` input table's first and
/// last entries are `0x0000` and `0xFFFF` exactly (verified from the bytes),
/// so a device 0 or 1 maps to CLUT-input 0 or 1, which is node 0 or node 8.
/// At a node every interpolation weight is 0 or 1 for n-linear, and the
/// Sakamoto simplex degenerates to its `c0` term for tetrahedral. **The method
/// difference is identically zero there**, by construction and not by
/// tolerance.
///
/// So this record measures the *rest* of the pipeline with the dominant term
/// switched off: index order, table lookup, the legacy Lab decode, the PCS
/// encoding. It is the sensitivity control that makes [`DE_PCS_CROSSCHECK`]'s
/// width defensible — without it, a 2.0 ΔE2000 gate on the PCS side could hide
/// a genuine error of 1.9 ΔE and nobody would know.
///
/// ## Where 1×10⁻³ comes from — and why it is far tighter than the budget
/// [`DE_PCS_EMULATED`] carries
///
/// At a node, lcms2's quantisation terms **vanish rather than accumulate**,
/// and that is worth stating because it was not obvious in advance: the CLUT
/// input is an exact `u16` (a table endpoint), so `FromFloatTo16` is lossless;
/// the interpolated value *is* the stored `u16`, so `Eval4Inputs`'s fixed-point
/// arithmetic has nothing to round; and the 2-entry output tables are the
/// identity. The only term left is **`transicc`'s 4-decimal Lab print**,
/// ±5×10⁻⁵ per component ⇒ a ΔE00 floor of ≈1×10⁻⁴.
///
/// **Observed 6.6558×10⁻⁵ at perceptual/saturation and 5.9131×10⁻⁵ at
/// media-relative** — exactly that floor, and 70× below the 4.6×10⁻³ the same
/// comparison shows between nodes. **1×10⁻³ is 10× the print floor.**
///
/// **What it would catch that nothing else here would:** the v2/v4 Lab
/// encoding error (`ARCHITECTURE.md` §2's named hazard, ≈0.39 `L*` at white
/// and far worse in `a*`/`b*`), a swapped ink order, an off-by-one in the CLUT
/// node index. All are ≥1000× this bound at a corner, where the paper-white and
/// full-ink extremes live.
pub const DE_PCS_CORNERS: Tolerance = Tolerance::new(
    1e-3,
    "the 16 hypercube corners are exact CLUT NODES (each mft2 input table starts at 0x0000 and \
     ends at 0xFFFF, verified from the bytes), where n-linear and tetrahedral agree IDENTICALLY \
     AND lcms2's quantisation terms vanish rather than accumulate (the CLUT input is an exact \
     u16, the interpolated value IS the stored u16, the output tables are identity). What is \
     left is transicc's 4-decimal Lab print: a dE00 floor of ~1e-4. 1e-3 is 10x that. \
     It is what makes DE_PCS_CROSSCHECK's 2.0 defensible: without a node-only control, a wide \
     gate could hide a real 1.9 dE00 error",
);

/// **Records 1 and 3 — end-to-end CMYK → sRGB device agreement.**
///
/// Metric: max |Δ| over every component of every grid point, **normalised
/// device units (0..1)**, lcms2's output clamped into `[0,1]` first (the
/// unclamped maximum and the excursion count are reported separately, as in
/// Pass 3 — see README §13.4 and corpus M3).
///
/// ## Where 2×10⁻² comes from
///
/// A closed-form union bound was tried first and thrown away, which is worth
/// recording because the thrown-away version is the one a reader would expect:
/// the PCS method difference reaches 1.57 ΔE2000, `dY/dL* = 3/116` puts that
/// at ≤4×10⁻² in PCS `Y`, `‖M_dst⁻¹‖∞ ≈ 3.1` for the sRGB colorants, and the
/// sRGB inverse TRC's slope is ≤12.92 (its linear segment near black — note
/// this is **bounded**, unlike Pass 3's pure-gamma destination, which had no
/// finite uniform bound at all). The product is ≈1.6 device units, which is
/// larger than the whole range and therefore useless: it assumes the worst PCS
/// error, in the worst channel, at the darkest point, all at once.
///
/// **The number is set from the propagated envelope evaluated point by point
/// instead.** `pass4_report` §4 pushes the *method difference* — n-linear
/// versus lcms2's geometry, no lcms2 output involved — through the **actual**
/// destination model at every grid point and takes the maximum:
/// **1.0751×10⁻²** (perceptual/saturation) and **2.9012×10⁻³**
/// (media-relative), in normalised device units.
///
/// **2×10⁻² is the larger of those with ~86 % headroom.** The observed
/// iccce-vs-lcms2 maximum is **1.0816×10⁻²** — 0.6 % *above* the propagated
/// envelope, which is the right sign and the right size: the envelope models
/// the interpolation geometry only, and lcms2's 16-bit quantisation is
/// genuinely on top of it.
///
/// **Not perceptual, and grid-dependent by construction** — the same two
/// caveats Pass 3's device tolerance carries, for the same reasons. A grid
/// with more deep-shadow CMYK would re-derive it.
pub const DEVICE_CROSSCHECK: Tolerance = Tolerance::new(
    2e-2,
    "the CLUT interpolation-method envelope propagated through the ACTUAL destination model \
     point by point (pass4_report section 4), which involves NO lcms2 output: 1.0751e-2 \
     normalised device units at perceptual/saturation, 2.9012e-3 at media-relative. 2e-2 is the \
     larger with ~86% headroom, sized to admit lcms2's 16-bit quantisation on top of the \
     geometry. A closed-form union bound was tried and discarded as useless (1.6 device units, \
     wider than the range). Bounded at all only because the sRGB inverse TRC's slope is <=12.92 \
     near black - a pure-gamma destination has no finite bound, cf. Pass 3. \
     GRID-DEPENDENT BY CONSTRUCTION; arithmetic-agreement, NOT perceptual",
);

/// **Record 3 — end-to-end agreement expressed perceptually.**
///
/// Metric: max ΔE2000 between iccce's and lcms2's destination device RGB, both
/// carried into D50 CIELAB through iccce's sRGB matrix/TRC model.
///
/// ## Where 2.0 ΔE2000 comes from
///
/// The PCS-side envelope carried through the destination leg, which is **not**
/// lossless: PCS Lab → XYZ → the F.8–F.16 `[0,1]` clamp → TRC⁻¹, and SWOP's
/// gamut is **not** contained in sRGB's. Unlike Pass 3 — where sRGB ⊂ Adobe RGB
/// made the clip path reachable only by 1-lsb white-point excursions — **this
/// pair produces genuine out-of-gamut clipping** on a large fraction of the
/// grid, which both implementations perform but not necessarily identically.
///
/// The measured decomposition (`pass4_report` §4): the method envelope alone,
/// propagated end-to-end and expressed in ΔE2000, is **1.6639**
/// (perceptual/saturation) and **0.254 23** (media-relative); the observed
/// iccce-vs-lcms2 maxima are **1.6590** and **0.252 94**. Those agree to
/// **0.3 %** and **0.5 %** — the same shape of result as Pass 3's white-clamp
/// prediction: the disagreement is *accounted for* by a named non-error
/// mechanism rather than merely being small.
///
/// **2.0 is the larger envelope with ~20 % headroom**, and it inherits every
/// caveat [`DE_PCS_CROSSCHECK`] carries: it is **above** `TOLERANCES.md` §2's
/// ⚠ provisional 1.0 ΔE2000 perceptibility anchor, so **it does not
/// demonstrate perceptual agreement and must never be quoted as if it did.**
/// It detects structural error. The agreement claim on this pass belongs to
/// [`DE_PCS_EMULATED`] and [`DE_PCS_CORNERS`], which are 100× and 2000×
/// tighter and have the method difference switched off.
///
/// **At the icc-absolute intent this tolerance is NOT used** — see
/// [`ABSOLUTE_REPORTED`].
pub const DE_CROSSCHECK: Tolerance = Tolerance::new(
    2.0,
    "the CLUT interpolation-method envelope propagated end-to-end through the destination model \
     and expressed in dE2000 (no lcms2 output enters the envelope): 1.6639 at \
     perceptual/saturation, 0.25423 at media-relative, against observed 1.6590 and 0.25294 - \
     agreeing to 0.3% and 0.5%, so the disagreement is ACCOUNTED FOR by a named non-error \
     mechanism (NA-006) rather than merely small. 2.0 is the larger envelope with ~20% headroom. \
     ABOVE the (PROVISIONAL) 1.0 dE00 perceptibility anchor: this record does NOT demonstrate \
     perceptual agreement and must never be quoted as if it did; it detects structural error. \
     The agreement claim is DE_PCS_EMULATED's and DE_PCS_CORNERS's",
);

/// **The absolute-intent record that IS graded.**
///
/// Metric: max ΔE2000 between lcms2's ICC-absolute output and a prediction of
/// it that replaces exactly two of iccce's choices with lcms2's — the CLUT
/// geometry, and the **destination media white**.
///
/// ## The divergence this exists to explain
///
/// At ICC-absolute, iccce and lcms2 differ by **up to 11.2 ΔE2000** (mean
/// 4.67) on this pair — two orders of magnitude more than at any other intent,
/// and far above anything the interpolation-method envelope (0.25 ΔE00 for
/// `A2B1`) could account for. That is a divergence, and leaving it inside a
/// widened tolerance would be exactly the failure mode this whole role exists
/// to prevent.
///
/// **The mechanism, read out of the pinned source and then measured:**
///
/// ```c
/// // cmsio1.c, _cmsReadMediaWhitePoint:
/// //   ... reads cmsSigMediaWhitePointTag ...
/// //   // V2 display profiles should give D50
/// //   if (cmsGetEncodedICCversion(hProfile) < 0x4000000) {
/// //       if (cmsGetDeviceClass(hProfile) == cmsSigDisplayClass) {
/// //           *Dest = *cmsD50_XYZ(); return TRUE;
/// //       }
/// //   }
/// ```
///
/// The destination here is a **v2 `mntr`** profile, and — this is the part
/// that makes the effect large — **its `wtpt` tag holds D65**
/// `(0.950 455, 1.0, 1.089 050)`, not D50, which is a common v2-era encoding
/// (the colorants are D50-adapted while `wtpt` records the unadapted white).
/// So at `AdaptationState == 1.0` lcms2's `ComputeAbsoluteIntent` scales the
/// PCS by `wtpt_src / D50`, while iccce (NA-007: `wtpt` **as stored**) scales
/// by `wtpt_src / wtpt_dst`. The two differ by `D65/D50` per component —
/// (0.9858, 1.0, 1.3202) — which is a ~32 % error in `Z`. That is the 11 ΔE.
///
/// ## Which one is right? — NOT SETTLED HERE, and that is the finding
///
/// `NUMERIC_CLAIMS.md` **NA-007** registers iccce's as-stored reading, and
/// corpus **A4b** — *what a v2 profile's `wtpt` means* — is **UNVERIFIED**,
/// because ICC.1:2001-04 has not been obtained. lcms2's substitution is
/// justified in its source by a comment, not by a clause. **A dispatch to
/// `icc-spec-librarian` is owed** (README §14.6 states the question). Until it
/// is settled this record grades the *model*, not the *policy*: it asserts
/// that iccce's absolute path differs from lcms2's **only** in the white point
/// it reads, which is a falsifiable claim and the one worth pinning.
///
/// ## Where 5×10⁻² ΔE2000 comes from — and this is the weakest-justified
/// tolerance in the pass
///
/// With the geometry emulated too, what remains is the oracle's quantisation
/// (≤4.6×10⁻³ ΔE00 in the PCS, measured by [`DE_PCS_EMULATED`] at
/// media-relative) carried through the destination leg, plus `transicc`'s
/// 4-decimal RGB print. **Observed max 2.1677×10⁻², mean 3.4034×10⁻³** — a
/// **517×** collapse from the 11.217 ΔE00 the two implementations differ by
/// unmodelled, and a **1372×** collapse in the mean.
///
/// **5×10⁻² is ~2.3× the observed maximum**, and unlike the other tolerances
/// here it is a *bracket* rather than a derivation: no closed form was
/// computed for how the destination leg amplifies a PCS residual at this
/// intent, where much of the grid lands in deep shadow and against the gamut
/// clamp. Stated plainly so nobody quotes it as though it were derived. What
/// makes it usable anyway is the ratio it sits between: **500× below the
/// divergence it must detect** and **2× above the quantisation floor it must
/// not trip on**. A mechanism that was only approximately right — say the
/// right white point but the wrong direction, or D50 substituted on the wrong
/// profile — would land orders of magnitude outside it.
pub const WP_POLICY_EMULATED: Tolerance = Tolerance::new(
    5e-2,
    "with lcms2's CLUT geometry AND its destination media white (D50, substituted by \
     _cmsReadMediaWhitePoint for a v2 DISPLAY profile) both adopted, the 11.2 dE00 \
     absolute-intent divergence collapses to 2.1677e-2 max / 3.4034e-3 mean - a 517x collapse. \
     5e-2 is ~2.3x that observed maximum and is a BRACKET, not a derivation: no closed form was \
     computed for how the destination leg amplifies a PCS residual in deep shadow at this intent, \
     and saying so is the point. It sits 500x below the divergence it must detect and 2x above \
     the quantisation floor it must not trip on. NOT a claim that lcms2's white-point policy is \
     correct: corpus A4b (what a v2 wtpt means) is UNSOURCED and the dispatch is owed",
);

/// **The absolute-intent raw comparisons — REPORTED, NOT GRADED, and why.**
///
/// A gate here would be grading a **policy divergence whose mechanism is
/// established and whose specification question is unsourced** (see
/// [`WP_POLICY_EMULATED`]). Two dishonest options were available and are
/// recorded as rejected:
///
/// - **Widen the tolerance to ~15 ΔE00 so it passes.** That is a number chosen
///   because it passed, and 15 ΔE00 is a *different colour*. It would also
///   silently absorb any future arithmetic error in the absolute path.
/// - **Let it fail permanently.** A red line that never changes is a line
///   people stop reading, and it would report "iccce disagrees with lcms2" as
///   though the disagreement were unexplained, which it is not.
///
/// So the raw numbers are **recorded with an infinite tolerance** — the same
/// device already used for the means — and the **gate at this intent is
/// [`WP_POLICY_EMULATED`]**, which is tight and *would* catch a regression in
/// the absolute path. The moment A4b is settled, one of the two
/// implementations acquires a defect and this becomes a graded row again.
pub const ABSOLUTE_REPORTED: Tolerance = Tolerance::new(
    f64::INFINITY,
    "REPORTED, NOT GRADED at icc-absolute. iccce and lcms2 use DIFFERENT DESTINATION MEDIA WHITES \
     here - iccce the wtpt tag as stored (D65 in this v2 mntr file, NA-007), lcms2 D50 by its \
     v2-display substitution rule (cmsio1.c) - so this number is a POLICY divergence, ~11 dE00, \
     whose mechanism is established by the white-point-policy record and whose spec question \
     (corpus A4b, v2 wtpt semantics) is UNSOURCED. Grading it would mean either a ~15 dE00 \
     tolerance chosen because it passed, or a permanent red that says nothing new. The GATE at \
     this intent is pass4/.../white-point-policy-emulated",
);

/// **Record 8 — perceptual and saturation are the same transform, exactly.**
///
/// Metric: max |Δ| in normalised device units between the perceptual and the
/// saturation output, **on both sides**, reduced to one number.
///
/// ## Why the tolerance is exactly zero and that is not a stunt
///
/// `A2B0` and `A2B2` in this file are two tag-table entries pointing at **one
/// shared block of tag data** — same offset, same size (README §8.4, verified
/// again here from the parsed tag table). Perceptual and saturation are
/// therefore not merely similar: they are the *same bytes*, evaluated by the
/// same code, from the same input. Any difference at all is a defect in the
/// 8.10.2 tag-selection fallback — picking `A2B0` for one and, say, falling
/// through to the matrix/TRC path for the other — and there is no arithmetic
/// that could produce a small one.
///
/// **`0.0` with `<=` is therefore the only honest bound.** A "small" tolerance
/// here would admit exactly the class of bug the record exists to catch.
pub const INTENT_SHARED_TAG: Tolerance = Tolerance::new(
    0.0,
    "A2B0 and A2B2 in this file are ONE shared block of tag data (same offset, same size), so \
     perceptual and saturation are the same bytes through the same code: any difference is a \
     tag-selection defect in the 8.10.2 fallback, and no arithmetic could make it small. \
     Exact equality is the only honest bound; a small epsilon would admit the bug",
);

// ===========================================================================
// The grid
// ===========================================================================

/// The input grid: **deterministic CMYK quadruples in `[0,1]`**, assembled so
/// a reader can tell what is covered without running it.
///
/// | block | count | why it is there |
/// |---|---|---|
/// | hypercube corners | 16 | paper (0,0,0,0), 100 % K, the four single inks, 0/100/100/0 process red, and 400 % total ink. **Every one is an exact CLUT node**, which is what makes the corner control possible |
/// | K ramp `(0,0,0,k/8)` | 9 | the black channel alone — where a CMYK profile's separation behaviour lives |
/// | CMY composite neutral `(v,v,v,0)` | 9 | the *other* neutral axis. A wrong ink order shows up here and nowhere else |
/// | rich neutral `(v,v,v,v)` | 9 | all four channels moving together, into the deepest shadow the profile can reach |
/// | 4-D lattice on `{0, ⅓, ⅔, 1}` | 256 | systematic interior coverage. ⅓ and ⅔ are deliberately **not** CLUT nodes (the nodes are multiples of ⅛ in CLUT-input space, and the input tables are non-identity anyway), so every one of these interpolates |
/// | pseudo-random interior | 64 | fixed-seed LCG (MMIX constants) into `[0.02, 0.98]⁴`. Systematic grids can sit on structure; these deliberately do not |
///
/// Duplicates are removed by exact bit pattern.
///
/// **Deterministic by construction** — no `rand`, no clock, no hash seed. Two
/// runs on two machines compare the same colours or the comparison between
/// their reports means nothing. Pinned by unit tests, including the count.
///
/// ## What this grid does NOT cover
///
/// - **No total-ink-limit realism.** Real SWOP separations rarely exceed
///   ~300 % total ink; this grid goes to 400 %. That is deliberate — the CLUT
///   is defined there and disagreements are larger — but it means the *mean*
///   over this grid is not the mean over printable colour.
/// - **Nothing below 1/8 in a single channel except exact zero**, except
///   through the random block.
/// - **One profile pair, one direction.** The B2A direction (`mft1` in this
///   file) is not evaluated by `iccce-cmm` at all yet.
#[must_use]
pub fn grid() -> Vec<[f64; 4]> {
    let mut out: Vec<[f64; 4]> = Vec::new();
    let push = |t: [f64; 4], out: &mut Vec<[f64; 4]>| {
        let key = |v: f64| v.to_bits();
        if !out.iter().any(|e| (0..4).all(|i| key(e[i]) == key(t[i]))) {
            out.push(t);
        }
    };

    // 1. The sixteen hypercube corners, first and explicitly — they are the
    //    interpolation-free control block and `corner_indices()` depends on
    //    them being the first sixteen entries.
    for c in [0.0, 1.0] {
        for m in [0.0, 1.0] {
            for y in [0.0, 1.0] {
                for k in [0.0, 1.0] {
                    push([c, m, y, k], &mut out);
                }
            }
        }
    }

    // 2. Three neutral axes: K alone, CMY together, and all four together.
    for step in 0..=8 {
        let v = f64::from(step) / 8.0;
        push([0.0, 0.0, 0.0, v], &mut out);
        push([v, v, v, 0.0], &mut out);
        push([v, v, v, v], &mut out);
    }

    // 3. A 4-D lattice on {0, 1/3, 2/3, 1}.
    let axis = [0.0, 1.0 / 3.0, 2.0 / 3.0, 1.0];
    for &c in &axis {
        for &m in &axis {
            for &y in &axis {
                for &k in &axis {
                    push([c, m, y, k], &mut out);
                }
            }
        }
    }

    // 4. Pseudo-random interior points. Deterministic LCG (MMIX constants),
    //    fixed seed, mapped into [0.02, 0.98] so this block cannot re-cover
    //    the corners it exists to complement.
    let mut x: u64 = 0x1CCC_E000_0004_0001;
    let mut next = || -> f64 {
        x = x
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        #[allow(clippy::cast_precision_loss)] // exactly 53 bits: lossless
        let u = (x >> 11) as f64 / ((1u64 << 53) as f64);
        0.02 + u * 0.96
    };
    for _ in 0..64 {
        let t = [next(), next(), next(), next()];
        push(t, &mut out);
    }

    out
}

/// The indices of the interpolation-free control block within [`grid`] — the
/// sixteen hypercube corners, which `grid()` emits first.
///
/// Kept as a function rather than a constant range so that a future change to
/// `grid()`'s ordering is caught by [`tests::corner_indices_really_are_corners`]
/// instead of silently re-pointing the control at interior points.
#[must_use]
pub fn corner_indices(g: &[[f64; 4]]) -> Vec<usize> {
    g.iter()
        .enumerate()
        .filter(|(_, t)| t.iter().all(|&v| v == 0.0 || v == 1.0))
        .map(|(i, _)| i)
        .collect()
}

/// The five named CMYK points `README.md` §8.4 already carries `transicc`
/// numbers for, at all four intents. Reported point-by-point in
/// `pass4_report` so the new run can be compared against the Pass 0 record by
/// eye — a cheap regression on the oracle itself.
pub const NAMED_POINTS: [([f64; 4], &str); 5] = [
    ([0.0, 1.0, 1.0, 0.0], "0/100/100/0 process red (README 8.4)"),
    ([1.0, 0.0, 0.0, 0.0], "100% C single ink"),
    ([0.0, 1.0, 0.0, 0.0], "100% M single ink"),
    ([0.0, 0.0, 1.0, 0.0], "100% Y single ink"),
    ([0.0, 0.0, 0.0, 1.0], "100% K single ink"),
];

// ===========================================================================
// The mft2 pipeline, reimplemented in the harness — twice
// ===========================================================================

/// Which CLUT interpolation geometry to use.
///
/// **Both are implemented here, in the harness, in `f64`.** Nothing in
/// `crates/` gains a tetrahedral evaluator from this file: `iccce-cmm`'s
/// interpolation remains n-linear (NA-006), and lcms2's geometry exists here
/// only as a *model of the oracle's arithmetic*, the same way README §13.6.1
/// modelled `cmsEvalToneCurveFloat` to explain Pass 3's residual.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scheme {
    /// iccce's choice: n-linear (quadrilinear) over all four inputs. This
    /// implementation must agree with `iccce_cmm::clut::Clut::eval` to `f64`
    /// noise, which [`SourcePipeline::self_check`] asserts before any result
    /// from this module is believed.
    NLinear,
    /// lcms2's `Eval4Inputs` geometry as read at pin `21c582a`: **linear along
    /// input channel 0**, **Sakamoto tetrahedral in channels 1–3**.
    ///
    /// Emulated in `f64`. lcms2's actual float path runs the *fixed-point*
    /// twin (`EvaluateCLUTfloatIn16` → `Lerp16`), so this models the
    /// **geometry** and not the quantisation; the quantisation is what
    /// [`DE_PCS_EMULATED`] budgets for.
    Lcms2Hybrid,
}

/// The `lut16Type` device→PCS pipeline, reimplemented in the harness so that
/// the CLUT interpolation geometry can be **substituted**.
///
/// ## Why this exists at all, given `iccce_cmm::lut_transform::Lut16Model`
///
/// The experiment that makes Pass 4's tolerances defensible needs the *same*
/// pipeline evaluated two ways, differing in exactly one component. That
/// substitution cannot be made inside `iccce-cmm` — this role does not edit
/// `crates/`, and putting a tetrahedral evaluator there would change the
/// subject of the test. So the pipeline is rebuilt here from the parsed tag,
/// and [`SourcePipeline::self_check`] holds the `NLinear` arm against
/// `Lut16Model` on every grid point before any conclusion is drawn from it.
/// **If that self-check ever fails, every number this module prints is
/// void**, and it is graded as its own record rather than left as an
/// assumption.
#[derive(Debug, Clone)]
pub struct SourcePipeline {
    /// Per-channel input tables, normalised (÷65535).
    input_tables: Vec<Vec<f64>>,
    /// CLUT nodes per axis (9 for both SWOP A2B tags).
    points: usize,
    inputs: usize,
    outputs: usize,
    /// Normalised CLUT samples, first channel slowest (clause 10.10, A20).
    clut: Vec<f64>,
    output_tables: Vec<Vec<f64>>,
}

impl SourcePipeline {
    /// Build from a decoded `mft2` tag. The 3×3 matrix is **not** applied:
    /// the A2B input side is device, never PCSXYZ (A21), exactly as
    /// `Lut16Model::from_lut16(.., false, ..)` does.
    #[must_use]
    pub fn from_lut16(lut: &Lut16) -> SourcePipeline {
        let inputs = usize::from(lut.input_chan);
        let outputs = usize::from(lut.output_chan);
        let ie = usize::from(lut.input_ent);
        let oe = usize::from(lut.output_ent);
        let norm = |v: u16| f64::from(v) / 65535.0;
        SourcePipeline {
            input_tables: (0..inputs)
                .map(|c| lut.input_tables[c * ie..(c + 1) * ie].iter().copied().map(norm).collect())
                .collect(),
            points: usize::from(lut.clut_points),
            inputs,
            outputs,
            clut: lut.clut.iter().copied().map(norm).collect(),
            output_tables: (0..outputs)
                .map(|c| lut.output_tables[c * oe..(c + 1) * oe].iter().copied().map(norm).collect())
                .collect(),
        }
    }

    /// Evaluate device → **legacy-decoded** D50 CIELAB.
    ///
    /// The decode is the legacy 16-bit form (`L* = code/652.8`,
    /// `a*/b* = code/256 − 128`) because that is this tag type's rule —
    /// ICC.1:2022 6.3.4.2 NOTE 3 and 10.10, corroborated by measured lcms2
    /// behaviour **M1** (it keys on the tag type, not the profile version).
    /// The continuous `f64`-code form is used, matching
    /// `lut_transform.rs`'s `decode_lab_*_f`: rounding the interpolated table
    /// output to an integer code before decoding would quantise the pipeline
    /// to 16 bits mid-stream, which is lcms2's behaviour, not the model's.
    #[must_use]
    pub fn eval_lab(&self, device: &[f64], scheme: Scheme) -> Lab {
        let v: Vec<f64> = device
            .iter()
            .zip(&self.input_tables)
            .map(|(&x, t)| interp_table(t, x))
            .collect();
        let mut clut_out = vec![0.0f64; self.outputs];
        match scheme {
            Scheme::NLinear => self.eval_nlinear(&v, &mut clut_out),
            Scheme::Lcms2Hybrid => self.eval_lcms2(&v, &mut clut_out),
        }
        let o: Vec<f64> = clut_out
            .iter()
            .zip(&self.output_tables)
            .map(|(&x, t)| interp_table(t, x))
            .collect();
        Lab {
            l: o[0] * 65535.0 / 652.8,
            a: o[1] * 65535.0 / 256.0 - 128.0,
            b: o[2] * 65535.0 / 256.0 - 128.0,
        }
    }

    /// Flat index of a node, first input channel slowest (A20).
    fn node(&self, idx: &[usize]) -> usize {
        let mut flat = 0usize;
        for &i in idx {
            flat = flat * self.points + i;
        }
        flat * self.outputs
    }

    /// n-linear: the 2ⁿ-corner convex combination. Mirrors
    /// `iccce_cmm::clut::Clut::eval`, including its clamped-index-then-
    /// fraction pairing.
    fn eval_nlinear(&self, input: &[f64], out: &mut [f64]) {
        let d = self.inputs;
        let mut base = vec![0usize; d];
        let mut frac = vec![0.0f64; d];
        for dim in 0..d {
            let (i, f) = cell(input[dim], self.points);
            base[dim] = i;
            frac[dim] = f;
        }
        out.fill(0.0);
        let mut idx = vec![0usize; d];
        for corner in 0..(1usize << d) {
            let mut w = 1.0f64;
            for dim in 0..d {
                let hi = (corner >> (d - 1 - dim)) & 1 == 1;
                idx[dim] = base[dim] + usize::from(hi);
                w *= if hi { frac[dim] } else { 1.0 - frac[dim] };
            }
            if w == 0.0 {
                continue;
            }
            let b = self.node(&idx);
            for (o, slot) in out.iter_mut().enumerate() {
                *slot += w * self.clut[b + o];
            }
        }
    }

    /// lcms2's `Eval4Inputs` geometry: linear along channel 0 between two
    /// 3-D tetrahedral interpolations of channels 1–3.
    ///
    /// Transcribed from `cmsintrp.c` at pin `21c582a` — `Eval4InputsFloat` for
    /// the outer structure and `TetrahedralInterpFloat` (Sakamoto, six
    /// simplices selected by the ordering of the three fractions) for the
    /// inner. The `>= 1.0` upper-node collapse at the top of each axis is
    /// reproduced; it is what makes the scheme exact at the far corner.
    ///
    /// Generalised to `inputs != 4` by recursion on the leading channels, so
    /// the same code serves a hypothetical 5-ink table; for 3 inputs it is
    /// pure tetrahedral, which is what lcms2 uses there.
    fn eval_lcms2(&self, input: &[f64], out: &mut [f64]) {
        self.eval_lcms2_rec(input, 0, 0, out);
    }

    fn eval_lcms2_rec(&self, input: &[f64], dim: usize, base: usize, out: &mut [f64]) {
        let remaining = self.inputs - dim;
        if remaining == 3 {
            self.tetrahedral(&input[dim..], base, out);
            return;
        }
        // Linear in this channel between two sub-hypercubes, exactly as
        // Eval4InputsFloat does for channel 0.
        let (k0, rest) = cell_lcms2(input[dim], self.points);
        // The stride of one step along `dim`: outputs × points^(trailing
        // dimensions). Built by repeated multiplication rather than `pow`
        // with a cast, because the exponent is a channel count (≤15 by the
        // format's own bound) and a `usize → u32` cast here would be a
        // truncation clippy is right to flag even though it cannot fire.
        let mut stride = self.outputs;
        for _ in 0..(self.inputs - dim - 1) {
            stride *= self.points;
        }
        let lo = base + stride * k0;
        let hi = if fclamp(input[dim]) >= 1.0 { lo } else { lo + stride };
        let mut a = vec![0.0f64; self.outputs];
        let mut b = vec![0.0f64; self.outputs];
        self.eval_lcms2_rec(input, dim + 1, lo, &mut a);
        self.eval_lcms2_rec(input, dim + 1, hi, &mut b);
        for o in 0..self.outputs {
            out[o] = a[o] + (b[o] - a[o]) * rest;
        }
    }

    /// Sakamoto tetrahedral interpolation over three channels of a sub-cube
    /// rooted at flat offset `base`.
    ///
    /// `x` is the slowest of the three (stride `points²`), `z` the fastest
    /// (stride 1) — the same order `TetrahedralInterpFloat` uses via
    /// `opta[2] / opta[1] / opta[0]`.
    fn tetrahedral(&self, input: &[f64], base: usize, out: &mut [f64]) {
        let n = self.outputs;
        let sx = n * self.points * self.points;
        let sy = n * self.points;
        let sz = n;
        let (x0, rx) = cell_lcms2(input[0], self.points);
        let (y0, ry) = cell_lcms2(input[1], self.points);
        let (z0, rz) = cell_lcms2(input[2], self.points);
        let big_x0 = sx * x0;
        let big_x1 = big_x0 + if fclamp(input[0]) >= 1.0 { 0 } else { sx };
        let big_y0 = sy * y0;
        let big_y1 = big_y0 + if fclamp(input[1]) >= 1.0 { 0 } else { sy };
        let big_z0 = sz * z0;
        let big_z1 = big_z0 + if fclamp(input[2]) >= 1.0 { 0 } else { sz };

        // `out` is indexed by the same channel counter the CLUT is offset by,
        // so an iterator over `out` alone would still need the index; the
        // enumerate form keeps clippy's needless-range-loop lint satisfied
        // without pretending the two are independent.
        for (ch, slot) in out.iter_mut().enumerate().take(n) {
            let dens = |i: usize, j: usize, k: usize| self.clut[base + i + j + k + ch];
            let c0 = dens(big_x0, big_y0, big_z0);
            let (c1, c2, c3) = if rx >= ry && ry >= rz {
                (
                    dens(big_x1, big_y0, big_z0) - c0,
                    dens(big_x1, big_y1, big_z0) - dens(big_x1, big_y0, big_z0),
                    dens(big_x1, big_y1, big_z1) - dens(big_x1, big_y1, big_z0),
                )
            } else if rx >= rz && rz >= ry {
                (
                    dens(big_x1, big_y0, big_z0) - c0,
                    dens(big_x1, big_y1, big_z1) - dens(big_x1, big_y0, big_z1),
                    dens(big_x1, big_y0, big_z1) - dens(big_x1, big_y0, big_z0),
                )
            } else if rz >= rx && rx >= ry {
                (
                    dens(big_x1, big_y0, big_z1) - dens(big_x0, big_y0, big_z1),
                    dens(big_x1, big_y1, big_z1) - dens(big_x1, big_y0, big_z1),
                    dens(big_x0, big_y0, big_z1) - c0,
                )
            } else if ry >= rx && rx >= rz {
                (
                    dens(big_x1, big_y1, big_z0) - dens(big_x0, big_y1, big_z0),
                    dens(big_x0, big_y1, big_z0) - c0,
                    dens(big_x1, big_y1, big_z1) - dens(big_x1, big_y1, big_z0),
                )
            } else if ry >= rz && rz >= rx {
                (
                    dens(big_x1, big_y1, big_z1) - dens(big_x0, big_y1, big_z1),
                    dens(big_x0, big_y1, big_z0) - c0,
                    dens(big_x0, big_y1, big_z1) - dens(big_x0, big_y1, big_z0),
                )
            } else if rz >= ry && ry >= rx {
                (
                    dens(big_x1, big_y1, big_z1) - dens(big_x0, big_y1, big_z1),
                    dens(big_x0, big_y1, big_z1) - dens(big_x0, big_y0, big_z1),
                    dens(big_x0, big_y0, big_z1) - c0,
                )
            } else {
                (0.0, 0.0, 0.0)
            };
            *slot = c0 + c1 * rx + c2 * ry + c3 * rz;
        }
    }
}

/// lcms2's input clamp, `cmsintrp.c`:
///
/// ```c
/// cmsINLINE cmsFloat32Number fclamp(cmsFloat32Number v)
/// {
///     return ((v < 1.0e-9f) || isnan(v)) ? 0.0f : (v > 1.0f ? 1.0f : v);
/// }
/// ```
///
/// Note the **1×10⁻⁹ floor**, which is not the same as `max(v, 0)`: lcms2
/// snaps a very small positive input to exactly zero. Transcribed rather than
/// approximated, because the whole point of this emulation is that it is
/// lcms2's arithmetic and not a paraphrase of it.
pub(crate) fn fclamp(v: f64) -> f64 {
    if v < 1.0e-9 || v.is_nan() {
        0.0
    } else if v > 1.0 {
        1.0
    } else {
        v
    }
}

/// ★ **lcms2's** cell origin and fraction — *not* the same convention as
/// [`cell`], and the difference is load-bearing at the top of each axis.
///
/// iccce (and `Clut::eval`, and every 1-D table evaluator in `iccce-cmm`)
/// clamps the cell index to `points − 2` and lets the fraction reach 1.0, so
/// an input of exactly 1.0 is "the last cell, fully to the right". lcms2
/// instead takes `k0 = floor(pk)` **unclamped** — which is `points − 1` at the
/// top — with `rest = 0`, and separately collapses the upper node
/// (`K1 = K0 + (Input >= 1.0 ? 0 : opta)`).
///
/// Both land on the same value; the two conventions are equivalent *provided
/// each is used with its own upper-node rule*. Mixing them — which the first
/// draft of this module did — returns the value at node 0 for an input of 1.0
/// and was caught by
/// [`tests::both_schemes_reproduce_a_separable_function_exactly`], which is
/// exactly what that test is for.
pub(crate) fn cell_lcms2(x: f64, points: usize) -> (usize, f64) {
    let x = fclamp(x);
    #[allow(clippy::cast_precision_loss)]
    let pk = x * (points - 1) as f64;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let k0 = pk.floor() as usize;
    #[allow(clippy::cast_precision_loss)]
    let rest = pk - k0 as f64;
    (k0, rest)
}

/// Cell origin and fraction for one axis of a `points`-node grid — **iccce's**
/// convention, mirroring `iccce_cmm::clut::Clut::eval`.
pub(crate) fn cell(x: f64, points: usize) -> (usize, f64) {
    let x = x.clamp(0.0, 1.0);
    #[allow(clippy::cast_precision_loss)]
    let pos = x * (points - 1) as f64;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let idx = (pos.floor() as usize).min(points - 2);
    #[allow(clippy::cast_precision_loss)]
    let frac = pos - idx as f64;
    (idx, frac)
}

/// 1-D table linear interpolation over `[0,1]` — same shape as
/// `lut_transform.rs`'s `interp_table`, which is the function this one must
/// agree with.
pub(crate) fn interp_table(t: &[f64], x: f64) -> f64 {
    let n = t.len();
    let x = x.clamp(0.0, 1.0);
    #[allow(clippy::cast_precision_loss)]
    let pos = x * (n - 1) as f64;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let idx = (pos.floor() as usize).min(n - 2);
    #[allow(clippy::cast_precision_loss)]
    let frac = pos - idx as f64;
    t[idx] + (t[idx + 1] - t[idx]) * frac
}

// ===========================================================================
// Loading the corpus
// ===========================================================================

/// A2B tag signatures, intent-indexed per 8.10.2 — the same mapping
/// `iccce_cmm::transform` uses, restated here because this module reads the
/// tags directly for the PCS-side records.
mod tag {
    use iccce_profile::num::Signature;
    pub const A2B0: Signature = Signature(0x4132_4230);
    pub const A2B1: Signature = Signature(0x4132_4231);
    pub const A2B2: Signature = Signature(0x4132_4232);
}

fn a2b_sig(intent: Intent) -> Signature {
    match intent {
        Intent::Perceptual => tag::A2B0,
        Intent::RelativeColorimetric | Intent::AbsoluteColorimetric => tag::A2B1,
        Intent::Saturation => tag::A2B2,
    }
}

fn cmm_intent(intent: Intent) -> CmmIntent {
    match intent {
        Intent::Perceptual => CmmIntent::Perceptual,
        Intent::RelativeColorimetric => CmmIntent::MediaRelative,
        Intent::Saturation => CmmIntent::Saturation,
        Intent::AbsoluteColorimetric => CmmIntent::Absolute,
    }
}

fn read_lut16(profile: &Profile, sig: Signature) -> Option<Lut16> {
    let entry = profile.tags.iter().find(|t| t.sig == sig)?;
    match profile.decode_tag(entry) {
        Some(Ok(d)) => match d.data {
            TagData::Lut16(l) => Some(l),
            _ => None,
        },
        _ => None,
    }
}

// ===========================================================================
// The analysis
// ===========================================================================

/// Everything one intent's comparison measured.
#[derive(Debug)]
pub struct IntentRun {
    pub intent: Intent,
    /// iccce's destination device RGB, 0..1, from the **shipped**
    /// `iccce transform` binary (6-decimal print, re-parsed).
    pub iccce_rgb: Vec<[f64; 3]>,
    /// lcms2's destination device RGB as printed, 0..255.
    pub lcms2_rgb_255: Vec<[f64; 3]>,
    /// lcms2's PCS rendering of the source profile alone, D50 CIELAB.
    /// `None` at the absolute intent — see [`IntentRun::pcs_note`].
    pub lcms2_pcs: Option<Vec<Lab>>,
    /// iccce's PCS, n-linear (the shipped choice).
    pub iccce_pcs: Vec<Lab>,
    /// The same pipeline with lcms2's CLUT geometry emulated.
    pub emulated_pcs: Vec<Lab>,

    // --- reductions -------------------------------------------------------
    pub device_dev_clamped: Vec<f64>,
    pub device_dev_raw: Vec<f64>,
    pub de_end_to_end: Vec<f64>,
    /// ΔE2000, iccce's PCS vs lcms2's PCS. Empty when `lcms2_pcs` is `None`.
    pub de_pcs: Vec<f64>,
    /// ΔE2000, emulated-geometry PCS vs lcms2's PCS.
    pub de_pcs_emulated: Vec<f64>,
    /// ΔE2000, n-linear vs emulated geometry — **the method envelope**, which
    /// involves no lcms2 output at all.
    pub de_method_envelope: Vec<f64>,
    /// The method envelope propagated end-to-end through the destination
    /// model, in ΔE2000 and in device units.
    pub de_method_end_to_end: Vec<f64>,
    pub device_method_end_to_end: Vec<f64>,
    /// ★ **The absolute-intent white-point-policy experiment.** Per-point
    /// ΔE2000 between lcms2's absolute output and a *re-prediction* of it in
    /// which two of iccce's choices are replaced by lcms2's:
    /// (i) the CLUT geometry, and (ii) the **destination media white**, which
    /// lcms2 substitutes with D50 for a **v2 display-class** profile
    /// regardless of what the `wtpt` tag says (`cmsio1.c`
    /// `_cmsReadMediaWhitePoint`).
    ///
    /// Empty except at the absolute intent. See README §14.6.
    pub de_wp_policy: Vec<f64>,
    /// lcms2 output components outside `[0,1]`, as `(point, channel, value)`.
    pub lcms2_out_of_range: Vec<(usize, usize, f64)>,
    /// Max |Δ| between the harness's n-linear reimplementation and
    /// `iccce_cmm`'s own `Lut16Model`, in `L*`/`a*`/`b*` units. **Must be
    /// f64 noise or nothing in this module means anything.**
    pub self_check: f64,
}

impl IntentRun {
    /// Why the PCS-side records skip at the absolute intent.
    pub fn pcs_note(&self) -> &'static str {
        if self.lcms2_pcs.is_some() {
            "PCS side compared against transicc -o*Lab4 at this intent"
        } else {
            "PCS-side records SKIP at icc-absolute: transicc -o*Lab4 -t3 applies the D.6/D.7 \
             media-white scale to the PCS on lcms2's side, and iccce's A2Bx evaluation is \
             media-relative by construction, so the two are not the same quantity. \
             Comparing them would require reproducing lcms2's absolute handling in the harness, \
             which would be modelling the oracle rather than measuring it. The end-to-end \
             records DO run at absolute; only the PCS isolation is withheld"
    }
    }
}

/// One whole Pass 4 run: the corpus facts, and one [`IntentRun`] per intent.
#[derive(Debug)]
pub struct Analysis {
    pub src_path: PathBuf,
    pub dst_path: PathBuf,
    pub grid: Vec<[f64; 4]>,
    pub runs: Vec<IntentRun>,
    /// `(source, destination)` header version words, read from the parsed
    /// headers. **Both must be below `0x04000000`** or lcms2's forced-BPC
    /// branch is live at perceptual and saturation (DL-013 / M2).
    pub version_words: (u32, u32),
    /// A one-line structural summary of both profiles, printed on every
    /// record so a substituted profile cannot pass unnoticed.
    pub structure: String,
    /// Max |Δ| in device units between the perceptual and saturation outputs,
    /// on iccce's side and on lcms2's side.
    pub per_vs_sat_iccce: f64,
    pub per_vs_sat_lcms2: f64,
    pub oracle_banner: String,
    /// Which `iccce` binary answered, and whether it was a debug build.
    pub iccce_exe: PathBuf,
    pub iccce_is_debug: bool,
}

/// Why a Pass 4 run could not happen.
#[derive(Debug)]
pub enum Unavailable {
    Skip(String),
    Error(String),
}

impl std::fmt::Display for Unavailable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unavailable::Skip(s) | Unavailable::Error(s) => f.write_str(s),
        }
    }
}

impl From<DiffError> for Unavailable {
    fn from(e: DiffError) -> Self {
        Unavailable::Error(e.to_string())
    }
}

/// Run the whole Pass 4 experiment.
///
/// # Errors
/// [`Unavailable::Skip`] when a category (c) profile is absent;
/// [`Unavailable::Error`] when a profile refuses to parse, a model refuses to
/// build, or the oracle misbehaves.
pub fn analyse(oracle: &Oracle, src_path: &Path, dst_path: &Path) -> Result<Analysis, Unavailable> {
    for p in [src_path, dst_path] {
        if !p.is_file() {
            return Err(Unavailable::Skip(format!(
                "profile not present on this machine: {} (LEGAL.md §3 category (c): \
                 read locally, never committed, never a required input)",
                p.display()
            )));
        }
    }
    let iccce = match Iccce::locate() {
        Err(e) => return Err(Unavailable::Error(e.to_string())),
        Ok(None) => {
            return Err(Unavailable::Skip(
                "iccce binary not found: run `cargo build --release -p iccce-cli`, or set \
                 $ICCCE_BIN. Pass 4 needs the N-channel `transform` of commit 490191b or later"
                    .to_string(),
            ));
        }
        Ok(Some(i)) => i,
    };
    let src_bytes = std::fs::read(src_path).map_err(|e| Unavailable::Error(e.to_string()))?;
    let dst_bytes = std::fs::read(dst_path).map_err(|e| Unavailable::Error(e.to_string()))?;
    let src = Profile::parse(&src_bytes).map_err(|e| Unavailable::Error(e.to_string()))?;
    let dst = Profile::parse(&dst_bytes).map_err(|e| Unavailable::Error(e.to_string()))?;
    let dst_model = MatrixTrc::from_profile(&dst)
        .map_err(|e| Unavailable::Error(format!("destination has no matrix/TRC model: {e}")))?;

    let structure = format!(
        "src: v{:08X} {} {}->{} {} tags (A2B0@{} A2B1@{} A2B2@{}) | dst: v{:08X} {} {}->{} {} tags, \
         B2A present={}",
        src.header.version.raw,
        src.header.device_class,
        src.header.color_space,
        src.header.pcs,
        src.tags.len(),
        tag_offset(&src, tag::A2B0),
        tag_offset(&src, tag::A2B1),
        tag_offset(&src, tag::A2B2),
        dst.header.version.raw,
        dst.header.device_class,
        dst.header.color_space,
        dst.header.pcs,
        dst.tags.len(),
        dst.tags
            .iter()
            .any(|t| matches!(t.sig.0, 0x4232_4130..=0x4232_4132)),
    );

    let grid = grid();
    let mut runs = Vec::new();

    for intent in [
        Intent::Perceptual,
        Intent::RelativeColorimetric,
        Intent::Saturation,
        Intent::AbsoluteColorimetric,
    ] {
        // --- iccce, the SHIPPED binary ----------------------------------
        // The channel count is checked in-process first: a `Chain` that
        // refuses to build, or builds with the wrong arity, should be
        // reported as that rather than as an unparsable subprocess.
        let chain = Chain::new(&src, &dst, cmm_intent(intent))
            .map_err(|e| Unavailable::Error(format!("chain refused at {}: {e}", intent.name())))?;
        if chain.input_channels() != 4 {
            return Err(Unavailable::Error(format!(
                "chain expects {} input channels, expected 4 for a CMYK source",
                chain.input_channels()
            )));
        }
        let rows: Vec<Vec<f64>> = grid.iter().map(|q| q.to_vec()).collect();
        let iccce_rgb = iccce.transform_rows(src_path, dst_path, intent, &rows)?;

        // --- iccce's PCS side, both geometries --------------------------
        let lut = read_lut16(&src, a2b_sig(intent)).ok_or_else(|| {
            Unavailable::Error(format!("no decodable mft2 for intent {}", intent.name()))
        })?;
        let pipeline = SourcePipeline::from_lut16(&lut);
        let model = Lut16Model::from_lut16(&lut, false, PcsKind::Lab)
            .map_err(|e| Unavailable::Error(e.to_string()))?;

        let mut iccce_pcs = Vec::with_capacity(grid.len());
        let mut emulated_pcs = Vec::with_capacity(grid.len());
        let mut self_check = 0.0f64;
        for q in &grid {
            let mine = pipeline.eval_lab(q, Scheme::NLinear);
            // The apparatus self-check: the harness's n-linear arm against
            // the crate's own evaluator, every point, every intent.
            if let Some(PcsValue::Lab(theirs)) = model.device_to_pcs(q) {
                self_check = self_check
                    .max((mine.l - theirs.l).abs())
                    .max((mine.a - theirs.a).abs())
                    .max((mine.b - theirs.b).abs());
            } else {
                return Err(Unavailable::Error(
                    "Lut16Model::device_to_pcs refused a grid point".into(),
                ));
            }
            iccce_pcs.push(mine);
            emulated_pcs.push(pipeline.eval_lab(q, Scheme::Lcms2Hybrid));
        }

        // --- lcms2, subprocess: device -> device ------------------------
        // CMYK in 0..100 (cmspack.c's IsInkSpace convention, measured), RGB
        // out 0..255. The scaling happens HERE and nowhere else.
        let rgb_req = Request {
            input: Space::profile(src_path),
            output: Space::profile(dst_path),
            intent,
            precalc: Precalc::Exact,
            bpc: Bpc::Off,
            values: grid.iter().flat_map(|q| q.iter().map(|v| v * 100.0)).collect(),
        };
        let rows = oracle.convert_batch_shaped(&rgb_req, 4, 3)?;
        let lcms2_rgb_255: Vec<[f64; 3]> = rows.iter().map(|r| [r[0], r[1], r[2]]).collect();

        // --- lcms2, subprocess: device -> PCS ---------------------------
        let lcms2_pcs = if intent == Intent::AbsoluteColorimetric {
            None
        } else {
            let lab_req = Request {
                input: Space::profile(src_path),
                output: Space::lab_v4(),
                intent,
                precalc: Precalc::Exact,
                bpc: Bpc::Off,
                values: grid.iter().flat_map(|q| q.iter().map(|v| v * 100.0)).collect(),
            };
            let rows = oracle.convert_batch_shaped(&lab_req, 4, 3)?;
            Some(
                rows.iter()
                    .map(|r| Lab {
                        l: r[0],
                        a: r[1],
                        b: r[2],
                    })
                    .collect::<Vec<Lab>>(),
            )
        };

        // --- reductions --------------------------------------------------
        let n = grid.len();
        let mut device_dev_clamped = Vec::with_capacity(n);
        let mut device_dev_raw = Vec::with_capacity(n);
        let mut de_end_to_end = Vec::with_capacity(n);
        let mut de_pcs = Vec::new();
        let mut de_pcs_emulated = Vec::new();
        let mut de_method_envelope = Vec::with_capacity(n);
        let mut de_method_end_to_end = Vec::with_capacity(n);
        let mut device_method_end_to_end = Vec::with_capacity(n);
        let mut lcms2_out_of_range = Vec::new();

        for i in 0..n {
            let a = iccce_rgb[i];
            let b = [
                lcms2_rgb_255[i][0] / 255.0,
                lcms2_rgb_255[i][1] / 255.0,
                lcms2_rgb_255[i][2] / 255.0,
            ];
            let mut raw = 0.0f64;
            let mut clamped = 0.0f64;
            for c in 0..3 {
                raw = raw.max((a[c] - b[c]).abs());
                clamped = clamped.max((a[c] - b[c].clamp(0.0, 1.0)).abs());
                if !(0.0..=1.0).contains(&b[c]) {
                    lcms2_out_of_range.push((i, c, b[c]));
                }
            }
            device_dev_raw.push(raw);
            device_dev_clamped.push(clamped);
            de_end_to_end.push(delta_e_2000(
                to_lab(&dst_model, a),
                to_lab(&dst_model, b),
            ));

            if let Some(p) = &lcms2_pcs {
                de_pcs.push(delta_e_2000(iccce_pcs[i], p[i]));
                de_pcs_emulated.push(delta_e_2000(emulated_pcs[i], p[i]));
            }
            de_method_envelope.push(delta_e_2000(iccce_pcs[i], emulated_pcs[i]));

            // The method difference carried through the ACTUAL destination
            // model — the propagation that DEVICE_CROSSCHECK and DE_CROSSCHECK
            // rest on. No lcms2 output enters this quantity.
            let nl = dst_model.pcs_to_device(iccce_pcs[i].to_xyz(D50));
            let em = dst_model.pcs_to_device(emulated_pcs[i].to_xyz(D50));
            match (nl, em) {
                (Ok(nl), Ok(em)) => {
                    de_method_end_to_end
                        .push(delta_e_2000(to_lab(&dst_model, nl), to_lab(&dst_model, em)));
                    device_method_end_to_end.push(
                        (0..3).map(|c| (nl[c] - em[c]).abs()).fold(0.0f64, f64::max),
                    );
                }
                _ => {
                    de_method_end_to_end.push(f64::NAN);
                    device_method_end_to_end.push(f64::NAN);
                }
            }
        }

        // --- the absolute-intent white-point-policy experiment ------------
        // Re-predict lcms2's absolute output with TWO of iccce's choices
        // replaced by lcms2's, and nothing else: the CLUT geometry, and the
        // destination media white (D50 rather than the stored wtpt, which is
        // what `_cmsReadMediaWhitePoint` does for a v2 DISPLAY profile). If
        // the disagreement collapses, the mechanism is established rather
        // than asserted.
        let mut de_wp_policy = Vec::new();
        if intent == Intent::AbsoluteColorimetric {
            let mw_src = read_wtpt(&src).ok_or_else(|| {
                Unavailable::Error("source has no wtpt: the absolute path needs one".into())
            })?;
            for i in 0..grid.len() {
                let rel = emulated_pcs[i].to_xyz(D50);
                let abs = iccce_color::Xyz {
                    x: rel.x * (mw_src.x / D50.x),
                    y: rel.y * (mw_src.y / D50.y),
                    z: rel.z * (mw_src.z / D50.z),
                };
                let predicted = match dst_model.pcs_to_device(abs) {
                    Ok(v) => v,
                    Err(_) => {
                        de_wp_policy.push(f64::NAN);
                        continue;
                    }
                };
                let observed = [
                    lcms2_rgb_255[i][0] / 255.0,
                    lcms2_rgb_255[i][1] / 255.0,
                    lcms2_rgb_255[i][2] / 255.0,
                ];
                de_wp_policy.push(delta_e_2000(
                    to_lab(&dst_model, predicted),
                    to_lab(&dst_model, observed),
                ));
            }
        }

        runs.push(IntentRun {
            intent,
            iccce_rgb,
            lcms2_rgb_255,
            lcms2_pcs,
            iccce_pcs,
            emulated_pcs,
            device_dev_clamped,
            device_dev_raw,
            de_end_to_end,
            de_pcs,
            de_pcs_emulated,
            de_method_envelope,
            de_method_end_to_end,
            device_method_end_to_end,
            de_wp_policy,
            lcms2_out_of_range,
            self_check,
        });
    }

    // Perceptual vs saturation, both sides. A2B0 and A2B2 share tag data, so
    // both must be exactly zero.
    let per = runs.iter().find(|r| r.intent == Intent::Perceptual).expect("built above");
    let sat = runs.iter().find(|r| r.intent == Intent::Saturation).expect("built above");
    let per_vs_sat_iccce = (0..grid.len())
        .flat_map(|i| (0..3).map(move |c| (i, c)))
        .map(|(i, c)| (per.iccce_rgb[i][c] - sat.iccce_rgb[i][c]).abs())
        .fold(0.0f64, f64::max);
    let per_vs_sat_lcms2 = (0..grid.len())
        .flat_map(|i| (0..3).map(move |c| (i, c)))
        .map(|(i, c)| (per.lcms2_rgb_255[i][c] - sat.lcms2_rgb_255[i][c]).abs() / 255.0)
        .fold(0.0f64, f64::max);

    Ok(Analysis {
        src_path: src_path.to_path_buf(),
        dst_path: dst_path.to_path_buf(),
        grid,
        runs,
        version_words: (src.header.version.raw, dst.header.version.raw),
        structure,
        per_vs_sat_iccce,
        per_vs_sat_lcms2,
        oracle_banner: oracle.banner().unwrap_or_default(),
        iccce_exe: iccce.path().to_path_buf(),
        iccce_is_debug: iccce.is_debug_build(),
    })
}

/// The `wtpt` tag as stored — no substitution, no adaptation. iccce's
/// `Chain` reads it the same way (NA-007), and the difference between that and
/// what lcms2 reads is the subject of the absolute-intent experiment.
fn read_wtpt(profile: &Profile) -> Option<iccce_color::Xyz> {
    const WTPT: Signature = Signature(0x7774_7074);
    let entry = profile.tags.iter().find(|t| t.sig == WTPT)?;
    match profile.decode_tag(entry) {
        Some(Ok(d)) => match d.data {
            TagData::Xyz(v) if v.len() == 1 => Some(iccce_color::Xyz {
                x: v[0].x.to_f64(),
                y: v[0].y.to_f64(),
                z: v[0].z.to_f64(),
            }),
            _ => None,
        },
        _ => None,
    }
}

fn tag_offset(p: &Profile, sig: Signature) -> String {
    p.tags
        .iter()
        .find(|t| t.sig == sig)
        .map_or_else(|| "absent".to_string(), |t| t.offset.to_string())
}

/// Device RGB → D50 CIELAB through a matrix/TRC model. **The instrument**, and
/// it is made of the code under test; Pass 3's record 7 bounds its error
/// against lcms2's rendering of the same profile at 8.79×10⁻⁵ ΔE2000, on the
/// Adobe RGB profile. That bound is *inherited* here rather than re-measured,
/// which is a stated weakness: this pass's destination is the **sRGB** system
/// profile, and its instrument check has not been run.
fn to_lab(model: &MatrixTrc, rgb: [f64; 3]) -> Lab {
    Lab::from_xyz(model.device_to_pcs(rgb), D50)
}

fn max_mean(v: &[f64]) -> (f64, f64) {
    if v.is_empty() {
        return (f64::NAN, f64::NAN);
    }
    let max = v.iter().copied().fold(0.0_f64, f64::max);
    #[allow(clippy::cast_precision_loss)]
    let mean = v.iter().sum::<f64>() / v.len() as f64;
    (max, mean)
}

/// Max and mean over a subset of indices — used for the corner control.
fn max_mean_at(v: &[f64], idx: &[usize]) -> (f64, f64) {
    let sub: Vec<f64> = idx.iter().map(|&i| v[i]).collect();
    max_mean(&sub)
}

// ===========================================================================
// Turning the analysis into graded records
// ===========================================================================

/// The records a Pass 4 run produces: six per intent (four graded, two
/// reported-only), plus two whole-run records — the apparatus self-check and
/// the shared-tag intent identity.
#[must_use]
pub fn records(a: &Analysis) -> Vec<Record> {
    let mut out = Vec::new();
    let provenance = format!(
        "{} | src={} dst={} grid={} points | iccce={} ({}) | oracle={} | \
         end-to-end: BOTH sides are subprocesses (iccce transform --intent, N-channel, \
         commit 490191b+); PCS-side records call iccce-cmm in-process as an INSTRUMENT",
        a.structure,
        a.src_path.display(),
        a.dst_path.display(),
        a.grid.len(),
        a.iccce_exe.display(),
        if a.iccce_is_debug {
            "DEBUG BUILD - not the shipped artefact"
        } else {
            "release"
        },
        a.oracle_banner
    );
    let version_note = format!(
        "src version {:#010X}, dst version {:#010X}; lcms2's forced-BPC branch \
         (cmscnvrt.c, DL-013/M2) needs >= 0x04000000 and is therefore UNREACHABLE for this pair \
         at every intent — verified from the parsed headers in this run, not assumed",
        a.version_words.0, a.version_words.1
    );
    let both_ran = "BOTH SIDES COMPUTED IN THIS RUN — no recorded expectation is being reproduced. \
                    iccce's end-to-end numbers from the shipped `iccce transform` binary, lcms2's \
                    from the pinned transicc. Cross-check, NOT ground truth.";

    let corners = corner_indices(&a.grid);
    let self_check_max = a.runs.iter().map(|r| r.self_check).fold(0.0f64, f64::max);

    // --- whole-run record 1: the apparatus ---------------------------------
    out.push(Record::graded(
        "pass4/apparatus/harness-nlinear-matches-iccce-cmm",
        Kind::SelfConsistency,
        Metric::AbsMaxComponent,
        Tolerance::new(
            1e-9,
            "the harness reimplements the mft2 pipeline so the CLUT geometry can be SUBSTITUTED; \
             its n-linear arm must reproduce iccce_cmm::lut_transform::Lut16Model to f64 noise or \
             every number in this module is void. 1e-9 in L*/a*/b* units is ~7 orders above f64 \
             noise on this arithmetic and ~6 orders below anything colorimetric: it cannot pass a \
             real divergence and cannot fail on rounding",
        ),
        self_check_max,
        "BOTH SIDES ARE ICCCE-DERIVED. This grades the APPARATUS, not the colour: it is the \
         precondition for believing the interpolation-envelope experiment at all.",
        format!(
            "{provenance} | max |delta| in L*/a*/b* units over every grid point and all four \
             intents, harness SourcePipeline(NLinear) vs Lut16Model::device_to_pcs"
        ),
    ));

    // --- whole-run record 2: the shared-tag identity -----------------------
    out.push(Record::graded(
        "pass4/swop/perceptual-equals-saturation",
        Kind::CrossCheck,
        Metric::DeviceAbsMaxNormalised,
        INTENT_SHARED_TAG,
        a.per_vs_sat_iccce.max(a.per_vs_sat_lcms2),
        "A PROPERTY OF THE FIXTURE, asserted against both implementations: A2B0 and A2B2 point at \
         one shared block of tag data in this file (README §8.4). Not a colour claim.",
        format!(
            "{provenance} | iccce max |delta| {:.6e}, lcms2 max |delta| {:.6e}, normalised device \
             units; the record grades the larger. A non-zero value on iccce's side is an 8.10.2 \
             fallback defect; on lcms2's side it would mean the Pass 0 finding had changed",
            a.per_vs_sat_iccce, a.per_vs_sat_lcms2
        ),
    ));

    for r in &a.runs {
        let id = |what: &str| format!("pass4/swop-to-srgb/{}/{what}", intent_slug(r.intent));
        let pair = format!(
            "{provenance} | intent={} precalc=exact(-c0,NOOPTIMIZE) bpc=not-requested | \
             {version_note}",
            r.intent.name()
        );

        let (dev_max, dev_mean) = max_mean(&r.device_dev_clamped);
        let (dev_raw_max, _) = max_mean(&r.device_dev_raw);
        let (de_max, de_mean) = max_mean(&r.de_end_to_end);
        let (env_max, env_mean) = max_mean(&r.de_method_envelope);
        let (env_e2e_max, _) = max_mean(&r.de_method_end_to_end);
        let (env_dev_max, _) = max_mean(&r.device_method_end_to_end);

        // At icc-absolute the raw comparison is reported, not graded — the
        // divergence is a white-point POLICY difference, gated by its own
        // record instead. See ABSOLUTE_REPORTED.
        let (dev_tol, de_tol) = if r.intent == Intent::AbsoluteColorimetric {
            (ABSOLUTE_REPORTED, ABSOLUTE_REPORTED)
        } else {
            (DEVICE_CROSSCHECK, DE_CROSSCHECK)
        };

        out.push(Record::graded(
            id("device-vs-lcms2"),
            Kind::CrossCheck,
            Metric::DeviceAbsMaxNormalised,
            dev_tol,
            dev_max,
            both_ran,
            format!(
                "{pair} | lcms2 output clamped into [0,1] before comparison; UNCLAMPED max \
                 {dev_raw_max:.6e}, {} components outside [0,1] (README §13.4 / corpus M3). \
                 The interpolation-method envelope propagated through the destination model is \
                 {env_dev_max:.6e} in the same units — the tolerance's own quantity, recomputed \
                 in this run",
                r.lcms2_out_of_range.len()
            ),
        ));
        out.push(Record::graded(
            id("device-mean"),
            Kind::CrossCheck,
            Metric::DeviceAbsMeanNormalised,
            Tolerance::new(
                f64::INFINITY,
                "REPORTED, NOT GRADED. A mean over a grid hides exactly the outlier a colour \
                 engine gets wrong; recorded so the distribution sits on file next to the max, \
                 and never to be quoted as if it were the max",
            ),
            dev_mean,
            both_ran,
            pair.clone(),
        ));
        out.push(Record::graded(
            id("de2000-vs-lcms2"),
            Kind::CrossCheck,
            Metric::DeltaE2000Max,
            de_tol,
            de_max,
            both_ran,
            format!(
                "{pair} | both device outputs carried into D50 CIELAB through iccce's sRGB \
                 matrix/TRC model (the ruler; Pass 3 record 7 bounds a DIFFERENT profile's ruler \
                 at 8.79e-5 dE00 and that bound is INHERITED here, not re-measured). \
                 Interpolation-method envelope propagated end-to-end: {env_e2e_max:.6e} dE00 \
                 against this observed {de_max:.6e} — the disagreement is ACCOUNTED FOR, not \
                 merely small. mean {de_mean:.6e}"
            ),
        ));
        out.push(Record::graded(
            id("de2000-mean"),
            Kind::CrossCheck,
            Metric::DeltaE2000Mean,
            Tolerance::new(
                f64::INFINITY,
                "REPORTED, NOT GRADED — see the device-mean record",
            ),
            de_mean,
            both_ran,
            pair.clone(),
        ));

        // --- PCS-side records ------------------------------------------
        if r.lcms2_pcs.is_some() {
            let (pcs_max, pcs_mean) = max_mean(&r.de_pcs);
            let (emu_max, emu_mean) = max_mean(&r.de_pcs_emulated);
            let (corner_max, _) = max_mean_at(&r.de_pcs, &corners);

            out.push(Record::graded(
                id("pcs-lab-vs-lcms2"),
                Kind::CrossCheck,
                Metric::DeltaE2000Max,
                DE_PCS_CROSSCHECK,
                pcs_max,
                both_ran,
                format!(
                    "{pair} | the SOURCE profile alone: iccce's A2Bx (n-linear, NA-006) vs \
                     transicc -o*Lab4. Isolates the CLUT from the destination model. \
                     mean {pcs_mean:.6e}. The method envelope computed from the table and the two \
                     ALGORITHMS ALONE (no lcms2 output) is max {env_max:.6e} mean {env_mean:.6e} \
                     dE00 — that is what this tolerance is, and it is why this record cannot \
                     claim agreement"
                ),
            ));
            out.push(Record::graded(
                id("pcs-lab-emulated-geometry"),
                Kind::CrossCheck,
                Metric::DeltaE2000Max,
                DE_PCS_EMULATED,
                emu_max,
                "BOTH SIDES COMPUTED IN THIS RUN. iccce's side is the harness's reimplementation \
                 of the mft2 pipeline with lcms2's OWN CLUT geometry substituted for n-linear — \
                 a model of the oracle's arithmetic, as README §13.6.1 modelled its tone-curve \
                 quantisation. Cross-check, NOT ground truth, and NOT a claim about the shipped \
                 interpolation (which stays n-linear).",
                format!(
                    "{pair} | with lcms2's linear-in-C x tetrahedral-in-MYK geometry emulated in \
                     f64, max {emu_max:.6e} mean {emu_mean:.6e} dE00 against the same oracle \
                     output that the n-linear arm differs from by {pcs_max:.6e}. \
                     ★ THIS is the record that claims agreement between the two lut16 pipelines"
                ),
            ));
            out.push(Record::graded(
                id("pcs-lab-corners-interpolation-free"),
                Kind::CrossCheck,
                Metric::DeltaE2000Max,
                DE_PCS_CORNERS,
                corner_max,
                both_ran,
                format!(
                    "{pair} | the {} hypercube corners ONLY. Every one is an exact CLUT node \
                     (input tables start 0x0000, end 0xFFFF), so n-linear and tetrahedral agree \
                     identically there and this measures the rest of the pipeline — index order, \
                     table lookup, LEGACY Lab decode — with the dominant term switched off. \
                     THE SENSITIVITY CONTROL for the 1.0 dE00 gate above",
                    corners.len()
                ),
            ));
        } else {
            // The absolute intent: the PCS isolation is not comparable, but
            // the white-point-policy experiment is, and it is the record that
            // explains the whole divergence at this intent.
            let (wp_max, wp_mean) = max_mean(&r.de_wp_policy);
            out.push(Record::graded(
                id("white-point-policy-emulated"),
                Kind::CrossCheck,
                Metric::DeltaE2000Max,
                WP_POLICY_EMULATED,
                wp_max,
                "BOTH SIDES COMPUTED IN THIS RUN. The prediction replaces exactly TWO of iccce's \
                 choices with lcms2's — the CLUT geometry and the DESTINATION MEDIA WHITE — and \
                 changes nothing else. It is a model of the oracle's policy, not a claim that the \
                 policy is right: which white point ICC.1 requires here is corpus A4b, UNSOURCED.",
                format!(
                    "{pair} | lcms2 substitutes D50 for the wtpt tag of a v2 DISPLAY-class \
                     profile (cmsio1.c _cmsReadMediaWhitePoint), and the destination here is \
                     v2 'mntr' whose wtpt tag holds D65 (0.95045,1.0,1.08905) — so lcms2 scales \
                     by wtpt_src/D50 where iccce scales by wtpt_src/wtpt_dst (NA-007, as stored). \
                     Re-predicting lcms2's output with that one substitution gives max \
                     {wp_max:.6e} mean {wp_mean:.6e} dE00, against the {:.6e} dE00 the two \
                     implementations differ by unmodelled. ★ THE MECHANISM OF THE ABSOLUTE-INTENT \
                     DIVERGENCE, established rather than asserted",
                    de_max
                ),
            ));
            for what in [
                "pcs-lab-vs-lcms2",
                "pcs-lab-emulated-geometry",
                "pcs-lab-corners-interpolation-free",
            ] {
                out.push(Record::skipped(
                    id(what),
                    Kind::CrossCheck,
                    Metric::DeltaE2000Max,
                    DE_PCS_CROSSCHECK,
                    "not comparable at this intent",
                    r.pcs_note(),
                ));
            }
        }
    }

    out
}

fn intent_slug(i: Intent) -> &'static str {
    match i {
        Intent::Perceptual => "perceptual",
        Intent::RelativeColorimetric => "media-relative",
        Intent::Saturation => "saturation",
        Intent::AbsoluteColorimetric => "icc-absolute",
    }
}

/// The full set of record ids, for a run that could not happen — so a report
/// has the same shape whether or not the machine could run it.
#[must_use]
pub fn unavailable_records(u: &Unavailable) -> Vec<Record> {
    let reason = u.to_string();
    let mut specs: Vec<(String, Kind, Metric, Tolerance)> = vec![
        (
            "pass4/apparatus/harness-nlinear-matches-iccce-cmm".into(),
            Kind::SelfConsistency,
            Metric::AbsMaxComponent,
            Tolerance::new(1e-9, "apparatus self-check"),
        ),
        (
            "pass4/swop/perceptual-equals-saturation".into(),
            Kind::CrossCheck,
            Metric::DeviceAbsMaxNormalised,
            INTENT_SHARED_TAG,
        ),
    ];
    for intent in [
        Intent::Perceptual,
        Intent::RelativeColorimetric,
        Intent::Saturation,
        Intent::AbsoluteColorimetric,
    ] {
        let s = intent_slug(intent);
        specs.push((
            format!("pass4/swop-to-srgb/{s}/device-vs-lcms2"),
            Kind::CrossCheck,
            Metric::DeviceAbsMaxNormalised,
            DEVICE_CROSSCHECK,
        ));
        specs.push((
            format!("pass4/swop-to-srgb/{s}/device-mean"),
            Kind::CrossCheck,
            Metric::DeviceAbsMeanNormalised,
            Tolerance::new(f64::INFINITY, "REPORTED, NOT GRADED"),
        ));
        specs.push((
            format!("pass4/swop-to-srgb/{s}/de2000-vs-lcms2"),
            Kind::CrossCheck,
            Metric::DeltaE2000Max,
            DE_CROSSCHECK,
        ));
        specs.push((
            format!("pass4/swop-to-srgb/{s}/de2000-mean"),
            Kind::CrossCheck,
            Metric::DeltaE2000Mean,
            Tolerance::new(f64::INFINITY, "REPORTED, NOT GRADED"),
        ));
        specs.push((
            format!("pass4/swop-to-srgb/{s}/pcs-lab-vs-lcms2"),
            Kind::CrossCheck,
            Metric::DeltaE2000Max,
            DE_PCS_CROSSCHECK,
        ));
        specs.push((
            format!("pass4/swop-to-srgb/{s}/pcs-lab-emulated-geometry"),
            Kind::CrossCheck,
            Metric::DeltaE2000Max,
            DE_PCS_EMULATED,
        ));
        specs.push((
            format!("pass4/swop-to-srgb/{s}/pcs-lab-corners-interpolation-free"),
            Kind::CrossCheck,
            Metric::DeltaE2000Max,
            DE_PCS_CORNERS,
        ));
        if intent == Intent::AbsoluteColorimetric {
            specs.push((
                format!("pass4/swop-to-srgb/{s}/white-point-policy-emulated"),
                Kind::CrossCheck,
                Metric::DeltaE2000Max,
                WP_POLICY_EMULATED,
            ));
        }
    }
    specs
        .into_iter()
        .map(|(id, kind, metric, tol)| match u {
            Unavailable::Skip(_) => {
                Record::skipped(id, kind, metric, tol, "not run on this machine", reason.clone())
            }
            Unavailable::Error(_) => {
                Record::errored(id, kind, metric, tol, "not run on this machine", reason.clone())
            }
        })
        .collect()
}

/// Convenience: run the standard SWOP → system sRGB experiment.
#[must_use]
pub fn run(oracle: &Oracle) -> (Option<Analysis>, Vec<Record>) {
    match analyse(oracle, Path::new(SWOP), Path::new(SRGB)) {
        Ok(a) => {
            let r = records(&a);
            (Some(a), r)
        }
        Err(u) => (None, unavailable_records(&u)),
    }
}

// ===========================================================================
// Tests — of the apparatus, not of any colour
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_is_deterministic_and_documented_size() {
        let a = grid();
        let b = grid();
        assert_eq!(a.len(), 341, "grid size is quoted in the docs and in README §14");
        assert_eq!(a, b, "grid must not depend on clock, hash seed or thread");
    }

    #[test]
    fn grid_is_inside_the_unit_hypercube() {
        for q in grid() {
            for c in q {
                assert!((0.0..=1.0).contains(&c), "grid point out of range: {q:?}");
            }
        }
    }

    /// The control block must really be the CLUT-node corners, and there must
    /// be sixteen of them. If `grid()`'s ordering changes, this fails rather
    /// than the control silently pointing at interior points.
    #[test]
    fn corner_indices_really_are_corners() {
        let g = grid();
        let idx = corner_indices(&g);
        assert_eq!(idx.len(), 16);
        for &i in &idx {
            assert!(g[i].iter().all(|&v| v == 0.0 || v == 1.0), "{:?}", g[i]);
        }
        assert_eq!(idx, (0..16).collect::<Vec<_>>(), "corners come first");
    }

    /// The five named points README §8.4 carries oracle numbers for are all
    /// present in the grid, so the report can print them without a special
    /// case.
    #[test]
    fn named_points_are_in_the_grid() {
        let g = grid();
        for (p, name) in NAMED_POINTS {
            assert!(g.contains(&p), "named point missing: {name}");
        }
    }

    /// A hand-built 2×2×2×2 CLUT storing a **multilinear** function: n-linear
    /// must reproduce it exactly, and so must lcms2's geometry, because with
    /// one cell per axis and a multilinear generator the two schemes coincide
    /// on the tetrahedral decomposition's vertices AND on any point where the
    /// function is affine along the chosen simplex. This pins the index order
    /// (A20: first channel slowest) in both evaluators against each other.
    #[test]
    fn both_schemes_reproduce_a_separable_function_exactly() {
        // f(c,m,y,k) = c (channel 0 only) — affine in every simplex, so both
        // schemes must return it exactly, and a transposed index order would
        // return m, y or k instead.
        let points = 2usize;
        let mut clut = vec![0u16; points.pow(4) * 3];
        for c in 0..points {
            for m in 0..points {
                for y in 0..points {
                    for k in 0..points {
                        let flat = (((c * points + m) * points + y) * points + k) * 3;
                        #[allow(clippy::cast_possible_truncation)]
                        let v = (c * 65535 / (points - 1)) as u16;
                        clut[flat] = v;
                        clut[flat + 1] = 0x8000;
                        clut[flat + 2] = 0x8000;
                    }
                }
            }
        }
        let lut = Lut16 {
            input_chan: 4,
            output_chan: 3,
            clut_points: 2,
            matrix: [iccce_profile::num::S15Fixed16(0); 9],
            input_ent: 2,
            output_ent: 2,
            input_tables: vec![0, 65535, 0, 65535, 0, 65535, 0, 65535],
            clut,
            output_tables: vec![0, 65535, 0, 65535, 0, 65535],
        };
        let p = SourcePipeline::from_lut16(&lut);
        for q in [
            [0.25, 0.5, 0.75, 0.1],
            [0.9, 0.2, 0.4, 0.6],
            [1.0, 1.0, 1.0, 1.0],
            [0.0, 0.7, 0.3, 0.2],
        ] {
            let n = p.eval_lab(&q, Scheme::NLinear);
            let t = p.eval_lab(&q, Scheme::Lcms2Hybrid);
            // L* = code/652.8 with code = c * 65535.
            let expect = q[0] * 65535.0 / 652.8;
            assert!((n.l - expect).abs() < 1e-9, "n-linear {} vs {expect}", n.l);
            assert!((t.l - expect).abs() < 1e-9, "lcms2 {} vs {expect}", t.l);
            assert!(n.a.abs() < 1e-9 && t.a.abs() < 1e-9);
        }
    }

    /// At a CLUT node the two schemes must agree **exactly** — the property
    /// the corner control rests on. Built with a deliberately non-multilinear
    /// table so that agreement between nodes is not expected and agreement
    /// *at* nodes is meaningful.
    #[test]
    fn schemes_agree_exactly_at_nodes_and_differ_between_them() {
        let points = 3usize;
        let n_nodes = points.pow(4);
        let mut clut = vec![0u16; n_nodes * 3];
        // A deliberately lumpy table: a pseudo-random value per node.
        let mut x: u64 = 0x5EED_0001;
        for slot in &mut clut {
            x = x.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
            #[allow(clippy::cast_possible_truncation)]
            {
                *slot = (x >> 48) as u16;
            }
        }
        let lut = Lut16 {
            input_chan: 4,
            output_chan: 3,
            clut_points: 3,
            matrix: [iccce_profile::num::S15Fixed16(0); 9],
            input_ent: 2,
            output_ent: 2,
            input_tables: vec![0, 65535, 0, 65535, 0, 65535, 0, 65535],
            clut,
            output_tables: vec![0, 65535, 0, 65535, 0, 65535],
        };
        let p = SourcePipeline::from_lut16(&lut);
        // At nodes (multiples of 1/2 for a 3-point axis): identical.
        for q in [
            [0.0, 0.5, 1.0, 0.5],
            [1.0, 1.0, 1.0, 1.0],
            [0.5, 0.0, 0.5, 0.0],
        ] {
            let n = p.eval_lab(&q, Scheme::NLinear);
            let t = p.eval_lab(&q, Scheme::Lcms2Hybrid);
            assert!((n.l - t.l).abs() < 1e-12, "node disagreement {} {}", n.l, t.l);
            assert!((n.a - t.a).abs() < 1e-12);
        }
        // Between nodes: they must NOT agree, or the emulation is not doing
        // anything and the whole experiment would be vacuous.
        let q = [0.3, 0.7, 0.2, 0.9];
        let n = p.eval_lab(&q, Scheme::NLinear);
        let t = p.eval_lab(&q, Scheme::Lcms2Hybrid);
        assert!(
            (n.l - t.l).abs() > 1e-6 || (n.a - t.a).abs() > 1e-6,
            "the two schemes returned the same answer between nodes — the emulation is inert"
        );
    }
}
