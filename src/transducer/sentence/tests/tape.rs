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
    f.vertices.push(Vertex::rebased_on(
        0,
        1,
        stalk::Invariant(vec![3, 5]),
        Box::new(element::carriage(&q1(), &(q0(), q1()))),
        vec![(1, 2)],
    ));
    let unfolded = f.unfold(Measure::Occupied);
    let demand = f.vertices.len() as u16;
    f.vertices.push(Vertex::Demand {
        kind: Demand::Source { multiplicity: 2 },
        chart: stalk::Invariant(vec![1, 4, 4]),
        bodies: vec![(9, 9, 0, 1), (9, 10, 0, 1)],
    });
    // A FACE CARRYING AN EDGE AT ONE MEMBER PAIR. `FormFace::new` records the
    // word alone, so the pair is written here with its edge on it.
    let mut edged = FormFace::new(1, glued, w.clone());
    edged.at.insert(
        crate::transducer::sentence::face_pair_key(
            &stalk::Invariant::default(),
            &stalk::Invariant(vec![1, 4, 4]),
        ),
        crate::transducer::sentence::FacePlace {
            carriage: element::identity_versor(),
            gauge: Vec::new(),
            edge: Some(Transport::from_word(w.clone())),
        },
    );
    f.faces.push(edged);
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    f.remainder = crate::transducer::element::place_chain(&[(9, 9, 1, 0)])
        .expect("one link is A5's chain");
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
            Vertex::Word { derivation: a, acts: aa, contact: ca_c, .. },
            Vertex::Word { derivation: b, acts: ab, contact: cb_c, .. },
        ) => {
            assert_eq!(ca_c.map(|c| (c.anchor, c.ring)), Some((1, 2)));
            assert_eq!(cb_c.map(|c| (c.anchor, c.ring)), Some((1, 2)));
            assert_eq!(a, b, "M1 is the rotor, contact is beside it");
            // THE WHOLE FIBRE ROUND-TRIPS, not a representative of it: every
            // chart the vertex acts on comes back with the carriage and the
            // gauge it acts by, and `Keyed`'s equality is the fibre's own —
            // the same members under the same keys, in no order (18.2).
            assert_eq!(aa, ab, "every member comes back filed at its own chart");
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
    match (&back.vertices[rebased as usize], &f.vertices[rebased as usize]) {
        (Vertex::Rebased { by, of, acts }, Vertex::Rebased { acts: was, .. }) => {
            assert_eq!(*by, 0);
            assert_eq!(*of, 1);
            // THE WHOLE FIBRE ROUND-TRIPS, keyed by chart: the relative and the
            // hue map at each chart `of` acted on, and one member here.
            assert_eq!(acts, was, "every member comes back filed at its own chart");
            let m = acts
                .get(stalk::Invariant(vec![3, 5]).key().as_bytes())
                .expect("the member at the chart it was written on");
            assert!(element::same_versor(
                &m.carriage,
                &element::carriage(&q1(), &(q0(), q1()))
            ));
            assert_eq!(m.gauge, vec![(1, 2)]);
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
    assert_eq!(back.faces[0].word, f.faces[0].word);
    // THE WHOLE MEMBER FIBRE RIDES THE TAPE, keyed by chart pair: the placement
    // at each pair and the edge where the pair has one.
    assert_eq!(back.faces[0].at, f.faces[0].at, "every member pair comes back at its own key");
    assert!(!back.faces[0].has_edge(), "a face with no measured edge transport is a remainder edge");
    let last = back.faces.len() - 1;
    assert_eq!(
        back.faces[last].at.scoped().filter_map(|(_, p)| p.edge.as_ref().map(|t| t.word.clone())).collect::<Vec<_>>(),
        vec![w.clone()],
        "the edge transport rides the face at its member pair"
    );
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
    use crate::transducer::alphabet::{standing, Rotor};
    let q = |n: i64| num_rational::BigRational::from_integer(num_bigint::BigInt::from(n));
    let chart = stalk::Invariant(vec![1, 4, 4]);
    let other = stalk::Invariant(vec![1, 6, 2]);
    // `3 + e₀₁ + e₀₂`: weight 11, held by no coordinate plane (37.7).
    let eleven = Rotor::scalar(q(3)).add(&Rotor::blade(0b0011, q(1))).add(&Rotor::blade(0b0101, q(1)));
    // `(1 + e₀₂)(1 + e₁₃)` carries the sensor plane wholly off itself: its
    // reading there is the zero block, and it is a demand for all of it.
    let off = Rotor::scalar(q(1)).add(&Rotor::blade(0b0101, q(1))).geo(&Rotor::scalar(q(1)).add(&Rotor::blade(0b1010, q(1))));
    let shadowless = standing(&off, (0, 1)).expect("a versor");
    assert!(
        matches!(&shadowless, Demand::Standing { block, .. } if block.iter().flatten().all(|x| x == &q0())),
        "the plane reads nothing of it"
    );
    let kinds = [
        Demand::RotorPrime {
            plane: (0, 1),
            prime: 17,
            witness: Box::new([[q1(), q0()], [q0(), q1()]]),
        },
        standing(&eleven, (0, 1)).expect("a versor"),
        shadowless,
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
    assert_ne!(demand_key(&kinds[3], &chart), demand_key(&kinds[4], &chart), "two kinds on one chart are two demands");
    assert_ne!(demand_key(&kinds[3], &chart), demand_key(&kinds[3], &other), "one kind on two charts is two demands");
    // THE WITNESS IS A RAY (1.5, A1): `λΦ` and `Φ` stand one demand.
    assert_eq!(
        demand_key(&standing(&eleven, (0, 1)).expect("a versor"), &chart),
        demand_key(&standing(&eleven.scale(&q(-2)), (0, 1)).expect("a versor"), &chart),
        "one witness under two scales is one key"
    );
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
        // The witness is the identity: a demand for nothing, whatever
        // block is written beside it (checklist (m) — the block cannot
        // witness on its own).
        Demand::Standing {
            block: Box::new([[q0(), q0()], [q0(), q0()]]),
            rotor: Box::new(Rotor::identity()),
        },
        Demand::Standing {
            block: Box::new([[q1(), q0()], [q0(), q1()]]),
            rotor: Box::new(Rotor::identity()),
        },
        // A standing block on a rotor with no weight names a witness that
        // is no versor, and a court has nothing to docket (15.3).
        Demand::Standing {
            block: Box::new([[q1(), q0()], [q0(), q1()]]),
            rotor: Box::new(Rotor::zero()),
        },
        Demand::TopologicalDeformation { image: other.clone(), axes: Vec::new() },
    ] {
        assert_eq!(untyped.typed(), Err(crate::court::verdict::REFUSE_DEMAND_UNTYPED), "{}", untyped.label());
    }
}

/// 50.5 — a form says what it could not close once per (kind, chart): a
/// second standing of the same kind on the same chart gathers its bodies
/// under the one vertex, and a different chart is a second vertex.
#[test]
fn a_form_stands_one_demand_per_kind_and_chart_and_gathers_the_bodies() {
    let chart = stalk::Invariant(vec![1, 4, 4]);
    let other = stalk::Invariant(vec![1, 6, 2]);
    let mut f = Form::default();
    f.stand_demand(Demand::Creation, &chart, vec![(0, 0, 0, 3)]);
    f.stand_demand(Demand::Creation, &chart, vec![(0, 1, 0, 3), (0, 0, 0, 3)]);
    f.stand_demand(Demand::Creation, &other, vec![(5, 5, 0, 1)]);
    let demanded: Vec<(Demand, Vec<Link>)> =
        f.demand_vertices().map(|(_, k, _, links)| (k.clone(), links.to_vec())).collect();
    assert_eq!(demanded.len(), 2, "one vertex per (kind, chart), got {demanded:?}");
    let on_chart = f
        .demands
        .get(&demand_key(&Demand::Creation, &chart))
        .copied()
        .expect("the demand is indexed under its key");
    let Some(Vertex::Demand { bodies, .. }) = f.vertices.get(on_chart as usize) else {
        panic!("the index names the demand vertex")
    };
    assert_eq!(bodies, &vec![(0, 0, 0, 3), (0, 1, 0, 3)], "the bodies are gathered, sorted, once each");
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
