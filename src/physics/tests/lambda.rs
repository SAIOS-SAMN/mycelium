use super::super::*;

/// A measured round trip is counted in strokes, and a same-stroke reply
/// reads zero, not a floor.
#[test]
fn from_transport_counts_rtt_in_strokes() {
    assert_eq!(from_transport(3, 4), Lambda::Strokes(12), "measured rtt × strokes");
    assert_eq!(from_transport(3, 0), Lambda::Strokes(3), "zero strokes is still one transport");
    assert_eq!(
        from_transport(0, 4),
        Lambda::Strokes(0),
        "a same-stroke reply is zero, not a floor"
    );
    let (Lambda::Strokes(quiet), Lambda::Strokes(one)) =
        (digest_ceiling(3), from_transport(3, 1))
    else {
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
        from_transport(u64::MAX, 2),
        Lambda::BeyondTheLedger,
        "a product that does not fit is beyond the ledger, not a saturated extent"
    );
    assert_ne!(
        from_transport(u64::MAX, 2),
        Lambda::Unmeasured,
        "and it is not the same reading as a transport nobody measured"
    );
}
