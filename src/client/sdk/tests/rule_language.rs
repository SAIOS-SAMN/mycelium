// 78.143 — THE RULE LANGUAGE REFUSES BY NAME: an unnamed tie and a
// predicate on a coordinate that is not exact, at the seam and at the
// court. The rule language's corpus tasks (minted over a task's train pairs
// and solved on its own test pair) are read through the corpus's decoder,
// in `sources/arc-agi/tests/rule_language.rs` (ROADMAP track 26, S1).

use crate::client::sdk::rule::{sentence, would_judge, Coord, Extent, NoRule, Order, Pred, Step, Tie, Unstated};
use crate::cognition::{Court, SolveVerdict};
use crate::transducer::element::Element;
use crate::transducer::family::{Grid, NoCrystal};

fn judged(rule: &Element, withheld: &[(Grid, Grid)]) -> (SolveVerdict, Result<(), NoRule>) {
    let mut k = Court::new();
    let verdict = k.judge_solve(rule, withheld);
    let court: &Court = &k;
    let seam = would_judge(rule, withheld, &|a| court.resolve_form(a), court);
    (verdict, seam)
}

/// AN UNNAMED TIE IS REFUSED BY NAME: at the seam, over a family where two
/// bodies tie for the largest; and at the court, a rule minted where no tie
/// stood, judged on a pair where one does.
#[test]
fn sentence_refuses_unnamed_tie() {
    let largest = || Step::Select(Pred::extremum(Coord::Shell, Order::Greatest, Tie::Refuse));
    let g = |rows: &[&str]| -> Grid { rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect() };
    let tied = vec![(g(&["11000", "00000", "00022"]), g(&["11"]))];
    let got = sentence(&[largest(), Step::Crop(Extent::Body)], &tied, &NoCrystal);
    assert!(matches!(got, Err(NoRule::Unstated(Unstated::Tie))), "the seam refuses the tie: {got:?}");
    // HELD, the tie selects both, and a crop of two is refused by its own name.
    let held = sentence(
        &[Step::Select(Pred::extremum(Coord::Shell, Order::Greatest, Tie::Hold)), Step::Crop(Extent::Body)],
        &tied,
        &NoCrystal,
    );
    assert!(matches!(held, Err(NoRule::Unstated(Unstated::NotOne))), "a held tie is two bodies: {held:?}");
    let clear = vec![(g(&["11100", "00000", "00022"]), g(&["111"]))];
    let rule = sentence(&[largest(), Step::Crop(Extent::Body)], &clear, &NoCrystal).expect("no tie, it mints");
    let (verdict, seam) = judged(&rule, &tied);
    match &verdict {
        SolveVerdict::Refused(r) => assert_eq!(r.reason, crate::verdict::REFUSE_RULE_TIE, "by name: {r:?}"),
        other => panic!("the court refuses the tie: {other:?}"),
    }
    assert!(matches!(seam, Err(NoRule::Refused(_))), "the seam agrees: {seam:?}");
}

/// A PREDICATE ON A COORDINATE THAT IS NOT EXACT IS REFUSED BY NAME: a μ
/// lane is never a rule's coordinate, and a radius is read only where it is
/// a rational. At the seam, and at the court.
#[test]
fn sentence_refuses_inexact_coordinate() {
    let g = |rows: &[&str]| -> Grid { rows.iter().map(|r| r.bytes().map(|b| b - b'0').collect()).collect() };
    let family = vec![(g(&["110", "000", "000"]), g(&["220", "000", "000"]))];
    let lane = sentence(&[Step::Select(Pred::count(Coord::Lane(0), 1)), Step::Hue(vec![(1, 2)])], &family, &NoCrystal);
    assert!(matches!(lane, Err(NoRule::Unstated(Unstated::Inexact))), "a μ lane: {lane:?}");
    // A two-cell bar's dual sphere has r² = 5/12 + …, no rational square.
    let radius =
        sentence(&[Step::Select(Pred::count(Coord::Radius, 1)), Step::Hue(vec![(1, 2)])], &family, &NoCrystal);
    assert!(matches!(radius, Err(NoRule::Unstated(Unstated::Inexact))), "a radius no rational: {radius:?}");
    // At the court: a rule that reads the radius only of a body of hue 3 —
    // absent from its train — judged on a pair that carries one.
    let rule = sentence(
        &[
            Step::Select(Pred::Hue(1)),
            Step::Hue(vec![(1, 2)]),
            Step::Select(Pred::And(vec![Pred::Hue(3), Pred::count(Coord::Radius, 1)])),
            Step::Hue(vec![(3, 4)]),
        ],
        &family,
        &NoCrystal,
    )
    .expect("the radius is read on no body of the train");
    let withheld = vec![(g(&["110", "000", "033"]), g(&["220", "000", "033"]))];
    let (verdict, seam) = judged(&rule, &withheld);
    match &verdict {
        SolveVerdict::Refused(r) => assert_eq!(r.reason, crate::verdict::REFUSE_RULE_INEXACT, "by name: {r:?}"),
        other => panic!("the court refuses the inexact read: {other:?}"),
    }
    assert!(matches!(seam, Err(NoRule::Refused(_))), "the seam agrees: {seam:?}");
}
