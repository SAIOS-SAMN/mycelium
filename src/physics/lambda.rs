use super::calibration::{aperture_cycle, reserve_max};

/// 47.5's Λ as a reading, with its two refusing arms told apart (law 5).
///
/// `Unmeasured` is no horizon: no round trip to this peer has been
/// measured, so nothing is known about how long a listing may wait.
/// `BeyondTheLedger` is a horizon WIDER than any extent the ledger counts
/// — arithmetic, not a failure, and it says no listing has waited that
/// long. One `None` over both would flatten two facts onto the eviction
/// path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lambda {
    /// The horizon, in strokes.
    Strokes(u64),
    /// No round trip measured to this peer.
    Unmeasured,
    /// The product does not fit the carrier the ledger counts in.
    BeyondTheLedger,
}

/// A duration in strokes: measured round-trip strokes times a stroke-count
/// factor, both non-zero by their measurement (D2.6: a measured round trip
/// counts the poll that read its first byte, and a factor is an aperture or
/// a reserve).
pub fn from_transport(rtt: std::num::NonZeroU64, strokes: std::num::NonZeroU64) -> Lambda {
    match rtt.get().checked_mul(strokes.get()) {
        Some(n) => Lambda::Strokes(n),
        None => Lambda::BeyondTheLedger,
    }
}

/// `rtt` round trips times `factor`, where both are measured: a zero is no
/// measurement, read `Unmeasured`, never one.
fn of_measured(rtt: u64, factor: u64) -> Lambda {
    match (std::num::NonZeroU64::new(rtt), std::num::NonZeroU64::new(factor)) {
        (Some(r), Some(f)) => from_transport(r, f),
        _ => Lambda::Unmeasured,
    }
}

/// Deadline for one nerve dial, in strokes: an aperture of the measured
/// transport. Unmeasured RTT has no deadline.
pub fn transport_deadline(rtt: u64) -> Lambda {
    persist_dwell(rtt)
}

/// Persist dwell: a full aperture of measured transports, in strokes.
pub fn persist_dwell(rtt: u64) -> Lambda {
    of_measured(rtt, u64::from(aperture_cycle()))
}

/// Quiet-digest ceiling: a full reserve of measured transports, in strokes.
pub fn digest_ceiling(rtt: u64) -> Lambda {
    of_measured(rtt, reserve_max())
}
