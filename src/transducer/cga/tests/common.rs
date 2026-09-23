// A4 — the one fixture constructor the algebra's tests share: an exact rational.

use num_bigint::BigInt;
use num_rational::BigRational;

pub(super) fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(BigInt::from(n), BigInt::from(d))
}

/// One stroke's read token for a fixture (BD1: the root reader spends the
/// stroke's budget). Sized by the test, as `stroke_n` is for test callers:
/// no fixture's coefficients come near it, and the refusal when a stroke is
/// spent is tested at its own size in `roots.rs`.
pub(super) fn budget() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 16)
}
