// MU-CLIENT §4.4 — A RULE KNOWN BY ITS PERSPECTIVES, AND ONE UNDETERMINED BY
// THEM (ruled 2026-10-22): on a two-pair family each sibling
// set is one pair, and the act reader states no clause from one pair, so a
// clause rule proven on both pairs is undetermined by its perspectives — sent
// for the court to judge — never refuted by a sibling that cannot speak it.

use crate::client::sdk::mind::family::{ascertain_on, Ascertained, Unexempt};
use crate::client::sdk::*;

fn digits(rows: &[&str]) -> crate::transducer::family::SectionSample {
    rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect()
}

/// The mirrored echo: every body repeated three columns over, mirrored
/// across its own columns — a clause rule the act reader states over two
/// pairs and cannot state over one.
fn mirrored(pairs: usize) -> Vec<(crate::transducer::family::SectionSample, crate::transducer::family::SectionSample)> {
    let all = [
        (digits(&["1100000", "1000000", "0000000"]), digits(&["1101100", "1000100", "0000000"])),
        (digits(&["0000000", "0022000", "0020000"]), digits(&["0000000", "0022022", "0020002"])),
        (digits(&["000000", "110000", "100000", "000000"]), digits(&["000000", "110110", "100010", "000000"])),
    ];
    all.into_iter().take(pairs).collect()
}

fn rule_of(train: &[(crate::transducer::family::SectionSample, crate::transducer::family::SectionSample)]) -> crate::transducer::sentence::SentenceRule {
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
    crate::transducer::family::extract_rule(train, &mut budget).expect("the family has a rule")
}

/// THE TWO-PAIR CLAUSE RULE IS UNDETERMINED, NOT UNKNOWN: its full rule is
/// the act reader's clauses, proven on both pairs; each one-pair sibling set
/// yields no clause rule, its held-out pair fails — and the ascertainment
/// reads undetermined, never a refutation.
#[test]
fn ascertain_on_leaves_two_pair_rule_undetermined() {
    crate::transducer::raster::stand_on(0);
    let train = mirrored(2);
    let rule = rule_of(&train);
    assert!(!rule.form.elements.is_empty(), "the act reader stated it as clauses");
    let epi = MindLanguage::new();
    let (outcome, _) = ascertain_on(&epi, &train, &rule);
    assert!(matches!(outcome, Ascertained::Undetermined), "undetermined by its one-pair perspectives: {outcome:?}");
}

/// GUARD 1 — A VERTEX FIT IS NEVER EXEMPT: the same family judged with its
/// clauses struck from the rule stays unknown.
#[test]
fn ascertain_on_leaves_vertex_fit_unknown() {
    let train = mirrored(2);
    let mut rule = rule_of(&train);
    rule.form.elements.clear();
    let (outcome, _) = ascertain_on(&MindLanguage::new(), &train, &rule);
    assert!(matches!(outcome, Ascertained::Unknown(Unexempt::Vertex)), "no clauses, no exemption — guard 1 named: {outcome:?}");
}

/// GUARD 2 — THE RULE MUST CLOSE BOTH PAIRS: a clause rule read off another
/// family, which does not re-derive these pairs, is not undetermined here.
#[test]
fn ascertain_on_leaves_unclosing_rule_unproven() {
    let train = mirrored(2);
    let other = rule_of(&[
        (digits(&["0000", "3000", "0000", "0000"]), digits(&["0000", "3333", "0000", "0000"])),
        (digits(&["0000", "0000", "0030", "0000"]), digits(&["0000", "0000", "0033", "0000"])),
    ]);
    assert!(!other.form.elements.is_empty(), "the other family's rule is clauses too");
    let (outcome, _) = ascertain_on(&MindLanguage::new(), &train, &other);
    assert!(matches!(outcome, Ascertained::Unknown(Unexempt::Unproven)), "unproven on these pairs — guard 2 named: {outcome:?}");
}

/// GUARD 3 — THREE PAIRS ARE READ BY THE PAIRS' PERSPECTIVES: with two pairs to
/// each sibling set the act reader states the rule, and the family is known
/// by its perspectives — the exemption is never reached past two pairs.
#[test]
fn ascertain_on_knows_rule_on_three_pairs() {
    crate::transducer::raster::stand_on(0);
    let train = mirrored(3);
    let rule = rule_of(&train);
    let (outcome, _) = ascertain_on(&MindLanguage::new(), &train, &rule);
    assert!(matches!(outcome, Ascertained::Known), "known by its perspectives: {outcome:?}");
}

/// 1x1 whole-frame dihedral symmetry (`blocks = [1, 1]`, non-identity turn)
/// is extracted by `read_tiles` via `read_acts` and ascertained across the family.
#[test]
fn read_tiles_1x1_dihedral_symmetry() {
    crate::transducer::raster::stand_on(0);
    // 90° turn (`t = 4`: `[a, b] -> [b, first - 1 - a]`):
    let train = vec![
        (digits(&["10", "20"]), digits(&["21", "00"])),
        (digits(&["300", "400", "000"]), digits(&["043", "000", "000"])),
        (digits(&["56", "00"]), digits(&["05", "06"])),
    ];
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
    let clauses = crate::transducer::family::acts::read_acts(
        &crate::transducer::sentence::Form::default(),
        &train,
        &mut budget,
    )
    .expect("read_acts should extract 1x1 Tile");
    assert!(
        clauses.iter().any(|c| matches!(
            &c.act,
            crate::transducer::sentence::clause::Act::Tile { blocks: [1, 1], turns } if turns == &[4]
        )),
        "expected 1x1 Tile turn 4, got {clauses:?}"
    );
    let elements = crate::transducer::sentence::rule_element::lower_all(&clauses).expect("lower_all");
    let rule = crate::transducer::sentence::rule_of_form(crate::transducer::sentence::Form {
        elements,
        ..crate::transducer::sentence::Form::default()
    });
    let epi = MindLanguage::new();
    let (outcome, _) = ascertain_on(&epi, &train, &rule);
    assert!(
        matches!(outcome, Ascertained::Known | Ascertained::Undetermined),
        "expected Known or Undetermined, got {outcome:?}"
    );
    let test_in = digits(&["70", "80"]);
    let (outputs, _) = crate::client::sdk::mind::family::tests_spoken(&epi, &rule, &[test_in]);
    assert_eq!(outputs, Some(vec![digits(&["87", "00"])]));
}

/// Autonomous conformal grounding from `TaskPercept` derives a `Tile` rule
/// when `extract_on` / `ascertain_on` would otherwise stop at `Stop::Unknown`.
#[test]
fn ground_from_percept_solves_unknown_conformal_symmetry() {
    crate::transducer::raster::stand_on(0);
    let train = vec![
        (digits(&["10", "20"]), digits(&["21", "00"])),
        (digits(&["300", "400", "000"]), digits(&["043", "000", "000"])),
    ];
    let tests = vec![digits(&["70", "80"])];
    let to_sample = |s: &crate::transducer::family::SectionSample| {
        let ex = s.extents();
        let symbols: Vec<u8> = s.sites().map(|(_, h)| h).collect();
        crate::transducer::sample::Sample::new(ex.to_vec(), 1, symbols, crate::transducer::sample::Place::new(vec![0, 0]))
            .expect("valid sample")
    };
    let s_train: Vec<_> = train.iter().map(|(x, y)| (to_sample(x), to_sample(y))).collect();
    let s_tests: Vec<_> = tests.iter().map(to_sample).collect();
    let mut budget = crate::physics::Physics::default().stroke_n(1 << 20);
    let percept = crate::transducer::family::transduce::TaskPercept::of(&s_train, &s_tests, &mut budget).expect("valid percept");

    let mut mind = Mind::open("127.0.0.1:1", Voice::named("conformal"));
    let id = [9u8; 32];
    mind.board.percepts.insert(id, percept);

    let scene = crate::client::sdk::mind::perceive::Scene {
        id: id.to_vec(),
        train,
        tests,
        reading: crate::client::sdk::mind::perceive::SceneKind::Task,
    };
    let (_, outputs) = mind.ground_from_percept(&scene).expect("TaskPercept conformal symmetry should ground");
    assert_eq!(outputs, vec![digits(&["87", "00"])]);
}
