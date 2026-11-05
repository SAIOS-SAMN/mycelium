// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// mycelium#87; MU-DERIVATIONS D2.5, D3.2 — THE CHART OF `0b148d64`'S PAIRS
// AND ITS INTAKE COST, read on the metabolic budget. An observation: it
// states relations the intake and the budget hold on this task and prints
// what was read; no engine branch reads it, and no count or chart value is
// a criterion.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use mycelium::transducer::family::speak::residue_of;
use mycelium::transducer::family::transduce::{need_of, Transduction, TransductionRefusal};
use mycelium::transducer::raster::on_ground_of;
use mycelium::transducer::stalk::invariant_of;
use mycelium_source_arc_agi::{corpus, decode};

/// THE INTAKE CHARTS NONE OF `0b148d64`'S PAIRS, AND EACH PAIR IS READ
/// WITHIN THE STROKE ITS OWN NEED SIZES. Each pair's two enclosures differ,
/// so it has no route until the turn-native chart (D3.2, O8): the intake
/// charts nothing for this task. The `≥ 19 s` a pair once took was the
/// chart of the common refinement `K_lcm`, which no reader forms now.
///
/// Each pair's relation is its transduction, on a stroke of the pair's
/// closed-form need (D2.5: a stroke reads the work's own need), and it
/// spends within it. The input's turns land on no boundary of the output's
/// periods (`OffSite`), so the pair stands as the output against nothing
/// on the family's ground; that demand's chart, the need, the spend and the
/// links are printed (`--nocapture`), never asserted.
#[test]
fn chart_of_0b148d64_pairs_read_on_metabolic_budget() {
    let json = std::fs::read_to_string(format!("{}/0b148d64.json", corpus("training"))).expect("the corpus stands");
    let task = decode(&json).expect("the task decodes");
    assert!(mycelium::server::route::routes_of(&task).is_empty(), "no pair of 0b148d64 is routed");
    on_ground_of(&task.train, || {
        for (pair, (a, b)) in task.train.iter().enumerate() {
            let need = need_of(a.sample(), b.sample()).expect("both frames stand");
            let mut budget = mycelium::physics::Physics::default().stroke_n(need);
            let t = Transduction::of(a.sample(), b.sample(), &mut budget).expect("the need is the stroke");
            let spent = need.checked_sub(budget.remaining()).expect("the spend stays within the need's stroke");
            assert!(
                matches!(t.projected(), Err(TransductionRefusal::OffSite { .. })),
                "pair {pair}: the input's turns land on no output boundary"
            );
            let chain = residue_of(None, b).expect("the family reads a ground");
            let [h, w] = a.extents();
            let [h2, w2] = b.extents();
            let frame = mycelium::transducer::element::Frame::new(h, w, h2, w2);
            let word = mycelium::transducer::element::from_residue_framed(&chain, &frame).expect("a canonical chain");
            let chart = invariant_of(&word);
            eprintln!("  0b148d64 pair {pair}: {h}x{w} -> {h2}x{w2}, need {need} spent {spent}, demand {} links, chart {chart:?}", chain.len());
        }
    });
}
