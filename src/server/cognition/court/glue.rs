// 23.7, 47.3 — GLUE: what a word fetched from a peer becomes here, and how
// two organs converge on one object.
//
//! A fetched word is replayed against this court's own tapes before it is
//! admitted to anything. A witness that RE-DERIVED the object converges by
//! `same_vertex` (M1), never by comparing the bytes it is spelled with.
//! The install that convergence rests on is `install.rs`.

use crate::cognition::*;
use crate::transducer::address::Address;
use crate::transducer::element::{self, Element};

/// RM1 — a Congruent fill's remainder as the glue carries it to the mint:
/// the class `κ`, the deficit `g`, the period `ℓ`.
type Torsed = (Box<crate::transducer::sentence::ClassVector>, Box<crate::transducer::alphabet::Rotor>, u32);

impl Kernel {
    /// On the stroke of the act that fetched the word (65.5): the exchange's
    /// glue, or the witness door's offer — never one of its own.
    pub fn glue_fetched(
        &mut self,
        spoken: Element,
        peer: [u8; 32],
        self_id: [u8; 32],
        cover: &mut crate::network::nerve::Cover,
        budget: &mut crate::physics::Budget,
    ) -> Glue {
        let addr = spoken.address();
        // PHASE 1 DIAGNOSTIC — WHICH PATH HOLDS A FETCHED WORD AS DIALECT:
        // three courts stood discordant in one stratum with no voice
        // speaking, c gluing b's words as dialect 121 times.
        let dialect = |path: usize, colour: bool| {
            static SEEN: [std::sync::atomic::AtomicU64; 5] = [const { std::sync::atomic::AtomicU64::new(0) }; 5];
            const NAMES: [&str; 5] = ["already heard", "colour unproven", "a citation missing", "a transition between sections", "a class simplex"];
            let n = SEEN[path].fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1;
            if n.is_power_of_two() {
                eprintln!("  glue dialect ({n}) by {} — colour word {colour}", NAMES[path]);
            }
        };
        let colour = spoken.is_colour_word();
        if self.heard.contains(&addr) {
            dialect(0, colour);
            crate::observe::note_replication(Some(peer), crate::observe::rep::GLUE_HEARD, 1);
            return Glue::Dialect;
        }
        if let Some(reason) = self.court_replay(&spoken, budget) {
            let inv = crate::transducer::stalk::invariant_of(&spoken);
            self.heard_mark(addr, &inv);
            crate::observe::note_replication(Some(peer), crate::observe::rep::GLUE_REPLAY_REFUSED, 1);
            return Glue::Slashed(reason);
        }
        // 78.55 — A PEER'S COLOUR WORD IS ADMITTED HERE BY A PROOF HELD
        // HERE: the task that witnessed it, re-derived from this court's own
        // copy of that family, or the two installed words it cites. Its
        // integrity replay above proves neither. Unproven, it waits as
        // dialect and is not marked heard: once the words it cites are
        // installed here, the next fetch proves it.
        if spoken.is_colour_word() && !self.colour_proof(&spoken, None).stands() {
            dialect(1, colour);
            crate::observe::note_replication(Some(peer), crate::observe::rep::GLUE_COLOUR_UNPROVEN, 1);
            return Glue::Dialect;
        }
        let inv = crate::transducer::stalk::invariant_of(&spoken);
        let asg = element::assignment(spoken.sub());
        if spoken.is_sentence() {
            // COVER COMPLETENESS IS ASKED OF THE NERVE (54.5 Road (d), B4).
            // This decoded the arriving sentence to read its vertices for
            // `Glued` citations; a citation is a 1-simplex `⟨A_i, A_j, φ⟩`
            // and rides the envelope, so the question — does every form this
            // one mates with resolve here — is a read of `nerve` and builds
            // no form. A pair form's Word vertices were never part of it:
            // they are the percept's own bodies, not citations.
            let missing = spoken.receipts().nerve.iter().any(|e| {
                e.target != addr && self.word_at(e.target.as_bytes()).is_none()
            });
            if missing {
                dialect(2, colour);
                self.heard_mark(addr, &inv);
                crate::observe::note_replication(Some(peer), crate::observe::rep::GLUE_CITATION_MISSING, 1);
                return Glue::Dialect;
            }
        }
        let mag = spoken.magnitude();
        let lanes = crate::network::nerve::occupied_lanes(&mag);
        let mut verts: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
        verts.insert(self_id, ());
        verts.insert(peer, ());
        for lane in &lanes {
            for v in cover.fiber(*lane, &inv) {
                verts.insert(v, ());
            }
        }
        // A chart with no complex yet stands on an empty one; this is
        // where its first section is written.
        let mut cx = match self.complexes.remove(&inv.key()) {
            Some(c) => c,
            None => crate::transducer::cech::Complex::new(),
        };
        let vol0 = self.volume_of_simplex(&inv);
        let (peer_gamma, peer_carriage) = (
            match spoken.carried_rotor() {
                Some(r) => r,
                None => crate::transducer::alphabet::Rotor::identity(),
            },
            match spoken.versor() {
                Some(v) => v,
                None => crate::transducer::element::identity_versor(),
            },
        );
        cx.set_section(
            peer,
            crate::transducer::cech::Section {
                addr: addr.clone(),
                gamma: peer_gamma,
                carriage: peer_carriage,
            },
            vol0,
        );
        let self_rep = cx.sections.get(self_id.as_slice()).map(|s| s.addr.clone());
        let mut outcome = Glue::Admitted;
        let held_rep = self_rep.and_then(|a| self.word_at(&a).cloned());
        // PHASE 1 ITEM 1, 78.62 STEP 4 — WHAT THE PEER'S SENTENCE IS to this
        // court's own section on its chart, read exactly and counted per
        // peer: the instrument the court-triangle balance is read from. A
        // reading only: what is admitted below is unchanged.
        if let Some(rep) = held_rep.as_ref().filter(|r| r.is_sentence() && spoken.is_sentence()) {
            use crate::observe::rep;
            use crate::transducer::sentence::{from_element_form, relation, Relation};
            let axis = match (rep.address() == addr, from_element_form(rep), from_element_form(&spoken)) {
                (true, _, _) => Some(rep::RELATION_SAME),
                (false, Some(h), Some(o)) => Some(match relation(&h, &o) {
                    Relation::Refracted(_) => rep::RELATION_REFRACTED,
                    Relation::Conjugate(_) => rep::RELATION_CONJUGATE,
                    Relation::Unspellable(_) => rep::RELATION_UNSPELLABLE,
                    Relation::Unrelated => rep::RELATION_DISTINCT,
                }),
                (false, _, _) => None,
            };
            if let Some(axis) = axis {
                crate::observe::note_replication(Some(peer), axis, 1);
            }
            // 78.61 C1 — AND PERCEIVED: both lifted into perception, compared
            // lane by lane on TUniversal {S, G, T, H}: coherent where the four
            // deltas are congruent; else each diverging lane counted apart.
            if let (Some(h), Some(o)) = (from_element_form(rep), from_element_form(&spoken)) {
                use crate::transducer::sentence::{compare, lift};
                let c = compare(&lift(&h, budget), &lift(&o, budget));
                if c.coherent() {
                    crate::observe::note_replication(Some(peer), rep::PERCEIVED_COHERENT, 1);
                } else if c.unread != (0, 0) {
                    crate::observe::note_replication(Some(peer), rep::PERCEIVED_UNREAD, 1);
                }
                for lane in c.diverging() {
                    crate::observe::note_replication(Some(peer), rep::RESIDUE_S + lane, 1);
                }
            }
        }
        if let Some(rep) = &held_rep {
            match crate::transducer::cech::transition(rep, &spoken) {
                None => {}
                Some(g) if g.is_identity() => {}
                Some(g) if crate::transducer::stalk::placement_of(&g).is_some() => {
                    outcome = Glue::Attached;
                }
                // TWO SECTIONS OF ONE CHART RELATED BY A TURN ARE BOTH KEPT
                // (ruled 2026-09-30, the operator's word: every section, never
                // a pair — 78.44 — and the curvature between them is content).
                // The peer's word was held as dialect here, so three courts
                // stood discordant in one stratum for good, each keeping its
                // own section and refusing the others' (c: 121 re-glues of b's
                // words as dialect with no voice speaking). It is admitted as
                // another member of the stratum; the turn is the cocycle it
                // files, its class recorded. THE GATE (the operator's
                // directive): only an EXACT rotation of finite order links
                // two sections — the chart's orbit is then finite — and a
                // transition that is no such rotation is a genuine
                // contradiction on the chart, refused.
                // A WORD NO MOTION RELATES TO THE HELD ONE IS A DISTINCT WORD
                // (ruled 2026-09-30, the operator's word): not a second
                // section of the same object but another word sharing the
                // chart — one chart held 46–51 installed words at each court
                // on epoch 115, and refusing these as contradictions kept the
                // courts' installed sets apart for good. It is admitted as
                // another member, with no cocycle.
                Some(g) => match crate::transducer::cech::rotor_of(&g) {
                    None => {}
                    // A rotation relates them: it must be exact and finite,
                    // and is then filed as the cocycle; one that is not is a
                    // contradiction between sections of one object, refused,
                    // named by the part of the gate it fails.
                    Some(r) => {
                        use crate::transducer::alphabet::Order;
                        let gate = match (matches!(r.order(), Some(Order::Finite(_))), rotor_versor(&r), r.class_key()) {
                            (false, _, _) => Err("sections of one chart related by a rotation of no finite order"),
                            (true, Some(v), Some(ck)) => Ok((v, ck)),
                            (true, _, _) => Err("sections of one chart related by a rotation with no versor or class"),
                        };
                        match gate {
                            Ok((v, ck)) => self.push_cocycle(&inv, self_id, peer, v, ck),
                            Err(why) => {
                                dialect(3, colour);
                                outcome = Glue::Slashed(why);
                            }
                        }
                    }
                },
            }
        }
        let others: Vec<Vec<u8>> = verts.scoped().map(|(k, _)| k).collect();
        for v in &others {
            if v.as_slice() == peer.as_slice() {
                continue;
            }
            let Some(sec) = cx.sections.get(v).cloned() else {
                continue;
            };
            let Some(w) = self.word_at(&sec.addr) else {
                continue;
            };
            // R6 (47.3): two sections of one body at two vantages glue by
            // the identity edge — the versor between their shapes is the
            // identity — which fills, pays and records; two bodies a versor
            // relates but no alphabet rotor spells stand as a remainder
            // edge, the obstruction recorded. Words with a group transition
            // glue by its rotor, as before.
            let bodies = (
                crate::transducer::element::shape_of_chain(w.sub()),
                crate::transducer::element::shape_of_chain(spoken.sub()),
            );
            let tr = crate::transducer::cech::transition(w, &spoken);
            match tr.as_ref().and_then(crate::transducer::cech::rotor_of) {
                Some(g) => {
                    let carriage = match tr.as_ref().and_then(|el| el.versor()) {
                        Some(c) => c,
                        None => crate::transducer::element::identity_versor(),
                    };
                    cx.set_edge(
                        v,
                        peer.as_slice(),
                        g,
                        carriage,
                        vol0,
                    );
                }
                None => match bodies {
                    (Some(a), Some(b)) => match crate::transducer::blade::versor_between(&a, &b, budget) {
                        Ok(between)
                            if crate::transducer::element::same_versor(
                                &between,
                                &crate::transducer::element::identity_versor(),
                            ) =>
                        {
                            cx.set_edge(
                                v,
                                peer.as_slice(),
                                crate::transducer::alphabet::Rotor::identity(),
                                crate::transducer::element::identity_versor(),
                                vol0,
                            )
                        }
                        Ok(_) => cx.set_remainder_edge(v, peer.as_slice()),
                        Err(_) => {}
                    },
                    _ => {
                        if tr.is_some() {
                            cx.set_remainder_edge(v, peer.as_slice());
                        }
                    }
                },
            }
        }
        // 78.26(6) — the closed rings a moved gauge reopened, counted by what
        // each cut; the court has no candidates, only the cover's gauges.
        for (_, cut) in cx.coboundary_on_link(&peer, |_| self.volume_of_simplex(&inv)) {
            match cut {
                crate::transducer::cech::Cut::Filled => self.reopened[0] += 1,
                crate::transducer::cech::Cut::Congruent => self.reopened[1] += 1,
            }
        }
        let mut filled = false;
        let mut class_simplex: Option<(crate::transducer::alphabet::Rotor, usize)> = None;
        // RM1 — the congruent fill's remainder (κ, g, ℓ), its words, and the
        // simplex's own key: the Torse rewrite's inputs.
        let mut congruent: Option<(Torsed, Vec<Address>, Vec<u8>)> = None;
        let mut fill_gamma = crate::transducer::alphabet::Rotor::identity();
        let mut fill_carriage = crate::transducer::element::identity_versor();
        let mut fill_addrs: Vec<Address> = Vec::new();
        let spoken_rotor = spoken.letters().map(|l| l.spelling.word.rotor());
        // index: link of the fetched peer
        for key in cx.link_of(&peer) {
            let Some((d, volume, vertices)) = cx.simplex(&key).map(|c| (c.degree(), c.volume, c.vertices.clone())) else {
                continue;
            };
            if d == 0 {
                continue;
            }
            if !budget.spend() {
                let dkey = chart_simplex_key(&inv, &key);
                self.unfilled.insert(dkey, (inv.clone(), Address::from(key.as_slice())));
                continue;
            }
            let outcome_fill = if spoken.is_translation() {
                if let Some(g) = &spoken_rotor {
                    cx.fill_by_gauge(
                        &key,
                        g,
                        &crate::transducer::element::identity_versor(),
                    )
                } else {
                    cx.fill(&key)
                }
            } else {
                cx.fill(&key)
            };
            match outcome_fill {
                crate::transducer::cech::CoboundaryOutcome::Attached(
                    crate::transducer::cech::Fill::Filled(gamma, carriage),
                ) => {
                    if volume.iter().any(|&x| x > 0) {
                        filled = true;
                        fill_gamma = *gamma;
                        fill_carriage = *carriage;
                        fill_addrs = vertices
                            .iter()
                            .filter_map(|v| cx.sections.get(v).map(|s| s.addr.clone()))
                            .collect();
                    }
                }
                crate::transducer::cech::CoboundaryOutcome::Attached(
                    crate::transducer::cech::Fill::Congruent(kappa, g, ell),
                ) => {
                    let addrs: Vec<Address> = vertices
                        .iter()
                        .filter_map(|v| cx.sections.get(v).map(|s| s.addr.clone()))
                        .collect();
                    congruent = Some(((kappa, g, ell), addrs, key.clone()));
                }
                crate::transducer::cech::CoboundaryOutcome::Attached(
                    crate::transducer::cech::Fill::Class(_, class),
                ) => {
                    let class = class.as_ref().clone();
                    class_simplex = Some((class.clone(), d));
                    let dkey = chart_simplex_key(&inv, &key);
                    self.unfilled.insert(dkey, (inv.clone(), Address::from(key.as_slice())));
                    if d >= 3 {
                        self.pending_refine.insert(inv.key(), (inv.clone(), class, d));
                    }
                }
                // 37.7 at the court's glue: a rotation the seventeen do
                // not spell. It fills nothing and files no class — the
                // simplex stays unfilled and is named for what it is — AND
                // it is adjoined, which is the whole of what 37.7 is for.
                // The simplex staying unfilled and the letter being minted
                // are not alternatives: the transition is still an
                // obstruction on this cover, and the language now has a
                // word for it.
                crate::transducer::cech::CoboundaryOutcome::Attached(
                    crate::transducer::cech::Fill::Unspellable(r),
                ) => {
                    let dkey = chart_simplex_key(&inv, &key);
                    self.unfilled.insert(dkey, (inv.clone(), Address::from(key.as_slice())));
                    self.open_unspellable(r.as_ref(), &inv, &[], &peer);
                }
                // THE BOUNDARY WAS EVALUATED AND DID NOT CLOSE. The
                // simplex stands open and is filed as unfilled, which is
                // the court's record of work still owed.
                crate::transducer::cech::CoboundaryOutcome::Attached(
                    crate::transducer::cech::Fill::Open,
                ) => {
                    let dkey = chart_simplex_key(&inv, &key);
                    self.unfilled.insert(dkey, (inv.clone(), Address::from(key.as_slice())));
                }
                // EVALUATED AND NOT MEASURED (78.26(5)): work still owed,
                // filed as unfilled as it was when it answered `Open`.
                crate::transducer::cech::CoboundaryOutcome::Attached(
                    crate::transducer::cech::Fill::Unread(_),
                ) => {
                    let dkey = chart_simplex_key(&inv, &key);
                    self.unfilled.insert(dkey, (inv.clone(), Address::from(key.as_slice())));
                }
                // ALREADY SETTLED: nothing was attempted, so nothing is
                // owed and nothing is filed.
                crate::transducer::cech::CoboundaryOutcome::AlreadyFilled => {}
                // NOT THERE TO EVALUATE — no such simplex, or a section
                // the fill needed was missing. This used to answer `Open`
                // and was filed as unfilled alongside genuinely open
                // cells, so the court's ledger of owed work counted cells
                // it had never read. An unread simplex is not an open one
                // (rule 11), and it is not filed.
                crate::transducer::cech::CoboundaryOutcome::Absent => {}
            }
        }
        self.complexes.insert(inv.key(), cx);
        // A section the gate refused (no exact rotation to this court's) is
        // refused whole: nothing below fills, pays or classes it.
        if let Glue::Slashed(_) = outcome {
            self.heard_mark(addr, &inv);
            crate::observe::note_replication(Some(peer), crate::observe::rep::GLUE_GATE_REFUSED, 1);
            return outcome;
        }
        if let Some(reason) = self.court_torsion(&spoken, &inv, self_id, peer) {
            self.heard_mark(addr, &inv);
            crate::observe::note_replication(Some(peer), crate::observe::rep::GLUE_TORSION_REFUSED, 1);
            return Glue::Slashed(reason);
        }
        if filled {
            self.record_gamma(&fill_addrs, &fill_gamma, &fill_carriage);
            self.pay_simplex_volume(&inv, &fill_addrs);
            if !fill_gamma.is_identity() {
                if let Some((a, b)) = edge_pair(&fill_addrs) {
                    self.mint_or_admit_translation(&fill_gamma, a, b, peer, budget);
                }
            }
            outcome = Glue::Attached;
        }
        if let Some((remainder, addrs, simplex)) = congruent {
            self.pay_simplex_volume(&inv, &addrs);
            // RM1 — the Torse rewrite minted with its remainder on its record;
            // the torsion paid cites it.
            if let Some(rw) = self.mint_torse_at(&inv, &simplex, remainder, &addrs) {
                self.pay_torse(&addrs, rw.as_bytes());
            }
            outcome = Glue::Attached;
        }
        if let Some((class, degree)) = class_simplex {
            dialect(4, colour);
            self.mark_class(&inv, class.clone(), degree);
            outcome = Glue::Dialect;
        }
        // 78.63 — the path this word left the glue by, past the early exits.
        let path = match &outcome {
            Glue::Admitted => crate::observe::rep::GLUE_ADMITTED,
            Glue::Attached => crate::observe::rep::GLUE_ATTACHED,
            Glue::Dialect | Glue::Slashed(_) => crate::observe::rep::GLUE_CLASS_DIALECT,
        };
        crate::observe::note_replication(Some(peer), path, 1);
        match outcome {
            // 78.63 — AN ATTACHED WORD IS HELD, as an admitted one is. It
            // glues: it fills a simplex, the same word at another placement,
            // and the placement is the dislocation recorded on this court's
            // own section. It was left unheld — `ruled_admit` rules only a
            // word this court holds, and nothing pushed it — so the peer sent
            // it again every pass and the orbit never advanced: b attached
            // 717 peer words in fifteen minutes on 263cc317, every one the
            // same shear read again. Holding it absorbs the shear; the orbit
            // advances by it.
            Glue::Admitted | Glue::Attached => {
                if matches!(outcome, Glue::Attached) {
                    if let Some(rep) = held_rep {
                        if let Some(g) = crate::transducer::cech::transition(&rep, &spoken) {
                            if let Some(v) = crate::transducer::stalk::placement_of(&g) {
                                if let Some(w) = self.word_mut(&rep.address()) {
                                    w.update(|e| {
                                        e.record_dislocation(asg.clone(), v, element::FIBER_SPATIAL)
                                    });
                                    self.touch_at(&rep.address());
                                }
                            }
                        }
                    }
                }
                if self.basis.get(&addr).is_none()
                    && !(spoken.is_connection() && spoken.letters().is_none())
                {
                    self.basis_push(spoken.clone(), peer, budget);
                }
                if let Some(w) = self.word_mut(&addr) {
                    w.update(|e| {
                        e.record_dislocation(
                            asg,
                            element::identity_versor(),
                            element::FIBER_SPATIAL,
                        )
                    });
                    self.touch_at(&addr);
                }
                // ACT1-hold (78.27 Q1): EVERY HOLDING PATH ADMITS BY AN ACT.
                // The glue installs a peer-ruled word on this court's rows, so
                // it is an Install on this court's own line, the peer its
                // witness; the act admits the word, and its stamp is the
                // word's `admitted_at`. It held the word at no act before.
                // 78.27 (3): sealed before the federation files the word, so
                // the word stands at the act that ruled it in.
                self.append_step(crate::chronometry::ActKind::Install, &addr, peer);
                self.ruled_admit(&addr, budget);
                self.fed_admit(&addr, spoken.clone());
                // EC1: the released claim is minted AT the Install that
                // carries it (POUW), never at the act that escrowed it.
                let installed = self.anchor();
                if let Some(crate::economy::escrow::Escrowed { voices, mu, .. }) =
                    self.treasury.release_award(&addr)
                {
                    self.treasury.mint_claim(&voices, mu, installed);
                }
                self.unlist(&addr, false);
                self.leave_provisional(&addr);
                self.court_upheld += 1;
                self.mark_persist();
            }
            Glue::Dialect | Glue::Slashed(_) => {
                self.heard_mark(addr, &inv);
            }
        }
        outcome
    }

    /// The home installs a word it ruled once a peer's ruled digest holds
    /// the same stratum (same root and count) — evidence the peer glued it.
    pub fn confirm_peer_holds(&mut self, remote: &[crate::transducer::sheaf::StratumDigest], peer: [u8; 32]) {
        // THE ACT'S ROOT (65.5): the exchange hands the peer's digest here;
        // one stroke is the confirmation's draw, threaded to every install.
        let mut budget = self.physics.stroke();
        // index: the peer's digest charts; members from ruled fiber
        for d in remote {
            let members = self.ruled.fiber(&d.inv);
            // ONE CONSTRUCTION OF A STRATUM'S ROOT (law 12). This rebuilt
            // the sorted concatenation by hand beside `sheaf::stratum_root`,
            // so the two had to be kept equal by discipline; when the root
            // became a root the copy here would have compared a hash
            // against a concatenation and confirmed nothing, silently.
            //
            // AND THE COUNT NO LONGER DECIDES (rule 2). This read
            // `&& members.len() as u32 == d.count`: the root is taken over
            // exactly these members, so equal roots are equal sets and the
            // count agreed whenever it was consulted. It was a second gate
            // on a reading the first one already made.
            if crate::transducer::sheaf::stratum_root(&members) == d.root {
                for addr in members {
                    if self.provisional.contains(&addr) {
                        self.home_install(&addr, peer, &mut budget);
                    }
                }
            }
            // THIS CHART'S HEARD ADDRESSES, not every heard address filtered
            // down to this chart. The read was the whole of `heard` once per
            // remote digest — strata × |heard|, per peer, per cycle — to
            // reach the few standing under one chart. `heard_at` is the
            // fiber; the chart test the filter made is what the fiber IS.
            let ck = d.inv.key();
            let drop: Vec<Vec<u8>> = match self.heard_at.get(ck.as_bytes()) {
                None => Vec::new(),
                Some(here) => here
                    .scoped()
                    .filter(|(addr, _)| {
                        self.heard.get(addr).is_some_and(|(_, stored)| stored != &d.root)
                    })
                    .map(|(addr, _)| addr)
                    .collect(),
            };
            for addr in drop {
                self.heard_forget(&addr);
            }
        }
    }

    /// Deflation a form may mint: what speaking it measures on the
    /// bound family's trains, or the listing's own magnitude when the word
    /// *is* that listing. Association with a listing (recorded deflation,
    /// sibling trains, whole magnitude at zero) is not a mint.
    pub(crate) fn verified_install_deflation(&self, addr: &[u8], form: &Element) -> [u64; 4] {
        if form.is_sentence() {
            let task = self
                .speech_task
                .get(addr)
                .or_else(|| self.listing_task.get(addr))
                .cloned();
            if let Some(task) = task {
                if let Some(family) = self.task_family.get(&task) {
                    return self.measured_replay(form, family);
                }
            }
            return [0; 4];
        }
        if let Some(e) = self.book_elem(addr) {
            return e.magnitude();
        }
        [0; 4]
    }

    /// 47.3, 13.9: WHAT THIS COURT HOLDS OF ITS OWN ON A CLASS — the Čech
    /// section it wrote from its own perception, at its own vantage. This is
    /// the second vantage a home's offer asks for, and it is the only thing
    /// on this court that is one.
    ///
    /// It is emphatically NOT `word_at(offered_address)`. `glue_fetched`'s
    /// `Admitted` arm calls `basis_push(spoken.clone(), peer)`, which files
    /// the HOME'S OWN TAPE at the offered address, so reading that address
    /// back returns the bytes the home just sent. A witness that answers
    /// with them "can only confirm that a voice sent bytes it had seen
    /// before, which is byte exactness, the one failure this architecture
    /// cannot absorb" (1.7). `None` when this court has no section of its
    /// own on the class: it has re-derived nothing, and an offer it has not
    /// independently reached is UNMEASURED, never agreement.
    pub fn own_section(&self, inv: &crate::transducer::stalk::Invariant) -> Option<&Element> {
        let cx = self.complexes.get(inv.key().as_bytes())?;
        let sec = cx.sections.get(self.identity.as_slice())?;
        self.word_at(&sec.addr)
    }

    /// 13.9, 13.10: THIS COURT RECORDS ITS OWN SECTION. Piece `A` of the
    /// Mayer–Vietoris union is "the hypha's own complex on chart `inv`
    /// (`self.complexes` at `inv.key()`, sections for `self_id` and every
    /// hypha in `cover.fiber(lane, &inv)` it already holds)" — so a word
    /// this court has ruled on a chart is its section there, at its own
    /// vantage, with the identity transition to itself.
    ///
    /// It is the first vantage every later comparison needs. `glue_fetched`
    /// writes the incoming peer's section at the PEER's key, and without a
    /// section at this court's own key `self_rep` reads nothing, `held_rep`
    /// is empty, and everything behind them is unreachable: 13.9 step 3's
    /// difference on vertices, step 4's automorphism check, step 5's
    /// connecting image, 13.10's Consilient/Attached/Dialect table, and
    /// `push_cocycle` (M5). Every glue would be `Admitted` for want of a
    /// first vantage to compare a second against.
    pub(crate) fn section_own(
        &mut self,
        inv: &crate::transducer::stalk::Invariant,
        addr: &crate::transducer::address::Address,
    ) {
        let me = self.identity;
        let vol = self.volume_of_simplex(inv);
        let mut cx = match self.complexes.remove(&inv.key()) {
            Some(c) => c,
            None => crate::transducer::cech::Complex::new(),
        };
        let (self_gamma, self_carriage) = match self.word_at(addr) {
            Some(w) => (
                match w.carried_rotor() {
                    Some(r) => r,
                    None => crate::transducer::alphabet::Rotor::identity(),
                },
                match w.versor() {
                    Some(v) => v,
                    None => crate::transducer::element::identity_versor(),
                },
            ),
            None => (
                crate::transducer::alphabet::Rotor::identity(),
                crate::transducer::element::identity_versor(),
            ),
        };
        cx.set_section(
            me,
            crate::transducer::cech::Section {
                addr: addr.clone(),
                gamma: self_gamma,
                carriage: self_carriage,
            },
            vol,
        );
        self.complexes.insert(inv.key(), cx);
    }

    /// A WITNESS HYPHA replayed an offered form on its own tapes and sent
    /// back the form it reached. This is 47.3's second vantage: two organs
    /// re-derived one object, and `Converged` asks whether they reached the
    /// same VERTEX (M1), not whether they spelled it the same way. On
    /// convergence the home installs; on a genuine difference it refuses
    /// with REFUSE_MV_INEXACT (13.9 step 8), which is inconsistent data and
    /// never a vote to be broken.
    pub fn witness_rederived(
        &mut self,
        object: &crate::transducer::address::Address,
        witness: [u8; 32],
        reached: Element,
    ) {
        if !self.provisional.contains(object) {
            return;
        }
        // S12.4d: the witness sent its vantage; the offer is answered at
        // this witness whatever the two re-derivations turn out to be. A
        // reply transports no frame to the word, so it moves no flux.
        self.note_witness_answered(object.as_bytes(), witness);
        let (k, ht) = (self.k_step(), self.height());
        match self.converge_witness(object, witness, reached, k, ht) {
            crate::court::convergence::Converged::Witnessed { witness, .. } => {
                // THE ACT'S ROOT (65.5): the witness's reply arrives on the
                // wire here, and the install is its one stroke's draw.
                let mut budget = self.physics.stroke();
                self.home_install(object, witness, &mut budget);
            }
            // `REFUSE_MV_INEXACT`: the two re-derivations differ, and the
            // offer stands answered and uninstalled.
            crate::court::convergence::Converged::Inexact { .. } | crate::court::convergence::Converged::AwaitingWitness => {}
        }
    }

    /// A peer's have-list naming a ruled provisional is the same evidence:
    /// the address is on their cover, so the home installs.
    pub fn confirm_peer_holds_addrs(&mut self, have: &[crate::transducer::address::Address], peer: [u8; 32]) {
        // THE ACT'S ROOT (65.5): the exchange hands the have-list here; one
        // stroke is the confirmation's draw, threaded to every install.
        let mut budget = self.physics.stroke();
        // index: the peer's have-list
        for h in have {
            if self.provisional.contains(h) && self.ruled_contains(h) {
                // A HAVE-LIST IS A CLAIM ABOUT AN ADDRESS, not a
                // re-derivation this court can compare. The peer says it
                // holds the form under `h`; the object it reached never
                // travels on this act, so the object recorded here is the
                // one THIS court holds and `same_vertex` against it is
                // vacuous. The re-derivation that is not vacuous arrives on
                // the Offer reply, where the witness sends back the form it
                // reached after replaying on its own tapes. This site stands
                // as the weaker evidence it is rather than pretending; 17.2
                // says a form reads back from its address, and when that
                // decode exists this site can re-derive instead of trusting.
                let Some(mine) = self.provisional.get(h).cloned() else {
                    continue;
                };
                let (k, ht) = (self.k_step(), self.height());
                match self.converge_witness(h, peer, mine, k, ht) {
                    crate::court::convergence::Converged::Witnessed { witness, .. } => {
                        self.home_install(h, witness, &mut budget);
                    }
                    crate::court::convergence::Converged::Inexact { .. }
                    | crate::court::convergence::Converged::AwaitingWitness => {}
                }
            }
        }
    }

}
