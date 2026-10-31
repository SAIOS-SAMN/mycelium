// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE TWO-SIDED CLOSENESS, IN SHARES (CONCORDANCE 2.10; THE FLAT RETIRED:
//! shares read as squares `t²` are flat and antithetical to the design, so
//! an amplitude form whose shares are the squares of half-angle points is
//! deleted, not kept beside this one). A universe that reads closeness in
//! degrees holds its own form: a ruled divergence (A7).
//!
//! THE LAW. A body's turning is shared among its completely orthogonal
//! planes: shares `w_k ≥ 0`, summing to one. Two bodies are close in a plane
//! as far as both turn there — the share they hold in common, `min(w_k,
//! w′_k)` — and there by the linear in-step measure of the bodies' phase
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
//! THE OBSERVER (A6). The shares are the body's own; the reading is exact,
//! and carried into another observer's frame only by a frame form
//! ([`crate::frame::Frame`], `closeness_in`) — never floored on a grid.

use crate::Q;
use num_traits::{One, Signed, Zero};

/// WHY SHARES OR A CLOSENESS ARE REFUSED, by name.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClosenessRefusal {
    /// A share below nothing.
    Negative,
    /// Shares that do not sum to the whole turning.
    Unsummed,
    /// Two bodies' shares, or the shares and the phases, over different
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
    pub fn of(w: &[Q]) -> Result<Shares, ClosenessRefusal> {
        if w.iter().any(Signed::is_negative) {
            return Err(ClosenessRefusal::Negative);
        }
        if w.iter().fold(Q::zero(), |acc, x| acc + x) != Q::one() {
            return Err(ClosenessRefusal::Unsummed);
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
    pub fn closeness(&self, other: &Shares, delta: &[Q]) -> Result<Q, ClosenessRefusal> {
        if other.w.len() != self.w.len() || delta.len() != self.w.len() {
            return Err(ClosenessRefusal::Unmatched);
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

impl Shares {
    /// THE CLOSENESS READ IN AN OBSERVER'S FRAME: the exact reading carried
    /// by the frame form, never floored. Refused where the closeness is, or
    /// the frame does not carry it (`crate::frame::FrameRefusal`, by its name).
    pub fn closeness_in(&self, other: &Shares, delta: &[Q], frame: &crate::frame::Frame) -> Result<Q, InFrame> {
        let read = self.closeness(other, delta).map_err(InFrame::Closeness)?;
        match frame.carry(std::slice::from_ref(&read)).map_err(InFrame::Frame)?.as_slice() {
            [x] => Ok(x.clone()),
            _ => Err(InFrame::Frame(crate::frame::FrameRefusal::Unmatched)),
        }
    }
}

/// WHY A CLOSENESS IS NOT READ IN A FRAME: the closeness's own refusal, or
/// the frame's.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InFrame {
    Closeness(ClosenessRefusal),
    Frame(crate::frame::FrameRefusal),
}

/// A phase difference in turns, reduced to `[−½, ½]` and read as its size.
pub(super) fn reduced(d: &Q) -> Q {
    let half = Q::new(1.into(), 2.into());
    let within = d - d.floor();
    match within > half {
        true => Q::one() - within,
        false => within,
    }
}
