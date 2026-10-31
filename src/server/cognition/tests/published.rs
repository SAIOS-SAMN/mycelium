// 78.151 — THE PUBLISHED READING: the door's reads answered off what the court
// stroke published, the same answers the kernel gives, at the height published.

use crate::anastome::dialogue::{
    decode_receipts_answer, decode_receipts_answer_at, encode_receipts_reply, put_reading_height, ReceiptSection, ReceiptsAnswer,
};
use crate::chronometry::{ActKind, KStepReceipt, Place, ReceiptChain, Stamp};
use crate::cognition::*;
use crate::court::admission::SignedPrefix;
use crate::court::identity::HyphaKey;

const FAR: Place = Place { k: u64::MAX, i: u64::MAX };

/// A court signing as the key `name` derives.
fn court(name: &str) -> (HyphaKey, Court) {
    let key = super::common::court_key(name);
    let mut k = Court::new();
    k.bind_identity(key.vk_bytes());
    (key, k)
}

/// `n` strokes: one act and its orbit's close each (OK1).
fn strokes(k: &mut Court, n: u64) {
    for _ in 0..n {
        k.append_step(ActKind::List, [1; 32], [2; 32]);
        k.orbit();
    }
}

/// A 47.1 position signed by `key` at `head`, over `payload`.
fn position(key: &HyphaKey, head: &KStepReceipt, payload: Vec<u8>) -> ReceiptSection {
    let prefix = SignedPrefix {
        genesis: [7; 32],
        epoch: 2,
        prev_receipt_hash: head.prev_hash(),
        at: head.stamp.place(),
        height: head.stamp.h,
        chart: vec![],
        payload,
    };
    let sig = key.sign(&prefix.bytes().expect("the prefix frames"));
    ReceiptSection { prefix, sig, receipt: head.clone() }
}

/// The fold on two signatures, its proof kept.
fn fold(k: &mut Court, key: &HyphaKey) {
    let p = k.propose_fold().expect("a final prefix");
    let head = k.chain().head().expect("the court acted");
    let home = position(key, &head, p.root.hash.to_vec());
    let wkey = super::common::court_key("published-witness");
    let mut wc = ReceiptChain::with_vk(wkey.vk_bytes());
    let wh = wc.append(ActKind::List, [0; 4], Vec::new(), [0; 32], Vec::new()).expect("appends");
    let witness = position(&wkey, &wh, p.root.hash.to_vec());
    k.commit_fold_proved(&home, &witness).expect("two signatures seal the fold");
}

/// A page, or the root's place where the answer is the root.
fn read(a: ReceiptsAnswer) -> Result<Vec<KStepReceipt>, Place> {
    match a {
        ReceiptsAnswer::Page(p) => Ok(p),
        ReceiptsAnswer::Root(r) => Err(r.root.at),
    }
}

/// The kernel and its published reading answer every ask alike.
fn alike(k: &Court, p: &Published, signer: &[u8; 32], asks: &[(Option<Place>, Place, bool)]) {
    for &(after, upto, replay) in asks {
        assert_eq!(
            read(p.serve_receipts(signer, after, upto, replay, 8, usize::MAX)),
            read(k.serve_receipts(signer, after, upto, replay, 8, usize::MAX)),
            "after {after:?} upto {upto:?} replay {replay}"
        );
    }
}

/// A catch-up below the fold root is answered with the root, one from the
/// root with the hot line, and an era replay with the archive — off the
/// published reading exactly as off the kernel, at the height published.
#[test]
fn publish_serves_receipts_as_kernel_does() {
    let (key, mut b) = court("published-b");
    let vk = key.vk_bytes();
    strokes(&mut b, 4);
    fold(&mut b, &key);
    strokes(&mut b, 3);
    let p = b.publish(None).expect("a first reading always publishes");
    let root = b.chain().root().expect("folded").at;
    assert_eq!(read(p.serve_receipts(&vk, None, FAR, false, 8, usize::MAX)), Err(root), "below the root: the root");
    alike(&b, &p, &vk, &[(None, FAR, false), (Some(root), FAR, false), (None, root, true), (Some(Place::GENESIS), root, true)]);
    assert_eq!(p.height(), b.chain().head_height(), "the reading stands at the court's head");
    assert_eq!(p.head(), b.chain().head_stamp());
}

/// A WRITE IS READ AT THE NEXT PUBLICATION. Acts sealed after a reading was
/// published are not in it: a range past its head answers short at the
/// height it was published at. Published again from it, the reading reaches
/// the head — and only what grew was copied.
#[test]
fn publish_lags_until_next_and_copies_growth() {
    let (key, mut b) = court("published-lag");
    let vk = key.vk_bytes();
    strokes(&mut b, 3);
    let p1 = b.publish(None).expect("published");
    let head1 = b.chain().head_stamp().expect("acted").place();
    assert!(b.publish(Some(&p1)).is_none(), "nothing moved: nothing is published");
    strokes(&mut b, 1);
    let grew = b.chain().line_between(&vk, Some(head1), FAR, usize::MAX);
    assert_eq!(grew.len(), 2, "one stroke: an act and its close");
    assert_eq!(read(p1.serve_receipts(&vk, Some(head1), FAR, false, 8, usize::MAX)), Ok(Vec::new()), "not yet at the new head");
    assert!(p1.height() < b.chain().head_height(), "the reading says how far it reaches");
    let p2 = b.publish(Some(&p1)).expect("the line moved");
    assert_eq!(read(p2.serve_receipts(&vk, Some(head1), FAR, false, 8, usize::MAX)), Ok(grew.clone()), "the next reading reaches the head");
    alike(&b, &p2, &vk, &[(None, FAR, false), (Some(Place::GENESIS), FAR, false)]);
    let tapes: u64 = grew
        .iter()
        .map(|r| {
            let mut t = Vec::new();
            r.write(&mut t).expect("writes");
            t.len() as u64
        })
        .sum();
    let c = p2.cost();
    assert_eq!((c.receipts, c.bytes, c.eras), (2, tapes, 0), "the two receipts that grew, and the receipts' tapes; the six held stay shared");
}

/// A PEER'S LINE, admitted in two pieces, is published as the court holds
/// it: each piece read off the reading as off the kernel.
#[test]
fn publish_serves_peer_line_admitted_in_pieces() {
    let (akey, mut a) = court("published-peer-a");
    let avk = akey.vk_bytes();
    strokes(&mut a, 4);
    let line = a.chain().line_between(&avk, None, FAR, usize::MAX);
    let (first, rest) = line.split_at(3);
    let (_, mut c) = court("published-peer-c");
    strokes(&mut c, 1);
    super::common::admit_line(&mut c, &akey, first);
    let p1 = c.publish(None).expect("published");
    alike(&c, &p1, &avk, &[(None, FAR, false)]);
    super::common::admit_line(&mut c, &akey, rest);
    let p2 = c.publish(Some(&p1)).expect("the peer's line moved");
    alike(&c, &p2, &avk, &[(None, FAR, false), (Some(Place::GENESIS), FAR, false)]);
}

/// A WALLET OFF THE READING is the kernel's wallet at the publication: an
/// unseen party reads zero, a payment made after it is read at the next.
#[test]
fn publish_wallet_matches_kernel_at_publication() {
    let (_, mut k) = court("published-wallet");
    strokes(&mut k, 1);
    let anchor = (Stamp::default(), [0u8; 32]);
    let (human, entity, unseen) = ([7u8; 32], [8u8; 32], [9u8; 32]);
    assert_eq!(k.treasury.genesis(human, [10, 6, 4, 2], anchor), [10, 6, 4, 2]);
    k.treasury.pay(human, entity, [3, 1, 0, 1], anchor).expect("the human pays");
    k.treasury.hold(human, b"lease", [2, 0, 1, 0], anchor).expect("the human holds");
    let p1 = k.publish(None).expect("published");
    for who in [human, entity, unseen] {
        assert_eq!(p1.wallet(&who), k.wallet(&who), "the account published is the kernel's");
    }
    k.treasury.pay(human, entity, [1, 1, 1, 0], anchor).expect("pays again");
    assert_ne!(p1.wallet(&entity), k.wallet(&entity), "a payment after the reading is not in it");
    let p2 = k.publish(Some(&p1)).expect("the accounts moved");
    for who in [human, entity, unseen] {
        assert_eq!(p2.wallet(&who), k.wallet(&who), "read at the next publication");
    }
    assert_eq!(p2.cost().receipts, 0, "no line moved, none copied");
}

/// 78.151 (WIRE-CHANGE) — THE HEIGHT RIDES BEHIND THE ANSWER: a page with
/// the height behind it reads back with it; one without it (a door that
/// writes none) reads back with none; any other tail refuses.
#[test]
fn decode_receipts_answer_at_reads_trailing_height() {
    let (key, mut b) = court("published-wire");
    strokes(&mut b, 2);
    let line = b.chain().line_between(&key.vk_bytes(), None, FAR, usize::MAX);
    let bare = encode_receipts_reply(&line, 1 << 16);
    let mut body = bare.clone();
    put_reading_height(&mut body, 41);
    let Some((ReceiptsAnswer::Page(page), Some(h))) = decode_receipts_answer_at(&body) else { panic!("the page reads back with its height") };
    assert_eq!((page, h), (line.clone(), 41));
    assert!(matches!(decode_receipts_answer_at(&bare), Some((ReceiptsAnswer::Page(_), None))), "no height written, none read");
    assert!(decode_receipts_answer(&body[..body.len() - 1]).is_none(), "a tail that is no height refuses");
}
