// MU-CLIENT §4.3 — THE BREATH AND ITS PINCHES, APPLIED: a held form breathes
// within each motion (what it takes forward, a translation, against what it
// carries back through the inversion, a transversion) and across its motions
// (every pair of their generators), and every pinch twists; a form whose
// motions commute breathes nothing.

use crate::network::nerve::Keyed;
use crate::transducer::alphabet::{Derivation, Word};
use crate::transducer::sentence::{Acting, Form, Vertex};

fn q(n: i64) -> crate::core::Q {
    crate::core::Q::from_integer(n.into())
}

fn budget() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 16)
}

/// A carriage translating by `a` and transversing by `b`.
fn carriage(a: &[i64], b: &[i64]) -> crate::transducer::element::Versor {
    crate::transducer::element::Conformal {
        translation: a.iter().map(|&v| q(v)).collect(),
        transversion: b.iter().map(|&v| q(v)).collect(),
        rho: q(1),
        rotation: crate::transducer::element::identity_versor(),
        inverted: false,
    }
    .carriage()
}

/// A form of one word whose acts carry the carriages, each on a chart of its
/// own: a form keys its words by derivation, so the motions one word carries
/// on several charts are its acts.
fn form_of(carriages: &[crate::transducer::element::Versor]) -> Form {
    let mut form = Form::default();
    form.put_vertex(Vertex::Frame).expect("the frame");
    let derivation = Derivation::spelled(Word::identity());
    let mut acts: Keyed<Acting, crate::network::nerve::coord::Section> = Keyed::new();
    for (n, c) in (1u64..).zip(carriages) {
        let chart = crate::transducer::stalk::Invariant::counts(&[n]);
        let member = Acting { chart, carriage: Box::new(c.clone()), gauge: Vec::new(), frames: Keyed::new() };
        acts.put(&member.key(), member);
    }
    let address = crate::transducer::sentence::word_vertex_address(&derivation, &acts, None);
    form.put_vertex(Vertex::Word { address, derivation, acts, contact: None }).expect("the word");
    form
}

/// WITHIN ONE MOTION: a held motion carrying both a translation and a
/// transversion breathes, its pinch the plane of the two, its twist an exact
/// rotor — what the mind winds on the thought's vertex.
#[test]
fn a_held_motion_carrying_both_breathes_and_its_pinch_twists() {
    let (a, b) = ([1, 2, 0], [0, 1, 1]);
    let held = crate::transducer::bubble::hold::breaths_of(&form_of(&[carriage(&a, &b)]), &mut budget());
    let [one] = held.as_slice() else { panic!("one breath held: {}", held.len()) };
    let expected = crate::transducer::cga::breath(&a.map(q), &b.map(q), &mut budget()).expect("the pair breathes");
    assert_eq!((one.radial.clone(), one.rest.clone()), (expected.radial, expected.rest), "the breath of the motion's own pair");
    let twists = one.twists();
    assert!(twists.len() == 1 && crate::transducer::cga::is_versor(&twists[0]), "a ∦ b: one pinch, an exact rotor");
}

/// ACROSS MOTIONS: one act translates and another transverses — neither
/// breathes alone — and the two breathe against each other, their
/// generators' commutator pinching in the plane of the two.
#[test]
fn two_held_motions_breathe_against_each_other() {
    let form = form_of(&[carriage(&[1, 2, 0], &[0, 0, 0]), carriage(&[0, 0, 0], &[0, 1, 1])]);
    let held = crate::transducer::bubble::hold::breaths_of(&form, &mut budget());
    let [one] = held.as_slice() else { panic!("one breath, across the two: {}", held.len()) };
    assert_eq!(one.by_kind(), [1, 0, 0], "a pinch in the plane the two span");
    assert_eq!(one.twists().len(), 1, "and it twists");
}

/// A FORM WHOSE MOTIONS COMMUTE BREATHES NOTHING: two translations carry
/// nothing back through the inversion, and translations commute.
#[test]
fn commuting_motions_breathe_nothing() {
    let form = form_of(&[carriage(&[1, 2, 0], &[0, 0, 0]), carriage(&[3, 0, 1], &[0, 0, 0])]);
    let held = crate::transducer::bubble::hold::breaths_of(&form, &mut budget());
    assert!(held.iter().all(|b| num_traits::Zero::is_zero(&b.radial) && b.planes.is_empty()), "nothing breathed: {}", held.len());
}
