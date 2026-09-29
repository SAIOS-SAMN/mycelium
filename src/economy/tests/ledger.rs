// E1 — the ledger holds: the supply is the sum of the minted deflations,
// a stake moves value and never creates it, and the audit survives genesis,
// mint and burn.

use super::{at, word};
use crate::economy::*;

/// Supply after N installs equals the sum of their deflations per lane.
#[test]
fn supply_is_the_sum_of_minted_deflations() {
    let mut t = Treasury::new();
    t.enroll([1; 32], UNIT_BOND, at(0, [0; 32]));
    let d1 = [4u64, 2, 0, 2];
    let d2 = [0u64, 4, 6, 2];
    t.escrow_award(&word([8; 32]), vec![[1; 32], [2; 32]], d1, at(1, [1; 32]));
    t.escrow_award(&word([9; 32]), vec![[1; 32], [2; 32]], d2, at(2, [2; 32]));
    // Released at the acts they were escrowed at: they stood no height.
    let h1 = t.release_award(&[8; 32], at(1, [1; 32])).expect("d1");
    t.mint_claim(&h1.voices, h1.mu, h1.anchor);
    let h2 = t.release_award(&[9; 32], at(2, [2; 32])).expect("d2");
    t.mint_claim(&h2.voices, h2.mu, h2.anchor);
    let expect: [u64; 4] = std::array::from_fn(|l| d1[l] + d2[l]);
    assert_eq!(t.supply().expect("supply adds up"), expect);
}

/// A slash debits the bond and mints nothing.
///
/// The identity is seeded first, because a bond is μ⃗ STAKED FROM A BALANCE
/// (§4) and an identity with no balance enrols at a bond of zero. Asserting
/// against `bond[0].saturating_sub(8)` read that zero as a pass — the floor
/// agreed with an empty bond, and the slash this test names was never
/// exercised on anything.
#[test]
fn slash_debits_the_bond_and_mints_nothing() {
    let mut t = Treasury::new();
    t.genesis([1; 32], [8, 0, 0, 0], at(0, [0; 32]));
    let bond = t.enroll([1; 32], [8, 0, 0, 0], at(0, [0; 32]));
    assert_eq!(bond, [8, 0, 0, 0], "the balance covered the bond asked for");
    let addr = [3u8; 32];
    t.escrow_award(&word(addr), vec![[1; 32]], [8, 0, 0, 0], at(1, [1; 32]));
    assert!(t.revoke_award(&addr, true).is_some());
    assert_eq!(
        t.bond_of(&[1; 32]).expect("an enrolled identity holds a bond")[0],
        bond[0].checked_sub(8).expect("the bond covered the revoked award")
    );
    assert_eq!(t.supply().expect("supply adds up"), [0; 4]);
    assert_eq!(t.balance_mu(&[1; 32]), [0; 4]);
}

/// Pool + stakes survive the persisted form exactly (78.27: the stakes and
/// balances as their own tries, the rows through the row codec).
#[test]
fn escrow_snapshot_round_trips() {
    let mut t = Treasury::new();
    t.enroll([3; 32], UNIT_BOND, at(0, [4; 32]));
    t.settle_subsidy([3; 32], [0, 0, 5, 0], at(1, [4; 32]));
    t.settle_recall([3; 32], &[0, 0, 3, 0], &[0; 4], at(2, [4; 32]));
    let back = super::round_trip(&t).expect("the treasury round-trips");
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
    let a = at(1, [1; 32]);
    t.genesis([1; 32], [10, 10, 10, 10], a);
    assert_eq!(t.audit().expect("genesis conserves"), [10, 10, 10, 10]);
    t.settle_subsidy([2; 32], [4, 0, 2, 0], a);
    assert_eq!(t.audit().expect("a mint conserves"), [14, 10, 12, 10]);
    let burned = t.burn([1; 32], [3, 0, 0, 0], a);
    assert_eq!(burned, [3, 0, 0, 0], "burned from what was held");
    assert_eq!(t.audit().expect("a burn conserves"), [11, 10, 12, 10]);
    assert_eq!(t.supply().expect("supply adds up"), [11, 10, 12, 10]);
}

/// STD1 (rule 11) — MONEY NEVER TOPS OUT SILENTLY. A mint into a balance
/// with no room on a lane is refused before its row is written: nothing is
/// minted, the journal and the supply stand, the audit still holds, and the
/// refusal is counted on the economy reading.
#[test]
fn a_credit_past_its_carrier_refuses_the_mint_and_writes_no_row() {
    let mut t = Treasury::new();
    let a = at(1, [1; 32]);
    t.genesis([1; 32], [u64::MAX, 0, 0, 0], a);
    let rows = t.entry_count();
    assert_eq!(t.settle_subsidy([1; 32], [1, 1, 0, 0], a), [0; 4], "a lane with no room refuses the whole mint");
    assert_eq!(t.entry_count(), rows, "and writes no row");
    assert_eq!(t.balance_mu(&[1; 32]), [u64::MAX, 0, 0, 0], "the balance stands on every lane");
    assert_eq!(t.audit().expect("the ledger still conserves"), [u64::MAX, 0, 0, 0]);
    assert_eq!(t.economy().expect("the economy reads").overflowed, 1, "counted, by name");
    assert_eq!(t.settle_subsidy([2; 32], [1, 1, 0, 0], a), [1, 1, 0, 0], "another balance with room is paid");
}

/// §4, §6, Stage 12 S12.5a (I38) — A STAKE OUTSIDE THE LIVE GENESIS IS
/// BURNED, and the audit holds through it: the bond leaves the carriers and
/// the supply by one vector as one `Burn` row; the identity's balance stays;
/// the live bond stands; nothing is written that moves no supply.
#[test]
fn a_stake_outside_the_live_genesis_is_burned_and_the_audit_holds() {
    let mut t = Treasury::new();
    let a = at(1, [1; 32]);
    t.genesis([1; 32], [10, 10, 10, 10], a);
    t.enroll([1; 32], [6, 20, 0, 26], a);
    t.genesis([2; 32], [4, 4, 4, 4], a);
    t.enroll([2; 32], [4, 4, 4, 4], a);
    assert!(t.audit().is_ok(), "two bonded identities conserve");
    let burned = t.bind_live(&[[2; 32]], a);
    assert_eq!(burned, vec![([1; 32], [6, 10, 0, 10])], "the dropped stake is burned whole");
    assert_eq!(t.bond_of(&[1; 32]), None, "the identity is no longer bonded");
    assert_eq!(t.bond_of(&[2; 32]), Some([4, 4, 4, 4]), "the live bond stands");
    assert_eq!(t.balance_mu(&[1; 32]), [4, 0, 10, 0], "its balance stays; only the bond left");
    assert!(
        t.entries()
            .iter()
            .any(|e| e.kind == Kind::Burn && e.who == [1; 32] && e.mu == [6, 10, 0, 10]),
        "one burn row names what left"
    );
    assert_eq!(
        t.audit().expect("the audit holds through the burn"),
        [8, 4, 14, 4],
        "supply = genesis − burn, and the carriers agree"
    );
}

/// R9 on a claim. A CLAIM escrow holds μ⃗ that does not exist yet
/// (§3.1), so it is not counted and moves no carrier, and a ruling MINTS
/// it.
#[test]
fn a_claim_escrow_is_not_supply_until_a_ruling_mints_it() {
    let mut t = Treasury::new();
    let a = at(1, [1; 32]);
    t.genesis([1; 32], [10, 0, 0, 0], a);
    let seeded = t.audit().expect("genesis conserves");

    // A CLAIM: nobody paid for it, so the carriers do not move and
    // supply does not move.
    t.escrow_award(&word([7u8; 32]), vec![[2; 32]], [4, 0, 0, 0], a);
    assert_eq!(t.audit().expect("a claim is not supply"), seeded);
    assert_eq!(t.balance_mu(&[2; 32]), [0; 4], "no one was debited for it");

    // The claim, released, MINTS — supply rises by what the ruling
    // measured, which is E1's whole mechanism.
    let held = t.release_award(&[7u8; 32], a).expect("the claim stood");
    t.mint_claim(&held.voices, held.mu, held.anchor);
    let after = t.audit().expect("a ruling conserves");
    assert!(
        after.iter().zip(seeded.iter()).any(|(x, y)| x > y),
        "a claim released is issuance: {seeded:?} then {after:?}"
    );
}

/// THE TEMPORAL LANE OF A CLAIM IS THE TIME IT STOOD. A word's magnitude
/// reads 𝒯 off its chain, which has no clock, so a claim is escrowed at 0 on
/// 𝒯; released five heights later it carries those five heights on 𝒯, and
/// the other lanes as escrowed. A release stamped below its escrow is a
/// broken count and accrues nothing.
#[test]
fn a_claim_accrues_on_the_temporal_lane_the_heights_it_stood() {
    let mut t = Treasury::new();
    t.escrow_award(&word([7u8; 32]), vec![[2; 32]], [4, 1, 0, 3], at(1, [1; 32]));
    let held = t.release_award(&[7u8; 32], at(6, [6; 32])).expect("the claim stood");
    assert_eq!(held.mu, [4, 1, 5, 3], "five heights stood, on 𝒯 alone");
    t.escrow_award(&word([8u8; 32]), vec![[2; 32]], [4, 1, 0, 3], at(6, [6; 32]));
    let back = t.release_award(&[8u8; 32], at(1, [1; 32])).expect("the claim stood");
    assert_eq!(back.mu, [4, 1, 0, 3], "a release below its escrow accrues nothing");
}

/// A stake MOVES value between two carriers of one identity and never
/// creates any (§4: bond is μ at risk, staked from a balance). This
/// assertion is the inverse of the one it replaces, which pinned the
/// failure: `enroll` used to insert the bond and write its row without
/// debiting anything, so bond existed that was never issued.
#[test]
fn a_stake_moves_value_and_never_creates_it() {
    let mut t = Treasury::new();
    let a = at(1, [1; 32]);
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
/// of the supply: `Σ balance + Σ bond` fell while
/// `genesis + mint − burn` held, and the audit failed on every lane anyone
/// had ever read, queued or been slashed on. §2's carriers are balance,
/// bond and escrow — there is no pool among them — so what these take
/// leaves circulation.
#[test]
fn a_fee_and_a_slash_leave_the_supply_with_the_carrier() {
    let mut t = Treasury::new();
    t.genesis([1; 32], [20, 20, 20, 20], at(0, [0; 32]));
    t.enroll([1; 32], [6, 6, 6, 6], at(1, [0; 32]));
    let issued = t.audit().expect("a funded, bonded voice conserves");

    // Lanes by the names the tree gives them, never a bare `[2]` (§2.5).
    use crate::transducer::rewrite::{LANE_GAUGE, LANE_SHELL, LANE_SPATIAL, LANE_TEMPORAL};
    let less = |v: [u64; 4], lane: usize, by: u64| -> [u64; 4] {
        let mut out = v;
        out[lane] -= by;
        out
    };

    // A READING FEE is WHAT THE READING EXERCISED, per lane (§5) — not one
    // unit per lane touched, which was a monetary constant §8 forbids.
    let (fee, short) = t.settle_recall([1; 32], &[4, 0, 2, 0], &[0; 4], at(2, [1; 32]));
    assert_eq!(fee, [4, 0, 2, 0], "the structure moved on each lane, not a unit");
    assert_eq!(short, [0; 4], "a balance of 20 covers it on every lane");
    let after = t.audit().expect("the ledger conserves through a recall fee");
    assert_eq!(
        after,
        less(less(issued, LANE_SPATIAL, 4), LANE_TEMPORAL, 2),
        "and the supply fell by exactly the fee, on the lanes it was read on"
    );
    assert_eq!(after[LANE_GAUGE], issued[LANE_GAUGE], "a lane not read is untouched");

    // A DOOR FEE.
    let (paid, short) = t.settle_door([1; 32], [2, 0, 0, 3], at(3, [1; 32]));
    assert_eq!((paid, short), ([2, 0, 0, 3], [0; 4]));
    let after2 = t.audit().expect("the ledger conserves through a door fee");
    assert_eq!(after2, less(less(after, LANE_SPATIAL, 2), LANE_SHELL, 3));

    // A SLASH takes bond, and the entry names what LEFT the bond rather
    // than what the escrow asked for: a bond of 6 slashed for 9 loses 6.
    t.escrow_award(&word([7u8; 32]), vec![[1; 32]], [9, 0, 0, 0], at(4, [1; 32]));
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
    let (paid, short) = t.settle_door([1; 32], [held[0] + 5, 0, 0, 0], at(5, [1; 32]));
    assert_eq!(paid[0], held[0], "everything the balance held went");
    assert_eq!(short[0], 5, "and the five it did not is the reading");
    assert_eq!(crate::economy::lanes_short(&short), vec![0], "named by lane");
    assert!(t.audit().is_ok(), "and it conserves through a partial payment");

    // And nothing was minted by any of it: the earned supply never moved.
    assert_eq!(t.minted().expect("minted adds up"), [0; 4]);
}

/// THE SUPPLY IS FOLDED AS IT IS WRITTEN, not read whole on every call.
/// `supply()` iterated the whole journal, `audit()` calls `supply()`, and
/// §6 requires the audit before every snapshot — so the cost of proving
/// the ledger conserves grew with the ledger, on a carrier that grows for
/// as long as the mesh runs (rule 10). The fold must agree with the whole
/// read it replaces on every kind, and survive a restore.
#[test]
fn the_supply_folded_as_it_is_written_equals_the_journal_it_describes() {
    let (a, b) = ([0x11u8; 32], [0x22u8; 32]);
    let mut t = Treasury::new();
    assert_eq!(t.supply(), Some([0; 4]), "a fresh ledger has issued nothing, measured");

    // Every kind that moves the supply, and several that do not.
    t.genesis(a, [10, 10, 10, 10], at(0, [0; 32]));
    t.enroll(a, [4, 4, 4, 4], at(1, [0; 32])); // stake: moves no supply
    t.settle_subsidy(b, [0, 3, 0, 0], at(2, [0; 32])); // mint
    t.settle_use(a, b, [1, 0, 0, 0], at(3, [0; 32])); // two transfers
    t.settle_recall(a, &[2, 0, 0, 0], &[0; 4], at(4, [0; 32])); // burn
    t.escrow_award(&word([7u8; 32]), vec![a], [9, 0, 0, 0], at(5, [0; 32]));
    t.revoke_award(&[7u8; 32], true); // slash

    // The whole read this replaced, recomputed here from the rows themselves.
    let mut issued = [0u64; 4];
    let mut burned = [0u64; 4];
    for e in t.entries() {
        let into = match e.kind.issues() {
            Some(true) => &mut issued,
            Some(false) => &mut burned,
            None => continue,
        };
        for (x, d) in into.iter_mut().zip(&e.mu) {
            *x += d;
        }
    }
    let read_whole: [u64; 4] = std::array::from_fn(|l| issued[l] - burned[l]);
    assert_eq!(t.supply(), Some(read_whole), "the fold is the whole read, on every kind");
    assert!(t.audit().is_ok(), "and the carriers agree with it");

    // It comes back from disk. `restore` reads rows straight off the tape,
    // so the fold is rebuilt there in the one O(journal) pass.
    let back = super::round_trip(&t).expect("restores");
    assert_eq!(back.supply(), t.supply(), "the fold survives a restore");

    // AND SO DO THE CARRIERS. `recompute_balances` applied `mint` and
    // paired `transfer` and ignored genesis, burn, stake, unstake, escrow,
    // release, revoke and slash, so a restored ledger that had used any of
    // them failed its own audit -- which §6 requires before every snapshot,
    // so the mesh could not persist. Every voice's balance comes back, and
    // the audit holds against a supply re-derived independently of it.
    assert_eq!(back.balance_mu(&a), t.balance_mu(&a), "a's balance survives");
    assert_eq!(back.balance_mu(&b), t.balance_mu(&b), "and b's");
    assert_eq!(back.audit().ok(), t.audit().ok(), "and the audit holds through a restore");
    assert!(back.audit().is_ok(), "on a ledger that used every kind that moves value");
}

/// AN UNDEFINED ENTRY TAG REFUSES THE TAPE. It decoded as the kind
/// `"unknown"`, which no arm of the supply fold matched, so the row was
/// dropped in silence and the audit then compared the carriers against a
/// journal it had not read whole — rule 11, and A21 1's "the decoder is
/// total on well-formed keys and refuses otherwise". The typed `Kind`
/// leaves nothing for an undefined tag to decode into.
#[test]
fn an_undefined_entry_tag_refuses_the_tape() {
    let mut t = Treasury::new();
    t.genesis([0x33; 32], [1, 0, 0, 0], at(0, [0; 32]));
    let [row] = t.entries() else { panic!("one row") };
    let mut tape = Vec::new();
    row.write(&mut tape);
    // The row's tag is its first byte.
    assert_eq!(tape[0], 16, "the row written is a genesis");
    tape[0] = tape[0].wrapping_add(0x80); // a tag the layout defines no kind for
    let mut pos = 0;
    assert!(Entry::read(&tape, &mut pos).is_none(), "the row is refused, not read past the tag it cannot name");
    // And an arm byte the layout does not define refuses the same way.
    let mut tape = Vec::new();
    row.write(&mut tape);
    tape[1] = 9;
    assert!(Entry::read(&tape, &mut 0).is_none(), "an arm byte that names no proof of work refuses");
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
    t.genesis(a, [10, 10, 10, 10], at(0, [0; 32]));
    t.genesis(b, [10, 10, 10, 10], at(0, [0; 32]));
    t.enroll(a, [5, 5, 5, 5], at(1, [0; 32]));
    t.enroll(b, [2, 2, 2, 2], at(1, [0; 32]));
    let before = t.audit().expect("two bonded voices conserve");

    t.escrow_award(&word([9u8; 32]), vec![a, b], [4, 0, 0, 0], at(2, [1; 32]));
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
    t.escrow_award(&word([8u8; 32]), vec![a], [1, 0, 0, 0], at(3, [1; 32]));
    let r = t.revoke_award(&[8u8; 32], false).expect("the escrow was there");
    assert!(r.slashed.is_empty() && r.short.is_empty(), "a revoke is not a slash");
    assert_eq!(r.released, [1, 0, 0, 0], "and it says what claim it dropped");

    // An address holding no escrow is the third state, which the old bool
    // could not tell from "slashed nothing".
    assert!(t.revoke_award(&[7u8; 32], true).is_none(), "no escrow, nothing revoked");
}
