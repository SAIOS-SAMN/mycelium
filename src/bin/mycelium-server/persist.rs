// 28 — the ledger on disk: the kernel's state written whole, and written
// only when the kernel says it is due.

use mycelium::cognition::Kernel;
use std::sync::Mutex;

/// Durable state: the kernel snapshot lands atomically (to .tmp, then
/// rename) so a crash mid-write never corrupts the ledger. Every path that
/// changes the held language — germination, a verdict, an adoption — writes
/// through here; a restart then resumes exactly what the node held.
pub(crate) fn persist(kernel: &Mutex<Kernel>, state_path: &str) {
    // The stamp the snapshot is taken at, read in the same hold (ACT1).
    let (snap, at) = match kernel.lock() {
        Ok(k) => (k.snapshot(), k.snapshot_at()),
        Err(_) => return,
    };
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
