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
///
/// 78.137 — DURABLY: each write synced to the disk, its rename's directory
/// synced; whether the snapshot landed is answered, so a court act is
/// acknowledged only once it is on the disk.
pub(crate) fn persist(kernel: &Mutex<Kernel>, state_path: &str) -> bool {
    // The stamp the snapshot is taken at, and the line it stands on, read
    // in the same hold (ACT1).
    let (line, snap, at) = match kernel.lock() {
        Ok(mut k) => (k.take_line(), k.snapshot(), k.snapshot_at()),
        Err(_) => return false,
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
        return false;
    }
    let landed = mycelium::chronometry::replace_durably(std::path::Path::new(state_path), &snap);
    // Only a write that landed is a persist on the clock (78.21(2)).
    if landed {
        if let Ok(mut k) = kernel.lock() {
            k.note_persisted(at);
        }
    }
    landed
}

/// 78.137 — A COURT ACT MADE DURABLE BEFORE IT IS ANSWERED: where the act
/// changed what the court holds, the line and snapshot are persisted now,
/// synced; `false` where that did not land — the act is then not answered,
/// and its resend is answered with its original receipt once durable.
pub(crate) fn durable(kernel: &Mutex<Kernel>, state_path: &str) -> bool {
    let due = kernel.lock().ok().is_some_and(|mut k| k.take_persist());
    if !due || persist(kernel, state_path) {
        return true;
    }
    if let Ok(mut k) = kernel.lock() {
        k.persist_failed();
    }
    false
}

/// Persist where a change was marked; whether it was.
pub(crate) fn persist_if_due(kernel: &Mutex<Kernel>, state_path: &str) -> bool {
    let due = kernel.lock().ok().is_some_and(|mut k| k.take_persist());
    if due {
        persist(kernel, state_path);
    }
    due
}
