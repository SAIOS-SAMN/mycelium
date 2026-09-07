// 23.4 — TENSION: what a listing is under, and the field the market reads
// it from.
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
        let mut listing = self
            .field
            .get(addr)
            .cloned()
            .or_else(|| self.field_cold.remove(addr))
            .unwrap_or_default();
        for e in &mut listing.entries {
            if e.state.is_flat() {
                continue;
            }
            for l in 0..4 {
                if before[l] > 0 && after[l] == before[l] {
                    e.stagnant[l] = e.stagnant[l].saturating_add(1);
                } else {
                    e.stagnant[l] = 0;
                }
            }
        }
        let mut stag = [0u32; 4];
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
                listing.nonflat = listing.nonflat.saturating_add(1);
            } else if !was_flat && now_flat {
                listing.nonflat = listing.nonflat.saturating_sub(1);
            }
            }
        } else {
            if !state.is_flat() {
                listing.nonflat = listing.nonflat.saturating_add(1);
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
        let mag = self.tension.get(addr).copied().unwrap_or(*after);
        let lanes = crate::network::nerve::occupied_lanes(&mag);
        // index: the four lanes
        for lane in &lanes {
            if now_flat {
                self.own_cover.retract(*lane, &chart, addr);
            } else {
                self.own_cover.admit(*lane, &chart, addr);
            }
        }
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
    pub fn volume(&self) -> (usize, usize, usize) {
        (self.basis.len(), self.sections_total(), self.pending.len())
    }

    /// The polytonic kinematic pressure + torsion spectrum (per lane).
    pub fn physics(&self) -> &crate::physics::Physics {
        &self.physics
    }

    /// Crate-internal: the deliberative arm (synthesis::contemplate) spends
    /// the reserve through here — thought is metabolically real.
    #[cfg(test)]
    pub(crate) fn physics_mut(&mut self) -> &mut crate::physics::Physics {
        &mut self.physics
    }

}
