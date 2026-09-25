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
    /// ADVANCE THE REFINEMENT CIRCUIT BY ONE COORDINATE, wrapping at the end
    /// of the trie — the carrier's own circuit (45.7), not `% len`. A genome
    /// of fewer than two words has no redundancy to seek.
    fn advance_refine_cursor(&mut self) -> Option<Vec<u8>> {
        if self.epigenome.genome_len() < 2 {
            return None;
        }
        let at = self.epigenome.next_word_after(self.refine_cursor.as_deref())?;
        self.refine_cursor = Some(at.clone());
        Some(at)
    }

    pub fn seek_refinement(&mut self) -> Option<(crate::transducer::address::Address, Vec<dialogue::CiteWire>)> {
        let at = self.advance_refine_cursor()?;
        let sub = self.epigenome.word_at(&at)?.substance();
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
            // to a test that asks whether links pave links. `algebraic_redundancy`
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
        let located = self.epigenome.locate(sub.sub(), Some(&at));
        let (cites, residual) = (located.cites, located.residual);
        // RULED IS "THE GENOME HOLDS THIS ADDRESS", asked at the coordinate.
        // It was `c.word < g.len()` — an offset compared against a length,
        // which is true of any index into the dialect's range as soon as
        // the genome grows past it. That is the read voice 4 died on (I35).
        let ruled = cites.iter().all(|c| self.epigenome.word_at(c.word.as_bytes()).is_some());
        match (cites.len(), residual.as_slice(), ruled) {
            (1.., [], true) => {
                let word = self.epigenome.word_at(&at)?.address();
                let epi = &self.epigenome;
                let wires: Option<Vec<dialogue::CiteWire>> = cites
                    .into_iter()
                    .map(|c| {
                        let cited = epi.word_at(c.word.as_bytes())?;
                        // L6: a cite IS the body it reconstructs. One whose
                        // gauge reconstructs none is no cite, and a bid
                        // that cannot spell one of its cites is no bid.
                        let body =
                            crate::transducer::sentence::cited_pattern(cited, &c.assignment)?;
                        Some(dialogue::CiteWire {
                            addr: cited.address(),
                            standing: Standing::Ruled.as_u8(),
                            transport: c.transport,
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
        // LD0: a bid whose address cannot be framed is not spoken.
        let Some(frame) = dialogue::encode_refine(&voice, word, cites) else {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "refine bid refused: an address cannot be framed"));
        };
        let body = self.session.speak_keyed(word, frame)?;
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
        let Some(start) = self.session.peer_start(n) else { return self.session.quote(asked) };
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
    /// LINKS pave this word's links. That is extensional — it treats a word
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
    /// THE CANDIDATES ARE READ AT THE CONJUGACY COORDINATE, never read whole.
    ///
    /// The first cut of this swept the whole genome and compared the
    /// cursor's word against every other — a Cartesian pass over the carrier
    /// being compressed, which is rule 1 and law 3, and the same shape R27
    /// removed from both syntropy probes. It was written here anyway.
    ///
    /// The coordinate was already in hand and is the same one the question
    /// is about. A section of a bundle is a CONJUGATE, and `versor_chart` is
    /// `[0, class_axes…]` — the rotor's conjugacy axes — which is exactly
    /// what `GeneratorIndex` is keyed by (1.8, 50.2 step 2). So the words
    /// that could possibly answer either question are the fibre under that
    /// chart, reached by one keyed prefix, and `generators_under` coarsens
    /// as A21 §2 requires rather than answering empty at a finer stratum.
    ///
    /// Both questions are then asked of that fibre and of nothing else:
    /// operator equality (Expansion 1, decided by the rotor), then a
    /// solvable change of basepoint (Expansion 2). A word outside the fibre
    /// is in another conjugacy class and can answer neither.
    pub fn algebraic_redundancy(&mut self) -> Option<Redundancy> {
        let at = self.refine_cursor.clone()?;
        let held = self.epigenome.word_at(&at)?;
        let me = held.address();
        let mine = crate::transducer::sentence::acting_of_element(held.element())
            .into_iter()
            .map(|t| t.word)
            .find(|w| !w.is_identity())?;
        let chart = crate::transducer::sentence::versor_chart(&mine)?;
        let mut budget = self.physics.stroke();
        let fibre = crate::transducer::family::Crystal::generators_under(
            &self.epigenome,
            &chart,
            &mut budget,
        );
        let mut siblings: Vec<(crate::transducer::address::Address, crate::transducer::alphabet::Word)> =
            Vec::new();
        for (form, _vertex, word) in fibre {
            if form.as_bytes() == me.as_bytes() || word.is_identity() {
                continue;
            }
            siblings.push((form, word));
        }
        for (addr, other) in &siblings {
            if mine.operator_eq(other) {
                return Some(Redundancy::SameOperator { held: addr.clone() });
            }
        }
        for (addr, other) in &siblings {
            match crate::transducer::alphabet::bundle::conjugation(other, &mine) {
                Some(crate::transducer::alphabet::bundle::Conjugation::Spelled(gamma)) => {
                    return Some(Redundancy::Section {
                        held: addr.clone(),
                        gamma: Box::new(gamma),
                    });
                }
                // 37.7 — the gauge exists and the seventeen cannot say it.
                // That is a candidate generator and not a silence.
                Some(crate::transducer::alphabet::bundle::Conjugation::Unspellable(g)) => {
                    return Some(Redundancy::UnspellableGauge {
                        held: addr.clone(),
                        gamma: Box::new(g),
                        chart,
                    });
                }
                None => {}
            }
        }
        None
    }
}

impl Entity {
    /// EXPANSIONS 1 AND 2 AT THE CURSOR — ONE READING, TWO MOVES. The
    /// algebraic redundancy of the cursor's word is read once; what it
    /// says decides the move:
    ///
    /// - 37.7 — a gauge the language measured and cannot spell is ADJOINED.
    ///   A letter this mind minted stands in its own dialect, `Stood` and
    ///   not `Ruled`, as every other adjunction does, and needs no court to
    ///   begin.
    /// - a word that induces a held word's operator, or is a section of its
    ///   bundle by a gauge the seventeen spell, is a BID. The spatial bid's
    ///   replay expects a chain of links, which an operator has none of, so
    ///   the court rules this one on its own algebra (`try_refine_algebraic`):
    ///   the bid goes to the court with its witness and the word retires
    ///   where the court re-derives it.
    ///
    /// One word per stroke, at the cursor `algebraic_redundancy` already
    /// advances — no second circuit, no second cursor, and no second solve.
    pub fn algebraic_step(&mut self) -> Option<AlgebraicStep> {
        use crate::anastome::dialogue::AlgebraicWitness;
        let word = {
            let at = self.refine_cursor.clone()?;
            self.epigenome.word_at(&at)?.address()
        };
        match self.algebraic_redundancy()? {
            // 37.7: a gauge the alphabet cannot spell against a held word,
            // adjoined.
            Redundancy::UnspellableGauge { gamma, chart, .. } => {
                let adjoined: crate::transducer::bubble::drive::Adjoined = vec![(*gamma, chart)];
                self.adjoin_letters(&adjoined);
                Some(AlgebraicStep::Adjoined)
            }
            Redundancy::SameOperator { held } => Some(AlgebraicStep::Bid(Box::new(AlgebraicBid {
                word,
                held,
                witness: AlgebraicWitness::SameOperator,
            }))),
            Redundancy::Section { held, gamma } => Some(AlgebraicStep::Bid(Box::new(AlgebraicBid {
                word,
                held,
                witness: AlgebraicWitness::Section(gamma),
            }))),
        }
    }

    /// BID THE ALGEBRAIC REFINEMENT on the wire: `(REFINE_ACCEPTED, μ⃗)` if
    /// the court's algebra confirms the witness and the word retires;
    /// `(REFINE_NOTHING, 0⃗)` otherwise. On acceptance this mind drops its
    /// own copy too, as the spatial bid does.
    pub fn bid_algebraic_refinement(
        &mut self,
        word: &[u8],
        held: &[u8],
        witness: &crate::anastome::dialogue::AlgebraicWitness,
    ) -> io::Result<(u8, [u64; 4])> {
        let voice = self.session.kernel().voice();
        let Some(frame) = dialogue::encode_refine_algebraic(&voice, word, held, witness) else {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "refine bid refused: an address cannot be framed"));
        };
        let body = self.session.speak_keyed(word, frame)?;
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
    /// 37.7 — A GAUGE THE LANGUAGE CANNOT SAY. The engine proved this word
    /// and a held one are related by a `γ ∈ Spin(4,0)`, and proved the
    /// seventeen cannot spell it. That is not a redundancy to bid and not a
    /// silence: it is the measured obstruction a generator is made of, and
    /// it is carried out to be adjoined with the chart it was met on.
    ///
    /// Boxed for the reason the `Section`'s word is: a `Rotor` is sixteen
    /// rationals and a variant that holds none should not carry the
    /// footprint of one that does.
    UnspellableGauge {
        held: crate::transducer::address::Address,
        gamma: Box<crate::transducer::alphabet::Rotor>,
        chart: crate::transducer::stalk::Invariant,
    },
}
