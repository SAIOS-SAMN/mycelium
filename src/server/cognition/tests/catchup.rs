// 78.147 — CATCH-UP FROM A FOLD ROOT: courts in one process, one far behind
// another's fold, read on the replication and serving counters, and a gap
// past a court's capacity closed through the root (MU-HOSTING §5a.9).

use crate::anastome::dialogue::{
    decode_receipts_answer, encode_fold_root_reply, ReceiptSection, ReceiptsAnswer, RootProof,
};
use crate::chronometry::{ActKind, KStepReceipt, Parent, Place, ReceiptChain};
use crate::cognition::*;
use crate::court::admission::{Admitted, Predecessor, SignedPrefix};
use crate::court::identity::HyphaKey;
use crate::observe::{replication, serves, ServeAxis};
use crate::server::backfill::{AdoptedOrigin, Ask};
use std::sync::Mutex;

const ORIGIN: AdoptedOrigin =Some(([7; 32], 2));
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

/// Another chain hash than `h`: its first byte moved.
fn another(h: Predecessor) -> Predecessor {
    let mut b = h.bytes();
    b[0] ^= 1;
    Predecessor::of_bytes(b)
}

/// A court signing as the key `name` derives.
fn court(name: &str) -> (HyphaKey, Court) {
    let key = super::fixtures::court_key(name);
    let mut k = Court::new();
    k.bind_identity(key.verifying_bytes());
    (key, k)
}

/// `n` strokes: one act and its orbit's close each (OK1). Peer receipts
/// admitted since the last are cited by the act.
fn strokes(k: &mut Court, n: u64) {
    for _ in 0..n {
        k.append_step(ActKind::List, [1; 32], [2; 32]);
        k.orbit();
    }
}

/// THE FOLD ON TWO SIGNATURES, the proof kept: the home's position over the
/// root written out and a witness's, the witness another key.
fn fold(k: &mut Court, key: &HyphaKey) -> RootProof {
    let p = k.propose_fold().expect("a final prefix");
    let head = k.chain().head().expect("the court acted");
    let home = position(key, &head, p.root.signed());
    let wkey = super::fixtures::court_key("catchup-witness");
    let mut wc = ReceiptChain::with_signer(wkey.verifying_bytes());
    let wh = wc.append(ActKind::List, [0; 4], Vec::new(), [0; 32], Vec::new()).expect("appends");
    let witness = position(&wkey, &wh, p.root.signed());
    k.commit_fold_proved(&home, &witness).expect("two signatures seal the fold");
    k.root_proof(&key.verifying_bytes()).expect("the root is held with its proof").clone()
}

/// A court's head and the prefix its digest signs for it.
fn frame(k: &Court) -> (KStepReceipt, SignedPrefix) {
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
fn line(k: &Court, after: Option<Place>) -> Vec<KStepReceipt> {
    let own_vk = k.chain().signer();
    k.chain().line_between(&own_vk, after, Place { k: u64::MAX, i: u64::MAX }, usize::MAX)
}

/// One signer's axes on the replication reading.
fn axes(signer: [u8; 32]) -> [u64; crate::observe::REP_AXES] {
    let r = crate::observe::replication();
    r.signers.iter().find(|(s, _)| *s == signer).map(|(_, a)| *a).expect("the signer is read")
}

/// Every ask the source served: the signer, the place after which, the ask.
type Asked = Mutex<Vec<([u8; 32], Option<Place>, Ask)>>;

/// The page source over courts in this process: each signer's own court
/// answers its line through the serving door, eight receipts a page.
fn source<'a>(
    courts: &'a [&'a Court],
    asked: &'a Asked,
) -> impl FnMut(&[u8; 32], Option<Place>, Place, Ask) -> Option<ReceiptsAnswer> + 'a {
    move |signer: &[u8; 32], after: Option<Place>, upto: Place, ask: Ask| {
        if let Ok(mut a) = asked.lock() {
            a.push((*signer, after, ask));
        }
        let k = courts.iter().find(|k| k.chain().signer() == *signer)?;
        Some(k.serve_receipts(signer, after, upto, ask == Ask::Era, 8, usize::MAX))
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
fn hear_adopts_root_and_cites_head_behind_fold() {
    let (bkey, mut b) = court("catchup-b");
    let bvk = bkey.verifying_bytes();
    strokes(&mut b, 1);
    let early = line(&b, None);
    strokes(&mut b, 5);
    let proof = fold(&mut b, &bkey);
    strokes(&mut b, 3);
    let mut c0 = Court::new();
    c0.append_step(ActKind::Attest, [0xAA; 40], [1; 32]);
    let mut budget = c0.stroke();
    let (n, missing) = c0.admit_backfill(&super::fixtures::page(&early), &mut budget);
    assert!(n == early.len() && missing.is_none(), "c holds b's first stroke");
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
        || Ok(c.lock().expect("c").admit_receipt(AT, super::fixtures::verified(&pre, &bkey), &super::fixtures::reconstructed(&head))),
        || c.lock().ok().map(|mut k| k.stroke()),
        source(&courts, &asked),
    );
    assert!(matches!(heard, Ok(Admitted::Parent(_))), "b's head is cited: {heard:?}");
    let after = serves();
    assert!(after[ServeAxis::ServedRoot as usize] > before[ServeAxis::ServedRoot as usize], "the catch-up was answered with the root");
    assert!(axes(bvk)[replication::ROOT_ADOPTED] > 0, "the root adopted, counted under b");
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
/// A court whose fold kept no proof answers below its root from the archive,
/// so the page links to the asker's head, and counts it. A proof with a broken signature, at another origin,
/// or over another root does not stand.
#[test]
#[expect(clippy::disallowed_methods, reason = "rule 15 — an era replay is read in its own order, and its first act is the era's opening: the order is content")]
fn serve_receipts_answers_below_root_with_root() {
    let (bkey, mut b) = court("catchup-serve-b");
    let bvk = bkey.verifying_bytes();
    strokes(&mut b, 4);
    let proof = fold(&mut b, &bkey);
    strokes(&mut b, 2);
    let far = Place { k: u64::MAX, i: u64::MAX };
    let before = serves();
    let answer = b.serve_receipts(&bvk, None, far, false, 8, usize::MAX);
    let mid = serves();
    let ReceiptsAnswer::Root(root) = answer else { panic!("a catch-up below the root is answered with the root") };
    assert!(Predecessor::cites(&root.parent(), &proof.parent()), "b's own root");
    assert!(mid[ServeAxis::ServedRoot as usize] > before[ServeAxis::ServedRoot as usize], "counted as a served root");
    let body = encode_fold_root_reply(&root).expect("the proof frames");
    let Some(ReceiptsAnswer::Root(back)) = decode_receipts_answer(&body) else { panic!("the root answer reads back") };
    assert!(Predecessor::cites(&back.parent(), &proof.parent()), "whole");
    assert_eq!(back.verify(AT), Ok(()), "and it verifies at the origin");
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
    // A fold with no proof kept: paged from the archive below its root, so
    // the page links to the asker's head, and counted.
    let (_, mut u) = court("catchup-unproved");
    let uvk = u.chain().signer();
    strokes(&mut u, 3);
    let p = u.propose_fold().expect("a final prefix");
    let wkey = super::fixtures::court_key("catchup-witness");
    let mut wc = ReceiptChain::with_signer(wkey.verifying_bytes());
    let wh = wc.append(ActKind::List, [0; 4], Vec::new(), [0; 32], Vec::new()).expect("appends");
    u.commit_fold([3; 64], &position(&wkey, &wh, p.root.signed())).expect("sealed");
    strokes(&mut u, 1);
    let unproved = serves()[ServeAxis::UnprovedRoot as usize];
    let ReceiptsAnswer::Page(page) = u.serve_receipts(&uvk, None, far, false, 8, usize::MAX) else { panic!("no proof: a page") };
    assert!(page.first().is_some_and(|r| r.stamp.place() == Place::GENESIS), "the page opens at the asker's head (here: the line's first act), below the root");
    assert!(serves()[ServeAxis::UnprovedRoot as usize] > unproved, "and the unproved root is counted");
    // Proofs that do not stand.
    let mut forged = proof.clone();
    forged.witness.sig[0] ^= 1;
    assert!(forged.verify(AT).is_err(), "a broken witness signature");
    assert!(proof.verify(([8; 32], 2)).is_err(), "another origin");
    let mut moved = proof.clone();
    moved.root.hash = another(moved.root.hash);
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
fn hear_proves_citation_into_held_era() {
    let (bkey, mut b) = court("catchup-era-b");
    let bvk = bkey.verifying_bytes();
    strokes(&mut b, 3);
    let b0 = b.chain().at_place(Place::GENESIS).expect("b's first act");
    let proof = fold(&mut b, &bkey);
    strokes(&mut b, 1);
    let mut c0 = Court::new();
    c0.append_step(ActKind::Attest, [0xAA; 40], [1; 32]);
    assert_eq!(c0.adopt_root(AT, &proof), RootAdoption::Adopted, "c stands on b's root");
    assert_eq!(c0.adopt_root(AT, &proof), RootAdoption::Held, "and adopts it once");
    assert_eq!(c0.known_head(&bvk).map(|(at, _)| at), Some(proof.root.at), "b's line stands at the root here");
    let c = Mutex::new(c0);
    let dkey = super::fixtures::court_key("catchup-era-d");
    let dvk = dkey.verifying_bytes();
    let mut dc = ReceiptChain::with_signer(dvk);
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
    let proved = axes(bvk)[replication::FOLDED_PROVED];
    let heard: Result<_, ()> = crate::server::backfill::hear(
        &c,
        ORIGIN,
        || Ok(c.lock().expect("c").admit_receipt(AT, super::fixtures::verified(&pre, &dkey), &super::fixtures::reconstructed(&head))),
        || c.lock().ok().map(|mut k| k.stroke()),
        source(&courts, &asked),
    );
    assert!(matches!(heard, Ok(Admitted::Parent(_))), "d's head is cited: {heard:?}");
    assert!(axes(bvk)[replication::FOLDED_PROVED] > proved, "the citation proved by its era");
    let asks = asked.into_inner().expect("asks");
    assert!(asks.iter().all(|(who, _, ask)| *who != bvk || *ask == Ask::Era), "b's line asked only as the era's replay: {asks:?}");
}

/// 47.3 — A ROOT ALREADY CO-SIGNED IS ANSWERED WITH THE CO-SIGNATURE HELD (the
/// fold-witness storm read on the mesh, 2026-10-15). A court holding b's root
/// with the witness's signed position answers that witness's re-asked
/// proposal of the same root with that very position — nothing re-derived;
/// for another root, or as another key than the one that co-signed, it holds
/// no answer and the prefix is re-derived as before.
#[test]
fn cosigned_root_answers_with_held_cosignature() {
    let (bkey, mut b) = court("catchup-cosigned-b");
    let bvk = bkey.verifying_bytes();
    strokes(&mut b, 3);
    let proof = fold(&mut b, &bkey);
    let wvk = super::fixtures::court_key("catchup-witness").verifying_bytes();
    let mut w = Court::new();
    w.append_step(ActKind::Attest, [0xAA; 40], [1; 32]);
    assert_eq!(w.adopt_root(AT, &proof), RootAdoption::Adopted, "the root held with both signed positions");
    let said = w.cosigned_root(&bvk, &proof.root, &wvk).expect("its own co-signature, said again");
    assert_eq!((said.sig, &said.prefix.payload), (proof.witness.sig, &proof.witness.prefix.payload), "the very position it signed over the root");
    assert!(super::fixtures::one_receipt(&said.receipt, &proof.witness.receipt), "and the receipt it signed it with");
    assert!(w.cosigned_root(&bvk, &proof.root, &[0x77; 32]).is_none(), "another key co-signed nothing here");
    let mut moved = proof.root;
    moved.hash = another(moved.hash);
    assert!(w.cosigned_root(&bvk, &moved, &wvk).is_none(), "another root is re-derived");
    assert!(w.cosigned_root(&[0x55; 32], &proof.root, &wvk).is_none(), "another home's root is not this one");
}

/// 78.147 (3′) — AN ERA LONGER THAN A PAGE IS PROVED PAGE BY PAGE (the wedge
/// read on hypha b, 2026-10-15). As (3), but b's era runs twelve strokes —
/// two dozen receipts and more against a page of eight — so no one page
/// reaches the root from the cited act. The walk keeps its place at each
/// page's end and the next page opens there; the citation is proved and d's
/// head cited, every ask of b's line an era replay, more than one of them.
#[test]
fn hear_proves_long_era_page_by_page() {
    let (bkey, mut b) = court("catchup-long-era-b");
    let bvk = bkey.verifying_bytes();
    strokes(&mut b, 12);
    let b0 = b.chain().at_place(Place::GENESIS).expect("b's first act");
    let proof = fold(&mut b, &bkey);
    strokes(&mut b, 1);
    let mut c0 = Court::new();
    c0.append_step(ActKind::Attest, [0xAA; 40], [1; 32]);
    assert_eq!(c0.adopt_root(AT, &proof), RootAdoption::Adopted, "c stands on b's root");
    let c = Mutex::new(c0);
    let dkey = super::fixtures::court_key("catchup-long-era-d");
    let mut dc = ReceiptChain::with_signer(dkey.verifying_bytes());
    dc.admit(b0.clone());
    dc.append(ActKind::List, [0; 4], Vec::new(), [0; 32], vec![Parent { at: b0.stamp, hash: b0.chain_hash }]).expect("d cites b0");
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
    let asked = Mutex::new(Vec::new());
    let courts = [&b];
    let proved = axes(bvk)[replication::FOLDED_PROVED];
    let heard: Result<_, ()> = crate::server::backfill::hear(
        &c,
        ORIGIN,
        || Ok(c.lock().expect("c").admit_receipt(AT, super::fixtures::verified(&pre, &dkey), &super::fixtures::reconstructed(&r))),
        || c.lock().ok().map(|mut k| k.stroke()),
        source(&courts, &asked),
    );
    assert!(matches!(heard, Ok(Admitted::Parent(_))), "d's head is cited: {heard:?}");
    assert!(axes(bvk)[replication::FOLDED_PROVED] > proved, "the citation proved by its era");
    let asks = asked.into_inner().expect("asks");
    let replays = asks.iter().filter(|(who, _, ask)| *who == bvk && *ask == Ask::Era).count();
    assert!(replays > 1, "the era walked over more than one page: {asks:?}");
    assert!(asks.iter().all(|(who, _, ask)| *who != bvk || *ask == Ask::Era), "b's line asked only as the era's replay");
}

/// 78.147 (4) — CROSS-CITING LINES EACH ADVANCE FROM ITS ROOT. Courts a
/// and b cite each other's acts, fold on two signatures each, and cite each
/// other past the two roots. Court c heard only the first strokes of a and b. Hearing
/// a's head, then b's, c adopts both roots — the citations a's hot line makes
/// of b's are caught up from b's root in the same hearing — and cites both
/// heads.
#[test]
#[expect(clippy::disallowed_methods, reason = "rule 15 — a line is read in its own order, and its last receipt is the head the hearing resumes after: the order is content")]
fn hear_advances_cross_citing_lines_from_roots() {
    let (akey, mut a) = court("catchup-cross-a");
    let (bkey, mut b) = court("catchup-cross-b");
    let (avk, bvk) = (akey.verifying_bytes(), bkey.verifying_bytes());
    strokes(&mut a, 1);
    strokes(&mut b, 1);
    let mut c0 = Court::new();
    c0.append_step(ActKind::Attest, [0xAA; 40], [1; 32]);
    super::fixtures::admit_line(&mut c0, &akey, &line(&a, None));
    super::fixtures::admit_line(&mut c0, &bkey, &line(&b, None));
    // Each hears the other's line and cites it, twice over.
    let mut heard_a: Option<Place> = None;
    let mut heard_b: Option<Place> = None;
    let cross = |a: &mut Court, b: &mut Court, heard_a: &mut Option<Place>, heard_b: &mut Option<Place>| {
        let from_a = line(a, *heard_a);
        super::fixtures::admit_line(b, &akey, &from_a);
        *heard_a = from_a.last().map(|r| r.stamp.place()).or(*heard_a);
        strokes(b, 1);
        let from_b = line(b, *heard_b);
        super::fixtures::admit_line(a, &bkey, &from_b);
        *heard_b = from_b.last().map(|r| r.stamp.place()).or(*heard_b);
        strokes(a, 1);
    };
    for _ in 0..2 {
        cross(&mut a, &mut b, &mut heard_a, &mut heard_b);
    }
    // b hears a's last stroke, so each holds the other's line whole; then
    // both fold, and what each reads of the other opens at the root.
    let rest = line(&a, heard_a);
    super::fixtures::admit_line(&mut b, &akey, &rest);
    heard_a = rest.last().map(|r| r.stamp.place()).or(heard_a);
    let ra = fold(&mut a, &akey);
    let rb = fold(&mut b, &bkey);
    assert_eq!((heard_a, heard_b), (Some(ra.root.at), Some(rb.root.at)), "each heard the other up to its root");
    for _ in 0..2 {
        cross(&mut a, &mut b, &mut heard_a, &mut heard_b);
    }
    // b answers a's last stroke, so neither head cites the other's.
    super::fixtures::admit_line(&mut b, &akey, &line(&a, heard_a));
    strokes(&mut b, 1);
    let c = Mutex::new(c0);
    let courts = [&a, &b];
    for (key, k, root) in [(&akey, &a, &ra), (&bkey, &b, &rb)] {
        let asked = Mutex::new(Vec::new());
        let (head, pre) = frame(k);
        let heard: Result<_, ()> = crate::server::backfill::hear(
            &c,
            ORIGIN,
            || Ok(c.lock().expect("c").admit_receipt(AT, super::fixtures::verified(&pre, key), &super::fixtures::reconstructed(&head))),
            || c.lock().ok().map(|mut k| k.stroke()),
            source(&courts, &asked),
        );
        assert!(matches!(heard, Ok(Admitted::Parent(_))), "the head is cited: {heard:?}");
        let held = c.lock().expect("c").known_head(&k.chain().signer()).map(|(at, _)| at);
        assert!(held.is_some_and(|h| h > root.root.at), "the line advanced past its root");
    }
    assert!(axes(avk)[replication::ROOT_ADOPTED] > 0 && axes(bvk)[replication::ROOT_ADOPTED] > 0, "both roots adopted");
    let mut c = c.into_inner().expect("c");
    c.append_step(ActKind::Attest, [0xAB; 40], [1; 32]);
    let top = a.chain().head_height().max(b.chain().head_height());
    assert!(c.chain().head_height() > top, "c's act cites both heads");
}

/// D2.5 — A CAPACITY OF EIGHT READS: `C = ⌊M_proc / b⌋` at eight bytes and
/// one byte a read.
fn eight_reads() -> crate::physics::Physics {
    use crate::physics::{Calibration, Memory, MemorySource, Physics};
    let one = std::num::NonZeroU64::MIN;
    let eight = std::num::NonZeroU64::new(8).expect("eight bytes");
    Physics::calibrated(&Calibration::of(Memory::of(eight, MemorySource::CgroupMax), one, one).expect("eight reads of one byte"))
}

/// A PAGE SOURCE HOLDING `line` AND NO ROOT: every ask is answered with
/// the line's receipts strictly after `after` up to `upto`, eight a page —
/// a court that holds the signer's line but not its fold's proof.
fn mirror(line: &[KStepReceipt]) -> impl FnMut(&[u8; 32], Option<Place>, Place, Ask) -> Option<ReceiptsAnswer> + '_ {
    move |_: &[u8; 32], after: Option<Place>, upto: Place, _: Ask| {
        let page = line.iter().filter(|r| after.is_none_or(|a| a < r.stamp.place()) && r.stamp.place() <= upto).take(8).cloned();
        Some(ReceiptsAnswer::Page(page.collect()))
    }
}

/// `k`'s head heard at `c` over `fetch`, a stroke of a fixture's size each
/// pass: what the head came to.
fn hear_head(
    c: &Mutex<Court>,
    key: &HyphaKey,
    k: &Court,
    fetch: impl FnMut(&[u8; 32], Option<Place>, Place, Ask) -> Option<ReceiptsAnswer>,
) -> Result<Admitted, ()> {
    let (head, pre) = frame(k);
    crate::server::backfill::hear(
        c,
        ORIGIN,
        || Ok(c.lock().expect("c").admit_receipt(AT, super::fixtures::verified(&pre, key), &super::fixtures::reconstructed(&head))),
        || Some(super::fixtures::budget()),
        fetch,
    )
}

/// A COURT BEHIND A SIGNER BY MORE THAN ITS CAPACITY, AT THE BOUND. Court c,
/// of capacity eight, heard b's first stroke under b's signature. b then
/// acted `below` strokes, folded on two signatures, and acted `above`
/// strokes past the root. c hears b's head through a source holding b's
/// line and no root: eight receipts of the gap are held unreached, the
/// ninth is refused, and the head waits on its gap. Answers b's key, b, c,
/// b's root proof and the place c's reached line of b stands at.
fn behind_past_capacity(name: &str, below: u64, above: u64) -> (HyphaKey, Court, Mutex<Court>, RootProof, Place) {
    let (bkey, mut b) = court(name);
    let bvk = bkey.verifying_bytes();
    strokes(&mut b, 1);
    let mut c0 = Court::new();
    c0.physics = eight_reads();
    c0.append_step(ActKind::Attest, [0xAA; 40], [1; 32]);
    let signed = super::fixtures::admit_line(&mut c0, &bkey, &line(&b, None));
    assert!(signed.iter().all(|v| matches!(v, Admitted::Parent(_))), "c holds b's first stroke under b's signature: {signed:?}");
    let reached = c0.known_head(&bvk).map(|(at, _)| at).expect("b's first stroke held");
    strokes(&mut b, below);
    let early = line(&b, None);
    let proof = fold(&mut b, &bkey);
    strokes(&mut b, above);
    let mut whole: Vec<KStepReceipt> = early.into_iter().filter(|r| r.stamp.place() <= proof.root.at).collect();
    whole.extend(line(&b, None));
    let gap = whole.iter().filter(|r| r.stamp.place() > reached).count();
    assert!(u64::try_from(gap).is_ok_and(|g| g > c0.physics.capacity()), "the gap, {gap} receipts, is longer than c's capacity");
    let c = Mutex::new(c0);
    let heard = hear_head(&c, &bkey, &b, mirror(&whole));
    assert!(matches!(heard, Ok(Admitted::Gap { .. })), "no root, no close: the head waits on its gap: {heard:?}");
    let held = c.lock().expect("c").unreached_held();
    assert_eq!(u64::try_from(held).ok(), Some(c.lock().expect("c").physics.capacity()), "the gap fills the held bound and stops there");
    (bkey, b, c, proof, reached)
}

/// MU-HOSTING §5a.9, 78.147 (#88) — A GAP PAST THE CAPACITY CLOSES THROUGH
/// A FOLD ROOT. c, of capacity eight, holds eight receipts of b's gap
/// unreached, past b's root, and refuses the ninth. Heard from b, whose
/// door holds the root's proof, the page past the bound stops and the range
/// is asked from below c's unreached floor: b answers with its root. The
/// receipt c holds at the root's place is the one the root names, so the
/// root's two signatures reach it and every receipt held below it; the gate
/// frees, the rest of the gap is held within the bound, and b's head is
/// cited.
#[test]
fn gap_past_capacity_closes_through_a_fold_root() {
    let (bkey, b, c, proof, _) = behind_past_capacity("catchup-capacity-reach-b", 3, 3);
    let bvk = bkey.verifying_bytes();
    let top = c.lock().expect("c").known_head(&bvk).map(|(at, _)| at).expect("held");
    assert!(top > proof.root.at, "c's held line of b stands past b's root");
    let before = serves();
    let asked = Mutex::new(Vec::new());
    let courts = [&b];
    let heard = hear_head(&c, &bkey, &b, source(&courts, &asked));
    assert!(matches!(heard, Ok(Admitted::Parent(_))), "b's head is cited: {heard:?}");
    assert!(serves()[ServeAxis::ServedRoot as usize] > before[ServeAxis::ServedRoot as usize], "the gap was answered with b's root");
    let asks = asked.into_inner().expect("asks");
    assert!(asks.iter().any(|(_, at, _)| at.is_some_and(|a| a < proof.root.at)), "the root was asked from below c's unreached floor: {asks:?}");
    let mut c = c.into_inner().expect("c");
    assert_eq!(c.unreached_held(), 0, "b's signatures reached every receipt of the gap");
    assert_eq!(c.known_head(&bvk).map(|(at, _)| at), Some(frame(&b).0.stamp.place()), "c holds b's line to its head");
    c.append_step(ActKind::Attest, [0xAB; 40], [1; 32]);
    assert!(c.chain().head_height() > b.chain().head_height(), "c's act cites b's head and stands above it");
}

/// D2.5, MU-HOSTING §5a.9 (#88) — THE BOUND HOLDS WHILE A LONG GAP CLOSES.
/// c, of capacity eight, holds eight receipts of a gap of b's line more
/// than twice its capacity, all below b's root. Heard from b, the gap is
/// answered with the root, adopted on its two signatures; the receipts held
/// unreached below it leave the chain and the gate with the root's shed,
/// and the hot line past the root is held within the bound. At every ask
/// and at the close, c holds at most eight receipts unreached.
#[test]
fn capacity_bound_holds_while_a_long_gap_closes() {
    let (bkey, b, c, proof, _) = behind_past_capacity("catchup-capacity-bound-b", 9, 1);
    let bvk = bkey.verifying_bytes();
    let capacity = c.lock().expect("c").physics.capacity();
    let top = c.lock().expect("c").known_head(&bvk).map(|(at, _)| at).expect("held");
    assert!(top < proof.root.at, "c's held line of b stands below b's root");
    let mut most = 0usize;
    let asked = Mutex::new(Vec::new());
    let courts = [&b];
    let mut serve = source(&courts, &asked);
    let heard = hear_head(&c, &bkey, &b, |who: &[u8; 32], after: Option<Place>, upto: Place, ask: Ask| {
        let held = c.lock().expect("c").unreached_held();
        most = most.max(held);
        serve(who, after, upto, ask)
    });
    assert!(matches!(heard, Ok(Admitted::Parent(_))), "b's head is cited: {heard:?}");
    assert!(axes(bvk)[replication::ROOT_ADOPTED] > 0, "the root adopted, counted under b");
    let c = c.into_inner().expect("c");
    assert!(u64::try_from(most).is_ok_and(|m| m <= capacity), "at most {capacity} held unreached at every ask: {most}");
    assert_eq!(c.unreached_held(), 0, "and none at the close");
    assert_eq!(c.known_head(&bvk).map(|(at, _)| at), Some(frame(&b).0.stamp.place()), "c holds b's line to its head");
}

/// 78.147, MU-HOSTING §5a.10 (#88) — A FORGED FOLD ROOT IS REFUSED BY NAME
/// AND EVICTS NOTHING REACHED. c holds b's first stroke under b's signature
/// and eight receipts of b's gap unreached, at its bound. A root whose
/// witness signature is broken is refused `REFUSE_SIGNATURE`, and one whose
/// hash was moved off the signed positions `REFUSE_RECEIPT`; heard through
/// a source that answers with the forged root, the head still waits and the
/// refusal is counted under b. Every receipt c held stands: each reached
/// one at its place, each unreached one still held.
#[test]
fn adopt_root_refuses_forged_root_keeping_reached() {
    let (bkey, b, c, proof, reached) = behind_past_capacity("catchup-capacity-forged-b", 3, 3);
    let bvk = bkey.verifying_bytes();
    let held_before: Vec<KStepReceipt> = c.lock().expect("c").chain().line_between(&bvk, None, Place { k: u64::MAX, i: u64::MAX }, usize::MAX);
    let mut forged = proof.clone();
    forged.witness.sig[0] ^= 1;
    let mut moved = proof.clone();
    moved.root.hash = another(moved.root.hash);
    {
        let mut k = c.lock().expect("c");
        assert_eq!(k.adopt_root(AT, &forged), RootAdoption::Refused(crate::verdict::REFUSE_SIGNATURE), "a broken witness signature");
        assert_eq!(k.adopt_root(AT, &moved), RootAdoption::Refused(crate::verdict::REFUSE_RECEIPT), "a root the signed positions do not bind");
    }
    let refused = axes(bvk)[replication::ROOT_REFUSED];
    let courts = [&b];
    let asked = Mutex::new(Vec::new());
    let mut serve = source(&courts, &asked);
    let heard = hear_head(&c, &bkey, &b, |who: &[u8; 32], after: Option<Place>, upto: Place, ask: Ask| match serve(who, after, upto, ask) {
        Some(ReceiptsAnswer::Root(_)) => Some(ReceiptsAnswer::Root(Box::new(forged.clone()))),
        other => other,
    });
    assert!(matches!(heard, Ok(Admitted::Gap { .. })), "the head still waits on its gap: {heard:?}");
    assert!(axes(bvk)[replication::ROOT_REFUSED] > refused, "the forged root refused, counted under b");
    let c = c.into_inner().expect("c");
    assert_eq!(c.chain().peer_root(&bvk), None, "no root of b stands at c");
    for r in &held_before {
        assert!(c.chain().at(&bvk, r.stamp.place()).is_some_and(|h| super::fixtures::one_receipt(&h, r)), "{:?} stands", r.stamp.place());
        let unreached = c.unreached_at(&bvk, r.stamp.place()).is_some();
        assert_eq!(unreached, r.stamp.place() > reached, "{:?}: reached below the signed stroke, held unreached above it", r.stamp.place());
    }
}

/// A court holding none of `name`'s line, and the fold root that signer
/// sealed on two signatures after three strokes: the signer's key, the
/// court and the proof.
fn root_unheld(name: &str) -> ([u8; 32], Court, RootProof) {
    let (bkey, mut b) = court(name);
    strokes(&mut b, 3);
    let proof = fold(&mut b, &bkey);
    let bvk = bkey.verifying_bytes();
    let mut w = Court::new();
    w.append_step(ActKind::Attest, [0xAA; 40], [1; 32]);
    assert!(w.known_head(&bvk).is_none(), "w holds nothing of b's line");
    (bvk, w, proof)
}

/// What a refused root leaves at `w`: no root of `bvk` stands, no proof of
/// it is filed, and the refusal is counted under it.
fn nothing_adopted(w: &Court, bvk: &[u8; 32]) {
    assert_eq!(w.chain().peer_root(bvk), None, "no root of b stands at w");
    assert!(w.root_proof(bvk).is_none(), "no proof of b is filed");
    assert!(axes(*bvk)[replication::ROOT_REFUSED] > 0, "the refusal counted under b");
}

/// MU-HOSTING §5a.9, §5a.10 (#91) — A ROOT RESTATED AT ANOTHER PLACE IS
/// REFUSED WITH NO RECEIPT HELD TO CHECK IT AGAINST. b folds on two
/// signatures, each over its root written out (`Root::signed`). w holds
/// nothing of b's line, so neither the receipt at the root's place
/// (`reach_by_root`) nor a hot line past it reads the place. The proof
/// restated with the signed hash and height at another `K` or another `i`
/// is refused `REFUSE_RECEIPT` by the signatures alone, and nothing is
/// adopted. The proof as signed is then adopted at the place its
/// signatures state.
#[test]
fn root_at_another_place_refuses_without_held_receipt() {
    let (bvk, mut w, proof) = root_unheld("catchup-root-place-b");
    let at = proof.root.at;
    for moved_at in [Place { k: at.k + 1, ..at }, Place { i: at.i + 1, ..at }] {
        let mut moved = proof.clone();
        moved.root.at = moved_at;
        assert_eq!(w.adopt_root(AT, &moved), RootAdoption::Refused(crate::verdict::REFUSE_RECEIPT), "{moved_at:?}: a place the signatures do not state");
    }
    nothing_adopted(&w, &bvk);
    let signed = crate::chronometry::Root::of_signed(&proof.home.prefix.payload);
    assert!(signed.is_some_and(|r| r.restates(&proof.root)), "the home's signature states the root's place, height and hash");
    assert_eq!(w.adopt_root(AT, &proof), RootAdoption::Adopted, "the root as signed");
    assert_eq!(w.chain().peer_root(&bvk), Some(at), "b's root stands at the place signed");
}

/// MU-HOSTING §5a.9, §5a.10 (#91) — A ROOT RESTATED AT ANOTHER HEIGHT IS
/// REFUSED. As above, with the signed place and hash kept and the height
/// one above or one below the one signed: `REFUSE_RECEIPT`, and nothing is
/// adopted. A position whose payload is the bare hash, as version 1 signed
/// it, states no root and is refused the same way.
#[test]
fn root_at_another_height_refuses() {
    let (bvk, mut w, proof) = root_unheld("catchup-root-height-b");
    let h = proof.root.height;
    let below = h.checked_sub(1).expect("the root stands above genesis");
    for height in [h + 1, below] {
        let mut moved = proof.clone();
        moved.root.height = height;
        assert_eq!(w.adopt_root(AT, &moved), RootAdoption::Refused(crate::verdict::REFUSE_RECEIPT), "height {height}: a height the signatures do not state");
    }
    nothing_adopted(&w, &bvk);
    let mut bare = proof.clone();
    bare.home.prefix.payload = proof.root.hash.bytes().to_vec();
    let rekey = super::fixtures::court_key("catchup-root-height-b");
    bare.home.sig = rekey.sign(&bare.home.prefix.bytes().expect("the prefix frames"));
    assert_eq!(w.adopt_root(AT, &bare), RootAdoption::Refused(crate::verdict::REFUSE_RECEIPT), "a bare hash states no root");
    nothing_adopted(&w, &bvk);
    assert_eq!(w.adopt_root(AT, &proof), RootAdoption::Adopted, "the root as signed");
}
