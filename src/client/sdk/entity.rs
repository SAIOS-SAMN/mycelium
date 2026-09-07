// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! The entity: perceive, think, stand, utter (24.3) — the stroke,
//! the quote, the attestation and what each writes.
//!
//! Split from sdk.rs under Stage 8's reading bound. Whole `impl` blocks
//! move verbatim; a private field is visible to its module and its
//! descendants, so the carriers are reached unchanged.

use super::*;

impl Entity {
    pub fn add_hypha(&mut self, addr: &str) {
        self.session.add_peer(addr);
    }

    /// Ask the bootstrap hypha for the rest of the nerve, one peer per spend.
    pub fn discover(&mut self) {
        let mut budget = self.physics.stroke();
        self.session.discover_budgeted(&mut budget);
    }

    pub fn open(addr: &str, kernel: Kernel) -> Self {
        let mut session = Session::open(addr, kernel);
        session.discover();
        Entity {
            session,
            epigenome: Epigenome::new(),
            refine_cursor: 0,
            taught_up: crate::network::nerve::Keyed::new(),
            discordance: 0,
            discordance_measured: false,
            obstructions: crate::network::nerve::Keyed::new(),
            glued: crate::network::nerve::Keyed::new(),
            contemplation: crate::transducer::bubble::Contemplation::new(),
            thought_gen: 0,
            opened: crate::network::nerve::Keyed::new(),
            ideas: crate::network::nerve::Keyed::new(),
            physics: crate::physics::Physics::default(),
            height: crate::chronometry::Height::default(),
        }
    }

    pub fn set_height(&mut self, h: crate::chronometry::Height) {
        self.height = h;
    }

    pub fn now_tu(&self) -> u64 {
        self.height.get()
    }

    /// Open with CONTINUITY: restore the crystallized genome from `genome_path`
    /// if present (GEN3). The epigenome starts empty. An older file is refused.
    pub fn open_persistent(addr: &str, kernel: Kernel, genome_path: &str, _thoughts_path: &str) -> Self {
        let mut e = Entity::open(addr, kernel);
        if let Ok(bytes) = std::fs::read(genome_path) {
            if e.epigenome.uncrystallize(&bytes) {
                let n = e.epigenome.genome_len();
                let kb = std::fs::metadata(genome_path).map(|m| m.len() / 1024).unwrap_or(0);
                eprintln!("genome {n} words (GEN3, {kb} KB), epigenome empty");
            }
        } else {
            eprintln!("genome 0 words, epigenome empty");
        }
        e.thought_gen = e.epigenome.genome_len();
        e
    }

    /// Persist the genome atomically (write-then-rename). Call from the
    /// syntropic phase when the genome has grown.
    pub fn save_genome(&self, genome_path: &str) -> io::Result<()> {
        let tmp = format!("{genome_path}.tmp");
        let bytes = self.epigenome.crystallize();
        std::fs::write(&tmp, bytes)?;
        std::fs::rename(&tmp, genome_path)
    }

    /// Persist open thoughts atomically.
    pub fn save_thoughts(&self, thoughts_path: &str) -> io::Result<()> {
        let tmp = format!("{thoughts_path}.tmp");
        let mut bytes = Vec::new();
        self.contemplation.to_bytes(&mut bytes);
        std::fs::write(&tmp, bytes)?;
        std::fs::rename(&tmp, thoughts_path)
    }


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
        let bytes = rule.map(|e| e.to_bytes());
        let body = self.session.speak_keyed(
            id,
            dialogue::encode_solve_with_rule(&voice, id, outputs, bytes.as_deref()),
        )?;
        Ok(dialogue::decode_solve_receipt(&body).unwrap_or((dialogue::SOLVE_NOTHING, [0; 4])))
    }

    /// The perceived transport aperture (see [`TransportAperture`]) — the
    /// entity's answer to "why can I not speak to the server, and what can
    /// I do?": measured impedance, adapted pacing, and an inward turn under
    /// occlusion.
    pub fn transport(&self) -> TransportAperture {
        self.session.transport()
    }

    /// The measured transport impedance Z (consecutive failed exchanges).
    pub fn impedance(&self) -> u64 {
        self.session.impedance()
    }

    pub fn transport_rtt(&self) -> Option<u64> {
        self.session.rtt()
    }

    /// SEEK REFINEMENT — the compression probe (0x09, one word per stroke):
    /// advance the cursor to the next genome word and ask, in the entity's
    /// OWN language, whether the rest of the genome already speaks it
    /// (decompose its substance with itself excluded). A derivable word is a
    /// redundancy the vocabulary is carrying — a compression bid. The probe
    /// is a virtual rollout (zero mutation); the hypha's deterministic
    /// replay, not this discovery, decides retirement.
    pub fn seek_refinement(&mut self) -> Option<(crate::transducer::a21::Address, Vec<dialogue::CiteWire>)> {
        let g = self.epigenome.genome();
        if g.len() < 2 {
            return None;
        }
        self.refine_cursor = (self.refine_cursor + 1) % g.len();
        let i = self.refine_cursor;
        let sub = g[i].substance();
        if sub.sub().is_empty() {
            return None;
        }
        let located = self.epigenome.locate(sub.sub(), Some(i));
        let (cites, residual) = (located.cites, located.residual);
        let g = self.epigenome.genome();
        match (cites.len(), residual.as_slice()) {
            (1.., []) => Some((
                g[i].address(),
                cites
                    .into_iter()
                    .map(|(j, t, asg)| {
                        let body = crate::transducer::sentence::cited_pattern(&g[j], asg).unwrap_or_default();
                        (g[j].address(), Standing::Ruled.as_u8(), t, asg as u32, body)
                    })
                    .collect(),
            )),
            _ => None,
        }
    }

    /// BID the refinement on the wire: `(REFINE_ACCEPTED, μ⃗ compression
    /// yield)` if the hypha's replay confirms and the word retires;
    /// `(REFINE_NOTHING, 0⃗)` otherwise. On acceptance the entity drops its
    /// own copy too — the compression is global, and its next decomposition
    /// of the retired meaning re-derives the sentence form.
    pub fn bid_refinement(&mut self, word: &[u8], cites: &[dialogue::CiteWire]) -> io::Result<(u8, [u64; 4])> {
        let voice = self.session.kernel().voice();
        let body = self.session.speak_keyed(word, dialogue::encode_refine(&voice, word, cites))?;
        let (code, mu) =
            dialogue::decode_solve_receipt(&body).unwrap_or((dialogue::REFINE_NOTHING, [0; 4]));
        if code == dialogue::REFINE_ACCEPTED {
            self.epigenome.retire(word);
        }
        Ok((code, mu))
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
        self.session.absorb_digest(&body);
        let remote = dialogue::decode_digests_reply(&body).unwrap_or_default();
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
            .map(|d| (d.inv.clone(), sheaf::have(&atlas, &d.inv)))
            .collect();
        let request = dialogue::encode_fetch(&voice, &charts, crate::network::wire::frame_budget());
        let mut learned = 0usize;
        let body = self.session.speak_keyed(&voice, request)?;
        for w in dialogue::decode_syllabus_reply(&body).unwrap_or_default() {
            let mut pos = 0usize;
            let fetched = Element::from_bytes(&w, &mut pos);
            if self.absorb_taught(&w).is_some() {
                learned += 1;
            }
            // ČECH AFTER FETCH: the word is glued on the elements, not on
            // the root that indexed it. Every held word on the same fibre
            // with a group view is a chart overlapping this one; the
            // transition between them is the obstruction, and only a
            // transition with no axis and no chain glues. What does not
            // vanish is kept as the dialect measured, never discarded.
            let Some(fetched) = fetched else { continue };
            if let [] = fetched.axes().as_slice() {
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
            let Some((held, g)) = held_g else { continue };
            match (g.axes().as_slice(), g.sub()) {
                ([], []) => {
                    self.glued.insert(inv.key(), ());
                }
                _ => {
                    if !self.obstructions.contains(&fetched.address()) {
                        // THE OBSTRUCTION RIDES UP: the two views of one
                        // object are spoken to the hypha as a percept pair
                        // (held view → fetched view), a demand on the book
                        // for the word that carries one onto the other. The
                        // docket takes it to the court, where it lands on
                        // the Dialect arm if the federation cannot glue it.
                        let scene = |w: &Element| -> Vec<(i64, i64, u8)> { w.sub().iter().map(|&(r, c, _, h)| (r, c, h)).collect() };
                        let (a, b) = (scene(held), scene(&fetched));
                        let extent = |s: &[(i64, i64, u8)]| -> (usize, usize) {
                            let h = s.iter().map(|c| c.0 + 1).max().unwrap_or(0).max(1) as usize;
                            let w = s.iter().map(|c| c.1 + 1).max().unwrap_or(0).max(1) as usize;
                            (h, w)
                        };
                        let ((ha, wa), (hb, wb)) = (extent(&a), extent(&b));
                        let _ = self.session.submit(&a, &b, &(ha, wa, hb, wb), &[]);
                        self.obstructions.insert(fetched.address(), g);
                    }
                }
            }
        }
        self.absorb_class_page(&body, voice);
        // THE OTHER DIRECTION OF THE SHEAF: in a discordant stratum this
        // genome may hold what the hypha lacks (a basis that regressed, a
        // word learned from another judge). Those words are taught up for
        // adoption, one contact's budget per stroke; the hypha verifies
        // and installs what it cannot already derive.
        let mut taught = 0usize;
        let mut used = 0usize;
        'strata: for (_, held) in &charts {
            for addr in held {
                if self.already_taught_addr(addr) {
                    continue; // spoken for adoption once; the judge decided
                }
                let Some(word) = self.epigenome.word_at(addr).cloned() else { continue };
                if self.already_taught(&word) {
                    continue;
                }
                let bytes = word.to_bytes();
                if used + bytes.len() > crate::network::wire::frame_budget() {
                    break 'strata;
                }
                used += bytes.len();
                if self.session.speak_keyed(addr, dialogue::encode_adopt(&voice, &bytes)).is_ok() {
                    self.taught_up.insert(addr, word.clone());
                    taught += 1;
                }
            }
        }
        Ok(learned + taught)
    }

    fn absorb_class_page(&mut self, body: &[u8], from: [u8; 32]) {
        let pages = dialogue::decode_class_records(body);
        let mut recs = Vec::new();
        for p in pages {
            let mut pos = 0usize;
            let Some(r) = crate::transducer::alphabet::Rotor::from_bytes(&p.rotor, &mut pos) else {
                continue;
            };
            recs.push(crate::transducer::bubble::ClassRecord {
                key: p.key,
                order: crate::transducer::alphabet::Order::from_u32(p.order),
                representative: r,
            });
        }
        let n = self.contemplation.seed_horizon(&recs);
        if n > 0 {
            eprintln!(
                "  horizon {n} seeded from {:02x}{:02x}",
                from[0], from[1]
            );
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
            let start = self.session.peer_cursor() % n;
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
                for w in dialogue::decode_syllabus_reply(&body).unwrap_or_default() {
                    if self.absorb_taught(&w).is_some() {
                        learned += 1;
                    }
                }
                self.absorb_class_page(&body, voice);
            }
            return Ok(learned);
        }
        let body = self.session.speak_keyed(&voice, dialogue::encode_syllabus_request(&voice))?;
        let mut words = dialogue::decode_syllabus_reply(&body).unwrap_or_default();
        words.sort_by_key(|w| {
            let mut pos = 0usize;
            crate::transducer::element::Element::from_bytes(w, &mut pos)
                .and_then(|e| crate::transducer::sentence::from_element_form(&e))
                .map(|f| f.depth)
                .unwrap_or(0)
        });
        let mut learned = 0usize;
        for w in &words {
            if self.absorb_taught(w).is_some() {
                learned += 1;
            }
        }
        self.absorb_class_page(&body, voice);
        Ok(learned)
    }




    pub fn kernel(&self) -> &Kernel {
        self.session.kernel()
    }
    /// Submit a percept; COMPREHEND anything taught back (the C-stack, not a
    /// byte-store). Returns the reply and the comprehension of the teaching.
    /// CITE: this kernel's reduction of a chain over its own genome — the
    /// citations (address, transport, gauge coordinate) and the residual it
    /// could not reduce. Cognition happens here; the hypha only replays.
    fn idea_word(&self, addr: &[u8]) -> Option<Element> {
        self.ideas.get(addr).cloned()
    }

    pub fn cite(&mut self, chain: &[(i64, i64, u8, u8)]) -> (Vec<dialogue::CiteWire>, Vec<(i64, i64, u8, u8)>) {
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
            .filter_map(|(i, g, asg)| {
                let w = word_of(i)?;
                let body = crate::transducer::sentence::cited_pattern(w, asg).unwrap_or_default();
                Some((w.address(), standing_of(i).as_u8(), g, asg as u32, body))
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
            t if t.is_empty() => None,
            t => self.absorb_taught(t),
        };
        Ok((reply, heard))
    }

    /// Visit every known hypha by a keyed cursor under the stroke Budget.
    /// Used while the genome is empty so the first syllabus/quote is a fan-out,
    /// not a seed pin.
    fn quote_all_hyphae(&mut self, asked: &[crate::transducer::stalk::Invariant]) -> io::Result<YieldQuote> {
        let peers = self.session.peers();
        if peers.is_empty() {
            return self.session.quote(asked);
        }
        let mut budget = self.physics.stroke();
        let n = peers.len();
        let start = self.session.peer_cursor() % n;
        self.session.bump_cursor();
        let mut last_err = io::Error::new(io::ErrorKind::NotConnected, "no hypha");
        let mut out: Option<YieldQuote> = None;
        let asked_st: Vec<(crate::transducer::stalk::Invariant, u8)> = asked
            .iter()
            .map(|inv| {
                let st = self
                    .working_atlas()
                    .fiber(inv)
                    .first()
                    .map(|(_, s)| s.as_u8())
                    .unwrap_or(Standing::Held.as_u8());
                (inv.clone(), st)
            })
            .collect();
        let req = dialogue::encode_quote_request_standing(&self.session.kernel().voice(), &asked_st);
        for i in 0..n {
            if !budget.spend() {
                break;
            }
            let addr = &peers[(start + i) % n];
            match self.session.exchange_at(addr, req.clone()) {
                Ok(body) => {
                    let (height, surface, k, items) = dialogue::decode_quote_reply(&body)
                        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "quote decode"))?;
                    let clusters: Vec<Cluster> = items
                        .into_iter()
                        .map(|(address, mu, chain, open)| Cluster { address, mu, chain, open })
                        .collect();
                    match &mut out {
                        None => out = Some(YieldQuote { height, surface, k, clusters }),
                        Some(q) => {
                            let mut have: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
                            for h in &q.clusters {
                                have.insert(&h.address, ());
                            }
                            for c in clusters {
                                if have.contains(&c.address) {
                                    continue;
                                }
                                have.insert(&c.address, ());
                                q.clusters.push(c);
                            }
                        }
                    }
                }
                Err(e) => last_err = e,
            }
        }
        match out {
            Some(q) => Ok(q),
            None if last_err.kind() == io::ErrorKind::WouldBlock => Ok(YieldQuote {
                height: 0,
                surface: [0; 4],
                k: 0,
                clusters: Vec::new(),
            }),
            None => Err(last_err),
        }
    }

    pub fn quote(&mut self) -> io::Result<YieldQuote> {
        let thought_charts: Vec<crate::transducer::stalk::Invariant> = {
            let mut charts = Vec::new();
            // index: held bubbles; each cover is a peel of its invariant
            for (_, b) in self.contemplation.bubbles.fiber(crate::network::nerve::ROOT) {
                let inv = crate::transducer::stalk::invariant(&b.remainder);
                let mut cur = inv;
                loop {
                    if b.cover.contains(&cur.key()) {
                        charts.push(cur.clone());
                    }
                    if cur.0.is_empty() {
                        break;
                    }
                    cur = cur.peel();
                }
            }
            charts
        };
        let charts = thought_charts;
        let mut dests: crate::network::nerve::Keyed<Vec<crate::transducer::stalk::Invariant>> =
            crate::network::nerve::Keyed::new();
        let mut dest_order: Vec<String> = Vec::new();
        let bubbles: Vec<(Vec<(i64, i64, u8, u8)>, [u64; 4])> = self
            .contemplation
            .bubbles
            .fiber(crate::network::nerve::ROOT)
            .map(|(_, b)| (b.remainder.clone(), b.volume))
            .collect();
        for (remainder, mag) in &bubbles {
            let inv = crate::transducer::stalk::invariant(remainder);
            let dest = self.session.route_chart(&inv, mag);
            let dk = dest.as_bytes();
            match dests.get_mut(dk) {
                Some(held) => {
                    if !held.iter().any(|h| h == &inv) {
                        held.push(inv);
                    }
                }
                None => {
                    dest_order.push(dest.clone());
                    dests.insert(dk, vec![inv]);
                }
            }
        }
        if dests.is_empty() || self.atlas_is_empty() {
            // An empty atlas is incident to the root prefix: the first
            // quote reads the book at depth zero, so a mind need not
            // already hold a word to learn one.
            let root = [crate::transducer::stalk::Invariant(Vec::new())];
            let asked = if charts.is_empty() || self.atlas_is_empty() {
                root.as_slice()
            } else {
                charts.as_slice()
            };
            return self.quote_all_hyphae(asked);
        }
        let mut last_err = io::Error::new(io::ErrorKind::NotConnected, "no hypha");
        let mut out: Option<YieldQuote> = None;
        for dest in dest_order {
            let invs = dests.get(dest.as_bytes()).cloned().unwrap_or_default();
            let asked = if invs.is_empty() { charts.clone() } else { invs };
            let asked_st: Vec<(crate::transducer::stalk::Invariant, u8)> = asked
                .iter()
                .map(|inv| {
                    let st = self
                        .working_atlas()
                        .fiber(inv)
                        .first()
                        .map(|(_, s)| s.as_u8())
                        .unwrap_or(Standing::Held.as_u8());
                    (inv.clone(), st)
                })
                .collect();
            let req = dialogue::encode_quote_request_standing(&self.session.kernel().voice(), &asked_st);
            match self.session.exchange_at(&dest, req.clone()).or_else(|e| {
                if e.kind() == io::ErrorKind::WouldBlock {
                    self.session.speak_nerve(req)
                } else {
                    Err(e)
                }
            }) {
                Ok(body) => {
                    let (height, surface, k, items) = dialogue::decode_quote_reply(&body)
                        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "quote decode"))?;
                    let clusters: Vec<Cluster> = items
                        .into_iter()
                        .map(|(address, mu, chain, open)| Cluster { address, mu, chain, open })
                        .collect();
                    match &mut out {
                        None => {
                            out = Some(YieldQuote { height, surface, k, clusters });
                        }
                        Some(q) => {
                            let mut have: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
                            for h in &q.clusters {
                                have.insert(&h.address, ());
                            }
                            for c in clusters {
                                if have.contains(&c.address) {
                                    continue;
                                }
                                have.insert(&c.address, ());
                                q.clusters.push(c);
                            }
                        }
                    }
                }
                Err(e) => last_err = e,
            }
        }
        match out {
            Some(q) => Ok(q),
            None if last_err.kind() == io::ErrorKind::WouldBlock => Ok(YieldQuote {
                height: 0,
                surface: [0; 4],
                k: 0,
                clusters: Vec::new(),
            }),
            None => Err(last_err),
        }
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
    ) -> (Vec<dialogue::CiteWire>, Vec<(i64, i64, u8, u8)>, bool) {
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
    ) -> (Vec<dialogue::CiteWire>, Vec<(i64, i64, u8, u8)>, bool) {
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
    /// The discordance measured at the last sync: strata whose roots differ
    /// from the hypha's. Zero is a glued genome.
    /// The Čech obstructions measured after fetch: fetched words whose
    /// transition to a held word on the same fibre did not vanish.
    pub fn obstructions(&self) -> Vec<(Vec<u8>, Element)> {
        self.obstructions
            .fiber(crate::network::nerve::ROOT)
            .map(|(a, g)| (a, g.clone()))
            .collect()
    }

    /// HOLD A THOUGHT: a remainder this mind has not collapsed, as a bubble
    /// whose cover is a peel of its invariant. Connections are the link of
    /// the new bubble through by_chart.
    pub fn hold_thought(&mut self, remainder: Vec<(i64, i64, u8, u8)>) {
        self.contemplation.hold(remainder.clone());
        let e = crate::transducer::element::from_residual(&remainder);
        self.ideas.insert(e.address(), e);
    }

    /// Hold a withheld form so the pivot loop sees it next stroke (ch. 30.2).
    pub fn hold_form(
        &mut self,
        f: &crate::transducer::sentence::Form,
        train: &[(Vec<Vec<u8>>, Vec<Vec<u8>>)],
        remainders: &[Vec<(i64, i64, u8, u8)>],
    ) {
        self.contemplation.hold_form(f, train, remainders);
        if let Some(e) = crate::transducer::sentence::as_element_form(f, [0; 4]) {
            self.ideas.insert(e.address(), e);
        }
        for r in remainders {
            if r.is_empty() {
                continue;
            }
            let e = crate::transducer::element::from_residual(r);
            self.ideas.insert(e.address(), e);
        }
    }

    /// Stand a withheld form, hold it as an idea, conclude each remainder,
    /// and utter each residual word once with its cites (24.3).
    pub fn withhold_form(
        &mut self,
        f: &crate::transducer::sentence::Form,
        train: &[(Vec<Vec<u8>>, Vec<Vec<u8>>)],
        remainders: &[Vec<(i64, i64, u8, u8)>],
        mag: [u64; 4],
    ) {
        let k = self.session.voice().k();
        let tu = self.now_tu();
        if let Some(word) = crate::transducer::sentence::as_element_form(f, mag) {
            self.epigenome.stand_dialect_withheld(word, k, tu);
        }
        self.hold_form(f, train, remainders);
        for (i, r) in remainders.iter().enumerate() {
            if r.is_empty() {
                continue;
            }
            let (cites, residual, _) = self.conclude(r);
            if residual.is_empty() {
                continue;
            }
            let word = crate::transducer::element::from_residual(&residual);
            let addr = word.address();
            eprintln!(
                "  remainder expressed {:02x}{:02x}… pair {i} cells {} cites {}",
                addr[0],
                addr[1],
                residual.len(),
                cites.len()
            );
            self.taught_up.insert(&addr, word.clone());
            let mu = crate::transducer::element::measure_lanes(&residual);
            eprintln!(
                "  uttered {:02x}{:02x}… standing Stood K {k} lanes {mu:?}",
                addr[0], addr[1]
            );
            let _ = self.attest(&residual, &cites);
        }
    }

    /// Addresses this mind has uttered (receipt: address at this K).
    pub fn uttered_n(&self) -> usize {
        self.taught_up.len()
    }

    pub fn uttered(&self, addr: &[u8]) -> bool {
        self.taught_up.contains(addr)
    }

    /// Hold a withheld sentence as a bubble: form, remainder, cover.
    pub fn hold_sentence(&mut self, word: &Element) {
        if let Some(f) = crate::transducer::sentence::from_element_form(word) {
            self.hold_form(&f, &[], &[]);
        } else {
            self.hold_thought(word.sub().to_vec());
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

    /// OPEN AN AXIS ON MULTIPLICITY — several held words deflated one
    /// listing; the glue between them is the new coordinate. Never on
    /// failure (a listing no held word deflates is a thought, not an axis).
    pub fn open_axis(
        &mut self,
        listing: &[u8],
        chain: &[(i64, i64, u8, u8)],
        words: &[Element],
    ) -> Option<crate::transducer::a21::Address> {
        use crate::transducer::element::measure_lanes;
        if let Some(a) = self.open_contemplation_axis() {
            return Some(a);
        }
        if words.len() < 2 || self.opened.contains(listing) {
            return None;
        }
        let [base, other, ..] = words else { return None };
        // The glue between the two words that deflated this listing.
        let glue = crate::transducer::cech::transition(base, other)?;
        let v = glue.versor()?;
        if !base.is_connection() {
            return None; // the axis stands on a spoken motion, never on a bare residual
        }
        let mut germ = base.clone();
        germ.open_axis(v);
        let mut mag = measure_lanes(chain);
        mag[3] = mag[3].saturating_add(words.len() as u64);
        germ.set_magnitude(mag);
        self.opened.insert(listing, ());
        match self.speak_word(&germ) {
            Ok(Utterance::Spoken) => Some(germ.address()),
            _ => None,
        }
    }

    /// Seed from a class at any degree on the contemplation — the coordinate
    /// the current axes could not express. A class is an ideology; the gauge
    /// that would translate one frame into the other is a translation word.
    fn open_contemplation_axis(&mut self) -> Option<crate::transducer::a21::Address> {
        let classes = self.contemplation.classes_held();
        for (class, _degree) in classes {
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
        self.opened.len()
    }

    /// Expand dirty ideas once per syntropic phase. Returns the lane
    /// measure of A16 snaps (Discharged, Vortex, TorsionFiled).
    pub fn expand_ideas(&mut self) -> [u64; 4] {
        let mut budget = self.physics.stroke();
        self.contemplation.set_now(self.session.voice().k());
        let rotors = crate::network::nerve::Keyed::new();
        let (_spent, pivoted) = self.contemplation.expand_all(
            self.epigenome.atlas(),
            &rotors,
            &self.epigenome,
            &mut budget,
        );
        self.contemplation.retire_on_t(self.physics.reserve[2] as usize);
        pivoted
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
        let added: Vec<Element> = self.epigenome.genome()[self.thought_gen..].to_vec();
        self.thought_gen = n;
        let mut spoken = 0usize;
        for w in &added {
            spoken += self.pivot_on(w);
        }
        spoken
    }

    /// Hear a taught word by pivoting the contemplation before banking it.
    /// k is the voice's K (law 9), tu is TUniversal; they are not one clock.
    pub fn absorb_taught(&mut self, taught: &[u8]) -> Option<Comprehension> {
        let mut pos = 0usize;
        while pos < taught.len() {
            let before = pos;
            let Some(e) = Element::from_bytes(taught, &mut pos) else {
                break;
            };
            if pos == before {
                break;
            }
            let _ = self.pivot_on(&e);
        }
        self.epigenome
            .learn_at(taught, self.session.voice().k(), self.now_tu())
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

    fn already_taught_addr(&self, addr: &[u8]) -> bool {
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
        if word.is_identity() || !(word.is_connection() || !word.opened_axes().is_empty()) {
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

    pub fn discordance(&self) -> usize {
        if self.discordance_measured {
            self.discordance
        } else {
            self.contemplation.boundary_norm()
        }
    }

    pub fn shed(&mut self) -> usize {
        self.shed_at(self.session.voice().k())
    }

    pub fn shed_at(&mut self, now_k: u64) -> usize {
        self.epigenome.note_stroke(now_k);
        let glued = |a: &[u8]| self.contemplation.glues_word(a);
        let n = self.epigenome.shed_except(now_k, &glued);
        n.saturating_add(self.contemplation.shed_unglued())
    }

    pub fn dialect_len(&self) -> usize {
        self.epigenome.dialect_len()
    }

    /// The mind's working atlas: genome ∪ dialect ∪ ideas, each fiber
    /// member carrying its standing.
    pub fn working_atlas(&self) -> WorkingAtlas<'_> {
        self.epigenome.working_atlas_with(&self.contemplation.by_chart)
    }

    /// Empty when genome, dialect and ideas are all empty.
    pub fn atlas_is_empty(&self) -> bool {
        self.epigenome.atlas_is_empty() && self.contemplation.len() == 0
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
