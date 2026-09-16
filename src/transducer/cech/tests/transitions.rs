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
/// its tag, because a sentence is one level above a word — so `rotor_of`
/// answered `None` on that path even after a composite of two WORDS
/// carried its own. The glue then took its `None` arm on every edge it
/// wrote, `set_edge` never saw a six-plane rotor, `Complex::fill`
/// short-circuited on the identity coefficient before `classify`, and
/// `Fill::Class` and `Fill::Unspellable` were unreachable in the tree.
///
/// A form whose word vertices agree on one rotor up to projective
/// equality HAS that rotor. The discriminating case is the same one the
/// composite used: the two forms here carry quarters on `(0, 1)` and on
/// `(2, 3)`, which have the SAME conformal action on the base — the
/// second is the identity there — so neither sentence has a versor at all
/// and `Cl(3, 1)` cannot separate them. The rotors can.
#[test]
fn a_transition_between_two_sentences_answers_the_rotor_its_vertices_agree_on() {
    use crate::transducer::alphabet::{self, LetterId, Word};
    use crate::transducer::element::Element;
    use crate::transducer::sentence::{as_element_form, Form, Vertex};
    use crate::transducer::stalk;
    let q = |n: i64| num_rational::BigRational::from_integer(num_bigint::BigInt::from(n));
    let spoken = |w: &Word| -> Element {
        let mut f = Form::default();
        f.vertices.push(Vertex::word_on(
            crate::transducer::address::Address::new(Vec::new()),
            alphabet::Derivation::spelled(w.clone()),
            stalk::Invariant(vec![3, 5]),
            Box::new(crate::transducer::element::carriage(&q(1), &(q(0), q(1)))),
            Vec::new(),
            None,
        ));
        f.faces.push(f.face_of(0, 1).expect("the frame star"));
        f.reindex();
        as_element_form(&f, vol()).expect("a form with a vertex and a magnitude")
    };
    let q01 = Word::from_letters(&[(LetterId::quarter((0, 1)).expect("the sensor plane"), false)]);
    let q23 = Word::from_letters(&[(LetterId::quarter((2, 3)).expect("the far plane"), false)]);
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
    let want = rb.geo(&ra.inverse().expect("a quarter inverts"));
    assert!(got.projective_eq(&want), "which is g_y ∘ g_x⁻¹ in Cl(4,0)");
    assert!(
        !got.is_identity(),
        "and not the identity the court wrote on every edge it could write"
    );
    // AND THE ELEMENT CARRYING IT IS NOT THE IDENTITY ELEMENT. `is_identity`
    // is `axes().is_empty()`, `axes` read the versor alone, and this
    // transition has none — so a rotor the court can finally see would have
    // arrived at `glue_fetched`'s `Some(g) if g.is_identity()` arm and been
    // discarded one line after it was won.
    assert_eq!(g.axes(), vec![crate::transducer::element::FIBER_SPATIAL]);
    assert!(!g.is_identity(), "a turn the sensor cannot see is still a turn");

    // A FORM WHOSE VERTICES DISAGREE HAS NO ONE ROTOR, and says so rather
    // than picking a representative (rule 15).
    let mut mixed = Form::default();
    for w in [&q01, &q23] {
        mixed.vertices.push(Vertex::word_on(
            crate::transducer::address::Address::new(Vec::new()),
            alphabet::Derivation::spelled(w.clone()),
            stalk::Invariant(vec![3, 5]),
            Box::new(crate::transducer::element::carriage(&q(1), &(q(0), q(1)))),
            Vec::new(),
            None,
        ));
    }
    for j in 1..=2u16 {
        mixed.faces.push(mixed.face_of(0, j).expect("the frame star"));
    }
    mixed.reindex();
    let m = as_element_form(&mixed, vol()).expect("a form of two disagreeing vertices");
    assert!(
        m.carried_rotor().is_none(),
        "two rotors are not one rotor, and a representative would be a position pick"
    );
}

/// A TRANSITION is antisymmetric: `g_XY ∘ g_YX = 1` (`g_YX = g_XY⁻¹`).
#[test]
fn a_transition_and_its_reverse_cancel() {
    let (gx, gy) = (motion(3), motion(6));
    let f = transition(&gx, &gy).unwrap();
    let r = transition(&gy, &gx).unwrap();
    assert!(f.compose(&r).is_identity(), "g_XY ∘ g_YX = 1 — asymmetric coboundary");
}

#[test]
fn consilient_charts_glue() {
    let g = motion(3);
    let id = transition(&g, &g).unwrap();
    assert!(id.is_identity());
    assert!(glues(&id, &id, &id), "identical views → one sheaf");
}

#[test]
fn a_global_section_always_glues() {
    let (ga, gb, gc) = (motion(3), motion(6), motion(9));
    let g_ab = transition(&ga, &gb).unwrap();
    let g_bc = transition(&gb, &gc).unwrap();
    let g_ca = transition(&gc, &ga).unwrap();
    assert!(glues(&g_ab, &g_bc, &g_ca), "a coboundary always satisfies δδ = 0");
}

#[test]
fn an_equivocation_shows_up_as_curvature() {
    let g = motion(3);
    let id = transition(&g, &g).unwrap();
    let disagree = transition(&motion(3), &motion(6)).unwrap();
    assert!(!disagree.is_identity(), "the disagreeing edge is nontrivial");
    assert!(!glues(&id, &id, &disagree), "a lone dialect edge → curvature (δσ ≠ 0)");
    assert!(!holonomy(&id, &id, &disagree).is_identity(), "the holonomy is the exact obstruction");
}

/// The link of one vertex is only the incident cells; a tetra not on
/// the link is not formed. Volume-zero overlaps form nothing.
#[test]
fn the_link_is_the_star_and_volume_gates_cells() {
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
        let cell = c.cell(&k).expect("link key is held");
        assert!(cell.contains(&a), "link(a) contains only a");
    }
    assert!(c.degrees.get(3).is_none_or(|l| l.is_empty()), "no tetra on a's link");
    let mut empty = Complex::new();
    empty.set_section(&a, Section::at(a.clone(), Rotor::identity()), [0; 4]);
    empty.set_edge(&a, &b, Rotor::identity(), ident(), [0; 4]);
    assert!(
        empty.degrees.get(2).is_none_or(|l| l.is_empty()),
        "zero remainder volume forms no higher cell"
    );
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
    let q = quarter();
    c.set_edge(&a, &b, q.clone(), ident(), vol());
    c.set_edge(&b, &cc, q.clone(), ident(), vol());
    c.set_edge(&a, &cc, q.geo(&q), ident(), vol());
    c.coboundary_on_link(&a, |_| vol());
    let tri = c.link(&a, 2);
    assert_eq!(tri.len(), 1);
    let key = tri[0].clone();
    // EXACT: a coboundary fills by the identity gauge with no placement.
    // `matches!(_, Fill::Filled(_))` said only that some gauge came back,
    // which is true of every gauge including a wrong one.
    match c.fill(&key) {
        Fill::Filled(g, carriage) => {
            assert!(g.is_identity(), "a coboundary's leftover is the identity: {g:?}");
            assert!(same_versor(&carriage, &ident()), "and carries no placement");
        }
        other => panic!("a coboundary fills by a gauge, not {other:?}"),
    }
    // The holonomy left over is q·q — a HALF turn, since the two legs are
    // quarters and the chord is now the identity — so it is congruent at
    // TWO. The period was unasserted before, so a congruence at any period
    // passed, including this one under a comment claiming four.
    c.set_edge(&a, &cc, Rotor::identity(), ident(), vol());
    c.coboundary_on_link(&a, |_| vol());
    let key = c.link(&a, 2)[0].clone();
    match c.fill(&key) {
        Fill::Congruent(g, ell) => {
            assert_eq!(ell, 2, "two quarter turns against an identity chord close at two");
            assert_eq!(g.order(), Some(Order::Finite(2)), "and the filed rotor says so");
            assert!(
                g.projective_eq(&quarter().geo(&quarter())),
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
        Fill::Class(kappa, r) => {
            assert_eq!(
                r.order(),
                Some(Order::Infinite),
                "the (3,4,5) rotor is a versor of infinite order"
            );
            assert!(
                r.projective_eq(&pythag5())
                    || r.inverse().is_some_and(|i| i.projective_eq(&pythag5())),
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
fn a_placement_that_does_not_close_is_a_class_carrying_it() {
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
    let coeff = c.cell(&key).expect("the triangle is held");
    assert!(
        coeff.coefficient.is_identity(),
        "every rotor is the identity, so the rotor half closes exactly"
    );
    assert!(
        !same_versor(&coeff.carriage, &ident()),
        "and the placement half does not: it is the whole obstruction"
    );
    match c.fill(&key) {
        Fill::Class(kappa, r) => {
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
fn congruent_is_read_by_class_key() {
    let mut c = Complex::new();
    let q = quarter();
    let ck = q.class_key().expect("quarter class");
    c.file_congruent(v(1), v(2), q.clone(), 4);
    let got = c.congruent(ck.as_bytes()).expect("Congruent cell by class key");
    assert_eq!(got.1, 4);
    assert!(got.0.projective_eq(&q), "the index holds the rotor that was filed");
    // A class key that was NEVER filed is absent. The claim was
    // `Rotor::identity().class_key().is_none_or(..)`, which passes on the
    // `None` arm without reading the index at all — and the identity may
    // have no class key, so it usually did. `pythag5` has one and was
    // never filed, so the index must answer for it.
    let other = pythag5().class_key().expect("the (3,4,5) rotor has a class key");
    assert!(
        c.congruent(other.as_bytes()).is_none(),
        "a class never filed is not in the index"
    );
}

/// A CELL ON NO VERTICES IS NO SIMPLEX. It was remembered at degree zero,
/// where a point lives, so the complex held a cell with no support beside
/// its vertices and `degree` answered for it by flooring a subtraction.
/// It is not remembered, and every reader of `degree` takes `n − 1` whole.
#[test]
fn a_cell_on_no_vertices_is_not_a_cell() {
    let mut c = Complex::new();
    c.remember(Cell {
        vertices: Vec::new(),
        coefficient: Rotor::identity(),
        carriage: ident(),
        volume: vol(),
        fill: Fill::Open,
    });
    assert_eq!(c.cell_count(), 0, "no simplex was remembered");
    assert!(c.get(&[]).is_none(), "and none is there to read");
}

#[test]
fn fill_is_attempted_once() {
    let mut c = Complex::new();
    let (a, b) = (v(1), v(2));
    c.set_section(&a, Section::at(a.clone(), Rotor::identity()), vol());
    c.set_section(&b, Section::at(b.clone(), Rotor::identity()), vol());
    c.set_edge(&a, &b, Rotor::identity(), ident(), vol());
    let key = c.link(&a, 1)[0].clone();
    let first = c.fill(&key);
    assert!(matches!(first, Fill::Filled(..)));
    let second = c.fill(&key);
    assert!(matches!(second, Fill::Already), "a second fill is not a transition");
    assert!(matches!(c.cell(&key).map(|c| &c.fill), Some(Fill::Filled(..))));
}
