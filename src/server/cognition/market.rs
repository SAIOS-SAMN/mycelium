// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 26 — ESCROW AND SETTLEMENT: what a compression pays each speaker it
//! cites, and the receipts each act writes. What a voice may draw against
//! its bond is `room.rs`.

use super::*;

impl Kernel {
    pub fn is_home_listing(&self, addr: &[u8], closest_is_self: bool) -> bool {
        let _ = addr;
        closest_is_self
    }

    /// Bind this mind to the key that signs for it -- both the identity it
    /// draws room against and the ledger that seals its receipts. The key
    /// is a coordinate of every stamp and in every hash (78.24), so a
    /// ledger that has sealed under another key keeps it, and says so:
    /// binding comes before a court's first act.
    pub fn bind_identity(&mut self, vk: [u8; 32]) {
        self.identity = vk;
        if !self.chain.bind_vk(vk) {
            eprintln!(
                "  court ledger keeps its key: it sealed under {:02x}{:02x}… before {:02x}{:02x}… was bound",
                self.chain.vk()[0],
                self.chain.vk()[1],
                vk[0],
                vk[1]
            );
        }
    }

    /// The SELECTION-FIBER critical pairs awaiting the court's anchor
    /// refinement (7c) — (word address, Q_anchor, scene). Draining is the
    /// court's move; the queue is bounded and in-memory.
    pub fn anchor_mismatch_dockets(&self) -> &[crate::transducer::anchor::AddressedEvidence] {
        &self.anchor_mismatches
    }

    /// The selection fiber's birth evidence (word address, footprint, scene) —
    /// what the court's refinement derives π over.
    pub fn anchor_evidence(&self) -> &[crate::transducer::anchor::AddressedEvidence] {
        &self.anchor_evidence
    }

    /// The chronometric anchor, public (settlement callers outside the
    /// kernel — the registry's solve settlements — bind to the same chain).
    pub fn anchor_pub(&self) -> crate::economy::Anchor {
        self.anchor()
    }

    /// SETTLE a verified SOLVE (PoUW: the registry confirmed the prediction
    /// against held answers). Pays the registered task tension, pool-funded
    /// and float-law bounded; the anti-replay collapse happened upstream —
    /// this pays whatever the registry ruled (bounty or floor).
    pub fn treasury_settle_solve(
        &mut self,
        voice: [u8; 32],
        mu: [u64; 4],
        anchor: crate::economy::Anchor,
    ) -> [u64; 4] {
        self.treasury.settle_solve(voice, mu, anchor)
    }

    /// THE REFINEMENT BID (0x09 — compression IS learning): a client claims
    /// basis word W is DERIVABLE — the rest of the basis already speaks it.
    /// Verification is the redundancy probe's own deterministic replay
    /// (decompose W's substance with W excluded); no trust, no vote. On
    /// success: W retires (the vocabulary compresses — ontological
    /// optimization, halting geometric bloat), the bidder earns the
    /// COMPRESSION YIELD Δ‖·‖_DM × tension (float-bounded), and the rewrite
    /// seals a K-step. Provisional words are the court's business — refused.
    pub fn try_refine(
        &mut self,
        addr: &[u8],
        bidder: [u8; 32],
        cites: &[crate::anastome::dialogue::CiteWire],
    ) -> Option<[u64; 4]> {
        let candidate = self.basis.get(addr).map(|w| (**w).clone())?;
        if self.provisional.contains(addr) {
            return None;
        }
        // Never a BUNDLE — its sections/cycles carry family volume no other
        // word speaks.
        if !candidate.sections().is_empty() || crate::transducer::sentence::carries_torsion(&candidate) {
            return None;
        }
        // THE REPLAY rules: the bidder's sentence, spoken by other words,
        // must reproduce the candidate's substance exactly. No search here.
        // Each cite resolves or the bid withholds that cite.
        let mut resolved: Vec<Cite> = Vec::new();
        for c in cites {
            // The cite already NAMES the word; what has to be checked is
            // that the word is live and is not the candidate citing itself.
            // It used to be resolved to a slot here, which is the offset
            // round trip the reading no longer does.
            (self.is_live(&c.addr) && c.addr.as_bytes() != addr).then_some(())?;
            resolved.push(Cite {
                word: c.addr.clone(),
                transport: c.transport.clone(),
                gauge: c.gauge as usize,
            });
        }
        if resolved.is_empty() {
            return None;
        }
        let sub = candidate.substance();
        let residual = crate::transducer::sentence::verify_utterance(cites, sub.sub());
        match residual.as_slice() {
            [] => {
                // THE COMPRESSION AXIS. The bidder proved that other words
                // reproduce this word's substance exactly, so the word is
                // redundant and retires: the language shrinks toward the
                // geodesic while the meaning it carried is unchanged (3.8,
                // chapter 27, T10). That is the move, and it is a
                // retirement — not a refinement of a chart. It was minted as
                // `Refine { chart, axis: addr[0] }`: under A21 the first
                // byte of an address is the TAG NAMING THE OBJECT KIND, so
                // every such rewrite claimed one axis, and chapter 35's "no
                // axis is a hash fragment" fell to a fragment of an address
                // standing in for A7's class invariants. Its yield was a
                // constant besides — `yield_from_payload` hardcoded 1 on the
                // shell lane and `yield_vector` carried `.max(1)` so the two
                // would agree. There is no axis here because no chart is
                // being refined.
                Some(self.retire_by_bid(addr, &candidate, bidder, cites.iter().map(|c| c.addr.clone()).collect()))
            }
            _ => None,
        }
    }

    /// EXPANSION 2 — THE ALGEBRAIC REFINEMENT BID, RULED ON THE COURT'S
    /// OWN ALGEBRA. A client claims ruled word W carries nothing the ruled
    /// word H does not: W's operator is H's, or W is a section of H's
    /// bundle by a gauge `γ` the seventeen spell. The spatial bid
    /// (`try_refine`) replays a chain of links; this one cannot be answered
    /// that way, since an operator has no links to pave, so its receipt is
    /// one product and one projective comparison in the exact algebra,
    /// `γ·h·γ⁻¹ = w` or `w = h`, never a search and never trust.
    ///
    /// Both words are this court's ruled language, and W is not a bundle
    /// (its sections and cycles carry volume no other word speaks); each
    /// speaks ONE operator, or the claim is about a word this receipt
    /// cannot read and is refused. On success W retires citing H, exactly
    /// as the spatial bid retires citing its cites (law 12: one retirement).
    pub fn try_refine_algebraic(
        &mut self,
        addr: &[u8],
        bidder: [u8; 32],
        held: &[u8],
        witness: &crate::anastome::dialogue::AlgebraicWitness,
    ) -> Option<[u64; 4]> {
        use crate::anastome::dialogue::AlgebraicWitness;
        if addr == held {
            return None;
        }
        let candidate = self.basis.get(addr).map(|w| (**w).clone())?;
        if self.provisional.contains(addr) || !self.ruled_contains(addr) || !self.ruled_contains(held) {
            return None;
        }
        if !candidate.sections().is_empty() || crate::transducer::sentence::carries_torsion(&candidate) {
            return None;
        }
        let against = self.basis.get(held).map(|w| (**w).clone())?;
        let (w, h) = (single_rotor(&candidate)?, single_rotor(&against)?);
        let proved = match witness {
            AlgebraicWitness::SameOperator => w.projective_eq(&h),
            AlgebraicWitness::Section(gamma) => gamma.rotor().conjugate_holds(&h, &w),
        };
        if !proved {
            return None;
        }
        Some(self.retire_by_bid(addr, &candidate, bidder, vec![Address::from(held)]))
    }

    /// THE RETIREMENT A BID EARNS, one construction for both bids: the
    /// word leaves the language citing what speaks for it, the freed
    /// measure is the compression yield, and the ruling is a step of this
    /// court's clock authored by the bidder.
    fn retire_by_bid(&mut self, addr: &[u8], candidate: &Element, bidder: [u8; 32], cites: Vec<Address>) -> [u64; 4] {
        let saved = candidate.magnitude();
        let payload = crate::transducer::rewrite::RewritePayload::Retire { cites, freed: saved };
        let _ = self.mint_rewrite(
            crate::transducer::rewrite::RewriteKind::Retire,
            std::slice::from_ref(candidate),
            candidate,
            payload,
            bidder,
        );
        // 78.24 (2c): the word is cited as it stood, read before it leaves.
        let cite = self.cite_of(addr);
        self.basis_remove(addr);
        self.physics.syntropy(&saved);
        self.append_after_removal(crate::chronometry::ActKind::Ruling, addr, cite, bidder);
        saved
    }

    /// GENOME SYNCHRONIZATION, commit step (operator-ruled: servers hold no
    /// dialects — one universal genome on the server plane, authored only
    /// by the court's finality). The caller (the deliberation worker) has
    /// already verified the blind replay and derivability OFF the lock on a
    /// basis snapshot; this brief step re-checks presence and installs. The
    /// ratifying voice is the discoverer of record — real provenance, no
    /// sentinel identity.
    pub fn install_adopted(&mut self, word: &Element, ratifier: [u8; 32]) -> Adoption {
        // The orbit check at the mint: an arriving word's torsion cycles are
        // run round before they enter the genome. Only a witnessed closed orbit
        // at the spoken period is believed.
        for (stored, witness) in crate::transducer::torsion_witness::witness_stored(word) {
            match witness {
                crate::transducer::torsion_witness::HolonomicOrbitWitness::Closed { period, .. } if period == stored => {}
                crate::transducer::torsion_witness::HolonomicOrbitWitness::Closed { .. } => {
                    return Adoption::Refused("torsion period does not re-derive — the winding was misspoken")
                }
                crate::transducer::torsion_witness::HolonomicOrbitWitness::Open { .. } => {
                    return Adoption::Refused("torsion cycle does not close — the period was a filing")
                }
            }
        }
        let addr = word.address();
        if self.basis.contains(&addr) {
            // The address is held: the arriving placement's fibre merges into
            // the bundle (sections, connection) — the genome converges in
            // volume, not only in vocabulary.
            if let Some(w) = self.word_mut(&addr) {
                w.update(|e| e.merge_fibre(word));
            }
            self.touch_at(&addr);
            // Presence is not finality: an unruled connection stays on the
            // docket until the court measures it.
            if (word.is_connection() || word.is_sentence() || word.is_rewrite()) && !self.ruled_contains(&addr) {
                self.docket_connection(word, ratifier);
            }
            return Adoption::Known;
        }
        // A CONNECTION arriving from an entity is listed as itself: it enters
        // the docket with its own magnitude as the deflation it claims, so
        // the court rules it (inversion, remainder, torsion) and the mesh
        // pays it as a word, never as a copied pair. A residual word taught
        // up is installed as before. A sentence is the same: Offered on the
        // docket so a second organ's replay installs it as language.
        if word.is_connection() || word.is_sentence() || word.is_rewrite() {
            self.docket_connection(word, ratifier);
        }
        self.basis_push(word.clone(), ratifier);
        // A word the court itself teaches is finalized language: it joins
        // the ruled cover on arrival.
        if self.court_voice == Some(ratifier) {
            self.ruled_admit(&addr);
        }
        self.physics.syntropy(&word.magnitude()); // convergence is order built
        Adoption::Installed
    }

    /// A voice spoke on a listing in its current state.
    pub(super) fn record_spoken(&mut self, listing: crate::transducer::address::Address, voice: [u8; 32]) {
        self.spoken.nested_insert(&listing, voice, ());
        self.spoken_by.nested_insert(voice, &listing, ());
    }

    /// The receipts of speech, keyed by listing: read at a listing's own
    /// address, never read whole.
    pub fn spoken(&self) -> &crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>> {
        &self.spoken
    }

    pub fn quote_frontier(&self) -> &crate::network::nerve::Keyed<()> {
        &self.quote_frontier
    }

    /// PUNCTURE held words from a verified utterance's carried bodies: the
    /// fiber deepens on what the hypha verified. A deepened basis moves the
    /// basis generation so the language snapshot refreshes.
    pub fn puncture_from(&mut self, cites: &[crate::anastome::dialogue::CiteWire]) -> usize {
        let deepened = crate::transducer::sentence::puncture_from(&mut self.basis, cites);
        if deepened > 0 {
            self.basis_gen += 1;
        }
        deepened
    }

    /// RECORD a settlement for the court, bounded by the reserve ceiling
    /// (the oldest leaves when the docket is full and the court has not
    /// pulled).
    pub fn record_settlement(
        &mut self,
        listing: crate::transducer::address::Address,
        chain: Vec<(i64, i64, u8, u8)>,
        voice: [u8; 32],
        cites: &[crate::anastome::dialogue::CiteWire],
        before: [u64; 4],
        after: [u64; 4],
    ) {
        // Settlements are receipts: they stay until the court hears them, never drop.
        let k = self.k_step();
        let tu = self.height();
        self.settlements.push(crate::anastome::dialogue::Settlement {
            listing,
            chain,
            voice,
            cites: cites.to_vec(),
            before,
            after,
            k,
            t_universal: tu,
        });
        // A court whose clock has not yet reached a full reserve of steps
        // has nothing behind the floor to cool.
        let Some(floor) = self.k_step().checked_sub(crate::physics::reserve_max()) else {
            return;
        };
        #[expect(
            clippy::disallowed_methods,
            reason = "rule 15 — FIFO drain head: the settlements are pushed in K order and drained from the oldest"
        )]
        while self
            .settlements
            .first()
            .is_some_and(|s| s.k < floor && s.k + 1 < self.k_step())
        {
            let old = self.settlements.remove(0);
            self.settlements_cold.push(old);
        }
    }

    /// The settlements past a court's "since": the relation is in k-step
    /// order, so the walk is over the delta only, from the newest back.
    pub fn settlements_since(&self, since_k: u64) -> &[crate::anastome::dialogue::Settlement] {
        let mut start = self.settlements.len();
        while start > 0 && self.settlements[start - 1].k > since_k {
            start -= 1;
        }
        &self.settlements[start..]
    }

    /// The court stands at `since_k`: everything at or before it is heard
    /// and leaves the docket.
    pub fn settlements_heard_to(&mut self, since_k: u64) {
        let mut n = 0usize;
        while n < self.settlements.len() && self.settlements[n].k <= since_k {
            n += 1;
        }
        self.settlements_cold.extend(self.settlements.drain(..n));
    }

    /// PRESSURE IS THE OPEN BOOK: the per-lane magnitude of every listing,
    /// measured whenever the book moved — never an accumulator. A demand
    /// the language already speaks adds nothing; a settlement removes
    /// exactly what it settled.
    pub(super) fn remeasure_pressure(&mut self) {
        let now = self.height();
        let mut p = self.pressure_sum;
        p[2] = 0;
        for (addr, _) in self.pending.scoped() {
            // A pressure that omits a listing is not the book's pressure. A
            // listing whose extent cannot be read leaves the whole lane
            // unmeasured, so the standing reading holds and the generation
            // is not advanced — the next measurement re-reads it.
            let Some(t) = self.t_extent(&addr, now) else {
                return;
            };
            p[2] = p[2].saturating_add(t);
        }
        self.physics.pressure = p;
        self.pressure_gen = self.book_gen;
    }

    /// The deflation a provisional word's install measured on the book.
    pub fn deflation_of(&self, addr: &[u8]) -> Option<[u64; 4]> {
        self.deflations.get(addr).copied()
    }

    /// Measure and record what installing `addr` settles on the book: the
    /// magnitude of the listed demand at that address, or nothing.
    pub(super) fn record_deflation(&mut self, word: Address, listing: Address, own: [u64; 4]) {
        // The demand a percept settles is the listing of its whole chain;
        // the word installed may be the residual after citations, so the
        // deflation is read on the listing, then on the word itself. A germ
        // consilienced at this home from routed demands whose listing never
        // stood here (or was evicted) deflated at least its own measure —
        // never a fabricated zero for the court to slash.
        let on = |addr: &Address| self.book_elem(addr.as_bytes()).map(|p| p.magnitude());
        let Some(deflation) = on(&listing).or_else(|| on(&word)).or_else(|| {
            own.iter().any(|&x| x > 0).then_some(own)
        }) else {
            return;
        };
        self.deflations.insert(word, deflation);
    }

    /// The court's chain head as ruled to this hypha.
    pub fn set_court_head(&mut self, head: [u8; 32]) {
        self.court_head = head;
    }

    pub fn court_head(&self) -> [u8; 32] {
        self.court_head
    }

    /// THE CORPUS INTAKE DOOR (server-side ingestion authority): an
    /// un-factored demand from the hypha's OWN registered corpus lists on
    /// the open market under the same thermodynamic discipline as every
    /// spoken demand — no voice spoke it (the corpus is the source), so it
    /// germinates only when client voices independently attest it. The
    /// server decides what is work; the entities do the cognition.
    ///
    /// The corpus draws no room (23.8; I31, S12.5b): room is a voice's,
    /// drawn by its utterance and released by closure, ruling or lapse, and
    /// a corpus task was uttered by no voice. Drawn from the hypha's own
    /// bond, the intake exhausted it in the first minutes of every boot and
    /// nothing homed here could lapse it back; the listing still tensions
    /// the book, and room stays the throttle on what voices utter.
    ///
    /// A task is listed one BODY at a time (20, 24.4 P6; S12.3a): each
    /// connected piece of the residual is its own demand on its own body
    /// chart, so a body two tasks share is one listing on one stratum and a
    /// held word deflates it wherever it recurs. Listed whole, a task's
    /// residual was a chain of several bodies with a chart no other task
    /// shared — a thousand unrelated descriptions (I34). A body already
    /// speakable or already listed is not listed again; the intake answers
    /// whether anything new was.
    pub fn intake(&mut self, chain: &[(i64, i64, u8, u8)], frame: &crate::arc::Frame) -> bool {
        if chain.is_empty() {
            return false;
        }
        let mut listed = false;
        for body in crate::transducer::sentence::pieces(chain) {
            // A piece that cannot be addressed is a refusal that travels:
            // never listed and never counted (U14).
            let Some(residual) = element::from_residual_framed(&body, frame) else {
                continue;
            };
            let addr = residual.address();
            if self.basis.contains(&addr) || self.pending.contains(&addr) {
                continue; // already speakable or already listed
            }
            self.stamp_standing(addr);
            self.book_list_undrawn(residual);
            listed = true;
        }
        listed
    }

    /// DEPOSIT evidence from the wire (an entity's verified dialect finding):
    /// local thought seeds the court's global refinement — what tasks
    /// repeatedly confirm locally is what the mesh promotes. Bounded, deduped.
    pub fn deposit_evidence(
        &mut self,
        addr: Address,
        fp: Vec<(i64, i64)>,
        scene: Vec<(i64, i64, u8)>,
    ) -> bool {
        if fp.is_empty() {
            return false;
        }
        let key = footprint_key(&addr, &fp, &scene);
        if self.evidence_at.contains(&key) {
            return false;
        }
        self.evidence_at.insert(key, ());
        self.anchor_evidence.push(crate::transducer::anchor::EvidenceOf {
            addr,
            footprint: fp,
            scene,
        });
        true
    }

    /// REBIND (Step 7c): the court's family-derived π* upgrades γ = (⊤, Δ) →
    /// (π*, Δ) in place — canonical substance identity untouched (the address
    /// is BASE-only), the dual condition activates for this word, and the
    /// word's spent evidence/mismatch queues clear. Returns false for an
    /// unknown address.
    pub fn apply_rebind(&mut self, addr: &[u8], pi: crate::transducer::anchor::Sel) -> bool {
        let mut bound = false;
        if let Some(held) = self.basis.get(addr).map(|w| w.element().clone()) {
            let rebound = crate::transducer::element::Held::of(held.with_anchor(pi.clone()));
            self.basis.insert(addr, std::sync::Arc::new(rebound));
            self.touch_at(addr);
            bound = true;
        }
        // A germ still awaiting its witness accepts the bind too — it
        // installs already anchored (evidence often reaches family size
        // before the witness re-derivation lands).
        if let Some(elem) = self.pending.get(addr).cloned() {
            self.pending.insert(addr, elem.with_anchor(pi));
            self.book_gen += 1;
            bound = true;
        }
        if bound {
            // Spent critical pairs clear (they argued against the OLD π);
            // EVIDENCE PERSISTS — the family keeps growing so the court can
            // re-refine iteratively: loose early carriers tighten as more
            // witnessed placements accumulate (rebinding is never one-shot).
            self.mismatch_at.remove(addr);
            self.anchor_mismatches.retain(|e| e.addr.as_bytes() != addr);
        }
        bound
    }

    /// ENROLL an identity with a per-lane bond. The hypha enrolls itself
    /// from genesis or from a restored stake; arriving voices stay unbonded
    /// until they bond from balance. Returns the vector added this call.
    pub fn enroll(&mut self, who: [u8; 32], bond: [u64; 4]) -> [u64; 4] {
        let anchor = self.anchor();
        // A stake moves value from a balance and never creates any (§4), so
        // an identity bonded at genesis is FUNDED first and stakes from that
        // -- MU-PRODUCT §4: "a genesis allocation funds our customer voice
        // and bonds our hyphae". The allocation is its own ledger kind, so
        // §6 keeps seeded value and earned value separable; what is already
        // held is never re-allocated.
        let held = self.treasury.balance_mu(&who);
        // What genesis must seed is the shortfall of what is held against
        // the bond: nothing on a lane already at or above it.
        let short = crate::economy::shortfall(&bond, &held);
        if short.iter().any(|&x| x > 0) {
            self.treasury.genesis(who, short, anchor);
        }
        self.treasury.enroll(who, bond, anchor)
    }

    /// Mint a genesis allocation once per identity.
    pub fn allocate(&mut self, who: [u8; 32], mu: [u64; 4]) -> [u64; 4] {
        if self.treasury.has_mint(&who) {
            return [0; 4];
        }
        self.treasury.settle_subsidy(who, mu, self.anchor())
    }
}

/// THE ONE OPERATOR A WORD SPEAKS, read off its tape: a sentence's single
/// non-identity word vertex, by the rotor its derivation composes to (a
/// letter's, an adjoined generator's, a spelled residue's alike — L14), or
/// a connection's letters. `None` for a word that speaks none or several:
/// an algebraic receipt is about one operator against one, and a sentence
/// of several vertices is not redundant because one of them is.
fn single_rotor(e: &Element) -> Option<crate::transducer::alphabet::Rotor> {
    use crate::transducer::sentence::Vertex;
    let rotors: Vec<crate::transducer::alphabet::Rotor> = match crate::transducer::sentence::from_element_form(e) {
        Some(f) => f
            .vertices
            .iter()
            .filter_map(|v| match v {
                Vertex::Word { derivation, .. } => Some(derivation.rotor()),
                _ => None,
            })
            .filter(|r| !r.is_identity())
            .collect(),
        None => crate::transducer::cech::rotor_of(e).into_iter().filter(|r| !r.is_identity()).collect(),
    };
    match rotors.as_slice() {
        [r] => Some(r.clone()),
        _ => None,
    }
}
