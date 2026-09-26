// 26, law 7 — THE LAPSE: what a docket that ran out of time releases.

use crate::cognition::*;
use crate::transducer::address::Address;

impl Kernel {
    /// The key a held word is filed under: the K of its last flux,
    /// big-endian so the key's order is the orbit's, then its address.
    fn flux_key(k: u64, addr: &[u8]) -> Vec<u8> {
        let mut key = k.to_be_bytes().to_vec();
        key.extend_from_slice(addr);
        key
    }

    /// 78.37 (3), FX1 — FLUX REACHED A HELD WORD in this orbit of the court:
    /// its admission, a citer's frame, a carriage, a witness's reply. It is
    /// filed again under this orbit's `K`.
    pub(crate) fn note_flux(&mut self, addr: &[u8]) {
        let k = self.k_step();
        if let Some(&was) = self.flux_k.get(addr) {
            self.lapse_at.remove(&Self::flux_key(was, addr));
        }
        self.flux_k.insert(addr, k);
        self.lapse_at.insert(Self::flux_key(k, addr), ());
    }

    /// The word left the book or the docket: it is filed nowhere.
    pub(crate) fn forget_flux(&mut self, addr: &[u8]) {
        if let Some(was) = self.flux_k.remove(addr) {
            self.lapse_at.remove(&Self::flux_key(was, addr));
        }
    }

    /// THE COURT'S ORBIT CLOSED AT `K`, AND WHAT NOTHING FLOWED TO LAPSES
    /// (78.37 (3), `ZeroFlux`): a held word whose last flux stands before
    /// `K − 1` saw a whole orbit pass with nothing flowing to it on any
    /// lane. A filament lapses because nothing flowed through it, never
    /// because a clock ran out. The index is read in its own key order and
    /// stops at the first word that flowed since — the words due, each by
    /// its key — and the words in flight are never walked.
    pub(crate) fn lapse_silent(&mut self) {
        let Some(bound) = self.k_step().checked_sub(1) else { return };
        let due: Vec<Vec<u8>> = self
            .lapse_at
            .scoped()
            .map(|(k, _)| k)
            .take_while(|k| k.get(..8).and_then(|b| <[u8; 8]>::try_from(b).ok()).is_some_and(|b| u64::from_be_bytes(b) < bound))
            .collect();
        for key in due {
            let Some(addr) = key.get(8..).map(<[u8]>::to_vec) else { continue };
            self.forget_flux(&addr);
            match (self.provisional.contains(&addr), self.pending.contains(&addr)) {
                (true, _) => self.lapse_word(&addr),
                (false, true) => self.lapse_listing(&addr),
                (false, false) => {}
            }
        }
    }

    /// A LISTING NOTHING FLOWED TO FOR A WHOLE ORBIT: its escrow returns at
    /// the door, its draw is unlisted, and the lapse is this court's own act
    /// (45.1), the listing cited as it stood (78.24 (2c)).
    fn lapse_listing(&mut self, addr: &[u8]) {
        let me = self.identity;
        let cite = self.cite_of(addr);
        let _ = self.treasury.revoke_award(addr, false);
        self.unlist(&Address::from(addr.to_vec()), false);
        self.append_after_removal(crate::chronometry::ActKind::Lapse, addr, cite, me);
    }

    /// A PROVISIONAL WORD NOTHING FLOWED TO, OR WHOSE ESCROW A CARRIAGE
    /// EXHAUSTED: what is left of it returns at the door, the lapse is sealed,
    /// the word leaves the docket, and every sentence that waited on it is
    /// released as the demand it is (D) — it cites a word this court does
    /// not hold.
    pub(crate) fn lapse_word(&mut self, addr: &[u8]) {
        let me = self.identity;
        let cite = self.cite_of(addr);
        let _ = self.treasury.revoke_award(addr, false);
        self.forget_flux(addr);
        self.leave_provisional(addr);
        // A word listed on the book as well leaves it in the same act.
        if self.pending.contains(addr) {
            self.unlist(addr, false);
        }
        self.append_after_removal(crate::chronometry::ActKind::Lapse, addr, cite, me);
        self.release_awaiting(addr);
    }

    /// THE LISTINGS A SIMPLEX IS OVER (13.9, 47.3; R6): the open listings
    /// on the chart. A simplex's vertices are vantages — hyphas — and no
    /// speaker's index gates the reading: read through an index keyed by
    /// voice, a hypha vertex measures zero, so no section stands a vertex,
    /// no edge has volume, and nothing fills though the chart holds two
    /// sections. The vantages are the simplex's, not the measure's, so the
    /// simplex's vertex list is not an argument here.
    pub(crate) fn listings_on(
        &self,
        inv: &crate::transducer::stalk::Invariant,
    ) -> crate::network::nerve::Keyed<()> {
        match self.chart_listings.get(&inv.key()) {
            Some(on_chart) => on_chart.meet(&self.open),
            None => crate::network::nerve::Keyed::new(),
        }
    }
}
