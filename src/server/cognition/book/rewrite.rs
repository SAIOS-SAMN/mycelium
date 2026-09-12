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
    /// X10: the PARTIES of the word this rewrite optimised, not one of
    /// them. An empty set is a rewrite with no payee, and it is refused
    /// rather than docketed to nobody — the escrow would stand forever
    /// (`release_rewrite` already refuses to release one).
    pub(crate) fn offer_rewrite(&mut self, rw: Element, parties: &[[u8; 32]]) -> bool {
        if !rw.is_rewrite() || rw.magnitude().iter().all(|&x| x == 0) || parties.is_empty() {
            return false;
        }
        let addr = rw.address();
        // THIS COURT RAN THE STROKE that minted the rewrite, so this
        // court's key is the act's operator; the parties are the voices
        // whose derivations reached the word it optimised, and they are
        // paid. The two were one argument, and the payee was being written
        // into the receipt as the operator of an act it never performed.
        let operator = self.identity;
        self.docket_connection(&rw, operator, parties);
        self.offered.insert(addr, ());
        true
    }

    pub(crate) fn mint_rewrite(
        &mut self,
        kind: crate::transducer::rewrite::RewriteKind,
        before: &[Element],
        after: &Element,
        payload: crate::transducer::rewrite::RewritePayload,
        parties: &[[u8; 32]],
    ) -> bool {
        let Some(mu) = self.treasury.settle_rewrite(kind, before, after) else {
            return false;
        };
        let before_addr: Vec<Address> = before.iter().map(|e| e.address()).collect();
        let Some(rw) = crate::transducer::rewrite::mint(kind, &before_addr, after.address(), payload, mu) else {
            return false;
        };
        self.offer_rewrite(rw, parties)
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
            self.cavity_book.insert(addr, elem.clone());
            self.cavities.push(elem);
        }
    }
}
