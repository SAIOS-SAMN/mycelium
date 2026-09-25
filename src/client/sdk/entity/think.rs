// 25 — THINKING: what this entity holds under contemplation, what it opens
// an axis on, and what it sheds.

use crate::client::sdk::*;

impl Entity {
    /// The Čech obstructions measured after fetch: fetched words whose
    /// transition to a held word on the same fibre did not vanish.
    pub fn obstructions(&self) -> Vec<(Vec<u8>, Element)> {
        // This mind's standing obstructions, which is what it is asked for.
        self.obstructions
            .scoped()
            .map(|(a, g)| (a, g.clone()))
            .collect()
    }

    /// HOLD A CHAIN AS THE FORM IT IS (50.6 seam 1): a demand this mind has
    /// not closed is held as an idea — the pair form of the scene the chain
    /// describes, its painted sites before against its painted sites after in one frame —
    /// never as a bare chain minted into a word. An idea is a form; its
    /// words are the vertices the eyes could pair and the alphabet could
    /// spell, and what they could not is its demand vertices (50.5).
    pub fn hold_chain(&mut self, chain: Vec<(i64, i64, u8, u8)>) {
        use crate::transducer::family::{scene_form, scene_of_chain};
        let Some(scene) = scene_of_chain(&chain) else {
            return;
        };
        // 50.2 step 2: this mind's own presentation is what its word
        // vertices derive against, under a stroke sized by the scene it is
        // handed (L8w); a fixed spend unit does not reach one scene.
        let mut budget = self
            .physics
            .stroke_n(crate::transducer::family::extraction_spend(std::slice::from_ref(&scene)));
        // `None` is a vertex the stroke could not factor through this
        // mind's presentation: unmeasured, so nothing is held rather than
        // a form held with the presentation's reach asserted unread.
        let Some(f) = scene_form(&self.epigenome, &mut budget, &scene.0, &scene.1) else {
            return;
        };
        self.hold_form(f, &[], &[]);
    }

    /// Hold a withheld form as an idea so the pivot orbit sees it next
    /// stroke (30.2). The contemplation is the one carrier of held forms
    /// (24.1 as ruled 2026-09-09): its remainders stay on the bubble as
    /// what the form does not speak, and its words are read at a chart
    /// when a cite asks — through the form's own `by_class` — never
    /// listed here ahead of the question.
    /// Z4 item 1: the form is MOVED into the bubble. A caller that needs
    /// it afterwards reads it back from the contemplation by reference.
    pub fn hold_form(
        &mut self,
        f: crate::transducer::sentence::Form,
        train: &crate::transducer::family::Family,
        remainders: &[Vec<(i64, i64, u8, u8)>],
    ) -> bool {
        // Z4d — THE DRAW IS THE ROOM (door 3), AND IT IS PER LANE. The
        // thought's own four-lane measure leaves the reserve when it is
        // held and returns when it lapses or is promoted, so what a voice
        // can still escrow on each lane IS its attention: a mind spent on
        // `𝒮` withholds placements while it still holds turns on `𝒽`. The
        // four lanes move apart because the MEASURE differs per lane; they
        // were four copies of one number only while nothing drew on them
        // separately.
        //
        // EXACT OR WITHHELD (rule 11). A lane that cannot fund the thought
        // has no room for it, and the subtraction refuses rather than
        // flooring at zero — which is the same reading as door 3's, taken
        // once, where the payment happens.
        let draw = crate::transducer::element::measure_lanes(&f.remainder.placed());
        let mut next = self.physics.reserve;
        for (r, d) in next.iter_mut().zip(&draw) {
            match r.checked_sub(*d) {
                Some(v) => *r = v,
                None => return false,
            }
        }
        self.physics.reserve = next;
        self.contemplation.hold_form(f, train, remainders, draw);
        true
    }

    /// Z4d — THE STROKE'S LAPSE: every thought the clock has passed leaves,
    /// as a set, and its escrow returns to the reserve LANE BY LANE.
    ///
    /// A thought held before any aperture was measured is paid through one
    /// here, on the first stroke that can measure a horizon, so it is
    /// neither held against a height nothing measured nor held forever for
    /// want of one (rule 15).
    pub fn lapse_held(&mut self) -> usize {
        let _ = self.contemplation.repay_unpaid();
        let (returned, n) = self.contemplation.lapse_on_clock();
        for (r, back) in self.physics.reserve.iter_mut().zip(&returned) {
            *r = r.saturating_add(*back);
        }
        // TC1 — and what the truncation removed since, returned alike.
        let truncated = self.contemplation.take_truncated_escrow();
        for (r, back) in self.physics.reserve.iter_mut().zip(&truncated) {
            *r = r.saturating_add(*back);
        }
        if n > 0 {
            eprintln!("  held lapsed {n}, escrow returned {returned:?}");
        }
        n
    }

    /// DOOR 3 OF CHAPTER 25 — ROOM, PER LANE, AND NOTHING IS EVICTED.
    ///
    /// Room is `boundary_measure` against this voice's reserve, LANE BY
    /// LANE (law 5): the open thoughts' own four-lane measure, never their
    /// sum, never a count of bubbles, and never the box's memory — which is
    /// what `retire_on_t` read and why it is deleted.
    ///
    /// A LANE WITH NO ROOM WITHHOLDS THE NEW THOUGHT. It does not evict a
    /// held one, because eviction CHOOSES: whichever held thought went
    /// would be picked by age, size or arrival, and rules 4 and 15 refuse
    /// all three. The thought that cannot be held is withheld and its
    /// demand stands on the court's docket, which is the one place a
    /// thought this mind has no room for can still be answered.
    ///
    /// A thought whose lanes overflow the carrier has no room either: the
    /// sum is refused rather than ceilinged (rule 11).
    pub fn room_for(&self, f: &crate::transducer::sentence::Form) -> bool {
        let want = crate::transducer::element::measure_lanes(&f.remainder.placed());
        // The reserve already excludes what is escrowed on held thoughts
        // (Z4d), so the room on a lane is the reserve on it and the check
        // is the draw's own, asked without making it.
        self.physics.reserve.iter().zip(&want).all(|(r, d)| r.checked_sub(*d).is_some())
    }

    /// LANE 𝒯 FOR A FORM THIS MIND IS ABOUT TO SPEAK: the span in receipts
    /// its idea has stood over. The contemplation is the one carrier of held
    /// forms and the one reader of the span (law 12); a speaker asks it here
    /// rather than reaching into it.
    pub fn span_of(
        &self,
        f: &crate::transducer::sentence::Form,
    ) -> Option<crate::transducer::bubble::contemplation::Deliberation> {
        self.contemplation.span_of(f)
    }

    /// Stand a withheld form and hold it as an idea (24.3 as amended,
    /// 50.6 seam 1). Nothing is expressed as a residual word and no chain
    /// is uttered: what the form could not close is its demand vertices,
    /// spoken with the sentence by `speak_form`, and a remainder no demand
    /// names is the idea's own work.
    pub fn withhold_form(
        &mut self,
        f: crate::transducer::sentence::Form,
        train: &crate::transducer::family::Family,
        remainders: &[Vec<(i64, i64, u8, u8)>],
        mag: [u64; 4],
    ) {
        let k = self.session.voice().k();
        let tu = self.now_tu();
        // The element is taken from the form BEFORE it is handed over, so
        // the withhold pays no copy of the form either (Z4 item 1).
        // THE DEMAND STANDS WHETHER OR NOT THERE IS ROOM TO HOLD IT. The
        // standing is what puts it on the court's docket; the hold is this
        // mind keeping a copy to work on. Door 3 refuses the second when a
        // lane is spent, and never the first.
        if let Some(word) = crate::transducer::sentence::as_element_form(&f, mag) {
            self.epigenome.stand_dialect_withheld(word, k, tu);
        }
        if !self.hold_form(f, train, remainders) {
            eprintln!("  thought withheld: no room on a lane, the demand stands on the docket");
        }
    }

    /// Hold a withheld sentence as a bubble: form, remainder, cover. A word
    /// that is not a sentence is held as the form of its chain.
    pub fn hold_sentence(&mut self, word: &Element) {
        if let Some(f) = crate::transducer::sentence::from_element_form(word) {
            self.hold_form(f, &[], &[]);
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

    pub fn horizon_len(&self) -> usize {
        self.contemplation.horizon_len()
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
    /// orbit with the epigenome as its crystal (the closed forms it holds,
    /// ruled or stood; an idea is glued only once it has closed and stood,
    /// 13.6). An idea that closes crystallizes (25.2), stands in the
    /// dialect and is uttered once (24.3). Returns the lane measure of the
    /// A16 snaps (Discharged, Vortex, TorsionFiled) and the dirty ring's
    /// circuit reading — circuits and period in strokes — when this stroke
    /// completed a circuit (45.7, S13.3).
    pub fn expand_ideas(&mut self) -> ([u64; 4], Option<(u64, u64)>) {
        let mut budget = self.physics.stroke();
        // THE CONTEMPLATION'S CLOCK IS TUNIVERSAL, WHICH IS WHAT IT SAYS IT
        // IS. `Contemplation.now_k` reads "TUniversal the entity last
        // stamped" and `Bubble.moved_k` reads "TUniversal when this bubble's
        // charts last moved"; this stamped both with
        // `session.voice().k()` — `Seat::k` is `chain.next_step_index()`,
        // this voice's own receipt index, which advances only when THIS
        // voice seals a receipt. `absorb_taught` states the rule in one
        // line: "k is the voice's K (law 9), tu is TUniversal; they are not
        // one clock."
        //
        // Everything downstream of `now_k` is a span in TUniversal:
        // `held_since(now_k, moved_k)` is `pivot_orbit`'s `t_extent`,
        // `span_of` is lane 𝒯 of a minted word (R2), and `retire_on_t` is
        // the T-room. All three were reading a span between two stamps of
        // the wrong clock.
        let tu = self.now_tu();
        self.contemplation.set_now(tu);
        let crate::transducer::bubble::drive::Expanded {
            pivoted,
            crystallized,
            circuit,
            adjoined,
            ..
        } = {
            let epigenome = &self.epigenome;
            let atlas = epigenome.working_atlas();
            // The held word's SECTION, both halves off the one element:
            // `rotor_of` stood here and the carriage never reached the
            // cover, so every transition classified as if the word sat at
            // the origin.
            let section_at = |a: &[u8]| {
                epigenome
                    .working_word(a)
                    .and_then(|h| crate::transducer::cech::section_of(h.element()))
            };
            self.contemplation
                .expand_all(&atlas, &section_at, epigenome, &mut budget)
        };
        // 37.7 — ADJOIN WHAT THE ALPHABET CANNOT SPELL. A cover meet that
        // produced a rotor the seventeen have no letter for is the one place
        // a mind can CONSTRUCT a generator rather than receive one, and it
        // was dropped at three sites. Measured across the fleet: every Word
        // vertex and every edge transport of every admitted form carries the
        // identity — `turning 0` and `turning_edges 0`, totals, over 38 and
        // 44 admits — so `crystal.on_axis` answered 0 on every read and the
        // pivot was never offered anything. It could not be: nothing in the
        // tree minted one.
        //
        // The generator stands in the DIALECT, not the genome: it is this
        // mind's own, memory only, until a court rules it (24.1). That is
        // also why it does not suppress a demand — `pivot_orbit` defers on
        // `Standing::Ruled` alone — so the mind adjoins the letter AND keeps
        // asking the mesh for one, and whichever arrives first closes the
        // loop.
        self.adjoin_letters(&adjoined);
        for e in &crystallized {
            self.stand_crystallized(e);
        }
        (pivoted, circuit)
    }

    /// 37.7 — MINT AND STAND WHAT THE PIVOT COULD NOT SPELL.
    ///
    /// ONE READER FOR BOTH PATHS. The drive runs `pivot_orbit` from two
    /// places — the expand ring and `pivot_on` — and this was written into
    /// the first only, so a stroke that pivoted through the second adjoined
    /// nothing. Measured at 1d98080: `adjoin from residue` 8, every drain
    /// outcome 0. Two copies of a mint is two chances for one path to mint
    /// what the other does not (law 12).
    ///
    /// The letter stands in the DIALECT: it is this mind's own, memory only,
    /// until a court rules it (24.1), which is also why it does not suppress
    /// the demand — `pivot_orbit` defers on `Standing::Ruled` alone since R11.
    pub(crate) fn adjoin_letters(&mut self, adjoined: &crate::transducer::bubble::drive::Adjoined) {
        for (rotor, chart) in adjoined {
            // WHY A CANDIDATE MINTS NOTHING (§8.1, measurement only). Measured
            // over one recut: `adjoin from residue` 68, `letter adjoined` 0.
            // The pivot carries the candidate out sixty-eight times and the
            // language gains nothing, and the two silent `continue`s below
            // are the only places it can go. They are different facts — a
            // rotor that cannot be minted at all, and one the mind already
            // stands — and neither said so.
            let Some(e) = crate::transducer::sentence::generator_form(rotor, chart, &[]) else {
                eprintln!("  letter refused: the rotor mints no generator");
                continue;
            };
            let addr = e.address();
            if let Some(st) = self.epigenome.standing_of(addr.as_bytes()) {
                // Not a defect on its own: the same class is adjoined every
                // stroke it is measured, so after the first the letter is
                // held and there is nothing to add. It IS the reading that
                // says whether the one letter ever became a word — a class
                // stood once and re-offered forever is a vocabulary of one
                // that never ruled.
                eprintln!(
                    "  letter held {:02x}{:02x}… already {st:?}",
                    addr.as_bytes()[0],
                    addr.as_bytes()[1]
                );
                continue;
            }
            self.stand_crystallized(&e);
            eprintln!(
                "  letter adjoined {:02x}{:02x}… on chart {:?}",
                addr.as_bytes()[0],
                addr.as_bytes()[1],
                chart.0
            );
        }
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
            // One sentence was sent, so the reply is one code.
            Ok(codes) if matches!(codes.as_slice(), [dialogue::SPEECH_REFUSED]) => {
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
        // TUniversal, not the voice's K — see `expand_ideas`.
        let tu = self.now_tu();
        self.contemplation.set_now(tu);
        let (_fills, _formed, _classes, spoken, adjoined) =
            self.contemplation.pivot_on(word, &self.epigenome, &mut budget);
        // The other path a pivot runs on, minting through the same reader.
        self.adjoin_letters(&adjoined);
        let mut n = 0usize;
        for e in spoken {
            if let Ok(Utterance::Spoken) = self.speak_word(&e) { n += 1 }
        }
        n
    }

    /// PIVOT THE WORDS THE PIVOT HAS NOT REACHED — a set of coordinates,
    /// drained.
    ///
    /// It was "every genome word added since last stroke", read first off
    /// a vector's tail and then off an admission ordinal. Both answer a
    /// question about arrival; the pivot's question is which obstructions
    /// are undischarged, and that is a set at coordinates.
    pub fn pivot_thoughts(&mut self) -> usize {
        let added: Vec<Element> = self.epigenome.take_unpivoted();
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
        // DOOR 1 (25): the addresses the dialect just dropped have lapsed
        // from the working atlas, so the bubbles standing at them and the
        // bubbles citing them lapse on THIS stroke, jointly. `dropped` is
        // the signal and it is already in hand; no sweep discovers these
        // later.
        let lapsed = self.contemplation.lapse_with(&dropped);
        // Z4d: AND THE CLOCK'S OWN LAPSE. A thought whose paid height
        // TUniversal has passed leaves here too, as a set, with its escrow
        // returned per lane. Door 1 answers for what a thought CITES;
        // this answers for what it was PAID, and a thought that cites
        // nothing standing still has a horizon.
        let expired = self.lapse_held();
        dropped
            .len()
            .saturating_add(lapsed)
            .saturating_add(expired)
            .saturating_add(self.contemplation.shed_unglued())
    }

    /// Z4b's instrument: why the shed declined each held thought, by the
    /// term that rejected it — glued_by, glues, dirty, sheddable. A
    /// measurement (§8.1), read on the stroke line so the holds column is
    /// attributed and never a branch.
    pub fn shed_census(&self) -> [usize; 4] {
        self.contemplation.shed_census()
    }

    /// Z4d: what this voice can still escrow, per lane. Its attention.
    pub fn reserve(&self) -> [u64; 4] {
        self.physics.reserve
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

    /// DOOR 2 OF CHAPTER 25 — RETIREMENT AT PROMOTION.
    ///
    /// A form ratified into the genome is held by the atlas; the bubble
    /// still standing for it in the contemplation is a SECOND CARRIER FOR
    /// ONE FACT (law 12), and it is retired here, at the promotion, rather
    /// than left for a later sweep to notice.
    ///
    /// THE RETIREMENT RIDES THE ONE DRAIN. `take_promoted` empties the
    /// list, so a second reader draining it would find nothing; the
    /// retirement happens where the drain does and the caller still gets
    /// the addresses it logs.
    ///
    /// It goes through door 1's `lapse_with`, because a promoted address is
    /// an address that has left the contemplation's keeping: whatever cited
    /// it cited a bubble that no longer stands here, and it lapses with it
    /// on the same stroke.
    pub fn take_promoted(&mut self) -> Vec<Vec<u8>> {
        let promoted = self.epigenome.take_promoted();
        if !promoted.is_empty() {
            let n = self.contemplation.lapse_with(&promoted);
            if n > 0 {
                eprintln!("  promotion retired {n} held thoughts");
            }
        }
        promoted
    }

    /// Working incidence: a non-empty fiber on the working atlas.
    pub fn incident_chart(&self, c: &crate::transducer::stalk::Invariant) -> bool {
        !self.working_atlas().fiber(c).is_empty()
    }

    pub fn ideas_len(&self) -> usize {
        self.contemplation.len()
    }

    /// CS2 — the contemplation's pivots against the cells their orbits
    /// cross, by content (78.32(c), note only).
    pub fn pivot_reading(&self) -> crate::transducer::sentence::PivotTally {
        self.contemplation.pivot_reading()
    }

    /// 13.12 — the contemplation's merges, merged / stayed two / unnamed.
    pub fn merge_reading(&self) -> crate::transducer::bubble::merge::MergeTally {
        self.contemplation.merge_reading()
    }

    /// 78.37(1), TP1 — the contemplation's meet graph as it stands, with its
    /// decoherences since the contemplation began.
    pub fn meet_graph(&self) -> crate::transducer::bubble::MeetGraph {
        let mut m = self.contemplation.meet_graph();
        // RM1 — the remainders the epigenome's index could not file over one
        // standing, beside the contemplation's own.
        m.remainders.coincident = self.epigenome.remainders_coincident;
        // And the receptor's: nested, discordant, the calluses.
        let (e, c) = (&self.epigenome, &self.epigenome.calluses);
        (m.remainders.nested, m.remainders.discordant) = (e.remainders_nested, e.remainders_discordant);
        m.remainders.calluses = c.stood;
        m.remainders.unspelled = c.unspelled;
        m.remainders.untopped = c.untopped;
        m.remainders.uncited = c.uncited;
        m.remainders.callus_unminted = c.unminted;
        m
    }

    /// THE CAVITY'S SIZE, on the wire (§8.1). `retire_on_t` writes one entry
    /// per retirement, every stroke, and the only drain is `pivot_on`'s —
    /// which fires when an incoming word's chart prefix happens to match a
    /// cavity entry's, so it is coincidence and not policy. `cavity_len` has
    /// had no caller outside its own tests since it was written, so a
    /// carrier that grows on the hot path has never been readable.
    ///
    /// Measured beside it: the voices hold 786 MB to 1565 MB each, 105
    /// minutes after a recut, against `ideas 78`, `genome 144` and
    /// `dialect 31` — every counted carrier flat. This is the one carrier on
    /// the stroke path with no bound and no reading, which makes it the
    /// candidate and not the cause; this line is what decides between them.
    pub fn cavity_len(&self) -> usize {
        self.contemplation.cavity_len()
    }

    /// THE LINKS THE CAVITY HOLDS, which is what it costs. A cavity entry is
    /// a whole remainder — `Vec<Link>` — so the entry COUNT says how many
    /// retirements happened and this says how much they kept. A thousand
    /// entries of one link and ten of a thousand are the same count and not
    /// the same carrier (rule 2 — the count does not decide).
    pub fn cavity_links(&self) -> usize {
        self.contemplation.cavity_links()
    }

    /// The carriers a bubble holds that no count reports — sections, nerve
    /// simplices, horizon classes, bundle pairs, certificates. See
    /// [`crate::transducer::bubble::Contemplation::weight`].
    pub fn idea_weight(&self) -> [usize; 5] {
        self.contemplation.weight()
    }

    /// WHAT THE ENTITY HOLDS THAT NO LINE REPORTS (§8.1, measurement only):
    /// the utterance receipts (`taught_up`), the obstructions fetched from
    /// the court, the citations owed and the contractions pending — each
    /// carrying a sentence and its train — the sealed and the contracted
    /// addresses, the words expressed with their index, the recollections,
    /// the charts waited on, and the stroke marks. Counts and never bytes;
    /// what they say is WHICH carrier grows while every counted one is
    /// flat, beside [`Entity::idea_weight`], which says it for the ideas.
    /// Z4b term 2 / the growth's own instrument — EVERY CARRIER THIS
    /// ENTITY OWNS, ITS KEY BYTES AND ITS ENTRIES, NEVER A SUM.
    ///
    /// "Named before shaved": four rows were aimed at the holds on the
    /// reading that the holds were the growth, and the fleet then ran ten
    /// minutes with holds at zero and grew +157 MB/min anyway. A total
    /// cannot say which carrier moved; a vector over carriers can, and a
    /// vector is what law 5 says a measurement of several things is.
    ///
    /// Key bytes and entry count are reported APART for each carrier. A
    /// form's address is megabytes, so a carrier can grow in bytes without
    /// gaining an entry (a bigger form held) or gain entries without bytes
    /// (small keys), and one number could not tell those apart.
    /// A shape cache's own bytes: every blade its `OnceLock` has filled.
    /// An unfilled slot is the header alone, which is what it costs.
    fn bytes_of_shape(
        c: &std::sync::OnceLock<Option<crate::transducer::blade::Shape>>,
    ) -> usize {
        use crate::transducer::bytes::Bytes;
        std::mem::size_of_val(c) + c.get().and_then(Option::as_ref).map_or(0, |s| s.blade.bytes())
    }

    pub fn carrier_bytes(&self) -> Vec<(&'static str, usize, usize, usize)> {
        use crate::transducer::bytes::Bytes;
        let c = &self.contemplation;
        let e = &self.epigenome;
        let held_w = |k: &crate::network::nerve::Keyed<crate::transducer::element::Element>| -> usize {
            k.scoped().map(|(_, v)| v.bytes()).sum()
        };
        vec![
            ("taught_up", self.taught_up.key_bytes(), held_w(&self.taught_up), self.taught_up.len()),
            ("taught_at", self.taught_at.key_bytes(), self.taught_at.scoped().map(|(_, m)| m.key_bytes()).sum(), self.taught_at.len()),
            ("obstructions", self.obstructions.key_bytes(), held_w(&self.obstructions), self.obstructions.len()),
            ("glued", self.glued.key_bytes(), 0, self.glued.len()),
            ("opened", self.opened.key_bytes(), 0, self.opened.len()),
            ("sealed", self.sealed.key_bytes(), 0, self.sealed.len()),
            ("contractions", self.contractions.key_bytes(), 0, self.contractions.len()),
            ("contracted", 0, 0, self.contracted.len()),
            ("owed", 0, 0, self.owed.len()),
            // THE GENOME IS WHERE LONG-TERM MEMORY LIVES BY DESIGN and it
            // holds DECODED forms: its weight is the whole of each `Held`,
            // tape and form together, which is exactly what the previous
            // instrument read as zero.
            // Z11's TWO HALVES, READ APART. `epi.index` is what stays
            // resident when the genome moves to disk — the addresses as the
            // trie path the words live at, plus each entry's expression and
            // ordinal. `epi.resident` is the payload the row moves out.
            // One number for both could not say which half Z11 removes.
            ("epi.index", e.genome.key_bytes(), e.genome.len() * 16, e.genome.len()),
            ("epi.resident", 0, e.resident.scoped().map(|(_, w)| w.held.bytes() + 16).sum(), e.resident.len()),
            ("epi.dialect", e.dialect.key_bytes(), e.dialect.scoped().map(|(_, d)| d.sentence.bytes()).sum(), e.dialect.len()),
            ("epi.expressed", e.expressed.key_bytes(), e.expressed.scoped().map(|(_, x)| x.word.bytes() + 16).sum(), e.expressed.len()),
            ("epi.recollections", e.recollections.key_bytes(), e.recollections.scoped().map(|(_, (v, _))| v.iter().map(Vec::len).sum::<usize>()).sum(), e.recollections.len()),
            ("epi.promoted", e.promoted.iter().map(Vec::len).sum(), 0, e.promoted.len()),
            // Z10, the vector completed: the atlases, the shape caches, the
            // axis index and the moduli index. The first table reached 45%
            // of the allocator's figure and these are what it did not list.
            ("epi.atlas", 0, e.atlas.bytes(), e.atlas_len),
            ("epi.dialect_atlas", 0, e.dialect_atlas.bytes(), 0),
            ("epi.expressed_atlas", 0, e.expressed_atlas.bytes(), 0),
            // The shape caches are no longer carriers of their own: each
            // word's shape is lifted under the word it measures.
            ("epi.shapes", 0, e.resident.scoped().map(|(_, w)| Self::bytes_of_shape(&w.shape)).sum(), e.resident.len()),
            ("epi.axis_index", e.axis_index.key_bytes(), e.axis_index.scoped().map(|(_, a)| a.key_bytes() + a.scoped().map(|(_, w)| w.key_bytes() + w.scoped().map(|(_, x)| x.rotor().bytes()).sum::<usize>()).sum::<usize>()).sum::<usize>(), e.axis_index.len()),
            ("epi.by_moduli", e.by_moduli.key_bytes(), e.by_moduli.scoped().map(|(_, m)| m.key_bytes()).sum(), e.by_moduli.len()),
            ("epi.waiting_on", e.waiting_on.key_bytes(), e.waiting_on.scoped().map(|(_, m)| m.key_bytes()).sum(), e.waiting_on.len()),
            ("epi.stroke_k", 0, e.stroke_k.len() * 8, e.stroke_k.len()),
            ("con.bubbles", c.bubbles.key_bytes(), c.bubbles.scoped().map(|(_, b)| b.form.as_ref().map_or(0, Bytes::bytes)).sum(), c.bubbles.len()),
            ("con.by_chart", c.by_chart.key_bytes(), c.by_chart.scoped().map(|(_, m)| m.key_bytes()).sum(), c.by_chart.len()),
            ("con.cited", c.cited.key_bytes(), c.cited.scoped().map(|(_, m)| m.key_bytes()).sum(), c.cited.len()),
            ("con.ordinals", c.ordinals.key_bytes(), 0, c.ordinals.len()),
            ("con.addresses", c.addresses.iter().flatten().map(|a| a.len()).sum(), 0, c.addresses.len()),
            ("con.dirty", c.dirty.key_bytes(), 0, c.dirty.len()),
            ("con.cavity", c.cavity.key_bytes(), c.cavity.scoped().map(|(_, v)| v.len() * std::mem::size_of::<crate::transducer::sentence::Link>()).sum(), c.cavity.len()),
            ("con.complex", c.complex.sections.key_bytes(), 0, c.complex.sections.len()),
        ]
    }

    pub fn carriers(&self) -> [usize; 10] {
        [
            self.taught_up.len(),
            self.obstructions.len(),
            self.owed.len(),
            self.contracted.len(),
            self.sealed.len(),
            self.contractions.len(),
            self.epigenome.expressed.len(),
            self.epigenome.recollections.len(),
            self.epigenome.waiting_on.len(),
            self.epigenome.stroke_k.len(),
        ]
    }

    pub fn shed_n(&self) -> u64 {
        self.epigenome.shed_n()
    }

    /// WHAT THE SESSION HOLDS OF THE MESH, as counts (§8.1, L33): the
    /// carriers below the entity, read on the stroke line beside
    /// [`Entity::carriers`]. See [`Session::sizes`].
    pub fn session_sizes(&self) -> [usize; 9] {
        self.session.sizes()
    }

}
