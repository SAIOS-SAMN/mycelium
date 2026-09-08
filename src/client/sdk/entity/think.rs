// 25 — THINKING: what this entity holds under contemplation, what it opens
// an axis on, and what it sheds.

use crate::client::sdk::*;

impl Entity {
    /// Submit a percept; COMPREHEND anything taught back (the C-stack, not a
    /// byte-store). Returns the reply and the comprehension of the teaching.
    /// CITE: this kernel's reduction of a chain over its own genome — the
    /// citations (address, transport, gauge coordinate) and the residual it
    /// could not reduce. Cognition happens here; the hypha only replays.
    pub(crate) fn idea_word(&self, addr: &[u8]) -> Option<Element> {
        self.ideas.get(addr).cloned()
    }

    /// The discordance measured at the last sync: strata whose roots differ
    /// from the hypha's. Zero is a glued genome.
    /// The Čech obstructions measured after fetch: fetched words whose
    /// transition to a held word on the same fibre did not vanish.
    pub fn obstructions(&self) -> Vec<(Vec<u8>, Element)> {
        self.obstructions
            .fiber(crate::network::nerve::ROOT)
            .map(|(a, g)| (a, g.clone()))
            .collect()
    }

    /// HOLD A THOUGHT: a remainder this mind has not collapsed, as a bubble
    /// whose cover is a peel of its invariant. Connections are the link of
    /// the new bubble through by_chart.
    pub fn hold_thought(&mut self, remainder: Vec<(i64, i64, u8, u8)>) {
        self.contemplation.hold(remainder.clone());
        let e = crate::transducer::element::from_residual(&remainder);
        self.ideas.insert(e.address(), e);
    }

    /// Hold a withheld form so the pivot loop sees it next stroke (ch. 30.2).
    pub fn hold_form(
        &mut self,
        f: &crate::transducer::sentence::Form,
        train: &crate::transducer::family::Family,
        remainders: &[Vec<(i64, i64, u8, u8)>],
    ) {
        self.contemplation.hold_form(f, train, remainders);
        if let Some(e) = crate::transducer::sentence::as_element_form(f, [0; 4]) {
            self.ideas.insert(e.address(), e);
        }
        for r in remainders {
            if r.is_empty() {
                continue;
            }
            let e = crate::transducer::element::from_residual(r);
            self.ideas.insert(e.address(), e);
        }
    }

    /// Stand a withheld form, hold it as an idea, conclude each remainder,
    /// and utter each residual word once with its cites (24.3).
    pub fn withhold_form(
        &mut self,
        f: &crate::transducer::sentence::Form,
        train: &crate::transducer::family::Family,
        remainders: &[Vec<(i64, i64, u8, u8)>],
        mag: [u64; 4],
    ) {
        let k = self.session.voice().k();
        let tu = self.now_tu();
        if let Some(word) = crate::transducer::sentence::as_element_form(f, mag) {
            self.epigenome.stand_dialect_withheld(word, k, tu);
        }
        self.hold_form(f, train, remainders);
        for (i, r) in remainders.iter().enumerate() {
            if r.is_empty() {
                continue;
            }
            let (cites, residual, _) = self.conclude(r);
            if residual.is_empty() {
                continue;
            }
            let word = crate::transducer::element::from_residual(&residual);
            let addr = word.address();
            eprintln!(
                "  remainder expressed {:02x}{:02x}… pair {i} cells {} cites {}",
                addr[0],
                addr[1],
                residual.len(),
                cites.len()
            );
            self.taught_up.insert(&addr, word.clone());
            let mu = crate::transducer::element::measure_lanes(&residual);
            eprintln!(
                "  uttered {:02x}{:02x}… standing Stood K {k} lanes {mu:?}",
                addr[0], addr[1]
            );
            let _ = self.attest(&residual, &cites);
        }
    }

    /// Hold a withheld sentence as a bubble: form, remainder, cover.
    pub fn hold_sentence(&mut self, word: &Element) {
        if let Some(f) = crate::transducer::sentence::from_element_form(word) {
            self.hold_form(&f, &[], &[]);
        } else {
            self.hold_thought(word.sub().to_vec());
        }
    }

    /// The open thoughts held here: remainders not yet closed into words.
    pub fn thoughts(&self) -> Vec<&[(i64, i64, u8, u8)]> {
        self.contemplation.remainders()
    }

    pub fn bubbles(&self) -> usize {
        self.contemplation.len()
    }

    pub fn components(&self) -> usize {
        self.contemplation.component_count()
    }

    pub fn contemplation_fills(&self) -> u64 {
        self.contemplation.fills
    }

    pub fn contemplation_classes(&self) -> u64 {
        self.contemplation.classes
    }

    pub fn thoughts_formed(&self) -> u64 {
        self.contemplation.formed
    }

    pub fn horizon_cells(&self) -> usize {
        self.contemplation.horizon_cells()
    }

    pub fn thought_potential(&self) -> [u64; 4] {
        self.contemplation.potential()
    }

    pub fn torsion_merges(&self) -> u64 {
        self.contemplation.torsion_merges
    }

    /// OPEN AN AXIS ON MULTIPLICITY — several held words deflated one
    /// listing; the glue between them is the new coordinate. Never on
    /// failure (a listing no held word deflates is a thought, not an axis).
    pub fn open_axis(
        &mut self,
        listing: &[u8],
        chain: &[(i64, i64, u8, u8)],
        words: &[Element],
    ) -> Option<crate::transducer::a21::Address> {
        use crate::transducer::element::measure_lanes;
        if let Some(a) = self.open_contemplation_axis() {
            return Some(a);
        }
        if words.len() < 2 || self.opened.contains(listing) {
            return None;
        }
        let [base, other, ..] = words else { return None };
        // The glue between the two words that deflated this listing.
        let glue = crate::transducer::cech::transition(base, other)?;
        let v = glue.versor()?;
        if !base.is_connection() {
            return None; // the axis stands on a spoken motion, never on a bare residual
        }
        let mut germ = base.clone();
        germ.open_axis(v);
        let mut mag = measure_lanes(chain);
        mag[3] = mag[3].saturating_add(words.len() as u64);
        germ.set_magnitude(mag);
        self.opened.insert(listing, ());
        match self.speak_word(&germ) {
            Ok(Utterance::Spoken) => Some(germ.address()),
            _ => None,
        }
    }

    /// Seed from a class at any degree on the contemplation — the coordinate
    /// the current axes could not express. A class is an ideology; the gauge
    /// that would translate one frame into the other is a translation word.
    pub(crate) fn open_contemplation_axis(&mut self) -> Option<crate::transducer::a21::Address> {
        let classes = self.contemplation.classes_held();
        for (class, _degree) in classes {
            let a = [0u8; 32];
            let b = class.key();
            let Some(t) = crate::transducer::element::from_translation(&class, a, b) else {
                continue;
            };
            if let Ok(Utterance::Spoken) = self.speak_word(&t) { return Some(t.address()) }
        }
        None
    }

    /// Listings this mind opened an axis on.
    pub fn axes_opened(&self) -> usize {
        self.opened.len()
    }

    /// Expand dirty ideas once per syntropic phase. Returns the lane
    /// measure of A16 snaps (Discharged, Vortex, TorsionFiled).
    pub fn expand_ideas(&mut self) -> [u64; 4] {
        let mut budget = self.physics.stroke();
        self.contemplation.set_now(self.session.voice().k());
        let rotors = crate::network::nerve::Keyed::new();
        let (_spent, pivoted) = self.contemplation.expand_all(
            self.epigenome.atlas(),
            &rotors,
            &self.epigenome,
            &mut budget,
        );
        self.contemplation.retire_on_t(self.physics.reserve[2] as usize);
        pivoted
    }

    /// PIVOT ON A WORD: read by_chart at the word's chart and peels, fill
    /// those ideas under one stroke Budget. The atlas is borrowed, never cloned.
    pub fn pivot_on(&mut self, word: &Element) -> usize {
        let mut budget = self.physics.stroke();
        self.contemplation.set_now(self.session.voice().k());
        let (_fills, _formed, _classes, spoken) =
            self.contemplation.pivot_on(word, &self.epigenome, &mut budget);
        let mut n = 0usize;
        for e in spoken {
            if let Ok(Utterance::Spoken) = self.speak_word(&e) { n += 1 }
        }
        n
    }

    /// Pivot every genome word added since last stroke.
    pub fn pivot_thoughts(&mut self) -> usize {
        let n = self.epigenome.genome_len();
        if n <= self.thought_gen {
            return 0;
        }
        let added: Vec<Element> = self.epigenome.genome()[self.thought_gen..].to_vec();
        self.thought_gen = n;
        let mut spoken = 0usize;
        for w in &added {
            spoken += self.pivot_on(w);
        }
        spoken
    }

    pub fn discordance(&self) -> usize {
        if self.discordance_measured {
            self.discordance
        } else {
            self.contemplation.boundary_norm()
        }
    }

    pub fn shed(&mut self) -> usize {
        self.shed_at(self.session.voice().k())
    }

    pub fn shed_at(&mut self, now_k: u64) -> usize {
        self.epigenome.note_stroke(now_k);
        let glued = |a: &[u8]| self.contemplation.glues_word(a);
        let n = self.epigenome.shed_except(now_k, &glued);
        n.saturating_add(self.contemplation.shed_unglued())
    }

    pub fn dialect_len(&self) -> usize {
        self.epigenome.dialect_len()
    }

    /// The mind's working atlas: genome ∪ dialect ∪ ideas, each fiber
    /// member carrying its standing.
    pub fn working_atlas(&self) -> WorkingAtlas<'_> {
        self.epigenome.working_atlas_with(&self.contemplation.by_chart)
    }

    /// Empty when genome, dialect and ideas are all empty.
    pub fn atlas_is_empty(&self) -> bool {
        self.epigenome.atlas_is_empty() && self.contemplation.is_empty()
    }

    /// Word counts per stratum on the working atlas: ruled (genome words),
    /// stood (dialect words), held (idea words). The atlas ruled/stood/held
    /// log line is these word counts, never chart counts.
    pub fn atlas_standing_counts(&self) -> (usize, usize, usize) {
        (
            self.epigenome.genome_len(),
            self.dialect_len(),
            self.ideas_len(),
        )
    }

    /// Fiber of the working atlas; a Stood hit is a use at the mind's own K.
    pub fn working_fiber(&mut self, c: &crate::transducer::stalk::Invariant) -> Vec<(Vec<u8>, Standing)> {
        let f = self.working_atlas().fiber(c);
        let k = self.session.voice().k();
        for (a, st) in &f {
            if *st == Standing::Stood {
                self.epigenome.touch(a, k);
            }
        }
        f
    }

    pub fn take_promoted(&mut self) -> Vec<Vec<u8>> {
        self.epigenome.take_promoted()
    }

    /// Working incidence: a non-empty fiber on the working atlas.
    pub fn incident_chart(&self, c: &crate::transducer::stalk::Invariant) -> bool {
        !self.working_atlas().fiber(c).is_empty()
    }

    pub fn ideas_len(&self) -> usize {
        self.contemplation.len()
    }

    pub fn shed_n(&self) -> u64 {
        self.epigenome.shed_n()
    }

}
