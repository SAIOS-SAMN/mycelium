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
        if c != dialogue::SPEECH_REFUSED {
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
pub(crate) fn vertex_stratum(word: &Element) -> Vec<u8> {
    if word.is_sentence() {
        return word.address().into_vec();
    }
    match word.letters() {
        Some(l) => l.spelling.word.key().as_bytes().to_vec(),
        None => {
            let chain = crate::transducer::element::canonical_chain(word.sub())
                .unwrap_or_else(|| word.sub().to_vec());
            let mut out = Vec::with_capacity(chain.len() * 18);
            for (r, c, b, a) in &chain {
                out.extend_from_slice(&r.to_le_bytes());
                out.extend_from_slice(&c.to_le_bytes());
                out.push(*b);
                out.push(*a);
            }
            out
        }
    }
}

impl Entity {
    /// File an uttered word in both indices at once: by address, and in
    /// the stratum its vertex lies in.
    pub(crate) fn file_taught(&mut self, word: &Element) {
        let addr = word.address();
        self.taught_at.nested_insert(vertex_stratum(word), &addr, ());
        self.taught_up.insert(addr, word.clone());
    }

    pub fn cite(&mut self, chain: &[(i64, i64, u8, u8)]) -> Cited {
        // A remainder is not a citation of itself: deconstruction reads
        // other words on the working atlas, never the chain under locate.
        let self_addr = crate::transducer::element::from_residual(&chain.to_vec()).address();
        let extras: Vec<Element> = self
            .ideas
            .fiber(crate::network::nerve::ROOT)
            .filter_map(|(a, e)| (a.as_slice() != self_addr.as_bytes()).then(|| e.clone()))
            .collect();
        let located = self.epigenome.locate_on(chain, None, &extras);
        let n_genome = self.epigenome.genome_len();
        let dialect_addrs: Vec<Vec<u8>> = self
            .epigenome
            .dialect
            .fiber(crate::network::nerve::ROOT)
            .map(|(a, _)| a)
            .collect();
        let n_d = dialect_addrs.len();
        let word_of = |i: usize| -> Option<&Element> {
            if i < n_genome {
                self.epigenome.genome().get(i)
            } else if i < n_genome + n_d {
                dialect_addrs.get(i - n_genome).and_then(|a| self.epigenome.working_word(a))
            } else {
                extras.get(i - n_genome - n_d)
            }
        };
        // Standing is a coordinate on the cited word (Ruled, Stood, Held),
        // never a weight multiplied into μ⃗.
        let standing_of = |i: usize| -> Standing {
            if i < n_genome {
                Standing::Ruled
            } else if i < n_genome + n_d {
                Standing::Stood
            } else {
                Standing::Held
            }
        };
        let mut cites: Vec<dialogue::CiteWire> = located
            .cites
            .into_iter()
            .filter_map(|c| {
                let w = word_of(c.word)?;
                let body =
                    crate::transducer::sentence::cited_pattern(w, c.gauge).unwrap_or_default();
                Some(dialogue::CiteWire {
                    addr: w.address(),
                    standing: standing_of(c.word).as_u8(),
                    transport: c.transport,
                    gauge: c.gauge as u32,
                    body,
                })
            })
            .collect();
        for p in &located.punctures {
            let Some(w) = word_of(p.word) else { continue };
            cites.push(dialogue::CiteWire {
                addr: w.address(),
                standing: standing_of(p.word).as_u8(),
                transport: p.transport.clone(),
                gauge: u32::MAX,
                body: p.cells.clone(),
            });
        }
        (cites, located.residual)
    }

    /// Deepen fibres from puncture cites the hypha has verified. Linear in
    /// the proof; each address is a keyed read.
    pub fn puncture_verified(&mut self, cites: &[dialogue::CiteWire]) -> usize {
        let mut deepened = 0usize;
        for c in cites {
            if c.gauge != u32::MAX {
                continue;
            }
            let Some(i) = self.epigenome.index_of(&c.addr) else { continue };
            if self.epigenome.puncture(i, crate::transducer::element::assignment(&c.body)) {
                deepened += 1;
            }
        }
        deepened
    }

    /// SPEAK a percept as a proof: perceive the pair form, reduce over
    /// the genome, and submit the citations with the bodies.
    pub fn submit(
        &mut self,
        a: &[(i64, i64, u8)],
        b: &[(i64, i64, u8)],
        frame: &crate::arc::Frame,
    ) -> io::Result<(Reply, Option<Comprehension>)> {
        let (a_bodies, b_bodies) = perceive(a, b);
        let proof: Vec<Vec<dialogue::CiteWire>> =
            crate::cognition::percept_elements(&a_bodies, &b_bodies, frame, &self.epigenome)
                .map(|germs| germs.iter().map(|g| self.cite(g.sub()).0).collect())
                .unwrap_or_default();
        let reply = self.session.submit(a, b, frame, &proof)?;
        let heard = match reply.verdict.taught() {
            [] => None,
            t => self.absorb_taught(t),
        };
        Ok((reply, heard))
    }

    /// An act of this entity was admitted; its own clock steps (45.4).
    pub fn advance(&mut self) -> u64 {
        self.session.advance()
    }

    pub fn attest(&self, chain: &[(i64, i64, u8, u8)], cites: &[dialogue::CiteWire]) -> io::Result<Reply> {
        self.session.attest(chain, cites)
    }

    /// CONCLUDE a listing in this genome's own algebra: the chain is
    /// deconstructed into the words that compose it; the residual is
    /// expressed as this entity's own word. Returns the citations, the
    /// residual, and whether the residual was already claimed (familiar):
    /// a familiar residual is not spoken again.
    pub fn conclude(
        &mut self,
        chain: &[(i64, i64, u8, u8)],
    ) -> crate::client::sdk::Concluded {
        let (cites, residual) = self.cite(chain);
        let familiar = !self
            .epigenome
            .express(&residual, self.session.voice().k(), self.now_tu());
        Concluded { cites, residual, familiar }
    }

    /// A served listing: cite over the working atlas. Residual strictly
    /// below the chain on a lane is expressed; residual that does not
    /// descend is held as thought and never expressed.
    pub fn conclude_listing(
        &mut self,
        chain: &[(i64, i64, u8, u8)],
    ) -> crate::client::sdk::Concluded {
        let (cites, residual) = self.cite(chain);
        // 24.2: a residual that raised a lane, or moved nothing at all, is
        // held as thought and never spoken. The reading says which case.
        let reading = residual_move(&residual, chain);
        if !crate::economy::lanes_risen(&reading).is_empty()
            || crate::economy::lanes_fallen(&reading).is_empty()
        {
            self.hold_thought(chain.to_vec());
            return Concluded { cites, residual: chain.to_vec(), familiar: false };
        }
        let familiar = !self
            .epigenome
            .express(&residual, self.session.voice().k(), self.now_tu());
        Concluded { cites, residual, familiar }
    }

    /// The word a cite rests on: genome, dialect, or idea.
    pub fn cited_word(&self, addr: &[u8]) -> Option<Element> {
        self.epigenome
            .working_word(addr)
            .cloned()
            .or_else(|| self.idea_word(addr))
    }

    pub fn probe(&self, target: &[u8], cites: &[dialogue::CiteWire]) -> io::Result<(u8, [u64; 4], [u64; 4])> {
        self.session.probe(target, cites)
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
            // 45.4: STANDING A FORM IS AN ADMITTED ACT of this mind, so the
            // mind's own clock steps and the form stands at the step it
            // took. The stamp is the act's own K, not a number the hypha
            // last replied with.
            let k = self.session.advance();
            self.epigenome.stand_dialect_at(e.clone(), k, tu);
            if !taught {
                codes[i] = 0xff; // asked about on the wire below
            }
        }
        codes
    }

    /// SPEAK a family's cohering sentences as one act. Refuses if two
    /// sentences share a germ with a non-identity transition. Each word
    /// STANDS in the dialect before it is uttered (24.3: perceive, think,
    /// stand, utter); the genome is frozen. A vertex already on `taught_up`
    /// stands in the dialect and does not go on the wire.
    pub fn speak(&mut self, sentences: &[Element]) -> io::Result<Vec<u8>> {
        if sentences.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "no sentence"));
        }
        let tu = self.now_tu();
        let mut codes = self.stand(sentences, tu);
        let fresh: Vec<&Element> = sentences
            .iter()
            .enumerate()
            .filter(|(i, _)| codes[*i] == 0xff)
            .map(|(_, e)| e)
            .collect();
        if fresh.is_empty() {
            return Ok(codes);
        }
        let voice = self.session.kernel().voice();
        let bytes: Vec<Vec<u8>> = fresh.iter().map(|e| e.to_bytes()).collect();
        let body = dialogue::encode_speech(&voice, crate::transducer::address::Address::new(Vec::new()), tu, &bytes);
        let reply = self.session.speak_nerve(body)?;
        let got = dialogue::decode_speech_reply(&reply).unwrap_or_default();
        for i in read_speech_reply(&mut codes, &got) {
            // The utterance was admitted: the second act, and the second
            // step of this mind's clock (45.4). A refused utterance is not
            // an act and moves nothing.
            self.session.advance();
            let e = sentences[i].clone();
            self.file_taught(&e);
        }
        Ok(codes)
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
        // THE STRATUM, THEN THE EXACT TEST. This walked the whole of
        // `taught_up` running `same_vertex` against every word this mind
        // had ever uttered, on every sentence of every speak (law 3, and
        // its answer: the carrier was missing a key). `taught_at` names the
        // words that COULD be one vertex with this one; the exact test
        // decides over that fiber and nothing else.
        let at = vertex_stratum(word);
        if let Some(fiber) = self.taught_at.get(&at) {
            let candidates: Vec<Vec<u8>> = fiber.fiber(crate::network::nerve::ROOT).map(|(k, _)| k).collect();
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
            self.epigenome.touch(&a, self.session.voice().k());
            return true;
        }
        false
    }

    /// Stand the vertex in the dialect and mark it taught, without the wire.
    pub fn remember_spoken(&mut self, word: &Element) {
        let k = self.session.voice().k();
        self.epigenome.stand_dialect_at(word.clone(), k, self.now_tu());
        self.file_taught(word);
    }

    /// SPEAK A WORD as a one-sentence family. The word enters the dialect
    /// when it stands; the genome receives it only from the syllabus.
    pub fn speak_word(&mut self, word: &Element) -> io::Result<Utterance> {
        // A thought that still has a remainder is work, not a message: the
        // wire never carries ∂g. A word with a chain is held, not uttered.
        if let [_, ..] = word.sub() {
            self.hold_thought(word.sub().to_vec());
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "a remainder is thought, not speech"));
        }
        // Speech is a connection or a connection with an opened axis; a
        // residual germ or the unit is not a message.
        if word.is_identity() || (!word.is_connection() && word.opened_axes().is_empty()) {
            return Ok(Utterance::Silent);
        }
        let voice = self.session.kernel().voice();
        let addr = word.address();
        if self.already_taught(word) {
            return Ok(Utterance::Already);
        }
        let bytes = word.to_bytes();
        let codes = self.speak(std::slice::from_ref(word))?;
        self.file_taught(word);
        let _ = (voice, bytes, addr);
        match codes.first().copied() {
            Some(dialogue::SPEECH_STOOD) | None => Ok(Utterance::Spoken),
            Some(dialogue::SPEECH_REFUSED) => Ok(Utterance::Silent),
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
