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

    /// DOOR 1 OF CHAPTER 25 — LAPSE, ALONG THE CITATION FIBRATION.
    ///
    /// A bubble stands while what it cites stands. When an address lapses
    /// from the working atlas — the dialect shed it, or the court retired
    /// it — the bubble AT that address and every bubble CITING it are
    /// retired on the same stroke, as a set.
    ///
    /// JOINTLY, NOT BY A LATER SWEEP. A pin that outlives its target is a
    /// dead reference, and a sweep that discovers dead references after
    /// the fact holds them for however long the sweep takes to come round.
    /// The lapse of the cited address is the signal, and it is already in
    /// the caller's hand: `Epigenome::shed_except` returns exactly the
    /// addresses it dropped.
    ///
    /// AND IT IS TRANSITIVE. A cites B and B cites C: when C lapses, B goes
    /// with it and A goes with B, in this call and not on three successive
    /// strokes. The frontier is walked until it adds nothing, which is a
    /// walk of the fibration's own boundary (rule 1) and not a search.
    ///
    /// MEASURED before it was written (the 60-task slice): 21 of 51 held
    /// bubbles cite, every one of their 21 pins is LIVE, and none is dead.
    /// So a dead-pin collector would retire nothing and this door is the
    /// only one that reaches them.
    ///
    /// Returns how many bubbles left.
    pub fn lapse_with(&mut self, lapsed: &[Vec<u8>]) -> usize {
        let mut frontier: Vec<Vec<u8>> = lapsed.to_vec();
        let mut doomed: Keyed<()> = Keyed::new();
        while let Some(addr) = frontier.pop() {
            // The bubble standing AT the lapsed address.
            if self.bubbles.contains(&addr) && !doomed.contains(&addr) {
                doomed.insert(&addr, ());
                frontier.push(addr.clone());
            }
            // And every bubble citing it, by keyed read of the fibration.
            let citers: Vec<u16> = match self.cited.get(&addr) {
                Some(m) => m.scoped().filter_map(|(k, _)| Self::ordinal_of_key(&k)).collect(),
                None => Vec::new(),
            };
            for o in citers {
                let Some(k) = self.address_at(o).map(<[u8]>::to_vec) else { continue };
                if doomed.contains(&k) {
                    continue;
                }
                doomed.insert(&k, ());
                frontier.push(k);
            }
        }
        let keys: Vec<Vec<u8>> = doomed.scoped().map(|(k, _)| k).collect();
        let n = keys.len();
        for k in &keys {
            if let Some(b) = self.bubbles.remove(k) {
                self.retract_incidence(k, &b);
                self.dirty.remove(k);
                // What it could not close goes to the cavity at its chart,
                // as a retirement has always left it, so a word arriving on
                // that chart can read it back (`pivot_on`).
                self.cavity.insert(cavity_key(&b.remainder, k), b.remainder.to_vec());
            }
        }
        n
    }

    /// Shed ideas no live glue holds. Returns how many bubbles left.
    /// Why a bubble was not shed, by the term that rejected it. A
    /// measurement (§8.1), printed only under `MYC_TRACE`, never a branch.
    pub fn shed_census(&self) -> [usize; 4] {
        let mut c = [0usize; 4];
        for (_, b) in self.bubbles.scoped() {
            if !b.glued_by.is_empty() {
                c[0] += 1;
            } else if !b.glues.is_empty() {
                c[1] += 1;
            } else if b.dirty {
                c[2] += 1;
            } else {
                c[3] += 1;
            }
        }
        c
    }

    pub fn shed_unglued(&mut self) -> usize {
        if std::env::var("MYC_TRACE").is_ok_and(|v| v != "0") {
            let c = self.shed_census();
            eprintln!(
                "  shed census held {} glued_by {} glues {} dirty {} sheddable {}",
                self.bubbles.len(), c[0], c[1], c[2], c[3]
            );
        }
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
