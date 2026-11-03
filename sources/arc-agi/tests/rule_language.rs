// 78.143 — THE RULE LANGUAGE GROWS: real ARC-AGI-2 TRAINING tasks whose
// rules need a selection, a region, a creation, a count or a comparison,
// each stated as Steps, minted by `sentence` over the task's train pairs,
// and SOLVED by the real `Court::judge_solve` on the task's own test pair,
// which the rule never saw — `would_judge` agreeing. Read through the
// corpus's decoder (ROADMAP track 26, S1; moved from the core's
// `client::sdk::tests::rule_language`, whose refusals by name stay there).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use mycelium::client::sdk::rule::{
    sentence, would_judge, Coord, Create, Extent, NoRule, Order, Paint, Pred, Region, Step, Stop, Tie, Unstated,
};
use mycelium::cognition::{Court, SolveVerdict};
use mycelium::transducer::element::Element;
use mycelium::transducer::family::NoCrystal;
use mycelium_source_arc_agi::Grid;

type Pairs = Vec<(Grid, Grid)>;

/// The task as written (the frame as the file gives it): its train pairs
/// and its test pairs.
fn task(id: &str) -> (Pairs, Pairs) {
    let text = std::fs::read_to_string(format!("{}/{id}.json", mycelium_source_arc_agi::corpus("training"))).expect("the corpus holds the task");
    let t = mycelium_source_arc_agi::decode(&text).expect("the task parses");
    (t.train, t.test)
}

/// The court judges the withheld pairs on the ground the task's train pairs
/// name, as the door opens it (MU-SENSING §3.4), never on one the withheld
/// pairs would name alone.
fn judged(rule: &Element, withheld: &[(Grid, Grid)]) -> (SolveVerdict, Result<(), NoRule>) {
    let mut k = Court::new();
    let verdict = k.judge_solve(rule, withheld);
    let court: &Court = &k;
    let seam = would_judge(rule, withheld, &|a| court.resolve_form(a), court);
    (verdict, seam)
}

/// THE TASK SOLVED: the steps mint over the train pairs, and the court
/// solves the minted word on each test pair it never saw, both on the
/// ground the train pairs name (`raster::ground_of`).
fn solves(id: &str, steps: &[Step]) {
    let (train, _) = task(id);
    solves_on(id, mycelium::transducer::raster::ground_of(&train), steps);
}

/// [`solves`] on a ground the claim states. Used where the task's deduced
/// reading is a vortex or no ground (MU-SENSING §3.4, §8): the claim is the
/// rule language's on the canvas the task is drawn on, and the task read
/// end to end on its deduced ground refuses until a vortex or an absent
/// ground is read per perspective (track 26, S6).
fn solves_on(id: &str, ground: mycelium::transducer::family::transduce::Ground<u8>, steps: &[Step]) {
    let (train, test) = task(id);
    mycelium::transducer::raster::on_ground(ground, || {
        let rule = match sentence(steps, &train, &NoCrystal) {
            Ok(rule) => rule,
            Err(e) => panic!("{id}: the rule mints over its train pairs: {e:?}"),
        };
        for (i, pair) in test.iter().enumerate() {
            let (verdict, seam) = judged(&rule, std::slice::from_ref(pair));
            assert!(matches!(verdict, SolveVerdict::Solved { .. }), "{id} test {i}: the court solves it: {verdict:?}");
            assert!(seam.is_ok(), "{id} test {i}: the seam agrees: {seam:?}");
        }
    });
}

/// The canvas a task is drawn on, stated by the claim.
fn canvas(symbol: u8) -> mycelium::transducer::family::transduce::Ground<u8> {
    mycelium::transducer::family::transduce::Ground::One(symbol)
}

// ---- SLICE 1: CREATE, REGION and SIZED OUTPUTS, with SELECT as they need it.

/// 00d62c1b — every background region the shapes enclose is filled yellow.
#[test]
fn sentence_fills_enclosed_regions() {
    solves("00d62c1b", &[Step::Select(Pred::All), Step::Region(Region::Interior), Step::Hue(vec![(0, 4)])]);
}

/// a5313dff — the same fill in blue, a body inside the enclosure on its rim.
#[test]
fn sentence_fills_enclosure_around_inner_body() {
    solves("a5313dff", &[Step::Select(Pred::All), Step::Region(Region::Interior), Step::Hue(vec![(0, 1)])]);
}

/// 178fcbfb — red seeds extend to the border, each along its column; blue and
/// green seeds, each along its row, over the red.
#[test]
fn sentence_extends_lines_to_border() {
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
fn sentence_copies_template_onto_seed() {
    solves(
        "88a10436",
        &[Step::Select(Pred::Hue(5)), Step::Create(Create::Copy { template: Pred::Not(Box::new(Pred::Hue(5))) })],
    );
}

/// 1cf80156 — the output is the extent of the one body.
#[test]
fn sentence_crops_to_one_body() {
    solves("1cf80156", &[Step::Select(Pred::All), Step::Crop(Extent::Body)]);
}

/// c59eb873 — every cell becomes a 2 × 2 block. The clause SPEAKS every
/// pair exactly; but a scaling alone is a refinement of the pair (`refine`),
/// which the economy measures as no deflation, so it mints no word: a zero
/// μ⃗ is not a word, refused by the mint's own name (a reading, not a fault
/// of the clause).
#[test]
fn sentence_of_mints_no_word_for_scale_alone() {
    use mycelium::transducer::sentence::form::clause::{Act, Clause};
    let (train, test) = task("c59eb873");
    let clauses = [Clause { select: Pred::All, act: Act::Scale { up: 2, down: 1 } }];
    for (x, y) in train.iter().chain(&test) {
        let spoken = mycelium::transducer::family::statement::speak_clauses(&clauses, x, &mycelium::transducer::family::perceive(x))
            .expect("stated");
        let mut out = vec![vec![0u8; spoken.frame.1]; spoken.frame.0];
        for &(r, c, h) in &spoken.painted {
            out[usize::try_from(r).expect("a row")][usize::try_from(c).expect("a column")] = h;
        }
        assert_eq!(&out, y, "the scaled grid is the output");
    }
    let got = sentence(&[Step::Scale { up: 2, down: 1 }], &train, &NoCrystal);
    assert!(
        matches!(got, Err(NoRule::Mint(mycelium::transducer::sentence::ObstructionRefusal::MintNoTape))),
        "a refinement measures no μ and mints nothing: {got:?}"
    );
}

/// A crop with no single body to keep is refused by name. be94b721 reads
/// no ground (an output shows no 0), so the claim states its canvas.
#[test]
fn sentence_refuses_crop_of_two_bodies() {
    let (train, _) = task("be94b721");
    let got = mycelium::transducer::raster::on_ground(canvas(0), || {
        sentence(&[Step::Select(Pred::All), Step::Crop(Extent::Body)], &train, &NoCrystal)
    });
    assert!(matches!(got, Err(NoRule::Unstated(Unstated::NotOne))), "refused by name: {got:?}");
}

/// The clauses round-trip through the address the court decodes: the word
/// the seam mints is read back by the decoder with every clause.
#[test]
fn from_address_round_trips_minted_clauses() {
    let (train, _) = task("88a10436");
    let steps = [Step::Select(Pred::Hue(5)), Step::Create(Create::Copy { template: Pred::Not(Box::new(Pred::Hue(5))) })];
    let rule = sentence(&steps, &train, &NoCrystal).expect("mints");
    let form = mycelium::transducer::sentence::from_address(&rule.address()).expect("the address decodes");
    assert_eq!(form.elements.len(), 1, "one clause, read back");
    assert_eq!(mycelium::transducer::sentence::form_vertex_key(&form), rule.address(), "and written again it is the same address");
}

// ---- SLICE 2: COUNT and COMPARE (Concordance 1.10), in an order the rule
// names, ties included.

fn erase() -> Vec<(u8, u8)> {
    (1..=9u8).map(|h| (h, 0)).collect()
}

/// be94b721 — the output is the extent of the LARGEST body; a tie refused.
/// The task reads no ground (an output shows no 0): stated on its canvas.
#[test]
fn sentence_crops_to_largest_body() {
    solves_on("be94b721", canvas(0), &[Step::Select(Pred::extremum(Coord::Shell, Order::Greatest, Tie::Refuse)), Step::Crop(Extent::Body)]);
}

/// 6e82a1ae — a body of 4 cells turns blue, of 3 red, of 2 green: exact counts.
#[test]
fn sentence_recolours_by_exact_size() {
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
fn sentence_recolours_by_count_and_complement() {
    let six = Pred::count(Coord::Shell, 6);
    solves(
        "d2abd087",
        &[Step::Select(six.clone()), Step::Hue(vec![(5, 2)]), Step::Select(Pred::Not(Box::new(six))), Step::Hue(vec![(5, 1)])],
    );
}

/// 08ed6ac7 — the bars recoloured by height rank, tallest first.
#[test]
fn sentence_recolours_by_rank() {
    let mut steps = Vec::new();
    for (place, hue) in [(0u32, 1u8), (1, 2), (2, 3), (3, 4)] {
        steps.push(Step::Select(Pred::Rank { coord: Coord::Height, order: Order::Greatest, place, tie: Tie::Refuse }));
        steps.push(Step::Hue(vec![(5, hue)]));
    }
    solves("08ed6ac7", &steps);
}

/// a61f2674 — the tallest bar blue, the shortest red, every other erased.
#[test]
fn sentence_keeps_extremes() {
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
/// 7 yellow, 9 green; the dot inside each stays inside it. The task reads
/// a vortex of 0 and 2 (the frames meet the seam in every sample): stated
/// on its canvas.
#[test]
fn sentence_fills_interior_by_frame_width() {
    let mut steps = Vec::new();
    for (w, hue) in [(5i64, 8u8), (7, 4), (9, 3)] {
        steps.push(Step::Select(Pred::count(Coord::Width, w)));
        steps.push(Step::Region(Region::Interior));
        steps.push(Step::Hue(vec![(0, hue)]));
    }
    solves_on("00dbd492", canvas(0), &steps);
}

/// 42a50994 — a lone cell that touches no other body is erased.
#[test]
fn sentence_erases_cells_touching_nothing() {
    solves(
        "42a50994",
        &[Step::Select(Pred::And(vec![Pred::count(Coord::Shell, 1), Pred::count(Coord::Contacts, 0)])), Step::Hue(erase())],
    );
}
