// 26 — UTTERANCE: what this entity speaks to a court and what it cites.
//
//! A citation is a word this entity holds; a speech is a sentence it has
//! not spoken before. `already_taught` reads what the court has, so the
//! same word is not spoken twice.

use crate::client::sdk::*;

/// WHAT THE HYPHA SAID ABOUT EACH SENTENCE SENT, written into `codes` in
/// the order the sentences stand, and the indices it ADMITTED.
///
/// A slot still holding `0xff` is one this mind put on the wire; the reply
/// answers those in order. A CODE THE HYPHA DID NOT SEND IS NOT AN
/// OUTCOME: this read `got.get(gi).copied().unwrap_or(SPEECH_STOOD)`, so
/// a reply shorter than the sentences sent — a truncated frame, a hypha
/// that answered about fewer — was read as the hypha having STOOD the
/// rest. The mind then stepped its clock for an act nobody admitted
/// (45.4) and filed the word in `taught_up`, where it would never be
/// spoken again. An invented value, and the value it invents is the
/// favourable one (rule 11, checklist (i)). Nothing was heard about that
/// sentence, so it is `SPEECH_DIALECT` — stood here, unanswered there —
/// and it may be spoken again.
pub(crate) fn read_speech_reply(codes: &mut [u8], got: &[u8]) -> Vec<usize> {
    let mut admitted = Vec::new();
    let mut gi = 0usize;
    for (i, slot) in codes.iter_mut().enumerate() {
        if *slot != 0xff {
            continue;
        }
        let Some(c) = got.get(gi).copied() else {
            *slot = dialogue::SPEECH_DIALECT;
            continue;
        };
        gi += 1;
        *slot = c;
        // AN ADMISSION IS A VERDICT: stood or ratified. A refusal is not an
        // act, and a wait (`SPEECH_DIALECT`, "stood there, unanswered here,
        // spoken again later") admitted nothing either — filed as taught,
        // the waiting sentence would never be spoken again (50.9).
        if matches!(c, dialogue::SPEECH_STOOD | dialogue::SPEECH_RATIFIED) {
            admitted.push(i);
        }
    }
    admitted
}

/// THE STRATUM A WORD'S VERTEX LIES IN: the M1 key of its rotor where it
/// has letters, its canonical chain where it does not, its address where it
/// is a sentence.
///
/// Every one is a NECESSARY condition of `same_vertex`: that test requires
/// the rotors to be projectively equal (M1), the canonical chains to be
/// equal (A5), and it decides two sentence forms by address alone. So two
/// words that are one vertex share this key, and a fiber read under it
/// holds every candidate. It is not sufficient — the exact test still
/// decides — which is A21 §3's own shape.
///
/// `None` when the chain is one A5 refuses (two transitions on one site):
/// such a word is not a vertex, and it has no stratum. It was keyed by its
/// raw bytes instead — `unwrap_or_else(|| word.sub().to_vec())`, a key
/// invented where the canonical one is refused (rule 11, checklist i).
pub(crate) fn vertex_stratum(word: &Element) -> Option<Vec<u8>> {
    if word.is_sentence() {
        return Some(word.address().into_vec());
    }
    match word.letters() {
        Some(l) => Some(l.spelling.word.key().as_bytes().to_vec()),
        None => {
            let chain = crate::transducer::element::canonical_chain(word.sub())?;
            let mut out = Vec::with_capacity(chain.len() * 18);
            for (r, c, b, a) in chain.as_slice() {
                out.extend_from_slice(&r.to_le_bytes());
                out.extend_from_slice(&c.to_le_bytes());
                out.push(*b);
                out.push(*a);
            }
            Some(out)
        }
    }
}

impl Entity {
    /// File an uttered word in both indices at once: by address, and in
    /// the stratum its vertex lies in.
    pub(crate) fn file_taught(&mut self, word: &Element) {
        let addr = word.address();
        // A word with no stratum (a chain A5 refuses) is filed by address
        // alone: there is no vertex for another spelling to share.
        if let Some(at) = vertex_stratum(word) {
            self.taught_at.nested_insert(at, &addr, ());
        }
        // An admitted citer is owed to no court any more.
        self.owed_tries.remove(addr.as_bytes());
        self.taught_up.insert(addr, word.clone());
    }

    /// The utterance receipt of a word that left the dialect leaves both
    /// indices with it (S12.6a): by address, and in its vertex stratum.
    pub(crate) fn forget_utterance(&mut self, addr: &[u8]) {
        let Some(word) = self.taught_up.remove(addr) else {
            return;
        };
        if let Some(at) = vertex_stratum(&word) {
            if let Some(inner) = self.taught_at.get_mut(&at) {
                inner.remove(addr);
            }
        }
    }

    /// How many utterance receipts the mind holds (7.3's measurement).
    pub fn utterances_filed(&self) -> usize {
        self.taught_up.len()
    }

    /// CITE a chain over the working atlas: the genome and the dialect by
    /// index, and the held forms' words read at each body's chart as the
    /// reading asks (24.1 as ruled 2026-09-09) — the vertex's action
    /// evaluated on the body, never a list of elements built ahead of the
    /// question. A remainder is not a citation of itself: the vertex whose
    /// address is the chain's own key is passed over, since deconstruction
    /// reads other words, never the chain under locate. A citation is a use
    /// (45.4): a stood word cited here is touched at the mind's own K.
    pub fn cite(&mut self, chain: &[(i64, i64, u8, u8)], budget: &mut crate::physics::Budget) -> Cited {
        // The chain's own key, a listing key (17.2): the one address the
        // held stratum does not answer with.
        // A chain with no A5 form cites nothing and stands whole as the
        // residual: there is no word here to be a use of.
        let Some(own) = crate::transducer::element::from_residual(&chain.to_vec()) else {
            return (Vec::new(), chain.to_vec());
        };
        let own = own.address();
        let _ = &own;
        // The chain as the cite would reconstruct it: sorted and deduped,
        // which is the form `vertex_cites` hands back.
        let whole = {
            let mut w = chain.to_vec();
            w.sort_unstable();
            w.dedup();
            w
        };
        // The caller's stroke reads the citation (65.5).
        let contemplation = &self.contemplation;
        let held_at = |inv: &crate::transducer::stalk::Invariant,
                       piece: &[(i64, i64, u8, u8)],
                       budget: &mut crate::physics::Budget| {
            contemplation
                .held_at(inv)
                .into_iter()
                .filter_map(|v| crate::transducer::sentence::vertex_cites(v, piece, budget))
                // A REMAINDER IS NOT A CITATION OF ITSELF, and what says so
                // is the BODY the cite reconstructs (L6: "a cite IS the
                // body it reconstructs"), not the vertex's name.
                //
                // It used to drop a held vertex whose ADDRESS equalled the
                // chain's, which worked only while a word vertex carried
                // the address of the word the reading located. Under 54.2
                // a vertex's address is its own coordinates and that
                // citation is a receipt, so the guard now asks the question
                // it always meant: does this cite reconstruct the very
                // chain being cited.
                .filter(|c| {
                    let mut body = c.body.clone();
                    body.sort_unstable();
                    body.dedup();
                    body != whole
                })
                .collect::<Vec<_>>()
        };
        let located = self.epigenome.locate_on(chain, None, &held_at, budget);
        let word_of = |a: &[u8]| -> Option<&Element> {
            self.epigenome.working_word(a).map(|h| h.element())
        };
        // STANDING IS A COORDINATE ON THE CITED WORD (Ruled, Stood, Held),
        // never a weight multiplied into μ⃗ — and never, as it was here, a
        // comparison of an offset against the genome's length. Each stratum
        // is asked at the address, in the strata's own order (24.1); a word
        // the working language does not hold has no standing to report and
        // is not put on the wire.
        let standing_of = |a: &[u8]| -> Option<Standing> { self.epigenome.standing_of(a) };
        let mut cites: Vec<dialogue::CiteWire> = located
            .cites
            .into_iter()
            .filter_map(|c| {
                let w = word_of(c.word.as_bytes())?;
                // L6: A CITE IS THE BODY IT RECONSTRUCTS. One whose gauge
                // reconstructs none is not a cite carrying nothing — it is
                // no cite, and it does not go on the wire.
                let body = crate::transducer::sentence::cited_pattern(w, &c.assignment)?;
                Some(dialogue::CiteWire {
                    addr: w.address(),
                    standing: standing_of(c.word.as_bytes())?.as_u8(),
                    transport: c.transport,
                    body,
                })
            })
            .collect();
        for p in &located.punctures {
            let Some(w) = word_of(p.word.as_bytes()) else { continue };
            let Some(st) = standing_of(p.word.as_bytes()) else { continue };
            cites.push(dialogue::CiteWire {
                addr: w.address(),
                standing: st.as_u8(),
                transport: p.transport.clone(),
                body: p.links.clone(),
            });
        }
        // A held cite carries the body in the frame the vertex's action
        // was evaluated in — the chain's own — so its transport is the
        // identity and its one section is the vertex's gauge.
        for h in located.held {
            cites.push(dialogue::CiteWire {
                addr: h.address,
                standing: Standing::Held.as_u8(),
                transport: crate::transducer::element::identity_versor(),
                body: h.body,
            });
        }
        let k = self.own_k();
        for c in &cites {
            if c.standing == Standing::Stood.as_u8() {
                self.epigenome.touch(&c.addr, k);
            }
        }
        (cites, located.residual)
    }

    /// Deepen fibres from puncture cites the hypha has verified. Linear in
    /// the proof; each address is a keyed read.
    pub fn puncture_verified(&mut self, cites: &[dialogue::CiteWire]) -> usize {
        let mut deepened = 0usize;
        // EVERY CITE'S BODY IS OFFERED: `Epigenome::puncture` answers `None`
        // for an assignment the word already speaks, so the hypha's own
        // reading decides what is a section — never a flag the speaker set.
        for c in cites {
            let Some(at) = self.epigenome.puncture(&c.addr, crate::transducer::sentence::body_assignment(&c.body)) else {
                continue;
            };
            deepened += 1;
            // THE PUNCTURED WORD IS ADMITTED BY AN ACT ON THIS VOICE'S LINE
            // (78.49): a new address learned from a proof the hypha
            // verified — an act of this mind; the court admits it when it
            // rules it, and its stamp is then the word's locator.
            if let Some(word) = self.epigenome.word_at(at.as_bytes()).map(|h| h.element().clone()) {
                self.act_on(&word);
            }
        }
        deepened
    }

    /// A served listing: cite over the working atlas. Residual strictly
    /// below the chain on a lane is expressed; residual that raised a
    /// lane or moved nothing is held as thought — the form of the listing's chain
    /// (50.6) — and never expressed.
    pub fn conclude_listing(
        &mut self,
        chain: &[(i64, i64, u8, u8)],
    ) -> crate::client::sdk::Concluded {
        // The root of this act (no caller holds a stroke): one stroke for
        // the citation and the expression both (65.5).
        let mut budget = self.physics.stroke();
        let (cites, residual) = self.cite(chain, &mut budget);
        // 24.2: a residual that raised a lane, or moved nothing at all, is
        // held as thought and never spoken. The reading says which case.
        let reading = residual_move(&residual, chain);
        if !crate::economy::lanes_risen(&reading).is_empty()
            || crate::economy::lanes_fallen(&reading).is_empty()
        {
            self.hold_chain(chain.to_vec());
            return Concluded { cites, residual: chain.to_vec(), familiar: false };
        }
        let familiar = !self
            .epigenome
            .express(&residual, self.own_k(), self.now_tu(), &mut budget);
        Concluded { cites, residual, familiar }
    }

    /// PR1 — A PROBE, AND ITS READING: folded into this voice's typed tally
    /// (served on `Learning`), and returned.
    pub fn probe(&mut self, target: &[u8], cites: &[dialogue::CiteWire]) -> io::Result<dialogue::ProbeReading> {
        let r = self.session.probe(target, cites)?;
        self.probes.note(&r);
        Ok(r)
    }

    /// STAND every sentence in the dialect, and say which of them are
    /// fresh. 24.3 fixes the order — "perceive, think, stand, utter" — and
    /// these are two of those acts, not one: standing is complete before
    /// anything reaches the wire, and it is what steps this mind's clock.
    ///
    /// A slot reads `0xff` when the sentence is fresh and the utterance
    /// will ask about it; `SPEECH_STOOD` when this mind has already taught
    /// that vertex, which stands in the dialect and goes nowhere.
    ///
    /// It is its own function so that the claim "standing steps the mind's
    /// own K, and no reply moves it" can be proved without a socket, which
    /// §3.7 does not let a test open.
    pub(crate) fn stand(&mut self, sentences: &[Element], tu: u64) -> Vec<u8> {
        let mut codes = vec![dialogue::SPEECH_STOOD; sentences.len()];
        for (i, e) in sentences.iter().enumerate() {
            let taught = self.already_taught(e);
            // 78.49: STANDING A FORM IS AN ACT of this mind, and the form
            // stands at the mind's own K — never a number the hypha last
            // replied with. The court seals what it admits.
            let k = self.act_on(e);
            self.epigenome.stand_dialect_at(e.clone(), k, tu);
            if !taught {
                codes[i] = 0xff; // asked about on the wire below
            }
        }
        codes
    }

    /// SPEAK sentences as one act, bound to no family and no task: a word
    /// spoken for itself (an axis germ, a crystallized idea).
    pub fn speak(&mut self, sentences: &[Element], budget: &mut crate::physics::Budget) -> io::Result<Vec<u8>> {
        self.speak_with(sentences, &[], &[], budget)
    }

    /// SPEAK a family's cohering sentences as one act, naming the task so
    /// the court replays them on its copy of the family (23.8, 50.2 G).
    /// Each word STANDS in the dialect before it is uttered (24.3:
    /// perceive, think, stand, utter); the genome is frozen. A vertex
    /// already on `taught_up` stands in the dialect and does not go on the
    /// wire. THE SEALED SET IS READ FIRST (50.9): a tape a court refused is
    /// refused by this mind before any frame is built, and never stood. The
    /// reply is one adjudication per sentence sent: a stood or ratified
    /// sentence is filed and its receipt sealed; a refused one is contracted
    /// (`receive_adjudication`) and its contraction queued.
    pub fn speak_with(
        &mut self,
        sentences: &[Element],
        train: &crate::transducer::family::Family,
        task: &[u8],
        budget: &mut crate::physics::Budget,
    ) -> io::Result<Vec<u8>> {
        if sentences.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "no sentence"));
        }
        let tu = self.now_tu();
        let mut codes = vec![0xffu8; sentences.len()];
        let mut open: Vec<usize> = Vec::new();
        for (i, e) in sentences.iter().enumerate() {
            let addr = e.address();
            // Refused at a K this mind sealed: never uttered again.
            match self.sealed.get(&addr) {
                Some(_) => codes[i] = dialogue::SPEECH_REFUSED,
                None => open.push(i),
            }
        }
        let standing: Vec<Element> = open.iter().map(|&i| sentences[i].clone()).collect();
        let stood = self.stand(&standing, tu);
        for (j, &i) in open.iter().enumerate() {
            codes[i] = stood[j];
        }
        let fresh: Vec<usize> = (0..sentences.len()).filter(|&i| codes[i] == 0xff).collect();
        if fresh.is_empty() {
            return Ok(codes);
        }
        let voice = self.session.kernel().voice();
        // STD1 3c: a sentence whose record does not write refuses the speech
        // whole, never a speech with that sentence left out.
        // 78.99 (5) — EACH SENTENCE CARRIES THE COORDINATES OF THE WORDS ITS
        // FORM TEACHES, read under this speech's stroke: the relay is holes
        // and windings, re-read by the court on its own construction.
        let spoken: Option<Vec<dialogue::Spoken>> = fresh
            .iter()
            .map(|&i| {
                let (coordinates, _unread) = crate::transducer::sentence::rung::coordinates_taught(&sentences[i], budget);
                crate::transducer::cech::note_construction(crate::transducer::cech::board::RELAYED, coordinates.len() as u64);
                Some(dialogue::Spoken {
                    tape: sentences[i].to_bytes()?,
                    standing: Standing::Stood.as_u8(),
                    contraction: self.certificate_for(&sentences[i]),
                    coordinates,
                })
            })
            .collect();
        let Some(spoken) = spoken else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "speech refused: a sentence's record passes a length its field writes",
            ));
        };
        // LD0: a speech whose task address cannot be framed is not spoken.
        let Some(body) = dialogue::encode_speech_spoken(&voice, task, tu, &spoken) else {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "speech refused: the task's address cannot be framed"));
        };
        let reply = self.session.speak_nerve(body)?;
        // B7: A REPLY THAT DOES NOT DECODE IS NOT AN OUTCOME. Reading it as
        // zero adjudications says the court answered and adjudicated
        // nothing, which is a different fact from "this mind cannot read
        // what came back" — and it is the second that happened.
        let Some(adjudicated) = dialogue::decode_speech_reply(&reply) else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "speech reply does not decode",
            ));
        };
        let got: Vec<u8> = adjudicated.iter().map(|a| a.code()).collect();
        for i in read_speech_reply(&mut codes, &got) {
            // The utterance was admitted by the court: an act of this mind
            // (78.49). A refused utterance is not an admitted one.
            let e = sentences[i].clone();
            self.act_on(&e);
            // 78.92 — WHY IT WAS SAID, read off the sentence the court
            // admitted: the same reader the court's copy answers to.
            if let Some(r) = crate::transducer::sentence::Reasons::of(&e) {
                self.reasons.note(&r);
                self.said = Some(r);
                // 78.93 B0 — and where the court says it stands.
                self.said_at = fresh.iter().position(|&f| f == i).and_then(|j| adjudicated.get(j)).and_then(|a| a.at());
            }
            self.file_taught(&e);
            self.contractions.remove(&e.address());
            if let Some(k) = crate::transducer::sentence::from_element_form(&e)
                .map(|f| crate::transducer::sentence::form_vertex_key(&f))
            {
                self.contractions.remove(&k);
            }
        }
        for (j, &i) in fresh.iter().enumerate() {
            match adjudicated.get(j) {
                Some(dialogue::Adjudication::Refused { kind, at, receipt }) => {
                    self.receive_adjudication(&sentences[i], *kind, at.clone(), *receipt, train, task, budget);
                }
                // 21 step 3: the court has not ruled a form this sentence
                // cited. Nothing is sealed — this is not a refusal — and
                // the mind owes that court the cited form before the
                // citer (chapter 6's depth order). The mind HOLDS it,
                // because it derived through it.
                Some(dialogue::Adjudication::Awaiting { form }) => {
                    let _ = self.owe_citation(&sentences[i], form, train, task);
                }
                // Stood or ratified: sealed and filed above.
                Some(_) | None => {}
            }
        }
        Ok(codes)
    }

    /// 78.53 — SPEAK WHAT THE COLOUR FIBRE DID. The family's fibre relation
    /// (`fibre_relation`, read where the base stands still, in the sensor's
    /// labels) is minted as a colour word, stood in this mind's dialect —
    /// its short-term language, pruned by truncation — and spoken to the
    /// court WITH THE TASK it was seen in, so the court can re-derive it
    /// from its own copy of the family. A colour word is its relation, so
    /// one already spoken or already in the genome is found by its address
    /// (every colour word shares the motionless vertex; a vertex test would
    /// call them all one). Answers the court's code, or `None` where the
    /// family's fibre moved no hue or the word was already said.
    pub fn speak_colour(
        &mut self,
        task: &[u8],
        train: &crate::transducer::family::Family,
        budget: &mut crate::physics::Budget,
    ) -> Option<u8> {
        let sigma = crate::transducer::family::fibre_relation(train)?;
        let word = crate::transducer::element::colour_word(&sigma)?;
        let addr = word.address();
        if self.already_taught_addr(addr.as_bytes()) || self.epigenome.holds(addr.as_bytes()) {
            return None;
        }
        // 78.55 — DID THE LANGUAGE ALREADY SAY IT? A relation this mind has
        // not heard may still be DERIVED from the colour words it holds: one
        // of them, an inverse, or two composed. Derived, the mind understood
        // the scene from its language — a deduction, not an observation.
        if self.colour_derivation(&sigma, budget) {
            self.colour_derived += 1;
        }
        let codes = self.speak_with(std::slice::from_ref(&word), train, task, budget).ok()?;
        match codes.as_slice() {
            [code] => Some(*code),
            _ => None,
        }
    }

    /// 78.55 — A COLOUR THEOREM: the language's closure, spoken. The ruled
    /// colour words this mind holds are composed in pairs (the genome's own
    /// key order, so minds holding one genome propose one theorem), and the
    /// first composite it neither holds nor has said is spoken with its
    /// derivation cited — no task, no scene: the court admits it by
    /// composing the two words it cites. Each composition draws one read.
    /// Answers the court's code, or `None` where the language is closed
    /// under composition as far as the stroke reached.
    pub fn speak_colour_theorem(&mut self, budget: &mut crate::physics::Budget) -> Option<u8> {
        let held: Vec<Element> =
            self.epigenome.genome_words().filter(|w| w.is_colour_word()).map(|w| (**w).clone()).collect();
        let mut theorem: Option<Element> = None;
        'pairs: for first in &held {
            for after in &held {
                if !budget.spend() {
                    break 'pairs;
                }
                let Some(w) = crate::transducer::element::derived_colour_word(first, after) else { continue };
                let addr = w.address();
                // A theorem the court refused is sealed here and never
                // proposed again (it would be counted and not sent).
                if !self.already_taught_addr(addr.as_bytes())
                    && !self.epigenome.holds(addr.as_bytes())
                    && self.sealed.get(&addr).is_none()
                {
                    theorem = Some(w);
                    break 'pairs;
                }
            }
        }
        let theorem = theorem?;
        let codes = self.speak_with(std::slice::from_ref(&theorem), &[], &[], budget).ok()?;
        self.colour_theorems += 1;
        match codes.as_slice() {
            [code] => Some(*code),
            _ => None,
        }
    }

    /// 78.55 — WHETHER THIS MIND'S COLOUR WORDS DERIVE `sigma`: a genome
    /// colour word, its inverse, or the composite of two of those, equal to
    /// it. The candidates are the court-ruled colour words it holds (its
    /// genome, never its dialect: a derivation stands on ruled language),
    /// and each composition draws one read from the stroke; a spent stroke
    /// derives nothing, which is the honest reading of a search not made.
    pub fn colour_derivation(&self, sigma: &[(u8, u8)], budget: &mut crate::physics::Budget) -> bool {
        use crate::transducer::element::{compose_fibre, invert_fibre};
        let mut held: Vec<Vec<(u8, u8)>> = Vec::new();
        for w in self.epigenome.genome_words().filter(|w| w.is_colour_word()) {
            if let Some(l) = w.letters() {
                held.push(l.sigma.clone());
                held.push(invert_fibre(&l.sigma));
            }
        }
        if held.iter().any(|h| h.as_slice() == sigma) {
            return true;
        }
        for a in &held {
            for b in &held {
                if !budget.spend() {
                    return false;
                }
                if compose_fibre(a, b).is_some_and(|c| c.as_slice() == sigma) {
                    return true;
                }
            }
        }
        false
    }

    pub(crate) fn already_taught_addr(&self, addr: &[u8]) -> bool {
        self.taught_up.contains(addr)
    }

    /// Already uttered this operator: same vertex on the working atlas
    /// or already sent up, not the same digest. A dialect hit is a use.
    pub fn already_taught(&mut self, word: &Element) -> bool {
        if self.taught_up.contains(&word.address()) {
            return true;
        }
        // THE STRATUM, THEN THE EXACT TEST. This read the whole of
        // `taught_up` running `same_vertex` against every word this mind
        // had ever uttered, on every sentence of every speak (law 3, and
        // its answer: the carrier was missing a key). `taught_at` names the
        // words that COULD be one vertex with this one; the exact test
        // decides over that fiber and nothing else.
        if let Some(fiber) = vertex_stratum(word).and_then(|at| self.taught_at.get(&at)) {
            let candidates: Vec<Vec<u8>> = fiber.scoped().map(|(k, _)| k).collect();
            for a in candidates {
                if self.taught_up.get(&a).is_some_and(|e| e.same_vertex(word)) {
                    return true;
                }
            }
        }
        if self.epigenome.same_vertex_in_genome(word).is_some() {
            return true;
        }
        if let Some(a) = self.epigenome.same_vertex_in_dialect(word) {
            self.epigenome.touch(&a, self.own_k());
            return true;
        }
        false
    }

    /// Stand the vertex in the dialect and mark it taught, without the wire.
    pub fn remember_spoken(&mut self, word: &Element) {
        let k = self.own_k();
        self.epigenome.stand_dialect_at(word.clone(), k, self.now_tu());
        self.file_taught(word);
    }

    /// SPEAK A WORD as a one-sentence family. The word enters the dialect
    /// when it stands; the genome receives it only from the syllabus.
    pub fn speak_word(&mut self, word: &Element, budget: &mut crate::physics::Budget) -> io::Result<Utterance> {
        // A thought that still has a remainder is work, not a message: the
        // wire never carries ∂g. A word with a chain is held, not uttered.
        if let [_, ..] = word.sub() {
            self.hold_chain(word.sub().to_vec());
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "a remainder is thought, not speech"));
        }
        // Speech is a connection or a connection with an opened axis; a
        // residual germ or the unit is not a message. A thought is never
        // speech: a closed thought is spoken as the word it transcribes to
        // (78.52), a connection like any other.
        if word.is_identity() || (!word.is_connection() && word.opened_axes().is_empty()) {
            return Ok(Utterance::Silent);
        }
        let voice = self.session.kernel().voice();
        let addr = word.address();
        if self.already_taught(word) {
            return Ok(Utterance::Already);
        }
        let bytes = word.to_bytes();
        let codes = self.speak(std::slice::from_ref(word), budget)?;
        self.file_taught(word);
        let _ = (voice, bytes, addr);
        // ONE SENTENCE WAS SENT, so the reply is one code — the slice says
        // that rather than a reader taking whichever code came first.
        match codes.as_slice() {
            [dialogue::SPEECH_STOOD] | [] => Ok(Utterance::Spoken),
            [dialogue::SPEECH_REFUSED] => Ok(Utterance::Silent),
            _ => Ok(Utterance::Spoken),
        }
    }

    /// Addresses this mind has uttered (receipt: address at this K).
    pub fn uttered_n(&self) -> usize {
        self.taught_up.len()
    }

    pub fn uttered(&self, addr: &[u8]) -> bool {
        self.taught_up.contains(addr)
    }

}
