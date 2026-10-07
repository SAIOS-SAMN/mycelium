// 78.119 — ACT_UPLINK's wire: a signed fold and the court's answers read back
// whole; a byte left over is no fold.

use crate::anastome::dialogue::uplink::{
    decode_uplink, decode_uplink_reply, encode_uplink, encode_uplink_reply, ChildNet, ChildRefusal, Discontinuous, TierFold, Uplinked,
};
use crate::chronometry::Stamp;

#[test]
fn a_signed_fold_and_every_answer_read_back_whole() {
    let key = ed25519_dalek::SigningKey::from_bytes(&[5u8; 32]);
    let fold = TierFold {
        tier: key.verifying_key().to_bytes(),
        prefix: [2u8; 32],
        rows: vec![([1u8; 32], [3, -1, 0, 7]), ([4u8; 32], [-3, 1, 0, -7])],
        declared: vec![([8u8; 32], [0, 0, 0, 0])],
        children: Vec::new(),
    };
    // 78.123 B — a fold with no children writes exactly as 78.119's: its
    // bytes end at the signature, and it signs no children's commitment.
    let single = 1 + 32 + 32 + fold.rows_bytes().len() + 64;
    assert_eq!(fold.children_hash(), None);
    let signed = fold.clone().sign(&key);
    let mut asked = encode_uplink(&signed);
    assert_eq!(asked.len(), single, "a single-level fold's bytes unchanged");
    assert_eq!(decode_uplink(&asked), Some(signed));
    asked.push(0);
    assert_eq!(decode_uplink(&asked), None, "a byte left over is no fold");
    let at = Stamp { h: 3, vk: [6u8; 32], k: 2, i: 1 };
    // A chained fold — its children after the signature — read back whole.
    let child = ChildNet {
        at,
        rows: vec![([1u8; 32], [2, 0, 0, 0])],
        declared: vec![([1u8; 32], [2, 0, 0, 0])],
        included_rows: vec![([1u8; 32], [1, 0, 0, 0])],
        included_declared: vec![([1u8; 32], [1, 0, 0, 0])],
        rate: [(1, 1), (3, 2), (1, 1), (1, 1)],
    };
    let chained = TierFold { children: vec![child], ..fold }.sign(&key);
    let asked = encode_uplink(&chained);
    assert!(asked.len() > single, "its children ride after the signature");
    assert_eq!(decode_uplink(&asked), Some(chained));
    for u in [
        Uplinked::Admitted { at, receipt: [9u8; 32] },
        Uplinked::Unsigned,
        Uplinked::Discontinuous(Discontinuous::Unordered),
        Uplinked::Discontinuous(Discontinuous::Overflow { lane: 2 }),
        Uplinked::Discontinuous(Discontinuous::Unbalanced { lane: 3 }),
        Uplinked::Discontinuous(Discontinuous::Rate { lane: 0 }),
        Uplinked::LineRefused,
        Uplinked::Unbound,
        Uplinked::Chained { at, receipt: [9u8; 32], provisional: true, obstructions: 2, open_remainders: 1 },
        Uplinked::ChildRefused { index: 3, why: ChildRefusal::NotOnLine },
        Uplinked::ChildRefused { index: 0, why: ChildRefusal::Mismatch },
        Uplinked::ChildRefused { index: 1, why: ChildRefusal::ForeignDomain },
        Uplinked::ChildRefused { index: 2, why: ChildRefusal::OverIncluded { lane: 1 } },
    ] {
        assert_eq!(decode_uplink_reply(&encode_uplink_reply(&u)), Some(u.clone()), "{u:?}");
    }
}
