// 45.2, 45.3 — THE LEDGER IS THE CLOCK: causal order is partial, and an
// extent is replayed from the chain that holds it.

use super::common::speak_pair;
use crate::cognition::*;

/// A2 / 5.2b. Supersession on the dialect book is CAUSAL ORDER (45.3), not
/// a height scalar. `push_cocycle` compared `tu < old_tu`, and two heights
/// from two organs are exactly the quantity 45.3 says is partial: two
/// transitions that are concurrent were ordered by that comparison and one
/// was silently discarded (W1, and rule 15's "when unsure, hold").
///
/// A transition that demonstrably follows the incumbent supersedes it. One
/// that is merely CONCURRENT with it — neither receipt's frontier below the
/// other's — is kept on the chain beside it, and nothing is chosen.
#[test]
fn cocycle_supersedes_by_causal_order_and_keeps_concurrent() {
use crate::transducer::alphabet::{LetterId, Word};
let inv = crate::transducer::stalk::Invariant::counts(&[1]);
// Two words on the sensor plane: one quarter turn and two. Both keep
// the plane closed, so both lift to versors, and they are not the same
// projective element.
let vsr = |n: usize| {
    let q = LetterId::quarter((0, 1)).unwrap();
    let w = Word::from_letters(&vec![(q, false); n]);
    (rotor_versor(&w.rotor()).expect("versor"), w.rotor().class_key().expect("ck"))
};
let (v1, ck1) = vsr(1);
let (v2, ck2) = vsr(2);
assert!(!crate::transducer::element::same_versor(&v1, &v2), "two transitions");

// ORDERED: both stand at receipts on this mind's own chain, so the
// second's frontier dominates the first's and it supersedes.
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1)];
let b = vec![(0, 0, 2), (0, 1, 2)];
speak_pair(&mut k, &a, &b, [1; 32]);
let r0 = k.chain().head_stamp().expect("the mind has acted");
k.push_cocycle_at(&inv, [1; 32], [2; 32], v1.clone(), ck1.clone(), Some(r0));
// A second voice: the first has its one claim in flight on this lane
// (§3.1 as ruled, S13.15), and the act here only needs the mind to move.
speak_pair(&mut k, &a, &[(0, 0, 5), (0, 1, 5)], [2; 32]);
let r1 = k.chain().head_stamp().expect("the mind has acted");
assert_ne!(r0, r1, "the mind moved");
let chain_before = k.dialect_chain_len();
k.push_cocycle_at(&inv, [1; 32], [2; 32], v2.clone(), ck2.clone(), Some(r1));
let standing = k
    .cocycle_at
    .scoped()
    .next()
    .map(|(_, row)| row.gauge.clone())
    .expect("a row stands");
assert!(
    crate::transducer::element::same_versor(&standing, &v2),
    "the later receipt supersedes"
);
assert_eq!(k.dialect_chain_len(), chain_before + 1, "the superseded one is kept");

// CONCURRENT: the second stands at a PEER's receipt, admitted but never
// cited, so neither frontier is below the other. Nothing supersedes.
let mut k = Kernel::new();
speak_pair(&mut k, &a, &b, [1; 32]);
let mine = k.chain().head_stamp().expect("the mind has acted");
k.push_cocycle_at(&inv, [1; 32], [2; 32], v1.clone(), ck1.clone(), Some(mine));

let key = super::common::court_key("peer");
let mut peer = crate::chronometry::ReceiptChain::with_vk(key.vk_bytes());
let pr = peer.append(crate::chronometry::ActKind::List, [0; 4], Vec::new(), [0; 32], Vec::new()).expect("appends").clone();
let pre = crate::court::admission::SignedPrefix {
    genesis: [7; 32],
    epoch: 2,
    prev_receipt_hash: pr.prev_hash(),
    at: pr.stamp.place(),
    height: pr.stamp.h,
    chart: vec![],
    payload: pr.stamp.key(),
};
assert!(matches!(
    k.admit_receipt(([7; 32], 2), super::common::verified(&pre, &key), &pr),
    crate::court::admission::Admitted::Parent(_)
));
let chain_before = k.dialect_chain_len();
k.push_cocycle_at(&inv, [1; 32], [2; 32], v2.clone(), ck2, Some(pr.stamp));
let standing = k
    .cocycle_at
    .scoped()
    .next()
    .map(|(_, row)| row.gauge.clone())
    .expect("a row stands");
assert!(
    crate::transducer::element::same_versor(&standing, &v1),
    "concurrent: the incumbent is not displaced"
);
assert_eq!(
    k.dialect_chain_len(),
    chain_before + 1,
    "and the concurrent one is kept, not dropped"
);
}

/// Standing T is a height difference on the home chain; a witness
/// re-derives it from the receipts (T16).
#[test]
fn extent_replays_from_the_home_chain() {
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1)];
let b = vec![(0, 0, 2), (0, 1, 2)];
speak_pair(&mut k, &a, &b, [1; 32]);
let addr = k.pending.scoped().next().expect("listed").1.address();
// The install height is the admitting act's, read off its stamp (78.24).
let h0 = k.admitted_at(addr.as_bytes()).expect("the List act admitted it").h;
assert_eq!(k.t_extent(addr.as_bytes(), h0), Some(0), "extent is zero at first vantage");
assert_eq!(
    k.t_extent(addr.as_bytes(), h0 + 7),
    Some(7),
    "extent is h_now − h_install on the home chain"
);
let mut k2 = Kernel::new();
k2.standing_h.insert(&addr, 4);
assert_eq!(
    k2.t_extent(addr.as_bytes(), 3),
    None,
    "a height below the stamp is a count that went backwards, not an extent of zero"
);
assert_eq!(
    k2.t_extent(addr.as_bytes(), 11),
    Some(7),
    "a witness re-derives the same extent from the recorded height"
);
}

/// A court with one provisional word standing, ruled by two voices. The
/// fixture 45.1's four acts each need, built through the constructors the
/// production code exports (§3.4).
fn docketed(me: [u8; 32]) -> (Kernel, crate::transducer::address::Address) {
    let mut k = Kernel::new();
    k.bind_identity(me);
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 0, 4), (0, 1, 4), (1, 0, 4)]; // the L recoloured whole: one body, one word (20 as ruled, L7)
    assert_eq!(speak_pair(&mut k, &a, &b, [1; 32]), Outcome::GermPending);
    assert_eq!(speak_pair(&mut k, &a, &b, [2; 32]), Outcome::GermPending);
    k.court_stroke();
    let addr = k.provisional.scoped().next().unwrap().0;
    (k, crate::transducer::address::Address::from(addr))
}

/// 45.1: A RULING IS AN ADMITTED ACT, so it is a node on this court's own
/// line — an act of its open orbit, `K` moving only where the orbit closes
/// (OK1) — and a receipt in its chain. It sealed none: the book admitted the
/// word to the ruled cover, or evaporated its award, or excised it as a
/// cavity, at no position in the chain that is supposed to record it.
#[test]
fn a_ruling_seals_a_receipt_at_the_word_it_ruled() {
    use crate::anastome::dialogue::RULING_UPHELD;
    let (mut k, addr) = docketed([0x5a; 32]);
    let before = k.chain().own_len();
    assert!(k.apply_verdict(&addr, RULING_UPHELD), "the ruling lands");
    assert_eq!(k.chain().own_len(), before + 1, "a ruling is one act on the court's line");
    let at = k.admitted_at(addr.as_bytes()).expect("the ruled word keeps its admitting stamp");
    assert_eq!(
        k.chain().head().map(|r| r.cites.iter().map(|c| c.at()).collect::<Vec<_>>()),
        Some(vec![Some(at)]),
        "and the node cites the word it ruled on, by the stamp that admitted it"
    );
}

/// And a ruling that rules NOTHING seals nothing: both refusals return
/// before the provisional is removed, which is the one point past which
/// the book has moved on every path.
#[test]
fn a_ruling_that_rules_nothing_seals_nothing() {
    use crate::anastome::dialogue::RULING_UPHELD;
    let (mut k, addr) = docketed([0x5a; 32]);
    assert!(k.apply_verdict(&addr, RULING_UPHELD), "the first lands");
    let held = k.chain().own_len();
    assert!(!k.apply_verdict(&addr, RULING_UPHELD), "a verdict lands once");
    assert!(!k.apply_verdict(&addr, 0xfe), "an unknown ruling rules nothing");
    assert_eq!(k.chain().own_len(), held, "neither sealed an act");
}

/// 45.1: a slash is a ruling on the other arm, and seals its own receipt.
#[test]
fn a_slash_seals_a_receipt() {
    use crate::anastome::dialogue::RULING_SLASHED;
    let (mut k, addr) = docketed([0x5b; 32]);
    let before = k.chain().own_len();
    k.apply_verdict(&addr, RULING_SLASHED);
    assert_eq!(k.chain().own_len(), before + 1, "a slash is one step too");
}

/// 45.1: "a syllabus page served" is one of the acts the chapter names.
/// The door wrote `receipt K … h …` beside every page it served — a
/// receipt nothing sealed, so the K it named was whatever the last
/// unrelated act had left there.
#[test]
fn a_syllabus_page_served_seals_a_receipt() {
    let (mut k, addr) = docketed([0x5a; 32]);
    let before = k.chain().own_len();
    k.note_served(&addr, [3; 32]);
    assert_eq!(k.chain().own_len(), before + 1, "a page served is one step");
}

/// 45.1, 47.3: an install on a second vantage seals a receipt, at the
/// installed address. A book that cannot say where an install stands
/// cannot re-derive it.
#[test]
fn an_install_seals_a_receipt_at_the_installed_address() {
    let (mut k, addr) = docketed([0x5c; 32]);
    let before = k.chain().own_len();
    k.home_install(&addr, [0xbb; 32]);
    assert_eq!(k.chain().own_len(), before + 1, "an install is one step");
    let at = k.admitted_at(addr.as_bytes()).expect("the installed word keeps its admitting stamp");
    assert!(
        k.chain().head().is_some_and(|r| r.cites.iter().any(|c| c.at() == Some(at))),
        "the node cites the installed word by its admitting stamp"
    );
}

/// 45.1: a lapsed citation releases an escrow and unlists a draw, so it is
/// this court's own act, sealed after the orbit close that lapsed it by
/// zero flux (78.37 (3)).
#[test]
fn a_lapsed_citation_seals_a_receipt() {
    let mut k = Kernel::new();
    k.bind_identity([0x5d; 32]);
    let a = vec![(0, 0, 1), (0, 1, 1), (1, 0, 1)];
    let b = vec![(0, 0, 4), (0, 1, 4), (1, 0, 4)]; // the L recoloured whole: one body, one word (20 as ruled, L7)
    assert_eq!(speak_pair(&mut k, &a, &b, [1; 32]), Outcome::GermPending);
    let laddr = k.pending.scoped().next().unwrap().0;
    let at = k.admitted_at(&laddr).expect("the listing was admitted by its List act");
    k.orbit();
    let elsewhere = crate::transducer::address::Address::from_bytes([crate::transducer::address::TAG_FORM, 7]);
    k.note_served(&elsewhere, [3; 32]);
    let before = k.chain().own_len();
    k.orbit();
    assert!(!k.pending.contains(&laddr), "a whole orbit with no flux lapsed it");
    assert_eq!(k.chain().own_len(), before + 2, "the orbit's close, and the lapse as its own step");
    assert!(
        k.chain().head().is_some_and(|r| r.cites.iter().any(|c| c.at() == Some(at))),
        "the node cites the listing whose draw was released, read before it left (78.24 (2c))"
    );
}

/// ACT1 — A COURT ON THE CLOCK (78.21, 78.27(1), EC1). The reading is its
/// head's stamp and `K`, where its snapshot stands, per peer line the last
/// height admitted and the gap to its head, and the three counts: an act
/// sealed on a word nothing measures is counted as sealed without a frame,
/// and a word held with no act is counted held without a stamp, by row.
#[test]
fn a_court_reads_itself_on_the_clock() {
use crate::transducer::element::from_canonical;
use crate::transducer::germinate::germinate;
let mut k = Kernel::new();
let a = vec![(0, 0, 1), (0, 1, 1)];
let b = vec![(0, 0, 2), (0, 1, 2)];
speak_pair(&mut k, &a, &b, [1; 32]);
let c = k.clock(Some(([9; 32], 2)));
let head = k.chain().head().expect("the court has acted");
let [s, g, t, h] = head.mu;
assert_eq!(c.head, [head.stamp.h, s, g, t, h, head.stamp.k, head.stamp.i], "the head's stamp, h:S:G:T:H:K.i");
assert_eq!(c.k, k.chain().next_step_index());
assert_eq!(c.genesis, Some(([9; 32], 2)), "the origin it adopted, carried as given");
assert_eq!((c.persisted, c.folded), (None, None), "never persisted, never folded");
assert_eq!(c.unstamped, [0; 5], "every word the percept held, an act admitted");
// R1: the List act names the listing, which the listing book holds and
// measures — its frame is read there, never counted as missing.
assert_eq!(c.unframed, 0, "every act the percept sealed named a word a row holds");

// An act naming a word no row holds reads its frame as nothing.
k.append_step(crate::chronometry::ActKind::Attest, [0xEE; 40], [1; 32]);
assert_eq!(k.clock(None).unframed, 1, "sealed without a frame, counted at seal");

// A word placed on the basis with no act is held without a stamp.
k.basis_insert(from_canonical(&germinate(&[(0, 0, 1u8), (0, 1, 2)], &[(0, 0, 1u8), (0, 1, 9)]).unwrap()), [7; 32]);
assert_eq!(k.clock(None).unstamped, [1, 0, 0, 0, 0], "named by its row");

// A snapshot noted where it was taken, once it landed.
let at = k.snapshot_at();
let _ = k.snapshot();
k.note_persisted(at);
assert_eq!(k.clock(None).persisted, Some(at));

// A peer's line: the last height admitted from it, and the gap.
let key = super::common::court_key("peer");
let mut peer = crate::chronometry::ReceiptChain::with_vk(key.vk_bytes());
let pr = peer.append(crate::chronometry::ActKind::List, [0; 4], Vec::new(), [0; 32], Vec::new()).expect("appends").clone();
let pre = crate::court::admission::SignedPrefix {
    genesis: [7; 32],
    epoch: 2,
    prev_receipt_hash: pr.prev_hash(),
    at: pr.stamp.place(),
    height: pr.stamp.h,
    chart: vec![],
    payload: pr.stamp.key(),
};
assert!(matches!(
    k.admit_receipt(([7; 32], 2), super::common::verified(&pre, &key), &pr),
    crate::court::admission::Admitted::Parent(_)
));
let c = k.clock(None);
let own_h = i64::try_from(c.head[0]).expect("a height");
assert_eq!(c.peers, vec![(key.vk_bytes(), pr.stamp.h, own_h - i64::try_from(pr.stamp.h).expect("a height"))]);
assert_eq!(c.rulings, [[0; 4]; 4], "a List is no ruling");
}
