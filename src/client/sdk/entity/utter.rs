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
            for (r, c, b, a) in &chain {
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
    pub fn cite(&mut self, chain: &[(i64, i64, u8, u8)]) -> Cited {
        // The chain's own key, a listing key (17.2): the one address the
        // held stratum does not answer with.
        let own = crate::transducer::element::from_residual(&chain.to_vec()).address();
        let contemplation = &self.contemplation;
        let held_at = |inv: &crate::transducer::stalk::Invariant, piece: &[(i64, i64, u8, u8)]| {
            contemplation
                .held_at(inv)
                .into_iter()
                .filter(|v| !matches!(v, crate::transducer::sentence::Vertex::Word { address, .. } if *address == own))
                .filter_map(|v| crate::transducer::sentence::vertex_cites(v, piece))
                .collect::<Vec<_>>()
        };
        let located = self.epigenome.locate_on(chain, None, &held_at);
        let n_genome = self.epigenome.genome_len();
        let dialect_addrs: Vec<Vec<u8>> = self
            .epigenome
            .dialect
            .fiber(crate::network::nerve::ROOT)
            .map(|(a, _)| a)
            .collect();
        let word_of = |i: usize| -> Option<&Element> {
            if i < n_genome {
                self.epigenome.genome().get(i)
            } else {
                dialect_addrs.get(i - n_genome).and_then(|a| self.epigenome.working_word(a))
            }
        };
        // Standing is a coordinate on the cited word (Ruled, Stood, Held),
        // never a weight multiplied into μ⃗.
        let standing_of = |i: usize| -> Standing {
            if i < n_genome {
                Standing::Ruled
            } else {
                Standing::Stood
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
        // A held cite carries the body in the frame the vertex's action
        // was evaluated in — the chain's own — so its transport is the
        // identity and its one section is the vertex's gauge.
        for h in located.held {
            cites.push(dialogue::CiteWire {
                addr: h.address,
                standing: Standing::Held.as_u8(),
                transport: crate::transducer::element::identity_versor(),
                gauge: 0,
                body: h.body,
            });
        }
        let k = self.session.voice().k();
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

    /// A served listing: cite over the working atlas. Residual strictly
    /// below the chain on a lane is expressed; residual that does not
    /// descend is held as thought — the form of the listing's chain
    /// (50.6) — and never expressed.
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
            self.hold_chain(chain.to_vec());
            return Concluded { cites, residual: chain.to_vec(), familiar: false };
        }
        let familiar = !self
            .epigenome
            .express(&residual, self.session.voice().k(), self.now_tu());
        Concluded { cites, residual, familiar }
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
            // 45.1, 45.4: STANDING A FORM IS AN ADMITTED ACT of this mind,
            // so a receipt naming the form is sealed on the mind's own chain
            // and the form stands at the K it took. The stamp is the act's
            // own K, not a number the hypha last replied with.
            let k = self.session.seal(crate::chronometry::ActKind::Stood, e.address().as_bytes());
            self.epigenome.stand_dialect_at(e.clone(), k, tu);
            if !taught {
                codes[i] = 0xff; // asked about on the wire below
            }
        }
        codes
    }

    /// SPEAK sentences as one act, bound to no family and no task: a word
    /// spoken for itself (an axis germ, a crystallized idea).
    pub fn speak(&mut self, sentences: &[Element]) -> io::Result<Vec<u8>> {
        self.speak_with(sentences, &[], &[])
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
    ) -> io::Result<Vec<u8>> {
        if sentences.is_empty() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "no sentence"));
        }
        let tu = self.now_tu();
        let mut codes = vec![0xffu8; sentences.len()];
        let mut open: Vec<usize> = Vec::new();
        for (i, e) in sentences.iter().enumerate() {
            let addr = e.address();
            match self.sealed.get(&addr).copied() {
                Some(k) => {
                    eprintln!(
                        "  sealed {:02x}{:02x}…: refused at K {k}, not uttered again",
                        addr[0], addr[1]
                    );
                    codes[i] = dialogue::SPEECH_REFUSED;
                }
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
        let spoken: Vec<dialogue::Spoken> = fresh
            .iter()
            .map(|&i| dialogue::Spoken {
                tape: sentences[i].to_bytes(),
                standing: Standing::Stood.as_u8(),
                contraction: self.certificate_for(&sentences[i]),
            })
            .collect();
        let body = dialogue::encode_speech_spoken(&voice, task, tu, &spoken);
        let reply = self.session.speak_nerve(body)?;
        let adjudicated = dialogue::decode_speech_reply(&reply).unwrap_or_default();
        let got: Vec<u8> = adjudicated.iter().map(|a| a.code()).collect();
        for i in read_speech_reply(&mut codes, &got) {
            // The utterance was admitted: the second act, and the second
            // receipt on this mind's chain (45.1, 45.4). A refused utterance
            // is not an act and seals nothing.
            let e = sentences[i].clone();
            self.session.seal(crate::chronometry::ActKind::Uttered, e.address().as_bytes());
            self.file_taught(&e);
            if let Some(f) = crate::transducer::sentence::from_element_form(&e) {
                self.contractions.remove(&crate::transducer::sentence::form_vertex_key(&f));
            }
        }
        for (j, &i) in fresh.iter().enumerate() {
            let addr = sentences[i].address();
            match adjudicated.get(j) {
                Some(dialogue::Adjudication::Refused { kind, at, receipt }) => {
                    self.receive_adjudication(&sentences[i], *kind, at.clone(), *receipt, train, task);
                }
                Some(a) => eprintln!(
                    "  adjudicated {:02x}{:02x}…: {} receipt {:02x}{:02x}…",
                    addr[0],
                    addr[1],
                    if a.code() == dialogue::SPEECH_RATIFIED { "ratified" } else { "stood" },
                    a.receipt()[0],
                    a.receipt()[1]
                ),
                None => {}
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
        // THE STRATUM, THEN THE EXACT TEST. This walked the whole of
        // `taught_up` running `same_vertex` against every word this mind
        // had ever uttered, on every sentence of every speak (law 3, and
        // its answer: the carrier was missing a key). `taught_at` names the
        // words that COULD be one vertex with this one; the exact test
        // decides over that fiber and nothing else.
        if let Some(fiber) = vertex_stratum(word).and_then(|at| self.taught_at.get(&at)) {
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
            self.hold_chain(word.sub().to_vec());
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
