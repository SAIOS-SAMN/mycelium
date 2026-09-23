// 72.1c, FR1 — The relative pose: read on the tube over every plane the two
// bodies occupy, and unchanged on the sensor's plane at one section.

use crate::transducer::base::GRID_PLANE;
use crate::transducer::blade::*;
use crate::transducer::cga::{self, E1, E2, E3};

type Cells = [(i64, i64, i64)];

fn body(cells: &Cells) -> Blade {
    shape_of(&super::common::sites(cells)).blade
}

/// `V e_i Ṽ` as the multiple of one base axis it must be: every other
/// coefficient zero, this one strictly positive.
fn sends(v: &cga::Mv, from: usize, to: usize) -> bool {
    let q = |n: i64| num_rational::BigRational::from_integer(n.into());
    let image = v.sandwich(&cga::Mv::blade(from, q(1)));
    let others = (0..image.slot_count()).filter(|&m| m != to).all(|m| *image.coeff(m) == q(0));
    others && *image.coeff(to) > q(0)
}

const L4: [(i64, i64, i64); 4] = [(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, 1, 0)];

#[test]
fn at_one_section_the_pose_is_the_sensor_planes_reading() {
    let l = body(&L4);
    let turned = body(&[(0, 0, 0), (0, 1, 0), (0, 2, 0), (-1, 2, 0)]);
    let j = body(&[(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, -1, 0)]);
    let s = body(&[(0, 0, 0), (1, 0, 0), (1, 1, 0), (2, 1, 0)]);
    for other in [&turned, &j, &s, &l] {
        assert!(!on_tube(&l, other), "one section, both");
        let (pose, planar) = (relative_pose(&l, other), relative_rotor(&l, other, GRID_PLANE));
        assert_eq!(pose.as_ref().err(), planar.as_ref().err(), "the same refusal");
        if let (Ok(p), Ok(r)) = (&pose, &planar) {
            assert!(p.rotor().projective_eq(r.rotor()), "the same turn");
        }
    }
}

#[test]
fn two_flat_bodies_at_two_sections_turn_in_the_sensors_plane() {
    // A body carried from one section to the next: each is flat, and the
    // turn between them is the sensor plane's; the displacement along `e₃`
    // is the carriage's (A13), never the pose's.
    let a = body(&L4);
    let b = body(&[(0, 0, 1), (1, 0, 1), (2, 0, 1), (2, 1, 1)]);
    assert!(!on_tube(&a, &b));
    assert!(matches!(relative_pose(&a, &b), Ok(Pose::Turn(_)) | Ok(Pose::HalfOpen(_))));
}

#[test]
fn a_body_turning_between_two_sections_reads_a_pose_with_a_plane_containing_e3() {
    // L4 at one section, and L4 turned a quarter in the plane of the row
    // and the succession axis, `(x₁, x₃) ↦ (−x₃, x₁)`: it now stands along
    // three sections. No turn of the sensor's plane relates them.
    let a = body(&L4);
    let b = body(&[(0, 0, 0), (0, 0, 1), (0, 0, 2), (0, 1, 2)]);
    assert!(on_tube(&a, &b), "the turned body spans sections");
    let Ok(Pose::Turn(v)) = relative_pose(&a, &b) else {
        panic!("a turn: {:?}", relative_pose(&a, &b));
    };
    assert!(*v.coeff(E1 | E3) != num_rational::BigRational::from_integer(0.into()), "the plane e13 is lit: {v:?}");
    assert!(sends(&v, E1, E3), "the row goes to the succession axis: {v:?}");
    assert!(sends(&v, E2, E2), "the column is fixed: {v:?}");
}

/// A body with extent on all three axes and NO symmetry: its columns span
/// the base (checked exactly when the fixture was chosen; a body with a
/// hidden half turn — `L4` with one cell above its corner is one, by
/// `(x₁, x₂, x₃) ↦ (2 − x₁, x₃, x₂)` — leaves its pose unread, as it should).
const SOLID: [(i64, i64, i64); 6] = [(0, 0, 0), (1, 0, 0), (2, 0, 0), (2, 1, 0), (0, 0, 1), (0, 0, 2)];

fn mapped(f: impl Fn((i64, i64, i64)) -> (i64, i64, i64)) -> Blade {
    body(&SOLID.map(f))
}

#[test]
fn a_solid_tube_reads_its_turn_its_mirror_and_its_irrational_scale() {
    let a = body(&SOLID);
    // Turned a quarter in `e₂₃`: `(x₂, x₃) ↦ (−x₃, x₂)`.
    let turned = mapped(|(r, c, k)| (r, -k, c));
    let Ok(Pose::Turn(v)) = relative_pose(&a, &turned) else {
        panic!("a turn in e23: {:?}", relative_pose(&a, &turned));
    };
    assert!(sends(&v, E2, E3) && sends(&v, E1, E1), "the turn is in e23: {v:?}");
    // Mirrored across the row: no proper motion carries it.
    let mirrored = mapped(|(r, c, k)| (-r, c, k));
    assert_eq!(relative_pose(&a, &mirrored).err(), Some(NoPose::Mirrored));
    // Doubled as points: the scale is 2 and the turn the identity.
    let doubled = mapped(|(r, c, k)| (2 * r, 2 * c, 2 * k));
    let Ok(Pose::Turn(id)) = relative_pose(&a, &doubled) else {
        panic!("a pure dilation: {:?}", relative_pose(&a, &doubled));
    };
    assert!(id.projective_eq(&cga::Mv::scalar(num_rational::BigRational::from_integer(1.into()))));
    // Turned an eighth in `e₁₃` at scale √2, `(x₁, x₃) ↦ (x₁ − x₃, x₁ + x₃)`:
    // `ρ² = 2`, and no versor over ℚ carries it.
    let oblique = mapped(|(r, c, k)| (r - k, c, r + k));
    assert_eq!(relative_pose(&a, &oblique).err(), Some(NoPose::Irrational));
    // And a (3, 4, 5) turn in `e₁₃`, which the square base cannot turn by:
    // on the tube it is read, exactly, as the rational turn it is.
    let pythagorean = mapped(|(r, c, k)| (3 * r - 4 * k, 5 * c, 4 * r + 3 * k));
    assert!(matches!(relative_pose(&a, &pythagorean), Ok(Pose::Turn(_))), "{:?}", relative_pose(&a, &pythagorean));
}
