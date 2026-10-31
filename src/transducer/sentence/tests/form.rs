// 18.2 — THE FORM: vertices, faces by constancy, glue and retract, depth,
// and what a chain deconstructs into.

use super::common::*;
use crate::transducer::sentence::ToralPeriod;
use crate::network::nerve::{coord, Coordinate, Keyed};
use crate::transducer::alphabet::{self, Word};
use crate::transducer::blade::Shape;
use crate::transducer::element::{self, from_canonical, Element};
use crate::transducer::germinate::germinate;
use crate::transducer::sentence::*;
use crate::transducer::stalk::{self, Stalks};

/// 50.3 — THE EDGE HOLONOMY DOES NOT TELESCOPE. The face transports of an
/// orbit are the coboundary of the placements and compose to the identity
/// by construction; the edge transports are stabilizer elements, and two
/// 90° turns around a triangle compose to the 180° turn, a class the
/// orbit holds until it is filed. An orbit through a remainder edge is not
/// evaluated.
#[test]
fn edge_holonomies_do_not_telescope() {
    let qw = Word::from_letters(&[(alphabet::LetterId::turn_90((0, 1)).unwrap(), false)]);
    let mut f = Form::default();
    // AT1 — THREE BODIES OF ONE WORD ARE ONE VERTEX WITH THREE MEMBERS (Q1),
    // so the three stand at three charts: two members on one chart at one
    // frame would be one chart of the atlas, and the triangle would have no
    // corners to close on.
    let mut charts = Vec::new();
    for k in 1..=3u8 {
        let v = Vertex::word_on(
            alphabet::Derivation::spelled(Word::identity()),
            stalk::Invariant::counts(&[u64::from(k)]),
            Box::new(element::carriage(
                &q1(),
                &(q0(), num_rational::BigRational::from_integer(num_bigint::BigInt::from(i64::from(k)))),
            )),
            Vec::new(),
            [FRAME],
            None,
        );
        charts.push(f.stand_member(&v, FRAME).expect("a word vertex stands"));
    }
    // The faces among the bodies only: the orbit 1-2-3 closes on the chord
    // 1-3 without passing the Frame.
    let overlap = |f: &mut Form, i: usize, j: usize| {
        f.place_overlap(charts[i].clone(), charts[j].clone(), None).expect("an overlap between two members")
    };
    let (e12, e23, e13) = (overlap(&mut f, 0, 1), overlap(&mut f, 1, 2), overlap(&mut f, 0, 2));
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    // THE ORBIT STANDS BEFORE ITS EDGES ARE WRITTEN. Its faces are real —
    // between bodies, not star — and carry no edge yet, so every member
    // path drops on a remainder edge and the orbit is UNREAD rather than
    // flat; a trivial-cycle filter that read an empty edge answer as a flat
    // one would prune it (rule 11).
    assert_eq!(f.orbits.len(), 1, "one orbit among three bodies");
    assert!(f.coheres(), "the coboundary of the placements telescopes: every orbit is the identity");
    // THE EDGE IS SET ON THE OVERLAP. An overlap relates ONE member pair at
    // one frame (AT1 Q2), so the orbit has one member path — which is what
    // lets this read a single holonomy below.
    let set_edge = |f: &mut Form, o: &coord::Overlap, edge: Option<Transport>| {
        f.faces.at_mut(o).expect("the overlap is held").place.edge = edge;
    };
    set_edge(&mut f, &e12, Some(Transport::from_word(qw.clone())));
    set_edge(&mut f, &e23, Some(Transport::from_word(qw.clone())));
    set_edge(&mut f, &e13, Some(Transport::from_word(Word::identity())));
    // A face write invalidates the orbits — `apply_move` says so in its own
    // line — and the cycle stands now that its edges compose to a 180° turn.
    f.orbits = Form::orbits_of(&f.faces, &f.vertices);
    let [triangle, ..] = f.orbits.as_slice() else { panic!("the orbit 1-2-3 among the bodies") };
    // ONE MEMBER PATH, so one holonomy — and the slice pattern says so rather
    // than taking the first of several.
    let one_holonomy = |f: &Form, lp: &Orbit| -> Option<Transport> {
        let hs = f.edge_holonomies(lp)?;
        let [(_, h)] = hs.scoped().collect::<Vec<_>>()[..] else {
            return None;
        };
        Some(h.clone())
    };
    let h = one_holonomy(&f, triangle).expect("every edge of the orbit is carried");
    let turn_180 = qw.compose(&qw);
    assert!(h.word.rotor().projective_eq(&turn_180.rotor()), "90°, 90°, identity compose to the 180° turn: {:?}", h.word);
    assert_eq!(
        f.edge_classes(triangle).scoped().map(|(_, c)| c.clone()).collect::<Vec<_>>(),
        vec![turn_180.rotor().holonomy_key().expect("the 180° turn has a class")],
        "read up to conjugacy: the 180° turn's class, on the one path"
    );
    // A COMPACT HOLONOMY IS CONTENT AND COHERES (54.5, as ruled
    // 2026-09-20). This read `!f.coheres()` and then filed a period to
    // make it true, which asserted that an ANNOTATION'S PRESENCE changes a
    // geometric property. Every field of a `FiledOrbit` is derivable —
    // `orbit_classes` computes the class, the period through
    // `cech::classify`, σ from the transports — so law 12 forbids storing
    // one beside the coordinates that determine it, and a predicate
    // reading it was asking "has a stroke got around to this orbit".
    //
    // The 180° turn returns after two circuits about a fixed point. That
    // is `ℤ/2ℤ` torsion, and torsion is content (law 7): the orbit coheres
    // BECAUSE it is compact, not because anyone wrote it down.
    assert!(f.coheres(), "a 180° turn returns: ℤ/2ℤ is a compact resonance, and it coheres");
    // AND THE FILING CHANGES NOTHING HERE. It is the speaker's
    // attestation — what its stroke discharged against its own budget —
    // and it prices the yield (E2) and answers whether the stroke
    // finished. Neither is a fact about the connection.
    // THE SPECTRUM THE ROTOR ACTUALLY HAS (54.7), not a scalar chosen for
    // it: a filing that names a period the geometry does not carry is the
    // thing `REFUSE_INFLATED_YIELD` exists to catch, so a fixture must file
    // what the rotor says.
    f.torsion.put(
        &turn_180.rotor().holonomy_key().expect("class"),
        FiledOrbit { sigma: Vec::new(), periods: turn_180.rotor().toral_period(), chord: Closes::On(triangle.chord.clone()) },
    );
    assert!(f.coheres(), "and filing it says nothing the geometry did not already say");
    // NO RE-READ HERE. `edge_holonomies` reads `self.faces` live, so the
    // `Orbit` — its vertices, its faces, its chord — stays valid across an
    // edge write; re-reading the orbits would drop this cycle, because `q, q⁻¹, 1`
    // composes to the identity and a cycle that encloses nothing is not an
    // orbit. The reading being asserted is exactly that identity.
    set_edge(&mut f, &e23, Some(Transport::from_word(qw.inverse())));
    let [triangle, ..] = f.orbits.as_slice() else { panic!("the orbit") };
    let h = one_holonomy(&f, triangle).expect("carried");
    assert!(h.word.rotor().projective_eq(&alphabet::Rotor::identity()), "90°, 90° inverse, identity is the identity");
    set_edge(&mut f, &e13, None);
    let [triangle, ..] = f.orbits.as_slice() else { panic!("the orbit") };
    assert!(
        f.edge_holonomies(triangle).is_some_and(|hs| hs.is_empty()),
        "an orbit through a remainder edge is not evaluated on any path"
    );
    assert!(f.coheres(), "and holds nothing against the form");
}

/// 52.3 AS AMENDED (54.5, ruled 2026-09-20) — AN ORBIT OF ANY ORDER FILES,
/// AND FILING IS NOT COHERING.
///
/// A (3,4,5) rotor on a chord's edge is a holonomy with NO PERIOD: `g^ℓ`
/// is never the identity, so the circuit runs on and never returns. It is a
/// free part — a non-compact obstruction — and what the form owes for it
/// is a `Demand::Dislocation` carrying the slip, not an entry in a
/// side-table.
///
/// This test asserted the opposite for four commits: that filing the class
/// at period 0 made the orbit COHERE. That is procedural compliance wearing
/// geometry's clothes, and it let an open dislocation read as a closed
/// manifold. Every field of a `FiledOrbit` is derivable from the complex
/// (`orbit_classes`), so by law 12 the annotation carries no fact the
/// geometry does not already determine — it records that a stroke looked.
///
/// What remains true, and is what the filing is FOR: the stroke reads the
/// class, files it at 0, and the reading rides `Element.filing` as the
/// speaker's attestation, where it prices the yield and says whether the
/// stroke finished.
#[test]
fn coheres_refuses_infinite_order_orbit_filed_or_not() {
    let w = infinite();
    assert_eq!(w.order(), alphabet::Order::Infinite, "the (3,4,5) rotor has no period");
    let mut f = edge_chord_form(std::slice::from_ref(&w));
    let orbits = f.orbits.clone();
    let [lp] = orbits.as_slice() else { panic!("one chord closes one orbit") };
    assert!(!f.edge_coheres(lp), "an edge of infinite order holds against the form");
    assert!(!f.coheres());
    let classes: Vec<_> = f.edge_classes(lp).scoped().map(|(_, c)| c.clone()).collect();
    let [class] = classes.as_slice() else { panic!("one member path, so one class") };
    // A FILING DOES NOT CLOSE A CIRCUIT THAT NEVER RETURNS (54.5, as ruled
    // 2026-09-20). This filed period 0 — A19's spelling of infinite order
    // — and then asserted the orbit COHERED, which made an open dislocation
    // read as a closed manifold. A (3,4,5) rotor has no period: `g^ℓ` is
    // never the identity, the circuit never returns, and no annotation
    // changes that. Writing "0" in a notebook records that the stroke
    // looked; it does not make the geometry close.
    f.torsion.put(
        class,
        FiledOrbit { sigma: Vec::new(), periods: ToralPeriod::OPEN, chord: Closes::On(lp.chord.clone()) },
    );
    assert!(!f.edge_coheres(lp), "filed or unfiled, a free part is not a compact resonance");
    assert!(!f.coheres());
    // A SPECTRUM THE ORBIT DOES NOT HAVE. `[4, 4]` is an isoclinic 90°
    // turn and this orbit is a free part; naming it is not a filing.
    f.torsion.put(
        class,
        FiledOrbit { sigma: Vec::new(), periods: ToralPeriod::from_pair(4, 4), chord: Closes::On(lp.chord.clone()) },
    );
    assert!(!f.coheres(), "and a period the orbit does not have is not a filing either");

    let mut g = chord_form(std::slice::from_ref(&w));
    let orbits = g.orbits.clone();
    let [gl] = orbits.as_slice() else { panic!("one chord closes one orbit") };
    // THE HOLONOMY IS A READ along the orbit's overlaps (AT1, ruled), never
    // a field held beside them.
    let holonomy = orbit_word_on(&g.faces, &g.vertices, gl).expect("the orbit's word composes");
    assert!(!holonomy.is_identity(), "the transport composite carries the rotor");
    assert!(!g.is_cauterized(), "unfiled on the word, the form is not cauterized");
    g.torsion.put(
        &holonomy.rotor().holonomy_key().expect("a class"),
        FiledOrbit { sigma: Vec::new(), periods: ToralPeriod::OPEN, chord: Closes::On(gl.chord.clone()) },
    );
    assert!(g.is_cauterized(), "filed as open under the holonomy's class, it is");
}

/// The face of a missing vertex is none, while the frame is present.
#[test]
fn face_place_of_missing_vertex_is_none() {
    let f = Form::default();
    // AT1 — A FACE IS AN OVERLAP OF TWO CHARTS, and its place is read off
    // the two ends' vertices: a chart of a vertex the form does not hold
    // places nothing.
    let missing = super::common::chart_of(&word_vertex_on(5, word_turn_90(), stalk::Invariant::counts(&[5])), stalk::Invariant::counts(&[5]));
    assert!(
        f.face_place(&coord::ChartAt::Frame, &missing, None).is_none(),
        "a missing vertex is not the identity word"
    );
    assert!(
        f.face_place(&coord::ChartAt::Frame, &coord::ChartAt::Frame, None).is_some(),
        "the frame is present"
    );
}

/// Glue and retract remap the faces, and a tampered placement refuses at
/// replay.
#[test]
fn glue_retract_remaps_faces_and_placement_refuses() {
    let w = Word::from_letters(&[(alphabet::LetterId::turn_90((0, 1)).unwrap(), false)]);
    let mut host = Form::default();
    // AT1 — TWO WORD VERTICES ARE TWO CLASSES (Q1): two vertices of one word
    // are one vertex, and retracting it would retract both. The second is
    // the 180° turn, so one of two can be retracted and one remain.
    let v1 = Vertex::word_on(
        alphabet::Derivation::spelled(w.clone()),
        stalk::Invariant::default(),
        Box::new(element::carriage(&q1(), &(q0(), q0()))),
        Vec::new(),
        [FRAME],
        None,
    );
    let v2 = Vertex::word_on(
        alphabet::Derivation::spelled(word_turn_180()),
        stalk::Invariant::default(),
        Box::new(element::carriage(&q1(), &(q0(), q1()))),
        Vec::new(),
        [FRAME],
        None,
    );
    let c1 = stand_with_star(&mut host, &v1);
    let c2 = stand_with_star(&mut host, &v2);
    host.place_overlap(c1.clone(), c2, None).expect("1-2");
    let inner = closed_word_form(w.clone(), None);
    let inner_e = as_element_form(&inner, [1, 0, 0, 0]).expect("inner");
    let gi = host
        .glue(&inner, inner_e.address(), w.clone(), [1, 0, 0, 0])
        .expect("glue");
    // 54.7 B5a — A CITATION IS KEYED BY ITS SITE, and the site is what the
    // vertex holds; the cited address is the cover's. AT1: the site IS the
    // vertex's coordinate.
    let site = coord::Site::decode(&crate::transducer::sentence::vertex_bytes(
        host.vertices.at(&gi).expect("the glued vertex"),
    ))
    .expect("a citation's own boundary record");
    assert_eq!(gi, coord::VertexAt::Glued(site), "the citation stands at its site");
    assert_eq!(
        host.glued_vertices().map(|(k, _)| k).collect::<Vec<_>>(),
        vec![gi.clone()],
        "and the Glued stratum holds it"
    );
    assert!(
        host.cover.iter().any(|e| e.target == inner_e.address()),
        "and the cover names the form the boundary mates with"
    );
    // THE PLACEMENT OVERLAP is the one overlap at the glued vertex, from the
    // Frame's chart.
    let at_glued = host.overlaps_at(&gi);
    let [placement] = at_glued.as_slice() else {
        panic!("glue writes one overlap at the glued vertex, got {}", at_glued.len())
    };
    assert_eq!(placement.from, coord::ChartAt::Frame, "glue writes the Frame→Glued placement overlap");
    assert_eq!(placement.to_vertex(), gi, "to the glued vertex");
    assert!(host.retract_vertices(std::slice::from_ref(&c1.vertex())), "the retracted form's depth reads");
    assert_eq!(host.vertices.len(), 3, "Frame, remaining Word, Glued");
    for (o, face) in host.faces.entries() {
        assert_eq!(
            host.face_place(&o.from, &o.to, face.place.edge.clone()).as_ref(),
            Some(&face.place),
            "every surviving overlap re-derives its place"
        );
    }
    let mut taped = host.clone();
    let mag = [1u64, 0, 0, 0];
    let clean = as_element_form(&taped, mag).expect("tape");
    assert_eq!(crate::court::verdict::blind_replay(&clean, &mut super::common::budget()), Some(crate::court::verdict::REFUSE_GLUED_UNRULED));
    let resolve = |a: &[u8]| (a == inner_e.address().as_bytes()).then(|| inner.clone());
    assert_eq!(crate::court::verdict::blind_replay_with(&clean, &resolve, &mut super::common::budget()), None);
    // THE WORD TAMPER: the placement overlap's derived word made the
    // identity. The word is derived (AT1 Q2), so the only way to write it
    // is a left factor — and a factor no certificate produced is refused at
    // replay (AT1 step 2, ruled: the factor's producer is the certificate
    // whose chord names the overlap).
    let was = taped.face_word(placement).expect("the placement overlap's word");
    assert!(!was.is_identity(), "the placement carries the glue's word");
    let mut worded = taped.clone();
    worded.faces.at_mut(placement).expect("placement overlap").left = was.inverse();
    assert!(worded.face_word(placement).is_some_and(|x| x.is_identity()), "tampered to the identity");
    let bad = as_element_form(&worded, mag).expect("tampered placement word");
    assert_eq!(
        crate::court::verdict::blind_replay_with(&bad, &resolve, &mut super::common::budget()),
        Some(crate::court::verdict::REFUSE_PIVOT_INEXACT),
        "a factor no pivot produced refuses"
    );
    // THE PLACE TAMPER: the measured place moved off what the two ends
    // derive — the face no longer re-derives from its vertices.
    taped.faces.at_mut(placement).expect("placement overlap").place.carriage = element::translator_versor(1, 0);
    let bad = as_element_form(&taped, mag).expect("tampered placement");
    assert_eq!(
        crate::court::verdict::blind_replay_with(&bad, &resolve, &mut super::common::budget()),
        Some(crate::court::verdict::REFUSE_FACE),
        "a tampered placement face refuses"
    );
}

/// The bundle's agreed shift is meaning: two translations of one linear
/// block are two vertices.
#[test]
fn as_element_form_separates_two_shifts() {
    let letter = alphabet::LetterId::turn_90((0, 1)).expect("90° turn");
    let w = Word::from_letters(&[(letter, false)]);
    let mut a = closed_word_form(w.clone(), None);
    let mut b = closed_word_form(w, None);
    let (wa, wb) = (only_word(&a), only_word(&b));
    a.vertices.at_mut(&wa).expect("the word vertex").set_carriage(element::carriage(&q1(), &(q0(), q1())));
    b.vertices.at_mut(&wb).expect("the word vertex").set_carriage(element::carriage(
        &q1(),
        &(
            q0(),
            num_rational::BigRational::from_integer(num_bigint::BigInt::from(3)),
        ),
    ));
    let ea = as_element_form(&a, [1, 0, 0, 0]).expect("a");
    let eb = as_element_form(&b, [1, 0, 0, 0]).expect("b");
    assert_ne!(ea.address(), eb.address(), "shift is not quotiented off the vertex");
    assert!(!ea.same_vertex(&eb));
}

/// The bundle's frame ratio and its sections are meaning: they are on the
/// form key, and a key with a zero denominator is refused rather than
/// repaired.
///
/// 54.7 B6 — THE TAMPER MOVED WITH THE CARRIER. It zeroed two bytes of
/// `Element.description` at a fixed offset from its end and asked the tape
/// decoder to refuse them. A sentence has no tape, so the offset indexed
/// into nothing; and a byte offset into a buffer was never the reading —
/// the claim is that a ratio whose denominator is zero is not a bundle,
/// wherever it is written. Put on the FORM, it needs no offset at all and
/// asks the one decoder that remains.
#[test]
fn form_vertex_key_carries_ratio_and_sections() {
    let letter = alphabet::LetterId::turn_90((0, 1)).expect("90° turn");
    let w = Word::from_letters(&[(letter, false)]);
    let a = closed_word_form(w.clone(), None);
    let mut b = closed_word_form(w.clone(), None);
    b.ratio = ((2, 1), (2, 1));
    let mut c = closed_word_form(w.clone(), None);
    // 54.7 — A SECTION IS FILED AT A SECTION COORDINATE, not at arbitrary
    // bytes. It stood at `form_addr(7)`, two bytes that spell no stratum of
    // this system, and the carrier accepted them because it took bytes.
    let class = coord::SectionAt::Class(coord::Transport {
        kind: crate::transducer::peel::ShellKind::Whole,
        class: w.rotor().holonomy_key().expect("the 90° turn has a class"),
    });
    // AT1 — A MEMBER HOLDS ITS FRAMES, and the sections are read off them:
    // the word's member stands at frame 1 as well as the fixture's frame.
    let word = c.vertices.at(&only_word(&c)).cloned().expect("the word vertex");
    c.stand_member(&word, 1).expect("the word stands at frame 1");
    let ea = as_element_form(&a, [1, 0, 0, 0]).expect("a");
    let eb = as_element_form(&b, [1, 0, 0, 0]).expect("b");
    let ec = as_element_form(&c, [1, 0, 0, 0]).expect("c");
    assert_ne!(ea.address(), eb.address(), "the frame ratio is on the key");
    assert_ne!(ea.address(), ec.address(), "a section is on the key");
    let back_b = crate::transducer::sentence::from_element_form(&eb).expect("v2 tape");
    assert_eq!(back_b.ratio, ((2, 1), (2, 1)));
    let back_c = crate::transducer::sentence::from_element_form(&ec).expect("v2 tape");
    assert_eq!(
        back_c.sections().at(&class).map(frames_of_section),
        Some(vec![FRAME, 1]),
        "the section round-trips as the frames it stands on"
    );
    // A ZERO HEIGHT DENOMINATOR IS NOT A BUNDLE. `ratio` is `((wn, wd),
    // (hn, hd))` and `hd = 0` is no ratio at all, so the key that carries
    // it does not decode back to a form — refused, never repaired to 1.
    let mut broken = closed_word_form(
        Word::from_letters(&[(alphabet::LetterId::turn_90((0, 1)).expect("90° turn"), false)]),
        None,
    );
    broken.ratio = ((1, 1), (1, 0));
    let key = crate::transducer::sentence::form_vertex_key(&broken);
    assert!(
        crate::transducer::sentence::from_address(&key).is_none(),
        "a zero denominator refuses the key"
    );
}

/// Depth is one plus the deepest glued form — A READING OF THE CITATIONS,
/// and no longer a refusal (54.2, as ruled 2026-09-20).
///
/// This asserted that a citation at equal or greater depth was refused, and
/// read that as acyclicity. It was neither: the refusal compared against
/// the host's depth AS IT STOOD, so it answered by arrival order, and it
/// exempted a host with no citation yet, so two leaves could always cite
/// each other. Charts of an open cover do not own each other; the guard
/// against a cycle is `φ_ij φ_jk φ_ki = 1`, not a posetal height, and
/// `sentence::tests::nerve` carries both readings.
///
/// What remains true, and is what this test is for: `depth_of` is one more
/// than the DEEPEST citation, never a count of them.
#[test]
fn depth_of_is_one_plus_max_glued() {
    let q = alphabet::LetterId::turn_90((0, 1)).expect("90° turn");
    let w = Word::from_letters(&[(q, false)]);
    let inner = closed_word_form(w.clone(), None);
    assert_eq!(inner.depth_of(), Some(0));
    let mut outer = Form::default();
    let a0 = as_element_form(&inner, [1, 0, 0, 0]).expect("inner").address();
    outer.glue(&inner, a0.clone(), Word::identity(), [0, 0, 0, 1]).expect("glue into empty host");
    assert_eq!(outer.depth_of(), Some(1));
    assert_eq!(outer.depth, 1);
    let mut wrap = Form::default();
    wrap.glue(&outer, form_addr(2), Word::identity(), [0, 0, 0, 1]).expect("glue depth 1 into depth 0");
    assert_eq!(wrap.depth_of(), Some(2));
    // A CITATION AT EQUAL DEPTH, AND ONE DEEPER, both stand: the nerve has
    // no stratification and the depth that comes back is the deepest of
    // them, read off the citations themselves.
    let outer_copy = outer.clone();
    outer
        .glue(&outer_copy, form_addr(3), Word::identity(), [0, 0, 0, 1])
        .expect("a citation at equal depth is a citation");
    assert_eq!(outer.depth_of(), Some(2), "the equal-depth citation is one deep, so the host is two");
    outer
        .glue(&wrap, form_addr(4), Word::identity(), [0, 0, 0, 1])
        .expect("a citation of a deeper form is a citation");
    assert_eq!(outer.depth_of(), Some(3), "and the deepest citation is what depth reads");
    let mut again = Form::default();
    again.glue(&inner, a0.clone(), Word::identity(), [0, 0, 0, 1]).unwrap();
    again.glue(&inner, form_addr(5), Word::identity(), [0, 0, 0, 1]).expect("another depth-0 glue");
    assert_eq!(again.depth_of(), Some(1), "max glued depth, not a count");
}

/// A language of a bar and a dot deconstructs a rotated, translated,
/// and dilated bar through its moduli with a verified transport, speaks
/// a chain of two bodies as two words with the words' relative transport,
/// leaves the unknown body as the residual, and refuses to cite a word
/// with itself.
#[test]
fn deconstruct_at_cites_words_and_leaves_residual() {
    let bar = from_canonical(&germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap());
    let dot = from_canonical(&germinate(&[(0, 0, 3u8)], &[(0, 0, 4u8)]).unwrap());
    let bar_other = from_canonical(&germinate(&[(0, 0, 7u8), (0, 1, 7)], &[(0, 0, 8u8), (0, 1, 8)]).unwrap());
    let words = vec![bar, dot, bar_other];
    let atlas = Stalks::over_elements(words.iter());
    // The language AT ITS COORDINATES: a word is reached by its address,
    // which is what the atlas hands the reading.
    let mut at: Keyed<Element> = crate::network::nerve::Keyed::new();
    let mut shapes: Keyed<Option<Shape>> = crate::network::nerve::Keyed::new();
    for w in &words {
        shapes.insert(w.address(), element::shape_of_chain(w.sub()));
        at.insert(w.address(), w.clone());
    }
    let word_at = |a: &[u8]| at.get(a);
    let shape_at = |a: &[u8]| shapes.get(a).and_then(|s| s.as_ref());
    // A vertical bar far away: the bar rotated and translated.
    let target = vec![(5, 5, 1, 2), (6, 5, 1, 2)];
    let s = deconstruct_at(&word_at, &atlas, &shape_at,&target, None, &mut super::common::budget());
    assert_eq!(s.cites.len(), 1);
    assert_eq!(s.cites[0].word, words[0].address(), "the bar, not the other-hued bar");
    assert!(s.is_closed());
    assert!(replay_at(&word_at, &s.cites, &target).is_empty(), "the citation replays to closure");
    assert!(
        !s.multiplicity.is_empty(),
        "stratum width is recorded per body"
    );
    // A 2× dilated bar with the other hues: the other bar at ρ = 2.
    let big = vec![(0, 0, 7, 8), (0, 1, 7, 8), (0, 2, 7, 8), (0, 3, 7, 8), (1, 0, 7, 8), (1, 1, 7, 8), (1, 2, 7, 8), (1, 3, 7, 8)];
    let s = deconstruct_at(&word_at, &atlas, &shape_at,&big, None, &mut super::common::budget());
    assert_eq!(s.cites.len(), 1);
    assert_eq!(s.cites[0].word, words[2].address());
    assert!(s.is_closed());
    // Two bodies and an unknown: two citations with one relative
    // transport between them, one residual body.
    let mixed = vec![(0, 0, 1, 2), (0, 1, 1, 2), (4, 4, 3, 4), (9, 9, 5, 6), (9, 10, 5, 6), (10, 9, 5, 6)];
    let s = deconstruct_at(&word_at, &atlas, &shape_at,&mixed, None, &mut super::common::budget());
    assert_eq!(s.cites.len(), 2);
    assert_eq!(
        s.multiplicity.len(),
        4,
        "the chain and each of bar, dot, residual have a width, never one maximum"
    );
    assert_eq!(s.connection.len(), 1, "one relative transport between two cited bodies");
    assert_eq!(s.residual, vec![(9, 9, 5, 6), (9, 10, 5, 6), (10, 9, 5, 6)]);
    // Excluding the bar itself: a bar chain has no other word of hue 1→2.
    let s = deconstruct_at(&word_at, &atlas, &shape_at,words[0].sub(), Some(words[0].address().as_bytes()), &mut super::common::budget());
    assert!(s.cites.is_empty() && !s.is_closed(), "no word but itself speaks it");
}

/// Deconstruction records its multiplicity per body: the chain and every
/// cited body keeps its own width.
#[test]
fn deconstruct_at_records_multiplicity_per_body() {
    let bar = from_canonical(&germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap());
    let dot = from_canonical(&germinate(&[(0, 0, 3u8)], &[(0, 0, 4u8)]).unwrap());
    let words = vec![bar, dot];
    let atlas = Stalks::over_elements(words.iter());
    let mut at: Keyed<Element> = Keyed::new();
    let mut shapes: Keyed<Option<Shape>> = Keyed::new();
    for w in &words {
        shapes.insert(w.address(), element::shape_of_chain(w.sub()));
        at.insert(w.address(), w.clone());
    }
    let word_at = |a: &[u8]| at.get(a);
    let shape_at = |a: &[u8]| shapes.get(a).and_then(|s| s.as_ref());
    let mixed = vec![(0, 0, 1, 2), (0, 1, 1, 2), (4, 4, 3, 4)];
    let s = deconstruct_at(&word_at, &atlas, &shape_at, &mixed, None, &mut super::common::budget());
    assert_eq!(s.cites.len(), 2);
    assert_eq!(
        s.multiplicity.len(),
        3,
        "the chain and each cited body have a width"
    );
    let widths: Vec<usize> = s
        .multiplicity
        .scoped()
        .map(|(_, w)| *w)
        .collect();
    assert_eq!(
        widths.iter().filter(|&&w| w >= 1).count(),
        2,
        "each cited body keeps its own stratum width; the chain records 0 when it is not a word: {widths:?}"
    );
}

/// The receiving direction: two citations carrying different words
/// onto the same sites are a seam defect the replay alone would miss;
/// a transport and its inverse are a flat orbit, two translations are
/// not.
#[test]
fn seam_defect_at_and_is_flat_locate_exactly() {
    let bar = from_canonical(&germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap());
    let bar_other = from_canonical(&germinate(&[(0, 0, 7u8), (0, 1, 7)], &[(0, 0, 8u8), (0, 1, 8)]).unwrap());
    let words = vec![bar, bar_other];
    let mut at: Keyed<Element> = crate::network::nerve::Keyed::new();
    for w in &words {
        at.insert(w.address(), w.clone());
    }
    let word_at = |a: &[u8]| at.get(a);
    let target = vec![(0, 0, 1, 2), (0, 1, 1, 2)];
    let id = element::identity_versor();
    let cites: Vec<Cite> = vec![
        Cite { word: words[0].address(), transport: id.clone(), assignment: element::assignment(words[0].sub()) },
        Cite { word: words[1].address(), transport: id.clone(), assignment: element::assignment(words[1].sub()) },
    ];
    assert!(replay_at(&word_at, &cites[..1], &target).is_empty());
    assert_eq!(seam_defect_at(&word_at, &cites, &target), target, "both links spoken twice, differently");
    assert_eq!(verify_at(&word_at, &cites, &target), target);
    assert!(verify_at(&word_at, &cites[..1], &target).is_empty(), "one word, no seam");
    let t = element::translator_versor(3, 0);
    assert!(is_flat(&[t.clone(), t.reverse()]));
    assert!(!is_flat(&[t.clone(), element::translator_versor(0, 2)]));
    assert!(is_flat(&[]), "the empty orbit is flat");
}

/// Two word vertices and the frame star are a tree; the coboundary is exact.
#[test]
fn coheres_on_two_germ_tree() {
    let f = star_form(2, Vec::new());
    // AT1 (Q1): two germs of one word are ONE vertex with two members.
    assert_eq!(f.vertices.len(), 2, "frame plus the one word class");
    assert_eq!(
        f.vertices.at(&only_word(&f)).map(|v| v.charts().len()),
        Some(2),
        "the word stands at the two germs' charts"
    );
    assert_eq!(f.faces.len(), 2, "the frame star has an overlap per member");
    assert!(f.orbits.is_empty(), "a tree has no independent cycle");
    // `solve_coboundary` RETIRED with the tree gauge (AT1 step 2, ruled): a
    // tree's exactness is read as every overlap's derived word, the
    // coboundary of its ends, with no orbit to obstruct it.
    assert!(f.coheres(), "a tree coheres");
}

/// Separated bodies meet by relative word: the star form holds no adjacency
/// face between them.
#[test]
fn faces_hold_no_adjacency_between_separated_bodies() {
    let f = star_form(2, Vec::new());
    let word_word = f.faces.entries().filter(|(o, _)| !on_frame(o)).count();
    assert_eq!(word_word, 0, "separated bodies have no adjacency face");
}

/// Does this overlap stand on the Frame's chart at either end — a star
/// overlap (Q-B1)?
fn on_frame(o: &coord::Overlap) -> bool {
    o.from == coord::ChartAt::Frame || o.to == coord::ChartAt::Frame
}

/// A row of three has faces by constancy: the faces are bounded per chart
/// and a touching row has two adjacency faces.
#[test]
fn faces_on_row_of_three_follow_constancy() {
    let tree = star_form(3, Vec::new());
    // THE BOUND IS PER CHART (AT1): three spokes of one word are one vertex
    // with three members, and an overlap relates one member pair.
    let charts: usize = tree
        .vertices
        .entries()
        .filter(|(k, _)| *k != coord::VertexAt::Frame)
        .map(|(_, v)| v.charts().len())
        .sum();
    assert_eq!(charts, 3, "three spokes, three member charts");
    assert!(tree.faces.len() <= 2 * charts);
    let touching = star_form(3, vec![(1, 2, Word::identity()), (2, 3, Word::identity())]);
    let adj = touching.faces.entries().filter(|(o, _)| !on_frame(o)).count();
    assert_eq!(adj, 2, "two adjacency faces on a touching row, got {adj}");
    assert!(!touching.orbits.is_empty(), "the 1-skeleton is not a star tree");
}

/// A21 §2 — THE CHART LEADS WITH ITS KIND AND THEN ITS RANK. The leading
/// 2 keeps a sentence's chart disjoint from a residual's and a
/// connection's; the four after it are A18's rank, which is the COARSE
/// STRATUM (R5: the rank of a union is the sum of the parts). Nothing per
/// vertex follows, so the chart's width does not grow with the object.
#[test]
fn invariant_of_sentence_leads_with_two_then_rank() {
    let f = star_form(2, Vec::new());
    let e = as_element_form(&f, [1, 0, 0, 0]).expect("form element");
    assert!(e.is_sentence());
    let inv = stalk::invariant_of(&e);
    let rank = f.rank().expect("a star tree has a rank");
    assert_eq!(
        inv.0.get(0..5),
        Some(
            [
                2,
                rank.torsion,
                rank.free,
                rank.offplane,
                rank.axes
            ]
            .map(crate::transducer::stalk::ChartAxis::Count)
            .as_slice()
        ),
        "the kind, then the rank: {inv:?}"
    );
    // A star tree closes no orbit, so there is no class after the rank.
    assert_eq!(inv.0.len(), 5, "and nothing per vertex: {inv:?}");

    // THE SAME OBJECT WITH MORE VERTICES SITS ON THE SAME CHART. Six
    // bodies on a star instead of two: the rank is the same, the classes
    // are the same, and the chart does not widen by one coordinate.
    let wide = star_form(6, Vec::new());
    let wide_element = as_element_form(&wide, [1, 0, 0, 0]).expect("form element");
    assert_eq!(
        stalk::invariant_of(&wide_element),
        inv,
        "a form of six vertices and one of two hold the same thing up to gauge"
    );
}

/// Face constancy is the word, not the versor.
#[test]
fn face_word_carries_constancy_not_versor() {
    let a = star_form(2, vec![(1, 2, Word::identity())]);
    let b = star_form(2, vec![(1, 2, Word::identity())]);
    // THE WORD IS DERIVED on each overlap (AT1 Q2), and read there.
    let words1: Vec<_> = a.faces.entries().map(|(o, _)| a.face_word(&o)).collect();
    let words2: Vec<_> = b.faces.entries().map(|(o, _)| b.face_word(&o)).collect();
    assert!(words1.iter().all(Option::is_some), "every overlap's word derives");
    assert_eq!(words1, words2, "constancy is the word");
    let _ = as_element_form(&a, [1, 0, 0, 0]).expect("rule element");
}

/// A word-constant triangle keeps its cycle, and every orbit's word is the
/// identity.
#[test]
fn orbit_word_on_keeps_word_constant_triangle() {
    let f = star_form(3, vec![(1, 2, Word::identity()), (2, 3, Word::identity()), (1, 3, Word::identity())]);
    assert!(!f.orbits.is_empty(), "a triangle has an orbit");
    assert!(f
        .orbits
        .iter()
        .all(|lp| orbit_word_on(&f.faces, &f.vertices, lp).is_some_and(|w| w.is_identity())));
    let _ = as_element_form(&f, [1, 0, 0, 0]).expect("rule element");
}

/// Placement is not the sentence vertex: one form minted twice is one
/// vertex at one address.
#[test]
fn same_vertex_excludes_placement() {
    let f = star_form(2, Vec::new());
    assert!(!f.faces.is_empty(), "the frame star relates the two words");
    let e1 = as_element_form(&f, [1, 0, 0, 0]).expect("e1");
    let e2 = as_element_form(&f, [1, 0, 0, 0]).expect("e2");
    assert!(e1.same_vertex(&e2));
    assert_eq!(e1.address(), e2.address());
}

/// Admitting words to an axis index costs work linear in the number of words, and
/// an on-axis read costs the same whatever the index holds.
#[test]
fn admit_words_linear_and_on_axis_constant() {
    use crate::network::nerve::work;
    let inv = stalk::Invariant::default();
    let w = Transport {
        word: word_turn_180(),
        carriage: crate::transducer::element::identity_versor(),
        gauge: Vec::new(),
    };
    let run = |n: usize| {
        work::reset();
        let mut index = AxisIndex::new();
        for _ in 0..n {
            admit_words(&mut index, &inv, std::slice::from_ref(&w));
        }
        let inserts = work::snapshot().1;
        work::reset();
        let _ = on_axis_of(&index, &inv, Axis::Orientation(0));
        let gets = work::snapshot().0;
        (inserts, gets)
    };
    let (i8, g8) = run(8);
    let (i32n, g32) = run(32);
    assert!(
        i8 > 0 && i32n >= i8 && i8.checked_mul(5).is_some_and(|cap| i32n <= cap),
        "4N admits cost ~4× N inserts: {i8} then {i32n}"
    );
    assert_eq!(g8, g32, "on_axis get cost is independent of N: {g8} vs {g32}");
}

/// An orbit's chord is never a star face.
#[test]
fn orbit_chord_is_never_star_face() {
    let f = chord_form(&[word_turn_90(), word_turn_180()]);
    assert_eq!(f.orbits.len(), 2, "two chords, two orbits");
    for lp in &f.orbits {
        assert!(f.faces.holds(&lp.chord), "the chord is an overlap the form holds");
        assert!(!on_frame(&lp.chord), "chord {:?} is a star overlap", lp.chord);
    }
}

/// 18.2, 24.1 — A CHART NAMES THE WORD VERTICES ON IT. `reindex` files each
/// Word vertex under its class in the one pass it already makes, so a
/// listing body's chart reads its fibre of vertices and never reads the
/// whole form; a chart no vertex is on names nothing, and a Demand vertex on a
/// chart is not a word there.
#[test]
fn by_class_names_word_vertices_on_chart() {
    let (near, far) = (stalk::Invariant::counts(&[3, 5]), stalk::Invariant::counts(&[7]));
    let mut f = Form::default();
    // AT1 (Q1): three words of ONE class would be one vertex, so the three
    // are three classes — the identity, the 90° turn and the 180° turn.
    let mut at = Vec::new();
    for (k, word, class) in [(1u8, Word::identity(), &near), (2, word_turn_90(), &far), (3, word_turn_180(), &near)] {
        at.push(f.put_vertex(word_vertex_on(k, word, class.clone())).expect("a word vertex has a coordinate"));
    }
    f.put_vertex(Vertex::demand(Demand::Creation, near.clone(), vec![(0, 0, 0, 1)])).expect("a demand has a coordinate");
    f.reindex();
    // THE READ IS WRITTEN AS THE CHART IT IS (54.7 R5). `by_class` takes a
    // `Key<coord::Chart>` and nothing else, so the test asks at the same
    // coordinate `reindex` files by — which is the agreement that used to
    // hold only in prose, and did not hold.
    let chart_key = |i: &stalk::Invariant| {
        coord::Chart(i.clone())
    };
    // THE FIBRE IS A SET OF COORDINATES (AT1): no vertex order stands behind
    // it, so it is read as the two it holds.
    let on_near = f.by_class.at(&chart_key(&near)).expect("the near chart names its words");
    assert!(
        on_near.len() == 2 && on_near.contains(&at[0]) && on_near.contains(&at[2]),
        "both words on the near chart, got {on_near:?}"
    );
    assert_eq!(f.by_class.at(&chart_key(&far)), Some(&vec![at[1].clone()]), "the one word on the far chart");
    assert_eq!(f.by_class.at(&chart_key(&stalk::Invariant::counts(&[9]))), None, "a chart no vertex is on names nothing");
    for (chart, on) in f.by_class.scoped() {
        for i in on {
            let v = f.vertices.at(i).expect("a coordinate the fibre names is a vertex");
            // THE FIBRE IS ASKED, not a representative read off the vertex:
            // `acts` holds every chart the vertex acts on, and the claim is
            // that this chart is among them (D7's whole point — one of two
            // is the defect, and a test that reads `acting()` would report
            // the representative agreeing with itself).
            assert!(
                matches!(v, Vertex::Word { .. }) && v.acts_on_chart_key(chart.as_slice()),
                "vertex {i:?} is a Word acting on the chart that names it"
            );
        }
    }
}

/// 21 step 1 — A TRANSPORT CLASS NAMES THE WORD VERTICES OF THAT CLASS, and
/// it names them ACROSS CHARTS, which is the whole reason the chart fibre
/// above cannot serve correspondence: the two vertices here carry the same
/// word on two different charts, so `by_class` splits them into two entries
/// and `by_transport` holds them as the one class they are.
///
/// Asked with the reader's own key — `transport_class(v).key()` — because an
/// index a reader cannot address with the key it builds is not an index.
#[test]
fn by_transport_names_word_vertices_across_charts() {
    let (near, far) = (stalk::Invariant::counts(&[3, 5]), stalk::Invariant::counts(&[7]));
    let mut f = Form::default();
    // Two words of one class on two charts, and a third of a DIFFERENT class
    // (the 90° turn) on a chart one of them is already on.
    // AT1 (Q1): the two words of one class are ONE vertex whose members
    // stand at the two charts; `stand_member` joins the second to the first.
    let id_at = f.stand_member(&word_vertex_on(1, Word::identity(), near.clone()), FRAME).expect("near").vertex();
    let id_far = f.stand_member(&word_vertex_on(2, Word::identity(), far.clone()), FRAME).expect("far").vertex();
    assert_eq!(id_at, id_far, "one class, one vertex");
    let q_at = f.stand_member(&word_vertex_on(3, word_turn_90(), near.clone()), FRAME).expect("90° turn").vertex();
    f.reindex();
    let key_of = |at: &coord::VertexAt| {
        crate::transducer::family::transport_class(f.vertices.at(at).expect("a held vertex"))
            .expect("a word vertex has a transport class")
            .key()
    };
    let (identity_class, turn_90_class) = (key_of(&id_at), key_of(&q_at));
    assert_ne!(identity_class, turn_90_class, "two words, two transport classes");
    assert_eq!(
        f.by_transport.at(&identity_class),
        Some(&vec![id_at.clone()]),
        "the identity class names its one vertex"
    );
    assert_eq!(
        f.vertices.at(&id_at).map(|v| v.charts().len()),
        Some(2),
        "and that vertex stands across the two charts"
    );
    assert_eq!(
        f.by_transport.at(&turn_90_class),
        Some(&vec![q_at.clone()]),
        "the 90° turn is its own class on a chart another vertex is also on"
    );
    // The chart fibre cannot answer the same question: it splits the one
    // class over two charts and merges two classes on one chart.
    let chart_key = |i: &stalk::Invariant| {
        coord::Chart(i.clone())
    };
    let on_near = f.by_class.at(&chart_key(&near)).expect("the near chart names its words");
    assert!(
        on_near.len() == 2 && on_near.contains(&id_at) && on_near.contains(&q_at),
        "chart near: two classes, got {on_near:?}"
    );
    assert_eq!(f.by_class.at(&chart_key(&far)), Some(&vec![id_at.clone()]), "chart far: one of the identity class");
    // Every coordinate the fibre names is a Word of that class, and every
    // Word is named by its own: the relation is total in both directions.
    for (k, on) in f.by_transport.entries() {
        for i in on {
            let v = f.vertices.at(i).expect("a coordinate the fibre names is a vertex");
            assert_eq!(
                crate::transducer::family::transport_class(v).map(|c| c.key()),
                Some(k.clone()),
                "vertex {i:?} is of the class that names it"
            );
        }
    }
    for (i, v) in f.vertices.entries() {
        let Some(k) = crate::transducer::family::transport_class(v).map(|c| c.key()) else {
            continue;
        };
        assert!(
            f.by_transport.at(&k).is_some_and(|on| on.contains(&i)),
            "vertex {i:?} carries a transport class and the fibre names it"
        );
    }
}

/// A21 §4 AT THE ONE KIND THE ENGINE IS MADE OF — `from_key` for
/// `TAG_FORM`, written at last.
///
/// §1.5 writes the key "so that the word form is recovered from its key",
/// and nothing recovered it: `Rotor::from_key`, `Mv::from_key` and
/// `ClassVector::from_key` existed, `TAG_FORM` had no decoder, and that
/// gap was read as a structural division between naming and rebuilding.
/// It was an unwritten inverse, and the tape carried the key's content a
/// second time because of it — measured at 68% of every tape.
///
/// THE READING IS RE-DERIVATION, NEVER BYTES (1.5). A form rebuilt from
/// its key is the same form iff it keys to the same address. That is the
/// property; byte equality of two serialisations is not, and asserting it
/// would pin a layout as an invariant.
///
/// Two shapes, both closed and remainder-free: a word form on one chart,
/// and a form that glues — so a `Glued` vertex's cited address, which IS
/// in the key, comes back through the inverse.
#[test]
fn from_address_recovers_form_from_key() {
    use crate::transducer::sentence::{form_vertex_key, from_address};
    let q = alphabet::LetterId::turn_90((0, 1)).expect("90° turn");
    let w = Word::from_letters(&[(q, false)]);
    let inner = closed_word_form(w.clone(), None);
    let inner_addr = as_element_form(&inner, [1, 0, 0, 0]).expect("inner").address();
    let mut host = Form::default();
    host.glue(&inner, inner_addr.clone(), Word::identity(), [0, 0, 0, 1])
        .expect("glue");

    for f in [&inner, &host] {
        let key = form_vertex_key(f);
        let back = from_address(&key).expect("the key recovers its form");
        assert_eq!(
            form_vertex_key(&back),
            key,
            "the recovered form keys to the address it was recovered from"
        );
        assert_eq!(back.vertices.len(), f.vertices.len(), "every vertex");
        assert_eq!(back.faces.len(), f.faces.len(), "every face");
    }

    // The cited address is IN the key and comes back through the inverse —
    // identity is not shortened (1.7), so the inverse does not shorten it.
    let key = form_vertex_key(&host);
    let back = from_address(&key).expect("recovers");
    // 54.7 B5a — THE BOUNDARY IS RECOVERED FROM THE KEY, AND THE COVER IS
    // NOT. The key states that this chart has an open boundary at a rank, a
    // depth, a placement and a φ; which form that boundary mates with is
    // the cover's fact and reaches a reader on the word's envelope, never
    // from the key. `from_address` invents nothing for it (rule 11).
    assert!(
        back.glued_vertices().next().is_some(),
        "the glued boundary is recovered from the key alone"
    );
    assert!(
        back.cover.is_empty(),
        "and a form reconstituted from a key alone knows of no incidence, because the key carries none"
    );
    let _ = &inner_addr;

    // A key that is not one is refused rather than part-read (rule 11).
    // `from_address` takes an ADDRESS (law 2), so calling a run of bytes a
    // coordinate is a claim the caller has to write out — and refusing it
    // is what this asserts.
    use crate::transducer::address::Address;
    assert!(from_address(&Address::new(Vec::new())).is_none());
    assert!(from_address(&Address::new(vec![0xFF, 0, 0, 0, 0])).is_none());
}

/// 54.2/54.3 — THE KEY CARRIES THE CLOSED SUB-COMPLEX WHOLE, so the
/// inverse returns a demand's remainder and a face's edge, which it could
/// not before.
///
/// Three properties the completed key makes checkable, none of them about
/// bytes (1.5): a DEMAND comes back at its coordinates — kind, chart, the
/// chain's measure and pieces (AT1: the bodies ride the record); the FORM's A5
/// remainder comes back, because the remainder is identity and not
/// evidence, being what the form does not close; and a face's EDGE comes
/// back as the transport it is, where only its presence was keyed.
#[test]
fn from_address_recovers_demand_and_remainder_coordinates() {
    use crate::transducer::sentence::{form_vertex_key, from_address, Vertex};
    let mut f = Form::default();
    // A demand standing for two bodies, and a remainder the form does not
    // close.
    let bodies: Vec<(i64, i64, u8, u8)> = vec![(1, 1, 1, 2), (1, 2, 1, 2)];
    let demand = f
        .put_vertex(Vertex::demand(
            crate::transducer::sentence::Demand::Source { multiplicity: 2 },
            crate::transducer::stalk::Invariant::default(),
            bodies.clone(),
        ))
        .expect("a demand has a coordinate");
    let rem: Vec<(i64, i64, u8, u8)> = vec![(7, 7, 3, 4)];
    f.remainder = crate::transducer::element::place_chain(&rem).expect("a chain places");

    let key = form_vertex_key(&f);
    let back = from_address(&key).expect("recovers");

    // AT1, 54.5 ROAD (d) — THE KEY CARRIES THE DEMAND BY ITS COORDINATES:
    // kind, chart, the chain's measure per lane and its piece count. The
    // bodies are evidence and ride the record, never the key, so a form
    // read from its key alone holds the demand at the same coordinate and
    // no bodies — which `from_element_form` refills from the envelope.
    let got: Vec<coord::VertexAt> = back.demand_vertices().map(|(at, ..)| at).collect();
    assert_eq!(got, vec![demand.clone()], "the demand comes back at its own coordinate");
    let Some(Vertex::Demand { measure, pieces, bodies: back_bodies, .. }) = back.vertices.at(&demand) else {
        panic!("the demand vertex")
    };
    let Some(Vertex::Demand { measure: was_measure, pieces: was_pieces, .. }) = f.vertices.at(&demand) else {
        panic!("the demand vertex")
    };
    assert_eq!((measure, pieces), (was_measure, was_pieces), "the chain's measure and pieces come back");
    assert!(back_bodies.is_empty(), "and the bodies do not: the key carries none");
    // THE REMAINDER COMES BACK AS ITS COORDINATES (AT1, 54.5 Road (d)): its
    // measure, pieces and corner; the chain rides the record as a demand's
    // bodies do.
    assert!(back.remainder.placed().is_empty(), "the chain is not in the key");
    assert_eq!(
        back.remainder_at,
        ChainAt::of(&f.remainder.placed()),
        "the remainder comes back by its coordinates: it is identity, not evidence"
    );
    assert_eq!(
        form_vertex_key(&back),
        key,
        "and the recovered form keys to the address it came from"
    );
}
