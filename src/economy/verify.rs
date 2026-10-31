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
pub fn verification_share(mint: &[u64; 4], strain: &[u64; 4], capacity: u64, organs: usize) -> [u64; 4] {
    let Ok(n) = u128::try_from(organs) else { return [0; 4] };
    if n == 0 {
        return [0; 4];
    }
    std::array::from_fn(|l| {
        let (m, y, r) = (u128::from(mint[l]), u128::from(strain[l]), u128::from(capacity));
        let Some(whole) = y.checked_add(r).filter(|&d| d > 0) else { return 0 };
        let Some(v) = m.checked_mul(y).map(|p| p / whole) else { return 0 };
        // v ≤ m ≤ u64::MAX, so the share fits its lane; a share that did
        // not would be no share, never a clipped one.
        match u64::try_from(v / n) {
            Ok(s) => s,
            Err(_) => 0,
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A COURT AT REST TAKES NOTHING: with no strain on any lane, the whole
    /// mint stays the closer's.
    #[test]
    fn verification_share_is_zero_at_rest() {
        assert_eq!(verification_share(&[40, 12, 0, 9], &[0; 4], 100, 2), [0; 4]);
    }

    /// THE SHARE IS READ PER LANE OFF THE STRAIN: a lane at full deficit
    /// (`y = R`) pays verification half the lane, split between two organs;
    /// a lane at a quarter of it pays a fifth; a lane the closure did not
    /// move pays nothing whatever its strain.
    #[test]
    fn verification_share_reads_strain_against_reserve_per_lane() {
        let share = verification_share(&[40, 50, 0, 8], &[100, 25, 100, 0], 100, 2);
        // Lane S: 40·100/200 = 20, two organs 10 each.
        // Lane G: 50·25/125 = 10, two organs 5 each.
        // Lane T: nothing minted, nothing shared.
        // Lane H: no strain, nothing shared.
        assert_eq!(share, [10, 5, 0, 0]);
    }

    /// ONE ORGAN TAKES THE WHOLE VERIFICATION, AND NO SHARE PASSES WHAT IS
    /// MINTED: the organs' take never exceeds the mint on any lane, and what
    /// does not divide stays with the closer.
    #[test]
    fn verification_share_never_exceeds_mint() {
        let mint = [7, 1, 3, 5];
        let strain = [u64::MAX, 3, 1, 9];
        for n in 1..=3u64 {
            let each = verification_share(&mint, &strain, 4, usize::try_from(n).expect("small"));
            for l in 0..4 {
                assert!(each[l] * n <= mint[l], "lane {l}, {n} organs");
            }
        }
        assert_eq!(verification_share(&mint, &strain, 4, 0), [0; 4], "no organ, no share");
    }
}
