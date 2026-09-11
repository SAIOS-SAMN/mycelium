// 25 — THINKING: what this entity holds under contemplation, what it opens
// an axis on, and what it sheds.

use crate::client::sdk::*;

impl Entity {
    /// The Čech obstructions measured after fetch: fetched words whose
    /// transition to a held word on the same fibre did not vanish.
    pub fn obstructions(&self) -> Vec<(Vec<u8>, Element)> {
        self.obstructions
            .fiber(crate::network::nerve::ROOT)
            .map(|(a, g)| (a, g.clone()))
            .collect()
    }

    /// HOLD A CHAIN AS THE FORM IT IS (50.6 seam 1): a demand this mind has
    /// not closed is held as an idea — the pair form of the scene the chain
    /// describes, its cells before against its cells after in one frame —
    /// never as a bare chain minted into a word. An idea is a form; its
    /// words are the vertices the eyes could pair and the alphabet could
    /// spell, and what they could not is its demand vertices (50.5).
    pub fn hold_chain(&mut self, chain: Vec<(i64, i64, u8, u8)>) {
        use crate::transducer::family::{family_ratio, pair_form, scene_of_chain};
        let Some(scene) = scene_of_chain(&chain) else {
            return;
        };
        let scale = family_ratio(std::slice::from_ref(&scene));
        // 50.2 step 2: this mind's own presentation is what its word
        // vertices derive against, under one stroke like every other
        // keyed read (rule 10).
        let mut budget = self.physics.stroke();
        // `None` is a vertex the stroke could not factor through this
        // mind's presentation: unmeasured, so nothing is held rather than
        // a form held with the presentation's reach asserted unread.
        let Some(f) = pair_form(&self.epigenome, &mut budget, &scene.0, &scene.1, scale) else {
            return;
        };
        self.hold_form(&f, &[], &[]);
    }

    /// Hold a withheld form as an idea so the pivot loop sees it next
    /// stroke (30.2). The contemplation is the one carrier of held forms
    /// (24.1 as ruled 2026-09-09): its remainders stay on the bubble as
    /// what the form does not speak, and its words are read at a chart
    /// when a cite asks — through the form's own `by_class` — never
    /// listed here ahead of the question.
    pub fn hold_form(
        &mut self,
        f: &crate::transducer::sentence::Form,
        train: &crate::transducer::family::Family,
        remainders: &[Vec<(i64, i64, u8, u8)>],
    ) {
        self.contemplation.hold_form(f, train, remainders);
    }

    /// Stand a withheld form and hold it as an idea (24.3 as amended,
    /// 50.6 seam 1). Nothing is expressed as a residual word and no chain
    /// is uttered: what the form could not close is its demand vertices,
    /// spoken with the sentence by `speak_form`, and a remainder no demand
    /// names is the idea's own work.
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
    }

    /// Hold a withheld sentence as a bubble: form, remainder, cover. A word
    /// that is not a sentence is held as the form of its chain.
    pub fn hold_sentence(&mut self, word: &Element) {
        if let Some(f) = crate::transducer::sentence::from_element_form(word) {
            self.hold_form(&f, &[], &[]);
        } else {
            self.hold_chain(word.sub().to_vec());
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

    /// OPEN AN AXIS ON MULTIPLICITY (24.3) — several held words deflated
    /// one listing; the glue between them is the new coordinate. A held
    /// word is a vertex of a held form, and what it deflated is a body of
    /// the listing (the held cite's body, 24.1), so the glue is the
    /// dislocation versor between the bodies' shapes and the germ a
    /// connection carrying it (`axis_germ`). The listing is recorded before
    /// the germ is spoken, so it is tried once per language
    /// (`reopen_axes`); a glue that cannot be read or spoken is refused with
    /// its reason, which the stroke writes. Never on failure: a listing no
    /// held word deflates is a thought, not an axis.
    pub fn open_axis(
        &mut self,
        listing: &[u8],
        chain: &[(i64, i64, u8, u8)],
        bodies: &[Vec<(i64, i64, u8, u8)>],
    ) -> Result<crate::transducer::address::Address, &'static str> {
        if let Some(a) = self.open_contemplation_axis() {
            return Ok(a);
        }
        if self.opened.contains(listing) {
            return Err(AXIS_TRIED);
        }
        let germ = Self::axis_germ(chain, bodies)?;
        self.opened.insert(listing, ());
        match self.speak_word(&germ) {
            Ok(Utterance::Spoken) => {
                self.axes_spoken = self.axes_spoken.saturating_add(1);
                Ok(germ.address())
            }
            Ok(Utterance::Already) => Err(AXIS_ALREADY),
            _ => Err(AXIS_REFUSED_SPEECH),
        }
    }

    /// THE AXIS GERM, read off the bodies alone: the versor carrying the
    /// first body's shape onto the second's (the one the court's `Twisted`
    /// closure carries), minted as a connection with the identity word and
    /// that carriage, the axis opened on it, its magnitude the listing's
    /// with the holonomic lane raised by the count of words that met. One
    /// body, a body with no shape, or bodies no rational versor relates are
    /// refused by name.
    pub fn axis_germ(chain: &[(i64, i64, u8, u8)], bodies: &[Vec<(i64, i64, u8, u8)>]) -> Result<Element, &'static str> {
        use crate::transducer::element::{from_connection, measure_lanes, shape_of_chain};
        let [base, other, ..] = bodies else { return Err(AXIS_ONE_BODY) };
        let sa = shape_of_chain(base).ok_or(AXIS_NO_BODY)?;
        let sb = shape_of_chain(other).ok_or(AXIS_NO_BODY)?;
        let v = crate::transducer::blade::versor_between(&sa, &sb).ok_or(AXIS_NO_VERSOR)?;
        let mut germ = from_connection(&crate::transducer::alphabet::Word::identity(), &v, &[], &Vec::new(), [0; 4])
            .ok_or(AXIS_NO_VERSOR)?;
        germ.open_axis(v);
        let mut mag = measure_lanes(chain);
        mag[3] = mag[3].saturating_add(bodies.len() as u64);
        germ.set_magnitude(mag);
        Ok(germ)
    }

    /// The language moved: every listing may open an axis again (24.3).
    pub fn reopen_axes(&mut self) {
        self.opened = crate::network::nerve::Keyed::new();
    }

    /// Seed from a class at any degree on the contemplation — the coordinate
    /// the current axes could not express. A class is an ideology; the gauge
    /// that would translate one frame into the other is a translation word.
    pub(crate) fn open_contemplation_axis(&mut self) -> Option<crate::transducer::address::Address> {
        let classes = self.contemplation.classes_held();
        for (_kappa, class, _degree) in classes {
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
        self.axes_spoken
    }

    /// Expand dirty ideas once per syntropic phase (25.1): each dirty idea
    /// expands over the working atlas — genome and dialect, read as one
    /// atlas, the held word's rotor read by address — and runs the pivot
    /// loop with the epigenome as its crystal (the closed forms it holds,
    /// ruled or stood; an idea is glued only once it has closed and stood,
    /// 13.6). An idea that closes crystallizes (25.2), stands in the
    /// dialect and is uttered once (24.3). Returns the lane measure of the
    /// A16 snaps (Discharged, Vortex, TorsionFiled) and the dirty ring's
    /// orbit reading — circuits and period in strokes — when this stroke
    /// completed a circuit (45.7, S13.3).
    pub fn expand_ideas(&mut self) -> ([u64; 4], Option<(u64, u64)>) {
        let mut budget = self.physics.stroke();
        self.contemplation.set_now(self.session.voice().k());
        let crate::transducer::bubble::drive::Expanded { pivoted, crystallized, orbit, .. } = {
            let epigenome = &self.epigenome;
            let atlas = epigenome.working_atlas();
            let rotor_at = |a: &[u8]| {
                epigenome
                    .working_word(a)
                    .and_then(|h| crate::transducer::cech::rotor_of(h.element()))
            };
            self.contemplation
                .expand_all(&atlas, &rotor_at, epigenome, &mut budget)
        };
        self.contemplation.retire_on_t(self.physics.reserve[2] as usize);
        for e in &crystallized {
            self.stand_crystallized(e);
        }
        (pivoted, orbit)
    }

    /// A crystallized idea stands in the dialect and is uttered once
    /// (25.2, 24.3): the same vertex already stood or taught is a use, not
    /// a second utterance.
    fn stand_crystallized(&mut self, e: &Element) {
        let addr = e.address();
        if self.already_taught(e) {
            self.remember_spoken(e);
            return;
        }
        let (depth, vertices, glued, faces, torsion) =
            match crate::transducer::sentence::from_element_form(e) {
                Some(f) => (
                    f.depth,
                    f.vertices.len(),
                    f.vertices
                        .iter()
                        .filter(|v| matches!(v, crate::transducer::sentence::Vertex::Glued { .. }))
                        .count(),
                    f.faces.len(),
                    f.torsion.len(),
                ),
                None => (0, 0, 0, 0, 0),
            };
        match self.speak(std::slice::from_ref(e)) {
            Ok(codes) if codes.first().copied() == Some(dialogue::SPEECH_REFUSED) => {
                eprintln!(
                    "  form withheld: {:02x}{:02x}… refused on the wire",
                    addr[0], addr[1]
                );
            }
            Ok(_) => {
                eprintln!(
                    "  form spoken: {:02x}{:02x}… depth {depth} vertices {vertices} glued {glued} faces {faces} torsion {torsion}",
                    addr[0], addr[1]
                );
            }
            Err(err) => {
                eprintln!(
                    "  form withheld: {:02x}{:02x}… {err}",
                    addr[0], addr[1]
                );
            }
        }
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
        let added: Vec<Element> = self.epigenome.genome()[self.thought_gen..]
            .iter()
            .map(|h| h.element().clone())
            .collect();
        self.thought_gen = n;
        let mut spoken = 0usize;
        for w in &added {
            spoken += self.pivot_on(w);
        }
        spoken
    }

    /// The strata whose roots differed at the last sync (learn.rs), or
    /// `None` while no sync has measured it. It is not the boundary measure
    /// of the held thoughts: the two were one `usize` once, which summed
    /// four lanes into a count (law 5, L7a).
    pub fn discordance(&self) -> Option<usize> {
        if self.discordance_measured {
            Some(self.discordance)
        } else {
            None
        }
    }

    /// The four-lane measure of what this mind holds open (law 5), read
    /// per lane by whoever reads it.
    pub fn boundary_measure(&self) -> [u64; 4] {
        self.contemplation.boundary_measure()
    }

    pub fn shed(&mut self) -> usize {
        self.shed_at(self.session.voice().k())
    }

    pub fn shed_at(&mut self, now_k: u64) -> usize {
        self.epigenome.note_stroke(now_k);
        let glued = |a: &[u8]| self.contemplation.glues_word(a);
        let dropped = self.epigenome.shed_except(now_k, &glued);
        // 7.3, S12.6a: the utterance receipt of a word that shed leaves
        // with it. Kept, `taught_up` grew by every word the mind ever
        // uttered — the strokes, not the shed's residue — and 24.3's "nothing
        // is uttered twice" only needs the receipt while the word stands.
        for a in &dropped {
            self.forget_utterance(a);
        }
        dropped.len().saturating_add(self.contemplation.shed_unglued())
    }

    pub fn dialect_len(&self) -> usize {
        self.epigenome.dialect_len()
    }

    /// The mind's working atlas: genome ∪ dialect ∪ the held forms' words,
    /// each fiber member carrying its standing; the Held stratum is read
    /// from the contemplation at the chart asked (24.1).
    pub fn working_atlas(&self) -> WorkingAtlas<'_> {
        self.epigenome.working_atlas_with(&self.contemplation)
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
