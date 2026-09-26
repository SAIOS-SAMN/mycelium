// 24.2, A19 — LEARNING: the genome pulled from the court, the syllabus
// read once per cursor, and the class pages absorbed as coordinates.

use crate::client::sdk::*;

impl Entity {
    /// PULL a signed work envelope from the hypha's authoritative registry —
    /// the client never arbitrates raw corpus; the server decides what is
    /// useful work. None = the book is settled (or no registry).
    pub fn pull_task(&self) -> io::Result<Option<dialogue::TaskEnvelope>> {
        let voice = self.session.kernel().voice();
        let body = self.session.speak_keyed(&voice, dialogue::encode_task_request(&voice))?;
        if body.is_empty() {
            return Ok(None);
        }
        Ok(dialogue::decode_task_envelope(&body))
    }

    /// SUBMIT predicted outputs for a dispatched task. The hypha verifies
    /// against its held answers: (SOLVE_BOUNTY, μ⃗) on the first verified
    /// solve, (SOLVE_FLOOR, μ⃗) on a replay, (SOLVE_NOTHING, 0⃗) otherwise —
    /// the anti-replay collapse is the server's ruling, never the client's.
    pub fn submit_solve(
        &self,
        id: &[u8],
        outputs: &[Grid],
    ) -> io::Result<(u8, [u64; 4])> {
        self.submit_solve_rule(id, outputs, None)
    }

    /// Submit predicted outputs together with the sentence that spoke them.
    pub fn submit_solve_rule(
        &self,
        id: &[u8],
        outputs: &[Grid],
        rule: Option<&Element>,
    ) -> io::Result<(u8, [u64; 4])> {
        let voice = self.session.kernel().voice();
        // A rule whose record does not write is not sent as no rule (STD1 3c).
        let bytes = match rule {
            Some(e) => Some(e.to_bytes().ok_or_else(|| {
                io::Error::new(io::ErrorKind::InvalidInput, "the rule's record passes a length its field writes")
            })?),
            None => None,
        };
        // A solve that does not fit the lengths the frame writes is not
        // sent short: it is not sent (47.1).
        let Some(act) = dialogue::encode_solve_with_rule(&voice, id, outputs, bytes.as_deref())
        else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "solve does not fit the lengths its frame writes",
            ));
        };
        let body = self.session.speak_keyed(id, act)?;
        // A receipt that does not decode says nothing was solved and
        // nothing was paid, which is what the solve earned.
        Ok(match dialogue::decode_solve_receipt(&body) {
            Some(r) => r,
            None => (dialogue::SOLVE_NOTHING, [0; 4]),
        })
    }

    /// PULL THE SYLLABUS — the court-ratified teaching stream, served from
    /// this voice's cursor on the hypha (push-on-contact). Every received
    /// word folds into the genome by address (Epigenome::learn), so both
    /// new vocabulary and rebound anchors propagate; the dialect then
    /// re-resolves against the higher-order language on the next study.
    /// Returns how many words were learned this contact.
    /// THE SHEAF SYNC (Q3): converge this genome on the hypha's language by
    /// stratum. The hypha's section digests over the atlas arrive; the
    /// genome measures its own over the same cover; only the discordant
    /// strata are fetched, each with this genome's have-list, so only the
    /// lacking words travel (their sections and connection riding along).
    /// Strata per contact follow the two-stroke ratio; a wide stratum
    /// converges over successive contacts. Returns the words learned.
    pub fn sync_genome(&mut self) -> io::Result<usize> {
        use crate::transducer::sheaf;
        let voice = self.session.kernel().voice();
        let body = self.session.speak_keyed(&voice, dialogue::encode_digests_request(&voice))?;
        // A4: the digest is the roots, so a hypha whose roots moved is
        // asked for its cover at the address this voice's own key routed
        // to -- the same hypha that answered, by the same route.
        if self.session.absorb_digest(&body) {
            if let Some(addr) = self.session.route(&voice) {
                self.session.fetch_cover(&addr);
            }
        }
        // A reply that does not decode names no strata, and nothing is
        // discordant against a reading nobody took.
        let Some(remote) = dialogue::decode_digests_reply(&body) else {
            return Ok(0);
        };
        // The genome's own atlas (current, admitted in place), never rebuilt here.
        let atlas = self.epigenome.atlas.clone();
        let local = sheaf::digests(&atlas);
        // The outer index names what differs by address; a stratum the
        // fibre Čech already glued is not walked again.
        let strata: Vec<_> = sheaf::discordant(&remote, &local)
            .into_iter()
            .filter(|d| !self.glued.contains(&d.inv.key()))
            .cloned()
            .collect();
        self.discordance = strata.len();
        self.discordance_measured = true;
        // One exchange per contact: every discordant chart with this
        // genome's have-list, the request and the reply each filled to the
        // carrier. A wide discordance converges over contacts.
        let charts: Vec<dialogue::Chart> = strata
            .iter()
            .map(|d| dialogue::ChartOf { inv: d.inv.clone(), have: sheaf::have(&atlas, &d.inv) })
            .collect();
        let request = dialogue::encode_fetch(&voice, &charts, crate::network::wire::frame_budget());
        let mut learned = 0usize;
        let body = self.session.speak_keyed(&voice, request)?;
        // The root of this act (the arm's sync): one stroke for every page
        // it absorbs (65.5).
        let mut budget = self.physics.stroke();
        // A reply that does not decode teaches nothing.
        for t in dialogue::decode_syllabus_reply(&body).into_iter().flatten() {
            let mut pos = 0usize;
            let fetched = Element::from_bytes(&t.word, &mut pos);
            if self.absorb_taught_at(&t.word, Some(t.at), &mut budget).is_some() {
                learned += 1;
            }
            // ČECH AFTER FETCH: the word is glued on the elements, not on
            // the root that indexed it. Every held word on the same fibre
            // with a group view is a chart overlapping this one; the
            // transition between them is the obstruction, and only a
            // transition with no axis and no chain glues. What does not
            // vanish is kept as the dialect measured, never discarded.
            let Some(fetched) = fetched else { continue };
            if fetched.axes().as_slice().is_empty() {
                continue; // no group view: the additive coboundary has no transition
            }
            let inv = crate::transducer::stalk::invariant_of(&fetched);
            let fibre: Vec<Vec<u8>> = self.epigenome.atlas.fiber(&inv).to_vec();
            // One transition, to the stratum's representative: the first
            // held word on the fibre with a group view that is not the
            // fetched word itself. Siblings are interchangeable under the
            // class's symmetry; the transition to one of them is the glue.
            let mut held_g: Option<(&Element, crate::transducer::element::Element)> = None;
            for a in &fibre {
                let Some(held) = self.epigenome.word_at(a) else {
                    continue;
                };
                if held.address() == fetched.address() || held.axes().is_empty() {
                    continue;
                }
                let Some(g) = crate::transducer::cech::transition(held, &fetched) else {
                    continue;
                };
                match (g.axes().as_slice(), g.sub()) {
                    ([], []) => {
                        held_g = Some((held, g));
                        break;
                    }
                    _ => match &held_g {
                        None => held_g = Some((held, g)),
                        Some((h, _)) if held.address() < h.address() => held_g = Some((held, g)),
                        _ => {}
                    },
                }
            }
            let Some((_, g)) = held_g else { continue };
            match (g.axes().as_slice(), g.sub()) {
                ([], []) => {
                    self.glued.insert(inv.key(), ());
                }
                _ => {
                    if !self.obstructions.contains(&fetched.address()) {
                        // THE OBSTRUCTION IS KEPT AS MEASURED: two views of
                        // one object whose transition does not vanish. It
                        // rode up as a percept pair (held view → fetched
                        // view), a demand on the book for the word that
                        // carries one onto the other — a voice's painted sites at a
                        // court, which law 8 refuses (ruled 2026-09-10). It
                        // is held here as the dialect measured; the
                        // contraction protocol (50.9, L8) carries it as a
                        // demand vertex of a sentence.
                        self.obstructions.insert(fetched.address(), g);
                    }
                }
            }
        }
        self.absorb_class_page(&body, voice, &mut budget);
        // THE OTHER DIRECTION OF THE SHEAF: in a discordant stratum this
        // genome may hold what the hypha lacks (a basis that regressed, a
        // word learned from another judge). Those words are taught up for
        // adoption, one contact's budget per stroke; the hypha verifies
        // and installs what it cannot already derive.
        let mut taught = 0usize;
        let mut used = 0usize;
        'strata: for dialogue::ChartOf { have: held, .. } in &charts {
            for addr in held {
                if self.already_taught_addr(addr) {
                    continue; // spoken for adoption once; the judge decided
                }
                let Some(word) = self.epigenome.word_at(addr).cloned() else { continue };
                if self.already_taught(&word) {
                    continue;
                }
                // A word whose record does not write is not taught (STD1 3c).
                let Some(bytes) = word.to_bytes() else {
                    eprintln!("  adopt refused: a word's record passes a length its field writes");
                    continue;
                };
                if used + bytes.len() > crate::network::wire::frame_budget() {
                    break 'strata;
                }
                used += bytes.len();
                if self.session.speak_keyed(addr, dialogue::encode_adopt(&voice, &bytes)).is_ok() {
                    self.file_taught(&word);
                    taught += 1;
                }
            }
        }
        Ok(learned + taught)
    }

    pub(crate) fn absorb_class_page(&mut self, body: &[u8], _from: [u8; 32], budget: &mut crate::physics::Budget) {
        let pages = dialogue::decode_class_records(body);
        let mut recs = Vec::new();
        for p in pages {
            recs.push(crate::transducer::bubble::ClassRecord {
                key: p.key,
                order: crate::transducer::alphabet::Order::from_u32(p.order),
                representative: p.rotor,
            });
        }
        // A RULING COLLAPSES A SUPERPOSITION AND THE OTHER HYPOTHESIS IS A
        // SYMMETRY (Expansion 4). Two candidates that both discharged one
        // orbit proved it admits their difference; the ruling says which
        // discharge the mesh holds, and the difference remains a rotor this
        // mind measured. It is adjoined rather than cleared — the language
        // keeps what the collapse resolved.
        let (_, symmetries) = self.contemplation.seed_horizon_with(&recs);
        if !symmetries.is_empty() {
            let chart = crate::transducer::stalk::Invariant::default();
            let adjoined: crate::transducer::bubble::drive::Adjoined =
                symmetries.into_iter().map(|r| (r, chart.clone())).collect();
            self.adjoin_letters(&adjoined, budget);
        }
    }

    pub fn pull_syllabus(&mut self) -> io::Result<usize> {
        let voice = self.session.kernel().voice();
        let empty = self.epigenome.genome_len() == 0 || self.epigenome.atlas_is_empty();
        if empty {
            let peers = self.session.peers();
            if peers.is_empty() {
                return Ok(0);
            }
            let mut budget = self.physics.stroke();
            let n = peers.len();
            let Some(start) = self.session.peer_start(n) else { return Ok(0) };
            self.session.bump_cursor();
            let mut learned = 0usize;
            let req = dialogue::encode_syllabus_request(&voice);
            for i in 0..n {
                if !budget.spend() {
                    break;
                }
                let addr = &peers[(start + i) % n];
                let Ok(body) = self.session.exchange_at(addr, req.clone()) else {
                    continue;
                };
                // A reply that does not decode teaches nothing.
                for t in dialogue::decode_syllabus_reply(&body).into_iter().flatten() {
                    if self.absorb_taught_at(&t.word, Some(t.at), &mut budget).is_some() {
                        learned += 1;
                    }
                }
                self.absorb_class_page(&body, voice, &mut budget);
            }
            return Ok(learned);
        }
        let body = self.session.speak_keyed(&voice, dialogue::encode_syllabus_request(&voice))?;
        // A reply that does not decode teaches nothing.
        let Some(mut words) = dialogue::decode_syllabus_reply(&body) else {
            return Ok(0);
        };
        // DEPTH ORDER IS THE DOCTRINE, not a choice by a scalar: "a mind
        // never receives depth d before it holds every cited form below d;
        // the syllabus serves in depth order" (chapter 6, 24.2), and
        // chapter 4's well-foundedness rests on it. What is not doctrine is
        // `unwrap_or(0)`: a word whose tape does not re-derive as a form
        // was given DEPTH ZERO and sorted to the front, ahead of the words
        // it might cite (rule 11, checklist (i)). A tape that does not
        // parse has no depth; it is learned last, where it can cite
        // everything already in hand, and `learn_one` refuses it there on
        // its own replay.
        #[expect(
            clippy::disallowed_methods,
            reason = "rule 4 — canonical key order: depth order is chapter 6's own (24.2), the order a syllabus is served in, and a tape that does not parse has no depth and is learned last"
        )]
        words.sort_by_key(|t| {
            let mut pos = 0usize;
            match crate::transducer::element::Element::from_bytes(&t.word, &mut pos) {
                // An ARRIVING syllabus tape: nothing holds it yet, so the
                // decode is named here (L16).
                Some(e) => match crate::transducer::sentence::from_element_form(&e) {
                    Some(f) => (0u8, f.depth),
                    None => (1u8, 0),
                },
                None => (1u8, 0),
            }
        });
        // The root of this act (the arm's pull): one stroke for every page
        // it absorbs (65.5).
        let mut budget = self.physics.stroke();
        let mut learned = 0usize;
        for t in &words {
            if self.absorb_taught_at(&t.word, Some(t.at), &mut budget).is_some() {
                learned += 1;
            }
        }
        self.absorb_class_page(&body, voice, &mut budget);
        Ok(learned)
    }

    /// Hear a taught word with no stamp to locate it by: a word handed in
    /// whole rather than served, admitted by the Learned node that hears it
    /// (78.27, LD3-c). The root of its act: one stroke, minted here (65.5).
    pub fn absorb_taught(&mut self, taught: &[u8]) -> Option<Comprehension> {
        let mut budget = self.physics.stroke();
        self.absorb_taught_at(taught, None, &mut budget)
    }

    /// Hear a taught word by pivoting the contemplation before banking it.
    /// k is the voice's K (law 9), tu is TUniversal; they are not one clock.
    /// `at` is the stamp the page carried beside the word (78.24 (2b)):
    /// filed as the word's locator, it is what the Learned node cites.
    pub fn absorb_taught_at(
        &mut self,
        taught: &[u8],
        at: Option<crate::chronometry::Stamp>,
        budget: &mut crate::physics::Budget,
    ) -> Option<Comprehension> {
        let mut pos = 0usize;
        // WHAT THE PAGE MEASURED, per lane: the sum over the words it
        // carried, lane by lane and never across lanes (law 5). A page is
        // one act and its measure is what it brought.
        let mut mu = [0u64; 4];
        let mut heard_words = Vec::new();
        while pos < taught.len() {
            let before = pos;
            let Some(e) = Element::from_bytes(taught, &mut pos) else {
                break;
            };
            if pos == before {
                break;
            }
            // STD1 — a page whose measure passes its carrier is refused whole:
            // its words arrived from the wire at any lane.
            if !crate::core::lanes::lanes_step(&mut mu, &e.magnitude()) {
                eprintln!("  page refused: its measure passes a lane's carrier");
                return None;
            }
            heard_words.push(e);
        }
        // THE WORDS THE PAGE TAUGHT, cited by their fibers and the stamp
        // that taught them (78.24): one citation per word, never the
        // page's bytes.
        //
        // A WORD THE PAGE CARRIED NO STAMP FOR, and this voice holds none
        // for, is ADMITTED by this page's own Learned node (78.27, LD3-c):
        // the node cites it at the node's own stamp, as `seal_word` admits a
        // word no act has, and the stamp is filed as its locator once the
        // node stands.
        let admitting = match at {
            Some(_) => None,
            None => self.session.voice().chain().next_stamp(&[]),
        };
        let mut cites = Vec::with_capacity(heard_words.len());
        let mut admitted_here = Vec::new();
        for e in &heard_words {
            let addr = e.address();
            if let Some(at) = at {
                self.session.voice_mut().file_admitted(&addr, at);
            }
            match (self.session.voice().cite_of(&addr, e), admitting) {
                (Some(c), _) => cites.push(c),
                (None, Some(here)) => {
                    cites.push(crate::chronometry::Cite::word(e, here));
                    admitted_here.push(addr);
                }
                (None, None) => {}
            }
            let _ = self.pivot_on(e, budget);
            // 78.37(1) — FIXED BY CLASS: a word arriving with its admitting
            // stamp is court-ruled, and every thought on its chart is fixed.
            if at.is_some() {
                self.contemplation.fix_by_class(e);
            }
        }
        // 45.1, 45.4: a page LEARNED is an admitted act of this mind, so a
        // node citing the words is sealed on its own chain and the page is
        // learned at the K it took.
        let k = self.session.seal(crate::chronometry::ActKind::Learned, mu, cites);
        if let Some(here) = admitting {
            if self.session.voice().chain().at_place(here.place()).is_some() {
                for addr in &admitted_here {
                    self.session.voice_mut().file_admitted(addr, here);
                }
            }
        }
        // The caller's stroke (65.5): the page is learned inside its act.
        let heard = self.epigenome.learn_at(taught, k, self.now_tu(), budget);
        // 24.3: a page learned is a mode of the stroke's outcome, measured
        // by what the page brought; a page refused on replay measures
        // nothing. The stroke takes the measure once.
        if heard.is_some() {
            crate::physics::StrokeOutcome::add(&mut self.learned_mu, &mu);
        }
        heard
    }

    /// WHAT THIS MIND LEARNED SINCE THE STROKE LAST ASKED (24.3): the
    /// measure of the pages admitted, per lane, taken once — the stroke's
    /// `learned` mode.
    pub fn take_learned_mu(&mut self) -> [u64; 4] {
        std::mem::take(&mut self.learned_mu)
    }
}
