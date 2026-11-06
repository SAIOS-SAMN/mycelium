// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// 78.171 amendment (d), ROADMAP track 26 S7 stage 3 — a sphere enters only
// as a holding's own enclosure: the membrane, and its images under the
// field's motions, never a sphere built from a squared radius.

use super::fixtures::{ax, q};
use crate::transducer::cga::*;
use num_rational::BigRational;

/// A centre at `n` axes, each coordinate exact.
fn centre(n: usize) -> Vec<BigRational> {
    (0..n).map(|i| q(2 * i as i64 - 1, 3)).collect()
}

/// A SPHERE IS BUILT WITHOUT A SQUARED RADIUS: the membrane at the origin is
/// the throat's fixed sphere `−e₊` at unit weight, and the enclosure about
/// `c` at the gauge `ρ` is its image under the dilation by `ρ` and the slide
/// to `c`, at every `n` from 1 to 4. It holds `c ± ρ·eᵢ` on each axis and
/// not `c + ρ/2·eᵢ`, its centre reads back `c` exactly, and it is spacelike,
/// so it acts; no gauge `ρ ≤ 0` names an enclosure.
#[test]
fn sphere_built_without_squared_radius() {
    assert_eq!(membrane(), Mv::blade(EP, q(-1, 1)), "the membrane is −e₊");
    assert!(membrane_at(&[], &q(1, 1)).is_some_and(|m| m.projective_eq(&membrane())), "unit gauge at the origin");
    let rho = q(5, 2);
    for n in 1..=4usize {
        let c = centre(n);
        let s = membrane_at(&c, &rho).expect("a positive gauge");
        let stands = |offset: &BigRational, i: usize| {
            let mut p = c.clone();
            p[i] = &p[i] + offset;
            point_at(&p).geo(&s).scalar_part() == q(0, 1)
        };
        for i in 0..n {
            assert!(stands(&rho, i) && stands(&-rho.clone(), i), "n = {n}: c ± ρ·e{} stands on it", i + 1);
            assert!(!stands(&(&rho / q(2, 1)), i), "n = {n}: c + ρ/2·e{} is inside", i + 1);
        }
        assert_eq!(base_at(&s, n), Some(c.clone()), "n = {n}: its centre reads back");
        assert!(is_versor(&s), "n = {n}: an enclosure has volume, so it acts");
        assert_eq!(s.coeff(ax(1)), &c[0], "n = {n}: read at unit weight");
    }
    assert_eq!(membrane_at(&centre(2), &q(0, 1)), None, "no extent, no enclosure");
    assert_eq!(membrane_at(&centre(2), &q(-1, 1)), None, "a negative gauge names none");
}
