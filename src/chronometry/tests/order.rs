// 45.3, T15, 78.24 — the order two nodes stand in, read off the nodes' stamps,
// and what an exhausted budget answers.

use super::common::budget;
use crate::chronometry::*;

/// One act with nothing named.
fn step(c: &mut ReceiptChain, parents: Vec<Parent>) -> KStepReceipt {
    c.append(ActKind::List, [0; 4], Vec::new(), [9; 32], parents).expect("appends").clone()
}

fn cite(r: &KStepReceipt) -> Parent {
    Parent { at: r.stamp, hash: r.chain_hash }
}

/// Precedence implies a lower height, and two geneses of two signers stay concurrent.
#[test]
fn precedes_implies_lower_height_and_keeps_concurrency() {
    let mut c = ReceiptChain::with_vk([1; 32]);
    let r0 = step(&mut c, Vec::new());
    let r1 = step(&mut c, Vec::new());
    let r2 = step(&mut c, Vec::new());
    let mut b = budget(8);
    assert_eq!(c.precedes(&r0.stamp, &r1.stamp, &mut b), Some(true), "earlier ≺ later");
    let mut b = budget(8);
    assert_eq!(c.precedes(&r1.stamp, &r2.stamp, &mut b), Some(true));
    let mut b = budget(8);
    assert_eq!(c.precedes(&r2.stamp, &r0.stamp, &mut b), Some(false), "later does not precede earlier");
    assert!(r0.stamp.h < r1.stamp.h && r1.stamp.h < r2.stamp.h);
    // TWO GENESES OF TWO SIGNERS ARE TWO NODES: the signer is a coordinate
    // of the stamp and in the hash, so the same act sealed by two chains
    // is two receipts.
    let mut d = ReceiptChain::with_vk([3; 32]);
    let d0 = step(&mut d, Vec::new());
    let mut e = ReceiptChain::with_vk([4; 32]);
    let e0 = step(&mut e, Vec::new());
    assert_ne!(d0.chain_hash, e0.chain_hash, "two signers, two nodes");
    let mut mix = ReceiptChain::with_vk([1; 32]);
    step(&mut mix, Vec::new());
    mix.admit(d0.clone());
    mix.admit(e0.clone());
    step(&mut mix, vec![cite(&d0), cite(&e0)]);
    let mut b = budget(8);
    assert_eq!(mix.precedes(&d0.stamp, &e0.stamp, &mut b), Some(false), "two geneses are concurrent");
    assert_eq!(mix.at_tick(0).len(), 3, "and stand at one tick with this chain's genesis: the width at h 0");
}

/// ORDERING TWO OF THIS CHAIN'S OWN NODES IS ONE READ, at any depth — the
/// chain's own nodes are a line and `K` orders them — and a budget with
/// nothing left refuses rather than answering short (rule 11).
#[test]
fn precedes_orders_own_nodes_in_one_read_and_refuses_empty_budget() {
    let mut c = ReceiptChain::with_vk([1; 32]);
    let mut all = Vec::new();
    for _ in 0..64 {
        all.push(step(&mut c, Vec::new()));
    }
    let (r0, rn) = (&all[0].stamp, &all[63].stamp);
    let mut one = budget(1);
    assert_eq!(c.precedes(r0, rn, &mut one), Some(true), "sixty-four nodes deep and one read answers it");
    let mut one = budget(1);
    assert_eq!(c.precedes(rn, r0, &mut one), Some(false), "and the other way");
    let mut one = budget(1);
    assert_eq!(c.precedes(r0, r0, &mut one), Some(false), "nothing precedes itself");
    let mut none = budget(0);
    assert_eq!(c.precedes(r0, rn, &mut none), None, "unmeasured, not false");
}

/// A PEER NODE CUTS THIS CHAIN'S OWN LINE IN TWO PLACES and nowhere else:
/// the greatest of the chain's own `K` it reaches, and the least of the
/// chain's own nodes that reaches it.
#[test]
fn precedes_places_peer_node_by_its_cut_of_own_line() {
    let mut c = ReceiptChain::with_vk([1; 32]);
    let own0 = step(&mut c, Vec::new());

    // A PEER THAT REACHES THE CHAIN'S FIRST NODE: it cites it as a parent.
    let mut d = ReceiptChain::with_vk([3; 32]);
    step(&mut d, Vec::new());
    d.admit(own0.clone());
    let over = step(&mut d, vec![cite(&own0)]);
    c.admit(over.clone());
    let mut b = budget(4);
    assert_eq!(c.precedes(&own0.stamp, &over.stamp, &mut b), Some(true), "the chain's own node is below a peer node that cites it");
    let mut b = budget(4);
    assert_eq!(c.precedes(&over.stamp, &own0.stamp, &mut b), Some(false), "and the peer is not below the chain's own node");

    // UNTIL THE CHAIN CITES IT, a peer node is above nothing of the chain's own.
    let mut e = ReceiptChain::with_vk([4; 32]);
    let apart = step(&mut e, Vec::new());
    c.admit(apart.clone());
    let mut b = budget(4);
    assert_eq!(c.precedes(&apart.stamp, &own0.stamp, &mut b), Some(false), "an uncited peer node precedes nothing of the chain's own");
    let mut b = budget(4);
    assert_eq!(c.precedes(&own0.stamp, &apart.stamp, &mut b), Some(false), "and nothing of the chain's own precedes it");
    let mut b = budget(4);
    assert_eq!(c.precedes(&apart.stamp, &over.stamp, &mut b), Some(false), "two peers neither of which reaches the other are concurrent");

    // ONCE THE CHAIN CITES IT, every node of its own from that step on reaches it.
    let own1 = step(&mut c, vec![cite(&apart)]);
    let mut b = budget(4);
    assert_eq!(c.precedes(&apart.stamp, &own1.stamp, &mut b), Some(true), "the step that cited it is above it");
    let mut b = budget(4);
    assert_eq!(c.precedes(&apart.stamp, &own0.stamp, &mut b), Some(false), "and the step before that still is not");
}

/// A FOLD RENUMBERS NOTHING (45.8, 78.24): a peer node's place against this
/// chain's own line is the chain's own place, which a fold does not move, so a peer node reached
/// before the fold is reached by every node that survives it.
#[test]
fn fold_before_keeps_peer_placements() {
    let mut c = ReceiptChain::with_vk([1; 32]);
    let mut e = ReceiptChain::with_vk([4; 32]);
    let peer = step(&mut e, Vec::new());
    step(&mut c, Vec::new());
    c.admit(peer.clone());
    step(&mut c, vec![cite(&peer)]);
    // A fold cuts only at an orbit's close (OK1): the two acts close the
    // orbit they share, and the survivor opens the next.
    assert!(c.fold_before(crate::chronometry::Place { k: 0, i: 1 }).is_none(), "an act that closes nothing is no cut");
    let (_, frame) = c.open_orbit().expect("open");
    let shut = c.append(ActKind::Orbit, frame, Vec::new(), [9; 32], Vec::new()).expect("the close").clone();
    let survivor = step(&mut c, Vec::new());
    let (cold, root) = c.fold_before(shut.stamp.place()).expect("the first orbit folds");
    assert_eq!(cold.len(), 3, "its two acts and its close left the hot line");
    assert_eq!(root.at, shut.stamp.place(), "the line stands on the close");
    assert!(c.verify_chain_integrity(), "the hot line verifies on its root");
    let mut b = budget(4);
    assert_eq!(c.precedes(&peer.stamp, &survivor.stamp, &mut b), Some(true), "reached before the fold is reached by everything after it");
    assert!(c.node(&peer.stamp).is_some(), "the peer node stays held at its coordinates");
}
