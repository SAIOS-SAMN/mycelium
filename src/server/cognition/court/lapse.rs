// 26, law 7 — THE LAPSE: what a docket that ran out of time releases.

use crate::cognition::*;
use crate::transducer::address::Address;

impl Kernel {
    /// The key a held word is filed under: the K of its last flux,
    /// big-endian so the key's order is the orbit's, then its admitting
    /// stamp — a coordinate, never a copy of the address.
    fn flux_key(k: u64, at: &[u8]) -> Vec<u8> {
        let mut key = k.to_be_bytes().to_vec();
        key.extend_from_slice(at);
        key
    }

    /// File the word admitted at `at` under this orbit's `K`, moving it from
    /// where its last flux filed it.
    pub(crate) fn file_flux(&mut self, at: &[u8]) {
        let k = self.k_step();
        if let Some(&was) = self.flux_k.get(at) {
            self.lapse_at.remove(&Self::flux_key(was, at));
        }
        self.flux_k.insert(at, k);
        self.lapse_at.insert(Self::flux_key(k, at), ());
    }

    /// 78.37 (3), FX1 — FLUX REACHED A HELD WORD in this orbit of the court:
    /// a citer's frame, a carriage's draw, a voice's escrow. A touch that
    /// carries nothing on any lane is no circulation and moves nothing: only
    /// a flux `Φ` non-zero on some lane re-files the word. A word not yet
    /// admitted is filed by its admission (`file_admitted`).
    pub(crate) fn note_flux(&mut self, addr: &[u8], flux: [u64; 4]) {
        if flux.iter().all(|&l| l == 0) {
            return;
        }
        if let Some(at) = self.stamp_key(addr) {
            self.file_flux(&at);
        }
    }

    /// The stamp is filed nowhere.
    pub(crate) fn forget_flux_at(&mut self, at: &[u8]) {
        if let Some(was) = self.flux_k.remove(at) {
            self.lapse_at.remove(&Self::flux_key(was, at));
        }
    }

    /// The word left the book or the docket: it is filed nowhere.
    pub(crate) fn forget_flux(&mut self, addr: &[u8]) {
        if let Some(at) = self.stamp_key(addr) {
            self.forget_flux_at(&at);
        }
    }

    /// THE LAPSE SWEEP (78.37 (3) `ZeroFlux`, 45.7, rule 10), taken on every
    /// court stroke and at every close of the court's orbit, never only on a
    /// sealed close. `lapse_at` is read in its own key order, oldest flux
    /// first, one entry per spend:
    ///
    /// - a word whose last flux stands before `K − 1` saw a whole orbit pass
    ///   with nothing flowing to it, and lapses;
    /// - and while the book stands over its ceiling (`physics::book_ceiling`,
    ///   what this node's memory carries) the listings of oldest flux lapse
    ///   whatever their `K`, down to `BOOK_LOW`: the manifold is finite and
    ///   the book is not entitled to exceed it.
    ///
    /// The sweep's stroke is the depth it has to clear where the book is
    /// over its ceiling, else one stroke.
    pub(crate) fn lapse_silent(&mut self) {
        // At the first orbit no word has seen a whole orbit pass: nothing is
        // silent, and only the ceiling lapses.
        let bound = self.k_step().checked_sub(1);
        let held = self.pending.len();
        let over = held > crate::physics::book_ceiling();
        let mut sweep = match over {
            true => self.physics.stroke_n(held - crate::physics::BOOK_LOW),
            false => self.physics.stroke(),
        };
        let mut due: Vec<(u64, Vec<u8>)> = Vec::new();
        for (key, ()) in self.lapse_at.scoped() {
            let Some(k) = key.get(..8).and_then(|b| <[u8; 8]>::try_from(b).ok()).map(u64::from_be_bytes) else {
                continue;
            };
            if !over && bound.is_none_or(|b| k >= b) {
                break;
            }
            let Some(at) = key.get(8..) else { continue };
            if !sweep.spend() {
                break;
            }
            due.push((k, at.to_vec()));
        }
        for (k, at) in due {
            let Some(addr) = self.word_of_stamp(&at) else {
                self.forget_flux_at(&at);
                continue;
            };
            let silent = bound.is_some_and(|b| k < b);
            let evict = !silent && self.pending.len() > crate::physics::BOOK_LOW;
            match (self.provisional.contains(addr.as_bytes()), self.pending.contains(addr.as_bytes())) {
                (true, _) if silent => {
                    self.forget_flux_at(&at);
                    self.lapse_word(addr.as_bytes());
                }
                (false, true) if silent || evict => {
                    self.forget_flux_at(&at);
                    self.lapse_listing(addr.as_bytes());
                }
                (false, false) => self.forget_flux_at(&at),
                _ => {}
            }
        }
    }

    /// A LISTING NOTHING FLOWED TO FOR A WHOLE ORBIT: its escrow returns at
    /// the door, its draw is unlisted, and the lapse is this court's own act
    /// (45.1), the listing cited as it stood (78.24 (2c)).
    fn lapse_listing(&mut self, addr: &[u8]) {
        let me = self.identity;
        let cite = self.cite_of(addr);
        let _ = self.treasury.revoke_award(addr, crate::economy::escrow::Revocation::Drop);
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
        let _ = self.treasury.revoke_award(addr, crate::economy::escrow::Revocation::Drop);
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
