// 45.8, 47.3 — THE ERA: a fold proposed, witnessed and committed on two
// signatures; what it archives, what it refuses, the chain that continues
// over its root, and the K-range read across the archive.

use crate::anastome::dialogue::ReceiptSection;
use crate::chronometry::{ActKind, Place, ReceiptChain};
use crate::cognition::*;
use crate::court::admission::SignedPrefix;
use crate::verdict::{
    REFUSE_FOLD_INEXACT, REFUSE_FOLD_OPEN, REFUSE_RECEIPT, REFUSE_SIGNATURE, REFUSE_WITNESS_DEPENDENT,
    REFUSE_WITNESS_ROOM,
};

/// A witness's signed position over a root hash, as a fixture: its own
/// chain of one receipt and a 47.1 prefix whose payload is the hash,
/// signed by the key its name derives.
fn co_sign(name: &str, root_hash: [u8; 32]) -> ReceiptSection {
    let key = super::common::court_key(name);
    let mut c = ReceiptChain::with_vk(key.vk_bytes());
    let head = c.append(ActKind::List, [0; 4], Vec::new(), [0; 32], Vec::new()).expect("appends").clone();
    let prefix = SignedPrefix {
        genesis: [7; 32],
        epoch: 2,
        prev_receipt_hash: head.prev_hash(),
        at: head.stamp.place(),
        height: head.stamp.h,
        chart: vec![],
        payload: root_hash.to_vec(),
    };
    let sig = key.sign(&prefix.bytes().expect("the prefix frames"));
    ReceiptSection { prefix, sig, receipt: head }
}

/// Three sealed acts and nothing open propose a fold; the witness
/// re-derives the prefix it fetched and refuses a wrong net, a short
/// prefix, or its own missing bond; the home seals only on a signature
/// that verifies, from another key, over this root; then the prefix is
/// cold under its root with both signatures, the `Fold` receipt cites the
/// root and continues K and height, the ledger conserves, the snapshot
/// carries it, the witness holds the same era under the home's key, and a
/// second fold folds the fold receipt itself. A listing open at the first
/// receipt proposes nothing; a ledger that does not conserve proposes
/// nothing.
#[test]
fn commit_fold_seals_on_two_signatures() {
// Three strokes, each one act and its close (OK1): orbit `n` holds its act
// at `(n, 0)` and its close at `(n, 1)`. A fold cuts only at a close.
let act = |k: u64| Place { k, i: 0 };
let close = |k: u64| Place { k, i: 1 };
let home = [0x5c; 32];
let mut k = Kernel::new();
k.bind_identity(home);
assert_eq!(k.propose_fold(), Err(REFUSE_FOLD_OPEN), "no receipt, nothing to fold");
k.append_step(ActKind::List, [1; 32], [2; 32]);
assert_eq!(k.propose_fold(), Err(REFUSE_FOLD_OPEN), "an orbit not yet closed is no cut");
k.orbit();
for _ in 0..2 {
    k.append_step(ActKind::List, [1; 32], [2; 32]);
    k.orbit();
}
let head_hash = k.chain().head_hash();
let head_height = k.chain().head_height();
let before = k.treasury().audit().expect("conserves before");
let p = k.propose_fold().expect("a final prefix proposes");
assert_eq!((p.after, p.root.at, p.root.hash, p.root.height), (None, close(2), head_hash, head_height));
assert!(p.epochs.iter().any(|(who, _)| *who == [0x11; 32]), "the fixture hypha's genesis nets");
assert_eq!(k.fold_proposal(), Some(&p), "held for the exchange to carry");

// THE WITNESS re-derives the prefix it fetched.
let receipts = k.receipts_range(&home, p.after, p.root.at, 10);
let rows = k.treasury().rows_below(p.root.at);
let witness = Kernel::new();
assert_eq!(witness.verify_fold(home, &p, &receipts, &rows), Ok(()), "the prefix re-derives");
let mut wrong = p.clone();
wrong.epochs.clear();
assert_eq!(witness.verify_fold(home, &wrong, &receipts, &rows), Err(REFUSE_FOLD_INEXACT), "a net that is not the rows'");
assert_eq!(witness.verify_fold(home, &p, &receipts[..2], &rows), Err(REFUSE_RECEIPT), "a prefix that does not reach the root");
assert_eq!(Kernel::boot().verify_fold(home, &p, &receipts, &rows), Err(REFUSE_WITNESS_ROOM), "a witness without bond");

// THE HOME seals only on a signature that verifies, from another key, over this root.
let home_sig = [3u8; 64];
let mut dependent = co_sign("w", p.root.hash);
dependent.receipt.stamp.vk = home;
assert_eq!(k.commit_fold(home_sig, &dependent), Err(REFUSE_WITNESS_DEPENDENT), "a court is not its own witness");
let mut forged = co_sign("w", p.root.hash);
forged.sig[0] ^= 1;
assert_eq!(k.commit_fold(home_sig, &forged), Err(REFUSE_SIGNATURE));
assert_eq!(k.commit_fold(home_sig, &co_sign("w", [9; 32])), Err(REFUSE_RECEIPT), "a position over another root");
assert!(k.fold_proposal().is_some(), "the proposal stands through a witness's refusals");
let w = co_sign("w", p.root.hash);
let done = k.commit_fold(home_sig, &w).expect("two signatures seal the fold");
assert_eq!((done.first, done.last, done.at, done.witness), (act(0), close(2), Some(act(3)), w.receipt.stamp.vk));
assert_eq!(done.h, head_height + 1, "height continues over the fold");
assert!(k.fold_proposal().is_none(), "sealed, nothing pends");
let root = k.chain().root().expect("the chain stands on its root");
assert_eq!(root, p.root);
// 78.92, 78.94 — WHY STILL ANSWERS WHAT A FOLD ARCHIVED: the act is gone
// from the hot chain and stands in the era, where it is read by its place.
assert!(k.chain().at_place(act(1)).is_none(), "the fold took it off the hot chain");
let archived = k.why(crate::anastome::dialogue::why::WhyAsk::Own(act(1))).expect("an archived act is still asked about");
assert_eq!((archived.kind, archived.at.place()), (ActKind::List, act(1)));
let fold = k.chain().opening().expect("the hot line opens at the fold").clone();
assert_eq!((fold.kind, fold.stamp.k, fold.prev_hash()), (ActKind::Fold, 3, head_hash));
assert_eq!(
    fold.cites,
    vec![crate::chronometry::Cite::Root {
        at: crate::chronometry::Stamp::new(root.height, home, root.at),
        hash: head_hash
    }],
    "the fold node cites the archived head by its stamp and hash"
);
assert!(k.chain().verify_chain_integrity(), "the hot chain re-verifies from its root");
assert_eq!(k.k_step(), 3, "K continues past the fold: the fold receipt opens orbit 3");
let era = &k.archive()[0];
assert_eq!(
    (era.root, era.signer, era.witness, era.sig_home, era.sig_witness),
    (root, home, w.receipt.stamp.vk, home_sig, w.sig),
    "the era carries the witness and both signatures"
);
assert_eq!((era.receipts.len(), era.first_place()), (6, act(0)));
assert!(ReceiptChain::from_receipts(era.receipts.clone()).is_some(), "the archived prefix re-verifies as a chain of its own");
assert_eq!(k.treasury().audit().expect("conserves after"), before);
assert_eq!(
    k.receipts_range(&home, None, act(3), 10).iter().map(|r| r.stamp.place()).collect::<Vec<_>>(),
    vec![act(0), close(0), act(1), close(1), act(2), close(2), act(3)],
    "the place-range read spans ARC1 and the hot chain"
);

let snap = k.snapshot();
let mut back = Kernel::new();
assert!(back.restore(&snap), "the snapshot carries the root and the era");
assert_eq!(back.chain().root(), Some(root));
assert_eq!(back.archive(), k.archive());
assert!(back.chain().verify_chain_integrity());

// THE WITNESS holds the same era under the home's key and serves it.
let mut held = Kernel::new();
held.hold_era(era.clone());
assert_eq!(held.receipts_range(&home, None, close(2), 10), era.receipts, "a witnessed era is read under its signer's key");
assert_eq!(held.rows_range(&home, None, act(9), 0, 10).0, 0, "another signer's rows are not this court's to serve");

assert_eq!(k.propose_fold(), Err(REFUSE_FOLD_OPEN), "the fold receipt's orbit is open: no cut yet");
k.orbit();
// 78.67 (A), 78.166 2a: the close asks, and a prefix of the fold and its
// close alone moves no work — its cut falls past it, where no close stands
// yet — so nothing is proposed, and a fold never feeds the next one.
assert!(k.propose_fold_at_close().is_err(), "a fold's own orbit proposes nothing");
assert!(k.fold_proposal().is_none(), "the close proposed nothing");
let p2 = k.propose_fold().expect("the fold receipt's orbit closed, and is itself final");
assert_eq!((p2.after, p2.root.at), (Some(close(2)), close(3)));
assert!(k.commit_fold(home_sig, &co_sign("w", p2.root.hash)).is_ok());
assert_eq!(k.archive().len(), 2);
assert_eq!(k.chain().root().map(|r| r.at), Some(close(3)));

let mut open = Kernel::new();
let a = vec![(0, 0, 1u8), (0, 1, 1)];
let b = vec![(0, 0, 2u8), (0, 1, 2)];
assert_eq!(super::common::speak_pair(&mut open, &a, &b, [7; 32]), Outcome::GermPending);
assert_eq!(open.final_prefix_head(), None, "no orbit has closed: no cut yet");
assert_eq!(open.propose_fold(), Err(REFUSE_FOLD_OPEN));
assert!(open.chain().root().is_none() && open.archive().is_empty(), "nothing was archived");

let mut broken = Kernel::new();
broken.append_step(ActKind::List, [1; 32], [2; 32]);
broken.orbit();
assert!(broken.treasury.credit([9; 32], [1, 0, 0, 0]), "a credit with room lands");
assert_eq!(broken.propose_fold(), Err(REFUSE_FOLD_INEXACT), "a ledger that does not conserve proposes nothing");
assert!(broken.chain().root().is_none() && broken.fold_proposal().is_none());
}

/// THE COLD SEGMENT KEEPS EACH ERA IN ITS OWN SPAN. Two folds filed into a
/// real segment are each read back as its own receipts, and a court that
/// binds the segment again holds both eras from the eras' descriptors alone,
/// each answering its own range.
#[test]
fn bind_eras_reads_each_era_own_range() {
    let close = |k: u64| Place { k, i: 1 };
    let home = [0x5c; 32];
    let path = std::env::temp_dir().join(format!("mycelium-eras-{}-{}", std::process::id(), line!()));
    let _ = std::fs::remove_file(&path);
    let mut k = Kernel::new();
    k.bind_identity(home);
    k.bind_eras(path.clone()).expect("a new segment binds");
    for _ in 0..3 {
        k.append_step(ActKind::List, [1; 32], [2; 32]);
        k.orbit();
    }
    let p = k.propose_fold().expect("a final prefix");
    k.commit_fold([3; 64], &co_sign("w", p.root.hash)).expect("sealed");
    k.orbit();
    let p2 = k.propose_fold().expect("the second fold");
    k.commit_fold([3; 64], &co_sign("w", p2.root.hash)).expect("sealed");
    let [first, second] = k.archive() else { panic!("two eras") };
    assert!(first.at.is_some() && second.at.is_some() && first.at != second.at, "each at its own offset");
    let read = |k: &Kernel, after, upto| k.receipts_range(&home, after, upto, 100).iter().map(|r| r.stamp.place()).collect::<Vec<_>>();
    let one = read(&k, None, close(2));
    let two = read(&k, Some(close(2)), close(3));
    assert!(one.contains(&close(2)) && !one.contains(&close(3)), "the first era answers its own range");
    assert!(two.contains(&close(3)) && !two.contains(&close(2)), "the second era answers its own range, not the first's");
    // The snapshot carries no filed era: the segment does.
    let mut back = Kernel::new();
    assert!(back.restore(&k.snapshot()));
    assert!(back.archive().is_empty(), "no filed era in the snapshot");
    back.bind_eras(path.clone()).expect("the segment binds");
    assert_eq!(back.archive(), k.archive(), "the eras held from the eras' descriptors");
    assert_eq!((read(&back, None, close(2)), read(&back, Some(close(2)), close(3))), (one, two), "each era answers its own range");
    let _ = std::fs::remove_file(&path);
}

/// 2026-10-08 (rulings 6a/6b) — A FOLDED LINE READ FROM ITS LAST FOLD
/// ROOT PAGES NO ARCHIVE, AND THE SERVING COURT SAYS WHAT IT PAGED. Three
/// folds archive the line's early orbits. Read from genesis, the page spends
/// the cold archive (the serving reading counts the eras); read from the
/// start of the last fold root's orbit — where `--console --ledger` now opens —
/// it pages at most the one era that root closes, and every receipt it
/// serves stands at or past that orbit.
#[test]
fn receipts_page_from_root_pages_no_earlier_era() {
    use crate::observe::{serves, ServeAxis};
    let home = [0x6d; 32];
    let mut k = Kernel::new();
    k.bind_identity(home);
    for _ in 0..3 {
        k.append_step(ActKind::List, [1; 32], [2; 32]);
        k.orbit();
    }
    for _ in 0..3 {
        let p = k.propose_fold().expect("a final prefix");
        k.commit_fold([3; 64], &co_sign("w-fold-root", p.root.hash)).expect("sealed");
        k.append_step(ActKind::List, [1; 32], [2; 32]);
        k.orbit();
    }
    assert_eq!(k.archive().len(), 3, "three eras archived");
    let root_k = k.chain().root().expect("the line stands on its last root").at.k;
    let upto = Place { k: u64::MAX, i: u64::MAX };
    let before = serves();
    let whole = k.receipts_page(&home, None, upto, 1000, usize::MAX);
    let mid = serves();
    assert!(whole.iter().any(|r| r.stamp.k < root_k), "from genesis the page reaches below the root");
    assert!(mid[ServeAxis::Cold as usize] > before[ServeAxis::Cold as usize], "a cold page is counted");
    assert!(mid[ServeAxis::ErasPaged as usize] >= before[ServeAxis::ErasPaged as usize] + 3, "every era is paged");
    let from_root = root_k.checked_sub(1).map(|k| Place { k, i: u64::MAX });
    let hot = k.receipts_page(&home, from_root, upto, 1000, usize::MAX);
    assert!(!hot.is_empty(), "the line from its root answers");
    assert!(hot.iter().all(|r| r.stamp.k >= root_k), "nothing below the root's orbit is served");
    let opening = from_root.expect("three folds: the root's orbit is past genesis");
    assert!(k.archive().iter().filter(|e| e.first_place() > opening).count() <= 1, "at most the one era the root closes lies past the opening");
}

/// THE LINE AND THE SNAPSHOT ON DISK AGREE. A court bound to a line writes
/// it before the snapshot that names where it stood; a court binding the
/// same line restores that snapshot to the same head, root and eras; a line
/// that ran ahead of an older snapshot — folded past the stamp it named —
/// still restores it; and a snapshot naming a stamp its line does not hold
/// refuses.
#[test]
fn restore_agrees_with_bound_line() {
    let home = [0x5c; 32];
    let dir = std::env::temp_dir().join(format!("mycelium-line-{}-{}", std::process::id(), line!()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a home");
    let (line, eras) = (dir.join("line"), dir.join("eras"));
    let bound = |k: &mut Kernel, line: &std::path::Path| {
        k.bind_line(line.to_path_buf()).expect("the line binds");
        k.bind_eras(eras.clone()).expect("the segment binds");
    };
    let persist = |k: &mut Kernel| -> Vec<u8> {
        if let Some((path, w)) = k.take_line().expect("the line writes") {
            assert!(w.land(&path), "the line lands");
        }
        k.snapshot()
    };
    let mut k = Kernel::new();
    bound(&mut k, &line);
    k.bind_identity(home);
    for _ in 0..3 {
        k.append_step(ActKind::List, [1; 32], [2; 32]);
        k.orbit();
    }
    let early = persist(&mut k);
    let p = k.propose_fold().expect("a final prefix");
    k.commit_fold([3; 64], &co_sign("w", p.root.hash)).expect("sealed");
    k.append_step(ActKind::List, [1; 32], [2; 32]);
    k.orbit();
    let snap = persist(&mut k);

    let mut back = Kernel::new();
    bound(&mut back, &line);
    assert!(back.restore(&snap), "the snapshot restores on its line");
    assert_eq!((back.chain().head_stamp(), back.chain().root()), (k.chain().head_stamp(), k.chain().root()));
    assert_eq!(back.chain().nodes().collect::<Vec<_>>(), k.chain().nodes().collect::<Vec<_>>());
    assert_eq!(back.archive(), k.archive(), "and the eras from the segment");

    let mut ahead = Kernel::new();
    bound(&mut ahead, &line);
    assert!(ahead.restore(&early), "a line folded past the stamp an older snapshot named restores it");

    let mut lost = Kernel::new();
    bound(&mut lost, &dir.join("no-line"));
    assert!(!lost.restore(&snap), "a snapshot whose line does not hold its stamp refuses");
    let _ = std::fs::remove_dir_all(&dir);
}

/// A PEER RECEIPT CITING A RECEIPT THIS COURT FOLDED IS ADMITTED. The
/// court's own receipts leave its nodes for the archive when it folds; a
/// peer that heard one before the fold still cites it. The citation is read
/// back from the era by its stamp and hash, and the peer's line is admitted
/// — before, admission read the folded receipt as missing for good, and
/// every peer line citing it stopped at this court.
#[test]
fn admit_backfill_admits_citation_of_folded_receipt() {
    let home = [0x5c; 32];
    let path = std::env::temp_dir().join(format!("mycelium-eras-{}-{}", std::process::id(), line!()));
    let _ = std::fs::remove_file(&path);
    let mut k = Kernel::new();
    k.bind_identity(home);
    k.bind_eras(path.clone()).expect("a new segment binds");
    for _ in 0..3 {
        k.append_step(ActKind::List, [1; 32], [2; 32]);
        k.orbit();
    }
    // The peer hears the court's receipt at (1, 0) before the fold.
    let cited = k.receipts_range(&home, Some(Place { k: 0, i: 1 }), Place { k: 1, i: 0 }, 1).remove(0);
    let p = k.propose_fold().expect("a final prefix");
    k.commit_fold([3; 64], &co_sign("w", p.root.hash)).expect("sealed");
    assert!(k.chain().at(&home, cited.stamp.place()).is_none(), "the cited receipt left the nodes for the archive");
    let dvk = super::common::court_key("folded-cite-d").vk_bytes();
    let mut d = ReceiptChain::with_vk(dvk);
    d.admit(cited.clone());
    d.append(ActKind::List, [0; 4], Vec::new(), [0; 32], vec![crate::chronometry::Parent { at: cited.stamp, hash: cited.chain_hash }])
        .expect("d cites the court's receipt");
    let line: Vec<_> = d.line_between(&dvk, None, d.head().expect("d acted").stamp.place(), 16);
    let mut budget = k.stroke();
    assert_eq!(k.admit_backfill(&line, &mut budget), (line.len(), None), "the folded citation is read back and d's line admitted");
    let _ = std::fs::remove_file(&path);
}

/// A SECOND FOLD VERIFIES ON THE ROWS A WITNESS FETCHES. The proposer nets
/// every row below its head — the first fold's epoch rows and all since —
/// and a witness pages the rows it verifies with through `rows_range`
/// (ACT_ROWS). The first fold's witness read the whole journal; from the
/// second on, the span began at the prefix the fold stands on, and every
/// court refused its peers' folds as not conserving (epoch 115, thousands
/// an hour).
#[test]
fn verify_fold_second_fold_on_fetched_rows() {
    let home = [0x5c; 32];
    let mut k = Kernel::new();
    k.bind_identity(home);
    for _ in 0..3 {
        k.append_step(ActKind::List, [1; 32], [2; 32]);
        k.orbit();
    }
    let p = k.propose_fold().expect("a final prefix");
    let witness = Kernel::new();
    let fetched = |k: &Kernel, p: &Proposal| k.rows_range(&home, p.after, p.root.at, 0, usize::MAX).1;
    let first = k.receipts_range(&home, p.after, p.root.at, 100);
    assert_eq!(witness.verify_fold(home, &p, &first, &fetched(&k, &p)), Ok(()), "the first fold verifies");
    k.commit_fold([3; 64], &co_sign("w", p.root.hash)).expect("sealed");
    k.orbit();
    let p2 = k.propose_fold().expect("the second fold");
    let second = k.receipts_range(&home, p2.after, p2.root.at, 100);
    assert_eq!(fetched(&k, &p2), k.treasury().rows_below(p2.root.at), "the witness is served exactly the rows the proposal nets over");
    assert_eq!(witness.verify_fold(home, &p2, &second, &fetched(&k, &p2)), Ok(()), "the second fold verifies on the fetched rows");
}

/// 78.65 — THE EPOCH ROWS STAND AT THE ROOT THEY NET TO. A court proposes a
/// fold, and before its witness co-signs it pays out of a balance the fold
/// will net and closes an orbit. The epoch rows, standing at the root, sit
/// before that payment in the journal, so the next fold cut at that close
/// nets the payer from what it held — at the fold node, above the payment,
/// they were outside the cut and the payer read below zero.
#[test]
fn commit_fold_stands_epoch_rows_at_root() {
    let home = [0x11; 32];
    let mut k = Kernel::new();
    k.bind_identity(home);
    k.append_step(ActKind::List, [1; 32], [2; 32]);
    k.orbit();
    let p = k.propose_fold().expect("a final prefix");
    // Between the proposal and its co-signature: a payment out of the
    // funded identity, and a close.
    k.append_step(ActKind::List, [1; 32], [2; 32]);
    let paid_at = k.anchor();
    let _ = k.treasury.settle_use(home, [0x22; 32], [1, 0, 0, 0], paid_at);
    k.orbit();
    k.commit_fold([3; 64], &co_sign("w", p.root.hash)).expect("sealed");
    let root = k.chain().root().expect("folded");
    assert!(
        k.treasury().entries().iter().filter(|e| e.kind == crate::economy::Kind::Epoch).all(|e| e.stands_at() == Some(root.at)),
        "the epoch rows stand at the root"
    );
    let p2 = k.propose_fold().expect("the next cut nets exactly");
    assert!(p2.root.at > root.at && p2.root.at < k.chain().head_place().expect("a head"), "it cuts between the payment and the fold node");
    assert_eq!(crate::economy::nets_read(&k.treasury().rows_below(p2.root.at)).map(|n| !n.rows.is_empty()), Ok(true));
}

/// 78.65 — FINALITY IS PER ITEM. A listing, a provisional word and an
/// escrow open below the cut hold nothing back: the line folds past them.
/// The word is still read by its stamp, now in the era: it lapses after the
/// fold, the lapse citing it. The escrow is a claim off the ledger: it is
/// slashed after the fold, the slash standing at the act that slashed it.
/// The ledger conserves throughout, and the next fold re-derives on its
/// witness's rows.
#[test]
fn propose_fold_passes_open_listing_and_escrow() {
    // The fixture's bonded identity, which the escrow names.
    let home = [0x11; 32];
    let mut k = Kernel::new();
    k.bind_identity(home);
    let a = vec![(0, 0, 1u8), (0, 1, 1)];
    let b = vec![(0, 0, 2u8), (0, 1, 2)];
    assert_eq!(super::common::speak_pair(&mut k, &a, &b, [7; 32]), Outcome::GermPending);
    let word: Vec<u8> = k.provisional.scoped().map(|(a, _)| a).next().expect("a provisional word");
    let admitted = k.admitted_at(&word).expect("the word stands at its admitting act");
    assert!(k.pending.contains(&word), "and is listed");
    // An escrow on it, standing at the act that escrowed it, voiced by a
    // bonded identity so a slash debits something.
    let escrowed_at = k.anchor();
    k.treasury.escrow_award(&crate::transducer::address::Address::from(word.clone()), vec![home], [1, 0, 0, 0], escrowed_at);
    // One close: a cut, and not yet a whole orbit with nothing flowing to
    // the word, which would lapse it (78.37 (3)).
    k.append_step(ActKind::List, [1; 32], [2; 32]);
    k.orbit();

    let p = k.propose_fold().expect("nothing open pins the line");
    assert!(p.root.at >= admitted.place(), "the fold passes the word's admitting act");
    k.commit_fold([3; 64], &co_sign("w", p.root.hash)).expect("sealed");
    assert!(k.chain().node(&admitted).is_none(), "the admitting act is in the era now");
    assert!(k.pending.contains(&word) && k.provisional.contains(&word), "and the word is still open");

    // The escrow closes after the fold, at the act that closes it.
    k.append_step(ActKind::List, [1; 32], [2; 32]);
    let slashed_at = k.anchor();
    let r = k
        .treasury
        .revoke_award(&word, crate::economy::escrow::Revocation::Slash(slashed_at))
        .expect("the escrow was there");
    assert!(!r.slashed.is_empty(), "the bond is debited");
    assert!(
        k.treasury().entries().iter().filter(|e| e.kind == crate::economy::Kind::Slash).all(|e| (e.at, e.receipt) == slashed_at),
        "the slash stands at the act that slashed, past the fold"
    );
    // The word lapses after the fold, cited by the stamp the era holds.
    k.lapse_word(&word);
    assert!(!k.pending.contains(&word) && !k.provisional.contains(&word), "the word left every row");
    assert!(k.treasury().audit().is_ok(), "the ledger conserves through all of it");

    k.orbit();
    let p2 = k.propose_fold().expect("the line folds again");
    let fetched = k.rows_range(&home, p2.after, p2.root.at, 0, usize::MAX).1;
    let line = k.chain().vk();
    let receipts = k.receipts_range(&line, p2.after, p2.root.at, 100);
    assert!(!receipts.is_empty(), "the witness is served the prefix");
    assert_eq!(Kernel::new().verify_fold(line, &p2, &receipts, &fetched), Ok(()), "and its witness re-derives it");
    k.commit_fold([3; 64], &co_sign("w", p2.root.hash)).expect("sealed");
    assert!(k.treasury().audit().is_ok());
}

/// 78.166 2a — AFTER A FOLD, AN ORBIT LONGER THAN A PAGE FOLDS AT ITS CLOSE.
/// The line past a fresh root opens with the fold and its own orbit's close;
/// the orbit after them holds more acts than one witness page carries. The
/// last close on the page was that fold's own, its prefix carried no work,
/// and every close read idle while the root stood still (hypha b). The cut
/// now falls past the page: nothing is proposed while the long orbit is open
/// (no fold feeds the next), and at its close the fold is proposed there.
#[test]
fn propose_fold_at_close_cuts_long_orbit_at_close() {
    let home = [0x5d; 32];
    let mut k = Kernel::new();
    k.bind_identity(home);
    for _ in 0..3 {
        k.append_step(ActKind::List, [1; 32], [2; 32]);
        k.orbit();
    }
    let p = k.propose_fold().expect("a final prefix proposes");
    k.commit_fold([3; 64], &co_sign("w", p.root.hash)).expect("two signatures seal the fold");
    k.orbit();
    let (at_most, _) = Kernel::receipts_frame().expect("a frame");
    // The long orbit changes the algorithm, as b's did (15 670 installs): a
    // generator enters the presentation, then more than a page of lists.
    k.append_step(ActKind::Install, [1; 32], [2; 32]);
    for _ in 0..=at_most {
        k.append_step(ActKind::List, [1; 32], [2; 32]);
    }
    assert_eq!(k.propose_fold_at_close(), Err(REFUSE_FOLD_OPEN), "the long orbit is open: nothing to cut, nothing proposed");
    let closed = match k.orbit() {
        crate::chronometry::Close::Sealed(at) => at.place(),
        other => panic!("the long orbit closes: {other:?}"),
    };
    let proposed = k.propose_fold_at_close().expect("the long orbit's close carries work: the fold is proposed");
    assert_eq!(proposed.root.at, closed, "cut at the long orbit's own close, past the page");
}
