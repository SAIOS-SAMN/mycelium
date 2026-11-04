// MU-HOSTING §5a.9 B0 — REPLICATION BY RECONSTRUCTION: a domain founded on
// one hypha, its founding receipts back-filled to a second in process, and the
// second's registries reconstructed from them under the first's signature.

use crate::anastome::dialogue::domain::{BigBangRecord, DomainGenesis, Founded, Reservation};
use crate::anastome::dialogue::uplink::{TierFold, Uplinked};
use crate::anastome::dialogue::ReceiptsAnswer;
use crate::chronometry::{ActKind, Cite, KStepReceipt, ReceiptChain};
use crate::court::admission::{Admitted, SignedPrefix};
use crate::court::identity::HyphaKey;
use crate::server::cognition::Court;
use ed25519_dalek::SigningKey;

/// The origin every fixture frame is signed at.
const ORIGIN: crate::server::backfill::AdoptedOrigin = Some(([7; 32], 2));

fn domain_key() -> SigningKey {
    SigningKey::from_bytes(&[4; 32])
}

fn tier_key() -> SigningKey {
    SigningKey::from_bytes(&[9; 32])
}

fn vk(k: &SigningKey) -> [u8; 32] {
    k.verifying_key().to_bytes()
}

/// A hypha's court, bound to the hypha's key before its first act.
fn hypha(name: &str) -> (Court, HyphaKey) {
    let key = super::fixtures::court_key(name);
    let mut k = Court::new();
    k.bind_identity(key.verifying_bytes());
    (k, key)
}

/// The domain's genesis naming `tiers`, signed by the domain's key.
fn genesis(tiers: Vec<[u8; 32]>) -> crate::anastome::dialogue::domain::SignedDomain {
    DomainGenesis { domain: vk(&domain_key()), tiers }.sign(&domain_key())
}

/// The domain's origin committing to `commitment`, signed by the domain's key.
fn origin(commitment: u8) -> crate::anastome::dialogue::domain::SignedOrigin {
    BigBangRecord { domain: vk(&domain_key()), kind: 1, version: 3, commitment: [commitment; 32], universe: None }.sign(&domain_key())
}

/// THE FOUNDING HYPHA'S ACTS: the domain founded naming its one tier, the
/// tier designated its reserve, and its origin recorded.
fn found(k: &mut Court) {
    assert!(matches!(k.admit_domain(&genesis(vec![vk(&tier_key())])), Founded::Sealed { .. }), "founded");
    let r = Reservation { domain: vk(&domain_key()), tier: vk(&tier_key()), nonce: [0; 32] };
    assert!(matches!(k.designate_reserve(&r.sign(&domain_key())), Founded::Reserved { .. }), "reserved");
    assert!(matches!(k.record_origin(&origin(9)), Founded::Originated { .. }), "originated");
}

/// The prefix a digest signs for one receipt at the fixture origin.
fn prefix_of(r: &KStepReceipt) -> SignedPrefix {
    SignedPrefix {
        genesis: [7; 32],
        epoch: 2,
        prev_receipt_hash: r.prev_hash(),
        at: r.stamp.place(),
        height: r.stamp.h,
        chart: vec![],
        payload: r.stamp.key(),
    }
}

/// THE EXCHANGE'S LOOP IN PROCESS (`backfill::hear`): `a`'s signed head
/// heard on `b`, every gap it names back-filled page by page from `a`'s
/// line, and the head admitted again. Answers `b` and what the head came to.
fn hear(b: Court, a: &Court, key: &HyphaKey) -> (Court, Admitted) {
    let head = a.chain.head().expect("the founding hypha has acted");
    let pre = prefix_of(&head);
    let c = std::sync::Mutex::new(b);
    let fetch = |signer: &[u8; 32], after: Option<crate::chronometry::Place>, upto: crate::chronometry::Place, _| {
        (*signer == key.verifying_bytes()).then(|| ReceiptsAnswer::Page(a.chain.line_between(signer, after, upto, 64)))
    };
    let heard: Result<Admitted, ()> = crate::server::backfill::hear(
        &c,
        ORIGIN,
        || Ok(c.lock().expect("b").admit_receipt(([7; 32], 2), super::fixtures::verified(&pre, key), &super::fixtures::reconstructed(&head))),
        || c.lock().ok().map(|mut k| k.stroke()),
        fetch,
    );
    (c.into_inner().expect("b"), heard.expect("the loop answers"))
}

/// The receipt on `k`'s own line whose first citation `kind_of` reads.
fn sealed(k: &Court, kind_of: impl Fn(&Cite) -> bool) -> KStepReceipt {
    k.chain.own().find(|r| r.cites.iter().any(&kind_of)).expect("sealed on the founding line")
}

/// A trade between two identities, nothing made: `A` +5, `B` −5 on `S`.
fn trade() -> TierFold {
    TierFold {
        tier: vk(&tier_key()),
        prefix: [7; 32],
        rows: vec![([1; 32], [5, 0, 0, 0]), ([2; 32], [-5, 0, 0, 0])],
        declared: Vec::new(),
        children: Vec::new(),
        handoffs: Vec::new(),
    }
}

/// B0 — A DOMAIN FOUNDED ON ONE HYPHA IS RECONSTRUCTED ON ANOTHER from the
/// founding receipts back-fill delivers: its tiers, its reserve and its
/// origin, each the same object the founding hypha holds, once the founding
/// hypha's signed head links down to them.
#[test]
fn admit_backfill_reconstructs_domain_from_founding_receipts() {
    let (mut a, key) = hypha("b0-founding-a");
    found(&mut a);
    let d = vk(&domain_key());
    let (b, _) = hypha("b0-founding-b");
    assert_eq!(b.domains.get(&d), None, "nothing founded on b");
    let (b, heard) = hear(b, &a, &key);
    assert!(matches!(heard, Admitted::Parent(_)), "a's head is admitted under its signature: {heard:?}");
    assert_eq!(b.domains.get(&d), a.domains.get(&d), "the genesis and its tiers");
    assert_eq!(b.tier_domain.get(&vk(&tier_key())), Some(&d), "the tier indexed to its domain");
    assert_eq!(b.reserve_reading(&d), a.reserve_reading(&d), "the reserve");
    assert_eq!(b.origin_of(&d), a.origin_of(&d), "the origin, with the stamp and receipt that sealed it");
    assert!(b.universe_reading(&d).originated, "the domain reads as a universe on b");
}

/// B0 — A FOUNDING RECEIPT NO SIGNATURE REACHES IS NEVER READ INTO THE
/// REGISTRIES: back-filled with no signed frame of its signer, it is held on
/// the chain and the domain stays unfounded; a founding forged under the
/// hypha's key is held only until the hypha's own signed head refuses to
/// link to it; and a receipt whose citation was altered is refused by its
/// chain (its hash does not re-derive).
#[test]
fn admit_backfill_refuses_founding_not_on_signed_chain() {
    let (mut a, key) = hypha("b0-unsigned-a");
    found(&mut a);
    let d = vk(&domain_key());
    let head = a.chain.head().expect("acted");
    let page = a.chain.line_between(&key.verifying_bytes(), None, head.stamp.place(), 64);
    let (mut b, _) = hypha("b0-unsigned-b");
    let mut budget = b.stroke();
    assert_eq!(b.admit_backfill(&super::fixtures::page(&page), &mut budget), (page.len(), None), "the page links and is held");
    assert_eq!(b.domains.get(&d), None, "no signature reaches it: the domain stays unfounded");
    assert_eq!(b.origin_of(&d), None, "and holds no origin");
    assert_eq!(b.admit_uplink(&trade().sign(&tier_key())), Uplinked::Unbound, "its tier's fold is unbound");

    let mut forged = ReceiptChain::with_signer(key.verifying_bytes());
    let thief = vk(&SigningKey::from_bytes(&[66; 32]));
    forged
        .append(ActKind::Domain, [0; 4], vec![Cite::Domain { domain: d, tiers: vec![thief] }], [0; 32], Vec::new())
        .expect("a line anyone can write");
    let forged_page = forged.line_between(&key.verifying_bytes(), None, crate::chronometry::Place::GENESIS, 64);
    let (mut c, _) = hypha("b0-unsigned-c");
    let mut budget = c.stroke();
    assert_eq!(c.admit_backfill(&super::fixtures::page(&forged_page), &mut budget), (1, None), "the forged genesis links from nothing");
    let signed = super::fixtures::admit_line(&mut c, &key, &[head]);
    assert!(!matches!(signed.as_slice(), [Admitted::Parent(_)] | [Admitted::Stale]), "a's own head does not link to it: {signed:?}");
    assert_eq!(c.domains.get(&d), None, "the forged founding is never read");
    assert_eq!(c.tier_domain.get(&thief), None, "and indexes no tier");

    let mut altered = sealed(&a, |c| matches!(c, Cite::Domain { .. }));
    altered.cites = vec![Cite::Domain { domain: d, tiers: vec![thief] }];
    let (mut e, _) = hypha("b0-unsigned-e");
    let mut budget = e.stroke();
    assert_eq!(e.admit_backfill(&super::fixtures::page(&[altered]), &mut budget), (0, None), "an altered citation's hash does not re-derive");
    assert_eq!(e.domains.get(&d), None, "and nothing is founded");
}

/// B0 — THE RESERVE'S FOLD ON THE SECOND HYPHA: before the founding
/// receipts reach it, the tier's fold is unbound there; reconstructed, the
/// same fold is admitted there exactly as on the founding hypha.
#[test]
fn backfilled_domain_accepts_reserve_fold_on_second_hypha() {
    let (mut a, key) = hypha("b0-fold-a");
    found(&mut a);
    let (mut b, _) = hypha("b0-fold-b");
    assert_eq!(b.admit_uplink(&trade().sign(&tier_key())), Uplinked::Unbound, "unknown on b before the back-fill");
    let (mut b, _) = hear(b, &a, &key);
    assert!(matches!(a.admit_uplink(&trade().sign(&tier_key())), Uplinked::Admitted { .. }), "admitted on a");
    assert!(matches!(b.admit_uplink(&trade().sign(&tier_key())), Uplinked::Admitted { .. }), "and on b");
}

/// B0 — ONCE BY CONTENT: the founding line heard and back-filled twice
/// changes nothing (a reserve read again would stand pending), and a hypha
/// that founded the same genesis itself holds it unchanged and refuses
/// nothing.
#[test]
fn backfill_applies_founding_once_by_content() {
    let (mut a, key) = hypha("b0-once-a");
    found(&mut a);
    let d = vk(&domain_key());
    let (b, _) = hypha("b0-once-b");
    let (b, _) = hear(b, &a, &key);
    let first = (b.domains.get(&d).cloned(), b.reserve_reading(&d), b.origin_of(&d));
    let (mut b, again) = hear(b, &a, &key);
    assert_eq!(again, Admitted::Stale, "the head is held already");
    let head = a.chain.head().expect("acted");
    let page = a.chain.line_between(&key.verifying_bytes(), None, head.stamp.place(), 64);
    let mut budget = b.stroke();
    let _ = b.admit_backfill(&super::fixtures::page(&page), &mut budget);
    let _ = super::fixtures::admit_line(&mut b, &key, &[head]);
    assert_eq!((b.domains.get(&d).cloned(), b.reserve_reading(&d), b.origin_of(&d)), first, "nothing changed");
    assert_eq!(b.reserve_reading(&d).pending, None, "the reserve was designated once");

    let (mut c, _) = hypha("b0-once-c");
    assert!(matches!(c.admit_domain(&genesis(vec![vk(&tier_key())])), Founded::Sealed { .. }), "c founds the same genesis");
    let (c, _) = hear(c, &a, &key);
    let founding = sealed(&a, |c| matches!(c, Cite::Domain { .. }));
    assert_eq!(c.domains.get(&d), a.domains.get(&d), "the same genesis held");
    assert_eq!(c.founding_refusal(&founding.stamp), None, "and nothing refused");
    assert_eq!(c.reserve_reading(&d), a.reserve_reading(&d), "the reserve reconstructed beside it");
}

/// B0 — THE FOUNDING HYPHA'S REFUSALS: a hypha that founded the domain on
/// other tiers refuses the peer's genesis `AlreadyFounded`; one whose
/// other domain names the tier refuses it `TierHeld`; one that holds
/// another origin refuses the peer's `AlreadyOriginated`. Each refusal is
/// held by the receipt's stamp, and the registries keep what they held.
#[test]
fn backfill_refuses_with_founding_hyphas_refusals() {
    let (mut a, key) = hypha("b0-refuse-a");
    found(&mut a);
    let d = vk(&domain_key());
    let founding = sealed(&a, |c| matches!(c, Cite::Domain { .. }));
    let originating = sealed(&a, |c| matches!(c, Cite::Origin { .. }));

    let other = vk(&SigningKey::from_bytes(&[21; 32]));
    let (mut b, _) = hypha("b0-refuse-b");
    assert!(matches!(b.admit_domain(&genesis(vec![other])), Founded::Sealed { .. }), "b founds the domain on another tier");
    let (b, _) = hear(b, &a, &key);
    assert_eq!(b.founding_refusal(&founding.stamp), Some(Founded::AlreadyFounded));
    assert_eq!(b.domains.get(&d), Some(&vec![other]), "b keeps its own genesis");

    let neighbour = SigningKey::from_bytes(&[5; 32]);
    let (mut c, _) = hypha("b0-refuse-c");
    let g = DomainGenesis { domain: vk(&neighbour), tiers: vec![vk(&tier_key())] }.sign(&neighbour);
    assert!(matches!(c.admit_domain(&g), Founded::Sealed { .. }), "c founds another domain on the tier");
    let (c, _) = hear(c, &a, &key);
    assert_eq!(c.founding_refusal(&founding.stamp), Some(Founded::TierHeld));
    assert_eq!(c.domains.get(&d), None, "the peer's domain is not founded on c");
    assert_eq!(c.tier_domain.get(&vk(&tier_key())), Some(&vk(&neighbour)), "the tier stays c's neighbour's");

    let (mut e, _) = hypha("b0-refuse-e");
    assert!(matches!(e.admit_domain(&genesis(vec![vk(&tier_key())])), Founded::Sealed { .. }), "e founds the same genesis");
    assert!(matches!(e.record_origin(&origin(8)), Founded::Originated { .. }), "and records another origin");
    let held = e.origin_of(&d);
    let (e, _) = hear(e, &a, &key);
    assert_eq!(e.founding_refusal(&originating.stamp), Some(Founded::AlreadyOriginated));
    assert_eq!(e.origin_of(&d), held, "e keeps its own origin");
}
