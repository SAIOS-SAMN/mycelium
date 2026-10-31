// MD1b, BD1 — the rational roots of a polynomial over ℚ at any degree, by the
// rational root theorem; what has none refused by its degree; the divisor
// loop spending the stroke's budget and refusing by name when it is spent.

use super::common::{budget, q};
use crate::transducer::cga::modes::rational_roots;
use crate::transducer::cga::Undecomposed;

/// `x (x − 1/2)(x + 3)` reads all three roots, zero among them, and a
/// repeated factor is read once.
#[test]
fn rational_roots_reads_each_root_once() {
    // x(x − 1/2)(x + 3) = x³ + 5/2 x² − 3/2 x, low to high.
    let p = [q(0, 1), q(-3, 2), q(5, 2), q(1, 1)];
    assert_eq!(rational_roots(&p, &mut budget()), Ok(vec![q(-3, 1), q(0, 1), q(1, 2)]));
    // (x − 2)²(x + 1) = x³ − 3x² + 4: the double root once.
    let repeated = [q(4, 1), q(0, 1), q(-3, 1), q(1, 1)];
    assert_eq!(rational_roots(&repeated, &mut budget()), Ok(vec![q(-1, 1), q(2, 1)]));
}

/// WHAT HAS NO RATIONAL ROOT IS REFUSED BY ITS DEGREE: `x³ − 2` whole, and
/// `(x − 1)(x² − 2)` down to its quadratic after its rational root is read.
#[test]
fn rational_roots_refuses_irrational_factor_by_degree() {
    let cubic = [q(-2, 1), q(0, 1), q(0, 1), q(1, 1)];
    assert_eq!(rational_roots(&cubic, &mut budget()), Err(Undecomposed::Irrational(3)));
    // (x − 1)(x² − 2) = x³ − x² − 2x + 2.
    let mixed = [q(2, 1), q(-2, 1), q(-1, 1), q(1, 1)];
    assert_eq!(rational_roots(&mixed, &mut budget()), Err(Undecomposed::Irrational(2)));
}

/// 78.100, BD1 — NO INTEGER IS FACTORED UP TO DEGREE THREE. `x² − N` with `N`
/// the semiprime `1 000 003 · 1 000 033` asked a million trial divisions and
/// refused under a stroke of a thousand; the exact bisection reads it within
/// the same stroke — no rational root, refused by its degree as irrational.
/// A stroke that truly runs out still refuses by name and runs on no
/// further.
#[test]
fn rational_roots_reads_large_semiprime_within_stroke() {
    let n = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1_000_003u64 * 1_000_033u64));
    let hard = [-n, q(0, 1), q(1, 1)];
    let mut stroke = crate::physics::Physics::default().stroke_n(1000);
    assert_eq!(rational_roots(&hard, &mut stroke), Err(Undecomposed::Irrational(2)), "no rational root, read and refused by its degree");
    assert!(stroke.remaining() > 0, "within the stroke");
    let mut stroke = crate::physics::Physics::default().stroke_n(1000);
    assert_eq!(rational_roots(&[q(-9, 1), q(0, 1), q(1, 1)], &mut stroke), Ok(vec![q(-3, 1), q(3, 1)]));
    let mut spent = crate::physics::Physics::default().stroke_n(2);
    assert_eq!(rational_roots(&hard, &mut spent), Err(Undecomposed::Unbounded(2)), "a stroke that runs out refuses by name");
}

/// 78.100 — THE WIDTHS EPOCH 125 MET, READ EXACTLY. A cubic with rational
/// roots whose numerators run past 80 bits — `(x − 3/7)(x + 5/11)(x − 2⁸¹/3)`
/// — and a quadratic `(x − p/q)(x − r/s)` with 64-bit parts are each read
/// whole within a stroke of a few hundred; a cubic with one rational root and
/// an irreducible quadratic reads its root and refuses the rest by degree.
#[test]
fn rational_roots_reads_eighty_bit_roots_within_stroke() {
    use num_bigint::BigInt;
    let r = |n: BigInt, d: i64| num_rational::BigRational::new(n, BigInt::from(d));
    let mul = |p: &[num_rational::BigRational], root: &num_rational::BigRational| {
        // p(x)·(x − root), ascending.
        let mut out = vec![q(0, 1); p.len() + 1];
        for (i, c) in p.iter().enumerate() {
            out[i + 1] = &out[i + 1] + c;
            out[i] = &out[i] - c * root;
        }
        out
    };
    let big = r(BigInt::from(1) << 81, 3);
    let roots = [q(3, 7), q(-5, 11), big.clone()];
    let p = roots.iter().fold(vec![q(1, 1)], |p, root| mul(&p, root));
    let mut stroke = crate::physics::Physics::default().stroke_n(600);
    let mut want = roots.to_vec();
    want.sort();
    assert_eq!(rational_roots(&p, &mut stroke), Ok(want), "every rational root of the 81-bit cubic, read exactly");
    let (a, b) = (r(BigInt::from(0x1234_5678_9abc_def1u64), 97), r(-BigInt::from(0x0fed_cba9_8765_4321u64), 101));
    let quad = mul(&mul(&[q(1, 1)], &a), &b);
    let mut stroke = crate::physics::Physics::default().stroke_n(600);
    let mut want = vec![a, b];
    want.sort();
    assert_eq!(rational_roots(&quad, &mut stroke), Ok(want), "the 64-bit quadratic's roots, exactly");
    // (x − 1/2)(x² + x + 1): its rational root read, the rest refused.
    let mixed = mul(&[q(1, 1), q(1, 1), q(1, 1)], &q(1, 2));
    assert_eq!(rational_roots(&mixed, &mut crate::physics::Physics::default().stroke_n(600)), Err(Undecomposed::Irrational(2)));
    // 78.102 — NO DEGREE CUT-OFF: a quartic with a 70-bit root is read by the
    // same exact bisection, never by factoring.
    let quartic_roots = [q(1, 2), r(-(BigInt::from(1u8) << 70usize), 7), q(3, 1), q(-5, 3)];
    let quartic = quartic_roots.iter().fold(vec![q(1, 1)], |p, root| mul(&p, root));
    let mut want = quartic_roots.to_vec();
    want.sort();
    assert_eq!(rational_roots(&quartic, &mut crate::physics::Physics::default().stroke_n(4000)), Ok(want), "every root of the quartic, exactly");
}

/// 78.111 C1b — THE STROKE A POLYNOMIAL'S ROOTS NEED, READ OFF IT: a stroke of
/// exactly `roots_need(p)` reads every polynomial above to its end — roots
/// or a refusal by degree, never `Unbounded`, at degree two to six.
#[test]
fn roots_need_suffices_to_read_every_polynomial() {
    use crate::transducer::cga::modes::roots_need;
    let n = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1_000_003u64 * 1_000_033u64));
    let polys: Vec<Vec<num_rational::BigRational>> = vec![
        vec![q(0, 1), q(-3, 2), q(5, 2), q(1, 1)],
        vec![q(4, 1), q(0, 1), q(-3, 1), q(1, 1)],
        vec![q(-2, 1), q(0, 1), q(0, 1), q(1, 1)],
        vec![q(2, 1), q(-2, 1), q(-1, 1), q(1, 1)],
        vec![-n, q(0, 1), q(1, 1)],
        vec![q(6, 1), q(-5, 1), q(-5, 1), q(5, 1), q(-1, 1), q(0, 1), q(1, 1)],
    ];
    for p in &polys {
        let mut need = crate::physics::Physics::default().stroke_n(roots_need(p).expect("a need inside the carrier"));
        assert!(!matches!(rational_roots(p, &mut need), Err(Undecomposed::Unbounded(_))), "read to its end within its need: {p:?}");
    }
}
