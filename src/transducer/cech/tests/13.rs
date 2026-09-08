// 13 — the Čech complex: transitions cancel, consilient charts glue, an
// equivocation shows up as curvature, and a fill is a gauge or a class.

use super::common::*;
use crate::transducer::alphabet::Rotor;
use crate::transducer::cech::*;

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
        c.set_section(x, Section { addr: (*x).clone(), gamma: Rotor::identity() }, vol());
    }
    let id = Rotor::identity();
    c.set_edge(&a, &b, id.clone(), vol());
    c.set_edge(&a, &cc, id.clone(), vol());
    c.set_edge(&b, &cc, id.clone(), vol());
    c.set_edge(&cc, &d, id, vol());
    c.coboundary_on_link(&a, |_| vol());
    for k in c.link(&a, 2) {
        let cell = c.cell(&k).expect("link key is held");
        assert!(cell.contains(&a), "link(a) contains only a");
    }
    assert!(c.degrees.get(3).is_none_or(|l| l.is_empty()), "no tetra on a's link");
    let mut empty = Complex::new();
    empty.set_section(&a, Section { addr: a.clone(), gamma: Rotor::identity() }, [0; 4]);
    empty.set_edge(&a, &b, Rotor::identity(), [0; 4]);
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
        c.set_section(x, Section { addr: (*x).clone(), gamma: Rotor::identity() }, vol());
    }
    let q = quarter();
    c.set_edge(&a, &b, q.clone(), vol());
    c.set_edge(&b, &cc, q.clone(), vol());
    c.set_edge(&a, &cc, q.geo(&q), vol());
    c.coboundary_on_link(&a, |_| vol());
    let tri = c.link(&a, 2);
    assert_eq!(tri.len(), 1);
    let key = tri[0].clone();
    assert!(matches!(c.fill(&key), Fill::Filled(_)));
    c.set_edge(&a, &cc, Rotor::identity(), vol());
    c.coboundary_on_link(&a, |_| vol());
    let key = c.link(&a, 2)[0].clone();
    assert!(matches!(c.fill(&key), Fill::Congruent(_, _)));
    c.set_edge(&a, &b, pythag5(), vol());
    c.set_edge(&b, &cc, Rotor::identity(), vol());
    c.set_edge(&a, &cc, Rotor::identity(), vol());
    c.coboundary_on_link(&a, |_| vol());
    let key = c.link(&a, 2)[0].clone();
    assert!(matches!(c.fill(&key), Fill::Class(_)));
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
    assert!(Rotor::identity().class_key().is_none_or(|k| c.congruent(k.as_bytes()).is_none()));
}

#[test]
fn fill_is_attempted_once() {
    let mut c = Complex::new();
    let (a, b) = (v(1), v(2));
    c.set_section(&a, Section { addr: a.clone(), gamma: Rotor::identity() }, vol());
    c.set_section(&b, Section { addr: b.clone(), gamma: Rotor::identity() }, vol());
    c.set_edge(&a, &b, Rotor::identity(), vol());
    let key = c.link(&a, 1)[0].clone();
    let first = c.fill(&key);
    assert!(matches!(first, Fill::Filled(_)));
    let second = c.fill(&key);
    assert!(matches!(second, Fill::Already), "a second fill is not a transition");
    assert!(matches!(c.cell(&key).map(|c| &c.fill), Some(Fill::Filled(_))));
}
