// 23.8 — THE LANGUAGE SNAPSHOT: the arc a reduction runs against with no
// lock held, and the closure it is sealed at.

use super::super::language::{Language, LanguageWord};
use crate::cognition::*;
use crate::transducer::element::Element;

impl Kernel {
    /// The digests the court reconciles against: the ruled cover, served to
    /// the court's own voice. Any other asker (an entity's sync) is answered
    /// with the whole language elsewhere.
    pub fn ruled_digests_for(&self, voice: &[u8; 32]) -> Option<Vec<crate::transducer::sheaf::StratumDigest>> {
        (self.court_voice.as_ref() == Some(voice)).then(|| crate::transducer::sheaf::digests(&self.ruled))
    }

    /// Remove the word at `addr` from the basis, its discoverer, and its stalk.
    pub(crate) fn basis_remove(&mut self, addr: &[u8]) -> Element {
        let Some(held) = self.basis.get(addr).map(|w| w.element().clone()) else {
            return Element::identity();
        };
        self.unindex_word(&held);
        let inv = crate::transducer::stalk::invariant_of(&held);
        let words = crate::transducer::sentence::acting_of_element(&held);
        crate::transducer::sentence::retract_words(&mut self.axis_index, &inv, &words);
        self.forget_discoverer(addr);
        std::sync::Arc::make_mut(&mut self.stalks).remove(addr);
        self.ruled.remove(addr);
        if self.ruled_addrs.remove(addr).is_some() {
            self.refresh_ruled_root();
        }
        self.basis_gen += 1;
        self.retire_gen += 1;
        self.basis.remove(addr);
        self.language_drop(addr);
        held
    }

    pub(crate) fn basis_clear(&mut self) {
        self.basis = crate::network::nerve::Keyed::new();
        self.by_action = crate::network::nerve::Keyed::new();
        self.by_footprint = crate::network::nerve::Keyed::new();
        self.discoverers = crate::network::nerve::Keyed::new();
        self.discovered_by = crate::network::nerve::Keyed::new();
        self.stalks = std::sync::Arc::new(crate::transducer::stalk::Stalks::default());
        self.basis_gen += 1;
        self.language_rebuild();
    }

    /// A held word changed in place: the Arc in `basis` is already unique
    /// (via `word_mut`); rebuild the language pointer slice.
    pub(crate) fn touch_at(&mut self, _addr: &[u8]) {
        self.basis_gen += 1;
        self.language_rebuild();
    }

    /// The language as an immutable shared snapshot. An append clones the
    /// pointer slice and one new Arc; a retirement rebuilds. Never clones
    /// the whole basis of Elements.
    pub fn language(&mut self) -> std::sync::Arc<Language> {
        if self.language_gen != self.basis_gen || self.language.len() != self.basis.len() {
            self.language_rebuild();
        }
        std::sync::Arc::clone(&self.language)
    }

    /// THE SNAPSHOT IS THE BASIS AT ITS OWN COORDINATES. The basis is a
    /// `Keyed` by address; this used to flatten it into a vector and build
    /// an index to undo the flattening, plus a third vector of empty
    /// `OnceLock`s to sit parallel to it.
    pub(crate) fn language_rebuild(&mut self) {
        let mut words: crate::network::nerve::Keyed<LanguageWord> =
            crate::network::nerve::Keyed::new();
        let mut seq = 0u64;
        for (addr, w) in self.basis.scoped() {
            words.insert(
                &addr,
                LanguageWord {
                    held: std::sync::Arc::clone(w),
                    shape: std::sync::OnceLock::new(),
                    seq,
                },
            );
            seq = seq.saturating_add(1);
        }
        self.language = std::sync::Arc::new(Language {
            words,
            stalks: std::sync::Arc::clone(&self.stalks),
            retire_gen: self.retire_gen,
            next_seq: seq,
        });
        self.language_gen = self.basis_gen;
    }

    /// A word leaves the snapshot. It used to rebuild the vector, re-key
    /// the whole index and discard every shape lifted so far, because a
    /// removal renumbered everything after the hole.
    pub(crate) fn language_drop(&mut self, addr: &[u8]) {
        let stalks = std::sync::Arc::clone(&self.stalks);
        let retire_gen = self.retire_gen;
        let lang = std::sync::Arc::make_mut(&mut self.language);
        lang.words.remove(addr);
        lang.stalks = stalks;
        lang.retire_gen = retire_gen;
        self.language_gen = self.basis_gen;
    }

    pub(crate) fn language_append_arc(
        &mut self,
        word: std::sync::Arc<crate::transducer::element::Held>,
    ) {
        let addr = word.address();
        let inv = crate::transducer::stalk::invariant_of(word.element());
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
        // One vertex, one coordinate: a word already at this address is
        // replaced with its lifted shape, keeping the ordinal it was
        // admitted at; a new one takes the next ordinal.
        let seq = match lang.words.get(&addr) {
            Some(w) => w.seq,
            None => {
                let s = lang.next_seq;
                lang.next_seq = lang.next_seq.saturating_add(1);
                s
            }
        };
        lang.words.insert(&addr, LanguageWord { held: word, shape: std::sync::OnceLock::new(), seq });
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
        for (addr, _) in self.pending.scoped() {
            // A listing whose T cannot be measured carries no T on the
            // view: the key is absent, which is what unmeasured is here,
            // and not a zero every reader would take for a fresh listing.
            if let Some(t) = self.t_extent(&addr, now) {
                standing_t.insert(&addr, t);
            }
        }
        MarketView {
            language: self.language(),
            pending: std::sync::Arc::clone(&self.book.1),
            surface: crate::economy::yield_surface(&self.physics),
            pressure: self.physics.pressure,
            pool: self.treasury.bond_of(&self.identity),
            k_step: self.chain.next_step_index(),
            head_k: self.chain.head_k(),
            head_height: now,
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
    pub(crate) fn closure_now(&mut self) -> std::sync::Arc<Rank> {
        let gens = (self.basis_gen, self.book_gen);
        if (self.closure.0, self.closure.1) != gens {
            let language = self.language();
            let measured = rank_over(&language, &self.pending, &self.closure.2, gens.0, &self.chart_listings);
            self.closure = (gens.0, gens.1, std::sync::Arc::new(measured));
        }
        std::sync::Arc::clone(&self.closure.2)
    }

    /// The settlement ledger (bounties, consilience, recall yields).
    pub fn treasury_entry_count(&self) -> usize {
        self.treasury.entry_count()
    }

    pub fn treasury(&self) -> &crate::economy::Treasury {
        &self.treasury
    }

    /// Restrict stakes to the live genesis identities: a stake outside it
    /// is burned at this court's position (§6 holds, I38). Returns what
    /// left, per identity.
    pub fn bind_live(&mut self, live: &[[u8; 32]]) -> Vec<([u8; 32], [u64; 4])> {
        let anchor = self.anchor();
        self.treasury.bind_live(live, anchor)
    }

    #[cfg(test)]
    pub(crate) fn treasury_mut(&mut self) -> &mut crate::economy::Treasury {
        &mut self.treasury
    }

}
