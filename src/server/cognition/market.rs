// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! 26 — ROOM, ESCROW AND SETTLEMENT: what a voice may draw against its
//! bond, what a compression pays each speaker it cites, and the receipts
//! each of those writes.

use super::*;

impl Kernel {
    pub fn is_home_listing(&self, addr: &[u8], closest_is_self: bool) -> bool {
        let _ = addr;
        closest_is_self
    }

    /// Room is the uttering voice's remaining bond, per lane. Unbonded is
    /// zero room. No ×(reserve/spend): the genesis bond already covers a pair.
    /// Bind this mind to the key that signs for it -- both the identity it
    /// draws room against and the chain that seals its receipts. Binding
    /// only the first left every receipt claiming `[0u8; 32]` as its
    /// author, so no peer could verify one under the key it claimed.
    pub fn bind_identity(&mut self, vk: [u8; 32]) {
        self.identity = vk;
        self.chain.bind_vk(vk);
    }

    pub fn lane_room(&self) -> [usize; 4] {
        self.lane_room_of(&self.identity)
    }

    pub fn lane_room_of(&self, voice: &[u8; 32]) -> [usize; 4] {
        let occ = self.drawn.get(voice).copied().unwrap_or([0; 4]);
        if let Some(cap) = self.lane_cap {
            return std::array::from_fn(|l| cap[l].saturating_sub(occ[l] as usize));
        }
        let Some(bond) = self.treasury.bond_of(voice) else {
            return [0; 4];
        };
        std::array::from_fn(|l| bond[l].saturating_sub(occ[l]) as usize)
    }

    pub fn occupancy(&self) -> [u64; 4] {
        self.occupancy
    }

    /// Checkout a T-lease for a quoting voice. One lease per voice.
    pub fn grant_t_lease(&mut self, voice: [u8; 32]) {
        if self.t_leases.contains(&voice) {
            return;
        }
        self.t_leases.insert(voice, ());
        self.occupancy[2] = self.occupancy[2].saturating_add(1);
    }

    /// Return a T-lease when the voice's quoted work settles or expires.
    pub fn release_t_lease(&mut self, voice: [u8; 32]) {
        if self.t_leases.remove(&voice).is_some() {
            self.occupancy[2] = self.occupancy[2].saturating_sub(1);
        }
    }

    pub fn reserve(&self) -> [u64; 4] {
        self.physics.reserve
    }

    /// Test helper: remaining listing room, from each lane's own bond.
    pub fn dynamic_lane_capacity(&self) -> [usize; 4] {
        let room = self.lane_room();
        std::array::from_fn(|l| room[l].saturating_add(self.occupancy[l] as usize))
    }

    /// THE SHORTFALL of a draw against the uttering voice's room, per lane:
    /// what the draw needs beyond the room, zero where the lane fits (S9.6,
    /// §2.3). A reading, never a verdict. The bit this replaced threw the
    /// lane away, and its one caller then read the room a second time to
    /// learn which lane had refused — the recorded proof that a `bool` over
    /// four lanes destroys what the caller needs.
    ///
    /// Room is what bond buys (MU-ECONOMY §3.1 as ruled 2026-09-09,
    /// S13.15): a bonded voice draws against its bond less what it has
    /// drawn; an unbonded voice has room for exactly one claim in flight
    /// per lane — a lane it already draws on is short by the whole draw,
    /// a lane it does not is not short at all, since speaking needs no
    /// bond. A lane the voice owes an unpaid recall on (`recall_short`)
    /// is short by the whole draw while its claim in flight on that lane
    /// stands. A test cap (`lane_cap`) reads as a bond.
    pub(super) fn room_shortfall(&self, voice: &[u8; 32], draw: &[u64; 4]) -> [u64; 4] {
        let occ = self.drawn.get(voice).copied().unwrap_or([0; 4]);
        let owed = self.recall_short.get(voice).copied().unwrap_or([0; 4]);
        let bonded = self.lane_cap.is_some() || self.treasury.bond_of(voice).is_some();
        let room = self.lane_room_of(voice);
        std::array::from_fn(|l| match (draw[l], bonded) {
            (0, _) => 0,
            (d, _) if owed[l] > 0 && occ[l] > 0 => d,
            (d, true) => d.saturating_sub(room[l] as u64),
            (d, false) if occ[l] > 0 => d,
            (_, false) => 0,
        })
    }

    /// A recall the voice could not pay (§3.1 as ruled, S13.15): the
    /// shortfall is recorded per lane; it closes the voice's room on each
    /// short lane while a claim of the voice is in flight there, and clears
    /// when that claim closes (`occupy`). Nothing is written for a recall
    /// that was paid in full.
    pub(super) fn record_recall_short(&mut self, voice: [u8; 32], short: &[u64; 4]) {
        if crate::economy::lanes_short(short).is_empty() {
            return;
        }
        let mut owed = self.recall_short.get(&voice).copied().unwrap_or([0; 4]);
        for l in crate::economy::lanes_short(short) {
            owed[l] = owed[l].saturating_add(short[l]);
        }
        self.recall_short.insert(voice, owed);
    }

    /// One line per short lane (34.2), read off the shortfall the caller
    /// holds: room is the named voice's, never the hypha's, and it is not
    /// measured again here.
    pub(super) fn log_room_refused(&self, voice: &[u8; 32], draw: &[u64; 4], short: &[u64; 4]) {
        for l in crate::economy::lanes_short(short) {
            eprintln!(
                "  court room refused voice {:02x}{:02x}… lane {l} draw {} room {}",
                voice[0],
                voice[1],
                draw[l],
                draw[l] - short[l]
            );
        }
    }

    /// THE CONGESTION DOOR (26, 48; S9.6b): the fee on a lane is the
    /// reserve deficit on that lane itself — no divisor, since none is
    /// measured — and the speaker pays what its balance holds. The reading
    /// returned is per lane: on a lane the mesh has spent to nothing, the
    /// fee that went unpaid; zero on every lane that was paid on, or that
    /// still holds reserve. A calm mesh keeps a free door and reads zero
    /// everywhere. The caller acts on the lanes it reads; the one `bool`
    /// this replaced said "somewhere" and named nothing.
    pub(super) fn door_deferral(&mut self, voice: [u8; 32]) -> [u64; 4] {
        let deficit = self.physics.reserve_deficit();
        if crate::economy::lanes_short(&deficit).is_empty() {
            return [0; 4];
        }
        let anchor = self.anchor();
        // TWO READINGS, BOTH READ. A lane defers when the mesh has spent
        // that lane to nothing AND the speaker did not cover the fee on it;
        // the amount deferred is what it did not cover. The second half was
        // `paid[l] == 0`, a zero test standing for the shortfall, which
        // cannot tell a lane half-paid from one wholly unpaid and reported
        // the whole deficit for a single unit short. `settle_door` now
        // returns that reading, so the door reads it instead of guessing it.
        let (_paid, short) = self.treasury.settle_door(voice, deficit, anchor);
        std::array::from_fn(|l| match self.physics.reserve[l] == 0 {
            true => short[l],
            false => 0,
        })
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
            let j = self.live_slot(&c.addr).filter(|_| c.addr.as_bytes() != addr)?;
            resolved.push(Cite {
                word: j,
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
                let saved = candidate.magnitude();
                let payload = crate::transducer::rewrite::RewritePayload::Retire {
                    cites: cites.iter().map(|c| c.addr.clone()).collect(),
                    freed: saved,
                };
                let _ = self.mint_rewrite(
                    crate::transducer::rewrite::RewriteKind::Retire,
                    std::slice::from_ref(&candidate),
                    &candidate,
                    payload,
                    bidder,
                );
                self.basis_remove(addr);
                self.physics.syntropy(&candidate.magnitude());
                self.append_step(addr, bidder);
                Some(saved)
            }
            _ => None,
        }
    }

    /// GENOME SYNCHRONIZATION, commit step (operator-ruled: servers hold no
    /// dialects — one universal genome on the server plane, authored only
    /// by the court's finality). The caller (the deliberation worker) has
    /// already verified the blind replay and derivability OFF the lock on a
    /// basis snapshot; this brief step re-checks presence and installs. The
    /// ratifying voice is the discoverer of record — real provenance, no
    /// sentinel identity.
    pub fn install_adopted(&mut self, word: &Element, ratifier: [u8; 32]) -> Adoption {
        // The loop check at the mint: an arriving word's torsion cycles are
        // walked before they enter the genome. Only a witnessed closed loop
        // at the spoken period is believed.
        for (stored, witness) in crate::transducer::torsion_witness::witness_stored(word) {
            match witness {
                crate::transducer::torsion_witness::HolonomicLoopWitness::Closed { period, .. } if period == stored => {}
                crate::transducer::torsion_witness::HolonomicLoopWitness::Closed { .. } => {
                    return Adoption::Refused("torsion period does not re-derive — the winding was misspoken")
                }
                crate::transducer::torsion_witness::HolonomicLoopWitness::Open { .. } => {
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
                w.merge_fibre(word);
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

    /// A K-STEP on this hypha's chain, linked to the court: the operator
    /// digest folds the acting voice with the court head last ruled here,
    /// so the receipt is causally ordered against the mesh's chain of
    /// record and verifiable end to end.
    /// Seal one K-step, CITING every peer receipt admitted since the last
    /// one (47.2 step 3). The parents are taken only when the append
    /// succeeds: `append` refuses a parent whose height this chain cannot
    /// re-derive, and dropping the queue on a refusal would lose the
    /// admission rather than hold it for the step that can cite it.
    pub(super) fn append_step(&mut self, addr: impl AsRef<[u8]>, voice: [u8; 32]) {
        let op = crate::chronometry::operator_hash(&voice, &self.court_head);
        let queued = std::mem::take(&mut self.pending_parents);
        let parents: Vec<_> = queued
            .fiber(crate::network::nerve::ROOT)
            .map(|(_, p)| p.clone())
            .collect();
        if self.chain.append(addr.as_ref(), op, parents).is_none() {
            self.pending_parents = queued;
            self.chain.append(addr.as_ref(), op, Vec::new());
        }
    }

    /// A voice spoke on a listing in its current state.
    pub(super) fn record_spoken(&mut self, listing: crate::transducer::address::Address, voice: [u8; 32]) {
        self.spoken.nested_insert(&listing, voice, ());
        self.spoken_by.nested_insert(voice, &listing, ());
    }

    /// The receipts of speech, keyed by listing: read at a listing's own
    /// address, never walked.
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
        let floor = self.k_step().saturating_sub(crate::physics::reserve_max());
        while self.settlements.first().map(|s| s.k < floor && s.k + 1 < self.k_step()).unwrap_or(false) {
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
        for (addr, _) in self.pending.fiber(crate::network::nerve::ROOT) {
            p[2] = p[2].saturating_add(self.t_extent(&addr, now));
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
            let residual = element::from_residual_framed(&body, frame);
            let addr = residual.address();
            if self.basis.contains(&addr) || self.pending.contains(&addr) {
                continue; // already speakable or already listed
            }
            let k_now = self.k_step();
            self.stamp_standing(addr);
            self.book_list_undrawn(residual, k_now);
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
        if let Some(held) = self.basis.get(addr).map(|w| (**w).clone()) {
            let rebound = held.with_anchor(pi.clone());
            self.basis.insert(addr, std::sync::Arc::new(rebound));
            self.touch_at(addr);
            bound = true;
        }
        // A germ still awaiting its witness accepts the bind too — it
        // installs already anchored (evidence often reaches family size
        // before the witness re-derivation lands).
        if let Some((elem, listed)) = self.pending.get(addr).cloned() {
            self.pending.insert(addr, (elem.with_anchor(pi), listed));
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
        let short: [u64; 4] = std::array::from_fn(|l| bond[l].saturating_sub(held[l]));
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
