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

/// THE SHARE IS EXACT, NEVER FLOORED: a mint of 7 on S at strain 1 against
/// a reserve of 2, three organs, is 7·1/((1 + 2)·3) = 7/9 each — where the
/// floor read ⌊⌊7/3⌋/3⌋ = 0 and paid verification nothing — and the closer
/// keeps exactly 7 − 3·7/9 = 14/3.
#[test]
fn powv_share_is_exact() {
    use crate::core::q::Q;
    use num_bigint::BigInt;
    let mint = Mu::counts([7, 0, 4, 0]);
    let each = verification_share(&mint, &[1, 0, 2, 0], 2, 3);
    let q = |n: i64, d: i64| Q::new(BigInt::from(n), BigInt::from(d));
    assert_eq!(each, Mu::of([q(7, 9), q(0, 1), q(4, 6), q(0, 1)]).expect("non-negative"));
    let organs = Mu::sum([&each, &each, &each]);
    assert_eq!(mint.draw(&organs), Mu::of([q(14, 3), q(0, 1), q(2, 1), q(0, 1)]), "the closer the exact rest");
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

/// MU-ECONOMY §5.1 — UNDER MODES THE LANE'S FACTOR SCALES EACH MODE ON IT. A
/// mint of 6 on S held as a symbol's creation (4) and its erasure (2), at a
/// strain equal to the reserve with two organs, pays each organ 6·100/200/2
/// = 3/2 of S as 1 of the creation and 1/2 of the erasure: the re-traversed
/// closure's modes, never the founding mode. The closer keeps the same
/// modes in the same proportion, 2 and 1.
#[test]
fn verification_share_scales_each_mode_on_its_lane() {
    use crate::core::q::Q;
    use crate::economy::modes::{Mode, Modes, S};
    use num_bigint::BigInt;
    let q = |n: i64, d: i64| Q::new(BigInt::from(n), BigInt::from(d));
    let create = Mode::SymbolCreation { from_ground: 0, to_symbol: 4, forward: true };
    let erase = Mode::SymbolCreation { from_ground: 0, to_symbol: 4, forward: false };
    let mut held = Modes::new();
    held.add_mode(create.clone(), q(4, 1));
    held.add_mode(erase.clone(), q(2, 1));
    let mint = Mu::of_modes(held);
    let each = verification_share(&mint, &[100, 0, 0, 0], 100, 2);
    assert_eq!(each.modes().amount_of(&create), Some(&q(1, 1)), "the creation scaled by the lane's factor");
    assert_eq!(each.modes().amount_of(&erase), Some(&q(1, 2)), "the erasure scaled by the same factor");
    let founding = Mode::founding(S).expect("S has a founding mode");
    assert!(each.modes().amount_of(&founding).is_none(), "never the founding mode");
    let rest = mint.draw(&each.add(&each)).expect("the shares fit the mint");
    assert_eq!(rest.modes().amount_of(&create), Some(&q(2, 1)), "the closer keeps the creation's rest");
    assert_eq!(rest.modes().amount_of(&erase), Some(&q(1, 1)), "and the erasure's");
}
