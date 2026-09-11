// 13 — the Čech complex: transitions cancel, consilient charts glue, an
// equivocation shows up as curvature, and a fill is a gauge or a class.

use super::common::*;
use crate::transducer::alphabet::{Order, Rotor};
use crate::transducer::cech::*;
use crate::transducer::element::{identity_versor as ident, same_versor};

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
