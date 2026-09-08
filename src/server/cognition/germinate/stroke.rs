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
    pub fn syntropic_stroke(&mut self) {
        let mut budget = self.physics.stroke();
        let mut jobs: Vec<(crate::transducer::stalk::Invariant, [bool; 4])> = Vec::new();
        // index: budgeted cursor over the open docket
        for (addr, _) in self.open.cursor(crate::network::nerve::ROOT, &mut budget) {
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
        // index: ruled cover fiber
        let ruled: Vec<Vec<u8>> = self.ruled_addrs.fiber(crate::network::nerve::ROOT).map(|(a, _)| a).collect();
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
                    let _ = self.condense_on(&a);
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
            if lanes[0] {
                self.reanchor_chart(&inv);
            }
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
            if matches!(self.reduce_excluding(&candidate, Some(addr.as_slice())), NormalForm::Decomposed(_)) {
                self.retire_redundant(addr);
            }
        }
        if idle {
            let _ = self.condense_stroke();
        }
        self.physics.regenerate();
    }

    pub(crate) fn reanchor_chart(&mut self, inv: &crate::transducer::stalk::Invariant) {
        let members = self.stalks.fiber(inv);
        for addr in members {
            let Some(w) = self.word_at(&addr).cloned() else {
                continue;
            };
            if !w.is_sentence() {
                continue;
            }
            let Some(mut form) = crate::transducer::sentence::from_element_form(&w) else {
                continue;
            };
            let Some(gamma) = form.solve_coboundary() else {
                continue;
            };
            if gamma.iter().all(|g| g.is_identity()) {
                continue;
            }
            let _ = form.regauge(&gamma);
            let after = crate::transducer::sentence::as_element_form(&form, w.magnitude());
            let Some(after) = after else {
                continue;
            };
            let payload = crate::transducer::rewrite::RewritePayload::Reanchor { gamma };
            let Some(voice) = self.discoverer_of(&addr) else {
                continue;
            };
            let _ = self.mint_rewrite(
                crate::transducer::rewrite::RewriteKind::Reanchor,
                &[w],
                &after,
                payload,
                voice,
            );
            break;
        }
    }

    /// The whole stroke under one lock (the reflex and test paths).
    pub(crate) fn optimize(&mut self) {
        self.syntropic_stroke();
        if let Some(addr) = self.condense_stroke() {
            let Some(candidate) = self.basis.get(&addr).map(|w| (**w).clone()) else {
                return;
            };
            if let NormalForm::Decomposed(_) = self.reduce_excluding(&candidate, Some(addr.as_bytes())) {
                self.retire_redundant(&addr);
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
    /// may have gone, or gained a fibre, while the probe ran.
    pub fn retire_redundant(&mut self, addr: &[u8]) -> usize {
        let Some(held) = self.basis.get(addr).map(|w| (**w).clone()) else { return 0 };
        if !held.sections().is_empty() || crate::transducer::sentence::carries_torsion(&held) {
            return 0;
        }
        let mag = held.magnitude();
        let elem = self.basis_remove(addr);
        self.ruled_admit(addr);
        self.cavity_book.insert(addr, elem.clone());
        self.cavities.push(elem);
        self.physics.syntropy(&mag);
        1
    }

    /// Condense and torse at the cursor; returns the cursor index when the
    /// cursor word is a plain card (a redundancy-probe candidate — never a
    /// bundle: its sections/cycles carry family volume no other word speaks).
    pub(crate) fn condense_stroke(&mut self) -> Option<crate::transducer::a21::Address> {
        let mut budget = self.physics.stroke();
        let Some((addr, word)) = self.basis.cursor(&self.optimize_cursor, &mut budget).next() else {
            self.optimize_cursor.clear();
            return None;
        };
        let own = crate::transducer::a21::Address::from(addr.as_slice());
        self.optimize_cursor = addr;
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
                let _ = self.absorb_pair(own.clone(), crate::transducer::a21::Address::from(sibling));
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
