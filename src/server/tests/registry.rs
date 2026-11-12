use crate::transducer::family::{task_of_lines, SectionSample, Task};
use crate::server::registry::*;

fn toy_task() -> Task {
    task_of_lines(
        vec![(vec![vec![1, 1], vec![0, 0]], vec![vec![2, 2], vec![0, 0]])],
        vec![(vec![vec![0, 0], vec![1, 1]], vec![vec![0, 0], vec![2, 2]])],
    )
}

/// THE REGISTRY RECORDS, NEVER JUDGES: identity is content
/// (re-registration is idempotent); the first ruled closure closes the
/// task, later ones are counted again; a task this book does not hold has
/// no closure.
#[test]
fn close_records_first_then_again() {
    let mut reg = TaskRegistry::new();
    let id = reg.register(toy_task()).expect("a sample pair's chain is A5's");
    assert_eq!(
        reg.register(toy_task()).expect("a sample pair's chain is A5's"),
        id,
        "content is identity — no re-feeding"
    );
    assert_eq!(reg.len(), 1);
    assert_eq!(reg.open(&id), Some(true), "open before any closure");
    assert_eq!(reg.close(&id), Some(ClosureOrder::First), "the first ruled closure");
    assert_eq!(reg.open(&id), Some(false));
    assert_eq!(reg.close(&id), Some(ClosureOrder::Again), "a closure of a closed task");
    assert_eq!(reg.close(&[9u8; 32]), None, "no origin, no closure");
    assert_eq!(reg.unsolved(), 0);
}

/// A restart must not resurrect settled bounties: snapshot → fresh
/// registry (corpus re-registered) → overlay → the reconstruction still
/// collapses to the floor.
#[test]
fn overlay_restores_settled_states_after_restart() {
    let mut reg = TaskRegistry::new();
    let id = reg.register(toy_task()).expect("a sample pair's chain is A5's");
    assert_eq!(reg.close(&id), Some(ClosureOrder::First));
    let snap = reg.snapshot().expect("the snapshot writes");

    let mut reborn = TaskRegistry::new();
    reborn.register(toy_task()); // corpus re-registration at boot
    assert_eq!(reborn.overlay(&snap), 1, "one settled state restored");
    assert_eq!(reborn.close(&id), Some(ClosureOrder::Again), "the restart must not re-open the task");
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
    // Many pairs of busy frames: each sample's content is written into the
    // task's address, so the address grows with them (40 pairs of 30×30
    // frames pass the wall).
    let busy = |s: u8| -> SectionSample { (0..30).map(|r| (0..30).map(|c| u8::try_from((r * 7 + c * 3 + usize::from(s)) % 9).expect("a residue mod 9") + 1).collect()).collect() };
    let wide = Task { train: (0..40u8).map(|s| (busy(s), busy(s + 1))).collect(), test: vec![(busy(41), None)] };
    let mut reg = TaskRegistry::new();
    reg.register(toy_task());
    let id = reg.register(wide.clone()).expect("the wide task has an identity");
    let address = task_address(&wide).expect("the wide task has an address");
    assert!(address.as_bytes().len() > usize::from(u16::MAX), "the fixture passes the wall: {} bytes", address.as_bytes().len());
    assert_eq!(id, citation(&address), "the book names it by its citation");
    assert_eq!(reg.close(&id), Some(ClosureOrder::First));
    let snap = reg.snapshot().expect("a book holding a task wider than the wall still snapshots");
    let mut reborn = TaskRegistry::new();
    reborn.register(toy_task());
    reborn.register(wide);
    assert_eq!(reborn.overlay(&snap), 1, "its settled state restored, by citation");
    assert_eq!(reborn.close(&id), Some(ClosureOrder::Again), "and the restart does not re-open it");
}

/// LAW 2 — AN IDENTITY IS NEVER FORMED WITH A MEMBER SKIPPED: a ragged raster
/// is no sample, refused by name at intake (track 26 S4d), so no task holds
/// a member that could be passed over and the task named by its other pairs
/// alone.
#[test]
fn task_address_refuses_ragged_pair() {
    use crate::transducer::sample::SampleRefusal;
    use crate::transducer::spelled::Unspelled;
    assert!(task_address(&toy_task()).is_some(), "every member has an enclosure");
    assert_eq!(SectionSample::of_lines(&[vec![1, 1], vec![1]]), Err(Unspelled::Sample(SampleRefusal::Length { expected: 4, found: 3 })), "a ragged raster is no sample");
}

/// MU-DERIVATIONS D3.2, O3 — A PAIR'S MEMBER IS BOTH DELIMITED' FULL CONTENT
/// WITH THE FOUR PERIODS. `[[1]] → [[2, 2]]` and `[[1, 1]] → [[2, 2]]`
/// read one coboundary on `K_lcm`, `{(0,0,1,2), (0,1,1,2)}`, and neither
/// wrote its frame (one ratio anisotropic, one the identity), so two tasks
/// differing only there shared an address. The two input periods differ,
/// so the two contents do.
#[test]
fn task_address_separates_isotropic_and_anisotropic_pairs() {
    let with = |input: Vec<Vec<u8>>| task_of_lines(vec![(input, vec![vec![2, 2]])], Vec::new());
    let anisotropic = task_address(&with(vec![vec![1]])).expect("addressed");
    let planar = task_address(&with(vec![vec![1, 1]])).expect("addressed");
    assert!(anisotropic != planar, "the two pairs differ in one period, so the two tasks do");
    // And one change on different ground is different content.
    let on = |ground: u8| task_of_lines(vec![(vec![vec![1, ground]], vec![vec![2, ground]])], Vec::new());
    assert!(task_address(&on(0)).expect("addressed") != task_address(&on(3)).expect("addressed"));
}

/// MU-DERIVATIONS D3.2 — TWO TASKS DIFFERING ONLY IN A 29×29 → 30×30 PAIR
/// ARE TWO TASKS: the pair is a member of each address, never passed over
/// for its periods' lcm (870 per axis).
#[test]
fn task_address_separates_tasks_differing_in_incommensurate_pair() {
    let coprime = |hue: u8| -> (SectionSample, SectionSample) {
        let mut out = SectionSample::from(vec![vec![1; 30]; 30]);
        out.set([0, 0], hue);
        (SectionSample::from(vec![vec![1; 29]; 29]), out)
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
/// (every site of every sample is written), and an index copies the
/// address it is keyed by. The book keys and holds the task by its 32-byte
/// citation; the address is recomputed from the task where its coordinates
/// are needed, never held.
fn busy_task() -> Task {
    // 40 pairs of 30 × 30 frames: their content passes 64 KiB.
    let busy = |s: u8| -> SectionSample { (0..30).map(|r| (0..30).map(|c| u8::try_from((r * 7 + c * 3 + usize::from(s)) % 9).expect("a residue mod 9") + 1).collect()).collect() };
    let mut task = toy_task();
    task.train.extend((0..40u8).map(|s| (busy(s), busy(s + 1))));
    task
}

/// TRACK 26 S2 (scope separation, ruled 2026-11-03) — A TASK'S IDENTITY
/// READS NO GROUND: the address of one task is the same whatever ground a
/// reader opened (0, 7, a vortex) and with none open; and the same samples
/// always give the same address.
#[test]
fn task_identity_reads_no_ground() {
    use crate::transducer::family::transduce::Ground;
    use crate::transducer::raster::on_ground;
    let task = toy_task();
    let bare = task_address(&task).expect("addressed");
    assert_eq!(on_ground(Ground::One(0), || task_address(&task)), Some(bare.clone()), "on a ground of 0");
    assert_eq!(on_ground(Ground::One(7), || task_address(&task)), Some(bare.clone()), "on a ground of 7");
    assert_eq!(on_ground(Ground::Vortex(vec![0, 1]), || task_address(&task)), Some(bare.clone()), "on a vortex");
    assert_eq!(task_address(&toy_task()), Some(bare), "the same samples, the same address");
}

/// TRACK 26 S2 (scope separation, ruled 2026-11-03) — A TASK'S IDENTITY
/// FIXES NO FRAME: a task and the same task turned by a quarter turn as a
/// whole (every raster of it) are two posted tasks with two addresses,
/// where a frame fixed before addressing would read them as one.
#[test]
fn task_identity_reads_no_frame_fixing() {
    let task = task_of_lines(
        vec![(vec![vec![1, 0, 0], vec![1, 0, 0]], vec![vec![2, 0, 0], vec![2, 0, 0]])],
        vec![(vec![vec![0, 0, 1], vec![0, 0, 1]], Vec::new())],
    );
    // A quarter turn of a sample: line `j` of the turn is column `j` read
    // from the last line up.
    let turn = |g: &SectionSample| -> SectionSample {
        let ([_, width], lines) = (g.extents(), g.raster());
        (0..width).map(|j| lines.iter().rev().filter_map(|line| line.get(j).copied()).collect()).collect()
    };
    let turned = Task {
        train: task.train.iter().map(|(x, y)| (turn(x), turn(y))).collect(),
        test: task.test.iter().map(|(x, y)| (turn(x), y.clone())).collect(),
    };
    assert_ne!(task, turned, "the turn moves the samples");
    let (a, b) = (task_address(&task).expect("addressed"), task_address(&turned).expect("addressed"));
    assert!(a != b, "two posted tasks that differ by a turn have two addresses");
    assert_eq!(task_address(&task), Some(a), "and the same samples always give the same address");
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
    assert_eq!(reg.close(&id), Some(ClosureOrder::First));
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
    let id = reg.register(toy_task()).expect("a sample pair's chain is A5's");
    let (d1, _) = reg.dispatch().expect("work exists");
    assert_eq!(d1, id);
    assert_eq!(reg.close(&id), Some(ClosureOrder::First));
    assert!(reg.dispatch().is_none(), "a settled book dispatches nothing");
}

/// 78.153c — THE DISPATCH CURSOR SURVIVES A RESTART WITH NOTHING CLOSED:
/// its record is due once a page of dispatches passed, and a registry
/// re-registered from the corpus and restored from it dispatches the task
/// after the last one served, never the head. A record of another book is
/// refused whole.
#[test]
fn restore_cursor_resumes_dispatch_after_restart() {
    let other = task_of_lines(
        vec![(vec![vec![3, 0], vec![0, 0]], vec![vec![0, 3], vec![0, 0]])],
        vec![(vec![vec![0, 0], vec![3, 0]], vec![vec![0, 0], vec![0, 3]])],
    );
    let third = task_of_lines(
        vec![(vec![vec![4, 4, 0]], vec![vec![0, 4, 4]])],
        vec![(vec![vec![0, 4, 4]], vec![vec![4, 4, 0]])],
    );
    let book = || {
        let mut reg = TaskRegistry::new();
        let ids: Vec<TaskCitation> = [toy_task(), other.clone(), third.clone()]
            .into_iter()
            .map(|t| reg.register(t).expect("a sample pair's chain is A5's"))
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
    let id = reg.register(toy_task()).expect("a sample pair's chain is A5's");
    let first = reg.next_intake_batch(8);
    assert_eq!(first.len(), 1);
    assert_eq!(first[0].0, id);
    assert!(reg.intake_done());
    assert!(reg.next_intake_batch(8).is_empty(), "the manifest is exhausted");
    assert_eq!(reg.next_cover_batch(1).len(), 1, "cover re-home still walks");
}

/// TRACK 26 S5 — A TASK'S IDENTITY WRITES RANK AND WIDTH: four posted tasks
/// whose symbol bytes are the same four bytes, read as one axis of four
/// sites, two axes of two, one axis of two sites at width two, and one of
/// one site at width four, are four tasks; the raster task's address is its
/// posted samples' address. The frame's own content (`Enclosed::put_content`)
/// writes the rank and the width as well, so it is lossless across both.
#[test]
fn task_identity_writes_rank_and_width() {
    use crate::anastome::dialogue::samples::{position, Member, Split};
    use crate::anastome::dialogue::PostedTask;
    use crate::transducer::sample::Sample;
    let bytes = vec![1u8, 2, 3, 4];
    let posted = |extents: Vec<usize>, width: usize| {
        let place = |member| position(Split::Train, 0, member).expect("a position");
        let x = Sample::new(extents.clone(), width, bytes.clone(), place(Member::Input)).expect("a sample");
        let y = Sample::new(extents, width, bytes.clone(), place(Member::Output)).expect("a sample");
        task_address_of(&PostedTask { train: vec![(x, y)], test: Vec::new() })
    };
    let addresses = [posted(vec![4], 1), posted(vec![2, 2], 1), posted(vec![2], 2), posted(vec![1], 4)];
    for (i, a) in addresses.iter().enumerate() {
        for b in &addresses[i + 1..] {
            assert_ne!(a, b, "a rank or a width is part of the identity");
        }
    }
    let raster = task_of_lines(vec![(vec![vec![1, 2], vec![3, 4]], vec![vec![1, 2], vec![3, 4]])], Vec::new());
    assert_eq!(task_address(&raster), Some(addresses[1].clone()), "a raster task is its samples");

    use crate::transducer::family::transduce::Enclosed;
    use crate::transducer::sample::Place;
    let frame = |extents: Vec<usize>, width: usize| {
        let mut out = Vec::new();
        Enclosed::of(&Sample::new(extents, width, vec![5; 4], Place::new(Vec::new())).expect("a sample")).put_content(&mut out);
        out
    };
    assert_ne!(frame(vec![4], 1), frame(vec![2, 2], 1), "the rank is written");
    assert_ne!(frame(vec![2], 2), frame(vec![1], 4), "the width is written");
}

/// TRACK 26 S4c — THE HYPHA SERVES A RANK-3 TASK: posted from its samples,
/// it is named by their content alone, dispatched, and its envelope served
/// as posted at rank 3, the withheld output never riding. Only the
/// alphabet limits it, by name: the word machinery's readers (the section,
/// the withheld rasters a rule is reconstructed on, the bound families, the
/// routes) take nothing of it, and the walks pass it once routed to none.
#[test]
fn hypha_serves_a_rank_three_task() {
    use crate::anastome::dialogue::{decode_task, encode_task};
    use crate::transducer::alphabet::AlphabetRank;
    use crate::transducer::sentence::SectionRefusal;
    let posted = crate::server::tests::fixtures::rank_three_task();
    let mut reg = TaskRegistry::new();
    let id = reg.post(posted.clone());
    assert_eq!(id, citation(&task_address_of(&posted)), "named by the posted samples' content alone");
    assert_eq!(reg.post(posted.clone()), id, "content is identity: posting again posts nothing");
    assert_eq!(reg.len(), 1);
    let (served, task) = reg.dispatch().expect("the open task is dispatched whatever its rank");
    assert_eq!(served, id);
    let sent = task.family(id);
    let env = decode_task(&encode_task(&id, &sent.train, &sent.tests).expect("the envelope writes at rank 3")).expect("and reads whole");
    assert_eq!(env.id, id);
    assert_eq!(env.train, posted.train, "the train pairs served as posted");
    assert_eq!(env.tests, vec![posted.test[0].0.clone()], "the test input, never the withheld output");
    assert!(env.train.iter().all(|(x, y)| x.rank() == 3 && y.rank() == 3), "served at its own rank");
    assert!(
        matches!(reg.section(&id), Some(Err(SectionRefusal::Rank(AlphabetRank { rank: 3 })))),
        "the alphabet's section refuses it by the alphabet's name"
    );
    assert!(reg.withheld(&id).is_empty(), "no rule is reconstructed on it");
    assert_eq!(reg.families().count(), 0, "it binds no word family");
    let registry = std::sync::Mutex::new(reg);
    assert!(crate::server::route::intake_walk(&registry, 8).is_empty(), "nothing is listed for it");
    let reg = registry.lock().expect("a fresh lock");
    assert_eq!(reg.routes(&id).map(|r| r.len()), Some(0), "routed to none, so no walk reads it again");
    assert_eq!(reg.open(&id), Some(true), "and it stays open");
}

/// #158 — THE WITHHELD PAIRS ARE FIXED BY COORDINATE AT REGISTRATION: each
/// withheld output's place `[split, pair, member]` is recorded when the task
/// is posted, a test input with no output withholds nothing, the reader
/// serves exactly the pairs at those coordinates, and posting the same
/// content again fixes nothing new.
#[test]
fn withheld_pairs_are_fixed_by_coordinate_at_registration() {
    let task = task_of_lines(
        vec![(vec![vec![1, 1], vec![0, 0]], vec![vec![2, 2], vec![0, 0]])],
        vec![
            (vec![vec![0, 1], vec![0, 0]], Vec::new()),
            (vec![vec![0, 0], vec![1, 1]], vec![vec![0, 0], vec![2, 2]]),
        ],
    );
    let mut reg = TaskRegistry::new();
    let id = reg.register(task.clone()).expect("the task has an identity");
    let places = reg.withheld_at(&id).expect("the book holds the task").to_vec();
    assert_eq!(places.len(), 1, "the test input with no output withholds nothing");
    let posted = crate::anastome::dialogue::PostedTask::of_section(&task).expect("the samples post");
    let at = posted.test.iter().find_map(|(_, y)| y.as_ref()).expect("one output posted").place().clone();
    assert_eq!(places, vec![at.clone()], "fixed at the posted output's own coordinates");
    let withheld = reg.withheld(&id);
    assert_eq!(withheld.len(), 1);
    assert_eq!(withheld[0].1.sample().place(), &at, "the reader serves the pair at that coordinate");
    assert_eq!(withheld[0].1, vec![vec![0, 0], vec![2, 2]]);
    assert_eq!(reg.register(task), Some(id));
    assert_eq!(reg.withheld_at(&id).map(<[_]>::to_vec), Some(places), "registered again, nothing new is fixed");
    assert!(reg.withheld_at(&[7u8; 32]).is_none(), "a task the book does not hold fixes none");
}
