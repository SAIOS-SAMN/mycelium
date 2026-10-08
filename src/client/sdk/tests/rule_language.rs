// 78.143 — THE RULE LANGUAGE GROWS: real ARC-AGI-2 TRAINING tasks whose
// rules need a selection, a region, a creation, a count or a comparison,
// each stated as Steps, minted by `sentence` over the task's train pairs,
// and SOLVED by the real `Kernel::judge_solve` on the task's own test pair,
// which the rule never saw — `would_judge` agreeing. Refusals by name: an
// unnamed tie and a predicate on a coordinate that is not exact, at the seam
// and at the court.

use crate::arc::Grid;
use crate::client::sdk::rule::{
    sentence, would_judge, Create, Extent, NoRule, Paint, Pred, Region, Step, Stop, Unstated,
};
use crate::cognition::{Kernel, SolveVerdict};
use crate::transducer::element::Element;
use crate::transducer::family::NoCrystal;

const CORPUS: &str = "/opt/mycelium-v0.1.0/data/arc-agi-2/training";

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
    let text = std::fs::read_to_string(format!("{CORPUS}/{id}.json")).expect("the corpus holds the task");
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
    assert_eq!(form.clauses.len(), 1, "one clause, read back");
    assert_eq!(crate::transducer::sentence::form_vertex_key(&form), rule.address(), "and written again it is the same address");
}
