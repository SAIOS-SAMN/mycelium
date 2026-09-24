// 23.8, 26, 50.5 — WHAT LISTING A SENTENCE WRITES: the room it draws, the
// book it enters, its demand vertices by kind, and the docket it waits on.
//
//! The one path from a heard sentence to the book (law 8, ruled
//! 2026-09-10). `perceive_sentence` replays the tape by G and hands what
//! stood to `list_sentence`; nothing here reads a painted site of the
//! voice's scene, and the commit of a reduced germ that did is deleted with
//! the percept act.

use crate::cognition::*;
use crate::transducer::element::Element;
use crate::transducer::sentence::ToralPeriod;

/// 37.7 P2 — THE LETTER AS A FORM. `π_p = a + b·i` becomes the rotor
/// `a + b·e_ij` on the plane the demand named: a rotor acts by a
/// two-sided sandwich, so it is the HALF-ANGLE of the rotation it
/// induces, and the arc of its block is `π_p²` of norm `p²`. That is
/// exactly the action a versor carrying this prime stands on, and it is
/// why the factor of norm `p` is the right thing to adjoin.
///
/// The rotor is written UNNORMALISED — `a` and `b` as they came out of
/// Euclid — because identity is projective (1.5) and normalising by
/// `√p` would leave ℚ.
fn letter_form(
    pi: &crate::core::gaussian::Gauss,
    plane: (u8, u8),
    chart: &crate::transducer::stalk::Invariant,
    bodies: &[crate::transducer::sentence::Link],
) -> Option<Element> {
    use crate::transducer::alphabet::Rotor;
    let mask = (1usize << plane.0) | (1usize << plane.1);
    let q = |n: &num_bigint::BigInt| {
        num_rational::BigRational::from_integer(n.clone())
    };
    let rotor = Rotor::scalar(q(&pi.a)).add(&Rotor::blade(mask, q(&pi.b)));
    crate::transducer::sentence::generator_form(&rotor, chart, bodies)
}

impl Kernel {
    /// 50.5, 34.2 (L7): the demand vertices of a listed sentence, written
    /// by kind and chart as the court lists them — the market relays and
    /// prices them under the sentence; under L8 they are listed beside its
    /// closed sub-complex.
    pub(crate) fn list_demands(&mut self, sentence: &Element, voice: &[u8; 32]) {
        // A LISTED sentence: not on the basis, so the decode is named. It
        // is taken ONCE here and the second read below is gone with it —
        // the borrow that forced two reads ended with `Element::form`
        // (L16).
        let Some(f) = crate::transducer::sentence::from_element_form(sentence) else {
            return;
        };
        // 37.7 P2–P3 — THE COURT OPENS A LETTER. A `RotorPrime` demand
        // carries the residue block the shells left; the factor is read
        // off it by Euclid over ℤ[i] and docketed as a provisional
        // generator, on the same path a sentence takes. The witness was
        // already checked at replay (`REFUSE_LETTER_WITNESS`), so what
        // arrives here carries its prime.
        //
        // The kind byte routes this line and nothing else: the docket
        // path is the sentence's, and there is no per-kind closer.
        // A `Standing` demand carries the rotor the peel could not spell as
        // its own witness (16.16 step 6): the court dockets that rotor as
        // the candidate generator, by the same docket the court's own
        // unspellable simplex takes (37.7, the rotor level).
        let mut opened: Vec<(crate::transducer::sentence::Demand, crate::transducer::stalk::Invariant, Vec<crate::transducer::sentence::Link>)> = Vec::new();
        let mut stood: Vec<(crate::transducer::alphabet::Rotor, crate::transducer::stalk::Invariant, Vec<crate::transducer::sentence::Link>)> = Vec::new();
        for (_, kind, chart, bodies) in f.demand_vertices() {
            match kind {
                crate::transducer::sentence::Demand::RotorPrime { .. } => {
                    opened.push((kind.clone(), chart.clone(), bodies.to_vec()));
                }
                crate::transducer::sentence::Demand::Standing { rotor, .. } => {
                    stood.push(((**rotor).clone(), chart.clone(), bodies.to_vec()));
                }
                _ => {}
            }
        }
        for (kind, chart, bodies) in opened {
            self.open_letter(&kind, &chart, &bodies, voice);
        }
        for (rotor, chart, bodies) in stood {
            self.open_unspellable(&rotor, &chart, &bodies, voice);
        }
        for (_, kind, chart, _) in f.demand_vertices() {
            let chart_key = chart.key();
            let prefix: String = chart_key.as_bytes().iter().take(2).map(|b| format!("{b:02x}")).collect();
            eprintln!(
                "  court demand listed {} chart {prefix}… from {:02x}{:02x}…",
                kind.label(),
                voice[0],
                voice[1]
            );
            self.court_demands_listed = self.court_demands_listed.saturating_add(1);
        }
    }

    /// 37.7 P2 — THE PROVISIONAL GENERATOR A DEMAND OPENS. The factor is
    /// a form like any other: a Frame and one word vertex that cites
    /// nothing, spells nothing, and carries the factor as its canonical
    /// rotor — which L14 made representable with no escape byte, so a
    /// letter is named by content (law 6) and no act byte is added.
    ///
    /// Its address is `form_vertex_key`, so two courts deriving one
    /// factor reach ONE address and arity 2 converges on it (P3). Its μ⃗
    /// is the demand's bodies measured lane-wise — what the language
    /// could not say without it, the same measure a listing's remainder
    /// takes. It closes nothing on any family by itself, so its install
    /// mints none and says so; what its speakers earn is room when it is
    /// used (26 as ruled).
    /// 37.7 AT THE ROTOR LEVEL: a rotor the seventeen cannot spell, met as
    /// the court's own unspellable simplex or carried by a voice's `Standing`
    /// demand as its witness (16.16 step 6). It is the same adjunction
    /// `open_letter` performs for a demanded prime, so it takes the same
    /// docket: re-derived, escrowed, provisional, discovered.
    ///
    /// The DISCOVERER is the voice whose speech carried or produced the
    /// rotor. A court is never a discoverer (47.3).
    pub(crate) fn open_unspellable(
        &mut self,
        rotor: &crate::transducer::alphabet::Rotor,
        chart: &crate::transducer::stalk::Invariant,
        bodies: &[crate::transducer::sentence::Link],
        voice: &[u8; 32],
    ) {
        let Some(e) = crate::transducer::sentence::generator_form(rotor, chart, bodies) else {
            return;
        };
        let addr = e.address();
        self.docket_letter(e, voice);
        eprintln!(
            "  court letter opened {:02x}{:02x}… rotor from {:02x}{:02x}…",
            addr[0], addr[1], voice[0], voice[1]
        );
    }

    fn open_letter(
        &mut self,
        kind: &crate::transducer::sentence::Demand,
        chart: &crate::transducer::stalk::Invariant,
        bodies: &[crate::transducer::sentence::Link],
        voice: &[u8; 32],
    ) {
        use crate::transducer::sentence::Demand;
        let Demand::RotorPrime { plane, prime, witness } = kind else {
            return;
        };
        let Some(arc) = crate::core::gaussian::from_block(witness) else {
            return;
        };
        let Some(pi) = crate::core::gaussian::prime_factor(&arc, &num_bigint::BigInt::from(*prime))
        else {
            return;
        };
        // A demand whose bodies measure nothing opens nothing: there was
        // no cost to what the language could not say, so there is no
        // letter's worth to escrow (rule 11 — a zero is not a floor).
        let Some(e) = letter_form(&pi, *plane, chart, bodies) else {
            return;
        };
        let addr = e.address();
        self.docket_letter(e, voice);
        eprintln!(
            "  court letter opened {:02x}{:02x}… prime {prime} plane ({}, {}) from {:02x}{:02x}…",
            addr[0], addr[1], plane.0, plane.1, voice[0], voice[1]
        );
    }

    /// THE DOCKET A MINTED LETTER TAKES, written once because both mints take
    /// it: the demanded prime of `open_letter` and the court's own
    /// `Fill::Unspellable`. Two copies is two chances for one of them to
    /// escrow, stamp or discover differently from the other, and a letter
    /// admitted by one route and not the other is a generator half the mesh
    /// holds.
    fn docket_letter(&mut self, e: Element, voice: &[u8; 32]) {
        let addr = e.address();
        // The DEMANDING VOICE is the discoverer, through the same docket
        // path a sentence takes; a second voice whose demand yields this
        // factor is a party on the same address, as `list_sentence`
        // already records one. A court is never a discoverer.
        if self.provisional.contains(&addr) || self.ruled_contains(addr.as_bytes()) {
            self.converge_here(&e, *voice);
            return;
        }
        // The docket shape a sentence takes, written out: this court's own
        // re-derivation recorded so a witness can converge with it (47.3),
        // the escrow to the demanding voice as a party, the discoverer,
        // and the provisional itself. `docket_connection` is not used
        // because it refuses an element whose action is the identity, and
        // a form's action is carried by its tape and not by its chain.
        let addr_owned = addr.clone();
        let mu = crate::economy::price(&e.magnitude(), &self.physics.pressure);
        let anchor = self.anchor();
        self.converge_here(&e, *voice);
        self.treasury.escrow_award(&addr_owned, vec![*voice], mu, anchor);
        self.deflations.insert(&addr_owned, e.magnitude());
        let k_now = self.k_step();
        self.provisional_k.insert(&addr_owned, k_now);
        self.provisional.insert(&addr_owned, e.clone());
        self.note_discovered(&addr_owned, *voice);
        self.stamp_standing(&addr_owned);
        self.provisional_h.insert(&addr_owned, self.height());
        self.mark_persist();
    }

    /// LIST A SENTENCE THE REPLAY STOOD (23.8, 26). A sentence the language
    /// holds is a USE of it: the reader's fee and the discoverer's settle,
    /// nothing listed. One already on the docket is a second voice on the
    /// same address — a party to the settlement, never a second listing
    /// (§3.1 as ruled, S13.15). A connection or a rewrite is docketed as it
    /// was. A new sentence draws the voice's room on the lanes it occupies
    /// and is refused with `REFUSE_ROOM` where a lane does not cover it —
    /// nothing docketed, drawn, escrowed or sealed; admitted, it is listed
    /// on the open book with its demands, escrowed to its parties, docketed
    /// and Offered. On a depleted lane it is listed and charged by its
    /// orbits' orders, and the install waits for the reserve (E3).
    pub(crate) fn list_sentence(&mut self, word: &Element, voice: [u8; 32]) -> Outcome {
        let addr = word.address();
        let mag = word.magnitude();
        if self.is_live(&addr) {
            return self.settle_use_of(voice, &addr, &mag);
        }
        if self.provisional.contains(&addr) {
            // A second voice on the address is a party to the settlement
            // (24.1): the escrow names every party, so the release pays it.
            self.convergence.used_by(addr.as_bytes(), voice);
            self.record_spoken(addr.clone(), voice);
            let mu = crate::economy::price(&mag, &self.physics.pressure);
            let anchor = self.anchor();
            self.treasury.escrow_award(&addr, self.convergence.parties(&addr), mu, anchor);
            return Outcome::GermPending;
        }
        if !word.is_sentence() {
            self.docket_connection(word, voice);
            return Outcome::GermPending;
        }
        // ADMISSION IS ROOM (23.8, S9.6): T-room is swept before the draw
        // and the book extends before it refuses (12.6, S9.7). A use of an
        // address already listed draws nothing new.
        if !self.pending.contains(&addr) {
            // A room that cannot be read refuses the draw: the listing is
            // not admitted against a count that is broken.
            let Some(short) = self.room_after_extending(&voice, &mag) else {
                return Outcome::Refused(crate::verdict::REFUSE_ROOM);
            };
            if !crate::economy::lanes_short(&short).is_empty() {
                self.log_room_refused(&voice, &mag, &short);
                return Outcome::Refused(crate::verdict::REFUSE_ROOM);
            }
        }
        if crate::economy::lanes_short(&self.physics.reserve_shortfall(word.lanes())).is_empty() {
            self.list_and_docket(word, voice)
        } else {
            self.list_charged(word, voice)
        }
    }

    /// A SENTENCE THE LANGUAGE HOLDS, spoken again (24.1): the reader's fee
    /// per lane (§5) with the lanes it could not cover named and recorded —
    /// they close the voice's room on a lane while its claim in flight there
    /// stands (§3.1 as ruled) — the discoverer paid the structure exercised,
    /// and order built.
    fn settle_use_of(
        &mut self,
        voice: [u8; 32],
        at: &crate::transducer::address::Address,
        mag: &[u64; 4],
    ) -> Outcome {
        let anchor = self.anchor();
        let pressure = self.physics.pressure;
        let (_paid, short) = self.treasury.settle_recall(voice, mag, &pressure, anchor);
        self.record_recall_short(voice, &short);
        for l in crate::economy::lanes_short(&short) {
            eprintln!(
                "  court recall short voice {:02x}{:02x}… lane {l} by {}",
                voice[0], voice[1], short[l]
            );
        }
        if let Some(w) = self.language.word_at(at.as_bytes()) {
            let held = w.magnitude();
            if let Some(disc) = self.discoverer_of(&w.address()) {
                self.treasury.settle_use(voice, disc, held, anchor);
            }
        }
        self.physics.syntropy(mag);
        Outcome::Decomposed(vec![at.clone()])
    }

    /// ONE VANTAGE DOCKETS (47.3): this court replayed the sentence to get
    /// here, which is its own re-derivation recorded; the second is a
    /// witness hypha's, and it arrives through the Offer. The sentence is
    /// docketed AS A LISTING — one thing, not two: quotable on the open book
    /// while it awaits its witness, its demands listed under it, its escrow
    /// held for its parties, its deflation what the listing measures, and
    /// the install sealed as a step of this court's clock.
    fn list_and_docket(&mut self, word: &Element, voice: [u8; 32]) -> Outcome {
        let addr = word.address();
        let mag = word.magnitude();
        self.converge_here(word, voice);
        self.book_list(word.clone(), voice);
        self.list_demands(word, &voice);
        let _ = self.treasury.settle_install(word, voice); // POW++ gate only
        let mu = crate::economy::price(&mag, &self.physics.pressure);
        let anchor = self.anchor();
        self.treasury.escrow_award(&addr, self.convergence.parties(&addr), mu, anchor);
        self.record_deflation(addr.clone(), addr.clone(), mag);
        let k_now = self.k_step();
        self.provisional_k.insert(&addr, k_now);
        self.provisional.insert(&addr, word.clone());
        self.provisional_h.insert(&addr, self.height());
        self.stamp_standing(&addr);
        self.physics.relieve(&mag);
        self.physics.syntropy(&mag); // a completed rule is order built
        self.physics.spend(word.lanes()); // the expansion SHRINKS the reserve
        self.append_step(crate::chronometry::ActKind::Install, &addr, voice);
        if self.field.get(&addr).is_none() {
            let inv = crate::transducer::stalk::invariant_of(word);
            self.write_field(
                &addr,
                inv,
                crate::transducer::stalk::Closure::Cavity,
                word.sub().to_vec(),
                &mag,
                &mag,
            );
        }
        self.offered.insert(&addr, ());
        Outcome::GermPending
    }

    /// THE OPEN-DEMAND ARM (24.1, 26): a sentence the lane cannot expand on
    /// is still listed, charged for itself, and docketed; the metabolism
    /// defers the install and nothing strands it.
    fn list_charged(&mut self, word: &Element, voice: [u8; 32]) -> Outcome {
        let addr = word.address();
        let mag = word.magnitude();
        self.charge_orbits(word, &mag);
        self.convergence.used_by(addr.as_bytes(), voice);
        if !self.pending.contains(&addr) {
            self.book_list(word.clone(), voice);
            self.list_demands(word, &voice);
            self.docket_connection(word, voice);
            self.offered.insert(&addr, ());
        }
        Outcome::GermPending
    }

    /// THE TORSION SOURCE (12.6, 18.2): the charge is read at the level the
    /// carrier is on. A stored cycle is the conjugate level already — an
    /// Open orbit pivots the lanes the sentence occupies and bumps them for
    /// the length it traversed; a Closed orbit is proven monodromy and charges
    /// nothing. With no stored cycle a sentence's words are its orbit
    /// holonomies, each closing in the order of its class: a finite order
    /// bumps that many times, per orbit and never summed; an orbit that never
    /// closes is an open orbit and is charged as one.
    ///
    /// AND AN ORBIT THE SENTENCE FILED IS CHARGED AS ONE TOO, BUT ONLY WHERE
    /// THE SPEAKER DEMANDED ADJUDICATION OVER IT (ruled 2026-09-14,
    /// inverting D10 of the same day).
    ///
    /// "What will not close" has two readings and D10 fixed the weaker one:
    /// this asked whether the ALGEBRA closes the cycle, never whether the
    /// speaker did, so `Order::Infinite` was the only thing that charged and
    /// this corpus produces none. A filed cycle is one no candidate
    /// discharged, which is `Physics::pivot`'s own "demand for a new fiber".
    /// That much stands.
    ///
    /// What D10 got wrong is that it then charged UNCONDITIONALLY. A bare
    /// unclosed cycle is algebraically a NON-GENERATOR — `∂c ≠ 0` — and a
    /// voice submitting one is exploring the topology, not claiming a
    /// resource. Charging metabolism for that taxes the search itself: the
    /// voices file cycles precisely when they are looking for a word they do
    /// not have, and the engine would have billed every attempt.
    ///
    /// So an unclosed cycle is a cheap algebraic refusal at the gate — it is
    /// not admitted to the basis and it is not a taxable metabolic event.
    /// The charge is assessed only where the speaker requested a full
    /// adjudication stroke over that boundary, which on this tape is a
    /// DEMAND VERTEX (50.5): the sentence saying out loud what it could not
    /// close and asking the market to answer it. Claiming the stroke is what
    /// costs; failing to close is not.
    fn charge_orbits(&mut self, word: &Element, mag: &[u64; 4]) {
        // THE GATE: did this sentence ask for adjudication over what it
        // could not close? A form with no demand vertex claims nothing and
        // its cycles fall through free.
        let demanded = crate::transducer::sentence::from_element_form(word)
            .is_some_and(|f| f.demand_vertices().next().is_some());
        let witnesses = crate::transducer::torsion_witness::witness_thought(word);
        if witnesses.is_empty() {
            let Some(f) = crate::transducer::sentence::from_element_form(word) else {
                return;
            };
            // THE PERIODS THE COMPLEX DERIVES, by class — one reader
            // (`orbit_classes`), the same one `file_orbit` writes through.
            let mut derived: crate::network::nerve::Keyed<
                crate::transducer::sentence::ToralPeriod,
                crate::network::nerve::coord::Holonomy,
            > =
                crate::network::nerve::Keyed::new();
            for lp in &f.orbits {
                for row in f.orbit_classes(lp) {
                    derived.put(&row.class, row.periods);
                }
            }
            for (w, _) in f.orbit_words() {
                // A CYCLE THIS SENTENCE FILED is one the speaker could not
                // discharge: `file_orbit` writes `Form.torsion` only where no
                // candidate discharged the orbit, so it stands as a class
                // awaiting a word — which is exactly what `Physics::pivot`
                // calls "the demand for a new fiber". It is keyed by the
                // holonomy's class, the same key this reader has in hand.
                //
                // ITS ORDER SAYS WHETHER THE ALGEBRA CLOSES IT; THE FILING
                // SAYS WHETHER THIS MIND DID, and the metabolism is about
                // the second. Charging torsion on `Order::Infinite` alone
                // asked only the first, and this corpus has never produced
                // one: measured at e9399c4, 12187 flat and 256 finite edge
                // holonomies, INFINITE ZERO, against 200 cycles the voices
                // filed. So `Physics.torsion` could not be charged by any
                // sentence this engine has ever spoken, `order_flux` stayed
                // `syntropic − 0`, and the `𝒯` lane of every yield surface
                // was zero — which is what 406 of 406 dwell readings showed
                // after 25.9 stopped the voice from zeroing it itself.
                //
                // A cycle the tree gauge closed or a pivot discharged is not
                // filed and charges nothing here; it closed.
                // THE FILING IS THE SPEAKER'S AND IS READ ON THE SPEAKER'S
                // WORD (54.5, as ruled 2026-09-20). This asked
                // `f.torsion` — the DECODED form's — and a form recovered
                // from its address files nothing, so `filed` was false for
                // every sentence and the charge stopped being assessed at
                // all.
                //
                // The carrier moved; the reading did not. This line is
                // about LABOUR — "the filing says whether this mind did",
                // two paragraphs up — so the attestation is exactly what it
                // should ask, and `Element::filing` is where an attestation
                // lives (df1eeaa). It is the one reader in this session's
                // audit that wanted the receipt rather than the geometry.
                let filed = w
                    .rotor()
                    .holonomy_key()
                    .is_some_and(|k| word.filing().holds(&k));
                // AND WHETHER THE ALGEBRA CLOSES IT IS THE FILER'S READER,
                // NOT THE ROTOR'S. `w.order()` is the bare rotor order with
                // no carriage and no fixed-point test: a screw has a finite
                // rotor order and closes at nothing, so this read `Finite`
                // and called the circuit closed. `orbit_classes`
                // gates on the fixed point and gives it `0`.
                // THE CYCLE CLOSED, WHICH IS BOTH PLANES. It read
                // `ell > 0` on the `lcm`, true whenever either factor is
                // a root of unity, so an orbit with one plane turning
                // forever read as closed and was never charged as
                // torsion (54.7).
                let closes = w
                    .rotor()
                    .holonomy_key()
                    .is_some_and(|k| derived.at(&k).is_some_and(ToralPeriod::closes));
                // The charge is owed for a cycle that did not close AND that
                // the sentence demanded adjudication over. Without the
                // demand it is a free algebraic refusal: `∂c ≠ 0` is not a
                // generator and exploring for one is not a taxable event.
                if demanded && (filed || !closes) {
                    self.physics.pivot(crate::physics::Physics::lanes_of(mag));
                }
                let steps = match w.order() {
                    crate::transducer::alphabet::Order::Finite(p) => u64::from(p),
                    crate::transducer::alphabet::Order::Infinite => w.len() as u64,
                };
                for _ in 0..steps {
                    self.physics.bump(mag);
                }
            }
            return;
        }
        // A STORED CYCLE IS ONE NO CANDIDATE DISCHARGED — `file_orbit` runs
        // only under `Discharge::NoneDischarged` — so by construction every
        // one of them is a cycle this mind has no word for. D10 read that as
        // grounds to charge whatever the witness said, on the argument that
        // "a Closed orbit is proven monodromy and charges nothing" asks about
        // the OBJECT instead of the MIND. The reading is right and the
        // conclusion was not: having no word for a cycle is the ordinary
        // condition of a voice searching, and billing it taxes the search.
        //
        // The charge follows the DEMAND, not the failure. A sentence that
        // asked the market to adjudicate the boundary claimed an adjudication
        // stroke and pays for it; one that merely carries a cycle it could
        // not close is refused at the gate for free.
        //
        // The witness still refines how much PRESSURE rides with it — the
        // span traversed when Open, the period closed at when Closed — and
        // pressure is the backlog, not the tax.
        for w in &witnesses {
            if demanded {
                self.physics.pivot(crate::physics::Physics::lanes_of(mag));
            }
            let steps = match w {
                crate::transducer::torsion_witness::HolonomicOrbitWitness::Open { run } => {
                    u64::from(*run)
                }
                crate::transducer::torsion_witness::HolonomicOrbitWitness::Closed {
                    period, ..
                } => u64::from(*period),
            };
            for _ in 0..steps {
                self.physics.bump(mag);
            }
        }
    }
}
