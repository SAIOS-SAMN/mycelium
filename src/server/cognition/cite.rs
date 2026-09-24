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
    fn named_word(&self, addr: &[u8]) -> Option<&Element> {
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

    /// The word an admitting stamp locates, where this court holds it.
    pub fn word_admitted_at(&self, at: &Stamp) -> Option<&Address> {
        self.held_at.get(&at.key())
    }

    /// FILE A WORD'S ADMITTING STAMP on its row, and the stamp's reverse.
    /// A word admitted once keeps the stamp it was admitted at.
    pub fn file_admitted(&mut self, addr: &Address, at: Stamp) {
        if self.admitted_at.contains(addr) {
            return;
        }
        self.admitted_at.insert_shared(addr.shared(), at);
        self.held_at.insert(at.key(), addr.clone());
    }

    /// A word left every row: its admitting stamp leaves with it.
    pub fn drop_admitted(&mut self, addr: &[u8]) {
        if let Some(at) = self.admitted_at.remove(addr) {
            self.held_at.remove(&at.key());
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
        let mu = self.word_at(addr).map_or([0; 4], |w| w.magnitude());
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
            self.file_admitted(&Address::from(addr), r.stamp);
        }
    }

    /// A K-STEP ON A WORD THAT HAS LEFT ITS ROW (78.24 (2c)): cited by the
    /// citation read before the removal; what the act measured is read at
    /// the address now, which is zero where nothing holds it. A word that
    /// no row holds any more leaves its admitting stamp with it.
    pub(super) fn append_after_removal(&mut self, kind: ActKind, addr: &[u8], cite: Option<Cite>, voice: [u8; 32]) {
        let mu = self.word_at(addr).map_or([0; 4], |w| w.magnitude());
        self.append_cited(kind, cite.into_iter().collect(), mu, voice);
        self.forget_if_unheld(addr);
    }

    /// A K-STEP naming what it names by citations already read: the fiber
    /// and stamp of a word read BEFORE it left its carrier (78.24 (2c)),
    /// the root a fold leaves, the words a page taught.
    pub(super) fn append_cited(&mut self, kind: ActKind, cites: Vec<Cite>, mu: [u64; 4], voice: [u8; 32]) {
        self.seal_step(kind, mu, voice, |_| cites.clone());
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
    /// per signer, only its highest `K` (a line is reached through its
    /// head); and a head another head cites directly, read off that head's
    /// own node. What is left is a superset of the antichain, never a
    /// subset, so no parent a height re-derives through is dropped.
    fn antichain_of(&self, queued: &crate::network::nerve::Keyed<Parent>) -> Vec<Parent> {
        let mut heads: crate::network::nerve::Keyed<Parent> = crate::network::nerve::Keyed::new();
        for (_, p) in queued.scoped() {
            match heads.get(&p.at.vk) {
                Some(h) if h.at.k >= p.at.k => {}
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
