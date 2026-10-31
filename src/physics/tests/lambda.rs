use super::super::*;
use std::num::NonZeroU64;

fn nz(n: u64) -> NonZeroU64 {
    NonZeroU64::new(n).expect("non-zero")
}

/// A measured round trip is counted in strokes, times a non-zero factor: the
/// measurement counts the poll that read its first byte, so no zero is
/// passed (the claim that a same-stroke reply reads zero is dropped with it,
/// MU-DERIVATIONS §2 item 13); a zero handed to a deadline is no
/// measurement, read `Unmeasured`.
#[test]
fn from_transport_counts_rtt_in_strokes() {
    assert_eq!(from_transport(nz(3), nz(4)), Lambda::Strokes(12), "measured rtt × strokes");
    assert_eq!(from_transport(nz(3), nz(1)), Lambda::Strokes(3), "one stroke's factor is one transport");
    assert_eq!(transport_deadline(0), Lambda::Unmeasured, "a zero is no measurement");
    let (Lambda::Strokes(quiet), Lambda::Strokes(one)) = (digest_ceiling(3), from_transport(nz(3), nz(1))) else {
        panic!("a measured round trip has a horizon the ledger counts");
    };
    assert!(quiet >= one);
    let d = transport_deadline(5);
    assert_eq!(d, persist_dwell(5), "the deadline is an aperture of the measured transport");
}

/// 47.5's two refusing arms are two readings (law 5): a round trip
/// nobody measured, and a horizon wider than any extent the ledger
/// counts. One `None` over both would put an arithmetic fact and a
/// missing measurement on one branch of the eviction path.
#[test]
fn from_transport_reads_overflow_as_beyond_ledger() {
    assert_eq!(
        from_transport(nz(u64::MAX), nz(2)),
        Lambda::BeyondTheLedger,
        "a product that does not fit is beyond the ledger, not a saturated extent"
    );
    assert_ne!(
        from_transport(nz(u64::MAX), nz(2)),
        Lambda::Unmeasured,
        "and it is not the same reading as a transport nobody measured"
    );
}
