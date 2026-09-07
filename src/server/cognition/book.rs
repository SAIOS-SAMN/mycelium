// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! The book: listings, the coherence field, the language snapshot,
//! the market's open page and what a probe settles on it.
//!
//! Split from cognition.rs under Stage 8's reading bound. A private
//! item is visible to its module AND its descendants, so these are the
//! same `impl Kernel` reaching the same private carriers: the split
//! moves code and changes nothing else.

use super::*;

impl Kernel {
    pub(super) fn drain_unreachable(&mut self) {
        let kappa = crate::physics::aperture_cycle();
        let mut budget = self.physics.stroke();
        let mut keys: Vec<Vec<u8>> = Vec::new();
        // index: budgeted keyed cursor over the open docket
        for (k, _) in self.open.cursor(crate::network::nerve::ROOT, &mut budget) {
            keys.push(k);
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
    pub(super) fn book_list(&mut self, residual: Element, k_now: u64, voice: [u8; 32]) {
        let addr = residual.address();
        self.stamp_standing(&addr);
        let mag = residual.magnitude();
        self.physics.bump(&mag);
        let inv = crate::transducer::stalk::invariant_of(&residual);
        self.listing_atlas.admit(&inv, &addr);
        self.tension.insert(&addr, mag);
        self.on_page.insert(&addr, ());
        self.drawers.insert(&addr, voice);
        self.occupy(&addr, &mag, true);
        self.pending.insert(&addr, (residual, k_now));
        self.book_gen += 1;
        self.append_step(&addr, voice);
    }

    fn pending_take(&mut self, addr: &[u8]) -> Option<Element> {
        let (elem, _) = self.pending.remove(addr)?;
        let mag = elem.magnitude();
        self.occupy(addr, &mag, false);
        self.drawers.remove(addr);
        self.physics.relieve(&mag);
        Some(elem)
    }

    /// True closure of the whole listing: the thought glued. Leaves the
    /// page and the listing atlas (the word, if any, lives on the language).
    pub(super) fn unlist(&mut self, addr: &[u8], as_cavity: bool) {
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
    pub(super) fn relocate_cavity(&mut self, addr: &[u8]) {
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
        self.cavity_book.insert(&addr, elem.clone());
        self.cavities.push(elem);
        self.chill_field(addr);
        self.book_gen += 1;
    }

    fn recompute_tension(&mut self, addr: &[u8]) {
        let t = if self.open.contains(addr) {
            self.field
                .get(addr)
                .filter(|f| f.nonflat > 0)
                .map(|f| element::measure_lanes(&union_remainders(&f.entries)))
        } else {
            self.book_elem(addr)
                .map(|p| p.magnitude())
                .or_else(|| self.cavity_book.get(addr).map(|e| e.magnitude()))
        };
        match t {
            Some(t) => {
                self.tension.insert(&addr, t);
            }
            None => {
                self.tension.remove(addr);
            }
        }
    }

    pub(super) fn write_field(
        &mut self,
        addr: &[u8],
        chart: crate::transducer::stalk::Invariant,
        state: crate::transducer::stalk::Closure,
        remainder: Vec<(i64, i64, u8, u8)>,
        before: &[u64; 4],
        after: &[u64; 4],
    ) {
        use crate::transducer::stalk::Closure;
        let mut listing = self
            .field
            .get(addr)
            .cloned()
            .or_else(|| self.field_cold.remove(addr))
            .unwrap_or_default();
        for e in &mut listing.entries {
            if e.state.is_flat() {
                continue;
            }
            for l in 0..4 {
                if before[l] > 0 && after[l] == before[l] {
                    e.stagnant[l] = e.stagnant[l].saturating_add(1);
                } else {
                    e.stagnant[l] = 0;
                }
            }
        }
        let mut stag = [0u32; 4];
        // index: the four lanes
        for l in 0..4 {
            if before[l] > 0 && after[l] == before[l] {
                stag[l] = 1;
            }
        }
        let now_flat = state.is_flat();
        let idxs: Vec<usize> = listing
            .entries
            .iter()
            .enumerate()
            .filter(|(_, e)| e.chart == chart)
            .map(|(i, _)| i)
            .collect();
        if !idxs.is_empty() {
            for i in idxs {
                let e = &mut listing.entries[i];
            if !e.state.is_flat() {
                stag = e.stagnant;
            }
            if e.state.is_flat() != state.is_flat() && !state.is_flat() {
                // keep atlas admission for a newly opened chart
            }
            if matches!(
                (&e.state, &state),
                (Closure::Twisted(_) | Closure::Class(..) | Closure::Cavity, Closure::Flat)
            ) {
                let base = self.listing_atlas.chart_of(addr).cloned();
                if base.as_ref() != Some(&chart) {
                    self.listing_atlas.retract(&chart, addr);
                }
            }
            let was_flat = e.state.is_flat();
            e.state = state.clone();
            e.remainder = remainder.clone();
            e.stagnant = stag;
            if was_flat && !now_flat {
                listing.nonflat = listing.nonflat.saturating_add(1);
            } else if !was_flat && now_flat {
                listing.nonflat = listing.nonflat.saturating_sub(1);
            }
            }
        } else {
            if !state.is_flat() {
                listing.nonflat = listing.nonflat.saturating_add(1);
            }
            listing.entries.push(FieldEntry {
                chart: chart.clone(),
                state,
                remainder,
                stagnant: stag,
            });
        }
        if listing.nonflat > 0 {
            self.open.insert(&addr, ());
            self.field.insert(&addr, listing);
        } else {
            self.open.remove(addr);
            self.field_cold.insert(addr, listing);
            self.field.remove(addr);
        }
        let mag = self.tension.get(addr).copied().unwrap_or(*after);
        let lanes = crate::network::nerve::occupied_lanes(&mag);
        // index: the four lanes
        for lane in &lanes {
            if now_flat {
                self.own_cover.retract(*lane, &chart, addr);
            } else {
                self.own_cover.admit(*lane, &chart, addr);
            }
        }
        let ck = chart.key();
        match self.chart_listings.get_mut(&ck) {
            Some(m) => {
                if now_flat {
                    m.remove(addr);
                } else {
                    m.insert(&addr, ());
                }
            }
            None if !now_flat => {
                let mut m = crate::network::nerve::Keyed::new();
                m.insert(&addr, ());
                self.chart_listings.insert(ck, m);
            }
            None => {}
        }
        self.recompute_tension(addr);
    }

    /// The remainder face of a listing: the unique non-flat chart. Several
    /// open charts are the field, not a representative.
    pub fn face_of(&self, addr: &[u8]) -> Option<&crate::transducer::stalk::Invariant> {
        self.field
            .get(addr)
            .or_else(|| self.field_cold.get(addr))
            .and_then(|f| {
                if f.nonflat == 0 {
                    return None;
                }
                let mut one = None;
                for e in &f.entries {
                    if e.state.is_flat() {
                        continue;
                    }
                    if one.is_some() {
                        return None;
                    }
                    one = Some(&e.chart);
                }
                one
            })
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

    /// Book generation: the height the quote and demand replies carry.
    pub fn book_height(&self) -> u64 {
        self.book_gen
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
            for l in 0..4 {
                s[l] = s[l].max(e.stagnant[l]);
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
    pub(super) fn condense_on(&mut self, addr: &[u8]) -> crate::transducer::a21::Address {
        let Some(word) = self.word_at(addr).cloned() else {
            return crate::transducer::a21::Address::from_bytes(addr);
        };
        if word.is_sentence() {
            let inv = crate::transducer::stalk::invariant_of(&word);
            let members: Vec<Vec<u8>> = self.stalks.fiber(&inv).iter().cloned().filter(|a| a != addr).collect();
            if let Some(na) = crate::transducer::sentence::from_element_form(&word) {
                let mut survivor = crate::transducer::a21::Address::from_bytes(addr);
                for sib in members {
                    let Some(other) = self.word_at(&sib).cloned() else { continue };
                    let Some(nb) = crate::transducer::sentence::from_element_form(&other) else { continue };
                    if crate::transducer::sentence::coboundary_gauge(&na, &nb).is_some() {
                        survivor = self.absorb_pair(survivor, crate::transducer::a21::Address::from(sib));
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
            return crate::transducer::a21::Address::from_bytes(addr);
        };
        self.absorb_pair(crate::transducer::a21::Address::from_bytes(addr), sib)
    }

    /// Absorb one of `new` / `sib` into the other: a bundle keeps the
    /// fibre; two cards make the newly admitted word the bundle. Pays the
    /// absorbed discoverer. Returns the surviving address.
    pub(super) fn absorb_pair(&mut self, new: crate::transducer::a21::Address, sib: crate::transducer::a21::Address) -> crate::transducer::a21::Address {
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
        let Some(card) = self.basis.get(&absorbed).map(|w| (**w).clone()) else {
            return new;
        };
        let Some(disc) = self.discoverer_of(&absorbed) else {
            return new;
        };
        self.basis_remove(&absorbed);
        if self.word_mut(&base).map(|w| w.attach(&card, &g)).unwrap_or(0) > 0 {
            let after = self.basis.get(&base).map(|w| (**w).clone()).unwrap_or_else(|| card.clone());
            let payload = crate::transducer::rewrite::RewritePayload::Condense {
                sections: after.sections().to_vec(),
                gauge: g.clone(),
            };
            let _ = self.mint_rewrite(
                crate::transducer::rewrite::RewriteKind::Condense,
                &[card.clone()],
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
    pub(super) fn torse_at(&mut self, addr: &[u8]) {
        let spanned = match self.word_mut(addr) {
            Some(w) => w.torse(),
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
            .min_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)))
            .map(|(s, p, _)| (s.clone(), *p))
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
    pub(super) fn refine_stratum(&mut self, new: &[u8]) {
        let Some(after) = self.word_at(new).cloned() else {
            return;
        };
        let inv = crate::transducer::stalk::invariant_of(&after);
        let members: Vec<Vec<u8>> = self
            .stalks
            .fiber(&inv)
            .iter()
            .cloned()
            .filter(|a| a.as_slice() != new)
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
                &[candidate.clone()],
                &after,
                payload,
                bidder,
            );
            self.basis_remove(&addr);
            self.physics.syntropy(&candidate.magnitude());
            self.append_step(&addr, bidder);
        }
    }

    /// The word at an address, read by its coordinate. Restored keys are
    /// the coordinates that were written; an older file is refused.
    pub fn word_at(&self, addr: &[u8]) -> Option<&Element> {
        self.basis
            .get(addr)
            .map(|w| w.as_ref())
            .or_else(|| self.cavity_book.get(addr))
            .or_else(|| self.provisional.get(addr))
    }

    /// The word this court INSTALLED, never a provisional one. `word_at`
    /// resolves for replay and falls through to `provisional` and the
    /// cavity on purpose; the teaching path must not, because a taught word
    /// is crystallized on the receiver (24.1). Chapter 23.2 states the same
    /// rule for `resolve_form`: provisional and dialect return `None`.
    pub(super) fn installed_word(&self, addr: &[u8]) -> Option<&Element> {
        self.basis.get(addr).map(|w| w.as_ref())
    }

    /// A word the court finalized here joins the ruled cover (keyed admit).
    pub(super) fn ruled_admit(&mut self, addr: &[u8]) {
        if let Some(w) = self.word_at(addr) {
            let inv = crate::transducer::stalk::invariant_of(w);
            let words = crate::transducer::sentence::words_of_element(w);
            self.ruled.admit(&inv, addr);
            crate::transducer::sentence::admit_words(&mut self.axis_index, &inv, &words);
            self.ruled_addrs.insert(&addr, ());
            self.refresh_ruled_root();
        }
    }

    pub(super) fn ruled_contains(&self, addr: &[u8]) -> bool {
        self.ruled_addrs.contains(addr)
    }

    /// A crystallized form this hypha has ruled, or retired to the cavity,
    /// read as a `Form` for the court's replay of a citer. Provisional and
    /// heard words resolve to nothing: a citation stands only on a ruling.
    pub fn resolve_form(&self, addr: &[u8]) -> Option<crate::transducer::sentence::Form> {
        if !self.ruled_contains(addr) && !self.cavity_book.contains(addr) {
            return None;
        }
        let e = self.word_at(addr)?;
        e.form().cloned()
    }

    /// List a sentence on the docket if it is not already provisional.
    fn docket_sentence(&mut self, word: &Element, ratifier: [u8; 32]) {
        self.docket_connection(word, ratifier);
    }

    /// Perceive a family's speech: replay each sentence, docket what stands.
    /// A false sentence is refused with the replay reason in the journal
    /// (`court refuse speech {addr}: {reason}`) before the wire code.
    /// `task` binds the stood sentence to that family's trains so install
    /// can mint only measured descent.
    pub fn perceive_speech(&mut self, voice: [u8; 32], task: &[u8], sentences: &[Element]) -> Vec<u8> {
        let task = task;
        let mut codes = Vec::with_capacity(sentences.len());
        let family = self.task_family.get(task).cloned();
        for s in sentences {
            if let Some(reason) = crate::court::verdict::blind_replay_on(
                s,
                &|a| self.resolve_form(a),
                family.as_deref(),
                self,
            ) {
                let a = s.address();
                eprintln!("  court refuse speech {:02x}{:02x}…: {reason}", a[0], a[1]);
                codes.push(crate::anastome::dialogue::SPEECH_REFUSED);
                continue;
            }
            self.docket_sentence(s, voice);
            if task != [0u8; 32] {
                self.speech_task.insert(s.address(), Address::from(task));
            }
            codes.push(crate::anastome::dialogue::SPEECH_STOOD);
        }
        codes
    }

    /// List a word on the docket if it is not already provisional.
    pub(super) fn docket_connection(&mut self, word: &Element, ratifier: [u8; 32]) {
        if word.is_identity() && !word.is_rewrite() {
            return;
        }
        let addr = word.address();
        if self.provisional.contains(&addr) {
            return;
        }
        let mag = word.magnitude();
        let mu = if word.is_rewrite() {
            mag
        } else {
            crate::economy::price(&mag, &self.physics.pressure)
        };
        let anchor = self.anchor();
        self.treasury.escrow_award(&addr, vec![ratifier], mu, anchor);
        self.deflations.insert(&addr, mag);
        let k_now = self.k_step();
        self.provisional_k.insert(&addr, k_now);
        self.provisional.insert(&addr, word.clone());
        self.stamp_standing(&addr);
        self.provisional_h.insert(&addr, self.height());
        self.mark_persist();
        let inv = crate::transducer::stalk::invariant_of(word);
        if self.field.get(&addr).is_none() {
            self.write_field(
                &addr,
                inv,
                crate::transducer::stalk::Closure::Cavity,
                word.sub().to_vec(),
                &mag,
                &mag,
            );
        }
        self.book_gen += 1;
        self.append_step(&addr, ratifier);
    }

    /// Speak an optimisation: refuse a zero vector, docket the Rewrite,
    /// Offer it, and keep it as language. Payment waits on the second organ.
    fn offer_rewrite(&mut self, rw: Element, voice: [u8; 32]) -> bool {
        if !rw.is_rewrite() || rw.magnitude().iter().all(|&x| x == 0) {
            return false;
        }
        let addr = rw.address();
        self.docket_connection(&rw, voice);
        self.offered.insert(addr, ());
        true
    }

    pub(super) fn mint_rewrite(
        &mut self,
        kind: crate::transducer::rewrite::RewriteKind,
        before: &[Element],
        after: &Element,
        payload: crate::transducer::rewrite::RewritePayload,
        voice: [u8; 32],
    ) -> bool {
        let Some(mu) = self.treasury.settle_rewrite(kind, before, after) else {
            return false;
        };
        let before_addr: Vec<Address> = before.iter().map(|e| e.address()).collect();
        let Some(rw) = crate::transducer::rewrite::mint(kind, &before_addr, after.address(), payload, mu) else {
            return false;
        };
        self.offer_rewrite(rw, voice)
    }

    pub(super) fn retire_rewrite_inputs(&mut self, rw: &Element) {
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

    /// The court's voice, heard on its verdicts.
    pub fn set_court_voice(&mut self, voice: [u8; 32]) {
        self.court_voice = Some(voice);
    }

    /// The digests the court reconciles against: the ruled cover, served to
    /// the court's own voice. Any other asker (an entity's sync) is answered
    /// with the whole language elsewhere.
    pub fn ruled_digests_for(&self, voice: &[u8; 32]) -> Option<Vec<crate::transducer::sheaf::StratumDigest>> {
        (self.court_voice.as_ref() == Some(voice)).then(|| crate::transducer::sheaf::digests(&self.ruled))
    }

    /// Remove the word at `addr` from the basis, its discoverer, and its stalk.
    pub(super) fn basis_remove(&mut self, addr: &[u8]) -> Element {
        let Some(held) = self.basis.get(addr).map(|w| (**w).clone()) else {
            return Element::identity();
        };
        self.unindex_word(&held);
        let inv = crate::transducer::stalk::invariant_of(&held);
        let words = crate::transducer::sentence::words_of_element(&held);
        crate::transducer::sentence::retract_words(&mut self.axis_index, &inv, &words);
        self.discoverers.remove(addr);
        std::sync::Arc::make_mut(&mut self.stalks).remove(addr);
        self.ruled.remove(addr);
        if self.ruled_addrs.remove(addr).is_some() {
            self.refresh_ruled_root();
        }
        self.basis_gen += 1;
        self.retire_gen += 1;
        let removed = self.basis.remove(addr).unwrap_or_else(|| std::sync::Arc::new(held.clone()));
        self.language_drop(addr);
        (*removed).clone()
    }

    pub(super) fn basis_clear(&mut self) {
        self.basis = crate::network::nerve::Keyed::new();
        self.by_action = crate::network::nerve::Keyed::new();
        self.by_footprint = crate::network::nerve::Keyed::new();
        self.discoverers = crate::network::nerve::Keyed::new();
        self.stalks = std::sync::Arc::new(crate::transducer::stalk::Stalks::default());
        self.basis_gen += 1;
        self.language_rebuild();
    }

    /// A held word changed in place: the Arc in `basis` is already unique
    /// (via `word_mut`); rebuild the language pointer slice.
    pub(super) fn touch_at(&mut self, _addr: &[u8]) {
        self.basis_gen += 1;
        self.language_rebuild();
    }

    /// The language as an immutable shared snapshot. An append clones the
    /// pointer slice and one new Arc; a retirement rebuilds. Never clones
    /// the whole basis of Elements.
    pub fn language(&mut self) -> std::sync::Arc<Language> {
        if self.language_gen != self.basis_gen || self.language.words.len() != self.basis.len() {
            self.language_rebuild();
        }
        std::sync::Arc::clone(&self.language)
    }

    fn language_from_words(
        &self,
        words: Vec<std::sync::Arc<Element>>,
        stalks: std::sync::Arc<crate::transducer::stalk::Stalks>,
        index: crate::network::nerve::Keyed<usize>,
    ) -> Language {
        let shapes = (0..words.len()).map(|_| std::sync::OnceLock::new()).collect();
        Language {
            words,
            stalks,
            shapes,
            retire_gen: self.retire_gen,
            index,
        }
    }

    pub(super) fn language_rebuild(&mut self) {
        let mut words = Vec::new();
        let mut index = crate::network::nerve::Keyed::new();
        for (addr, w) in self.basis.fiber(crate::network::nerve::ROOT) {
            index.insert(&addr, words.len());
            words.push(std::sync::Arc::clone(w));
        }
        let stalks = std::sync::Arc::clone(&self.stalks);
        self.language = std::sync::Arc::new(self.language_from_words(words, stalks, index));
        self.language_gen = self.basis_gen;
    }

    fn language_drop(&mut self, addr: &[u8]) {
        let words: Vec<std::sync::Arc<Element>> = self
            .language
            .words
            .iter()
            .filter(|w| w.address().as_bytes() != addr)
            .cloned()
            .collect();
        let mut index = crate::network::nerve::Keyed::new();
        for (i, w) in words.iter().enumerate() {
            index.insert(w.address(), i);
        }
        let stalks = std::sync::Arc::clone(&self.stalks);
        self.language = std::sync::Arc::new(self.language_from_words(words, stalks, index));
        self.language_gen = self.basis_gen;
    }

    pub(super) fn language_append_arc(&mut self, word: std::sync::Arc<Element>) {
        let addr = word.address();
        let inv = crate::transducer::stalk::invariant_of(word.as_ref());
        std::sync::Arc::make_mut(&mut self.stalks).admit(&inv, &addr);
        if self.retire_gen != self.language.retire_gen {
            self.language_rebuild();
            return;
        }
        // MODIFY the snapshot, never build another beside it. This copied
        // the whole word vector AND deep-cloned the whole index trie AND
        // allocated N fresh OnceLocks per word admitted -- four O(N) passes
        // for one observation, and the fresh `shapes` discarded every shape
        // lifted so far, so each admit also forced them all to be lifted
        // again. `Arc::make_mut` mutates in place when nobody is reading and
        // copies once when someone is: the cost of sharing is paid by the
        // sharing, not by the growth.
        let stalks = std::sync::Arc::clone(&self.stalks);
        let retire_gen = self.retire_gen;
        let lang = std::sync::Arc::make_mut(&mut self.language);
        lang.stalks = stalks;
        lang.retire_gen = retire_gen;
        match lang.index.get(&addr).copied() {
            Some(i) if i < lang.words.len() => {
                // One vertex, one slot: the word at its address is replaced,
                // and its lifted shape with it.
                lang.words[i] = word;
                lang.shapes[i] = std::sync::OnceLock::new();
            }
            _ => {
                let i = lang.words.len();
                lang.index.insert(&addr, i);
                lang.words.push(word);
                lang.shapes.push(std::sync::OnceLock::new());
            }
        }
        self.language_gen = self.basis_gen;
    }

    /// THE MARKET VIEW — under a brief lock: the language snapshot, the open
    /// book, the yield surface, the escrow pool, the clock, and the reserve.
    /// Quoting and the cohesion witness compute over this with no lock held.
    pub fn market_open(&mut self) -> MarketView {
        self.remeasure_pressure();
        let gens = (self.basis_gen, self.book_gen);
        let closure = match (self.closure.0, self.closure.1) == gens {
            true => Some(std::sync::Arc::clone(&self.closure.2)),
            false => None,
        };
        if self.book.0 != self.book_gen {
            self.book = (
                self.book_gen,
                std::sync::Arc::new(self.pending.clone()),
            );
        }
        let now = self.height();
        let mut standing_t = crate::network::nerve::Keyed::new();
        for (addr, _) in self.pending.fiber(crate::network::nerve::ROOT) {
            standing_t.insert(&addr, self.t_extent(&addr, now));
        }
        MarketView {
            language: self.language(),
            pending: std::sync::Arc::clone(&self.book.1),
            surface: crate::economy::yield_surface(&self.physics),
            pressure: self.physics.pressure,
            pool: self.treasury.bond_of(&self.identity),
            k_step: self.chain.next_step_index(),
            head_k: self.chain.head_k(),
            standing_t: std::sync::Arc::new(standing_t),
            reserve: self.physics.reserve,
            gens,
            closure,
            prior: std::sync::Arc::clone(&self.closure.2),
            chart_listings: std::sync::Arc::new(self.chart_listings.clone()),
        }
    }

    /// Seal a rank measured off the lock, if the generations it was
    /// measured at still hold. Returns the rank now in force.
    pub fn seal_closure(&mut self, gens: (u64, u64), rank: Rank) -> std::sync::Arc<Rank> {
        // The measurement is served and kept with the generations it was
        // measured at: the market view reads it as current only when those
        // generations still hold, and as the prior the next measurement
        // continues from otherwise — so a book that never stands still is
        // still measured incrementally, never from nothing.
        let measured = std::sync::Arc::new(rank);
        self.closure = (gens.0, gens.1, std::sync::Arc::clone(&measured));
        measured
    }

    /// The rank in force, measured now under the lock (tests and local use;
    /// the server measures off the lock through the market view).
    pub(super) fn closure_now(&mut self) -> std::sync::Arc<Rank> {
        let gens = (self.basis_gen, self.book_gen);
        if (self.closure.0, self.closure.1) != gens {
            let language = self.language();
            let measured = rank_over(&language, &self.pending, &self.closure.2, gens.0, &self.chart_listings);
            self.closure = (gens.0, gens.1, std::sync::Arc::new(measured));
        }
        std::sync::Arc::clone(&self.closure.2)
    }

    /// APPLY an attach ruling: the court connected `sibling` to `base` by
    /// the transport g. A provisional listing of the sibling clears with its
    /// award evaporating (it is not a base word). When both words are held
    /// here and g carries the sibling onto the base, the sibling transports
    /// into the bundle as a section and its card retires — its discoverer
    /// earns the section subsidy when the fibre gained volume. Returns the
    /// cards retired (0 or 1).
    pub fn apply_attach(
        &mut self,
        sibling: &[u8],
        base: &[u8],
        g: &element::Versor,
    ) -> usize {
        if self.provisional.remove(sibling).is_some() {
            let _ = self.treasury.revoke_award(sibling, false);
        }
        let Some(base_pos) = self.basis.get(base).map(|w| element::positions(w.sub())) else {
            return 0;
        };
        let Some(sib) = self.basis.get(sibling).map(|w| (**w).clone()) else {
            return 0;
        };
        if element::pullback(&base_pos, sib.sub(), g).is_none() {
            return 0;
        };
        let Some(disc) = self.discoverer_of(sibling) else {
            return 0;
        };
        let gained = self.word_mut(base).map(|w| w.attach(&sib, g)).unwrap_or(0);
        self.touch_at(base);
        self.basis_remove(sibling);
        if gained > 0 {
            if let Some(after) = self.basis.get(base).map(|w| (**w).clone()) {
                let payload = crate::transducer::rewrite::RewritePayload::Condense {
                    sections: after.sections().to_vec(),
                    gauge: g.clone(),
                };
                let _ = self.mint_rewrite(
                    crate::transducer::rewrite::RewriteKind::Condense,
                    &[sib.clone()],
                    &after,
                    payload,
                    disc,
                );
                self.torse_at(base);
            }
        }
        self.physics.syntropy(&[0, 0, 1, 0]);
        1
    }

    /// SETTLE A PROBE: the replayed residual of an open listing against the
    /// speaker's sentence. Closure (empty residual) of a fibre section or a
    /// connection listing pays morphism deflation; a tape covering pays
    /// nothing. A partial descent updates tension and relieves the physics
    /// (the measurement is real) but pays μ⃗ = 0⃗ — covering cells is not
    /// an axial move. Cited words still earn settle_use. Returns
    /// (code, paid μ⃗, remaining μ⃗).
    pub fn settle_probe(
        &mut self,
        voice: [u8; 32],
        target: &[u8],
        residual: Vec<(i64, i64, u8, u8)>,
        cites: &[crate::anastome::dialogue::CiteWire],
        section: bool,
    ) -> (u8, [u64; 4], [u64; 4]) {
        use crate::anastome::dialogue::{PROBE_CLOSED, PROBE_DESCENT, PROBE_NOTHING};
        use crate::transducer::stalk::Closure;
        let Some((listing, _)) = self.pending.get(target) else {
            return (PROBE_NOTHING, [0; 4], [0; 4]);
        };
        let listing = listing.clone();
        self.record_spoken(Address::from(target), voice);
        let before = self.tension.get(target).copied().unwrap_or_else(|| listing.magnitude());
        let rem = element::measure_lanes(&residual);
        let after = rem;
        let anchor = self.anchor();
        let mag = listing.magnitude();
        let mut charts_at: crate::network::nerve::Keyed<crate::transducer::stalk::Invariant> =
            crate::network::nerve::Keyed::new();
        for (_, _, _, _, body) in cites {
            let inv = crate::transducer::stalk::invariant(body);
            charts_at.insert(inv.key(), inv);
        }
        let mut charts: Vec<crate::transducer::stalk::Invariant> = charts_at
            .fiber(crate::network::nerve::ROOT)
            .map(|(_, inv)| inv.clone())
            .collect();
        if charts.is_empty() {
            charts.push(crate::transducer::stalk::invariant(listing.sub()));
        }
        // The transport a flat loop is recorded under is the composite of
        // the cites that flattened it, by the geometric product, not the
        // first cite's versor with the identity invented when there are
        // none. A versor invented where no transport exists is a claim the
        // court cannot replay (rule 11); `None` records no loop at all.
        let cite_g: Option<element::Versor> = cites
            .iter()
            .map(|(_, _, g, _, _)| g.clone())
            .reduce(|acc, g| acc.geo(&g));
        if let [] = residual.as_slice() {
            let paid = if section || listing.is_connection() {
                self.treasury.settle_subsidy(voice, crate::economy::price(&before, &self.physics.pressure), anchor)
            } else {
                [0; 4]
            };
            for (addr, _, _, _, _) in cites {
                if let Some(w) = self.basis.get(addr) {
                    let mag = w.magnitude();
                    if let Some(disc) = self.discoverer_of(addr) {
                        self.treasury.settle_use(voice, disc, mag, anchor);
                    }
                }
            }
            for chart in charts {
                self.write_field(target, chart.clone(), Closure::Flat, Vec::new(), &before, &[0; 4]);
                if let Some(g) = cite_g.clone() {
                    self.record_flat(target, chart, voice, g);
                }
            }
            self.unlist(target, false);
            self.physics.relieve(&mag);
            self.physics.syntropy(&mag);
            self.append_step(target, voice);
            return (PROBE_CLOSED, paid, [0; 4]);
        }
        let mut listed = listing.sub().to_vec();
        listed.sort_unstable();
        if residual == listed {
            let standing = self
                .field
                .get(target)
                .map(|f| union_remainders(&f.entries))
                .filter(|r| !r.is_empty())
                .unwrap_or_else(|| residual.clone());
            for chart in charts {
                self.write_field(target, chart, Closure::Cavity, standing.clone(), &before, &before);
            }
            return (PROBE_NOTHING, [0; 4], before);
        }
        let rem_inv = crate::transducer::stalk::invariant(&residual);
        for chart in &charts {
            if *chart != rem_inv {
                self.write_field(target, chart.clone(), Closure::Flat, Vec::new(), &before, &after);
                if let Some(g) = cite_g.clone() {
                    self.record_flat(target, chart.clone(), voice, g);
                }
            }
        }
        let v_disloc = {
            let base_shape = element::shape_of_chain(listing.sub());
            let rem_shape = element::shape_of_chain(&residual);
            match (base_shape, rem_shape) {
                (Some(sb), Some(sr)) => crate::transducer::blade::versor_between(&sb, &sr),
                _ => None,
            }
        };
        let state = match v_disloc.clone() {
            Some(v) => Closure::Twisted(v),
            None => Closure::Cavity,
        };
        self.write_field(target, rem_inv.clone(), state, residual.clone(), &before, &after);
        self.listing_atlas.admit_also(&rem_inv, target);
        let mag_shift = if let Some(held) = self.pending.get_mut(target) {
            let old = held.0.magnitude();
            held.0.mag[3] = residual.len() as u64;
            if let Some(v) = v_disloc {
                held.0.record_dislocation(element::assignment(&residual), v, element::FIBER_SPATIAL);
            }
            Some((old, held.0.magnitude()))
        } else {
            None
        };
        if let Some((old, newm)) = mag_shift {
            self.occupy(target, &old, false);
            self.occupy(target, &newm, true);
        }
        let settled: [u64; 4] = std::array::from_fn(|l| before[l].saturating_sub(after[l]));
        self.forget_spoken(target);
        self.physics.relieve(&settled);
        self.book_gen += 1;
        self.physics.syntropy(&mag);
        self.append_step(target, voice);
        (PROBE_DESCENT, [0; 4], after)
    }

    fn record_flat(
        &mut self,
        listing: &[u8],
        chart: crate::transducer::stalk::Invariant,
        voice: [u8; 32],
        g: element::Versor,
    ) {
        let key = loop_key(&chart, &voice);
        if self.flat_loops.get(listing).is_some_and(|m| m.contains(&key)) {
            return;
        }
        match self.flat_loops.get_mut(listing) {
            Some(m) => {
                m.insert(key, (chart, g));
            }
            None => {
                let mut m = crate::network::nerve::Keyed::new();
                m.insert(key, (chart, g));
                self.flat_loops.insert(listing, m);
            }
        }
    }

    /// Topological volume: cards, fibre span, open listings. K is algebraic
    /// time and is not a persist event — ticking it must not flush the ledger.
    pub fn volume(&self) -> (usize, usize, usize) {
        (self.basis.len(), self.sections_total(), self.pending.len())
    }

    /// The polytonic kinematic pressure + torsion spectrum (per lane).
    pub fn physics(&self) -> &crate::physics::Physics {
        &self.physics
    }

    /// The settlement ledger (bounties, consilience, recall yields).
    pub fn treasury_entry_count(&self) -> usize {
        self.treasury.entry_count()
    }

    pub fn treasury(&self) -> &crate::economy::Treasury {
        &self.treasury
    }

    /// Restrict stakes to the live genesis identities.
    pub fn bind_live(&mut self, live: &[[u8; 32]]) {
        self.treasury.bind_live(live);
    }

    #[cfg(test)]
    pub(crate) fn treasury_mut(&mut self) -> &mut crate::economy::Treasury {
        &mut self.treasury
    }

    /// Crate-internal: the deliberative arm (synthesis::contemplate) spends
    /// the reserve through here — thought is metabolically real.
    #[cfg(test)]
    pub(crate) fn physics_mut(&mut self) -> &mut crate::physics::Physics {
        &mut self.physics
    }

    /// THE DOCKET (Step 5): every provisional word, exported for the court's
    /// blind replay + confluence audit — (element, attesting voices, escrowed μ⃗).
    pub fn dockets(&self) -> Vec<(Address, Element, Vec<[u8; 32]>, [u64; 4], [u64; 4], u64)> {
        self.dockets_since(0)
    }

    /// The provisional words installed past a court's "since" k-step.
    pub fn dockets_since(&self, since_k: u64) -> Vec<(Address, Element, Vec<[u8; 32]>, [u64; 4], [u64; 4], u64)> {
        self.provisional
            .fiber(crate::network::nerve::ROOT)
            .filter_map(|(addr, spoken)| {
                let k = self.provisional_k.get(&addr).copied()?;
                if k <= since_k && since_k > 0 {
                    return None;
                }
                let (voices, mu, _) = self.treasury.escrow_of(&addr)?;
                // A word whose deflation is unmeasured is not served: the
                // court rules what was measured, never a fabricated zero.
                let deflation = self.deflation_of(&addr)?;
                if deflation.iter().all(|&x| x == 0) {
                    return None;
                }
                Some((Address::from(addr), spoken.clone(), voices.clone(), *mu, deflation, k))
            })
            .collect()
    }
}
