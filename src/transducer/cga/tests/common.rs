// A4 — the one fixture constructor the algebra's tests share: an exact rational.

use num_bigint::BigInt;
use num_rational::BigRational;

pub(super) fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(BigInt::from(n), BigInt::from(d))
}
