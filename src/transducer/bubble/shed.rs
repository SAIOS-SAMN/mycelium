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
    // `retire_on_t(room)` STOOD HERE AND IS DELETED (25, Z4b).
    //
    // It retired ONE bubble per stroke, the one with the largest T extent,
    // and only when `self.bubbles.len() > room` — with `room` being
    // `physics.reserve[2]`. PROBED on this box: `reserve [88, 88, 88, 88]`,
    // `reserve_max 88`, and `reserve_max` is
    // `((cores × (1 + MemAvailable_GiB)).max(16) / 8) × 8` read at boot. So
    // the only live bound on the contemplation was the machine's memory,
    // and the fleet's holds reached 47 against a bound of 88 — the trigger
    // never fired at all.
    //
    // Two faults, not one. A COUNT against a bound is rule 2's, and the
    // bound being the box's RAM is rule 5's; and choosing the oldest is
    // rule 4's, a selection by a scalar. The three doors replace it: a hold
    // lapses with what it cites (door 1), is retired at its promotion
    // (door 2), and a lane with no room WITHHOLDS a new thought rather than
    // evicting a held one (door 3). None of the three reads a count, a
    // size, or an age.
    //
    // The cavity is unchanged and still fed: `lapse_with` leaves every
    // retirement's remainder at its chart, which is what `pivot_on` reads
    // back.

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

    /// Z4d — THE RECEIPT IS THE ESCROW: every thought whose paid height the
    /// clock has passed lapses, AS A SET, and its escrow returns.
    ///
    /// A thought stands while its receipts are ahead of TUniversal and
    /// leaves when the clock passes it. NOTHING IS CHOSEN: the set is every
    /// bubble whose `until` the clock has passed, taken together on the
    /// stroke that reads the height, never one by age, size or arrival
    /// (rules 4 and 15). That is what separates a lapse from an eviction.
    ///
    /// A thought with no `until` is one held before any aperture was
    /// measured. It does not lapse here — an unmeasured horizon is held,
    /// never guessed (rule 15) — and it is paid to a height by `repay` on
    /// the first stroke that can measure one.
    ///
    /// Returns the escrow returned, per lane, and the count. The escrow is
    /// a VECTOR and is never summed (law 5): a voice that lapses a thought
    /// rich in `𝒽` gets its turns back and not its placements.
    pub fn lapse_on_clock(&mut self) -> ([u64; 4], usize) {
        let now = self.now_k;
        let doomed: Vec<Vec<u8>> = self
            .bubbles
            .scoped()
            .filter(|(_, b)| b.until.is_some_and(|u| u <= now))
            .map(|(k, _)| k)
            .collect();
        let mut returned = [0u64; 4];
        let n = doomed.len();
        for k in &doomed {
            if let Some(b) = self.bubbles.remove(k) {
                for (acc, e) in returned.iter_mut().zip(&b.escrow) {
                    *acc = acc.saturating_add(*e);
                }
                self.retract_incidence(k, &b);
                self.dirty.remove(k);
                self.cavity.insert(cavity_key(&b.remainder, k), b.remainder.to_vec());
            }
        }
        (returned, n)
    }

    /// Z4d — A THOUGHT HELD BEFORE ANY APERTURE WAS MEASURED IS PAID NOW.
    /// The first stroke that can measure a horizon pays every unpaid
    /// thought through it, so a thought is never held against a height
    /// nothing measured and never held forever for want of one.
    pub fn repay_unpaid(&mut self) -> usize {
        let Some(span) = self.aperture_tu() else {
            return 0;
        };
        let Some(until) = self.now_k.checked_add(span) else {
            return 0;
        };
        let unpaid: Vec<Vec<u8>> =
            self.bubbles.scoped().filter(|(_, b)| b.until.is_none()).map(|(k, _)| k).collect();
        let n = unpaid.len();
        for k in unpaid {
            if let Some(b) = self.bubbles.get_mut(&k) {
                b.until = Some(until);
            }
        }
        n
    }

    /// Z4d — RENEWAL. A thought a court has answered for — standing what it
    /// cites, or listing what it asks — is paid through another aperture
    /// from the height that answer was read at. Renewal is the only thing
    /// that moves `until` forward, so a thought nothing answers for lapses
    /// on schedule however long it has stood.
    pub fn renew(&mut self, key: &[u8]) -> bool {
        let Some(span) = self.aperture_tu() else {
            return false;
        };
        let Some(until) = self.now_k.checked_add(span) else {
            return false;
        };
        match self.bubbles.get_mut(key) {
            Some(b) => {
                b.until = Some(until);
                true
            }
            None => false,
        }
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
