use crate::economy::verify::*;
use crate::economy::Mu;

/// A COURT AT REST TAKES NOTHING: with no strain on any lane, the whole
/// mint stays the closer's.
#[test]
fn verification_share_is_zero_at_rest() {
    assert_eq!(verification_share(&Mu::counts([40, 12, 0, 9]), &[0; 4], 100, 2), [0; 4]);
}

/// THE SHARE IS READ PER LANE OFF THE STRAIN: a lane at full deficit
/// (`y = R`) pays verification half the lane, split between two organs;
/// a lane at a quarter of it pays a fifth; a lane the closure did not
/// move pays nothing whatever its strain.
#[test]
fn verification_share_reads_strain_against_reserve_per_lane() {
    let share = verification_share(&Mu::counts([40, 50, 0, 8]), &[100, 25, 100, 0], 100, 2);
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
    for n in 1..=3usize {
        let each = verification_share(&Mu::counts(mint), &strain, 4, n);
        let all = Mu::sum(std::iter::repeat_n(&each, n));
        assert!(all.shortfall(&Mu::counts(mint)).is_zero(), "{n} organs take at most the mint, every lane");
    }
    assert_eq!(verification_share(&Mu::counts(mint), &strain, 4, 0), [0; 4], "no organ, no share");
}
