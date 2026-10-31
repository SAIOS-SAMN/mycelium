// MU-RECURSION §0.0 — A TEST PER FOUNDING PRINCIPLE: the harmonic return, the
// luminal null flux, the holonomic content, every dimension an exponent, and
// the aperture read off the base it stands for.

use crate::network::nerve::coord::BivectorKind;
use crate::transducer::cga::{self, Mv, E1, E2, E3, EM, EP};
use num_rational::BigRational;

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(n.into(), d.into())
}

fn stroke() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 16)
}

fn modes(m: &Mv) -> Vec<cga::MotionMode> {
    match cga::motion_modes(m, &mut stroke()) {
        Some(Ok(ms)) => ms,
        other => panic!("the modes read: {other:?}"),
    }
}

/// HARMONIC — A TURN RETURNS AT ITS READ ORDER, OR NEVER. The 90° turn of
/// the grid plane is one elliptic mode of order four, and its fourth power is
/// the identity ray. The (3, 4, 5) turn is elliptic with no order: no power
/// up to twice Niven's largest order returns it, and its winding is content.
#[test]
fn motion_modes_returns_turn_at_read_order_or_never() {
    let ninety = Mv::scalar(q(1, 1)).add(&Mv::blade(E1 | E2, q(1, 1)));
    let [m] = modes(&ninety).try_into().expect("one mode");
    assert_eq!(m.kind, BivectorKind::Elliptic);
    assert_eq!(m.order, Some(4), "its order is read off it");
    let one = Mv::scalar(q(1, 1));
    let fourth = ninety.geo(&ninety).geo(&ninety).geo(&ninety);
    assert!(fourth.projective_eq(&one), "the fourth power returns");
    let rosette = Mv::scalar(q(3, 1)).add(&Mv::blade(E1 | E2, q(4, 1)));
    let [r] = modes(&rosette).try_into().expect("one mode");
    assert_eq!(r.kind, BivectorKind::Elliptic);
    assert_eq!(r.order, None, "no order: it never returns");
    let mut power = rosette.clone();
    for k in 1..12 {
        assert!(!power.projective_eq(&one), "power {k} has not returned");
        power = power.geo(&rosette);
    }
}

/// LUMINAL — A NULL GENERATOR HAS NO LENGTH, AND STAYS NULL WHEREVER THE FLOW
/// CARRIES IT. A translation's holonomy carries one parabolic flux whose
/// generator squares to zero; carried by a turn, it squares to zero still.
#[test]
fn null_fluxes_keeps_generator_null_under_flow() {
    let slide = cga::translator_at(&[q(1, 1), q(0, 1), q(0, 1)]);
    let flux = crate::transducer::cech::flux::null_fluxes(&slide, &mut stroke()).expect("the modes read");
    let [b] = flux.as_slice() else { panic!("one null flux: {flux:?}") };
    assert!(b.geo(b).is_zero(), "B² = 0: no length");
    let turn = Mv::scalar(q(1, 1)).add(&Mv::blade(E1 | E2, q(1, 1)));
    let carried = turn.sandwich(b);
    assert!(!carried.is_zero() && carried.geo(&carried).is_zero(), "carried by the flow, still null");
}

/// HOLONOMIC — A LOOP'S COMPOSITE IS ITS CONTENT. A loop composing to a turn
/// about one point and a slide, read again from another base point, is the
/// same holonomy conjugated: its modes — the modes' kinds and invariants —
/// are the loop's, never the frame's.
#[test]
fn motion_modes_ignores_base_of_holonomy() {
    let turn = Mv::scalar(q(1, 1)).add(&Mv::blade(E1 | E3, q(1, 1)));
    let holonomy = cga::translator_at(&[q(0, 1), q(2, 1), q(0, 1)]).geo(&turn);
    let base = cga::translator_at(&[q(3, 1), q(-1, 1), q(0, 1)]);
    let rebased = base.sandwich(&holonomy);
    // The modes come in the modes' own canonical order (parabolic first, then by
    // `c`), so the two readings compare as read.
    let read = |m: &Mv| modes(m).into_iter().map(|x| (x.kind, x.c)).collect::<Vec<(BivectorKind, BigRational)>>();
    assert_eq!(read(&holonomy), read(&rebased), "the class is the loop's");
}

/// EVERY DIMENSION IS AN EXPONENT. The product of `k` orthogonal generators
/// is a blade of grade `k`, for every grade the algebra holds; and the
/// sensor base's unit cell has `2ⁿ` vertices for its `n` axes — the aperture.
#[test]
fn geo_carries_k_generators_to_grade_k_blade() {
    let generators = [E1, E2, E3, EP, EM];
    let mut product = Mv::scalar(q(1, 1));
    let mut mask = 0usize;
    for (k, g) in generators.iter().enumerate() {
        product = product.geo(&Mv::blade(*g, q(1, 1)));
        mask |= g;
        let grade = u32::try_from(k + 1).expect("five generators");
        assert!(product.projective_eq(&Mv::blade(mask, q(1, 1))), "the product of {} is one blade", k + 1);
        assert!(product.projective_eq(&product.grade(grade)), "of grade {}", k + 1);
    }
    let axes = crate::transducer::base::FRAME_AXIS + 1;
    let vertices = (0..axes).fold(1u64, |cell, _| cell * 2);
    assert_eq!(crate::physics::STROKES_PER_RESERVE, vertices, "the aperture is the base's unit cell");
    assert_eq!(u64::from(crate::physics::aperture_cycle()), vertices);
}

/// NO CONSTANT: every number the tree still declares where a reading belongs
/// is named, with what derives it.
#[test]
fn open_physics_names_value_and_derivation() {
    for (number, derivation) in crate::physics::axioms::OPEN_PHYSICS {
        assert!(number.contains('='), "{number} names its value");
        assert!(!derivation.is_empty(), "{number} names what derives it");
    }
}
