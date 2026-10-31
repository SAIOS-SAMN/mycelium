// 19 — THE FORM'S ADDRESS CODEC (TN1; was `tests/tape.rs`): a form
// round-trips its v4 tape exactly, and a short or partial tape is refused
// rather than partly read.

use super::common::*;
use crate::network::nerve::coord;
use crate::transducer::sentence::ToralPeriod;
use crate::transducer::alphabet::{self, Word};
use crate::transducer::element;
use crate::transducer::sentence::*;
use crate::transducer::stalk::{self};

/// 52.3 AS AMENDED (54.5, ruled 2026-09-20) — A FILED ZERO IS LABOUR, NOT
/// CLOSURE. An orbit whose edge is a (3,4,5) rotor files at period 0, A19's
/// spelling of infinite order; the filing rides the SPEAKER'S word and the
/// form does not cohere on it, because a circuit that never returns is not
/// closed by anyone writing down that it never returns.
#[test]
fn as_element_form_carries_filed_zero_as_open() {
    let w = infinite();
    let mut f = edge_chord_form(std::slice::from_ref(&w));
    let orbits = f.orbits.clone();
    let [lp] = orbits.as_slice() else { panic!("one chord closes one orbit") };
    let classes: Vec<_> = f.edge_classes(lp).scoped().map(|(_, c)| c.clone()).collect();
    let [class] = classes.as_slice() else { panic!("one member path, so one class") };
    f.torsion.put(
        class,
        FiledOrbit { sigma: Vec::new(), periods: ToralPeriod::OPEN, chord: Closes::On(lp.chord.clone()) },
    );
    // THE FILING IS THE SPEAKER'S AND IT DOES NOT CLOSE THE CIRCUIT (54.5,
    // as ruled 2026-09-20). This asserted `f.coheres()` — that writing "0"
    // in the form's own side-table made a free part cohere. A (3,4,5)
    // rotor has no period; `g^ℓ` is never the identity and the circuit
    // never returns. What the zero records is that the stroke READ the class, which
    // is labour, and labour rides `Element.filing`.
    assert!(!f.coheres(), "a free part does not cohere, filed or not");
    let e = as_element_form(&f, [1, 0, 0, 0]).expect("a filed form has a tape");
    // AND THE FILING RIDES THE ELEMENT, NOT THE FORM (df1eeaa). The mint
    // lifts it off the form it was made on; a form decoded from its ADDRESS
    // carries none, because the address is the geometry and a filing is not
    // geometry.
    let filed: Vec<ToralPeriod> = e.filing().scoped().map(|(_, l)| l.periods).collect();
    assert_eq!(
        filed,
        vec![ToralPeriod::OPEN],
        "neither plane closes, on the speaker's word — a (3,4,5) rotor has no period at all"
    );
    let read = from_element_form(&e).expect("the address rebuilds the form");
    assert!(
        read.torsion.is_empty(),
        "and the decoded form files nothing: `from_address` recovers geometry alone"
    );
    assert_eq!(
        read.fibre_volume(e.filing()),
        Some(0),
        "an infinite class reaches no members, whatever is filed for it"
    );
}

/// SENTENCE_TAG v6 round-trips Frame, Word (with contact), Glued,
/// Rebased, Unfolded, Demand (50.5), faces, each with its edge transport
/// (50.3), and orbits.
#[test]
fn from_element_form_round_trips_every_vertex_kind() {
    let q = alphabet::LetterId::turn_90((0, 1)).expect("90° turn");
    let w = Word::from_letters(&[(q, false)]);
    let inner = closed_word_form(w.clone(), None);
    assert!(inner.is_closed(), "a tree of two vertices coheres");
    assert!(as_element_form(&inner, [0; 4]).is_none(), "zero residual is not spoken");
    let inner_e = as_element_form(&inner, [1, 0, 0, 0]).expect("inner tape");
    let inner_addr = inner_e.address();

    let mut f = closed_word_form(
        w.clone(),
        Some(alphabet::Contact { anchor: 1, ring: 2 }),
    );
    let glued = f
        .glue(&inner, inner_addr, Word::identity(), [0, 0, 0, 1])
        .expect("glue depth 0 into depth 0");
    // AT1 — A VERTEX IS REACHED BY ITS COORDINATE: the word by its class,
    // the rebase by its base's class then the class it held (ruled).
    let word = only_word(&f);
    let word_class = f.vertices.at(&word).and_then(Vertex::class).expect("the word's class");
    let rebased = f
        .put_vertex(Vertex::rebased_on(
            coord::VertexAt::Frame,
            word_class.clone(),
            stalk::Invariant::counts(&[3, 5]),
            Box::new(element::carriage(&q1(), &(q0(), q1()))),
            vec![(1, 2)],
            [FRAME],
        ))
        .expect("a rebase on the Frame has a coordinate");
    let unfolded = f.unfold(Measure::Occupied).expect("an unfolded vertex has a coordinate");
    let demand = f
        .put_vertex(Vertex::demand(
            Demand::Source { multiplicity: 2 },
            stalk::Invariant::counts(&[1, 4, 4]),
            vec![(9, 9, 0, 1), (9, 10, 0, 1)],
        ))
        .expect("a demand has a coordinate");
    // AN OVERLAP CARRYING AN EDGE, from the word's chart to the citation's
    // (AT1 Q2: one member pair at one frame, the edge on its place).
    let glued_chart = coord::ChartAt::at(
        glued.clone(),
        f.vertices.at(&glued).map(Vertex::charts).and_then(|c| c.into_iter().next()).expect("the citation's chart"),
        FRAME,
    );
    let word_chart = coord::ChartAt::at(word.clone(), stalk::Invariant::counts(&[3, 5]), FRAME);
    let edged = edge_chord(&mut f, &word_chart, &glued_chart, Transport::from_word(w.clone()));
    let star = coord::Overlap { from: coord::ChartAt::Frame, to: word_chart };
    assert!(f.faces.holds(&star), "the fixture's star overlap");
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    f.remainder = crate::transducer::element::place_chain(&[(9, 9, 1, 0)])
        .expect("one link is A5's chain");
    f.reindex();
    assert!(matches!(f.vertices.at(&glued), Some(Vertex::Glued { .. })));
    assert!(matches!(
        f.vertices.at(&rebased),
        Some(Vertex::Rebased { by: coord::VertexAt::Frame, class, .. }) if *class == word_class
    ));
    assert!(matches!(
        f.vertices.at(&unfolded),
        Some(Vertex::Unfolded {
            measure: Measure::Occupied
        })
    ));

    let e = as_element_form(&f, [1, 0, 0, 0]).expect("mint");
    assert!(e.is_sentence(), "by its address's own TAG_FORM (54.7 B6)");
    assert!(e.description().is_empty(), "and it carries no tape to name a version");
    let back = crate::transducer::sentence::from_element_form(&e).expect("decode");
    assert_eq!(back.vertices.len(), f.vertices.len());
    assert_eq!(back.faces.len(), f.faces.len());
    assert_eq!(back.orbits.len(), f.orbits.len());
    // The remainder rides the tape as it sits in the scene, so the chain
    // and the placement both survive the round trip.
    assert_eq!(back.remainder.placed(), f.remainder.placed());
    assert_eq!(back.remainder.chain(), f.remainder.chain());
    assert_eq!(back.depth, f.depth);
    match (f.vertices.at(&word), back.vertices.at(&word)) {
        (
            Some(Vertex::Word { derivation: a, acts: aa, contact: ca_c, .. }),
            Some(Vertex::Word { derivation: b, acts: ab, contact: cb_c, .. }),
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
    match (f.vertices.at(&glued), back.vertices.at(&glued)) {
        (Some(Vertex::Glued { depth: da, .. }), Some(Vertex::Glued { depth: db, .. })) => {
            assert_eq!(da, db);
        }
        _ => panic!("glued vertex"),
    }
    // 54.7 B5a — AND THE COVER COMES BACK WITH IT. The target left the
    // vertex; the incidence is the form's, one edge per citation.
    assert_eq!(f.cover, back.cover, "the cover round-trips whole");
    match (back.vertices.at(&rebased), f.vertices.at(&rebased)) {
        (Some(Vertex::Rebased { by, class, acts }), Some(Vertex::Rebased { acts: was, .. })) => {
            assert_eq!(*by, coord::VertexAt::Frame);
            assert_eq!(*class, word_class, "the class it held before the rebase");
            // THE WHOLE FIBRE ROUND-TRIPS, keyed by chart: the relative and the
            // hue map at each chart `of` acted on, and one member here.
            assert_eq!(acts, was, "every member comes back filed at its own chart");
            // A FIBRE READ AT THE CHART'S STRATUM (54.7 R1): the key is
            // `chart ‖ carriage`, so the chart is a prefix.
            let prefix = crate::transducer::sentence::Acting::chart_prefix(
                &stalk::Invariant::counts(&[3, 5]),
            );
            let m = acts
                .scoped()
                .find(|(k, _)| k.starts_with(&prefix))
                .map(|(_, x)| x.clone())
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
        back.vertices.at(&unfolded),
        Some(&Vertex::Unfolded {
            measure: Measure::Occupied
        })
    );
    // THE STAR OVERLAP comes back at its own key, Frame first, its word
    // derived as it was.
    assert!(back.faces.holds(&star), "the star overlap comes back at its key");
    assert_eq!(back.face_word(&star), f.face_word(&star));
    // THE OVERLAP'S PLACE RIDES THE TAPE: the placement at its member pair
    // and the edge where it has one.
    assert_eq!(
        back.faces.at(&star).map(|x| &x.place),
        f.faces.at(&star).map(|x| &x.place),
        "the overlap's place comes back at its own key"
    );
    assert!(
        back.faces.at(&star).is_some_and(|x| !x.has_edge()),
        "a face with no measured edge transport is a remainder edge"
    );
    assert_eq!(
        back.faces.at(&edged).and_then(|x| x.place.edge.as_ref()).map(|t| t.word.clone()),
        Some(w.clone()),
        "the edge transport rides the overlap at its member pair"
    );
    match (f.vertices.at(&demand), back.vertices.at(&demand)) {
        (
            Some(Vertex::Demand { kind: ka, chart: ca, bodies: ba, .. }),
            Some(Vertex::Demand { kind: kb, chart: cb, bodies: bb, .. }),
        ) => {
            assert_eq!(ka, kb);
            assert_eq!(ca, cb);
            assert_eq!(ba, bb, "the bodies a demand stands for ride the tape");
        }
        _ => panic!("demand vertex"),
    }
    assert!(!back.is_closed(), "a sentence with a demand vertex is not closed");
    let (want_kind, want_chart) = (Demand::Source { multiplicity: 2 }, stalk::Invariant::counts(&[1, 4, 4]));
    let by_kind_chart: Vec<coord::VertexAt> = back
        .demand_vertices()
        .filter(|(_, k, c, _)| **k == want_kind && **c == want_chart)
        .map(|(at, ..)| at)
        .collect();
    assert_eq!(by_kind_chart, vec![demand.clone()], "the demand is read by (kind, chart)");
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
fn demand_key_separates_kind_and_chart_refuses_empty() {
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
fn stand_demand_gathers_bodies_per_kind_and_chart() {
    let chart = stalk::Invariant::counts(&[1, 4, 4]);
    let other = stalk::Invariant::counts(&[1, 6, 2]);
    let mut f = Form::default();
    f.stand_demand(Demand::Creation, &chart, vec![(0, 0, 0, 3)]);
    f.stand_demand(Demand::Creation, &chart, vec![(0, 1, 0, 3), (0, 0, 0, 3)]);
    f.stand_demand(Demand::Creation, &other, vec![(5, 5, 0, 1)]);
    let demanded: Vec<(Demand, Vec<Link>)> =
        f.demand_vertices().map(|(_, k, _, links)| (k.clone(), links.to_vec())).collect();
    assert_eq!(demanded.len(), 2, "one vertex per (kind, chart), got {demanded:?}");
    // AT1 — THE DEMAND STANDS AT ITS OWN COORDINATE, in the Demand stratum
    // of the vertices, and is read there by its kind and chart.
    let on_chart: Vec<_> = f
        .demand_vertices()
        .filter(|(_, k, c, _)| **k == Demand::Creation && **c == chart)
        .map(|(at, ..)| at)
        .collect();
    let [on_chart] = on_chart.as_slice() else { panic!("one demand on the chart, got {on_chart:?}") };
    let Some(Vertex::Demand { bodies, .. }) = f.vertices.at(on_chart) else {
        panic!("the index names the demand vertex")
    };
    assert_eq!(bodies, &vec![(0, 0, 0, 3), (0, 1, 0, 3)], "the bodies are gathered, sorted, once each");
}

/// 54.7 B6 — A TRUNCATED KEY REFUSES, and that is where the reading now
/// lives.
///
/// `tape_v4_refuses_short_or_partial` truncated `Element.description` and
/// asked the tape decoder to refuse it. A sentence has no description; the
/// carrier that can arrive short is the ADDRESS, and a key missing its tail
/// must yield no form rather than a form missing its tail. Same claim, one
/// carrier over.
#[test]
fn from_address_refuses_truncated_key() {
    let f = closed_word_form(Word::identity(), None);
    let key = crate::transducer::sentence::form_vertex_key(&f);
    assert!(
        crate::transducer::sentence::from_address(&key).is_some(),
        "the whole key decodes"
    );
    let bytes = key.as_bytes();
    for cut in [bytes.len() - 1, bytes.len() / 2, 1] {
        let short = crate::transducer::address::Address::new(bytes[..cut].to_vec());
        assert!(
            crate::transducer::sentence::from_address(&short).is_none(),
            "a key cut at {cut} of {} decodes to nothing, never to a shorter form",
            bytes.len()
        );
    }
}

/// A sentence round-trips its nerve: vertices, faces and orbits survive the
/// element form.
#[test]
fn from_element_form_round_trips_nerve() {
    let f = star_form(2, Vec::new());
    let e = as_element_form(&f, [1, 0, 0, 0]).expect("form element");
    assert!(e.is_sentence());
    let back = crate::transducer::sentence::from_element_form(&e).expect("the form survives the tape");
    assert_eq!(back.vertices.len(), f.vertices.len());
    assert_eq!(back.faces.len(), f.faces.len());
    assert_eq!(back.orbits.len(), f.orbits.len());
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
/// quantified citation removes it in kind, one citation per fibre (K2).
#[test]
fn glue_cites_whole_address_and_re_derives() {
    use crate::transducer::sentence::{as_element_form, from_element_form, Vertex};
    let q = alphabet::LetterId::turn_90((0, 1)).expect("90° turn");
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
    let places = [Word::identity(), word_turn_90(), word_turn_180()];
    for p in &places {
        host.glue(&inner, inner_addr.clone(), p.clone(), [0, 0, 0, 1])
            .expect("glue the same form at another place");
    }
    let glued = host.glued_vertices().filter(|(_, v)| matches!(v, Vertex::Glued { .. })).count();
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
        one.glued_vertices().filter(|(_, v)| matches!(v, Vertex::Glued { .. })).count(),
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

/// STD1 3c-2, ruled (d) — A KEY COMPONENT'S LENGTH IS ITS OWN COORDINATE.
/// Any length round-trips its LEB128 prefix exactly — across every width a
/// fixed field would have walled — and a short component takes one length
/// byte. Two distinct components, or one split differently, never share a
/// key; and an over-long spelling of a length is no key at all.
#[test]
fn put_key_bytes_round_trips_any_length_without_collision() {
    use crate::transducer::address::{put_key_bytes, take_key_bytes};
    for n in [0usize, 1, 127, 128, 16_383, 16_384, 70_000] {
        let comp = vec![0xabu8; n];
        let mut k = Vec::new();
        put_key_bytes(&mut k, &comp);
        let mut p = 0;
        assert_eq!(take_key_bytes(&k, &mut p), Some(comp), "length {n} round-trips");
        assert_eq!(p, k.len(), "and consumes its key exactly");
    }
    let mut short = Vec::new();
    put_key_bytes(&mut short, b"chart");
    assert_eq!(short.len(), 1 + 5, "a component under 128 bytes takes one length byte");

    let key = |parts: &[&[u8]]| {
        let mut k = Vec::new();
        for part in parts {
            put_key_bytes(&mut k, part);
        }
        k
    };
    assert_ne!(key(&[b"ab", b"c"]), key(&[b"a", b"bc"]), "a split is part of the key");
    assert_ne!(key(&[b"ab"]), key(&[b"ab", b""]), "an empty component is a component");
    assert_ne!(key(&[&[0u8; 128]]), key(&[&[0u8; 127]]), "neighbouring lengths differ");

    // 0x80 0x00 spells zero over-long: refused, so a key has one spelling.
    let mut p = 0;
    assert_eq!(take_key_bytes(&[0x80, 0x00], &mut p), None);
}
