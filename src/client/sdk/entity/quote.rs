// 23.8 — THE QUOTE: what the market offers this entity, read across every
// hypha it knows, and the refinement it bids for.

use crate::client::sdk::*;

impl Entity {
    /// SEEK REFINEMENT — the compression probe (0x09, one word per stroke):
    /// advance the cursor to the next genome word and ask, in the entity's
    /// OWN language, whether the rest of the genome already speaks it
    /// (decompose its substance with itself excluded). A derivable word is a
    /// redundancy the vocabulary is carrying — a compression bid. The probe
    /// is a virtual rollout (zero mutation); the hypha's deterministic
    /// replay, not this discovery, decides retirement.
    pub fn seek_refinement(&mut self) -> Option<(crate::transducer::address::Address, Vec<dialogue::CiteWire>)> {
        let g = self.epigenome.genome();
        if g.len() < 2 {
            return None;
        }
        self.refine_cursor = (self.refine_cursor + 1) % g.len();
        let i = self.refine_cursor;
        let sub = g.get(i)?.substance();
        if sub.sub().is_empty() {
            // A SENTENCE HAS NO CHAIN AND IS NOT THEREFORE IRREDUCIBLE.
            // `as_element_form` writes a description and a magnitude and
            // never a chain, so `sub()` is empty for EVERY sentence and this
            // arm has always refused all of them: the spatial deconstruction
            // below can only ever examine chain-carrying residual words.
            //
            // A word is an OPERATOR, not a tile. Two words can induce one
            // element of `Spin(4,0)` with disjoint supports, and one word can
            // be a change of basepoint away from another. Neither is visible
            // to a test that asks whether cells pave cells. `algebraic_redundancy`
            // asks the two algebraic questions instead — is this operator
            // already held, and is it a section of a bundle already held —
            // and both are decided exactly rather than searched.
            return None;
        }
        // `locate` deconstructs over the WORKING atlas (U2): a cite's index
        // runs over the genome, then the dialect, then the ideas. A
        // refinement bid cites ruled words only (rule 11: a cite the genome
        // does not hold is not a compression the mesh can replay), so a
        // deconstruction that leans on a stood or held word is no bid; it
        // was read as a genome index and voice 4 died on it (I35).
        let located = self.epigenome.locate(sub.sub(), Some(i));
        let (cites, residual) = (located.cites, located.residual);
        let g = self.epigenome.genome();
        let ruled = cites.iter().all(|c| c.word < g.len());
        match (cites.len(), residual.as_slice(), ruled) {
            (1.., [], true) => {
                let word = g.get(i)?.address();
                let wires: Option<Vec<dialogue::CiteWire>> = cites
                    .into_iter()
                    .map(|c| {
                        let cited = g.get(c.word)?;
                        // L6: a cite IS the body it reconstructs. One whose
                        // gauge reconstructs none is no cite, and a bid
                        // that cannot spell one of its cites is no bid.
                        let body =
                            crate::transducer::sentence::cited_pattern(cited, c.gauge)?;
                        Some(dialogue::CiteWire {
                            addr: cited.address(),
                            standing: Standing::Ruled.as_u8(),
                            transport: c.transport,
                            gauge: c.gauge as u32,
                            body,
                        })
                    })
                    .collect();
                Some((word, wires?))
            }
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
        // A receipt that does not decode is not a receipt saying the bid
        // earned nothing: REFINE_NOTHING is an answer the hypha gives, and
        // this mind did not get one.
        let Some((code, mu)) = dialogue::decode_solve_receipt(&body) else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "refine receipt does not decode",
            ));
        };
        if code == dialogue::REFINE_ACCEPTED {
            self.epigenome.retire(word);
        }
        Ok((code, mu))
    }

    /// Visit every known hypha by a keyed cursor under the stroke Budget.
    /// Used while the genome is empty so the first syllabus/quote is a fan-out,
    /// not a seed pin.
    pub(crate) fn quote_all_hyphae(&mut self, asked: &[crate::transducer::stalk::Invariant]) -> io::Result<YieldQuote> {
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
                (inv.clone(), self.working_atlas().standing_on(inv).as_u8())
            })
            .collect();
        // An ask whose charts do not fit the count the frame writes is not
        // asked short (47.1).
        let Some(req) = dialogue::encode_quote_request_standing(&self.session.kernel().voice(), &asked_st)
        else {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "the ask does not fit the lengths its frame writes",
            ));
        };
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
                        .map(|i| Cluster { address: i.addr, mu: i.mu, element: i.element, open: i.open })
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
            // THE BUBBLES THIS MIND HOLDS, all of them: the charts it asks
            // a quote on are the charts it is thinking about, and one left
            // out is a thought the book is never asked about.
            for (_, b) in self.contemplation.bubbles.scoped() {
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
        let bubbles: Vec<Bubbled> = self
            .contemplation
            .bubbles
            .scoped()
            .map(|(_, b)| (b.remainder.to_vec(), b.volume))
            .collect();
        for (remainder, mag) in &bubbles {
            let inv = crate::transducer::stalk::invariant(remainder);
            // A chart no hypha is known for is not routed to the empty
            // address; it is not routed. The whole ask goes to every hypha
            // below when nothing routes.
            let Some(dest) = self.session.route_chart(&inv, mag) else {
                continue;
            };
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
            // A destination the routing named no charts for is asked the
            // whole ask below; the empty list is that reading.
            let invs = match dests.get(dest.as_bytes()) {
                Some(i) => i.clone(),
                None => Vec::new(),
            };
            let asked = if invs.is_empty() { charts.clone() } else { invs };
            let asked_st: Vec<(crate::transducer::stalk::Invariant, u8)> = asked
                .iter()
                .map(|inv| {
                    (inv.clone(), self.working_atlas().standing_on(inv).as_u8())
                })
                .collect();
            let Some(req) =
                dialogue::encode_quote_request_standing(&self.session.kernel().voice(), &asked_st)
            else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "the ask does not fit the lengths its frame writes",
                ));
            };
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
                        .map(|i| Cluster { address: i.addr, mu: i.mu, element: i.element, open: i.open })
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

    /// EXPANSIONS 1 AND 2 — WHY A WORD IS REDUNDANT, ALGEBRAICALLY.
    ///
    /// `seek_refinement` asks one question: do the rest of the vocabulary's
    /// CELLS pave this word's cells. That is extensional — it treats a word
    /// as a tile — and it misses two ways a word can carry nothing new:
    ///
    /// 1. SAME OPERATOR (Expansion 1). Two tapes can induce one element of
    ///    `Spin(4,0)` with entirely disjoint supports, because a word is an
    ///    operator and its support is only where it was last observed
    ///    acting. The word problem here is DECIDED — the rotor
    ///    representation is faithful over ℚ — so this is one geometric
    ///    product and one projective comparison, not a search.
    ///
    /// 2. SAME BUNDLE (Expansion 2). A word conjugate to a held word is a
    ///    SECTION of it: one base, two basepoints. `γ` is solved from the
    ///    linear relation `γa = λbγ`, exactly, and the language keeps the
    ///    base and the transition instead of two monoliths.
    ///
    /// Both answers carry their WITNESS — the held word, and for a section
    /// the `γ` that relates them — because "redundant" on its own is an
    /// assertion and with the witness it is a re-derivation anyone replays
    /// (16.12 step 4).
    ///
    /// The vocabulary read is the RULED words: a compression bid the court
    /// must replay cannot lean on a word the mesh does not hold (rule 11),
    /// which is the same condition the spatial bid already enforces.
    pub fn algebraic_redundancy(&self) -> Option<Redundancy> {
        let g = self.epigenome.genome();
        if g.len() < 2 {
            return None;
        }
        let i = self.refine_cursor % g.len();
        let held = g.get(i)?;
        let w = crate::transducer::sentence::words_of_element(held.element());
        let mine = w.into_iter().next()?;
        if mine.is_identity() {
            return None;
        }
        // Every other ruled word's carried letters, as candidates. The
        // genome is the carrier being compressed, so walking it is the
        // object's own boundary and not a lookup that had a coordinate.
        let mut others: Vec<(crate::transducer::address::Address, crate::transducer::alphabet::Word)> =
            Vec::new();
        for (j, h) in g.iter().enumerate() {
            if j == i {
                continue;
            }
            for word in crate::transducer::sentence::words_of_element(h.element()) {
                if !word.is_identity() {
                    others.push((h.address(), word));
                }
            }
        }
        for (addr, other) in &others {
            if mine.operator_eq(other) {
                return Some(Redundancy::SameOperator { held: addr.clone() });
            }
        }
        let words: Vec<crate::transducer::alphabet::Word> =
            others.iter().map(|(_, w)| w.clone()).collect();
        if let Some((base, gamma)) = crate::transducer::alphabet::bundle::section_of(&words, &mine) {
            let held = others
                .iter()
                .find(|(_, w)| {
                    crate::transducer::alphabet::rewrite::tape_key(w)
                        == crate::transducer::alphabet::rewrite::tape_key(&base)
                })
                .map(|(a, _)| a.clone())?;
            return Some(Redundancy::Section { held, gamma: Box::new(gamma) });
        }
        None
    }
}

/// WHY A WORD CARRIES NOTHING NEW, with the witness that shows it.
#[derive(Clone, Debug)]
pub enum Redundancy {
    /// The language already holds a word inducing this operator. The tapes
    /// may differ entirely; the element does not.
    SameOperator {
        held: crate::transducer::address::Address,
    },
    /// This word is a section of a held word's bundle: one base, and `gamma`
    /// is the change of basepoint between them.
    ///
    /// Boxed because a `Word` carries its memoised rotor — sixteen
    /// rationals — and a variant that does not hold one should not pay that
    /// footprint for the variant that does.
    Section {
        held: crate::transducer::address::Address,
        gamma: Box<crate::transducer::alphabet::Word>,
    },
}
