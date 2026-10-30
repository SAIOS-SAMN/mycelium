// POWV — PROOF OF WORK VERIFICATION (MU-ECONOMY §5.1): the courts whose
// replay verified a closure take their share of the release, per lane, read
// off the verifying court's strain against its reserve; the closer the
// rest; total issuance exactly the release.

use crate::cognition::*;
use crate::transducer::address::Address;

/// A court with a claim of `held` escrowed for `voice` at `addr`.
fn court_with_claim(addr: &Address, voice: [u8; 32], held: [u64; 4]) -> Kernel {
    let mut k = Kernel::new();
    let anchor = k.anchor();
    k.treasury.escrow_award(addr, vec![voice], held, anchor);
    k
}

/// A COURT AT REST TAKES NOTHING: with its reserve full, a release pays the
/// closer the whole of it and no verification row is written.
#[test]
fn a_court_at_rest_pays_the_closer_the_whole_release() {
    let (addr, voice, witness) = (Address::from_bytes([51u8; 32]), [5u8; 32], [6u8; 32]);
    let held = [40, 20, 0, 8];
    let mut k = court_with_claim(&addr, voice, held);
    let strain = crate::economy::yield_surface(&k.physics).expect("the surface reads");
    assert_eq!(strain, [0; 4], "a fresh court is at rest");
    let at = k.anchor();
    let organs = [k.identity, witness];
    let paid = k.release_verified(addr.as_bytes(), held, &organs, at).expect("the claim releases");
    assert_eq!(paid, held, "the closer is paid the whole release");
    assert_eq!(k.treasury().balance_mu(&voice), held);
    assert_eq!(k.treasury().balance_mu(&witness), [0; 4], "the witness took nothing");
    let economy = k.treasury().economy(&k.identity).expect("the economy reads");
    assert_eq!(economy.paid[crate::economy::Arm::Powv.index()], [0; 4], "no POWV row");
}

/// A STRAINED COURT AND ITS WITNESS TAKE THEIR SHARE, AND ISSUANCE IS THE
/// RELEASE: with the reserve spent, each verifying court is minted its
/// share per lane as POWV, the closer the rest as POUW, the two together
/// exactly the base the claim released, and the ledger's invariant holds.
#[test]
fn a_strained_court_and_its_witness_are_paid_out_of_the_release() {
    let (addr, voice, witness) = (Address::from_bytes([52u8; 32]), [7u8; 32], [8u8; 32]);
    let held = [40, 20, 0, 8];
    let mut k = court_with_claim(&addr, voice, held);
    k.physics.reserve = crate::physics::Reserve::of([0; 4]);
    let strain = crate::economy::yield_surface(&k.physics).expect("the surface reads");
    assert!(strain.iter().all(|&y| y > 0), "a spent reserve strains every lane: {strain:?}");
    let each = crate::economy::verification_share(&held, &strain, crate::physics::reserve_max(), 2);
    assert!(each.iter().any(|&m| m > 0), "the strain pays verification something");

    let at = k.anchor();
    let organs = [k.identity, witness];
    let paid = k.release_verified(addr.as_bytes(), held, &organs, at).expect("the claim releases");
    let me = k.identity;
    assert_eq!(k.treasury().balance_mu(&me), each, "this court is minted its share");
    assert_eq!(k.treasury().balance_mu(&witness), each, "the witness is minted the same share");
    let rest: [u64; 4] = std::array::from_fn(|l| held[l] - 2 * each[l]);
    assert_eq!(paid, rest, "the closer is paid the rest");
    assert_eq!(k.treasury().balance_mu(&voice), rest);
    assert_eq!(k.treasury().minted(), Some(held), "issuance is exactly the release");
    assert!(k.treasury().audit().is_ok(), "the supply invariant holds");
    let economy = k.treasury().economy(&me).expect("the economy reads");
    assert_eq!(
        economy.paid[crate::economy::Arm::Powv.index()],
        std::array::from_fn::<u64, 4, _>(|l| 2 * each[l]),
        "the reading carries both shares under POWV"
    );
    assert_eq!(economy.paid[crate::economy::Arm::Pouw.index()], rest, "and the closer's under POUW");
}

/// THE RELEASE IS CAPPED BEFORE IT IS SHARED, AND ONE COURT IS ONE ORGAN:
/// a discharge below the escrow is the base the share is read on, and a
/// witness that is this court itself takes one share, not two.
#[test]
fn the_share_is_read_on_the_capped_release_and_one_court_is_one_organ() {
    let (addr, voice) = (Address::from_bytes([53u8; 32]), [9u8; 32]);
    let held = [40, 20, 0, 8];
    let discharge = [10, 30, 0, 8];
    let mut k = court_with_claim(&addr, voice, held);
    k.physics.reserve = crate::physics::Reserve::of([0; 4]);
    let strain = crate::economy::yield_surface(&k.physics).expect("the surface reads");
    let base = [10, 20, 0, 8];
    let each = crate::economy::verification_share(&base, &strain, crate::physics::reserve_max(), 1);

    let at = k.anchor();
    let me = k.identity;
    let organs = [me, me];
    let paid = k.release_verified(addr.as_bytes(), discharge, &organs, at).expect("the claim releases");
    assert_eq!(k.treasury().balance_mu(&me), each, "one organ, one share");
    assert_eq!(paid, std::array::from_fn::<u64, 4, _>(|l| base[l] - each[l]), "the closer the rest of the capped base");
    assert_eq!(k.treasury().minted(), Some(base), "nothing above the discharge was issued");
    assert!(k.treasury().audit().is_ok());
}
