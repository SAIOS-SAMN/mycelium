// 78.184 §6 (ruled 2026-11-16) — μ FOLLOWS ITS NUCLEI ACROSS A TUBE: the
// transport multi-axial, nucleons conserved, dissipation to the pool.

use super::atlas::{commit, domain, founded, fp, fund, key, map, mouth, reserve, stake, verifying};
use crate::anastome::dialogue::atlas::{Ledgered, Mapped};
use crate::anastome::dialogue::crossing::{Crossed, Motion, Traverse};
use crate::anastome::dialogue::domain::{BigBangRecord, Founded};
use crate::anastome::dialogue::endowment::{Granted, Manifest, Nucleus};
use crate::economy::Mu;
use crate::server::cognition::Court;
use ed25519_dalek::Signer;
use num_bigint::BigUint;

fn q(n: i64, d: i64) -> crate::core::q::Q {
    crate::core::q::Q::new(n.into(), d.into())
}

/// A final point whose T lane reads `t`.
fn on_t(h: u64, t: i64) -> crate::chronometry::tuniversal::TPoint {
    crate::chronometry::tuniversal::TPoint { lanes: Mu::of([q(0, 1), q(0, 1), q(t, 1), q(0, 1)]).expect("lanes"), ..fp(h) }
}

fn hydrogen(n: u32) -> crate::anastome::dialogue::endowment::Nuclei {
    vec![(Nucleus::of(1, 0), BigUint::from(n))]
}

/// Domain `d` originated and granted `n` hydrogen nuclei.
fn granted(k: &mut Court, d: u8, n: u32) {
    let o = BigBangRecord { domain: domain(d), kind: 1, version: 1, commitment: [d; 32], universe: None };
    assert!(matches!(k.record_origin(&o.sign(&key(d))), Founded::Originated { .. }));
    let m = Manifest { domain: domain(d), nuclei: hydrogen(n), nonce: [d; 32] };
    let s = m.sign(crate::anastome::dialogue::crossing_record::still(), &reserve(d)).expect("writes");
    assert!(matches!(k.manifest(&s), Granted::Sealed { founding: true, .. }));
}

/// A universe's motion at the tube, signed by its sitting reserve.
fn motion(t: &Traverse, d: u8, released: u32, taken: u32) -> Motion {
    let (r, tk) = (if released == 0 { Vec::new() } else { hydrogen(released) }, if taken == 0 { Vec::new() } else { hydrogen(taken) });
    let sig = reserve(d).sign(&t.motion_bytes(&domain(d), &r, &tk).expect("writes")).to_bytes();
    Motion { domain: domain(d), released: r, taken: tk, sig }
}

/// THE NUCLEI CROSS AND THEIR μ WITH THEM. Universe 4 holds 1,000 hydrogen
/// nuclei, half its allotment delivered (`T₀ = 2`, `T = 4`). It releases
/// 100 at the tube and universe 5 takes in 90: ten are lost. The delivered
/// half of the 100's μ moves now, nine tenths to 5's reserve and one tenth
/// to the recycled pool; the owed half moves as carried allotments in the
/// same proportion, delivered along the same T lane. Each element account
/// reads in nucleons, and nothing is minted at the crossing.
#[test]
fn mu_follows_its_nuclei_across_a_tube_with_dissipation_to_the_pool() {
    let (mut k, m, _) = founded();
    granted(&mut k, 4, 1000);
    granted(&mut k, 5, 10);
    let _ = k.deliver_at(&on_t(3, 2));
    let _ = k.deliver_at(&on_t(4, 4));
    let (ra, rb) = (verifying(&reserve(4)), verifying(&reserve(5)));
    let (a_before, b_before, pool_before) = (k.treasury.balance_mu(&ra), k.treasury.balance_mu(&rb), k.treasury.recycled());
    let carried = Mu::of_modes(k.elemental_creation(&hydrogen(100)).expect("rows").scaled(&k.alignment_share(&domain(4)).expect("aligned")).expect("scales"));
    assert!(!carried.is_zero(), "the 100 carry μ");
    assert!(matches!(map(&mut k, 4, 10, mouth(m, 0, 3)), Mapped::Sealed { .. }));
    assert!(matches!(map(&mut k, 5, 10, mouth(m, 1, 3)), Mapped::Sealed { .. }));
    assert!(matches!(commit(&mut k, 4, 1, 10, 10, Vec::new()), Ledgered::Sealed { .. }));
    assert!(matches!(commit(&mut k, 5, 1, 10, 10, Vec::new()), Ledgered::Sealed { .. }));
    let payer = key(60);
    fund(&mut k, verifying(&payer), [10, 600, 10, 600]);
    assert_eq!(stake(&mut k, &payer, [10, 600, 10, 600], false, 1), Ok([10, 600, 10, 600]));
    let head = |k: &Court, d: u8| k.atlas.head(&domain(d)).map_or(([0; 32], 0), |e| (e.hash, e.entry.u));
    let ((entry_a, u_a), (entry_b, u_b)) = (head(&k, 4), head(&k, 5));
    let mut t = Traverse { from: domain(4), payer: verifying(&payer), mouth: m, side: 0, at: fp(10), point_a: fp(10), point_b: fp(10), entry_a, u_a, entry_b, u_b, route: Vec::new(), motions: Vec::new(), nonce: [1; 32] };
    let point = |k: &Court, d: u8, side: u8| {
        let id = crate::server::cognition::atlas::mouth_citation(&domain(d), &m, side);
        k.atlas.records.get(&crate::server::cognition::atlas::record_key(&domain(d), &id)).map_or(fp(0), |r| r.point.clone())
    };
    t.point_a = point(&k, 4, 0);
    t.point_b = point(&k, 5, 1);
    t.motions = vec![motion(&t, 4, 100, 0), motion(&t, 5, 0, 90)];
    let minted = k.treasury.minted();
    let crossed = k.traverse(&t.sign(&payer).expect("signs"), Some(fp(10)));
    let Crossed::Sealed { gas, .. } = crossed else { panic!("crossed: {crossed:?}") };
    assert_eq!((gas.g, gas.h), (300, 270), "G the nucleons released, H those taken, in thirds");
    assert_eq!(k.treasury.minted(), minted, "nothing minted at the crossing");
    let half = carried.scaled(&q(1, 2)).expect("half");
    assert_eq!(k.treasury.balance_mu(&rb), b_before.add(&half.scaled(&q(9, 10)).expect("nine tenths")), "the taker holds nine tenths of the delivered half");
    assert_eq!(a_before.draw(&half).ok(), Some(k.treasury.balance_mu(&ra)), "the releaser gives up the delivered half");
    assert_eq!(k.treasury.recycled(), pool_before.add(&half.scaled(&q(1, 10)).expect("a tenth")), "the lost tenth to the pool");
    assert_eq!(k.element_account(&domain(4)), Some(BigUint::from(2700u32)), "1,000 less the 100 released, in thirds");
    assert_eq!(k.element_account(&domain(5)), Some(BigUint::from(300u32)), "10 granted and 90 taken in, in thirds");
    // The owed half follows on the T lane: at T = 8, d = 3/4, so half of
    // what was owed at d = 1/2 arrives.
    let beats = k.deliver_at(&on_t(5, 8));
    let to_b: Mu = beats.iter().filter(|(d, _)| *d == domain(5)).fold(Mu::zero(), |s, (_, mu)| s.add(mu));
    let owed_b = half.scaled(&q(9, 10)).expect("nine tenths");
    let founding_b = k.grants.get(&domain(5)).and_then(|g| g.iter().find(|g| g.founding).map(|g| g.allotment.scaled(&q(1, 4)).expect("a quarter"))).expect("5's founding");
    assert_eq!(to_b, founding_b.add(&owed_b.scaled(&q(1, 2)).expect("half")), "5's own founding beat and half its carried allotment");
}


/// NO TRAVERSAL IS A FREE LOOP. μ is held on the nuclei, never re-derived
/// from the holder: universe 4 releases 100 of its 1,000 hydrogen and 5
/// (granted nothing) takes 90; 5 then releases those 90 and 4 takes 81
/// back. Each pass dissipates a tenth to the pool, so 4's reserve ends
/// short by exactly the two passes' losses, 19% of the delivered μ that
/// left, and never gains.
#[test]
fn out_and_back_returns_the_same_mu_less_each_dissipation() {
    let (mut k, m, _) = founded();
    granted(&mut k, 4, 1000);
    let _ = k.deliver_at(&on_t(3, 2));
    let _ = k.deliver_at(&on_t(4, 4));
    let ra = verifying(&reserve(4));
    let a_before = k.treasury.balance_mu(&ra);
    let half = Mu::of_modes(k.elemental_creation(&hydrogen(100)).expect("rows").scaled(&k.alignment_share(&domain(4)).expect("aligned")).expect("scales")).scaled(&q(1, 2)).expect("half");
    assert!(matches!(map(&mut k, 4, 10, mouth(m, 0, 3)), Mapped::Sealed { .. }));
    assert!(matches!(map(&mut k, 5, 10, mouth(m, 1, 3)), Mapped::Sealed { .. }));
    let payer = key(60);
    fund(&mut k, verifying(&payer), [20, 1200, 20, 1200]);
    assert_eq!(stake(&mut k, &payer, [20, 1200, 20, 1200], false, 1), Ok([20, 1200, 20, 1200]));
    let point = |k: &Court, d: u8, side: u8| {
        let id = crate::server::cognition::atlas::mouth_citation(&domain(d), &m, side);
        k.atlas.records.get(&crate::server::cognition::atlas::record_key(&domain(d), &id)).map_or(fp(0), |r| r.point.clone())
    };
    let cross = |k: &mut Court, u: u64, out: (u8, u32), into: (u8, u32), nonce: u8| {
        assert!(matches!(commit(k, 4, u, 10, 10, Vec::new()), Ledgered::Sealed { .. }));
        assert!(matches!(commit(k, 5, u, 10, 10, Vec::new()), Ledgered::Sealed { .. }));
        let head = |k: &Court, d: u8| k.atlas.head(&domain(d)).map_or(([0; 32], 0), |e| (e.hash, e.entry.u));
        let ((entry_a, u_a), (entry_b, u_b)) = (head(k, 4), head(k, 5));
        let mut t = Traverse { from: domain(4), payer: verifying(&payer), mouth: m, side: 0, at: fp(10), point_a: point(k, 4, 0), point_b: point(k, 5, 1), entry_a, u_a, entry_b, u_b, route: Vec::new(), motions: Vec::new(), nonce: [nonce; 32] };
        t.motions = vec![motion(&t, out.0, out.1, 0), motion(&t, into.0, 0, into.1)];
        let crossed = k.traverse(&t.sign(&payer).expect("signs"), Some(fp(10)));
        assert!(matches!(crossed, Crossed::Sealed { .. }), "crossed: {crossed:?}");
    };
    cross(&mut k, 1, (4, 100), (5, 90), 1);
    cross(&mut k, 2, (5, 90), (4, 81), 2);
    assert_eq!(k.element_account(&domain(4)), Some(BigUint::from(2943u32)), "1,000 less 100 out plus 81 back, in thirds");
    assert_eq!(k.element_account(&domain(5)), Some(BigUint::from(0u32)), "all it took, released");
    let lost = half.scaled(&q(19, 100)).expect("the two losses");
    assert_eq!(a_before.draw(&lost).ok(), Some(k.treasury.balance_mu(&ra)), "4 holds what it began with less exactly the two dissipations");
}
