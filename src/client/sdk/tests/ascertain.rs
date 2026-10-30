// MU-CLIENT §4.4 — A RULE KNOWN BY ITS PERSPECTIVES, AND ONE UNDETERMINED BY
// THEM (ruled 2026-10-22): on a two-pair family each sibling
// set is one pair, and the act reader states no clause from one pair, so a
// clause rule proven on both pairs is undetermined by its perspectives — sent
// for the court to judge — never refuted by a sibling that cannot speak it.

use crate::client::sdk::entity::family::{ascertain_on, Ascertained, Unexempt};
use crate::client::sdk::*;

fn grid(rows: &[&str]) -> crate::arc::Grid {
    rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
}

/// The mirrored echo: every body repeated three columns over, mirrored
/// across its own columns — a clause rule the act reader states over two
/// pairs and cannot state over one.
fn mirrored(pairs: usize) -> Vec<(crate::arc::Grid, crate::arc::Grid)> {
    let all = [
        (grid(&["1100000", "1000000", "0000000"]), grid(&["1101100", "1000100", "0000000"])),
        (grid(&["0000000", "0022000", "0020000"]), grid(&["0000000", "0022022", "0020002"])),
        (grid(&["000000", "110000", "100000", "000000"]), grid(&["000000", "110110", "100010", "000000"])),
    ];
    all.into_iter().take(pairs).collect()
}

fn rule_of(train: &[(crate::arc::Grid, crate::arc::Grid)]) -> crate::transducer::sentence::SentenceRule {
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
    crate::transducer::family::extract_rule(train, &mut budget).expect("the family has a rule")
}

/// THE TWO-PAIR CLAUSE RULE IS UNDETERMINED, NOT UNKNOWN: its full rule is
/// the act reader's clauses, proven on both pairs; each one-pair sibling set
/// yields no clause rule, its held-out pair fails — and the ascertainment
/// reads undetermined, never a refutation.
#[test]
fn a_two_pair_clause_rule_is_undetermined_by_its_perspectives() {
    let train = mirrored(2);
    let rule = rule_of(&train);
    assert!(!rule.form.elements.is_empty(), "the act reader stated it as clauses");
    let epi = Epigenome::new();
    let (outcome, _) = ascertain_on(&epi, &train, &rule);
    assert!(matches!(outcome, Ascertained::Undetermined), "undetermined by its one-pair perspectives: {outcome:?}");
}

/// GUARD 1 — A VERTEX FIT IS NEVER EXEMPT: the same family judged with its
/// clauses struck from the rule stays unknown.
#[test]
fn a_vertex_fit_on_two_pairs_stays_unknown() {
    let train = mirrored(2);
    let mut rule = rule_of(&train);
    rule.form.elements.clear();
    let (outcome, _) = ascertain_on(&Epigenome::new(), &train, &rule);
    assert!(matches!(outcome, Ascertained::Unknown(Unexempt::Vertex)), "no clauses, no exemption — guard 1 named: {outcome:?}");
}

/// GUARD 2 — THE RULE MUST CLOSE BOTH PAIRS: a clause rule read off another
/// family, which does not re-derive these pairs, is not undetermined here.
#[test]
fn a_clause_rule_that_does_not_close_its_pairs_is_not_undetermined() {
    let train = mirrored(2);
    let other = rule_of(&[
        (grid(&["0000", "3000", "0000", "0000"]), grid(&["0000", "3333", "0000", "0000"])),
        (grid(&["0000", "0000", "0030", "0000"]), grid(&["0000", "0000", "0033", "0000"])),
    ]);
    assert!(!other.form.elements.is_empty(), "the other family's rule is clauses too");
    let (outcome, _) = ascertain_on(&Epigenome::new(), &train, &other);
    assert!(matches!(outcome, Ascertained::Unknown(Unexempt::Unproven)), "unproven on these pairs — guard 2 named: {outcome:?}");
}

/// GUARD 3 — THREE PAIRS ARE READ BY THEIR PERSPECTIVES: with two pairs to
/// each sibling set the act reader states the rule, and the family is known
/// by its perspectives — the exemption is never reached past two pairs.
#[test]
fn three_pairs_are_read_by_their_perspectives() {
    let train = mirrored(3);
    let rule = rule_of(&train);
    let (outcome, _) = ascertain_on(&Epigenome::new(), &train, &rule);
    assert!(matches!(outcome, Ascertained::Known), "known by its perspectives: {outcome:?}");
}
