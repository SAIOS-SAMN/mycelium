// MU-DERIVATIONS D2.5 (a) — THE NERVE'S QUEUE, bounded by the process's
// memory as the seal read it.
//
//! Every act this node speaks to a peer leaves through the nerve, drained
//! by one thread (`exchange::spawn_nerve`), so the door never dials. A
//! nerve is built from a [`Sealed`] and from nothing else, so no door that
//! speaks on it, and no act it serves, exists before the seal. The queue
//! holds what was spoken and not yet sent while the bytes it holds fit the
//! process's memory `M_proc`; an act past that is refused (dropped, as a
//! full reflex pool defers a demand), never queued without bound. The bound
//! is counted in bytes, never laid out as channel slots.

use mycelium::physics::{Memory, Sealed};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::Arc;

/// One act spoken to a peer: the peer's address and the act's body.
pub(crate) type Spoken = (String, Vec<u8>);

/// The bytes a spoken act holds on the queue: its address and its body.
fn held_by(spoken: &Spoken) -> Option<u64> {
    u64::try_from(spoken.0.len()).ok()?.checked_add(u64::try_from(spoken.1.len()).ok()?)
}

/// The speaking end: queues an act, or refuses it where the nerve's bytes
/// would pass the process's memory.
#[derive(Clone)]
pub(crate) struct Nerve {
    tx: Sender<Spoken>,
    queued: Arc<AtomicU64>,
    memory: Memory,
}

/// The draining end, read by the nerve thread; each act read leaves the
/// count.
pub(crate) struct NerveRx {
    rx: Receiver<Spoken>,
    queued: Arc<AtomicU64>,
}

impl Nerve {
    /// THE NERVE OF A SEALED PROCESS: bounded by `M_proc`, the seal's memory.
    pub(crate) fn of(sealed: &Sealed) -> (Nerve, NerveRx) {
        let (tx, rx) = std::sync::mpsc::channel();
        let queued = Arc::new(AtomicU64::new(0));
        (Nerve { tx, queued: Arc::clone(&queued), memory: sealed.calibration().memory() }, NerveRx { rx, queued })
    }

    /// Queue `spoken` without blocking: `false` where its bytes would pass
    /// the process's memory, or where the draining thread has gone.
    pub(crate) fn try_send(&self, spoken: Spoken) -> bool {
        let Some(bytes) = held_by(&spoken) else { return false };
        let memory = self.memory;
        let fits = |q: u64| q.checked_add(bytes).filter(|&n| memory.holds(Some(n)));
        if self.queued.fetch_update(Ordering::AcqRel, Ordering::Acquire, fits).is_err() {
            return false;
        }
        if self.tx.send(spoken).is_err() {
            self.queued.fetch_sub(bytes, Ordering::AcqRel);
            return false;
        }
        true
    }
}

impl Iterator for NerveRx {
    type Item = Spoken;

    /// The next act spoken, waiting for one; `None` once every speaking end
    /// has gone.
    fn next(&mut self) -> Option<Spoken> {
        let spoken = self.rx.recv().ok()?;
        if let Some(bytes) = held_by(&spoken) {
            self.queued.fetch_sub(bytes, Ordering::AcqRel);
        }
        Some(spoken)
    }
}
