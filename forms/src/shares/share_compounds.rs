// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! A SHARE COMPOUNDS FRAME BY FRAME (CONCORDANCE 2.10; for instance a burn
//! at the rate of exhaustion, or a cooling channel).
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
pub enum ShareRefusal {
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
    pub fn after(mut self, x: &Q, rings: u64) -> Result<Remaining, ShareRefusal> {
        if x < &Q::zero() {
            return Err(ShareRefusal::Negative);
        }
        if x >= &Q::one() {
            return Err(ShareRefusal::Whole);
        }
        if rings == 0 {
            return Ok(self);
        }
        let left = Q::one() - x;
        match self.factors.last_mut() {
            Some((was, n)) if *was == left => *n = n.checked_add(rings).ok_or(ShareRefusal::Uncounted)?,
            _ => self.factors.push((left, rings)),
        }
        Ok(self)
    }

    /// THE COMPOSITION: this history, then `later`'s. The runs are joined,
    /// the seam folded where the two runs that meet there have one share —
    /// so how a history was cut into pieces leaves no trace in what remains.
    pub fn then(mut self, later: &Remaining) -> Result<Remaining, ShareRefusal> {
        for (left, rings) in &later.factors {
            match self.factors.last_mut() {
                Some((was, n)) if was == left => *n = n.checked_add(*rings).ok_or(ShareRefusal::Uncounted)?,
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
pub(super) fn power(base: &Q, e: u64) -> Q {
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
