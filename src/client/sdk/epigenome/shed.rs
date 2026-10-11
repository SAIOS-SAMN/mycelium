// 24.1 — THE SHED: what this client stops holding, and the aperture in
// receipts that decides when (law 9).

use crate::client::sdk::*;

impl Epigenome {
    /// A working-atlas read of a stood word is a use, stamped with the
    /// mind's OWN K (45.4). The height beside it is standing extent and is
    /// left alone; the lifecycle is the mind's, and a mind that hears
    /// nothing still thinks.
    pub(crate) fn touch(&mut self, addr: &[u8], now_k: u64) {
        if let Some(d) = self.dialect.get_mut(addr) {
            d.last_used_k = now_k;
            #[expect(clippy::disallowed_methods, reason = "rule 11 — one per use: a u64 of events cannot pass 2⁶⁴")]
            let n = d.uses.saturating_add(1);
            d.uses = n;
        }
    }

    /// Drop dialect unused for an aperture of the MIND'S OWN K (45.4, 78.49:
    /// the mind sheds by its own orbits, never by the mesh's height). A
    /// dialect entry a live idea glues is not shed. The genome never sheds.
    ///
    /// Read against TUniversal this could not fire at all on a mesh that
    /// admitted nothing: heights advance only on admitted receipts (45.2),
    /// so a fleet refusing work also could not reclaim what it held, and the
    /// two failures compounded into a client the kernel killed. A mind's own
    /// K advances on its own acts, so truncation is independent of whether
    /// anyone answered it.
    #[cfg(test)]
    pub(crate) fn shed(&mut self, now_k: u64) -> usize {
        self.shed_except(now_k, &|_| false).len()
    }

    pub(crate) fn note_stroke(&mut self, now_k: u64) {
        let n = crate::physics::aperture_cycle() as usize;
        self.stroke_k.push(now_k);
        if self.stroke_k.len() > n {
            self.stroke_k.remove(0);
        }
    }

    /// The span of the last aperture of strokes IN THIS MIND'S OWN K, one
    /// unit, measured. This line read "TUniversal span", which is the clock
    /// `shed` above rules OUT in its own words — 45.4: the mind sheds by its
    /// own K, never by the mesh's height — and every caller passes
    /// `Entity::own_k()`. A doc naming the other clock is how the two get
    /// crossed, and they were: the contemplation's `now_k` says TUniversal
    /// and was being stamped with the mind's K.
    ///
    /// `None` until a whole aperture of strokes has been noted: an
    /// unmeasured aperture is held, never guessed (rule 15), so nothing
    /// sheds in the first aperture after boot.
    pub(crate) fn aperture_k(&self, now_k: u64) -> Option<u64> {
        let n = crate::physics::aperture_cycle() as usize;
        if self.stroke_k.len() < n {
            return None;
        }
        #[expect(
            clippy::disallowed_methods,
            reason = "rule 15 — FIFO drain head: the window is filled in stroke order, so its head is the oldest stroke in it"
        )]
        let first = *self.stroke_k.first()?;
        // The window's first stroke is a K this mind has passed, so the
        // aperture is exact; a first stroke ahead of now is a broken clock
        // and measures no aperture.
        now_k.checked_sub(first)
    }

    /// Returns the addresses shed, so the carriers that hang off a dialect
    /// word — its utterance receipt above all (S12.6a) — leave with it.
    pub(crate) fn shed_except(&mut self, now_k: u64, glued: &impl Fn(&[u8]) -> bool) -> Vec<Vec<u8>> {
        let Some(aperture_k) = self.aperture_k(now_k) else {
            return Vec::new();
        };
        // Every dialect entry is asked whether it has stood past the
        // aperture -- that IS the shed -- and the filter is the reading.
        let drop: Vec<Vec<u8>> = self
            .dialect
            .scoped()
            // An entry last used at a K this mind has not reached is not
            // idle; only what has stood past the aperture is shed.
            .filter(|(a, d)| {
                now_k.checked_sub(d.last_used_k).is_some_and(|idle| idle > aperture_k)
                    && !glued(a)
            })
            .map(|(a, _)| a)
            .collect();
        for a in &drop {
            self.drop_dialect(a);
        }
        #[expect(clippy::disallowed_methods, reason = "rule 11 — one per resident entry shed: a u64 of events cannot pass 2⁶⁴")]
        let n = self.shed_n.saturating_add(drop.len() as u64);
        self.shed_n = n;
        drop
    }

    /// FX1, alternative B — A SEALED FORM'S CIRCUIT RAN THROUGH THESE WORD
    /// VERTICES at the voice's `k`: the dialect words among them are
    /// traversed.
    pub(crate) fn note_traversed(&mut self, words: &[Vec<u8>], k: u64) {
        for w in words {
            if self.dialect.contains(w) {
                self.traversed_k.insert(w, k);
                self.traversed_orbit.insert(w, ());
            }
        }
    }

    /// FX1, alternative B — AT THE ORBIT'S CLOSE: the dialect words traversed
    /// this orbit, those traversed over the aperture (`aperture_k`, the
    /// idle-K door's own window; `None` where it has not been measured), and
    /// the idle ones — dialect words no circuit ran through over the window.
    /// The orbit's set is emptied.
    pub(crate) fn dialect_flux(&mut self, now_k: u64) -> (u64, Option<u64>, Option<u64>) {
        let orbit = std::mem::take(&mut self.traversed_orbit).len() as u64;
        let Some(aperture) = self.aperture_k(now_k) else {
            return (orbit, None, None);
        };
        let within = self
            .dialect
            .scoped()
            .filter(|(a, _)| self.traversed_k.get(a).is_some_and(|&k| now_k.checked_sub(k).is_some_and(|d| d <= aperture)))
            .count() as u64;
        (orbit, Some(within), Some(self.dialect.len() as u64 - within))
    }

}
