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

/// THE CODEC ROUND-TRIPS EVERY AMOUNT, and refuses a lane written out of
/// lowest terms, a zero not written `0/1`, a negative lane, a zero
/// denominator, a limb run with a zero top limb and trailing bytes.
#[test]
fn mu_round_trips_and_refuses_unreduced_ratio() {
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
    // Lane G written as 2/4 — the value 1/2 out of lowest terms.
    let lane = |sign: u8, n: &[u64], d: &[u64]| {
        let mut out = vec![sign];
        for run in [n, d] {
            crate::transducer::address::put_key_count(&mut out, run.len());
            for x in run {
                out.extend_from_slice(&x.to_le_bytes());
            }
        }
        out
    };
    let with_g = |g: Vec<u8>| {
        let mut out = lane(0, &[], &[1]);
        out.extend(g);
        out.extend(lane(0, &[], &[1]));
        out.extend(lane(0, &[], &[1]));
        out
    };
    assert_eq!(Mu::from_bytes(&with_g(lane(0, &[1], &[2]))).map(|m| m.lane(1).clone()), Ok(q(1, 2)));
    assert_eq!(Mu::from_bytes(&with_g(lane(0, &[2], &[4]))), Err(MuRefusal::NotReduced { lane: 1 }));
    assert_eq!(Mu::from_bytes(&with_g(lane(0, &[], &[2]))), Err(MuRefusal::NotReduced { lane: 1 }), "zero is 0/1");
    assert_eq!(Mu::from_bytes(&with_g(lane(0, &[1], &[]))), Err(MuRefusal::NotReduced { lane: 1 }), "no zero denominator");
    assert_eq!(Mu::from_bytes(&with_g(lane(1, &[1], &[2]))), Err(MuRefusal::Negative { lane: 1 }));
    assert_eq!(Mu::from_bytes(&with_g(lane(0, &[1, 0], &[2]))), Err(MuRefusal::Malformed), "one writing per value");
    let mut trailing = Mu::counts([1, 2, 3, 4]).to_bytes();
    trailing.push(0);
    assert_eq!(Mu::from_bytes(&trailing), Err(MuRefusal::Malformed));
    assert_eq!(Mu::of([q(0, 1), q(0, 1), q(-1, 2), q(0, 1)]), Err(MuRefusal::Negative { lane: 2 }));
}

/// A COUNT KEEPS ITS VALUE: n becomes n/1 on its lane, and reads back as n.
#[test]
fn mu_counts_equal_their_rationals() {
    let m = Mu::counts([3, 0, 8, 1]);
    assert_eq!(m, [3, 0, 8, 1]);
    assert_eq!(m.lane(2), &q(8, 1));
    assert!(m.is_integral());
    assert_ne!(mu([(1, 2), (0, 1), (0, 1), (0, 1)]), [0, 0, 0, 0]);
}

/// A DRAW PAST A HOLDING IS REFUSED naming the first lane short; nothing is
/// saturated, and a draw within it is exact.
#[test]
fn draw_past_holding_refuses_by_name() {
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
    assert_eq!(third.lane(0), &q(7, 3));
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
