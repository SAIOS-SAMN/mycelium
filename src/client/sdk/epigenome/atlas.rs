// 24.2 — THE WORKING ATLAS: what this client holds as its own genome, and
// the index that reaches it by address.

use crate::client::sdk::*;
use crate::transducer::element::{Element, Held};

impl Epigenome {
    /// The held word at an address, read from the key. A `Held`, so its
    /// form is a field and not a decode this read performs (L16); it
    /// derefs to the element, so every element reader takes it unchanged.
    /// THE HELD WORD AT AN ADDRESS. Under Z11 the index says whether the
    /// genome HOLDS a word (`holds`) and this says whether it is decoded
    /// and here; today every held word is resident, so the two agree.
    pub fn word_at(&self, addr: &[u8]) -> Option<&Held> {
        self.resident.get(addr).map(|w| &w.held)
    }

    /// WHETHER THE GENOME HOLDS A WORD, asked of the INDEX — which is the
    /// question, and which stays true under Z11 whether or not the word is
    /// decoded. Every reader that means "does this mind know it" asks
    /// here; a reader that means "can I read its form now" asks `word_at`.
    pub fn holds(&self, addr: &[u8]) -> bool {
        self.genome.contains(addr)
    }

    /// Working-atlas root: concatenated A21 stratum roots of genome ∪ dialect.
    pub fn atlas_root(&self) -> crate::transducer::address::Address {
        let mut rows = crate::transducer::sheaf::digests(&self.atlas);
        rows.extend(crate::transducer::sheaf::digests(&self.dialect_atlas));
        #[expect(
            clippy::disallowed_methods,
            reason = "rule 4 — canonical key order: the stratum roots and then the chart keys, so two minds holding one atlas write one root"
        )]
        rows.sort_by(|a, b| a.root.cmp(&b.root).then_with(|| a.inv.key().cmp(&b.inv.key())));
        let mut buf = Vec::new();
        for r in &rows {
            crate::transducer::address::put_key_bytes(&mut buf, &r.root);
        }
        crate::transducer::address::Address::new(buf)
    }

    /// This word's shape, lifted once under the word it measures.
    pub fn shape_at(&self, addr: &[u8]) -> Option<&crate::transducer::blade::Shape> {
        let w = self.resident.get(addr)?;
        w.shape
            .get_or_init(|| crate::transducer::element::shape_of_chain(w.held.sub()))
            .as_ref()
    }

    /// Deepen a held word's fiber with a section this genome verified.
    /// Through `Held::update`, so the form is re-decoded with the word and
    /// the two cannot describe different tapes.
    ///
    /// A PUNCTURE CHANGES THE TAPE AND THEREFORE THE ADDRESS, so the entry
    /// is re-filed at the coordinate it now derives for itself rather than
    /// left sitting under the coordinate it used to have. Under the old
    /// parallel vectors the word stayed at its slot and `index` silently
    /// went stale; law 2 does not allow a key that no longer names its
    /// value.
    ///
    /// AND A WORD AT A NEW ADDRESS IS A NEW WORD: no act admitted it. The
    /// punctured word's address is answered so the caller admits it by an
    /// act on its own line (LD3-c) — `None` where nothing was punctured.
    pub fn puncture(&mut self, addr: &[u8], asg: Vec<(u8, u8)>) -> Option<crate::transducer::address::Address> {
        let entry = self.genome.remove(addr)?;
        let Some(mut w) = self.resident.remove(addr) else {
            // The index holds it and the payload does not: under Z11 that
            // is a word on disk and not decoded, and a puncture of a word
            // this mind has not read is refused rather than guessed.
            self.genome.insert(addr, entry);
            return None;
        };
        if !w.held.update(|e| e.puncture(asg)) {
            self.genome.insert(addr, entry);
            self.resident.insert(addr, w);
            return None;
        }
        // The shape was measured on the old chain and is not the new
        // word's; it is dropped rather than carried across (law 12).
        w.shape = std::sync::OnceLock::new();
        // RM1 — the old address's remainders leave the index.
        self.drop_remainders(addr);
        let at = w.held.address();
        self.genome.insert(&at, entry);
        self.resident.insert(&at, w);
        Some(at)
    }

    pub fn new() -> Self {
        Epigenome::default()
    }

    /// A WORD IS FILED WHERE IT ACTS, and the form is read off the `Held`
    /// rather than decoded again (L16). A sentence files each Word vertex's
    /// word under every chart that vertex acts on — the same charts
    /// `candidates_for` asks under and `body_charts_of` uses for
    /// `by_moduli`. A connection carries no form and files under its own
    /// invariant, which for it is `connection_chart`, the rotor's own axes.
    pub(crate) fn axis_admit(&mut self, e: &Held) {
        if let Some(f) = e.form() {
            crate::transducer::sentence::admit_words_acting(&mut self.axis_index, f);
            return;
        }
        let inv = crate::transducer::stalk::invariant_of(e);
        crate::transducer::sentence::admit_words(
            &mut self.axis_index,
            &inv,
            &crate::transducer::sentence::acting_of_element(e),
        );
    }

    pub(crate) fn axis_retract(&mut self, e: &Held) {
        if let Some(f) = e.form() {
            crate::transducer::sentence::retract_words_acting(&mut self.axis_index, f);
            return;
        }
        let inv = crate::transducer::stalk::invariant_of(e);
        crate::transducer::sentence::retract_words(
            &mut self.axis_index,
            &inv,
            &crate::transducer::sentence::acting_of_element(e),
        );
    }

    /// File a closed form under the chart of every body its words carry
    /// (21 step 3): what the glue asks the crystal for. A form that is not
    /// closed is not filed — 13.6 lets a citer reuse a cited form's verdict
    /// only because the cited form is closed, and `Form::glue` refuses an
    /// unclosed one.
    pub(crate) fn moduli_admit(&mut self, e: &Held) {
        let addr = e.address();
        for chart in body_charts_of(e) {
            self.by_moduli.nested_insert(chart.key(), &addr, ());
        }
        for (key, vertex) in act_keys_of(e) {
            let at = crate::transducer::sentence::generator_key(&addr, &vertex);
            self.by_act.nested_insert(key, at, (addr.clone(), vertex));
        }
        // 1.8: the form's word vertices are the generators it contributes
        // to this mind's presentation.
        if let Some(f) = e.form().cloned() {
            crate::transducer::sentence::admit_generators(&mut self.generators, &addr, &f);
            crate::transducer::sentence::admit_generators_by_lane(&mut self.lanes, &addr, &f);
        }
    }

    /// Empty what `moduli_admit` filled: a word that sheds, is retired, or
    /// is promoted leaves the index with the stratum it left.
    pub(crate) fn moduli_retract(&mut self, e: &Held) {
        let addr = e.address();
        for chart in body_charts_of(e) {
            if let Some(bucket) = self.by_moduli.get_mut(chart.key().as_bytes()) {
                bucket.remove(&addr);
            }
        }
        for (key, vertex) in act_keys_of(e) {
            let empty = match self.by_act.get_mut(&key) {
                Some(bucket) => {
                    bucket.remove(&crate::transducer::sentence::generator_key(&addr, &vertex));
                    bucket.is_empty()
                }
                None => false,
            };
            if empty {
                self.by_act.remove(&key);
            }
        }
        if let Some(f) = e.form() {
            crate::transducer::sentence::retract_generators(&mut self.generators, &addr, f);
            crate::transducer::sentence::retract_generators_by_lane(&mut self.lanes, &addr, f);
        }
    }

    /// The standing of a held address, read in the strata's own order
    /// (24.1): Ruled, then Stood. A word this epigenome does not hold has
    /// no standing here.
    pub fn standing_of(&self, addr: &[u8]) -> Option<Standing> {
        if self.genome.contains(addr) {
            return Some(Standing::Ruled);
        }
        if self.dialect.contains(addr) {
            return Some(Standing::Stood);
        }
        None
    }
}

/// The charts of the bodies a closed sentence's word vertices carry. A
/// `Vertex::Word` states the chart it acts on (18.2); nothing is measured
/// here and no painted site is read.
pub(crate) fn body_charts_of(e: &Held) -> Vec<crate::transducer::stalk::Invariant> {
    let Some(f) = e.form() else {
        return Vec::new();
    };
    if !f.is_closed() {
        return Vec::new();
    }
    let mut charts = Vec::new();
    let mut seen: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    // EVERY CHART EVERY WORD VERTEX ACTS ON. A form's charts are what its
    // words speak to, and a vertex acting on two charts speaks to bodies on
    // both — listing one left the other unnamed by the atlas, so a cite at that
    // chart found nothing the form holds.
    for (_, v) in f.vertices.scoped() {
        let crate::transducer::sentence::Vertex::Word { .. } = v else {
            continue;
        };
        for class in v.charts() {
            let k = class.key();
            if seen.contains(k.as_bytes()) {
                continue;
            }
            seen.insert(k.as_bytes(), ());
            charts.push(class);
        }
    }
    charts
}

/// What every member of a closed sentence's word vertices does, as the
/// coordinates the glue asks at (`presentation::act_key`), each with the
/// word vertex whose member acts so.
pub(crate) fn act_keys_of(e: &Held) -> Vec<(Vec<u8>, crate::transducer::address::Address)> {
    let Some(f) = e.form().filter(|f| f.is_closed()) else {
        return Vec::new();
    };
    let mut keys = Vec::new();
    for (_, v) in f.vertices.scoped() {
        let crate::transducer::sentence::Vertex::Word { address, derivation, acts, .. } = v else { continue };
        for (_, a) in acts.scoped() {
            if let Some(k) =
                crate::transducer::sentence::presentation::act_key(&a.chart, &derivation.rotor(), &a.carriage, &a.gauge)
            {
                keys.push((k, address.clone()));
            }
        }
    }
    keys
}

impl Epigenome {

    pub fn genome_len(&self) -> usize {
        self.genome.len()
    }

    /// Every genome word, in the trie's order — which is the addresses'
    /// own order and is content (law 2), not arrival order.
    pub fn genome_words(&self) -> impl Iterator<Item = &Held> {
        self.resident.scoped().map(|(_, w)| &w.held)
    }

    /// THE WORDS THE PIVOT HAS NOT REACHED, taken and cleared. Their
    /// coordinates, not their arrival: a word is here because its
    /// obstruction is undischarged, and it leaves when the pivot takes it.
    ///
    /// This replaced `genome_since(seq)` — "every word admitted at or
    /// after an ordinal" — which answered a question about history when
    /// the pivot's question is about standing work.
    pub fn take_unpivoted(&mut self) -> Vec<Element> {
        let at: Vec<Vec<u8>> = self.unpivoted.scoped().map(|(k, ())| k).collect();
        self.unpivoted = crate::network::nerve::Keyed::new();
        at.iter()
            .filter_map(|k| self.resident.get(k).map(|w| w.held.element().clone()))
            .collect()
    }

    /// How many coordinates the pivot has not reached.
    pub fn unpivoted_len(&self) -> usize {
        self.unpivoted.len()
    }

    /// THE CIRCUIT FROM A COORDINATE: every genome address once, starting
    /// after `at` and wrapping to close on it (45.7). This is what a traversal
    /// `for offset in 0..len { (start + offset) % len }` was reaching for
    /// — with the difference that a circuit over coordinates does not
    /// renumber itself when a word is admitted or retired mid-circuit.
    pub fn genome_circuit_from(&self, at: Option<&[u8]>) -> Vec<Vec<u8>> {
        let all: Vec<Vec<u8>> = self.genome.scoped().map(|(k, _)| k).collect();
        let Some(at) = at else { return all };
        match all.iter().position(|k| k.as_slice() == at) {
            // The cursor's word has retired; the circuit starts where the
            // carrier now starts rather than at an invented offset.
            None => all,
            Some(i) => {
                let mut out: Vec<Vec<u8>> = all[i + 1..].to_vec();
                out.extend_from_slice(&all[..=i]);
                out
            }
        }
    }

    pub fn next_word_after(&self, at: Option<&[u8]>) -> Option<Vec<u8>> {
        let mut past = at.is_none();
        let mut first: Option<Vec<u8>> = None;
        for (k, _) in self.genome.scoped() {
            if first.is_none() {
                first = Some(k.clone());
            }
            if past {
                return Some(k);
            }
            if at == Some(k.as_slice()) {
                past = true;
            }
        }
        // Past the last coordinate, or the cursor named a word that has
        // since retired: the circuit closes on the first.
        first
    }

    /// RETIRE a word whose meaning the rest of the genome already carries
    /// (the accepted compression bid). Expression rides along (parallel
    /// vector); recollections citing the retired index dissolve — they were
    /// spellings in the old vocabulary — and surviving citations reindex.
    /// The genome's atlas, admitted in place. Persistence and crystallize
    /// only. Cognitive reads use [`working_atlas`].
    pub fn atlas(&self) -> &crate::transducer::stalk::Stalks {
        &self.atlas
    }

    /// Empty when the working language has no genome word and no dialect word.
    pub fn atlas_is_empty(&self) -> bool {
        self.atlas_len == 0 && self.dialect.is_empty()
    }

    /// Working atlas of genome + dialect. The held forms join at
    /// `Entity::working_atlas`, read from the contemplation (24.1).
    pub fn working_atlas(&self) -> WorkingAtlas<'_> {
        WorkingAtlas {
            genome: &self.atlas,
            dialect: &self.dialect_atlas,
            held: None,
        }
    }

    pub fn working_atlas_with<'a>(
        &'a self,
        held: &'a crate::transducer::bubble::Contemplation,
    ) -> WorkingAtlas<'a> {
        WorkingAtlas {
            genome: &self.atlas,
            dialect: &self.dialect_atlas,
            held: Some(held),
        }
    }

    /// A word on the working language: genome first, then dialect. Both
    /// strata hold a `Held`, so the form is read and never decoded here.
    pub fn working_word(&self, addr: &[u8]) -> Option<&Held> {
        self.word_at(addr)
            .or_else(|| self.dialect.get(addr).map(|d| &d.sentence))
    }

    /// THE ADDRESSES STANDING ON ONE CHART (21 step 3) — the stratum, as
    /// coordinates and nothing more.
    ///
    /// `Crystal::on_body` answers the same stratum with every element and
    /// every form in it CLONED. A caller that means to speak one of them
    /// and stop at the first that answers pays for all of them to find
    /// out, and a caller that asks once per demand of a served page pays
    /// that per demand: the page's charts times the words on each, with a
    /// whole `Form` copied at every crossing. Thirty-two bytes per member
    /// is the read; the form is fetched by key when it is about to be
    /// spoken.
    pub fn chart_stratum(&self, chart: &crate::transducer::stalk::Invariant) -> Vec<Vec<u8>> {
        let Some(bucket) = self.by_moduli.get(chart.key().as_bytes()) else {
            return Vec::new();
        };
        // `by_moduli.get(chart)` is the read by the key: the addresses on that
        // chart, which is what this function is asked for.
        bucket.scoped().map(|(a, ())| a).collect()
    }

    pub fn take_promoted(&mut self) -> Vec<Vec<u8>> {
        std::mem::take(&mut self.promoted)
    }

    /// Size of the axis index: charts filed, and total word entries filed across all charts and axes.
    pub fn axis_index_counts(&self) -> (usize, usize) {
        let charts = self.axis_index.len();
        let words: usize = self
            .axis_index
            .scoped()
            .map(|(_, axes)| axes.scoped().map(|(_, w)| w.len()).sum::<usize>())
            .sum();
        (charts, words)
    }

    /// The dialect entry at this form's address, WHEN IT IS THIS VERTEX.
    ///
    /// The name says `same_vertex` and the re-derivation that proves it
    /// stood in a `debug_assert!`, which is nothing in a release build: in
    /// the build the mesh runs, this answered "yes, the same vertex" on
    /// nothing but an address match. Law 2 is that identity is CONTENT and
    /// an address is a coordinate on it; `same_vertex` is M1's own
    /// re-derivation and `element/vertex.rs` says beside it that two forms
    /// agreeing on an address and not on a vertex is the case it exists
    /// for. A caller then took a held word for the one it was holding.
    pub(crate) fn same_vertex_in_dialect(&self, e: &Element) -> Option<Vec<u8>> {
        let addr = e.address();
        let d = self.dialect.get(addr.as_bytes())?;
        if !d.sentence.same_vertex(e) {
            return None;
        }
        Some(addr.into_vec())
    }

    /// The genome slot at this form's address, WHEN IT IS THIS VERTEX.
    /// The same debug-only re-derivation, and the index can also LAG the
    /// carrier: a slot that names a word the genome has since moved is not
    /// a smaller answer, it is no answer (rule 11).
    pub(crate) fn same_vertex_in_genome(&self, e: &Element) -> Option<Vec<u8>> {
        let addr = e.address();
        if !self.resident.get(addr.as_bytes()).is_some_and(|w| w.held.same_vertex(e)) {
            return None;
        }
        Some(addr.into_vec())
    }

    /// Deconstruct a chain over the working language: genome and dialect,
    /// with no Held stratum. A word this mind stood closes a remainder
    /// without waiting for the court.
    pub(crate) fn locate(
        &mut self,
        chain: &[(i64, i64, u8, u8)],
        exclude: Option<&[u8]>,
        budget: &mut crate::physics::Budget,
    ) -> crate::transducer::sentence::Sentence {
        self.locate_on(chain, exclude, &crate::transducer::sentence::no_held, budget)
    }

    /// Deconstruct a chain over the working language with the Held stratum
    /// the caller reads at each body's chart (24.1 as ruled 2026-09-09):
    /// `held_at` is the contemplation's forms evaluated on the body, never
    /// a list of elements joined to the snapshot.
    ///
    /// THERE IS NO SNAPSHOT AND NO OFFSET SPACE. It built `dialect_addrs`
    /// and a `loc: Keyed<usize>` on EVERY call so the dialect's words could
    /// be given offsets `n..` above the genome's `0..n`, and the reading
    /// then converted each address the atlas handed it into one of those
    /// offsets and straight back into a word. Both strata are keyed by
    /// address already, so the reader tries the genome and then the dialect
    /// at the coordinate and there is nothing to number.
    pub(crate) fn locate_on<H>(
        &mut self,
        chain: &[(i64, i64, u8, u8)],
        exclude: Option<&[u8]>,
        held_at: &H,
        budget: &mut crate::physics::Budget,
    ) -> crate::transducer::sentence::Sentence
    where
        H: Fn(
            &crate::transducer::stalk::Invariant,
            &[(i64, i64, u8, u8)],
            &mut crate::physics::Budget,
        ) -> Vec<crate::transducer::sentence::HeldCite>,
    {
        // THE READING RUNS OVER WHAT IS DECODED. Under Z11 the fiber read
        // decodes what it reaches before the closures are built; today
        // every word the index holds is resident and the two are the same
        // set.
        let genome = &self.resident;
        let dialect = &self.dialect;
        let atlas = &self.atlas;
        let dialect_atlas = &self.dialect_atlas;
        // RULED FIRST, THEN STOOD (24.1), read at the coordinate in that
        // order — the same precedence the offset ranges used to encode.
        let word_at = |a: &[u8]| -> Option<&Element> {
            genome
                .get(a)
                .map(|w| w.held.element())
                .or_else(|| dialect.get(a).map(|d| d.sentence.element()))
        };
        let shape_at = |a: &[u8]| -> Option<&crate::transducer::blade::Shape> {
            match genome.get(a) {
                Some(w) => w
                    .shape
                    .get_or_init(|| crate::transducer::element::shape_of_chain(w.held.sub()))
                    .as_ref(),
                None => dialect.get(a).and_then(|d| d.shape.as_ref()),
            }
        };
        let fiber_of = |inv: &crate::transducer::stalk::Invariant| {
            let mut v = atlas.fiber(inv);
            for a in dialect_atlas.fiber(inv) {
                if !v.iter().any(|b| b == &a) {
                    v.push(a);
                }
            }
            v
        };
        crate::transducer::sentence::deconstruct_on(
            &crate::transducer::sentence::Reading {
                word_at: &word_at,
                fiber_of: &fiber_of,
                shape_at: &shape_at,
                held_at,
            },
            chain,
            exclude,
            budget,
        )
    }

}
