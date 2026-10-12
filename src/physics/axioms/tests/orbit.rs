// 78.23 as amended, OK1 — A K-STEP IS ONE ORBIT: `K` moves where a cycle
// closes by name and nowhere else, and a close carries its orbit's frame.

use crate::chronometry::{ActKind, KStepReceipt, Parent, Place, ReceiptChain};
use crate::court::admission::{admit_backfilled, Admitted, REFUSE_RECEIPT};

/// One act measuring `mu`.
fn act(c: &mut ReceiptChain, mu: [u64; 4]) -> KStepReceipt {
    c.append(ActKind::List, mu, Vec::new(), [0; 32], Vec::new()).expect("appends").clone()
}

/// The close of `c`'s open orbit, at the frame the ledger reads for it.
fn close(c: &mut ReceiptChain) -> KStepReceipt {
    let (_, frame) = c.open_orbit().expect("an orbit is open");
    c.append(ActKind::Orbit, frame, Vec::new(), [0; 32], Vec::new()).expect("closes").clone()
}

/// OK1 AT THE WRITER: acts move `i` and never `K`; the close takes the
/// orbit's last place carrying the per-lane sum of its acts' frames, at
/// the orbit's tick; the next act opens `K + 1` at `i = 0`; a close with
/// no act since the last seals nothing; a close at any other frame is
/// refused.
#[test]
fn k_moves_where_a_cycle_closes_and_nowhere_else() {
    assert!(!crate::physics::axioms::OK1_K_COUNTS_CLOSED_CYCLES.is_empty());
    let mut c = ReceiptChain::with_vk([1; 32]);
    assert!(c.open_orbit().is_none(), "nothing acted, nothing to close");
    let a0 = act(&mut c, [1, 0, 2, 0]);
    let a1 = act(&mut c, [0, 3, 0, 0]);
    let a2 = act(&mut c, [4, 0, 0, 5]);
    assert_eq!(
        [a0.stamp.place(), a1.stamp.place(), a2.stamp.place()],
        [Place { k: 0, i: 0 }, Place { k: 0, i: 1 }, Place { k: 0, i: 2 }],
        "three acts, one orbit: i moves, K does not"
    );
    assert_eq!(c.next_step_index(), 0, "K stands while the orbit is open");
    assert!(
        c.append(ActKind::Orbit, [5, 3, 2, 4], Vec::new(), [0; 32], Vec::new()).is_none(),
        "a close at a frame that is not its orbit's is refused at the writer"
    );
    let shut = close(&mut c);
    assert_eq!(shut.stamp.place(), Place { k: 0, i: 3 }, "the close stands at the orbit's last place");
    assert_eq!(shut.mu, [5, 3, 2, 5], "carrying the per-lane sum of its acts, lanes never mixed");
    assert_eq!(shut.stamp.h, a2.stamp.h + 1, "at the orbit's tick");
    assert_eq!(c.next_step_index(), 1, "K moved at the close");
    assert!(c.open_orbit().is_none(), "and a close with no act since seals nothing");
    assert!(
        c.append(ActKind::Orbit, [0; 4], Vec::new(), [0; 32], Vec::new()).is_none(),
        "an empty close is refused"
    );
    let b0 = act(&mut c, [0, 0, 0, 1]);
    assert_eq!(b0.stamp.place(), Place { k: 1, i: 0 }, "the next act opens K + 1 at i = 0");
    assert_eq!(c.last_close().map(|r| r.stamp), Some(shut.stamp), "the last close is read off the line");
    assert!(c.verify_chain_integrity(), "the line re-derives, the close's frame with it");
}

/// OK1 AT THE READERS: a close whose frame is not its orbit's sum — sealed
/// with a hash that re-derives, so only the frame is wrong — is refused by
/// the ledger's integrity check and by a peer's admission, exactly as a
/// height that does not re-derive is.
#[test]
fn a_close_whose_frame_is_not_its_orbits_is_refused_on_read() {
    let mut b = ReceiptChain::with_vk([2; 32]);
    let a0 = act(&mut b, [1, 1, 0, 0]);
    let forged = KStepReceipt::seal(
        crate::chronometry::Stamp::new(a0.stamp.h + 1, [2; 32], Place { k: 0, i: 1 }),
        ActKind::Orbit,
        [9, 1, 0, 0],
        Some(Parent { at: a0.stamp, hash: a0.chain_hash }),
        Vec::new(),
        Vec::new(),
        [0; 32],
    )
    .expect("seals");
    assert!(forged.rederives(), "its hash is its own; only its frame lies");
    assert!(
        ReceiptChain::from_receipts(vec![a0.clone(), forged.clone()]).is_none(),
        "the integrity check re-derives the frame and refuses the line"
    );
    let mut peer = ReceiptChain::with_vk([3; 32]);
    peer.admit(a0);
    assert_eq!(admit_backfilled(&forged, &peer), Admitted::Refused(REFUSE_RECEIPT), "and admission refuses it");
    let mut honest = ReceiptChain::with_vk([2; 32]);
    act(&mut honest, [1, 1, 0, 0]);
    let shut = close(&mut honest);
    assert!(matches!(admit_backfilled(&shut, &peer), Admitted::Parent(_)), "the true close admits");
    let early = KStepReceipt::seal(
        crate::chronometry::Stamp::new(shut.stamp.h + 1, [2; 32], Place { k: 0, i: 2 }),
        ActKind::List,
        [0; 4],
        Some(Parent { at: shut.stamp, hash: shut.chain_hash }),
        Vec::new(),
        Vec::new(),
        [0; 32],
    )
    .expect("seals");
    peer.admit(shut);
    assert_eq!(
        admit_backfilled(&early, &peer),
        Admitted::Refused(REFUSE_RECEIPT),
        "an act after a close that does not open the next orbit is refused"
    );
}

/// A COLLISION IS DECIDED BY EXACT EQUALITY, lane by lane, against another
/// signer's orbit closed at the same tick — no constant, no threshold. The
/// withholding it decides is the court's (`server::cognition::tests::orbit`).
#[test]
fn a_close_is_decided_by_exact_equality_of_frames_at_one_tick() {
    let mut other = ReceiptChain::with_vk([4; 32]);
    act(&mut other, [2, 0, 0, 0]);
    let theirs = close(&mut other);
    let mut mine = ReceiptChain::with_vk([5; 32]);
    act(&mut mine, [2, 0, 0, 0]);
    for r in other.own().collect::<Vec<_>>() {
        mine.admit(r);
    }
    let (_, frame) = mine.open_orbit().expect("open");
    let tick = mine.next_stamp(&[]).expect("a stamp").h;
    assert_eq!(tick, theirs.stamp.h, "one tick");
    assert!(mine.collides(tick, frame), "equal on every lane at one tick: a collision");
    assert!(!mine.collides(tick, [2, 0, 0, 1]), "one lane apart is not");
    assert!(!mine.collides(tick + 1, frame), "nor is another tick");
}
