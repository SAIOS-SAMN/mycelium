// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! POWV — PROOF OF WORK VERIFICATION: what the hyphae that verified a
//! closure take of the mint it releases (MU-ECONOMY §5.1).
//!
//! A closure is paid only after reconstruction: the home measures its discharge, and
//! on an install a witness re-derives the same vertex. That reconstruction is work
//! the closure cannot be paid without. Its share is read per lane off the
//! verifying court's own metabolism — the strain `y_l` its yield surface
//! reads on the lane (reserve deficit plus entropic excess) against its
//! calibrated reserve `R`:
//!
//! ```text
//! v_l = M_l · y_l / ((y_l + R) · n)        each of n organs, exact over ℚ
//! ```
//!
//! It is taken OUT of the mint `M`, never added to it: total issuance is the
//! closure's measured discharge as before, and supply still tracks the
//! language. A court at rest (`y_l = 0`) takes nothing; a strained lane pays
//! verification more, up to half the lane at a full deficit and more where
//! torsion outruns syntropy — the scarce resource is paid where it is
//! scarce. No constant enters: the fraction is a ratio of two readings of
//! the court's own physics, and the split among organs is equal because
//! each reconstructed the same closure. The share is a rational part of the
//! mint, never floored: the closer takes exactly `M − n · v`.

use super::modes::Modes;
use super::Mu;
use crate::core::q::Q;
use num_bigint::BigInt;

/// THE VERIFICATION SHARE EACH ORGAN TAKES of the mint `mint`, per lane:
/// `M_l · y_l / ((y_l + R) · n)` exactly, for strain `strain` (`y`),
/// the process's sealed `capacity` (`R`, D2.5) and `organs` verifying
/// courts (`n`).
/// Zero on every lane where no organ verified, where the court is at rest
/// on the lane, or where `y_l + R` is zero. The organs together take
/// `M_l · y_l / (y_l + R) ≤ M_l`, at most half the lane unless the lane's
/// entropic excess carries the strain past the reserve.
pub fn verification_share(mint: &Mu, strain: &[u64; 4], capacity: u64, organs: usize) -> Mu {
    let Some(f) = factors(strain, capacity, organs) else { return Mu::zero() };
    // Every factor is non-negative, so the lanes are.
    match Mu::of(std::array::from_fn(|l| mint.lane(l) * &f[l])) {
        Ok(v) => v,
        Err(_) => Mu::zero(),
    }
}

/// THE VERIFICATION SHARE EACH ORGAN TAKES OF A MINT'S MODES: the lane's
/// factor `y_l / ((y_l + R) · n)` scales each mode on lane `l` (MU-ECONOMY
/// §5.1, "under modes the same factor scales each mode on lane `l`").
/// Nothing where no organ verified.
pub fn verification_modes(mint: &Modes, strain: &[u64; 4], capacity: u64, organs: usize) -> Modes {
    match factors(strain, capacity, organs) {
        Some(f) => mint.scaled_lanes(&f),
        None => Modes::new(),
    }
}

/// Each lane's factor `y_l / ((y_l + R) · n)`, zero where `y_l + R` is
/// zero; `None` for no organs.
fn factors(strain: &[u64; 4], capacity: u64, organs: usize) -> Option<[Q; 4]> {
    if organs == 0 {
        return None;
    }
    let r = BigInt::from(capacity);
    let n = BigInt::from(organs);
    Some(std::array::from_fn(|l| {
        let y = BigInt::from(strain[l]);
        let whole = (&y + &r) * &n;
        match num_traits::Zero::is_zero(&whole) {
            true => Q::default(),
            false => Q::new(y, whole),
        }
    }))
}
