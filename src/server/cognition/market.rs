// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 26 — ESCROW AND SETTLEMENT: what a compression pays each speaker it
//! cites, and the receipts each act writes. What a voice may draw against
//! its bond is `room.rs`.

use super::*;

impl Court {
    pub fn is_home_listing(&self, address: &[u8], closest_is_self: bool) -> bool {
        let _ = address;
        closest_is_self
    }

    /// Bind this mind to the key that signs for it -- both the identity it
    /// draws room against and the ledger that seals its receipts. The key
    /// is a coordinate of every stamp and in every hash (78.24), so a
    /// ledger that has sealed under another key keeps it, and says so:
    /// binding comes before a court's first act.
    pub fn bind_identity(&mut self, key: [u8; 32]) {
        self.identity = key;
        let was = self.chain.signer();
        if self.chain.bind_signer(key) {
            // LD3-a: rows written before the first act stand at the line's
            // pre-genesis origin, which is keyed by the line's key; a line
            // rebound before it sealed anything moves its origin, and the
            // rows standing there move with it — so every row stands on
            // this court's own line.
            if was != key {
                let origin = |signer| crate::chronometry::Stamp::new(0, signer, crate::chronometry::Place::GENESIS);
                self.treasury.rebind_origin(origin(was), origin(key));
            }
        } else {
            eprintln!(
                "  court ledger keeps its key: it sealed under {:02x}{:02x}… before {:02x}{:02x}… was bound",
                self.chain.signer()[0],
                self.chain.signer()[1],
                key[0],
                key[1]
            );
        }
    }

    /// The SELECTION-FIBRE critical pairs awaiting the court's anchor
    /// refinement (7c) — (word address, Q_anchor, scene). Draining is the
    /// court's move; the queue is bounded and in-memory.
    pub fn anchor_mismatch_dockets(&self) -> &[crate::transducer::anchor::AddressedEvidence] {
        &self.anchor_mismatches
    }

    /// The selection fibre's birth evidence (word address, footprint, scene) —
    /// what the court's refinement derives π over.
    pub fn anchor_evidence(&self) -> &[crate::transducer::anchor::AddressedEvidence] {
        &self.anchor_evidence
    }

    /// The chronometric anchor, public (settlement callers outside the
    /// court — the registry's solve settlements — bind to the same chain).
    pub fn anchor_pub(&self) -> crate::economy::Anchor {
        self.anchor()
    }

    /// SETTLE a verified SOLVE (POUW: the registry confirmed the prediction
    /// against held answers). Pays the registered task tension, pool-funded
    /// and law-bounded; the anti-replay collapse happened upstream —
    /// this pays whatever the registry ruled (bounty or floor).
    pub fn settle_solve(
        &mut self,
        voice: [u8; 32],
        mu: [u64; 4],
        anchor: crate::economy::Anchor,
    ) -> [u64; 4] {
        self.treasury.settle_solve(voice, mu, anchor)
    }

    /// EC1 — A RETIREMENT IS A RULING, sealed as `Ruling(Retired)` citing
    /// what a court admitting its receipt needs to replay it (78.33, as
    /// ruled): the retired word as it stood, then the rewrite it rests on
    /// and every word that speaks for it, each by its fibre and admitting
    /// stamp. A cite this court cannot write — a word no act admitted — is
    /// not invented. A word left on no row takes its stamp with it.
    pub(crate) fn seal_retirement(
        &mut self,
        address: &[u8],
        retired: Option<crate::chronometry::Cite>,
        rewrite: Option<Address>,
        speakers: &[Address],
        author: [u8; 32],
    ) {
        let mut cites: Vec<crate::chronometry::Cite> = retired.into_iter().collect();
        cites.extend(rewrite.iter().chain(speakers).filter_map(|a| self.cite_of(a.as_bytes())));
        let mu = self.frame_of(address);
        self.append_cited(crate::chronometry::ActKind::Ruling(crate::chronometry::Ruled::Retired), cites, mu, author);
        self.forget_if_unheld(address);
    }

    /// GENOME SYNCHRONIZATION, commit step (ruled: servers hold no
    /// dialects — one universal genome on the server plane, authored only
    /// by the court's finality). The caller (the deliberation worker) has
    /// already verified the blind replay and derivability OFF the lock on a
    /// basis snapshot; this brief step re-checks presence and installs. The
    /// ratifying voice is the discoverer of record — real provenance, no
    /// sentinel identity. On the stroke the adoption was replayed under
    /// (65.5): the deliberation worker's `adopt` is the act's root.
    pub fn install_adopted(&mut self, word: &Element, ratifier: [u8; 32], budget: &mut crate::physics::Budget) -> Adoption {
        // The orbit check at the mint: an arriving word's torsion cycles are
        // run round before they enter the genome. Only a witnessed closed orbit
        // at the spoken period is believed.
        for (stored, witness) in crate::transducer::torsion_witness::witness_stored(word) {
            match witness {
                crate::transducer::torsion_witness::HolonomicOrbitWitness::Closed { period, .. } if period == stored => {}
                crate::transducer::torsion_witness::HolonomicOrbitWitness::Closed { .. } => {
                    return Adoption::Refused(crate::verdict::REFUSE_TORSION_PERIOD)
                }
                crate::transducer::torsion_witness::HolonomicOrbitWitness::Open { .. } => {
                    return Adoption::Refused(crate::verdict::REFUSE_TORSION_UNCLOSED)
                }
            }
        }
        let address = word.address();
        if self.basis.contains(&address) {
            // The address is held: the arriving placement's fibre merges into
            // the bundle (sections, connection) — the genome converges in
            // volume, not only in vocabulary.
            let added = match self.word_mut(&address) {
                Some(w) => w.update(|e| e.merge_fibre(word)),
                None => 0,
            };
            // 78.74 — each section punctured is the medium reshaped on 𝒢.
            for _ in 0..added {
                self.physics.plastic([false, true, false, false]);
            }
            self.touch_at(&address);
            // Presence is not finality: an unruled connection stays on the
            // docket until the court measures it.
            if (word.is_connection() || word.is_sentence() || word.is_rewrite()) && !self.ruled_contains(&address) {
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
        self.basis_push(word.clone(), ratifier, budget);
        // ACT1-hold (78.27 Q1): the adoption is an act on this court's own
        // line by the ratifier, and it admits the word it holds.
        self.append_step(crate::chronometry::ActKind::Adopted, &address, ratifier);
        // A word the court itself teaches is finalized language: it joins
        // the ruled cover on arrival.
        if self.court_voice == Some(ratifier) {
            self.ruled_admit(&address, budget);
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
    /// fibre deepens on what the hypha verified. A deepened basis moves the
    /// basis generation so the language snapshot refreshes.
    pub fn puncture_from(&mut self, cites: &[crate::anastome::dialogue::CiteWire]) -> usize {
        let deepened = crate::transducer::sentence::puncture_from(&mut self.basis, cites);
        if deepened > 0 {
            self.basis_gen += 1;
        }
        // 78.74 — each fibre deepened is the medium reshaped on 𝒢 (plastic).
        for _ in 0..deepened {
            self.physics.plastic([false, true, false, false]);
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
        // Settlements are receipts: they stay until the court hears them,
        // never drop. Each stands at the act it follows on this court's own
        // line (78.27 (6)); a settlement before this court's first act has
        // no act to stand at and is refused by name, and counted.
        let Some(at) = self.chain.head_stamp() else {
            self.settlements_unplaced += 1;
            let named = match listing.as_bytes() {
                [a, b, ..] => format!("{a:02x}{b:02x}…"),
                _ => "an address too short to name".to_string(),
            };
            eprintln!("  court settlement refused on {named}: this court has sealed no act to stand it at");
            return;
        };
        self.settlements.push(crate::anastome::dialogue::Settlement {
            listing,
            chain,
            voice,
            cites: cites.to_vec(),
            before,
            after,
            at,
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
            .is_some_and(|s| s.at.k < floor && s.at.k + 1 < self.k_step())
        {
            let old = self.settlements.remove(0);
            self.settlements_cold.push(old);
        }
    }

    /// The settlements past a court's "since": the relation is in line
    /// order, so the walk is over the delta only, from the newest back.
    pub fn settlements_since(&self, since: crate::chronometry::Place) -> &[crate::anastome::dialogue::Settlement] {
        let mut start = self.settlements.len();
        while start > 0 && self.settlements[start - 1].at.place() > since {
            start -= 1;
        }
        &self.settlements[start..]
    }

    /// The court stands at `since`: everything at or before it is heard
    /// and leaves the docket.
    pub fn settlements_heard_to(&mut self, since: crate::chronometry::Place) {
        let mut n = 0usize;
        while n < self.settlements.len() && self.settlements[n].at.place() <= since {
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
        for (address, _) in self.pending.scoped() {
            // A pressure that omits a listing is not the book's pressure. A
            // listing whose extent cannot be read leaves the whole lane
            // unmeasured, so the standing reading holds and the generation
            // is not advanced — the next measurement re-reads it.
            let Some(t) = self.t_extent(&address, now) else {
                return;
            };
            // STD1 — and a lane whose extent passes its carrier is unmeasured
            // the same way, counted.
            let Some(next) = p[2].checked_add(t) else {
                self.overflowed += 1;
                return;
            };
            p[2] = next;
        }
        self.physics.pressure = p;
        self.pressure_gen = self.book_gen;
    }

    /// The deflation a provisional word's install measured on the book.
    pub fn deflation_of(&self, address: &[u8]) -> Option<[u64; 4]> {
        self.deflations.get(address).copied()
    }

    /// Measure and record what installing `address` settles on the book: the
    /// magnitude of the listed demand at that address, or nothing.
    pub(super) fn record_deflation(&mut self, word: Address, listing: Address, own: [u64; 4]) {
        // The demand a percept settles is the listing of its whole chain;
        // the word installed may be the residual after citations, so the
        // deflation is read on the listing, then on the word itself. A germ
        // consilienced at this home from routed demands whose listing never
        // stood here (or was evicted) deflated at least its own measure —
        // never a fabricated zero for the court to slash.
        let on = |address: &Address| self.book_element(address.as_bytes()).map(|p| p.magnitude());
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
        self.intake_prepared(prepare_intake(chain, frame))
    }

    /// THE COMMIT HALF OF THE INTAKE: pieces already read — each residual
    /// and its chart — listed under the court. The reading is
    /// `prepare_intake`'s and takes no lock: hypha b, ten minutes after
    /// boot, held its court in `intake → list → invariant_of` in seven
    /// samples of ten while fold, quote and syllabus judges waited behind
    /// it, 0.2–9 s each (gdb and the wire, 2026-09-30).
    pub fn intake_prepared(&mut self, pieces: Vec<(Element, crate::transducer::stalk::Invariant)>) -> bool {
        let mut listed = false;
        for (residual, chart) in pieces {
            let address = residual.address();
            if self.basis.contains(&address) || self.pending.contains(&address) {
                continue; // already speakable or already listed
            }
            self.stamp_standing(address);
            self.book_list_charted(residual, chart);
            listed = true;
        }
        listed
    }
}

/// THE READING HALF OF THE INTAKE, with no court: the chain's pieces, each
/// as its residual and its chart. A piece that cannot be addressed is a
/// refusal that travels: never listed and never counted (U14).
pub fn prepare_intake(
    chain: &[(i64, i64, u8, u8)],
    frame: &crate::arc::Frame,
) -> Vec<(Element, crate::transducer::stalk::Invariant)> {
    if chain.is_empty() {
        return Vec::new();
    }
    crate::transducer::sentence::pieces(chain)
        .into_iter()
        .filter_map(|body| element::from_residual_framed(&body, frame))
        .map(|residual| {
            let chart = crate::transducer::stalk::invariant_of(&residual);
            (residual, chart)
        })
        .collect()
}

impl Court {

    /// DEPOSIT evidence from the wire (an entity's verified dialect finding):
    /// local thought seeds the court's global refinement — what tasks
    /// repeatedly confirm locally is what the mesh promotes. Bounded, deduped.
    pub fn deposit_evidence(
        &mut self,
        address: Address,
        fp: Vec<(i64, i64)>,
        scene: Vec<(i64, i64, u8)>,
    ) -> bool {
        if fp.is_empty() {
            return false;
        }
        let key = footprint_key(&address, &fp, &scene);
        if self.evidence_at.contains(&key) {
            return false;
        }
        self.evidence_at.insert(key, ());
        self.anchor_evidence.push(crate::transducer::anchor::EvidenceOf {
            addr: address,
            footprint: fp,
            scene,
        });
        true
    }


    /// ENROLL an identity with a per-lane bond. The hypha enrolls itself
    /// from genesis or from a restored stake; arriving voices stay unbonded
    /// until they bond from balance. Returns the vector added this call.
    pub fn enroll(&mut self, party: [u8; 32], bond: [u64; 4]) -> [u64; 4] {
        let anchor = self.anchor();
        // A stake moves value from a balance and never creates any (§4), so
        // an identity bonded at genesis is FUNDED first and stakes from that
        // -- MU-PRODUCT §4: a genesis allocation funds the operator's
        // customer voice and bonds the operator's hyphae. The allocation is its own ledger kind, so
        // §6 keeps seeded value and earned value separable; what is already
        // held is never re-allocated.
        let held = self.treasury.balance_mu(&party);
        // What genesis must seed is the shortfall of what is held against
        // the bond: nothing on a lane already at or above it.
        let short = crate::economy::shortfall(&bond, &held);
        if short.iter().any(|&x| x > 0) {
            self.treasury.genesis(party, short, anchor);
        }
        self.treasury.enroll(party, bond, anchor)
    }

    /// E3 — A GENESIS ALLOCATION TO SPEND: `mu` onto `party`'s balance, its
    /// own ledger kind (§6 keeps seeded value separable from earned), at
    /// this court's head. Answers what landed.
    pub fn fund(&mut self, party: [u8; 32], mu: [u64; 4]) -> [u64; 4] {
        let anchor = self.anchor();
        self.treasury.genesis(party, mu, anchor)
    }

    /// EC1 — THIS COURT'S ECONOMY: its ledger by arm, by kind, each
    /// identity's standing, and the tuition paid to this court.
    pub fn economy(&self) -> Option<crate::observe::Economy> {
        self.treasury.economy(&self.identity)
    }

    /// A genesis allocation, once per identity, written as GENESIS — never a
    /// mint (§6: seeded value and earned value stay separable; it was booked
    /// as a subsidy mint). A home whose allocation was booked as a mint before
    /// this keeps it, and is not allocated twice.
    pub fn allocate(&mut self, party: [u8; 32], mu: [u64; 4]) -> [u64; 4] {
        if self.treasury.has_genesis(&party) || self.treasury.has_mint(&party) {
            return [0; 4];
        }
        let anchor = self.anchor();
        self.treasury.genesis(party, mu, anchor)
    }
}
