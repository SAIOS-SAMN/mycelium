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
//! - a [`Task`] as the file gives it ([`decode`]), each grid read at intake
//!   as a sample on the alphabet's section, its frame as read: no
//!   ground is read and no frame is fixed here (track 26 S2: frame fixing
//!   is a perspective a sensor may take after intake, never before the
//!   hypha addresses the task);
//! - the task's samples, each placed in its family ([`samples_of`]);
//! - the posted task file (`dialogue::encode_posted_task`) the hypha, the
//!   soak and a client's feed read ([`post`]), so none of them parses this
//!   format.

use mycelium::transducer::family::Task;
use mycelium::transducer::sample::{Place, Sample, SampleRefusal};
use mycelium::transducer::spelled::SectionSample;
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
/// (empty) test output absent, the frame as the file gives it. Each grid is
/// read at intake as a sample of rank 2 and width 1 at its place
/// `[split, pair, member]` (`raster::sample_of`), on the alphabet's section.
/// `None` where the text is not a task, or a grid is no such sample (ragged
/// or empty).
pub fn decode(json: &str) -> Option<Task> {
    let raw: RawTask = serde_json::from_str(json).ok()?;
    let read = |g: &Grid, split: i64, pair: i64, member: i64| -> Option<SectionSample> {
        let sample = mycelium::transducer::raster::sample_of(g, Place::new(vec![split, pair, member])).ok()?;
        SectionSample::of(sample).ok()
    };
    let mut train = Vec::with_capacity(raw.train.len());
    for (pair, p) in (0i64..).zip(&raw.train) {
        train.push((read(&p.input, TRAIN, pair, INPUT)?, read(&p.output, TRAIN, pair, OUTPUT)?));
    }
    let mut test = Vec::with_capacity(raw.test.len());
    for (pair, p) in (0i64..).zip(&raw.test) {
        let output = match p.output.is_empty() {
            true => None,
            false => Some(read(&p.output, TEST, pair, OUTPUT)?),
        };
        test.push((read(&p.input, TEST, pair, INPUT)?, output));
    }
    Some(Task { train, test })
}

/// A pair's place in its split: train or test.
const TRAIN: i64 = 0;
const TEST: i64 = 1;
/// A sample's place in its pair: the input or the output.
const INPUT: i64 = 0;
const OUTPUT: i64 = 1;

/// THE TASK'S SAMPLES, each placed in its family (MU-SENSING §2 (4)): the
/// place is `[split, pair, member]`, the split 0 for train and 1 for test,
/// the member 0 for the input and 1 for the output. A withheld (absent)
/// output has no sample and is left out.
pub fn samples_of(task: &Task) -> Result<Vec<Sample>, SampleRefusal> {
    let place = |split: i64, pair: i64, member: i64| Place::new(vec![split, pair, member]);
    let mut out = Vec::new();
    for (pair, (x, y)) in (0i64..).zip(&task.train) {
        out.push(x.sample().placed(place(TRAIN, pair, INPUT)));
        out.push(y.sample().placed(place(TRAIN, pair, OUTPUT)));
    }
    for (pair, (x, y)) in (0i64..).zip(&task.test) {
        out.push(x.sample().placed(place(TEST, pair, INPUT)));
        if let Some(y) = y {
            out.push(y.sample().placed(place(TEST, pair, OUTPUT)));
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
