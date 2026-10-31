// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! POWV — PROOF OF WORK VERIFICATION: what the hyphae that verified a
//! closure take of the mint it releases (MU-ECONOMY §5.1).
//!
//! A closure is paid only after replay: the home measures its discharge, and
//! on an install a witness re-derives the same vertex. That replay is work
//! the closure cannot be paid without. Its share is read per lane off the
//! verifying court's own metabolism — the strain `y_l` its yield surface
//! reads on the lane (reserve deficit plus entropic excess) against its
//! calibrated reserve `R`:
//!
//! ```text
//! v_l = ⌊ M_l · y_l / (y_l + R) ⌋        each of n organs takes ⌊ v_l / n ⌋
//! ```
//!
//! It is taken OUT of the mint `M`, never added to it: total issuance is the
//! closure's measured discharge as before, and supply still tracks the
//! language. A court at rest (`y_l = 0`) takes nothing; a strained lane pays
//! verification more, up to half the lane at a full deficit and more where
//! torsion outruns syntropy — the scarce resource is paid where it is
//! scarce. No constant enters: the fraction is a ratio of two readings of
//! the court's own physics, and the split among organs is equal because
//! each replayed the same closure. What no organ can take whole stays with
//! the closer.

/// THE VERIFICATION SHARE EACH ORGAN TAKES of the mint `mint`, per lane:
/// `⌊ ⌊M_l · y_l / (y_l + R)⌋ / n ⌋` for strain `strain` (`y`), calibrated
/// reserve `capacity` (`R`) and `organs` verifying courts (`n`). Zero on
/// every lane where no organ verified, where the court is at rest on the
/// lane, or where `y_l + R` is zero. Exact: computed in `u128`, and the
/// share never exceeds `M_l / (2n)` unless the lane's entropic excess
/// carries it past the deficit.
pub fn verification_share(mint: &super::Mu, strain: &[u64; 4], capacity: u64, organs: usize) -> super::Mu {
    use crate::core::q::Q;
    use num_bigint::BigInt;
    let r = BigInt::from(capacity);
    let lanes: [Q; 4] = std::array::from_fn(|l| {
        let y = BigInt::from(strain[l]);
        let whole = &y + &r;
        if num_traits::Zero::is_zero(&whole) {
            return Q::default();
        }
        (mint.lane(l) * Q::new(y, whole)).floor()
    });
    let Ok(v) = super::Mu::of(lanes) else { return super::Mu::zero() };
    match v.part(organs) {
        Some(each) => each.floor(),
        None => super::Mu::zero(),
    }
}
