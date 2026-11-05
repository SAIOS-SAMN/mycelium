// LAW-PASSAGE v2 (proposal 78.172 §5): the passage `q ↦ v_k·q` as a
// one-sided product in `Cl(p, q)`, each frame plane hyperbolic, its four
// joint-read fixtures and the law's claims over every plane the algebra holds.

use super::fixtures::q;
use crate::transducer::cga::{base_axis, Clifford, EM};
use num_rational::BigRational;

/// Plane `k`'s closed pole `e_Zk`, squaring to +1: base axis `2k − 1`.
fn closed(k: usize) -> Option<usize> {
    base_axis(2 * k - 1)
}

/// Plane `k`'s open pole `e_Qk`, squaring to −1: base axis `2k`.
fn open(k: usize) -> Option<usize> {
    base_axis(2 * k)
}

/// Every frame plane the algebra's masks hold, in order.
fn planes() -> impl Iterator<Item = usize> {
    (1..).map_while(|k| open(k).map(|_| k))
}

/// The signature of the frame planes `1..=m`: each plane's open pole
/// negative, beside the conformal pair's `e₋`.
fn signature(m: usize) -> usize {
    (1..=m).filter_map(open).fold(EM, |s, g| s | g)
}

/// A pole of a plane the fixture names.
fn pole(g: Option<usize>) -> usize {
    g.expect("a plane the algebra holds")
}

/// `c` times the product of the generators `gens`, in the order written,
/// under the signature of the planes `1..=m`.
fn word(m: usize, c: BigRational, gens: &[usize]) -> Clifford {
    let sig = signature(m);
    let one = Clifford::blade(sig, 0, c).expect("the scalar");
    gens.iter().fold(one, |w, g| {
        let generator = Clifford::blade(sig, *g, q(1, 1)).expect("a generator the algebra holds");
        w.product(&generator).expect("one signature")
    })
}

/// The sum of `words`, each under one signature.
fn sum(words: Vec<Clifford>) -> Clifford {
    let mut it = words.into_iter();
    let first = it.next().expect("a term");
    it.fold(first, |s, w| s.sum(&w).expect("one signature"))
}

/// The direction `h·e_Zk + r·e_Qk` of plane `k`, under the planes `1..=m`.
fn direction(m: usize, k: usize, h: BigRational, r: BigRational) -> Clifford {
    sum(vec![word(m, h, &[pole(closed(k))]), word(m, r, &[pole(open(k))])])
}

/// FIXTURE 2's state: `q = 3 + 2·e_Z2Z3 + e_Z1Q2`, an even multivector over
/// three planes.
fn state() -> Clifford {
    sum(vec![
        word(3, q(3, 1), &[]),
        word(3, q(2, 1), &[pole(closed(2)), pole(closed(3))]),
        word(3, q(1, 1), &[pole(closed(1)), pole(open(2))]),
    ])
}

/// The degrees the value holds terms on, ascending, each once.
fn degrees(v: &Clifford) -> Vec<u32> {
    let mut d: Vec<u32> = v.terms().map(|(m, _)| m.count_ones()).collect();
    d.sort_unstable();
    d.dedup();
    d
}

/// FIXTURE 1: at the horizon the passage is one-way. With the null
/// direction `v = e_Z1 + e_Q1`, `v·v·q = 0` for fixture 2's state.
#[test]
fn product_of_null_direction_twice_annuls_state() {
    let v = direction(3, 1, q(1, 1), q(1, 1));
    let once = v.product(&state()).expect("one signature");
    assert!(!once.is_zero(), "one passage carries the state");
    assert!(v.product(&once).expect("one signature").is_zero(), "v·v·q = 0");
}

/// FIXTURE 2: one crossing takes even degrees to odd ones. With
/// `v = e_Z1 + 2·e_Q1` and `q = 3 + 2·e_Z2Z3 + e_Z1Q2`, `v·q` has terms on
/// degrees 1 and 3 only.
#[test]
fn product_carries_even_state_to_odd_degrees() {
    let s = state();
    assert_eq!(degrees(&s), [0, 2], "the state is even");
    let v = direction(3, 1, q(1, 1), q(2, 1));
    assert_eq!(degrees(&v.product(&s).expect("one signature")), [1, 3]);
}

/// FIXTURE 3: two boosts sharing an axis leave a turning on their two other
/// axes: `[e_Z1Q1, e_Z1Q2] = e_Z1Q1·e_Z1Q2 − e_Z1Q2·e_Z1Q1 = −2·e_Q1Q2`, the
/// commutator unhalved.
#[test]
fn product_commutator_of_boosts_equals_open_turning() {
    let a = word(2, q(1, 1), &[pole(closed(1)), pole(open(1))]);
    let b = word(2, q(1, 1), &[pole(closed(1)), pole(open(2))]);
    let commutator = a.product(&b).expect("one signature").difference(&b.product(&a).expect("one signature")).expect("one signature");
    let turning = word(2, q(-2, 1), &[pole(open(1)), pole(open(2))]);
    assert!(commutator.difference(&turning).expect("one signature").is_zero(), "[e_Z1Q1, e_Z1Q2] = −2·e_Q1Q2");
    assert!(!commutator.is_zero());
}

/// FIXTURE 4: a turning and a boost sharing `Z₂` at equal sizes are a null
/// pair: `(e_Z1Z2 + e_Q1Z2)² = 0`.
#[test]
fn product_squares_null_pair_to_zero() {
    let pair = sum(vec![
        word(2, q(1, 1), &[pole(closed(1)), pole(closed(2))]),
        word(2, q(1, 1), &[pole(open(1)), pole(closed(2))]),
    ]);
    assert!(!pair.is_zero());
    assert!(pair.product(&pair).expect("one signature").is_zero());
}

/// EVERY PLANE'S HORIZON IS ONE-WAY: in every frame plane the algebra
/// holds, a direction with `H = ±R` annuls twice, `v_k·v_k·q = 0`, for
/// exact rational sizes on either sheet.
#[test]
fn product_annuls_twice_at_every_horizon() {
    let m = planes().count();
    let s = sum(vec![word(m, q(3, 1), &[]), word(m, q(2, 1), &[pole(closed(1)), pole(open(m))])]);
    for k in planes() {
        for (h, r) in [(q(5, 3), q(5, 3)), (q(-2, 7), q(2, 7))] {
            let v = direction(m, k, h, r);
            let twice = v.product(&v.product(&s).expect("one signature")).expect("one signature");
            assert!(twice.is_zero(), "plane {k}");
        }
    }
}

/// PASSAGES IN DIFFERENT PLANES ANTICOMMUTE: over every pair of frame
/// planes the algebra holds, `v_k·v_l = −v_l·v_k`, so their order is held
/// in the state's sign.
#[test]
fn product_anticommutes_across_planes() {
    let m = planes().count();
    for k in planes() {
        for l in planes().filter(|l| *l != k) {
            let vk = direction(m, k, q(3, 2), q(-1, 4));
            let vl = direction(m, l, q(2, 1), q(7, 5));
            let kl = vk.product(&vl).expect("one signature");
            let lk = vl.product(&vk).expect("one signature");
            assert!(!kl.is_zero());
            assert!(kl.sum(&lk).expect("one signature").is_zero(), "planes {k}, {l}");
        }
    }
}
