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
/// factor. Unmeasured RTT is not passed here.
pub fn from_transport(rtt: u64, strokes: u64) -> Lambda {
    match rtt.checked_mul(strokes.max(1)) {
        Some(n) => Lambda::Strokes(n),
        None => Lambda::BeyondTheLedger,
    }
}

/// Deadline for one nerve dial, in strokes: an aperture of the measured
/// transport. Unmeasured RTT has no deadline.
pub fn transport_deadline(rtt: u64) -> Lambda {
    persist_dwell(rtt)
}

/// Persist dwell: a full aperture of measured transports, in strokes.
pub fn persist_dwell(rtt: u64) -> Lambda {
    from_transport(rtt, aperture_cycle() as u64)
}

/// Quiet-digest ceiling: a full reserve of measured transports, in strokes.
pub fn digest_ceiling(rtt: u64) -> Lambda {
    from_transport(rtt, reserve_max())
}
