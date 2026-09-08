// 19 — THE TAPE: a form round-trips its v4 tape exactly, and a short or
// partial tape is refused rather than partly read.

use super::common::*;
use crate::transducer::alphabet::{self, Word};
use crate::transducer::element::{self, Element};
use crate::transducer::sentence::*;
use crate::transducer::stalk::{self};

/// SENTENCE_TAG v4 round-trips Frame, Word (with contact), Glued,
/// Rebased, Unfolded, faces, and loops.
#[test]
fn form_tape_v4_round_trips() {
    let q = alphabet::LetterId::quarter((0, 1)).expect("quarter");
    let w = Word::from_letters(&[(q, false)]);
    let inner = closed_word_form(w.clone(), None);
    assert!(inner.is_closed(), "a tree of two vertices coheres");
    assert!(as_element_form(&inner, [0; 4]).is_none(), "zero descent is not spoken");
    let inner_e = as_element_form(&inner, [1, 0, 0, 0]).expect("inner tape");
    let inner_addr = inner_e.address();

    let mut f = closed_word_form(
        w.clone(),
        Some(alphabet::Contact { anchor: 1, ring: 2 }),
    );
    let glued = f
        .glue(&inner, inner_addr, Word::identity(), [0, 0, 0, 1])
        .expect("glue depth 0 into depth 0");
    let rebased = f.vertices.len() as u16;
    f.vertices.push(Vertex::Rebased {
        by: 0,
        of: 1,
        carriage: Box::new(element::carriage(&q1(), &(q0(), q1()))),
        gauge: vec![(1, 2)],
        class: stalk::Invariant(vec![3, 5]),
    });
    let unfolded = f.unfold(Measure::Occupied);
    f.faces.push(FormFace::new(1, glued, w.clone(),));
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    f.remainder = vec![(9, 9, 1, 0)];
    assert!(matches!(f.vertices[glued as usize], Vertex::Glued { .. }));
    assert!(matches!(
        f.vertices[rebased as usize],
        Vertex::Rebased { by: 0, of: 1, .. }
    ));
    assert!(matches!(
        f.vertices[unfolded as usize],
        Vertex::Unfolded {
            measure: Measure::Occupied
        }
    ));

    let e = as_element_form(&f, [1, 0, 0, 0]).expect("v4 tape");
    assert!(e.is_sentence());
    assert_eq!(e.description()[1], FORM_TAPE_VERSION, "the tape names its version");
    let back = from_element_form(&e).expect("decode v4");
    assert_eq!(back.vertices.len(), f.vertices.len());
    assert_eq!(back.faces.len(), f.faces.len());
    assert_eq!(back.loops.len(), f.loops.len());
    assert_eq!(back.remainder, f.remainder);
    assert_eq!(back.depth, f.depth);
    match (&f.vertices[1], &back.vertices[1]) {
        (
            Vertex::Word { word: a, gauge: ga, class: ca, contact: ca_c, .. },
            Vertex::Word { word: b, gauge: gb, class: cb, contact: cb_c, .. },
        ) => {
            assert_eq!(ca_c.map(|c| (c.anchor, c.ring)), Some((1, 2)));
            assert_eq!(cb_c.map(|c| (c.anchor, c.ring)), Some((1, 2)));
            assert_eq!(a, b, "M1 is the rotor, contact is beside it");
            assert_eq!(ga, gb);
            assert_eq!(ca, cb);
        }
        _ => panic!("word vertex"),
    }
    match (&f.vertices[glued as usize], &back.vertices[glued as usize]) {
        (Vertex::Glued { address: a, depth: da, .. }, Vertex::Glued { address: b, depth: db, .. }) => {
            assert_eq!(a, b);
            assert_eq!(da, db);
        }
        _ => panic!("glued vertex"),
    }
    match &back.vertices[rebased as usize] {
        Vertex::Rebased {
            by,
            of,
            carriage,
            gauge,
            class,
        } => {
            assert_eq!(*by, 0);
            assert_eq!(*of, 1);
            assert!(element::same_versor(
                carriage,
                &element::carriage(&q1(), &(q0(), q1()))
            ));
            assert_eq!(gauge, &vec![(1, 2)]);
            assert_eq!(class, &stalk::Invariant(vec![3, 5]));
        }
        _ => panic!("rebased vertex"),
    }
    assert_eq!(
        back.vertices[unfolded as usize],
        Vertex::Unfolded {
            measure: Measure::Occupied
        }
    );
    assert_eq!(back.faces[0].i, 0);
    assert_eq!(back.faces[0].j, 1);
    assert_eq!(back.faces[0].transport.word, f.faces[0].transport.word);
    let e_plain = as_element_form(&closed_word_form(w.clone(), None), [1, 0, 0, 0]).expect("plain");
    let e_contact = as_element_form(
        &closed_word_form(
            w.clone(),
            Some(alphabet::Contact { anchor: 0, ring: 2 }),
        ),
        [1, 0, 0, 0],
    )
    .expect("contact");
    assert_ne!(
        e_plain.address(),
        e_contact.address(),
        "contact is on the form key"
    );
    assert_eq!(w, w, "M1 is the rotor; contact is not a letter");
}

#[test]
fn tape_v4_refuses_short_or_partial() {
    let f = closed_word_form(Word::identity(), None);
    let e = as_element_form(&f, [1, 0, 0, 0]).expect("tape");
    assert_eq!(e.description()[1], FORM_TAPE_VERSION);
    let mut short = e.description().to_vec();
    short.pop();
    let mut truncated = Element::identity();
    truncated.set_description(short);
    truncated.set_magnitude([1, 0, 0, 0]);
    assert!(
        from_element_form(&truncated).is_none(),
        "a short v4 tape is refused"
    );
    let mut v2 = e.description().to_vec();
    v2[1] = 2;
    let mut old = Element::identity();
    old.set_description(v2);
    old.set_magnitude([1, 0, 0, 0]);
    assert!(
        from_element_form(&old).is_none(),
        "a v2 tape is refused"
    );
}

#[test]
fn a_sentence_round_trips_its_nerve() {
    let f = star_form(2, Vec::new());
    let e = as_element_form(&f, [1, 0, 0, 0]).expect("form element");
    assert!(e.is_sentence());
    let back = from_element_form(&e).expect("the form survives the tape");
    assert_eq!(back.vertices.len(), f.vertices.len());
    assert_eq!(back.faces.len(), f.faces.len());
    assert_eq!(back.loops.len(), f.loops.len());
}
