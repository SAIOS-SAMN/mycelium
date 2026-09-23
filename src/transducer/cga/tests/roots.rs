// MD1b, BD1 — the rational roots of a polynomial over ℚ at any degree, by the
// rational root theorem; what has none refused by its degree; the divisor
// loop spending the stroke's budget and refusing by name when it is spent.

use super::common::{budget, q};
use crate::transducer::cga::modes::rational_roots;
use crate::transducer::cga::Undecomposed;

/// `x (x − 1/2)(x + 3)` reads all three roots, zero among them, and a
/// repeated factor is read once.
#[test]
fn every_rational_root_is_read_once_at_any_degree() {
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
fn a_factor_with_no_rational_root_is_refused_by_its_degree() {
    let cubic = [q(-2, 1), q(0, 1), q(0, 1), q(1, 1)];
    assert_eq!(rational_roots(&cubic, &mut budget()), Err(Undecomposed::Irrational(3)));
    // (x − 1)(x² − 2) = x³ − x² − 2x + 2.
    let mixed = [q(2, 1), q(-2, 1), q(-1, 1), q(1, 1)];
    assert_eq!(rational_roots(&mixed, &mut budget()), Err(Undecomposed::Irrational(2)));
}

/// BD1 — THE DIVISOR LOOP SPENDS THE STROKE'S BUDGET. `x² − N` with `N` the
/// semiprime `1 000 003 · 1 000 033` needs a million trial divisions to
/// factor; under a stroke of a thousand reads it refuses by name as
/// `Unbounded(2)` and does not run on. The same polynomial with a small
/// constant term reads within the same stroke.
#[test]
fn a_large_semiprime_refuses_by_name_within_the_budget() {
    let n = num_rational::BigRational::from_integer(num_bigint::BigInt::from(1_000_003u64 * 1_000_033u64));
    let hard = [-n, q(0, 1), q(1, 1)];
    let mut stroke = crate::physics::Physics::default().stroke_n(1000);
    assert_eq!(rational_roots(&hard, &mut stroke), Err(Undecomposed::Unbounded(2)));
    assert_eq!(stroke.remaining(), 0, "the refusal spent the stroke and no more");
    let mut stroke = crate::physics::Physics::default().stroke_n(1000);
    assert_eq!(rational_roots(&[q(-9, 1), q(0, 1), q(1, 1)], &mut stroke), Ok(vec![q(-3, 1), q(3, 1)]));
}
