// 24.2, A19 — LEARNING: the genome pulled from the court, the syllabus
// read once per cursor, and the class pages absorbed as coordinates.

use crate::client::sdk::*;

impl Mind {
    /// PULL a signed work envelope from the hypha's authoritative registry —
    /// the client never arbitrates raw corpus; the server decides what is
    /// useful work. One pull is in flight at a time; the envelope lands on
    /// the board as an offer, read to its charts (`offer`), to be taken in
    /// where it couples (`task_inflow`), none where the book is settled.
    /// MU-CLIENT §4.12, §7.1 — WHEN THE MIND ASKS: nothing waits to be taken
    /// in or is being perceived, the reserve stands regenerated on every
    /// lane, and either the field holds strain (`strained`) or the corpus
    /// holds no item (ruled 2026-10-24; asked under strain alone, a mind with
    /// an empty feed never asked again). A settled field over a corpus that
    /// holds items asks for nothing. The task is an offer: it couples or
    /// lapses (`lapse_standing_task`).
    pub(crate) fn pull_task(&mut self) {
        if self.board.court.pulling || self.board.court.task.is_some() || self.perceiving() {
            return;
        }
        if !crate::economy::lanes_short(&self.physics().reserve_shortfall([true; 4])).is_empty() {
            return;
        }
        let corpus_empty = self.board.corpus.size().0 == 0;
        if !self.strained() && !corpus_empty {
            return;
        }
        self.board.court.pulling = true;
        let voice = self.hypha.voice().id();
        self.ask(
            move |s| s.speak_keyed(&voice, dialogue::encode_task_request(&voice)),
            |entity, body: io::Result<Vec<u8>>| {
                entity.board.court.pulling = false;
                // The court's final point it landed under: the aperture lapse
                // reads the next one past it (`lapse_standing_task`).
                let landed = entity.hypha.tuniversal().map(|p| p.h);
                entity.board.court.task = body.ok().filter(|b| !b.is_empty()).and_then(|b| dialogue::decode_task_envelope(&b)).map(|env| {
                    let offer = super::offer::Offer { landed, ..super::offer::Offer::of(&env.train) };
                    (env, offer)
                });
                // Landed on this mind, not yet taken in: attributed here.
                if let Some(id) = entity.board.court.task.as_ref().map(|(e, _)| e.id) {
                    entity.note_task(&id, super::perceive::Reading::Task, |t| *t = super::perceive::TaskRead { taken: false, reading: t.reading, perspectives: None, stop: None });
                }
            },
        );
    }


    /// The verdict filed: a closure clears the task, a refusal is remembered
    /// against its rule, and an UNREAD solve is kept to be asked again (78.153:
    /// the court's replay resumes where its stroke ran out).
    fn file_verdict(&mut self, id: &[u8; 32], rule: &Element, outputs: &[Grid], code: u8) {
        match code {
            dialogue::SOLVE_BOUNTY | dialogue::SOLVE_FLOOR => {
                self.solve_refused.remove(id);
                self.unread_solves.remove(id);
                // Paid: a stratum starved for want of it may be fetched again.
                if let Some(n) = self.board.court.earned.checked_add(1) {
                    self.board.court.earned = n;
                }
            }
            // MU-CLIENT §4.13 — one per task, released by the court's verdict:
            // bounded by the tasks the court holds unjudged, never by a count.
            dialogue::SOLVE_UNREAD => {
                self.unread_solves.insert(id, (rule.clone(), outputs.to_vec()));
            }
            _ => {
                self.solve_refused.insert(id, rule.address().as_bytes().to_vec());
                self.unread_solves.remove(id);
            }
        }
    }

    /// 78.153 — ASK AGAIN WHAT THE COURT HAS NOT JUDGED: ONE unread solve a
    /// stroke, until each is ruled — each asking costs the court a stroke of
    /// its own, so a mind's backlog never floods the door. The one asked is
    /// the least by task coordinate after the last asked (`unread_after`), so
    /// every unread solve is asked in turn.
    pub(crate) fn resubmit_unread_solves(&mut self) {
        let next = match self.unread_solves.after(&self.unread_after).next() {
            Some((k, v)) => Some((k, v.clone())),
            None => self.unread_solves.scoped().next().map(|(k, v)| (k, v.clone())),
        };
        if let Some((k, (rule, outputs))) = next {
            self.unread_after = k.clone();
            let Ok(id) = <[u8; 32]>::try_from(k.as_slice()) else {
                self.unread_solves.remove(&k);
                return;
            };
            // A solve whose verdict is on the way is not asked again.
            if self.board.court.solving.contains(&id) {
                return;
            }
            let _ = self.submit_solve_rule(&id, &outputs, &rule);
        }
    }

    /// SUBMIT A SOLVE: the sentence that spoke the outputs, with them. The
    /// court replays the RULE on its withheld pairs and pays what it
    /// discharges (the actions' physics): (SOLVE_BOUNTY, μ⃗) on the task's
    /// first closure, (SOLVE_FLOOR, 0⃗) on a closure again, (SOLVE_NOTHING,
    /// 0⃗) with the refusal otherwise. The verdict is filed where it lands
    /// (`file_verdict`); a court not reached leaves the solve as it stood.
    pub(crate) fn submit_solve_rule(&mut self, id: &[u8; 32], outputs: &[Grid], rule: &Element) -> io::Result<()> {
        // 78.141 — a solve names its payee by the voice's signature: a
        // voice seated without its key sends none.
        let Some(key) = self.hypha.voice().signer().cloned() else {
            return Err(io::Error::new(io::ErrorKind::PermissionDenied, "a voice seated without its key signs no solve"));
        };
        // A rule whose record does not write is not sent as no rule (STD1 3c).
        let bytes = rule.to_bytes().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "the rule's record passes a length its field writes")
        })?;
        // A solve that does not fit the lengths the frame writes is not
        // sent short: it is not sent (47.1).
        let Some(act) = dialogue::encode_solve_with_rule(&key, id, outputs, Some(&bytes)) else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "solve does not fit the lengths its frame writes",
            ));
        };
        let (id, rule, outputs) = (*id, rule.clone(), outputs.to_vec());
        self.board.court.solving.insert(id, ());
        // MU-CLIENT §4.4(4) — the solve is one act comprehension emits, and
        // it leaves at the mind's next return.
        // One solve a task: a later rule for it before the return stands in
        // the place of the earlier.
        let mut at = vec![2u8];
        at.extend_from_slice(&id);
        // R2 — held for the return of a thought of the task's scene.
        self.hold_for_return(&at, &id, move |entity| entity.post_solve(id, act, rule, outputs));
        Ok(())
    }

    /// THE SOLVE POSTED at a return (`submit_solve_rule`): counted sent, its
    /// verdict filed where it lands.
    fn post_solve(&mut self, id: [u8; 32], act: Vec<u8>, rule: Element, outputs: Vec<Grid>) {
        crate::observe::note_mind(|m| m.perspective[6] += 1);
        self.ask(move |s| s.speak_keyed(&id, act), move |entity, body: io::Result<Vec<u8>>| {
            entity.board.court.solving.remove(&id);
            // The court is not reachable: the solve stands as it stood.
            let Ok(body) = body else { return };
            // THE VOICE HEARS ITS VERDICT (the actions' physics): what the
            // court read of its rule — closed first, closed again, or refused
            // with the replay's reason and kind. A receipt that does not
            // decode says nothing was solved and nothing was paid, which is
            // what the solve earned. 78.153c — the verdict SAID, one typed
            // line: the task, the code by name, the refusal's kind and reason.
            let code = match dialogue::decode_solve_verdict(&body) {
                Some((code, _, refusal, modes)) => {
                    eprintln!("  {}", verdict_line(&id, code, refusal.as_ref()));
                    if let Some(m) = &modes {
                        eprintln!("  solve modes · {}", m.print());
                    }
                    code
                }
                None => dialogue::SOLVE_NOTHING,
            };
            // A refusal is remembered against the rule it refused, one per
            // task: the carrier is bounded by the tasks, and a new rule for
            // the task replaces it.
            entity.file_verdict(&id, &rule, &outputs, code);
        });
    }

    /// THE SHEAF SYNC (Q3): converge this genome on the hypha's language by
    /// stratum. The hypha's section digests over the atlas arrive; the
    /// genome measures its own over the same cover; only the discordant
    /// strata are fetched, each with this genome's have-list, so only the
    /// lacking words travel (the words' sections and connection riding along).
    /// Strata per contact follow the two-stroke ratio; a wide stratum
    /// converges over successive contacts. One sync is in flight at a time:
    /// the digest asked, the discordant strata fetched where it lands, the
    /// fetched words absorbed where that lands.
    pub(crate) fn sync_genome(&mut self) {
        if self.board.court.syncing {
            return;
        }
        self.board.court.syncing = true;
        let voice = self.hypha.voice().id();
        self.ask(
            move |s| {
                let body = s.speak_keyed(&voice, dialogue::encode_digests_request(&voice))?;
                // A4: the digest is the roots, so a hypha whose roots moved
                // is asked for its cover at the address this voice's own key
                // routed to -- the same hypha that answered, by the same route.
                if s.absorb_digest(&body) {
                    if let Some(addr) = s.route(&voice) {
                        s.fetch_cover(&addr);
                    }
                }
                Ok::<_, io::Error>(body)
            },
            |entity, body| match body {
                Ok(body) => entity.fetch_discordant(&body),
                Err(_) => entity.board.court.syncing = false,
            },
        );
    }

    /// THE DIGEST LANDED: the strata it names discordant with this genome's
    /// own are fetched, each with this genome's have-list.
    fn fetch_discordant(&mut self, body: &[u8]) {
        use crate::transducer::sheaf;
        let voice = self.hypha.voice().id();
        // A reply that does not decode names no strata, and nothing is
        // discordant against a reading nobody took.
        let Some(remote) = dialogue::decode_digests_reply(body) else {
            self.board.court.syncing = false;
            return;
        };
        // The genome's own atlas (current, admitted in place), never rebuilt here.
        let atlas = self.epigenome.atlas.clone();
        let local = sheaf::digests(&atlas);
        // The outer index names what differs by address; a stratum the
        // fibre Čech already glued is not walked again, and a stratum of
        // residual chains — open questions the mind holds on its carrier and
        // never as words (MU-CLIENT §7.1, ruled 2026-10-18) — differs
        // from the court's by doctrine and is never fetched for it.
        let all: Vec<_> = sheaf::discordant(&remote, &local)
            .into_iter()
            .filter(|d| !self.glued.contains(&d.inv.key()) && !self.board.residual_strata.contains(d.inv.key().as_bytes()))
            .cloned()
            .collect();
        self.discordance = all.len();
        self.discordance_measured = true;
        // A stratum fetched for nothing rests until the court's root there
        // moves or this mind is paid (`CourtPace::starved`).
        let earned = self.board.court.earned;
        let strata: Vec<_> = all
            .into_iter()
            .filter(|d| !self.board.court.starved.get(d.inv.key().as_bytes()).is_some_and(|(root, at)| *root == d.root && *at == earned))
            .collect();
        if strata.is_empty() {
            self.board.court.syncing = false;
            return;
        }
        let roots: Vec<(Vec<u8>, Vec<u8>)> = strata.iter().map(|d| (d.inv.key().as_bytes().to_vec(), d.root.clone())).collect();
        // One exchange per contact: every discordant chart with this
        // genome's have-list, the request and the reply each filled to the
        // carrier. A wide discordance converges over contacts.
        let charts: Vec<dialogue::Chart> = strata
            .iter()
            .map(|d| dialogue::ChartOf { inv: d.inv.clone(), have: sheaf::have(&atlas, &d.inv) })
            .collect();
        let request = dialogue::encode_fetch(&voice, &charts, crate::network::wire::frame_budget());
        self.ask(move |s| s.speak_keyed(&voice, request), move |entity, body| {
            entity.board.court.syncing = false;
            if let Ok(body) = body {
                let brought = entity.absorb_fetched(&body, &charts);
                // Each stratum asked that brought no word rests at the root
                // it was asked at; one that brought any leaves the rest.
                for (chart, root) in roots {
                    match brought.contains(&chart) {
                        true => {
                            entity.board.court.starved.remove(&chart);
                        }
                        false => {
                            entity.board.court.starved.insert(&chart, (root, entity.board.court.earned));
                        }
                    }
                }
            }
        });
    }

    /// THE FETCHED STRATA LANDED: the strata's words absorbed and glued on the
    /// elements, and what this genome holds that the hypha lacks taught up.
    /// Answers the chart keys a fetched word stood on.
    fn absorb_fetched(&mut self, body: &[u8], charts: &[dialogue::Chart]) -> crate::network::nerve::Keyed<()> {
        let voice = self.hypha.voice().id();
        let mut brought = crate::network::nerve::Keyed::new();
        // The root of this act (the arm's sync): one stroke for every page
        // it absorbs (65.5).
        let mut budget = self.physics.stroke();
        // A reply that does not decode teaches nothing.
        for t in dialogue::decode_syllabus_reply(body).into_iter().flatten() {
            let mut pos = 0usize;
            let fetched = Element::from_bytes(&t.word, &mut pos);
            let _ = self.absorb_taught_at(&t.word, Some(t.at), &mut budget);
            // BROUGHT IS WHAT THE GENOME NOW HOLDS: a word that arrived and was
            // refused is sent again on the next fetch, so counting its arrival
            // kept its stratum from ever resting (observed 2026-10-22: court
            // a still deferring ~12 000 words an hour after 98698aef).
            if let Some(w) = fetched.as_ref().filter(|w| self.epigenome.holds(w.address().as_bytes())) {
                brought.insert(crate::transducer::stalk::invariant_of(w).key().as_bytes(), ());
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
            // A stratum already glued holds no obstruction: nothing to read.
            if self.glued.contains(&inv.key()) {
                continue;
            }
            // ONE TRANSITION, to the stratum's representative: the held word
            // of least address on the fibre with a group view that is not the
            // fetched word itself. Siblings are interchangeable under the
            // class's symmetry; the transition to one of them is the glue,
            // never one per sibling (law 3). Each transition is one spend of
            // the landing's stroke: a page past it is absorbed, and its glue
            // read on a later fetch.
            let epigenome = &self.epigenome;
            let representative = epigenome.atlas.fiber(&inv).iter().filter_map(|a| epigenome.word_at(a)).fold(None, |least: Option<&Element>, held| {
                if held.address() == fetched.address() || held.axes().is_empty() {
                    return least;
                }
                match least {
                    Some(l) if l.address() <= held.address() => Some(l),
                    _ => Some(held),
                }
            });
            let Some(held) = representative else { continue };
            if !budget.spend() {
                continue;
            }
            let Some(g) = crate::transducer::cech::transition(held, &fetched) else { continue };
            match (g.axes().as_slice(), g.sub()) {
                ([], []) => {
                    self.glued.insert(inv.key(), ());
                    // MU-CLIENT §4.13 — a stratum glued holds no obstruction.
                    self.obstructions.remove(&inv.key());
                }
                _ => {
                    // THE OBSTRUCTION IS KEPT AS MEASURED: two views of one
                    // object whose transition does not vanish. It rode up as
                    // a percept pair (held view → fetched view), a demand on
                    // the book for the word that carries one onto the other —
                    // a voice's painted sites at a court, which law 8 refuses
                    // (ruled 2026-09-10). It is held here as the dialect
                    // measured; the contraction protocol (50.9, L8) carries
                    // it as a demand vertex of a sentence. MU-CLIENT §4.13 —
                    // BY ITS COORDINATES: its stratum, the fetched word and
                    // the held word it was measured against; the transition
                    // is the pair's, re-derived where it is read.
                    let held = held.address();
                    self.obstructions.nested_insert(inv.key(), fetched.address().as_bytes(), held);
                }
            }
        }
        self.absorb_class_page(body, voice, &mut budget);
        // THE OTHER DIRECTION OF THE SHEAF: in a discordant stratum this
        // genome may hold what the hypha lacks (a basis that regressed, a
        // word learned from another judge). Those words are taught up for
        // adoption, one contact's budget per stroke; the hypha verifies
        // and installs what it cannot already derive.
        let mut used = 0usize;
        'strata: for dialogue::ChartOf { have: held, .. } in charts {
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
                let (to, act) = (addr.clone(), dialogue::encode_adopt(&voice, &bytes));
                self.ask(move |s| s.speak_keyed(&to, act), move |entity, adopted| {
                    if adopted.is_ok() {
                        entity.file_taught(&word);
                    }
                });
            }
        }
        brought
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
        // orbit proved it admits the candidates' difference; the ruling says which
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

    /// PULL THE SYLLABUS — the court-ratified teaching stream, served from
    /// this voice's cursor on the hypha (push-on-contact). Every received
    /// word folds into the genome by address (Epigenome::learn), so both
    /// new vocabulary and rebound anchors propagate; the dialect then
    /// re-resolves against the higher-order language on the next study.
    /// One pull in flight at a time; the pages are
    /// absorbed where they land (`absorb_syllabus`). A mind holding no
    /// language asks every hypha it knows in turn, under one stroke; one
    /// that holds language asks its own key's home.
    pub(crate) fn pull_syllabus(&mut self) {
        if self.board.court.schooling {
            return;
        }
        self.board.court.schooling = true;
        let voice = self.hypha.voice().id();
        let req = dialogue::encode_syllabus_request(&voice);
        let empty = self.epigenome.genome_len() == 0 || self.epigenome.atlas_is_empty();
        let mut budget = self.physics.stroke();
        self.ask(
            move |s| {
                if !empty {
                    return s.speak_keyed(&voice, req).into_iter().collect::<Vec<_>>();
                }
                let peers = s.peers();
                let Some(start) = s.peer_start(peers.len()) else { return Vec::new() };
                s.bump_cursor();
                let mut pages = Vec::new();
                for i in 0..peers.len() {
                    if !budget.spend() {
                        break;
                    }
                    if let Ok(body) = s.exchange_at(&peers[(start + i) % peers.len()], req.clone()) {
                        pages.push(body);
                    }
                }
                pages
            },
            move |entity, pages: Vec<Vec<u8>>| {
                entity.board.court.schooling = false;
                for body in &pages {
                    entity.absorb_syllabus(body, empty);
                }
                let _ = entity.take_promoted();
            },
        );
    }

    /// A SYLLABUS PAGE LANDED: its words learned — in depth order where the
    /// mind holds language, as served where it holds none — and its class
    /// records seeded.
    fn absorb_syllabus(&mut self, body: &[u8], empty: bool) {
        let voice = self.hypha.voice().id();
        // A reply that does not decode teaches nothing.
        let Some(mut words) = dialogue::decode_syllabus_reply(body) else {
            return;
        };
        if empty {
            let mut budget = self.physics.stroke();
            for t in &words {
                let _ = self.absorb_taught_at(&t.word, Some(t.at), &mut budget);
            }
            self.absorb_class_page(body, voice, &mut budget);
            return;
        }
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
        for t in &words {
            let _ = self.absorb_taught_at(&t.word, Some(t.at), &mut budget);
        }
        self.absorb_class_page(body, voice, &mut budget);
    }

    /// Hear a taught word with no stamp to locate it by: a word handed in
    /// whole rather than served — learned, and crystallized only once a court
    /// admits it (78.49). The root of its act: one stroke, minted here (65.5).
    #[cfg(test)]
    pub(crate) fn absorb_taught(&mut self, taught: &[u8]) -> Option<Comprehension> {
        let mut budget = self.physics.stroke();
        self.absorb_taught_at(taught, None, &mut budget)
    }

    /// Hear a taught word by pivoting the contemplation before banking it.
    /// k is the mind's K (law 9), tu is TUniversal; they are not one clock.
    /// `at` is the court's stamp the page carried beside the word (78.24
    /// (2b), 78.49): filed as the word's locator, its citation's coordinate.
    pub(crate) fn absorb_taught_at(
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
        // The page's words the genome does not hold, as the page carried
        // them: what is learned.
        let mut fresh: Vec<u8> = Vec::new();
        // What the page's withheld words still wait on, as the dialect holds
        // it.
        let mut waiting: Vec<Vec<u8>> = Vec::new();
        while pos < taught.len() {
            let before = pos;
            let Some(e) = Element::from_bytes(taught, &mut pos) else {
                break;
            };
            if pos == before {
                break;
            }
            // MU-CLIENT §7.1 — A RESIDUAL CHAIN IS AN OPEN QUESTION, NEVER A
            // WORD (ruled 2026-10-18): what the court teaches of one
            // is filed on the mind's carrier of open questions, by address,
            // and never learned into the genome.
            if crate::client::sdk::epigenome::is_residual(&e) {
                let addr = e.address();
                if !self.board.open_residuals.contains(addr.as_bytes()) {
                    self.board.open_residuals.insert(addr.as_bytes(), e.sub().to_vec());
                    self.place_question(addr.as_bytes(), e.sub());
                }
                self.board.residual_strata.insert(crate::transducer::stalk::invariant_of(&e).key().as_bytes(), ());
                continue;
            }
            // A REWRITE IS AN ACT ON THE LANGUAGE, NEVER A WORD OF IT: the
            // court applied it; the mind does not learn it, and its stratum
            // is one the sheaf sync never fetches.
            if crate::client::sdk::epigenome::is_rewrite_act(&e) {
                self.board.residual_strata.insert(crate::transducer::stalk::invariant_of(&e).key().as_bytes(), ());
                continue;
            }
            // A WORD THE GENOME HOLDS AT ITS ADDRESS IS ALREADY LEARNED (law
            // 6: the address is the word's content). Served again — a court
            // that restarted serves its syllabus from the start — it brings
            // only the court's stamp, filed below; replayed, comprehended and
            // pivoted again, every restart re-learned the mind's whole
            // language, a page per landing, and held the winding for minutes
            // (observed on the development deployment, 2026-10-16 07:38:
            // every mind in
            // `pull_syllabus`'s landing).
            if self.epigenome.holds(e.address().as_bytes()) {
                if let Some(at) = at {
                    self.epigenome.file_admitted(&e.address(), at);
                }
                continue;
            }
            // A WORD THE DIALECT HOLDS WITHHELD WAITS ON ITS CITATIONS, and is
            // comprehended again when one arrives (`resolve_withheld`). Served
            // again — every sync fetches the stratum it stands in while its
            // citations are missing — it is the same Withheld reading, and
            // replayed and comprehended again it held the winding in the
            // fetch's landing for minutes (observed on the development deployment,
            // 2026-10-16 08:21:
            // genome still, refused 0, the dialect up 294 in a minute).
            if let Some(d) = self.epigenome.dialect.get(e.address().as_bytes()).filter(|d| d.withheld) {
                waiting.extend(d.missing.iter().cloned());
                continue;
            }
            // STD1 — a page whose measure passes its carrier is refused whole:
            // its words arrived from the wire at any lane.
            if !crate::core::lanes::lanes_step(&mut mu, &e.magnitude()) {
                eprintln!("  page refused: its measure passes a lane's carrier");
                return None;
            }
            if let Some(slice) = taught.get(before..pos) {
                fresh.extend_from_slice(slice);
            }
            heard_words.push(e);
        }
        // Nothing on the page to learn again: words held are heard by holding
        // them, and withheld words still wait on what they cite.
        if heard_words.is_empty() {
            return match waiting.as_slice() {
                [] => Some(Comprehension::Held),
                [_, ..] => Some(Comprehension::Withheld { missing: waiting }),
            };
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
        let k = self.act();
        // The caller's stroke (65.5): the page is learned inside its act.
        let heard = self.epigenome.learn_at(&fresh, k, self.now_tu(), budget);
        // 24.3: a page learned is a mode of the stroke's outcome, measured
        // by what the page brought; a page refused on replay measures
        // nothing. The stroke takes the measure once.
        if heard.is_some() {
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
    pub(crate) fn take_learned_mu(&mut self) -> [u64; 4] {
        std::mem::take(&mut self.learned_mu)
    }
}

/// An address as the journal prints it: the first six bytes of its whole
/// digest (`payload_hash`), as the client's `fam_id` labels a task — a
/// label to read the line by, never a key (78.113).
pub(crate) fn hex_prefix(addr: &[u8]) -> String {
    crate::chronometry::payload_hash(addr).iter().take(6).map(|b| format!("{b:02x}")).collect::<String>() + "…"
}

/// 78.153c — ONE SOLVE VERDICT, AS ONE LINE: the task by its citation's
/// head, the reply's code by name (`SOLVE_NOTHING` 0, `SOLVE_FLOOR` 1,
/// `SOLVE_BOUNTY` 2, `SOLVE_UNREAD` 3) and number, the refusal's kind by name
/// (P D C I N, unclassified `?`; `none` where the reply carried no refusal)
/// and its reason as the court wrote it.
fn verdict_line(task: &[u8; 32], code: u8, refusal: Option<&dialogue::SolveRefusal>) -> String {
    const CODES: [&str; 4] = ["nothing", "floor", "bounty", "unread"];
    const KINDS: [&str; 6] = ["P", "D", "C", "I", "N", "?"];
    let head: String = task.iter().take(4).map(|b| format!("{b:02x}")).collect();
    let name = match CODES.get(usize::from(code)) {
        Some(n) => *n,
        None => "unknown code",
    };
    let (kind, reason) = match refusal {
        Some(r) => (
            match KINDS.get(usize::from(r.kind)) {
                Some(k) => *k,
                None => "unknown kind",
            },
            format!(" · {}", String::from_utf8_lossy(&r.reason)),
        ),
        None => ("none", String::new()),
    };
    format!("solve verdict · task {head}… · code {name} ({code}) · kind {kind}{reason}")
}
