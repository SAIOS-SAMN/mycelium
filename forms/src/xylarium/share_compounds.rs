// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! A SHARE COMPOUNDS FRAME BY FRAME (CONCORDANCE 2.10; proposed by
//! netstratum, selected by xylarium — 779a67f, its burn at the rate of
//! exhaustion and its cooling channels).
//!
//! THE LAW. Where a universe's physics gives a share `x` of what REMAINS per
//! ring of a body's own clock, `n` rings leave `(1 − x)^n`. It is derived,
//! not chosen: the law must compose — regrouping the rings must not change
//! what is left — so `f(a + b) = f(a)·f(b)` with `f(1) = 1 − x`, and on whole
//! rings that is `(1 − x)^n`. The ring count is ℤ, the share ℚ.
//!
//! THE OBSERVER (A6). The rings are the BODY'S OWN clock; a reading on
//! TUniversal, or on another body's clock, is carried there by the exact map
//! its universe states, never assumed.
//!
//! THE EXACT OBJECT, NEVER ITS EXPANSION. `(1 − x)^n` over ℚ is exact, but
//! its numerator and denominator grow with `n`. The form holds the power
//! itself — `(1 − x, n)` — and a body whose share is read again as it changes
//! (its mix, its share, its coherence) holds the PRODUCT
//! `∏ (1 − x_i)^(n_i)`: a run of rings at one share is one factor, its
//! exponents added; a changed share opens a new factor. The rational is
//! expanded only where a reading asks for it (`reading`). Nothing is
//! approximated, and nothing grows that was not asked for.

use crate::Q;
use num_traits::{One, Zero};

/// WHY A SHARE OR A COUNT IS REFUSED, by name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refused {
    /// A share below nothing: a ring takes no negative part of what remains.
    Negative,
    /// A share not below one: a share of what REMAINS leaves something, so
    /// it is below one by its parts.
    Whole,
    /// A ring count past what its carrier holds: refused, never capped.
    Uncounted,
}

/// WHAT REMAINS after a body's history: the exact product
/// `∏ (1 − x_i)^(n_i)`, one factor per run of rings at one share, in the
/// order the runs were lived. The empty product is the whole: nothing taken.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Remaining {
    /// Each run: what a ring at that share leaves (`1 − x`), and its rings.
    factors: Vec<(Q, u64)>,
}

impl Remaining {
    /// Nothing taken yet: the whole remains.
    pub fn whole() -> Remaining {
        Remaining::default()
    }

    /// `rings` more rings at the share `x`, compounding: folded into the last
    /// run where its share is the same, a new run where it is not. Refused
    /// where `x` is not a share of what remains, or the count passes its
    /// carrier.
    pub fn after(mut self, x: &Q, rings: u64) -> Result<Remaining, Refused> {
        if x < &Q::zero() {
            return Err(Refused::Negative);
        }
        if x >= &Q::one() {
            return Err(Refused::Whole);
        }
        if rings == 0 {
            return Ok(self);
        }
        let left = Q::one() - x;
        match self.factors.last_mut() {
            Some((was, n)) if *was == left => *n = n.checked_add(rings).ok_or(Refused::Uncounted)?,
            _ => self.factors.push((left, rings)),
        }
        Ok(self)
    }

    /// THE COMPOSITION: this history, then `later`'s. The runs are joined,
    /// the seam folded where the two runs that meet there have one share —
    /// so how a history was cut into pieces leaves no trace in what remains.
    pub fn then(mut self, later: &Remaining) -> Result<Remaining, Refused> {
        for (left, rings) in &later.factors {
            match self.factors.last_mut() {
                Some((was, n)) if was == left => *n = n.checked_add(*rings).ok_or(Refused::Uncounted)?,
                _ => self.factors.push((left.clone(), *rings)),
            }
        }
        Ok(self)
    }

    /// The runs, as held: what a ring at each share leaves, and its rings.
    pub fn factors(&self) -> &[(Q, u64)] {
        &self.factors
    }

    /// THE READING: the product expanded, exactly. The only place the
    /// rational is grown.
    pub fn reading(&self) -> Q {
        self.factors.iter().fold(Q::one(), |acc, (left, rings)| acc * power(left, *rings))
    }
}

/// WHAT REMAINS, READ IN ANY OBSERVER'S FRAME (ruled 2026-10-21:
/// forms read in frames that bend, twist, stretch, expand and contract —
/// never on a grid): its reading is the one-axis point it is, carried
/// exactly by the frame form.
impl crate::frame::ReadIn for Remaining {
    fn reading_point(&self) -> Vec<Q> {
        vec![self.reading()]
    }
}

/// `base^e` exactly, by squaring along the exponent's own bits.
fn power(base: &Q, e: u64) -> Q {
    let mut out = Q::one();
    let mut square = base.clone();
    let mut rest = e;
    while rest > 0 {
        if rest & 1 == 1 {
            out = &out * &square;
        }
        rest >>= 1;
        if rest > 0 {
            square = &square * &square;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn q(n: i64, d: i64) -> Q {
        Q::new(n.into(), d.into())
    }

    /// THE LAW'S OWN FIXED POINTS: nothing taken leaves the whole; no rings
    /// leave the whole; a share of nothing leaves the whole, however long.
    #[test]
    fn nothing_taken_leaves_the_whole() {
        assert_eq!(Remaining::whole().reading(), Q::one());
        assert_eq!(Remaining::whole().after(&q(1, 3), 0).map(|r| r.reading()), Ok(Q::one()));
        assert_eq!(Remaining::whole().after(&Q::zero(), 1_000_000).map(|r| r.reading()), Ok(Q::one()));
    }

    /// `(1 − x)^n`, exactly: a quarter taken each of three rings leaves 27/64.
    #[test]
    fn a_share_compounds_exactly() {
        assert_eq!(Remaining::whole().after(&q(1, 4), 3).map(|r| r.reading()), Ok(q(27, 64)));
    }

    /// COMPOSITION: regrouping the rings changes nothing — `a` then `b` rings
    /// at one share are one run of `a + b`, held and read alike, however the
    /// history was cut.
    #[test]
    fn regrouping_the_rings_changes_nothing() {
        let x = q(2, 7);
        let at_once = Remaining::whole().after(&x, 12).expect("a share");
        let in_runs = Remaining::whole().after(&x, 5).and_then(|r| r.after(&x, 4)).and_then(|r| r.after(&x, 3)).expect("a share");
        assert_eq!(in_runs, at_once, "one run, held alike");
        let first = Remaining::whole().after(&x, 7).expect("a share");
        let second = Remaining::whole().after(&x, 5).expect("a share");
        assert_eq!(first.then(&second), Ok(at_once.clone()), "two histories composed fold at their seam");
        assert_eq!(at_once.reading(), at_once.factors().iter().fold(Q::one(), |a, (l, n)| a * power(l, *n)));
    }

    /// A SHARE READ AGAIN OPENS A NEW RUN, and the reading is the product of
    /// the runs: what changed is held as the body's history, never averaged.
    #[test]
    fn a_changed_share_opens_a_new_run() {
        let r = Remaining::whole().after(&q(1, 2), 2).and_then(|r| r.after(&q(1, 3), 1)).expect("shares");
        assert_eq!(r.factors().len(), 2);
        assert_eq!(r.reading(), q(1, 4) * q(2, 3));
    }

    /// WHAT REMAINS IS READ IN A FRAME, AND BACK: a history's exact product
    /// carried into an observer's frame — contracted, twisted, bent — and
    /// back by the inverse frame returns bit for bit; no floor is taken.
    #[test]
    fn what_remains_is_read_in_a_frame_and_back() {
        use crate::frame::{Frame, Move, ReadIn};
        let r = Remaining::whole().after(&q(1, 4), 3).and_then(|r| r.after(&q(2, 9), 5)).expect("shares");
        let frame = Frame::identity().then(Move::Contract(q(3, 1))).then(Move::Twist(vec![q(1, 1)])).then(Move::Bend(vec![q(2, 5)]));
        let there = r.read_in(&frame).expect("carried");
        assert_eq!(frame.inverse().carry(&there), Ok(vec![r.reading()]), "there and back, exactly");
        assert_eq!(r.read_in(&Frame::identity()), Ok(vec![q(27, 64) * power(&q(7, 9), 5)]), "the identity reads it as it is");
    }

    /// REFUSED BY NAME: a negative share, a share that would take all that
    /// remains or more, and a count past its carrier — never clamped.
    #[test]
    fn what_is_no_share_is_refused_by_name() {
        assert_eq!(Remaining::whole().after(&q(-1, 5), 1), Err(Refused::Negative));
        assert_eq!(Remaining::whole().after(&Q::one(), 1), Err(Refused::Whole));
        assert_eq!(Remaining::whole().after(&q(3, 2), 1), Err(Refused::Whole));
        let full = Remaining::whole().after(&q(1, 9), u64::MAX).expect("a share");
        assert_eq!(full.after(&q(1, 9), 1), Err(Refused::Uncounted));
    }
}
