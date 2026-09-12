// 19 — THE TAPE: a form round-trips its v4 tape exactly, and a short or
// partial tape is refused rather than partly read.

use super::common::*;
use crate::transducer::alphabet::{self, Word};
use crate::transducer::element::{self, Element};
use crate::transducer::sentence::*;
use crate::transducer::stalk::{self};

/// SENTENCE_TAG v6 round-trips Frame, Word (with contact), Glued,
/// Rebased, Unfolded, Demand (50.5), faces with their edge transport
/// (50.3), and loops.
#[test]
fn form_tape_v6_round_trips() {
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
    let demand = f.vertices.len() as u16;
    f.vertices.push(Vertex::Demand {
        kind: Demand::Source { multiplicity: 2 },
        chart: stalk::Invariant(vec![1, 4, 4]),
        bodies: vec![(9, 9, 0, 1), (9, 10, 0, 1)],
    });
    let mut edged = FormFace::new(1, glued, w.clone());
    edged.edge = Some(Transport::from_word(w.clone()));
    f.faces.push(edged);
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    f.remainder = crate::transducer::element::place_chain(&[(9, 9, 1, 0)])
        .expect("one cell is A5's chain");
    f.reindex();
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
    let back = crate::transducer::sentence::from_element_form(&e).expect("decode v4");
    assert_eq!(back.vertices.len(), f.vertices.len());
    assert_eq!(back.faces.len(), f.faces.len());
    assert_eq!(back.loops.len(), f.loops.len());
    // The remainder rides the tape as it sits in the scene, so the chain
    // and the placement both survive the round trip.
    assert_eq!(back.remainder.placed(), f.remainder.placed());
    assert_eq!(back.remainder.chain(), f.remainder.chain());
    assert_eq!(back.depth, f.depth);
    match (&f.vertices[1], &back.vertices[1]) {
        (
            Vertex::Word { derivation: a, gauge: ga, class: ca, contact: ca_c, .. },
            Vertex::Word { derivation: b, gauge: gb, class: cb, contact: cb_c, .. },
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
    assert_eq!(back.faces[0].edge, None, "a face with no measured edge transport is a remainder edge");
    let last = back.faces.len() - 1;
    assert_eq!(back.faces[last].edge.as_ref().map(|t| t.word.clone()), Some(w.clone()), "the edge transport rides the face");
    match (&f.vertices[demand as usize], &back.vertices[demand as usize]) {
        (
            Vertex::Demand { kind: ka, chart: ca, bodies: ba },
            Vertex::Demand { kind: kb, chart: cb, bodies: bb },
        ) => {
            assert_eq!(ka, kb);
            assert_eq!(ca, cb);
            assert_eq!(ba, bb, "the bodies a demand stands for ride the tape");
        }
        _ => panic!("demand vertex"),
    }
    assert!(!back.is_closed(), "a sentence with a demand vertex is not closed");
    assert_eq!(
        back.demands.get(&demand_key(&Demand::Source { multiplicity: 2 }, &stalk::Invariant(vec![1, 4, 4]))),
        Some(&demand),
        "the demand is read by (kind, chart)"
    );
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

/// 50.5 — a demand is named by its kind and the coordinates it carries: two
/// kinds on one chart are two demands, one kind on two charts is two, and
/// every kind decodes to itself; a kind carrying nothing is not a demand.
#[test]
fn a_demand_keys_by_kind_and_coordinates_and_an_empty_one_is_untyped() {
    let chart = stalk::Invariant(vec![1, 4, 4]);
    let other = stalk::Invariant(vec![1, 6, 2]);
    let kinds = [
        Demand::RotorPrime {
            plane: (0, 1),
            prime: 17,
            witness: Box::new([[q1(), q0()], [q0(), q1()]]),
        },
        Demand::Standing { block: Box::new([[q1(), q0()], [q0(), q1()]]) },
        Demand::Source { multiplicity: 3 },
        Demand::Creation,
        Demand::TopologicalDeformation { image: other.clone(), axes: vec![1, 2] },
    ];
    for k in &kinds {
        let mut b = Vec::new();
        k.to_bytes(&mut b);
        let mut pos = 0;
        assert_eq!(Demand::from_bytes(&b, &mut pos).as_ref(), Some(k), "every kind decodes to itself");
        assert_eq!(pos, b.len(), "and consumes exactly its bytes");
        assert_eq!(k.typed(), Ok(()), "{}", k.label());
        assert_eq!(k.key().tag(), Some(crate::transducer::address::TAG_DEMAND));
    }
    assert_ne!(demand_key(&kinds[2], &chart), demand_key(&kinds[3], &chart), "two kinds on one chart are two demands");
    assert_ne!(demand_key(&kinds[2], &chart), demand_key(&kinds[2], &other), "one kind on two charts is two demands");
    for untyped in [
        Demand::Source { multiplicity: 0 },
        Demand::RotorPrime {
            plane: (1, 1),
            prime: 17,
            witness: Box::new([[q1(), q0()], [q0(), q1()]]),
        },
        Demand::RotorPrime {
            plane: (0, 1),
            prime: 5,
            witness: Box::new([[q1(), q0()], [q0(), q1()]]),
        },
        // 37.7 P2: a demand with no witness names a prime it does not
        // carry, and the court has nothing to read the factor off.
        Demand::RotorPrime {
            plane: (0, 1),
            prime: 17,
            witness: Box::new([[q0(), q0()], [q0(), q0()]]),
        },
        Demand::Standing { block: Box::new([[q0(), q0()], [q0(), q0()]]) },
        Demand::TopologicalDeformation { image: other.clone(), axes: Vec::new() },
    ] {
        assert_eq!(untyped.typed(), Err(crate::court::verdict::REFUSE_DEMAND_UNTYPED), "{}", untyped.label());
    }
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
        crate::transducer::sentence::from_element_form(&truncated).is_none(),
        "a short v4 tape is refused"
    );
    let mut v2 = e.description().to_vec();
    v2[1] = 2;
    let mut old = Element::identity();
    old.set_description(v2);
    old.set_magnitude([1, 0, 0, 0]);
    assert!(
        crate::transducer::sentence::from_element_form(&old).is_none(),
        "a v2 tape is refused"
    );
}

#[test]
fn a_sentence_round_trips_its_nerve() {
    let f = star_form(2, Vec::new());
    let e = as_element_form(&f, [1, 0, 0, 0]).expect("form element");
    assert!(e.is_sentence());
    let back = crate::transducer::sentence::from_element_form(&e).expect("the form survives the tape");
    assert_eq!(back.vertices.len(), f.vertices.len());
    assert_eq!(back.faces.len(), f.faces.len());
    assert_eq!(back.loops.len(), f.loops.len());
}
