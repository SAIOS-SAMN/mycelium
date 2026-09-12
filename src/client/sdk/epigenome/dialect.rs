// 24.2 — THE DIALECT: what this client stands but the court has not ruled,
// and what it is waiting on before it can.

use crate::client::sdk::*;
use crate::transducer::element::Element;

impl Epigenome {
    /// The count of taught words refused for not verifying.
    pub fn refused(&self) -> u64 {
        self.refused
    }

    pub fn dialect_len(&self) -> usize {
        self.dialect.len()
    }

    /// True when a dialect sentence lives on this chart (exact invariant).
    pub fn dialect_holds(&self, c: &crate::transducer::stalk::Invariant) -> bool {
        !self.dialect_atlas.fiber(c).is_empty()
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
        if let Some(d) = self.dialect.get(da) {
            let shed = d.sentence.clone();
            self.moduli_retract(&shed);
        }
        let (inv, words, missing) = match self.dialect.get(da) {
            Some(d) => (
                crate::transducer::stalk::invariant_of(&d.sentence),
                crate::transducer::sentence::words_of_element(&d.sentence),
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
        crate::transducer::sentence::retract_words(&mut self.axis_index, &inv, &words);
        for m in &missing {
            self.retract_waiter(m, da);
        }
        self.dialect_atlas.remove(da);
        self.dialect.remove(da);
    }

    pub fn stand_dialect_at(&mut self, sentence: Element, k: u64, now_tu: u64) -> Option<Vec<u8>> {
        if let Some(i) = self.same_vertex_in_genome(&sentence) {
            if let Some(x) = self.expression.get_mut(i) {
                *x = x.saturating_add(1);
            }
            return None;
        }
        let addr = sentence.address();
        if let Some(d) = self.dialect.get_mut(addr.as_bytes()) {
            d.last_used_k = k;
            d.last_used_tu = now_tu;
            d.uses = d.uses.saturating_add(1);
            return Some(addr.into_vec());
        }
        if let Some(existing) = self.same_vertex_in_dialect(&sentence) {
            if let Some(d) = self.dialect.get_mut(&existing) {
                d.last_used_k = k;
                d.last_used_tu = now_tu;
                d.uses = d.uses.saturating_add(1);
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
        eprintln!(
            "  dialect stood {:02x}{:02x}… k {k} tu {now_tu}",
            addr[0], addr[1]
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

    /// The words this entity has expressed and not yet had ratified.
    pub fn expressed_len(&self) -> usize {
        self.expressed.len()
    }

    /// EXPRESS a residual: germinate it as this entity's own word. Returns
    /// false when the expressed vocabulary already speaks it (the same body
    /// under a transport) — a claim already made, not to be made again.
    /// `k` is the voice's K (law 9); `now_tu` is TUniversal.
    pub fn express(&mut self, residual: &[(i64, i64, u8, u8)], k: u64, now_tu: u64) -> bool {
        if residual.is_empty() {
            return false;
        }
        let located = {
            let expressed = &self.expressed;
            let shapes = &self.expressed_shapes;
            let index = &self.expressed_index;
            let shape_at = |i: usize| {
                shapes.get(i).and_then(|c| c.get_or_init(|| crate::transducer::element::shape_of_chain(expressed[i].sub())).as_ref())
            };
            crate::transducer::sentence::deconstruct(expressed, &self.expressed_atlas, &|a| index.get(a).copied(), &shape_at, residual, None)
        };
        if let ([], []) = (located.residual.as_slice(), located.punctures.as_slice()) {
            return false; // already spoken: the same body under a transport
        }
        // A residual with no A5 form is no word, so nothing is expressed.
        let Some(word) = crate::transducer::element::from_residual(&residual.to_vec()) else {
            return false;
        };
        self.expressed_atlas.admit(&crate::transducer::stalk::invariant_of(&word), word.address());
        self.expressed_index.insert(word.address(), self.expressed.len());
        self.expressed_shapes.push(std::sync::OnceLock::new());
        self.stand_dialect_at(word.clone(), k, now_tu);
        self.expressed.push(word);
        true
    }

    /// The strongest-expressed word's weight (the dialect's leading edge).
    /// A dialect expressing nothing has no leading edge, which is a weight
    /// of zero measured on an empty carrier and not a default standing in
    /// for one.
    #[expect(
        clippy::disallowed_methods,
        reason = "rule 4 — the peak of the expression vector is its extent, content"
    )]
    pub fn peak_expression(&self) -> u64 {
        #[expect(
            clippy::manual_unwrap_or_default,
            reason = "rule 11 — `unwrap_or_default` is the spelling this refuses; a dialect expressing nothing has a peak of zero by measurement"
        )]
        match self.expression.iter().copied().max() {
            Some(w) => w,
            None => 0,
        }
    }

    pub fn recollections(&self) -> Vec<(Vec<Vec<u8>>, u64)> {
        self.recollections
            .fiber(crate::network::nerve::ROOT)
            .map(|(_, (a, n))| (a.clone(), *n))
            .collect()
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
