// MU-CLIENT §4.4(1) — THE TWIST OF THE PULL, a test per clause: a body
// carried about the attractor is reached by a rotation that holds the
// attractor; a body carried off it is reached by none; and three in a line
// name no plane, so no twist is chosen.

use crate::transducer::cga::*;
use num_rational::BigRational;

fn q(n: i64) -> BigRational {
    BigRational::from_integer(n.into())
}

/// A body a 90° turn about the attractor from where it was predicted: the
/// twist carries it there, holds the attractor still, and returns after four.
#[test]
fn twist_about_turn_90_holds_attractor() {
    let centre = [q(2), q(2)];
    let (p, s) = (point_at(&[q(4), q(2)]), point_at(&[q(2), q(4)]));
    let tau = twist_about(&centre, &p, &s, 2).expect("a rotation of the pull");
    assert!(tau.sandwich(&p).projective_eq(&s), "it carries the predicted body onto the perceived one");
    let c = point_at(&centre);
    assert!(tau.sandwich(&c).projective_eq(&c), "and holds the attractor still");
    let fourth = tau.geo(&tau).geo(&tau).geo(&tau);
    assert!(fourth.projective_eq(&Mv::scalar(q(1))), "a 90° turn: four of it are the identity");
}

/// A body carried FARTHER from the attractor is no rotation of its pull: no
/// twist reaches it, and the orbit reads it as carried off.
#[test]
fn twist_about_refuses_body_carried_off() {
    let centre = [q(2), q(2)];
    assert!(twist_about(&centre, &point_at(&[q(4), q(2)]), &point_at(&[q(5), q(2)]), 2).is_none());
}

/// The predicted body, the perceived one and the attractor in one line name
/// no plane of rotation: refused, never a plane chosen for them.
#[test]
fn twist_about_refuses_collinear_bodies() {
    let centre = [q(2), q(2)];
    assert!(twist_about(&centre, &point_at(&[q(4), q(2)]), &point_at(&[q(0), q(2)]), 2).is_none());
}
