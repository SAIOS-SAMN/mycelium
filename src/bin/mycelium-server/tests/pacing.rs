// 45.7 — the exchange's pace: a peer is asked again only after its Λ on
// this court's own K, or when this court's own roots moved.

use crate::transport::exchange_due;

/// S13.1 — A PEER IS ASKED AGAIN ONLY AFTER ITS Λ OR WHEN THE ROOTS MOVED.
/// Never asked: due. Asked and Λ not yet elapsed on this court's K: not
/// due. Λ elapsed: due. Λ unmeasured — a peer that never answered: asked
/// again once per aperture of this court's K, never sooner, and at once
/// when these roots move. It was never re-asked short of the roots moving,
/// and a peer silent while it booted was cut off for good (2026-09-30).
#[test]
fn exchange_due_waits_for_lambda_unless_roots_moved() {
    let aperture = u64::from(mycelium::physics::aperture_cycle());
    assert!(exchange_due(None, 5, None, false), "never asked: due");
    assert!(!exchange_due(Some(5), 6, Some(4), false), "asked at K 5, Λ 4, K 6: not due");
    assert!(exchange_due(Some(5), 9, Some(4), false), "asked at K 5, Λ 4, K 9: due");
    assert!(!exchange_due(Some(5), 5 + aperture - 1, None, false), "Λ unmeasured: not before an aperture");
    assert!(exchange_due(Some(5), 5 + aperture, None, false), "Λ unmeasured: asked again after an aperture");
    assert!(exchange_due(Some(5), 6, None, true), "the roots moved: due whatever Λ says");
    assert!(exchange_due(Some(5), 6, Some(4), true), "the roots moved: due before Λ elapsed");
}
