// 78.14, 78.15, GR2, CX1, MR1b, DF1, SY1 — a set of constructors read as a
// reflection group; the product per order read as a motion; the dual frame.

use crate::network::nerve::coord::BivectorKind;
use crate::transducer::cech::*;
use crate::transducer::cga::{self, DualFrame, Mv, PairRegime, E1, E2, E3};
use num_rational::BigRational;

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(n.into(), d.into())
}

fn stroke() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 16)
}

fn v(coords: &[(usize, i64)]) -> Mv {
    coords.iter().fold(Mv::zero(), |acc, &(m, c)| acc.add(&Mv::blade(m, q(c, 1))))
}

/// The plane `x_axis = d` as a constructor through `e_∞`.
fn plane(axis: usize, d: i64) -> Mv {
    Mv::blade(axis, q(1, 1)).add(&cga::e_inf().scale(&q(d, 1)))
}

/// A₃'S SIMPLE ROOTS (as D₃: `e₁ − e₂`, `e₂ − e₃`, `e₂ + e₃`) ARE A SYSTEM:
/// balanced, independent, definite — FINITE by the theorem, and the check
/// counts the group's twenty-four elements.
#[test]
fn a3s_simple_roots_read_finite_by_the_theorem() {
    let roots = [v(&[(E1, 1), (E2, -1)]), v(&[(E2, 1), (E3, -1)]), v(&[(E2, 1), (E3, 1)])];
    let r = read_coxeter(&roots, &[], &mut stroke());
    assert_eq!(r.components, vec![(vec![0, 1, 2], ComponentClass::Finite)]);
    assert_eq!(r.closure, Closure::Finite { by_theorem: true, order: Some(24) });
}

/// A₂'S THREE ROOTS ARE NOT A SYSTEM — dependent — so the theorem does not
/// read them and the worklist certifies: finite, of order six.
#[test]
fn a2s_three_roots_are_certified_finite_of_order_six_by_the_check() {
    let (a, b) = (v(&[(E1, 1), (E2, -1)]), v(&[(E2, 1), (E3, -1)]));
    let c = a.add(&b);
    let r = read_coxeter(&[a, b, c], &[], &mut stroke());
    assert_eq!(r.components, vec![(vec![0, 1, 2], ComponentClass::Unrecognized)]);
    assert_eq!(r.closure, Closure::Finite { by_theorem: false, order: Some(6) });
}

/// THE CUBE'S CORNER — three orthogonal mirrors — READS FINITE `ℤ₂³`; ITS
/// SIX FACE PLANES READ AFFINE, one `Ã₁` per axis.
#[test]
fn a_cube_corner_reads_finite_and_its_six_faces_read_affine() {
    let corner = [plane(E1, 0), plane(E2, 0), plane(E3, 0)];
    let r = read_coxeter(&corner, &[], &mut stroke());
    assert!(r.components.iter().all(|(_, k)| *k == ComponentClass::Finite));
    assert_eq!(r.closure, Closure::Finite { by_theorem: true, order: Some(8) });
    let faces = [plane(E1, 0), plane(E1, 1), plane(E2, 0), plane(E2, 1), plane(E3, 0), plane(E3, 1)];
    let r = read_coxeter(&faces, &[], &mut stroke());
    assert_eq!(
        r.components,
        vec![
            (vec![0, 1], ComponentClass::Affine),
            (vec![2, 3], ComponentClass::Affine),
            (vec![4, 5], ComponentClass::Affine)
        ]
    );
    assert!(matches!(r.closure, Closure::Infinite(Refusal::Affine(_))));
}

/// MG1b — A PARALLEL FAMILY READS AFFINE `Ã₁`, its translation `2·gcd` of
/// its spacings, no generator pair chosen: mirrors at 0, 1, 2 step 2; at 0,
/// 1/2, 1 step 1; a mirror written at another scale reads the same.
#[test]
fn a_parallel_family_reads_affine_with_twice_the_gcd_of_its_spacings() {
    let at = |ks: &[(i64, i64)]| -> Vec<Mv> {
        ks.iter().map(|&(n, d)| Mv::blade(E1, q(1, 1)).add(&cga::e_inf().scale(&q(n, d)))).collect()
    };
    let r = read_coxeter(&at(&[(0, 1), (1, 1), (2, 1)]), &[], &mut stroke());
    assert_eq!(r.components, vec![(vec![0, 1, 2], ComponentClass::Affine)]);
    assert_eq!(r.parallel, vec![(vec![0, 1, 2], q(2, 1))]);
    assert!(matches!(r.closure, Closure::Infinite(Refusal::Affine(_))), "affine precedes the parabolic pair");
    let half = read_coxeter(&at(&[(0, 1), (1, 2), (1, 1)]), &[], &mut stroke());
    assert_eq!(half.parallel, vec![(vec![0, 1, 2], q(1, 1))]);
    let mut scaled = at(&[(0, 1), (1, 1), (2, 1)]);
    scaled[1] = scaled[1].scale(&q(-3, 1));
    assert_eq!(read_coxeter(&scaled, &[], &mut stroke()).parallel, vec![(vec![0, 1, 2], q(2, 1))], "the ray, not its writing");
}

/// A ROSETTE PAIR IS REFUSED BY ITS PAIR AND BY ITS PRODUCT: a double-angle point off the crystallographic set, and
/// the product of the two reflections is a turn of no admitted order.
#[test]
fn a_rosette_pair_reads_an_infinite_order_product() {
    let pair = [v(&[(E1, 1)]), v(&[(E1, 1), (E2, 2)])];
    let p = coxeter_product(&pair, &[0, 1], true, &mut stroke()).expect("a product");
    assert_eq!(p.infinite, Some(BivectorKind::Elliptic), "a turn that never closes");
    let r = read_coxeter(&pair, &[p], &mut stroke());
    assert_eq!(r.closure, Closure::Infinite(Refusal::Pair(0, 1, PairRegime::Rosette)));
}

/// A 1-CELL'S TETHER READS ITS PENCIL AS A MOTION: two intersecting spheres
/// a turn, two disjoint a boost, two tangent a slip.
#[test]
fn a_tether_reads_turn_boost_or_slip_by_its_pair() {
    let sphere = |x: i64, r2: (i64, i64)| cga::sphere_at(&[q(x, 1), q(0, 1), q(0, 1)], &q(r2.0, r2.1));
    let kind = |a: Mv, b: Mv| {
        let p = coxeter_product(&[a, b], &[0, 1], true, &mut stroke()).expect("a product");
        let Some(Ok(ms)) = p.modes else { panic!("modes read") };
        ms.iter().map(|m| m.kind).collect::<Vec<_>>()
    };
    assert_eq!(kind(sphere(0, (1, 1)), sphere(1, (1, 1))), vec![BivectorKind::Elliptic], "intersecting: a turn");
    assert_eq!(kind(sphere(0, (1, 4)), sphere(3, (1, 4))), vec![BivectorKind::Hyperbolic], "disjoint: a boost");
    assert_eq!(kind(plane(E1, 0), plane(E1, 1)), vec![BivectorKind::Parabolic], "parallel planes: a slip");
}

/// CX1 — THE CUBE'S CORNER: each mirror pair's product is of order two; the
/// three mirrors' product is the point reflection, odd, `det −1`; its grades
/// are the meet alone, and its weight is the mirrors' own scalars.
#[test]
fn the_corner_products_are_of_order_two_and_the_whole_is_odd() {
    let corner = [plane(E1, 0), plane(E2, 0), plane(E3, 0)];
    let pair = coxeter_product(&corner, &[0, 1], true, &mut stroke()).expect("a product");
    let Some(Ok(ms)) = &pair.modes else { panic!("modes") };
    assert_eq!(ms.iter().map(|m| m.order).collect::<Vec<_>>(), vec![Some(2)], "a pair: a 180° turn");
    let whole = coxeter_product(&corner, &[0, 1, 2], true, &mut stroke()).expect("a product");
    assert_eq!(whole.det, Some(-1), "an odd product reverses orientation");
    assert_eq!(whole.grades, vec![3], "orthogonal mirrors: the meet alone, no Gram entry to place");
    // A theorem of the product, read here as a test of it and never carried
    // on it (78.28(3)): `P P̃ = Π s_a²`.
    let spheres = corner
        .iter()
        .fold(Mv::scalar(BigRational::from_integer(1.into())), |acc, s| acc.geo(&Mv::scalar(s.geo(s).scalar_part())));
    assert_eq!(whole.product.geo(&whole.product.reverse()), spheres, "P P̃ = Π s_a²");
    assert!(whole.infinite.is_none(), "every mode closes");
}

/// MR1b — ONE REFLECTION READS ONE MODE, `c = −1` ON ITS NORMAL, `det −1`,
/// and it closes at two.
#[test]
fn one_reflection_reads_one_mode_on_its_normal() {
    let mirror = v(&[(E1, 1), (E2, 1)]);
    let Some(Ok(ms)) = cga::motion_modes(&mirror, &mut stroke()) else { panic!("modes") };
    let [m] = ms.as_slice() else { panic!("one mode: {ms:?}") };
    assert_eq!((m.c.clone(), m.reflects, m.planes, m.order), (q(-1, 1), true, 0, Some(2)));
    assert_eq!(m.ray, mirror, "its ray is the normal");
    assert_eq!(cga::action_det(&mirror), Some(-1));
}

/// SY1 — THE SEALING STEP SPLITS: what it adds is a blade of the next grade,
/// and what it shares carries the pair's inversive product.
#[test]
fn the_sealing_step_splits_into_what_it_adds_and_what_it_shares() {
    let (a, b) = (v(&[(E1, 1)]), v(&[(E1, 1), (E2, 1)]));
    let p = coxeter_product(&[a.clone(), b.clone()], &[0, 1], true, &mut stroke()).expect("a product");
    assert_eq!(p.seal, b.geo(&a).grade(2), "what the step adds: the meet");
    assert_eq!(p.shared.scalar_part(), a.geo(&b).scalar_part(), "what it shares: s_b·s_a");
}

/// DF1 — THE DUAL FRAME: `s^a · s_b = δ_ab`; contracting the meet by `s^a`
/// gives the face opposite `a`, the wedge of the other two.
#[test]
fn a_2cells_dual_frame_contracts_its_meet_to_each_face() {
    let s = [v(&[(E1, 1)]), v(&[(E1, 1), (E2, 1)]), v(&[(E2, 1), (E3, 2)])];
    let DualFrame::Frame(dual) = cga::dual_frame(&s) else { panic!("a frame") };
    for (a, d) in dual.iter().enumerate() {
        for (b, t) in s.iter().enumerate() {
            let expect = if a == b { q(1, 1) } else { q(0, 1) };
            assert_eq!(d.geo(t).scalar_part(), expect, "s^{a} · s_{b}");
        }
    }
    let m = cga::wedge(&s);
    for (a, d) in dual.iter().enumerate() {
        let others: Vec<Mv> = s.iter().enumerate().filter(|&(b, _)| b != a).map(|(_, x)| x.clone()).collect();
        assert_eq!(cga::contract_to_face(d, &m, 3), cga::wedge(&others), "the face opposite {a}, as a ray");
    }
}

/// DF1 — A₂ NAMES ALL THREE ROOTS as its minimal dependent set, none chosen;
/// the corner's polars are its own mirrors.
#[test]
fn a2_names_all_three_roots_and_the_corners_polars_are_its_mirrors() {
    let (a, b) = (v(&[(E1, 1), (E2, -1)]), v(&[(E2, 1), (E3, -1)]));
    assert_eq!(cga::dual_frame(&[a.clone(), b.clone(), a.add(&b)]), DualFrame::Redundant(vec![0, 1, 2]));
    let corner = [plane(E1, 0), plane(E2, 0), plane(E3, 0)];
    let DualFrame::Frame(polars) = cga::dual_frame(&corner) else { panic!("a frame") };
    assert_eq!(polars, corner.to_vec(), "an orthonormal set is its own dual");
}
