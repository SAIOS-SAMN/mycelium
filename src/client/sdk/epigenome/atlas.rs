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

    /// Working atlas of genome + dialect. Ideas join at Entity::working_atlas.
    pub fn working_atlas(&self) -> WorkingAtlas<'_> {
        WorkingAtlas {
            genome: &self.atlas,
            dialect: &self.dialect_atlas,
            ideas: None,
        }
    }

    pub fn working_atlas_with<'a>(
        &'a self,
        ideas: &'a crate::network::nerve::Keyed<crate::network::nerve::Keyed<()>>,
    ) -> WorkingAtlas<'a> {
        WorkingAtlas {
            genome: &self.atlas,
            dialect: &self.dialect_atlas,
            ideas: Some(ideas),
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

    pub(crate) fn same_vertex_in_dialect(&self, e: &Element) -> Option<Vec<u8>> {
        let addr = e.address();
        let d = self.dialect.get(addr.as_bytes())?;
        debug_assert!(d.sentence.same_vertex(e));
        Some(addr.into_vec())
    }

    pub(crate) fn same_vertex_in_genome(&self, e: &Element) -> Option<usize> {
        let i = self.index.get(&e.address()).copied()?;
        debug_assert!(self.genome.get(i).is_some_and(|w| w.same_vertex(e)));
        Some(i)
    }

    /// Deconstruct a chain over the working language: genome, dialect, and
    /// extra words (ideas). A word this mind stood closes a remainder
    /// without waiting for the court.
    pub(crate) fn locate(
        &mut self,
        chain: &[(i64, i64, u8, u8)],
        exclude: Option<usize>,
    ) -> crate::transducer::sentence::Sentence {
        self.locate_on(chain, exclude, &[])
    }

    pub(crate) fn locate_on(
        &mut self,
        chain: &[(i64, i64, u8, u8)],
        exclude: Option<usize>,
        extra: &[Element],
    ) -> crate::transducer::sentence::Sentence {
        let n = self.genome.len();
        let dialect_addrs: Vec<Vec<u8>> = self
            .dialect
            .fiber(crate::network::nerve::ROOT)
            .map(|(a, _)| a)
            .collect();
        let n_d = dialect_addrs.len();
        let mut loc: crate::network::nerve::Keyed<usize> = crate::network::nerve::Keyed::new();
        for (j, a) in dialect_addrs.iter().enumerate() {
            loc.insert(a, n + j);
        }
        for (j, w) in extra.iter().enumerate() {
            loc.insert(w.address(), n + n_d + j);
        }
        let extra_shapes: Vec<Option<crate::transducer::blade::Shape>> = extra
            .iter()
            .map(|w| crate::transducer::element::shape_of_chain(w.sub()))
            .collect();
        let mut extra_atlas = crate::transducer::stalk::Stalks::default();
        for w in extra {
            extra_atlas.admit(&crate::transducer::stalk::invariant_of(w), w.address());
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
            } else if i < n + n_d {
                dialect_addrs.get(i - n).and_then(|a| dialect.get(a).map(|d| &d.sentence))
            } else {
                extra.get(i - n - n_d)
            }
        };
        let index_of = |a: &[u8]| -> Option<usize> {
            index.get(a).copied().or_else(|| loc.get(a).copied())
        };
        let shape_at = |i: usize| -> Option<&crate::transducer::blade::Shape> {
            if i < n {
                shapes.get(i).and_then(|c| c.get_or_init(|| crate::transducer::element::shape_of_chain(genome[i].sub())).as_ref())
            } else if i < n + n_d {
                dialect_addrs.get(i - n).and_then(|a| dialect.get(a).and_then(|d| d.shape.as_ref()))
            } else {
                extra_shapes.get(i - n - n_d).and_then(|s| s.as_ref())
            }
        };
        let fiber_of = |inv: &crate::transducer::stalk::Invariant| {
            let mut v = atlas.fiber(inv);
            for a in dialect_atlas.fiber(inv) {
                if !v.iter().any(|b| b == &a) {
                    v.push(a);
                }
            }
            for a in extra_atlas.fiber(inv) {
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
            },
            chain,
            exclude,
        )
    }

}
