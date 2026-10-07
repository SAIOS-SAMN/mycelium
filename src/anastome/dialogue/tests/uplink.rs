// 78.119 — ACT_UPLINK's wire: a signed fold and the court's answers read back
// whole; a byte left over is no fold.

use crate::anastome::dialogue::uplink::{decode_uplink, decode_uplink_reply, encode_uplink, encode_uplink_reply, Discontinuous, TierFold, Uplinked};
use crate::chronometry::Stamp;

#[test]
fn a_signed_fold_and_every_answer_read_back_whole() {
    let key = ed25519_dalek::SigningKey::from_bytes(&[5u8; 32]);
    let fold = TierFold {
        tier: key.verifying_key().to_bytes(),
        prefix: [2u8; 32],
        rows: vec![([1u8; 32], [3, -1, 0, 7]), ([4u8; 32], [-3, 1, 0, -7])],
        declared: vec![([8u8; 32], [0, 0, 0, 0])],
    };
    let signed = fold.sign(&key);
    let mut asked = encode_uplink(&signed);
    assert_eq!(decode_uplink(&asked), Some(signed));
    asked.push(0);
    assert_eq!(decode_uplink(&asked), None, "a byte left over is no fold");
    let at = Stamp { h: 3, vk: [6u8; 32], k: 2, i: 1 };
    for u in [
        Uplinked::Admitted { at, receipt: [9u8; 32] },
        Uplinked::Unsigned,
        Uplinked::Discontinuous(Discontinuous::Unordered),
        Uplinked::Discontinuous(Discontinuous::Overflow { lane: 2 }),
        Uplinked::Discontinuous(Discontinuous::Unbalanced { lane: 3 }),
        Uplinked::LineRefused,
    ] {
        assert_eq!(decode_uplink_reply(&encode_uplink_reply(&u)), Some(u.clone()), "{u:?}");
    }
}
