// 78.151 — THE SHELF: the court's published reading, swapped whole by the
// court stroke and read by the door without the kernel.
//
//! A reader takes the reading standing on the shelf — one pointer, cloned
//! under a read lock held for that clone alone — and answers off it at its
//! height. The court stroke publishes under the kernel it already holds and
//! swaps the pointer under the write lock, held for the swap alone, so no
//! reader ever sees a reading half-written. A write by any act is read at
//! the next stroke's publication: a writer reading its own write lags it by
//! at most one court stroke.

use mycelium::cognition::{Kernel, Published};
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub(crate) struct Shelf(Arc<RwLock<Arc<Published>>>);

impl Shelf {
    /// The shelf holding a court's first reading, published at boot.
    pub(crate) fn new(first: Published) -> Shelf {
        Shelf(Arc::new(RwLock::new(Arc::new(first))))
    }

    /// The reading standing now. A poisoned lock still holds a whole
    /// reading — a swap is one pointer written — so it is read on.
    pub(crate) fn take(&self) -> Arc<Published> {
        match self.0.read() {
            Ok(g) => Arc::clone(&g),
            Err(p) => Arc::clone(&p.into_inner()),
        }
    }

    /// THE COURT STROKE'S PUBLICATION, under the kernel it holds: built off
    /// the reading standing, swapped whole where anything moved, and its cost
    /// counted on the observer's serving reading.
    pub(crate) fn publish(&self, k: &mut Kernel) {
        let standing = self.take();
        let Some(next) = k.publish(Some(&standing)) else { return };
        note_cost(&next);
        let next = Arc::new(next);
        match self.0.write() {
            Ok(mut g) => *g = next,
            Err(p) => *p.into_inner() = next,
        }
    }
}

/// THE PUBLICATION'S COST on the observer (`Court::serve`): one count per
/// publication, the entries and bytes it copied, and the most entries one
/// publication copied — counts of the work, never a clock (law 9).
pub(crate) fn note_cost(p: &Published) {
    use mycelium::observe::{note_serve, ServeAxis};
    let c = p.cost();
    note_serve(ServeAxis::Publishes, 1);
    note_serve(ServeAxis::PublishedReceipts, c.receipts);
    note_serve(ServeAxis::PublishedEras, c.eras);
    note_serve(ServeAxis::PublishedAccounts, c.accounts);
    note_serve(ServeAxis::PublishedKeys, c.keys);
    note_serve(ServeAxis::PublishedBytes, c.bytes);
    match c.entries() {
        Some(n) if !c.passed => note_serve(ServeAxis::MostPublished, n),
        _ => note_serve(ServeAxis::PublishPassed, 1),
    }
}
