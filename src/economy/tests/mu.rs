// MU-ECONOMY §1 — μ as exact lane amounts: a count keeps its value as n/1,
// a draw past a holding refuses naming the lane, and the codec writes one
// value one way and refuses every other writing.

use crate::core::q::Q;
use crate::economy::{LaneMove, Mu, MuRefusal};
use num_bigint::BigInt;

fn q(n: i64, d: i64) -> Q {
    Q::new(BigInt::from(n), BigInt::from(d))
}

fn mu(l: [(i64, i64); 4]) -> Mu {
    Mu::of(l.map(|(n, d)| q(n, d))).expect("non-negative")
}

/// THE CODEC IS THE MODES CODEC (MU-ECONOMY §9): every holding round-trips
/// mode by mode, an amount of no mode writes as the holding of nothing, and
/// trailing bytes, a mode held twice or an amount that is not positive are
/// refused whole.
#[test]
fn mu_round_trips_as_modes_and_refuses_a_malformed_holding() {
    let big = Q::new(BigInt::from(1) << 130, BigInt::from(3));
    let amounts = [
        Mu::zero(),
        Mu::counts([1, 0, u64::MAX, 7]),
        mu([(1, 2), (0, 1), (22, 7), (5, 1)]),
        Mu::of([big, q(1, 3), q(0, 1), q(9, 4)]).expect("non-negative"),
    ];
    for m in &amounts {
        let bytes = m.to_bytes();
        assert_eq!(Mu::from_bytes(&bytes), Ok(m.clone()), "{m}");
    }
    assert_eq!(Mu::zero().to_bytes().len(), Mu::MIN_LEN, "nothing held is its count alone");
    let mut trailing = Mu::counts([1, 2, 3, 4]).to_bytes();
    trailing.push(0);
    assert_eq!(Mu::from_bytes(&trailing), Err(MuRefusal::Malformed));
    let one = Mu::counts([0, 1, 0, 0]).to_bytes();
    let mut twice = 2u32.to_le_bytes().to_vec();
    twice.extend_from_slice(&one[4..]);
    twice.extend_from_slice(&one[4..]);
    assert_eq!(Mu::from_bytes(&twice), Err(MuRefusal::Malformed), "a mode held twice");
    let mut none = one.clone();
    let n = none.len();
    none[n - 1] = 0;
    assert!(Mu::from_bytes(&none).is_err(), "an amount that is not positive");
    assert_eq!(Mu::of([q(0, 1), q(0, 1), q(-1, 2), q(0, 1)]), Err(MuRefusal::Negative { lane: 2 }));
}

/// A COUNT KEEPS ITS VALUE: n becomes n/1 on its lane, and reads back as n.
#[test]
fn mu_counts_equal_rationals() {
    let m = Mu::counts([3, 0, 8, 1]);
    assert_eq!(m, [3, 0, 8, 1]);
    assert_eq!(m.lane(2), q(8, 1));
    assert!(m.is_integral());
    assert_ne!(mu([(1, 2), (0, 1), (0, 1), (0, 1)]), [0, 0, 0, 0]);
}

/// A DRAW PAST A HOLDING IS REFUSED naming the first lane short; nothing is
/// saturated, and a draw within it is exact.
#[test]
fn draw_refuses_past_holding() {
    let held = mu([(3, 1), (1, 2), (0, 1), (5, 3)]);
    let asked = mu([(1, 1), (2, 3), (0, 1), (5, 3)]);
    assert_eq!(held.draw(&asked), Err(MuRefusal::Short { lane: 1 }));
    let fits = mu([(3, 2), (1, 3), (0, 1), (5, 3)]);
    assert_eq!(held.draw(&fits), Ok(mu([(3, 2), (1, 6), (0, 1), (0, 1)])));
    assert_eq!(held.draw(&fits).expect("fits").add(&fits), held, "a draw and its add return the holding");
}

/// THE ARITHMETIC IS EXACT AND PER LANE: n equal parts are the whole; a
/// shortfall is zero where a lane fits; a cap takes the lesser per lane;
/// the reading of two amounts names each lane's move with its amount.
#[test]
fn mu_parts_shortfalls_and_moves_are_exact() {
    let m = Mu::counts([7, 1, 0, 9]);
    let third = m.part(3).expect("three parts");
    assert_eq!(Mu::sum([&third, &third, &third]), m);
    assert_eq!(third.lane(0), q(7, 3));
    assert_eq!(m.part(0), None);
    let room = mu([(5, 1), (3, 2), (0, 1), (9, 1)]);
    assert_eq!(m.shortfall(&room), [2, 0, 0, 0]);
    assert_eq!(m.capped(&room), mu([(5, 1), (1, 1), (0, 1), (9, 1)]));
    assert_eq!(
        room.lane_moves(&m),
        [LaneMove::Fell(q(2, 1)), LaneMove::Rose(q(1, 2)), LaneMove::Held, LaneMove::Held]
    );
    assert_eq!(m.occupied(), vec![0, 1, 3]);
}

/// MU-ECONOMY §2.5 — A MINT CARRIES ITS PROOF'S MODES: each lane's ruled
/// amount is set on that lane's modes in proportion, so the holding reads
/// the same lane by lane and holds the motion; a lane the motion does not
/// reach stays at its founding mode.
#[test]
fn carried_by_sets_each_lane_on_its_modes_in_proportion() {
    use crate::economy::modes::{Mode, Modes};
    let mut moved = Modes::founding(3, q(1, 1)).expect("H");
    moved.add_mode(Mode::Founding { lane: 3 }, q(1, 1));
    let hue_move = Mode::Hue(crate::transducer::sentence::presentation::GaugeMode::Move(1, 2));
    moved.add_mode(hue_move.clone(), q(3, 1));
    let lanes = mu([(1, 2), (2, 3), (0, 1), (5, 1)]);
    let held = Mu::carried_by(&lanes, &moved);
    assert_eq!(held.lanes(), lanes.lanes(), "every lane reads its ruled share");
    assert_eq!(held.modes().amount_of(&hue_move), Some(&q(2, 3)), "G's share on the hue move that moved it");
    assert_eq!(held.modes().amount_of(&Mode::Founding { lane: 0 }), Some(&q(1, 2)), "S, which no mode reached, at its founding mode");
    assert_eq!(held.modes().amount_of(&Mode::Founding { lane: 3 }), Some(&q(5, 1)));
    assert!(Mu::carried_by(&Mu::zero(), &moved).is_zero(), "nothing carried, nothing held");
}
