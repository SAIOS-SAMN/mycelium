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
    /// 0 to the volume however many orbits its form had filed — the ruling at
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
    ///
    /// In the period's carrier, `u64`, the sum exact or `None` (78.43(2)).
    pub fn fibre_volume_total(&self) -> Option<u64> {
        let mut total = 0u64;
        for v in self
            .basis
            .scoped()
            // The form is read off the HELD word, which decoded it once when
            // the organ took it in (L16), not through `form_torsion`, which
            // re-parses the tape on every call for every word.
            .map(|(_, t)| match t.form() {
                // THE CLASS FROM THE RECEIPT, THE PERIOD FROM THE COMPLEX
                // (54.5). `Form::fibre_volume` takes the speaker's filing and
                // prices it against the periods the form's own orbits derive,
                // so a word cannot be paid for capacity its geometry does not
                // carry. The filing rides the ELEMENT the organ holds; a form
                // recovered from an address files nothing.
                Some(f) if !t.element().filing().is_empty() => {
                    f.fibre_volume(t.element().filing())
                }
                // A v1 sentence that adopted its cycles element-side still
                // reads as a bundle here, which is why the form arm is guarded
                // on having filed anything rather than on being a sentence.
                _ => t.fibre_volume(),
            })
        {
            total = total.checked_add(v?)?;
        }
        Some(total)
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

    /// The current chronometric anchor for ledger entries (78.24): the
    /// stamp of this court's head receipt and its hash — the act a row
    /// stands at, named by its whole coordinate. Before genesis, the empty
    /// stamp under this court's key and the empty hash.
    pub(crate) fn anchor(&self) -> crate::economy::Anchor {
        let at = match self.chain.head_stamp() {
            Some(s) => s,
            None => crate::chronometry::Stamp::new(0, self.chain.vk(), crate::chronometry::Place::GENESIS),
        };
        (at, self.chain.head_hash())
    }

    /// The chronometric receipt chain (verifiable, parent-linked).
    pub fn chain(&self) -> &crate::chronometry::ReceiptChain {
        &self.chain
    }

    /// A CITATION INTO A PEER'S CLOSED ERA, PROVED BY THE ERA'S OWN CHAIN
    /// (78.65, 78.87 R2): the page of the signer's line from `p` up to its
    /// fold root, walked by the ledger (`ReceiptChain::prove_from_era`).
    pub fn prove_folded(&mut self, p: &crate::chronometry::Parent, page: &[crate::chronometry::KStepReceipt]) -> bool {
        self.chain.prove_from_era(p, page)
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
