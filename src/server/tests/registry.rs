use crate::arc::{Grid, Task};
use crate::server::registry::*;

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
    let address = task_address(&wide).expect("the wide task has an address");
    assert!(address.as_bytes().len() > usize::from(u16::MAX), "the fixture passes the wall: {} bytes", address.as_bytes().len());
    assert_eq!(id, citation(&address), "the book names it by its citation");
    assert_eq!(reg.close(&id), Some(Closure::First));
    let snap = reg.snapshot().expect("a book holding a task wider than the wall still snapshots");
    let mut reborn = TaskRegistry::new();
    reborn.register(toy_task());
    reborn.register(wide);
    assert_eq!(reborn.overlay(&snap), 1, "its settled state restored, by citation");
    assert_eq!(reborn.close(&id), Some(Closure::Again), "and the restart does not re-open it");
}

/// LAW 2 — AN IDENTITY IS NEVER FORMED WITH A MEMBER SKIPPED: a train pair
/// with no enclosure (a ragged grid) refuses the task's address, where it
/// would be passed over and the task named by its other pairs alone.
#[test]
fn task_address_refuses_ragged_pair() {
    let mut task = toy_task();
    assert!(task_address(&task).is_some(), "every member has an enclosure");
    task.train.push((vec![vec![1, 1], vec![1]], vec![vec![2, 2], vec![2, 2]]));
    assert!(task_address(&task).is_none(), "a member with no enclosure leaves no identity");
}

/// MU-DERIVATIONS D3.2, O3 — A PAIR'S MEMBER IS BOTH FRAMES' FULL CONTENT
/// WITH THE FOUR PERIODS. `[[1]] → [[2, 2]]` and `[[1, 1]] → [[2, 2]]`
/// read one coboundary on `K_lcm`, `{(0,0,1,2), (0,1,1,2)}`, and neither
/// wrote its frame (one ratio anisotropic, one the identity), so two tasks
/// differing only there shared an address. The two input periods differ,
/// so the two contents do.
#[test]
fn task_address_separates_isotropic_and_anisotropic_pairs() {
    let with = |input: Grid| Task { train: vec![(input, vec![vec![2, 2]])], test: Vec::new() };
    let anisotropic = task_address(&with(vec![vec![1]])).expect("addressed");
    let planar = task_address(&with(vec![vec![1, 1]])).expect("addressed");
    assert!(anisotropic != planar, "the two pairs differ in one period, so the two tasks do");
    // And one change on different ground is different content.
    let on = |ground: u8| Task { train: vec![(vec![vec![1, ground]], vec![vec![2, ground]])], test: Vec::new() };
    assert!(task_address(&on(0)).expect("addressed") != task_address(&on(3)).expect("addressed"));
}

/// MU-DERIVATIONS D3.2 — TWO TASKS DIFFERING ONLY IN A 29×29 → 30×30 PAIR
/// ARE TWO TASKS: the pair is a member of each address, never passed over
/// for its periods' lcm (870 per axis).
#[test]
fn task_address_separates_tasks_differing_in_incommensurate_pair() {
    let coprime = |hue: u8| -> (Grid, Grid) {
        let mut out: Grid = vec![vec![1; 30]; 30];
        out[0][0] = hue;
        (vec![vec![1; 29]; 29], out)
    };
    let with = |hue: u8| {
        let mut task = toy_task();
        task.train.push(coprime(hue));
        task
    };
    let two = task_address(&with(2)).expect("a 29×29 → 30×30 pair is addressed");
    let three = task_address(&with(3)).expect("a 29×29 → 30×30 pair is addressed");
    assert!(two != three, "the tasks differ in one member, so their addresses do");
    let without = task_address(&toy_task()).expect("the toy task is addressed");
    assert!(two != without, "the pair is a member of the address, never passed over");
}

/// A task of many busy frames has a content address of tens of kilobytes
/// (every band of every frame is written), and an index copies the address
/// it is keyed by. The book keys and holds the task by its 32-byte citation;
/// the address is recomputed from the task where its coordinates are
/// needed, never held.
fn busy_task() -> Task {
    // Each row changes hue at every column and no row repeats the one
    // above, so every site of each 30 × 30 frame is its own band.
    let busy = |s: u8| -> Grid { (0..30).map(|r| (0..30).map(|c| u8::try_from((r * 7 + c * 3 + usize::from(s)) % 9).expect("a residue mod 9") + 1).collect()).collect() };
    let mut task = toy_task();
    task.train.extend((0..12u8).map(|s| (busy(s), busy(s + 1))));
    task
}

#[test]
fn registry_indexes_by_citation_not_address() {
    let task = busy_task();
    let address = task_address(&task).expect("the busy pairs are addressed");
    assert!(address.as_bytes().len() > usize::from(u16::MAX), "the fixture's address passes 64 KiB: {} bytes", address.as_bytes().len());
    let mut reg = TaskRegistry::new();
    let id = reg.register(task).expect("registers");
    assert_eq!(id, citation(&address));
    assert_eq!(reg.index_keys(), vec![id.to_vec()], "the index holds the 32-byte citation and nothing wider");
    assert_eq!(reg.dispatch().map(|(d, _)| d), Some(id), "dispatch names it by the citation");
}

/// THE CITATION IS THE TASK'S, NOT THE BOOK'S: registered, snapshotted,
/// re-registered into a fresh book and overlaid, the task keeps the citation
/// it was registered under, and the citation read off the task agrees.
#[test]
fn citation_of_task_is_stable_across_registry_round_trip() {
    let mut reg = TaskRegistry::new();
    let id = reg.register(toy_task()).expect("registers");
    assert_eq!(reg.close(&id), Some(Closure::First));
    let snap = reg.snapshot().expect("writes");
    let mut reborn = TaskRegistry::new();
    let again = reborn.register(toy_task()).expect("registers");
    assert_eq!(again, id, "the same content, the same citation");
    assert_eq!(task_citation(&toy_task()), Some(id), "read off the task alone");
    assert_eq!(reborn.overlay(&snap), 1);
    assert_eq!(reborn.open(&id), Some(false), "its state comes back under the citation");
    assert_eq!(reborn.settled_ids(), vec![id]);
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
        let ids: Vec<TaskCitation> = [toy_task(), other.clone(), third.clone()]
            .into_iter()
            .map(|t| reg.register(t).expect("a grid pair's chain is A5's"))
            .collect();
        (reg, ids)
    };
    let (mut reg, ids) = book();
    assert_eq!(ids.len(), 3);
    assert_eq!(reg.dispatch().map(|(id, _)| id), Some(ids[0]));
    assert_eq!(reg.cursor_due(2), None, "one dispatch is not a page of two");
    assert_eq!(reg.dispatch().map(|(id, _)| id), Some(ids[1]));
    let record = reg.cursor_due(2).expect("a page of two dispatches passed");
    assert_eq!(&record[..4], b"CURS");
    assert_eq!(record[4..8], 2u32.to_le_bytes(), "the cursor stands on the third task");
    assert_eq!(record[8..], 3u32.to_le_bytes(), "of a book of three");
    assert_eq!(reg.cursor_due(2), None, "the count begins again");
    assert!(reg.snapshot().is_some() && reg.settled_ids().is_empty(), "nothing closed");

    // THE RESTART: the corpus re-registered, the record restored.
    let (mut reborn, _) = book();
    assert!(reborn.restore_cursor(&record), "the record restores");
    assert_eq!(reborn.dispatch().map(|(id, _)| id), Some(ids[2]), "the next task, not the head");
    let (mut fresh, _) = book();
    assert_eq!(fresh.dispatch().map(|(id, _)| id), Some(ids[0]), "with no record, the head");

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
