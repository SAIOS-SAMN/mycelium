// 23.8, 26 — THE BOOK: what is listed, what is pending, what has drained
// into a cavity, and what is on the page.
//
//! A listing enters by `book_list` and leaves by `unlist` or by draining
//! when nothing can reach it. The book has NO ORDER: every read here is by
//! address, chart or lane, never by a rank over the whole of it.

use crate::cognition::*;
use crate::transducer::address::Address;
use crate::transducer::element::Element;

/// ONE ROW OF THE DOCKET, as the court replays it. Six fields, and which
/// μ⃗ is which is the whole question: `escrowed` is what was staked against
/// the word, `deflation` is what it MEASURED. A row where those two swap
/// places is a mint for unmeasured work.
pub struct Docketed {
    /// The listing the word answers.
    pub addr: Address,
    /// The provisional word itself, to be replayed blind.
    pub spoken: Element,
    /// The voices attesting it — parties to the settlement, never vantages.
    pub voices: Vec<[u8; 32]>,
    /// The μ⃗ escrowed against the word: a claim on issuance (§5).
    pub escrowed: [u64; 4],
    /// The μ⃗ the word measured, per lane. Never fabricated: a word with no
    /// measured deflation is not served.
    pub deflation: [u64; 4],
    /// The k-step it was installed on.
    pub k: u64,
}

impl Kernel {
    pub(crate) fn drain_unreachable(&mut self) {
        let kappa = crate::physics::aperture_cycle();
        let mut budget = self.physics.stroke();
        let mut keys: Vec<Vec<u8>> = Vec::new();
        // index: budgeted keyed cursor over the open docket, resumed from
        // the last stroke (rule 10, S13.3) — an orbit, so the drain reads
        // a docket beyond one budget whole across strokes.
        {
            let from = self.drain_cursor.clone();
            let mut cur = self.open.cursor(&from, &mut budget);
            for (k, _) in &mut cur {
                keys.push(k);
            }
            if let Some(k) = cur.last_key() {
                self.drain_cursor = k.to_vec();
            }
        }
        let keys: Vec<Vec<u8>> = keys
            .into_iter()
            .filter_map(|k| {
                if !self.on_page.contains(&k) {
                    return None;
                }
                let listing = self.field.get(&k)?;
                let t = self.tension.get(&k)?;
                if listing.nonflat == 0 {
                    return None;
                }
                let occupied = t.iter().any(|&x| x > 0);
                let nonflat: Vec<_> = listing.entries.iter().filter(|e| !e.state.is_flat()).collect();
                if occupied
                    && !nonflat.is_empty()
                    && (0..4).all(|l| t[l] == 0 || nonflat.iter().all(|e| e.stagnant[l] >= kappa))
                {
                    Some(k)
                } else {
                    None
                }
            })
            .collect();
        for key in keys {
            self.relocate_cavity(&key);
        }
    }

    /// List a residual on the book and admit its base chart. The draw occupies
    /// the uttering voice's bond; a receipt is sealed so lapse can fire.
    pub(crate) fn book_list(&mut self, residual: Element, k_now: u64, voice: [u8; 32]) {
        self.list(residual, k_now, voice, Some(voice));
    }

    /// THE CORPUS INTAKE'S LISTING (23.8; I31, S12.5b): the hypha lists its
    /// own corpus with no drawer. Room is a voice's — drawn by its utterance
    /// and released by closure, ruling or lapse — and no voice uttered a
    /// corpus task; drawn from the hypha's own bond the corpus exhausted it
    /// in the first minutes of every boot (245 refusals on hypha a in 150 s
    /// of the 2026-09-08 recut) and nothing homed here could lapse it back.
    /// The listing still tensions the book and admits its chart, and its
    /// receipt names the hypha as the actor.
    pub(crate) fn book_list_undrawn(&mut self, residual: Element, k_now: u64) {
        let actor = self.identity;
        self.list(residual, k_now, actor, None);
    }

    fn list(&mut self, residual: Element, k_now: u64, actor: [u8; 32], drawer: Option<[u8; 32]>) {
        let addr = residual.address();
        self.stamp_standing(&addr);
        let mag = residual.magnitude();
        self.physics.bump(&mag);
        let inv = crate::transducer::stalk::invariant_of(&residual);
        self.listing_atlas.admit(&inv, &addr);
        self.tension.insert(&addr, mag);
        self.on_page.insert(&addr, ());
        // A LISTING IS DRAWN ONCE (§3.1 as ruled, S13.15): an address
        // already on the page keeps its drawer and its occupancy, so a
        // second vantage on it neither doubles the occupancy nor takes the
        // drawer over — which left the first voice's room drawn for good
        // when the install released the second's instead.
        let already = self.pending.contains(&addr);
        if let (Some(voice), false) = (drawer, already) {
            self.drawers.insert(&addr, voice);
        }
        if !already {
            self.occupy(&addr, &mag, true);
        }
        self.pending.insert(&addr, (residual, k_now));
        self.book_gen += 1;
        self.append_step(crate::chronometry::ActKind::List, &addr, actor);
    }

    pub(crate) fn pending_take(&mut self, addr: &[u8]) -> Option<Element> {
        let (elem, _) = self.pending.remove(addr)?;
        let mag = elem.magnitude();
        self.occupy(addr, &mag, false);
        self.drawers.remove(addr);
        self.physics.relieve(&mag);
        Some(elem)
    }

    /// True closure of the whole listing: the thought glued. Leaves the
    /// page and the listing atlas (the word, if any, lives on the language).
    pub(crate) fn unlist(&mut self, addr: &[u8], as_cavity: bool) {
        if as_cavity {
            self.relocate_cavity(addr);
            return;
        }
        if let Some(elem) = self.pending_take(addr) {
            let _ = elem;
        }
        if let Some(listing) = self.field.remove(addr) {
            for e in &listing.entries {
                if !e.state.is_flat() {
                    self.listing_atlas.retract(&e.chart, addr);
                }
            }
            self.field_cold.insert(addr, listing);
        }
        self.listing_atlas.remove(addr);
        self.tension.remove(addr);
        self.forget_spoken(addr);
        self.quote_frontier.remove(addr);
        self.on_page.remove(addr);
        self.flat_loops.remove(addr);
        self.book_gen += 1;
    }

    /// Relocate a listing off the quote page onto the cavity atlas. The
    /// element, field, and charts stay; pressure re-measures without it.
    pub(crate) fn relocate_cavity(&mut self, addr: &[u8]) {
        let Some(elem) = self.pending_take(addr) else {
            return;
        };
        self.on_page.remove(addr);
        self.quote_frontier.remove(addr);
        self.forget_spoken(addr);
        if let Some(listing) = self.field.get(addr) {
            for e in &listing.entries {
                if !e.state.is_flat() {
                    self.listing_atlas.admit_also(&e.chart, addr);
                }
            }
        }
        let inv = crate::transducer::stalk::invariant_of(&elem);
        self.listing_atlas.admit_also(&inv, addr);
        self.cavity_book.insert(addr, elem.clone());
        self.cavities.push(elem);
        self.chill_field(addr);
        self.book_gen += 1;
    }

    /// The coherence field, keyed by listing.
    pub fn field(&self) -> &crate::network::nerve::Keyed<ListingField> {
        &self.field
    }

    pub fn listing_atlas(&self) -> &crate::transducer::stalk::Stalks {
        &self.listing_atlas
    }

    pub fn on_page(&self) -> &crate::network::nerve::Keyed<()> {
        &self.on_page
    }

    pub fn pending(&self) -> &crate::network::nerve::Keyed<(Element, u64)> {
        &self.pending
    }

    /// THE BOOK'S HEIGHT (23.8, 45.2): the height of this court's chain
    /// head, which the quote and demand replies carry and a voice paces
    /// on. It read `book_gen`, the count of listings written, so every
    /// voice read `[h 0]` while the chain stood thousands of receipts high
    /// and "quote asked when the height advanced" never fired (I39).
    pub fn book_height(&self) -> u64 {
        self.height()
    }

    /// Open (non-flat) charts of a listing, or its base chart if unprobed.
    pub fn open_charts(&self, addr: &[u8]) -> Vec<crate::transducer::stalk::Invariant> {
        let listing = self
            .pending
            .get(addr)
            .map(|(p, _)| p)
            .or_else(|| self.cavity_book.get(addr));
        match listing {
            Some(p) => open_charts_of(addr, p, &self.field),
            None => Vec::new(),
        }
    }

    pub fn cavities(&self) -> &[Element] {
        &self.cavities
    }

    /// The court heard `n` cavities; they leave the docket.
    pub fn cavities_heard(&mut self, n: usize) {
        let n = n.min(self.cavities.len());
        self.cavities.drain(..n);
    }

    #[cfg(test)]
    pub(crate) fn stagnant_of(&self, addr: &[u8]) -> [u32; 4] {
        let Some(listing) = self.field.get(addr) else {
            return [0; 4];
        };
        let mut s = [0u32; 4];
        for e in &listing.entries {
            if e.state.is_flat() {
                continue;
            }
            for (t, d) in s.iter_mut().zip(&e.stagnant) {
                *t = (*t).max(*d);
            }
        }
        s
    }

    #[cfg(test)]
    pub(crate) fn drain_now(&mut self) {
        self.drain_unreachable();
    }

    /// If the fibre over `addr` names exactly one other word a versor
    /// relates, absorb the card into the bundle and pay the absorbed
    /// discoverer the condense yield. Returns the surviving address.
    pub(crate) fn condense_on(&mut self, addr: &[u8]) -> crate::transducer::address::Address {
        let Some(word) = self.word_at(addr).cloned() else {
            return crate::transducer::address::Address::from_bytes(addr);
        };
        if word.is_sentence() {
            let inv = crate::transducer::stalk::invariant_of(&word);
            let members: Vec<Vec<u8>> = self.stalks.fiber(&inv).iter().filter(|&a| a != addr).cloned().collect();
            if let Some(na) = crate::transducer::sentence::from_element_form(&word) {
                let mut survivor = crate::transducer::address::Address::from_bytes(addr);
                for sib in members {
                    // THE SIBLING'S FORM OFF THE HELD WORD (L16): the
                    // sibling is installed, so its form is a field and this
                    // comparison parses nothing. A sibling the court has
                    // not installed has no form to compare and is skipped —
                    // which is what `condense_on` wants, since it absorbs
                    // into the basis.
                    let Some(nb) = self.held_word(&sib).and_then(|e| e.form()).cloned() else {
                        continue;
                    };
                    if crate::transducer::sentence::coboundary_gauge(&na, &nb).is_some() {
                        survivor = self.absorb_pair(survivor, crate::transducer::address::Address::from(sib));
                    }
                }
                return survivor;
            }
        }
        let Some((sib, _)) = crate::transducer::stalk::congruent(
            &self.stalks,
            |a| {
                if a == addr {
                    None
                } else {
                    self.word_at(a)
                }
            },
            &word,
        ) else {
            return crate::transducer::address::Address::from_bytes(addr);
        };
        self.absorb_pair(crate::transducer::address::Address::from_bytes(addr), sib)
    }

    /// Absorb one of `new` / `sib` into the other: a bundle keeps the
    /// fibre; two cards make the newly admitted word the bundle. Pays the
    /// absorbed discoverer. Returns the surviving address.
    pub(crate) fn absorb_pair(&mut self, new: crate::transducer::address::Address, sib: crate::transducer::address::Address) -> crate::transducer::address::Address {
        let Some(nw) = self.basis.get(&new) else {
            return new;
        };
        let Some(sw) = self.basis.get(&sib) else {
            return new;
        };
        let new_card = nw.sections().is_empty() && !crate::transducer::sentence::carries_torsion(nw);
        let sib_card = sw.sections().is_empty() && !crate::transducer::sentence::carries_torsion(sw);
        let (base, absorbed) = if new_card && !sib_card { (sib.clone(), new.clone()) } else { (new.clone(), sib.clone()) };
        let Some(g) = element::transition(
            self.basis.get(&absorbed).map(|w| w.sub()).unwrap_or(&[]),
            self.basis.get(&base).map(|w| w.sub()).unwrap_or(&[]),
        ) else {
            return new;
        };
        let Some(card) = self.basis.get(&absorbed).map(|w| w.element().clone()) else {
            return new;
        };
        let Some(disc) = self.discoverer_of(&absorbed) else {
            return new;
        };
        self.basis_remove(&absorbed);
        if self.word_mut(&base).map(|w| w.update(|e| e.attach(&card, &g))).unwrap_or(0) > 0 {
            let after = self
                .basis
                .get(&base)
                .map(|w| w.element().clone())
                .unwrap_or_else(|| card.clone());
            let payload = crate::transducer::rewrite::RewritePayload::Condense {
                sections: after.sections().to_vec(),
                gauge: g.clone(),
            };
            let _ = self.mint_rewrite(
                crate::transducer::rewrite::RewriteKind::Condense,
                std::slice::from_ref(&card),
                &after,
                payload,
                disc,
            );
            self.physics.syntropy(&[0, 1, 0, 0]);
            self.torse_at(&base);
        }
        base
    }

    /// Torse one word: a closed orbit pays the torse coin on the residual
    /// lane immediately, not on the idle cursor.
    pub(crate) fn torse_at(&mut self, addr: &[u8]) {
        let spanned = match self.word_mut(addr) {
            Some(w) => w.update(|e| e.torse()),
            None => return,
        };
        if spanned == 0 {
            return;
        }
        let Some(after) = self.basis.get(addr).map(|w| (**w).clone()) else {
            return;
        };
        let lanes = after.lanes();
        self.physics.torse_relief(lanes, spanned as u64);
        // The payload carries a cycle the word actually filed: the one with
        // the least class coordinates, canonical so two nodes torsing one
        // word speak one rewrite. `last()` read whichever cycle the Vec
        // happened to end on, and the fallback invented an empty sigma with
        // the torse span standing in for a period that was never witnessed
        // (A17: a period is believed only when the loop closes at it, T5).
        // A word carrying no filed cycle has no torse to speak (rule 8).
        let Some((sigma, period)) = after
            .torsion()
            .iter()
            .min_by(|a, b| a.sigma.cmp(&b.sigma).then(a.period.cmp(&b.period)))
            .map(|c| (c.sigma.clone(), c.period))
        else {
            return;
        };
        let payload = crate::transducer::rewrite::RewritePayload::Torse { sigma, period };
        if let Some(disc) = self.discoverer_of(addr) {
            let _ = self.mint_rewrite(
                crate::transducer::rewrite::RewriteKind::Torse,
                std::slice::from_ref(&after),
                &after,
                payload,
                disc,
            );
        }
        self.touch_at(addr);
    }

    /// Redundancy probe on the admitted word's own stratum: a plain card
    /// the rest now derives retires, and the voice whose word derived it
    /// earns the refine yield.
    pub(crate) fn refine_stratum(&mut self, new: &[u8]) {
        let Some(after) = self.word_at(new).cloned() else {
            return;
        };
        let inv = crate::transducer::stalk::invariant_of(&after);
        let members: Vec<Vec<u8>> = self
            .stalks
            .fiber(&inv)
            .iter().filter(|&a| a.as_slice() != new).cloned()
            .collect();
        let Some(bidder) = self.discoverer_of(new) else {
            return;
        };
        for addr in members {
            let Some(held) = self.basis.get(&addr) else {
                continue;
            };
            if !held.sections().is_empty() || crate::transducer::sentence::carries_torsion(held) {
                continue;
            }
            if self.provisional.contains(&addr) {
                continue;
            }
            let candidate = (**held).clone();
            if !matches!(self.reduce_excluding(&candidate, Some(addr.as_slice())), NormalForm::Decomposed(_)) {
                continue;
            }
            let chart = crate::transducer::stalk::invariant_of(&candidate);
            let axis = addr[0] as u64;
            let payload = crate::transducer::rewrite::RewritePayload::Refine { chart, axis };
            let _ = self.mint_rewrite(
                crate::transducer::rewrite::RewriteKind::Refine,
                std::slice::from_ref(&candidate),
                &after,
                payload,
                bidder,
            );
            self.basis_remove(&addr);
            self.physics.syntropy(&candidate.magnitude());
            self.append_step(crate::chronometry::ActKind::Ruling, &addr, bidder);
        }
    }

    /// The word at an address, read by its coordinate. Restored keys are
    /// the coordinates that were written; an older file is refused.
    pub fn word_at(&self, addr: &[u8]) -> Option<&Element> {
        self.basis
            .get(addr)
            .map(|w| w.element())
            .or_else(|| self.cavity_book.get(addr))
            .or_else(|| self.provisional.get(addr))
    }

    /// THE HELD WORD THIS COURT INSTALLED (L16): the element AND the form
    /// its tape decoded to, for a reader that needs the form. The basis
    /// alone — a cavity and a provisional are elements this court has not
    /// installed, and `word_at` falls through to them on purpose.
    pub(crate) fn held_word(&self, addr: &[u8]) -> Option<&crate::transducer::element::Held> {
        self.basis.get(addr).map(|w| w.as_ref())
    }

    /// The word this court INSTALLED, never a provisional one. `word_at`
    /// resolves for replay and falls through to `provisional` and the
    /// cavity on purpose; the teaching path must not, because a taught word
    /// is crystallized on the receiver (24.1). Chapter 23.2 states the same
    /// rule for `resolve_form`: provisional and dialect return `None`.
    pub(crate) fn installed_word(&self, addr: &[u8]) -> Option<&Element> {
        self.basis.get(addr).map(|w| w.element())
    }

    /// A word the court finalized here joins the ruled cover (keyed admit).
    ///
    /// This does NOT write the court's own Čech section, though every one
    /// of its callers rules a word. `glue_fetched`'s `Admitted` arm calls it
    /// for a word a PEER sent, and sectioning that at this court's own
    /// vantage would file the peer's tape as this court's re-derivation —
    /// the same byte exactness one level down (1.7). The three sites where
    /// the word is this court's own call `section_own` by name.
    pub(crate) fn ruled_admit(&mut self, addr: &[u8]) {
        if let Some(w) = self.word_at(addr) {
            let inv = crate::transducer::stalk::invariant_of(w);
            let words = crate::transducer::sentence::words_of_element(w);
            // 1.8: a ruled form's word vertices are the generators this
            // court can compose a spoken path through (50.2 step 2). Read
            // before the carriers are touched, so the word's borrow ends
            // here and not across them. `word_at` reaches the cavity and
            // the provisional as well as the basis, so the decode is named
            // here rather than read off a held word that may not exist yet:
            // `ruled_admit` runs BEFORE the word is installed on some paths
            // (L16).
            let form = crate::transducer::sentence::from_element_form(w);
            self.ruled.admit(&inv, addr);
            crate::transducer::sentence::admit_words(&mut self.axis_index, &inv, &words);
            if let Some(f) = form {
                let a = crate::transducer::address::Address::from(addr.to_vec());
                let mut ix = std::mem::take(&mut self.generators);
                crate::transducer::sentence::admit_generators(&mut ix, &a, &f);
                self.generators = ix;
            }
            self.ruled_addrs.insert(addr, ());
            self.refresh_ruled_root();
        }
        // 21 step 3: this address is ruled now, so every sentence that
        // waited on it as a citation is heard again. Outside the
        // `word_at` borrow, and after the cover carries it — a re-hearing
        // that ran first would find it still unruled and wait again.
        self.rehear_awaiting(addr);
    }

    /// A word this court reached on its OWN tapes and ruled: it joins the
    /// ruled cover and becomes this court's section on that chart, at its
    /// own vantage (13.9, 13.10). The stroke's own ruling, the home's
    /// install, and an upheld verdict are the three; a peer's glued word is
    /// not one of them.
    pub(crate) fn rule_own(&mut self, addr: &[u8]) {
        self.ruled_admit(addr);
        let Some(w) = self.word_at(addr) else { return };
        let inv = crate::transducer::stalk::invariant_of(w);
        self.section_own(&inv, &crate::transducer::address::Address::from(addr.to_vec()));
    }

    pub(crate) fn ruled_contains(&self, addr: &[u8]) -> bool {
        self.ruled_addrs.contains(addr)
    }

    /// A crystallized form this hypha has ruled, or retired to the cavity,
    /// read as a `Form` for the court's replay of a citer. Provisional and
    /// heard words resolve to nothing: a citation stands only on a ruling.
    pub fn resolve_form(&self, addr: &[u8]) -> Option<crate::transducer::sentence::Form> {
        if !self.ruled_contains(addr) && !self.cavity_book.contains(addr) {
            return None;
        }
        // A RULED WORD is installed and its form is a field; a CAVITY word
        // is not, and its tape is decoded here by name (L16).
        if let Some(f) = self.held_word(addr).and_then(|h| h.form()) {
            return Some(f.clone());
        }
        crate::transducer::sentence::from_element_form(self.word_at(addr)?)
    }

    /// THE DOCKET (Step 5): every provisional word, exported for the court's
    /// blind replay + confluence audit — (element, attesting voices, escrowed μ⃗).
    pub fn dockets(&self) -> Vec<Docketed> {
        self.dockets_since(0)
    }

    /// The provisional words installed past a court's "since" k-step.
    pub fn dockets_since(&self, since_k: u64) -> Vec<Docketed> {
        self.provisional
            .fiber(crate::network::nerve::ROOT)
            .filter_map(|(addr, spoken)| {
                let k = self.provisional_k.get(&addr).copied()?;
                if k <= since_k && since_k > 0 {
                    return None;
                }
                let held = self.treasury.escrow_of(&addr)?;
                // A word whose deflation is unmeasured is not served: the
                // court rules what was measured, never a fabricated zero.
                let deflation = self.deflation_of(&addr)?;
                if deflation.iter().all(|&x| x == 0) {
                    return None;
                }
                Some(Docketed {
                    addr: Address::from(addr),
                    spoken: spoken.clone(),
                    voices: held.voices.clone(),
                    escrowed: held.mu,
                    deflation,
                    k,
                })
            })
            .collect()
    }

}
