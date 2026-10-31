// 78.15(1), CX1 — the tree conjugacy, tested before the cells read it.

use crate::transducer::cech::{coxeter_product, product_class, read_coxeter, Closure};
use crate::transducer::cga::{Mv, E1, E2, E3};
use num_rational::BigRational;

fn v(coords: &[(usize, i64)]) -> Mv {
    coords.iter().fold(Mv::zero(), |acc, &(m, c)| acc.add(&Mv::blade(m, BigRational::from_integer(c.into()))))
}

/// A₃ (AS D₃) IS A RECOGNISED SYSTEM WHOSE DIAGRAM IS A TREE, a path through
/// `e₁ − e₂`: the product of its three reflections in all six orders reads
/// ONE class — the Coxeter element, of order four (A₃'s Coxeter number).
#[test]
fn coxeter_product_reads_one_class_over_six_orders() {
    let roots = [v(&[(E1, 1), (E2, -1)]), v(&[(E2, 1), (E3, -1)]), v(&[(E2, 1), (E3, 1)])];
    let mut stroke = crate::physics::Physics::default().stroke_n(1 << 16);
    assert!(matches!(read_coxeter(&roots, &[], &mut stroke).closure, Closure::Finite { by_theorem: true, .. }));
    let orders = [[0, 1, 2], [0, 2, 1], [1, 0, 2], [1, 2, 0], [2, 0, 1], [2, 1, 0]];
    let classes: Vec<Vec<u8>> = orders
        .iter()
        .map(|o| product_class(&coxeter_product(&roots, o, true, &mut stroke).expect("a product")).expect("its modes read"))
        .collect();
    assert!(classes.windows(2).all(|w| w[0] == w[1]), "one class over every order");
    let p = coxeter_product(&roots, &[0, 1, 2], true, &mut stroke).expect("a product");
    let Some(Ok(modes)) = &p.modes else { panic!("modes") };
    let h = modes.iter().filter_map(|m| m.order).fold(1u32, |a, b| a / gcd(a, b) * b);
    assert_eq!(h, 4, "the Coxeter number of A₃");
}

fn gcd(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}
