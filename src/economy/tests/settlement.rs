// 26, E2 — the market settles: novelty pays on use and not on existence,
// and every settlement is a vector of four lanes.

use super::at;
use crate::economy::*;
use crate::transducer::element::{from_canonical, Element};
use crate::transducer::germinate::germinate;

fn germ() -> Element {
    // A germinated pattern (post-excision, the only species of germ).
    from_canonical(&germinate(&[(0, 0, 1u8), (0, 1, 2)], &[(0, 0, 1u8), (0, 1, 9)]).unwrap())
}

#[test]
fn novelty_pays_on_use_not_on_existence() {
    let mut t = Treasury::new();
    t.enroll([1; 32], UNIT_BOND, at(0, [0; 32])); // genesis: the bond must exist
    let g = germ();
    let anchor: Anchor = at(3, [7; 32]);
    assert!(magnitude(&g).iter().any(|&m| m >= 1), "a germinated pattern has cost on the lanes it moves");
    assert!(t.settle_install(&g, [1; 32]), "gate admits a reducing germ");
    assert_eq!(t.balance_mu(&[1; 32]), [0; 4], "installing pays NOTHING — no corpus proven yet");
    // CIRCULATION: a funded demander pays the discoverer on each real use.
    t.settle_subsidy([2; 32], [0, 0, 100, 0], anchor); // the demander earned earlier
    t.settle_use([2; 32], [1; 32], magnitude(&g), anchor); // a real demand decomposed through it
    t.settle_use([2; 32], [1; 32], magnitude(&g), anchor);
    let bal = t.balance_mu(&[1; 32]);
    assert_eq!(bal[2], 2 * magnitude(&g)[2], "reward flows with real use, per lane");
    assert_eq!(bal[0], 0, "no lane is invented");
    assert_eq!(t.balance_mu(&[2; 32])[2], 100 - 2 * magnitude(&g)[2], "the demander PAID — a transfer, not a mint");
    // Every YIELD entry carries its chronometric anchor — the unforgeable
    // binding (the genesis stake carries its own enrollment anchor).
    assert!(t.entries().iter().filter(|e| e.kind != Kind::Stake).all(|e| (e.at, e.receipt) == at(3, [7; 32])));
}

/// The dimensional ledger: consilience splits the PRICED μ⃗ per lane (nonzero
/// lanes never split to zero), and the torse coin books on the holonomic lane.
#[test]
fn consilience_and_torse_settle_as_vectors() {
    let mut t = Treasury::new();
    t.enroll([1; 32], UNIT_BOND, at(0, [0; 32])); // capitalize before settling
    let anchor: Anchor = at(9, [1; 32]);
    t.settle_consilience(&[[1; 32], [2; 32]], [0, 0, 5, 0], anchor);
    assert_eq!(t.balance_mu(&[1; 32]), [0, 0, 2, 0]);
    assert_eq!(t.balance_mu(&[2; 32]), [0, 0, 2, 0]);
    t.settle_torse([1; 32], [0, 0, 40, 0], anchor);
    assert_eq!(
        t.balance_mu(&[1; 32])[2],
        42,
        "the torse coin mints the measured vector"
    );
    // The vector ledger round-trips with anchors intact.
    let back = super::round_trip(&t).expect("restore");
    assert_eq!(back.entries(), t.entries(), "μ⃗ entries + anchors survive, exactly");
}

/// A release with an empty stake bond still pays the measured vector.
#[test]
fn release_mints_when_the_bond_is_empty() {
    let mut t = Treasury::new();
    let addr = [7u8; 32];
    t.escrow_award(&super::word(addr), vec![[1; 32], [2; 32]], [4, 0, 2, 0], at(1, [9; 32]));
    assert_eq!(t.bond_total(), Some([0; 4]));
    let held = t.release_award(&addr).expect("claim");
    t.mint_claim(&held.voices, held.mu, held.anchor);
    assert_eq!(t.balance_mu(&[1; 32]), [2, 0, 1, 0]);
    assert_eq!(t.balance_mu(&[2; 32]), [2, 0, 1, 0]);
    assert_eq!(t.supply().expect("supply adds up"), [4, 0, 2, 0]);
    assert_eq!(t.bond_total(), Some([0; 4]), "mint does not touch the stake bond");
}

/// S9.3 — THE READING, PER LANE, AND THE AMOUNT INSIDE IT. Each lane is
/// `Fell(d)`, `Held` or `Rose(d)`: a lane that fell by three and a lane
/// that fell by one are different readings, and a lane that held is a third
/// thing rather than a zero. Nothing here decides anything.
#[test]
fn a_lane_move_is_read_per_lane() {
    use crate::economy::LaneMove::{Fell, Held, Rose};
    let fell = lane_moves(&[0, 0, 0, 0], &[1, 0, 0, 0]);
    assert_eq!(fell, [Fell(1), Held, Held, Held], "the fall is on its lane, by its amount");
    assert_eq!(lanes_fallen(&fell), vec![0]);
    assert!(lanes_risen(&fell).is_empty(), "and nothing rose");
    assert_eq!(deflation_of(&fell), [1, 0, 0, 0], "what fell is what is paid");

    // A trade: one lane down, another up. A bool said only "no".
    let traded = lane_moves(&[1, 0, 0, 0], &[0, 1, 0, 0]);
    assert_eq!(traded, [Rose(1), Fell(1), Held, Held], "both moves are in the reading");
    assert_eq!(lanes_risen(&traded), vec![0], "the raise is named");
    assert_eq!(lanes_fallen(&traded), vec![1], "and so is the fall traded for it");
    assert_eq!(deflation_of(&traded), [0, 1, 0, 0], "a raised lane pays nothing");

    // Nothing moved: four held lanes, which is not "no trajectory" and not zero.
    let still = lane_moves(&[1, 1, 0, 0], &[1, 1, 0, 0]);
    assert_eq!(still, [Held; 4], "the mind did not reduce");
    assert!(lanes_risen(&still).is_empty() && lanes_fallen(&still).is_empty());
}

/// G4. An escrow names every voice that stood the claim — `escrow_award`
/// accumulates them, pushing each new one onto the entry — so a release
/// pays all of them, split per lane, as a consilience award is split.
/// Paying `voices.first()` paid one of several who did the work, chosen
/// by the order they arrived in.
#[test]
fn a_released_rewrite_pays_every_voice_the_escrow_names() {
    let mut t = Treasury::new();
    let addr = [3u8; 32];
    t.escrow_award(&super::word(addr), vec![[1; 32]], [8, 0, 4, 0], at(1, [1; 32]));
    t.escrow_award(&super::word(addr), vec![[2; 32]], [8, 0, 4, 0], at(1, [1; 32]));
    let released = at(5, [5; 32]);
    let share = t.release_rewrite(&addr, released).expect("two parties stood it");
    assert_eq!(share, [4, 0, 2, 0], "one quantity, split per lane, lanes never meeting");
    assert_eq!(t.balance_mu(&[1; 32]), share, "the voice that stood it");
    assert_eq!(t.balance_mu(&[2; 32]), share, "and the one that stood it after");
    let mints: Vec<&Entry> = t.entries().iter().filter(|e| e.kind == Kind::Mint).collect();
    assert!(
        mints.iter().all(|e| (e.at, e.receipt, e.arm) == (released.0, released.1, Some(Arm::Powr))),
        "each mint stands at the act that released it and pays POWR (78.33, EC1)"
    );
    assert!(t.release_rewrite(&addr, released).is_none(), "the escrow is spent once");
}

/// A mint with no payee is a mint to nobody: an escrow naming no voice
/// is not released and stands (rule 11 — the old fallback invented one).
#[test]
fn an_escrow_with_no_voice_is_not_released() {
    let mut t = Treasury::new();
    let addr = [4u8; 32];
    t.escrow_award(&super::word(addr), Vec::new(), [8, 0, 0, 0], at(1, [1; 32]));
    assert!(t.release_rewrite(&addr, at(5, [5; 32])).is_none(), "no party, no release");
    assert_eq!(t.escrowed_remaining(&addr), [8, 0, 0, 0], "the escrow stands");
}

/// Rule 11: a lane split cannot invent units. An award smaller than the number
/// of voices does not floor to 1 per voice.
#[test]
fn settlement_split_never_invents_units_below_voice_count() {
    let mut t = Treasury::new();
    let anchor: Anchor = at(1, [1; 32]);
    // 1 unit on lane 2 split across 4 voices: integer quotient is 0, no floor to 1.
    t.settle_consilience(&[[1; 32], [2; 32], [3; 32], [4; 32]], [0, 0, 1, 0], anchor);
    assert_eq!(t.balance_mu(&[1; 32]), [0; 4]);
    assert_eq!(t.balance_mu(&[2; 32]), [0; 4]);
    assert_eq!(t.balance_mu(&[3; 32]), [0; 4]);
    assert_eq!(t.balance_mu(&[4; 32]), [0; 4]);

    // Same exact law for settle_assembly
    t.settle_assembly(&[[1; 32], [2; 32], [3; 32], [4; 32]], [0, 0, 1, 0], anchor);
    assert_eq!(t.balance_mu(&[1; 32]), [0; 4]);
}

/// STD1 — paid ≤ held, AND THE DEBIT SHORTS NOTHING. A recall and a door
/// fee asking more than the balance holds pay what it holds and read the
/// rest as the shortfall; the debit of the paid amount against that same
/// balance goes through whole, its burn row is written, and the ledger
/// still conserves. The arm that read a short debit here could not fire,
/// and this is the reading it was standing in for.
#[test]
fn a_fee_past_the_balance_pays_what_is_held_and_the_debit_shorts_nothing() {
    let mut t = Treasury::new();
    let a = at(1, [1; 32]);
    t.genesis([7; 32], [5, 2, 0, 0], a);
    let rows = t.entry_count();
    let (paid, short) = t.settle_recall([7; 32], &[8, 1, 0, 0], &[0; 4], a);
    assert_eq!((paid, short), ([5, 1, 0, 0], [3, 0, 0, 0]), "paid is what was held, the rest is short");
    assert_eq!(t.balance_mu(&[7; 32]), [0, 1, 0, 0], "the whole of paid left the balance");
    assert_eq!(t.entry_count(), rows + 1, "one burn row, for what left");
    let (paid, short) = t.settle_door([7; 32], [0, 4, 0, 0], a);
    assert_eq!((paid, short), ([0, 1, 0, 0], [0, 3, 0, 0]), "the door fee reads the same way");
    assert_eq!(t.balance_mu(&[7; 32]), [0; 4]);
    assert_eq!(t.audit().expect("the ledger still conserves"), [0; 4]);
}

/// STD1 — THE SAME CLAMP AT THREE MORE DOORS, AND ITS DEBIT GOES THROUGH. A
/// use, an enrolment and a burn each asking past the balance move exactly
/// what was held — the use's transfer, the stake, the burn — and the ledger
/// conserves after each. Each had an arm reading a short debit that could not
/// fire; this is what stands in its place.
#[test]
fn a_use_an_enrolment_and_a_burn_past_the_balance_move_what_is_held() {
    let mut t = Treasury::new();
    let a = at(1, [1; 32]);
    t.genesis([1; 32], [3, 1, 0, 0], a);
    t.genesis([2; 32], [0, 0, 0, 0], a);
    assert_eq!(t.settle_use([1; 32], [2; 32], [5, 1, 0, 0], a), [3, 1, 0, 0], "the use moves what the demander held");
    assert_eq!((t.balance_mu(&[1; 32]), t.balance_mu(&[2; 32])), ([0; 4], [3, 1, 0, 0]));
    assert_eq!(t.audit().expect("a transfer conserves"), [3, 1, 0, 0]);
    assert_eq!(t.enroll([2; 32], [9, 9, 0, 0], a), [3, 1, 0, 0], "the enrolment stakes what the balance held");
    assert_eq!((t.balance_mu(&[2; 32]), t.bond_of(&[2; 32])), ([0; 4], Some([3, 1, 0, 0])));
    assert_eq!(t.audit().expect("a stake conserves"), [3, 1, 0, 0]);
    t.genesis([2; 32], [0, 2, 0, 0], a);
    assert_eq!(t.burn([2; 32], [0, 5, 0, 0], a), [0, 3, 0, 0], "the burn takes the bond, then the balance, no more");
    assert_eq!((t.balance_mu(&[2; 32]), t.bond_of(&[2; 32])), ([0; 4], Some([3, 0, 0, 0])));
    assert_eq!(t.audit().expect("a burn conserves"), [3, 0, 0, 0]);
}

