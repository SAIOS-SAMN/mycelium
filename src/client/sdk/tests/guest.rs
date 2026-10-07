// 78.117 P1 — the guest math's standard half: a kernel composes a transition
// from the algebra's own constructors and reads the court's verdict on it
// before sending.

use crate::client::sdk::guest::{body_at, carry, compose, inverse, sink, source, translate, turn, NotRigid, TransitionBuilder};
use crate::physics::laws::{verify_sized, Breach, Flow, Law};
use num_rational::BigRational;

fn q(n: i64) -> BigRational {
    BigRational::from_integer(n.into())
}

fn moments() -> Vec<Vec<BigRational>> {
    vec![vec![q(1), q(0), q(0)], vec![q(0), q(0), q(0)], vec![q(0), q(0), q(0)]]
}

/// A TURN THAT CARRIES ITS BODIES IS ADMITTED, and the check is the court's
/// own verdict: the (3, 4, 5) turn on `e₁e₂`, two bodies carried by it.
#[test]
fn a_turn_built_and_checked_is_the_courts_admission() {
    let m = turn((1, 2), &q(2), &q(1)).expect("a plane of the base");
    let t = TransitionBuilder::new(m, 7)
        .carried(body_at(vec![q(1), q(0), q(0)], q(1), moments()))
        .and_then(|b| b.carried(body_at(vec![q(0), q(2), q(0)], q(1), moments())))
        .expect("a turn carries rigidly")
        .checked()
        .expect("admitted");
    assert_eq!(t.heights, (7, 8));
    assert!(verify_sized(&t).admitted(), "the court's verdict, read locally");
}

/// A MOTION AND ITS INVERSE COMPOSE TO THE IDENTITY RAY; two translations to
/// their sum.
#[test]
fn compose_and_inverse_are_the_algebras() {
    let m = turn((2, 3), &q(3), &q(4)).expect("a plane");
    let back = inverse(&m).expect("a versor has an inverse");
    assert!(compose(&m, &back).projective_eq(&crate::transducer::cga::Mv::scalar(q(1))));
    let two = compose(&translate(&[q(1), q(0), q(0)]), &translate(&[q(0), q(2), q(0)]));
    assert!(two.projective_eq(&translate(&[q(1), q(2), q(0)])), "translations add");
    assert_eq!(turn((2, 2), &q(1), &q(1)), None, "no plane");
    assert_eq!(turn((1, 4), &q(1), &q(1)), None, "not a plane of the base");
}

/// A BODY NOT WHERE THE MOTION SENDS IT IS REFUSED BY NAME, and a dilation
/// carries no body rigidly.
#[test]
fn a_misplaced_body_is_refused_and_a_dilation_carries_none() {
    let m = turn((1, 2), &q(2), &q(1)).expect("a plane");
    let before = body_at(vec![q(1), q(0), q(0)], q(1), moments());
    let mut after = carry(&before, &m).expect("rigid");
    after.nucleus = crate::transducer::cga::sphere_at(&[q(5), q(5), q(0)], &q(1));
    let v = TransitionBuilder::new(m, 1).body(before.clone(), after).checked().expect_err("misplaced");
    assert!(v.breaches.contains(&(Law::Conformal, Breach::NotCarried { body: 0 })), "{:?}", v.breaches);
    let grow = crate::transducer::cga::dilator(&q(2));
    assert_eq!(TransitionBuilder::new(grow, 1).carried(before).err(), Some(NotRigid));
}

/// A SOURCE AND A SINK stand at their place with their mass, each its flow.
#[test]
fn a_source_and_a_sink_are_fluxes_at_their_place() {
    let at = [q(1), q(1), q(0)];
    let (s, k) = (source(&at, q(3)), sink(&at, q(3)));
    assert_eq!((s.flow, k.flow), (Flow::Source, Flow::Sink));
    assert_eq!(s.at, crate::transducer::cga::point_at(&at));
    assert_eq!(s.mass, q(3));
}
