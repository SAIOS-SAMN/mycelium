// DG1, 78.45 — THE TURN: how far a rotor turns in one plane, measured
// against the full circle, exactly.
//
//! A rotor of the even subalgebra splits into two commuting quaternion
//! factors of one weight `w`, with scalar parts `a ± p` (15.3,
//! [`Rotor::plane_periods`]). Each factor's half-angle `θ` has
//! `cos² θ = s²/w`, an exact rational, and the factor TURNS by the double
//! angle, whose cosine is exact too: `cos 2θ = 2s²/w − 1`. So every turn has
//! an exact cosine, and its sine squared `1 − cos²` is exact; the sine itself
//! is rational exactly where that square is.
//!
//! A turn is served, in order of what is exact (78.45(3)):
//! - `Finite { t, ℓ }` where the factor has finite order — `cos² θ` is one
//!   of Niven's five `{1, 3/4, 1/2, 1/4, 0}` — the fraction `t/ℓ` of the full
//!   circle, served as `360·t/ℓ` degrees, exact;
//! - `Point { cos, sin_sq }` where it has not — the turn's point on its
//!   plane's circle, the sine taken non-negative (the plane's orientation
//!   carries the sense) — served as the point, its sine exact where
//!   `sin_sq` is a rational square, and the SECTOR of the ℚ-protractor it
//!   lies in, decided by comparing `cos²` and the sign of `cos` against the
//!   marks' own; never a decimal.

use super::*;
use num_traits::{One, Signed};

/// How far one plane turns (DG1).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Turn {
    /// `t/ℓ` of the full circle, in lowest terms, `0 ≤ t/ℓ ≤ 1/2`.
    Finite { t: u32, l: u32 },
    /// The turn's point on its circle: its cosine and its sine squared, both
    /// exact; the sine non-negative.
    Point { cos: Q, sin_sq: Q },
}

/// THE ℚ-PROTRACTOR'S MARKS (78.45(3)): the angles in `[0°, 180°]` whose
/// `cos²` is rational — Niven's set — each with its cosine's square and
/// sign, in increasing angle.
const MARKS: [(u32, (i64, i64), i8); 9] = [
    (0, (1, 1), 1),
    (30, (3, 4), 1),
    (45, (1, 2), 1),
    (60, (1, 4), 1),
    (90, (0, 1), 0),
    (120, (1, 4), -1),
    (135, (1, 2), -1),
    (150, (3, 4), -1),
    (180, (1, 1), -1),
];

impl Turn {
    /// The turn of one quaternion factor with scalar part `s` and weight
    /// `w`. `None` where `w` is not positive (no rotor).
    pub fn of_factor(s: &Q, w: &Q) -> Option<Turn> {
        if !w.is_positive() {
            return None;
        }
        if let Some((k, m)) = half_angle(s, w) {
            // The half-angle is `kπ/m`, so the turn `2kπ/m` is `k/m` of the
            // circle; `half_angle` reads `k ≤ m`, and a turn past half the
            // circle is the same turn the other way round the plane.
            let (t, l) = (k, m);
            let g = gcd_u32(t, l).max(1);
            let (t, l) = (t / g, l / g);
            let (t, l) = match 2 * t > l {
                true => (l - t, l),
                false => (t, l),
            };
            return Some(Turn::Finite { t, l });
        }
        let two = Q::from_integer(2.into());
        let cos = &(&two * &(s * s)) / w - Q::one();
        let sin_sq = Q::one() - &cos * &cos;
        Some(Turn::Point { cos, sin_sq })
    }

    /// The degrees, where they are exact: `360·t/ℓ` for a finite turn;
    /// `None` for a point, which is served by its point and its sector.
    pub fn degrees(&self) -> Option<Q> {
        match self {
            Turn::Finite { t, l } => Some(Q::new((360 * u64::from(*t)).into(), u64::from(*l).into())),
            Turn::Point { .. } => None,
        }
    }

    /// The sine where it is rational.
    pub fn sin(&self) -> Option<Q> {
        match self {
            Turn::Finite { .. } => None,
            Turn::Point { sin_sq, .. } => rational_sqrt(sin_sq),
        }
    }

    /// THE SECTOR a point lies in, as the two marks around it in degrees,
    /// decided by exact comparison of `cos²` and the sign of `cos` (78.45(3)).
    /// `None` for a finite turn, which sits ON its degrees.
    pub fn sector(&self) -> Option<(u32, u32)> {
        let Turn::Point { cos, .. } = self else { return None };
        let c2 = cos * cos;
        let sign: i8 = match (cos.is_positive(), cos.is_negative()) {
            (true, _) => 1,
            (_, true) => -1,
            _ => 0,
        };
        // The angle rises as the signed cosine falls, so the point lies
        // between the last mark whose signed cosine is above its own and the
        // first whose is not. Signed cosines compare by sign first, then by
        // the square — larger for a positive cosine, smaller for a negative
        // one — so no root is taken.
        let above = |mark: &(u32, (i64, i64), i8)| -> bool {
            let mc2 = Q::new(mark.1 .0.into(), mark.1 .1.into());
            match (mark.2, sign) {
                (m, s) if m != s => m > s,
                (1, _) => mc2 > c2,
                (-1, _) => mc2 < c2,
                _ => false,
            }
        };
        let mut lo = 0u32;
        for mark in MARKS.iter() {
            match above(mark) {
                true => lo = mark.0,
                false => {
                    return Some((lo, mark.0));
                }
            }
        }
        None
    }
}

impl std::fmt::Display for Turn {
    /// Degrees where exact, else the point with its sector; no decimal.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Turn::Finite { .. } => match self.degrees() {
                Some(d) if d.is_integer() => write!(f, "{}°", d.numer()),
                Some(d) => write!(f, "{}/{}°", d.numer(), d.denom()),
                None => write!(f, "unread"),
            },
            Turn::Point { cos, sin_sq } => {
                let sin = match self.sin() {
                    Some(s) => s.to_string(),
                    None => format!("√{sin_sq}"),
                };
                match self.sector() {
                    Some((lo, hi)) => write!(f, "({cos}, {sin}) in ({lo}°, {hi}°)"),
                    None => write!(f, "({cos}, {sin})"),
                }
            }
        }
    }
}
