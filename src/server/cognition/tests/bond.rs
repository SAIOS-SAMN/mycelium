// 78.129 — BONDS AND THE SLASH: a tier's bond posted by its domain's reserve
// (topped up, withdrawn only once every fold of the tier is final), the
// bonded flag a finality read carries, and a falsified fold slashing its
// bond against its exposure, the challenger paid by the defect it proved
// (MU-DERIVATIONS D2.1, D2.2).

use crate::anastome::dialogue::challenge::{line_head, Challenge, Challenged, Entry};
use crate::anastome::dialogue::domain::{Bond, DomainGenesis, Founded, Reservation};
use crate::anastome::dialogue::uplink::{Child, ChildNet, FoldAt, TierFold, Uplinked};
use crate::economy::tests::Counts;
use crate::server::cognition::Court;
use ed25519_dalek::SigningKey;

fn key(b: u8) -> SigningKey {
    SigningKey::from_bytes(&[b; 32])
}

fn verifying(k: &SigningKey) -> [u8; 32] {
    k.verifying_key().to_bytes()
}

fn domain() -> SigningKey {
    key(4)
}
/// The reserve, and the tier it backs.
fn reserve() -> SigningKey {
    key(21)
}
fn tier() -> SigningKey {
    key(22)
}
fn challenger() -> SigningKey {
    key(40)
}

fn founded() -> Court {
    let mut k = Court::new();
    let mut tiers = vec![verifying(&reserve()), verifying(&tier())];
    tiers.sort_unstable();
    let g = DomainGenesis { domain: verifying(&domain()), tiers };
    assert!(matches!(k.hear_domain(&g.sign(&domain())), Founded::Sealed { .. }));
    let r = Reservation { domain: verifying(&domain()), tier: verifying(&reserve()), nonce: [0; 32] };
    assert!(matches!(k.designate_reserve(&r.sign(&domain())), Founded::Reserved { .. }));
    let anchor = k.anchor();
    k.treasury.genesis(verifying(&reserve()), [10, 0, 0, 0], anchor);
    k
}

fn bond(mu: Option<[u64; 4]>, nonce: u8) -> Bond {
    Bond { domain: verifying(&domain()), reserve: verifying(&reserve()), tier: verifying(&tier()), mu: mu.map(Into::into), nonce: [nonce; 32] }
}

/// The tier's bond, as counts, where a bond was answered.
fn bonded(f: Founded) -> Option<[u64; 4]> {
    bonded_exactly(f).map(Counts::n)
}

/// The tier's bond, exactly, where a bond was answered.
fn bonded_exactly(f: Founded) -> Option<crate::economy::Mu> {
    match f {
        Founded::Bonded { bond, .. } => Some(bond),
        _ => None,
    }
}

/// The tier's fold whose rows its line does not sum to: `w1` 5, `w2` −5 on
/// S folded, but the line moved 4 and −4 — a lie a reconstruction catches.
fn lying_fold() -> (TierFold, Vec<Entry>) {
    let line = vec![Entry { rows: vec![([1; 32], [4, 0, 0, 0]), ([2; 32], [-4, 0, 0, 0])], declared: Vec::new() }];
    let f = TierFold {
        tier: verifying(&tier()),
        prefix: line_head([0; 32], &line),
        rows: vec![([1; 32], [5, 0, 0, 0]), ([2; 32], [-5, 0, 0, 0])],
        declared: Vec::new(),
        children: Vec::new(),
        handoffs: Vec::new(),
        transitions: Vec::new(),
    };
    (f, line)
}

/// 78.138 — THE TIER'S FOLD CARRIED in a fresh fold of its reserve, which
/// includes none of it: where it stands, `[0]` inside the receipt — or the
/// refusal.
fn carry(k: &mut Court, f: TierFold, nonce: u8) -> Result<FoldAt, Uplinked> {
    let carried = ChildNet {
        rows: f.rows.clone(),
        declared: f.declared.clone(),
        child: Child::Carried(Box::new(f.sign(&tier()))),
        included_rows: Vec::new(),
        included_declared: Vec::new(),
        rate: [(1, 1); 4],
    };
    let carrier = TierFold { tier: verifying(&reserve()), prefix: [nonce; 32], rows: Vec::new(), declared: Vec::new(), children: vec![carried], handoffs: Vec::new(), transitions: Vec::new() };
    match k.hear_uplink(&carrier.sign(&reserve())) {
        Uplinked::Chained { at, .. } => Ok(FoldAt { stamp: at, path: vec![0] }),
        other => Err(other),
    }
}

/// THE RESERVE BONDS ITS TIERS: only the reserve, for a tier its domain
/// names, from μ it holds; a top-up adds to the bond; a fold of a bonded tier
/// reads bonded, an unbonded one not. 78.138 — an unbonded line is never
/// carried.
#[test]
fn bond_posts_from_reserve_and_fold_reads_bonded() {
    let mut k = founded();
    let refused = carry(&mut k, lying_fold().0, 8);
    assert_eq!(refused, Err(Uplinked::ChildRefused { index: 0, why: crate::anastome::dialogue::uplink::ChildRefusal::Unbonded }), "each answers for its own work");
    let refused = |k: &mut Court, b: Bond, by: &SigningKey| k.bond(&b.sign(by));
    assert_eq!(refused(&mut k, Bond { reserve: verifying(&tier()), ..bond(Some([1, 0, 0, 0]), 1) }, &tier()), Founded::NotReserve);
    assert_eq!(refused(&mut k, bond(Some([1, 0, 0, 0]), 1), &tier()), Founded::Unsigned);
    assert_eq!(refused(&mut k, Bond { tier: [9; 32], ..bond(Some([1, 0, 0, 0]), 1) }, &reserve()), Founded::NotATier);
    assert_eq!(refused(&mut k, bond(Some([11, 0, 0, 0]), 1), &reserve()), Founded::Short, "only from what the reserve holds");
    assert_eq!(bonded(k.bond(&bond(Some([4, 0, 0, 0]), 1).sign(&reserve()))), Some([4, 0, 0, 0]));
    assert_eq!(bonded(k.bond(&bond(Some([3, 0, 0, 0]), 2).sign(&reserve()))), Some([7, 0, 0, 0]), "topped up");
    assert_eq!((k.bond_of(&verifying(&tier())), k.treasury.balance_mu(&verifying(&reserve()))).n(), ([7, 0, 0, 0], [3, 0, 0, 0]));
    let (f, _) = lying_fold();
    let at = carry(&mut k, f, 9).expect("carried, the line bonded");
    assert!(k.fold_bonded(&at), "a fold of a bonded tier");
    let Uplinked::Admitted { at: bare, .. } = k.hear_uplink(&TierFold { prefix: [3; 32], rows: Vec::new(), tier: verifying(&reserve()), ..lying_fold().0 }.sign(&reserve())) else {
        panic!("admitted")
    };
    assert!(!k.fold_bonded(&FoldAt::sealed(bare)), "an unbacked fold reads unbonded");
    let mut booted = Court::new();
    assert!(booted.restore(&k.snapshot()), "restores");
    assert_eq!(booted.bond_of(&verifying(&tier())), [7, 0, 0, 0], "the bond held across a restart");
}

/// A FALSIFIED FOLD SLASHES ITS BOND AGAINST ITS EXPOSURE, AND NO BOND LEAVES
/// WHILE A FOLD IT BACKS IS OPEN: withdrawal waits on the tier's open fold; the
/// reconstruction falsifies it — 7 bonded on S against an exposure of 5 (the
/// 5 the fold moved to `w1`) cuts 7·5/12 = 35/12; the defect 2 (the fold's 5
/// and −5 against the line's 4 and −4) against the exposure pays the
/// challenger 35/12 · 2/7 = 5/6 and retires 25/12; 49/12 stays bonded
/// (D2.1, D2.2); then, nothing outstanding, the bond withdraws to the reserve.
#[test]
fn challenge_slashes_bond_against_exposure_and_withdrawal_waits() {
    let mut k = founded();
    assert!(matches!(k.bond(&bond(Some([7, 0, 0, 0]), 1).sign(&reserve())), Founded::Bonded { .. }));
    let (f, line) = lying_fold();
    let at = carry(&mut k, f.clone(), 9).expect("carried");
    assert_eq!(k.bond(&bond(None, 2).sign(&reserve())), Founded::Outstanding, "a fold it backs is open");
    let supply_before = k.treasury.balance_mu(&verifying(&reserve())).add(&k.bond_of(&verifying(&tier())));
    let issued_before = k.treasury.supply().expect("adds up");
    let evidence = Challenge::Evidence { fold: at, rows: f.rows.clone(), declared: Vec::new(), children: Vec::new(), entries: line };
    assert!(matches!(k.challenge(&evidence.sign(&challenger())), Challenged::Falsified { .. }));
    let s = |n: u64, d: usize| crate::economy::Mu::counts([n, 0, 0, 0]).part(d).expect("a part");
    assert_eq!(k.bond_of(&verifying(&tier())), s(49, 12), "the cut 35/12 out of 7, against the exposure");
    assert_eq!(k.treasury.balance_mu(&verifying(&challenger())), s(5, 6), "the bounty: the defect against the exposure");
    let after = k.treasury.balance_mu(&verifying(&reserve())).add(&k.bond_of(&verifying(&tier())));
    assert_eq!(supply_before.draw(&after), Ok(s(35, 12)), "35/12 left the reserve: 5/6 paid, 25/12 retired");
    let issued_after = k.treasury.supply().expect("adds up");
    assert_eq!(issued_before.draw(&issued_after), Ok(s(25, 12)), "the bounty is taken from the cut, never minted: supply falls by the retired part");
    assert!(bonded_exactly(k.bond(&bond(None, 2).sign(&reserve()))).is_some_and(|b| b.is_zero()), "nothing outstanding: withdrawn");
    assert_eq!(k.bond_of(&verifying(&tier())), [0; 4]);
    assert_eq!(k.treasury.balance_mu(&verifying(&reserve())), crate::economy::Mu::counts([3, 0, 0, 0]).add(&s(49, 12)), "the kept part back to the reserve");
    assert!(k.treasury.audit().is_ok(), "both equations hold through the forfeit");
    assert_eq!(k.bond(&bond(None, 3).sign(&reserve())), Founded::Unbonded);
}
