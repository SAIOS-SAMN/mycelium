use super::super::share_compounds::*;
use crate::Q;
use num_traits::{One, Zero};

fn q(n: i64, d: i64) -> Q {
    Q::new(n.into(), d.into())
}

/// THE LAW'S OWN FIXED POINTS: nothing taken leaves the whole; no rings
/// leave the whole; a share of nothing leaves the whole, however long.
#[test]
fn whole_reads_one_when_nothing_taken() {
    assert_eq!(Remaining::whole().reading(), Q::one());
    assert_eq!(Remaining::whole().after(&q(1, 3), 0).map(|r| r.reading()), Ok(Q::one()));
    assert_eq!(Remaining::whole().after(&Q::zero(), 1_000_000).map(|r| r.reading()), Ok(Q::one()));
}

/// `(1 − x)^n`, exactly: a quarter taken each of three rings leaves 27/64.
#[test]
fn after_compounds_share_exactly() {
    assert_eq!(Remaining::whole().after(&q(1, 4), 3).map(|r| r.reading()), Ok(q(27, 64)));
}

/// COMPOSITION: regrouping the rings changes nothing — `a` then `b` rings
/// at one share are one run of `a + b`, held and read alike, however the
/// history was cut.
#[test]
fn after_and_then_ignore_ring_grouping() {
    let x = q(2, 7);
    let at_once = Remaining::whole().after(&x, 12).expect("a share");
    let in_runs = Remaining::whole().after(&x, 5).and_then(|r| r.after(&x, 4)).and_then(|r| r.after(&x, 3)).expect("a share");
    assert_eq!(in_runs, at_once, "one run, held alike");
    let first = Remaining::whole().after(&x, 7).expect("a share");
    let second = Remaining::whole().after(&x, 5).expect("a share");
    assert_eq!(first.then(&second), Ok(at_once.clone()), "two histories composed fold at the seam");
    assert_eq!(at_once.reading(), at_once.factors().iter().fold(Q::one(), |a, (l, n)| a * power(l, *n)));
}

/// A SHARE READ AGAIN OPENS A NEW RUN, and the reading is the product of
/// the runs: what changed is held as the body's history, never averaged.
#[test]
fn after_changed_share_opens_new_run() {
    let r = Remaining::whole().after(&q(1, 2), 2).and_then(|r| r.after(&q(1, 3), 1)).expect("shares");
    assert_eq!(r.factors().len(), 2);
    assert_eq!(r.reading(), q(1, 4) * q(2, 3));
}

/// WHAT REMAINS IS READ IN A FRAME, AND BACK: a history's exact product
/// carried into an observer's frame — contracted, twisted, bent — and
/// back by the inverse frame returns bit for bit; no floor is taken.
#[test]
fn read_in_round_trips_through_frame() {
    use crate::frame::{Frame, Move, ReadIn};
    let r = Remaining::whole().after(&q(1, 4), 3).and_then(|r| r.after(&q(2, 9), 5)).expect("shares");
    let frame = Frame::identity().then(Move::Contract(q(3, 1))).then(Move::Twist(vec![q(1, 1)])).then(Move::Bend(vec![q(2, 5)]));
    let there = r.read_in(&frame).expect("carried");
    assert_eq!(frame.inverse().carry(&there), Ok(vec![r.reading()]), "there and back, exactly");
    assert_eq!(r.read_in(&Frame::identity()), Ok(vec![q(27, 64) * power(&q(7, 9), 5)]), "the identity reads it as it is");
}

/// REFUSED BY NAME: a negative share, a share that would take all that
/// remains or more, and a count past its carrier — never clamped.
#[test]
fn after_refuses_by_name() {
    assert_eq!(Remaining::whole().after(&q(-1, 5), 1), Err(ShareRefusal::Negative));
    assert_eq!(Remaining::whole().after(&Q::one(), 1), Err(ShareRefusal::Whole));
    assert_eq!(Remaining::whole().after(&q(3, 2), 1), Err(ShareRefusal::Whole));
    let full = Remaining::whole().after(&q(1, 9), u64::MAX).expect("a share");
    assert_eq!(full.after(&q(1, 9), 1), Err(ShareRefusal::Uncounted));
}
