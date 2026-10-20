// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE TWO-SIDED CLOSENESS, IN SHARES (CONCORDANCE 2.10; xylarium's
//! selection — its operator's ruling of 2026-10-20, THE FLAT RETIRED, spec
//! aa9ab61: "this is flat squares: 't²' … antithetical"; the amplitude form
//! of ff0e817d, its shares the squares of half-angle points, is deleted, not
//! kept beside this one). netstratum's closeness is its own, in degrees: a
//! ruled divergence (A7).
//!
//! THE LAW. A body's turning is shared among its completely orthogonal
//! planes: shares `w_k ≥ 0`, summing to one. Two bodies are close in a plane
//! as far as both turn there — the share they hold in common, `min(w_k,
//! w′_k)` — and there by the linear in-step measure of their phase
//! difference; turning with no partner reads as orthogonal, ½:
//!
//! ```text
//!     m  = Σ_k min(w_k, w′_k),
//!     C₂ = Σ_k min(w_k, w′_k) · (1 − 2|Δ_k|)  +  (1 − m) / 2,
//! ```
//!
//! `Δ_k` the phase difference in plane `k` in turns, reduced to `[−½, ½]`.
//! No square, no root, no tangent: shares and a linear measure, exact over ℚ.
//! Its own fixed points: a body with itself is 1; bodies turning in disjoint
//! planes are ½; one plane in antiphase is 0; one plane alone is the linear
//! in-step measure `1 − 2|Δ|`.
//!
//! THE OBSERVER (A6). The shares are the body's own; the reading is exact
//! (the EXACT frame) and carried to the BRIDGE frame only by
//! [`super::frames::bridge`], for landings.

use crate::Q;
use num_traits::{One, Signed, Zero};

/// WHY SHARES OR A CLOSENESS ARE REFUSED, by name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refused {
    /// A share below nothing.
    Negative,
    /// Shares that do not sum to the whole turning.
    Unsummed,
    /// Two bodies' shares, or the shares and their phases, over different
    /// planes.
    Unmatched,
}

/// A BODY'S SHARES OF ITS TURNING over its planes, summing to one exactly.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Shares {
    w: Vec<Q>,
}

impl Shares {
    /// The shares `w`, refused where one is negative or they do not sum to
    /// one.
    pub fn of(w: &[Q]) -> Result<Shares, Refused> {
        if w.iter().any(Signed::is_negative) {
            return Err(Refused::Negative);
        }
        if w.iter().fold(Q::zero(), |acc, x| acc + x) != Q::one() {
            return Err(Refused::Unsummed);
        }
        Ok(Shares { w: w.to_vec() })
    }

    /// The shares, plane by plane.
    pub fn shares(&self) -> &[Q] {
        &self.w
    }

    /// THE TWO-SIDED CLOSENESS to `other`, the phases `delta` (in turns, one
    /// per plane) between them, exactly. Refused where the two bodies, or the
    /// phases, are over different planes.
    pub fn closeness(&self, other: &Shares, delta: &[Q]) -> Result<Q, Refused> {
        if other.w.len() != self.w.len() || delta.len() != self.w.len() {
            return Err(Refused::Unmatched);
        }
        let two = Q::from_integer(2.into());
        let mut held = Q::zero();
        let mut close = Q::zero();
        for ((a, b), d) in self.w.iter().zip(&other.w).zip(delta) {
            let common = match a < b {
                true => a.clone(),
                false => b.clone(),
            };
            close += &common * (Q::one() - &two * reduced(d));
            held += common;
        }
        Ok(close + (Q::one() - held) / two)
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

    /// THE LAW'S OWN FIXED POINTS: itself 1; disjoint planes ½; one plane in
    /// antiphase 0; one plane alone the linear in-step measure `1 − 2|Δ|`;
    /// and the closeness is two-sided — the same read from either body.
    #[test]
    fn the_closeness_keeps_its_fixed_points() {
        let a = Shares::of(&qs(&[(1, 3), (1, 2), (1, 6)])).expect("shares");
        assert_eq!(a.closeness(&a, &qs(&[(0, 1), (0, 1), (0, 1)])), Ok(Q::one()));
        let (p0, p1) = (Shares::of(&qs(&[(1, 1), (0, 1)])).expect("plane 0"), Shares::of(&qs(&[(0, 1), (1, 1)])).expect("plane 1"));
        assert_eq!(p0.closeness(&p1, &qs(&[(1, 5), (1, 7)])), Ok(q(1, 2)));
        let one = Shares::of(&qs(&[(1, 1)])).expect("one plane");
        assert_eq!(one.closeness(&one, &qs(&[(1, 2)])), Ok(Q::zero()));
        for d in [q(1, 8), q(3, 10), q(-2, 5), q(7, 6)] {
            assert_eq!(one.closeness(&one, std::slice::from_ref(&d)), Ok(Q::one() - q(2, 1) * reduced(&d)));
        }
        let b = Shares::of(&qs(&[(2, 7), (1, 7), (4, 7)])).expect("shares");
        let delta = qs(&[(1, 12), (5, 9), (7, 10)]);
        assert_eq!(a.closeness(&b, &delta), b.closeness(&a, &delta), "two-sided");
    }

    /// REFUSED BY NAME: a negative share, shares not summing to one, planes
    /// unmatched.
    #[test]
    fn what_is_no_share_is_refused_by_name() {
        assert_eq!(Shares::of(&qs(&[(-1, 3), (4, 3)])), Err(Refused::Negative));
        assert_eq!(Shares::of(&qs(&[(1, 2), (1, 3)])), Err(Refused::Unsummed));
        let (a, b) = (Shares::of(&qs(&[(1, 1)])).expect("a"), Shares::of(&qs(&[(1, 2), (1, 2)])).expect("b"));
        assert_eq!(a.closeness(&b, &qs(&[(0, 1), (0, 1)])), Err(Refused::Unmatched));
        assert_eq!(b.closeness(&b, &qs(&[(0, 1)])), Err(Refused::Unmatched));
    }

    /// THE JOINT READ (the landing rule): xylarium's vector of 2026-10-20 —
    /// shares, phases, and the closeness it reads in the EXACT frame and the
    /// BRIDGE frame (the numerator over 2^64) — reproduced bit for bit.
    #[test]
    fn xylariums_read_is_reproduced() {
        let whole: num_bigint::BigInt = num_bigint::BigInt::from(1u8) << 64usize;
        type Row = (Vec<Q>, Vec<Q>, Vec<Q>, Q, num_bigint::BigInt);
        let rows: Vec<Row> = vec![
            (qs(&[(1, 1), (0, 1)]), qs(&[(1, 1), (0, 1)]), qs(&[(0, 1), (1, 3)]), Q::one(), whole.clone()),
            (qs(&[(1, 1), (0, 1)]), qs(&[(0, 1), (1, 1)]), qs(&[(1, 5), (1, 7)]), q(1, 2), 9_223_372_036_854_775_808u64.into()),
            (qs(&[(1, 1)]), qs(&[(1, 1)]), qs(&[(1, 2)]), Q::zero(), 0u64.into()),
            (qs(&[(1, 1)]), qs(&[(1, 1)]), qs(&[(1, 8)]), q(3, 4), 13_835_058_055_282_163_712u64.into()),
            (qs(&[(3, 5), (2, 5)]), qs(&[(1, 4), (3, 4)]), qs(&[(1, 8), (1, 3)]), q(119, 240), 9_146_510_603_214_319_342u64.into()),
            (qs(&[(1, 3), (1, 2), (1, 6)]), qs(&[(2, 7), (1, 7), (4, 7)]), qs(&[(1, 12), (5, 9), (7, 10)]), q(659, 1260), 9_647_939_956_011_582_948u64.into()),
            (
                qs(&[(1, 60), (29, 60), (1, 4), (1, 4)]),
                qs(&[(1, 2), (1, 6), (1, 6), (1, 6)]),
                qs(&[(1, 60), (59, 60), (1, 2), (1, 3)]),
                q(427, 900),
                8_751_955_243_859_976_155u64.into(),
            ),
        ];
        for (w, w2, delta, c2, numer) in rows {
            let (a, b) = (Shares::of(&w).expect("shares"), Shares::of(&w2).expect("shares"));
            let read = a.closeness(&b, &delta).expect("over one set of planes");
            assert_eq!(read, c2, "the EXACT frame");
            assert_eq!(bridge(&read), Q::new(numer, whole.clone()), "the BRIDGE frame");
        }
    }
}
