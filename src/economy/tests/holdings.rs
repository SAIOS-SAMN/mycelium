// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// 78.174 §1 item 2, #169 (ruled 2026-11-06) — EVERY HOLDING IS ITS MODES: a
// genesis allocation at the founding mode, a bond holding the modes it was
// staked from, a per-lane amount drawn from a lane's modes in proportion,
// the modes drawn travelling with the transfer, and both equations of §6
// mode by mode.

use super::{at, round_trip};
use crate::economy::modes::{Mode, Modes, G, H, S, T};
use crate::economy::{Mu, Treasury};
use crate::transducer::cga;
use num_rational::BigRational;

fn q(n: i64, d: i64) -> BigRational {
    BigRational::new(n.into(), d.into())
}

fn lanes(s: (i64, i64), g: (i64, i64), t: (i64, i64), h: (i64, i64)) -> Mu {
    Mu::of([q(s.0, s.1), q(g.0, g.1), q(t.0, t.1), q(h.0, h.1)]).expect("non-negative lanes")
}

/// A slide by `x` along the first axis, its modes at `share` of the frame.
fn slide(x: i64, share: BigRational) -> Modes {
    let m = cga::translator_at(&[q(x, 1), q(0, 1), q(0, 1)]);
    Modes::of_motion(&m, &share, &mut crate::physics::Physics::default().stroke_n(1 << 16)).expect("the slide's modes read")
}

/// A genesis allocation is held at the founding mode of each lane it
/// allocates: the lane's identity motion, stated as the mode, never empty.
#[test]
fn genesis_allocation_sits_at_the_founding_mode() {
    let mut t = Treasury::new();
    let party = [7; 32];
    let read = t.genesis(party, [8, 0, 3, 1], at(0, [0; 32]));
    assert_eq!(read, [8, 0, 3, 1], "the allocation reads as allocated");
    let held = t.balance_modes(&party);
    assert!(held.is_founding(), "every mode of a genesis allocation is a founding mode: {}", held.print());
    assert_eq!(held.amount_of(&Mode::Founding { lane: S }), Some(&q(8, 1)));
    assert_eq!(held.amount_of(&Mode::Founding { lane: G }), None, "a lane at zero holds no mode");
    assert_eq!(held.amount_of(&Mode::Founding { lane: T }), Some(&q(3, 1)));
    assert_eq!(held.amount_of(&Mode::Founding { lane: H }), Some(&q(1, 1)));
    let row = t.entries().iter().next_back().expect("a genesis row");
    assert_eq!(row.modes, held, "the row carries the founding modes it allocated");
    assert!(t.audit().is_ok());
}

/// The founding mode of each lane has its own coordinates, travels the wire
/// whole, and a founding mark on no lane does not read.
#[test]
fn founding_mode_is_keyed_by_its_lane_and_round_trips() {
    let all = Modes::founding(&Mu::counts([1, 2, 3, 4]));
    assert_eq!(all.modes().count(), 4, "one founding mode per lane");
    for lane in [S, G, T, H] {
        let key = Mode::Founding { lane }.key().expect("a founding mode has coordinates");
        assert_eq!(key, vec![crate::economy::modes::FOUNDING | lane]);
    }
    let bytes = all.to_bytes();
    assert_eq!(Modes::from_bytes(&bytes), Some(all.clone()));
    let mut off_lane = Vec::new();
    off_lane.extend_from_slice(&1u32.to_le_bytes());
    off_lane.push(crate::economy::modes::FOUNDING | 4);
    crate::transducer::alphabet::put_q(&mut off_lane, &q(1, 1));
    assert!(Modes::from_bytes(&off_lane).is_none(), "a founding mark on no lane refuses");
    assert!(Mode::Founding { lane: 4 }.key().is_none());
}

/// A per-lane amount is drawn from the modes on its lane in proportion to
/// their amounts, exactly: the parts sum to the amount, and no lane is paid
/// from another's modes.
#[test]
fn draw_takes_a_lane_in_proportion_to_its_modes() {
    let mut held = Modes::founding(&lanes((0, 1), (0, 1), (1, 1), (2, 1)));
    held.add(&slide(1, q(1, 1)));
    let flux_t = held.lane_amount(T);
    assert!(held.modes().filter(|(m, _)| m.lane() == T).count() == 2, "T holds its founding mode and the slide's flux");
    // Ask half the T lane: each T mode gives half of itself.
    let half = Mu::of([q(0, 1), q(0, 1), flux_t.clone() / q(2, 1), q(0, 1)]).expect("non-negative");
    let drawn = held.draw(&half).expect("half of what the lane holds");
    assert_eq!(drawn.amount_of(&Mode::Founding { lane: T }), Some(&q(1, 2)));
    assert_eq!(drawn.lane_amount(T), flux_t.clone() / q(2, 1), "the parts sum to the amount asked");
    assert_eq!(held.lane_amount(T), flux_t / q(2, 1), "and the lane keeps the rest");
    assert_eq!(held.lane_amount(H), q(2, 1), "no other lane is touched");
    assert_eq!(drawn.lane_amount(H), q(0, 1));
}

/// A lane holding less than is asked refuses the draw by name, and nothing
/// moves; another lane's modes never cover it.
#[test]
fn draw_refuses_a_short_lane_and_moves_nothing() {
    let mut held = Modes::founding(&Mu::counts([5, 0, 1, 0]));
    let before = held.clone();
    let short = held.draw(&Mu::counts([0, 1, 0, 0])).expect_err("G holds nothing");
    assert_eq!((short.lane, short.held.clone(), short.asked.clone()), (G, q(0, 1), q(1, 1)));
    assert_eq!(held, before, "a refused draw takes nothing");
}

/// A bond holds the modes it was staked from: a stake draws the cover from
/// the balance's modes in proportion on each lane, and those modes, not
/// others, are what the bond holds and what an unstake returns.
#[test]
fn bond_holds_the_modes_it_was_staked_from() {
    let mut t = Treasury::new();
    let party = [3; 32];
    t.genesis(party, [4, 0, 2, 0], at(0, [0; 32]));
    let added = t.enroll(party, [2, 0, 1, 0], at(1, [1; 32]));
    assert_eq!(added, [2, 0, 1, 0]);
    let bond = t.bond_modes(&party).expect("enrolled").clone();
    assert_eq!(bond, Modes::founding(&Mu::counts([2, 0, 1, 0])), "the bond holds the founding modes the balance held");
    assert_eq!(t.entries().iter().next_back().map(|e| &e.modes), Some(&bond), "the stake row carries them");
    let back = t.unstake(party, [1, 0, 0, 0], at(2, [2; 32]));
    assert_eq!(back, [1, 0, 0, 0]);
    assert_eq!(t.balance_modes(&party), Modes::founding(&Mu::counts([3, 0, 1, 0])));
    assert!(t.audit().is_ok());
}

/// A transfer carries the modes drawn from the payer: the payee holds them,
/// and both rows of the transfer carry the same modes.
#[test]
fn transfer_carries_the_modes_drawn_from_the_payer() {
    let mut t = Treasury::new();
    let (payer, payee) = ([1; 32], [2; 32]);
    t.genesis(payer, [0, 0, 2, 0], at(0, [0; 32]));
    t.settle_solve(payer, slide(1, q(1, 1)), at(1, [1; 32]));
    let t_lane = t.balance_modes(&payer).lane_amount(T);
    let asked = Mu::of([q(0, 1), q(0, 1), t_lane.clone() / q(2, 1), q(0, 1)]).expect("non-negative");
    let mut expected = t.balance_modes(&payer);
    let drawn = expected.draw(&asked).expect("half the lane");
    t.pay(payer, payee, asked, at(2, [2; 32])).expect("the payer holds it");
    assert_eq!(t.balance_modes(&payee), drawn, "the payee holds the drawn modes");
    assert_eq!(t.balance_modes(&payer), expected, "the payer keeps the rest of each mode");
    let rows: Vec<_> = t.entries().iter().rev().take(2).map(|e| e.modes.clone()).collect();
    assert_eq!(rows, vec![drawn.clone(), drawn], "both halves carry the same modes");
    assert!(t.audit().is_ok());
}

/// A mint carries the modes of the motion its proof cites; a subsidy, whose
/// proof cites none, is minted at the founding mode.
#[test]
fn mint_carries_cited_modes_and_subsidy_the_founding_mode() {
    let mut t = Treasury::new();
    let closer = [9; 32];
    let modes = slide(2, q(1, 18));
    let paid = t.settle_solve(closer, modes.clone(), at(0, [0; 32]));
    assert_eq!(paid, modes.reading());
    assert_eq!(t.balance_modes(&closer), modes, "the closer holds the closure's modes");
    t.settle_subsidy(closer, [0, 1, 0, 0], at(1, [1; 32]));
    assert_eq!(t.balance_modes(&closer).amount_of(&Mode::Founding { lane: G }), Some(&q(1, 1)));
    assert!(t.audit().is_ok());
}

/// Both equations of §6 hold mode by mode, and they survive the persisted
/// form: the carriers and the journal carry the modes whole.
#[test]
fn audit_holds_mode_by_mode_through_the_persisted_form() {
    let mut t = Treasury::new();
    let (a, b) = ([1; 32], [2; 32]);
    t.genesis(a, [6, 2, 0, 0], at(0, [0; 32]));
    t.settle_solve(a, slide(1, q(1, 2)), at(1, [1; 32]));
    t.enroll(a, [3, 0, 0, 0], at(2, [2; 32]));
    t.pay(a, b, [2, 1, 0, 0], at(3, [3; 32])).expect("held");
    t.hold(a, b"lease", [1, 0, 0, 0], at(4, [4; 32])).expect("held");
    t.burn(a, [1, 0, 0, 0], at(5, [5; 32]));
    let supply = t.audit().expect("the carriers are what was issued");
    let back = round_trip(&t).expect("the persisted form reads back");
    assert_eq!(back.audit(), Ok(supply));
    assert_eq!(back.balance_modes(&a), t.balance_modes(&a));
    assert_eq!(back.bond_modes(&a), t.bond_modes(&a));
}
