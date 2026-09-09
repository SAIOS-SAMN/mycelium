// 45.7 — the exchange's pace: a peer is asked again only after its Λ on
// this court's own K, or when this court's own roots moved.

use crate::transport::exchange_due;

/// S13.1 — A PEER IS ASKED AGAIN ONLY AFTER ITS Λ OR WHEN THE ROOTS MOVED.
/// Never asked: due. Asked and Λ not yet elapsed on this court's K: not
/// due. Λ elapsed: due. Λ unmeasured: not polled, until these roots move.
#[test]
fn a_peer_is_asked_again_only_after_its_lambda_or_when_the_roots_moved() {
    assert!(exchange_due(None, 5, None, false), "never asked: due");
    assert!(!exchange_due(Some(5), 6, Some(4), false), "asked at K 5, Λ 4, K 6: not due");
    assert!(exchange_due(Some(5), 9, Some(4), false), "asked at K 5, Λ 4, K 9: due");
    assert!(!exchange_due(Some(5), 900, None, false), "Λ unmeasured: a silent peer is not polled");
    assert!(exchange_due(Some(5), 6, None, true), "the roots moved: due whatever Λ says");
    assert!(exchange_due(Some(5), 6, Some(4), true), "the roots moved: due before Λ elapsed");
}
