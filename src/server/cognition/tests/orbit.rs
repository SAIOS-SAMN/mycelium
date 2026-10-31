// OK1, 78.23 as amended — THE COURT'S CYCLE CLOSES BY NAME, and a collision
// is prevented at the boundary, never named after the fact.

use crate::chronometry::{ActKind, Close, ReceiptChain};
use crate::cognition::*;

/// A peer's line of one act at `mu`, closed: its two nodes.
fn peer_orbit(signer: [u8; 32], mu: [u64; 4]) -> Vec<crate::chronometry::KStepReceipt> {
    let mut p = ReceiptChain::with_signer(signer);
    p.append(ActKind::List, mu, Vec::new(), [0; 32], Vec::new()).expect("acts");
    let (_, frame) = p.open_orbit().expect("open");
    p.append(ActKind::Orbit, frame, Vec::new(), [0; 32], Vec::new()).expect("closes");
    p.own().collect()
}

/// THE COURT'S CLOSE, WITHHELD AND THEN SEALED. The court acts once and
/// holds a peer whose orbit closed at the very tick the court's close would
/// take, at an equal frame on every lane: the close is WITHHELD — the
/// orbit stays open, `K` stands, the count goes on the clock. The court's
/// next stroke acts, the close's tick moves past the peer's, and it seals:
/// `K + 1`, and the clock names the per-lane distance to the peer's orbit
/// closed at the court's closing tick, where there is one.
#[test]
fn orbit_withholds_colliding_close_then_seals() {
    let mut k = Court::new();
    assert_eq!(k.orbit(), Close::Nothing, "a cycle that acted on nothing closes nothing");
    k.append_step(ActKind::Attest, [0xAA; 40], [1; 32]);
    let (_, own_frame) = k.chain.open_orbit().expect("the court's orbit is open");
    for r in peer_orbit([9; 32], own_frame) {
        k.chain.admit(r);
    }
    assert_eq!(k.orbit(), Close::Withheld, "its frame met the peer's at one tick");
    assert_eq!(k.k_step(), 0, "K stands");
    assert!(k.chain.open_orbit().is_some(), "the orbit stays open");
    assert_eq!(k.clock(None).orbits.withheld, 1, "the withheld close is on the clock");
    k.append_step(ActKind::Attest, [0xAB; 40], [1; 32]);
    let Close::Sealed(at) = k.orbit() else { panic!("the next stroke's close seals") };
    assert_eq!((at.k, k.k_step()), (0, 1), "K moved at the close");
    assert!(k.chain.verify_chain_integrity());
    assert!(
        k.clock(None).orbits.distance.is_empty(),
        "no other orbit closed at the court's closing tick, so no distance is named"
    );
}

/// 78.68, 78.69 — THE FOLD IS ASKED AT EVERY CLOSE AND ANSWERS BY WHETHER THE
/// ALGORITHM CHANGED. A court that never ran a docket circuit is asked at
/// its closes all the same. However much work its prefix carries, it
/// proposes nothing while no transition stands there
/// (`REFUSE_FOLD_NO_TRANSITION`). At the first close whose prefix holds a
/// `Return`, a `RankStep`, or a generator admitted (`Install`,
/// `Ruling(Upheld)`, `Adopted`), the fold is proposed, cut at that close.
/// Neither bytes nor the reserve enter.
#[test]
fn orbit_proposes_fold_only_after_transition() {
    use crate::chronometry::Ruled;
    for transition in [ActKind::Return, ActKind::RankStep, ActKind::Reconnect, ActKind::Install, ActKind::Ruling(Ruled::Upheld), ActKind::Adopted] {
        let mut k = Court::new();
        k.bind_identity([0x5c; 32]);
        for _ in 0..8 {
            k.append_step(ActKind::List, [1; 32], [2; 32]);
            assert!(matches!(k.orbit(), Close::Sealed(_)), "the close seals");
            assert!(k.fold_proposal().is_none(), "work without a transition proposes nothing");
            assert_eq!(k.propose_fold_at_close(), Err(crate::verdict::REFUSE_FOLD_NO_TRANSITION));
        }
        k.append_step(transition, [3; 32], [0x5c; 32]);
        assert!(matches!(k.orbit(), Close::Sealed(_)));
        assert_eq!(k.docket_circuits().circuits(), 0, "no docket circuit ran");
        let p = k.fold_proposal().expect("the close after a transition proposes");
        assert_eq!(p.root.at, crate::chronometry::Place { k: 8, i: 1 }, "the cut is the close just made");
    }
}

/// THE DISTANCE AT A TICK, per lane and signed: a peer's orbit closed at the
/// court's closing tick at another frame is read at its distance, never a
/// sum of lanes.
#[test]
fn clock_names_per_lane_distance_at_close() {
    let mut k = Court::new();
    k.append_step(ActKind::Attest, [0xAA; 40], [1; 32]);
    let (_, own_frame) = k.chain.open_orbit().expect("open");
    let peer_frame = [own_frame[0] + 3, own_frame[1], own_frame[2], own_frame[3] + 1];
    for r in peer_orbit([9; 32], peer_frame) {
        k.chain.admit(r);
    }
    assert!(matches!(k.orbit(), Close::Sealed(_)), "frames apart: the close seals");
    assert_eq!(k.clock(None).orbits.distance, vec![([9; 32], [-3, 0, 0, -1])], "the court's own frame less the peer's, lane by lane");
}
