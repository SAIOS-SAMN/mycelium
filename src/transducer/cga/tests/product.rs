// A4, MU-SENSING §3.7 — FACTS OF THE PRODUCT IN `Cl(p, q)`, written in the
// generators of frame planes with one negative unit each. These were
// LAW-PASSAGE v2's fixtures (78.172, the product `v_k·q`); since v3 the
// passage is the pole swap (`forms::passage`), and they stand here as facts
// of `Clifford::product` alone, never as the law.

use super::fixtures::q;
use crate::transducer::cga::{base_axis, Clifford, EM};
use num_rational::BigRational;

/// Plane `k`'s positive unit: base axis `2k − 1`.
fn positive(k: usize) -> Option<usize> {
    base_axis(2 * k - 1)
}

/// Plane `k`'s negative unit: base axis `2k`.
fn negative(k: usize) -> Option<usize> {
    base_axis(2 * k)
}

/// Every plane the algebra's masks hold, in order.
fn planes() -> impl Iterator<Item = usize> {
    (1..).map_while(|k| negative(k).map(|_| k))
}

/// The signature of the planes `1..=m`, beside `e₋`.
fn signature(m: usize) -> usize {
    (1..=m).filter_map(negative).fold(EM, |s, g| s | g)
}

fn unit(g: Option<usize>) -> usize {
    g.expect("a plane the algebra holds")
}

/// `c` times the product of `gens`, in the order written, under the
/// signature of the planes `1..=m`.
fn word(m: usize, c: BigRational, gens: &[usize]) -> Clifford {
    let sig = signature(m);
    let one = Clifford::blade(sig, 0, c).expect("the scalar");
    gens.iter().fold(one, |w, g| w.product(&Clifford::blade(sig, *g, q(1, 1)).expect("a generator")).expect("one signature"))
}

fn sum(words: Vec<Clifford>) -> Clifford {
    let mut it = words.into_iter();
    let first = it.next().expect("a term");
    it.fold(first, |s, w| s.sum(&w).expect("one signature"))
}

/// The vector `h·e₊ₖ + r·e₋ₖ`, under the planes `1..=m`.
fn vector(m: usize, k: usize, h: BigRational, r: BigRational) -> Clifford {
    sum(vec![word(m, h, &[unit(positive(k))]), word(m, r, &[unit(negative(k))])])
}

/// An even value over three planes: `3 + 2·e₊₂e₊₃ + e₊₁e₋₂`.
fn even() -> Clifford {
    sum(vec![
        word(3, q(3, 1), &[]),
        word(3, q(2, 1), &[unit(positive(2)), unit(positive(3))]),
        word(3, q(1, 1), &[unit(positive(1)), unit(negative(2))]),
    ])
}

fn degrees(v: &Clifford) -> Vec<u32> {
    let mut d: Vec<u32> = v.terms().map(|(m, _)| m.count_ones()).collect();
    d.sort_unstable();
    d.dedup();
    d
}

/// A null vector annuls on a second product, in every plane and on either
/// sign: `v·(v·x) = 0` where `v = h·e₊ₖ ± h·e₋ₖ`.
#[test]
fn product_of_null_vector_twice_annuls() {
    let m = planes().count();
    let x = sum(vec![word(m, q(3, 1), &[]), word(m, q(2, 1), &[unit(positive(1)), unit(negative(m))])]);
    for k in planes() {
        for (h, r) in [(q(1, 1), q(1, 1)), (q(5, 3), q(5, 3)), (q(-2, 7), q(2, 7))] {
            let v = vector(m, k, h, r);
            assert!(!v.product(&x).expect("one signature").is_zero(), "plane {k}");
            assert!(v.product(&v.product(&x).expect("one signature")).expect("one signature").is_zero(), "plane {k}");
        }
    }
}

/// A vector times an even value lies on odd degrees: `(e₊₁ + 2·e₋₁)·x` on
/// degrees 1 and 3 only.
#[test]
fn product_of_vector_and_even_value_is_odd() {
    let x = even();
    assert_eq!(degrees(&x), [0, 2]);
    assert_eq!(degrees(&vector(3, 1, q(1, 1), q(2, 1)).product(&x).expect("one signature")), [1, 3]);
}

/// Vectors of different planes anticommute, over every pair the algebra
/// holds.
#[test]
fn product_anticommutes_across_planes() {
    let m = planes().count();
    for k in planes() {
        for l in planes().filter(|l| *l != k) {
            let (a, b) = (vector(m, k, q(3, 2), q(-1, 4)), vector(m, l, q(2, 1), q(7, 5)));
            let (ab, ba) = (a.product(&b).expect("one signature"), b.product(&a).expect("one signature"));
            assert!(!ab.is_zero());
            assert!(ab.sum(&ba).expect("one signature").is_zero(), "planes {k}, {l}");
        }
    }
}

/// Two boosts sharing an axis leave a turning, the commutator unhalved:
/// `[e₊₁e₋₁, e₊₁e₋₂] = −2·e₋₁e₋₂`.
#[test]
fn product_commutator_of_boosts_equals_turning() {
    let a = word(2, q(1, 1), &[unit(positive(1)), unit(negative(1))]);
    let b = word(2, q(1, 1), &[unit(positive(1)), unit(negative(2))]);
    let commutator = a.product(&b).expect("one signature").difference(&b.product(&a).expect("one signature")).expect("one signature");
    let turning = word(2, q(-2, 1), &[unit(negative(1)), unit(negative(2))]);
    assert!(!commutator.is_zero());
    assert!(commutator.difference(&turning).expect("one signature").is_zero());
}

/// A turning and a boost sharing an axis at equal sizes square to zero:
/// `(e₊₁e₊₂ + e₋₁e₊₂)² = 0`.
#[test]
fn product_squares_null_pair_to_zero() {
    let pair = sum(vec![word(2, q(1, 1), &[unit(positive(1)), unit(positive(2))]), word(2, q(1, 1), &[unit(negative(1)), unit(positive(2))])]);
    assert!(!pair.is_zero());
    assert!(pair.product(&pair).expect("one signature").is_zero());
}
