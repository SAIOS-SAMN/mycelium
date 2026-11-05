// 25.1, 26 — THE STROKE: what a court does to its own book between
// percepts.
//
//! Condensing a family into one bundle at the basis cursor, from the reflex
//! opening (`reflex_open`). Bounded by a Budget in strokes (law 9), never by
//! a timer. The idle consolidation worker and its syntropic stroke and
//! redundancy probe are retired: thinking moves to the client ("clients
//! think, hyphas verify"; 78.157 S3).

use crate::cognition::*;

impl Court {
    /// Condense and torse at the cursor; returns the cursor index when the
    /// cursor word is a plain card (never a bundle: its sections/cycles carry
    /// family volume no other word speaks).
    pub(crate) fn condense_stroke(&mut self) -> Option<crate::transducer::address::Address> {
        let mut budget = self.physics.stroke();
        // The cursor circuits the basis (S13.3): past the last word it
        // continues from the first, so `None` here is an empty basis.
        let Some((address, word)) = self.basis.cursor(self.optimize_cursor.as_deref(), &mut budget).next()
        else {
            self.optimize_cursor = None;
            return None;
        };
        let own = crate::transducer::address::Address::from(address.as_slice());
        self.optimize_cursor = Some(address);
        let word = (**word).clone();
        if !word.sub().is_empty() {
            let inv = crate::transducer::stalk::invariant_of(&word);
            let fk = chart_footprint_key(&inv, word.sub());
            let mut fibre: Vec<Vec<u8>> = self
                .by_footprint
                .link(&fk)
                .map(|(a, _)| a)
                .filter(|a| a.as_slice() != own.as_bytes())
                .collect();
            if fibre.is_empty() {
                fibre = self
                    .stalks
                    .fibre(&inv)
                    .iter().filter(|&a| a.as_slice() != own.as_bytes()).cloned()
                    .collect();
            }
            let mut spend = self.physics.stroke();
            // NO TETHER IS MEASURED HERE (78.74). A held word's chain is
            // CENTRED (`canonical_chain`: r − min r, c − min c), so the
            // distance between two words' chains is the canonical form's,
            // never a placement, and a strength read off it would sever by an
            // artifact. Tethers are measured where pivots stand where they
            // are: the percept's pieces (`family::torsion_quotient`).
            for sibling in fibre {
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
