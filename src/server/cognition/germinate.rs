// AI-GENERATED CONTENT — see DISCLAIMER.md.
//
//! The percept path and the strokes that work it: ingest, reduce,
//! condense, consolidate, and the utterance receipts.
//!
//! Split from cognition.rs under Stage 8's reading bound. A private
//! item is visible to its module AND its descendants, so these are the
//! same `impl Kernel` reaching the same private carriers: the split
//! moves code and changes nothing else.

use super::*;

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
        if self.door_defers(voice) {
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
                    .map(|(a, _)| crate::transducer::a21::Address::from(a))
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
                    let _ = self.admission.record(Receipt { voice, germ: saddr.clone() });
                    self.record_spoken(saddr.clone(), voice);
                    let quorum = self.admission.voices(&saddr) >= crate::network::consensus::QUORUM;
                    if quorum && self.physics.can_expand(lanes) {
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
                // KB COMPLETION — a genuinely new base thought adjoins on
                // two-kernel consilience. Confluence is the gate: two kernels
                // reducing the same demand reach the SAME residual (same address),
                // so agreement is a theorem about the algebra, not a matched card.
                let addr = residual.address();
                // A refused draw is not an attestation: room that does not
                // accept the residual must not count toward quorum.
                let heard = self.admission.voices_of(&addr).iter().any(|v| v == &voice);
                let n = self.admission.voices(&addr);
                let n_after = if heard { n } else { n.saturating_add(1) };
                // ENTROPIC EXPANSION needs BOTH consilience (quorum of distinct
                // kernels) AND metabolic fuel (reserve above the floor on the germ's
                // lanes). A germ at quorum with a depleted lane DEFERS — it stays
                // pending while the mesh optimizes/regenerates, and installs on a
                // later voice's submission once the reserve recovers (the recheck
                // below, not record()'s one-shot crossing, so deferral never
                // strands a germ).
                let quorum = n_after >= crate::network::consensus::QUORUM;
                if quorum && self.physics.can_expand(lanes) {
                    let _ = self.admission.record(Receipt { voice, germ: addr.clone() });
                    let _ = self.treasury.settle_install(&residual, voice); // POW++ gate only
                    let mu = crate::economy::price(&residual.magnitude(), &self.physics.pressure);
                    let anchor = self.anchor();
                    // PROVISIONAL (Step 5): consilience escrows pending the
                    // court's admission docket; the word joins the basis as
                    // local dialect, final settlement waits for the verdict.
                    self.treasury.escrow_award(&addr, self.admission.voices_of(&addr), mu, anchor);
                    self.record_deflation(addr.clone(), listing, residual.magnitude());
                    let k_now = self.k_step();
                    self.provisional_k.insert(&addr, k_now);
                    self.provisional.insert(&addr, residual.clone());
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
                    // THE TORSION SOURCE: the open demand's pressure is the
                    // torsion the word actually carries. Every stored cycle
                    // is walked from the base assignment: an Open walk is
                    // the anholonomity — its residual span charges the
                    // lanes; a Closed loop is proven monodromy and charges
                    // nothing. A word with no cycle falls back to the
                    // δ-field's kinematic residue. Bounded per demand;
                    // per-lane as ever, never a summed winding.
                    let witnesses = crate::transducer::toroid::witness_thought(&residual);
                    let k = match witnesses.is_empty() {
                        true => {
                            let residue = crate::torsion::residue_of_chain(residual.sub());
                            crate::torsion::required_k_steps(&residue)
                        }
                        false => witnesses
                            .iter()
                            .map(|w| match w {
                                crate::transducer::toroid::HolonomicLoopWitness::Open { walked } => {
                                    // A failed loop is the torsion charge the economy
                                    // already names: it pivots the lanes the demand
                                    // occupies, and the listing below carries it to
                                    // the market as the demand for a new fiber.
                                    self.physics.pivot(crate::physics::Physics::lanes_of(&mag));
                                    (*walked as u64).max(1)
                                }
                                crate::transducer::toroid::HolonomicLoopWitness::Closed { .. } => 0,
                            })
                            .sum::<u64>(),
                    }
                    .min(8);
                    for _ in 0..k.max(1) {
                        self.physics.bump(&mag);
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
                    let draw = residual.magnitude();
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
                    if self.pending.contains(&addr) {
                        let _ = self.admission.record(Receipt { voice, germ: addr.clone() });
                    } else {
                        let k_now = self.k_step();
                        self.relocate_t_exhausted();
                        if !self.fits_book(&voice, &draw) {
                            self.log_room_refused(&voice, &draw);
                        } else {
                            let _ = self.admission.record(Receipt { voice, germ: addr.clone() });
                            self.book_list(residual.clone(), k_now, voice);
                            self.docket_connection(&residual, voice);
                            self.offered.insert(&addr, ());
                        }
                    }
                    Outcome::GermPending
                }
            }
        }
    }

    /// The reduction with one generator EXCLUDED from the alphabet — the syntropic
    /// stroke's redundancy probe: "does the REST of the basis already derive this
    /// generator?" (`exclude = None` is the ordinary full reduction.)
    pub(super) fn reduce_excluding(&self, g: &Element, exclude: Option<&[u8]>) -> NormalForm {
        // Post-excision, a perceived demand is a CENTERED COBOUNDARY CHAIN — the
        // chain reduction decomposes it over the germinated patterns. The group
        // path below remains for pure-algebra elements.
        let sub = g.substance();
        if !sub.sub().is_empty() {
            let ex = exclude.and_then(|a| self.live_slot(a));
            return self.reduce_chain(sub.sub().to_vec(), ex);
        }
        let mut s = sub;
        let mut word = Vec::new();
        let budget = self.peel_budget(&g.magnitude());
        // ≺_DM is well-founded so this terminates; the 64 is only a safety net.
        for step in 0..64 {
            if s.is_identity() {
                return NormalForm::Decomposed(word); // fully reduced (empty = pure transposition)
            }
            // Terminal: the substance IS an installed generator (length-1 sentence).
            if let Some(k) = action_index_key(&s) {
                if let Some(addr) = self.by_action.get(&k) {
                    if exclude.map(|e| e == addr.as_bytes()) != Some(true) {
                        if let Some(held) = self.basis.get(addr) {
                            debug_assert!(held.substance().same_action(&s));
                            if let Some(i) = self.live_slot(addr) {
                                word.push(i);
                                return NormalForm::Decomposed(word);
                            }
                        }
                    }
                }
            }
            // METABOLIC THROTTLE: peeling is the expensive arm.
            if step >= budget {
                return NormalForm::Irreducible(s);
            }
            match self.best_peel(&s, exclude) {
                Some((i, next)) => {
                    word.push(i);
                    s = next;
                }
                None => return NormalForm::Irreducible(s),
            }
        }
        NormalForm::Irreducible(s)
    }

    /// The metabolic PEEL BUDGET — how many expensive peels the confluent reduction
    /// may spend, governed by the current per-lane PRESSURE (the entropic backlog),
    /// never a monotone scalar (#34). Polytonic: the most-pressured lane governs
    /// (Liebig). Calm → full reduction; loaded → shallow; saturated → direct-match
    /// only, then germinate the compound (refined later, when the mesh is calm).
    fn peel_budget(&self, mag: &[u64; 4]) -> usize {
        // The occupied lane of this reduction; no max across lanes.
        for l in 0..4 {
            if mag[l] > 0 {
                let unit = crate::physics::expansion_unit().max(1);
                return (self.physics.reserve[l] / unit) as usize;
            }
        }
        0
    }

    /// Peel the MAXIMAL ≺_DM-descending generator from substance `s`: try each
    /// invertible (group) generator γ, form `γ⁻¹ ∘ s`, and keep the one whose
    /// reduction magnitude descends most (tie-broken by basis order — deterministic,
    /// so the reduction is confluent). `None` when no generator lowers `s`.
    fn best_peel(&self, s: &Element, exclude: Option<&[u8]>) -> Option<(usize, Element)> {
        // Retired by M6/A12 (HNF membership + budgeted coset shortening).
        // Left as a fiber walk so the group path still compiles; it is not
        // repaired in place.
        let before = s.reduction_magnitude();
        let mut best: Option<(usize, Element, [u64; 4])> = None;
        for (addr, gen) in self.basis.fiber(crate::network::nerve::ROOT) {
            if exclude.map(|e| e == addr.as_slice()) == Some(true) {
                continue;
            }
            let Some(i) = self.live_slot(&addr) else { continue };
            let gs = gen.substance();
            let Some(inv) = gs.solution() else { continue }; // only group generators peel
            let cand = inv.compose(s);
            let m = cand.reduction_magnitude();
            if crate::economy::descends(&m, &before) {
                let take = match &best {
                    Some((_, _, bm)) => crate::economy::descends(&m, bm),
                    None => true,
                };
                if take {
                    best = Some((i, cand, m));
                }
            }
        }
        best.map(|(i, cand, _)| (i, cand))
    }


    /// The kernel's chain reduction — delegates to the shared projective
    /// decomposition ([`decompose_over`]) at the metabolic projection budget.
    fn reduce_chain(&self, first: Vec<(i64, i64, u8, u8)>, exclude: Option<usize>) -> NormalForm {
        normal_form(&self.language, &first, exclude)
    }

    /// THE SYNTROPIC STROKE — one bounded consolidation step, run when a lane's
    /// reserve is depleted. Probe the cursor generator for REDUNDANCY: if the rest
    /// of the basis already derives it (its substance decomposes with it excluded),
    /// it is a closed critical pair wearing a generator's name — RETIRE it, and the
    /// basis compresses toward a minimal confluent generating set. Either way the
    /// reserve REGENERATES: optimization is the recovery stroke, so entropic
    /// expansion resumes only at the pace the syntropic counterpart earns.
    /// One bounded consolidation stroke, public for the server's idle
    /// deliberation worker: condense, torse, probe redundancy, regenerate.
    pub fn consolidate(&mut self) {
        self.optimize();
    }

    /// The syntropic stroke reads the field's stagnation per chart per lane.
    /// A chart stagnant on gauge is a condense candidate, holonomic torse,
    /// shell refine, spatial re-anchor. Temporal stagnation relocates on T.
    /// Candidates from open and the ruled cover.
    pub fn syntropic_stroke(&mut self) {
        let mut budget = self.physics.stroke();
        let mut jobs: Vec<(crate::transducer::stalk::Invariant, [bool; 4])> = Vec::new();
        // index: budgeted cursor over the open docket
        for (addr, _) in self.open.cursor(crate::network::nerve::ROOT, &mut budget) {
            let Some(listing) = self.field.get(&addr) else {
                continue;
            };
            for e in &listing.entries {
                if e.state.is_flat() {
                    continue;
                }
                let lanes = std::array::from_fn(|l| e.stagnant[l] > 0);
                if lanes.iter().any(|&x| x) {
                    jobs.push((e.chart.clone(), lanes));
                }
            }
        }
        // index: ruled cover fiber
        let ruled: Vec<Vec<u8>> = self.ruled_addrs.fiber(crate::network::nerve::ROOT).map(|(a, _)| a).collect();
        for addr in &ruled {
            if !budget.spend() {
                break;
            }
            if let Some(w) = self.word_at(addr) {
                jobs.push((crate::transducer::stalk::invariant_of(w), [true; 4]));
            }
        }
        let idle = jobs.is_empty();
        let mut t_stale = false;
        for (inv, lanes) in jobs {
            if lanes[1] {
                // Canonical, not positional: the least address on the chart.
                // Two hyphae holding one chart must condense the same word,
                // or the distributed language forks along insertion order.
                if let Some(a) = self.stalks.fiber(&inv).iter().min().cloned() {
                    let _ = self.condense_on(&a);
                }
            }
            if lanes[2] {
                t_stale = true;
            }
            if lanes[3] {
                // Canonical, not positional: the least address on the chart.
                // Two hyphae holding one chart must condense the same word,
                // or the distributed language forks along insertion order.
                if let Some(a) = self.stalks.fiber(&inv).iter().min().cloned() {
                    self.torse_at(&a);
                    self.refine_stratum(&a);
                }
            }
            if lanes[0] {
                self.reanchor_chart(&inv);
            }
        }
        if t_stale {
            self.relocate_t_exhausted();
        }
        for addr in &ruled {
            if !budget.spend() {
                break;
            }
            let Some(candidate) = self.basis.get(addr).map(|w| (**w).clone()) else {
                continue;
            };
            if matches!(self.reduce_excluding(&candidate, Some(addr.as_slice())), NormalForm::Decomposed(_)) {
                self.retire_redundant(addr);
            }
        }
        if idle {
            let _ = self.condense_stroke();
        }
        self.physics.regenerate();
    }

    fn reanchor_chart(&mut self, inv: &crate::transducer::stalk::Invariant) {
        let members = self.stalks.fiber(inv);
        for addr in members {
            let Some(w) = self.word_at(&addr).cloned() else {
                continue;
            };
            if !w.is_sentence() {
                continue;
            }
            let Some(mut form) = crate::transducer::sentence::from_element_form(&w) else {
                continue;
            };
            let Some(gamma) = form.solve_coboundary() else {
                continue;
            };
            if gamma.iter().all(|g| g.is_identity()) {
                continue;
            }
            let _ = form.regauge(&gamma);
            let after = crate::transducer::sentence::as_element_form(&form, w.magnitude());
            let Some(after) = after else {
                continue;
            };
            let payload = crate::transducer::rewrite::RewritePayload::Reanchor { gamma };
            let Some(voice) = self.discoverer_of(&addr) else {
                continue;
            };
            let _ = self.mint_rewrite(
                crate::transducer::rewrite::RewriteKind::Reanchor,
                &[w],
                &after,
                payload,
                voice,
            );
            break;
        }
    }

    /// The whole stroke under one lock (the reflex and test paths).
    pub(crate) fn optimize(&mut self) {
        self.syntropic_stroke();
        if let Some(addr) = self.condense_stroke() {
            let Some(candidate) = self.basis.get(&addr).map(|w| (**w).clone()) else {
                return;
            };
            if let NormalForm::Decomposed(_) = self.reduce_excluding(&candidate, Some(addr.as_bytes())) {
                self.retire_redundant(&addr);
            }
        }
    }

    /// The stroke for the idle deliberation worker: the cheap moves (condense,
    /// torse, regenerate) run here under the lock; the redundancy probe — a
    /// full decomposition — is handed out as (address, candidate, the basis
    /// without it) to run on that snapshot with no lock held, exactly as
    /// adoption's derivability does. The reflex path never waits on it.
    pub fn consolidate_stroke(&mut self) {
        self.syntropic_stroke();
    }

    /// Retire a plain card the rest of the basis derives (the off-lock probe
    /// found it decomposable). Returns the cards retired (0 or 1): the word
    /// may have gone, or gained a fibre, while the probe ran.
    pub fn retire_redundant(&mut self, addr: &[u8]) -> usize {
        let Some(held) = self.basis.get(addr).map(|w| (**w).clone()) else { return 0 };
        if !held.sections().is_empty() || crate::transducer::sentence::carries_torsion(&held) {
            return 0;
        }
        let mag = held.magnitude();
        let elem = self.basis_remove(addr);
        self.ruled_admit(addr);
        self.cavity_book.insert(&addr, elem.clone());
        self.cavities.push(elem);
        self.physics.syntropy(&mag);
        1
    }

    /// Condense and torse at the cursor; returns the cursor index when the
    /// cursor word is a plain card (a redundancy-probe candidate — never a
    /// bundle: its sections/cycles carry family volume no other word speaks).
    fn condense_stroke(&mut self) -> Option<crate::transducer::a21::Address> {
        let mut budget = self.physics.stroke();
        let Some((addr, word)) = self.basis.cursor(&self.optimize_cursor, &mut budget).next() else {
            self.optimize_cursor.clear();
            return None;
        };
        let own = crate::transducer::a21::Address::from(addr.as_slice());
        self.optimize_cursor = addr;
        let word = (**word).clone();
        if !word.sub().is_empty() {
            let inv = crate::transducer::stalk::invariant_of(&word);
            let fk = chart_footprint_key(&inv, word.sub());
            let mut fiber: Vec<Vec<u8>> = self
                .by_footprint
                .link(&fk)
                .map(|(a, _)| a)
                .filter(|a| a.as_slice() != own.as_bytes())
                .collect();
            if fiber.is_empty() {
                fiber = self
                    .stalks
                    .fiber(&inv)
                    .iter()
                    .cloned()
                    .filter(|a| a.as_slice() != own.as_bytes())
                    .collect();
            }
            let mut spend = self.physics.stroke();
            for sibling in fiber {
                if !spend.spend() {
                    break;
                }
                let card = self.basis.get(&sibling).is_some_and(|w| {
                    w.sections().is_empty() && !crate::transducer::sentence::carries_torsion(w)
                });
                if !card {
                    continue;
                }
                let _ = self.absorb_pair(own.clone(), crate::transducer::a21::Address::from(sibling));
            }
        }
        self.torse_at(&own);
        let Some(held) = self.basis.get(&own) else {
            return None;
        };
        if held.sections().is_empty() && !crate::transducer::sentence::carries_torsion(held) {
            return Some(own);
        }
        None
    }

    /// A read-only view of the germinated basis in canonical key order — a
    /// display and test seam, never an index for cognition, where every read
    /// of a word is `word_at` by its address. It reads `basis` itself:
    /// `language.words` is a snapshot that diverges from the carrier the
    /// moment `word_mut` makes an `Arc` unique, so a view onto it reports the
    /// word as it was before the puncture (rule 12, one fact one carrier).
    pub fn basis(&self) -> Vec<std::sync::Arc<Element>> {
        self.basis
            .fiber(crate::network::nerve::ROOT)
            .map(|(_, w)| std::sync::Arc::clone(w))
            .collect()
    }

    /// QUOTE — a local reading of the page: listings still on_page, in
    /// atlas order, under one stroke Budget. Tests use this; the wire
    /// quote is an incidence query over the voice's charts.
    pub fn quote(&mut self) -> ([u64; 4], Vec<(Element, [u64; 4])>) {
        self.remeasure_pressure();
        let surface = crate::economy::yield_surface(&self.physics);
        let mut priced = Vec::new();
        let mut budget = self.physics.stroke();
        for (addr, _) in self.on_page.cursor(crate::network::nerve::ROOT, &mut budget) {
            let Some((p, _)) = self.pending.get(&addr) else { continue };
            let mut mag = p.magnitude();
            mag[2] = self.t_extent(&p.address(), self.height());
            let mu = crate::economy::price(&mag, &self.physics.pressure);
            priced.push((p.clone(), mu));
        }
        (surface, priced)
    }

    /// ATTEST — a peer's INDEPENDENT derivation of an open residual arrives as a
    /// spoken `demand` act. The projective reduction runs first (a stale quote
    /// DECOMPOSES instead of double-minting); a genuine irreducible joins the
    /// admission as this voice's attestation — consilience through conversation.
    /// The local path has no voice K on the call: K = 0, same as a demand
    /// frame that omits it. A later K is `attest_at`.
    pub fn attest(&mut self, chain: &[(i64, i64, u8, u8)], voice: [u8; 32]) -> Outcome {
        self.attest_at(chain, voice, &[], 0)
    }

    /// Demand with the speaker's citations and the voice's K. Same (vk,
    /// address, K) is answered from the docket; a later K is a use.
    pub fn attest_at(
        &mut self,
        chain: &[(i64, i64, u8, u8)],
        voice: [u8; 32],
        cites: &[crate::anastome::dialogue::CiteWire],
        k: u64,
    ) -> Outcome {
        self.ensure_bonded(voice);
        let germ = element::from_residual(&chain.to_vec());
        let listing = germ.address();
        if let Some(out) = self.answer_if_duplicate(&voice, &listing, k) {
            return out;
        }
        let Some((language, _budget)) = self.reflex_open(voice) else {
            return Outcome::GermPending;
        };
        let frame = (0usize, 0usize, 0usize, 0usize);
        let reduction = reduce_proven(&language, germ, cites, &frame, listing.clone());
        let residual = match &reduction.form {
            NormalForm::Decomposed(_) => 0,
            NormalForm::Irreducible(r) => r.sub().len(),
        };
        let out = self.commit_all(vec![reduction], voice);
        self.record_utterance(voice, listing, k, cites.len(), residual, chain.len(), out)
    }

    /// A repeated (vk, address, K) is the reply last given, if any.
    pub fn answer_if_duplicate(&self, voice: &[u8; 32], addr: &[u8], k: u64) -> Option<Outcome> {
        let out = self.utterance_of(voice, addr, k)?.clone();
        eprintln!(
            "  court attest duplicate {:02x}{:02x}… voice {:02x}{:02x}… K {k}",
            addr[0], addr[1], voice[0], voice[1]
        );
        Some(out)
    }

    fn utterance_of(&self, voice: &[u8; 32], addr: &[u8], k: u64) -> Option<&Outcome> {
        self.utterances.get(addr)?.get(voice)?.get(&utterance_k_key(k))
    }

    fn remember_utterance(&mut self, voice: [u8; 32], addr: Address, k: u64, out: Outcome) {
        let kk = utterance_k_key(k);
        let mut by_v = self.utterances.remove(&addr).unwrap_or_default();
        let mut by_k = by_v.remove(&voice).unwrap_or_default();
        by_k.insert(kk, out);
        by_v.insert(voice, by_k);
        self.utterances.insert(addr, by_v);
    }

    /// Seal the utterance receipt and log the first hearing.
    pub fn record_utterance(
        &mut self,
        voice: [u8; 32],
        addr: Address,
        k: u64,
        cites: usize,
        residual: usize,
        chain_len: usize,
        out: Outcome,
    ) -> Outcome {
        eprintln!(
            "  court attest {:02x}{:02x}… voice {:02x}{:02x}… K {k} cites {cites} residual {residual} of {chain_len}",
            addr[0], addr[1], voice[0], voice[1]
        );
        self.remember_utterance(voice, addr, k, out.clone());
        out
    }

    /// Words taught on a decomposed sentence: each cited word the docket
    /// holds anchored. Sentence length does not decide.
    pub fn taught_anchored(&self, word: &[usize]) -> Vec<u8> {
        taught_anchored_words(word.iter().filter_map(|&i| self.language.words.get(i).map(|e| e.as_ref())))
    }

    /// The consilience voices on record for a germ — the settle receipt's parties.
    pub fn voices_of(&self, germ: &[u8]) -> Vec<[u8; 32]> {
        self.admission.voices_of(germ)
    }

    /// The basis's interior VOLUME — the full fiber span across all punctured
    /// axes (free sections + every torsion orbit to its period; the condensation
    /// metric: cardinality falls while this rises, and cycles carry span in O(1)).
    pub fn sections_total(&self) -> usize {
        self.basis
            .fiber(crate::network::nerve::ROOT)
            .map(|(_, t)| t.fiber_span().saturating_sub(1))
            .sum()
    }

    /// The torsion cycles held across the basis — how much of the language has
    /// condensed from enumeration into group presentation.
    pub fn torsion_total(&self) -> usize {
        self.basis
            .fiber(crate::network::nerve::ROOT)
            .map(|(_, t)| {
                let n = crate::transducer::sentence::form_torsion(t).len();
                if n > 0 {
                    n
                } else {
                    t.torsion().len()
                }
            })
            .sum()
    }

    /// The mesh's algebraic clock — the next K-step index (chronometry import).
    pub fn k_step(&self) -> u64 {
        self.chain.next_step_index()
    }

    /// The current chronometric anchor for ledger entries: (K, head hash) — the
    /// unforgeable temporal binding every settlement carries.
    pub(super) fn anchor(&self) -> crate::economy::Anchor {
        (self.chain.next_step_index(), self.chain.head_hash())
    }

    /// The chronometric receipt chain (verifiable, parent-linked).
    pub fn chain(&self) -> &crate::chronometry::ReceiptChain {
        &self.chain
    }

    /// THE COHESION WITNESS (cohesion import) — the joint (C, I) state, both
    /// axes exact and never traded: C counts open demands sitting on KNOWN
    /// geometry (closure failing — collapse-side work); I counts open demands on
    /// geometry NO thought speaks (coverage failing — expansion-side work).
    pub fn cohesion(&mut self) -> crate::cohesion::CohesionWitness {
        let rank = self.closure_now();
        cohesion_over(&rank)
    }
}
