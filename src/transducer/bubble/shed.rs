// 24.1, 25 — THE SHED: what a mind stops holding when the room is spent.
//
//! T room is measured in receipts, never in seconds (law 9): the oldest
//! bubble by receipt index moves to the cavity when the room is short, and
//! a bubble nothing glues is shed outright.

use super::*;

/// THE KEY A RETIRED REMAINDER IS FILED UNDER: its chart, then the bubble it
/// was. The chart leads because it is what `pivot_on` asks at — a word
/// arrives on a chart and the cavity entries that chart could re-hold are the
/// subtree under it, one keyed descent. The bubble key follows so two
/// remainders retired at one chart stay two remainders (law 7).
///
/// `stalk::invariant` of the remainder links is the reading `pivot_on` asks
/// at; making it the key puts that derivation at the write, where it
/// happens once, and not at the read, where it would happen for every
/// entry on every call.
pub(crate) fn cavity_key(rem: &[Link], bubble: &[u8]) -> Vec<u8> {
    let inv = stalk::invariant(rem);
    let mut k = inv.key().into_vec();
    crate::transducer::address::put_bytes(&mut k, bubble);
    k
}

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
        for (k, b) in self.bubbles.scoped() {
            if !b.glued_by.is_empty() {
                continue;
            }
            // The receipts since the bubble's charts last moved — the FIFO
            // head by the kernel's own K (law 9), zero when it moved now. A
            // bubble marked above this clock is not shed against a count
            // that went backwards.
            let Some(t) = super::idea::held_since(now, b.moved_k) else {
                continue;
            };
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
            // THE CAVITY IS KEYED BY THE CHART ITS REMAINDER STANDS ON, then
            // by the bubble's own key. `pivot_on` reads it back when a word
            // arrives on a matching chart, so the CHART is the coordinate
            // that read asks at; keyed by the bubble alone it could not ask.
            //
            // The bubble key stays as the suffix. Two remainders retired at
            // one chart are two remainders and law 7 does not lose either;
            // collapsing them onto the chart alone would.
            self.cavity.insert(cavity_key(&b.remainder, &k), b.remainder.to_vec());
        }
    }

    /// Shed ideas no live glue holds. Returns how many bubbles left.
    pub fn shed_unglued(&mut self) -> usize {
        let drop: Vec<Vec<u8>> = self
            .bubbles
            .scoped()
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
