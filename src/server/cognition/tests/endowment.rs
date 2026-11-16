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
fn nucleus(z: u16, q: u16) -> crate::anastome::dialogue::endowment::Nucleus {
    crate::anastome::dialogue::endowment::Nucleus::of(z, q)
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
    assert_eq!(decode_domain_request(&encode_endowment_read(verifying(&key(DOMAIN)))), Some(DomainRequest::ReadEndowment(verifying(&key(DOMAIN)), 0)));
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
    let held = k.grants.get(&reserve_domain()).and_then(|g| g.iter().find(|g| g.founding).cloned()).expect("held");
    assert_eq!(held.allotment, crate::economy::Mu::of_modes(created.scaled(&share).expect("scales")), "the allotment, scaled by the alignment share");
    assert!(!held.allotment.is_zero(), "a manifest of nuclei creates an allotment");
    assert!(mu.is_zero(), "delivered per final point (78.184 §5), nothing at the manifest");
    let after = (k.treasury.balance_mu(&court_vk), k.treasury.balance_mu(&reserve));
    assert_eq!(after.0, before.0, "the court's earned balance is not drawn");
    assert_eq!(after.1, before.1, "the reserve receives nothing until the first beat");
    let r = k.endowment_reading(&reserve_domain()).expect("reads");
    assert_eq!(r.founding, mu, "the founding read is what was delivered: nothing yet");
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

/// #332 — A READING PAST ONE PAGE reads whole across its pages: each page
/// carries the reading's hash and its offset, and the pages assemble into
/// the reading the court holds. A reading that changes between pages is
/// caught by the hash and read again from the first page.
#[test]
fn endowment_reading_pages_past_the_wire_budget_and_assembles() {
    use crate::anastome::dialogue::endowment::{decode_domain_offset, encode_endowment_page, encode_endowment_reading, read_endowment, EndowmentReading, Nucleus, ENDOWMENT_PAGE};
    let many: Vec<(Nucleus, BigUint)> = (1..=60000u32).map(|i| (Nucleus::of(u16::try_from(1 + i / 200).expect("fits"), u16::try_from(i % 200).expect("fits")), BigUint::from(10u8).pow(40))).collect();
    let r = EndowmentReading { nuclei: many, ..Default::default() };
    let whole = encode_endowment_reading(&r).expect("encodes");
    assert!(whole.len() > ENDOWMENT_PAGE, "past one page: {} bytes", whole.len());
    let domain = verifying(&key(DOMAIN));
    let mut asks = 0usize;
    let read = read_endowment(domain, |ask| {
        asks += 1;
        encode_endowment_page(&whole, decode_domain_offset(&ask)?)
    });
    assert_eq!(read, Some(r.clone()), "assembled whole");
    assert!(asks >= 2, "more than one page asked");
    // A reading that moves after its first page: the hash catches it, and
    // the second pass reads the reading that stands.
    let mut moved = r.clone();
    moved.nuclei.pop();
    let later = encode_endowment_reading(&moved).expect("encodes");
    let mut first = true;
    let read = read_endowment(domain, |ask| {
        let offset = decode_domain_offset(&ask)?;
        let src = match (first, offset) {
            (true, 0) => &whole,
            _ => {
                first = false;
                &later
            }
        };
        encode_endowment_page(src, offset)
    });
    assert_eq!(read, Some(moved), "the changed reading is read again from its first page");
}

/// 78.184 §8 — MANIFEST v4: nucleons in whole thirds, a kernel's pattern
/// counts beside them, and the phase state as planes in their composition
/// order, each `(plane, K, R_p, σ_p)`. An entry reads back exactly; one
/// isotope at two phase states is two entries, ordered apart. A plane is at
/// rest when `R_p = −σ_p`, and the axis plane `0`, where stated, has the
/// nucleon split as its pole. A phase past one turn is no entry.
#[test]
fn manifest_v4_carries_thirds_patterns_and_the_phase_state_per_plane() {
    use crate::anastome::dialogue::endowment::{nuclei_hold, Nucleus, Plane};
    let q = |n: i64, d: i64| crate::core::q::Q::new(n.into(), d.into());
    let plane = |id: u16, k: u64, phase, pole| Plane { plane: id, k, phase, pole };
    // Iron-56 on the axis plane: pole (26 − 30)/56 = −1/14, rest phase 1/14.
    let rest = Nucleus { planes: vec![plane(0, 0, q(1, 14), q(-1, 14))], ..Nucleus::of(26, 30) };
    let left = Nucleus::of(26, 30);
    assert!(rest.at_rest(), "R_p = −σ_p on the axis plane, its pole the nucleon split");
    assert!(left.at_rest(), "no planes: the identity, left to the kernel");
    assert!(!Nucleus { planes: vec![plane(0, 0, q(1, 7), q(-1, 7))], ..Nucleus::of(26, 30) }.at_rest(), "the axis plane's pole is the nucleon split, never stated past it");
    assert!(!Nucleus { planes: vec![plane(0, 0, q(1, 7), q(-1, 14))], ..Nucleus::of(26, 30) }.at_rest(), "a phase off its pole's antipole");
    // A kernel's three planes sharing an axis, in its own order, nucleons in
    // thirds and a pattern beside them; the order is part of the key.
    let thirds = Nucleus { z3: 79, q3: 90, pattern: vec![1, 0, 2, 0, 0, 1, 0, 0], planes: vec![plane(1, 2, q(1, 3), q(-1, 3)), plane(2, 0, q(-1, 5), q(1, 5)), plane(3, 1, q(0, 1), q(0, 1))] };
    assert!(thirds.at_rest(), "every plane at its pole's antipole; no axis plane stated");
    assert_eq!(thirds.z(), q(79, 3), "Z = F + Σℓ/3, exact");
    let reordered = Nucleus { planes: thirds.planes.iter().rev().cloned().collect(), ..thirds.clone() };
    assert_ne!(thirds, reordered, "the composition order is part of the key");
    let image = thirds.image();
    assert_eq!((image.z3, image.q3, image.nucleons3()), (90, 79, thirds.nucleons3()), "the pole swap keeps 3A");
    assert!(image.at_rest(), "the image of an entry at rest is at rest");
    assert_eq!(image.image(), thirds, "the image is the reverse, its own inverse on one tube");
    let mut m = manifest(9);
    // Each entry entered at its place in the key's order (rule 4: no sort).
    let mut nuclei: Vec<(Nucleus, BigUint)> = Vec::new();
    for e in [(left.clone(), BigUint::from(2u8)), (rest.clone(), BigUint::from(3u8)), (thirds.clone(), BigUint::from(10u8).pow(60)), (reordered.clone(), BigUint::from(1u8))] {
        let at = nuclei.partition_point(|(k, _)| *k < e.0);
        nuclei.insert(at, e);
    }
    m.nuclei = nuclei;
    assert!(nuclei_hold(&m.nuclei), "ascending by the whole key");
    let back = crate::anastome::dialogue::domain::decode_domain_request(&crate::anastome::dialogue::endowment::encode_manifest(&signed(m.clone(), RESERVE)).expect("encodes"));
    assert!(matches!(back, Some(crate::anastome::dialogue::domain::DomainRequest::Manifest(s)) if s.manifest == m), "every entry reads back exactly");
    let mut over = m.clone();
    over.nuclei = vec![(Nucleus { planes: vec![plane(1, 0, q(3, 2), q(-3, 2))], ..Nucleus::of(1, 0) }, BigUint::from(1u8))];
    assert!(!nuclei_hold(&over.nuclei), "a phase past one turn is no entry");
}

/// 78.184 §8 — A MANIFEST PAST ONE WIRE BODY ARRIVES IN PAGES: 15,000 entries
/// cut into pages that each fit the budget, the court holding them in order
/// (`Paged`) and judging the manifest once the last lands. A page out of
/// order is refused, and nothing is granted.
#[test]
fn manifest_past_one_body_is_granted_from_its_pages() {
    use crate::anastome::dialogue::domain::{decode_domain_request, DomainRequest};
    use crate::anastome::dialogue::endowment::{encode_manifest_pages, Nucleus, Plane};
    let q = |n: i64, d: i64| crate::core::q::Q::new(n.into(), d.into());
    let mut m = manifest(5);
    m.nuclei = (1u16..=3000)
        .flat_map(|z| (0u16..5).map(move |n| (z, z + n)))
        .map(|(z, n)| (Nucleus { pattern: vec![u32::from(z); 8], planes: vec![Plane { plane: 1, k: 1, phase: q(1, 3), pole: q(-1, 3) }], ..Nucleus::of(z, n) }, BigUint::from(10u8).pow(40)))
        .collect();
    let s = signed(m, RESERVE);
    let pages = encode_manifest_pages(&s).expect("pages");
    assert!(pages.len() > 1, "past one body: {} pages", pages.len());
    assert!(pages.iter().all(|p| p.len() <= crate::network::wire::BODY_BUDGET), "each page within the budget");
    let page = |b: &[u8]| match decode_domain_request(b) {
        Some(DomainRequest::ManifestPage(d, p)) => (d, p),
        other => panic!("a page: {other:?}"),
    };
    let mut k = universe(true);
    let (d1, p1) = page(&pages[1]);
    assert_eq!(k.manifest_page(&d1, &p1), Granted::Refused(GrantRefusal::Malformed), "a page out of order");
    let mut last = None;
    for (i, b) in pages.iter().enumerate() {
        let (d, p) = page(b);
        let g = k.manifest_page(&d, &p);
        match i + 1 == pages.len() {
            false => assert!(matches!(g, Granted::Paged { .. }), "held: {g:?}"),
            true => last = Some(g),
        }
    }
    assert!(matches!(last, Some(Granted::Sealed { founding: true, .. })), "judged once whole: {last:?}");
    assert_eq!(k.grants.get(&reserve_domain()).map(|g| g[0].manifest.manifest.nuclei.len()), Some(15000), "every entry held");
}

/// 78.184 §5 — DELIVERY PER FINAL POINT. A founding grant mints nothing at
/// the manifest: its allotment is delivered along TUniversal's temporal
/// plane. The first final point with `T > 0` fixes `T₀` and delivers
/// nothing; then `d_T = 1 − T₀/T` of the allotment has arrived by each
/// point: half at `T = 2T₀`, three quarters at `4T₀`. A point read twice
/// delivers nothing more, and a point with `T = 0` delivers nothing.
#[test]
fn founding_allotment_is_delivered_per_final_point_along_the_temporal_plane() {
    use crate::chronometry::tuniversal::{Origin, TPoint};
    use crate::economy::Mu;
    let q = |n: i64, d: i64| crate::core::q::Q::new(n.into(), d.into());
    let at = |h: u64, t: crate::core::q::Q| TPoint {
        origin: Origin { genesis: [7; 32], epoch: 0 },
        h,
        lanes: Mu::of([q(0, 1), q(0, 1), t, q(0, 1)]).expect("lanes"),
        k: 0,
    };
    let mut k = universe(true);
    let reserve = verifying(&key(RESERVE));
    let before = k.treasury.balance_mu(&reserve);
    let Granted::Sealed { mu, founding, .. } = k.manifest(&signed(manifest(1), RESERVE)) else { panic!("granted") };
    assert!(founding);
    assert!(mu.is_zero(), "nothing is minted at the manifest");
    assert_eq!(k.treasury.balance_mu(&reserve), before, "the reserve holds nothing yet");
    let allotment = k.grants.get(&reserve_domain()).and_then(|g| g.iter().find(|g| g.founding).map(|g| g.allotment.clone())).expect("an allotment");
    assert!(!allotment.is_zero(), "the allotment is created against the nuclei");
    assert!(k.deliver_at(&at(10, q(0, 1))).is_empty(), "T = 0: no temporal work, nothing delivered");
    assert!(k.deliver_at(&at(11, q(3, 1))).is_empty(), "the first T > 0 fixes T₀ and delivers nothing");
    let half = k.deliver_at(&at(12, q(6, 1)));
    assert_eq!(half.len(), 1);
    assert_eq!(Some(half[0].1.clone()), allotment.scaled(&q(1, 2)), "T = 2T₀: half the allotment");
    let more = k.deliver_at(&at(13, q(12, 1)));
    assert_eq!(Some(more[0].1.clone()), allotment.scaled(&q(1, 4)), "T = 4T₀: three quarters in all, a quarter this beat");
    assert!(k.deliver_at(&at(14, q(12, 1))).is_empty(), "a point with T unmoved delivers nothing more");
    assert_eq!(k.treasury.balance_mu(&reserve), before.add(&allotment.scaled(&q(3, 4)).expect("scales")), "three quarters held by the reserve");
    // The endowment read carries what was delivered on the grant itself.
    let read = k.endowment_reading(&reserve_domain()).expect("a reading");
    assert_eq!(Some(read.grants[0].mu.clone()), allotment.scaled(&q(3, 4)), "the grant reads the three quarters delivered");
}

/// 78.155b — A GRANT IS APPLIED BY THE UNIVERSE'S OWN LEDGER: an entry signed
/// by the sitting reserve lists the grant's hash in its `applied`, as a
/// pending crossing is listed; the reading then says applied, the same grant
/// listed again is refused, and the growth manifest the founding held back
/// is admitted.
#[test]
fn ledger_entry_applies_a_grant_and_admits_the_growth_manifest() {
    use crate::anastome::dialogue::atlas::{LedgerEntry, LedgerRefusal, Ledgered};
    use crate::anastome::dialogue::crossing_record::SignedStill;
    use crate::chronometry::tuniversal::{Origin, TPoint};
    let fp = |h: u64| TPoint { origin: Origin { genesis: [9; 32], epoch: 0 }, h, lanes: [h, 0, 0, 0].into(), k: h };
    let mut k = universe(true);
    let domain = reserve_domain();
    assert!(matches!(k.manifest(&signed(manifest(1), RESERVE)), Granted::Sealed { founding: true, .. }));
    let grant = k.endowment_reading(&domain).expect("a reading").grants[0].manifest;
    assert!(matches!(k.manifest(&signed(manifest(2), RESERVE)), Granted::Refused(GrantRefusal::InFlight)), "the founding is not applied yet");
    let entry = |k: &Court, u: u64, applied: Vec<([u8; 32], u64)>| {
        let parent = k.atlas.head(&domain).map_or([0; 32], |e| e.hash);
        LedgerEntry { domain, u, commitment: [1; 32], events: [2; 32], point: fp(u + 1), parent, applied, nonce: [u8::try_from(u).expect("a small cycle"); 32] }.signed(&key(RESERVE))
    };
    let first = entry(&k, 1, vec![(grant, 1)]);
    assert!(matches!(k.ledger(&first, Some(fp(2))), Ledgered::Sealed { .. }), "the grant applied as a crossing is");
    assert!(k.endowment_reading(&domain).expect("a reading").grants[0].applied);
    let again = entry(&k, 2, vec![(grant, 2)]);
    assert_eq!(k.ledger(&again, Some(fp(3))), Ledgered::Refused(LedgerRefusal::Applied), "applied once");
    assert!(matches!(k.manifest(&signed(manifest(2), RESERVE)), Granted::Sealed { founding: false, .. }), "the growth manifest is admitted");
}

/// 78.184 §5 (ruled 2026-11-16, #348) — A STATED PHASE OFF ITS REST POINT IS
/// REFUSED BY NAME, nothing granted: iron (26, 30) rests at `R = 1/14`, so
/// `R = 1/7` is refused `NotAtRest`, while the rest point itself and `R = 0`
/// are granted.
#[test]
fn manifest_refuses_a_phase_off_its_rest_point() {
    use crate::anastome::dialogue::endowment::Nucleus;
    let q = |n: i64, d: i64| crate::core::q::Q::new(n.into(), d.into());
    let entry = |r: crate::core::q::Q| {
        use num_traits::Zero;
        let planes = match r.is_zero() {
            true => Vec::new(),
            false => vec![crate::anastome::dialogue::endowment::Plane { plane: 0, k: 0, phase: r, pole: q(-1, 14) }],
        };
        vec![(Nucleus { planes, ..Nucleus::of(26, 30) }, BigUint::from(10u8))]
    };
    let mut k = universe(true);
    let mut off = manifest(1);
    off.nuclei = entry(q(1, 7));
    assert!(matches!(k.manifest(&signed(off, RESERVE)), Granted::Refused(GrantRefusal::NotAtRest)));
    assert!(k.grants.get(&reserve_domain()).is_none(), "nothing granted");
    let mut rest = manifest(2);
    rest.nuclei = entry(q(1, 14));
    assert!(matches!(k.manifest(&signed(rest, RESERVE)), Granted::Sealed { .. }), "the rest point is granted");
    let mut unstated = manifest(3);
    unstated.nuclei = entry(q(0, 1));
    let mut k2 = universe(true);
    assert!(matches!(k2.manifest(&signed(unstated, RESERVE)), Granted::Sealed { .. }), "R = 0 is left to the kernel");
    assert_eq!(GrantRefusal::of(13), Some(GrantRefusal::NotAtRest));
}
