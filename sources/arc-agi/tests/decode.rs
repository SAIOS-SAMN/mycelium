// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// MU-SENSING §2, §6, §7; ROADMAP track 26 S1 — THE CORPUS'S DECODER: the
// JSON read into a task, the task into placed samples and into the posted
// form the hypha reads, and the corpus's sweep through the sample bridge
// (moved from the core's former `tests::arc`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use mycelium::anastome::dialogue::{decode_posted_task, encode_posted_task};
use mycelium::transducer::raster::links_of;
use mycelium::transducer::sample::AxisReading;
use mycelium::transducer::spelled::SectionSample;
use mycelium_source_arc_agi::{corpus, decode, samples_of, task_names};

/// The training split's tasks, in name order, through the decoder.
fn training() -> Vec<(String, mycelium::transducer::family::Task)> {
    let dir = corpus("training");
    let names = task_names(&dir).expect("the corpus stands");
    names
        .into_iter()
        .filter_map(|name| {
            let json = std::fs::read_to_string(format!("{dir}/{name}")).ok()?;
            decode(&json).map(|t| (name, t))
        })
        .collect()
}

/// A task's JSON decodes into its train pairs and test inputs as the file
/// gives them, each grid a sample on the alphabet's section, and a withheld
/// test output absent.
#[test]
fn decode_reads_train_and_withheld_pairs() {
    let json = r#"{"train":[{"input":[[0,1],[2,0]],"output":[[1,0],[0,2]]}],"test":[{"input":[[3]]}]}"#;
    let t = decode(json).expect("decodes");
    assert_eq!(t.train.len(), 1);
    assert_eq!(t.test.len(), 1);
    assert_eq!(t.train[0].0, SectionSample::of_lines(&[vec![0, 1], vec![2, 0]]).expect("a sample"));
    assert!(t.test[0].1.is_none(), "a withheld test output is absent");
    assert!(decode("[1, 2]").is_none(), "a text that is no task is refused");
    let ragged = r#"{"train":[{"input":[[0,1],[2]],"output":[[1]]}],"test":[]}"#;
    assert!(decode(ragged).is_none(), "a ragged grid is no sample, refused at intake");
}

/// A SAMPLE IS WRITTEN BACK INTO THE CORPUS'S ROWS BY ITS SYMBOLS' BYTES
/// (the speech end of the decoder, through the public `Symbol::bytes`): a
/// grid decoded and written back is the grid; a sample of another rank or
/// width is no grid of the corpus, refused.
#[test]
fn rows_of_writes_a_sample_back_into_the_corpus_rows() {
    let json = r#"{"train":[{"input":[[0,1,7],[2,0,9]],"output":[[1]]}],"test":[]}"#;
    let t = decode(json).expect("decodes");
    let samples = samples_of(&t).expect("every grid stands");
    assert_eq!(mycelium_source_arc_agi::rows_of(&samples[0]), Some(vec![vec![0, 1, 7], vec![2, 0, 9]]));
    assert_eq!(mycelium_source_arc_agi::rows_of(&samples[1]), Some(vec![vec![1]]));
    let place = mycelium::transducer::sample::Place::new(Vec::new());
    let line = mycelium::transducer::sample::Sample::new(vec![3], 1, vec![1, 2, 3], place.clone()).expect("a sample");
    assert_eq!(mycelium_source_arc_agi::rows_of(&line), None, "rank 1 is no grid of the corpus");
    let wide = mycelium::transducer::sample::Sample::new(vec![1, 1], 2, vec![0, 1], place).expect("a sample");
    assert_eq!(mycelium_source_arc_agi::rows_of(&wide), None, "two bytes per symbol is no grid of the corpus");
}

/// A CORPUS TASK DECODES INTO PLACED SAMPLES: each grid a sample of rank 2
/// and width 1, its extents the grid's own, placed at `[split, pair,
/// member]` in its family, a withheld output left out.
#[test]
fn samples_of_places_corpus_grids_in_family() {
    let (name, task) = training().into_iter().next().expect("a corpus task");
    let samples = samples_of(&task).expect("every grid stands");
    let train = (0i64..).zip(&task.train).flat_map(|(pair, (x, y))| [(x, [0, pair, 0]), (y, [0, pair, 1])]);
    let test = (0i64..).zip(&task.test).flat_map(|(pair, (x, y))| std::iter::once((x, [1, pair, 0])).chain(y.as_ref().map(|y| (y, [1, pair, 1]))));
    let grids: Vec<(&SectionSample, [i64; 3])> = train.chain(test).collect();
    assert_eq!(samples.len(), grids.len(), "{name}: one sample per grid shown");
    for (s, (g, place)) in samples.iter().zip(&grids) {
        let [h, w] = g.extents();
        assert_eq!((s.rank(), s.width()), (2, 1), "{name}: rank 2, width 1");
        assert_eq!((s.extent(0), s.extent(1)), (AxisReading::Read(h), AxisReading::Read(w)), "{name}: the grid's own extents");
        assert_eq!(s.place().coordinates(), place, "{name}: placed in its family");
    }
}

/// THE POSTED TASK CARRIES THE CORPUS EXACTLY: every training task, posted
/// and read back without the decoder, is the same task, and its content
/// address (rank 2, width 1, as the registry names it) is unchanged.
#[test]
fn posted_task_round_trips_corpus_with_address_unchanged() {
    let tasks = training();
    assert!(!tasks.is_empty(), "the corpus held tasks");
    for (name, task) in &tasks {
        let bytes = encode_posted_task(task).expect("the task posts");
        let back = decode_posted_task(&bytes).expect("the posted task reads");
        assert_eq!(&back, task, "{name}: the same task");
        assert_eq!(
            mycelium::registry::task_address(&back),
            mycelium::registry::task_address(task),
            "{name}: the same address"
        );
    }
}

/// THE BODY-BOUNDARY RECORD READ BACK THROUGH PROJECTION IS TODAY'S δ on
/// every equal-extent pair of the corpus, read through the bridge: on one
/// enclosure every turn lands, and the projected record, written back as
/// planar links, is the planar coboundary link for link.
#[test]
fn corpus_pairs_through_bridge_project_to_delta() {
    use mycelium::transducer::family::transduce::{need_of, Transduction};
    let mut read = 0usize;
    for (name, task) in training() {
        let tests = task.test.iter().filter_map(|(x, y)| Some((x, y.as_ref()?)));
        for (x, y) in task.train.iter().map(|(x, y)| (x, y)).chain(tests) {
            if x.extents() != y.extents() {
                continue;
            }
            let (a, b) = (x.sample(), y.sample());
            let need = need_of(a, b).expect("one rank");
            let t = Transduction::of(a, b, &mut mycelium::physics::Physics::default().stroke_n(need)).expect("the need is the stroke");
            let projected = links_of(&t.projected().expect("one enclosure projects"));
            assert_eq!(projected, mycelium::transducer::family::speak::residue_of(Some(x), y).expect("one enclosure needs no ground"), "{name}");
            read += 1;
        }
    }
    assert!(read > 0, "the corpus held equal-extent pairs");
}
