// 27, 3.8 — THE REWRITE AT THE BOOK: an optimisation offered, minted on
// a second organ, and the inputs it retires.
//
//! Split from `speech.rs` at the §1.1 600-line reading bound; the items
//! are moved, not rewritten (§1.6).

use crate::cognition::*;
use crate::transducer::element::Element;

impl Kernel {
    /// Speak an optimisation: refuse a zero vector, docket the Rewrite,
    /// Offer it, and keep it as language. Payment waits on the second organ.
    pub(crate) fn offer_rewrite(&mut self, rw: Element, voice: [u8; 32]) -> bool {
        if !rw.is_rewrite() || rw.magnitude().iter().all(|&x| x == 0) {
            return false;
        }
        let addr = rw.address();
        self.docket_connection(&rw, voice);
        self.offered.insert(addr, ());
        true
    }

    pub(crate) fn mint_rewrite(
        &mut self,
        kind: crate::transducer::rewrite::RewriteKind,
        before: &[Element],
        after: &Element,
        payload: crate::transducer::rewrite::RewritePayload,
        voice: [u8; 32],
    ) -> Option<Address> {
        let mu = self.treasury.settle_rewrite(kind, before, after)?;
        let before_addr: Vec<Address> = before.iter().map(|e| e.address()).collect();
        let rw = crate::transducer::rewrite::mint(kind, &before_addr, after.address(), payload, mu)?;
        let addr = rw.address();
        // The rewrite's address, where it was offered: a retirement cites
        // the rewrite it rests on (EC1).
        self.offer_rewrite(rw, voice).then_some(addr)
    }

    pub(crate) fn retire_rewrite_inputs(&mut self, rw: &Element) {
        let Some(r) = crate::transducer::rewrite::Rewrite::from_element(rw) else {
            return;
        };
        for addr in r.before {
            let Some(w) = self.basis.get(&addr).map(|e| (**e).clone()) else {
                continue;
            };
            if !matches!(self.reduce_excluding(&w, Some(addr.as_bytes())), NormalForm::Decomposed(_)) {
                continue;
            }
            let elem = self.basis_remove(&addr);
            self.ruled_admit(&addr);
            self.cavity_book.insert(addr, elem.clone().into());
            self.cavities.push(elem);
        }
    }
}
