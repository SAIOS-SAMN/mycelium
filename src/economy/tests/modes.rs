// 78.162 — μ AS MODES (the inverse-Sisyphus protocol): what a motion moved,
// held mode by mode with exact amounts, composed by its coordinates, drawn
// whole or refused naming the mode, and carried whole on the wire.

use crate::economy::modes::{Mode, Modes, G, H, T};
use crate::transducer::cga::{self, Mv, E1, E2};
use num_rational::BigRational;

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(n.into(), d.into())
}

fn stroke() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 16)
}

fn modes(m: &Mv) -> Modes {
    Modes::of_motion(m, &q(1, 1), &mut stroke()).expect("the motion's modes read")
}

/// The 90° turn of the `e₁e₂` plane, as its rotor ray.
fn quarter() -> Mv {
    Mv::scalar(q(1, 1)).add(&Mv::blade(E1 | E2, q(1, 1))).widened(3)
}

fn slide(x: i64, y: i64) -> Mv {
    cga::translator_at(&[q(x, 1), q(y, 1), q(0, 1)])
}

/// A turn is held on H as its exact fraction of a full turn, and two turns
/// of 90° are one turn of 180° on the same plane.
#[test]
fn of_motion_holds_turn_on_h_as_exact_fraction() {
    let one = modes(&quarter());
    let [(m, a)] = one.modes().collect::<Vec<_>>()[..] else { panic!("one mode: {}", one.print()) };
    assert_eq!(m.lane(), H);
    assert!(*a == q(1, 4) || *a == q(3, 4), "a quarter of 360°, on one side or the other: {a}");
    let mut twice = one.clone();
    twice.add(&one);
    let half = modes(&quarter().geo(&quarter()));
    let both: Vec<_> = twice.modes().map(|(m, a)| (m.clone(), a.clone())).collect();
    assert_eq!(both.len(), 1, "one plane, one mode");
    assert_eq!(half.amount_of(&both[0].0), Some(&q(1, 2)), "the turn's square is a half turn on the same plane");
}

/// A slide is held on T by its count, and its reverse is another mode.
#[test]
fn of_motion_holds_slide_on_t_by_count() {
    let two = modes(&slide(2, 0));
    let [(m, a)] = two.modes().collect::<Vec<_>>()[..] else { panic!("one mode: {}", two.print()) };
    assert_eq!(m.lane(), T);
    let one = modes(&slide(1, 0));
    let mut held = one.clone();
    held.add(&one);
    assert_eq!(held, two, "a slide by one, twice, is the slide by two: one mode, the counts added");
    // The count is read against the canonical generator: the slide by two
    // counts twice the slide by one, exactly.
    let unit = one.modes().map(|(_, a)| a.clone()).next().expect("a count");
    assert_eq!(*a, unit * q(2, 1));
    let back = modes(&slide(-2, 0));
    assert_ne!(back, two, "the reverse slide runs the other side: another mode, never a negative amount");
}

/// A draw takes whole or refuses, naming the mode and both amounts.
#[test]
fn take_draws_whole_or_refuses_naming_mode() {
    let mut held = modes(&slide(3, 0));
    held.add(&Modes::of_gauge(&[(1, 2), (2, 1)], &q(1, 1)).expect("a swap is a map"));
    let ask_too_much = {
        let mut a = modes(&slide(1, 0));
        a.add(&modes(&slide(4, 0)));
        a
    };
    let before = held.clone();
    let unit = modes(&slide(1, 0)).modes().map(|(_, a)| a.clone()).next().expect("a count");
    let short = held.take(&ask_too_much).expect_err("five along the axis is more than three");
    assert_eq!(
        (short.held.clone(), short.asked.clone()),
        (unit.clone() * q(3, 1), unit.clone() * q(5, 1)),
        "the mode short, named with both amounts"
    );
    assert_eq!(short.mode.lane(), T);
    assert_eq!(held, before, "a refused draw takes nothing");
    held.take(&modes(&slide(2, 0))).expect("two of three");
    assert_eq!(held.on_lane(T).map(|(_, a)| a.clone()).collect::<Vec<_>>(), vec![unit], "one along the axis is left");
    assert_eq!(held.on_lane(G).count(), 1, "the hue swap is untouched");
}

/// A holding travels the wire whole, and a zero amount does not read.
#[test]
fn modes_round_trip_and_refuse_zero_amount() {
    let mut held = modes(&slide(2, 1));
    held.add(&modes(&quarter()));
    held.add(&Modes::of_gauge(&[(3, 0)], &q(5, 2)).expect("a merge is a map"));
    let mut b = Vec::new();
    held.put(&mut b).expect("it writes");
    let mut p = 0usize;
    assert_eq!(Modes::take_from(&b, &mut p).expect("it reads"), held);
    assert_eq!(p, b.len());
    // A hand-written amount of zero is no holding.
    let mut zero = Vec::new();
    zero.extend_from_slice(&1u32.to_le_bytes());
    zero.extend_from_slice(&[G, 0, 4, 0]);
    crate::transducer::alphabet::put_q(&mut zero, &q(0, 1));
    assert!(Modes::take_from(&zero, &mut 0usize).is_none(), "an amount of zero refuses");
    // The identity moves nothing: no mode at all, never a zero.
    assert!(modes(&Mv::scalar(q(1, 1))).is_empty());
    let _ = Mode::Hue(crate::transducer::sentence::presentation::GaugeMode::Move(3, 0)).key().expect("a move has coordinates");
}
