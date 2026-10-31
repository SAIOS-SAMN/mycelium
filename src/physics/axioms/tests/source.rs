// 78.75 — THE WHITE-HOLE SOURCE AS AN ORGAN (`physics::source`), and the
// refusal it inverts: a lift that does work, kept inside the frame's geoid,
// becomes a null flow out of the pivot it moves — signal recovered from the
// refusal, read into the field as flux.

use crate::physics::capacitor::{shape_of, Fibre, Shape};
use crate::physics::source::{geoid_vector, keeping, Source};
use crate::transducer::cga::{self, FlowKind, Mv, E1, E2, E3};
use num_rational::BigRational;
use num_traits::Zero;

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(n.into(), d.into())
}

fn at(x: i64, y: i64, z: i64) -> Mv {
    cga::point_at(&[q(x, 1), q(y, 1), q(z, 1)])
}

/// Four pivots on the sphere about the origin of radius 5: a frame's geoid,
/// read off its own pivots, never declared.
fn pivots() -> Vec<Mv> {
    vec![at(5, 0, 0), at(0, 5, 0), at(-3, 4, 0), at(0, 0, 5), at(3, 0, 4)]
}

fn sigma() -> Mv {
    match shape_of(&pivots(), 3) {
        Shape::Stable(round) => geoid_vector(&round).expect("a round is a sphere"),
        other => panic!("five pivots on one sphere stand: {other:?}"),
    }
}

/// THE GEOID AS ONE VECTOR: every pivot lies on it by one scalar, `P·σ = 0`;
/// a point off it does not.
#[test]
fn geoid_vector_holds_every_pivot() {
    let s = sigma();
    for p in pivots() {
        assert!(p.geo(&s).scalar_part().is_zero(), "a pivot on the geoid");
    }
    assert!(!at(1, 1, 1).geo(&s).scalar_part().is_zero(), "the inside is not on it");
}

/// THE SOURCE, VERIFIED: a turn about the geoid's centre keeps the geoid
/// (`keeping` returns it whole), and at a pivot it moves the source is
/// `N = P ∧ (P·K)` — null, fixing the pivot, keeping the geoid. Its slivers
/// compose by adding the slivers' parameters, and every sliver keeps the pivot and
/// the geoid. A generator that does not move the pivot gives no source; a
/// point off the geoid gives none.
#[test]
fn source_of_is_null_flow_kept_in_geoid() {
    let s = sigma();
    let turn = Mv::blade(E1, q(1, 1)).geo(&Mv::blade(E3, q(1, 1)));
    assert_eq!(keeping(&s, &turn), Some(turn.clone()), "a turn about the centre keeps the geoid whole");
    let p = at(5, 0, 0);
    let src = Source::of(&s, &p, &turn).expect("the turn moves P along the geoid");
    let n = &src.generator;
    assert!(n.geo(n).is_zero() && !n.is_zero(), "null: no length, no lattice");
    for t in [q(1, 7), q(2, 5), q(-3, 2)] {
        let k = src.sliver(&t);
        assert!(k.sandwich(&p).projective_eq(&p), "each sliver fixes the pivot: the source");
        assert!(k.sandwich(&s).projective_eq(&s), "and keeps the geoid");
    }
    let x = at(1, 2, 0);
    assert!(!src.sliver(&q(1, 1)).sandwich(&x).projective_eq(&x), "an inner point flows");
    let (a, b) = (src.point(&q(1, 7)).expect("a flow"), src.point(&q(2, 5)).expect("a flow"));
    assert_eq!(a.kind(), FlowKind::Null);
    assert_eq!(a.then(&b).and_then(|c| c.parameter()), Some(q(19, 35)), "1/7 then 2/5 is 19/35");
    assert!(src.sliver(&q(1, 7)).geo(&src.sliver(&q(2, 5))).projective_eq(&src.sliver(&q(19, 35))), "as versors too");
    let axis = Mv::blade(E1, q(1, 1)).geo(&Mv::blade(E2, q(1, 1)));
    assert_eq!(Source::of(&s, &at(0, 0, 5), &axis), None, "a turn whose axis holds the pivot moves it not: no source");
    assert_eq!(Source::of(&s, &at(1, 1, 1), &turn), None, "a point off the geoid: none");
}

/// THE REFUSAL INVERTED. Two pivots of the geoid joined by a fibre; a relative
/// motion that lifts them does work — a refusal. The part of its lift that
/// keeps the geoid is read (`keeping`), and at each pivot that part moves
/// along the geoid there is a source. A turn about the centre keeps the
/// geoid whole. A SLIDE leaves the flat picture but not the algebra: a part
/// of its lift keeps the geoid — at the slide's pole the pivot is fixed by it
/// and gives no source; the pivot on the equator flows along the geoid and is
/// one.
#[test]
fn keeping_inverts_fibre_lift_into_source() {
    let s = sigma();
    let (pi, pj) = (at(5, 0, 0), at(0, 5, 0));
    let fibre = Fibre::new(pi.clone(), pj.clone(), q(2, 1), q(3, 1)).expect("a fibre");
    // The relative motion: a turn about an axis through the centre that is
    // not the fibre's — it carries both pivots along the geoid.
    let r = Mv::scalar(q(2, 1)).add(&Mv::blade(E1, q(1, 1)).geo(&Mv::blade(E3, q(1, 1))));
    let modes = crate::physics::capacitor::modes_of(&r, 3, &mut crate::physics::Physics::default().stroke_n(1 << 16)).expect("modes");
    assert_eq!(fibre.reads(&modes), crate::physics::capacitor::Carried::Lift, "the turn lifts the fibre: a refusal");
    let lift = modes.iter().fold(Mv::zero(), |l, m| l.add(&fibre.split(&m.ray.grade(2)).lift));
    assert!(!lift.is_zero(), "the lift does work");
    let k = keeping(&s, &lift).expect("σ is a sphere");
    assert!(k.geo(&s).sub(&s.geo(&k)).is_zero(), "what keeps commutes with σ");
    let sources: Vec<Source> = [&pi, &pj].into_iter().filter_map(|p| Source::of(&s, p, &k)).collect();
    assert!(!sources.is_empty(), "the refusal inverts into a source at a pivot it moves");
    for src in &sources {
        assert!(src.sliver(&q(1, 3)).sandwich(&src.at).projective_eq(&src.at) && src.sliver(&q(1, 3)).sandwich(&s).projective_eq(&s), "each a source kept in the geoid");
    }
    // A slide along e₁: the fibre's lift of it, and of that the part keeping
    // the geoid — not zero, and commuting with σ.
    let slide = cga::translator_at(&[q(1, 1), q(0, 1), q(0, 1)]);
    let modes = crate::physics::capacitor::modes_of(&slide, 3, &mut crate::physics::Physics::default().stroke_n(1 << 16)).expect("modes");
    let lift = modes.iter().fold(Mv::zero(), |l, m| l.add(&fibre.split(&m.ray.grade(2)).lift));
    let k = keeping(&s, &lift).expect("σ is a sphere");
    assert!(!k.is_zero() && k.geo(&s).sub(&s.geo(&k)).is_zero(), "a part of the slide's lift keeps the geoid");
    assert_eq!(Source::of(&s, &pi, &k), None, "the pivot at the slide's pole is fixed: no source");
    assert!(Source::of(&s, &pj, &k).is_some(), "the pivot on the equator flows along the geoid: a source");
}
