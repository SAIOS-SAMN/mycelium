// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 78.129, MU-DERIVATIONS D2.1 and D2.2 (ruled 2026-10-31) — THE FORFEIT OF
//! A FALSIFIED FOLD: what its tier's bond loses, what the challenger is paid
//! and what is retired, each read off the readings the court holds at the
//! falsification and nothing chosen.
//!
//! - `B_l`, the tier's bond on lane `l`;
//! - `X_l`, the EXPOSURE: the value the falsified fold's commitment moved
//!   on the lane, read before reversal (`Court::exposure_of`);
//! - `D_l`, the DEFECT: the fold's measured lie on the lane, the
//!   reconstruction's own residue (`challenge::defect`); a default publishes
//!   nothing, so its defect is its whole exposure.
//!
//! ```text
//! cut_l     = B_l · X_l / (B_l + X_l)        0 where B_l + X_l = 0
//! bounty_l  = cut_l · D_l / (D_l + X_l)      0 where D_l + X_l = 0
//! retired_l = cut_l − bounty_l
//! kept_l    = B_l − cut_l
//! ```
//!
//! The cut is half the harmonic mean of the bond and the exposure: never more
//! than either, about the exposure where the bond is deep, nearly the whole
//! bond where the exposure passes it, and exactly half the bond where the two
//! are equal (78.129's half). The bounty pays the challenger the share of
//! the cut its proof of the defect bears against the exposure: half on a
//! default, more where a reconstruction proves a lie past the exposure, little
//! for a small defect on a large fold. Every amount is exact over ℚ, and
//! `kept + bounty + retired = B` on every lane.

use super::Mu;
use crate::core::q::Q;
use num_traits::Zero;

/// THE FORFEIT, per lane: the cut from the bond, its bounty and its retired
/// part, and what stays bonded.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Forfeit {
    pub cut: Mu,
    pub bounty: Mu,
    pub retired: Mu,
    pub kept: Mu,
}

/// `a · b / (c + b)` on one lane, zero where `c + b` is zero.
fn share(a: &Q, b: &Q, c: &Q) -> Q {
    let whole = c + b;
    match whole.is_zero() {
        true => Q::zero(),
        false => a * b / whole,
    }
}

/// The forfeit of a bond `bond` on a fold of exposure `exposure` whose
/// reconstruction proved a defect `defect` (MU-DERIVATIONS D2.1, D2.2).
pub fn forfeit(bond: &Mu, exposure: &Mu, defect: &Mu) -> Forfeit {
    let lanes = |f: &dyn Fn(usize) -> Q| -> Mu {
        // Every term is a product and quotient of non-negative lanes.
        match Mu::of(std::array::from_fn(f)) {
            Ok(m) => m,
            Err(_) => Mu::zero(),
        }
    };
    let cut = lanes(&|l| share(bond.lane(l), exposure.lane(l), bond.lane(l)));
    let bounty = lanes(&|l| share(cut.lane(l), defect.lane(l), exposure.lane(l)));
    // bounty ≤ cut ≤ bond on every lane: each is its whole times a fraction
    // at most one.
    let (_, retired) = cut.split_at(&bounty);
    let (_, kept) = bond.split_at(&cut);
    Forfeit { cut, bounty, retired, kept }
}
