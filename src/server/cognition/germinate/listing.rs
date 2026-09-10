// 23.8, 26, 50.5 — WHAT LISTING A SENTENCE WRITES: the room it draws, the
// book it enters, its demand vertices by kind, and the docket it waits on.
//
//! The one path from a heard sentence to the book (law 8, ruled
//! 2026-09-10). `perceive_sentence` replays the tape by G and hands what
//! stood to `list_sentence`; nothing here reads a cell of the voice's
//! scene, and the commit of a reduced germ that did is deleted with the
//! percept act.

use crate::cognition::*;
use crate::transducer::element::Element;

impl Kernel {
    /// 50.5, 34.2 (L7): the demand vertices of a listed sentence, written
    /// by kind and chart as the court lists them — the market relays and
    /// prices them under the sentence; under L8 they are listed beside its
    /// closed sub-complex.
    pub(crate) fn list_demands(&mut self, sentence: &Element, voice: &[u8; 32]) {
        let Some(f) = sentence.form() else {
            return;
        };
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
    /// loops' orders, and the install waits for the reserve (E3).
    pub(crate) fn list_sentence(&mut self, word: &Element, voice: [u8; 32]) -> Outcome {
        let addr = word.address();
        let mag = word.magnitude();
        if let Some(slot) = self.live_slot(&addr) {
            return self.settle_use_of(voice, slot, &mag);
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
            let short = self.room_after_extending(&voice, &mag);
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
    fn settle_use_of(&mut self, voice: [u8; 32], slot: usize, mag: &[u64; 4]) -> Outcome {
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
        if let Some(w) = self.language.words.get(slot) {
            let held = w.magnitude();
            if let Some(disc) = self.discoverer_of(&w.address()) {
                self.treasury.settle_use(voice, disc, held, anchor);
            }
        }
        self.physics.syntropy(mag);
        Outcome::Decomposed(vec![slot])
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
        let k_now = self.k_step();
        self.book_list(word.clone(), k_now, voice);
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
        self.charge_loops(word, &mag);
        self.convergence.used_by(addr.as_bytes(), voice);
        if !self.pending.contains(&addr) {
            let k_now = self.k_step();
            self.book_list(word.clone(), k_now, voice);
            self.list_demands(word, &voice);
            self.docket_connection(word, voice);
            self.offered.insert(&addr, ());
        }
        Outcome::GermPending
    }

    /// THE TORSION SOURCE (12.6, 18.2): the charge is read at the level the
    /// carrier is on. A stored cycle is the conjugate level already — an
    /// Open walk pivots the lanes the sentence occupies and bumps them for
    /// the length it walked; a Closed loop is proven monodromy and charges
    /// nothing. With no stored cycle a sentence's words are its loop
    /// holonomies, each closing in the order of its class: a finite order
    /// bumps that many times, per loop and never summed; a loop that never
    /// closes is an open walk and is charged as one.
    fn charge_loops(&mut self, word: &Element, mag: &[u64; 4]) {
        let witnesses = crate::transducer::torsion_witness::witness_thought(word);
        if witnesses.is_empty() {
            let Some(f) = word.form() else {
                return;
            };
            for (w, _) in f.loop_words() {
                match w.order() {
                    crate::transducer::alphabet::Order::Finite(p) => {
                        for _ in 0..p {
                            self.physics.bump(mag);
                        }
                    }
                    crate::transducer::alphabet::Order::Infinite => {
                        self.physics.pivot(crate::physics::Physics::lanes_of(mag));
                        for _ in 0..w.len() as u64 {
                            self.physics.bump(mag);
                        }
                    }
                }
            }
            return;
        }
        for w in &witnesses {
            if let crate::transducer::torsion_witness::HolonomicLoopWitness::Open { walked } = w {
                self.physics.pivot(crate::physics::Physics::lanes_of(mag));
                for _ in 0..*walked as u64 {
                    self.physics.bump(mag);
                }
            }
        }
    }
}
