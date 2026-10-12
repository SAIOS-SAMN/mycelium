// 25 — THINKING: what this entity holds under contemplation, and what it
// sheds.

use crate::client::sdk::*;

impl Entity {
    /// HOLD A CHAIN AS THE FORM IT IS (50.6 seam 1): a demand this mind has
    /// not closed is held as an idea — the pair form of the scene the chain
    /// describes, its painted sites before against its painted sites after in one frame —
    /// never as a bare chain minted into a word. An idea is a form; its
    /// words are the vertices the eyes could pair and the alphabet could
    /// spell, and what they could not is its demand vertices (50.5).
    pub(crate) fn hold_chain(&mut self, chain: Vec<(i64, i64, u8, u8)>) {
        use crate::transducer::family::{extraction_spend, perceive_family, scene_form_on, scene_of_chain};
        let Some(scene) = scene_of_chain(&chain) else {
            return;
        };
        // 50.2 step 2: this mind's own presentation is what its word
        // vertices derive against, under a stroke sized by the scene it is
        // handed (L8w); a fixed spend unit does not reach one scene. The
        // scene's frames are perceived once: the stroke is sized by them and
        // the percept reads them.
        let frames = perceive_family(std::slice::from_ref(&scene));
        let mut budget = self.physics.stroke_n(extraction_spend(std::slice::from_ref(&scene), &frames));
        // `None` is a vertex the stroke could not factor through this
        // mind's presentation: unmeasured, so nothing is held rather than
        // a form held with the presentation's reach asserted unread.
        let Some(f) = scene_form_on(&self.epigenome, &mut budget, &scene.0, &scene.1, &frames) else {
            return;
        };
        // A perception, held with the bodies it was perceived as (N4); a
        // remainder scene is named by no task. Held with no bundle, weighed
        // on the scene it was perceived from.
        self.hold_perceived_from(f, &frames, (&[][..], std::slice::from_ref(&scene)), &[], &[], &mut budget);
    }

    /// N4 — HOLD A PERCEPTION with the frames it was perceived from, so the
    /// thoughts it forms with the perceptions it meets are the transitions
    /// between their bodies. The draw is `hold_form`'s, one reading of it.
    /// `family` is the task the perception was seen in (78.56 III: its
    /// thoughts wind only within it); empty where none is named.
    pub(crate) fn hold_perceived(
        &mut self,
        f: crate::transducer::sentence::Form,
        frames: &crate::transducer::family::Frames,
        train: &crate::transducer::family::Family,
        remainders: &[Vec<(i64, i64, u8, u8)>],
        family: &[u8],
        budget: &mut crate::physics::Budget,
    ) -> bool {
        self.hold_perceived_from(f, frames, (train, train), remainders, family, budget)
    }

    /// [`Self::hold_perceived`], with the SCENE the form was perceived from
    /// named apart from the training bundle it is held with: an hour of a
    /// family is perceived from its own pair and held with no bundle
    /// (78.56 III), and what it says is weighed on that pair (78.102).
    pub(crate) fn hold_perceived_from(
        &mut self,
        f: crate::transducer::sentence::Form,
        frames: &crate::transducer::family::Frames,
        (train, scene): (&crate::transducer::family::Family, &crate::transducer::family::Family),
        remainders: &[Vec<(i64, i64, u8, u8)>],
        family: &[u8],
        budget: &mut crate::physics::Budget,
    ) -> bool {
        // A PERCEPTION IS ITS BODIES. An extraction memo hit hands no frames
        // (the family was perceived, and held, when it was first
        // extracted), and holding that form again held a thought with no
        // bodies — a perception of nothing, filed beside the real one on
        // every re-dispatch of the task (the cognition audit, 2026-09-29).
        if frames.is_empty() {
            return false;
        }
        let Some(draw) = self.draw_for(&f) else { return false };
        // 78.100 D1 — the words this perception's sentences taught, transcribed
        // before the form is handed over: spelled, each is spoken below.
        let (taught, ineffable) = self.transcribe_taught(budget);
        // 78.102, the letter carrier's voice half — A WORD NO HELD LETTER
        // SPELLS IS SPOKEN AS THE SENTENCE THAT TAUGHT IT: the perceived form
        // whose ring taught it goes to the court as an open sentence, where
        // its rings are re-derived and the letter they need offered
        // (`hear_open`, the speaking voice its discoverer). Never the bare
        // rotor: the court replays, it never takes a voice's word for it.
        //
        // SIZED AS THE VOICE'S SPEECH SIZES A FORM: what it closes over the
        // scene it was perceived from (`form_deflation`), and where that is
        // nothing, the escrow it was held at. It was the escrow alone — the
        // remainder's measure — and a perceived form that closed everything
        // it saw holds no remainder: a zero magnitude, which
        // `as_element_form` refuses (epoch 133). Then over the training
        // bundle — and an hour is held with none, so it weighed nothing
        // again (epoch 134: `open sentences spoken` 0 on every voice). The
        // hour's scene is its own pair.
        // 78.109 K1 — the chart of the form that taught them, where the mind
        // files the letters it makes.
        let chart = crate::transducer::sentence::chart_of_form(&f);
        let open = match !ineffable.is_empty() {
            true => {
                let (closed, _) = crate::transducer::family::form_deflation(&f, scene, &self.epigenome);
                let mag = match closed.iter().any(|&x| x > 0) {
                    true => closed,
                    false => draw,
                };
                crate::transducer::sentence::as_element_form(&f, mag)
            }
            false => None,
        };
        self.contemplation.hold_perceived(f, frames, train, remainders, draw, family, budget);
        // 78.112 H7a — what the hold released vents into the mind's field.
        self.vent_into_field(budget);
        // 78.52 — A CLOSED THOUGHT SPEAKS IN THE ACT THAT CLOSED IT: the thoughts
        // this perception closed are transcribed against this mind's own
        // letters, and each word stands in the dialect and goes to the court
        // now, never on some later word's arrival. What the wire answers is
        // the speech's own reading (`Utterance`); a refusal there stands as
        // told.
        for word in self.transcribe_closed(budget) {
            let _ = self.speak_word(&word);
        }
        // 78.100 D1 — and the words its sentences taught enter its language
        // the same way: transcribed, stood, spoken.
        for word in taught {
            let _ = self.speak_word(&word);
        }
        if let Some(sentence) = open {
            self.speak_with(std::slice::from_ref(&sentence), train, family, |_, _| {});
        }
        // 78.109 K1 — THE MIND MAKES ITS LETTER. A word its own sentence taught
        // and no held letter spells is adjoined in its local dialect at once,
        // as the pivot's unspellable rotor is (`adjoin_letters`, 37.7): its
        // own letters now spell it, D3 cites it, words climb on it. Volatile,
        // the mind's own, lapsed on its own clock; said to other minds as
        // chatter, which they re-derive and agree or differ on (78.161), never
        // taken into their genomes. What it offers the mesh is the sentence
        // above; whether the mesh holds it too is the court's to verify.
        // 78.109 C1 — through the one road a residue takes: constructed,
        // certified, then adjoined.
        for residue in &ineffable {
            self.construct_and_adjoin(residue, chart.as_ref(), budget);
        }
        true
    }

    /// 78.112 H7a — THE VOICE'S VENT REACHES ITS FIELD. A membrane released or
    /// a tether cauterized in the mind's complex vents its strain's reverse
    /// (78.106), and the valve opened onto nothing: only a court armed its
    /// complexes' outflow, so at the voice the released torsion was dropped.
    /// It enters the voice's field now by the door the court's outflow takes
    /// (`mark_flux`, the voices' one field reader, law 12): null flux into the
    /// tubes, turns and boosts onto their orbits, each motion weighing one.
    /// Returns how many motions vented. The complex listens from here on —
    /// armed where it is taken, so a complex read back from disk listens too.
    ///
    /// MU-CLIENT §4.5 — EACH FLOW ON ITS LINE: a tether condensed or a face
    /// sealed winds its strain in and threads what its modes land on; a
    /// tether drained or a membrane released unwinds it and leaves them. What
    /// the field RELEASED in admitting them — the lines on an orbit that
    /// returned, on a tube whose net flux cancelled — is the tension that
    /// relaxed, and the complex lets those lines go (`Contemplation::release`):
    /// the field's events, never a poll.
    pub(crate) fn vent_into_field(&mut self, budget: &mut crate::physics::Budget) -> usize {
        use crate::transducer::family::pairing::{mark_flux, FieldLine};
        self.contemplation.complex.arm_outflow();
        let lines: Vec<FieldLine> = self
            .contemplation
            .complex
            .take_outflow()
            .into_iter()
            .map(|f| FieldLine { carriage: f.motion, gauge: Vec::new(), mass: 1, free: false, line: f.line })
            .collect();
        if !lines.is_empty() {
            mark_flux(&[], &lines, budget);
        }
        if let Ok(released) = crate::transducer::mind::with_field(|field| field.take_released()) {
            self.contemplation.release(&released);
        }
        lines.len()
    }

    /// 78.109 C1, 78.47 — A RESIDUE THE MIND CANNOT SAY, CONSTRUCTED AND MADE
    /// ITS OWN. The residue is spelled as constructors (`cga::construct`,
    /// constructive Cartan–Dieudonné in the algebra's own metric, each mode
    /// certified on its fixed locus, the whole composing back, T1): a
    /// congruent thought, or a refusal counted by why. Constructed, its
    /// rotation is adjoined as the mind's own letter on its chart
    /// (`adjoin_letters`, the pivot's path); a residue whose motion holds no
    /// turn is counted apart — it constructs, and no letter type holds it yet
    /// (the ledger's row 8 reads this).
    pub(crate) fn construct_and_adjoin(
        &mut self,
        residue: &crate::transducer::cga::Mv,
        chart: Option<&crate::transducer::stalk::Invariant>,
        budget: &mut crate::physics::Budget,
    ) {
        use crate::transducer::cga::{construct, modes_need};
        // 78.111 C1b — the stroke the residue's modes need, read off the
        // residue: its construction is never refused for a stroke sized for
        // something else. Where there is none to read (no versor, or a need
        // past the carrier) the stroke is empty and `construct` refuses it by
        // name.
        let mut need = match modes_need(residue) {
            Some(n) => self.physics.stroke_n(n),
            None => self.physics.stroke_n(0),
        };
        if construct(residue, &mut need).is_err() {
            return;
        }
        let rotor = crate::transducer::element::conformal_factors(residue)
            .ok()
            .and_then(|f| crate::transducer::element::rotor_of_factors(&f).ok())
            .filter(|r| !r.projective_eq(&crate::transducer::alphabet::Rotor::identity()));
        match (rotor, chart) {
            (Some(r), Some(c)) => {
                self.adjoin_letters(&vec![(r, c.clone())], budget);
                self.contemplation.own_letter(residue);
            }
            // 78.68 ROW 8 — A RESIDUE THAT TURNS NOTHING IS STILL A LETTER: its
            // whole motion on the letter's carriage (a slide, a dilation about
            // a point), stood in the mind's dialect and spoken — the court
            // verifies and hardens it or refuses by name (C4) — and kept, so
            // the cell that demanded it is answered by it from here on.
            (None, Some(c)) => {
                if let Some(e) = crate::transducer::sentence::motion_letter_form(residue, c, &[]) {
                    self.contemplation.own_letter(residue);
                    if !self.already_taught(&e) {
                        self.stand_crystallized(&e);
                    }
                }
            }
            (_, None) => {}
        }
    }

    /// 78.109 C1 — THE DEMAND TAKER: each demand the mind's pivot docketed
    /// (an undischarged orbit's class with its whole residue) taken once and
    /// constructed — the residue on the first chart of its cover — where only
    /// a discharge answered one before (VN1).
    pub(crate) fn take_demands(&mut self, budget: &mut crate::physics::Budget) {
        for (residue, charts) in self.contemplation.take_demands() {
            // The chart its letter is filed on: the cover's least by its own
            // coordinate — content, never an arrival (rule 15).
            let least = charts.iter().fold(None, |least: Option<&crate::transducer::stalk::Invariant>, c| match least {
                Some(l) if l.key() <= c.key() => Some(l),
                _ => Some(c),
            });
            if let Some(r) = residue {
                self.construct_and_adjoin(&r, least, budget);
            }
        }
    }

    /// 37.7 — THE DOCKET TAKEN ON EVERY STROKE, on the stroke's own budget:
    /// a demand docketed is taken and constructed where it stands, not only
    /// on a short or idle stroke (`expand_ideas`). A mind on the fleet
    /// docketed 471 demands and took 1 (epoch 146): its reserve never fell
    /// short, so the syntropic phase that alone took them never ran.
    pub(crate) fn take_docketed_demands(&mut self) {
        let mut budget = self.physics.stroke();
        self.take_demands(&mut budget);
        self.speak_unheard(&mut budget);
    }

    /// 78.52 — TRANSCRIPTION, AGAINST THIS MIND'S OWN LETTERS (78.47 (i)). A
    /// closed thought's generator factors into rotation and carriage; the
    /// rotation is read through the mind's crystal, adjoined letters
    /// included (`letter_reading`), never the fixed seventeen alone:
    /// - SPELLED by the seventeen: the word is that spelling's connection at
    ///   that carriage, carrying the return (`ℓ − 1` on the shell) — spoken.
    /// - DERIVABLE only through a letter this mind adjoined: that letter
    ///   already stands in the dialect and says it; nothing new is spoken.
    /// - UNSPELLED by every held letter: the thought is INEFFABLE — kept, and
    ///   nothing emitted; the same class reads derivable once a letter for it
    ///   is adjoined.
    /// - No rotation to read: unread.
    pub(crate) fn transcribe_closed(&mut self, budget: &mut crate::physics::Budget) -> Vec<Element> {
        use crate::transducer::sentence::rung::Transcribed;
        let mut words = Vec::new();
        for ground in self.contemplation.take_closed() {
            if let (Transcribed::Spelled, Some(word)) = self.transcribe(&ground.generator, ground.period, budget) {
                words.push(word);
            }
        }
        words
    }

    /// 78.100 D1 — THE WORDS A THEOREM TAUGHT, ENTERING THE LANGUAGE. Each
    /// word the mind's sentences taught since the last take is transcribed
    /// by the one transcriber a closed thought is (`transcribe`): spelled, it
    /// is the word the voice stands in its dialect and speaks — so it can be
    /// cited, adjudicated, and taught — exactly as a closed thought's word
    /// is; derivable only through an adjoined letter, it already stands;
    /// ineffable, a letter demand (37.7). Counted on the sentences row; and
    /// the ineffable words' rotors — what the mind adjoins as its own
    /// letters (78.109 K1), and why its sentence is spoken (78.102).
    pub(crate) fn transcribe_taught(&mut self, budget: &mut crate::physics::Budget) -> (Vec<Element>, Vec<crate::transducer::cga::Mv>) {
        self.transcribe_these(crate::transducer::sentence::rung::take_taught(), budget)
    }

    /// [`Self::transcribe_taught`] on the taught words given: the spelled
    /// words, and the ineffable words' whole motions — the residues the mind
    /// constructs and adjoins (78.109 C1).
    pub(crate) fn transcribe_these(
        &mut self,
        taught: Vec<crate::transducer::sentence::rung::Taught>,
        budget: &mut crate::physics::Budget,
    ) -> (Vec<Element>, Vec<crate::transducer::cga::Mv>) {
        use crate::transducer::sentence::rung::{Taught, Transcribed};
        let mut words = Vec::new();
        let mut ineffable = Vec::new();
        for Taught { motion, period } in taught {
            match self.transcribe(&motion, period, budget) {
                (Transcribed::Spelled, Some(word)) => words.push(word),
                // Its whole motion: the residue the mind constructs.
                (Transcribed::Ineffable, _) => ineffable.push(motion.clone()),
                _ => {}
            }
        }
        (words, ineffable)
    }

    /// 78.52, 78.100 D1 — THE ONE TRANSCRIBER: a motion's generator factored
    /// into rotation and carriage; the rotation read through this mind's own
    /// letters (`letter_reading`, adjoined letters included); spelled by the
    /// seventeen, the word is that spelling's connection at that carriage,
    /// carrying the return (`ℓ − 1` on the shell).
    fn transcribe(
        &mut self,
        generator: &crate::transducer::cga::Mv,
        period: u64,
        budget: &mut crate::physics::Budget,
    ) -> (crate::transducer::sentence::rung::Transcribed, Option<Element>) {
        use crate::transducer::family::{letter_reading, LetterReading};
        use crate::transducer::sentence::rung::Transcribed;
        let factored = crate::transducer::element::conformal_factors(generator)
            .ok()
            .and_then(|f| crate::transducer::element::rotor_of_factors(&f).ok().map(|r| (f, r)));
        let Some((f, rotor)) = factored else { return (Transcribed::Unread, None) };
        match letter_reading(&self.epigenome, budget, &rotor) {
            LetterReading::Derivable => {
                let spelled = match crate::transducer::alphabet::factor_rotor(&rotor) {
                    Some(Ok(word)) => period.checked_sub(1).and_then(|beyond| {
                        crate::transducer::element::from_connection(&word, &f.carriage(), &[], &Vec::new(), [0, 0, 0, beyond])
                    }),
                    _ => None,
                };
                match spelled {
                    Some(word) => (Transcribed::Spelled, Some(word)),
                    None => (Transcribed::Adjoined, None),
                }
            }
            LetterReading::Unspelled => (Transcribed::Ineffable, None),
            LetterReading::Unread | LetterReading::NoVersor => (Transcribed::Unread, None),
        }
    }

    /// Door 3's draw for a thought: its four-lane measure out of the reserve,
    /// lane by lane, or `None` where a lane cannot fund it (rule 11).
    fn draw_for(&mut self, f: &crate::transducer::sentence::Form) -> Option<[u64; 4]> {
        let draw = crate::transducer::element::measure_lanes(&f.remainder.placed());
        // All or nothing, through the metabolic ledger (65.5): a lane that
        // cannot fund it refuses the whole draw by name, and the attention it
        // blocked presses that lane (78.73 (1)).
        self.physics.draw(draw).ok()?;
        Some(draw)
    }

    /// Hold a withheld form as an idea so the pivot orbit sees it next
    /// stroke (30.2). The contemplation is the one carrier of held forms
    /// (24.1 as ruled 2026-09-09): its remainders stay on the bubble as
    /// what the form does not speak, and its words are read at a chart
    /// when a cite asks — through the form's own `by_class` — never
    /// listed here ahead of the question.
    /// Z4 item 1: the form is MOVED into the bubble. A caller that needs
    /// it afterwards reads it back from the contemplation by reference.
    pub(crate) fn hold_form(
        &mut self,
        f: crate::transducer::sentence::Form,
        train: &crate::transducer::family::Family,
        remainders: &[Vec<(i64, i64, u8, u8)>],
        budget: &mut crate::physics::Budget,
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
        let Some(draw) = self.draw_for(&f) else { return false };
        // 65.5: the caller's stroke — a hold is part of the act that holds.
        self.contemplation.hold_form(f, train, remainders, draw, budget);
        true
    }

    /// Z4d — THE STROKE'S LAPSE: every thought the clock has passed leaves,
    /// as a set, and its escrow returns to the reserve LANE BY LANE.
    ///
    /// A thought held before any aperture was measured is paid through one
    /// here, on the first stroke that can measure a horizon, so it is
    /// neither held against a height nothing measured nor held forever for
    /// want of one (rule 15).
    pub(crate) fn lapse_held(&mut self) -> usize {
        let _ = self.contemplation.repay_unpaid();
        let (returned, n) = self.contemplation.lapse_on_clock();
        // A return past a lane's carrier is refused and counted on the
        // metabolic ledger, the lane standing where it was (STD1).
        self.physics.reserve.restore(returned);
        // TC1 — and what the truncation removed since, returned alike.
        let truncated = self.contemplation.take_truncated_escrow();
        self.physics.reserve.restore(truncated);
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
    #[cfg(test)]
    pub(crate) fn room_for(&self, f: &crate::transducer::sentence::Form) -> bool {
        let want = crate::transducer::element::measure_lanes(&f.remainder.placed());
        // The reserve already excludes what is escrowed on held thoughts
        // (Z4d), so the room on a lane is the reserve on it and the check
        // is the draw's own, asked without making it.
        self.physics.reserve.lanes().iter().zip(&want).all(|(r, d)| r.checked_sub(*d).is_some())
    }

    /// LANE 𝒯 FOR A FORM THIS MIND IS ABOUT TO SPEAK: the span in receipts
    /// its idea has stood over. The contemplation is the one carrier of held
    /// forms and the one reader of the span (law 12); a speaker asks it here
    /// rather than reaching into it.
    pub(crate) fn span_of(
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
    #[cfg(test)]
    pub(crate) fn withhold_form(
        &mut self,
        f: crate::transducer::sentence::Form,
        train: &crate::transducer::family::Family,
        remainders: &[Vec<(i64, i64, u8, u8)>],
        mag: [u64; 4],
        budget: &mut crate::physics::Budget,
    ) {
        // A form withheld with no frames holds no perception, so it names
        // no family.
        self.withhold_with(f, None, train, remainders, mag, &[], budget);
    }

    /// [`Self::withhold_form`] for a perception: held with the frames it was
    /// perceived from, so its thoughts are transitions between bodies (N4).
    #[expect(clippy::too_many_arguments, reason = "78.56 III — the family is the perception's own coordinate, passed where it is held")]
    pub(crate) fn withhold_perceived(
        &mut self,
        f: crate::transducer::sentence::Form,
        frames: &crate::transducer::family::Frames,
        train: &crate::transducer::family::Family,
        remainders: &[Vec<(i64, i64, u8, u8)>],
        mag: [u64; 4],
        family: &[u8],
        budget: &mut crate::physics::Budget,
    ) {
        self.withhold_with(f, Some(frames), train, remainders, mag, family, budget);
    }

    #[expect(clippy::too_many_arguments, reason = "78.56 III — the family rides with the frames it scopes")]
    fn withhold_with(
        &mut self,
        f: crate::transducer::sentence::Form,
        frames: Option<&crate::transducer::family::Frames>,
        train: &crate::transducer::family::Family,
        remainders: &[Vec<(i64, i64, u8, u8)>],
        mag: [u64; 4],
        family: &[u8],
        budget: &mut crate::physics::Budget,
    ) {
        let k = self.own_k();
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
        // No room on a lane withholds the thought; the demand stands on the
        // docket either way. A perception is held with its bodies.
        let _ = match frames {
            Some(frames) => self.hold_perceived(f, frames, train, remainders, family, budget),
            None => self.hold_form(f, train, remainders, budget),
        };
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
    pub(crate) fn expand_ideas(&mut self) -> ([u64; 4], Option<(u64, u64)>) {
        let mut budget = self.physics.stroke();
        // THE CONTEMPLATION'S CLOCK IS TUNIVERSAL, WHICH IS WHAT IT SAYS IT
        // IS. `Contemplation.now_k` reads "TUniversal the entity last
        // stamped" and `Bubble.moved_k` reads "TUniversal when this bubble's
        // charts last moved"; this stamped both with the mind's own K,
        // which counts the orbits this mind acted in and moves on nothing
        // else (78.49). `absorb_taught` states the rule in one line: "k is
        // the mind's K (law 9), tu is TUniversal; they are not one clock."
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
        self.adjoin_letters(&adjoined, &mut budget);
        for e in &crystallized {
            self.stand_crystallized(e);
        }
        // 78.109 C1 — and what the pivot demanded is taken and constructed.
        self.take_demands(&mut budget);
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
    pub(crate) fn adjoin_letters(
        &mut self,
        adjoined: &crate::transducer::bubble::drive::Adjoined,
        budget: &mut crate::physics::Budget,
    ) {
        for (rotor, chart) in adjoined {
            // THE GÖDEL STEP, READ THROUGH THIS MIND'S OWN LETTERS (78.47): if
            // adjoined-by-closure reads 0, this is the reading that says why
            // (GD1). SPELLED by the held letters — adjoined ones included — it
            // is DERIVABLE (78.32(c)): counted, no refusal, and no new letter.
            // Unspelled it is D, the letter demand 37.7 names, and the letter
            // stands; no versor, P; no reading, unclassified. The same class
            // read D before an adjunction reads derivable after it.
            use crate::transducer::family::{letter_reading, LetterReading};
            use crate::transducer::sentence::Refused;
            // The caller's stroke (65.5): the reading is part of its act.
            let reading = letter_reading(&self.epigenome, budget, rotor);
            if reading == LetterReading::Derivable {
                self.contemplation.pivots.derivable += 1;
                continue;
            }
            if let Some(kind) = reading.kind() {
                let _ = Refused::RotorMintsNoGenerator(kind).noted();
            }
            // A rotor the mind already stands is not a refusal: the same class
            // is adjoined every stroke it is measured, and after the first the
            // letter is held with nothing to add.
            let Some(e) = crate::transducer::sentence::generator_form(rotor, chart, &[]) else {
                continue;
            };
            // Standing in this mind's own dialect is not being heard: only a
            // court's standing or ratifying (`already_taught`) ends the
            // utterance; a deferred letter is said again.
            if self.already_taught(&e) {
                continue;
            }
            self.stand_crystallized(&e);
        }
    }

    /// A crystallized idea stands in the dialect and is uttered once
    /// (25.2, 24.3): the same vertex already stood or taught is a use, not
    /// a second utterance.
    fn stand_crystallized(&mut self, e: &Element) {
        let addr = e.address();
        if self.already_taught(e) {
            self.unheard.remove(addr.as_bytes());
            self.remember_spoken(e);
            return;
        }
        // 78.161 item 3 — a letter goes to the court once another mind
        // agreed to it; until then it stands in the dialect and is said.
        if self.hold_for_union(e) {
            return;
        }
        // Spoken, refused on the wire, or withheld on a transport error: the
        // court's reply is the record (its refusal code, its docket). 37.7 —
        // where it lands, what no court taught on this utterance (deferred,
        // unread, unreachable) is said again on a later stroke; a refusal
        // seals it and it leaves.
        let word = e.clone();
        self.speak(std::slice::from_ref(e), move |me, _| {
            let sealed = me.sealed.get(addr.as_bytes()).is_some();
            if me.already_taught(&word) || sealed {
                me.unheard.remove(addr.as_bytes());
            } else if me.unheard.contains(addr.as_bytes()) || me.unheard.len() < crate::physics::aperture_cycle() as usize {
                me.unheard.insert(addr.as_bytes(), word);
            }
        });
    }

    /// 37.7 — SAY AGAIN WHAT NO COURT TAUGHT: every word this mind spoke that
    /// no court has stood, ratified or sealed is spoken once more, within the
    /// stroke's budget. A word said again leaves the unheard set while its
    /// answer is on the way, and returns to it where none taught it.
    fn speak_unheard(&mut self, budget: &mut crate::physics::Budget) {
        let pending: Vec<(Vec<u8>, Element)> = self.unheard.scoped().map(|(a, e)| (a, e.clone())).collect();
        for (a, e) in pending {
            if !budget.spend() {
                return;
            }
            self.unheard.remove(&a);
            self.stand_crystallized(&e);
        }
    }

    /// PIVOT ON A WORD: read by_chart at the word's chart and peels, fill
    /// those ideas under the caller's stroke (65.5). The atlas is borrowed,
    /// never cloned.
    pub(crate) fn pivot_on(&mut self, word: &Element, budget: &mut crate::physics::Budget) -> usize {
        // TUniversal, not the voice's K — see `expand_ideas`.
        let tu = self.now_tu();
        self.contemplation.set_now(tu);
        let (_fills, _formed, _classes, spoken, adjoined) =
            self.contemplation.pivot_on(word, &self.epigenome, budget);
        // The other path a pivot runs on, minting through the same reader.
        self.adjoin_letters(&adjoined, budget);
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
    pub(crate) fn pivot_thoughts(&mut self) -> usize {
        let added: Vec<Element> = self.epigenome.take_unpivoted();
        // The root of this act (called by the arm and the loop): one stroke
        // for every pivot it runs (65.5).
        let mut budget = self.physics.stroke();
        let mut spoken = 0usize;
        for w in &added {
            spoken += self.pivot_on(w, &mut budget);
        }
        spoken
    }

    pub(crate) fn shed(&mut self) -> usize {
        self.shed_at(self.own_k())
    }

    #[expect(
        clippy::disallowed_methods,
        reason = "rule 11 — one per resident item shed: four counts of what this process held cannot pass a 64-bit usize"
    )]
    pub(crate) fn shed_at(&mut self, now_k: u64) -> usize {
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
        let unglued = self.contemplation.shed_unglued();
        dropped.len().saturating_add(lapsed).saturating_add(expired).saturating_add(unglued)
    }

    pub(crate) fn dialect_len(&self) -> usize {
        self.epigenome.dialect_len()
    }

    /// The mind's working atlas: genome ∪ dialect ∪ the held forms' words,
    /// each fiber member carrying its standing; the Held stratum is read
    /// from the contemplation at the chart asked (24.1).
    pub(crate) fn working_atlas(&self) -> WorkingAtlas<'_> {
        self.epigenome.working_atlas_with(&self.contemplation)
    }

    /// Empty when genome, dialect and ideas are all empty.
    pub(crate) fn atlas_is_empty(&self) -> bool {
        self.epigenome.atlas_is_empty() && self.contemplation.is_empty()
    }

    /// Word counts per stratum on the working atlas: ruled (genome words),
    /// stood (dialect words), held (idea words). The atlas ruled/stood/held
    /// log line is these word counts, never chart counts.
    pub(crate) fn atlas_standing_counts(&self) -> (usize, usize, usize) {
        (
            self.epigenome.genome_len(),
            self.dialect_len(),
            self.ideas_len(),
        )
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
    pub(crate) fn take_promoted(&mut self) -> Vec<Vec<u8>> {
        let promoted = self.epigenome.take_promoted();
        if !promoted.is_empty() {
            let _ = self.contemplation.lapse_with(&promoted);
        }
        promoted
    }

    /// Working incidence: a non-empty fiber on the working atlas.
    #[cfg(test)]
    pub(crate) fn incident_chart(&self, c: &crate::transducer::stalk::Invariant) -> bool {
        !self.working_atlas().fiber(c).is_empty()
    }

    pub(crate) fn ideas_len(&self) -> usize {
        self.contemplation.len()
    }

    /// 78.37(1), TP1 — the contemplation's meet graph as it stands, with its
    /// decoherences since the contemplation began.
    #[cfg(test)]
    pub(crate) fn meet_graph(&self) -> crate::transducer::bubble::MeetGraph {
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
        m.remainders.callus_unminted = c.unminted;
        (m.remainders.receive_malformed, m.remainders.receive_unrecorded) = (c.malformed, c.unrecorded);
        // RM1 (c) — the re-fills.
        (m.remainders.answered_skipped, m.remainders.refill_acts, m.remainders.refill_refused) =
            (c.answered, c.refilled, c.refill_refused);
        m
    }

    /// The sizes of the carriers this entity owns, by count.
    #[cfg(test)]
    pub(crate) fn carriers(&self) -> [usize; 10] {
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
}
