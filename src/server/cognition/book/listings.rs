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

impl Court {
    pub(crate) fn drain_unreachable(&mut self) {
        let kappa = crate::physics::aperture_cycle();
        let mut budget = self.physics.stroke();
        let mut keys: Vec<Vec<u8>> = Vec::new();
        // index: budgeted keyed cursor over the open docket, resumed from
        // the last stroke (rule 10, S13.3) — a circuit, so the drain reads
        // a docket beyond one budget whole across strokes.
        {
            let from = self.drain_cursor.clone();
            let mut cur = self.open.cursor(from.as_deref(), &mut budget);
            for (k, _) in &mut cur {
                keys.push(k);
            }
            if let Some(k) = cur.last_key() {
                self.drain_cursor = Some(k.to_vec());
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
                    && (0..4).all(|l| t[l] == 0 || nonflat.iter().all(|e| e.stagnant[l] >= u64::from(kappa)))
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
    /// Where the listing stands is the stamp of the List act that admits
    /// it (78.24), sealed here.
    pub(crate) fn book_list(&mut self, residual: Element, voice: [u8; 32]) {
        self.list(residual, voice, Some(voice));
    }

    /// THE CORPUS INTAKE'S LISTING (23.8; I31, S12.5b): the hypha lists its
    /// own corpus with no drawer. Room is a voice's — drawn by its utterance
    /// and released by closure, ruling or lapse — and no voice uttered a
    /// corpus task; drawn from the hypha's own bond the corpus exhausted it
    /// in the first minutes of every boot (245 refusals on hypha a in 150 s
    /// of the 2026-09-08 recut) and nothing homed here could lapse it back.
    /// The listing still tensions the book and admits its chart, and its
    /// receipt names the hypha as the actor.
    /// The corpus's listing with its chart already read, outside the
    /// court (`market::prepare_intake`): the lock commits, it never reads.
    pub(crate) fn book_list_charted(&mut self, residual: Element, chart: crate::transducer::stalk::Invariant) {
        let actor = self.identity;
        self.list_charted(residual, chart, actor, None);
    }

    fn list(&mut self, residual: Element, actor: [u8; 32], drawer: Option<[u8; 32]>) {
        let chart = crate::transducer::stalk::invariant_of(&residual);
        self.list_charted(residual, chart, actor, drawer);
    }

    fn list_charted(&mut self, residual: Element, inv: crate::transducer::stalk::Invariant, actor: [u8; 32], drawer: Option<[u8; 32]>) {
        let address = residual.address();
        let mag = residual.magnitude();
        // A LISTING IS DRAWN ONCE (§3.1 as ruled, S13.15): an address
        // already on the page keeps its drawer and its occupancy, so a
        // second vantage on it neither doubles the occupancy nor takes the
        // drawer over — which left the first voice's room drawn for good
        // when the install released the second's instead.
        let already = self.pending.contains(&address);
        // STD1 — the lanes its draw would stand at, read before anything is
        // written: a draw past a lane's carrier refuses the listing whole,
        // counted on the court.
        let occupied = match already {
            true => None,
            false => {
                let occupant = match drawer {
                    Some(v) => Some(v),
                    None => self.drawers.get(&address).copied(),
                };
                let Some(o) = self.occupied_by(occupant, &mag) else {
                    self.overflowed += 1;
                    eprintln!("  listing refused: its draw passes a lane's carrier");
                    return;
                };
                Some(o)
            }
        };
        self.stamp_standing(&address);
        self.physics.bump(&mag);
        self.listing_atlas.admit(&inv, &address);
        self.tension.insert(&address, mag);
        self.on_page.insert(&address, ());
        if let (Some(voice), false) = (drawer, already) {
            self.drawers.insert(&address, voice);
        }
        if let Some(o) = occupied {
            self.occupy(&address, &mag, o);
        }
        // A word listed again leaves the cavity it was relocated to.
        self.cavity_book.remove(&address);
        self.pending.insert(&address, residual);
        self.book_gen += 1;
        self.append_step_charted(crate::chronometry::ActKind::List, &address, actor, Some(inv));
    }

    pub(crate) fn pending_take(&mut self, address: &[u8]) -> Option<Element> {
        let elem = self.pending.remove(address)?;
        let mag = elem.magnitude();
        self.vacate(address, &mag);
        self.drawers.remove(address);
        self.physics.relieve(&mag);
        // 78.69: and the pressure its witnesses raised for it, exactly.
        if let Some(raised) = self.witnessed.remove(address) {
            self.physics.relieve(&raised);
        }
        Some(elem)
    }

    /// True closure of the whole listing: the thought glued. Leaves the
    /// page and the listing atlas (the word, if any, lives on the language).
    pub(crate) fn unlist(&mut self, address: &[u8], as_cavity: bool) {
        if as_cavity {
            self.relocate_cavity(address);
            return;
        }
        if let Some(elem) = self.pending_take(address) {
            let _ = elem;
        }
        if let Some(listing) = self.field.remove(address) {
            for e in &listing.entries {
                if !e.state.is_flat() {
                    self.listing_atlas.retract(&e.chart, address);
                    // Off the chart it was open on.
                    let ck = e.chart.key();
                    let emptied = match self.chart_listings.get_mut(&ck) {
                        Some(on) => {
                            on.remove(address);
                            on.is_empty()
                        }
                        None => false,
                    };
                    if emptied {
                        self.chart_listings.remove(&ck);
                    }
                }
            }
            self.field_cold.insert(address, listing);
        }
        // Off the docket and the cover its open charts stood on.
        self.open.remove(address);
        self.own_cover.retract_hypha(address);
        self.own_cover_gen = self.own_cover_gen.wrapping_add(1);
        self.listing_atlas.remove(address);
        self.tension.remove(address);
        self.forget_spoken(address);
        self.quote_frontier.remove(address);
        self.on_page.remove(address);
        self.flat_orbits.remove(address);
        self.book_gen += 1;
        // An unlisted word no other row holds leaves its admitting stamp
        // (78.24); an act that cites it after reads the citation first.
        self.forget_if_unheld(address);
    }

    /// Relocate a listing off the quote page onto the cavity atlas. The
    /// element, field, and charts stay; pressure re-measures without it.
    pub(crate) fn relocate_cavity(&mut self, address: &[u8]) {
        let Some(elem) = self.pending_take(address) else {
            return;
        };
        self.on_page.remove(address);
        self.quote_frontier.remove(address);
        self.forget_spoken(address);
        if let Some(listing) = self.field.get(address) {
            for e in &listing.entries {
                if !e.state.is_flat() {
                    self.listing_atlas.admit_also(&e.chart, address);
                }
            }
        }
        // On the chart its listing was admitted on, read off the filed
        // citation: a relocation charts nothing under the court (mycelium#14).
        let inv = self.chart_filed(address, &elem);
        self.listing_atlas.admit_also(&inv, address);
        self.cavity_book.insert(address, elem);
        self.chill_field(address);
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

    pub fn pending(&self) -> &crate::network::nerve::Keyed<Element> {
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
    pub fn open_charts(&self, address: &[u8]) -> Vec<crate::transducer::stalk::Invariant> {
        let listing = self
            .pending
            .get(address)
            .or_else(|| self.cavity_book.get(address));
        match listing {
            Some(p) => open_charts_of(address, p, &self.field, &self.listing_atlas),
            None => Vec::new(),
        }
    }

    /// The cavity's words, read off the cavity book in key order.
    pub fn cavities(&self) -> Vec<&Element> {
        self.cavity_book.scoped().map(|(_, e)| e).collect()
    }

    #[cfg(test)]
    pub(crate) fn stagnant_of(&self, address: &[u8]) -> [u64; 4] {
        let Some(listing) = self.field.get(address) else {
            return [0; 4];
        };
        let mut s = [0u64; 4];
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

    /// If the fibre over `address` names exactly one other word a versor
    /// relates, absorb the card into the bundle and pay the absorbed
    /// discoverer the condense yield. Returns the surviving address.
    pub(crate) fn condense_on(
        &mut self,
        address: &[u8],
        budget: &mut crate::physics::Budget,
    ) -> crate::transducer::address::Address {
        let Some(word) = self.word_at(address).cloned() else {
            return crate::transducer::address::Address::from_bytes(address);
        };
        if word.is_sentence() {
            let inv = crate::transducer::stalk::invariant_of(&word);
            let members: Vec<Vec<u8>> = self.stalks.fibre(&inv).iter().filter(|&a| a != address).cloned().collect();
            if let Some(na) = crate::transducer::sentence::from_element_form(&word) {
                let mut survivor = crate::transducer::address::Address::from_bytes(address);
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
                        survivor = self.absorb_pair(survivor, crate::transducer::address::Address::from(sib), budget);
                    }
                }
                return survivor;
            }
        }
        // 78.74 — THE MULTIPLICITY SETTLED BY CONGRUENCE: every held word
        // relating to this one is a rigid copy of it, so of one another —
        // one algorithm. One relates: absorb as before. Several: they are
        // condensed together pair by pair through `absorb_pair`'s own
        // survivor rule (a bundle keeps the fibre), never a first-wins pick,
        // and the copies become one bundle where they stood refused.
        let related: Vec<crate::transducer::address::Address> = if word.is_connection() {
            match crate::transducer::stalk::congruent(&self.stalks, |a| if a == address { None } else { self.word_at(a) }, &word, budget) {
                Some((sib, _)) => vec![sib],
                None => Vec::new(),
            }
        } else {
            // No tether is measured between held words: the words' chains are
            // centred, so a distance between them is the canonical form's
            // (see `condense_stroke`).
            crate::transducer::stalk::congruent_all(&self.stalks, |a| if a == address { None } else { self.word_at(a) }, &word, budget)
                .into_iter()
                .map(|(sib, _)| sib)
                .collect()
        };
        if related.len() > 1 {
            crate::physics::metabolism::step_or_refuse(&mut self.multiplicity_condensed, 1);
        }
        let mut survivor = crate::transducer::address::Address::from_bytes(address);
        for sib in related {
            survivor = self.absorb_pair(survivor, sib, budget);
        }
        survivor
    }

    /// Absorb one of `new` / `sib` into the other: a bundle keeps the
    /// fibre; two cards make the newly admitted word the bundle. Pays the
    /// absorbed discoverer. Returns the surviving address.
    pub(crate) fn absorb_pair(
        &mut self,
        new: crate::transducer::address::Address,
        sib: crate::transducer::address::Address,
        budget: &mut crate::physics::Budget,
    ) -> crate::transducer::address::Address {
        let Some(nw) = self.basis.get(&new) else {
            return new;
        };
        let Some(sw) = self.basis.get(&sib) else {
            return new;
        };
        let new_card = nw.sections().is_empty() && !crate::transducer::sentence::carries_torsion(nw);
        let sib_card = sw.sections().is_empty() && !crate::transducer::sentence::carries_torsion(sw);
        let (base, absorbed) = if new_card && !sib_card { (sib.clone(), new.clone()) } else { (new.clone(), sib.clone()) };
        // A word the basis does not hold has no chain to transition
        // between, and there is nothing to condense.
        let (Some(from), Some(to)) = (self.basis.get(&absorbed), self.basis.get(&base)) else {
            return new;
        };
        let Some(g) = element::transition(from.sub(), to.sub(), budget) else {
            return new;
        };
        let Some(card) = self.basis.get(&absorbed).map(|w| w.element().clone()) else {
            return new;
        };
        let Some(disc) = self.discoverer_of(&absorbed) else {
            return new;
        };
        self.basis_remove(&absorbed);
        // A base the basis does not hold gains nothing.
        let gained = match self.word_mut(&base) {
            Some(w) => w.update(|e| e.attach(&card, &g)),
            None => 0,
        };
        if gained > 0 {
            // The base as it now stands; the card itself where the base
            // left the basis between the attach and this read.
            let after = match self.basis.get(&base) {
                Some(w) => w.element().clone(),
                None => card.clone(),
            };
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
            // 78.74 — a section punctured: the medium reshaped (plastic), on
            // 𝒢 — a section is a hue assignment over the word's own support.
            self.physics.plastic([false, true, false, false]);
            self.torse_at(&base);
        }
        // The absorbed card lives on as the base's fibre, not on a row of
        // its own: its admitting stamp leaves with it (78.24).
        self.forget_if_unheld(&absorbed);
        base
    }

    /// Torse one word: a closed orbit pays the torse coin on the residual
    /// lane immediately, not on the idle cursor.
    #[expect(
        clippy::disallowed_methods,
        reason = "rule 4 — least by (sigma, period) is lexicographic key order over content, so two courts speak the same torse of one word"
    )]
    pub(crate) fn torse_at(&mut self, address: &[u8]) {
        let spanned = match self.word_mut(address) {
            Some(w) => w.update(|e| e.torse()),
            None => return,
        };
        if spanned == 0 {
            return;
        }
        let Some(after) = self.basis.get(address).map(|w| (**w).clone()) else {
            return;
        };
        let lanes = after.lanes();
        self.physics.torse_relief(lanes, spanned as u64);
        // The payload carries a cycle the word actually filed: the one with
        // the least class coordinates, canonical so two nodes torsing one
        // word speak one rewrite. `last()` read whichever cycle the Vec
        // happened to end on, and the fallback invented an empty sigma with
        // the torse span standing in for a period that was never witnessed
        // (A17: a period is believed only when the orbit closes at it, T5).
        // A word carrying no filed cycle has no torse to speak (rule 8).
        let Some((generator, period)) = after
            .torsion()
            .iter()
            .min_by(|a, b| a.generator.key().cmp(&b.generator.key()).then(a.period.cmp(&b.period)))
            .map(|c| (c.generator.clone(), c.period))
        else {
            return;
        };
        let payload = crate::transducer::rewrite::RewritePayload::Torse { generator, period };
        if let Some(disc) = self.discoverer_of(address) {
            let _ = self.mint_rewrite(
                crate::transducer::rewrite::RewriteKind::Torse,
                std::slice::from_ref(&after),
                &after,
                payload,
                disc,
            );
        }
        self.touch_at(address);
    }

    /// Redundancy probe on the admitted word's own stratum: a plain card
    /// the rest now derives retires, and the voice whose word derived it
    /// earns the refine yield.
    /// 27, S9.8 — WHAT THIS INSTALL MAKES REDUNDANT. A word leaves the
    /// language when the basis DERIVES it, and the candidates are the
    /// forms that cite what this one cites: a keyed read at each address
    /// `new` names, never a whole-carrier read.
    ///
    /// THE STRATUM IS NOT THE SCOPE. This read `stalks.fibre(&inv)` — every
    /// member of the stratum — which was invisibly cheap only because the
    /// chart was the object serialised and every form was a stratum of one.
    /// With a chart that is the rank and the orbit classes (A21 §2), a tree
    /// form shares its stratum with every other tree form, and a scope of
    /// "the stratum" is the whole basis against every install: the
    /// cartesian product law 3 and rule 1 refuse. A stratum ROUTES and does
    /// not decide (21 step 3); `cited_by` is the coordinate that narrows.
    pub(crate) fn refine_stratum(&mut self, new: &[u8]) {
        let Some(after) = self.word_at(new).cloned() else {
            return;
        };
        let Some(bidder) = self.discoverer_of(new) else {
            return;
        };
        // WHO THIS WORD CITES IS READ OFF ITS ENVELOPE (54.5 Road (d), B4).
        // It decoded the whole form to read its vertices; a citation is a
        // 1-simplex of the nerve and a vertex's generators are its
        // presentation, and both ride `Element.receipts`.
        let cites = after.receipts().cited_addresses();
        // The candidates, by keyed read: every form that cites an address
        // this one cites. Two forms over one bundle are where a
        // redundancy can be, and nowhere else is read.
        let mut candidates: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        for cited in &cites {
            let Some(bucket) = self.cited_by.get(cited.as_bytes()) else {
                continue;
            };
            for (citer, ()) in bucket.scoped() {
                if citer.as_slice() != new {
                    candidates.insert(&citer, ());
                }
            }
        }
        let addrs: Vec<Vec<u8>> = candidates.scoped().map(|(a, _)| a).collect();
        for address in addrs {
            let Some(held) = self.basis.get(&address) else {
                continue;
            };
            if !held.sections().is_empty() || crate::transducer::sentence::carries_torsion(held) {
                continue;
            }
            if self.provisional.contains(&address) {
                continue;
            }
            let candidate = (**held).clone();
            if !self.basis_derives(&candidate, &address) {
                continue;
            }
            // G6: IT MINTS A RETIREMENT. This minted `Refine { chart, axis:
            // address[0] }` — the first byte of an A21 address is the tag
            // naming the object KIND, so every such rewrite claimed one
            // axis and chapter 35's "no axis is a hash fragment" fell to a
            // fragment of an address. G6 replaced that shape at
            // `try_refine` on 2026-09-07 and this second site kept it,
            // invisible because the stratum was always empty. No chart is
            // being refined here; a word is being retired.
            let freed = candidate.magnitude();
            let payload = crate::transducer::rewrite::RewritePayload::Retire {
                cites: cites.clone(),
                freed,
            };
            let rewrite = self.mint_rewrite(
                crate::transducer::rewrite::RewriteKind::Retire,
                std::slice::from_ref(&candidate),
                &candidate,
                payload,
                bidder,
            );
            // 78.24 (2c): the word is cited as it stood, read before it
            // moves.
            let retired = self.cite_of(&address);
            // A RETIREMENT KEEPS THE ADDRESS (27): the word leaves the
            // language and stays resolvable for replay, so a citation of it
            // still reads. `basis_remove` alone is not retirement in any
            // reading — it is forgetting.
            self.relocate_cavity(&address);
            self.physics.syntropy(&candidate.magnitude());
            // EC1, as ruled: this retirement is the court's own ruling, and
            // the court authors it.
            let court = self.identity;
            self.seal_retirement(&address, retired, rewrite, &cites, court);
        }
    }

    /// S9.8 — THE BASIS DERIVES THIS WORD, PROVED BY REPLAY. A12's witness
    /// is the vertex's own derivation: the generators it cites, composed
    /// from what this court HOLDS, compared to the rotor the vertex claims
    /// — projectively, because a rotor and its negation are one rotation.
    ///
    /// `reduce_excluding` was the probe, and on a letter it is VACUOUS: it
    /// opens with `g.substance()`, which empties the description tape, and
    /// an adjoined generator carries its rotor in the derivation's rotor
    /// field on that tape (37.7). The substance is then the identity, and
    /// eleven lines in, the identity answers `Decomposed(Vec::new())` — the
    /// empty basis derives it. A word derived by nothing is the identity,
    /// and the identity needs no generator to retire it.
    ///
    /// A rotor outside the span of the seventeen is composed by no product
    /// of them, so nothing here special-cases a letter: it is simply not
    /// derived, and it stays.
    fn basis_derives(&self, candidate: &crate::transducer::element::Element, exclude: &[u8]) -> bool {
        let Some(f) = crate::transducer::sentence::from_element_form(candidate) else {
            return false;
        };
        // THE CITATIONS FROM THE ENVELOPE, THE ACTION FROM THE FORM (54.5
        // Road (d), B3). The two halves of the question are deliberately in
        // two carriers: the rotor is the vertex's key and the citations are
        // its presentation, which the key excludes (`form_vertex_key_ignores_citations_and_certificates`).
        let env = candidate.receipts();
        let mut any = false;
        for (_, v) in f.vertices.scoped() {
            let crate::transducer::sentence::Vertex::Word { address, derivation, .. } = v else {
                continue;
            };
            any = true;
            // A vertex that cites nothing is derived by nothing.
            let Some(cite) = env.cite_for(address.as_bytes()) else {
                return false;
            };
            if cite.generators.is_empty() {
                return false;
            }
            let mut composed = crate::transducer::alphabet::Rotor::identity();
            for (form, vertex, e) in &cite.generators {
                if form.as_bytes() == exclude {
                    return false;
                }
                let Some(g) = self.generator_rotor(form, vertex) else {
                    return false;
                };
                let Some(p) = crate::transducer::alphabet::rotor_power(&g, *e) else {
                    return false;
                };
                composed = composed.geo(&p);
            }
            if !composed.projective_eq(&derivation.rotor()) {
                return false;
            }
        }
        any
    }

    /// The rotor a cited vertex carries, read off the form this court
    /// holds at that address. `None` where the court does not hold it,
    /// which is a citation it cannot replay and so cannot call derived.
    fn generator_rotor(
        &self,
        form: &crate::transducer::address::Address,
        vertex: &crate::transducer::address::Address,
    ) -> Option<crate::transducer::alphabet::Rotor> {
        let f = self.held_word(form.as_bytes()).and_then(|e| e.form().cloned())?;
        crate::transducer::sentence::rotor_at_vertex(&f, vertex.as_bytes())
    }

    /// The word at an address, read by its coordinate. Restored keys are
    /// the coordinates that were written; an older file is refused.
    pub fn word_at(&self, address: &[u8]) -> Option<&Element> {
        self.basis
            .get(address)
            .map(|w| w.element())
            .or_else(|| self.cavity_book.get(address))
            .or_else(|| self.provisional.get(address))
    }

    /// THE HELD WORD THIS COURT INSTALLED (L16): the element AND the form
    /// its tape decoded to, for a reader that needs the form. The basis
    /// alone — a cavity and a provisional are elements this court has not
    /// installed, and `word_at` falls through to them on purpose.
    pub(crate) fn held_word(&self, address: &[u8]) -> Option<&crate::transducer::element::Held> {
        self.basis.get(address).map(|w| w.as_ref())
    }

    /// The word this court INSTALLED, never a provisional one. `word_at`
    /// resolves for replay and falls through to `provisional` and the
    /// cavity on purpose; the teaching path must not, because a taught word
    /// is crystallized on the receiver (24.1). Chapter 23.2 states the same
    /// rule for `resolve_form`: provisional and dialect return `None`.
    pub(crate) fn installed_word(&self, address: &[u8]) -> Option<&Element> {
        self.basis.get(address).map(|w| w.element())
    }

    /// A word the court finalized here joins the ruled cover (keyed admit).
    ///
    /// This does NOT write the court's own Čech section, though every one
    /// of its callers rules a word. `glue_fetched`'s `Admitted` arm calls it
    /// for a word a PEER sent, and sectioning that at this court's own
    /// vantage would file the peer's tape as this court's re-derivation —
    /// the same byte exactness one level down (1.7). The three sites where
    /// the word is this court's own call `section_own` by name.
    ///
    /// The ruling is inside an act, and the re-hearing it wakes draws on that
    /// act's stroke (65.5), threaded here.
    pub(crate) fn ruled_admit(&mut self, address: &[u8], budget: &mut crate::physics::Budget) {
        // WHETHER THIS ADDRESS ACTUALLY ENTERED THE RULED COVER. The
        // re-hearing below was unconditional, and the insert is not: a
        // court asked to rule an address whose word it does not hold does
        // nothing here and then woke every sentence waiting on that
        // citation, which found it still unruled and waited again. The
        // backlog is that loop — `court speech awaiting` 98 → 160 → 206
        // over twenty minutes on ca485cc, every line reading "cited … is
        // not ruled here".
        //
        // A ruling that did not happen is not a ruling, and nothing is
        // reheard on it.
        let mut ruled = false;
        if let Some(w) = self.word_at(address) {
            let inv = crate::transducer::stalk::invariant_of(w);
            let words = crate::transducer::sentence::acting_of_element(w);
            // 1.8: a ruled form's word vertices are the generators this
            // court can compose a spoken path through (50.2 step 2). Read
            // before the carriers are touched, so the word's borrow ends
            // here and not across them. `word_at` reaches the cavity and
            // the provisional as well as the basis, so the decode is named
            // here rather than read off a held word that may not exist yet:
            // `ruled_admit` runs BEFORE the word is installed on some paths
            // (L16).
            let form = crate::transducer::sentence::from_element_form(w);
            self.ruled.admit(&inv, address);
            // A WORD IS FILED WHERE IT ACTS. This filed at `inv`, the
            // SENTENCE's own chart — `chart_of_form`, leading `2`, a
            // projection of what the form holds — while `candidates_for`
            // asks under the charts an orbit's face vertices act on, which
            // are body charts leading `1`. No prefix bridges the two, and
            // `on_axis_of` peels rather than following a fibre's prefix, so
            // everything this court filed was unreachable by the one
            // reader of the index. `admit_words_acting` is the same
            // admission the voice makes (L8r); a connection carries no
            // form and files under its own invariant, which for it is the
            // rotor's own axes.
            match &form {
                Some(f) => {
                    crate::transducer::sentence::admit_words_acting(&mut self.axis_index, f)
                }
                None => {
                    crate::transducer::sentence::admit_words(&mut self.axis_index, &inv, &words)
                }
            }
            if let Some(f) = form {
                let a = crate::transducer::address::Address::from(address.to_vec());
                let mut ix = std::mem::take(&mut self.generators);
                crate::transducer::sentence::admit_generators(&mut ix, &a, &f);
                self.generators = ix;
                // 78.160 P1 — and under the holes its motions wind about.
                self.presentation.file_form(&a, &f, budget);
            }
            self.ruled_addrs.insert(address, ());
            self.refresh_ruled_root();
            ruled = true;
        }
        // 21 step 3: this address is ruled now, so every sentence that
        // waited on it as a citation is heard again. Outside the
        // `word_at` borrow, and after the cover carries it — a re-hearing
        // that ran first would find it still unruled and wait again — and
        // only where the cover actually carries it.
        if ruled {
            // 78.68 — the word's circuits join its chart's holonomy algebra;
            // a new dimension is one RankStep act.
            self.step_rank(address, budget);
            // 78.73 — PLASTIC: the language took a generator on the lanes
            // the word occupies.
            if let Some(lanes) = self.word_at(address).map(|w| w.lanes()) {
                self.physics.plastic(lanes);
            }
            // 78.71, 78.73 — and its field against this court's tubes and
            // orbits.
            self.step_flux(address, budget);
            self.rehear_awaiting(address, budget);
        }
    }

    /// A word this court reached on its OWN tapes and ruled: it joins the
    /// ruled cover and becomes this court's section on that chart, at its
    /// own vantage (13.9, 13.10). The stroke's own ruling, the home's
    /// install, and an upheld verdict are the three; a peer's glued word is
    /// not one of them.
    pub(crate) fn rule_own(&mut self, address: &[u8], budget: &mut crate::physics::Budget) {
        self.ruled_admit(address, budget);
        let Some(w) = self.word_at(address) else { return };
        let inv = crate::transducer::stalk::invariant_of(w);
        self.section_own(&inv, &crate::transducer::address::Address::from(address.to_vec()));
    }

    pub(crate) fn ruled_contains(&self, address: &[u8]) -> bool {
        self.ruled_addrs.contains(address)
    }

    /// A crystallized form this hypha has ruled, or retired to the cavity,
    /// read as a `Form` for the court's replay of a citer. Provisional and
    /// heard words resolve to nothing: a citation stands only on a ruling.
    /// Read through the court's one reader of its held words
    /// ([`crate::transducer::family::Crystal::by_address`]), so the citation
    /// check and the speech that follows it can never disagree.
    pub fn resolve_form(&self, address: &[u8]) -> Option<crate::transducer::sentence::Form> {
        crate::transducer::family::Crystal::by_address(self, address).map(|(_, f)| f.into_owned())
    }

    /// 78.153 — [`Court::resolve_form`] BORROWED where the court holds the
    /// form decoded (an installed word): the replay reads a cited form and
    /// keeps nothing, so it clones nothing. A cavity word is still decoded
    /// from its address at the read.
    pub fn resolve_form_ref(&self, address: &[u8]) -> Option<std::borrow::Cow<'_, crate::transducer::sentence::Form>> {
        crate::transducer::family::Crystal::by_address(self, address).map(|(_, f)| f)
    }

    /// THE DOCKET (Step 5): every provisional word, exported for the court's
    /// blind replay + confluence audit — (element, attesting voices, escrowed μ⃗).
    pub fn dockets(&self) -> Vec<Docketed> {
        self.dockets_since(0)
    }

    /// The provisional words installed past a court's "since" k-step.
    pub fn dockets_since(&self, since_k: u64) -> Vec<Docketed> {
        self.provisional
            .scoped()
            .filter_map(|(address, spoken)| {
                // Where the provisional stands is the act that admitted it
                // (78.24); one admitted AT the court's "since" is served, as
                // the act it names is the one the court heard last.
                let k = self.admitted_at(&address)?.k;
                if k < since_k && since_k > 0 {
                    return None;
                }
                let held = self.treasury.escrow_of(&address)?;
                // A word whose deflation is unmeasured is not served: the
                // court rules what was measured, never a fabricated zero.
                let deflation = self.deflation_of(&address)?;
                if deflation.iter().all(|&x| x == 0) {
                    return None;
                }
                Some(Docketed {
                    addr: Address::from(address),
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
