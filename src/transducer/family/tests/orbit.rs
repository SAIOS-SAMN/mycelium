// 22 — THE ORBIT: where a vertex's versor carries a body, and when there
// is no orbit at all.

use crate::transducer::family::*;

/// Contact when the first step moves the ring; NoOrbit when it does not.
#[test]
fn orbit_contact_and_no_orbit() {
let zero = q(0);
let one = q(1);
let moving = Vertex::Word {
    address: crate::transducer::a21::Address::from(vec![1]),
    word: crate::transducer::alphabet::Word::identity(),
    carriage: Box::new(element::carriage(&one.clone(), &(zero.clone(), one.clone()))),
    gauge: Vec::new(),
    class: Invariant::default(),
    contact: None,
};
let still = Vertex::Word {
    address: crate::transducer::a21::Address::from(vec![2]),
    word: crate::transducer::alphabet::Word::identity(),
    carriage: Box::new(element::carriage(&one, &(zero.clone(), zero))),
    gauge: Vec::new(),
    class: Invariant::default(),
    contact: None,
};
let cells = vec![(0i64, 0i64, 1u8)];
let anchor = vec![(0i64, 3i64)];
let frame = (5usize, 5usize);
match orbit(&moving, &cells, &anchor, frame) {
    OrbitEnd::Contact { steps, .. } => assert!(steps >= 1, "contact takes at least one step"),
    other => panic!("expected Contact, got {other:?}"),
}
assert_eq!(
    orbit(&still, &cells, &anchor, frame),
    OrbitEnd::NoOrbit,
    "a still word has no orbit"
);
let with_c = Vertex::Word {
    address: crate::transducer::a21::Address::from(vec![3]),
    word: crate::transducer::alphabet::Word::identity(),
    carriage: Box::new(element::carriage(&q(1), &(q(0), q(1)))),
    gauge: Vec::new(),
    class: Invariant::default(),
    contact: Some(crate::transducer::alphabet::Contact { anchor: 0, ring: 0 }),
};
match orbit(&with_c, &cells, &anchor, frame) {
    OrbitEnd::Contact { .. } | OrbitEnd::LeftFrame { .. } | OrbitEnd::Torsion { .. } => {}
    OrbitEnd::NoOrbit => panic!("contact word must orbit"),
}
}
