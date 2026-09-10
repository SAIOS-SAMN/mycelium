// 24.2 — THE WORKING ATLAS: what this client holds as its own genome, and
// the index that reaches it by address.

use crate::client::sdk::*;
use crate::transducer::element::Element;

impl Epigenome {
    /// The held word at an address, read from the key.
    pub fn word_at(&self, addr: &[u8]) -> Option<&Element> {
        self.index.get(addr).and_then(|&i| self.genome.get(i))
    }

    /// Working-atlas root: concatenated A21 stratum roots of genome ∪ dialect.
    pub fn atlas_root(&self) -> crate::transducer::address::Address {
        let mut rows = crate::transducer::sheaf::digests(&self.atlas);
        rows.extend(crate::transducer::sheaf::digests(&self.dialect_atlas));
        rows.sort_by(|a, b| a.root.cmp(&b.root).then_with(|| a.inv.key().cmp(&b.inv.key())));
        let mut buf = Vec::new();
        for r in &rows {
            crate::transducer::address::put_bytes(&mut buf, &r.root);
        }
        crate::transducer::address::Address::new(buf)
    }

    /// The genome index of an address, read from the key.
    pub fn index_of(&self, addr: &[u8]) -> Option<usize> {
        self.index.get(addr).copied()
    }

    /// Re-key the genome after a removal shifted positions.
    pub(crate) fn reindex(&mut self) {
        self.index = crate::network::nerve::Keyed::new();
        for (i, w) in self.genome.iter().enumerate() {
            self.index.insert(w.address(), i);
        }
    }

    /// Deepen a held word's fiber with a section this genome verified.
    pub fn puncture(&mut self, i: usize, asg: Vec<(u8, u8)>) -> bool {
        self.genome.get_mut(i).is_some_and(|w| w.puncture(asg))
    }

    pub fn new() -> Self {
        Epigenome::default()
    }

    pub(crate) fn axis_admit(&mut self, e: &Element) {
        let inv = crate::transducer::stalk::invariant_of(e);
        crate::transducer::sentence::admit_words(
            &mut self.axis_index,
            &inv,
            &crate::transducer::sentence::words_of_element(e),
        );
    }

    pub(crate) fn axis_retract(&mut self, e: &Element) {
        let inv = crate::transducer::stalk::invariant_of(e);
        crate::transducer::sentence::retract_words(
            &mut self.axis_index,
            &inv,
            &crate::transducer::sentence::words_of_element(e),
        );
    }

    /// File a closed form under the chart of every body its words carry
    /// (21 step 3): what the glue asks the crystal for. A form that is not
    /// closed is not filed — 13.6 lets a citer reuse a cited form's verdict
    /// only because the cited form is closed, and `Form::glue` refuses an
    /// unclosed one.
    pub(crate) fn moduli_admit(&mut self, e: &Element) {
        let addr = e.address();
        for chart in body_charts_of(e) {
            self.by_moduli.nested_insert(chart.key(), &addr, ());
        }
        // 1.8: the form's word vertices are the generators it contributes
        // to this mind's presentation.
        if let Some(f) = crate::transducer::sentence::from_element_form(e) {
            crate::transducer::sentence::admit_generators(&mut self.generators, &addr, &f);
            // 37.7: a generator the alphabet cannot spell, learned. Read
            // from the vertex — cites nothing and A22 cannot spell its
            // action — never from a flag on the tape.
            for v in &f.vertices {
                let crate::transducer::sentence::Vertex::Word { derivation, .. } = v else {
                    continue;
                };
                if !derivation.generators.is_empty() {
                    continue;
                }
                let spelled =
                    crate::transducer::alphabet::factor_rotor(&derivation.rotor());
                if !matches!(spelled, Some(Ok(_))) {
                    eprintln!("  letter learned {:02x}{:02x}…", addr[0], addr[1]);
                }
            }
        }
    }

    /// Empty what `moduli_admit` filled: a word that sheds, is retired, or
    /// is promoted leaves the index with the stratum it left.
    pub(crate) fn moduli_retract(&mut self, e: &Element) {
        let addr = e.address();
        for chart in body_charts_of(e) {
            if let Some(bucket) = self.by_moduli.get_mut(chart.key().as_bytes()) {
                bucket.remove(&addr);
            }
        }
        if let Some(f) = crate::transducer::sentence::from_element_form(e) {
            crate::transducer::sentence::retract_generators(&mut self.generators, &addr, &f);
        }
    }

    /// The standing of a held address, read in the strata's own order
    /// (24.1): Ruled, then Stood. A word this epigenome does not hold has
    /// no standing here.
    pub(crate) fn standing_of(&self, addr: &[u8]) -> Option<Standing> {
        if self.index.contains(addr) {
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
/// here and no cell is read.
pub(crate) fn body_charts_of(e: &Element) -> Vec<crate::transducer::stalk::Invariant> {
    let Some(f) = crate::transducer::sentence::from_element_form(e) else {
        return Vec::new();
    };
    if !f.is_closed() {
        return Vec::new();
    }
    let mut charts = Vec::new();
    let mut seen: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    for v in &f.vertices {
        let crate::transducer::sentence::Vertex::Word { class, .. } = v else {
            continue;
        };
        let k = class.key();
        if seen.contains(k.as_bytes()) {
            continue;
        }
        seen.insert(k.as_bytes(), ());
        charts.push(class.clone());
    }
    charts
}

impl Epigenome {

    pub fn genome_len(&self) -> usize {
        self.genome.len()
    }

    pub fn genome(&self) -> &[Element] {
        &self.genome
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

    /// A word on the working language: genome first, then dialect.
    pub fn working_word(&self, addr: &[u8]) -> Option<&Element> {
        self.word_at(addr)
            .or_else(|| self.dialect.get(addr).map(|d| &d.sentence))
    }

    pub fn take_promoted(&mut self) -> Vec<Vec<u8>> {
        std::mem::take(&mut self.promoted)
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
    pub(crate) fn same_vertex_in_genome(&self, e: &Element) -> Option<usize> {
        let i = self.index.get(&e.address()).copied()?;
        if !self.genome.get(i).is_some_and(|w| w.same_vertex(e)) {
            return None;
        }
        Some(i)
    }

    /// Deconstruct a chain over the working language: genome and dialect,
    /// with no Held stratum. A word this mind stood closes a remainder
    /// without waiting for the court.
    pub(crate) fn locate(
        &mut self,
        chain: &[(i64, i64, u8, u8)],
        exclude: Option<usize>,
    ) -> crate::transducer::sentence::Sentence {
        self.locate_on(chain, exclude, &crate::transducer::sentence::no_held)
    }

    /// Deconstruct a chain over the working language with the Held stratum
    /// the caller reads at each body's chart (24.1 as ruled 2026-09-09):
    /// `held_at` is the contemplation's forms evaluated on the body, never
    /// a list of elements joined to the snapshot.
    pub(crate) fn locate_on<H>(
        &mut self,
        chain: &[(i64, i64, u8, u8)],
        exclude: Option<usize>,
        held_at: &H,
    ) -> crate::transducer::sentence::Sentence
    where
        H: Fn(&crate::transducer::stalk::Invariant, &[(i64, i64, u8, u8)]) -> Vec<crate::transducer::sentence::HeldCite>,
    {
        let n = self.genome.len();
        let dialect_addrs: Vec<Vec<u8>> = self
            .dialect
            .fiber(crate::network::nerve::ROOT)
            .map(|(a, _)| a)
            .collect();
        let mut loc: crate::network::nerve::Keyed<usize> = crate::network::nerve::Keyed::new();
        for (j, a) in dialect_addrs.iter().enumerate() {
            loc.insert(a, n + j);
        }
        let genome = &self.genome;
        let dialect = &self.dialect;
        let atlas = &self.atlas;
        let dialect_atlas = &self.dialect_atlas;
        let index = &self.index;
        let shapes = &self.shapes;
        let word_at = |i: usize| -> Option<&Element> {
            if i < n {
                genome.get(i)
            } else {
                dialect_addrs.get(i - n).and_then(|a| dialect.get(a).map(|d| &d.sentence))
            }
        };
        let index_of = |a: &[u8]| -> Option<usize> {
            index.get(a).copied().or_else(|| loc.get(a).copied())
        };
        let shape_at = |i: usize| -> Option<&crate::transducer::blade::Shape> {
            if i < n {
                shapes.get(i).and_then(|c| c.get_or_init(|| crate::transducer::element::shape_of_chain(genome[i].sub())).as_ref())
            } else {
                dialect_addrs.get(i - n).and_then(|a| dialect.get(a).and_then(|d| d.shape.as_ref()))
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
                index_of: &index_of,
                shape_at: &shape_at,
                held_at,
            },
            chain,
            exclude,
        )
    }

}
