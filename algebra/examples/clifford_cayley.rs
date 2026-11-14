//! Standalone exact rational Clifford algebra example in `mycelium-algebra` (#283):
//!
//! 1. Construct exact basis vectors `e1, e2, e3` in `Cl(3, 0)(Q)`.
//! 2. Compute the geometric product `e1 e2` (unit bivector `e12`).
//! 3. Apply a Cayley rotor `R = 1 - e12` to rotate `e1` by 90 degrees exactly
//!    with zero floating-point error.
//! 4. Verify conformal null generator properties in `Cl(4, 1)(Q)`:
//!    `e_+^2 = +1`, `e_-^2 = -1`, `e_inf = e_- + e_+` with `e_inf^2 = 0`.

use mycelium_algebra::{Mv, E1, E2, E3, EM, EP};
use num_rational::BigRational;

fn main() {
    let one = BigRational::from_integer(1.into());
    let neg_one = BigRational::from_integer((-1).into());
    let half = BigRational::new(1.into(), 2.into());

    // 1. Exact basis vectors in Cl(3, 0)(Q).
    let e1 = Mv::blade(E1, one.clone());
    let e2 = Mv::blade(E2, one.clone());
    let e3 = Mv::blade(E3, one.clone());
    assert_eq!(e1.geo(&e1), Mv::scalar(one.clone()));
    assert_eq!(e2.geo(&e2), Mv::scalar(one.clone()));
    assert_eq!(e3.geo(&e3), Mv::scalar(one.clone()));

    // 2. Geometric product e1 * e2 = unit bivector e12 (e12^2 = -1).
    let e12 = e1.geo(&e2);
    assert_eq!(e12, Mv::blade(E1 | E2, one.clone()));
    assert_eq!(e12.geo(&e12), Mv::scalar(neg_one));
    println!("e1 * e2 = {e12:?}");

    // 3. Exact 90-degree rotation of e1 -> e2 using Cayley rotor R = 1 - e12.
    // Since R * R~ = (1 - e12)(1 + e12) = 2, scaling the sandwich by 1/2 yields e2 exactly.
    let rotor = Mv::scalar(one.clone()).sub(&e12);
    let rotated = rotor.sandwich(&e1).scale(&half);
    assert_eq!(rotated, e2);
    assert!(rotor.sandwich(&e1).projective_eq(&e2));
    println!("rotated e1 by 90 deg = {rotated:?} (exact == e2)");

    // 4. Conformal null generators in Cl(4, 1)(Q).
    let ep = Mv::blade(EP, one.clone());
    let em = Mv::blade(EM, one.clone());
    assert_eq!(ep.geo(&ep), Mv::scalar(one.clone()));
    assert_eq!(em.geo(&em), Mv::scalar(BigRational::from_integer((-1).into())));

    let e_inf = em.add(&ep);
    assert!(e_inf.geo(&e_inf).is_zero());
    println!("e_+^2 = +1, e_-^2 = -1, e_inf^2 = 0 verified exactly");
}
