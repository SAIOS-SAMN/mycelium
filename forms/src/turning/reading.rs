// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! ONE PLANE'S CLOSED FORMS AND THE DEGREES AS A READING (78.173 fixture
//! 11b, netstratum's statement of 2026-11-06).
//!
//! A SIMPLE RATE `B = b·e` on one plane, `e` its unit bivector:
//! - ELLIPTIC (`e² = −1`): `R = ((1 − b²) + 2b·e)/(1 + b²)`, and two such
//!   motions compose as `(b₁ + b₂)/(1 − b₁b₂)`;
//! - HYPERBOLIC (`e² = +1`): `R = ((1 + b²) + 2b·e)/(1 − b²)`, and two
//!   compose as `(b₁ + b₂)/(1 + b₁b₂)`, the flow law.
//!
//! These are the general law ([`super::motion`], [`super::compose_rates`])
//! read on one plane, never a second law: the claims check one against the
//! other.
//!
//! DEGREES ARE A READING ONLY. On an elliptic plane `tan(θ/4) = b`, `θ` the
//! turning a vector undergoes under the sandwich `R·v·R̃`. The reading is a
//! CERTIFIED SECTOR ([`degrees`]): the one sexagesimal sector at a stated
//! depth that holds `θ`, fixed by exact rational bounds on `θ` refined
//! until both bounds fall in it. Nothing is compared, stored or decided by
//! it (LAW-COORDINATE §1, base 60 display only). A rational `b` is a
//! Pythagorean angle: `θ` is a rational number of degrees only at `b = 0`
//! and `b = ±1` (`0°`, `±180°`), and the bounds close on every other.

use super::{TurningRefusal, motion};
use crate::coordinate::{Sector, DEGREES_PER_TURN};
use crate::Q;
use algebra::Clifford;
use num_bigint::BigInt;
use num_traits::{One, Signed, Zero};

/// HOW A PLANE TURNS, read from its unit bivector's square: elliptic where
/// `e² = −1` (a turning), hyperbolic where `e² = +1` (a boost).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Regime {
    Elliptic,
    Hyperbolic,
}

impl Regime {
    /// THE REGIME OF THE PLANE `e`: one blade of grade 2 at coefficient
    /// `±1`, its square read by the product. Refused `NotGenerator` with
    /// its masks where `e` is no such blade.
    pub fn of(e: &Clifford) -> Result<Regime, TurningRefusal> {
        let terms: Vec<(usize, Q)> = e.terms().map(|(m, c)| (m, c.clone())).collect();
        let refused = || TurningRefusal::NotGenerator { masks: terms.iter().map(|(m, _)| *m).collect() };
        match terms.as_slice() {
            [(mask, c)] if mask.count_ones() == 2 && c.abs().is_one() => {}
            _ => return Err(refused()),
        }
        let square: Vec<(usize, Q)> = e.product(e)?.terms().map(|(m, c)| (m, c.clone())).collect();
        match square.as_slice() {
            [(0, c)] if *c == -Q::one() => Ok(Regime::Elliptic),
            [(0, c)] if c.is_one() => Ok(Regime::Hyperbolic),
            _ => Err(refused()),
        }
    }
}

/// THE MOTION OF `b·e` ON THE ONE PLANE `e`, by the general law: refused
/// as [`Regime::of`] and [`super::motion`] refuse.
pub fn on_plane(b: &Q, e: &Clifford) -> Result<Clifford, TurningRefusal> {
    Regime::of(e)?;
    motion(&Clifford::blade(e.negatives(), 0, b.clone())?.product(e)?)
}

/// TWO RATES ON ONE PLANE COMPOSED, the closed form of the regime:
/// `(b₁ + b₂)/(1 − b₁b₂)` elliptic, `(b₁ + b₂)/(1 + b₁b₂)` hyperbolic.
/// Refused `NoMotion` where a hyperbolic rate is `±1` (its `1 − b²` is
/// zero), and `NoGenerator` where the denominator vanishes: the composite
/// is a motion with no generator (`R = −1` on an elliptic plane).
pub fn composed(regime: Regime, b1: &Q, b2: &Q) -> Result<Q, TurningRefusal> {
    let product = b1 * b2;
    let denominator = match regime {
        Regime::Elliptic => Q::one() - product,
        Regime::Hyperbolic => {
            if [b1, b2].iter().any(|b| (*b * *b).is_one()) {
                return Err(TurningRefusal::NoMotion);
            }
            Q::one() + product
        }
    };
    match denominator.is_zero() {
        true => Err(TurningRefusal::NoGenerator),
        false => Ok((b1 + b2) / denominator),
    }
}

/// THE PARTIAL SUM AND THE NEXT TERM OF EULER'S SERIES for `atan(x)`,
/// `x > 0`, at `terms` terms: `tₖ₊₁ = tₖ·(2k + 2)/(2k + 3)·x²/(1 + x²)`
/// from `t₀ = x/(1 + x²)`. Every term is positive and the ratio is below
/// `r = x²/(1 + x²)`, so the remainder lies below `next·(1 + x²)`.
fn euler(x: &Q, terms: u32) -> (Q, Q) {
    let lift = Q::one() + x * x;
    let ratio = x * x / &lift;
    let mut t = x / &lift;
    let mut sum = Q::zero();
    for k in 0..terms {
        sum += &t;
        let k = BigInt::from(k);
        t = t * Q::new(BigInt::from(2) * &k + 2, BigInt::from(2) * &k + 3) * &ratio;
    }
    (sum, t)
}

/// EXACT BOUNDS `lo < atan(x) < hi` for `0 < x`, at `terms` terms.
fn arctangent(x: &Q, terms: u32) -> (Q, Q) {
    let (sum, next) = euler(x, terms);
    let hi = &sum + next * (Q::one() + x * x);
    (sum, hi)
}

/// EXACT BOUNDS `lo < θ < hi` on `θ = 4·atan(b)` in degrees for `b > 0`,
/// at `terms` terms of each series: `θ = 720·atan(b)/π` with
/// `π = 4·atan(1)`, and `θ = 360 − 720·atan(1/b)/π` past `b = 1`, where
/// the series runs slower.
fn bounds(b: &Q, terms: u32) -> (Q, Q) {
    let turn = Q::from_integer(BigInt::from(DEGREES_PER_TURN));
    let two_turns = &turn + &turn;
    let (quarter_lo, quarter_hi) = arctangent(&Q::one(), terms);
    let four = Q::from_integer(BigInt::from(4));
    let (pi_lo, pi_hi) = (&quarter_lo * &four, &quarter_hi * &four);
    match *b > Q::one() {
        false => {
            let (a_lo, a_hi) = arctangent(b, terms);
            (&two_turns * a_lo / &pi_hi, &two_turns * a_hi / &pi_lo)
        }
        true => {
            let (a_lo, a_hi) = arctangent(&(Q::one() / b), terms);
            (&turn - &two_turns * a_hi / &pi_lo, &turn - &two_turns * a_lo / &pi_hi)
        }
    }
}

/// THE CERTIFIED SECTOR OF THE TURNING `θ = 4·atan(b)` IN DEGREES at
/// `depth`, on an elliptic plane (display only). `b = 0` and `b = ±1` read
/// exactly (`0°`, `±180°`); every other rational `b` is a Pythagorean
/// angle whose degrees are irrational, so the exact bounds, refined by
/// doubling the terms, fall in one sector at every depth. `None` only
/// where the terms would pass their `u32` carrier.
pub fn degrees(b: &Q, depth: u32) -> Option<Sector> {
    let half_turn = Q::from_integer(BigInt::from(DEGREES_PER_TURN)) / Q::from_integer(BigInt::from(2));
    if b.is_zero() || b.abs().is_one() {
        return Some(Sector::holding(&(b * half_turn), depth));
    }
    let mut terms = 1u32;
    loop {
        let (lo, hi) = bounds(&b.abs(), terms);
        let (lo, hi) = match b.is_negative() {
            true => (-hi, -lo),
            false => (lo, hi),
        };
        let sector = Sector::holding(&lo, depth);
        if sector == Sector::holding(&hi, depth) {
            return Some(sector);
        }
        terms = terms.checked_mul(2)?;
    }
}
