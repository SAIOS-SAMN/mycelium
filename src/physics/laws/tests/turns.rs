// MU-HOSTING §5a.10, MU-SENSING §3.7 — the conformal law on a motion stated
// as turns: admitted by turn arithmetic, composed by addition with the
// winding kept, and refused by name where a turn leaves the frame's periods.

use super::super::*;
use crate::transducer::family::transduce::{Enclosure, Turn};

/// The `6 × 4` frame: two phases, of periods 6 and 4.
fn frame() -> Enclosure {
    Enclosure::new(vec![6, 4]).expect("two periods")
}

fn motion(turns: &[(usize, usize)]) -> TurnMotion {
    TurnMotion { turns: turns.iter().map(|&(r, h)| Turn::of(r, h)).collect() }
}

fn place(turns: &[(usize, usize)]) -> Vec<Turn> {
    turns.iter().map(|&(r, h)| Turn::of(r, h)).collect()
}

/// THE CONFORMAL LAW ACCEPTS A MOTION STATED AS TURNS: on the `6 × 4` frame
/// the motion `(1/3, 1/4)` lands on both periods, and carries the place
/// `(0, 1/4)` onto `(1/3, 1/2)` and `(5/6, 3/4)` across both seams onto
/// `(1/6, 0)`. The turn `1/3` has no rotor over ℚ (Niven): it is admitted
/// by its arithmetic in ℚ/ℤ alone, the same as the quarter turn beside it.
#[test]
fn conformal_accepts_turn_valued_motion() {
    let t = TurnTransition {
        frame: frame(),
        motion: motion(&[(1, 3), (1, 4)]),
        bodies: vec![
            (place(&[(0, 6), (1, 4)]), place(&[(2, 6), (2, 4)])),
            (place(&[(5, 6), (3, 4)]), place(&[(1, 6), (0, 4)])),
        ],
    };
    let v = verify_turns(&t);
    assert!(v.admitted(), "{:?}", v.breaches);
}

/// A PLACE THE MOTION DOES NOT CARRY IS NAMED WITH ITS RESIDUE: the place
/// `(0, 1/4)` claimed onto `(1/2, 1/2)` under `(1/3, 1/4)` leaves the
/// residue `1/2 − 1/3 = 1/6` on the first phase and a whole turn on the
/// second, held as turns and never as a flag.
#[test]
fn verify_turns_names_residue_of_uncarried_place() {
    let t = TurnTransition {
        frame: frame(),
        motion: motion(&[(1, 3), (1, 4)]),
        bodies: vec![(place(&[(0, 6), (1, 4)]), place(&[(3, 6), (2, 4)]))],
    };
    let v = verify_turns(&t);
    let [(law, TurnBreach::NotCarried { body: 0, residue })] = v.breaches.as_slice() else {
        panic!("one uncarried place: {:?}", v.breaches)
    };
    assert_eq!(*law, Law::Conformal);
    assert_eq!(residue.as_slice(), [Turn::of(1, 6), Turn::of(0, 1)].as_slice());
    assert!(!residue[0].is_whole() && residue[1].is_whole());
}

/// A TURN MOTION COMPOSES BY ADDITION, phase by phase, its winding kept:
/// `(1/3, 3/4)` then `(2/3, 1/2)` is `(1, 5/4)`, never `(0, 1/4)`; carrying
/// a place by the one and then the other is carrying it by the composition;
/// the composition commutes and associates; and a motion composed with its
/// inverse is whole on every phase.
#[test]
fn turn_motion_composes_by_addition() {
    let a = motion(&[(1, 3), (3, 4)]);
    let b = motion(&[(2, 3), (1, 2)]);
    let c = motion(&[(1, 6), (1, 4)]);
    let ab = a.then(&b).expect("one rank");
    assert_eq!(ab, motion(&[(1, 1), (5, 4)]));
    assert_ne!(ab, motion(&[(0, 1), (1, 4)]), "the whole turns are kept");
    let p = place(&[(5, 6), (1, 4)]);
    let stepwise = b.carry(&a.carry(&p).expect("one rank")).expect("one rank");
    assert_eq!(ab.carry(&p).expect("one rank"), stepwise);
    assert_eq!(b.then(&a).expect("one rank"), ab);
    assert_eq!(ab.then(&c).expect("one rank"), a.then(&b.then(&c).expect("one rank")).expect("one rank"));
    let inverse = motion(&[(2, 3), (1, 4)]);
    assert!(a.then(&inverse).expect("one rank").turns.iter().all(Turn::is_whole));
}

/// COMPOSED TURNS KEEP THEIR WINDING (78.171 amendment (c)): a fifth of a
/// turn (72°) composed five times is `5/5`, one whole turn, `K = 1` and
/// phase 0, never read back as 0; three quarters then three quarters is
/// `6/4`, `K = 1` with the phase `1/2`; `K = ⌊θ/360⌋` for `θ = 360·r/h`
/// degrees, read exactly. A place carried across the seam is still
/// admitted against its claimed phase, the residue a phase.
#[test]
fn composed_turns_keep_their_winding() {
    let fifth = motion(&[(1, 5)]);
    let five = (1..5).try_fold(fifth.clone(), |m, _| m.then(&fifth)).expect("one rank");
    assert_eq!(five, motion(&[(5, 5)]));
    assert_eq!(five.turns[0].winding(), Some(1));
    assert_eq!(five.turns[0].phase(), Some(Turn::of(0, 5)));
    assert!(five.turns[0].is_whole() && five.turns[0] != Turn::of(0, 1), "a whole turn, held as one");
    let three = motion(&[(3, 4)]);
    let six = three.then(&three).expect("one rank");
    assert_eq!(six.turns[0], Turn::of(6, 4));
    assert_eq!((six.turns[0].winding(), six.turns[0].phase()), (Some(1), Some(Turn::of(1, 2))));
    let ten = (1..10).try_fold(fifth.clone(), |m, _| m.then(&fifth)).expect("one rank");
    assert_eq!(ten.turns[0].winding(), Some(2), "720°, K = 2");
    assert_eq!(Turn::of(1, 3).winding(), Some(0));
    assert_eq!(Turn::of(1, 0).winding(), None, "a period of 0 holds no turn");
    let t = TurnTransition {
        frame: Enclosure::new(vec![5]).expect("one period"),
        motion: five,
        bodies: vec![(place(&[(2, 5)]), place(&[(2, 5)]))],
    };
    assert!(verify_turns(&t).admitted(), "a whole turn carries a place onto itself");
}

/// A TURN THAT LEAVES THE FRAME'S PERIODS IS REFUSED BY NAME: the quarter
/// turn on the phase of period 6 lands at `6/4` of it, no site
/// (`LeavesPeriods`), while the quarter turn on the phase of period 4 lands.
/// A motion stating three turns on the frame's two phases is refused
/// `Rank`, and a place of one turn `PlaceRank`.
#[test]
fn verify_turns_refuses_turn_leaving_frame_periods() {
    let off = TurnTransition { frame: frame(), motion: motion(&[(1, 4), (1, 4)]), bodies: Vec::new() };
    assert_eq!(
        verify_turns(&off).breaches,
        vec![(Law::Conformal, TurnBreach::LeavesPeriods { phase: 0, turn: Turn::of(1, 4), period: 6 })]
    );
    let three = TurnTransition { frame: frame(), motion: motion(&[(1, 3), (1, 4), (1, 2)]), bodies: Vec::new() };
    assert_eq!(verify_turns(&three).breaches, vec![(Law::Conformal, TurnBreach::Rank { stated: 3, phases: 2 })]);
    let short = TurnTransition {
        frame: frame(),
        motion: motion(&[(1, 3), (1, 4)]),
        bodies: vec![(place(&[(0, 6)]), place(&[(2, 6), (1, 4)]))],
    };
    assert_eq!(verify_turns(&short).breaches, vec![(Law::Conformal, TurnBreach::PlaceRank { body: 0, stated: 1 })]);
}
