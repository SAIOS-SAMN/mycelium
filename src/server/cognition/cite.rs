// 45.1, 78.22, 78.24 — THIS COURT'S ACTS ON THE LEDGER: each a node at its
// stamp, naming what it acts on by coordinates the reader already holds.
//
//! A word is named by its FIBER — its chart, and for a word with a rotor
//! its class record — and by the stamp of the act that ADMITTED it
//! (`admitted_at`, the ruled locator on every word row). The act that
//! first names a word no act has admitted is the act that admits it: it
//! cites its own stamp, and that stamp becomes the word's `admitted_at`.

use super::*;
use crate::chronometry::{ActKind, Cite, KStepReceipt, Parent, Stamp};

impl Kernel {
    /// THE CITATION OF A WORD THIS COURT HOLDS: its fiber and the stamp of
    /// the act that admitted it. `None` where the word is not held, or no
    /// act has admitted it yet.
    pub(crate) fn cite_of(&self, addr: &[u8]) -> Option<Cite> {
        let w = self.named_word(addr)?;
        let at = *self.admitted_at.get(addr)?;
        Some(Cite::word(w, at))
    }

    /// THE WORD AN ACT NAMES, on whichever row holds it: the basis, the
    /// cavity, provisional, the listing book, or the court's federation.
    pub(crate) fn named_word(&self, addr: &[u8]) -> Option<&Element> {
        self.word_at(addr).or_else(|| self.pending.get(addr)).or_else(|| {
            let i = *self.court_fed_index.get(addr)?;
            self.court_federation.get(i).map(|(_, w)| w)
        })
    }

    /// A WORD THAT HAS LEFT ITS LAST ROW takes its admitting stamp with it
    /// (78.24, R6): an entry nothing holds a word for is a copy of nothing.
    /// Called where a removal may have been the word's last row, after any
    /// act that cites it.
    pub(crate) fn forget_if_unheld(&mut self, addr: &[u8]) {
        if self.named_word(addr).is_none() {
            self.drop_admitted(addr);
        }
    }

    /// The stamp a held word was admitted at.
    pub fn admitted_at(&self, addr: &[u8]) -> Option<Stamp> {
        self.admitted_at.get(addr).copied()
    }

    /// LD3-1 — HELD WITHOUT A STAMP, BY ROW (78.27 Q1): the words each row
    /// holds that no act has admitted — basis, provisional, the listing
    /// book, the cavity, the court's federation. Every holding path admits
    /// by an act, so each count is a path that did not; a word no act
    /// admitted has no coordinates and the snapshot refuses it.
    pub fn unstamped(&self) -> [u64; 5] {
        let count = |keys: Vec<Vec<u8>>| keys.iter().filter(|a| !self.admitted_at.contains(a)).count() as u64;
        [
            count(self.basis.scoped().map(|(a, _)| a).collect()),
            count(self.provisional.scoped().map(|(a, _)| a).collect()),
            count(self.pending.scoped().map(|(a, _)| a).collect()),
            count(self.cavity_book.scoped().map(|(a, _)| a).collect()),
            count(self.court_federation.iter().map(|(a, _)| a.as_bytes().to_vec()).collect()),
        ]
    }

    /// The word an admitting stamp locates, where this court holds it.
    pub fn word_admitted_at(&self, at: &Stamp) -> Option<&Address> {
        self.held_at.get(&at.key())
    }

    /// FILE A WORD'S ADMITTING STAMP on its row, and the stamp's reverse.
    /// A word admitted once keeps the stamp it was admitted at.
    ///
    /// WHETHER THIS CALL FILED IT: `true` where the stamp given is now the
    /// word's, `false` where the word already stood at its own. A caller
    /// that files a peer's stamp provisionally and withdraws it on a
    /// refusal withdraws only what it filed — withdrawing unconditionally
    /// erased this court's OWN stamp on a word it still held whenever a
    /// peer offered the same address and the offer was refused, and that
    /// word, provisional and unstamped, held every fold prefix open for the
    /// life of the process (court a, epoch 99: unstamped provisional 0 → 4,
    /// folded never).
    ///
    /// A ruling that waited on the stamp is heard on the stroke of the act
    /// that files it (65.5), threaded here.
    pub fn file_admitted(&mut self, addr: &Address, at: Stamp, budget: &mut crate::physics::Budget) -> bool {
        if self.admitted_at.contains(addr) {
            return false;
        }
        self.admitted_at.insert_shared(addr.shared(), at);
        self.held_at.insert(at.key(), addr.clone());
        // THE ADMISSION IS THE FIRST FLUX (78.37 (3)): the word is filed where
        // it lapses by the stamp just given, and a provisional word's orbit
        // opens at it — the act that admits it offers it.
        let key = at.key();
        self.file_flux(&key);
        if self.provisional.contains(addr) {
            self.enter_flight(addr.as_bytes(), &key);
        }
        // LD3-a: the word by its fiber, where a row holds it to read the
        // fiber off; a restore files it from T1's key instead.
        let fiber = self.named_word(addr).and_then(|w| {
            let mut key = Vec::new();
            Cite::word(w, at).put_canonical(&mut key)?;
            Some(key)
        });
        if let Some(key) = fiber {
            self.file_fiber(addr, at, key);
        }
        // RM1: and its remainders, by where each was perceived.
        let keys: Vec<Vec<u8>> = match self.named_word(addr) {
            Some(w) => w.remainders().map(|r| r.index_key()).collect(),
            None => Vec::new(),
        };
        self.file_remainders(addr, keys);
        // EC1: a peer's ruling that waited on this stamp is heard now.
        self.rehear_rulings(&at, budget);
        true
    }

    /// FILE A WORD AT ITS FIBER KEY — T1's own key — beside its admitting
    /// stamp.
    pub(crate) fn file_fiber(&mut self, addr: &Address, at: Stamp, key: Vec<u8>) {
        self.fiber_key_at.insert(at.key(), key.clone());
        self.word_fibers.insert(key, addr.clone());
    }

    /// RM1 — FILE A WORD'S REMAINDERS at the remainders' index keys (the residue's
    /// chart ‖ κ, read off its torsion record). One remainder per class at a
    /// chart: a key already standing for another word is not filed over and
    /// is counted.
    pub(crate) fn file_remainders(&mut self, addr: &Address, keys: Vec<Vec<u8>>) {
        let mut filed = Vec::new();
        for k in keys {
            match self.remainder_fibers.get(&k) {
                Some(a) if a == addr => filed.push(k),
                Some(_) => self.remainders_coincident += 1,
                None => {
                    self.remainder_fibers.insert(&k, addr.clone());
                    filed.push(k);
                }
            }
        }
        if !filed.is_empty() {
            self.remainder_keys.insert(addr.as_bytes(), filed);
        }
    }

    /// RM1 — the words whose remainder of this class was perceived on this
    /// chart chain: a keyed read.
    pub fn remainder_at(&self, key: &[u8]) -> Option<&Address> {
        self.remainder_fibers.get(key)
    }

    /// LD3-a — THE FIBER READ: every word this court holds on the fiber
    /// `chart‖class`, whatever act admitted it — a prefix read on the words
    /// by fiber, never a scan. Each word once.
    pub fn fiber_read(
        &self,
        chart: &crate::transducer::stalk::Invariant,
        class: &Option<crate::chronometry::ClassRecord>,
    ) -> Vec<Address> {
        let mut key = Vec::new();
        let probe = Cite::Word { chart: chart.clone(), class: class.clone(), at: Stamp::default() };
        if probe.put_canonical(&mut key).is_none() {
            return Vec::new();
        }
        // The fiber is the citation without its stamp, which is written
        // last and fixed in length.
        key.truncate(key.len() - Stamp::LEN);
        let mut out: crate::network::nerve::Keyed<Address> = crate::network::nerve::Keyed::new();
        if let Some(hits) = self.word_fibers.fiber_at(&key) {
            for (_, addr) in hits {
                out.insert(addr.as_bytes(), addr.clone());
            }
        }
        out.scoped().map(|(_, a)| a.clone()).collect()
    }

    /// LD3-a — THE STAMP ALIAS (EC1's gap, as ruled): a peer node admitted
    /// here whose own act admits a word — a `Cite::Word` at the node's own
    /// stamp — names that word by a stamp this court did not admit it at.
    /// Where the fiber read here yields EXACTLY ONE word, the peer's stamp
    /// is filed as an alias of it: one word, two admissions, both stamps
    /// held (rule 15: never a choice). Where it yields several, nothing is
    /// filed — the word waits for the syllabus to teach it with its home
    /// stamp (78.24 (2b)). A ruling waiting on the aliased stamp is heard.
    /// On the stroke of the act that admitted the receipt (65.5).
    pub(crate) fn alias_admitting(&mut self, r: &KStepReceipt, budget: &mut crate::physics::Budget) {
        for c in &r.cites {
            let Cite::Word { chart, class, at } = c else { continue };
            if *at != r.stamp || self.word_admitted_at(at).is_some() {
                continue;
            }
            let hits = self.fiber_read(chart, class);
            let [one] = hits.as_slice() else { continue };
            let one = one.clone();
            self.held_at.insert(at.key(), one.clone());
            // ONE ALIAS PER SIGNER: a signer admits a word at one stamp at a
            // time, and its acts on the word cite that one; a newer
            // admission replaces the older, which leaves with its depth.
            let older: Vec<crate::chronometry::Stamp> = match self.aliases.get_mut(one.as_bytes()) {
                Some(v) => {
                    let (gone, kept): (Vec<_>, Vec<_>) = v.iter().partition(|s| s.vk == at.vk);
                    *v = kept;
                    v.push(*at);
                    gone
                }
                None => {
                    self.aliases.insert(one.as_bytes(), vec![*at]);
                    Vec::new()
                }
            };
            for s in older {
                self.held_at.remove(&s.key());
                self.attestors.remove(&s.key());
            }
            self.rehear_rulings(at, budget);
        }
    }

    /// A word left every row: its admitting stamp leaves with it.
    pub fn drop_admitted(&mut self, addr: &[u8]) {
        if let Some(at) = self.admitted_at.remove(addr) {
            // Filed nowhere: no flux, no orbit in flight.
            let key = at.key();
            self.forget_flux_at(&key);
            self.leave_flight(&key);
            self.held_at.remove(&key);
            // Its depth is read at its stamp, and leaves with it.
            self.attestors.remove(&key);
            if let Some(key) = self.fiber_key_at.remove(&at.key()) {
                self.word_fibers.remove(&key);
            }
        }
        // Its remainders leave the index by the keys it filed.
        if let Some(keys) = self.remainder_keys.remove(addr) {
            for k in keys {
                self.remainder_fibers.remove(&k);
            }
        }
        // Its aliases leave with it, and the depth read at each.
        if let Some(stamps) = self.aliases.remove(addr) {
            for at in stamps {
                self.held_at.remove(&at.key());
                self.attestors.remove(&at.key());
            }
        }
        self.drop_word_fields(addr);
    }

    /// WHAT THE COURT HELD OF A WORD THAT LEFT: the deflation its install
    /// settled, its coherence field hot or cold, the height it first stood
    /// at, the organs' re-derivations of it, the task its sentence was
    /// bound to, the task its listing paid, and what it taught each voice
    /// of it — the per-word fields the snapshot already refuses as orphans.
    fn drop_word_fields(&mut self, addr: &[u8]) {
        self.deflations.remove(addr);
        self.field.remove(addr);
        self.field_cold.remove(addr);
        self.standing_h.remove(addr);
        self.convergence.forget(addr);
        self.speech_task.remove(addr);
        if let Some(task) = self.listing_task.remove(addr) {
            let emptied = match self.task_listings.get_mut(task.as_bytes()) {
                Some(listings) => {
                    listings.retain(|l| l.as_bytes() != addr);
                    listings.is_empty()
                }
                None => false,
            };
            if emptied {
                self.task_listings.remove(task.as_bytes());
            }
        }
        let width = addr.len() + 32;
        let taught: Vec<Vec<u8>> = match self.taught_voice.fiber_at(addr) {
            Some(fiber) => fiber.map(|(key, ())| key).filter(|key| key.len() == width).collect(),
            None => Vec::new(),
        };
        for key in taught {
            self.taught_voice.remove(&key);
        }
    }

    /// THE FRAME AN ACT ON A WORD MEASURED: the word's magnitude on whichever
    /// row holds it — the basis, the cavity, provisional, the listing book
    /// or the court's federation. Where no row holds it, nothing measured
    /// it: the act's μ⃗ is read as `[0; 4]` and the clock counts an act
    /// sealed without a frame (ACT1, 78.27 Q1(a)). It read `word_at`, which
    /// does not read the listing book or the federation, so every List act
    /// on a held listing read no frame (ACT1-reading's return R1).
    pub(crate) fn frame_of(&mut self, addr: &[u8]) -> [u64; 4] {
        match self.named_word(addr) {
            Some(w) => w.magnitude(),
            None => {
                self.unframed += 1;
                [0; 4]
            }
        }
    }

    /// A K-STEP naming ONE WORD by its address, linked to the court: the
    /// operator digest folds the acting voice with the court head last
    /// ruled here. The word is cited by its fiber and admitting stamp; a
    /// held word no act has admitted is admitted by this one.
    ///
    /// WHAT THE ACT MEASURED is read here from this court's own carriers at
    /// the address (45.1, law 12), never handed in; an act naming a word
    /// this court no longer holds measured nothing, and says so.
    pub(super) fn append_step(&mut self, kind: ActKind, addr: impl AsRef<[u8]>, voice: [u8; 32]) {
        self.append_step_charted(kind, addr, voice, None);
    }

    /// `append_step` with the named word's chart already read where the
    /// caller holds it: the cite reuses it instead of perceiving the word
    /// again under the kernel.
    pub(super) fn append_step_charted(
        &mut self,
        kind: ActKind,
        addr: impl AsRef<[u8]>,
        voice: [u8; 32],
        chart: Option<crate::transducer::stalk::Invariant>,
    ) {
        let addr = addr.as_ref();
        let mu = self.frame_of(addr);
        let held = self.cite_of(addr);
        let admits = match &held {
            None => self.named_word(addr).cloned(),
            Some(_) => None,
        };
        let written = self.seal_step(kind, mu, voice, |at| match (&held, &admits, &chart) {
            (Some(c), _, _) => vec![c.clone()],
            (None, Some(w), Some(inv)) => vec![Cite::word_charted(w, inv.clone(), at)],
            (None, Some(w), None) => vec![Cite::word(w, at)],
            (None, None, _) => Vec::new(),
        });
        if let (Some(r), Some(_)) = (written, admits) {
            // OPEN (65.5): the filing re-hears the peer rulings that waited
            // on this stamp, and that re-hearing belongs to the act this step
            // seals — whose stroke never reaches here, because `append_step`
            // is the seal of every act and takes none. Until every act's
            // stroke is threaded to its seal, the stroke is minted here, the
            // one callee mint the ruling has not yet reached.
            let mut budget = self.physics.stroke();
            self.file_admitted(&Address::from(addr), r.stamp, &mut budget);
        }
    }

    /// A K-STEP ON A WORD THAT HAS LEFT ITS ROW (78.24 (2c)): cited by the
    /// citation read before the removal; what the act measured is read at
    /// the address now, which is zero where nothing holds it. A word that
    /// no row holds any more leaves its admitting stamp with it.
    pub(super) fn append_after_removal(&mut self, kind: ActKind, addr: &[u8], cite: Option<Cite>, voice: [u8; 32]) {
        let mu = self.frame_of(addr);
        let at = cite.as_ref().and_then(Cite::at);
        self.append_cited(kind, cite.into_iter().collect(), mu, voice);
        self.forget_if_unheld(addr);
        // Depth is read only where a word stands: the act sealed after its
        // word left files none at the stamp no word stands at.
        if let Some(at) = at {
            if !self.held_at.contains(&at.key()) {
                self.attestors.remove(&at.key());
            }
        }
    }

    /// A K-STEP naming what it names by citations already read: the fiber
    /// and stamp of a word read BEFORE it left its carrier (78.24 (2c)),
    /// the root a fold leaves, the words a page taught.
    pub(super) fn append_cited(&mut self, kind: ActKind, cites: Vec<Cite>, mu: [u64; 4], voice: [u8; 32]) {
        self.seal_step(kind, mu, voice, |_| cites.clone());
    }

    /// 78.93 B0 — A REFUSAL SEALED ON THIS COURT'S LINE: the refused
    /// sentence by its fiber and the refusal's kind, so a "no" and why
    /// outlive the process that said it. The act measured nothing it admits,
    /// so its frame is the zero reading. Its receipt, or `None` where the
    /// line could not take it — the refusal then stands in memory alone and
    /// the verdict says so by the head it answers with.
    pub(super) fn seal_refusal(&mut self, s: &Element, kind: crate::transducer::sentence::RefusalKind, voice: [u8; 32]) -> Option<KStepReceipt> {
        let cite = Cite::Refusal { chart: crate::transducer::stalk::invariant_of(s), kind: kind.to_byte() };
        self.seal_step(ActKind::Refused, [0; 4], voice, |_| vec![cite.clone()])
    }

    /// 78.120 — A DOMAIN FOUNDED (78.62's GENESIS act): the domain's signature
    /// over its genesis, its tier keys in the keys' one writing, a domain not
    /// founded here already and no tier key another domain names — then the
    /// genesis sealed on this court's line by its keys (`Cite::Domain`), and
    /// the domain held: from here its tiers' folds are admitted.
    pub fn admit_domain(&mut self, w: &crate::anastome::dialogue::domain::SignedDomain) -> crate::anastome::dialogue::domain::Founded {
        use crate::anastome::dialogue::domain::Founded;
        let g = &w.genesis;
        if !crate::court::identity::verify(&g.domain, &g.signed_bytes(), &w.sig) {
            return Founded::Unsigned;
        }
        if !g.ordered() {
            return Founded::Unordered;
        }
        if self.domains.get(&g.domain).is_some() {
            return Founded::AlreadyFounded;
        }
        if g.tiers.iter().any(|t| self.tier_domain.get(t).is_some()) {
            return Founded::TierHeld;
        }
        let cite = Cite::Domain { domain: g.domain, tiers: g.tiers.clone() };
        match self.seal_step(ActKind::Domain, [0; 4], g.domain, |_| vec![cite.clone()]) {
            Some(r) => {
                self.hold_domain(g.domain, g.tiers.clone());
                self.mark_persist();
                Founded::Sealed { at: r.stamp, receipt: r.chain_hash }
            }
            None => Founded::LineRefused,
        }
    }

    /// A domain and its tier keys held, and each tier indexed to it — the one
    /// writer of both, at a founding and at a restore (law 12).
    pub(crate) fn hold_domain(&mut self, domain: [u8; 32], tiers: Vec<[u8; 32]>) {
        for t in &tiers {
            self.tier_domain.insert(t, domain);
        }
        self.domains.insert(domain, tiers);
    }

    /// 78.119 — A TIER'S FOLD ADMITTED (78.62 step 3). This court is the
    /// verifier outside the tier's delegation closure: the tier's signature
    /// over the fold, then its continuity (every lane's nets sum exactly to
    /// its declared emission); admitted, the fold's coordinates — the tier,
    /// the prefix head it nets, its rows' commitment — are sealed on this
    /// court's line and its receipt id returned. Its frame is the zero
    /// reading: a tier's settlement moves nothing in this court's book.
    pub fn admit_uplink(&mut self, f: &crate::anastome::dialogue::uplink::SignedFold) -> crate::anastome::dialogue::uplink::Uplinked {
        use crate::anastome::dialogue::uplink::Uplinked;
        if !crate::court::identity::verify(&f.fold.tier, &f.fold.signed_bytes(), &f.sig) {
            return Uplinked::Unsigned;
        }
        // 78.120 — the tier must be named by a domain's genesis on this court.
        let Some(domain) = self.tier_domain.get(&f.fold.tier).copied() else {
            return Uplinked::Unbound;
        };
        // 78.138 — ONLY THE ANCIENTS UPLINK (the Concordance, 4.1): the
        // folding tier is its domain's reserve; a line's fold is carried in
        // its reserve's, and lines never touch the court.
        if self.reserve_of(&domain) != Some(f.fold.tier) {
            return Uplinked::NotReserve;
        }
        // 78.125 — a fold over its tier's current head: a resend answered with
        // its original receipt, or a repeat refused; nothing sealed twice.
        if let Some(answer) = self.repeat_of(&f.fold, &domain) {
            return answer;
        }
        // 78.126 — the handoff sides it consumes, proved, and the sides' nets: what
        // crosses the domain's boundary stands with its declared emission.
        let crossing = match self.sides_of(&f.fold) {
            Ok(nets) => nets,
            Err((index, why)) => return Uplinked::HandoffRefused { index, why },
        };
        if let Err(d) = f.fold.continuity_with(&crossing) {
            return Uplinked::Discontinuous(d);
        }
        // 78.154a — its CONNECTION_TAX flow, read before anything seals.
        let tax = match self.connection_tax(&f.fold) {
            Ok(mu) => mu,
            Err(why) => return Uplinked::Tax(why),
        };
        // 78.123 B — a fold netting children is read on the ledger tree.
        let reply = match f.fold.children.is_empty() {
            false => self.admit_chained(f, domain),
            true => {
                let cite = Cite::Uplink { tier: f.fold.tier, prefix: f.fold.prefix, rows: f.fold.rows_hash() };
                match self.seal_step(ActKind::Uplink, [0; 4], f.fold.tier, |_| vec![cite.clone()]) {
                    Some(r) => {
                        // 78.123 B′, 78.125 — held by its sealed coordinates
                        // until a parent nets all of it, whatever the hot
                        // line folds meanwhile, and recorded for its
                        // challenge window.
                        self.hold_admitted(r.stamp, &f.fold, domain, Vec::new());
                        self.mark_persist();
                        Uplinked::Admitted { at: r.stamp, receipt: r.chain_hash }
                    }
                    None => Uplinked::LineRefused,
                }
            }
        };
        // Sealed: each side it consumed is spent, once.
        if let Uplinked::Admitted { at, receipt } | Uplinked::Chained { at, receipt, .. } = &reply {
            // 78.154a — the tax paid from the reserve into the recycled pool.
            self.pay_connection_tax(f.fold.tier, tax, (*at, *receipt));
            self.consume_sides(&at.key(), &f.fold.handoffs);
            // 78.127 — the reserve tier's fold is its era anchor: a pending
            // redesignation holds from here.
            self.anchor_passed(&f.fold.tier, &domain);
        }
        reply
    }

    /// OK1 — THE COURT'S CYCLE CLOSES BY NAME (78.23 as amended): one
    /// circuit of exchange, deliberation, lapse and persist is one orbit,
    /// and its close is an act — a node of kind `Orbit` at the orbit's last
    /// place, carrying the orbit's frame, the per-lane sum over its acts.
    /// `K` moves here and nowhere else.
    ///
    /// A COLLISION IS PREVENTED, NEVER NAMED AFTER: the close takes its tick
    /// from the parents it would cite, and where a peer's orbit closed at
    /// that tick with a frame equal on every lane, this close is WITHHELD —
    /// the orbit stays open, the next stroke's work (the docket's next
    /// question) is the separating transport, and the count goes on the
    /// clock. A close with no act since the last seals nothing.
    pub fn orbit(&mut self) -> crate::chronometry::Close {
        use crate::chronometry::Close;
        let Some((_, frame)) = self.chain.open_orbit() else { return Close::Nothing };
        let parents = self.antichain_of(&self.pending_parents);
        let Some(at) = self.chain.next_stamp(&parents).or_else(|| self.chain.next_stamp(&[])) else {
            return Close::Nothing;
        };
        if self.chain.collides(at.h, frame) {
            self.withheld += 1;
            return Close::Withheld;
        }
        let own_vk = self.identity;
        match self.seal_step(ActKind::Orbit, frame, own_vk, |_| Vec::new()) {
            Some(r) => {
                // The orbit closed: what nothing flowed to for a whole orbit
                // lapses by `ZeroFlux` (78.37 (3)).
                self.lapse_silent();
                // 78.125 — and the challenge windows settle: final records
                // pruned to stubs, spent stubs and stale replays let go.
                self.settle_windows();
                // AND THE RESERVE REGENERATES within its capacity (78.56 IV):
                // Σ active bonds plus the work this court verified since the
                // last close — never a hardware constant. The court never
                // regenerated but on hearing a sentence, so its reserve
                // settled near the floor and refused 95–99% of its syllabus
                // pages on the development deployment (epoch 106). A bond total that does not
                // add up is unmeasured (STD1): the capacity is the verified
                // work alone.
                #[expect(
                    clippy::manual_unwrap_or_default,
                    reason = "rule 11 — `unwrap_or_default` is the spelling this refuses; an unmeasured bond total contributes nothing to the capacity, named"
                )]
                let bonds = match self.treasury.bond_total() {
                    Some(b) => b,
                    None => [0; 4],
                };
                self.physics.regenerate_within(bonds);
                // 78.69: AND THE PRESSURE RELAXES BY THE ORDER THE ORBIT BUILT,
                // per lane (PHY1). It relaxed only where an utterance was
                // heard (`reflex_open`), so a court booking order without
                // hearing one kept its pressure whole.
                self.physics.relax();
                // 78.67 — THE FOLD IS ASKED AT THE CLOSE, and the prefix
                // answers (45.8 as amended): it folds where it carries work
                // and that work outweighs the era's framing
                // (`fold_amortizes`), never on a height interval, a memory
                // reading, a count of docket circuits or a reserve reading.
                // 78.66 drew the hot ledger's node count from a reserve
                // counted in strokes; the two never met, so every lane was
                // short at every close and the courts folded at each one.
                if self.fold_pending.is_none() {
                    // Counted by outcome, never discarded (LD0).
                    let outcome = match self.propose_fold_at_close() {
                        Ok(_) => crate::observe::FoldOutcome::Proposed,
                        Err(crate::verdict::REFUSE_FOLD_OPEN) => crate::observe::FoldOutcome::Open,
                        Err(crate::verdict::REFUSE_FOLD_IDLE) => crate::observe::FoldOutcome::Idle,
                        Err(crate::verdict::REFUSE_FOLD_NO_TRANSITION) => crate::observe::FoldOutcome::NoTransition,
                        Err(_) => crate::observe::FoldOutcome::Inexact,
                    };
                    crate::observe::note_fold(outcome);
                }
                Close::Sealed(r.stamp)
            }
            None => Close::Nothing,
        }
    }

    /// SEAL ONE STEP, citing the peer receipts admitted since the last
    /// (47.2 step 3). The parents are taken only when the append succeeds:
    /// a refused parent is held for the step that can cite it. `cites`
    /// receives the stamp this node will take, for an act that admits
    /// what it names. L15: the node joins the depth index where it joins
    /// the ledger.
    pub(super) fn seal_step(
        &mut self,
        kind: ActKind,
        mu: [u64; 4],
        voice: [u8; 32],
        cites: impl Fn(Stamp) -> Vec<Cite>,
    ) -> Option<KStepReceipt> {
        let op = crate::chronometry::operator_hash(&voice, &self.court_head);
        let queued = std::mem::take(&mut self.pending_parents);
        let parents = self.antichain_of(&queued);
        let sealed = match self.chain.next_stamp(&parents) {
            Some(at) => self.chain.append(kind, mu, cites(at), op, parents),
            None => None,
        };
        let written = match sealed {
            Some(r) => Some(r),
            None => {
                self.pending_parents = queued;
                let at = self.chain.next_stamp(&[])?;
                self.chain.append(kind, mu, cites(at), op, Vec::new())
            }
        };
        match &written {
            Some(r) => {
                self.note_attestor(r);
                // 78.69: where this court's algorithm last changed, kept as
                // it is sealed so the fold need not search the line for it.
                if kind.changes_algorithm() {
                    self.last_transition = Some(Some(r.stamp.place()));
                }
            }
            None => eprintln!("  step refused: {} could not be sealed", String::from_utf8_lossy(kind.name())),
        }
        written
    }

    /// THE MAXIMAL ANTICHAIN of what this step has admitted: the
    /// un-dominated heads of the peer DAG, which is what 47.2 step 3 means
    /// by citing a parent. Two prunes, both bounded by the mesh's width:
    /// per signer, only its highest place (a line is reached through its
    /// head); and a head another head cites directly, read off that head's
    /// own node. What is left is a superset of the antichain, never a
    /// subset, so no parent a height re-derives through is dropped.
    fn antichain_of(&self, queued: &crate::network::nerve::Keyed<Parent>) -> Vec<Parent> {
        let mut heads: crate::network::nerve::Keyed<Parent> = crate::network::nerve::Keyed::new();
        for (_, p) in queued.scoped() {
            match heads.get(&p.at.vk) {
                Some(h) if h.at.place() >= p.at.place() => {}
                _ => {
                    heads.insert(p.at.vk, *p);
                }
            }
        }
        let mut cited: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        for (_, p) in heads.scoped() {
            let Some(r) = self.chain.node(&p.at) else { continue };
            for q in r.prev.iter().chain(r.parents.iter()) {
                cited.insert(q.at.key(), ());
            }
        }
        heads.scoped().map(|(_, p)| *p).filter(|p| !cited.contains(&p.at.key())).collect()
    }
}
