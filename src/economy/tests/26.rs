// 26, E2 — the market settles: novelty pays on use and not on existence,
// and every settlement is a vector of four lanes.

use crate::economy::*;
use crate::transducer::element::{from_residual, Element};
use crate::transducer::germinate::germinate;

fn germ() -> Element {
    // A germinated pattern (post-excision, the only species of germ).
    from_residual(&germinate(&[(0, 0, 1u8), (0, 1, 2)], &[(0, 0, 1u8), (0, 1, 9)]).unwrap())
}

#[test]
fn novelty_pays_on_use_not_on_existence() {
    let mut t = Treasury::new();
    t.enroll([1; 32], UNIT_BOND, (0, [0; 32])); // genesis: the bond must exist
    let g = germ();
    let anchor: Anchor = (3, [7; 32]);
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
    assert!(t.entries().iter().filter(|e| e.kind != "stake").all(|e| e.k_step == 3 && e.receipt == [7; 32]));
}

/// The dimensional ledger: consilience splits the PRICED μ⃗ per lane (nonzero
/// lanes never split to zero), and the torse coin books on the holonomic lane.
#[test]
fn consilience_and_torse_settle_as_vectors() {
    let mut t = Treasury::new();
    t.enroll([1; 32], UNIT_BOND, (0, [0; 32])); // capitalize before settling
    let anchor: Anchor = (9, [1; 32]);
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
    let snap = t.snapshot();
    let mut pos = 0;
    let back = Treasury::restore(&snap, &mut pos).expect("restore");
    assert_eq!(back.entries(), t.entries(), "μ⃗ entries + anchors survive, exactly");
}

/// A release with an empty stake bond still pays the measured vector.
#[test]
fn release_mints_when_the_bond_is_empty() {
    let mut t = Treasury::new();
    let addr = [7u8; 32];
    t.escrow_award(addr, vec![[1; 32], [2; 32]], [4, 0, 2, 0], (1, [9; 32]));
    assert_eq!(t.bond_total(), [0; 4]);
    let held = t.release_award(&addr).expect("claim");
    t.mint_claim(&held.voices, held.mu, held.anchor);
    assert_eq!(t.balance_mu(&[1; 32]), [2, 0, 1, 0]);
    assert_eq!(t.balance_mu(&[2; 32]), [2, 0, 1, 0]);
    assert_eq!(t.supply().expect("supply adds up"), [4, 0, 2, 0]);
    assert_eq!(t.bond_total(), [0; 4], "mint does not touch the stake bond");
}

/// S9.3 — THE READING, PER LANE. `lane_moves` says what each lane did and
/// decides nothing; Dershowitz–Manna descent is one sentence a reader can
/// say about it (no lane rose, some lane fell), and the reading keeps the
/// three cases apart that one bool collapsed into `false`.
#[test]
fn a_lane_move_is_read_per_lane() {
    let fell = lane_moves(&[0, 0, 0, 0], &[1, 0, 0, 0]);
    assert_eq!(fell.per_lane, [-1, 0, 0, 0], "the fall is signed and on its lane");
    assert_eq!(fell.fell(), vec![0]);
    assert!(fell.rose().is_empty(), "and nothing rose");
    assert_eq!(fell.deflation(), [1, 0, 0, 0], "what fell is what is paid");

    // A trade: one lane down, another up. The old bool said only "no".
    let traded = lane_moves(&[1, 0, 0, 0], &[0, 1, 0, 0]);
    assert_eq!(traded.rose(), vec![0], "the raise is named");
    assert_eq!(traded.fell(), vec![1], "and so is the fall it was traded for");
    assert_eq!(traded.deflation(), [0, 1, 0, 0], "a raised lane pays nothing");

    // Equal is not descent — and now it says why: nothing moved at all.
    let still = lane_moves(&[1, 1, 0, 0], &[1, 1, 0, 0]);
    assert!(still.is_still(), "the mind did not reduce");
    assert!(still.rose().is_empty() && still.fell().is_empty());
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
    t.escrow_award(addr, vec![[1; 32]], [8, 0, 4, 0], (1, [1; 32]));
    t.escrow_award(addr, vec![[2; 32]], [8, 0, 4, 0], (1, [1; 32]));
    let share = t.release_rewrite(&addr).expect("two parties stood it");
    assert_eq!(share, [4, 0, 2, 0], "one quantity, split per lane, lanes never meeting");
    assert_eq!(t.balance_mu(&[1; 32]), share, "the voice that stood it");
    assert_eq!(t.balance_mu(&[2; 32]), share, "and the one that stood it after");
    assert!(t.release_rewrite(&addr).is_none(), "the escrow is spent once");
}

/// A mint with no payee is a mint to nobody: an escrow naming no voice
/// is not released and stands (rule 11 — the old fallback invented one).
#[test]
fn an_escrow_with_no_voice_is_not_released() {
    let mut t = Treasury::new();
    let addr = [4u8; 32];
    t.escrow_award(addr, Vec::new(), [8, 0, 0, 0], (1, [1; 32]));
    assert!(t.release_rewrite(&addr).is_none(), "no party, no release");
    assert_eq!(t.escrowed_remaining(&addr), [8, 0, 0, 0], "the escrow stands");
}
