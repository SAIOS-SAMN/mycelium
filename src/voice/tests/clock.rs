// 45.1, 45.4 — the voice's clock is its own receipt chain: one receipt per
// admitted act, naming it, and no reply moves it.

use crate::anastome::dialogue;
use crate::chronometry::ActKind;
use crate::voice::Voice;

/// 45.1, 45.4 — A VOICE'S CLOCK IS ITS OWN RECEIPT CHAIN. It seats with an
/// empty chain and seals one receipt per admitted act — a form stood, an
/// utterance, a page learned — each naming what was admitted under this
/// voice's key and linked to the one before; its K is the chain's length.
/// Nothing sets it, so a reply carrying another kernel's k-step cannot
/// move it, and a request seals nothing. It was a bare count (I25).
#[test]
fn a_voice_seals_one_receipt_per_admitted_act_and_its_k_is_the_chain() {
    let mut v = Voice::named("worker-a");
    assert_eq!(v.identity(), Voice::named("worker-a").identity());
    assert_ne!(v.identity(), Voice::named("worker-b").identity());
    assert_eq!(v.k(), 0, "a voice that has spoken nothing accrues nothing");
    assert!(v.chain().receipts().is_empty(), "and holds no receipt");

    assert_eq!(v.seal(ActKind::Stood, b"a stood form"), 1, "one admitted act is one receipt");
    assert_eq!(v.seal(ActKind::Uttered, b"an utterance"), 2);
    assert_eq!(v.seal(ActKind::Learned, b"a learned page"), 3);
    assert_eq!(v.k(), 3, "K is the chain's length");

    let rs = v.chain().receipts();
    assert_eq!(rs.len(), 3);
    assert_eq!(rs[0].delta, b"a stood form".to_vec(), "the receipt names what was admitted");
    assert_eq!(rs[0].vk, v.identity(), "under this voice's key");
    assert_eq!(rs[0].step_index, 0);
    assert_eq!(rs[1].prev_receipt_hash, rs[0].chain_hash, "and links to its predecessor");
    assert_ne!(rs[0].operator_hash, rs[1].operator_hash, "the act is on the receipt");
    assert!(v.chain().verify_chain_integrity(), "the chain re-derives end to end");

    assert_eq!(v.quote()[0], dialogue::ACT_QUOTE);
    assert_eq!(v.k(), 3, "a request is not an admitted act and seals nothing");
}
