// 18.2 — THE FORM: vertices, faces by constancy, glue and retract, depth,
// and what a chain deconstructs into.

use super::common::*;
use crate::network::nerve::Keyed;
use crate::transducer::alphabet::{self, Word};
use crate::transducer::blade::Shape;
use crate::transducer::element::{self, from_canonical};
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
            form_addr(k),
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
        vec![half.rotor().class_key().expect("the half turn has a class")],
        "read up to conjugacy: the half turn's class, on the one path"
    );
    assert!(!f.coheres(), "a held stabilizer holonomy is not coherence");
    f.torsion.insert(half.rotor().class_key().expect("class"), FiledLoop { sigma: Vec::new(), period: 2, chord: triangle.chord });
    assert!(f.coheres(), "filed at its period, the loop coheres (A17)");
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

/// 52.3 — A LOOP OF ANY ORDER FILES. A (3,4,5) rotor on a chord's edge is a
/// holonomy with no period: unfiled, the loop holds against the form; filed
/// under its class at period 0, A19's spelling of infinite order, it
/// coheres; at a period the loop does not have it does not. The same rotor
/// on the chord's word is the transport composite, and the form is
/// cauterized by the same filing under the holonomy's class.
#[test]
fn a_loop_of_infinite_order_is_filed_at_period_zero_and_coheres() {
    let w = infinite();
    assert_eq!(w.order(), alphabet::Order::Infinite, "the (3,4,5) rotor has no period");
    let mut f = edge_chord_form(std::slice::from_ref(&w));
    let loops = f.loops.clone();
    let [lp] = loops.as_slice() else { panic!("one chord closes one loop") };
    assert!(!f.edge_coheres(lp), "unfiled, the edge of infinite order holds against the form");
    assert!(!f.coheres());
    let classes: Vec<_> = f.edge_classes(lp).scoped().map(|(_, c)| c.clone()).collect();
    let [class] = classes.as_slice() else { panic!("one member path, so one class") };
    f.torsion.insert(class.clone(), FiledLoop { sigma: Vec::new(), period: 0, chord: lp.chord });
    assert!(f.edge_coheres(lp), "filed at period 0, it coheres");
    assert!(f.coheres());
    f.torsion.insert(class.clone(), FiledLoop { sigma: Vec::new(), period: 4, chord: lp.chord });
    assert!(!f.coheres(), "a period the loop does not have is not a filing");

    let mut g = chord_form(std::slice::from_ref(&w));
    let loops = g.loops.clone();
    let [gl] = loops.as_slice() else { panic!("one chord closes one loop") };
    assert!(!gl.holonomy.is_identity(), "the transport composite carries the rotor");
    assert!(!g.is_cauterized(), "unfiled on the word, the form is not cauterized");
    g.torsion.insert(
        gl.holonomy.rotor().class_key().expect("a class"),
        FiledLoop { sigma: Vec::new(), period: 0, chord: gl.chord },
    );
    assert!(g.is_cauterized(), "filed at period 0 under the holonomy's class, it is");
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
        form_addr(1),
        alphabet::Derivation::spelled(w.clone()),
        stalk::Invariant::default(),
        Box::new(element::carriage(&q1(), &(q0(), q0()))),
        Vec::new(),
        None,
    ));
    host.vertices.push(Vertex::word_on(
        form_addr(2),
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
    assert_eq!(host.glued.get(&inner_e.address()).copied(), Some(gi));
    // The placement face is the one glue just pushed, so the tail is it.
    #[expect(
        clippy::disallowed_methods,
        reason = "rule 15 — slice pattern on the value just pushed: `glue` appends the placement face, so the tail is that face"
    )]
    let wrote = host.faces.last().map(|f| (f.i, f.j));
    assert_eq!(wrote, Some((0, gi)), "glue writes the Frame→Glued placement face");
    let mut drop = Keyed::new();
    drop.insert(vertex_ix_key(1), ());
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

/// The bundle's frame ratio and its sections are meaning: they are on
/// the form key and on the tape, and a tape with a zero denominator is
/// refused rather than repaired.
#[test]
fn ratio_and_sections_are_on_the_form_key() {
    let letter = alphabet::LetterId::quarter((0, 1)).expect("quarter");
    let w = Word::from_letters(&[(letter, false)]);
    let a = closed_word_form(w.clone(), None);
    let mut b = closed_word_form(w.clone(), None);
    b.ratio = ((2, 1), (2, 1));
    let mut c = closed_word_form(w, None);
    c.sections.insert(form_addr(7), vec![1]);
    let ea = as_element_form(&a, [1, 0, 0, 0]).expect("a");
    let eb = as_element_form(&b, [1, 0, 0, 0]).expect("b");
    let ec = as_element_form(&c, [1, 0, 0, 0]).expect("c");
    assert_ne!(ea.address(), eb.address(), "the frame ratio is on the key");
    assert_ne!(ea.address(), ec.address(), "a section is on the key");
    let back_b = crate::transducer::sentence::from_element_form(&eb).expect("v2 tape");
    assert_eq!(back_b.ratio, ((2, 1), (2, 1)));
    let back_c = crate::transducer::sentence::from_element_form(&ec).expect("v2 tape");
    assert_eq!(back_c.sections.get(&form_addr(7)), Some(&vec![1u16]));
    // Zero the height denominator on the tape: after certificates the
    // tail is ratio (8) + sections count (2) + torsion count (2) + axes
    // count (2). hd is the second u16 of the ratio.
    let mut bytes = ea.description().to_vec();
    let n = bytes.len();
    bytes[n - 12] = 0;
    bytes[n - 11] = 0;
    let mut broken = ea.clone();
    broken.set_description(bytes);
    assert!(crate::transducer::sentence::from_element_form(&broken).is_none(), "a zero denominator refuses the tape");
}

/// Depth is one plus the deepest glued form; gluing at equal depth is refused.
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
    let outer_copy = outer.clone();
    assert!(
        outer.glue(&outer_copy, form_addr(3), Word::identity(), [0, 0, 0, 1]).is_none(),
        "equal depth is refused"
    );
    assert!(
        outer.glue(&wrap, form_addr(4), Word::identity(), [0, 0, 0, 1]).is_none(),
        "deeper form is refused"
    );
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
    let mut index = crate::network::nerve::Keyed::new();
    for (i, w) in words.iter().enumerate() {
        index.insert(w.address(), i);
    }
    let index_of = |a: &[u8]| index.get(a).copied();
    let shapes: Vec<Option<Shape>> = words.iter().map(|w| element::shape_of_chain(w.sub())).collect();
    let shape_at = |i: usize| shapes.get(i).and_then(|s| s.as_ref());
    // A vertical bar far away: the bar rotated and translated.
    let target = vec![(5, 5, 1, 2), (6, 5, 1, 2)];
    let s = deconstruct(&words, &atlas, &index_of, &shape_at,&target, None);
    assert_eq!(s.cites.len(), 1);
    assert_eq!(s.cites[0].word, 0, "the bar, not the other-hued bar");
    assert!(s.is_closed());
    assert!(replay(&words, &s.cites, &target).is_empty(), "the citation replays to closure");
    assert!(
        !s.multiplicity.is_empty(),
        "stratum width is recorded per body"
    );
    // A 2× dilated bar with the other hues: the other bar at ρ = 2.
    let big = vec![(0, 0, 7, 8), (0, 1, 7, 8), (0, 2, 7, 8), (0, 3, 7, 8), (1, 0, 7, 8), (1, 1, 7, 8), (1, 2, 7, 8), (1, 3, 7, 8)];
    let s = deconstruct(&words, &atlas, &index_of, &shape_at,&big, None);
    assert_eq!(s.cites.len(), 1);
    assert_eq!(s.cites[0].word, 2);
    assert!(s.is_closed());
    // Two bodies and an unknown: two citations with one relative
    // transport between them, one residual body.
    let mixed = vec![(0, 0, 1, 2), (0, 1, 1, 2), (4, 4, 3, 4), (9, 9, 5, 6), (9, 10, 5, 6), (10, 9, 5, 6)];
    let s = deconstruct(&words, &atlas, &index_of, &shape_at,&mixed, None);
    assert_eq!(s.cites.len(), 2);
    assert_eq!(
        s.multiplicity.len(),
        4,
        "the chain and each of bar, dot, residual have a width, never one maximum"
    );
    assert_eq!(s.connection.len(), 1, "one relative transport between two cited bodies");
    assert_eq!(s.residual, vec![(9, 9, 5, 6), (9, 10, 5, 6), (10, 9, 5, 6)]);
    // Excluding the bar itself: a bar chain has no other word of hue 1→2.
    let s = deconstruct(&words, &atlas, &index_of, &shape_at,words[0].sub(), Some(0));
    assert!(s.cites.is_empty() && !s.is_closed(), "no word but itself speaks it");
}

#[test]
fn deconstruct_multiplicity_is_per_body() {
    let bar = from_canonical(&germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap());
    let dot = from_canonical(&germinate(&[(0, 0, 3u8)], &[(0, 0, 4u8)]).unwrap());
    let words = vec![bar, dot];
    let atlas = Stalks::over_elements(words.iter());
    let mut index = Keyed::new();
    for (i, w) in words.iter().enumerate() {
        index.insert(w.address(), i);
    }
    let index_of = |a: &[u8]| index.get(a).copied();
    let shapes: Vec<Option<Shape>> = words.iter().map(|w| element::shape_of_chain(w.sub())).collect();
    let shape_at = |i: usize| shapes.get(i).and_then(|s| s.as_ref());
    let mixed = vec![(0, 0, 1, 2), (0, 1, 1, 2), (4, 4, 3, 4)];
    let s = deconstruct(&words, &atlas, &index_of, &shape_at, &mixed, None);
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
    let target = vec![(0, 0, 1, 2), (0, 1, 1, 2)];
    let id = element::identity_versor();
    let cites: Vec<Cite> = vec![
        Cite { word: 0, transport: id.clone(), gauge: 0 },
        Cite { word: 1, transport: id.clone(), gauge: 0 },
    ];
    assert!(replay(&words, &cites[..1], &target).is_empty());
    assert_eq!(seam_defect(&words, &cites, &target), target, "both links spoken twice, differently");
    assert_eq!(verify(&words, &cites, &target), target);
    assert!(verify(&words, &cites[..1], &target).is_empty(), "one word, no seam");
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
    assert_eq!(f.by_class.get(&near.key()), Some(&vec![1u16, 3]), "both words on the near chart, in vertex order");
    assert_eq!(f.by_class.get(&far.key()), Some(&vec![2u16]), "the one word on the far chart");
    assert_eq!(f.by_class.get(&stalk::Invariant::counts(&[9]).key()), None, "a chart no vertex is on names nothing");
    for (chart, on) in f.by_class.scoped() {
        for &i in on {
            let v = f.vertices.get(i as usize).expect("an index the fiber names is a vertex");
            // THE FIBRE IS ASKED, not a representative read off the vertex:
            // `acts` holds every chart the vertex acts on, and the claim is
            // that this chart is among them (D7's whole point — one of two
            // is the defect, and a test that reads `acting()` would report
            // the representative agreeing with itself).
            assert!(
                matches!(v, Vertex::Word { acts, .. } if acts.get(chart.as_slice()).is_some()),
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
        f.by_transport.get(&identity_class),
        Some(&vec![1u16, 2]),
        "the identity class names both its vertices, across the two charts"
    );
    assert_eq!(
        f.by_transport.get(&quarter_class),
        Some(&vec![3u16]),
        "the quarter turn is its own class on a chart another vertex is also on"
    );
    // The chart fiber cannot answer the same question: it splits the one
    // class over two charts and merges two classes on one chart.
    assert_eq!(f.by_class.get(&near.key()), Some(&vec![1u16, 3]), "chart near: two classes");
    assert_eq!(f.by_class.get(&far.key()), Some(&vec![2u16]), "chart far: one of the identity class");
    // Every index the fiber names is a Word of that class, and every Word is
    // named by its own: the relation is total in both directions.
    for (k, on) in f.by_transport.scoped() {
        for &i in on {
            let v = f.vertices.get(i as usize).expect("an index the fiber names is a vertex");
            assert_eq!(
                crate::transducer::family::transport_class(v).map(|c| c.key()).as_deref(),
                Some(k.as_slice()),
                "vertex {i} is of the class that names it"
            );
        }
    }
    for (i, v) in f.vertices.iter().enumerate() {
        let Some(k) = crate::transducer::family::transport_class(v).map(|c| c.key()) else {
            continue;
        };
        assert!(
            f.by_transport.get(&k).is_some_and(|on| on.contains(&(i as u16))),
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
                f.by_address.get(address),
                Some(&(i as u16)),
                "vertex {i} at {:?} is not what the meet resolves",
                address.as_bytes()
            );
        }
    }
    assert_eq!(words, 2, "the chord form carries its two word vertices");
}
