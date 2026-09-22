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

/// 50.3 — THE EDGE HOLONOMY DOES NOT TELESCOPE. The face transports of a
/// loop are the coboundary of the placements and compose to the identity
/// by construction; the edge transports are stabilizer elements, and two
/// quarter turns around a triangle compose to the half turn, a class the
/// loop holds until it is filed. A loop through a remainder edge is not
/// evaluated.
#[test]
fn loop_holonomy_in_the_stabilizer_does_not_telescope() {
    let qw = Word::from_letters(&[(alphabet::LetterId::quarter((0, 1)).unwrap(), false)]);
    let mut f = Form::default();
    for k in 1..=3u8 {
        f.vertices.push(Vertex::word_on(
            alphabet::Derivation::spelled(Word::identity()),
            stalk::Invariant::default(),
            Box::new(element::carriage(
                &q1(),
                &(q0(), num_rational::BigRational::from_integer(num_bigint::BigInt::from(k as i64))),
            )),
            Vec::new(),
            None,
        ));
    }
    // The faces among the bodies only: the loop 1-2-3 closes on the chord
    // 1-3 without passing the Frame.
    for (i, j) in [(1u16, 2u16), (2, 3), (1, 3)] {
        f.faces.push(f.face_of(i, j).expect("vertices"));
    }
    f.loops = Form::loops_of(&f.faces, &f.vertices);
    // THE LOOP STANDS BEFORE ITS EDGES ARE WRITTEN. Its faces are real —
    // between bodies, not star — and carry no edge yet, so every member
    // path drops on a remainder edge and the loop is UNREAD rather than
    // flat; a trivial-cycle filter that read an empty edge answer as a flat
    // one would prune it (rule 11).
    assert_eq!(f.loops.len(), 1, "one loop among three bodies");
    assert!(f.coheres(), "the coboundary of the placements telescopes: every loop is the identity");
    let at = |f: &Form, i: u16, j: u16| f.faces.iter().position(|fc| (fc.i, fc.j) == (i, j)).expect("face");
    let (e12, e23, e13) = (at(&f, 1, 2), at(&f, 2, 3), at(&f, 1, 3));
    // THE EDGE IS SET AT EVERY MEMBER PAIR of the face. Each of these vertices
    // stands at one chart, so each face has one pair and the loop has one
    // member path — which is what lets this read a single holonomy below.
    let set_edge = |f: &mut Form, fi: usize, edge: Option<Transport>| {
        let pairs: Vec<Vec<u8>> = f.faces[fi].at.scoped().map(|(k, _)| k).collect();
        assert_eq!(pairs.len(), 1, "one chart per vertex here, so one member pair per face");
        for k in pairs {
            if let Some(p) = f.faces[fi].at.get_mut(&k) {
                p.edge = edge.clone();
            }
        }
    };
    set_edge(&mut f, e12, Some(Transport::from_word(qw.clone())));
    set_edge(&mut f, e23, Some(Transport::from_word(qw.clone())));
    set_edge(&mut f, e13, Some(Transport::from_word(Word::identity())));
    // A face write invalidates the loops — `apply_move` says so in its own
    // line — and the cycle stands now that its edges compose to a half turn.
    f.loops = Form::loops_of(&f.faces, &f.vertices);
    let [triangle, ..] = f.loops.as_slice() else { panic!("the loop 1-2-3 among the bodies") };
    // ONE MEMBER PATH, so one holonomy — and the slice pattern says so rather
    // than taking the first of several.
    let one_holonomy = |f: &Form, lp: &Loop| -> Option<Transport> {
        let hs = f.edge_holonomies(lp)?;
        let [(_, h)] = hs.scoped().collect::<Vec<_>>()[..] else {
            return None;
        };
        Some(h.clone())
    };
    let h = one_holonomy(&f, triangle).expect("every edge of the loop is carried");
    let half = qw.compose(&qw);
    assert!(h.word.rotor().projective_eq(&half.rotor()), "quarter, quarter, identity compose to the half turn: {:?}", h.word);
    assert_eq!(
        f.edge_classes(triangle).scoped().map(|(_, c)| c.clone()).collect::<Vec<_>>(),
        vec![half.rotor().holonomy_key().expect("the half turn has a class")],
        "read up to conjugacy: the half turn's class, on the one path"
    );
    // A COMPACT HOLONOMY IS CONTENT AND COHERES (54.5, as ruled
    // 2026-09-20). This read `!f.coheres()` and then filed a period to
    // make it true, which asserted that an ANNOTATION'S PRESENCE changes a
    // geometric property. Every field of a `FiledLoop` is derivable —
    // `loop_classes` computes the class, the period through
    // `cech::classify`, σ from the transports — so law 12 forbids storing
    // one beside the coordinates that determine it, and a predicate
    // reading it was asking "has a stroke got around to this loop".
    //
    // The half turn returns after two circuits about a fixed point. That
    // is `ℤ/2ℤ` torsion, and torsion is content (law 7): the loop coheres
    // BECAUSE it is compact, not because anyone wrote it down.
    assert!(f.coheres(), "a half turn returns: ℤ/2ℤ is a compact resonance, and it coheres");
    // AND THE FILING CHANGES NOTHING HERE. It is the speaker's
    // attestation — what its stroke discharged against its own budget —
    // and it prices the yield (E2) and answers whether the stroke
    // finished. Neither is a fact about the connection.
    // THE SPECTRUM THE ROTOR ACTUALLY HAS (54.7), not a scalar chosen for
    // it: a filing that names a period the geometry does not carry is the
    // thing `REFUSE_INFLATED_YIELD` exists to catch, so a fixture must file
    // what the rotor says.
    f.torsion.put(
        &half.rotor().holonomy_key().expect("class"),
        FiledLoop { sigma: Vec::new(), periods: half.rotor().toral_period(), chord: triangle.chord },
    );
    assert!(f.coheres(), "and filing it says nothing the geometry did not already say");
    // NO RE-WALK HERE. `edge_holonomies` reads `self.faces` live, so the
    // `Loop` — its vertices, its faces, its chord — stays valid across an
    // edge write; re-walking would drop this cycle, because `q, q⁻¹, 1`
    // composes to the identity and a cycle that encloses nothing is not a
    // loop. The reading being asserted is exactly that identity.
    set_edge(&mut f, e23, Some(Transport::from_word(qw.inverse())));
    let [triangle, ..] = f.loops.as_slice() else { panic!("the loop") };
    let h = one_holonomy(&f, triangle).expect("carried");
    assert!(h.word.rotor().projective_eq(&alphabet::Rotor::identity()), "quarter, quarter inverse, identity is the identity");
    set_edge(&mut f, e13, None);
    let [triangle, ..] = f.loops.as_slice() else { panic!("the loop") };
    assert!(
        f.edge_holonomies(triangle).is_some_and(|hs| hs.is_empty()),
        "a loop through a remainder edge is not evaluated on any path"
    );
    assert!(f.coheres(), "and holds nothing against the form");
}

/// 52.3 AS AMENDED (54.5, ruled 2026-09-20) — A LOOP OF ANY ORDER FILES,
/// AND FILING IS NOT COHERING.
///
/// A (3,4,5) rotor on a chord's edge is a holonomy with NO PERIOD: `g^ℓ`
/// is never the identity, so the circuit walks and never returns. It is a
/// free part — a non-compact obstruction — and what the form owes for it
/// is a `Demand::Dislocation` carrying the slip, not an entry in a
/// side-table.
///
/// This test asserted the opposite for four commits: that filing the class
/// at period 0 made the loop COHERE. That is procedural compliance wearing
/// geometry's clothes, and it let an open dislocation read as a closed
/// manifold. Every field of a `FiledLoop` is derivable from the complex
/// (`loop_classes`), so by law 12 the annotation carries no fact the
/// geometry does not already determine — it records that a stroke looked.
///
/// What remains true, and is what the filing is FOR: the stroke reads the
/// class, files it at 0, and the reading rides `Element.filing` as the
/// speaker's attestation, where it prices the yield and says whether the
/// stroke finished.
#[test]
fn a_loop_of_infinite_order_files_at_period_zero_and_does_not_cohere() {
    let w = infinite();
    assert_eq!(w.order(), alphabet::Order::Infinite, "the (3,4,5) rotor has no period");
    let mut f = edge_chord_form(std::slice::from_ref(&w));
    let loops = f.loops.clone();
    let [lp] = loops.as_slice() else { panic!("one chord closes one loop") };
    assert!(!f.edge_coheres(lp), "an edge of infinite order holds against the form");
    assert!(!f.coheres());
    let classes: Vec<_> = f.edge_classes(lp).scoped().map(|(_, c)| c.clone()).collect();
    let [class] = classes.as_slice() else { panic!("one member path, so one class") };
    // A FILING DOES NOT CLOSE A CIRCUIT THAT WALKS (54.5, as ruled
    // 2026-09-20). This filed period 0 — A19's spelling of infinite order
    // — and then asserted the loop COHERED, which made an open dislocation
    // read as a closed manifold. A (3,4,5) rotor has no period: `g^ℓ` is
    // never the identity, the circuit never returns, and no annotation
    // changes that. Writing "0" in a notebook records that the stroke
    // looked; it does not make the geometry close.
    f.torsion.put(
        class,
        FiledLoop { sigma: Vec::new(), periods: ToralPeriod::OPEN, chord: lp.chord },
    );
    assert!(!f.edge_coheres(lp), "filed or unfiled, a free part is not a compact resonance");
    assert!(!f.coheres());
    // A SPECTRUM THE LOOP DOES NOT HAVE. `[4, 4]` is an isoclinic quarter
    // turn and this loop is a free part; naming it is not a filing.
    f.torsion.put(
        class,
        FiledLoop { sigma: Vec::new(), periods: ToralPeriod::from_pair(4, 4), chord: lp.chord },
    );
    assert!(!f.coheres(), "and a period the loop does not have is not a filing either");

    let mut g = chord_form(std::slice::from_ref(&w));
    let loops = g.loops.clone();
    let [gl] = loops.as_slice() else { panic!("one chord closes one loop") };
    assert!(!gl.holonomy.is_identity(), "the transport composite carries the rotor");
    assert!(!g.is_cauterized(), "unfiled on the word, the form is not cauterized");
    g.torsion.put(
        &gl.holonomy.rotor().holonomy_key().expect("a class"),
        FiledLoop { sigma: Vec::new(), periods: ToralPeriod::OPEN, chord: gl.chord },
    );
    assert!(g.is_cauterized(), "filed as open under the holonomy's class, it is");
}

#[test]
fn face_of_a_missing_vertex_is_none() {
    let f = Form::default();
    assert!(
        f.face_of(0, 5).is_none(),
        "a missing vertex is not the identity word"
    );
    assert!(f.face_of(0, 0).is_some(), "the frame is present");
}

#[test]
fn glue_retract_remaps_faces_and_placement_refuses() {
    let w = Word::from_letters(&[(alphabet::LetterId::quarter((0, 1)).unwrap(), false)]);
    let mut host = Form::default();
    host.vertices.push(Vertex::word_on(
        alphabet::Derivation::spelled(w.clone()),
        stalk::Invariant::default(),
        Box::new(element::carriage(&q1(), &(q0(), q0()))),
        Vec::new(),
        None,
    ));
    host.vertices.push(Vertex::word_on(
        alphabet::Derivation::spelled(w.clone()),
        stalk::Invariant::default(),
        Box::new(element::carriage(&q1(), &(q0(), q1()))),
        Vec::new(),
        None,
    ));
    host.faces.push(host.face_of(0, 1).expect("0-1"));
    host.faces.push(host.face_of(1, 2).expect("1-2"));
    host.faces.push(host.face_of(0, 2).expect("0-2"));
    let inner = closed_word_form(w.clone(), None);
    let inner_e = as_element_form(&inner, [1, 0, 0, 0]).expect("inner");
    let gi = host
        .glue(&inner, inner_e.address(), w.clone(), [1, 0, 0, 0])
        .expect("glue");
    // 54.7 B5a — `glued` IS KEYED BY THE SITE, and the site is what the
    // vertex holds; the cited address is the cover's.
    let site = coord::Site::decode(&crate::transducer::sentence::vertex_bytes(
        &host.vertices[gi as usize],
    ))
    .expect("a citation's own boundary record");
    assert_eq!(host.glued.at(&site).copied(), Some(gi));
    assert!(
        host.cover.iter().any(|e| e.target == inner_e.address()),
        "and the cover names the form the boundary mates with"
    );
    // The placement face is the one glue just pushed, so the tail is it.
    #[expect(
        clippy::disallowed_methods,
        reason = "rule 15 — slice pattern on the value just pushed: `glue` appends the placement face, so the tail is that face"
    )]
    let wrote = host.faces.last().map(|f| (f.i, f.j));
    assert_eq!(wrote, Some((0, gi)), "glue writes the Frame→Glued placement face");
    let mut drop: Keyed<(), coord::VertexIx> = Keyed::new();
    drop.put(&vertex_ix_key(1), ());
    host.retract_vertices(&drop);
    assert_eq!(host.vertices.len(), 3, "Frame, remaining Word, Glued");
    for face in &host.faces {
        assert_eq!(
            host.face_of(face.i, face.j).as_ref(),
            Some(face),
            "every surviving face re-derives, word and every member pair"
        );
    }
    let mut taped = host.clone();
    let mag = [1u64, 0, 0, 0];
    let clean = as_element_form(&taped, mag).expect("tape");
    assert_eq!(crate::court::verdict::blind_replay(&clean), Some(crate::court::verdict::REFUSE_GLUED_UNRULED));
    let resolve = |a: &[u8]| (a == inner_e.address().as_bytes()).then(|| inner.clone());
    assert_eq!(crate::court::verdict::blind_replay_with(&clean, &resolve), None);
    let Some(fi) = taped.faces.iter().position(|f| f.i == 0 && matches!(taped.vertices.get(f.j as usize), Some(Vertex::Glued { .. }))) else {
        panic!("placement face");
    };
    taped.faces[fi].word = Word::identity();
    let bad = as_element_form(&taped, mag).expect("tampered placement");
    assert_eq!(
        crate::court::verdict::blind_replay_with(&bad, &resolve),
        Some(crate::court::verdict::REFUSE_FACE),
        "a tampered placement face refuses"
    );
}

/// The bundle's agreed shift is meaning: two translations of one linear
/// block are two vertices.
#[test]
fn two_shifts_are_two_addresses() {
    let letter = alphabet::LetterId::quarter((0, 1)).expect("quarter");
    let w = Word::from_letters(&[(letter, false)]);
    let mut a = closed_word_form(w.clone(), None);
    let mut b = closed_word_form(w, None);
    a.vertices[1].set_carriage(element::carriage(&q1(), &(q0(), q1())));
    b.vertices[1].set_carriage(element::carriage(
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
fn ratio_and_sections_are_on_the_form_key() {
    let letter = alphabet::LetterId::quarter((0, 1)).expect("quarter");
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
        class: w.rotor().holonomy_key().expect("the quarter turn has a class"),
    });
    c.sections.put(&class, section_of_frames([1]));
    let ea = as_element_form(&a, [1, 0, 0, 0]).expect("a");
    let eb = as_element_form(&b, [1, 0, 0, 0]).expect("b");
    let ec = as_element_form(&c, [1, 0, 0, 0]).expect("c");
    assert_ne!(ea.address(), eb.address(), "the frame ratio is on the key");
    assert_ne!(ea.address(), ec.address(), "a section is on the key");
    let back_b = crate::transducer::sentence::from_element_form(&eb).expect("v2 tape");
    assert_eq!(back_b.ratio, ((2, 1), (2, 1)));
    let back_c = crate::transducer::sentence::from_element_form(&ec).expect("v2 tape");
    assert_eq!(
        back_c.sections.at(&class).map(frames_of_section),
        Some(vec![1u16]),
        "the section round-trips as the frame it stands on"
    );
    // A ZERO HEIGHT DENOMINATOR IS NOT A BUNDLE. `ratio` is `((wn, wd),
    // (hn, hd))` and `hd = 0` is no ratio at all, so the key that carries
    // it does not decode back to a form — refused, never repaired to 1.
    let mut broken = closed_word_form(
        Word::from_letters(&[(alphabet::LetterId::quarter((0, 1)).expect("quarter"), false)]),
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
fn depth_is_one_plus_max_glued() {
    let q = alphabet::LetterId::quarter((0, 1)).expect("quarter");
    let w = Word::from_letters(&[(q, false)]);
    let inner = closed_word_form(w.clone(), None);
    assert_eq!(inner.depth_of(), 0);
    let mut outer = Form::default();
    let a0 = as_element_form(&inner, [1, 0, 0, 0]).expect("inner").address();
    outer.glue(&inner, a0.clone(), Word::identity(), [0, 0, 0, 1]).expect("glue into empty host");
    assert_eq!(outer.depth_of(), 1);
    assert_eq!(outer.depth, 1);
    let mut wrap = Form::default();
    wrap.glue(&outer, form_addr(2), Word::identity(), [0, 0, 0, 1]).expect("glue depth 1 into depth 0");
    assert_eq!(wrap.depth_of(), 2);
    // A CITATION AT EQUAL DEPTH, AND ONE DEEPER, both stand: the nerve has
    // no stratification and the depth that comes back is the deepest of
    // them, read off the citations themselves.
    let outer_copy = outer.clone();
    outer
        .glue(&outer_copy, form_addr(3), Word::identity(), [0, 0, 0, 1])
        .expect("a citation at equal depth is a citation");
    assert_eq!(outer.depth_of(), 2, "the equal-depth citation is one deep, so the host is two");
    outer
        .glue(&wrap, form_addr(4), Word::identity(), [0, 0, 0, 1])
        .expect("a citation of a deeper form is a citation");
    assert_eq!(outer.depth_of(), 3, "and the deepest citation is what depth reads");
    let mut again = Form::default();
    again.glue(&inner, a0.clone(), Word::identity(), [0, 0, 0, 1]).unwrap();
    again.glue(&inner, form_addr(5), Word::identity(), [0, 0, 0, 1]).expect("another depth-0 glue");
    assert_eq!(again.depth_of(), 1, "max glued depth, not a count");
}

/// A language of a bar and a dot deconstructs a rotated, translated,
/// and dilated bar through its moduli with a verified transport, speaks
/// a chain of two bodies as two words with their relative transport,
/// leaves the unknown body as the residual, and refuses to cite a word
/// with itself.
#[test]
fn a_chain_deconstructs_into_its_words_and_residual() {
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
    let s = deconstruct_at(&word_at, &atlas, &shape_at,&target, None);
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
    let s = deconstruct_at(&word_at, &atlas, &shape_at,&big, None);
    assert_eq!(s.cites.len(), 1);
    assert_eq!(s.cites[0].word, words[2].address());
    assert!(s.is_closed());
    // Two bodies and an unknown: two citations with one relative
    // transport between them, one residual body.
    let mixed = vec![(0, 0, 1, 2), (0, 1, 1, 2), (4, 4, 3, 4), (9, 9, 5, 6), (9, 10, 5, 6), (10, 9, 5, 6)];
    let s = deconstruct_at(&word_at, &atlas, &shape_at,&mixed, None);
    assert_eq!(s.cites.len(), 2);
    assert_eq!(
        s.multiplicity.len(),
        4,
        "the chain and each of bar, dot, residual have a width, never one maximum"
    );
    assert_eq!(s.connection.len(), 1, "one relative transport between two cited bodies");
    assert_eq!(s.residual, vec![(9, 9, 5, 6), (9, 10, 5, 6), (10, 9, 5, 6)]);
    // Excluding the bar itself: a bar chain has no other word of hue 1→2.
    let s = deconstruct_at(&word_at, &atlas, &shape_at,words[0].sub(), Some(words[0].address().as_bytes()));
    assert!(s.cites.is_empty() && !s.is_closed(), "no word but itself speaks it");
}

#[test]
fn deconstruct_multiplicity_is_per_body() {
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
    let s = deconstruct_at(&word_at, &atlas, &shape_at, &mixed, None);
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
/// a transport and its inverse are a flat loop, two translations are
/// not.
#[test]
fn seams_and_holonomy_are_located_exactly() {
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
        Cite { word: words[0].address(), transport: id.clone(), gauge: 0 },
        Cite { word: words[1].address(), transport: id.clone(), gauge: 0 },
    ];
    assert!(replay_at(&word_at, &cites[..1], &target).is_empty());
    assert_eq!(seam_defect_at(&word_at, &cites, &target), target, "both links spoken twice, differently");
    assert_eq!(verify_at(&word_at, &cites, &target), target);
    assert!(verify_at(&word_at, &cites[..1], &target).is_empty(), "one word, no seam");
    let t = element::translator_versor(3, 0);
    assert!(is_flat(&[t.clone(), t.versor_inverse().unwrap()]));
    assert!(!is_flat(&[t.clone(), element::translator_versor(0, 2)]));
    assert!(is_flat(&[]), "the empty loop is flat");
}

/// Two word vertices and the frame star are a tree; the coboundary is exact.
#[test]
fn a_two_germ_nerve_is_a_tree_and_closes() {
    let f = star_form(2, Vec::new());
    assert_eq!(f.vertices.len(), 3, "frame plus two words");
    assert!(!f.faces.is_empty(), "the frame star has faces");
    assert!(f.loops.is_empty(), "a tree has no independent cycle");
    assert!(f.solve_coboundary().is_some(), "a tree 1-cochain is exact");
    assert!(f.coheres(), "a tree coheres");
}

#[test]
fn separated_bodies_meet_by_relative_word() {
    let f = star_form(2, Vec::new());
    let word_word = f.faces.iter().filter(|fa| fa.i != 0 && fa.j != 0).count();
    assert_eq!(word_word, 0, "separated bodies have no adjacency face");
}

#[test]
fn a_row_of_three_has_faces_by_constancy() {
    let tree = star_form(3, Vec::new());
    assert!(tree.faces.len() <= 2 * (tree.vertices.len() - 1));
    let touching = star_form(3, vec![(1, 2, Word::identity()), (2, 3, Word::identity())]);
    let adj = touching.faces.iter().filter(|fa| fa.i != 0 && fa.j != 0).count();
    assert_eq!(adj, 2, "two adjacency faces on a touching row, got {adj}");
    assert!(!touching.loops.is_empty(), "the 1-skeleton is not a star tree");
}

/// A21 §2 — THE CHART LEADS WITH ITS KIND AND THEN ITS RANK. The leading
/// 2 keeps a sentence's chart disjoint from a residual's and a
/// connection's; the four after it are A18's rank, which is the COARSE
/// STRATUM (R5: the rank of a union is the sum of the parts). Nothing per
/// vertex follows, so the chart's width does not grow with the object.
#[test]
fn a_sentence_chart_leads_with_axis_two_then_its_rank() {
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
                u64::from(rank.torsion),
                u64::from(rank.free),
                u64::from(rank.offplane),
                u64::from(rank.axes)
            ]
            .map(crate::transducer::stalk::ChartAxis::Count)
            .as_slice()
        ),
        "the kind, then the rank: {inv:?}"
    );
    // A star tree closes no loop, so there is no class after the rank.
    assert_eq!(inv.0.len(), 5, "and nothing per vertex: {inv:?}");

    // THE SAME OBJECT WITH MORE VERTICES SITS ON THE SAME CHART. Six
    // bodies on a star instead of two: the rank is the same, the classes
    // are the same, and the chart does not widen by one coordinate.
    let wide = star_form(6, Vec::new());
    let we = as_element_form(&wide, [1, 0, 0, 0]).expect("form element");
    assert_eq!(
        stalk::invariant_of(&we),
        inv,
        "a form of six vertices and one of two hold the same thing up to gauge"
    );
}

#[test]
fn face_constancy_is_the_word_not_the_versor() {
    let a = star_form(2, vec![(1, 2, Word::identity())]);
    let b = star_form(2, vec![(1, 2, Word::identity())]);
    let words1: Vec<_> = a.faces.iter().map(|fa| fa.word.clone()).collect();
    let words2: Vec<_> = b.faces.iter().map(|fa| fa.word.clone()).collect();
    assert_eq!(words1, words2, "constancy is the word");
    let _ = as_element_form(&a, [1, 0, 0, 0]).expect("rule element");
}

#[test]
fn word_constant_triangle_keeps_its_cycle() {
    let f = star_form(3, vec![(1, 2, Word::identity()), (2, 3, Word::identity()), (1, 3, Word::identity())]);
    assert!(!f.loops.is_empty(), "a triangle has a loop");
    assert!(f.loops.iter().all(|lp| lp.holonomy.is_identity()));
    let _ = as_element_form(&f, [1, 0, 0, 0]).expect("rule element");
}

#[test]
fn placement_is_not_the_sentence_vertex() {
    let f = star_form(2, Vec::new());
    assert!(!f.faces.is_empty(), "the frame star relates the two words");
    let e1 = as_element_form(&f, [1, 0, 0, 0]).expect("e1");
    let e2 = as_element_form(&f, [1, 0, 0, 0]).expect("e2");
    assert!(e1.same_vertex(&e2));
    assert_eq!(e1.address(), e2.address());
}

#[test]
fn axis_index_admit_is_linear_on_axis_is_constant() {
    use crate::network::nerve::work;
    let inv = stalk::Invariant::default();
    let w = Transport {
        word: order2(),
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

#[test]
fn chord_is_never_a_star_face() {
    let f = chord_form(&[order4(), order2()]);
    assert_eq!(f.loops.len(), 2, "two chords, two loops");
    for lp in &f.loops {
        let ch = &f.faces[lp.chord as usize];
        assert!(
            ch.i != 0 && ch.j != 0,
            "chord {} is a star face ({}, {})",
            lp.chord,
            ch.i,
            ch.j
        );
    }
}

/// 18.2, 24.1 — A CHART NAMES THE WORD VERTICES ON IT. `reindex` files each
/// Word vertex under its class in the one pass it already makes, so a
/// listing body's chart reads its fiber of vertices and never walks the
/// form; a chart no vertex is on names nothing, and a Demand vertex on a
/// chart is not a word there.
#[test]
fn a_chart_names_the_word_vertices_on_it() {
    let (near, far) = (stalk::Invariant::counts(&[3, 5]), stalk::Invariant::counts(&[7]));
    let mut f = Form::default();
    for (k, class) in [(1u8, &near), (2, &far), (3, &near)] {
        f.vertices.push(word_vertex_on(k, Word::identity(), class.clone()));
    }
    f.vertices.push(Vertex::Demand {
        kind: Demand::Creation,
        chart: near.clone(),
        bodies: vec![(0, 0, 0, 1)],
    });
    f.reindex();
    // THE READ IS WRITTEN AS THE CHART IT IS (54.7 R5). `by_class` takes a
    // `Key<coord::Chart>` and nothing else, so the test asks at the same
    // coordinate `reindex` files by — which is the agreement that used to
    // hold only in prose, and did not hold.
    let chart_key = |i: &stalk::Invariant| {
        coord::Chart(i.clone())
    };
    assert_eq!(f.by_class.at(&chart_key(&near)), Some(&vec![1u16, 3]), "both words on the near chart, in vertex order");
    assert_eq!(f.by_class.at(&chart_key(&far)), Some(&vec![2u16]), "the one word on the far chart");
    assert_eq!(f.by_class.at(&chart_key(&stalk::Invariant::counts(&[9]))), None, "a chart no vertex is on names nothing");
    for (chart, on) in f.by_class.scoped() {
        for &i in on {
            let v = f.vertices.get(i as usize).expect("an index the fiber names is a vertex");
            // THE FIBRE IS ASKED, not a representative read off the vertex:
            // `acts` holds every chart the vertex acts on, and the claim is
            // that this chart is among them (D7's whole point — one of two
            // is the defect, and a test that reads `acting()` would report
            // the representative agreeing with itself).
            assert!(
                matches!(v, Vertex::Word { .. }) && v.acts_on_chart_key(chart.as_slice()),
                "vertex {i} is a Word acting on the chart that names it"
            );
        }
    }
}

/// 21 step 1 — A TRANSPORT CLASS NAMES THE WORD VERTICES OF THAT CLASS, and
/// it names them ACROSS CHARTS, which is the whole reason the chart fiber
/// above cannot serve correspondence: the two vertices here carry the same
/// word on two different charts, so `by_class` splits them into two entries
/// and `by_transport` holds them as the one class they are.
///
/// Asked with the reader's own key — `transport_class(v).key()` — because an
/// index a reader cannot address with the key it builds is not an index.
#[test]
fn a_transport_class_names_the_word_vertices_of_that_class() {
    let (near, far) = (stalk::Invariant::counts(&[3, 5]), stalk::Invariant::counts(&[7]));
    let mut f = Form::default();
    // Two words of one class on two charts, and a third of a DIFFERENT class
    // (the quarter turn) on a chart one of them is already on.
    f.vertices.push(word_vertex_on(1, Word::identity(), near.clone()));
    f.vertices.push(word_vertex_on(2, Word::identity(), far.clone()));
    f.vertices.push(word_vertex_on(3, order4(), near.clone()));
    f.reindex();
    let key_of = |i: usize| {
        crate::transducer::family::transport_class(&f.vertices[i])
            .expect("a word vertex has a transport class")
            .key()
    };
    let (identity_class, quarter_class) = (key_of(1), key_of(3));
    assert_ne!(identity_class, quarter_class, "two words, two transport classes");
    assert_eq!(
        f.by_transport.at(&identity_class),
        Some(&vec![1u16, 2]),
        "the identity class names both its vertices, across the two charts"
    );
    assert_eq!(
        f.by_transport.at(&quarter_class),
        Some(&vec![3u16]),
        "the quarter turn is its own class on a chart another vertex is also on"
    );
    // The chart fiber cannot answer the same question: it splits the one
    // class over two charts and merges two classes on one chart.
    let chart_key = |i: &stalk::Invariant| {
        coord::Chart(i.clone())
    };
    assert_eq!(f.by_class.at(&chart_key(&near)), Some(&vec![1u16, 3]), "chart near: two classes");
    assert_eq!(f.by_class.at(&chart_key(&far)), Some(&vec![2u16]), "chart far: one of the identity class");
    // Every index the fiber names is a Word of that class, and every Word is
    // named by its own: the relation is total in both directions.
    for (k, on) in f.by_transport.entries() {
        for &i in on {
            let v = f.vertices.get(i as usize).expect("an index the fiber names is a vertex");
            assert_eq!(
                crate::transducer::family::transport_class(v).map(|c| c.key()),
                Some(k.clone()),
                "vertex {i} is of the class that names it"
            );
        }
    }
    for (i, v) in f.vertices.iter().enumerate() {
        let Some(k) = crate::transducer::family::transport_class(v).map(|c| c.key()) else {
            continue;
        };
        assert!(
            f.by_transport.at(&k).is_some_and(|on| on.contains(&(i as u16))),
            "vertex {i} carries a transport class and the fiber names it"
        );
    }
}

/// A6: the address→vertex meet a stroke reads is DERIVED at the stroke, not
/// carried in from the caller. A form assembled vertex by vertex has an
/// empty meet until something fills it; a pivot on such a form used to go
/// ahead with the empty one in release and only assert in debug, so A6 and
/// Mayer–Vietoris would look up a word and find no vertex holding it.
#[test]
fn a_pivot_derives_the_address_meet_it_reads() {
    let mut f = chord_form(&[order4()]);
    f.by_address = Keyed::new();
    let _ = f.pivot_loop(
        [0x2a; 32],
        &Keyed::new(),
        &crate::transducer::family::NoCrystal,
        &mut crate::physics::Physics::default().stroke_n(16),
        None,
        0,
    );
    let mut words = 0usize;
    for (i, v) in f.vertices.iter().enumerate() {
        if let Vertex::Word { address, .. } = v {
            words += 1;
            assert_eq!(
                f.by_address
                    .at(&coord::Member::decode(address.as_bytes())
                        .expect("a word vertex address")),
                Some(&(i as u16)),
                "vertex {i} at {:?} is not what the meet resolves",
                address.as_bytes()
            );
        }
    }
    assert_eq!(words, 2, "the chord form carries its two word vertices");
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
fn a_form_is_recovered_from_its_key() {
    use crate::transducer::sentence::{form_vertex_key, from_address};
    let q = alphabet::LetterId::quarter((0, 1)).expect("quarter");
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
        back.vertices
            .iter()
            .any(|v| matches!(v, crate::transducer::sentence::Vertex::Glued { .. })),
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
/// bytes (1.5): a DEMAND's bodies come back — they were "on the tape and
/// not in the key" and are now that chain's own address; the FORM's A5
/// remainder comes back, because the remainder is identity and not
/// evidence, being what the form does not close; and a face's EDGE comes
/// back as the transport it is, where only its presence was keyed.
#[test]
fn the_key_carries_the_demand_the_remainder_and_the_edge() {
    use crate::transducer::sentence::{form_vertex_key, from_address, Vertex};
    let mut f = Form::default();
    // A demand standing for two bodies, and a remainder the form does not
    // close.
    let bodies: Vec<(i64, i64, u8, u8)> = vec![(1, 1, 1, 2), (1, 2, 1, 2)];
    f.vertices.push(Vertex::Demand {
        kind: crate::transducer::sentence::Demand::Source { multiplicity: 2 },
        chart: crate::transducer::stalk::Invariant::default(),
        bodies: bodies.clone(),
    });
    let rem: Vec<(i64, i64, u8, u8)> = vec![(7, 7, 3, 4)];
    f.remainder = crate::transducer::element::place_chain(&rem).expect("a chain places");

    let key = form_vertex_key(&f);
    let back = from_address(&key).expect("recovers");

    let got: Vec<Vec<(i64, i64, u8, u8)>> = back
        .vertices
        .iter()
        .filter_map(|v| match v {
            Vertex::Demand { bodies, .. } => Some(bodies.clone()),
            _ => None,
        })
        .collect();
    // THE BODIES COME BACK CENTRED, and that is identity, not loss.
    // 54.2 keys the remainder as `from_residual`'s address, and A5's chain
    // is centred — "the body's absolute position is already quotiented on
    // the chain". Two demands differing only in where they sit are one
    // demand, so the reading is equality of the chains' ADDRESSES (1.5),
    // never of their placed sites.
    let addr_of = |c: &Vec<(i64, i64, u8, u8)>| {
        crate::transducer::element::from_residual(c).map(|e| e.address())
    };
    let [back_bodies] = got.as_slice() else { panic!("one demand comes back, got {}", got.len()) };
    assert_eq!(
        addr_of(back_bodies),
        addr_of(&bodies),
        "the demand's chain comes back as the chain it was"
    );
    assert_eq!(back_bodies, &bodies, "and at the corner it stood at");
    assert_eq!(
        back.remainder.placed(),
        f.remainder.placed(),
        "the remainder comes back: it is identity, not evidence"
    );
    assert_eq!(
        form_vertex_key(&back),
        key,
        "and the recovered form keys to the address it came from"
    );
}
