// 21, 26 — INGEST: a percept arrives, is reduced against the language, and
// what it could not reduce is committed as a germ.
//
//! `commit_germ` is where a reduction becomes a listing, a section, or a
//! docket. Nothing here searches: the reduction is replayed and what
//! remains is what the language does not yet say.

use crate::cognition::*;
use crate::transducer::address::Address;
use crate::transducer::element::{self, Element};

impl Kernel {
    pub fn ingest(&mut self, a: &[(i64, i64, u8)], b: &[(i64, i64, u8)], voice: [u8; 32]) -> Outcome {
        let frame = frame_of(a, b);
        self.adjudicate(&[a.to_vec()], &[b.to_vec()], voice, &frame)
    }

    /// THE JUDGE. Given a client's PERCEPT, extract the pair form on this
    /// court as Crystal, and answer valid (Recognised/Installed) or invalid
    /// (Pivot). No sensing here — the client perceived; the judge reasons.
    pub fn adjudicate(
        &mut self,
        ca: &[Vec<(i64, i64, u8)>],
        cb: &[Vec<(i64, i64, u8)>],
        voice: [u8; 32],
        frame: &crate::arc::Frame,
    ) -> Outcome {
        // The whole path under one lock (tests and local use); the server
        // runs the same three phases with the lock dropped around the
        // reduction (synthesis::adjudicate).
        let Some((language, budget)) = self.reflex_open(voice) else {
            return Outcome::GermPending;
        };
        let Some(germs) = percept_elements(ca, cb, frame, self) else {
            return self.pivot();
        };
        let reductions = reduce_all(&language, budget, germs);
        self.commit_all(reductions, voice)
    }

    /// THE REFLEX OPENING — under a brief lock: the voice enrols on first
    /// speech, the metabolism dissipates its backlog, the cheap syntropic
    /// moves fire in both resting states of the two-stroke (depleted or
    /// idle; the redundancy probe stays with the idle worker), and the
    /// escrow-priced door prices congestion. Returns the language snapshot
    /// and the projection budget the off-lock reduction runs with; None when
    /// the door defers (the honest "come back", which clients retry).
    pub fn reflex_open(
        &mut self,
        voice: [u8; 32],
    ) -> Option<(std::sync::Arc<Language>, (usize, usize))> {
        self.physics.relax();
        self.ensure_bonded(voice);
        let idle = self.physics.metabolism_rate().iter().all(|&p| p <= 1);
        if self.physics.depleted() || idle {
            self.condense_stroke();
            self.physics.regenerate();
        }
        // The door's reading is per lane (S9.6b): a lane spent to nothing
        // that the speaker paid nothing on defers the act, and the reading
        // names the lanes it was deferred on.
        let deferred = self.door_deferral(voice);
        if !crate::economy::lanes_short(&deferred).is_empty() {
            return None;
        }
        let budget = (0usize, 0usize); // reduction is closed-form: no search budget
        Some((self.language(), budget))
    }

    /// A degenerate percept: nothing to reduce.
    pub fn pivot(&mut self) -> Outcome {
        self.physics.pivot([false; 4]);
        Outcome::Pivot
    }

    /// COMMIT every reduction under the lock: one reduction is the demand's
    /// outcome; several are a fragmented demand recomposed per body.
    pub fn commit_all(&mut self, mut reductions: Vec<Reduction>, voice: [u8; 32]) -> Outcome {
        match reductions.len() {
            1 => self.commit_germ(reductions.swap_remove(0), voice),
            _ => Outcome::Fragmented(
                reductions.into_iter().map(|r| self.commit_germ(r, voice)).collect(),
            ),
        }
    }

    /// COMMIT a reduction: the state changes a normal form implies, under the
    /// lock. Cited words resolve from the snapshot's indices to addresses and
    /// then to the live basis; a citation that left the basis while the
    /// reduction ran defers the demand (it re-reduces on the client's retry).
    /// Opaque: matched by canonical address, never by "kind".
    pub fn commit_germ(&mut self, r: Reduction, voice: [u8; 32]) -> Outcome {
        let Reduction { germ: g, form, qs: _, language, listing, frontier } = r;
        let lanes = g.lanes();
        let mag = g.magnitude();
        let normal_form = match form {
            NormalForm::Decomposed(cited) => {
                let mut word = Vec::with_capacity(cited.len());
                for wi in cited {
                    let live = language
                        .words
                        .get(wi)
                        .map(|w| w.address())
                        .and_then(|a| self.live_slot(&a));
                    match live {
                        Some(i) => word.push(i),
                        None => {
                            self.physics.bump(&mag);
                            return Outcome::GermPending; // the language moved under it
                        }
                    }
                }
                NormalForm::Decomposed(word)
            }
            NormalForm::Irreducible(res) => {
                // A germ is born ⊤ — ONE evidence underdetermines its
                // selection (the same law as anchor consensus: the family
                // decides, never a single pair). Its birth footprint+scene are
                // RECORDED as evidence; the court's KB refinement (7c) derives
                // and binds π over the accumulated family.
                let fp: Vec<(i64, i64)> = res.sub().iter().map(|(r, c, _, _)| (*r, *c)).collect();
                if !fp.is_empty() {
                    let scene: Vec<(i64, i64, u8)> =
                        res.sub().iter().map(|(r, c, b, _)| (*r, *c, *b)).collect();
                    let _ = self.deposit_evidence(res.address(), fp, scene);
                }
                NormalForm::Irreducible(res)
            }
        };
        match normal_form {
            NormalForm::Decomposed(word) => {
                let anchor = self.anchor();
                self.treasury.settle_recall(voice, lanes, anchor); // reader's fee → the pool recharges
                for &i in &word {
                    // PoUW circulation: the demander pays each cited word's
                    // discoverer the structure exercised, from earned balance.
                    let Some(w) = self.language.words.get(i) else { continue };
                    let mag = w.magnitude();
                    if let Some(disc) = self.discoverer_of(&w.address()) {
                        self.treasury.settle_use(voice, disc, mag, anchor);
                    }
                }
                self.physics.syntropy(&mag); // a spanned sentence builds order
                Outcome::Decomposed(word)
            }
            NormalForm::Irreducible(residual) => {
                // PUNCTURE-BEFORE-MINT: if this residual sits on an EXISTING
                // thought's base positions with a new transition assignment, it is
                // a SECTION prospect — the germinated axis extends (quorum-gated),
                // and no sibling card is minted. Growth as +axis, not +card.
                // (A residual matching a held section never reaches here — the
                // reduction's direct terminal speaks the family.)
                let centered = residual.sub().to_vec();
                // The stratum over the residual's chart, read by key: a held
                // word on the same base positions is a section prospect.
                let inv = crate::transducer::stalk::invariant(&centered);
                let fk = chart_footprint_key(&inv, &centered);
                if let Some(taddr) = self
                    .by_footprint
                    .link(&fk)
                    .next()
                    .map(|(a, _)| crate::transducer::address::Address::from(a))
                {
                    let Some(held) = self.basis.get(&taddr).map(|w| (**w).clone()) else {
                        self.physics.bump(&mag);
                        return Outcome::GermPending;
                    };
                    debug_assert_eq!(element::positions(held.sub()), element::positions(&centered));
                    // The slot names this word in the language snapshot or
                    // the snapshot lags the carrier; 0 is another word.
                    let Some(ti) = self.live_slot(&taddr) else {
                        self.physics.bump(&mag);
                        return Outcome::GermPending;
                    };
                    let asg = element::assignment(&centered);
                    let saddr = Address::from(section_address(&held.address(), &asg));
                    // This court RE-DERIVED the section from its own held
                    // word; the voice that uttered it is a use and a
                    // settlement party (24.1), never a vantage. The second
                    // vantage is a witness hypha that re-derives the same
                    // address (47.3), and it arrives through the Offer.
                    let mut section = held.clone();
                    section.puncture(asg.clone());
                    self.converge_here_at(&saddr, &section, voice);
                    self.record_spoken(saddr.clone(), voice);
                    if self.physics.can_expand(lanes) {
                        if let Some(w) = self.word_mut(&taddr) {
                            w.puncture(asg);
                        }
                        self.touch_at(&taddr);
                        self.physics.relieve(&mag);
                        self.physics.syntropy(&mag);
                        self.unlist(&residual.address(), false);
                        self.append_step(&saddr, voice);
                        return Outcome::Sectioned(ti);
                    }
                    self.physics.bump(&mag);
                    return Outcome::GermPending;
                }
                // KB COMPLETION. Confluence IS the condition: two organs
                // reducing one demand reach the same residual address
                // because the algebra is confluent, and a differing address
                // is inconsistent data, not a vote to be broken. The old
                // comment said exactly this and then counted mouths.
                let addr = residual.address();
                // ONE VANTAGE DOCKETS. This court re-derived the residual to
                // get here, which is its own re-derivation recorded; the
                // second is a witness hypha's, and it arrives through the
                // Offer. What still gates is the metabolism, which is the
                // only thing that should (E3): a germ with a depleted lane
                // DEFERS and dockets on a later submission once the reserve
                // recovers, so deferral never strands one.
                if self.physics.can_expand(lanes) {
                    // ADMISSION IS ROOM (23.8): the draw comes from the
                    // voice's own room on each lane it occupies, and a lane
                    // that does not cover it REFUSES THE ACT — nothing is
                    // docketed, drawn, escrowed, offered or sealed. The
                    // shortfall is read per lane and the refusal names the
                    // lanes; before S9.6 the act was logged as refused and
                    // then escrowed, marked provisional and offered anyway.
                    // T-room is swept BEFORE the draw, exactly as the other
                    // listing site does it: standing extent that has aged out
                    // relocates to the cavity so the lane it held is free.
                    // Drawing first would refuse an act the sweep would have
                    // covered, which reads as a cap and is really a stale row.
                    let draw = residual.magnitude();
                    let k_now = self.k_step();
                    self.relocate_t_exhausted();
                    let short = self.room_shortfall(&voice, &draw);
                    if !crate::economy::lanes_short(&short).is_empty() {
                        self.log_room_refused(&voice, &draw, &short);
                        return Outcome::Refused(crate::verdict::REFUSE_ROOM);
                    }
                    self.converge_here(&residual, voice);
                    // 23.8: the residual is "docketed AS A LISTING". One
                    // thing, not two -- it stays quotable on the open book
                    // while it awaits a witness, so other voices can reduce
                    // it and be credited for the use. Only the install
                    // clears the page.
                    self.book_list((*residual).clone(), k_now, voice);
                    let _ = self.treasury.settle_install(&residual, voice); // POW++ gate only
                    let mu = crate::economy::price(&residual.magnitude(), &self.physics.pressure);
                    let anchor = self.anchor();
                    // PROVISIONAL (Step 5): consilience escrows pending the
                    // court's admission docket; the word joins the basis as
                    // local dialect, final settlement waits for the verdict.
                    self.treasury.escrow_award(&addr, self.convergence.parties(&addr), mu, anchor);
                    self.record_deflation(addr.clone(), listing, residual.magnitude());
                    let k_now = self.k_step();
                    self.provisional_k.insert(&addr, k_now);
                    self.provisional.insert(&addr, (*residual).clone());
                    self.provisional_h.insert(&addr, self.height());
                    self.stamp_standing(&addr);
                    self.physics.relieve(&mag);
                    self.physics.syntropy(&mag); // a completed rule is order built
                    self.physics.spend(lanes); // the expansion SHRINKS the reserve
                    // Seal the K-step receipt: the install IS a tick of algebraic
                    // time, parent-linked to the mesh's whole history.
                    self.append_step(&addr, voice);
                    // The listing is settled. The basis grows only from gluing,
                    // never from a single hypha's two voices. The field stays
                    // so the court stroke can read it; the page clears.
                    let inv = crate::transducer::stalk::invariant_of(&residual);
                    if self.field.get(&addr).is_none() {
                        self.write_field(
                            &addr,
                            inv,
                            crate::transducer::stalk::Closure::Cavity,
                            residual.sub().to_vec(),
                            &residual.magnitude(),
                            &residual.magnitude(),
                        );
                    }
                    self.docket_connection(&residual, voice);
                    self.offered.insert(&addr, ());
                    Outcome::GermPending
                } else {
                    // ADMISSION IS ROOM here too (23.8): a demand the lane
                    // cannot expand on is still listed, and the listing draws
                    // the voice's room. A short lane refuses the act before
                    // anything is charged or written; a use of an address
                    // already listed draws nothing new.
                    let draw = residual.magnitude();
                    let listed = self.pending.contains(&addr);
                    if !listed {
                        self.relocate_t_exhausted();
                        let short = self.room_shortfall(&voice, &draw);
                        if !crate::economy::lanes_short(&short).is_empty() {
                            self.log_room_refused(&voice, &draw, &short);
                            return Outcome::Refused(crate::verdict::REFUSE_ROOM);
                        }
                    }
                    // THE TORSION SOURCE: the open demand's pressure is the
                    // torsion the word actually carries. Every stored cycle
                    // is walked from the base assignment: an Open walk is
                    // the anholonomity — its residual span charges the
                    // lanes; a Closed loop is proven monodromy and charges
                    // nothing. A word with no cycle falls back to the
                    // δ-field's kinematic residue. Bounded per demand;
                    // per-lane as ever, never a summed winding.
                    let witnesses = crate::transducer::torsion_witness::witness_thought(&residual);
                    if witnesses.is_empty() {
                        // NO STORED CYCLE. The charge is read at the LEVEL
                        // the carrier is on (18.2): letters spell a word,
                        // words up to conjugacy are a class, a sentence is a
                        // nerve of words one level above that.
                        //
                        // There is no WORD arm here, and that is a reading
                        // of the six routes into this branch rather than an
                        // omission. `reduce.rs` builds through
                        // `from_residual` (306), `from_residual_framed`
                        // (442, 444) — no tape — and `ingest.rs` passes
                        // `g.substance()` (445, 464, 488), which clears
                        // `word` by construction. The one route that keeps a
                        // word is `reduce.rs:340`, the `is_sentence` branch.
                        // So a connection or translation tape cannot arrive
                        // at this arm, and an arm for one would be dead code
                        // asserting a carrier the path cannot deliver.
                        match residual.form() {
                            // A SENTENCE. Its words are its loop holonomies,
                            // and each closes in the order of its class. Per
                            // loop, never summed: a bundle's pressure is the
                            // lane-wise reading of its parts (12.6), and
                            // summing the orders into one bump count is the
                            // shape F2r1 removed from the witness branch.
                            Some(f) => {
                                for (w, _) in f.loop_words() {
                                    match w.order() {
                                        crate::transducer::alphabet::Order::Finite(p) => {
                                            for _ in 0..p {
                                                self.physics.bump(&mag);
                                            }
                                        }
                                        // A loop that never closes is an
                                        // open walk, the same anholonomity
                                        // the witness branch charges: it
                                        // pivots the lanes the demand
                                        // occupies and bumps them for the
                                        // letters it walked.
                                        crate::transducer::alphabet::Order::Infinite => {
                                            self.physics
                                                .pivot(crate::physics::Physics::lanes_of(&mag));
                                            for _ in 0..w.len() as u64 {
                                                self.physics.bump(&mag);
                                            }
                                        }
                                    }
                                }
                            }
                            // A PURE RESIDUAL CHAIN — no word yet, because
                            // the alphabet has no letter for it. This is the
                            // germination demand itself: charged by the
                            // support its chain touches, and LISTED.
                            None if !residual.sub().is_empty() => {
                                for _ in 0..residual.sub().len() as u64 {
                                    self.physics.bump(&mag);
                                }
                            }
                            // Neither nerve nor chain: nothing to reduce and
                            // nothing to list.
                            None => return Outcome::Pivot,
                        }
                    } else {
                        // A STORED CYCLE is the conjugate level already: an
                        // Open or Closed walk is a statement about a
                        // holonomy class. Each Open walk pivots the lanes
                        // the demand occupies and bumps them for the length
                        // IT walked; a Closed loop is proven monodromy and
                        // charges nothing.
                        for w in &witnesses {
                            if let crate::transducer::torsion_witness::HolonomicLoopWitness::Open { walked } = w {
                                self.physics.pivot(crate::physics::Physics::lanes_of(&mag));
                                for _ in 0..*walked as u64 {
                                    self.physics.bump(&mag);
                                }
                            }
                        }
                    }
                    // LIST the un-factored residual on the open-demand market.
                    // Capacity is THERMODYNAMIC (live memory headroom × lane
                    // tension, recomputed every listing) and eviction is by
                    // ACTION POTENTIAL — the market sheds its lowest-tension
                    // stale listing, never its oldest (FIFO is banned). A new
                    // demand weaker than everything already listed is not
                    // listed at all: the book holds only what the machine can
                    // honestly carry, ordered by what the network most needs.
                    // A residual a held word carries by shape, under a gauge
                    // the word does not hold, is holonomic demand — its
                    // tension projects onto lane 3, so the puncture that
                    // settles it is paid there.
                    let residual = {
                        let mut r = residual;
                        if crate::transducer::stalk::congruent(&self.stalks, |a| self.word_at(a), &r).is_some() {
                            r.mag[3] = r.sub().len() as u64;
                        }
                        r
                    };
                    if frontier {
                        self.quote_frontier.insert(&addr, ());
                    }
                    self.convergence.used_by(addr.as_bytes(), voice);
                    if !listed {
                        let k_now = self.k_step();
                        self.book_list((*residual).clone(), k_now, voice);
                        self.docket_connection(&residual, voice);
                        self.offered.insert(&addr, ());
                    }
                    Outcome::GermPending
                }
            }
        }
    }

    /// The reduction with one generator EXCLUDED from the alphabet — the syntropic
    /// stroke's redundancy probe: "does the REST of the basis already derive this
    /// generator?" (`exclude = None` is the ordinary full reduction.)
    pub(crate) fn reduce_excluding(&self, g: &Element, exclude: Option<&[u8]>) -> NormalForm {
        // Post-excision, a perceived demand is a CENTERED COBOUNDARY CHAIN — the
        // chain reduction decomposes it over the germinated patterns. The group
        // path below remains for pure-algebra elements.
        let sub = g.substance();
        if !sub.sub().is_empty() {
            let ex = exclude.and_then(|a| self.live_slot(a));
            return self.reduce_chain(sub.sub().to_vec(), ex);
        }
        // A12, ONCE. This peeled one generator at a time for up to 64
        // steps, re-deriving the whole question at every step from the new
        // residue -- and each step scanned the basis and kept a running best
        // by `descends`, which is selection by measure (law 10) and dropped
        // every factorisation that did not dominate (rule 15). A12 answers
        // the whole question in one reading: membership in the lattice the
        // held words span, decided by Hermite normal form, with the
        // factorisation as its witness and a canonical residue when it
        // fails. There is nothing left to peel, so the peel is gone.
        let s = sub;
        if s.is_identity() {
            return NormalForm::Decomposed(Vec::new()); // a pure transposition
        }
        // Terminal: the substance IS an installed generator.
        if let Some(k) = action_index_key(&s) {
            if let Some(addr) = self.by_action.get(&k) {
                if exclude.map(|e| e == addr.as_bytes()) != Some(true) {
                    if let Some(held) = self.basis.get(addr) {
                        debug_assert!(held.substance().same_action(&s));
                        if let Some(i) = self.live_slot(addr) {
                            return NormalForm::Decomposed(vec![i]);
                        }
                    }
                }
            }
        }
        let Some(gf) = s.letters().and_then(|l| l.spelling.word.form()) else {
            return NormalForm::Irreducible(Box::new(s));
        };
        let mut slots = Vec::new();
        let mut forms = Vec::new();
        for (addr, gen) in self.basis.fiber(crate::network::nerve::ROOT) {
            if exclude.map(|e| e == addr.as_slice()) == Some(true) {
                continue;
            }
            let Some(i) = self.live_slot(&addr) else { continue };
            if gen.substance().solution().is_none() {
                continue; // only group generators factor
            }
            let Some(f) = gen.letters().and_then(|l| l.spelling.word.form()) else {
                continue;
            };
            slots.push(i);
            forms.push(f);
        }
        if forms.is_empty() {
            return NormalForm::Irreducible(Box::new(s));
        }
        match crate::transducer::factor::factor_on_plane(&gf, &forms) {
            // `Decomposed` is the CITATION SET -- which held words the
            // substance decomposes through -- so the exponents are dropped
            // here and the tape itself is `factor::word_of` when a caller
            // needs the ordered product.
            crate::transducer::factor::Factored::Understood(tape) => {
                let mut cited: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
                let mut word = Vec::new();
                for (i, _) in tape {
                    let Some(slot) = slots.get(i).copied() else { continue };
                    let k = (slot as u64).to_le_bytes();
                    if cited.contains(&k) {
                        continue;
                    }
                    cited.insert(k, ());
                    word.push(slot);
                }
                NormalForm::Decomposed(word)
            }
            // A remainder is a demand and an unmeasured factorisation is
            // neither: both leave the substance irreducible here, and the
            // remainder travels as the residual it already is.
            _ => NormalForm::Irreducible(Box::new(s)),
        }
    }

    /// Peel the MAXIMAL ≺_DM-descending generator from substance `s`: try each
    /// invertible (group) generator γ, form `γ⁻¹ ∘ s`, and keep the one whose
    /// reduction magnitude descends most (tie-broken by basis order — deterministic,
    /// so the reduction is confluent). `None` when no generator lowers `s`.
    /// The kernel's chain reduction — delegates to the shared projective
    /// decomposition ([`decompose_over`]) at the metabolic projection budget.
    pub(crate) fn reduce_chain(&self, first: Vec<(i64, i64, u8, u8)>, exclude: Option<usize>) -> NormalForm {
        normal_form(&self.language, &first, exclude)
    }

}
