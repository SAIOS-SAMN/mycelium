// 10 — THE GAUGE and its coboundary: the hue permutation around a loop,
// the spanning forest that solves it, and the residue a placement leaves.

use super::common::*;
use crate::network::nerve::{Keyed};
use crate::transducer::alphabet::{self, Word};
use crate::transducer::element::{self};
use crate::transducer::sentence::*;
use crate::transducer::stalk::{self};

#[test]
fn placement_residue_is_the_affine_translation() {
    let mut f = chord_form(&[Word::identity()]);
    for fa in &mut f.faces {
        fa.transport.gauge.clear();
    }
    let chord = f.loops[0].chord as usize;
    f.faces[chord].transport.carriage = element::carriage(&q1(), &(q1(), q0()));
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    let kappa = f.class_vector(0).expect("loop");
    assert!(
        kappa.occupies(Axis::Placement),
        "a nonzero composed shift occupies Placement"
    );
    assert!(
        element::translator_of(&kappa.carriage)
            .is_some_and(|t| !element::same_versor(&t, &element::identity_versor())),
        "the residue is the translator of the holonomy versor"
    );
    let snaps = f.pivot_loop(
        [0xa4; 32],
        &Keyed::new(),
        &crate::transducer::family::NoCrystal,
        &mut crate::physics::Physics::default().stroke_n(8),
        None,
        0,
    );
    assert!(
        snaps.iter().any(|s| matches!(s, Snap::Discharged { certificate, .. } if certificate.axis == Axis::Placement)),
        "the residue discharges on Placement: {snaps:?}"
    );
    let after = f.class_vector(0).expect("loop remains");
    assert!(
        !after.occupies(Axis::Placement),
        "Placement is zero after the pivot"
    );
}

#[test]
fn hue_cycle_type_composes_around_the_loop() {
    let mut f = chord_form(&[Word::identity()]);
    for fa in &mut f.faces {
        fa.transport.gauge.clear();
    }
    let chord = f.loops[0].chord as usize;
    f.faces[chord].transport.gauge = vec![(3, 4), (4, 3)];
    f.loops = Form::loops_of(&f.faces, f.vertices.len());
    let kappa = f.class_vector(0).expect("loop");
    assert!(
        kappa.occupies(Axis::Hue),
        "a non-identity composed gauge occupies Hue, got {:?}",
        kappa.hue
    );
    let snaps = f.pivot_loop(
        [0xa5; 32],
        &Keyed::new(),
        &crate::transducer::family::NoCrystal,
        &mut crate::physics::Physics::default().stroke_n(8),
        None,
        0,
    );
    assert!(
        snaps.iter().any(|s| matches!(s, Snap::Discharged { certificate, .. } if certificate.axis == Axis::Hue)),
        "the cycle type discharges on Hue: {snaps:?}"
    );
    let after = f.class_vector(0).expect("loop remains");
    assert!(!after.occupies(Axis::Hue), "Hue is identity after the pivot");
}

#[test]
fn conjugate_hue_permutations_are_one_cycle_type() {
    let a = CycleType::of(&[(1, 2), (2, 1)]);
    let b = CycleType::of(&[(3, 4), (4, 3)]);
    assert_eq!(a, b, "cycle type is sorted lengths, not the listing");
    assert_eq!(a, CycleType::Type(vec![2]));
    let z = element::identity_versor();
    let ka = ClassVector::read(&Word::identity(), z.clone(), a).expect("identity");
    let kb = ClassVector::read(&Word::identity(), z, b).expect("identity");
    assert!(ka.occupies(Axis::Hue) && kb.occupies(Axis::Hue));
    assert_eq!(
        ka.class_key(),
        kb.class_key(),
        "(1 2) and (3 4) are one Hue coordinate"
    );
    assert!(!ka.is_zero(), "a hue cycle is not closed");
}

#[test]
fn coboundary_gauge_matches_by_address() {
    let q = Word::from_letters(&[(alphabet::LetterId::quarter((0, 1)).unwrap(), false)]);
    let mut a = Form::default();
    a.vertices.push(word_vertex(1, q.clone()));
    a.vertices.push(word_vertex(2, Word::identity()));
    a.faces.push(FormFace {
        i: 0,
        j: 1,
        transport: a.face_of(0, 1).expect("a star 1"),
    });
    a.faces.push(FormFace {
        i: 0,
        j: 2,
        transport: a.face_of(0, 2).expect("a star 2"),
    });
    a.vertices.push(Vertex::Rebased {
        by: 0,
        of: 1,
        carriage: Box::new(element::carriage(&q1(), &(q0(), q1()))),
        gauge: Vec::new(),
        class: stalk::Invariant::default(),
    });
    a.vertices.push(Vertex::Unfolded {
        measure: Measure::Occupied,
    });
    a.reindex();
    let mut b = Form::default();
    b.vertices.push(word_vertex(2, Word::identity()));
    b.vertices.push(word_vertex(1, q.clone()));
    b.faces.push(FormFace {
        i: 0,
        j: 1,
        transport: b.face_of(0, 1).expect("b star 1"),
    });
    b.faces.push(FormFace {
        i: 0,
        j: 2,
        transport: b.face_of(0, 2).expect("b star 2"),
    });
    b.vertices.push(Vertex::Unfolded {
        measure: Measure::Occupied,
    });
    b.vertices.push(Vertex::Rebased {
        by: 0,
        of: 2,
        carriage: Box::new(element::carriage(&q1(), &(q0(), q1()))),
        gauge: Vec::new(),
        class: stalk::Invariant::default(),
    });
    b.reindex();
    assert!(
        coboundary_gauge(&a, &b).is_some(),
        "the same words in a different vertex order are one coboundary"
    );
}

#[test]
fn solve_coboundary_is_a_spanning_forest() {
    let mut f = Form::default();
    f.vertices.push(Vertex::Word {
        address: form_addr(1),
        word: Word::identity(),
        carriage: Box::new(element::carriage(&q1(), &(q0(), q0()))),
        gauge: Vec::new(),
        class: stalk::Invariant::default(),
        contact: None,
    });
    f.vertices.push(Vertex::Word {
        address: form_addr(2),
        word: Word::identity(),
        carriage: Box::new(element::carriage(&q1(), &(q0(), q0()))),
        gauge: Vec::new(),
        class: stalk::Invariant::default(),
        contact: None,
    });
    f.faces.push(FormFace::new(0, 1, Word::identity(),));
    let gamma = f.solve_coboundary().expect("an isolated vertex roots its own tree");
    assert_eq!(gamma.len(), 3);
    assert!(gamma[0].is_identity(), "the least index of a component is γ = 1");
    assert!(gamma[2].is_identity(), "vertex 2 is its own root");
    f.faces.push(FormFace::new(0, 9, Word::identity(),));
    assert!(
        f.solve_coboundary().is_none(),
        "a face that names a vertex outside the form refuses"
    );
    assert!(f.regauge(&[Word::identity()]).is_none(), "a short gauge refuses");
}

#[test]
fn holonomy_is_oriented() {
    let w = order4();
    let mut fwd = Form::default();
    fwd.vertices.push(word_vertex(1, Word::identity()));
    fwd.vertices.push(word_vertex(2, Word::identity()));
    fwd.faces.push(FormFace {
        i: 0,
        j: 1,
        transport: fwd.face_of(0, 1).expect("star"),
    });
    fwd.faces.push(FormFace {
        i: 0,
        j: 2,
        transport: fwd.face_of(0, 2).expect("star"),
    });
    fwd.faces.push(FormFace::new(1, 2, w.clone()));
    fwd.loops = Form::loops_of(&fwd.faces, fwd.vertices.len());
    let mut rev = Form::default();
    rev.vertices.push(word_vertex(1, Word::identity()));
    rev.vertices.push(word_vertex(2, Word::identity()));
    rev.faces.push(FormFace {
        i: 0,
        j: 1,
        transport: rev.face_of(0, 1).expect("star"),
    });
    rev.faces.push(FormFace {
        i: 0,
        j: 2,
        transport: rev.face_of(0, 2).expect("star"),
    });
    rev.faces.push(FormFace::new(2, 1, w.inverse()));
    rev.loops = Form::loops_of(&rev.faces, rev.vertices.len());
    assert_eq!(fwd.loops.len(), 1);
    assert_eq!(rev.loops.len(), 1);
    assert_eq!(
        fwd.loops[0].holonomy.class_key(),
        rev.loops[0].holonomy.class_key(),
        "a reversed chord stores the inverse word and the same class"
    );
}

#[test]
fn relative_gauge_composite_fixes_what_neither_fixes() {
    let t = vec![(1u8, 2), (2, 1)];
    assert!(
        compose_gauge(&t, &t).is_empty(),
        "two (1 2) transpositions compose to the identity"
    );
    let a = vec![(1u8, 2)];
    let b = vec![(2u8, 1)];
    let c = compose_gauge(&b, &a);
    assert!(
        c.iter().all(|&(h, k)| h != 1 || k != 1),
        "composite listing omits fixed points, got {c:?}"
    );
    assert_eq!(
        gauge_image(&gauge_keyed(&c), 1),
        1,
        "1→2 then 2→1 fixes 1, which neither map lists as fixed"
    );
}
