// 45.1, 45.4, 78.24 — the voice's clock is its own ledger: one node per
// admitted act, naming what it acts on by coordinates, and no reply moves
// it.

use crate::anastome::dialogue;
use crate::chronometry::{ActKind, Cite, Parent, ReceiptChain, Stamp};
use crate::voice::Voice;

/// 45.1, 45.4 — A VOICE'S CLOCK IS ITS OWN LEDGER. It seats with an empty
/// line and seals one node per admitted act — a form stood, an utterance, a
/// page learned — each under this voice's key and linked to the one before;
/// its K is the line's length. Nothing sets it, so a reply carrying another
/// kernel's k-step cannot move it, and a request seals nothing.
#[test]
fn a_voice_seals_one_node_per_admitted_act_and_its_k_is_the_line() {
    let mut v = Voice::named("worker-a");
    assert_eq!(v.identity(), Voice::named("worker-a").identity());
    assert_ne!(v.identity(), Voice::named("worker-b").identity());
    assert_eq!(v.k(), 0, "a voice that has spoken nothing accrues nothing");
    assert_eq!(v.chain().own().count(), 0, "and holds no node");

    let taught = Cite::Word {
        chart: crate::transducer::stalk::Invariant::counts(&[1]),
        class: None,
        at: Stamp { h: 7, vk: [9; 32], k: 3, i: 0 },
    };
    // Three acts of distinct measure, so the nodes can be told apart by
    // what they carried and not only by what kind they were.
    assert_eq!(v.seal(ActKind::Stood, [1, 0, 0, 0], Vec::new()), 1, "one admitted act is one node");
    assert_eq!(v.seal(ActKind::Uttered, [0, 2, 0, 0], Vec::new()), 2);
    assert_eq!(v.seal(ActKind::Learned, [0, 0, 0, 3], vec![taught.clone()]), 3);
    assert_eq!(v.k(), 3, "K is the line's length");

    let rs: Vec<_> = v.chain().own().cloned().collect();
    assert_eq!(rs.len(), 3);
    assert_eq!(rs[2].cites, vec![taught], "the node names what it learned by its coordinates");
    // 45.1: and what it measured, per lane, on the node itself.
    assert_eq!(rs[0].mu, [1, 0, 0, 0], "the act's own measure rides the node");
    assert_eq!(rs[1].mu, [0, 2, 0, 0]);
    assert_eq!(rs[2].mu, [0, 0, 0, 3]);
    assert_eq!(rs[0].stamp.vk, v.identity(), "under this voice's key");
    assert_eq!(rs[0].stamp.k, 0);
    assert_eq!(rs[1].prev, Some(Parent { at: rs[0].stamp, hash: rs[0].chain_hash }), "and links to its predecessor");
    assert_ne!(rs[0].operator_hash, rs[1].operator_hash, "the act is on the node");
    assert!(v.chain().verify_chain_integrity(), "the line re-derives end to end");

    let ask = v.quote().expect("the ask fits its lengths");
    assert_eq!(dialogue::act_tag(&ask), Some(dialogue::ACT_QUOTE));
    assert_eq!(v.k(), 3, "a request is not an admitted act and seals nothing");
}

/// 78.24 (d1) — A VOICE'S LEDGER RESTORES UNDER ITS OWN KEY AND NO OTHER:
/// its K continues across a boot, and a ledger signed by another key is
/// refused whole.
#[test]
fn a_voices_ledger_restores_under_its_own_key_and_no_other() {
    let mut v = Voice::named("worker-a");
    v.seal(ActKind::Stood, [1, 0, 0, 0], Vec::new());
    v.seal(ActKind::Uttered, [0, 1, 0, 0], Vec::new());
    let mut tape = Vec::new();
    v.chain().write(&mut tape).expect("the ledger writes");
    let chain = ReceiptChain::read(&tape, &mut 0).expect("and reads");
    let mut booted = Voice::named("worker-a");
    assert!(booted.restore(chain.clone(), Vec::new()), "its own ledger restores");
    assert_eq!(booted.k(), 2, "and its K continues from where it stood");
    let mut other = Voice::named("worker-b");
    assert!(!other.restore(chain, Vec::new()), "another voice's ledger is refused");
    assert_eq!(other.k(), 0, "and nothing moved");
}
