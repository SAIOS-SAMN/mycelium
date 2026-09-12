// 23 — WHAT THE COURT HOLDS: the basis, the sections, the torsion, the
// receipt index, and the cohesion witness.

use crate::cognition::*;

impl Kernel {
    /// A read-only view of the germinated basis in canonical key order — a
    /// display and test seam, never an index for cognition, where every read
    /// of a word is `word_at` by its address. It reads `basis` itself:
    /// `language.words` is a snapshot that diverges from the carrier the
    /// moment `word_mut` makes an `Arc` unique, so a view onto it reports the
    /// word as it was before the puncture (rule 12, one fact one carrier).
    pub fn basis(&self) -> Vec<std::sync::Arc<crate::transducer::element::Held>> {
        self.basis
            .fiber(crate::network::nerve::ROOT)
            .map(|(_, w)| std::sync::Arc::clone(w))
            .collect()
    }

    /// The basis's interior VOLUME — the full fiber span across all punctured
    /// axes (free sections + every torsion orbit to its period; the condensation
    /// metric: cardinality falls while this rises, and cycles carry span in O(1)).
    pub fn sections_total(&self) -> usize {
        self.basis
            .fiber(crate::network::nerve::ROOT)
            // A word's span COUNTS ITS BASE — `fiber_span` opens at one —
            // so the sections are what lies beyond it and the subtraction
            // is total.
            .map(|(_, t)| t.fiber_span() - 1)
            .sum()
    }

    /// The torsion cycles held across the basis — how much of the language has
    /// condensed from enumeration into group presentation.
    pub fn torsion_total(&self) -> usize {
        self.basis
            .fiber(crate::network::nerve::ROOT)
            .map(|(_, t)| {
                let n = crate::transducer::sentence::form_torsion(t).len();
                if n > 0 {
                    n
                } else {
                    t.torsion().len()
                }
            })
            .sum()
    }

    /// The mesh's algebraic clock — the next K-step index (chronometry import).
    pub fn k_step(&self) -> u64 {
        self.chain.next_step_index()
    }

    /// The current chronometric anchor for ledger entries: (K, head hash) — the
    /// unforgeable temporal binding every settlement carries.
    pub(crate) fn anchor(&self) -> crate::economy::Anchor {
        (self.chain.next_step_index(), self.chain.head_hash())
    }

    /// The chronometric receipt chain (verifiable, parent-linked).
    pub fn chain(&self) -> &crate::chronometry::ReceiptChain {
        &self.chain
    }

    /// THE COHESION READING — the joint (C, I) state, both
    /// axes exact and never traded: C counts open demands sitting on KNOWN
    /// geometry (closure failing — collapse-side work); I counts open demands on
    /// geometry NO thought speaks (coverage failing — expansion-side work).
    pub fn cohesion(&mut self) -> (u64, u64) {
        let rank = self.closure_now();
        cohesion_over(&rank)
    }

}
