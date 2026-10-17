// 78.164 — THE FEED, A BOUNDARY: the corpus this mind's home holds, one
// family at a time, entering the field as a family to think through whole.
//
//! A feed file is retired when it has entered, so a restart does not present
//! the same family twice.

use std::path::{Path, PathBuf};

/// The families waiting in `home/feed`, in file order.
pub fn pending_feed(home: &Path) -> Vec<PathBuf> {
    // A directory that will not open lists no files.
    let Ok(rd) = std::fs::read_dir(home.join("feed")) else {
        return Vec::new();
    };
    let mut files: Vec<_> = rd.filter_map(|e| e.ok().map(|e| e.path())).filter(|p| p.extension().is_some_and(|x| x == "json")).collect();
    files.sort();
    files
}

fn retire(path: &Path) {
    let Some(parent) = path.parent() else { return };
    let done = parent.join(".done");
    let _ = std::fs::create_dir_all(&done);
    if let Some(name) = path.file_name() {
        let _ = std::fs::rename(path, done.join(name));
    }
}

/// THE FEED'S CURSOR: the files read at the last listing, and the next.
pub(crate) struct Feed {
    home: PathBuf,
    files: Vec<PathBuf>,
    at: usize,
}

impl Feed {
    pub(crate) fn new(home: PathBuf) -> Feed {
        Feed { home, files: Vec::new(), at: 0 }
    }

    /// The next family file, listing the folder again once the last is read.
    fn next(&mut self) -> Option<PathBuf> {
        if self.at >= self.files.len() {
            self.files = pending_feed(&self.home);
            self.at = 0;
        }
        self.files.get(self.at).cloned()
    }

    /// THE NEXT FAMILY THE FEED HOLDS, and its file: files that hold no task
    /// are retired and stepped over. The family stands at its file until it
    /// enters ([`Feed::entered`]) — a family the reserve cannot fund waits.
    pub(crate) fn peek(&mut self) -> Option<(PathBuf, crate::arc::Task)> {
        while let Some(path) = self.next() {
            match std::fs::read_to_string(&path).ok().and_then(|t| crate::arc::parse_task(&t)) {
                Some(task) => return Some((path, task)),
                None => {
                    retire(&path);
                    self.at += 1;
                }
            }
        }
        None
    }

    /// The family at `path` entered the field: its file is retired.
    pub(crate) fn entered(&mut self, path: &Path) {
        retire(path);
        self.at += 1;
    }

    /// MU-CLIENT §7.1 — THE FAMILY STANDING WAS PASSED: the mind did not take
    /// it, and it stays on disk, offered again at the next listing.
    pub(crate) fn passed(&mut self) {
        self.at += 1;
    }
}
