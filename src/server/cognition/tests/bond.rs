// 78.129 — BONDS AND THE SLASH: a tier's bond posted by its domain's reserve
// (topped up, withdrawn only once every fold of the tier is final), the
// bonded flag a finality read carries, and a falsified fold slashing half
// the bond — half the cut to the challenger, half retired.

use crate::anastome::dialogue::challenge::{line_head, Challenge, Challenged, Entry};
use crate::anastome::dialogue::domain::{Bond, DomainGenesis, Founded, Reservation};
use crate::anastome::dialogue::uplink::{TierFold, Uplinked};
use crate::server::cognition::Kernel;
use ed25519_dalek::SigningKey;

fn key(b: u8) -> SigningKey {
    SigningKey::from_bytes(&[b; 32])
}

fn vk(k: &SigningKey) -> [u8; 32] {
    k.verifying_key().to_bytes()
}

fn domain() -> SigningKey {
    key(4)
}
/// The reserve (the ancients), and the tier it backs.
fn reserve() -> SigningKey {
    key(21)
}
fn tier() -> SigningKey {
    key(22)
}
fn challenger() -> SigningKey {
    key(40)
}

fn founded() -> Kernel {
    let mut k = Kernel::new();
    let mut tiers = vec![vk(&reserve()), vk(&tier())];
    tiers.sort_unstable();
    let g = DomainGenesis { domain: vk(&domain()), tiers };
    assert!(matches!(k.admit_domain(&g.sign(&domain())), Founded::Sealed { .. }));
    let r = Reservation { domain: vk(&domain()), tier: vk(&reserve()), nonce: [0; 32] };
    assert!(matches!(k.designate_reserve(&r.sign(&domain())), Founded::Reserved { .. }));
    let anchor = k.anchor();
    k.treasury.genesis(vk(&reserve()), [10, 0, 0, 0], anchor);
    k
}

fn bond(mu: Option<[u64; 4]>, nonce: u8) -> Bond {
    Bond { domain: vk(&domain()), reserve: vk(&reserve()), tier: vk(&tier()), mu, nonce: [nonce; 32] }
}

/// The tier's fold whose rows its line does not sum to: `w1` 5, `w2` −5 on
/// S folded, but the line moved 4 and −4 — a lie a replay catches.
fn lying_fold() -> (TierFold, Vec<Entry>) {
    let line = vec![Entry { rows: vec![([1; 32], [4, 0, 0, 0]), ([2; 32], [-4, 0, 0, 0])], declared: Vec::new() }];
    let f = TierFold {
        tier: vk(&tier()),
        prefix: line_head([0; 32], &line),
        rows: vec![([1; 32], [5, 0, 0, 0]), ([2; 32], [-5, 0, 0, 0])],
        declared: Vec::new(),
        children: Vec::new(),
        handoffs: Vec::new(),
    };
    (f, line)
}

/// THE RESERVE BONDS ITS TIERS: only the reserve, for a tier its domain
/// names, from μ it holds; a top-up adds to the bond; a fold of a bonded tier
/// reads bonded, an unbonded one not.
#[test]
fn the_reserve_bonds_a_tier_and_a_fold_reads_bonded() {
    let mut k = founded();
    let refused = |k: &mut Kernel, b: Bond, by: &SigningKey| k.bond(&b.sign(by));
    assert_eq!(refused(&mut k, Bond { reserve: vk(&tier()), ..bond(Some([1, 0, 0, 0]), 1) }, &tier()), Founded::NotReserve);
    assert_eq!(refused(&mut k, bond(Some([1, 0, 0, 0]), 1), &tier()), Founded::Unsigned);
    assert_eq!(refused(&mut k, Bond { tier: [9; 32], ..bond(Some([1, 0, 0, 0]), 1) }, &reserve()), Founded::NotATier);
    assert_eq!(refused(&mut k, bond(Some([11, 0, 0, 0]), 1), &reserve()), Founded::Short, "only from what the reserve holds");
    assert!(matches!(k.bond(&bond(Some([4, 0, 0, 0]), 1).sign(&reserve())), Founded::Bonded { bond: [4, 0, 0, 0], .. }));
    assert!(matches!(k.bond(&bond(Some([3, 0, 0, 0]), 2).sign(&reserve())), Founded::Bonded { bond: [7, 0, 0, 0], .. }), "topped up");
    assert_eq!((k.bond_of(&vk(&tier())), k.treasury.balance_mu(&vk(&reserve()))), ([7, 0, 0, 0], [3, 0, 0, 0]));
    let (f, _) = lying_fold();
    let Uplinked::Admitted { at, .. } = k.admit_uplink(&f.sign(&tier())) else { panic!("admitted") };
    assert!(k.fold_bonded(&at), "a fold of a bonded tier");
    let Uplinked::Admitted { at: bare, .. } = k.admit_uplink(&TierFold { prefix: [3; 32], rows: Vec::new(), tier: vk(&reserve()), ..lying_fold().0 }.sign(&reserve())) else {
        panic!("admitted")
    };
    assert!(!k.fold_bonded(&bare), "an unbacked fold reads unbonded");
    let mut booted = Kernel::new();
    assert!(booted.restore(&k.snapshot()), "restores");
    assert_eq!(booted.bond_of(&vk(&tier())), [7, 0, 0, 0], "the bond held across a restart");
}

/// A FALSIFIED FOLD SLASHES HALF THE BOND, AND NO BOND LEAVES WHILE A FOLD
/// IT BACKS IS OPEN: withdrawal waits on the tier's open fold; the replay
/// falsifies it — 7 bonded, 3 cut (the odd unit stays): 1 to the challenger,
/// 2 retired, 4 still bonded; then, nothing outstanding, the bond withdraws
/// to the reserve.
#[test]
fn a_falsified_fold_slashes_half_the_bond_and_withdrawal_waits_on_finality() {
    let mut k = founded();
    assert!(matches!(k.bond(&bond(Some([7, 0, 0, 0]), 1).sign(&reserve())), Founded::Bonded { .. }));
    let (f, line) = lying_fold();
    let Uplinked::Admitted { at, .. } = k.admit_uplink(&f.clone().sign(&tier())) else { panic!("admitted") };
    assert_eq!(k.bond(&bond(None, 2).sign(&reserve())), Founded::Outstanding, "a fold it backs is open");
    let supply_before = k.treasury.balance_mu(&vk(&reserve()))[0] + k.bond_of(&vk(&tier()))[0];
    let evidence = Challenge::Evidence { fold: at, rows: f.rows.clone(), declared: Vec::new(), children: Vec::new(), entries: line };
    assert!(matches!(k.challenge(&evidence.sign(&challenger())), Challenged::Falsified { .. }));
    assert_eq!(k.bond_of(&vk(&tier())), [4, 0, 0, 0], "half slashed, the odd unit bonded");
    assert_eq!(k.treasury.balance_mu(&vk(&challenger())), [1, 0, 0, 0], "the bounty");
    assert_eq!(k.treasury.balance_mu(&vk(&reserve()))[0] + k.bond_of(&vk(&tier()))[0], supply_before - 3, "3 left the reserve: 1 paid, 2 retired");
    assert!(matches!(k.bond(&bond(None, 2).sign(&reserve())), Founded::Bonded { bond: [0, 0, 0, 0], .. }), "nothing outstanding: withdrawn");
    assert_eq!((k.bond_of(&vk(&tier())), k.treasury.balance_mu(&vk(&reserve()))), ([0; 4], [7, 0, 0, 0]));
    assert_eq!(k.bond(&bond(None, 3).sign(&reserve())), Founded::Unbonded);
}
