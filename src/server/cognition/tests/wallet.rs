// 78.93 B1 — EVERY PARTY HOLDS A WALLET (`Court::wallet`, `dialogue::wallet`):
// a flat per-lane account read off the treasury's carriers, standing at the
// court's head; across courts, summed per lane exactly or not at all.

use crate::anastome::dialogue::wallet::{decode_wallet, decode_wallet_reply, encode_wallet, encode_wallet_reply, Wallet};
use crate::chronometry::Stamp;
use crate::cognition::*;

/// A human's key is allocated at genesis, pays an entity, and sets part
/// aside against a key. Each party's wallet reads exactly what the carriers
/// hold — balance, bond, holds, per lane — an unseen party reads zero on
/// every lane (the reading of absence), and what the two own together is the
/// supply the audit proves. The reading crosses the wire whole, and a wallet
/// across two courts owns the per-lane sum of its accounts.
#[test]
fn wallet_reads_each_party_and_sums_to_supply() {
    let mut k = Court::new();
    k.bind_identity([1; 32]);
    let anchor = (Stamp::default(), [0u8; 32]);
    let (human, entity, unseen) = ([7u8; 32], [8u8; 32], [9u8; 32]);
    // A court comes up holding its own allocation; what this test issues is
    // read against it.
    let before = k.treasury.audit().expect("the ledger conserves");
    assert_eq!(k.treasury.genesis(human, [10, 6, 4, 2], anchor), [10, 6, 4, 2]);
    k.treasury.pay(human, entity, [3, 1, 0, 1], anchor).expect("the human pays");
    k.treasury.hold(human, b"lease", [2, 0, 1, 0], anchor).expect("the human holds");
    let h = k.wallet(&human).expect("measured");
    assert_eq!((h.balance, h.bond, h.held), ([5, 5, 3, 1], [0; 4], [2, 0, 1, 0]));
    let e = k.wallet(&entity).expect("measured");
    assert_eq!((e.balance, e.held), ([3, 1, 0, 1], [0; 4]));
    let u = k.wallet(&unseen).expect("measured");
    assert_eq!(u.owned(), Some([0; 4]), "a party never seen holds nothing");
    let after = k.treasury.audit().expect("the ledger conserves");
    let issued: [u64; 4] = std::array::from_fn(|l| after[l] - before[l]);
    let both = Wallet { who: human, accounts: vec![h, e], unanswered: 0 };
    assert_eq!(both.owned(), Some(issued), "what the parties own is what was issued to them");
    // The wire.
    assert_eq!(decode_wallet(&encode_wallet(&human)), Some(human));
    let reply = encode_wallet_reply(&h);
    assert_eq!(decode_wallet_reply(&reply), Some(h));
    assert!(decode_wallet_reply(&reply[..reply.len() - 1]).is_none(), "a truncated reply is no reply");
    assert_eq!(h.court, [1; 32], "the account names its court");
}
