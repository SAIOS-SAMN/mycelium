// E1 — the ledger holds: the supply is the sum of the minted deflations,
// a stake moves value and never creates it, and the audit survives genesis,
// mint and burn.

use crate::economy::*;

/// Supply after N installs equals the sum of their deflations per lane.
#[test]
fn supply_is_the_sum_of_minted_deflations() {
    let mut t = Treasury::new();
    t.enroll([1; 32], UNIT_BOND, (0, [0; 32]));
    let d1 = [4u64, 2, 0, 2];
    let d2 = [0u64, 4, 6, 2];
    t.escrow_award([8; 32], vec![[1; 32], [2; 32]], d1, (1, [1; 32]));
    t.escrow_award([9; 32], vec![[1; 32], [2; 32]], d2, (2, [2; 32]));
    let h1 = t.release_award(&[8; 32]).expect("d1");
    t.mint_claim(&h1.voices, h1.mu, h1.anchor);
    let h2 = t.release_award(&[9; 32]).expect("d2");
    t.mint_claim(&h2.voices, h2.mu, h2.anchor);
    let expect: [u64; 4] = std::array::from_fn(|l| d1[l] + d2[l]);
    assert_eq!(t.supply().expect("supply adds up"), expect);
}

/// A slash debits the bond and mints nothing.
#[test]
fn slash_debits_the_bond_and_mints_nothing() {
    let mut t = Treasury::new();
    let bond = t.enroll([1; 32], [8, 8, 8, 8], (0, [0; 32]));
    let addr = [3u8; 32];
    t.escrow_award(addr, vec![[1; 32]], [8, 0, 0, 0], (1, [1; 32]));
    assert!(t.revoke_award(&addr, true).is_some());
    assert_eq!(t.bond_of(&[1; 32]).unwrap()[0], bond[0].saturating_sub(8));
    assert_eq!(t.supply().expect("supply adds up"), [0; 4]);
    assert_eq!(t.balance_mu(&[1; 32]), [0; 4]);
}

/// Pool + stakes survive the v3 snapshot exactly.
#[test]
fn escrow_snapshot_round_trips() {
    let mut t = Treasury::new();
    t.enroll([3; 32], UNIT_BOND, (0, [4; 32]));
    t.settle_subsidy([3; 32], [0, 0, 5, 0], (1, [4; 32]));
    t.settle_recall([3; 32], &[0, 0, 3, 0], (2, [4; 32]));
    let snap = t.snapshot();
    let mut pos = 0;
    let back = Treasury::restore(&snap, &mut pos).expect("restore v3");
    assert_eq!(back.bond_total(), t.bond_total(), "live bonds survive, per lane");
    assert_eq!(back.stakes(), t.stakes(), "stakes survive");
    assert_eq!(back.entries(), t.entries());
}

/// §6, both equations, on a treasury that has actually moved value.
/// The fold of LEDGER_EPOCH_DESIGN is admissible only where this holds
/// at the prefix it folds, so this is the gate on 7.5.
#[test]
fn the_audit_holds_through_genesis_mint_and_burn() {
    let mut t = Treasury::new();
    let a = (1u64, [1u8; 32]);
    t.genesis([1; 32], [10, 10, 10, 10], a);
    assert_eq!(t.audit().expect("genesis conserves"), [10, 10, 10, 10]);
    t.settle_subsidy([2; 32], [4, 0, 2, 0], a);
    assert_eq!(t.audit().expect("a mint conserves"), [14, 10, 12, 10]);
    let burned = t.burn([1; 32], [3, 0, 0, 0], a);
    assert_eq!(burned, [3, 0, 0, 0], "burned from what was held");
    assert_eq!(t.audit().expect("a burn conserves"), [11, 10, 12, 10]);
    assert_eq!(t.supply().expect("supply adds up"), [11, 10, 12, 10]);
}

/// R9, and the distinction the audit turns on. A FUNDED escrow holds
/// μ⃗ that left a balance and is still in the mesh, so it is counted;
/// discharging it MOVES value and mints nothing. A CLAIM escrow holds
/// μ⃗ that does not exist yet (§3.1), so it is not counted, and a ruling
/// MINTS it. Removing escrow from the equation outright would make the
/// audit hold by not looking; counting all of it asserts value that was
/// never created. Both readings conserve here, and they conserve
/// differently — which is the whole point.
#[test]
fn a_funded_escrow_is_counted_and_a_claim_escrow_is_not() {
    let mut t = Treasury::new();
    let a = (1u64, [1u8; 32]);
    t.genesis([1; 32], [10, 0, 0, 0], a);
    let seeded = t.audit().expect("genesis conserves");

    // A CLAIM: nobody paid for it, so the carriers do not move and
    // supply does not move.
    t.escrow_award([7u8; 32], vec![[2; 32]], [4, 0, 0, 0], a);
    assert_eq!(t.audit().expect("a claim is not supply"), seeded);
    assert_eq!(t.balance_mu(&[2; 32]), [0; 4], "no one was debited for it");

    // A FUND: the payer is debited, so the μ⃗ moved out of a balance and
    // must still be found — in the escrow.
    let paid = t.escrow_from_balance([8u8; 32], [1; 32], [6, 0, 0, 0], a);
    assert_eq!(paid, [6, 0, 0, 0]);
    assert_eq!(t.balance_mu(&[1; 32]), [4, 0, 0, 0], "the payer paid");
    assert_eq!(
        t.audit().expect("a funded escrow is counted, not ignored"),
        seeded,
        "supply unmoved: value moved between carriers"
    );

    // Discharging it moves value back and mints nothing.
    assert!(t.revoke_award(&[8u8; 32], false).is_some(), "the award was there");
    assert_eq!(t.balance_mu(&[1; 32]), [10, 0, 0, 0], "returned whence it came");
    assert_eq!(t.audit().expect("a discharge conserves"), seeded);
    assert_eq!(t.minted().expect("minted"), [0; 4], "and nothing was earned");

    // The claim, released, MINTS — supply rises by what the ruling
    // measured, which is E1's whole mechanism.
    let held = t.release_award(&[7u8; 32]).expect("the claim stood");
    t.mint_claim(&held.voices, held.mu, held.anchor);
    let after = t.audit().expect("a ruling conserves");
    assert!(
        after.iter().zip(seeded.iter()).any(|(x, y)| x > y),
        "a claim released is issuance: {seeded:?} then {after:?}"
    );
}

/// A stake MOVES value between two carriers of one identity and never
/// creates any (§4: bond is μ at risk, staked from a balance). This
/// assertion is the inverse of the one it replaces, which pinned the
/// failure: `enroll` used to insert the bond and write its row without
/// debiting anything, so bond existed that was never issued.
#[test]
fn a_stake_moves_value_and_never_creates_it() {
    let mut t = Treasury::new();
    let a = (1u64, [1u8; 32]);
    t.genesis([1; 32], [10, 0, 0, 0], a);
    assert_eq!(t.audit().expect("genesis conserves"), [10, 0, 0, 0]);

    let staked = t.enroll([1; 32], [4, 0, 0, 0], a);
    assert_eq!(staked, [4, 0, 0, 0], "staked from the balance");
    assert_eq!(t.balance_mu(&[1; 32]), [6, 0, 0, 0], "the balance fell by it");
    assert_eq!(t.bond_of(&[1; 32]), Some([4, 0, 0, 0]));
    assert_eq!(t.audit().expect("a stake conserves"), [10, 0, 0, 0], "supply unmoved");

    // What the balance cannot cover is not staked.
    let over = t.enroll([1; 32], [99, 0, 0, 0], a);
    assert_eq!(over, [6, 0, 0, 0], "only what was held");
    assert_eq!(t.balance_mu(&[1; 32]), [0, 0, 0, 0]);
    assert_eq!(t.audit().expect("still conserves"), [10, 0, 0, 0]);

    // And it returns the way it went.
    let back = t.unstake([1; 32], [10, 0, 0, 0], a);
    assert_eq!(back, [10, 0, 0, 0], "never more than is bonded");
    assert_eq!(t.balance_mu(&[1; 32]), [10, 0, 0, 0]);
    assert_eq!(t.audit().expect("an unstake conserves"), [10, 0, 0, 0]);
}

/// §6 ON THE PATHS THAT SPEND. A recall fee, a door fee and a slash each
/// take μ⃗ out of a carrier, and each did it without taking the same μ⃗ out
/// of the supply: `Σ balance + Σ bond + Σ funded` fell while
/// `genesis + mint − burn` held, and the audit failed on every lane anyone
/// had ever read, queued or been slashed on. §2's carriers are balance,
/// bond and escrow — there is no pool among them — so what these take
/// leaves circulation.
#[test]
fn a_fee_and_a_slash_leave_the_supply_with_the_carrier() {
    let mut t = Treasury::new();
    t.genesis([1; 32], [20, 20, 20, 20], (0, [0; 32]));
    t.enroll([1; 32], [6, 6, 6, 6], (1, [0; 32]));
    let issued = t.audit().expect("a funded, bonded voice conserves");

    // A READING FEE is WHAT THE READING EXERCISED, per lane (§5) — not one
    // unit per lane touched, which was a monetary constant §8 forbids.
    let (fee, short) = t.settle_recall([1; 32], &[4, 0, 2, 0], (2, [1; 32]));
    assert_eq!(fee, [4, 0, 2, 0], "the structure moved on each lane, not a unit");
    assert_eq!(short, [0; 4], "a balance of 20 covers it on every lane");
    let after = t.audit().expect("the ledger conserves through a recall fee");
    assert_eq!(
        after,
        [issued[0] - 4, issued[1], issued[2] - 2, issued[3]],
        "and the supply fell by exactly the fee"
    );

    // A DOOR FEE.
    let (paid, short) = t.settle_door([1; 32], [2, 0, 0, 3], (3, [1; 32]));
    assert_eq!((paid, short), ([2, 0, 0, 3], [0; 4]));
    let after2 = t.audit().expect("the ledger conserves through a door fee");
    assert_eq!(after2, [after[0] - 2, after[1], after[2], after[3] - 3]);

    // A SLASH takes bond, and the entry names what LEFT the bond rather
    // than what the escrow asked for: a bond of 6 slashed for 9 loses 6.
    t.escrow_award([7u8; 32], vec![[1; 32]], [9, 0, 0, 0], (4, [1; 32]));
    // The slash reads what left each named voice's bond, and the lanes on
    // which the bond did not cover the ask, rather than clamping to it.
    let r = t.revoke_award(&[7u8; 32], true).expect("the escrow was there");
    assert_eq!(r.slashed, vec![([1u8; 32], [6, 0, 0, 0])], "the whole bond, and no more");
    assert_eq!(r.short, vec![([1u8; 32], [3, 0, 0, 0])], "and the three it could not cover");
    assert_eq!(t.bond_of(&[1; 32]).expect("still enrolled")[0], 0, "the whole bond went");
    let after3 = t.audit().expect("the ledger conserves through a slash");
    assert_eq!(after3[0], after2[0] - 6, "supply fell by the bond that left, not by the ask");

    // WHAT A BALANCE CANNOT COVER IS READ, NOT CLAMPED. `min(fee, held)`
    // returned a value and the caller then guessed the deferral from a zero
    // test; the shortfall says on which lanes, and by how much.
    let held = t.balance_mu(&[1; 32]);
    let (paid, short) = t.settle_door([1; 32], [held[0] + 5, 0, 0, 0], (5, [1; 32]));
    assert_eq!(paid[0], held[0], "everything the balance held went");
    assert_eq!(short[0], 5, "and the five it did not is the reading");
    assert_eq!(crate::economy::lanes_short(&short), vec![0], "named by lane");
    assert!(t.audit().is_ok(), "and it conserves through a partial payment");

    // And nothing was minted by any of it: the earned supply never moved.
    assert_eq!(t.minted().expect("minted adds up"), [0; 4]);
}

/// §4: "a word that fails replay debits the voice that spoke it" — every
/// voice the escrow names, as `mint_claim` credits every one. It slashed
/// `voices.first()`, one voice chosen by arrival order, which is rule 15's
/// "a tie-break by index, by size, by first found presented as a decision":
/// a word two voices stood cost the second of them nothing at all.
#[test]
fn a_slash_debits_every_voice_the_escrow_names() {
    let (a, b) = ([0xa1u8; 32], [0xb2u8; 32]);
    let mut t = Treasury::new();
    t.genesis(a, [10, 10, 10, 10], (0, [0; 32]));
    t.genesis(b, [10, 10, 10, 10], (0, [0; 32]));
    t.enroll(a, [5, 5, 5, 5], (1, [0; 32]));
    t.enroll(b, [2, 2, 2, 2], (1, [0; 32]));
    let before = t.audit().expect("two bonded voices conserve");

    t.escrow_award([9u8; 32], vec![a, b], [4, 0, 0, 0], (2, [1; 32]));
    let r = t.revoke_award(&[9u8; 32], true).expect("the escrow was there");

    assert_eq!(
        r.slashed,
        vec![(a, [4, 0, 0, 0]), (b, [2, 0, 0, 0])],
        "both voices are debited, each by what its own bond covered"
    );
    assert_eq!(
        r.short,
        vec![(b, [2, 0, 0, 0])],
        "and the lanes one of them could not cover are named, not clamped away"
    );
    assert_eq!(t.bond_of(&a).expect("a is enrolled")[0], 1, "a had five, four went");
    assert_eq!(t.bond_of(&b).expect("b is enrolled")[0], 0, "b had two, both went");

    // Six left the bonds, so six left the supply: §6 holds through it.
    let after = t.audit().expect("the ledger conserves through a two-voice slash");
    assert_eq!(after[0], before[0] - 6, "supply fell by what left the bonds");

    // A plain revoke drops the claim and slashes nobody.
    t.escrow_award([8u8; 32], vec![a], [1, 0, 0, 0], (3, [1; 32]));
    let r = t.revoke_award(&[8u8; 32], false).expect("the escrow was there");
    assert!(r.slashed.is_empty() && r.short.is_empty(), "a revoke is not a slash");
    assert_eq!(r.released, [1, 0, 0, 0], "and it says what claim it dropped");

    // An address holding no escrow is the third state, which the old bool
    // could not tell from "slashed nothing".
    assert!(t.revoke_award(&[7u8; 32], true).is_none(), "no escrow, nothing revoked");
}
