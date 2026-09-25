// 22 — THE ORBIT: where a vertex's versor carries a body, and when there
// is no orbit at all.

use crate::transducer::family::*;

/// Contact when the first step moves the ring; NoOrbit when it does not.
#[test]
fn orbit_contact_and_no_orbit() {
let zero = q(0);
let one = q(1);
let moving = Vertex::word_on(
    crate::transducer::alphabet::Derivation::spelled(
        crate::transducer::alphabet::Word::identity(),
    ),
    Invariant::default(),
    Box::new(element::carriage(&one.clone(), &(zero.clone(), one.clone()))),
    Vec::new(),
    [0u64],
    None,
);
let still = Vertex::word_on(
    crate::transducer::alphabet::Derivation::spelled(
        crate::transducer::alphabet::Word::identity(),
    ),
    Invariant::default(),
    Box::new(element::carriage(&one, &(zero.clone(), zero))),
    Vec::new(),
    [0u64],
    None,
);
let painted = vec![(0i64, 0i64, 1u8)];
let anchor = vec![(0i64, 3i64)];
let frame = (5usize, 5usize);
match orbit(&moving, &painted, &anchor, frame) {
    OrbitEnd::Contact { steps, .. } => assert!(steps >= 1, "contact takes at least one step"),
    other => panic!("expected Contact, got {other:?}"),
}
assert_eq!(
    orbit(&still, &painted, &anchor, frame),
    OrbitEnd::NoOrbit,
    "a still word has no orbit"
);
let with_c = Vertex::word_on(
    crate::transducer::alphabet::Derivation::spelled(
        crate::transducer::alphabet::Word::identity(),
    ),
    Invariant::default(),
    Box::new(element::carriage(&q(1), &(q(0), q(1)))),
    Vec::new(),
    [0u64],
    Some(crate::transducer::alphabet::Contact { anchor: 0, ring: 0 }),
);
match orbit(&with_c, &painted, &anchor, frame) {
    OrbitEnd::Contact { .. } | OrbitEnd::LeftFrame { .. } | OrbitEnd::Torsion { .. } => {}
    OrbitEnd::NoOrbit => panic!("contact word must orbit"),
}
}
