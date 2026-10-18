// 78.164 — THE FEED, A BOUNDARY: the corpus this mind's home holds, each
// family placed on the mind's corpus at its own coordinates (`focus`) and met
// where the mind's orbits stand, entering the field as a family to think
// through whole.
//
//! A feed file is retired when it has entered, so a restart does not present
//! the same family twice; a family not met stays on disk.
//!
//! NO ORDER OF FILES (the operator, 2026-10-18: "feed walked sounds
//! uni-directional and linear"): the folder is listed where a family ARRIVES
//! (the kernel's file notice, waking the resting mind — no clock polls the
//! folder) and at the first reading; each new family is handed out once to
//! be placed at its coordinates, and taken back by its citation where the
//! mind's focus meets it.

use crate::network::nerve::Keyed;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

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

/// A FILE'S KEY: its path as written.
fn file_key(path: &Path) -> Vec<u8> {
    path.to_string_lossy().as_bytes().to_vec()
}

/// THE FEED: the files already handed out, each family's file by its
/// citation, and whether a family arrived since the folder was last listed.
pub(crate) struct Feed {
    home: PathBuf,
    listed: Keyed<()>,
    files: Keyed<PathBuf>,
    arrived: Arc<AtomicBool>,
}

impl Feed {
    pub(crate) fn new(home: PathBuf) -> Feed {
        // The first reading lists the folder as an arrival does.
        Feed { home, listed: Keyed::new(), files: Keyed::new(), arrived: Arc::new(AtomicBool::new(true)) }
    }

    /// THE FOLDER WATCHED: a family written or moved into `home/feed` marks
    /// the feed arrived and wakes the mind on `wake` (the kernel's file
    /// notice, on its own thread; no clock). A folder that cannot be watched
    /// is said once; what it holds at the first reading is still placed.
    pub(crate) fn watch(&self, wake: std::sync::mpsc::Sender<()>) {
        use inotify::{Inotify, WatchMask};
        let dir = self.home.join("feed");
        let _ = std::fs::create_dir_all(&dir);
        let watched = Inotify::init().and_then(|i| i.watches().add(&dir, WatchMask::CLOSE_WRITE | WatchMask::MOVED_TO).map(|_| i));
        let mut inotify = match watched {
            Ok(i) => i,
            Err(e) => {
                eprintln!("  feed not watched ({e}): read at the first reading only");
                return;
            }
        };
        let arrived = Arc::clone(&self.arrived);
        let spawned = std::thread::Builder::new().name("feed".into()).spawn(move || {
            let mut buffer = [0u8; 4096];
            while let Ok(mut events) = inotify.read_events_blocking(&mut buffer) {
                if events.next().is_some() {
                    arrived.store(true, Ordering::SeqCst);
                    if wake.send(()).is_err() {
                        return;
                    }
                }
            }
        });
        if let Err(e) = spawned {
            eprintln!("  feed not watched ({e}): read at the first reading only");
        }
    }

    /// THE FAMILIES THAT ARRIVED, each once with its file: the folder listed
    /// where a family arrived since the last listing, and every file not yet
    /// handed out read whole — one that holds no task is retired. None where
    /// nothing arrived.
    pub(crate) fn arrivals(&mut self) -> Vec<(PathBuf, crate::arc::Task)> {
        if !self.arrived.swap(false, Ordering::SeqCst) {
            return Vec::new();
        }
        let mut out = Vec::new();
        for path in pending_feed(&self.home) {
            let key = file_key(&path);
            if self.listed.contains(&key) {
                continue;
            }
            self.listed.insert(key, ());
            match std::fs::read_to_string(&path).ok().and_then(|t| crate::arc::parse_task(&t)) {
                Some(task) => out.push((path, task)),
                None => retire(&path),
            }
        }
        out
    }

    /// The family at `path` filed by its citation `id`, to be taken back by it.
    pub(crate) fn file(&mut self, path: &Path, id: &[u8]) {
        self.files.insert(id, path.to_path_buf());
    }

    /// THE FAMILY AT ITS CITATION, read again from its file; none where the
    /// file left or no longer reads.
    pub(crate) fn take(&self, id: &[u8]) -> Option<(PathBuf, crate::arc::Task)> {
        let path = self.files.get(id)?.clone();
        let task = std::fs::read_to_string(&path).ok().and_then(|t| crate::arc::parse_task(&t))?;
        Some((path, task))
    }

    /// The family at `path`, citation `id`, entered the field: its file is
    /// retired.
    pub(crate) fn entered(&mut self, path: &Path, id: &[u8]) {
        retire(path);
        self.files.remove(id);
        self.listed.remove(&file_key(path));
    }

    /// A family filed at `id` that no longer reads: let go.
    pub(crate) fn forget(&mut self, id: &[u8]) {
        if let Some(path) = self.files.remove(id) {
            self.listed.remove(&file_key(&path));
        }
    }

    /// The families placed and not yet entered.
    pub(crate) fn held(&self) -> usize {
        self.files.len()
    }
}
