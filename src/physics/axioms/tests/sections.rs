// 78.74 (draft, sections) — TWO GEOIDS OF ONE CLASS, RELATED THREE WAYS. A
// section of a class is one carriage of it: a body carried from its input
// pivot P to its output pivot Q, a fibre whose switch S = m_Q P − m_P Q is the
// white-hole construction's geoid (source at P, sink at Q). Two sections
// give two geoids, and they relate by:
//
// - CONGRUENCE — they overlay: the same weighed length and the same masses,
//   `S₁² = S₂²` (`S² = m_P m_Q d²`, read as `−2 m_P m_Q P·Q`, never a sum of
//   squares) — the second section is the first displaced: a copy.
// - COHERENCE — their carriages are one motion up to torsion about the
//   fibre joining their sources: the relative motion keeps both sources and
//   every level.
// - COHESION — entangled: the fascicle of the two geoids, `S₂ S₁`, binds them
//   when the geoids cut at an angle of finite order, so the bound pair
//   returns; apart (a boost) or at an angle of no finite order (a rosette),
//   nothing binds.

use crate::network::nerve::coord::BivectorKind;
use crate::physics::capacitor::{Carried, Fibre};
use crate::physics::fascicle::{Fascicle, Streamline};
use crate::transducer::cga::{self, Mv, E1, E2};
use num_rational::BigRational;

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(n.into(), d.into())
}

fn at(x: i64, y: i64) -> Mv {
    cga::point_at(&[q(x, 1), q(y, 1)])
}

/// A section's geoid: the switch of the fibre from where it was to where it
/// landed, weighed by the compartments at each end.
fn geoid(from: Mv, to: Mv, m_in: i64, m_out: i64) -> Fibre {
    Fibre::new(from, to, q(m_in, 1), q(m_out, 1)).expect("a section moves")
}

/// The bound pair's period, if the geoids' fascicle turns by an angle of
/// finite order.
fn binds(s1: &Mv, s2: &Mv) -> Option<u32> {
    let f = Fascicle { first: s1.clone(), second: s2.clone() };
    if f.streamline() != Some(Streamline::Elliptic) {
        return None;
    }
    let modes = match cga::motion_modes(&f.versor(), &mut crate::physics::Physics::default().stroke_n(1 << 16)) {
        Some(Ok(m)) => m,
        _ => return None,
    };
    let turns: Vec<_> = modes.iter().filter(|m| m.kind == BivectorKind::Elliptic).collect();
    match turns.as_slice() {
        [one] => one.order,
        _ => None,
    }
}

/// CONGRUENCE: two copies of one section, displaced, overlay — equal
/// weighed lengths and masses; a section moved further, or carrying another
/// mass, does not.
#[test]
fn two_sections_overlay_where_their_geoids_are_congruent() {
    let a = geoid(at(0, 0), at(3, 4), 2, 2);
    let copy = geoid(at(10, 1), at(13, 5), 2, 2);
    let further = geoid(at(10, 1), at(16, 9), 2, 2);
    let heavier = geoid(at(10, 1), at(13, 5), 3, 3);
    let s2 = |f: &Fibre| f.switch().geo(&f.switch()).scalar_part();
    assert_eq!(s2(&a), s2(&copy), "a copy: S² = m m' d² alike — congruent");
    assert_ne!(s2(&a), s2(&further), "moved further: not congruent");
    assert_ne!(s2(&a), s2(&heavier), "heavier: not congruent");
    assert_eq!(s2(&a), q(4 * 25, 1), "S² = m_P m_Q d², exact: 2·2·25");
}

/// COHERENCE: a section and its mirror across the line joining their
/// sources are one motion up to torsion — the relative motion keeps both
/// sources and every level of their fibre; a section slid instead is not.
#[test]
fn two_sections_cohere_where_their_carriages_differ_by_torsion() {
    let (p1, p2) = (at(0, 0), at(3, 4));
    let joining = Fibre::new(p1, p2, q(2, 1), q(2, 1)).expect("two sources");
    let v1 = Mv::scalar(q(1, 1));
    let mirror = Mv::blade(E1, q(4, 1)).sub(&Mv::blade(E2, q(3, 1)));
    let slid = cga::translator_at(&[q(1, 1), q(0, 1)]);
    assert_eq!(joining.carried_by(&v1.reverse().geo(&mirror)), Carried::Torsion, "coherent: the flip about their line");
    assert_eq!(joining.carried_by(&v1.reverse().geo(&slid)), Carried::Lift, "not coherent: the slide moves a source");
}

/// COHESION: two geoids that cut at 90° entangle into a 180° turn about
/// their common round — period 2, bound; geoids that cut at another angle of
/// finite order bind at its period; two that stand apart breathe (a boost)
/// and never bind.
#[test]
fn two_geoids_entangle_into_cohesion_where_they_cut_at_a_finite_angle() {
    // Equal masses: each geoid is its fibre's bisecting plane. The planes
    // x = 1 and y = 1 cut at 90°.
    let vertical = geoid(at(0, 0), at(2, 0), 1, 1).switch();
    let horizontal = geoid(at(0, 0), at(0, 2), 1, 1).switch();
    assert_eq!(binds(&vertical, &horizontal), Some(2), "90°: entangled into a 180° turn, period 2");
    // The planes x = 1 and x = y cut at 45°: a 90° turn, period 4.
    let diagonal = geoid(at(0, 0), at(1, -1), 1, 1).switch();
    let bound = binds(&geoid(at(0, 0), at(2, 0), 1, 1).switch(), &diagonal);
    assert!(matches!(bound, Some(4)), "45°: a 90° turn, period 4: {bound:?}");
    // Two concentric geoids stand apart: their fascicle breathes, unbound.
    let inner = geoid(at(0, 0), at(1, 0), 1, 3).switch();
    let outer = geoid(at(0, 0), at(1, 0), 1, 5).switch();
    let apart = Fascicle { first: inner.clone(), second: outer.clone() }.streamline();
    assert_ne!(apart, Some(Streamline::Elliptic), "apart: no cut");
    assert_eq!(binds(&inner, &outer), None, "nothing binds");
}
