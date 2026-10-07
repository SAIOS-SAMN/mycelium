// 78.119 — A TIER'S FOLD ADMITTED: the court, outside the tier's closure,
// checks the tier's signature and the fold's continuity, and seals the fold's
// coordinates on its line — a receipt that restores with the line.

use crate::anastome::dialogue::uplink::{Discontinuous, TierFold, Uplinked};
use crate::chronometry::{ActKind, Cite};
use crate::server::cognition::Kernel;
use ed25519_dalek::SigningKey;

fn tier_key() -> SigningKey {
    SigningKey::from_bytes(&[9u8; 32])
}

fn who(b: u8) -> [u8; 32] {
    [b; 32]
}

fn domain_key() -> SigningKey {
    SigningKey::from_bytes(&[4u8; 32])
}

/// 78.120 — a court on which the tier's domain is founded, naming the tier.
fn founded() -> Kernel {
    use crate::anastome::dialogue::domain::{Founded, DomainGenesis};
    let mut k = Kernel::new();
    let g = DomainGenesis { domain: domain_key().verifying_key().to_bytes(), tiers: vec![tier_key().verifying_key().to_bytes()] };
    assert!(matches!(k.admit_domain(&g.sign(&domain_key())), Founded::Sealed { .. }), "the domain founded");
    k
}

/// A trade between two identities, nothing made: `A` +5, `B` −5 on `S`.
fn trade(key: &SigningKey) -> TierFold {
    TierFold {
        tier: key.verifying_key().to_bytes(),
        prefix: [7u8; 32],
        rows: vec![(who(1), [5, 0, 0, 0]), (who(2), [-5, 0, 0, 0])],
        declared: Vec::new(),
    }
}

/// AN ADMITTED FOLD IS SEALED BY ITS COORDINATES AND RESTORES WITH THE LINE:
/// the receipt's kind is the uplink, its one citation the tier, the prefix
/// head and the rows' commitment; the answer is the receipt's id; and the
/// line written with it restores into a fresh court, the head the same.
#[test]
fn an_admitted_fold_is_sealed_by_its_coordinates_and_restores() {
    let key = tier_key();
    let fold = trade(&key);
    let rows = fold.rows_hash();
    let mut k = founded();
    let Uplinked::Admitted { at, receipt } = k.admit_uplink(&fold.clone().sign(&key)) else { panic!("admitted") };
    let head = k.chain.head().expect("sealed on the line");
    assert_eq!((head.stamp, head.chain_hash), (at, receipt), "the answer is the receipt's id");
    assert_eq!(head.kind, ActKind::Uplink);
    assert_eq!(head.cites, vec![Cite::Uplink { tier: fold.tier, prefix: fold.prefix, rows }]);
    let snap = k.snapshot();
    let mut booted = Kernel::new();
    assert!(booted.restore(&snap), "the line with an uplink restores");
    assert_eq!(booted.chain.head().map(|h| h.chain_hash), Some(receipt), "the same head");
    // 78.120 — and the domain restores with it: its tier's next fold admitted.
    let next = TierFold { prefix: [8u8; 32], ..trade(&key) };
    assert!(matches!(booted.admit_uplink(&next.sign(&key)), Uplinked::Admitted { .. }), "the domain held across the restart");
}

/// A FOLD THE TIER DID NOT SIGN IS REFUSED: a row changed after signing, or
/// a fold signed by another key in the tier's name — nothing sealed.
#[test]
fn a_fold_the_tier_did_not_sign_is_refused() {
    let key = tier_key();
    let mut signed = trade(&key).sign(&key);
    signed.fold.rows[0].1[0] = 6;
    let mut k = founded();
    assert_eq!(k.admit_uplink(&signed), Uplinked::Unsigned, "a row changed after signing");
    let other = SigningKey::from_bytes(&[3u8; 32]);
    let forged = trade(&key).sign(&other);
    assert_eq!(k.admit_uplink(&forged), Uplinked::Unsigned, "another key in the tier's name");
    assert_eq!(k.chain.head().map(|h| h.kind), Some(ActKind::Domain), "nothing sealed after the genesis");
}

/// 78.120 — A TIER NO DOMAIN NAMES IS UNBOUND: its fold, signed and
/// continuous, is refused until a domain's genesis names it.
#[test]
fn a_fold_from_a_tier_no_domain_names_is_unbound() {
    let key = tier_key();
    let mut k = Kernel::new();
    assert_eq!(k.admit_uplink(&trade(&key).sign(&key)), Uplinked::Unbound);
    assert!(k.chain.head().is_none(), "nothing sealed");
}

/// 78.120 — A DOMAIN IS FOUNDED BY ITS OWN KEY, ONCE, ITS TIERS ITS OWN: the
/// genesis sealed by its keys; another key's signature, unordered tiers, a
/// second founding and a tier another domain names are refused by name.
#[test]
fn a_domain_is_founded_once_by_its_own_key_its_tiers_its_own() {
    use crate::anastome::dialogue::domain::{Founded, DomainGenesis};
    let k = founded();
    let head = k.chain.head().expect("the genesis on the line");
    let (domain, tier) = (domain_key().verifying_key().to_bytes(), tier_key().verifying_key().to_bytes());
    assert_eq!(head.cites, vec![Cite::Domain { domain, tiers: vec![tier] }]);
    let mut k = k;
    let again = DomainGenesis { domain, tiers: vec![tier] };
    assert_eq!(k.admit_domain(&again.clone().sign(&domain_key())), Founded::AlreadyFounded);
    let impostor = SigningKey::from_bytes(&[6u8; 32]);
    assert_eq!(k.admit_domain(&again.sign(&impostor)), Founded::Unsigned, "signed by another key");
    let other_domain = SigningKey::from_bytes(&[5u8; 32]);
    let claims = DomainGenesis { domain: other_domain.verifying_key().to_bytes(), tiers: vec![tier] };
    assert_eq!(k.admit_domain(&claims.sign(&other_domain)), Founded::TierHeld, "a tier settles for one domain");
    let unordered = DomainGenesis { domain: other_domain.verifying_key().to_bytes(), tiers: vec![who(9), who(2)] };
    assert_eq!(k.admit_domain(&unordered.sign(&other_domain)), Founded::Unordered);
}

/// CONTINUITY: value made inside the fold is refused unless declared — a
/// source declaring it admits the same rows; an unordered list and a lane
/// passing its carrier are refused by name.
#[test]
fn a_fold_that_makes_value_undeclared_is_refused_and_a_declared_source_admits_it() {
    let key = tier_key();
    let mut made = trade(&key);
    made.rows[1].1[0] = -4;
    let mut k = founded();
    assert_eq!(k.admit_uplink(&made.clone().sign(&key)), Uplinked::Discontinuous(Discontinuous::Unbalanced { lane: 0 }));
    made.declared = vec![(who(9), [1, 0, 0, 0])];
    assert!(matches!(k.admit_uplink(&made.sign(&key)), Uplinked::Admitted { .. }), "a declared source accounts for it");
    let mut unordered = trade(&key);
    unordered.rows.reverse();
    assert_eq!(unordered.continuity(), Err(Discontinuous::Unordered));
    let mut past = trade(&key);
    past.rows = vec![(who(1), [i128::MAX, 0, 0, 0]), (who(2), [1, 0, 0, 0])];
    assert_eq!(past.continuity(), Err(Discontinuous::Overflow { lane: 0 }));
}
