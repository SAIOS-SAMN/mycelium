// 78.155b — THE HYPHA SPONSORS A UNIVERSE'S PACKAGE: a manifest of nuclei per
// Z answered by a grant from what the court has earned, never a mint.

use crate::anastome::dialogue::domain::{decode_domain_request, DomainGenesis, BigBangRecord, DomainRequest, Founded, Reservation};
use crate::anastome::dialogue::endowment::{
    decode_endowment_reading, decode_grant_reply, encode_endowment_read, encode_endowment_reading, encode_grant_reply, encode_manifest, GrantRefusal, Granted, Manifest, SignedManifest,
};
use crate::economy::tests::Counts;
use crate::server::cognition::Court;
use ed25519_dalek::SigningKey;
use num_bigint::BigUint;

fn key(b: u8) -> SigningKey {
    SigningKey::from_bytes(&[b; 32])
}

fn verifying(k: &SigningKey) -> [u8; 32] {
    k.verifying_key().to_bytes()
}

const DOMAIN: u8 = 7;
const RESERVE: u8 = 27;

/// A domain founded with its reserve designated; originated where `origin`.
fn universe(origin: bool) -> Court {
    let mut k = Court::new();
    let mut tiers = vec![verifying(&key(RESERVE)), verifying(&key(47))];
    tiers.sort_unstable();
    let g = DomainGenesis { domain: verifying(&key(DOMAIN)), tiers };
    assert!(matches!(k.hear_domain(&g.sign(&key(DOMAIN))), Founded::Sealed { .. }));
    let r = Reservation { domain: verifying(&key(DOMAIN)), tier: verifying(&key(RESERVE)), nonce: [0; 32] };
    assert!(matches!(k.designate_reserve(&r.sign(&key(DOMAIN))), Founded::Reserved { .. }));
    super::boundary::signs(&mut k, &key(RESERVE));
    if origin {
        let o = BigBangRecord { domain: verifying(&key(DOMAIN)), kind: 1, version: 1, commitment: [DOMAIN; 32], universe: None };
        assert!(matches!(k.record_origin(&o.sign(&key(DOMAIN))), Founded::Originated { .. }));
    }
    k
}

/// One nucleus of `z` protons and `q` neutrons.
fn nucleus(z: u16, q: u32) -> crate::anastome::dialogue::endowment::Nucleus {
    crate::anastome::dialogue::endowment::Nucleus { z, q }
}

/// A package of hydrogen, carbon and iron nuclei, counted astronomically.
fn manifest(nonce: u8) -> Manifest {
    let big = |e: u32| BigUint::from(10u8).pow(e);
    Manifest { domain: verifying(&key(DOMAIN)), nuclei: vec![(nucleus(1, 0), big(60)), (nucleus(6, 6), big(58)), (nucleus(26, 30), big(55))], nonce: [nonce; 32] }
}

fn signed(m: Manifest, by: u8) -> SignedManifest {
    m.sign(crate::anastome::dialogue::crossing_record::still(), &key(by)).expect("the manifest writes")
}

/// THE MANIFEST AND ITS READING TRAVEL WHOLE: a manifest of astronomical
/// counts reads back as itself off the domain door; the grant's reply and the
/// endowment's reading likewise. A count that is no positive integer is no
/// manifest.
#[test]
fn manifest_round_trips_with_reading() {
    let s = signed(manifest(1), RESERVE);
    let bytes = encode_manifest(&s).expect("it writes");
    assert_eq!(decode_domain_request(&bytes), Some(DomainRequest::Manifest(Box::new(s.clone()))));
    assert_eq!(decode_domain_request(&encode_endowment_read(verifying(&key(DOMAIN)))), Some(DomainRequest::ReadEndowment(verifying(&key(DOMAIN)))));
    let refused = Granted::Refused(GrantRefusal::InFlight);
    assert_eq!(decode_grant_reply(&encode_grant_reply(&refused)), Some(refused));
    let mut k = universe(true);
    let anchor = k.anchor();
    let court_vk = k.identity;
    k.treasury.genesis(court_vk, [100, 100, 100, 100], anchor);
    let granted = k.manifest(&s);
    assert_eq!(decode_grant_reply(&encode_grant_reply(&granted)), Some(granted));
    let reading = k.endowment_reading(&verifying(&key(DOMAIN))).expect("reads");
    assert_eq!(decode_endowment_reading(&encode_endowment_reading(&reading).expect("writes")), Some(reading));
}

/// THE FOUNDING GRANT, CREATED AGAINST THE ELEMENTAL ROWS (78.184, ruled
/// 2026-11-13; MU-EQUATIONS §6.13). A universe's first manifest creates
/// `Σ n · row(Z, Q)` for the nuclei it carries, scaled by the kernel's
/// alignment share, and mints it to the reserve; the court's earned balance
/// is not drawn. The reading says so: founding granted, the universe's
/// nuclei those of its manifest. A second manifest waits while the first is
/// not applied in the universe's ledger; the same manifest is never granted
/// twice. (Until 2026-11-13 the founding grant was `kinds · p_l`, a transfer
/// from what the court earned, refused `Short` on a fresh mesh.)
#[test]
fn manifest_creates_founding_against_elemental_rows() {
    let mut k = universe(true);
    let anchor = k.anchor();
    let court_vk = k.identity;
    k.treasury.genesis(court_vk, [100, 100, 100, 100], anchor);
    let reserve = verifying(&key(RESERVE));
    let before = (k.treasury.balance_mu(&court_vk), k.treasury.balance_mu(&reserve));
    let m = manifest(1);
    let share = k.alignment_share(&reserve_domain()).expect("the alignment reads");
    let created = k.elemental_creation(&m.nuclei).expect("every nucleus has a row");
    let Granted::Sealed { mu, founding, .. } = k.manifest(&signed(m.clone(), RESERVE)) else { panic!("granted") };
    assert!(founding, "the first is the founding grant");
    assert_eq!(mu, crate::economy::Mu::of_modes(created.scaled(&share).expect("scales")), "the creation, scaled by the alignment share");
    assert!(!mu.is_zero(), "a manifest of nuclei creates");
    let after = (k.treasury.balance_mu(&court_vk), k.treasury.balance_mu(&reserve));
    assert_eq!(after.0, before.0, "the court's earned balance is not drawn");
    assert_eq!(after.1, before.1.add(&mu), "into the reserve: created, a mint");
    let r = k.endowment_reading(&reserve_domain()).expect("reads");
    assert_eq!(r.founding, mu);
    assert_eq!(r.nuclei, m.nuclei, "the universe holds its manifest's nuclei");
    assert!(matches!(r.grants.as_slice(), [g] if g.founding && !g.applied));
    assert_eq!(k.manifest(&signed(m, RESERVE)), Granted::Refused(GrantRefusal::Granted), "never twice");
    assert_eq!(k.manifest(&signed(manifest(2), RESERVE)), Granted::Refused(GrantRefusal::InFlight), "growth waits on the founding grant's application");
}

fn reserve_domain() -> [u8; 32] {
    verifying(&key(DOMAIN))
}

/// A GRANT IS REFUSED BY NAME: signed by another key than the sitting
/// reserve; a domain with no origin; nuclei unordered or of a zero count —
/// nothing moved, nothing minted. A court that has earned nothing still
/// grants the founding creation (78.184): the fresh mesh's `Short` is gone.
#[test]
fn manifest_refuses_unsigned_malformed_or_short_grant() {
    let mut unoriginated = universe(false);
    assert_eq!(unoriginated.manifest(&signed(manifest(1), RESERVE)), Granted::Refused(GrantRefusal::NotAUniverse));
    let mut k = universe(true);
    let off_core = crate::court::admission::Boundary::NotThroughCore { act: crate::court::admission::UniverseAct::Manifest, domain: verifying(&key(DOMAIN)) };
    assert_eq!(k.manifest(&signed(manifest(1), DOMAIN)), Granted::Boundary(off_core), "the domain key is not the sitting reserve");
    assert_eq!(k.manifest(&signed(manifest(1), 77)), Granted::Refused(GrantRefusal::Unsigned), "no key of the domain");
    let mut unordered = manifest(1);
    unordered.nuclei.reverse();
    assert_eq!(k.manifest(&signed(unordered, RESERVE)), Granted::Refused(GrantRefusal::Malformed));
    let mut zero = manifest(1);
    zero.nuclei[0].1 = BigUint::from(0u8);
    assert_eq!(k.manifest(&signed(zero, RESERVE)), Granted::Refused(GrantRefusal::Malformed));
    assert!(matches!(k.manifest(&signed(manifest(1), RESERVE)), Granted::Sealed { founding: true, .. }), "a court that earned nothing creates the founding grant");
}

/// THE GRANTS SURVIVE A RESTART: written on the court's own node and read
/// back whole, the reading the same after as before.
#[test]
fn endowment_reading_survives_restart() {
    let mut k = universe(true);
    let anchor = k.anchor();
    let court_vk = k.identity;
    k.treasury.genesis(court_vk, [100, 100, 100, 100], anchor);
    assert!(matches!(k.manifest(&signed(manifest(1), RESERVE)), Granted::Sealed { .. }));
    let before = k.endowment_reading(&reserve_domain());
    let snap = k.snapshot();
    let booted = Court::new().reconstruct(&snap).expect("a clean reconstruction").into_object();
    let after = booted.endowment_reading(&reserve_domain());
    assert_eq!(after.as_ref().map(|r| (&r.founding, &r.grants, &r.nuclei)), before.as_ref().map(|r| (&r.founding, &r.grants, &r.nuclei)));
}

/// 78.155c — A GROWTH GRANT IS SIZED BY THE CLOSURES JUDGED IN ITS UNIVERSE
/// (ruled 2026-10-14). Before any closure there is no growth grant. A
/// first closure by a key of the domain — any key it registered, membership
/// never tier — grows the next growth grant by exactly its measured discharge;
/// a closure by a key no domain names grows nothing; a second adds to it.
/// The size survives a restart, and a grant sealed takes it whole: the next
/// growth grant counts only closures judged after it.
#[test]
fn growth_size_sums_closures_in_universe() {
    let mut k = universe(true);
    let domain = reserve_domain();
    assert_eq!(k.growth_size(&domain), Err(GrantRefusal::NoNewClosures), "no closure judged: no growth grant");
    let rule = crate::transducer::element::from_residue(&vec![crate::transducer::sentence::tests::fixtures::link(0, 0, 1, 2)]).expect("a rule");
    let line = verifying(&key(47));
    let _ = k.pay_solve(b"task-a", line, &rule, [3, 0, 2, 1].into(), true);
    assert_eq!(k.growth_size(&domain).n(), Ok([3, 0, 2, 1]), "the closure's measured discharge, a line's key counting for its universe");
    let _ = k.pay_solve(b"task-b", [0xee; 32], &rule, [9, 9, 9, 9].into(), true);
    assert_eq!(k.growth_size(&domain).n(), Ok([3, 0, 2, 1]), "a key no domain names grows nothing here");
    let _ = k.pay_solve(b"task-c", verifying(&key(RESERVE)), &rule, [1, 1, 0, 0].into(), true);
    assert_eq!(k.growth_size(&domain).n(), Ok([4, 1, 2, 1]), "a second closure adds, exact per lane");
    let booted = Court::new().reconstruct(&k.snapshot()).expect("a clean reconstruction").into_object();
    assert_eq!(booted.growth_size(&domain).n(), Ok([4, 1, 2, 1]), "the size survives a restart");
    // A grant sealed takes the closures it answered.
    let anchor = k.anchor();
    let court_vk = k.identity;
    k.treasury.genesis(court_vk, [100, 100, 100, 100], anchor);
    assert!(matches!(k.manifest(&signed(manifest(1), RESERVE)), Granted::Sealed { founding: true, .. }));
    assert_eq!(k.growth_size(&domain), Err(GrantRefusal::NoNewClosures), "the closures before a grant are its own");
}

/// 78.155c AS AMENDED (MU-CLIENT §5.6, E4) — WHAT THE COURTS PAID GROWS THE
/// GRANT, not only first solves: a POWC mint (a demand closed by another
/// mind) and a POUW claim released to two voices (a word's install, split
/// between its speaker and the mind that agreed) each grow the next growth
/// grant of the universe whose key was paid, by exactly what that key was
/// paid; what a key no domain names was paid grows nothing.
#[test]
fn grow_grants_by_paid_grows_universe_grant() {
    let mut k = universe(true);
    let domain = reserve_domain();
    let line = verifying(&key(47));
    let anchor = k.anchor();
    let _ = k.treasury.settle_convergence(line, [2, 0, 0, 1], anchor);
    k.grow_grants_by_paid();
    assert_eq!(k.growth_size(&domain).n(), Ok([2, 0, 0, 1]), "a POWC mint to a key of the universe");
    k.treasury.mint_claim(&[line, [0xee; 32]], [4, 2, 0, 0], anchor);
    k.grow_grants_by_paid();
    assert_eq!(k.growth_size(&domain).n(), Ok([4, 1, 0, 1]), "a claim split two ways: the universe's key's half, exactly");
    let _ = k.treasury.settle_convergence([0xee; 32], [9, 9, 9, 9], anchor);
    k.grow_grants_by_paid();
    assert_eq!(k.growth_size(&domain).n(), Ok([4, 1, 0, 1]), "a key no domain names grows nothing here");
}
