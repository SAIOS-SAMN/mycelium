// 78.143 — THE RULE LANGUAGE GROWS: real ARC-AGI-2 TRAINING tasks whose
// rules need a selection, a region, a creation, a count or a comparison,
// each stated as Steps, minted by `sentence` over the task's train pairs,
// and SOLVED by the real `Court::judge_solve` on the task's own test pair,
// which the rule never saw — `would_judge` agreeing. Read through the
// corpus's decoder (ROADMAP track 26, S1; moved from the core's
// `client::sdk::tests::rule_language`, whose refusals by name stay there).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use mycelium::client::sdk::rule::{
    sentence, would_judge, Coord, Create, Extent, NoRule, Order, Paint, PlaneAxis, Pred, RegionKind, Step, Stop, Tie, Unstated,
};
use mycelium::cognition::{Court, SolveVerdict};
use mycelium::transducer::element::Element;
use mycelium::transducer::family::NoCrystal;
use mycelium::transducer::spelled::SectionSample;

type Pairs = Vec<(SectionSample, SectionSample)>;

/// The task as written (the frame as the file gives it): its train pairs
/// and the test pairs whose outputs the file holds.
fn task(id: &str) -> (Pairs, Pairs) {
    let text = std::fs::read_to_string(format!("{}/{id}.json", mycelium_source_arc_agi::corpus("training"))).expect("the corpus holds the task");
    let t = mycelium_source_arc_agi::decode(&text).expect("the task parses");
    (t.train, t.test.into_iter().filter_map(|(x, y)| Some((x, y?))).collect())
}

/// The court judges the withheld pairs under the ground the claim states,
/// read against the task's train pairs (`cognition::stated_ground`), as
/// the door reads it (MU-SENSING §3.4, §6); a stated ground that is no
/// candidate is refused before any replay.
fn judged(rule: &Element, stated: Option<&[u8]>, train: &[(SectionSample, SectionSample)], withheld: &[(SectionSample, SectionSample)]) -> (SolveVerdict, Result<(), NoRule>) {
    let Ok(ground) = mycelium::cognition::stated_ground(train, stated) else {
        return (SolveVerdict::NotARule, Err(NoRule::NotAVersor));
    };
    mycelium::transducer::raster::on_ground(ground, || {
        let mut k = Court::new();
        let verdict = k.judge_solve(rule, withheld);
        let court: &Court = &k;
        let seam = would_judge(rule, withheld, &|a| court.resolve_form(a), court);
        (verdict, seam)
    })
}

/// THE TASK SOLVED: the steps are minted over the train pairs once per
/// perspective of the task's ground (`rule::on_each_perspective_of`; under
/// a vortex once per candidate), each claim stating the ground it was
/// spoken under, and the court, reconstructing under each claim's stated
/// ground, solves some claim on every test pair it never saw. No claim is
/// picked before the court reads it.
fn solves(id: &str, steps: &[Step]) {
    let (train, test) = task(id);
    let claims = mycelium::transducer::raster::on_each_perspective_of(&train, || sentence(steps, &train, &NoCrystal));
    let mut solved = Vec::new();
    for (ground, rule) in &claims {
        let Ok(rule) = rule else { continue };
        let stated = mycelium::transducer::raster::stated(ground);
        let all = test.iter().all(|pair| {
            let (verdict, seam) = judged(rule, stated.as_deref(), &train, std::slice::from_ref(pair));
            matches!(verdict, SolveVerdict::Solved { .. }) && seam.is_ok()
        });
        if all {
            solved.push(ground.clone());
        }
    }
    let read: Vec<_> = claims.iter().map(|(g, r)| (g.clone(), r.as_ref().err().cloned())).collect();
    assert!(!solved.is_empty(), "{id}: some claim, under the ground it states, is solved on every test pair: {read:?}");
}

// ---- SLICE 1: CREATE, REGION and SIZED OUTPUTS, with SELECT as they need it.

/// 00d62c1b — every background region the shapes enclose is filled yellow.
#[test]
fn sentence_fills_enclosed_regions() {
    solves("00d62c1b", &[Step::Select(Pred::All), Step::Region(RegionKind::Interior), Step::Hue(vec![(0, 4)])]);
}

/// a5313dff — the same fill in blue, a body inside the enclosure on its rim.
#[test]
fn sentence_fills_enclosure_around_inner_body() {
    solves("a5313dff", &[Step::Select(Pred::All), Step::Region(RegionKind::Interior), Step::Hue(vec![(0, 1)])]);
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
        let mut out = mycelium::transducer::spelled::SectionSample::filled(spoken.extents, 0).expect("the spoken frame is a sample");
        for p in &spoken.painted {
            out.paint(p.site, p.symbol).expect("a site of the sample");
        }
        assert_eq!(out.raster(), y.raster(), "the scaled sample is the output");
    }
    let got = sentence(&[Step::Scale { up: 2, down: 1 }], &train, &NoCrystal);
    assert!(
        matches!(got, Err(NoRule::Mint(mycelium::transducer::sentence::ObstructionRefusal::MintNoTape))),
        "a refinement measures no μ and mints nothing: {got:?}"
    );
}

/// A crop with no single body to keep is refused by name, under every
/// perspective of the task's ground.
#[test]
fn sentence_refuses_crop_of_two_bodies() {
    let (train, _) = task("be94b721");
    let got = mycelium::transducer::raster::on_each_perspective_of(&train, || {
        sentence(&[Step::Select(Pred::All), Step::Crop(Extent::Body)], &train, &NoCrystal)
    });
    assert!(got.iter().all(|(_, r)| matches!(r, Err(NoRule::Unstated(Unstated::NotOne)))), "refused by name: {got:?}");
}

/// The clauses round-trip through the address the court decodes: the word
/// the seam mints, under each perspective of the task's ground it mints
/// under, is read back by the decoder with every clause.
#[test]
fn from_address_round_trips_minted_clauses() {
    let (train, _) = task("88a10436");
    let steps = [Step::Select(Pred::Hue(5)), Step::Create(Create::Copy { template: Pred::Not(Box::new(Pred::Hue(5))) })];
    let minted: Vec<_> = mycelium::transducer::raster::on_each_perspective_of(&train, || sentence(&steps, &train, &NoCrystal))
        .into_iter()
        .filter_map(|(_, r)| r.ok())
        .collect();
    assert!(!minted.is_empty(), "the clauses mint under some perspective");
    for rule in minted {
        let form = mycelium::transducer::sentence::from_address(&rule.address()).expect("the address decodes");
        assert_eq!(form.elements.len(), 1, "one clause, read back");
        assert_eq!(mycelium::transducer::sentence::form_vertex_key(&form), rule.address(), "and written again it is the same address");
    }
}

// ---- SLICE 2: COUNT and COMPARE (Concordance 1.10), in an order the rule
// names, ties included.

fn erase() -> Vec<(u8, u8)> {
    (1..=9u8).map(|h| (h, 0)).collect()
}

/// be94b721 — the output is the extent of the LARGEST body; a tie refused.
#[test]
fn sentence_crops_to_largest_body() {
    solves("be94b721", &[Step::Select(Pred::extremum(Coord::Cells, Order::Greatest, Tie::Refuse)), Step::Crop(Extent::Body)]);
}

/// 6e82a1ae — a body of 4 cells turns blue, of 3 red, of 2 green: exact counts.
#[test]
fn sentence_recolours_by_exact_size() {
    solves(
        "6e82a1ae",
        &[
            Step::Select(Pred::count(Coord::Cells, 4)),
            Step::Hue(vec![(5, 1)]),
            Step::Select(Pred::count(Coord::Cells, 3)),
            Step::Hue(vec![(5, 2)]),
            Step::Select(Pred::count(Coord::Cells, 2)),
            Step::Hue(vec![(5, 3)]),
        ],
    );
}

/// d2abd087 — a body of six cells turns red, every other blue.
#[test]
fn sentence_recolours_by_count_and_complement() {
    let six = Pred::count(Coord::Cells, 6);
    solves(
        "d2abd087",
        &[Step::Select(six.clone()), Step::Hue(vec![(5, 2)]), Step::Select(Pred::Not(Box::new(six))), Step::Hue(vec![(5, 1)])],
    );
}

/// 08ed6ac7 — the bars recoloured by their extent along the plane's first
/// axis, the greatest first.
#[test]
fn sentence_recolours_by_rank() {
    let mut steps = Vec::new();
    for (place, hue) in [(0u32, 1u8), (1, 2), (2, 3), (3, 4)] {
        steps.push(Step::Select(Pred::Rank { coord: Coord::Extent(PlaneAxis::First), order: Order::Greatest, place, tie: Tie::Refuse }));
        steps.push(Step::Hue(vec![(5, hue)]));
    }
    solves("08ed6ac7", &steps);
}

/// a61f2674 — the bar of greatest extent along the plane's first axis blue,
/// the least red, every other erased.
#[test]
fn sentence_keeps_extremes() {
    let tallest = Pred::extremum(Coord::Extent(PlaneAxis::First), Order::Greatest, Tie::Refuse);
    let shortest = Pred::extremum(Coord::Extent(PlaneAxis::First), Order::Least, Tie::Refuse);
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

/// 00dbd492 — each square frame's interior filled by its extent along the
/// plane's second axis: 5 cyan,
/// 7 yellow, 9 green; the dot inside each stays inside it. The task reads
/// a vortex of 0 and 2 (the frames meet the seam in every sample): a claim
/// is spoken under each, and the court solves the one under 0.
#[test]
fn sentence_fills_interior_by_frame_width() {
    let mut steps = Vec::new();
    for (w, hue) in [(5i64, 8u8), (7, 4), (9, 3)] {
        steps.push(Step::Select(Pred::count(Coord::Extent(PlaneAxis::Second), w)));
        steps.push(Step::Region(RegionKind::Interior));
        steps.push(Step::Hue(vec![(0, hue)]));
    }
    solves("00dbd492", &steps);
}

/// 42a50994 — a lone cell that touches no other body is erased.
#[test]
fn sentence_erases_cells_touching_nothing() {
    solves(
        "42a50994",
        &[Step::Select(Pred::And(vec![Pred::count(Coord::Cells, 1), Pred::count(Coord::Contacts, 0)])), Step::Hue(erase())],
    );
}
