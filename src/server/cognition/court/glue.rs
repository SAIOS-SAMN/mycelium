// 23.7, 47.3 — GLUE: what a word fetched from a peer becomes here, and how
// two organs converge on one object.
//
//! A fetched word is replayed against this court's own tapes before it is
//! admitted to anything. A witness that RE-DERIVED the object converges by
//! `same_vertex` (M1), never by comparing the bytes it is spelled with.
//! The install that convergence rests on is `install.rs`.

use crate::cognition::*;
use crate::observe::rep;
use crate::transducer::address::Address;
use crate::transducer::alphabet::Rotor;
use crate::transducer::element::{self, Element, Versor};

/// RM1 — a Congruent fill's remainder as the glue carries it to the mint:
/// the class `κ`, the deficit `g`, the period `ℓ`.
type Torsed = (Box<crate::transducer::sentence::ClassVector>, Box<Rotor>, u32);

/// WHERE THE GLUE STOPS before the word meets this court's cover (23.7).
enum Stop {
    /// Heard on its chart; the confirm re-opens it when a peer's root moves.
    Heard,
    /// This court's replay refused it.
    Replay(&'static str),
    /// A colour word whose proof does not stand here yet (78.55): it waits as
    /// dialect, unheard, and the next fetch proves it once its citations are
    /// installed.
    ColourUnproven,
    /// A sentence citing a form this court does not hold (54.5 B4): the
    /// nerve's targets are read, no form is built.
    CitationMissing,
}

/// HOW THE PEER'S SECTION STANDS TO THIS COURT'S OWN on the chart (13.10,
/// 78.44): every section kept, the curvature between them content.
enum Standing {
    /// No section of this court's own, the identity, or no motion relates
    /// the two: another member of the stratum, no cocycle.
    Member,
    /// The same word at another placement: the placement is the dislocation
    /// recorded on this court's section, and the peer's word is held.
    Placed(Versor),
    /// A turn of finite order relates them: filed as the cocycle.
    Turned,
    /// A turn the gate refuses: no finite order, or no versor or class.
    Refused(&'static str),
}

/// WHAT THE FILLS OF THE PEER'S LINK CAME TO (13.9, RM1): a filled simplex
/// with volume, a congruent one with its remainder, a class simplex. Each
/// holds the last simplex of the link that read it.
#[derive(Default)]
struct Fills {
    filled: Option<(Rotor, Versor, Vec<Address>, Vec<u8>)>,
    congruent: Option<(Torsed, Vec<Address>, Vec<u8>)>,
    class: Option<(Rotor, usize)>,
}

impl Kernel {
    /// On the stroke of the act that fetched the word (65.5): the exchange's
    /// glue, or the witness door's offer — never one of its own. The acts in
    /// order: the stops; the peer's section; its standing to this court's
    /// own; the edges to every section of the cover; the fills of its link;
    /// the gate and torsion; the verdict; the word held or heard.
    pub fn glue_fetched(
        &mut self,
        spoken: Element,
        peer: [u8; 32],
        self_id: [u8; 32],
        cover: &mut crate::network::nerve::Cover,
        budget: &mut crate::physics::Budget,
    ) -> Glue {
        let addr = spoken.address();
        let inv = crate::transducer::stalk::invariant_of(&spoken);
        // 78.71 — A WORD HELD, HEARD AGAIN FROM THE SAME PEER, IS NO NEW
        // EVIDENCE. Only dialect and slashed words were marked heard, so a
        // word this court had admitted was glued whole every time a peer's
        // page named it: an `Install` sealed, its order booked, a fold
        // proposed, for nothing — 13 509 admissions of 201 words on hypha a in
        // twenty minutes, the same words circling the mesh. Its install is
        // stamped with this court's key, so a repeat adds no depth either.
        // A first hearing from a NEW peer still writes that peer's section:
        // a vantage for consilience.
        // Keyed `address ‖ peer`: a complex holds ONE section per peer, the
        // last word glued from it on that chart, so reading the section missed
        // every earlier word from the same peer (c re-admitted 635 in five
        // minutes after the first cut of this stop).
        let mut heard_key = addr.as_bytes().to_vec();
        heard_key.extend_from_slice(&peer);
        let from_this_peer = self.glued_from.contains(&heard_key);
        if from_this_peer && self.ruled_contains(&addr) {
            crate::observe::note_replication(Some(peer), rep::GLUE_HEARD, 1);
            return Glue::Held;
        }
        if let Some(stop) = self.glue_stop(&spoken, &addr, budget) {
            return self.stopped(stop, &spoken, addr, peer);
        }
        let mut cx = match self.complexes.remove(&inv.key()) {
            Some(c) => c,
            None => crate::transducer::cech::Complex::new(),
        };
        // 78.75 — the court listens to its pivots on every chart it glues.
        cx.arm_outflow();
        let vol0 = self.volume_of_simplex(&inv);
        write_peer_section(&mut cx, &spoken, &addr, peer, vol0);
        let held = cx.sections.get(self_id.as_slice()).and_then(|s| self.word_at(&s.addr).cloned());
        let standing = match &held {
            Some(rep) => {
                self.read_against_section(rep, &spoken, &addr, peer, budget);
                self.stand(rep, &spoken, &inv, self_id, peer)
            }
            None => Standing::Member,
        };
        let verts = cover_vertices(cover, &spoken, &inv, self_id, peer);
        self.glue_edges(&mut cx, &verts, &spoken, peer, vol0, budget);
        self.count_reopened(&mut cx, &peer, &inv);
        let fills = self.read_fills(&mut cx, &spoken, &inv, peer, budget);
        self.complexes.insert(inv.key(), cx);
        // 78.75 — what the link's fills sealed, and what earlier releases on
        // the chart handed back, read into the field and the chart's algebra.
        self.book_outflow(&inv, addr.as_bytes(), budget);
        if let Standing::Refused(why) = standing {
            return self.refuse(addr, &inv, peer, rep::GLUE_GATE_REFUSED, why);
        }
        if let Some(why) = self.court_torsion(&spoken, &inv, self_id, peer) {
            return self.refuse(addr, &inv, peer, rep::GLUE_TORSION_REFUSED, why);
        }
        let outcome = self.settle(fills, &standing, &inv, peer, budget);
        crate::observe::note_replication(
            Some(peer),
            match &outcome {
                Glue::Admitted => rep::GLUE_ADMITTED,
                Glue::Attached => rep::GLUE_ATTACHED,
                Glue::Dialect | Glue::Slashed(_) => rep::GLUE_CLASS_DIALECT,
                Glue::Held => rep::GLUE_HEARD,
            },
            1,
        );
        if matches!(outcome, Glue::Admitted | Glue::Attached) {
            self.glued_from.insert(&heard_key, ());
        }
        match (&outcome, standing, held) {
            (Glue::Admitted | Glue::Attached, Standing::Placed(v), Some(rep)) => {
                self.hold_glued(&spoken, addr, peer, Some((rep.address(), v)), budget);
            }
            (Glue::Admitted | Glue::Attached, _, _) => self.hold_glued(&spoken, addr, peer, None, budget),
            (Glue::Dialect | Glue::Slashed(_), _, _) => self.heard_mark(addr, &inv),
            (Glue::Held, _, _) => {}
        }
        outcome
    }

    /// THE STOPS, in order: heard, the replay's refusal, an unproven colour
    /// word, a missing citation. `None` where the word goes on to the cover.
    fn glue_stop(&mut self, spoken: &Element, addr: &Address, budget: &mut crate::physics::Budget) -> Option<Stop> {
        if self.heard.contains(addr) {
            return Some(Stop::Heard);
        }
        if let Some(reason) = self.court_replay(spoken, budget) {
            return Some(Stop::Replay(reason));
        }
        if spoken.is_colour_word() && !self.colour_proof(spoken, None).stands() {
            return Some(Stop::ColourUnproven);
        }
        let cites_unheld = |s: &Element| {
            s.receipts().nerve.iter().any(|e| &e.target != addr && self.word_at(e.target.as_bytes()).is_none())
        };
        (spoken.is_sentence() && cites_unheld(spoken)).then_some(Stop::CitationMissing)
    }

    /// A STOP'S VERDICT, counted on its path: a heard word and an unproven
    /// colour word wait as dialect; a refused replay and a missing citation
    /// are marked heard.
    fn stopped(&mut self, stop: Stop, spoken: &Element, addr: Address, peer: [u8; 32]) -> Glue {
        let inv = crate::transducer::stalk::invariant_of(spoken);
        match stop {
            Stop::Heard => {
                crate::observe::note_replication(Some(peer), rep::GLUE_HEARD, 1);
                Glue::Dialect
            }
            Stop::Replay(why) => self.refuse(addr, &inv, peer, rep::GLUE_REPLAY_REFUSED, why),
            Stop::ColourUnproven => {
                crate::observe::note_replication(Some(peer), rep::GLUE_COLOUR_UNPROVEN, 1);
                Glue::Dialect
            }
            Stop::CitationMissing => {
                self.heard_mark(addr, &inv);
                crate::observe::note_replication(Some(peer), rep::GLUE_CITATION_MISSING, 1);
                Glue::Dialect
            }
        }
    }

    /// A REFUSAL: the word marked heard on its chart, the path counted, the
    /// reason carried.
    fn refuse(&mut self, addr: Address, inv: &crate::transducer::stalk::Invariant, peer: [u8; 32], axis: usize, why: &'static str) -> Glue {
        self.heard_mark(addr, inv);
        crate::observe::note_replication(Some(peer), axis, 1);
        Glue::Slashed(why)
    }

    /// 78.60, 78.61 C1 — THE PEER'S SENTENCE READ AGAINST THIS COURT'S
    /// SECTION, counted per peer and deciding nothing: its exact relation
    /// (one Γ, a conjugate, an unspellable conjugator, or distinct), and the
    /// two lifted into perception and compared lane by lane on TUniversal
    /// {S, G, T, H} — coherent, unread, or the lanes that diverge.
    fn read_against_section(&self, rep: &Element, spoken: &Element, addr: &Address, peer: [u8; 32], budget: &mut crate::physics::Budget) {
        use crate::transducer::sentence::{compare, from_element_form, lift, relation, Relation};
        let (Some(h), Some(o)) = (from_element_form(rep), from_element_form(spoken)) else { return };
        let axis = match (&rep.address() == addr, relation(&h, &o)) {
            (true, _) => rep::RELATION_SAME,
            (false, Relation::Refracted(_)) => rep::RELATION_REFRACTED,
            (false, Relation::Conjugate(_)) => rep::RELATION_CONJUGATE,
            (false, Relation::Unspellable(_)) => rep::RELATION_UNSPELLABLE,
            (false, Relation::Unrelated) => rep::RELATION_DISTINCT,
        };
        crate::observe::note_replication(Some(peer), axis, 1);
        let c = compare(&lift(&h, budget), &lift(&o, budget));
        match (c.coherent(), c.unread) {
            (true, _) => crate::observe::note_replication(Some(peer), rep::PERCEIVED_COHERENT, 1),
            (false, (0, 0)) => {}
            (false, _) => crate::observe::note_replication(Some(peer), rep::PERCEIVED_UNREAD, 1),
        }
        for lane in c.diverging() {
            crate::observe::note_replication(Some(peer), rep::RESIDUE_S + lane, 1);
        }
    }

    /// THE STANDING of the peer's section to this court's own (13.10): the
    /// transition between them read as a placement, a turn, or nothing.
    fn stand(&mut self, rep: &Element, spoken: &Element, inv: &crate::transducer::stalk::Invariant, self_id: [u8; 32], peer: [u8; 32]) -> Standing {
        let Some(g) = crate::transducer::cech::transition(rep, spoken) else { return Standing::Member };
        match (g.is_identity(), crate::transducer::stalk::placement_of(&g), crate::transducer::cech::rotor_of(&g)) {
            (true, _, _) | (false, None, None) => Standing::Member,
            (false, Some(v), _) => Standing::Placed(v),
            (false, None, Some(r)) => self.gate_turn(&r, inv, self_id, peer),
        }
    }

    /// THE GATE ON A TURN between two sections (78.44): an exact rotation of
    /// finite order with a versor and a class is filed as the cocycle; any
    /// other is refused by the part of the gate it fails.
    fn gate_turn(&mut self, r: &Rotor, inv: &crate::transducer::stalk::Invariant, self_id: [u8; 32], peer: [u8; 32]) -> Standing {
        use crate::transducer::alphabet::Order;
        match (r.order(), rotor_versor(r), r.class_key()) {
            (Some(Order::Finite(_)), Some(v), Some(ck)) => {
                self.push_cocycle(inv, self_id, peer, v, ck);
                Standing::Turned
            }
            (Some(Order::Finite(_)), _, _) => Standing::Refused(crate::verdict::REFUSE_SECTION_TURN_UNREAD),
            _ => Standing::Refused(crate::verdict::REFUSE_SECTION_TURN_OPEN),
        }
    }

    /// R6 (47.3) — THE EDGES from every other section of the cover to the
    /// peer's: a group transition by its rotor; two bodies the identity
    /// versor relates by the identity edge; two bodies a versor relates but
    /// no rotor spells, or a transition no rotor realises, by a remainder
    /// edge — the obstruction recorded.
    fn glue_edges(
        &self,
        cx: &mut crate::transducer::cech::Complex,
        verts: &[Vec<u8>],
        spoken: &Element,
        peer: [u8; 32],
        vol0: [u64; 4],
        budget: &mut crate::physics::Budget,
    ) {
        let spoken_body = element::shape_of_chain(spoken.sub());
        for v in verts.iter().filter(|v| v.as_slice() != peer.as_slice()) {
            let Some(w) = cx.sections.get(v).and_then(|s| self.word_at(&s.addr)) else { continue };
            let tr = crate::transducer::cech::transition(w, spoken);
            match (tr.as_ref().and_then(crate::transducer::cech::rotor_of), element::shape_of_chain(w.sub()), &spoken_body) {
                (Some(g), _, _) => {
                    let carriage = match tr.as_ref().and_then(|el| el.versor()) {
                        Some(c) => c,
                        None => element::identity_versor(),
                    };
                    cx.set_edge(v, peer.as_slice(), g, carriage, vol0);
                }
                (None, Some(a), Some(b)) => match crate::transducer::blade::versor_between(&a, b, budget) {
                    Ok(between) if element::same_versor(&between, &element::identity_versor()) => {
                        cx.set_edge(v, peer.as_slice(), Rotor::identity(), element::identity_versor(), vol0);
                    }
                    Ok(_) => cx.set_remainder_edge(v, peer.as_slice()),
                    Err(_) => {}
                },
                (None, _, _) if tr.is_some() => cx.set_remainder_edge(v, peer.as_slice()),
                (None, _, _) => {}
            }
        }
    }

    /// 78.26(6) — THE CLOSED RINGS A MOVED GAUGE REOPENED on the peer's
    /// link, counted by what each cut.
    fn count_reopened(&mut self, cx: &mut crate::transducer::cech::Complex, peer: &[u8; 32], inv: &crate::transducer::stalk::Invariant) {
        for (_, cut) in cx.coboundary_on_link(peer, |_| self.volume_of_simplex(inv)) {
            match cut {
                crate::transducer::cech::Cut::Filled => self.reopened[0] += 1,
                crate::transducer::cech::Cut::Congruent => self.reopened[1] += 1,
            }
        }
    }

    /// 13.9 — THE FILLS OF THE PEER'S LINK, each simplex under the stroke:
    /// a translation fills by its gauge; what is owed is filed unfilled; an
    /// unspellable rotation is adjoined (37.7) and its simplex stays
    /// unfilled; a settled or absent simplex files nothing.
    fn read_fills(
        &mut self,
        cx: &mut crate::transducer::cech::Complex,
        spoken: &Element,
        inv: &crate::transducer::stalk::Invariant,
        peer: [u8; 32],
        budget: &mut crate::physics::Budget,
    ) -> Fills {
        use crate::transducer::cech::{CoboundaryOutcome as O, Fill};
        let mut fills = Fills::default();
        let gauge = spoken.letters().map(|l| l.spelling.word.rotor());
        for key in cx.link_of(&peer) {
            let Some((d, volume, vertices)) = cx.simplex(&key).map(|c| (c.degree(), c.volume, c.vertices.clone())) else {
                continue;
            };
            if d == 0 {
                continue;
            }
            let owed = || (chart_simplex_key(inv, &key), (inv.clone(), Address::from(key.as_slice())));
            if !budget.spend() {
                let (k, v) = owed();
                self.unfilled.insert(k, v);
                continue;
            }
            let fill = match (spoken.is_translation(), &gauge) {
                (true, Some(g)) => cx.fill_by_gauge(&key, g, &element::identity_versor()),
                _ => cx.fill(&key),
            };
            let addrs = || -> Vec<Address> { vertices.iter().filter_map(|v| cx.sections.get(v).map(|s| s.addr.clone())).collect() };
            match fill {
                O::Attached(Fill::Filled(gamma, carriage)) => {
                    if volume.iter().any(|&x| x > 0) {
                        fills.filled = Some((*gamma, *carriage, addrs(), key.clone()));
                    }
                }
                O::Attached(Fill::Congruent(kappa, g, ell)) => fills.congruent = Some(((kappa, g, ell), addrs(), key.clone())),
                O::Attached(Fill::Class(_, class)) => {
                    let (k, v) = owed();
                    self.unfilled.insert(k, v);
                    if d >= 3 {
                        self.pending_refine.insert(inv.key(), (inv.clone(), class.as_ref().clone(), d));
                    }
                    fills.class = Some((*class, d));
                }
                O::Attached(Fill::Unspellable(r)) => {
                    let (k, v) = owed();
                    self.unfilled.insert(k, v);
                    self.open_unspellable(r.as_ref(), inv, &[], &peer);
                }
                O::Attached(Fill::Open | Fill::Unread(_)) => {
                    let (k, v) = owed();
                    self.unfilled.insert(k, v);
                }
                O::AlreadyFilled | O::Absent => {}
            }
        }
        fills
    }

    /// THE VERDICT AND WHAT THE FILLS PAY: a class simplex holds the word as
    /// dialect, its class marked; a filled simplex records its γ, pays its
    /// volume and mints its translation; a congruent one pays and mints its
    /// Torse; either, or a placement, attaches the word; otherwise it is
    /// admitted.
    fn settle(&mut self, fills: Fills, standing: &Standing, inv: &crate::transducer::stalk::Invariant, peer: [u8; 32], budget: &mut crate::physics::Budget) -> Glue {
        let outcome = match (&fills.class, &fills.filled, &fills.congruent, standing) {
            (Some(_), _, _, _) => Glue::Dialect,
            (None, Some(_), _, _) | (None, None, Some(_), _) | (None, None, None, Standing::Placed(_)) => Glue::Attached,
            (None, None, None, _) => Glue::Admitted,
        };
        if let Some((gamma, carriage, addrs, simplex)) = fills.filled {
            self.record_gamma(&addrs, &gamma, &carriage);
            // 78.68: a ring closed to the identity, period one.
            self.close_ring(inv, &simplex, 1, &addrs, None);
            if let (false, Some((a, b))) = (gamma.is_identity(), edge_pair(&addrs)) {
                self.mint_or_admit_translation(&gamma, a, b, peer, budget);
            }
        }
        if let Some((remainder, addrs, simplex)) = fills.congruent {
            // 78.68: the first close settles and mints the Torse; a later
            // one is carried until the ring returns at ℓ.
            let ell = u64::from(remainder.2);
            self.close_ring(inv, &simplex, ell, &addrs, Some(remainder));
        }
        if let Some((class, degree)) = fills.class {
            self.mark_class(inv, class, degree);
        }
        outcome
    }

    /// 78.63, 78.27 Q1 — THE WORD IS HELD: a placement recorded as the
    /// dislocation on this court's own section; the word in the basis with
    /// its identity dislocation; an Install act on this court's line, the
    /// peer its witness; the ruled cover and the federation; the released
    /// claim minted at that Install (EC1, POUW).
    fn hold_glued(&mut self, spoken: &Element, addr: Address, peer: [u8; 32], placed: Option<(Address, Versor)>, budget: &mut crate::physics::Budget) {
        let asg = element::assignment(spoken.sub());
        if let Some((rep, v)) = placed {
            if let Some(w) = self.word_mut(&rep) {
                w.update(|e| e.record_dislocation(asg.clone(), v, element::FIBER_SPATIAL));
                self.touch_at(&rep);
            }
        }
        // Whether the language gains a word here, read before it is held.
        let entered = !self.ruled_contains(&addr);
        if let (None, false) = (self.basis.get(&addr), spoken.is_connection() && spoken.letters().is_none()) {
            self.basis_push(spoken.clone(), peer, budget);
        }
        if let Some(w) = self.word_mut(&addr) {
            w.update(|e| e.record_dislocation(asg, element::identity_versor(), element::FIBER_SPATIAL));
            self.touch_at(&addr);
        }
        self.append_step(crate::chronometry::ActKind::Install, &addr, peer);
        // 78.69 — A GENERATOR ENTERED THE PRESENTATION: the order the
        // language gained, booked on every lane the word carries — its
        // standing extent on 𝒯 among them, which no producer of order on
        // this court reached, so its 𝒯 pressure could only climb. ONCE, where
        // the word enters (78.71): booked on every re-glue it read 36 481 on
        // 𝒮 at hypha a, order the language never gained.
        if entered {
            // 78.71 — AND ITS FLUX ON 𝒯: the word's chain holds no transport
            // (its magnitude reads 𝒯 = 0), so the order it carries on 𝒯 is
            // read off its own transports — each word vertex's acting
            // carriage, by its modes, in its tubes' quanta, the mass its
            // support carries (`transport_order`, the voices' one reader).
            // Its flux on 𝒯 is read where every admission is, `ruled_admit`
            // (`Kernel::step_flux`), against the court's held tubes.
            self.physics.syntropy(&spoken.magnitude());
        }
        self.ruled_admit(&addr, budget);
        self.fed_admit(&addr, spoken.clone());
        let installed = self.anchor();
        if let Some(crate::economy::escrow::Escrowed { voices, mu, .. }) = self.treasury.release_award(&addr) {
            self.treasury.mint_claim(&voices, mu, installed);
        }
        self.unlist(&addr, false);
        self.leave_provisional(&addr);
        self.court_upheld += 1;
        self.mark_persist();
    }

    /// The home installs a word it ruled once a peer's ruled digest holds
    /// the same stratum, by one root (`sheaf::stratum_root`) — evidence the
    /// peer glued it; and every heard address under that chart whose
    /// recorded root is not the peer's is re-opened.
    pub fn confirm_peer_holds(&mut self, remote: &[crate::transducer::sheaf::StratumDigest], peer: [u8; 32]) {
        let mut budget = self.physics.stroke();
        // index: the peer's digest charts; members from ruled fiber
        for d in remote {
            let members = self.ruled.fiber(&d.inv);
            if crate::transducer::sheaf::stratum_root(&members) == d.root {
                let held: Vec<Vec<u8>> = members.into_iter().filter(|a| self.provisional.contains(a)).collect();
                for addr in held {
                    self.home_install(&addr, peer, &mut budget);
                }
            }
            let drop: Vec<Vec<u8>> = match self.heard_at.get(d.inv.key().as_bytes()) {
                None => Vec::new(),
                Some(here) => here
                    .scoped()
                    .filter(|(addr, _)| self.heard.get(addr).is_some_and(|(_, stored)| stored != &d.root))
                    .map(|(addr, _)| addr)
                    .collect(),
            };
            for addr in drop {
                self.heard_forget(&addr);
            }
        }
    }

    /// Deflation a form may mint: what speaking it measures on the bound
    /// family's trains, or the listing's own magnitude when the word is that
    /// listing. Association with a listing is not a mint.
    pub(crate) fn verified_install_deflation(&self, addr: &[u8], form: &Element) -> [u64; 4] {
        match (form.is_sentence(), self.book_elem(addr)) {
            (false, Some(e)) => e.magnitude(),
            (false, None) => [0; 4],
            (true, _) => {
                let task = self.speech_task.get(addr).or_else(|| self.listing_task.get(addr));
                match task.and_then(|t| self.task_family.get(t.as_bytes())) {
                    Some(family) => self.measured_replay(form, family),
                    None => [0; 4],
                }
            }
        }
    }

    /// 47.3, 13.9: WHAT THIS COURT HOLDS OF ITS OWN ON A CLASS — the Čech
    /// section it wrote from its own perception, at its own vantage: the
    /// second vantage a home's offer asks for. Never `word_at` of the offered
    /// address, which reads back the bytes the home sent (1.7). `None` where
    /// this court has no section of its own on the class: unmeasured, never
    /// agreement.
    pub fn own_section(&self, inv: &crate::transducer::stalk::Invariant) -> Option<&Element> {
        let cx = self.complexes.get(inv.key().as_bytes())?;
        let sec = cx.sections.get(self.identity.as_slice())?;
        self.word_at(&sec.addr)
    }

    /// 13.9, 13.10: THIS COURT RECORDS ITS OWN SECTION — a word it has ruled
    /// on a chart is its section there, at its own vantage, with the
    /// identity transition to itself: the first vantage every glue compares
    /// a peer's against.
    pub(crate) fn section_own(&mut self, inv: &crate::transducer::stalk::Invariant, addr: &crate::transducer::address::Address) {
        let me = self.identity;
        let vol = self.volume_of_simplex(inv);
        let mut cx = match self.complexes.remove(&inv.key()) {
            Some(c) => c,
            None => crate::transducer::cech::Complex::new(),
        };
        let (gamma, carriage) = match self.word_at(addr) {
            Some(w) => section_of(w),
            None => (Rotor::identity(), element::identity_versor()),
        };
        cx.set_section(me, crate::transducer::cech::Section { addr: addr.clone(), gamma, carriage }, vol);
        self.complexes.insert(inv.key(), cx);
    }

    /// A WITNESS HYPHA replayed an offered form on its own tapes and sent
    /// back the form it reached: 47.3's second vantage. On convergence to the
    /// same vertex (M1) the home installs; on a genuine difference the offer
    /// stands answered and uninstalled (`REFUSE_MV_INEXACT`, 13.9 step 8).
    pub fn witness_rederived(&mut self, object: &crate::transducer::address::Address, witness: [u8; 32], reached: Element) {
        if !self.provisional.contains(object) {
            return;
        }
        self.note_witness_answered(object.as_bytes(), witness);
        let (k, ht) = (self.k_step(), self.height());
        match self.converge_witness(object, witness, reached, k, ht) {
            crate::court::convergence::Converged::Witnessed { witness, .. } => {
                self.note_provisional("witness reached: installed");
                let mut budget = self.physics.stroke();
                self.home_install(object, witness, &mut budget);
            }
            crate::court::convergence::Converged::Inexact { .. } => self.note_provisional("witness reached: inexact"),
            crate::court::convergence::Converged::AwaitingWitness => self.note_provisional("witness reached: awaiting a vantage"),
        }
    }

    /// 78.102, 47.3 — one step of a provisional word's road, counted on the
    /// acts by its name ([`crate::observe::PROVISIONAL`]).
    pub fn note_provisional(&mut self, name: &str) {
        crate::observe::count(&mut self.acts.provisional, &crate::observe::PROVISIONAL, name);
    }

    /// A peer's have-list naming a ruled provisional is weaker evidence than
    /// a witness's re-derivation: a claim about an address, against which
    /// this court can compare only the form it holds itself.
    pub fn confirm_peer_holds_addrs(&mut self, have: &[crate::transducer::address::Address], peer: [u8; 32]) {
        let mut budget = self.physics.stroke();
        // index: the peer's have-list
        let held: Vec<&Address> = have.iter().filter(|h| self.provisional.contains(h) && self.ruled_contains(h)).collect();
        for h in held {
            let Some(mine) = self.provisional.get(h).cloned() else { continue };
            let (k, ht) = (self.k_step(), self.height());
            match self.converge_witness(h, peer, mine, k, ht) {
                crate::court::convergence::Converged::Witnessed { witness, .. } => self.home_install(h, witness, &mut budget),
                crate::court::convergence::Converged::Inexact { .. } | crate::court::convergence::Converged::AwaitingWitness => {}
            }
        }
    }
}

/// A WORD'S SECTION COORDINATES (13.10): its carried rotor and carriage; a
/// word carrying neither stands at the identity.
fn section_of(w: &Element) -> (Rotor, Versor) {
    (
        match w.carried_rotor() {
            Some(r) => r,
            None => Rotor::identity(),
        },
        match w.versor() {
            Some(v) => v,
            None => element::identity_versor(),
        },
    )
}

impl Kernel {
    /// 78.74 — the vent's reading: every cell the court's vantages released.
    pub fn released_cells(&self) -> crate::transducer::cech::Release {
        self.released_cells
    }

    /// How many charts hold a complex.
    pub fn complex_count(&self) -> usize {
        self.complexes.len()
    }

    /// 78.75 — THIS COURT'S NERVE BY DEGREE, summed over its charts'
    /// complexes: what each degree holds, and each cell's motion by its
    /// modes — under one observer's stroke of its own (never the court's
    /// reserve); a cell past it is unread by name.
    pub fn nerve_census(&self) -> Vec<crate::transducer::cech::DegreeCensus> {
        let mut out: Vec<crate::transducer::cech::DegreeCensus> = Vec::new();
        let mut budget = crate::physics::Physics::default().stroke();
        for (_, cx) in self.complexes.scoped() {
            for (d, c) in cx.census(&mut budget).iter().enumerate() {
                if out.len() <= d {
                    out.resize(d + 1, crate::transducer::cech::DegreeCensus::default());
                }
                out[d].add(c);
            }
        }
        out
    }

    /// 78.74 — every seal this court's complexes made, summed off the
    /// complexes themselves (a thread's ledger sees only its own thread's).
    pub fn cells_sealed(&self) -> u64 {
        self.complexes.scoped().fold(0u64, |s, (_, cx)| match s.checked_add(cx.sealed) {
            Some(t) => t,
            None => s,
        })
    }

    /// 78.74 — multiplicities condense settled by congruence.
    pub fn multiplicity_condensed(&self) -> u64 {
        self.multiplicity_condensed
    }

    /// 78.74 — THE NERVE'S ELASTIC VENT AT THE COURT: a word leaving the
    /// language voids every vantage whose section it was on its chart, and
    /// each such vantage releases its star (`Complex::release_vertex`) —
    /// membranes descend, tethers drain (vacuum, or cauterized with their
    /// record) — so the complex no longer only accretes.
    pub(crate) fn release_vantages(&mut self, inv: &crate::transducer::stalk::Invariant, addr: &[u8]) {
        let Some(cx) = self.complexes.get_mut(&inv.key()) else { return };
        // 78.75 — what the release hands back is held for the chart's next
        // read under a stroke (`book_outflow`).
        cx.arm_outflow();
        let vantages: Vec<Vec<u8>> = cx.sections.scoped().filter(|(_, s)| s.addr.as_bytes() == addr).map(|(v, _)| v).collect();
        for v in vantages {
            let r = cx.release_vertex(&v);
            let t = &mut self.released_cells;
            for (held, by) in [
                (&mut t.released, r.released),
                (&mut t.unsealed, r.unsealed),
                (&mut t.vacuum, r.vacuum),
                (&mut t.cauterized, r.cauterized),
                (&mut t.pinned, r.pinned),
            ] {
                crate::physics::metabolism::step_or_refuse(held, by);
            }
        }
    }
}

/// THE PEER'S SECTION on the chart, at the peer's key.
fn write_peer_section(cx: &mut crate::transducer::cech::Complex, spoken: &Element, addr: &Address, peer: [u8; 32], vol0: [u64; 4]) {
    let (gamma, carriage) = section_of(spoken);
    cx.set_section(peer, crate::transducer::cech::Section { addr: addr.clone(), gamma, carriage }, vol0);
}

/// THE COVER'S VERTICES on the chart: this court, the peer, and every hypha
/// the cover names on a lane the word occupies.
fn cover_vertices(
    cover: &crate::network::nerve::Cover,
    spoken: &Element,
    inv: &crate::transducer::stalk::Invariant,
    self_id: [u8; 32],
    peer: [u8; 32],
) -> Vec<Vec<u8>> {
    let mut verts: crate::network::nerve::Keyed<()> = crate::network::nerve::Keyed::new();
    verts.insert(self_id, ());
    verts.insert(peer, ());
    for lane in crate::network::nerve::occupied_lanes(&spoken.magnitude()) {
        for v in cover.fiber(lane, inv) {
            verts.insert(v, ());
        }
    }
    verts.scoped().map(|(k, _)| k).collect()
}
