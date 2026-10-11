// 78.164 — THE FIELD RELAXES: where the reserve is short or nothing flowed
// in, the mind does its inward work — it sheds, expands its dirty ideas,
// pivots held thoughts on the charts its language now holds, takes in the
// court's language where it is due, compresses its vocabulary by one rewrite,
// and regenerates.
//
//! One rewrite a tick, the first the field offers in the bundle synthesis
//! order: a re-anchor (a gauge transition between conjugate sections), a
//! condensation (sibling forms as sections of one bundle), the algebraic step
//! (37.7: a gauge the language measures and cannot spell is adjoined, or a
//! word is bid against a held word's operator), or a citation refinement.

use crate::client::sdk::*;
use crate::physics::StrokeOutcome;

impl Entity {
    pub(crate) fn relax(&mut self, outcome: &mut StrokeOutcome) {
        let _ = self.shed();
        let (pivoted, _) = self.expand_ideas();
        StrokeOutcome::add(&mut outcome.pivoted, &pivoted);
        let _ = self.pivot_thoughts();
        self.language_inflow();
        if let Some((rw, retires, after)) = self.probe_conjugate_reanchor() {
            let _ = self.bid_bundle_rewrite(&rw, &retires, after.as_ref());
        } else if let Some((rw, retires, after)) = self.probe_bundle_condensation() {
            let _ = self.bid_bundle_rewrite(&rw, &retires, after.as_ref());
        } else if let Some(step) = self.algebraic_step() {
            if let AlgebraicStep::Bid(bid) = step {
                let AlgebraicBid { word, held, witness } = *bid;
                let _ = self.bid_algebraic_refinement(word.as_bytes(), held.as_bytes(), &witness);
            }
        } else if let Some((word, cites)) = self.seek_refinement() {
            // A word the rest of the vocabulary derives and the court does
            // not hold is compression this dialect owes itself: retired
            // locally; a word the court holds returns through the sync.
            if let Ok((code, _)) = self.bid_refinement(&word, &cites) {
                if code != crate::anastome::dialogue::REFINE_ACCEPTED {
                    let _ = self.epigenome.retire(&word);
                }
            }
        }
        self.physics_mut().regenerate();
        self.physics_mut().relax();
    }
}
