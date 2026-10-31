// 13 — the Čech complex: transitions cancel, consilient charts glue, an
// equivocation shows up as curvature, and a fill is a gauge or a class.

use super::common::*;
use crate::transducer::alphabet::{Order, Rotor};
use crate::transducer::cech::*;
use crate::transducer::element::{identity_versor as ident, same_versor};

/// A TRANSITION BETWEEN TWO SPOKEN SENTENCES ANSWERS A ROTOR.
///
/// The court's glue composes SENTENCES: `glue_fetched` reads
/// `transition(w, &spoken).and_then(rotor_of)` where both operands are
/// forms a peer spoke. A sentence carries no tape — `letters()` refuses
/// its tag, because a sentence is one level above a word — so the rotor
/// is read from what the form's vertices agree on, or the glue writes no
/// six-plane rotor on any edge, `Complex::fill` short-circuits on the
/// identity coefficient before `classify`, and `Fill::Class` and
/// `Fill::Unspellable` are unreachable.
///
/// A form whose word vertices agree on one rotor up to projective
/// equality HAS that rotor. The discriminating case: the two forms here
/// carry 90° turns on `(0, 1)` and on `(2, 3)`, which have the SAME
/// conformal action on the base — the second is the identity there — so
/// neither sentence has a versor at all and `Cl(3, 1)` cannot separate
/// them. The rotors can.
#[test]
fn transition_between_answers_agreed_rotor_for_sentences() {
    use crate::transducer::alphabet::{self, LetterId, Word};
    use crate::transducer::element::Element;
    use crate::transducer::sentence::{as_element_form, Form, Vertex};
    use crate::transducer::stalk;
    let q = |n: i64| num_rational::BigRational::from_integer(num_bigint::BigInt::from(n));
    use crate::transducer::sentence::tests::common::FRAME;
    // AT1: a star overlap's place is MEASURED (the coboundary word's base
    // versor), and these words carry no conformal action on the base — the
    // premise of the test — so no star derives (rule 11). The member stands
    // at the fixtures' frame without one.
    let stand = |f: &mut Form, v: &Vertex| f.stand_member(v, FRAME).expect("a word vertex stands");
    let word_vertex = |w: &Word| {
        Vertex::word_on(
            alphabet::Derivation::spelled(w.clone()),
            stalk::Invariant::counts(&[3, 5]),
            Box::new(crate::transducer::element::carriage(&q(1), &(q(0), q(1)))),
            Vec::new(),
            [FRAME],
            None,
        )
    };
    let spoken = |w: &Word| -> Element {
        let mut f = Form::default();
        stand(&mut f, &word_vertex(w));
        f.reindex();
        as_element_form(&f, vol()).expect("a form with a vertex and a magnitude")
    };
    let q01 = Word::from_letters(&[(LetterId::turn_90((0, 1)).expect("the sensor plane"), false)]);
    let q23 = Word::from_letters(&[(LetterId::turn_90((2, 3)).expect("the far plane"), false)]);
    let (a, b) = (spoken(&q01), spoken(&q23));

    assert!(a.is_sentence() && b.is_sentence(), "both are forms, not words");
    assert!(a.letters().is_none(), "and a sentence carries no tape to read a rotor off");
    assert!(
        a.versor().is_none() && b.versor().is_none(),
        "nor a conformal action: Cl(3,1) holds nothing that separates these two"
    );

    let (ra, rb) = (
        a.carried_rotor().expect("the form's one word vertex agrees with itself"),
        b.carried_rotor().expect("likewise"),
    );
    assert!(ra.projective_eq(&q01.rotor()), "the rotor its vertices agree on");
    assert!(!ra.projective_eq(&rb), "and the two forms turn in different planes");

    let g = transition(&a, &b).expect("a transition between two spoken sentences");
    let got = rotor_of(&g).expect("and it has a rotor");
    let want = rb.geo(&ra.reverse());
    assert!(got.projective_eq(&want), "which is g_y ∘ g_x⁻¹ in Cl(4,0)");
    assert!(!got.is_identity(), "and not the identity");
    // AND THE ELEMENT CARRYING IT IS NOT THE IDENTITY ELEMENT: `is_identity`
    // reads the axes, and this transition has a spatial axis though it has
    // no versor, so `glue_fetched`'s identity arm does not discard it.
    assert_eq!(g.axes(), vec![crate::transducer::element::FIBRE_SPATIAL]);
    assert!(!g.is_identity(), "a turn the sensor cannot see is still a turn");

    // A FORM WHOSE VERTICES DISAGREE HAS NO ONE ROTOR, and says so rather
    // than picking a representative (rule 15).
    //
    // AT1 (Q1): a Word vertex is keyed by its TRANSPORT CLASS, and the
    // 90° turns on `(0, 1)` and `(2, 3)` are one class — one vertex, the
    // second member merging into the first. Two disagreeing vertices are two
    // CLASSES: the 90° turn against the (3, 4, 5) turn, of infinite order.
    let p01 = Word::from_letters(&[(LetterId::pythagorean_5((0, 1)).expect("the sensor plane"), false)]);
    let mut mixed = Form::default();
    for w in [&q01, &p01] {
        stand(&mut mixed, &word_vertex(w));
    }
    mixed.reindex();
    assert_eq!(mixed.vertices.len(), 3, "the Frame and two word classes");
    let m = as_element_form(&mixed, vol()).expect("a form of two disagreeing vertices");
    assert!(
        m.carried_rotor().is_none(),
        "two rotors are not one rotor, and a representative would be a position pick"
    );
}

/// A TRANSITION is antisymmetric: `g_XY ∘ g_YX = 1` (`g_YX = g_XY⁻¹`).
#[test]
fn transition_cancels_with_reverse() {
    let (gx, gy) = (motion(3), motion(6));
    let f = transition(&gx, &gy).unwrap();
    let r = transition(&gy, &gx).unwrap();
    assert!(f.compose(&r).expect("the composite forms").is_identity(), "g_XY ∘ g_YX = 1 — asymmetric coboundary");
}

/// Consilient charts glue: identical views form one sheaf.
#[test]
fn glues_accepts_consilient_charts() {
    let g = motion(3);
    let id = transition(&g, &g).unwrap();
    assert!(id.is_identity());
    assert_eq!(glues(&id, &id, &id), Some(true), "identical views → one sheaf");
}

/// A global section always glues, since a coboundary satisfies δδ = 0.
#[test]
fn glues_reads_global_section() {
    let (ga, gb, gc) = (motion(3), motion(6), motion(9));
    let g_ab = transition(&ga, &gb).unwrap();
    let g_bc = transition(&gb, &gc).unwrap();
    let g_ca = transition(&gc, &ga).unwrap();
    assert_eq!(glues(&g_ab, &g_bc, &g_ca), Some(true), "a coboundary always satisfies δδ = 0");
}

/// An equivocation shows up as curvature: a lone disagreeing edge does not
/// glue and its holonomy is not the identity.
#[test]
fn glues_refuses_equivocation_as_curvature() {
    let g = motion(3);
    let id = transition(&g, &g).unwrap();
    let disagree = transition(&motion(3), &motion(6)).unwrap();
    assert!(!disagree.is_identity(), "the disagreeing edge is nontrivial");
    assert_eq!(glues(&id, &id, &disagree), Some(false), "a lone dialect edge → curvature (δσ ≠ 0)");
    assert!(
        !holonomy(&id, &id, &disagree).expect("the holonomy forms").is_identity(),
        "the holonomy is the exact obstruction"
    );
}

/// The link of one vertex is only the incident simplices; a tetra not on
/// the link is not formed. Volume-zero overlaps form nothing.
#[test]
fn link_is_star_and_volume_gates_simplices() {
    let mut c = Complex::new();
    let (a, b, cc, d) = (v(1), v(2), v(3), v(4));
    for x in [&a, &b, &cc, &d] {
        c.set_section(x, Section::at((*x).clone(), Rotor::identity()), vol());
    }
    let id = Rotor::identity();
    c.set_edge(&a, &b, id.clone(), ident(), vol());
    c.set_edge(&a, &cc, id.clone(), ident(), vol());
    c.set_edge(&b, &cc, id.clone(), ident(), vol());
    c.set_edge(&cc, &d, id, ident(), vol());
    c.coboundary_on_link(&a, |_| vol());
    for k in c.link(&a, 2) {
        let s = c.simplex(&k).expect("link key is held");
        assert!(s.contains(&a), "link(a) contains only a");
    }
    assert!(c.degrees.get(3).is_none_or(|l| l.is_empty()), "no tetra on a's link");
    let mut empty = Complex::new();
    empty.set_section(&a, Section::at(a.clone(), Rotor::identity()), [0; 4]);
    empty.set_edge(&a, &b, Rotor::identity(), ident(), [0; 4]);
    assert!(
        empty.degrees.get(2).is_none_or(|l| l.is_empty()),
        "zero remainder volume forms no higher simplex"
    );
}

/// 78.26(6), TP1 — A FILLED RING WHOSE COEFFICIENT MOVES IS CUT OPEN, AND THE
/// LINK SAYS WHICH: the triangle filled at holonomy 1 comes back reopened when
/// a chord's gauge moves; a ring that never filled is reopened silently, not
/// counted as a cut, since there was no coherence to lose.
#[test]
fn coboundary_on_link_names_filled_ring_cut_by_gauge() {
    let mut c = Complex::new();
    let (a, b, cc) = (v(1), v(2), v(3));
    for x in [&a, &b, &cc] {
        c.set_section(x, Section::at((*x).clone(), Rotor::identity()), vol());
    }
    let q = turn_90();
    c.set_edge(&a, &b, q.clone(), ident(), vol());
    c.set_edge(&b, &cc, q.clone(), ident(), vol());
    c.set_edge(&a, &cc, q.geo(&q), ident(), vol());
    assert!(c.coboundary_on_link(&a, |_| vol()).is_empty(), "a new ring cuts nothing");
    let key = c.link(&a, 2)[0].clone();
    assert!(matches!(c.fill(&key), CoboundaryOutcome::Attached(Fill::Filled(..))), "the ring closes");
    c.set_edge(&a, &cc, Rotor::identity(), ident(), vol());
    assert_eq!(
        c.coboundary_on_link(&a, |_| vol()),
        vec![(key.clone(), Cut::Filled)],
        "the filled ring, cut open by the moved gauge"
    );
    // Now open and unasked: moving the gauge again cuts no coherence.
    c.set_edge(&a, &cc, q.geo(&q), ident(), vol());
    assert!(c.coboundary_on_link(&a, |_| vol()).is_empty(), "an open ring reopened is no decoherence");
}

/// A coboundary 1-cochain fills; a finite leftover is Congruent; an
/// infinite-order leftover (prime 5) is a Class.
#[test]
fn fill_is_gauge_or_class() {
    let mut c = Complex::new();
    let (a, b, cc) = (v(1), v(2), v(3));
    for x in [&a, &b, &cc] {
        c.set_section(x, Section::at((*x).clone(), Rotor::identity()), vol());
    }
    let q = turn_90();
    c.set_edge(&a, &b, q.clone(), ident(), vol());
    c.set_edge(&b, &cc, q.clone(), ident(), vol());
    c.set_edge(&a, &cc, q.geo(&q), ident(), vol());
    c.coboundary_on_link(&a, |_| vol());
    let tri = c.link(&a, 2);
    assert_eq!(tri.len(), 1);
    let key = tri[0].clone();
    // EXACT: a coboundary fills by the identity gauge with no placement,
    // and some gauge coming back is true of every gauge including a wrong
    // one.
    match c.fill(&key) {
        CoboundaryOutcome::Attached(Fill::Filled(g, carriage)) => {
            assert!(g.is_identity(), "a coboundary's leftover is the identity: {g:?}");
            assert!(same_versor(&carriage, &ident()), "and carries no placement");
        }
        other => panic!("a coboundary fills by a gauge, not {other:?}"),
    }
    // The holonomy left over is q·q — a 180° turn, since the two legs are
    // 90° turns and the chord is now the identity — so it is congruent at
    // TWO, and the period is asserted: a congruence at any period is not
    // this claim.
    c.set_edge(&a, &cc, Rotor::identity(), ident(), vol());
    c.coboundary_on_link(&a, |_| vol());
    let key = c.link(&a, 2)[0].clone();
    match c.fill(&key) {
        CoboundaryOutcome::Attached(Fill::Congruent(_, g, ell)) => {
            assert_eq!(ell, 2, "two 90° turns against an identity chord close at two");
            assert_eq!(g.order(), Some(Order::Finite(2)), "and the filed rotor says so");
            assert!(
                g.projective_eq(&turn_90().geo(&turn_90())),
                "the filed rotor is the holonomy itself: {g:?}"
            );
        }
        other => panic!("a finite-order leftover is congruent, not {other:?}"),
    }
    // The (3,4,5) rotor has no finite order, so it is a class — and the
    // class names that rotor and carries no placement residue.
    c.set_edge(&a, &b, pythag5(), ident(), vol());
    c.set_edge(&b, &cc, Rotor::identity(), ident(), vol());
    c.set_edge(&a, &cc, Rotor::identity(), ident(), vol());
    c.coboundary_on_link(&a, |_| vol());
    let key = c.link(&a, 2)[0].clone();
    match c.fill(&key) {
        CoboundaryOutcome::Attached(Fill::Class(kappa, r)) => {
            assert_eq!(
                r.order(),
                Some(Order::Infinite),
                "the (3,4,5) rotor is a versor of infinite order"
            );
            assert!(
                r.projective_eq(&pythag5()) || r.reverse().projective_eq(&pythag5()),
                "the class names the rotor that caused it: {r:?}"
            );
            assert!(same_versor(&kappa.carriage, &ident()), "no placement residue here");
        }
        other => panic!("an infinite-order leftover is a class, not {other:?}"),
    }
}

/// A PLACEMENT that does not close is a class, and the class carries it.
/// This is the reading a rotor-only section could not make at all: every
/// rotor here is the identity, so the rotor half of the holonomy closes
/// exactly and the obstruction lives entirely in the carriage.
#[test]
fn fill_reads_unclosed_placement_as_class() {
    let mut c = Complex::new();
    let (a, b, cc) = (v(1), v(2), v(3));
    for x in [&a, &b, &cc] {
        c.set_section(x, Section::at((*x).clone(), Rotor::identity()), vol());
    }
    let t = |r, col| crate::transducer::element::translator_versor(r, col);
    let id = Rotor::identity();
    c.set_edge(&a, &b, id.clone(), t(0, 1), vol());
    c.set_edge(&b, &cc, id.clone(), t(0, 1), vol());
    // a → cc directly by ONE step, where a → b → cc is two: the triangle's
    // placements do not close and its rotors do.
    c.set_edge(&a, &cc, id, t(0, 1), vol());
    c.coboundary_on_link(&a, |_| vol());
    let key = c.link(&a, 2)[0].clone();
    let coeff = c.simplex(&key).expect("the triangle is held");
    assert!(
        coeff.coefficient.is_identity(),
        "every rotor is the identity, so the rotor half closes exactly"
    );
    assert!(
        !same_versor(&coeff.carriage, &ident()),
        "and the placement half does not: it is the whole obstruction"
    );
    match c.fill(&key) {
        CoboundaryOutcome::Attached(Fill::Class(kappa, r)) => {
            assert!(r.is_identity(), "the class's rotor part is the identity");
            assert!(
                !same_versor(&kappa.carriage, &ident()),
                "and the class carries the placement that did not close"
            );
        }
        other => panic!("an unclosed placement is a class, not {other:?}"),
    }
}

/// file_congruent writes the class-key index; court torsion reads it.
#[test]
fn congruent_agrees_with_class_key() {
    let mut c = Complex::new();
    let q = turn_90();
    let ck = q.class_key().expect("90° turn class");
    c.file_congruent(v(1), v(2), q.clone(), 4);
    // The index is keyed by the orbit's WHOLE class key and asked here by
    // the rotor's conjugacy key, which is that key's depth-one stratum:
    // the answer is the fibre under it, holding the one orbit filed.
    let got = c.congruent(ck.as_bytes());
    let [(rotor, ell)] = got.as_slice() else {
        panic!("one Congruent orbit stands under the 90° turn's conjugacy class, got {got:?}")
    };
    assert_eq!(*ell, 4);
    assert!(rotor.projective_eq(&q), "the index holds the rotor that was filed");
    // A class key that was NEVER filed is absent, asked at a rotor that
    // HAS a class key: the identity may have none, and a claim that passes
    // on its `None` arm reads the index not at all. `pythag5` has one and
    // was never filed, so the index must answer for it.
    let other = pythag5().class_key().expect("the (3,4,5) rotor has a class key");
    assert!(
        c.congruent(other.as_bytes()).is_empty(),
        "a class never filed is not in the index"
    );
}

/// A SIMPLEX ON NO VERTICES IS NO SIMPLEX. Degree zero is a point, and a
/// value with no vertices is not one: it is not remembered, and every
/// reader of `degree` takes `n − 1` whole rather than flooring it.
#[test]
fn remember_refuses_simplex_without_vertices() {
    let mut c = Complex::new();
    c.remember(Simplex {
        vertices: Vec::new(),
        coefficient: Rotor::identity(),
        carriage: ident(),
        volume: vol(),
        fill: Fill::Open,
    });
    assert_eq!(c.simplex_count(), 0, "no simplex was remembered");
    assert!(c.get(&[]).is_none(), "and none is there to read");
}

/// A fill is attempted once: a second fill answers `AlreadyFilled` and the
/// stored fill stands.
#[test]
fn fill_attempts_once() {
    let mut c = Complex::new();
    let (a, b) = (v(1), v(2));
    c.set_section(&a, Section::at(a.clone(), Rotor::identity()), vol());
    c.set_section(&b, Section::at(b.clone(), Rotor::identity()), vol());
    c.set_edge(&a, &b, Rotor::identity(), ident(), vol());
    let key = c.link(&a, 1)[0].clone();
    let first = c.fill(&key);
    assert!(matches!(first.attached(), Some(Fill::Filled(..))));
    let second = c.fill(&key);
    assert!(
        matches!(second, CoboundaryOutcome::AlreadyFilled),
        "a second fill is not a transition — and it is now a distinct TYPE \
         from the state the simplex holds, so it can no longer be stored"
    );
    assert!(matches!(c.simplex(&key).map(|c| &c.fill), Some(Fill::Filled(..))));
}
