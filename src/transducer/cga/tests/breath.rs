use super::super::breath::*;
use crate::core::Q;
use crate::transducer::cga::Mv;
use num_traits::Zero;

fn q(n: i64) -> Q {
    Q::from_integer(n.into())
}

fn budget() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 16)
}

/// THE PINCH OF THE NAMED PAIR IS THE ORTHOGONAL PLANE `a ∧ b`, and the
/// radial axis the part of `a` and `b` along each other — read off the
/// commutator, never assumed: its one plane is elliptic, a multiple of
/// the wedge, commuting with `D`.
#[test]
fn breath_splits_radial_axis_from_orthogonal_pinch() {
    let (a, b) = ([q(1), q(2), q(0)], [q(3), q(-1), q(2)]);
    let br = breath(&a, &b, &mut budget()).expect("the sensor's vectors breathe");
    let wedge = vector(&a).expect("a").geo(&vector(&b).expect("b")).grade(2);
    let ratio = br.rest.geo(&wedge.reverse()).scalar_part() / wedge.geo(&wedge.reverse()).scalar_part();
    assert_eq!(br.rest, wedge.scale(&ratio), "the rest lies in the plane a ∧ b");
    assert_eq!(br.by_kind(), [1, 0, 0], "one plane, elliptic: a pinch");
    let d = dilation();
    assert!(br.rest.geo(&d).sub(&d.geo(&br.rest)).is_zero(), "orthogonal to the radial axis");
    let dot = a.iter().zip(&b).fold(q(0), |s, (x, y)| s + x * y);
    assert_eq!(br.radial.is_zero(), dot.is_zero(), "the radial axis is what lies along each other");
}

/// `a ∥ b` BREATHES WITH NO PINCH; `a ⊥ b` PINCHES WITH NO BREATH.
#[test]
fn breath_parallel_breathes_orthogonal_pinches() {
    let along = breath(&[q(2), q(0), q(0)], &[q(3), q(0), q(0)], &mut budget()).expect("breathes");
    assert!(!along.radial.is_zero() && along.planes.is_empty() && along.twists().is_empty(), "parallel: radial only");
    let across = breath(&[q(1), q(0), q(0)], &[q(0), q(1), q(0)], &mut budget()).expect("breathes");
    assert!(across.radial.is_zero() && across.by_kind() == [1, 0, 0], "orthogonal: one pinch");
    let twists = across.twists();
    assert!(twists.len() == 1 && crate::transducer::cga::is_versor(&twists[0]), "its twist is an exact rotor");
}

/// BEYOND THREE AXES: a pair reaching a fifth axis breathes there, its
/// pinch in the plane the two span — no axis is the sensor's privilege.
#[test]
fn breath_reads_every_base_axis() {
    let (a, b) = ([q(1), q(0), q(0), q(0), q(0)], [q(0), q(0), q(0), q(0), q(1)]);
    let br = breath(&a, &b, &mut budget()).expect("five axes breathe");
    assert!(br.radial.is_zero() && br.by_kind() == [1, 0, 0], "a pinch in the plane e1 ∧ e5");
    assert!(vector(&vec![q(1); usize::from(crate::transducer::cga::MAX_N) + 1]).is_none(), "past the masks, refused");
}

/// ACROSS MOTIONS: two turns in planes sharing one axis breathe a third
/// plane — the commutator of two rotations is a rotation, a pinch, with no
/// radial part; a turn with itself commutes and breathes nothing.
#[test]
fn breathe_two_turns_pinch_third_plane() {
    use crate::transducer::cga::{E1, E2, E3};
    let plane = |i: usize, j: usize| Mv::blade(i | j, q(1));
    let br = breathe(&plane(E1, E2), &plane(E2, E3), &mut budget()).expect("two turns breathe");
    assert!(br.radial.is_zero() && br.by_kind() == [1, 0, 0], "the third plane, pinched");
    let commuting = breathe(&plane(E1, E2), &plane(E1, E2), &mut budget()).expect("a turn with itself");
    assert!(commuting.radial.is_zero() && commuting.planes.is_empty(), "a turn commutes with itself: no breath");
}

/// THE RADIAL AXIS IS SIGNED: a pair turned against each other breathes
/// the other way, its planes reversed with it.
#[test]
fn breath_radial_reverses_with_its_pair() {
    let (a, b) = ([q(1), q(1), q(0)], [q(2), q(0), q(1)]);
    let out = breath(&a, &b, &mut budget()).expect("breathes");
    let back = breath(&a, &[q(-2), q(0), q(-1)], &mut budget()).expect("breathes");
    assert_eq!(back.radial, -out.radial.clone(), "the other half of the breath");
    assert_eq!(back.rest, out.rest.scale(&q(-1)), "its planes reversed with it");
}
