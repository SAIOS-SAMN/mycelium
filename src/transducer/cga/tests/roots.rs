// MD1b — the rational roots of a polynomial over ℚ at any degree, by the
// rational root theorem; what has none refused by its degree.

use super::common::q;
use crate::transducer::cga::modes::rational_roots;
use crate::transducer::cga::Undecomposed;

/// `x (x − 1/2)(x + 3)` reads all three roots, zero among them, and a
/// repeated factor is read once.
#[test]
fn every_rational_root_is_read_once_at_any_degree() {
    // x(x − 1/2)(x + 3) = x³ + 5/2 x² − 3/2 x, low to high.
    let p = [q(0, 1), q(-3, 2), q(5, 2), q(1, 1)];
    assert_eq!(rational_roots(&p), Ok(vec![q(-3, 1), q(0, 1), q(1, 2)]));
    // (x − 2)²(x + 1) = x³ − 3x² + 4: the double root once.
    let repeated = [q(4, 1), q(0, 1), q(-3, 1), q(1, 1)];
    assert_eq!(rational_roots(&repeated), Ok(vec![q(-1, 1), q(2, 1)]));
}

/// WHAT HAS NO RATIONAL ROOT IS REFUSED BY ITS DEGREE: `x³ − 2` whole, and
/// `(x − 1)(x² − 2)` down to its quadratic after its rational root is read.
#[test]
fn a_factor_with_no_rational_root_is_refused_by_its_degree() {
    assert_eq!(rational_roots(&[q(-2, 1), q(0, 1), q(0, 1), q(1, 1)]), Err(Undecomposed::Irrational(3)));
    // (x − 1)(x² − 2) = x³ − x² − 2x + 2.
    assert_eq!(rational_roots(&[q(2, 1), q(-2, 1), q(-1, 1), q(1, 1)]), Err(Undecomposed::Irrational(2)));
}
