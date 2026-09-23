// BQ3, 71.2(3) — THE CARRIER IS GENERIC IN n: the algebra at n = 4, with its
// own count of blades and planes, holding the n = 3 algebra inside it.
//
// The n = 3 values are proved by the oracle (`oracle.rs`), which now runs on
// this carrier against the golden file the fixed carrier wrote.

use crate::transducer::cga::{self, base_axis, blades, bivector_planes, pseudo, Mv, EM, EP};
use num_rational::BigRational;
use num_traits::{One, Zero};

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(n.into(), d.into())
}

/// THE ALGEBRA AT n = 4: `2⁶` blades, fifteen planes, the signature's
/// metric, the new axis at its own bit, and the n = 3 algebra inside it.
#[test]
fn the_algebra_at_n4_has_its_own_count_and_holds_n3_inside_it() {
    assert_eq!(blades(4), 64, "Cl(5,1) has 2^6 blades");
    assert_eq!(bivector_planes(4).len(), 15, "C(5,2) + 5 = 15 planes");
    assert_eq!(bivector_planes(3).len(), 10, "and the n = 3 row of the family is ten");
    let e4 = base_axis(4);
    assert_eq!(e4, 1 << 5, "the fourth base axis takes bit 5, so e+ and e- never move");
    let one = |mask| Mv::blade(mask, BigRational::one());
    assert_eq!(one(e4).n(), 4, "a blade on the fourth axis belongs to n = 4");
    assert_eq!(one(e4).norm2(), q(1, 1), "a base axis squares to +1");
    assert_eq!(one(EP).norm2(), q(1, 1), "e+ squares to +1");
    assert_eq!(one(EM).norm2(), q(-1, 1), "e- squares to -1");
    assert_eq!(one(EP | EM).n(), 0, "e+ and e- belong to every algebra");
    // The pseudoscalar is grade n + 2, and invertible.
    let i4 = one(pseudo(4));
    assert_eq!((pseudo(4) as u32).count_ones(), 6, "grade n + 2");
    assert!(i4.versor_inverse().is_some(), "the pseudoscalar is invertible");
    // THE INCLUSION: an n = 3 value is the same value at n = 4.
    let a = Mv::blade(0b00011, q(2, 1));
    let b = one(e4 | EM);
    let ab = a.geo(&b);
    assert_eq!(ab.n(), 4, "a product lives in the larger algebra");
    assert_eq!(ab.coeff(0b00011 | e4 | EM).clone(), q(2, 1), "e1e2 · e4e- is the grade-4 blade, exactly");
    assert!(a.widened(4).projective_eq(&a), "widening moves no value");
    // A plane through the fourth axis turns and one through e- boosts.
    let turn = one(base_axis(1) | e4);
    let boost = one(e4 | EM);
    assert!(turn.geo(&turn).scalar_part() < BigRational::zero(), "e1e4 squares negative: harmonic");
    assert!(boost.geo(&boost).scalar_part() > BigRational::zero(), "e4e- squares positive: luminal");
    // The generated names read every plane at n = 4, the new axis by its
    // own index.
    let names: Vec<String> = bivector_planes(4).into_iter().map(cga::plane_name).collect();
    assert!(names.contains(&"e14".to_string()) && names.contains(&"e4m".to_string()), "{names:?}");
    // And the cyclotomic candidates widen with n: φ(ℓ) ≤ 5 admits no new
    // order over φ(ℓ) ≤ 4 (φ is even past 2), and φ(ℓ) ≤ 6 admits 7 and 9.
    assert_eq!(cga::cyclotomic_orders(3), vec![1, 2, 3, 4, 5, 6, 8, 10, 12]);
    assert_eq!(cga::cyclotomic_orders(4), cga::cyclotomic_orders(3));
    assert!(cga::cyclotomic_orders(5).contains(&7) && cga::cyclotomic_orders(5).contains(&9));
}
