// MU-CLIENT §4.3 — THE BREATH'S PINCH, APPLIED: a held form whose motion
// carries what it takes forward (a translation) and what it carries back
// through the inversion (a transversion) breathes, and its orthogonal pinch
// twists; a motion carrying only one of the two breathes nothing.

use crate::network::nerve::Keyed;
use crate::transducer::alphabet::{Derivation, Word};
use crate::transducer::sentence::{Acting, Form, Vertex};

fn q(n: i64) -> crate::core::Q {
    crate::core::Q::from_integer(n.into())
}

/// A form of one word act whose carriage translates by `a` and transverses
/// by `b`.
fn form_carrying(a: &[i64], b: &[i64]) -> Form {
    let c = crate::transducer::element::Conformal {
        translation: a.iter().map(|&v| q(v)).collect(),
        transversion: b.iter().map(|&v| q(v)).collect(),
        rho: q(1),
        rotation: crate::transducer::element::identity_versor(),
        inverted: false,
    };
    let derivation = Derivation::spelled(Word::identity());
    let mut acts: Keyed<Acting, crate::network::nerve::coord::Section> = Keyed::new();
    let member = Acting { chart: crate::transducer::stalk::Invariant::default(), carriage: Box::new(c.carriage()), gauge: Vec::new(), frames: Keyed::new() };
    acts.put(&member.key(), member);
    let address = crate::transducer::sentence::word_vertex_address(&derivation, &acts, None);
    let mut form = Form::default();
    form.put_vertex(Vertex::Frame).expect("the frame");
    form.put_vertex(Vertex::Word { address, derivation, acts, contact: None }).expect("the word");
    form
}

/// A HELD MOTION CARRYING BOTH BREATHES, AND ITS PINCH TWISTS: the breath
/// read off the held form is the breath of its translation and transversion,
/// split into its radial axis and its orthogonal pinch, and the pinch's
/// twist is an exact rotor — what the mind winds on the thought's vertex.
#[test]
fn a_held_motion_carrying_both_breathes_and_its_pinch_twists() {
    let (a, b) = ([1, 2, 0], [0, 1, 1]);
    let held = crate::transducer::bubble::hold::breaths_of(&form_carrying(&a, &b));
    let [one] = held.as_slice() else { panic!("one breath held: {}", held.len()) };
    let expected = crate::transducer::cga::breath(&a.map(q), &b.map(q)).expect("the pair breathes");
    assert_eq!(one, &expected, "the breath of the motion's own translation and transversion");
    let twist = one.twist().expect("a ∦ b: the pinch twists");
    assert!(crate::transducer::cga::is_versor(&twist), "an exact rotor, wound on the vertex");
}

/// A MOTION CARRYING ONLY ONE OF THE TWO BREATHES NOTHING: a translation
/// alone carries nothing back through the inversion.
#[test]
fn a_motion_carrying_one_side_alone_breathes_nothing() {
    assert!(crate::transducer::bubble::hold::breaths_of(&form_carrying(&[1, 2, 0], &[0, 0, 0])).is_empty());
}
