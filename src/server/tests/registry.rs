use crate::arc::{Grid, Task};
use crate::server::registry::*;
use crate::transducer::address::Address;

fn toy_task() -> Task {
    Task {
        train: vec![(vec![vec![1, 1], vec![0, 0]], vec![vec![2, 2], vec![0, 0]])],
        test: vec![(vec![vec![0, 0], vec![1, 1]], vec![vec![0, 0], vec![2, 2]])],
    }
}

/// THE REGISTRY RECORDS, NEVER JUDGES: identity is content
/// (re-registration is idempotent); the first ruled closure closes the
/// task, later ones are counted again; a task this book does not hold has
/// no closure.
#[test]
fn close_records_first_then_again() {
    let mut reg = TaskRegistry::new();
    let id = reg.register(toy_task()).expect("a grid pair's chain is A5's");
    assert_eq!(
        reg.register(toy_task()).expect("a grid pair's chain is A5's"),
        id,
        "content is identity — no re-feeding"
    );
    assert_eq!(reg.len(), 1);
    assert_eq!(reg.open(&id), Some(true), "open before any closure");
    assert_eq!(reg.close(&id), Some(Closure::First), "the first ruled closure");
    assert_eq!(reg.open(&id), Some(false));
    assert_eq!(reg.close(&id), Some(Closure::Again), "a closure of a closed task");
    assert_eq!(reg.close(&[9u8; 32]), None, "no origin, no closure");
    assert_eq!(reg.unsolved(), 0);
}

/// A restart must not resurrect settled bounties: snapshot → fresh
/// registry (corpus re-registered) → overlay → the replay still
/// collapses to the floor.
#[test]
fn overlay_restores_settled_states_after_restart() {
    let mut reg = TaskRegistry::new();
    let id = reg.register(toy_task()).expect("a grid pair's chain is A5's");
    assert_eq!(reg.close(&id), Some(Closure::First));
    let snap = reg.snapshot().expect("the snapshot writes");

    let mut reborn = TaskRegistry::new();
    reborn.register(toy_task()); // corpus re-registration at boot
    assert_eq!(reborn.overlay(&snap), 1, "one settled state restored");
    assert_eq!(reborn.close(&id), Some(Closure::Again), "the restart must not re-open the task");
    assert_eq!(reborn.overlay(&[]), 0, "garbage snapshot restores nothing");
    // STD1 — an older layout, with `u32` counts, is refused whole.
    let mut older = snap.clone();
    older[3] = b'1';
    let mut again = TaskRegistry::new();
    again.register(toy_task());
    assert_eq!(again.overlay(&older), 0, "a CUR1 snapshot is not read at the u64 width");
}

/// A TASK WIDER THAN THE U16 WALL IS SETTLED ACROSS A RESTART (observed on
/// the development deployment, 2026-10-20: 12 of the 1 000 training tasks have content
/// addresses past 65 535 bytes, and the address-keyed snapshot refused
/// the whole book, so no court ever kept a settled task). Named by its
/// citation, it snapshots, restores, and stays closed.
#[test]
fn overlay_restores_task_wider_than_u16_wall() {
    // Many pairs of busy frames: each pair's listing key is written
    // into the task's address, so the address grows with them.
    let busy = |s: u8| -> Grid { (0..30).map(|r| (0..30).map(|c| u8::try_from((r * 7 + c * 3 + usize::from(s)) % 9).expect("a residue mod 9") + 1).collect()).collect() };
    let wide = Task { train: (0..12u8).map(|s| (busy(s), busy(s + 1))).collect(), test: vec![(busy(40), Vec::new())] };
    let mut reg = TaskRegistry::new();
    reg.register(toy_task());
    let id = reg.register(wide.clone()).expect("the wide task has an identity");
    assert!(id.as_bytes().len() > usize::from(u16::MAX), "the fixture passes the wall: {} bytes", id.as_bytes().len());
    assert_eq!(reg.close(&id), Some(Closure::First));
    let snap = reg.snapshot().expect("a book holding a task wider than the wall still snapshots");
    let mut reborn = TaskRegistry::new();
    reborn.register(toy_task());
    reborn.register(wide);
    assert_eq!(reborn.overlay(&snap), 1, "its settled state restored, by citation");
    assert_eq!(reborn.close(&id), Some(Closure::Again), "and the restart does not re-open it");
}

/// Dispatch rotates over the unsolved book and dries up when settled.
#[test]
fn dispatch_rotates_and_dries_up() {
    let mut reg = TaskRegistry::new();
    let id = reg.register(toy_task()).expect("a grid pair's chain is A5's");
    let (d1, _) = reg.dispatch().expect("work exists");
    assert_eq!(d1, id);
    assert_eq!(reg.close(&id), Some(Closure::First));
    assert!(reg.dispatch().is_none(), "a settled book dispatches nothing");
}

/// 78.153c — THE DISPATCH CURSOR SURVIVES A RESTART WITH NOTHING CLOSED:
/// its record is due once a page of dispatches passed, and a registry
/// re-registered from the corpus and restored from it dispatches the task
/// after the last one served, never the head. A record of another book is
/// refused whole.
#[test]
fn restore_cursor_resumes_dispatch_after_restart() {
    let other = Task {
        train: vec![(vec![vec![3, 0], vec![0, 0]], vec![vec![0, 3], vec![0, 0]])],
        test: vec![(vec![vec![0, 0], vec![3, 0]], vec![vec![0, 0], vec![0, 3]])],
    };
    let third = Task {
        train: vec![(vec![vec![4, 4, 0]], vec![vec![0, 4, 4]])],
        test: vec![(vec![vec![0, 4, 4]], vec![vec![4, 4, 0]])],
    };
    let book = || {
        let mut reg = TaskRegistry::new();
        let ids: Vec<Address> = [toy_task(), other.clone(), third.clone()]
            .into_iter()
            .map(|t| reg.register(t).expect("a grid pair's chain is A5's"))
            .collect();
        (reg, ids)
    };
    let (mut reg, ids) = book();
    assert_eq!(ids.len(), 3);
    assert_eq!(reg.dispatch().map(|(id, _)| id), Some(ids[0].clone()));
    assert_eq!(reg.cursor_due(2), None, "one dispatch is not a page of two");
    assert_eq!(reg.dispatch().map(|(id, _)| id), Some(ids[1].clone()));
    let record = reg.cursor_due(2).expect("a page of two dispatches passed");
    assert_eq!(&record[..4], b"CURS");
    assert_eq!(record[4..8], 2u32.to_le_bytes(), "the cursor stands on the third task");
    assert_eq!(record[8..], 3u32.to_le_bytes(), "of a book of three");
    assert_eq!(reg.cursor_due(2), None, "the count begins again");
    assert!(reg.snapshot().is_some() && reg.settled_ids().is_empty(), "nothing closed");

    // THE RESTART: the corpus re-registered, the record restored.
    let (mut reborn, _) = book();
    assert!(reborn.restore_cursor(&record), "the record restores");
    assert_eq!(reborn.dispatch().map(|(id, _)| id), Some(ids[2].clone()), "the next task, not the head");
    let (mut fresh, _) = book();
    assert_eq!(fresh.dispatch().map(|(id, _)| id), Some(ids[0].clone()), "with no record, the head");

    // ANOTHER BOOK'S RECORD, OR ANOTHER LAYOUT, RESTORES NOTHING.
    let mut small = TaskRegistry::new();
    small.register(toy_task());
    assert!(!small.restore_cursor(&record), "a book of one is not a book of three");
    assert!(!small.restore_cursor(&record[..8]), "a short record is refused whole");
    let mut wrong = record;
    wrong[0] = b'X';
    let (mut again, _) = book();
    assert!(!again.restore_cursor(&wrong), "another layout is refused");
}

/// First-time intake walks the manifest once under a budget; further
/// calls are empty. Cover-change batches still wrap.
#[test]
fn next_intake_batch_walks_manifest_once() {
    let mut reg = TaskRegistry::new();
    let id = reg.register(toy_task()).expect("a grid pair's chain is A5's");
    let first = reg.next_intake_batch(8);
    assert_eq!(first.len(), 1);
    assert_eq!(first[0].0, id);
    assert!(reg.intake_done());
    assert!(reg.next_intake_batch(8).is_empty(), "the manifest is exhausted");
    assert_eq!(reg.next_cover_batch(1).len(), 1, "cover re-home still walks");
}
