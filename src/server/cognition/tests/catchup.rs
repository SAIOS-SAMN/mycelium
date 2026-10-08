// 78.147 — CATCH-UP FROM A FOLD ROOT: courts in one process, one far behind
// another's fold, read on the replication and serving counters.

use crate::anastome::dialogue::{
    decode_receipts_answer, encode_fold_root_reply, ReceiptSection, ReceiptsAnswer, RootProof,
};
use crate::chronometry::{ActKind, KStepReceipt, Parent, Place, ReceiptChain};
use crate::cognition::*;
use crate::court::admission::{Admitted, SignedPrefix};
use crate::court::identity::HyphaKey;
use crate::observe::{rep, serves, ServeAxis};
use crate::server::backfill::{Ask, Origin};
use std::sync::Mutex;

const ORIGIN: Origin = Some(([7; 32], 2));
const AT: ([u8; 32], u32) = ([7; 32], 2);

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

/// A court signing as the key `name` derives.
fn court(name: &str) -> (HyphaKey, Kernel) {
    let key = super::common::court_key(name);
    let mut k = Kernel::new();
    k.bind_identity(key.vk_bytes());
    (key, k)
}

/// `n` strokes: one act and its orbit's close each (OK1). Peer receipts
/// admitted since the last are cited by the act.
fn strokes(k: &mut Kernel, n: u64) {
    for _ in 0..n {
        k.append_step(ActKind::List, [1; 32], [2; 32]);
        k.orbit();
    }
}

/// THE FOLD ON TWO SIGNATURES, the proof kept: the home's position over the
/// root hash and a witness's, the witness another key.
fn fold(k: &mut Kernel, key: &HyphaKey) -> RootProof {
    let p = k.propose_fold().expect("a final prefix");
    let head = k.chain().head().expect("the court acted");
    let home = position(key, &head, p.root.hash.to_vec());
    let wkey = super::common::court_key("catchup-witness");
    let mut wc = ReceiptChain::with_vk(wkey.vk_bytes());
    let wh = wc.append(ActKind::List, [0; 4], Vec::new(), [0; 32], Vec::new()).expect("appends");
    let witness = position(&wkey, &wh, p.root.hash.to_vec());
    k.commit_fold_proved(&home, &witness).expect("two signatures seal the fold");
    k.root_proof(&key.vk_bytes()).expect("the root is held with its proof").clone()
}

/// A court's head and the prefix its digest signs for it.
fn frame(k: &Kernel) -> (KStepReceipt, SignedPrefix) {
    let r = k.chain().head().expect("the court acted");
    let pre = SignedPrefix {
        genesis: [7; 32],
        epoch: 2,
        prev_receipt_hash: r.prev_hash(),
        at: r.stamp.place(),
        height: r.stamp.h,
        chart: vec![],
        payload: r.stamp.key(),
    };
    (r, pre)
}

/// A court's own line between two places, as the court reads it.
fn line(k: &Kernel, after: Option<Place>) -> Vec<KStepReceipt> {
    let me = k.chain().vk();
    k.chain().line_between(&me, after, Place { k: u64::MAX, i: u64::MAX }, usize::MAX)
}

/// One signer's axes on the replication reading.
fn axes(vk: [u8; 32]) -> [u64; crate::observe::REP_AXES] {
    let r = crate::observe::replication();
    r.signers.iter().find(|(s, _)| *s == vk).map(|(_, a)| *a).expect("the signer is read")
}

/// The page source over courts in this process: each signer's own court
/// answers its line through the serving door, eight receipts a page.
fn source<'a>(
    courts: &'a [&'a Kernel],
    asked: &'a Mutex<Vec<([u8; 32], Option<Place>, Ask)>>,
) -> impl FnMut(&[u8; 32], Option<Place>, Place, Ask) -> Option<ReceiptsAnswer> + 'a {
    move |who: &[u8; 32], after: Option<Place>, upto: Place, ask: Ask| {
        if let Ok(mut a) = asked.lock() {
            a.push((*who, after, ask));
        }
        let k = courts.iter().find(|k| k.chain().vk() == *who)?;
        Some(k.serve_receipts(who, after, upto, ask == Ask::Replay, 8, usize::MAX))
    }
}

/// 78.147 (1) — A COURT FAR BEHIND A SIGNER'S FOLD ADOPTS ITS ROOT AND CITES
/// ITS HEAD. Court c heard b's first stroke; b then acted on, folded on two
/// signatures, and acted past the root. c hears b's head: the gap's catch-up
/// opens below b's root and is answered with the root, which c adopts on its
/// two signatures; b's line is paged on from it, hot, and b's head is
/// admitted as a parent — c's next act stands above it. Nothing below the
/// root is asked again, and no era is paged to answer.
#[test]
fn a_court_behind_a_fold_adopts_the_root_and_cites_the_head() {
    let (bkey, mut b) = court("catchup-b");
    let bvk = bkey.vk_bytes();
    strokes(&mut b, 1);
    let early = line(&b, None);
    strokes(&mut b, 5);
    let proof = fold(&mut b, &bkey);
    strokes(&mut b, 3);
    let mut c0 = Kernel::new();
    c0.append_step(ActKind::Attest, [0xAA; 40], [1; 32]);
    let mut budget = c0.stroke();
    assert_eq!(c0.admit_backfill(&early, &mut budget), (early.len(), None), "c holds b's first stroke");
    let held = c0.known_head(&bvk).map(|(at, _)| at).expect("held");
    assert!(held < proof.root.at, "c's head of b stands below b's root");
    let c = Mutex::new(c0);
    let asked = Mutex::new(Vec::new());
    let before = serves();
    let (head, pre) = frame(&b);
    let courts = [&b];
    let heard: Result<_, ()> = crate::server::backfill::hear(
        &c,
        ORIGIN,
        || Ok(c.lock().expect("c").admit_receipt(AT, super::common::verified(&pre, &bkey), &head)),
        || c.lock().ok().map(|mut k| k.stroke()),
        source(&courts, &asked),
    );
    assert!(matches!(heard, Ok(Admitted::Parent(_))), "b's head is cited: {heard:?}");
    let after = serves();
    assert!(after[ServeAxis::ServedRoot as usize] > before[ServeAxis::ServedRoot as usize], "the catch-up was answered with the root");
    assert!(axes(bvk)[rep::ROOT_ADOPTED] > 0, "the root adopted, counted under b");
    let asks = asked.into_inner().expect("asks");
    let below: Vec<_> = asks.iter().filter(|(_, at, ask)| *ask == Ask::CatchUp && at.is_none_or(|a| a < proof.root.at)).collect();
    assert_eq!(below.len(), 1, "one catch-up opened below the root — the one the root answered: {asks:?}");
    assert!(asks.iter().all(|(_, _, ask)| *ask == Ask::CatchUp), "no era replayed for a catch-up");
    let mut c = c.into_inner().expect("c");
    assert_eq!(c.known_head(&bvk).map(|(at, _)| at), Some(head.stamp.place()), "c holds b's line to its head");
    c.append_step(ActKind::Attest, [0xAB; 40], [1; 32]);
    assert!(c.chain().head_height() > head.stamp.h, "c's act cites b's head and stands above it");
    assert!(c.chain().verify_chain_integrity(), "c's own line re-verifies");
}

/// 78.147 (2) — THE SERVING DOOR ANSWERS BELOW ITS ROOT WITH THE ROOT. A
/// catch-up from genesis is answered with the root and its two signatures —
/// no page, so no era is read for it — and the answer rides the wire whole.
/// From the root the hot line answers; only an era replay pages the archive.
/// A court whose fold kept no proof answers below its root off the hot line
/// alone, and counts it. A proof with a broken signature, at another origin,
/// or over another root does not stand.
#[test]
#[expect(clippy::disallowed_methods, reason = "rule 15 — an era replay is read in its own order, and its first act is the era's opening: the order is content")]
fn a_catch_up_below_the_root_is_answered_with_the_root_not_cold_pages() {
    let (bkey, mut b) = court("catchup-serve-b");
    let bvk = bkey.vk_bytes();
    strokes(&mut b, 4);
    let proof = fold(&mut b, &bkey);
    strokes(&mut b, 2);
    let far = Place { k: u64::MAX, i: u64::MAX };
    let before = serves();
    let answer = b.serve_receipts(&bvk, None, far, false, 8, usize::MAX);
    let mid = serves();
    let ReceiptsAnswer::Root(root) = answer else { panic!("a catch-up below the root is answered with the root") };
    assert_eq!(root.parent(), proof.parent(), "b's own root");
    assert!(mid[ServeAxis::ServedRoot as usize] > before[ServeAxis::ServedRoot as usize], "counted as a served root");
    let body = encode_fold_root_reply(&root).expect("the proof frames");
    let Some(ReceiptsAnswer::Root(back)) = decode_receipts_answer(&body) else { panic!("the root answer reads back") };
    assert_eq!((back.parent(), back.verify(AT)), (proof.parent(), Ok(())), "whole, and it verifies at the origin");
    assert!(crate::anastome::dialogue::decode_receipts_reply(&body).is_none(), "a reader of pages alone reads no page in it");
    // From the root: the hot line.
    let ReceiptsAnswer::Page(hot) = b.serve_receipts(&bvk, Some(proof.root.at), far, false, 8, usize::MAX) else {
        panic!("a catch-up from the root is a page")
    };
    assert!(!hot.is_empty() && hot.iter().all(|r| r.stamp.place() > proof.root.at), "hot, past the root");
    // An era replay pages the archive.
    let ReceiptsAnswer::Page(era) = b.serve_receipts(&bvk, None, proof.root.at, true, 8, usize::MAX) else {
        panic!("a replay is a page")
    };
    assert!(era.first().is_some_and(|r| r.stamp.place() == Place::GENESIS), "the replay reads the era from its first act");
    assert!(serves()[ServeAxis::Replays as usize] > mid[ServeAxis::Replays as usize], "counted as a replay");
    // A fold with no proof kept: hot alone below its root, counted.
    let (_, mut u) = court("catchup-unproved");
    let uvk = u.chain().vk();
    strokes(&mut u, 3);
    let p = u.propose_fold().expect("a final prefix");
    let wkey = super::common::court_key("catchup-witness");
    let mut wc = ReceiptChain::with_vk(wkey.vk_bytes());
    let wh = wc.append(ActKind::List, [0; 4], Vec::new(), [0; 32], Vec::new()).expect("appends");
    u.commit_fold([3; 64], &position(&wkey, &wh, p.root.hash.to_vec())).expect("sealed");
    strokes(&mut u, 1);
    let unproved = serves()[ServeAxis::UnprovedRoot as usize];
    let ReceiptsAnswer::Page(page) = u.serve_receipts(&uvk, None, far, false, 8, usize::MAX) else { panic!("no proof: a page") };
    assert!(page.iter().all(|r| r.stamp.place() > p.root.at), "nothing below the root is read for a catch-up");
    assert!(serves()[ServeAxis::UnprovedRoot as usize] > unproved, "and the unproved root is counted");
    // Proofs that do not stand.
    let mut forged = proof.clone();
    forged.witness.sig[0] ^= 1;
    assert!(forged.verify(AT).is_err(), "a broken witness signature");
    assert!(proof.verify(([8; 32], 2)).is_err(), "another origin");
    let mut moved = proof.clone();
    moved.root.hash[0] ^= 1;
    assert!(moved.verify(AT).is_err(), "another root");
    let mut own = proof.clone();
    own.witness = own.home.clone();
    assert_eq!(own.verify(AT), Err(crate::verdict::REFUSE_WITNESS_DEPENDENT), "a home is not its own witness");
}

/// 78.147 (3) — A CITATION INTO A HELD ERA IS PROVED, NOT CHASED. Court c
/// stands on b's adopted root; d's head cites b's first act, inside the era
/// that root closes. The citation is proved by one era replay walked to the
/// root, and d's head is cited — b's line is never asked as a catch-up below
/// the root it holds.
#[test]
fn a_citation_into_a_held_era_is_proved_not_chased() {
    let (bkey, mut b) = court("catchup-era-b");
    let bvk = bkey.vk_bytes();
    strokes(&mut b, 3);
    let b0 = b.chain().at_place(Place::GENESIS).expect("b's first act");
    let proof = fold(&mut b, &bkey);
    strokes(&mut b, 1);
    let mut c0 = Kernel::new();
    c0.append_step(ActKind::Attest, [0xAA; 40], [1; 32]);
    assert_eq!(c0.adopt_root(AT, &proof), RootAdoption::Adopted, "c stands on b's root");
    assert_eq!(c0.adopt_root(AT, &proof), RootAdoption::Held, "and adopts it once");
    assert_eq!(c0.known_head(&bvk).map(|(at, _)| at), Some(proof.root.at), "b's line stands at the root here");
    let c = Mutex::new(c0);
    let dkey = super::common::court_key("catchup-era-d");
    let dvk = dkey.vk_bytes();
    let mut dc = ReceiptChain::with_vk(dvk);
    dc.admit(b0.clone());
    dc.append(ActKind::List, [0; 4], Vec::new(), [0; 32], vec![Parent { at: b0.stamp, hash: b0.chain_hash }]).expect("d cites b0");
    let (head, pre) = {
        let r = dc.head().expect("d acted");
        let pre = SignedPrefix {
            genesis: [7; 32],
            epoch: 2,
            prev_receipt_hash: r.prev_hash(),
            at: r.stamp.place(),
            height: r.stamp.h,
            chart: vec![],
            payload: r.stamp.key(),
        };
        (r, pre)
    };
    let asked = Mutex::new(Vec::new());
    let courts = [&b];
    let proved = axes(bvk)[rep::FOLDED_PROVED];
    let heard: Result<_, ()> = crate::server::backfill::hear(
        &c,
        ORIGIN,
        || Ok(c.lock().expect("c").admit_receipt(AT, super::common::verified(&pre, &dkey), &head)),
        || c.lock().ok().map(|mut k| k.stroke()),
        source(&courts, &asked),
    );
    assert!(matches!(heard, Ok(Admitted::Parent(_))), "d's head is cited: {heard:?}");
    assert!(axes(bvk)[rep::FOLDED_PROVED] > proved, "the citation proved by its era");
    let asks = asked.into_inner().expect("asks");
    assert!(asks.iter().all(|(who, _, ask)| *who != bvk || *ask == Ask::Replay), "b's line asked only as the era's replay: {asks:?}");
}

/// 78.147 (4) — CROSS-CITING LINES BOTH ADVANCE FROM THEIR ROOTS. Courts a
/// and b cite each other's acts, fold on two signatures each, and cite each
/// other past their roots. Court c heard only their first strokes. Hearing
/// a's head, then b's, c adopts both roots — the citations a's hot line makes
/// of b's are caught up from b's root in the same hearing — and cites both
/// heads.
#[test]
#[expect(clippy::disallowed_methods, reason = "rule 15 — a line is read in its own order, and its last receipt is the head the hearing resumes after: the order is content")]
fn cross_citing_lines_both_advance_from_their_roots() {
    let (akey, mut a) = court("catchup-cross-a");
    let (bkey, mut b) = court("catchup-cross-b");
    let (avk, bvk) = (akey.vk_bytes(), bkey.vk_bytes());
    strokes(&mut a, 1);
    strokes(&mut b, 1);
    let mut c0 = Kernel::new();
    c0.append_step(ActKind::Attest, [0xAA; 40], [1; 32]);
    super::common::admit_line(&mut c0, &akey, &line(&a, None));
    super::common::admit_line(&mut c0, &bkey, &line(&b, None));
    // Each hears the other's line and cites it, twice over.
    let mut heard_a: Option<Place> = None;
    let mut heard_b: Option<Place> = None;
    let cross = |a: &mut Kernel, b: &mut Kernel, heard_a: &mut Option<Place>, heard_b: &mut Option<Place>| {
        let from_a = line(a, *heard_a);
        super::common::admit_line(b, &akey, &from_a);
        *heard_a = from_a.last().map(|r| r.stamp.place()).or(*heard_a);
        strokes(b, 1);
        let from_b = line(b, *heard_b);
        super::common::admit_line(a, &bkey, &from_b);
        *heard_b = from_b.last().map(|r| r.stamp.place()).or(*heard_b);
        strokes(a, 1);
    };
    for _ in 0..2 {
        cross(&mut a, &mut b, &mut heard_a, &mut heard_b);
    }
    // b hears a's last stroke, so each holds the other's line whole; then
    // both fold, and what each reads of the other opens at the root.
    let rest = line(&a, heard_a);
    super::common::admit_line(&mut b, &akey, &rest);
    heard_a = rest.last().map(|r| r.stamp.place()).or(heard_a);
    let ra = fold(&mut a, &akey);
    let rb = fold(&mut b, &bkey);
    assert_eq!((heard_a, heard_b), (Some(ra.root.at), Some(rb.root.at)), "each heard the other up to its root");
    for _ in 0..2 {
        cross(&mut a, &mut b, &mut heard_a, &mut heard_b);
    }
    // b answers a's last stroke, so neither head cites the other's.
    super::common::admit_line(&mut b, &akey, &line(&a, heard_a));
    strokes(&mut b, 1);
    let c = Mutex::new(c0);
    let courts = [&a, &b];
    for (key, k, root) in [(&akey, &a, &ra), (&bkey, &b, &rb)] {
        let asked = Mutex::new(Vec::new());
        let (head, pre) = frame(k);
        let heard: Result<_, ()> = crate::server::backfill::hear(
            &c,
            ORIGIN,
            || Ok(c.lock().expect("c").admit_receipt(AT, super::common::verified(&pre, key), &head)),
            || c.lock().ok().map(|mut k| k.stroke()),
            source(&courts, &asked),
        );
        assert!(matches!(heard, Ok(Admitted::Parent(_))), "the head is cited: {heard:?}");
        let held = c.lock().expect("c").known_head(&k.chain().vk()).map(|(at, _)| at);
        assert!(held.is_some_and(|h| h > root.root.at), "the line advanced past its root");
    }
    assert!(axes(avk)[rep::ROOT_ADOPTED] > 0 && axes(bvk)[rep::ROOT_ADOPTED] > 0, "both roots adopted");
    let mut c = c.into_inner().expect("c");
    c.append_step(ActKind::Attest, [0xAB; 40], [1; 32]);
    let top = a.chain().head_height().max(b.chain().head_height());
    assert!(c.chain().head_height() > top, "c's act cites both heads");
}
