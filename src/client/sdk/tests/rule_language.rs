// 78.143 — THE RULE LANGUAGE GROWS: real ARC-AGI-2 TRAINING tasks whose
// rules need a selection, a region, a creation, a count or a comparison,
// each stated as Steps, minted by `sentence` over the task's train pairs,
// and SOLVED by the real `Kernel::judge_solve` on the task's own test pair,
// which the rule never saw — `would_judge` agreeing. Refusals by name: an
// unnamed tie and a predicate on a coordinate that is not exact, at the seam
// and at the court.

use crate::arc::Grid;
use crate::client::sdk::rule::{
    sentence, would_judge, Coord, Create, Extent, NoRule, Order, Paint, Pred, Region, Step, Stop, Tie, Unstated,
};
use crate::cognition::{Kernel, SolveVerdict};
use crate::transducer::element::Element;
use crate::transducer::family::NoCrystal;

type Pairs = Vec<(Grid, Grid)>;

fn grids(v: &serde_json::Value) -> Pairs {
    let grid = |g: &serde_json::Value| -> Grid {
        g.as_array()
            .expect("rows")
            .iter()
            .map(|r| r.as_array().expect("a row").iter().map(|c| u8::try_from(c.as_u64().expect("a hue")).expect("a hue")).collect())
            .collect()
    };
    v.as_array().expect("pairs").iter().map(|p| (grid(&p["input"]), grid(&p["output"]))).collect()
}

/// The task as written: its train pairs and its test pairs.
fn task(id: &str) -> (Pairs, Pairs) {
    let text = std::fs::read_to_string(format!("{}/{id}.json", crate::arc::corpus("training"))).expect("the corpus holds the task");
    let v: serde_json::Value = serde_json::from_str(&text).expect("the task parses");
    (grids(&v["train"]), grids(&v["test"]))
}

fn judged(rule: &Element, withheld: &[(Grid, Grid)]) -> (SolveVerdict, Result<(), NoRule>) {
    let mut k = Kernel::new();
    let verdict = k.judge_solve(rule, withheld);
    let court: &Kernel = &k;
    let seam = would_judge(rule, withheld, &|a| court.resolve_form(a), court);
    (verdict, seam)
}

/// THE TASK SOLVED: the steps mint over the train pairs, and the court
/// solves the minted word on each test pair it never saw.
fn solves(id: &str, steps: &[Step]) {
    let (train, test) = task(id);
    let rule = match sentence(steps, &train, &NoCrystal) {
        Ok(rule) => rule,
        Err(e) => panic!("{id}: the rule mints over its train pairs: {e:?}"),
    };
    for (i, pair) in test.iter().enumerate() {
        let (verdict, seam) = judged(&rule, std::slice::from_ref(pair));
        assert!(matches!(verdict, SolveVerdict::Solved { .. }), "{id} test {i}: the court solves it: {verdict:?}");
        assert!(seam.is_ok(), "{id} test {i}: the seam agrees: {seam:?}");
    }
}

// ---- SLICE 1: CREATE, REGION and SIZED OUTPUTS, with SELECT as they need it.

/// 00d62c1b — every background region the shapes enclose is filled yellow.
#[test]
fn fill_the_enclosed_regions_00d62c1b() {
    solves("00d62c1b", &[Step::Select(Pred::All), Step::Region(Region::Interior), Step::Hue(vec![(0, 4)])]);
}

/// a5313dff — the same fill in blue, a body inside the enclosure on its rim.
#[test]
fn fill_the_enclosed_regions_a5313dff() {
    solves("a5313dff", &[Step::Select(Pred::All), Step::Region(Region::Interior), Step::Hue(vec![(0, 1)])]);
}

/// 178fcbfb — red seeds extend to the border along their column; blue and
/// green seeds along their row, over the red.
#[test]
fn extend_lines_to_the_border_178fcbfb() {
    let line = |dr: i8, dc: i8| Step::Create(Create::Extend { ray: (dr, dc), paint: Paint::Own, stop: Stop::Border });
    solves(
        "178fcbfb",
        &[
            Step::Select(Pred::Hue(2)),
            line(-1, 0),
            line(1, 0),
            Step::Select(Pred::Any(vec![Pred::Hue(1), Pred::Hue(3)])),
            line(0, -1),
            line(0, 1),
        ],
    );
}

/// 88a10436 — the one many-hued body is copied, centre on centre, onto the
/// grey seed, which it replaces.
#[test]
fn copy_the_template_onto_the_seed_88a10436() {
    solves(
        "88a10436",
        &[Step::Select(Pred::Hue(5)), Step::Create(Create::Copy { template: Pred::Not(Box::new(Pred::Hue(5))) })],
    );
}

/// 1cf80156 — the output is the extent of the one body.
#[test]
fn crop_to_the_one_body_1cf80156() {
    solves("1cf80156", &[Step::Select(Pred::All), Step::Crop(Extent::Body)]);
}

/// c59eb873 — every cell becomes a 2 × 2 block. The clause SPEAKS every
/// pair exactly; but a scaling alone is a refinement of the pair (`refine`),
/// which the economy measures as no deflation, so it mints no word: a zero
/// μ⃗ is not a word, refused by the mint's own name (a reading, not a fault
/// of the clause).
#[test]
fn a_scale_alone_speaks_every_pair_and_mints_no_word_c59eb873() {
    use crate::transducer::sentence::form::clause::{Act, Clause};
    let (train, test) = task("c59eb873");
    let clauses = [Clause { select: Pred::All, act: Act::Scale { up: 2, down: 1 } }];
    for (x, y) in train.iter().chain(&test) {
        let spoken = crate::transducer::family::statement::speak_clauses(&clauses, x, &crate::transducer::family::perceive(x))
            .expect("stated");
        let mut out = vec![vec![0u8; spoken.frame.1]; spoken.frame.0];
        for &(r, c, h) in &spoken.painted {
            out[usize::try_from(r).expect("a row")][usize::try_from(c).expect("a column")] = h;
        }
        assert_eq!(&out, y, "the scaled grid is the output");
    }
    let got = sentence(&[Step::Scale { up: 2, down: 1 }], &train, &NoCrystal);
    assert!(
        matches!(got, Err(NoRule::Mint(crate::transducer::sentence::Refused::MintNoTape))),
        "a refinement measures no μ and mints nothing: {got:?}"
    );
}

/// A crop with no single body to keep is refused by name.
#[test]
fn a_crop_of_two_bodies_is_refused_by_name() {
    let (train, _) = task("be94b721");
    let got = sentence(&[Step::Select(Pred::All), Step::Crop(Extent::Body)], &train, &NoCrystal);
    assert!(matches!(got, Err(NoRule::Unstated(Unstated::NotOne))), "refused by name: {got:?}");
}

/// The clauses round-trip through the address the court decodes: the word
/// the seam mints is read back by the decoder with every clause.
#[test]
fn the_clauses_ride_the_address() {
    let (train, _) = task("88a10436");
    let steps = [Step::Select(Pred::Hue(5)), Step::Create(Create::Copy { template: Pred::Not(Box::new(Pred::Hue(5))) })];
    let rule = sentence(&steps, &train, &NoCrystal).expect("mints");
    let form = crate::transducer::sentence::from_address(&rule.address()).expect("the address decodes");
    assert_eq!(form.elements.len(), 1, "one clause, read back");
    assert_eq!(crate::transducer::sentence::form_vertex_key(&form), rule.address(), "and written again it is the same address");
}

// ---- SLICE 2: COUNT and COMPARE (Concordance 1.10), in an order the rule
// names, ties included.

fn erase() -> Vec<(u8, u8)> {
    (1..=9u8).map(|h| (h, 0)).collect()
}

/// be94b721 — the output is the extent of the LARGEST body; a tie refused.
#[test]
fn crop_to_the_largest_body_be94b721() {
    solves("be94b721", &[Step::Select(Pred::extremum(Coord::Shell, Order::Greatest, Tie::Refuse)), Step::Crop(Extent::Body)]);
}

/// 6e82a1ae — a body of 4 cells turns blue, of 3 red, of 2 green: exact counts.
#[test]
fn recolour_by_exact_size_6e82a1ae() {
    solves(
        "6e82a1ae",
        &[
            Step::Select(Pred::count(Coord::Shell, 4)),
            Step::Hue(vec![(5, 1)]),
            Step::Select(Pred::count(Coord::Shell, 3)),
            Step::Hue(vec![(5, 2)]),
            Step::Select(Pred::count(Coord::Shell, 2)),
            Step::Hue(vec![(5, 3)]),
        ],
    );
}

/// d2abd087 — a body of six cells turns red, every other blue.
#[test]
fn recolour_by_a_count_and_its_complement_d2abd087() {
    let six = Pred::count(Coord::Shell, 6);
    solves(
        "d2abd087",
        &[Step::Select(six.clone()), Step::Hue(vec![(5, 2)]), Step::Select(Pred::Not(Box::new(six))), Step::Hue(vec![(5, 1)])],
    );
}

/// 08ed6ac7 — the bars recoloured by the rank of their height, tallest first.
#[test]
fn recolour_by_rank_08ed6ac7() {
    let mut steps = Vec::new();
    for (place, hue) in [(0u32, 1u8), (1, 2), (2, 3), (3, 4)] {
        steps.push(Step::Select(Pred::Rank { coord: Coord::Height, order: Order::Greatest, place, tie: Tie::Refuse }));
        steps.push(Step::Hue(vec![(5, hue)]));
    }
    solves("08ed6ac7", &steps);
}

/// a61f2674 — the tallest bar blue, the shortest red, every other erased.
#[test]
fn keep_the_extremes_a61f2674() {
    let tallest = Pred::extremum(Coord::Height, Order::Greatest, Tie::Refuse);
    let shortest = Pred::extremum(Coord::Height, Order::Least, Tie::Refuse);
    solves(
        "a61f2674",
        &[
            Step::Select(tallest.clone()),
            Step::Hue(vec![(5, 1)]),
            Step::Select(shortest.clone()),
            Step::Hue(vec![(5, 2)]),
            Step::Select(Pred::Not(Box::new(Pred::Any(vec![tallest, shortest])))),
            Step::Hue(erase()),
        ],
    );
}

/// 00dbd492 — each square frame's interior filled by its width: 5 cyan,
/// 7 yellow, 9 green; the dot inside each stays inside it.
#[test]
fn fill_each_interior_by_its_frame_s_width_00dbd492() {
    let mut steps = Vec::new();
    for (w, hue) in [(5i64, 8u8), (7, 4), (9, 3)] {
        steps.push(Step::Select(Pred::count(Coord::Width, w)));
        steps.push(Step::Region(Region::Interior));
        steps.push(Step::Hue(vec![(0, hue)]));
    }
    solves("00dbd492", &steps);
}

/// 42a50994 — a lone cell that touches no other body is erased.
#[test]
fn erase_the_cells_touching_nothing_42a50994() {
    solves(
        "42a50994",
        &[Step::Select(Pred::And(vec![Pred::count(Coord::Shell, 1), Pred::count(Coord::Contacts, 0)])), Step::Hue(erase())],
    );
}

/// AN UNNAMED TIE IS REFUSED BY NAME: at the seam, over a family where two
/// bodies tie for the largest; and at the court, a rule minted where no tie
/// stood, judged on a pair where one does.
#[test]
fn an_unnamed_tie_is_refused_by_name() {
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
fn a_predicate_on_a_coordinate_that_is_not_exact_is_refused_by_name() {
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
