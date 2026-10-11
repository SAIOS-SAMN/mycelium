// 62.1, MU-CLIENT II.3 — THE PUNCTURE / SEAL AXIS ON THE MIND'S OWN COMPLEX:
// a ring held at holonomy ≠ 1 is ascended and punctured, keeping its class
// as a hole; a discharge of that class seals it, and the seal is the
// puncture's inverse on content.

use crate::transducer::alphabet::{LetterId, Rotor, Word};
use crate::transducer::bubble::*;
use crate::transducer::cech::Section;
use crate::transducer::element::identity_versor;

/// Three thoughts on one ring, the (3, 4, 5) turn on one edge: the arrivals
/// around it disagree, so a front from the first holds the ring.
fn held_ring() -> (Contemplation, Vec<Vec<u8>>) {
    let p = Word::from_letters(&[(LetterId::pythagorean_5((0, 1)).expect("the sensor plane"), false)]).rotor();
    let mut c = Contemplation::new();
    let vs: Vec<Vec<u8>> = (1u8..=3).map(|i| vec![i; 4]).collect();
    for (x, g) in vs.iter().zip([Rotor::identity(), p.clone(), Rotor::identity()]) {
        c.complex.set_section(x, Section::at(x.clone().into(), g), [1, 0, 0, 0]);
    }
    c.complex.set_edge(&vs[0], &vs[1], p, identity_versor(), [1, 0, 0, 0]);
    c.complex.set_edge(&vs[1], &vs[2], Rotor::identity(), identity_versor(), [1, 0, 0, 0]);
    c.complex.set_edge(&vs[0], &vs[2], Rotor::identity(), identity_versor(), [1, 0, 0, 0]);
    let _ = c.complex.coboundary_on_link(&vs[0], |_| [1, 0, 0, 0]);
    c.fix(&vs[0], false);
    c.transport(1);
    c.transport(2);
    (c, vs)
}

/// THE HELD RING IS A QUESTION, AND ITS ANSWER IS THE PUNCTURE UNDONE ON
/// CONTENT. The held ring's cell reads the class of its holonomy and is
/// opened into a hole keeping exactly that class, filed under its class key;
/// the hole pins its boundary (no edge under it drains). A discharge of that
/// class seals it: the cap re-ascended over the unchanged boundary reads the
/// very class the hole kept — `seal ∘ puncture` is the identity on content —
/// the class leaves the questions, and the cell stands sealed again.
#[test]
fn a_held_ring_is_punctured_and_a_discharge_of_its_class_seals_it() {
    let (mut c, vs) = held_ring();
    let board = c.complex.board();
    let [(cell, kept)] = board.holes.as_slice() else { panic!("the held ring is one hole: {board:?}") };
    let Fill::Class(kappa, held) = kept else { panic!("the hole keeps a class: {kept:?}") };
    assert!(board.sealed.is_empty() && board.open.is_empty(), "no 2-cell stands beside the hole");
    let class_key = kappa.class_key();
    assert!(c.questions.get(class_key.as_bytes()).is_some_and(|q| q.contains(cell)), "filed under its class key");
    let edge = c.complex.link(&vs[0], 1).into_iter().find(|k| c.complex.vertices_at(k).is_some_and(|v| v.contains(&vs[1])));
    let edge = edge.expect("the turned edge stands");
    assert!(matches!(c.complex.drain(&edge), crate::transducer::cech::Drained::Pinned), "the hole pins its boundary");

    c.docket_discharged(class_key.as_bytes());

    assert!(!c.complex.is_hole(cell), "the hole is sealed");
    assert!(c.questions.get(class_key.as_bytes()).is_none(), "the class leaves the questions");
    let after = c.complex.board();
    let [(sealed, cap)] = after.sealed.as_slice() else { panic!("the cell stands sealed: {after:?}") };
    assert_eq!(sealed, cell);
    let Fill::Class(cap_kappa, cap_rotor) = cap else { panic!("the cap reads a class: {cap:?}") };
    assert_eq!(cap_kappa.class_key(), class_key, "the cap reads the class the hole kept");
    assert!(cap_rotor.projective_eq(held), "and the same holonomy: seal undoes puncture on content");
}

/// A RING WHOSE ARRIVALS AGREE ASKS NOTHING: its cell fills flat through the
/// one fill path and no hole is opened.
#[test]
fn a_ring_whose_arrivals_agree_punctures_nothing() {
    let q = Word::from_letters(&[(LetterId::turn_90((0, 1)).expect("the sensor plane"), false)]).rotor();
    let mut c = Contemplation::new();
    let vs: Vec<Vec<u8>> = (1u8..=3).map(|i| vec![i; 4]).collect();
    for (x, g) in vs.iter().zip([Rotor::identity(), q.clone(), q.geo(&q)]) {
        c.complex.set_section(x, Section::at(x.clone().into(), g), [1, 0, 0, 0]);
    }
    c.complex.set_edge(&vs[0], &vs[1], q.clone(), identity_versor(), [1, 0, 0, 0]);
    c.complex.set_edge(&vs[1], &vs[2], q.clone(), identity_versor(), [1, 0, 0, 0]);
    c.complex.set_edge(&vs[0], &vs[2], q.geo(&q), identity_versor(), [1, 0, 0, 0]);
    let _ = c.complex.coboundary_on_link(&vs[0], |_| [1, 0, 0, 0]);
    c.fix(&vs[0], false);
    c.transport(1);
    c.transport(2);
    assert!(c.complex.board().holes.is_empty() && c.questions.is_empty(), "no question");
    assert_eq!(c.complex.board().sealed.len(), 1, "the ring's cell sealed flat");
}
