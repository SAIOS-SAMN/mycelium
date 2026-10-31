use super::super::closeness::*;
use crate::Q;
use num_traits::{One, Zero};

fn q(n: i64, d: i64) -> Q {
    Q::new(n.into(), d.into())
}

fn qs(v: &[(i64, i64)]) -> Vec<Q> {
    v.iter().map(|&(n, d)| q(n, d)).collect()
}

/// THE LAW'S OWN FIXED POINTS: itself 1; disjoint planes ½; one plane in
/// antiphase 0; one plane alone the linear in-step measure `1 − 2|Δ|`;
/// and the closeness is two-sided — the same read from either body.
#[test]
fn closeness_keeps_fixed_points() {
    let a = Shares::of(&qs(&[(1, 3), (1, 2), (1, 6)])).expect("shares");
    assert_eq!(a.closeness(&a, &qs(&[(0, 1), (0, 1), (0, 1)])), Ok(Q::one()));
    let (p0, p1) = (Shares::of(&qs(&[(1, 1), (0, 1)])).expect("plane 0"), Shares::of(&qs(&[(0, 1), (1, 1)])).expect("plane 1"));
    assert_eq!(p0.closeness(&p1, &qs(&[(1, 5), (1, 7)])), Ok(q(1, 2)));
    let one = Shares::of(&qs(&[(1, 1)])).expect("one plane");
    assert_eq!(one.closeness(&one, &qs(&[(1, 2)])), Ok(Q::zero()));
    for d in [q(1, 8), q(3, 10), q(-2, 5), q(7, 6)] {
        assert_eq!(one.closeness(&one, std::slice::from_ref(&d)), Ok(Q::one() - q(2, 1) * reduced(&d)));
    }
    let b = Shares::of(&qs(&[(2, 7), (1, 7), (4, 7)])).expect("shares");
    let delta = qs(&[(1, 12), (5, 9), (7, 10)]);
    assert_eq!(a.closeness(&b, &delta), b.closeness(&a, &delta), "two-sided");
}

/// REFUSED BY NAME: a negative share, shares not summing to one, planes
/// unmatched.
#[test]
fn shares_of_and_closeness_refuse_by_name() {
    assert_eq!(Shares::of(&qs(&[(-1, 3), (4, 3)])), Err(ClosenessRefusal::Negative));
    assert_eq!(Shares::of(&qs(&[(1, 2), (1, 3)])), Err(ClosenessRefusal::Unsummed));
    let (a, b) = (Shares::of(&qs(&[(1, 1)])).expect("a"), Shares::of(&qs(&[(1, 2), (1, 2)])).expect("b"));
    assert_eq!(a.closeness(&b, &qs(&[(0, 1), (0, 1)])), Err(ClosenessRefusal::Unmatched));
    assert_eq!(b.closeness(&b, &qs(&[(0, 1)])), Err(ClosenessRefusal::Unmatched));
}

/// THE JOINT READ (the landing rule): the selecting universe's vector —
/// shares, phases, and the EXACT closeness it reads — reproduced bit for
/// bit. No grid is read: floors sent beside them are the sender's own
/// observer's, never a joint frame.
#[test]
fn closeness_reproduces_selecting_universe_vector() {
    type Row = (Vec<Q>, Vec<Q>, Vec<Q>, Q);
    let rows: Vec<Row> = vec![
        (qs(&[(1, 1), (0, 1)]), qs(&[(1, 1), (0, 1)]), qs(&[(0, 1), (1, 3)]), Q::one()),
        (qs(&[(1, 1), (0, 1)]), qs(&[(0, 1), (1, 1)]), qs(&[(1, 5), (1, 7)]), q(1, 2)),
        (qs(&[(1, 1)]), qs(&[(1, 1)]), qs(&[(1, 2)]), Q::zero()),
        (qs(&[(1, 1)]), qs(&[(1, 1)]), qs(&[(1, 8)]), q(3, 4)),
        (qs(&[(3, 5), (2, 5)]), qs(&[(1, 4), (3, 4)]), qs(&[(1, 8), (1, 3)]), q(119, 240)),
        (qs(&[(1, 3), (1, 2), (1, 6)]), qs(&[(2, 7), (1, 7), (4, 7)]), qs(&[(1, 12), (5, 9), (7, 10)]), q(659, 1260)),
        (qs(&[(1, 60), (29, 60), (1, 4), (1, 4)]), qs(&[(1, 2), (1, 6), (1, 6), (1, 6)]), qs(&[(1, 60), (59, 60), (1, 2), (1, 3)]), q(427, 900)),
    ];
    for (w, w2, delta, c2) in rows {
        let (a, b) = (Shares::of(&w).expect("shares"), Shares::of(&w2).expect("shares"));
        assert_eq!(a.closeness(&b, &delta), Ok(c2), "exact");
    }
}

/// THE CLOSENESS READ IN A FRAME, AND BACK: carried into an observer's
/// frame — a stretch, an expansion, a bend — and back by the frame's
/// inverse, the closeness returns exactly; the identity frame reads it
/// as it is.
#[test]
fn closeness_in_round_trips_through_frame() {
    use crate::frame::{Frame, Move};
    let (a, b) = (Shares::of(&qs(&[(3, 5), (2, 5)])).expect("a"), Shares::of(&qs(&[(1, 4), (3, 4)])).expect("b"));
    let delta = qs(&[(1, 8), (1, 3)]);
    assert_eq!(a.closeness_in(&b, &delta, &Frame::identity()), Ok(q(119, 240)), "the identity reads it as it is");
    let frame = Frame::identity().then(Move::Stretch(vec![q(1, 3)])).then(Move::Expand(q(5, 2))).then(Move::Bend(vec![q(1, 7)]));
    let there = a.closeness_in(&b, &delta, &frame).expect("carried");
    let back = frame.inverse().carry(&[there]).expect("carried back");
    assert_eq!(back, vec![q(119, 240)], "there and back, exactly");
}
