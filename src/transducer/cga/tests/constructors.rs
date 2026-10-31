// 78.12(3), CS1 — the constructors' orbit under the generators they build:
// closed at once where the generators are flat, closed at its own size under
// a finite turn, open by the mode that carries a constructor off.

use super::common::{budget, q};
use crate::network::nerve::coord::BivectorKind;
use crate::transducer::cga::at_three::translator;
use crate::transducer::cga::*;

fn axes() -> Vec<Mv> {
    vec![Mv::blade(E1, q(1, 1)), Mv::blade(E2, q(1, 1)), Mv::blade(E3, q(1, 1))]
}

/// A FLAT CELL'S ORBIT CLOSES AT ONCE: under the identity every constructor
/// is its own orbit.
#[test]
fn constructor_orbit_closes_at_once_under_identity() {
    let flat = Mv::scalar(q(1, 1));
    assert_eq!(constructor_orbit(&axes(), &[flat], &mut budget()), ConstructorOrbit::Closed { rays: 3 });
}

/// A THREE-BODY CELL'S CONSTRUCTORS UNDER A 90° TURN: `e₁` and `e₂` are
/// carried onto each other's rays and `e₃` stays — the orbit closes on the
/// three rays it began with.
#[test]
fn constructor_orbit_closes_on_three_rays_under_turn_90() {
    let turn_90 = rotator(&q(1, 1), &q(1, 1));
    assert_eq!(carry(&turn_90, &axes()[0]), Some(Mv::blade(E2, q(1, 1))), "e₁ is carried onto e₂'s ray");
    assert_eq!(constructor_orbit(&axes(), &[turn_90], &mut budget()), ConstructorOrbit::Closed { rays: 3 });
}

/// A SCREW'S ORBIT NEVER CLOSES: its slip is a parabolic mode that carries
/// a constructor off, named with the generator, the constructor and the
/// mode's class.
#[test]
fn constructor_orbit_opens_on_screw_parabolic_mode() {
    let screw = rotator(&q(1, 1), &q(1, 1)).geo(&translator(&q(0, 1), &q(0, 1), &q(1, 1)));
    let orbit = constructor_orbit(&axes(), &[screw], &mut budget());
    assert!(
        matches!(orbit, ConstructorOrbit::Open { generator: 0, kind: BivectorKind::Parabolic, .. }),
        "the slip carries a constructor off: {orbit:?}"
    );
}

/// WHAT IS NOT A CONSTRUCTOR OR A GENERATOR IS NOT READ, AND SAYS WHICH
/// (78.26(4)): a null vector is no reflection, by its place; an element with
/// no inverse is no versor, by its place.
#[test]
fn constructor_orbit_refuses_null_constructor_or_non_versor() {
    use crate::transducer::cga::OrbitUnread;
    let null = Mv::blade(EP, q(1, 1)).add(&Mv::blade(EM, q(1, 1)));
    assert_eq!(
        constructor_orbit(&[null], &[Mv::scalar(q(1, 1))], &mut budget()),
        ConstructorOrbit::Unread(OrbitUnread::NotConstructor { constructor: 0 })
    );
    let cone = Mv::scalar(q(1, 1)).add(&Mv::blade(E1 | EM, q(1, 1)));
    assert_eq!(
        constructor_orbit(&axes(), &[cone], &mut budget()),
        ConstructorOrbit::Unread(OrbitUnread::NotVersor { generator: 0 })
    );
}

fn composed(c: &Constructed) -> Mv {
    c.reflections().iter().fold(Mv::scalar(q(1, 1)), |acc, u| acc.geo(u))
}

/// 78.47 — THE INVERSE: a turn by the arc `4 + i`, whose prime 17 the
/// seventeen letters do not spell, spelled back into two constructors that
/// compose to it (T1) and meet on its axis; their Gram form reads.
#[test]
fn construct_spells_turn_as_two_mirrors_on_axis() {
    let turn = rotator(&q(4, 1), &q(1, 1));
    let c = construct(&turn, &mut budget()).expect("a turn spells");
    assert_eq!(c.shapes.len(), 1, "one mode");
    assert_eq!(c.shapes[0].len(), 2, "a planar turn is two mirrors");
    assert!(composed(&c).projective_eq(&turn), "the constructors compose back to the residue (T1)");
    assert!(c.reflections().iter().all(is_constructor), "each shape is a sphere or a plane");
    assert_eq!(c.gram.rank, 2, "the Gram form of the two shapes reads");
}

/// A turn about a point off the origin, spelled mode by mode, the whole T1,
/// within `n + 2` constructors.
#[test]
fn construct_spells_off_origin_turn_composing_back() {
    let t = translator(&q(2, 1), &q(1, 1), &q(0, 1));
    let turn = t.geo(&rotator(&q(3, 1), &q(4, 1))).geo(&t.reverse());
    let c = construct(&turn, &mut budget()).expect("a turn about a point spells");
    assert!(composed(&c).projective_eq(&turn), "T1");
    assert!(c.reflections().len() <= 5, "at most n + 2 constructors");
}

/// 78.114 — THE RESIDUES THE MINDS CARRY CONSTRUCT: a translation (parabolic,
/// its forced step null at once) and a dilation about a point off the origin
/// (hyperbolic, the light from a star at a point) — 64–317 a voice refused
/// "null step" on the live wire — are each spelled by the forced two-step,
/// certified on their fixed loci, and compose back (T1).
#[test]
fn construct_spells_translation_and_dilation_composing_back() {
    let slide = translator(&q(2, 1), &q(1, 1), &q(0, 1));
    let c = construct(&slide, &mut budget()).expect("a translation spells");
    assert!(composed(&c).projective_eq(&slide), "T1: the slide");
    let t = translator(&q(1, 1), &q(0, 1), &q(3, 1));
    let star = t.geo(&dilator(&q(2, 1))).geo(&t.reverse());
    let c = construct(&star, &mut budget()).expect("a dilation about a point spells");
    assert!(composed(&c).projective_eq(&star), "T1: the dilation about its centre");
}

/// No residue, nothing constructed; no versor, no action to spell.
#[test]
fn construct_refuses_identity_and_non_versor_by_name() {
    assert!(matches!(construct(&Mv::scalar(q(1, 1)), &mut budget()), Err(NotConstructed::NoResidue)));
    let cone = Mv::scalar(q(1, 1)).add(&Mv::blade(E1 | EM, q(1, 1)));
    assert!(matches!(construct(&cone, &mut budget()), Err(NotConstructed::NotVersor)));
}
