use crate::core::gaussian::*;
use num_bigint::BigInt;

fn g(a: i64, b: i64) -> Gauss {
    Gauss::from_i64(a, b)
}

/// 37.7 P2 on the prime the alphabet does not carry. `17 = 4² + 1²`,
/// and the witness `4 + i` carries the factor: the gcd is that factor
/// itself, in canonical form, and its norm is the prime.
#[test]
fn prime_factor_reads_factor_of_prime_norm() {
    let p = BigInt::from(17);
    let pi = prime_factor(&g(4, 1), &p).expect("the witness carries 17");
    assert_eq!(pi.norm(), p, "the factor's norm IS the prime it factors");
    assert_eq!(pi, g(4, 1), "in A1 canonical form: the real part leads");
}

/// TWO COURTS, ONE FACTOR. Different associates of one factor — the
/// four units and the conjugate — reach the same canonical
/// representative, so two organs that each ran Euclid compare CONTENT
/// and never "the same up to a unit" (law 2, P3).
#[test]
fn canonical_maps_associates_to_one_representative() {
    let want = g(4, 1);
    for w in [g(4, 1), g(-1, 4), g(-4, -1), g(1, -4), g(1, 4)] {
        assert_eq!(w.canonical(), want, "{w:?} is the same factor");
    }
}

/// A witness that does not carry the prime is REFUSED, not rounded:
/// the gcd is a unit and there is no letter to open
/// (`REFUSE_LETTER_WITNESS`). `2 + i` has norm 5 and is coprime to 17.
#[test]
fn prime_factor_refuses_witness_without_prime() {
    assert_eq!(prime_factor(&g(2, 1), &BigInt::from(17)), None, "a unit gcd is no factor");
    // gcd(0, p) is p, whose norm is p² and not p: the arithmetic is
    // right and the witness still carries nothing.
    assert_eq!(prime_factor(&g(0, 0), &BigInt::from(17)), None, "and no witness is no factor");
    assert_eq!(prime_factor(&g(17, 0), &BigInt::from(17)), None, "nor a witness p divides");
}

/// The alphabet's own primes come back too — the arithmetic does not
/// know which primes the seventeen already spell, and must not.
#[test]
fn prime_factor_factors_alphabet_primes() {
    assert_eq!(prime_factor(&g(2, 1), &BigInt::from(5)).expect("5"), g(2, 1));
    assert_eq!(prime_factor(&g(3, 2), &BigInt::from(13)).expect("13"), g(3, 2));
}

/// Rule 10: Euclid descends on the norm, which is a non-negative
/// integer that strictly falls, so the remainder of a division is
/// always smaller than the divisor and the loop is bounded by the
/// witness's own size.
#[test]
fn rem_is_smaller_than_divisor() {
    for (x, y) in [(g(17, 0), g(4, 1)), (g(100, 37), g(6, 5)), (g(-9, 14), g(3, 2))] {
        let r = x.rem(&y).expect("a nonzero divisor divides");
        assert!(r.norm() < y.norm(), "{r:?} is smaller than {y:?}");
    }
}

/// The witness travels as a plane block (50.5): `[[s, −t], [t, s]]`
/// is `s + t·i`, and a block that is not a Gaussian arc refuses.
#[test]
fn from_block_reads_gaussian_and_refuses_non_integer() {
    use num_rational::BigRational;
    let q = |n: i64| BigRational::from_integer(BigInt::from(n));
    let block = [[q(4), q(-1)], [q(1), q(4)]];
    assert_eq!(from_block(&block).expect("an arc"), g(4, 1));
    let half = BigRational::new(BigInt::from(1), BigInt::from(2));
    assert_eq!(from_block(&[[half, q(0)], [q(0), q(1)]]), None, "not a Gaussian integer");
}
