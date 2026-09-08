// 18.2 — THE FORM: vertices, faces by constancy, glue and retract, depth,
// and what a chain deconstructs into.

use super::common::*;
use crate::network::nerve::{Keyed, ROOT};
use crate::transducer::alphabet::{self, Word};
use crate::transducer::blade::Shape;
use crate::transducer::element::{self, from_residual};
use crate::transducer::germinate::germinate;
use crate::transducer::sentence::*;
use crate::transducer::stalk::{self, Stalks};

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
    host.vertices.push(Vertex::Word {
        address: form_addr(1),
        word: w.clone(),
        carriage: Box::new(element::carriage(&q1(), &(q0(), q0()))),
        gauge: Vec::new(),
        class: stalk::Invariant::default(),
        contact: None,
    });
    host.vertices.push(Vertex::Word {
        address: form_addr(2),
        word: w.clone(),
        carriage: Box::new(element::carriage(&q1(), &(q0(), q1()))),
        gauge: Vec::new(),
        class: stalk::Invariant::default(),
        contact: None,
    });
    host.faces.push(FormFace { i: 0, j: 1, transport: host.face_of(0, 1).expect("0-1"), });
    host.faces.push(FormFace { i: 1, j: 2, transport: host.face_of(1, 2).expect("1-2"), });
    host.faces.push(FormFace { i: 0, j: 2, transport: host.face_of(0, 2).expect("0-2"), });
    let inner = closed_word_form(w.clone(), None);
    let inner_e = as_element_form(&inner, [1, 0, 0, 0]).expect("inner");
    let gi = host
        .glue(&inner, inner_e.address(), w.clone(), [1, 0, 0, 0])
        .expect("glue");
    assert_eq!(host.glued.get(&inner_e.address()).copied(), Some(gi));
    assert_eq!(
        host.faces.last().map(|f| (f.i, f.j)),
        Some((0, gi)),
        "glue writes the Frame→Glued placement face"
    );
    let mut drop = Keyed::new();
    drop.insert(vertex_ix_key(1), ());
    host.retract_vertices(&drop);
    assert_eq!(host.vertices.len(), 3, "Frame, remaining Word, Glued");
    for face in &host.faces {
        assert_eq!(
            host.face_of(face.i, face.j).as_ref(),
            Some(&face.transport),
            "every surviving face re-derives"
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
    taped.faces[fi].transport.word = Word::identity();
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
    if let Vertex::Word { carriage, .. } = &mut a.vertices[1] {
        **carriage = element::carriage(&q1(), &(q0(), q1()));
    }
    if let Vertex::Word { carriage, .. } = &mut b.vertices[1] {
        **carriage = element::carriage(
            &q1(),
            &(
                q0(),
                num_rational::BigRational::from_integer(num_bigint::BigInt::from(3)),
            ),
        );
    }
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
    let back_b = from_element_form(&eb).expect("v2 tape");
    assert_eq!(back_b.ratio, ((2, 1), (2, 1)));
    let back_c = from_element_form(&ec).expect("v2 tape");
    assert_eq!(back_c.sections.get(&form_addr(7)), Some(&vec![1u16]));
    // Zero the height denominator on the tape: after certificates the
    // tail is ratio (8) + sections count (2) + torsion count (2). hd is
    // the second u16 of the ratio.
    let mut bytes = ea.description().to_vec();
    let n = bytes.len();
    bytes[n - 10] = 0;
    bytes[n - 9] = 0;
    let mut broken = ea.clone();
    broken.set_description(bytes);
    assert!(from_element_form(&broken).is_none(), "a zero denominator refuses the tape");
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
    let bar = from_residual(&germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap());
    let dot = from_residual(&germinate(&[(0, 0, 3u8)], &[(0, 0, 4u8)]).unwrap());
    let bar_other = from_residual(&germinate(&[(0, 0, 7u8), (0, 1, 7)], &[(0, 0, 8u8), (0, 1, 8)]).unwrap());
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
    let bar = from_residual(&germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap());
    let dot = from_residual(&germinate(&[(0, 0, 3u8)], &[(0, 0, 4u8)]).unwrap());
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
        .fiber(ROOT)
        .map(|(_, w)| *w)
        .collect();
    assert_eq!(
        widths.iter().filter(|&&w| w >= 1).count(),
        2,
        "each cited body keeps its own stratum width; the chain records 0 when it is not a word: {widths:?}"
    );
}

/// The receiving direction: two citations carrying different words
/// onto the same cells are a seam defect the replay alone would miss;
/// a transport and its inverse are a flat loop, two translations are
/// not.
#[test]
fn seams_and_holonomy_are_located_exactly() {
    let bar = from_residual(&germinate(&[(0, 0, 1u8), (0, 1, 1)], &[(0, 0, 2u8), (0, 1, 2)]).unwrap());
    let bar_other = from_residual(&germinate(&[(0, 0, 7u8), (0, 1, 7)], &[(0, 0, 8u8), (0, 1, 8)]).unwrap());
    let words = vec![bar, bar_other];
    let target = vec![(0, 0, 1, 2), (0, 1, 1, 2)];
    let id = element::identity_versor();
    let cites: Vec<Cite> = vec![
        Cite { word: 0, transport: id.clone(), gauge: 0 },
        Cite { word: 1, transport: id.clone(), gauge: 0 },
    ];
    assert!(replay(&words, &cites[..1], &target).is_empty());
    assert_eq!(seam_defect(&words, &cites, &target), target, "both cells spoken twice, differently");
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

#[test]
fn a_sentence_chart_leads_with_axis_two() {
    let f = star_form(2, Vec::new());
    let e = as_element_form(&f, [1, 0, 0, 0]).expect("form element");
    assert!(e.is_sentence());
    let inv = stalk::invariant_of(&e);
    assert_eq!(inv.0.first().copied(), Some(2), "leading axis 2");
    assert!(inv.0.get(1).copied().unwrap_or(0) >= 2, "vertex count");
}

#[test]
fn face_constancy_is_the_word_not_the_versor() {
    let a = star_form(2, vec![(1, 2, Word::identity())]);
    let b = star_form(2, vec![(1, 2, Word::identity())]);
    let words1: Vec<_> = a.faces.iter().map(|fa| fa.transport.word.clone()).collect();
    let words2: Vec<_> = b.faces.iter().map(|fa| fa.transport.word.clone()).collect();
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
    let w = order2();
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
        i8 > 0 && i32n >= i8 && i32n <= i8.saturating_mul(5),
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
