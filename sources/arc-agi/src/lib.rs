// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! THE ARC-AGI-2 CORPUS'S DECODER (MU-SENSING §7; ROADMAP track 26, S1).
//!
//! The corpus's own format, read into the core's carriers and stopped
//! there: its JSON (`{"train":[{"input","output"}],"test":[...]}`), its
//! directory layout (`training/`, `evaluation/`, one file per task) and its
//! split into train and test. The symbols `0`–`9` are labels. Nothing of
//! the corpus enters the sensor, the mind, the court or the wire.
//!
//! What it writes:
//! - a [`Task`] as the file gives it ([`decode`]), its frame as read: no
//!   ground is read and no frame is fixed here (track 26 S2: frame fixing
//!   is a perspective a sensor may take after intake, never before the
//!   hypha addresses the task);
//! - the task's samples, each placed in its family ([`samples_of`]);
//! - the posted task file (`dialogue::encode_posted_task`) the hypha, the
//!   soak and a client's feed read ([`post`]), so none of them parses this
//!   format.

use mycelium::transducer::family::Task;
use mycelium::transducer::sample::{Place, Sample, SampleRefusal};
use serde::Deserialize;

/// A grid of the corpus: its rows of symbols, as the JSON writes them.
pub type Grid = Vec<Vec<u8>>;

#[derive(Deserialize)]
struct RawPair {
    input: Grid,
    #[serde(default)]
    output: Grid,
}

#[derive(Deserialize)]
struct RawTask {
    #[serde(default)]
    train: Vec<RawPair>,
    #[serde(default)]
    test: Vec<RawPair>,
}

/// THE CORPUS A READER OPENS: the `split` directory (`training` or
/// `evaluation`) under `MYCELIUM_CORPUS` where it is set, else under
/// `data/arc-agi-2` at the repository's root. The corpus is not vendored; a
/// reader that finds no corpus says so.
pub fn corpus(split: &str) -> String {
    let root = match std::env::var("MYCELIUM_CORPUS") {
        Ok(r) => r,
        Err(_) => concat!(env!("CARGO_MANIFEST_DIR"), "/../../data/arc-agi-2").to_string(),
    };
    format!("{root}/{split}")
}

/// THE TASK FILES OF A SPLIT DIRECTORY, `*.json`, in name order: the
/// coordinate the corpus files its tasks under, never the directory's own
/// order.
pub fn task_names(dir: &str) -> std::io::Result<Vec<String>> {
    let mut names: Vec<String> = Vec::new();
    for e in std::fs::read_dir(dir)? {
        let name = e?.file_name().to_string_lossy().into_owned();
        if name.ends_with(".json") {
            names.push(name);
        }
    }
    names.sort();
    Ok(names)
}

/// A TASK'S JSON AS WRITTEN: its train pairs and its test pairs, a withheld
/// test output empty, the frame as the file gives it. `None` where the text
/// is not a task.
pub fn decode(json: &str) -> Option<Task> {
    let raw: RawTask = serde_json::from_str(json).ok()?;
    Some(Task {
        train: raw.train.into_iter().map(|p| (p.input, p.output)).collect(),
        test: raw.test.into_iter().map(|p| (p.input, p.output)).collect(),
    })
}

/// A pair's place in its split: train or test.
const TRAIN: i64 = 0;
const TEST: i64 = 1;
/// A sample's place in its pair: the input or the output.
const INPUT: i64 = 0;
const OUTPUT: i64 = 1;

/// THE TASK'S SAMPLES, each placed in its family (MU-SENSING §2 (4)): the
/// place is `[split, pair, member]`, the split 0 for train and 1 for test,
/// the member 0 for the input and 1 for the output. Each grid is a sample
/// of rank 2 and width 1 (`raster::sample_of`). A withheld (empty) output
/// has no sample and is left out; a ragged grid is refused by name.
pub fn samples_of(task: &Task) -> Result<Vec<Sample>, SampleRefusal> {
    let mut out = Vec::new();
    for (split, pairs) in [(TRAIN, &task.train), (TEST, &task.test)] {
        for (pair, (x, y)) in (0i64..).zip(pairs) {
            out.push(mycelium::transducer::raster::sample_of(x, Place::new(vec![split, pair, INPUT]))?);
            if !y.is_empty() {
                out.push(mycelium::transducer::raster::sample_of(y, Place::new(vec![split, pair, OUTPUT]))?);
            }
        }
    }
    Ok(out)
}

/// WHAT [`post`] DID: the tasks written, and each file refused with why.
#[derive(Debug, Default)]
pub struct Posted {
    pub written: usize,
    pub refused: Vec<(String, &'static str)>,
}

/// POSTS A SPLIT: each task file of `from` in name order, up to `limit`,
/// read by [`decode`] as the file gives it (no ground read, no frame fixed)
/// and written to `to` as `<stem>.task`, the posted form the hypha, the
/// soak and a client's feed read. A file that does not read, is not a task,
/// or does not fit the posted form is refused by name and skipped; nothing
/// is guessed.
pub fn post(from: &str, to: &std::path::Path, limit: usize) -> std::io::Result<Posted> {
    std::fs::create_dir_all(to)?;
    let mut posted = Posted::default();
    for name in task_names(from)?.into_iter().take(limit) {
        let Ok(json) = std::fs::read_to_string(format!("{from}/{name}")) else {
            posted.refused.push((name, "does not read"));
            continue;
        };
        let Some(task) = decode(&json) else {
            posted.refused.push((name, "is not a task"));
            continue;
        };
        let Some(bytes) = mycelium::anastome::dialogue::encode_posted_task(&task) else {
            posted.refused.push((name, "does not fit the posted form"));
            continue;
        };
        let stem = name.trim_end_matches(".json");
        std::fs::write(to.join(format!("{stem}.{}", mycelium::anastome::dialogue::POSTED_TASK)), bytes)?;
        posted.written += 1;
    }
    Ok(posted)
}
