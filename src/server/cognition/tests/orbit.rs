// OK1, 78.23 as amended — THE COURT'S CYCLE CLOSES BY NAME, and a collision
// is prevented at the boundary, never named after the fact.

use crate::chronometry::{ActKind, Close, ReceiptChain};
use crate::cognition::*;

/// A peer's line of one act at `mu`, closed: its two nodes.
fn peer_orbit(vk: [u8; 32], mu: [u64; 4]) -> Vec<crate::chronometry::KStepReceipt> {
    let mut p = ReceiptChain::with_vk(vk);
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
fn a_court_close_that_would_collide_is_withheld_and_sealed_a_stroke_on() {
    let mut k = Kernel::new();
    assert_eq!(k.orbit(), Close::Nothing, "a cycle that acted on nothing closes nothing");
    k.append_step(ActKind::Attest, [0xAA; 40], [1; 32]);
    let (_, ours) = k.chain.open_orbit().expect("the court's orbit is open");
    for r in peer_orbit([9; 32], ours) {
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

/// 78.66 — THE FOLD BY LEAST ACTION. A court that never ran a docket circuit
/// (no `court_stroke`, nothing listed) still folds: its orbit's close draws
/// what holding the hot ledger costs, and a lane that cannot cover it
/// proposes the fold at that close. Folding needed docket circuits before,
/// so a court with an empty docket held its whole history hot.
#[test]
fn a_court_with_an_empty_docket_folds_when_holding_its_ledger_runs_a_lane_short() {
    let mut k = Kernel::new();
    k.bind_identity([0x5c; 32]);
    k.append_step(ActKind::List, [1; 32], [2; 32]);
    k.physics.reserve = crate::physics::Reserve::of([0; 4]);
    assert!(matches!(k.orbit(), Close::Sealed(_)), "the close seals");
    assert_eq!(k.docket_circuits().circuits(), 0, "no docket circuit ran");
    let p = k.fold_proposal().expect("the close that ran a lane short proposed the fold");
    assert_eq!(p.root.at, crate::chronometry::Place { k: 0, i: 1 }, "the cut is the close just made");
}

/// HOLDING IS A DRAW, NAMED: a ledger held within every lane draws its
/// length off each, and one past a lane refuses by that lane and empties
/// what it held — never past empty.
#[test]
fn holding_the_ledger_draws_its_length_and_names_the_lane_that_refuses() {
    let mut p = crate::physics::Physics { reserve: crate::physics::Reserve::of([10, 10, 3, 10]), ..Default::default() };
    assert_eq!(p.hold(2), None, "held within every lane");
    assert_eq!(p.reserve.lanes(), [8, 8, 1, 8]);
    let refused = p.hold(4).expect("the 𝒯 lane holds one");
    assert_eq!(refused.lane, 2, "the refusal names the lane");
    assert_eq!(p.reserve.lanes(), [4, 4, 0, 4], "each lane is charged what it holds, never past empty");
}

/// THE DISTANCE AT A TICK, per lane and signed: a peer's orbit closed at the
/// court's closing tick at another frame is read at its distance, never a
/// sum of lanes.
#[test]
fn the_clock_names_the_per_lane_distance_at_the_last_close() {
    let mut k = Kernel::new();
    k.append_step(ActKind::Attest, [0xAA; 40], [1; 32]);
    let (_, ours) = k.chain.open_orbit().expect("open");
    let theirs = [ours[0] + 3, ours[1], ours[2], ours[3] + 1];
    for r in peer_orbit([9; 32], theirs) {
        k.chain.admit(r);
    }
    assert!(matches!(k.orbit(), Close::Sealed(_)), "frames apart: the close seals");
    assert_eq!(k.clock(None).orbits.distance, vec![([9; 32], [-3, 0, 0, -1])], "ours less theirs, lane by lane");
}
