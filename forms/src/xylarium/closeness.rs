// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE TWO-SIDED CLOSENESS (CONCORDANCE 2.10; derived in xylarium 11.0
//! (27)(R6), a984d79, its operator's ruling 2026-10-20 "accept the
//! derivations; proceed with the cutover"; xylarium's selection, by its
//! 11.4b — netstratum's selection pending its operator's word on degrees).
//!
//! THE LAW. A body's turning is shared among its `P` completely orthogonal
//! planes; the shares are a DIRECTION — a rational point `x` on `S^(P−1)`,
//! the share of plane `k` being `x_k²` — built from rational half-angle
//! tangents by the inverse stereographic projection, so the amplitudes are
//! rational and no root is taken. Two bodies' closeness is
//!
//! ```text
//!     κ  = Σ_k x_k · x′_k · (1 − 4|Δ_k|),     C₂ = (1 + κ) / 2,
//! ```
//!
//! `Δ_k` the phase difference in plane `k` in turns, reduced to `[−½, ½]`.
//! Its own fixed points: a body with itself is 1; bodies wholly in
//! orthogonal planes are ½; one plane in antiphase is 0; both wholly in one
//! plane is `1 − 2|Δ|`, the linear in-step measure (netstratum's docs/79) —
//! the one-plane case of this one.
//!
//! THE OBSERVER (A6). The tangents are the body's own; the reading is exact
//! (the EXACT frame) and is carried to the BRIDGE frame only by
//! [`super::frames::bridge`], for landings.

use crate::Q;
use num_traits::{One, Signed, Zero};

/// WHY A DIRECTION OR A CLOSENESS IS REFUSED, by name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refused {
    /// A tangent below nothing: the direction is read on the orthant.
    Negative,
    /// Tangents whose square passes one: the point would leave the orthant
    /// of the sphere the shares are read on.
    Beyond,
    /// Two directions, or a direction and its phases, over different planes.
    Unmatched,
}

/// A BODY'S SHARES OVER ITS PLANES: a rational point on `S^(P−1)`, held as
/// its amplitudes `x` (the share of plane `k` is `x_k²`, the shares summing
/// to one exactly).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Direction {
    x: Vec<Q>,
}

impl Direction {
    /// THE DIRECTION OF RATIONAL HALF-ANGLE TANGENTS `t` (one fewer than the
    /// planes), by the inverse stereographic projection:
    /// `x₀ = (1 − |t|²)/(1 + |t|²)`, `x_k = 2t_k/(1 + |t|²)`. Refused where a
    /// tangent is negative or `|t|²` passes one.
    pub fn of_tangents(t: &[Q]) -> Result<Direction, Refused> {
        if t.iter().any(Signed::is_negative) {
            return Err(Refused::Negative);
        }
        let norm = t.iter().fold(Q::zero(), |acc, tk| acc + tk * tk);
        if norm > Q::one() {
            return Err(Refused::Beyond);
        }
        let over = Q::one() + &norm;
        let two = Q::from_integer(2.into());
        let mut x = Vec::with_capacity(t.len() + 1);
        x.push((Q::one() - &norm) / &over);
        x.extend(t.iter().map(|tk| &two * tk / &over));
        Ok(Direction { x })
    }

    /// The amplitudes, plane by plane.
    pub fn amplitudes(&self) -> &[Q] {
        &self.x
    }

    /// THE TWO-SIDED CLOSENESS to `other`, the phases `delta` (in turns, one
    /// per plane) between them: `C₂ = (1 + κ)/2`, exactly. Refused where the
    /// two directions, or the phases, are over different planes.
    pub fn closeness(&self, other: &Direction, delta: &[Q]) -> Result<Q, Refused> {
        if other.x.len() != self.x.len() || delta.len() != self.x.len() {
            return Err(Refused::Unmatched);
        }
        let four = Q::from_integer(4.into());
        let kappa = self
            .x
            .iter()
            .zip(&other.x)
            .zip(delta)
            .fold(Q::zero(), |acc, ((a, b), d)| acc + a * b * (Q::one() - &four * reduced(d)));
        Ok((Q::one() + kappa) / Q::from_integer(2.into()))
    }
}

/// A phase difference in turns, reduced to `[−½, ½]` and read as its size.
fn reduced(d: &Q) -> Q {
    let half = Q::new(1.into(), 2.into());
    let within = d - d.floor();
    match within > half {
        true => Q::one() - within,
        false => within,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xylarium::frames::bridge;

    fn q(n: i64, d: i64) -> Q {
        Q::new(n.into(), d.into())
    }

    fn qs(v: &[(i64, i64)]) -> Vec<Q> {
        v.iter().map(|&(n, d)| q(n, d)).collect()
    }

    /// THE SHARES SUM TO ONE, EXACTLY: the amplitudes' squares, whatever the
    /// tangents.
    #[test]
    fn the_shares_sum_to_one() {
        for t in [qs(&[(1, 2)]), qs(&[(2, 7), (1, 3)]), qs(&[(3, 11), (1, 9), (5, 6)]), qs(&[])] {
            let x = Direction::of_tangents(&t).expect("on the orthant");
            assert_eq!(x.amplitudes().iter().fold(Q::zero(), |a, xk| a + xk * xk), Q::one());
        }
    }

    /// THE LAW'S OWN FIXED POINTS: itself 1; orthogonal planes ½; one plane
    /// in antiphase 0; one plane alone the linear in-step measure `1 − 2|Δ|`.
    #[test]
    fn the_closeness_keeps_its_fixed_points() {
        let a = Direction::of_tangents(&qs(&[(2, 7), (1, 3)])).expect("a direction");
        assert_eq!(a.closeness(&a, &qs(&[(0, 1), (0, 1), (0, 1)])), Ok(Q::one()));
        let (only0, only1) = (Direction::of_tangents(&qs(&[(0, 1)])).expect("x₀"), Direction::of_tangents(&qs(&[(1, 1)])).expect("x₁"));
        assert_eq!(only0.closeness(&only1, &qs(&[(1, 5), (1, 7)])), Ok(q(1, 2)));
        assert_eq!(only0.closeness(&only0, &qs(&[(1, 2), (0, 1)])), Ok(Q::zero()));
        for d in [q(1, 8), q(3, 10), q(-2, 5), q(7, 6)] {
            assert_eq!(only0.closeness(&only0, &[d.clone(), Q::zero()]), Ok(Q::one() - q(2, 1) * reduced(&d)));
        }
    }

    /// REFUSED BY NAME: a negative tangent, tangents off the orthant, planes
    /// unmatched.
    #[test]
    fn what_is_no_direction_is_refused_by_name() {
        assert_eq!(Direction::of_tangents(&qs(&[(-1, 3)])), Err(Refused::Negative));
        assert_eq!(Direction::of_tangents(&qs(&[(4, 5), (4, 5)])), Err(Refused::Beyond));
        let (a, b) = (Direction::of_tangents(&qs(&[(1, 2)])).expect("a"), Direction::of_tangents(&qs(&[(1, 2), (1, 3)])).expect("b"));
        assert_eq!(a.closeness(&b, &qs(&[(0, 1), (0, 1)])), Err(Refused::Unmatched));
        assert_eq!(a.closeness(&a, &qs(&[(0, 1)])), Err(Refused::Unmatched));
    }

    /// THE JOINT READ (the landing rule): xylarium's vector of 2026-10-20 —
    /// tangents, phases, and the closeness it reads in the EXACT frame and
    /// the BRIDGE frame — reproduced bit for bit.
    #[test]
    fn xylariums_read_is_reproduced() {
        type Row = (Vec<Q>, Vec<Q>, Vec<Q>, Vec<Q>, Vec<Q>, Q, u64);
        let rows: Vec<Row> = vec![
            (qs(&[(1, 2)]), qs(&[(1, 3)]), qs(&[(3, 5), (4, 5)]), qs(&[(4, 5), (3, 5)]), qs(&[(1, 8), (1, 3)]), q(27, 50), 9_961_241_799_803_157_872),
            (qs(&[(1, 2)]), qs(&[(1, 2)]), qs(&[(3, 5), (4, 5)]), qs(&[(3, 5), (4, 5)]), qs(&[(0, 1), (0, 1)]), Q::one(), 0),
            (qs(&[(0, 1)]), qs(&[(1, 1)]), qs(&[(1, 1), (0, 1)]), qs(&[(0, 1), (1, 1)]), qs(&[(1, 5), (1, 7)]), q(1, 2), 9_223_372_036_854_775_808),
            (
                qs(&[(2, 7), (1, 3)]),
                qs(&[(1, 5), (3, 4)]),
                qs(&[(178, 263), (126, 263), (147, 263)]),
                qs(&[(159, 641), (160, 641), (600, 641)]),
                qs(&[(1, 12), (5, 9), (7, 10)]),
                q(154_131, 337_166),
                8_432_686_305_336_027_061,
            ),
            (
                qs(&[(3, 11), (1, 9), (5, 6)]),
                qs(&[(1, 2), (2, 3), (1, 4)]),
                qs(&[(8579, 69829), (21384, 69829), (8712, 69829), (65340, 69829)]),
                qs(&[(35, 253), (144, 253), (192, 253), (72, 253)]),
                qs(&[(1, 60), (59, 60), (1, 2), (1, 3)]),
                q(263_701_949, 530_002_110),
                9_178_156_601_945_242_106,
            ),
        ];
        let whole = Q::from_integer(num_bigint::BigInt::from(1u8) << 64);
        for (t, t2, x, x2, delta, c2, numer) in rows {
            let (a, b) = (Direction::of_tangents(&t).expect("a direction"), Direction::of_tangents(&t2).expect("a direction"));
            assert_eq!((a.amplitudes(), b.amplitudes()), (x.as_slice(), x2.as_slice()), "the amplitudes");
            let read = a.closeness(&b, &delta).expect("over one set of planes");
            assert_eq!(read, c2, "the EXACT frame");
            // The whole reads 2^64/2^64 in the bridge frame.
            let bridged = match numer {
                0 => Q::one(),
                n => Q::new(n.into(), 1.into()) / &whole,
            };
            assert_eq!(bridge(&read), bridged, "the BRIDGE frame");
        }
    }
}
