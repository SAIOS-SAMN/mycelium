// Issue #118, ROADMAP track 24 B2/B4 — THE ALIGNMENT READ BY THE SIGNING
// EVENTS ALONE: a crossing's reading visits the reserve's signing events,
// never the ledger, and reads what the whole walk of the ledger reads, its
// own folded line included.

use crate::anastome::dialogue::domain::{DomainGenesis, Founded, Reservation};
use crate::anastome::dialogue::ReceiptSection;
use crate::chronometry::{ActKind, Cite, KStepReceipt, ReceiptChain, Stamp};
use crate::court::admission::SignedPrefix;
use crate::court::identity::HyphaKey;
use crate::network::nerve::Keyed;
use crate::physics::laws::clauses::{reconstruct, register, Choice, SignedClause, Signing, SigningEvent};
use crate::server::cognition::Court;
use ed25519_dalek::SigningKey;

fn domain_key() -> SigningKey {
    SigningKey::from_bytes(&[4; 32])
}

fn kernel_key() -> SigningKey {
    SigningKey::from_bytes(&[9; 32])
}

fn vk(k: &SigningKey) -> [u8; 32] {
    k.verifying_key().to_bytes()
}

/// A stroke wide enough that no reading here is cut short.
fn reads() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 20)
}

/// The domain founded naming the kernel as its one tier, the kernel its
/// reserve.
fn found(k: &mut Court) {
    let genesis = DomainGenesis { domain: vk(&domain_key()), tiers: vec![vk(&kernel_key())] }.sign(&domain_key());
    assert!(matches!(k.hear_domain(&genesis), Founded::Sealed { .. }), "founded");
    let r = Reservation { domain: vk(&domain_key()), tier: vk(&kernel_key()), nonce: [0; 32] };
    assert!(matches!(k.designate_reserve(&r.sign(&domain_key())), Founded::Reserved { .. }), "reserved");
}

/// The kernel's signing of every registered clause, enabled and reached.
fn signing() -> Vec<u8> {
    let clauses = register()
        .expect("the register stands")
        .into_iter()
        .map(|r| SignedClause { id: r.clause.id.clone(), choice: Choice::Enabled { reached: r.clause.fixtures.iter().map(|f| reconstruct(f).expect("reached")).collect() }, fixtures: r.clause.fixtures })
        .collect();
    Signing::sign(&kernel_key(), clauses).to_bytes()
}

/// `n` strokes, one act and its orbit's close each, none a signing.
fn strokes(k: &mut Court, n: u64) {
    for _ in 0..n {
        k.append_step(ActKind::List, [1; 32], [2; 32]);
        k.orbit();
    }
}

/// THE READING AS THE WHOLE WALK READ IT before the index: every receipt of
/// the own folded line and every hot node a signature reached, each read,
/// its signing citations kept in the ledger's stamp order.
fn walked(k: &Court) -> Vec<SigningEvent> {
    let mut found: Keyed<SigningEvent> = Keyed::new();
    let mut file = |r: &KStepReceipt| {
        if r.kind != ActKind::Domain {
            return;
        }
        for (n, c) in r.cites.iter().enumerate() {
            let Cite::Signing { kernel, reserve, verdict } = c else { continue };
            let mut key = r.stamp.key();
            key.extend_from_slice(&(n as u64).to_be_bytes());
            found.insert(key, SigningEvent { at: r.stamp, kernel: *kernel, reserve: *reserve, verdict: (**verdict).clone() });
        }
    };
    for e in k.archive.iter().filter(|e| e.signer == k.identity) {
        for r in k.era_receipts(e).into_iter().flatten() {
            file(&r);
        }
    }
    for r in k.chain.nodes() {
        if !k.unreached.contains(&Stamp::line_key(&r.stamp.vk, r.stamp.place())) {
            file(&r);
        }
    }
    found.scoped().map(|(_, e)| e.clone()).collect()
}

/// The reading `k` gives on the stroke `reads`, and the reads it spent.
fn read_spending(k: &mut Court, only: Option<[u8; 32]>) -> (crate::physics::laws::clauses::Alignment, usize) {
    let mut stroke = reads();
    let before = stroke.remaining();
    let read = k.alignment(only, &mut stroke).expect("read whole");
    (read, before - stroke.remaining())
}

/// The reading as the walk's events give it.
fn walked_reading(k: &Court, only: Option<[u8; 32]>) -> crate::physics::laws::clauses::Alignment {
    let ids: Vec<_> = register().expect("stands").into_iter().map(|r| r.clause.id).collect();
    crate::physics::laws::clauses::Alignment {
        kernels: crate::physics::laws::clauses::read_alignment(&ids, &walked(k), only),
        history: crate::physics::laws::clauses::history(),
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

/// The court's own line folded on two signatures, the witness another key.
fn fold(k: &mut Court, key: &HyphaKey) {
    let p = k.propose_fold().expect("a final prefix");
    let head = k.chain().head().expect("the court acted");
    let home = position(key, &head, p.root.signed());
    let wkey = super::fixtures::court_key("signing-index-witness");
    let mut wc = ReceiptChain::with_signer(wkey.verifying_bytes());
    let wh = wc.append(ActKind::List, [0; 4], Vec::new(), [0; 32], Vec::new()).expect("appends");
    let witness = position(&wkey, &wh, p.root.signed());
    k.commit_fold_proved(&home, &witness).expect("two signatures seal the fold");
}

/// A CROSSING'S READ COSTS ITS RESERVE'S SIGNING EVENTS, NEVER THE LEDGER:
/// the reserve's reading spends one read per signing event it holds, the
/// same over a ledger of a few receipts and one grown by sixty strokes.
#[test]
fn crossing_alignment_reads_only_signing_events() {
    let (mut k, _) = super::founding::hypha("signing-index-cost");
    found(&mut k);
    assert!(k.hear_signing(&signing()).is_ok());
    let (_, short) = read_spending(&mut k, Some(vk(&kernel_key())));
    let held = k.chain.len();
    strokes(&mut k, 60);
    assert!(k.chain.len() >= held + 120, "the ledger grew by the strokes");
    let (read, long) = read_spending(&mut k, Some(vk(&kernel_key())));
    assert_eq!((short, long), (1, 1), "one read per signing event, whatever the ledger holds");
    assert_eq!(read, walked_reading(&k, Some(vk(&kernel_key()))));
    // Another kernel's reading visits none of this kernel's events.
    let (_, other) = read_spending(&mut k, Some([3; 32]));
    assert_eq!(other, 0, "a kernel with no event reads nothing");
}

/// THE READING EQUALS THE WHOLE WALK'S, across a fold of the court's own
/// line: a signing folded into an era and one sealed after it both read, on
/// the asked kernel and on every kernel. The era is read once, when the
/// reading first meets it; a later reading spends the events alone.
#[test]
fn alignment_equals_full_walk_across_own_fold() {
    let (mut k, key) = super::founding::hypha("signing-index-fold");
    found(&mut k);
    assert!(k.hear_signing(&signing()).is_ok());
    strokes(&mut k, 3);
    let folded_at = k.chain.own().find(|r| r.kind == ActKind::Domain && !r.cites.is_empty()).map(|r| r.stamp);
    fold(&mut k, &key);
    assert!(k.chain.own().all(|r| Some(r.stamp) != folded_at), "the first signing left the hot line");
    assert!(!k.archive.is_empty(), "the era is held");
    assert!(k.hear_signing(&signing()).is_ok());
    strokes(&mut k, 2);
    for only in [None, Some(vk(&kernel_key()))] {
        let (read, _) = read_spending(&mut k, only);
        assert_eq!(read, walked_reading(&k, only), "the indexed reading is the walk's ({only:?})");
    }
    assert_eq!(walked(&k).len(), 2, "both signings stand, the folded one among them");
    let (_, again) = read_spending(&mut k, Some(vk(&kernel_key())));
    assert_eq!(again, 2, "the era is not read again: one read per event");
}

/// THE READING EQUALS THE WHOLE WALK'S ON A PEER: a second hypha holding
/// the first's signing receipts under its signature reads them by the
/// index its ledger filed as it admitted them.
#[test]
fn alignment_equals_full_walk_on_a_peer() {
    let (mut a, key) = super::founding::hypha("signing-index-a");
    found(&mut a);
    assert!(a.hear_signing(&signing()).is_ok());
    let (b, _) = super::founding::hypha("signing-index-b");
    let (mut b, _) = super::founding::hear(b, &a, &key);
    let (read, spent) = read_spending(&mut b, None);
    assert_eq!(read, walked_reading(&b, None));
    assert_eq!(read.kernels.len(), 1);
    assert_eq!(spent, 1);
}
