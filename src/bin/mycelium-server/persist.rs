// 28, 45.8 — the ledger on disk: the line written in its own order, then
// the snapshot that names where the line stood, written only when the
// kernel says it is due.

use mycelium::cognition::Kernel;
use std::sync::Mutex;

/// Durable state, in the order the two depend on each other: the line
/// first — a frame appended, or the whole through a rename — then the
/// snapshot, atomically (to .tmp, then rename), which names the stamp the
/// line stood at. A snapshot is never written over a line that did not
/// land, so the two on disk always agree; a crash between them leaves a
/// line ahead of its snapshot, which restores. Every path that changes the
/// held language writes through here.
pub(crate) fn persist(kernel: &Mutex<Kernel>, state_path: &str) {
    // The stamp the snapshot is taken at, and the line it stands on, read
    // in the same hold (ACT1).
    let (line, snap, at) = match kernel.lock() {
        Ok(mut k) => (k.take_line(), k.snapshot(), k.snapshot_at()),
        Err(_) => return,
    };
    let line_landed = match line {
        Ok(None) => true,
        Ok(Some((path, write))) => write.land(&path),
        Err(why) => {
            eprintln!("  persist refused: {why}; nothing is written");
            false
        }
    };
    if !line_landed {
        if let Ok(mut k) = kernel.lock() {
            k.line_lost();
        }
        return;
    }
    let tmp = format!("{state_path}.tmp");
    let landed = std::fs::write(&tmp, &snap).is_ok() && std::fs::rename(&tmp, state_path).is_ok();
    // Only a write that landed is a persist on the clock (78.21(2)).
    if landed {
        if let Ok(mut k) = kernel.lock() {
            k.note_persisted(at);
        }
    }
}

/// Persist where a change was marked; whether it was.
pub(crate) fn persist_if_due(kernel: &Mutex<Kernel>, state_path: &str) -> bool {
    let due = kernel.lock().ok().is_some_and(|mut k| k.take_persist());
    if due {
        persist(kernel, state_path);
    }
    due
}
