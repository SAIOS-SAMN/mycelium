// CONCORDANCE §0, ROADMAP track 24 B2 — THE SIGNATORY RECORD: a kernel's
// signing act, judged by reconstruction, is an event on the court's receipt
// chain, admitted or refused; and the alignment reading is derived from those
// events, the latest per kernel and clause version, at every hypha that holds
// them.

use crate::anastome::dialogue::domain::{DomainGenesis, Founded, Reservation};
use crate::chronometry::{ActKind, Cite, KStepReceipt};
use crate::physics::laws::clauses::{
    history, reconstruct, register, Admitted, Assent, Choice, ClauseReading, KernelAlignment, SignedClause, Signing, SigningRefusal,
};
use crate::server::cognition::Court;
use ed25519_dalek::SigningKey;

fn domain_key() -> SigningKey {
    SigningKey::from_bytes(&[4; 32])
}

/// The kernel: the domain's one tier, and its sitting reserve.
fn kernel_key() -> SigningKey {
    SigningKey::from_bytes(&[9; 32])
}

fn vk(k: &SigningKey) -> [u8; 32] {
    k.verifying_key().to_bytes()
}

fn reads() -> crate::physics::Budget {
    crate::physics::Physics::default().stroke_n(1 << 20)
}

/// The domain founded naming the kernel as its one tier, and the kernel
/// designated its reserve.
fn found(k: &mut Court) {
    let genesis = DomainGenesis { domain: vk(&domain_key()), tiers: vec![vk(&kernel_key())] }.sign(&domain_key());
    assert!(matches!(k.hear_domain(&genesis), Founded::Sealed { .. }), "founded");
    let r = Reservation { domain: vk(&domain_key()), tier: vk(&kernel_key()), nonce: [0; 32] };
    assert!(matches!(k.designate_reserve(&r.sign(&domain_key())), Founded::Reserved { .. }), "reserved");
}

/// Every registered clause, enabled, each fixture reached as the hypha
/// reaches it.
fn enabled() -> Vec<SignedClause> {
    register()
        .expect("the register stands")
        .into_iter()
        .map(|r| SignedClause { id: r.clause.id.clone(), choice: Choice::Enabled { reached: r.clause.fixtures.iter().map(|f| reconstruct(f).expect("reached")).collect() }, fixtures: r.clause.fixtures })
        .collect()
}

/// The same clauses with the passage clause's second fixture presented as
/// its first's object: not reached.
fn unreached() -> (Vec<SignedClause>, SigningRefusal) {
    let mut clauses = enabled();
    let passage = clauses.iter_mut().find(|c| c.id.law == forms::passage::LAW_ID).expect("the passage clause");
    let Choice::Enabled { reached } = &mut passage.choice else { panic!("enabled") };
    let reconstructed = reached[1].clone();
    reached[1] = reached[0].clone();
    let refusal = SigningRefusal::NotSignatory {
        id: passage.id.clone(),
        fixture: Box::new(passage.fixtures[1].clone()),
        presented: Box::new(reached[0].clone()),
        reconstructed: Box::new(reconstructed),
    };
    (clauses, refusal)
}

fn signed(clauses: Vec<SignedClause>) -> Vec<u8> {
    Signing::sign(&kernel_key(), clauses).to_bytes()
}

/// The receipt the court sealed last, which must carry the signing.
fn head(k: &Court) -> KStepReceipt {
    k.chain.head().expect("sealed")
}

/// The one kernel's alignment as the court reads it.
fn aligned(k: &Court) -> KernelAlignment {
    let a = k.alignment(Some(vk(&kernel_key())), &mut reads()).expect("read");
    assert_eq!(a.history, history(), "the signings of the past stand beside the reading");
    let [one] = a.kernels.as_slice() else { panic!("one kernel asked") };
    one.clone()
}

/// A SIGNING ADMITTED IS AN EVENT ON THE LEDGER: a `Domain` act citing the
/// kernel, the domain it is the sitting reserve of, and each clause with its
/// assent; the alignment reads every clause as conversing at that stamp.
#[test]
fn signing_is_recorded_on_the_ledger_and_reads_converses() {
    let (mut k, _) = super::founding::hypha("b2-recorded");
    found(&mut k);
    let verdict = k.hear_signing(&signed(enabled()));
    let admitted: Vec<Admitted> = enabled().into_iter().map(|c| Admitted { id: c.id, assent: Assent::Enabled }).collect();
    assert_eq!(verdict, Ok(admitted.clone()));
    let r = head(&k);
    assert_eq!(r.kind, ActKind::Domain);
    assert!(
        super::fixtures::cites_restate(&r.cites, &[Cite::Signing { kernel: vk(&kernel_key()), reserve: Some(vk(&domain_key())), verdict: Box::new(Ok(admitted)) }]),
        "{:?}",
        r.cites
    );
    let a = aligned(&k);
    assert_eq!(a.reserve, Some(vk(&domain_key())));
    assert!(a.clauses.iter().all(|c| c.reading == ClauseReading::Converses { at: r.stamp }), "{a:?}");
    assert_eq!(a.clauses.len(), register().expect("stands").len());
}

/// A REFUSED SIGNING IS RECORDED TOO, never discarded: the `NotSignatory`
/// naming both sides is on the ledger, and the clause it names collides with
/// it as the residue, while the clause the earlier admitted act signed still
/// converses at that act's stamp.
#[test]
fn refused_signing_is_recorded_as_collision_with_residue() {
    let (mut k, _) = super::founding::hypha("b2-collides");
    found(&mut k);
    assert!(k.hear_signing(&signed(enabled())).is_ok());
    let first = head(&k).stamp;
    let (wrong, refusal) = unreached();
    assert_eq!(k.hear_signing(&signed(wrong)), Err(refusal.clone()));
    let r = head(&k);
    assert!(matches!(r.cites.as_slice(), [Cite::Signing { verdict, .. }] if **verdict == Err(refusal.clone())), "the refusal is on the ledger");
    for c in aligned(&k).clauses {
        match c.id.law.as_str() {
            forms::passage::LAW_ID => assert_eq!(c.reading, ClauseReading::Collides { at: r.stamp, residue: Box::new(refusal.clone()) }),
            _ => assert_eq!(c.reading, ClauseReading::Converses { at: first }),
        }
    }
}

/// HISTORY IS NEVER REWRITTEN: a re-signing is a new event beside the
/// refused one, and the reading takes the latest.
#[test]
fn resigning_is_a_new_event_and_the_latest_reads() {
    let (mut k, _) = super::founding::hypha("b2-resigned");
    found(&mut k);
    let (wrong, _) = unreached();
    assert!(k.hear_signing(&signed(wrong)).is_err());
    assert!(k.hear_signing(&signed(enabled())).is_ok());
    let r = head(&k);
    let events = k.chain.own().filter(|r| r.cites.iter().any(|c| matches!(c, Cite::Signing { .. }))).count();
    assert_eq!(events, 2, "both acts stand on the ledger");
    assert!(aligned(&k).clauses.iter().all(|c| c.reading == ClauseReading::Converses { at: r.stamp }));
}

/// An act whose signature does not verify is no kernel's act: recorded
/// nowhere, and the kernel reads unread on every clause.
#[test]
fn unverified_signing_is_recorded_nowhere() {
    let (mut k, _) = super::founding::hypha("b2-unsigned");
    let mut s = Signing::sign(&kernel_key(), enabled());
    s.sig[0] ^= 1;
    assert_eq!(k.hear_signing(&s.to_bytes()), Err(SigningRefusal::Unsigned));
    assert_eq!(k.hear_signing(&[]), Err(SigningRefusal::Malformed));
    assert!(k.chain.head().is_none(), "nothing sealed");
    assert!(k.alignment(None, &mut reads()).expect("read").kernels.is_empty());
    let a = aligned(&k);
    assert_eq!(a.reserve, None);
    assert!(a.clauses.iter().all(|c| c.reading == ClauseReading::Unread));
}

/// A reading cut short is never served as the whole: a stroke spent before
/// the ledger is read answers `StrokeSpent`.
#[test]
fn alignment_refuses_when_the_stroke_is_spent() {
    let (mut k, _) = super::founding::hypha("b2-stroke");
    assert!(k.hear_signing(&signed(enabled())).is_ok());
    let mut none = crate::physics::Physics::default().stroke_n(0);
    assert_eq!(k.alignment(None, &mut none), Err(crate::physics::laws::clauses::AlignmentRefusal::StrokeSpent));
}

/// THE READING IS THE MESH'S: a signing recorded on one hypha reads the same
/// on a second that holds its receipt under the first's signature.
#[test]
fn signing_replicates_to_a_second_hypha() {
    let (mut a, key) = super::founding::hypha("b2-mesh-a");
    found(&mut a);
    let (wrong, _) = unreached();
    assert!(a.hear_signing(&signed(enabled())).is_ok());
    assert!(a.hear_signing(&signed(wrong)).is_err());
    let (b, _) = super::founding::hypha("b2-mesh-b");
    let (b, _) = super::founding::hear(b, &a, &key);
    let read = b.alignment(None, &mut reads()).expect("read on b");
    assert_eq!(read, a.alignment(None, &mut reads()).expect("read on a"));
    assert_eq!(read.kernels.len(), 1);
}
