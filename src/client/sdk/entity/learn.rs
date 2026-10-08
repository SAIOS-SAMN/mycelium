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

    /// A SOLVE THIS VOICE CAN STAND BEHIND, OR NONE SENT. A solve is a rule,
    /// and the court replays it on the task's withheld pairs; so the voice
    /// first replays it on the pairs it holds — the train — exactly as the
    /// court will (`blind_replay_at` over this mind's own crystal), and:
    /// - a solve with no minted rule is not sent (UNMINTED): a raster is no
    ///   proof of a motion, and the court refuses it by construction;
    /// - a rule its own train refutes is not sent (REFUTED AT HOME);
    /// - a rule the court already refused for this task is not sent again
    ///   until this voice's rule for it changes (WITHHELD AGAIN).
    ///
    /// On the fleet (epoch 106) each voice sent ~600 solves the court
    /// refused, the same rules round after round, with none of these read.
    /// Answers the court's verdict where one was asked for.
    pub fn solve_task(
        &mut self,
        id: &[u8],
        train: &[(Grid, Grid)],
        outputs: &[Grid],
        rule: Option<&Element>,
        budget: &mut crate::physics::Budget,
    ) -> Option<(u8, [u64; 4])> {
        let Some(rule) = rule else {
            self.solves.unminted += 1;
            return None;
        };
        // 78.142 — THE WORD THAT CROSSES TO THE WITHHELD PAIRS carries no
        // overlap between two of its TRAINING frames (`rule::crossing`): the
        // court reads those frame indices on pairs they do not name and
        // refuses them I. A rule that cannot be re-minted without them has
        // nothing to say about a pair it never saw, and is refuted at home.
        let crossed = match crate::client::sdk::rule::crossing(rule, train, &self.epigenome) {
            Ok(e) => e,
            Err(_) => {
                self.solves.refuted += 1;
                return None;
            }
        };
        let rule = &crossed;
        let addr = rule.address();
        if self.solve_refused.get(id).is_some_and(|a| a == addr.as_bytes()) {
            self.solves.withheld_again += 1;
            return None;
        }
        let resolve = |a: &[u8]| self.epigenome.working_word(a).and_then(|e| e.form().cloned());
        if crate::verdict::blind_replay_at(rule, &resolve, Some(train), &self.epigenome, budget).is_some() {
            self.solves.refuted += 1;
            return None;
        }
        let verdict = self.submit_solve_rule(id, outputs, Some(rule)).ok()?;
        // A refusal is remembered against the rule it refused, one per
        // task: the carrier is bounded by the tasks, and a new rule for the
        // task replaces it.
        match verdict.0 {
            dialogue::SOLVE_BOUNTY | dialogue::SOLVE_FLOOR => {
                self.solve_refused.remove(id);
            }
            _ => {
                self.solve_refused.insert(id, addr.as_bytes().to_vec());
            }
        }
        Some(verdict)
    }

    /// SUBMIT A SOLVE: the sentence that spoke the outputs, with them. The
    /// court replays the RULE on its withheld pairs and pays what it
    /// discharges (the actions' physics): (SOLVE_BOUNTY, μ⃗) on the task's
    /// first closure, (SOLVE_FLOOR, 0⃗) on a closure again, (SOLVE_NOTHING,
    /// 0⃗) with the refusal otherwise. A solve without its rule is refused: a
    /// raster is no proof of a motion.
    pub fn submit_solve_rule(
        &mut self,
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
        // THE VOICE HEARS ITS VERDICT (the actions' physics): what the court
        // read of its rule — closed first, closed again, or refused with the
        // replay's reason and kind — folded into this voice's tally. A
        // receipt that does not decode says nothing was solved and nothing
        // was paid, which is what the solve earned, counted unruled.
        Ok(match dialogue::decode_solve_verdict(&body) {
            Some((code, mu, refusal)) => {
                self.solves.note(code, &mu, refusal.as_ref());
                (code, mu)
            }
            None => {
                self.solves.unruled += 1;
                (dialogue::SOLVE_NOTHING, [0; 4])
            }
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
        // HEARING — one page heard, on every path a page arrives by.
        crate::observe::count(&mut self.hearing, &crate::observe::HEARING, "pages");
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
    /// whole rather than served — learned, and crystallized only once a court
    /// admits it (78.49). The root of its act: one stroke, minted here (65.5).
    pub fn absorb_taught(&mut self, taught: &[u8]) -> Option<Comprehension> {
        let mut budget = self.physics.stroke();
        self.absorb_taught_at(taught, None, &mut budget)
    }

    /// Hear a taught word by pivoting the contemplation before banking it.
    /// k is the mind's K (law 9), tu is TUniversal; they are not one clock.
    /// `at` is the court's stamp the page carried beside the word (78.24
    /// (2b), 78.49): filed as the word's locator, its citation's coordinate.
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
        // THE WORDS THE PAGE TAUGHT (78.24, 78.49): a page carrying the
        // court's stamp teaches words the court admitted, and each is filed at
        // that stamp — its citation's coordinate, the court's seal. A page
        // with no stamp teaches words no court has admitted yet: learned, and
        // crystallized only once a court rules them.
        for e in &heard_words {
            if let Some(at) = at {
                self.epigenome.file_admitted(&e.address(), at);
            }
            let _ = self.pivot_on(e, budget);
            // 78.37(1) — FIXED BY CLASS: a word arriving with its admitting
            // stamp is court-ruled, and every thought on its chart is fixed.
            if at.is_some() {
                self.contemplation.fix_by_class(e);
            }
        }
        // 78.49: a page LEARNED is an act of this mind, learned at its own K.
        self.acted = true;
        let k = self.own_k();
        // The caller's stroke (65.5): the page is learned inside its act.
        let heard = self.epigenome.learn_at(taught, k, self.now_tu(), budget);
        // HEARING — every word heard passes here; the ones admitted too.
        crate::observe::count(&mut self.hearing, &crate::observe::HEARING, "words heard");
        // 24.3: a page learned is a mode of the stroke's outcome, measured
        // by what the page brought; a page refused on replay measures
        // nothing. The stroke takes the measure once.
        if heard.is_some() {
            crate::observe::count(&mut self.hearing, &crate::observe::HEARING, "words admitted");
            crate::physics::StrokeOutcome::add(&mut self.learned_mu, &mu);
            // 78.73 — PLASTIC: each court-admitted word this mind took in
            // reshaped its language on the lanes the word occupies.
            if at.is_some() {
                for e in &heard_words {
                    self.physics.plastic(e.lanes());
                }
            }
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
