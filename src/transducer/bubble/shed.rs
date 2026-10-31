// 24.1, 25 — THE SHED: what a mind stops holding when the room is spent.
//
//! T room is measured in receipts, never in seconds (law 9): the oldest
//! bubble by receipt index moves to the cavity when the room is short, and
//! a bubble nothing glues is shed outright.

use super::*;

/// THE KEY A RETIRED REMAINDER IS FILED UNDER: its chart, then the bubble it
/// was. The chart leads because it is what `pivot_on` asks at — a word
/// arrives on a chart and the cavity entries that chart could re-hold are the
/// subtree under it, one keyed read. The bubble key follows so two
/// remainders retired at one chart stay two remainders (law 7).
///
/// `stalk::invariant` of the remainder links is the reading `pivot_on` asks
/// at; making it the key puts that derivation at the write, where it
/// happens once, and not at the read, where it would happen for every
/// entry on every call.
pub(crate) fn cavity_key(rem: &[Link], bubble: &[u8]) -> Vec<u8> {
    let inv = stalk::invariant(rem);
    let mut k = inv.key().into_vec();
    crate::transducer::address::put_key_bytes(&mut k, bubble);
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
    // and the deployment's holds reached 47 against a bound of 88 — the trigger
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
    /// strokes. The frontier is traversed until it adds nothing, which is a
    /// read of the fibration's own boundary (rule 1) and not a search.
    ///
    /// MEASURED before it was written (the 60-task slice): 21 of 51 held
    /// bubbles cite, every one of the 21 bubbles' pins is LIVE, and none is dead.
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
            let citers: Vec<u16> = match self.cited.get(&Self::cited_at(&addr)) {
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
        // TC1 — what this door takes that carries a loop, read around it.
        let before = self.shed_before(&keys);
        let touched = self.touched_by(&keys);
        for k in &keys {
            if let Some(b) = self.bubbles.remove(k) {
                self.retract_incidence(k, &b);
                self.dirty.remove(k);
                self.unthreaded.remove(k);
                self.unheld.remove(k);
                // What it could not close goes to the cavity at its chart,
                // as a retirement has always left it, so a word arriving on
                // that chart can read it back (`pivot_on`); and the escrow it
                // held returns, once.
                self.cavity.insert(cavity_key(&b.remainder, k), b.remainder.to_vec());
                self.return_escrow(&b.escrow);
            }
        }
        self.shed_after(super::core::Door::Citation, n, before);
        // The removal moved degrees: the touched components truncate.
        if n > 0 {
            self.truncate(&touched);
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
        // THE UNGLUED DOOR ACTS ON THOUGHTS ONLY (N4): a perception never
        // passes it — it leaves by the physics, its line let go by the field
        // (`line`) or what it cites lapsed — and a thought's glue is also the
        // transition that formed it, an edge of the meet graph.
        let drop: Vec<Vec<u8>> = self
            .bubbles
            .scoped()
            .filter(|(k, b)| {
                !b.percept
                    && b.glued_by.is_empty()
                    && b.glues.is_empty()
                    && !b.dirty
                    && self.complex.link(k, 1).is_empty()
            })
            .map(|(k, _)| k)
            .collect();
        let n = drop.len();
        // TC1 — what this door takes that carries a loop, read around it.
        let before = self.shed_before(&drop);
        let touched = self.touched_by(&drop);
        for k in &drop {
            if let Some(b) = self.bubbles.remove(k) {
                self.retract_incidence(k, &b);
                self.dirty.remove(k);
                self.unthreaded.remove(k);
                self.unheld.remove(k);
                // The escrow it held returns, once.
                self.return_escrow(&b.escrow);
            }
        }
        self.shed_after(super::core::Door::Unglued, n, before);
        if n > 0 {
            self.truncate(&touched);
        }
        n
    }

}
