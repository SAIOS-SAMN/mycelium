// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// 78.184 §3, MU-EQUATIONS §6.13 (ruled 2026-11-13) — THE ELEMENTAL ROW: the
// μ one nucleus creates, read off its universe's stated `(Z, Q)` as one
// coupled nucleon dipole (LAW-COUPLING v2, 78.181).
//
//! Z is coupled to Q three ways:
//! - ALGEBRAICALLY, the dipole's push `p = (Z − Q)/(Z + Q)`, a rate in
//!   `(−1, 1)` composed by the flow law: its dilation `D(p) = Z/Q` is one
//!   boost, on S;
//! - GEOMETRICALLY, one completed rotation of the dipole's axis plane, a
//!   full turn (`K = 1`), on H. The completion is the full turn itself, so
//!   it exists for every nucleus;
//! - INTRINSICALLY, the rest phase `R = −p`: the dipole rests where its push
//!   balances its phase, so `p`'s sign is the side the dipole stands on, and
//!   it shows as the boost's direction (`D(Z/Q)` against `D(Q/Z)`). A
//!   completed turn is one whole turn on either side, so the turn's own
//!   mode carries no side (the modes reader holds a turn's ray canonical).
//!
//! The row is weighted by the nucleus's whole `3W = 3(Z + Q)`. It is a lane
//! vector of modes (MU-ECONOMY §1), read by the machinery every motion's
//! modes are read by (`Modes::of_motion`), never four numbers.
//!
//! The composed motion `D(p)·D(R)` was first proposed (§6.13 as first
//! written) and is the identity at the rest point (`D(p ⊕ R) = D(0)`), so it
//! moved nothing; the ruling reads the boost and the completed turn apart.

use super::modes::Modes;
use crate::transducer::cga::{self, Mv, E1, E2};
use num_rational::BigRational as Q;
use num_traits::{One, Signed, Zero};

/// WHY A STATED `(Z, Q)` GIVES NO ROW.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RowRefusal {
    /// `Z` is not positive: a nucleus has at least one proton.
    NoProton,
    /// `Q` is negative.
    NegativeNeutrons,
    /// The boost's or the turn's modes did not read (rule 11).
    Unread,
}

/// THE DIPOLE'S PUSH, `p = (Z − Q)/(Z + Q)`, exact in `(−1, 1]`; `1` where
/// `Q = 0` (all charge, hydrogen's bare proton).
pub fn push(z: &Q, q: &Q) -> Option<Q> {
    let whole = z + q;
    (!whole.is_zero()).then(|| (z - q) / whole)
}

/// THE ELEMENTAL ROW of one nucleus of `Z` protons and `Q` neutrons: `3W`
/// times the boost `D(Z/Q)` on S (none at the throat `Z = Q`, and none
/// where `Q = 0`, whose dilation is unbounded) and one full turn of the
/// dipole's axis plane on H. The rest phase `R = −p` gives the side, which
/// the boost's direction carries.
pub fn row(z: &Q, q: &Q, budget: &mut crate::physics::Budget) -> Result<Modes, RowRefusal> {
    if !z.is_positive() {
        return Err(RowRefusal::NoProton);
    }
    if q.is_negative() {
        return Err(RowRefusal::NegativeNeutrons);
    }
    let whole = Q::from_integer(3.into()) * (z + q);
    let p = push(z, q).ok_or(RowRefusal::Unread)?;
    let mut out = Modes::new();
    if !q.is_zero() {
        let boost = Modes::of_motion(&cga::dilator(&(z / q)), &whole, budget).ok_or(RowRefusal::Unread)?;
        out.add(&boost);
    }
    // The push's whole reading is the boost's direction; it is read here
    // only to refuse a dipole with no whole (rule 11).
    let _ = p;
    // The turn's mode is read off a quarter turn of the axis plane and held
    // at one whole turn: the completion.
    let quarter: Mv = Mv::scalar(Q::one()).add(&Mv::blade(E1 | E2, Q::one())).widened(3);
    let read = Modes::of_motion(&quarter, &Q::one(), budget).ok_or(RowRefusal::Unread)?;
    let turn = read.modes().next().map(|(m, _)| m.clone()).ok_or(RowRefusal::Unread)?;
    out.add_mode(turn, whole);
    Ok(out)
}
