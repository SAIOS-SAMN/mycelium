// MU-CLIENT §5.4, S3 — THE MIND PROPOSES AN OPTIMIZATION: a word its ruled
// words reproduce exactly, with no residue, retires — the language moves
// toward the geodesic, its structure changed and its meaning not (3.8, 27).
//
//! The mind finds the word (`seek_refinement`: deconstructed over the
//! genome alone, every cite ruled, nothing left over), mints the `Retire`
//! rewrite naming it as both what it rewrote and what it reached (the
//! meaning unchanged), citing the words that speak it and freeing the lanes
//! it carried, and speaks it. The court replays the retirement on its own
//! language (`retire_rewrite_inputs`, `reduce_excluding`) and keeps an input
//! that does not decompose; the freed description is POWR.

use crate::client::sdk::*;
use crate::transducer::rewrite::{mint, RewriteKind, RewritePayload};

impl Entity {
    /// THE NEXT RETIREMENT the vocabulary holds, minted: the word at the
    /// refine cursor that its ruled words reproduce with no residue, or
    /// `None` where the word there does not decompose over the rest.
    pub(crate) fn retirement(&mut self) -> Option<crate::transducer::element::Element> {
        let (word, cites) = self.seek_refinement()?;
        let freed = self.epigenome.word_at(word.as_bytes())?.magnitude();
        let payload = RewritePayload::Retire { cites: cites.iter().map(|c| c.addr.clone()).collect(), freed };
        mint(RewriteKind::Retire, std::slice::from_ref(&word), word.clone(), payload, freed)
    }

    /// ONE RETIREMENT PROPOSED to the court, where the vocabulary holds one:
    /// the speech's codes.
    pub(crate) fn propose_retirement(&mut self, budget: &mut crate::physics::Budget) -> Option<io::Result<Vec<u8>>> {
        let rw = self.retirement()?;
        Some(self.speak(std::slice::from_ref(&rw), budget))
    }
}
