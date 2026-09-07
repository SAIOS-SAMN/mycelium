// 28 — the ledger on disk: the kernel's state written whole, and written
// only when the kernel says it is due.

use mycelium::cognition::Kernel;
use std::sync::Mutex;

/// Durable state: the kernel snapshot lands atomically (to .tmp, then
/// rename) so a crash mid-write never corrupts the ledger. Every path that
/// changes the held language — germination, a verdict, an adoption — writes
/// through here; a restart then resumes exactly what the node held.
pub(crate) fn persist(kernel: &Mutex<Kernel>, state_path: &str) {
    let snap = match kernel.lock() {
        Ok(k) => k.snapshot(),
        Err(_) => return,
    };
    let tmp = format!("{state_path}.tmp");
    if std::fs::write(&tmp, &snap).is_ok() {
        let _ = std::fs::rename(&tmp, state_path);
    }
}

pub(crate) fn persist_if_due(kernel: &Mutex<Kernel>, state_path: &str) {
    let due = kernel.lock().ok().is_some_and(|mut k| k.take_persist());
    if due {
        persist(kernel, state_path);
    }
}
