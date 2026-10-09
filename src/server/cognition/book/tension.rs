// 23.8, 26 — TENSION: what a listing is under, and the field the market
// reads it from.
//
//! The field is written from the listing's own deflation and its stagnancy,
//! per lane. Nothing is capped and nothing is defaulted: a lane with no
//! measurement is empty, which is not the same as zero.

use crate::cognition::*;

impl Kernel {
    pub(crate) fn recompute_tension(&mut self, addr: &[u8]) {
        let t = if self.open.contains(addr) {
            self.field
                .get(addr)
                .filter(|f| f.nonflat > 0)
                .map(|f| element::measure_lanes(&union_remainders(&f.entries)))
        } else {
            self.book_elem(addr)
                .map(|p| p.magnitude())
                .or_else(|| self.cavity_book.get(addr).map(|e| e.magnitude()))
        };
        match t {
            Some(t) => {
                self.tension.insert(addr, t);
            }
            None => {
                self.tension.remove(addr);
            }
        }
    }

    #[expect(
        clippy::manual_unwrap_or_default,
        reason = "rule 11 — `unwrap_or_default` is the spelling this refuses; a listing with no field yet holds an empty one, the reading of absence on a keyed carrier"
    )]
    pub(crate) fn write_field(
        &mut self,
        addr: &[u8],
        chart: crate::transducer::stalk::Invariant,
        state: crate::transducer::stalk::Closure,
        remainder: Vec<(i64, i64, u8, u8)>,
        before: &[u64; 4],
        after: &[u64; 4],
    ) {
        use crate::transducer::stalk::Closure;
        // A listing with no field yet stands on an empty one; this is
        // where its first entry is written.
        let mut listing =
            match self.field.get(addr).cloned().or_else(|| self.field_cold.remove(addr)) {
                Some(f) => f,
                None => ListingField::default(),
            };
        for e in &mut listing.entries {
            if e.state.is_flat() {
                continue;
            }
            for l in 0..4 {
                if before[l] > 0 && after[l] == before[l] {
                    #[expect(clippy::disallowed_methods, reason = "rule 11 — one per re-measure event in one process: a u64 cannot pass 2⁶⁴ events before the process ends")]
                    let n = e.stagnant[l].saturating_add(1);
                    e.stagnant[l] = n;
                } else {
                    e.stagnant[l] = 0;
                }
            }
        }
        let mut stag = [0u64; 4];
        // index: the four lanes
        for l in 0..4 {
            if before[l] > 0 && after[l] == before[l] {
                stag[l] = 1;
            }
        }
        let now_flat = state.is_flat();
        let idxs: Vec<usize> = listing
            .entries
            .iter()
            .enumerate()
            .filter(|(_, e)| e.chart == chart)
            .map(|(i, _)| i)
            .collect();
        if !idxs.is_empty() {
            for i in idxs {
                let e = &mut listing.entries[i];
            if !e.state.is_flat() {
                stag = e.stagnant;
            }
            if e.state.is_flat() != state.is_flat() && !state.is_flat() {
                // keep atlas admission for a newly opened chart
            }
            if matches!(
                (&e.state, &state),
                (Closure::Twisted(_) | Closure::Class(..) | Closure::Cavity, Closure::Flat)
            ) {
                let base = self.listing_atlas.chart_of(addr).cloned();
                if base.as_ref() != Some(&chart) {
                    self.listing_atlas.retract(&chart, addr);
                }
            }
            let was_flat = e.state.is_flat();
            e.state = state.clone();
            e.remainder = remainder.clone();
            e.stagnant = stag;
            if was_flat && !now_flat {
                #[expect(clippy::disallowed_methods, reason = "rule 11 — one per resident entry: a count of what this listing holds cannot pass a u64")]
                let n = listing.nonflat.saturating_add(1);
                listing.nonflat = n;
            } else if !was_flat && now_flat {
                // The entry was counted as non-flat, so the count carries
                // it; a count at zero holding one is broken and stays.
                if let Some(next) = listing.nonflat.checked_sub(1) {
                    listing.nonflat = next;
                }
            }
            }
        } else {
            if !state.is_flat() {
                #[expect(clippy::disallowed_methods, reason = "rule 11 — one per resident entry: a count of what this listing holds cannot pass a u64")]
                let n = listing.nonflat.saturating_add(1);
                listing.nonflat = n;
            }
            listing.entries.push(FieldEntry {
                chart: chart.clone(),
                state,
                remainder,
                stagnant: stag,
            });
        }
        if listing.nonflat > 0 {
            self.open.insert(addr, ());
            self.field.insert(addr, listing);
        } else {
            self.open.remove(addr);
            self.field_cold.insert(addr, listing);
            self.field.remove(addr);
        }
        // A listing with no recorded tension stands at what this reading
        // just measured on it.
        let mag = match self.tension.get(addr) {
            Some(t) => *t,
            None => *after,
        };
        let lanes = crate::network::nerve::occupied_lanes(&mag);
        // index: the four lanes
        for lane in &lanes {
            if now_flat {
                self.own_cover.retract(*lane, &chart, addr);
            } else {
                self.own_cover.admit(*lane, &chart, addr);
            }
        }
        // 78.153b: the shelf copies the charts again only where they moved.
        // A count of moves, compared for equality alone: it wraps, never
        // ordered.
        self.own_cover_gen = self.own_cover_gen.wrapping_add(1);
        let ck = chart.key();
        match self.chart_listings.get_mut(&ck) {
            Some(m) => {
                if now_flat {
                    m.remove(addr);
                } else {
                    m.insert(addr, ());
                }
            }
            None if !now_flat => {
                let mut m = crate::network::nerve::Keyed::new();
                m.insert(addr, ());
                self.chart_listings.insert(ck, m);
            }
            None => {}
        }
        self.recompute_tension(addr);
    }

    /// The remainder face of a listing: the unique non-flat chart. Several
    /// open charts are the field, not a representative.
    pub fn face_of(&self, addr: &[u8]) -> Option<&crate::transducer::stalk::Invariant> {
        self.field
            .get(addr)
            .or_else(|| self.field_cold.get(addr))
            .and_then(|f| {
                if f.nonflat == 0 {
                    return None;
                }
                let mut one = None;
                for e in &f.entries {
                    if e.state.is_flat() {
                        continue;
                    }
                    if one.is_some() {
                        return None;
                    }
                    one = Some(&e.chart);
                }
                one
            })
    }

    /// Topological volume: cards, fibre span, open listings. K is algebraic
    /// time and is not a persist event — ticking it must not flush the ledger.
    /// The fibre span in the period's carrier (78.43(2)), `None` past it.
    pub fn volume(&self) -> (usize, Option<u64>, usize) {
        (self.basis.len(), self.fibre_volume_total(), self.pending.len())
    }

    /// The polytonic kinematic pressure + torsion spectrum (per lane).
    pub fn physics(&self) -> &crate::physics::Physics {
        &self.physics
    }

    /// ONE STROKE OF THIS COURT'S OWN METABOLISM (rule 10, 45.7): the
    /// `Budget` a page, an offer round or an idle wait is paced by. The
    /// binaries drew it from `Physics::default()`, a fresh default standing
    /// in for the kernel's measured reserve (rule 5); what a court can
    /// afford in one stroke is read off the court.
    ///
    /// 78.153b — THE CRATE'S, NEVER THE BINARY'S: a binary takes its stroke
    /// with the kernel, one per hold (`server::held::lock`), so a hold cannot
    /// mint itself a second.
    pub(crate) fn stroke(&mut self) -> crate::physics::Budget {
        self.physics.stroke()
    }


    /// Crate-internal: the deliberative arm (synthesis::contemplate) spends
    /// the reserve through here — thought is metabolically real.
    #[cfg(test)]
    pub(crate) fn physics_mut(&mut self) -> &mut crate::physics::Physics {
        &mut self.physics
    }

    /// A VOICE HAS CLOSED ONE CHART OF A LISTING, and the transport it
    /// closed by: the flat orbit, filed once per (chart, voice). Recorded
    /// beside the field because it is a reading of the same settlement.
    pub(crate) fn record_flat(
        &mut self,
        listing: &[u8],
        chart: crate::transducer::stalk::Invariant,
        voice: [u8; 32],
        g: element::Versor,
    ) {
        let key = crate::cognition::orbit_key(&chart, &voice);
        if self.flat_orbits.get(listing).is_some_and(|m| m.contains(&key)) {
            return;
        }
        match self.flat_orbits.get_mut(listing) {
            Some(m) => {
                m.insert(key, (chart, g));
            }
            None => {
                let mut m = crate::network::nerve::Keyed::new();
                m.insert(key, (chart, g));
                self.flat_orbits.insert(listing, m);
            }
        }
    }

}
