// 78.184 §3, MU-EQUATIONS §6.13 — the elemental row of a coupled nucleon dipole.

use crate::economy::elemental::{push, row, RowRefusal};
use crate::economy::modes::{H, S};
use num_rational::BigRational as Q;

fn n(v: i64) -> Q {
    Q::from_integer(v.into())
}

fn stroke() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 16)
}

/// The amounts a row holds on one lane, in mode order.
fn on(lane: u8, m: &crate::economy::modes::Modes) -> Vec<Q> {
    m.modes().filter(|(mode, _)| mode.lane() == lane).map(|(_, a)| a.clone()).collect()
}

/// THE THROAT: a nucleus with as many neutrons as protons has no push, so no
/// boost on S, and its one completed turn on H weighs its whole `3W`.
#[test]
fn throat_nucleus_holds_one_full_turn_weighted_by_its_whole() {
    let r = row(&n(6), &n(6), &mut stroke()).expect("a row");
    assert_eq!(push(&n(6), &n(6)), Some(n(0)));
    assert!(on(S, &r).is_empty(), "no boost at the throat");
    assert_eq!(on(H, &r), vec![n(36)], "one full turn, weighed by 3W = 3(6 + 6)");
}

/// OFF THE THROAT: the boost `D(Z/Q)` stands on S beside the completed turn,
/// both weighed by `3W`; Z and Q are read as one coupled body, so swapping
/// them reverses the push, and the boost stands the other way.
#[test]
fn off_throat_nucleus_boosts_on_s_and_turns_on_its_rest_side() {
    let heavy = row(&n(26), &n(30), &mut stroke()).expect("a row");
    assert_eq!(push(&n(26), &n(30)), Some(Q::new((-4).into(), 56.into())));
    assert_eq!(on(S, &heavy), vec![n(168)], "one boost, weighed by 3W = 168");
    assert_eq!(on(H, &heavy), vec![n(168)], "one full turn, weighed by 3W");
    let swapped = row(&n(30), &n(26), &mut stroke()).expect("a row");
    let boost = |m: &crate::economy::modes::Modes| m.modes().find(|(mode, _)| mode.lane() == S).map(|(mode, _)| mode.clone());
    assert_ne!(boost(&heavy), boost(&swapped), "D(Z/Q) and D(Q/Z) are boosts the opposite way");
    assert_ne!(heavy, swapped, "Z and Q are not interchangeable counts");
}

/// THE WHOLE SCALES THE ROW: a heavier nucleus of the same push creates
/// proportionally more, exactly.
#[test]
fn row_scales_with_the_nucleon_whole() {
    let one = row(&n(2), &n(3), &mut stroke()).expect("a row");
    let two = row(&n(4), &n(6), &mut stroke()).expect("a row");
    assert_eq!(on(H, &two), vec![n(30)]);
    assert_eq!(on(H, &one), vec![n(15)]);
    assert_eq!(on(S, &two).len(), 1, "the same push, one boost");
}

/// WHAT NO NUCLEUS IS is refused by name: no proton, negative neutrons. A bare
/// proton (Q = 0) has an unbounded dilation and holds the turn alone.
#[test]
fn row_refuses_what_no_nucleus_is_and_reads_the_bare_proton() {
    assert_eq!(row(&n(0), &n(1), &mut stroke()), Err(RowRefusal::NoProton));
    assert_eq!(row(&n(1), &n(-1), &mut stroke()), Err(RowRefusal::NegativeNeutrons));
    let h = row(&n(1), &n(0), &mut stroke()).expect("hydrogen's bare proton");
    assert!(on(S, &h).is_empty());
    assert_eq!(on(H, &h), vec![n(3)]);
}
