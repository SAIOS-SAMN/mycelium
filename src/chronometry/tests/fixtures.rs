// 45 — fixtures for the chronometry claims.

use crate::chronometry::{KStepReceipt, Parent};
use crate::court::admission::Predecessor;
use crate::physics::{Budget, Physics};

/// ONE RUN OF RECEIPTS, read by the link sense (MU-HOSTING §5a.10): the
/// same count, and each receipt of `a` at the stamp of its fellow in `b`
/// with a chain continuing across the two, each re-deriving its own hash.
/// A receipt has no `==`; this is how two runs are compared.
pub(super) fn one_run(a: &[KStepReceipt], b: &[KStepReceipt]) -> bool {
    a.len() == b.len() && a.iter().zip(b).all(|(x, y)| x.rederives() && y.rederives() && Predecessor::cites(&Parent::of(x), &Parent::of(y)))
}

/// A budget of exactly `n` visits. The walks below are bounded by the
/// receipts they reach, not by this box's reserve: `stroke()` is
/// `(reserve_max()/8).max(2)`, so a hardware-derived budget made these
/// claims pass on a large host and fail on a small one.
pub(super) fn budget(n: usize) -> Budget {
    Physics::default().stroke_n(n)
}
