// 25.1, 26 — THE STROKE: what a court does to its own book between
// percepts.
//
//! Consolidation, the syntropic stroke, re-anchoring a chart, retiring what
//! is derivable and condensing a family into one bundle. Each is bounded by
//! a Budget in strokes (law 9), never by a timer.

use crate::cognition::*;

impl Kernel {
    /// THE SYNTROPIC STROKE — one bounded consolidation step, run when a lane's
    /// reserve is depleted. Probe the cursor generator for REDUNDANCY: if the rest
    /// of the basis already derives it (its substance decomposes with it excluded),
    /// it is a closed critical pair wearing a generator's name — RETIRE it, and the
    /// basis compresses toward a minimal confluent generating set. Either way the
    /// reserve REGENERATES: optimization is the recovery stroke, so entropic
    /// expansion resumes only at the pace the syntropic counterpart earns.
    /// One bounded consolidation stroke, public for the server's idle
    /// deliberation worker: condense, torse, probe redundancy, regenerate.
    pub fn consolidate(&mut self) {
        self.optimize();
    }

    /// The syntropic stroke reads the field's stagnation per chart per lane.
    /// A chart stagnant on gauge is a condense candidate, holonomic torse,
    /// shell refine, spatial re-anchor. Temporal stagnation relocates on T.
    /// Candidates from open and the ruled cover.
    #[expect(
        clippy::disallowed_methods,
        reason = "rule 4 — the least address on a chart is key order, which is content: two hyphae holding one chart must condense the same word or the language forks along insertion order"
    )]
    pub fn syntropic_stroke(&mut self) {
        let mut budget = self.physics.stroke();
        let mut jobs: Vec<(crate::transducer::stalk::Invariant, [bool; 4])> = Vec::new();
        // index: budgeted cursor over the open docket, resumed from the
        // last stroke (rule 10, S13.3) — a circuit, so a docket beyond one
        // budget is read whole across strokes and never only its head.
        {
            let from = self.syntropic_cursor.clone();
            let mut cur = self.open.cursor(from.as_deref(), &mut budget);
            for (addr, _) in &mut cur {
                let Some(listing) = self.field.get(&addr) else {
                    continue;
                };
                for e in &listing.entries {
                    if e.state.is_flat() {
                        continue;
                    }
                    let lanes = std::array::from_fn(|l| e.stagnant[l] > 0);
                    if lanes.iter().any(|&x| x) {
                        jobs.push((e.chart.clone(), lanes));
                    }
                }
            }
            if let Some(k) = cur.last_key() {
                self.syntropic_cursor = Some(k.to_vec());
            }
        }
        // index: ruled cover fiber
        let ruled: Vec<Vec<u8>> = self.ruled_addrs.scoped().map(|(a, _)| a).collect();
        for addr in &ruled {
            if !budget.spend() {
                break;
            }
            if let Some(w) = self.word_at(addr) {
                jobs.push((crate::transducer::stalk::invariant_of(w), [true; 4]));
            }
        }
        let idle = jobs.is_empty();
        let mut t_stale = false;
        for (inv, lanes) in jobs {
            if lanes[1] {
                // Canonical, not positional: the least address on the chart.
                // Two hyphae holding one chart must condense the same word,
                // or the distributed language forks along insertion order.
                if let Some(a) = self.stalks.fiber(&inv).iter().min().cloned() {
                    let _ = self.condense_on(&a, &mut budget);
                }
            }
            if lanes[2] {
                t_stale = true;
            }
            if lanes[3] {
                // Canonical, not positional: the least address on the chart.
                // Two hyphae holding one chart must condense the same word,
                // or the distributed language forks along insertion order.
                if let Some(a) = self.stalks.fiber(&inv).iter().min().cloned() {
                    self.torse_at(&a);
                    self.refine_stratum(&a);
                }
            }
            // Lane 0 RE-ANCHORED a chart's words by the tree gauge; the
            // gauge retired with the slots (AT1 step 2, ruled (a)): a face's
            // word is derived from its ends and there is no gauge to move.
        }
        if t_stale {
            self.relocate_t_exhausted();
        }
        for addr in &ruled {
            if !budget.spend() {
                break;
            }
            let Some(candidate) = self.basis.get(addr).map(|w| (**w).clone()) else {
                continue;
            };
            if matches!(self.reduce_excluding(&candidate, Some(addr.as_slice()), &mut budget), NormalForm::Decomposed(_)) {
                self.retire_redundant(addr, &mut budget);
            }
        }
        if idle {
            let _ = self.condense_stroke();
        }
        self.physics.regenerate();
    }

    // `reanchor_chart` STOOD HERE AND RETIRED (AT1 step 2, ruled (a)): it
    // solved a chart's words' tree gauge and minted the regauged word as a
    // `Reanchor` rewrite. The gauge was a choice held beside the coordinates;
    // with the face word derived from its ends there is nothing to regauge.

    /// The whole stroke under one lock (the reflex and test paths).
    pub(crate) fn optimize(&mut self) {
        self.syntropic_stroke();
        if let Some(addr) = self.condense_stroke() {
            let Some(candidate) = self.basis.get(&addr).map(|w| (**w).clone()) else {
                return;
            };
            let mut budget = self.physics.stroke();
            if let NormalForm::Decomposed(_) = self.reduce_excluding(&candidate, Some(addr.as_bytes()), &mut budget) {
                self.retire_redundant(&addr, &mut budget);
            }
        }
    }

    /// The stroke for the idle deliberation worker: the cheap moves (condense,
    /// torse, regenerate) run here under the lock; the redundancy probe — a
    /// full decomposition — is handed out as (address, candidate, the basis
    /// without it) to run on that snapshot with no lock held, exactly as
    /// adoption's derivability does. The reflex path never waits on it.
    pub fn consolidate_stroke(&mut self) {
        self.syntropic_stroke();
    }

    /// Retire a plain card the rest of the basis derives (the off-lock probe
    /// found it decomposable). Returns the cards retired (0 or 1): the word
    /// may have gone, or gained a fibre, while the probe ran. On the stroke
    /// of the act that probed it (65.5).
    pub fn retire_redundant(&mut self, addr: &[u8], budget: &mut crate::physics::Budget) -> usize {
        let Some(held) = self.basis.get(addr).map(|w| (**w).clone()) else { return 0 };
        if !held.sections().is_empty() || crate::transducer::sentence::carries_torsion(&held) {
            return 0;
        }
        let mag = held.magnitude();
        let elem = self.basis_remove(addr);
        self.ruled_admit(addr, budget);
        self.cavity_book.insert(addr, elem);
        self.physics.syntropy(&mag);
        1
    }

    /// Condense and torse at the cursor; returns the cursor index when the
    /// cursor word is a plain card (a redundancy-probe candidate — never a
    /// bundle: its sections/cycles carry family volume no other word speaks).
    pub(crate) fn condense_stroke(&mut self) -> Option<crate::transducer::address::Address> {
        let mut budget = self.physics.stroke();
        // The cursor circuits the basis (S13.3): past the last word it
        // continues from the first, so `None` here is an empty basis.
        let Some((addr, word)) = self.basis.cursor(self.optimize_cursor.as_deref(), &mut budget).next()
        else {
            self.optimize_cursor = None;
            return None;
        };
        let own = crate::transducer::address::Address::from(addr.as_slice());
        self.optimize_cursor = Some(addr);
        let word = (**word).clone();
        if !word.sub().is_empty() {
            let inv = crate::transducer::stalk::invariant_of(&word);
            let fk = chart_footprint_key(&inv, word.sub());
            let mut fiber: Vec<Vec<u8>> = self
                .by_footprint
                .link(&fk)
                .map(|(a, _)| a)
                .filter(|a| a.as_slice() != own.as_bytes())
                .collect();
            if fiber.is_empty() {
                fiber = self
                    .stalks
                    .fiber(&inv)
                    .iter().filter(|&a| a.as_slice() != own.as_bytes()).cloned()
                    .collect();
            }
            let mut spend = self.physics.stroke();
            // NO TETHER IS MEASURED HERE (78.74). A held word's chain is
            // CENTRED (`canonical_chain`: r − min r, c − min c), so the
            // distance between two words' chains is the canonical form's,
            // never a placement, and a strength read off it would sever by an
            // artifact. Tethers are measured where pivots stand where they
            // are: the percept's pieces (`family::fibres`).
            for sibling in fiber {
                if !spend.spend() {
                    break;
                }
                let card = self.basis.get(&sibling).is_some_and(|w| {
                    w.sections().is_empty() && !crate::transducer::sentence::carries_torsion(w)
                });
                if !card {
                    continue;
                }
                let _ = self.absorb_pair(own.clone(), crate::transducer::address::Address::from(sibling), &mut spend);
            }
        }
        self.torse_at(&own);
        let held = self.basis.get(&own)?;
        if held.sections().is_empty() && !crate::transducer::sentence::carries_torsion(held) {
            return Some(own);
        }
        None
    }

}
