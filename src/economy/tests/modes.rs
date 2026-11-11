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
fn of_motion_reads_turn_on_h_as_exact_fraction() {
    let one = modes(&quarter());
    let [(m, a)] = one.modes().collect::<Vec<_>>()[..] else { panic!("one mode: {}", one.print()) };
    assert_eq!(m.lane(), H);
    assert!(*a == q(1, 4) || *a == q(3, 4), "a quarter of 360°, on one side or the other: {a}");
    let mut twice = one.clone();
    twice.add(&one);
    let half = modes(&quarter().geo(&quarter()));
    let both: Vec<_> = twice.modes().map(|(m, a)| (m.clone(), a.clone())).collect();
    assert_eq!(both.len(), 1, "one plane, one mode");
    assert_eq!(half.amount_of(&both[0].0), Some(&q(1, 2)), "the turn's square is a 180° turn on the same plane");
}

/// A slide is held on T by its count, and its reverse is another mode.
#[test]
fn of_motion_reads_slide_on_t_by_count() {
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
fn modes_round_trips_and_refuses_zero_amount() {
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

/// A MOTION'S MODES ARE READ AT THE SHARE GIVEN (78.174 §1 item 9;
/// MU-ENGINEERING §9.3): the enclosure's measure is the product of its
/// periods at every rank (5; 6; 24; 120 at ranks 1 to 4), and a slide's
/// mode amount is its unit amount times the share `m / measure`, exactly,
/// at each.
#[test]
fn of_motion_scales_by_open_dimensional_enclosure_share() {
    let slide_mv = slide(2, 0);
    for (periods, expected_measure) in [
        (vec![5usize], 5i64),
        (vec![2, 3], 6),
        (vec![2, 3, 4], 24),
        (vec![2, 3, 4, 5], 120),
    ] {
        let enclosure = crate::transducer::family::transduce::Enclosure::new(periods).expect("enclosure");
        let measure = enclosure.measure().expect("measure") as i64;
        assert_eq!(measure, expected_measure, "rank {} enclosure measure is exact product", enclosure.rank());
        let mass = 3i64;
        let share = q(mass, measure);
        let m = Modes::of_motion(&slide_mv, &share, &mut stroke()).expect("modes read");
        let [(mode, amount)] = m.modes().collect::<Vec<_>>()[..] else { panic!("one mode") };
        assert_eq!(mode.lane(), T);
        let unit_modes = modes(&slide(1, 0));
        let unit_amount = unit_modes.modes().next().expect("unit amount").1.clone();
        assert_eq!(*amount, unit_amount * q(2, 1) * share, "mode amount matches exact enclosure share at rank {}", enclosure.rank());
    }
}

/// MU-ECONOMY §2.5 — THE FOUNDING MODE IS THE LANE'S IDENTITY MOTION, held
/// as a mode, never left empty: one per base lane, apart from every motion's
/// mode on the lane, carried whole on the wire and refused past the four
/// base lanes.
#[test]
fn founding_mode_is_held_per_lane_and_round_trips() {
    use crate::economy::modes::S;
    let mut held = Modes::founding(G, q(3, 2)).expect("G is a base lane");
    held.add(&Modes::founding(H, q(1, 4)).expect("H is a base lane"));
    held.add(&modes(&quarter()));
    assert_eq!(held.amount_of(&Mode::Founding { lane: G }), Some(&q(3, 2)));
    assert_eq!(held.amount_of(&Mode::Founding { lane: H }), Some(&q(1, 4)), "the founding H mode stands apart from the turn on H");
    assert_eq!(held.on_lane(H).count(), 2);
    assert_eq!(Mode::Founding { lane: S }.lane(), S);
    let mut wire = Vec::new();
    held.put(&mut wire).expect("written");
    let mut p = 0;
    assert_eq!(Modes::take_from(&wire, &mut p), Some(held), "read back whole");
    assert_eq!(p, wire.len());
    assert_eq!(Mode::founding(4), None, "a fifth lane opens only by a ruled act");
    assert_eq!(Modes::founding(4, q(1, 1)), None);
    assert!(Modes::founding(T, q(0, 1)).expect("T is a base lane").is_empty(), "a zero amount holds nothing");
}

/// 78.180 ruling 3 — A PER-LANE DRAW TAKES FROM THE LANE'S MODES IN
/// PROPORTION, exactly, and never from another lane: the modes drawn travel
/// with the transfer, the holding keeps the rest, and a lane holding less is
/// refused whole, naming the lane's first mode with the lane's whole.
#[test]
fn draw_lane_is_proportional_exact_and_refused_whole() {
    let turn = modes(&quarter());
    let [(turn_mode, t)] = turn.modes().collect::<Vec<_>>()[..] else { panic!("one turn") };
    let (turn_mode, t) = (turn_mode.clone(), t.clone());
    let mut held = Modes::founding(H, q(1, 2)).expect("H");
    held.add(&turn);
    held.add(&Modes::founding(G, q(5, 1)).expect("G"));
    let before = held.clone();
    let whole = q(1, 2) + &t;
    let drawn = held.draw_lane(H, &(&whole / q(2, 1))).expect("half the lane");
    assert_eq!(drawn.amount_of(&Mode::Founding { lane: H }), Some(&q(1, 4)), "half the lane draws half of each mode on it");
    assert_eq!(drawn.amount_of(&turn_mode), Some(&(&t / q(2, 1))), "the turn in its own proportion");
    assert_eq!(drawn.on_lane(H).count(), 2);
    assert_eq!(drawn.on_lane(G).count(), 0, "never from another lane");
    assert_eq!(held.amount_of(&Mode::Founding { lane: H }), Some(&q(1, 4)));
    assert_eq!(held.amount_of(&Mode::Founding { lane: G }), Some(&q(5, 1)), "G untouched");
    let mut back = held.clone();
    back.add(&drawn);
    assert_eq!(back, before, "what was drawn and what was kept make the holding whole");
    let short = held.draw_lane(H, &q(2, 1)).expect_err("the lane holds half its whole");
    assert_eq!((short.held.clone(), short.asked.clone()), (&whole / q(2, 1), q(2, 1)));
    assert_eq!(short.mode.lane(), H, "named on its lane");
    assert_eq!(held.draw_lane(T, &q(1, 1)).expect_err("T holds nothing").mode, Mode::Founding { lane: T });
    assert!(held.draw_lane(H, &q(0, 1)).expect("nothing").is_empty(), "a draw of nothing draws nothing");
    assert_eq!(back.draw_lane(H, &whole).expect("whole lane").on_lane(H).count(), 2);
    assert_eq!(back.on_lane(H).count(), 0, "a whole lane drawn leaves none of its modes");
}

/// §12 item 8 — AN ASK IS READ AGAINST THE HOLDING MODE BY MODE: per lane,
/// in proportion, all lanes or none; a lane short refuses the whole ask and
/// leaves the holding as it was.
#[test]
fn draw_lanes_reads_an_ask_per_lane_all_or_none() {
    let mut held = Modes::founding(G, q(4, 1)).expect("G");
    held.add(&Modes::founding(H, q(2, 1)).expect("H"));
    let before = held.clone();
    let ask = crate::economy::Mu::of([q(0, 1), q(1, 1), q(0, 1), q(3, 1)]).expect("an ask");
    let short = held.draw_lanes(&ask).expect_err("H holds 2");
    assert_eq!((short.mode.lane(), short.held.clone(), short.asked.clone()), (H, q(2, 1), q(3, 1)));
    assert_eq!(held, before, "a refused ask moves nothing");
    let ask = crate::economy::Mu::of([q(0, 1), q(1, 1), q(0, 1), q(1, 2)]).expect("an ask");
    let drawn = held.draw_lanes(&ask).expect("both lanes hold it");
    assert_eq!(drawn.amount_of(&Mode::Founding { lane: G }), Some(&q(1, 1)));
    assert_eq!(drawn.amount_of(&Mode::Founding { lane: H }), Some(&q(1, 2)));
    assert_eq!(held.amount_of(&Mode::Founding { lane: G }), Some(&q(3, 1)));
}
