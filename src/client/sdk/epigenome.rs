// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! The epigenome: the working atlas, comprehension, the dialect
//! lifecycle, and the Crystal it presents to the extractor.
//!
//! Split from sdk.rs under Stage 8's reading bound. Whole `impl` blocks
//! move verbatim; a private field is visible to its module and its
//! descendants, so the carriers are reached unchanged.

use super::*;

impl Epigenome {
    /// The held word at an address, read from the key.
    pub fn word_at(&self, addr: &[u8]) -> Option<&Element> {
        self.index.get(addr).and_then(|&i| self.genome.get(i))
    }

    /// Working-atlas root: concatenated A21 stratum roots of genome ∪ dialect.
    pub fn atlas_root(&self) -> crate::transducer::a21::Address {
        let mut rows = crate::transducer::sheaf::digests(&self.atlas);
        rows.extend(crate::transducer::sheaf::digests(&self.dialect_atlas));
        rows.sort_by(|a, b| a.root.cmp(&b.root).then_with(|| a.inv.key().cmp(&b.inv.key())));
        let mut buf = Vec::new();
        for r in &rows {
            crate::transducer::a21::put_bytes(&mut buf, &r.root);
        }
        crate::transducer::a21::Address::new(buf)
    }

    /// The genome index of an address, read from the key.
    pub fn index_of(&self, addr: &[u8]) -> Option<usize> {
        self.index.get(addr).copied()
    }

    /// Re-key the genome after a removal shifted positions.
    fn reindex(&mut self) {
        self.index = crate::network::nerve::Keyed::new();
        for (i, w) in self.genome.iter().enumerate() {
            self.index.insert(w.address(), i);
        }
    }

    /// Deepen a held word's fiber with a section this genome verified.
    pub fn puncture(&mut self, i: usize, asg: Vec<(u8, u8)>) -> bool {
        self.genome.get_mut(i).is_some_and(|w| w.puncture(asg))
    }

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

    pub fn shed_n(&self) -> u64 {
        self.shed_n
    }

    /// A working-atlas read of a stood word is a use, stamped with the
    /// mind's OWN K (45.4). The height beside it is standing extent and is
    /// left alone; the lifecycle is the mind's, and a mind that hears
    /// nothing still thinks.
    pub fn touch(&mut self, addr: &[u8], now_k: u64) {
        if let Some(d) = self.dialect.get_mut(addr) {
            d.last_used_k = now_k;
            d.uses = d.uses.saturating_add(1);
        }
    }

    /// Drop dialect unused for an aperture of the MIND'S OWN K (45.4: the
    /// mind sheds by its own receipt index, never by the mesh's height). A
    /// dialect entry a live idea glues is not shed. The genome never sheds.
    ///
    /// Read against TUniversal this could not fire at all on a mesh that
    /// admitted nothing: heights advance only on admitted receipts (45.2),
    /// so a fleet refusing work also could not reclaim what it held, and the
    /// two failures compounded into a client the kernel killed. A mind's own
    /// K advances on its own acts, so truncation is independent of whether
    /// anyone answered it.
    pub fn shed(&mut self, now_k: u64) -> usize {
        self.shed_except(now_k, &|_| false)
    }

    pub fn note_stroke(&mut self, now_k: u64) {
        let n = crate::physics::aperture_cycle().max(1) as usize;
        self.stroke_k.push(now_k);
        if self.stroke_k.len() > n {
            self.stroke_k.remove(0);
        }
    }

    /// TUniversal span of the last aperture of strokes. One unit, measured.
    /// `None` until a whole aperture of strokes has been noted: an
    /// unmeasured aperture is held, never guessed (rule 15), so nothing
    /// sheds in the first aperture after boot.
    fn aperture_k(&self, now_k: u64) -> Option<u64> {
        let n = crate::physics::aperture_cycle().max(1) as usize;
        if self.stroke_k.len() < n {
            return None;
        }
        let first = *self.stroke_k.first()?;
        Some(now_k.saturating_sub(first))
    }

    pub fn shed_except(&mut self, now_k: u64, glued: &impl Fn(&[u8]) -> bool) -> usize {
        let Some(aperture_k) = self.aperture_k(now_k) else {
            return 0;
        };
        let drop: Vec<Vec<u8>> = self
            .dialect
            .fiber(crate::network::nerve::ROOT)
            .filter(|(a, d)| now_k.saturating_sub(d.last_used_k) > aperture_k && !glued(a))
            .map(|(a, _)| a)
            .collect();
        let n = drop.len();
        for a in &drop {
            self.drop_dialect(a);
        }
        self.shed_n = self.shed_n.saturating_add(n as u64);
        n
    }

    fn wait_on(&mut self, missing: &[u8], citer: impl AsRef<[u8]>) {
        let citer = citer.as_ref();
        if let Some(citers) = self.waiting_on.get_mut(missing) {
            citers.insert(citer, ());
            return;
        }
        let mut citers = crate::network::nerve::Keyed::new();
        citers.insert(citer, ());
        self.waiting_on.insert(missing, citers);
    }

    fn retract_waiter(&mut self, missing: &[u8], citer: &[u8]) {
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

    fn drop_dialect(&mut self, da: &[u8]) {
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
        self.axis_admit(&sentence);
        let shape = crate::transducer::element::shape_of_chain(sentence.sub());
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
        if let [] = residual {
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
        let word = crate::transducer::element::from_residual(&residual.to_vec());
        self.expressed_atlas.admit(&crate::transducer::stalk::invariant_of(&word), word.address());
        self.expressed_index.insert(word.address(), self.expressed.len());
        self.expressed_shapes.push(std::sync::OnceLock::new());
        self.stand_dialect_at(word.clone(), k, now_tu);
        self.expressed.push(word);
        true
    }

    pub fn new() -> Self {
        Epigenome::default()
    }

    fn axis_admit(&mut self, e: &Element) {
        let inv = crate::transducer::stalk::invariant_of(e);
        crate::transducer::sentence::admit_words(
            &mut self.axis_index,
            &inv,
            &crate::transducer::sentence::words_of_element(e),
        );
    }

    fn axis_retract(&mut self, e: &Element) {
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
    /// The strongest-expressed word's weight (the dialect's leading edge).
    pub fn peak_expression(&self) -> u64 {
        self.expression.iter().copied().max().unwrap_or(0)
    }
    pub fn recollections(&self) -> Vec<(Vec<Vec<u8>>, u64)> {
        self.recollections
            .fiber(crate::network::nerve::ROOT)
            .map(|(_, (a, n))| (a.clone(), *n))
            .collect()
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

    pub(super) fn same_vertex_in_dialect(&self, e: &Element) -> Option<Vec<u8>> {
        let addr = e.address();
        let d = self.dialect.get(addr.as_bytes())?;
        debug_assert!(d.sentence.same_vertex(e));
        Some(addr.into_vec())
    }

    pub(super) fn same_vertex_in_genome(&self, e: &Element) -> Option<usize> {
        let i = self.index.get(&e.address()).copied()?;
        debug_assert!(self.genome.get(i).is_some_and(|w| w.same_vertex(e)));
        Some(i)
    }

    /// Deconstruct a chain over the working language: genome, dialect, and
    /// extra words (ideas). A word this mind stood closes a remainder
    /// without waiting for the court.
    pub(super) fn locate(
        &mut self,
        chain: &[(i64, i64, u8, u8)],
        exclude: Option<usize>,
    ) -> crate::transducer::sentence::Sentence {
        self.locate_on(chain, exclude, &[])
    }

    pub(super) fn locate_on(
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
        crate::transducer::sentence::deconstruct_on(&word_at, &fiber_of, &index_of, &shape_at, chain, exclude)
    }

    pub fn retire(&mut self, addr: &[u8]) -> bool {
        let Some(i) = self.index.get(addr).copied() else {
            return false;
        };
        let gone = self.genome[i].address();
        let held = self.genome[i].clone();
        self.axis_retract(&held);
        self.genome.remove(i);
        if i < self.expression.len() {
            self.expression.remove(i);
        }
        let drop: Vec<Vec<u8>> = self
            .recollections
            .fiber(crate::network::nerve::ROOT)
            .filter(|(_, (addrs, _))| addrs.iter().any(|a| a.as_slice() == gone.as_bytes()))
            .map(|(k, _)| k)
            .collect();
        for k in drop {
            self.recollections.remove(&k);
        }
        self.atlas.remove(&gone);
        if i < self.shapes.len() {
            self.shapes.remove(i);
        }
        self.atlas_len = self.genome.len();
        self.reindex();
        true
    }

    /// COMPREHEND a received utterance: decompose it over the genome (the shared
    /// projective mathematics, `cognition::decompose_over`) and fold the
    /// understanding into the epigenome — expression weights rise on the cited
    /// words, the sentence joins the recollections. The genome is untouched.
    pub fn comprehend(&mut self, utterance: &Element) -> Comprehension {
        // A taught sentence is re-derived as a form: vertices, faces, loops.
        // Understanding is re-deriving its cochain, not matching its bytes.
        if utterance.is_sentence() {
            if let Some(f) = crate::transducer::sentence::from_element_form(utterance) {
                return self.comprehend_form(&f);
            }
            return Comprehension::Novel { residual_cells: utterance.sub().len() };
        }
        // HEAR g BY HOLDING IT: a connection has no chain to cover. It is
        // glued against the representative on its fibre — the transition
        // vanishes (held), does not (dialect, the element carried), or there
        // is no representative (admitted). Never an empty covering.
        if utterance.is_translation() {
            return Comprehension::Admitted;
        }
        if utterance.is_connection() && utterance.sub().is_empty() {
            let inv = crate::transducer::stalk::invariant_of(utterance);
            let fibre = self.working_atlas().fiber(&inv);
            // M6 STEP 3, A12: the incoming word is factored through the
            // WHOLE fiber, not tested against one representative at a time.
            // A mind speaks what it can re-derive from what it holds
            // (chapter 6), and a word may be a product of several held words
            // while matching none of them — the pairwise transition test
            // below could only ever find it in one.
            if let Some(spoken) = utterance.letters() {
                let mut words = Vec::new();
                let mut basis = Vec::new();
                for (a, _) in &fibre {
                    let Some(held) = self.working_word(a) else { continue };
                    if held.address() == utterance.address() {
                        continue;
                    }
                    let Some(l) = held.letters() else { continue };
                    words.push(l.spelling.word.clone());
                    basis.push(l.spelling.word.rotor());
                }
                let g = spoken.spelling.word.rotor();
                if let (Some(gf), true) =
                    (g.spelled_word().and_then(|w| w.form()), !basis.is_empty())
                {
                    let forms: Vec<crate::transducer::alphabet::PlaneForm> = words
                        .iter()
                        .filter_map(|w| w.form())
                        .collect();
                    if forms.len() == words.len() {
                        if let crate::transducer::factor::Factored::Understood(tape) =
                            crate::transducer::factor::factor_on_plane(&gf, &forms)
                        {
                            // Verified by re-derivation, never trusted from
                            // the algebra that produced it (16.12 step 4).
                            if crate::transducer::factor::replays(&tape, &basis, &g) {
                                return Comprehension::Held;
                            }
                        }
                    }
                }
            }
            let mut dialect: Option<(crate::transducer::a21::Address, Element)> = None;
            for (a, _) in fibre {
                let Some(held) = self.working_word(&a).cloned() else {
                    continue;
                };
                if held.address() == utterance.address() || held.axes().is_empty() {
                    continue;
                }
                match crate::transducer::cech::transition(&held, utterance) {
                    None => {}
                    Some(g) => match (g.axes().as_slice(), g.sub()) {
                        ([], []) => return Comprehension::Held,
                        _ => match &dialect {
                            None => dialect = Some((held.address(), g)),
                            Some((addr, _)) if held.address() < *addr => {
                                dialect = Some((held.address(), g));
                            }
                            _ => {}
                        },
                    },
                }
            }
            return match dialect {
                None => Comprehension::Admitted,
                Some((_, g)) => Comprehension::Dialect(g),
            };
        }
        // Located through the atlas and the versor, never searched. A taught
        // word is one piece: one stroke locates it.
        let located = self.locate(utterance.sub(), None);
        // The remainder decides: no residual and no fiber to deepen is the
        // understood arm; anything left over is the novelty, measured.
        match (located.residual.len(), located.punctures.len()) {
            (0, 0) => {
                let sentence: Vec<(Vec<u8>, Standing)> = located
                    .cites
                    .iter()
                    .filter_map(|(i, _, _)| {
                        let w = self.genome.get(*i)?;
                        Some((w.address().into_vec(), Standing::Ruled))
                    })
                    .collect();
                for (i, _, _) in &located.cites {
                    if let Some(x) = self.expression.get_mut(*i) {
                        *x += 1;
                    }
                }
                if !sentence.is_empty() {
                    let addrs: Vec<Vec<u8>> = sentence.iter().map(|(a, _)| a.clone()).collect();
                    let key = utterance.address();
                    match self.recollections.get_mut(&key) {
                        Some((_, n)) => *n += 1,
                        None => {
                            self.recollections.insert(key, (addrs, 1));
                        }
                    }
                }
                Comprehension::Understood(sentence)
            }
            _ => Comprehension::Novel { residual_cells: located.open(self.genome(), utterance.sub()).len() },
        }
    }

    /// Chapter 24.2 / 13.11: a form whose Glued citations this mind holds,
    /// or whose cited chart is held under other addresses. No re-gauge:
    /// a closed citation contributes no class. No Dialect from a sentence.
    fn comprehend_form(&mut self, f: &crate::transducer::sentence::Form) -> Comprehension {
        use crate::transducer::family::Crystal;
        use crate::transducer::sentence::Vertex;
        let mut missing = Vec::new();
        let mut understood = Vec::new();
        for v in &f.vertices {
            let Vertex::Glued { address, chart, .. } = v else {
                continue;
            };
            let address = address.clone();
            let chart = chart.clone();
            if let Some(&i) = self.index.get(&address) {
                if let Some(x) = self.expression.get_mut(i) {
                    *x += 1;
                }
                understood.push((address.clone().into_vec(), Standing::Ruled));
                continue;
            }
            if self.working_word(&address).is_some() {
                let Some(st) = self.working_atlas().standing_of(&address, &chart) else {
                    missing.push(address.clone().into_vec());
                    continue;
                };
                understood.push((address.clone().into_vec(), st));
                continue;
            }
            let mut through = 0usize;
            for (e, _, st) in Crystal::on_class(self, &chart) {
                if e.address() == address {
                    continue;
                }
                understood.push((e.address().into_vec(), st));
                through += 1;
                eprintln!(
                    "  form understood through {:02x}{:02x}… for {:02x}{:02x}…",
                    e.address()[0], e.address()[1], address[0], address[1]
                );
            }
            if through == 0 {
                missing.push(address.into_vec());
            }
        }
        if !missing.is_empty() {
            return Comprehension::Withheld { missing };
        }
        if !understood.is_empty() {
            return Comprehension::Understood(understood);
        }
        let vk = crate::transducer::sentence::form_vertex_key(f);
        eprintln!("  form admitted {:02x}{:02x}…", vk[0], vk[1]);
        Comprehension::Admitted
    }

    /// LEARN — the mesh expressed a word down (`teach`): comprehend it against
    /// what is already held, then bank it in the genome if it is new (by base
    /// address). Inheritance, not local invention: only taught words enter.
    /// Tests without a voice; a live mind uses [`learn_at`] with K and tu.
    pub fn learn(&mut self, taught: &[u8]) -> Option<Comprehension> {
        self.learn_at(taught, 0, 0)
    }

    pub fn learn_at(&mut self, taught: &[u8], k: u64, now_tu: u64) -> Option<Comprehension> {
        // The syllabus may carry several words back to back (every anchored
        // citation of a sentence): each is learned in turn; the first
        // comprehension is returned.
        let mut pos = 0usize;
        let mut first: Option<Comprehension> = None;
        while pos < taught.len() {
            let before = pos;
            let heard = self.learn_one(taught, &mut pos, k, now_tu);
            if pos == before {
                break;
            }
            match heard {
                Some(h) => {
                    if first.is_none() {
                        first = Some(h);
                    }
                }
                None => {
                    first.as_ref()?;
                }
            }
        }
        first
    }

    fn learn_one(&mut self, taught: &[u8], pos: &mut usize, k: u64, now_tu: u64) -> Option<Comprehension> {
        let e = Element::from_bytes(taught, pos)?;
        let addr = e.address();
        let resolve = |a: &[u8]| {
            self.working_word(a)
                .and_then(crate::transducer::sentence::from_element_form)
        };
        match crate::verdict::blind_replay_with(&e, &resolve) {
            Some(crate::verdict::REFUSE_GLUED_UNRULED)
            | Some(crate::verdict::REFUSE_GLUED_DEPTH) => {}
            Some(reason) => {
                eprintln!(
                    "  syllabus refused {:02x}{:02x}…: {reason}",
                    addr[0], addr[1]
                );
                self.refused += 1;
                return None;
            }
            None => {
                if let Some(f) = crate::transducer::sentence::from_element_form(&e) {
                    if f.rank().is_none() {
                        eprintln!(
                            "  syllabus refused {:02x}{:02x}…: {}",
                            addr[0],
                            addr[1],
                            crate::verdict::REFUSE_RANK_LOST
                        );
                        self.refused += 1;
                        return None;
                    }
                }
            }
        }
        let spelled = !e.is_connection() || e.letters_rederive() == Some(true);
        if !spelled {
            self.refused += 1;
            return None;
        }
        // The APERTURE tunes on what is inherited: each transition refracts a
        // value signal into h[0]; each torsion period a spatial signal into h[1]
        // — the entity's optics drift with its diet (dialect-legitimate).
        for &(_, _, before, after) in e.sub() {
            self.aperture.absorb_value_signal(before, after);
        }
        let ft = crate::transducer::sentence::form_torsion(&e);
        if !ft.is_empty() {
            for (_, (_, l, _)) in ft.fiber(crate::network::nerve::ROOT) {
                self.aperture.absorb_spatial_signal(*l);
            }
        } else {
            for (_, l, _) in e.torsion() {
                self.aperture.absorb_spatial_signal(*l);
            }
        }
        let heard = self.comprehend(&e);
        if let Comprehension::Withheld { missing } = &heard {
            let missing = missing.clone();
            if let Some(da) = self.stand_dialect_at(e, k, now_tu) {
                let old = self
                    .dialect
                    .get(&da)
                    .map(|d| d.missing.clone())
                    .unwrap_or_default();
                for m in &old {
                    self.retract_waiter(m, &da);
                }
                if let Some(d) = self.dialect.get_mut(&da) {
                    d.withheld = true;
                    d.missing = missing.clone();
                }
                for m in &missing {
                    self.wait_on(m, da.clone());
                }
                eprintln!(
                    "  form withheld waiting {:02x}{:02x}…",
                    da[0], da[1]
                );
            }
            return Some(heard);
        }
        // The court promotes: a syllabus word that is the same vertex as a
        // dialect entry moves that entry into the genome, carrying its uses.
        if let Some(da) = self.same_vertex_in_dialect(&e) {
            let uses = self.dialect.get(&da).map(|d| d.uses).unwrap_or(1);
            self.drop_dialect(&da);
            if let Some(i) = self.same_vertex_in_genome(&e) {
                let _ = self.genome[i].merge_fibre(&e);
                if let Some(x) = self.expression.get_mut(i) {
                    *x = x.saturating_add(uses);
                }
            } else {
                self.atlas.admit(&crate::transducer::stalk::invariant_of(&e), e.address());
                self.axis_admit(&e);
                self.shapes.push(std::sync::OnceLock::new());
                self.atlas_len = self.atlas_len.saturating_add(1);
                self.index.insert(e.address(), self.genome.len());
                self.genome.push(e);
                self.expression.push(uses);
            }
            self.promoted.push(addr.into_vec());
            return Some(heard);
        }
        let arrived = e.clone();
        match self.index.get(&addr).copied() {
            Some(i) => self.genome[i] = e,
            None => {
                self.atlas.admit(&crate::transducer::stalk::invariant_of(&e), e.address());
                self.axis_admit(&e);
                self.shapes.push(std::sync::OnceLock::new());
                self.atlas_len = self.atlas_len.saturating_add(1);
                self.index.insert(e.address(), self.genome.len());
                self.genome.push(e);
                self.expression.push(0);
            }
        }
        self.resolve_withheld(&arrived);
        Some(heard)
    }

    /// A cited tape that just arrived: withheld sentences naming it
    /// re-comprehend. The waiting-on index is the read.
    fn resolve_withheld(&mut self, arrived: &Element) {
        let addr = arrived.address();
        let citing: Vec<Vec<u8>> = match self.waiting_on.remove(&addr) {
            Some(citers) => citers
                .fiber(crate::network::nerve::ROOT)
                .map(|(k, _)| k)
                .collect(),
            None => return,
        };
        for sa in citing {
            let Some(sentence) = self.dialect.get(&sa).map(|d| d.sentence.clone()) else {
                continue;
            };
            let old_missing = self
                .dialect
                .get(&sa)
                .map(|d| d.missing.clone())
                .unwrap_or_default();
            let heard = self.comprehend(&sentence);
            match heard {
                Comprehension::Withheld { missing } => {
                    for m in &old_missing {
                        if m.as_slice() != addr.as_bytes() {
                            self.retract_waiter(m, &sa);
                        }
                    }
                    if let Some(d) = self.dialect.get_mut(&sa) {
                        d.missing = missing.clone();
                    }
                    for m in &missing {
                        self.wait_on(m, sa.clone());
                    }
                }
                Comprehension::Understood(_) | Comprehension::Admitted | Comprehension::Dialect(_) => {
                    for m in &old_missing {
                        if m.as_slice() != addr.as_bytes() {
                            self.retract_waiter(m, &sa);
                        }
                    }
                    if let Some(d) = self.dialect.get_mut(&sa) {
                        d.withheld = false;
                        d.missing.clear();
                    }
                    eprintln!(
                        "  form withheld resolved {:02x}{:02x}…",
                        sa[0], sa[1]
                    );
                }
                _ => {}
            }
        }
    }

    #[cfg(test)]
    pub(super) fn waiting_on_len(&self, missing: &[u8]) -> usize {
        self.waiting_on.get(missing).map(|c| c.len()).unwrap_or(0)
    }

    /// CRYSTALLIZE the genome for persistence — the permanent language only.
    /// The epigenome is memory-only. Format: `GEN3 | n u32 | (len u32 | element)…`.
    pub fn crystallize(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(4 + self.genome.len() * 64);
        out.extend_from_slice(b"GEN3");
        out.extend_from_slice(&(self.genome.len() as u32).to_le_bytes());
        for g in &self.genome {
            let b = g.to_bytes();
            out.extend_from_slice(&(b.len() as u32).to_le_bytes());
            out.extend_from_slice(&b);
        }
        out
    }

    /// Restore GEN3 only. An older file is refused.
    pub fn uncrystallize(&mut self, bytes: &[u8]) -> bool {
        if bytes.len() < 8 {
            return false;
        }
        let tag = &bytes[..4];
        if tag != b"GEN3" {
            return false;
        }
        let n = u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as usize;
        if n > bytes.len().saturating_sub(8) / 5 {
            return false; // untrusted count vs remaining bytes
        }
        let mut genome = Vec::with_capacity(n);
        let mut pos = 8usize;
        let mut skipped = 0usize;
        for _ in 0..n {
            let Some(l4) = bytes.get(pos..pos + 4) else { break };
            let len = u32::from_le_bytes(l4.try_into().unwrap()) as usize;
            pos += 4;
            if len > bytes.len().saturating_sub(pos) {
                break;
            }
            let slice = &bytes[pos..pos + len];
            pos += len;
            let mut ep = 0usize;
            match Element::from_bytes(slice, &mut ep) {
                Some(e) => genome.push(e),
                None => {
                    // Previous GEN1 layouts (pre-fibre-index tails, a
                    // word that will not uncrystallize) must not sink the
                    // rest: residuals are never lost.
                    skipped += 1;
                }
            }
        }
        if genome.is_empty() {
            return false;
        }
        if skipped > 0 {
            eprintln!("genome restored: {} words, {skipped} skipped", genome.len());
        }
        self.expression = vec![0; genome.len()];
        self.genome = genome;
        self.atlas = crate::transducer::stalk::Stalks::default();
        self.axis_index = crate::transducer::sentence::AxisIndex::new();
        self.shapes.clear();
        for w in &self.genome {
            self.atlas.admit(&crate::transducer::stalk::invariant_of(w), w.address());
            crate::transducer::sentence::admit_words(
                &mut self.axis_index,
                &crate::transducer::stalk::invariant_of(w),
                &crate::transducer::sentence::words_of_element(w),
            );
            self.shapes.push(std::sync::OnceLock::new());
        }
        self.atlas_len = self.genome.len();
        self.reindex();
        true
    }
}

impl crate::transducer::family::Crystal for Epigenome {
    fn on_class(
        &self,
        class: &crate::transducer::stalk::Invariant,
    ) -> Vec<(
        Element,
        crate::transducer::sentence::Form,
        crate::transducer::stalk::Standing,
    )> {
        self.working_atlas()
            .fiber(class)
            .into_iter()
            .filter_map(|(a, st)| {
                let e = self.working_word(&a)?.clone();
                let f = crate::transducer::sentence::from_element_form(&e)?;
                if !f.is_closed() {
                    return None;
                }
                Some((e, f, st))
            })
            .collect()
    }

    fn speak_form(
        &self,
        f: &crate::transducer::sentence::Form,
        x: &crate::transducer::family::Grid,
        placement: &crate::transducer::alphabet::Word,
    ) -> Option<crate::transducer::family::Speech> {
        crate::transducer::family::apply_form(f, x, placement, self)
    }

    fn by_address(
        &self,
        addr: &[u8],
    ) -> Option<(Element, crate::transducer::sentence::Form)> {
        let e = self.working_word(addr)?.clone();
        let f = crate::transducer::sentence::from_element_form(&e)?;
        Some((e, f))
    }

    fn on_axis(
        &self,
        chart: &crate::transducer::stalk::Invariant,
        axis: crate::transducer::sentence::Axis,
    ) -> Vec<crate::transducer::alphabet::Word> {
        crate::transducer::sentence::on_axis_of(&self.axis_index, chart, axis)
    }
}
