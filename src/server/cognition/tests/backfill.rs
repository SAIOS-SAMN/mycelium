// 45.2, 47.2 step 3 — THE CHASE OF A GAP, read on the observer (ACT1-hold-b,
// LD0's defect 3b): two courts, one signing faster than the other's pass
// can page, and what the chase did per signer.

use crate::cognition::*;
use crate::observe::rep;

/// b's head receipt and the prefix a digest signs for it — the frame a
/// digest carries, verified at the call under b's key.
fn head_of(
    b: &crate::chronometry::ReceiptChain,
) -> (crate::chronometry::KStepReceipt, crate::court::admission::SignedPrefix) {
    let r = b.head().expect("b has acted").clone();
    let pre = crate::court::admission::SignedPrefix {
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

/// b's line extended by `n` strokes of its own, each one act and the close
/// of its orbit (OK1), so b's `K` moves once per stroke.
fn act(b: &mut crate::chronometry::ReceiptChain, n: u64) {
    for _ in 0..n {
        b.append(crate::chronometry::ActKind::List, [0; 4], Vec::new(), [0; 32], Vec::new()).expect("appends");
        let (_, frame) = b.open_orbit().expect("open");
        b.append(crate::chronometry::ActKind::Orbit, frame, Vec::new(), [0; 32], Vec::new()).expect("closes");
    }
}

/// ACT1-hold-b — A HEARD HEAD IS CITED ONCE ITS GAP CLOSES (45.2). Court b
/// signs between c's exchanges faster than one pass of c's chase pages —
/// the rate is measured off c's own stroke and the page, never set. Each
/// exchange c hears b's head, finds the gap, chases it, and admits the head
/// again. What 45.2 asks: the pass reaches the reported `K − 1`, the head
/// is admitted as a PARENT (the one admission that advances c's height),
/// and the K lag at report does not grow.
///
/// Mechanism (A), which this read red before the fix: a pass paced by one
/// stroke's budget ended short every time, the head re-admitted into
/// another gap, no parent was ever cited, and the lag climbed 143, 215,
/// 287, 359 — hypha c's 16 202 back-filled receipts and no parent, read in
/// two kernels. With the aperture measured against the lag, every pass
/// reaches `K − 1` and cites the head.
#[test]
fn a_heard_head_is_cited_once_its_gap_closes() {
    let key = super::common::court_key("backfill-b");
    let vk = key.vk_bytes();
    let mut b = crate::chronometry::ReceiptChain::with_vk(vk);
    // c has acted on its own line, as a running court has: its height is
    // then `1 + max(prev, parents)` (45.2).
    let mut c0 = Kernel::new();
    c0.append_step(crate::chronometry::ActKind::Attest, [0xAA; 40], [1; 32]);
    let c = std::sync::Mutex::new(c0);
    const PAGE: u64 = 8;
    let pages_per_pass = c.lock().expect("c").stroke().remaining() as u64;
    // Faster than one pass can page: twice what a pass can fetch.
    let rate = 2 * pages_per_pass * PAGE;
    let mut lags = Vec::new();
    for _ in 0..4 {
        act(&mut b, rate);
        let (head, pre) = head_of(&b);
        let fetch = |who: &[u8; 32],
                     after: Option<crate::chronometry::Place>,
                     upto: crate::chronometry::Place|
         -> Option<Vec<crate::chronometry::KStepReceipt>> {
            (*who == vk).then(|| b.line_between(who, after, upto, usize::try_from(PAGE).expect("PAGE is the constant 8")).into_iter().cloned().collect())
        };
        let heard: Result<_, ()> = crate::server::backfill::hear(
            &c,
            || Ok(c.lock().expect("c").admit_receipt(([7; 32], 2), super::common::verified(&pre, &key), &head)),
            || c.lock().ok().map(|mut k| k.stroke()),
            fetch,
        );
        let _ = heard;
        let r = crate::observe::replication();
        let axes = r.signers.iter().find(|(s, _)| *s == vk).map(|(_, a)| *a).expect("b is heard");
        lags.push(axes[rep::LAG]);
    }
    let r = crate::observe::replication();
    let axes = r.signers.iter().find(|(s, _)| *s == vk).map(|(_, a)| *a).expect("b is heard");
    eprintln!(
        "chase on b: gap reported {} reached {} budget {} parent after {} lag by exchange {lags:?}",
        axes[rep::GAP],
        axes[rep::BACKFILL_REACHED],
        axes[rep::BACKFILL_BUDGET],
        axes[rep::PARENT_AFTER_BACKFILL]
    );
    assert!(axes[rep::BACKFILL_REACHED] > 0, "a pass reaches the reported K − 1");
    assert!(axes[rep::PARENT_AFTER_BACKFILL] > 0, "and the head is cited as a parent (45.2)");
    assert!(lags.windows(2).all(|w| w[1] <= w[0]), "and the lag does not grow: {lags:?}");
    // 45.2: the parent is what advances c's height. c's next act cites b's
    // head and stands above it.
    let mut c = c.into_inner().expect("c");
    c.append_step(crate::chronometry::ActKind::Attest, [0xAB; 40], [1; 32]);
    let b_h = b.head().expect("b").stamp.h;
    assert!(c.chain().head_height() > b_h, "c's act cites b's head: height {} over b's {b_h}", c.chain().head_height());
}

/// PHASE 1 — A HEAD THAT CITES ANOTHER SIGNER IS CITED ONCE THAT SIGNER IS
/// FETCHED. b's head names a receipt of d's as a parent, and the court
/// hearing it holds neither line: the chase closes b's own gap, finds d's
/// receipt missing, chases d up to the place cited, and admits b's head as
/// a parent. Before, `hear` chased only the frame's own line and returned
/// on the missing parent — on the fleet, courts b's and c's lines, each
/// citing the other's receipts, never advanced at a.
#[test]
fn a_head_citing_another_signer_is_cited_once_that_signer_is_fetched() {
    let key = super::common::court_key("backfill-cites-b");
    let vk = key.vk_bytes();
    let dvk = super::common::court_key("backfill-cites-d").vk_bytes();
    let mut d = crate::chronometry::ReceiptChain::with_vk(dvk);
    act(&mut d, 2);
    let dh = d.head().expect("d has acted").clone();
    let mut b = crate::chronometry::ReceiptChain::with_vk(vk);
    act(&mut b, 1);
    // b holds d's line, as a court holds what it heard, and cites its head.
    for r in d.line_between(&dvk, None, dh.stamp.place(), 64) {
        b.admit(r.clone());
    }
    b.append(
        crate::chronometry::ActKind::List,
        [0; 4],
        Vec::new(),
        [0; 32],
        vec![crate::chronometry::Parent { at: dh.stamp, hash: dh.chain_hash }],
    )
    .expect("b cites d");
    let (head, pre) = head_of(&b);
    let mut c0 = Kernel::new();
    c0.append_step(crate::chronometry::ActKind::Attest, [0xAA; 40], [1; 32]);
    let c = std::sync::Mutex::new(c0);
    let fetch = |who: &[u8; 32], after: Option<crate::chronometry::Place>, upto: crate::chronometry::Place| {
        let line = match () {
            _ if *who == vk => &b,
            _ if *who == dvk => &d,
            _ => return None,
        };
        Some(line.line_between(who, after, upto, 64).into_iter().cloned().collect::<Vec<_>>())
    };
    let heard: Result<_, ()> = crate::server::backfill::hear(
        &c,
        || Ok(c.lock().expect("c").admit_receipt(([7; 32], 2), super::common::verified(&pre, &key), &head)),
        || c.lock().ok().map(|mut k| k.stroke()),
        fetch,
    );
    assert!(matches!(heard, Ok(crate::court::admission::Admitted::Parent(_))), "b's head is cited: {heard:?}");
    assert!(c.lock().expect("c").known_head(&dvk).is_some(), "d's line was fetched for the citation");
}
