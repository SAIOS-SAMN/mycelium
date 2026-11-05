// A4, MU-SENSING §3.7 — THE SIGNATURE IS DATA: every value of `Cl(n+1, 1)`
// keeps `e₋` as its one negative generator and its products unchanged; a
// value of `Cl(p, q)` reads its metric from its mask and refuses a mismatch.

use super::fixtures::{ax, q};
use crate::transducer::cga::{point_at, slots, Clifford, Mv, SignatureRefusal, E1, E2, EM, MAX_N};

/// The sign of reordering blade `a` before blade `b` into ascending order,
/// counted generator by generator: each generator of `a` above a generator
/// of `b` is one transposition.
fn transpositions_sign(a: usize, b: usize) -> i64 {
    let bits = |m: usize| (0..usize::BITS).filter(move |i| (m >> i) & 1 == 1);
    let swaps = bits(a).flat_map(|i| bits(b).filter(move |j| i > *j)).count();
    match swaps % 2 {
        0 => 1,
        _ => -1,
    }
}

/// THE CONFORMAL ALGEBRA IS UNCHANGED: over every pair of blades the
/// algebra at `n ≤ 4` spans, the product of two values of `Cl(n+1, 1)` is
/// `Cl(n+1, 1)`'s own, the reordering sign times −1 exactly where the two
/// blades share `e₋`, and nothing else. The masks at `n ≤ 3` are the fixed
/// `Cl(4,1)` carrier's thirty-two; the oracle golden reads the rest.
#[test]
fn geo_agrees_with_conformal_metric_on_every_blade_pair() {
    let span = slots(4).expect("the algebra at n = 4");
    for a in 0..span {
        for b in 0..span {
            let metric = match a & b & EM {
                0 => 1,
                _ => -1,
            };
            let expected = Mv::blade(a ^ b, q(transpositions_sign(a, b) * metric, 1));
            let product = Mv::blade(a, q(1, 1)).geo(&Mv::blade(b, q(1, 1)));
            assert!(product.sub(&expected).is_zero(), "e_{a:b} · e_{b:b}");
            assert_eq!(product.negatives(), EM, "e_{a:b} · e_{b:b} stays in Cl(n+1, 1)");
        }
    }
}

/// EVERY VALUE OF `Cl(n+1, 1)` CARRIES `e₋` ALONE: a scalar, a blade, a
/// point, their products and sums, and a value read back off the wire.
#[test]
fn negatives_equals_e_minus_for_conformal_values() {
    let p = point_at(&[q(1, 2), q(-3, 1), q(2, 1), q(5, 7)]);
    let built = [
        Mv::zero(),
        Mv::scalar(q(3, 1)),
        Mv::blade(E1 | E2, q(2, 1)),
        p.clone(),
        p.geo(&p.reverse()),
        p.add(&Mv::blade(ax(5), q(1, 1))),
        p.sub(&Mv::blade(EM, q(1, 1))).grade(1),
    ];
    for v in built {
        let mut bytes = Vec::new();
        v.to_bytes(&mut bytes);
        let back = Mv::from_bytes(&bytes, &mut 0).expect("the value reads back");
        assert_eq!(v.negatives(), EM, "{v:?}");
        assert_eq!(back.negatives(), EM, "{v:?} read back");
    }
}

/// A GENERATOR SQUARES TO −1 EXACTLY WHERE THE MASK NAMES IT: each base
/// axis squared under a mask naming it is −1, and +1 under `e₋` alone.
#[test]
fn product_reads_square_sign_from_negatives() {
    for i in 1..=usize::from(MAX_N) {
        let positive = Clifford::blade(EM, ax(i), q(1, 1)).expect("an axis the algebra holds");
        let negative = Clifford::blade(EM | ax(i), ax(i), q(1, 1)).expect("an axis the algebra holds");
        let up: Vec<_> = positive.product(&positive).expect("one signature").terms().map(|(m, v)| (m, v.clone())).collect();
        let down: Vec<_> = negative.product(&negative).expect("one signature").terms().map(|(m, v)| (m, v.clone())).collect();
        assert_eq!(up, [(0, q(1, 1))], "axis {i} under e₋");
        assert_eq!(down, [(0, q(-1, 1))], "axis {i} named negative");
    }
}

/// ON VALUES OF `Cl(n+1, 1)` THE CHECKED PRODUCT IS THE PRODUCT: their
/// inclusions compose to the inclusion of their geometric product.
#[test]
fn product_agrees_with_geo_on_conformal_values() {
    let a = point_at(&[q(1, 1), q(2, 3), q(-1, 2)]);
    let b = point_at(&[q(0, 1), q(5, 1), q(1, 4), q(3, 1)]).add(&Mv::blade(EM | ax(2), q(2, 1)));
    let checked = Clifford::of_conformal(&a).product(&Clifford::of_conformal(&b)).expect("one signature");
    let direct = Clifford::of_conformal(&a.geo(&b));
    assert!(checked.difference(&direct).expect("one signature").is_zero());
}

/// TWO SIGNATURES THAT DISAGREE ARE REFUSED, NEVER COERCED: a value with
/// `e₂` negative composed with a value of `Cl(n+1, 1)` holding `e₂` is
/// refused `Mismatched`, naming `e₂`, by the product, the sum and the
/// difference alike.
#[test]
fn product_refuses_mismatched_signatures() {
    let flipped = Clifford::blade(EM | E2, E1, q(1, 1)).expect("an axis the algebra holds");
    let conformal = Clifford::of_conformal(&Mv::blade(E2, q(1, 1)));
    let refused = Err(SignatureRefusal::Mismatched { generators: E2 });
    assert_eq!(flipped.product(&conformal).map(|v| v.n()), refused);
    assert_eq!(conformal.product(&flipped).map(|v| v.n()), refused);
    assert_eq!(flipped.sum(&conformal).map(|v| v.n()), refused);
    assert_eq!(flipped.difference(&conformal).map(|v| v.n()), refused);
}

/// A GENERATOR ONE ALGEBRA DOES NOT HOLD IS NO DISAGREEMENT: a value with
/// `e₂` negative composes with a value of `Cl(2, 1)`, which holds no `e₂`,
/// and the product keeps `e₂` negative.
#[test]
fn product_ignores_generator_one_algebra_lacks() {
    let flipped = Clifford::blade(EM | E2, E2, q(1, 1)).expect("an axis the algebra holds");
    let narrow = Clifford::of_conformal(&Mv::blade(E1, q(1, 1)));
    let composed = flipped.product(&narrow).expect("no shared generator disagrees");
    assert_eq!(composed.negatives(), EM | E2);
    let square = composed.product(&composed).expect("one signature");
    let read: Vec<_> = square.terms().map(|(m, v)| (m, v.clone())).collect();
    assert_eq!(read, [(0, q(1, 1))], "(e₂e₁)² = −e₂²e₁² = +1 with e₂² = −1");
}

/// A MASK PAST THE ALGEBRA'S BOUND IS REFUSED `Malformed`, as a blade and as
/// a negative mask, never shifted into another generator.
#[test]
fn blade_refuses_mask_past_max_n() {
    let past = 1usize << (usize::BITS - 1);
    assert_eq!(Clifford::blade(EM, past, q(1, 1)).map(|v| v.n()), Err(SignatureRefusal::Malformed { mask: past }));
    assert_eq!(Clifford::blade(EM | past, E1, q(1, 1)).map(|v| v.n()), Err(SignatureRefusal::Malformed { mask: EM | past }));
}
