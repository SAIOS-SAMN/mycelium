// 38 — THE FEED: the corpus this voice has been given, one task at a time.
//
//! A feed file is retired when it is spent, so a restart does not present
//! the same pair twice.

use mycelium::arc::{embed, grid_to_cells};
use mycelium::transducer::element::measure_lanes;
use mycelium::transducer::germinate::germinate;
use std::path::{Path, PathBuf};

pub(crate) fn pending_feed(home: &Path) -> Vec<PathBuf> {
    let dir = home.join("feed");
    let mut files: Vec<_> = std::fs::read_dir(&dir)
        .map(|rd| {
            rd.filter_map(|e| e.ok().map(|e| e.path()))
                .filter(|p| p.extension().is_some_and(|x| x == "json"))
                .collect()
        })
        .unwrap_or_default();
    files.sort();
    files
}

pub(crate) fn retire_feed(path: &Path) {
    let Some(parent) = path.parent() else { return };
    let done = parent.join(".done");
    let _ = std::fs::create_dir_all(&done);
    if let Some(name) = path.file_name() {
        let _ = std::fs::rename(path, done.join(name));
    }
}

/// Corpus cursor: `next` is the next feed path. Pair index lives on the
/// cursor so a depleted stroke resumes the same file.
pub(crate) struct Feed {
    home: PathBuf,
    files: Vec<PathBuf>,
    ti: usize,
    pi: usize,
    conductor: Option<String>,
}

impl Feed {
    pub(crate) fn new(home: PathBuf, conductor: Option<String>) -> Self {
        Self {
            home,
            files: Vec::new(),
            ti: 0,
            pi: 0,
            conductor,
        }
    }

    pub(crate) fn next(&mut self) -> Option<PathBuf> {
        if self.ti >= self.files.len() {
            self.files = pending_feed(&self.home);
            if self.files.is_empty() {
                return None;
            }
            if let Some(board) = &self.conductor {
                let cursor = conductor_cursor(board) % self.files.len().max(1);
                self.files.rotate_left(cursor);
            }
            self.ti = 0;
            self.pi = 0;
        }
        self.files.get(self.ti).cloned()
    }

    pub(crate) fn pair_index(&self) -> usize {
        self.pi
    }

    pub(crate) fn advance_pair(&mut self) {
        self.pi += 1;
    }

    pub(crate) fn finish(&mut self) {
        self.ti += 1;
        self.pi = 0;
    }

    pub(crate) fn exhausted(&self) -> bool {
        self.ti >= self.files.len()
    }
}

pub(crate) fn family_magnitude(train: &[(Vec<Vec<u8>>, Vec<Vec<u8>>)]) -> [u64; 4] {
    let mut mag = [0u64; 4];
    for (x, y) in train {
        let (ho, wo) = (y.len(), y.first().map_or(0, |r| r.len()));
        if x.is_empty() || ho == 0 || wo == 0 {
            continue;
        }
        let (a, b) = (grid_to_cells(&embed(x, ho, wo)), grid_to_cells(y));
        if let Some(g) = germinate(&a, &b) {
            let m = measure_lanes(&g);
            for l in 0..4 {
                mag[l] = mag[l].saturating_add(m[l]);
            }
        }
    }
    mag
}

/// The conductor's cursor — a world model's board schedules the corpus order.
pub(crate) fn conductor_cursor(path: &str) -> usize {
    let Ok(text) = std::fs::read_to_string(path) else { return 0 };
    text.split("\"arc_next\":")
        .nth(1)
        .and_then(|t| t.chars().take_while(|c| c.is_ascii_digit()).collect::<String>().parse().ok())
        .unwrap_or(0)
}
