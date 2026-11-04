// 45 — fixtures for the chronometry claims.

use crate::physics::{Budget, Physics};

/// A budget of exactly `n` visits. The walks below are bounded by the
/// receipts they reach, not by this box's reserve: `stroke()` is
/// `(reserve_max()/8).max(2)`, so a hardware-derived budget made these
/// claims pass on a large host and fail on a small one.
pub(super) fn budget(n: usize) -> Budget {
    Physics::default().stroke_n(n)
}
