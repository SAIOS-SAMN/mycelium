// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
// #273, MU-ENGINEERING §8 — THE COURT'S REJOIN: the receipts of its own line
// a peer returns after a restart, held before the court signs again.

use super::Court;
use crate::chronometry::{KStepReceipt, Place, RejoinRefusal};
use crate::court::admission::Reconstructed;

/// WHAT ONE PAGE OF THE REJOIN DID: own receipts held, and the first that
/// was not, with why.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Rejoined {
    pub held: usize,
    pub stopped: Option<(Place, RejoinRefusal)>,
}

impl Court {
    /// #273 — A PAGE OF THIS COURT'S OWN RECEIPTS, from a peer, taken back in
    /// line order: each reconstructed from its own coordinates and held where
    /// it continues the head (`ReceiptChain::rejoin`). The page stops at the
    /// first receipt that does not, named; one already held is passed over.
    pub fn rejoin_own(&mut self, page: &[KStepReceipt]) -> Rejoined {
        let mut out = Rejoined::default();
        for r in page {
            let at = r.stamp.place();
            let Some(r) = Reconstructed::receipt(r) else {
                out.stopped = Some((at, RejoinRefusal::Unsealed));
                return out;
            };
            match self.chain.rejoin(r) {
                Ok(()) => out.held += 1,
                Err(RejoinRefusal::Held) => {}
                Err(why) => {
                    out.stopped = Some((at, why));
                    return out;
                }
            }
        }
        out
    }

    /// The place this court's next own receipt would stand at: the head the
    /// rejoin asks above.
    pub fn own_head_place(&self) -> Option<Place> {
        self.chain.head_place()
    }
}
