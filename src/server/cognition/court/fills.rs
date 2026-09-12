// 23.7, 42 — WHAT ONE STROKE OF THE COURT FILLS, PAYS AND FILES: the
// unfilled cells and the pending refinements, the volume a fill is paid
// by, the torsion a congruent cell files.
//
//! Split from `replay.rs` (MU-STANDARDS 1.1, one organ per file): the
//! court's stroke rules the docket there and fills here, on the same
//! stroke Budget.

use crate::cognition::*;
use crate::transducer::address::Address;
use crate::transducer::element;

impl Kernel {
    pub(crate) fn stroke_fills(&mut self, budget: &mut crate::physics::Budget) {
        if budget.remaining() == 0 {
            return;
        }
        let mut pending: Vec<(crate::transducer::stalk::Invariant, crate::transducer::address::Address)> = Vec::new();
        {
            // Resumed, never restarted at the first key (rule 10, S13.3):
            // a head that refills faster than one budget no longer starves
            // the tail, and the orbit reaches every cell.
            let from = self.fill_cursor.clone();
            let mut cur = self.unfilled.cursor(&from, budget);
            for (_, v) in &mut cur {
                pending.push(v.clone());
            }
            if let Some(k) = cur.last_key() {
                self.fill_cursor = k.to_vec();
            }
        }
        for (inv, sim_key) in pending {
            let dkey = chart_simplex_key(&inv, &sim_key);
            self.unfilled.remove(&dkey);
            let filled = {
                let Some(cx) = self.complexes.get_mut(&inv.key()) else {
                    continue;
                };
                let addrs: Vec<Address> = cx
                    .sections
                    .fiber(crate::network::nerve::ROOT)
                    .map(|(_, s)| s.addr.clone())
                    .collect();
                (cx.fill(&sim_key), addrs)
            };
            match filled {
                (crate::transducer::cech::Fill::Filled(gamma, carriage), addrs) => {
                    eprintln!(
                        "  court cell filled {:02x}{:02x}… γ recorded",
                        sim_key[0], sim_key[1]
                    );
                    self.record_gamma(&addrs, &gamma, &carriage);
                    self.pay_cell_volume(&inv, &addrs);
                    if !gamma.is_identity() {
                        if let Some((a, b)) = edge_pair(&addrs) {
                            self.mint_or_admit_translation(&gamma, a.clone(), b.clone(), [0; 32]);
                            let key = Self::cocycle_key(&inv, &a, &b);
                            if let Some(row) = self.cocycle_at.remove(&key) {
                                self.dialect_chain.push((key.clone(), row.gauge, row.t_universal));
                            }
                            if let Some(m) = self.cocycle_by_chart.get_mut(&inv.key()) {
                                m.remove(&key);
                            }
                        }
                    }
                }
                (crate::transducer::cech::Fill::Congruent(_g, _ell), addrs) => {
                    eprintln!(
                        "  court cell congruent {:02x}{:02x}… torsion filed",
                        sim_key[0], sim_key[1]
                    );
                    self.file_torsion(&addrs, _ell);
                    self.pay_cell_volume(&inv, &addrs);
                    self.pay_torse(&addrs);
                }
                (crate::transducer::cech::Fill::Class(_, class), _) => {
                    let class = *class;
                    eprintln!(
                        "  court cell class {:02x}{:02x}…",
                        sim_key[0], sim_key[1]
                    );
                    let degree = self
                        .complexes
                        .get(&inv.key())
                        .and_then(|c| c.class(&sim_key).map(|(_, _, d)| d as u8))
                        // A class the complex does not hold stands at the
                        // edge degree, which is where a fill first names one.
                        .map_or(2, |d| d);
                    self.mark_class(&inv, class.clone(), degree);
                    if degree >= 3 {
                        self.pending_refine.insert(inv.key(), (inv.clone(), class, degree));
                    }
                }
                // 37.7 AT THE COURT: a cell whose transition is a rotation
                // the seventeen do not spell. It fills nothing and files no
                // class, and it is the court's own adjunction candidate —
                // named on its own line so the day one appears is a
                // measurement and not an anecdote.
                (crate::transducer::cech::Fill::Unspellable(_), _) => {
                    eprintln!(
                        "  court cell unspellable {:02x}{:02x}… 37.7",
                        sim_key[0], sim_key[1]
                    );
                }
                (crate::transducer::cech::Fill::Open, _) | (crate::transducer::cech::Fill::Already, _) => {}
            }
        }
        let mut refine: Vec<(
            crate::transducer::stalk::Invariant,
            crate::transducer::alphabet::Rotor,
            u8,
        )> = Vec::new();
        {
            let from = self.refine_cursor.clone();
            let mut cur = self.pending_refine.cursor(&from, budget);
            for (_, v) in &mut cur {
                refine.push(v.clone());
            }
            if let Some(k) = cur.last_key() {
                self.refine_cursor = k.to_vec();
            }
        }
        for (inv, class, degree) in refine {
            let key = inv.key();
            self.pending_refine.remove(&key);
            eprintln!(
                "  court class degree {degree} {:02x}{:02x}… chart refined",
                key[0], key[1]
            );
            self.refine_chart(&inv, class, degree, None);
        }
    }

    /// R6 (47.3): the volume of a cell is the measure of the open listings
    /// on its chart. Its vertices are vantages — hyphas — and no speaker's
    /// index gates it, so the cell's vertex list is not an argument here.
    pub(crate) fn volume_of_cell(&self, inv: &crate::transducer::stalk::Invariant) -> [u64; 4] {
        let held = self.listings_on(inv);
        let mut chain = Vec::new();
        for (a, _) in held.fiber(crate::network::nerve::ROOT) {
            if let Some(f) = self.field.get(&a) {
                chain.extend(union_remainders(&f.entries));
            }
        }
        element::measure_lanes(&chain)
    }

    pub(crate) fn pay_cell_volume<A: AsRef<[u8]>>(&mut self, inv: &crate::transducer::stalk::Invariant, addrs: &[A]) {
        let held = self.listings_on(inv);
        let vol = self.volume_of_cell(inv);
        // Every vertex of the cell is a party to what the cell closed. The
        // volume is one quantity for all of them, so it splits per lane as
        // a consilience award does; paying `addrs.first()` paid whichever
        // member the cell happened to list first (rule 15).
        let parties: Vec<[u8; 32]> = addrs
            .iter()
            .filter_map(|a| <[u8; 32]>::try_from(a.as_ref()).ok())
            .collect();
        if parties.is_empty() {
            return;
        }
        let n = parties.len() as u64;
        let share: [u64; 4] =
            std::array::from_fn(|l| if vol[l] == 0 { 0 } else { (vol[l] / n).max(1) });
        let vol = share;
        let anchor = self.anchor();
        for (a, _) in held.fiber(crate::network::nerve::ROOT) {
            for v in &parties {
                let _ = self.treasury.draw_escrow(&a, *v, vol, anchor);
            }
        }
    }

    /// Each member of the cell is paid the torsion IT measured. Reading the
    /// magnitude off `addrs.first()` made the value of a torsion depend on
    /// which member the cell happened to list first, and paid it to that
    /// member alone; a member with no measured tension is unmeasured and is
    /// paid nothing, never paid another member's reading (rule 11, 15).
    pub(crate) fn pay_torse<A: AsRef<[u8]>>(&mut self, addrs: &[A]) {
        let anchor = self.anchor();
        let parties: Vec<([u8; 32], [u64; 4])> = addrs
            .iter()
            .filter_map(|a| {
                let who = <[u8; 32]>::try_from(a.as_ref()).ok()?;
                let mag = self.tension.get(a.as_ref()).copied()?;
                Some((who, mag))
            })
            .collect();
        for (who, mag) in parties {
            let _ = self.treasury.settle_torse(who, mag, anchor);
        }
    }

    /// Torsion is filed on the cell's LEAST ADDRESS BY KEY. Both callers
    /// hand these over in key order — one from `sorted_verts`, one from a
    /// keyed fiber — so the head is the least key and every node files the
    /// cycle on the same word rather than on whichever section it met
    /// first.
    pub(crate) fn file_torsion<A: AsRef<[u8]>>(&mut self, addrs: &[A], ell: u32) {
        #[expect(
            clippy::disallowed_methods,
            reason = "rule 15 — canonical key order: the addresses arrive in key order, so the head is the least key"
        )]
        let Some(rep) = addrs.first() else {
            return;
        };
        let addr = rep.as_ref();
        let Some(asg) = self.basis.get(addr).map(|w| element::assignment(w.sub())) else {
            return;
        };
        if let Some(wm) = self.word_mut(addr) {
            wm.update(|e| e.record_torsion(asg, ell, element::FIBER_SPATIAL));
        }
        self.touch_at(addr);
    }
}
