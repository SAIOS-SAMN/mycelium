// 45.7 — the exchange's pace: a peer is asked again only after its Λ on
// this court's own K, or when this court's own roots moved; a quiet peer
// only once the TUniversal final point has advanced (D2.5, 78.43).

use crate::transport::{exchange_due, quiet_due};

/// D2.5, 78.43 — A QUIET PEER IS NOT ASKED AGAIN WHILE THE FINAL POINT
/// STANDS STILL: asked when the final point had advanced 7 times, it is not
/// asked again at 7, however many exchange cycles pass, so an idle network
/// stays quiet.
#[test]
fn quiet_peer_not_asked_again_while_final_point_still() {
    let asked_at = Some(7);
    for _cycle in 0..1_000 {
        assert!(!quiet_due(asked_at, 7), "the final point has not advanced since the ask");
    }
}

/// D2.5, 78.43 — A QUIET PEER IS ASKED ONCE AFTER THE FINAL POINT ADVANCES:
/// never asked, it is due; asked at 7, it is due once the final point
/// advances to 8, and once asked there (the ask reads 8), it is not due
/// again until the final point moves on.
#[test]
fn quiet_peer_asked_once_after_final_point_advances() {
    assert!(quiet_due(None, 0), "never asked: due");
    assert!(quiet_due(Some(7), 8), "the final point advanced since the ask: due");
    assert!(!quiet_due(Some(8), 8), "asked at the advance: not due again");
    assert!(quiet_due(Some(8), 9), "due again once it moves on");
}

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
