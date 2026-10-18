// 78.164 — THE FEED, A BOUNDARY: the corpus this mind's home holds, each
// family an offer the mind takes or passes (MU-CLIENT §7.1), entering the
// field as a family to think through whole.
//
//! A feed file is retired when it has entered, so a restart does not present
//! the same family twice. A family the mind passes stays on disk.
//!
//! ONE WALK OF THE LISTING (the operator, 2026-10-18): the families are
//! offered in file order, each read once to its offer (its citation and the
//! charts its bodies stand on) and held by its file; one that couples to
//! nothing is passed at once and the walk goes on, so no family stands in
//! front of the rest. At the listing's end the walk waits — listed again
//! only where a family ARRIVED at the folder (the kernel's file notice,
//! waking the resting mind: no clock polls the folder) or the mind's
//! language MOVED since the walk ended, so a family passed may couple now.

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

/// A FILE'S KEY in the offers held: its path as written.
fn file_key(path: &Path) -> Vec<u8> {
    path.to_string_lossy().as_bytes().to_vec()
}

/// THE FEED: the files read at the last listing and where the walk stands,
/// each family's offer held by its file (its citation, the charts its bodies
/// stand on), the language root the last walk ended at, and whether a family
/// arrived since.
pub(crate) struct Feed {
    home: PathBuf,
    files: Vec<PathBuf>,
    at: usize,
    offers: Keyed<(Vec<u8>, Vec<Vec<u8>>)>,
    walked_at: Option<crate::transducer::address::Address>,
    arrived: Arc<AtomicBool>,
}

impl Feed {
    pub(crate) fn new(home: PathBuf) -> Feed {
        Feed { home, files: Vec::new(), at: 0, offers: Keyed::new(), walked_at: None, arrived: Arc::new(AtomicBool::new(false)) }
    }

    /// THE FOLDER WATCHED: a family written or moved into `home/feed` marks
    /// the feed arrived and wakes the mind on `wake` (the kernel's file
    /// notice, on its own thread; no clock). A folder that cannot be watched
    /// is said once, and the feed is listed again where the language moves.
    pub(crate) fn watch(&self, wake: std::sync::mpsc::Sender<()>) {
        use inotify::{Inotify, WatchMask};
        let dir = self.home.join("feed");
        let _ = std::fs::create_dir_all(&dir);
        let watched = Inotify::init().and_then(|i| i.watches().add(&dir, WatchMask::CLOSE_WRITE | WatchMask::MOVED_TO).map(|_| i));
        let mut inotify = match watched {
            Ok(i) => i,
            Err(e) => {
                eprintln!("  feed not watched ({e}): read again where the language moves");
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
            eprintln!("  feed not watched ({e}): read again where the language moves");
        }
    }

    /// WHETHER THE WALK GOES ON: within the listing, it does; at its end, the
    /// folder is listed again where a family arrived or the language (`root`)
    /// moved since the walk ended — else the walk waits.
    pub(crate) fn walk(&mut self, root: &crate::transducer::address::Address) -> bool {
        if self.at < self.files.len() {
            return true;
        }
        let arrived = self.arrived.swap(false, Ordering::SeqCst);
        if !arrived && self.walked_at.as_ref() == Some(root) {
            return false;
        }
        self.files = pending_feed(&self.home);
        self.at = 0;
        self.walked_at = Some(root.clone());
        let listed: Keyed<()> = {
            let mut k = Keyed::new();
            for f in &self.files {
                k.insert(file_key(f), ());
            }
            k
        };
        // An offer held for a file no longer listed leaves with it.
        let gone: Vec<Vec<u8>> = self.offers.scoped().map(|(k, _)| k).filter(|k| !listed.contains(k)).collect();
        for k in gone {
            self.offers.remove(&k);
        }
        !self.files.is_empty()
    }

    /// THE NEXT FAMILY OF THE WALK, and its file: files that hold no task are
    /// retired and stepped over. `None` at the listing's end.
    pub(crate) fn next_family(&mut self) -> Option<(PathBuf, crate::arc::Task)> {
        while let Some(path) = self.files.get(self.at).cloned() {
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

    /// The offer held for the family at `path`: its citation and charts.
    pub(crate) fn offer_of(&self, path: &Path) -> Option<&(Vec<u8>, Vec<Vec<u8>>)> {
        self.offers.get(&file_key(path))
    }

    /// The offer of the family at `path`, read once and held.
    pub(crate) fn hold_offer(&mut self, path: &Path, id: Vec<u8>, charts: Vec<Vec<u8>>) {
        self.offers.insert(file_key(path), (id, charts));
    }

    /// The family at `path` entered the field: its file is retired.
    pub(crate) fn entered(&mut self, path: &Path) {
        retire(path);
        self.offers.remove(&file_key(path));
        self.at += 1;
    }

    /// MU-CLIENT §7.1 — THE FAMILY WAS PASSED: the mind did not take it, it
    /// stays on disk, and the walk goes on.
    pub(crate) fn passed(&mut self) {
        self.at += 1;
    }

    /// The observer's reading: files listed, where the walk stands, offers
    /// held.
    pub(crate) fn reading(&self) -> (usize, usize, usize) {
        (self.files.len(), self.at, self.offers.len())
    }
}
