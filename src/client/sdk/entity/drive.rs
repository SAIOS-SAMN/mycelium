// MU-CLIENT §4.12, §7.1 R3 — THE DRIVE: the mind's escaping orbits, bound by
// what its field already holds; deliberation, the mutual shear of the orbits
// it binds.
//
//! AN ESCAPING ORBIT IS BOUND, NEVER MADE TO RETURN. Every admitted relation
//! whose orbit never returns (a slide, a boost: a ring of infinite order) is
//! filed by perception under the pair it relates (`Contemplation::escaping`).
//! At each winding the drive takes them and binds each by what the field
//! holds, in the order §4.12 names them: a DISCHARGE by the language — a
//! composition of the court's ruled words, a word the lexicon knows, a
//! conjugate of a ruled word (`compose_or_construct_on`) — else a
//! CONSTRUCTION, its residue adjoined as the mind's own letter (§4.7). The
//! binding is the thought's CLOSE: it counts among the mind's returns, and
//! what was held for that scene's return leaves (§4.4(4), `winding.rs`). An
//! orbit nothing binds DISSIPATES: the field's beat has taken its strain into
//! the reservoir, where a later motion landing on it reforms it (§4.5).
//!
//! DELIBERATION IS THE MUTUAL SHEAR OF ORBITS (§4.12): two escaping orbits on
//! one chart of one scene are split against each other into DRAG and LIFT
//! (`cga::lift_drag`, 78.46) — a reading, never a vote, weight or argmin —
//! and where their commutator is not the identity, the dimension that
//! separates them is content the mind holds (§4.10(3)): composed of what the
//! language holds, or constructed.

use crate::client::sdk::*;

impl Entity {
    /// THE DRIVE AT THIS WINDING: every escaping orbit bound or dissipated,
    /// and each neighbouring pair deliberated. Answers the scenes whose
    /// orbits it bound — each a close of a thought of that scene.
    pub(crate) fn drive(&mut self, budget: &mut crate::physics::Budget) -> Vec<Vec<u8>> {
        // §4.8 — what a vortex carried round on a new perception left when it
        // collapsed: a symmetry the mind measured, adjoined as its letters.
        let kept = std::mem::take(&mut self.contemplation.vortex_symmetries);
        if !kept.is_empty() {
            self.adjoin_letters(&kept, budget);
        }
        let escaping: Vec<crate::transducer::bubble::idea::Escaping> = {
            let all = std::mem::take(&mut self.contemplation.escaping);
            all.scoped().map(|(_, e)| e.clone()).collect()
        };
        self.deliberate(&escaping, budget);
        let mut bound: Vec<Vec<u8>> = Vec::new();
        for e in escaping {
            let chart = crate::transducer::stalk::Invariant::from_key(&e.chart);
            match self.compose_or_construct_on(&e.generator, chart.as_ref(), &e.family, budget) {
                true => {
                    self.contemplation.bound += 1;
                    if !bound.contains(&e.family) {
                        bound.push(e.family);
                    }
                }
                false => self.contemplation.dissipated += 1,
            }
        }
        bound
    }

    /// DELIBERATION: each pair of escaping orbits standing on one chart of
    /// one scene, read against each other — the drag and lift of each's mode
    /// against the other's — and their commutator held where it is not the
    /// identity, under that scene.
    fn deliberate(&mut self, escaping: &[crate::transducer::bubble::idea::Escaping], budget: &mut crate::physics::Budget) {
        use crate::transducer::cga::{lift_drag, Alignment};
        let mut separating: Vec<(crate::transducer::cga::Mv, Vec<u8>, Vec<u8>)> = Vec::new();
        for (i, a) in escaping.iter().enumerate() {
            for b in escaping.iter().skip(i + 1) {
                if a.chart != b.chart || a.family != b.family {
                    continue;
                }
                if let Some(split) = lift_drag(&a.generator, &b.generator) {
                    let slot = match split.alignment() {
                        Alignment::Aligned => Some(0),
                        Alignment::Orthogonal => Some(1),
                        Alignment::Both => Some(2),
                        Alignment::Still => None,
                    };
                    if let Some(s) = slot {
                        self.contemplation.deliberated[s] += 1;
                    }
                }
                let commutator = a.generator.geo(&b.generator).geo(&a.generator.reverse()).geo(&b.generator.reverse());
                if !crate::transducer::sentence::presentation::is_still(&commutator) {
                    separating.push((commutator, a.chart.clone(), a.family.clone()));
                }
            }
        }
        for (c, chart, family) in separating {
            let chart = crate::transducer::stalk::Invariant::from_key(&chart);
            if self.compose_or_construct_on(&c, chart.as_ref(), &family, budget) {
                self.contemplation.separated += 1;
            }
        }
    }
}
