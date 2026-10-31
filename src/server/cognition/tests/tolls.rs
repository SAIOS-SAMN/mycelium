// 78.133 — PER-UNIT AND SPLIT TOLLS on a wormhole (as
// ruled): mass pays on S, information on H, each floored; the charge split
// to the keeper and its progenitor lines, every lane's parts summing to its
// charge exactly; held from acceptance to the window's close, then paid
// together — or, the traversal's OUT-side fold reversed, refunded together.

use crate::anastome::dialogue::challenge::{Challenge, Challenged, Finality};
use crate::anastome::dialogue::domain::{DomainGenesis, Founded, Reservation};
use crate::anastome::dialogue::handoff::{Handed, Handoff, HandoffRefusal, Mouth, Offer, Side, State, Tariff, Untariffed};
use crate::anastome::dialogue::uplink::{TierFold, Uplinked};
use crate::chronometry::ActKind;
use crate::physics::laws::CHALLENGE_WINDOW;
use crate::server::cognition::handoff::toll_key;
use crate::server::cognition::Court;
use ed25519_dalek::SigningKey;

fn key(b: u8) -> SigningKey {
    SigningKey::from_bytes(&[b; 32])
}

fn vk(k: &SigningKey) -> [u8; 32] {
    k.verifying_key().to_bytes()
}

/// Domain `d`'s key and its reserve tier; domain 4's progenitor line, 30.
fn domain(d: u8) -> SigningKey {
    key(d)
}
fn reserve(d: u8) -> SigningKey {
    key(d + 20)
}
fn line() -> SigningKey {
    key(30)
}

/// Domains 4 (its reserve and its line) and 5 (its reserve), on one court.
fn founded() -> Court {
    let mut k = Court::new();
    for d in [4u8, 5] {
        let mut tiers = vec![vk(&reserve(d))];
        if d == 4 {
            tiers.push(vk(&line()));
            tiers.sort_unstable();
        }
        let g = DomainGenesis { domain: vk(&domain(d)), tiers };
        assert!(matches!(k.admit_domain(&g.sign(&domain(d))), Founded::Sealed { .. }));
        let r = Reservation { domain: vk(&domain(d)), tier: vk(&reserve(d)), nonce: [0; 32] };
        assert!(matches!(k.designate_reserve(&r.sign(&domain(d))), Founded::Reserved { .. }));
    }
    k
}

/// A split in its one writing: the recipients ascending.
fn split(mut parts: Vec<([u8; 32], (u64, u64))>) -> Vec<([u8; 32], (u64, u64))> {
    parts.sort_unstable();
    parts
}

/// 3/2 μ-S per unit of mass, 5 μ-H per unit of information; a third to the
/// line, two thirds to the keeper (domain 4's reserve).
fn tariff() -> Tariff {
    Tariff { rate_s: (3, 2), rate_h: (5, 1), split: split(vec![(vk(&line()), (1, 3)), (vk(&reserve(4)), (2, 3))]) }
}

fn mouth(t: Tariff) -> Mouth {
    let one = crate::transducer::element::identity_versor();
    Mouth {
        from_tier: vk(&reserve(4)),
        to_tier: vk(&reserve(5)),
        keeper: vk(&reserve(4)),
        from_frame: one.clone(),
        to_frame: one,
        toll: [0; 4],
        nonce: [4; 32],
        tariff: Some(t),
        tube: Some(crate::anastome::dialogue::handoff::Tube {
            mass: (1, 1),
            from_space: crate::anastome::dialogue::universe::Geometry::Curved((1, 1)),
            to_space: crate::anastome::dialogue::universe::Geometry::Curved((1, 1)),
            supersedes: None,
            core: false,
        }),
    }
}

fn open(k: &mut Court, m: Mouth) -> [u8; 32] {
    // The throat's bank, funded first so balances read as before.
    let anchor = k.anchor();
    k.treasury.genesis(vk(&reserve(4)), [1, 0, 0, 0], anchor);
    let id = m.id();
    assert!(matches!(k.handoff(&Handoff::Mouth(Box::new(m)).sign(&reserve(4))), Handed::Sealed { .. }));
    assert!(matches!(k.handoff(&Handoff::Accept { id }.sign(&reserve(5))), Handed::Sealed { state: State::Accepted, .. }));
    id
}

/// `s` of matter and `h` of information from domain 5 to domain 4.
fn passage(s: i128, h: i128, nonce: u8) -> Offer {
    Offer {
        from_tier: vk(&reserve(5)),
        to_tier: vk(&reserve(4)),
        out_nets: vec![([1; 32], [-s, 0, 0, -h])],
        in_nets: vec![([2; 32], [s, 0, 0, h])],
        charter: [0; 32],
        nonce: [nonce; 32],
    }
}

fn traverse(k: &mut Court, mouth: [u8; 32], o: Offer) -> Handed {
    k.handoff(&Handoff::Traverse { offer: Box::new(o), mouth }.sign(&reserve(5)))
}

fn fund(k: &mut Court, who: [u8; 32], mu: [u64; 4]) {
    let anchor = k.anchor();
    k.treasury.genesis(who, mu, anchor);
}

fn pass(k: &mut Court, h: u64) {
    while k.chain.head_height() < h {
        k.append_step(ActKind::List, [1; 32], [2; 32]);
    }
}

/// THE PARTS SUM TO THE CHARGE EXACTLY, NO DUST, under adversarial shares:
/// thirds, sevenths, shares within one part in 10⁹ of whole, shares of
/// `u64::MAX`, a split with no keeper share — every lane's parts sum to its
/// charge, each non-keeper part is exactly its floor, the keeper's last.
#[test]
fn tariff_parts_sum_exactly_under_adversarial_shares() {
    let (a, b, keeper) = ([1u8; 32], [2u8; 32], [9u8; 32]);
    let splits = [
        vec![(a, (1, 3)), (b, (1, 3)), (keeper, (1, 3))],
        vec![(a, (1, 7)), (b, (2, 7)), (keeper, (4, 7))],
        vec![(a, (999_999_999, 1_000_000_000)), (keeper, (1, 1_000_000_000))],
        vec![(a, (u64::MAX - 1, u64::MAX)), (keeper, (1, u64::MAX))],
        vec![(a, (1, 2)), (b, (1, 2))],
    ];
    let charges = [[0; 4], [1, 0, 0, 1], [100, 0, 0, 7], [u64::MAX, 0, 0, u64::MAX - 1], [1_000_003, 0, 0, 999_983]];
    for s in &splits {
        let t = Tariff { rate_s: (1, 1), rate_h: (1, 1), split: s.clone() };
        assert_eq!(t.holds(), Ok(()), "{s:?}");
        for c in &charges {
            let parts = t.parts(&keeper, c).expect("parts");
            for lane in 0..4 {
                let sum: u128 = parts.iter().map(|(_, p)| u128::from(p[lane])).sum();
                assert_eq!(sum, u128::from(c[lane]), "lane {lane} of {c:?} under {s:?}");
            }
            for (who, part) in parts.iter().filter(|(who, _)| *who != keeper) {
                let (n, d) = s.iter().find(|(w, _)| w == who).map(|(_, f)| *f).expect("a recipient");
                for lane in 0..4 {
                    assert_eq!(u128::from(part[lane]), u128::from(n) * u128::from(c[lane]) / u128::from(d), "the floor");
                }
            }
            if let Some((last, _)) = parts.iter().next_back() {
                assert!(*last == keeper || parts.iter().all(|(w, _)| *w != keeper), "the keeper's part last");
            }
        }
    }
    let t = |split| Tariff { rate_s: (1, 1), rate_h: (1, 1), split };
    assert_eq!(t(vec![(a, (1, 3)), (keeper, (1, 3))]).holds(), Err(Untariffed::NotWhole));
    assert_eq!(t(vec![(a, (1, 0)), (keeper, (1, 1))]).holds(), Err(Untariffed::ZeroDenominator));
    assert_eq!(t(vec![(keeper, (1, 2)), (a, (1, 2))]).holds(), Err(Untariffed::Unordered));
    assert_eq!(Tariff { rate_s: (1, 0), ..t(vec![(keeper, (1, 1))]) }.holds(), Err(Untariffed::ZeroDenominator));
}

/// THE SPLIT IS THE KEEPER'S AND ITS LINES' ONLY, AND ONE LAW: a share to
/// the other domain's tier or to a stranger, a flat toll beside the tariff,
/// shares not whole — each refused at the offer.
#[test]
fn handoff_mouth_refuses_split_beyond_keeper_and_lines() {
    let mut k = founded();
    let offer = |k: &mut Court, m: Mouth| match k.handoff(&Handoff::Mouth(Box::new(m)).sign(&reserve(4))) {
        Handed::Refused(r) => Some(r),
        _ => None,
    };
    let to_other = Tariff { split: split(vec![(vk(&reserve(5)), (1, 3)), (vk(&reserve(4)), (2, 3))]), ..tariff() };
    assert_eq!(offer(&mut k, mouth(to_other)), Some(HandoffRefusal::NotALine), "the traveller's domain is not the keeper's");
    let stranger = Tariff { split: split(vec![([7; 32], (1, 3)), (vk(&reserve(4)), (2, 3))]), ..tariff() };
    assert_eq!(offer(&mut k, mouth(stranger)), Some(HandoffRefusal::NotALine));
    let flat = Mouth { toll: [1, 0, 0, 0], ..mouth(tariff()) };
    assert_eq!(offer(&mut k, flat), Some(HandoffRefusal::Untariffed(Untariffed::Flat)), "a mouth's toll is one law");
    let partial = Tariff { split: split(vec![(vk(&line()), (1, 3)), (vk(&reserve(4)), (1, 3))]), ..tariff() };
    assert_eq!(offer(&mut k, mouth(partial)), Some(HandoffRefusal::Untariffed(Untariffed::NotWhole)));
    fund(&mut k, vk(&reserve(4)), [1, 0, 0, 0]);
    assert_eq!(offer(&mut k, mouth(tariff())), None, "the keeper and its line, its throat banked");
}

/// EVERY PASSAGE PAYS ON ITS OWN LANES, FLOORED: 7 of mass is ⌊10.5⌋ = 10
/// μ-S; 3 of information is 15 μ-H; both, both; G and T never. A traveller
/// short on a lane is refused and never debited. Accepted, the parts are
/// HELD: the recipients' balances untouched, the recipients' line heads reading it.
#[test]
fn handoff_traverse_charges_own_lanes_and_refuses_short() {
    let mut k = founded();
    let m = open(&mut k, mouth(tariff()));
    let traveller = vk(&reserve(5));
    fund(&mut k, traveller, [100, 9, 9, 4]);
    let before = k.treasury.balance_mu(&traveller);
    assert_eq!(traverse(&mut k, m, passage(0, 3, 1)), Handed::Refused(HandoffRefusal::HolderShort), "15 μ-H against 4");
    assert_eq!(k.treasury.balance_mu(&traveller), before, "never debited");
    fund(&mut k, traveller, [0, 0, 0, 96]);
    for (s, h, nonce, charge) in [(7, 0, 2, [10, 0, 0, 0]), (0, 3, 3, [0, 0, 0, 15]), (7, 3, 4, [10, 0, 0, 15])] {
        let Handed::Sealed { id, .. } = traverse(&mut k, m, passage(s, h, nonce)) else { panic!("crossing") };
        assert_eq!(k.treasury.held_of(&toll_key(&id)).map(|h| h.mu), Some(charge), "mass on S, information on H");
    }
    assert_eq!(k.treasury.balance_mu(&traveller), [80, 9, 9, 70], "G and T never charged");
    let Handed::Sealed { id, .. } = traverse(&mut k, m, passage(7, 3, 5)) else { panic!() };
    assert!(matches!(k.handoff(&Handoff::Accept { id }.sign(&reserve(4))), Handed::Sealed { state: State::Accepted, .. }));
    assert_eq!(k.treasury.balance_mu(&vk(&line())), [0; 4], "held, not paid");
    assert_eq!(k.toll_held_for(&vk(&line())), Some([3, 0, 0, 5]), "a third, floored");
    assert_eq!(k.toll_held_for(&vk(&reserve(4))), Some([7, 0, 0, 10]), "the keeper, the rest");
}

/// THE PARTS ARE PAID TOGETHER WHEN THE WINDOW CLOSES: held across a
/// restart; before `W` from the acceptance nothing moves; after it, every
/// part at once, each part's rows citing the traverse act.
#[test]
fn settle_windows_releases_toll_parts_together() {
    let mut k = founded();
    let m = open(&mut k, mouth(tariff()));
    fund(&mut k, vk(&reserve(5)), [10, 0, 0, 15]);
    let Handed::Sealed { id, at: traversed, .. } = traverse(&mut k, m, passage(7, 3, 1)) else { panic!() };
    let Handed::Sealed { at: accepted, .. } = k.handoff(&Handoff::Accept { id }.sign(&reserve(4))) else { panic!() };
    let mut booted = Court::new();
    assert!(booted.restore(&k.snapshot()), "restores");
    assert_eq!(booted.toll_held_for(&vk(&line())), Some([3, 0, 0, 5]), "the toll held across a restart");
    let mut k = booted;
    pass(&mut k, accepted.h + CHALLENGE_WINDOW - 1);
    k.settle_windows();
    assert_eq!(k.treasury.balance_mu(&vk(&line())), [0; 4], "the window stands");
    pass(&mut k, accepted.h + CHALLENGE_WINDOW);
    k.settle_windows();
    assert_eq!(k.treasury.balance_mu(&vk(&line())), [3, 0, 0, 5]);
    assert_eq!(k.treasury.balance_mu(&vk(&reserve(4))), [7, 0, 0, 10]);
    assert_eq!(k.treasury.balance_mu(&vk(&reserve(5))), [0; 4]);
    assert_eq!(k.toll_held_for(&vk(&line())), Some([0; 4]), "released");
    let cited = k.treasury.entries().iter().filter(|e| e.who == vk(&line()) && e.at == traversed).count();
    assert_eq!(cited, 1, "the line's receipt ties back to the traverse act");
}

/// AN OUT-SIDE FOLD REVERSED REFUNDS EVERY PART TOGETHER: the traveller's
/// fold consumed the passage leaving; demanded, it holds the toll past the
/// window; defaulted, it is reversed, and every part returns at once.
#[test]
fn challenge_default_refunds_toll_parts_together() {
    let mut k = founded();
    let m = open(&mut k, mouth(tariff()));
    let traveller = vk(&reserve(5));
    fund(&mut k, traveller, [10, 0, 0, 15]);
    let Handed::Sealed { id, .. } = traverse(&mut k, m, passage(7, 3, 1)) else { panic!() };
    let Handed::Sealed { at: accepted, .. } = k.handoff(&Handoff::Accept { id }.sign(&reserve(4))) else { panic!() };
    let leaving = vec![([1; 32], [-7, 0, 0, -3])];
    let fold = TierFold { tier: traveller, prefix: [1; 32], rows: leaving, declared: Vec::new(), children: Vec::new(), handoffs: vec![(id, Side::Out)] };
    let Uplinked::Admitted { at, .. } = k.admit_uplink(&fold.sign(&reserve(5))) else { panic!("the OUT side consumed") };
    let at = crate::anastome::dialogue::uplink::FoldAt::sealed(at);
    assert!(k.fold_holds_toll(&at), "the toll rides on the fold");
    let Challenged::Demanded { deadline, .. } = k.challenge(&Challenge::Demand { fold: at.clone() }.sign(&key(60))) else { panic!() };
    pass(&mut k, accepted.h + CHALLENGE_WINDOW);
    k.settle_windows();
    assert_eq!(k.toll_held_for(&vk(&line())), Some([3, 0, 0, 5]), "the demand holds it past the window");
    pass(&mut k, deadline);
    assert!(matches!(k.challenge(&Challenge::Default { fold: at.clone() }.sign(&key(60))), Challenged::Falsified { .. }));
    assert_eq!(k.finality(&at), Finality::Falsified);
    assert_eq!(k.treasury.balance_mu(&traveller), [10, 0, 0, 15], "every part back together");
    assert_eq!(k.toll_held_for(&vk(&line())), Some([0; 4]));
    assert_eq!(k.toll_held_for(&vk(&reserve(4))), Some([0; 4]));
    assert!(!k.fold_holds_toll(&at));
}
