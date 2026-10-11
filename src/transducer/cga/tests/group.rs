// A4, 51.2 — the rational conformal group acts exactly, and the frame axis
// is a base coordinate.

use super::common::q;
use crate::transducer::cga::at_three::{base, point, sphere, translator};
use crate::transducer::cga::*;
use num_traits::Zero;

/// The dilator scales points exactly with no root or logarithm; the
/// reflector negates rows; both are versors; projective equality reads
/// the scalar ratio and refuses non-proportional blades; bytes round-trip.
#[test]
fn rational_versors_act_exactly_and_are_verified_in_closed_form() {
    let d = dilator(&q(3, 2));
    assert!(is_versor(&d), "the dilator is a versor");
    let moved = d.sandwich(&point(&q(2, 1), &q(-4, 1), &q(6, 1)));
    assert_eq!(base(&moved), Some((q(3, 1), q(-6, 1), q(9, 1))), "X(2,−4,6) → X(3,−6,9) at ρ = 3/2");
    let r = reflector();
    let flipped = r.sandwich(&point(&q(2, 1), &q(5, 1), &q(7, 1)));
    assert_eq!(base(&flipped), Some((q(-2, 1), q(5, 1), q(7, 1))));
    let s = sphere(&q(1, 1), &q(1, 1), &q(0, 1), &q(5, 1));
    assert_eq!(projective_ratio(&s.scale(&q(7, 3)), &s), Some(q(7, 3)));
    assert!(projective_ratio(&s, &sphere(&q(1, 1), &q(1, 1), &q(0, 1), &q(6, 1))).is_none());
    let composed = translator(&q(1, 1), &q(0, 1), &q(2, 1)).geo(&d);
    let mut bytes = Vec::new();
    composed.to_bytes(&mut bytes);
    let mut pos = 0usize;
    assert_eq!(Mv::from_bytes(&bytes, &mut pos), Some(composed));
}

#[test]
fn a4_projective_eq_either_sign_and_key() {
    let t = translator(&q(2, 1), &q(0, 1), &q(0, 1));
    assert!(t.projective_eq(&t.scale(&q(-3, 2))));
    assert_eq!(t.canonical().unwrap().key(), t.scale(&q(-3, 2)).canonical().unwrap().key());
    assert_eq!(t, t.scale(&q(5, 1)), "PartialEq is projective");
    let u = translator(&q(3, 1), &q(0, 1), &q(0, 1));
    assert_ne!(t.key(), u.key());
}

/// 51.2 — THE THIRD COORDINATE IS A BASE COORDINATE: translators along
/// the three axes commute, a rotation in the `e1e3` plane carries the
/// row axis onto the frame axis exactly, and a body in one frame at
/// `k = 0` is carried there and nowhere else by a plane motion.
#[test]
fn the_frame_axis_is_a_base_coordinate() {
    let (z, o) = (q(0, 1), q(1, 1));
    let t1 = translator(&q(2, 1), &z, &z);
    let t3 = translator(&z, &z, &q(5, 1));
    assert!(t1.geo(&t3).projective_eq(&t3.geo(&t1)), "translators commute (51.2)");
    assert_eq!(
        base(&t1.geo(&t3).sandwich(&point(&z, &z, &z))),
        Some((q(2, 1), z.clone(), q(5, 1))),
        "and compose by adding the displacements"
    );
    let turn_90_13 = rotator_on_blade(&o, &o, E1 | E3);
    let moved = base(&turn_90_13.sandwich(&point(&o, &z, &z))).expect("a rotation keeps the point finite");
    assert!(moved.0.is_zero() && moved.1.is_zero() && !moved.2.is_zero(), "the row axis turns onto the frame axis: {moved:?}");
    let plane = rotator(&o, &o).geo(&translator(&q(3, 1), &q(-1, 1), &z));
    let carried = base(&plane.sandwich(&point(&q(4, 1), &q(4, 1), &z))).expect("finite");
    assert!(carried.2.is_zero(), "a plane motion keeps k = 0: {carried:?}");
}

/// VA1 (78.28(1) as amended) — A VERSOR IS KNOWN BY ITS ACTION, and the old
/// condition is a test OF the reader: `m m̃` a non-zero scalar agrees with
/// the action reader on every element here — even and odd versors, a mixed
/// parity, `1 + e₁₂ + e₃₊` (every generator to an independent vector, yet
/// `m m̃` not a scalar: the pseudoscalar clause refuses it), `1 + I` (every
/// generator to zero), a null vector, and zero.
#[test]
fn the_action_reader_agrees_with_the_theorem_it_implies() {
    use crate::transducer::cga::multivector::pseudo;
    let old = |m: &Mv| {
        let p = m.geo(&m.reverse());
        let s = p.scalar_part();
        !s.is_zero() && p.sub(&Mv::scalar(s)).is_zero()
    };
    let one = Mv::scalar(q(1, 1));
    let blade = |mask: usize| Mv::blade(mask, q(1, 1));
    let z = q(0, 1);
    let cases: Vec<(&str, Mv, bool)> = vec![
        ("the scalar", one.clone(), true),
        ("a dilator", dilator(&q(2, 1)), true),
        ("a translator", translator(&q(3, 1), &z, &z), true),
        ("a reflection, odd", reflector(), true),
        ("three reflections, odd", reflector().geo(&blade(E2)).geo(&blade(E3)), true),
        ("mixed parity", one.add(&blade(E1)), false),
        ("1 + e12 + e3+", one.add(&blade(E1 | E2)).add(&blade(E3 | EP)), false),
        ("1 + I", one.add(&blade(pseudo(3).expect("the pseudoscalar at n = 3"))), false),
        ("a null vector", e_inf(), false),
        ("zero", Mv::zero(), false),
    ];
    for (name, m, want) in &cases {
        assert_eq!(is_versor(m), *want, "{name}");
        assert_eq!(is_versor(m), old(m), "{name}: the theorem agrees with the reader");
        assert_eq!(weight_of(m).is_some(), *want, "{name}: a weight is read only for a versor");
    }
}

/// VA1's d1 — AN ODD MOTION'S FIXED SET IS READ ON ITS TWISTED ADJOINT. The
/// reflection across `e₁ = 0` holds its whole mirror — four vectors, the
/// origin among them — at its own weight. Read with the plain sandwich, the
/// mirror sat at `−w` and the reading held only the normal.
#[test]
fn a_reflection_holds_its_mirror() {
    match centre_of(&reflector(), 3) {
        Centre::Fixed { dim, point } => {
            assert_eq!(dim, 4, "e₂, e₃, e₊, e₋: the mirror and the point at infinity");
            assert!(point[0].is_zero(), "the point read is on the mirror: {point:?}");
        }
        other => panic!("a reflection fixes its mirror, got {other:?}"),
    }
}
