use crate::frame::*;
use crate::Q;

fn q(n: i64, d: i64) -> Q {
    Q::new(n.into(), d.into())
}

fn v(xs: &[(i64, i64)]) -> Vec<Q> {
    xs.iter().map(|&(n, d)| q(n, d)).collect()
}

/// EVERY MOVE IS UNDONE EXACTLY: carried by a move and by its inverse, a
/// reading returns bit for bit — stretch, twist, expand, contract, bend.
#[test]
fn move_inverse_undoes_every_move_exactly() {
    let x = v(&[(3, 7), (-2, 5), (1, 1)]);
    let moves = [
        Move::Stretch(v(&[(1, 2), (0, 1), (-4, 3)])),
        Move::Twist(v(&[(0, 1), (2, 1), (0, 1)])),
        Move::Expand(q(5, 2)),
        Move::Contract(q(7, 3)),
        Move::Bend(v(&[(1, 3), (-1, 4), (0, 1)])),
    ];
    for m in moves {
        let there = m.apply(&x).expect("carried");
        assert_eq!(m.inverse().apply(&there), Ok(x.clone()), "{m:?} undone");
    }
}

/// A FRAME AND ITS INVERSE ARE THE IDENTITY: five moves composed, a
/// reading carried there and back returns exactly; the composite of a
/// frame with its inverse carries every reading to itself.
#[test]
fn frame_inverse_composes_to_identity() {
    let frame = Frame::identity()
        .then(Move::Bend(v(&[(1, 5), (1, 7)])))
        .then(Move::Twist(v(&[(0, 1), (4, 1)])))
        .then(Move::Turn { plane: (0, 1), degrees: q(270, 1) })
        .then(Move::Expand(q(9, 4)))
        .then(Move::Stretch(v(&[(-1, 3), (2, 1)])))
        .then(Move::Contract(q(3, 2)));
    for x in [v(&[(1, 2), (1, 3)]), v(&[(-5, 2), (7, 11)]), v(&[(0, 1), (0, 1)])] {
        let there = frame.carry(&x).expect("carried");
        assert_eq!(frame.inverse().carry(&there), Ok(x.clone()), "there and back");
        assert_eq!(frame.clone().compose(&frame.inverse()).carry(&x), Ok(x), "the composite is the identity");
    }
}

/// TWO TWISTS ARE A TURN: reflections in the two axes' mirrors turn the
/// plane by 180°, so a point goes to its antipode — exactly the turn of
/// 180°; a twist keeps every distance from the origin. A mirror at any
/// other slope is refused by name (ruled 2026-10-21):
/// spatial frames are the square's symmetries.
#[test]
fn twist_composes_twice_to_turn_of_180_degrees() {
    let turn_180 = Frame::identity().then(Move::Twist(v(&[(1, 1), (0, 1)]))).then(Move::Twist(v(&[(0, 1), (1, 1)])));
    let x = v(&[(3, 5), (-2, 7)]);
    assert_eq!(turn_180.carry(&x), Ok(v(&[(-3, 5), (2, 7)])), "the antipode");
    assert_eq!(turn_180.carry(&x), Move::Turn { plane: (0, 1), degrees: q(180, 1) }.apply(&x), "the turn of 180°");
    let one = Move::Twist(v(&[(0, 1), (3, 1)])).apply(&x).expect("reflected");
    assert_eq!(dot(&one, &one), dot(&x, &x), "a twist keeps the distance");
    assert_eq!(Move::Twist(v(&[(2, 1), (1, 1)])).apply(&x), Err(FrameRefusal::NotAxial), "a mirror off the axes: refused");
}

/// AN ORDERED TURN IN DEGREES: held exactly as its angle, undone by its
/// negation; a 90° turn carries a position exactly; a turn by 30°
/// alone is refused by name — no cosine of it is rational — while three
/// of them in one plane compose to 90° and carry exactly.
#[test]
fn turn_carries_exactly_and_refuses_inexact_degrees() {
    let x = v(&[(3, 5), (-2, 7), (1, 1)]);
    let quarter = Move::Turn { plane: (0, 1), degrees: q(90, 1) };
    assert_eq!(quarter.apply(&x), Ok(v(&[(2, 7), (3, 5), (1, 1)])), "a 90° turn from axis 0 toward axis 1");
    assert_eq!(quarter.inverse().apply(&quarter.apply(&x).expect("turned")), Ok(x.clone()), "undone by its negation");
    let thirty = Move::Turn { plane: (0, 1), degrees: q(30, 1) };
    assert_eq!(thirty.apply(&x), Err(FrameRefusal::Inexact), "30° alone: refused, never rounded");
    let three = Frame::identity().then(thirty.clone()).then(thirty.clone()).then(thirty);
    assert_eq!(three.carry(&x), quarter.apply(&x), "three turns of 30° in one plane compose to 90°, exactly");
    assert_eq!(Move::Turn { plane: (1, 1), degrees: q(90, 1) }.apply(&x), Err(FrameRefusal::NoPlane));
}

/// REFUSED BY NAME: a bend's pole carried to infinity, a mirror of zero, a
/// ratio not above one, axes unmatched — never rounded.
#[test]
fn move_apply_refuses_pole_and_unmatched_move() {
    // The pole of a bend by b is x = −b/|b|².
    assert_eq!(Move::Bend(v(&[(1, 2), (0, 1)])).apply(&v(&[(-2, 1), (0, 1)])), Err(FrameRefusal::AtPole));
    assert_eq!(Move::Twist(v(&[(0, 1), (0, 1)])).apply(&v(&[(1, 1), (1, 1)])), Err(FrameRefusal::NoMirror));
    assert_eq!(Move::Expand(q(1, 1)).apply(&v(&[(1, 1)])), Err(FrameRefusal::Ratio));
    assert_eq!(Move::Contract(q(1, 2)).apply(&v(&[(1, 1)])), Err(FrameRefusal::Ratio));
    assert_eq!(Move::Stretch(v(&[(1, 1)])).apply(&v(&[(1, 1), (1, 1)])), Err(FrameRefusal::Unmatched));
}
