// 78.154 — THE MAP, THE LEDGER, THE TRAVERSE AND THE CONNECTION TAX on one
// court (MU-ECONOMY §7.1–7.2; 78.156): a universe's reserve maps its mouths
// and chains its ledger; a traverse buys both mouths' coordinates only where
// both are mapped and each universe's latest entry is fresh by its own
// mouth's N; its map fee — the route's gas per lane, exact — is claimed on
// the payer's stake and paid whole to the court; the reserve's connection
// tax fills the recycled pool, which pays a closer's POUW before any mint.

use crate::anastome::dialogue::atlas::{local_record_id, local_throat_id, LedgerEntry, LedgerRefusal, Ledgered, MapAct, MapRecord, MapRefusal, Mapped, Segment};
use crate::anastome::dialogue::crossing::{CrossRefusal, Crossed, Declaration, Gas, Side, Traverse};
use crate::anastome::dialogue::domain::{DomainGenesis, Founded, Reservation};
use crate::anastome::dialogue::handoff::{Handed, Handoff, Mouth, State, Tube};
use crate::anastome::dialogue::uplink::{TaxRefusal, TierFold, Uplinked, CONNECTION_TAX};
use crate::anastome::dialogue::universe::{Geometry, Metric, Stated, Statement};
use crate::transducer::cga::Mv;
use crate::anastome::dialogue::wallet::{StakeAct, StakeRefusal};
use crate::chronometry::tuniversal::{Origin, TPoint};
use crate::chronometry::ActKind;
use crate::physics::laws::CHALLENGE_WINDOW;
use crate::server::cognition::Kernel;
use ed25519_dalek::{Signer, SigningKey};

fn key(b: u8) -> SigningKey {
    SigningKey::from_bytes(&[b; 32])
}

fn vk(k: &SigningKey) -> [u8; 32] {
    k.verifying_key().to_bytes()
}

fn domain(d: u8) -> [u8; 32] {
    vk(&key(d))
}

fn reserve(d: u8) -> SigningKey {
    key(d + 20)
}

/// A final point of the mesh at tick `h`.
fn fp(h: u64) -> TPoint {
    TPoint { origin: Origin { genesis: [9; 32], epoch: 0 }, h, lanes: [h, 0, 0, 0], k: h }
}

/// Domains 4 and 5 founded, each its own reserve, and a tube open between
/// their reserves (4 offers and keeps it).
fn founded() -> (Kernel, [u8; 32], [u8; 32]) {
    let mut k = Kernel::new();
    for d in [4u8, 5] {
        let g = DomainGenesis { domain: domain(d), tiers: vec![vk(&reserve(d))] };
        assert!(matches!(k.admit_domain(&g.sign(&key(d))), Founded::Sealed { .. }));
        let r = Reservation { domain: domain(d), tier: vk(&reserve(d)), nonce: [0; 32] };
        assert!(matches!(k.designate_reserve(&r.sign(&key(d))), Founded::Reserved { .. }));
    }
    let one = crate::transducer::element::identity_versor();
    let tube = Tube { mass: (1, 1), from_space: Geometry::Curved((1, 1)), to_space: Geometry::Curved((1, 1)), supersedes: None, core: false };
    let m = Mouth {
        from_tier: vk(&reserve(4)),
        to_tier: vk(&reserve(5)),
        keeper: vk(&reserve(4)),
        from_frame: one.clone(),
        to_frame: one,
        toll: [0; 4],
        nonce: [4; 32],
        tariff: None,
        tube: Some(tube),
    };
    fund(&mut k, vk(&reserve(4)), [1, 0, 0, 0]);
    let id = m.id();
    assert!(matches!(k.handoff(&Handoff::Mouth(Box::new(m)).sign(&reserve(4))), Handed::Sealed { .. }));
    assert!(matches!(k.handoff(&Handoff::Accept { id }.sign(&reserve(5))), Handed::Sealed { state: State::Accepted, .. }));
    let keeper = vk(&reserve(4));
    (k, id, keeper)
}

fn fund(k: &mut Kernel, who: [u8; 32], mu: [u64; 4]) {
    let anchor = k.anchor();
    k.treasury.genesis(who, mu, anchor);
}

fn pass(k: &mut Kernel, h: u64) {
    while k.chain.head_height() < h {
        k.append_step(ActKind::List, [1; 32], [2; 32]);
    }
}

/// Thirty-two bytes naming `n` exactly: its little-endian bytes, then zeros.
fn bytes_of(n: u64) -> [u8; 32] {
    let mut b = [0u8; 32];
    b[..8].copy_from_slice(&n.to_le_bytes());
    b
}

fn map(k: &mut Kernel, d: u8, h: u64, record: MapRecord) -> Mapped {
    k.map(&MapAct { domain: domain(d), point: fp(h), record, nonce: bytes_of(h) }.sign(&reserve(d)), Some(fp(h)))
}

fn mouth(id: [u8; 32], side: u8, freshness: u64) -> MapRecord {
    MapRecord::Mouth { place: (id, side), freshness }
}

fn throat(id: [u8; 32], span: (u64, u64), keeper: [u8; 32]) -> MapRecord {
    MapRecord::Segment { kind: Segment::Throat, a: (id, 0), b: (id, 1), span, keeper: Some(keeper) }
}

/// The next entry of `d`'s chain at `u`, made under `fp(h)`, the court's
/// final point `fp(fin)`; its hash where sealed.
fn commit(k: &mut Kernel, d: u8, u: u64, h: u64, fin: u64, applied: Vec<([u8; 32], u64)>) -> Ledgered {
    let parent = k.atlas.head(&domain(d)).map_or([0; 32], |e| e.hash);
    let e = LedgerEntry { domain: domain(d), u, commitment: bytes_of(u), events: [0; 32], point: fp(h), parent, applied, nonce: [0; 32] };
    k.ledger(&e.sign(&reserve(d)), Some(fp(fin)))
}

fn stake(k: &mut Kernel, who: &SigningKey, mu: [u64; 4], unstake: bool, nonce: u8) -> Result<[u64; 4], StakeRefusal> {
    k.stake(&StakeAct { who: vk(who), unstake, mu, nonce: [nonce; 32] }.sign(who))
}

/// A traverse from domain 4 at `fp(at)`, citing both mouths' points and
/// both universes' latest entries, over `route`, with `carried` declared by
/// both reserves (none where `None`).
fn traverse(k: &mut Kernel, m: [u8; 32], at: u64, route: Vec<[u8; 32]>, carried: Option<[u64; 4]>, nonce: u8) -> Crossed {
    let head = |k: &Kernel, d: u8| k.atlas.head(&domain(d)).map_or(([0; 32], 0), |e| (e.hash, e.entry.u));
    let ((entry_a, u_a), (entry_b, u_b)) = (head(k, 4), head(k, 5));
    let point = |k: &Kernel, d: u8, side: u8| {
        let id = crate::server::cognition::atlas::mouth_id(&domain(d), &m, side);
        k.atlas.records.get(&crate::server::cognition::atlas::record_key(&domain(d), &id)).map_or(fp(0), |r| r.point)
    };
    let (point_a, point_b) = (point(k, 4, 0), point(k, 5, 1));
    let payer = key(60);
    let mut t = Traverse { from: domain(4), payer: vk(&payer), mouth: m, side: 0, at: fp(at), point_a, point_b, entry_a, u_a, entry_b, u_b, route, debit: None, credit: None, nonce: [nonce; 32] };
    if let Some(lanes) = carried {
        t.debit = Some(Declaration { lanes, sig: reserve(4).sign(&t.debit_bytes(&lanes)).to_bytes() });
        t.credit = Some(Declaration { lanes, sig: reserve(5).sign(&t.credit_bytes(&lanes)).to_bytes() });
    }
    k.traverse(&t.sign(&payer), Some(fp(at)))
}

/// THE MAP, THEN THE TRAVERSE, END TO END: both mouths mapped and a throat
/// segment on each side; both ledgers committed; the payer staked; the
/// traverse reads the map at its final point. Its gas is exact per lane —
/// S the route's span 3/2 + 5/4 = 11/4 Mpc, G and H what both reserves
/// declared, T the ticks between the mouths' mapped points — priced at the
/// court's read price (a calm court: one μ a unit), claimed on the stake,
/// and paid whole to the court when the window closes; never minted. The
/// edge reads pending until the receiver applies the crossing in its next
/// entry, then its residual.
#[test]
fn traverse_pays_exact_gas_to_map_keeper() {
    let (mut k, m, keeper) = founded();
    assert!(matches!(map(&mut k, 4, 10, mouth(m, 0, 3)), Mapped::Sealed { .. }));
    assert!(matches!(map(&mut k, 5, 12, mouth(m, 1, 2)), Mapped::Sealed { .. }));
    let Mapped::Sealed { record: ta, .. } = map(&mut k, 4, 12, throat(m, (3, 2), keeper)) else { panic!("A's throat") };
    let Mapped::Sealed { record: tb, .. } = map(&mut k, 5, 12, throat(m, (5, 4), keeper)) else { panic!("B's throat") };
    assert_ne!(ta, tb, "one throat mapped by both universes is two records");
    let read = k.map_reading(&domain(4));
    assert_eq!(read.len(), 2, "a mouth and a throat");
    assert!(read.iter().any(|r| r.places.len() == 1), "the mouth's place read off its tube");
    assert!(matches!(commit(&mut k, 4, 100, 12, 12, Vec::new()), Ledgered::Sealed { .. }));
    assert!(matches!(commit(&mut k, 5, 7, 12, 12, Vec::new()), Ledgered::Sealed { .. }));
    let payer = key(60);
    fund(&mut k, vk(&payer), [10, 10, 10, 10]);
    assert_eq!(stake(&mut k, &payer, [10, 10, 10, 10], false, 1), Ok([10, 10, 10, 10]));
    let court = k.identity;
    let court_before = k.treasury.balance_mu(&court);
    let minted_before = k.treasury.minted();
    let price = k.read_price().expect("a calm court reads its price");
    assert_eq!(price, [1, 1, 1, 1], "a unit of gas at rest is one μ");
    let Crossed::Sealed { crossing, gas, fee, price: priced, h, .. } = traverse(&mut k, m, 14, vec![ta, tb], Some([5, 2, 0, 3]), 1) else { panic!("crossed") };
    assert_eq!(gas, Gas { s: (11, 4), g: 2, t: 2, h: 3 }, "per lane, exact, never collapsed");
    assert_eq!(priced, price);
    assert_eq!(fee, [2, 2, 2, 3], "⌊11/4⌋ on S, the rest whole");
    assert_eq!(k.treasury.stake_of(&vk(&payer)), Some([8, 8, 8, 7]), "drawn from the stake");
    assert_eq!(k.claims_on(&vk(&payer)), Some([2, 2, 2, 3]), "held as a claim");
    assert_eq!(k.treasury.balance_mu(&court), court_before, "held, not yet paid");
    let edge = k.crossing_edge(&crossing).expect("the edge on the map");
    assert_eq!((edge.u_a, edge.u_entry, edge.u_applied, edge.residual()), (100, 7, None, None), "pending, never a disagreement");
    pass(&mut k, h + CHALLENGE_WINDOW);
    k.settle_windows();
    let paid: [u64; 4] = std::array::from_fn(|l| k.treasury.balance_mu(&court)[l] - court_before[l]);
    assert_eq!(paid, [2, 2, 2, 3], "paid WHOLE to the court keeping the map");
    assert_eq!(k.treasury.minted(), minted_before, "a transfer, never a mint");
    assert_eq!(k.claims_on(&vk(&payer)), Some([0; 4]));
    assert!(k.treasury.audit().is_ok(), "both equations hold");
    assert!(matches!(commit(&mut k, 5, 9, 14, 14, vec![(crossing, 9)]), Ledgered::Sealed { .. }));
    let edge = k.crossing_edge(&crossing).expect("the edge");
    assert_eq!((edge.u_applied, edge.residual()), (Some(9), Some(2)), "applied at 9 against the entry's 7");
    assert_eq!(commit(&mut k, 5, 10, 14, 14, vec![(crossing, 10)]), Ledgered::Refused(LedgerRefusal::Applied), "applied once");
}

/// AN UNMAPPED MOUTH IS UNTRAVERSABLE: neither side mapped, then A's side
/// alone; a route segment on no map; a throat's keeper not the mouth's; a
/// side in another universe — each refused by name.
#[test]
fn traverse_refuses_unmapped_mouth() {
    let (mut k, m, keeper) = founded();
    assert!(matches!(commit(&mut k, 4, 1, 10, 10, Vec::new()), Ledgered::Sealed { .. }));
    assert!(matches!(commit(&mut k, 5, 1, 10, 10, Vec::new()), Ledgered::Sealed { .. }));
    assert_eq!(traverse(&mut k, m, 10, Vec::new(), None, 1), Crossed::Refused(CrossRefusal::Unmapped(Side::A)));
    assert_eq!(map(&mut k, 4, 10, mouth(m, 1, 3)), Mapped::Refused(MapRefusal::NotThisSide), "B's side is not A's to map");
    assert_eq!(map(&mut k, 4, 10, throat(m, (1, 1), keeper)), Mapped::Refused(MapRefusal::Unmapped), "a throat from an unmapped side");
    assert!(matches!(map(&mut k, 4, 10, mouth(m, 0, 3)), Mapped::Sealed { .. }));
    assert_eq!(map(&mut k, 4, 10, throat(m, (1, 1), [7; 32])), Mapped::Refused(MapRefusal::Keeper));
    assert_eq!(map(&mut k, 4, 11, mouth([8; 32], 0, 3)), Mapped::Refused(MapRefusal::NoMouth));
    assert_eq!(traverse(&mut k, m, 10, Vec::new(), None, 1), Crossed::Refused(CrossRefusal::Unmapped(Side::B)));
    assert!(matches!(map(&mut k, 5, 10, mouth(m, 1, 3)), Mapped::Sealed { .. }));
    assert_eq!(traverse(&mut k, m, 10, vec![[3; 32]], None, 1), Crossed::Refused(CrossRefusal::Route));
    assert!(matches!(traverse(&mut k, m, 10, Vec::new(), None, 1), Crossed::Sealed { .. }), "both mapped: coordinates served");
    assert_eq!(traverse(&mut k, m, 10, Vec::new(), None, 1), Crossed::Refused(CrossRefusal::Crossed), "one crossing, once");
}

/// A STALE ENTRY IS NOT YET AT h, EACH SIDE BY ITS OWN MOUTH'S N: the final
/// point advances to 20 (14, 16, 18, 20 held); A's entry at 12 stands four
/// advances behind its N of 3; renewed at 18, A is fresh and B — N 2, its
/// entry at 12 — is named; B commits at 20 and the crossing is admitted.
#[test]
fn traverse_refuses_stale_entry_per_side() {
    let (mut k, m, _) = founded();
    assert!(matches!(map(&mut k, 4, 10, mouth(m, 0, 3)), Mapped::Sealed { .. }));
    assert!(matches!(map(&mut k, 5, 10, mouth(m, 1, 2)), Mapped::Sealed { .. }));
    assert!(matches!(commit(&mut k, 4, 1, 12, 12, Vec::new()), Ledgered::Sealed { .. }));
    assert!(matches!(commit(&mut k, 5, 1, 12, 12, Vec::new()), Ledgered::Sealed { .. }));
    for h in [14, 16, 18] {
        k.note_final(fp(h));
    }
    assert_eq!(traverse(&mut k, m, 20, Vec::new(), None, 1), Crossed::Refused(CrossRefusal::NotYetAtH(Side::A)));
    assert!(matches!(commit(&mut k, 4, 2, 18, 20, Vec::new()), Ledgered::Sealed { .. }));
    assert_eq!(traverse(&mut k, m, 20, Vec::new(), None, 2), Crossed::Refused(CrossRefusal::NotYetAtH(Side::B)), "a lax mouth never sets the other side's bar");
    assert!(matches!(commit(&mut k, 5, 2, 20, 20, Vec::new()), Ledgered::Sealed { .. }));
    assert!(matches!(traverse(&mut k, m, 20, Vec::new(), None, 3), Crossed::Sealed { .. }));
}

/// THE DECLARATIONS ARE BOTH OR NEITHER, EACH ITS RESERVE'S, EQUAL: one
/// side alone, a credit not B's reserve's, a credit differing on a lane —
/// each refused; the court reads no book.
#[test]
fn traverse_refuses_one_sided_or_unequal_declarations() {
    let (mut k, m, _) = founded();
    assert!(matches!(map(&mut k, 4, 10, mouth(m, 0, 3)), Mapped::Sealed { .. }));
    assert!(matches!(map(&mut k, 5, 10, mouth(m, 1, 3)), Mapped::Sealed { .. }));
    assert!(matches!(commit(&mut k, 4, 1, 10, 10, Vec::new()), Ledgered::Sealed { .. }));
    assert!(matches!(commit(&mut k, 5, 1, 10, 10, Vec::new()), Ledgered::Sealed { .. }));
    let payer = key(60);
    let base = |k: &Kernel| {
        let (a, b) = (k.atlas.head(&domain(4)).expect("A"), k.atlas.head(&domain(5)).expect("B"));
        Traverse {
            from: domain(4),
            payer: vk(&payer),
            mouth: m,
            side: 0,
            at: fp(10),
            point_a: fp(10),
            point_b: fp(10),
            entry_a: a.hash,
            u_a: a.entry.u,
            entry_b: b.hash,
            u_b: b.entry.u,
            route: Vec::new(),
            debit: None,
            credit: None,
            nonce: [1; 32],
        }
    };
    let lanes = [4, 0, 0, 1];
    let mut one = base(&k);
    one.debit = Some(Declaration { lanes, sig: reserve(4).sign(&one.debit_bytes(&lanes)).to_bytes() });
    assert_eq!(k.traverse(&one.clone().sign(&payer), Some(fp(10))), Crossed::Refused(CrossRefusal::OneSided));
    let mut forged = one.clone();
    forged.credit = Some(Declaration { lanes, sig: reserve(4).sign(&forged.credit_bytes(&lanes)).to_bytes() });
    assert_eq!(k.traverse(&forged.sign(&payer), Some(fp(10))), Crossed::Refused(CrossRefusal::DeclarationUnsigned(Side::B)));
    let more = [4, 0, 0, 2];
    let mut minted = one.clone();
    minted.credit = Some(Declaration { lanes: more, sig: reserve(5).sign(&minted.credit_bytes(&more)).to_bytes() });
    assert_eq!(k.traverse(&minted.sign(&payer), Some(fp(10))), Crossed::Refused(CrossRefusal::Mismatched), "no universe mints matter at a throat");
}

/// THE LEDGER CHAINS, NEVER JUDGES: entries chain by parent from zero; a
/// fork (a parent not the head) is refused; `u` must rise; the point must
/// be final in this court's view and never below the head's; the signer
/// the sitting reserve — the founder's key is not, a stranger's is no key
/// of the domain; an unknown domain. Read after `u`, with each segment's
/// dilation `Δu/Δh` exact; carried across a restart.
#[test]
fn ledger_chains_refuses_forks_and_reads_dilation() {
    let (mut k, _, _) = founded();
    let entry = |u: u64, h: u64, parent: [u8; 32]| LedgerEntry { domain: domain(4), u, commitment: [1; 32], events: [2; 32], point: fp(h), parent, applied: Vec::new(), nonce: [0; 32] };
    assert_eq!(k.ledger(&entry(1, 10, [3; 32]).sign(&reserve(4)), Some(fp(10))), Ledgered::Refused(LedgerRefusal::Parent), "a first entry names no parent");
    let Ledgered::Sealed { entry: first, point, .. } = k.ledger(&entry(10, 10, [0; 32]).sign(&reserve(4)), Some(fp(10))) else { panic!("the first") };
    assert_eq!(point, fp(10), "sealed at the court's final point");
    assert_eq!(k.ledger(&entry(11, 10, [0; 32]).sign(&reserve(4)), Some(fp(10))), Ledgered::Refused(LedgerRefusal::Parent), "a fork");
    assert_eq!(k.ledger(&entry(10, 10, first).sign(&reserve(4)), Some(fp(10))), Ledgered::Refused(LedgerRefusal::NotAbove));
    assert_eq!(k.ledger(&entry(9, 10, first).sign(&reserve(4)), Some(fp(10))), Ledgered::Refused(LedgerRefusal::NotAbove));
    assert_eq!(k.ledger(&entry(12, 13, first).sign(&reserve(4)), Some(fp(12))), Ledgered::Refused(LedgerRefusal::NotFinal), "above the court's point");
    assert_eq!(k.ledger(&entry(12, 12, first).sign(&reserve(4)), None), Ledgered::Refused(LedgerRefusal::NotFinal), "no final point read");
    assert_eq!(k.ledger(&entry(12, 9, first).sign(&reserve(4)), Some(fp(12))), Ledgered::Refused(LedgerRefusal::Behind));
    let other_lanes = TPoint { lanes: [1, 1, 1, 1], ..fp(10) };
    let forged = LedgerEntry { point: other_lanes, ..entry(12, 10, first) };
    assert_eq!(k.ledger(&forged.sign(&reserve(4)), Some(fp(12))), Ledgered::Refused(LedgerRefusal::NotFinal), "a point at a held tick that is not the point held");
    assert_eq!(k.ledger(&entry(12, 12, first).sign(&key(4)), Some(fp(12))), Ledgered::Refused(LedgerRefusal::NotReserve), "the founder is not the reserve");
    assert_eq!(k.ledger(&entry(12, 12, first).sign(&key(77)), Some(fp(12))), Ledgered::Refused(LedgerRefusal::Unsigned));
    let stranger = LedgerEntry { domain: [77; 32], ..entry(1, 10, [0; 32]) };
    assert_eq!(k.ledger(&stranger.sign(&reserve(4)), Some(fp(10))), Ledgered::Refused(LedgerRefusal::UnknownDomain));
    let Ledgered::Sealed { entry: second, .. } = k.ledger(&entry(13, 25, first).sign(&reserve(4)), Some(fp(25))) else { panic!("the second") };
    assert!(matches!(k.ledger(&entry(19, 25, second).sign(&reserve(4)), Some(fp(25))), Ledgered::Sealed { .. }), "h held: dilation unmeasured");
    assert_eq!(k.dilations(&domain(4)), vec![(10, None), (13, Some((1, 5))), (19, None)], "Δu/Δh = 3/15, reduced; Δh = 0 unmeasured");
    let page = k.ledger_page(&domain(4), 10);
    assert_eq!(page.entries.iter().map(|e| e.entry.u).collect::<Vec<_>>(), vec![13, 19]);
    assert!(!page.more);
    let mut booted = Kernel::new();
    assert!(booted.restore(&k.snapshot()), "restores");
    assert_eq!(booted.dilations(&domain(4)), k.dilations(&domain(4)), "the chain across a restart");
    assert_eq!(booted.atlas.head(&domain(4)).map(|e| e.entry.u), Some(19));
}

/// THE RECYCLED POOL PAYS POUW BEFORE ANY MINT: a reserve's fold declares
/// its CONNECTION_TAX — a sink on the lanes it dissipated — and is admitted
/// with it paid into the court's pool; a solve's POUW is paid from the pool
/// first and only the remainder minted. A source lane, a reserve short, a
/// carried line declaring it — each refuses the fold whole.
#[test]
fn settle_solve_pays_from_recycled_pool_before_mint() {
    let (mut k, _, _) = founded();
    let tier = vk(&reserve(4));
    fund(&mut k, tier, [3, 0, 2, 0]);
    let tax = |lanes: [i128; 4], prefix: u8| TierFold {
        tier,
        prefix: [prefix; 32],
        rows: vec![([1; 32], lanes)],
        declared: vec![(CONNECTION_TAX, lanes)],
        children: Vec::new(),
        handoffs: Vec::new(),
    };
    assert_eq!(k.admit_uplink(&tax([-3, 1, -2, 0], 1).sign(&reserve(4))), Uplinked::Tax(TaxRefusal::Source { lane: 1 }), "the tax only leaves");
    assert_eq!(k.admit_uplink(&tax([-4, 0, -2, 0], 1).sign(&reserve(4))), Uplinked::Tax(TaxRefusal::Short));
    let minted_before = k.treasury.minted().expect("minted reads");
    assert!(matches!(k.admit_uplink(&tax([-3, 0, -2, 0], 1).sign(&reserve(4))), Uplinked::Admitted { .. }));
    assert_eq!(k.treasury.recycled(), [3, 0, 2, 0], "the dissipation returned to the hypha");
    assert_eq!(k.treasury.balance_mu(&tier), [0; 4], "paid from the reserve");
    assert_eq!(k.treasury.minted(), Some(minted_before), "a transfer, never a mint");
    let closer = [70; 32];
    let anchor = k.anchor();
    assert_eq!(k.treasury.settle_solve(closer, [5, 1, 1, 0], anchor), [5, 1, 1, 0], "the closer paid whole");
    assert_eq!(k.treasury.balance_mu(&closer), [5, 1, 1, 0]);
    assert_eq!(k.treasury.recycled(), [0, 0, 1, 0], "the pool paid first");
    let minted: [u64; 4] = std::array::from_fn(|l| k.treasury.minted().expect("reads")[l] - minted_before[l]);
    assert_eq!(minted, [2, 1, 0, 0], "only the remainder minted");
    assert!(k.treasury.audit().is_ok());
    let Some(_) = k.treasury.held_of(crate::economy::RECYCLED) else { panic!("the pool held by the court") };
    assert_eq!(k.treasury.held_of(crate::economy::RECYCLED).map(|h| h.payer), Some(k.identity));
}

/// THE STAKE IS ESCROW THAT DOES NOT DECAY: staked from the balance (short
/// refused, a resent act moving nothing); a traverse's fee and a coordinate
/// retrieval each draw a claim; an unstake is refused while a claim stands,
/// admitted once every claim drawn on it settled.
#[test]
fn stake_unstakes_only_after_claims_settle() {
    let (mut k, m, _) = founded();
    let payer = key(60);
    fund(&mut k, vk(&payer), [4, 4, 4, 4]);
    assert_eq!(stake(&mut k, &payer, [5, 0, 0, 0], false, 1), Err(StakeRefusal::Short));
    assert_eq!(stake(&mut k, &payer, [4, 4, 4, 4], false, 1), Ok([4, 4, 4, 4]));
    assert_eq!(stake(&mut k, &payer, [0, 0, 0, 1], false, 1), Err(StakeRefusal::Spent), "a resend moves nothing");
    assert_eq!(k.treasury.balance_mu(&vk(&payer)), [0; 4]);
    assert!(matches!(map(&mut k, 4, 10, mouth(m, 0, 3)), Mapped::Sealed { .. }));
    assert!(matches!(map(&mut k, 5, 12, mouth(m, 1, 3)), Mapped::Sealed { .. }));
    assert!(matches!(commit(&mut k, 4, 1, 10, 12, Vec::new()), Ledgered::Sealed { .. }));
    assert!(matches!(commit(&mut k, 5, 1, 12, 12, Vec::new()), Ledgered::Sealed { .. }));
    let Crossed::Sealed { fee, h, .. } = traverse(&mut k, m, 12, Vec::new(), Some([0, 0, 0, 1]), 1) else { panic!("crossed") };
    assert_eq!(fee, [0, 0, 2, 1], "T: the mouths mapped two ticks apart");
    assert_eq!(stake(&mut k, &payer, [1, 0, 0, 0], true, 2), Err(StakeRefusal::Claimed), "a claim stands on the stake");
    let read = k.retrieve(vk(&payer), &fp(10), Some(fp(12)), [1; 32]).expect("served");
    let units = read.len() as u64;
    assert_eq!(k.claims_on(&vk(&payer)), Some([0, 0, 2, 1 + units]), "one unit of information per receipt read");
    assert_eq!(k.retrieve(vk(&payer), &fp(13), Some(fp(12)), [2; 32]), Err(crate::server::cognition::crossing::Unretrieved::NotFinal));
    pass(&mut k, h + CHALLENGE_WINDOW + 1);
    k.settle_windows();
    assert_eq!(k.claims_on(&vk(&payer)), Some([0; 4]), "settled");
    let left = k.treasury.stake_of(&vk(&payer)).expect("staked");
    assert_eq!(left, [4, 4, 2, 3 - units]);
    assert_eq!(stake(&mut k, &payer, left, true, 3), Ok([0; 4]), "unstaked once every claim settled");
    assert_eq!(k.treasury.balance_mu(&vk(&payer)), left);
    assert!(k.treasury.audit().is_ok());
}

fn q(v: i64) -> num_rational::BigRational {
    num_rational::BigRational::from_integer(v.into())
}

/// A slide by `(x, y)` in the torus's plane: a motion of flat space.
fn slide(x: i64, y: i64) -> Mv {
    crate::transducer::cga::translator_at(&[q(x), q(y), q(0)])
}

/// Domain 4's universe a flat torus of periods 30 and 20.
fn torus_for_4(k: &mut Kernel) {
    let space = Geometry::Torus { periods: vec![(30, 1), (20, 1)], metric: Metric::L1 };
    assert!(matches!(k.state(&Statement { domain: domain(4), stated: Stated::Geometry(space) }.sign(&reserve(4))), Founded::Stated { .. }));
}

/// A local throat of domain 4 by the progenitor line `keeper`, its ends
/// slid to `(32, −17)` and `(5, 4)`.
fn local(keeper: [u8; 32], freshness: u64) -> MapRecord {
    MapRecord::LocalThroat { throat: [42; 32], keeper, frames: (slide(32, -17), slide(5, 4)), freshness }
}

/// A LOCAL THROAT MAPS WITHOUT A MOUTH PAIR: refused in a universe that
/// declared no geometry, with a frame zero or no versor, with an end that is
/// no motion of the torus, or signed by the founder; mapped, its id the
/// derived one and each end's place read exactly — `(32, −17)` modulo
/// `(30, 20)` is `(2, 3)` — and both ends mapped places of the universe: a
/// throat between them kept by its keeper (another keeper refused), a road
/// from one of them.
#[test]
fn map_local_throat_reads_ends_exactly() {
    use crate::transducer::cga::{base_vector, dilator};
    let (mut k, m, _) = founded();
    let line = [77; 32];
    assert_eq!(map(&mut k, 4, 10, local(line, 3)), Mapped::Refused(MapRefusal::NoGeometry), "no declared geometry: no place reads");
    torus_for_4(&mut k);
    let mixed = Mv::scalar(q(1)).add(&base_vector(&[q(1), q(0)]));
    for bad in [Mv::zero(), mixed] {
        let record = MapRecord::LocalThroat { throat: [42; 32], keeper: line, frames: (slide(1, 1), bad), freshness: 3 };
        assert_eq!(map(&mut k, 4, 10, record), Mapped::Refused(MapRefusal::NotVersor));
    }
    for off in [crate::transducer::cga::translator_at(&[q(0), q(0), q(1)]), dilator(&q(2))] {
        let record = MapRecord::LocalThroat { throat: [42; 32], keeper: line, frames: (off, slide(5, 4)), freshness: 3 };
        assert_eq!(map(&mut k, 4, 10, record), Mapped::Refused(MapRefusal::Unplaced), "no motion of the torus");
    }
    let founder = MapAct { domain: domain(4), point: fp(10), record: local(line, 3), nonce: [1; 32] }.sign(&key(4));
    assert_eq!(k.map(&founder, Some(fp(10))), Mapped::Refused(MapRefusal::NotReserve));
    let Mapped::Sealed { record, .. } = map(&mut k, 4, 10, local(line, 3)) else { panic!("mapped") };
    let mut material = domain(4).to_vec();
    material.extend_from_slice(&[42; 32]);
    let lid = blake3::derive_key("mycelium local throat v1", &material);
    assert_eq!(local_throat_id(&domain(4), &[42; 32]), lid, "the derived mouth id");
    assert_eq!(record, local_record_id(&domain(4), &lid));
    let read = k.map_reading(&domain(4));
    let entry = read.iter().find(|e| matches!(e.held.record, MapRecord::LocalThroat { .. })).expect("read");
    let [p0, p1] = entry.places.as_slice() else { panic!("both ends") };
    assert!(p0.sub(&base_vector(&[q(2), q(3)])).is_zero(), "(32, −17) modulo (30, 20), exactly");
    assert!(p1.sub(&base_vector(&[q(5), q(4)])).is_zero());
    let own = |keeper: [u8; 32]| MapRecord::Segment { kind: Segment::Throat, a: (lid, 0), b: (lid, 1), span: (3, 2), keeper: Some(keeper) };
    assert_eq!(map(&mut k, 4, 11, own([7; 32])), Mapped::Refused(MapRefusal::Keeper));
    assert!(matches!(map(&mut k, 4, 11, own(line)), Mapped::Sealed { .. }), "its throat, kept by its keeper");
    assert!(matches!(map(&mut k, 4, 11, mouth(m, 0, 3)), Mapped::Sealed { .. }));
    let road = MapRecord::Segment { kind: Segment::Tube, a: (lid, 1), b: (m, 0), span: (1, 1), keeper: None };
    assert!(matches!(map(&mut k, 4, 12, road), Mapped::Sealed { .. }), "a road from a local throat's end");
    assert_eq!(map(&mut k, 5, 12, MapRecord::Segment { kind: Segment::Tube, a: (lid, 0), b: (m, 1), span: (1, 1), keeper: None }), Mapped::Refused(MapRefusal::Unmapped), "another universe's");
}

/// A TRAVERSE OVER A LOCAL THROAT, END TO END: both ends in domain 4, so B
/// is A; its freshness the record's N; the gas S 3/2 (its throat segment), G
/// and H as declared, T 0 (one record, one mapped point); the fee `[1, 2, 0,
/// 3]` claimed and paid whole at the window, nothing minted; the edge
/// pending, then applied in A's next entry at 101 against 100: residual 1.
#[test]
fn traverse_local_throat_pays_and_applies() {
    let (mut k, _, _) = founded();
    torus_for_4(&mut k);
    let line = [77; 32];
    assert!(matches!(map(&mut k, 4, 10, local(line, 3)), Mapped::Sealed { .. }));
    let lid = local_throat_id(&domain(4), &[42; 32]);
    let seg = MapRecord::Segment { kind: Segment::Throat, a: (lid, 0), b: (lid, 1), span: (3, 2), keeper: Some(line) };
    let Mapped::Sealed { record: seg, .. } = map(&mut k, 4, 10, seg) else { panic!("its throat") };
    let Ledgered::Sealed { entry, .. } = commit(&mut k, 4, 100, 12, 12, Vec::new()) else { panic!("committed") };
    let payer = key(60);
    fund(&mut k, vk(&payer), [10, 10, 10, 10]);
    assert_eq!(stake(&mut k, &payer, [10, 10, 10, 10], false, 1), Ok([10, 10, 10, 10]));
    let court = k.identity;
    let (court_before, minted_before) = (k.treasury.balance_mu(&court), k.treasury.minted());
    let mut t = Traverse {
        from: domain(4),
        payer: vk(&payer),
        mouth: lid,
        side: 0,
        at: fp(14),
        point_a: fp(10),
        point_b: fp(10),
        entry_a: entry,
        u_a: 100,
        entry_b: entry,
        u_b: 100,
        route: vec![seg],
        debit: None,
        credit: None,
        nonce: [1; 32],
    };
    let lanes = [5, 2, 0, 3];
    t.debit = Some(Declaration { lanes, sig: reserve(4).sign(&t.debit_bytes(&lanes)).to_bytes() });
    t.credit = Some(Declaration { lanes, sig: reserve(4).sign(&t.credit_bytes(&lanes)).to_bytes() });
    let unmapped = Traverse { mouth: local_throat_id(&domain(4), &[43; 32]), ..t.clone() };
    assert_eq!(k.traverse(&unmapped.sign(&payer), Some(fp(14))), Crossed::Refused(CrossRefusal::NoMouth), "a throat key never mapped");
    let Crossed::Sealed { crossing, gas, fee, h, .. } = k.traverse(&t.sign(&payer), Some(fp(14))) else { panic!("crossed") };
    assert_eq!(gas, Gas { s: (3, 2), g: 2, t: 0, h: 3 });
    assert_eq!(fee, [1, 2, 0, 3], "⌊3/2⌋ on S");
    assert_eq!(k.claims_on(&vk(&payer)), Some([1, 2, 0, 3]));
    let edge = k.crossing_edge(&crossing).expect("the edge");
    assert_eq!((edge.from, edge.to, edge.u_entry, edge.residual()), (domain(4), domain(4), 100, None), "pending, into its own universe");
    pass(&mut k, h + CHALLENGE_WINDOW);
    k.settle_windows();
    let paid: [u64; 4] = std::array::from_fn(|l| k.treasury.balance_mu(&court)[l] - court_before[l]);
    assert_eq!(paid, [1, 2, 0, 3], "paid whole to the court keeping the map");
    assert_eq!(k.treasury.minted(), minted_before, "never a mint");
    assert!(matches!(commit(&mut k, 4, 101, 14, 14, vec![(crossing, 101)]), Ledgered::Sealed { .. }));
    assert_eq!(k.crossing_edge(&crossing).and_then(|e| e.residual()), Some(1), "applied at 101 against 100");
    assert!(k.treasury.audit().is_ok());
}
