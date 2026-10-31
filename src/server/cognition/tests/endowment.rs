// 78.155b — THE HYPHA SPONSORS A UNIVERSE'S PACKAGE: a manifest of nuclei per
// Z answered by a grant from what the court has earned, never a mint.

use crate::anastome::dialogue::domain::{decode_domain_request, DomainGenesis, DomainRequest, Founded, Origin, Reservation};
use crate::anastome::dialogue::endowment::{
    decode_endowment_reading, decode_grant_reply, encode_endowment_read, encode_endowment_reading, encode_grant_reply, encode_manifest, GrantRefusal, Granted, Manifest, SignedManifest,
};
use crate::server::cognition::Court;
use ed25519_dalek::SigningKey;
use num_bigint::BigUint;

fn key(b: u8) -> SigningKey {
    SigningKey::from_bytes(&[b; 32])
}

fn vk(k: &SigningKey) -> [u8; 32] {
    k.verifying_key().to_bytes()
}

const DOMAIN: u8 = 7;
const RESERVE: u8 = 27;

/// A domain founded with its reserve designated; originated where `origin`.
fn universe(origin: bool) -> Court {
    let mut k = Court::new();
    let mut tiers = vec![vk(&key(RESERVE)), vk(&key(47))];
    tiers.sort_unstable();
    let g = DomainGenesis { domain: vk(&key(DOMAIN)), tiers };
    assert!(matches!(k.admit_domain(&g.sign(&key(DOMAIN))), Founded::Sealed { .. }));
    let r = Reservation { domain: vk(&key(DOMAIN)), tier: vk(&key(RESERVE)), nonce: [0; 32] };
    assert!(matches!(k.designate_reserve(&r.sign(&key(DOMAIN))), Founded::Reserved { .. }));
    if origin {
        let o = Origin { domain: vk(&key(DOMAIN)), kind: 1, version: 1, commitment: [DOMAIN; 32], universe: None };
        assert!(matches!(k.record_origin(&o.sign(&key(DOMAIN))), Founded::Originated { .. }));
    }
    k
}

/// A package of hydrogen, carbon and iron nuclei, counted astronomically.
fn manifest(nonce: u8) -> Manifest {
    let big = |e: u32| BigUint::from(10u8).pow(e);
    Manifest { domain: vk(&key(DOMAIN)), nuclei: vec![(1, big(60)), (6, big(58)), (26, big(55))], nonce: [nonce; 32] }
}

fn signed(m: Manifest, by: u8) -> SignedManifest {
    m.sign(&key(by)).expect("the manifest writes")
}

/// THE MANIFEST AND ITS READING TRAVEL WHOLE: a manifest of astronomical
/// counts reads back as itself off the domain door; the grant's reply and the
/// endowment's reading likewise. A count that is no positive integer is no
/// manifest.
#[test]
fn manifest_and_reading_round_trip_on_wire() {
    let s = signed(manifest(1), RESERVE);
    let bytes = encode_manifest(&s).expect("it writes");
    assert_eq!(decode_domain_request(&bytes), Some(DomainRequest::Manifest(Box::new(s.clone()))));
    assert_eq!(decode_domain_request(&encode_endowment_read(vk(&key(DOMAIN)))), Some(DomainRequest::ReadEndowment(vk(&key(DOMAIN)))));
    let refused = Granted::Refused(GrantRefusal::InFlight);
    assert_eq!(decode_grant_reply(&encode_grant_reply(&refused)), Some(refused));
    let mut k = universe(true);
    let anchor = k.anchor();
    let court_vk = k.identity;
    k.treasury.genesis(court_vk, [100, 100, 100, 100], anchor);
    let granted = k.manifest(&s);
    assert_eq!(decode_grant_reply(&encode_grant_reply(&granted)), Some(granted));
    let reading = k.endowment_reading(&vk(&key(DOMAIN))).expect("reads");
    assert_eq!(decode_endowment_reading(&encode_endowment_reading(&reading).expect("writes")), Some(reading));
}

/// THE FOUNDING GRANT, FROM WHAT THE COURT EARNED. A universe's first
/// manifest is granted the court's read price for one unit of gas per lane
/// per kind of nuclei it carries — moved from the court's balance to the
/// reserve, never minted — and the reading says so: founding granted, the
/// universe's nuclei those of its manifest. A second manifest waits while
/// the first is not applied in the universe's ledger; the same manifest is
/// never granted twice.
#[test]
fn manifest_grants_founding_from_court_balance() {
    let mut k = universe(true);
    let anchor = k.anchor();
    let court_vk = k.identity;
    k.treasury.genesis(court_vk, [100, 100, 100, 100], anchor);
    let reserve = vk(&key(RESERVE));
    let price = k.read_price().expect("a price");
    let before = (k.treasury.balance_mu(&court_vk), k.treasury.balance_mu(&reserve));
    let m = manifest(1);
    let Granted::Sealed { mu, founding, .. } = k.manifest(&signed(m.clone(), RESERVE)) else { panic!("granted") };
    assert!(founding, "the first is the founding grant");
    assert_eq!(mu, price.map(|p| p * 3), "three kinds of nuclei: three units of gas per lane at the read price");
    let after = (k.treasury.balance_mu(&court_vk), k.treasury.balance_mu(&reserve));
    assert_eq!(after.0, std::array::from_fn(|l| before.0[l] - mu[l]), "out of the court's earned balance");
    assert_eq!(after.1, std::array::from_fn(|l| before.1[l] + mu[l]), "into the reserve: a transfer, never a mint");
    let r = k.endowment_reading(&reserve_domain()).expect("reads");
    assert_eq!(r.founding, mu);
    assert_eq!(r.nuclei, m.nuclei, "the universe holds its manifest's nuclei");
    assert!(matches!(r.grants.as_slice(), [g] if g.founding && !g.applied));
    assert_eq!(k.manifest(&signed(m, RESERVE)), Granted::Refused(GrantRefusal::Granted), "never twice");
    assert_eq!(k.manifest(&signed(manifest(2), RESERVE)), Granted::Refused(GrantRefusal::InFlight), "growth waits on the founding grant's application");
}

fn reserve_domain() -> [u8; 32] {
    vk(&key(DOMAIN))
}

/// A GRANT IS REFUSED BY NAME: signed by another key than the sitting
/// reserve; a domain with no origin; nuclei unordered or of a zero count; and
/// a court that has earned too little — nothing moved, nothing minted.
#[test]
fn manifest_refuses_grant_by_name() {
    let mut unoriginated = universe(false);
    assert_eq!(unoriginated.manifest(&signed(manifest(1), RESERVE)), Granted::Refused(GrantRefusal::NotAUniverse));
    let mut k = universe(true);
    assert_eq!(k.manifest(&signed(manifest(1), DOMAIN)), Granted::Refused(GrantRefusal::Unsigned), "the domain key is not the sitting reserve");
    let mut unordered = manifest(1);
    unordered.nuclei.reverse();
    assert_eq!(k.manifest(&signed(unordered, RESERVE)), Granted::Refused(GrantRefusal::Malformed));
    let mut zero = manifest(1);
    zero.nuclei[0].1 = BigUint::from(0u8);
    assert_eq!(k.manifest(&signed(zero, RESERVE)), Granted::Refused(GrantRefusal::Malformed));
    assert_eq!(k.manifest(&signed(manifest(1), RESERVE)), Granted::Refused(GrantRefusal::Short), "a court that earned nothing grants nothing");
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
    let mut booted = Court::new();
    assert!(booted.restore(&snap), "clean restore");
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
    let rule = crate::transducer::element::from_residual(&vec![(0i64, 0i64, 1u8, 2u8)]).expect("a rule");
    let line = vk(&key(47));
    let _ = k.pay_solve(b"task-a", line, &rule, [3, 0, 2, 1], true);
    assert_eq!(k.growth_size(&domain), Ok([3, 0, 2, 1]), "the closure's measured discharge, a line's key counting for its universe");
    let _ = k.pay_solve(b"task-b", [0xee; 32], &rule, [9, 9, 9, 9], true);
    assert_eq!(k.growth_size(&domain), Ok([3, 0, 2, 1]), "a key no domain names grows nothing here");
    let _ = k.pay_solve(b"task-c", vk(&key(RESERVE)), &rule, [1, 1, 0, 0], true);
    assert_eq!(k.growth_size(&domain), Ok([4, 1, 2, 1]), "a second closure adds, exact per lane");
    let mut booted = Court::new();
    assert!(booted.restore(&k.snapshot()), "clean restore");
    assert_eq!(booted.growth_size(&domain), Ok([4, 1, 2, 1]), "the size survives a restart");
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
    let line = vk(&key(47));
    let anchor = k.anchor();
    let _ = k.treasury.settle_convergence(line, [2, 0, 0, 1], anchor);
    k.grow_grants_by_paid();
    assert_eq!(k.growth_size(&domain), Ok([2, 0, 0, 1]), "a POWC mint to a key of the universe");
    k.treasury.mint_claim(&[line, [0xee; 32]], [4, 2, 0, 0], anchor);
    k.grow_grants_by_paid();
    assert_eq!(k.growth_size(&domain), Ok([4, 1, 0, 1]), "a claim split two ways: the universe's key's half, exactly");
    let _ = k.treasury.settle_convergence([0xee; 32], [9, 9, 9, 9], anchor);
    k.grow_grants_by_paid();
    assert_eq!(k.growth_size(&domain), Ok([4, 1, 0, 1]), "a key no domain names grows nothing here");
}
