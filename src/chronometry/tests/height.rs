// 45.2 — height is the mesh's, not this mind's: it climbs on the receipts
// a step cites.

use crate::chronometry::*;

#[test]
fn height_is_one_plus_max_of_prev_and_parents() {
    let mut a = ReceiptChain::with_vk([1; 32]);
    a.append(ActKind::List, [0; 4], [1u8], [9; 32], Vec::new()).expect("genesis");
    assert_eq!(a.head_height(), 0, "h(genesis) = 0");
    a.append(ActKind::List, [0; 4], [2u8], [9; 32], Vec::new()).expect("second");
    assert_eq!(a.head_height(), 1, "h = 1 + h(prev)");
    let mut b = ReceiptChain::with_vk([2; 32]);
    b.append(ActKind::List, [0; 4], [1u8], [9; 32], Vec::new()).expect("b genesis");
    b.append(ActKind::List, [0; 4], [2u8], [9; 32], Vec::new()).expect("b second");
    b.append(ActKind::List, [0; 4], [3u8], [9; 32], Vec::new()).expect("b third");
    assert_eq!(b.head_height(), 2);
    let parent = Parent {
        vk: [2; 32],
        k: b.head_k(),
        hash: b.head_hash(),
    };
    a.admit(b.head().expect("b has sealed a receipt").clone());
    a.append(ActKind::List, [0; 4], [3u8], [9; 32], vec![parent]).expect("cite b");
    assert_eq!(
        a.head_height(),
        1 + 2,
        "h = 1 + max(h(prev)=1, h(parent)=2)"
    );
}

/// EVERY RECEIPT THE CHAIN HOLDS IS FOUND BY ITS HASH — its own and the
/// ones it admitted alike — and one it does not hold is unmeasured. The
/// admitted set was a keyed read and this chain's own receipts were the
/// `receipts.iter().find(…)` behind it, so re-deriving the height of an
/// own receipt walked the whole chain (rule 10): once per admission
/// (47.2 step 4) and once per frontier visit.
#[test]
fn a_chains_own_receipts_are_found_by_hash_like_the_ones_it_admitted() {
    let mut peer = ReceiptChain::with_vk([2; 32]);
    peer.append(ActKind::List, [0; 4], [1u8], [9; 32], Vec::new()).expect("peer genesis");
    let mut c = ReceiptChain::with_vk([1; 32]);
    c.append(ActKind::List, [0; 4], [1u8], [9; 32], Vec::new()).expect("genesis");
    c.append(ActKind::List, [0; 4], [2u8], [9; 32], Vec::new()).expect("second");
    c.admit(peer.receipts()[0].clone());
    for r in c.receipts() {
        assert_eq!(c.height_of(&r.chain_hash), Some(r.height), "its own, by hash");
    }
    assert_eq!(c.height_of(&peer.head_hash()), Some(0), "and one it admitted");
    assert_eq!(c.height_of(&[0xab; 32]), None, "a hash it holds neither way is unmeasured");
}
