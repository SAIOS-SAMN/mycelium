// 78.14, GR1 — the constructors' Gram form: the span signature beside the
// vector rank, and each pair's regime by δ², with no iteration.

use super::common::q;
use crate::transducer::cga::*;

fn v(coords: &[(usize, i64)]) -> Mv {
    coords.iter().fold(Mv::zero(), |acc, &(m, c)| acc.add(&Mv::blade(m, q(c, 1))))
}

/// A₂'S THREE ROOTS, OBTUSE: singular, positive semidefinite, every pair of
/// order three — and NOT affine. The zero direction is dependence (three
/// roots in a plane), not degeneracy; the group is dihedral of order six.
#[test]
fn a2s_three_roots_are_singular_by_dependence_not_affine() {
    let (a, b) = (v(&[(E1, 1), (E2, -1)]), v(&[(E2, 1), (E3, -1)]));
    let c = a.add(&b).scale(&q(-1, 1));
    let g = read_gram(&[a, b, c]);
    assert_eq!(g.signature, (2, 0, 1), "positive semidefinite, one zero direction");
    assert_eq!(g.rank, 2, "three roots in a plane");
    assert_eq!((g.degenerate(), g.dependent()), (0, 1), "the zero is dependence, not a null direction: not affine");
    assert!(g.pairs.iter().all(|&(_, _, _, r)| r == PairRegime::Finite(3)), "every pair of order three: {:?}", g.pairs);
}

/// A REAL AND AN IMAGINARY SPHERE, ORTHOGONAL: `G = diag(1, −1)`, indefinite,
/// and the pair's product is of order two — `ℤ₂ × ℤ₂`, finite. The pairwise
/// `δ²`, not the signature, names open.
#[test]
fn an_indefinite_orthogonal_pair_is_finite() {
    let g = read_gram(&[Mv::blade(E1, q(1, 1)), Mv::blade(EM, q(1, 1))]);
    assert_eq!(g.signature, (1, 1, 0));
    assert_eq!(g.pairs[0].3, PairRegime::Finite(2));
}

/// THE REGIMES BY δ²: a 90°-turn pair, two parallel planes (parabolic),
/// two disjoint spheres (a boost), a real sphere against an imaginary one
/// not orthogonal (a boost), and a non-crystallographic angle (a rosette).
#[test]
fn each_pair_reads_its_regime_by_delta_squared() {
    let regime = |a: Mv, b: Mv| read_gram(&[a, b]).pairs[0].3;
    assert_eq!(regime(v(&[(E1, 1)]), v(&[(E1, 1), (E2, 1)])), PairRegime::Finite(4));
    let plane = |d: i64| v(&[(E1, 1)]).add(&e_inf().scale(&q(d, 1)));
    assert_eq!(regime(plane(0), plane(1)), PairRegime::Parabolic, "parallel planes: δ² = 1");
    let sphere = |x: i64| sphere_at(&[q(x, 1), q(0, 1), q(0, 1)], &q(1, 4));
    assert_eq!(regime(sphere(0), sphere(3)), PairRegime::Boost, "disjoint spheres: δ² > 1");
    assert_eq!(regime(v(&[(E1, 1), (EM, 1)]).add(&v(&[(E1, 1)])), v(&[(EM, 1)])), PairRegime::Boost, "real against imaginary, not orthogonal");
    assert_eq!(regime(v(&[(E1, 1)]), v(&[(E1, 1), (E2, 2)])), PairRegime::Rosette, "δ² = 1/5: no finite order over ℚ");
}
