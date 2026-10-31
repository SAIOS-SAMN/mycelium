// 26, mycelium#14 — A TASK'S ROUTES: read once on the intake's thread, kept
// by the task's citation, and handed to every cover change by lookup.

use crate::arc::Task;
use crate::cognition::Court;
use crate::server::registry::TaskRegistry;
use crate::server::route::{cover_walk, intake_walk, route_next};
use crate::transducer::stalk::chart::charts_read;
use std::sync::{Arc, Mutex};

fn pair_task() -> Task {
    Task {
        train: vec![
            (vec![vec![1, 1, 0], vec![0, 3, 0]], vec![vec![2, 2, 0], vec![0, 3, 0]]),
            (vec![vec![0, 1], vec![1, 1]], vec![vec![0, 2], vec![2, 2]]),
        ],
        test: vec![(vec![vec![0, 0], vec![1, 1]], vec![vec![0, 0], vec![2, 2]])],
    }
}

/// A COVER CHANGE RE-HOMES BY LOOKUP: the walk `relocate_corpus` takes
/// hands out the routes the intake read — each pair's address, chart,
/// magnitude and pieces — so a second relocation over an unchanged
/// registry charts nothing, and re-homing on those routes (the listing
/// step, the relocation onto the cavity) charts nothing either. A task the
/// intake has not routed is left to it, never read on the cover's walk.
#[test]
fn relocate_corpus_recomputes_nothing_for_unchanged_registry() {
    let registry = Mutex::new(TaskRegistry::new());
    let id = registry
        .lock()
        .expect("a fresh lock")
        .register(pair_task())
        .expect("a grid pair's chain is A5's");
    let before = charts_read();
    assert!(cover_walk(&registry, 8).is_empty(), "an unrouted task is left to the intake");
    assert_eq!(charts_read(), before, "the cover's walk read no chart");
    let intake = intake_walk(&registry, 8);
    let [walked] = intake.as_slice() else { panic!("one task in the manifest") };
    assert_eq!(walked.citation, id);
    assert_eq!(walked.routes.len(), 2, "each pair is routed");
    assert!(charts_read() > before, "the intake read the charts");
    let mut k = Court::new();
    for r in walked.routes.iter() {
        k.intake_prepared(&r.pieces);
    }
    let first = cover_walk(&registry, 8);
    let before = charts_read();
    let second = cover_walk(&registry, 8);
    let [again] = second.as_slice() else { panic!("the cover walks the one task") };
    assert!(Arc::ptr_eq(&first[0].routes, &again.routes), "the routes kept are the routes handed out");
    for r in again.routes.iter() {
        k.intake_prepared(&r.pieces);
        for (piece, _) in &r.pieces {
            let address = piece.address();
            if k.has_listing(address.as_bytes()) {
                k.relocate_listing(address.as_bytes());
            }
        }
    }
    assert_eq!(charts_read(), before, "a second relocation recomputes no chart");
    assert!(route_next(&registry).is_none(), "every task is routed");
}
