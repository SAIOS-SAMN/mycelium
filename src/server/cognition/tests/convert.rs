// 78.127 — 2(e) μ CONVERSION: a domain's reserve tier designated by its key
// (redesignated only from the reserve's next fold); game value exchanged
// against μ the reserve actually holds — out in one act, in through escrow —
// each G side consumed once by the reserve tier's fold, and a rate never
// minting μ.

use crate::anastome::dialogue::domain::{Founded, Reservation};
use crate::anastome::dialogue::handoff::{ConvertIn, ConvertOut, Handed, Handoff, HandoffRefusal, Side, State};
use crate::anastome::dialogue::uplink::{Discontinuous, Net, SideRefusal, TierFold, Uplinked};
use crate::server::cognition::Kernel;
use ed25519_dalek::SigningKey;

fn key(b: u8) -> SigningKey {
    SigningKey::from_bytes(&[b; 32])
}

fn vk(k: &SigningKey) -> [u8; 32] {
    k.verifying_key().to_bytes()
}

/// The domain's key, its two tiers, and a μ holder.
fn domain() -> SigningKey {
    key(4)
}
fn x() -> SigningKey {
    key(21)
}
fn x2() -> SigningKey {
    key(22)
}
fn holder() -> SigningKey {
    key(50)
}

/// The in-game holder whose game value converts.
const PLAYER: [u8; 32] = [7; 32];

fn founded() -> Kernel {
    use crate::anastome::dialogue::domain::DomainGenesis;
    let mut k = Kernel::new();
    let mut tiers = vec![vk(&x()), vk(&x2())];
    tiers.sort_unstable();
    let g = DomainGenesis { domain: vk(&domain()), tiers };
    assert!(matches!(k.admit_domain(&g.sign(&domain())), Founded::Sealed { .. }));
    k
}

fn designate(k: &mut Kernel, tier: &SigningKey, nonce: u8) -> Founded {
    k.designate_reserve(&Reservation { domain: vk(&domain()), tier: vk(tier), nonce: [nonce; 32] }.sign(&domain()))
}

/// 78.139 — the ancients' survival basket stated by the sitting reserve `x`.
fn basket(k: &mut Kernel) {
    use crate::anastome::dialogue::universe::{Statement, Stated};
    let s = Statement { domain: vk(&domain()), stated: Stated::Basket(vec![(1, (1, 1)), (8, (8, 1))]) };
    assert!(matches!(k.state(&s.sign(&x())), Founded::Stated { .. }));
}

/// μ seeded on a key, as the fixture's genesis seeds its own.
fn fund(k: &mut Kernel, who: [u8; 32], mu: [u64; 4]) {
    let anchor = k.anchor();
    k.treasury.genesis(who, mu, anchor);
}

fn fold(prefix: u8, rows: Vec<Net>, handoffs: Vec<([u8; 32], Side)>) -> TierFold {
    TierFold { tier: vk(&x()), prefix: [prefix; 32], rows, declared: Vec::new(), children: Vec::new(), handoffs }
}

fn out(g: i128, rate: (u64, u64), nonce: u8) -> ConvertOut {
    ConvertOut { tier: vk(&x()), holder: vk(&holder()), identity: PLAYER, g, rate: [rate, (0, 1), (0, 1), (0, 1)], nonce: [nonce; 32] }
}

/// THE RESERVE IS THE DOMAIN'S TO NAME: by its key, a tier its genesis
/// names; the first takes hold at once, a redesignation only from the
/// current reserve's next fold — never mid-era — and both restore.
#[test]
fn designate_reserve_redesignates_at_reserve_fold() {
    let mut k = founded();
    assert_eq!(k.designate_reserve(&Reservation { domain: vk(&domain()), tier: vk(&x()), nonce: [0; 32] }.sign(&x())), Founded::Unsigned);
    assert_eq!(designate(&mut k, &key(99), 0), Founded::NotATier);
    let stranger = Reservation { domain: vk(&key(5)), tier: vk(&x()), nonce: [0; 32] }.sign(&key(5));
    assert_eq!(k.designate_reserve(&stranger), Founded::NotFounded);
    assert_eq!(k.reserve_reading(&vk(&domain())), crate::anastome::dialogue::domain::ReserveReading::default(), "none yet");
    assert!(matches!(designate(&mut k, &x(), 1), Founded::Reserved { now: true, .. }));
    assert_eq!(k.reserve_of(&vk(&domain())), Some(vk(&x())));
    fund(&mut k, vk(&x()), [4, 0, 0, 1]);
    let read = k.reserve_reading(&vk(&domain()));
    assert_eq!((read.current, read.pending, read.balance), (Some(vk(&x())), None, [4, 0, 0, 1]), "the reserve as read");
    // 78.136 — succession is the sitting reserve's to sign, never the
    // founder's key alone.
    assert_eq!(designate(&mut k, &x2(), 2), Founded::Unsigned, "the domain designates once");
    let succession = Reservation { domain: vk(&domain()), tier: vk(&x2()), nonce: [2; 32] }.sign(&x());
    assert!(matches!(k.designate_reserve(&succession), Founded::Reserved { now: false, .. }), "pending, signed by the sitting reserve");
    assert_eq!(k.reserve_of(&vk(&domain())), Some(vk(&x())), "never mid-era");
    assert_eq!(k.reserve_reading(&vk(&domain())).pending, Some(vk(&x2())), "the pending redesignation, read");
    let mut booted = Kernel::new();
    assert!(booted.restore(&k.snapshot()), "restores");
    assert_eq!(booted.reserve_of(&vk(&domain())), Some(vk(&x())));
    assert!(matches!(booted.admit_uplink(&fold(1, Vec::new(), Vec::new()).sign(&x())), Uplinked::Admitted { .. }), "the reserve's anchor");
    assert_eq!(booted.reserve_of(&vk(&domain())), Some(vk(&x2())), "the redesignation holds from the anchor");
}

/// CONVERT-OUT PAYS FROM THE RESERVE, NEVER MINTS: 10 of game value at ½ μ
/// each is exactly 5 μ, paid from the reserve to the holder; a rate that is
/// not exact, a tier that is not the reserve, a conversion past the reserve
/// — each refused; a resend answers with its receipt. The G side leaves the
/// player's game value in the reserve tier's fold, once.
#[test]
fn convert_out_pays_from_reserve_never_mints() {
    let mut k = founded();
    assert!(matches!(designate(&mut k, &x(), 1), Founded::Reserved { .. }));
    fund(&mut k, vk(&x()), [8, 0, 0, 0]);
    let refused = |k: &mut Kernel, c: ConvertOut, by: &SigningKey| match k.handoff(&Handoff::ConvertOut(Box::new(c)).sign(by)) {
        Handed::Refused(r) => r,
        other => panic!("refused: {other:?}"),
    };
    assert_eq!(refused(&mut k, out(10, (1, 2), 1), &x()), HandoffRefusal::NoBasket, "78.139: no conversion before the ancients' basket");
    basket(&mut k);
    assert_eq!(refused(&mut k, out(10, (1, 3), 1), &x()), HandoffRefusal::Inexact, "10/3 is no integer");
    assert_eq!(refused(&mut k, ConvertOut { tier: vk(&x2()), ..out(10, (1, 2), 1) }, &x2()), HandoffRefusal::NotReserve);
    assert_eq!(refused(&mut k, out(20, (1, 1), 1), &x()), HandoffRefusal::ReserveShort, "a rate never mints μ");
    let supply = k.treasury.balance_mu(&vk(&x()))[0] + k.treasury.balance_mu(&vk(&holder()))[0];
    let first = k.handoff(&Handoff::ConvertOut(Box::new(out(10, (1, 2), 1))).sign(&x()));
    let Handed::Sealed { id, state: State::Accepted, .. } = first else { panic!("{first:?}") };
    assert_eq!((k.treasury.balance_mu(&vk(&x()))[0], k.treasury.balance_mu(&vk(&holder()))[0]), (3, 5));
    assert_eq!(k.treasury.balance_mu(&vk(&x()))[0] + k.treasury.balance_mu(&vk(&holder()))[0], supply, "Σ = 0");
    assert_eq!(k.handoff(&Handoff::ConvertOut(Box::new(out(10, (1, 2), 1))).sign(&x())), first, "a resend");
    assert_eq!(k.treasury.balance_mu(&vk(&holder()))[0], 5, "paid once");
    let leaving = vec![(PLAYER, [0, -10, 0, 0])];
    assert!(matches!(k.admit_uplink(&fold(1, leaving.clone(), Vec::new()).sign(&x())), Uplinked::Discontinuous(Discontinuous::Unbalanced { lane: 1 })));
    assert!(matches!(k.admit_uplink(&fold(1, leaving.clone(), vec![(id, Side::Out)]).sign(&x())), Uplinked::Admitted { .. }));
    assert_eq!(
        k.admit_uplink(&fold(2, leaving, vec![(id, Side::Out)]).sign(&x())),
        Uplinked::HandoffRefused { index: 0, why: SideRefusal::Consumed }
    );
}

/// CONVERT-IN THROUGH ESCROW: the holder's μ held until the reserve answers;
/// accepted, it enters the reserve and the player's game value arrives in
/// the reserve tier's fold; refused, it is refunded whole and nothing
/// arrives.
#[test]
fn convert_in_holds_escrow_until_reserve_answers() {
    let mut k = founded();
    assert!(matches!(designate(&mut k, &x(), 1), Founded::Reserved { .. }));
    basket(&mut k);
    fund(&mut k, vk(&holder()), [6, 0, 0, 0]);
    let offer = |nonce: u8| ConvertIn { holder: vk(&holder()), tier: vk(&x()), identity: PLAYER, g: 7, mu: [3, 0, 0, 0], nonce: [nonce; 32] };
    let Handed::Sealed { id, state: State::Offered { .. }, .. } = k.handoff(&Handoff::ConvertIn(Box::new(offer(1))).sign(&holder())) else {
        panic!("offered")
    };
    assert_eq!(k.treasury.balance_mu(&vk(&holder()))[0], 3, "held from the holder");
    let arriving = vec![(PLAYER, [0, 7, 0, 0])];
    assert_eq!(
        k.admit_uplink(&fold(1, arriving.clone(), vec![(id, Side::In)]).sign(&x())),
        Uplinked::HandoffRefused { index: 0, why: SideRefusal::NotYet }
    );
    assert!(matches!(k.handoff(&Handoff::Accept { id }.sign(&holder())), Handed::Refused(HandoffRefusal::NotTheTier)));
    assert!(matches!(k.handoff(&Handoff::Accept { id }.sign(&x())), Handed::Sealed { state: State::Accepted, .. }));
    assert_eq!(k.treasury.balance_mu(&vk(&x()))[0], 3, "into the reserve");
    assert!(matches!(k.admit_uplink(&fold(1, arriving, vec![(id, Side::In)]).sign(&x())), Uplinked::Admitted { .. }));
    // Refused: refunded whole.
    let Handed::Sealed { id, .. } = k.handoff(&Handoff::ConvertIn(Box::new(offer(2))).sign(&holder())) else { panic!() };
    assert!(matches!(k.handoff(&Handoff::Refuse { id }.sign(&x())), Handed::Sealed { state: State::Returned, .. }));
    assert_eq!(k.treasury.balance_mu(&vk(&holder()))[0], 3, "refunded");
    assert_eq!(k.handoff_read(&id), Some((State::Returned, false, false)));
}
