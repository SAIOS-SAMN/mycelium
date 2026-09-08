// 26 — UTTERANCE: what this entity speaks to a court and what it cites.
//
//! A citation is a word this entity holds; a speech is a sentence it has
//! not spoken before. `already_taught` reads what the court has, so the
//! same word is not spoken twice.

use crate::client::sdk::*;

impl Entity {
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
                Some((
                    w.address(),
                    standing_of(c.word).as_u8(),
                    c.transport,
                    c.gauge as u32,
                    body,
                ))
            })
            .collect();
        for (i, g, body) in &located.punctures {
            let Some(w) = word_of(*i) else { continue };
            cites.push((w.address(), standing_of(*i).as_u8(), g.clone(), u32::MAX, body.clone()));
        }
        (cites, located.residual)
    }

    /// Deepen fibres from puncture cites the hypha has verified. Linear in
    /// the proof; each address is a keyed read.
    pub fn puncture_verified(&mut self, cites: &[dialogue::CiteWire]) -> usize {
        let mut deepened = 0usize;
        for (addr, _, _, asg, body) in cites {
            if *asg != u32::MAX {
                continue;
            }
            let Some(i) = self.epigenome.index_of(addr) else { continue };
            if self.epigenome.puncture(i, crate::transducer::element::assignment(body)) {
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

    /// This entity's voice read the judge's clock.
    pub fn at(&mut self, k: u64) {
        self.session.at(k);
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
        (cites, residual, familiar)
    }

    /// A served listing: cite over the working atlas. Residual strictly
    /// below the chain on a lane is expressed; residual that does not
    /// descend is held as thought and never expressed.
    pub fn conclude_listing(
        &mut self,
        chain: &[(i64, i64, u8, u8)],
    ) -> crate::client::sdk::Concluded {
        let (cites, residual) = self.cite(chain);
        if !residual_descends(&residual, chain) {
            self.hold_thought(chain.to_vec());
            return (cites, chain.to_vec(), false);
        }
        let familiar = !self
            .epigenome
            .express(&residual, self.session.voice().k(), self.now_tu());
        (cites, residual, familiar)
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

    /// SPEAK a family's cohering sentences as one act. Refuses if two
    /// sentences share a germ with a non-identity transition. Each uttered
    /// word enters the dialect (epigenome handles); the genome is frozen.
    /// A vertex already on `taught_up` stands in the dialect and does not
    /// go on the wire.
    pub fn speak(&mut self, sentences: &[Element]) -> io::Result<Vec<u8>> {
        if sentences.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "no sentence"));
        }
        let k = self.session.voice().k();
        let mut fresh: Vec<&Element> = Vec::new();
        let mut codes = vec![dialogue::SPEECH_STOOD; sentences.len()];
        for (i, e) in sentences.iter().enumerate() {
            if self.already_taught(e) {
                self.epigenome.stand_dialect_at(e.clone(), k, self.now_tu());
                continue;
            }
            fresh.push(e);
            codes[i] = 0xff; // filled from the wire below
        }
        if fresh.is_empty() {
            return Ok(codes);
        }
        let voice = self.session.kernel().voice();
        let bytes: Vec<Vec<u8>> = fresh.iter().map(|e| e.to_bytes()).collect();
        let body = dialogue::encode_speech(&voice, crate::transducer::a21::Address::new(Vec::new()), self.now_tu(), &bytes);
        let reply = self.session.speak_nerve(body)?;
        let got = dialogue::decode_speech_reply(&reply).unwrap_or_default();
        let mut gi = 0usize;
        for (i, e) in sentences.iter().enumerate() {
            if codes[i] != 0xff {
                continue;
            }
            let c = got.get(gi).copied().unwrap_or(dialogue::SPEECH_STOOD);
            gi += 1;
            codes[i] = c;
            self.epigenome.stand_dialect_at(e.clone(), k, self.now_tu());
            if c != dialogue::SPEECH_REFUSED {
                self.taught_up.insert(e.address(), e.clone());
            }
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
        if self
            .taught_up
            .fiber(crate::network::nerve::ROOT)
            .any(|(_, e)| e.same_vertex(word))
        {
            return true;
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
        self.taught_up.insert(word.address(), word.clone());
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
        self.taught_up.insert(addr, word.clone());
        let _ = (voice, bytes);
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
