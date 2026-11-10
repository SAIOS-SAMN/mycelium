// MU-ECONOMY §0, §3.4, §5.3; 78.174 §1 item 7 — the proofs-of-work reading:
// the level a payment stands at, read off the words its act names; another
// court's reading of the same act, the same point or a named residue; and
// one closure paid twice as a first closure, an equivocation of the advance.

use crate::proofs::{retraverse, second_mints, Level, Ruled, Traversal};
use mycelium::anastome::dialogue::why::{Named, Why};
use mycelium::chronometry::{ActKind, Stamp};
use mycelium::economy::Arm;
use mycelium::observe::{JudgedSolve, JudgedSolves};
use mycelium::transducer::sentence::Reasons;

fn word(head: u8) -> Named {
    Named { head: [head; 8], admitted: Stamp::default(), task: Some([7; 8]), reasons: None }
}

fn sentence(head: u8) -> Named {
    Named { reasons: Some(Reasons::default()), ..word(head) }
}

fn act(kind: ActKind, named: Vec<Named>) -> Why {
    Why { at: Stamp::default(), kind, mu: [0; 4], named, unheld: 0, refused: Vec::new() }
}

fn first(citation: u8, voice: u8, times_first: u64) -> JudgedSolve {
    JudgedSolve { citation: [citation; 32], voice: [voice; 32], verdict: 2, height: 9, times: [0, 0, times_first, 0] }
}

#[test]
fn level_is_a_sentence_where_a_named_word_carries_reasons() {
    assert_eq!(Level::of(&[word(1), sentence(2)]), Level::Sentence);
}

#[test]
fn level_is_a_word_where_no_named_word_carries_reasons() {
    assert_eq!(Level::of(&[word(1), word(2)]), Level::Word);
}

#[test]
fn level_is_unread_where_the_act_names_no_word() {
    assert_eq!(Level::of(&[]), Level::Unread);
}

/// 78.174 §1 item 7: sentences POUW, words POWR — each ruled at its own
/// level and not at the other's.
#[test]
fn sentences_are_ruled_pouw_and_words_powr() {
    assert_eq!(Level::Sentence.ruled(Arm::Pouw), Ruled::AtThisLevel);
    assert_eq!(Level::Word.ruled(Arm::Powr), Ruled::AtThisLevel);
    assert_eq!(Level::Sentence.ruled(Arm::Powr), Ruled::NotAtThisLevel);
    assert_eq!(Level::Sentence.ruled(Arm::Powc), Ruled::NotAtThisLevel);
}

/// At a word, POUW and POWC stand only as citation shares: no served
/// reading tells a conjugate class from a word.
#[test]
fn pouw_and_powc_at_a_word_stand_only_as_citation_shares() {
    assert_eq!(Level::Word.ruled(Arm::Pouw), Ruled::CitationShare);
    assert_eq!(Level::Word.ruled(Arm::Powc), Ruled::CitationShare);
}

/// POWV is ruled at every level, even where no level is read; every other
/// arm at an unread level stands nowhere in the map.
#[test]
fn powv_is_ruled_at_every_level_and_nothing_else_at_an_unread_one() {
    for level in [Level::Sentence, Level::Word, Level::Unread] {
        assert_eq!(level.ruled(Arm::Powv), Ruled::AtEveryLevel);
    }
    for arm in [Arm::Pouw, Arm::Powc, Arm::Powr] {
        assert_eq!(Level::Unread.ruled(arm), Ruled::LevelUnread);
    }
}

#[test]
fn the_same_act_and_words_at_another_court_is_the_same_point() {
    let here = act(ActKind::Speech, vec![sentence(1), word(2)]);
    let there = act(ActKind::Speech, vec![word(2), sentence(1)]);
    assert_eq!(retraverse(&here, &there), Traversal::Same);
}

#[test]
fn a_word_held_at_one_court_only_is_a_named_residue() {
    let here = act(ActKind::Speech, vec![sentence(1), word(2)]);
    let there = act(ActKind::Speech, vec![sentence(1), word(3)]);
    let Traversal::Residue(r) = retraverse(&here, &there) else { panic!("another point") };
    assert_eq!(r.len(), 2, "the word missing there and the word only there, each named: {r:?}");
}

#[test]
fn another_act_kind_at_the_stamp_is_a_named_residue() {
    let here = act(ActKind::Speech, vec![sentence(1)]);
    let there = act(ActKind::Solve, vec![sentence(1)]);
    assert!(matches!(retraverse(&here, &there), Traversal::Residue(r) if r.len() == 1));
}

#[test]
fn a_word_whose_reasons_differ_there_is_a_named_residue() {
    let here = act(ActKind::Speech, vec![sentence(1)]);
    let there = act(ActKind::Speech, vec![word(1)]);
    assert!(matches!(retraverse(&here, &there), Traversal::Residue(r) if r.len() == 1));
}

#[test]
fn a_closure_judged_first_at_two_courts_is_two_mints_of_one_point() {
    let a = JudgedSolves { solves: vec![first(5, 1, 1), first(6, 1, 1)] };
    let b = JudgedSolves { solves: vec![first(5, 1, 1)] };
    let twice = second_mints(&[([0xa; 32], a), ([0xb; 32], b)]);
    assert_eq!(twice.len(), 1, "only the task closed first at both courts");
    assert_eq!(twice[0].0, [5; 32]);
    assert_eq!(twice[0].1.len(), 2, "each court's first closure named");
}

#[test]
fn a_closure_judged_first_twice_at_one_court_is_two_mints_of_one_point() {
    let a = JudgedSolves { solves: vec![first(5, 1, 1), first(5, 2, 1)] };
    assert_eq!(second_mints(&[([0xa; 32], a)]).len(), 1, "two voices paid first for one task");
    let b = JudgedSolves { solves: vec![first(6, 1, 2)] };
    assert_eq!(second_mints(&[([0xb; 32], b)]).len(), 1, "one voice paid first twice for one task");
}

#[test]
fn a_closure_judged_first_once_is_no_equivocation() {
    let a = JudgedSolves { solves: vec![first(5, 1, 1), JudgedSolve { times: [3, 4, 0, 0], ..first(5, 2, 0) }] };
    assert!(second_mints(&[([0xa; 32], a)]).is_empty(), "a closure again pays nothing and mints nothing twice");
}
