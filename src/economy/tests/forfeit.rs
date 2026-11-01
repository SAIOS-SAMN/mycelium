// 78.129, MU-DERIVATIONS D2.1–D2.2 — the forfeit of a falsified fold: the
// bond cut against the exposure, the bounty the defect against the exposure,
// every amount exact.

use crate::core::q::Q;
use crate::economy::{forfeit, Mu};
use num_bigint::BigInt;

fn q(n: i64, d: i64) -> Q {
    Q::new(BigInt::from(n), BigInt::from(d))
}

fn mu(l: [(i64, i64); 4]) -> Mu {
    Mu::of(l.map(|(n, d)| q(n, d))).expect("non-negative")
}

/// 78.129'S HALF IS THE CASE WHERE THE EXPOSURE EQUALS THE BOND: B = X cuts
/// B/2 on the lane, exactly — an odd bond's half included.
#[test]
fn slash_is_half_where_exposure_equals_bond() {
    let bond = Mu::counts([7, 4, 0, 0]);
    let f = forfeit(&bond, &bond, &bond);
    assert_eq!(f.cut, bond.part(2).expect("halves"));
}

/// A LIE NEVER FORFEITS MORE THAN WAS BONDED NOR MORE THAN IT EXPOSED: the
/// cut is half the harmonic mean, below both on every lane; a small exposure
/// on a deep bond cuts about the exposure, a large one nearly the bond.
#[test]
fn slash_never_exceeds_bond_or_exposure() {
    let bond = Mu::counts([1_000, 1, 9, 5]);
    let exposure = Mu::counts([1, 1_000, 9, 2]);
    let f = forfeit(&bond, &exposure, &exposure);
    assert!(f.cut.shortfall(&bond).is_zero(), "never past the bond: {}", f.cut);
    assert!(f.cut.shortfall(&exposure).is_zero(), "never past the exposure: {}", f.cut);
    assert_eq!(f.cut, mu([(1_000, 1_001), (1_000, 1_001), (9, 2), (10, 7)]));
}

/// A LANE THE FOLD MOVED NOTHING ON IS NOT CUT, however deep its bond; and a
/// lane with neither bond nor exposure reads nothing.
#[test]
fn slash_is_zero_on_unexposed_lane() {
    let f = forfeit(&Mu::counts([8, 8, 0, 3]), &Mu::counts([0, 2, 0, 0]), &Mu::counts([0, 1, 0, 0]));
    assert_eq!(f.cut, mu([(0, 1), (8, 5), (0, 1), (0, 1)]));
    assert_eq!(f.kept, mu([(8, 1), (32, 5), (0, 1), (3, 1)]));
}

/// A DEFAULT PROVES ITS WHOLE EXPOSURE (D := X), SO IT PAYS HALF THE CUT.
#[test]
fn bounty_is_half_on_default() {
    let exposure = Mu::counts([3, 0, 5, 0]);
    let f = forfeit(&Mu::counts([6, 2, 5, 0]), &exposure, &exposure);
    assert_eq!(f.bounty, f.cut.part(2).expect("halves"));
    assert_eq!(f.retired, f.bounty, "the other half retired");
}

/// THE BOUNTY READS THE DEFECT AGAINST THE EXPOSURE: a lie past the exposure
/// pays more than half the cut, a small defect on a large fold little.
#[test]
fn bounty_reads_defect_against_exposure() {
    let bond = Mu::counts([10, 10, 0, 0]);
    let exposure = Mu::counts([10, 10, 0, 0]);
    let f = forfeit(&bond, &exposure, &Mu::counts([30, 1, 0, 0]));
    // The cut is 5 on each lane; S proved 30 against 10, G 1 against 10.
    assert_eq!(f.cut, Mu::counts([5, 5, 0, 0]));
    assert_eq!(f.bounty, mu([(15, 4), (5, 11), (0, 1), (0, 1)]));
}

/// THE CUT DIVIDES WHOLE: bounty and retired are the cut, and what stays
/// bonded with the cut is the bond, on every lane, exactly.
#[test]
fn bounty_plus_retired_equals_cut() {
    let bond = mu([(7, 1), (5, 3), (1, 2), (0, 1)]);
    let f = forfeit(&bond, &Mu::counts([5, 1, 4, 3]), &Mu::counts([2, 1, 0, 3]));
    assert_eq!(f.bounty.add(&f.retired), f.cut);
    assert_eq!(f.kept.add(&f.cut), bond, "slash conserves the bond");
}
