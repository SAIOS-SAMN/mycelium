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

    /// RM1 — FILE A WORD'S REMAINDERS at their index keys (the residue's
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
            match self.aliases.get_mut(one.as_bytes()) {
                Some(v) => v.push(*at),
                None => {
                    self.aliases.insert(one.as_bytes(), vec![*at]);
                }
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
        // Its aliases leave with it.
        if let Some(stamps) = self.aliases.remove(addr) {
            for at in stamps {
                self.held_at.remove(&at.key());
            }
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
        let addr = addr.as_ref();
        let mu = self.frame_of(addr);
        let held = self.cite_of(addr);
        let admits = match &held {
            None => self.named_word(addr).cloned(),
            Some(_) => None,
        };
        let written = self.seal_step(kind, mu, voice, |at| match (&held, &admits) {
            (Some(c), _) => vec![c.clone()],
            (None, Some(w)) => vec![Cite::word(w, at)],
            (None, None) => Vec::new(),
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
        self.append_cited(kind, cite.into_iter().collect(), mu, voice);
        self.forget_if_unheld(addr);
    }

    /// A K-STEP naming what it names by citations already read: the fiber
    /// and stamp of a word read BEFORE it left its carrier (78.24 (2c)),
    /// the root a fold leaves, the words a page taught.
    pub(super) fn append_cited(&mut self, kind: ActKind, cites: Vec<Cite>, mu: [u64; 4], voice: [u8; 32]) {
        self.seal_step(kind, mu, voice, |_| cites.clone());
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
        let me = self.identity;
        match self.seal_step(ActKind::Orbit, frame, me, |_| Vec::new()) {
            Some(r) => {
                // The orbit closed: what nothing flowed to for a whole orbit
                // lapses by `ZeroFlux` (78.37 (3)).
                self.lapse_silent();
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
    fn seal_step(
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
            Some(at) => self.chain.append(kind, mu, cites(at), op, parents).cloned(),
            None => None,
        };
        let written = match sealed {
            Some(r) => Some(r),
            None => {
                self.pending_parents = queued;
                let at = self.chain.next_stamp(&[])?;
                self.chain.append(kind, mu, cites(at), op, Vec::new()).cloned()
            }
        };
        match &written {
            Some(r) => self.note_attestor(r),
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
