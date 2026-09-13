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
            .scoped()
            .map(|(_, w)| std::sync::Arc::clone(w))
            .collect()
    }

    /// The basis's interior VOLUME — the fibre span above the base across all
    /// punctured axes (free sections + every torsion orbit to its period; the
    /// condensation metric: cardinality falls while this rises, and cycles
    /// carry span in O(1)).
    ///
    /// IT READS THE CARRIER THAT HOLDS THE FIBRE, which is what `torsion_total`
    /// below already did and this did not. A sentence's cycles are on its FORM;
    /// `Element::fibre_volume` sums the element's own two fields, which a
    /// minted sentence leaves empty. So every sentence in the basis contributed
    /// 0 to the volume however many loops its form had filed — the ruling at
    /// `court_torsion_on_v2_form_is_consilient_or_filed_or_refused` ("the court
    /// reads Form.torsion by the filed loop's class key, never the element's
    /// empty section") held at one site of its shape and not at its neighbour
    /// three lines down.
    ///
    /// A FORM HAS NO FREE-SECTION CARRIER, so nothing stands in for the free
    /// part on that arm: `Form.sections` is PAIR INDICES of the bundle — which
    /// pairs a class was measured on — not transition assignments over a base,
    /// and summing it here would be the two-objects-one-name fault a row is
    /// already owed on. A sentence's free part is absent, not zero.
    ///
    /// It was `sections_total`, which named one component and returned the
    /// composite of two (§2.2), and the caller (`Kernel::volume`) always wanted
    /// the volume its own doc says it takes.
    pub fn fibre_volume_total(&self) -> usize {
        self.basis
            .scoped()
            // The form is read off the HELD word, which decoded it once when
            // the organ took it in (L16), not through `form_torsion`, which
            // re-parses the tape on every call for every word.
            .map(|(_, t)| match t.form() {
                Some(f) if !f.torsion.is_empty() => f
                    .torsion
                    .scoped()
                    // The form tape refuses a filed zero (A17: a loop is filed
                    // at the period it CLOSES at), so ℓ ≥ 1 here and the
                    // subtraction is total.
                    .map(|(_, l)| l.period as usize - 1)
                    .sum(),
                // A v1 sentence that adopted its cycles element-side still
                // reads as a bundle here, which is why the form arm is guarded
                // on having filed anything rather than on being a sentence.
                _ => t.fibre_volume(),
            })
            .sum()
    }

    /// The torsion cycles held across the basis — how much of the language has
    /// condensed from enumeration into group presentation.
    pub fn torsion_total(&self) -> usize {
        self.basis
            .scoped()
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
