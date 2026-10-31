// The transduction's tests, one file per organ they prove (MU-DERIVATIONS
// D3.2).

mod enclosed;
mod motion;
mod record;
mod relation;

use crate::transducer::family::perceive::Grid;

/// A grid from rows of digits, one hue per character.
pub(super) fn grid(rows: &[&str]) -> Grid {
    rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
}

/// A stroke of exactly `n` reads.
pub(super) fn stroke(n: usize) -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(n)
}
