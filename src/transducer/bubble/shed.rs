// 24.1, 25 — THE SHED: what a mind stops holding when the room is spent.
//
//! T room is measured in receipts, never in seconds (law 9): the oldest
//! bubble by receipt index moves to the cavity when the room is short, and
//! a bubble nothing glues is shed outright.

use super::*;

impl Contemplation {
    /// When T-room on the entity's reserve is exhausted, the bubble with
    /// the largest T magnitude (ticks since its charts last moved) goes to
    /// a cavity chart; pivot_on reads it back when its chart moves.
    pub fn retire_on_t(&mut self, room: usize) {
        // A room of zero is no room (L7b, rule 5): nothing is floored so
        // that one bubble is always kept; the reserve says how many stand.
        if self.bubbles.len() <= room {
            return;
        }
        let now = self.now_k;
        let mut best: Option<(Vec<u8>, u64)> = None;
        for (k, b) in self.bubbles.fiber(ROOT) {
            if !b.glued_by.is_empty() {
                continue;
            }
            // The receipts since the bubble's charts last moved — the FIFO
            // head by the kernel's own K (law 9), zero when it moved now.
            let t = now.saturating_sub(b.moved_k);
            match best {
                Some((_, bt)) if t <= bt => {}
                _ => best = Some((k.clone(), t)),
            }
        }
        let Some((k, _)) = best else {
            return;
        };
        if let Some(b) = self.bubbles.remove(&k) {
            self.retract_incidence(&k, &b);
            self.dirty.remove(&k);
            self.cavity.insert(k, b.remainder);
        }
    }

    /// Shed ideas no live glue holds. Returns how many bubbles left.
    pub fn shed_unglued(&mut self) -> usize {
        let drop: Vec<Vec<u8>> = self
            .bubbles
            .fiber(ROOT)
            .filter(|(_, b)| b.glued_by.is_empty() && b.glues.is_empty() && !b.dirty)
            .map(|(k, _)| k)
            .collect();
        let n = drop.len();
        for k in &drop {
            if let Some(b) = self.bubbles.remove(k) {
                self.retract_incidence(k, &b);
                self.dirty.remove(k);
            }
        }
        n
    }

}
