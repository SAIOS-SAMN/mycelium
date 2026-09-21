// 19 — THE TAPE: a form round-trips its v4 tape exactly, and a short or
// partial tape is refused rather than partly read.

use super::common::*;
use crate::transducer::alphabet::{self, Word};
use crate::transducer::element::{self, Element};
use crate::transducer::sentence::*;
use crate::transducer::stalk::{self};

/// 52.3 AS AMENDED (54.5, ruled 2026-09-20) — A FILED ZERO IS LABOUR, NOT
/// CLOSURE. A loop whose edge is a (3,4,5) rotor files at period 0, A19's
/// spelling of infinite order; the filing rides the SPEAKER'S word and the
/// form does not cohere on it, because a circuit that walks is not closed
/// by anyone writing down that it walks.
#[test]
fn the_tape_carries_a_filed_zero_as_infinite_order() {
    let w = infinite();
    let mut f = edge_chord_form(std::slice::from_ref(&w));
    let loops = f.loops.clone();
    let [lp] = loops.as_slice() else { panic!("one chord closes one loop") };
    let classes: Vec<_> = f.edge_classes(lp).scoped().map(|(_, c)| c.clone()).collect();
    let [class] = classes.as_slice() else { panic!("one member path, so one class") };
    f.torsion.insert(class.clone(), FiledLoop { sigma: Vec::new(), period: 0, chord: lp.chord });
    // THE FILING IS THE SPEAKER'S AND IT DOES NOT CLOSE THE CIRCUIT (54.5,
    // as ruled 2026-09-20). This asserted `f.coheres()` — that writing "0"
    // in the form's own side-table made a free part cohere. A (3,4,5)
    // rotor has no period; `g^ℓ` is never the identity and the circuit
    // walks. What the zero records is that the stroke READ the class, which
    // is labour, and labour rides `Element.filing`.
    assert!(!f.coheres(), "a free part does not cohere, filed or not");
    let e = as_element_form(&f, [1, 0, 0, 0]).expect("a filed form has a tape");
    // AND THE FILING RIDES THE ELEMENT, NOT THE FORM (df1eeaa). The mint
    // lifts it off the form it was made on; a form decoded from its ADDRESS
    // carries none, because the address is the geometry and a filing is not
    // geometry.
    let filed: Vec<u32> = e.filing().scoped().map(|(_, l)| l.period).collect();
    assert_eq!(filed, vec![0], "the period reads 0, infinite order, on the speaker's word");
    let read = from_element_form(&e).expect("the address rebuilds the form");
    assert!(
        read.torsion.is_empty(),
        "and the decoded form files nothing: `from_address` recovers geometry alone"
    );
    assert_eq!(
        read.fibre_volume(e.filing()),
        0,
        "an infinite class reaches no members, whatever is filed for it"
    );
}

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
        stalk::Invariant::counts(&[3, 5]),
        Box::new(element::carriage(&q1(), &(q0(), q1()))),
        vec![(1, 2)],
    ));
    let unfolded = f.unfold(Measure::Occupied);
    let demand = f.vertices.len() as u16;
    f.vertices.push(Vertex::Demand {
        kind: Demand::Source { multiplicity: 2 },
        chart: stalk::Invariant::counts(&[1, 4, 4]),
        bodies: vec![(9, 9, 0, 1), (9, 10, 0, 1)],
    });
    // A FACE CARRYING AN EDGE AT ONE MEMBER PAIR. `FormFace::new` records the
    // word alone, so the pair is written here with its edge on it.
    let mut edged = FormFace::new(1, glued, w.clone());
    edged.at.insert(
        crate::transducer::sentence::face_pair_key(
            &stalk::Invariant::default(),
            &stalk::Invariant::counts(&[1, 4, 4]),
        ),
        crate::transducer::sentence::FacePlace {
            carriage: element::identity_versor(),
            gauge: Vec::new(),
            edge: Some(Transport::from_word(w.clone())),
        },
    );
    f.faces.push(edged);
    f.loops = Form::loops_of(&f.faces, &f.vertices);
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
        (Vertex::Glued { depth: da, .. }, Vertex::Glued { depth: db, .. }) => {
            assert_eq!(da, db);
        }
        _ => panic!("glued vertex"),
    }
    // 54.7 B5a — AND THE COVER COMES BACK WITH IT. The target left the
    // vertex; the incidence is the form's, one edge per citation.
    assert_eq!(f.cover, back.cover, "the cover round-trips whole");
    match (&back.vertices[rebased as usize], &f.vertices[rebased as usize]) {
        (Vertex::Rebased { by, of, acts }, Vertex::Rebased { acts: was, .. }) => {
            assert_eq!(*by, 0);
            assert_eq!(*of, 1);
            // THE WHOLE FIBRE ROUND-TRIPS, keyed by chart: the relative and the
            // hue map at each chart `of` acted on, and one member here.
            assert_eq!(acts, was, "every member comes back filed at its own chart");
            let m = acts
                .get(stalk::Invariant::counts(&[3, 5]).key().as_bytes())
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
        back.demands.get(&demand_key(&Demand::Source { multiplicity: 2 }, &stalk::Invariant::counts(&[1, 4, 4]))),
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
    let chart = stalk::Invariant::counts(&[1, 4, 4]);
    let other = stalk::Invariant::counts(&[1, 6, 2]);
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
    let chart = stalk::Invariant::counts(&[1, 4, 4]);
    let other = stalk::Invariant::counts(&[1, 6, 2]);
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

/// 1.5 AND 1.7 — A CITATION IS THE CITED FORM'S ADDRESS, WRITTEN WHOLE,
/// AND WHAT IS CHECKED IS THAT IT RE-DERIVES.
///
/// This test asserted, for four commits, that the tape held a cited
/// address exactly ONCE — a reading of where bytes sit, defending a
/// tape-local table of addresses named by ordinal. 1.5 is titled
/// "byte-exactness is not identity" and says what is checked "is never
/// byte exactness; it is the mathematics adding up"; rule 15 says a
/// position is not a coordinate. The table and the byte count are gone and
/// the version is spent.
///
/// What remains is the property that was always the right one: identity is
/// not shortened, so the citing form's KEY carries the cited form's whole
/// address, and a NESTED citation — a form gluing a form that glues a form
/// — decodes to the address it cited and re-mints to the same address,
/// which is A21 §4's decode property at the one kind the engine is made of.
///
/// The repetition a tape can carry — one word gluing one form at many
/// vertices — is real and is not the encoder's to shorten: 53.4's
/// quantified citation removes it in kind, one citation per fiber (K2).
#[test]
fn a_citation_is_the_cited_address_and_it_re_derives() {
    use crate::transducer::sentence::{as_element_form, from_element_form, Vertex};
    let q = alphabet::LetterId::quarter((0, 1)).expect("quarter");
    let w = Word::from_letters(&[(q, false)]);
    let inner = closed_word_form(w.clone(), None);
    let inner_addr = as_element_form(&inner, [1, 0, 0, 0]).expect("inner").address();

    // THREE CITATIONS OF ONE FORM AT THREE PLACES. They were three at ONE
    // place — same address, same chart, same depth, same identity
    // placement and transition — which §1.5 makes ONE vertex, so the form
    // held a citation three times and said nothing a form holding it once
    // does not. The address collapsed them and the tape did not, and this
    // test read the tape.
    //
    // What distinguishes a citation is its own coordinates, and the
    // placement is one of them: the cited form stands at three different
    // places inside the host, which is three boundaries and three
    // citations.
    let mut host = Form::default();
    let places = [Word::identity(), order4(), order2()];
    for p in &places {
        host.glue(&inner, inner_addr.clone(), p.clone(), [0, 0, 0, 1])
            .expect("glue the same form at another place");
    }
    let glued = host
        .vertices
        .iter()
        .filter(|v| matches!(v, Vertex::Glued { .. }))
        .count();
    assert_eq!(glued, 3, "three glued vertices, got {glued}");

    // IDENTITY IS NOT SHORTENED (1.7) — AND IT IS THE COVER THAT CARRIES
    // IT (54.7 B5a). The cited form's whole address stands on every edge,
    // unshortened and undigested; what the KEY carries is this chart's own
    // boundary, because a chart does not know what it overlaps.
    //
    // The eviction is why: with the target inline, `key(A) ⊃ key(B)` gives
    // `|A| > |B|` for every citation, so by well-ordering no cycle
    // `A → … → A` could exist and the manifold was a directed acyclic
    // forest by its naming scheme alone.
    assert_eq!(host.cover.len(), 3, "three boundaries, three incidences");
    assert!(
        host.cover.iter().all(|e| e.target == inner_addr),
        "the cover carries the cited coordinates whole, on every edge"
    );
    let key = crate::transducer::sentence::form_vertex_key(&host);
    assert!(
        !key.as_bytes()
            .windows(inner_addr.len())
            .any(|x| x == inner_addr.as_bytes()),
        "and the key carries none of them: a citation costs the boundary, not the form"
    );

    // AND THE TAPE RE-DERIVES: the host decodes to a form that cites the
    // same address and re-mints to the same one.
    let e = as_element_form(&host, [1, 0, 0, 0]).expect("the host mints");
    let back = from_element_form(&e).expect("the tape round trips");
    let cited: Vec<crate::transducer::address::Address> =
        back.cover.iter().map(|e| e.target.clone()).collect();
    assert_eq!(cited.len(), 3, "every citation comes back");
    assert!(cited.iter().all(|a| *a == inner_addr), "each is the address cited");

    // AND THE SAME FORM CITED THREE TIMES AT ONE PLACE IS ONE CITATION.
    // The three differ in nothing — one address, one chart, one depth, one
    // placement, one transition — so §1.5 makes them one vertex and the
    // address says so. This is the quotient the key lives on, and it is
    // what 53.4's quantified citation removes in kind (K2).
    let mut once = Form::default();
    for _ in 0..3 {
        once.glue(&inner, inner_addr.clone(), Word::identity(), [0, 0, 0, 1])
            .expect("glue the same form at the same place");
    }
    let one = from_element_form(&as_element_form(&once, [1, 0, 0, 0]).expect("mints"))
        .expect("decodes");
    assert_eq!(
        one.vertices.iter().filter(|v| matches!(v, Vertex::Glued { .. })).count(),
        1,
        "one boundary mating under one transition is one citation, however often it was written"
    );
    assert_eq!(
        as_element_form(&back, [1, 0, 0, 0]).map(|x| x.address()),
        Some(e.address()),
        "re-minting the decoded form reaches the same address (A21 §4, 1.5)"
    );

    // NESTED: a form that glues a form that glues a form.
    let host_addr = e.address();
    let mut nested = Form::default();
    nested
        .glue(&host, host_addr.clone(), Word::identity(), [0, 0, 0, 1])
        .expect("glue the host, which itself glues");
    let ne = as_element_form(&nested, [1, 0, 0, 0]).expect("the nested form mints");
    let nback = from_element_form(&ne).expect("a nested tape round trips");
    assert!(
        nback.cover.iter().any(|e| e.target == host_addr),
        "the nested citation decodes to the address it cited, whole"
    );
    assert_eq!(
        as_element_form(&nback, [1, 0, 0, 0]).map(|x| x.address()),
        Some(ne.address()),
        "and the nested form re-derives its address too"
    );
}
