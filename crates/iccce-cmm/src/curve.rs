//! # Tone-curve evaluation and inversion
//!
//! Evaluates the curves `iccce-profile` represents (`curveType`'s
//! three cases, `parametricCurveType`'s five function types) and
//! inverts them per **ICC.1:2022 Annex F.1 — which is NORMATIVE**
//! (`ICC_Spec/icc/icc__s__computational_models.md`, primary_spec; the
//! corpus's first pass wrongly recorded inversion as unspecified
//! because Annex F had not been read).
//!
//! ## The rules implemented here, with their clauses
//!
//! - **Sampled-table interpolation is linear**, normatively (clause
//!   10.6 verbatim: "Function values between the entries shall be
//!   obtained through linear interpolation" — corpus A15, RESOLVED).
//! - **Parametric outputs are clipped to [0,1]**, normatively (clause
//!   10.18 verbatim: "The domain and range of each function shall be
//!   [0,0 1,0]. Any function value outside the range shall be clipped"
//!   — corpus A19, RESOLVED).
//! - **Inversion follows F.1**: monotone non-constant curves invert by
//!   coordinate interchange; a **flat subdomain** inverts to the
//!   **highest** x when the plateau ends before the domain's end and
//!   the **lowest** x when it reaches it (the rule FLIPS — "a printer
//!   profile with a flat shadow shoulder inverts to the wrong ink
//!   limit" if it doesn't); out-of-range y clamps to the nearest
//!   attainable y (F.1(b), one of A9's normative clips).
//! - **Two failure states that must not merge** (F.2/F.3 verbatim):
//!   a *constant* curve **cannot** be inverted (hard error); a
//!   *non-monotonic* curve's inverse is **undefined** (the spec allows
//!   anything; iccce refuses and reports rather than choosing
//!   silently, per the report-don't-repair rule).
//!
//! ## Named divergence from ICC's sample code
//!
//! `pow(negative, fractional)` is NaN. lcms2 guards the base before
//! calling `pow`; ICC's own `IccTagLut.cpp` does not — a real
//! behavioural difference between the two reference implementations on
//! malformed/extreme parameters (`icc__type__curve_parametric.md`
//! Guards). **iccce follows lcms2 (guard, evaluate the clamped-legal
//! branch)**, recorded as a deliberate divergence from ICC's sample
//! code. Cost: none on well-formed curves; on malformed ones it turns
//! NaN into a defined, reported value.

use iccce_profile::tag_types::{Curve, ParametricCurve};

/// A tone curve in evaluable form, converted from the profile layer's
/// raw representation. Table entries stay `u16` (the file's values);
/// normalisation by 65535.0 happens at evaluation, where it is cited.
#[derive(Debug, Clone, PartialEq)]
pub enum Trc {
    /// `curveType` count 0.
    Identity,
    /// `curveType` count 1 (u8Fixed8, already exact in f64).
    Gamma(f64),
    /// `curveType` count ≥ 2. INVARIANT: len ≥ 2 (enforced in
    /// `from_curve`).
    Table(Vec<u16>),
    /// `parametricCurveType`, params as f64 in Table 67 order
    /// (g, a, b, c, d, e, f — only the first N for the type).
    Parametric { func_type: u16, params: Vec<f64> },
}

/// Why a curve could not be converted, evaluated or inverted. Every
/// variant is a *report*; none is a silent fallback.
#[derive(Debug, Clone, PartialEq)]
pub enum CurveError {
    /// A `curveType` with count ≥ 2 shorter than 2 entries can't reach
    /// here (the profile layer would have refused), but the constructor
    /// checks anyway — belt and braces on an invariant.
    TableTooShort,
    /// Parametric funcType outside 0..=4, or params shorter than
    /// Table 67 requires: not evaluable.
    ParametricUnevaluable { func_type: u16 },
    /// F.2/F.3: a constant curve CANNOT be inverted. Hard error.
    ConstantNotInvertible,
    /// F.2/F.3: non-monotonic (but non-constant) — inverse is
    /// UNDEFINED. iccce refuses and reports instead of choosing.
    NonMonotonicInverseUndefined,
    /// Degenerate parametric parameters (a ≤ 0, c ≤ 0, or g ≤ 0 where
    /// the inverse needs them positive for monotonicity): the analytic
    /// inverse does not exist. Reported, not guessed — the same
    /// posture as a non-monotonic table.
    DegenerateParams { func_type: u16 },
}

impl std::fmt::Display for CurveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TableTooShort => write!(f, "curve table has fewer than 2 entries"),
            Self::ParametricUnevaluable { func_type } => {
                write!(f, "parametric funcType {func_type} not evaluable")
            }
            Self::ConstantNotInvertible => {
                write!(f, "constant curve cannot be inverted (Annex F, normative)")
            }
            Self::NonMonotonicInverseUndefined => write!(
                f,
                "non-monotonic curve: inverse undefined (Annex F); refused rather than chosen"
            ),
            Self::DegenerateParams { func_type } => {
                write!(
                    f,
                    "parametric funcType {func_type}: degenerate parameters, no inverse"
                )
            }
        }
    }
}

impl Trc {
    /// From a decoded `curveType`.
    pub fn from_curve(c: &Curve) -> Result<Trc, CurveError> {
        Ok(match c {
            Curve::Identity => Trc::Identity,
            Curve::Gamma(g) => Trc::Gamma(g.to_f64()),
            Curve::Table(t) => {
                if t.len() < 2 {
                    return Err(CurveError::TableTooShort);
                }
                Trc::Table(t.clone())
            }
        })
    }

    /// From a decoded `parametricCurveType`. Verifies the parameter
    /// count is at least what Table 67 requires (extra params were
    /// already reported by the profile layer; missing ones make the
    /// curve unevaluable).
    pub fn from_parametric(p: &ParametricCurve) -> Result<Trc, CurveError> {
        let needed = match p.func_type {
            0 => 1,
            1 => 3,
            2 => 4,
            3 => 5,
            4 => 7,
            _ => {
                return Err(CurveError::ParametricUnevaluable {
                    func_type: p.func_type,
                });
            }
        };
        if p.params.len() < needed {
            return Err(CurveError::ParametricUnevaluable {
                func_type: p.func_type,
            });
        }
        Ok(Trc::Parametric {
            func_type: p.func_type,
            params: p.params.iter().map(|v| v.to_f64()).collect(),
        })
    }

    /// Forward evaluation over the domain [0,1]. Input outside the
    /// domain is clamped to it (clause 10.18's normative domain for
    /// parametrics; sampled tables have no values outside it to
    /// consult; behaviour outside the domain is otherwise A19
    /// territory and clamping is the recorded choice).
    #[must_use]
    pub fn eval(&self, x: f64) -> f64 {
        let x = x.clamp(0.0, 1.0);
        match self {
            Trc::Identity => x,
            // 10.6: "the exponent … and not as an inverse".
            Trc::Gamma(g) => x.powf(*g),
            Trc::Table(t) => eval_table(t, x),
            Trc::Parametric { func_type, params } => {
                // 10.18: range shall be [0,1]; out-of-range clipped.
                eval_parametric(*func_type, params, x).clamp(0.0, 1.0)
            }
        }
    }

    /// Inverse evaluation per Annex F.1 (see module doc). `y` outside
    /// the attainable range clamps to the nearest attainable value
    /// first (F.1(b), normative).
    pub fn eval_inverse(&self, y: f64) -> Result<f64, CurveError> {
        match self {
            Trc::Identity => Ok(y.clamp(0.0, 1.0)),
            Trc::Gamma(g) => {
                if *g == 0.0 {
                    // x^0 = 1 everywhere: constant, cannot invert.
                    return Err(CurveError::ConstantNotInvertible);
                }
                Ok(y.clamp(0.0, 1.0).powf(1.0 / *g))
            }
            Trc::Table(t) => invert_table(t, y),
            Trc::Parametric { func_type, params } => invert_parametric(*func_type, params, y),
        }
    }
}

/// Sampled-table forward: entries over [0,1] with spacing 1/(n−1),
/// 0x0000 → 0.0, 0xFFFF → 1.0, LINEAR interpolation between entries
/// (clause 10.6, normative — A15).
fn eval_table(t: &[u16], x: f64) -> f64 {
    let n = t.len();
    debug_assert!(n >= 2, "constructor enforces len >= 2");
    #[allow(clippy::cast_precision_loss)] // n ≤ 65535, exact in f64
    let pos = x * (n - 1) as f64;
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    // pos ∈ [0, n−1] by the domain clamp, so floor() is a valid index.
    let idx = (pos.floor() as usize).min(n - 2);
    // WHY frac derives from the CLAMPED index: at x = 1.0, floor(pos)
    // is n−1 but the segment index clamps to n−2; pairing the clamped
    // index with the unclamped fraction (0) returns t[n−2] instead of
    // t[n−1] — TRC(1.0) ≈ 0.998 instead of 1.0, a plausible-looking
    // 0.2% error this crate's exact-value tests caught on first run
    // (2026-08-11).
    #[allow(clippy::cast_precision_loss)]
    let frac = pos - idx as f64;
    let a = f64::from(t[idx]) / 65535.0;
    let b = f64::from(t[idx + 1]) / 65535.0;
    a + (b - a) * frac
}

/// Parametric forward, types 0–4 per Table 68 (formulas
/// cross-verified in `icc__type__curve_parametric.md`; the caller
/// clips the result to [0,1] per 10.18).
///
/// Guards, per the corpus's Guards section: `a == 0` special cases
/// (type 1 → 0, type 2 → c: `−b/a` would divide by zero — both
/// reference codebases do this); negative `pow` bases evaluate the
/// legal branch value instead of NaN (the lcms2-following divergence
/// named in the module doc).
fn eval_parametric(func_type: u16, p: &[f64], x: f64) -> f64 {
    let pow_guarded = |base: f64, exp: f64| if base > 0.0 { base.powf(exp) } else { 0.0 };
    match func_type {
        0 => pow_guarded(x, p[0]),
        1 => {
            let (g, a, b) = (p[0], p[1], p[2]);
            if a == 0.0 {
                return 0.0;
            }
            if x >= -b / a {
                pow_guarded(a * x + b, g)
            } else {
                0.0
            }
        }
        2 => {
            let (g, a, b, c) = (p[0], p[1], p[2], p[3]);
            if a == 0.0 {
                return c;
            }
            if x >= -b / a {
                pow_guarded(a * x + b, g) + c
            } else {
                c
            }
        }
        3 => {
            let (g, a, b, c, d) = (p[0], p[1], p[2], p[3], p[4]);
            if x >= d {
                pow_guarded(a * x + b, g)
            } else {
                c * x
            }
        }
        4 => {
            let (g, a, b, c, d, e, f) = (p[0], p[1], p[2], p[3], p[4], p[5], p[6]);
            if x >= d {
                pow_guarded(a * x + b, g) + e
            } else {
                c * x + f
            }
        }
        _ => unreachable!("constructor rejects funcType > 4"),
    }
}

/// Table inversion per Annex F.1 (see module doc for the rules and
/// their verbatim source).
///
/// Strategy: establish direction and monotonicity in one scan, clamp
/// `y` to the attainable range (F.1(b)), then find the solution
/// interval `[x_lo, x_hi]` of `f(x) = y` and apply the plateau
/// tie-break: `x_hi` when the plateau ends before the domain's end,
/// `x_lo` when it reaches it. A linear scan is deliberate — tables are
/// ≤ 4096 entries and optimisation is Pass 6's job, after correct.
fn invert_table(t: &[u16], y: f64) -> Result<f64, CurveError> {
    let n = t.len();
    debug_assert!(n >= 2);

    // Direction and monotonicity. Plateaus (equal neighbours) are
    // allowed; a sign change of slope is not.
    let mut increasing = false;
    let mut decreasing = false;
    for w in t.windows(2) {
        match w[1].cmp(&w[0]) {
            std::cmp::Ordering::Greater => increasing = true,
            std::cmp::Ordering::Less => decreasing = true,
            std::cmp::Ordering::Equal => {}
        }
    }
    match (increasing, decreasing) {
        (false, false) => return Err(CurveError::ConstantNotInvertible),
        (true, true) => return Err(CurveError::NonMonotonicInverseUndefined),
        _ => {}
    }

    // Work on a rising view; mirror x for a falling table. The F.1
    // tie-break is stated in x-space, so the mirroring must be undone
    // before applying it — handled by mirroring the tie-break too
    // (max-x in mirrored space = min-x in real space, and the "plateau
    // reaches the domain end" condition mirrors from x_n to x_0).
    let rising: Vec<u16> = if increasing {
        t.to_vec()
    } else {
        t.iter().rev().copied().collect()
    };

    // F.1(b): clamp y to the attainable range.
    let y_min = f64::from(rising[0]) / 65535.0;
    let y_max = f64::from(rising[n - 1]) / 65535.0;
    let y = y.clamp(y_min, y_max);

    // Solution interval in the rising view.
    #[allow(clippy::cast_precision_loss)]
    let step = 1.0 / (n - 1) as f64;
    let mut x_lo: Option<f64> = None;
    let mut x_hi: Option<f64> = None;
    for i in 0..n - 1 {
        let a = f64::from(rising[i]) / 65535.0;
        let b = f64::from(rising[i + 1]) / 65535.0;
        if y < a || y > b {
            continue;
        }
        #[allow(clippy::cast_precision_loss)]
        let x_a = i as f64 * step;
        let (seg_lo, seg_hi) = if a == b {
            (x_a, x_a + step) // flat segment: whole segment maps to y
        } else {
            let x = x_a + (y - a) / (b - a) * step;
            (x, x)
        };
        if x_lo.is_none() {
            x_lo = Some(seg_lo);
        }
        x_hi = Some(seg_hi);
    }
    let (lo, hi) = (
        x_lo.expect("y clamped to attainable range, an interval exists"),
        x_hi.expect("same"),
    );

    // F.1(a) tie-break in the rising view: highest x unless the
    // plateau reaches x_n, then lowest.
    let x = if (hi - 1.0).abs() < f64::EPSILON && lo < hi {
        lo
    } else {
        hi
    };

    // Undo the mirror for a falling table.
    Ok(if increasing { x } else { 1.0 - x })
}

/// Analytic parametric inverses for all five function types.
///
/// Each is the algebraic inversion of its sourced forward formula
/// (Table 68 via `icc__type__curve_parametric.md`), assuming the
/// parameter signs that make the forward curve monotone increasing
/// (`a > 0`, `g > 0`, and `c > 0` where a linear branch exists);
/// anything else is refused as [`CurveError::DegenerateParams`] —
/// refused, not guessed, because a non-monotone parametric has an
/// UNDEFINED inverse exactly as a non-monotone table does.
///
/// Where a curve has a flat low region (types 1/2: every `x` below
/// `−b/a` maps to the same floor value), the inverse of the floor
/// follows **F.1(a)**: the plateau ends before the domain end, so the
/// HIGHEST x of the plateau (`−b/a`) is returned.
///
/// Branch selection at type 3/4's breakpoint compares against the
/// LINEAR side's top value — continuity at `X = d` is not guaranteed
/// (corpus A18: the spec imposes none), so nothing assumes the two
/// branches meet.
fn invert_parametric(func_type: u16, p: &[f64], y: f64) -> Result<f64, CurveError> {
    let y = y.clamp(0.0, 1.0);
    match func_type {
        0 => {
            let g = p[0];
            if g == 0.0 {
                return Err(CurveError::ConstantNotInvertible);
            }
            Ok(y.powf(1.0 / g))
        }
        1 => {
            // Forward: Y = (aX+b)^g for X ≥ −b/a; Y = 0 otherwise.
            let (g, a, b) = (p[0], p[1], p[2]);
            if a <= 0.0 || g <= 0.0 {
                return Err(CurveError::DegenerateParams { func_type });
            }
            let x0 = (-b / a).clamp(0.0, 1.0); // plateau end (may be 0)
            if y <= 0.0 {
                // F.1(a): plateau [0, x0] → 0; ends before x_n → highest.
                Ok(x0)
            } else {
                Ok(((y.powf(1.0 / g) - b) / a).clamp(0.0, 1.0))
            }
        }
        2 => {
            // Forward: Y = (aX+b)^g + c for X ≥ −b/a; Y = c otherwise.
            let (g, a, b, c) = (p[0], p[1], p[2], p[3]);
            if a <= 0.0 || g <= 0.0 {
                return Err(CurveError::DegenerateParams { func_type });
            }
            let x0 = (-b / a).clamp(0.0, 1.0);
            let t = y - c;
            if t <= 0.0 {
                Ok(x0) // floor value c: F.1(a) highest x of the plateau
            } else {
                Ok(((t.powf(1.0 / g) - b) / a).clamp(0.0, 1.0))
            }
        }
        3 => {
            // Forward: Y = (aX+b)^g for X ≥ d; Y = cX otherwise.
            // The inverse needs a > 0, c > 0, g > 0 to be single-valued
            // on a monotone curve; anything else is refused, not
            // guessed. Branch selection: the linear branch covers
            // Y ∈ [0, c·d); continuity at X = d is NOT guaranteed
            // (corpus A18 — the spec imposes none), so the boundary
            // value c·d is compared against, not assumed equal to
            // (ad+b)^g.
            let (g, a, b, c, d) = (p[0], p[1], p[2], p[3], p[4]);
            if a <= 0.0 || g <= 0.0 || c < 0.0 || d < 0.0 {
                return Err(CurveError::DegenerateParams { func_type });
            }
            let y_at_d_linear = c * d;
            if y < y_at_d_linear {
                if c == 0.0 {
                    return Err(CurveError::DegenerateParams { func_type });
                }
                Ok(y / c)
            } else {
                let base = y.powf(1.0 / g);
                Ok(((base - b) / a).clamp(0.0, 1.0))
            }
        }
        4 => {
            // Forward: Y = (aX+b)^g + e for X ≥ d; Y = cX + f below.
            let (g, a, b, c, d, e, f) = (p[0], p[1], p[2], p[3], p[4], p[5], p[6]);
            if a <= 0.0 || g <= 0.0 || c <= 0.0 || d < 0.0 {
                return Err(CurveError::DegenerateParams { func_type });
            }
            let y_lin_top = c * d + f;
            if y < y_lin_top {
                Ok(((y - f) / c).clamp(0.0, 1.0))
            } else {
                let t = y - e;
                if t < 0.0 {
                    // Power branch cannot produce this y; it belongs to
                    // no branch (the A18 gap between discontinuous
                    // branches). Nearest attainable is the boundary —
                    // the F.1(b) posture applied to the gap.
                    return Ok(d.clamp(0.0, 1.0));
                }
                Ok(((t.powf(1.0 / g) - b) / a).clamp(0.0, 1.0))
            }
        }
        _ => Err(CurveError::ParametricUnevaluable { func_type }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Gamma forward/inverse round trip — arithmetic identity
    /// (x^g)^(1/g) = x in exact arithmetic; tolerance is f64 noise.
    #[test]
    fn gamma_round_trip() {
        let trc = Trc::Gamma(2.19921875); // the u8Fixed8-exact 2.2
        for &x in &[0.0, 0.1, 0.5, 0.9, 1.0] {
            let back = trc.eval_inverse(trc.eval(x)).unwrap();
            assert!((back - x).abs() < 1e-12, "x={x} back={back}");
        }
    }

    /// Table forward hits the sample points exactly: entry i sits at
    /// x = i/(n−1) and decodes as t[i]/65535 (clause 10.6 layout).
    #[test]
    fn table_eval_exact_at_samples() {
        let t = vec![0u16, 13107, 32768, 65535];
        let trc = Trc::Table(t.clone());
        for (i, &e) in t.iter().enumerate() {
            #[allow(clippy::cast_precision_loss)]
            let x = i as f64 / 3.0;
            assert!((trc.eval(x) - f64::from(e) / 65535.0).abs() < 1e-15);
        }
        // Linear midpoint (A15: normatively linear).
        let mid = trc.eval(0.5 / 3.0);
        assert!((mid - (0.0 + 13107.0 / 65535.0) / 2.0).abs() < 1e-12);
    }

    /// F.1(a) first case: plateau ENDS BEFORE the domain end → the
    /// inverse is the HIGHEST x. Table: rise, flat middle, rise.
    /// Expectation from the verbatim F.1 rule, not from this code.
    #[test]
    fn f1_plateau_mid_domain_inverts_to_highest_x() {
        // 5 entries at x = 0, .25, .5, .75, 1: values 0, A, A, B, C.
        let a = 20000u16;
        let t = vec![0, a, a, 40000, 65535];
        let trc = Trc::Table(t);
        let x = trc.eval_inverse(f64::from(a) / 65535.0).unwrap();
        // Plateau spans x ∈ [0.25, 0.5]; 0.5 < 1.0 → highest.
        assert!((x - 0.5).abs() < 1e-12, "x={x}");
    }

    /// F.1(a) second case: plateau REACHES the domain end → LOWEST x.
    #[test]
    fn f1_plateau_at_domain_end_inverts_to_lowest_x() {
        let m = 65535u16;
        let t = vec![0, 30000, m, m, m];
        let trc = Trc::Table(t);
        let x = trc.eval_inverse(1.0).unwrap();
        // Plateau spans x ∈ [0.5, 1.0], reaches x_n → lowest = 0.5.
        assert!((x - 0.5).abs() < 1e-12, "x={x}");
    }

    /// F.1(b): out-of-range y clamps to the nearest attainable value.
    #[test]
    fn f1b_out_of_range_clamps_to_attainable() {
        let t = vec![6553u16, 32768, 58982]; // range ~[0.1, 0.9]
        let trc = Trc::Table(t);
        // y = 1.0 unattainable → nearest attainable is the max → x = 1.
        assert!((trc.eval_inverse(1.0).unwrap() - 1.0).abs() < 1e-12);
        // y = 0.0 → nearest is the min → x = 0.
        assert!((trc.eval_inverse(0.0).unwrap() - 0.0).abs() < 1e-12);
    }

    /// The two failure states stay distinct (F.2/F.3 verbatim:
    /// "cannot" vs "undefined") — merging them would erase exactly the
    /// distinction the spec draws.
    #[test]
    fn constant_and_nonmonotonic_are_distinct_errors() {
        let constant = Trc::Table(vec![100u16, 100, 100]);
        assert_eq!(
            constant.eval_inverse(0.5),
            Err(CurveError::ConstantNotInvertible)
        );
        let wobble = Trc::Table(vec![0u16, 40000, 20000, 65535]);
        assert_eq!(
            wobble.eval_inverse(0.5),
            Err(CurveError::NonMonotonicInverseUndefined)
        );
    }

    /// Falling tables invert with the mirror undone. The probed y is
    /// the exact encoding of the middle sample (32768/65535 — NOT 0.5;
    /// the first version of this test assumed 0.5 and failed, an
    /// encoding slip in the expectation, not in the code).
    #[test]
    fn falling_table_inverts() {
        let t = vec![65535u16, 32768, 0];
        let trc = Trc::Table(t);
        let x = trc.eval_inverse(f64::from(32768u16) / 65535.0).unwrap();
        assert!((x - 0.5).abs() < 1e-12, "x={x}");
        let x = trc.eval_inverse(1.0).unwrap();
        assert!((x - 0.0).abs() < 1e-12, "x={x}");
    }

    /// Parametric type 3 (sRGB shape) round trip, both branches.
    /// Parameters are the corpus's sRGB-as-type-3 SHAPE (γ=2.4,
    /// a=1/1.055, b=0.055/1.055, c=1/12.92, d=0.04045) — used here as
    /// an arithmetic round-trip fixture, NOT as a claim about sRGB
    /// (the sRGB file is single-source; `iec__s__srgb.md`).
    #[test]
    fn parametric_type3_round_trip() {
        let trc = Trc::Parametric {
            func_type: 3,
            params: vec![2.4, 1.0 / 1.055, 0.055 / 1.055, 1.0 / 12.92, 0.04045],
        };
        for &x in &[0.0, 0.02, 0.04045, 0.05, 0.5, 1.0] {
            let back = trc.eval_inverse(trc.eval(x)).unwrap();
            assert!((back - x).abs() < 1e-9, "x={x} back={back}");
        }
    }

    /// Types 1, 2, 4 round-trip on both branches — arithmetic
    /// identities on the sourced forward formulas. Parameters are
    /// chosen so the forward curve stays WITHIN [0,1] on the domain:
    /// this test's first version used curves exceeding 1.0, and the
    /// normative range clip (10.18) turns the excess into a top
    /// plateau whose information is destroyed — the "failure" was the
    /// inverse correctly giving F.1's lowest-x-of-an-end-plateau
    /// answer for a y the clipped curve reaches early. The code was
    /// right; the expectation ignored the clip.
    #[test]
    fn parametric_types_1_2_4_round_trip() {
        let cases = [
            // a + b = 1.0 → max exactly 1.0, no top clip.
            (1u16, vec![2.0, 1.1, -0.1]),
            // (a+b)^g + c = 0.974² + 0.05 ≈ 0.9987 < 1.
            (2u16, vec![2.0, 1.1, -0.126, 0.05]),
            // sRGB-shaped split; e = 0 so the top lands exactly at 1.
            (
                4u16,
                vec![
                    2.4,
                    1.0 / 1.055,
                    0.055 / 1.055,
                    1.0 / 12.92,
                    0.04045,
                    0.0,
                    0.0005,
                ],
            ),
        ];
        for (func_type, params) in cases {
            let trc = Trc::Parametric {
                func_type,
                params: params.clone(),
            };
            // Probe strictly above the flat/low region (x = 0.15 on)
            // for exact round trips…
            for &x in &[0.15, 0.3, 0.7, 1.0] {
                let back = trc.eval_inverse(trc.eval(x)).unwrap();
                assert!(
                    (back - x).abs() < 1e-9,
                    "type {func_type}: x={x} back={back}"
                );
            }
        }
    }

    /// The floor of types 1/2 inverts to the HIGHEST x of the plateau
    /// (F.1(a): the flat region [0, −b/a] ends before the domain end).
    /// Expectation from the verbatim F.1 rule.
    #[test]
    fn parametric_floor_inverts_to_plateau_end() {
        // −b/a = 0.125/1.25 = 0.1.
        let t1 = Trc::Parametric {
            func_type: 1,
            params: vec![2.0, 1.25, -0.125],
        };
        assert!((t1.eval_inverse(0.0).unwrap() - 0.1).abs() < 1e-12);
        let t2 = Trc::Parametric {
            func_type: 2,
            params: vec![2.0, 1.25, -0.125, 0.05],
        };
        // Floor value is c = 0.05.
        assert!((t2.eval_inverse(0.05).unwrap() - 0.1).abs() < 1e-12);
    }

    /// Degenerate parameters refuse by name instead of inventing a
    /// non-monotone inverse.
    #[test]
    fn degenerate_parametric_inverse_refused() {
        let trc = Trc::Parametric {
            func_type: 1,
            params: vec![2.0, -1.0, 0.5], // a < 0: not monotone increasing
        };
        assert_eq!(
            trc.eval_inverse(0.5),
            Err(CurveError::DegenerateParams { func_type: 1 })
        );
    }
}
