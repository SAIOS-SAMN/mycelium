// 24.2 — THE DIALECT: what this client stands but the court has not ruled,
// and what it is waiting on before it can.

use crate::client::sdk::*;
use crate::transducer::element::Element;

impl Epigenome {
    pub fn dialect_len(&self) -> usize {
        self.dialect.len()
    }

    pub(crate) fn wait_on(&mut self, missing: &[u8], citer: impl AsRef<[u8]>) {
        let citer = citer.as_ref();
        if let Some(citers) = self.waiting_on.get_mut(missing) {
            citers.insert(citer, ());
            return;
        }
        let mut citers = crate::network::nerve::Keyed::new();
        citers.insert(citer, ());
        self.waiting_on.insert(missing, citers);
    }

    pub(crate) fn retract_waiter(&mut self, missing: &[u8], citer: &[u8]) {
        let empty = match self.waiting_on.get_mut(missing) {
            Some(citers) => {
                citers.remove(citer);
                citers.is_empty()
            }
            None => return,
        };
        if empty {
            self.waiting_on.remove(missing);
        }
    }

    pub(crate) fn drop_dialect(&mut self, da: &[u8]) {
        let (shed, missing) = match self.dialect.get(da) {
            Some(d) => (
                d.sentence.clone(),
                if d.withheld {
                    d.missing.clone()
                } else {
                    Vec::new()
                },
            ),
            None => {
                self.dialect_atlas.remove(da);
                self.dialect.remove(da);
                return;
            }
        };
        self.moduli_retract(&shed);
        match shed.form() {
            Some(f) => self.vocabulary.write().unfile_form(&shed.address(), f),
            None => self.vocabulary.write().unfile_letter(&shed.address()),
        }
        // THE MIRROR OF THE ADMISSION, CHART FOR CHART. `stand_dialect_at`
        // admits through `axis_admit`, which for a sentence files each Word
        // vertex's word under every chart that vertex ACTS on. This retracted
        // through `retract_words` at `invariant_of(&d.sentence)` — the
        // SENTENCE's own chart, `chart_of_form`, leading `2`, where the
        // admission filed at body charts leading `1`.
        //
        // `retract_words` returns at its first keyed read on the missing outer
        // key, so nothing was ever removed: every word of every shed dialect
        // sentence stayed in the axis index for the life of the process, and
        // the index is on the stroke path with no other drain. The correct
        // mirror already existed and was reached only from genome removal.
        self.axis_retract(&shed);
        for m in &missing {
            self.retract_waiter(m, da);
        }
        self.dialect_atlas.remove(da);
        self.dialect.remove(da);
        // RM1 — its remainders leave the index, where it is not resident.
        self.drop_remainders(da);
    }

    pub fn stand_dialect_at(&mut self, sentence: Element, k: u64, now_tu: u64) -> Option<Vec<u8>> {
        if let Some(at) = self.same_vertex_in_genome(&sentence) {
            if let Some(w) = self.genome.get_mut(&at) {
                #[expect(clippy::disallowed_methods, reason = "rule 11 — one per use: a u64 of events cannot pass 2⁶⁴")]
                let n = w.expression.saturating_add(1);
                w.expression = n;
            }
            return None;
        }
        let addr = sentence.address();
        if let Some(d) = self.dialect.get_mut(addr.as_bytes()) {
            d.last_used_k = k;
            d.last_used_tu = now_tu;
            #[expect(clippy::disallowed_methods, reason = "rule 11 — one per use: a u64 of events cannot pass 2⁶⁴")]
            let n = d.uses.saturating_add(1);
            d.uses = n;
            return Some(addr.into_vec());
        }
        if let Some(existing) = self.same_vertex_in_dialect(&sentence) {
            if let Some(d) = self.dialect.get_mut(&existing) {
                d.last_used_k = k;
                d.last_used_tu = now_tu;
                #[expect(clippy::disallowed_methods, reason = "rule 11 — one per use: a u64 of events cannot pass 2⁶⁴")]
                let n = d.uses.saturating_add(1);
                d.uses = n;
            }
            return Some(existing);
        }
        let inv = crate::transducer::stalk::invariant_of(&sentence);
        self.dialect_atlas.admit(&inv, addr.clone());
        let shape = crate::transducer::element::shape_of_chain(sentence.sub());
        // ONE DECODE, HERE, where the sentence stands (L16).
        let sentence = crate::transducer::element::Held::of(sentence);
        self.axis_admit(&sentence);
        self.moduli_admit(&sentence);
        // 78.161 — the vocabulary learns the mind's own words too, as its
        // dialect.
        match sentence.form() {
            Some(f) => {
                self.vocabulary.write().learn_form(&addr, f, false);
            }
            None => {
                self.vocabulary.write().learn_letter(&addr, sentence.element(), false);
            }
        }
        // RM1 — its remainders, by where they were perceived.
        self.file_remainders(&addr, sentence.element());
        self.dialect.insert(
            addr.clone(),
            Dialect {
                sentence,
                last_used_k: k,
                last_used_tu: now_tu,
                uses: 1,
                shape,
                withheld: false,
                missing: Vec::new(),
            },
        );
        Some(addr.into_vec())
    }

    /// Stand a withheld form in the dialect (24.2, 24.3): `withheld: true`,
    /// never the genome.
    pub fn stand_dialect_withheld(
        &mut self,
        sentence: Element,
        k: u64,
        now_tu: u64,
    ) -> Option<Vec<u8>> {
        let addr = self.stand_dialect_at(sentence, k, now_tu)?;
        if let Some(d) = self.dialect.get_mut(&addr) {
            d.withheld = true;
        }
        Some(addr)
    }

    /// EXPRESS a residual: germinate it as this entity's own word. Returns
    /// false when the expressed vocabulary already speaks it (the same body
    /// under a transport) — a claim already made, not to be made again.
    /// `k` is the voice's K (law 9); `now_tu` is TUniversal.
    #[cfg(test)]
    pub(crate) fn express(
        &mut self,
        residual: &[(i64, i64, u8, u8)],
        k: u64,
        now_tu: u64,
        budget: &mut crate::physics::Budget,
    ) -> bool {
        if residual.is_empty() {
            return false;
        }
        let located = {
            let expressed = &self.expressed;
            let word_at = |a: &[u8]| expressed.get(a).map(|e| &e.word);
            let shape_at = |a: &[u8]| {
                let e = expressed.get(a)?;
                e.shape
                    .get_or_init(|| crate::transducer::element::shape_of_chain(e.word.sub()))
                    .as_ref()
            };
            crate::transducer::sentence::deconstruct_at(
                &word_at,
                &self.expressed_atlas,
                &shape_at,
                residual,
                None,
                budget,
            )
        };
        if let ([], []) = (located.residual.as_slice(), located.punctures.as_slice()) {
            return false; // already spoken: the same body under a transport
        }
        // A residual with no A5 form is no word, so nothing is expressed.
        let Some(word) = crate::transducer::element::from_residual(&residual.to_vec()) else {
            return false;
        };
        let at = word.address();
        self.expressed_atlas.admit(&crate::transducer::stalk::invariant_of(&word), at.clone());
        self.stand_dialect_at(word.clone(), k, now_tu);
        self.expressed.insert(at, ExpressedWord { word, shape: std::sync::OnceLock::new() });
        true
    }

    /// The strongest-expressed word's weight (the dialect's leading edge).
    /// `None` where the dialect expresses nothing: the extremum of an empty
    /// carrier is not zero — zero is a weight some word could carry — it is
    /// a measurement with nothing to take it on.
    #[expect(
        clippy::disallowed_methods,
        reason = "rule 4 — the peak of the expression readings is the readings' extent, content"
    )]
    #[cfg(test)]
    pub(crate) fn peak_expression(&self) -> Option<u64> {
        self.genome.scoped().map(|(_, w)| w.expression).max()
    }

    #[cfg(test)]
    pub(crate) fn waiting_on_len(&self, missing: &[u8]) -> usize {
        // Nothing waits on a word the carrier does not name.
        match self.waiting_on.get(missing) {
            Some(c) => c.len(),
            None => 0,
        }
    }

}
