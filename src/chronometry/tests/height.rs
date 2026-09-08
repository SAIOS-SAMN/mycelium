// 45.2 — height is the mesh's, not this mind's: it climbs on the receipts
// a step cites.

use crate::chronometry::*;

#[test]
fn height_is_one_plus_max_of_prev_and_parents() {
    let mut a = ReceiptChain::with_vk([1; 32]);
    a.append([1u8], [9; 32], Vec::new()).expect("genesis");
    assert_eq!(a.head_height(), 0, "h(genesis) = 0");
    a.append([2u8], [9; 32], Vec::new()).expect("second");
    assert_eq!(a.head_height(), 1, "h = 1 + h(prev)");
    let mut b = ReceiptChain::with_vk([2; 32]);
    b.append([1u8], [9; 32], Vec::new()).expect("b genesis");
    b.append([2u8], [9; 32], Vec::new()).expect("b second");
    b.append([3u8], [9; 32], Vec::new()).expect("b third");
    assert_eq!(b.head_height(), 2);
    let parent = Parent {
        vk: [2; 32],
        k: b.head_k(),
        hash: b.head_hash(),
    };
    a.admit(b.receipts().last().unwrap().clone());
    a.append([3u8], [9; 32], vec![parent]).expect("cite b");
    assert_eq!(
        a.head_height(),
        1 + 2,
        "h = 1 + max(h(prev)=1, h(parent)=2)"
    );
}
