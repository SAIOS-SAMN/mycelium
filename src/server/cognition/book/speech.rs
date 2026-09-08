// 26 — SPEECH AGAINST THE BOOK: a voice standing sentences, a probe
// settling against an open listing, and a rewrite offered for one.

use crate::cognition::*;
use crate::transducer::a21::Address;
use crate::transducer::element::Element;

impl Kernel {
    /// List a sentence on the docket if it is not already provisional.
    pub(crate) fn docket_sentence(&mut self, word: &Element, ratifier: [u8; 32]) {
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
    pub(crate) fn docket_connection(&mut self, word: &Element, ratifier: [u8; 32]) {
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
        // THIS COURT re-derived the word to docket it: its own vantage,
        // recorded so a witness's re-derivation can converge with it (47.3).
        // Every path that dockets records one, or the object waits for a
        // second organ it can never reach because the first was never there.
        self.converge_here(word, ratifier);
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

    /// The court's voice, heard on its verdicts.
    pub fn set_court_voice(&mut self, voice: [u8; 32]) {
        self.court_voice = Some(voice);
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
                    std::slice::from_ref(&sib),
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
        if residual.as_slice().is_empty() {
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

    pub(crate) fn record_flat(
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

}
